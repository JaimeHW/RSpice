#![allow(dead_code, unused_assignments, unused_imports, unused_parens, unused_variables)]

use super::{AdValue, GeneratedEvalContext, GeneratedReactiveStamper, GeneratedStamper, Scratch, THERMAL_VOLTAGE_PER_K};
use super::super::state::Instance;

impl Instance {

    pub(super) fn stamp_transient_block_40(
        &mut self,
        ctx: &GeneratedEvalContext<'_>,
        stamper: &mut GeneratedStamper<'_>,
        scratch: &mut Scratch,
    ) {
        let _ = stamper;
        if ((((((scratch.values[2444] != 0.0) && (!(scratch.values[2445] != 0.0))) && (!(scratch.values[2485] != 0.0))) && (!(scratch.values[2496] != 0.0))) && (!(scratch.values[2498] != 0.0))) && (scratch.values[2499] != 0.0)) {
            let assign53440_ad_e69307: AdValue = AdValue::offset(AdValue::mul(AdValue::sub_from_scalar((-230.25850929940458), AdValue::div(AdValue::neg(scratch.ad_value(491)), scratch.ad_value(688))), AdValue::offset(AdValue::scale(AdValue::mul(AdValue::sub_from_scalar((-230.25850929940458), AdValue::div(AdValue::neg(scratch.ad_value(491)), scratch.ad_value(688))), AdValue::offset(AdValue::scale(AdValue::sub_from_scalar((-230.25850929940458), AdValue::div(AdValue::neg(scratch.ad_value(491)), scratch.ad_value(688))), 0.3333333333333333), 1.0)), 0.5), 1.0)), 1.0);
            scratch.store_ad(663, &AdValue::div_from_scalar(1e-100, assign53440_ad_e69307));
        }

        if ((((((scratch.values[2444] != 0.0) && (!(scratch.values[2445] != 0.0))) && (!(scratch.values[2485] != 0.0))) && (!(scratch.values[2496] != 0.0))) && (!(scratch.values[2498] != 0.0))) && (!(scratch.values[2499] != 0.0))) {
            let assign53450_ad_e69358: AdValue = AdValue::scale(AdValue::offset(AdValue::mul(AdValue::offset(AdValue::div(AdValue::neg(scratch.ad_value(491)), scratch.ad_value(688)), (-230.25850929940458)), AdValue::offset(AdValue::scale(AdValue::mul(AdValue::offset(AdValue::div(AdValue::neg(scratch.ad_value(491)), scratch.ad_value(688)), (-230.25850929940458)), AdValue::offset(AdValue::scale(AdValue::offset(AdValue::div(AdValue::neg(scratch.ad_value(491)), scratch.ad_value(688)), (-230.25850929940458)), 0.3333333333333333), 1.0)), 0.5), 1.0)), 1.0), 1e100);
            scratch.store_ad(663, &assign53450_ad_e69358);
        }

        if ((((scratch.values[2444] != 0.0) && (!(scratch.values[2445] != 0.0))) && (!(scratch.values[2485] != 0.0))) && (!(scratch.values[2496] != 0.0))) {
            scratch.store_ad(687, &AdValue::scale(AdValue::mul(AdValue::mul(AdValue::mul(scratch.ad_value(858), scratch.ad_value(688)), scratch.ad_value(688)), scratch.ad_value(663)), scratch.values[396]));
        }

        scratch.values[2500] = if (scratch.values[405] > 1000.0) { 1.0 } else { 0.0 };

        if ((((scratch.values[2444] != 0.0) && (!(scratch.values[2445] != 0.0))) && (!(scratch.values[2485] != 0.0))) && (scratch.values[2500] != 0.0)) {
            scratch.values[689] = 1.0;
            scratch.node_derivatives[689] = [0.0; Instance::NODE_COUNT];
            scratch.branch_derivatives[689] = [0.0; Instance::BRANCH_COUNT];
        }

        scratch.values[2501] = if (scratch.values[662] > ((-scratch.values[493]) * scratch.values[405])) { 1.0 } else { 0.0 };

        scratch.values[2502] = if (scratch.values[408] == 4.0) { 1.0 } else { 0.0 };

        if ((((((scratch.values[2444] != 0.0) && (!(scratch.values[2445] != 0.0))) && (!(scratch.values[2485] != 0.0))) && (!(scratch.values[2500] != 0.0))) && (scratch.values[2501] != 0.0)) && (scratch.values[2502] != 0.0)) {
            scratch.store_ad(663, &AdValue::mul(AdValue::mul(AdValue::mul(AdValue::scale(scratch.ad_value(662), scratch.values[498]), AdValue::scale(scratch.ad_value(662), scratch.values[498])), AdValue::scale(scratch.ad_value(662), scratch.values[498])), AdValue::scale(scratch.ad_value(662), scratch.values[498])));
        }

        if ((((((scratch.values[2444] != 0.0) && (!(scratch.values[2445] != 0.0))) && (!(scratch.values[2485] != 0.0))) && (!(scratch.values[2500] != 0.0))) && (scratch.values[2501] != 0.0)) && (!(scratch.values[2502] != 0.0))) {
            scratch.store_ad(663, &AdValue::powf(AdValue::abs(AdValue::scale(scratch.ad_value(662), scratch.values[498])), scratch.values[408]));
        }

        if (((((scratch.values[2444] != 0.0) && (!(scratch.values[2445] != 0.0))) && (!(scratch.values[2485] != 0.0))) && (!(scratch.values[2500] != 0.0))) && (scratch.values[2501] != 0.0)) {
            scratch.store_ad(689, &AdValue::div_from_scalar(1.0, AdValue::sub_from_scalar(1.0, scratch.ad_value(663))));
        }

        if (((((scratch.values[2444] != 0.0) && (!(scratch.values[2445] != 0.0))) && (!(scratch.values[2485] != 0.0))) && (!(scratch.values[2500] != 0.0))) && (!(scratch.values[2501] != 0.0))) {
            scratch.store_ad(689, &AdValue::offset(AdValue::scale(AdValue::offset(scratch.ad_value(662), (scratch.values[493] * scratch.values[405])), scratch.values[501]), scratch.values[495]));
        }

        if (((scratch.values[2444] != 0.0) && (!(scratch.values[2445] != 0.0))) && (!(scratch.values[2485] != 0.0))) {
            scratch.store_ad(2035, &AdValue::mul(AdValue::add(AdValue::add(AdValue::add(scratch.ad_value(664), scratch.ad_value(665)), scratch.ad_value(672)), scratch.ad_value(687)), scratch.ad_value(689)));
        }

        scratch.values[2503] = if (scratch.values[692] == 0.0) { 1.0 } else { 0.0 };

        if (((scratch.values[2444] != 0.0) && (!(scratch.values[2445] != 0.0))) && (scratch.values[2503] != 0.0)) {
            scratch.values[2036] = 0.0;
            scratch.node_derivatives[2036] = [0.0; Instance::NODE_COUNT];
            scratch.branch_derivatives[2036] = [0.0; Instance::BRANCH_COUNT];
        }

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

        if (((scratch.values[2444] != 0.0) && (!(scratch.values[2445] != 0.0))) && (!(scratch.values[2503] != 0.0))) {
            scratch.store_ad(664, &AdValue::scale(scratch.ad_value(654), scratch.values[438]));
        }

        scratch.values[2505] = if ((scratch.values[388] == 0.0) && (scratch.values[391] == 0.0)) { 1.0 } else { 0.0 };

        if ((((scratch.values[2444] != 0.0) && (!(scratch.values[2445] != 0.0))) && (!(scratch.values[2503] != 0.0))) && (scratch.values[2505] != 0.0)) {
            scratch.values[665] = 0.0;
            scratch.node_derivatives[665] = [0.0; Instance::NODE_COUNT];
            scratch.branch_derivatives[665] = [0.0; Instance::BRANCH_COUNT];
        }

        if ((((scratch.values[2444] != 0.0) && (!(scratch.values[2445] != 0.0))) && (!(scratch.values[2503] != 0.0))) && (!(scratch.values[2505] != 0.0))) {
            scratch.store_ad(666, &AdValue::sub_from_scalar(scratch.values[444], scratch.ad_value(660)));
        }

        if ((((scratch.values[2444] != 0.0) && (!(scratch.values[2445] != 0.0))) && (!(scratch.values[2503] != 0.0))) && (!(scratch.values[2505] != 0.0))) {
            scratch.store_ad(667, &AdValue::sub_from_scalar(1.0, AdValue::sqrt(AdValue::sub_from_scalar(1.0, AdValue::div(scratch.ad_value(658), scratch.ad_value(666))))));
        }

        scratch.values[2506] = if (scratch.values[377] == 0.5) { 1.0 } else { 0.0 };

        if (((((scratch.values[2444] != 0.0) && (!(scratch.values[2445] != 0.0))) && (!(scratch.values[2503] != 0.0))) && (!(scratch.values[2505] != 0.0))) && (scratch.values[2506] != 0.0)) {
            scratch.values[668] = 0.0;
            scratch.node_derivatives[668] = [0.0; Instance::NODE_COUNT];
            scratch.branch_derivatives[668] = [0.0; Instance::BRANCH_COUNT];
        }

        if (((((scratch.values[2444] != 0.0) && (!(scratch.values[2445] != 0.0))) && (!(scratch.values[2503] != 0.0))) && (!(scratch.values[2505] != 0.0))) && (!(scratch.values[2506] != 0.0))) {
            scratch.store_ad(668, &AdValue::scale(AdValue::add(AdValue::div(AdValue::mul(AdValue::square(scratch.ad_value(667)), AdValue::ln(scratch.ad_value(667))), AdValue::sub_from_scalar(1.0, scratch.ad_value(667))), scratch.ad_value(667)), (1.0 - (2.0 * scratch.values[377]))));
        }

        if ((((scratch.values[2444] != 0.0) && (!(scratch.values[2445] != 0.0))) && (!(scratch.values[2503] != 0.0))) && (!(scratch.values[2505] != 0.0))) {
            scratch.store_ad(669, &AdValue::add(scratch.ad_value(667), scratch.ad_value(668)));
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

        if ((((scratch.values[2444] != 0.0) && (!(scratch.values[2445] != 0.0))) && (!(scratch.values[2503] != 0.0))) && (!(scratch.values[2505] != 0.0))) {
            scratch.store_ad(671, &AdValue::scale(AdValue::mul(AdValue::offset(scratch.ad_value(657), (-1.0)), scratch.ad_value(670)), scratch.values[435]));
        }

        if ((((scratch.values[2444] != 0.0) && (!(scratch.values[2445] != 0.0))) && (!(scratch.values[2503] != 0.0))) && (!(scratch.values[2505] != 0.0))) {
            scratch.store_ad(665, &AdValue::scale(AdValue::mul(scratch.ad_value(671), scratch.ad_value(669)), scratch.values[388]));
        }

        scratch.values[2508] = if (scratch.values[391] == 0.0) { 1.0 } else { 0.0 };

        if ((((scratch.values[2444] != 0.0) && (!(scratch.values[2445] != 0.0))) && (!(scratch.values[2503] != 0.0))) && (scratch.values[2508] != 0.0)) {
            scratch.values[672] = 0.0;
            scratch.node_derivatives[672] = [0.0; Instance::NODE_COUNT];
            scratch.branch_derivatives[672] = [0.0; Instance::BRANCH_COUNT];
        }

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

        scratch.values[2509] = if (((-scratch.values[377]) * scratch.values[462]) == (-1.0)) { 1.0 } else { 0.0 };

        if (((((scratch.values[2444] != 0.0) && (!(scratch.values[2445] != 0.0))) && (!(scratch.values[2503] != 0.0))) && (!(scratch.values[2508] != 0.0))) && (scratch.values[2509] != 0.0)) {
            scratch.store_ad(679, &AdValue::div_from_scalar(1.0, AdValue::offset(AdValue::mul(scratch.ad_value(673), scratch.ad_value(678)), 1.0)));
        }

        if (((((scratch.values[2444] != 0.0) && (!(scratch.values[2445] != 0.0))) && (!(scratch.values[2503] != 0.0))) && (!(scratch.values[2508] != 0.0))) && (!(scratch.values[2509] != 0.0))) {
            scratch.store_ad(679, &AdValue::powf(AdValue::offset(AdValue::mul(scratch.ad_value(673), scratch.ad_value(678)), 1.0), ((-scratch.values[377]) * scratch.values[462])));
        }

        if ((((scratch.values[2444] != 0.0) && (!(scratch.values[2445] != 0.0))) && (!(scratch.values[2503] != 0.0))) && (!(scratch.values[2508] != 0.0))) {
            scratch.store_ad(680, &AdValue::div(AdValue::mul(scratch.ad_value(669), scratch.ad_value(679)), AdValue::add(scratch.ad_value(669), scratch.ad_value(679))));
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

        scratch.values[2510] = if (scratch.values[684] > 0.0) { 1.0 } else { 0.0 };

        if (((((scratch.values[2444] != 0.0) && (!(scratch.values[2445] != 0.0))) && (!(scratch.values[2503] != 0.0))) && (!(scratch.values[2508] != 0.0))) && (scratch.values[2510] != 0.0)) {
            scratch.store_ad(646, &AdValue::div_from_scalar(1.0, AdValue::offset(AdValue::scale(scratch.ad_value(684), scratch.values[421]), 1.0)));
        }

        if (((((scratch.values[2444] != 0.0) && (!(scratch.values[2445] != 0.0))) && (!(scratch.values[2503] != 0.0))) && (!(scratch.values[2508] != 0.0))) && (!(scratch.values[2510] != 0.0))) {
            scratch.store_ad(646, &AdValue::div_from_scalar(1.0, AdValue::sub_from_scalar(1.0, AdValue::scale(scratch.ad_value(684), scratch.values[421]))));
        }

        scratch.values[2511] = if (((-scratch.values[645]) + scratch.values[683]) > (-230.25850929940458)) { 1.0 } else { 0.0 };

        if (((((scratch.values[2444] != 0.0) && (!(scratch.values[2445] != 0.0))) && (!(scratch.values[2503] != 0.0))) && (!(scratch.values[2508] != 0.0))) && (scratch.values[2511] != 0.0)) {
            scratch.store_ad(663, &AdValue::exp(AdValue::sub(scratch.ad_value(683), scratch.ad_value(645))));
        }

        if (((((scratch.values[2444] != 0.0) && (!(scratch.values[2445] != 0.0))) && (!(scratch.values[2503] != 0.0))) && (!(scratch.values[2508] != 0.0))) && (!(scratch.values[2511] != 0.0))) {
            scratch.store_ad(663, &AdValue::div_from_scalar(1e-100, AdValue::offset(AdValue::mul(AdValue::sub_from_scalar((-230.25850929940458), AdValue::sub(scratch.ad_value(683), scratch.ad_value(645))), AdValue::offset(AdValue::scale(AdValue::mul(AdValue::sub_from_scalar((-230.25850929940458), AdValue::sub(scratch.ad_value(683), scratch.ad_value(645))), AdValue::offset(AdValue::scale(AdValue::sub_from_scalar((-230.25850929940458), AdValue::sub(scratch.ad_value(683), scratch.ad_value(645))), 0.3333333333333333), 1.0)), 0.5), 1.0)), 1.0)));
        }

        if ((((scratch.values[2444] != 0.0) && (!(scratch.values[2445] != 0.0))) && (!(scratch.values[2503] != 0.0))) && (!(scratch.values[2508] != 0.0))) {
            scratch.store_ad(647, &AdValue::mul(AdValue::add(AdValue::add(AdValue::scale(scratch.ad_value(646), 0.29214664), AdValue::scale(AdValue::square(scratch.ad_value(646)), scratch.values[422])), AdValue::scale(AdValue::mul(AdValue::square(scratch.ad_value(646)), scratch.ad_value(646)), scratch.values[423])), scratch.ad_value(663)));
        }

        scratch.values[2512] = if (scratch.values[684] > 0.0) { 1.0 } else { 0.0 };

        if (((((scratch.values[2444] != 0.0) && (!(scratch.values[2445] != 0.0))) && (!(scratch.values[2503] != 0.0))) && (!(scratch.values[2508] != 0.0))) && (scratch.values[2512] != 0.0)) {
            scratch.values[685] = scratch.values[647];
            scratch.node_derivatives[685] = scratch.node_derivatives[647];
            scratch.branch_derivatives[685] = scratch.branch_derivatives[647];
        }

        scratch.values[2513] = if (scratch.values[683] > (-230.25850929940458)) { 1.0 } else { 0.0 };

        if ((((((scratch.values[2444] != 0.0) && (!(scratch.values[2445] != 0.0))) && (!(scratch.values[2503] != 0.0))) && (!(scratch.values[2508] != 0.0))) && (!(scratch.values[2512] != 0.0))) && (scratch.values[2513] != 0.0)) {
            scratch.store_ad(663, &AdValue::exp(scratch.ad_value(683)));
        }

        if ((((((scratch.values[2444] != 0.0) && (!(scratch.values[2445] != 0.0))) && (!(scratch.values[2503] != 0.0))) && (!(scratch.values[2508] != 0.0))) && (!(scratch.values[2512] != 0.0))) && (!(scratch.values[2513] != 0.0))) {
            scratch.store_ad(663, &AdValue::div_from_scalar(1e-100, AdValue::offset(AdValue::mul(AdValue::sub_from_scalar((-230.25850929940458), scratch.ad_value(683)), AdValue::offset(AdValue::scale(AdValue::mul(AdValue::sub_from_scalar((-230.25850929940458), scratch.ad_value(683)), AdValue::offset(AdValue::scale(AdValue::sub_from_scalar((-230.25850929940458), scratch.ad_value(683)), 0.3333333333333333), 1.0)), 0.5), 1.0)), 1.0)));
        }

        if (((((scratch.values[2444] != 0.0) && (!(scratch.values[2445] != 0.0))) && (!(scratch.values[2503] != 0.0))) && (!(scratch.values[2508] != 0.0))) && (!(scratch.values[2512] != 0.0))) {
            scratch.store_ad(685, &AdValue::sub(AdValue::scale(scratch.ad_value(663), 2.0), scratch.ad_value(647)));
        }

        if ((((scratch.values[2444] != 0.0) && (!(scratch.values[2445] != 0.0))) && (!(scratch.values[2503] != 0.0))) && (!(scratch.values[2508] != 0.0))) {
            scratch.store_ad(686, &AdValue::scale(AdValue::div(AdValue::scale(scratch.ad_value(685), scratch.values[486]), scratch.ad_value(681)), (1.772453850905516 * 0.5)));
        }

        if ((((scratch.values[2444] != 0.0) && (!(scratch.values[2445] != 0.0))) && (!(scratch.values[2503] != 0.0))) && (!(scratch.values[2508] != 0.0))) {
            scratch.store_ad(672, &AdValue::scale(AdValue::mul(AdValue::mul(scratch.ad_value(671), scratch.ad_value(686)), scratch.ad_value(680)), scratch.values[391]));
        }

        scratch.values[2514] = if (scratch.values[397] == 0.0) { 1.0 } else { 0.0 };

        if ((((scratch.values[2444] != 0.0) && (!(scratch.values[2445] != 0.0))) && (!(scratch.values[2503] != 0.0))) && (scratch.values[2514] != 0.0)) {
            scratch.values[687] = 0.0;
            scratch.node_derivatives[687] = [0.0; Instance::NODE_COUNT];
            scratch.branch_derivatives[687] = [0.0; Instance::BRANCH_COUNT];
        }

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

        if ((((scratch.values[2444] != 0.0) && (!(scratch.values[2445] != 0.0))) && (!(scratch.values[2503] != 0.0))) && (!(scratch.values[2514] != 0.0))) {
            scratch.store_ad(687, &AdValue::scale(AdValue::mul(AdValue::mul(AdValue::mul(scratch.ad_value(858), scratch.ad_value(688)), scratch.ad_value(688)), scratch.ad_value(663)), scratch.values[397]));
        }

        scratch.values[2518] = if (scratch.values[406] > 1000.0) { 1.0 } else { 0.0 };

        if ((((scratch.values[2444] != 0.0) && (!(scratch.values[2445] != 0.0))) && (!(scratch.values[2503] != 0.0))) && (scratch.values[2518] != 0.0)) {
            scratch.values[689] = 1.0;
            scratch.node_derivatives[689] = [0.0; Instance::NODE_COUNT];
            scratch.branch_derivatives[689] = [0.0; Instance::BRANCH_COUNT];
        }

        scratch.values[2519] = if (scratch.values[662] > ((-scratch.values[493]) * scratch.values[406])) { 1.0 } else { 0.0 };

        scratch.values[2520] = if (scratch.values[409] == 4.0) { 1.0 } else { 0.0 };

        if ((((((scratch.values[2444] != 0.0) && (!(scratch.values[2445] != 0.0))) && (!(scratch.values[2503] != 0.0))) && (!(scratch.values[2518] != 0.0))) && (scratch.values[2519] != 0.0)) && (scratch.values[2520] != 0.0)) {
            scratch.store_ad(663, &AdValue::mul(AdValue::mul(AdValue::mul(AdValue::scale(scratch.ad_value(662), scratch.values[499]), AdValue::scale(scratch.ad_value(662), scratch.values[499])), AdValue::scale(scratch.ad_value(662), scratch.values[499])), AdValue::scale(scratch.ad_value(662), scratch.values[499])));
        }

        if ((((((scratch.values[2444] != 0.0) && (!(scratch.values[2445] != 0.0))) && (!(scratch.values[2503] != 0.0))) && (!(scratch.values[2518] != 0.0))) && (scratch.values[2519] != 0.0)) && (!(scratch.values[2520] != 0.0))) {
            scratch.store_ad(663, &AdValue::powf(AdValue::abs(AdValue::scale(scratch.ad_value(662), scratch.values[499])), scratch.values[409]));
        }

        if (((((scratch.values[2444] != 0.0) && (!(scratch.values[2445] != 0.0))) && (!(scratch.values[2503] != 0.0))) && (!(scratch.values[2518] != 0.0))) && (scratch.values[2519] != 0.0)) {
            scratch.store_ad(689, &AdValue::div_from_scalar(1.0, AdValue::sub_from_scalar(1.0, scratch.ad_value(663))));
        }

        if (((((scratch.values[2444] != 0.0) && (!(scratch.values[2445] != 0.0))) && (!(scratch.values[2503] != 0.0))) && (!(scratch.values[2518] != 0.0))) && (!(scratch.values[2519] != 0.0))) {
            scratch.store_ad(689, &AdValue::offset(AdValue::scale(AdValue::offset(scratch.ad_value(662), (scratch.values[493] * scratch.values[406])), scratch.values[502]), scratch.values[496]));
        }

        if (((scratch.values[2444] != 0.0) && (!(scratch.values[2445] != 0.0))) && (!(scratch.values[2503] != 0.0))) {
            scratch.store_ad(2036, &AdValue::mul(AdValue::add(AdValue::add(AdValue::add(scratch.ad_value(664), scratch.ad_value(665)), scratch.ad_value(672)), scratch.ad_value(687)), scratch.ad_value(689)));
        }

        if ((scratch.values[2444] != 0.0) && (!(scratch.values[2445] != 0.0))) {
            scratch.store_ad(2033, &AdValue::add(AdValue::add(AdValue::mul(scratch.ad_value(690), scratch.ad_value(2034)), AdValue::mul(scratch.ad_value(691), scratch.ad_value(2035))), AdValue::mul(scratch.ad_value(692), scratch.ad_value(2036))));
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
            scratch.values[2038] = 0.0;
            scratch.node_derivatives[2038] = [0.0; Instance::NODE_COUNT];
            scratch.branch_derivatives[2038] = [0.0; Instance::BRANCH_COUNT];
        }

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

        if (((scratch.values[2444] != 0.0) && (!(scratch.values[2445] != 0.0))) && (!(scratch.values[2526] != 0.0))) {
            scratch.store_ad(664, &AdValue::mul(scratch.ad_value(588), scratch.ad_value(654)));
        }

        scratch.values[2528] = if ((scratch.values[553] == 0.0) && (scratch.values[556] == 0.0)) { 1.0 } else { 0.0 };

        if ((((scratch.values[2444] != 0.0) && (!(scratch.values[2445] != 0.0))) && (!(scratch.values[2526] != 0.0))) && (scratch.values[2528] != 0.0)) {
            scratch.values[665] = 0.0;
            scratch.node_derivatives[665] = [0.0; Instance::NODE_COUNT];
            scratch.branch_derivatives[665] = [0.0; Instance::BRANCH_COUNT];
        }

        if ((((scratch.values[2444] != 0.0) && (!(scratch.values[2445] != 0.0))) && (!(scratch.values[2526] != 0.0))) && (!(scratch.values[2528] != 0.0))) {
            scratch.store_ad(666, &AdValue::sub(scratch.ad_value(594), scratch.ad_value(660)));
        }

        if ((((scratch.values[2444] != 0.0) && (!(scratch.values[2445] != 0.0))) && (!(scratch.values[2526] != 0.0))) && (!(scratch.values[2528] != 0.0))) {
            scratch.store_ad(667, &AdValue::sub_from_scalar(1.0, AdValue::sqrt(AdValue::sub_from_scalar(1.0, AdValue::div(scratch.ad_value(658), scratch.ad_value(666))))));
        }

        scratch.values[2529] = if (scratch.values[542] == 0.5) { 1.0 } else { 0.0 };

        if (((((scratch.values[2444] != 0.0) && (!(scratch.values[2445] != 0.0))) && (!(scratch.values[2526] != 0.0))) && (!(scratch.values[2528] != 0.0))) && (scratch.values[2529] != 0.0)) {
            scratch.values[668] = 0.0;
            scratch.node_derivatives[668] = [0.0; Instance::NODE_COUNT];
            scratch.branch_derivatives[668] = [0.0; Instance::BRANCH_COUNT];
        }

        if (((((scratch.values[2444] != 0.0) && (!(scratch.values[2445] != 0.0))) && (!(scratch.values[2526] != 0.0))) && (!(scratch.values[2528] != 0.0))) && (!(scratch.values[2529] != 0.0))) {
            scratch.store_ad(668, &AdValue::mul(AdValue::add(AdValue::div(AdValue::mul(AdValue::square(scratch.ad_value(667)), AdValue::ln(scratch.ad_value(667))), AdValue::sub_from_scalar(1.0, scratch.ad_value(667))), scratch.ad_value(667)), AdValue::sub_from_scalar(1.0, AdValue::scale(scratch.ad_value(542), 2.0))));
        }

        if ((((scratch.values[2444] != 0.0) && (!(scratch.values[2445] != 0.0))) && (!(scratch.values[2526] != 0.0))) && (!(scratch.values[2528] != 0.0))) {
            scratch.store_ad(669, &AdValue::add(scratch.ad_value(667), scratch.ad_value(668)));
        }

        scratch.values[2530] = if (scratch.values[542] == 0.5) { 1.0 } else { 0.0 };

        if (((((scratch.values[2444] != 0.0) && (!(scratch.values[2445] != 0.0))) && (!(scratch.values[2526] != 0.0))) && (!(scratch.values[2528] != 0.0))) && (scratch.values[2530] != 0.0)) {
            scratch.store_ad(663, &AdValue::sqrt(AdValue::mul(scratch.ad_value(666), scratch.ad_value(621))));
        }

    }

    pub(super) fn stamp_transient_block_41(
        &mut self,
        ctx: &GeneratedEvalContext<'_>,
        stamper: &mut GeneratedStamper<'_>,
        scratch: &mut Scratch,
    ) {
        let _ = stamper;
        if (((((scratch.values[2444] != 0.0) && (!(scratch.values[2445] != 0.0))) && (!(scratch.values[2526] != 0.0))) && (!(scratch.values[2528] != 0.0))) && (!(scratch.values[2530] != 0.0))) {
            scratch.store_ad(663, &AdValue::pow(AdValue::mul(scratch.ad_value(666), scratch.ad_value(621)), scratch.ad_value(542)));
        }

        if ((((scratch.values[2444] != 0.0) && (!(scratch.values[2445] != 0.0))) && (!(scratch.values[2526] != 0.0))) && (!(scratch.values[2528] != 0.0))) {
            scratch.store_ad(670, &AdValue::mul(scratch.ad_value(615), scratch.ad_value(663)));
        }

        if ((((scratch.values[2444] != 0.0) && (!(scratch.values[2445] != 0.0))) && (!(scratch.values[2526] != 0.0))) && (!(scratch.values[2528] != 0.0))) {
            scratch.store_ad(671, &AdValue::mul(scratch.ad_value(585), AdValue::mul(AdValue::offset(scratch.ad_value(657), (-1.0)), scratch.ad_value(670))));
        }

        if ((((scratch.values[2444] != 0.0) && (!(scratch.values[2445] != 0.0))) && (!(scratch.values[2526] != 0.0))) && (!(scratch.values[2528] != 0.0))) {
            scratch.store_ad(665, &AdValue::mul(scratch.ad_value(553), AdValue::mul(scratch.ad_value(671), scratch.ad_value(669))));
        }

        scratch.values[2531] = if (scratch.values[556] == 0.0) { 1.0 } else { 0.0 };

        if ((((scratch.values[2444] != 0.0) && (!(scratch.values[2445] != 0.0))) && (!(scratch.values[2526] != 0.0))) && (scratch.values[2531] != 0.0)) {
            scratch.values[672] = 0.0;
            scratch.node_derivatives[672] = [0.0; Instance::NODE_COUNT];
            scratch.branch_derivatives[672] = [0.0; Instance::BRANCH_COUNT];
        }

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

        scratch.values[2532] = if (((-scratch.values[542]) * scratch.values[603]) == (-1.0)) { 1.0 } else { 0.0 };

        if (((((scratch.values[2444] != 0.0) && (!(scratch.values[2445] != 0.0))) && (!(scratch.values[2526] != 0.0))) && (!(scratch.values[2531] != 0.0))) && (scratch.values[2532] != 0.0)) {
            scratch.store_ad(679, &AdValue::div_from_scalar(1.0, AdValue::offset(AdValue::mul(scratch.ad_value(673), scratch.ad_value(678)), 1.0)));
        }

        if (((((scratch.values[2444] != 0.0) && (!(scratch.values[2445] != 0.0))) && (!(scratch.values[2526] != 0.0))) && (!(scratch.values[2531] != 0.0))) && (!(scratch.values[2532] != 0.0))) {
            scratch.store_ad(679, &AdValue::pow(AdValue::offset(AdValue::mul(scratch.ad_value(673), scratch.ad_value(678)), 1.0), AdValue::mul(AdValue::neg(scratch.ad_value(542)), scratch.ad_value(603))));
        }

        if ((((scratch.values[2444] != 0.0) && (!(scratch.values[2445] != 0.0))) && (!(scratch.values[2526] != 0.0))) && (!(scratch.values[2531] != 0.0))) {
            scratch.store_ad(680, &AdValue::div(AdValue::mul(scratch.ad_value(669), scratch.ad_value(679)), AdValue::add(scratch.ad_value(669), scratch.ad_value(679))));
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

        scratch.values[2533] = if (scratch.values[684] > 0.0) { 1.0 } else { 0.0 };

        if (((((scratch.values[2444] != 0.0) && (!(scratch.values[2445] != 0.0))) && (!(scratch.values[2526] != 0.0))) && (!(scratch.values[2531] != 0.0))) && (scratch.values[2533] != 0.0)) {
            scratch.store_ad(646, &AdValue::div_from_scalar(1.0, AdValue::offset(AdValue::scale(scratch.ad_value(684), scratch.values[421]), 1.0)));
        }

        if (((((scratch.values[2444] != 0.0) && (!(scratch.values[2445] != 0.0))) && (!(scratch.values[2526] != 0.0))) && (!(scratch.values[2531] != 0.0))) && (!(scratch.values[2533] != 0.0))) {
            scratch.store_ad(646, &AdValue::div_from_scalar(1.0, AdValue::sub_from_scalar(1.0, AdValue::scale(scratch.ad_value(684), scratch.values[421]))));
        }

        scratch.values[2534] = if (((-scratch.values[645]) + scratch.values[683]) > (-230.25850929940458)) { 1.0 } else { 0.0 };

        if (((((scratch.values[2444] != 0.0) && (!(scratch.values[2445] != 0.0))) && (!(scratch.values[2526] != 0.0))) && (!(scratch.values[2531] != 0.0))) && (scratch.values[2534] != 0.0)) {
            scratch.store_ad(663, &AdValue::exp(AdValue::sub(scratch.ad_value(683), scratch.ad_value(645))));
        }

        if (((((scratch.values[2444] != 0.0) && (!(scratch.values[2445] != 0.0))) && (!(scratch.values[2526] != 0.0))) && (!(scratch.values[2531] != 0.0))) && (!(scratch.values[2534] != 0.0))) {
            scratch.store_ad(663, &AdValue::div_from_scalar(1e-100, AdValue::offset(AdValue::mul(AdValue::sub_from_scalar((-230.25850929940458), AdValue::sub(scratch.ad_value(683), scratch.ad_value(645))), AdValue::offset(AdValue::scale(AdValue::mul(AdValue::sub_from_scalar((-230.25850929940458), AdValue::sub(scratch.ad_value(683), scratch.ad_value(645))), AdValue::offset(AdValue::scale(AdValue::sub_from_scalar((-230.25850929940458), AdValue::sub(scratch.ad_value(683), scratch.ad_value(645))), 0.3333333333333333), 1.0)), 0.5), 1.0)), 1.0)));
        }

        if ((((scratch.values[2444] != 0.0) && (!(scratch.values[2445] != 0.0))) && (!(scratch.values[2526] != 0.0))) && (!(scratch.values[2531] != 0.0))) {
            scratch.store_ad(647, &AdValue::mul(AdValue::add(AdValue::add(AdValue::scale(scratch.ad_value(646), 0.29214664), AdValue::scale(AdValue::square(scratch.ad_value(646)), scratch.values[422])), AdValue::scale(AdValue::mul(AdValue::square(scratch.ad_value(646)), scratch.ad_value(646)), scratch.values[423])), scratch.ad_value(663)));
        }

        scratch.values[2535] = if (scratch.values[684] > 0.0) { 1.0 } else { 0.0 };

        if (((((scratch.values[2444] != 0.0) && (!(scratch.values[2445] != 0.0))) && (!(scratch.values[2526] != 0.0))) && (!(scratch.values[2531] != 0.0))) && (scratch.values[2535] != 0.0)) {
            scratch.values[685] = scratch.values[647];
            scratch.node_derivatives[685] = scratch.node_derivatives[647];
            scratch.branch_derivatives[685] = scratch.branch_derivatives[647];
        }

        scratch.values[2536] = if (scratch.values[683] > (-230.25850929940458)) { 1.0 } else { 0.0 };

        if ((((((scratch.values[2444] != 0.0) && (!(scratch.values[2445] != 0.0))) && (!(scratch.values[2526] != 0.0))) && (!(scratch.values[2531] != 0.0))) && (!(scratch.values[2535] != 0.0))) && (scratch.values[2536] != 0.0)) {
            scratch.store_ad(663, &AdValue::exp(scratch.ad_value(683)));
        }

        if ((((((scratch.values[2444] != 0.0) && (!(scratch.values[2445] != 0.0))) && (!(scratch.values[2526] != 0.0))) && (!(scratch.values[2531] != 0.0))) && (!(scratch.values[2535] != 0.0))) && (!(scratch.values[2536] != 0.0))) {
            scratch.store_ad(663, &AdValue::div_from_scalar(1e-100, AdValue::offset(AdValue::mul(AdValue::sub_from_scalar((-230.25850929940458), scratch.ad_value(683)), AdValue::offset(AdValue::scale(AdValue::mul(AdValue::sub_from_scalar((-230.25850929940458), scratch.ad_value(683)), AdValue::offset(AdValue::scale(AdValue::sub_from_scalar((-230.25850929940458), scratch.ad_value(683)), 0.3333333333333333), 1.0)), 0.5), 1.0)), 1.0)));
        }

        if (((((scratch.values[2444] != 0.0) && (!(scratch.values[2445] != 0.0))) && (!(scratch.values[2526] != 0.0))) && (!(scratch.values[2531] != 0.0))) && (!(scratch.values[2535] != 0.0))) {
            scratch.store_ad(685, &AdValue::sub(AdValue::scale(scratch.ad_value(663), 2.0), scratch.ad_value(647)));
        }

        if ((((scratch.values[2444] != 0.0) && (!(scratch.values[2445] != 0.0))) && (!(scratch.values[2526] != 0.0))) && (!(scratch.values[2531] != 0.0))) {
            scratch.store_ad(686, &AdValue::scale(AdValue::div(AdValue::mul(scratch.ad_value(627), scratch.ad_value(685)), scratch.ad_value(681)), (1.772453850905516 * 0.5)));
        }

        if ((((scratch.values[2444] != 0.0) && (!(scratch.values[2445] != 0.0))) && (!(scratch.values[2526] != 0.0))) && (!(scratch.values[2531] != 0.0))) {
            scratch.store_ad(672, &AdValue::mul(scratch.ad_value(556), AdValue::mul(AdValue::mul(scratch.ad_value(671), scratch.ad_value(686)), scratch.ad_value(680))));
        }

        scratch.values[2537] = if (scratch.values[562] == 0.0) { 1.0 } else { 0.0 };

        if ((((scratch.values[2444] != 0.0) && (!(scratch.values[2445] != 0.0))) && (!(scratch.values[2526] != 0.0))) && (scratch.values[2537] != 0.0)) {
            scratch.values[687] = 0.0;
            scratch.node_derivatives[687] = [0.0; Instance::NODE_COUNT];
            scratch.branch_derivatives[687] = [0.0; Instance::BRANCH_COUNT];
        }

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

        if ((((scratch.values[2444] != 0.0) && (!(scratch.values[2445] != 0.0))) && (!(scratch.values[2526] != 0.0))) && (!(scratch.values[2537] != 0.0))) {
            scratch.store_ad(687, &AdValue::mul(scratch.ad_value(562), AdValue::mul(AdValue::mul(AdValue::mul(scratch.ad_value(859), scratch.ad_value(688)), scratch.ad_value(688)), scratch.ad_value(663))));
        }

        scratch.values[2541] = if (scratch.values[571] > 1000.0) { 1.0 } else { 0.0 };

        if ((((scratch.values[2444] != 0.0) && (!(scratch.values[2445] != 0.0))) && (!(scratch.values[2526] != 0.0))) && (scratch.values[2541] != 0.0)) {
            scratch.values[689] = 1.0;
            scratch.node_derivatives[689] = [0.0; Instance::NODE_COUNT];
            scratch.branch_derivatives[689] = [0.0; Instance::BRANCH_COUNT];
        }

        scratch.values[2542] = if (scratch.values[662] > ((-scratch.values[493]) * scratch.values[571])) { 1.0 } else { 0.0 };

        scratch.values[2543] = if (scratch.values[574] == 4.0) { 1.0 } else { 0.0 };

        if ((((((scratch.values[2444] != 0.0) && (!(scratch.values[2445] != 0.0))) && (!(scratch.values[2526] != 0.0))) && (!(scratch.values[2541] != 0.0))) && (scratch.values[2542] != 0.0)) && (scratch.values[2543] != 0.0)) {
            scratch.store_ad(663, &AdValue::mul(AdValue::mul(AdValue::mul(AdValue::mul(scratch.ad_value(662), scratch.ad_value(639)), AdValue::mul(scratch.ad_value(662), scratch.ad_value(639))), AdValue::mul(scratch.ad_value(662), scratch.ad_value(639))), AdValue::mul(scratch.ad_value(662), scratch.ad_value(639))));
        }

        if ((((((scratch.values[2444] != 0.0) && (!(scratch.values[2445] != 0.0))) && (!(scratch.values[2526] != 0.0))) && (!(scratch.values[2541] != 0.0))) && (scratch.values[2542] != 0.0)) && (!(scratch.values[2543] != 0.0))) {
            scratch.store_ad(663, &AdValue::pow(AdValue::abs(AdValue::mul(scratch.ad_value(662), scratch.ad_value(639))), scratch.ad_value(574)));
        }

        if (((((scratch.values[2444] != 0.0) && (!(scratch.values[2445] != 0.0))) && (!(scratch.values[2526] != 0.0))) && (!(scratch.values[2541] != 0.0))) && (scratch.values[2542] != 0.0)) {
            scratch.store_ad(689, &AdValue::div_from_scalar(1.0, AdValue::sub_from_scalar(1.0, scratch.ad_value(663))));
        }

        if (((((scratch.values[2444] != 0.0) && (!(scratch.values[2445] != 0.0))) && (!(scratch.values[2526] != 0.0))) && (!(scratch.values[2541] != 0.0))) && (!(scratch.values[2542] != 0.0))) {
            scratch.store_ad(689, &AdValue::add(scratch.ad_value(636), AdValue::mul(AdValue::add(scratch.ad_value(662), AdValue::scale(scratch.ad_value(571), scratch.values[493])), scratch.ad_value(642))));
        }

        if (((scratch.values[2444] != 0.0) && (!(scratch.values[2445] != 0.0))) && (!(scratch.values[2526] != 0.0))) {
            scratch.store_ad(2038, &AdValue::mul(AdValue::add(AdValue::add(AdValue::add(scratch.ad_value(664), scratch.ad_value(665)), scratch.ad_value(672)), scratch.ad_value(687)), scratch.ad_value(689)));
        }

        scratch.values[2544] = if (scratch.values[718] == 0.0) { 1.0 } else { 0.0 };

        if (((scratch.values[2444] != 0.0) && (!(scratch.values[2445] != 0.0))) && (scratch.values[2544] != 0.0)) {
            scratch.values[2039] = 0.0;
            scratch.node_derivatives[2039] = [0.0; Instance::NODE_COUNT];
            scratch.branch_derivatives[2039] = [0.0; Instance::BRANCH_COUNT];
        }

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

        if (((scratch.values[2444] != 0.0) && (!(scratch.values[2445] != 0.0))) && (!(scratch.values[2544] != 0.0))) {
            scratch.store_ad(664, &AdValue::mul(scratch.ad_value(589), scratch.ad_value(654)));
        }

        scratch.values[2546] = if ((scratch.values[554] == 0.0) && (scratch.values[557] == 0.0)) { 1.0 } else { 0.0 };

        if ((((scratch.values[2444] != 0.0) && (!(scratch.values[2445] != 0.0))) && (!(scratch.values[2544] != 0.0))) && (scratch.values[2546] != 0.0)) {
            scratch.values[665] = 0.0;
            scratch.node_derivatives[665] = [0.0; Instance::NODE_COUNT];
            scratch.branch_derivatives[665] = [0.0; Instance::BRANCH_COUNT];
        }

        if ((((scratch.values[2444] != 0.0) && (!(scratch.values[2445] != 0.0))) && (!(scratch.values[2544] != 0.0))) && (!(scratch.values[2546] != 0.0))) {
            scratch.store_ad(666, &AdValue::sub(scratch.ad_value(595), scratch.ad_value(660)));
        }

        if ((((scratch.values[2444] != 0.0) && (!(scratch.values[2445] != 0.0))) && (!(scratch.values[2544] != 0.0))) && (!(scratch.values[2546] != 0.0))) {
            scratch.store_ad(667, &AdValue::sub_from_scalar(1.0, AdValue::sqrt(AdValue::sub_from_scalar(1.0, AdValue::div(scratch.ad_value(658), scratch.ad_value(666))))));
        }

        scratch.values[2547] = if (scratch.values[543] == 0.5) { 1.0 } else { 0.0 };

        if (((((scratch.values[2444] != 0.0) && (!(scratch.values[2445] != 0.0))) && (!(scratch.values[2544] != 0.0))) && (!(scratch.values[2546] != 0.0))) && (scratch.values[2547] != 0.0)) {
            scratch.values[668] = 0.0;
            scratch.node_derivatives[668] = [0.0; Instance::NODE_COUNT];
            scratch.branch_derivatives[668] = [0.0; Instance::BRANCH_COUNT];
        }

        if (((((scratch.values[2444] != 0.0) && (!(scratch.values[2445] != 0.0))) && (!(scratch.values[2544] != 0.0))) && (!(scratch.values[2546] != 0.0))) && (!(scratch.values[2547] != 0.0))) {
            scratch.store_ad(668, &AdValue::mul(AdValue::add(AdValue::div(AdValue::mul(AdValue::square(scratch.ad_value(667)), AdValue::ln(scratch.ad_value(667))), AdValue::sub_from_scalar(1.0, scratch.ad_value(667))), scratch.ad_value(667)), AdValue::sub_from_scalar(1.0, AdValue::scale(scratch.ad_value(543), 2.0))));
        }

        if ((((scratch.values[2444] != 0.0) && (!(scratch.values[2445] != 0.0))) && (!(scratch.values[2544] != 0.0))) && (!(scratch.values[2546] != 0.0))) {
            scratch.store_ad(669, &AdValue::add(scratch.ad_value(667), scratch.ad_value(668)));
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

        if ((((scratch.values[2444] != 0.0) && (!(scratch.values[2445] != 0.0))) && (!(scratch.values[2544] != 0.0))) && (!(scratch.values[2546] != 0.0))) {
            scratch.store_ad(671, &AdValue::mul(scratch.ad_value(586), AdValue::mul(AdValue::offset(scratch.ad_value(657), (-1.0)), scratch.ad_value(670))));
        }

        if ((((scratch.values[2444] != 0.0) && (!(scratch.values[2445] != 0.0))) && (!(scratch.values[2544] != 0.0))) && (!(scratch.values[2546] != 0.0))) {
            scratch.store_ad(665, &AdValue::mul(scratch.ad_value(554), AdValue::mul(scratch.ad_value(671), scratch.ad_value(669))));
        }

        scratch.values[2549] = if (scratch.values[557] == 0.0) { 1.0 } else { 0.0 };

        if ((((scratch.values[2444] != 0.0) && (!(scratch.values[2445] != 0.0))) && (!(scratch.values[2544] != 0.0))) && (scratch.values[2549] != 0.0)) {
            scratch.values[672] = 0.0;
            scratch.node_derivatives[672] = [0.0; Instance::NODE_COUNT];
            scratch.branch_derivatives[672] = [0.0; Instance::BRANCH_COUNT];
        }

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

        scratch.values[2550] = if (((-scratch.values[543]) * scratch.values[604]) == (-1.0)) { 1.0 } else { 0.0 };

        if (((((scratch.values[2444] != 0.0) && (!(scratch.values[2445] != 0.0))) && (!(scratch.values[2544] != 0.0))) && (!(scratch.values[2549] != 0.0))) && (scratch.values[2550] != 0.0)) {
            scratch.store_ad(679, &AdValue::div_from_scalar(1.0, AdValue::offset(AdValue::mul(scratch.ad_value(673), scratch.ad_value(678)), 1.0)));
        }

        if (((((scratch.values[2444] != 0.0) && (!(scratch.values[2445] != 0.0))) && (!(scratch.values[2544] != 0.0))) && (!(scratch.values[2549] != 0.0))) && (!(scratch.values[2550] != 0.0))) {
            scratch.store_ad(679, &AdValue::pow(AdValue::offset(AdValue::mul(scratch.ad_value(673), scratch.ad_value(678)), 1.0), AdValue::mul(AdValue::neg(scratch.ad_value(543)), scratch.ad_value(604))));
        }

        if ((((scratch.values[2444] != 0.0) && (!(scratch.values[2445] != 0.0))) && (!(scratch.values[2544] != 0.0))) && (!(scratch.values[2549] != 0.0))) {
            scratch.store_ad(680, &AdValue::div(AdValue::mul(scratch.ad_value(669), scratch.ad_value(679)), AdValue::add(scratch.ad_value(669), scratch.ad_value(679))));
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

        scratch.values[2551] = if (scratch.values[684] > 0.0) { 1.0 } else { 0.0 };

        if (((((scratch.values[2444] != 0.0) && (!(scratch.values[2445] != 0.0))) && (!(scratch.values[2544] != 0.0))) && (!(scratch.values[2549] != 0.0))) && (scratch.values[2551] != 0.0)) {
            scratch.store_ad(646, &AdValue::div_from_scalar(1.0, AdValue::offset(AdValue::scale(scratch.ad_value(684), scratch.values[421]), 1.0)));
        }

        if (((((scratch.values[2444] != 0.0) && (!(scratch.values[2445] != 0.0))) && (!(scratch.values[2544] != 0.0))) && (!(scratch.values[2549] != 0.0))) && (!(scratch.values[2551] != 0.0))) {
            scratch.store_ad(646, &AdValue::div_from_scalar(1.0, AdValue::sub_from_scalar(1.0, AdValue::scale(scratch.ad_value(684), scratch.values[421]))));
        }

        scratch.values[2552] = if (((-scratch.values[645]) + scratch.values[683]) > (-230.25850929940458)) { 1.0 } else { 0.0 };

        if (((((scratch.values[2444] != 0.0) && (!(scratch.values[2445] != 0.0))) && (!(scratch.values[2544] != 0.0))) && (!(scratch.values[2549] != 0.0))) && (scratch.values[2552] != 0.0)) {
            scratch.store_ad(663, &AdValue::exp(AdValue::sub(scratch.ad_value(683), scratch.ad_value(645))));
        }

        if (((((scratch.values[2444] != 0.0) && (!(scratch.values[2445] != 0.0))) && (!(scratch.values[2544] != 0.0))) && (!(scratch.values[2549] != 0.0))) && (!(scratch.values[2552] != 0.0))) {
            scratch.store_ad(663, &AdValue::div_from_scalar(1e-100, AdValue::offset(AdValue::mul(AdValue::sub_from_scalar((-230.25850929940458), AdValue::sub(scratch.ad_value(683), scratch.ad_value(645))), AdValue::offset(AdValue::scale(AdValue::mul(AdValue::sub_from_scalar((-230.25850929940458), AdValue::sub(scratch.ad_value(683), scratch.ad_value(645))), AdValue::offset(AdValue::scale(AdValue::sub_from_scalar((-230.25850929940458), AdValue::sub(scratch.ad_value(683), scratch.ad_value(645))), 0.3333333333333333), 1.0)), 0.5), 1.0)), 1.0)));
        }

        if ((((scratch.values[2444] != 0.0) && (!(scratch.values[2445] != 0.0))) && (!(scratch.values[2544] != 0.0))) && (!(scratch.values[2549] != 0.0))) {
            scratch.store_ad(647, &AdValue::mul(AdValue::add(AdValue::add(AdValue::scale(scratch.ad_value(646), 0.29214664), AdValue::scale(AdValue::square(scratch.ad_value(646)), scratch.values[422])), AdValue::scale(AdValue::mul(AdValue::square(scratch.ad_value(646)), scratch.ad_value(646)), scratch.values[423])), scratch.ad_value(663)));
        }

        scratch.values[2553] = if (scratch.values[684] > 0.0) { 1.0 } else { 0.0 };

        if (((((scratch.values[2444] != 0.0) && (!(scratch.values[2445] != 0.0))) && (!(scratch.values[2544] != 0.0))) && (!(scratch.values[2549] != 0.0))) && (scratch.values[2553] != 0.0)) {
            scratch.values[685] = scratch.values[647];
            scratch.node_derivatives[685] = scratch.node_derivatives[647];
            scratch.branch_derivatives[685] = scratch.branch_derivatives[647];
        }

        scratch.values[2554] = if (scratch.values[683] > (-230.25850929940458)) { 1.0 } else { 0.0 };

        if ((((((scratch.values[2444] != 0.0) && (!(scratch.values[2445] != 0.0))) && (!(scratch.values[2544] != 0.0))) && (!(scratch.values[2549] != 0.0))) && (!(scratch.values[2553] != 0.0))) && (scratch.values[2554] != 0.0)) {
            scratch.store_ad(663, &AdValue::exp(scratch.ad_value(683)));
        }

        if ((((((scratch.values[2444] != 0.0) && (!(scratch.values[2445] != 0.0))) && (!(scratch.values[2544] != 0.0))) && (!(scratch.values[2549] != 0.0))) && (!(scratch.values[2553] != 0.0))) && (!(scratch.values[2554] != 0.0))) {
            scratch.store_ad(663, &AdValue::div_from_scalar(1e-100, AdValue::offset(AdValue::mul(AdValue::sub_from_scalar((-230.25850929940458), scratch.ad_value(683)), AdValue::offset(AdValue::scale(AdValue::mul(AdValue::sub_from_scalar((-230.25850929940458), scratch.ad_value(683)), AdValue::offset(AdValue::scale(AdValue::sub_from_scalar((-230.25850929940458), scratch.ad_value(683)), 0.3333333333333333), 1.0)), 0.5), 1.0)), 1.0)));
        }

        if (((((scratch.values[2444] != 0.0) && (!(scratch.values[2445] != 0.0))) && (!(scratch.values[2544] != 0.0))) && (!(scratch.values[2549] != 0.0))) && (!(scratch.values[2553] != 0.0))) {
            scratch.store_ad(685, &AdValue::sub(AdValue::scale(scratch.ad_value(663), 2.0), scratch.ad_value(647)));
        }

        if ((((scratch.values[2444] != 0.0) && (!(scratch.values[2445] != 0.0))) && (!(scratch.values[2544] != 0.0))) && (!(scratch.values[2549] != 0.0))) {
            scratch.store_ad(686, &AdValue::scale(AdValue::div(AdValue::mul(scratch.ad_value(628), scratch.ad_value(685)), scratch.ad_value(681)), (1.772453850905516 * 0.5)));
        }

        if ((((scratch.values[2444] != 0.0) && (!(scratch.values[2445] != 0.0))) && (!(scratch.values[2544] != 0.0))) && (!(scratch.values[2549] != 0.0))) {
            scratch.store_ad(672, &AdValue::mul(scratch.ad_value(557), AdValue::mul(AdValue::mul(scratch.ad_value(671), scratch.ad_value(686)), scratch.ad_value(680))));
        }

        scratch.values[2555] = if (scratch.values[563] == 0.0) { 1.0 } else { 0.0 };

        if ((((scratch.values[2444] != 0.0) && (!(scratch.values[2445] != 0.0))) && (!(scratch.values[2544] != 0.0))) && (scratch.values[2555] != 0.0)) {
            scratch.values[687] = 0.0;
            scratch.node_derivatives[687] = [0.0; Instance::NODE_COUNT];
            scratch.branch_derivatives[687] = [0.0; Instance::BRANCH_COUNT];
        }

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

        if ((((scratch.values[2444] != 0.0) && (!(scratch.values[2445] != 0.0))) && (!(scratch.values[2544] != 0.0))) && (!(scratch.values[2555] != 0.0))) {
            scratch.store_ad(687, &AdValue::mul(scratch.ad_value(563), AdValue::mul(AdValue::mul(AdValue::mul(scratch.ad_value(859), scratch.ad_value(688)), scratch.ad_value(688)), scratch.ad_value(663))));
        }

        scratch.values[2559] = if (scratch.values[572] > 1000.0) { 1.0 } else { 0.0 };

        if ((((scratch.values[2444] != 0.0) && (!(scratch.values[2445] != 0.0))) && (!(scratch.values[2544] != 0.0))) && (scratch.values[2559] != 0.0)) {
            scratch.values[689] = 1.0;
            scratch.node_derivatives[689] = [0.0; Instance::NODE_COUNT];
            scratch.branch_derivatives[689] = [0.0; Instance::BRANCH_COUNT];
        }

        scratch.values[2560] = if (scratch.values[662] > ((-scratch.values[493]) * scratch.values[572])) { 1.0 } else { 0.0 };

        scratch.values[2561] = if (scratch.values[575] == 4.0) { 1.0 } else { 0.0 };

        if ((((((scratch.values[2444] != 0.0) && (!(scratch.values[2445] != 0.0))) && (!(scratch.values[2544] != 0.0))) && (!(scratch.values[2559] != 0.0))) && (scratch.values[2560] != 0.0)) && (scratch.values[2561] != 0.0)) {
            scratch.store_ad(663, &AdValue::mul(AdValue::mul(AdValue::mul(AdValue::mul(scratch.ad_value(662), scratch.ad_value(640)), AdValue::mul(scratch.ad_value(662), scratch.ad_value(640))), AdValue::mul(scratch.ad_value(662), scratch.ad_value(640))), AdValue::mul(scratch.ad_value(662), scratch.ad_value(640))));
        }

        if ((((((scratch.values[2444] != 0.0) && (!(scratch.values[2445] != 0.0))) && (!(scratch.values[2544] != 0.0))) && (!(scratch.values[2559] != 0.0))) && (scratch.values[2560] != 0.0)) && (!(scratch.values[2561] != 0.0))) {
            scratch.store_ad(663, &AdValue::pow(AdValue::abs(AdValue::mul(scratch.ad_value(662), scratch.ad_value(640))), scratch.ad_value(575)));
        }

        if (((((scratch.values[2444] != 0.0) && (!(scratch.values[2445] != 0.0))) && (!(scratch.values[2544] != 0.0))) && (!(scratch.values[2559] != 0.0))) && (scratch.values[2560] != 0.0)) {
            scratch.store_ad(689, &AdValue::div_from_scalar(1.0, AdValue::sub_from_scalar(1.0, scratch.ad_value(663))));
        }

        if (((((scratch.values[2444] != 0.0) && (!(scratch.values[2445] != 0.0))) && (!(scratch.values[2544] != 0.0))) && (!(scratch.values[2559] != 0.0))) && (!(scratch.values[2560] != 0.0))) {
            scratch.store_ad(689, &AdValue::add(scratch.ad_value(637), AdValue::mul(AdValue::add(scratch.ad_value(662), AdValue::scale(scratch.ad_value(572), scratch.values[493])), scratch.ad_value(643))));
        }

        if (((scratch.values[2444] != 0.0) && (!(scratch.values[2445] != 0.0))) && (!(scratch.values[2544] != 0.0))) {
            scratch.store_ad(2039, &AdValue::mul(AdValue::add(AdValue::add(AdValue::add(scratch.ad_value(664), scratch.ad_value(665)), scratch.ad_value(672)), scratch.ad_value(687)), scratch.ad_value(689)));
        }

        scratch.values[2562] = if (scratch.values[719] == 0.0) { 1.0 } else { 0.0 };

        if (((scratch.values[2444] != 0.0) && (!(scratch.values[2445] != 0.0))) && (scratch.values[2562] != 0.0)) {
            scratch.values[2040] = 0.0;
            scratch.node_derivatives[2040] = [0.0; Instance::NODE_COUNT];
            scratch.branch_derivatives[2040] = [0.0; Instance::BRANCH_COUNT];
        }

        if (((scratch.values[2444] != 0.0) && (!(scratch.values[2445] != 0.0))) && (scratch.values[2562] != 0.0)) {
            scratch.values[2048] = 0.0;
            scratch.node_derivatives[2048] = [0.0; Instance::NODE_COUNT];
            scratch.branch_derivatives[2048] = [0.0; Instance::BRANCH_COUNT];
        }

        scratch.values[2563] = if (scratch.values[602] == 0.5) { 1.0 } else { 0.0 };

    }

    pub(super) fn stamp_transient_block_42(
        &mut self,
        ctx: &GeneratedEvalContext<'_>,
        stamper: &mut GeneratedStamper<'_>,
        scratch: &mut Scratch,
    ) {
        let _ = stamper;
        if ((((scratch.values[2444] != 0.0) && (!(scratch.values[2445] != 0.0))) && (!(scratch.values[2562] != 0.0))) && (scratch.values[2563] != 0.0)) {
            scratch.store_ad(663, &AdValue::sqrt(AdValue::sub_from_scalar(1.0, AdValue::mul(scratch.ad_value(655), scratch.ad_value(599)))));
        }

        if ((((scratch.values[2444] != 0.0) && (!(scratch.values[2445] != 0.0))) && (!(scratch.values[2562] != 0.0))) && (!(scratch.values[2563] != 0.0))) {
            scratch.store_ad(663, &AdValue::pow(AdValue::sub_from_scalar(1.0, AdValue::mul(scratch.ad_value(655), scratch.ad_value(599))), scratch.ad_value(602)));
        }

        if (((scratch.values[2444] != 0.0) && (!(scratch.values[2445] != 0.0))) && (!(scratch.values[2562] != 0.0))) {
            scratch.store_ad(2048, &AdValue::add(AdValue::mul(scratch.ad_value(611), AdValue::sub_from_scalar(1.0, scratch.ad_value(663))), AdValue::mul(scratch.ad_value(614), AdValue::sub(scratch.ad_value(859), scratch.ad_value(655)))));
        }

        if (((scratch.values[2444] != 0.0) && (!(scratch.values[2445] != 0.0))) && (!(scratch.values[2562] != 0.0))) {
            scratch.store_ad(664, &AdValue::mul(scratch.ad_value(590), scratch.ad_value(654)));
        }

        scratch.values[2564] = if ((scratch.values[555] == 0.0) && (scratch.values[558] == 0.0)) { 1.0 } else { 0.0 };

        if ((((scratch.values[2444] != 0.0) && (!(scratch.values[2445] != 0.0))) && (!(scratch.values[2562] != 0.0))) && (scratch.values[2564] != 0.0)) {
            scratch.values[665] = 0.0;
            scratch.node_derivatives[665] = [0.0; Instance::NODE_COUNT];
            scratch.branch_derivatives[665] = [0.0; Instance::BRANCH_COUNT];
        }

        if ((((scratch.values[2444] != 0.0) && (!(scratch.values[2445] != 0.0))) && (!(scratch.values[2562] != 0.0))) && (!(scratch.values[2564] != 0.0))) {
            scratch.store_ad(666, &AdValue::sub(scratch.ad_value(596), scratch.ad_value(660)));
        }

        if ((((scratch.values[2444] != 0.0) && (!(scratch.values[2445] != 0.0))) && (!(scratch.values[2562] != 0.0))) && (!(scratch.values[2564] != 0.0))) {
            scratch.store_ad(667, &AdValue::sub_from_scalar(1.0, AdValue::sqrt(AdValue::sub_from_scalar(1.0, AdValue::div(scratch.ad_value(658), scratch.ad_value(666))))));
        }

        scratch.values[2565] = if (scratch.values[544] == 0.5) { 1.0 } else { 0.0 };

        if (((((scratch.values[2444] != 0.0) && (!(scratch.values[2445] != 0.0))) && (!(scratch.values[2562] != 0.0))) && (!(scratch.values[2564] != 0.0))) && (scratch.values[2565] != 0.0)) {
            scratch.values[668] = 0.0;
            scratch.node_derivatives[668] = [0.0; Instance::NODE_COUNT];
            scratch.branch_derivatives[668] = [0.0; Instance::BRANCH_COUNT];
        }

        if (((((scratch.values[2444] != 0.0) && (!(scratch.values[2445] != 0.0))) && (!(scratch.values[2562] != 0.0))) && (!(scratch.values[2564] != 0.0))) && (!(scratch.values[2565] != 0.0))) {
            scratch.store_ad(668, &AdValue::mul(AdValue::add(AdValue::div(AdValue::mul(AdValue::square(scratch.ad_value(667)), AdValue::ln(scratch.ad_value(667))), AdValue::sub_from_scalar(1.0, scratch.ad_value(667))), scratch.ad_value(667)), AdValue::sub_from_scalar(1.0, AdValue::scale(scratch.ad_value(544), 2.0))));
        }

        if ((((scratch.values[2444] != 0.0) && (!(scratch.values[2445] != 0.0))) && (!(scratch.values[2562] != 0.0))) && (!(scratch.values[2564] != 0.0))) {
            scratch.store_ad(669, &AdValue::add(scratch.ad_value(667), scratch.ad_value(668)));
        }

        scratch.values[2566] = if (scratch.values[544] == 0.5) { 1.0 } else { 0.0 };

        if (((((scratch.values[2444] != 0.0) && (!(scratch.values[2445] != 0.0))) && (!(scratch.values[2562] != 0.0))) && (!(scratch.values[2564] != 0.0))) && (scratch.values[2566] != 0.0)) {
            scratch.store_ad(663, &AdValue::sqrt(AdValue::mul(scratch.ad_value(666), scratch.ad_value(623))));
        }

        if (((((scratch.values[2444] != 0.0) && (!(scratch.values[2445] != 0.0))) && (!(scratch.values[2562] != 0.0))) && (!(scratch.values[2564] != 0.0))) && (!(scratch.values[2566] != 0.0))) {
            scratch.store_ad(663, &AdValue::pow(AdValue::mul(scratch.ad_value(666), scratch.ad_value(623)), scratch.ad_value(544)));
        }

        if ((((scratch.values[2444] != 0.0) && (!(scratch.values[2445] != 0.0))) && (!(scratch.values[2562] != 0.0))) && (!(scratch.values[2564] != 0.0))) {
            scratch.store_ad(670, &AdValue::mul(scratch.ad_value(617), scratch.ad_value(663)));
        }

        if ((((scratch.values[2444] != 0.0) && (!(scratch.values[2445] != 0.0))) && (!(scratch.values[2562] != 0.0))) && (!(scratch.values[2564] != 0.0))) {
            scratch.store_ad(671, &AdValue::mul(scratch.ad_value(587), AdValue::mul(AdValue::offset(scratch.ad_value(657), (-1.0)), scratch.ad_value(670))));
        }

        if ((((scratch.values[2444] != 0.0) && (!(scratch.values[2445] != 0.0))) && (!(scratch.values[2562] != 0.0))) && (!(scratch.values[2564] != 0.0))) {
            scratch.store_ad(665, &AdValue::mul(scratch.ad_value(555), AdValue::mul(scratch.ad_value(671), scratch.ad_value(669))));
        }

        scratch.values[2567] = if (scratch.values[558] == 0.0) { 1.0 } else { 0.0 };

        if ((((scratch.values[2444] != 0.0) && (!(scratch.values[2445] != 0.0))) && (!(scratch.values[2562] != 0.0))) && (scratch.values[2567] != 0.0)) {
            scratch.values[672] = 0.0;
            scratch.node_derivatives[672] = [0.0; Instance::NODE_COUNT];
            scratch.branch_derivatives[672] = [0.0; Instance::BRANCH_COUNT];
        }

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

        scratch.values[2568] = if (((-scratch.values[544]) * scratch.values[605]) == (-1.0)) { 1.0 } else { 0.0 };

        if (((((scratch.values[2444] != 0.0) && (!(scratch.values[2445] != 0.0))) && (!(scratch.values[2562] != 0.0))) && (!(scratch.values[2567] != 0.0))) && (scratch.values[2568] != 0.0)) {
            scratch.store_ad(679, &AdValue::div_from_scalar(1.0, AdValue::offset(AdValue::mul(scratch.ad_value(673), scratch.ad_value(678)), 1.0)));
        }

        if (((((scratch.values[2444] != 0.0) && (!(scratch.values[2445] != 0.0))) && (!(scratch.values[2562] != 0.0))) && (!(scratch.values[2567] != 0.0))) && (!(scratch.values[2568] != 0.0))) {
            scratch.store_ad(679, &AdValue::pow(AdValue::offset(AdValue::mul(scratch.ad_value(673), scratch.ad_value(678)), 1.0), AdValue::mul(AdValue::neg(scratch.ad_value(544)), scratch.ad_value(605))));
        }

        if ((((scratch.values[2444] != 0.0) && (!(scratch.values[2445] != 0.0))) && (!(scratch.values[2562] != 0.0))) && (!(scratch.values[2567] != 0.0))) {
            scratch.store_ad(680, &AdValue::div(AdValue::mul(scratch.ad_value(669), scratch.ad_value(679)), AdValue::add(scratch.ad_value(669), scratch.ad_value(679))));
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

        scratch.values[2569] = if (scratch.values[684] > 0.0) { 1.0 } else { 0.0 };

        if (((((scratch.values[2444] != 0.0) && (!(scratch.values[2445] != 0.0))) && (!(scratch.values[2562] != 0.0))) && (!(scratch.values[2567] != 0.0))) && (scratch.values[2569] != 0.0)) {
            scratch.store_ad(646, &AdValue::div_from_scalar(1.0, AdValue::offset(AdValue::scale(scratch.ad_value(684), scratch.values[421]), 1.0)));
        }

        if (((((scratch.values[2444] != 0.0) && (!(scratch.values[2445] != 0.0))) && (!(scratch.values[2562] != 0.0))) && (!(scratch.values[2567] != 0.0))) && (!(scratch.values[2569] != 0.0))) {
            scratch.store_ad(646, &AdValue::div_from_scalar(1.0, AdValue::sub_from_scalar(1.0, AdValue::scale(scratch.ad_value(684), scratch.values[421]))));
        }

        scratch.values[2570] = if (((-scratch.values[645]) + scratch.values[683]) > (-230.25850929940458)) { 1.0 } else { 0.0 };

        if (((((scratch.values[2444] != 0.0) && (!(scratch.values[2445] != 0.0))) && (!(scratch.values[2562] != 0.0))) && (!(scratch.values[2567] != 0.0))) && (scratch.values[2570] != 0.0)) {
            scratch.store_ad(663, &AdValue::exp(AdValue::sub(scratch.ad_value(683), scratch.ad_value(645))));
        }

        if (((((scratch.values[2444] != 0.0) && (!(scratch.values[2445] != 0.0))) && (!(scratch.values[2562] != 0.0))) && (!(scratch.values[2567] != 0.0))) && (!(scratch.values[2570] != 0.0))) {
            scratch.store_ad(663, &AdValue::div_from_scalar(1e-100, AdValue::offset(AdValue::mul(AdValue::sub_from_scalar((-230.25850929940458), AdValue::sub(scratch.ad_value(683), scratch.ad_value(645))), AdValue::offset(AdValue::scale(AdValue::mul(AdValue::sub_from_scalar((-230.25850929940458), AdValue::sub(scratch.ad_value(683), scratch.ad_value(645))), AdValue::offset(AdValue::scale(AdValue::sub_from_scalar((-230.25850929940458), AdValue::sub(scratch.ad_value(683), scratch.ad_value(645))), 0.3333333333333333), 1.0)), 0.5), 1.0)), 1.0)));
        }

        if ((((scratch.values[2444] != 0.0) && (!(scratch.values[2445] != 0.0))) && (!(scratch.values[2562] != 0.0))) && (!(scratch.values[2567] != 0.0))) {
            scratch.store_ad(647, &AdValue::mul(AdValue::add(AdValue::add(AdValue::scale(scratch.ad_value(646), 0.29214664), AdValue::scale(AdValue::square(scratch.ad_value(646)), scratch.values[422])), AdValue::scale(AdValue::mul(AdValue::square(scratch.ad_value(646)), scratch.ad_value(646)), scratch.values[423])), scratch.ad_value(663)));
        }

        scratch.values[2571] = if (scratch.values[684] > 0.0) { 1.0 } else { 0.0 };

        if (((((scratch.values[2444] != 0.0) && (!(scratch.values[2445] != 0.0))) && (!(scratch.values[2562] != 0.0))) && (!(scratch.values[2567] != 0.0))) && (scratch.values[2571] != 0.0)) {
            scratch.values[685] = scratch.values[647];
            scratch.node_derivatives[685] = scratch.node_derivatives[647];
            scratch.branch_derivatives[685] = scratch.branch_derivatives[647];
        }

        scratch.values[2572] = if (scratch.values[683] > (-230.25850929940458)) { 1.0 } else { 0.0 };

        if ((((((scratch.values[2444] != 0.0) && (!(scratch.values[2445] != 0.0))) && (!(scratch.values[2562] != 0.0))) && (!(scratch.values[2567] != 0.0))) && (!(scratch.values[2571] != 0.0))) && (scratch.values[2572] != 0.0)) {
            scratch.store_ad(663, &AdValue::exp(scratch.ad_value(683)));
        }

        if ((((((scratch.values[2444] != 0.0) && (!(scratch.values[2445] != 0.0))) && (!(scratch.values[2562] != 0.0))) && (!(scratch.values[2567] != 0.0))) && (!(scratch.values[2571] != 0.0))) && (!(scratch.values[2572] != 0.0))) {
            scratch.store_ad(663, &AdValue::div_from_scalar(1e-100, AdValue::offset(AdValue::mul(AdValue::sub_from_scalar((-230.25850929940458), scratch.ad_value(683)), AdValue::offset(AdValue::scale(AdValue::mul(AdValue::sub_from_scalar((-230.25850929940458), scratch.ad_value(683)), AdValue::offset(AdValue::scale(AdValue::sub_from_scalar((-230.25850929940458), scratch.ad_value(683)), 0.3333333333333333), 1.0)), 0.5), 1.0)), 1.0)));
        }

        if (((((scratch.values[2444] != 0.0) && (!(scratch.values[2445] != 0.0))) && (!(scratch.values[2562] != 0.0))) && (!(scratch.values[2567] != 0.0))) && (!(scratch.values[2571] != 0.0))) {
            scratch.store_ad(685, &AdValue::sub(AdValue::scale(scratch.ad_value(663), 2.0), scratch.ad_value(647)));
        }

        if ((((scratch.values[2444] != 0.0) && (!(scratch.values[2445] != 0.0))) && (!(scratch.values[2562] != 0.0))) && (!(scratch.values[2567] != 0.0))) {
            scratch.store_ad(686, &AdValue::scale(AdValue::div(AdValue::mul(scratch.ad_value(629), scratch.ad_value(685)), scratch.ad_value(681)), (1.772453850905516 * 0.5)));
        }

        if ((((scratch.values[2444] != 0.0) && (!(scratch.values[2445] != 0.0))) && (!(scratch.values[2562] != 0.0))) && (!(scratch.values[2567] != 0.0))) {
            scratch.store_ad(672, &AdValue::mul(scratch.ad_value(558), AdValue::mul(AdValue::mul(scratch.ad_value(671), scratch.ad_value(686)), scratch.ad_value(680))));
        }

        scratch.values[2573] = if (scratch.values[564] == 0.0) { 1.0 } else { 0.0 };

        if ((((scratch.values[2444] != 0.0) && (!(scratch.values[2445] != 0.0))) && (!(scratch.values[2562] != 0.0))) && (scratch.values[2573] != 0.0)) {
            scratch.values[687] = 0.0;
            scratch.node_derivatives[687] = [0.0; Instance::NODE_COUNT];
            scratch.branch_derivatives[687] = [0.0; Instance::BRANCH_COUNT];
        }

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

        if ((((scratch.values[2444] != 0.0) && (!(scratch.values[2445] != 0.0))) && (!(scratch.values[2562] != 0.0))) && (!(scratch.values[2573] != 0.0))) {
            scratch.store_ad(687, &AdValue::mul(scratch.ad_value(564), AdValue::mul(AdValue::mul(AdValue::mul(scratch.ad_value(859), scratch.ad_value(688)), scratch.ad_value(688)), scratch.ad_value(663))));
        }

        scratch.values[2577] = if (scratch.values[573] > 1000.0) { 1.0 } else { 0.0 };

        if ((((scratch.values[2444] != 0.0) && (!(scratch.values[2445] != 0.0))) && (!(scratch.values[2562] != 0.0))) && (scratch.values[2577] != 0.0)) {
            scratch.values[689] = 1.0;
            scratch.node_derivatives[689] = [0.0; Instance::NODE_COUNT];
            scratch.branch_derivatives[689] = [0.0; Instance::BRANCH_COUNT];
        }

        scratch.values[2578] = if (scratch.values[662] > ((-scratch.values[493]) * scratch.values[573])) { 1.0 } else { 0.0 };

        scratch.values[2579] = if (scratch.values[576] == 4.0) { 1.0 } else { 0.0 };

        if ((((((scratch.values[2444] != 0.0) && (!(scratch.values[2445] != 0.0))) && (!(scratch.values[2562] != 0.0))) && (!(scratch.values[2577] != 0.0))) && (scratch.values[2578] != 0.0)) && (scratch.values[2579] != 0.0)) {
            scratch.store_ad(663, &AdValue::mul(AdValue::mul(AdValue::mul(AdValue::mul(scratch.ad_value(662), scratch.ad_value(641)), AdValue::mul(scratch.ad_value(662), scratch.ad_value(641))), AdValue::mul(scratch.ad_value(662), scratch.ad_value(641))), AdValue::mul(scratch.ad_value(662), scratch.ad_value(641))));
        }

        if ((((((scratch.values[2444] != 0.0) && (!(scratch.values[2445] != 0.0))) && (!(scratch.values[2562] != 0.0))) && (!(scratch.values[2577] != 0.0))) && (scratch.values[2578] != 0.0)) && (!(scratch.values[2579] != 0.0))) {
            scratch.store_ad(663, &AdValue::pow(AdValue::abs(AdValue::mul(scratch.ad_value(662), scratch.ad_value(641))), scratch.ad_value(576)));
        }

        if (((((scratch.values[2444] != 0.0) && (!(scratch.values[2445] != 0.0))) && (!(scratch.values[2562] != 0.0))) && (!(scratch.values[2577] != 0.0))) && (scratch.values[2578] != 0.0)) {
            scratch.store_ad(689, &AdValue::div_from_scalar(1.0, AdValue::sub_from_scalar(1.0, scratch.ad_value(663))));
        }

        if (((((scratch.values[2444] != 0.0) && (!(scratch.values[2445] != 0.0))) && (!(scratch.values[2562] != 0.0))) && (!(scratch.values[2577] != 0.0))) && (!(scratch.values[2578] != 0.0))) {
            scratch.store_ad(689, &AdValue::add(scratch.ad_value(638), AdValue::mul(AdValue::add(scratch.ad_value(662), AdValue::scale(scratch.ad_value(573), scratch.values[493])), scratch.ad_value(644))));
        }

        if (((scratch.values[2444] != 0.0) && (!(scratch.values[2445] != 0.0))) && (!(scratch.values[2562] != 0.0))) {
            scratch.store_ad(2040, &AdValue::mul(AdValue::add(AdValue::add(AdValue::add(scratch.ad_value(664), scratch.ad_value(665)), scratch.ad_value(672)), scratch.ad_value(687)), scratch.ad_value(689)));
        }

        if ((scratch.values[2444] != 0.0) && (!(scratch.values[2445] != 0.0))) {
            scratch.store_ad(2037, &AdValue::add(AdValue::add(AdValue::mul(scratch.ad_value(717), scratch.ad_value(2038)), AdValue::mul(scratch.ad_value(718), scratch.ad_value(2039))), AdValue::mul(scratch.ad_value(719), scratch.ad_value(2040))));
        }

        scratch.store_ad(953, &AdValue::scale(scratch.ad_value(846), scratch.values[762]));

        scratch.store_ad(954, &AdValue::scale(scratch.ad_value(847), scratch.values[762]));

        scratch.store_ad(955, &AdValue::scale(scratch.ad_value(848), scratch.values[762]));

        scratch.store_ad(956, &AdValue::scale(scratch.ad_value(849), scratch.values[762]));

        scratch.store_ad(959, &AdValue::scale(scratch.ad_value(850), scratch.values[762]));

        scratch.store_ad(958, &AdValue::scale(scratch.ad_value(851), scratch.values[762]));

        scratch.store_ad(957, &AdValue::scale(scratch.ad_value(852), scratch.values[762]));

        scratch.values[2580] = if (scratch.values[2002] > 0.0) { 1.0 } else { 0.0 };

        scratch.values[2581] = if (scratch.values[296] > 0.0) { 1.0 } else { 0.0 };

        scratch.values[2582] = if (scratch.values[297] > 0.0) { 1.0 } else { 0.0 };

        scratch.values[2583] = if (scratch.values[298] > 0.0) { 1.0 } else { 0.0 };

        scratch.values[2584] = if (scratch.values[299] > 0.0) { 1.0 } else { 0.0 };

        scratch.values[2585] = if (scratch.values[300] > 0.0) { 1.0 } else { 0.0 };

        scratch.values[2586] = if (scratch.values[301] > 0.0) { 1.0 } else { 0.0 };

        scratch.values[2587] = if (scratch.values[302] > 0.0) { 1.0 } else { 0.0 };

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

        scratch.values[985] = 0.0;

        scratch.values[1001] = 0.0;

        scratch.values[993] = 0.0;

        scratch.values[1003] = 1e-40;

        scratch.values[1005] = 0.0;

        scratch.values[1008] = 0.0;

        scratch.store_ad(1006, &AdValue::mul(scratch.ad_value(2032), scratch.ad_value(2028)));

        scratch.values[1002] = 0.0;

        scratch.values[1007] = 0.0;

        scratch.values[2073] = 0.0;

        scratch.values[2074] = 0.0;

        scratch.values[2075] = 0.0;

        scratch.values[2590] = if (((scratch.values[2013] > 0.0) && (scratch.values[25] > 0.0)) && (scratch.values[760] > 0.0)) { 1.0 } else { 0.0 };

        if (scratch.values[2590] != 0.0) {
            scratch.store_ad(982, &AdValue::scale(AdValue::mul(scratch.ad_value(814), scratch.ad_value(2014)), scratch.values[759]));
        }

        if (scratch.values[2590] != 0.0) {
            scratch.store_ad(983, &AdValue::mul(scratch.ad_value(814), scratch.ad_value(2017)));
        }

        if (scratch.values[2590] != 0.0) {
            scratch.store_ad(984, &AdValue::mul(AdValue::mul(scratch.ad_value(814), scratch.ad_value(2014)), scratch.ad_value(2015)));
        }

        if (scratch.values[2590] != 0.0) {
            scratch.store_ad(985, &AdValue::mul(AdValue::add(AdValue::sub(scratch.ad_value(274), AdValue::mul(scratch.ad_value(275), scratch.ad_value(982))), AdValue::mul(scratch.ad_value(276), AdValue::square(scratch.ad_value(982)))), AdValue::ln(AdValue::div(AdValue::add(scratch.ad_value(983), AdValue::scale(scratch.ad_value(984), 0.5)), AdValue::sub(scratch.ad_value(983), AdValue::scale(scratch.ad_value(984), 0.5))))));
        }

        if (scratch.values[2590] != 0.0) {
            scratch.store_ad(985, &AdValue::add(scratch.ad_value(985), AdValue::mul(AdValue::add(scratch.ad_value(275), AdValue::mul(scratch.ad_value(276), AdValue::sub(scratch.ad_value(983), AdValue::scale(scratch.ad_value(982), 2.0)))), scratch.ad_value(984))));
        }

        if (scratch.values[2590] != 0.0) {
            scratch.store_ad(985, &AdValue::div(AdValue::mul(AdValue::mul(AdValue::mul(scratch.ad_value(765), scratch.ad_value(2021)), scratch.ad_value(2020)), scratch.ad_value(985)), scratch.ad_value(982)));
        }

        if (scratch.values[2590] != 0.0) {
            scratch.store_ad(985, &{
                if (scratch.values[985] > 0.0) {
                    scratch.ad_value(985)
                } else {
                    AdValue::constant(0.0)
                }
            });
        }

        if (scratch.values[2590] != 0.0) {
            scratch.store_ad(2031, &AdValue::div(scratch.ad_value(2017), scratch.ad_value(2014)));
        }

        if (scratch.values[2590] != 0.0) {
            scratch.store_ad(986, &AdValue::div(scratch.ad_value(2016), scratch.ad_value(2017)));
        }

        if (scratch.values[2590] != 0.0) {
            scratch.store_ad(987, &AdValue::scale(AdValue::div(scratch.ad_value(2015), scratch.ad_value(2031)), (0.5 * 0.16666666666666666)));
        }

        if (scratch.values[2590] != 0.0) {
            scratch.store_ad(988, &AdValue::square(scratch.ad_value(987)));
        }

        if (scratch.values[2590] != 0.0) {
            scratch.store_ad(989, &AdValue::offset(AdValue::div(scratch.ad_value(2031), scratch.ad_value(2018)), (-1.0)));
        }

        if (scratch.values[2590] != 0.0) {
            scratch.store_ad(990, &{
                if ((1.0 - (12.0 * (scratch.values[989] * scratch.values[988]))) > 1e-20) {
                    AdValue::sub_from_scalar(1.0, AdValue::scale(AdValue::mul(scratch.ad_value(989), scratch.ad_value(988)), 12.0))
                } else {
                    AdValue::constant(1e-20)
                }
            });
        }

        if (scratch.values[2590] != 0.0) {
            scratch.store_ad(991, &AdValue::div_from_scalar(1.0, AdValue::square(scratch.ad_value(990))));
        }

        if (scratch.values[2590] != 0.0) {
            scratch.store_ad(992, &AdValue::mul(scratch.ad_value(760), AdValue::mul(AdValue::mul(scratch.ad_value(2019), scratch.ad_value(2017)), scratch.ad_value(2020))));
        }

        if (scratch.values[2590] != 0.0) {
            scratch.store_ad(993, &AdValue::sub(AdValue::add(scratch.ad_value(986), AdValue::scale(scratch.ad_value(988), 12.0)), AdValue::scale(AdValue::mul(AdValue::mul(AdValue::offset(scratch.ad_value(986), 1.0), scratch.ad_value(988)), scratch.ad_value(989)), 24.0)));
        }

        if (scratch.values[2590] != 0.0) {
            scratch.store_ad(993, &{
                if (scratch.values[993] > 1e-40) {
                    scratch.ad_value(993)
                } else {
                    AdValue::constant(1e-40)
                }
            });
        }

        if (scratch.values[2590] != 0.0) {
            scratch.store_ad(993, &AdValue::mul(AdValue::mul(scratch.ad_value(992), scratch.ad_value(991)), scratch.ad_value(993)));
        }

        scratch.values[2591] = if (scratch.values[273] > 0.0) { 1.0 } else { 0.0 };

        if ((scratch.values[2590] != 0.0) && (scratch.values[2591] != 0.0)) {
            scratch.store_ad(994, &AdValue::mul(scratch.ad_value(2016), scratch.ad_value(2052)));
        }

        if ((scratch.values[2590] != 0.0) && (scratch.values[2591] != 0.0)) {
            scratch.store_ad(995, &AdValue::scale(AdValue::div(scratch.ad_value(994), AdValue::offset(scratch.ad_value(994), 100.0)), 100.0));
        }

        scratch.values[2592] = if (scratch.values[239] < 0.0) { 1.0 } else { 0.0 };

        if (((scratch.values[2590] != 0.0) && (scratch.values[2591] != 0.0)) && (scratch.values[2592] != 0.0)) {
            scratch.store_ad(996, &AdValue::div_from_scalar(1.0, AdValue::sub_from_scalar(1.0, AdValue::mul(scratch.ad_value(239), scratch.ad_value(995)))));
        }

        if (((scratch.values[2590] != 0.0) && (scratch.values[2591] != 0.0)) && (!(scratch.values[2592] != 0.0))) {
            scratch.store_ad(996, &AdValue::offset(AdValue::mul(scratch.ad_value(239), scratch.ad_value(995)), 1.0));
        }

        if ((scratch.values[2590] != 0.0) && (scratch.values[2591] != 0.0)) {
            scratch.store_ad(997, &AdValue::mul(scratch.ad_value(764), AdValue::div(scratch.ad_value(996), scratch.ad_value(2051))));
        }

        if ((scratch.values[2590] != 0.0) && (scratch.values[2591] != 0.0)) {
            scratch.store_ad(998, &AdValue::mul(AdValue::mul(AdValue::square(scratch.ad_value(997)), scratch.ad_value(2015)), scratch.ad_value(2015)));
        }

        scratch.values[2593] = if (scratch.values[0] == (-1.0)) { 1.0 } else { 0.0 };

        if (((scratch.values[2590] != 0.0) && (scratch.values[2591] != 0.0)) && (scratch.values[2593] != 0.0)) {
            scratch.store_ad(998, &AdValue::div(scratch.ad_value(998), AdValue::offset(AdValue::mul(scratch.ad_value(997), scratch.ad_value(2015)), 1.0)));
        }

        if ((scratch.values[2590] != 0.0) && (scratch.values[2591] != 0.0)) {
            scratch.store_ad(999, &AdValue::scale(AdValue::mul(scratch.ad_value(2051), AdValue::offset(AdValue::sqrt(AdValue::offset(AdValue::scale(scratch.ad_value(998), 2.0), 1.0)), 1.0)), 0.5));
        }

        if ((scratch.values[2590] != 0.0) && (scratch.values[2591] != 0.0)) {
            scratch.store_ad(1000, &AdValue::div(scratch.ad_value(2051), AdValue::mul(scratch.ad_value(999), scratch.ad_value(990))));
        }

        if ((scratch.values[2590] != 0.0) && (scratch.values[2591] != 0.0)) {
            scratch.store_ad(1001, &AdValue::mul(AdValue::mul(AdValue::mul(AdValue::mul(scratch.ad_value(845), scratch.ad_value(2021)), scratch.ad_value(2053)), scratch.ad_value(1000)), scratch.ad_value(1000)));
        }

        if ((scratch.values[2590] != 0.0) && (scratch.values[2591] != 0.0)) {
            scratch.store_ad(993, &AdValue::add(scratch.ad_value(993), AdValue::scale(scratch.ad_value(1001), 1.0 / (scratch.values[762]))));
        }

        if (scratch.values[2590] != 0.0) {
            scratch.store_ad(1002, &AdValue::sqrt(AdValue::mul(scratch.ad_value(763), scratch.ad_value(993))));
        }

        scratch.values[2594] = if ((scratch.values[10] == 1.0) && (scratch.values[763] > 0.0)) { 1.0 } else { 0.0 };

        if ((scratch.values[2590] != 0.0) && (scratch.values[2594] != 0.0)) {
            scratch.store_ad(1003, &AdValue::sub(AdValue::sub(AdValue::scale(scratch.ad_value(986), 0.08333333333333333), AdValue::mul(scratch.ad_value(988), AdValue::sub(AdValue::offset(scratch.ad_value(986), 0.2), AdValue::scale(scratch.ad_value(988), 12.0)))), AdValue::scale(AdValue::mul(AdValue::mul(scratch.ad_value(988), AdValue::sub(AdValue::offset(scratch.ad_value(986), 1.0), AdValue::scale(scratch.ad_value(988), 12.0))), scratch.ad_value(989)), 1.6)));
        }

    }

    pub(super) fn stamp_transient_block_43(
        &mut self,
        ctx: &GeneratedEvalContext<'_>,
        stamper: &mut GeneratedStamper<'_>,
        scratch: &mut Scratch,
    ) {
        let _ = stamper;
        if ((scratch.values[2590] != 0.0) && (scratch.values[2594] != 0.0)) {
            scratch.store_ad(1003, &{
                if (scratch.values[1003] > 1e-40) {
                    scratch.ad_value(1003)
                } else {
                    AdValue::constant(1e-40)
                }
            });
        }

        if ((scratch.values[2590] != 0.0) && (scratch.values[2594] != 0.0)) {
            scratch.store_ad(1003, &AdValue::mul(AdValue::div(scratch.ad_value(991), scratch.ad_value(992)), scratch.ad_value(1003)));
        }

        if ((scratch.values[2590] != 0.0) && (scratch.values[2594] != 0.0)) {
            scratch.store_ad(1004, &AdValue::mul(AdValue::mul(scratch.ad_value(991), scratch.ad_value(987)), AdValue::sub(AdValue::sub_from_scalar(1.0, AdValue::scale(scratch.ad_value(988), 12.0)), AdValue::mul(AdValue::sub(AdValue::add(scratch.ad_value(986), AdValue::scale(scratch.ad_value(988), 19.2)), AdValue::scale(AdValue::mul(scratch.ad_value(986), scratch.ad_value(988)), 12.0)), scratch.ad_value(989)))));
        }

        if ((scratch.values[2590] != 0.0) && (scratch.values[2594] != 0.0)) {
            scratch.store_ad(1006, &AdValue::div(AdValue::mul(AdValue::mul(AdValue::square(scratch.ad_value(2029)), scratch.ad_value(2032)), scratch.ad_value(2028)), AdValue::square(scratch.ad_value(2030))));
        }

        scratch.values[2595] = if (scratch.values[273] > 0.0) { 1.0 } else { 0.0 };

        if (((scratch.values[2590] != 0.0) && (scratch.values[2594] != 0.0)) && (scratch.values[2595] != 0.0)) {
            scratch.store_ad(1003, &AdValue::add(scratch.ad_value(1003), AdValue::div(AdValue::mul(scratch.ad_value(1001), AdValue::offset(AdValue::scale(scratch.ad_value(988), 12.0), 1.0)), AdValue::scale(AdValue::mul(AdValue::scale(scratch.ad_value(992), 12.0), scratch.ad_value(992)), scratch.values[762]))));
        }

        if (((scratch.values[2590] != 0.0) && (scratch.values[2594] != 0.0)) && (scratch.values[2595] != 0.0)) {
            scratch.store_ad(1004, &AdValue::sub(scratch.ad_value(1004), AdValue::div(AdValue::mul(AdValue::mul(scratch.ad_value(1001), scratch.ad_value(987)), AdValue::offset(scratch.ad_value(989), 1.0)), AdValue::scale(scratch.ad_value(992), scratch.values[762]))));
        }

        if ((scratch.values[2590] != 0.0) && (scratch.values[2594] != 0.0)) {
            scratch.store_ad(1007, &AdValue::sqrt(AdValue::div(scratch.ad_value(763), scratch.ad_value(1003))));
        }

        scratch.values[2596] = if (scratch.values[1002] == 0.0) { 1.0 } else { 0.0 };

        if (((scratch.values[2590] != 0.0) && (scratch.values[2594] != 0.0)) && (scratch.values[2596] != 0.0)) {
            scratch.values[1008] = 0.0;
            scratch.node_derivatives[1008] = [0.0; Instance::NODE_COUNT];
            scratch.branch_derivatives[1008] = [0.0; Instance::BRANCH_COUNT];
        }

        if (((scratch.values[2590] != 0.0) && (scratch.values[2594] != 0.0)) && (!(scratch.values[2596] != 0.0))) {
            scratch.store_ad(1008, &AdValue::div(AdValue::mul(scratch.ad_value(1004), scratch.ad_value(1007)), scratch.ad_value(1002)));
        }

        if ((scratch.values[2590] != 0.0) && (scratch.values[2594] != 0.0)) {
            scratch.store_ad(1008, &{
                if (scratch.values[1008] > 0.0) {
                    {
                        if (scratch.values[1008] < 1.0) {
                            scratch.ad_value(1008)
                        } else {
                            AdValue::constant(1.0)
                        }
                    }
                } else {
                    AdValue::constant(0.0)
                }
            });
        }

        if ((scratch.values[2590] != 0.0) && (scratch.values[2594] != 0.0)) {
            scratch.store_ad(1005, &AdValue::div(AdValue::mul(scratch.ad_value(1008), scratch.ad_value(1002)), scratch.ad_value(1007)));
        }

        scratch.store_ad(1009, &AdValue::scale(AdValue::abs(scratch.ad_value(2027)), (2.0 * 1.6021918e-19)));

        scratch.store_ad(1010, &AdValue::scale(AdValue::abs(scratch.ad_value(2026)), (2.0 * 1.6021918e-19)));

        scratch.store_ad(1011, &AdValue::scale(AdValue::abs(scratch.ad_value(2025)), (2.0 * 1.6021918e-19)));

        scratch.store_ad(1012, &AdValue::scale(AdValue::abs(scratch.ad_value(2024)), (2.0 * 1.6021918e-19)));

        scratch.store_ad(1013, &AdValue::scale(AdValue::mul(AdValue::offset(scratch.ad_value(2023), 1.0), AdValue::abs(scratch.ad_value(2022))), (2.0 * 1.6021918e-19)));

        scratch.store_ad(1014, &AdValue::scale(AdValue::abs(scratch.ad_value(2033)), (2.0 * 1.6021918e-19)));

        scratch.store_ad(1015, &AdValue::scale(AdValue::abs(scratch.ad_value(2037)), (2.0 * 1.6021918e-19)));

        scratch.values[2597] = if (scratch.values[2002] > 0.0) { 1.0 } else { 0.0 };

        if (scratch.values[2597] != 0.0) {
            scratch.store_ad(1016, &AdValue::add(scratch.ad_value(1009), scratch.ad_value(1011)));
        }

        if (scratch.values[2597] != 0.0) {
            scratch.store_ad(1017, &AdValue::add(scratch.ad_value(1010), scratch.ad_value(1012)));
        }

        if (scratch.values[2597] != 0.0) {
            scratch.values[2049] = scratch.values[1014];
            scratch.node_derivatives[2049] = scratch.node_derivatives[1014];
            scratch.branch_derivatives[2049] = scratch.branch_derivatives[1014];
        }

        if (scratch.values[2597] != 0.0) {
            scratch.store_ad(2050, &AdValue::add(scratch.ad_value(1015), scratch.ad_value(1013)));
        }

        if (!(scratch.values[2597] != 0.0)) {
            scratch.store_ad(1016, &AdValue::add(scratch.ad_value(1010), scratch.ad_value(1011)));
        }

        if (!(scratch.values[2597] != 0.0)) {
            scratch.store_ad(1017, &AdValue::add(scratch.ad_value(1009), scratch.ad_value(1012)));
        }

        if (!(scratch.values[2597] != 0.0)) {
            scratch.store_ad(2049, &AdValue::add(scratch.ad_value(1014), scratch.ad_value(1013)));
        }

        if (!(scratch.values[2597] != 0.0)) {
            scratch.values[2050] = scratch.values[1015];
            scratch.node_derivatives[2050] = scratch.node_derivatives[1015];
            scratch.branch_derivatives[2050] = scratch.branch_derivatives[1015];
        }

        scratch.values[2598] = if (((scratch.values[8] != 0.0) && (scratch.values[283] > 0.0)) && (scratch.values[2061] > 0.0)) { 1.0 } else { 0.0 };

        if (scratch.values[2598] != 0.0) {
            scratch.store_ad(2077, &AdValue::div(AdValue::scale(scratch.ad_value(2070), 4.0), scratch.ad_value(767)));
        }

        if (scratch.values[2598] != 0.0) {
            scratch.store_ad(1018, &AdValue::div(AdValue::sqrt(AdValue::offset(scratch.ad_value(2077), 1.0)), AdValue::offset(AdValue::sqrt(AdValue::offset(scratch.ad_value(2077), 1.1)), (-1.0))));
        }

        if (scratch.values[2598] != 0.0) {
            scratch.store_ad(2077, &AdValue::scale(scratch.ad_value(814), scratch.values[759]));
        }

        if (scratch.values[2598] != 0.0) {
            scratch.store_ad(1019, &AdValue::mul(scratch.ad_value(2077), scratch.ad_value(1018)));
        }

        if (scratch.values[2598] != 0.0) {
            scratch.store_ad(1020, &AdValue::mul(scratch.ad_value(2077), AdValue::add(scratch.ad_value(2069), scratch.ad_value(1018))));
        }

        if (scratch.values[2598] != 0.0) {
            scratch.store_ad(1021, &AdValue::mul(AdValue::mul(AdValue::mul(AdValue::neg(scratch.ad_value(2077)), scratch.ad_value(1018)), scratch.ad_value(2071)), scratch.ad_value(2067)));
        }

        if (scratch.values[2598] != 0.0) {
            scratch.store_ad(2073, &AdValue::mul(AdValue::sub(scratch.ad_value(292), AdValue::mul(AdValue::sub(scratch.ad_value(293), AdValue::mul(scratch.ad_value(294), scratch.ad_value(1019))), scratch.ad_value(1019))), AdValue::ln(AdValue::div(AdValue::add(scratch.ad_value(1020), AdValue::scale(scratch.ad_value(1021), 0.5)), AdValue::sub(scratch.ad_value(1020), AdValue::scale(scratch.ad_value(1021), 0.5))))));
        }

        if (scratch.values[2598] != 0.0) {
            scratch.store_ad(2073, &AdValue::add(scratch.ad_value(2073), AdValue::mul(AdValue::add(scratch.ad_value(293), AdValue::mul(scratch.ad_value(294), AdValue::sub(scratch.ad_value(1020), AdValue::scale(scratch.ad_value(1019), 2.0)))), scratch.ad_value(1021))));
        }

        if (scratch.values[2598] != 0.0) {
            scratch.store_ad(2073, &AdValue::div(AdValue::mul(AdValue::mul(AdValue::mul(scratch.ad_value(769), scratch.ad_value(2072)), scratch.ad_value(2020)), scratch.ad_value(2073)), scratch.ad_value(1019)));
        }

        if (scratch.values[2598] != 0.0) {
            scratch.store_ad(2073, &{
                if (scratch.values[2073] > 0.0) {
                    scratch.ad_value(2073)
                } else {
                    AdValue::constant(0.0)
                }
            });
        }

        if (scratch.values[2598] != 0.0) {
            scratch.store_ad(1022, &AdValue::div(AdValue::scale(AdValue::add(scratch.ad_value(2069), scratch.ad_value(1018)), scratch.values[759]), scratch.ad_value(1018)));
        }

        if (scratch.values[2598] != 0.0) {
            scratch.store_ad(1023, &AdValue::div(AdValue::mul(AdValue::scale(scratch.ad_value(2011), 1.0 / (scratch.values[759])), scratch.ad_value(2069)), AdValue::add(scratch.ad_value(2069), scratch.ad_value(1018))));
        }

        if (scratch.values[2598] != 0.0) {
            scratch.store_ad(1024, &AdValue::div(AdValue::mul(AdValue::scale(scratch.ad_value(2071), (((-0.5) * 0.16666666666666666) * scratch.values[759])), scratch.ad_value(2067)), scratch.ad_value(1022)));
        }

        if (scratch.values[2598] != 0.0) {
            scratch.store_ad(1025, &AdValue::square(scratch.ad_value(1024)));
        }

        if (scratch.values[2598] != 0.0) {
            scratch.values[1026] = 0.0;
            scratch.node_derivatives[1026] = [0.0; Instance::NODE_COUNT];
            scratch.branch_derivatives[1026] = [0.0; Instance::BRANCH_COUNT];
        }

        if (scratch.values[2598] != 0.0) {
            scratch.store_ad(2077, &AdValue::mul(scratch.ad_value(2014), scratch.ad_value(2018)));
        }

        scratch.values[2599] = if (scratch.values[2077] > 1e-10) { 1.0 } else { 0.0 };

        if ((scratch.values[2598] != 0.0) && (scratch.values[2599] != 0.0)) {
            scratch.store_ad(1026, &AdValue::offset(AdValue::div(AdValue::mul(scratch.ad_value(1018), scratch.ad_value(1022)), scratch.ad_value(2077)), (-1.0)));
        }

        if (scratch.values[2598] != 0.0) {
            scratch.store_ad(1027, &{
                if ((1.0 - (12.0 * (scratch.values[1026] * scratch.values[1025]))) > 1e-20) {
                    AdValue::sub_from_scalar(1.0, AdValue::scale(AdValue::mul(scratch.ad_value(1026), scratch.ad_value(1025)), 12.0))
                } else {
                    AdValue::constant(1e-20)
                }
            });
        }

        if (scratch.values[2598] != 0.0) {
            scratch.store_ad(1028, &AdValue::div_from_scalar(1.0, AdValue::square(scratch.ad_value(1027))));
        }

        if (scratch.values[2598] != 0.0) {
            scratch.store_ad(1029, &AdValue::mul(AdValue::mul(AdValue::mul(AdValue::scale(scratch.ad_value(761), scratch.values[759]), AdValue::add(scratch.ad_value(2069), scratch.ad_value(1018))), scratch.ad_value(2019)), scratch.ad_value(2020)));
        }

        if (scratch.values[2598] != 0.0) {
            scratch.store_ad(2074, &AdValue::sub(AdValue::add(scratch.ad_value(1023), AdValue::scale(scratch.ad_value(1025), 12.0)), AdValue::scale(AdValue::mul(AdValue::mul(AdValue::offset(scratch.ad_value(1023), 1.0), scratch.ad_value(1025)), scratch.ad_value(1026)), 24.0)));
        }

        if (scratch.values[2598] != 0.0) {
            scratch.store_ad(2074, &{
                if (scratch.values[2074] > 1e-40) {
                    scratch.ad_value(2074)
                } else {
                    AdValue::constant(1e-40)
                }
            });
        }

        if (scratch.values[2598] != 0.0) {
            scratch.store_ad(2074, &AdValue::mul(AdValue::mul(scratch.ad_value(1029), scratch.ad_value(1028)), scratch.ad_value(2074)));
        }

        if (scratch.values[2598] != 0.0) {
            scratch.store_ad(2075, &AdValue::sqrt(AdValue::mul(scratch.ad_value(770), scratch.ad_value(2074))));
        }

    }
}
