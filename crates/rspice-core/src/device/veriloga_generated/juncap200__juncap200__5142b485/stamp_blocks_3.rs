#![allow(dead_code, unused_assignments, unused_imports, unused_parens, unused_variables)]

use super::{AdValue, GeneratedEvalContext, GeneratedReactiveStamper, GeneratedStamper, ReactiveScratch, Scratch, THERMAL_VOLTAGE_PER_K};
use super::super::state::Instance;

impl Instance {

    pub(super) fn stamp_transient_block_12(
        &mut self,
        ctx: &GeneratedEvalContext<'_>,
        stamper: &mut GeneratedStamper<'_>,
        scratch: &mut Scratch,
    ) {
        let _ = stamper;
        if (((!(scratch.values[637] != 0.0)) && (!(scratch.values[652] != 0.0))) && (!(scratch.values[657] != 0.0))) {
            scratch.store_ad(199, &AdValue::scale(AdValue::mul(AdValue::mul(scratch.ad_value(198), scratch.ad_value(213)), scratch.ad_value(207)), scratch.values[25]));
        }

        scratch.values[663] = if (scratch.values[31] == 0.0) { 1.0 } else { 0.0 };

        if (((!(scratch.values[637] != 0.0)) && (!(scratch.values[652] != 0.0))) && (scratch.values[663] != 0.0)) {
            scratch.values[214] = 0.0;
            scratch.node_derivatives[214] = [0.0; Instance::NODE_COUNT];
            scratch.branch_derivatives[214] = [0.0; Instance::BRANCH_COUNT];
        }

        scratch.values[664] = if (scratch.values[11] == 0.5) { 1.0 } else { 0.0 };

        if ((((!(scratch.values[637] != 0.0)) && (!(scratch.values[652] != 0.0))) && (!(scratch.values[663] != 0.0))) && (scratch.values[664] != 0.0)) {
            scratch.store_ad(190, &AdValue::sqrt(AdValue::scale(AdValue::sub_from_scalar(scratch.values[8], scratch.ad_value(188)), scratch.values[114])));
        }

        if ((((!(scratch.values[637] != 0.0)) && (!(scratch.values[652] != 0.0))) && (!(scratch.values[663] != 0.0))) && (!(scratch.values[664] != 0.0))) {
            scratch.store_ad(190, &AdValue::powf(AdValue::scale(AdValue::sub_from_scalar(scratch.values[8], scratch.ad_value(188)), scratch.values[114]), scratch.values[11]));
        }

        if (((!(scratch.values[637] != 0.0)) && (!(scratch.values[652] != 0.0))) && (!(scratch.values[663] != 0.0))) {
            scratch.store_ad(215, &AdValue::scale(AdValue::div(AdValue::scale(AdValue::sub_from_scalar(scratch.values[8], scratch.ad_value(188)), scratch.values[111]), scratch.ad_value(190)), scratch.values[96]));
        }

        scratch.values[665] = if (((((-scratch.values[126]) / scratch.values[215])) as f64).abs() < 230.25850929940458) { 1.0 } else { 0.0 };

        if ((((!(scratch.values[637] != 0.0)) && (!(scratch.values[652] != 0.0))) && (!(scratch.values[663] != 0.0))) && (scratch.values[665] != 0.0)) {
            scratch.store_ad(190, &AdValue::exp(AdValue::div(AdValue::neg(scratch.ad_value(126)), scratch.ad_value(215))));
        }

        scratch.values[666] = if (((-scratch.values[126]) / scratch.values[215]) < 0.0) { 1.0 } else { 0.0 };

        if (((((!(scratch.values[637] != 0.0)) && (!(scratch.values[652] != 0.0))) && (!(scratch.values[663] != 0.0))) && (!(scratch.values[665] != 0.0))) && (scratch.values[666] != 0.0)) {
            let assign17140_ad_e22344: AdValue = AdValue::offset(AdValue::mul(AdValue::sub_from_scalar((-230.25850929940458), AdValue::div(AdValue::neg(scratch.ad_value(126)), scratch.ad_value(215))), AdValue::offset(AdValue::scale(AdValue::mul(AdValue::sub_from_scalar((-230.25850929940458), AdValue::div(AdValue::neg(scratch.ad_value(126)), scratch.ad_value(215))), AdValue::offset(AdValue::scale(AdValue::sub_from_scalar((-230.25850929940458), AdValue::div(AdValue::neg(scratch.ad_value(126)), scratch.ad_value(215))), 0.3333333333333333), 1.0)), 0.5), 1.0)), 1.0);
            scratch.store_ad(190, &AdValue::div_from_scalar(1e-100, assign17140_ad_e22344));
        }

        if (((((!(scratch.values[637] != 0.0)) && (!(scratch.values[652] != 0.0))) && (!(scratch.values[663] != 0.0))) && (!(scratch.values[665] != 0.0))) && (!(scratch.values[666] != 0.0))) {
            let assign17150_ad_e22393: AdValue = AdValue::scale(AdValue::offset(AdValue::mul(AdValue::offset(AdValue::div(AdValue::neg(scratch.ad_value(126)), scratch.ad_value(215)), (-230.25850929940458)), AdValue::offset(AdValue::scale(AdValue::mul(AdValue::offset(AdValue::div(AdValue::neg(scratch.ad_value(126)), scratch.ad_value(215)), (-230.25850929940458)), AdValue::offset(AdValue::scale(AdValue::offset(AdValue::div(AdValue::neg(scratch.ad_value(126)), scratch.ad_value(215)), (-230.25850929940458)), 0.3333333333333333), 1.0)), 0.5), 1.0)), 1.0), 1e100);
            scratch.store_ad(190, &assign17150_ad_e22393);
        }

        if (((!(scratch.values[637] != 0.0)) && (!(scratch.values[652] != 0.0))) && (!(scratch.values[663] != 0.0))) {
            scratch.store_ad(214, &AdValue::scale(AdValue::mul(AdValue::mul(AdValue::mul(scratch.ad_value(636), scratch.ad_value(215)), scratch.ad_value(215)), scratch.ad_value(190)), scratch.values[31]));
        }

        scratch.values[667] = if (scratch.values[40] > 1000.0) { 1.0 } else { 0.0 };

        if (((!(scratch.values[637] != 0.0)) && (!(scratch.values[652] != 0.0))) && (scratch.values[667] != 0.0)) {
            scratch.values[216] = 1.0;
            scratch.node_derivatives[216] = [0.0; Instance::NODE_COUNT];
            scratch.branch_derivatives[216] = [0.0; Instance::BRANCH_COUNT];
        }

        scratch.values[668] = if (scratch.values[189] > ((-scratch.values[129]) * scratch.values[40])) { 1.0 } else { 0.0 };

        scratch.values[669] = if (scratch.values[43] == 4.0) { 1.0 } else { 0.0 };

        if (((((!(scratch.values[637] != 0.0)) && (!(scratch.values[652] != 0.0))) && (!(scratch.values[667] != 0.0))) && (scratch.values[668] != 0.0)) && (scratch.values[669] != 0.0)) {
            scratch.store_ad(190, &AdValue::mul(AdValue::mul(AdValue::mul(AdValue::scale(scratch.ad_value(189), scratch.values[133]), AdValue::scale(scratch.ad_value(189), scratch.values[133])), AdValue::scale(scratch.ad_value(189), scratch.values[133])), AdValue::scale(scratch.ad_value(189), scratch.values[133])));
        }

        if (((((!(scratch.values[637] != 0.0)) && (!(scratch.values[652] != 0.0))) && (!(scratch.values[667] != 0.0))) && (scratch.values[668] != 0.0)) && (!(scratch.values[669] != 0.0))) {
            scratch.store_ad(190, &AdValue::powf(AdValue::abs(AdValue::scale(scratch.ad_value(189), scratch.values[133])), scratch.values[43]));
        }

        if ((((!(scratch.values[637] != 0.0)) && (!(scratch.values[652] != 0.0))) && (!(scratch.values[667] != 0.0))) && (scratch.values[668] != 0.0)) {
            scratch.store_ad(216, &AdValue::div_from_scalar(1.0, AdValue::sub_from_scalar(1.0, scratch.ad_value(190))));
        }

        if ((((!(scratch.values[637] != 0.0)) && (!(scratch.values[652] != 0.0))) && (!(scratch.values[667] != 0.0))) && (!(scratch.values[668] != 0.0))) {
            scratch.store_ad(216, &AdValue::offset(AdValue::scale(AdValue::offset(scratch.ad_value(189), (scratch.values[129] * scratch.values[40])), scratch.values[136]), scratch.values[130]));
        }

        if ((!(scratch.values[637] != 0.0)) && (!(scratch.values[652] != 0.0))) {
            scratch.store_ad(627, &AdValue::mul(AdValue::add(AdValue::add(AdValue::add(scratch.ad_value(191), scratch.ad_value(192)), scratch.ad_value(199)), scratch.ad_value(214)), scratch.ad_value(216)));
        }

        scratch.values[670] = if (scratch.values[218] == 0.0) { 1.0 } else { 0.0 };

        if ((!(scratch.values[637] != 0.0)) && (scratch.values[670] != 0.0)) {
            scratch.values[629] = 0.0;
            scratch.node_derivatives[629] = [0.0; Instance::NODE_COUNT];
            scratch.branch_derivatives[629] = [0.0; Instance::BRANCH_COUNT];
        }

        if ((!(scratch.values[637] != 0.0)) && (scratch.values[670] != 0.0)) {
            scratch.values[630] = 0.0;
            scratch.node_derivatives[630] = [0.0; Instance::NODE_COUNT];
            scratch.branch_derivatives[630] = [0.0; Instance::BRANCH_COUNT];
        }

        scratch.values[671] = if (scratch.values[94] == 0.5) { 1.0 } else { 0.0 };

        if (((!(scratch.values[637] != 0.0)) && (!(scratch.values[670] != 0.0))) && (scratch.values[671] != 0.0)) {
            scratch.store_ad(190, &AdValue::sqrt(AdValue::sub_from_scalar(1.0, AdValue::scale(scratch.ad_value(182), scratch.values[91]))));
        }

        if (((!(scratch.values[637] != 0.0)) && (!(scratch.values[670] != 0.0))) && (!(scratch.values[671] != 0.0))) {
            scratch.store_ad(190, &AdValue::powf(AdValue::sub_from_scalar(1.0, AdValue::scale(scratch.ad_value(182), scratch.values[91])), scratch.values[94]));
        }

        if ((!(scratch.values[637] != 0.0)) && (!(scratch.values[670] != 0.0))) {
            scratch.store_ad(630, &AdValue::add(AdValue::scale(AdValue::sub_from_scalar(1.0, scratch.ad_value(190)), scratch.values[103]), AdValue::scale(AdValue::sub(scratch.ad_value(636), scratch.ad_value(182)), scratch.values[106])));
        }

        if ((!(scratch.values[637] != 0.0)) && (!(scratch.values[670] != 0.0))) {
            scratch.store_ad(191, &AdValue::scale(scratch.ad_value(181), scratch.values[73]));
        }

        scratch.values[672] = if ((scratch.values[23] == 0.0) && (scratch.values[26] == 0.0)) { 1.0 } else { 0.0 };

        if (((!(scratch.values[637] != 0.0)) && (!(scratch.values[670] != 0.0))) && (scratch.values[672] != 0.0)) {
            scratch.values[192] = 0.0;
            scratch.node_derivatives[192] = [0.0; Instance::NODE_COUNT];
            scratch.branch_derivatives[192] = [0.0; Instance::BRANCH_COUNT];
        }

        if (((!(scratch.values[637] != 0.0)) && (!(scratch.values[670] != 0.0))) && (!(scratch.values[672] != 0.0))) {
            scratch.store_ad(193, &AdValue::sub_from_scalar(scratch.values[79], scratch.ad_value(187)));
        }

        if (((!(scratch.values[637] != 0.0)) && (!(scratch.values[670] != 0.0))) && (!(scratch.values[672] != 0.0))) {
            scratch.store_ad(194, &AdValue::sub_from_scalar(1.0, AdValue::sqrt(AdValue::sub_from_scalar(1.0, AdValue::div(scratch.ad_value(185), scratch.ad_value(193))))));
        }

        scratch.values[673] = if (scratch.values[12] == 0.5) { 1.0 } else { 0.0 };

        if ((((!(scratch.values[637] != 0.0)) && (!(scratch.values[670] != 0.0))) && (!(scratch.values[672] != 0.0))) && (scratch.values[673] != 0.0)) {
            scratch.values[195] = 0.0;
            scratch.node_derivatives[195] = [0.0; Instance::NODE_COUNT];
            scratch.branch_derivatives[195] = [0.0; Instance::BRANCH_COUNT];
        }

        if ((((!(scratch.values[637] != 0.0)) && (!(scratch.values[670] != 0.0))) && (!(scratch.values[672] != 0.0))) && (!(scratch.values[673] != 0.0))) {
            scratch.store_ad(195, &AdValue::scale(AdValue::add(AdValue::div(AdValue::mul(AdValue::square(scratch.ad_value(194)), AdValue::ln(scratch.ad_value(194))), AdValue::sub_from_scalar(1.0, scratch.ad_value(194))), scratch.ad_value(194)), (1.0 - (2.0 * scratch.values[12]))));
        }

        if (((!(scratch.values[637] != 0.0)) && (!(scratch.values[670] != 0.0))) && (!(scratch.values[672] != 0.0))) {
            scratch.store_ad(196, &AdValue::add(scratch.ad_value(194), scratch.ad_value(195)));
        }

        scratch.values[674] = if (scratch.values[12] == 0.5) { 1.0 } else { 0.0 };

        if ((((!(scratch.values[637] != 0.0)) && (!(scratch.values[670] != 0.0))) && (!(scratch.values[672] != 0.0))) && (scratch.values[674] != 0.0)) {
            scratch.store_ad(190, &AdValue::sqrt(AdValue::scale(scratch.ad_value(193), scratch.values[115])));
        }

        if ((((!(scratch.values[637] != 0.0)) && (!(scratch.values[670] != 0.0))) && (!(scratch.values[672] != 0.0))) && (!(scratch.values[674] != 0.0))) {
            scratch.store_ad(190, &AdValue::powf(AdValue::scale(scratch.ad_value(193), scratch.values[115]), scratch.values[12]));
        }

        if (((!(scratch.values[637] != 0.0)) && (!(scratch.values[670] != 0.0))) && (!(scratch.values[672] != 0.0))) {
            scratch.store_ad(197, &AdValue::scale(scratch.ad_value(190), scratch.values[109]));
        }

        if (((!(scratch.values[637] != 0.0)) && (!(scratch.values[670] != 0.0))) && (!(scratch.values[672] != 0.0))) {
            scratch.store_ad(198, &AdValue::scale(AdValue::mul(AdValue::offset(scratch.ad_value(184), (-1.0)), scratch.ad_value(197)), scratch.values[70]));
        }

        if (((!(scratch.values[637] != 0.0)) && (!(scratch.values[670] != 0.0))) && (!(scratch.values[672] != 0.0))) {
            scratch.store_ad(192, &AdValue::scale(AdValue::mul(scratch.ad_value(198), scratch.ad_value(196)), scratch.values[23]));
        }

        scratch.values[675] = if (scratch.values[26] == 0.0) { 1.0 } else { 0.0 };

        if (((!(scratch.values[637] != 0.0)) && (!(scratch.values[670] != 0.0))) && (scratch.values[675] != 0.0)) {
            scratch.values[199] = 0.0;
            scratch.node_derivatives[199] = [0.0; Instance::NODE_COUNT];
            scratch.branch_derivatives[199] = [0.0; Instance::BRANCH_COUNT];
        }

        if (((!(scratch.values[637] != 0.0)) && (!(scratch.values[670] != 0.0))) && (!(scratch.values[675] != 0.0))) {
            scratch.store_ad(200, &AdValue::scale(AdValue::div(AdValue::scale(scratch.ad_value(197), scratch.values[94]), scratch.ad_value(193)), scratch.values[124]));
        }

        if (((!(scratch.values[637] != 0.0)) && (!(scratch.values[670] != 0.0))) && (!(scratch.values[675] != 0.0))) {
            scratch.store_ad(201, &AdValue::div_from_scalar((0.666666666666667 * scratch.values[121]), scratch.ad_value(200)));
        }

        if (((!(scratch.values[637] != 0.0)) && (!(scratch.values[670] != 0.0))) && (!(scratch.values[675] != 0.0))) {
            scratch.store_ad(202, &AdValue::square(scratch.ad_value(201)));
        }

        if (((!(scratch.values[637] != 0.0)) && (!(scratch.values[670] != 0.0))) && (!(scratch.values[675] != 0.0))) {
            scratch.store_ad(203, &AdValue::sqrt(AdValue::div(AdValue::square(scratch.ad_value(202)), AdValue::offset(AdValue::square(scratch.ad_value(202)), 1.0))));
        }

        if (((!(scratch.values[637] != 0.0)) && (!(scratch.values[670] != 0.0))) && (!(scratch.values[675] != 0.0))) {
            scratch.store_ad(204, &AdValue::sqrt(AdValue::abs(scratch.ad_value(203))));
        }

        if (((!(scratch.values[637] != 0.0)) && (!(scratch.values[670] != 0.0))) && (!(scratch.values[675] != 0.0))) {
            scratch.store_ad(205, &AdValue::mul(scratch.ad_value(203), scratch.ad_value(204)));
        }

        scratch.values[676] = if (((-scratch.values[12]) * scratch.values[97]) == (-1.0)) { 1.0 } else { 0.0 };

        if ((((!(scratch.values[637] != 0.0)) && (!(scratch.values[670] != 0.0))) && (!(scratch.values[675] != 0.0))) && (scratch.values[676] != 0.0)) {
            scratch.store_ad(206, &AdValue::div_from_scalar(1.0, AdValue::offset(AdValue::mul(scratch.ad_value(200), scratch.ad_value(205)), 1.0)));
        }

        if ((((!(scratch.values[637] != 0.0)) && (!(scratch.values[670] != 0.0))) && (!(scratch.values[675] != 0.0))) && (!(scratch.values[676] != 0.0))) {
            scratch.store_ad(206, &AdValue::powf(AdValue::offset(AdValue::mul(scratch.ad_value(200), scratch.ad_value(205)), 1.0), ((-scratch.values[12]) * scratch.values[97])));
        }

        if (((!(scratch.values[637] != 0.0)) && (!(scratch.values[670] != 0.0))) && (!(scratch.values[675] != 0.0))) {
            scratch.store_ad(207, &AdValue::div(AdValue::mul(scratch.ad_value(196), scratch.ad_value(206)), AdValue::add(scratch.ad_value(196), scratch.ad_value(206))));
        }

        if (((!(scratch.values[637] != 0.0)) && (!(scratch.values[670] != 0.0))) && (!(scratch.values[675] != 0.0))) {
            scratch.store_ad(208, &AdValue::sqrt(AdValue::scale(AdValue::div(scratch.ad_value(200), scratch.ad_value(204)), 0.375)));
        }

        if (((!(scratch.values[637] != 0.0)) && (!(scratch.values[670] != 0.0))) && (!(scratch.values[675] != 0.0))) {
            scratch.store_ad(209, &AdValue::sub(AdValue::scale(AdValue::mul(scratch.ad_value(201), scratch.ad_value(204)), 2.0), scratch.ad_value(203)));
        }

        if (((!(scratch.values[637] != 0.0)) && (!(scratch.values[670] != 0.0))) && (!(scratch.values[675] != 0.0))) {
            scratch.store_ad(210, &AdValue::add(AdValue::sub(AdValue::mul(AdValue::scale(scratch.ad_value(201), scratch.values[121]), scratch.ad_value(204)), AdValue::scale(scratch.ad_value(203), scratch.values[121])), AdValue::scale(AdValue::mul(scratch.ad_value(200), scratch.ad_value(205)), 0.5)));
        }

        if (((!(scratch.values[637] != 0.0)) && (!(scratch.values[670] != 0.0))) && (!(scratch.values[675] != 0.0))) {
            scratch.store_ad(211, &AdValue::mul(AdValue::offset(scratch.ad_value(209), (-1.0)), scratch.ad_value(208)));
        }

        if (((!(scratch.values[637] != 0.0)) && (!(scratch.values[670] != 0.0))) && (!(scratch.values[675] != 0.0))) {
            scratch.store_ad(172, &AdValue::square(scratch.ad_value(211)));
        }

        scratch.values[677] = if (scratch.values[211] > 0.0) { 1.0 } else { 0.0 };

        if ((((!(scratch.values[637] != 0.0)) && (!(scratch.values[670] != 0.0))) && (!(scratch.values[675] != 0.0))) && (scratch.values[677] != 0.0)) {
            scratch.store_ad(173, &AdValue::div_from_scalar(1.0, AdValue::offset(AdValue::scale(scratch.ad_value(211), scratch.values[57]), 1.0)));
        }

        if ((((!(scratch.values[637] != 0.0)) && (!(scratch.values[670] != 0.0))) && (!(scratch.values[675] != 0.0))) && (!(scratch.values[677] != 0.0))) {
            scratch.store_ad(173, &AdValue::div_from_scalar(1.0, AdValue::sub_from_scalar(1.0, AdValue::scale(scratch.ad_value(211), scratch.values[57]))));
        }

        scratch.values[678] = if (((-scratch.values[172]) + scratch.values[210]) > (-230.25850929940458)) { 1.0 } else { 0.0 };

        if ((((!(scratch.values[637] != 0.0)) && (!(scratch.values[670] != 0.0))) && (!(scratch.values[675] != 0.0))) && (scratch.values[678] != 0.0)) {
            scratch.store_ad(190, &AdValue::exp(AdValue::sub(scratch.ad_value(210), scratch.ad_value(172))));
        }

        if ((((!(scratch.values[637] != 0.0)) && (!(scratch.values[670] != 0.0))) && (!(scratch.values[675] != 0.0))) && (!(scratch.values[678] != 0.0))) {
            scratch.store_ad(190, &AdValue::div_from_scalar(1e-100, AdValue::offset(AdValue::mul(AdValue::sub_from_scalar((-230.25850929940458), AdValue::sub(scratch.ad_value(210), scratch.ad_value(172))), AdValue::offset(AdValue::scale(AdValue::mul(AdValue::sub_from_scalar((-230.25850929940458), AdValue::sub(scratch.ad_value(210), scratch.ad_value(172))), AdValue::offset(AdValue::scale(AdValue::sub_from_scalar((-230.25850929940458), AdValue::sub(scratch.ad_value(210), scratch.ad_value(172))), 0.3333333333333333), 1.0)), 0.5), 1.0)), 1.0)));
        }

        if (((!(scratch.values[637] != 0.0)) && (!(scratch.values[670] != 0.0))) && (!(scratch.values[675] != 0.0))) {
            scratch.store_ad(174, &AdValue::mul(AdValue::add(AdValue::add(AdValue::scale(scratch.ad_value(173), 0.29214664), AdValue::scale(AdValue::square(scratch.ad_value(173)), scratch.values[58])), AdValue::scale(AdValue::mul(AdValue::square(scratch.ad_value(173)), scratch.ad_value(173)), scratch.values[59])), scratch.ad_value(190)));
        }

        scratch.values[679] = if (scratch.values[211] > 0.0) { 1.0 } else { 0.0 };

        if ((((!(scratch.values[637] != 0.0)) && (!(scratch.values[670] != 0.0))) && (!(scratch.values[675] != 0.0))) && (scratch.values[679] != 0.0)) {
            scratch.values[212] = scratch.values[174];
            scratch.node_derivatives[212] = scratch.node_derivatives[174];
            scratch.branch_derivatives[212] = scratch.branch_derivatives[174];
        }

        scratch.values[680] = if (scratch.values[210] > (-230.25850929940458)) { 1.0 } else { 0.0 };

        if (((((!(scratch.values[637] != 0.0)) && (!(scratch.values[670] != 0.0))) && (!(scratch.values[675] != 0.0))) && (!(scratch.values[679] != 0.0))) && (scratch.values[680] != 0.0)) {
            scratch.store_ad(190, &AdValue::exp(scratch.ad_value(210)));
        }

        if (((((!(scratch.values[637] != 0.0)) && (!(scratch.values[670] != 0.0))) && (!(scratch.values[675] != 0.0))) && (!(scratch.values[679] != 0.0))) && (!(scratch.values[680] != 0.0))) {
            scratch.store_ad(190, &AdValue::div_from_scalar(1e-100, AdValue::offset(AdValue::mul(AdValue::sub_from_scalar((-230.25850929940458), scratch.ad_value(210)), AdValue::offset(AdValue::scale(AdValue::mul(AdValue::sub_from_scalar((-230.25850929940458), scratch.ad_value(210)), AdValue::offset(AdValue::scale(AdValue::sub_from_scalar((-230.25850929940458), scratch.ad_value(210)), 0.3333333333333333), 1.0)), 0.5), 1.0)), 1.0)));
        }

        if ((((!(scratch.values[637] != 0.0)) && (!(scratch.values[670] != 0.0))) && (!(scratch.values[675] != 0.0))) && (!(scratch.values[679] != 0.0))) {
            scratch.store_ad(212, &AdValue::sub(AdValue::scale(scratch.ad_value(190), 2.0), scratch.ad_value(174)));
        }

        if (((!(scratch.values[637] != 0.0)) && (!(scratch.values[670] != 0.0))) && (!(scratch.values[675] != 0.0))) {
            scratch.store_ad(213, &AdValue::scale(AdValue::div(AdValue::scale(scratch.ad_value(212), scratch.values[121]), scratch.ad_value(208)), (1.772453850905516 * 0.5)));
        }

        if (((!(scratch.values[637] != 0.0)) && (!(scratch.values[670] != 0.0))) && (!(scratch.values[675] != 0.0))) {
            scratch.store_ad(199, &AdValue::scale(AdValue::mul(AdValue::mul(scratch.ad_value(198), scratch.ad_value(213)), scratch.ad_value(207)), scratch.values[26]));
        }

        scratch.values[681] = if (scratch.values[32] == 0.0) { 1.0 } else { 0.0 };

        if (((!(scratch.values[637] != 0.0)) && (!(scratch.values[670] != 0.0))) && (scratch.values[681] != 0.0)) {
            scratch.values[214] = 0.0;
            scratch.node_derivatives[214] = [0.0; Instance::NODE_COUNT];
            scratch.branch_derivatives[214] = [0.0; Instance::BRANCH_COUNT];
        }

        scratch.values[682] = if (scratch.values[12] == 0.5) { 1.0 } else { 0.0 };

        if ((((!(scratch.values[637] != 0.0)) && (!(scratch.values[670] != 0.0))) && (!(scratch.values[681] != 0.0))) && (scratch.values[682] != 0.0)) {
            scratch.store_ad(190, &AdValue::sqrt(AdValue::scale(AdValue::sub_from_scalar(scratch.values[9], scratch.ad_value(188)), scratch.values[115])));
        }

        if ((((!(scratch.values[637] != 0.0)) && (!(scratch.values[670] != 0.0))) && (!(scratch.values[681] != 0.0))) && (!(scratch.values[682] != 0.0))) {
            scratch.store_ad(190, &AdValue::powf(AdValue::scale(AdValue::sub_from_scalar(scratch.values[9], scratch.ad_value(188)), scratch.values[115]), scratch.values[12]));
        }

        if (((!(scratch.values[637] != 0.0)) && (!(scratch.values[670] != 0.0))) && (!(scratch.values[681] != 0.0))) {
            scratch.store_ad(215, &AdValue::scale(AdValue::div(AdValue::scale(AdValue::sub_from_scalar(scratch.values[9], scratch.ad_value(188)), scratch.values[112]), scratch.ad_value(190)), scratch.values[97]));
        }

        scratch.values[683] = if (((((-scratch.values[127]) / scratch.values[215])) as f64).abs() < 230.25850929940458) { 1.0 } else { 0.0 };

        if ((((!(scratch.values[637] != 0.0)) && (!(scratch.values[670] != 0.0))) && (!(scratch.values[681] != 0.0))) && (scratch.values[683] != 0.0)) {
            scratch.store_ad(190, &AdValue::exp(AdValue::div(AdValue::neg(scratch.ad_value(127)), scratch.ad_value(215))));
        }

        scratch.values[684] = if (((-scratch.values[127]) / scratch.values[215]) < 0.0) { 1.0 } else { 0.0 };

        if (((((!(scratch.values[637] != 0.0)) && (!(scratch.values[670] != 0.0))) && (!(scratch.values[681] != 0.0))) && (!(scratch.values[683] != 0.0))) && (scratch.values[684] != 0.0)) {
            let assign17890_ad_e23493: AdValue = AdValue::offset(AdValue::mul(AdValue::sub_from_scalar((-230.25850929940458), AdValue::div(AdValue::neg(scratch.ad_value(127)), scratch.ad_value(215))), AdValue::offset(AdValue::scale(AdValue::mul(AdValue::sub_from_scalar((-230.25850929940458), AdValue::div(AdValue::neg(scratch.ad_value(127)), scratch.ad_value(215))), AdValue::offset(AdValue::scale(AdValue::sub_from_scalar((-230.25850929940458), AdValue::div(AdValue::neg(scratch.ad_value(127)), scratch.ad_value(215))), 0.3333333333333333), 1.0)), 0.5), 1.0)), 1.0);
            scratch.store_ad(190, &AdValue::div_from_scalar(1e-100, assign17890_ad_e23493));
        }

        if (((((!(scratch.values[637] != 0.0)) && (!(scratch.values[670] != 0.0))) && (!(scratch.values[681] != 0.0))) && (!(scratch.values[683] != 0.0))) && (!(scratch.values[684] != 0.0))) {
            let assign17900_ad_e23542: AdValue = AdValue::scale(AdValue::offset(AdValue::mul(AdValue::offset(AdValue::div(AdValue::neg(scratch.ad_value(127)), scratch.ad_value(215)), (-230.25850929940458)), AdValue::offset(AdValue::scale(AdValue::mul(AdValue::offset(AdValue::div(AdValue::neg(scratch.ad_value(127)), scratch.ad_value(215)), (-230.25850929940458)), AdValue::offset(AdValue::scale(AdValue::offset(AdValue::div(AdValue::neg(scratch.ad_value(127)), scratch.ad_value(215)), (-230.25850929940458)), 0.3333333333333333), 1.0)), 0.5), 1.0)), 1.0), 1e100);
            scratch.store_ad(190, &assign17900_ad_e23542);
        }

        if (((!(scratch.values[637] != 0.0)) && (!(scratch.values[670] != 0.0))) && (!(scratch.values[681] != 0.0))) {
            scratch.store_ad(214, &AdValue::scale(AdValue::mul(AdValue::mul(AdValue::mul(scratch.ad_value(636), scratch.ad_value(215)), scratch.ad_value(215)), scratch.ad_value(190)), scratch.values[32]));
        }

        scratch.values[685] = if (scratch.values[41] > 1000.0) { 1.0 } else { 0.0 };

        if (((!(scratch.values[637] != 0.0)) && (!(scratch.values[670] != 0.0))) && (scratch.values[685] != 0.0)) {
            scratch.values[216] = 1.0;
            scratch.node_derivatives[216] = [0.0; Instance::NODE_COUNT];
            scratch.branch_derivatives[216] = [0.0; Instance::BRANCH_COUNT];
        }

        scratch.values[686] = if (scratch.values[189] > ((-scratch.values[129]) * scratch.values[41])) { 1.0 } else { 0.0 };

        scratch.values[687] = if (scratch.values[44] == 4.0) { 1.0 } else { 0.0 };

        if (((((!(scratch.values[637] != 0.0)) && (!(scratch.values[670] != 0.0))) && (!(scratch.values[685] != 0.0))) && (scratch.values[686] != 0.0)) && (scratch.values[687] != 0.0)) {
            scratch.store_ad(190, &AdValue::mul(AdValue::mul(AdValue::mul(AdValue::scale(scratch.ad_value(189), scratch.values[134]), AdValue::scale(scratch.ad_value(189), scratch.values[134])), AdValue::scale(scratch.ad_value(189), scratch.values[134])), AdValue::scale(scratch.ad_value(189), scratch.values[134])));
        }

        if (((((!(scratch.values[637] != 0.0)) && (!(scratch.values[670] != 0.0))) && (!(scratch.values[685] != 0.0))) && (scratch.values[686] != 0.0)) && (!(scratch.values[687] != 0.0))) {
            scratch.store_ad(190, &AdValue::powf(AdValue::abs(AdValue::scale(scratch.ad_value(189), scratch.values[134])), scratch.values[44]));
        }

        if ((((!(scratch.values[637] != 0.0)) && (!(scratch.values[670] != 0.0))) && (!(scratch.values[685] != 0.0))) && (scratch.values[686] != 0.0)) {
            scratch.store_ad(216, &AdValue::div_from_scalar(1.0, AdValue::sub_from_scalar(1.0, scratch.ad_value(190))));
        }

        if ((((!(scratch.values[637] != 0.0)) && (!(scratch.values[670] != 0.0))) && (!(scratch.values[685] != 0.0))) && (!(scratch.values[686] != 0.0))) {
            scratch.store_ad(216, &AdValue::offset(AdValue::scale(AdValue::offset(scratch.ad_value(189), (scratch.values[129] * scratch.values[41])), scratch.values[137]), scratch.values[131]));
        }

        if ((!(scratch.values[637] != 0.0)) && (!(scratch.values[670] != 0.0))) {
            scratch.store_ad(629, &AdValue::mul(AdValue::add(AdValue::add(AdValue::add(scratch.ad_value(191), scratch.ad_value(192)), scratch.ad_value(199)), scratch.ad_value(214)), scratch.ad_value(216)));
        }

        scratch.values[688] = if (scratch.values[219] == 0.0) { 1.0 } else { 0.0 };

        if ((!(scratch.values[637] != 0.0)) && (scratch.values[688] != 0.0)) {
            scratch.values[631] = 0.0;
            scratch.node_derivatives[631] = [0.0; Instance::NODE_COUNT];
            scratch.branch_derivatives[631] = [0.0; Instance::BRANCH_COUNT];
        }

        if ((!(scratch.values[637] != 0.0)) && (scratch.values[688] != 0.0)) {
            scratch.values[632] = 0.0;
            scratch.node_derivatives[632] = [0.0; Instance::NODE_COUNT];
            scratch.branch_derivatives[632] = [0.0; Instance::BRANCH_COUNT];
        }

        scratch.values[689] = if (scratch.values[95] == 0.5) { 1.0 } else { 0.0 };

        if (((!(scratch.values[637] != 0.0)) && (!(scratch.values[688] != 0.0))) && (scratch.values[689] != 0.0)) {
            scratch.store_ad(190, &AdValue::sqrt(AdValue::sub_from_scalar(1.0, AdValue::scale(scratch.ad_value(182), scratch.values[92]))));
        }

        if (((!(scratch.values[637] != 0.0)) && (!(scratch.values[688] != 0.0))) && (!(scratch.values[689] != 0.0))) {
            scratch.store_ad(190, &AdValue::powf(AdValue::sub_from_scalar(1.0, AdValue::scale(scratch.ad_value(182), scratch.values[92])), scratch.values[95]));
        }

        if ((!(scratch.values[637] != 0.0)) && (!(scratch.values[688] != 0.0))) {
            scratch.store_ad(632, &AdValue::add(AdValue::scale(AdValue::sub_from_scalar(1.0, scratch.ad_value(190)), scratch.values[104]), AdValue::scale(AdValue::sub(scratch.ad_value(636), scratch.ad_value(182)), scratch.values[107])));
        }

        if ((!(scratch.values[637] != 0.0)) && (!(scratch.values[688] != 0.0))) {
            scratch.store_ad(191, &AdValue::scale(scratch.ad_value(181), scratch.values[74]));
        }

        scratch.values[690] = if ((scratch.values[24] == 0.0) && (scratch.values[27] == 0.0)) { 1.0 } else { 0.0 };

        if (((!(scratch.values[637] != 0.0)) && (!(scratch.values[688] != 0.0))) && (scratch.values[690] != 0.0)) {
            scratch.values[192] = 0.0;
            scratch.node_derivatives[192] = [0.0; Instance::NODE_COUNT];
            scratch.branch_derivatives[192] = [0.0; Instance::BRANCH_COUNT];
        }

        if (((!(scratch.values[637] != 0.0)) && (!(scratch.values[688] != 0.0))) && (!(scratch.values[690] != 0.0))) {
            scratch.store_ad(193, &AdValue::sub_from_scalar(scratch.values[80], scratch.ad_value(187)));
        }

        if (((!(scratch.values[637] != 0.0)) && (!(scratch.values[688] != 0.0))) && (!(scratch.values[690] != 0.0))) {
            scratch.store_ad(194, &AdValue::sub_from_scalar(1.0, AdValue::sqrt(AdValue::sub_from_scalar(1.0, AdValue::div(scratch.ad_value(185), scratch.ad_value(193))))));
        }

        scratch.values[691] = if (scratch.values[13] == 0.5) { 1.0 } else { 0.0 };

        if ((((!(scratch.values[637] != 0.0)) && (!(scratch.values[688] != 0.0))) && (!(scratch.values[690] != 0.0))) && (scratch.values[691] != 0.0)) {
            scratch.values[195] = 0.0;
            scratch.node_derivatives[195] = [0.0; Instance::NODE_COUNT];
            scratch.branch_derivatives[195] = [0.0; Instance::BRANCH_COUNT];
        }

        if ((((!(scratch.values[637] != 0.0)) && (!(scratch.values[688] != 0.0))) && (!(scratch.values[690] != 0.0))) && (!(scratch.values[691] != 0.0))) {
            scratch.store_ad(195, &AdValue::scale(AdValue::add(AdValue::div(AdValue::mul(AdValue::square(scratch.ad_value(194)), AdValue::ln(scratch.ad_value(194))), AdValue::sub_from_scalar(1.0, scratch.ad_value(194))), scratch.ad_value(194)), (1.0 - (2.0 * scratch.values[13]))));
        }

        if (((!(scratch.values[637] != 0.0)) && (!(scratch.values[688] != 0.0))) && (!(scratch.values[690] != 0.0))) {
            scratch.store_ad(196, &AdValue::add(scratch.ad_value(194), scratch.ad_value(195)));
        }

        scratch.values[692] = if (scratch.values[13] == 0.5) { 1.0 } else { 0.0 };

        if ((((!(scratch.values[637] != 0.0)) && (!(scratch.values[688] != 0.0))) && (!(scratch.values[690] != 0.0))) && (scratch.values[692] != 0.0)) {
            scratch.store_ad(190, &AdValue::sqrt(AdValue::scale(scratch.ad_value(193), scratch.values[116])));
        }

        if ((((!(scratch.values[637] != 0.0)) && (!(scratch.values[688] != 0.0))) && (!(scratch.values[690] != 0.0))) && (!(scratch.values[692] != 0.0))) {
            scratch.store_ad(190, &AdValue::powf(AdValue::scale(scratch.ad_value(193), scratch.values[116]), scratch.values[13]));
        }

        if (((!(scratch.values[637] != 0.0)) && (!(scratch.values[688] != 0.0))) && (!(scratch.values[690] != 0.0))) {
            scratch.store_ad(197, &AdValue::scale(scratch.ad_value(190), scratch.values[110]));
        }

        if (((!(scratch.values[637] != 0.0)) && (!(scratch.values[688] != 0.0))) && (!(scratch.values[690] != 0.0))) {
            scratch.store_ad(198, &AdValue::scale(AdValue::mul(AdValue::offset(scratch.ad_value(184), (-1.0)), scratch.ad_value(197)), scratch.values[71]));
        }

        if (((!(scratch.values[637] != 0.0)) && (!(scratch.values[688] != 0.0))) && (!(scratch.values[690] != 0.0))) {
            scratch.store_ad(192, &AdValue::scale(AdValue::mul(scratch.ad_value(198), scratch.ad_value(196)), scratch.values[24]));
        }

        scratch.values[693] = if (scratch.values[27] == 0.0) { 1.0 } else { 0.0 };

        if (((!(scratch.values[637] != 0.0)) && (!(scratch.values[688] != 0.0))) && (scratch.values[693] != 0.0)) {
            scratch.values[199] = 0.0;
            scratch.node_derivatives[199] = [0.0; Instance::NODE_COUNT];
            scratch.branch_derivatives[199] = [0.0; Instance::BRANCH_COUNT];
        }

        if (((!(scratch.values[637] != 0.0)) && (!(scratch.values[688] != 0.0))) && (!(scratch.values[693] != 0.0))) {
            scratch.store_ad(200, &AdValue::scale(AdValue::div(AdValue::scale(scratch.ad_value(197), scratch.values[95]), scratch.ad_value(193)), scratch.values[125]));
        }

        if (((!(scratch.values[637] != 0.0)) && (!(scratch.values[688] != 0.0))) && (!(scratch.values[693] != 0.0))) {
            scratch.store_ad(201, &AdValue::div_from_scalar((0.666666666666667 * scratch.values[122]), scratch.ad_value(200)));
        }

        if (((!(scratch.values[637] != 0.0)) && (!(scratch.values[688] != 0.0))) && (!(scratch.values[693] != 0.0))) {
            scratch.store_ad(202, &AdValue::square(scratch.ad_value(201)));
        }

        if (((!(scratch.values[637] != 0.0)) && (!(scratch.values[688] != 0.0))) && (!(scratch.values[693] != 0.0))) {
            scratch.store_ad(203, &AdValue::sqrt(AdValue::div(AdValue::square(scratch.ad_value(202)), AdValue::offset(AdValue::square(scratch.ad_value(202)), 1.0))));
        }

        if (((!(scratch.values[637] != 0.0)) && (!(scratch.values[688] != 0.0))) && (!(scratch.values[693] != 0.0))) {
            scratch.store_ad(204, &AdValue::sqrt(AdValue::abs(scratch.ad_value(203))));
        }

        if (((!(scratch.values[637] != 0.0)) && (!(scratch.values[688] != 0.0))) && (!(scratch.values[693] != 0.0))) {
            scratch.store_ad(205, &AdValue::mul(scratch.ad_value(203), scratch.ad_value(204)));
        }

        scratch.values[694] = if (((-scratch.values[13]) * scratch.values[98]) == (-1.0)) { 1.0 } else { 0.0 };

        if ((((!(scratch.values[637] != 0.0)) && (!(scratch.values[688] != 0.0))) && (!(scratch.values[693] != 0.0))) && (scratch.values[694] != 0.0)) {
            scratch.store_ad(206, &AdValue::div_from_scalar(1.0, AdValue::offset(AdValue::mul(scratch.ad_value(200), scratch.ad_value(205)), 1.0)));
        }

        if ((((!(scratch.values[637] != 0.0)) && (!(scratch.values[688] != 0.0))) && (!(scratch.values[693] != 0.0))) && (!(scratch.values[694] != 0.0))) {
            scratch.store_ad(206, &AdValue::powf(AdValue::offset(AdValue::mul(scratch.ad_value(200), scratch.ad_value(205)), 1.0), ((-scratch.values[13]) * scratch.values[98])));
        }

        if (((!(scratch.values[637] != 0.0)) && (!(scratch.values[688] != 0.0))) && (!(scratch.values[693] != 0.0))) {
            scratch.store_ad(207, &AdValue::div(AdValue::mul(scratch.ad_value(196), scratch.ad_value(206)), AdValue::add(scratch.ad_value(196), scratch.ad_value(206))));
        }

        if (((!(scratch.values[637] != 0.0)) && (!(scratch.values[688] != 0.0))) && (!(scratch.values[693] != 0.0))) {
            scratch.store_ad(208, &AdValue::sqrt(AdValue::scale(AdValue::div(scratch.ad_value(200), scratch.ad_value(204)), 0.375)));
        }

        if (((!(scratch.values[637] != 0.0)) && (!(scratch.values[688] != 0.0))) && (!(scratch.values[693] != 0.0))) {
            scratch.store_ad(209, &AdValue::sub(AdValue::scale(AdValue::mul(scratch.ad_value(201), scratch.ad_value(204)), 2.0), scratch.ad_value(203)));
        }

        if (((!(scratch.values[637] != 0.0)) && (!(scratch.values[688] != 0.0))) && (!(scratch.values[693] != 0.0))) {
            scratch.store_ad(210, &AdValue::add(AdValue::sub(AdValue::mul(AdValue::scale(scratch.ad_value(201), scratch.values[122]), scratch.ad_value(204)), AdValue::scale(scratch.ad_value(203), scratch.values[122])), AdValue::scale(AdValue::mul(scratch.ad_value(200), scratch.ad_value(205)), 0.5)));
        }

        if (((!(scratch.values[637] != 0.0)) && (!(scratch.values[688] != 0.0))) && (!(scratch.values[693] != 0.0))) {
            scratch.store_ad(211, &AdValue::mul(AdValue::offset(scratch.ad_value(209), (-1.0)), scratch.ad_value(208)));
        }

    }

    pub(super) fn stamp_transient_block_13(
        &mut self,
        ctx: &GeneratedEvalContext<'_>,
        stamper: &mut GeneratedStamper<'_>,
        scratch: &mut Scratch,
    ) {
        let _ = stamper;
        if (((!(scratch.values[637] != 0.0)) && (!(scratch.values[688] != 0.0))) && (!(scratch.values[693] != 0.0))) {
            scratch.store_ad(172, &AdValue::square(scratch.ad_value(211)));
        }

        scratch.values[695] = if (scratch.values[211] > 0.0) { 1.0 } else { 0.0 };

        if ((((!(scratch.values[637] != 0.0)) && (!(scratch.values[688] != 0.0))) && (!(scratch.values[693] != 0.0))) && (scratch.values[695] != 0.0)) {
            scratch.store_ad(173, &AdValue::div_from_scalar(1.0, AdValue::offset(AdValue::scale(scratch.ad_value(211), scratch.values[57]), 1.0)));
        }

        if ((((!(scratch.values[637] != 0.0)) && (!(scratch.values[688] != 0.0))) && (!(scratch.values[693] != 0.0))) && (!(scratch.values[695] != 0.0))) {
            scratch.store_ad(173, &AdValue::div_from_scalar(1.0, AdValue::sub_from_scalar(1.0, AdValue::scale(scratch.ad_value(211), scratch.values[57]))));
        }

        scratch.values[696] = if (((-scratch.values[172]) + scratch.values[210]) > (-230.25850929940458)) { 1.0 } else { 0.0 };

        if ((((!(scratch.values[637] != 0.0)) && (!(scratch.values[688] != 0.0))) && (!(scratch.values[693] != 0.0))) && (scratch.values[696] != 0.0)) {
            scratch.store_ad(190, &AdValue::exp(AdValue::sub(scratch.ad_value(210), scratch.ad_value(172))));
        }

        if ((((!(scratch.values[637] != 0.0)) && (!(scratch.values[688] != 0.0))) && (!(scratch.values[693] != 0.0))) && (!(scratch.values[696] != 0.0))) {
            scratch.store_ad(190, &AdValue::div_from_scalar(1e-100, AdValue::offset(AdValue::mul(AdValue::sub_from_scalar((-230.25850929940458), AdValue::sub(scratch.ad_value(210), scratch.ad_value(172))), AdValue::offset(AdValue::scale(AdValue::mul(AdValue::sub_from_scalar((-230.25850929940458), AdValue::sub(scratch.ad_value(210), scratch.ad_value(172))), AdValue::offset(AdValue::scale(AdValue::sub_from_scalar((-230.25850929940458), AdValue::sub(scratch.ad_value(210), scratch.ad_value(172))), 0.3333333333333333), 1.0)), 0.5), 1.0)), 1.0)));
        }

        if (((!(scratch.values[637] != 0.0)) && (!(scratch.values[688] != 0.0))) && (!(scratch.values[693] != 0.0))) {
            scratch.store_ad(174, &AdValue::mul(AdValue::add(AdValue::add(AdValue::scale(scratch.ad_value(173), 0.29214664), AdValue::scale(AdValue::square(scratch.ad_value(173)), scratch.values[58])), AdValue::scale(AdValue::mul(AdValue::square(scratch.ad_value(173)), scratch.ad_value(173)), scratch.values[59])), scratch.ad_value(190)));
        }

        scratch.values[697] = if (scratch.values[211] > 0.0) { 1.0 } else { 0.0 };

        if ((((!(scratch.values[637] != 0.0)) && (!(scratch.values[688] != 0.0))) && (!(scratch.values[693] != 0.0))) && (scratch.values[697] != 0.0)) {
            scratch.values[212] = scratch.values[174];
            scratch.node_derivatives[212] = scratch.node_derivatives[174];
            scratch.branch_derivatives[212] = scratch.branch_derivatives[174];
        }

        scratch.values[698] = if (scratch.values[210] > (-230.25850929940458)) { 1.0 } else { 0.0 };

        if (((((!(scratch.values[637] != 0.0)) && (!(scratch.values[688] != 0.0))) && (!(scratch.values[693] != 0.0))) && (!(scratch.values[697] != 0.0))) && (scratch.values[698] != 0.0)) {
            scratch.store_ad(190, &AdValue::exp(scratch.ad_value(210)));
        }

        if (((((!(scratch.values[637] != 0.0)) && (!(scratch.values[688] != 0.0))) && (!(scratch.values[693] != 0.0))) && (!(scratch.values[697] != 0.0))) && (!(scratch.values[698] != 0.0))) {
            scratch.store_ad(190, &AdValue::div_from_scalar(1e-100, AdValue::offset(AdValue::mul(AdValue::sub_from_scalar((-230.25850929940458), scratch.ad_value(210)), AdValue::offset(AdValue::scale(AdValue::mul(AdValue::sub_from_scalar((-230.25850929940458), scratch.ad_value(210)), AdValue::offset(AdValue::scale(AdValue::sub_from_scalar((-230.25850929940458), scratch.ad_value(210)), 0.3333333333333333), 1.0)), 0.5), 1.0)), 1.0)));
        }

        if ((((!(scratch.values[637] != 0.0)) && (!(scratch.values[688] != 0.0))) && (!(scratch.values[693] != 0.0))) && (!(scratch.values[697] != 0.0))) {
            scratch.store_ad(212, &AdValue::sub(AdValue::scale(scratch.ad_value(190), 2.0), scratch.ad_value(174)));
        }

        if (((!(scratch.values[637] != 0.0)) && (!(scratch.values[688] != 0.0))) && (!(scratch.values[693] != 0.0))) {
            scratch.store_ad(213, &AdValue::scale(AdValue::div(AdValue::scale(scratch.ad_value(212), scratch.values[122]), scratch.ad_value(208)), (1.772453850905516 * 0.5)));
        }

        if (((!(scratch.values[637] != 0.0)) && (!(scratch.values[688] != 0.0))) && (!(scratch.values[693] != 0.0))) {
            scratch.store_ad(199, &AdValue::scale(AdValue::mul(AdValue::mul(scratch.ad_value(198), scratch.ad_value(213)), scratch.ad_value(207)), scratch.values[27]));
        }

        scratch.values[699] = if (scratch.values[33] == 0.0) { 1.0 } else { 0.0 };

        if (((!(scratch.values[637] != 0.0)) && (!(scratch.values[688] != 0.0))) && (scratch.values[699] != 0.0)) {
            scratch.values[214] = 0.0;
            scratch.node_derivatives[214] = [0.0; Instance::NODE_COUNT];
            scratch.branch_derivatives[214] = [0.0; Instance::BRANCH_COUNT];
        }

        scratch.values[700] = if (scratch.values[13] == 0.5) { 1.0 } else { 0.0 };

        if ((((!(scratch.values[637] != 0.0)) && (!(scratch.values[688] != 0.0))) && (!(scratch.values[699] != 0.0))) && (scratch.values[700] != 0.0)) {
            scratch.store_ad(190, &AdValue::sqrt(AdValue::scale(AdValue::sub_from_scalar(scratch.values[10], scratch.ad_value(188)), scratch.values[116])));
        }

        if ((((!(scratch.values[637] != 0.0)) && (!(scratch.values[688] != 0.0))) && (!(scratch.values[699] != 0.0))) && (!(scratch.values[700] != 0.0))) {
            scratch.store_ad(190, &AdValue::powf(AdValue::scale(AdValue::sub_from_scalar(scratch.values[10], scratch.ad_value(188)), scratch.values[116]), scratch.values[13]));
        }

        if (((!(scratch.values[637] != 0.0)) && (!(scratch.values[688] != 0.0))) && (!(scratch.values[699] != 0.0))) {
            scratch.store_ad(215, &AdValue::scale(AdValue::div(AdValue::scale(AdValue::sub_from_scalar(scratch.values[10], scratch.ad_value(188)), scratch.values[113]), scratch.ad_value(190)), scratch.values[98]));
        }

        scratch.values[701] = if (((((-scratch.values[128]) / scratch.values[215])) as f64).abs() < 230.25850929940458) { 1.0 } else { 0.0 };

        if ((((!(scratch.values[637] != 0.0)) && (!(scratch.values[688] != 0.0))) && (!(scratch.values[699] != 0.0))) && (scratch.values[701] != 0.0)) {
            scratch.store_ad(190, &AdValue::exp(AdValue::div(AdValue::neg(scratch.ad_value(128)), scratch.ad_value(215))));
        }

        scratch.values[702] = if (((-scratch.values[128]) / scratch.values[215]) < 0.0) { 1.0 } else { 0.0 };

        if (((((!(scratch.values[637] != 0.0)) && (!(scratch.values[688] != 0.0))) && (!(scratch.values[699] != 0.0))) && (!(scratch.values[701] != 0.0))) && (scratch.values[702] != 0.0)) {
            let assign18640_ad_e24642: AdValue = AdValue::offset(AdValue::mul(AdValue::sub_from_scalar((-230.25850929940458), AdValue::div(AdValue::neg(scratch.ad_value(128)), scratch.ad_value(215))), AdValue::offset(AdValue::scale(AdValue::mul(AdValue::sub_from_scalar((-230.25850929940458), AdValue::div(AdValue::neg(scratch.ad_value(128)), scratch.ad_value(215))), AdValue::offset(AdValue::scale(AdValue::sub_from_scalar((-230.25850929940458), AdValue::div(AdValue::neg(scratch.ad_value(128)), scratch.ad_value(215))), 0.3333333333333333), 1.0)), 0.5), 1.0)), 1.0);
            scratch.store_ad(190, &AdValue::div_from_scalar(1e-100, assign18640_ad_e24642));
        }

        if (((((!(scratch.values[637] != 0.0)) && (!(scratch.values[688] != 0.0))) && (!(scratch.values[699] != 0.0))) && (!(scratch.values[701] != 0.0))) && (!(scratch.values[702] != 0.0))) {
            let assign18650_ad_e24691: AdValue = AdValue::scale(AdValue::offset(AdValue::mul(AdValue::offset(AdValue::div(AdValue::neg(scratch.ad_value(128)), scratch.ad_value(215)), (-230.25850929940458)), AdValue::offset(AdValue::scale(AdValue::mul(AdValue::offset(AdValue::div(AdValue::neg(scratch.ad_value(128)), scratch.ad_value(215)), (-230.25850929940458)), AdValue::offset(AdValue::scale(AdValue::offset(AdValue::div(AdValue::neg(scratch.ad_value(128)), scratch.ad_value(215)), (-230.25850929940458)), 0.3333333333333333), 1.0)), 0.5), 1.0)), 1.0), 1e100);
            scratch.store_ad(190, &assign18650_ad_e24691);
        }

        if (((!(scratch.values[637] != 0.0)) && (!(scratch.values[688] != 0.0))) && (!(scratch.values[699] != 0.0))) {
            scratch.store_ad(214, &AdValue::scale(AdValue::mul(AdValue::mul(AdValue::mul(scratch.ad_value(636), scratch.ad_value(215)), scratch.ad_value(215)), scratch.ad_value(190)), scratch.values[33]));
        }

        scratch.values[703] = if (scratch.values[42] > 1000.0) { 1.0 } else { 0.0 };

        if (((!(scratch.values[637] != 0.0)) && (!(scratch.values[688] != 0.0))) && (scratch.values[703] != 0.0)) {
            scratch.values[216] = 1.0;
            scratch.node_derivatives[216] = [0.0; Instance::NODE_COUNT];
            scratch.branch_derivatives[216] = [0.0; Instance::BRANCH_COUNT];
        }

        scratch.values[704] = if (scratch.values[189] > ((-scratch.values[129]) * scratch.values[42])) { 1.0 } else { 0.0 };

        scratch.values[705] = if (scratch.values[45] == 4.0) { 1.0 } else { 0.0 };

        if (((((!(scratch.values[637] != 0.0)) && (!(scratch.values[688] != 0.0))) && (!(scratch.values[703] != 0.0))) && (scratch.values[704] != 0.0)) && (scratch.values[705] != 0.0)) {
            scratch.store_ad(190, &AdValue::mul(AdValue::mul(AdValue::mul(AdValue::scale(scratch.ad_value(189), scratch.values[135]), AdValue::scale(scratch.ad_value(189), scratch.values[135])), AdValue::scale(scratch.ad_value(189), scratch.values[135])), AdValue::scale(scratch.ad_value(189), scratch.values[135])));
        }

        if (((((!(scratch.values[637] != 0.0)) && (!(scratch.values[688] != 0.0))) && (!(scratch.values[703] != 0.0))) && (scratch.values[704] != 0.0)) && (!(scratch.values[705] != 0.0))) {
            scratch.store_ad(190, &AdValue::powf(AdValue::abs(AdValue::scale(scratch.ad_value(189), scratch.values[135])), scratch.values[45]));
        }

        if ((((!(scratch.values[637] != 0.0)) && (!(scratch.values[688] != 0.0))) && (!(scratch.values[703] != 0.0))) && (scratch.values[704] != 0.0)) {
            scratch.store_ad(216, &AdValue::div_from_scalar(1.0, AdValue::sub_from_scalar(1.0, scratch.ad_value(190))));
        }

        if ((((!(scratch.values[637] != 0.0)) && (!(scratch.values[688] != 0.0))) && (!(scratch.values[703] != 0.0))) && (!(scratch.values[704] != 0.0))) {
            scratch.store_ad(216, &AdValue::offset(AdValue::scale(AdValue::offset(scratch.ad_value(189), (scratch.values[129] * scratch.values[42])), scratch.values[138]), scratch.values[132]));
        }

        if ((!(scratch.values[637] != 0.0)) && (!(scratch.values[688] != 0.0))) {
            scratch.store_ad(631, &AdValue::mul(AdValue::add(AdValue::add(AdValue::add(scratch.ad_value(191), scratch.ad_value(192)), scratch.ad_value(199)), scratch.ad_value(214)), scratch.ad_value(216)));
        }

        if (!(scratch.values[637] != 0.0)) {
            scratch.store_ad(633, &AdValue::add(AdValue::add(AdValue::scale(scratch.ad_value(627), scratch.values[217]), AdValue::scale(scratch.ad_value(629), scratch.values[218])), AdValue::scale(scratch.ad_value(631), scratch.values[219])));
        }

        scratch.store_ad(634, &AdValue::add(AdValue::add(AdValue::scale(scratch.ad_value(628), scratch.values[217]), AdValue::scale(scratch.ad_value(630), scratch.values[218])), AdValue::scale(scratch.ad_value(632), scratch.values[219])));

        scratch.store_ad(635, &AdValue::scale(AdValue::abs(scratch.ad_value(633)), (2.0 * 1.6021918e-19)));

    }

    pub(super) fn stamp_reactive_block_0(
        &mut self,
        ctx: &GeneratedEvalContext<'_>,
        stamper: &mut GeneratedReactiveStamper<'_>,
        scratch: &mut ReactiveScratch,
    ) {
        let _ = stamper;
        scratch.values[1] = (8.8541878176e-12 * 11.8);

        scratch.values[2] = (if (self.params.trj > (-250.0)) { self.params.trj } else { (-250.0) });

        scratch.values[3] = (if (self.params.imax > 1e-12) { self.params.imax } else { 1e-12 });

        scratch.values[4] = (if (self.params.frev > 10.0) { (if (self.params.frev < 10000000000.0) { self.params.frev } else { 10000000000.0 }) } else { 10.0 });

        scratch.values[5] = (if (self.params.cjorbot > 1e-12) { self.params.cjorbot } else { 1e-12 });

        scratch.values[6] = (if (self.params.cjorsti > 1e-18) { self.params.cjorsti } else { 1e-18 });

        scratch.values[7] = (if (self.params.cjorgat > 1e-18) { self.params.cjorgat } else { 1e-18 });

        scratch.values[8] = (if (self.params.vbirbot > 0.05) { self.params.vbirbot } else { 0.05 });

        scratch.values[9] = (if (self.params.vbirsti > 0.05) { self.params.vbirsti } else { 0.05 });

        scratch.values[10] = (if (self.params.vbirgat > 0.05) { self.params.vbirgat } else { 0.05 });

        scratch.values[11] = (if (self.params.pbot > 0.05) { (if (self.params.pbot < 0.95) { self.params.pbot } else { 0.95 }) } else { 0.05 });

        scratch.values[12] = (if (self.params.psti > 0.05) { (if (self.params.psti < 0.95) { self.params.psti } else { 0.95 }) } else { 0.05 });

        scratch.values[13] = (if (self.params.pgat > 0.05) { (if (self.params.pgat < 0.95) { self.params.pgat } else { 0.95 }) } else { 0.05 });

        scratch.values[14] = self.params.phigbot;

        scratch.values[15] = self.params.phigsti;

        scratch.values[16] = self.params.phiggat;

        scratch.values[17] = (if (self.params.idsatrbot > 0.0) { self.params.idsatrbot } else { 0.0 });

        scratch.values[18] = (if (self.params.idsatrsti > 0.0) { self.params.idsatrsti } else { 0.0 });

        scratch.values[19] = (if (self.params.idsatrgat > 0.0) { self.params.idsatrgat } else { 0.0 });

        scratch.values[22] = (if (self.params.csrhbot > 0.0) { self.params.csrhbot } else { 0.0 });

        scratch.values[23] = (if (self.params.csrhsti > 0.0) { self.params.csrhsti } else { 0.0 });

        scratch.values[24] = (if (self.params.csrhgat > 0.0) { self.params.csrhgat } else { 0.0 });

        scratch.values[20] = (if (self.params.xjunsti > 1e-9) { self.params.xjunsti } else { 1e-9 });

        scratch.values[21] = (if (self.params.xjungat > 1e-9) { self.params.xjungat } else { 1e-9 });

        scratch.values[25] = (if (self.params.ctatbot > 0.0) { self.params.ctatbot } else { 0.0 });

        scratch.values[26] = (if (self.params.ctatsti > 0.0) { self.params.ctatsti } else { 0.0 });

        scratch.values[27] = (if (self.params.ctatgat > 0.0) { self.params.ctatgat } else { 0.0 });

        scratch.values[28] = (if (self.params.mefftatbot > 0.01) { self.params.mefftatbot } else { 0.01 });

        scratch.values[29] = (if (self.params.mefftatsti > 0.01) { self.params.mefftatsti } else { 0.01 });

        scratch.values[30] = (if (self.params.mefftatgat > 0.01) { self.params.mefftatgat } else { 0.01 });

        scratch.values[31] = (if (self.params.cbbtbot > 0.0) { self.params.cbbtbot } else { 0.0 });

        scratch.values[32] = (if (self.params.cbbtsti > 0.0) { self.params.cbbtsti } else { 0.0 });

        scratch.values[33] = (if (self.params.cbbtgat > 0.0) { self.params.cbbtgat } else { 0.0 });

        scratch.values[34] = self.params.fbbtrbot;

        scratch.values[35] = self.params.fbbtrsti;

        scratch.values[36] = self.params.fbbtrgat;

        scratch.values[37] = self.params.stfbbtbot;

        scratch.values[38] = self.params.stfbbtsti;

        scratch.values[39] = self.params.stfbbtgat;

        scratch.values[40] = (if (self.params.vbrbot > 0.1) { self.params.vbrbot } else { 0.1 });

        scratch.values[41] = (if (self.params.vbrsti > 0.1) { self.params.vbrsti } else { 0.1 });

        scratch.values[42] = (if (self.params.vbrgat > 0.1) { self.params.vbrgat } else { 0.1 });

        scratch.values[43] = (if (self.params.pbrbot > 0.1) { self.params.pbrbot } else { 0.1 });

        scratch.values[44] = (if (self.params.pbrsti > 0.1) { self.params.pbrsti } else { 0.1 });

        scratch.values[45] = (if (self.params.pbrgat > 0.1) { self.params.pbrgat } else { 0.1 });

        scratch.values[46] = 0.0;

        scratch.values[261] = if (self.params.swjunexp > 0.5) { 1.0 } else { 0.0 };

        if (scratch.values[261] != 0.0) {
            scratch.values[46] = 1.0;
            scratch.node_derivatives[46] = [0.0; Instance::NODE_COUNT];
            scratch.branch_derivatives[46] = [0.0; Instance::BRANCH_COUNT];
        }

        if (!(scratch.values[261] != 0.0)) {
            scratch.values[46] = 0.0;
            scratch.node_derivatives[46] = [0.0; Instance::NODE_COUNT];
            scratch.branch_derivatives[46] = [0.0; Instance::BRANCH_COUNT];
        }

        scratch.values[48] = (if (self.params.fjunq > 0.0) { self.params.fjunq } else { 0.0 });

        scratch.values[49] = (273.15 + scratch.values[2]);

        scratch.values[50] = ((ctx.temperature() + self.params.dta)).max((273.15 + (-250.0)));

        scratch.values[51] = (scratch.values[50] / scratch.values[49]);

        scratch.values[52] = (1.3806505e-23 / 1.6021918e-19);

        scratch.values[53] = (scratch.values[52] * scratch.values[49]);

        scratch.values[54] = (1.0 / scratch.values[53]);

        scratch.values[55] = (scratch.values[52] * scratch.values[50]);

        scratch.values[56] = (1.0 / scratch.values[55]);

        scratch.values[60] = ((-((0.000702 * scratch.values[49]) * scratch.values[49])) / (1108.0 + scratch.values[49]));

        scratch.values[63] = (scratch.values[14] + scratch.values[60]);

        scratch.values[64] = (scratch.values[15] + scratch.values[60]);

        scratch.values[65] = (scratch.values[16] + scratch.values[60]);

        scratch.values[61] = ((-((0.000702 * scratch.values[50]) * scratch.values[50])) / (1108.0 + scratch.values[50]));

        scratch.values[66] = (scratch.values[14] + scratch.values[61]);

        scratch.values[67] = (scratch.values[15] + scratch.values[61]);

        scratch.values[68] = (scratch.values[16] + scratch.values[61]);

        scratch.values[69] = (((scratch.values[51]) as f64).powf(1.5) * (((0.5 * ((scratch.values[63] * scratch.values[54]) - (scratch.values[66] * scratch.values[56])))) as f64).exp());

        scratch.values[70] = (((scratch.values[51]) as f64).powf(1.5) * (((0.5 * ((scratch.values[64] * scratch.values[54]) - (scratch.values[67] * scratch.values[56])))) as f64).exp());

        scratch.values[71] = (((scratch.values[51]) as f64).powf(1.5) * (((0.5 * ((scratch.values[65] * scratch.values[54]) - (scratch.values[68] * scratch.values[56])))) as f64).exp());

        scratch.values[72] = ((scratch.values[17] * scratch.values[69]) * scratch.values[69]);

        scratch.values[73] = ((scratch.values[18] * scratch.values[70]) * scratch.values[70]);

        scratch.values[74] = ((scratch.values[19] * scratch.values[71]) * scratch.values[71]);

        scratch.values[75] = ((scratch.values[8] * scratch.values[51]) - ((2.0 * scratch.values[55]) * ((scratch.values[69]) as f64).ln()));

        scratch.values[76] = ((scratch.values[9] * scratch.values[51]) - ((2.0 * scratch.values[55]) * ((scratch.values[70]) as f64).ln()));

        scratch.values[77] = ((scratch.values[10] * scratch.values[51]) - ((2.0 * scratch.values[55]) * ((scratch.values[71]) as f64).ln()));

        scratch.values[78] = (scratch.values[75] + (scratch.values[55] * (((1.0 + ((((0.05 - scratch.values[75]) * scratch.values[56])) as f64).exp())) as f64).ln()));

        scratch.values[79] = (scratch.values[76] + (scratch.values[55] * (((1.0 + ((((0.05 - scratch.values[76]) * scratch.values[56])) as f64).exp())) as f64).ln()));

        scratch.values[80] = (scratch.values[77] + (scratch.values[55] * (((1.0 + ((((0.05 - scratch.values[77]) * scratch.values[56])) as f64).exp())) as f64).ln()));

        scratch.values[90] = (1.0 / scratch.values[78]);

        scratch.values[91] = (1.0 / scratch.values[79]);

        scratch.values[92] = (1.0 / scratch.values[80]);

        scratch.values[93] = (1.0 - scratch.values[11]);

        scratch.values[94] = (1.0 - scratch.values[12]);

        scratch.values[95] = (1.0 - scratch.values[13]);

        scratch.values[96] = (1.0 / scratch.values[93]);

        scratch.values[97] = (1.0 / scratch.values[94]);

        scratch.values[98] = (1.0 / scratch.values[95]);

        scratch.values[99] = (scratch.values[5] * (((scratch.values[8] * scratch.values[90])) as f64).powf(scratch.values[11]));

        scratch.values[100] = (scratch.values[6] * (((scratch.values[9] * scratch.values[91])) as f64).powf(scratch.values[12]));

        scratch.values[101] = (scratch.values[7] * (((scratch.values[10] * scratch.values[92])) as f64).powf(scratch.values[13]));

        scratch.values[102] = ((scratch.values[99] * scratch.values[78]) * scratch.values[96]);

        scratch.values[103] = ((scratch.values[100] * scratch.values[79]) * scratch.values[97]);

        scratch.values[104] = ((scratch.values[101] * scratch.values[80]) * scratch.values[98]);

        scratch.values[105] = (2.0 * scratch.values[99]);

        scratch.values[106] = (2.0 * scratch.values[100]);

        scratch.values[107] = (2.0 * scratch.values[101]);

        scratch.values[108] = (scratch.values[1] / scratch.values[5]);

        scratch.values[109] = ((scratch.values[20] * scratch.values[1]) / scratch.values[6]);

        scratch.values[110] = ((scratch.values[21] * scratch.values[1]) / scratch.values[7]);

        scratch.values[111] = (1.0 / scratch.values[108]);

        scratch.values[112] = (1.0 / scratch.values[109]);

        scratch.values[113] = (1.0 / scratch.values[110]);

        scratch.values[114] = (1.0 / scratch.values[8]);

        scratch.values[115] = (1.0 / scratch.values[9]);

        scratch.values[116] = (1.0 / scratch.values[10]);

        scratch.values[117] = ((0.5 * scratch.values[66])).max(scratch.values[55]);

        scratch.values[118] = ((0.5 * scratch.values[67])).max(scratch.values[55]);

        scratch.values[119] = ((0.5 * scratch.values[68])).max(scratch.values[55]);

        scratch.values[120] = (scratch.values[117] * scratch.values[56]);

        scratch.values[121] = (scratch.values[118] * scratch.values[56]);

        scratch.values[122] = (scratch.values[119] * scratch.values[56]);

        scratch.values[123] = (((((((32.0 * scratch.values[28]) * 9.1093826e-31) * 1.6021918e-19) * ((scratch.values[117] * scratch.values[117]) * scratch.values[117]))) as f64).sqrt() / (3.0 * 1.05457168e-34));

        scratch.values[124] = (((((((32.0 * scratch.values[29]) * 9.1093826e-31) * 1.6021918e-19) * ((scratch.values[118] * scratch.values[118]) * scratch.values[118]))) as f64).sqrt() / (3.0 * 1.05457168e-34));

        scratch.values[125] = (((((((32.0 * scratch.values[30]) * 9.1093826e-31) * 1.6021918e-19) * ((scratch.values[119] * scratch.values[119]) * scratch.values[119]))) as f64).sqrt() / (3.0 * 1.05457168e-34));

        scratch.values[126] = (scratch.values[34] * (1.0 + (scratch.values[37] * (scratch.values[50] - scratch.values[49]))));

        scratch.values[127] = (scratch.values[35] * (1.0 + (scratch.values[38] * (scratch.values[50] - scratch.values[49]))));

        scratch.values[128] = (scratch.values[36] * (1.0 + (scratch.values[39] * (scratch.values[50] - scratch.values[49]))));

        if !(scratch.values[126] > 0.0) {
            scratch.values[126] = 0.0;
            scratch.node_derivatives[126] = [0.0; Instance::NODE_COUNT];
            scratch.branch_derivatives[126] = [0.0; Instance::BRANCH_COUNT];
        }

        if !(scratch.values[127] > 0.0) {
            scratch.values[127] = 0.0;
            scratch.node_derivatives[127] = [0.0; Instance::NODE_COUNT];
            scratch.branch_derivatives[127] = [0.0; Instance::BRANCH_COUNT];
        }

        if !(scratch.values[128] > 0.0) {
            scratch.values[128] = 0.0;
            scratch.node_derivatives[128] = [0.0; Instance::NODE_COUNT];
            scratch.branch_derivatives[128] = [0.0; Instance::BRANCH_COUNT];
        }

        scratch.values[129] = (1.0 - (1.0 / scratch.values[4]));

        scratch.values[133] = (1.0 / scratch.values[40]);

        scratch.values[134] = (1.0 / scratch.values[41]);

        scratch.values[135] = (1.0 / scratch.values[42]);

        scratch.values[217] = (if (self.params.ab > 0.0) { self.params.ab } else { 0.0 });

        scratch.values[218] = (if (self.params.ls > 0.0) { self.params.ls } else { 0.0 });

        scratch.values[219] = (if (self.params.lg > 0.0) { self.params.lg } else { 0.0 });

        scratch.values[0] = (if (self.params.mult > 0.0) { self.params.mult } else { 0.0 });

        scratch.values[224] = 0.0;

        scratch.values[262] = if ((scratch.values[72] * scratch.values[217]) > 0.0) { 1.0 } else { 0.0 };

        if (scratch.values[262] != 0.0) {
            scratch.values[139] = (scratch.values[55] * ((((scratch.values[3] / (scratch.values[72] * scratch.values[217])) + 1.0)) as f64).ln());
            scratch.node_derivatives[139] = [0.0; Instance::NODE_COUNT];
            scratch.branch_derivatives[139] = [0.0; Instance::BRANCH_COUNT];
        }

        if (!(scratch.values[262] != 0.0)) {
            scratch.values[139] = 100000000.0;
            scratch.node_derivatives[139] = [0.0; Instance::NODE_COUNT];
            scratch.branch_derivatives[139] = [0.0; Instance::BRANCH_COUNT];
        }

        scratch.values[263] = if ((scratch.values[73] * scratch.values[218]) > 0.0) { 1.0 } else { 0.0 };

        if (scratch.values[263] != 0.0) {
            scratch.values[140] = (scratch.values[55] * ((((scratch.values[3] / (scratch.values[73] * scratch.values[218])) + 1.0)) as f64).ln());
            scratch.node_derivatives[140] = [0.0; Instance::NODE_COUNT];
            scratch.branch_derivatives[140] = [0.0; Instance::BRANCH_COUNT];
        }

        if (!(scratch.values[263] != 0.0)) {
            scratch.values[140] = 100000000.0;
            scratch.node_derivatives[140] = [0.0; Instance::NODE_COUNT];
            scratch.branch_derivatives[140] = [0.0; Instance::BRANCH_COUNT];
        }

        scratch.values[264] = if ((scratch.values[74] * scratch.values[219]) > 0.0) { 1.0 } else { 0.0 };

        if (scratch.values[264] != 0.0) {
            scratch.values[141] = (scratch.values[55] * ((((scratch.values[3] / (scratch.values[74] * scratch.values[219])) + 1.0)) as f64).ln());
            scratch.node_derivatives[141] = [0.0; Instance::NODE_COUNT];
            scratch.branch_derivatives[141] = [0.0; Instance::BRANCH_COUNT];
        }

        if (!(scratch.values[264] != 0.0)) {
            scratch.values[141] = 100000000.0;
            scratch.node_derivatives[141] = [0.0; Instance::NODE_COUNT];
            scratch.branch_derivatives[141] = [0.0; Instance::BRANCH_COUNT];
        }

        scratch.store_ad(223, &AdValue::min(AdValue::min(scratch.ad_value(139), scratch.ad_value(140)), scratch.ad_value(141)));

        scratch.values[265] = if ((((scratch.values[223] * scratch.values[56])) as f64).abs() < 230.25850929940458) { 1.0 } else { 0.0 };

        if (scratch.values[265] != 0.0) {
            scratch.store_ad(224, &AdValue::exp(AdValue::scale(scratch.ad_value(223), scratch.values[56])));
        }

        scratch.values[266] = if ((scratch.values[223] * scratch.values[56]) < 0.0) { 1.0 } else { 0.0 };

        if ((!(scratch.values[265] != 0.0)) && (scratch.values[266] != 0.0)) {
            scratch.store_ad(224, &AdValue::div_from_scalar(1e-100, AdValue::offset(AdValue::mul(AdValue::sub_from_scalar((-230.25850929940458), AdValue::scale(scratch.ad_value(223), scratch.values[56])), AdValue::offset(AdValue::scale(AdValue::mul(AdValue::sub_from_scalar((-230.25850929940458), AdValue::scale(scratch.ad_value(223), scratch.values[56])), AdValue::offset(AdValue::scale(AdValue::sub_from_scalar((-230.25850929940458), AdValue::scale(scratch.ad_value(223), scratch.values[56])), 0.3333333333333333), 1.0)), 0.5), 1.0)), 1.0)));
        }

        if ((!(scratch.values[265] != 0.0)) && (!(scratch.values[266] != 0.0))) {
            scratch.store_ad(224, &AdValue::scale(AdValue::offset(AdValue::mul(AdValue::offset(AdValue::scale(scratch.ad_value(223), scratch.values[56]), (-230.25850929940458)), AdValue::offset(AdValue::scale(AdValue::mul(AdValue::offset(AdValue::scale(scratch.ad_value(223), scratch.values[56]), (-230.25850929940458)), AdValue::offset(AdValue::scale(AdValue::offset(AdValue::scale(scratch.ad_value(223), scratch.values[56]), (-230.25850929940458)), 0.3333333333333333), 1.0)), 0.5), 1.0)), 1.0), 1e100));
        }

        scratch.values[81] = scratch.values[78];

        scratch.values[82] = scratch.values[79];

        scratch.values[83] = scratch.values[80];

        scratch.values[84] = scratch.values[11];

        scratch.values[85] = scratch.values[12];

        scratch.values[86] = scratch.values[13];

        scratch.values[87] = scratch.values[8];

        scratch.values[88] = scratch.values[9];

        scratch.values[89] = scratch.values[10];

        scratch.values[267] = if (scratch.values[217] == 0.0) { 1.0 } else { 0.0 };

        if (scratch.values[267] != 0.0) {
            scratch.values[81] = (scratch.values[79] + scratch.values[80]);
            scratch.node_derivatives[81] = [0.0; Instance::NODE_COUNT];
            scratch.branch_derivatives[81] = [0.0; Instance::BRANCH_COUNT];
        }

        if (scratch.values[267] != 0.0) {
            scratch.values[84] = (0.9 * (scratch.values[12]).min(scratch.values[13]));
            scratch.node_derivatives[84] = [0.0; Instance::NODE_COUNT];
            scratch.branch_derivatives[84] = [0.0; Instance::BRANCH_COUNT];
        }

        if (scratch.values[267] != 0.0) {
            scratch.values[87] = (scratch.values[9] + scratch.values[10]);
            scratch.node_derivatives[87] = [0.0; Instance::NODE_COUNT];
            scratch.branch_derivatives[87] = [0.0; Instance::BRANCH_COUNT];
        }

        scratch.values[268] = if (scratch.values[218] == 0.0) { 1.0 } else { 0.0 };

        if (scratch.values[268] != 0.0) {
            scratch.values[82] = (scratch.values[78] + scratch.values[80]);
            scratch.node_derivatives[82] = [0.0; Instance::NODE_COUNT];
            scratch.branch_derivatives[82] = [0.0; Instance::BRANCH_COUNT];
        }

        if (scratch.values[268] != 0.0) {
            scratch.values[85] = (0.9 * (scratch.values[11]).min(scratch.values[13]));
            scratch.node_derivatives[85] = [0.0; Instance::NODE_COUNT];
            scratch.branch_derivatives[85] = [0.0; Instance::BRANCH_COUNT];
        }

        if (scratch.values[268] != 0.0) {
            scratch.values[88] = (scratch.values[8] + scratch.values[10]);
            scratch.node_derivatives[88] = [0.0; Instance::NODE_COUNT];
            scratch.branch_derivatives[88] = [0.0; Instance::BRANCH_COUNT];
        }

        scratch.values[269] = if (scratch.values[219] == 0.0) { 1.0 } else { 0.0 };

        if (scratch.values[269] != 0.0) {
            scratch.values[83] = (scratch.values[78] + scratch.values[79]);
            scratch.node_derivatives[83] = [0.0; Instance::NODE_COUNT];
            scratch.branch_derivatives[83] = [0.0; Instance::BRANCH_COUNT];
        }

        if (scratch.values[269] != 0.0) {
            scratch.values[86] = (0.9 * (scratch.values[11]).min(scratch.values[12]));
            scratch.node_derivatives[86] = [0.0; Instance::NODE_COUNT];
            scratch.branch_derivatives[86] = [0.0; Instance::BRANCH_COUNT];
        }

        if (scratch.values[269] != 0.0) {
            scratch.values[89] = (scratch.values[8] + scratch.values[9]);
            scratch.node_derivatives[89] = [0.0; Instance::NODE_COUNT];
            scratch.branch_derivatives[89] = [0.0; Instance::BRANCH_COUNT];
        }

        scratch.store_ad(225, &AdValue::min(AdValue::min(scratch.ad_value(81), scratch.ad_value(82)), scratch.ad_value(83)));

        scratch.store_ad(226, &AdValue::scale(scratch.ad_value(225), 0.1));

        scratch.store_ad(62, &AdValue::max(AdValue::max(scratch.ad_value(84), scratch.ad_value(85)), scratch.ad_value(86)));

        scratch.store_ad(227, &AdValue::mul(scratch.ad_value(225), AdValue::sub_from_scalar(1.0, AdValue::pow_from_scalar(2.0, AdValue::div_from_scalar((-1.0), scratch.ad_value(62))))));

        scratch.store_ad(228, &AdValue::offset(AdValue::min(AdValue::min(scratch.ad_value(87), scratch.ad_value(88)), scratch.ad_value(89)), (-0.05)));

        scratch.values[168] = 0.0;

        scratch.values[220] = 1.0;

        scratch.values[221] = 1.0;

        scratch.values[222] = 1.0;

        scratch.values[270] = if (scratch.values[46] == 1.0) { 1.0 } else { 0.0 };

        if (scratch.values[270] != 0.0) {
            scratch.values[168] = (scratch.values[48] * (((scratch.values[217] * scratch.values[99]) + (scratch.values[218] * scratch.values[100])) + (scratch.values[219] * scratch.values[101])));
            scratch.node_derivatives[168] = [0.0; Instance::NODE_COUNT];
            scratch.branch_derivatives[168] = [0.0; Instance::BRANCH_COUNT];
        }

        scratch.values[623] = if ((scratch.values[217] * scratch.values[99]) <= scratch.values[168]) { 1.0 } else { 0.0 };

        if ((scratch.values[270] != 0.0) && (scratch.values[623] != 0.0)) {
            scratch.values[220] = 0.0;
            scratch.node_derivatives[220] = [0.0; Instance::NODE_COUNT];
            scratch.branch_derivatives[220] = [0.0; Instance::BRANCH_COUNT];
        }

        scratch.values[624] = if ((scratch.values[218] * scratch.values[100]) <= scratch.values[168]) { 1.0 } else { 0.0 };

        if ((scratch.values[270] != 0.0) && (scratch.values[624] != 0.0)) {
            scratch.values[221] = 0.0;
            scratch.node_derivatives[221] = [0.0; Instance::NODE_COUNT];
            scratch.branch_derivatives[221] = [0.0; Instance::BRANCH_COUNT];
        }

        scratch.values[625] = if ((scratch.values[219] * scratch.values[101]) <= scratch.values[168]) { 1.0 } else { 0.0 };

        if ((scratch.values[270] != 0.0) && (scratch.values[625] != 0.0)) {
            scratch.values[222] = 0.0;
            scratch.node_derivatives[222] = [0.0; Instance::NODE_COUNT];
            scratch.branch_derivatives[222] = [0.0; Instance::BRANCH_COUNT];
        }

        scratch.store_ad(636, &AdValue::scale(AdValue::voltage(ctx, &self.nodes, Some(0), Some(1)), self.params.type_));

        scratch.values[637] = if (scratch.values[46] == 1.0) { 1.0 } else { 0.0 };

        if (scratch.values[637] != 0.0) {
            scratch.values[639] = 0.0;
            scratch.node_derivatives[639] = [0.0; Instance::NODE_COUNT];
            scratch.branch_derivatives[639] = [0.0; Instance::BRANCH_COUNT];
        }

        if (scratch.values[637] != 0.0) {
            scratch.values[640] = 0.0;
            scratch.node_derivatives[640] = [0.0; Instance::NODE_COUNT];
            scratch.branch_derivatives[640] = [0.0; Instance::BRANCH_COUNT];
        }

        if (scratch.values[637] != 0.0) {
            scratch.store_ad(175, &AdValue::mul(AdValue::scale(scratch.ad_value(226), 4.0), scratch.ad_value(226)));
        }

        if (scratch.values[637] != 0.0) {
            scratch.store_ad(176, &AdValue::div(scratch.ad_value(226), scratch.ad_value(227)));
        }

        if (scratch.values[637] != 0.0) {
            scratch.store_ad(177, &AdValue::add(scratch.ad_value(636), AdValue::mul(scratch.ad_value(226), scratch.ad_value(176))));
        }

        if (scratch.values[637] != 0.0) {
            scratch.store_ad(178, &AdValue::add(scratch.ad_value(227), scratch.ad_value(177)));
        }

        if (scratch.values[637] != 0.0) {
            scratch.store_ad(179, &AdValue::sub(scratch.ad_value(227), scratch.ad_value(177)));
        }

        if (scratch.values[637] != 0.0) {
            scratch.store_ad(180, &AdValue::sqrt(AdValue::add(AdValue::square(scratch.ad_value(179)), scratch.ad_value(175))));
        }

        if (scratch.values[637] != 0.0) {
            scratch.store_ad(640, &AdValue::scale(AdValue::div(AdValue::mul(scratch.ad_value(636), scratch.ad_value(227)), AdValue::add(scratch.ad_value(178), scratch.ad_value(180))), 2.0));
        }

        scratch.values[641] = if (scratch.values[220] > 0.5) { 1.0 } else { 0.0 };

    }

    pub(super) fn stamp_reactive_block_1(
        &mut self,
        ctx: &GeneratedEvalContext<'_>,
        stamper: &mut GeneratedReactiveStamper<'_>,
        scratch: &mut ReactiveScratch,
    ) {
        let _ = stamper;
        scratch.values[642] = if (scratch.values[93] == 0.5) { 1.0 } else { 0.0 };

        if (((scratch.values[637] != 0.0) && (scratch.values[641] != 0.0)) && (scratch.values[642] != 0.0)) {
            scratch.store_ad(639, &AdValue::sqrt(AdValue::sub_from_scalar(1.0, AdValue::scale(scratch.ad_value(640), scratch.values[90]))));
        }

        if (((scratch.values[637] != 0.0) && (scratch.values[641] != 0.0)) && (!(scratch.values[642] != 0.0))) {
            scratch.store_ad(639, &AdValue::powf(AdValue::sub_from_scalar(1.0, AdValue::scale(scratch.ad_value(640), scratch.values[90])), scratch.values[93]));
        }

        if ((scratch.values[637] != 0.0) && (scratch.values[641] != 0.0)) {
            scratch.store_ad(628, &AdValue::add(AdValue::scale(AdValue::sub_from_scalar(1.0, scratch.ad_value(639)), scratch.values[102]), AdValue::scale(AdValue::sub(scratch.ad_value(636), scratch.ad_value(640)), scratch.values[105])));
        }

        scratch.values[643] = if (scratch.values[221] > 0.5) { 1.0 } else { 0.0 };

        scratch.values[644] = if (scratch.values[94] == 0.5) { 1.0 } else { 0.0 };

        if (((scratch.values[637] != 0.0) && (scratch.values[643] != 0.0)) && (scratch.values[644] != 0.0)) {
            scratch.store_ad(639, &AdValue::sqrt(AdValue::sub_from_scalar(1.0, AdValue::scale(scratch.ad_value(640), scratch.values[91]))));
        }

        if (((scratch.values[637] != 0.0) && (scratch.values[643] != 0.0)) && (!(scratch.values[644] != 0.0))) {
            scratch.store_ad(639, &AdValue::powf(AdValue::sub_from_scalar(1.0, AdValue::scale(scratch.ad_value(640), scratch.values[91])), scratch.values[94]));
        }

        if ((scratch.values[637] != 0.0) && (scratch.values[643] != 0.0)) {
            scratch.store_ad(630, &AdValue::add(AdValue::scale(AdValue::sub_from_scalar(1.0, scratch.ad_value(639)), scratch.values[103]), AdValue::scale(AdValue::sub(scratch.ad_value(636), scratch.ad_value(640)), scratch.values[106])));
        }

        scratch.values[645] = if (scratch.values[222] > 0.5) { 1.0 } else { 0.0 };

        scratch.values[646] = if (scratch.values[95] == 0.5) { 1.0 } else { 0.0 };

        if (((scratch.values[637] != 0.0) && (scratch.values[645] != 0.0)) && (scratch.values[646] != 0.0)) {
            scratch.store_ad(639, &AdValue::sqrt(AdValue::sub_from_scalar(1.0, AdValue::scale(scratch.ad_value(640), scratch.values[92]))));
        }

        if (((scratch.values[637] != 0.0) && (scratch.values[645] != 0.0)) && (!(scratch.values[646] != 0.0))) {
            scratch.store_ad(639, &AdValue::powf(AdValue::sub_from_scalar(1.0, AdValue::scale(scratch.ad_value(640), scratch.values[92])), scratch.values[95]));
        }

        if ((scratch.values[637] != 0.0) && (scratch.values[645] != 0.0)) {
            scratch.store_ad(632, &AdValue::add(AdValue::scale(AdValue::sub_from_scalar(1.0, scratch.ad_value(639)), scratch.values[104]), AdValue::scale(AdValue::sub(scratch.ad_value(636), scratch.ad_value(640)), scratch.values[107])));
        }

        if (!(scratch.values[637] != 0.0)) {
            scratch.values[188] = 0.0;
            scratch.node_derivatives[188] = [0.0; Instance::NODE_COUNT];
            scratch.branch_derivatives[188] = [0.0; Instance::BRANCH_COUNT];
        }

        if (!(scratch.values[637] != 0.0)) {
            scratch.values[185] = 0.0;
            scratch.node_derivatives[185] = [0.0; Instance::NODE_COUNT];
            scratch.branch_derivatives[185] = [0.0; Instance::BRANCH_COUNT];
        }

        scratch.values[647] = if !(((scratch.values[217] == 0.0) && (scratch.values[218] == 0.0)) && (scratch.values[219] == 0.0)) { 1.0 } else { 0.0 };

        if ((!(scratch.values[637] != 0.0)) && (scratch.values[647] != 0.0)) {
            scratch.store_ad(175, &AdValue::mul(AdValue::scale(scratch.ad_value(226), 4.0), scratch.ad_value(226)));
        }

        if ((!(scratch.values[637] != 0.0)) && (scratch.values[647] != 0.0)) {
            scratch.store_ad(176, &AdValue::div(scratch.ad_value(226), scratch.ad_value(227)));
        }

        if ((!(scratch.values[637] != 0.0)) && (scratch.values[647] != 0.0)) {
            scratch.store_ad(177, &AdValue::add(scratch.ad_value(636), AdValue::mul(scratch.ad_value(226), scratch.ad_value(176))));
        }

        if ((!(scratch.values[637] != 0.0)) && (scratch.values[647] != 0.0)) {
            scratch.store_ad(178, &AdValue::add(scratch.ad_value(227), scratch.ad_value(177)));
        }

        if ((!(scratch.values[637] != 0.0)) && (scratch.values[647] != 0.0)) {
            scratch.store_ad(179, &AdValue::sub(scratch.ad_value(227), scratch.ad_value(177)));
        }

        if ((!(scratch.values[637] != 0.0)) && (scratch.values[647] != 0.0)) {
            scratch.store_ad(180, &AdValue::sqrt(AdValue::add(AdValue::square(scratch.ad_value(179)), scratch.ad_value(175))));
        }

        if ((!(scratch.values[637] != 0.0)) && (scratch.values[647] != 0.0)) {
            scratch.store_ad(182, &AdValue::scale(AdValue::div(AdValue::mul(scratch.ad_value(636), scratch.ad_value(227)), AdValue::add(scratch.ad_value(178), scratch.ad_value(180))), 2.0));
        }

        scratch.values[648] = if (scratch.values[636] < scratch.values[223]) { 1.0 } else { 0.0 };

        scratch.values[649] = if ((((0.5 * (scratch.values[636] * scratch.values[56]))) as f64).abs() < 230.25850929940458) { 1.0 } else { 0.0 };

        if ((((!(scratch.values[637] != 0.0)) && (scratch.values[647] != 0.0)) && (scratch.values[648] != 0.0)) && (scratch.values[649] != 0.0)) {
            scratch.store_ad(184, &AdValue::exp(AdValue::scale(scratch.ad_value(636), (scratch.values[56] * 0.5))));
        }

        scratch.values[650] = if ((0.5 * (scratch.values[636] * scratch.values[56])) < 0.0) { 1.0 } else { 0.0 };

        if (((((!(scratch.values[637] != 0.0)) && (scratch.values[647] != 0.0)) && (scratch.values[648] != 0.0)) && (!(scratch.values[649] != 0.0))) && (scratch.values[650] != 0.0)) {
            let assign16370_ad_e21135: AdValue = AdValue::div_from_scalar(1e-100, AdValue::offset(AdValue::mul(AdValue::sub_from_scalar((-230.25850929940458), AdValue::scale(scratch.ad_value(636), (scratch.values[56] * 0.5))), AdValue::offset(AdValue::scale(AdValue::mul(AdValue::sub_from_scalar((-230.25850929940458), AdValue::scale(scratch.ad_value(636), (scratch.values[56] * 0.5))), AdValue::offset(AdValue::scale(AdValue::sub_from_scalar((-230.25850929940458), AdValue::scale(scratch.ad_value(636), (scratch.values[56] * 0.5))), 0.3333333333333333), 1.0)), 0.5), 1.0)), 1.0));
            scratch.store_ad(184, &assign16370_ad_e21135);
        }

        if (((((!(scratch.values[637] != 0.0)) && (scratch.values[647] != 0.0)) && (scratch.values[648] != 0.0)) && (!(scratch.values[649] != 0.0))) && (!(scratch.values[650] != 0.0))) {
            scratch.store_ad(184, &AdValue::scale(AdValue::offset(AdValue::mul(AdValue::offset(AdValue::scale(scratch.ad_value(636), (scratch.values[56] * 0.5)), (-230.25850929940458)), AdValue::offset(AdValue::scale(AdValue::mul(AdValue::offset(AdValue::scale(scratch.ad_value(636), (scratch.values[56] * 0.5)), (-230.25850929940458)), AdValue::offset(AdValue::scale(AdValue::offset(AdValue::scale(scratch.ad_value(636), (scratch.values[56] * 0.5)), (-230.25850929940458)), 0.3333333333333333), 1.0)), 0.5), 1.0)), 1.0), 1e100));
        }

        if (((!(scratch.values[637] != 0.0)) && (scratch.values[647] != 0.0)) && (scratch.values[648] != 0.0)) {
            scratch.store_ad(181, &AdValue::square(scratch.ad_value(184)));
        }

        if (((!(scratch.values[637] != 0.0)) && (scratch.values[647] != 0.0)) && (!(scratch.values[648] != 0.0))) {
            scratch.store_ad(181, &AdValue::mul(AdValue::offset(AdValue::scale(AdValue::sub(scratch.ad_value(636), scratch.ad_value(223)), scratch.values[56]), 1.0), scratch.ad_value(224)));
        }

        if (((!(scratch.values[637] != 0.0)) && (scratch.values[647] != 0.0)) && (!(scratch.values[648] != 0.0))) {
            scratch.store_ad(184, &AdValue::sqrt(scratch.ad_value(181)));
        }

        if ((!(scratch.values[637] != 0.0)) && (scratch.values[647] != 0.0)) {
            scratch.store_ad(181, &AdValue::offset(scratch.ad_value(181), (-1.0)));
        }

        if ((!(scratch.values[637] != 0.0)) && (scratch.values[647] != 0.0)) {
            scratch.store_ad(183, &AdValue::div_from_scalar(1.0, scratch.ad_value(184)));
        }

        scratch.values[651] = if (scratch.values[636] > 0.0) { 1.0 } else { 0.0 };

        if (((!(scratch.values[637] != 0.0)) && (scratch.values[647] != 0.0)) && (scratch.values[651] != 0.0)) {
            scratch.store_ad(185, &AdValue::scale(AdValue::ln(AdValue::add(AdValue::offset(scratch.ad_value(183), 2.0), AdValue::sqrt(AdValue::mul(AdValue::offset(scratch.ad_value(183), 1.0), AdValue::offset(scratch.ad_value(183), 3.0))))), (scratch.values[55] * 2.0)));
        }

        if (((!(scratch.values[637] != 0.0)) && (scratch.values[647] != 0.0)) && (!(scratch.values[651] != 0.0))) {
            scratch.store_ad(185, &AdValue::sub(AdValue::scale(AdValue::ln(AdValue::add(AdValue::offset(AdValue::scale(scratch.ad_value(184), 2.0), 1.0), AdValue::sqrt(AdValue::mul(AdValue::offset(scratch.ad_value(184), 1.0), AdValue::offset(AdValue::scale(scratch.ad_value(184), 3.0), 1.0))))), (scratch.values[55] * 2.0)), scratch.ad_value(636)));
        }

        if ((!(scratch.values[637] != 0.0)) && (scratch.values[647] != 0.0)) {
            scratch.store_ad(186, &AdValue::sub(scratch.ad_value(225), scratch.ad_value(185)));
        }

        if ((!(scratch.values[637] != 0.0)) && (scratch.values[647] != 0.0)) {
            scratch.store_ad(187, &AdValue::scale(AdValue::sub(AdValue::add(scratch.ad_value(636), scratch.ad_value(186)), AdValue::sqrt(AdValue::offset(AdValue::mul(AdValue::sub(scratch.ad_value(636), scratch.ad_value(186)), AdValue::sub(scratch.ad_value(636), scratch.ad_value(186))), ((4.0 * scratch.values[55]) * scratch.values[55])))), 0.5));
        }

        if ((!(scratch.values[637] != 0.0)) && (scratch.values[647] != 0.0)) {
            scratch.store_ad(188, &AdValue::scale(AdValue::sub(AdValue::add(scratch.ad_value(636), scratch.ad_value(228)), AdValue::sqrt(AdValue::offset(AdValue::mul(AdValue::sub(scratch.ad_value(636), scratch.ad_value(228)), AdValue::sub(scratch.ad_value(636), scratch.ad_value(228))), ((4.0 * scratch.values[53]) * scratch.values[53])))), 0.5));
        }

        if ((!(scratch.values[637] != 0.0)) && (scratch.values[647] != 0.0)) {
            scratch.store_ad(189, &AdValue::scale(AdValue::sub(scratch.ad_value(636), AdValue::sqrt(AdValue::offset(AdValue::mul(scratch.ad_value(636), scratch.ad_value(636)), ((4.0 * 1e-6) * 1e-6)))), 0.5));
        }

        scratch.values[652] = if (scratch.values[217] == 0.0) { 1.0 } else { 0.0 };

        if ((!(scratch.values[637] != 0.0)) && (scratch.values[652] != 0.0)) {
            scratch.values[628] = 0.0;
            scratch.node_derivatives[628] = [0.0; Instance::NODE_COUNT];
            scratch.branch_derivatives[628] = [0.0; Instance::BRANCH_COUNT];
        }

        scratch.values[653] = if (scratch.values[93] == 0.5) { 1.0 } else { 0.0 };

        if (((!(scratch.values[637] != 0.0)) && (!(scratch.values[652] != 0.0))) && (scratch.values[653] != 0.0)) {
            scratch.store_ad(190, &AdValue::sqrt(AdValue::sub_from_scalar(1.0, AdValue::scale(scratch.ad_value(182), scratch.values[90]))));
        }

        if (((!(scratch.values[637] != 0.0)) && (!(scratch.values[652] != 0.0))) && (!(scratch.values[653] != 0.0))) {
            scratch.store_ad(190, &AdValue::powf(AdValue::sub_from_scalar(1.0, AdValue::scale(scratch.ad_value(182), scratch.values[90])), scratch.values[93]));
        }

        if ((!(scratch.values[637] != 0.0)) && (!(scratch.values[652] != 0.0))) {
            scratch.store_ad(628, &AdValue::add(AdValue::scale(AdValue::sub_from_scalar(1.0, scratch.ad_value(190)), scratch.values[102]), AdValue::scale(AdValue::sub(scratch.ad_value(636), scratch.ad_value(182)), scratch.values[105])));
        }

        scratch.values[654] = if ((scratch.values[22] == 0.0) && (scratch.values[25] == 0.0)) { 1.0 } else { 0.0 };

        if (((!(scratch.values[637] != 0.0)) && (!(scratch.values[652] != 0.0))) && (!(scratch.values[654] != 0.0))) {
            scratch.store_ad(193, &AdValue::sub_from_scalar(scratch.values[78], scratch.ad_value(187)));
        }

        scratch.values[656] = if (scratch.values[11] == 0.5) { 1.0 } else { 0.0 };

        if ((((!(scratch.values[637] != 0.0)) && (!(scratch.values[652] != 0.0))) && (!(scratch.values[654] != 0.0))) && (scratch.values[656] != 0.0)) {
            scratch.store_ad(190, &AdValue::sqrt(AdValue::scale(scratch.ad_value(193), scratch.values[114])));
        }

        if ((((!(scratch.values[637] != 0.0)) && (!(scratch.values[652] != 0.0))) && (!(scratch.values[654] != 0.0))) && (!(scratch.values[656] != 0.0))) {
            scratch.store_ad(190, &AdValue::powf(AdValue::scale(scratch.ad_value(193), scratch.values[114]), scratch.values[11]));
        }

        if (((!(scratch.values[637] != 0.0)) && (!(scratch.values[652] != 0.0))) && (!(scratch.values[654] != 0.0))) {
            scratch.store_ad(197, &AdValue::scale(scratch.ad_value(190), scratch.values[108]));
        }

        scratch.values[657] = if (scratch.values[25] == 0.0) { 1.0 } else { 0.0 };

        if (((!(scratch.values[637] != 0.0)) && (!(scratch.values[652] != 0.0))) && (!(scratch.values[657] != 0.0))) {
            scratch.store_ad(200, &AdValue::scale(AdValue::div(AdValue::scale(scratch.ad_value(197), scratch.values[93]), scratch.ad_value(193)), scratch.values[123]));
        }

        if (((!(scratch.values[637] != 0.0)) && (!(scratch.values[652] != 0.0))) && (!(scratch.values[657] != 0.0))) {
            scratch.store_ad(201, &AdValue::div_from_scalar((0.666666666666667 * scratch.values[120]), scratch.ad_value(200)));
        }

        if (((!(scratch.values[637] != 0.0)) && (!(scratch.values[652] != 0.0))) && (!(scratch.values[657] != 0.0))) {
            scratch.store_ad(202, &AdValue::square(scratch.ad_value(201)));
        }

        if (((!(scratch.values[637] != 0.0)) && (!(scratch.values[652] != 0.0))) && (!(scratch.values[657] != 0.0))) {
            scratch.store_ad(203, &AdValue::sqrt(AdValue::div(AdValue::square(scratch.ad_value(202)), AdValue::offset(AdValue::square(scratch.ad_value(202)), 1.0))));
        }

        if (((!(scratch.values[637] != 0.0)) && (!(scratch.values[652] != 0.0))) && (!(scratch.values[657] != 0.0))) {
            scratch.store_ad(204, &AdValue::sqrt(AdValue::abs(scratch.ad_value(203))));
        }

        if (((!(scratch.values[637] != 0.0)) && (!(scratch.values[652] != 0.0))) && (!(scratch.values[657] != 0.0))) {
            scratch.store_ad(205, &AdValue::mul(scratch.ad_value(203), scratch.ad_value(204)));
        }

        if (((!(scratch.values[637] != 0.0)) && (!(scratch.values[652] != 0.0))) && (!(scratch.values[657] != 0.0))) {
            scratch.store_ad(208, &AdValue::sqrt(AdValue::scale(AdValue::div(scratch.ad_value(200), scratch.ad_value(204)), 0.375)));
        }

        if (((!(scratch.values[637] != 0.0)) && (!(scratch.values[652] != 0.0))) && (!(scratch.values[657] != 0.0))) {
            scratch.store_ad(209, &AdValue::sub(AdValue::scale(AdValue::mul(scratch.ad_value(201), scratch.ad_value(204)), 2.0), scratch.ad_value(203)));
        }

        if (((!(scratch.values[637] != 0.0)) && (!(scratch.values[652] != 0.0))) && (!(scratch.values[657] != 0.0))) {
            scratch.store_ad(210, &AdValue::add(AdValue::sub(AdValue::mul(AdValue::scale(scratch.ad_value(201), scratch.values[120]), scratch.ad_value(204)), AdValue::scale(scratch.ad_value(203), scratch.values[120])), AdValue::scale(AdValue::mul(scratch.ad_value(200), scratch.ad_value(205)), 0.5)));
        }

        if (((!(scratch.values[637] != 0.0)) && (!(scratch.values[652] != 0.0))) && (!(scratch.values[657] != 0.0))) {
            scratch.store_ad(211, &AdValue::mul(AdValue::offset(scratch.ad_value(209), (-1.0)), scratch.ad_value(208)));
        }

        if (((!(scratch.values[637] != 0.0)) && (!(scratch.values[652] != 0.0))) && (!(scratch.values[657] != 0.0))) {
            scratch.store_ad(172, &AdValue::square(scratch.ad_value(211)));
        }

        scratch.values[660] = if (((-scratch.values[172]) + scratch.values[210]) > (-230.25850929940458)) { 1.0 } else { 0.0 };

        if ((((!(scratch.values[637] != 0.0)) && (!(scratch.values[652] != 0.0))) && (!(scratch.values[657] != 0.0))) && (scratch.values[660] != 0.0)) {
            scratch.store_ad(190, &AdValue::exp(AdValue::sub(scratch.ad_value(210), scratch.ad_value(172))));
        }

        if ((((!(scratch.values[637] != 0.0)) && (!(scratch.values[652] != 0.0))) && (!(scratch.values[657] != 0.0))) && (!(scratch.values[660] != 0.0))) {
            scratch.store_ad(190, &AdValue::div_from_scalar(1e-100, AdValue::offset(AdValue::mul(AdValue::sub_from_scalar((-230.25850929940458), AdValue::sub(scratch.ad_value(210), scratch.ad_value(172))), AdValue::offset(AdValue::scale(AdValue::mul(AdValue::sub_from_scalar((-230.25850929940458), AdValue::sub(scratch.ad_value(210), scratch.ad_value(172))), AdValue::offset(AdValue::scale(AdValue::sub_from_scalar((-230.25850929940458), AdValue::sub(scratch.ad_value(210), scratch.ad_value(172))), 0.3333333333333333), 1.0)), 0.5), 1.0)), 1.0)));
        }

        scratch.values[661] = if (scratch.values[211] > 0.0) { 1.0 } else { 0.0 };

        scratch.values[662] = if (scratch.values[210] > (-230.25850929940458)) { 1.0 } else { 0.0 };

        if (((((!(scratch.values[637] != 0.0)) && (!(scratch.values[652] != 0.0))) && (!(scratch.values[657] != 0.0))) && (!(scratch.values[661] != 0.0))) && (scratch.values[662] != 0.0)) {
            scratch.store_ad(190, &AdValue::exp(scratch.ad_value(210)));
        }

        if (((((!(scratch.values[637] != 0.0)) && (!(scratch.values[652] != 0.0))) && (!(scratch.values[657] != 0.0))) && (!(scratch.values[661] != 0.0))) && (!(scratch.values[662] != 0.0))) {
            scratch.store_ad(190, &AdValue::div_from_scalar(1e-100, AdValue::offset(AdValue::mul(AdValue::sub_from_scalar((-230.25850929940458), scratch.ad_value(210)), AdValue::offset(AdValue::scale(AdValue::mul(AdValue::sub_from_scalar((-230.25850929940458), scratch.ad_value(210)), AdValue::offset(AdValue::scale(AdValue::sub_from_scalar((-230.25850929940458), scratch.ad_value(210)), 0.3333333333333333), 1.0)), 0.5), 1.0)), 1.0)));
        }

        scratch.values[663] = if (scratch.values[31] == 0.0) { 1.0 } else { 0.0 };

        scratch.values[664] = if (scratch.values[11] == 0.5) { 1.0 } else { 0.0 };

        if ((((!(scratch.values[637] != 0.0)) && (!(scratch.values[652] != 0.0))) && (!(scratch.values[663] != 0.0))) && (scratch.values[664] != 0.0)) {
            scratch.store_ad(190, &AdValue::sqrt(AdValue::scale(AdValue::sub_from_scalar(scratch.values[8], scratch.ad_value(188)), scratch.values[114])));
        }

        if ((((!(scratch.values[637] != 0.0)) && (!(scratch.values[652] != 0.0))) && (!(scratch.values[663] != 0.0))) && (!(scratch.values[664] != 0.0))) {
            scratch.store_ad(190, &AdValue::powf(AdValue::scale(AdValue::sub_from_scalar(scratch.values[8], scratch.ad_value(188)), scratch.values[114]), scratch.values[11]));
        }

        if (((!(scratch.values[637] != 0.0)) && (!(scratch.values[652] != 0.0))) && (!(scratch.values[663] != 0.0))) {
            scratch.store_ad(215, &AdValue::scale(AdValue::div(AdValue::scale(AdValue::sub_from_scalar(scratch.values[8], scratch.ad_value(188)), scratch.values[111]), scratch.ad_value(190)), scratch.values[96]));
        }

        scratch.values[665] = if (((((-scratch.values[126]) / scratch.values[215])) as f64).abs() < 230.25850929940458) { 1.0 } else { 0.0 };

        if ((((!(scratch.values[637] != 0.0)) && (!(scratch.values[652] != 0.0))) && (!(scratch.values[663] != 0.0))) && (scratch.values[665] != 0.0)) {
            scratch.store_ad(190, &AdValue::exp(AdValue::div(AdValue::neg(scratch.ad_value(126)), scratch.ad_value(215))));
        }

        scratch.values[666] = if (((-scratch.values[126]) / scratch.values[215]) < 0.0) { 1.0 } else { 0.0 };

        if (((((!(scratch.values[637] != 0.0)) && (!(scratch.values[652] != 0.0))) && (!(scratch.values[663] != 0.0))) && (!(scratch.values[665] != 0.0))) && (scratch.values[666] != 0.0)) {
            let assign17140_ad_e22344: AdValue = AdValue::offset(AdValue::mul(AdValue::sub_from_scalar((-230.25850929940458), AdValue::div(AdValue::neg(scratch.ad_value(126)), scratch.ad_value(215))), AdValue::offset(AdValue::scale(AdValue::mul(AdValue::sub_from_scalar((-230.25850929940458), AdValue::div(AdValue::neg(scratch.ad_value(126)), scratch.ad_value(215))), AdValue::offset(AdValue::scale(AdValue::sub_from_scalar((-230.25850929940458), AdValue::div(AdValue::neg(scratch.ad_value(126)), scratch.ad_value(215))), 0.3333333333333333), 1.0)), 0.5), 1.0)), 1.0);
            scratch.store_ad(190, &AdValue::div_from_scalar(1e-100, assign17140_ad_e22344));
        }

        if (((((!(scratch.values[637] != 0.0)) && (!(scratch.values[652] != 0.0))) && (!(scratch.values[663] != 0.0))) && (!(scratch.values[665] != 0.0))) && (!(scratch.values[666] != 0.0))) {
            let assign17150_ad_e22393: AdValue = AdValue::scale(AdValue::offset(AdValue::mul(AdValue::offset(AdValue::div(AdValue::neg(scratch.ad_value(126)), scratch.ad_value(215)), (-230.25850929940458)), AdValue::offset(AdValue::scale(AdValue::mul(AdValue::offset(AdValue::div(AdValue::neg(scratch.ad_value(126)), scratch.ad_value(215)), (-230.25850929940458)), AdValue::offset(AdValue::scale(AdValue::offset(AdValue::div(AdValue::neg(scratch.ad_value(126)), scratch.ad_value(215)), (-230.25850929940458)), 0.3333333333333333), 1.0)), 0.5), 1.0)), 1.0), 1e100);
            scratch.store_ad(190, &assign17150_ad_e22393);
        }

        scratch.values[667] = if (scratch.values[40] > 1000.0) { 1.0 } else { 0.0 };

        scratch.values[668] = if (scratch.values[189] > ((-scratch.values[129]) * scratch.values[40])) { 1.0 } else { 0.0 };

        scratch.values[669] = if (scratch.values[43] == 4.0) { 1.0 } else { 0.0 };

        if (((((!(scratch.values[637] != 0.0)) && (!(scratch.values[652] != 0.0))) && (!(scratch.values[667] != 0.0))) && (scratch.values[668] != 0.0)) && (scratch.values[669] != 0.0)) {
            scratch.store_ad(190, &AdValue::mul(AdValue::mul(AdValue::mul(AdValue::scale(scratch.ad_value(189), scratch.values[133]), AdValue::scale(scratch.ad_value(189), scratch.values[133])), AdValue::scale(scratch.ad_value(189), scratch.values[133])), AdValue::scale(scratch.ad_value(189), scratch.values[133])));
        }

        if (((((!(scratch.values[637] != 0.0)) && (!(scratch.values[652] != 0.0))) && (!(scratch.values[667] != 0.0))) && (scratch.values[668] != 0.0)) && (!(scratch.values[669] != 0.0))) {
            scratch.store_ad(190, &AdValue::powf(AdValue::abs(AdValue::scale(scratch.ad_value(189), scratch.values[133])), scratch.values[43]));
        }

        scratch.values[670] = if (scratch.values[218] == 0.0) { 1.0 } else { 0.0 };

        if ((!(scratch.values[637] != 0.0)) && (scratch.values[670] != 0.0)) {
            scratch.values[630] = 0.0;
            scratch.node_derivatives[630] = [0.0; Instance::NODE_COUNT];
            scratch.branch_derivatives[630] = [0.0; Instance::BRANCH_COUNT];
        }

        scratch.values[671] = if (scratch.values[94] == 0.5) { 1.0 } else { 0.0 };

        if (((!(scratch.values[637] != 0.0)) && (!(scratch.values[670] != 0.0))) && (scratch.values[671] != 0.0)) {
            scratch.store_ad(190, &AdValue::sqrt(AdValue::sub_from_scalar(1.0, AdValue::scale(scratch.ad_value(182), scratch.values[91]))));
        }

        if (((!(scratch.values[637] != 0.0)) && (!(scratch.values[670] != 0.0))) && (!(scratch.values[671] != 0.0))) {
            scratch.store_ad(190, &AdValue::powf(AdValue::sub_from_scalar(1.0, AdValue::scale(scratch.ad_value(182), scratch.values[91])), scratch.values[94]));
        }

        if ((!(scratch.values[637] != 0.0)) && (!(scratch.values[670] != 0.0))) {
            scratch.store_ad(630, &AdValue::add(AdValue::scale(AdValue::sub_from_scalar(1.0, scratch.ad_value(190)), scratch.values[103]), AdValue::scale(AdValue::sub(scratch.ad_value(636), scratch.ad_value(182)), scratch.values[106])));
        }

        scratch.values[672] = if ((scratch.values[23] == 0.0) && (scratch.values[26] == 0.0)) { 1.0 } else { 0.0 };

        if (((!(scratch.values[637] != 0.0)) && (!(scratch.values[670] != 0.0))) && (!(scratch.values[672] != 0.0))) {
            scratch.store_ad(193, &AdValue::sub_from_scalar(scratch.values[79], scratch.ad_value(187)));
        }

        scratch.values[674] = if (scratch.values[12] == 0.5) { 1.0 } else { 0.0 };

        if ((((!(scratch.values[637] != 0.0)) && (!(scratch.values[670] != 0.0))) && (!(scratch.values[672] != 0.0))) && (scratch.values[674] != 0.0)) {
            scratch.store_ad(190, &AdValue::sqrt(AdValue::scale(scratch.ad_value(193), scratch.values[115])));
        }

        if ((((!(scratch.values[637] != 0.0)) && (!(scratch.values[670] != 0.0))) && (!(scratch.values[672] != 0.0))) && (!(scratch.values[674] != 0.0))) {
            scratch.store_ad(190, &AdValue::powf(AdValue::scale(scratch.ad_value(193), scratch.values[115]), scratch.values[12]));
        }

        if (((!(scratch.values[637] != 0.0)) && (!(scratch.values[670] != 0.0))) && (!(scratch.values[672] != 0.0))) {
            scratch.store_ad(197, &AdValue::scale(scratch.ad_value(190), scratch.values[109]));
        }

        scratch.values[675] = if (scratch.values[26] == 0.0) { 1.0 } else { 0.0 };

        if (((!(scratch.values[637] != 0.0)) && (!(scratch.values[670] != 0.0))) && (!(scratch.values[675] != 0.0))) {
            scratch.store_ad(200, &AdValue::scale(AdValue::div(AdValue::scale(scratch.ad_value(197), scratch.values[94]), scratch.ad_value(193)), scratch.values[124]));
        }

        if (((!(scratch.values[637] != 0.0)) && (!(scratch.values[670] != 0.0))) && (!(scratch.values[675] != 0.0))) {
            scratch.store_ad(201, &AdValue::div_from_scalar((0.666666666666667 * scratch.values[121]), scratch.ad_value(200)));
        }

        if (((!(scratch.values[637] != 0.0)) && (!(scratch.values[670] != 0.0))) && (!(scratch.values[675] != 0.0))) {
            scratch.store_ad(202, &AdValue::square(scratch.ad_value(201)));
        }

        if (((!(scratch.values[637] != 0.0)) && (!(scratch.values[670] != 0.0))) && (!(scratch.values[675] != 0.0))) {
            scratch.store_ad(203, &AdValue::sqrt(AdValue::div(AdValue::square(scratch.ad_value(202)), AdValue::offset(AdValue::square(scratch.ad_value(202)), 1.0))));
        }

        if (((!(scratch.values[637] != 0.0)) && (!(scratch.values[670] != 0.0))) && (!(scratch.values[675] != 0.0))) {
            scratch.store_ad(204, &AdValue::sqrt(AdValue::abs(scratch.ad_value(203))));
        }

        if (((!(scratch.values[637] != 0.0)) && (!(scratch.values[670] != 0.0))) && (!(scratch.values[675] != 0.0))) {
            scratch.store_ad(205, &AdValue::mul(scratch.ad_value(203), scratch.ad_value(204)));
        }

        if (((!(scratch.values[637] != 0.0)) && (!(scratch.values[670] != 0.0))) && (!(scratch.values[675] != 0.0))) {
            scratch.store_ad(208, &AdValue::sqrt(AdValue::scale(AdValue::div(scratch.ad_value(200), scratch.ad_value(204)), 0.375)));
        }

        if (((!(scratch.values[637] != 0.0)) && (!(scratch.values[670] != 0.0))) && (!(scratch.values[675] != 0.0))) {
            scratch.store_ad(209, &AdValue::sub(AdValue::scale(AdValue::mul(scratch.ad_value(201), scratch.ad_value(204)), 2.0), scratch.ad_value(203)));
        }

        if (((!(scratch.values[637] != 0.0)) && (!(scratch.values[670] != 0.0))) && (!(scratch.values[675] != 0.0))) {
            scratch.store_ad(210, &AdValue::add(AdValue::sub(AdValue::mul(AdValue::scale(scratch.ad_value(201), scratch.values[121]), scratch.ad_value(204)), AdValue::scale(scratch.ad_value(203), scratch.values[121])), AdValue::scale(AdValue::mul(scratch.ad_value(200), scratch.ad_value(205)), 0.5)));
        }

        if (((!(scratch.values[637] != 0.0)) && (!(scratch.values[670] != 0.0))) && (!(scratch.values[675] != 0.0))) {
            scratch.store_ad(211, &AdValue::mul(AdValue::offset(scratch.ad_value(209), (-1.0)), scratch.ad_value(208)));
        }

        if (((!(scratch.values[637] != 0.0)) && (!(scratch.values[670] != 0.0))) && (!(scratch.values[675] != 0.0))) {
            scratch.store_ad(172, &AdValue::square(scratch.ad_value(211)));
        }

        scratch.values[678] = if (((-scratch.values[172]) + scratch.values[210]) > (-230.25850929940458)) { 1.0 } else { 0.0 };

        if ((((!(scratch.values[637] != 0.0)) && (!(scratch.values[670] != 0.0))) && (!(scratch.values[675] != 0.0))) && (scratch.values[678] != 0.0)) {
            scratch.store_ad(190, &AdValue::exp(AdValue::sub(scratch.ad_value(210), scratch.ad_value(172))));
        }

        if ((((!(scratch.values[637] != 0.0)) && (!(scratch.values[670] != 0.0))) && (!(scratch.values[675] != 0.0))) && (!(scratch.values[678] != 0.0))) {
            scratch.store_ad(190, &AdValue::div_from_scalar(1e-100, AdValue::offset(AdValue::mul(AdValue::sub_from_scalar((-230.25850929940458), AdValue::sub(scratch.ad_value(210), scratch.ad_value(172))), AdValue::offset(AdValue::scale(AdValue::mul(AdValue::sub_from_scalar((-230.25850929940458), AdValue::sub(scratch.ad_value(210), scratch.ad_value(172))), AdValue::offset(AdValue::scale(AdValue::sub_from_scalar((-230.25850929940458), AdValue::sub(scratch.ad_value(210), scratch.ad_value(172))), 0.3333333333333333), 1.0)), 0.5), 1.0)), 1.0)));
        }

        scratch.values[679] = if (scratch.values[211] > 0.0) { 1.0 } else { 0.0 };

        scratch.values[680] = if (scratch.values[210] > (-230.25850929940458)) { 1.0 } else { 0.0 };

        if (((((!(scratch.values[637] != 0.0)) && (!(scratch.values[670] != 0.0))) && (!(scratch.values[675] != 0.0))) && (!(scratch.values[679] != 0.0))) && (scratch.values[680] != 0.0)) {
            scratch.store_ad(190, &AdValue::exp(scratch.ad_value(210)));
        }

        if (((((!(scratch.values[637] != 0.0)) && (!(scratch.values[670] != 0.0))) && (!(scratch.values[675] != 0.0))) && (!(scratch.values[679] != 0.0))) && (!(scratch.values[680] != 0.0))) {
            scratch.store_ad(190, &AdValue::div_from_scalar(1e-100, AdValue::offset(AdValue::mul(AdValue::sub_from_scalar((-230.25850929940458), scratch.ad_value(210)), AdValue::offset(AdValue::scale(AdValue::mul(AdValue::sub_from_scalar((-230.25850929940458), scratch.ad_value(210)), AdValue::offset(AdValue::scale(AdValue::sub_from_scalar((-230.25850929940458), scratch.ad_value(210)), 0.3333333333333333), 1.0)), 0.5), 1.0)), 1.0)));
        }

        scratch.values[681] = if (scratch.values[32] == 0.0) { 1.0 } else { 0.0 };

        scratch.values[682] = if (scratch.values[12] == 0.5) { 1.0 } else { 0.0 };

        if ((((!(scratch.values[637] != 0.0)) && (!(scratch.values[670] != 0.0))) && (!(scratch.values[681] != 0.0))) && (scratch.values[682] != 0.0)) {
            scratch.store_ad(190, &AdValue::sqrt(AdValue::scale(AdValue::sub_from_scalar(scratch.values[9], scratch.ad_value(188)), scratch.values[115])));
        }

        if ((((!(scratch.values[637] != 0.0)) && (!(scratch.values[670] != 0.0))) && (!(scratch.values[681] != 0.0))) && (!(scratch.values[682] != 0.0))) {
            scratch.store_ad(190, &AdValue::powf(AdValue::scale(AdValue::sub_from_scalar(scratch.values[9], scratch.ad_value(188)), scratch.values[115]), scratch.values[12]));
        }

        if (((!(scratch.values[637] != 0.0)) && (!(scratch.values[670] != 0.0))) && (!(scratch.values[681] != 0.0))) {
            scratch.store_ad(215, &AdValue::scale(AdValue::div(AdValue::scale(AdValue::sub_from_scalar(scratch.values[9], scratch.ad_value(188)), scratch.values[112]), scratch.ad_value(190)), scratch.values[97]));
        }

        scratch.values[683] = if (((((-scratch.values[127]) / scratch.values[215])) as f64).abs() < 230.25850929940458) { 1.0 } else { 0.0 };

        if ((((!(scratch.values[637] != 0.0)) && (!(scratch.values[670] != 0.0))) && (!(scratch.values[681] != 0.0))) && (scratch.values[683] != 0.0)) {
            scratch.store_ad(190, &AdValue::exp(AdValue::div(AdValue::neg(scratch.ad_value(127)), scratch.ad_value(215))));
        }

        scratch.values[684] = if (((-scratch.values[127]) / scratch.values[215]) < 0.0) { 1.0 } else { 0.0 };

        if (((((!(scratch.values[637] != 0.0)) && (!(scratch.values[670] != 0.0))) && (!(scratch.values[681] != 0.0))) && (!(scratch.values[683] != 0.0))) && (scratch.values[684] != 0.0)) {
            let assign17890_ad_e23493: AdValue = AdValue::offset(AdValue::mul(AdValue::sub_from_scalar((-230.25850929940458), AdValue::div(AdValue::neg(scratch.ad_value(127)), scratch.ad_value(215))), AdValue::offset(AdValue::scale(AdValue::mul(AdValue::sub_from_scalar((-230.25850929940458), AdValue::div(AdValue::neg(scratch.ad_value(127)), scratch.ad_value(215))), AdValue::offset(AdValue::scale(AdValue::sub_from_scalar((-230.25850929940458), AdValue::div(AdValue::neg(scratch.ad_value(127)), scratch.ad_value(215))), 0.3333333333333333), 1.0)), 0.5), 1.0)), 1.0);
            scratch.store_ad(190, &AdValue::div_from_scalar(1e-100, assign17890_ad_e23493));
        }

        if (((((!(scratch.values[637] != 0.0)) && (!(scratch.values[670] != 0.0))) && (!(scratch.values[681] != 0.0))) && (!(scratch.values[683] != 0.0))) && (!(scratch.values[684] != 0.0))) {
            let assign17900_ad_e23542: AdValue = AdValue::scale(AdValue::offset(AdValue::mul(AdValue::offset(AdValue::div(AdValue::neg(scratch.ad_value(127)), scratch.ad_value(215)), (-230.25850929940458)), AdValue::offset(AdValue::scale(AdValue::mul(AdValue::offset(AdValue::div(AdValue::neg(scratch.ad_value(127)), scratch.ad_value(215)), (-230.25850929940458)), AdValue::offset(AdValue::scale(AdValue::offset(AdValue::div(AdValue::neg(scratch.ad_value(127)), scratch.ad_value(215)), (-230.25850929940458)), 0.3333333333333333), 1.0)), 0.5), 1.0)), 1.0), 1e100);
            scratch.store_ad(190, &assign17900_ad_e23542);
        }

        scratch.values[685] = if (scratch.values[41] > 1000.0) { 1.0 } else { 0.0 };

        scratch.values[686] = if (scratch.values[189] > ((-scratch.values[129]) * scratch.values[41])) { 1.0 } else { 0.0 };

        scratch.values[687] = if (scratch.values[44] == 4.0) { 1.0 } else { 0.0 };

        if (((((!(scratch.values[637] != 0.0)) && (!(scratch.values[670] != 0.0))) && (!(scratch.values[685] != 0.0))) && (scratch.values[686] != 0.0)) && (scratch.values[687] != 0.0)) {
            scratch.store_ad(190, &AdValue::mul(AdValue::mul(AdValue::mul(AdValue::scale(scratch.ad_value(189), scratch.values[134]), AdValue::scale(scratch.ad_value(189), scratch.values[134])), AdValue::scale(scratch.ad_value(189), scratch.values[134])), AdValue::scale(scratch.ad_value(189), scratch.values[134])));
        }

        if (((((!(scratch.values[637] != 0.0)) && (!(scratch.values[670] != 0.0))) && (!(scratch.values[685] != 0.0))) && (scratch.values[686] != 0.0)) && (!(scratch.values[687] != 0.0))) {
            scratch.store_ad(190, &AdValue::powf(AdValue::abs(AdValue::scale(scratch.ad_value(189), scratch.values[134])), scratch.values[44]));
        }

        scratch.values[688] = if (scratch.values[219] == 0.0) { 1.0 } else { 0.0 };

        if ((!(scratch.values[637] != 0.0)) && (scratch.values[688] != 0.0)) {
            scratch.values[632] = 0.0;
            scratch.node_derivatives[632] = [0.0; Instance::NODE_COUNT];
            scratch.branch_derivatives[632] = [0.0; Instance::BRANCH_COUNT];
        }

        scratch.values[689] = if (scratch.values[95] == 0.5) { 1.0 } else { 0.0 };

        if (((!(scratch.values[637] != 0.0)) && (!(scratch.values[688] != 0.0))) && (scratch.values[689] != 0.0)) {
            scratch.store_ad(190, &AdValue::sqrt(AdValue::sub_from_scalar(1.0, AdValue::scale(scratch.ad_value(182), scratch.values[92]))));
        }

        if (((!(scratch.values[637] != 0.0)) && (!(scratch.values[688] != 0.0))) && (!(scratch.values[689] != 0.0))) {
            scratch.store_ad(190, &AdValue::powf(AdValue::sub_from_scalar(1.0, AdValue::scale(scratch.ad_value(182), scratch.values[92])), scratch.values[95]));
        }

        if ((!(scratch.values[637] != 0.0)) && (!(scratch.values[688] != 0.0))) {
            scratch.store_ad(632, &AdValue::add(AdValue::scale(AdValue::sub_from_scalar(1.0, scratch.ad_value(190)), scratch.values[104]), AdValue::scale(AdValue::sub(scratch.ad_value(636), scratch.ad_value(182)), scratch.values[107])));
        }

        scratch.values[690] = if ((scratch.values[24] == 0.0) && (scratch.values[27] == 0.0)) { 1.0 } else { 0.0 };

        if (((!(scratch.values[637] != 0.0)) && (!(scratch.values[688] != 0.0))) && (!(scratch.values[690] != 0.0))) {
            scratch.store_ad(193, &AdValue::sub_from_scalar(scratch.values[80], scratch.ad_value(187)));
        }

        scratch.values[692] = if (scratch.values[13] == 0.5) { 1.0 } else { 0.0 };

        if ((((!(scratch.values[637] != 0.0)) && (!(scratch.values[688] != 0.0))) && (!(scratch.values[690] != 0.0))) && (scratch.values[692] != 0.0)) {
            scratch.store_ad(190, &AdValue::sqrt(AdValue::scale(scratch.ad_value(193), scratch.values[116])));
        }

        if ((((!(scratch.values[637] != 0.0)) && (!(scratch.values[688] != 0.0))) && (!(scratch.values[690] != 0.0))) && (!(scratch.values[692] != 0.0))) {
            scratch.store_ad(190, &AdValue::powf(AdValue::scale(scratch.ad_value(193), scratch.values[116]), scratch.values[13]));
        }

        if (((!(scratch.values[637] != 0.0)) && (!(scratch.values[688] != 0.0))) && (!(scratch.values[690] != 0.0))) {
            scratch.store_ad(197, &AdValue::scale(scratch.ad_value(190), scratch.values[110]));
        }

    }
}
