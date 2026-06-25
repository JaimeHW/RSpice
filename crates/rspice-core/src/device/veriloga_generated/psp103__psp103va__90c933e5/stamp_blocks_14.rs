#![allow(dead_code, unused_assignments, unused_imports, unused_parens, unused_variables)]

use super::{AdValue, GeneratedEvalContext, GeneratedReactiveStamper, GeneratedStamper, ReactiveScratch, THERMAL_VOLTAGE_PER_K};
use super::super::state::Instance;

impl Instance {

    pub(super) fn stamp_reactive_block_12(
        &mut self,
        ctx: &GeneratedEvalContext<'_>,
        stamper: &mut GeneratedReactiveStamper<'_>,
        scratch: &mut ReactiveScratch,
    ) {
        let _ = stamper;
        if ((!(scratch.values[2220] != 0.0)) && (scratch.values[2244] != 0.0)) {
            scratch.store_ad(2171, &AdValue::div(scratch.ad_value(2171), AdValue::offset(AdValue::mul(scratch.ad_value(2122), scratch.ad_value(2141)), 1.0)));
        }

        if (!(scratch.values[2220] != 0.0)) {
            scratch.store_ad(2172, &AdValue::scale(AdValue::mul(scratch.ad_value(2170), AdValue::offset(AdValue::sqrt(AdValue::offset(AdValue::scale(scratch.ad_value(2171), 2.0), 1.0)), 1.0)), 0.5));
        }

        if (!(scratch.values[2220] != 0.0)) {
            scratch.store_ad(2173, &AdValue::div_from_scalar(1.0, scratch.ad_value(2172)));
        }

        if (!(scratch.values[2220] != 0.0)) {
            scratch.store_ad(2174, &AdValue::mul(scratch.ad_value(2150), scratch.ad_value(2011)));
        }

        if (!(scratch.values[2220] != 0.0)) {
            scratch.store_ad(2076, &AdValue::mul(scratch.ad_value(2170), scratch.ad_value(2173)));
        }

        if (!(scratch.values[2220] != 0.0)) {
            scratch.store_ad(2175, &AdValue::mul(scratch.ad_value(2153), AdValue::offset(AdValue::scale(AdValue::mul(AdValue::mul(scratch.ad_value(2171), scratch.ad_value(2076)), scratch.ad_value(2076)), 0.5), 1.0)));
        }

        if (!(scratch.values[2220] != 0.0)) {
            scratch.store_ad(2176, &AdValue::div(AdValue::mul(scratch.ad_value(2076), scratch.ad_value(2163)), scratch.ad_value(2175)));
        }

        scratch.values[2013] = scratch.values[2092];
        scratch.node_derivatives[2013] = scratch.node_derivatives[2092];
        scratch.branch_derivatives[2013] = scratch.branch_derivatives[2092];

        scratch.values[879] = scratch.values[2166];
        scratch.node_derivatives[879] = scratch.node_derivatives[2166];
        scratch.branch_derivatives[879] = scratch.branch_derivatives[2166];

        scratch.values[880] = scratch.values[2174];
        scratch.node_derivatives[880] = scratch.node_derivatives[2174];
        scratch.branch_derivatives[880] = scratch.branch_derivatives[2174];

        scratch.values[2014] = scratch.values[2153];
        scratch.node_derivatives[2014] = scratch.node_derivatives[2153];
        scratch.branch_derivatives[2014] = scratch.branch_derivatives[2153];

        scratch.values[2015] = scratch.values[2141];
        scratch.node_derivatives[2015] = scratch.node_derivatives[2141];
        scratch.branch_derivatives[2015] = scratch.branch_derivatives[2141];

        scratch.values[2016] = scratch.values[2162];
        scratch.node_derivatives[2016] = scratch.node_derivatives[2162];
        scratch.branch_derivatives[2016] = scratch.branch_derivatives[2162];

        scratch.values[881] = scratch.values[2169];
        scratch.node_derivatives[881] = scratch.node_derivatives[2169];
        scratch.branch_derivatives[881] = scratch.branch_derivatives[2169];

        scratch.values[2018] = scratch.values[2176];
        scratch.node_derivatives[2018] = scratch.node_derivatives[2176];
        scratch.branch_derivatives[2018] = scratch.branch_derivatives[2176];

        scratch.values[882] = scratch.values[2151];
        scratch.node_derivatives[882] = scratch.node_derivatives[2151];
        scratch.branch_derivatives[882] = scratch.branch_derivatives[2151];

        scratch.values[883] = scratch.values[2172];
        scratch.node_derivatives[883] = scratch.node_derivatives[2172];
        scratch.branch_derivatives[883] = scratch.branch_derivatives[2172];

        scratch.values[884] = scratch.values[2170];
        scratch.node_derivatives[884] = scratch.node_derivatives[2170];
        scratch.branch_derivatives[884] = scratch.branch_derivatives[2170];

        scratch.values[885] = scratch.values[2138];
        scratch.node_derivatives[885] = scratch.node_derivatives[2138];
        scratch.branch_derivatives[885] = scratch.branch_derivatives[2138];

        scratch.values[886] = scratch.values[2145];
        scratch.node_derivatives[886] = scratch.node_derivatives[2145];
        scratch.branch_derivatives[886] = scratch.branch_derivatives[2145];

        scratch.values[889] = scratch.values[2133];
        scratch.node_derivatives[889] = scratch.node_derivatives[2133];
        scratch.branch_derivatives[889] = scratch.branch_derivatives[2133];

        scratch.values[2053] = scratch.values[2132];
        scratch.node_derivatives[2053] = scratch.node_derivatives[2132];
        scratch.branch_derivatives[2053] = scratch.branch_derivatives[2132];

        scratch.values[2247] = if (((((scratch.values[2] != 0.0) && ((scratch.values[252] > 0.0) || (scratch.values[253] > 0.0))) || ((scratch.values[4] != 0.0) && ((scratch.values[258] > 0.0) || (scratch.values[259] > 0.0)))) || (scratch.values[267] > 0.0)) || (scratch.values[268] > 0.0)) { 1.0 } else { 0.0 };

        if (scratch.values[2247] != 0.0) {
            scratch.store_ad(890, &AdValue::scale(AdValue::add(scratch.ad_value(871), AdValue::sqrt(AdValue::add(AdValue::square(scratch.ad_value(871)), scratch.ad_value(828)))), 0.5));
        }

        if (scratch.values[2247] != 0.0) {
            scratch.store_ad(891, &AdValue::add(AdValue::add(AdValue::sub(AdValue::neg(scratch.ad_value(890)), AdValue::scale(scratch.ad_value(826), 0.5)), AdValue::mul(scratch.ad_value(824), AdValue::sqrt(AdValue::add(AdValue::add(scratch.ad_value(890), AdValue::scale(scratch.ad_value(826), 0.25)), scratch.ad_value(829))))), scratch.ad_value(830)));
        }

        if (scratch.values[2247] != 0.0) {
            scratch.store_ad(890, &AdValue::scale(AdValue::add(scratch.ad_value(872), AdValue::sqrt(AdValue::add(AdValue::square(scratch.ad_value(872)), scratch.ad_value(831)))), 0.5));
        }

        if (scratch.values[2247] != 0.0) {
            scratch.store_ad(892, &AdValue::add(AdValue::add(AdValue::sub(AdValue::neg(scratch.ad_value(890)), AdValue::scale(scratch.ad_value(827), 0.5)), AdValue::mul(scratch.ad_value(825), AdValue::sqrt(AdValue::add(AdValue::add(scratch.ad_value(890), AdValue::scale(scratch.ad_value(827), 0.25)), scratch.ad_value(832))))), scratch.ad_value(833)));
        }

        if (scratch.values[2247] != 0.0) {
            scratch.store_ad(893, &AdValue::scale(AdValue::add(scratch.ad_value(871), scratch.ad_value(891)), (-scratch.values[356])));
        }

        if (scratch.values[2247] != 0.0) {
            scratch.store_ad(894, &AdValue::scale(AdValue::add(scratch.ad_value(872), scratch.ad_value(892)), (-scratch.values[356])));
        }

        scratch.values[2248] = if (scratch.values[2] != 0.0) { 1.0 } else { 0.0 };

        scratch.values[2249] = if (scratch.values[252] > 0.0) { 1.0 } else { 0.0 };

        if ((scratch.values[2248] != 0.0) && (scratch.values[2249] != 0.0)) {
            scratch.store_ad(895, &AdValue::mul(AdValue::sqrt(AdValue::offset(AdValue::square(scratch.ad_value(893)), 1e-6)), scratch.ad_value(834)));
        }

        scratch.values[2250] = if (scratch.values[256] < 0.0) { 1.0 } else { 0.0 };

        if (((scratch.values[2248] != 0.0) && (scratch.values[2249] != 0.0)) && (scratch.values[2250] != 0.0)) {
            scratch.store_ad(895, &AdValue::scale(AdValue::sub(AdValue::add(scratch.ad_value(895), scratch.ad_value(839)), AdValue::sqrt(AdValue::offset(AdValue::mul(AdValue::sub(scratch.ad_value(895), scratch.ad_value(839)), AdValue::sub(scratch.ad_value(895), scratch.ad_value(839))), 1e-6))), 0.5));
        }

        if ((scratch.values[2248] != 0.0) && (scratch.values[2249] != 0.0)) {
            scratch.store_ad(2076, &AdValue::mul(scratch.ad_value(837), AdValue::offset(AdValue::mul(scratch.ad_value(895), AdValue::add(scratch.ad_value(255), AdValue::mul(scratch.ad_value(256), scratch.ad_value(895)))), (-1.5))));
        }

        if ((scratch.values[2248] != 0.0) && (scratch.values[2249] != 0.0)) {
            scratch.store_ad(897, &AdValue::offset(scratch.ad_value(891), 3.0));
        }

        if ((scratch.values[2248] != 0.0) && (scratch.values[2249] != 0.0)) {
            scratch.store_ad(898, &AdValue::sub_from_scalar((-3.0), scratch.ad_value(250)));
        }

        if ((scratch.values[2248] != 0.0) && (scratch.values[2249] != 0.0)) {
            scratch.store_ad(899, &AdValue::scale(scratch.ad_value(860), 30.0));
        }

        if ((scratch.values[2248] != 0.0) && (scratch.values[2249] != 0.0)) {
            scratch.values[960] = (4.0 - 0.9);
            scratch.node_derivatives[960] = [0.0; Instance::NODE_COUNT];
            scratch.branch_derivatives[960] = [0.0; Instance::BRANCH_COUNT];
        }

        if ((scratch.values[2248] != 0.0) && (scratch.values[2249] != 0.0)) {
            scratch.store_ad(961, &AdValue::add(scratch.ad_value(897), scratch.ad_value(899)));
        }

        if ((scratch.values[2248] != 0.0) && (scratch.values[2249] != 0.0)) {
            scratch.store_ad(2076, &AdValue::mul(AdValue::div_from_scalar(2.0, scratch.ad_value(960)), AdValue::sub(scratch.ad_value(961), AdValue::sqrt(AdValue::sub(AdValue::square(scratch.ad_value(961)), AdValue::mul(AdValue::mul(scratch.ad_value(960), scratch.ad_value(897)), scratch.ad_value(899)))))));
        }

        if ((scratch.values[2248] != 0.0) && (scratch.values[2249] != 0.0)) {
            scratch.values[960] = (4.0 - 0.3);
            scratch.node_derivatives[960] = [0.0; Instance::NODE_COUNT];
            scratch.branch_derivatives[960] = [0.0; Instance::BRANCH_COUNT];
        }

        if ((scratch.values[2248] != 0.0) && (scratch.values[2249] != 0.0)) {
            scratch.store_ad(961, &AdValue::add(scratch.ad_value(898), scratch.ad_value(2076)));
        }

        scratch.values[2253] = if (scratch.values[253] > 0.0) { 1.0 } else { 0.0 };

        if ((scratch.values[2248] != 0.0) && (scratch.values[2253] != 0.0)) {
            scratch.store_ad(895, &AdValue::mul(AdValue::sqrt(AdValue::offset(AdValue::square(scratch.ad_value(894)), 1e-6)), scratch.ad_value(834)));
        }

        scratch.values[2254] = if (scratch.values[256] < 0.0) { 1.0 } else { 0.0 };

        if (((scratch.values[2248] != 0.0) && (scratch.values[2253] != 0.0)) && (scratch.values[2254] != 0.0)) {
            scratch.store_ad(895, &AdValue::scale(AdValue::sub(AdValue::add(scratch.ad_value(895), scratch.ad_value(839)), AdValue::sqrt(AdValue::offset(AdValue::mul(AdValue::sub(scratch.ad_value(895), scratch.ad_value(839)), AdValue::sub(scratch.ad_value(895), scratch.ad_value(839))), 1e-6))), 0.5));
        }

        if ((scratch.values[2248] != 0.0) && (scratch.values[2253] != 0.0)) {
            scratch.store_ad(2076, &AdValue::mul(scratch.ad_value(838), AdValue::offset(AdValue::mul(scratch.ad_value(895), AdValue::add(scratch.ad_value(255), AdValue::mul(scratch.ad_value(256), scratch.ad_value(895)))), (-1.5))));
        }

        if ((scratch.values[2248] != 0.0) && (scratch.values[2253] != 0.0)) {
            scratch.store_ad(897, &AdValue::offset(scratch.ad_value(892), 3.0));
        }

        if ((scratch.values[2248] != 0.0) && (scratch.values[2253] != 0.0)) {
            scratch.store_ad(898, &AdValue::sub_from_scalar((-3.0), scratch.ad_value(250)));
        }

        if ((scratch.values[2248] != 0.0) && (scratch.values[2253] != 0.0)) {
            scratch.store_ad(899, &AdValue::scale(scratch.ad_value(863), 30.0));
        }

        if ((scratch.values[2248] != 0.0) && (scratch.values[2253] != 0.0)) {
            scratch.values[960] = (4.0 - 0.9);
            scratch.node_derivatives[960] = [0.0; Instance::NODE_COUNT];
            scratch.branch_derivatives[960] = [0.0; Instance::BRANCH_COUNT];
        }

        if ((scratch.values[2248] != 0.0) && (scratch.values[2253] != 0.0)) {
            scratch.store_ad(961, &AdValue::add(scratch.ad_value(897), scratch.ad_value(899)));
        }

        if ((scratch.values[2248] != 0.0) && (scratch.values[2253] != 0.0)) {
            scratch.store_ad(2076, &AdValue::mul(AdValue::div_from_scalar(2.0, scratch.ad_value(960)), AdValue::sub(scratch.ad_value(961), AdValue::sqrt(AdValue::sub(AdValue::square(scratch.ad_value(961)), AdValue::mul(AdValue::mul(scratch.ad_value(960), scratch.ad_value(897)), scratch.ad_value(899)))))));
        }

        if ((scratch.values[2248] != 0.0) && (scratch.values[2253] != 0.0)) {
            scratch.values[960] = (4.0 - 0.3);
            scratch.node_derivatives[960] = [0.0; Instance::NODE_COUNT];
            scratch.branch_derivatives[960] = [0.0; Instance::BRANCH_COUNT];
        }

        if ((scratch.values[2248] != 0.0) && (scratch.values[2253] != 0.0)) {
            scratch.store_ad(961, &AdValue::add(scratch.ad_value(898), scratch.ad_value(2076)));
        }

        scratch.values[2257] = if (scratch.values[251] > 0.0) { 1.0 } else { 0.0 };

        scratch.values[2258] = if (scratch.values[2013] <= 0.0) { 1.0 } else { 0.0 };

        if (((scratch.values[2248] != 0.0) && (scratch.values[2257] != 0.0)) && (scratch.values[2258] != 0.0)) {
            scratch.store_ad(2076, &AdValue::pow(AdValue::div(scratch.ad_value(854), scratch.ad_value(869)), scratch.ad_value(240)));
        }

        if (((scratch.values[2248] != 0.0) && (scratch.values[2257] != 0.0)) && (scratch.values[2258] != 0.0)) {
            scratch.store_ad(889, &AdValue::mul(AdValue::mul(scratch.ad_value(854), AdValue::pow(AdValue::offset(scratch.ad_value(2076), 1.0), AdValue::neg(scratch.ad_value(820)))), scratch.ad_value(2012)));
        }

        scratch.values[2259] = if ((scratch.values[885] - scratch.values[889]) > (-230.25850929940458)) { 1.0 } else { 0.0 };

        if (((scratch.values[2248] != 0.0) && (scratch.values[2257] != 0.0)) && (scratch.values[2259] != 0.0)) {
            scratch.store_ad(2076, &AdValue::exp(AdValue::sub(scratch.ad_value(885), scratch.ad_value(889))));
        }

        if (((scratch.values[2248] != 0.0) && (scratch.values[2257] != 0.0)) && (!(scratch.values[2259] != 0.0))) {
            scratch.store_ad(2076, &AdValue::div_from_scalar(1e-100, AdValue::offset(AdValue::mul(AdValue::sub_from_scalar((-230.25850929940458), AdValue::sub(scratch.ad_value(885), scratch.ad_value(889))), AdValue::offset(AdValue::scale(AdValue::mul(AdValue::sub_from_scalar((-230.25850929940458), AdValue::sub(scratch.ad_value(885), scratch.ad_value(889))), AdValue::offset(AdValue::scale(AdValue::sub_from_scalar((-230.25850929940458), AdValue::sub(scratch.ad_value(885), scratch.ad_value(889))), 0.3333333333333333), 1.0)), 0.5), 1.0)), 1.0)));
        }

        if ((scratch.values[2248] != 0.0) && (scratch.values[2257] != 0.0)) {
            scratch.store_ad(901, &AdValue::add(scratch.ad_value(873), AdValue::mul(scratch.ad_value(2011), AdValue::sub(AdValue::scale(scratch.ad_value(885), 0.5), AdValue::ln(AdValue::scale(AdValue::offset(scratch.ad_value(2076), 1.0), 0.5))))));
        }

        if ((scratch.values[2248] != 0.0) && (scratch.values[2257] != 0.0)) {
            scratch.store_ad(902, &AdValue::mul(scratch.ad_value(250), scratch.ad_value(2011)));
        }

        if ((scratch.values[2248] != 0.0) && (scratch.values[2257] != 0.0)) {
            scratch.store_ad(903, &AdValue::add(scratch.ad_value(880), scratch.ad_value(902)));
        }

        if ((scratch.values[2248] != 0.0) && (scratch.values[2257] != 0.0)) {
            scratch.store_ad(904, &AdValue::scale(AdValue::sub(scratch.ad_value(903), AdValue::sqrt(AdValue::offset(AdValue::mul(AdValue::neg(scratch.ad_value(903)), AdValue::neg(scratch.ad_value(903))), 0.01))), 0.5));
        }

        if ((scratch.values[2248] != 0.0) && (scratch.values[2257] != 0.0)) {
            scratch.store_ad(895, &AdValue::mul(AdValue::sqrt(AdValue::offset(AdValue::square(scratch.ad_value(880)), 1e-6)), scratch.ad_value(834)));
        }

        scratch.values[2260] = if (scratch.values[256] < 0.0) { 1.0 } else { 0.0 };

        if (((scratch.values[2248] != 0.0) && (scratch.values[2257] != 0.0)) && (scratch.values[2260] != 0.0)) {
            scratch.store_ad(895, &AdValue::scale(AdValue::sub(AdValue::add(scratch.ad_value(895), scratch.ad_value(839)), AdValue::sqrt(AdValue::offset(AdValue::mul(AdValue::sub(scratch.ad_value(895), scratch.ad_value(839)), AdValue::sub(scratch.ad_value(895), scratch.ad_value(839))), 1e-6))), 0.5));
        }

        if ((scratch.values[2248] != 0.0) && (scratch.values[2257] != 0.0)) {
            scratch.store_ad(905, &AdValue::add(scratch.ad_value(886), AdValue::mul(AdValue::sub(AdValue::sub(scratch.ad_value(904), scratch.ad_value(785)), scratch.ad_value(901)), scratch.ad_value(2012))));
        }

        if ((scratch.values[2248] != 0.0) && (scratch.values[2257] != 0.0)) {
            scratch.store_ad(905, &AdValue::mul(AdValue::neg(AdValue::sub(AdValue::add(scratch.ad_value(853), scratch.ad_value(873)), scratch.ad_value(901))), scratch.ad_value(2012)));
        }

        scratch.values[2263] = if (((scratch.values[905]) as f64).abs() < 230.25850929940458) { 1.0 } else { 0.0 };

        if (((scratch.values[2248] != 0.0) && (scratch.values[2257] != 0.0)) && (scratch.values[2263] != 0.0)) {
            scratch.store_ad(2076, &AdValue::exp(scratch.ad_value(905)));
        }

        scratch.values[2264] = if (scratch.values[905] < 0.0) { 1.0 } else { 0.0 };

        if ((((scratch.values[2248] != 0.0) && (scratch.values[2257] != 0.0)) && (!(scratch.values[2263] != 0.0))) && (scratch.values[2264] != 0.0)) {
            scratch.store_ad(2076, &AdValue::div_from_scalar(1e-100, AdValue::offset(AdValue::mul(AdValue::sub_from_scalar((-230.25850929940458), scratch.ad_value(905)), AdValue::offset(AdValue::scale(AdValue::mul(AdValue::sub_from_scalar((-230.25850929940458), scratch.ad_value(905)), AdValue::offset(AdValue::scale(AdValue::sub_from_scalar((-230.25850929940458), scratch.ad_value(905)), 0.3333333333333333), 1.0)), 0.5), 1.0)), 1.0)));
        }

        if ((((scratch.values[2248] != 0.0) && (scratch.values[2257] != 0.0)) && (!(scratch.values[2263] != 0.0))) && (!(scratch.values[2264] != 0.0))) {
            scratch.store_ad(2076, &AdValue::scale(AdValue::offset(AdValue::mul(AdValue::offset(scratch.ad_value(905), (-230.25850929940458)), AdValue::offset(AdValue::scale(AdValue::mul(AdValue::offset(scratch.ad_value(905), (-230.25850929940458)), AdValue::offset(AdValue::scale(AdValue::offset(scratch.ad_value(905), (-230.25850929940458)), 0.3333333333333333), 1.0)), 0.5), 1.0)), 1.0), 1e100));
        }

        if ((scratch.values[2248] != 0.0) && (scratch.values[2257] != 0.0)) {
            scratch.store_ad(2076, &AdValue::mul(scratch.ad_value(836), AdValue::offset(AdValue::mul(scratch.ad_value(895), AdValue::add(scratch.ad_value(255), AdValue::mul(scratch.ad_value(256), scratch.ad_value(895)))), (-1.5))));
        }

        scratch.values[2267] = if ((scratch.values[2013] <= 0.0) || ((scratch.values[255] == 0.0) && (scratch.values[256] == 0.0))) { 1.0 } else { 0.0 };

        if (((scratch.values[2248] != 0.0) && (scratch.values[2257] != 0.0)) && (!(scratch.values[2267] != 0.0))) {
            scratch.store_ad(2076, &AdValue::add(scratch.ad_value(255), AdValue::mul(AdValue::scale(scratch.ad_value(256), 2.0), scratch.ad_value(895))));
        }

        if (((scratch.values[2248] != 0.0) && (scratch.values[2257] != 0.0)) && (!(scratch.values[2267] != 0.0))) {
            scratch.store_ad(911, &AdValue::div(scratch.ad_value(257), AdValue::mul(scratch.ad_value(2076), scratch.ad_value(836))));
        }

        if (((scratch.values[2248] != 0.0) && (scratch.values[2257] != 0.0)) && (!(scratch.values[2267] != 0.0))) {
            scratch.store_ad(912, &AdValue::scale(AdValue::div(scratch.ad_value(2015), scratch.ad_value(911)), 0.5));
        }

        scratch.values[2268] = if (scratch.values[912] < 0.001) { 1.0 } else { 0.0 };

        scratch.values[2269] = if (((scratch.values[912]) as f64).abs() < 230.25850929940458) { 1.0 } else { 0.0 };

        if (((((scratch.values[2248] != 0.0) && (scratch.values[2257] != 0.0)) && (!(scratch.values[2267] != 0.0))) && (!(scratch.values[2268] != 0.0))) && (scratch.values[2269] != 0.0)) {
            scratch.store_ad(918, &AdValue::exp(scratch.ad_value(912)));
        }

        scratch.values[2270] = if (scratch.values[912] < 0.0) { 1.0 } else { 0.0 };

        if ((((((scratch.values[2248] != 0.0) && (scratch.values[2257] != 0.0)) && (!(scratch.values[2267] != 0.0))) && (!(scratch.values[2268] != 0.0))) && (!(scratch.values[2269] != 0.0))) && (scratch.values[2270] != 0.0)) {
            scratch.store_ad(918, &AdValue::div_from_scalar(1e-100, AdValue::offset(AdValue::mul(AdValue::sub_from_scalar((-230.25850929940458), scratch.ad_value(912)), AdValue::offset(AdValue::scale(AdValue::mul(AdValue::sub_from_scalar((-230.25850929940458), scratch.ad_value(912)), AdValue::offset(AdValue::scale(AdValue::sub_from_scalar((-230.25850929940458), scratch.ad_value(912)), 0.3333333333333333), 1.0)), 0.5), 1.0)), 1.0)));
        }

        if ((((((scratch.values[2248] != 0.0) && (scratch.values[2257] != 0.0)) && (!(scratch.values[2267] != 0.0))) && (!(scratch.values[2268] != 0.0))) && (!(scratch.values[2269] != 0.0))) && (!(scratch.values[2270] != 0.0))) {
            scratch.store_ad(918, &AdValue::scale(AdValue::offset(AdValue::mul(AdValue::offset(scratch.ad_value(912), (-230.25850929940458)), AdValue::offset(AdValue::scale(AdValue::mul(AdValue::offset(scratch.ad_value(912), (-230.25850929940458)), AdValue::offset(AdValue::scale(AdValue::offset(scratch.ad_value(912), (-230.25850929940458)), 0.3333333333333333), 1.0)), 0.5), 1.0)), 1.0), 1e100));
        }

        if ((((scratch.values[2248] != 0.0) && (scratch.values[2257] != 0.0)) && (!(scratch.values[2267] != 0.0))) && (!(scratch.values[2268] != 0.0))) {
            scratch.store_ad(919, &AdValue::div_from_scalar(1.0, scratch.ad_value(918)));
        }

        if ((((scratch.values[2248] != 0.0) && (scratch.values[2257] != 0.0)) && (!(scratch.values[2267] != 0.0))) && (!(scratch.values[2268] != 0.0))) {
            scratch.store_ad(2076, &AdValue::sub(scratch.ad_value(918), scratch.ad_value(919)));
        }

        if ((((scratch.values[2248] != 0.0) && (scratch.values[2257] != 0.0)) && (!(scratch.values[2267] != 0.0))) && (!(scratch.values[2268] != 0.0))) {
            scratch.store_ad(2078, &AdValue::add(scratch.ad_value(918), scratch.ad_value(919)));
        }

        scratch.values[2271] = if (scratch.values[4] != 0.0) { 1.0 } else { 0.0 };

        scratch.values[2272] = if ((scratch.values[259] > 0.0) && (scratch.values[894] < 0.0)) { 1.0 } else { 0.0 };

        if ((scratch.values[2271] != 0.0) && (scratch.values[2272] != 0.0)) {
            scratch.store_ad(923, &AdValue::sqrt(AdValue::offset(AdValue::add(AdValue::square(scratch.ad_value(894)), AdValue::mul(AdValue::square(scratch.ad_value(265)), AdValue::square(scratch.ad_value(862)))), 1e-6)));
        }

        if ((scratch.values[2271] != 0.0) && (scratch.values[2272] != 0.0)) {
            scratch.store_ad(2076, &AdValue::div(AdValue::neg(scratch.ad_value(844)), scratch.ad_value(923)));
        }

        scratch.values[2273] = if (scratch.values[2076] > (-230.25850929940458)) { 1.0 } else { 0.0 };

        if (((scratch.values[2271] != 0.0) && (scratch.values[2272] != 0.0)) && (scratch.values[2273] != 0.0)) {
            scratch.store_ad(2078, &AdValue::exp(scratch.ad_value(2076)));
        }

        if (((scratch.values[2271] != 0.0) && (scratch.values[2272] != 0.0)) && (!(scratch.values[2273] != 0.0))) {
            scratch.store_ad(2078, &AdValue::div_from_scalar(1e-100, AdValue::offset(AdValue::mul(AdValue::sub_from_scalar((-230.25850929940458), scratch.ad_value(2076)), AdValue::offset(AdValue::scale(AdValue::mul(AdValue::sub_from_scalar((-230.25850929940458), scratch.ad_value(2076)), AdValue::offset(AdValue::scale(AdValue::sub_from_scalar((-230.25850929940458), scratch.ad_value(2076)), 0.3333333333333333), 1.0)), 0.5), 1.0)), 1.0)));
        }

        scratch.values[2274] = if ((scratch.values[258] > 0.0) && (scratch.values[893] < 0.0)) { 1.0 } else { 0.0 };

        if ((scratch.values[2271] != 0.0) && (scratch.values[2274] != 0.0)) {
            scratch.store_ad(925, &AdValue::sqrt(AdValue::offset(AdValue::add(AdValue::square(scratch.ad_value(893)), AdValue::mul(AdValue::square(scratch.ad_value(264)), AdValue::square(scratch.ad_value(861)))), 1e-6)));
        }

        if ((scratch.values[2271] != 0.0) && (scratch.values[2274] != 0.0)) {
            scratch.store_ad(2076, &AdValue::div(AdValue::neg(scratch.ad_value(843)), scratch.ad_value(925)));
        }

        scratch.values[2275] = if (scratch.values[2076] > (-230.25850929940458)) { 1.0 } else { 0.0 };

        if (((scratch.values[2271] != 0.0) && (scratch.values[2274] != 0.0)) && (scratch.values[2275] != 0.0)) {
            scratch.store_ad(2078, &AdValue::exp(scratch.ad_value(2076)));
        }

        if (((scratch.values[2271] != 0.0) && (scratch.values[2274] != 0.0)) && (!(scratch.values[2275] != 0.0))) {
            scratch.store_ad(2078, &AdValue::div_from_scalar(1e-100, AdValue::offset(AdValue::mul(AdValue::sub_from_scalar((-230.25850929940458), scratch.ad_value(2076)), AdValue::offset(AdValue::scale(AdValue::mul(AdValue::sub_from_scalar((-230.25850929940458), scratch.ad_value(2076)), AdValue::offset(AdValue::scale(AdValue::sub_from_scalar((-230.25850929940458), scratch.ad_value(2076)), 0.3333333333333333), 1.0)), 0.5), 1.0)), 1.0)));
        }

        scratch.values[2061] = 0.0;

        scratch.values[2067] = 0.0;

        scratch.values[2069] = 0.0;

        scratch.values[2070] = 1e-40;

        scratch.values[2276] = if ((scratch.values[8] != 0.0) && (scratch.values[283] > 0.0)) { 1.0 } else { 0.0 };

        if (scratch.values[2276] != 0.0) {
            scratch.store_ad(2076, &AdValue::add(AdValue::scale(AdValue::sub(AdValue::add(scratch.ad_value(856), scratch.ad_value(855)), AdValue::sqrt(AdValue::add(AdValue::mul(AdValue::sub(scratch.ad_value(856), scratch.ad_value(855)), AdValue::sub(scratch.ad_value(856), scratch.ad_value(855))), scratch.ad_value(807)))), 0.5), scratch.ad_value(805)));
        }

        if (scratch.values[2276] != 0.0) {
            scratch.store_ad(2054, &AdValue::add(AdValue::sub(scratch.ad_value(855), AdValue::scale(AdValue::sub(scratch.ad_value(2076), AdValue::sqrt(AdValue::add(AdValue::mul(scratch.ad_value(2076), scratch.ad_value(2076)), scratch.ad_value(806)))), 0.5)), scratch.ad_value(809)));
        }

        if (scratch.values[2276] != 0.0) {
            scratch.store_ad(2055, &AdValue::add(scratch.ad_value(2054), AdValue::scale(AdValue::sub(scratch.ad_value(854), scratch.ad_value(864)), 0.5)));
        }

        if (scratch.values[2276] != 0.0) {
            scratch.store_ad(2056, &AdValue::mul(AdValue::mul(scratch.ad_value(285), AdValue::offset(AdValue::mul(scratch.ad_value(287), scratch.ad_value(864)), 1.0)), AdValue::offset(AdValue::mul(scratch.ad_value(286), scratch.ad_value(2055)), 1.0)));
        }

        if (scratch.values[2276] != 0.0) {
            scratch.store_ad(2057, &AdValue::mul(scratch.ad_value(766), AdValue::offset(scratch.ad_value(2056), 1.0)));
        }

        if (scratch.values[2276] != 0.0) {
            scratch.store_ad(2058, &AdValue::div_from_scalar(1.0, scratch.ad_value(2057)));
        }

        scratch.values[2277] = if (scratch.values[289] < 0.05) { 1.0 } else { 0.0 };

        if ((scratch.values[2276] != 0.0) && (scratch.values[2277] != 0.0)) {
            scratch.values[2059] = scratch.values[864];
            scratch.node_derivatives[2059] = scratch.node_derivatives[864];
            scratch.branch_derivatives[2059] = scratch.branch_derivatives[864];
        }

        if ((scratch.values[2276] != 0.0) && (!(scratch.values[2277] != 0.0))) {
            scratch.store_ad(2059, &AdValue::div(AdValue::scale(AdValue::offset(AdValue::sqrt(AdValue::offset(AdValue::mul(scratch.ad_value(289), scratch.ad_value(864)), 1.0)), (-1.0)), 2.0), scratch.ad_value(289)));
        }

        if (scratch.values[2276] != 0.0) {
            scratch.store_ad(2060, &AdValue::mul(AdValue::mul(scratch.ad_value(288), scratch.ad_value(2059)), AdValue::offset(AdValue::mul(scratch.ad_value(290), scratch.ad_value(2055)), 1.0)));
        }

        if (scratch.values[2276] != 0.0) {
            scratch.store_ad(2061, &AdValue::mul(scratch.ad_value(2058), AdValue::sub(AdValue::add(AdValue::add(scratch.ad_value(853), scratch.ad_value(2054)), scratch.ad_value(2060)), scratch.ad_value(757))));
        }

        if (scratch.values[2276] != 0.0) {
            scratch.store_ad(2062, &AdValue::mul(scratch.ad_value(2058), scratch.ad_value(803)));
        }

        if (scratch.values[2276] != 0.0) {
            scratch.store_ad(2063, &AdValue::scale(AdValue::ln(AdValue::add(AdValue::div(scratch.ad_value(2062), scratch.ad_value(804)), AdValue::sqrt(scratch.ad_value(2062)))), 2.0));
        }

        if (scratch.values[2276] != 0.0) {
            scratch.store_ad(2064, &AdValue::mul(scratch.ad_value(2058), scratch.ad_value(2054)));
        }

        if (scratch.values[2276] != 0.0) {
            scratch.store_ad(967, &AdValue::add(scratch.ad_value(2062), scratch.ad_value(2064)));
        }

        if (scratch.values[2276] != 0.0) {
            scratch.store_ad(968, &AdValue::add(scratch.ad_value(967), AdValue::mul(scratch.ad_value(804), AdValue::sqrt(scratch.ad_value(967)))));
        }

        if (scratch.values[2276] != 0.0) {
            scratch.store_ad(969, &AdValue::add(scratch.ad_value(968), scratch.ad_value(2063)));
        }

        if (scratch.values[2276] != 0.0) {
            scratch.store_ad(970, &AdValue::offset(AdValue::div(scratch.ad_value(804), AdValue::scale(AdValue::sqrt(scratch.ad_value(967)), 2.0)), 1.0));
        }

        if (scratch.values[2276] != 0.0) {
            scratch.store_ad(971, &AdValue::div_from_scalar(1.0, scratch.ad_value(970)));
        }

        if (scratch.values[2276] != 0.0) {
            scratch.store_ad(972, &AdValue::sub(scratch.ad_value(2061), scratch.ad_value(969)));
        }

        scratch.values[2278] = if (scratch.values[972] > (-12.0)) { 1.0 } else { 0.0 };

        if ((scratch.values[2276] != 0.0) && (scratch.values[2278] != 0.0)) {
            scratch.store_ad(973, &AdValue::offset(AdValue::add(scratch.ad_value(972), scratch.ad_value(768)), (-1.0)));
        }

        if ((scratch.values[2276] != 0.0) && (scratch.values[2278] != 0.0)) {
            scratch.store_ad(974, &AdValue::scale(AdValue::add(scratch.ad_value(973), AdValue::sqrt(AdValue::offset(AdValue::square(scratch.ad_value(973)), 10.0))), 0.5));
        }

        if ((scratch.values[2276] != 0.0) && (scratch.values[2278] != 0.0)) {
            scratch.store_ad(975, &AdValue::add(AdValue::sub(scratch.ad_value(972), AdValue::mul(scratch.ad_value(970), AdValue::ln(scratch.ad_value(974)))), scratch.ad_value(768)));
        }

        if ((scratch.values[2276] != 0.0) && (scratch.values[2278] != 0.0)) {
            scratch.store_ad(976, &AdValue::scale(AdValue::add(scratch.ad_value(975), AdValue::sqrt(AdValue::offset(AdValue::square(scratch.ad_value(975)), 2.0))), 0.5));
        }

        scratch.values[2279] = if ((scratch.values[972] - scratch.values[976]) < 230.25850929940458) { 1.0 } else { 0.0 };

        if (((scratch.values[2276] != 0.0) && (scratch.values[2278] != 0.0)) && (scratch.values[2279] != 0.0)) {
            scratch.store_ad(977, &AdValue::exp(AdValue::sub(scratch.ad_value(972), scratch.ad_value(976))));
        }

        if (((scratch.values[2276] != 0.0) && (scratch.values[2278] != 0.0)) && (!(scratch.values[2279] != 0.0))) {
            scratch.store_ad(977, &AdValue::scale(AdValue::offset(AdValue::mul(AdValue::offset(AdValue::sub(scratch.ad_value(972), scratch.ad_value(976)), (-230.25850929940458)), AdValue::offset(AdValue::scale(AdValue::mul(AdValue::offset(AdValue::sub(scratch.ad_value(972), scratch.ad_value(976)), (-230.25850929940458)), AdValue::offset(AdValue::scale(AdValue::offset(AdValue::sub(scratch.ad_value(972), scratch.ad_value(976)), (-230.25850929940458)), 0.3333333333333333), 1.0)), 0.5), 1.0)), 1.0), 1e100));
        }

        if ((scratch.values[2276] != 0.0) && (scratch.values[2278] != 0.0)) {
            scratch.store_ad(978, &AdValue::mul(scratch.ad_value(767), scratch.ad_value(977)));
        }

        if ((scratch.values[2276] != 0.0) && (scratch.values[2278] != 0.0)) {
            scratch.store_ad(979, &AdValue::pow(scratch.ad_value(978), scratch.ad_value(971)));
        }

        if ((scratch.values[2276] != 0.0) && (scratch.values[2278] != 0.0)) {
            scratch.store_ad(980, &AdValue::add(AdValue::square(scratch.ad_value(970)), AdValue::mul(AdValue::sub(AdValue::scale(AdValue::add(scratch.ad_value(976), scratch.ad_value(970)), 2.0), scratch.ad_value(979)), scratch.ad_value(979))));
        }

    }

    pub(super) fn stamp_reactive_block_13(
        &mut self,
        ctx: &GeneratedEvalContext<'_>,
        stamper: &mut GeneratedReactiveStamper<'_>,
        scratch: &mut ReactiveScratch,
    ) {
        let _ = stamper;
        if ((scratch.values[2276] != 0.0) && (scratch.values[2278] != 0.0)) {
            scratch.store_ad(981, &AdValue::mul(scratch.ad_value(970), AdValue::offset(AdValue::div(AdValue::sub(AdValue::sqrt(scratch.ad_value(980)), scratch.ad_value(970)), scratch.ad_value(979)), (-1.0))));
        }

        if ((scratch.values[2276] != 0.0) && (scratch.values[2278] != 0.0)) {
            scratch.store_ad(2065, &AdValue::sub(scratch.ad_value(976), scratch.ad_value(981)));
        }

        scratch.values[2280] = if ((scratch.values[971] * (scratch.values[972] + scratch.values[768])) > (-230.25850929940458)) { 1.0 } else { 0.0 };

        if (((scratch.values[2276] != 0.0) && (!(scratch.values[2278] != 0.0))) && (scratch.values[2280] != 0.0)) {
            scratch.store_ad(2065, &AdValue::exp(AdValue::mul(scratch.ad_value(971), AdValue::add(scratch.ad_value(972), scratch.ad_value(768)))));
        }

        if (((scratch.values[2276] != 0.0) && (!(scratch.values[2278] != 0.0))) && (!(scratch.values[2280] != 0.0))) {
            let assign46300_ad_e59603: AdValue = AdValue::mul(AdValue::sub_from_scalar((-230.25850929940458), AdValue::mul(scratch.ad_value(971), AdValue::add(scratch.ad_value(972), scratch.ad_value(768)))), AdValue::offset(AdValue::scale(AdValue::mul(AdValue::sub_from_scalar((-230.25850929940458), AdValue::mul(scratch.ad_value(971), AdValue::add(scratch.ad_value(972), scratch.ad_value(768)))), AdValue::offset(AdValue::scale(AdValue::sub_from_scalar((-230.25850929940458), AdValue::mul(scratch.ad_value(971), AdValue::add(scratch.ad_value(972), scratch.ad_value(768)))), 0.3333333333333333), 1.0)), 0.5), 1.0));
            scratch.store_ad(2065, &AdValue::div_from_scalar(1e-100, AdValue::offset(assign46300_ad_e59603, 1.0)));
        }

        if (scratch.values[2276] != 0.0) {
            scratch.store_ad(2066, &AdValue::mul(scratch.ad_value(2058), AdValue::add(scratch.ad_value(2053), scratch.ad_value(2054))));
        }

        scratch.values[2281] = if ((scratch.values[2065] < 0.001) && (scratch.values[2053] < 1e-6)) { 1.0 } else { 0.0 };

        scratch.values[2282] = if (((-scratch.values[2066]) + scratch.values[2064]) > (-230.25850929940458)) { 1.0 } else { 0.0 };

        if (((scratch.values[2276] != 0.0) && (scratch.values[2281] != 0.0)) && (scratch.values[2282] != 0.0)) {
            scratch.store_ad(2076, &AdValue::exp(AdValue::sub(scratch.ad_value(2064), scratch.ad_value(2066))));
        }

        if (((scratch.values[2276] != 0.0) && (scratch.values[2281] != 0.0)) && (!(scratch.values[2282] != 0.0))) {
            let assign46350_ad_e59682: AdValue = AdValue::div_from_scalar(1e-100, AdValue::offset(AdValue::mul(AdValue::sub_from_scalar((-230.25850929940458), AdValue::sub(scratch.ad_value(2064), scratch.ad_value(2066))), AdValue::offset(AdValue::scale(AdValue::mul(AdValue::sub_from_scalar((-230.25850929940458), AdValue::sub(scratch.ad_value(2064), scratch.ad_value(2066))), AdValue::offset(AdValue::scale(AdValue::sub_from_scalar((-230.25850929940458), AdValue::sub(scratch.ad_value(2064), scratch.ad_value(2066))), 0.3333333333333333), 1.0)), 0.5), 1.0)), 1.0));
            scratch.store_ad(2076, &assign46350_ad_e59682);
        }

        if ((scratch.values[2276] != 0.0) && (scratch.values[2281] != 0.0)) {
            scratch.store_ad(2067, &AdValue::mul(scratch.ad_value(2065), AdValue::offset(scratch.ad_value(2076), (-1.0))));
        }

        if ((scratch.values[2276] != 0.0) && (scratch.values[2281] != 0.0)) {
            scratch.store_ad(2068, &AdValue::add(scratch.ad_value(2067), scratch.ad_value(2065)));
        }

        if ((scratch.values[2276] != 0.0) && (!(scratch.values[2281] != 0.0))) {
            scratch.store_ad(967, &AdValue::add(scratch.ad_value(2062), scratch.ad_value(2066)));
        }

        if ((scratch.values[2276] != 0.0) && (!(scratch.values[2281] != 0.0))) {
            scratch.store_ad(968, &AdValue::add(scratch.ad_value(967), AdValue::mul(scratch.ad_value(804), AdValue::sqrt(scratch.ad_value(967)))));
        }

        if ((scratch.values[2276] != 0.0) && (!(scratch.values[2281] != 0.0))) {
            scratch.store_ad(969, &AdValue::add(scratch.ad_value(968), scratch.ad_value(2063)));
        }

        if ((scratch.values[2276] != 0.0) && (!(scratch.values[2281] != 0.0))) {
            scratch.store_ad(970, &AdValue::offset(AdValue::div(scratch.ad_value(804), AdValue::scale(AdValue::sqrt(scratch.ad_value(967)), 2.0)), 1.0));
        }

        if ((scratch.values[2276] != 0.0) && (!(scratch.values[2281] != 0.0))) {
            scratch.store_ad(971, &AdValue::div_from_scalar(1.0, scratch.ad_value(970)));
        }

        if ((scratch.values[2276] != 0.0) && (!(scratch.values[2281] != 0.0))) {
            scratch.store_ad(972, &AdValue::sub(scratch.ad_value(2061), scratch.ad_value(969)));
        }

        scratch.values[2283] = if (scratch.values[972] > (-12.0)) { 1.0 } else { 0.0 };

        if (((scratch.values[2276] != 0.0) && (!(scratch.values[2281] != 0.0))) && (scratch.values[2283] != 0.0)) {
            scratch.store_ad(973, &AdValue::offset(AdValue::add(scratch.ad_value(972), scratch.ad_value(768)), (-1.0)));
        }

        if (((scratch.values[2276] != 0.0) && (!(scratch.values[2281] != 0.0))) && (scratch.values[2283] != 0.0)) {
            scratch.store_ad(974, &AdValue::scale(AdValue::add(scratch.ad_value(973), AdValue::sqrt(AdValue::offset(AdValue::square(scratch.ad_value(973)), 10.0))), 0.5));
        }

        if (((scratch.values[2276] != 0.0) && (!(scratch.values[2281] != 0.0))) && (scratch.values[2283] != 0.0)) {
            scratch.store_ad(975, &AdValue::add(AdValue::sub(scratch.ad_value(972), AdValue::mul(scratch.ad_value(970), AdValue::ln(scratch.ad_value(974)))), scratch.ad_value(768)));
        }

        if (((scratch.values[2276] != 0.0) && (!(scratch.values[2281] != 0.0))) && (scratch.values[2283] != 0.0)) {
            scratch.store_ad(976, &AdValue::scale(AdValue::add(scratch.ad_value(975), AdValue::sqrt(AdValue::offset(AdValue::square(scratch.ad_value(975)), 2.0))), 0.5));
        }

        scratch.values[2284] = if ((scratch.values[972] - scratch.values[976]) < 230.25850929940458) { 1.0 } else { 0.0 };

        if ((((scratch.values[2276] != 0.0) && (!(scratch.values[2281] != 0.0))) && (scratch.values[2283] != 0.0)) && (scratch.values[2284] != 0.0)) {
            scratch.store_ad(977, &AdValue::exp(AdValue::sub(scratch.ad_value(972), scratch.ad_value(976))));
        }

        if ((((scratch.values[2276] != 0.0) && (!(scratch.values[2281] != 0.0))) && (scratch.values[2283] != 0.0)) && (!(scratch.values[2284] != 0.0))) {
            scratch.store_ad(977, &AdValue::scale(AdValue::offset(AdValue::mul(AdValue::offset(AdValue::sub(scratch.ad_value(972), scratch.ad_value(976)), (-230.25850929940458)), AdValue::offset(AdValue::scale(AdValue::mul(AdValue::offset(AdValue::sub(scratch.ad_value(972), scratch.ad_value(976)), (-230.25850929940458)), AdValue::offset(AdValue::scale(AdValue::offset(AdValue::sub(scratch.ad_value(972), scratch.ad_value(976)), (-230.25850929940458)), 0.3333333333333333), 1.0)), 0.5), 1.0)), 1.0), 1e100));
        }

        if (((scratch.values[2276] != 0.0) && (!(scratch.values[2281] != 0.0))) && (scratch.values[2283] != 0.0)) {
            scratch.store_ad(978, &AdValue::mul(scratch.ad_value(767), scratch.ad_value(977)));
        }

        if (((scratch.values[2276] != 0.0) && (!(scratch.values[2281] != 0.0))) && (scratch.values[2283] != 0.0)) {
            scratch.store_ad(979, &AdValue::pow(scratch.ad_value(978), scratch.ad_value(971)));
        }

        if (((scratch.values[2276] != 0.0) && (!(scratch.values[2281] != 0.0))) && (scratch.values[2283] != 0.0)) {
            scratch.store_ad(980, &AdValue::add(AdValue::square(scratch.ad_value(970)), AdValue::mul(AdValue::sub(AdValue::scale(AdValue::add(scratch.ad_value(976), scratch.ad_value(970)), 2.0), scratch.ad_value(979)), scratch.ad_value(979))));
        }

        if (((scratch.values[2276] != 0.0) && (!(scratch.values[2281] != 0.0))) && (scratch.values[2283] != 0.0)) {
            scratch.store_ad(981, &AdValue::mul(scratch.ad_value(970), AdValue::offset(AdValue::div(AdValue::sub(AdValue::sqrt(scratch.ad_value(980)), scratch.ad_value(970)), scratch.ad_value(979)), (-1.0))));
        }

        if (((scratch.values[2276] != 0.0) && (!(scratch.values[2281] != 0.0))) && (scratch.values[2283] != 0.0)) {
            scratch.store_ad(2068, &AdValue::sub(scratch.ad_value(976), scratch.ad_value(981)));
        }

        scratch.values[2285] = if ((scratch.values[971] * (scratch.values[972] + scratch.values[768])) > (-230.25850929940458)) { 1.0 } else { 0.0 };

        if ((((scratch.values[2276] != 0.0) && (!(scratch.values[2281] != 0.0))) && (!(scratch.values[2283] != 0.0))) && (scratch.values[2285] != 0.0)) {
            scratch.store_ad(2068, &AdValue::exp(AdValue::mul(scratch.ad_value(971), AdValue::add(scratch.ad_value(972), scratch.ad_value(768)))));
        }

        if ((((scratch.values[2276] != 0.0) && (!(scratch.values[2281] != 0.0))) && (!(scratch.values[2283] != 0.0))) && (!(scratch.values[2285] != 0.0))) {
            let assign46590_ad_e60035: AdValue = AdValue::mul(AdValue::sub_from_scalar((-230.25850929940458), AdValue::mul(scratch.ad_value(971), AdValue::add(scratch.ad_value(972), scratch.ad_value(768)))), AdValue::offset(AdValue::scale(AdValue::mul(AdValue::sub_from_scalar((-230.25850929940458), AdValue::mul(scratch.ad_value(971), AdValue::add(scratch.ad_value(972), scratch.ad_value(768)))), AdValue::offset(AdValue::scale(AdValue::sub_from_scalar((-230.25850929940458), AdValue::mul(scratch.ad_value(971), AdValue::add(scratch.ad_value(972), scratch.ad_value(768)))), 0.3333333333333333), 1.0)), 0.5), 1.0));
            scratch.store_ad(2068, &AdValue::div_from_scalar(1e-100, AdValue::offset(assign46590_ad_e60035, 1.0)));
        }

        if ((scratch.values[2276] != 0.0) && (!(scratch.values[2281] != 0.0))) {
            scratch.store_ad(2067, &AdValue::sub(scratch.ad_value(2068), scratch.ad_value(2065)));
        }

        if (scratch.values[2276] != 0.0) {
            scratch.store_ad(2069, &AdValue::scale(AdValue::add(scratch.ad_value(2068), scratch.ad_value(2065)), 0.5));
        }

        if (scratch.values[2276] != 0.0) {
            scratch.store_ad(2070, &AdValue::max_with_scalar(AdValue::sub(scratch.ad_value(2061), scratch.ad_value(2069)), 1e-40));
        }

        scratch.values[2286] = if ((scratch.values[2013] > 0.0) && (scratch.values[3] != 0.0)) { 1.0 } else { 0.0 };

        if (scratch.values[2286] != 0.0) {
            scratch.store_ad(927, &AdValue::sub(scratch.ad_value(854), AdValue::mul(scratch.ad_value(248), scratch.ad_value(2015))));
        }

        scratch.values[2287] = if (scratch.values[927] > 0.0) { 1.0 } else { 0.0 };

        if ((scratch.values[2286] != 0.0) && (scratch.values[2287] != 0.0)) {
            scratch.store_ad(2078, &AdValue::mul(scratch.ad_value(756), AdValue::div(AdValue::offset(AdValue::mul(scratch.ad_value(249), AdValue::sub(AdValue::sqrt(AdValue::add(scratch.ad_value(771), scratch.ad_value(873))), scratch.ad_value(779))), 1.0), AdValue::offset(scratch.ad_value(927), 1e-30))));
        }

        scratch.values[2288] = if ((((-scratch.values[2078])) as f64).abs() < 230.25850929940458) { 1.0 } else { 0.0 };

        if (((scratch.values[2286] != 0.0) && (scratch.values[2287] != 0.0)) && (scratch.values[2288] != 0.0)) {
            scratch.store_ad(2076, &AdValue::exp(AdValue::neg(scratch.ad_value(2078))));
        }

        scratch.values[2289] = if ((-scratch.values[2078]) < 0.0) { 1.0 } else { 0.0 };

        if ((((scratch.values[2286] != 0.0) && (scratch.values[2287] != 0.0)) && (!(scratch.values[2288] != 0.0))) && (scratch.values[2289] != 0.0)) {
            scratch.store_ad(2076, &AdValue::div_from_scalar(1e-100, AdValue::offset(AdValue::mul(AdValue::sub_from_scalar((-230.25850929940458), AdValue::neg(scratch.ad_value(2078))), AdValue::offset(AdValue::scale(AdValue::mul(AdValue::sub_from_scalar((-230.25850929940458), AdValue::neg(scratch.ad_value(2078))), AdValue::offset(AdValue::scale(AdValue::sub_from_scalar((-230.25850929940458), AdValue::neg(scratch.ad_value(2078))), 0.3333333333333333), 1.0)), 0.5), 1.0)), 1.0)));
        }

        if ((((scratch.values[2286] != 0.0) && (scratch.values[2287] != 0.0)) && (!(scratch.values[2288] != 0.0))) && (!(scratch.values[2289] != 0.0))) {
            scratch.store_ad(2076, &AdValue::scale(AdValue::offset(AdValue::mul(AdValue::offset(AdValue::neg(scratch.ad_value(2078)), (-230.25850929940458)), AdValue::offset(AdValue::scale(AdValue::mul(AdValue::offset(AdValue::neg(scratch.ad_value(2078)), (-230.25850929940458)), AdValue::offset(AdValue::scale(AdValue::offset(AdValue::neg(scratch.ad_value(2078)), (-230.25850929940458)), 0.3333333333333333), 1.0)), 0.5), 1.0)), 1.0), 1e100));
        }

        scratch.values[2404] = if ((scratch.values[7] == 1.0) || (scratch.values[9] != 0.0)) { 1.0 } else { 0.0 };

        scratch.values[2405] = if (scratch.values[9] != 0.0) { 1.0 } else { 0.0 };

        if ((scratch.values[2404] != 0.0) && (scratch.values[2405] != 0.0)) {
            scratch.store_ad(2076, &AdValue::add(AdValue::scale(AdValue::sub(AdValue::add(scratch.ad_value(856), scratch.ad_value(855)), AdValue::sqrt(AdValue::add(AdValue::mul(AdValue::sub(scratch.ad_value(856), scratch.ad_value(855)), AdValue::sub(scratch.ad_value(856), scratch.ad_value(855))), scratch.ad_value(792)))), 0.5), scratch.ad_value(790)));
        }

        if ((scratch.values[2404] != 0.0) && (scratch.values[2405] != 0.0)) {
            scratch.store_ad(928, &AdValue::add(AdValue::sub(scratch.ad_value(855), AdValue::scale(AdValue::sub(scratch.ad_value(2076), AdValue::sqrt(AdValue::add(AdValue::mul(scratch.ad_value(2076), scratch.ad_value(2076)), scratch.ad_value(791)))), 0.5)), scratch.ad_value(793)));
        }

        if ((scratch.values[2404] != 0.0) && (scratch.values[2405] != 0.0)) {
            scratch.values[2292] = scratch.values[928];
            scratch.node_derivatives[2292] = scratch.node_derivatives[928];
            scratch.branch_derivatives[2292] = scratch.branch_derivatives[928];
        }

        if ((scratch.values[2404] != 0.0) && (scratch.values[2405] != 0.0)) {
            scratch.values[2290] = scratch.values[788];
            scratch.node_derivatives[2290] = scratch.node_derivatives[788];
            scratch.branch_derivatives[2290] = scratch.branch_derivatives[788];
        }

        if ((scratch.values[2404] != 0.0) && (scratch.values[2405] != 0.0)) {
            scratch.values[2291] = scratch.values[789];
            scratch.node_derivatives[2291] = scratch.node_derivatives[789];
            scratch.branch_derivatives[2291] = scratch.branch_derivatives[789];
        }

        if ((scratch.values[2404] != 0.0) && (!(scratch.values[2405] != 0.0))) {
            scratch.values[2292] = scratch.values[874];
            scratch.node_derivatives[2292] = scratch.node_derivatives[874];
            scratch.branch_derivatives[2292] = scratch.branch_derivatives[874];
        }

        if ((scratch.values[2404] != 0.0) && (!(scratch.values[2405] != 0.0))) {
            scratch.values[2290] = scratch.values[771];
            scratch.node_derivatives[2290] = scratch.node_derivatives[771];
            scratch.branch_derivatives[2290] = scratch.branch_derivatives[771];
        }

        if ((scratch.values[2404] != 0.0) && (!(scratch.values[2405] != 0.0))) {
            scratch.values[2291] = scratch.values[772];
            scratch.node_derivatives[2291] = scratch.node_derivatives[772];
            scratch.branch_derivatives[2291] = scratch.branch_derivatives[772];
        }

        if (scratch.values[2404] != 0.0) {
            scratch.values[2355] = 0.0;
            scratch.node_derivatives[2355] = [0.0; Instance::NODE_COUNT];
            scratch.branch_derivatives[2355] = [0.0; Instance::BRANCH_COUNT];
        }

        if (scratch.values[2404] != 0.0) {
            scratch.values[2371] = 1.0;
            scratch.node_derivatives[2371] = [0.0; Instance::NODE_COUNT];
            scratch.branch_derivatives[2371] = [0.0; Instance::BRANCH_COUNT];
        }

        if (scratch.values[2404] != 0.0) {
            scratch.values[2370] = 0.0;
            scratch.node_derivatives[2370] = [0.0; Instance::NODE_COUNT];
            scratch.branch_derivatives[2370] = [0.0; Instance::BRANCH_COUNT];
        }

        if (scratch.values[2404] != 0.0) {
            scratch.values[2366] = 0.0;
            scratch.node_derivatives[2366] = [0.0; Instance::NODE_COUNT];
            scratch.branch_derivatives[2366] = [0.0; Instance::BRANCH_COUNT];
        }

        if (scratch.values[2404] != 0.0) {
            scratch.values[2343] = 0.0;
            scratch.node_derivatives[2343] = [0.0; Instance::NODE_COUNT];
            scratch.branch_derivatives[2343] = [0.0; Instance::BRANCH_COUNT];
        }

        if (scratch.values[2404] != 0.0) {
            scratch.values[2364] = 0.0;
            scratch.node_derivatives[2364] = [0.0; Instance::NODE_COUNT];
            scratch.branch_derivatives[2364] = [0.0; Instance::BRANCH_COUNT];
        }

        if (scratch.values[2404] != 0.0) {
            scratch.values[2365] = 0.0;
            scratch.node_derivatives[2365] = [0.0; Instance::NODE_COUNT];
            scratch.branch_derivatives[2365] = [0.0; Instance::BRANCH_COUNT];
        }

        if (scratch.values[2404] != 0.0) {
            scratch.values[2378] = 1.0;
            scratch.node_derivatives[2378] = [0.0; Instance::NODE_COUNT];
            scratch.branch_derivatives[2378] = [0.0; Instance::BRANCH_COUNT];
        }

        if (scratch.values[2404] != 0.0) {
            scratch.values[2369] = 0.0;
            scratch.node_derivatives[2369] = [0.0; Instance::NODE_COUNT];
            scratch.branch_derivatives[2369] = [0.0; Instance::BRANCH_COUNT];
        }

        if (scratch.values[2404] != 0.0) {
            scratch.values[2353] = 1.0;
            scratch.node_derivatives[2353] = [0.0; Instance::NODE_COUNT];
            scratch.branch_derivatives[2353] = [0.0; Instance::BRANCH_COUNT];
        }

        if (scratch.values[2404] != 0.0) {
            scratch.values[2374] = 1.0;
            scratch.node_derivatives[2374] = [0.0; Instance::NODE_COUNT];
            scratch.branch_derivatives[2374] = [0.0; Instance::BRANCH_COUNT];
        }

        if (scratch.values[2404] != 0.0) {
            scratch.values[2375] = 1.0;
            scratch.node_derivatives[2375] = [0.0; Instance::NODE_COUNT];
            scratch.branch_derivatives[2375] = [0.0; Instance::BRANCH_COUNT];
        }

        if (scratch.values[2404] != 0.0) {
            scratch.values[2400] = 0.0;
            scratch.node_derivatives[2400] = [0.0; Instance::NODE_COUNT];
            scratch.branch_derivatives[2400] = [0.0; Instance::BRANCH_COUNT];
        }

        if (scratch.values[2404] != 0.0) {
            scratch.values[2305] = 0.0;
            scratch.node_derivatives[2305] = [0.0; Instance::NODE_COUNT];
            scratch.branch_derivatives[2305] = [0.0; Instance::BRANCH_COUNT];
        }

        if (scratch.values[2404] != 0.0) {
            scratch.values[2354] = 0.0;
            scratch.node_derivatives[2354] = [0.0; Instance::NODE_COUNT];
            scratch.branch_derivatives[2354] = [0.0; Instance::BRANCH_COUNT];
        }

        if (scratch.values[2404] != 0.0) {
            scratch.values[2322] = 0.0;
            scratch.node_derivatives[2322] = [0.0; Instance::NODE_COUNT];
            scratch.branch_derivatives[2322] = [0.0; Instance::BRANCH_COUNT];
        }

        if (scratch.values[2404] != 0.0) {
            scratch.values[2317] = 0.0;
            scratch.node_derivatives[2317] = [0.0; Instance::NODE_COUNT];
            scratch.branch_derivatives[2317] = [0.0; Instance::BRANCH_COUNT];
        }

        if (scratch.values[2404] != 0.0) {
            scratch.values[2321] = 1.0;
            scratch.node_derivatives[2321] = [0.0; Instance::NODE_COUNT];
            scratch.branch_derivatives[2321] = [0.0; Instance::BRANCH_COUNT];
        }

        if (scratch.values[2404] != 0.0) {
            scratch.values[2372] = 1.0;
            scratch.node_derivatives[2372] = [0.0; Instance::NODE_COUNT];
            scratch.branch_derivatives[2372] = [0.0; Instance::BRANCH_COUNT];
        }

        if (scratch.values[2404] != 0.0) {
            scratch.values[2335] = 0.0;
            scratch.node_derivatives[2335] = [0.0; Instance::NODE_COUNT];
            scratch.branch_derivatives[2335] = [0.0; Instance::BRANCH_COUNT];
        }

        if (scratch.values[2404] != 0.0) {
            scratch.values[2324] = 0.0;
            scratch.node_derivatives[2324] = [0.0; Instance::NODE_COUNT];
            scratch.branch_derivatives[2324] = [0.0; Instance::BRANCH_COUNT];
        }

        if (scratch.values[2404] != 0.0) {
            scratch.values[2352] = 0.0;
            scratch.node_derivatives[2352] = [0.0; Instance::NODE_COUNT];
            scratch.branch_derivatives[2352] = [0.0; Instance::BRANCH_COUNT];
        }

        if (scratch.values[2404] != 0.0) {
            scratch.store_ad(865, &AdValue::add(scratch.ad_value(853), scratch.ad_value(2292)));
        }

        if (scratch.values[2404] != 0.0) {
            scratch.store_ad(866, &AdValue::sub(scratch.ad_value(865), scratch.ad_value(744)));
        }

        if (scratch.values[2404] != 0.0) {
            scratch.store_ad(2293, &AdValue::add(scratch.ad_value(2292), AdValue::scale(AdValue::sub(scratch.ad_value(854), scratch.ad_value(864)), 0.5)));
        }

        scratch.values[2406] = if (scratch.values[214] < 1e-10) { 1.0 } else { 0.0 };

        if ((scratch.values[2404] != 0.0) && (scratch.values[2406] != 0.0)) {
            scratch.values[867] = scratch.values[864];
            scratch.node_derivatives[867] = scratch.node_derivatives[864];
            scratch.branch_derivatives[867] = scratch.branch_derivatives[864];
        }

        if ((scratch.values[2404] != 0.0) && (!(scratch.values[2406] != 0.0))) {
            scratch.store_ad(867, &AdValue::div(AdValue::scale(AdValue::offset(AdValue::sqrt(AdValue::offset(AdValue::mul(scratch.ad_value(214), scratch.ad_value(864)), 1.0)), (-1.0)), 2.0), scratch.ad_value(214)));
        }

        if (scratch.values[2404] != 0.0) {
            scratch.store_ad(868, &AdValue::mul(AdValue::mul(scratch.ad_value(213), scratch.ad_value(867)), AdValue::offset(AdValue::mul(scratch.ad_value(215), scratch.ad_value(2293)), 1.0)));
        }

        if (scratch.values[2404] != 0.0) {
            scratch.store_ad(2003, &AdValue::mul(AdValue::mul(scratch.ad_value(216), AdValue::offset(AdValue::mul(scratch.ad_value(218), scratch.ad_value(864)), 1.0)), AdValue::offset(AdValue::mul(scratch.ad_value(217), scratch.ad_value(2293)), 1.0)));
        }

        if (scratch.values[2404] != 0.0) {
            scratch.store_ad(866, &AdValue::add(scratch.ad_value(866), scratch.ad_value(868)));
        }

        scratch.values[2407] = if (scratch.values[202] > 0.0) { 1.0 } else { 0.0 };

        if ((scratch.values[2404] != 0.0) && (scratch.values[2407] != 0.0)) {
            scratch.store_ad(2295, &AdValue::mul(AdValue::scale(scratch.ad_value(202), 0.5), AdValue::add(AdValue::sub(AdValue::add(scratch.ad_value(853), scratch.ad_value(855)), scratch.ad_value(200)), AdValue::sqrt(AdValue::add(AdValue::mul(AdValue::neg(AdValue::sub(AdValue::add(scratch.ad_value(853), scratch.ad_value(855)), scratch.ad_value(200))), AdValue::neg(AdValue::sub(AdValue::add(scratch.ad_value(853), scratch.ad_value(855)), scratch.ad_value(200)))), scratch.ad_value(201))))));
        }

        if ((scratch.values[2404] != 0.0) && (scratch.values[2407] != 0.0)) {
            scratch.store_ad(2296, &AdValue::mul(scratch.ad_value(2291), AdValue::sqrt(AdValue::offset(scratch.ad_value(2295), 1.0))));
        }

        if ((scratch.values[2404] != 0.0) && (!(scratch.values[2407] != 0.0))) {
            scratch.values[2296] = scratch.values[2291];
            scratch.node_derivatives[2296] = scratch.node_derivatives[2291];
            scratch.branch_derivatives[2296] = scratch.branch_derivatives[2291];
        }

        if (scratch.values[2404] != 0.0) {
            scratch.store_ad(2297, &AdValue::square(scratch.ad_value(2296)));
        }

        if (scratch.values[2404] != 0.0) {
            scratch.store_ad(2298, &AdValue::div_from_scalar(1.0, scratch.ad_value(2297)));
        }

        if (scratch.values[2404] != 0.0) {
            scratch.values[2009] = 1.0;
            scratch.node_derivatives[2009] = [0.0; Instance::NODE_COUNT];
            scratch.branch_derivatives[2009] = [0.0; Instance::BRANCH_COUNT];
        }

        scratch.values[2408] = if (scratch.values[207] > 0.0) { 1.0 } else { 0.0 };

        if ((scratch.values[2404] != 0.0) && (scratch.values[2408] != 0.0)) {
            scratch.store_ad(2004, &AdValue::scale(scratch.ad_value(866), (2.0 * scratch.values[363])));
        }

        if ((scratch.values[2404] != 0.0) && (scratch.values[2408] != 0.0)) {
            scratch.store_ad(2077, &AdValue::add(scratch.ad_value(2297), scratch.ad_value(2004)));
        }

        if ((scratch.values[2404] != 0.0) && (scratch.values[2408] != 0.0)) {
            scratch.store_ad(2078, &AdValue::scale(AdValue::add(AdValue::add(scratch.ad_value(2077), scratch.ad_value(2004)), AdValue::sqrt(AdValue::offset(AdValue::mul(AdValue::add(scratch.ad_value(2077), scratch.ad_value(2004)), AdValue::add(scratch.ad_value(2077), scratch.ad_value(2004))), 5.0))), 0.5));
        }

        if ((scratch.values[2404] != 0.0) && (scratch.values[2408] != 0.0)) {
            scratch.store_ad(2005, &AdValue::scale(AdValue::sub(scratch.ad_value(2077), AdValue::mul(scratch.ad_value(2296), AdValue::sqrt(scratch.ad_value(2078)))), 0.5));
        }

        if ((scratch.values[2404] != 0.0) && (scratch.values[2408] != 0.0)) {
            scratch.store_ad(2006, &AdValue::scale(scratch.ad_value(2290), scratch.values[363]));
        }

        if ((scratch.values[2404] != 0.0) && (scratch.values[2408] != 0.0)) {
            scratch.store_ad(2007, &AdValue::scale(scratch.ad_value(2293), scratch.values[363]));
        }

        if ((scratch.values[2404] != 0.0) && (scratch.values[2408] != 0.0)) {
            scratch.store_ad(2077, &AdValue::offset(AdValue::add(scratch.ad_value(2006), scratch.ad_value(2007)), 2.0));
        }

        if ((scratch.values[2404] != 0.0) && (scratch.values[2408] != 0.0)) {
            scratch.store_ad(2008, &AdValue::scale(AdValue::sub(AdValue::add(scratch.ad_value(2005), scratch.ad_value(2077)), AdValue::sqrt(AdValue::offset(AdValue::mul(AdValue::sub(scratch.ad_value(2005), scratch.ad_value(2077)), AdValue::sub(scratch.ad_value(2005), scratch.ad_value(2077))), 5.0))), 0.5));
        }

        if ((scratch.values[2404] != 0.0) && (scratch.values[2408] != 0.0)) {
            scratch.store_ad(2078, &AdValue::mul(scratch.ad_value(746), AdValue::sub(scratch.ad_value(2008), AdValue::mul(AdValue::offset(scratch.ad_value(208), 1.0), AdValue::add(AdValue::scale(scratch.ad_value(2006), 0.5), scratch.ad_value(2007))))));
        }

        scratch.values[2409] = if (scratch.values[2078] > (-230.25850929940458)) { 1.0 } else { 0.0 };

        if (((scratch.values[2404] != 0.0) && (scratch.values[2408] != 0.0)) && (scratch.values[2409] != 0.0)) {
            scratch.store_ad(2009, &AdValue::exp(scratch.ad_value(2078)));
        }

        if (((scratch.values[2404] != 0.0) && (scratch.values[2408] != 0.0)) && (!(scratch.values[2409] != 0.0))) {
            scratch.store_ad(2009, &AdValue::div_from_scalar(1e-100, AdValue::offset(AdValue::mul(AdValue::sub_from_scalar((-230.25850929940458), scratch.ad_value(2078)), AdValue::offset(AdValue::scale(AdValue::mul(AdValue::sub_from_scalar((-230.25850929940458), scratch.ad_value(2078)), AdValue::offset(AdValue::scale(AdValue::sub_from_scalar((-230.25850929940458), scratch.ad_value(2078)), 0.3333333333333333), 1.0)), 0.5), 1.0)), 1.0)));
        }

        if (scratch.values[2404] != 0.0) {
            scratch.store_ad(2010, &AdValue::offset(AdValue::mul(scratch.ad_value(745), scratch.ad_value(2009)), 1.0));
        }

        if (scratch.values[2404] != 0.0) {
            scratch.store_ad(2011, &AdValue::mul(AdValue::scale(scratch.ad_value(2010), scratch.values[759]), AdValue::offset(scratch.ad_value(2003), 1.0)));
        }

        if (scratch.values[2404] != 0.0) {
            scratch.store_ad(2012, &AdValue::div_from_scalar(1.0, scratch.ad_value(2011)));
        }

        if (scratch.values[2404] != 0.0) {
            scratch.store_ad(2294, &AdValue::mul(scratch.ad_value(866), scratch.ad_value(2012)));
        }

        if (scratch.values[2404] != 0.0) {
            scratch.store_ad(2299, &AdValue::offset(AdValue::scale(scratch.ad_value(2296), 0.7071067811865475), 1.0));
        }

        if (scratch.values[2404] != 0.0) {
            scratch.store_ad(2300, &AdValue::div_from_scalar(1.0, scratch.ad_value(2299)));
        }

        if (scratch.values[2404] != 0.0) {
            scratch.store_ad(2301, &AdValue::mul(scratch.ad_value(2292), scratch.ad_value(2012)));
        }

        if (scratch.values[2404] != 0.0) {
            scratch.store_ad(2302, &AdValue::add(AdValue::mul(scratch.ad_value(2290), scratch.ad_value(2012)), scratch.ad_value(2301)));
        }

        scratch.values[2410] = if (scratch.values[2302] < 460.51701859880916) { 1.0 } else { 0.0 };

        if ((scratch.values[2404] != 0.0) && (scratch.values[2410] != 0.0)) {
            scratch.store_ad(2303, &AdValue::exp(AdValue::neg(scratch.ad_value(2302))));
        }

        if ((scratch.values[2404] != 0.0) && (!(scratch.values[2410] != 0.0))) {
            scratch.store_ad(2303, &AdValue::div_from_scalar(1e-200, AdValue::offset(AdValue::mul(AdValue::offset(scratch.ad_value(2302), (-460.51701859880916)), AdValue::offset(AdValue::scale(AdValue::mul(AdValue::offset(scratch.ad_value(2302), (-460.51701859880916)), AdValue::offset(AdValue::scale(AdValue::offset(scratch.ad_value(2302), (-460.51701859880916)), 0.3333333333333333), 1.0)), 0.5), 1.0)), 1.0)));
        }

        if (scratch.values[2404] != 0.0) {
            scratch.store_ad(2304, &AdValue::scale(scratch.ad_value(2299), 1e-5));
        }

        scratch.values[2411] = if (((scratch.values[2294]) as f64).abs() <= scratch.values[2304]) { 1.0 } else { 0.0 };

    }

    pub(super) fn stamp_reactive_block_14(
        &mut self,
        ctx: &GeneratedEvalContext<'_>,
        stamper: &mut GeneratedReactiveStamper<'_>,
        scratch: &mut ReactiveScratch,
    ) {
        let _ = stamper;
        if ((scratch.values[2404] != 0.0) && (scratch.values[2411] != 0.0)) {
            scratch.store_ad(2380, &AdValue::scale(AdValue::square(scratch.ad_value(2300)), (0.16666666666666666 * 0.7071067811865475)));
        }

        if ((scratch.values[2404] != 0.0) && (scratch.values[2411] != 0.0)) {
            scratch.store_ad(2305, &AdValue::mul(AdValue::mul(scratch.ad_value(2294), scratch.ad_value(2300)), AdValue::offset(AdValue::mul(AdValue::mul(AdValue::mul(scratch.ad_value(2294), AdValue::sub_from_scalar(1.0, scratch.ad_value(2303))), scratch.ad_value(2296)), scratch.ad_value(2380)), 1.0)));
        }

        scratch.values[2412] = if (scratch.values[2294] < (-scratch.values[2304])) { 1.0 } else { 0.0 };

        if (((scratch.values[2404] != 0.0) && (!(scratch.values[2411] != 0.0))) && (scratch.values[2412] != 0.0)) {
            scratch.store_ad(2382, &AdValue::neg(scratch.ad_value(2294)));
        }

        if (((scratch.values[2404] != 0.0) && (!(scratch.values[2411] != 0.0))) && (scratch.values[2412] != 0.0)) {
            scratch.store_ad(2383, &AdValue::scale(AdValue::mul(scratch.ad_value(2382), scratch.ad_value(2300)), 1.25));
        }

        if (((scratch.values[2404] != 0.0) && (!(scratch.values[2411] != 0.0))) && (scratch.values[2412] != 0.0)) {
            scratch.store_ad(2384, &AdValue::scale(AdValue::sub(AdValue::offset(scratch.ad_value(2383), 10.0), AdValue::sqrt(AdValue::offset(AdValue::mul(AdValue::offset(scratch.ad_value(2383), (-6.0)), AdValue::offset(scratch.ad_value(2383), (-6.0))), 64.0))), 0.5));
        }

        if (((scratch.values[2404] != 0.0) && (!(scratch.values[2411] != 0.0))) && (scratch.values[2412] != 0.0)) {
            scratch.store_ad(2379, &AdValue::sub(scratch.ad_value(2382), scratch.ad_value(2384)));
        }

        if (((scratch.values[2404] != 0.0) && (!(scratch.values[2411] != 0.0))) && (scratch.values[2412] != 0.0)) {
            scratch.store_ad(2385, &AdValue::add(AdValue::square(scratch.ad_value(2379)), AdValue::mul(scratch.ad_value(2297), AdValue::offset(scratch.ad_value(2384), 1.0))));
        }

        if (((scratch.values[2404] != 0.0) && (!(scratch.values[2411] != 0.0))) && (scratch.values[2412] != 0.0)) {
            scratch.store_ad(2386, &AdValue::sub(AdValue::scale(scratch.ad_value(2379), 2.0), scratch.ad_value(2297)));
        }

        if (((scratch.values[2404] != 0.0) && (!(scratch.values[2411] != 0.0))) && (scratch.values[2412] != 0.0)) {
            scratch.store_ad(2387, &AdValue::sub(AdValue::ln(AdValue::mul(scratch.ad_value(2385), scratch.ad_value(2298))), scratch.ad_value(2384)));
        }

        if (((scratch.values[2404] != 0.0) && (!(scratch.values[2411] != 0.0))) && (scratch.values[2412] != 0.0)) {
            scratch.store_ad(966, &AdValue::add(scratch.ad_value(2385), scratch.ad_value(2386)));
        }

        if (((scratch.values[2404] != 0.0) && (!(scratch.values[2411] != 0.0))) && (scratch.values[2412] != 0.0)) {
            scratch.store_ad(965, &AdValue::add(AdValue::square(scratch.ad_value(966)), AdValue::mul(scratch.ad_value(2387), AdValue::sub(AdValue::scale(AdValue::square(scratch.ad_value(2386)), 0.5), scratch.ad_value(2385)))));
        }

        if (((scratch.values[2404] != 0.0) && (!(scratch.values[2411] != 0.0))) && (scratch.values[2412] != 0.0)) {
            scratch.store_ad(2388, &AdValue::add(scratch.ad_value(2384), AdValue::div(AdValue::mul(AdValue::mul(scratch.ad_value(2385), scratch.ad_value(966)), scratch.ad_value(2387)), AdValue::add(scratch.ad_value(965), AdValue::mul(AdValue::mul(AdValue::mul(AdValue::mul(AdValue::div(scratch.ad_value(966), scratch.ad_value(965)), scratch.ad_value(2387)), scratch.ad_value(2387)), scratch.ad_value(2386)), AdValue::sub(AdValue::scale(AdValue::square(scratch.ad_value(2386)), 0.3333333333333333), scratch.ad_value(2385)))))));
        }

        scratch.values[2413] = if (scratch.values[2388] < 230.25850929940458) { 1.0 } else { 0.0 };

        if ((((scratch.values[2404] != 0.0) && (!(scratch.values[2411] != 0.0))) && (scratch.values[2412] != 0.0)) && (scratch.values[2413] != 0.0)) {
            scratch.store_ad(2389, &AdValue::exp(scratch.ad_value(2388)));
        }

        if ((((scratch.values[2404] != 0.0) && (!(scratch.values[2411] != 0.0))) && (scratch.values[2412] != 0.0)) && (!(scratch.values[2413] != 0.0))) {
            scratch.store_ad(2389, &AdValue::scale(AdValue::offset(AdValue::mul(AdValue::offset(scratch.ad_value(2388), (-230.25850929940458)), AdValue::offset(AdValue::scale(AdValue::mul(AdValue::offset(scratch.ad_value(2388), (-230.25850929940458)), AdValue::offset(AdValue::scale(AdValue::offset(scratch.ad_value(2388), (-230.25850929940458)), 0.3333333333333333), 1.0)), 0.5), 1.0)), 1.0), 1e100));
        }

        if (((scratch.values[2404] != 0.0) && (!(scratch.values[2411] != 0.0))) && (scratch.values[2412] != 0.0)) {
            scratch.store_ad(2390, &AdValue::div_from_scalar(1.0, scratch.ad_value(2389)));
        }

        if (((scratch.values[2404] != 0.0) && (!(scratch.values[2411] != 0.0))) && (scratch.values[2412] != 0.0)) {
            scratch.store_ad(2379, &AdValue::div_from_scalar(1.0, AdValue::offset(AdValue::square(scratch.ad_value(2388)), 2.0)));
        }

        if (((scratch.values[2404] != 0.0) && (!(scratch.values[2411] != 0.0))) && (scratch.values[2412] != 0.0)) {
            scratch.store_ad(2391, &AdValue::mul(AdValue::square(scratch.ad_value(2388)), scratch.ad_value(2379)));
        }

        if (((scratch.values[2404] != 0.0) && (!(scratch.values[2411] != 0.0))) && (scratch.values[2412] != 0.0)) {
            scratch.store_ad(2392, &AdValue::scale(AdValue::mul(AdValue::mul(scratch.ad_value(2388), scratch.ad_value(2379)), scratch.ad_value(2379)), 4.0));
        }

        if (((scratch.values[2404] != 0.0) && (!(scratch.values[2411] != 0.0))) && (scratch.values[2412] != 0.0)) {
            scratch.store_ad(2393, &AdValue::mul(AdValue::mul(AdValue::sub(AdValue::scale(scratch.ad_value(2379), 8.0), AdValue::scale(scratch.ad_value(2391), 12.0)), scratch.ad_value(2379)), scratch.ad_value(2379)));
        }

        if (((scratch.values[2404] != 0.0) && (!(scratch.values[2411] != 0.0))) && (scratch.values[2412] != 0.0)) {
            scratch.store_ad(2379, &AdValue::sub(scratch.ad_value(2382), scratch.ad_value(2388)));
        }

        if (((scratch.values[2404] != 0.0) && (!(scratch.values[2411] != 0.0))) && (scratch.values[2412] != 0.0)) {
            scratch.store_ad(2380, &AdValue::mul(scratch.ad_value(2303), scratch.ad_value(2390)));
        }

        if (((scratch.values[2404] != 0.0) && (!(scratch.values[2411] != 0.0))) && (scratch.values[2412] != 0.0)) {
            scratch.store_ad(2394, &AdValue::add(AdValue::scale(scratch.ad_value(2379), 2.0), AdValue::mul(scratch.ad_value(2297), AdValue::add(AdValue::sub(AdValue::offset(scratch.ad_value(2389), (-1.0)), scratch.ad_value(2380)), AdValue::mul(scratch.ad_value(2303), AdValue::sub_from_scalar(1.0, scratch.ad_value(2392)))))));
        }

        if (((scratch.values[2404] != 0.0) && (!(scratch.values[2411] != 0.0))) && (scratch.values[2412] != 0.0)) {
            scratch.store_ad(2395, &AdValue::sub(AdValue::square(scratch.ad_value(2379)), AdValue::mul(scratch.ad_value(2297), AdValue::add(AdValue::add(AdValue::offset(AdValue::sub(scratch.ad_value(2389), scratch.ad_value(2388)), (-1.0)), scratch.ad_value(2380)), AdValue::mul(scratch.ad_value(2303), AdValue::sub(AdValue::offset(scratch.ad_value(2388), (-1.0)), scratch.ad_value(2391)))))));
        }

        if (((scratch.values[2404] != 0.0) && (!(scratch.values[2411] != 0.0))) && (scratch.values[2412] != 0.0)) {
            scratch.store_ad(2379, &AdValue::sub_from_scalar(2.0, AdValue::mul(scratch.ad_value(2297), AdValue::sub(AdValue::add(scratch.ad_value(2389), scratch.ad_value(2380)), AdValue::mul(scratch.ad_value(2303), scratch.ad_value(2393))))));
        }

        if (((scratch.values[2404] != 0.0) && (!(scratch.values[2411] != 0.0))) && (scratch.values[2412] != 0.0)) {
            scratch.store_ad(2379, &AdValue::sub(AdValue::square(scratch.ad_value(2394)), AdValue::scale(AdValue::mul(scratch.ad_value(2395), scratch.ad_value(2379)), 2.0)));
        }

        if (((scratch.values[2404] != 0.0) && (!(scratch.values[2411] != 0.0))) && (scratch.values[2412] != 0.0)) {
            scratch.store_ad(2305, &AdValue::sub(AdValue::neg(scratch.ad_value(2388)), AdValue::scale(AdValue::div(scratch.ad_value(2395), AdValue::add(scratch.ad_value(2394), AdValue::sqrt(scratch.ad_value(2379)))), 2.0)));
        }

        if (((scratch.values[2404] != 0.0) && (!(scratch.values[2411] != 0.0))) && (!(scratch.values[2412] != 0.0))) {
            scratch.store_ad(2396, &AdValue::div_from_scalar(1.0, AdValue::offset(AdValue::scale(scratch.ad_value(2296), 0.7324648775608221), 1.25)));
        }

        if (((scratch.values[2404] != 0.0) && (!(scratch.values[2411] != 0.0))) && (!(scratch.values[2412] != 0.0))) {
            scratch.store_ad(2397, &AdValue::mul(AdValue::offset(AdValue::mul(AdValue::scale(scratch.ad_value(2299), 1.25), scratch.ad_value(2396)), (-1.0)), scratch.ad_value(2396)));
        }

        if (((scratch.values[2404] != 0.0) && (!(scratch.values[2411] != 0.0))) && (!(scratch.values[2412] != 0.0))) {
            scratch.store_ad(2398, &AdValue::mul(AdValue::mul(scratch.ad_value(2294), scratch.ad_value(2300)), AdValue::offset(AdValue::mul(scratch.ad_value(2397), scratch.ad_value(2294)), 1.0)));
        }

        scratch.values[2414] = if ((-scratch.values[2398]) > (-230.25850929940458)) { 1.0 } else { 0.0 };

        if ((((scratch.values[2404] != 0.0) && (!(scratch.values[2411] != 0.0))) && (!(scratch.values[2412] != 0.0))) && (scratch.values[2414] != 0.0)) {
            scratch.store_ad(2379, &AdValue::exp(AdValue::neg(scratch.ad_value(2398))));
        }

        if ((((scratch.values[2404] != 0.0) && (!(scratch.values[2411] != 0.0))) && (!(scratch.values[2412] != 0.0))) && (!(scratch.values[2414] != 0.0))) {
            scratch.store_ad(2379, &AdValue::div_from_scalar(1e-100, AdValue::offset(AdValue::mul(AdValue::sub_from_scalar((-230.25850929940458), AdValue::neg(scratch.ad_value(2398))), AdValue::offset(AdValue::scale(AdValue::mul(AdValue::sub_from_scalar((-230.25850929940458), AdValue::neg(scratch.ad_value(2398))), AdValue::offset(AdValue::scale(AdValue::sub_from_scalar((-230.25850929940458), AdValue::neg(scratch.ad_value(2398))), 0.3333333333333333), 1.0)), 0.5), 1.0)), 1.0)));
        }

        if (((scratch.values[2404] != 0.0) && (!(scratch.values[2411] != 0.0))) && (!(scratch.values[2412] != 0.0))) {
            scratch.store_ad(2399, &AdValue::sub_from_scalar(1.0, scratch.ad_value(2379)));
        }

        if (((scratch.values[2404] != 0.0) && (!(scratch.values[2411] != 0.0))) && (!(scratch.values[2412] != 0.0))) {
            scratch.store_ad(2400, &AdValue::sub(AdValue::add(scratch.ad_value(2294), AdValue::scale(scratch.ad_value(2297), 0.5)), AdValue::mul(scratch.ad_value(2296), AdValue::sqrt(AdValue::sub(AdValue::add(scratch.ad_value(2294), AdValue::scale(scratch.ad_value(2297), 0.25)), scratch.ad_value(2399))))));
        }

        if (((scratch.values[2404] != 0.0) && (!(scratch.values[2411] != 0.0))) && (!(scratch.values[2412] != 0.0))) {
            scratch.store_ad(2401, &AdValue::offset(scratch.ad_value(2302), 3.0));
        }

        if (((scratch.values[2404] != 0.0) && (!(scratch.values[2411] != 0.0))) && (!(scratch.values[2412] != 0.0))) {
            scratch.store_ad(2384, &AdValue::sub(AdValue::scale(AdValue::sub(AdValue::add(scratch.ad_value(2400), scratch.ad_value(2401)), AdValue::sqrt(AdValue::offset(AdValue::mul(AdValue::sub(scratch.ad_value(2400), scratch.ad_value(2401)), AdValue::sub(scratch.ad_value(2400), scratch.ad_value(2401))), 5.0))), 0.5), AdValue::scale(AdValue::sub(scratch.ad_value(2401), AdValue::sqrt(AdValue::offset(AdValue::square(scratch.ad_value(2401)), 5.0))), 0.5)));
        }

        if (((scratch.values[2404] != 0.0) && (!(scratch.values[2411] != 0.0))) && (!(scratch.values[2412] != 0.0))) {
            scratch.store_ad(2379, &AdValue::sub(scratch.ad_value(2294), scratch.ad_value(2384)));
        }

        if (((scratch.values[2404] != 0.0) && (!(scratch.values[2411] != 0.0))) && (!(scratch.values[2412] != 0.0))) {
            scratch.store_ad(2380, &AdValue::exp(AdValue::neg(scratch.ad_value(2384))));
        }

        if (((scratch.values[2404] != 0.0) && (!(scratch.values[2411] != 0.0))) && (!(scratch.values[2412] != 0.0))) {
            scratch.store_ad(2381, &AdValue::div_from_scalar(1.0, AdValue::offset(AdValue::square(scratch.ad_value(2384)), 2.0)));
        }

        if (((scratch.values[2404] != 0.0) && (!(scratch.values[2411] != 0.0))) && (!(scratch.values[2412] != 0.0))) {
            scratch.store_ad(2391, &AdValue::mul(AdValue::square(scratch.ad_value(2384)), scratch.ad_value(2381)));
        }

        if (((scratch.values[2404] != 0.0) && (!(scratch.values[2411] != 0.0))) && (!(scratch.values[2412] != 0.0))) {
            scratch.store_ad(2392, &AdValue::scale(AdValue::mul(AdValue::mul(scratch.ad_value(2384), scratch.ad_value(2381)), scratch.ad_value(2381)), 4.0));
        }

        if (((scratch.values[2404] != 0.0) && (!(scratch.values[2411] != 0.0))) && (!(scratch.values[2412] != 0.0))) {
            scratch.store_ad(2393, &AdValue::mul(AdValue::mul(AdValue::sub(AdValue::scale(scratch.ad_value(2381), 8.0), AdValue::scale(scratch.ad_value(2391), 12.0)), scratch.ad_value(2381)), scratch.ad_value(2381)));
        }

        if (((scratch.values[2404] != 0.0) && (!(scratch.values[2411] != 0.0))) && (!(scratch.values[2412] != 0.0))) {
            scratch.store_ad(2385, &AdValue::max_from_scalar(1e-40, AdValue::sub(AdValue::square(scratch.ad_value(2379)), AdValue::mul(scratch.ad_value(2297), AdValue::sub(AdValue::offset(AdValue::add(scratch.ad_value(2380), scratch.ad_value(2384)), (-1.0)), AdValue::mul(scratch.ad_value(2303), AdValue::add(AdValue::offset(scratch.ad_value(2384), 1.0), scratch.ad_value(2391))))))));
        }

        if (((scratch.values[2404] != 0.0) && (!(scratch.values[2411] != 0.0))) && (!(scratch.values[2412] != 0.0))) {
            scratch.store_ad(2402, &AdValue::sub_from_scalar(1.0, AdValue::scale(AdValue::mul(scratch.ad_value(2297), AdValue::sub(scratch.ad_value(2380), AdValue::mul(scratch.ad_value(2303), scratch.ad_value(2393)))), 0.5)));
        }

        if (((scratch.values[2404] != 0.0) && (!(scratch.values[2411] != 0.0))) && (!(scratch.values[2412] != 0.0))) {
            scratch.store_ad(2386, &AdValue::add(AdValue::scale(scratch.ad_value(2379), 2.0), AdValue::mul(scratch.ad_value(2297), AdValue::sub(AdValue::sub_from_scalar(1.0, scratch.ad_value(2380)), AdValue::mul(scratch.ad_value(2303), AdValue::offset(scratch.ad_value(2392), 1.0))))));
        }

        if (((scratch.values[2404] != 0.0) && (!(scratch.values[2411] != 0.0))) && (!(scratch.values[2412] != 0.0))) {
            scratch.store_ad(2387, &AdValue::add(AdValue::sub(scratch.ad_value(2302), scratch.ad_value(2384)), AdValue::ln(AdValue::div(scratch.ad_value(2385), scratch.ad_value(2297)))));
        }

        if (((scratch.values[2404] != 0.0) && (!(scratch.values[2411] != 0.0))) && (!(scratch.values[2412] != 0.0))) {
            scratch.store_ad(966, &AdValue::add(scratch.ad_value(2385), scratch.ad_value(2386)));
        }

        if (((scratch.values[2404] != 0.0) && (!(scratch.values[2411] != 0.0))) && (!(scratch.values[2412] != 0.0))) {
            scratch.store_ad(965, &AdValue::add(AdValue::square(scratch.ad_value(966)), AdValue::mul(scratch.ad_value(2387), AdValue::sub(AdValue::scale(AdValue::square(scratch.ad_value(2386)), 0.5), AdValue::mul(scratch.ad_value(2385), scratch.ad_value(2402))))));
        }

        if (((scratch.values[2404] != 0.0) && (!(scratch.values[2411] != 0.0))) && (!(scratch.values[2412] != 0.0))) {
            let assign48050_ad_e61814: AdValue = AdValue::add(scratch.ad_value(2384), AdValue::div(AdValue::mul(AdValue::mul(scratch.ad_value(2385), scratch.ad_value(966)), scratch.ad_value(2387)), AdValue::add(scratch.ad_value(965), AdValue::mul(AdValue::mul(AdValue::mul(AdValue::mul(AdValue::div(scratch.ad_value(966), scratch.ad_value(965)), scratch.ad_value(2387)), scratch.ad_value(2387)), scratch.ad_value(2386)), AdValue::sub(AdValue::scale(AdValue::square(scratch.ad_value(2386)), 0.3333333333333333), AdValue::mul(scratch.ad_value(2385), scratch.ad_value(2402)))))));
            scratch.store_ad(2403, &assign48050_ad_e61814);
        }

        scratch.values[2415] = if (scratch.values[2403] < 230.25850929940458) { 1.0 } else { 0.0 };

        if ((((scratch.values[2404] != 0.0) && (!(scratch.values[2411] != 0.0))) && (!(scratch.values[2412] != 0.0))) && (scratch.values[2415] != 0.0)) {
            scratch.store_ad(2389, &AdValue::exp(scratch.ad_value(2403)));
        }

        if ((((scratch.values[2404] != 0.0) && (!(scratch.values[2411] != 0.0))) && (!(scratch.values[2412] != 0.0))) && (scratch.values[2415] != 0.0)) {
            scratch.store_ad(2390, &AdValue::div_from_scalar(1.0, scratch.ad_value(2389)));
        }

        if ((((scratch.values[2404] != 0.0) && (!(scratch.values[2411] != 0.0))) && (!(scratch.values[2412] != 0.0))) && (scratch.values[2415] != 0.0)) {
            scratch.store_ad(2389, &AdValue::mul(scratch.ad_value(2303), scratch.ad_value(2389)));
        }

        scratch.values[2416] = if (scratch.values[2403] > (scratch.values[2302] - 230.25850929940458)) { 1.0 } else { 0.0 };

        if (((((scratch.values[2404] != 0.0) && (!(scratch.values[2411] != 0.0))) && (!(scratch.values[2412] != 0.0))) && (!(scratch.values[2415] != 0.0))) && (scratch.values[2416] != 0.0)) {
            scratch.store_ad(2389, &AdValue::exp(AdValue::sub(scratch.ad_value(2403), scratch.ad_value(2302))));
        }

        if (((((scratch.values[2404] != 0.0) && (!(scratch.values[2411] != 0.0))) && (!(scratch.values[2412] != 0.0))) && (!(scratch.values[2415] != 0.0))) && (scratch.values[2416] != 0.0)) {
            scratch.store_ad(2390, &AdValue::div(scratch.ad_value(2303), scratch.ad_value(2389)));
        }

        if (((((scratch.values[2404] != 0.0) && (!(scratch.values[2411] != 0.0))) && (!(scratch.values[2412] != 0.0))) && (!(scratch.values[2415] != 0.0))) && (!(scratch.values[2416] != 0.0))) {
            scratch.store_ad(2389, &AdValue::div_from_scalar(1e-100, AdValue::offset(AdValue::mul(AdValue::offset(AdValue::sub(scratch.ad_value(2302), scratch.ad_value(2403)), (-230.25850929940458)), AdValue::offset(AdValue::scale(AdValue::mul(AdValue::offset(AdValue::sub(scratch.ad_value(2302), scratch.ad_value(2403)), (-230.25850929940458)), AdValue::offset(AdValue::scale(AdValue::offset(AdValue::sub(scratch.ad_value(2302), scratch.ad_value(2403)), (-230.25850929940458)), 0.3333333333333333), 1.0)), 0.5), 1.0)), 1.0)));
        }

        if (((((scratch.values[2404] != 0.0) && (!(scratch.values[2411] != 0.0))) && (!(scratch.values[2412] != 0.0))) && (!(scratch.values[2415] != 0.0))) && (!(scratch.values[2416] != 0.0))) {
            scratch.store_ad(2390, &AdValue::div_from_scalar(1e-100, AdValue::offset(AdValue::mul(AdValue::offset(scratch.ad_value(2403), (-230.25850929940458)), AdValue::offset(AdValue::scale(AdValue::mul(AdValue::offset(scratch.ad_value(2403), (-230.25850929940458)), AdValue::offset(AdValue::scale(AdValue::offset(scratch.ad_value(2403), (-230.25850929940458)), 0.3333333333333333), 1.0)), 0.5), 1.0)), 1.0)));
        }

        if (((scratch.values[2404] != 0.0) && (!(scratch.values[2411] != 0.0))) && (!(scratch.values[2412] != 0.0))) {
            scratch.store_ad(2379, &AdValue::div_from_scalar(1.0, AdValue::offset(AdValue::square(scratch.ad_value(2403)), 2.0)));
        }

        if (((scratch.values[2404] != 0.0) && (!(scratch.values[2411] != 0.0))) && (!(scratch.values[2412] != 0.0))) {
            scratch.store_ad(2391, &AdValue::mul(AdValue::square(scratch.ad_value(2403)), scratch.ad_value(2379)));
        }

        if (((scratch.values[2404] != 0.0) && (!(scratch.values[2411] != 0.0))) && (!(scratch.values[2412] != 0.0))) {
            scratch.store_ad(2392, &AdValue::scale(AdValue::mul(AdValue::mul(scratch.ad_value(2403), scratch.ad_value(2379)), scratch.ad_value(2379)), 4.0));
        }

        if (((scratch.values[2404] != 0.0) && (!(scratch.values[2411] != 0.0))) && (!(scratch.values[2412] != 0.0))) {
            scratch.store_ad(2393, &AdValue::mul(AdValue::mul(AdValue::sub(AdValue::scale(scratch.ad_value(2379), 8.0), AdValue::scale(scratch.ad_value(2391), 12.0)), scratch.ad_value(2379)), scratch.ad_value(2379)));
        }

        if (((scratch.values[2404] != 0.0) && (!(scratch.values[2411] != 0.0))) && (!(scratch.values[2412] != 0.0))) {
            scratch.store_ad(2379, &AdValue::sub(scratch.ad_value(2294), scratch.ad_value(2403)));
        }

        if (((scratch.values[2404] != 0.0) && (!(scratch.values[2411] != 0.0))) && (!(scratch.values[2412] != 0.0))) {
            scratch.store_ad(2394, &AdValue::add(AdValue::scale(scratch.ad_value(2379), 2.0), AdValue::mul(scratch.ad_value(2297), AdValue::sub(AdValue::add(AdValue::sub_from_scalar(1.0, scratch.ad_value(2390)), scratch.ad_value(2389)), AdValue::mul(scratch.ad_value(2303), AdValue::offset(scratch.ad_value(2392), 1.0))))));
        }

        if (((scratch.values[2404] != 0.0) && (!(scratch.values[2411] != 0.0))) && (!(scratch.values[2412] != 0.0))) {
            scratch.store_ad(2395, &AdValue::sub(AdValue::square(scratch.ad_value(2379)), AdValue::mul(scratch.ad_value(2297), AdValue::sub(AdValue::add(AdValue::offset(AdValue::add(scratch.ad_value(2390), scratch.ad_value(2403)), (-1.0)), scratch.ad_value(2389)), AdValue::mul(scratch.ad_value(2303), AdValue::add(AdValue::offset(scratch.ad_value(2403), 1.0), scratch.ad_value(2391)))))));
        }

        if (((scratch.values[2404] != 0.0) && (!(scratch.values[2411] != 0.0))) && (!(scratch.values[2412] != 0.0))) {
            scratch.store_ad(2379, &AdValue::sub_from_scalar(2.0, AdValue::mul(scratch.ad_value(2297), AdValue::sub(AdValue::add(scratch.ad_value(2390), scratch.ad_value(2389)), AdValue::mul(scratch.ad_value(2303), scratch.ad_value(2393))))));
        }

        if (((scratch.values[2404] != 0.0) && (!(scratch.values[2411] != 0.0))) && (!(scratch.values[2412] != 0.0))) {
            scratch.store_ad(2379, &AdValue::sub(AdValue::square(scratch.ad_value(2394)), AdValue::scale(AdValue::mul(scratch.ad_value(2395), scratch.ad_value(2379)), 2.0)));
        }

        if (((scratch.values[2404] != 0.0) && (!(scratch.values[2411] != 0.0))) && (!(scratch.values[2412] != 0.0))) {
            scratch.store_ad(2305, &AdValue::add(scratch.ad_value(2403), AdValue::scale(AdValue::div(scratch.ad_value(2395), AdValue::add(scratch.ad_value(2394), AdValue::sqrt(scratch.ad_value(2379)))), 2.0)));
        }

        if (scratch.values[2404] != 0.0) {
            scratch.values[2339] = scratch.values[2305];
            scratch.node_derivatives[2339] = scratch.node_derivatives[2305];
            scratch.branch_derivatives[2339] = scratch.branch_derivatives[2305];
        }

        if (scratch.values[2404] != 0.0) {
            scratch.values[2347] = scratch.values[2305];
            scratch.node_derivatives[2347] = scratch.node_derivatives[2305];
            scratch.branch_derivatives[2347] = scratch.branch_derivatives[2305];
        }

        if (scratch.values[2404] != 0.0) {
            scratch.values[2340] = 0.0;
            scratch.node_derivatives[2340] = [0.0; Instance::NODE_COUNT];
            scratch.branch_derivatives[2340] = [0.0; Instance::BRANCH_COUNT];
        }

        if (scratch.values[2404] != 0.0) {
            scratch.store_ad(869, &AdValue::scale(scratch.ad_value(2011), 3.912023005));
        }

        scratch.values[2417] = if (scratch.values[2294] <= 0.0) { 1.0 } else { 0.0 };

        if ((scratch.values[2404] != 0.0) && (scratch.values[2417] != 0.0)) {
            scratch.values[2315] = 0.0;
            scratch.node_derivatives[2315] = [0.0; Instance::NODE_COUNT];
            scratch.branch_derivatives[2315] = [0.0; Instance::BRANCH_COUNT];
        }

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

    }

    pub(super) fn stamp_reactive_block_15(
        &mut self,
        ctx: &GeneratedEvalContext<'_>,
        stamper: &mut GeneratedReactiveStamper<'_>,
        scratch: &mut ReactiveScratch,
    ) {
        let _ = stamper;
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

    }
}
