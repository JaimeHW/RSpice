#![allow(dead_code, unused_assignments, unused_imports, unused_parens, unused_variables)]

use super::{AdValue, GeneratedEvalContext, GeneratedReactiveStamper, GeneratedStamper, Scratch, THERMAL_VOLTAGE_PER_K};
use super::super::state::Instance;

impl Instance {

    pub(super) fn stamp_transient_block_4(
        &mut self,
        ctx: &GeneratedEvalContext<'_>,
        stamper: &mut GeneratedStamper<'_>,
        scratch: &mut Scratch,
    ) {
        let _ = stamper;
        if (((scratch.values[270] != 0.0) && (!(scratch.values[386] != 0.0))) && (!(scratch.values[391] != 0.0))) {
            scratch.store_ad(298, &AdValue::scale(AdValue::mul(AdValue::mul(scratch.ad_value(297), scratch.ad_value(312)), scratch.ad_value(306)), scratch.values[25]));
        }

        scratch.values[397] = if (scratch.values[31] == 0.0) { 1.0 } else { 0.0 };

        if (((scratch.values[270] != 0.0) && (!(scratch.values[386] != 0.0))) && (scratch.values[397] != 0.0)) {
            scratch.values[313] = 0.0;
            scratch.node_derivatives[313] = [0.0; Instance::NODE_COUNT];
            scratch.branch_derivatives[313] = [0.0; Instance::BRANCH_COUNT];
        }

        scratch.values[398] = if (scratch.values[11] == 0.5) { 1.0 } else { 0.0 };

        if ((((scratch.values[270] != 0.0) && (!(scratch.values[386] != 0.0))) && (!(scratch.values[397] != 0.0))) && (scratch.values[398] != 0.0)) {
            scratch.store_ad(289, &AdValue::sqrt(AdValue::scale(AdValue::sub_from_scalar(scratch.values[8], scratch.ad_value(287)), scratch.values[114])));
        }

        if ((((scratch.values[270] != 0.0) && (!(scratch.values[386] != 0.0))) && (!(scratch.values[397] != 0.0))) && (!(scratch.values[398] != 0.0))) {
            scratch.store_ad(289, &AdValue::powf(AdValue::scale(AdValue::sub_from_scalar(scratch.values[8], scratch.ad_value(287)), scratch.values[114]), scratch.values[11]));
        }

        if (((scratch.values[270] != 0.0) && (!(scratch.values[386] != 0.0))) && (!(scratch.values[397] != 0.0))) {
            scratch.store_ad(314, &AdValue::scale(AdValue::div(AdValue::scale(AdValue::sub_from_scalar(scratch.values[8], scratch.ad_value(287)), scratch.values[111]), scratch.ad_value(289)), scratch.values[96]));
        }

        scratch.values[399] = if (((((-scratch.values[126]) / scratch.values[314])) as f64).abs() < 230.25850929940458) { 1.0 } else { 0.0 };

        if ((((scratch.values[270] != 0.0) && (!(scratch.values[386] != 0.0))) && (!(scratch.values[397] != 0.0))) && (scratch.values[399] != 0.0)) {
            scratch.store_ad(289, &AdValue::exp(AdValue::div(AdValue::neg(scratch.ad_value(126)), scratch.ad_value(314))));
        }

        scratch.values[400] = if (((-scratch.values[126]) / scratch.values[314]) < 0.0) { 1.0 } else { 0.0 };

        if (((((scratch.values[270] != 0.0) && (!(scratch.values[386] != 0.0))) && (!(scratch.values[397] != 0.0))) && (!(scratch.values[399] != 0.0))) && (scratch.values[400] != 0.0)) {
            let assign6100_ad_e6444: AdValue = AdValue::offset(AdValue::mul(AdValue::sub_from_scalar((-230.25850929940458), AdValue::div(AdValue::neg(scratch.ad_value(126)), scratch.ad_value(314))), AdValue::offset(AdValue::scale(AdValue::mul(AdValue::sub_from_scalar((-230.25850929940458), AdValue::div(AdValue::neg(scratch.ad_value(126)), scratch.ad_value(314))), AdValue::offset(AdValue::scale(AdValue::sub_from_scalar((-230.25850929940458), AdValue::div(AdValue::neg(scratch.ad_value(126)), scratch.ad_value(314))), 0.3333333333333333), 1.0)), 0.5), 1.0)), 1.0);
            scratch.store_ad(289, &AdValue::div_from_scalar(1e-100, assign6100_ad_e6444));
        }

        if (((((scratch.values[270] != 0.0) && (!(scratch.values[386] != 0.0))) && (!(scratch.values[397] != 0.0))) && (!(scratch.values[399] != 0.0))) && (!(scratch.values[400] != 0.0))) {
            let assign6110_ad_e6492: AdValue = AdValue::scale(AdValue::offset(AdValue::mul(AdValue::offset(AdValue::div(AdValue::neg(scratch.ad_value(126)), scratch.ad_value(314)), (-230.25850929940458)), AdValue::offset(AdValue::scale(AdValue::mul(AdValue::offset(AdValue::div(AdValue::neg(scratch.ad_value(126)), scratch.ad_value(314)), (-230.25850929940458)), AdValue::offset(AdValue::scale(AdValue::offset(AdValue::div(AdValue::neg(scratch.ad_value(126)), scratch.ad_value(314)), (-230.25850929940458)), 0.3333333333333333), 1.0)), 0.5), 1.0)), 1.0), 1e100);
            scratch.store_ad(289, &assign6110_ad_e6492);
        }

        if (((scratch.values[270] != 0.0) && (!(scratch.values[386] != 0.0))) && (!(scratch.values[397] != 0.0))) {
            scratch.store_ad(313, &AdValue::scale(AdValue::mul(AdValue::mul(AdValue::mul(scratch.ad_value(153), scratch.ad_value(314)), scratch.ad_value(314)), scratch.ad_value(289)), scratch.values[31]));
        }

        scratch.values[401] = if (scratch.values[40] > 1000.0) { 1.0 } else { 0.0 };

        if (((scratch.values[270] != 0.0) && (!(scratch.values[386] != 0.0))) && (scratch.values[401] != 0.0)) {
            scratch.values[315] = 1.0;
            scratch.node_derivatives[315] = [0.0; Instance::NODE_COUNT];
            scratch.branch_derivatives[315] = [0.0; Instance::BRANCH_COUNT];
        }

        scratch.values[402] = if (scratch.values[288] > ((-scratch.values[129]) * scratch.values[40])) { 1.0 } else { 0.0 };

        scratch.values[403] = if (scratch.values[43] == 4.0) { 1.0 } else { 0.0 };

        if (((((scratch.values[270] != 0.0) && (!(scratch.values[386] != 0.0))) && (!(scratch.values[401] != 0.0))) && (scratch.values[402] != 0.0)) && (scratch.values[403] != 0.0)) {
            scratch.store_ad(289, &AdValue::mul(AdValue::mul(AdValue::mul(AdValue::scale(scratch.ad_value(288), scratch.values[133]), AdValue::scale(scratch.ad_value(288), scratch.values[133])), AdValue::scale(scratch.ad_value(288), scratch.values[133])), AdValue::scale(scratch.ad_value(288), scratch.values[133])));
        }

        if (((((scratch.values[270] != 0.0) && (!(scratch.values[386] != 0.0))) && (!(scratch.values[401] != 0.0))) && (scratch.values[402] != 0.0)) && (!(scratch.values[403] != 0.0))) {
            scratch.store_ad(289, &AdValue::powf(AdValue::abs(AdValue::scale(scratch.ad_value(288), scratch.values[133])), scratch.values[43]));
        }

        if ((((scratch.values[270] != 0.0) && (!(scratch.values[386] != 0.0))) && (!(scratch.values[401] != 0.0))) && (scratch.values[402] != 0.0)) {
            scratch.store_ad(315, &AdValue::div_from_scalar(1.0, AdValue::sub_from_scalar(1.0, scratch.ad_value(289))));
        }

        if ((((scratch.values[270] != 0.0) && (!(scratch.values[386] != 0.0))) && (!(scratch.values[401] != 0.0))) && (!(scratch.values[402] != 0.0))) {
            scratch.store_ad(315, &AdValue::offset(AdValue::scale(AdValue::offset(scratch.ad_value(288), (scratch.values[129] * scratch.values[40])), scratch.values[136]), scratch.values[130]));
        }

        if ((scratch.values[270] != 0.0) && (!(scratch.values[386] != 0.0))) {
            scratch.store_ad(316, &AdValue::mul(AdValue::add(AdValue::add(AdValue::add(scratch.ad_value(290), scratch.ad_value(291)), scratch.ad_value(298)), scratch.ad_value(313)), scratch.ad_value(315)));
        }

        scratch.values[404] = if (scratch.values[218] == 0.0) { 1.0 } else { 0.0 };

        if ((scratch.values[270] != 0.0) && (scratch.values[404] != 0.0)) {
            scratch.values[317] = 0.0;
            scratch.node_derivatives[317] = [0.0; Instance::NODE_COUNT];
            scratch.branch_derivatives[317] = [0.0; Instance::BRANCH_COUNT];
        }

        scratch.values[405] = if (scratch.values[94] == 0.5) { 1.0 } else { 0.0 };

        if (((scratch.values[270] != 0.0) && (!(scratch.values[404] != 0.0))) && (scratch.values[405] != 0.0)) {
            scratch.store_ad(289, &AdValue::sqrt(AdValue::sub_from_scalar(1.0, AdValue::scale(scratch.ad_value(281), scratch.values[91]))));
        }

        if (((scratch.values[270] != 0.0) && (!(scratch.values[404] != 0.0))) && (!(scratch.values[405] != 0.0))) {
            scratch.store_ad(289, &AdValue::powf(AdValue::sub_from_scalar(1.0, AdValue::scale(scratch.ad_value(281), scratch.values[91])), scratch.values[94]));
        }

        if ((scratch.values[270] != 0.0) && (!(scratch.values[404] != 0.0))) {
            scratch.store_ad(290, &AdValue::scale(scratch.ad_value(280), scratch.values[73]));
        }

        scratch.values[406] = if ((scratch.values[23] == 0.0) && (scratch.values[26] == 0.0)) { 1.0 } else { 0.0 };

        if (((scratch.values[270] != 0.0) && (!(scratch.values[404] != 0.0))) && (scratch.values[406] != 0.0)) {
            scratch.values[291] = 0.0;
            scratch.node_derivatives[291] = [0.0; Instance::NODE_COUNT];
            scratch.branch_derivatives[291] = [0.0; Instance::BRANCH_COUNT];
        }

        if (((scratch.values[270] != 0.0) && (!(scratch.values[404] != 0.0))) && (!(scratch.values[406] != 0.0))) {
            scratch.store_ad(292, &AdValue::sub_from_scalar(scratch.values[79], scratch.ad_value(286)));
        }

        if (((scratch.values[270] != 0.0) && (!(scratch.values[404] != 0.0))) && (!(scratch.values[406] != 0.0))) {
            scratch.store_ad(293, &AdValue::sub_from_scalar(1.0, AdValue::sqrt(AdValue::sub_from_scalar(1.0, AdValue::div(scratch.ad_value(284), scratch.ad_value(292))))));
        }

        scratch.values[407] = if (scratch.values[12] == 0.5) { 1.0 } else { 0.0 };

        if ((((scratch.values[270] != 0.0) && (!(scratch.values[404] != 0.0))) && (!(scratch.values[406] != 0.0))) && (scratch.values[407] != 0.0)) {
            scratch.values[294] = 0.0;
            scratch.node_derivatives[294] = [0.0; Instance::NODE_COUNT];
            scratch.branch_derivatives[294] = [0.0; Instance::BRANCH_COUNT];
        }

        if ((((scratch.values[270] != 0.0) && (!(scratch.values[404] != 0.0))) && (!(scratch.values[406] != 0.0))) && (!(scratch.values[407] != 0.0))) {
            scratch.store_ad(294, &AdValue::scale(AdValue::add(AdValue::div(AdValue::mul(AdValue::square(scratch.ad_value(293)), AdValue::ln(scratch.ad_value(293))), AdValue::sub_from_scalar(1.0, scratch.ad_value(293))), scratch.ad_value(293)), (1.0 - (2.0 * scratch.values[12]))));
        }

        if (((scratch.values[270] != 0.0) && (!(scratch.values[404] != 0.0))) && (!(scratch.values[406] != 0.0))) {
            scratch.store_ad(295, &AdValue::add(scratch.ad_value(293), scratch.ad_value(294)));
        }

        scratch.values[408] = if (scratch.values[12] == 0.5) { 1.0 } else { 0.0 };

        if ((((scratch.values[270] != 0.0) && (!(scratch.values[404] != 0.0))) && (!(scratch.values[406] != 0.0))) && (scratch.values[408] != 0.0)) {
            scratch.store_ad(289, &AdValue::sqrt(AdValue::scale(scratch.ad_value(292), scratch.values[115])));
        }

        if ((((scratch.values[270] != 0.0) && (!(scratch.values[404] != 0.0))) && (!(scratch.values[406] != 0.0))) && (!(scratch.values[408] != 0.0))) {
            scratch.store_ad(289, &AdValue::powf(AdValue::scale(scratch.ad_value(292), scratch.values[115]), scratch.values[12]));
        }

        if (((scratch.values[270] != 0.0) && (!(scratch.values[404] != 0.0))) && (!(scratch.values[406] != 0.0))) {
            scratch.store_ad(296, &AdValue::scale(scratch.ad_value(289), scratch.values[109]));
        }

        if (((scratch.values[270] != 0.0) && (!(scratch.values[404] != 0.0))) && (!(scratch.values[406] != 0.0))) {
            scratch.store_ad(297, &AdValue::scale(AdValue::mul(AdValue::offset(scratch.ad_value(283), (-1.0)), scratch.ad_value(296)), scratch.values[70]));
        }

        if (((scratch.values[270] != 0.0) && (!(scratch.values[404] != 0.0))) && (!(scratch.values[406] != 0.0))) {
            scratch.store_ad(291, &AdValue::scale(AdValue::mul(scratch.ad_value(297), scratch.ad_value(295)), scratch.values[23]));
        }

        scratch.values[409] = if (scratch.values[26] == 0.0) { 1.0 } else { 0.0 };

        if (((scratch.values[270] != 0.0) && (!(scratch.values[404] != 0.0))) && (scratch.values[409] != 0.0)) {
            scratch.values[298] = 0.0;
            scratch.node_derivatives[298] = [0.0; Instance::NODE_COUNT];
            scratch.branch_derivatives[298] = [0.0; Instance::BRANCH_COUNT];
        }

        if (((scratch.values[270] != 0.0) && (!(scratch.values[404] != 0.0))) && (!(scratch.values[409] != 0.0))) {
            scratch.store_ad(299, &AdValue::scale(AdValue::div(AdValue::scale(scratch.ad_value(296), scratch.values[94]), scratch.ad_value(292)), scratch.values[124]));
        }

        if (((scratch.values[270] != 0.0) && (!(scratch.values[404] != 0.0))) && (!(scratch.values[409] != 0.0))) {
            scratch.store_ad(300, &AdValue::div_from_scalar((0.666666666666667 * scratch.values[121]), scratch.ad_value(299)));
        }

        if (((scratch.values[270] != 0.0) && (!(scratch.values[404] != 0.0))) && (!(scratch.values[409] != 0.0))) {
            scratch.store_ad(301, &AdValue::square(scratch.ad_value(300)));
        }

        if (((scratch.values[270] != 0.0) && (!(scratch.values[404] != 0.0))) && (!(scratch.values[409] != 0.0))) {
            scratch.store_ad(302, &AdValue::sqrt(AdValue::div(AdValue::square(scratch.ad_value(301)), AdValue::offset(AdValue::square(scratch.ad_value(301)), 1.0))));
        }

        if (((scratch.values[270] != 0.0) && (!(scratch.values[404] != 0.0))) && (!(scratch.values[409] != 0.0))) {
            scratch.store_ad(303, &AdValue::sqrt(AdValue::abs(scratch.ad_value(302))));
        }

        if (((scratch.values[270] != 0.0) && (!(scratch.values[404] != 0.0))) && (!(scratch.values[409] != 0.0))) {
            scratch.store_ad(304, &AdValue::mul(scratch.ad_value(302), scratch.ad_value(303)));
        }

        scratch.values[410] = if (((-scratch.values[12]) * scratch.values[97]) == (-1.0)) { 1.0 } else { 0.0 };

        if ((((scratch.values[270] != 0.0) && (!(scratch.values[404] != 0.0))) && (!(scratch.values[409] != 0.0))) && (scratch.values[410] != 0.0)) {
            scratch.store_ad(305, &AdValue::div_from_scalar(1.0, AdValue::offset(AdValue::mul(scratch.ad_value(299), scratch.ad_value(304)), 1.0)));
        }

        if ((((scratch.values[270] != 0.0) && (!(scratch.values[404] != 0.0))) && (!(scratch.values[409] != 0.0))) && (!(scratch.values[410] != 0.0))) {
            scratch.store_ad(305, &AdValue::powf(AdValue::offset(AdValue::mul(scratch.ad_value(299), scratch.ad_value(304)), 1.0), ((-scratch.values[12]) * scratch.values[97])));
        }

        if (((scratch.values[270] != 0.0) && (!(scratch.values[404] != 0.0))) && (!(scratch.values[409] != 0.0))) {
            scratch.store_ad(306, &AdValue::div(AdValue::mul(scratch.ad_value(295), scratch.ad_value(305)), AdValue::add(scratch.ad_value(295), scratch.ad_value(305))));
        }

        if (((scratch.values[270] != 0.0) && (!(scratch.values[404] != 0.0))) && (!(scratch.values[409] != 0.0))) {
            scratch.store_ad(307, &AdValue::sqrt(AdValue::scale(AdValue::div(scratch.ad_value(299), scratch.ad_value(303)), 0.375)));
        }

        if (((scratch.values[270] != 0.0) && (!(scratch.values[404] != 0.0))) && (!(scratch.values[409] != 0.0))) {
            scratch.store_ad(308, &AdValue::sub(AdValue::scale(AdValue::mul(scratch.ad_value(300), scratch.ad_value(303)), 2.0), scratch.ad_value(302)));
        }

        if (((scratch.values[270] != 0.0) && (!(scratch.values[404] != 0.0))) && (!(scratch.values[409] != 0.0))) {
            scratch.store_ad(309, &AdValue::add(AdValue::sub(AdValue::mul(AdValue::scale(scratch.ad_value(300), scratch.values[121]), scratch.ad_value(303)), AdValue::scale(scratch.ad_value(302), scratch.values[121])), AdValue::scale(AdValue::mul(scratch.ad_value(299), scratch.ad_value(304)), 0.5)));
        }

        if (((scratch.values[270] != 0.0) && (!(scratch.values[404] != 0.0))) && (!(scratch.values[409] != 0.0))) {
            scratch.store_ad(310, &AdValue::mul(AdValue::offset(scratch.ad_value(308), (-1.0)), scratch.ad_value(307)));
        }

        if (((scratch.values[270] != 0.0) && (!(scratch.values[404] != 0.0))) && (!(scratch.values[409] != 0.0))) {
            scratch.store_ad(271, &AdValue::square(scratch.ad_value(310)));
        }

        scratch.values[411] = if (scratch.values[310] > 0.0) { 1.0 } else { 0.0 };

        if ((((scratch.values[270] != 0.0) && (!(scratch.values[404] != 0.0))) && (!(scratch.values[409] != 0.0))) && (scratch.values[411] != 0.0)) {
            scratch.store_ad(272, &AdValue::div_from_scalar(1.0, AdValue::offset(AdValue::scale(scratch.ad_value(310), scratch.values[57]), 1.0)));
        }

        if ((((scratch.values[270] != 0.0) && (!(scratch.values[404] != 0.0))) && (!(scratch.values[409] != 0.0))) && (!(scratch.values[411] != 0.0))) {
            scratch.store_ad(272, &AdValue::div_from_scalar(1.0, AdValue::sub_from_scalar(1.0, AdValue::scale(scratch.ad_value(310), scratch.values[57]))));
        }

        scratch.values[412] = if (((-scratch.values[271]) + scratch.values[309]) > (-230.25850929940458)) { 1.0 } else { 0.0 };

        if ((((scratch.values[270] != 0.0) && (!(scratch.values[404] != 0.0))) && (!(scratch.values[409] != 0.0))) && (scratch.values[412] != 0.0)) {
            scratch.store_ad(289, &AdValue::exp(AdValue::sub(scratch.ad_value(309), scratch.ad_value(271))));
        }

        if ((((scratch.values[270] != 0.0) && (!(scratch.values[404] != 0.0))) && (!(scratch.values[409] != 0.0))) && (!(scratch.values[412] != 0.0))) {
            scratch.store_ad(289, &AdValue::div_from_scalar(1e-100, AdValue::offset(AdValue::mul(AdValue::sub_from_scalar((-230.25850929940458), AdValue::sub(scratch.ad_value(309), scratch.ad_value(271))), AdValue::offset(AdValue::scale(AdValue::mul(AdValue::sub_from_scalar((-230.25850929940458), AdValue::sub(scratch.ad_value(309), scratch.ad_value(271))), AdValue::offset(AdValue::scale(AdValue::sub_from_scalar((-230.25850929940458), AdValue::sub(scratch.ad_value(309), scratch.ad_value(271))), 0.3333333333333333), 1.0)), 0.5), 1.0)), 1.0)));
        }

        if (((scratch.values[270] != 0.0) && (!(scratch.values[404] != 0.0))) && (!(scratch.values[409] != 0.0))) {
            scratch.store_ad(273, &AdValue::mul(AdValue::add(AdValue::add(AdValue::scale(scratch.ad_value(272), 0.29214664), AdValue::scale(AdValue::square(scratch.ad_value(272)), scratch.values[58])), AdValue::scale(AdValue::mul(AdValue::square(scratch.ad_value(272)), scratch.ad_value(272)), scratch.values[59])), scratch.ad_value(289)));
        }

        scratch.values[413] = if (scratch.values[310] > 0.0) { 1.0 } else { 0.0 };

        if ((((scratch.values[270] != 0.0) && (!(scratch.values[404] != 0.0))) && (!(scratch.values[409] != 0.0))) && (scratch.values[413] != 0.0)) {
            scratch.values[311] = scratch.values[273];
            scratch.node_derivatives[311] = scratch.node_derivatives[273];
            scratch.branch_derivatives[311] = scratch.branch_derivatives[273];
        }

        scratch.values[414] = if (scratch.values[309] > (-230.25850929940458)) { 1.0 } else { 0.0 };

        if (((((scratch.values[270] != 0.0) && (!(scratch.values[404] != 0.0))) && (!(scratch.values[409] != 0.0))) && (!(scratch.values[413] != 0.0))) && (scratch.values[414] != 0.0)) {
            scratch.store_ad(289, &AdValue::exp(scratch.ad_value(309)));
        }

        if (((((scratch.values[270] != 0.0) && (!(scratch.values[404] != 0.0))) && (!(scratch.values[409] != 0.0))) && (!(scratch.values[413] != 0.0))) && (!(scratch.values[414] != 0.0))) {
            scratch.store_ad(289, &AdValue::div_from_scalar(1e-100, AdValue::offset(AdValue::mul(AdValue::sub_from_scalar((-230.25850929940458), scratch.ad_value(309)), AdValue::offset(AdValue::scale(AdValue::mul(AdValue::sub_from_scalar((-230.25850929940458), scratch.ad_value(309)), AdValue::offset(AdValue::scale(AdValue::sub_from_scalar((-230.25850929940458), scratch.ad_value(309)), 0.3333333333333333), 1.0)), 0.5), 1.0)), 1.0)));
        }

        if ((((scratch.values[270] != 0.0) && (!(scratch.values[404] != 0.0))) && (!(scratch.values[409] != 0.0))) && (!(scratch.values[413] != 0.0))) {
            scratch.store_ad(311, &AdValue::sub(AdValue::scale(scratch.ad_value(289), 2.0), scratch.ad_value(273)));
        }

        if (((scratch.values[270] != 0.0) && (!(scratch.values[404] != 0.0))) && (!(scratch.values[409] != 0.0))) {
            scratch.store_ad(312, &AdValue::scale(AdValue::div(AdValue::scale(scratch.ad_value(311), scratch.values[121]), scratch.ad_value(307)), (1.772453850905516 * 0.5)));
        }

        if (((scratch.values[270] != 0.0) && (!(scratch.values[404] != 0.0))) && (!(scratch.values[409] != 0.0))) {
            scratch.store_ad(298, &AdValue::scale(AdValue::mul(AdValue::mul(scratch.ad_value(297), scratch.ad_value(312)), scratch.ad_value(306)), scratch.values[26]));
        }

        scratch.values[415] = if (scratch.values[32] == 0.0) { 1.0 } else { 0.0 };

        if (((scratch.values[270] != 0.0) && (!(scratch.values[404] != 0.0))) && (scratch.values[415] != 0.0)) {
            scratch.values[313] = 0.0;
            scratch.node_derivatives[313] = [0.0; Instance::NODE_COUNT];
            scratch.branch_derivatives[313] = [0.0; Instance::BRANCH_COUNT];
        }

        scratch.values[416] = if (scratch.values[12] == 0.5) { 1.0 } else { 0.0 };

        if ((((scratch.values[270] != 0.0) && (!(scratch.values[404] != 0.0))) && (!(scratch.values[415] != 0.0))) && (scratch.values[416] != 0.0)) {
            scratch.store_ad(289, &AdValue::sqrt(AdValue::scale(AdValue::sub_from_scalar(scratch.values[9], scratch.ad_value(287)), scratch.values[115])));
        }

        if ((((scratch.values[270] != 0.0) && (!(scratch.values[404] != 0.0))) && (!(scratch.values[415] != 0.0))) && (!(scratch.values[416] != 0.0))) {
            scratch.store_ad(289, &AdValue::powf(AdValue::scale(AdValue::sub_from_scalar(scratch.values[9], scratch.ad_value(287)), scratch.values[115]), scratch.values[12]));
        }

        if (((scratch.values[270] != 0.0) && (!(scratch.values[404] != 0.0))) && (!(scratch.values[415] != 0.0))) {
            scratch.store_ad(314, &AdValue::scale(AdValue::div(AdValue::scale(AdValue::sub_from_scalar(scratch.values[9], scratch.ad_value(287)), scratch.values[112]), scratch.ad_value(289)), scratch.values[97]));
        }

        scratch.values[417] = if (((((-scratch.values[127]) / scratch.values[314])) as f64).abs() < 230.25850929940458) { 1.0 } else { 0.0 };

        if ((((scratch.values[270] != 0.0) && (!(scratch.values[404] != 0.0))) && (!(scratch.values[415] != 0.0))) && (scratch.values[417] != 0.0)) {
            scratch.store_ad(289, &AdValue::exp(AdValue::div(AdValue::neg(scratch.ad_value(127)), scratch.ad_value(314))));
        }

        scratch.values[418] = if (((-scratch.values[127]) / scratch.values[314]) < 0.0) { 1.0 } else { 0.0 };

        if (((((scratch.values[270] != 0.0) && (!(scratch.values[404] != 0.0))) && (!(scratch.values[415] != 0.0))) && (!(scratch.values[417] != 0.0))) && (scratch.values[418] != 0.0)) {
            let assign6850_ad_e7536: AdValue = AdValue::offset(AdValue::mul(AdValue::sub_from_scalar((-230.25850929940458), AdValue::div(AdValue::neg(scratch.ad_value(127)), scratch.ad_value(314))), AdValue::offset(AdValue::scale(AdValue::mul(AdValue::sub_from_scalar((-230.25850929940458), AdValue::div(AdValue::neg(scratch.ad_value(127)), scratch.ad_value(314))), AdValue::offset(AdValue::scale(AdValue::sub_from_scalar((-230.25850929940458), AdValue::div(AdValue::neg(scratch.ad_value(127)), scratch.ad_value(314))), 0.3333333333333333), 1.0)), 0.5), 1.0)), 1.0);
            scratch.store_ad(289, &AdValue::div_from_scalar(1e-100, assign6850_ad_e7536));
        }

        if (((((scratch.values[270] != 0.0) && (!(scratch.values[404] != 0.0))) && (!(scratch.values[415] != 0.0))) && (!(scratch.values[417] != 0.0))) && (!(scratch.values[418] != 0.0))) {
            let assign6860_ad_e7584: AdValue = AdValue::scale(AdValue::offset(AdValue::mul(AdValue::offset(AdValue::div(AdValue::neg(scratch.ad_value(127)), scratch.ad_value(314)), (-230.25850929940458)), AdValue::offset(AdValue::scale(AdValue::mul(AdValue::offset(AdValue::div(AdValue::neg(scratch.ad_value(127)), scratch.ad_value(314)), (-230.25850929940458)), AdValue::offset(AdValue::scale(AdValue::offset(AdValue::div(AdValue::neg(scratch.ad_value(127)), scratch.ad_value(314)), (-230.25850929940458)), 0.3333333333333333), 1.0)), 0.5), 1.0)), 1.0), 1e100);
            scratch.store_ad(289, &assign6860_ad_e7584);
        }

        if (((scratch.values[270] != 0.0) && (!(scratch.values[404] != 0.0))) && (!(scratch.values[415] != 0.0))) {
            scratch.store_ad(313, &AdValue::scale(AdValue::mul(AdValue::mul(AdValue::mul(scratch.ad_value(153), scratch.ad_value(314)), scratch.ad_value(314)), scratch.ad_value(289)), scratch.values[32]));
        }

        scratch.values[419] = if (scratch.values[41] > 1000.0) { 1.0 } else { 0.0 };

        if (((scratch.values[270] != 0.0) && (!(scratch.values[404] != 0.0))) && (scratch.values[419] != 0.0)) {
            scratch.values[315] = 1.0;
            scratch.node_derivatives[315] = [0.0; Instance::NODE_COUNT];
            scratch.branch_derivatives[315] = [0.0; Instance::BRANCH_COUNT];
        }

        scratch.values[420] = if (scratch.values[288] > ((-scratch.values[129]) * scratch.values[41])) { 1.0 } else { 0.0 };

        scratch.values[421] = if (scratch.values[44] == 4.0) { 1.0 } else { 0.0 };

        if (((((scratch.values[270] != 0.0) && (!(scratch.values[404] != 0.0))) && (!(scratch.values[419] != 0.0))) && (scratch.values[420] != 0.0)) && (scratch.values[421] != 0.0)) {
            scratch.store_ad(289, &AdValue::mul(AdValue::mul(AdValue::mul(AdValue::scale(scratch.ad_value(288), scratch.values[134]), AdValue::scale(scratch.ad_value(288), scratch.values[134])), AdValue::scale(scratch.ad_value(288), scratch.values[134])), AdValue::scale(scratch.ad_value(288), scratch.values[134])));
        }

        if (((((scratch.values[270] != 0.0) && (!(scratch.values[404] != 0.0))) && (!(scratch.values[419] != 0.0))) && (scratch.values[420] != 0.0)) && (!(scratch.values[421] != 0.0))) {
            scratch.store_ad(289, &AdValue::powf(AdValue::abs(AdValue::scale(scratch.ad_value(288), scratch.values[134])), scratch.values[44]));
        }

        if ((((scratch.values[270] != 0.0) && (!(scratch.values[404] != 0.0))) && (!(scratch.values[419] != 0.0))) && (scratch.values[420] != 0.0)) {
            scratch.store_ad(315, &AdValue::div_from_scalar(1.0, AdValue::sub_from_scalar(1.0, scratch.ad_value(289))));
        }

        if ((((scratch.values[270] != 0.0) && (!(scratch.values[404] != 0.0))) && (!(scratch.values[419] != 0.0))) && (!(scratch.values[420] != 0.0))) {
            scratch.store_ad(315, &AdValue::offset(AdValue::scale(AdValue::offset(scratch.ad_value(288), (scratch.values[129] * scratch.values[41])), scratch.values[137]), scratch.values[131]));
        }

        if ((scratch.values[270] != 0.0) && (!(scratch.values[404] != 0.0))) {
            scratch.store_ad(317, &AdValue::mul(AdValue::add(AdValue::add(AdValue::add(scratch.ad_value(290), scratch.ad_value(291)), scratch.ad_value(298)), scratch.ad_value(313)), scratch.ad_value(315)));
        }

        scratch.values[422] = if (scratch.values[219] == 0.0) { 1.0 } else { 0.0 };

        if ((scratch.values[270] != 0.0) && (scratch.values[422] != 0.0)) {
            scratch.values[318] = 0.0;
            scratch.node_derivatives[318] = [0.0; Instance::NODE_COUNT];
            scratch.branch_derivatives[318] = [0.0; Instance::BRANCH_COUNT];
        }

        scratch.values[423] = if (scratch.values[95] == 0.5) { 1.0 } else { 0.0 };

        if (((scratch.values[270] != 0.0) && (!(scratch.values[422] != 0.0))) && (scratch.values[423] != 0.0)) {
            scratch.store_ad(289, &AdValue::sqrt(AdValue::sub_from_scalar(1.0, AdValue::scale(scratch.ad_value(281), scratch.values[92]))));
        }

        if (((scratch.values[270] != 0.0) && (!(scratch.values[422] != 0.0))) && (!(scratch.values[423] != 0.0))) {
            scratch.store_ad(289, &AdValue::powf(AdValue::sub_from_scalar(1.0, AdValue::scale(scratch.ad_value(281), scratch.values[92])), scratch.values[95]));
        }

        if ((scratch.values[270] != 0.0) && (!(scratch.values[422] != 0.0))) {
            scratch.store_ad(290, &AdValue::scale(scratch.ad_value(280), scratch.values[74]));
        }

        scratch.values[424] = if ((scratch.values[24] == 0.0) && (scratch.values[27] == 0.0)) { 1.0 } else { 0.0 };

        if (((scratch.values[270] != 0.0) && (!(scratch.values[422] != 0.0))) && (scratch.values[424] != 0.0)) {
            scratch.values[291] = 0.0;
            scratch.node_derivatives[291] = [0.0; Instance::NODE_COUNT];
            scratch.branch_derivatives[291] = [0.0; Instance::BRANCH_COUNT];
        }

        if (((scratch.values[270] != 0.0) && (!(scratch.values[422] != 0.0))) && (!(scratch.values[424] != 0.0))) {
            scratch.store_ad(292, &AdValue::sub_from_scalar(scratch.values[80], scratch.ad_value(286)));
        }

        if (((scratch.values[270] != 0.0) && (!(scratch.values[422] != 0.0))) && (!(scratch.values[424] != 0.0))) {
            scratch.store_ad(293, &AdValue::sub_from_scalar(1.0, AdValue::sqrt(AdValue::sub_from_scalar(1.0, AdValue::div(scratch.ad_value(284), scratch.ad_value(292))))));
        }

        scratch.values[425] = if (scratch.values[13] == 0.5) { 1.0 } else { 0.0 };

        if ((((scratch.values[270] != 0.0) && (!(scratch.values[422] != 0.0))) && (!(scratch.values[424] != 0.0))) && (scratch.values[425] != 0.0)) {
            scratch.values[294] = 0.0;
            scratch.node_derivatives[294] = [0.0; Instance::NODE_COUNT];
            scratch.branch_derivatives[294] = [0.0; Instance::BRANCH_COUNT];
        }

        if ((((scratch.values[270] != 0.0) && (!(scratch.values[422] != 0.0))) && (!(scratch.values[424] != 0.0))) && (!(scratch.values[425] != 0.0))) {
            scratch.store_ad(294, &AdValue::scale(AdValue::add(AdValue::div(AdValue::mul(AdValue::square(scratch.ad_value(293)), AdValue::ln(scratch.ad_value(293))), AdValue::sub_from_scalar(1.0, scratch.ad_value(293))), scratch.ad_value(293)), (1.0 - (2.0 * scratch.values[13]))));
        }

        if (((scratch.values[270] != 0.0) && (!(scratch.values[422] != 0.0))) && (!(scratch.values[424] != 0.0))) {
            scratch.store_ad(295, &AdValue::add(scratch.ad_value(293), scratch.ad_value(294)));
        }

        scratch.values[426] = if (scratch.values[13] == 0.5) { 1.0 } else { 0.0 };

        if ((((scratch.values[270] != 0.0) && (!(scratch.values[422] != 0.0))) && (!(scratch.values[424] != 0.0))) && (scratch.values[426] != 0.0)) {
            scratch.store_ad(289, &AdValue::sqrt(AdValue::scale(scratch.ad_value(292), scratch.values[116])));
        }

        if ((((scratch.values[270] != 0.0) && (!(scratch.values[422] != 0.0))) && (!(scratch.values[424] != 0.0))) && (!(scratch.values[426] != 0.0))) {
            scratch.store_ad(289, &AdValue::powf(AdValue::scale(scratch.ad_value(292), scratch.values[116]), scratch.values[13]));
        }

        if (((scratch.values[270] != 0.0) && (!(scratch.values[422] != 0.0))) && (!(scratch.values[424] != 0.0))) {
            scratch.store_ad(296, &AdValue::scale(scratch.ad_value(289), scratch.values[110]));
        }

        if (((scratch.values[270] != 0.0) && (!(scratch.values[422] != 0.0))) && (!(scratch.values[424] != 0.0))) {
            scratch.store_ad(297, &AdValue::scale(AdValue::mul(AdValue::offset(scratch.ad_value(283), (-1.0)), scratch.ad_value(296)), scratch.values[71]));
        }

        if (((scratch.values[270] != 0.0) && (!(scratch.values[422] != 0.0))) && (!(scratch.values[424] != 0.0))) {
            scratch.store_ad(291, &AdValue::scale(AdValue::mul(scratch.ad_value(297), scratch.ad_value(295)), scratch.values[24]));
        }

        scratch.values[427] = if (scratch.values[27] == 0.0) { 1.0 } else { 0.0 };

        if (((scratch.values[270] != 0.0) && (!(scratch.values[422] != 0.0))) && (scratch.values[427] != 0.0)) {
            scratch.values[298] = 0.0;
            scratch.node_derivatives[298] = [0.0; Instance::NODE_COUNT];
            scratch.branch_derivatives[298] = [0.0; Instance::BRANCH_COUNT];
        }

        if (((scratch.values[270] != 0.0) && (!(scratch.values[422] != 0.0))) && (!(scratch.values[427] != 0.0))) {
            scratch.store_ad(299, &AdValue::scale(AdValue::div(AdValue::scale(scratch.ad_value(296), scratch.values[95]), scratch.ad_value(292)), scratch.values[125]));
        }

        if (((scratch.values[270] != 0.0) && (!(scratch.values[422] != 0.0))) && (!(scratch.values[427] != 0.0))) {
            scratch.store_ad(300, &AdValue::div_from_scalar((0.666666666666667 * scratch.values[122]), scratch.ad_value(299)));
        }

        if (((scratch.values[270] != 0.0) && (!(scratch.values[422] != 0.0))) && (!(scratch.values[427] != 0.0))) {
            scratch.store_ad(301, &AdValue::square(scratch.ad_value(300)));
        }

        if (((scratch.values[270] != 0.0) && (!(scratch.values[422] != 0.0))) && (!(scratch.values[427] != 0.0))) {
            scratch.store_ad(302, &AdValue::sqrt(AdValue::div(AdValue::square(scratch.ad_value(301)), AdValue::offset(AdValue::square(scratch.ad_value(301)), 1.0))));
        }

        if (((scratch.values[270] != 0.0) && (!(scratch.values[422] != 0.0))) && (!(scratch.values[427] != 0.0))) {
            scratch.store_ad(303, &AdValue::sqrt(AdValue::abs(scratch.ad_value(302))));
        }

        if (((scratch.values[270] != 0.0) && (!(scratch.values[422] != 0.0))) && (!(scratch.values[427] != 0.0))) {
            scratch.store_ad(304, &AdValue::mul(scratch.ad_value(302), scratch.ad_value(303)));
        }

        scratch.values[428] = if (((-scratch.values[13]) * scratch.values[98]) == (-1.0)) { 1.0 } else { 0.0 };

        if ((((scratch.values[270] != 0.0) && (!(scratch.values[422] != 0.0))) && (!(scratch.values[427] != 0.0))) && (scratch.values[428] != 0.0)) {
            scratch.store_ad(305, &AdValue::div_from_scalar(1.0, AdValue::offset(AdValue::mul(scratch.ad_value(299), scratch.ad_value(304)), 1.0)));
        }

        if ((((scratch.values[270] != 0.0) && (!(scratch.values[422] != 0.0))) && (!(scratch.values[427] != 0.0))) && (!(scratch.values[428] != 0.0))) {
            scratch.store_ad(305, &AdValue::powf(AdValue::offset(AdValue::mul(scratch.ad_value(299), scratch.ad_value(304)), 1.0), ((-scratch.values[13]) * scratch.values[98])));
        }

        if (((scratch.values[270] != 0.0) && (!(scratch.values[422] != 0.0))) && (!(scratch.values[427] != 0.0))) {
            scratch.store_ad(306, &AdValue::div(AdValue::mul(scratch.ad_value(295), scratch.ad_value(305)), AdValue::add(scratch.ad_value(295), scratch.ad_value(305))));
        }

        if (((scratch.values[270] != 0.0) && (!(scratch.values[422] != 0.0))) && (!(scratch.values[427] != 0.0))) {
            scratch.store_ad(307, &AdValue::sqrt(AdValue::scale(AdValue::div(scratch.ad_value(299), scratch.ad_value(303)), 0.375)));
        }

        if (((scratch.values[270] != 0.0) && (!(scratch.values[422] != 0.0))) && (!(scratch.values[427] != 0.0))) {
            scratch.store_ad(308, &AdValue::sub(AdValue::scale(AdValue::mul(scratch.ad_value(300), scratch.ad_value(303)), 2.0), scratch.ad_value(302)));
        }

        if (((scratch.values[270] != 0.0) && (!(scratch.values[422] != 0.0))) && (!(scratch.values[427] != 0.0))) {
            scratch.store_ad(309, &AdValue::add(AdValue::sub(AdValue::mul(AdValue::scale(scratch.ad_value(300), scratch.values[122]), scratch.ad_value(303)), AdValue::scale(scratch.ad_value(302), scratch.values[122])), AdValue::scale(AdValue::mul(scratch.ad_value(299), scratch.ad_value(304)), 0.5)));
        }

        if (((scratch.values[270] != 0.0) && (!(scratch.values[422] != 0.0))) && (!(scratch.values[427] != 0.0))) {
            scratch.store_ad(310, &AdValue::mul(AdValue::offset(scratch.ad_value(308), (-1.0)), scratch.ad_value(307)));
        }

        if (((scratch.values[270] != 0.0) && (!(scratch.values[422] != 0.0))) && (!(scratch.values[427] != 0.0))) {
            scratch.store_ad(271, &AdValue::square(scratch.ad_value(310)));
        }

        scratch.values[429] = if (scratch.values[310] > 0.0) { 1.0 } else { 0.0 };

        if ((((scratch.values[270] != 0.0) && (!(scratch.values[422] != 0.0))) && (!(scratch.values[427] != 0.0))) && (scratch.values[429] != 0.0)) {
            scratch.store_ad(272, &AdValue::div_from_scalar(1.0, AdValue::offset(AdValue::scale(scratch.ad_value(310), scratch.values[57]), 1.0)));
        }

        if ((((scratch.values[270] != 0.0) && (!(scratch.values[422] != 0.0))) && (!(scratch.values[427] != 0.0))) && (!(scratch.values[429] != 0.0))) {
            scratch.store_ad(272, &AdValue::div_from_scalar(1.0, AdValue::sub_from_scalar(1.0, AdValue::scale(scratch.ad_value(310), scratch.values[57]))));
        }

        scratch.values[430] = if (((-scratch.values[271]) + scratch.values[309]) > (-230.25850929940458)) { 1.0 } else { 0.0 };

        if ((((scratch.values[270] != 0.0) && (!(scratch.values[422] != 0.0))) && (!(scratch.values[427] != 0.0))) && (scratch.values[430] != 0.0)) {
            scratch.store_ad(289, &AdValue::exp(AdValue::sub(scratch.ad_value(309), scratch.ad_value(271))));
        }

    }

    pub(super) fn stamp_transient_block_5(
        &mut self,
        ctx: &GeneratedEvalContext<'_>,
        stamper: &mut GeneratedStamper<'_>,
        scratch: &mut Scratch,
    ) {
        let _ = stamper;
        if ((((scratch.values[270] != 0.0) && (!(scratch.values[422] != 0.0))) && (!(scratch.values[427] != 0.0))) && (!(scratch.values[430] != 0.0))) {
            scratch.store_ad(289, &AdValue::div_from_scalar(1e-100, AdValue::offset(AdValue::mul(AdValue::sub_from_scalar((-230.25850929940458), AdValue::sub(scratch.ad_value(309), scratch.ad_value(271))), AdValue::offset(AdValue::scale(AdValue::mul(AdValue::sub_from_scalar((-230.25850929940458), AdValue::sub(scratch.ad_value(309), scratch.ad_value(271))), AdValue::offset(AdValue::scale(AdValue::sub_from_scalar((-230.25850929940458), AdValue::sub(scratch.ad_value(309), scratch.ad_value(271))), 0.3333333333333333), 1.0)), 0.5), 1.0)), 1.0)));
        }

        if (((scratch.values[270] != 0.0) && (!(scratch.values[422] != 0.0))) && (!(scratch.values[427] != 0.0))) {
            scratch.store_ad(273, &AdValue::mul(AdValue::add(AdValue::add(AdValue::scale(scratch.ad_value(272), 0.29214664), AdValue::scale(AdValue::square(scratch.ad_value(272)), scratch.values[58])), AdValue::scale(AdValue::mul(AdValue::square(scratch.ad_value(272)), scratch.ad_value(272)), scratch.values[59])), scratch.ad_value(289)));
        }

        scratch.values[431] = if (scratch.values[310] > 0.0) { 1.0 } else { 0.0 };

        if ((((scratch.values[270] != 0.0) && (!(scratch.values[422] != 0.0))) && (!(scratch.values[427] != 0.0))) && (scratch.values[431] != 0.0)) {
            scratch.values[311] = scratch.values[273];
            scratch.node_derivatives[311] = scratch.node_derivatives[273];
            scratch.branch_derivatives[311] = scratch.branch_derivatives[273];
        }

        scratch.values[432] = if (scratch.values[309] > (-230.25850929940458)) { 1.0 } else { 0.0 };

        if (((((scratch.values[270] != 0.0) && (!(scratch.values[422] != 0.0))) && (!(scratch.values[427] != 0.0))) && (!(scratch.values[431] != 0.0))) && (scratch.values[432] != 0.0)) {
            scratch.store_ad(289, &AdValue::exp(scratch.ad_value(309)));
        }

        if (((((scratch.values[270] != 0.0) && (!(scratch.values[422] != 0.0))) && (!(scratch.values[427] != 0.0))) && (!(scratch.values[431] != 0.0))) && (!(scratch.values[432] != 0.0))) {
            scratch.store_ad(289, &AdValue::div_from_scalar(1e-100, AdValue::offset(AdValue::mul(AdValue::sub_from_scalar((-230.25850929940458), scratch.ad_value(309)), AdValue::offset(AdValue::scale(AdValue::mul(AdValue::sub_from_scalar((-230.25850929940458), scratch.ad_value(309)), AdValue::offset(AdValue::scale(AdValue::sub_from_scalar((-230.25850929940458), scratch.ad_value(309)), 0.3333333333333333), 1.0)), 0.5), 1.0)), 1.0)));
        }

        if ((((scratch.values[270] != 0.0) && (!(scratch.values[422] != 0.0))) && (!(scratch.values[427] != 0.0))) && (!(scratch.values[431] != 0.0))) {
            scratch.store_ad(311, &AdValue::sub(AdValue::scale(scratch.ad_value(289), 2.0), scratch.ad_value(273)));
        }

        if (((scratch.values[270] != 0.0) && (!(scratch.values[422] != 0.0))) && (!(scratch.values[427] != 0.0))) {
            scratch.store_ad(312, &AdValue::scale(AdValue::div(AdValue::scale(scratch.ad_value(311), scratch.values[122]), scratch.ad_value(307)), (1.772453850905516 * 0.5)));
        }

        if (((scratch.values[270] != 0.0) && (!(scratch.values[422] != 0.0))) && (!(scratch.values[427] != 0.0))) {
            scratch.store_ad(298, &AdValue::scale(AdValue::mul(AdValue::mul(scratch.ad_value(297), scratch.ad_value(312)), scratch.ad_value(306)), scratch.values[27]));
        }

        scratch.values[433] = if (scratch.values[33] == 0.0) { 1.0 } else { 0.0 };

        if (((scratch.values[270] != 0.0) && (!(scratch.values[422] != 0.0))) && (scratch.values[433] != 0.0)) {
            scratch.values[313] = 0.0;
            scratch.node_derivatives[313] = [0.0; Instance::NODE_COUNT];
            scratch.branch_derivatives[313] = [0.0; Instance::BRANCH_COUNT];
        }

        scratch.values[434] = if (scratch.values[13] == 0.5) { 1.0 } else { 0.0 };

        if ((((scratch.values[270] != 0.0) && (!(scratch.values[422] != 0.0))) && (!(scratch.values[433] != 0.0))) && (scratch.values[434] != 0.0)) {
            scratch.store_ad(289, &AdValue::sqrt(AdValue::scale(AdValue::sub_from_scalar(scratch.values[10], scratch.ad_value(287)), scratch.values[116])));
        }

        if ((((scratch.values[270] != 0.0) && (!(scratch.values[422] != 0.0))) && (!(scratch.values[433] != 0.0))) && (!(scratch.values[434] != 0.0))) {
            scratch.store_ad(289, &AdValue::powf(AdValue::scale(AdValue::sub_from_scalar(scratch.values[10], scratch.ad_value(287)), scratch.values[116]), scratch.values[13]));
        }

        if (((scratch.values[270] != 0.0) && (!(scratch.values[422] != 0.0))) && (!(scratch.values[433] != 0.0))) {
            scratch.store_ad(314, &AdValue::scale(AdValue::div(AdValue::scale(AdValue::sub_from_scalar(scratch.values[10], scratch.ad_value(287)), scratch.values[113]), scratch.ad_value(289)), scratch.values[98]));
        }

        scratch.values[435] = if (((((-scratch.values[128]) / scratch.values[314])) as f64).abs() < 230.25850929940458) { 1.0 } else { 0.0 };

        if ((((scratch.values[270] != 0.0) && (!(scratch.values[422] != 0.0))) && (!(scratch.values[433] != 0.0))) && (scratch.values[435] != 0.0)) {
            scratch.store_ad(289, &AdValue::exp(AdValue::div(AdValue::neg(scratch.ad_value(128)), scratch.ad_value(314))));
        }

        scratch.values[436] = if (((-scratch.values[128]) / scratch.values[314]) < 0.0) { 1.0 } else { 0.0 };

        if (((((scratch.values[270] != 0.0) && (!(scratch.values[422] != 0.0))) && (!(scratch.values[433] != 0.0))) && (!(scratch.values[435] != 0.0))) && (scratch.values[436] != 0.0)) {
            let assign7600_ad_e8628: AdValue = AdValue::offset(AdValue::mul(AdValue::sub_from_scalar((-230.25850929940458), AdValue::div(AdValue::neg(scratch.ad_value(128)), scratch.ad_value(314))), AdValue::offset(AdValue::scale(AdValue::mul(AdValue::sub_from_scalar((-230.25850929940458), AdValue::div(AdValue::neg(scratch.ad_value(128)), scratch.ad_value(314))), AdValue::offset(AdValue::scale(AdValue::sub_from_scalar((-230.25850929940458), AdValue::div(AdValue::neg(scratch.ad_value(128)), scratch.ad_value(314))), 0.3333333333333333), 1.0)), 0.5), 1.0)), 1.0);
            scratch.store_ad(289, &AdValue::div_from_scalar(1e-100, assign7600_ad_e8628));
        }

        if (((((scratch.values[270] != 0.0) && (!(scratch.values[422] != 0.0))) && (!(scratch.values[433] != 0.0))) && (!(scratch.values[435] != 0.0))) && (!(scratch.values[436] != 0.0))) {
            let assign7610_ad_e8676: AdValue = AdValue::scale(AdValue::offset(AdValue::mul(AdValue::offset(AdValue::div(AdValue::neg(scratch.ad_value(128)), scratch.ad_value(314)), (-230.25850929940458)), AdValue::offset(AdValue::scale(AdValue::mul(AdValue::offset(AdValue::div(AdValue::neg(scratch.ad_value(128)), scratch.ad_value(314)), (-230.25850929940458)), AdValue::offset(AdValue::scale(AdValue::offset(AdValue::div(AdValue::neg(scratch.ad_value(128)), scratch.ad_value(314)), (-230.25850929940458)), 0.3333333333333333), 1.0)), 0.5), 1.0)), 1.0), 1e100);
            scratch.store_ad(289, &assign7610_ad_e8676);
        }

        if (((scratch.values[270] != 0.0) && (!(scratch.values[422] != 0.0))) && (!(scratch.values[433] != 0.0))) {
            scratch.store_ad(313, &AdValue::scale(AdValue::mul(AdValue::mul(AdValue::mul(scratch.ad_value(153), scratch.ad_value(314)), scratch.ad_value(314)), scratch.ad_value(289)), scratch.values[33]));
        }

        scratch.values[437] = if (scratch.values[42] > 1000.0) { 1.0 } else { 0.0 };

        if (((scratch.values[270] != 0.0) && (!(scratch.values[422] != 0.0))) && (scratch.values[437] != 0.0)) {
            scratch.values[315] = 1.0;
            scratch.node_derivatives[315] = [0.0; Instance::NODE_COUNT];
            scratch.branch_derivatives[315] = [0.0; Instance::BRANCH_COUNT];
        }

        scratch.values[438] = if (scratch.values[288] > ((-scratch.values[129]) * scratch.values[42])) { 1.0 } else { 0.0 };

        scratch.values[439] = if (scratch.values[45] == 4.0) { 1.0 } else { 0.0 };

        if (((((scratch.values[270] != 0.0) && (!(scratch.values[422] != 0.0))) && (!(scratch.values[437] != 0.0))) && (scratch.values[438] != 0.0)) && (scratch.values[439] != 0.0)) {
            scratch.store_ad(289, &AdValue::mul(AdValue::mul(AdValue::mul(AdValue::scale(scratch.ad_value(288), scratch.values[135]), AdValue::scale(scratch.ad_value(288), scratch.values[135])), AdValue::scale(scratch.ad_value(288), scratch.values[135])), AdValue::scale(scratch.ad_value(288), scratch.values[135])));
        }

        if (((((scratch.values[270] != 0.0) && (!(scratch.values[422] != 0.0))) && (!(scratch.values[437] != 0.0))) && (scratch.values[438] != 0.0)) && (!(scratch.values[439] != 0.0))) {
            scratch.store_ad(289, &AdValue::powf(AdValue::abs(AdValue::scale(scratch.ad_value(288), scratch.values[135])), scratch.values[45]));
        }

        if ((((scratch.values[270] != 0.0) && (!(scratch.values[422] != 0.0))) && (!(scratch.values[437] != 0.0))) && (scratch.values[438] != 0.0)) {
            scratch.store_ad(315, &AdValue::div_from_scalar(1.0, AdValue::sub_from_scalar(1.0, scratch.ad_value(289))));
        }

        if ((((scratch.values[270] != 0.0) && (!(scratch.values[422] != 0.0))) && (!(scratch.values[437] != 0.0))) && (!(scratch.values[438] != 0.0))) {
            scratch.store_ad(315, &AdValue::offset(AdValue::scale(AdValue::offset(scratch.ad_value(288), (scratch.values[129] * scratch.values[42])), scratch.values[138]), scratch.values[132]));
        }

        if ((scratch.values[270] != 0.0) && (!(scratch.values[422] != 0.0))) {
            scratch.store_ad(318, &AdValue::mul(AdValue::add(AdValue::add(AdValue::add(scratch.ad_value(290), scratch.ad_value(291)), scratch.ad_value(298)), scratch.ad_value(313)), scratch.ad_value(315)));
        }

        if (scratch.values[270] != 0.0) {
            scratch.store_ad(143, &AdValue::add(AdValue::add(AdValue::scale(scratch.ad_value(316), scratch.values[217]), AdValue::scale(scratch.ad_value(317), scratch.values[218])), AdValue::scale(scratch.ad_value(318), scratch.values[219])));
        }

        if (scratch.values[270] != 0.0) {
            scratch.values[287] = 0.0;
            scratch.node_derivatives[287] = [0.0; Instance::NODE_COUNT];
            scratch.branch_derivatives[287] = [0.0; Instance::BRANCH_COUNT];
        }

        if (scratch.values[270] != 0.0) {
            scratch.values[284] = 0.0;
            scratch.node_derivatives[284] = [0.0; Instance::NODE_COUNT];
            scratch.branch_derivatives[284] = [0.0; Instance::BRANCH_COUNT];
        }

        scratch.values[440] = if !(((scratch.values[217] == 0.0) && (scratch.values[218] == 0.0)) && (scratch.values[219] == 0.0)) { 1.0 } else { 0.0 };

        if ((scratch.values[270] != 0.0) && (scratch.values[440] != 0.0)) {
            scratch.store_ad(274, &AdValue::mul(AdValue::scale(scratch.ad_value(226), 4.0), scratch.ad_value(226)));
        }

        if ((scratch.values[270] != 0.0) && (scratch.values[440] != 0.0)) {
            scratch.store_ad(275, &AdValue::div(scratch.ad_value(226), scratch.ad_value(227)));
        }

        if ((scratch.values[270] != 0.0) && (scratch.values[440] != 0.0)) {
            scratch.store_ad(276, &AdValue::add(scratch.ad_value(154), AdValue::mul(scratch.ad_value(226), scratch.ad_value(275))));
        }

        if ((scratch.values[270] != 0.0) && (scratch.values[440] != 0.0)) {
            scratch.store_ad(277, &AdValue::add(scratch.ad_value(227), scratch.ad_value(276)));
        }

        if ((scratch.values[270] != 0.0) && (scratch.values[440] != 0.0)) {
            scratch.store_ad(278, &AdValue::sub(scratch.ad_value(227), scratch.ad_value(276)));
        }

        if ((scratch.values[270] != 0.0) && (scratch.values[440] != 0.0)) {
            scratch.store_ad(279, &AdValue::sqrt(AdValue::add(AdValue::square(scratch.ad_value(278)), scratch.ad_value(274))));
        }

        if ((scratch.values[270] != 0.0) && (scratch.values[440] != 0.0)) {
            scratch.store_ad(281, &AdValue::scale(AdValue::div(AdValue::mul(scratch.ad_value(154), scratch.ad_value(227)), AdValue::add(scratch.ad_value(277), scratch.ad_value(279))), 2.0));
        }

        scratch.values[441] = if (scratch.values[154] < scratch.values[223]) { 1.0 } else { 0.0 };

        scratch.values[442] = if ((((0.5 * (scratch.values[154] * scratch.values[56]))) as f64).abs() < 230.25850929940458) { 1.0 } else { 0.0 };

        if ((((scratch.values[270] != 0.0) && (scratch.values[440] != 0.0)) && (scratch.values[441] != 0.0)) && (scratch.values[442] != 0.0)) {
            scratch.store_ad(283, &AdValue::exp(AdValue::scale(scratch.ad_value(154), (scratch.values[56] * 0.5))));
        }

        scratch.values[443] = if ((0.5 * (scratch.values[154] * scratch.values[56])) < 0.0) { 1.0 } else { 0.0 };

        if (((((scratch.values[270] != 0.0) && (scratch.values[440] != 0.0)) && (scratch.values[441] != 0.0)) && (!(scratch.values[442] != 0.0))) && (scratch.values[443] != 0.0)) {
            let assign7870_ad_e9001: AdValue = AdValue::div_from_scalar(1e-100, AdValue::offset(AdValue::mul(AdValue::sub_from_scalar((-230.25850929940458), AdValue::scale(scratch.ad_value(154), (scratch.values[56] * 0.5))), AdValue::offset(AdValue::scale(AdValue::mul(AdValue::sub_from_scalar((-230.25850929940458), AdValue::scale(scratch.ad_value(154), (scratch.values[56] * 0.5))), AdValue::offset(AdValue::scale(AdValue::sub_from_scalar((-230.25850929940458), AdValue::scale(scratch.ad_value(154), (scratch.values[56] * 0.5))), 0.3333333333333333), 1.0)), 0.5), 1.0)), 1.0));
            scratch.store_ad(283, &assign7870_ad_e9001);
        }

        if (((((scratch.values[270] != 0.0) && (scratch.values[440] != 0.0)) && (scratch.values[441] != 0.0)) && (!(scratch.values[442] != 0.0))) && (!(scratch.values[443] != 0.0))) {
            scratch.store_ad(283, &AdValue::scale(AdValue::offset(AdValue::mul(AdValue::offset(AdValue::scale(scratch.ad_value(154), (scratch.values[56] * 0.5)), (-230.25850929940458)), AdValue::offset(AdValue::scale(AdValue::mul(AdValue::offset(AdValue::scale(scratch.ad_value(154), (scratch.values[56] * 0.5)), (-230.25850929940458)), AdValue::offset(AdValue::scale(AdValue::offset(AdValue::scale(scratch.ad_value(154), (scratch.values[56] * 0.5)), (-230.25850929940458)), 0.3333333333333333), 1.0)), 0.5), 1.0)), 1.0), 1e100));
        }

        if (((scratch.values[270] != 0.0) && (scratch.values[440] != 0.0)) && (scratch.values[441] != 0.0)) {
            scratch.store_ad(280, &AdValue::square(scratch.ad_value(283)));
        }

        if (((scratch.values[270] != 0.0) && (scratch.values[440] != 0.0)) && (!(scratch.values[441] != 0.0))) {
            scratch.store_ad(280, &AdValue::mul(AdValue::offset(AdValue::scale(AdValue::sub(scratch.ad_value(154), scratch.ad_value(223)), scratch.values[56]), 1.0), scratch.ad_value(224)));
        }

        if (((scratch.values[270] != 0.0) && (scratch.values[440] != 0.0)) && (!(scratch.values[441] != 0.0))) {
            scratch.store_ad(283, &AdValue::sqrt(scratch.ad_value(280)));
        }

        if ((scratch.values[270] != 0.0) && (scratch.values[440] != 0.0)) {
            scratch.store_ad(280, &AdValue::offset(scratch.ad_value(280), (-1.0)));
        }

        if ((scratch.values[270] != 0.0) && (scratch.values[440] != 0.0)) {
            scratch.store_ad(282, &AdValue::div_from_scalar(1.0, scratch.ad_value(283)));
        }

        scratch.values[444] = if (scratch.values[154] > 0.0) { 1.0 } else { 0.0 };

        if (((scratch.values[270] != 0.0) && (scratch.values[440] != 0.0)) && (scratch.values[444] != 0.0)) {
            scratch.store_ad(284, &AdValue::scale(AdValue::ln(AdValue::add(AdValue::offset(scratch.ad_value(282), 2.0), AdValue::sqrt(AdValue::mul(AdValue::offset(scratch.ad_value(282), 1.0), AdValue::offset(scratch.ad_value(282), 3.0))))), (scratch.values[55] * 2.0)));
        }

        if (((scratch.values[270] != 0.0) && (scratch.values[440] != 0.0)) && (!(scratch.values[444] != 0.0))) {
            scratch.store_ad(284, &AdValue::sub(AdValue::scale(AdValue::ln(AdValue::add(AdValue::offset(AdValue::scale(scratch.ad_value(283), 2.0), 1.0), AdValue::sqrt(AdValue::mul(AdValue::offset(scratch.ad_value(283), 1.0), AdValue::offset(AdValue::scale(scratch.ad_value(283), 3.0), 1.0))))), (scratch.values[55] * 2.0)), scratch.ad_value(154)));
        }

        if ((scratch.values[270] != 0.0) && (scratch.values[440] != 0.0)) {
            scratch.store_ad(285, &AdValue::sub(scratch.ad_value(225), scratch.ad_value(284)));
        }

        if ((scratch.values[270] != 0.0) && (scratch.values[440] != 0.0)) {
            scratch.store_ad(286, &AdValue::scale(AdValue::sub(AdValue::add(scratch.ad_value(154), scratch.ad_value(285)), AdValue::sqrt(AdValue::offset(AdValue::mul(AdValue::sub(scratch.ad_value(154), scratch.ad_value(285)), AdValue::sub(scratch.ad_value(154), scratch.ad_value(285))), ((4.0 * scratch.values[55]) * scratch.values[55])))), 0.5));
        }

        if ((scratch.values[270] != 0.0) && (scratch.values[440] != 0.0)) {
            scratch.store_ad(287, &AdValue::scale(AdValue::sub(AdValue::add(scratch.ad_value(154), scratch.ad_value(228)), AdValue::sqrt(AdValue::offset(AdValue::mul(AdValue::sub(scratch.ad_value(154), scratch.ad_value(228)), AdValue::sub(scratch.ad_value(154), scratch.ad_value(228))), ((4.0 * scratch.values[53]) * scratch.values[53])))), 0.5));
        }

        if ((scratch.values[270] != 0.0) && (scratch.values[440] != 0.0)) {
            scratch.store_ad(288, &AdValue::scale(AdValue::sub(scratch.ad_value(154), AdValue::sqrt(AdValue::offset(AdValue::mul(scratch.ad_value(154), scratch.ad_value(154)), ((4.0 * 1e-6) * 1e-6)))), 0.5));
        }

        scratch.values[445] = if (scratch.values[217] == 0.0) { 1.0 } else { 0.0 };

        if ((scratch.values[270] != 0.0) && (scratch.values[445] != 0.0)) {
            scratch.values[316] = 0.0;
            scratch.node_derivatives[316] = [0.0; Instance::NODE_COUNT];
            scratch.branch_derivatives[316] = [0.0; Instance::BRANCH_COUNT];
        }

        scratch.values[446] = if (scratch.values[93] == 0.5) { 1.0 } else { 0.0 };

        if (((scratch.values[270] != 0.0) && (!(scratch.values[445] != 0.0))) && (scratch.values[446] != 0.0)) {
            scratch.store_ad(289, &AdValue::sqrt(AdValue::sub_from_scalar(1.0, AdValue::scale(scratch.ad_value(281), scratch.values[90]))));
        }

        if (((scratch.values[270] != 0.0) && (!(scratch.values[445] != 0.0))) && (!(scratch.values[446] != 0.0))) {
            scratch.store_ad(289, &AdValue::powf(AdValue::sub_from_scalar(1.0, AdValue::scale(scratch.ad_value(281), scratch.values[90])), scratch.values[93]));
        }

        if ((scratch.values[270] != 0.0) && (!(scratch.values[445] != 0.0))) {
            scratch.store_ad(290, &AdValue::scale(scratch.ad_value(280), scratch.values[72]));
        }

        scratch.values[447] = if ((scratch.values[22] == 0.0) && (scratch.values[25] == 0.0)) { 1.0 } else { 0.0 };

        if (((scratch.values[270] != 0.0) && (!(scratch.values[445] != 0.0))) && (scratch.values[447] != 0.0)) {
            scratch.values[291] = 0.0;
            scratch.node_derivatives[291] = [0.0; Instance::NODE_COUNT];
            scratch.branch_derivatives[291] = [0.0; Instance::BRANCH_COUNT];
        }

        if (((scratch.values[270] != 0.0) && (!(scratch.values[445] != 0.0))) && (!(scratch.values[447] != 0.0))) {
            scratch.store_ad(292, &AdValue::sub_from_scalar(scratch.values[78], scratch.ad_value(286)));
        }

        if (((scratch.values[270] != 0.0) && (!(scratch.values[445] != 0.0))) && (!(scratch.values[447] != 0.0))) {
            scratch.store_ad(293, &AdValue::sub_from_scalar(1.0, AdValue::sqrt(AdValue::sub_from_scalar(1.0, AdValue::div(scratch.ad_value(284), scratch.ad_value(292))))));
        }

        scratch.values[448] = if (scratch.values[11] == 0.5) { 1.0 } else { 0.0 };

        if ((((scratch.values[270] != 0.0) && (!(scratch.values[445] != 0.0))) && (!(scratch.values[447] != 0.0))) && (scratch.values[448] != 0.0)) {
            scratch.values[294] = 0.0;
            scratch.node_derivatives[294] = [0.0; Instance::NODE_COUNT];
            scratch.branch_derivatives[294] = [0.0; Instance::BRANCH_COUNT];
        }

        if ((((scratch.values[270] != 0.0) && (!(scratch.values[445] != 0.0))) && (!(scratch.values[447] != 0.0))) && (!(scratch.values[448] != 0.0))) {
            scratch.store_ad(294, &AdValue::scale(AdValue::add(AdValue::div(AdValue::mul(AdValue::square(scratch.ad_value(293)), AdValue::ln(scratch.ad_value(293))), AdValue::sub_from_scalar(1.0, scratch.ad_value(293))), scratch.ad_value(293)), (1.0 - (2.0 * scratch.values[11]))));
        }

        if (((scratch.values[270] != 0.0) && (!(scratch.values[445] != 0.0))) && (!(scratch.values[447] != 0.0))) {
            scratch.store_ad(295, &AdValue::add(scratch.ad_value(293), scratch.ad_value(294)));
        }

        scratch.values[449] = if (scratch.values[11] == 0.5) { 1.0 } else { 0.0 };

        if ((((scratch.values[270] != 0.0) && (!(scratch.values[445] != 0.0))) && (!(scratch.values[447] != 0.0))) && (scratch.values[449] != 0.0)) {
            scratch.store_ad(289, &AdValue::sqrt(AdValue::scale(scratch.ad_value(292), scratch.values[114])));
        }

        if ((((scratch.values[270] != 0.0) && (!(scratch.values[445] != 0.0))) && (!(scratch.values[447] != 0.0))) && (!(scratch.values[449] != 0.0))) {
            scratch.store_ad(289, &AdValue::powf(AdValue::scale(scratch.ad_value(292), scratch.values[114]), scratch.values[11]));
        }

        if (((scratch.values[270] != 0.0) && (!(scratch.values[445] != 0.0))) && (!(scratch.values[447] != 0.0))) {
            scratch.store_ad(296, &AdValue::scale(scratch.ad_value(289), scratch.values[108]));
        }

        if (((scratch.values[270] != 0.0) && (!(scratch.values[445] != 0.0))) && (!(scratch.values[447] != 0.0))) {
            scratch.store_ad(297, &AdValue::scale(AdValue::mul(AdValue::offset(scratch.ad_value(283), (-1.0)), scratch.ad_value(296)), scratch.values[69]));
        }

        if (((scratch.values[270] != 0.0) && (!(scratch.values[445] != 0.0))) && (!(scratch.values[447] != 0.0))) {
            scratch.store_ad(291, &AdValue::scale(AdValue::mul(scratch.ad_value(297), scratch.ad_value(295)), scratch.values[22]));
        }

        scratch.values[450] = if (scratch.values[25] == 0.0) { 1.0 } else { 0.0 };

        if (((scratch.values[270] != 0.0) && (!(scratch.values[445] != 0.0))) && (scratch.values[450] != 0.0)) {
            scratch.values[298] = 0.0;
            scratch.node_derivatives[298] = [0.0; Instance::NODE_COUNT];
            scratch.branch_derivatives[298] = [0.0; Instance::BRANCH_COUNT];
        }

        if (((scratch.values[270] != 0.0) && (!(scratch.values[445] != 0.0))) && (!(scratch.values[450] != 0.0))) {
            scratch.store_ad(299, &AdValue::scale(AdValue::div(AdValue::scale(scratch.ad_value(296), scratch.values[93]), scratch.ad_value(292)), scratch.values[123]));
        }

        if (((scratch.values[270] != 0.0) && (!(scratch.values[445] != 0.0))) && (!(scratch.values[450] != 0.0))) {
            scratch.store_ad(300, &AdValue::div_from_scalar((0.666666666666667 * scratch.values[120]), scratch.ad_value(299)));
        }

        if (((scratch.values[270] != 0.0) && (!(scratch.values[445] != 0.0))) && (!(scratch.values[450] != 0.0))) {
            scratch.store_ad(301, &AdValue::square(scratch.ad_value(300)));
        }

        if (((scratch.values[270] != 0.0) && (!(scratch.values[445] != 0.0))) && (!(scratch.values[450] != 0.0))) {
            scratch.store_ad(302, &AdValue::sqrt(AdValue::div(AdValue::square(scratch.ad_value(301)), AdValue::offset(AdValue::square(scratch.ad_value(301)), 1.0))));
        }

        if (((scratch.values[270] != 0.0) && (!(scratch.values[445] != 0.0))) && (!(scratch.values[450] != 0.0))) {
            scratch.store_ad(303, &AdValue::sqrt(AdValue::abs(scratch.ad_value(302))));
        }

        if (((scratch.values[270] != 0.0) && (!(scratch.values[445] != 0.0))) && (!(scratch.values[450] != 0.0))) {
            scratch.store_ad(304, &AdValue::mul(scratch.ad_value(302), scratch.ad_value(303)));
        }

        scratch.values[451] = if (((-scratch.values[11]) * scratch.values[96]) == (-1.0)) { 1.0 } else { 0.0 };

        if ((((scratch.values[270] != 0.0) && (!(scratch.values[445] != 0.0))) && (!(scratch.values[450] != 0.0))) && (scratch.values[451] != 0.0)) {
            scratch.store_ad(305, &AdValue::div_from_scalar(1.0, AdValue::offset(AdValue::mul(scratch.ad_value(299), scratch.ad_value(304)), 1.0)));
        }

        if ((((scratch.values[270] != 0.0) && (!(scratch.values[445] != 0.0))) && (!(scratch.values[450] != 0.0))) && (!(scratch.values[451] != 0.0))) {
            scratch.store_ad(305, &AdValue::powf(AdValue::offset(AdValue::mul(scratch.ad_value(299), scratch.ad_value(304)), 1.0), ((-scratch.values[11]) * scratch.values[96])));
        }

        if (((scratch.values[270] != 0.0) && (!(scratch.values[445] != 0.0))) && (!(scratch.values[450] != 0.0))) {
            scratch.store_ad(306, &AdValue::div(AdValue::mul(scratch.ad_value(295), scratch.ad_value(305)), AdValue::add(scratch.ad_value(295), scratch.ad_value(305))));
        }

        if (((scratch.values[270] != 0.0) && (!(scratch.values[445] != 0.0))) && (!(scratch.values[450] != 0.0))) {
            scratch.store_ad(307, &AdValue::sqrt(AdValue::scale(AdValue::div(scratch.ad_value(299), scratch.ad_value(303)), 0.375)));
        }

        if (((scratch.values[270] != 0.0) && (!(scratch.values[445] != 0.0))) && (!(scratch.values[450] != 0.0))) {
            scratch.store_ad(308, &AdValue::sub(AdValue::scale(AdValue::mul(scratch.ad_value(300), scratch.ad_value(303)), 2.0), scratch.ad_value(302)));
        }

        if (((scratch.values[270] != 0.0) && (!(scratch.values[445] != 0.0))) && (!(scratch.values[450] != 0.0))) {
            scratch.store_ad(309, &AdValue::add(AdValue::sub(AdValue::mul(AdValue::scale(scratch.ad_value(300), scratch.values[120]), scratch.ad_value(303)), AdValue::scale(scratch.ad_value(302), scratch.values[120])), AdValue::scale(AdValue::mul(scratch.ad_value(299), scratch.ad_value(304)), 0.5)));
        }

        if (((scratch.values[270] != 0.0) && (!(scratch.values[445] != 0.0))) && (!(scratch.values[450] != 0.0))) {
            scratch.store_ad(310, &AdValue::mul(AdValue::offset(scratch.ad_value(308), (-1.0)), scratch.ad_value(307)));
        }

        if (((scratch.values[270] != 0.0) && (!(scratch.values[445] != 0.0))) && (!(scratch.values[450] != 0.0))) {
            scratch.store_ad(271, &AdValue::square(scratch.ad_value(310)));
        }

        scratch.values[452] = if (scratch.values[310] > 0.0) { 1.0 } else { 0.0 };

        if ((((scratch.values[270] != 0.0) && (!(scratch.values[445] != 0.0))) && (!(scratch.values[450] != 0.0))) && (scratch.values[452] != 0.0)) {
            scratch.store_ad(272, &AdValue::div_from_scalar(1.0, AdValue::offset(AdValue::scale(scratch.ad_value(310), scratch.values[57]), 1.0)));
        }

        if ((((scratch.values[270] != 0.0) && (!(scratch.values[445] != 0.0))) && (!(scratch.values[450] != 0.0))) && (!(scratch.values[452] != 0.0))) {
            scratch.store_ad(272, &AdValue::div_from_scalar(1.0, AdValue::sub_from_scalar(1.0, AdValue::scale(scratch.ad_value(310), scratch.values[57]))));
        }

        scratch.values[453] = if (((-scratch.values[271]) + scratch.values[309]) > (-230.25850929940458)) { 1.0 } else { 0.0 };

        if ((((scratch.values[270] != 0.0) && (!(scratch.values[445] != 0.0))) && (!(scratch.values[450] != 0.0))) && (scratch.values[453] != 0.0)) {
            scratch.store_ad(289, &AdValue::exp(AdValue::sub(scratch.ad_value(309), scratch.ad_value(271))));
        }

        if ((((scratch.values[270] != 0.0) && (!(scratch.values[445] != 0.0))) && (!(scratch.values[450] != 0.0))) && (!(scratch.values[453] != 0.0))) {
            scratch.store_ad(289, &AdValue::div_from_scalar(1e-100, AdValue::offset(AdValue::mul(AdValue::sub_from_scalar((-230.25850929940458), AdValue::sub(scratch.ad_value(309), scratch.ad_value(271))), AdValue::offset(AdValue::scale(AdValue::mul(AdValue::sub_from_scalar((-230.25850929940458), AdValue::sub(scratch.ad_value(309), scratch.ad_value(271))), AdValue::offset(AdValue::scale(AdValue::sub_from_scalar((-230.25850929940458), AdValue::sub(scratch.ad_value(309), scratch.ad_value(271))), 0.3333333333333333), 1.0)), 0.5), 1.0)), 1.0)));
        }

        if (((scratch.values[270] != 0.0) && (!(scratch.values[445] != 0.0))) && (!(scratch.values[450] != 0.0))) {
            scratch.store_ad(273, &AdValue::mul(AdValue::add(AdValue::add(AdValue::scale(scratch.ad_value(272), 0.29214664), AdValue::scale(AdValue::square(scratch.ad_value(272)), scratch.values[58])), AdValue::scale(AdValue::mul(AdValue::square(scratch.ad_value(272)), scratch.ad_value(272)), scratch.values[59])), scratch.ad_value(289)));
        }

        scratch.values[454] = if (scratch.values[310] > 0.0) { 1.0 } else { 0.0 };

        if ((((scratch.values[270] != 0.0) && (!(scratch.values[445] != 0.0))) && (!(scratch.values[450] != 0.0))) && (scratch.values[454] != 0.0)) {
            scratch.values[311] = scratch.values[273];
            scratch.node_derivatives[311] = scratch.node_derivatives[273];
            scratch.branch_derivatives[311] = scratch.branch_derivatives[273];
        }

        scratch.values[455] = if (scratch.values[309] > (-230.25850929940458)) { 1.0 } else { 0.0 };

        if (((((scratch.values[270] != 0.0) && (!(scratch.values[445] != 0.0))) && (!(scratch.values[450] != 0.0))) && (!(scratch.values[454] != 0.0))) && (scratch.values[455] != 0.0)) {
            scratch.store_ad(289, &AdValue::exp(scratch.ad_value(309)));
        }

        if (((((scratch.values[270] != 0.0) && (!(scratch.values[445] != 0.0))) && (!(scratch.values[450] != 0.0))) && (!(scratch.values[454] != 0.0))) && (!(scratch.values[455] != 0.0))) {
            scratch.store_ad(289, &AdValue::div_from_scalar(1e-100, AdValue::offset(AdValue::mul(AdValue::sub_from_scalar((-230.25850929940458), scratch.ad_value(309)), AdValue::offset(AdValue::scale(AdValue::mul(AdValue::sub_from_scalar((-230.25850929940458), scratch.ad_value(309)), AdValue::offset(AdValue::scale(AdValue::sub_from_scalar((-230.25850929940458), scratch.ad_value(309)), 0.3333333333333333), 1.0)), 0.5), 1.0)), 1.0)));
        }

        if ((((scratch.values[270] != 0.0) && (!(scratch.values[445] != 0.0))) && (!(scratch.values[450] != 0.0))) && (!(scratch.values[454] != 0.0))) {
            scratch.store_ad(311, &AdValue::sub(AdValue::scale(scratch.ad_value(289), 2.0), scratch.ad_value(273)));
        }

        if (((scratch.values[270] != 0.0) && (!(scratch.values[445] != 0.0))) && (!(scratch.values[450] != 0.0))) {
            scratch.store_ad(312, &AdValue::scale(AdValue::div(AdValue::scale(scratch.ad_value(311), scratch.values[120]), scratch.ad_value(307)), (1.772453850905516 * 0.5)));
        }

        if (((scratch.values[270] != 0.0) && (!(scratch.values[445] != 0.0))) && (!(scratch.values[450] != 0.0))) {
            scratch.store_ad(298, &AdValue::scale(AdValue::mul(AdValue::mul(scratch.ad_value(297), scratch.ad_value(312)), scratch.ad_value(306)), scratch.values[25]));
        }

        scratch.values[456] = if (scratch.values[31] == 0.0) { 1.0 } else { 0.0 };

        if (((scratch.values[270] != 0.0) && (!(scratch.values[445] != 0.0))) && (scratch.values[456] != 0.0)) {
            scratch.values[313] = 0.0;
            scratch.node_derivatives[313] = [0.0; Instance::NODE_COUNT];
            scratch.branch_derivatives[313] = [0.0; Instance::BRANCH_COUNT];
        }

        scratch.values[457] = if (scratch.values[11] == 0.5) { 1.0 } else { 0.0 };

        if ((((scratch.values[270] != 0.0) && (!(scratch.values[445] != 0.0))) && (!(scratch.values[456] != 0.0))) && (scratch.values[457] != 0.0)) {
            scratch.store_ad(289, &AdValue::sqrt(AdValue::scale(AdValue::sub_from_scalar(scratch.values[8], scratch.ad_value(287)), scratch.values[114])));
        }

        if ((((scratch.values[270] != 0.0) && (!(scratch.values[445] != 0.0))) && (!(scratch.values[456] != 0.0))) && (!(scratch.values[457] != 0.0))) {
            scratch.store_ad(289, &AdValue::powf(AdValue::scale(AdValue::sub_from_scalar(scratch.values[8], scratch.ad_value(287)), scratch.values[114]), scratch.values[11]));
        }

        if (((scratch.values[270] != 0.0) && (!(scratch.values[445] != 0.0))) && (!(scratch.values[456] != 0.0))) {
            scratch.store_ad(314, &AdValue::scale(AdValue::div(AdValue::scale(AdValue::sub_from_scalar(scratch.values[8], scratch.ad_value(287)), scratch.values[111]), scratch.ad_value(289)), scratch.values[96]));
        }

        scratch.values[458] = if (((((-scratch.values[126]) / scratch.values[314])) as f64).abs() < 230.25850929940458) { 1.0 } else { 0.0 };

        if ((((scratch.values[270] != 0.0) && (!(scratch.values[445] != 0.0))) && (!(scratch.values[456] != 0.0))) && (scratch.values[458] != 0.0)) {
            scratch.store_ad(289, &AdValue::exp(AdValue::div(AdValue::neg(scratch.ad_value(126)), scratch.ad_value(314))));
        }

        scratch.values[459] = if (((-scratch.values[126]) / scratch.values[314]) < 0.0) { 1.0 } else { 0.0 };

        if (((((scratch.values[270] != 0.0) && (!(scratch.values[445] != 0.0))) && (!(scratch.values[456] != 0.0))) && (!(scratch.values[458] != 0.0))) && (scratch.values[459] != 0.0)) {
            let assign8640_ad_e10149: AdValue = AdValue::offset(AdValue::mul(AdValue::sub_from_scalar((-230.25850929940458), AdValue::div(AdValue::neg(scratch.ad_value(126)), scratch.ad_value(314))), AdValue::offset(AdValue::scale(AdValue::mul(AdValue::sub_from_scalar((-230.25850929940458), AdValue::div(AdValue::neg(scratch.ad_value(126)), scratch.ad_value(314))), AdValue::offset(AdValue::scale(AdValue::sub_from_scalar((-230.25850929940458), AdValue::div(AdValue::neg(scratch.ad_value(126)), scratch.ad_value(314))), 0.3333333333333333), 1.0)), 0.5), 1.0)), 1.0);
            scratch.store_ad(289, &AdValue::div_from_scalar(1e-100, assign8640_ad_e10149));
        }

        if (((((scratch.values[270] != 0.0) && (!(scratch.values[445] != 0.0))) && (!(scratch.values[456] != 0.0))) && (!(scratch.values[458] != 0.0))) && (!(scratch.values[459] != 0.0))) {
            let assign8650_ad_e10197: AdValue = AdValue::scale(AdValue::offset(AdValue::mul(AdValue::offset(AdValue::div(AdValue::neg(scratch.ad_value(126)), scratch.ad_value(314)), (-230.25850929940458)), AdValue::offset(AdValue::scale(AdValue::mul(AdValue::offset(AdValue::div(AdValue::neg(scratch.ad_value(126)), scratch.ad_value(314)), (-230.25850929940458)), AdValue::offset(AdValue::scale(AdValue::offset(AdValue::div(AdValue::neg(scratch.ad_value(126)), scratch.ad_value(314)), (-230.25850929940458)), 0.3333333333333333), 1.0)), 0.5), 1.0)), 1.0), 1e100);
            scratch.store_ad(289, &assign8650_ad_e10197);
        }

        if (((scratch.values[270] != 0.0) && (!(scratch.values[445] != 0.0))) && (!(scratch.values[456] != 0.0))) {
            scratch.store_ad(313, &AdValue::scale(AdValue::mul(AdValue::mul(AdValue::mul(scratch.ad_value(154), scratch.ad_value(314)), scratch.ad_value(314)), scratch.ad_value(289)), scratch.values[31]));
        }

        scratch.values[460] = if (scratch.values[40] > 1000.0) { 1.0 } else { 0.0 };

        if (((scratch.values[270] != 0.0) && (!(scratch.values[445] != 0.0))) && (scratch.values[460] != 0.0)) {
            scratch.values[315] = 1.0;
            scratch.node_derivatives[315] = [0.0; Instance::NODE_COUNT];
            scratch.branch_derivatives[315] = [0.0; Instance::BRANCH_COUNT];
        }

        scratch.values[461] = if (scratch.values[288] > ((-scratch.values[129]) * scratch.values[40])) { 1.0 } else { 0.0 };

        scratch.values[462] = if (scratch.values[43] == 4.0) { 1.0 } else { 0.0 };

        if (((((scratch.values[270] != 0.0) && (!(scratch.values[445] != 0.0))) && (!(scratch.values[460] != 0.0))) && (scratch.values[461] != 0.0)) && (scratch.values[462] != 0.0)) {
            scratch.store_ad(289, &AdValue::mul(AdValue::mul(AdValue::mul(AdValue::scale(scratch.ad_value(288), scratch.values[133]), AdValue::scale(scratch.ad_value(288), scratch.values[133])), AdValue::scale(scratch.ad_value(288), scratch.values[133])), AdValue::scale(scratch.ad_value(288), scratch.values[133])));
        }

        if (((((scratch.values[270] != 0.0) && (!(scratch.values[445] != 0.0))) && (!(scratch.values[460] != 0.0))) && (scratch.values[461] != 0.0)) && (!(scratch.values[462] != 0.0))) {
            scratch.store_ad(289, &AdValue::powf(AdValue::abs(AdValue::scale(scratch.ad_value(288), scratch.values[133])), scratch.values[43]));
        }

        if ((((scratch.values[270] != 0.0) && (!(scratch.values[445] != 0.0))) && (!(scratch.values[460] != 0.0))) && (scratch.values[461] != 0.0)) {
            scratch.store_ad(315, &AdValue::div_from_scalar(1.0, AdValue::sub_from_scalar(1.0, scratch.ad_value(289))));
        }

        if ((((scratch.values[270] != 0.0) && (!(scratch.values[445] != 0.0))) && (!(scratch.values[460] != 0.0))) && (!(scratch.values[461] != 0.0))) {
            scratch.store_ad(315, &AdValue::offset(AdValue::scale(AdValue::offset(scratch.ad_value(288), (scratch.values[129] * scratch.values[40])), scratch.values[136]), scratch.values[130]));
        }

        if ((scratch.values[270] != 0.0) && (!(scratch.values[445] != 0.0))) {
            scratch.store_ad(316, &AdValue::mul(AdValue::add(AdValue::add(AdValue::add(scratch.ad_value(290), scratch.ad_value(291)), scratch.ad_value(298)), scratch.ad_value(313)), scratch.ad_value(315)));
        }

        scratch.values[463] = if (scratch.values[218] == 0.0) { 1.0 } else { 0.0 };

        if ((scratch.values[270] != 0.0) && (scratch.values[463] != 0.0)) {
            scratch.values[317] = 0.0;
            scratch.node_derivatives[317] = [0.0; Instance::NODE_COUNT];
            scratch.branch_derivatives[317] = [0.0; Instance::BRANCH_COUNT];
        }

        scratch.values[464] = if (scratch.values[94] == 0.5) { 1.0 } else { 0.0 };

        if (((scratch.values[270] != 0.0) && (!(scratch.values[463] != 0.0))) && (scratch.values[464] != 0.0)) {
            scratch.store_ad(289, &AdValue::sqrt(AdValue::sub_from_scalar(1.0, AdValue::scale(scratch.ad_value(281), scratch.values[91]))));
        }

    }

    pub(super) fn stamp_transient_block_6(
        &mut self,
        ctx: &GeneratedEvalContext<'_>,
        stamper: &mut GeneratedStamper<'_>,
        scratch: &mut Scratch,
    ) {
        let _ = stamper;
        if (((scratch.values[270] != 0.0) && (!(scratch.values[463] != 0.0))) && (!(scratch.values[464] != 0.0))) {
            scratch.store_ad(289, &AdValue::powf(AdValue::sub_from_scalar(1.0, AdValue::scale(scratch.ad_value(281), scratch.values[91])), scratch.values[94]));
        }

        if ((scratch.values[270] != 0.0) && (!(scratch.values[463] != 0.0))) {
            scratch.store_ad(290, &AdValue::scale(scratch.ad_value(280), scratch.values[73]));
        }

        scratch.values[465] = if ((scratch.values[23] == 0.0) && (scratch.values[26] == 0.0)) { 1.0 } else { 0.0 };

        if (((scratch.values[270] != 0.0) && (!(scratch.values[463] != 0.0))) && (scratch.values[465] != 0.0)) {
            scratch.values[291] = 0.0;
            scratch.node_derivatives[291] = [0.0; Instance::NODE_COUNT];
            scratch.branch_derivatives[291] = [0.0; Instance::BRANCH_COUNT];
        }

        if (((scratch.values[270] != 0.0) && (!(scratch.values[463] != 0.0))) && (!(scratch.values[465] != 0.0))) {
            scratch.store_ad(292, &AdValue::sub_from_scalar(scratch.values[79], scratch.ad_value(286)));
        }

        if (((scratch.values[270] != 0.0) && (!(scratch.values[463] != 0.0))) && (!(scratch.values[465] != 0.0))) {
            scratch.store_ad(293, &AdValue::sub_from_scalar(1.0, AdValue::sqrt(AdValue::sub_from_scalar(1.0, AdValue::div(scratch.ad_value(284), scratch.ad_value(292))))));
        }

        scratch.values[466] = if (scratch.values[12] == 0.5) { 1.0 } else { 0.0 };

        if ((((scratch.values[270] != 0.0) && (!(scratch.values[463] != 0.0))) && (!(scratch.values[465] != 0.0))) && (scratch.values[466] != 0.0)) {
            scratch.values[294] = 0.0;
            scratch.node_derivatives[294] = [0.0; Instance::NODE_COUNT];
            scratch.branch_derivatives[294] = [0.0; Instance::BRANCH_COUNT];
        }

        if ((((scratch.values[270] != 0.0) && (!(scratch.values[463] != 0.0))) && (!(scratch.values[465] != 0.0))) && (!(scratch.values[466] != 0.0))) {
            scratch.store_ad(294, &AdValue::scale(AdValue::add(AdValue::div(AdValue::mul(AdValue::square(scratch.ad_value(293)), AdValue::ln(scratch.ad_value(293))), AdValue::sub_from_scalar(1.0, scratch.ad_value(293))), scratch.ad_value(293)), (1.0 - (2.0 * scratch.values[12]))));
        }

        if (((scratch.values[270] != 0.0) && (!(scratch.values[463] != 0.0))) && (!(scratch.values[465] != 0.0))) {
            scratch.store_ad(295, &AdValue::add(scratch.ad_value(293), scratch.ad_value(294)));
        }

        scratch.values[467] = if (scratch.values[12] == 0.5) { 1.0 } else { 0.0 };

        if ((((scratch.values[270] != 0.0) && (!(scratch.values[463] != 0.0))) && (!(scratch.values[465] != 0.0))) && (scratch.values[467] != 0.0)) {
            scratch.store_ad(289, &AdValue::sqrt(AdValue::scale(scratch.ad_value(292), scratch.values[115])));
        }

        if ((((scratch.values[270] != 0.0) && (!(scratch.values[463] != 0.0))) && (!(scratch.values[465] != 0.0))) && (!(scratch.values[467] != 0.0))) {
            scratch.store_ad(289, &AdValue::powf(AdValue::scale(scratch.ad_value(292), scratch.values[115]), scratch.values[12]));
        }

        if (((scratch.values[270] != 0.0) && (!(scratch.values[463] != 0.0))) && (!(scratch.values[465] != 0.0))) {
            scratch.store_ad(296, &AdValue::scale(scratch.ad_value(289), scratch.values[109]));
        }

        if (((scratch.values[270] != 0.0) && (!(scratch.values[463] != 0.0))) && (!(scratch.values[465] != 0.0))) {
            scratch.store_ad(297, &AdValue::scale(AdValue::mul(AdValue::offset(scratch.ad_value(283), (-1.0)), scratch.ad_value(296)), scratch.values[70]));
        }

        if (((scratch.values[270] != 0.0) && (!(scratch.values[463] != 0.0))) && (!(scratch.values[465] != 0.0))) {
            scratch.store_ad(291, &AdValue::scale(AdValue::mul(scratch.ad_value(297), scratch.ad_value(295)), scratch.values[23]));
        }

        scratch.values[468] = if (scratch.values[26] == 0.0) { 1.0 } else { 0.0 };

        if (((scratch.values[270] != 0.0) && (!(scratch.values[463] != 0.0))) && (scratch.values[468] != 0.0)) {
            scratch.values[298] = 0.0;
            scratch.node_derivatives[298] = [0.0; Instance::NODE_COUNT];
            scratch.branch_derivatives[298] = [0.0; Instance::BRANCH_COUNT];
        }

        if (((scratch.values[270] != 0.0) && (!(scratch.values[463] != 0.0))) && (!(scratch.values[468] != 0.0))) {
            scratch.store_ad(299, &AdValue::scale(AdValue::div(AdValue::scale(scratch.ad_value(296), scratch.values[94]), scratch.ad_value(292)), scratch.values[124]));
        }

        if (((scratch.values[270] != 0.0) && (!(scratch.values[463] != 0.0))) && (!(scratch.values[468] != 0.0))) {
            scratch.store_ad(300, &AdValue::div_from_scalar((0.666666666666667 * scratch.values[121]), scratch.ad_value(299)));
        }

        if (((scratch.values[270] != 0.0) && (!(scratch.values[463] != 0.0))) && (!(scratch.values[468] != 0.0))) {
            scratch.store_ad(301, &AdValue::square(scratch.ad_value(300)));
        }

        if (((scratch.values[270] != 0.0) && (!(scratch.values[463] != 0.0))) && (!(scratch.values[468] != 0.0))) {
            scratch.store_ad(302, &AdValue::sqrt(AdValue::div(AdValue::square(scratch.ad_value(301)), AdValue::offset(AdValue::square(scratch.ad_value(301)), 1.0))));
        }

        if (((scratch.values[270] != 0.0) && (!(scratch.values[463] != 0.0))) && (!(scratch.values[468] != 0.0))) {
            scratch.store_ad(303, &AdValue::sqrt(AdValue::abs(scratch.ad_value(302))));
        }

        if (((scratch.values[270] != 0.0) && (!(scratch.values[463] != 0.0))) && (!(scratch.values[468] != 0.0))) {
            scratch.store_ad(304, &AdValue::mul(scratch.ad_value(302), scratch.ad_value(303)));
        }

        scratch.values[469] = if (((-scratch.values[12]) * scratch.values[97]) == (-1.0)) { 1.0 } else { 0.0 };

        if ((((scratch.values[270] != 0.0) && (!(scratch.values[463] != 0.0))) && (!(scratch.values[468] != 0.0))) && (scratch.values[469] != 0.0)) {
            scratch.store_ad(305, &AdValue::div_from_scalar(1.0, AdValue::offset(AdValue::mul(scratch.ad_value(299), scratch.ad_value(304)), 1.0)));
        }

        if ((((scratch.values[270] != 0.0) && (!(scratch.values[463] != 0.0))) && (!(scratch.values[468] != 0.0))) && (!(scratch.values[469] != 0.0))) {
            scratch.store_ad(305, &AdValue::powf(AdValue::offset(AdValue::mul(scratch.ad_value(299), scratch.ad_value(304)), 1.0), ((-scratch.values[12]) * scratch.values[97])));
        }

        if (((scratch.values[270] != 0.0) && (!(scratch.values[463] != 0.0))) && (!(scratch.values[468] != 0.0))) {
            scratch.store_ad(306, &AdValue::div(AdValue::mul(scratch.ad_value(295), scratch.ad_value(305)), AdValue::add(scratch.ad_value(295), scratch.ad_value(305))));
        }

        if (((scratch.values[270] != 0.0) && (!(scratch.values[463] != 0.0))) && (!(scratch.values[468] != 0.0))) {
            scratch.store_ad(307, &AdValue::sqrt(AdValue::scale(AdValue::div(scratch.ad_value(299), scratch.ad_value(303)), 0.375)));
        }

        if (((scratch.values[270] != 0.0) && (!(scratch.values[463] != 0.0))) && (!(scratch.values[468] != 0.0))) {
            scratch.store_ad(308, &AdValue::sub(AdValue::scale(AdValue::mul(scratch.ad_value(300), scratch.ad_value(303)), 2.0), scratch.ad_value(302)));
        }

        if (((scratch.values[270] != 0.0) && (!(scratch.values[463] != 0.0))) && (!(scratch.values[468] != 0.0))) {
            scratch.store_ad(309, &AdValue::add(AdValue::sub(AdValue::mul(AdValue::scale(scratch.ad_value(300), scratch.values[121]), scratch.ad_value(303)), AdValue::scale(scratch.ad_value(302), scratch.values[121])), AdValue::scale(AdValue::mul(scratch.ad_value(299), scratch.ad_value(304)), 0.5)));
        }

        if (((scratch.values[270] != 0.0) && (!(scratch.values[463] != 0.0))) && (!(scratch.values[468] != 0.0))) {
            scratch.store_ad(310, &AdValue::mul(AdValue::offset(scratch.ad_value(308), (-1.0)), scratch.ad_value(307)));
        }

        if (((scratch.values[270] != 0.0) && (!(scratch.values[463] != 0.0))) && (!(scratch.values[468] != 0.0))) {
            scratch.store_ad(271, &AdValue::square(scratch.ad_value(310)));
        }

        scratch.values[470] = if (scratch.values[310] > 0.0) { 1.0 } else { 0.0 };

        if ((((scratch.values[270] != 0.0) && (!(scratch.values[463] != 0.0))) && (!(scratch.values[468] != 0.0))) && (scratch.values[470] != 0.0)) {
            scratch.store_ad(272, &AdValue::div_from_scalar(1.0, AdValue::offset(AdValue::scale(scratch.ad_value(310), scratch.values[57]), 1.0)));
        }

        if ((((scratch.values[270] != 0.0) && (!(scratch.values[463] != 0.0))) && (!(scratch.values[468] != 0.0))) && (!(scratch.values[470] != 0.0))) {
            scratch.store_ad(272, &AdValue::div_from_scalar(1.0, AdValue::sub_from_scalar(1.0, AdValue::scale(scratch.ad_value(310), scratch.values[57]))));
        }

        scratch.values[471] = if (((-scratch.values[271]) + scratch.values[309]) > (-230.25850929940458)) { 1.0 } else { 0.0 };

        if ((((scratch.values[270] != 0.0) && (!(scratch.values[463] != 0.0))) && (!(scratch.values[468] != 0.0))) && (scratch.values[471] != 0.0)) {
            scratch.store_ad(289, &AdValue::exp(AdValue::sub(scratch.ad_value(309), scratch.ad_value(271))));
        }

        if ((((scratch.values[270] != 0.0) && (!(scratch.values[463] != 0.0))) && (!(scratch.values[468] != 0.0))) && (!(scratch.values[471] != 0.0))) {
            scratch.store_ad(289, &AdValue::div_from_scalar(1e-100, AdValue::offset(AdValue::mul(AdValue::sub_from_scalar((-230.25850929940458), AdValue::sub(scratch.ad_value(309), scratch.ad_value(271))), AdValue::offset(AdValue::scale(AdValue::mul(AdValue::sub_from_scalar((-230.25850929940458), AdValue::sub(scratch.ad_value(309), scratch.ad_value(271))), AdValue::offset(AdValue::scale(AdValue::sub_from_scalar((-230.25850929940458), AdValue::sub(scratch.ad_value(309), scratch.ad_value(271))), 0.3333333333333333), 1.0)), 0.5), 1.0)), 1.0)));
        }

        if (((scratch.values[270] != 0.0) && (!(scratch.values[463] != 0.0))) && (!(scratch.values[468] != 0.0))) {
            scratch.store_ad(273, &AdValue::mul(AdValue::add(AdValue::add(AdValue::scale(scratch.ad_value(272), 0.29214664), AdValue::scale(AdValue::square(scratch.ad_value(272)), scratch.values[58])), AdValue::scale(AdValue::mul(AdValue::square(scratch.ad_value(272)), scratch.ad_value(272)), scratch.values[59])), scratch.ad_value(289)));
        }

        scratch.values[472] = if (scratch.values[310] > 0.0) { 1.0 } else { 0.0 };

        if ((((scratch.values[270] != 0.0) && (!(scratch.values[463] != 0.0))) && (!(scratch.values[468] != 0.0))) && (scratch.values[472] != 0.0)) {
            scratch.values[311] = scratch.values[273];
            scratch.node_derivatives[311] = scratch.node_derivatives[273];
            scratch.branch_derivatives[311] = scratch.branch_derivatives[273];
        }

        scratch.values[473] = if (scratch.values[309] > (-230.25850929940458)) { 1.0 } else { 0.0 };

        if (((((scratch.values[270] != 0.0) && (!(scratch.values[463] != 0.0))) && (!(scratch.values[468] != 0.0))) && (!(scratch.values[472] != 0.0))) && (scratch.values[473] != 0.0)) {
            scratch.store_ad(289, &AdValue::exp(scratch.ad_value(309)));
        }

        if (((((scratch.values[270] != 0.0) && (!(scratch.values[463] != 0.0))) && (!(scratch.values[468] != 0.0))) && (!(scratch.values[472] != 0.0))) && (!(scratch.values[473] != 0.0))) {
            scratch.store_ad(289, &AdValue::div_from_scalar(1e-100, AdValue::offset(AdValue::mul(AdValue::sub_from_scalar((-230.25850929940458), scratch.ad_value(309)), AdValue::offset(AdValue::scale(AdValue::mul(AdValue::sub_from_scalar((-230.25850929940458), scratch.ad_value(309)), AdValue::offset(AdValue::scale(AdValue::sub_from_scalar((-230.25850929940458), scratch.ad_value(309)), 0.3333333333333333), 1.0)), 0.5), 1.0)), 1.0)));
        }

        if ((((scratch.values[270] != 0.0) && (!(scratch.values[463] != 0.0))) && (!(scratch.values[468] != 0.0))) && (!(scratch.values[472] != 0.0))) {
            scratch.store_ad(311, &AdValue::sub(AdValue::scale(scratch.ad_value(289), 2.0), scratch.ad_value(273)));
        }

        if (((scratch.values[270] != 0.0) && (!(scratch.values[463] != 0.0))) && (!(scratch.values[468] != 0.0))) {
            scratch.store_ad(312, &AdValue::scale(AdValue::div(AdValue::scale(scratch.ad_value(311), scratch.values[121]), scratch.ad_value(307)), (1.772453850905516 * 0.5)));
        }

        if (((scratch.values[270] != 0.0) && (!(scratch.values[463] != 0.0))) && (!(scratch.values[468] != 0.0))) {
            scratch.store_ad(298, &AdValue::scale(AdValue::mul(AdValue::mul(scratch.ad_value(297), scratch.ad_value(312)), scratch.ad_value(306)), scratch.values[26]));
        }

        scratch.values[474] = if (scratch.values[32] == 0.0) { 1.0 } else { 0.0 };

        if (((scratch.values[270] != 0.0) && (!(scratch.values[463] != 0.0))) && (scratch.values[474] != 0.0)) {
            scratch.values[313] = 0.0;
            scratch.node_derivatives[313] = [0.0; Instance::NODE_COUNT];
            scratch.branch_derivatives[313] = [0.0; Instance::BRANCH_COUNT];
        }

        scratch.values[475] = if (scratch.values[12] == 0.5) { 1.0 } else { 0.0 };

        if ((((scratch.values[270] != 0.0) && (!(scratch.values[463] != 0.0))) && (!(scratch.values[474] != 0.0))) && (scratch.values[475] != 0.0)) {
            scratch.store_ad(289, &AdValue::sqrt(AdValue::scale(AdValue::sub_from_scalar(scratch.values[9], scratch.ad_value(287)), scratch.values[115])));
        }

        if ((((scratch.values[270] != 0.0) && (!(scratch.values[463] != 0.0))) && (!(scratch.values[474] != 0.0))) && (!(scratch.values[475] != 0.0))) {
            scratch.store_ad(289, &AdValue::powf(AdValue::scale(AdValue::sub_from_scalar(scratch.values[9], scratch.ad_value(287)), scratch.values[115]), scratch.values[12]));
        }

        if (((scratch.values[270] != 0.0) && (!(scratch.values[463] != 0.0))) && (!(scratch.values[474] != 0.0))) {
            scratch.store_ad(314, &AdValue::scale(AdValue::div(AdValue::scale(AdValue::sub_from_scalar(scratch.values[9], scratch.ad_value(287)), scratch.values[112]), scratch.ad_value(289)), scratch.values[97]));
        }

        scratch.values[476] = if (((((-scratch.values[127]) / scratch.values[314])) as f64).abs() < 230.25850929940458) { 1.0 } else { 0.0 };

        if ((((scratch.values[270] != 0.0) && (!(scratch.values[463] != 0.0))) && (!(scratch.values[474] != 0.0))) && (scratch.values[476] != 0.0)) {
            scratch.store_ad(289, &AdValue::exp(AdValue::div(AdValue::neg(scratch.ad_value(127)), scratch.ad_value(314))));
        }

        scratch.values[477] = if (((-scratch.values[127]) / scratch.values[314]) < 0.0) { 1.0 } else { 0.0 };

        if (((((scratch.values[270] != 0.0) && (!(scratch.values[463] != 0.0))) && (!(scratch.values[474] != 0.0))) && (!(scratch.values[476] != 0.0))) && (scratch.values[477] != 0.0)) {
            let assign9390_ad_e11241: AdValue = AdValue::offset(AdValue::mul(AdValue::sub_from_scalar((-230.25850929940458), AdValue::div(AdValue::neg(scratch.ad_value(127)), scratch.ad_value(314))), AdValue::offset(AdValue::scale(AdValue::mul(AdValue::sub_from_scalar((-230.25850929940458), AdValue::div(AdValue::neg(scratch.ad_value(127)), scratch.ad_value(314))), AdValue::offset(AdValue::scale(AdValue::sub_from_scalar((-230.25850929940458), AdValue::div(AdValue::neg(scratch.ad_value(127)), scratch.ad_value(314))), 0.3333333333333333), 1.0)), 0.5), 1.0)), 1.0);
            scratch.store_ad(289, &AdValue::div_from_scalar(1e-100, assign9390_ad_e11241));
        }

        if (((((scratch.values[270] != 0.0) && (!(scratch.values[463] != 0.0))) && (!(scratch.values[474] != 0.0))) && (!(scratch.values[476] != 0.0))) && (!(scratch.values[477] != 0.0))) {
            let assign9400_ad_e11289: AdValue = AdValue::scale(AdValue::offset(AdValue::mul(AdValue::offset(AdValue::div(AdValue::neg(scratch.ad_value(127)), scratch.ad_value(314)), (-230.25850929940458)), AdValue::offset(AdValue::scale(AdValue::mul(AdValue::offset(AdValue::div(AdValue::neg(scratch.ad_value(127)), scratch.ad_value(314)), (-230.25850929940458)), AdValue::offset(AdValue::scale(AdValue::offset(AdValue::div(AdValue::neg(scratch.ad_value(127)), scratch.ad_value(314)), (-230.25850929940458)), 0.3333333333333333), 1.0)), 0.5), 1.0)), 1.0), 1e100);
            scratch.store_ad(289, &assign9400_ad_e11289);
        }

        if (((scratch.values[270] != 0.0) && (!(scratch.values[463] != 0.0))) && (!(scratch.values[474] != 0.0))) {
            scratch.store_ad(313, &AdValue::scale(AdValue::mul(AdValue::mul(AdValue::mul(scratch.ad_value(154), scratch.ad_value(314)), scratch.ad_value(314)), scratch.ad_value(289)), scratch.values[32]));
        }

        scratch.values[478] = if (scratch.values[41] > 1000.0) { 1.0 } else { 0.0 };

        if (((scratch.values[270] != 0.0) && (!(scratch.values[463] != 0.0))) && (scratch.values[478] != 0.0)) {
            scratch.values[315] = 1.0;
            scratch.node_derivatives[315] = [0.0; Instance::NODE_COUNT];
            scratch.branch_derivatives[315] = [0.0; Instance::BRANCH_COUNT];
        }

        scratch.values[479] = if (scratch.values[288] > ((-scratch.values[129]) * scratch.values[41])) { 1.0 } else { 0.0 };

        scratch.values[480] = if (scratch.values[44] == 4.0) { 1.0 } else { 0.0 };

        if (((((scratch.values[270] != 0.0) && (!(scratch.values[463] != 0.0))) && (!(scratch.values[478] != 0.0))) && (scratch.values[479] != 0.0)) && (scratch.values[480] != 0.0)) {
            scratch.store_ad(289, &AdValue::mul(AdValue::mul(AdValue::mul(AdValue::scale(scratch.ad_value(288), scratch.values[134]), AdValue::scale(scratch.ad_value(288), scratch.values[134])), AdValue::scale(scratch.ad_value(288), scratch.values[134])), AdValue::scale(scratch.ad_value(288), scratch.values[134])));
        }

        if (((((scratch.values[270] != 0.0) && (!(scratch.values[463] != 0.0))) && (!(scratch.values[478] != 0.0))) && (scratch.values[479] != 0.0)) && (!(scratch.values[480] != 0.0))) {
            scratch.store_ad(289, &AdValue::powf(AdValue::abs(AdValue::scale(scratch.ad_value(288), scratch.values[134])), scratch.values[44]));
        }

        if ((((scratch.values[270] != 0.0) && (!(scratch.values[463] != 0.0))) && (!(scratch.values[478] != 0.0))) && (scratch.values[479] != 0.0)) {
            scratch.store_ad(315, &AdValue::div_from_scalar(1.0, AdValue::sub_from_scalar(1.0, scratch.ad_value(289))));
        }

        if ((((scratch.values[270] != 0.0) && (!(scratch.values[463] != 0.0))) && (!(scratch.values[478] != 0.0))) && (!(scratch.values[479] != 0.0))) {
            scratch.store_ad(315, &AdValue::offset(AdValue::scale(AdValue::offset(scratch.ad_value(288), (scratch.values[129] * scratch.values[41])), scratch.values[137]), scratch.values[131]));
        }

        if ((scratch.values[270] != 0.0) && (!(scratch.values[463] != 0.0))) {
            scratch.store_ad(317, &AdValue::mul(AdValue::add(AdValue::add(AdValue::add(scratch.ad_value(290), scratch.ad_value(291)), scratch.ad_value(298)), scratch.ad_value(313)), scratch.ad_value(315)));
        }

        scratch.values[481] = if (scratch.values[219] == 0.0) { 1.0 } else { 0.0 };

        if ((scratch.values[270] != 0.0) && (scratch.values[481] != 0.0)) {
            scratch.values[318] = 0.0;
            scratch.node_derivatives[318] = [0.0; Instance::NODE_COUNT];
            scratch.branch_derivatives[318] = [0.0; Instance::BRANCH_COUNT];
        }

        scratch.values[482] = if (scratch.values[95] == 0.5) { 1.0 } else { 0.0 };

        if (((scratch.values[270] != 0.0) && (!(scratch.values[481] != 0.0))) && (scratch.values[482] != 0.0)) {
            scratch.store_ad(289, &AdValue::sqrt(AdValue::sub_from_scalar(1.0, AdValue::scale(scratch.ad_value(281), scratch.values[92]))));
        }

        if (((scratch.values[270] != 0.0) && (!(scratch.values[481] != 0.0))) && (!(scratch.values[482] != 0.0))) {
            scratch.store_ad(289, &AdValue::powf(AdValue::sub_from_scalar(1.0, AdValue::scale(scratch.ad_value(281), scratch.values[92])), scratch.values[95]));
        }

        if ((scratch.values[270] != 0.0) && (!(scratch.values[481] != 0.0))) {
            scratch.store_ad(290, &AdValue::scale(scratch.ad_value(280), scratch.values[74]));
        }

        scratch.values[483] = if ((scratch.values[24] == 0.0) && (scratch.values[27] == 0.0)) { 1.0 } else { 0.0 };

        if (((scratch.values[270] != 0.0) && (!(scratch.values[481] != 0.0))) && (scratch.values[483] != 0.0)) {
            scratch.values[291] = 0.0;
            scratch.node_derivatives[291] = [0.0; Instance::NODE_COUNT];
            scratch.branch_derivatives[291] = [0.0; Instance::BRANCH_COUNT];
        }

        if (((scratch.values[270] != 0.0) && (!(scratch.values[481] != 0.0))) && (!(scratch.values[483] != 0.0))) {
            scratch.store_ad(292, &AdValue::sub_from_scalar(scratch.values[80], scratch.ad_value(286)));
        }

        if (((scratch.values[270] != 0.0) && (!(scratch.values[481] != 0.0))) && (!(scratch.values[483] != 0.0))) {
            scratch.store_ad(293, &AdValue::sub_from_scalar(1.0, AdValue::sqrt(AdValue::sub_from_scalar(1.0, AdValue::div(scratch.ad_value(284), scratch.ad_value(292))))));
        }

        scratch.values[484] = if (scratch.values[13] == 0.5) { 1.0 } else { 0.0 };

        if ((((scratch.values[270] != 0.0) && (!(scratch.values[481] != 0.0))) && (!(scratch.values[483] != 0.0))) && (scratch.values[484] != 0.0)) {
            scratch.values[294] = 0.0;
            scratch.node_derivatives[294] = [0.0; Instance::NODE_COUNT];
            scratch.branch_derivatives[294] = [0.0; Instance::BRANCH_COUNT];
        }

        if ((((scratch.values[270] != 0.0) && (!(scratch.values[481] != 0.0))) && (!(scratch.values[483] != 0.0))) && (!(scratch.values[484] != 0.0))) {
            scratch.store_ad(294, &AdValue::scale(AdValue::add(AdValue::div(AdValue::mul(AdValue::square(scratch.ad_value(293)), AdValue::ln(scratch.ad_value(293))), AdValue::sub_from_scalar(1.0, scratch.ad_value(293))), scratch.ad_value(293)), (1.0 - (2.0 * scratch.values[13]))));
        }

        if (((scratch.values[270] != 0.0) && (!(scratch.values[481] != 0.0))) && (!(scratch.values[483] != 0.0))) {
            scratch.store_ad(295, &AdValue::add(scratch.ad_value(293), scratch.ad_value(294)));
        }

        scratch.values[485] = if (scratch.values[13] == 0.5) { 1.0 } else { 0.0 };

        if ((((scratch.values[270] != 0.0) && (!(scratch.values[481] != 0.0))) && (!(scratch.values[483] != 0.0))) && (scratch.values[485] != 0.0)) {
            scratch.store_ad(289, &AdValue::sqrt(AdValue::scale(scratch.ad_value(292), scratch.values[116])));
        }

        if ((((scratch.values[270] != 0.0) && (!(scratch.values[481] != 0.0))) && (!(scratch.values[483] != 0.0))) && (!(scratch.values[485] != 0.0))) {
            scratch.store_ad(289, &AdValue::powf(AdValue::scale(scratch.ad_value(292), scratch.values[116]), scratch.values[13]));
        }

        if (((scratch.values[270] != 0.0) && (!(scratch.values[481] != 0.0))) && (!(scratch.values[483] != 0.0))) {
            scratch.store_ad(296, &AdValue::scale(scratch.ad_value(289), scratch.values[110]));
        }

        if (((scratch.values[270] != 0.0) && (!(scratch.values[481] != 0.0))) && (!(scratch.values[483] != 0.0))) {
            scratch.store_ad(297, &AdValue::scale(AdValue::mul(AdValue::offset(scratch.ad_value(283), (-1.0)), scratch.ad_value(296)), scratch.values[71]));
        }

        if (((scratch.values[270] != 0.0) && (!(scratch.values[481] != 0.0))) && (!(scratch.values[483] != 0.0))) {
            scratch.store_ad(291, &AdValue::scale(AdValue::mul(scratch.ad_value(297), scratch.ad_value(295)), scratch.values[24]));
        }

        scratch.values[486] = if (scratch.values[27] == 0.0) { 1.0 } else { 0.0 };

        if (((scratch.values[270] != 0.0) && (!(scratch.values[481] != 0.0))) && (scratch.values[486] != 0.0)) {
            scratch.values[298] = 0.0;
            scratch.node_derivatives[298] = [0.0; Instance::NODE_COUNT];
            scratch.branch_derivatives[298] = [0.0; Instance::BRANCH_COUNT];
        }

        if (((scratch.values[270] != 0.0) && (!(scratch.values[481] != 0.0))) && (!(scratch.values[486] != 0.0))) {
            scratch.store_ad(299, &AdValue::scale(AdValue::div(AdValue::scale(scratch.ad_value(296), scratch.values[95]), scratch.ad_value(292)), scratch.values[125]));
        }

        if (((scratch.values[270] != 0.0) && (!(scratch.values[481] != 0.0))) && (!(scratch.values[486] != 0.0))) {
            scratch.store_ad(300, &AdValue::div_from_scalar((0.666666666666667 * scratch.values[122]), scratch.ad_value(299)));
        }

        if (((scratch.values[270] != 0.0) && (!(scratch.values[481] != 0.0))) && (!(scratch.values[486] != 0.0))) {
            scratch.store_ad(301, &AdValue::square(scratch.ad_value(300)));
        }

        if (((scratch.values[270] != 0.0) && (!(scratch.values[481] != 0.0))) && (!(scratch.values[486] != 0.0))) {
            scratch.store_ad(302, &AdValue::sqrt(AdValue::div(AdValue::square(scratch.ad_value(301)), AdValue::offset(AdValue::square(scratch.ad_value(301)), 1.0))));
        }

        if (((scratch.values[270] != 0.0) && (!(scratch.values[481] != 0.0))) && (!(scratch.values[486] != 0.0))) {
            scratch.store_ad(303, &AdValue::sqrt(AdValue::abs(scratch.ad_value(302))));
        }

        if (((scratch.values[270] != 0.0) && (!(scratch.values[481] != 0.0))) && (!(scratch.values[486] != 0.0))) {
            scratch.store_ad(304, &AdValue::mul(scratch.ad_value(302), scratch.ad_value(303)));
        }

        scratch.values[487] = if (((-scratch.values[13]) * scratch.values[98]) == (-1.0)) { 1.0 } else { 0.0 };

        if ((((scratch.values[270] != 0.0) && (!(scratch.values[481] != 0.0))) && (!(scratch.values[486] != 0.0))) && (scratch.values[487] != 0.0)) {
            scratch.store_ad(305, &AdValue::div_from_scalar(1.0, AdValue::offset(AdValue::mul(scratch.ad_value(299), scratch.ad_value(304)), 1.0)));
        }

        if ((((scratch.values[270] != 0.0) && (!(scratch.values[481] != 0.0))) && (!(scratch.values[486] != 0.0))) && (!(scratch.values[487] != 0.0))) {
            scratch.store_ad(305, &AdValue::powf(AdValue::offset(AdValue::mul(scratch.ad_value(299), scratch.ad_value(304)), 1.0), ((-scratch.values[13]) * scratch.values[98])));
        }

        if (((scratch.values[270] != 0.0) && (!(scratch.values[481] != 0.0))) && (!(scratch.values[486] != 0.0))) {
            scratch.store_ad(306, &AdValue::div(AdValue::mul(scratch.ad_value(295), scratch.ad_value(305)), AdValue::add(scratch.ad_value(295), scratch.ad_value(305))));
        }

        if (((scratch.values[270] != 0.0) && (!(scratch.values[481] != 0.0))) && (!(scratch.values[486] != 0.0))) {
            scratch.store_ad(307, &AdValue::sqrt(AdValue::scale(AdValue::div(scratch.ad_value(299), scratch.ad_value(303)), 0.375)));
        }

        if (((scratch.values[270] != 0.0) && (!(scratch.values[481] != 0.0))) && (!(scratch.values[486] != 0.0))) {
            scratch.store_ad(308, &AdValue::sub(AdValue::scale(AdValue::mul(scratch.ad_value(300), scratch.ad_value(303)), 2.0), scratch.ad_value(302)));
        }

        if (((scratch.values[270] != 0.0) && (!(scratch.values[481] != 0.0))) && (!(scratch.values[486] != 0.0))) {
            scratch.store_ad(309, &AdValue::add(AdValue::sub(AdValue::mul(AdValue::scale(scratch.ad_value(300), scratch.values[122]), scratch.ad_value(303)), AdValue::scale(scratch.ad_value(302), scratch.values[122])), AdValue::scale(AdValue::mul(scratch.ad_value(299), scratch.ad_value(304)), 0.5)));
        }

        if (((scratch.values[270] != 0.0) && (!(scratch.values[481] != 0.0))) && (!(scratch.values[486] != 0.0))) {
            scratch.store_ad(310, &AdValue::mul(AdValue::offset(scratch.ad_value(308), (-1.0)), scratch.ad_value(307)));
        }

        if (((scratch.values[270] != 0.0) && (!(scratch.values[481] != 0.0))) && (!(scratch.values[486] != 0.0))) {
            scratch.store_ad(271, &AdValue::square(scratch.ad_value(310)));
        }

        scratch.values[488] = if (scratch.values[310] > 0.0) { 1.0 } else { 0.0 };

        if ((((scratch.values[270] != 0.0) && (!(scratch.values[481] != 0.0))) && (!(scratch.values[486] != 0.0))) && (scratch.values[488] != 0.0)) {
            scratch.store_ad(272, &AdValue::div_from_scalar(1.0, AdValue::offset(AdValue::scale(scratch.ad_value(310), scratch.values[57]), 1.0)));
        }

        if ((((scratch.values[270] != 0.0) && (!(scratch.values[481] != 0.0))) && (!(scratch.values[486] != 0.0))) && (!(scratch.values[488] != 0.0))) {
            scratch.store_ad(272, &AdValue::div_from_scalar(1.0, AdValue::sub_from_scalar(1.0, AdValue::scale(scratch.ad_value(310), scratch.values[57]))));
        }

        scratch.values[489] = if (((-scratch.values[271]) + scratch.values[309]) > (-230.25850929940458)) { 1.0 } else { 0.0 };

        if ((((scratch.values[270] != 0.0) && (!(scratch.values[481] != 0.0))) && (!(scratch.values[486] != 0.0))) && (scratch.values[489] != 0.0)) {
            scratch.store_ad(289, &AdValue::exp(AdValue::sub(scratch.ad_value(309), scratch.ad_value(271))));
        }

        if ((((scratch.values[270] != 0.0) && (!(scratch.values[481] != 0.0))) && (!(scratch.values[486] != 0.0))) && (!(scratch.values[489] != 0.0))) {
            scratch.store_ad(289, &AdValue::div_from_scalar(1e-100, AdValue::offset(AdValue::mul(AdValue::sub_from_scalar((-230.25850929940458), AdValue::sub(scratch.ad_value(309), scratch.ad_value(271))), AdValue::offset(AdValue::scale(AdValue::mul(AdValue::sub_from_scalar((-230.25850929940458), AdValue::sub(scratch.ad_value(309), scratch.ad_value(271))), AdValue::offset(AdValue::scale(AdValue::sub_from_scalar((-230.25850929940458), AdValue::sub(scratch.ad_value(309), scratch.ad_value(271))), 0.3333333333333333), 1.0)), 0.5), 1.0)), 1.0)));
        }

        if (((scratch.values[270] != 0.0) && (!(scratch.values[481] != 0.0))) && (!(scratch.values[486] != 0.0))) {
            scratch.store_ad(273, &AdValue::mul(AdValue::add(AdValue::add(AdValue::scale(scratch.ad_value(272), 0.29214664), AdValue::scale(AdValue::square(scratch.ad_value(272)), scratch.values[58])), AdValue::scale(AdValue::mul(AdValue::square(scratch.ad_value(272)), scratch.ad_value(272)), scratch.values[59])), scratch.ad_value(289)));
        }

        scratch.values[490] = if (scratch.values[310] > 0.0) { 1.0 } else { 0.0 };

        if ((((scratch.values[270] != 0.0) && (!(scratch.values[481] != 0.0))) && (!(scratch.values[486] != 0.0))) && (scratch.values[490] != 0.0)) {
            scratch.values[311] = scratch.values[273];
            scratch.node_derivatives[311] = scratch.node_derivatives[273];
            scratch.branch_derivatives[311] = scratch.branch_derivatives[273];
        }

        scratch.values[491] = if (scratch.values[309] > (-230.25850929940458)) { 1.0 } else { 0.0 };

        if (((((scratch.values[270] != 0.0) && (!(scratch.values[481] != 0.0))) && (!(scratch.values[486] != 0.0))) && (!(scratch.values[490] != 0.0))) && (scratch.values[491] != 0.0)) {
            scratch.store_ad(289, &AdValue::exp(scratch.ad_value(309)));
        }

        if (((((scratch.values[270] != 0.0) && (!(scratch.values[481] != 0.0))) && (!(scratch.values[486] != 0.0))) && (!(scratch.values[490] != 0.0))) && (!(scratch.values[491] != 0.0))) {
            scratch.store_ad(289, &AdValue::div_from_scalar(1e-100, AdValue::offset(AdValue::mul(AdValue::sub_from_scalar((-230.25850929940458), scratch.ad_value(309)), AdValue::offset(AdValue::scale(AdValue::mul(AdValue::sub_from_scalar((-230.25850929940458), scratch.ad_value(309)), AdValue::offset(AdValue::scale(AdValue::sub_from_scalar((-230.25850929940458), scratch.ad_value(309)), 0.3333333333333333), 1.0)), 0.5), 1.0)), 1.0)));
        }

        if ((((scratch.values[270] != 0.0) && (!(scratch.values[481] != 0.0))) && (!(scratch.values[486] != 0.0))) && (!(scratch.values[490] != 0.0))) {
            scratch.store_ad(311, &AdValue::sub(AdValue::scale(scratch.ad_value(289), 2.0), scratch.ad_value(273)));
        }

        if (((scratch.values[270] != 0.0) && (!(scratch.values[481] != 0.0))) && (!(scratch.values[486] != 0.0))) {
            scratch.store_ad(312, &AdValue::scale(AdValue::div(AdValue::scale(scratch.ad_value(311), scratch.values[122]), scratch.ad_value(307)), (1.772453850905516 * 0.5)));
        }

        if (((scratch.values[270] != 0.0) && (!(scratch.values[481] != 0.0))) && (!(scratch.values[486] != 0.0))) {
            scratch.store_ad(298, &AdValue::scale(AdValue::mul(AdValue::mul(scratch.ad_value(297), scratch.ad_value(312)), scratch.ad_value(306)), scratch.values[27]));
        }

        scratch.values[492] = if (scratch.values[33] == 0.0) { 1.0 } else { 0.0 };

        if (((scratch.values[270] != 0.0) && (!(scratch.values[481] != 0.0))) && (scratch.values[492] != 0.0)) {
            scratch.values[313] = 0.0;
            scratch.node_derivatives[313] = [0.0; Instance::NODE_COUNT];
            scratch.branch_derivatives[313] = [0.0; Instance::BRANCH_COUNT];
        }

        scratch.values[493] = if (scratch.values[13] == 0.5) { 1.0 } else { 0.0 };

        if ((((scratch.values[270] != 0.0) && (!(scratch.values[481] != 0.0))) && (!(scratch.values[492] != 0.0))) && (scratch.values[493] != 0.0)) {
            scratch.store_ad(289, &AdValue::sqrt(AdValue::scale(AdValue::sub_from_scalar(scratch.values[10], scratch.ad_value(287)), scratch.values[116])));
        }

        if ((((scratch.values[270] != 0.0) && (!(scratch.values[481] != 0.0))) && (!(scratch.values[492] != 0.0))) && (!(scratch.values[493] != 0.0))) {
            scratch.store_ad(289, &AdValue::powf(AdValue::scale(AdValue::sub_from_scalar(scratch.values[10], scratch.ad_value(287)), scratch.values[116]), scratch.values[13]));
        }

        if (((scratch.values[270] != 0.0) && (!(scratch.values[481] != 0.0))) && (!(scratch.values[492] != 0.0))) {
            scratch.store_ad(314, &AdValue::scale(AdValue::div(AdValue::scale(AdValue::sub_from_scalar(scratch.values[10], scratch.ad_value(287)), scratch.values[113]), scratch.ad_value(289)), scratch.values[98]));
        }

        scratch.values[494] = if (((((-scratch.values[128]) / scratch.values[314])) as f64).abs() < 230.25850929940458) { 1.0 } else { 0.0 };

        if ((((scratch.values[270] != 0.0) && (!(scratch.values[481] != 0.0))) && (!(scratch.values[492] != 0.0))) && (scratch.values[494] != 0.0)) {
            scratch.store_ad(289, &AdValue::exp(AdValue::div(AdValue::neg(scratch.ad_value(128)), scratch.ad_value(314))));
        }

        scratch.values[495] = if (((-scratch.values[128]) / scratch.values[314]) < 0.0) { 1.0 } else { 0.0 };

        if (((((scratch.values[270] != 0.0) && (!(scratch.values[481] != 0.0))) && (!(scratch.values[492] != 0.0))) && (!(scratch.values[494] != 0.0))) && (scratch.values[495] != 0.0)) {
            let assign10140_ad_e12333: AdValue = AdValue::offset(AdValue::mul(AdValue::sub_from_scalar((-230.25850929940458), AdValue::div(AdValue::neg(scratch.ad_value(128)), scratch.ad_value(314))), AdValue::offset(AdValue::scale(AdValue::mul(AdValue::sub_from_scalar((-230.25850929940458), AdValue::div(AdValue::neg(scratch.ad_value(128)), scratch.ad_value(314))), AdValue::offset(AdValue::scale(AdValue::sub_from_scalar((-230.25850929940458), AdValue::div(AdValue::neg(scratch.ad_value(128)), scratch.ad_value(314))), 0.3333333333333333), 1.0)), 0.5), 1.0)), 1.0);
            scratch.store_ad(289, &AdValue::div_from_scalar(1e-100, assign10140_ad_e12333));
        }

        if (((((scratch.values[270] != 0.0) && (!(scratch.values[481] != 0.0))) && (!(scratch.values[492] != 0.0))) && (!(scratch.values[494] != 0.0))) && (!(scratch.values[495] != 0.0))) {
            let assign10150_ad_e12381: AdValue = AdValue::scale(AdValue::offset(AdValue::mul(AdValue::offset(AdValue::div(AdValue::neg(scratch.ad_value(128)), scratch.ad_value(314)), (-230.25850929940458)), AdValue::offset(AdValue::scale(AdValue::mul(AdValue::offset(AdValue::div(AdValue::neg(scratch.ad_value(128)), scratch.ad_value(314)), (-230.25850929940458)), AdValue::offset(AdValue::scale(AdValue::offset(AdValue::div(AdValue::neg(scratch.ad_value(128)), scratch.ad_value(314)), (-230.25850929940458)), 0.3333333333333333), 1.0)), 0.5), 1.0)), 1.0), 1e100);
            scratch.store_ad(289, &assign10150_ad_e12381);
        }

        if (((scratch.values[270] != 0.0) && (!(scratch.values[481] != 0.0))) && (!(scratch.values[492] != 0.0))) {
            scratch.store_ad(313, &AdValue::scale(AdValue::mul(AdValue::mul(AdValue::mul(scratch.ad_value(154), scratch.ad_value(314)), scratch.ad_value(314)), scratch.ad_value(289)), scratch.values[33]));
        }

        scratch.values[496] = if (scratch.values[42] > 1000.0) { 1.0 } else { 0.0 };

        if (((scratch.values[270] != 0.0) && (!(scratch.values[481] != 0.0))) && (scratch.values[496] != 0.0)) {
            scratch.values[315] = 1.0;
            scratch.node_derivatives[315] = [0.0; Instance::NODE_COUNT];
            scratch.branch_derivatives[315] = [0.0; Instance::BRANCH_COUNT];
        }

        scratch.values[497] = if (scratch.values[288] > ((-scratch.values[129]) * scratch.values[42])) { 1.0 } else { 0.0 };

        scratch.values[498] = if (scratch.values[45] == 4.0) { 1.0 } else { 0.0 };

    }

    pub(super) fn stamp_transient_block_7(
        &mut self,
        ctx: &GeneratedEvalContext<'_>,
        stamper: &mut GeneratedStamper<'_>,
        scratch: &mut Scratch,
    ) {
        let _ = stamper;
        if (((((scratch.values[270] != 0.0) && (!(scratch.values[481] != 0.0))) && (!(scratch.values[496] != 0.0))) && (scratch.values[497] != 0.0)) && (scratch.values[498] != 0.0)) {
            scratch.store_ad(289, &AdValue::mul(AdValue::mul(AdValue::mul(AdValue::scale(scratch.ad_value(288), scratch.values[135]), AdValue::scale(scratch.ad_value(288), scratch.values[135])), AdValue::scale(scratch.ad_value(288), scratch.values[135])), AdValue::scale(scratch.ad_value(288), scratch.values[135])));
        }

        if (((((scratch.values[270] != 0.0) && (!(scratch.values[481] != 0.0))) && (!(scratch.values[496] != 0.0))) && (scratch.values[497] != 0.0)) && (!(scratch.values[498] != 0.0))) {
            scratch.store_ad(289, &AdValue::powf(AdValue::abs(AdValue::scale(scratch.ad_value(288), scratch.values[135])), scratch.values[45]));
        }

        if ((((scratch.values[270] != 0.0) && (!(scratch.values[481] != 0.0))) && (!(scratch.values[496] != 0.0))) && (scratch.values[497] != 0.0)) {
            scratch.store_ad(315, &AdValue::div_from_scalar(1.0, AdValue::sub_from_scalar(1.0, scratch.ad_value(289))));
        }

        if ((((scratch.values[270] != 0.0) && (!(scratch.values[481] != 0.0))) && (!(scratch.values[496] != 0.0))) && (!(scratch.values[497] != 0.0))) {
            scratch.store_ad(315, &AdValue::offset(AdValue::scale(AdValue::offset(scratch.ad_value(288), (scratch.values[129] * scratch.values[42])), scratch.values[138]), scratch.values[132]));
        }

        if ((scratch.values[270] != 0.0) && (!(scratch.values[481] != 0.0))) {
            scratch.store_ad(318, &AdValue::mul(AdValue::add(AdValue::add(AdValue::add(scratch.ad_value(290), scratch.ad_value(291)), scratch.ad_value(298)), scratch.ad_value(313)), scratch.ad_value(315)));
        }

        if (scratch.values[270] != 0.0) {
            scratch.store_ad(144, &AdValue::add(AdValue::add(AdValue::scale(scratch.ad_value(316), scratch.values[217]), AdValue::scale(scratch.ad_value(317), scratch.values[218])), AdValue::scale(scratch.ad_value(318), scratch.values[219])));
        }

        if (scratch.values[270] != 0.0) {
            scratch.values[287] = 0.0;
            scratch.node_derivatives[287] = [0.0; Instance::NODE_COUNT];
            scratch.branch_derivatives[287] = [0.0; Instance::BRANCH_COUNT];
        }

        if (scratch.values[270] != 0.0) {
            scratch.values[284] = 0.0;
            scratch.node_derivatives[284] = [0.0; Instance::NODE_COUNT];
            scratch.branch_derivatives[284] = [0.0; Instance::BRANCH_COUNT];
        }

        scratch.values[499] = if !(((scratch.values[217] == 0.0) && (scratch.values[218] == 0.0)) && (scratch.values[219] == 0.0)) { 1.0 } else { 0.0 };

        if ((scratch.values[270] != 0.0) && (scratch.values[499] != 0.0)) {
            scratch.store_ad(274, &AdValue::mul(AdValue::scale(scratch.ad_value(226), 4.0), scratch.ad_value(226)));
        }

        if ((scratch.values[270] != 0.0) && (scratch.values[499] != 0.0)) {
            scratch.store_ad(275, &AdValue::div(scratch.ad_value(226), scratch.ad_value(227)));
        }

        if ((scratch.values[270] != 0.0) && (scratch.values[499] != 0.0)) {
            scratch.store_ad(276, &AdValue::add(scratch.ad_value(155), AdValue::mul(scratch.ad_value(226), scratch.ad_value(275))));
        }

        if ((scratch.values[270] != 0.0) && (scratch.values[499] != 0.0)) {
            scratch.store_ad(277, &AdValue::add(scratch.ad_value(227), scratch.ad_value(276)));
        }

        if ((scratch.values[270] != 0.0) && (scratch.values[499] != 0.0)) {
            scratch.store_ad(278, &AdValue::sub(scratch.ad_value(227), scratch.ad_value(276)));
        }

        if ((scratch.values[270] != 0.0) && (scratch.values[499] != 0.0)) {
            scratch.store_ad(279, &AdValue::sqrt(AdValue::add(AdValue::square(scratch.ad_value(278)), scratch.ad_value(274))));
        }

        if ((scratch.values[270] != 0.0) && (scratch.values[499] != 0.0)) {
            scratch.store_ad(281, &AdValue::scale(AdValue::div(AdValue::mul(scratch.ad_value(155), scratch.ad_value(227)), AdValue::add(scratch.ad_value(277), scratch.ad_value(279))), 2.0));
        }

        scratch.values[500] = if (scratch.values[155] < scratch.values[223]) { 1.0 } else { 0.0 };

        scratch.values[501] = if ((((0.5 * (scratch.values[155] * scratch.values[56]))) as f64).abs() < 230.25850929940458) { 1.0 } else { 0.0 };

        if ((((scratch.values[270] != 0.0) && (scratch.values[499] != 0.0)) && (scratch.values[500] != 0.0)) && (scratch.values[501] != 0.0)) {
            scratch.store_ad(283, &AdValue::exp(AdValue::scale(scratch.ad_value(155), (scratch.values[56] * 0.5))));
        }

        scratch.values[502] = if ((0.5 * (scratch.values[155] * scratch.values[56])) < 0.0) { 1.0 } else { 0.0 };

        if (((((scratch.values[270] != 0.0) && (scratch.values[499] != 0.0)) && (scratch.values[500] != 0.0)) && (!(scratch.values[501] != 0.0))) && (scratch.values[502] != 0.0)) {
            let assign10410_ad_e12706: AdValue = AdValue::div_from_scalar(1e-100, AdValue::offset(AdValue::mul(AdValue::sub_from_scalar((-230.25850929940458), AdValue::scale(scratch.ad_value(155), (scratch.values[56] * 0.5))), AdValue::offset(AdValue::scale(AdValue::mul(AdValue::sub_from_scalar((-230.25850929940458), AdValue::scale(scratch.ad_value(155), (scratch.values[56] * 0.5))), AdValue::offset(AdValue::scale(AdValue::sub_from_scalar((-230.25850929940458), AdValue::scale(scratch.ad_value(155), (scratch.values[56] * 0.5))), 0.3333333333333333), 1.0)), 0.5), 1.0)), 1.0));
            scratch.store_ad(283, &assign10410_ad_e12706);
        }

        if (((((scratch.values[270] != 0.0) && (scratch.values[499] != 0.0)) && (scratch.values[500] != 0.0)) && (!(scratch.values[501] != 0.0))) && (!(scratch.values[502] != 0.0))) {
            scratch.store_ad(283, &AdValue::scale(AdValue::offset(AdValue::mul(AdValue::offset(AdValue::scale(scratch.ad_value(155), (scratch.values[56] * 0.5)), (-230.25850929940458)), AdValue::offset(AdValue::scale(AdValue::mul(AdValue::offset(AdValue::scale(scratch.ad_value(155), (scratch.values[56] * 0.5)), (-230.25850929940458)), AdValue::offset(AdValue::scale(AdValue::offset(AdValue::scale(scratch.ad_value(155), (scratch.values[56] * 0.5)), (-230.25850929940458)), 0.3333333333333333), 1.0)), 0.5), 1.0)), 1.0), 1e100));
        }

        if (((scratch.values[270] != 0.0) && (scratch.values[499] != 0.0)) && (scratch.values[500] != 0.0)) {
            scratch.store_ad(280, &AdValue::square(scratch.ad_value(283)));
        }

        if (((scratch.values[270] != 0.0) && (scratch.values[499] != 0.0)) && (!(scratch.values[500] != 0.0))) {
            scratch.store_ad(280, &AdValue::mul(AdValue::offset(AdValue::scale(AdValue::sub(scratch.ad_value(155), scratch.ad_value(223)), scratch.values[56]), 1.0), scratch.ad_value(224)));
        }

        if (((scratch.values[270] != 0.0) && (scratch.values[499] != 0.0)) && (!(scratch.values[500] != 0.0))) {
            scratch.store_ad(283, &AdValue::sqrt(scratch.ad_value(280)));
        }

        if ((scratch.values[270] != 0.0) && (scratch.values[499] != 0.0)) {
            scratch.store_ad(280, &AdValue::offset(scratch.ad_value(280), (-1.0)));
        }

        if ((scratch.values[270] != 0.0) && (scratch.values[499] != 0.0)) {
            scratch.store_ad(282, &AdValue::div_from_scalar(1.0, scratch.ad_value(283)));
        }

        scratch.values[503] = if (scratch.values[155] > 0.0) { 1.0 } else { 0.0 };

        if (((scratch.values[270] != 0.0) && (scratch.values[499] != 0.0)) && (scratch.values[503] != 0.0)) {
            scratch.store_ad(284, &AdValue::scale(AdValue::ln(AdValue::add(AdValue::offset(scratch.ad_value(282), 2.0), AdValue::sqrt(AdValue::mul(AdValue::offset(scratch.ad_value(282), 1.0), AdValue::offset(scratch.ad_value(282), 3.0))))), (scratch.values[55] * 2.0)));
        }

        if (((scratch.values[270] != 0.0) && (scratch.values[499] != 0.0)) && (!(scratch.values[503] != 0.0))) {
            scratch.store_ad(284, &AdValue::sub(AdValue::scale(AdValue::ln(AdValue::add(AdValue::offset(AdValue::scale(scratch.ad_value(283), 2.0), 1.0), AdValue::sqrt(AdValue::mul(AdValue::offset(scratch.ad_value(283), 1.0), AdValue::offset(AdValue::scale(scratch.ad_value(283), 3.0), 1.0))))), (scratch.values[55] * 2.0)), scratch.ad_value(155)));
        }

        if ((scratch.values[270] != 0.0) && (scratch.values[499] != 0.0)) {
            scratch.store_ad(285, &AdValue::sub(scratch.ad_value(225), scratch.ad_value(284)));
        }

        if ((scratch.values[270] != 0.0) && (scratch.values[499] != 0.0)) {
            scratch.store_ad(286, &AdValue::scale(AdValue::sub(AdValue::add(scratch.ad_value(155), scratch.ad_value(285)), AdValue::sqrt(AdValue::offset(AdValue::mul(AdValue::sub(scratch.ad_value(155), scratch.ad_value(285)), AdValue::sub(scratch.ad_value(155), scratch.ad_value(285))), ((4.0 * scratch.values[55]) * scratch.values[55])))), 0.5));
        }

        if ((scratch.values[270] != 0.0) && (scratch.values[499] != 0.0)) {
            scratch.store_ad(287, &AdValue::scale(AdValue::sub(AdValue::add(scratch.ad_value(155), scratch.ad_value(228)), AdValue::sqrt(AdValue::offset(AdValue::mul(AdValue::sub(scratch.ad_value(155), scratch.ad_value(228)), AdValue::sub(scratch.ad_value(155), scratch.ad_value(228))), ((4.0 * scratch.values[53]) * scratch.values[53])))), 0.5));
        }

        if ((scratch.values[270] != 0.0) && (scratch.values[499] != 0.0)) {
            scratch.store_ad(288, &AdValue::scale(AdValue::sub(scratch.ad_value(155), AdValue::sqrt(AdValue::offset(AdValue::mul(scratch.ad_value(155), scratch.ad_value(155)), ((4.0 * 1e-6) * 1e-6)))), 0.5));
        }

        scratch.values[504] = if (scratch.values[217] == 0.0) { 1.0 } else { 0.0 };

        if ((scratch.values[270] != 0.0) && (scratch.values[504] != 0.0)) {
            scratch.values[316] = 0.0;
            scratch.node_derivatives[316] = [0.0; Instance::NODE_COUNT];
            scratch.branch_derivatives[316] = [0.0; Instance::BRANCH_COUNT];
        }

        scratch.values[505] = if (scratch.values[93] == 0.5) { 1.0 } else { 0.0 };

        if (((scratch.values[270] != 0.0) && (!(scratch.values[504] != 0.0))) && (scratch.values[505] != 0.0)) {
            scratch.store_ad(289, &AdValue::sqrt(AdValue::sub_from_scalar(1.0, AdValue::scale(scratch.ad_value(281), scratch.values[90]))));
        }

        if (((scratch.values[270] != 0.0) && (!(scratch.values[504] != 0.0))) && (!(scratch.values[505] != 0.0))) {
            scratch.store_ad(289, &AdValue::powf(AdValue::sub_from_scalar(1.0, AdValue::scale(scratch.ad_value(281), scratch.values[90])), scratch.values[93]));
        }

        if ((scratch.values[270] != 0.0) && (!(scratch.values[504] != 0.0))) {
            scratch.store_ad(290, &AdValue::scale(scratch.ad_value(280), scratch.values[72]));
        }

        scratch.values[506] = if ((scratch.values[22] == 0.0) && (scratch.values[25] == 0.0)) { 1.0 } else { 0.0 };

        if (((scratch.values[270] != 0.0) && (!(scratch.values[504] != 0.0))) && (scratch.values[506] != 0.0)) {
            scratch.values[291] = 0.0;
            scratch.node_derivatives[291] = [0.0; Instance::NODE_COUNT];
            scratch.branch_derivatives[291] = [0.0; Instance::BRANCH_COUNT];
        }

        if (((scratch.values[270] != 0.0) && (!(scratch.values[504] != 0.0))) && (!(scratch.values[506] != 0.0))) {
            scratch.store_ad(292, &AdValue::sub_from_scalar(scratch.values[78], scratch.ad_value(286)));
        }

        if (((scratch.values[270] != 0.0) && (!(scratch.values[504] != 0.0))) && (!(scratch.values[506] != 0.0))) {
            scratch.store_ad(293, &AdValue::sub_from_scalar(1.0, AdValue::sqrt(AdValue::sub_from_scalar(1.0, AdValue::div(scratch.ad_value(284), scratch.ad_value(292))))));
        }

        scratch.values[507] = if (scratch.values[11] == 0.5) { 1.0 } else { 0.0 };

        if ((((scratch.values[270] != 0.0) && (!(scratch.values[504] != 0.0))) && (!(scratch.values[506] != 0.0))) && (scratch.values[507] != 0.0)) {
            scratch.values[294] = 0.0;
            scratch.node_derivatives[294] = [0.0; Instance::NODE_COUNT];
            scratch.branch_derivatives[294] = [0.0; Instance::BRANCH_COUNT];
        }

        if ((((scratch.values[270] != 0.0) && (!(scratch.values[504] != 0.0))) && (!(scratch.values[506] != 0.0))) && (!(scratch.values[507] != 0.0))) {
            scratch.store_ad(294, &AdValue::scale(AdValue::add(AdValue::div(AdValue::mul(AdValue::square(scratch.ad_value(293)), AdValue::ln(scratch.ad_value(293))), AdValue::sub_from_scalar(1.0, scratch.ad_value(293))), scratch.ad_value(293)), (1.0 - (2.0 * scratch.values[11]))));
        }

        if (((scratch.values[270] != 0.0) && (!(scratch.values[504] != 0.0))) && (!(scratch.values[506] != 0.0))) {
            scratch.store_ad(295, &AdValue::add(scratch.ad_value(293), scratch.ad_value(294)));
        }

        scratch.values[508] = if (scratch.values[11] == 0.5) { 1.0 } else { 0.0 };

        if ((((scratch.values[270] != 0.0) && (!(scratch.values[504] != 0.0))) && (!(scratch.values[506] != 0.0))) && (scratch.values[508] != 0.0)) {
            scratch.store_ad(289, &AdValue::sqrt(AdValue::scale(scratch.ad_value(292), scratch.values[114])));
        }

        if ((((scratch.values[270] != 0.0) && (!(scratch.values[504] != 0.0))) && (!(scratch.values[506] != 0.0))) && (!(scratch.values[508] != 0.0))) {
            scratch.store_ad(289, &AdValue::powf(AdValue::scale(scratch.ad_value(292), scratch.values[114]), scratch.values[11]));
        }

        if (((scratch.values[270] != 0.0) && (!(scratch.values[504] != 0.0))) && (!(scratch.values[506] != 0.0))) {
            scratch.store_ad(296, &AdValue::scale(scratch.ad_value(289), scratch.values[108]));
        }

        if (((scratch.values[270] != 0.0) && (!(scratch.values[504] != 0.0))) && (!(scratch.values[506] != 0.0))) {
            scratch.store_ad(297, &AdValue::scale(AdValue::mul(AdValue::offset(scratch.ad_value(283), (-1.0)), scratch.ad_value(296)), scratch.values[69]));
        }

        if (((scratch.values[270] != 0.0) && (!(scratch.values[504] != 0.0))) && (!(scratch.values[506] != 0.0))) {
            scratch.store_ad(291, &AdValue::scale(AdValue::mul(scratch.ad_value(297), scratch.ad_value(295)), scratch.values[22]));
        }

        scratch.values[509] = if (scratch.values[25] == 0.0) { 1.0 } else { 0.0 };

        if (((scratch.values[270] != 0.0) && (!(scratch.values[504] != 0.0))) && (scratch.values[509] != 0.0)) {
            scratch.values[298] = 0.0;
            scratch.node_derivatives[298] = [0.0; Instance::NODE_COUNT];
            scratch.branch_derivatives[298] = [0.0; Instance::BRANCH_COUNT];
        }

        if (((scratch.values[270] != 0.0) && (!(scratch.values[504] != 0.0))) && (!(scratch.values[509] != 0.0))) {
            scratch.store_ad(299, &AdValue::scale(AdValue::div(AdValue::scale(scratch.ad_value(296), scratch.values[93]), scratch.ad_value(292)), scratch.values[123]));
        }

        if (((scratch.values[270] != 0.0) && (!(scratch.values[504] != 0.0))) && (!(scratch.values[509] != 0.0))) {
            scratch.store_ad(300, &AdValue::div_from_scalar((0.666666666666667 * scratch.values[120]), scratch.ad_value(299)));
        }

        if (((scratch.values[270] != 0.0) && (!(scratch.values[504] != 0.0))) && (!(scratch.values[509] != 0.0))) {
            scratch.store_ad(301, &AdValue::square(scratch.ad_value(300)));
        }

        if (((scratch.values[270] != 0.0) && (!(scratch.values[504] != 0.0))) && (!(scratch.values[509] != 0.0))) {
            scratch.store_ad(302, &AdValue::sqrt(AdValue::div(AdValue::square(scratch.ad_value(301)), AdValue::offset(AdValue::square(scratch.ad_value(301)), 1.0))));
        }

        if (((scratch.values[270] != 0.0) && (!(scratch.values[504] != 0.0))) && (!(scratch.values[509] != 0.0))) {
            scratch.store_ad(303, &AdValue::sqrt(AdValue::abs(scratch.ad_value(302))));
        }

        if (((scratch.values[270] != 0.0) && (!(scratch.values[504] != 0.0))) && (!(scratch.values[509] != 0.0))) {
            scratch.store_ad(304, &AdValue::mul(scratch.ad_value(302), scratch.ad_value(303)));
        }

        scratch.values[510] = if (((-scratch.values[11]) * scratch.values[96]) == (-1.0)) { 1.0 } else { 0.0 };

        if ((((scratch.values[270] != 0.0) && (!(scratch.values[504] != 0.0))) && (!(scratch.values[509] != 0.0))) && (scratch.values[510] != 0.0)) {
            scratch.store_ad(305, &AdValue::div_from_scalar(1.0, AdValue::offset(AdValue::mul(scratch.ad_value(299), scratch.ad_value(304)), 1.0)));
        }

        if ((((scratch.values[270] != 0.0) && (!(scratch.values[504] != 0.0))) && (!(scratch.values[509] != 0.0))) && (!(scratch.values[510] != 0.0))) {
            scratch.store_ad(305, &AdValue::powf(AdValue::offset(AdValue::mul(scratch.ad_value(299), scratch.ad_value(304)), 1.0), ((-scratch.values[11]) * scratch.values[96])));
        }

        if (((scratch.values[270] != 0.0) && (!(scratch.values[504] != 0.0))) && (!(scratch.values[509] != 0.0))) {
            scratch.store_ad(306, &AdValue::div(AdValue::mul(scratch.ad_value(295), scratch.ad_value(305)), AdValue::add(scratch.ad_value(295), scratch.ad_value(305))));
        }

        if (((scratch.values[270] != 0.0) && (!(scratch.values[504] != 0.0))) && (!(scratch.values[509] != 0.0))) {
            scratch.store_ad(307, &AdValue::sqrt(AdValue::scale(AdValue::div(scratch.ad_value(299), scratch.ad_value(303)), 0.375)));
        }

        if (((scratch.values[270] != 0.0) && (!(scratch.values[504] != 0.0))) && (!(scratch.values[509] != 0.0))) {
            scratch.store_ad(308, &AdValue::sub(AdValue::scale(AdValue::mul(scratch.ad_value(300), scratch.ad_value(303)), 2.0), scratch.ad_value(302)));
        }

        if (((scratch.values[270] != 0.0) && (!(scratch.values[504] != 0.0))) && (!(scratch.values[509] != 0.0))) {
            scratch.store_ad(309, &AdValue::add(AdValue::sub(AdValue::mul(AdValue::scale(scratch.ad_value(300), scratch.values[120]), scratch.ad_value(303)), AdValue::scale(scratch.ad_value(302), scratch.values[120])), AdValue::scale(AdValue::mul(scratch.ad_value(299), scratch.ad_value(304)), 0.5)));
        }

        if (((scratch.values[270] != 0.0) && (!(scratch.values[504] != 0.0))) && (!(scratch.values[509] != 0.0))) {
            scratch.store_ad(310, &AdValue::mul(AdValue::offset(scratch.ad_value(308), (-1.0)), scratch.ad_value(307)));
        }

        if (((scratch.values[270] != 0.0) && (!(scratch.values[504] != 0.0))) && (!(scratch.values[509] != 0.0))) {
            scratch.store_ad(271, &AdValue::square(scratch.ad_value(310)));
        }

        scratch.values[511] = if (scratch.values[310] > 0.0) { 1.0 } else { 0.0 };

        if ((((scratch.values[270] != 0.0) && (!(scratch.values[504] != 0.0))) && (!(scratch.values[509] != 0.0))) && (scratch.values[511] != 0.0)) {
            scratch.store_ad(272, &AdValue::div_from_scalar(1.0, AdValue::offset(AdValue::scale(scratch.ad_value(310), scratch.values[57]), 1.0)));
        }

        if ((((scratch.values[270] != 0.0) && (!(scratch.values[504] != 0.0))) && (!(scratch.values[509] != 0.0))) && (!(scratch.values[511] != 0.0))) {
            scratch.store_ad(272, &AdValue::div_from_scalar(1.0, AdValue::sub_from_scalar(1.0, AdValue::scale(scratch.ad_value(310), scratch.values[57]))));
        }

        scratch.values[512] = if (((-scratch.values[271]) + scratch.values[309]) > (-230.25850929940458)) { 1.0 } else { 0.0 };

        if ((((scratch.values[270] != 0.0) && (!(scratch.values[504] != 0.0))) && (!(scratch.values[509] != 0.0))) && (scratch.values[512] != 0.0)) {
            scratch.store_ad(289, &AdValue::exp(AdValue::sub(scratch.ad_value(309), scratch.ad_value(271))));
        }

        if ((((scratch.values[270] != 0.0) && (!(scratch.values[504] != 0.0))) && (!(scratch.values[509] != 0.0))) && (!(scratch.values[512] != 0.0))) {
            scratch.store_ad(289, &AdValue::div_from_scalar(1e-100, AdValue::offset(AdValue::mul(AdValue::sub_from_scalar((-230.25850929940458), AdValue::sub(scratch.ad_value(309), scratch.ad_value(271))), AdValue::offset(AdValue::scale(AdValue::mul(AdValue::sub_from_scalar((-230.25850929940458), AdValue::sub(scratch.ad_value(309), scratch.ad_value(271))), AdValue::offset(AdValue::scale(AdValue::sub_from_scalar((-230.25850929940458), AdValue::sub(scratch.ad_value(309), scratch.ad_value(271))), 0.3333333333333333), 1.0)), 0.5), 1.0)), 1.0)));
        }

        if (((scratch.values[270] != 0.0) && (!(scratch.values[504] != 0.0))) && (!(scratch.values[509] != 0.0))) {
            scratch.store_ad(273, &AdValue::mul(AdValue::add(AdValue::add(AdValue::scale(scratch.ad_value(272), 0.29214664), AdValue::scale(AdValue::square(scratch.ad_value(272)), scratch.values[58])), AdValue::scale(AdValue::mul(AdValue::square(scratch.ad_value(272)), scratch.ad_value(272)), scratch.values[59])), scratch.ad_value(289)));
        }

        scratch.values[513] = if (scratch.values[310] > 0.0) { 1.0 } else { 0.0 };

        if ((((scratch.values[270] != 0.0) && (!(scratch.values[504] != 0.0))) && (!(scratch.values[509] != 0.0))) && (scratch.values[513] != 0.0)) {
            scratch.values[311] = scratch.values[273];
            scratch.node_derivatives[311] = scratch.node_derivatives[273];
            scratch.branch_derivatives[311] = scratch.branch_derivatives[273];
        }

        scratch.values[514] = if (scratch.values[309] > (-230.25850929940458)) { 1.0 } else { 0.0 };

        if (((((scratch.values[270] != 0.0) && (!(scratch.values[504] != 0.0))) && (!(scratch.values[509] != 0.0))) && (!(scratch.values[513] != 0.0))) && (scratch.values[514] != 0.0)) {
            scratch.store_ad(289, &AdValue::exp(scratch.ad_value(309)));
        }

        if (((((scratch.values[270] != 0.0) && (!(scratch.values[504] != 0.0))) && (!(scratch.values[509] != 0.0))) && (!(scratch.values[513] != 0.0))) && (!(scratch.values[514] != 0.0))) {
            scratch.store_ad(289, &AdValue::div_from_scalar(1e-100, AdValue::offset(AdValue::mul(AdValue::sub_from_scalar((-230.25850929940458), scratch.ad_value(309)), AdValue::offset(AdValue::scale(AdValue::mul(AdValue::sub_from_scalar((-230.25850929940458), scratch.ad_value(309)), AdValue::offset(AdValue::scale(AdValue::sub_from_scalar((-230.25850929940458), scratch.ad_value(309)), 0.3333333333333333), 1.0)), 0.5), 1.0)), 1.0)));
        }

        if ((((scratch.values[270] != 0.0) && (!(scratch.values[504] != 0.0))) && (!(scratch.values[509] != 0.0))) && (!(scratch.values[513] != 0.0))) {
            scratch.store_ad(311, &AdValue::sub(AdValue::scale(scratch.ad_value(289), 2.0), scratch.ad_value(273)));
        }

        if (((scratch.values[270] != 0.0) && (!(scratch.values[504] != 0.0))) && (!(scratch.values[509] != 0.0))) {
            scratch.store_ad(312, &AdValue::scale(AdValue::div(AdValue::scale(scratch.ad_value(311), scratch.values[120]), scratch.ad_value(307)), (1.772453850905516 * 0.5)));
        }

        if (((scratch.values[270] != 0.0) && (!(scratch.values[504] != 0.0))) && (!(scratch.values[509] != 0.0))) {
            scratch.store_ad(298, &AdValue::scale(AdValue::mul(AdValue::mul(scratch.ad_value(297), scratch.ad_value(312)), scratch.ad_value(306)), scratch.values[25]));
        }

        scratch.values[515] = if (scratch.values[31] == 0.0) { 1.0 } else { 0.0 };

        if (((scratch.values[270] != 0.0) && (!(scratch.values[504] != 0.0))) && (scratch.values[515] != 0.0)) {
            scratch.values[313] = 0.0;
            scratch.node_derivatives[313] = [0.0; Instance::NODE_COUNT];
            scratch.branch_derivatives[313] = [0.0; Instance::BRANCH_COUNT];
        }

        scratch.values[516] = if (scratch.values[11] == 0.5) { 1.0 } else { 0.0 };

        if ((((scratch.values[270] != 0.0) && (!(scratch.values[504] != 0.0))) && (!(scratch.values[515] != 0.0))) && (scratch.values[516] != 0.0)) {
            scratch.store_ad(289, &AdValue::sqrt(AdValue::scale(AdValue::sub_from_scalar(scratch.values[8], scratch.ad_value(287)), scratch.values[114])));
        }

        if ((((scratch.values[270] != 0.0) && (!(scratch.values[504] != 0.0))) && (!(scratch.values[515] != 0.0))) && (!(scratch.values[516] != 0.0))) {
            scratch.store_ad(289, &AdValue::powf(AdValue::scale(AdValue::sub_from_scalar(scratch.values[8], scratch.ad_value(287)), scratch.values[114]), scratch.values[11]));
        }

        if (((scratch.values[270] != 0.0) && (!(scratch.values[504] != 0.0))) && (!(scratch.values[515] != 0.0))) {
            scratch.store_ad(314, &AdValue::scale(AdValue::div(AdValue::scale(AdValue::sub_from_scalar(scratch.values[8], scratch.ad_value(287)), scratch.values[111]), scratch.ad_value(289)), scratch.values[96]));
        }

        scratch.values[517] = if (((((-scratch.values[126]) / scratch.values[314])) as f64).abs() < 230.25850929940458) { 1.0 } else { 0.0 };

        if ((((scratch.values[270] != 0.0) && (!(scratch.values[504] != 0.0))) && (!(scratch.values[515] != 0.0))) && (scratch.values[517] != 0.0)) {
            scratch.store_ad(289, &AdValue::exp(AdValue::div(AdValue::neg(scratch.ad_value(126)), scratch.ad_value(314))));
        }

        scratch.values[518] = if (((-scratch.values[126]) / scratch.values[314]) < 0.0) { 1.0 } else { 0.0 };

        if (((((scratch.values[270] != 0.0) && (!(scratch.values[504] != 0.0))) && (!(scratch.values[515] != 0.0))) && (!(scratch.values[517] != 0.0))) && (scratch.values[518] != 0.0)) {
            let assign11180_ad_e13854: AdValue = AdValue::offset(AdValue::mul(AdValue::sub_from_scalar((-230.25850929940458), AdValue::div(AdValue::neg(scratch.ad_value(126)), scratch.ad_value(314))), AdValue::offset(AdValue::scale(AdValue::mul(AdValue::sub_from_scalar((-230.25850929940458), AdValue::div(AdValue::neg(scratch.ad_value(126)), scratch.ad_value(314))), AdValue::offset(AdValue::scale(AdValue::sub_from_scalar((-230.25850929940458), AdValue::div(AdValue::neg(scratch.ad_value(126)), scratch.ad_value(314))), 0.3333333333333333), 1.0)), 0.5), 1.0)), 1.0);
            scratch.store_ad(289, &AdValue::div_from_scalar(1e-100, assign11180_ad_e13854));
        }

        if (((((scratch.values[270] != 0.0) && (!(scratch.values[504] != 0.0))) && (!(scratch.values[515] != 0.0))) && (!(scratch.values[517] != 0.0))) && (!(scratch.values[518] != 0.0))) {
            let assign11190_ad_e13902: AdValue = AdValue::scale(AdValue::offset(AdValue::mul(AdValue::offset(AdValue::div(AdValue::neg(scratch.ad_value(126)), scratch.ad_value(314)), (-230.25850929940458)), AdValue::offset(AdValue::scale(AdValue::mul(AdValue::offset(AdValue::div(AdValue::neg(scratch.ad_value(126)), scratch.ad_value(314)), (-230.25850929940458)), AdValue::offset(AdValue::scale(AdValue::offset(AdValue::div(AdValue::neg(scratch.ad_value(126)), scratch.ad_value(314)), (-230.25850929940458)), 0.3333333333333333), 1.0)), 0.5), 1.0)), 1.0), 1e100);
            scratch.store_ad(289, &assign11190_ad_e13902);
        }

        if (((scratch.values[270] != 0.0) && (!(scratch.values[504] != 0.0))) && (!(scratch.values[515] != 0.0))) {
            scratch.store_ad(313, &AdValue::scale(AdValue::mul(AdValue::mul(AdValue::mul(scratch.ad_value(155), scratch.ad_value(314)), scratch.ad_value(314)), scratch.ad_value(289)), scratch.values[31]));
        }

        scratch.values[519] = if (scratch.values[40] > 1000.0) { 1.0 } else { 0.0 };

        if (((scratch.values[270] != 0.0) && (!(scratch.values[504] != 0.0))) && (scratch.values[519] != 0.0)) {
            scratch.values[315] = 1.0;
            scratch.node_derivatives[315] = [0.0; Instance::NODE_COUNT];
            scratch.branch_derivatives[315] = [0.0; Instance::BRANCH_COUNT];
        }

        scratch.values[520] = if (scratch.values[288] > ((-scratch.values[129]) * scratch.values[40])) { 1.0 } else { 0.0 };

        scratch.values[521] = if (scratch.values[43] == 4.0) { 1.0 } else { 0.0 };

        if (((((scratch.values[270] != 0.0) && (!(scratch.values[504] != 0.0))) && (!(scratch.values[519] != 0.0))) && (scratch.values[520] != 0.0)) && (scratch.values[521] != 0.0)) {
            scratch.store_ad(289, &AdValue::mul(AdValue::mul(AdValue::mul(AdValue::scale(scratch.ad_value(288), scratch.values[133]), AdValue::scale(scratch.ad_value(288), scratch.values[133])), AdValue::scale(scratch.ad_value(288), scratch.values[133])), AdValue::scale(scratch.ad_value(288), scratch.values[133])));
        }

        if (((((scratch.values[270] != 0.0) && (!(scratch.values[504] != 0.0))) && (!(scratch.values[519] != 0.0))) && (scratch.values[520] != 0.0)) && (!(scratch.values[521] != 0.0))) {
            scratch.store_ad(289, &AdValue::powf(AdValue::abs(AdValue::scale(scratch.ad_value(288), scratch.values[133])), scratch.values[43]));
        }

        if ((((scratch.values[270] != 0.0) && (!(scratch.values[504] != 0.0))) && (!(scratch.values[519] != 0.0))) && (scratch.values[520] != 0.0)) {
            scratch.store_ad(315, &AdValue::div_from_scalar(1.0, AdValue::sub_from_scalar(1.0, scratch.ad_value(289))));
        }

        if ((((scratch.values[270] != 0.0) && (!(scratch.values[504] != 0.0))) && (!(scratch.values[519] != 0.0))) && (!(scratch.values[520] != 0.0))) {
            scratch.store_ad(315, &AdValue::offset(AdValue::scale(AdValue::offset(scratch.ad_value(288), (scratch.values[129] * scratch.values[40])), scratch.values[136]), scratch.values[130]));
        }

        if ((scratch.values[270] != 0.0) && (!(scratch.values[504] != 0.0))) {
            scratch.store_ad(316, &AdValue::mul(AdValue::add(AdValue::add(AdValue::add(scratch.ad_value(290), scratch.ad_value(291)), scratch.ad_value(298)), scratch.ad_value(313)), scratch.ad_value(315)));
        }

        scratch.values[522] = if (scratch.values[218] == 0.0) { 1.0 } else { 0.0 };

        if ((scratch.values[270] != 0.0) && (scratch.values[522] != 0.0)) {
            scratch.values[317] = 0.0;
            scratch.node_derivatives[317] = [0.0; Instance::NODE_COUNT];
            scratch.branch_derivatives[317] = [0.0; Instance::BRANCH_COUNT];
        }

        scratch.values[523] = if (scratch.values[94] == 0.5) { 1.0 } else { 0.0 };

        if (((scratch.values[270] != 0.0) && (!(scratch.values[522] != 0.0))) && (scratch.values[523] != 0.0)) {
            scratch.store_ad(289, &AdValue::sqrt(AdValue::sub_from_scalar(1.0, AdValue::scale(scratch.ad_value(281), scratch.values[91]))));
        }

        if (((scratch.values[270] != 0.0) && (!(scratch.values[522] != 0.0))) && (!(scratch.values[523] != 0.0))) {
            scratch.store_ad(289, &AdValue::powf(AdValue::sub_from_scalar(1.0, AdValue::scale(scratch.ad_value(281), scratch.values[91])), scratch.values[94]));
        }

        if ((scratch.values[270] != 0.0) && (!(scratch.values[522] != 0.0))) {
            scratch.store_ad(290, &AdValue::scale(scratch.ad_value(280), scratch.values[73]));
        }

        scratch.values[524] = if ((scratch.values[23] == 0.0) && (scratch.values[26] == 0.0)) { 1.0 } else { 0.0 };

        if (((scratch.values[270] != 0.0) && (!(scratch.values[522] != 0.0))) && (scratch.values[524] != 0.0)) {
            scratch.values[291] = 0.0;
            scratch.node_derivatives[291] = [0.0; Instance::NODE_COUNT];
            scratch.branch_derivatives[291] = [0.0; Instance::BRANCH_COUNT];
        }

        if (((scratch.values[270] != 0.0) && (!(scratch.values[522] != 0.0))) && (!(scratch.values[524] != 0.0))) {
            scratch.store_ad(292, &AdValue::sub_from_scalar(scratch.values[79], scratch.ad_value(286)));
        }

        if (((scratch.values[270] != 0.0) && (!(scratch.values[522] != 0.0))) && (!(scratch.values[524] != 0.0))) {
            scratch.store_ad(293, &AdValue::sub_from_scalar(1.0, AdValue::sqrt(AdValue::sub_from_scalar(1.0, AdValue::div(scratch.ad_value(284), scratch.ad_value(292))))));
        }

        scratch.values[525] = if (scratch.values[12] == 0.5) { 1.0 } else { 0.0 };

        if ((((scratch.values[270] != 0.0) && (!(scratch.values[522] != 0.0))) && (!(scratch.values[524] != 0.0))) && (scratch.values[525] != 0.0)) {
            scratch.values[294] = 0.0;
            scratch.node_derivatives[294] = [0.0; Instance::NODE_COUNT];
            scratch.branch_derivatives[294] = [0.0; Instance::BRANCH_COUNT];
        }

        if ((((scratch.values[270] != 0.0) && (!(scratch.values[522] != 0.0))) && (!(scratch.values[524] != 0.0))) && (!(scratch.values[525] != 0.0))) {
            scratch.store_ad(294, &AdValue::scale(AdValue::add(AdValue::div(AdValue::mul(AdValue::square(scratch.ad_value(293)), AdValue::ln(scratch.ad_value(293))), AdValue::sub_from_scalar(1.0, scratch.ad_value(293))), scratch.ad_value(293)), (1.0 - (2.0 * scratch.values[12]))));
        }

        if (((scratch.values[270] != 0.0) && (!(scratch.values[522] != 0.0))) && (!(scratch.values[524] != 0.0))) {
            scratch.store_ad(295, &AdValue::add(scratch.ad_value(293), scratch.ad_value(294)));
        }

        scratch.values[526] = if (scratch.values[12] == 0.5) { 1.0 } else { 0.0 };

        if ((((scratch.values[270] != 0.0) && (!(scratch.values[522] != 0.0))) && (!(scratch.values[524] != 0.0))) && (scratch.values[526] != 0.0)) {
            scratch.store_ad(289, &AdValue::sqrt(AdValue::scale(scratch.ad_value(292), scratch.values[115])));
        }

        if ((((scratch.values[270] != 0.0) && (!(scratch.values[522] != 0.0))) && (!(scratch.values[524] != 0.0))) && (!(scratch.values[526] != 0.0))) {
            scratch.store_ad(289, &AdValue::powf(AdValue::scale(scratch.ad_value(292), scratch.values[115]), scratch.values[12]));
        }

        if (((scratch.values[270] != 0.0) && (!(scratch.values[522] != 0.0))) && (!(scratch.values[524] != 0.0))) {
            scratch.store_ad(296, &AdValue::scale(scratch.ad_value(289), scratch.values[109]));
        }

        if (((scratch.values[270] != 0.0) && (!(scratch.values[522] != 0.0))) && (!(scratch.values[524] != 0.0))) {
            scratch.store_ad(297, &AdValue::scale(AdValue::mul(AdValue::offset(scratch.ad_value(283), (-1.0)), scratch.ad_value(296)), scratch.values[70]));
        }

        if (((scratch.values[270] != 0.0) && (!(scratch.values[522] != 0.0))) && (!(scratch.values[524] != 0.0))) {
            scratch.store_ad(291, &AdValue::scale(AdValue::mul(scratch.ad_value(297), scratch.ad_value(295)), scratch.values[23]));
        }

        scratch.values[527] = if (scratch.values[26] == 0.0) { 1.0 } else { 0.0 };

        if (((scratch.values[270] != 0.0) && (!(scratch.values[522] != 0.0))) && (scratch.values[527] != 0.0)) {
            scratch.values[298] = 0.0;
            scratch.node_derivatives[298] = [0.0; Instance::NODE_COUNT];
            scratch.branch_derivatives[298] = [0.0; Instance::BRANCH_COUNT];
        }

        if (((scratch.values[270] != 0.0) && (!(scratch.values[522] != 0.0))) && (!(scratch.values[527] != 0.0))) {
            scratch.store_ad(299, &AdValue::scale(AdValue::div(AdValue::scale(scratch.ad_value(296), scratch.values[94]), scratch.ad_value(292)), scratch.values[124]));
        }

        if (((scratch.values[270] != 0.0) && (!(scratch.values[522] != 0.0))) && (!(scratch.values[527] != 0.0))) {
            scratch.store_ad(300, &AdValue::div_from_scalar((0.666666666666667 * scratch.values[121]), scratch.ad_value(299)));
        }

        if (((scratch.values[270] != 0.0) && (!(scratch.values[522] != 0.0))) && (!(scratch.values[527] != 0.0))) {
            scratch.store_ad(301, &AdValue::square(scratch.ad_value(300)));
        }

        if (((scratch.values[270] != 0.0) && (!(scratch.values[522] != 0.0))) && (!(scratch.values[527] != 0.0))) {
            scratch.store_ad(302, &AdValue::sqrt(AdValue::div(AdValue::square(scratch.ad_value(301)), AdValue::offset(AdValue::square(scratch.ad_value(301)), 1.0))));
        }

        if (((scratch.values[270] != 0.0) && (!(scratch.values[522] != 0.0))) && (!(scratch.values[527] != 0.0))) {
            scratch.store_ad(303, &AdValue::sqrt(AdValue::abs(scratch.ad_value(302))));
        }

        if (((scratch.values[270] != 0.0) && (!(scratch.values[522] != 0.0))) && (!(scratch.values[527] != 0.0))) {
            scratch.store_ad(304, &AdValue::mul(scratch.ad_value(302), scratch.ad_value(303)));
        }

    }
}
