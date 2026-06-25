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
        scratch.values[2506] = if (scratch.values[691] > 0.0) { 1.0 } else { 0.0 };

        if (((((scratch.values[2458] != 0.0) && (!(scratch.values[2459] != 0.0))) && (!(scratch.values[2499] != 0.0))) && (!(scratch.values[2504] != 0.0))) && (scratch.values[2506] != 0.0)) {
            scratch.store_ad(653, &AdValue::div_from_scalar(1.0, AdValue::offset(AdValue::scale(scratch.ad_value(691), scratch.values[428]), 1.0)));
        }

        if (((((scratch.values[2458] != 0.0) && (!(scratch.values[2459] != 0.0))) && (!(scratch.values[2499] != 0.0))) && (!(scratch.values[2504] != 0.0))) && (!(scratch.values[2506] != 0.0))) {
            scratch.store_ad(653, &AdValue::div_from_scalar(1.0, AdValue::sub_from_scalar(1.0, AdValue::scale(scratch.ad_value(691), scratch.values[428]))));
        }

        scratch.values[2507] = if (((-scratch.values[652]) + scratch.values[690]) > (-230.25850929940458)) { 1.0 } else { 0.0 };

        if (((((scratch.values[2458] != 0.0) && (!(scratch.values[2459] != 0.0))) && (!(scratch.values[2499] != 0.0))) && (!(scratch.values[2504] != 0.0))) && (scratch.values[2507] != 0.0)) {
            scratch.store_ad(670, &AdValue::exp(AdValue::sub(scratch.ad_value(690), scratch.ad_value(652))));
        }

        if (((((scratch.values[2458] != 0.0) && (!(scratch.values[2459] != 0.0))) && (!(scratch.values[2499] != 0.0))) && (!(scratch.values[2504] != 0.0))) && (!(scratch.values[2507] != 0.0))) {
            scratch.store_ad(670, &AdValue::div_from_scalar(1e-100, AdValue::offset(AdValue::mul(AdValue::sub_from_scalar((-230.25850929940458), AdValue::sub(scratch.ad_value(690), scratch.ad_value(652))), AdValue::offset(AdValue::scale(AdValue::mul(AdValue::sub_from_scalar((-230.25850929940458), AdValue::sub(scratch.ad_value(690), scratch.ad_value(652))), AdValue::offset(AdValue::scale(AdValue::sub_from_scalar((-230.25850929940458), AdValue::sub(scratch.ad_value(690), scratch.ad_value(652))), 0.3333333333333333), 1.0)), 0.5), 1.0)), 1.0)));
        }

        if ((((scratch.values[2458] != 0.0) && (!(scratch.values[2459] != 0.0))) && (!(scratch.values[2499] != 0.0))) && (!(scratch.values[2504] != 0.0))) {
            scratch.store_ad(654, &AdValue::mul(AdValue::add(AdValue::add(AdValue::scale(scratch.ad_value(653), 0.29214664), AdValue::scale(AdValue::square(scratch.ad_value(653)), scratch.values[429])), AdValue::scale(AdValue::mul(AdValue::square(scratch.ad_value(653)), scratch.ad_value(653)), scratch.values[430])), scratch.ad_value(670)));
        }

        scratch.values[2508] = if (scratch.values[691] > 0.0) { 1.0 } else { 0.0 };

        if (((((scratch.values[2458] != 0.0) && (!(scratch.values[2459] != 0.0))) && (!(scratch.values[2499] != 0.0))) && (!(scratch.values[2504] != 0.0))) && (scratch.values[2508] != 0.0)) {
            scratch.values[692] = scratch.values[654];
            scratch.node_derivatives[692] = scratch.node_derivatives[654];
            scratch.branch_derivatives[692] = scratch.branch_derivatives[654];
        }

        scratch.values[2509] = if (scratch.values[690] > (-230.25850929940458)) { 1.0 } else { 0.0 };

        if ((((((scratch.values[2458] != 0.0) && (!(scratch.values[2459] != 0.0))) && (!(scratch.values[2499] != 0.0))) && (!(scratch.values[2504] != 0.0))) && (!(scratch.values[2508] != 0.0))) && (scratch.values[2509] != 0.0)) {
            scratch.store_ad(670, &AdValue::exp(scratch.ad_value(690)));
        }

        if ((((((scratch.values[2458] != 0.0) && (!(scratch.values[2459] != 0.0))) && (!(scratch.values[2499] != 0.0))) && (!(scratch.values[2504] != 0.0))) && (!(scratch.values[2508] != 0.0))) && (!(scratch.values[2509] != 0.0))) {
            scratch.store_ad(670, &AdValue::div_from_scalar(1e-100, AdValue::offset(AdValue::mul(AdValue::sub_from_scalar((-230.25850929940458), scratch.ad_value(690)), AdValue::offset(AdValue::scale(AdValue::mul(AdValue::sub_from_scalar((-230.25850929940458), scratch.ad_value(690)), AdValue::offset(AdValue::scale(AdValue::sub_from_scalar((-230.25850929940458), scratch.ad_value(690)), 0.3333333333333333), 1.0)), 0.5), 1.0)), 1.0)));
        }

        if (((((scratch.values[2458] != 0.0) && (!(scratch.values[2459] != 0.0))) && (!(scratch.values[2499] != 0.0))) && (!(scratch.values[2504] != 0.0))) && (!(scratch.values[2508] != 0.0))) {
            scratch.store_ad(692, &AdValue::sub(AdValue::scale(scratch.ad_value(670), 2.0), scratch.ad_value(654)));
        }

        if ((((scratch.values[2458] != 0.0) && (!(scratch.values[2459] != 0.0))) && (!(scratch.values[2499] != 0.0))) && (!(scratch.values[2504] != 0.0))) {
            scratch.store_ad(693, &AdValue::scale(AdValue::div(AdValue::scale(scratch.ad_value(692), scratch.values[492]), scratch.ad_value(688)), (1.772453850905516 * 0.5)));
        }

        if ((((scratch.values[2458] != 0.0) && (!(scratch.values[2459] != 0.0))) && (!(scratch.values[2499] != 0.0))) && (!(scratch.values[2504] != 0.0))) {
            scratch.store_ad(679, &AdValue::scale(AdValue::mul(AdValue::mul(scratch.ad_value(678), scratch.ad_value(693)), scratch.ad_value(687)), scratch.values[397]));
        }

        scratch.values[2510] = if (scratch.values[403] == 0.0) { 1.0 } else { 0.0 };

        if ((((scratch.values[2458] != 0.0) && (!(scratch.values[2459] != 0.0))) && (!(scratch.values[2499] != 0.0))) && (scratch.values[2510] != 0.0)) {
            scratch.values[694] = 0.0;
            scratch.node_derivatives[694] = [0.0; Instance::NODE_COUNT];
            scratch.branch_derivatives[694] = [0.0; Instance::BRANCH_COUNT];
        }

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

        if ((((scratch.values[2458] != 0.0) && (!(scratch.values[2459] != 0.0))) && (!(scratch.values[2499] != 0.0))) && (!(scratch.values[2510] != 0.0))) {
            scratch.store_ad(694, &AdValue::scale(AdValue::mul(AdValue::mul(AdValue::mul(scratch.ad_value(854), scratch.ad_value(695)), scratch.ad_value(695)), scratch.ad_value(670)), scratch.values[403]));
        }

        scratch.values[2514] = if (scratch.values[412] > 1000.0) { 1.0 } else { 0.0 };

        if ((((scratch.values[2458] != 0.0) && (!(scratch.values[2459] != 0.0))) && (!(scratch.values[2499] != 0.0))) && (scratch.values[2514] != 0.0)) {
            scratch.values[696] = 1.0;
            scratch.node_derivatives[696] = [0.0; Instance::NODE_COUNT];
            scratch.branch_derivatives[696] = [0.0; Instance::BRANCH_COUNT];
        }

        scratch.values[2515] = if (scratch.values[669] > ((-scratch.values[500]) * scratch.values[412])) { 1.0 } else { 0.0 };

        scratch.values[2516] = if (scratch.values[415] == 4.0) { 1.0 } else { 0.0 };

        if ((((((scratch.values[2458] != 0.0) && (!(scratch.values[2459] != 0.0))) && (!(scratch.values[2499] != 0.0))) && (!(scratch.values[2514] != 0.0))) && (scratch.values[2515] != 0.0)) && (scratch.values[2516] != 0.0)) {
            scratch.store_ad(670, &AdValue::mul(AdValue::mul(AdValue::mul(AdValue::scale(scratch.ad_value(669), scratch.values[505]), AdValue::scale(scratch.ad_value(669), scratch.values[505])), AdValue::scale(scratch.ad_value(669), scratch.values[505])), AdValue::scale(scratch.ad_value(669), scratch.values[505])));
        }

        if ((((((scratch.values[2458] != 0.0) && (!(scratch.values[2459] != 0.0))) && (!(scratch.values[2499] != 0.0))) && (!(scratch.values[2514] != 0.0))) && (scratch.values[2515] != 0.0)) && (!(scratch.values[2516] != 0.0))) {
            scratch.store_ad(670, &AdValue::powf(AdValue::abs(AdValue::scale(scratch.ad_value(669), scratch.values[505])), scratch.values[415]));
        }

        if (((((scratch.values[2458] != 0.0) && (!(scratch.values[2459] != 0.0))) && (!(scratch.values[2499] != 0.0))) && (!(scratch.values[2514] != 0.0))) && (scratch.values[2515] != 0.0)) {
            scratch.store_ad(696, &AdValue::div_from_scalar(1.0, AdValue::sub_from_scalar(1.0, scratch.ad_value(670))));
        }

        if (((((scratch.values[2458] != 0.0) && (!(scratch.values[2459] != 0.0))) && (!(scratch.values[2499] != 0.0))) && (!(scratch.values[2514] != 0.0))) && (!(scratch.values[2515] != 0.0))) {
            scratch.store_ad(696, &AdValue::offset(AdValue::scale(AdValue::offset(scratch.ad_value(669), (scratch.values[500] * scratch.values[412])), scratch.values[508]), scratch.values[502]));
        }

        if (((scratch.values[2458] != 0.0) && (!(scratch.values[2459] != 0.0))) && (!(scratch.values[2499] != 0.0))) {
            scratch.store_ad(2032, &AdValue::mul(AdValue::add(AdValue::add(AdValue::add(scratch.ad_value(671), scratch.ad_value(672)), scratch.ad_value(679)), scratch.ad_value(694)), scratch.ad_value(696)));
        }

        scratch.values[2517] = if (scratch.values[699] == 0.0) { 1.0 } else { 0.0 };

        if (((scratch.values[2458] != 0.0) && (!(scratch.values[2459] != 0.0))) && (scratch.values[2517] != 0.0)) {
            scratch.values[2033] = 0.0;
            scratch.node_derivatives[2033] = [0.0; Instance::NODE_COUNT];
            scratch.branch_derivatives[2033] = [0.0; Instance::BRANCH_COUNT];
        }

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

        if (((scratch.values[2458] != 0.0) && (!(scratch.values[2459] != 0.0))) && (!(scratch.values[2517] != 0.0))) {
            scratch.store_ad(671, &AdValue::scale(scratch.ad_value(661), scratch.values[445]));
        }

        scratch.values[2519] = if ((scratch.values[395] == 0.0) && (scratch.values[398] == 0.0)) { 1.0 } else { 0.0 };

        if ((((scratch.values[2458] != 0.0) && (!(scratch.values[2459] != 0.0))) && (!(scratch.values[2517] != 0.0))) && (scratch.values[2519] != 0.0)) {
            scratch.values[672] = 0.0;
            scratch.node_derivatives[672] = [0.0; Instance::NODE_COUNT];
            scratch.branch_derivatives[672] = [0.0; Instance::BRANCH_COUNT];
        }

        if ((((scratch.values[2458] != 0.0) && (!(scratch.values[2459] != 0.0))) && (!(scratch.values[2517] != 0.0))) && (!(scratch.values[2519] != 0.0))) {
            scratch.store_ad(673, &AdValue::sub_from_scalar(scratch.values[451], scratch.ad_value(667)));
        }

        if ((((scratch.values[2458] != 0.0) && (!(scratch.values[2459] != 0.0))) && (!(scratch.values[2517] != 0.0))) && (!(scratch.values[2519] != 0.0))) {
            scratch.store_ad(674, &AdValue::sub_from_scalar(1.0, AdValue::sqrt(AdValue::sub_from_scalar(1.0, AdValue::div(scratch.ad_value(665), scratch.ad_value(673))))));
        }

        scratch.values[2520] = if (scratch.values[384] == 0.5) { 1.0 } else { 0.0 };

        if (((((scratch.values[2458] != 0.0) && (!(scratch.values[2459] != 0.0))) && (!(scratch.values[2517] != 0.0))) && (!(scratch.values[2519] != 0.0))) && (scratch.values[2520] != 0.0)) {
            scratch.values[675] = 0.0;
            scratch.node_derivatives[675] = [0.0; Instance::NODE_COUNT];
            scratch.branch_derivatives[675] = [0.0; Instance::BRANCH_COUNT];
        }

        if (((((scratch.values[2458] != 0.0) && (!(scratch.values[2459] != 0.0))) && (!(scratch.values[2517] != 0.0))) && (!(scratch.values[2519] != 0.0))) && (!(scratch.values[2520] != 0.0))) {
            scratch.store_ad(675, &AdValue::scale(AdValue::add(AdValue::div(AdValue::mul(AdValue::square(scratch.ad_value(674)), AdValue::ln(scratch.ad_value(674))), AdValue::sub_from_scalar(1.0, scratch.ad_value(674))), scratch.ad_value(674)), (1.0 - (2.0 * scratch.values[384]))));
        }

        if ((((scratch.values[2458] != 0.0) && (!(scratch.values[2459] != 0.0))) && (!(scratch.values[2517] != 0.0))) && (!(scratch.values[2519] != 0.0))) {
            scratch.store_ad(676, &AdValue::add(scratch.ad_value(674), scratch.ad_value(675)));
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

        if ((((scratch.values[2458] != 0.0) && (!(scratch.values[2459] != 0.0))) && (!(scratch.values[2517] != 0.0))) && (!(scratch.values[2519] != 0.0))) {
            scratch.store_ad(678, &AdValue::scale(AdValue::mul(AdValue::offset(scratch.ad_value(664), (-1.0)), scratch.ad_value(677)), scratch.values[442]));
        }

        if ((((scratch.values[2458] != 0.0) && (!(scratch.values[2459] != 0.0))) && (!(scratch.values[2517] != 0.0))) && (!(scratch.values[2519] != 0.0))) {
            scratch.store_ad(672, &AdValue::scale(AdValue::mul(scratch.ad_value(678), scratch.ad_value(676)), scratch.values[395]));
        }

        scratch.values[2522] = if (scratch.values[398] == 0.0) { 1.0 } else { 0.0 };

        if ((((scratch.values[2458] != 0.0) && (!(scratch.values[2459] != 0.0))) && (!(scratch.values[2517] != 0.0))) && (scratch.values[2522] != 0.0)) {
            scratch.values[679] = 0.0;
            scratch.node_derivatives[679] = [0.0; Instance::NODE_COUNT];
            scratch.branch_derivatives[679] = [0.0; Instance::BRANCH_COUNT];
        }

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

        scratch.values[2523] = if (((-scratch.values[384]) * scratch.values[469]) == (-1.0)) { 1.0 } else { 0.0 };

        if (((((scratch.values[2458] != 0.0) && (!(scratch.values[2459] != 0.0))) && (!(scratch.values[2517] != 0.0))) && (!(scratch.values[2522] != 0.0))) && (scratch.values[2523] != 0.0)) {
            scratch.store_ad(686, &AdValue::div_from_scalar(1.0, AdValue::offset(AdValue::mul(scratch.ad_value(680), scratch.ad_value(685)), 1.0)));
        }

        if (((((scratch.values[2458] != 0.0) && (!(scratch.values[2459] != 0.0))) && (!(scratch.values[2517] != 0.0))) && (!(scratch.values[2522] != 0.0))) && (!(scratch.values[2523] != 0.0))) {
            scratch.store_ad(686, &AdValue::powf(AdValue::offset(AdValue::mul(scratch.ad_value(680), scratch.ad_value(685)), 1.0), ((-scratch.values[384]) * scratch.values[469])));
        }

        if ((((scratch.values[2458] != 0.0) && (!(scratch.values[2459] != 0.0))) && (!(scratch.values[2517] != 0.0))) && (!(scratch.values[2522] != 0.0))) {
            scratch.store_ad(687, &AdValue::div(AdValue::mul(scratch.ad_value(676), scratch.ad_value(686)), AdValue::add(scratch.ad_value(676), scratch.ad_value(686))));
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

        scratch.values[2524] = if (scratch.values[691] > 0.0) { 1.0 } else { 0.0 };

        if (((((scratch.values[2458] != 0.0) && (!(scratch.values[2459] != 0.0))) && (!(scratch.values[2517] != 0.0))) && (!(scratch.values[2522] != 0.0))) && (scratch.values[2524] != 0.0)) {
            scratch.store_ad(653, &AdValue::div_from_scalar(1.0, AdValue::offset(AdValue::scale(scratch.ad_value(691), scratch.values[428]), 1.0)));
        }

        if (((((scratch.values[2458] != 0.0) && (!(scratch.values[2459] != 0.0))) && (!(scratch.values[2517] != 0.0))) && (!(scratch.values[2522] != 0.0))) && (!(scratch.values[2524] != 0.0))) {
            scratch.store_ad(653, &AdValue::div_from_scalar(1.0, AdValue::sub_from_scalar(1.0, AdValue::scale(scratch.ad_value(691), scratch.values[428]))));
        }

        scratch.values[2525] = if (((-scratch.values[652]) + scratch.values[690]) > (-230.25850929940458)) { 1.0 } else { 0.0 };

        if (((((scratch.values[2458] != 0.0) && (!(scratch.values[2459] != 0.0))) && (!(scratch.values[2517] != 0.0))) && (!(scratch.values[2522] != 0.0))) && (scratch.values[2525] != 0.0)) {
            scratch.store_ad(670, &AdValue::exp(AdValue::sub(scratch.ad_value(690), scratch.ad_value(652))));
        }

        if (((((scratch.values[2458] != 0.0) && (!(scratch.values[2459] != 0.0))) && (!(scratch.values[2517] != 0.0))) && (!(scratch.values[2522] != 0.0))) && (!(scratch.values[2525] != 0.0))) {
            scratch.store_ad(670, &AdValue::div_from_scalar(1e-100, AdValue::offset(AdValue::mul(AdValue::sub_from_scalar((-230.25850929940458), AdValue::sub(scratch.ad_value(690), scratch.ad_value(652))), AdValue::offset(AdValue::scale(AdValue::mul(AdValue::sub_from_scalar((-230.25850929940458), AdValue::sub(scratch.ad_value(690), scratch.ad_value(652))), AdValue::offset(AdValue::scale(AdValue::sub_from_scalar((-230.25850929940458), AdValue::sub(scratch.ad_value(690), scratch.ad_value(652))), 0.3333333333333333), 1.0)), 0.5), 1.0)), 1.0)));
        }

        if ((((scratch.values[2458] != 0.0) && (!(scratch.values[2459] != 0.0))) && (!(scratch.values[2517] != 0.0))) && (!(scratch.values[2522] != 0.0))) {
            scratch.store_ad(654, &AdValue::mul(AdValue::add(AdValue::add(AdValue::scale(scratch.ad_value(653), 0.29214664), AdValue::scale(AdValue::square(scratch.ad_value(653)), scratch.values[429])), AdValue::scale(AdValue::mul(AdValue::square(scratch.ad_value(653)), scratch.ad_value(653)), scratch.values[430])), scratch.ad_value(670)));
        }

        scratch.values[2526] = if (scratch.values[691] > 0.0) { 1.0 } else { 0.0 };

        if (((((scratch.values[2458] != 0.0) && (!(scratch.values[2459] != 0.0))) && (!(scratch.values[2517] != 0.0))) && (!(scratch.values[2522] != 0.0))) && (scratch.values[2526] != 0.0)) {
            scratch.values[692] = scratch.values[654];
            scratch.node_derivatives[692] = scratch.node_derivatives[654];
            scratch.branch_derivatives[692] = scratch.branch_derivatives[654];
        }

        scratch.values[2527] = if (scratch.values[690] > (-230.25850929940458)) { 1.0 } else { 0.0 };

        if ((((((scratch.values[2458] != 0.0) && (!(scratch.values[2459] != 0.0))) && (!(scratch.values[2517] != 0.0))) && (!(scratch.values[2522] != 0.0))) && (!(scratch.values[2526] != 0.0))) && (scratch.values[2527] != 0.0)) {
            scratch.store_ad(670, &AdValue::exp(scratch.ad_value(690)));
        }

        if ((((((scratch.values[2458] != 0.0) && (!(scratch.values[2459] != 0.0))) && (!(scratch.values[2517] != 0.0))) && (!(scratch.values[2522] != 0.0))) && (!(scratch.values[2526] != 0.0))) && (!(scratch.values[2527] != 0.0))) {
            scratch.store_ad(670, &AdValue::div_from_scalar(1e-100, AdValue::offset(AdValue::mul(AdValue::sub_from_scalar((-230.25850929940458), scratch.ad_value(690)), AdValue::offset(AdValue::scale(AdValue::mul(AdValue::sub_from_scalar((-230.25850929940458), scratch.ad_value(690)), AdValue::offset(AdValue::scale(AdValue::sub_from_scalar((-230.25850929940458), scratch.ad_value(690)), 0.3333333333333333), 1.0)), 0.5), 1.0)), 1.0)));
        }

        if (((((scratch.values[2458] != 0.0) && (!(scratch.values[2459] != 0.0))) && (!(scratch.values[2517] != 0.0))) && (!(scratch.values[2522] != 0.0))) && (!(scratch.values[2526] != 0.0))) {
            scratch.store_ad(692, &AdValue::sub(AdValue::scale(scratch.ad_value(670), 2.0), scratch.ad_value(654)));
        }

        if ((((scratch.values[2458] != 0.0) && (!(scratch.values[2459] != 0.0))) && (!(scratch.values[2517] != 0.0))) && (!(scratch.values[2522] != 0.0))) {
            scratch.store_ad(693, &AdValue::scale(AdValue::div(AdValue::scale(scratch.ad_value(692), scratch.values[493]), scratch.ad_value(688)), (1.772453850905516 * 0.5)));
        }

        if ((((scratch.values[2458] != 0.0) && (!(scratch.values[2459] != 0.0))) && (!(scratch.values[2517] != 0.0))) && (!(scratch.values[2522] != 0.0))) {
            scratch.store_ad(679, &AdValue::scale(AdValue::mul(AdValue::mul(scratch.ad_value(678), scratch.ad_value(693)), scratch.ad_value(687)), scratch.values[398]));
        }

        scratch.values[2528] = if (scratch.values[404] == 0.0) { 1.0 } else { 0.0 };

        if ((((scratch.values[2458] != 0.0) && (!(scratch.values[2459] != 0.0))) && (!(scratch.values[2517] != 0.0))) && (scratch.values[2528] != 0.0)) {
            scratch.values[694] = 0.0;
            scratch.node_derivatives[694] = [0.0; Instance::NODE_COUNT];
            scratch.branch_derivatives[694] = [0.0; Instance::BRANCH_COUNT];
        }

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

        if ((((scratch.values[2458] != 0.0) && (!(scratch.values[2459] != 0.0))) && (!(scratch.values[2517] != 0.0))) && (!(scratch.values[2528] != 0.0))) {
            scratch.store_ad(694, &AdValue::scale(AdValue::mul(AdValue::mul(AdValue::mul(scratch.ad_value(854), scratch.ad_value(695)), scratch.ad_value(695)), scratch.ad_value(670)), scratch.values[404]));
        }

        scratch.values[2532] = if (scratch.values[413] > 1000.0) { 1.0 } else { 0.0 };

        if ((((scratch.values[2458] != 0.0) && (!(scratch.values[2459] != 0.0))) && (!(scratch.values[2517] != 0.0))) && (scratch.values[2532] != 0.0)) {
            scratch.values[696] = 1.0;
            scratch.node_derivatives[696] = [0.0; Instance::NODE_COUNT];
            scratch.branch_derivatives[696] = [0.0; Instance::BRANCH_COUNT];
        }

        scratch.values[2533] = if (scratch.values[669] > ((-scratch.values[500]) * scratch.values[413])) { 1.0 } else { 0.0 };

        scratch.values[2534] = if (scratch.values[416] == 4.0) { 1.0 } else { 0.0 };

        if ((((((scratch.values[2458] != 0.0) && (!(scratch.values[2459] != 0.0))) && (!(scratch.values[2517] != 0.0))) && (!(scratch.values[2532] != 0.0))) && (scratch.values[2533] != 0.0)) && (scratch.values[2534] != 0.0)) {
            scratch.store_ad(670, &AdValue::mul(AdValue::mul(AdValue::mul(AdValue::scale(scratch.ad_value(669), scratch.values[506]), AdValue::scale(scratch.ad_value(669), scratch.values[506])), AdValue::scale(scratch.ad_value(669), scratch.values[506])), AdValue::scale(scratch.ad_value(669), scratch.values[506])));
        }

        if ((((((scratch.values[2458] != 0.0) && (!(scratch.values[2459] != 0.0))) && (!(scratch.values[2517] != 0.0))) && (!(scratch.values[2532] != 0.0))) && (scratch.values[2533] != 0.0)) && (!(scratch.values[2534] != 0.0))) {
            scratch.store_ad(670, &AdValue::powf(AdValue::abs(AdValue::scale(scratch.ad_value(669), scratch.values[506])), scratch.values[416]));
        }

        if (((((scratch.values[2458] != 0.0) && (!(scratch.values[2459] != 0.0))) && (!(scratch.values[2517] != 0.0))) && (!(scratch.values[2532] != 0.0))) && (scratch.values[2533] != 0.0)) {
            scratch.store_ad(696, &AdValue::div_from_scalar(1.0, AdValue::sub_from_scalar(1.0, scratch.ad_value(670))));
        }

        if (((((scratch.values[2458] != 0.0) && (!(scratch.values[2459] != 0.0))) && (!(scratch.values[2517] != 0.0))) && (!(scratch.values[2532] != 0.0))) && (!(scratch.values[2533] != 0.0))) {
            scratch.store_ad(696, &AdValue::offset(AdValue::scale(AdValue::offset(scratch.ad_value(669), (scratch.values[500] * scratch.values[413])), scratch.values[509]), scratch.values[503]));
        }

        if (((scratch.values[2458] != 0.0) && (!(scratch.values[2459] != 0.0))) && (!(scratch.values[2517] != 0.0))) {
            scratch.store_ad(2033, &AdValue::mul(AdValue::add(AdValue::add(AdValue::add(scratch.ad_value(671), scratch.ad_value(672)), scratch.ad_value(679)), scratch.ad_value(694)), scratch.ad_value(696)));
        }

        if ((scratch.values[2458] != 0.0) && (!(scratch.values[2459] != 0.0))) {
            scratch.store_ad(2030, &AdValue::add(AdValue::add(AdValue::mul(scratch.ad_value(697), scratch.ad_value(2031)), AdValue::mul(scratch.ad_value(698), scratch.ad_value(2032))), AdValue::mul(scratch.ad_value(699), scratch.ad_value(2033))));
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

    }

    pub(super) fn stamp_transient_block_41(
        &mut self,
        ctx: &GeneratedEvalContext<'_>,
        stamper: &mut GeneratedStamper<'_>,
        scratch: &mut Scratch,
    ) {
        let _ = stamper;
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
            scratch.values[2035] = 0.0;
            scratch.node_derivatives[2035] = [0.0; Instance::NODE_COUNT];
            scratch.branch_derivatives[2035] = [0.0; Instance::BRANCH_COUNT];
        }

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

        if (((scratch.values[2458] != 0.0) && (!(scratch.values[2459] != 0.0))) && (!(scratch.values[2540] != 0.0))) {
            scratch.store_ad(671, &AdValue::mul(scratch.ad_value(595), scratch.ad_value(661)));
        }

        scratch.values[2542] = if ((scratch.values[560] == 0.0) && (scratch.values[563] == 0.0)) { 1.0 } else { 0.0 };

        if ((((scratch.values[2458] != 0.0) && (!(scratch.values[2459] != 0.0))) && (!(scratch.values[2540] != 0.0))) && (scratch.values[2542] != 0.0)) {
            scratch.values[672] = 0.0;
            scratch.node_derivatives[672] = [0.0; Instance::NODE_COUNT];
            scratch.branch_derivatives[672] = [0.0; Instance::BRANCH_COUNT];
        }

        if ((((scratch.values[2458] != 0.0) && (!(scratch.values[2459] != 0.0))) && (!(scratch.values[2540] != 0.0))) && (!(scratch.values[2542] != 0.0))) {
            scratch.store_ad(673, &AdValue::sub(scratch.ad_value(601), scratch.ad_value(667)));
        }

        if ((((scratch.values[2458] != 0.0) && (!(scratch.values[2459] != 0.0))) && (!(scratch.values[2540] != 0.0))) && (!(scratch.values[2542] != 0.0))) {
            scratch.store_ad(674, &AdValue::sub_from_scalar(1.0, AdValue::sqrt(AdValue::sub_from_scalar(1.0, AdValue::div(scratch.ad_value(665), scratch.ad_value(673))))));
        }

        scratch.values[2543] = if (scratch.values[549] == 0.5) { 1.0 } else { 0.0 };

        if (((((scratch.values[2458] != 0.0) && (!(scratch.values[2459] != 0.0))) && (!(scratch.values[2540] != 0.0))) && (!(scratch.values[2542] != 0.0))) && (scratch.values[2543] != 0.0)) {
            scratch.values[675] = 0.0;
            scratch.node_derivatives[675] = [0.0; Instance::NODE_COUNT];
            scratch.branch_derivatives[675] = [0.0; Instance::BRANCH_COUNT];
        }

        if (((((scratch.values[2458] != 0.0) && (!(scratch.values[2459] != 0.0))) && (!(scratch.values[2540] != 0.0))) && (!(scratch.values[2542] != 0.0))) && (!(scratch.values[2543] != 0.0))) {
            scratch.store_ad(675, &AdValue::mul(AdValue::add(AdValue::div(AdValue::mul(AdValue::square(scratch.ad_value(674)), AdValue::ln(scratch.ad_value(674))), AdValue::sub_from_scalar(1.0, scratch.ad_value(674))), scratch.ad_value(674)), AdValue::sub_from_scalar(1.0, AdValue::scale(scratch.ad_value(549), 2.0))));
        }

        if ((((scratch.values[2458] != 0.0) && (!(scratch.values[2459] != 0.0))) && (!(scratch.values[2540] != 0.0))) && (!(scratch.values[2542] != 0.0))) {
            scratch.store_ad(676, &AdValue::add(scratch.ad_value(674), scratch.ad_value(675)));
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

        if ((((scratch.values[2458] != 0.0) && (!(scratch.values[2459] != 0.0))) && (!(scratch.values[2540] != 0.0))) && (!(scratch.values[2542] != 0.0))) {
            scratch.store_ad(678, &AdValue::mul(scratch.ad_value(592), AdValue::mul(AdValue::offset(scratch.ad_value(664), (-1.0)), scratch.ad_value(677))));
        }

        if ((((scratch.values[2458] != 0.0) && (!(scratch.values[2459] != 0.0))) && (!(scratch.values[2540] != 0.0))) && (!(scratch.values[2542] != 0.0))) {
            scratch.store_ad(672, &AdValue::mul(scratch.ad_value(560), AdValue::mul(scratch.ad_value(678), scratch.ad_value(676))));
        }

        scratch.values[2545] = if (scratch.values[563] == 0.0) { 1.0 } else { 0.0 };

        if ((((scratch.values[2458] != 0.0) && (!(scratch.values[2459] != 0.0))) && (!(scratch.values[2540] != 0.0))) && (scratch.values[2545] != 0.0)) {
            scratch.values[679] = 0.0;
            scratch.node_derivatives[679] = [0.0; Instance::NODE_COUNT];
            scratch.branch_derivatives[679] = [0.0; Instance::BRANCH_COUNT];
        }

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

        scratch.values[2546] = if (((-scratch.values[549]) * scratch.values[610]) == (-1.0)) { 1.0 } else { 0.0 };

        if (((((scratch.values[2458] != 0.0) && (!(scratch.values[2459] != 0.0))) && (!(scratch.values[2540] != 0.0))) && (!(scratch.values[2545] != 0.0))) && (scratch.values[2546] != 0.0)) {
            scratch.store_ad(686, &AdValue::div_from_scalar(1.0, AdValue::offset(AdValue::mul(scratch.ad_value(680), scratch.ad_value(685)), 1.0)));
        }

        if (((((scratch.values[2458] != 0.0) && (!(scratch.values[2459] != 0.0))) && (!(scratch.values[2540] != 0.0))) && (!(scratch.values[2545] != 0.0))) && (!(scratch.values[2546] != 0.0))) {
            scratch.store_ad(686, &AdValue::pow(AdValue::offset(AdValue::mul(scratch.ad_value(680), scratch.ad_value(685)), 1.0), AdValue::mul(AdValue::neg(scratch.ad_value(549)), scratch.ad_value(610))));
        }

        if ((((scratch.values[2458] != 0.0) && (!(scratch.values[2459] != 0.0))) && (!(scratch.values[2540] != 0.0))) && (!(scratch.values[2545] != 0.0))) {
            scratch.store_ad(687, &AdValue::div(AdValue::mul(scratch.ad_value(676), scratch.ad_value(686)), AdValue::add(scratch.ad_value(676), scratch.ad_value(686))));
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

        scratch.values[2547] = if (scratch.values[691] > 0.0) { 1.0 } else { 0.0 };

        if (((((scratch.values[2458] != 0.0) && (!(scratch.values[2459] != 0.0))) && (!(scratch.values[2540] != 0.0))) && (!(scratch.values[2545] != 0.0))) && (scratch.values[2547] != 0.0)) {
            scratch.store_ad(653, &AdValue::div_from_scalar(1.0, AdValue::offset(AdValue::scale(scratch.ad_value(691), scratch.values[428]), 1.0)));
        }

        if (((((scratch.values[2458] != 0.0) && (!(scratch.values[2459] != 0.0))) && (!(scratch.values[2540] != 0.0))) && (!(scratch.values[2545] != 0.0))) && (!(scratch.values[2547] != 0.0))) {
            scratch.store_ad(653, &AdValue::div_from_scalar(1.0, AdValue::sub_from_scalar(1.0, AdValue::scale(scratch.ad_value(691), scratch.values[428]))));
        }

        scratch.values[2548] = if (((-scratch.values[652]) + scratch.values[690]) > (-230.25850929940458)) { 1.0 } else { 0.0 };

        if (((((scratch.values[2458] != 0.0) && (!(scratch.values[2459] != 0.0))) && (!(scratch.values[2540] != 0.0))) && (!(scratch.values[2545] != 0.0))) && (scratch.values[2548] != 0.0)) {
            scratch.store_ad(670, &AdValue::exp(AdValue::sub(scratch.ad_value(690), scratch.ad_value(652))));
        }

        if (((((scratch.values[2458] != 0.0) && (!(scratch.values[2459] != 0.0))) && (!(scratch.values[2540] != 0.0))) && (!(scratch.values[2545] != 0.0))) && (!(scratch.values[2548] != 0.0))) {
            scratch.store_ad(670, &AdValue::div_from_scalar(1e-100, AdValue::offset(AdValue::mul(AdValue::sub_from_scalar((-230.25850929940458), AdValue::sub(scratch.ad_value(690), scratch.ad_value(652))), AdValue::offset(AdValue::scale(AdValue::mul(AdValue::sub_from_scalar((-230.25850929940458), AdValue::sub(scratch.ad_value(690), scratch.ad_value(652))), AdValue::offset(AdValue::scale(AdValue::sub_from_scalar((-230.25850929940458), AdValue::sub(scratch.ad_value(690), scratch.ad_value(652))), 0.3333333333333333), 1.0)), 0.5), 1.0)), 1.0)));
        }

        if ((((scratch.values[2458] != 0.0) && (!(scratch.values[2459] != 0.0))) && (!(scratch.values[2540] != 0.0))) && (!(scratch.values[2545] != 0.0))) {
            scratch.store_ad(654, &AdValue::mul(AdValue::add(AdValue::add(AdValue::scale(scratch.ad_value(653), 0.29214664), AdValue::scale(AdValue::square(scratch.ad_value(653)), scratch.values[429])), AdValue::scale(AdValue::mul(AdValue::square(scratch.ad_value(653)), scratch.ad_value(653)), scratch.values[430])), scratch.ad_value(670)));
        }

        scratch.values[2549] = if (scratch.values[691] > 0.0) { 1.0 } else { 0.0 };

        if (((((scratch.values[2458] != 0.0) && (!(scratch.values[2459] != 0.0))) && (!(scratch.values[2540] != 0.0))) && (!(scratch.values[2545] != 0.0))) && (scratch.values[2549] != 0.0)) {
            scratch.values[692] = scratch.values[654];
            scratch.node_derivatives[692] = scratch.node_derivatives[654];
            scratch.branch_derivatives[692] = scratch.branch_derivatives[654];
        }

        scratch.values[2550] = if (scratch.values[690] > (-230.25850929940458)) { 1.0 } else { 0.0 };

        if ((((((scratch.values[2458] != 0.0) && (!(scratch.values[2459] != 0.0))) && (!(scratch.values[2540] != 0.0))) && (!(scratch.values[2545] != 0.0))) && (!(scratch.values[2549] != 0.0))) && (scratch.values[2550] != 0.0)) {
            scratch.store_ad(670, &AdValue::exp(scratch.ad_value(690)));
        }

        if ((((((scratch.values[2458] != 0.0) && (!(scratch.values[2459] != 0.0))) && (!(scratch.values[2540] != 0.0))) && (!(scratch.values[2545] != 0.0))) && (!(scratch.values[2549] != 0.0))) && (!(scratch.values[2550] != 0.0))) {
            scratch.store_ad(670, &AdValue::div_from_scalar(1e-100, AdValue::offset(AdValue::mul(AdValue::sub_from_scalar((-230.25850929940458), scratch.ad_value(690)), AdValue::offset(AdValue::scale(AdValue::mul(AdValue::sub_from_scalar((-230.25850929940458), scratch.ad_value(690)), AdValue::offset(AdValue::scale(AdValue::sub_from_scalar((-230.25850929940458), scratch.ad_value(690)), 0.3333333333333333), 1.0)), 0.5), 1.0)), 1.0)));
        }

        if (((((scratch.values[2458] != 0.0) && (!(scratch.values[2459] != 0.0))) && (!(scratch.values[2540] != 0.0))) && (!(scratch.values[2545] != 0.0))) && (!(scratch.values[2549] != 0.0))) {
            scratch.store_ad(692, &AdValue::sub(AdValue::scale(scratch.ad_value(670), 2.0), scratch.ad_value(654)));
        }

        if ((((scratch.values[2458] != 0.0) && (!(scratch.values[2459] != 0.0))) && (!(scratch.values[2540] != 0.0))) && (!(scratch.values[2545] != 0.0))) {
            scratch.store_ad(693, &AdValue::scale(AdValue::div(AdValue::mul(scratch.ad_value(634), scratch.ad_value(692)), scratch.ad_value(688)), (1.772453850905516 * 0.5)));
        }

        if ((((scratch.values[2458] != 0.0) && (!(scratch.values[2459] != 0.0))) && (!(scratch.values[2540] != 0.0))) && (!(scratch.values[2545] != 0.0))) {
            scratch.store_ad(679, &AdValue::mul(scratch.ad_value(563), AdValue::mul(AdValue::mul(scratch.ad_value(678), scratch.ad_value(693)), scratch.ad_value(687))));
        }

        scratch.values[2551] = if (scratch.values[569] == 0.0) { 1.0 } else { 0.0 };

        if ((((scratch.values[2458] != 0.0) && (!(scratch.values[2459] != 0.0))) && (!(scratch.values[2540] != 0.0))) && (scratch.values[2551] != 0.0)) {
            scratch.values[694] = 0.0;
            scratch.node_derivatives[694] = [0.0; Instance::NODE_COUNT];
            scratch.branch_derivatives[694] = [0.0; Instance::BRANCH_COUNT];
        }

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

        if ((((scratch.values[2458] != 0.0) && (!(scratch.values[2459] != 0.0))) && (!(scratch.values[2540] != 0.0))) && (!(scratch.values[2551] != 0.0))) {
            scratch.store_ad(694, &AdValue::mul(scratch.ad_value(569), AdValue::mul(AdValue::mul(AdValue::mul(scratch.ad_value(855), scratch.ad_value(695)), scratch.ad_value(695)), scratch.ad_value(670))));
        }

        scratch.values[2555] = if (scratch.values[578] > 1000.0) { 1.0 } else { 0.0 };

        if ((((scratch.values[2458] != 0.0) && (!(scratch.values[2459] != 0.0))) && (!(scratch.values[2540] != 0.0))) && (scratch.values[2555] != 0.0)) {
            scratch.values[696] = 1.0;
            scratch.node_derivatives[696] = [0.0; Instance::NODE_COUNT];
            scratch.branch_derivatives[696] = [0.0; Instance::BRANCH_COUNT];
        }

        scratch.values[2556] = if (scratch.values[669] > ((-scratch.values[500]) * scratch.values[578])) { 1.0 } else { 0.0 };

        scratch.values[2557] = if (scratch.values[581] == 4.0) { 1.0 } else { 0.0 };

        if ((((((scratch.values[2458] != 0.0) && (!(scratch.values[2459] != 0.0))) && (!(scratch.values[2540] != 0.0))) && (!(scratch.values[2555] != 0.0))) && (scratch.values[2556] != 0.0)) && (scratch.values[2557] != 0.0)) {
            scratch.store_ad(670, &AdValue::mul(AdValue::mul(AdValue::mul(AdValue::mul(scratch.ad_value(669), scratch.ad_value(646)), AdValue::mul(scratch.ad_value(669), scratch.ad_value(646))), AdValue::mul(scratch.ad_value(669), scratch.ad_value(646))), AdValue::mul(scratch.ad_value(669), scratch.ad_value(646))));
        }

        if ((((((scratch.values[2458] != 0.0) && (!(scratch.values[2459] != 0.0))) && (!(scratch.values[2540] != 0.0))) && (!(scratch.values[2555] != 0.0))) && (scratch.values[2556] != 0.0)) && (!(scratch.values[2557] != 0.0))) {
            scratch.store_ad(670, &AdValue::pow(AdValue::abs(AdValue::mul(scratch.ad_value(669), scratch.ad_value(646))), scratch.ad_value(581)));
        }

        if (((((scratch.values[2458] != 0.0) && (!(scratch.values[2459] != 0.0))) && (!(scratch.values[2540] != 0.0))) && (!(scratch.values[2555] != 0.0))) && (scratch.values[2556] != 0.0)) {
            scratch.store_ad(696, &AdValue::div_from_scalar(1.0, AdValue::sub_from_scalar(1.0, scratch.ad_value(670))));
        }

        if (((((scratch.values[2458] != 0.0) && (!(scratch.values[2459] != 0.0))) && (!(scratch.values[2540] != 0.0))) && (!(scratch.values[2555] != 0.0))) && (!(scratch.values[2556] != 0.0))) {
            scratch.store_ad(696, &AdValue::add(scratch.ad_value(643), AdValue::mul(AdValue::add(scratch.ad_value(669), AdValue::scale(scratch.ad_value(578), scratch.values[500])), scratch.ad_value(649))));
        }

        if (((scratch.values[2458] != 0.0) && (!(scratch.values[2459] != 0.0))) && (!(scratch.values[2540] != 0.0))) {
            scratch.store_ad(2035, &AdValue::mul(AdValue::add(AdValue::add(AdValue::add(scratch.ad_value(671), scratch.ad_value(672)), scratch.ad_value(679)), scratch.ad_value(694)), scratch.ad_value(696)));
        }

        scratch.values[2558] = if (scratch.values[725] == 0.0) { 1.0 } else { 0.0 };

        if (((scratch.values[2458] != 0.0) && (!(scratch.values[2459] != 0.0))) && (scratch.values[2558] != 0.0)) {
            scratch.values[2036] = 0.0;
            scratch.node_derivatives[2036] = [0.0; Instance::NODE_COUNT];
            scratch.branch_derivatives[2036] = [0.0; Instance::BRANCH_COUNT];
        }

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

        if (((scratch.values[2458] != 0.0) && (!(scratch.values[2459] != 0.0))) && (!(scratch.values[2558] != 0.0))) {
            scratch.store_ad(671, &AdValue::mul(scratch.ad_value(596), scratch.ad_value(661)));
        }

        scratch.values[2560] = if ((scratch.values[561] == 0.0) && (scratch.values[564] == 0.0)) { 1.0 } else { 0.0 };

        if ((((scratch.values[2458] != 0.0) && (!(scratch.values[2459] != 0.0))) && (!(scratch.values[2558] != 0.0))) && (scratch.values[2560] != 0.0)) {
            scratch.values[672] = 0.0;
            scratch.node_derivatives[672] = [0.0; Instance::NODE_COUNT];
            scratch.branch_derivatives[672] = [0.0; Instance::BRANCH_COUNT];
        }

        if ((((scratch.values[2458] != 0.0) && (!(scratch.values[2459] != 0.0))) && (!(scratch.values[2558] != 0.0))) && (!(scratch.values[2560] != 0.0))) {
            scratch.store_ad(673, &AdValue::sub(scratch.ad_value(602), scratch.ad_value(667)));
        }

        if ((((scratch.values[2458] != 0.0) && (!(scratch.values[2459] != 0.0))) && (!(scratch.values[2558] != 0.0))) && (!(scratch.values[2560] != 0.0))) {
            scratch.store_ad(674, &AdValue::sub_from_scalar(1.0, AdValue::sqrt(AdValue::sub_from_scalar(1.0, AdValue::div(scratch.ad_value(665), scratch.ad_value(673))))));
        }

        scratch.values[2561] = if (scratch.values[550] == 0.5) { 1.0 } else { 0.0 };

        if (((((scratch.values[2458] != 0.0) && (!(scratch.values[2459] != 0.0))) && (!(scratch.values[2558] != 0.0))) && (!(scratch.values[2560] != 0.0))) && (scratch.values[2561] != 0.0)) {
            scratch.values[675] = 0.0;
            scratch.node_derivatives[675] = [0.0; Instance::NODE_COUNT];
            scratch.branch_derivatives[675] = [0.0; Instance::BRANCH_COUNT];
        }

        if (((((scratch.values[2458] != 0.0) && (!(scratch.values[2459] != 0.0))) && (!(scratch.values[2558] != 0.0))) && (!(scratch.values[2560] != 0.0))) && (!(scratch.values[2561] != 0.0))) {
            scratch.store_ad(675, &AdValue::mul(AdValue::add(AdValue::div(AdValue::mul(AdValue::square(scratch.ad_value(674)), AdValue::ln(scratch.ad_value(674))), AdValue::sub_from_scalar(1.0, scratch.ad_value(674))), scratch.ad_value(674)), AdValue::sub_from_scalar(1.0, AdValue::scale(scratch.ad_value(550), 2.0))));
        }

        if ((((scratch.values[2458] != 0.0) && (!(scratch.values[2459] != 0.0))) && (!(scratch.values[2558] != 0.0))) && (!(scratch.values[2560] != 0.0))) {
            scratch.store_ad(676, &AdValue::add(scratch.ad_value(674), scratch.ad_value(675)));
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

        if ((((scratch.values[2458] != 0.0) && (!(scratch.values[2459] != 0.0))) && (!(scratch.values[2558] != 0.0))) && (!(scratch.values[2560] != 0.0))) {
            scratch.store_ad(678, &AdValue::mul(scratch.ad_value(593), AdValue::mul(AdValue::offset(scratch.ad_value(664), (-1.0)), scratch.ad_value(677))));
        }

        if ((((scratch.values[2458] != 0.0) && (!(scratch.values[2459] != 0.0))) && (!(scratch.values[2558] != 0.0))) && (!(scratch.values[2560] != 0.0))) {
            scratch.store_ad(672, &AdValue::mul(scratch.ad_value(561), AdValue::mul(scratch.ad_value(678), scratch.ad_value(676))));
        }

        scratch.values[2563] = if (scratch.values[564] == 0.0) { 1.0 } else { 0.0 };

        if ((((scratch.values[2458] != 0.0) && (!(scratch.values[2459] != 0.0))) && (!(scratch.values[2558] != 0.0))) && (scratch.values[2563] != 0.0)) {
            scratch.values[679] = 0.0;
            scratch.node_derivatives[679] = [0.0; Instance::NODE_COUNT];
            scratch.branch_derivatives[679] = [0.0; Instance::BRANCH_COUNT];
        }

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

        scratch.values[2564] = if (((-scratch.values[550]) * scratch.values[611]) == (-1.0)) { 1.0 } else { 0.0 };

        if (((((scratch.values[2458] != 0.0) && (!(scratch.values[2459] != 0.0))) && (!(scratch.values[2558] != 0.0))) && (!(scratch.values[2563] != 0.0))) && (scratch.values[2564] != 0.0)) {
            scratch.store_ad(686, &AdValue::div_from_scalar(1.0, AdValue::offset(AdValue::mul(scratch.ad_value(680), scratch.ad_value(685)), 1.0)));
        }

        if (((((scratch.values[2458] != 0.0) && (!(scratch.values[2459] != 0.0))) && (!(scratch.values[2558] != 0.0))) && (!(scratch.values[2563] != 0.0))) && (!(scratch.values[2564] != 0.0))) {
            scratch.store_ad(686, &AdValue::pow(AdValue::offset(AdValue::mul(scratch.ad_value(680), scratch.ad_value(685)), 1.0), AdValue::mul(AdValue::neg(scratch.ad_value(550)), scratch.ad_value(611))));
        }

        if ((((scratch.values[2458] != 0.0) && (!(scratch.values[2459] != 0.0))) && (!(scratch.values[2558] != 0.0))) && (!(scratch.values[2563] != 0.0))) {
            scratch.store_ad(687, &AdValue::div(AdValue::mul(scratch.ad_value(676), scratch.ad_value(686)), AdValue::add(scratch.ad_value(676), scratch.ad_value(686))));
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

        scratch.values[2565] = if (scratch.values[691] > 0.0) { 1.0 } else { 0.0 };

        if (((((scratch.values[2458] != 0.0) && (!(scratch.values[2459] != 0.0))) && (!(scratch.values[2558] != 0.0))) && (!(scratch.values[2563] != 0.0))) && (scratch.values[2565] != 0.0)) {
            scratch.store_ad(653, &AdValue::div_from_scalar(1.0, AdValue::offset(AdValue::scale(scratch.ad_value(691), scratch.values[428]), 1.0)));
        }

        if (((((scratch.values[2458] != 0.0) && (!(scratch.values[2459] != 0.0))) && (!(scratch.values[2558] != 0.0))) && (!(scratch.values[2563] != 0.0))) && (!(scratch.values[2565] != 0.0))) {
            scratch.store_ad(653, &AdValue::div_from_scalar(1.0, AdValue::sub_from_scalar(1.0, AdValue::scale(scratch.ad_value(691), scratch.values[428]))));
        }

        scratch.values[2566] = if (((-scratch.values[652]) + scratch.values[690]) > (-230.25850929940458)) { 1.0 } else { 0.0 };

        if (((((scratch.values[2458] != 0.0) && (!(scratch.values[2459] != 0.0))) && (!(scratch.values[2558] != 0.0))) && (!(scratch.values[2563] != 0.0))) && (scratch.values[2566] != 0.0)) {
            scratch.store_ad(670, &AdValue::exp(AdValue::sub(scratch.ad_value(690), scratch.ad_value(652))));
        }

        if (((((scratch.values[2458] != 0.0) && (!(scratch.values[2459] != 0.0))) && (!(scratch.values[2558] != 0.0))) && (!(scratch.values[2563] != 0.0))) && (!(scratch.values[2566] != 0.0))) {
            scratch.store_ad(670, &AdValue::div_from_scalar(1e-100, AdValue::offset(AdValue::mul(AdValue::sub_from_scalar((-230.25850929940458), AdValue::sub(scratch.ad_value(690), scratch.ad_value(652))), AdValue::offset(AdValue::scale(AdValue::mul(AdValue::sub_from_scalar((-230.25850929940458), AdValue::sub(scratch.ad_value(690), scratch.ad_value(652))), AdValue::offset(AdValue::scale(AdValue::sub_from_scalar((-230.25850929940458), AdValue::sub(scratch.ad_value(690), scratch.ad_value(652))), 0.3333333333333333), 1.0)), 0.5), 1.0)), 1.0)));
        }

        if ((((scratch.values[2458] != 0.0) && (!(scratch.values[2459] != 0.0))) && (!(scratch.values[2558] != 0.0))) && (!(scratch.values[2563] != 0.0))) {
            scratch.store_ad(654, &AdValue::mul(AdValue::add(AdValue::add(AdValue::scale(scratch.ad_value(653), 0.29214664), AdValue::scale(AdValue::square(scratch.ad_value(653)), scratch.values[429])), AdValue::scale(AdValue::mul(AdValue::square(scratch.ad_value(653)), scratch.ad_value(653)), scratch.values[430])), scratch.ad_value(670)));
        }

        scratch.values[2567] = if (scratch.values[691] > 0.0) { 1.0 } else { 0.0 };

        if (((((scratch.values[2458] != 0.0) && (!(scratch.values[2459] != 0.0))) && (!(scratch.values[2558] != 0.0))) && (!(scratch.values[2563] != 0.0))) && (scratch.values[2567] != 0.0)) {
            scratch.values[692] = scratch.values[654];
            scratch.node_derivatives[692] = scratch.node_derivatives[654];
            scratch.branch_derivatives[692] = scratch.branch_derivatives[654];
        }

        scratch.values[2568] = if (scratch.values[690] > (-230.25850929940458)) { 1.0 } else { 0.0 };

        if ((((((scratch.values[2458] != 0.0) && (!(scratch.values[2459] != 0.0))) && (!(scratch.values[2558] != 0.0))) && (!(scratch.values[2563] != 0.0))) && (!(scratch.values[2567] != 0.0))) && (scratch.values[2568] != 0.0)) {
            scratch.store_ad(670, &AdValue::exp(scratch.ad_value(690)));
        }

        if ((((((scratch.values[2458] != 0.0) && (!(scratch.values[2459] != 0.0))) && (!(scratch.values[2558] != 0.0))) && (!(scratch.values[2563] != 0.0))) && (!(scratch.values[2567] != 0.0))) && (!(scratch.values[2568] != 0.0))) {
            scratch.store_ad(670, &AdValue::div_from_scalar(1e-100, AdValue::offset(AdValue::mul(AdValue::sub_from_scalar((-230.25850929940458), scratch.ad_value(690)), AdValue::offset(AdValue::scale(AdValue::mul(AdValue::sub_from_scalar((-230.25850929940458), scratch.ad_value(690)), AdValue::offset(AdValue::scale(AdValue::sub_from_scalar((-230.25850929940458), scratch.ad_value(690)), 0.3333333333333333), 1.0)), 0.5), 1.0)), 1.0)));
        }

        if (((((scratch.values[2458] != 0.0) && (!(scratch.values[2459] != 0.0))) && (!(scratch.values[2558] != 0.0))) && (!(scratch.values[2563] != 0.0))) && (!(scratch.values[2567] != 0.0))) {
            scratch.store_ad(692, &AdValue::sub(AdValue::scale(scratch.ad_value(670), 2.0), scratch.ad_value(654)));
        }

        if ((((scratch.values[2458] != 0.0) && (!(scratch.values[2459] != 0.0))) && (!(scratch.values[2558] != 0.0))) && (!(scratch.values[2563] != 0.0))) {
            scratch.store_ad(693, &AdValue::scale(AdValue::div(AdValue::mul(scratch.ad_value(635), scratch.ad_value(692)), scratch.ad_value(688)), (1.772453850905516 * 0.5)));
        }

        if ((((scratch.values[2458] != 0.0) && (!(scratch.values[2459] != 0.0))) && (!(scratch.values[2558] != 0.0))) && (!(scratch.values[2563] != 0.0))) {
            scratch.store_ad(679, &AdValue::mul(scratch.ad_value(564), AdValue::mul(AdValue::mul(scratch.ad_value(678), scratch.ad_value(693)), scratch.ad_value(687))));
        }

        scratch.values[2569] = if (scratch.values[570] == 0.0) { 1.0 } else { 0.0 };

        if ((((scratch.values[2458] != 0.0) && (!(scratch.values[2459] != 0.0))) && (!(scratch.values[2558] != 0.0))) && (scratch.values[2569] != 0.0)) {
            scratch.values[694] = 0.0;
            scratch.node_derivatives[694] = [0.0; Instance::NODE_COUNT];
            scratch.branch_derivatives[694] = [0.0; Instance::BRANCH_COUNT];
        }

        scratch.values[2570] = if (scratch.values[550] == 0.5) { 1.0 } else { 0.0 };

    }

    pub(super) fn stamp_transient_block_42(
        &mut self,
        ctx: &GeneratedEvalContext<'_>,
        stamper: &mut GeneratedStamper<'_>,
        scratch: &mut Scratch,
    ) {
        let _ = stamper;
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

        if ((((scratch.values[2458] != 0.0) && (!(scratch.values[2459] != 0.0))) && (!(scratch.values[2558] != 0.0))) && (!(scratch.values[2569] != 0.0))) {
            scratch.store_ad(694, &AdValue::mul(scratch.ad_value(570), AdValue::mul(AdValue::mul(AdValue::mul(scratch.ad_value(855), scratch.ad_value(695)), scratch.ad_value(695)), scratch.ad_value(670))));
        }

        scratch.values[2573] = if (scratch.values[579] > 1000.0) { 1.0 } else { 0.0 };

        if ((((scratch.values[2458] != 0.0) && (!(scratch.values[2459] != 0.0))) && (!(scratch.values[2558] != 0.0))) && (scratch.values[2573] != 0.0)) {
            scratch.values[696] = 1.0;
            scratch.node_derivatives[696] = [0.0; Instance::NODE_COUNT];
            scratch.branch_derivatives[696] = [0.0; Instance::BRANCH_COUNT];
        }

        scratch.values[2574] = if (scratch.values[669] > ((-scratch.values[500]) * scratch.values[579])) { 1.0 } else { 0.0 };

        scratch.values[2575] = if (scratch.values[582] == 4.0) { 1.0 } else { 0.0 };

        if ((((((scratch.values[2458] != 0.0) && (!(scratch.values[2459] != 0.0))) && (!(scratch.values[2558] != 0.0))) && (!(scratch.values[2573] != 0.0))) && (scratch.values[2574] != 0.0)) && (scratch.values[2575] != 0.0)) {
            scratch.store_ad(670, &AdValue::mul(AdValue::mul(AdValue::mul(AdValue::mul(scratch.ad_value(669), scratch.ad_value(647)), AdValue::mul(scratch.ad_value(669), scratch.ad_value(647))), AdValue::mul(scratch.ad_value(669), scratch.ad_value(647))), AdValue::mul(scratch.ad_value(669), scratch.ad_value(647))));
        }

        if ((((((scratch.values[2458] != 0.0) && (!(scratch.values[2459] != 0.0))) && (!(scratch.values[2558] != 0.0))) && (!(scratch.values[2573] != 0.0))) && (scratch.values[2574] != 0.0)) && (!(scratch.values[2575] != 0.0))) {
            scratch.store_ad(670, &AdValue::pow(AdValue::abs(AdValue::mul(scratch.ad_value(669), scratch.ad_value(647))), scratch.ad_value(582)));
        }

        if (((((scratch.values[2458] != 0.0) && (!(scratch.values[2459] != 0.0))) && (!(scratch.values[2558] != 0.0))) && (!(scratch.values[2573] != 0.0))) && (scratch.values[2574] != 0.0)) {
            scratch.store_ad(696, &AdValue::div_from_scalar(1.0, AdValue::sub_from_scalar(1.0, scratch.ad_value(670))));
        }

        if (((((scratch.values[2458] != 0.0) && (!(scratch.values[2459] != 0.0))) && (!(scratch.values[2558] != 0.0))) && (!(scratch.values[2573] != 0.0))) && (!(scratch.values[2574] != 0.0))) {
            scratch.store_ad(696, &AdValue::add(scratch.ad_value(644), AdValue::mul(AdValue::add(scratch.ad_value(669), AdValue::scale(scratch.ad_value(579), scratch.values[500])), scratch.ad_value(650))));
        }

        if (((scratch.values[2458] != 0.0) && (!(scratch.values[2459] != 0.0))) && (!(scratch.values[2558] != 0.0))) {
            scratch.store_ad(2036, &AdValue::mul(AdValue::add(AdValue::add(AdValue::add(scratch.ad_value(671), scratch.ad_value(672)), scratch.ad_value(679)), scratch.ad_value(694)), scratch.ad_value(696)));
        }

        scratch.values[2576] = if (scratch.values[726] == 0.0) { 1.0 } else { 0.0 };

        if (((scratch.values[2458] != 0.0) && (!(scratch.values[2459] != 0.0))) && (scratch.values[2576] != 0.0)) {
            scratch.values[2037] = 0.0;
            scratch.node_derivatives[2037] = [0.0; Instance::NODE_COUNT];
            scratch.branch_derivatives[2037] = [0.0; Instance::BRANCH_COUNT];
        }

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

        if (((scratch.values[2458] != 0.0) && (!(scratch.values[2459] != 0.0))) && (!(scratch.values[2576] != 0.0))) {
            scratch.store_ad(671, &AdValue::mul(scratch.ad_value(597), scratch.ad_value(661)));
        }

        scratch.values[2578] = if ((scratch.values[562] == 0.0) && (scratch.values[565] == 0.0)) { 1.0 } else { 0.0 };

        if ((((scratch.values[2458] != 0.0) && (!(scratch.values[2459] != 0.0))) && (!(scratch.values[2576] != 0.0))) && (scratch.values[2578] != 0.0)) {
            scratch.values[672] = 0.0;
            scratch.node_derivatives[672] = [0.0; Instance::NODE_COUNT];
            scratch.branch_derivatives[672] = [0.0; Instance::BRANCH_COUNT];
        }

        if ((((scratch.values[2458] != 0.0) && (!(scratch.values[2459] != 0.0))) && (!(scratch.values[2576] != 0.0))) && (!(scratch.values[2578] != 0.0))) {
            scratch.store_ad(673, &AdValue::sub(scratch.ad_value(603), scratch.ad_value(667)));
        }

        if ((((scratch.values[2458] != 0.0) && (!(scratch.values[2459] != 0.0))) && (!(scratch.values[2576] != 0.0))) && (!(scratch.values[2578] != 0.0))) {
            scratch.store_ad(674, &AdValue::sub_from_scalar(1.0, AdValue::sqrt(AdValue::sub_from_scalar(1.0, AdValue::div(scratch.ad_value(665), scratch.ad_value(673))))));
        }

        scratch.values[2579] = if (scratch.values[551] == 0.5) { 1.0 } else { 0.0 };

        if (((((scratch.values[2458] != 0.0) && (!(scratch.values[2459] != 0.0))) && (!(scratch.values[2576] != 0.0))) && (!(scratch.values[2578] != 0.0))) && (scratch.values[2579] != 0.0)) {
            scratch.values[675] = 0.0;
            scratch.node_derivatives[675] = [0.0; Instance::NODE_COUNT];
            scratch.branch_derivatives[675] = [0.0; Instance::BRANCH_COUNT];
        }

        if (((((scratch.values[2458] != 0.0) && (!(scratch.values[2459] != 0.0))) && (!(scratch.values[2576] != 0.0))) && (!(scratch.values[2578] != 0.0))) && (!(scratch.values[2579] != 0.0))) {
            scratch.store_ad(675, &AdValue::mul(AdValue::add(AdValue::div(AdValue::mul(AdValue::square(scratch.ad_value(674)), AdValue::ln(scratch.ad_value(674))), AdValue::sub_from_scalar(1.0, scratch.ad_value(674))), scratch.ad_value(674)), AdValue::sub_from_scalar(1.0, AdValue::scale(scratch.ad_value(551), 2.0))));
        }

        if ((((scratch.values[2458] != 0.0) && (!(scratch.values[2459] != 0.0))) && (!(scratch.values[2576] != 0.0))) && (!(scratch.values[2578] != 0.0))) {
            scratch.store_ad(676, &AdValue::add(scratch.ad_value(674), scratch.ad_value(675)));
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

        if ((((scratch.values[2458] != 0.0) && (!(scratch.values[2459] != 0.0))) && (!(scratch.values[2576] != 0.0))) && (!(scratch.values[2578] != 0.0))) {
            scratch.store_ad(678, &AdValue::mul(scratch.ad_value(594), AdValue::mul(AdValue::offset(scratch.ad_value(664), (-1.0)), scratch.ad_value(677))));
        }

        if ((((scratch.values[2458] != 0.0) && (!(scratch.values[2459] != 0.0))) && (!(scratch.values[2576] != 0.0))) && (!(scratch.values[2578] != 0.0))) {
            scratch.store_ad(672, &AdValue::mul(scratch.ad_value(562), AdValue::mul(scratch.ad_value(678), scratch.ad_value(676))));
        }

        scratch.values[2581] = if (scratch.values[565] == 0.0) { 1.0 } else { 0.0 };

        if ((((scratch.values[2458] != 0.0) && (!(scratch.values[2459] != 0.0))) && (!(scratch.values[2576] != 0.0))) && (scratch.values[2581] != 0.0)) {
            scratch.values[679] = 0.0;
            scratch.node_derivatives[679] = [0.0; Instance::NODE_COUNT];
            scratch.branch_derivatives[679] = [0.0; Instance::BRANCH_COUNT];
        }

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

        scratch.values[2582] = if (((-scratch.values[551]) * scratch.values[612]) == (-1.0)) { 1.0 } else { 0.0 };

        if (((((scratch.values[2458] != 0.0) && (!(scratch.values[2459] != 0.0))) && (!(scratch.values[2576] != 0.0))) && (!(scratch.values[2581] != 0.0))) && (scratch.values[2582] != 0.0)) {
            scratch.store_ad(686, &AdValue::div_from_scalar(1.0, AdValue::offset(AdValue::mul(scratch.ad_value(680), scratch.ad_value(685)), 1.0)));
        }

        if (((((scratch.values[2458] != 0.0) && (!(scratch.values[2459] != 0.0))) && (!(scratch.values[2576] != 0.0))) && (!(scratch.values[2581] != 0.0))) && (!(scratch.values[2582] != 0.0))) {
            scratch.store_ad(686, &AdValue::pow(AdValue::offset(AdValue::mul(scratch.ad_value(680), scratch.ad_value(685)), 1.0), AdValue::mul(AdValue::neg(scratch.ad_value(551)), scratch.ad_value(612))));
        }

        if ((((scratch.values[2458] != 0.0) && (!(scratch.values[2459] != 0.0))) && (!(scratch.values[2576] != 0.0))) && (!(scratch.values[2581] != 0.0))) {
            scratch.store_ad(687, &AdValue::div(AdValue::mul(scratch.ad_value(676), scratch.ad_value(686)), AdValue::add(scratch.ad_value(676), scratch.ad_value(686))));
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

        scratch.values[2583] = if (scratch.values[691] > 0.0) { 1.0 } else { 0.0 };

        if (((((scratch.values[2458] != 0.0) && (!(scratch.values[2459] != 0.0))) && (!(scratch.values[2576] != 0.0))) && (!(scratch.values[2581] != 0.0))) && (scratch.values[2583] != 0.0)) {
            scratch.store_ad(653, &AdValue::div_from_scalar(1.0, AdValue::offset(AdValue::scale(scratch.ad_value(691), scratch.values[428]), 1.0)));
        }

        if (((((scratch.values[2458] != 0.0) && (!(scratch.values[2459] != 0.0))) && (!(scratch.values[2576] != 0.0))) && (!(scratch.values[2581] != 0.0))) && (!(scratch.values[2583] != 0.0))) {
            scratch.store_ad(653, &AdValue::div_from_scalar(1.0, AdValue::sub_from_scalar(1.0, AdValue::scale(scratch.ad_value(691), scratch.values[428]))));
        }

        scratch.values[2584] = if (((-scratch.values[652]) + scratch.values[690]) > (-230.25850929940458)) { 1.0 } else { 0.0 };

        if (((((scratch.values[2458] != 0.0) && (!(scratch.values[2459] != 0.0))) && (!(scratch.values[2576] != 0.0))) && (!(scratch.values[2581] != 0.0))) && (scratch.values[2584] != 0.0)) {
            scratch.store_ad(670, &AdValue::exp(AdValue::sub(scratch.ad_value(690), scratch.ad_value(652))));
        }

        if (((((scratch.values[2458] != 0.0) && (!(scratch.values[2459] != 0.0))) && (!(scratch.values[2576] != 0.0))) && (!(scratch.values[2581] != 0.0))) && (!(scratch.values[2584] != 0.0))) {
            scratch.store_ad(670, &AdValue::div_from_scalar(1e-100, AdValue::offset(AdValue::mul(AdValue::sub_from_scalar((-230.25850929940458), AdValue::sub(scratch.ad_value(690), scratch.ad_value(652))), AdValue::offset(AdValue::scale(AdValue::mul(AdValue::sub_from_scalar((-230.25850929940458), AdValue::sub(scratch.ad_value(690), scratch.ad_value(652))), AdValue::offset(AdValue::scale(AdValue::sub_from_scalar((-230.25850929940458), AdValue::sub(scratch.ad_value(690), scratch.ad_value(652))), 0.3333333333333333), 1.0)), 0.5), 1.0)), 1.0)));
        }

        if ((((scratch.values[2458] != 0.0) && (!(scratch.values[2459] != 0.0))) && (!(scratch.values[2576] != 0.0))) && (!(scratch.values[2581] != 0.0))) {
            scratch.store_ad(654, &AdValue::mul(AdValue::add(AdValue::add(AdValue::scale(scratch.ad_value(653), 0.29214664), AdValue::scale(AdValue::square(scratch.ad_value(653)), scratch.values[429])), AdValue::scale(AdValue::mul(AdValue::square(scratch.ad_value(653)), scratch.ad_value(653)), scratch.values[430])), scratch.ad_value(670)));
        }

        scratch.values[2585] = if (scratch.values[691] > 0.0) { 1.0 } else { 0.0 };

        if (((((scratch.values[2458] != 0.0) && (!(scratch.values[2459] != 0.0))) && (!(scratch.values[2576] != 0.0))) && (!(scratch.values[2581] != 0.0))) && (scratch.values[2585] != 0.0)) {
            scratch.values[692] = scratch.values[654];
            scratch.node_derivatives[692] = scratch.node_derivatives[654];
            scratch.branch_derivatives[692] = scratch.branch_derivatives[654];
        }

        scratch.values[2586] = if (scratch.values[690] > (-230.25850929940458)) { 1.0 } else { 0.0 };

        if ((((((scratch.values[2458] != 0.0) && (!(scratch.values[2459] != 0.0))) && (!(scratch.values[2576] != 0.0))) && (!(scratch.values[2581] != 0.0))) && (!(scratch.values[2585] != 0.0))) && (scratch.values[2586] != 0.0)) {
            scratch.store_ad(670, &AdValue::exp(scratch.ad_value(690)));
        }

        if ((((((scratch.values[2458] != 0.0) && (!(scratch.values[2459] != 0.0))) && (!(scratch.values[2576] != 0.0))) && (!(scratch.values[2581] != 0.0))) && (!(scratch.values[2585] != 0.0))) && (!(scratch.values[2586] != 0.0))) {
            scratch.store_ad(670, &AdValue::div_from_scalar(1e-100, AdValue::offset(AdValue::mul(AdValue::sub_from_scalar((-230.25850929940458), scratch.ad_value(690)), AdValue::offset(AdValue::scale(AdValue::mul(AdValue::sub_from_scalar((-230.25850929940458), scratch.ad_value(690)), AdValue::offset(AdValue::scale(AdValue::sub_from_scalar((-230.25850929940458), scratch.ad_value(690)), 0.3333333333333333), 1.0)), 0.5), 1.0)), 1.0)));
        }

        if (((((scratch.values[2458] != 0.0) && (!(scratch.values[2459] != 0.0))) && (!(scratch.values[2576] != 0.0))) && (!(scratch.values[2581] != 0.0))) && (!(scratch.values[2585] != 0.0))) {
            scratch.store_ad(692, &AdValue::sub(AdValue::scale(scratch.ad_value(670), 2.0), scratch.ad_value(654)));
        }

        if ((((scratch.values[2458] != 0.0) && (!(scratch.values[2459] != 0.0))) && (!(scratch.values[2576] != 0.0))) && (!(scratch.values[2581] != 0.0))) {
            scratch.store_ad(693, &AdValue::scale(AdValue::div(AdValue::mul(scratch.ad_value(636), scratch.ad_value(692)), scratch.ad_value(688)), (1.772453850905516 * 0.5)));
        }

        if ((((scratch.values[2458] != 0.0) && (!(scratch.values[2459] != 0.0))) && (!(scratch.values[2576] != 0.0))) && (!(scratch.values[2581] != 0.0))) {
            scratch.store_ad(679, &AdValue::mul(scratch.ad_value(565), AdValue::mul(AdValue::mul(scratch.ad_value(678), scratch.ad_value(693)), scratch.ad_value(687))));
        }

        scratch.values[2587] = if (scratch.values[571] == 0.0) { 1.0 } else { 0.0 };

        if ((((scratch.values[2458] != 0.0) && (!(scratch.values[2459] != 0.0))) && (!(scratch.values[2576] != 0.0))) && (scratch.values[2587] != 0.0)) {
            scratch.values[694] = 0.0;
            scratch.node_derivatives[694] = [0.0; Instance::NODE_COUNT];
            scratch.branch_derivatives[694] = [0.0; Instance::BRANCH_COUNT];
        }

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

        if ((((scratch.values[2458] != 0.0) && (!(scratch.values[2459] != 0.0))) && (!(scratch.values[2576] != 0.0))) && (!(scratch.values[2587] != 0.0))) {
            scratch.store_ad(694, &AdValue::mul(scratch.ad_value(571), AdValue::mul(AdValue::mul(AdValue::mul(scratch.ad_value(855), scratch.ad_value(695)), scratch.ad_value(695)), scratch.ad_value(670))));
        }

        scratch.values[2591] = if (scratch.values[580] > 1000.0) { 1.0 } else { 0.0 };

        if ((((scratch.values[2458] != 0.0) && (!(scratch.values[2459] != 0.0))) && (!(scratch.values[2576] != 0.0))) && (scratch.values[2591] != 0.0)) {
            scratch.values[696] = 1.0;
            scratch.node_derivatives[696] = [0.0; Instance::NODE_COUNT];
            scratch.branch_derivatives[696] = [0.0; Instance::BRANCH_COUNT];
        }

        scratch.values[2592] = if (scratch.values[669] > ((-scratch.values[500]) * scratch.values[580])) { 1.0 } else { 0.0 };

        scratch.values[2593] = if (scratch.values[583] == 4.0) { 1.0 } else { 0.0 };

        if ((((((scratch.values[2458] != 0.0) && (!(scratch.values[2459] != 0.0))) && (!(scratch.values[2576] != 0.0))) && (!(scratch.values[2591] != 0.0))) && (scratch.values[2592] != 0.0)) && (scratch.values[2593] != 0.0)) {
            scratch.store_ad(670, &AdValue::mul(AdValue::mul(AdValue::mul(AdValue::mul(scratch.ad_value(669), scratch.ad_value(648)), AdValue::mul(scratch.ad_value(669), scratch.ad_value(648))), AdValue::mul(scratch.ad_value(669), scratch.ad_value(648))), AdValue::mul(scratch.ad_value(669), scratch.ad_value(648))));
        }

        if ((((((scratch.values[2458] != 0.0) && (!(scratch.values[2459] != 0.0))) && (!(scratch.values[2576] != 0.0))) && (!(scratch.values[2591] != 0.0))) && (scratch.values[2592] != 0.0)) && (!(scratch.values[2593] != 0.0))) {
            scratch.store_ad(670, &AdValue::pow(AdValue::abs(AdValue::mul(scratch.ad_value(669), scratch.ad_value(648))), scratch.ad_value(583)));
        }

        if (((((scratch.values[2458] != 0.0) && (!(scratch.values[2459] != 0.0))) && (!(scratch.values[2576] != 0.0))) && (!(scratch.values[2591] != 0.0))) && (scratch.values[2592] != 0.0)) {
            scratch.store_ad(696, &AdValue::div_from_scalar(1.0, AdValue::sub_from_scalar(1.0, scratch.ad_value(670))));
        }

        if (((((scratch.values[2458] != 0.0) && (!(scratch.values[2459] != 0.0))) && (!(scratch.values[2576] != 0.0))) && (!(scratch.values[2591] != 0.0))) && (!(scratch.values[2592] != 0.0))) {
            scratch.store_ad(696, &AdValue::add(scratch.ad_value(645), AdValue::mul(AdValue::add(scratch.ad_value(669), AdValue::scale(scratch.ad_value(580), scratch.values[500])), scratch.ad_value(651))));
        }

        if (((scratch.values[2458] != 0.0) && (!(scratch.values[2459] != 0.0))) && (!(scratch.values[2576] != 0.0))) {
            scratch.store_ad(2037, &AdValue::mul(AdValue::add(AdValue::add(AdValue::add(scratch.ad_value(671), scratch.ad_value(672)), scratch.ad_value(679)), scratch.ad_value(694)), scratch.ad_value(696)));
        }

        if ((scratch.values[2458] != 0.0) && (!(scratch.values[2459] != 0.0))) {
            scratch.store_ad(2034, &AdValue::add(AdValue::add(AdValue::mul(scratch.ad_value(724), scratch.ad_value(2035)), AdValue::mul(scratch.ad_value(725), scratch.ad_value(2036))), AdValue::mul(scratch.ad_value(726), scratch.ad_value(2037))));
        }

        scratch.store_ad(949, &AdValue::mul(scratch.ad_value(2077), scratch.ad_value(842)));

        scratch.store_ad(950, &AdValue::mul(scratch.ad_value(2077), scratch.ad_value(843)));

        scratch.store_ad(951, &AdValue::mul(scratch.ad_value(2077), scratch.ad_value(844)));

        scratch.store_ad(952, &AdValue::mul(scratch.ad_value(2077), scratch.ad_value(845)));

        scratch.store_ad(955, &AdValue::mul(scratch.ad_value(2077), scratch.ad_value(846)));

        scratch.store_ad(954, &AdValue::mul(scratch.ad_value(2077), scratch.ad_value(847)));

        scratch.store_ad(953, &AdValue::mul(scratch.ad_value(2077), scratch.ad_value(848)));

        scratch.values[2594] = if (scratch.values[1999] > 0.0) { 1.0 } else { 0.0 };

        scratch.values[2595] = if (scratch.values[299] > 0.0) { 1.0 } else { 0.0 };

        scratch.values[2596] = if (scratch.values[300] > 0.0) { 1.0 } else { 0.0 };

        scratch.values[2597] = if (scratch.values[301] > 0.0) { 1.0 } else { 0.0 };

        scratch.values[2598] = if (scratch.values[302] > 0.0) { 1.0 } else { 0.0 };

        scratch.values[2599] = if (scratch.values[303] > 0.0) { 1.0 } else { 0.0 };

        scratch.values[2600] = if (scratch.values[304] > 0.0) { 1.0 } else { 0.0 };

        scratch.values[2601] = if (scratch.values[305] > 0.0) { 1.0 } else { 0.0 };

        scratch.values[2073] = 0.0;

        scratch.values[2602] = 0.0;

        scratch.values[2603] = 0.0;

        scratch.values[2604] = if (scratch.values[300] > 0.0) { 1.0 } else { 0.0 };

        if (scratch.values[2604] != 0.0) {
            scratch.store_ad(2602, &AdValue::mul(AdValue::mul(scratch.ad_value(843), AdValue::voltage(ctx, &self.nodes, Some(2), Some(7))), AdValue::voltage(ctx, &self.nodes, Some(2), Some(7))));
        }

        scratch.values[2605] = if (scratch.values[301] > 0.0) { 1.0 } else { 0.0 };

        if (scratch.values[2605] != 0.0) {
            scratch.store_ad(2603, &AdValue::mul(AdValue::mul(scratch.ad_value(844), AdValue::voltage(ctx, &self.nodes, Some(0), Some(8))), AdValue::voltage(ctx, &self.nodes, Some(0), Some(8))));
        }

        scratch.values[2606] = if (scratch.values[187] > 0.001) { 1.0 } else { 0.0 };

        if (scratch.values[2606] != 0.0) {
            scratch.store_ad(2073, &AdValue::add(AdValue::add(AdValue::add(AdValue::mul(AdValue::add(scratch.ad_value(2018), scratch.ad_value(2069)), scratch.ad_value(850)), AdValue::mul(scratch.ad_value(2019), AdValue::add(scratch.ad_value(850), scratch.ad_value(851)))), scratch.ad_value(2602)), scratch.ad_value(2603)));
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

        scratch.values[981] = 0.0;

        scratch.values[997] = 0.0;

        scratch.values[989] = 0.0;

        scratch.values[999] = 1e-40;

        scratch.values[1001] = 0.0;

        scratch.values[1004] = 0.0;

        scratch.store_ad(1002, &AdValue::mul(scratch.ad_value(2029), scratch.ad_value(2025)));

        scratch.values[998] = 0.0;

        scratch.values[1003] = 0.0;

        scratch.values[2070] = 0.0;

        scratch.values[2071] = 0.0;

        scratch.values[2072] = 0.0;

        scratch.values[2609] = if (((scratch.values[2010] > 0.0) && (scratch.values[25] > 0.0)) && (scratch.values[2075] > 0.0)) { 1.0 } else { 0.0 };

        if (scratch.values[2609] != 0.0) {
            scratch.store_ad(978, &AdValue::mul(AdValue::mul(scratch.ad_value(810), scratch.ad_value(2011)), scratch.ad_value(2074)));
        }

        if (scratch.values[2609] != 0.0) {
            scratch.store_ad(979, &AdValue::mul(scratch.ad_value(810), scratch.ad_value(2014)));
        }

        if (scratch.values[2609] != 0.0) {
            scratch.store_ad(980, &AdValue::mul(AdValue::mul(scratch.ad_value(810), scratch.ad_value(2011)), scratch.ad_value(2012)));
        }

        if (scratch.values[2609] != 0.0) {
            scratch.store_ad(981, &AdValue::mul(AdValue::add(AdValue::sub(scratch.ad_value(277), AdValue::mul(scratch.ad_value(278), scratch.ad_value(978))), AdValue::mul(scratch.ad_value(279), AdValue::square(scratch.ad_value(978)))), AdValue::ln(AdValue::div(AdValue::add(scratch.ad_value(979), AdValue::scale(scratch.ad_value(980), 0.5)), AdValue::sub(scratch.ad_value(979), AdValue::scale(scratch.ad_value(980), 0.5))))));
        }

        if (scratch.values[2609] != 0.0) {
            scratch.store_ad(981, &AdValue::add(scratch.ad_value(981), AdValue::mul(AdValue::add(scratch.ad_value(278), AdValue::mul(scratch.ad_value(279), AdValue::sub(scratch.ad_value(979), AdValue::scale(scratch.ad_value(978), 2.0)))), scratch.ad_value(980))));
        }

        if (scratch.values[2609] != 0.0) {
            scratch.store_ad(981, &AdValue::div(AdValue::mul(AdValue::mul(AdValue::mul(scratch.ad_value(2080), scratch.ad_value(2018)), scratch.ad_value(2017)), scratch.ad_value(981)), scratch.ad_value(978)));
        }

        if (scratch.values[2609] != 0.0) {
            scratch.store_ad(981, &{
                if (scratch.values[981] > 0.0) {
                    scratch.ad_value(981)
                } else {
                    AdValue::constant(0.0)
                }
            });
        }

        if (scratch.values[2609] != 0.0) {
            scratch.store_ad(2028, &AdValue::div(scratch.ad_value(2014), scratch.ad_value(2011)));
        }

        if (scratch.values[2609] != 0.0) {
            scratch.store_ad(982, &AdValue::div(scratch.ad_value(2013), scratch.ad_value(2014)));
        }

        if (scratch.values[2609] != 0.0) {
            scratch.store_ad(983, &AdValue::scale(AdValue::div(scratch.ad_value(2012), scratch.ad_value(2028)), (0.5 * 0.16666666666666666)));
        }

        if (scratch.values[2609] != 0.0) {
            scratch.store_ad(984, &AdValue::square(scratch.ad_value(983)));
        }

    }

    pub(super) fn stamp_transient_block_43(
        &mut self,
        ctx: &GeneratedEvalContext<'_>,
        stamper: &mut GeneratedStamper<'_>,
        scratch: &mut Scratch,
    ) {
        let _ = stamper;
        if (scratch.values[2609] != 0.0) {
            scratch.store_ad(985, &AdValue::offset(AdValue::div(scratch.ad_value(2028), scratch.ad_value(2015)), (-1.0)));
        }

        if (scratch.values[2609] != 0.0) {
            scratch.store_ad(986, &{
                if ((1.0 - (12.0 * (scratch.values[985] * scratch.values[984]))) > 1e-20) {
                    AdValue::sub_from_scalar(1.0, AdValue::scale(AdValue::mul(scratch.ad_value(985), scratch.ad_value(984)), 12.0))
                } else {
                    AdValue::constant(1e-20)
                }
            });
        }

        if (scratch.values[2609] != 0.0) {
            scratch.store_ad(987, &AdValue::div_from_scalar(1.0, AdValue::square(scratch.ad_value(986))));
        }

        if (scratch.values[2609] != 0.0) {
            scratch.store_ad(988, &AdValue::mul(scratch.ad_value(2075), AdValue::mul(AdValue::mul(scratch.ad_value(2016), scratch.ad_value(2014)), scratch.ad_value(2017))));
        }

        if (scratch.values[2609] != 0.0) {
            scratch.store_ad(989, &AdValue::sub(AdValue::add(scratch.ad_value(982), AdValue::scale(scratch.ad_value(984), 12.0)), AdValue::scale(AdValue::mul(AdValue::mul(AdValue::offset(scratch.ad_value(982), 1.0), scratch.ad_value(984)), scratch.ad_value(985)), 24.0)));
        }

        if (scratch.values[2609] != 0.0) {
            scratch.store_ad(989, &{
                if (scratch.values[989] > 1e-40) {
                    scratch.ad_value(989)
                } else {
                    AdValue::constant(1e-40)
                }
            });
        }

        if (scratch.values[2609] != 0.0) {
            scratch.store_ad(989, &AdValue::mul(AdValue::mul(scratch.ad_value(988), scratch.ad_value(987)), scratch.ad_value(989)));
        }

        scratch.values[2610] = if (scratch.values[276] > 0.0) { 1.0 } else { 0.0 };

        if ((scratch.values[2609] != 0.0) && (scratch.values[2610] != 0.0)) {
            scratch.store_ad(990, &AdValue::mul(scratch.ad_value(2013), scratch.ad_value(2049)));
        }

        if ((scratch.values[2609] != 0.0) && (scratch.values[2610] != 0.0)) {
            scratch.store_ad(991, &AdValue::scale(AdValue::div(scratch.ad_value(990), AdValue::offset(scratch.ad_value(990), 100.0)), 100.0));
        }

        scratch.values[2611] = if (scratch.values[242] < 0.0) { 1.0 } else { 0.0 };

        if (((scratch.values[2609] != 0.0) && (scratch.values[2610] != 0.0)) && (scratch.values[2611] != 0.0)) {
            scratch.store_ad(992, &AdValue::div_from_scalar(1.0, AdValue::sub_from_scalar(1.0, AdValue::mul(scratch.ad_value(242), scratch.ad_value(991)))));
        }

        if (((scratch.values[2609] != 0.0) && (scratch.values[2610] != 0.0)) && (!(scratch.values[2611] != 0.0))) {
            scratch.store_ad(992, &AdValue::offset(AdValue::mul(scratch.ad_value(242), scratch.ad_value(991)), 1.0));
        }

        if ((scratch.values[2609] != 0.0) && (scratch.values[2610] != 0.0)) {
            scratch.store_ad(993, &AdValue::mul(scratch.ad_value(2079), AdValue::div(scratch.ad_value(992), scratch.ad_value(2048))));
        }

        if ((scratch.values[2609] != 0.0) && (scratch.values[2610] != 0.0)) {
            scratch.store_ad(994, &AdValue::mul(AdValue::mul(AdValue::square(scratch.ad_value(993)), scratch.ad_value(2012)), scratch.ad_value(2012)));
        }

        scratch.values[2612] = if (scratch.values[0] == (-1.0)) { 1.0 } else { 0.0 };

        if (((scratch.values[2609] != 0.0) && (scratch.values[2610] != 0.0)) && (scratch.values[2612] != 0.0)) {
            scratch.store_ad(994, &AdValue::div(scratch.ad_value(994), AdValue::offset(AdValue::mul(scratch.ad_value(993), scratch.ad_value(2012)), 1.0)));
        }

        if ((scratch.values[2609] != 0.0) && (scratch.values[2610] != 0.0)) {
            scratch.store_ad(995, &AdValue::scale(AdValue::mul(scratch.ad_value(2048), AdValue::offset(AdValue::sqrt(AdValue::offset(AdValue::scale(scratch.ad_value(994), 2.0), 1.0)), 1.0)), 0.5));
        }

        if ((scratch.values[2609] != 0.0) && (scratch.values[2610] != 0.0)) {
            scratch.store_ad(996, &AdValue::div(scratch.ad_value(2048), AdValue::mul(scratch.ad_value(995), scratch.ad_value(986))));
        }

        if ((scratch.values[2609] != 0.0) && (scratch.values[2610] != 0.0)) {
            scratch.store_ad(997, &AdValue::mul(AdValue::mul(AdValue::mul(AdValue::mul(scratch.ad_value(841), scratch.ad_value(2018)), scratch.ad_value(2050)), scratch.ad_value(996)), scratch.ad_value(996)));
        }

        if ((scratch.values[2609] != 0.0) && (scratch.values[2610] != 0.0)) {
            scratch.store_ad(989, &AdValue::add(scratch.ad_value(989), AdValue::div(scratch.ad_value(997), scratch.ad_value(2077))));
        }

        if (scratch.values[2609] != 0.0) {
            scratch.store_ad(998, &AdValue::sqrt(AdValue::mul(scratch.ad_value(2078), scratch.ad_value(989))));
        }

        scratch.values[2613] = if ((scratch.values[10] == 1.0) && (scratch.values[2078] > 0.0)) { 1.0 } else { 0.0 };

        if ((scratch.values[2609] != 0.0) && (scratch.values[2613] != 0.0)) {
            scratch.store_ad(999, &AdValue::sub(AdValue::sub(AdValue::scale(scratch.ad_value(982), 0.08333333333333333), AdValue::mul(scratch.ad_value(984), AdValue::sub(AdValue::offset(scratch.ad_value(982), 0.2), AdValue::scale(scratch.ad_value(984), 12.0)))), AdValue::scale(AdValue::mul(AdValue::mul(scratch.ad_value(984), AdValue::sub(AdValue::offset(scratch.ad_value(982), 1.0), AdValue::scale(scratch.ad_value(984), 12.0))), scratch.ad_value(985)), 1.6)));
        }

        if ((scratch.values[2609] != 0.0) && (scratch.values[2613] != 0.0)) {
            scratch.store_ad(999, &{
                if (scratch.values[999] > 1e-40) {
                    scratch.ad_value(999)
                } else {
                    AdValue::constant(1e-40)
                }
            });
        }

        if ((scratch.values[2609] != 0.0) && (scratch.values[2613] != 0.0)) {
            scratch.store_ad(999, &AdValue::mul(AdValue::div(scratch.ad_value(987), scratch.ad_value(988)), scratch.ad_value(999)));
        }

        if ((scratch.values[2609] != 0.0) && (scratch.values[2613] != 0.0)) {
            scratch.store_ad(1000, &AdValue::mul(AdValue::mul(scratch.ad_value(987), scratch.ad_value(983)), AdValue::sub(AdValue::sub_from_scalar(1.0, AdValue::scale(scratch.ad_value(984), 12.0)), AdValue::mul(AdValue::sub(AdValue::add(scratch.ad_value(982), AdValue::scale(scratch.ad_value(984), 19.2)), AdValue::scale(AdValue::mul(scratch.ad_value(982), scratch.ad_value(984)), 12.0)), scratch.ad_value(985)))));
        }

        if ((scratch.values[2609] != 0.0) && (scratch.values[2613] != 0.0)) {
            scratch.store_ad(1002, &AdValue::div(AdValue::mul(AdValue::mul(AdValue::square(scratch.ad_value(2026)), scratch.ad_value(2029)), scratch.ad_value(2025)), AdValue::square(scratch.ad_value(2027))));
        }

        scratch.values[2614] = if (scratch.values[276] > 0.0) { 1.0 } else { 0.0 };

        if (((scratch.values[2609] != 0.0) && (scratch.values[2613] != 0.0)) && (scratch.values[2614] != 0.0)) {
            scratch.store_ad(999, &AdValue::add(scratch.ad_value(999), AdValue::div(AdValue::mul(scratch.ad_value(997), AdValue::offset(AdValue::scale(scratch.ad_value(984), 12.0), 1.0)), AdValue::mul(AdValue::mul(AdValue::scale(scratch.ad_value(988), 12.0), scratch.ad_value(988)), scratch.ad_value(2077)))));
        }

        if (((scratch.values[2609] != 0.0) && (scratch.values[2613] != 0.0)) && (scratch.values[2614] != 0.0)) {
            scratch.store_ad(1000, &AdValue::sub(scratch.ad_value(1000), AdValue::div(AdValue::mul(AdValue::mul(scratch.ad_value(997), scratch.ad_value(983)), AdValue::offset(scratch.ad_value(985), 1.0)), AdValue::mul(scratch.ad_value(988), scratch.ad_value(2077)))));
        }

        if ((scratch.values[2609] != 0.0) && (scratch.values[2613] != 0.0)) {
            scratch.store_ad(1003, &AdValue::sqrt(AdValue::div(scratch.ad_value(2078), scratch.ad_value(999))));
        }

        scratch.values[2615] = if (scratch.values[998] == 0.0) { 1.0 } else { 0.0 };

        if (((scratch.values[2609] != 0.0) && (scratch.values[2613] != 0.0)) && (scratch.values[2615] != 0.0)) {
            scratch.values[1004] = 0.0;
            scratch.node_derivatives[1004] = [0.0; Instance::NODE_COUNT];
            scratch.branch_derivatives[1004] = [0.0; Instance::BRANCH_COUNT];
        }

        if (((scratch.values[2609] != 0.0) && (scratch.values[2613] != 0.0)) && (!(scratch.values[2615] != 0.0))) {
            scratch.store_ad(1004, &AdValue::div(AdValue::mul(scratch.ad_value(1000), scratch.ad_value(1003)), scratch.ad_value(998)));
        }

        if ((scratch.values[2609] != 0.0) && (scratch.values[2613] != 0.0)) {
            scratch.store_ad(1004, &{
                if (scratch.values[1004] > 0.0) {
                    {
                        if (scratch.values[1004] < 1.0) {
                            scratch.ad_value(1004)
                        } else {
                            AdValue::constant(1.0)
                        }
                    }
                } else {
                    AdValue::constant(0.0)
                }
            });
        }

        if ((scratch.values[2609] != 0.0) && (scratch.values[2613] != 0.0)) {
            scratch.store_ad(1001, &AdValue::div(AdValue::mul(scratch.ad_value(1004), scratch.ad_value(998)), scratch.ad_value(1003)));
        }

        scratch.store_ad(1005, &AdValue::scale(AdValue::abs(scratch.ad_value(2024)), (2.0 * 1.6021918e-19)));

        scratch.store_ad(1006, &AdValue::scale(AdValue::abs(scratch.ad_value(2023)), (2.0 * 1.6021918e-19)));

        scratch.store_ad(1007, &AdValue::scale(AdValue::abs(scratch.ad_value(2022)), (2.0 * 1.6021918e-19)));

        scratch.store_ad(1008, &AdValue::scale(AdValue::abs(scratch.ad_value(2021)), (2.0 * 1.6021918e-19)));

        scratch.store_ad(1009, &AdValue::scale(AdValue::mul(AdValue::offset(scratch.ad_value(2020), 1.0), AdValue::abs(scratch.ad_value(2019))), (2.0 * 1.6021918e-19)));

        scratch.store_ad(1010, &AdValue::scale(AdValue::abs(scratch.ad_value(2030)), (2.0 * 1.6021918e-19)));

        scratch.store_ad(1011, &AdValue::scale(AdValue::abs(scratch.ad_value(2034)), (2.0 * 1.6021918e-19)));

        scratch.values[2616] = if (scratch.values[1999] > 0.0) { 1.0 } else { 0.0 };

        if (scratch.values[2616] != 0.0) {
            scratch.store_ad(1012, &AdValue::add(scratch.ad_value(1005), scratch.ad_value(1007)));
        }

        if (scratch.values[2616] != 0.0) {
            scratch.store_ad(1013, &AdValue::add(scratch.ad_value(1006), scratch.ad_value(1008)));
        }

        if (scratch.values[2616] != 0.0) {
            scratch.values[2046] = scratch.values[1010];
            scratch.node_derivatives[2046] = scratch.node_derivatives[1010];
            scratch.branch_derivatives[2046] = scratch.branch_derivatives[1010];
        }

        if (scratch.values[2616] != 0.0) {
            scratch.store_ad(2047, &AdValue::add(scratch.ad_value(1011), scratch.ad_value(1009)));
        }

        if (!(scratch.values[2616] != 0.0)) {
            scratch.store_ad(1012, &AdValue::add(scratch.ad_value(1006), scratch.ad_value(1007)));
        }

        if (!(scratch.values[2616] != 0.0)) {
            scratch.store_ad(1013, &AdValue::add(scratch.ad_value(1005), scratch.ad_value(1008)));
        }

        if (!(scratch.values[2616] != 0.0)) {
            scratch.store_ad(2046, &AdValue::add(scratch.ad_value(1010), scratch.ad_value(1009)));
        }

        if (!(scratch.values[2616] != 0.0)) {
            scratch.values[2047] = scratch.values[1011];
            scratch.node_derivatives[2047] = scratch.node_derivatives[1011];
            scratch.branch_derivatives[2047] = scratch.branch_derivatives[1011];
        }

        scratch.values[2617] = if (((scratch.values[8] != 0.0) && (scratch.values[286] > 0.0)) && (scratch.values[2058] > 0.0)) { 1.0 } else { 0.0 };

        if (scratch.values[2617] != 0.0) {
            scratch.store_ad(2087, &AdValue::div(AdValue::scale(scratch.ad_value(2067), 4.0), scratch.ad_value(2082)));
        }

        if (scratch.values[2617] != 0.0) {
            scratch.store_ad(1014, &AdValue::div(AdValue::sqrt(AdValue::offset(scratch.ad_value(2087), 1.0)), AdValue::offset(AdValue::sqrt(AdValue::offset(scratch.ad_value(2087), 1.1)), (-1.0))));
        }

        if (scratch.values[2617] != 0.0) {
            scratch.store_ad(2087, &AdValue::mul(scratch.ad_value(810), scratch.ad_value(2074)));
        }

        if (scratch.values[2617] != 0.0) {
            scratch.store_ad(1015, &AdValue::mul(scratch.ad_value(2087), scratch.ad_value(1014)));
        }

        if (scratch.values[2617] != 0.0) {
            scratch.store_ad(1016, &AdValue::mul(scratch.ad_value(2087), AdValue::add(scratch.ad_value(2066), scratch.ad_value(1014))));
        }

        if (scratch.values[2617] != 0.0) {
            scratch.store_ad(1017, &AdValue::mul(AdValue::mul(AdValue::mul(AdValue::neg(scratch.ad_value(2087)), scratch.ad_value(1014)), scratch.ad_value(2068)), scratch.ad_value(2064)));
        }

        if (scratch.values[2617] != 0.0) {
            scratch.store_ad(2070, &AdValue::mul(AdValue::sub(scratch.ad_value(295), AdValue::mul(AdValue::sub(scratch.ad_value(296), AdValue::mul(scratch.ad_value(297), scratch.ad_value(1015))), scratch.ad_value(1015))), AdValue::ln(AdValue::div(AdValue::add(scratch.ad_value(1016), AdValue::scale(scratch.ad_value(1017), 0.5)), AdValue::sub(scratch.ad_value(1016), AdValue::scale(scratch.ad_value(1017), 0.5))))));
        }

        if (scratch.values[2617] != 0.0) {
            scratch.store_ad(2070, &AdValue::add(scratch.ad_value(2070), AdValue::mul(AdValue::add(scratch.ad_value(296), AdValue::mul(scratch.ad_value(297), AdValue::sub(scratch.ad_value(1016), AdValue::scale(scratch.ad_value(1015), 2.0)))), scratch.ad_value(1017))));
        }

        if (scratch.values[2617] != 0.0) {
            scratch.store_ad(2070, &AdValue::div(AdValue::mul(AdValue::mul(AdValue::mul(scratch.ad_value(2084), scratch.ad_value(2069)), scratch.ad_value(2017)), scratch.ad_value(2070)), scratch.ad_value(1015)));
        }

        if (scratch.values[2617] != 0.0) {
            scratch.store_ad(2070, &{
                if (scratch.values[2070] > 0.0) {
                    scratch.ad_value(2070)
                } else {
                    AdValue::constant(0.0)
                }
            });
        }

        if (scratch.values[2617] != 0.0) {
            scratch.store_ad(1018, &AdValue::div(AdValue::mul(scratch.ad_value(2074), AdValue::add(scratch.ad_value(2066), scratch.ad_value(1014))), scratch.ad_value(1014)));
        }

        if (scratch.values[2617] != 0.0) {
            scratch.store_ad(1019, &AdValue::div(AdValue::mul(AdValue::div(scratch.ad_value(2008), scratch.ad_value(2074)), scratch.ad_value(2066)), AdValue::add(scratch.ad_value(2066), scratch.ad_value(1014))));
        }

        if (scratch.values[2617] != 0.0) {
            scratch.store_ad(1020, &AdValue::div(AdValue::mul(AdValue::mul(AdValue::scale(scratch.ad_value(2074), ((-0.5) * 0.16666666666666666)), scratch.ad_value(2068)), scratch.ad_value(2064)), scratch.ad_value(1018)));
        }

        if (scratch.values[2617] != 0.0) {
            scratch.store_ad(1021, &AdValue::square(scratch.ad_value(1020)));
        }

        if (scratch.values[2617] != 0.0) {
            scratch.values[1022] = 0.0;
            scratch.node_derivatives[1022] = [0.0; Instance::NODE_COUNT];
            scratch.branch_derivatives[1022] = [0.0; Instance::BRANCH_COUNT];
        }

        if (scratch.values[2617] != 0.0) {
            scratch.store_ad(2087, &AdValue::mul(scratch.ad_value(2011), scratch.ad_value(2015)));
        }

        scratch.values[2618] = if (scratch.values[2087] > 1e-10) { 1.0 } else { 0.0 };

        if ((scratch.values[2617] != 0.0) && (scratch.values[2618] != 0.0)) {
            scratch.store_ad(1022, &AdValue::offset(AdValue::div(AdValue::mul(scratch.ad_value(1014), scratch.ad_value(1018)), scratch.ad_value(2087)), (-1.0)));
        }

        if (scratch.values[2617] != 0.0) {
            scratch.store_ad(1023, &{
                if ((1.0 - (12.0 * (scratch.values[1022] * scratch.values[1021]))) > 1e-20) {
                    AdValue::sub_from_scalar(1.0, AdValue::scale(AdValue::mul(scratch.ad_value(1022), scratch.ad_value(1021)), 12.0))
                } else {
                    AdValue::constant(1e-20)
                }
            });
        }

        if (scratch.values[2617] != 0.0) {
            scratch.store_ad(1024, &AdValue::div_from_scalar(1.0, AdValue::square(scratch.ad_value(1023))));
        }

        if (scratch.values[2617] != 0.0) {
            scratch.store_ad(1025, &AdValue::mul(AdValue::mul(AdValue::mul(AdValue::mul(scratch.ad_value(2076), scratch.ad_value(2074)), AdValue::add(scratch.ad_value(2066), scratch.ad_value(1014))), scratch.ad_value(2016)), scratch.ad_value(2017)));
        }

        if (scratch.values[2617] != 0.0) {
            scratch.store_ad(2071, &AdValue::sub(AdValue::add(scratch.ad_value(1019), AdValue::scale(scratch.ad_value(1021), 12.0)), AdValue::scale(AdValue::mul(AdValue::mul(AdValue::offset(scratch.ad_value(1019), 1.0), scratch.ad_value(1021)), scratch.ad_value(1022)), 24.0)));
        }

        if (scratch.values[2617] != 0.0) {
            scratch.store_ad(2071, &{
                if (scratch.values[2071] > 1e-40) {
                    scratch.ad_value(2071)
                } else {
                    AdValue::constant(1e-40)
                }
            });
        }

        if (scratch.values[2617] != 0.0) {
            scratch.store_ad(2071, &AdValue::mul(AdValue::mul(scratch.ad_value(1025), scratch.ad_value(1024)), scratch.ad_value(2071)));
        }

        if (scratch.values[2617] != 0.0) {
            scratch.store_ad(2072, &AdValue::sqrt(AdValue::mul(scratch.ad_value(2085), scratch.ad_value(2071))));
        }

    }
}
