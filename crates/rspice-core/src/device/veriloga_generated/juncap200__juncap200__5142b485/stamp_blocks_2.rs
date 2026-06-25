#![allow(dead_code, unused_assignments, unused_imports, unused_parens, unused_variables)]

use super::{AdValue, GeneratedEvalContext, GeneratedReactiveStamper, GeneratedStamper, Scratch, THERMAL_VOLTAGE_PER_K};
use super::super::state::Instance;

impl Instance {

    pub(super) fn stamp_transient_block_8(
        &mut self,
        ctx: &GeneratedEvalContext<'_>,
        stamper: &mut GeneratedStamper<'_>,
        scratch: &mut Scratch,
    ) {
        let _ = stamper;
        scratch.values[528] = if (((-scratch.values[12]) * scratch.values[97]) == (-1.0)) { 1.0 } else { 0.0 };

        if ((((scratch.values[270] != 0.0) && (!(scratch.values[522] != 0.0))) && (!(scratch.values[527] != 0.0))) && (scratch.values[528] != 0.0)) {
            scratch.store_ad(305, &AdValue::div_from_scalar(1.0, AdValue::offset(AdValue::mul(scratch.ad_value(299), scratch.ad_value(304)), 1.0)));
        }

        if ((((scratch.values[270] != 0.0) && (!(scratch.values[522] != 0.0))) && (!(scratch.values[527] != 0.0))) && (!(scratch.values[528] != 0.0))) {
            scratch.store_ad(305, &AdValue::powf(AdValue::offset(AdValue::mul(scratch.ad_value(299), scratch.ad_value(304)), 1.0), ((-scratch.values[12]) * scratch.values[97])));
        }

        if (((scratch.values[270] != 0.0) && (!(scratch.values[522] != 0.0))) && (!(scratch.values[527] != 0.0))) {
            scratch.store_ad(306, &AdValue::div(AdValue::mul(scratch.ad_value(295), scratch.ad_value(305)), AdValue::add(scratch.ad_value(295), scratch.ad_value(305))));
        }

        if (((scratch.values[270] != 0.0) && (!(scratch.values[522] != 0.0))) && (!(scratch.values[527] != 0.0))) {
            scratch.store_ad(307, &AdValue::sqrt(AdValue::scale(AdValue::div(scratch.ad_value(299), scratch.ad_value(303)), 0.375)));
        }

        if (((scratch.values[270] != 0.0) && (!(scratch.values[522] != 0.0))) && (!(scratch.values[527] != 0.0))) {
            scratch.store_ad(308, &AdValue::sub(AdValue::scale(AdValue::mul(scratch.ad_value(300), scratch.ad_value(303)), 2.0), scratch.ad_value(302)));
        }

        if (((scratch.values[270] != 0.0) && (!(scratch.values[522] != 0.0))) && (!(scratch.values[527] != 0.0))) {
            scratch.store_ad(309, &AdValue::add(AdValue::sub(AdValue::mul(AdValue::scale(scratch.ad_value(300), scratch.values[121]), scratch.ad_value(303)), AdValue::scale(scratch.ad_value(302), scratch.values[121])), AdValue::scale(AdValue::mul(scratch.ad_value(299), scratch.ad_value(304)), 0.5)));
        }

        if (((scratch.values[270] != 0.0) && (!(scratch.values[522] != 0.0))) && (!(scratch.values[527] != 0.0))) {
            scratch.store_ad(310, &AdValue::mul(AdValue::offset(scratch.ad_value(308), (-1.0)), scratch.ad_value(307)));
        }

        if (((scratch.values[270] != 0.0) && (!(scratch.values[522] != 0.0))) && (!(scratch.values[527] != 0.0))) {
            scratch.store_ad(271, &AdValue::square(scratch.ad_value(310)));
        }

        scratch.values[529] = if (scratch.values[310] > 0.0) { 1.0 } else { 0.0 };

        if ((((scratch.values[270] != 0.0) && (!(scratch.values[522] != 0.0))) && (!(scratch.values[527] != 0.0))) && (scratch.values[529] != 0.0)) {
            scratch.store_ad(272, &AdValue::div_from_scalar(1.0, AdValue::offset(AdValue::scale(scratch.ad_value(310), scratch.values[57]), 1.0)));
        }

        if ((((scratch.values[270] != 0.0) && (!(scratch.values[522] != 0.0))) && (!(scratch.values[527] != 0.0))) && (!(scratch.values[529] != 0.0))) {
            scratch.store_ad(272, &AdValue::div_from_scalar(1.0, AdValue::sub_from_scalar(1.0, AdValue::scale(scratch.ad_value(310), scratch.values[57]))));
        }

        scratch.values[530] = if (((-scratch.values[271]) + scratch.values[309]) > (-230.25850929940458)) { 1.0 } else { 0.0 };

        if ((((scratch.values[270] != 0.0) && (!(scratch.values[522] != 0.0))) && (!(scratch.values[527] != 0.0))) && (scratch.values[530] != 0.0)) {
            scratch.store_ad(289, &AdValue::exp(AdValue::sub(scratch.ad_value(309), scratch.ad_value(271))));
        }

        if ((((scratch.values[270] != 0.0) && (!(scratch.values[522] != 0.0))) && (!(scratch.values[527] != 0.0))) && (!(scratch.values[530] != 0.0))) {
            scratch.store_ad(289, &AdValue::div_from_scalar(1e-100, AdValue::offset(AdValue::mul(AdValue::sub_from_scalar((-230.25850929940458), AdValue::sub(scratch.ad_value(309), scratch.ad_value(271))), AdValue::offset(AdValue::scale(AdValue::mul(AdValue::sub_from_scalar((-230.25850929940458), AdValue::sub(scratch.ad_value(309), scratch.ad_value(271))), AdValue::offset(AdValue::scale(AdValue::sub_from_scalar((-230.25850929940458), AdValue::sub(scratch.ad_value(309), scratch.ad_value(271))), 0.3333333333333333), 1.0)), 0.5), 1.0)), 1.0)));
        }

        if (((scratch.values[270] != 0.0) && (!(scratch.values[522] != 0.0))) && (!(scratch.values[527] != 0.0))) {
            scratch.store_ad(273, &AdValue::mul(AdValue::add(AdValue::add(AdValue::scale(scratch.ad_value(272), 0.29214664), AdValue::scale(AdValue::square(scratch.ad_value(272)), scratch.values[58])), AdValue::scale(AdValue::mul(AdValue::square(scratch.ad_value(272)), scratch.ad_value(272)), scratch.values[59])), scratch.ad_value(289)));
        }

        scratch.values[531] = if (scratch.values[310] > 0.0) { 1.0 } else { 0.0 };

        if ((((scratch.values[270] != 0.0) && (!(scratch.values[522] != 0.0))) && (!(scratch.values[527] != 0.0))) && (scratch.values[531] != 0.0)) {
            scratch.values[311] = scratch.values[273];
            scratch.node_derivatives[311] = scratch.node_derivatives[273];
            scratch.branch_derivatives[311] = scratch.branch_derivatives[273];
        }

        scratch.values[532] = if (scratch.values[309] > (-230.25850929940458)) { 1.0 } else { 0.0 };

        if (((((scratch.values[270] != 0.0) && (!(scratch.values[522] != 0.0))) && (!(scratch.values[527] != 0.0))) && (!(scratch.values[531] != 0.0))) && (scratch.values[532] != 0.0)) {
            scratch.store_ad(289, &AdValue::exp(scratch.ad_value(309)));
        }

        if (((((scratch.values[270] != 0.0) && (!(scratch.values[522] != 0.0))) && (!(scratch.values[527] != 0.0))) && (!(scratch.values[531] != 0.0))) && (!(scratch.values[532] != 0.0))) {
            scratch.store_ad(289, &AdValue::div_from_scalar(1e-100, AdValue::offset(AdValue::mul(AdValue::sub_from_scalar((-230.25850929940458), scratch.ad_value(309)), AdValue::offset(AdValue::scale(AdValue::mul(AdValue::sub_from_scalar((-230.25850929940458), scratch.ad_value(309)), AdValue::offset(AdValue::scale(AdValue::sub_from_scalar((-230.25850929940458), scratch.ad_value(309)), 0.3333333333333333), 1.0)), 0.5), 1.0)), 1.0)));
        }

        if ((((scratch.values[270] != 0.0) && (!(scratch.values[522] != 0.0))) && (!(scratch.values[527] != 0.0))) && (!(scratch.values[531] != 0.0))) {
            scratch.store_ad(311, &AdValue::sub(AdValue::scale(scratch.ad_value(289), 2.0), scratch.ad_value(273)));
        }

        if (((scratch.values[270] != 0.0) && (!(scratch.values[522] != 0.0))) && (!(scratch.values[527] != 0.0))) {
            scratch.store_ad(312, &AdValue::scale(AdValue::div(AdValue::scale(scratch.ad_value(311), scratch.values[121]), scratch.ad_value(307)), (1.772453850905516 * 0.5)));
        }

        if (((scratch.values[270] != 0.0) && (!(scratch.values[522] != 0.0))) && (!(scratch.values[527] != 0.0))) {
            scratch.store_ad(298, &AdValue::scale(AdValue::mul(AdValue::mul(scratch.ad_value(297), scratch.ad_value(312)), scratch.ad_value(306)), scratch.values[26]));
        }

        scratch.values[533] = if (scratch.values[32] == 0.0) { 1.0 } else { 0.0 };

        if (((scratch.values[270] != 0.0) && (!(scratch.values[522] != 0.0))) && (scratch.values[533] != 0.0)) {
            scratch.values[313] = 0.0;
            scratch.node_derivatives[313] = [0.0; Instance::NODE_COUNT];
            scratch.branch_derivatives[313] = [0.0; Instance::BRANCH_COUNT];
        }

        scratch.values[534] = if (scratch.values[12] == 0.5) { 1.0 } else { 0.0 };

        if ((((scratch.values[270] != 0.0) && (!(scratch.values[522] != 0.0))) && (!(scratch.values[533] != 0.0))) && (scratch.values[534] != 0.0)) {
            scratch.store_ad(289, &AdValue::sqrt(AdValue::scale(AdValue::sub_from_scalar(scratch.values[9], scratch.ad_value(287)), scratch.values[115])));
        }

        if ((((scratch.values[270] != 0.0) && (!(scratch.values[522] != 0.0))) && (!(scratch.values[533] != 0.0))) && (!(scratch.values[534] != 0.0))) {
            scratch.store_ad(289, &AdValue::powf(AdValue::scale(AdValue::sub_from_scalar(scratch.values[9], scratch.ad_value(287)), scratch.values[115]), scratch.values[12]));
        }

        if (((scratch.values[270] != 0.0) && (!(scratch.values[522] != 0.0))) && (!(scratch.values[533] != 0.0))) {
            scratch.store_ad(314, &AdValue::scale(AdValue::div(AdValue::scale(AdValue::sub_from_scalar(scratch.values[9], scratch.ad_value(287)), scratch.values[112]), scratch.ad_value(289)), scratch.values[97]));
        }

        scratch.values[535] = if (((((-scratch.values[127]) / scratch.values[314])) as f64).abs() < 230.25850929940458) { 1.0 } else { 0.0 };

        if ((((scratch.values[270] != 0.0) && (!(scratch.values[522] != 0.0))) && (!(scratch.values[533] != 0.0))) && (scratch.values[535] != 0.0)) {
            scratch.store_ad(289, &AdValue::exp(AdValue::div(AdValue::neg(scratch.ad_value(127)), scratch.ad_value(314))));
        }

        scratch.values[536] = if (((-scratch.values[127]) / scratch.values[314]) < 0.0) { 1.0 } else { 0.0 };

        if (((((scratch.values[270] != 0.0) && (!(scratch.values[522] != 0.0))) && (!(scratch.values[533] != 0.0))) && (!(scratch.values[535] != 0.0))) && (scratch.values[536] != 0.0)) {
            let assign11930_ad_e14946: AdValue = AdValue::offset(AdValue::mul(AdValue::sub_from_scalar((-230.25850929940458), AdValue::div(AdValue::neg(scratch.ad_value(127)), scratch.ad_value(314))), AdValue::offset(AdValue::scale(AdValue::mul(AdValue::sub_from_scalar((-230.25850929940458), AdValue::div(AdValue::neg(scratch.ad_value(127)), scratch.ad_value(314))), AdValue::offset(AdValue::scale(AdValue::sub_from_scalar((-230.25850929940458), AdValue::div(AdValue::neg(scratch.ad_value(127)), scratch.ad_value(314))), 0.3333333333333333), 1.0)), 0.5), 1.0)), 1.0);
            scratch.store_ad(289, &AdValue::div_from_scalar(1e-100, assign11930_ad_e14946));
        }

        if (((((scratch.values[270] != 0.0) && (!(scratch.values[522] != 0.0))) && (!(scratch.values[533] != 0.0))) && (!(scratch.values[535] != 0.0))) && (!(scratch.values[536] != 0.0))) {
            let assign11940_ad_e14994: AdValue = AdValue::scale(AdValue::offset(AdValue::mul(AdValue::offset(AdValue::div(AdValue::neg(scratch.ad_value(127)), scratch.ad_value(314)), (-230.25850929940458)), AdValue::offset(AdValue::scale(AdValue::mul(AdValue::offset(AdValue::div(AdValue::neg(scratch.ad_value(127)), scratch.ad_value(314)), (-230.25850929940458)), AdValue::offset(AdValue::scale(AdValue::offset(AdValue::div(AdValue::neg(scratch.ad_value(127)), scratch.ad_value(314)), (-230.25850929940458)), 0.3333333333333333), 1.0)), 0.5), 1.0)), 1.0), 1e100);
            scratch.store_ad(289, &assign11940_ad_e14994);
        }

        if (((scratch.values[270] != 0.0) && (!(scratch.values[522] != 0.0))) && (!(scratch.values[533] != 0.0))) {
            scratch.store_ad(313, &AdValue::scale(AdValue::mul(AdValue::mul(AdValue::mul(scratch.ad_value(155), scratch.ad_value(314)), scratch.ad_value(314)), scratch.ad_value(289)), scratch.values[32]));
        }

        scratch.values[537] = if (scratch.values[41] > 1000.0) { 1.0 } else { 0.0 };

        if (((scratch.values[270] != 0.0) && (!(scratch.values[522] != 0.0))) && (scratch.values[537] != 0.0)) {
            scratch.values[315] = 1.0;
            scratch.node_derivatives[315] = [0.0; Instance::NODE_COUNT];
            scratch.branch_derivatives[315] = [0.0; Instance::BRANCH_COUNT];
        }

        scratch.values[538] = if (scratch.values[288] > ((-scratch.values[129]) * scratch.values[41])) { 1.0 } else { 0.0 };

        scratch.values[539] = if (scratch.values[44] == 4.0) { 1.0 } else { 0.0 };

        if (((((scratch.values[270] != 0.0) && (!(scratch.values[522] != 0.0))) && (!(scratch.values[537] != 0.0))) && (scratch.values[538] != 0.0)) && (scratch.values[539] != 0.0)) {
            scratch.store_ad(289, &AdValue::mul(AdValue::mul(AdValue::mul(AdValue::scale(scratch.ad_value(288), scratch.values[134]), AdValue::scale(scratch.ad_value(288), scratch.values[134])), AdValue::scale(scratch.ad_value(288), scratch.values[134])), AdValue::scale(scratch.ad_value(288), scratch.values[134])));
        }

        if (((((scratch.values[270] != 0.0) && (!(scratch.values[522] != 0.0))) && (!(scratch.values[537] != 0.0))) && (scratch.values[538] != 0.0)) && (!(scratch.values[539] != 0.0))) {
            scratch.store_ad(289, &AdValue::powf(AdValue::abs(AdValue::scale(scratch.ad_value(288), scratch.values[134])), scratch.values[44]));
        }

        if ((((scratch.values[270] != 0.0) && (!(scratch.values[522] != 0.0))) && (!(scratch.values[537] != 0.0))) && (scratch.values[538] != 0.0)) {
            scratch.store_ad(315, &AdValue::div_from_scalar(1.0, AdValue::sub_from_scalar(1.0, scratch.ad_value(289))));
        }

        if ((((scratch.values[270] != 0.0) && (!(scratch.values[522] != 0.0))) && (!(scratch.values[537] != 0.0))) && (!(scratch.values[538] != 0.0))) {
            scratch.store_ad(315, &AdValue::offset(AdValue::scale(AdValue::offset(scratch.ad_value(288), (scratch.values[129] * scratch.values[41])), scratch.values[137]), scratch.values[131]));
        }

        if ((scratch.values[270] != 0.0) && (!(scratch.values[522] != 0.0))) {
            scratch.store_ad(317, &AdValue::mul(AdValue::add(AdValue::add(AdValue::add(scratch.ad_value(290), scratch.ad_value(291)), scratch.ad_value(298)), scratch.ad_value(313)), scratch.ad_value(315)));
        }

        scratch.values[540] = if (scratch.values[219] == 0.0) { 1.0 } else { 0.0 };

        if ((scratch.values[270] != 0.0) && (scratch.values[540] != 0.0)) {
            scratch.values[318] = 0.0;
            scratch.node_derivatives[318] = [0.0; Instance::NODE_COUNT];
            scratch.branch_derivatives[318] = [0.0; Instance::BRANCH_COUNT];
        }

        scratch.values[541] = if (scratch.values[95] == 0.5) { 1.0 } else { 0.0 };

        if (((scratch.values[270] != 0.0) && (!(scratch.values[540] != 0.0))) && (scratch.values[541] != 0.0)) {
            scratch.store_ad(289, &AdValue::sqrt(AdValue::sub_from_scalar(1.0, AdValue::scale(scratch.ad_value(281), scratch.values[92]))));
        }

        if (((scratch.values[270] != 0.0) && (!(scratch.values[540] != 0.0))) && (!(scratch.values[541] != 0.0))) {
            scratch.store_ad(289, &AdValue::powf(AdValue::sub_from_scalar(1.0, AdValue::scale(scratch.ad_value(281), scratch.values[92])), scratch.values[95]));
        }

        if ((scratch.values[270] != 0.0) && (!(scratch.values[540] != 0.0))) {
            scratch.store_ad(290, &AdValue::scale(scratch.ad_value(280), scratch.values[74]));
        }

        scratch.values[542] = if ((scratch.values[24] == 0.0) && (scratch.values[27] == 0.0)) { 1.0 } else { 0.0 };

        if (((scratch.values[270] != 0.0) && (!(scratch.values[540] != 0.0))) && (scratch.values[542] != 0.0)) {
            scratch.values[291] = 0.0;
            scratch.node_derivatives[291] = [0.0; Instance::NODE_COUNT];
            scratch.branch_derivatives[291] = [0.0; Instance::BRANCH_COUNT];
        }

        if (((scratch.values[270] != 0.0) && (!(scratch.values[540] != 0.0))) && (!(scratch.values[542] != 0.0))) {
            scratch.store_ad(292, &AdValue::sub_from_scalar(scratch.values[80], scratch.ad_value(286)));
        }

        if (((scratch.values[270] != 0.0) && (!(scratch.values[540] != 0.0))) && (!(scratch.values[542] != 0.0))) {
            scratch.store_ad(293, &AdValue::sub_from_scalar(1.0, AdValue::sqrt(AdValue::sub_from_scalar(1.0, AdValue::div(scratch.ad_value(284), scratch.ad_value(292))))));
        }

        scratch.values[543] = if (scratch.values[13] == 0.5) { 1.0 } else { 0.0 };

        if ((((scratch.values[270] != 0.0) && (!(scratch.values[540] != 0.0))) && (!(scratch.values[542] != 0.0))) && (scratch.values[543] != 0.0)) {
            scratch.values[294] = 0.0;
            scratch.node_derivatives[294] = [0.0; Instance::NODE_COUNT];
            scratch.branch_derivatives[294] = [0.0; Instance::BRANCH_COUNT];
        }

        if ((((scratch.values[270] != 0.0) && (!(scratch.values[540] != 0.0))) && (!(scratch.values[542] != 0.0))) && (!(scratch.values[543] != 0.0))) {
            scratch.store_ad(294, &AdValue::scale(AdValue::add(AdValue::div(AdValue::mul(AdValue::square(scratch.ad_value(293)), AdValue::ln(scratch.ad_value(293))), AdValue::sub_from_scalar(1.0, scratch.ad_value(293))), scratch.ad_value(293)), (1.0 - (2.0 * scratch.values[13]))));
        }

        if (((scratch.values[270] != 0.0) && (!(scratch.values[540] != 0.0))) && (!(scratch.values[542] != 0.0))) {
            scratch.store_ad(295, &AdValue::add(scratch.ad_value(293), scratch.ad_value(294)));
        }

        scratch.values[544] = if (scratch.values[13] == 0.5) { 1.0 } else { 0.0 };

        if ((((scratch.values[270] != 0.0) && (!(scratch.values[540] != 0.0))) && (!(scratch.values[542] != 0.0))) && (scratch.values[544] != 0.0)) {
            scratch.store_ad(289, &AdValue::sqrt(AdValue::scale(scratch.ad_value(292), scratch.values[116])));
        }

        if ((((scratch.values[270] != 0.0) && (!(scratch.values[540] != 0.0))) && (!(scratch.values[542] != 0.0))) && (!(scratch.values[544] != 0.0))) {
            scratch.store_ad(289, &AdValue::powf(AdValue::scale(scratch.ad_value(292), scratch.values[116]), scratch.values[13]));
        }

        if (((scratch.values[270] != 0.0) && (!(scratch.values[540] != 0.0))) && (!(scratch.values[542] != 0.0))) {
            scratch.store_ad(296, &AdValue::scale(scratch.ad_value(289), scratch.values[110]));
        }

        if (((scratch.values[270] != 0.0) && (!(scratch.values[540] != 0.0))) && (!(scratch.values[542] != 0.0))) {
            scratch.store_ad(297, &AdValue::scale(AdValue::mul(AdValue::offset(scratch.ad_value(283), (-1.0)), scratch.ad_value(296)), scratch.values[71]));
        }

        if (((scratch.values[270] != 0.0) && (!(scratch.values[540] != 0.0))) && (!(scratch.values[542] != 0.0))) {
            scratch.store_ad(291, &AdValue::scale(AdValue::mul(scratch.ad_value(297), scratch.ad_value(295)), scratch.values[24]));
        }

        scratch.values[545] = if (scratch.values[27] == 0.0) { 1.0 } else { 0.0 };

        if (((scratch.values[270] != 0.0) && (!(scratch.values[540] != 0.0))) && (scratch.values[545] != 0.0)) {
            scratch.values[298] = 0.0;
            scratch.node_derivatives[298] = [0.0; Instance::NODE_COUNT];
            scratch.branch_derivatives[298] = [0.0; Instance::BRANCH_COUNT];
        }

        if (((scratch.values[270] != 0.0) && (!(scratch.values[540] != 0.0))) && (!(scratch.values[545] != 0.0))) {
            scratch.store_ad(299, &AdValue::scale(AdValue::div(AdValue::scale(scratch.ad_value(296), scratch.values[95]), scratch.ad_value(292)), scratch.values[125]));
        }

        if (((scratch.values[270] != 0.0) && (!(scratch.values[540] != 0.0))) && (!(scratch.values[545] != 0.0))) {
            scratch.store_ad(300, &AdValue::div_from_scalar((0.666666666666667 * scratch.values[122]), scratch.ad_value(299)));
        }

        if (((scratch.values[270] != 0.0) && (!(scratch.values[540] != 0.0))) && (!(scratch.values[545] != 0.0))) {
            scratch.store_ad(301, &AdValue::square(scratch.ad_value(300)));
        }

        if (((scratch.values[270] != 0.0) && (!(scratch.values[540] != 0.0))) && (!(scratch.values[545] != 0.0))) {
            scratch.store_ad(302, &AdValue::sqrt(AdValue::div(AdValue::square(scratch.ad_value(301)), AdValue::offset(AdValue::square(scratch.ad_value(301)), 1.0))));
        }

        if (((scratch.values[270] != 0.0) && (!(scratch.values[540] != 0.0))) && (!(scratch.values[545] != 0.0))) {
            scratch.store_ad(303, &AdValue::sqrt(AdValue::abs(scratch.ad_value(302))));
        }

        if (((scratch.values[270] != 0.0) && (!(scratch.values[540] != 0.0))) && (!(scratch.values[545] != 0.0))) {
            scratch.store_ad(304, &AdValue::mul(scratch.ad_value(302), scratch.ad_value(303)));
        }

        scratch.values[546] = if (((-scratch.values[13]) * scratch.values[98]) == (-1.0)) { 1.0 } else { 0.0 };

        if ((((scratch.values[270] != 0.0) && (!(scratch.values[540] != 0.0))) && (!(scratch.values[545] != 0.0))) && (scratch.values[546] != 0.0)) {
            scratch.store_ad(305, &AdValue::div_from_scalar(1.0, AdValue::offset(AdValue::mul(scratch.ad_value(299), scratch.ad_value(304)), 1.0)));
        }

        if ((((scratch.values[270] != 0.0) && (!(scratch.values[540] != 0.0))) && (!(scratch.values[545] != 0.0))) && (!(scratch.values[546] != 0.0))) {
            scratch.store_ad(305, &AdValue::powf(AdValue::offset(AdValue::mul(scratch.ad_value(299), scratch.ad_value(304)), 1.0), ((-scratch.values[13]) * scratch.values[98])));
        }

        if (((scratch.values[270] != 0.0) && (!(scratch.values[540] != 0.0))) && (!(scratch.values[545] != 0.0))) {
            scratch.store_ad(306, &AdValue::div(AdValue::mul(scratch.ad_value(295), scratch.ad_value(305)), AdValue::add(scratch.ad_value(295), scratch.ad_value(305))));
        }

        if (((scratch.values[270] != 0.0) && (!(scratch.values[540] != 0.0))) && (!(scratch.values[545] != 0.0))) {
            scratch.store_ad(307, &AdValue::sqrt(AdValue::scale(AdValue::div(scratch.ad_value(299), scratch.ad_value(303)), 0.375)));
        }

        if (((scratch.values[270] != 0.0) && (!(scratch.values[540] != 0.0))) && (!(scratch.values[545] != 0.0))) {
            scratch.store_ad(308, &AdValue::sub(AdValue::scale(AdValue::mul(scratch.ad_value(300), scratch.ad_value(303)), 2.0), scratch.ad_value(302)));
        }

        if (((scratch.values[270] != 0.0) && (!(scratch.values[540] != 0.0))) && (!(scratch.values[545] != 0.0))) {
            scratch.store_ad(309, &AdValue::add(AdValue::sub(AdValue::mul(AdValue::scale(scratch.ad_value(300), scratch.values[122]), scratch.ad_value(303)), AdValue::scale(scratch.ad_value(302), scratch.values[122])), AdValue::scale(AdValue::mul(scratch.ad_value(299), scratch.ad_value(304)), 0.5)));
        }

        if (((scratch.values[270] != 0.0) && (!(scratch.values[540] != 0.0))) && (!(scratch.values[545] != 0.0))) {
            scratch.store_ad(310, &AdValue::mul(AdValue::offset(scratch.ad_value(308), (-1.0)), scratch.ad_value(307)));
        }

        if (((scratch.values[270] != 0.0) && (!(scratch.values[540] != 0.0))) && (!(scratch.values[545] != 0.0))) {
            scratch.store_ad(271, &AdValue::square(scratch.ad_value(310)));
        }

        scratch.values[547] = if (scratch.values[310] > 0.0) { 1.0 } else { 0.0 };

        if ((((scratch.values[270] != 0.0) && (!(scratch.values[540] != 0.0))) && (!(scratch.values[545] != 0.0))) && (scratch.values[547] != 0.0)) {
            scratch.store_ad(272, &AdValue::div_from_scalar(1.0, AdValue::offset(AdValue::scale(scratch.ad_value(310), scratch.values[57]), 1.0)));
        }

        if ((((scratch.values[270] != 0.0) && (!(scratch.values[540] != 0.0))) && (!(scratch.values[545] != 0.0))) && (!(scratch.values[547] != 0.0))) {
            scratch.store_ad(272, &AdValue::div_from_scalar(1.0, AdValue::sub_from_scalar(1.0, AdValue::scale(scratch.ad_value(310), scratch.values[57]))));
        }

        scratch.values[548] = if (((-scratch.values[271]) + scratch.values[309]) > (-230.25850929940458)) { 1.0 } else { 0.0 };

        if ((((scratch.values[270] != 0.0) && (!(scratch.values[540] != 0.0))) && (!(scratch.values[545] != 0.0))) && (scratch.values[548] != 0.0)) {
            scratch.store_ad(289, &AdValue::exp(AdValue::sub(scratch.ad_value(309), scratch.ad_value(271))));
        }

        if ((((scratch.values[270] != 0.0) && (!(scratch.values[540] != 0.0))) && (!(scratch.values[545] != 0.0))) && (!(scratch.values[548] != 0.0))) {
            scratch.store_ad(289, &AdValue::div_from_scalar(1e-100, AdValue::offset(AdValue::mul(AdValue::sub_from_scalar((-230.25850929940458), AdValue::sub(scratch.ad_value(309), scratch.ad_value(271))), AdValue::offset(AdValue::scale(AdValue::mul(AdValue::sub_from_scalar((-230.25850929940458), AdValue::sub(scratch.ad_value(309), scratch.ad_value(271))), AdValue::offset(AdValue::scale(AdValue::sub_from_scalar((-230.25850929940458), AdValue::sub(scratch.ad_value(309), scratch.ad_value(271))), 0.3333333333333333), 1.0)), 0.5), 1.0)), 1.0)));
        }

        if (((scratch.values[270] != 0.0) && (!(scratch.values[540] != 0.0))) && (!(scratch.values[545] != 0.0))) {
            scratch.store_ad(273, &AdValue::mul(AdValue::add(AdValue::add(AdValue::scale(scratch.ad_value(272), 0.29214664), AdValue::scale(AdValue::square(scratch.ad_value(272)), scratch.values[58])), AdValue::scale(AdValue::mul(AdValue::square(scratch.ad_value(272)), scratch.ad_value(272)), scratch.values[59])), scratch.ad_value(289)));
        }

        scratch.values[549] = if (scratch.values[310] > 0.0) { 1.0 } else { 0.0 };

        if ((((scratch.values[270] != 0.0) && (!(scratch.values[540] != 0.0))) && (!(scratch.values[545] != 0.0))) && (scratch.values[549] != 0.0)) {
            scratch.values[311] = scratch.values[273];
            scratch.node_derivatives[311] = scratch.node_derivatives[273];
            scratch.branch_derivatives[311] = scratch.branch_derivatives[273];
        }

        scratch.values[550] = if (scratch.values[309] > (-230.25850929940458)) { 1.0 } else { 0.0 };

        if (((((scratch.values[270] != 0.0) && (!(scratch.values[540] != 0.0))) && (!(scratch.values[545] != 0.0))) && (!(scratch.values[549] != 0.0))) && (scratch.values[550] != 0.0)) {
            scratch.store_ad(289, &AdValue::exp(scratch.ad_value(309)));
        }

        if (((((scratch.values[270] != 0.0) && (!(scratch.values[540] != 0.0))) && (!(scratch.values[545] != 0.0))) && (!(scratch.values[549] != 0.0))) && (!(scratch.values[550] != 0.0))) {
            scratch.store_ad(289, &AdValue::div_from_scalar(1e-100, AdValue::offset(AdValue::mul(AdValue::sub_from_scalar((-230.25850929940458), scratch.ad_value(309)), AdValue::offset(AdValue::scale(AdValue::mul(AdValue::sub_from_scalar((-230.25850929940458), scratch.ad_value(309)), AdValue::offset(AdValue::scale(AdValue::sub_from_scalar((-230.25850929940458), scratch.ad_value(309)), 0.3333333333333333), 1.0)), 0.5), 1.0)), 1.0)));
        }

        if ((((scratch.values[270] != 0.0) && (!(scratch.values[540] != 0.0))) && (!(scratch.values[545] != 0.0))) && (!(scratch.values[549] != 0.0))) {
            scratch.store_ad(311, &AdValue::sub(AdValue::scale(scratch.ad_value(289), 2.0), scratch.ad_value(273)));
        }

        if (((scratch.values[270] != 0.0) && (!(scratch.values[540] != 0.0))) && (!(scratch.values[545] != 0.0))) {
            scratch.store_ad(312, &AdValue::scale(AdValue::div(AdValue::scale(scratch.ad_value(311), scratch.values[122]), scratch.ad_value(307)), (1.772453850905516 * 0.5)));
        }

        if (((scratch.values[270] != 0.0) && (!(scratch.values[540] != 0.0))) && (!(scratch.values[545] != 0.0))) {
            scratch.store_ad(298, &AdValue::scale(AdValue::mul(AdValue::mul(scratch.ad_value(297), scratch.ad_value(312)), scratch.ad_value(306)), scratch.values[27]));
        }

        scratch.values[551] = if (scratch.values[33] == 0.0) { 1.0 } else { 0.0 };

        if (((scratch.values[270] != 0.0) && (!(scratch.values[540] != 0.0))) && (scratch.values[551] != 0.0)) {
            scratch.values[313] = 0.0;
            scratch.node_derivatives[313] = [0.0; Instance::NODE_COUNT];
            scratch.branch_derivatives[313] = [0.0; Instance::BRANCH_COUNT];
        }

        scratch.values[552] = if (scratch.values[13] == 0.5) { 1.0 } else { 0.0 };

        if ((((scratch.values[270] != 0.0) && (!(scratch.values[540] != 0.0))) && (!(scratch.values[551] != 0.0))) && (scratch.values[552] != 0.0)) {
            scratch.store_ad(289, &AdValue::sqrt(AdValue::scale(AdValue::sub_from_scalar(scratch.values[10], scratch.ad_value(287)), scratch.values[116])));
        }

        if ((((scratch.values[270] != 0.0) && (!(scratch.values[540] != 0.0))) && (!(scratch.values[551] != 0.0))) && (!(scratch.values[552] != 0.0))) {
            scratch.store_ad(289, &AdValue::powf(AdValue::scale(AdValue::sub_from_scalar(scratch.values[10], scratch.ad_value(287)), scratch.values[116]), scratch.values[13]));
        }

        if (((scratch.values[270] != 0.0) && (!(scratch.values[540] != 0.0))) && (!(scratch.values[551] != 0.0))) {
            scratch.store_ad(314, &AdValue::scale(AdValue::div(AdValue::scale(AdValue::sub_from_scalar(scratch.values[10], scratch.ad_value(287)), scratch.values[113]), scratch.ad_value(289)), scratch.values[98]));
        }

        scratch.values[553] = if (((((-scratch.values[128]) / scratch.values[314])) as f64).abs() < 230.25850929940458) { 1.0 } else { 0.0 };

        if ((((scratch.values[270] != 0.0) && (!(scratch.values[540] != 0.0))) && (!(scratch.values[551] != 0.0))) && (scratch.values[553] != 0.0)) {
            scratch.store_ad(289, &AdValue::exp(AdValue::div(AdValue::neg(scratch.ad_value(128)), scratch.ad_value(314))));
        }

        scratch.values[554] = if (((-scratch.values[128]) / scratch.values[314]) < 0.0) { 1.0 } else { 0.0 };

        if (((((scratch.values[270] != 0.0) && (!(scratch.values[540] != 0.0))) && (!(scratch.values[551] != 0.0))) && (!(scratch.values[553] != 0.0))) && (scratch.values[554] != 0.0)) {
            let assign12680_ad_e16038: AdValue = AdValue::offset(AdValue::mul(AdValue::sub_from_scalar((-230.25850929940458), AdValue::div(AdValue::neg(scratch.ad_value(128)), scratch.ad_value(314))), AdValue::offset(AdValue::scale(AdValue::mul(AdValue::sub_from_scalar((-230.25850929940458), AdValue::div(AdValue::neg(scratch.ad_value(128)), scratch.ad_value(314))), AdValue::offset(AdValue::scale(AdValue::sub_from_scalar((-230.25850929940458), AdValue::div(AdValue::neg(scratch.ad_value(128)), scratch.ad_value(314))), 0.3333333333333333), 1.0)), 0.5), 1.0)), 1.0);
            scratch.store_ad(289, &AdValue::div_from_scalar(1e-100, assign12680_ad_e16038));
        }

        if (((((scratch.values[270] != 0.0) && (!(scratch.values[540] != 0.0))) && (!(scratch.values[551] != 0.0))) && (!(scratch.values[553] != 0.0))) && (!(scratch.values[554] != 0.0))) {
            let assign12690_ad_e16086: AdValue = AdValue::scale(AdValue::offset(AdValue::mul(AdValue::offset(AdValue::div(AdValue::neg(scratch.ad_value(128)), scratch.ad_value(314)), (-230.25850929940458)), AdValue::offset(AdValue::scale(AdValue::mul(AdValue::offset(AdValue::div(AdValue::neg(scratch.ad_value(128)), scratch.ad_value(314)), (-230.25850929940458)), AdValue::offset(AdValue::scale(AdValue::offset(AdValue::div(AdValue::neg(scratch.ad_value(128)), scratch.ad_value(314)), (-230.25850929940458)), 0.3333333333333333), 1.0)), 0.5), 1.0)), 1.0), 1e100);
            scratch.store_ad(289, &assign12690_ad_e16086);
        }

        if (((scratch.values[270] != 0.0) && (!(scratch.values[540] != 0.0))) && (!(scratch.values[551] != 0.0))) {
            scratch.store_ad(313, &AdValue::scale(AdValue::mul(AdValue::mul(AdValue::mul(scratch.ad_value(155), scratch.ad_value(314)), scratch.ad_value(314)), scratch.ad_value(289)), scratch.values[33]));
        }

        scratch.values[555] = if (scratch.values[42] > 1000.0) { 1.0 } else { 0.0 };

        if (((scratch.values[270] != 0.0) && (!(scratch.values[540] != 0.0))) && (scratch.values[555] != 0.0)) {
            scratch.values[315] = 1.0;
            scratch.node_derivatives[315] = [0.0; Instance::NODE_COUNT];
            scratch.branch_derivatives[315] = [0.0; Instance::BRANCH_COUNT];
        }

        scratch.values[556] = if (scratch.values[288] > ((-scratch.values[129]) * scratch.values[42])) { 1.0 } else { 0.0 };

        scratch.values[557] = if (scratch.values[45] == 4.0) { 1.0 } else { 0.0 };

        if (((((scratch.values[270] != 0.0) && (!(scratch.values[540] != 0.0))) && (!(scratch.values[555] != 0.0))) && (scratch.values[556] != 0.0)) && (scratch.values[557] != 0.0)) {
            scratch.store_ad(289, &AdValue::mul(AdValue::mul(AdValue::mul(AdValue::scale(scratch.ad_value(288), scratch.values[135]), AdValue::scale(scratch.ad_value(288), scratch.values[135])), AdValue::scale(scratch.ad_value(288), scratch.values[135])), AdValue::scale(scratch.ad_value(288), scratch.values[135])));
        }

        if (((((scratch.values[270] != 0.0) && (!(scratch.values[540] != 0.0))) && (!(scratch.values[555] != 0.0))) && (scratch.values[556] != 0.0)) && (!(scratch.values[557] != 0.0))) {
            scratch.store_ad(289, &AdValue::powf(AdValue::abs(AdValue::scale(scratch.ad_value(288), scratch.values[135])), scratch.values[45]));
        }

        if ((((scratch.values[270] != 0.0) && (!(scratch.values[540] != 0.0))) && (!(scratch.values[555] != 0.0))) && (scratch.values[556] != 0.0)) {
            scratch.store_ad(315, &AdValue::div_from_scalar(1.0, AdValue::sub_from_scalar(1.0, scratch.ad_value(289))));
        }

        if ((((scratch.values[270] != 0.0) && (!(scratch.values[540] != 0.0))) && (!(scratch.values[555] != 0.0))) && (!(scratch.values[556] != 0.0))) {
            scratch.store_ad(315, &AdValue::offset(AdValue::scale(AdValue::offset(scratch.ad_value(288), (scratch.values[129] * scratch.values[42])), scratch.values[138]), scratch.values[132]));
        }

        if ((scratch.values[270] != 0.0) && (!(scratch.values[540] != 0.0))) {
            scratch.store_ad(318, &AdValue::mul(AdValue::add(AdValue::add(AdValue::add(scratch.ad_value(290), scratch.ad_value(291)), scratch.ad_value(298)), scratch.ad_value(313)), scratch.ad_value(315)));
        }

        if (scratch.values[270] != 0.0) {
            scratch.store_ad(145, &AdValue::add(AdValue::add(AdValue::scale(scratch.ad_value(316), scratch.values[217]), AdValue::scale(scratch.ad_value(317), scratch.values[218])), AdValue::scale(scratch.ad_value(318), scratch.values[219])));
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

        scratch.values[558] = if !(((scratch.values[217] == 0.0) && (scratch.values[218] == 0.0)) && (scratch.values[219] == 0.0)) { 1.0 } else { 0.0 };

        if ((scratch.values[270] != 0.0) && (scratch.values[558] != 0.0)) {
            scratch.store_ad(274, &AdValue::mul(AdValue::scale(scratch.ad_value(226), 4.0), scratch.ad_value(226)));
        }

        if ((scratch.values[270] != 0.0) && (scratch.values[558] != 0.0)) {
            scratch.store_ad(275, &AdValue::div(scratch.ad_value(226), scratch.ad_value(227)));
        }

        if ((scratch.values[270] != 0.0) && (scratch.values[558] != 0.0)) {
            scratch.store_ad(276, &AdValue::add(scratch.ad_value(156), AdValue::mul(scratch.ad_value(226), scratch.ad_value(275))));
        }

        if ((scratch.values[270] != 0.0) && (scratch.values[558] != 0.0)) {
            scratch.store_ad(277, &AdValue::add(scratch.ad_value(227), scratch.ad_value(276)));
        }

        if ((scratch.values[270] != 0.0) && (scratch.values[558] != 0.0)) {
            scratch.store_ad(278, &AdValue::sub(scratch.ad_value(227), scratch.ad_value(276)));
        }

        if ((scratch.values[270] != 0.0) && (scratch.values[558] != 0.0)) {
            scratch.store_ad(279, &AdValue::sqrt(AdValue::add(AdValue::square(scratch.ad_value(278)), scratch.ad_value(274))));
        }

        if ((scratch.values[270] != 0.0) && (scratch.values[558] != 0.0)) {
            scratch.store_ad(281, &AdValue::scale(AdValue::div(AdValue::mul(scratch.ad_value(156), scratch.ad_value(227)), AdValue::add(scratch.ad_value(277), scratch.ad_value(279))), 2.0));
        }

        scratch.values[559] = if (scratch.values[156] < scratch.values[223]) { 1.0 } else { 0.0 };

        scratch.values[560] = if ((((0.5 * (scratch.values[156] * scratch.values[56]))) as f64).abs() < 230.25850929940458) { 1.0 } else { 0.0 };

        if ((((scratch.values[270] != 0.0) && (scratch.values[558] != 0.0)) && (scratch.values[559] != 0.0)) && (scratch.values[560] != 0.0)) {
            scratch.store_ad(283, &AdValue::exp(AdValue::scale(scratch.ad_value(156), (scratch.values[56] * 0.5))));
        }

        scratch.values[561] = if ((0.5 * (scratch.values[156] * scratch.values[56])) < 0.0) { 1.0 } else { 0.0 };

        if (((((scratch.values[270] != 0.0) && (scratch.values[558] != 0.0)) && (scratch.values[559] != 0.0)) && (!(scratch.values[560] != 0.0))) && (scratch.values[561] != 0.0)) {
            let assign12950_ad_e16411: AdValue = AdValue::div_from_scalar(1e-100, AdValue::offset(AdValue::mul(AdValue::sub_from_scalar((-230.25850929940458), AdValue::scale(scratch.ad_value(156), (scratch.values[56] * 0.5))), AdValue::offset(AdValue::scale(AdValue::mul(AdValue::sub_from_scalar((-230.25850929940458), AdValue::scale(scratch.ad_value(156), (scratch.values[56] * 0.5))), AdValue::offset(AdValue::scale(AdValue::sub_from_scalar((-230.25850929940458), AdValue::scale(scratch.ad_value(156), (scratch.values[56] * 0.5))), 0.3333333333333333), 1.0)), 0.5), 1.0)), 1.0));
            scratch.store_ad(283, &assign12950_ad_e16411);
        }

        if (((((scratch.values[270] != 0.0) && (scratch.values[558] != 0.0)) && (scratch.values[559] != 0.0)) && (!(scratch.values[560] != 0.0))) && (!(scratch.values[561] != 0.0))) {
            scratch.store_ad(283, &AdValue::scale(AdValue::offset(AdValue::mul(AdValue::offset(AdValue::scale(scratch.ad_value(156), (scratch.values[56] * 0.5)), (-230.25850929940458)), AdValue::offset(AdValue::scale(AdValue::mul(AdValue::offset(AdValue::scale(scratch.ad_value(156), (scratch.values[56] * 0.5)), (-230.25850929940458)), AdValue::offset(AdValue::scale(AdValue::offset(AdValue::scale(scratch.ad_value(156), (scratch.values[56] * 0.5)), (-230.25850929940458)), 0.3333333333333333), 1.0)), 0.5), 1.0)), 1.0), 1e100));
        }

        if (((scratch.values[270] != 0.0) && (scratch.values[558] != 0.0)) && (scratch.values[559] != 0.0)) {
            scratch.store_ad(280, &AdValue::square(scratch.ad_value(283)));
        }

        if (((scratch.values[270] != 0.0) && (scratch.values[558] != 0.0)) && (!(scratch.values[559] != 0.0))) {
            scratch.store_ad(280, &AdValue::mul(AdValue::offset(AdValue::scale(AdValue::sub(scratch.ad_value(156), scratch.ad_value(223)), scratch.values[56]), 1.0), scratch.ad_value(224)));
        }

    }

    pub(super) fn stamp_transient_block_9(
        &mut self,
        ctx: &GeneratedEvalContext<'_>,
        stamper: &mut GeneratedStamper<'_>,
        scratch: &mut Scratch,
    ) {
        let _ = stamper;
        if (((scratch.values[270] != 0.0) && (scratch.values[558] != 0.0)) && (!(scratch.values[559] != 0.0))) {
            scratch.store_ad(283, &AdValue::sqrt(scratch.ad_value(280)));
        }

        if ((scratch.values[270] != 0.0) && (scratch.values[558] != 0.0)) {
            scratch.store_ad(280, &AdValue::offset(scratch.ad_value(280), (-1.0)));
        }

        if ((scratch.values[270] != 0.0) && (scratch.values[558] != 0.0)) {
            scratch.store_ad(282, &AdValue::div_from_scalar(1.0, scratch.ad_value(283)));
        }

        scratch.values[562] = if (scratch.values[156] > 0.0) { 1.0 } else { 0.0 };

        if (((scratch.values[270] != 0.0) && (scratch.values[558] != 0.0)) && (scratch.values[562] != 0.0)) {
            scratch.store_ad(284, &AdValue::scale(AdValue::ln(AdValue::add(AdValue::offset(scratch.ad_value(282), 2.0), AdValue::sqrt(AdValue::mul(AdValue::offset(scratch.ad_value(282), 1.0), AdValue::offset(scratch.ad_value(282), 3.0))))), (scratch.values[55] * 2.0)));
        }

        if (((scratch.values[270] != 0.0) && (scratch.values[558] != 0.0)) && (!(scratch.values[562] != 0.0))) {
            scratch.store_ad(284, &AdValue::sub(AdValue::scale(AdValue::ln(AdValue::add(AdValue::offset(AdValue::scale(scratch.ad_value(283), 2.0), 1.0), AdValue::sqrt(AdValue::mul(AdValue::offset(scratch.ad_value(283), 1.0), AdValue::offset(AdValue::scale(scratch.ad_value(283), 3.0), 1.0))))), (scratch.values[55] * 2.0)), scratch.ad_value(156)));
        }

        if ((scratch.values[270] != 0.0) && (scratch.values[558] != 0.0)) {
            scratch.store_ad(285, &AdValue::sub(scratch.ad_value(225), scratch.ad_value(284)));
        }

        if ((scratch.values[270] != 0.0) && (scratch.values[558] != 0.0)) {
            scratch.store_ad(286, &AdValue::scale(AdValue::sub(AdValue::add(scratch.ad_value(156), scratch.ad_value(285)), AdValue::sqrt(AdValue::offset(AdValue::mul(AdValue::sub(scratch.ad_value(156), scratch.ad_value(285)), AdValue::sub(scratch.ad_value(156), scratch.ad_value(285))), ((4.0 * scratch.values[55]) * scratch.values[55])))), 0.5));
        }

        if ((scratch.values[270] != 0.0) && (scratch.values[558] != 0.0)) {
            scratch.store_ad(287, &AdValue::scale(AdValue::sub(AdValue::add(scratch.ad_value(156), scratch.ad_value(228)), AdValue::sqrt(AdValue::offset(AdValue::mul(AdValue::sub(scratch.ad_value(156), scratch.ad_value(228)), AdValue::sub(scratch.ad_value(156), scratch.ad_value(228))), ((4.0 * scratch.values[53]) * scratch.values[53])))), 0.5));
        }

        if ((scratch.values[270] != 0.0) && (scratch.values[558] != 0.0)) {
            scratch.store_ad(288, &AdValue::scale(AdValue::sub(scratch.ad_value(156), AdValue::sqrt(AdValue::offset(AdValue::mul(scratch.ad_value(156), scratch.ad_value(156)), ((4.0 * 1e-6) * 1e-6)))), 0.5));
        }

        scratch.values[563] = if (scratch.values[217] == 0.0) { 1.0 } else { 0.0 };

        if ((scratch.values[270] != 0.0) && (scratch.values[563] != 0.0)) {
            scratch.values[316] = 0.0;
            scratch.node_derivatives[316] = [0.0; Instance::NODE_COUNT];
            scratch.branch_derivatives[316] = [0.0; Instance::BRANCH_COUNT];
        }

        scratch.values[564] = if (scratch.values[93] == 0.5) { 1.0 } else { 0.0 };

        if (((scratch.values[270] != 0.0) && (!(scratch.values[563] != 0.0))) && (scratch.values[564] != 0.0)) {
            scratch.store_ad(289, &AdValue::sqrt(AdValue::sub_from_scalar(1.0, AdValue::scale(scratch.ad_value(281), scratch.values[90]))));
        }

        if (((scratch.values[270] != 0.0) && (!(scratch.values[563] != 0.0))) && (!(scratch.values[564] != 0.0))) {
            scratch.store_ad(289, &AdValue::powf(AdValue::sub_from_scalar(1.0, AdValue::scale(scratch.ad_value(281), scratch.values[90])), scratch.values[93]));
        }

        if ((scratch.values[270] != 0.0) && (!(scratch.values[563] != 0.0))) {
            scratch.store_ad(290, &AdValue::scale(scratch.ad_value(280), scratch.values[72]));
        }

        scratch.values[565] = if ((scratch.values[22] == 0.0) && (scratch.values[25] == 0.0)) { 1.0 } else { 0.0 };

        if (((scratch.values[270] != 0.0) && (!(scratch.values[563] != 0.0))) && (scratch.values[565] != 0.0)) {
            scratch.values[291] = 0.0;
            scratch.node_derivatives[291] = [0.0; Instance::NODE_COUNT];
            scratch.branch_derivatives[291] = [0.0; Instance::BRANCH_COUNT];
        }

        if (((scratch.values[270] != 0.0) && (!(scratch.values[563] != 0.0))) && (!(scratch.values[565] != 0.0))) {
            scratch.store_ad(292, &AdValue::sub_from_scalar(scratch.values[78], scratch.ad_value(286)));
        }

        if (((scratch.values[270] != 0.0) && (!(scratch.values[563] != 0.0))) && (!(scratch.values[565] != 0.0))) {
            scratch.store_ad(293, &AdValue::sub_from_scalar(1.0, AdValue::sqrt(AdValue::sub_from_scalar(1.0, AdValue::div(scratch.ad_value(284), scratch.ad_value(292))))));
        }

        scratch.values[566] = if (scratch.values[11] == 0.5) { 1.0 } else { 0.0 };

        if ((((scratch.values[270] != 0.0) && (!(scratch.values[563] != 0.0))) && (!(scratch.values[565] != 0.0))) && (scratch.values[566] != 0.0)) {
            scratch.values[294] = 0.0;
            scratch.node_derivatives[294] = [0.0; Instance::NODE_COUNT];
            scratch.branch_derivatives[294] = [0.0; Instance::BRANCH_COUNT];
        }

        if ((((scratch.values[270] != 0.0) && (!(scratch.values[563] != 0.0))) && (!(scratch.values[565] != 0.0))) && (!(scratch.values[566] != 0.0))) {
            scratch.store_ad(294, &AdValue::scale(AdValue::add(AdValue::div(AdValue::mul(AdValue::square(scratch.ad_value(293)), AdValue::ln(scratch.ad_value(293))), AdValue::sub_from_scalar(1.0, scratch.ad_value(293))), scratch.ad_value(293)), (1.0 - (2.0 * scratch.values[11]))));
        }

        if (((scratch.values[270] != 0.0) && (!(scratch.values[563] != 0.0))) && (!(scratch.values[565] != 0.0))) {
            scratch.store_ad(295, &AdValue::add(scratch.ad_value(293), scratch.ad_value(294)));
        }

        scratch.values[567] = if (scratch.values[11] == 0.5) { 1.0 } else { 0.0 };

        if ((((scratch.values[270] != 0.0) && (!(scratch.values[563] != 0.0))) && (!(scratch.values[565] != 0.0))) && (scratch.values[567] != 0.0)) {
            scratch.store_ad(289, &AdValue::sqrt(AdValue::scale(scratch.ad_value(292), scratch.values[114])));
        }

        if ((((scratch.values[270] != 0.0) && (!(scratch.values[563] != 0.0))) && (!(scratch.values[565] != 0.0))) && (!(scratch.values[567] != 0.0))) {
            scratch.store_ad(289, &AdValue::powf(AdValue::scale(scratch.ad_value(292), scratch.values[114]), scratch.values[11]));
        }

        if (((scratch.values[270] != 0.0) && (!(scratch.values[563] != 0.0))) && (!(scratch.values[565] != 0.0))) {
            scratch.store_ad(296, &AdValue::scale(scratch.ad_value(289), scratch.values[108]));
        }

        if (((scratch.values[270] != 0.0) && (!(scratch.values[563] != 0.0))) && (!(scratch.values[565] != 0.0))) {
            scratch.store_ad(297, &AdValue::scale(AdValue::mul(AdValue::offset(scratch.ad_value(283), (-1.0)), scratch.ad_value(296)), scratch.values[69]));
        }

        if (((scratch.values[270] != 0.0) && (!(scratch.values[563] != 0.0))) && (!(scratch.values[565] != 0.0))) {
            scratch.store_ad(291, &AdValue::scale(AdValue::mul(scratch.ad_value(297), scratch.ad_value(295)), scratch.values[22]));
        }

        scratch.values[568] = if (scratch.values[25] == 0.0) { 1.0 } else { 0.0 };

        if (((scratch.values[270] != 0.0) && (!(scratch.values[563] != 0.0))) && (scratch.values[568] != 0.0)) {
            scratch.values[298] = 0.0;
            scratch.node_derivatives[298] = [0.0; Instance::NODE_COUNT];
            scratch.branch_derivatives[298] = [0.0; Instance::BRANCH_COUNT];
        }

        if (((scratch.values[270] != 0.0) && (!(scratch.values[563] != 0.0))) && (!(scratch.values[568] != 0.0))) {
            scratch.store_ad(299, &AdValue::scale(AdValue::div(AdValue::scale(scratch.ad_value(296), scratch.values[93]), scratch.ad_value(292)), scratch.values[123]));
        }

        if (((scratch.values[270] != 0.0) && (!(scratch.values[563] != 0.0))) && (!(scratch.values[568] != 0.0))) {
            scratch.store_ad(300, &AdValue::div_from_scalar((0.666666666666667 * scratch.values[120]), scratch.ad_value(299)));
        }

        if (((scratch.values[270] != 0.0) && (!(scratch.values[563] != 0.0))) && (!(scratch.values[568] != 0.0))) {
            scratch.store_ad(301, &AdValue::square(scratch.ad_value(300)));
        }

        if (((scratch.values[270] != 0.0) && (!(scratch.values[563] != 0.0))) && (!(scratch.values[568] != 0.0))) {
            scratch.store_ad(302, &AdValue::sqrt(AdValue::div(AdValue::square(scratch.ad_value(301)), AdValue::offset(AdValue::square(scratch.ad_value(301)), 1.0))));
        }

        if (((scratch.values[270] != 0.0) && (!(scratch.values[563] != 0.0))) && (!(scratch.values[568] != 0.0))) {
            scratch.store_ad(303, &AdValue::sqrt(AdValue::abs(scratch.ad_value(302))));
        }

        if (((scratch.values[270] != 0.0) && (!(scratch.values[563] != 0.0))) && (!(scratch.values[568] != 0.0))) {
            scratch.store_ad(304, &AdValue::mul(scratch.ad_value(302), scratch.ad_value(303)));
        }

        scratch.values[569] = if (((-scratch.values[11]) * scratch.values[96]) == (-1.0)) { 1.0 } else { 0.0 };

        if ((((scratch.values[270] != 0.0) && (!(scratch.values[563] != 0.0))) && (!(scratch.values[568] != 0.0))) && (scratch.values[569] != 0.0)) {
            scratch.store_ad(305, &AdValue::div_from_scalar(1.0, AdValue::offset(AdValue::mul(scratch.ad_value(299), scratch.ad_value(304)), 1.0)));
        }

        if ((((scratch.values[270] != 0.0) && (!(scratch.values[563] != 0.0))) && (!(scratch.values[568] != 0.0))) && (!(scratch.values[569] != 0.0))) {
            scratch.store_ad(305, &AdValue::powf(AdValue::offset(AdValue::mul(scratch.ad_value(299), scratch.ad_value(304)), 1.0), ((-scratch.values[11]) * scratch.values[96])));
        }

        if (((scratch.values[270] != 0.0) && (!(scratch.values[563] != 0.0))) && (!(scratch.values[568] != 0.0))) {
            scratch.store_ad(306, &AdValue::div(AdValue::mul(scratch.ad_value(295), scratch.ad_value(305)), AdValue::add(scratch.ad_value(295), scratch.ad_value(305))));
        }

        if (((scratch.values[270] != 0.0) && (!(scratch.values[563] != 0.0))) && (!(scratch.values[568] != 0.0))) {
            scratch.store_ad(307, &AdValue::sqrt(AdValue::scale(AdValue::div(scratch.ad_value(299), scratch.ad_value(303)), 0.375)));
        }

        if (((scratch.values[270] != 0.0) && (!(scratch.values[563] != 0.0))) && (!(scratch.values[568] != 0.0))) {
            scratch.store_ad(308, &AdValue::sub(AdValue::scale(AdValue::mul(scratch.ad_value(300), scratch.ad_value(303)), 2.0), scratch.ad_value(302)));
        }

        if (((scratch.values[270] != 0.0) && (!(scratch.values[563] != 0.0))) && (!(scratch.values[568] != 0.0))) {
            scratch.store_ad(309, &AdValue::add(AdValue::sub(AdValue::mul(AdValue::scale(scratch.ad_value(300), scratch.values[120]), scratch.ad_value(303)), AdValue::scale(scratch.ad_value(302), scratch.values[120])), AdValue::scale(AdValue::mul(scratch.ad_value(299), scratch.ad_value(304)), 0.5)));
        }

        if (((scratch.values[270] != 0.0) && (!(scratch.values[563] != 0.0))) && (!(scratch.values[568] != 0.0))) {
            scratch.store_ad(310, &AdValue::mul(AdValue::offset(scratch.ad_value(308), (-1.0)), scratch.ad_value(307)));
        }

        if (((scratch.values[270] != 0.0) && (!(scratch.values[563] != 0.0))) && (!(scratch.values[568] != 0.0))) {
            scratch.store_ad(271, &AdValue::square(scratch.ad_value(310)));
        }

        scratch.values[570] = if (scratch.values[310] > 0.0) { 1.0 } else { 0.0 };

        if ((((scratch.values[270] != 0.0) && (!(scratch.values[563] != 0.0))) && (!(scratch.values[568] != 0.0))) && (scratch.values[570] != 0.0)) {
            scratch.store_ad(272, &AdValue::div_from_scalar(1.0, AdValue::offset(AdValue::scale(scratch.ad_value(310), scratch.values[57]), 1.0)));
        }

        if ((((scratch.values[270] != 0.0) && (!(scratch.values[563] != 0.0))) && (!(scratch.values[568] != 0.0))) && (!(scratch.values[570] != 0.0))) {
            scratch.store_ad(272, &AdValue::div_from_scalar(1.0, AdValue::sub_from_scalar(1.0, AdValue::scale(scratch.ad_value(310), scratch.values[57]))));
        }

        scratch.values[571] = if (((-scratch.values[271]) + scratch.values[309]) > (-230.25850929940458)) { 1.0 } else { 0.0 };

        if ((((scratch.values[270] != 0.0) && (!(scratch.values[563] != 0.0))) && (!(scratch.values[568] != 0.0))) && (scratch.values[571] != 0.0)) {
            scratch.store_ad(289, &AdValue::exp(AdValue::sub(scratch.ad_value(309), scratch.ad_value(271))));
        }

        if ((((scratch.values[270] != 0.0) && (!(scratch.values[563] != 0.0))) && (!(scratch.values[568] != 0.0))) && (!(scratch.values[571] != 0.0))) {
            scratch.store_ad(289, &AdValue::div_from_scalar(1e-100, AdValue::offset(AdValue::mul(AdValue::sub_from_scalar((-230.25850929940458), AdValue::sub(scratch.ad_value(309), scratch.ad_value(271))), AdValue::offset(AdValue::scale(AdValue::mul(AdValue::sub_from_scalar((-230.25850929940458), AdValue::sub(scratch.ad_value(309), scratch.ad_value(271))), AdValue::offset(AdValue::scale(AdValue::sub_from_scalar((-230.25850929940458), AdValue::sub(scratch.ad_value(309), scratch.ad_value(271))), 0.3333333333333333), 1.0)), 0.5), 1.0)), 1.0)));
        }

        if (((scratch.values[270] != 0.0) && (!(scratch.values[563] != 0.0))) && (!(scratch.values[568] != 0.0))) {
            scratch.store_ad(273, &AdValue::mul(AdValue::add(AdValue::add(AdValue::scale(scratch.ad_value(272), 0.29214664), AdValue::scale(AdValue::square(scratch.ad_value(272)), scratch.values[58])), AdValue::scale(AdValue::mul(AdValue::square(scratch.ad_value(272)), scratch.ad_value(272)), scratch.values[59])), scratch.ad_value(289)));
        }

        scratch.values[572] = if (scratch.values[310] > 0.0) { 1.0 } else { 0.0 };

        if ((((scratch.values[270] != 0.0) && (!(scratch.values[563] != 0.0))) && (!(scratch.values[568] != 0.0))) && (scratch.values[572] != 0.0)) {
            scratch.values[311] = scratch.values[273];
            scratch.node_derivatives[311] = scratch.node_derivatives[273];
            scratch.branch_derivatives[311] = scratch.branch_derivatives[273];
        }

        scratch.values[573] = if (scratch.values[309] > (-230.25850929940458)) { 1.0 } else { 0.0 };

        if (((((scratch.values[270] != 0.0) && (!(scratch.values[563] != 0.0))) && (!(scratch.values[568] != 0.0))) && (!(scratch.values[572] != 0.0))) && (scratch.values[573] != 0.0)) {
            scratch.store_ad(289, &AdValue::exp(scratch.ad_value(309)));
        }

        if (((((scratch.values[270] != 0.0) && (!(scratch.values[563] != 0.0))) && (!(scratch.values[568] != 0.0))) && (!(scratch.values[572] != 0.0))) && (!(scratch.values[573] != 0.0))) {
            scratch.store_ad(289, &AdValue::div_from_scalar(1e-100, AdValue::offset(AdValue::mul(AdValue::sub_from_scalar((-230.25850929940458), scratch.ad_value(309)), AdValue::offset(AdValue::scale(AdValue::mul(AdValue::sub_from_scalar((-230.25850929940458), scratch.ad_value(309)), AdValue::offset(AdValue::scale(AdValue::sub_from_scalar((-230.25850929940458), scratch.ad_value(309)), 0.3333333333333333), 1.0)), 0.5), 1.0)), 1.0)));
        }

        if ((((scratch.values[270] != 0.0) && (!(scratch.values[563] != 0.0))) && (!(scratch.values[568] != 0.0))) && (!(scratch.values[572] != 0.0))) {
            scratch.store_ad(311, &AdValue::sub(AdValue::scale(scratch.ad_value(289), 2.0), scratch.ad_value(273)));
        }

        if (((scratch.values[270] != 0.0) && (!(scratch.values[563] != 0.0))) && (!(scratch.values[568] != 0.0))) {
            scratch.store_ad(312, &AdValue::scale(AdValue::div(AdValue::scale(scratch.ad_value(311), scratch.values[120]), scratch.ad_value(307)), (1.772453850905516 * 0.5)));
        }

        if (((scratch.values[270] != 0.0) && (!(scratch.values[563] != 0.0))) && (!(scratch.values[568] != 0.0))) {
            scratch.store_ad(298, &AdValue::scale(AdValue::mul(AdValue::mul(scratch.ad_value(297), scratch.ad_value(312)), scratch.ad_value(306)), scratch.values[25]));
        }

        scratch.values[574] = if (scratch.values[31] == 0.0) { 1.0 } else { 0.0 };

        if (((scratch.values[270] != 0.0) && (!(scratch.values[563] != 0.0))) && (scratch.values[574] != 0.0)) {
            scratch.values[313] = 0.0;
            scratch.node_derivatives[313] = [0.0; Instance::NODE_COUNT];
            scratch.branch_derivatives[313] = [0.0; Instance::BRANCH_COUNT];
        }

        scratch.values[575] = if (scratch.values[11] == 0.5) { 1.0 } else { 0.0 };

        if ((((scratch.values[270] != 0.0) && (!(scratch.values[563] != 0.0))) && (!(scratch.values[574] != 0.0))) && (scratch.values[575] != 0.0)) {
            scratch.store_ad(289, &AdValue::sqrt(AdValue::scale(AdValue::sub_from_scalar(scratch.values[8], scratch.ad_value(287)), scratch.values[114])));
        }

        if ((((scratch.values[270] != 0.0) && (!(scratch.values[563] != 0.0))) && (!(scratch.values[574] != 0.0))) && (!(scratch.values[575] != 0.0))) {
            scratch.store_ad(289, &AdValue::powf(AdValue::scale(AdValue::sub_from_scalar(scratch.values[8], scratch.ad_value(287)), scratch.values[114]), scratch.values[11]));
        }

        if (((scratch.values[270] != 0.0) && (!(scratch.values[563] != 0.0))) && (!(scratch.values[574] != 0.0))) {
            scratch.store_ad(314, &AdValue::scale(AdValue::div(AdValue::scale(AdValue::sub_from_scalar(scratch.values[8], scratch.ad_value(287)), scratch.values[111]), scratch.ad_value(289)), scratch.values[96]));
        }

        scratch.values[576] = if (((((-scratch.values[126]) / scratch.values[314])) as f64).abs() < 230.25850929940458) { 1.0 } else { 0.0 };

        if ((((scratch.values[270] != 0.0) && (!(scratch.values[563] != 0.0))) && (!(scratch.values[574] != 0.0))) && (scratch.values[576] != 0.0)) {
            scratch.store_ad(289, &AdValue::exp(AdValue::div(AdValue::neg(scratch.ad_value(126)), scratch.ad_value(314))));
        }

        scratch.values[577] = if (((-scratch.values[126]) / scratch.values[314]) < 0.0) { 1.0 } else { 0.0 };

        if (((((scratch.values[270] != 0.0) && (!(scratch.values[563] != 0.0))) && (!(scratch.values[574] != 0.0))) && (!(scratch.values[576] != 0.0))) && (scratch.values[577] != 0.0)) {
            let assign13720_ad_e17559: AdValue = AdValue::offset(AdValue::mul(AdValue::sub_from_scalar((-230.25850929940458), AdValue::div(AdValue::neg(scratch.ad_value(126)), scratch.ad_value(314))), AdValue::offset(AdValue::scale(AdValue::mul(AdValue::sub_from_scalar((-230.25850929940458), AdValue::div(AdValue::neg(scratch.ad_value(126)), scratch.ad_value(314))), AdValue::offset(AdValue::scale(AdValue::sub_from_scalar((-230.25850929940458), AdValue::div(AdValue::neg(scratch.ad_value(126)), scratch.ad_value(314))), 0.3333333333333333), 1.0)), 0.5), 1.0)), 1.0);
            scratch.store_ad(289, &AdValue::div_from_scalar(1e-100, assign13720_ad_e17559));
        }

        if (((((scratch.values[270] != 0.0) && (!(scratch.values[563] != 0.0))) && (!(scratch.values[574] != 0.0))) && (!(scratch.values[576] != 0.0))) && (!(scratch.values[577] != 0.0))) {
            let assign13730_ad_e17607: AdValue = AdValue::scale(AdValue::offset(AdValue::mul(AdValue::offset(AdValue::div(AdValue::neg(scratch.ad_value(126)), scratch.ad_value(314)), (-230.25850929940458)), AdValue::offset(AdValue::scale(AdValue::mul(AdValue::offset(AdValue::div(AdValue::neg(scratch.ad_value(126)), scratch.ad_value(314)), (-230.25850929940458)), AdValue::offset(AdValue::scale(AdValue::offset(AdValue::div(AdValue::neg(scratch.ad_value(126)), scratch.ad_value(314)), (-230.25850929940458)), 0.3333333333333333), 1.0)), 0.5), 1.0)), 1.0), 1e100);
            scratch.store_ad(289, &assign13730_ad_e17607);
        }

        if (((scratch.values[270] != 0.0) && (!(scratch.values[563] != 0.0))) && (!(scratch.values[574] != 0.0))) {
            scratch.store_ad(313, &AdValue::scale(AdValue::mul(AdValue::mul(AdValue::mul(scratch.ad_value(156), scratch.ad_value(314)), scratch.ad_value(314)), scratch.ad_value(289)), scratch.values[31]));
        }

        scratch.values[578] = if (scratch.values[40] > 1000.0) { 1.0 } else { 0.0 };

        if (((scratch.values[270] != 0.0) && (!(scratch.values[563] != 0.0))) && (scratch.values[578] != 0.0)) {
            scratch.values[315] = 1.0;
            scratch.node_derivatives[315] = [0.0; Instance::NODE_COUNT];
            scratch.branch_derivatives[315] = [0.0; Instance::BRANCH_COUNT];
        }

        scratch.values[579] = if (scratch.values[288] > ((-scratch.values[129]) * scratch.values[40])) { 1.0 } else { 0.0 };

        scratch.values[580] = if (scratch.values[43] == 4.0) { 1.0 } else { 0.0 };

        if (((((scratch.values[270] != 0.0) && (!(scratch.values[563] != 0.0))) && (!(scratch.values[578] != 0.0))) && (scratch.values[579] != 0.0)) && (scratch.values[580] != 0.0)) {
            scratch.store_ad(289, &AdValue::mul(AdValue::mul(AdValue::mul(AdValue::scale(scratch.ad_value(288), scratch.values[133]), AdValue::scale(scratch.ad_value(288), scratch.values[133])), AdValue::scale(scratch.ad_value(288), scratch.values[133])), AdValue::scale(scratch.ad_value(288), scratch.values[133])));
        }

        if (((((scratch.values[270] != 0.0) && (!(scratch.values[563] != 0.0))) && (!(scratch.values[578] != 0.0))) && (scratch.values[579] != 0.0)) && (!(scratch.values[580] != 0.0))) {
            scratch.store_ad(289, &AdValue::powf(AdValue::abs(AdValue::scale(scratch.ad_value(288), scratch.values[133])), scratch.values[43]));
        }

        if ((((scratch.values[270] != 0.0) && (!(scratch.values[563] != 0.0))) && (!(scratch.values[578] != 0.0))) && (scratch.values[579] != 0.0)) {
            scratch.store_ad(315, &AdValue::div_from_scalar(1.0, AdValue::sub_from_scalar(1.0, scratch.ad_value(289))));
        }

        if ((((scratch.values[270] != 0.0) && (!(scratch.values[563] != 0.0))) && (!(scratch.values[578] != 0.0))) && (!(scratch.values[579] != 0.0))) {
            scratch.store_ad(315, &AdValue::offset(AdValue::scale(AdValue::offset(scratch.ad_value(288), (scratch.values[129] * scratch.values[40])), scratch.values[136]), scratch.values[130]));
        }

        if ((scratch.values[270] != 0.0) && (!(scratch.values[563] != 0.0))) {
            scratch.store_ad(316, &AdValue::mul(AdValue::add(AdValue::add(AdValue::add(scratch.ad_value(290), scratch.ad_value(291)), scratch.ad_value(298)), scratch.ad_value(313)), scratch.ad_value(315)));
        }

        scratch.values[581] = if (scratch.values[218] == 0.0) { 1.0 } else { 0.0 };

        if ((scratch.values[270] != 0.0) && (scratch.values[581] != 0.0)) {
            scratch.values[317] = 0.0;
            scratch.node_derivatives[317] = [0.0; Instance::NODE_COUNT];
            scratch.branch_derivatives[317] = [0.0; Instance::BRANCH_COUNT];
        }

        scratch.values[582] = if (scratch.values[94] == 0.5) { 1.0 } else { 0.0 };

        if (((scratch.values[270] != 0.0) && (!(scratch.values[581] != 0.0))) && (scratch.values[582] != 0.0)) {
            scratch.store_ad(289, &AdValue::sqrt(AdValue::sub_from_scalar(1.0, AdValue::scale(scratch.ad_value(281), scratch.values[91]))));
        }

        if (((scratch.values[270] != 0.0) && (!(scratch.values[581] != 0.0))) && (!(scratch.values[582] != 0.0))) {
            scratch.store_ad(289, &AdValue::powf(AdValue::sub_from_scalar(1.0, AdValue::scale(scratch.ad_value(281), scratch.values[91])), scratch.values[94]));
        }

        if ((scratch.values[270] != 0.0) && (!(scratch.values[581] != 0.0))) {
            scratch.store_ad(290, &AdValue::scale(scratch.ad_value(280), scratch.values[73]));
        }

        scratch.values[583] = if ((scratch.values[23] == 0.0) && (scratch.values[26] == 0.0)) { 1.0 } else { 0.0 };

        if (((scratch.values[270] != 0.0) && (!(scratch.values[581] != 0.0))) && (scratch.values[583] != 0.0)) {
            scratch.values[291] = 0.0;
            scratch.node_derivatives[291] = [0.0; Instance::NODE_COUNT];
            scratch.branch_derivatives[291] = [0.0; Instance::BRANCH_COUNT];
        }

        if (((scratch.values[270] != 0.0) && (!(scratch.values[581] != 0.0))) && (!(scratch.values[583] != 0.0))) {
            scratch.store_ad(292, &AdValue::sub_from_scalar(scratch.values[79], scratch.ad_value(286)));
        }

        if (((scratch.values[270] != 0.0) && (!(scratch.values[581] != 0.0))) && (!(scratch.values[583] != 0.0))) {
            scratch.store_ad(293, &AdValue::sub_from_scalar(1.0, AdValue::sqrt(AdValue::sub_from_scalar(1.0, AdValue::div(scratch.ad_value(284), scratch.ad_value(292))))));
        }

        scratch.values[584] = if (scratch.values[12] == 0.5) { 1.0 } else { 0.0 };

        if ((((scratch.values[270] != 0.0) && (!(scratch.values[581] != 0.0))) && (!(scratch.values[583] != 0.0))) && (scratch.values[584] != 0.0)) {
            scratch.values[294] = 0.0;
            scratch.node_derivatives[294] = [0.0; Instance::NODE_COUNT];
            scratch.branch_derivatives[294] = [0.0; Instance::BRANCH_COUNT];
        }

        if ((((scratch.values[270] != 0.0) && (!(scratch.values[581] != 0.0))) && (!(scratch.values[583] != 0.0))) && (!(scratch.values[584] != 0.0))) {
            scratch.store_ad(294, &AdValue::scale(AdValue::add(AdValue::div(AdValue::mul(AdValue::square(scratch.ad_value(293)), AdValue::ln(scratch.ad_value(293))), AdValue::sub_from_scalar(1.0, scratch.ad_value(293))), scratch.ad_value(293)), (1.0 - (2.0 * scratch.values[12]))));
        }

        if (((scratch.values[270] != 0.0) && (!(scratch.values[581] != 0.0))) && (!(scratch.values[583] != 0.0))) {
            scratch.store_ad(295, &AdValue::add(scratch.ad_value(293), scratch.ad_value(294)));
        }

        scratch.values[585] = if (scratch.values[12] == 0.5) { 1.0 } else { 0.0 };

        if ((((scratch.values[270] != 0.0) && (!(scratch.values[581] != 0.0))) && (!(scratch.values[583] != 0.0))) && (scratch.values[585] != 0.0)) {
            scratch.store_ad(289, &AdValue::sqrt(AdValue::scale(scratch.ad_value(292), scratch.values[115])));
        }

        if ((((scratch.values[270] != 0.0) && (!(scratch.values[581] != 0.0))) && (!(scratch.values[583] != 0.0))) && (!(scratch.values[585] != 0.0))) {
            scratch.store_ad(289, &AdValue::powf(AdValue::scale(scratch.ad_value(292), scratch.values[115]), scratch.values[12]));
        }

        if (((scratch.values[270] != 0.0) && (!(scratch.values[581] != 0.0))) && (!(scratch.values[583] != 0.0))) {
            scratch.store_ad(296, &AdValue::scale(scratch.ad_value(289), scratch.values[109]));
        }

        if (((scratch.values[270] != 0.0) && (!(scratch.values[581] != 0.0))) && (!(scratch.values[583] != 0.0))) {
            scratch.store_ad(297, &AdValue::scale(AdValue::mul(AdValue::offset(scratch.ad_value(283), (-1.0)), scratch.ad_value(296)), scratch.values[70]));
        }

        if (((scratch.values[270] != 0.0) && (!(scratch.values[581] != 0.0))) && (!(scratch.values[583] != 0.0))) {
            scratch.store_ad(291, &AdValue::scale(AdValue::mul(scratch.ad_value(297), scratch.ad_value(295)), scratch.values[23]));
        }

        scratch.values[586] = if (scratch.values[26] == 0.0) { 1.0 } else { 0.0 };

        if (((scratch.values[270] != 0.0) && (!(scratch.values[581] != 0.0))) && (scratch.values[586] != 0.0)) {
            scratch.values[298] = 0.0;
            scratch.node_derivatives[298] = [0.0; Instance::NODE_COUNT];
            scratch.branch_derivatives[298] = [0.0; Instance::BRANCH_COUNT];
        }

        if (((scratch.values[270] != 0.0) && (!(scratch.values[581] != 0.0))) && (!(scratch.values[586] != 0.0))) {
            scratch.store_ad(299, &AdValue::scale(AdValue::div(AdValue::scale(scratch.ad_value(296), scratch.values[94]), scratch.ad_value(292)), scratch.values[124]));
        }

        if (((scratch.values[270] != 0.0) && (!(scratch.values[581] != 0.0))) && (!(scratch.values[586] != 0.0))) {
            scratch.store_ad(300, &AdValue::div_from_scalar((0.666666666666667 * scratch.values[121]), scratch.ad_value(299)));
        }

        if (((scratch.values[270] != 0.0) && (!(scratch.values[581] != 0.0))) && (!(scratch.values[586] != 0.0))) {
            scratch.store_ad(301, &AdValue::square(scratch.ad_value(300)));
        }

        if (((scratch.values[270] != 0.0) && (!(scratch.values[581] != 0.0))) && (!(scratch.values[586] != 0.0))) {
            scratch.store_ad(302, &AdValue::sqrt(AdValue::div(AdValue::square(scratch.ad_value(301)), AdValue::offset(AdValue::square(scratch.ad_value(301)), 1.0))));
        }

        if (((scratch.values[270] != 0.0) && (!(scratch.values[581] != 0.0))) && (!(scratch.values[586] != 0.0))) {
            scratch.store_ad(303, &AdValue::sqrt(AdValue::abs(scratch.ad_value(302))));
        }

        if (((scratch.values[270] != 0.0) && (!(scratch.values[581] != 0.0))) && (!(scratch.values[586] != 0.0))) {
            scratch.store_ad(304, &AdValue::mul(scratch.ad_value(302), scratch.ad_value(303)));
        }

        scratch.values[587] = if (((-scratch.values[12]) * scratch.values[97]) == (-1.0)) { 1.0 } else { 0.0 };

        if ((((scratch.values[270] != 0.0) && (!(scratch.values[581] != 0.0))) && (!(scratch.values[586] != 0.0))) && (scratch.values[587] != 0.0)) {
            scratch.store_ad(305, &AdValue::div_from_scalar(1.0, AdValue::offset(AdValue::mul(scratch.ad_value(299), scratch.ad_value(304)), 1.0)));
        }

        if ((((scratch.values[270] != 0.0) && (!(scratch.values[581] != 0.0))) && (!(scratch.values[586] != 0.0))) && (!(scratch.values[587] != 0.0))) {
            scratch.store_ad(305, &AdValue::powf(AdValue::offset(AdValue::mul(scratch.ad_value(299), scratch.ad_value(304)), 1.0), ((-scratch.values[12]) * scratch.values[97])));
        }

        if (((scratch.values[270] != 0.0) && (!(scratch.values[581] != 0.0))) && (!(scratch.values[586] != 0.0))) {
            scratch.store_ad(306, &AdValue::div(AdValue::mul(scratch.ad_value(295), scratch.ad_value(305)), AdValue::add(scratch.ad_value(295), scratch.ad_value(305))));
        }

        if (((scratch.values[270] != 0.0) && (!(scratch.values[581] != 0.0))) && (!(scratch.values[586] != 0.0))) {
            scratch.store_ad(307, &AdValue::sqrt(AdValue::scale(AdValue::div(scratch.ad_value(299), scratch.ad_value(303)), 0.375)));
        }

        if (((scratch.values[270] != 0.0) && (!(scratch.values[581] != 0.0))) && (!(scratch.values[586] != 0.0))) {
            scratch.store_ad(308, &AdValue::sub(AdValue::scale(AdValue::mul(scratch.ad_value(300), scratch.ad_value(303)), 2.0), scratch.ad_value(302)));
        }

        if (((scratch.values[270] != 0.0) && (!(scratch.values[581] != 0.0))) && (!(scratch.values[586] != 0.0))) {
            scratch.store_ad(309, &AdValue::add(AdValue::sub(AdValue::mul(AdValue::scale(scratch.ad_value(300), scratch.values[121]), scratch.ad_value(303)), AdValue::scale(scratch.ad_value(302), scratch.values[121])), AdValue::scale(AdValue::mul(scratch.ad_value(299), scratch.ad_value(304)), 0.5)));
        }

        if (((scratch.values[270] != 0.0) && (!(scratch.values[581] != 0.0))) && (!(scratch.values[586] != 0.0))) {
            scratch.store_ad(310, &AdValue::mul(AdValue::offset(scratch.ad_value(308), (-1.0)), scratch.ad_value(307)));
        }

        if (((scratch.values[270] != 0.0) && (!(scratch.values[581] != 0.0))) && (!(scratch.values[586] != 0.0))) {
            scratch.store_ad(271, &AdValue::square(scratch.ad_value(310)));
        }

        scratch.values[588] = if (scratch.values[310] > 0.0) { 1.0 } else { 0.0 };

        if ((((scratch.values[270] != 0.0) && (!(scratch.values[581] != 0.0))) && (!(scratch.values[586] != 0.0))) && (scratch.values[588] != 0.0)) {
            scratch.store_ad(272, &AdValue::div_from_scalar(1.0, AdValue::offset(AdValue::scale(scratch.ad_value(310), scratch.values[57]), 1.0)));
        }

        if ((((scratch.values[270] != 0.0) && (!(scratch.values[581] != 0.0))) && (!(scratch.values[586] != 0.0))) && (!(scratch.values[588] != 0.0))) {
            scratch.store_ad(272, &AdValue::div_from_scalar(1.0, AdValue::sub_from_scalar(1.0, AdValue::scale(scratch.ad_value(310), scratch.values[57]))));
        }

        scratch.values[589] = if (((-scratch.values[271]) + scratch.values[309]) > (-230.25850929940458)) { 1.0 } else { 0.0 };

        if ((((scratch.values[270] != 0.0) && (!(scratch.values[581] != 0.0))) && (!(scratch.values[586] != 0.0))) && (scratch.values[589] != 0.0)) {
            scratch.store_ad(289, &AdValue::exp(AdValue::sub(scratch.ad_value(309), scratch.ad_value(271))));
        }

        if ((((scratch.values[270] != 0.0) && (!(scratch.values[581] != 0.0))) && (!(scratch.values[586] != 0.0))) && (!(scratch.values[589] != 0.0))) {
            scratch.store_ad(289, &AdValue::div_from_scalar(1e-100, AdValue::offset(AdValue::mul(AdValue::sub_from_scalar((-230.25850929940458), AdValue::sub(scratch.ad_value(309), scratch.ad_value(271))), AdValue::offset(AdValue::scale(AdValue::mul(AdValue::sub_from_scalar((-230.25850929940458), AdValue::sub(scratch.ad_value(309), scratch.ad_value(271))), AdValue::offset(AdValue::scale(AdValue::sub_from_scalar((-230.25850929940458), AdValue::sub(scratch.ad_value(309), scratch.ad_value(271))), 0.3333333333333333), 1.0)), 0.5), 1.0)), 1.0)));
        }

        if (((scratch.values[270] != 0.0) && (!(scratch.values[581] != 0.0))) && (!(scratch.values[586] != 0.0))) {
            scratch.store_ad(273, &AdValue::mul(AdValue::add(AdValue::add(AdValue::scale(scratch.ad_value(272), 0.29214664), AdValue::scale(AdValue::square(scratch.ad_value(272)), scratch.values[58])), AdValue::scale(AdValue::mul(AdValue::square(scratch.ad_value(272)), scratch.ad_value(272)), scratch.values[59])), scratch.ad_value(289)));
        }

        scratch.values[590] = if (scratch.values[310] > 0.0) { 1.0 } else { 0.0 };

        if ((((scratch.values[270] != 0.0) && (!(scratch.values[581] != 0.0))) && (!(scratch.values[586] != 0.0))) && (scratch.values[590] != 0.0)) {
            scratch.values[311] = scratch.values[273];
            scratch.node_derivatives[311] = scratch.node_derivatives[273];
            scratch.branch_derivatives[311] = scratch.branch_derivatives[273];
        }

        scratch.values[591] = if (scratch.values[309] > (-230.25850929940458)) { 1.0 } else { 0.0 };

        if (((((scratch.values[270] != 0.0) && (!(scratch.values[581] != 0.0))) && (!(scratch.values[586] != 0.0))) && (!(scratch.values[590] != 0.0))) && (scratch.values[591] != 0.0)) {
            scratch.store_ad(289, &AdValue::exp(scratch.ad_value(309)));
        }

        if (((((scratch.values[270] != 0.0) && (!(scratch.values[581] != 0.0))) && (!(scratch.values[586] != 0.0))) && (!(scratch.values[590] != 0.0))) && (!(scratch.values[591] != 0.0))) {
            scratch.store_ad(289, &AdValue::div_from_scalar(1e-100, AdValue::offset(AdValue::mul(AdValue::sub_from_scalar((-230.25850929940458), scratch.ad_value(309)), AdValue::offset(AdValue::scale(AdValue::mul(AdValue::sub_from_scalar((-230.25850929940458), scratch.ad_value(309)), AdValue::offset(AdValue::scale(AdValue::sub_from_scalar((-230.25850929940458), scratch.ad_value(309)), 0.3333333333333333), 1.0)), 0.5), 1.0)), 1.0)));
        }

        if ((((scratch.values[270] != 0.0) && (!(scratch.values[581] != 0.0))) && (!(scratch.values[586] != 0.0))) && (!(scratch.values[590] != 0.0))) {
            scratch.store_ad(311, &AdValue::sub(AdValue::scale(scratch.ad_value(289), 2.0), scratch.ad_value(273)));
        }

        if (((scratch.values[270] != 0.0) && (!(scratch.values[581] != 0.0))) && (!(scratch.values[586] != 0.0))) {
            scratch.store_ad(312, &AdValue::scale(AdValue::div(AdValue::scale(scratch.ad_value(311), scratch.values[121]), scratch.ad_value(307)), (1.772453850905516 * 0.5)));
        }

        if (((scratch.values[270] != 0.0) && (!(scratch.values[581] != 0.0))) && (!(scratch.values[586] != 0.0))) {
            scratch.store_ad(298, &AdValue::scale(AdValue::mul(AdValue::mul(scratch.ad_value(297), scratch.ad_value(312)), scratch.ad_value(306)), scratch.values[26]));
        }

        scratch.values[592] = if (scratch.values[32] == 0.0) { 1.0 } else { 0.0 };

    }

    pub(super) fn stamp_transient_block_10(
        &mut self,
        ctx: &GeneratedEvalContext<'_>,
        stamper: &mut GeneratedStamper<'_>,
        scratch: &mut Scratch,
    ) {
        let _ = stamper;
        if (((scratch.values[270] != 0.0) && (!(scratch.values[581] != 0.0))) && (scratch.values[592] != 0.0)) {
            scratch.values[313] = 0.0;
            scratch.node_derivatives[313] = [0.0; Instance::NODE_COUNT];
            scratch.branch_derivatives[313] = [0.0; Instance::BRANCH_COUNT];
        }

        scratch.values[593] = if (scratch.values[12] == 0.5) { 1.0 } else { 0.0 };

        if ((((scratch.values[270] != 0.0) && (!(scratch.values[581] != 0.0))) && (!(scratch.values[592] != 0.0))) && (scratch.values[593] != 0.0)) {
            scratch.store_ad(289, &AdValue::sqrt(AdValue::scale(AdValue::sub_from_scalar(scratch.values[9], scratch.ad_value(287)), scratch.values[115])));
        }

        if ((((scratch.values[270] != 0.0) && (!(scratch.values[581] != 0.0))) && (!(scratch.values[592] != 0.0))) && (!(scratch.values[593] != 0.0))) {
            scratch.store_ad(289, &AdValue::powf(AdValue::scale(AdValue::sub_from_scalar(scratch.values[9], scratch.ad_value(287)), scratch.values[115]), scratch.values[12]));
        }

        if (((scratch.values[270] != 0.0) && (!(scratch.values[581] != 0.0))) && (!(scratch.values[592] != 0.0))) {
            scratch.store_ad(314, &AdValue::scale(AdValue::div(AdValue::scale(AdValue::sub_from_scalar(scratch.values[9], scratch.ad_value(287)), scratch.values[112]), scratch.ad_value(289)), scratch.values[97]));
        }

        scratch.values[594] = if (((((-scratch.values[127]) / scratch.values[314])) as f64).abs() < 230.25850929940458) { 1.0 } else { 0.0 };

        if ((((scratch.values[270] != 0.0) && (!(scratch.values[581] != 0.0))) && (!(scratch.values[592] != 0.0))) && (scratch.values[594] != 0.0)) {
            scratch.store_ad(289, &AdValue::exp(AdValue::div(AdValue::neg(scratch.ad_value(127)), scratch.ad_value(314))));
        }

        scratch.values[595] = if (((-scratch.values[127]) / scratch.values[314]) < 0.0) { 1.0 } else { 0.0 };

        if (((((scratch.values[270] != 0.0) && (!(scratch.values[581] != 0.0))) && (!(scratch.values[592] != 0.0))) && (!(scratch.values[594] != 0.0))) && (scratch.values[595] != 0.0)) {
            let assign14470_ad_e18651: AdValue = AdValue::offset(AdValue::mul(AdValue::sub_from_scalar((-230.25850929940458), AdValue::div(AdValue::neg(scratch.ad_value(127)), scratch.ad_value(314))), AdValue::offset(AdValue::scale(AdValue::mul(AdValue::sub_from_scalar((-230.25850929940458), AdValue::div(AdValue::neg(scratch.ad_value(127)), scratch.ad_value(314))), AdValue::offset(AdValue::scale(AdValue::sub_from_scalar((-230.25850929940458), AdValue::div(AdValue::neg(scratch.ad_value(127)), scratch.ad_value(314))), 0.3333333333333333), 1.0)), 0.5), 1.0)), 1.0);
            scratch.store_ad(289, &AdValue::div_from_scalar(1e-100, assign14470_ad_e18651));
        }

        if (((((scratch.values[270] != 0.0) && (!(scratch.values[581] != 0.0))) && (!(scratch.values[592] != 0.0))) && (!(scratch.values[594] != 0.0))) && (!(scratch.values[595] != 0.0))) {
            let assign14480_ad_e18699: AdValue = AdValue::scale(AdValue::offset(AdValue::mul(AdValue::offset(AdValue::div(AdValue::neg(scratch.ad_value(127)), scratch.ad_value(314)), (-230.25850929940458)), AdValue::offset(AdValue::scale(AdValue::mul(AdValue::offset(AdValue::div(AdValue::neg(scratch.ad_value(127)), scratch.ad_value(314)), (-230.25850929940458)), AdValue::offset(AdValue::scale(AdValue::offset(AdValue::div(AdValue::neg(scratch.ad_value(127)), scratch.ad_value(314)), (-230.25850929940458)), 0.3333333333333333), 1.0)), 0.5), 1.0)), 1.0), 1e100);
            scratch.store_ad(289, &assign14480_ad_e18699);
        }

        if (((scratch.values[270] != 0.0) && (!(scratch.values[581] != 0.0))) && (!(scratch.values[592] != 0.0))) {
            scratch.store_ad(313, &AdValue::scale(AdValue::mul(AdValue::mul(AdValue::mul(scratch.ad_value(156), scratch.ad_value(314)), scratch.ad_value(314)), scratch.ad_value(289)), scratch.values[32]));
        }

        scratch.values[596] = if (scratch.values[41] > 1000.0) { 1.0 } else { 0.0 };

        if (((scratch.values[270] != 0.0) && (!(scratch.values[581] != 0.0))) && (scratch.values[596] != 0.0)) {
            scratch.values[315] = 1.0;
            scratch.node_derivatives[315] = [0.0; Instance::NODE_COUNT];
            scratch.branch_derivatives[315] = [0.0; Instance::BRANCH_COUNT];
        }

        scratch.values[597] = if (scratch.values[288] > ((-scratch.values[129]) * scratch.values[41])) { 1.0 } else { 0.0 };

        scratch.values[598] = if (scratch.values[44] == 4.0) { 1.0 } else { 0.0 };

        if (((((scratch.values[270] != 0.0) && (!(scratch.values[581] != 0.0))) && (!(scratch.values[596] != 0.0))) && (scratch.values[597] != 0.0)) && (scratch.values[598] != 0.0)) {
            scratch.store_ad(289, &AdValue::mul(AdValue::mul(AdValue::mul(AdValue::scale(scratch.ad_value(288), scratch.values[134]), AdValue::scale(scratch.ad_value(288), scratch.values[134])), AdValue::scale(scratch.ad_value(288), scratch.values[134])), AdValue::scale(scratch.ad_value(288), scratch.values[134])));
        }

        if (((((scratch.values[270] != 0.0) && (!(scratch.values[581] != 0.0))) && (!(scratch.values[596] != 0.0))) && (scratch.values[597] != 0.0)) && (!(scratch.values[598] != 0.0))) {
            scratch.store_ad(289, &AdValue::powf(AdValue::abs(AdValue::scale(scratch.ad_value(288), scratch.values[134])), scratch.values[44]));
        }

        if ((((scratch.values[270] != 0.0) && (!(scratch.values[581] != 0.0))) && (!(scratch.values[596] != 0.0))) && (scratch.values[597] != 0.0)) {
            scratch.store_ad(315, &AdValue::div_from_scalar(1.0, AdValue::sub_from_scalar(1.0, scratch.ad_value(289))));
        }

        if ((((scratch.values[270] != 0.0) && (!(scratch.values[581] != 0.0))) && (!(scratch.values[596] != 0.0))) && (!(scratch.values[597] != 0.0))) {
            scratch.store_ad(315, &AdValue::offset(AdValue::scale(AdValue::offset(scratch.ad_value(288), (scratch.values[129] * scratch.values[41])), scratch.values[137]), scratch.values[131]));
        }

        if ((scratch.values[270] != 0.0) && (!(scratch.values[581] != 0.0))) {
            scratch.store_ad(317, &AdValue::mul(AdValue::add(AdValue::add(AdValue::add(scratch.ad_value(290), scratch.ad_value(291)), scratch.ad_value(298)), scratch.ad_value(313)), scratch.ad_value(315)));
        }

        scratch.values[599] = if (scratch.values[219] == 0.0) { 1.0 } else { 0.0 };

        if ((scratch.values[270] != 0.0) && (scratch.values[599] != 0.0)) {
            scratch.values[318] = 0.0;
            scratch.node_derivatives[318] = [0.0; Instance::NODE_COUNT];
            scratch.branch_derivatives[318] = [0.0; Instance::BRANCH_COUNT];
        }

        scratch.values[600] = if (scratch.values[95] == 0.5) { 1.0 } else { 0.0 };

        if (((scratch.values[270] != 0.0) && (!(scratch.values[599] != 0.0))) && (scratch.values[600] != 0.0)) {
            scratch.store_ad(289, &AdValue::sqrt(AdValue::sub_from_scalar(1.0, AdValue::scale(scratch.ad_value(281), scratch.values[92]))));
        }

        if (((scratch.values[270] != 0.0) && (!(scratch.values[599] != 0.0))) && (!(scratch.values[600] != 0.0))) {
            scratch.store_ad(289, &AdValue::powf(AdValue::sub_from_scalar(1.0, AdValue::scale(scratch.ad_value(281), scratch.values[92])), scratch.values[95]));
        }

        if ((scratch.values[270] != 0.0) && (!(scratch.values[599] != 0.0))) {
            scratch.store_ad(290, &AdValue::scale(scratch.ad_value(280), scratch.values[74]));
        }

        scratch.values[601] = if ((scratch.values[24] == 0.0) && (scratch.values[27] == 0.0)) { 1.0 } else { 0.0 };

        if (((scratch.values[270] != 0.0) && (!(scratch.values[599] != 0.0))) && (scratch.values[601] != 0.0)) {
            scratch.values[291] = 0.0;
            scratch.node_derivatives[291] = [0.0; Instance::NODE_COUNT];
            scratch.branch_derivatives[291] = [0.0; Instance::BRANCH_COUNT];
        }

        if (((scratch.values[270] != 0.0) && (!(scratch.values[599] != 0.0))) && (!(scratch.values[601] != 0.0))) {
            scratch.store_ad(292, &AdValue::sub_from_scalar(scratch.values[80], scratch.ad_value(286)));
        }

        if (((scratch.values[270] != 0.0) && (!(scratch.values[599] != 0.0))) && (!(scratch.values[601] != 0.0))) {
            scratch.store_ad(293, &AdValue::sub_from_scalar(1.0, AdValue::sqrt(AdValue::sub_from_scalar(1.0, AdValue::div(scratch.ad_value(284), scratch.ad_value(292))))));
        }

        scratch.values[602] = if (scratch.values[13] == 0.5) { 1.0 } else { 0.0 };

        if ((((scratch.values[270] != 0.0) && (!(scratch.values[599] != 0.0))) && (!(scratch.values[601] != 0.0))) && (scratch.values[602] != 0.0)) {
            scratch.values[294] = 0.0;
            scratch.node_derivatives[294] = [0.0; Instance::NODE_COUNT];
            scratch.branch_derivatives[294] = [0.0; Instance::BRANCH_COUNT];
        }

        if ((((scratch.values[270] != 0.0) && (!(scratch.values[599] != 0.0))) && (!(scratch.values[601] != 0.0))) && (!(scratch.values[602] != 0.0))) {
            scratch.store_ad(294, &AdValue::scale(AdValue::add(AdValue::div(AdValue::mul(AdValue::square(scratch.ad_value(293)), AdValue::ln(scratch.ad_value(293))), AdValue::sub_from_scalar(1.0, scratch.ad_value(293))), scratch.ad_value(293)), (1.0 - (2.0 * scratch.values[13]))));
        }

        if (((scratch.values[270] != 0.0) && (!(scratch.values[599] != 0.0))) && (!(scratch.values[601] != 0.0))) {
            scratch.store_ad(295, &AdValue::add(scratch.ad_value(293), scratch.ad_value(294)));
        }

        scratch.values[603] = if (scratch.values[13] == 0.5) { 1.0 } else { 0.0 };

        if ((((scratch.values[270] != 0.0) && (!(scratch.values[599] != 0.0))) && (!(scratch.values[601] != 0.0))) && (scratch.values[603] != 0.0)) {
            scratch.store_ad(289, &AdValue::sqrt(AdValue::scale(scratch.ad_value(292), scratch.values[116])));
        }

        if ((((scratch.values[270] != 0.0) && (!(scratch.values[599] != 0.0))) && (!(scratch.values[601] != 0.0))) && (!(scratch.values[603] != 0.0))) {
            scratch.store_ad(289, &AdValue::powf(AdValue::scale(scratch.ad_value(292), scratch.values[116]), scratch.values[13]));
        }

        if (((scratch.values[270] != 0.0) && (!(scratch.values[599] != 0.0))) && (!(scratch.values[601] != 0.0))) {
            scratch.store_ad(296, &AdValue::scale(scratch.ad_value(289), scratch.values[110]));
        }

        if (((scratch.values[270] != 0.0) && (!(scratch.values[599] != 0.0))) && (!(scratch.values[601] != 0.0))) {
            scratch.store_ad(297, &AdValue::scale(AdValue::mul(AdValue::offset(scratch.ad_value(283), (-1.0)), scratch.ad_value(296)), scratch.values[71]));
        }

        if (((scratch.values[270] != 0.0) && (!(scratch.values[599] != 0.0))) && (!(scratch.values[601] != 0.0))) {
            scratch.store_ad(291, &AdValue::scale(AdValue::mul(scratch.ad_value(297), scratch.ad_value(295)), scratch.values[24]));
        }

        scratch.values[604] = if (scratch.values[27] == 0.0) { 1.0 } else { 0.0 };

        if (((scratch.values[270] != 0.0) && (!(scratch.values[599] != 0.0))) && (scratch.values[604] != 0.0)) {
            scratch.values[298] = 0.0;
            scratch.node_derivatives[298] = [0.0; Instance::NODE_COUNT];
            scratch.branch_derivatives[298] = [0.0; Instance::BRANCH_COUNT];
        }

        if (((scratch.values[270] != 0.0) && (!(scratch.values[599] != 0.0))) && (!(scratch.values[604] != 0.0))) {
            scratch.store_ad(299, &AdValue::scale(AdValue::div(AdValue::scale(scratch.ad_value(296), scratch.values[95]), scratch.ad_value(292)), scratch.values[125]));
        }

        if (((scratch.values[270] != 0.0) && (!(scratch.values[599] != 0.0))) && (!(scratch.values[604] != 0.0))) {
            scratch.store_ad(300, &AdValue::div_from_scalar((0.666666666666667 * scratch.values[122]), scratch.ad_value(299)));
        }

        if (((scratch.values[270] != 0.0) && (!(scratch.values[599] != 0.0))) && (!(scratch.values[604] != 0.0))) {
            scratch.store_ad(301, &AdValue::square(scratch.ad_value(300)));
        }

        if (((scratch.values[270] != 0.0) && (!(scratch.values[599] != 0.0))) && (!(scratch.values[604] != 0.0))) {
            scratch.store_ad(302, &AdValue::sqrt(AdValue::div(AdValue::square(scratch.ad_value(301)), AdValue::offset(AdValue::square(scratch.ad_value(301)), 1.0))));
        }

        if (((scratch.values[270] != 0.0) && (!(scratch.values[599] != 0.0))) && (!(scratch.values[604] != 0.0))) {
            scratch.store_ad(303, &AdValue::sqrt(AdValue::abs(scratch.ad_value(302))));
        }

        if (((scratch.values[270] != 0.0) && (!(scratch.values[599] != 0.0))) && (!(scratch.values[604] != 0.0))) {
            scratch.store_ad(304, &AdValue::mul(scratch.ad_value(302), scratch.ad_value(303)));
        }

        scratch.values[605] = if (((-scratch.values[13]) * scratch.values[98]) == (-1.0)) { 1.0 } else { 0.0 };

        if ((((scratch.values[270] != 0.0) && (!(scratch.values[599] != 0.0))) && (!(scratch.values[604] != 0.0))) && (scratch.values[605] != 0.0)) {
            scratch.store_ad(305, &AdValue::div_from_scalar(1.0, AdValue::offset(AdValue::mul(scratch.ad_value(299), scratch.ad_value(304)), 1.0)));
        }

        if ((((scratch.values[270] != 0.0) && (!(scratch.values[599] != 0.0))) && (!(scratch.values[604] != 0.0))) && (!(scratch.values[605] != 0.0))) {
            scratch.store_ad(305, &AdValue::powf(AdValue::offset(AdValue::mul(scratch.ad_value(299), scratch.ad_value(304)), 1.0), ((-scratch.values[13]) * scratch.values[98])));
        }

        if (((scratch.values[270] != 0.0) && (!(scratch.values[599] != 0.0))) && (!(scratch.values[604] != 0.0))) {
            scratch.store_ad(306, &AdValue::div(AdValue::mul(scratch.ad_value(295), scratch.ad_value(305)), AdValue::add(scratch.ad_value(295), scratch.ad_value(305))));
        }

        if (((scratch.values[270] != 0.0) && (!(scratch.values[599] != 0.0))) && (!(scratch.values[604] != 0.0))) {
            scratch.store_ad(307, &AdValue::sqrt(AdValue::scale(AdValue::div(scratch.ad_value(299), scratch.ad_value(303)), 0.375)));
        }

        if (((scratch.values[270] != 0.0) && (!(scratch.values[599] != 0.0))) && (!(scratch.values[604] != 0.0))) {
            scratch.store_ad(308, &AdValue::sub(AdValue::scale(AdValue::mul(scratch.ad_value(300), scratch.ad_value(303)), 2.0), scratch.ad_value(302)));
        }

        if (((scratch.values[270] != 0.0) && (!(scratch.values[599] != 0.0))) && (!(scratch.values[604] != 0.0))) {
            scratch.store_ad(309, &AdValue::add(AdValue::sub(AdValue::mul(AdValue::scale(scratch.ad_value(300), scratch.values[122]), scratch.ad_value(303)), AdValue::scale(scratch.ad_value(302), scratch.values[122])), AdValue::scale(AdValue::mul(scratch.ad_value(299), scratch.ad_value(304)), 0.5)));
        }

        if (((scratch.values[270] != 0.0) && (!(scratch.values[599] != 0.0))) && (!(scratch.values[604] != 0.0))) {
            scratch.store_ad(310, &AdValue::mul(AdValue::offset(scratch.ad_value(308), (-1.0)), scratch.ad_value(307)));
        }

        if (((scratch.values[270] != 0.0) && (!(scratch.values[599] != 0.0))) && (!(scratch.values[604] != 0.0))) {
            scratch.store_ad(271, &AdValue::square(scratch.ad_value(310)));
        }

        scratch.values[606] = if (scratch.values[310] > 0.0) { 1.0 } else { 0.0 };

        if ((((scratch.values[270] != 0.0) && (!(scratch.values[599] != 0.0))) && (!(scratch.values[604] != 0.0))) && (scratch.values[606] != 0.0)) {
            scratch.store_ad(272, &AdValue::div_from_scalar(1.0, AdValue::offset(AdValue::scale(scratch.ad_value(310), scratch.values[57]), 1.0)));
        }

        if ((((scratch.values[270] != 0.0) && (!(scratch.values[599] != 0.0))) && (!(scratch.values[604] != 0.0))) && (!(scratch.values[606] != 0.0))) {
            scratch.store_ad(272, &AdValue::div_from_scalar(1.0, AdValue::sub_from_scalar(1.0, AdValue::scale(scratch.ad_value(310), scratch.values[57]))));
        }

        scratch.values[607] = if (((-scratch.values[271]) + scratch.values[309]) > (-230.25850929940458)) { 1.0 } else { 0.0 };

        if ((((scratch.values[270] != 0.0) && (!(scratch.values[599] != 0.0))) && (!(scratch.values[604] != 0.0))) && (scratch.values[607] != 0.0)) {
            scratch.store_ad(289, &AdValue::exp(AdValue::sub(scratch.ad_value(309), scratch.ad_value(271))));
        }

        if ((((scratch.values[270] != 0.0) && (!(scratch.values[599] != 0.0))) && (!(scratch.values[604] != 0.0))) && (!(scratch.values[607] != 0.0))) {
            scratch.store_ad(289, &AdValue::div_from_scalar(1e-100, AdValue::offset(AdValue::mul(AdValue::sub_from_scalar((-230.25850929940458), AdValue::sub(scratch.ad_value(309), scratch.ad_value(271))), AdValue::offset(AdValue::scale(AdValue::mul(AdValue::sub_from_scalar((-230.25850929940458), AdValue::sub(scratch.ad_value(309), scratch.ad_value(271))), AdValue::offset(AdValue::scale(AdValue::sub_from_scalar((-230.25850929940458), AdValue::sub(scratch.ad_value(309), scratch.ad_value(271))), 0.3333333333333333), 1.0)), 0.5), 1.0)), 1.0)));
        }

        if (((scratch.values[270] != 0.0) && (!(scratch.values[599] != 0.0))) && (!(scratch.values[604] != 0.0))) {
            scratch.store_ad(273, &AdValue::mul(AdValue::add(AdValue::add(AdValue::scale(scratch.ad_value(272), 0.29214664), AdValue::scale(AdValue::square(scratch.ad_value(272)), scratch.values[58])), AdValue::scale(AdValue::mul(AdValue::square(scratch.ad_value(272)), scratch.ad_value(272)), scratch.values[59])), scratch.ad_value(289)));
        }

        scratch.values[608] = if (scratch.values[310] > 0.0) { 1.0 } else { 0.0 };

        if ((((scratch.values[270] != 0.0) && (!(scratch.values[599] != 0.0))) && (!(scratch.values[604] != 0.0))) && (scratch.values[608] != 0.0)) {
            scratch.values[311] = scratch.values[273];
            scratch.node_derivatives[311] = scratch.node_derivatives[273];
            scratch.branch_derivatives[311] = scratch.branch_derivatives[273];
        }

        scratch.values[609] = if (scratch.values[309] > (-230.25850929940458)) { 1.0 } else { 0.0 };

        if (((((scratch.values[270] != 0.0) && (!(scratch.values[599] != 0.0))) && (!(scratch.values[604] != 0.0))) && (!(scratch.values[608] != 0.0))) && (scratch.values[609] != 0.0)) {
            scratch.store_ad(289, &AdValue::exp(scratch.ad_value(309)));
        }

        if (((((scratch.values[270] != 0.0) && (!(scratch.values[599] != 0.0))) && (!(scratch.values[604] != 0.0))) && (!(scratch.values[608] != 0.0))) && (!(scratch.values[609] != 0.0))) {
            scratch.store_ad(289, &AdValue::div_from_scalar(1e-100, AdValue::offset(AdValue::mul(AdValue::sub_from_scalar((-230.25850929940458), scratch.ad_value(309)), AdValue::offset(AdValue::scale(AdValue::mul(AdValue::sub_from_scalar((-230.25850929940458), scratch.ad_value(309)), AdValue::offset(AdValue::scale(AdValue::sub_from_scalar((-230.25850929940458), scratch.ad_value(309)), 0.3333333333333333), 1.0)), 0.5), 1.0)), 1.0)));
        }

        if ((((scratch.values[270] != 0.0) && (!(scratch.values[599] != 0.0))) && (!(scratch.values[604] != 0.0))) && (!(scratch.values[608] != 0.0))) {
            scratch.store_ad(311, &AdValue::sub(AdValue::scale(scratch.ad_value(289), 2.0), scratch.ad_value(273)));
        }

        if (((scratch.values[270] != 0.0) && (!(scratch.values[599] != 0.0))) && (!(scratch.values[604] != 0.0))) {
            scratch.store_ad(312, &AdValue::scale(AdValue::div(AdValue::scale(scratch.ad_value(311), scratch.values[122]), scratch.ad_value(307)), (1.772453850905516 * 0.5)));
        }

        if (((scratch.values[270] != 0.0) && (!(scratch.values[599] != 0.0))) && (!(scratch.values[604] != 0.0))) {
            scratch.store_ad(298, &AdValue::scale(AdValue::mul(AdValue::mul(scratch.ad_value(297), scratch.ad_value(312)), scratch.ad_value(306)), scratch.values[27]));
        }

        scratch.values[610] = if (scratch.values[33] == 0.0) { 1.0 } else { 0.0 };

        if (((scratch.values[270] != 0.0) && (!(scratch.values[599] != 0.0))) && (scratch.values[610] != 0.0)) {
            scratch.values[313] = 0.0;
            scratch.node_derivatives[313] = [0.0; Instance::NODE_COUNT];
            scratch.branch_derivatives[313] = [0.0; Instance::BRANCH_COUNT];
        }

        scratch.values[611] = if (scratch.values[13] == 0.5) { 1.0 } else { 0.0 };

        if ((((scratch.values[270] != 0.0) && (!(scratch.values[599] != 0.0))) && (!(scratch.values[610] != 0.0))) && (scratch.values[611] != 0.0)) {
            scratch.store_ad(289, &AdValue::sqrt(AdValue::scale(AdValue::sub_from_scalar(scratch.values[10], scratch.ad_value(287)), scratch.values[116])));
        }

        if ((((scratch.values[270] != 0.0) && (!(scratch.values[599] != 0.0))) && (!(scratch.values[610] != 0.0))) && (!(scratch.values[611] != 0.0))) {
            scratch.store_ad(289, &AdValue::powf(AdValue::scale(AdValue::sub_from_scalar(scratch.values[10], scratch.ad_value(287)), scratch.values[116]), scratch.values[13]));
        }

        if (((scratch.values[270] != 0.0) && (!(scratch.values[599] != 0.0))) && (!(scratch.values[610] != 0.0))) {
            scratch.store_ad(314, &AdValue::scale(AdValue::div(AdValue::scale(AdValue::sub_from_scalar(scratch.values[10], scratch.ad_value(287)), scratch.values[113]), scratch.ad_value(289)), scratch.values[98]));
        }

        scratch.values[612] = if (((((-scratch.values[128]) / scratch.values[314])) as f64).abs() < 230.25850929940458) { 1.0 } else { 0.0 };

        if ((((scratch.values[270] != 0.0) && (!(scratch.values[599] != 0.0))) && (!(scratch.values[610] != 0.0))) && (scratch.values[612] != 0.0)) {
            scratch.store_ad(289, &AdValue::exp(AdValue::div(AdValue::neg(scratch.ad_value(128)), scratch.ad_value(314))));
        }

        scratch.values[613] = if (((-scratch.values[128]) / scratch.values[314]) < 0.0) { 1.0 } else { 0.0 };

        if (((((scratch.values[270] != 0.0) && (!(scratch.values[599] != 0.0))) && (!(scratch.values[610] != 0.0))) && (!(scratch.values[612] != 0.0))) && (scratch.values[613] != 0.0)) {
            let assign15220_ad_e19743: AdValue = AdValue::offset(AdValue::mul(AdValue::sub_from_scalar((-230.25850929940458), AdValue::div(AdValue::neg(scratch.ad_value(128)), scratch.ad_value(314))), AdValue::offset(AdValue::scale(AdValue::mul(AdValue::sub_from_scalar((-230.25850929940458), AdValue::div(AdValue::neg(scratch.ad_value(128)), scratch.ad_value(314))), AdValue::offset(AdValue::scale(AdValue::sub_from_scalar((-230.25850929940458), AdValue::div(AdValue::neg(scratch.ad_value(128)), scratch.ad_value(314))), 0.3333333333333333), 1.0)), 0.5), 1.0)), 1.0);
            scratch.store_ad(289, &AdValue::div_from_scalar(1e-100, assign15220_ad_e19743));
        }

        if (((((scratch.values[270] != 0.0) && (!(scratch.values[599] != 0.0))) && (!(scratch.values[610] != 0.0))) && (!(scratch.values[612] != 0.0))) && (!(scratch.values[613] != 0.0))) {
            let assign15230_ad_e19791: AdValue = AdValue::scale(AdValue::offset(AdValue::mul(AdValue::offset(AdValue::div(AdValue::neg(scratch.ad_value(128)), scratch.ad_value(314)), (-230.25850929940458)), AdValue::offset(AdValue::scale(AdValue::mul(AdValue::offset(AdValue::div(AdValue::neg(scratch.ad_value(128)), scratch.ad_value(314)), (-230.25850929940458)), AdValue::offset(AdValue::scale(AdValue::offset(AdValue::div(AdValue::neg(scratch.ad_value(128)), scratch.ad_value(314)), (-230.25850929940458)), 0.3333333333333333), 1.0)), 0.5), 1.0)), 1.0), 1e100);
            scratch.store_ad(289, &assign15230_ad_e19791);
        }

        if (((scratch.values[270] != 0.0) && (!(scratch.values[599] != 0.0))) && (!(scratch.values[610] != 0.0))) {
            scratch.store_ad(313, &AdValue::scale(AdValue::mul(AdValue::mul(AdValue::mul(scratch.ad_value(156), scratch.ad_value(314)), scratch.ad_value(314)), scratch.ad_value(289)), scratch.values[33]));
        }

        scratch.values[614] = if (scratch.values[42] > 1000.0) { 1.0 } else { 0.0 };

        if (((scratch.values[270] != 0.0) && (!(scratch.values[599] != 0.0))) && (scratch.values[614] != 0.0)) {
            scratch.values[315] = 1.0;
            scratch.node_derivatives[315] = [0.0; Instance::NODE_COUNT];
            scratch.branch_derivatives[315] = [0.0; Instance::BRANCH_COUNT];
        }

        scratch.values[615] = if (scratch.values[288] > ((-scratch.values[129]) * scratch.values[42])) { 1.0 } else { 0.0 };

        scratch.values[616] = if (scratch.values[45] == 4.0) { 1.0 } else { 0.0 };

        if (((((scratch.values[270] != 0.0) && (!(scratch.values[599] != 0.0))) && (!(scratch.values[614] != 0.0))) && (scratch.values[615] != 0.0)) && (scratch.values[616] != 0.0)) {
            scratch.store_ad(289, &AdValue::mul(AdValue::mul(AdValue::mul(AdValue::scale(scratch.ad_value(288), scratch.values[135]), AdValue::scale(scratch.ad_value(288), scratch.values[135])), AdValue::scale(scratch.ad_value(288), scratch.values[135])), AdValue::scale(scratch.ad_value(288), scratch.values[135])));
        }

        if (((((scratch.values[270] != 0.0) && (!(scratch.values[599] != 0.0))) && (!(scratch.values[614] != 0.0))) && (scratch.values[615] != 0.0)) && (!(scratch.values[616] != 0.0))) {
            scratch.store_ad(289, &AdValue::powf(AdValue::abs(AdValue::scale(scratch.ad_value(288), scratch.values[135])), scratch.values[45]));
        }

        if ((((scratch.values[270] != 0.0) && (!(scratch.values[599] != 0.0))) && (!(scratch.values[614] != 0.0))) && (scratch.values[615] != 0.0)) {
            scratch.store_ad(315, &AdValue::div_from_scalar(1.0, AdValue::sub_from_scalar(1.0, scratch.ad_value(289))));
        }

        if ((((scratch.values[270] != 0.0) && (!(scratch.values[599] != 0.0))) && (!(scratch.values[614] != 0.0))) && (!(scratch.values[615] != 0.0))) {
            scratch.store_ad(315, &AdValue::offset(AdValue::scale(AdValue::offset(scratch.ad_value(288), (scratch.values[129] * scratch.values[42])), scratch.values[138]), scratch.values[132]));
        }

        if ((scratch.values[270] != 0.0) && (!(scratch.values[599] != 0.0))) {
            scratch.store_ad(318, &AdValue::mul(AdValue::add(AdValue::add(AdValue::add(scratch.ad_value(290), scratch.ad_value(291)), scratch.ad_value(298)), scratch.ad_value(313)), scratch.ad_value(315)));
        }

        if (scratch.values[270] != 0.0) {
            scratch.store_ad(146, &AdValue::add(AdValue::add(AdValue::scale(scratch.ad_value(316), scratch.values[217]), AdValue::scale(scratch.ad_value(317), scratch.values[218])), AdValue::scale(scratch.ad_value(318), scratch.values[219])));
        }

        if (scratch.values[270] != 0.0) {
            scratch.values[235] = (((scratch.values[217] * scratch.values[72]) + (scratch.values[218] * scratch.values[73])) + (scratch.values[219] * scratch.values[74]));
            scratch.node_derivatives[235] = [0.0; Instance::NODE_COUNT];
            scratch.branch_derivatives[235] = [0.0; Instance::BRANCH_COUNT];
        }

        if (scratch.values[270] != 0.0) {
            scratch.store_ad(150, &AdValue::sub(scratch.ad_value(145), AdValue::mul(scratch.ad_value(235), AdValue::offset(AdValue::exp(AdValue::scale(scratch.ad_value(155), (scratch.values[56] * scratch.values[236]))), (-1.0)))));
        }

        if (scratch.values[270] != 0.0) {
            scratch.store_ad(151, &AdValue::sub(scratch.ad_value(146), AdValue::mul(scratch.ad_value(235), AdValue::offset(AdValue::exp(AdValue::scale(scratch.ad_value(156), (scratch.values[56] * scratch.values[236]))), (-1.0)))));
        }

        scratch.values[617] = if !(((scratch.values[217] == 0.0) && (scratch.values[218] == 0.0)) && (scratch.values[219] == 0.0)) { 1.0 } else { 0.0 };

        scratch.values[618] = if ((scratch.values[145] > 0.0) && (scratch.values[146] > 0.0)) { 1.0 } else { 0.0 };

        scratch.values[619] = if (((((scratch.values[150] / scratch.values[145]) > 0.001) || ((scratch.values[151] / scratch.values[146]) > 0.001)) && (scratch.values[150] > 0.0)) && (scratch.values[151] > 0.0)) { 1.0 } else { 0.0 };

        if ((((scratch.values[270] != 0.0) && (scratch.values[617] != 0.0)) && (scratch.values[618] != 0.0)) && (scratch.values[619] != 0.0)) {
            scratch.store_ad(157, &AdValue::div(scratch.ad_value(150), scratch.ad_value(151)));
        }

        if ((((scratch.values[270] != 0.0) && (scratch.values[617] != 0.0)) && (scratch.values[618] != 0.0)) && (scratch.values[619] != 0.0)) {
            scratch.store_ad(238, &AdValue::div(AdValue::scale(AdValue::ln(scratch.ad_value(157)), scratch.values[55]), AdValue::sub(scratch.ad_value(155), scratch.ad_value(156))));
        }

        if ((((scratch.values[270] != 0.0) && (scratch.values[617] != 0.0)) && (scratch.values[618] != 0.0)) && (scratch.values[619] != 0.0)) {
            scratch.store_ad(237, &AdValue::div(scratch.ad_value(150), AdValue::offset(AdValue::exp(AdValue::mul(AdValue::scale(scratch.ad_value(155), scratch.values[56]), scratch.ad_value(238))), (-1.0))));
        }

        if ((scratch.values[270] != 0.0) && (scratch.values[617] != 0.0)) {
            scratch.store_ad(147, &AdValue::sub(AdValue::sub(scratch.ad_value(142), AdValue::mul(scratch.ad_value(235), AdValue::offset(AdValue::exp(AdValue::scale(scratch.ad_value(152), (scratch.values[56] * scratch.values[236]))), (-1.0)))), AdValue::mul(scratch.ad_value(237), AdValue::offset(AdValue::exp(AdValue::mul(AdValue::scale(scratch.ad_value(152), scratch.values[56]), scratch.ad_value(238))), (-1.0)))));
        }

        if ((scratch.values[270] != 0.0) && (scratch.values[617] != 0.0)) {
            scratch.store_ad(148, &AdValue::sub(AdValue::sub(scratch.ad_value(143), AdValue::mul(scratch.ad_value(235), AdValue::offset(AdValue::exp(AdValue::scale(scratch.ad_value(153), (scratch.values[56] * scratch.values[236]))), (-1.0)))), AdValue::mul(scratch.ad_value(237), AdValue::offset(AdValue::exp(AdValue::mul(AdValue::scale(scratch.ad_value(153), scratch.values[56]), scratch.ad_value(238))), (-1.0)))));
        }

        if ((scratch.values[270] != 0.0) && (scratch.values[617] != 0.0)) {
            scratch.store_ad(149, &AdValue::sub(AdValue::sub(scratch.ad_value(144), AdValue::mul(scratch.ad_value(235), AdValue::offset(AdValue::exp(AdValue::scale(scratch.ad_value(154), (scratch.values[56] * scratch.values[236]))), (-1.0)))), AdValue::mul(scratch.ad_value(237), AdValue::offset(AdValue::exp(AdValue::mul(AdValue::scale(scratch.ad_value(154), scratch.values[56]), scratch.ad_value(238))), (-1.0)))));
        }

        scratch.values[620] = if (((scratch.values[142] < 0.0) && (scratch.values[143] < 0.0)) && (scratch.values[144] < 0.0)) { 1.0 } else { 0.0 };

        scratch.values[621] = if (((((((scratch.values[147] / scratch.values[142]) > 0.001) || ((scratch.values[148] / scratch.values[143]) > 0.001)) || ((scratch.values[149] / scratch.values[144]) > 0.001)) && (scratch.values[147] < 0.0)) && (scratch.values[148] < 0.0)) && (scratch.values[149] < 0.0)) { 1.0 } else { 0.0 };

        if ((((scratch.values[270] != 0.0) && (scratch.values[617] != 0.0)) && (scratch.values[620] != 0.0)) && (scratch.values[621] != 0.0)) {
            scratch.store_ad(157, &AdValue::div(scratch.ad_value(147), scratch.ad_value(148)));
        }

        if ((((scratch.values[270] != 0.0) && (scratch.values[617] != 0.0)) && (scratch.values[620] != 0.0)) && (scratch.values[621] != 0.0)) {
            scratch.store_ad(158, &AdValue::div(AdValue::scale(AdValue::ln(scratch.ad_value(157)), (-scratch.values[55])), AdValue::sub(scratch.ad_value(152), scratch.ad_value(153))));
        }

        if ((((scratch.values[270] != 0.0) && (scratch.values[617] != 0.0)) && (scratch.values[620] != 0.0)) && (scratch.values[621] != 0.0)) {
            scratch.store_ad(160, &AdValue::div(scratch.ad_value(153), AdValue::sub(scratch.ad_value(153), scratch.ad_value(152))));
        }

        if ((((scratch.values[270] != 0.0) && (scratch.values[617] != 0.0)) && (scratch.values[620] != 0.0)) && (scratch.values[621] != 0.0)) {
            scratch.store_ad(161, &AdValue::mul(AdValue::scale(AdValue::offset(scratch.ad_value(157), (-1.0)), scratch.values[55]), AdValue::offset(AdValue::pow(scratch.ad_value(157), scratch.ad_value(160)), (-1.0))));
        }

        if ((((scratch.values[270] != 0.0) && (scratch.values[617] != 0.0)) && (scratch.values[620] != 0.0)) && (scratch.values[621] != 0.0)) {
            scratch.store_ad(160, &AdValue::div(scratch.ad_value(152), AdValue::sub(scratch.ad_value(152), scratch.ad_value(153))));
        }

        if ((((scratch.values[270] != 0.0) && (scratch.values[617] != 0.0)) && (scratch.values[620] != 0.0)) && (scratch.values[621] != 0.0)) {
            scratch.store_ad(162, &AdValue::sub(AdValue::add(AdValue::mul(AdValue::pow(scratch.ad_value(157), scratch.ad_value(160)), AdValue::sub(scratch.ad_value(153), scratch.ad_value(152))), AdValue::mul(scratch.ad_value(157), scratch.ad_value(152))), scratch.ad_value(153)));
        }

        if ((((scratch.values[270] != 0.0) && (scratch.values[617] != 0.0)) && (scratch.values[620] != 0.0)) && (scratch.values[621] != 0.0)) {
            scratch.store_ad(159, &AdValue::div(scratch.ad_value(161), scratch.ad_value(162)));
        }

        if ((((scratch.values[270] != 0.0) && (scratch.values[617] != 0.0)) && (scratch.values[620] != 0.0)) && (scratch.values[621] != 0.0)) {
            scratch.store_ad(240, &AdValue::add(scratch.ad_value(158), scratch.ad_value(159)));
        }

        scratch.values[622] = if (((((scratch.values[154] * scratch.values[56]) * scratch.values[240])) as f64).abs() < 1e-6) { 1.0 } else { 0.0 };

        if (((((scratch.values[270] != 0.0) && (scratch.values[617] != 0.0)) && (scratch.values[620] != 0.0)) && (scratch.values[621] != 0.0)) && (scratch.values[622] != 0.0)) {
            scratch.values[241] = 1.0;
            scratch.node_derivatives[241] = [0.0; Instance::NODE_COUNT];
            scratch.branch_derivatives[241] = [0.0; Instance::BRANCH_COUNT];
        }

        if (((((scratch.values[270] != 0.0) && (scratch.values[617] != 0.0)) && (scratch.values[620] != 0.0)) && (scratch.values[621] != 0.0)) && (scratch.values[622] != 0.0)) {
            scratch.store_ad(239, &AdValue::mul(scratch.ad_value(149), AdValue::add(AdValue::div_from_scalar(1.0, scratch.ad_value(154)), AdValue::scale(scratch.ad_value(240), (0.5 * scratch.values[56])))));
        }

        if (((((scratch.values[270] != 0.0) && (scratch.values[617] != 0.0)) && (scratch.values[620] != 0.0)) && (scratch.values[621] != 0.0)) && (scratch.values[622] != 0.0)) {
            scratch.store_ad(240, &AdValue::div(AdValue::scale(AdValue::mul(AdValue::scale(scratch.ad_value(149), (-0.5)), scratch.ad_value(240)), scratch.values[56]), scratch.ad_value(154)));
        }

        if (((((scratch.values[270] != 0.0) && (scratch.values[617] != 0.0)) && (scratch.values[620] != 0.0)) && (scratch.values[621] != 0.0)) && (!(scratch.values[622] != 0.0))) {
            scratch.values[241] = 0.0;
            scratch.node_derivatives[241] = [0.0; Instance::NODE_COUNT];
            scratch.branch_derivatives[241] = [0.0; Instance::BRANCH_COUNT];
        }

        if (((((scratch.values[270] != 0.0) && (scratch.values[617] != 0.0)) && (scratch.values[620] != 0.0)) && (scratch.values[621] != 0.0)) && (!(scratch.values[622] != 0.0))) {
            scratch.store_ad(239, &AdValue::div(AdValue::neg(scratch.ad_value(149)), AdValue::offset(AdValue::exp(AdValue::mul(AdValue::scale(AdValue::neg(scratch.ad_value(154)), scratch.values[56]), scratch.ad_value(240))), (-1.0))));
        }

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

        scratch.values[626] = if !(((scratch.values[217] == 0.0) && (scratch.values[218] == 0.0)) && (scratch.values[219] == 0.0)) { 1.0 } else { 0.0 };

        if ((scratch.values[270] != 0.0) && (scratch.values[626] != 0.0)) {
            scratch.store_ad(229, &AdValue::ln(AdValue::div_from_scalar((0.5 * scratch.values[3]), AdValue::offset(scratch.ad_value(235), 1e-21))));
        }

        if ((scratch.values[270] != 0.0) && (scratch.values[626] != 0.0)) {
            scratch.store_ad(231, &AdValue::ln(AdValue::div_from_scalar((0.5 * scratch.values[3]), AdValue::offset(scratch.ad_value(237), 1e-21))));
        }

        if ((scratch.values[270] != 0.0) && (scratch.values[626] != 0.0)) {
            scratch.store_ad(233, &AdValue::ln(AdValue::div_from_scalar((0.5 * scratch.values[3]), AdValue::offset(AdValue::abs(scratch.ad_value(239)), 1e-21))));
        }

        if (scratch.values[270] != 0.0) {
            scratch.store_ad(229, &AdValue::min_with_scalar(scratch.ad_value(229), 230.25850929940458));
        }

        if (scratch.values[270] != 0.0) {
            scratch.store_ad(230, &AdValue::exp(scratch.ad_value(229)));
        }

        if (scratch.values[270] != 0.0) {
            scratch.store_ad(231, &AdValue::min_with_scalar(scratch.ad_value(231), 230.25850929940458));
        }

    }

    pub(super) fn stamp_transient_block_11(
        &mut self,
        ctx: &GeneratedEvalContext<'_>,
        stamper: &mut GeneratedStamper<'_>,
        scratch: &mut Scratch,
    ) {
        let _ = stamper;
        if (scratch.values[270] != 0.0) {
            scratch.store_ad(232, &AdValue::exp(scratch.ad_value(231)));
        }

        if (scratch.values[270] != 0.0) {
            scratch.store_ad(233, &AdValue::min_with_scalar(scratch.ad_value(233), 230.25850929940458));
        }

        if (scratch.values[270] != 0.0) {
            scratch.store_ad(234, &AdValue::exp(scratch.ad_value(233)));
        }

        scratch.values[633] = 0.0;

        scratch.values[627] = 0.0;

        scratch.values[629] = 0.0;

        scratch.values[631] = 0.0;

        scratch.store_ad(636, &AdValue::scale(AdValue::voltage(ctx, &self.nodes, Some(0), Some(1)), self.params.type_));

        scratch.values[637] = if (scratch.values[46] == 1.0) { 1.0 } else { 0.0 };

        if (scratch.values[637] != 0.0) {
            scratch.store_ad(163, &AdValue::scale(scratch.ad_value(636), (scratch.values[56] * scratch.values[236])));
        }

        if (scratch.values[637] != 0.0) {
            let assign15870_ad_e20603: AdValue = {
                if (scratch.values[163] < (-230.25850929940458)) {
                    AdValue::div_from_scalar(1e-100, AdValue::offset(AdValue::sub_from_scalar((-230.25850929940458), scratch.ad_value(163)), 1.0))
                } else {
                    {
                        if (scratch.values[163] > scratch.values[229]) {
                            AdValue::mul(scratch.ad_value(230), AdValue::offset(AdValue::sub(scratch.ad_value(163), scratch.ad_value(229)), 1.0))
                        } else {
                            AdValue::exp(scratch.ad_value(163))
                        }
                    }
                }
            };
            scratch.store_ad(164, &assign15870_ad_e20603);
        }

        if (scratch.values[637] != 0.0) {
            scratch.store_ad(169, &AdValue::mul(scratch.ad_value(235), AdValue::offset(scratch.ad_value(164), (-1.0))));
        }

        if (scratch.values[637] != 0.0) {
            scratch.store_ad(163, &AdValue::mul(AdValue::scale(scratch.ad_value(636), scratch.values[56]), scratch.ad_value(238)));
        }

        if (scratch.values[637] != 0.0) {
            let assign15900_ad_e20648: AdValue = {
                if (scratch.values[163] < (-230.25850929940458)) {
                    AdValue::div_from_scalar(1e-100, AdValue::offset(AdValue::sub_from_scalar((-230.25850929940458), scratch.ad_value(163)), 1.0))
                } else {
                    {
                        if (scratch.values[163] > scratch.values[231]) {
                            AdValue::mul(scratch.ad_value(232), AdValue::offset(AdValue::sub(scratch.ad_value(163), scratch.ad_value(231)), 1.0))
                        } else {
                            AdValue::exp(scratch.ad_value(163))
                        }
                    }
                }
            };
            scratch.store_ad(164, &assign15900_ad_e20648);
        }

        if (scratch.values[637] != 0.0) {
            scratch.store_ad(170, &AdValue::mul(scratch.ad_value(237), AdValue::offset(scratch.ad_value(164), (-1.0))));
        }

        if (scratch.values[637] != 0.0) {
            scratch.values[171] = 0.0;
            scratch.node_derivatives[171] = [0.0; Instance::NODE_COUNT];
            scratch.branch_derivatives[171] = [0.0; Instance::BRANCH_COUNT];
        }

        scratch.values[638] = if (scratch.values[241] > 0.0) { 1.0 } else { 0.0 };

        if ((scratch.values[637] != 0.0) && (scratch.values[638] != 0.0)) {
            scratch.store_ad(171, &AdValue::mul(scratch.ad_value(636), AdValue::add(scratch.ad_value(239), AdValue::mul(scratch.ad_value(636), scratch.ad_value(240)))));
        }

        if ((scratch.values[637] != 0.0) && (!(scratch.values[638] != 0.0))) {
            scratch.store_ad(163, &AdValue::mul(AdValue::scale(AdValue::neg(scratch.ad_value(636)), scratch.values[56]), scratch.ad_value(240)));
        }

        if ((scratch.values[637] != 0.0) && (!(scratch.values[638] != 0.0))) {
            let assign15960_ad_e20719: AdValue = {
                if (scratch.values[163] < (-230.25850929940458)) {
                    AdValue::div_from_scalar(1e-100, AdValue::offset(AdValue::sub_from_scalar((-230.25850929940458), scratch.ad_value(163)), 1.0))
                } else {
                    {
                        if (scratch.values[163] > scratch.values[233]) {
                            AdValue::mul(scratch.ad_value(234), AdValue::offset(AdValue::sub(scratch.ad_value(163), scratch.ad_value(233)), 1.0))
                        } else {
                            AdValue::exp(scratch.ad_value(163))
                        }
                    }
                }
            };
            scratch.store_ad(164, &assign15960_ad_e20719);
        }

        if ((scratch.values[637] != 0.0) && (!(scratch.values[638] != 0.0))) {
            scratch.store_ad(171, &AdValue::mul(AdValue::neg(scratch.ad_value(239)), AdValue::offset(scratch.ad_value(164), (-1.0))));
        }

        if (scratch.values[637] != 0.0) {
            scratch.store_ad(633, &AdValue::add(AdValue::add(scratch.ad_value(169), scratch.ad_value(170)), scratch.ad_value(171)));
        }

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
            scratch.values[627] = 0.0;
            scratch.node_derivatives[627] = [0.0; Instance::NODE_COUNT];
            scratch.branch_derivatives[627] = [0.0; Instance::BRANCH_COUNT];
        }

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

        if ((!(scratch.values[637] != 0.0)) && (!(scratch.values[652] != 0.0))) {
            scratch.store_ad(191, &AdValue::scale(scratch.ad_value(181), scratch.values[72]));
        }

        scratch.values[654] = if ((scratch.values[22] == 0.0) && (scratch.values[25] == 0.0)) { 1.0 } else { 0.0 };

        if (((!(scratch.values[637] != 0.0)) && (!(scratch.values[652] != 0.0))) && (scratch.values[654] != 0.0)) {
            scratch.values[192] = 0.0;
            scratch.node_derivatives[192] = [0.0; Instance::NODE_COUNT];
            scratch.branch_derivatives[192] = [0.0; Instance::BRANCH_COUNT];
        }

        if (((!(scratch.values[637] != 0.0)) && (!(scratch.values[652] != 0.0))) && (!(scratch.values[654] != 0.0))) {
            scratch.store_ad(193, &AdValue::sub_from_scalar(scratch.values[78], scratch.ad_value(187)));
        }

        if (((!(scratch.values[637] != 0.0)) && (!(scratch.values[652] != 0.0))) && (!(scratch.values[654] != 0.0))) {
            scratch.store_ad(194, &AdValue::sub_from_scalar(1.0, AdValue::sqrt(AdValue::sub_from_scalar(1.0, AdValue::div(scratch.ad_value(185), scratch.ad_value(193))))));
        }

        scratch.values[655] = if (scratch.values[11] == 0.5) { 1.0 } else { 0.0 };

        if ((((!(scratch.values[637] != 0.0)) && (!(scratch.values[652] != 0.0))) && (!(scratch.values[654] != 0.0))) && (scratch.values[655] != 0.0)) {
            scratch.values[195] = 0.0;
            scratch.node_derivatives[195] = [0.0; Instance::NODE_COUNT];
            scratch.branch_derivatives[195] = [0.0; Instance::BRANCH_COUNT];
        }

        if ((((!(scratch.values[637] != 0.0)) && (!(scratch.values[652] != 0.0))) && (!(scratch.values[654] != 0.0))) && (!(scratch.values[655] != 0.0))) {
            scratch.store_ad(195, &AdValue::scale(AdValue::add(AdValue::div(AdValue::mul(AdValue::square(scratch.ad_value(194)), AdValue::ln(scratch.ad_value(194))), AdValue::sub_from_scalar(1.0, scratch.ad_value(194))), scratch.ad_value(194)), (1.0 - (2.0 * scratch.values[11]))));
        }

        if (((!(scratch.values[637] != 0.0)) && (!(scratch.values[652] != 0.0))) && (!(scratch.values[654] != 0.0))) {
            scratch.store_ad(196, &AdValue::add(scratch.ad_value(194), scratch.ad_value(195)));
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

        if (((!(scratch.values[637] != 0.0)) && (!(scratch.values[652] != 0.0))) && (!(scratch.values[654] != 0.0))) {
            scratch.store_ad(198, &AdValue::scale(AdValue::mul(AdValue::offset(scratch.ad_value(184), (-1.0)), scratch.ad_value(197)), scratch.values[69]));
        }

        if (((!(scratch.values[637] != 0.0)) && (!(scratch.values[652] != 0.0))) && (!(scratch.values[654] != 0.0))) {
            scratch.store_ad(192, &AdValue::scale(AdValue::mul(scratch.ad_value(198), scratch.ad_value(196)), scratch.values[22]));
        }

        scratch.values[657] = if (scratch.values[25] == 0.0) { 1.0 } else { 0.0 };

        if (((!(scratch.values[637] != 0.0)) && (!(scratch.values[652] != 0.0))) && (scratch.values[657] != 0.0)) {
            scratch.values[199] = 0.0;
            scratch.node_derivatives[199] = [0.0; Instance::NODE_COUNT];
            scratch.branch_derivatives[199] = [0.0; Instance::BRANCH_COUNT];
        }

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

        scratch.values[658] = if (((-scratch.values[11]) * scratch.values[96]) == (-1.0)) { 1.0 } else { 0.0 };

        if ((((!(scratch.values[637] != 0.0)) && (!(scratch.values[652] != 0.0))) && (!(scratch.values[657] != 0.0))) && (scratch.values[658] != 0.0)) {
            scratch.store_ad(206, &AdValue::div_from_scalar(1.0, AdValue::offset(AdValue::mul(scratch.ad_value(200), scratch.ad_value(205)), 1.0)));
        }

        if ((((!(scratch.values[637] != 0.0)) && (!(scratch.values[652] != 0.0))) && (!(scratch.values[657] != 0.0))) && (!(scratch.values[658] != 0.0))) {
            scratch.store_ad(206, &AdValue::powf(AdValue::offset(AdValue::mul(scratch.ad_value(200), scratch.ad_value(205)), 1.0), ((-scratch.values[11]) * scratch.values[96])));
        }

        if (((!(scratch.values[637] != 0.0)) && (!(scratch.values[652] != 0.0))) && (!(scratch.values[657] != 0.0))) {
            scratch.store_ad(207, &AdValue::div(AdValue::mul(scratch.ad_value(196), scratch.ad_value(206)), AdValue::add(scratch.ad_value(196), scratch.ad_value(206))));
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

        scratch.values[659] = if (scratch.values[211] > 0.0) { 1.0 } else { 0.0 };

        if ((((!(scratch.values[637] != 0.0)) && (!(scratch.values[652] != 0.0))) && (!(scratch.values[657] != 0.0))) && (scratch.values[659] != 0.0)) {
            scratch.store_ad(173, &AdValue::div_from_scalar(1.0, AdValue::offset(AdValue::scale(scratch.ad_value(211), scratch.values[57]), 1.0)));
        }

        if ((((!(scratch.values[637] != 0.0)) && (!(scratch.values[652] != 0.0))) && (!(scratch.values[657] != 0.0))) && (!(scratch.values[659] != 0.0))) {
            scratch.store_ad(173, &AdValue::div_from_scalar(1.0, AdValue::sub_from_scalar(1.0, AdValue::scale(scratch.ad_value(211), scratch.values[57]))));
        }

        scratch.values[660] = if (((-scratch.values[172]) + scratch.values[210]) > (-230.25850929940458)) { 1.0 } else { 0.0 };

        if ((((!(scratch.values[637] != 0.0)) && (!(scratch.values[652] != 0.0))) && (!(scratch.values[657] != 0.0))) && (scratch.values[660] != 0.0)) {
            scratch.store_ad(190, &AdValue::exp(AdValue::sub(scratch.ad_value(210), scratch.ad_value(172))));
        }

        if ((((!(scratch.values[637] != 0.0)) && (!(scratch.values[652] != 0.0))) && (!(scratch.values[657] != 0.0))) && (!(scratch.values[660] != 0.0))) {
            scratch.store_ad(190, &AdValue::div_from_scalar(1e-100, AdValue::offset(AdValue::mul(AdValue::sub_from_scalar((-230.25850929940458), AdValue::sub(scratch.ad_value(210), scratch.ad_value(172))), AdValue::offset(AdValue::scale(AdValue::mul(AdValue::sub_from_scalar((-230.25850929940458), AdValue::sub(scratch.ad_value(210), scratch.ad_value(172))), AdValue::offset(AdValue::scale(AdValue::sub_from_scalar((-230.25850929940458), AdValue::sub(scratch.ad_value(210), scratch.ad_value(172))), 0.3333333333333333), 1.0)), 0.5), 1.0)), 1.0)));
        }

        if (((!(scratch.values[637] != 0.0)) && (!(scratch.values[652] != 0.0))) && (!(scratch.values[657] != 0.0))) {
            scratch.store_ad(174, &AdValue::mul(AdValue::add(AdValue::add(AdValue::scale(scratch.ad_value(173), 0.29214664), AdValue::scale(AdValue::square(scratch.ad_value(173)), scratch.values[58])), AdValue::scale(AdValue::mul(AdValue::square(scratch.ad_value(173)), scratch.ad_value(173)), scratch.values[59])), scratch.ad_value(190)));
        }

        scratch.values[661] = if (scratch.values[211] > 0.0) { 1.0 } else { 0.0 };

        if ((((!(scratch.values[637] != 0.0)) && (!(scratch.values[652] != 0.0))) && (!(scratch.values[657] != 0.0))) && (scratch.values[661] != 0.0)) {
            scratch.values[212] = scratch.values[174];
            scratch.node_derivatives[212] = scratch.node_derivatives[174];
            scratch.branch_derivatives[212] = scratch.branch_derivatives[174];
        }

        scratch.values[662] = if (scratch.values[210] > (-230.25850929940458)) { 1.0 } else { 0.0 };

        if (((((!(scratch.values[637] != 0.0)) && (!(scratch.values[652] != 0.0))) && (!(scratch.values[657] != 0.0))) && (!(scratch.values[661] != 0.0))) && (scratch.values[662] != 0.0)) {
            scratch.store_ad(190, &AdValue::exp(scratch.ad_value(210)));
        }

        if (((((!(scratch.values[637] != 0.0)) && (!(scratch.values[652] != 0.0))) && (!(scratch.values[657] != 0.0))) && (!(scratch.values[661] != 0.0))) && (!(scratch.values[662] != 0.0))) {
            scratch.store_ad(190, &AdValue::div_from_scalar(1e-100, AdValue::offset(AdValue::mul(AdValue::sub_from_scalar((-230.25850929940458), scratch.ad_value(210)), AdValue::offset(AdValue::scale(AdValue::mul(AdValue::sub_from_scalar((-230.25850929940458), scratch.ad_value(210)), AdValue::offset(AdValue::scale(AdValue::sub_from_scalar((-230.25850929940458), scratch.ad_value(210)), 0.3333333333333333), 1.0)), 0.5), 1.0)), 1.0)));
        }

        if ((((!(scratch.values[637] != 0.0)) && (!(scratch.values[652] != 0.0))) && (!(scratch.values[657] != 0.0))) && (!(scratch.values[661] != 0.0))) {
            scratch.store_ad(212, &AdValue::sub(AdValue::scale(scratch.ad_value(190), 2.0), scratch.ad_value(174)));
        }

        if (((!(scratch.values[637] != 0.0)) && (!(scratch.values[652] != 0.0))) && (!(scratch.values[657] != 0.0))) {
            scratch.store_ad(213, &AdValue::scale(AdValue::div(AdValue::scale(scratch.ad_value(212), scratch.values[120]), scratch.ad_value(208)), (1.772453850905516 * 0.5)));
        }

    }
}
