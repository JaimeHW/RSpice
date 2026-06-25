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
        if ((scratch.values[2404] != 0.0) && (scratch.values[2417] != 0.0)) {
            scratch.store_ad(2352, &AdValue::sub(scratch.ad_value(2294), scratch.ad_value(2305)));
        }

        if ((scratch.values[2404] != 0.0) && (scratch.values[2417] != 0.0)) {
            scratch.store_ad(2376, &AdValue::mul(scratch.ad_value(2352), scratch.ad_value(2011)));
        }

        if ((scratch.values[2404] != 0.0) && (scratch.values[2417] != 0.0)) {
            scratch.values[2368] = scratch.values[2376];
            scratch.node_derivatives[2368] = scratch.node_derivatives[2376];
            scratch.branch_derivatives[2368] = scratch.branch_derivatives[2376];
        }

        if ((scratch.values[2404] != 0.0) && (scratch.values[2417] != 0.0)) {
            scratch.values[870] = scratch.values[869];
            scratch.node_derivatives[870] = scratch.node_derivatives[869];
            scratch.branch_derivatives[870] = scratch.branch_derivatives[869];
        }

        if ((scratch.values[2404] != 0.0) && (scratch.values[2417] != 0.0)) {
            scratch.values[2334] = scratch.values[854];
            scratch.node_derivatives[2334] = scratch.node_derivatives[854];
            scratch.branch_derivatives[2334] = scratch.branch_derivatives[854];
        }

        if ((scratch.values[2404] != 0.0) && (!(scratch.values[2417] != 0.0))) {
            scratch.values[2306] = 0.0;
            scratch.node_derivatives[2306] = [0.0; Instance::NODE_COUNT];
            scratch.branch_derivatives[2306] = [0.0; Instance::BRANCH_COUNT];
        }

        if ((scratch.values[2404] != 0.0) && (!(scratch.values[2417] != 0.0))) {
            scratch.store_ad(2076, &AdValue::div_from_scalar(1.0, AdValue::offset(AdValue::square(scratch.ad_value(2305)), 2.0)));
        }

        if ((scratch.values[2404] != 0.0) && (!(scratch.values[2417] != 0.0))) {
            scratch.store_ad(2307, &AdValue::mul(AdValue::square(scratch.ad_value(2305)), scratch.ad_value(2076)));
        }

        if ((scratch.values[2404] != 0.0) && (!(scratch.values[2417] != 0.0))) {
            scratch.store_ad(2308, &AdValue::scale(AdValue::mul(AdValue::mul(scratch.ad_value(2305), scratch.ad_value(2076)), scratch.ad_value(2076)), 4.0));
        }

        if ((scratch.values[2404] != 0.0) && (!(scratch.values[2417] != 0.0))) {
            scratch.store_ad(2309, &AdValue::mul(AdValue::mul(AdValue::sub(AdValue::scale(scratch.ad_value(2076), 8.0), AdValue::scale(scratch.ad_value(2307), 12.0)), scratch.ad_value(2076)), scratch.ad_value(2076)));
        }

        scratch.values[2418] = if (scratch.values[2305] < 230.25850929940458) { 1.0 } else { 0.0 };

        if (((scratch.values[2404] != 0.0) && (!(scratch.values[2417] != 0.0))) && (scratch.values[2418] != 0.0)) {
            scratch.store_ad(2306, &AdValue::exp(scratch.ad_value(2305)));
        }

        if (((scratch.values[2404] != 0.0) && (!(scratch.values[2417] != 0.0))) && (scratch.values[2418] != 0.0)) {
            scratch.store_ad(2310, &AdValue::div_from_scalar(1.0, scratch.ad_value(2306)));
        }

        if (((scratch.values[2404] != 0.0) && (!(scratch.values[2417] != 0.0))) && (scratch.values[2418] != 0.0)) {
            scratch.store_ad(2306, &AdValue::mul(scratch.ad_value(2303), scratch.ad_value(2306)));
        }

        scratch.values[2419] = if (scratch.values[2305] > (scratch.values[2302] - 230.25850929940458)) { 1.0 } else { 0.0 };

        if ((((scratch.values[2404] != 0.0) && (!(scratch.values[2417] != 0.0))) && (!(scratch.values[2418] != 0.0))) && (scratch.values[2419] != 0.0)) {
            scratch.store_ad(2306, &AdValue::exp(AdValue::sub(scratch.ad_value(2305), scratch.ad_value(2302))));
        }

        if ((((scratch.values[2404] != 0.0) && (!(scratch.values[2417] != 0.0))) && (!(scratch.values[2418] != 0.0))) && (scratch.values[2419] != 0.0)) {
            scratch.store_ad(2310, &AdValue::div(scratch.ad_value(2303), scratch.ad_value(2306)));
        }

        if ((((scratch.values[2404] != 0.0) && (!(scratch.values[2417] != 0.0))) && (!(scratch.values[2418] != 0.0))) && (!(scratch.values[2419] != 0.0))) {
            scratch.store_ad(2306, &AdValue::div_from_scalar(1e-100, AdValue::offset(AdValue::mul(AdValue::offset(AdValue::sub(scratch.ad_value(2302), scratch.ad_value(2305)), (-230.25850929940458)), AdValue::offset(AdValue::scale(AdValue::mul(AdValue::offset(AdValue::sub(scratch.ad_value(2302), scratch.ad_value(2305)), (-230.25850929940458)), AdValue::offset(AdValue::scale(AdValue::offset(AdValue::sub(scratch.ad_value(2302), scratch.ad_value(2305)), (-230.25850929940458)), 0.3333333333333333), 1.0)), 0.5), 1.0)), 1.0)));
        }

        if ((((scratch.values[2404] != 0.0) && (!(scratch.values[2417] != 0.0))) && (!(scratch.values[2418] != 0.0))) && (!(scratch.values[2419] != 0.0))) {
            scratch.store_ad(2310, &AdValue::div_from_scalar(1e-100, AdValue::offset(AdValue::mul(AdValue::offset(scratch.ad_value(2305), (-230.25850929940458)), AdValue::offset(AdValue::scale(AdValue::mul(AdValue::offset(scratch.ad_value(2305), (-230.25850929940458)), AdValue::offset(AdValue::scale(AdValue::offset(scratch.ad_value(2305), (-230.25850929940458)), 0.3333333333333333), 1.0)), 0.5), 1.0)), 1.0)));
        }

        if ((scratch.values[2404] != 0.0) && (!(scratch.values[2417] != 0.0))) {
            scratch.store_ad(2311, &AdValue::sub(scratch.ad_value(2306), AdValue::mul(scratch.ad_value(2303), AdValue::add(AdValue::offset(scratch.ad_value(2305), 1.0), scratch.ad_value(2307)))));
        }

        scratch.values[2420] = if (scratch.values[2305] < 1e-5) { 1.0 } else { 0.0 };

        if (((scratch.values[2404] != 0.0) && (!(scratch.values[2417] != 0.0))) && (scratch.values[2420] != 0.0)) {
            scratch.store_ad(2312, &AdValue::scale(AdValue::mul(AdValue::square(scratch.ad_value(2305)), AdValue::sub_from_scalar(1.0, AdValue::scale(AdValue::mul(scratch.ad_value(2305), AdValue::sub_from_scalar(1.0, AdValue::scale(scratch.ad_value(2305), 0.25))), 0.3333333333333333))), 0.5));
        }

        if (((scratch.values[2404] != 0.0) && (!(scratch.values[2417] != 0.0))) && (scratch.values[2420] != 0.0)) {
            scratch.store_ad(2311, &AdValue::scale(AdValue::mul(AdValue::mul(AdValue::mul(AdValue::mul(scratch.ad_value(2303), scratch.ad_value(2305)), scratch.ad_value(2305)), scratch.ad_value(2305)), AdValue::offset(AdValue::scale(scratch.ad_value(2305), 1.75), 1.0)), 0.16666666666666666));
        }

        if (((scratch.values[2404] != 0.0) && (!(scratch.values[2417] != 0.0))) && (scratch.values[2420] != 0.0)) {
            scratch.store_ad(2076, &AdValue::sqrt(AdValue::sub_from_scalar(1.0, AdValue::scale(AdValue::mul(scratch.ad_value(2305), AdValue::sub_from_scalar(1.0, AdValue::scale(scratch.ad_value(2305), 0.25))), 0.3333333333333333))));
        }

        if (((scratch.values[2404] != 0.0) && (!(scratch.values[2417] != 0.0))) && (scratch.values[2420] != 0.0)) {
            scratch.store_ad(2354, &AdValue::scale(AdValue::mul(scratch.ad_value(2305), scratch.ad_value(2076)), 0.7071067811865475));
        }

        if (((scratch.values[2404] != 0.0) && (!(scratch.values[2417] != 0.0))) && (scratch.values[2420] != 0.0)) {
            scratch.store_ad(2355, &AdValue::offset(AdValue::div(AdValue::mul(AdValue::scale(scratch.ad_value(2296), 0.7071067811865475), AdValue::add(AdValue::sub_from_scalar(1.0, AdValue::scale(scratch.ad_value(2305), 0.5)), AdValue::scale(AdValue::square(scratch.ad_value(2305)), 0.16666666666666666))), scratch.ad_value(2076)), 1.0));
        }

        if (((scratch.values[2404] != 0.0) && (!(scratch.values[2417] != 0.0))) && (!(scratch.values[2420] != 0.0))) {
            scratch.store_ad(2312, &AdValue::add(AdValue::offset(scratch.ad_value(2305), (-1.0)), scratch.ad_value(2310)));
        }

        if (((scratch.values[2404] != 0.0) && (!(scratch.values[2417] != 0.0))) && (!(scratch.values[2420] != 0.0))) {
            scratch.store_ad(2354, &AdValue::sqrt(scratch.ad_value(2312)));
        }

        if (((scratch.values[2404] != 0.0) && (!(scratch.values[2417] != 0.0))) && (!(scratch.values[2420] != 0.0))) {
            scratch.store_ad(2355, &AdValue::offset(AdValue::scale(AdValue::div(AdValue::mul(scratch.ad_value(2296), AdValue::sub_from_scalar(1.0, scratch.ad_value(2310))), scratch.ad_value(2354)), 0.5), 1.0));
        }

        if ((scratch.values[2404] != 0.0) && (!(scratch.values[2417] != 0.0))) {
            scratch.values[2348] = scratch.values[2310];
            scratch.node_derivatives[2348] = scratch.node_derivatives[2310];
            scratch.branch_derivatives[2348] = scratch.branch_derivatives[2310];
        }

        if ((scratch.values[2404] != 0.0) && (!(scratch.values[2417] != 0.0))) {
            scratch.values[2345] = scratch.values[2348];
            scratch.node_derivatives[2345] = scratch.node_derivatives[2348];
            scratch.branch_derivatives[2345] = scratch.branch_derivatives[2348];
        }

        if ((scratch.values[2404] != 0.0) && (!(scratch.values[2417] != 0.0))) {
            scratch.values[2350] = scratch.values[2311];
            scratch.node_derivatives[2350] = scratch.node_derivatives[2311];
            scratch.branch_derivatives[2350] = scratch.branch_derivatives[2311];
        }

        if ((scratch.values[2404] != 0.0) && (!(scratch.values[2417] != 0.0))) {
            scratch.values[2346] = scratch.values[2350];
            scratch.node_derivatives[2346] = scratch.node_derivatives[2350];
            scratch.branch_derivatives[2346] = scratch.branch_derivatives[2350];
        }

        if ((scratch.values[2404] != 0.0) && (!(scratch.values[2417] != 0.0))) {
            scratch.store_ad(2313, &AdValue::div(AdValue::offset(AdValue::mul(AdValue::scale(scratch.ad_value(752), 0.2), scratch.ad_value(2293)), 1.0), AdValue::offset(AdValue::mul(scratch.ad_value(752), scratch.ad_value(2293)), 1.0)));
        }

        scratch.values[2421] = if (scratch.values[2311] > 1e-100) { 1.0 } else { 0.0 };

        if (((scratch.values[2404] != 0.0) && (!(scratch.values[2417] != 0.0))) && (scratch.values[2421] != 0.0)) {
            scratch.store_ad(2314, &AdValue::mul(scratch.ad_value(2296), AdValue::sqrt(AdValue::add(scratch.ad_value(2312), scratch.ad_value(2311)))));
        }

        if (((scratch.values[2404] != 0.0) && (!(scratch.values[2417] != 0.0))) && (scratch.values[2421] != 0.0)) {
            scratch.store_ad(2315, &AdValue::div(AdValue::mul(AdValue::mul(scratch.ad_value(2297), scratch.ad_value(2311)), scratch.ad_value(2011)), AdValue::add(scratch.ad_value(2314), AdValue::mul(scratch.ad_value(2296), scratch.ad_value(2354)))));
        }

        if (((scratch.values[2404] != 0.0) && (!(scratch.values[2417] != 0.0))) && (scratch.values[2421] != 0.0)) {
            scratch.store_ad(2316, &AdValue::mul(AdValue::mul(scratch.ad_value(2354), scratch.ad_value(2296)), scratch.ad_value(2011)));
        }

        scratch.values[2422] = if (scratch.values[234] < 0.0) { 1.0 } else { 0.0 };

        if ((((scratch.values[2404] != 0.0) && (!(scratch.values[2417] != 0.0))) && (scratch.values[2421] != 0.0)) && (scratch.values[2422] != 0.0)) {
            scratch.store_ad(2317, &AdValue::div_from_scalar(1.0, AdValue::sub_from_scalar(1.0, AdValue::mul(scratch.ad_value(234), scratch.ad_value(2293)))));
        }

        if ((((scratch.values[2404] != 0.0) && (!(scratch.values[2417] != 0.0))) && (scratch.values[2421] != 0.0)) && (!(scratch.values[2422] != 0.0))) {
            scratch.store_ad(2317, &AdValue::offset(AdValue::mul(scratch.ad_value(234), scratch.ad_value(2293)), 1.0));
        }

        scratch.values[2423] = if (scratch.values[235] < 0.0) { 1.0 } else { 0.0 };

        if ((((scratch.values[2404] != 0.0) && (!(scratch.values[2417] != 0.0))) && (scratch.values[2421] != 0.0)) && (scratch.values[2423] != 0.0)) {
            scratch.store_ad(2076, &AdValue::sub_from_scalar(1.0, AdValue::mul(scratch.ad_value(235), scratch.ad_value(2315))));
        }

        if ((((scratch.values[2404] != 0.0) && (!(scratch.values[2417] != 0.0))) && (scratch.values[2421] != 0.0)) && (!(scratch.values[2423] != 0.0))) {
            scratch.store_ad(2076, &AdValue::div_from_scalar(1.0, AdValue::offset(AdValue::mul(scratch.ad_value(235), scratch.ad_value(2315)), 1.0)));
        }

        if (((scratch.values[2404] != 0.0) && (!(scratch.values[2417] != 0.0))) && (scratch.values[2421] != 0.0)) {
            scratch.store_ad(2318, &AdValue::mul(scratch.ad_value(800), AdValue::mul(AdValue::mul(scratch.ad_value(2317), scratch.ad_value(2076)), scratch.ad_value(2315))));
        }

        if (((scratch.values[2404] != 0.0) && (!(scratch.values[2417] != 0.0))) && (scratch.values[2421] != 0.0)) {
            scratch.store_ad(2319, &AdValue::mul(scratch.ad_value(817), AdValue::add(scratch.ad_value(2316), AdValue::mul(scratch.ad_value(818), scratch.ad_value(2315)))));
        }

        if (((scratch.values[2404] != 0.0) && (!(scratch.values[2417] != 0.0))) && (scratch.values[2421] != 0.0)) {
            scratch.store_ad(2077, &AdValue::ln(AdValue::div(scratch.ad_value(2312), AdValue::offset(AdValue::add(scratch.ad_value(2312), scratch.ad_value(2311)), 1e-14))));
        }

        if (((scratch.values[2404] != 0.0) && (!(scratch.values[2417] != 0.0))) && (scratch.values[2421] != 0.0)) {
            scratch.store_ad(2320, &AdValue::add(AdValue::pow(AdValue::mul(scratch.ad_value(2319), scratch.ad_value(748)), scratch.ad_value(749)), AdValue::mul(scratch.ad_value(750), AdValue::exp(AdValue::mul(AdValue::scale(scratch.ad_value(751), 0.5), scratch.ad_value(2077))))));
        }

        if (((scratch.values[2404] != 0.0) && (!(scratch.values[2417] != 0.0))) && (scratch.values[2421] != 0.0)) {
            scratch.store_ad(2321, &AdValue::mul(AdValue::add(AdValue::offset(scratch.ad_value(2320), 1.0), scratch.ad_value(2318)), scratch.ad_value(2313)));
        }

        scratch.values[2424] = if (scratch.values[238] < 0.0) { 1.0 } else { 0.0 };

        if ((((scratch.values[2404] != 0.0) && (!(scratch.values[2417] != 0.0))) && (scratch.values[2421] != 0.0)) && (scratch.values[2424] != 0.0)) {
            scratch.store_ad(2322, &AdValue::div_from_scalar(1.0, AdValue::sub_from_scalar(1.0, AdValue::mul(scratch.ad_value(238), scratch.ad_value(2293)))));
        }

        if ((((scratch.values[2404] != 0.0) && (!(scratch.values[2417] != 0.0))) && (scratch.values[2421] != 0.0)) && (!(scratch.values[2424] != 0.0))) {
            scratch.store_ad(2322, &AdValue::offset(AdValue::mul(scratch.ad_value(238), scratch.ad_value(2293)), 1.0));
        }

        if (((scratch.values[2404] != 0.0) && (!(scratch.values[2417] != 0.0))) && (scratch.values[2421] != 0.0)) {
            scratch.store_ad(2078, &AdValue::mul(scratch.ad_value(2315), scratch.ad_value(2322)));
        }

        if (((scratch.values[2404] != 0.0) && (!(scratch.values[2417] != 0.0))) && (scratch.values[2421] != 0.0)) {
            scratch.store_ad(2323, &AdValue::scale(AdValue::div(scratch.ad_value(2078), AdValue::offset(scratch.ad_value(2078), 100.0)), 100.0));
        }

        scratch.values[2425] = if (scratch.values[239] < 0.0) { 1.0 } else { 0.0 };

        if ((((scratch.values[2404] != 0.0) && (!(scratch.values[2417] != 0.0))) && (scratch.values[2421] != 0.0)) && (scratch.values[2425] != 0.0)) {
            scratch.store_ad(2076, &AdValue::div_from_scalar(1.0, AdValue::sub_from_scalar(1.0, AdValue::mul(scratch.ad_value(239), scratch.ad_value(2323)))));
        }

        if ((((scratch.values[2404] != 0.0) && (!(scratch.values[2417] != 0.0))) && (scratch.values[2421] != 0.0)) && (!(scratch.values[2425] != 0.0))) {
            scratch.store_ad(2076, &AdValue::offset(AdValue::mul(scratch.ad_value(239), scratch.ad_value(2323)), 1.0));
        }

        if (((scratch.values[2404] != 0.0) && (!(scratch.values[2417] != 0.0))) && (scratch.values[2421] != 0.0)) {
            scratch.store_ad(2324, &AdValue::mul(scratch.ad_value(764), AdValue::div(scratch.ad_value(2076), scratch.ad_value(2321))));
        }

        if (((scratch.values[2404] != 0.0) && (!(scratch.values[2417] != 0.0))) && (scratch.values[2421] != 0.0)) {
            scratch.store_ad(2325, &AdValue::add(AdValue::div(scratch.ad_value(2315), scratch.ad_value(2355)), scratch.ad_value(2011)));
        }

        if (((scratch.values[2404] != 0.0) && (!(scratch.values[2417] != 0.0))) && (scratch.values[2421] != 0.0)) {
            scratch.store_ad(2326, &AdValue::scale(AdValue::mul(scratch.ad_value(2324), scratch.ad_value(2325)), 0.7071067811865475));
        }

        scratch.values[2426] = if (scratch.values[0] == (-1.0)) { 1.0 } else { 0.0 };

        if ((((scratch.values[2404] != 0.0) && (!(scratch.values[2417] != 0.0))) && (scratch.values[2421] != 0.0)) && (scratch.values[2426] != 0.0)) {
            scratch.store_ad(2326, &AdValue::div(scratch.ad_value(2326), AdValue::sqrt(AdValue::offset(scratch.ad_value(2326), 1.0))));
        }

        if (((scratch.values[2404] != 0.0) && (!(scratch.values[2417] != 0.0))) && (scratch.values[2421] != 0.0)) {
            scratch.store_ad(2327, &AdValue::div_from_scalar(2.0, AdValue::offset(AdValue::sqrt(AdValue::offset(AdValue::scale(scratch.ad_value(2326), 4.0), 1.0)), 1.0)));
        }

        if (((scratch.values[2404] != 0.0) && (!(scratch.values[2417] != 0.0))) && (scratch.values[2421] != 0.0)) {
            scratch.store_ad(2077, &AdValue::mul(scratch.ad_value(2327), scratch.ad_value(2326)));
        }

        if (((scratch.values[2404] != 0.0) && (!(scratch.values[2417] != 0.0))) && (scratch.values[2421] != 0.0)) {
            scratch.store_ad(2328, &AdValue::mul(AdValue::mul(scratch.ad_value(2325), scratch.ad_value(2327)), AdValue::offset(AdValue::scale(AdValue::div(AdValue::mul(scratch.ad_value(2077), AdValue::sub_from_scalar(1.0, AdValue::mul(scratch.ad_value(2077), scratch.ad_value(2327)))), AdValue::offset(AdValue::scale(AdValue::mul(AdValue::square(scratch.ad_value(2077)), scratch.ad_value(2327)), 4.0), 1.0)), 0.86), 1.0)));
        }

        if (((scratch.values[2404] != 0.0) && (!(scratch.values[2417] != 0.0))) && (scratch.values[2421] != 0.0)) {
            scratch.store_ad(2329, &AdValue::add(scratch.ad_value(2314), AdValue::scale(scratch.ad_value(2297), 0.5)));
        }

        if (((scratch.values[2404] != 0.0) && (!(scratch.values[2417] != 0.0))) && (scratch.values[2421] != 0.0)) {
            scratch.store_ad(2330, &AdValue::scale(AdValue::div(AdValue::mul(AdValue::mul(scratch.ad_value(2297), scratch.ad_value(2311)), scratch.ad_value(2011)), AdValue::add(scratch.ad_value(2329), AdValue::sqrt(AdValue::sub(AdValue::square(scratch.ad_value(2329)), AdValue::scale(AdValue::mul(scratch.ad_value(2297), scratch.ad_value(2311)), 0.98))))), 0.98));
        }

        if (((scratch.values[2404] != 0.0) && (!(scratch.values[2417] != 0.0))) && (scratch.values[2421] != 0.0)) {
            scratch.store_ad(2331, &AdValue::add(scratch.ad_value(2328), scratch.ad_value(2330)));
        }

        if (((scratch.values[2404] != 0.0) && (!(scratch.values[2417] != 0.0))) && (scratch.values[2421] != 0.0)) {
            scratch.store_ad(2332, &AdValue::scale(AdValue::mul(scratch.ad_value(2328), scratch.ad_value(2330)), 2.0));
        }

        if (((scratch.values[2404] != 0.0) && (!(scratch.values[2417] != 0.0))) && (scratch.values[2421] != 0.0)) {
            scratch.store_ad(2333, &AdValue::div(scratch.ad_value(2332), AdValue::add(scratch.ad_value(2331), AdValue::sqrt(AdValue::sub(AdValue::square(scratch.ad_value(2331)), AdValue::scale(scratch.ad_value(2332), 1.98))))));
        }

        if (((scratch.values[2404] != 0.0) && (!(scratch.values[2417] != 0.0))) && (scratch.values[2421] != 0.0)) {
            scratch.store_ad(870, &AdValue::sub(scratch.ad_value(2333), AdValue::mul(scratch.ad_value(2011), AdValue::ln(AdValue::offset(AdValue::div(AdValue::mul(AdValue::mul(scratch.ad_value(2333), AdValue::sub(scratch.ad_value(2333), AdValue::mul(AdValue::scale(scratch.ad_value(2329), 2.0), scratch.ad_value(2011)))), scratch.ad_value(2298)), AdValue::mul(AdValue::square(scratch.ad_value(2011)), scratch.ad_value(2311))), 1.0)))));
        }

        if (((scratch.values[2404] != 0.0) && (!(scratch.values[2417] != 0.0))) && (!(scratch.values[2421] != 0.0))) {
            scratch.values[870] = scratch.values[869];
            scratch.node_derivatives[870] = scratch.node_derivatives[869];
            scratch.branch_derivatives[870] = scratch.branch_derivatives[869];
        }

        if ((scratch.values[2404] != 0.0) && (!(scratch.values[2417] != 0.0))) {
            scratch.store_ad(2076, &AdValue::pow(AdValue::div(scratch.ad_value(854), scratch.ad_value(870)), scratch.ad_value(240)));
        }

        if ((scratch.values[2404] != 0.0) && (!(scratch.values[2417] != 0.0))) {
            scratch.store_ad(2334, &AdValue::mul(scratch.ad_value(854), AdValue::pow(AdValue::offset(scratch.ad_value(2076), 1.0), AdValue::neg(scratch.ad_value(820)))));
        }

        if ((scratch.values[2404] != 0.0) && (!(scratch.values[2417] != 0.0))) {
            scratch.store_ad(2335, &AdValue::mul(scratch.ad_value(2334), scratch.ad_value(2012)));
        }

        if ((scratch.values[2404] != 0.0) && (!(scratch.values[2417] != 0.0))) {
            scratch.store_ad(2336, &AdValue::add(scratch.ad_value(2302), scratch.ad_value(2335)));
        }

        scratch.values[2427] = if (scratch.values[2335] < 460.51701859880916) { 1.0 } else { 0.0 };

        if (((scratch.values[2404] != 0.0) && (!(scratch.values[2417] != 0.0))) && (scratch.values[2427] != 0.0)) {
            scratch.store_ad(2337, &AdValue::exp(AdValue::neg(scratch.ad_value(2335))));
        }

        if (((scratch.values[2404] != 0.0) && (!(scratch.values[2417] != 0.0))) && (!(scratch.values[2427] != 0.0))) {
            scratch.store_ad(2337, &AdValue::div_from_scalar(1e-200, AdValue::offset(AdValue::mul(AdValue::offset(scratch.ad_value(2335), (-460.51701859880916)), AdValue::offset(AdValue::scale(AdValue::mul(AdValue::offset(scratch.ad_value(2335), (-460.51701859880916)), AdValue::offset(AdValue::scale(AdValue::offset(scratch.ad_value(2335), (-460.51701859880916)), 0.3333333333333333), 1.0)), 0.5), 1.0)), 1.0)));
        }

        if ((scratch.values[2404] != 0.0) && (!(scratch.values[2417] != 0.0))) {
            scratch.store_ad(2338, &AdValue::mul(scratch.ad_value(2303), scratch.ad_value(2337)));
        }

        scratch.values[2428] = if (((scratch.values[2294]) as f64).abs() <= scratch.values[2304]) { 1.0 } else { 0.0 };

        if (((scratch.values[2404] != 0.0) && (!(scratch.values[2417] != 0.0))) && (scratch.values[2428] != 0.0)) {
            scratch.store_ad(2380, &AdValue::scale(AdValue::square(scratch.ad_value(2300)), (0.16666666666666666 * 0.7071067811865475)));
        }

        if (((scratch.values[2404] != 0.0) && (!(scratch.values[2417] != 0.0))) && (scratch.values[2428] != 0.0)) {
            scratch.store_ad(2339, &AdValue::mul(AdValue::mul(scratch.ad_value(2294), scratch.ad_value(2300)), AdValue::offset(AdValue::mul(AdValue::mul(AdValue::mul(scratch.ad_value(2294), AdValue::sub_from_scalar(1.0, scratch.ad_value(2338))), scratch.ad_value(2296)), scratch.ad_value(2380)), 1.0)));
        }

        if (((scratch.values[2404] != 0.0) && (!(scratch.values[2417] != 0.0))) && (!(scratch.values[2428] != 0.0))) {
            scratch.store_ad(2401, &AdValue::offset(scratch.ad_value(2336), 3.0));
        }

        if (((scratch.values[2404] != 0.0) && (!(scratch.values[2417] != 0.0))) && (!(scratch.values[2428] != 0.0))) {
            scratch.store_ad(2384, &AdValue::sub(AdValue::scale(AdValue::sub(AdValue::add(scratch.ad_value(2400), scratch.ad_value(2401)), AdValue::sqrt(AdValue::offset(AdValue::mul(AdValue::sub(scratch.ad_value(2400), scratch.ad_value(2401)), AdValue::sub(scratch.ad_value(2400), scratch.ad_value(2401))), 5.0))), 0.5), AdValue::scale(AdValue::sub(scratch.ad_value(2401), AdValue::sqrt(AdValue::offset(AdValue::square(scratch.ad_value(2401)), 5.0))), 0.5)));
        }

        if (((scratch.values[2404] != 0.0) && (!(scratch.values[2417] != 0.0))) && (!(scratch.values[2428] != 0.0))) {
            scratch.store_ad(2379, &AdValue::sub(scratch.ad_value(2294), scratch.ad_value(2384)));
        }

        if (((scratch.values[2404] != 0.0) && (!(scratch.values[2417] != 0.0))) && (!(scratch.values[2428] != 0.0))) {
            scratch.store_ad(2380, &AdValue::exp(AdValue::neg(scratch.ad_value(2384))));
        }

        if (((scratch.values[2404] != 0.0) && (!(scratch.values[2417] != 0.0))) && (!(scratch.values[2428] != 0.0))) {
            scratch.store_ad(2381, &AdValue::div_from_scalar(1.0, AdValue::offset(AdValue::square(scratch.ad_value(2384)), 2.0)));
        }

        if (((scratch.values[2404] != 0.0) && (!(scratch.values[2417] != 0.0))) && (!(scratch.values[2428] != 0.0))) {
            scratch.store_ad(2391, &AdValue::mul(AdValue::square(scratch.ad_value(2384)), scratch.ad_value(2381)));
        }

        if (((scratch.values[2404] != 0.0) && (!(scratch.values[2417] != 0.0))) && (!(scratch.values[2428] != 0.0))) {
            scratch.store_ad(2392, &AdValue::scale(AdValue::mul(AdValue::mul(scratch.ad_value(2384), scratch.ad_value(2381)), scratch.ad_value(2381)), 4.0));
        }

        if (((scratch.values[2404] != 0.0) && (!(scratch.values[2417] != 0.0))) && (!(scratch.values[2428] != 0.0))) {
            scratch.store_ad(2393, &AdValue::mul(AdValue::mul(AdValue::sub(AdValue::scale(scratch.ad_value(2381), 8.0), AdValue::scale(scratch.ad_value(2391), 12.0)), scratch.ad_value(2381)), scratch.ad_value(2381)));
        }

        if (((scratch.values[2404] != 0.0) && (!(scratch.values[2417] != 0.0))) && (!(scratch.values[2428] != 0.0))) {
            scratch.store_ad(2385, &AdValue::max_from_scalar(1e-40, AdValue::sub(AdValue::square(scratch.ad_value(2379)), AdValue::mul(scratch.ad_value(2297), AdValue::sub(AdValue::offset(AdValue::add(scratch.ad_value(2380), scratch.ad_value(2384)), (-1.0)), AdValue::mul(scratch.ad_value(2338), AdValue::add(AdValue::offset(scratch.ad_value(2384), 1.0), scratch.ad_value(2391))))))));
        }

        if (((scratch.values[2404] != 0.0) && (!(scratch.values[2417] != 0.0))) && (!(scratch.values[2428] != 0.0))) {
            scratch.store_ad(2402, &AdValue::sub_from_scalar(1.0, AdValue::scale(AdValue::mul(scratch.ad_value(2297), AdValue::sub(scratch.ad_value(2380), AdValue::mul(scratch.ad_value(2338), scratch.ad_value(2393)))), 0.5)));
        }

        if (((scratch.values[2404] != 0.0) && (!(scratch.values[2417] != 0.0))) && (!(scratch.values[2428] != 0.0))) {
            scratch.store_ad(2386, &AdValue::add(AdValue::scale(scratch.ad_value(2379), 2.0), AdValue::mul(scratch.ad_value(2297), AdValue::sub(AdValue::sub_from_scalar(1.0, scratch.ad_value(2380)), AdValue::mul(scratch.ad_value(2338), AdValue::offset(scratch.ad_value(2392), 1.0))))));
        }

        if (((scratch.values[2404] != 0.0) && (!(scratch.values[2417] != 0.0))) && (!(scratch.values[2428] != 0.0))) {
            scratch.store_ad(2387, &AdValue::add(AdValue::sub(scratch.ad_value(2336), scratch.ad_value(2384)), AdValue::ln(AdValue::div(scratch.ad_value(2385), scratch.ad_value(2297)))));
        }

        if (((scratch.values[2404] != 0.0) && (!(scratch.values[2417] != 0.0))) && (!(scratch.values[2428] != 0.0))) {
            scratch.store_ad(966, &AdValue::add(scratch.ad_value(2385), scratch.ad_value(2386)));
        }

        if (((scratch.values[2404] != 0.0) && (!(scratch.values[2417] != 0.0))) && (!(scratch.values[2428] != 0.0))) {
            scratch.store_ad(965, &AdValue::add(AdValue::square(scratch.ad_value(966)), AdValue::mul(scratch.ad_value(2387), AdValue::sub(AdValue::scale(AdValue::square(scratch.ad_value(2386)), 0.5), AdValue::mul(scratch.ad_value(2385), scratch.ad_value(2402))))));
        }

        if (((scratch.values[2404] != 0.0) && (!(scratch.values[2417] != 0.0))) && (!(scratch.values[2428] != 0.0))) {
            let assign49280_ad_e63649: AdValue = AdValue::add(scratch.ad_value(2384), AdValue::div(AdValue::mul(AdValue::mul(scratch.ad_value(2385), scratch.ad_value(966)), scratch.ad_value(2387)), AdValue::add(scratch.ad_value(965), AdValue::mul(AdValue::mul(AdValue::mul(AdValue::mul(AdValue::div(scratch.ad_value(966), scratch.ad_value(965)), scratch.ad_value(2387)), scratch.ad_value(2387)), scratch.ad_value(2386)), AdValue::sub(AdValue::scale(AdValue::square(scratch.ad_value(2386)), 0.3333333333333333), AdValue::mul(scratch.ad_value(2385), scratch.ad_value(2402)))))));
            scratch.store_ad(2403, &assign49280_ad_e63649);
        }

        scratch.values[2429] = if (scratch.values[2403] < 230.25850929940458) { 1.0 } else { 0.0 };

        if ((((scratch.values[2404] != 0.0) && (!(scratch.values[2417] != 0.0))) && (!(scratch.values[2428] != 0.0))) && (scratch.values[2429] != 0.0)) {
            scratch.store_ad(2389, &AdValue::exp(scratch.ad_value(2403)));
        }

        if ((((scratch.values[2404] != 0.0) && (!(scratch.values[2417] != 0.0))) && (!(scratch.values[2428] != 0.0))) && (scratch.values[2429] != 0.0)) {
            scratch.store_ad(2390, &AdValue::div_from_scalar(1.0, scratch.ad_value(2389)));
        }

        if ((((scratch.values[2404] != 0.0) && (!(scratch.values[2417] != 0.0))) && (!(scratch.values[2428] != 0.0))) && (scratch.values[2429] != 0.0)) {
            scratch.store_ad(2389, &AdValue::mul(scratch.ad_value(2338), scratch.ad_value(2389)));
        }

        scratch.values[2430] = if (scratch.values[2403] > (scratch.values[2336] - 230.25850929940458)) { 1.0 } else { 0.0 };

        if (((((scratch.values[2404] != 0.0) && (!(scratch.values[2417] != 0.0))) && (!(scratch.values[2428] != 0.0))) && (!(scratch.values[2429] != 0.0))) && (scratch.values[2430] != 0.0)) {
            scratch.store_ad(2389, &AdValue::exp(AdValue::sub(scratch.ad_value(2403), scratch.ad_value(2336))));
        }

        if (((((scratch.values[2404] != 0.0) && (!(scratch.values[2417] != 0.0))) && (!(scratch.values[2428] != 0.0))) && (!(scratch.values[2429] != 0.0))) && (scratch.values[2430] != 0.0)) {
            scratch.store_ad(2390, &AdValue::div(scratch.ad_value(2338), scratch.ad_value(2389)));
        }

        if (((((scratch.values[2404] != 0.0) && (!(scratch.values[2417] != 0.0))) && (!(scratch.values[2428] != 0.0))) && (!(scratch.values[2429] != 0.0))) && (!(scratch.values[2430] != 0.0))) {
            scratch.store_ad(2389, &AdValue::div_from_scalar(1e-100, AdValue::offset(AdValue::mul(AdValue::offset(AdValue::sub(scratch.ad_value(2336), scratch.ad_value(2403)), (-230.25850929940458)), AdValue::offset(AdValue::scale(AdValue::mul(AdValue::offset(AdValue::sub(scratch.ad_value(2336), scratch.ad_value(2403)), (-230.25850929940458)), AdValue::offset(AdValue::scale(AdValue::offset(AdValue::sub(scratch.ad_value(2336), scratch.ad_value(2403)), (-230.25850929940458)), 0.3333333333333333), 1.0)), 0.5), 1.0)), 1.0)));
        }

        if (((((scratch.values[2404] != 0.0) && (!(scratch.values[2417] != 0.0))) && (!(scratch.values[2428] != 0.0))) && (!(scratch.values[2429] != 0.0))) && (!(scratch.values[2430] != 0.0))) {
            scratch.store_ad(2390, &AdValue::div_from_scalar(1e-100, AdValue::offset(AdValue::mul(AdValue::offset(scratch.ad_value(2403), (-230.25850929940458)), AdValue::offset(AdValue::scale(AdValue::mul(AdValue::offset(scratch.ad_value(2403), (-230.25850929940458)), AdValue::offset(AdValue::scale(AdValue::offset(scratch.ad_value(2403), (-230.25850929940458)), 0.3333333333333333), 1.0)), 0.5), 1.0)), 1.0)));
        }

        if (((scratch.values[2404] != 0.0) && (!(scratch.values[2417] != 0.0))) && (!(scratch.values[2428] != 0.0))) {
            scratch.store_ad(2379, &AdValue::div_from_scalar(1.0, AdValue::offset(AdValue::square(scratch.ad_value(2403)), 2.0)));
        }

        if (((scratch.values[2404] != 0.0) && (!(scratch.values[2417] != 0.0))) && (!(scratch.values[2428] != 0.0))) {
            scratch.store_ad(2391, &AdValue::mul(AdValue::square(scratch.ad_value(2403)), scratch.ad_value(2379)));
        }

        if (((scratch.values[2404] != 0.0) && (!(scratch.values[2417] != 0.0))) && (!(scratch.values[2428] != 0.0))) {
            scratch.store_ad(2392, &AdValue::scale(AdValue::mul(AdValue::mul(scratch.ad_value(2403), scratch.ad_value(2379)), scratch.ad_value(2379)), 4.0));
        }

        if (((scratch.values[2404] != 0.0) && (!(scratch.values[2417] != 0.0))) && (!(scratch.values[2428] != 0.0))) {
            scratch.store_ad(2393, &AdValue::mul(AdValue::mul(AdValue::sub(AdValue::scale(scratch.ad_value(2379), 8.0), AdValue::scale(scratch.ad_value(2391), 12.0)), scratch.ad_value(2379)), scratch.ad_value(2379)));
        }

        if (((scratch.values[2404] != 0.0) && (!(scratch.values[2417] != 0.0))) && (!(scratch.values[2428] != 0.0))) {
            scratch.store_ad(2379, &AdValue::sub(scratch.ad_value(2294), scratch.ad_value(2403)));
        }

        if (((scratch.values[2404] != 0.0) && (!(scratch.values[2417] != 0.0))) && (!(scratch.values[2428] != 0.0))) {
            scratch.store_ad(2394, &AdValue::add(AdValue::scale(scratch.ad_value(2379), 2.0), AdValue::mul(scratch.ad_value(2297), AdValue::sub(AdValue::add(AdValue::sub_from_scalar(1.0, scratch.ad_value(2390)), scratch.ad_value(2389)), AdValue::mul(scratch.ad_value(2338), AdValue::offset(scratch.ad_value(2392), 1.0))))));
        }

        if (((scratch.values[2404] != 0.0) && (!(scratch.values[2417] != 0.0))) && (!(scratch.values[2428] != 0.0))) {
            scratch.store_ad(2395, &AdValue::sub(AdValue::square(scratch.ad_value(2379)), AdValue::mul(scratch.ad_value(2297), AdValue::sub(AdValue::add(AdValue::offset(AdValue::add(scratch.ad_value(2390), scratch.ad_value(2403)), (-1.0)), scratch.ad_value(2389)), AdValue::mul(scratch.ad_value(2338), AdValue::add(AdValue::offset(scratch.ad_value(2403), 1.0), scratch.ad_value(2391)))))));
        }

        if (((scratch.values[2404] != 0.0) && (!(scratch.values[2417] != 0.0))) && (!(scratch.values[2428] != 0.0))) {
            scratch.store_ad(2379, &AdValue::sub_from_scalar(2.0, AdValue::mul(scratch.ad_value(2297), AdValue::sub(AdValue::add(scratch.ad_value(2390), scratch.ad_value(2389)), AdValue::mul(scratch.ad_value(2338), scratch.ad_value(2393))))));
        }

        if (((scratch.values[2404] != 0.0) && (!(scratch.values[2417] != 0.0))) && (!(scratch.values[2428] != 0.0))) {
            scratch.store_ad(2379, &AdValue::sub(AdValue::square(scratch.ad_value(2394)), AdValue::scale(AdValue::mul(scratch.ad_value(2395), scratch.ad_value(2379)), 2.0)));
        }

        if (((scratch.values[2404] != 0.0) && (!(scratch.values[2417] != 0.0))) && (!(scratch.values[2428] != 0.0))) {
            scratch.store_ad(2339, &AdValue::add(scratch.ad_value(2403), AdValue::scale(AdValue::div(scratch.ad_value(2395), AdValue::add(scratch.ad_value(2394), AdValue::sqrt(scratch.ad_value(2379)))), 2.0)));
        }

        if ((scratch.values[2404] != 0.0) && (!(scratch.values[2417] != 0.0))) {
            scratch.store_ad(2340, &AdValue::sub(scratch.ad_value(2339), scratch.ad_value(2305)));
        }

        scratch.values[2431] = if (scratch.values[2340] < 1e-10) { 1.0 } else { 0.0 };

        if (((scratch.values[2404] != 0.0) && (!(scratch.values[2417] != 0.0))) && (scratch.values[2431] != 0.0)) {
            scratch.store_ad(2341, &AdValue::add(AdValue::scale(AdValue::sub(scratch.ad_value(2294), scratch.ad_value(2305)), 2.0), AdValue::mul(scratch.ad_value(2297), AdValue::sub(AdValue::add(AdValue::sub_from_scalar(1.0, scratch.ad_value(2310)), AdValue::mul(scratch.ad_value(2306), scratch.ad_value(2337))), AdValue::mul(scratch.ad_value(2338), AdValue::offset(scratch.ad_value(2308), 1.0))))));
        }

        if (((scratch.values[2404] != 0.0) && (!(scratch.values[2417] != 0.0))) && (scratch.values[2431] != 0.0)) {
            scratch.store_ad(2342, &AdValue::mul(AdValue::mul(scratch.ad_value(2297), AdValue::sub_from_scalar(1.0, scratch.ad_value(2337))), scratch.ad_value(2311)));
        }

        if (((scratch.values[2404] != 0.0) && (!(scratch.values[2417] != 0.0))) && (scratch.values[2431] != 0.0)) {
            scratch.store_ad(2076, &AdValue::sub_from_scalar(2.0, AdValue::mul(scratch.ad_value(2297), AdValue::sub(AdValue::add(scratch.ad_value(2310), AdValue::mul(scratch.ad_value(2306), scratch.ad_value(2337))), AdValue::mul(scratch.ad_value(2338), scratch.ad_value(2309))))));
        }

        if (((scratch.values[2404] != 0.0) && (!(scratch.values[2417] != 0.0))) && (scratch.values[2431] != 0.0)) {
            scratch.store_ad(2076, &AdValue::sub(AdValue::square(scratch.ad_value(2341)), AdValue::scale(AdValue::mul(scratch.ad_value(2076), scratch.ad_value(2342)), 2.0)));
        }

        if (((scratch.values[2404] != 0.0) && (!(scratch.values[2417] != 0.0))) && (scratch.values[2431] != 0.0)) {
            scratch.store_ad(2340, &AdValue::scale(AdValue::div(scratch.ad_value(2342), AdValue::add(scratch.ad_value(2341), AdValue::sqrt(scratch.ad_value(2076)))), 2.0));
        }

        if (((scratch.values[2404] != 0.0) && (!(scratch.values[2417] != 0.0))) && (scratch.values[2431] != 0.0)) {
            scratch.store_ad(2339, &AdValue::add(scratch.ad_value(2305), scratch.ad_value(2340)));
        }

        if ((scratch.values[2404] != 0.0) && (!(scratch.values[2417] != 0.0))) {
            scratch.store_ad(2343, &AdValue::mul(scratch.ad_value(2340), scratch.ad_value(2011)));
        }

        if ((scratch.values[2404] != 0.0) && (!(scratch.values[2417] != 0.0))) {
            scratch.store_ad(2344, &AdValue::div(AdValue::square(scratch.ad_value(2339)), AdValue::offset(AdValue::square(scratch.ad_value(2339)), 2.0)));
        }

        scratch.values[2432] = if (scratch.values[2339] < 230.25850929940458) { 1.0 } else { 0.0 };

        if (((scratch.values[2404] != 0.0) && (!(scratch.values[2417] != 0.0))) && (scratch.values[2432] != 0.0)) {
            scratch.store_ad(2345, &AdValue::exp(AdValue::neg(scratch.ad_value(2339))));
        }

        scratch.values[2433] = if (scratch.values[2339] < 1e-5) { 1.0 } else { 0.0 };

        if ((((scratch.values[2404] != 0.0) && (!(scratch.values[2417] != 0.0))) && (scratch.values[2432] != 0.0)) && (scratch.values[2433] != 0.0)) {
            scratch.store_ad(2346, &AdValue::mul(AdValue::mul(AdValue::mul(AdValue::mul(AdValue::scale(scratch.ad_value(2338), 0.16666666666666666), scratch.ad_value(2339)), scratch.ad_value(2339)), scratch.ad_value(2339)), AdValue::offset(AdValue::scale(scratch.ad_value(2339), 1.75), 1.0)));
        }

    }

    pub(super) fn stamp_transient_block_37(
        &mut self,
        ctx: &GeneratedEvalContext<'_>,
        stamper: &mut GeneratedStamper<'_>,
        scratch: &mut Scratch,
    ) {
        let _ = stamper;
        if ((((scratch.values[2404] != 0.0) && (!(scratch.values[2417] != 0.0))) && (scratch.values[2432] != 0.0)) && (!(scratch.values[2433] != 0.0))) {
            scratch.store_ad(2346, &AdValue::mul(scratch.ad_value(2338), AdValue::sub(AdValue::offset(AdValue::sub(AdValue::div_from_scalar(1.0, scratch.ad_value(2345)), scratch.ad_value(2339)), (-1.0)), scratch.ad_value(2344))));
        }

        scratch.values[2434] = if (scratch.values[2339] > (scratch.values[2336] - 230.25850929940458)) { 1.0 } else { 0.0 };

        if ((((scratch.values[2404] != 0.0) && (!(scratch.values[2417] != 0.0))) && (!(scratch.values[2432] != 0.0))) && (scratch.values[2434] != 0.0)) {
            scratch.store_ad(2076, &AdValue::exp(AdValue::sub(scratch.ad_value(2339), scratch.ad_value(2336))));
        }

        if ((((scratch.values[2404] != 0.0) && (!(scratch.values[2417] != 0.0))) && (!(scratch.values[2432] != 0.0))) && (scratch.values[2434] != 0.0)) {
            scratch.store_ad(2345, &AdValue::div(scratch.ad_value(2338), scratch.ad_value(2076)));
        }

        if ((((scratch.values[2404] != 0.0) && (!(scratch.values[2417] != 0.0))) && (!(scratch.values[2432] != 0.0))) && (scratch.values[2434] != 0.0)) {
            scratch.store_ad(2346, &AdValue::sub(scratch.ad_value(2076), AdValue::mul(scratch.ad_value(2338), AdValue::add(AdValue::offset(scratch.ad_value(2339), 1.0), scratch.ad_value(2344)))));
        }

        if ((((scratch.values[2404] != 0.0) && (!(scratch.values[2417] != 0.0))) && (!(scratch.values[2432] != 0.0))) && (!(scratch.values[2434] != 0.0))) {
            scratch.store_ad(2345, &AdValue::div_from_scalar(1e-100, AdValue::offset(AdValue::mul(AdValue::offset(scratch.ad_value(2339), (-230.25850929940458)), AdValue::offset(AdValue::scale(AdValue::mul(AdValue::offset(scratch.ad_value(2339), (-230.25850929940458)), AdValue::offset(AdValue::scale(AdValue::offset(scratch.ad_value(2339), (-230.25850929940458)), 0.3333333333333333), 1.0)), 0.5), 1.0)), 1.0)));
        }

        if ((((scratch.values[2404] != 0.0) && (!(scratch.values[2417] != 0.0))) && (!(scratch.values[2432] != 0.0))) && (!(scratch.values[2434] != 0.0))) {
            scratch.store_ad(2076, &AdValue::div_from_scalar(1e-100, AdValue::offset(AdValue::mul(AdValue::offset(AdValue::sub(scratch.ad_value(2336), scratch.ad_value(2339)), (-230.25850929940458)), AdValue::offset(AdValue::scale(AdValue::mul(AdValue::offset(AdValue::sub(scratch.ad_value(2336), scratch.ad_value(2339)), (-230.25850929940458)), AdValue::offset(AdValue::scale(AdValue::offset(AdValue::sub(scratch.ad_value(2336), scratch.ad_value(2339)), (-230.25850929940458)), 0.3333333333333333), 1.0)), 0.5), 1.0)), 1.0)));
        }

        if ((((scratch.values[2404] != 0.0) && (!(scratch.values[2417] != 0.0))) && (!(scratch.values[2432] != 0.0))) && (!(scratch.values[2434] != 0.0))) {
            scratch.store_ad(2346, &AdValue::sub(scratch.ad_value(2076), AdValue::mul(scratch.ad_value(2338), AdValue::add(AdValue::offset(scratch.ad_value(2339), 1.0), scratch.ad_value(2344)))));
        }

        if ((scratch.values[2404] != 0.0) && (!(scratch.values[2417] != 0.0))) {
            scratch.store_ad(2347, &AdValue::scale(AdValue::add(scratch.ad_value(2305), scratch.ad_value(2339)), 0.5));
        }

        if ((scratch.values[2404] != 0.0) && (!(scratch.values[2417] != 0.0))) {
            scratch.values[2348] = 0.0;
            scratch.node_derivatives[2348] = [0.0; Instance::NODE_COUNT];
            scratch.branch_derivatives[2348] = [0.0; Instance::BRANCH_COUNT];
        }

        if ((scratch.values[2404] != 0.0) && (!(scratch.values[2417] != 0.0))) {
            scratch.store_ad(2076, &AdValue::mul(scratch.ad_value(2345), scratch.ad_value(2310)));
        }

        scratch.values[2435] = if (scratch.values[2076] > 0.0) { 1.0 } else { 0.0 };

        if (((scratch.values[2404] != 0.0) && (!(scratch.values[2417] != 0.0))) && (scratch.values[2435] != 0.0)) {
            scratch.store_ad(2348, &AdValue::sqrt(scratch.ad_value(2076)));
        }

        if ((scratch.values[2404] != 0.0) && (!(scratch.values[2417] != 0.0))) {
            scratch.store_ad(2349, &AdValue::scale(AdValue::add(scratch.ad_value(2311), scratch.ad_value(2346)), 0.5));
        }

        if ((scratch.values[2404] != 0.0) && (!(scratch.values[2417] != 0.0))) {
            scratch.store_ad(2350, &AdValue::add(scratch.ad_value(2349), AdValue::scale(AdValue::mul(AdValue::square(scratch.ad_value(2340)), AdValue::sub(scratch.ad_value(2348), AdValue::scale(scratch.ad_value(2298), 2.0))), 0.125)));
        }

        scratch.values[2436] = if (scratch.values[2347] < 1e-5) { 1.0 } else { 0.0 };

        if (((scratch.values[2404] != 0.0) && (!(scratch.values[2417] != 0.0))) && (scratch.values[2436] != 0.0)) {
            scratch.store_ad(2351, &AdValue::scale(AdValue::mul(AdValue::square(scratch.ad_value(2347)), AdValue::sub_from_scalar(1.0, AdValue::scale(AdValue::mul(scratch.ad_value(2347), AdValue::sub_from_scalar(1.0, AdValue::scale(scratch.ad_value(2347), 0.25))), 0.3333333333333333))), 0.5));
        }

        if (((scratch.values[2404] != 0.0) && (!(scratch.values[2417] != 0.0))) && (scratch.values[2436] != 0.0)) {
            scratch.store_ad(2352, &AdValue::mul(scratch.ad_value(2296), AdValue::sqrt(AdValue::add(scratch.ad_value(2350), scratch.ad_value(2351)))));
        }

        scratch.values[2437] = if (scratch.values[773] > 0.0) { 1.0 } else { 0.0 };

        if ((((scratch.values[2404] != 0.0) && (!(scratch.values[2417] != 0.0))) && (scratch.values[2436] != 0.0)) && (scratch.values[2437] != 0.0)) {
            scratch.store_ad(2353, &AdValue::div_from_scalar(1.0, AdValue::sqrt(AdValue::offset(AdValue::mul(scratch.ad_value(773), scratch.ad_value(2352)), 1.0))));
        }

        if (((scratch.values[2404] != 0.0) && (!(scratch.values[2417] != 0.0))) && (scratch.values[2436] != 0.0)) {
            scratch.store_ad(2076, &AdValue::sqrt(AdValue::sub_from_scalar(1.0, AdValue::scale(AdValue::mul(scratch.ad_value(2347), AdValue::sub_from_scalar(1.0, AdValue::scale(scratch.ad_value(2347), 0.25))), 0.3333333333333333))));
        }

        if (((scratch.values[2404] != 0.0) && (!(scratch.values[2417] != 0.0))) && (scratch.values[2436] != 0.0)) {
            scratch.store_ad(2354, &AdValue::scale(AdValue::mul(scratch.ad_value(2347), scratch.ad_value(2076)), 0.7071067811865475));
        }

        if (((scratch.values[2404] != 0.0) && (!(scratch.values[2417] != 0.0))) && (scratch.values[2436] != 0.0)) {
            scratch.store_ad(2355, &AdValue::add(scratch.ad_value(2353), AdValue::scale(AdValue::div(AdValue::mul(scratch.ad_value(2296), AdValue::add(AdValue::sub_from_scalar(1.0, AdValue::scale(scratch.ad_value(2347), 0.5)), AdValue::scale(AdValue::square(scratch.ad_value(2347)), 0.16666666666666666))), scratch.ad_value(2076)), 0.7071067811865475)));
        }

        if (((scratch.values[2404] != 0.0) && (!(scratch.values[2417] != 0.0))) && (!(scratch.values[2436] != 0.0))) {
            scratch.store_ad(2351, &AdValue::add(AdValue::offset(scratch.ad_value(2347), (-1.0)), scratch.ad_value(2348)));
        }

        if (((scratch.values[2404] != 0.0) && (!(scratch.values[2417] != 0.0))) && (!(scratch.values[2436] != 0.0))) {
            scratch.store_ad(2352, &AdValue::mul(scratch.ad_value(2296), AdValue::sqrt(AdValue::add(scratch.ad_value(2350), scratch.ad_value(2351)))));
        }

        scratch.values[2438] = if (scratch.values[773] > 0.0) { 1.0 } else { 0.0 };

        if ((((scratch.values[2404] != 0.0) && (!(scratch.values[2417] != 0.0))) && (!(scratch.values[2436] != 0.0))) && (scratch.values[2438] != 0.0)) {
            scratch.store_ad(2356, &AdValue::add(AdValue::sub_from_scalar(1.0, scratch.ad_value(2348)), AdValue::scale(AdValue::mul(scratch.ad_value(2352), scratch.ad_value(2298)), 2.0)));
        }

        if ((((scratch.values[2404] != 0.0) && (!(scratch.values[2417] != 0.0))) && (!(scratch.values[2436] != 0.0))) && (scratch.values[2438] != 0.0)) {
            scratch.store_ad(2353, &AdValue::div_from_scalar(1.0, AdValue::sqrt(AdValue::offset(AdValue::mul(scratch.ad_value(773), scratch.ad_value(2352)), 1.0))));
        }

        if ((((scratch.values[2404] != 0.0) && (!(scratch.values[2417] != 0.0))) && (!(scratch.values[2436] != 0.0))) && (scratch.values[2438] != 0.0)) {
            scratch.store_ad(2076, &AdValue::div(scratch.ad_value(2353), AdValue::offset(scratch.ad_value(2353), 1.0)));
        }

        if ((((scratch.values[2404] != 0.0) && (!(scratch.values[2417] != 0.0))) && (!(scratch.values[2436] != 0.0))) && (scratch.values[2438] != 0.0)) {
            scratch.store_ad(2357, &AdValue::mul(scratch.ad_value(773), AdValue::mul(AdValue::mul(AdValue::square(scratch.ad_value(2076)), scratch.ad_value(2297)), scratch.ad_value(2350))));
        }

        if ((((scratch.values[2404] != 0.0) && (!(scratch.values[2417] != 0.0))) && (!(scratch.values[2436] != 0.0))) && (scratch.values[2438] != 0.0)) {
            scratch.store_ad(2358, &AdValue::add(AdValue::scale(AdValue::sub(scratch.ad_value(2352), scratch.ad_value(2357)), 2.0), AdValue::mul(scratch.ad_value(2297), AdValue::add(AdValue::sub_from_scalar(1.0, scratch.ad_value(2348)), scratch.ad_value(2350)))));
        }

        if ((((scratch.values[2404] != 0.0) && (!(scratch.values[2417] != 0.0))) && (!(scratch.values[2436] != 0.0))) && (scratch.values[2438] != 0.0)) {
            scratch.store_ad(2359, &AdValue::mul(scratch.ad_value(2357), AdValue::sub(scratch.ad_value(2357), AdValue::scale(scratch.ad_value(2352), 2.0))));
        }

        if ((((scratch.values[2404] != 0.0) && (!(scratch.values[2417] != 0.0))) && (!(scratch.values[2436] != 0.0))) && (scratch.values[2438] != 0.0)) {
            scratch.store_ad(2360, &AdValue::sub_from_scalar(1.0, AdValue::scale(AdValue::mul(scratch.ad_value(2297), AdValue::add(scratch.ad_value(2348), scratch.ad_value(2350))), 0.5)));
        }

        if ((((scratch.values[2404] != 0.0) && (!(scratch.values[2417] != 0.0))) && (!(scratch.values[2436] != 0.0))) && (scratch.values[2438] != 0.0)) {
            scratch.store_ad(2361, &AdValue::div(AdValue::mul(scratch.ad_value(2359), scratch.ad_value(2358)), AdValue::sub(AdValue::square(scratch.ad_value(2358)), AdValue::mul(scratch.ad_value(2360), scratch.ad_value(2359)))));
        }

        if ((((scratch.values[2404] != 0.0) && (!(scratch.values[2417] != 0.0))) && (!(scratch.values[2436] != 0.0))) && (scratch.values[2438] != 0.0)) {
            scratch.store_ad(2347, &AdValue::add(scratch.ad_value(2347), scratch.ad_value(2361)));
        }

        if ((((scratch.values[2404] != 0.0) && (!(scratch.values[2417] != 0.0))) && (!(scratch.values[2436] != 0.0))) && (scratch.values[2438] != 0.0)) {
            scratch.store_ad(2362, &AdValue::exp(scratch.ad_value(2361)));
        }

        if ((((scratch.values[2404] != 0.0) && (!(scratch.values[2417] != 0.0))) && (!(scratch.values[2436] != 0.0))) && (scratch.values[2438] != 0.0)) {
            scratch.store_ad(2348, &AdValue::div(scratch.ad_value(2348), scratch.ad_value(2362)));
        }

        if ((((scratch.values[2404] != 0.0) && (!(scratch.values[2417] != 0.0))) && (!(scratch.values[2436] != 0.0))) && (scratch.values[2438] != 0.0)) {
            scratch.store_ad(2350, &AdValue::mul(scratch.ad_value(2350), scratch.ad_value(2362)));
        }

        if ((((scratch.values[2404] != 0.0) && (!(scratch.values[2417] != 0.0))) && (!(scratch.values[2436] != 0.0))) && (scratch.values[2438] != 0.0)) {
            scratch.store_ad(2351, &AdValue::add(AdValue::offset(scratch.ad_value(2347), (-1.0)), scratch.ad_value(2348)));
        }

        if ((((scratch.values[2404] != 0.0) && (!(scratch.values[2417] != 0.0))) && (!(scratch.values[2436] != 0.0))) && (scratch.values[2438] != 0.0)) {
            scratch.store_ad(2352, &AdValue::mul(scratch.ad_value(2296), AdValue::sqrt(AdValue::add(scratch.ad_value(2350), scratch.ad_value(2351)))));
        }

        if ((((scratch.values[2404] != 0.0) && (!(scratch.values[2417] != 0.0))) && (!(scratch.values[2436] != 0.0))) && (scratch.values[2438] != 0.0)) {
            scratch.store_ad(2363, &AdValue::add(AdValue::sub_from_scalar(1.0, scratch.ad_value(2348)), AdValue::scale(AdValue::mul(AdValue::mul(scratch.ad_value(2352), scratch.ad_value(2353)), scratch.ad_value(2298)), 2.0)));
        }

        if ((((scratch.values[2404] != 0.0) && (!(scratch.values[2417] != 0.0))) && (!(scratch.values[2436] != 0.0))) && (scratch.values[2438] != 0.0)) {
            scratch.store_ad(2340, &AdValue::div(AdValue::mul(AdValue::mul(scratch.ad_value(2340), scratch.ad_value(2362)), AdValue::add(scratch.ad_value(2356), scratch.ad_value(2349))), AdValue::add(scratch.ad_value(2363), AdValue::mul(scratch.ad_value(2362), scratch.ad_value(2349)))));
        }

        if ((((scratch.values[2404] != 0.0) && (!(scratch.values[2417] != 0.0))) && (!(scratch.values[2436] != 0.0))) && (scratch.values[2438] != 0.0)) {
            scratch.store_ad(2343, &AdValue::mul(scratch.ad_value(2340), scratch.ad_value(2011)));
        }

        if (((scratch.values[2404] != 0.0) && (!(scratch.values[2417] != 0.0))) && (!(scratch.values[2436] != 0.0))) {
            scratch.store_ad(2354, &AdValue::sqrt(scratch.ad_value(2351)));
        }

        if (((scratch.values[2404] != 0.0) && (!(scratch.values[2417] != 0.0))) && (!(scratch.values[2436] != 0.0))) {
            scratch.store_ad(2355, &AdValue::add(scratch.ad_value(2353), AdValue::scale(AdValue::div(AdValue::mul(scratch.ad_value(2296), AdValue::sub_from_scalar(1.0, scratch.ad_value(2348))), scratch.ad_value(2354)), 0.5)));
        }

        if ((scratch.values[2404] != 0.0) && (!(scratch.values[2417] != 0.0))) {
            scratch.store_ad(2364, &AdValue::mul(scratch.ad_value(2011), AdValue::div(AdValue::mul(scratch.ad_value(2297), scratch.ad_value(2350)), AdValue::add(scratch.ad_value(2352), AdValue::mul(scratch.ad_value(2296), scratch.ad_value(2354))))));
        }

        if ((scratch.values[2404] != 0.0) && (!(scratch.values[2417] != 0.0))) {
            scratch.store_ad(2365, &AdValue::add(scratch.ad_value(2364), AdValue::mul(scratch.ad_value(2011), scratch.ad_value(2355))));
        }

        if ((scratch.values[2404] != 0.0) && (!(scratch.values[2417] != 0.0))) {
            scratch.store_ad(2366, &AdValue::mul(AdValue::mul(scratch.ad_value(2354), scratch.ad_value(2296)), scratch.ad_value(2011)));
        }

        scratch.values[2439] = if (scratch.values[235] < 0.0) { 1.0 } else { 0.0 };

        if (((scratch.values[2404] != 0.0) && (!(scratch.values[2417] != 0.0))) && (scratch.values[2439] != 0.0)) {
            scratch.store_ad(2076, &AdValue::sub_from_scalar(1.0, AdValue::mul(scratch.ad_value(235), scratch.ad_value(2364))));
        }

        if (((scratch.values[2404] != 0.0) && (!(scratch.values[2417] != 0.0))) && (!(scratch.values[2439] != 0.0))) {
            scratch.store_ad(2076, &AdValue::div_from_scalar(1.0, AdValue::offset(AdValue::mul(scratch.ad_value(235), scratch.ad_value(2364)), 1.0)));
        }

        if ((scratch.values[2404] != 0.0) && (!(scratch.values[2417] != 0.0))) {
            scratch.store_ad(2318, &AdValue::mul(scratch.ad_value(800), AdValue::mul(AdValue::mul(scratch.ad_value(2317), scratch.ad_value(2076)), scratch.ad_value(2364))));
        }

        if ((scratch.values[2404] != 0.0) && (!(scratch.values[2417] != 0.0))) {
            scratch.store_ad(2367, &AdValue::add(scratch.ad_value(2366), AdValue::mul(scratch.ad_value(818), scratch.ad_value(2364))));
        }

        if ((scratch.values[2404] != 0.0) && (!(scratch.values[2417] != 0.0))) {
            scratch.store_ad(2368, &AdValue::add(scratch.ad_value(2366), AdValue::mul(scratch.ad_value(819), scratch.ad_value(2364))));
        }

        if ((scratch.values[2404] != 0.0) && (!(scratch.values[2417] != 0.0))) {
            scratch.store_ad(2319, &AdValue::mul(scratch.ad_value(817), scratch.ad_value(2367)));
        }

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

    }

    pub(super) fn stamp_transient_block_38(
        &mut self,
        ctx: &GeneratedEvalContext<'_>,
        stamper: &mut GeneratedStamper<'_>,
        scratch: &mut Scratch,
    ) {
        let _ = stamper;
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

        scratch.values[2046] = 0.0;

        scratch.values[2047] = 0.0;

        scratch.values[2048] = 0.0;

        scratch.values[2444] = if (scratch.values[5] > 0.0) { 1.0 } else { 0.0 };

        scratch.values[2445] = if (scratch.values[410] == 1.0) { 1.0 } else { 0.0 };

        if ((scratch.values[2444] != 0.0) && (scratch.values[2445] != 0.0)) {
            scratch.store_ad(527, &AdValue::scale(scratch.ad_value(858), (scratch.values[420] * scratch.values[712])));
        }

        if ((scratch.values[2444] != 0.0) && (scratch.values[2445] != 0.0)) {
            let assign51050_ad_e65657: AdValue = {
                if (scratch.values[527] < (-230.25850929940458)) {
                    AdValue::div_from_scalar(1e-100, AdValue::offset(AdValue::sub_from_scalar((-230.25850929940458), scratch.ad_value(527)), 1.0))
                } else {
                    {
                        if (scratch.values[527] > scratch.values[704]) {
                            AdValue::mul(scratch.ad_value(705), AdValue::offset(AdValue::sub(scratch.ad_value(527), scratch.ad_value(704)), 1.0))
                        } else {
                            AdValue::exp(scratch.ad_value(527))
                        }
                    }
                }
            };
            scratch.store_ad(528, &assign51050_ad_e65657);
        }

        if ((scratch.values[2444] != 0.0) && (scratch.values[2445] != 0.0)) {
            scratch.store_ad(533, &AdValue::mul(scratch.ad_value(711), AdValue::offset(scratch.ad_value(528), (-1.0))));
        }

        if ((scratch.values[2444] != 0.0) && (scratch.values[2445] != 0.0)) {
            scratch.store_ad(527, &AdValue::mul(AdValue::scale(scratch.ad_value(858), scratch.values[420]), scratch.ad_value(714)));
        }

        if ((scratch.values[2444] != 0.0) && (scratch.values[2445] != 0.0)) {
            let assign51080_ad_e65708: AdValue = {
                if (scratch.values[527] < (-230.25850929940458)) {
                    AdValue::div_from_scalar(1e-100, AdValue::offset(AdValue::sub_from_scalar((-230.25850929940458), scratch.ad_value(527)), 1.0))
                } else {
                    {
                        if (scratch.values[527] > scratch.values[706]) {
                            AdValue::mul(scratch.ad_value(707), AdValue::offset(AdValue::sub(scratch.ad_value(527), scratch.ad_value(706)), 1.0))
                        } else {
                            AdValue::exp(scratch.ad_value(527))
                        }
                    }
                }
            };
            scratch.store_ad(528, &assign51080_ad_e65708);
        }

        if ((scratch.values[2444] != 0.0) && (scratch.values[2445] != 0.0)) {
            scratch.store_ad(534, &AdValue::mul(scratch.ad_value(713), AdValue::offset(scratch.ad_value(528), (-1.0))));
        }

        if ((scratch.values[2444] != 0.0) && (scratch.values[2445] != 0.0)) {
            scratch.values[535] = 0.0;
            scratch.node_derivatives[535] = [0.0; Instance::NODE_COUNT];
            scratch.branch_derivatives[535] = [0.0; Instance::BRANCH_COUNT];
        }

        scratch.values[2446] = if (scratch.values[710] > 0.0) { 1.0 } else { 0.0 };

        if (((scratch.values[2444] != 0.0) && (scratch.values[2445] != 0.0)) && (scratch.values[2446] != 0.0)) {
            scratch.store_ad(535, &AdValue::mul(scratch.ad_value(858), AdValue::add(scratch.ad_value(715), AdValue::mul(scratch.ad_value(858), scratch.ad_value(716)))));
        }

        if (((scratch.values[2444] != 0.0) && (scratch.values[2445] != 0.0)) && (!(scratch.values[2446] != 0.0))) {
            scratch.store_ad(527, &AdValue::mul(AdValue::scale(AdValue::neg(scratch.ad_value(858)), scratch.values[420]), scratch.ad_value(716)));
        }

        if (((scratch.values[2444] != 0.0) && (scratch.values[2445] != 0.0)) && (!(scratch.values[2446] != 0.0))) {
            let assign51140_ad_e65789: AdValue = {
                if (scratch.values[527] < (-230.25850929940458)) {
                    AdValue::div_from_scalar(1e-100, AdValue::offset(AdValue::sub_from_scalar((-230.25850929940458), scratch.ad_value(527)), 1.0))
                } else {
                    {
                        if (scratch.values[527] > scratch.values[708]) {
                            AdValue::mul(scratch.ad_value(709), AdValue::offset(AdValue::sub(scratch.ad_value(527), scratch.ad_value(708)), 1.0))
                        } else {
                            AdValue::exp(scratch.ad_value(527))
                        }
                    }
                }
            };
            scratch.store_ad(528, &assign51140_ad_e65789);
        }

        if (((scratch.values[2444] != 0.0) && (scratch.values[2445] != 0.0)) && (!(scratch.values[2446] != 0.0))) {
            scratch.store_ad(535, &AdValue::mul(AdValue::neg(scratch.ad_value(715)), AdValue::offset(scratch.ad_value(528), (-1.0))));
        }

        if ((scratch.values[2444] != 0.0) && (scratch.values[2445] != 0.0)) {
            scratch.store_ad(2033, &AdValue::add(AdValue::add(scratch.ad_value(533), scratch.ad_value(534)), scratch.ad_value(535)));
        }

        if ((scratch.values[2444] != 0.0) && (scratch.values[2445] != 0.0)) {
            scratch.store_ad(527, &AdValue::scale(scratch.ad_value(859), (scratch.values[420] * scratch.values[739])));
        }

        if ((scratch.values[2444] != 0.0) && (scratch.values[2445] != 0.0)) {
            let assign51180_ad_e65854: AdValue = {
                if (scratch.values[527] < (-230.25850929940458)) {
                    AdValue::div_from_scalar(1e-100, AdValue::offset(AdValue::sub_from_scalar((-230.25850929940458), scratch.ad_value(527)), 1.0))
                } else {
                    {
                        if (scratch.values[527] > scratch.values[731]) {
                            AdValue::mul(scratch.ad_value(732), AdValue::offset(AdValue::sub(scratch.ad_value(527), scratch.ad_value(731)), 1.0))
                        } else {
                            AdValue::exp(scratch.ad_value(527))
                        }
                    }
                }
            };
            scratch.store_ad(528, &assign51180_ad_e65854);
        }

        if ((scratch.values[2444] != 0.0) && (scratch.values[2445] != 0.0)) {
            scratch.store_ad(533, &AdValue::mul(scratch.ad_value(738), AdValue::offset(scratch.ad_value(528), (-1.0))));
        }

        if ((scratch.values[2444] != 0.0) && (scratch.values[2445] != 0.0)) {
            scratch.store_ad(527, &AdValue::mul(AdValue::scale(scratch.ad_value(859), scratch.values[420]), scratch.ad_value(741)));
        }

        if ((scratch.values[2444] != 0.0) && (scratch.values[2445] != 0.0)) {
            let assign51210_ad_e65905: AdValue = {
                if (scratch.values[527] < (-230.25850929940458)) {
                    AdValue::div_from_scalar(1e-100, AdValue::offset(AdValue::sub_from_scalar((-230.25850929940458), scratch.ad_value(527)), 1.0))
                } else {
                    {
                        if (scratch.values[527] > scratch.values[733]) {
                            AdValue::mul(scratch.ad_value(734), AdValue::offset(AdValue::sub(scratch.ad_value(527), scratch.ad_value(733)), 1.0))
                        } else {
                            AdValue::exp(scratch.ad_value(527))
                        }
                    }
                }
            };
            scratch.store_ad(528, &assign51210_ad_e65905);
        }

        if ((scratch.values[2444] != 0.0) && (scratch.values[2445] != 0.0)) {
            scratch.store_ad(534, &AdValue::mul(scratch.ad_value(740), AdValue::offset(scratch.ad_value(528), (-1.0))));
        }

        if ((scratch.values[2444] != 0.0) && (scratch.values[2445] != 0.0)) {
            scratch.values[535] = 0.0;
            scratch.node_derivatives[535] = [0.0; Instance::NODE_COUNT];
            scratch.branch_derivatives[535] = [0.0; Instance::BRANCH_COUNT];
        }

        scratch.values[2447] = if (scratch.values[737] > 0.0) { 1.0 } else { 0.0 };

        if (((scratch.values[2444] != 0.0) && (scratch.values[2445] != 0.0)) && (scratch.values[2447] != 0.0)) {
            scratch.store_ad(535, &AdValue::mul(scratch.ad_value(859), AdValue::add(scratch.ad_value(742), AdValue::mul(scratch.ad_value(859), scratch.ad_value(743)))));
        }

        if (((scratch.values[2444] != 0.0) && (scratch.values[2445] != 0.0)) && (!(scratch.values[2447] != 0.0))) {
            scratch.store_ad(527, &AdValue::mul(AdValue::scale(AdValue::neg(scratch.ad_value(859)), scratch.values[420]), scratch.ad_value(743)));
        }

        if (((scratch.values[2444] != 0.0) && (scratch.values[2445] != 0.0)) && (!(scratch.values[2447] != 0.0))) {
            let assign51270_ad_e65986: AdValue = {
                if (scratch.values[527] < (-230.25850929940458)) {
                    AdValue::div_from_scalar(1e-100, AdValue::offset(AdValue::sub_from_scalar((-230.25850929940458), scratch.ad_value(527)), 1.0))
                } else {
                    {
                        if (scratch.values[527] > scratch.values[735]) {
                            AdValue::mul(scratch.ad_value(736), AdValue::offset(AdValue::sub(scratch.ad_value(527), scratch.ad_value(735)), 1.0))
                        } else {
                            AdValue::exp(scratch.ad_value(527))
                        }
                    }
                }
            };
            scratch.store_ad(528, &assign51270_ad_e65986);
        }

        if (((scratch.values[2444] != 0.0) && (scratch.values[2445] != 0.0)) && (!(scratch.values[2447] != 0.0))) {
            scratch.store_ad(535, &AdValue::mul(AdValue::neg(scratch.ad_value(742)), AdValue::offset(scratch.ad_value(528), (-1.0))));
        }

        if ((scratch.values[2444] != 0.0) && (scratch.values[2445] != 0.0)) {
            scratch.store_ad(2037, &AdValue::add(AdValue::add(scratch.ad_value(533), scratch.ad_value(534)), scratch.ad_value(535)));
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
            scratch.values[2034] = 0.0;
            scratch.node_derivatives[2034] = [0.0; Instance::NODE_COUNT];
            scratch.branch_derivatives[2034] = [0.0; Instance::BRANCH_COUNT];
        }

    }

    pub(super) fn stamp_transient_block_39(
        &mut self,
        ctx: &GeneratedEvalContext<'_>,
        stamper: &mut GeneratedStamper<'_>,
        scratch: &mut Scratch,
    ) {
        let _ = stamper;
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

        if (((scratch.values[2444] != 0.0) && (!(scratch.values[2445] != 0.0))) && (!(scratch.values[2467] != 0.0))) {
            scratch.store_ad(664, &AdValue::scale(scratch.ad_value(654), scratch.values[436]));
        }

        scratch.values[2469] = if ((scratch.values[386] == 0.0) && (scratch.values[389] == 0.0)) { 1.0 } else { 0.0 };

        if ((((scratch.values[2444] != 0.0) && (!(scratch.values[2445] != 0.0))) && (!(scratch.values[2467] != 0.0))) && (scratch.values[2469] != 0.0)) {
            scratch.values[665] = 0.0;
            scratch.node_derivatives[665] = [0.0; Instance::NODE_COUNT];
            scratch.branch_derivatives[665] = [0.0; Instance::BRANCH_COUNT];
        }

        if ((((scratch.values[2444] != 0.0) && (!(scratch.values[2445] != 0.0))) && (!(scratch.values[2467] != 0.0))) && (!(scratch.values[2469] != 0.0))) {
            scratch.store_ad(666, &AdValue::sub_from_scalar(scratch.values[442], scratch.ad_value(660)));
        }

        if ((((scratch.values[2444] != 0.0) && (!(scratch.values[2445] != 0.0))) && (!(scratch.values[2467] != 0.0))) && (!(scratch.values[2469] != 0.0))) {
            scratch.store_ad(667, &AdValue::sub_from_scalar(1.0, AdValue::sqrt(AdValue::sub_from_scalar(1.0, AdValue::div(scratch.ad_value(658), scratch.ad_value(666))))));
        }

        scratch.values[2470] = if (scratch.values[375] == 0.5) { 1.0 } else { 0.0 };

        if (((((scratch.values[2444] != 0.0) && (!(scratch.values[2445] != 0.0))) && (!(scratch.values[2467] != 0.0))) && (!(scratch.values[2469] != 0.0))) && (scratch.values[2470] != 0.0)) {
            scratch.values[668] = 0.0;
            scratch.node_derivatives[668] = [0.0; Instance::NODE_COUNT];
            scratch.branch_derivatives[668] = [0.0; Instance::BRANCH_COUNT];
        }

        if (((((scratch.values[2444] != 0.0) && (!(scratch.values[2445] != 0.0))) && (!(scratch.values[2467] != 0.0))) && (!(scratch.values[2469] != 0.0))) && (!(scratch.values[2470] != 0.0))) {
            scratch.store_ad(668, &AdValue::scale(AdValue::add(AdValue::div(AdValue::mul(AdValue::square(scratch.ad_value(667)), AdValue::ln(scratch.ad_value(667))), AdValue::sub_from_scalar(1.0, scratch.ad_value(667))), scratch.ad_value(667)), (1.0 - (2.0 * scratch.values[375]))));
        }

        if ((((scratch.values[2444] != 0.0) && (!(scratch.values[2445] != 0.0))) && (!(scratch.values[2467] != 0.0))) && (!(scratch.values[2469] != 0.0))) {
            scratch.store_ad(669, &AdValue::add(scratch.ad_value(667), scratch.ad_value(668)));
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

        if ((((scratch.values[2444] != 0.0) && (!(scratch.values[2445] != 0.0))) && (!(scratch.values[2467] != 0.0))) && (!(scratch.values[2469] != 0.0))) {
            scratch.store_ad(671, &AdValue::scale(AdValue::mul(AdValue::offset(scratch.ad_value(657), (-1.0)), scratch.ad_value(670)), scratch.values[433]));
        }

        if ((((scratch.values[2444] != 0.0) && (!(scratch.values[2445] != 0.0))) && (!(scratch.values[2467] != 0.0))) && (!(scratch.values[2469] != 0.0))) {
            scratch.store_ad(665, &AdValue::scale(AdValue::mul(scratch.ad_value(671), scratch.ad_value(669)), scratch.values[386]));
        }

        scratch.values[2472] = if (scratch.values[389] == 0.0) { 1.0 } else { 0.0 };

        if ((((scratch.values[2444] != 0.0) && (!(scratch.values[2445] != 0.0))) && (!(scratch.values[2467] != 0.0))) && (scratch.values[2472] != 0.0)) {
            scratch.values[672] = 0.0;
            scratch.node_derivatives[672] = [0.0; Instance::NODE_COUNT];
            scratch.branch_derivatives[672] = [0.0; Instance::BRANCH_COUNT];
        }

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

        scratch.values[2473] = if (((-scratch.values[375]) * scratch.values[460]) == (-1.0)) { 1.0 } else { 0.0 };

        if (((((scratch.values[2444] != 0.0) && (!(scratch.values[2445] != 0.0))) && (!(scratch.values[2467] != 0.0))) && (!(scratch.values[2472] != 0.0))) && (scratch.values[2473] != 0.0)) {
            scratch.store_ad(679, &AdValue::div_from_scalar(1.0, AdValue::offset(AdValue::mul(scratch.ad_value(673), scratch.ad_value(678)), 1.0)));
        }

        if (((((scratch.values[2444] != 0.0) && (!(scratch.values[2445] != 0.0))) && (!(scratch.values[2467] != 0.0))) && (!(scratch.values[2472] != 0.0))) && (!(scratch.values[2473] != 0.0))) {
            scratch.store_ad(679, &AdValue::powf(AdValue::offset(AdValue::mul(scratch.ad_value(673), scratch.ad_value(678)), 1.0), ((-scratch.values[375]) * scratch.values[460])));
        }

        if ((((scratch.values[2444] != 0.0) && (!(scratch.values[2445] != 0.0))) && (!(scratch.values[2467] != 0.0))) && (!(scratch.values[2472] != 0.0))) {
            scratch.store_ad(680, &AdValue::div(AdValue::mul(scratch.ad_value(669), scratch.ad_value(679)), AdValue::add(scratch.ad_value(669), scratch.ad_value(679))));
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

        scratch.values[2474] = if (scratch.values[684] > 0.0) { 1.0 } else { 0.0 };

        if (((((scratch.values[2444] != 0.0) && (!(scratch.values[2445] != 0.0))) && (!(scratch.values[2467] != 0.0))) && (!(scratch.values[2472] != 0.0))) && (scratch.values[2474] != 0.0)) {
            scratch.store_ad(646, &AdValue::div_from_scalar(1.0, AdValue::offset(AdValue::scale(scratch.ad_value(684), scratch.values[421]), 1.0)));
        }

        if (((((scratch.values[2444] != 0.0) && (!(scratch.values[2445] != 0.0))) && (!(scratch.values[2467] != 0.0))) && (!(scratch.values[2472] != 0.0))) && (!(scratch.values[2474] != 0.0))) {
            scratch.store_ad(646, &AdValue::div_from_scalar(1.0, AdValue::sub_from_scalar(1.0, AdValue::scale(scratch.ad_value(684), scratch.values[421]))));
        }

        scratch.values[2475] = if (((-scratch.values[645]) + scratch.values[683]) > (-230.25850929940458)) { 1.0 } else { 0.0 };

        if (((((scratch.values[2444] != 0.0) && (!(scratch.values[2445] != 0.0))) && (!(scratch.values[2467] != 0.0))) && (!(scratch.values[2472] != 0.0))) && (scratch.values[2475] != 0.0)) {
            scratch.store_ad(663, &AdValue::exp(AdValue::sub(scratch.ad_value(683), scratch.ad_value(645))));
        }

        if (((((scratch.values[2444] != 0.0) && (!(scratch.values[2445] != 0.0))) && (!(scratch.values[2467] != 0.0))) && (!(scratch.values[2472] != 0.0))) && (!(scratch.values[2475] != 0.0))) {
            scratch.store_ad(663, &AdValue::div_from_scalar(1e-100, AdValue::offset(AdValue::mul(AdValue::sub_from_scalar((-230.25850929940458), AdValue::sub(scratch.ad_value(683), scratch.ad_value(645))), AdValue::offset(AdValue::scale(AdValue::mul(AdValue::sub_from_scalar((-230.25850929940458), AdValue::sub(scratch.ad_value(683), scratch.ad_value(645))), AdValue::offset(AdValue::scale(AdValue::sub_from_scalar((-230.25850929940458), AdValue::sub(scratch.ad_value(683), scratch.ad_value(645))), 0.3333333333333333), 1.0)), 0.5), 1.0)), 1.0)));
        }

        if ((((scratch.values[2444] != 0.0) && (!(scratch.values[2445] != 0.0))) && (!(scratch.values[2467] != 0.0))) && (!(scratch.values[2472] != 0.0))) {
            scratch.store_ad(647, &AdValue::mul(AdValue::add(AdValue::add(AdValue::scale(scratch.ad_value(646), 0.29214664), AdValue::scale(AdValue::square(scratch.ad_value(646)), scratch.values[422])), AdValue::scale(AdValue::mul(AdValue::square(scratch.ad_value(646)), scratch.ad_value(646)), scratch.values[423])), scratch.ad_value(663)));
        }

        scratch.values[2476] = if (scratch.values[684] > 0.0) { 1.0 } else { 0.0 };

        if (((((scratch.values[2444] != 0.0) && (!(scratch.values[2445] != 0.0))) && (!(scratch.values[2467] != 0.0))) && (!(scratch.values[2472] != 0.0))) && (scratch.values[2476] != 0.0)) {
            scratch.values[685] = scratch.values[647];
            scratch.node_derivatives[685] = scratch.node_derivatives[647];
            scratch.branch_derivatives[685] = scratch.branch_derivatives[647];
        }

        scratch.values[2477] = if (scratch.values[683] > (-230.25850929940458)) { 1.0 } else { 0.0 };

        if ((((((scratch.values[2444] != 0.0) && (!(scratch.values[2445] != 0.0))) && (!(scratch.values[2467] != 0.0))) && (!(scratch.values[2472] != 0.0))) && (!(scratch.values[2476] != 0.0))) && (scratch.values[2477] != 0.0)) {
            scratch.store_ad(663, &AdValue::exp(scratch.ad_value(683)));
        }

        if ((((((scratch.values[2444] != 0.0) && (!(scratch.values[2445] != 0.0))) && (!(scratch.values[2467] != 0.0))) && (!(scratch.values[2472] != 0.0))) && (!(scratch.values[2476] != 0.0))) && (!(scratch.values[2477] != 0.0))) {
            scratch.store_ad(663, &AdValue::div_from_scalar(1e-100, AdValue::offset(AdValue::mul(AdValue::sub_from_scalar((-230.25850929940458), scratch.ad_value(683)), AdValue::offset(AdValue::scale(AdValue::mul(AdValue::sub_from_scalar((-230.25850929940458), scratch.ad_value(683)), AdValue::offset(AdValue::scale(AdValue::sub_from_scalar((-230.25850929940458), scratch.ad_value(683)), 0.3333333333333333), 1.0)), 0.5), 1.0)), 1.0)));
        }

        if (((((scratch.values[2444] != 0.0) && (!(scratch.values[2445] != 0.0))) && (!(scratch.values[2467] != 0.0))) && (!(scratch.values[2472] != 0.0))) && (!(scratch.values[2476] != 0.0))) {
            scratch.store_ad(685, &AdValue::sub(AdValue::scale(scratch.ad_value(663), 2.0), scratch.ad_value(647)));
        }

        if ((((scratch.values[2444] != 0.0) && (!(scratch.values[2445] != 0.0))) && (!(scratch.values[2467] != 0.0))) && (!(scratch.values[2472] != 0.0))) {
            scratch.store_ad(686, &AdValue::scale(AdValue::div(AdValue::scale(scratch.ad_value(685), scratch.values[484]), scratch.ad_value(681)), (1.772453850905516 * 0.5)));
        }

        if ((((scratch.values[2444] != 0.0) && (!(scratch.values[2445] != 0.0))) && (!(scratch.values[2467] != 0.0))) && (!(scratch.values[2472] != 0.0))) {
            scratch.store_ad(672, &AdValue::scale(AdValue::mul(AdValue::mul(scratch.ad_value(671), scratch.ad_value(686)), scratch.ad_value(680)), scratch.values[389]));
        }

        scratch.values[2478] = if (scratch.values[395] == 0.0) { 1.0 } else { 0.0 };

        if ((((scratch.values[2444] != 0.0) && (!(scratch.values[2445] != 0.0))) && (!(scratch.values[2467] != 0.0))) && (scratch.values[2478] != 0.0)) {
            scratch.values[687] = 0.0;
            scratch.node_derivatives[687] = [0.0; Instance::NODE_COUNT];
            scratch.branch_derivatives[687] = [0.0; Instance::BRANCH_COUNT];
        }

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

        if ((((scratch.values[2444] != 0.0) && (!(scratch.values[2445] != 0.0))) && (!(scratch.values[2467] != 0.0))) && (!(scratch.values[2478] != 0.0))) {
            scratch.store_ad(687, &AdValue::scale(AdValue::mul(AdValue::mul(AdValue::mul(scratch.ad_value(858), scratch.ad_value(688)), scratch.ad_value(688)), scratch.ad_value(663)), scratch.values[395]));
        }

        scratch.values[2482] = if (scratch.values[404] > 1000.0) { 1.0 } else { 0.0 };

        if ((((scratch.values[2444] != 0.0) && (!(scratch.values[2445] != 0.0))) && (!(scratch.values[2467] != 0.0))) && (scratch.values[2482] != 0.0)) {
            scratch.values[689] = 1.0;
            scratch.node_derivatives[689] = [0.0; Instance::NODE_COUNT];
            scratch.branch_derivatives[689] = [0.0; Instance::BRANCH_COUNT];
        }

        scratch.values[2483] = if (scratch.values[662] > ((-scratch.values[493]) * scratch.values[404])) { 1.0 } else { 0.0 };

        scratch.values[2484] = if (scratch.values[407] == 4.0) { 1.0 } else { 0.0 };

        if ((((((scratch.values[2444] != 0.0) && (!(scratch.values[2445] != 0.0))) && (!(scratch.values[2467] != 0.0))) && (!(scratch.values[2482] != 0.0))) && (scratch.values[2483] != 0.0)) && (scratch.values[2484] != 0.0)) {
            scratch.store_ad(663, &AdValue::mul(AdValue::mul(AdValue::mul(AdValue::scale(scratch.ad_value(662), scratch.values[497]), AdValue::scale(scratch.ad_value(662), scratch.values[497])), AdValue::scale(scratch.ad_value(662), scratch.values[497])), AdValue::scale(scratch.ad_value(662), scratch.values[497])));
        }

        if ((((((scratch.values[2444] != 0.0) && (!(scratch.values[2445] != 0.0))) && (!(scratch.values[2467] != 0.0))) && (!(scratch.values[2482] != 0.0))) && (scratch.values[2483] != 0.0)) && (!(scratch.values[2484] != 0.0))) {
            scratch.store_ad(663, &AdValue::powf(AdValue::abs(AdValue::scale(scratch.ad_value(662), scratch.values[497])), scratch.values[407]));
        }

        if (((((scratch.values[2444] != 0.0) && (!(scratch.values[2445] != 0.0))) && (!(scratch.values[2467] != 0.0))) && (!(scratch.values[2482] != 0.0))) && (scratch.values[2483] != 0.0)) {
            scratch.store_ad(689, &AdValue::div_from_scalar(1.0, AdValue::sub_from_scalar(1.0, scratch.ad_value(663))));
        }

        if (((((scratch.values[2444] != 0.0) && (!(scratch.values[2445] != 0.0))) && (!(scratch.values[2467] != 0.0))) && (!(scratch.values[2482] != 0.0))) && (!(scratch.values[2483] != 0.0))) {
            scratch.store_ad(689, &AdValue::offset(AdValue::scale(AdValue::offset(scratch.ad_value(662), (scratch.values[493] * scratch.values[404])), scratch.values[500]), scratch.values[494]));
        }

        if (((scratch.values[2444] != 0.0) && (!(scratch.values[2445] != 0.0))) && (!(scratch.values[2467] != 0.0))) {
            scratch.store_ad(2034, &AdValue::mul(AdValue::add(AdValue::add(AdValue::add(scratch.ad_value(664), scratch.ad_value(665)), scratch.ad_value(672)), scratch.ad_value(687)), scratch.ad_value(689)));
        }

        scratch.values[2485] = if (scratch.values[691] == 0.0) { 1.0 } else { 0.0 };

        if (((scratch.values[2444] != 0.0) && (!(scratch.values[2445] != 0.0))) && (scratch.values[2485] != 0.0)) {
            scratch.values[2035] = 0.0;
            scratch.node_derivatives[2035] = [0.0; Instance::NODE_COUNT];
            scratch.branch_derivatives[2035] = [0.0; Instance::BRANCH_COUNT];
        }

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

        if (((scratch.values[2444] != 0.0) && (!(scratch.values[2445] != 0.0))) && (!(scratch.values[2485] != 0.0))) {
            scratch.store_ad(664, &AdValue::scale(scratch.ad_value(654), scratch.values[437]));
        }

        scratch.values[2487] = if ((scratch.values[387] == 0.0) && (scratch.values[390] == 0.0)) { 1.0 } else { 0.0 };

        if ((((scratch.values[2444] != 0.0) && (!(scratch.values[2445] != 0.0))) && (!(scratch.values[2485] != 0.0))) && (scratch.values[2487] != 0.0)) {
            scratch.values[665] = 0.0;
            scratch.node_derivatives[665] = [0.0; Instance::NODE_COUNT];
            scratch.branch_derivatives[665] = [0.0; Instance::BRANCH_COUNT];
        }

        if ((((scratch.values[2444] != 0.0) && (!(scratch.values[2445] != 0.0))) && (!(scratch.values[2485] != 0.0))) && (!(scratch.values[2487] != 0.0))) {
            scratch.store_ad(666, &AdValue::sub_from_scalar(scratch.values[443], scratch.ad_value(660)));
        }

        if ((((scratch.values[2444] != 0.0) && (!(scratch.values[2445] != 0.0))) && (!(scratch.values[2485] != 0.0))) && (!(scratch.values[2487] != 0.0))) {
            scratch.store_ad(667, &AdValue::sub_from_scalar(1.0, AdValue::sqrt(AdValue::sub_from_scalar(1.0, AdValue::div(scratch.ad_value(658), scratch.ad_value(666))))));
        }

        scratch.values[2488] = if (scratch.values[376] == 0.5) { 1.0 } else { 0.0 };

        if (((((scratch.values[2444] != 0.0) && (!(scratch.values[2445] != 0.0))) && (!(scratch.values[2485] != 0.0))) && (!(scratch.values[2487] != 0.0))) && (scratch.values[2488] != 0.0)) {
            scratch.values[668] = 0.0;
            scratch.node_derivatives[668] = [0.0; Instance::NODE_COUNT];
            scratch.branch_derivatives[668] = [0.0; Instance::BRANCH_COUNT];
        }

        if (((((scratch.values[2444] != 0.0) && (!(scratch.values[2445] != 0.0))) && (!(scratch.values[2485] != 0.0))) && (!(scratch.values[2487] != 0.0))) && (!(scratch.values[2488] != 0.0))) {
            scratch.store_ad(668, &AdValue::scale(AdValue::add(AdValue::div(AdValue::mul(AdValue::square(scratch.ad_value(667)), AdValue::ln(scratch.ad_value(667))), AdValue::sub_from_scalar(1.0, scratch.ad_value(667))), scratch.ad_value(667)), (1.0 - (2.0 * scratch.values[376]))));
        }

        if ((((scratch.values[2444] != 0.0) && (!(scratch.values[2445] != 0.0))) && (!(scratch.values[2485] != 0.0))) && (!(scratch.values[2487] != 0.0))) {
            scratch.store_ad(669, &AdValue::add(scratch.ad_value(667), scratch.ad_value(668)));
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

        if ((((scratch.values[2444] != 0.0) && (!(scratch.values[2445] != 0.0))) && (!(scratch.values[2485] != 0.0))) && (!(scratch.values[2487] != 0.0))) {
            scratch.store_ad(671, &AdValue::scale(AdValue::mul(AdValue::offset(scratch.ad_value(657), (-1.0)), scratch.ad_value(670)), scratch.values[434]));
        }

        if ((((scratch.values[2444] != 0.0) && (!(scratch.values[2445] != 0.0))) && (!(scratch.values[2485] != 0.0))) && (!(scratch.values[2487] != 0.0))) {
            scratch.store_ad(665, &AdValue::scale(AdValue::mul(scratch.ad_value(671), scratch.ad_value(669)), scratch.values[387]));
        }

        scratch.values[2490] = if (scratch.values[390] == 0.0) { 1.0 } else { 0.0 };

        if ((((scratch.values[2444] != 0.0) && (!(scratch.values[2445] != 0.0))) && (!(scratch.values[2485] != 0.0))) && (scratch.values[2490] != 0.0)) {
            scratch.values[672] = 0.0;
            scratch.node_derivatives[672] = [0.0; Instance::NODE_COUNT];
            scratch.branch_derivatives[672] = [0.0; Instance::BRANCH_COUNT];
        }

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

        scratch.values[2491] = if (((-scratch.values[376]) * scratch.values[461]) == (-1.0)) { 1.0 } else { 0.0 };

        if (((((scratch.values[2444] != 0.0) && (!(scratch.values[2445] != 0.0))) && (!(scratch.values[2485] != 0.0))) && (!(scratch.values[2490] != 0.0))) && (scratch.values[2491] != 0.0)) {
            scratch.store_ad(679, &AdValue::div_from_scalar(1.0, AdValue::offset(AdValue::mul(scratch.ad_value(673), scratch.ad_value(678)), 1.0)));
        }

        if (((((scratch.values[2444] != 0.0) && (!(scratch.values[2445] != 0.0))) && (!(scratch.values[2485] != 0.0))) && (!(scratch.values[2490] != 0.0))) && (!(scratch.values[2491] != 0.0))) {
            scratch.store_ad(679, &AdValue::powf(AdValue::offset(AdValue::mul(scratch.ad_value(673), scratch.ad_value(678)), 1.0), ((-scratch.values[376]) * scratch.values[461])));
        }

        if ((((scratch.values[2444] != 0.0) && (!(scratch.values[2445] != 0.0))) && (!(scratch.values[2485] != 0.0))) && (!(scratch.values[2490] != 0.0))) {
            scratch.store_ad(680, &AdValue::div(AdValue::mul(scratch.ad_value(669), scratch.ad_value(679)), AdValue::add(scratch.ad_value(669), scratch.ad_value(679))));
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

        scratch.values[2492] = if (scratch.values[684] > 0.0) { 1.0 } else { 0.0 };

        if (((((scratch.values[2444] != 0.0) && (!(scratch.values[2445] != 0.0))) && (!(scratch.values[2485] != 0.0))) && (!(scratch.values[2490] != 0.0))) && (scratch.values[2492] != 0.0)) {
            scratch.store_ad(646, &AdValue::div_from_scalar(1.0, AdValue::offset(AdValue::scale(scratch.ad_value(684), scratch.values[421]), 1.0)));
        }

        if (((((scratch.values[2444] != 0.0) && (!(scratch.values[2445] != 0.0))) && (!(scratch.values[2485] != 0.0))) && (!(scratch.values[2490] != 0.0))) && (!(scratch.values[2492] != 0.0))) {
            scratch.store_ad(646, &AdValue::div_from_scalar(1.0, AdValue::sub_from_scalar(1.0, AdValue::scale(scratch.ad_value(684), scratch.values[421]))));
        }

        scratch.values[2493] = if (((-scratch.values[645]) + scratch.values[683]) > (-230.25850929940458)) { 1.0 } else { 0.0 };

        if (((((scratch.values[2444] != 0.0) && (!(scratch.values[2445] != 0.0))) && (!(scratch.values[2485] != 0.0))) && (!(scratch.values[2490] != 0.0))) && (scratch.values[2493] != 0.0)) {
            scratch.store_ad(663, &AdValue::exp(AdValue::sub(scratch.ad_value(683), scratch.ad_value(645))));
        }

        if (((((scratch.values[2444] != 0.0) && (!(scratch.values[2445] != 0.0))) && (!(scratch.values[2485] != 0.0))) && (!(scratch.values[2490] != 0.0))) && (!(scratch.values[2493] != 0.0))) {
            scratch.store_ad(663, &AdValue::div_from_scalar(1e-100, AdValue::offset(AdValue::mul(AdValue::sub_from_scalar((-230.25850929940458), AdValue::sub(scratch.ad_value(683), scratch.ad_value(645))), AdValue::offset(AdValue::scale(AdValue::mul(AdValue::sub_from_scalar((-230.25850929940458), AdValue::sub(scratch.ad_value(683), scratch.ad_value(645))), AdValue::offset(AdValue::scale(AdValue::sub_from_scalar((-230.25850929940458), AdValue::sub(scratch.ad_value(683), scratch.ad_value(645))), 0.3333333333333333), 1.0)), 0.5), 1.0)), 1.0)));
        }

        if ((((scratch.values[2444] != 0.0) && (!(scratch.values[2445] != 0.0))) && (!(scratch.values[2485] != 0.0))) && (!(scratch.values[2490] != 0.0))) {
            scratch.store_ad(647, &AdValue::mul(AdValue::add(AdValue::add(AdValue::scale(scratch.ad_value(646), 0.29214664), AdValue::scale(AdValue::square(scratch.ad_value(646)), scratch.values[422])), AdValue::scale(AdValue::mul(AdValue::square(scratch.ad_value(646)), scratch.ad_value(646)), scratch.values[423])), scratch.ad_value(663)));
        }

        scratch.values[2494] = if (scratch.values[684] > 0.0) { 1.0 } else { 0.0 };

        if (((((scratch.values[2444] != 0.0) && (!(scratch.values[2445] != 0.0))) && (!(scratch.values[2485] != 0.0))) && (!(scratch.values[2490] != 0.0))) && (scratch.values[2494] != 0.0)) {
            scratch.values[685] = scratch.values[647];
            scratch.node_derivatives[685] = scratch.node_derivatives[647];
            scratch.branch_derivatives[685] = scratch.branch_derivatives[647];
        }

        scratch.values[2495] = if (scratch.values[683] > (-230.25850929940458)) { 1.0 } else { 0.0 };

        if ((((((scratch.values[2444] != 0.0) && (!(scratch.values[2445] != 0.0))) && (!(scratch.values[2485] != 0.0))) && (!(scratch.values[2490] != 0.0))) && (!(scratch.values[2494] != 0.0))) && (scratch.values[2495] != 0.0)) {
            scratch.store_ad(663, &AdValue::exp(scratch.ad_value(683)));
        }

        if ((((((scratch.values[2444] != 0.0) && (!(scratch.values[2445] != 0.0))) && (!(scratch.values[2485] != 0.0))) && (!(scratch.values[2490] != 0.0))) && (!(scratch.values[2494] != 0.0))) && (!(scratch.values[2495] != 0.0))) {
            scratch.store_ad(663, &AdValue::div_from_scalar(1e-100, AdValue::offset(AdValue::mul(AdValue::sub_from_scalar((-230.25850929940458), scratch.ad_value(683)), AdValue::offset(AdValue::scale(AdValue::mul(AdValue::sub_from_scalar((-230.25850929940458), scratch.ad_value(683)), AdValue::offset(AdValue::scale(AdValue::sub_from_scalar((-230.25850929940458), scratch.ad_value(683)), 0.3333333333333333), 1.0)), 0.5), 1.0)), 1.0)));
        }

        if (((((scratch.values[2444] != 0.0) && (!(scratch.values[2445] != 0.0))) && (!(scratch.values[2485] != 0.0))) && (!(scratch.values[2490] != 0.0))) && (!(scratch.values[2494] != 0.0))) {
            scratch.store_ad(685, &AdValue::sub(AdValue::scale(scratch.ad_value(663), 2.0), scratch.ad_value(647)));
        }

        if ((((scratch.values[2444] != 0.0) && (!(scratch.values[2445] != 0.0))) && (!(scratch.values[2485] != 0.0))) && (!(scratch.values[2490] != 0.0))) {
            scratch.store_ad(686, &AdValue::scale(AdValue::div(AdValue::scale(scratch.ad_value(685), scratch.values[485]), scratch.ad_value(681)), (1.772453850905516 * 0.5)));
        }

        if ((((scratch.values[2444] != 0.0) && (!(scratch.values[2445] != 0.0))) && (!(scratch.values[2485] != 0.0))) && (!(scratch.values[2490] != 0.0))) {
            scratch.store_ad(672, &AdValue::scale(AdValue::mul(AdValue::mul(scratch.ad_value(671), scratch.ad_value(686)), scratch.ad_value(680)), scratch.values[390]));
        }

        scratch.values[2496] = if (scratch.values[396] == 0.0) { 1.0 } else { 0.0 };

        if ((((scratch.values[2444] != 0.0) && (!(scratch.values[2445] != 0.0))) && (!(scratch.values[2485] != 0.0))) && (scratch.values[2496] != 0.0)) {
            scratch.values[687] = 0.0;
            scratch.node_derivatives[687] = [0.0; Instance::NODE_COUNT];
            scratch.branch_derivatives[687] = [0.0; Instance::BRANCH_COUNT];
        }

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

    }
}
