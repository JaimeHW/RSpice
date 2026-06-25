#![allow(dead_code, unused_assignments, unused_imports, unused_parens, unused_variables)]

use super::{AdValue, GeneratedEvalContext, GeneratedReactiveStamper, GeneratedStamper, Scratch, THERMAL_VOLTAGE_PER_K};
use super::super::state::Instance;

impl Instance {

    pub(super) fn stamp_transient_block_36(
        &mut self,
        ctx: &GeneratedEvalContext<'_>,
        stamper: &mut GeneratedStamper<'_>,
        scratch: &mut Scratch,
    ) {
        let _ = stamper;
        if (((((scratch.values[2418] != 0.0) && (!(scratch.values[2425] != 0.0))) && (!(scratch.values[2426] != 0.0))) && (!(scratch.values[2429] != 0.0))) && (scratch.values[2430] != 0.0)) {
            scratch.store_ad(2403, &AdValue::exp(AdValue::sub(scratch.ad_value(2417), scratch.ad_value(2316))));
        }

        if (((((scratch.values[2418] != 0.0) && (!(scratch.values[2425] != 0.0))) && (!(scratch.values[2426] != 0.0))) && (!(scratch.values[2429] != 0.0))) && (scratch.values[2430] != 0.0)) {
            scratch.store_ad(2404, &AdValue::div(scratch.ad_value(2317), scratch.ad_value(2403)));
        }

        if (((((scratch.values[2418] != 0.0) && (!(scratch.values[2425] != 0.0))) && (!(scratch.values[2426] != 0.0))) && (!(scratch.values[2429] != 0.0))) && (!(scratch.values[2430] != 0.0))) {
            scratch.store_ad(2403, &AdValue::div_from_scalar(1e-100, AdValue::offset(AdValue::mul(AdValue::offset(AdValue::sub(scratch.ad_value(2316), scratch.ad_value(2417)), (-230.25850929940458)), AdValue::offset(AdValue::scale(AdValue::mul(AdValue::offset(AdValue::sub(scratch.ad_value(2316), scratch.ad_value(2417)), (-230.25850929940458)), AdValue::offset(AdValue::scale(AdValue::offset(AdValue::sub(scratch.ad_value(2316), scratch.ad_value(2417)), (-230.25850929940458)), 0.3333333333333333), 1.0)), 0.5), 1.0)), 1.0)));
        }

        if (((((scratch.values[2418] != 0.0) && (!(scratch.values[2425] != 0.0))) && (!(scratch.values[2426] != 0.0))) && (!(scratch.values[2429] != 0.0))) && (!(scratch.values[2430] != 0.0))) {
            scratch.store_ad(2404, &AdValue::div_from_scalar(1e-100, AdValue::offset(AdValue::mul(AdValue::offset(scratch.ad_value(2417), (-230.25850929940458)), AdValue::offset(AdValue::scale(AdValue::mul(AdValue::offset(scratch.ad_value(2417), (-230.25850929940458)), AdValue::offset(AdValue::scale(AdValue::offset(scratch.ad_value(2417), (-230.25850929940458)), 0.3333333333333333), 1.0)), 0.5), 1.0)), 1.0)));
        }

        if (((scratch.values[2418] != 0.0) && (!(scratch.values[2425] != 0.0))) && (!(scratch.values[2426] != 0.0))) {
            scratch.store_ad(2393, &AdValue::div_from_scalar(1.0, AdValue::offset(AdValue::square(scratch.ad_value(2417)), 2.0)));
        }

        if (((scratch.values[2418] != 0.0) && (!(scratch.values[2425] != 0.0))) && (!(scratch.values[2426] != 0.0))) {
            scratch.store_ad(2405, &AdValue::mul(AdValue::square(scratch.ad_value(2417)), scratch.ad_value(2393)));
        }

        if (((scratch.values[2418] != 0.0) && (!(scratch.values[2425] != 0.0))) && (!(scratch.values[2426] != 0.0))) {
            scratch.store_ad(2406, &AdValue::scale(AdValue::mul(AdValue::mul(scratch.ad_value(2417), scratch.ad_value(2393)), scratch.ad_value(2393)), 4.0));
        }

        if (((scratch.values[2418] != 0.0) && (!(scratch.values[2425] != 0.0))) && (!(scratch.values[2426] != 0.0))) {
            scratch.store_ad(2407, &AdValue::mul(AdValue::mul(AdValue::sub(AdValue::scale(scratch.ad_value(2393), 8.0), AdValue::scale(scratch.ad_value(2405), 12.0)), scratch.ad_value(2393)), scratch.ad_value(2393)));
        }

        if (((scratch.values[2418] != 0.0) && (!(scratch.values[2425] != 0.0))) && (!(scratch.values[2426] != 0.0))) {
            scratch.store_ad(2393, &AdValue::sub(scratch.ad_value(2308), scratch.ad_value(2417)));
        }

        if (((scratch.values[2418] != 0.0) && (!(scratch.values[2425] != 0.0))) && (!(scratch.values[2426] != 0.0))) {
            scratch.store_ad(2408, &AdValue::add(AdValue::scale(scratch.ad_value(2393), 2.0), AdValue::mul(scratch.ad_value(2311), AdValue::sub(AdValue::add(AdValue::sub_from_scalar(1.0, scratch.ad_value(2404)), scratch.ad_value(2403)), AdValue::mul(scratch.ad_value(2317), AdValue::offset(scratch.ad_value(2406), 1.0))))));
        }

        if (((scratch.values[2418] != 0.0) && (!(scratch.values[2425] != 0.0))) && (!(scratch.values[2426] != 0.0))) {
            scratch.store_ad(2409, &AdValue::sub(AdValue::square(scratch.ad_value(2393)), AdValue::mul(scratch.ad_value(2311), AdValue::sub(AdValue::add(AdValue::offset(AdValue::add(scratch.ad_value(2404), scratch.ad_value(2417)), (-1.0)), scratch.ad_value(2403)), AdValue::mul(scratch.ad_value(2317), AdValue::add(AdValue::offset(scratch.ad_value(2417), 1.0), scratch.ad_value(2405)))))));
        }

        if (((scratch.values[2418] != 0.0) && (!(scratch.values[2425] != 0.0))) && (!(scratch.values[2426] != 0.0))) {
            scratch.store_ad(2393, &AdValue::sub_from_scalar(2.0, AdValue::mul(scratch.ad_value(2311), AdValue::sub(AdValue::add(scratch.ad_value(2404), scratch.ad_value(2403)), AdValue::mul(scratch.ad_value(2317), scratch.ad_value(2407))))));
        }

        if (((scratch.values[2418] != 0.0) && (!(scratch.values[2425] != 0.0))) && (!(scratch.values[2426] != 0.0))) {
            scratch.store_ad(2393, &AdValue::sub(AdValue::square(scratch.ad_value(2408)), AdValue::scale(AdValue::mul(scratch.ad_value(2409), scratch.ad_value(2393)), 2.0)));
        }

        if (((scratch.values[2418] != 0.0) && (!(scratch.values[2425] != 0.0))) && (!(scratch.values[2426] != 0.0))) {
            scratch.store_ad(2319, &AdValue::add(scratch.ad_value(2417), AdValue::scale(AdValue::div(scratch.ad_value(2409), AdValue::add(scratch.ad_value(2408), AdValue::sqrt(scratch.ad_value(2393)))), 2.0)));
        }

        if (scratch.values[2418] != 0.0) {
            scratch.values[2353] = scratch.values[2319];
            scratch.node_derivatives[2353] = scratch.node_derivatives[2319];
            scratch.branch_derivatives[2353] = scratch.branch_derivatives[2319];
        }

        if (scratch.values[2418] != 0.0) {
            scratch.values[2361] = scratch.values[2319];
            scratch.node_derivatives[2361] = scratch.node_derivatives[2319];
            scratch.branch_derivatives[2361] = scratch.branch_derivatives[2319];
        }

        if (scratch.values[2418] != 0.0) {
            scratch.values[2354] = 0.0;
            scratch.node_derivatives[2354] = [0.0; Instance::NODE_COUNT];
            scratch.branch_derivatives[2354] = [0.0; Instance::BRANCH_COUNT];
        }

        if (scratch.values[2418] != 0.0) {
            scratch.store_ad(865, &AdValue::scale(scratch.ad_value(2008), 3.912023005));
        }

        scratch.values[2431] = if (scratch.values[2308] <= 0.0) { 1.0 } else { 0.0 };

        if ((scratch.values[2418] != 0.0) && (scratch.values[2431] != 0.0)) {
            scratch.values[2329] = 0.0;
            scratch.node_derivatives[2329] = [0.0; Instance::NODE_COUNT];
            scratch.branch_derivatives[2329] = [0.0; Instance::BRANCH_COUNT];
        }

        if ((scratch.values[2418] != 0.0) && (scratch.values[2431] != 0.0)) {
            scratch.store_ad(2366, &AdValue::sub(scratch.ad_value(2308), scratch.ad_value(2319)));
        }

        if ((scratch.values[2418] != 0.0) && (scratch.values[2431] != 0.0)) {
            scratch.store_ad(2390, &AdValue::mul(scratch.ad_value(2366), scratch.ad_value(2008)));
        }

        if ((scratch.values[2418] != 0.0) && (scratch.values[2431] != 0.0)) {
            scratch.values[2382] = scratch.values[2390];
            scratch.node_derivatives[2382] = scratch.node_derivatives[2390];
            scratch.branch_derivatives[2382] = scratch.branch_derivatives[2390];
        }

        if ((scratch.values[2418] != 0.0) && (scratch.values[2431] != 0.0)) {
            scratch.values[866] = scratch.values[865];
            scratch.node_derivatives[866] = scratch.node_derivatives[865];
            scratch.branch_derivatives[866] = scratch.branch_derivatives[865];
        }

        if ((scratch.values[2418] != 0.0) && (scratch.values[2431] != 0.0)) {
            scratch.values[2348] = scratch.values[850];
            scratch.node_derivatives[2348] = scratch.node_derivatives[850];
            scratch.branch_derivatives[2348] = scratch.branch_derivatives[850];
        }

        if ((scratch.values[2418] != 0.0) && (!(scratch.values[2431] != 0.0))) {
            scratch.values[2320] = 0.0;
            scratch.node_derivatives[2320] = [0.0; Instance::NODE_COUNT];
            scratch.branch_derivatives[2320] = [0.0; Instance::BRANCH_COUNT];
        }

        if ((scratch.values[2418] != 0.0) && (!(scratch.values[2431] != 0.0))) {
            scratch.store_ad(2086, &AdValue::div_from_scalar(1.0, AdValue::offset(AdValue::square(scratch.ad_value(2319)), 2.0)));
        }

        if ((scratch.values[2418] != 0.0) && (!(scratch.values[2431] != 0.0))) {
            scratch.store_ad(2321, &AdValue::mul(AdValue::square(scratch.ad_value(2319)), scratch.ad_value(2086)));
        }

        if ((scratch.values[2418] != 0.0) && (!(scratch.values[2431] != 0.0))) {
            scratch.store_ad(2322, &AdValue::scale(AdValue::mul(AdValue::mul(scratch.ad_value(2319), scratch.ad_value(2086)), scratch.ad_value(2086)), 4.0));
        }

        if ((scratch.values[2418] != 0.0) && (!(scratch.values[2431] != 0.0))) {
            scratch.store_ad(2323, &AdValue::mul(AdValue::mul(AdValue::sub(AdValue::scale(scratch.ad_value(2086), 8.0), AdValue::scale(scratch.ad_value(2321), 12.0)), scratch.ad_value(2086)), scratch.ad_value(2086)));
        }

        scratch.values[2432] = if (scratch.values[2319] < 230.25850929940458) { 1.0 } else { 0.0 };

        if (((scratch.values[2418] != 0.0) && (!(scratch.values[2431] != 0.0))) && (scratch.values[2432] != 0.0)) {
            scratch.store_ad(2320, &AdValue::exp(scratch.ad_value(2319)));
        }

        if (((scratch.values[2418] != 0.0) && (!(scratch.values[2431] != 0.0))) && (scratch.values[2432] != 0.0)) {
            scratch.store_ad(2324, &AdValue::div_from_scalar(1.0, scratch.ad_value(2320)));
        }

        if (((scratch.values[2418] != 0.0) && (!(scratch.values[2431] != 0.0))) && (scratch.values[2432] != 0.0)) {
            scratch.store_ad(2320, &AdValue::mul(scratch.ad_value(2317), scratch.ad_value(2320)));
        }

        scratch.values[2433] = if (scratch.values[2319] > (scratch.values[2316] - 230.25850929940458)) { 1.0 } else { 0.0 };

        if ((((scratch.values[2418] != 0.0) && (!(scratch.values[2431] != 0.0))) && (!(scratch.values[2432] != 0.0))) && (scratch.values[2433] != 0.0)) {
            scratch.store_ad(2320, &AdValue::exp(AdValue::sub(scratch.ad_value(2319), scratch.ad_value(2316))));
        }

        if ((((scratch.values[2418] != 0.0) && (!(scratch.values[2431] != 0.0))) && (!(scratch.values[2432] != 0.0))) && (scratch.values[2433] != 0.0)) {
            scratch.store_ad(2324, &AdValue::div(scratch.ad_value(2317), scratch.ad_value(2320)));
        }

        if ((((scratch.values[2418] != 0.0) && (!(scratch.values[2431] != 0.0))) && (!(scratch.values[2432] != 0.0))) && (!(scratch.values[2433] != 0.0))) {
            scratch.store_ad(2320, &AdValue::div_from_scalar(1e-100, AdValue::offset(AdValue::mul(AdValue::offset(AdValue::sub(scratch.ad_value(2316), scratch.ad_value(2319)), (-230.25850929940458)), AdValue::offset(AdValue::scale(AdValue::mul(AdValue::offset(AdValue::sub(scratch.ad_value(2316), scratch.ad_value(2319)), (-230.25850929940458)), AdValue::offset(AdValue::scale(AdValue::offset(AdValue::sub(scratch.ad_value(2316), scratch.ad_value(2319)), (-230.25850929940458)), 0.3333333333333333), 1.0)), 0.5), 1.0)), 1.0)));
        }

        if ((((scratch.values[2418] != 0.0) && (!(scratch.values[2431] != 0.0))) && (!(scratch.values[2432] != 0.0))) && (!(scratch.values[2433] != 0.0))) {
            scratch.store_ad(2324, &AdValue::div_from_scalar(1e-100, AdValue::offset(AdValue::mul(AdValue::offset(scratch.ad_value(2319), (-230.25850929940458)), AdValue::offset(AdValue::scale(AdValue::mul(AdValue::offset(scratch.ad_value(2319), (-230.25850929940458)), AdValue::offset(AdValue::scale(AdValue::offset(scratch.ad_value(2319), (-230.25850929940458)), 0.3333333333333333), 1.0)), 0.5), 1.0)), 1.0)));
        }

        if ((scratch.values[2418] != 0.0) && (!(scratch.values[2431] != 0.0))) {
            scratch.store_ad(2325, &AdValue::sub(scratch.ad_value(2320), AdValue::mul(scratch.ad_value(2317), AdValue::add(AdValue::offset(scratch.ad_value(2319), 1.0), scratch.ad_value(2321)))));
        }

        scratch.values[2434] = if (scratch.values[2319] < 1e-5) { 1.0 } else { 0.0 };

        if (((scratch.values[2418] != 0.0) && (!(scratch.values[2431] != 0.0))) && (scratch.values[2434] != 0.0)) {
            scratch.store_ad(2326, &AdValue::scale(AdValue::mul(AdValue::square(scratch.ad_value(2319)), AdValue::sub_from_scalar(1.0, AdValue::scale(AdValue::mul(scratch.ad_value(2319), AdValue::sub_from_scalar(1.0, AdValue::scale(scratch.ad_value(2319), 0.25))), 0.3333333333333333))), 0.5));
        }

        if (((scratch.values[2418] != 0.0) && (!(scratch.values[2431] != 0.0))) && (scratch.values[2434] != 0.0)) {
            scratch.store_ad(2325, &AdValue::scale(AdValue::mul(AdValue::mul(AdValue::mul(AdValue::mul(scratch.ad_value(2317), scratch.ad_value(2319)), scratch.ad_value(2319)), scratch.ad_value(2319)), AdValue::offset(AdValue::scale(scratch.ad_value(2319), 1.75), 1.0)), 0.16666666666666666));
        }

        if (((scratch.values[2418] != 0.0) && (!(scratch.values[2431] != 0.0))) && (scratch.values[2434] != 0.0)) {
            scratch.store_ad(2086, &AdValue::sqrt(AdValue::sub_from_scalar(1.0, AdValue::scale(AdValue::mul(scratch.ad_value(2319), AdValue::sub_from_scalar(1.0, AdValue::scale(scratch.ad_value(2319), 0.25))), 0.3333333333333333))));
        }

        if (((scratch.values[2418] != 0.0) && (!(scratch.values[2431] != 0.0))) && (scratch.values[2434] != 0.0)) {
            scratch.store_ad(2368, &AdValue::scale(AdValue::mul(scratch.ad_value(2319), scratch.ad_value(2086)), 0.7071067811865475));
        }

        if (((scratch.values[2418] != 0.0) && (!(scratch.values[2431] != 0.0))) && (scratch.values[2434] != 0.0)) {
            scratch.store_ad(2369, &AdValue::offset(AdValue::div(AdValue::mul(AdValue::scale(scratch.ad_value(2310), 0.7071067811865475), AdValue::add(AdValue::sub_from_scalar(1.0, AdValue::scale(scratch.ad_value(2319), 0.5)), AdValue::scale(AdValue::square(scratch.ad_value(2319)), 0.16666666666666666))), scratch.ad_value(2086)), 1.0));
        }

        if (((scratch.values[2418] != 0.0) && (!(scratch.values[2431] != 0.0))) && (!(scratch.values[2434] != 0.0))) {
            scratch.store_ad(2326, &AdValue::add(AdValue::offset(scratch.ad_value(2319), (-1.0)), scratch.ad_value(2324)));
        }

        if (((scratch.values[2418] != 0.0) && (!(scratch.values[2431] != 0.0))) && (!(scratch.values[2434] != 0.0))) {
            scratch.store_ad(2368, &AdValue::sqrt(scratch.ad_value(2326)));
        }

        if (((scratch.values[2418] != 0.0) && (!(scratch.values[2431] != 0.0))) && (!(scratch.values[2434] != 0.0))) {
            scratch.store_ad(2369, &AdValue::offset(AdValue::scale(AdValue::div(AdValue::mul(scratch.ad_value(2310), AdValue::sub_from_scalar(1.0, scratch.ad_value(2324))), scratch.ad_value(2368)), 0.5), 1.0));
        }

        if ((scratch.values[2418] != 0.0) && (!(scratch.values[2431] != 0.0))) {
            scratch.values[2362] = scratch.values[2324];
            scratch.node_derivatives[2362] = scratch.node_derivatives[2324];
            scratch.branch_derivatives[2362] = scratch.branch_derivatives[2324];
        }

        if ((scratch.values[2418] != 0.0) && (!(scratch.values[2431] != 0.0))) {
            scratch.values[2359] = scratch.values[2362];
            scratch.node_derivatives[2359] = scratch.node_derivatives[2362];
            scratch.branch_derivatives[2359] = scratch.branch_derivatives[2362];
        }

        if ((scratch.values[2418] != 0.0) && (!(scratch.values[2431] != 0.0))) {
            scratch.values[2364] = scratch.values[2325];
            scratch.node_derivatives[2364] = scratch.node_derivatives[2325];
            scratch.branch_derivatives[2364] = scratch.branch_derivatives[2325];
        }

        if ((scratch.values[2418] != 0.0) && (!(scratch.values[2431] != 0.0))) {
            scratch.values[2360] = scratch.values[2364];
            scratch.node_derivatives[2360] = scratch.node_derivatives[2364];
            scratch.branch_derivatives[2360] = scratch.branch_derivatives[2364];
        }

        if ((scratch.values[2418] != 0.0) && (!(scratch.values[2431] != 0.0))) {
            scratch.store_ad(2327, &AdValue::div(AdValue::offset(AdValue::mul(AdValue::scale(scratch.ad_value(759), 0.2), scratch.ad_value(2307)), 1.0), AdValue::offset(AdValue::mul(scratch.ad_value(759), scratch.ad_value(2307)), 1.0)));
        }

        scratch.values[2435] = if (scratch.values[2325] > 1e-100) { 1.0 } else { 0.0 };

        if (((scratch.values[2418] != 0.0) && (!(scratch.values[2431] != 0.0))) && (scratch.values[2435] != 0.0)) {
            scratch.store_ad(2328, &AdValue::mul(scratch.ad_value(2310), AdValue::sqrt(AdValue::add(scratch.ad_value(2326), scratch.ad_value(2325)))));
        }

        if (((scratch.values[2418] != 0.0) && (!(scratch.values[2431] != 0.0))) && (scratch.values[2435] != 0.0)) {
            scratch.store_ad(2329, &AdValue::div(AdValue::mul(AdValue::mul(scratch.ad_value(2311), scratch.ad_value(2325)), scratch.ad_value(2008)), AdValue::add(scratch.ad_value(2328), AdValue::mul(scratch.ad_value(2310), scratch.ad_value(2368)))));
        }

        if (((scratch.values[2418] != 0.0) && (!(scratch.values[2431] != 0.0))) && (scratch.values[2435] != 0.0)) {
            scratch.store_ad(2330, &AdValue::mul(AdValue::mul(scratch.ad_value(2368), scratch.ad_value(2310)), scratch.ad_value(2008)));
        }

        scratch.values[2436] = if (scratch.values[237] < 0.0) { 1.0 } else { 0.0 };

        if ((((scratch.values[2418] != 0.0) && (!(scratch.values[2431] != 0.0))) && (scratch.values[2435] != 0.0)) && (scratch.values[2436] != 0.0)) {
            scratch.store_ad(2331, &AdValue::div_from_scalar(1.0, AdValue::sub_from_scalar(1.0, AdValue::mul(scratch.ad_value(237), scratch.ad_value(2307)))));
        }

        if ((((scratch.values[2418] != 0.0) && (!(scratch.values[2431] != 0.0))) && (scratch.values[2435] != 0.0)) && (!(scratch.values[2436] != 0.0))) {
            scratch.store_ad(2331, &AdValue::offset(AdValue::mul(scratch.ad_value(237), scratch.ad_value(2307)), 1.0));
        }

        scratch.values[2437] = if (scratch.values[238] < 0.0) { 1.0 } else { 0.0 };

        if ((((scratch.values[2418] != 0.0) && (!(scratch.values[2431] != 0.0))) && (scratch.values[2435] != 0.0)) && (scratch.values[2437] != 0.0)) {
            scratch.store_ad(2086, &AdValue::sub_from_scalar(1.0, AdValue::mul(scratch.ad_value(238), scratch.ad_value(2329))));
        }

        if ((((scratch.values[2418] != 0.0) && (!(scratch.values[2431] != 0.0))) && (scratch.values[2435] != 0.0)) && (!(scratch.values[2437] != 0.0))) {
            scratch.store_ad(2086, &AdValue::div_from_scalar(1.0, AdValue::offset(AdValue::mul(scratch.ad_value(238), scratch.ad_value(2329)), 1.0)));
        }

        if (((scratch.values[2418] != 0.0) && (!(scratch.values[2431] != 0.0))) && (scratch.values[2435] != 0.0)) {
            scratch.store_ad(2332, &AdValue::mul(scratch.ad_value(796), AdValue::mul(AdValue::mul(scratch.ad_value(2331), scratch.ad_value(2086)), scratch.ad_value(2329))));
        }

        if (((scratch.values[2418] != 0.0) && (!(scratch.values[2431] != 0.0))) && (scratch.values[2435] != 0.0)) {
            scratch.store_ad(2333, &AdValue::mul(scratch.ad_value(813), AdValue::add(scratch.ad_value(2330), AdValue::mul(scratch.ad_value(814), scratch.ad_value(2329)))));
        }

        if (((scratch.values[2418] != 0.0) && (!(scratch.values[2431] != 0.0))) && (scratch.values[2435] != 0.0)) {
            scratch.store_ad(2087, &AdValue::ln(AdValue::div(scratch.ad_value(2326), AdValue::offset(AdValue::add(scratch.ad_value(2326), scratch.ad_value(2325)), 1e-14))));
        }

        if (((scratch.values[2418] != 0.0) && (!(scratch.values[2431] != 0.0))) && (scratch.values[2435] != 0.0)) {
            scratch.store_ad(2334, &AdValue::add(AdValue::pow(AdValue::mul(scratch.ad_value(2333), scratch.ad_value(755)), scratch.ad_value(756)), AdValue::mul(scratch.ad_value(757), AdValue::exp(AdValue::mul(AdValue::scale(scratch.ad_value(758), 0.5), scratch.ad_value(2087))))));
        }

        if (((scratch.values[2418] != 0.0) && (!(scratch.values[2431] != 0.0))) && (scratch.values[2435] != 0.0)) {
            scratch.store_ad(2335, &AdValue::mul(AdValue::add(AdValue::offset(scratch.ad_value(2334), 1.0), scratch.ad_value(2332)), scratch.ad_value(2327)));
        }

        scratch.values[2438] = if (scratch.values[241] < 0.0) { 1.0 } else { 0.0 };

        if ((((scratch.values[2418] != 0.0) && (!(scratch.values[2431] != 0.0))) && (scratch.values[2435] != 0.0)) && (scratch.values[2438] != 0.0)) {
            scratch.store_ad(2336, &AdValue::div_from_scalar(1.0, AdValue::sub_from_scalar(1.0, AdValue::mul(scratch.ad_value(241), scratch.ad_value(2307)))));
        }

        if ((((scratch.values[2418] != 0.0) && (!(scratch.values[2431] != 0.0))) && (scratch.values[2435] != 0.0)) && (!(scratch.values[2438] != 0.0))) {
            scratch.store_ad(2336, &AdValue::offset(AdValue::mul(scratch.ad_value(241), scratch.ad_value(2307)), 1.0));
        }

        if (((scratch.values[2418] != 0.0) && (!(scratch.values[2431] != 0.0))) && (scratch.values[2435] != 0.0)) {
            scratch.store_ad(2088, &AdValue::mul(scratch.ad_value(2329), scratch.ad_value(2336)));
        }

        if (((scratch.values[2418] != 0.0) && (!(scratch.values[2431] != 0.0))) && (scratch.values[2435] != 0.0)) {
            scratch.store_ad(2337, &AdValue::scale(AdValue::div(scratch.ad_value(2088), AdValue::offset(scratch.ad_value(2088), 100.0)), 100.0));
        }

        scratch.values[2439] = if (scratch.values[242] < 0.0) { 1.0 } else { 0.0 };

        if ((((scratch.values[2418] != 0.0) && (!(scratch.values[2431] != 0.0))) && (scratch.values[2435] != 0.0)) && (scratch.values[2439] != 0.0)) {
            scratch.store_ad(2086, &AdValue::div_from_scalar(1.0, AdValue::sub_from_scalar(1.0, AdValue::mul(scratch.ad_value(242), scratch.ad_value(2337)))));
        }

        if ((((scratch.values[2418] != 0.0) && (!(scratch.values[2431] != 0.0))) && (scratch.values[2435] != 0.0)) && (!(scratch.values[2439] != 0.0))) {
            scratch.store_ad(2086, &AdValue::offset(AdValue::mul(scratch.ad_value(242), scratch.ad_value(2337)), 1.0));
        }

        if (((scratch.values[2418] != 0.0) && (!(scratch.values[2431] != 0.0))) && (scratch.values[2435] != 0.0)) {
            scratch.store_ad(2338, &AdValue::mul(scratch.ad_value(2079), AdValue::div(scratch.ad_value(2086), scratch.ad_value(2335))));
        }

        if (((scratch.values[2418] != 0.0) && (!(scratch.values[2431] != 0.0))) && (scratch.values[2435] != 0.0)) {
            scratch.store_ad(2339, &AdValue::add(AdValue::div(scratch.ad_value(2329), scratch.ad_value(2369)), scratch.ad_value(2008)));
        }

        if (((scratch.values[2418] != 0.0) && (!(scratch.values[2431] != 0.0))) && (scratch.values[2435] != 0.0)) {
            scratch.store_ad(2340, &AdValue::scale(AdValue::mul(scratch.ad_value(2338), scratch.ad_value(2339)), 0.7071067811865475));
        }

        scratch.values[2440] = if (scratch.values[0] == (-1.0)) { 1.0 } else { 0.0 };

        if ((((scratch.values[2418] != 0.0) && (!(scratch.values[2431] != 0.0))) && (scratch.values[2435] != 0.0)) && (scratch.values[2440] != 0.0)) {
            scratch.store_ad(2340, &AdValue::div(scratch.ad_value(2340), AdValue::sqrt(AdValue::offset(scratch.ad_value(2340), 1.0))));
        }

        if (((scratch.values[2418] != 0.0) && (!(scratch.values[2431] != 0.0))) && (scratch.values[2435] != 0.0)) {
            scratch.store_ad(2341, &AdValue::div_from_scalar(2.0, AdValue::offset(AdValue::sqrt(AdValue::offset(AdValue::scale(scratch.ad_value(2340), 4.0), 1.0)), 1.0)));
        }

        if (((scratch.values[2418] != 0.0) && (!(scratch.values[2431] != 0.0))) && (scratch.values[2435] != 0.0)) {
            scratch.store_ad(2087, &AdValue::mul(scratch.ad_value(2341), scratch.ad_value(2340)));
        }

        if (((scratch.values[2418] != 0.0) && (!(scratch.values[2431] != 0.0))) && (scratch.values[2435] != 0.0)) {
            scratch.store_ad(2342, &AdValue::mul(AdValue::mul(scratch.ad_value(2339), scratch.ad_value(2341)), AdValue::offset(AdValue::scale(AdValue::div(AdValue::mul(scratch.ad_value(2087), AdValue::sub_from_scalar(1.0, AdValue::mul(scratch.ad_value(2087), scratch.ad_value(2341)))), AdValue::offset(AdValue::scale(AdValue::mul(AdValue::square(scratch.ad_value(2087)), scratch.ad_value(2341)), 4.0), 1.0)), 0.86), 1.0)));
        }

        if (((scratch.values[2418] != 0.0) && (!(scratch.values[2431] != 0.0))) && (scratch.values[2435] != 0.0)) {
            scratch.store_ad(2343, &AdValue::add(scratch.ad_value(2328), AdValue::scale(scratch.ad_value(2311), 0.5)));
        }

        if (((scratch.values[2418] != 0.0) && (!(scratch.values[2431] != 0.0))) && (scratch.values[2435] != 0.0)) {
            scratch.store_ad(2344, &AdValue::scale(AdValue::div(AdValue::mul(AdValue::mul(scratch.ad_value(2311), scratch.ad_value(2325)), scratch.ad_value(2008)), AdValue::add(scratch.ad_value(2343), AdValue::sqrt(AdValue::sub(AdValue::square(scratch.ad_value(2343)), AdValue::scale(AdValue::mul(scratch.ad_value(2311), scratch.ad_value(2325)), 0.98))))), 0.98));
        }

        if (((scratch.values[2418] != 0.0) && (!(scratch.values[2431] != 0.0))) && (scratch.values[2435] != 0.0)) {
            scratch.store_ad(2345, &AdValue::add(scratch.ad_value(2342), scratch.ad_value(2344)));
        }

        if (((scratch.values[2418] != 0.0) && (!(scratch.values[2431] != 0.0))) && (scratch.values[2435] != 0.0)) {
            scratch.store_ad(2346, &AdValue::scale(AdValue::mul(scratch.ad_value(2342), scratch.ad_value(2344)), 2.0));
        }

        if (((scratch.values[2418] != 0.0) && (!(scratch.values[2431] != 0.0))) && (scratch.values[2435] != 0.0)) {
            scratch.store_ad(2347, &AdValue::div(scratch.ad_value(2346), AdValue::add(scratch.ad_value(2345), AdValue::sqrt(AdValue::sub(AdValue::square(scratch.ad_value(2345)), AdValue::scale(scratch.ad_value(2346), 1.98))))));
        }

        if (((scratch.values[2418] != 0.0) && (!(scratch.values[2431] != 0.0))) && (scratch.values[2435] != 0.0)) {
            scratch.store_ad(866, &AdValue::sub(scratch.ad_value(2347), AdValue::mul(scratch.ad_value(2008), AdValue::ln(AdValue::offset(AdValue::div(AdValue::mul(AdValue::mul(scratch.ad_value(2347), AdValue::sub(scratch.ad_value(2347), AdValue::mul(AdValue::scale(scratch.ad_value(2343), 2.0), scratch.ad_value(2008)))), scratch.ad_value(2312)), AdValue::mul(AdValue::square(scratch.ad_value(2008)), scratch.ad_value(2325))), 1.0)))));
        }

        if (((scratch.values[2418] != 0.0) && (!(scratch.values[2431] != 0.0))) && (!(scratch.values[2435] != 0.0))) {
            scratch.values[866] = scratch.values[865];
            scratch.node_derivatives[866] = scratch.node_derivatives[865];
            scratch.branch_derivatives[866] = scratch.branch_derivatives[865];
        }

        if ((scratch.values[2418] != 0.0) && (!(scratch.values[2431] != 0.0))) {
            scratch.store_ad(2086, &AdValue::pow(AdValue::div(scratch.ad_value(850), scratch.ad_value(866)), scratch.ad_value(243)));
        }

        if ((scratch.values[2418] != 0.0) && (!(scratch.values[2431] != 0.0))) {
            scratch.store_ad(2348, &AdValue::mul(scratch.ad_value(850), AdValue::pow(AdValue::offset(scratch.ad_value(2086), 1.0), AdValue::neg(scratch.ad_value(816)))));
        }

        if ((scratch.values[2418] != 0.0) && (!(scratch.values[2431] != 0.0))) {
            scratch.store_ad(2349, &AdValue::mul(scratch.ad_value(2348), scratch.ad_value(2009)));
        }

        if ((scratch.values[2418] != 0.0) && (!(scratch.values[2431] != 0.0))) {
            scratch.store_ad(2350, &AdValue::add(scratch.ad_value(2316), scratch.ad_value(2349)));
        }

        scratch.values[2441] = if (scratch.values[2349] < 460.51701859880916) { 1.0 } else { 0.0 };

        if (((scratch.values[2418] != 0.0) && (!(scratch.values[2431] != 0.0))) && (scratch.values[2441] != 0.0)) {
            scratch.store_ad(2351, &AdValue::exp(AdValue::neg(scratch.ad_value(2349))));
        }

        if (((scratch.values[2418] != 0.0) && (!(scratch.values[2431] != 0.0))) && (!(scratch.values[2441] != 0.0))) {
            scratch.store_ad(2351, &AdValue::div_from_scalar(1e-200, AdValue::offset(AdValue::mul(AdValue::offset(scratch.ad_value(2349), (-460.51701859880916)), AdValue::offset(AdValue::scale(AdValue::mul(AdValue::offset(scratch.ad_value(2349), (-460.51701859880916)), AdValue::offset(AdValue::scale(AdValue::offset(scratch.ad_value(2349), (-460.51701859880916)), 0.3333333333333333), 1.0)), 0.5), 1.0)), 1.0)));
        }

        if ((scratch.values[2418] != 0.0) && (!(scratch.values[2431] != 0.0))) {
            scratch.store_ad(2352, &AdValue::mul(scratch.ad_value(2317), scratch.ad_value(2351)));
        }

        scratch.values[2442] = if (((scratch.values[2308]) as f64).abs() <= scratch.values[2318]) { 1.0 } else { 0.0 };

        if (((scratch.values[2418] != 0.0) && (!(scratch.values[2431] != 0.0))) && (scratch.values[2442] != 0.0)) {
            scratch.store_ad(2394, &AdValue::scale(AdValue::square(scratch.ad_value(2314)), (0.16666666666666666 * 0.7071067811865475)));
        }

        if (((scratch.values[2418] != 0.0) && (!(scratch.values[2431] != 0.0))) && (scratch.values[2442] != 0.0)) {
            scratch.store_ad(2353, &AdValue::mul(AdValue::mul(scratch.ad_value(2308), scratch.ad_value(2314)), AdValue::offset(AdValue::mul(AdValue::mul(AdValue::mul(scratch.ad_value(2308), AdValue::sub_from_scalar(1.0, scratch.ad_value(2352))), scratch.ad_value(2310)), scratch.ad_value(2394)), 1.0)));
        }

        if (((scratch.values[2418] != 0.0) && (!(scratch.values[2431] != 0.0))) && (!(scratch.values[2442] != 0.0))) {
            scratch.store_ad(2415, &AdValue::offset(scratch.ad_value(2350), 3.0));
        }

        if (((scratch.values[2418] != 0.0) && (!(scratch.values[2431] != 0.0))) && (!(scratch.values[2442] != 0.0))) {
            scratch.store_ad(2398, &AdValue::sub(AdValue::scale(AdValue::sub(AdValue::add(scratch.ad_value(2414), scratch.ad_value(2415)), AdValue::sqrt(AdValue::offset(AdValue::mul(AdValue::sub(scratch.ad_value(2414), scratch.ad_value(2415)), AdValue::sub(scratch.ad_value(2414), scratch.ad_value(2415))), 5.0))), 0.5), AdValue::scale(AdValue::sub(scratch.ad_value(2415), AdValue::sqrt(AdValue::offset(AdValue::square(scratch.ad_value(2415)), 5.0))), 0.5)));
        }

        if (((scratch.values[2418] != 0.0) && (!(scratch.values[2431] != 0.0))) && (!(scratch.values[2442] != 0.0))) {
            scratch.store_ad(2393, &AdValue::sub(scratch.ad_value(2308), scratch.ad_value(2398)));
        }

        if (((scratch.values[2418] != 0.0) && (!(scratch.values[2431] != 0.0))) && (!(scratch.values[2442] != 0.0))) {
            scratch.store_ad(2394, &AdValue::exp(AdValue::neg(scratch.ad_value(2398))));
        }

        if (((scratch.values[2418] != 0.0) && (!(scratch.values[2431] != 0.0))) && (!(scratch.values[2442] != 0.0))) {
            scratch.store_ad(2395, &AdValue::div_from_scalar(1.0, AdValue::offset(AdValue::square(scratch.ad_value(2398)), 2.0)));
        }

        if (((scratch.values[2418] != 0.0) && (!(scratch.values[2431] != 0.0))) && (!(scratch.values[2442] != 0.0))) {
            scratch.store_ad(2405, &AdValue::mul(AdValue::square(scratch.ad_value(2398)), scratch.ad_value(2395)));
        }

        if (((scratch.values[2418] != 0.0) && (!(scratch.values[2431] != 0.0))) && (!(scratch.values[2442] != 0.0))) {
            scratch.store_ad(2406, &AdValue::scale(AdValue::mul(AdValue::mul(scratch.ad_value(2398), scratch.ad_value(2395)), scratch.ad_value(2395)), 4.0));
        }

        if (((scratch.values[2418] != 0.0) && (!(scratch.values[2431] != 0.0))) && (!(scratch.values[2442] != 0.0))) {
            scratch.store_ad(2407, &AdValue::mul(AdValue::mul(AdValue::sub(AdValue::scale(scratch.ad_value(2395), 8.0), AdValue::scale(scratch.ad_value(2405), 12.0)), scratch.ad_value(2395)), scratch.ad_value(2395)));
        }

        if (((scratch.values[2418] != 0.0) && (!(scratch.values[2431] != 0.0))) && (!(scratch.values[2442] != 0.0))) {
            scratch.store_ad(2399, &AdValue::max_from_scalar(1e-40, AdValue::sub(AdValue::square(scratch.ad_value(2393)), AdValue::mul(scratch.ad_value(2311), AdValue::sub(AdValue::offset(AdValue::add(scratch.ad_value(2394), scratch.ad_value(2398)), (-1.0)), AdValue::mul(scratch.ad_value(2352), AdValue::add(AdValue::offset(scratch.ad_value(2398), 1.0), scratch.ad_value(2405))))))));
        }

        if (((scratch.values[2418] != 0.0) && (!(scratch.values[2431] != 0.0))) && (!(scratch.values[2442] != 0.0))) {
            scratch.store_ad(2416, &AdValue::sub_from_scalar(1.0, AdValue::scale(AdValue::mul(scratch.ad_value(2311), AdValue::sub(scratch.ad_value(2394), AdValue::mul(scratch.ad_value(2352), scratch.ad_value(2407)))), 0.5)));
        }

        if (((scratch.values[2418] != 0.0) && (!(scratch.values[2431] != 0.0))) && (!(scratch.values[2442] != 0.0))) {
            scratch.store_ad(2400, &AdValue::add(AdValue::scale(scratch.ad_value(2393), 2.0), AdValue::mul(scratch.ad_value(2311), AdValue::sub(AdValue::sub_from_scalar(1.0, scratch.ad_value(2394)), AdValue::mul(scratch.ad_value(2352), AdValue::offset(scratch.ad_value(2406), 1.0))))));
        }

        if (((scratch.values[2418] != 0.0) && (!(scratch.values[2431] != 0.0))) && (!(scratch.values[2442] != 0.0))) {
            scratch.store_ad(2401, &AdValue::add(AdValue::sub(scratch.ad_value(2350), scratch.ad_value(2398)), AdValue::ln(AdValue::div(scratch.ad_value(2399), scratch.ad_value(2311)))));
        }

        if (((scratch.values[2418] != 0.0) && (!(scratch.values[2431] != 0.0))) && (!(scratch.values[2442] != 0.0))) {
            scratch.store_ad(962, &AdValue::add(scratch.ad_value(2399), scratch.ad_value(2400)));
        }

        if (((scratch.values[2418] != 0.0) && (!(scratch.values[2431] != 0.0))) && (!(scratch.values[2442] != 0.0))) {
            scratch.store_ad(961, &AdValue::add(AdValue::square(scratch.ad_value(962)), AdValue::mul(scratch.ad_value(2401), AdValue::sub(AdValue::scale(AdValue::square(scratch.ad_value(2400)), 0.5), AdValue::mul(scratch.ad_value(2399), scratch.ad_value(2416))))));
        }

        if (((scratch.values[2418] != 0.0) && (!(scratch.values[2431] != 0.0))) && (!(scratch.values[2442] != 0.0))) {
            let assign49400_ad_e63756: AdValue = AdValue::add(scratch.ad_value(2398), AdValue::div(AdValue::mul(AdValue::mul(scratch.ad_value(2399), scratch.ad_value(962)), scratch.ad_value(2401)), AdValue::add(scratch.ad_value(961), AdValue::mul(AdValue::mul(AdValue::mul(AdValue::mul(AdValue::div(scratch.ad_value(962), scratch.ad_value(961)), scratch.ad_value(2401)), scratch.ad_value(2401)), scratch.ad_value(2400)), AdValue::sub(AdValue::scale(AdValue::square(scratch.ad_value(2400)), 0.3333333333333333), AdValue::mul(scratch.ad_value(2399), scratch.ad_value(2416)))))));
            scratch.store_ad(2417, &assign49400_ad_e63756);
        }

        scratch.values[2443] = if (scratch.values[2417] < 230.25850929940458) { 1.0 } else { 0.0 };

        if ((((scratch.values[2418] != 0.0) && (!(scratch.values[2431] != 0.0))) && (!(scratch.values[2442] != 0.0))) && (scratch.values[2443] != 0.0)) {
            scratch.store_ad(2403, &AdValue::exp(scratch.ad_value(2417)));
        }

        if ((((scratch.values[2418] != 0.0) && (!(scratch.values[2431] != 0.0))) && (!(scratch.values[2442] != 0.0))) && (scratch.values[2443] != 0.0)) {
            scratch.store_ad(2404, &AdValue::div_from_scalar(1.0, scratch.ad_value(2403)));
        }

        if ((((scratch.values[2418] != 0.0) && (!(scratch.values[2431] != 0.0))) && (!(scratch.values[2442] != 0.0))) && (scratch.values[2443] != 0.0)) {
            scratch.store_ad(2403, &AdValue::mul(scratch.ad_value(2352), scratch.ad_value(2403)));
        }

        scratch.values[2444] = if (scratch.values[2417] > (scratch.values[2350] - 230.25850929940458)) { 1.0 } else { 0.0 };

        if (((((scratch.values[2418] != 0.0) && (!(scratch.values[2431] != 0.0))) && (!(scratch.values[2442] != 0.0))) && (!(scratch.values[2443] != 0.0))) && (scratch.values[2444] != 0.0)) {
            scratch.store_ad(2403, &AdValue::exp(AdValue::sub(scratch.ad_value(2417), scratch.ad_value(2350))));
        }

        if (((((scratch.values[2418] != 0.0) && (!(scratch.values[2431] != 0.0))) && (!(scratch.values[2442] != 0.0))) && (!(scratch.values[2443] != 0.0))) && (scratch.values[2444] != 0.0)) {
            scratch.store_ad(2404, &AdValue::div(scratch.ad_value(2352), scratch.ad_value(2403)));
        }

        if (((((scratch.values[2418] != 0.0) && (!(scratch.values[2431] != 0.0))) && (!(scratch.values[2442] != 0.0))) && (!(scratch.values[2443] != 0.0))) && (!(scratch.values[2444] != 0.0))) {
            scratch.store_ad(2403, &AdValue::div_from_scalar(1e-100, AdValue::offset(AdValue::mul(AdValue::offset(AdValue::sub(scratch.ad_value(2350), scratch.ad_value(2417)), (-230.25850929940458)), AdValue::offset(AdValue::scale(AdValue::mul(AdValue::offset(AdValue::sub(scratch.ad_value(2350), scratch.ad_value(2417)), (-230.25850929940458)), AdValue::offset(AdValue::scale(AdValue::offset(AdValue::sub(scratch.ad_value(2350), scratch.ad_value(2417)), (-230.25850929940458)), 0.3333333333333333), 1.0)), 0.5), 1.0)), 1.0)));
        }

        if (((((scratch.values[2418] != 0.0) && (!(scratch.values[2431] != 0.0))) && (!(scratch.values[2442] != 0.0))) && (!(scratch.values[2443] != 0.0))) && (!(scratch.values[2444] != 0.0))) {
            scratch.store_ad(2404, &AdValue::div_from_scalar(1e-100, AdValue::offset(AdValue::mul(AdValue::offset(scratch.ad_value(2417), (-230.25850929940458)), AdValue::offset(AdValue::scale(AdValue::mul(AdValue::offset(scratch.ad_value(2417), (-230.25850929940458)), AdValue::offset(AdValue::scale(AdValue::offset(scratch.ad_value(2417), (-230.25850929940458)), 0.3333333333333333), 1.0)), 0.5), 1.0)), 1.0)));
        }

        if (((scratch.values[2418] != 0.0) && (!(scratch.values[2431] != 0.0))) && (!(scratch.values[2442] != 0.0))) {
            scratch.store_ad(2393, &AdValue::div_from_scalar(1.0, AdValue::offset(AdValue::square(scratch.ad_value(2417)), 2.0)));
        }

    }

    pub(super) fn stamp_transient_block_37(
        &mut self,
        ctx: &GeneratedEvalContext<'_>,
        stamper: &mut GeneratedStamper<'_>,
        scratch: &mut Scratch,
    ) {
        let _ = stamper;
        if (((scratch.values[2418] != 0.0) && (!(scratch.values[2431] != 0.0))) && (!(scratch.values[2442] != 0.0))) {
            scratch.store_ad(2405, &AdValue::mul(AdValue::square(scratch.ad_value(2417)), scratch.ad_value(2393)));
        }

        if (((scratch.values[2418] != 0.0) && (!(scratch.values[2431] != 0.0))) && (!(scratch.values[2442] != 0.0))) {
            scratch.store_ad(2406, &AdValue::scale(AdValue::mul(AdValue::mul(scratch.ad_value(2417), scratch.ad_value(2393)), scratch.ad_value(2393)), 4.0));
        }

        if (((scratch.values[2418] != 0.0) && (!(scratch.values[2431] != 0.0))) && (!(scratch.values[2442] != 0.0))) {
            scratch.store_ad(2407, &AdValue::mul(AdValue::mul(AdValue::sub(AdValue::scale(scratch.ad_value(2393), 8.0), AdValue::scale(scratch.ad_value(2405), 12.0)), scratch.ad_value(2393)), scratch.ad_value(2393)));
        }

        if (((scratch.values[2418] != 0.0) && (!(scratch.values[2431] != 0.0))) && (!(scratch.values[2442] != 0.0))) {
            scratch.store_ad(2393, &AdValue::sub(scratch.ad_value(2308), scratch.ad_value(2417)));
        }

        if (((scratch.values[2418] != 0.0) && (!(scratch.values[2431] != 0.0))) && (!(scratch.values[2442] != 0.0))) {
            scratch.store_ad(2408, &AdValue::add(AdValue::scale(scratch.ad_value(2393), 2.0), AdValue::mul(scratch.ad_value(2311), AdValue::sub(AdValue::add(AdValue::sub_from_scalar(1.0, scratch.ad_value(2404)), scratch.ad_value(2403)), AdValue::mul(scratch.ad_value(2352), AdValue::offset(scratch.ad_value(2406), 1.0))))));
        }

        if (((scratch.values[2418] != 0.0) && (!(scratch.values[2431] != 0.0))) && (!(scratch.values[2442] != 0.0))) {
            scratch.store_ad(2409, &AdValue::sub(AdValue::square(scratch.ad_value(2393)), AdValue::mul(scratch.ad_value(2311), AdValue::sub(AdValue::add(AdValue::offset(AdValue::add(scratch.ad_value(2404), scratch.ad_value(2417)), (-1.0)), scratch.ad_value(2403)), AdValue::mul(scratch.ad_value(2352), AdValue::add(AdValue::offset(scratch.ad_value(2417), 1.0), scratch.ad_value(2405)))))));
        }

        if (((scratch.values[2418] != 0.0) && (!(scratch.values[2431] != 0.0))) && (!(scratch.values[2442] != 0.0))) {
            scratch.store_ad(2393, &AdValue::sub_from_scalar(2.0, AdValue::mul(scratch.ad_value(2311), AdValue::sub(AdValue::add(scratch.ad_value(2404), scratch.ad_value(2403)), AdValue::mul(scratch.ad_value(2352), scratch.ad_value(2407))))));
        }

        if (((scratch.values[2418] != 0.0) && (!(scratch.values[2431] != 0.0))) && (!(scratch.values[2442] != 0.0))) {
            scratch.store_ad(2393, &AdValue::sub(AdValue::square(scratch.ad_value(2408)), AdValue::scale(AdValue::mul(scratch.ad_value(2409), scratch.ad_value(2393)), 2.0)));
        }

        if (((scratch.values[2418] != 0.0) && (!(scratch.values[2431] != 0.0))) && (!(scratch.values[2442] != 0.0))) {
            scratch.store_ad(2353, &AdValue::add(scratch.ad_value(2417), AdValue::scale(AdValue::div(scratch.ad_value(2409), AdValue::add(scratch.ad_value(2408), AdValue::sqrt(scratch.ad_value(2393)))), 2.0)));
        }

        if ((scratch.values[2418] != 0.0) && (!(scratch.values[2431] != 0.0))) {
            scratch.store_ad(2354, &AdValue::sub(scratch.ad_value(2353), scratch.ad_value(2319)));
        }

        scratch.values[2445] = if (scratch.values[2354] < 1e-10) { 1.0 } else { 0.0 };

        if (((scratch.values[2418] != 0.0) && (!(scratch.values[2431] != 0.0))) && (scratch.values[2445] != 0.0)) {
            scratch.store_ad(2355, &AdValue::add(AdValue::scale(AdValue::sub(scratch.ad_value(2308), scratch.ad_value(2319)), 2.0), AdValue::mul(scratch.ad_value(2311), AdValue::sub(AdValue::add(AdValue::sub_from_scalar(1.0, scratch.ad_value(2324)), AdValue::mul(scratch.ad_value(2320), scratch.ad_value(2351))), AdValue::mul(scratch.ad_value(2352), AdValue::offset(scratch.ad_value(2322), 1.0))))));
        }

        if (((scratch.values[2418] != 0.0) && (!(scratch.values[2431] != 0.0))) && (scratch.values[2445] != 0.0)) {
            scratch.store_ad(2356, &AdValue::mul(AdValue::mul(scratch.ad_value(2311), AdValue::sub_from_scalar(1.0, scratch.ad_value(2351))), scratch.ad_value(2325)));
        }

        if (((scratch.values[2418] != 0.0) && (!(scratch.values[2431] != 0.0))) && (scratch.values[2445] != 0.0)) {
            scratch.store_ad(2086, &AdValue::sub_from_scalar(2.0, AdValue::mul(scratch.ad_value(2311), AdValue::sub(AdValue::add(scratch.ad_value(2324), AdValue::mul(scratch.ad_value(2320), scratch.ad_value(2351))), AdValue::mul(scratch.ad_value(2352), scratch.ad_value(2323))))));
        }

        if (((scratch.values[2418] != 0.0) && (!(scratch.values[2431] != 0.0))) && (scratch.values[2445] != 0.0)) {
            scratch.store_ad(2086, &AdValue::sub(AdValue::square(scratch.ad_value(2355)), AdValue::scale(AdValue::mul(scratch.ad_value(2086), scratch.ad_value(2356)), 2.0)));
        }

        if (((scratch.values[2418] != 0.0) && (!(scratch.values[2431] != 0.0))) && (scratch.values[2445] != 0.0)) {
            scratch.store_ad(2354, &AdValue::scale(AdValue::div(scratch.ad_value(2356), AdValue::add(scratch.ad_value(2355), AdValue::sqrt(scratch.ad_value(2086)))), 2.0));
        }

        if (((scratch.values[2418] != 0.0) && (!(scratch.values[2431] != 0.0))) && (scratch.values[2445] != 0.0)) {
            scratch.store_ad(2353, &AdValue::add(scratch.ad_value(2319), scratch.ad_value(2354)));
        }

        if ((scratch.values[2418] != 0.0) && (!(scratch.values[2431] != 0.0))) {
            scratch.store_ad(2357, &AdValue::mul(scratch.ad_value(2354), scratch.ad_value(2008)));
        }

        if ((scratch.values[2418] != 0.0) && (!(scratch.values[2431] != 0.0))) {
            scratch.store_ad(2358, &AdValue::div(AdValue::square(scratch.ad_value(2353)), AdValue::offset(AdValue::square(scratch.ad_value(2353)), 2.0)));
        }

        scratch.values[2446] = if (scratch.values[2353] < 230.25850929940458) { 1.0 } else { 0.0 };

        if (((scratch.values[2418] != 0.0) && (!(scratch.values[2431] != 0.0))) && (scratch.values[2446] != 0.0)) {
            scratch.store_ad(2359, &AdValue::exp(AdValue::neg(scratch.ad_value(2353))));
        }

        scratch.values[2447] = if (scratch.values[2353] < 1e-5) { 1.0 } else { 0.0 };

        if ((((scratch.values[2418] != 0.0) && (!(scratch.values[2431] != 0.0))) && (scratch.values[2446] != 0.0)) && (scratch.values[2447] != 0.0)) {
            scratch.store_ad(2360, &AdValue::mul(AdValue::mul(AdValue::mul(AdValue::mul(AdValue::scale(scratch.ad_value(2352), 0.16666666666666666), scratch.ad_value(2353)), scratch.ad_value(2353)), scratch.ad_value(2353)), AdValue::offset(AdValue::scale(scratch.ad_value(2353), 1.75), 1.0)));
        }

        if ((((scratch.values[2418] != 0.0) && (!(scratch.values[2431] != 0.0))) && (scratch.values[2446] != 0.0)) && (!(scratch.values[2447] != 0.0))) {
            scratch.store_ad(2360, &AdValue::mul(scratch.ad_value(2352), AdValue::sub(AdValue::offset(AdValue::sub(AdValue::div_from_scalar(1.0, scratch.ad_value(2359)), scratch.ad_value(2353)), (-1.0)), scratch.ad_value(2358))));
        }

        scratch.values[2448] = if (scratch.values[2353] > (scratch.values[2350] - 230.25850929940458)) { 1.0 } else { 0.0 };

        if ((((scratch.values[2418] != 0.0) && (!(scratch.values[2431] != 0.0))) && (!(scratch.values[2446] != 0.0))) && (scratch.values[2448] != 0.0)) {
            scratch.store_ad(2086, &AdValue::exp(AdValue::sub(scratch.ad_value(2353), scratch.ad_value(2350))));
        }

        if ((((scratch.values[2418] != 0.0) && (!(scratch.values[2431] != 0.0))) && (!(scratch.values[2446] != 0.0))) && (scratch.values[2448] != 0.0)) {
            scratch.store_ad(2359, &AdValue::div(scratch.ad_value(2352), scratch.ad_value(2086)));
        }

        if ((((scratch.values[2418] != 0.0) && (!(scratch.values[2431] != 0.0))) && (!(scratch.values[2446] != 0.0))) && (scratch.values[2448] != 0.0)) {
            scratch.store_ad(2360, &AdValue::sub(scratch.ad_value(2086), AdValue::mul(scratch.ad_value(2352), AdValue::add(AdValue::offset(scratch.ad_value(2353), 1.0), scratch.ad_value(2358)))));
        }

        if ((((scratch.values[2418] != 0.0) && (!(scratch.values[2431] != 0.0))) && (!(scratch.values[2446] != 0.0))) && (!(scratch.values[2448] != 0.0))) {
            scratch.store_ad(2359, &AdValue::div_from_scalar(1e-100, AdValue::offset(AdValue::mul(AdValue::offset(scratch.ad_value(2353), (-230.25850929940458)), AdValue::offset(AdValue::scale(AdValue::mul(AdValue::offset(scratch.ad_value(2353), (-230.25850929940458)), AdValue::offset(AdValue::scale(AdValue::offset(scratch.ad_value(2353), (-230.25850929940458)), 0.3333333333333333), 1.0)), 0.5), 1.0)), 1.0)));
        }

        if ((((scratch.values[2418] != 0.0) && (!(scratch.values[2431] != 0.0))) && (!(scratch.values[2446] != 0.0))) && (!(scratch.values[2448] != 0.0))) {
            scratch.store_ad(2086, &AdValue::div_from_scalar(1e-100, AdValue::offset(AdValue::mul(AdValue::offset(AdValue::sub(scratch.ad_value(2350), scratch.ad_value(2353)), (-230.25850929940458)), AdValue::offset(AdValue::scale(AdValue::mul(AdValue::offset(AdValue::sub(scratch.ad_value(2350), scratch.ad_value(2353)), (-230.25850929940458)), AdValue::offset(AdValue::scale(AdValue::offset(AdValue::sub(scratch.ad_value(2350), scratch.ad_value(2353)), (-230.25850929940458)), 0.3333333333333333), 1.0)), 0.5), 1.0)), 1.0)));
        }

        if ((((scratch.values[2418] != 0.0) && (!(scratch.values[2431] != 0.0))) && (!(scratch.values[2446] != 0.0))) && (!(scratch.values[2448] != 0.0))) {
            scratch.store_ad(2360, &AdValue::sub(scratch.ad_value(2086), AdValue::mul(scratch.ad_value(2352), AdValue::add(AdValue::offset(scratch.ad_value(2353), 1.0), scratch.ad_value(2358)))));
        }

        if ((scratch.values[2418] != 0.0) && (!(scratch.values[2431] != 0.0))) {
            scratch.store_ad(2361, &AdValue::scale(AdValue::add(scratch.ad_value(2319), scratch.ad_value(2353)), 0.5));
        }

        if ((scratch.values[2418] != 0.0) && (!(scratch.values[2431] != 0.0))) {
            scratch.values[2362] = 0.0;
            scratch.node_derivatives[2362] = [0.0; Instance::NODE_COUNT];
            scratch.branch_derivatives[2362] = [0.0; Instance::BRANCH_COUNT];
        }

        if ((scratch.values[2418] != 0.0) && (!(scratch.values[2431] != 0.0))) {
            scratch.store_ad(2086, &AdValue::mul(scratch.ad_value(2359), scratch.ad_value(2324)));
        }

        scratch.values[2449] = if (scratch.values[2086] > 0.0) { 1.0 } else { 0.0 };

        if (((scratch.values[2418] != 0.0) && (!(scratch.values[2431] != 0.0))) && (scratch.values[2449] != 0.0)) {
            scratch.store_ad(2362, &AdValue::sqrt(scratch.ad_value(2086)));
        }

        if ((scratch.values[2418] != 0.0) && (!(scratch.values[2431] != 0.0))) {
            scratch.store_ad(2363, &AdValue::scale(AdValue::add(scratch.ad_value(2325), scratch.ad_value(2360)), 0.5));
        }

        if ((scratch.values[2418] != 0.0) && (!(scratch.values[2431] != 0.0))) {
            scratch.store_ad(2364, &AdValue::add(scratch.ad_value(2363), AdValue::scale(AdValue::mul(AdValue::square(scratch.ad_value(2354)), AdValue::sub(scratch.ad_value(2362), AdValue::scale(scratch.ad_value(2312), 2.0))), 0.125)));
        }

        scratch.values[2450] = if (scratch.values[2361] < 1e-5) { 1.0 } else { 0.0 };

        if (((scratch.values[2418] != 0.0) && (!(scratch.values[2431] != 0.0))) && (scratch.values[2450] != 0.0)) {
            scratch.store_ad(2365, &AdValue::scale(AdValue::mul(AdValue::square(scratch.ad_value(2361)), AdValue::sub_from_scalar(1.0, AdValue::scale(AdValue::mul(scratch.ad_value(2361), AdValue::sub_from_scalar(1.0, AdValue::scale(scratch.ad_value(2361), 0.25))), 0.3333333333333333))), 0.5));
        }

        if (((scratch.values[2418] != 0.0) && (!(scratch.values[2431] != 0.0))) && (scratch.values[2450] != 0.0)) {
            scratch.store_ad(2366, &AdValue::mul(scratch.ad_value(2310), AdValue::sqrt(AdValue::add(scratch.ad_value(2364), scratch.ad_value(2365)))));
        }

        scratch.values[2451] = if (scratch.values[769] > 0.0) { 1.0 } else { 0.0 };

        if ((((scratch.values[2418] != 0.0) && (!(scratch.values[2431] != 0.0))) && (scratch.values[2450] != 0.0)) && (scratch.values[2451] != 0.0)) {
            scratch.store_ad(2367, &AdValue::div_from_scalar(1.0, AdValue::sqrt(AdValue::offset(AdValue::mul(scratch.ad_value(769), scratch.ad_value(2366)), 1.0))));
        }

        if (((scratch.values[2418] != 0.0) && (!(scratch.values[2431] != 0.0))) && (scratch.values[2450] != 0.0)) {
            scratch.store_ad(2086, &AdValue::sqrt(AdValue::sub_from_scalar(1.0, AdValue::scale(AdValue::mul(scratch.ad_value(2361), AdValue::sub_from_scalar(1.0, AdValue::scale(scratch.ad_value(2361), 0.25))), 0.3333333333333333))));
        }

        if (((scratch.values[2418] != 0.0) && (!(scratch.values[2431] != 0.0))) && (scratch.values[2450] != 0.0)) {
            scratch.store_ad(2368, &AdValue::scale(AdValue::mul(scratch.ad_value(2361), scratch.ad_value(2086)), 0.7071067811865475));
        }

        if (((scratch.values[2418] != 0.0) && (!(scratch.values[2431] != 0.0))) && (scratch.values[2450] != 0.0)) {
            scratch.store_ad(2369, &AdValue::add(scratch.ad_value(2367), AdValue::scale(AdValue::div(AdValue::mul(scratch.ad_value(2310), AdValue::add(AdValue::sub_from_scalar(1.0, AdValue::scale(scratch.ad_value(2361), 0.5)), AdValue::scale(AdValue::square(scratch.ad_value(2361)), 0.16666666666666666))), scratch.ad_value(2086)), 0.7071067811865475)));
        }

        if (((scratch.values[2418] != 0.0) && (!(scratch.values[2431] != 0.0))) && (!(scratch.values[2450] != 0.0))) {
            scratch.store_ad(2365, &AdValue::add(AdValue::offset(scratch.ad_value(2361), (-1.0)), scratch.ad_value(2362)));
        }

        if (((scratch.values[2418] != 0.0) && (!(scratch.values[2431] != 0.0))) && (!(scratch.values[2450] != 0.0))) {
            scratch.store_ad(2366, &AdValue::mul(scratch.ad_value(2310), AdValue::sqrt(AdValue::add(scratch.ad_value(2364), scratch.ad_value(2365)))));
        }

        scratch.values[2452] = if (scratch.values[769] > 0.0) { 1.0 } else { 0.0 };

        if ((((scratch.values[2418] != 0.0) && (!(scratch.values[2431] != 0.0))) && (!(scratch.values[2450] != 0.0))) && (scratch.values[2452] != 0.0)) {
            scratch.store_ad(2370, &AdValue::add(AdValue::sub_from_scalar(1.0, scratch.ad_value(2362)), AdValue::scale(AdValue::mul(scratch.ad_value(2366), scratch.ad_value(2312)), 2.0)));
        }

        if ((((scratch.values[2418] != 0.0) && (!(scratch.values[2431] != 0.0))) && (!(scratch.values[2450] != 0.0))) && (scratch.values[2452] != 0.0)) {
            scratch.store_ad(2367, &AdValue::div_from_scalar(1.0, AdValue::sqrt(AdValue::offset(AdValue::mul(scratch.ad_value(769), scratch.ad_value(2366)), 1.0))));
        }

        if ((((scratch.values[2418] != 0.0) && (!(scratch.values[2431] != 0.0))) && (!(scratch.values[2450] != 0.0))) && (scratch.values[2452] != 0.0)) {
            scratch.store_ad(2086, &AdValue::div(scratch.ad_value(2367), AdValue::offset(scratch.ad_value(2367), 1.0)));
        }

        if ((((scratch.values[2418] != 0.0) && (!(scratch.values[2431] != 0.0))) && (!(scratch.values[2450] != 0.0))) && (scratch.values[2452] != 0.0)) {
            scratch.store_ad(2371, &AdValue::mul(scratch.ad_value(769), AdValue::mul(AdValue::mul(AdValue::square(scratch.ad_value(2086)), scratch.ad_value(2311)), scratch.ad_value(2364))));
        }

        if ((((scratch.values[2418] != 0.0) && (!(scratch.values[2431] != 0.0))) && (!(scratch.values[2450] != 0.0))) && (scratch.values[2452] != 0.0)) {
            scratch.store_ad(2372, &AdValue::add(AdValue::scale(AdValue::sub(scratch.ad_value(2366), scratch.ad_value(2371)), 2.0), AdValue::mul(scratch.ad_value(2311), AdValue::add(AdValue::sub_from_scalar(1.0, scratch.ad_value(2362)), scratch.ad_value(2364)))));
        }

        if ((((scratch.values[2418] != 0.0) && (!(scratch.values[2431] != 0.0))) && (!(scratch.values[2450] != 0.0))) && (scratch.values[2452] != 0.0)) {
            scratch.store_ad(2373, &AdValue::mul(scratch.ad_value(2371), AdValue::sub(scratch.ad_value(2371), AdValue::scale(scratch.ad_value(2366), 2.0))));
        }

        if ((((scratch.values[2418] != 0.0) && (!(scratch.values[2431] != 0.0))) && (!(scratch.values[2450] != 0.0))) && (scratch.values[2452] != 0.0)) {
            scratch.store_ad(2374, &AdValue::sub_from_scalar(1.0, AdValue::scale(AdValue::mul(scratch.ad_value(2311), AdValue::add(scratch.ad_value(2362), scratch.ad_value(2364))), 0.5)));
        }

        if ((((scratch.values[2418] != 0.0) && (!(scratch.values[2431] != 0.0))) && (!(scratch.values[2450] != 0.0))) && (scratch.values[2452] != 0.0)) {
            scratch.store_ad(2375, &AdValue::div(AdValue::mul(scratch.ad_value(2373), scratch.ad_value(2372)), AdValue::sub(AdValue::square(scratch.ad_value(2372)), AdValue::mul(scratch.ad_value(2374), scratch.ad_value(2373)))));
        }

        if ((((scratch.values[2418] != 0.0) && (!(scratch.values[2431] != 0.0))) && (!(scratch.values[2450] != 0.0))) && (scratch.values[2452] != 0.0)) {
            scratch.store_ad(2361, &AdValue::add(scratch.ad_value(2361), scratch.ad_value(2375)));
        }

        if ((((scratch.values[2418] != 0.0) && (!(scratch.values[2431] != 0.0))) && (!(scratch.values[2450] != 0.0))) && (scratch.values[2452] != 0.0)) {
            scratch.store_ad(2376, &AdValue::exp(scratch.ad_value(2375)));
        }

        if ((((scratch.values[2418] != 0.0) && (!(scratch.values[2431] != 0.0))) && (!(scratch.values[2450] != 0.0))) && (scratch.values[2452] != 0.0)) {
            scratch.store_ad(2362, &AdValue::div(scratch.ad_value(2362), scratch.ad_value(2376)));
        }

        if ((((scratch.values[2418] != 0.0) && (!(scratch.values[2431] != 0.0))) && (!(scratch.values[2450] != 0.0))) && (scratch.values[2452] != 0.0)) {
            scratch.store_ad(2364, &AdValue::mul(scratch.ad_value(2364), scratch.ad_value(2376)));
        }

        if ((((scratch.values[2418] != 0.0) && (!(scratch.values[2431] != 0.0))) && (!(scratch.values[2450] != 0.0))) && (scratch.values[2452] != 0.0)) {
            scratch.store_ad(2365, &AdValue::add(AdValue::offset(scratch.ad_value(2361), (-1.0)), scratch.ad_value(2362)));
        }

        if ((((scratch.values[2418] != 0.0) && (!(scratch.values[2431] != 0.0))) && (!(scratch.values[2450] != 0.0))) && (scratch.values[2452] != 0.0)) {
            scratch.store_ad(2366, &AdValue::mul(scratch.ad_value(2310), AdValue::sqrt(AdValue::add(scratch.ad_value(2364), scratch.ad_value(2365)))));
        }

        if ((((scratch.values[2418] != 0.0) && (!(scratch.values[2431] != 0.0))) && (!(scratch.values[2450] != 0.0))) && (scratch.values[2452] != 0.0)) {
            scratch.store_ad(2377, &AdValue::add(AdValue::sub_from_scalar(1.0, scratch.ad_value(2362)), AdValue::scale(AdValue::mul(AdValue::mul(scratch.ad_value(2366), scratch.ad_value(2367)), scratch.ad_value(2312)), 2.0)));
        }

        if ((((scratch.values[2418] != 0.0) && (!(scratch.values[2431] != 0.0))) && (!(scratch.values[2450] != 0.0))) && (scratch.values[2452] != 0.0)) {
            scratch.store_ad(2354, &AdValue::div(AdValue::mul(AdValue::mul(scratch.ad_value(2354), scratch.ad_value(2376)), AdValue::add(scratch.ad_value(2370), scratch.ad_value(2363))), AdValue::add(scratch.ad_value(2377), AdValue::mul(scratch.ad_value(2376), scratch.ad_value(2363)))));
        }

        if ((((scratch.values[2418] != 0.0) && (!(scratch.values[2431] != 0.0))) && (!(scratch.values[2450] != 0.0))) && (scratch.values[2452] != 0.0)) {
            scratch.store_ad(2357, &AdValue::mul(scratch.ad_value(2354), scratch.ad_value(2008)));
        }

        if (((scratch.values[2418] != 0.0) && (!(scratch.values[2431] != 0.0))) && (!(scratch.values[2450] != 0.0))) {
            scratch.store_ad(2368, &AdValue::sqrt(scratch.ad_value(2365)));
        }

        if (((scratch.values[2418] != 0.0) && (!(scratch.values[2431] != 0.0))) && (!(scratch.values[2450] != 0.0))) {
            scratch.store_ad(2369, &AdValue::add(scratch.ad_value(2367), AdValue::scale(AdValue::div(AdValue::mul(scratch.ad_value(2310), AdValue::sub_from_scalar(1.0, scratch.ad_value(2362))), scratch.ad_value(2368)), 0.5)));
        }

        if ((scratch.values[2418] != 0.0) && (!(scratch.values[2431] != 0.0))) {
            scratch.store_ad(2378, &AdValue::mul(scratch.ad_value(2008), AdValue::div(AdValue::mul(scratch.ad_value(2311), scratch.ad_value(2364)), AdValue::add(scratch.ad_value(2366), AdValue::mul(scratch.ad_value(2310), scratch.ad_value(2368))))));
        }

        if ((scratch.values[2418] != 0.0) && (!(scratch.values[2431] != 0.0))) {
            scratch.store_ad(2379, &AdValue::add(scratch.ad_value(2378), AdValue::mul(scratch.ad_value(2008), scratch.ad_value(2369))));
        }

        if ((scratch.values[2418] != 0.0) && (!(scratch.values[2431] != 0.0))) {
            scratch.store_ad(2380, &AdValue::mul(AdValue::mul(scratch.ad_value(2368), scratch.ad_value(2310)), scratch.ad_value(2008)));
        }

        scratch.values[2453] = if (scratch.values[238] < 0.0) { 1.0 } else { 0.0 };

        if (((scratch.values[2418] != 0.0) && (!(scratch.values[2431] != 0.0))) && (scratch.values[2453] != 0.0)) {
            scratch.store_ad(2086, &AdValue::sub_from_scalar(1.0, AdValue::mul(scratch.ad_value(238), scratch.ad_value(2378))));
        }

        if (((scratch.values[2418] != 0.0) && (!(scratch.values[2431] != 0.0))) && (!(scratch.values[2453] != 0.0))) {
            scratch.store_ad(2086, &AdValue::div_from_scalar(1.0, AdValue::offset(AdValue::mul(scratch.ad_value(238), scratch.ad_value(2378)), 1.0)));
        }

        if ((scratch.values[2418] != 0.0) && (!(scratch.values[2431] != 0.0))) {
            scratch.store_ad(2332, &AdValue::mul(scratch.ad_value(796), AdValue::mul(AdValue::mul(scratch.ad_value(2331), scratch.ad_value(2086)), scratch.ad_value(2378))));
        }

        if ((scratch.values[2418] != 0.0) && (!(scratch.values[2431] != 0.0))) {
            scratch.store_ad(2381, &AdValue::add(scratch.ad_value(2380), AdValue::mul(scratch.ad_value(814), scratch.ad_value(2378))));
        }

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

    }

    pub(super) fn stamp_transient_block_38(
        &mut self,
        ctx: &GeneratedEvalContext<'_>,
        stamper: &mut GeneratedStamper<'_>,
        scratch: &mut Scratch,
    ) {
        let _ = stamper;
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

        scratch.values[2030] = 0.0;

        scratch.values[2031] = 0.0;

        scratch.values[2032] = 0.0;

        scratch.values[2033] = 0.0;

        scratch.values[2034] = 0.0;

        scratch.values[2035] = 0.0;

        scratch.values[2036] = 0.0;

        scratch.values[2037] = 0.0;

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
            scratch.store_ad(534, &AdValue::scale(scratch.ad_value(854), (scratch.values[427] * scratch.values[719])));
        }

        if ((scratch.values[2458] != 0.0) && (scratch.values[2459] != 0.0)) {
            let assign51170_ad_e65764: AdValue = {
                if (scratch.values[534] < (-230.25850929940458)) {
                    AdValue::div_from_scalar(1e-100, AdValue::offset(AdValue::sub_from_scalar((-230.25850929940458), scratch.ad_value(534)), 1.0))
                } else {
                    {
                        if (scratch.values[534] > scratch.values[711]) {
                            AdValue::mul(scratch.ad_value(712), AdValue::offset(AdValue::sub(scratch.ad_value(534), scratch.ad_value(711)), 1.0))
                        } else {
                            AdValue::exp(scratch.ad_value(534))
                        }
                    }
                }
            };
            scratch.store_ad(535, &assign51170_ad_e65764);
        }

        if ((scratch.values[2458] != 0.0) && (scratch.values[2459] != 0.0)) {
            scratch.store_ad(540, &AdValue::mul(scratch.ad_value(718), AdValue::offset(scratch.ad_value(535), (-1.0))));
        }

        if ((scratch.values[2458] != 0.0) && (scratch.values[2459] != 0.0)) {
            scratch.store_ad(534, &AdValue::mul(AdValue::scale(scratch.ad_value(854), scratch.values[427]), scratch.ad_value(721)));
        }

        if ((scratch.values[2458] != 0.0) && (scratch.values[2459] != 0.0)) {
            let assign51200_ad_e65815: AdValue = {
                if (scratch.values[534] < (-230.25850929940458)) {
                    AdValue::div_from_scalar(1e-100, AdValue::offset(AdValue::sub_from_scalar((-230.25850929940458), scratch.ad_value(534)), 1.0))
                } else {
                    {
                        if (scratch.values[534] > scratch.values[713]) {
                            AdValue::mul(scratch.ad_value(714), AdValue::offset(AdValue::sub(scratch.ad_value(534), scratch.ad_value(713)), 1.0))
                        } else {
                            AdValue::exp(scratch.ad_value(534))
                        }
                    }
                }
            };
            scratch.store_ad(535, &assign51200_ad_e65815);
        }

        if ((scratch.values[2458] != 0.0) && (scratch.values[2459] != 0.0)) {
            scratch.store_ad(541, &AdValue::mul(scratch.ad_value(720), AdValue::offset(scratch.ad_value(535), (-1.0))));
        }

        if ((scratch.values[2458] != 0.0) && (scratch.values[2459] != 0.0)) {
            scratch.values[542] = 0.0;
            scratch.node_derivatives[542] = [0.0; Instance::NODE_COUNT];
            scratch.branch_derivatives[542] = [0.0; Instance::BRANCH_COUNT];
        }

        scratch.values[2460] = if (scratch.values[717] > 0.0) { 1.0 } else { 0.0 };

        if (((scratch.values[2458] != 0.0) && (scratch.values[2459] != 0.0)) && (scratch.values[2460] != 0.0)) {
            scratch.store_ad(542, &AdValue::mul(scratch.ad_value(854), AdValue::add(scratch.ad_value(722), AdValue::mul(scratch.ad_value(854), scratch.ad_value(723)))));
        }

        if (((scratch.values[2458] != 0.0) && (scratch.values[2459] != 0.0)) && (!(scratch.values[2460] != 0.0))) {
            scratch.store_ad(534, &AdValue::mul(AdValue::scale(AdValue::neg(scratch.ad_value(854)), scratch.values[427]), scratch.ad_value(723)));
        }

        if (((scratch.values[2458] != 0.0) && (scratch.values[2459] != 0.0)) && (!(scratch.values[2460] != 0.0))) {
            let assign51260_ad_e65896: AdValue = {
                if (scratch.values[534] < (-230.25850929940458)) {
                    AdValue::div_from_scalar(1e-100, AdValue::offset(AdValue::sub_from_scalar((-230.25850929940458), scratch.ad_value(534)), 1.0))
                } else {
                    {
                        if (scratch.values[534] > scratch.values[715]) {
                            AdValue::mul(scratch.ad_value(716), AdValue::offset(AdValue::sub(scratch.ad_value(534), scratch.ad_value(715)), 1.0))
                        } else {
                            AdValue::exp(scratch.ad_value(534))
                        }
                    }
                }
            };
            scratch.store_ad(535, &assign51260_ad_e65896);
        }

        if (((scratch.values[2458] != 0.0) && (scratch.values[2459] != 0.0)) && (!(scratch.values[2460] != 0.0))) {
            scratch.store_ad(542, &AdValue::mul(AdValue::neg(scratch.ad_value(722)), AdValue::offset(scratch.ad_value(535), (-1.0))));
        }

        if ((scratch.values[2458] != 0.0) && (scratch.values[2459] != 0.0)) {
            scratch.store_ad(2030, &AdValue::add(AdValue::add(scratch.ad_value(540), scratch.ad_value(541)), scratch.ad_value(542)));
        }

        if ((scratch.values[2458] != 0.0) && (scratch.values[2459] != 0.0)) {
            scratch.store_ad(534, &AdValue::scale(scratch.ad_value(855), (scratch.values[427] * scratch.values[746])));
        }

        if ((scratch.values[2458] != 0.0) && (scratch.values[2459] != 0.0)) {
            let assign51300_ad_e65961: AdValue = {
                if (scratch.values[534] < (-230.25850929940458)) {
                    AdValue::div_from_scalar(1e-100, AdValue::offset(AdValue::sub_from_scalar((-230.25850929940458), scratch.ad_value(534)), 1.0))
                } else {
                    {
                        if (scratch.values[534] > scratch.values[738]) {
                            AdValue::mul(scratch.ad_value(739), AdValue::offset(AdValue::sub(scratch.ad_value(534), scratch.ad_value(738)), 1.0))
                        } else {
                            AdValue::exp(scratch.ad_value(534))
                        }
                    }
                }
            };
            scratch.store_ad(535, &assign51300_ad_e65961);
        }

        if ((scratch.values[2458] != 0.0) && (scratch.values[2459] != 0.0)) {
            scratch.store_ad(540, &AdValue::mul(scratch.ad_value(745), AdValue::offset(scratch.ad_value(535), (-1.0))));
        }

        if ((scratch.values[2458] != 0.0) && (scratch.values[2459] != 0.0)) {
            scratch.store_ad(534, &AdValue::mul(AdValue::scale(scratch.ad_value(855), scratch.values[427]), scratch.ad_value(748)));
        }

        if ((scratch.values[2458] != 0.0) && (scratch.values[2459] != 0.0)) {
            let assign51330_ad_e66012: AdValue = {
                if (scratch.values[534] < (-230.25850929940458)) {
                    AdValue::div_from_scalar(1e-100, AdValue::offset(AdValue::sub_from_scalar((-230.25850929940458), scratch.ad_value(534)), 1.0))
                } else {
                    {
                        if (scratch.values[534] > scratch.values[740]) {
                            AdValue::mul(scratch.ad_value(741), AdValue::offset(AdValue::sub(scratch.ad_value(534), scratch.ad_value(740)), 1.0))
                        } else {
                            AdValue::exp(scratch.ad_value(534))
                        }
                    }
                }
            };
            scratch.store_ad(535, &assign51330_ad_e66012);
        }

        if ((scratch.values[2458] != 0.0) && (scratch.values[2459] != 0.0)) {
            scratch.store_ad(541, &AdValue::mul(scratch.ad_value(747), AdValue::offset(scratch.ad_value(535), (-1.0))));
        }

        if ((scratch.values[2458] != 0.0) && (scratch.values[2459] != 0.0)) {
            scratch.values[542] = 0.0;
            scratch.node_derivatives[542] = [0.0; Instance::NODE_COUNT];
            scratch.branch_derivatives[542] = [0.0; Instance::BRANCH_COUNT];
        }

        scratch.values[2461] = if (scratch.values[744] > 0.0) { 1.0 } else { 0.0 };

        if (((scratch.values[2458] != 0.0) && (scratch.values[2459] != 0.0)) && (scratch.values[2461] != 0.0)) {
            scratch.store_ad(542, &AdValue::mul(scratch.ad_value(855), AdValue::add(scratch.ad_value(749), AdValue::mul(scratch.ad_value(855), scratch.ad_value(750)))));
        }

        if (((scratch.values[2458] != 0.0) && (scratch.values[2459] != 0.0)) && (!(scratch.values[2461] != 0.0))) {
            scratch.store_ad(534, &AdValue::mul(AdValue::scale(AdValue::neg(scratch.ad_value(855)), scratch.values[427]), scratch.ad_value(750)));
        }

        if (((scratch.values[2458] != 0.0) && (scratch.values[2459] != 0.0)) && (!(scratch.values[2461] != 0.0))) {
            let assign51390_ad_e66093: AdValue = {
                if (scratch.values[534] < (-230.25850929940458)) {
                    AdValue::div_from_scalar(1e-100, AdValue::offset(AdValue::sub_from_scalar((-230.25850929940458), scratch.ad_value(534)), 1.0))
                } else {
                    {
                        if (scratch.values[534] > scratch.values[742]) {
                            AdValue::mul(scratch.ad_value(743), AdValue::offset(AdValue::sub(scratch.ad_value(534), scratch.ad_value(742)), 1.0))
                        } else {
                            AdValue::exp(scratch.ad_value(534))
                        }
                    }
                }
            };
            scratch.store_ad(535, &assign51390_ad_e66093);
        }

        if (((scratch.values[2458] != 0.0) && (scratch.values[2459] != 0.0)) && (!(scratch.values[2461] != 0.0))) {
            scratch.store_ad(542, &AdValue::mul(AdValue::neg(scratch.ad_value(749)), AdValue::offset(scratch.ad_value(535), (-1.0))));
        }

        if ((scratch.values[2458] != 0.0) && (scratch.values[2459] != 0.0)) {
            scratch.store_ad(2034, &AdValue::add(AdValue::add(scratch.ad_value(540), scratch.ad_value(541)), scratch.ad_value(542)));
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

    }

    pub(super) fn stamp_transient_block_39(
        &mut self,
        ctx: &GeneratedEvalContext<'_>,
        stamper: &mut GeneratedStamper<'_>,
        scratch: &mut Scratch,
    ) {
        let _ = stamper;
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
            scratch.values[2031] = 0.0;
            scratch.node_derivatives[2031] = [0.0; Instance::NODE_COUNT];
            scratch.branch_derivatives[2031] = [0.0; Instance::BRANCH_COUNT];
        }

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

        if (((scratch.values[2458] != 0.0) && (!(scratch.values[2459] != 0.0))) && (!(scratch.values[2481] != 0.0))) {
            scratch.store_ad(671, &AdValue::scale(scratch.ad_value(661), scratch.values[443]));
        }

        scratch.values[2483] = if ((scratch.values[393] == 0.0) && (scratch.values[396] == 0.0)) { 1.0 } else { 0.0 };

        if ((((scratch.values[2458] != 0.0) && (!(scratch.values[2459] != 0.0))) && (!(scratch.values[2481] != 0.0))) && (scratch.values[2483] != 0.0)) {
            scratch.values[672] = 0.0;
            scratch.node_derivatives[672] = [0.0; Instance::NODE_COUNT];
            scratch.branch_derivatives[672] = [0.0; Instance::BRANCH_COUNT];
        }

        if ((((scratch.values[2458] != 0.0) && (!(scratch.values[2459] != 0.0))) && (!(scratch.values[2481] != 0.0))) && (!(scratch.values[2483] != 0.0))) {
            scratch.store_ad(673, &AdValue::sub_from_scalar(scratch.values[449], scratch.ad_value(667)));
        }

        if ((((scratch.values[2458] != 0.0) && (!(scratch.values[2459] != 0.0))) && (!(scratch.values[2481] != 0.0))) && (!(scratch.values[2483] != 0.0))) {
            scratch.store_ad(674, &AdValue::sub_from_scalar(1.0, AdValue::sqrt(AdValue::sub_from_scalar(1.0, AdValue::div(scratch.ad_value(665), scratch.ad_value(673))))));
        }

        scratch.values[2484] = if (scratch.values[382] == 0.5) { 1.0 } else { 0.0 };

        if (((((scratch.values[2458] != 0.0) && (!(scratch.values[2459] != 0.0))) && (!(scratch.values[2481] != 0.0))) && (!(scratch.values[2483] != 0.0))) && (scratch.values[2484] != 0.0)) {
            scratch.values[675] = 0.0;
            scratch.node_derivatives[675] = [0.0; Instance::NODE_COUNT];
            scratch.branch_derivatives[675] = [0.0; Instance::BRANCH_COUNT];
        }

        if (((((scratch.values[2458] != 0.0) && (!(scratch.values[2459] != 0.0))) && (!(scratch.values[2481] != 0.0))) && (!(scratch.values[2483] != 0.0))) && (!(scratch.values[2484] != 0.0))) {
            scratch.store_ad(675, &AdValue::scale(AdValue::add(AdValue::div(AdValue::mul(AdValue::square(scratch.ad_value(674)), AdValue::ln(scratch.ad_value(674))), AdValue::sub_from_scalar(1.0, scratch.ad_value(674))), scratch.ad_value(674)), (1.0 - (2.0 * scratch.values[382]))));
        }

        if ((((scratch.values[2458] != 0.0) && (!(scratch.values[2459] != 0.0))) && (!(scratch.values[2481] != 0.0))) && (!(scratch.values[2483] != 0.0))) {
            scratch.store_ad(676, &AdValue::add(scratch.ad_value(674), scratch.ad_value(675)));
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

        if ((((scratch.values[2458] != 0.0) && (!(scratch.values[2459] != 0.0))) && (!(scratch.values[2481] != 0.0))) && (!(scratch.values[2483] != 0.0))) {
            scratch.store_ad(678, &AdValue::scale(AdValue::mul(AdValue::offset(scratch.ad_value(664), (-1.0)), scratch.ad_value(677)), scratch.values[440]));
        }

        if ((((scratch.values[2458] != 0.0) && (!(scratch.values[2459] != 0.0))) && (!(scratch.values[2481] != 0.0))) && (!(scratch.values[2483] != 0.0))) {
            scratch.store_ad(672, &AdValue::scale(AdValue::mul(scratch.ad_value(678), scratch.ad_value(676)), scratch.values[393]));
        }

        scratch.values[2486] = if (scratch.values[396] == 0.0) { 1.0 } else { 0.0 };

        if ((((scratch.values[2458] != 0.0) && (!(scratch.values[2459] != 0.0))) && (!(scratch.values[2481] != 0.0))) && (scratch.values[2486] != 0.0)) {
            scratch.values[679] = 0.0;
            scratch.node_derivatives[679] = [0.0; Instance::NODE_COUNT];
            scratch.branch_derivatives[679] = [0.0; Instance::BRANCH_COUNT];
        }

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

        scratch.values[2487] = if (((-scratch.values[382]) * scratch.values[467]) == (-1.0)) { 1.0 } else { 0.0 };

        if (((((scratch.values[2458] != 0.0) && (!(scratch.values[2459] != 0.0))) && (!(scratch.values[2481] != 0.0))) && (!(scratch.values[2486] != 0.0))) && (scratch.values[2487] != 0.0)) {
            scratch.store_ad(686, &AdValue::div_from_scalar(1.0, AdValue::offset(AdValue::mul(scratch.ad_value(680), scratch.ad_value(685)), 1.0)));
        }

        if (((((scratch.values[2458] != 0.0) && (!(scratch.values[2459] != 0.0))) && (!(scratch.values[2481] != 0.0))) && (!(scratch.values[2486] != 0.0))) && (!(scratch.values[2487] != 0.0))) {
            scratch.store_ad(686, &AdValue::powf(AdValue::offset(AdValue::mul(scratch.ad_value(680), scratch.ad_value(685)), 1.0), ((-scratch.values[382]) * scratch.values[467])));
        }

        if ((((scratch.values[2458] != 0.0) && (!(scratch.values[2459] != 0.0))) && (!(scratch.values[2481] != 0.0))) && (!(scratch.values[2486] != 0.0))) {
            scratch.store_ad(687, &AdValue::div(AdValue::mul(scratch.ad_value(676), scratch.ad_value(686)), AdValue::add(scratch.ad_value(676), scratch.ad_value(686))));
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

        scratch.values[2488] = if (scratch.values[691] > 0.0) { 1.0 } else { 0.0 };

        if (((((scratch.values[2458] != 0.0) && (!(scratch.values[2459] != 0.0))) && (!(scratch.values[2481] != 0.0))) && (!(scratch.values[2486] != 0.0))) && (scratch.values[2488] != 0.0)) {
            scratch.store_ad(653, &AdValue::div_from_scalar(1.0, AdValue::offset(AdValue::scale(scratch.ad_value(691), scratch.values[428]), 1.0)));
        }

        if (((((scratch.values[2458] != 0.0) && (!(scratch.values[2459] != 0.0))) && (!(scratch.values[2481] != 0.0))) && (!(scratch.values[2486] != 0.0))) && (!(scratch.values[2488] != 0.0))) {
            scratch.store_ad(653, &AdValue::div_from_scalar(1.0, AdValue::sub_from_scalar(1.0, AdValue::scale(scratch.ad_value(691), scratch.values[428]))));
        }

        scratch.values[2489] = if (((-scratch.values[652]) + scratch.values[690]) > (-230.25850929940458)) { 1.0 } else { 0.0 };

        if (((((scratch.values[2458] != 0.0) && (!(scratch.values[2459] != 0.0))) && (!(scratch.values[2481] != 0.0))) && (!(scratch.values[2486] != 0.0))) && (scratch.values[2489] != 0.0)) {
            scratch.store_ad(670, &AdValue::exp(AdValue::sub(scratch.ad_value(690), scratch.ad_value(652))));
        }

        if (((((scratch.values[2458] != 0.0) && (!(scratch.values[2459] != 0.0))) && (!(scratch.values[2481] != 0.0))) && (!(scratch.values[2486] != 0.0))) && (!(scratch.values[2489] != 0.0))) {
            scratch.store_ad(670, &AdValue::div_from_scalar(1e-100, AdValue::offset(AdValue::mul(AdValue::sub_from_scalar((-230.25850929940458), AdValue::sub(scratch.ad_value(690), scratch.ad_value(652))), AdValue::offset(AdValue::scale(AdValue::mul(AdValue::sub_from_scalar((-230.25850929940458), AdValue::sub(scratch.ad_value(690), scratch.ad_value(652))), AdValue::offset(AdValue::scale(AdValue::sub_from_scalar((-230.25850929940458), AdValue::sub(scratch.ad_value(690), scratch.ad_value(652))), 0.3333333333333333), 1.0)), 0.5), 1.0)), 1.0)));
        }

        if ((((scratch.values[2458] != 0.0) && (!(scratch.values[2459] != 0.0))) && (!(scratch.values[2481] != 0.0))) && (!(scratch.values[2486] != 0.0))) {
            scratch.store_ad(654, &AdValue::mul(AdValue::add(AdValue::add(AdValue::scale(scratch.ad_value(653), 0.29214664), AdValue::scale(AdValue::square(scratch.ad_value(653)), scratch.values[429])), AdValue::scale(AdValue::mul(AdValue::square(scratch.ad_value(653)), scratch.ad_value(653)), scratch.values[430])), scratch.ad_value(670)));
        }

        scratch.values[2490] = if (scratch.values[691] > 0.0) { 1.0 } else { 0.0 };

        if (((((scratch.values[2458] != 0.0) && (!(scratch.values[2459] != 0.0))) && (!(scratch.values[2481] != 0.0))) && (!(scratch.values[2486] != 0.0))) && (scratch.values[2490] != 0.0)) {
            scratch.values[692] = scratch.values[654];
            scratch.node_derivatives[692] = scratch.node_derivatives[654];
            scratch.branch_derivatives[692] = scratch.branch_derivatives[654];
        }

        scratch.values[2491] = if (scratch.values[690] > (-230.25850929940458)) { 1.0 } else { 0.0 };

        if ((((((scratch.values[2458] != 0.0) && (!(scratch.values[2459] != 0.0))) && (!(scratch.values[2481] != 0.0))) && (!(scratch.values[2486] != 0.0))) && (!(scratch.values[2490] != 0.0))) && (scratch.values[2491] != 0.0)) {
            scratch.store_ad(670, &AdValue::exp(scratch.ad_value(690)));
        }

        if ((((((scratch.values[2458] != 0.0) && (!(scratch.values[2459] != 0.0))) && (!(scratch.values[2481] != 0.0))) && (!(scratch.values[2486] != 0.0))) && (!(scratch.values[2490] != 0.0))) && (!(scratch.values[2491] != 0.0))) {
            scratch.store_ad(670, &AdValue::div_from_scalar(1e-100, AdValue::offset(AdValue::mul(AdValue::sub_from_scalar((-230.25850929940458), scratch.ad_value(690)), AdValue::offset(AdValue::scale(AdValue::mul(AdValue::sub_from_scalar((-230.25850929940458), scratch.ad_value(690)), AdValue::offset(AdValue::scale(AdValue::sub_from_scalar((-230.25850929940458), scratch.ad_value(690)), 0.3333333333333333), 1.0)), 0.5), 1.0)), 1.0)));
        }

        if (((((scratch.values[2458] != 0.0) && (!(scratch.values[2459] != 0.0))) && (!(scratch.values[2481] != 0.0))) && (!(scratch.values[2486] != 0.0))) && (!(scratch.values[2490] != 0.0))) {
            scratch.store_ad(692, &AdValue::sub(AdValue::scale(scratch.ad_value(670), 2.0), scratch.ad_value(654)));
        }

        if ((((scratch.values[2458] != 0.0) && (!(scratch.values[2459] != 0.0))) && (!(scratch.values[2481] != 0.0))) && (!(scratch.values[2486] != 0.0))) {
            scratch.store_ad(693, &AdValue::scale(AdValue::div(AdValue::scale(scratch.ad_value(692), scratch.values[491]), scratch.ad_value(688)), (1.772453850905516 * 0.5)));
        }

        if ((((scratch.values[2458] != 0.0) && (!(scratch.values[2459] != 0.0))) && (!(scratch.values[2481] != 0.0))) && (!(scratch.values[2486] != 0.0))) {
            scratch.store_ad(679, &AdValue::scale(AdValue::mul(AdValue::mul(scratch.ad_value(678), scratch.ad_value(693)), scratch.ad_value(687)), scratch.values[396]));
        }

        scratch.values[2492] = if (scratch.values[402] == 0.0) { 1.0 } else { 0.0 };

        if ((((scratch.values[2458] != 0.0) && (!(scratch.values[2459] != 0.0))) && (!(scratch.values[2481] != 0.0))) && (scratch.values[2492] != 0.0)) {
            scratch.values[694] = 0.0;
            scratch.node_derivatives[694] = [0.0; Instance::NODE_COUNT];
            scratch.branch_derivatives[694] = [0.0; Instance::BRANCH_COUNT];
        }

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

        if ((((scratch.values[2458] != 0.0) && (!(scratch.values[2459] != 0.0))) && (!(scratch.values[2481] != 0.0))) && (!(scratch.values[2492] != 0.0))) {
            scratch.store_ad(694, &AdValue::scale(AdValue::mul(AdValue::mul(AdValue::mul(scratch.ad_value(854), scratch.ad_value(695)), scratch.ad_value(695)), scratch.ad_value(670)), scratch.values[402]));
        }

        scratch.values[2496] = if (scratch.values[411] > 1000.0) { 1.0 } else { 0.0 };

        if ((((scratch.values[2458] != 0.0) && (!(scratch.values[2459] != 0.0))) && (!(scratch.values[2481] != 0.0))) && (scratch.values[2496] != 0.0)) {
            scratch.values[696] = 1.0;
            scratch.node_derivatives[696] = [0.0; Instance::NODE_COUNT];
            scratch.branch_derivatives[696] = [0.0; Instance::BRANCH_COUNT];
        }

        scratch.values[2497] = if (scratch.values[669] > ((-scratch.values[500]) * scratch.values[411])) { 1.0 } else { 0.0 };

        scratch.values[2498] = if (scratch.values[414] == 4.0) { 1.0 } else { 0.0 };

        if ((((((scratch.values[2458] != 0.0) && (!(scratch.values[2459] != 0.0))) && (!(scratch.values[2481] != 0.0))) && (!(scratch.values[2496] != 0.0))) && (scratch.values[2497] != 0.0)) && (scratch.values[2498] != 0.0)) {
            scratch.store_ad(670, &AdValue::mul(AdValue::mul(AdValue::mul(AdValue::scale(scratch.ad_value(669), scratch.values[504]), AdValue::scale(scratch.ad_value(669), scratch.values[504])), AdValue::scale(scratch.ad_value(669), scratch.values[504])), AdValue::scale(scratch.ad_value(669), scratch.values[504])));
        }

        if ((((((scratch.values[2458] != 0.0) && (!(scratch.values[2459] != 0.0))) && (!(scratch.values[2481] != 0.0))) && (!(scratch.values[2496] != 0.0))) && (scratch.values[2497] != 0.0)) && (!(scratch.values[2498] != 0.0))) {
            scratch.store_ad(670, &AdValue::powf(AdValue::abs(AdValue::scale(scratch.ad_value(669), scratch.values[504])), scratch.values[414]));
        }

        if (((((scratch.values[2458] != 0.0) && (!(scratch.values[2459] != 0.0))) && (!(scratch.values[2481] != 0.0))) && (!(scratch.values[2496] != 0.0))) && (scratch.values[2497] != 0.0)) {
            scratch.store_ad(696, &AdValue::div_from_scalar(1.0, AdValue::sub_from_scalar(1.0, scratch.ad_value(670))));
        }

        if (((((scratch.values[2458] != 0.0) && (!(scratch.values[2459] != 0.0))) && (!(scratch.values[2481] != 0.0))) && (!(scratch.values[2496] != 0.0))) && (!(scratch.values[2497] != 0.0))) {
            scratch.store_ad(696, &AdValue::offset(AdValue::scale(AdValue::offset(scratch.ad_value(669), (scratch.values[500] * scratch.values[411])), scratch.values[507]), scratch.values[501]));
        }

        if (((scratch.values[2458] != 0.0) && (!(scratch.values[2459] != 0.0))) && (!(scratch.values[2481] != 0.0))) {
            scratch.store_ad(2031, &AdValue::mul(AdValue::add(AdValue::add(AdValue::add(scratch.ad_value(671), scratch.ad_value(672)), scratch.ad_value(679)), scratch.ad_value(694)), scratch.ad_value(696)));
        }

        scratch.values[2499] = if (scratch.values[698] == 0.0) { 1.0 } else { 0.0 };

        if (((scratch.values[2458] != 0.0) && (!(scratch.values[2459] != 0.0))) && (scratch.values[2499] != 0.0)) {
            scratch.values[2032] = 0.0;
            scratch.node_derivatives[2032] = [0.0; Instance::NODE_COUNT];
            scratch.branch_derivatives[2032] = [0.0; Instance::BRANCH_COUNT];
        }

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

        if (((scratch.values[2458] != 0.0) && (!(scratch.values[2459] != 0.0))) && (!(scratch.values[2499] != 0.0))) {
            scratch.store_ad(671, &AdValue::scale(scratch.ad_value(661), scratch.values[444]));
        }

        scratch.values[2501] = if ((scratch.values[394] == 0.0) && (scratch.values[397] == 0.0)) { 1.0 } else { 0.0 };

        if ((((scratch.values[2458] != 0.0) && (!(scratch.values[2459] != 0.0))) && (!(scratch.values[2499] != 0.0))) && (scratch.values[2501] != 0.0)) {
            scratch.values[672] = 0.0;
            scratch.node_derivatives[672] = [0.0; Instance::NODE_COUNT];
            scratch.branch_derivatives[672] = [0.0; Instance::BRANCH_COUNT];
        }

        if ((((scratch.values[2458] != 0.0) && (!(scratch.values[2459] != 0.0))) && (!(scratch.values[2499] != 0.0))) && (!(scratch.values[2501] != 0.0))) {
            scratch.store_ad(673, &AdValue::sub_from_scalar(scratch.values[450], scratch.ad_value(667)));
        }

        if ((((scratch.values[2458] != 0.0) && (!(scratch.values[2459] != 0.0))) && (!(scratch.values[2499] != 0.0))) && (!(scratch.values[2501] != 0.0))) {
            scratch.store_ad(674, &AdValue::sub_from_scalar(1.0, AdValue::sqrt(AdValue::sub_from_scalar(1.0, AdValue::div(scratch.ad_value(665), scratch.ad_value(673))))));
        }

        scratch.values[2502] = if (scratch.values[383] == 0.5) { 1.0 } else { 0.0 };

        if (((((scratch.values[2458] != 0.0) && (!(scratch.values[2459] != 0.0))) && (!(scratch.values[2499] != 0.0))) && (!(scratch.values[2501] != 0.0))) && (scratch.values[2502] != 0.0)) {
            scratch.values[675] = 0.0;
            scratch.node_derivatives[675] = [0.0; Instance::NODE_COUNT];
            scratch.branch_derivatives[675] = [0.0; Instance::BRANCH_COUNT];
        }

        if (((((scratch.values[2458] != 0.0) && (!(scratch.values[2459] != 0.0))) && (!(scratch.values[2499] != 0.0))) && (!(scratch.values[2501] != 0.0))) && (!(scratch.values[2502] != 0.0))) {
            scratch.store_ad(675, &AdValue::scale(AdValue::add(AdValue::div(AdValue::mul(AdValue::square(scratch.ad_value(674)), AdValue::ln(scratch.ad_value(674))), AdValue::sub_from_scalar(1.0, scratch.ad_value(674))), scratch.ad_value(674)), (1.0 - (2.0 * scratch.values[383]))));
        }

        if ((((scratch.values[2458] != 0.0) && (!(scratch.values[2459] != 0.0))) && (!(scratch.values[2499] != 0.0))) && (!(scratch.values[2501] != 0.0))) {
            scratch.store_ad(676, &AdValue::add(scratch.ad_value(674), scratch.ad_value(675)));
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

        if ((((scratch.values[2458] != 0.0) && (!(scratch.values[2459] != 0.0))) && (!(scratch.values[2499] != 0.0))) && (!(scratch.values[2501] != 0.0))) {
            scratch.store_ad(678, &AdValue::scale(AdValue::mul(AdValue::offset(scratch.ad_value(664), (-1.0)), scratch.ad_value(677)), scratch.values[441]));
        }

        if ((((scratch.values[2458] != 0.0) && (!(scratch.values[2459] != 0.0))) && (!(scratch.values[2499] != 0.0))) && (!(scratch.values[2501] != 0.0))) {
            scratch.store_ad(672, &AdValue::scale(AdValue::mul(scratch.ad_value(678), scratch.ad_value(676)), scratch.values[394]));
        }

        scratch.values[2504] = if (scratch.values[397] == 0.0) { 1.0 } else { 0.0 };

        if ((((scratch.values[2458] != 0.0) && (!(scratch.values[2459] != 0.0))) && (!(scratch.values[2499] != 0.0))) && (scratch.values[2504] != 0.0)) {
            scratch.values[679] = 0.0;
            scratch.node_derivatives[679] = [0.0; Instance::NODE_COUNT];
            scratch.branch_derivatives[679] = [0.0; Instance::BRANCH_COUNT];
        }

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

        scratch.values[2505] = if (((-scratch.values[383]) * scratch.values[468]) == (-1.0)) { 1.0 } else { 0.0 };

        if (((((scratch.values[2458] != 0.0) && (!(scratch.values[2459] != 0.0))) && (!(scratch.values[2499] != 0.0))) && (!(scratch.values[2504] != 0.0))) && (scratch.values[2505] != 0.0)) {
            scratch.store_ad(686, &AdValue::div_from_scalar(1.0, AdValue::offset(AdValue::mul(scratch.ad_value(680), scratch.ad_value(685)), 1.0)));
        }

        if (((((scratch.values[2458] != 0.0) && (!(scratch.values[2459] != 0.0))) && (!(scratch.values[2499] != 0.0))) && (!(scratch.values[2504] != 0.0))) && (!(scratch.values[2505] != 0.0))) {
            scratch.store_ad(686, &AdValue::powf(AdValue::offset(AdValue::mul(scratch.ad_value(680), scratch.ad_value(685)), 1.0), ((-scratch.values[383]) * scratch.values[468])));
        }

        if ((((scratch.values[2458] != 0.0) && (!(scratch.values[2459] != 0.0))) && (!(scratch.values[2499] != 0.0))) && (!(scratch.values[2504] != 0.0))) {
            scratch.store_ad(687, &AdValue::div(AdValue::mul(scratch.ad_value(676), scratch.ad_value(686)), AdValue::add(scratch.ad_value(676), scratch.ad_value(686))));
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

    }
}
