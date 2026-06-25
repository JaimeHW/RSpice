#![allow(dead_code, unused_assignments, unused_imports, unused_parens, unused_variables)]

use super::{AdValue, GeneratedEvalContext, GeneratedReactiveStamper, GeneratedStamper, Scratch, THERMAL_VOLTAGE_PER_K};
use super::super::state::Instance;

impl Instance {

    pub(super) fn stamp_transient_block_20(
        &mut self,
        ctx: &GeneratedEvalContext<'_>,
        stamper: &mut GeneratedStamper<'_>,
        scratch: &mut Scratch,
    ) {
        let _ = stamper;
        scratch.values[1675] = if (((-scratch.values[377]) * scratch.values[462]) == (-1.0)) { 1.0 } else { 0.0 };

        if (((((scratch.values[1323] != 0.0) && (scratch.values[1340] != 0.0)) && (!(scratch.values[1669] != 0.0))) && (!(scratch.values[1674] != 0.0))) && (scratch.values[1675] != 0.0)) {
            scratch.store_ad(1375, &AdValue::div_from_scalar(1.0, AdValue::offset(AdValue::mul(scratch.ad_value(1369), scratch.ad_value(1374)), 1.0)));
        }

        if (((((scratch.values[1323] != 0.0) && (scratch.values[1340] != 0.0)) && (!(scratch.values[1669] != 0.0))) && (!(scratch.values[1674] != 0.0))) && (!(scratch.values[1675] != 0.0))) {
            scratch.store_ad(1375, &AdValue::powf(AdValue::offset(AdValue::mul(scratch.ad_value(1369), scratch.ad_value(1374)), 1.0), ((-scratch.values[377]) * scratch.values[462])));
        }

        if ((((scratch.values[1323] != 0.0) && (scratch.values[1340] != 0.0)) && (!(scratch.values[1669] != 0.0))) && (!(scratch.values[1674] != 0.0))) {
            scratch.store_ad(1376, &AdValue::div(AdValue::mul(scratch.ad_value(1365), scratch.ad_value(1375)), AdValue::add(scratch.ad_value(1365), scratch.ad_value(1375))));
        }

        if ((((scratch.values[1323] != 0.0) && (scratch.values[1340] != 0.0)) && (!(scratch.values[1669] != 0.0))) && (!(scratch.values[1674] != 0.0))) {
            scratch.store_ad(1377, &AdValue::sqrt(AdValue::scale(AdValue::div(scratch.ad_value(1369), scratch.ad_value(1373)), 0.375)));
        }

        if ((((scratch.values[1323] != 0.0) && (scratch.values[1340] != 0.0)) && (!(scratch.values[1669] != 0.0))) && (!(scratch.values[1674] != 0.0))) {
            scratch.store_ad(1378, &AdValue::sub(AdValue::scale(AdValue::mul(scratch.ad_value(1370), scratch.ad_value(1373)), 2.0), scratch.ad_value(1372)));
        }

        if ((((scratch.values[1323] != 0.0) && (scratch.values[1340] != 0.0)) && (!(scratch.values[1669] != 0.0))) && (!(scratch.values[1674] != 0.0))) {
            scratch.store_ad(1379, &AdValue::add(AdValue::sub(AdValue::mul(AdValue::scale(scratch.ad_value(1370), scratch.values[486]), scratch.ad_value(1373)), AdValue::scale(scratch.ad_value(1372), scratch.values[486])), AdValue::scale(AdValue::mul(scratch.ad_value(1369), scratch.ad_value(1374)), 0.5)));
        }

        if ((((scratch.values[1323] != 0.0) && (scratch.values[1340] != 0.0)) && (!(scratch.values[1669] != 0.0))) && (!(scratch.values[1674] != 0.0))) {
            scratch.store_ad(1380, &AdValue::mul(AdValue::offset(scratch.ad_value(1378), (-1.0)), scratch.ad_value(1377)));
        }

        if ((((scratch.values[1323] != 0.0) && (scratch.values[1340] != 0.0)) && (!(scratch.values[1669] != 0.0))) && (!(scratch.values[1674] != 0.0))) {
            scratch.store_ad(1341, &AdValue::square(scratch.ad_value(1380)));
        }

        scratch.values[1676] = if (scratch.values[1380] > 0.0) { 1.0 } else { 0.0 };

        if (((((scratch.values[1323] != 0.0) && (scratch.values[1340] != 0.0)) && (!(scratch.values[1669] != 0.0))) && (!(scratch.values[1674] != 0.0))) && (scratch.values[1676] != 0.0)) {
            scratch.store_ad(1342, &AdValue::div_from_scalar(1.0, AdValue::offset(AdValue::scale(scratch.ad_value(1380), scratch.values[421]), 1.0)));
        }

        if (((((scratch.values[1323] != 0.0) && (scratch.values[1340] != 0.0)) && (!(scratch.values[1669] != 0.0))) && (!(scratch.values[1674] != 0.0))) && (!(scratch.values[1676] != 0.0))) {
            scratch.store_ad(1342, &AdValue::div_from_scalar(1.0, AdValue::sub_from_scalar(1.0, AdValue::scale(scratch.ad_value(1380), scratch.values[421]))));
        }

        scratch.values[1677] = if (((-scratch.values[1341]) + scratch.values[1379]) > (-230.25850929940458)) { 1.0 } else { 0.0 };

        if (((((scratch.values[1323] != 0.0) && (scratch.values[1340] != 0.0)) && (!(scratch.values[1669] != 0.0))) && (!(scratch.values[1674] != 0.0))) && (scratch.values[1677] != 0.0)) {
            scratch.store_ad(1359, &AdValue::exp(AdValue::sub(scratch.ad_value(1379), scratch.ad_value(1341))));
        }

        if (((((scratch.values[1323] != 0.0) && (scratch.values[1340] != 0.0)) && (!(scratch.values[1669] != 0.0))) && (!(scratch.values[1674] != 0.0))) && (!(scratch.values[1677] != 0.0))) {
            let assign26180_ad_e30277: AdValue = AdValue::div_from_scalar(1e-100, AdValue::offset(AdValue::mul(AdValue::sub_from_scalar((-230.25850929940458), AdValue::sub(scratch.ad_value(1379), scratch.ad_value(1341))), AdValue::offset(AdValue::scale(AdValue::mul(AdValue::sub_from_scalar((-230.25850929940458), AdValue::sub(scratch.ad_value(1379), scratch.ad_value(1341))), AdValue::offset(AdValue::scale(AdValue::sub_from_scalar((-230.25850929940458), AdValue::sub(scratch.ad_value(1379), scratch.ad_value(1341))), 0.3333333333333333), 1.0)), 0.5), 1.0)), 1.0));
            scratch.store_ad(1359, &assign26180_ad_e30277);
        }

        if ((((scratch.values[1323] != 0.0) && (scratch.values[1340] != 0.0)) && (!(scratch.values[1669] != 0.0))) && (!(scratch.values[1674] != 0.0))) {
            scratch.store_ad(1343, &AdValue::mul(AdValue::add(AdValue::add(AdValue::scale(scratch.ad_value(1342), 0.29214664), AdValue::scale(AdValue::square(scratch.ad_value(1342)), scratch.values[422])), AdValue::scale(AdValue::mul(AdValue::square(scratch.ad_value(1342)), scratch.ad_value(1342)), scratch.values[423])), scratch.ad_value(1359)));
        }

        scratch.values[1678] = if (scratch.values[1380] > 0.0) { 1.0 } else { 0.0 };

        if (((((scratch.values[1323] != 0.0) && (scratch.values[1340] != 0.0)) && (!(scratch.values[1669] != 0.0))) && (!(scratch.values[1674] != 0.0))) && (scratch.values[1678] != 0.0)) {
            scratch.values[1381] = scratch.values[1343];
            scratch.node_derivatives[1381] = scratch.node_derivatives[1343];
            scratch.branch_derivatives[1381] = scratch.branch_derivatives[1343];
        }

        scratch.values[1679] = if (scratch.values[1379] > (-230.25850929940458)) { 1.0 } else { 0.0 };

        if ((((((scratch.values[1323] != 0.0) && (scratch.values[1340] != 0.0)) && (!(scratch.values[1669] != 0.0))) && (!(scratch.values[1674] != 0.0))) && (!(scratch.values[1678] != 0.0))) && (scratch.values[1679] != 0.0)) {
            scratch.store_ad(1359, &AdValue::exp(scratch.ad_value(1379)));
        }

        if ((((((scratch.values[1323] != 0.0) && (scratch.values[1340] != 0.0)) && (!(scratch.values[1669] != 0.0))) && (!(scratch.values[1674] != 0.0))) && (!(scratch.values[1678] != 0.0))) && (!(scratch.values[1679] != 0.0))) {
            scratch.store_ad(1359, &AdValue::div_from_scalar(1e-100, AdValue::offset(AdValue::mul(AdValue::sub_from_scalar((-230.25850929940458), scratch.ad_value(1379)), AdValue::offset(AdValue::scale(AdValue::mul(AdValue::sub_from_scalar((-230.25850929940458), scratch.ad_value(1379)), AdValue::offset(AdValue::scale(AdValue::sub_from_scalar((-230.25850929940458), scratch.ad_value(1379)), 0.3333333333333333), 1.0)), 0.5), 1.0)), 1.0)));
        }

        if (((((scratch.values[1323] != 0.0) && (scratch.values[1340] != 0.0)) && (!(scratch.values[1669] != 0.0))) && (!(scratch.values[1674] != 0.0))) && (!(scratch.values[1678] != 0.0))) {
            scratch.store_ad(1381, &AdValue::sub(AdValue::scale(scratch.ad_value(1359), 2.0), scratch.ad_value(1343)));
        }

        if ((((scratch.values[1323] != 0.0) && (scratch.values[1340] != 0.0)) && (!(scratch.values[1669] != 0.0))) && (!(scratch.values[1674] != 0.0))) {
            scratch.store_ad(1382, &AdValue::scale(AdValue::div(AdValue::scale(scratch.ad_value(1381), scratch.values[486]), scratch.ad_value(1377)), (1.772453850905516 * 0.5)));
        }

        if ((((scratch.values[1323] != 0.0) && (scratch.values[1340] != 0.0)) && (!(scratch.values[1669] != 0.0))) && (!(scratch.values[1674] != 0.0))) {
            scratch.store_ad(1368, &AdValue::scale(AdValue::mul(AdValue::mul(scratch.ad_value(1367), scratch.ad_value(1382)), scratch.ad_value(1376)), scratch.values[391]));
        }

        scratch.values[1680] = if (scratch.values[397] == 0.0) { 1.0 } else { 0.0 };

        if ((((scratch.values[1323] != 0.0) && (scratch.values[1340] != 0.0)) && (!(scratch.values[1669] != 0.0))) && (scratch.values[1680] != 0.0)) {
            scratch.values[1383] = 0.0;
            scratch.node_derivatives[1383] = [0.0; Instance::NODE_COUNT];
            scratch.branch_derivatives[1383] = [0.0; Instance::BRANCH_COUNT];
        }

        scratch.values[1681] = if (scratch.values[377] == 0.5) { 1.0 } else { 0.0 };

        if (((((scratch.values[1323] != 0.0) && (scratch.values[1340] != 0.0)) && (!(scratch.values[1669] != 0.0))) && (!(scratch.values[1680] != 0.0))) && (scratch.values[1681] != 0.0)) {
            scratch.store_ad(1359, &AdValue::sqrt(AdValue::scale(AdValue::sub_from_scalar(scratch.values[374], scratch.ad_value(1357)), scratch.values[480])));
        }

        if (((((scratch.values[1323] != 0.0) && (scratch.values[1340] != 0.0)) && (!(scratch.values[1669] != 0.0))) && (!(scratch.values[1680] != 0.0))) && (!(scratch.values[1681] != 0.0))) {
            scratch.store_ad(1359, &AdValue::powf(AdValue::scale(AdValue::sub_from_scalar(scratch.values[374], scratch.ad_value(1357)), scratch.values[480]), scratch.values[377]));
        }

        if ((((scratch.values[1323] != 0.0) && (scratch.values[1340] != 0.0)) && (!(scratch.values[1669] != 0.0))) && (!(scratch.values[1680] != 0.0))) {
            scratch.store_ad(1384, &AdValue::scale(AdValue::div(AdValue::scale(AdValue::sub_from_scalar(scratch.values[374], scratch.ad_value(1357)), scratch.values[477]), scratch.ad_value(1359)), scratch.values[462]));
        }

        scratch.values[1682] = if (((((-scratch.values[492]) / scratch.values[1384])) as f64).abs() < 230.25850929940458) { 1.0 } else { 0.0 };

        if (((((scratch.values[1323] != 0.0) && (scratch.values[1340] != 0.0)) && (!(scratch.values[1669] != 0.0))) && (!(scratch.values[1680] != 0.0))) && (scratch.values[1682] != 0.0)) {
            scratch.store_ad(1359, &AdValue::exp(AdValue::div(AdValue::neg(scratch.ad_value(492)), scratch.ad_value(1384))));
        }

        scratch.values[1683] = if (((-scratch.values[492]) / scratch.values[1384]) < 0.0) { 1.0 } else { 0.0 };

        if ((((((scratch.values[1323] != 0.0) && (scratch.values[1340] != 0.0)) && (!(scratch.values[1669] != 0.0))) && (!(scratch.values[1680] != 0.0))) && (!(scratch.values[1682] != 0.0))) && (scratch.values[1683] != 0.0)) {
            let assign26370_ad_e30604: AdValue = AdValue::offset(AdValue::mul(AdValue::sub_from_scalar((-230.25850929940458), AdValue::div(AdValue::neg(scratch.ad_value(492)), scratch.ad_value(1384))), AdValue::offset(AdValue::scale(AdValue::mul(AdValue::sub_from_scalar((-230.25850929940458), AdValue::div(AdValue::neg(scratch.ad_value(492)), scratch.ad_value(1384))), AdValue::offset(AdValue::scale(AdValue::sub_from_scalar((-230.25850929940458), AdValue::div(AdValue::neg(scratch.ad_value(492)), scratch.ad_value(1384))), 0.3333333333333333), 1.0)), 0.5), 1.0)), 1.0);
            scratch.store_ad(1359, &AdValue::div_from_scalar(1e-100, assign26370_ad_e30604));
        }

        if ((((((scratch.values[1323] != 0.0) && (scratch.values[1340] != 0.0)) && (!(scratch.values[1669] != 0.0))) && (!(scratch.values[1680] != 0.0))) && (!(scratch.values[1682] != 0.0))) && (!(scratch.values[1683] != 0.0))) {
            let assign26380_ad_e30654: AdValue = AdValue::scale(AdValue::offset(AdValue::mul(AdValue::offset(AdValue::div(AdValue::neg(scratch.ad_value(492)), scratch.ad_value(1384)), (-230.25850929940458)), AdValue::offset(AdValue::scale(AdValue::mul(AdValue::offset(AdValue::div(AdValue::neg(scratch.ad_value(492)), scratch.ad_value(1384)), (-230.25850929940458)), AdValue::offset(AdValue::scale(AdValue::offset(AdValue::div(AdValue::neg(scratch.ad_value(492)), scratch.ad_value(1384)), (-230.25850929940458)), 0.3333333333333333), 1.0)), 0.5), 1.0)), 1.0), 1e100);
            scratch.store_ad(1359, &assign26380_ad_e30654);
        }

        if ((((scratch.values[1323] != 0.0) && (scratch.values[1340] != 0.0)) && (!(scratch.values[1669] != 0.0))) && (!(scratch.values[1680] != 0.0))) {
            scratch.store_ad(1383, &AdValue::scale(AdValue::mul(AdValue::mul(AdValue::mul(scratch.ad_value(520), scratch.ad_value(1384)), scratch.ad_value(1384)), scratch.ad_value(1359)), scratch.values[397]));
        }

        scratch.values[1684] = if (scratch.values[406] > 1000.0) { 1.0 } else { 0.0 };

        if ((((scratch.values[1323] != 0.0) && (scratch.values[1340] != 0.0)) && (!(scratch.values[1669] != 0.0))) && (scratch.values[1684] != 0.0)) {
            scratch.values[1385] = 1.0;
            scratch.node_derivatives[1385] = [0.0; Instance::NODE_COUNT];
            scratch.branch_derivatives[1385] = [0.0; Instance::BRANCH_COUNT];
        }

        scratch.values[1685] = if (scratch.values[1358] > ((-scratch.values[493]) * scratch.values[406])) { 1.0 } else { 0.0 };

        scratch.values[1686] = if (scratch.values[409] == 4.0) { 1.0 } else { 0.0 };

        if ((((((scratch.values[1323] != 0.0) && (scratch.values[1340] != 0.0)) && (!(scratch.values[1669] != 0.0))) && (!(scratch.values[1684] != 0.0))) && (scratch.values[1685] != 0.0)) && (scratch.values[1686] != 0.0)) {
            scratch.store_ad(1359, &AdValue::mul(AdValue::mul(AdValue::mul(AdValue::scale(scratch.ad_value(1358), scratch.values[499]), AdValue::scale(scratch.ad_value(1358), scratch.values[499])), AdValue::scale(scratch.ad_value(1358), scratch.values[499])), AdValue::scale(scratch.ad_value(1358), scratch.values[499])));
        }

        if ((((((scratch.values[1323] != 0.0) && (scratch.values[1340] != 0.0)) && (!(scratch.values[1669] != 0.0))) && (!(scratch.values[1684] != 0.0))) && (scratch.values[1685] != 0.0)) && (!(scratch.values[1686] != 0.0))) {
            scratch.store_ad(1359, &AdValue::powf(AdValue::abs(AdValue::scale(scratch.ad_value(1358), scratch.values[499])), scratch.values[409]));
        }

        if (((((scratch.values[1323] != 0.0) && (scratch.values[1340] != 0.0)) && (!(scratch.values[1669] != 0.0))) && (!(scratch.values[1684] != 0.0))) && (scratch.values[1685] != 0.0)) {
            scratch.store_ad(1385, &AdValue::div_from_scalar(1.0, AdValue::sub_from_scalar(1.0, scratch.ad_value(1359))));
        }

        if (((((scratch.values[1323] != 0.0) && (scratch.values[1340] != 0.0)) && (!(scratch.values[1669] != 0.0))) && (!(scratch.values[1684] != 0.0))) && (!(scratch.values[1685] != 0.0))) {
            scratch.store_ad(1385, &AdValue::offset(AdValue::scale(AdValue::offset(scratch.ad_value(1358), (scratch.values[493] * scratch.values[406])), scratch.values[502]), scratch.values[496]));
        }

        if (((scratch.values[1323] != 0.0) && (scratch.values[1340] != 0.0)) && (!(scratch.values[1669] != 0.0))) {
            scratch.store_ad(1388, &AdValue::mul(AdValue::add(AdValue::add(AdValue::add(scratch.ad_value(1360), scratch.ad_value(1361)), scratch.ad_value(1368)), scratch.ad_value(1383)), scratch.ad_value(1385)));
        }

        if ((scratch.values[1323] != 0.0) && (scratch.values[1340] != 0.0)) {
            scratch.store_ad(510, &AdValue::add(AdValue::add(AdValue::mul(scratch.ad_value(690), scratch.ad_value(1386)), AdValue::mul(scratch.ad_value(691), scratch.ad_value(1387))), AdValue::mul(scratch.ad_value(692), scratch.ad_value(1388))));
        }

        if ((scratch.values[1323] != 0.0) && (scratch.values[1340] != 0.0)) {
            scratch.store_ad(711, &AdValue::add(AdValue::add(AdValue::scale(scratch.ad_value(690), scratch.values[436]), AdValue::scale(scratch.ad_value(691), scratch.values[437])), AdValue::scale(scratch.ad_value(692), scratch.values[438])));
        }

        if ((scratch.values[1323] != 0.0) && (scratch.values[1340] != 0.0)) {
            scratch.store_ad(514, &AdValue::sub(scratch.ad_value(509), AdValue::mul(scratch.ad_value(711), AdValue::offset(AdValue::exp(AdValue::scale(scratch.ad_value(519), (scratch.values[420] * scratch.values[712]))), (-1.0)))));
        }

        if ((scratch.values[1323] != 0.0) && (scratch.values[1340] != 0.0)) {
            scratch.store_ad(515, &AdValue::sub(scratch.ad_value(510), AdValue::mul(scratch.ad_value(711), AdValue::offset(AdValue::exp(AdValue::scale(scratch.ad_value(520), (scratch.values[420] * scratch.values[712]))), (-1.0)))));
        }

        scratch.values[1687] = if !(((scratch.values[690] == 0.0) && (scratch.values[691] == 0.0)) && (scratch.values[692] == 0.0)) { 1.0 } else { 0.0 };

        scratch.values[1688] = if ((scratch.values[509] > 0.0) && (scratch.values[510] > 0.0)) { 1.0 } else { 0.0 };

        scratch.values[1689] = if (((((scratch.values[514] / scratch.values[509]) > 0.001) || ((scratch.values[515] / scratch.values[510]) > 0.001)) && (scratch.values[514] > 0.0)) && (scratch.values[515] > 0.0)) { 1.0 } else { 0.0 };

        if (((((scratch.values[1323] != 0.0) && (scratch.values[1340] != 0.0)) && (scratch.values[1687] != 0.0)) && (scratch.values[1688] != 0.0)) && (scratch.values[1689] != 0.0)) {
            scratch.store_ad(521, &AdValue::div(scratch.ad_value(514), scratch.ad_value(515)));
        }

        if (((((scratch.values[1323] != 0.0) && (scratch.values[1340] != 0.0)) && (scratch.values[1687] != 0.0)) && (scratch.values[1688] != 0.0)) && (scratch.values[1689] != 0.0)) {
            scratch.store_ad(714, &AdValue::div(AdValue::scale(AdValue::ln(scratch.ad_value(521)), scratch.values[419]), AdValue::sub(scratch.ad_value(519), scratch.ad_value(520))));
        }

        if (((((scratch.values[1323] != 0.0) && (scratch.values[1340] != 0.0)) && (scratch.values[1687] != 0.0)) && (scratch.values[1688] != 0.0)) && (scratch.values[1689] != 0.0)) {
            scratch.store_ad(713, &AdValue::div(scratch.ad_value(514), AdValue::offset(AdValue::exp(AdValue::mul(AdValue::scale(scratch.ad_value(519), scratch.values[420]), scratch.ad_value(714))), (-1.0))));
        }

        if (((scratch.values[1323] != 0.0) && (scratch.values[1340] != 0.0)) && (scratch.values[1687] != 0.0)) {
            scratch.store_ad(511, &AdValue::sub(AdValue::sub(scratch.ad_value(506), AdValue::mul(scratch.ad_value(711), AdValue::offset(AdValue::exp(AdValue::scale(scratch.ad_value(516), (scratch.values[420] * scratch.values[712]))), (-1.0)))), AdValue::mul(scratch.ad_value(713), AdValue::offset(AdValue::exp(AdValue::mul(AdValue::scale(scratch.ad_value(516), scratch.values[420]), scratch.ad_value(714))), (-1.0)))));
        }

        if (((scratch.values[1323] != 0.0) && (scratch.values[1340] != 0.0)) && (scratch.values[1687] != 0.0)) {
            scratch.store_ad(512, &AdValue::sub(AdValue::sub(scratch.ad_value(507), AdValue::mul(scratch.ad_value(711), AdValue::offset(AdValue::exp(AdValue::scale(scratch.ad_value(517), (scratch.values[420] * scratch.values[712]))), (-1.0)))), AdValue::mul(scratch.ad_value(713), AdValue::offset(AdValue::exp(AdValue::mul(AdValue::scale(scratch.ad_value(517), scratch.values[420]), scratch.ad_value(714))), (-1.0)))));
        }

        if (((scratch.values[1323] != 0.0) && (scratch.values[1340] != 0.0)) && (scratch.values[1687] != 0.0)) {
            scratch.store_ad(513, &AdValue::sub(AdValue::sub(scratch.ad_value(508), AdValue::mul(scratch.ad_value(711), AdValue::offset(AdValue::exp(AdValue::scale(scratch.ad_value(518), (scratch.values[420] * scratch.values[712]))), (-1.0)))), AdValue::mul(scratch.ad_value(713), AdValue::offset(AdValue::exp(AdValue::mul(AdValue::scale(scratch.ad_value(518), scratch.values[420]), scratch.ad_value(714))), (-1.0)))));
        }

        scratch.values[1690] = if (((scratch.values[506] < 0.0) && (scratch.values[507] < 0.0)) && (scratch.values[508] < 0.0)) { 1.0 } else { 0.0 };

        scratch.values[1691] = if (((((((scratch.values[511] / scratch.values[506]) > 0.001) || ((scratch.values[512] / scratch.values[507]) > 0.001)) || ((scratch.values[513] / scratch.values[508]) > 0.001)) && (scratch.values[511] < 0.0)) && (scratch.values[512] < 0.0)) && (scratch.values[513] < 0.0)) { 1.0 } else { 0.0 };

        if (((((scratch.values[1323] != 0.0) && (scratch.values[1340] != 0.0)) && (scratch.values[1687] != 0.0)) && (scratch.values[1690] != 0.0)) && (scratch.values[1691] != 0.0)) {
            scratch.store_ad(521, &AdValue::div(scratch.ad_value(511), scratch.ad_value(512)));
        }

        if (((((scratch.values[1323] != 0.0) && (scratch.values[1340] != 0.0)) && (scratch.values[1687] != 0.0)) && (scratch.values[1690] != 0.0)) && (scratch.values[1691] != 0.0)) {
            scratch.store_ad(522, &AdValue::div(AdValue::scale(AdValue::ln(scratch.ad_value(521)), (-scratch.values[419])), AdValue::sub(scratch.ad_value(516), scratch.ad_value(517))));
        }

        if (((((scratch.values[1323] != 0.0) && (scratch.values[1340] != 0.0)) && (scratch.values[1687] != 0.0)) && (scratch.values[1690] != 0.0)) && (scratch.values[1691] != 0.0)) {
            scratch.store_ad(524, &AdValue::div(scratch.ad_value(517), AdValue::sub(scratch.ad_value(517), scratch.ad_value(516))));
        }

        if (((((scratch.values[1323] != 0.0) && (scratch.values[1340] != 0.0)) && (scratch.values[1687] != 0.0)) && (scratch.values[1690] != 0.0)) && (scratch.values[1691] != 0.0)) {
            scratch.store_ad(525, &AdValue::mul(AdValue::scale(AdValue::offset(scratch.ad_value(521), (-1.0)), scratch.values[419]), AdValue::offset(AdValue::pow(scratch.ad_value(521), scratch.ad_value(524)), (-1.0))));
        }

        if (((((scratch.values[1323] != 0.0) && (scratch.values[1340] != 0.0)) && (scratch.values[1687] != 0.0)) && (scratch.values[1690] != 0.0)) && (scratch.values[1691] != 0.0)) {
            scratch.store_ad(524, &AdValue::div(scratch.ad_value(516), AdValue::sub(scratch.ad_value(516), scratch.ad_value(517))));
        }

        if (((((scratch.values[1323] != 0.0) && (scratch.values[1340] != 0.0)) && (scratch.values[1687] != 0.0)) && (scratch.values[1690] != 0.0)) && (scratch.values[1691] != 0.0)) {
            scratch.store_ad(526, &AdValue::sub(AdValue::add(AdValue::mul(AdValue::pow(scratch.ad_value(521), scratch.ad_value(524)), AdValue::sub(scratch.ad_value(517), scratch.ad_value(516))), AdValue::mul(scratch.ad_value(521), scratch.ad_value(516))), scratch.ad_value(517)));
        }

        if (((((scratch.values[1323] != 0.0) && (scratch.values[1340] != 0.0)) && (scratch.values[1687] != 0.0)) && (scratch.values[1690] != 0.0)) && (scratch.values[1691] != 0.0)) {
            scratch.store_ad(523, &AdValue::div(scratch.ad_value(525), scratch.ad_value(526)));
        }

        if (((((scratch.values[1323] != 0.0) && (scratch.values[1340] != 0.0)) && (scratch.values[1687] != 0.0)) && (scratch.values[1690] != 0.0)) && (scratch.values[1691] != 0.0)) {
            scratch.store_ad(716, &AdValue::add(scratch.ad_value(522), scratch.ad_value(523)));
        }

        scratch.values[1692] = if (((((scratch.values[518] * scratch.values[420]) * scratch.values[716])) as f64).abs() < 1e-6) { 1.0 } else { 0.0 };

        if ((((((scratch.values[1323] != 0.0) && (scratch.values[1340] != 0.0)) && (scratch.values[1687] != 0.0)) && (scratch.values[1690] != 0.0)) && (scratch.values[1691] != 0.0)) && (scratch.values[1692] != 0.0)) {
            scratch.values[710] = 1.0;
            scratch.node_derivatives[710] = [0.0; Instance::NODE_COUNT];
            scratch.branch_derivatives[710] = [0.0; Instance::BRANCH_COUNT];
        }

        if ((((((scratch.values[1323] != 0.0) && (scratch.values[1340] != 0.0)) && (scratch.values[1687] != 0.0)) && (scratch.values[1690] != 0.0)) && (scratch.values[1691] != 0.0)) && (scratch.values[1692] != 0.0)) {
            scratch.store_ad(715, &AdValue::mul(scratch.ad_value(513), AdValue::add(AdValue::div_from_scalar(1.0, scratch.ad_value(518)), AdValue::scale(scratch.ad_value(716), (0.5 * scratch.values[420])))));
        }

        if ((((((scratch.values[1323] != 0.0) && (scratch.values[1340] != 0.0)) && (scratch.values[1687] != 0.0)) && (scratch.values[1690] != 0.0)) && (scratch.values[1691] != 0.0)) && (scratch.values[1692] != 0.0)) {
            scratch.store_ad(716, &AdValue::div(AdValue::scale(AdValue::mul(AdValue::scale(scratch.ad_value(513), (-0.5)), scratch.ad_value(716)), scratch.values[420]), scratch.ad_value(518)));
        }

        if ((((((scratch.values[1323] != 0.0) && (scratch.values[1340] != 0.0)) && (scratch.values[1687] != 0.0)) && (scratch.values[1690] != 0.0)) && (scratch.values[1691] != 0.0)) && (!(scratch.values[1692] != 0.0))) {
            scratch.values[710] = 0.0;
            scratch.node_derivatives[710] = [0.0; Instance::NODE_COUNT];
            scratch.branch_derivatives[710] = [0.0; Instance::BRANCH_COUNT];
        }

        if ((((((scratch.values[1323] != 0.0) && (scratch.values[1340] != 0.0)) && (scratch.values[1687] != 0.0)) && (scratch.values[1690] != 0.0)) && (scratch.values[1691] != 0.0)) && (!(scratch.values[1692] != 0.0))) {
            scratch.store_ad(715, &AdValue::div(AdValue::neg(scratch.ad_value(513)), AdValue::offset(AdValue::exp(AdValue::mul(AdValue::scale(AdValue::neg(scratch.ad_value(518)), scratch.values[420]), scratch.ad_value(716))), (-1.0))));
        }

        if ((scratch.values[1323] != 0.0) && (scratch.values[1340] != 0.0)) {
            scratch.store_ad(532, &AdValue::scale(AdValue::add(AdValue::add(AdValue::scale(scratch.ad_value(690), scratch.values[463]), AdValue::scale(scratch.ad_value(691), scratch.values[464])), AdValue::scale(scratch.ad_value(692), scratch.values[465])), scratch.values[412]));
        }

        scratch.values[1693] = if ((scratch.values[690] * scratch.values[463]) <= scratch.values[532]) { 1.0 } else { 0.0 };

        if (((scratch.values[1323] != 0.0) && (scratch.values[1340] != 0.0)) && (scratch.values[1693] != 0.0)) {
            scratch.values[695] = 0.0;
            scratch.node_derivatives[695] = [0.0; Instance::NODE_COUNT];
            scratch.branch_derivatives[695] = [0.0; Instance::BRANCH_COUNT];
        }

        scratch.values[1694] = if ((scratch.values[691] * scratch.values[464]) <= scratch.values[532]) { 1.0 } else { 0.0 };

        if (((scratch.values[1323] != 0.0) && (scratch.values[1340] != 0.0)) && (scratch.values[1694] != 0.0)) {
            scratch.values[696] = 0.0;
            scratch.node_derivatives[696] = [0.0; Instance::NODE_COUNT];
            scratch.branch_derivatives[696] = [0.0; Instance::BRANCH_COUNT];
        }

        scratch.values[1695] = if ((scratch.values[692] * scratch.values[465]) <= scratch.values[532]) { 1.0 } else { 0.0 };

        if (((scratch.values[1323] != 0.0) && (scratch.values[1340] != 0.0)) && (scratch.values[1695] != 0.0)) {
            scratch.values[697] = 0.0;
            scratch.node_derivatives[697] = [0.0; Instance::NODE_COUNT];
            scratch.branch_derivatives[697] = [0.0; Instance::BRANCH_COUNT];
        }

        scratch.values[1696] = if !(((scratch.values[690] == 0.0) && (scratch.values[691] == 0.0)) && (scratch.values[692] == 0.0)) { 1.0 } else { 0.0 };

        if (((scratch.values[1323] != 0.0) && (scratch.values[1340] != 0.0)) && (scratch.values[1696] != 0.0)) {
            scratch.store_ad(704, &AdValue::ln(AdValue::div_from_scalar((0.5 * scratch.values[367]), AdValue::offset(scratch.ad_value(711), 1e-21))));
        }

        if (((scratch.values[1323] != 0.0) && (scratch.values[1340] != 0.0)) && (scratch.values[1696] != 0.0)) {
            scratch.store_ad(706, &AdValue::ln(AdValue::div_from_scalar((0.5 * scratch.values[367]), AdValue::offset(scratch.ad_value(713), 1e-21))));
        }

        if (((scratch.values[1323] != 0.0) && (scratch.values[1340] != 0.0)) && (scratch.values[1696] != 0.0)) {
            scratch.store_ad(708, &AdValue::ln(AdValue::div_from_scalar((0.5 * scratch.values[367]), AdValue::offset(AdValue::abs(scratch.ad_value(715)), 1e-21))));
        }

        if ((scratch.values[1323] != 0.0) && (scratch.values[1340] != 0.0)) {
            scratch.store_ad(704, &AdValue::min_with_scalar(scratch.ad_value(704), 230.25850929940458));
        }

        if ((scratch.values[1323] != 0.0) && (scratch.values[1340] != 0.0)) {
            scratch.store_ad(705, &AdValue::exp(scratch.ad_value(704)));
        }

        if ((scratch.values[1323] != 0.0) && (scratch.values[1340] != 0.0)) {
            scratch.store_ad(706, &AdValue::min_with_scalar(scratch.ad_value(706), 230.25850929940458));
        }

        if ((scratch.values[1323] != 0.0) && (scratch.values[1340] != 0.0)) {
            scratch.store_ad(707, &AdValue::exp(scratch.ad_value(706)));
        }

        if ((scratch.values[1323] != 0.0) && (scratch.values[1340] != 0.0)) {
            scratch.store_ad(708, &AdValue::min_with_scalar(scratch.ad_value(708), 230.25850929940458));
        }

        if ((scratch.values[1323] != 0.0) && (scratch.values[1340] != 0.0)) {
            scratch.store_ad(709, &AdValue::exp(scratch.ad_value(708)));
        }

        if ((scratch.values[1323] != 0.0) && (scratch.values[1340] != 0.0)) {
            scratch.values[529] = 0.4;
            scratch.node_derivatives[529] = [0.0; Instance::NODE_COUNT];
            scratch.branch_derivatives[529] = [0.0; Instance::BRANCH_COUNT];
        }

        if ((scratch.values[1323] != 0.0) && (scratch.values[1340] != 0.0)) {
            scratch.values[530] = 0.65;
            scratch.node_derivatives[530] = [0.0; Instance::NODE_COUNT];
            scratch.branch_derivatives[530] = [0.0; Instance::BRANCH_COUNT];
        }

        if ((scratch.values[1323] != 0.0) && (scratch.values[1340] != 0.0)) {
            scratch.values[531] = 0.8;
            scratch.node_derivatives[531] = [0.0; Instance::NODE_COUNT];
            scratch.branch_derivatives[531] = [0.0; Instance::BRANCH_COUNT];
        }

        if ((scratch.values[1323] != 0.0) && (scratch.values[1340] != 0.0)) {
            scratch.store_ad(516, &AdValue::mul(AdValue::neg(scratch.ad_value(529)), scratch.ad_value(577)));
        }

        if ((scratch.values[1323] != 0.0) && (scratch.values[1340] != 0.0)) {
            scratch.store_ad(517, &AdValue::mul(AdValue::neg(scratch.ad_value(530)), scratch.ad_value(577)));
        }

        if ((scratch.values[1323] != 0.0) && (scratch.values[1340] != 0.0)) {
            scratch.store_ad(518, &AdValue::mul(AdValue::neg(scratch.ad_value(531)), scratch.ad_value(577)));
        }

        if ((scratch.values[1323] != 0.0) && (scratch.values[1340] != 0.0)) {
            scratch.values[519] = 0.1;
            scratch.node_derivatives[519] = [0.0; Instance::NODE_COUNT];
            scratch.branch_derivatives[519] = [0.0; Instance::BRANCH_COUNT];
        }

        if ((scratch.values[1323] != 0.0) && (scratch.values[1340] != 0.0)) {
            scratch.values[520] = 0.2;
            scratch.node_derivatives[520] = [0.0; Instance::NODE_COUNT];
            scratch.branch_derivatives[520] = [0.0; Instance::BRANCH_COUNT];
        }

        if ((scratch.values[1323] != 0.0) && (scratch.values[1340] != 0.0)) {
            scratch.values[1357] = 0.0;
            scratch.node_derivatives[1357] = [0.0; Instance::NODE_COUNT];
            scratch.branch_derivatives[1357] = [0.0; Instance::BRANCH_COUNT];
        }

        if ((scratch.values[1323] != 0.0) && (scratch.values[1340] != 0.0)) {
            scratch.values[1354] = 0.0;
            scratch.node_derivatives[1354] = [0.0; Instance::NODE_COUNT];
            scratch.branch_derivatives[1354] = [0.0; Instance::BRANCH_COUNT];
        }

        scratch.values[1697] = if !(((scratch.values[717] == 0.0) && (scratch.values[718] == 0.0)) && (scratch.values[719] == 0.0)) { 1.0 } else { 0.0 };

        if (((scratch.values[1323] != 0.0) && (scratch.values[1340] != 0.0)) && (scratch.values[1697] != 0.0)) {
            scratch.store_ad(1344, &AdValue::mul(AdValue::scale(scratch.ad_value(728), 4.0), scratch.ad_value(728)));
        }

        if (((scratch.values[1323] != 0.0) && (scratch.values[1340] != 0.0)) && (scratch.values[1697] != 0.0)) {
            scratch.store_ad(1345, &AdValue::div(scratch.ad_value(728), scratch.ad_value(729)));
        }

        if (((scratch.values[1323] != 0.0) && (scratch.values[1340] != 0.0)) && (scratch.values[1697] != 0.0)) {
            scratch.store_ad(1346, &AdValue::add(scratch.ad_value(516), AdValue::mul(scratch.ad_value(728), scratch.ad_value(1345))));
        }

        if (((scratch.values[1323] != 0.0) && (scratch.values[1340] != 0.0)) && (scratch.values[1697] != 0.0)) {
            scratch.store_ad(1347, &AdValue::add(scratch.ad_value(729), scratch.ad_value(1346)));
        }

        if (((scratch.values[1323] != 0.0) && (scratch.values[1340] != 0.0)) && (scratch.values[1697] != 0.0)) {
            scratch.store_ad(1348, &AdValue::sub(scratch.ad_value(729), scratch.ad_value(1346)));
        }

        if (((scratch.values[1323] != 0.0) && (scratch.values[1340] != 0.0)) && (scratch.values[1697] != 0.0)) {
            scratch.store_ad(1349, &AdValue::sqrt(AdValue::add(AdValue::square(scratch.ad_value(1348)), scratch.ad_value(1344))));
        }

        if (((scratch.values[1323] != 0.0) && (scratch.values[1340] != 0.0)) && (scratch.values[1697] != 0.0)) {
            scratch.store_ad(1351, &AdValue::scale(AdValue::div(AdValue::mul(scratch.ad_value(516), scratch.ad_value(729)), AdValue::add(scratch.ad_value(1347), scratch.ad_value(1349))), 2.0));
        }

        scratch.values[1698] = if (scratch.values[516] < scratch.values[725]) { 1.0 } else { 0.0 };

        scratch.values[1699] = if ((((0.5 * (scratch.values[516] * scratch.values[420]))) as f64).abs() < 230.25850929940458) { 1.0 } else { 0.0 };

        if (((((scratch.values[1323] != 0.0) && (scratch.values[1340] != 0.0)) && (scratch.values[1697] != 0.0)) && (scratch.values[1698] != 0.0)) && (scratch.values[1699] != 0.0)) {
            scratch.store_ad(1353, &AdValue::exp(AdValue::scale(scratch.ad_value(516), (scratch.values[420] * 0.5))));
        }

        scratch.values[1700] = if ((0.5 * (scratch.values[516] * scratch.values[420])) < 0.0) { 1.0 } else { 0.0 };

        if ((((((scratch.values[1323] != 0.0) && (scratch.values[1340] != 0.0)) && (scratch.values[1697] != 0.0)) && (scratch.values[1698] != 0.0)) && (!(scratch.values[1699] != 0.0))) && (scratch.values[1700] != 0.0)) {
            let assign27170_ad_e31756: AdValue = AdValue::div_from_scalar(1e-100, AdValue::offset(AdValue::mul(AdValue::sub_from_scalar((-230.25850929940458), AdValue::scale(scratch.ad_value(516), (scratch.values[420] * 0.5))), AdValue::offset(AdValue::scale(AdValue::mul(AdValue::sub_from_scalar((-230.25850929940458), AdValue::scale(scratch.ad_value(516), (scratch.values[420] * 0.5))), AdValue::offset(AdValue::scale(AdValue::sub_from_scalar((-230.25850929940458), AdValue::scale(scratch.ad_value(516), (scratch.values[420] * 0.5))), 0.3333333333333333), 1.0)), 0.5), 1.0)), 1.0));
            scratch.store_ad(1353, &assign27170_ad_e31756);
        }

        if ((((((scratch.values[1323] != 0.0) && (scratch.values[1340] != 0.0)) && (scratch.values[1697] != 0.0)) && (scratch.values[1698] != 0.0)) && (!(scratch.values[1699] != 0.0))) && (!(scratch.values[1700] != 0.0))) {
            scratch.store_ad(1353, &AdValue::scale(AdValue::offset(AdValue::mul(AdValue::offset(AdValue::scale(scratch.ad_value(516), (scratch.values[420] * 0.5)), (-230.25850929940458)), AdValue::offset(AdValue::scale(AdValue::mul(AdValue::offset(AdValue::scale(scratch.ad_value(516), (scratch.values[420] * 0.5)), (-230.25850929940458)), AdValue::offset(AdValue::scale(AdValue::offset(AdValue::scale(scratch.ad_value(516), (scratch.values[420] * 0.5)), (-230.25850929940458)), 0.3333333333333333), 1.0)), 0.5), 1.0)), 1.0), 1e100));
        }

        if ((((scratch.values[1323] != 0.0) && (scratch.values[1340] != 0.0)) && (scratch.values[1697] != 0.0)) && (scratch.values[1698] != 0.0)) {
            scratch.store_ad(1350, &AdValue::square(scratch.ad_value(1353)));
        }

        if ((((scratch.values[1323] != 0.0) && (scratch.values[1340] != 0.0)) && (scratch.values[1697] != 0.0)) && (!(scratch.values[1698] != 0.0))) {
            scratch.store_ad(1350, &AdValue::mul(AdValue::offset(AdValue::scale(AdValue::sub(scratch.ad_value(516), scratch.ad_value(725)), scratch.values[420]), 1.0), scratch.ad_value(726)));
        }

        if ((((scratch.values[1323] != 0.0) && (scratch.values[1340] != 0.0)) && (scratch.values[1697] != 0.0)) && (!(scratch.values[1698] != 0.0))) {
            scratch.store_ad(1353, &AdValue::sqrt(scratch.ad_value(1350)));
        }

        if (((scratch.values[1323] != 0.0) && (scratch.values[1340] != 0.0)) && (scratch.values[1697] != 0.0)) {
            scratch.store_ad(1350, &AdValue::offset(scratch.ad_value(1350), (-1.0)));
        }

        if (((scratch.values[1323] != 0.0) && (scratch.values[1340] != 0.0)) && (scratch.values[1697] != 0.0)) {
            scratch.store_ad(1352, &AdValue::div_from_scalar(1.0, scratch.ad_value(1353)));
        }

        scratch.values[1701] = if (scratch.values[516] > 0.0) { 1.0 } else { 0.0 };

        if ((((scratch.values[1323] != 0.0) && (scratch.values[1340] != 0.0)) && (scratch.values[1697] != 0.0)) && (scratch.values[1701] != 0.0)) {
            scratch.store_ad(1354, &AdValue::scale(AdValue::ln(AdValue::add(AdValue::offset(scratch.ad_value(1352), 2.0), AdValue::sqrt(AdValue::mul(AdValue::offset(scratch.ad_value(1352), 1.0), AdValue::offset(scratch.ad_value(1352), 3.0))))), (scratch.values[419] * 2.0)));
        }

        if ((((scratch.values[1323] != 0.0) && (scratch.values[1340] != 0.0)) && (scratch.values[1697] != 0.0)) && (!(scratch.values[1701] != 0.0))) {
            scratch.store_ad(1354, &AdValue::sub(AdValue::scale(AdValue::ln(AdValue::add(AdValue::offset(AdValue::scale(scratch.ad_value(1353), 2.0), 1.0), AdValue::sqrt(AdValue::mul(AdValue::offset(scratch.ad_value(1353), 1.0), AdValue::offset(AdValue::scale(scratch.ad_value(1353), 3.0), 1.0))))), (scratch.values[419] * 2.0)), scratch.ad_value(516)));
        }

        if (((scratch.values[1323] != 0.0) && (scratch.values[1340] != 0.0)) && (scratch.values[1697] != 0.0)) {
            scratch.store_ad(1355, &AdValue::sub(scratch.ad_value(727), scratch.ad_value(1354)));
        }

        if (((scratch.values[1323] != 0.0) && (scratch.values[1340] != 0.0)) && (scratch.values[1697] != 0.0)) {
            scratch.store_ad(1356, &AdValue::scale(AdValue::sub(AdValue::add(scratch.ad_value(516), scratch.ad_value(1355)), AdValue::sqrt(AdValue::offset(AdValue::mul(AdValue::sub(scratch.ad_value(516), scratch.ad_value(1355)), AdValue::sub(scratch.ad_value(516), scratch.ad_value(1355))), ((4.0 * scratch.values[419]) * scratch.values[419])))), 0.5));
        }

        if (((scratch.values[1323] != 0.0) && (scratch.values[1340] != 0.0)) && (scratch.values[1697] != 0.0)) {
            scratch.store_ad(1357, &AdValue::scale(AdValue::sub(AdValue::add(scratch.ad_value(516), scratch.ad_value(730)), AdValue::sqrt(AdValue::offset(AdValue::mul(AdValue::sub(scratch.ad_value(516), scratch.ad_value(730)), AdValue::sub(scratch.ad_value(516), scratch.ad_value(730))), ((4.0 * scratch.values[417]) * scratch.values[417])))), 0.5));
        }

        if (((scratch.values[1323] != 0.0) && (scratch.values[1340] != 0.0)) && (scratch.values[1697] != 0.0)) {
            scratch.store_ad(1358, &AdValue::scale(AdValue::sub(scratch.ad_value(516), AdValue::sqrt(AdValue::offset(AdValue::mul(scratch.ad_value(516), scratch.ad_value(516)), ((4.0 * 1e-6) * 1e-6)))), 0.5));
        }

        scratch.values[1702] = if (scratch.values[717] == 0.0) { 1.0 } else { 0.0 };

        if (((scratch.values[1323] != 0.0) && (scratch.values[1340] != 0.0)) && (scratch.values[1702] != 0.0)) {
            scratch.values[1386] = 0.0;
            scratch.node_derivatives[1386] = [0.0; Instance::NODE_COUNT];
            scratch.branch_derivatives[1386] = [0.0; Instance::BRANCH_COUNT];
        }

        scratch.values[1703] = if (scratch.values[600] == 0.5) { 1.0 } else { 0.0 };

        if ((((scratch.values[1323] != 0.0) && (scratch.values[1340] != 0.0)) && (!(scratch.values[1702] != 0.0))) && (scratch.values[1703] != 0.0)) {
            scratch.store_ad(1359, &AdValue::sqrt(AdValue::sub_from_scalar(1.0, AdValue::mul(scratch.ad_value(1351), scratch.ad_value(597)))));
        }

        if ((((scratch.values[1323] != 0.0) && (scratch.values[1340] != 0.0)) && (!(scratch.values[1702] != 0.0))) && (!(scratch.values[1703] != 0.0))) {
            scratch.store_ad(1359, &AdValue::pow(AdValue::sub_from_scalar(1.0, AdValue::mul(scratch.ad_value(1351), scratch.ad_value(597))), scratch.ad_value(600)));
        }

        if (((scratch.values[1323] != 0.0) && (scratch.values[1340] != 0.0)) && (!(scratch.values[1702] != 0.0))) {
            scratch.store_ad(1360, &AdValue::mul(scratch.ad_value(588), scratch.ad_value(1350)));
        }

        scratch.values[1704] = if ((scratch.values[553] == 0.0) && (scratch.values[556] == 0.0)) { 1.0 } else { 0.0 };

    }

    pub(super) fn stamp_transient_block_21(
        &mut self,
        ctx: &GeneratedEvalContext<'_>,
        stamper: &mut GeneratedStamper<'_>,
        scratch: &mut Scratch,
    ) {
        let _ = stamper;
        if ((((scratch.values[1323] != 0.0) && (scratch.values[1340] != 0.0)) && (!(scratch.values[1702] != 0.0))) && (scratch.values[1704] != 0.0)) {
            scratch.values[1361] = 0.0;
            scratch.node_derivatives[1361] = [0.0; Instance::NODE_COUNT];
            scratch.branch_derivatives[1361] = [0.0; Instance::BRANCH_COUNT];
        }

        if ((((scratch.values[1323] != 0.0) && (scratch.values[1340] != 0.0)) && (!(scratch.values[1702] != 0.0))) && (!(scratch.values[1704] != 0.0))) {
            scratch.store_ad(1362, &AdValue::sub(scratch.ad_value(594), scratch.ad_value(1356)));
        }

        if ((((scratch.values[1323] != 0.0) && (scratch.values[1340] != 0.0)) && (!(scratch.values[1702] != 0.0))) && (!(scratch.values[1704] != 0.0))) {
            scratch.store_ad(1363, &AdValue::sub_from_scalar(1.0, AdValue::sqrt(AdValue::sub_from_scalar(1.0, AdValue::div(scratch.ad_value(1354), scratch.ad_value(1362))))));
        }

        scratch.values[1705] = if (scratch.values[542] == 0.5) { 1.0 } else { 0.0 };

        if (((((scratch.values[1323] != 0.0) && (scratch.values[1340] != 0.0)) && (!(scratch.values[1702] != 0.0))) && (!(scratch.values[1704] != 0.0))) && (scratch.values[1705] != 0.0)) {
            scratch.values[1364] = 0.0;
            scratch.node_derivatives[1364] = [0.0; Instance::NODE_COUNT];
            scratch.branch_derivatives[1364] = [0.0; Instance::BRANCH_COUNT];
        }

        if (((((scratch.values[1323] != 0.0) && (scratch.values[1340] != 0.0)) && (!(scratch.values[1702] != 0.0))) && (!(scratch.values[1704] != 0.0))) && (!(scratch.values[1705] != 0.0))) {
            scratch.store_ad(1364, &AdValue::mul(AdValue::add(AdValue::div(AdValue::mul(AdValue::square(scratch.ad_value(1363)), AdValue::ln(scratch.ad_value(1363))), AdValue::sub_from_scalar(1.0, scratch.ad_value(1363))), scratch.ad_value(1363)), AdValue::sub_from_scalar(1.0, AdValue::scale(scratch.ad_value(542), 2.0))));
        }

        if ((((scratch.values[1323] != 0.0) && (scratch.values[1340] != 0.0)) && (!(scratch.values[1702] != 0.0))) && (!(scratch.values[1704] != 0.0))) {
            scratch.store_ad(1365, &AdValue::add(scratch.ad_value(1363), scratch.ad_value(1364)));
        }

        scratch.values[1706] = if (scratch.values[542] == 0.5) { 1.0 } else { 0.0 };

        if (((((scratch.values[1323] != 0.0) && (scratch.values[1340] != 0.0)) && (!(scratch.values[1702] != 0.0))) && (!(scratch.values[1704] != 0.0))) && (scratch.values[1706] != 0.0)) {
            scratch.store_ad(1359, &AdValue::sqrt(AdValue::mul(scratch.ad_value(1362), scratch.ad_value(621))));
        }

        if (((((scratch.values[1323] != 0.0) && (scratch.values[1340] != 0.0)) && (!(scratch.values[1702] != 0.0))) && (!(scratch.values[1704] != 0.0))) && (!(scratch.values[1706] != 0.0))) {
            scratch.store_ad(1359, &AdValue::pow(AdValue::mul(scratch.ad_value(1362), scratch.ad_value(621)), scratch.ad_value(542)));
        }

        if ((((scratch.values[1323] != 0.0) && (scratch.values[1340] != 0.0)) && (!(scratch.values[1702] != 0.0))) && (!(scratch.values[1704] != 0.0))) {
            scratch.store_ad(1366, &AdValue::mul(scratch.ad_value(615), scratch.ad_value(1359)));
        }

        if ((((scratch.values[1323] != 0.0) && (scratch.values[1340] != 0.0)) && (!(scratch.values[1702] != 0.0))) && (!(scratch.values[1704] != 0.0))) {
            scratch.store_ad(1367, &AdValue::mul(scratch.ad_value(585), AdValue::mul(AdValue::offset(scratch.ad_value(1353), (-1.0)), scratch.ad_value(1366))));
        }

        if ((((scratch.values[1323] != 0.0) && (scratch.values[1340] != 0.0)) && (!(scratch.values[1702] != 0.0))) && (!(scratch.values[1704] != 0.0))) {
            scratch.store_ad(1361, &AdValue::mul(scratch.ad_value(553), AdValue::mul(scratch.ad_value(1367), scratch.ad_value(1365))));
        }

        scratch.values[1707] = if (scratch.values[556] == 0.0) { 1.0 } else { 0.0 };

        if ((((scratch.values[1323] != 0.0) && (scratch.values[1340] != 0.0)) && (!(scratch.values[1702] != 0.0))) && (scratch.values[1707] != 0.0)) {
            scratch.values[1368] = 0.0;
            scratch.node_derivatives[1368] = [0.0; Instance::NODE_COUNT];
            scratch.branch_derivatives[1368] = [0.0; Instance::BRANCH_COUNT];
        }

        if ((((scratch.values[1323] != 0.0) && (scratch.values[1340] != 0.0)) && (!(scratch.values[1702] != 0.0))) && (!(scratch.values[1707] != 0.0))) {
            scratch.store_ad(1369, &AdValue::mul(scratch.ad_value(630), AdValue::div(AdValue::mul(scratch.ad_value(1366), scratch.ad_value(600)), scratch.ad_value(1362))));
        }

        if ((((scratch.values[1323] != 0.0) && (scratch.values[1340] != 0.0)) && (!(scratch.values[1702] != 0.0))) && (!(scratch.values[1707] != 0.0))) {
            scratch.store_ad(1370, &AdValue::div(AdValue::scale(scratch.ad_value(627), 0.666666666666667), scratch.ad_value(1369)));
        }

        if ((((scratch.values[1323] != 0.0) && (scratch.values[1340] != 0.0)) && (!(scratch.values[1702] != 0.0))) && (!(scratch.values[1707] != 0.0))) {
            scratch.store_ad(1371, &AdValue::square(scratch.ad_value(1370)));
        }

        if ((((scratch.values[1323] != 0.0) && (scratch.values[1340] != 0.0)) && (!(scratch.values[1702] != 0.0))) && (!(scratch.values[1707] != 0.0))) {
            scratch.store_ad(1372, &AdValue::sqrt(AdValue::div(AdValue::square(scratch.ad_value(1371)), AdValue::offset(AdValue::square(scratch.ad_value(1371)), 1.0))));
        }

        if ((((scratch.values[1323] != 0.0) && (scratch.values[1340] != 0.0)) && (!(scratch.values[1702] != 0.0))) && (!(scratch.values[1707] != 0.0))) {
            scratch.store_ad(1373, &AdValue::sqrt(AdValue::abs(scratch.ad_value(1372))));
        }

        if ((((scratch.values[1323] != 0.0) && (scratch.values[1340] != 0.0)) && (!(scratch.values[1702] != 0.0))) && (!(scratch.values[1707] != 0.0))) {
            scratch.store_ad(1374, &AdValue::mul(scratch.ad_value(1372), scratch.ad_value(1373)));
        }

        scratch.values[1708] = if (((-scratch.values[542]) * scratch.values[603]) == (-1.0)) { 1.0 } else { 0.0 };

        if (((((scratch.values[1323] != 0.0) && (scratch.values[1340] != 0.0)) && (!(scratch.values[1702] != 0.0))) && (!(scratch.values[1707] != 0.0))) && (scratch.values[1708] != 0.0)) {
            scratch.store_ad(1375, &AdValue::div_from_scalar(1.0, AdValue::offset(AdValue::mul(scratch.ad_value(1369), scratch.ad_value(1374)), 1.0)));
        }

        if (((((scratch.values[1323] != 0.0) && (scratch.values[1340] != 0.0)) && (!(scratch.values[1702] != 0.0))) && (!(scratch.values[1707] != 0.0))) && (!(scratch.values[1708] != 0.0))) {
            scratch.store_ad(1375, &AdValue::pow(AdValue::offset(AdValue::mul(scratch.ad_value(1369), scratch.ad_value(1374)), 1.0), AdValue::mul(AdValue::neg(scratch.ad_value(542)), scratch.ad_value(603))));
        }

        if ((((scratch.values[1323] != 0.0) && (scratch.values[1340] != 0.0)) && (!(scratch.values[1702] != 0.0))) && (!(scratch.values[1707] != 0.0))) {
            scratch.store_ad(1376, &AdValue::div(AdValue::mul(scratch.ad_value(1365), scratch.ad_value(1375)), AdValue::add(scratch.ad_value(1365), scratch.ad_value(1375))));
        }

        if ((((scratch.values[1323] != 0.0) && (scratch.values[1340] != 0.0)) && (!(scratch.values[1702] != 0.0))) && (!(scratch.values[1707] != 0.0))) {
            scratch.store_ad(1377, &AdValue::sqrt(AdValue::scale(AdValue::div(scratch.ad_value(1369), scratch.ad_value(1373)), 0.375)));
        }

        if ((((scratch.values[1323] != 0.0) && (scratch.values[1340] != 0.0)) && (!(scratch.values[1702] != 0.0))) && (!(scratch.values[1707] != 0.0))) {
            scratch.store_ad(1378, &AdValue::sub(AdValue::scale(AdValue::mul(scratch.ad_value(1370), scratch.ad_value(1373)), 2.0), scratch.ad_value(1372)));
        }

        if ((((scratch.values[1323] != 0.0) && (scratch.values[1340] != 0.0)) && (!(scratch.values[1702] != 0.0))) && (!(scratch.values[1707] != 0.0))) {
            scratch.store_ad(1379, &AdValue::add(AdValue::sub(AdValue::mul(AdValue::mul(scratch.ad_value(627), scratch.ad_value(1370)), scratch.ad_value(1373)), AdValue::mul(scratch.ad_value(627), scratch.ad_value(1372))), AdValue::scale(AdValue::mul(scratch.ad_value(1369), scratch.ad_value(1374)), 0.5)));
        }

        if ((((scratch.values[1323] != 0.0) && (scratch.values[1340] != 0.0)) && (!(scratch.values[1702] != 0.0))) && (!(scratch.values[1707] != 0.0))) {
            scratch.store_ad(1380, &AdValue::mul(AdValue::offset(scratch.ad_value(1378), (-1.0)), scratch.ad_value(1377)));
        }

        if ((((scratch.values[1323] != 0.0) && (scratch.values[1340] != 0.0)) && (!(scratch.values[1702] != 0.0))) && (!(scratch.values[1707] != 0.0))) {
            scratch.store_ad(1341, &AdValue::square(scratch.ad_value(1380)));
        }

        scratch.values[1709] = if (scratch.values[1380] > 0.0) { 1.0 } else { 0.0 };

        if (((((scratch.values[1323] != 0.0) && (scratch.values[1340] != 0.0)) && (!(scratch.values[1702] != 0.0))) && (!(scratch.values[1707] != 0.0))) && (scratch.values[1709] != 0.0)) {
            scratch.store_ad(1342, &AdValue::div_from_scalar(1.0, AdValue::offset(AdValue::scale(scratch.ad_value(1380), scratch.values[421]), 1.0)));
        }

        if (((((scratch.values[1323] != 0.0) && (scratch.values[1340] != 0.0)) && (!(scratch.values[1702] != 0.0))) && (!(scratch.values[1707] != 0.0))) && (!(scratch.values[1709] != 0.0))) {
            scratch.store_ad(1342, &AdValue::div_from_scalar(1.0, AdValue::sub_from_scalar(1.0, AdValue::scale(scratch.ad_value(1380), scratch.values[421]))));
        }

        scratch.values[1710] = if (((-scratch.values[1341]) + scratch.values[1379]) > (-230.25850929940458)) { 1.0 } else { 0.0 };

        if (((((scratch.values[1323] != 0.0) && (scratch.values[1340] != 0.0)) && (!(scratch.values[1702] != 0.0))) && (!(scratch.values[1707] != 0.0))) && (scratch.values[1710] != 0.0)) {
            scratch.store_ad(1359, &AdValue::exp(AdValue::sub(scratch.ad_value(1379), scratch.ad_value(1341))));
        }

        if (((((scratch.values[1323] != 0.0) && (scratch.values[1340] != 0.0)) && (!(scratch.values[1702] != 0.0))) && (!(scratch.values[1707] != 0.0))) && (!(scratch.values[1710] != 0.0))) {
            let assign27750_ad_e32699: AdValue = AdValue::div_from_scalar(1e-100, AdValue::offset(AdValue::mul(AdValue::sub_from_scalar((-230.25850929940458), AdValue::sub(scratch.ad_value(1379), scratch.ad_value(1341))), AdValue::offset(AdValue::scale(AdValue::mul(AdValue::sub_from_scalar((-230.25850929940458), AdValue::sub(scratch.ad_value(1379), scratch.ad_value(1341))), AdValue::offset(AdValue::scale(AdValue::sub_from_scalar((-230.25850929940458), AdValue::sub(scratch.ad_value(1379), scratch.ad_value(1341))), 0.3333333333333333), 1.0)), 0.5), 1.0)), 1.0));
            scratch.store_ad(1359, &assign27750_ad_e32699);
        }

        if ((((scratch.values[1323] != 0.0) && (scratch.values[1340] != 0.0)) && (!(scratch.values[1702] != 0.0))) && (!(scratch.values[1707] != 0.0))) {
            scratch.store_ad(1343, &AdValue::mul(AdValue::add(AdValue::add(AdValue::scale(scratch.ad_value(1342), 0.29214664), AdValue::scale(AdValue::square(scratch.ad_value(1342)), scratch.values[422])), AdValue::scale(AdValue::mul(AdValue::square(scratch.ad_value(1342)), scratch.ad_value(1342)), scratch.values[423])), scratch.ad_value(1359)));
        }

        scratch.values[1711] = if (scratch.values[1380] > 0.0) { 1.0 } else { 0.0 };

        if (((((scratch.values[1323] != 0.0) && (scratch.values[1340] != 0.0)) && (!(scratch.values[1702] != 0.0))) && (!(scratch.values[1707] != 0.0))) && (scratch.values[1711] != 0.0)) {
            scratch.values[1381] = scratch.values[1343];
            scratch.node_derivatives[1381] = scratch.node_derivatives[1343];
            scratch.branch_derivatives[1381] = scratch.branch_derivatives[1343];
        }

        scratch.values[1712] = if (scratch.values[1379] > (-230.25850929940458)) { 1.0 } else { 0.0 };

        if ((((((scratch.values[1323] != 0.0) && (scratch.values[1340] != 0.0)) && (!(scratch.values[1702] != 0.0))) && (!(scratch.values[1707] != 0.0))) && (!(scratch.values[1711] != 0.0))) && (scratch.values[1712] != 0.0)) {
            scratch.store_ad(1359, &AdValue::exp(scratch.ad_value(1379)));
        }

        if ((((((scratch.values[1323] != 0.0) && (scratch.values[1340] != 0.0)) && (!(scratch.values[1702] != 0.0))) && (!(scratch.values[1707] != 0.0))) && (!(scratch.values[1711] != 0.0))) && (!(scratch.values[1712] != 0.0))) {
            scratch.store_ad(1359, &AdValue::div_from_scalar(1e-100, AdValue::offset(AdValue::mul(AdValue::sub_from_scalar((-230.25850929940458), scratch.ad_value(1379)), AdValue::offset(AdValue::scale(AdValue::mul(AdValue::sub_from_scalar((-230.25850929940458), scratch.ad_value(1379)), AdValue::offset(AdValue::scale(AdValue::sub_from_scalar((-230.25850929940458), scratch.ad_value(1379)), 0.3333333333333333), 1.0)), 0.5), 1.0)), 1.0)));
        }

        if (((((scratch.values[1323] != 0.0) && (scratch.values[1340] != 0.0)) && (!(scratch.values[1702] != 0.0))) && (!(scratch.values[1707] != 0.0))) && (!(scratch.values[1711] != 0.0))) {
            scratch.store_ad(1381, &AdValue::sub(AdValue::scale(scratch.ad_value(1359), 2.0), scratch.ad_value(1343)));
        }

        if ((((scratch.values[1323] != 0.0) && (scratch.values[1340] != 0.0)) && (!(scratch.values[1702] != 0.0))) && (!(scratch.values[1707] != 0.0))) {
            scratch.store_ad(1382, &AdValue::scale(AdValue::div(AdValue::mul(scratch.ad_value(627), scratch.ad_value(1381)), scratch.ad_value(1377)), (1.772453850905516 * 0.5)));
        }

        if ((((scratch.values[1323] != 0.0) && (scratch.values[1340] != 0.0)) && (!(scratch.values[1702] != 0.0))) && (!(scratch.values[1707] != 0.0))) {
            scratch.store_ad(1368, &AdValue::mul(scratch.ad_value(556), AdValue::mul(AdValue::mul(scratch.ad_value(1367), scratch.ad_value(1382)), scratch.ad_value(1376))));
        }

        scratch.values[1713] = if (scratch.values[562] == 0.0) { 1.0 } else { 0.0 };

        if ((((scratch.values[1323] != 0.0) && (scratch.values[1340] != 0.0)) && (!(scratch.values[1702] != 0.0))) && (scratch.values[1713] != 0.0)) {
            scratch.values[1383] = 0.0;
            scratch.node_derivatives[1383] = [0.0; Instance::NODE_COUNT];
            scratch.branch_derivatives[1383] = [0.0; Instance::BRANCH_COUNT];
        }

        scratch.values[1714] = if (scratch.values[542] == 0.5) { 1.0 } else { 0.0 };

        if (((((scratch.values[1323] != 0.0) && (scratch.values[1340] != 0.0)) && (!(scratch.values[1702] != 0.0))) && (!(scratch.values[1713] != 0.0))) && (scratch.values[1714] != 0.0)) {
            scratch.store_ad(1359, &AdValue::sqrt(AdValue::mul(AdValue::sub(scratch.ad_value(539), scratch.ad_value(1357)), scratch.ad_value(621))));
        }

        if (((((scratch.values[1323] != 0.0) && (scratch.values[1340] != 0.0)) && (!(scratch.values[1702] != 0.0))) && (!(scratch.values[1713] != 0.0))) && (!(scratch.values[1714] != 0.0))) {
            scratch.store_ad(1359, &AdValue::pow(AdValue::mul(AdValue::sub(scratch.ad_value(539), scratch.ad_value(1357)), scratch.ad_value(621)), scratch.ad_value(542)));
        }

        if ((((scratch.values[1323] != 0.0) && (scratch.values[1340] != 0.0)) && (!(scratch.values[1702] != 0.0))) && (!(scratch.values[1713] != 0.0))) {
            scratch.store_ad(1384, &AdValue::mul(scratch.ad_value(603), AdValue::div(AdValue::mul(AdValue::sub(scratch.ad_value(539), scratch.ad_value(1357)), scratch.ad_value(618)), scratch.ad_value(1359))));
        }

        scratch.values[1715] = if (((((-scratch.values[633]) / scratch.values[1384])) as f64).abs() < 230.25850929940458) { 1.0 } else { 0.0 };

        if (((((scratch.values[1323] != 0.0) && (scratch.values[1340] != 0.0)) && (!(scratch.values[1702] != 0.0))) && (!(scratch.values[1713] != 0.0))) && (scratch.values[1715] != 0.0)) {
            scratch.store_ad(1359, &AdValue::exp(AdValue::div(AdValue::neg(scratch.ad_value(633)), scratch.ad_value(1384))));
        }

        scratch.values[1716] = if (((-scratch.values[633]) / scratch.values[1384]) < 0.0) { 1.0 } else { 0.0 };

        if ((((((scratch.values[1323] != 0.0) && (scratch.values[1340] != 0.0)) && (!(scratch.values[1702] != 0.0))) && (!(scratch.values[1713] != 0.0))) && (!(scratch.values[1715] != 0.0))) && (scratch.values[1716] != 0.0)) {
            let assign27940_ad_e33026: AdValue = AdValue::offset(AdValue::mul(AdValue::sub_from_scalar((-230.25850929940458), AdValue::div(AdValue::neg(scratch.ad_value(633)), scratch.ad_value(1384))), AdValue::offset(AdValue::scale(AdValue::mul(AdValue::sub_from_scalar((-230.25850929940458), AdValue::div(AdValue::neg(scratch.ad_value(633)), scratch.ad_value(1384))), AdValue::offset(AdValue::scale(AdValue::sub_from_scalar((-230.25850929940458), AdValue::div(AdValue::neg(scratch.ad_value(633)), scratch.ad_value(1384))), 0.3333333333333333), 1.0)), 0.5), 1.0)), 1.0);
            scratch.store_ad(1359, &AdValue::div_from_scalar(1e-100, assign27940_ad_e33026));
        }

        if ((((((scratch.values[1323] != 0.0) && (scratch.values[1340] != 0.0)) && (!(scratch.values[1702] != 0.0))) && (!(scratch.values[1713] != 0.0))) && (!(scratch.values[1715] != 0.0))) && (!(scratch.values[1716] != 0.0))) {
            let assign27950_ad_e33076: AdValue = AdValue::scale(AdValue::offset(AdValue::mul(AdValue::offset(AdValue::div(AdValue::neg(scratch.ad_value(633)), scratch.ad_value(1384)), (-230.25850929940458)), AdValue::offset(AdValue::scale(AdValue::mul(AdValue::offset(AdValue::div(AdValue::neg(scratch.ad_value(633)), scratch.ad_value(1384)), (-230.25850929940458)), AdValue::offset(AdValue::scale(AdValue::offset(AdValue::div(AdValue::neg(scratch.ad_value(633)), scratch.ad_value(1384)), (-230.25850929940458)), 0.3333333333333333), 1.0)), 0.5), 1.0)), 1.0), 1e100);
            scratch.store_ad(1359, &assign27950_ad_e33076);
        }

        if ((((scratch.values[1323] != 0.0) && (scratch.values[1340] != 0.0)) && (!(scratch.values[1702] != 0.0))) && (!(scratch.values[1713] != 0.0))) {
            scratch.store_ad(1383, &AdValue::mul(scratch.ad_value(562), AdValue::mul(AdValue::mul(AdValue::mul(scratch.ad_value(516), scratch.ad_value(1384)), scratch.ad_value(1384)), scratch.ad_value(1359))));
        }

        scratch.values[1717] = if (scratch.values[571] > 1000.0) { 1.0 } else { 0.0 };

        if ((((scratch.values[1323] != 0.0) && (scratch.values[1340] != 0.0)) && (!(scratch.values[1702] != 0.0))) && (scratch.values[1717] != 0.0)) {
            scratch.values[1385] = 1.0;
            scratch.node_derivatives[1385] = [0.0; Instance::NODE_COUNT];
            scratch.branch_derivatives[1385] = [0.0; Instance::BRANCH_COUNT];
        }

        scratch.values[1718] = if (scratch.values[1358] > ((-scratch.values[493]) * scratch.values[571])) { 1.0 } else { 0.0 };

        scratch.values[1719] = if (scratch.values[574] == 4.0) { 1.0 } else { 0.0 };

        if ((((((scratch.values[1323] != 0.0) && (scratch.values[1340] != 0.0)) && (!(scratch.values[1702] != 0.0))) && (!(scratch.values[1717] != 0.0))) && (scratch.values[1718] != 0.0)) && (scratch.values[1719] != 0.0)) {
            scratch.store_ad(1359, &AdValue::mul(AdValue::mul(AdValue::mul(AdValue::mul(scratch.ad_value(1358), scratch.ad_value(639)), AdValue::mul(scratch.ad_value(1358), scratch.ad_value(639))), AdValue::mul(scratch.ad_value(1358), scratch.ad_value(639))), AdValue::mul(scratch.ad_value(1358), scratch.ad_value(639))));
        }

        if ((((((scratch.values[1323] != 0.0) && (scratch.values[1340] != 0.0)) && (!(scratch.values[1702] != 0.0))) && (!(scratch.values[1717] != 0.0))) && (scratch.values[1718] != 0.0)) && (!(scratch.values[1719] != 0.0))) {
            scratch.store_ad(1359, &AdValue::pow(AdValue::abs(AdValue::mul(scratch.ad_value(1358), scratch.ad_value(639))), scratch.ad_value(574)));
        }

        if (((((scratch.values[1323] != 0.0) && (scratch.values[1340] != 0.0)) && (!(scratch.values[1702] != 0.0))) && (!(scratch.values[1717] != 0.0))) && (scratch.values[1718] != 0.0)) {
            scratch.store_ad(1385, &AdValue::div_from_scalar(1.0, AdValue::sub_from_scalar(1.0, scratch.ad_value(1359))));
        }

        if (((((scratch.values[1323] != 0.0) && (scratch.values[1340] != 0.0)) && (!(scratch.values[1702] != 0.0))) && (!(scratch.values[1717] != 0.0))) && (!(scratch.values[1718] != 0.0))) {
            scratch.store_ad(1385, &AdValue::add(scratch.ad_value(636), AdValue::mul(AdValue::add(scratch.ad_value(1358), AdValue::scale(scratch.ad_value(571), scratch.values[493])), scratch.ad_value(642))));
        }

        if (((scratch.values[1323] != 0.0) && (scratch.values[1340] != 0.0)) && (!(scratch.values[1702] != 0.0))) {
            scratch.store_ad(1386, &AdValue::mul(AdValue::add(AdValue::add(AdValue::add(scratch.ad_value(1360), scratch.ad_value(1361)), scratch.ad_value(1368)), scratch.ad_value(1383)), scratch.ad_value(1385)));
        }

        scratch.values[1720] = if (scratch.values[718] == 0.0) { 1.0 } else { 0.0 };

        if (((scratch.values[1323] != 0.0) && (scratch.values[1340] != 0.0)) && (scratch.values[1720] != 0.0)) {
            scratch.values[1387] = 0.0;
            scratch.node_derivatives[1387] = [0.0; Instance::NODE_COUNT];
            scratch.branch_derivatives[1387] = [0.0; Instance::BRANCH_COUNT];
        }

        scratch.values[1721] = if (scratch.values[601] == 0.5) { 1.0 } else { 0.0 };

        if ((((scratch.values[1323] != 0.0) && (scratch.values[1340] != 0.0)) && (!(scratch.values[1720] != 0.0))) && (scratch.values[1721] != 0.0)) {
            scratch.store_ad(1359, &AdValue::sqrt(AdValue::sub_from_scalar(1.0, AdValue::mul(scratch.ad_value(1351), scratch.ad_value(598)))));
        }

        if ((((scratch.values[1323] != 0.0) && (scratch.values[1340] != 0.0)) && (!(scratch.values[1720] != 0.0))) && (!(scratch.values[1721] != 0.0))) {
            scratch.store_ad(1359, &AdValue::pow(AdValue::sub_from_scalar(1.0, AdValue::mul(scratch.ad_value(1351), scratch.ad_value(598))), scratch.ad_value(601)));
        }

        if (((scratch.values[1323] != 0.0) && (scratch.values[1340] != 0.0)) && (!(scratch.values[1720] != 0.0))) {
            scratch.store_ad(1360, &AdValue::mul(scratch.ad_value(589), scratch.ad_value(1350)));
        }

        scratch.values[1722] = if ((scratch.values[554] == 0.0) && (scratch.values[557] == 0.0)) { 1.0 } else { 0.0 };

        if ((((scratch.values[1323] != 0.0) && (scratch.values[1340] != 0.0)) && (!(scratch.values[1720] != 0.0))) && (scratch.values[1722] != 0.0)) {
            scratch.values[1361] = 0.0;
            scratch.node_derivatives[1361] = [0.0; Instance::NODE_COUNT];
            scratch.branch_derivatives[1361] = [0.0; Instance::BRANCH_COUNT];
        }

        if ((((scratch.values[1323] != 0.0) && (scratch.values[1340] != 0.0)) && (!(scratch.values[1720] != 0.0))) && (!(scratch.values[1722] != 0.0))) {
            scratch.store_ad(1362, &AdValue::sub(scratch.ad_value(595), scratch.ad_value(1356)));
        }

        if ((((scratch.values[1323] != 0.0) && (scratch.values[1340] != 0.0)) && (!(scratch.values[1720] != 0.0))) && (!(scratch.values[1722] != 0.0))) {
            scratch.store_ad(1363, &AdValue::sub_from_scalar(1.0, AdValue::sqrt(AdValue::sub_from_scalar(1.0, AdValue::div(scratch.ad_value(1354), scratch.ad_value(1362))))));
        }

        scratch.values[1723] = if (scratch.values[543] == 0.5) { 1.0 } else { 0.0 };

        if (((((scratch.values[1323] != 0.0) && (scratch.values[1340] != 0.0)) && (!(scratch.values[1720] != 0.0))) && (!(scratch.values[1722] != 0.0))) && (scratch.values[1723] != 0.0)) {
            scratch.values[1364] = 0.0;
            scratch.node_derivatives[1364] = [0.0; Instance::NODE_COUNT];
            scratch.branch_derivatives[1364] = [0.0; Instance::BRANCH_COUNT];
        }

        if (((((scratch.values[1323] != 0.0) && (scratch.values[1340] != 0.0)) && (!(scratch.values[1720] != 0.0))) && (!(scratch.values[1722] != 0.0))) && (!(scratch.values[1723] != 0.0))) {
            scratch.store_ad(1364, &AdValue::mul(AdValue::add(AdValue::div(AdValue::mul(AdValue::square(scratch.ad_value(1363)), AdValue::ln(scratch.ad_value(1363))), AdValue::sub_from_scalar(1.0, scratch.ad_value(1363))), scratch.ad_value(1363)), AdValue::sub_from_scalar(1.0, AdValue::scale(scratch.ad_value(543), 2.0))));
        }

        if ((((scratch.values[1323] != 0.0) && (scratch.values[1340] != 0.0)) && (!(scratch.values[1720] != 0.0))) && (!(scratch.values[1722] != 0.0))) {
            scratch.store_ad(1365, &AdValue::add(scratch.ad_value(1363), scratch.ad_value(1364)));
        }

        scratch.values[1724] = if (scratch.values[543] == 0.5) { 1.0 } else { 0.0 };

        if (((((scratch.values[1323] != 0.0) && (scratch.values[1340] != 0.0)) && (!(scratch.values[1720] != 0.0))) && (!(scratch.values[1722] != 0.0))) && (scratch.values[1724] != 0.0)) {
            scratch.store_ad(1359, &AdValue::sqrt(AdValue::mul(scratch.ad_value(1362), scratch.ad_value(622))));
        }

        if (((((scratch.values[1323] != 0.0) && (scratch.values[1340] != 0.0)) && (!(scratch.values[1720] != 0.0))) && (!(scratch.values[1722] != 0.0))) && (!(scratch.values[1724] != 0.0))) {
            scratch.store_ad(1359, &AdValue::pow(AdValue::mul(scratch.ad_value(1362), scratch.ad_value(622)), scratch.ad_value(543)));
        }

        if ((((scratch.values[1323] != 0.0) && (scratch.values[1340] != 0.0)) && (!(scratch.values[1720] != 0.0))) && (!(scratch.values[1722] != 0.0))) {
            scratch.store_ad(1366, &AdValue::mul(scratch.ad_value(616), scratch.ad_value(1359)));
        }

        if ((((scratch.values[1323] != 0.0) && (scratch.values[1340] != 0.0)) && (!(scratch.values[1720] != 0.0))) && (!(scratch.values[1722] != 0.0))) {
            scratch.store_ad(1367, &AdValue::mul(scratch.ad_value(586), AdValue::mul(AdValue::offset(scratch.ad_value(1353), (-1.0)), scratch.ad_value(1366))));
        }

        if ((((scratch.values[1323] != 0.0) && (scratch.values[1340] != 0.0)) && (!(scratch.values[1720] != 0.0))) && (!(scratch.values[1722] != 0.0))) {
            scratch.store_ad(1361, &AdValue::mul(scratch.ad_value(554), AdValue::mul(scratch.ad_value(1367), scratch.ad_value(1365))));
        }

        scratch.values[1725] = if (scratch.values[557] == 0.0) { 1.0 } else { 0.0 };

        if ((((scratch.values[1323] != 0.0) && (scratch.values[1340] != 0.0)) && (!(scratch.values[1720] != 0.0))) && (scratch.values[1725] != 0.0)) {
            scratch.values[1368] = 0.0;
            scratch.node_derivatives[1368] = [0.0; Instance::NODE_COUNT];
            scratch.branch_derivatives[1368] = [0.0; Instance::BRANCH_COUNT];
        }

        if ((((scratch.values[1323] != 0.0) && (scratch.values[1340] != 0.0)) && (!(scratch.values[1720] != 0.0))) && (!(scratch.values[1725] != 0.0))) {
            scratch.store_ad(1369, &AdValue::mul(scratch.ad_value(631), AdValue::div(AdValue::mul(scratch.ad_value(1366), scratch.ad_value(601)), scratch.ad_value(1362))));
        }

        if ((((scratch.values[1323] != 0.0) && (scratch.values[1340] != 0.0)) && (!(scratch.values[1720] != 0.0))) && (!(scratch.values[1725] != 0.0))) {
            scratch.store_ad(1370, &AdValue::div(AdValue::scale(scratch.ad_value(628), 0.666666666666667), scratch.ad_value(1369)));
        }

        if ((((scratch.values[1323] != 0.0) && (scratch.values[1340] != 0.0)) && (!(scratch.values[1720] != 0.0))) && (!(scratch.values[1725] != 0.0))) {
            scratch.store_ad(1371, &AdValue::square(scratch.ad_value(1370)));
        }

        if ((((scratch.values[1323] != 0.0) && (scratch.values[1340] != 0.0)) && (!(scratch.values[1720] != 0.0))) && (!(scratch.values[1725] != 0.0))) {
            scratch.store_ad(1372, &AdValue::sqrt(AdValue::div(AdValue::square(scratch.ad_value(1371)), AdValue::offset(AdValue::square(scratch.ad_value(1371)), 1.0))));
        }

        if ((((scratch.values[1323] != 0.0) && (scratch.values[1340] != 0.0)) && (!(scratch.values[1720] != 0.0))) && (!(scratch.values[1725] != 0.0))) {
            scratch.store_ad(1373, &AdValue::sqrt(AdValue::abs(scratch.ad_value(1372))));
        }

        if ((((scratch.values[1323] != 0.0) && (scratch.values[1340] != 0.0)) && (!(scratch.values[1720] != 0.0))) && (!(scratch.values[1725] != 0.0))) {
            scratch.store_ad(1374, &AdValue::mul(scratch.ad_value(1372), scratch.ad_value(1373)));
        }

        scratch.values[1726] = if (((-scratch.values[543]) * scratch.values[604]) == (-1.0)) { 1.0 } else { 0.0 };

        if (((((scratch.values[1323] != 0.0) && (scratch.values[1340] != 0.0)) && (!(scratch.values[1720] != 0.0))) && (!(scratch.values[1725] != 0.0))) && (scratch.values[1726] != 0.0)) {
            scratch.store_ad(1375, &AdValue::div_from_scalar(1.0, AdValue::offset(AdValue::mul(scratch.ad_value(1369), scratch.ad_value(1374)), 1.0)));
        }

        if (((((scratch.values[1323] != 0.0) && (scratch.values[1340] != 0.0)) && (!(scratch.values[1720] != 0.0))) && (!(scratch.values[1725] != 0.0))) && (!(scratch.values[1726] != 0.0))) {
            scratch.store_ad(1375, &AdValue::pow(AdValue::offset(AdValue::mul(scratch.ad_value(1369), scratch.ad_value(1374)), 1.0), AdValue::mul(AdValue::neg(scratch.ad_value(543)), scratch.ad_value(604))));
        }

        if ((((scratch.values[1323] != 0.0) && (scratch.values[1340] != 0.0)) && (!(scratch.values[1720] != 0.0))) && (!(scratch.values[1725] != 0.0))) {
            scratch.store_ad(1376, &AdValue::div(AdValue::mul(scratch.ad_value(1365), scratch.ad_value(1375)), AdValue::add(scratch.ad_value(1365), scratch.ad_value(1375))));
        }

        if ((((scratch.values[1323] != 0.0) && (scratch.values[1340] != 0.0)) && (!(scratch.values[1720] != 0.0))) && (!(scratch.values[1725] != 0.0))) {
            scratch.store_ad(1377, &AdValue::sqrt(AdValue::scale(AdValue::div(scratch.ad_value(1369), scratch.ad_value(1373)), 0.375)));
        }

        if ((((scratch.values[1323] != 0.0) && (scratch.values[1340] != 0.0)) && (!(scratch.values[1720] != 0.0))) && (!(scratch.values[1725] != 0.0))) {
            scratch.store_ad(1378, &AdValue::sub(AdValue::scale(AdValue::mul(scratch.ad_value(1370), scratch.ad_value(1373)), 2.0), scratch.ad_value(1372)));
        }

        if ((((scratch.values[1323] != 0.0) && (scratch.values[1340] != 0.0)) && (!(scratch.values[1720] != 0.0))) && (!(scratch.values[1725] != 0.0))) {
            scratch.store_ad(1379, &AdValue::add(AdValue::sub(AdValue::mul(AdValue::mul(scratch.ad_value(628), scratch.ad_value(1370)), scratch.ad_value(1373)), AdValue::mul(scratch.ad_value(628), scratch.ad_value(1372))), AdValue::scale(AdValue::mul(scratch.ad_value(1369), scratch.ad_value(1374)), 0.5)));
        }

        if ((((scratch.values[1323] != 0.0) && (scratch.values[1340] != 0.0)) && (!(scratch.values[1720] != 0.0))) && (!(scratch.values[1725] != 0.0))) {
            scratch.store_ad(1380, &AdValue::mul(AdValue::offset(scratch.ad_value(1378), (-1.0)), scratch.ad_value(1377)));
        }

        if ((((scratch.values[1323] != 0.0) && (scratch.values[1340] != 0.0)) && (!(scratch.values[1720] != 0.0))) && (!(scratch.values[1725] != 0.0))) {
            scratch.store_ad(1341, &AdValue::square(scratch.ad_value(1380)));
        }

        scratch.values[1727] = if (scratch.values[1380] > 0.0) { 1.0 } else { 0.0 };

        if (((((scratch.values[1323] != 0.0) && (scratch.values[1340] != 0.0)) && (!(scratch.values[1720] != 0.0))) && (!(scratch.values[1725] != 0.0))) && (scratch.values[1727] != 0.0)) {
            scratch.store_ad(1342, &AdValue::div_from_scalar(1.0, AdValue::offset(AdValue::scale(scratch.ad_value(1380), scratch.values[421]), 1.0)));
        }

        if (((((scratch.values[1323] != 0.0) && (scratch.values[1340] != 0.0)) && (!(scratch.values[1720] != 0.0))) && (!(scratch.values[1725] != 0.0))) && (!(scratch.values[1727] != 0.0))) {
            scratch.store_ad(1342, &AdValue::div_from_scalar(1.0, AdValue::sub_from_scalar(1.0, AdValue::scale(scratch.ad_value(1380), scratch.values[421]))));
        }

        scratch.values[1728] = if (((-scratch.values[1341]) + scratch.values[1379]) > (-230.25850929940458)) { 1.0 } else { 0.0 };

        if (((((scratch.values[1323] != 0.0) && (scratch.values[1340] != 0.0)) && (!(scratch.values[1720] != 0.0))) && (!(scratch.values[1725] != 0.0))) && (scratch.values[1728] != 0.0)) {
            scratch.store_ad(1359, &AdValue::exp(AdValue::sub(scratch.ad_value(1379), scratch.ad_value(1341))));
        }

        if (((((scratch.values[1323] != 0.0) && (scratch.values[1340] != 0.0)) && (!(scratch.values[1720] != 0.0))) && (!(scratch.values[1725] != 0.0))) && (!(scratch.values[1728] != 0.0))) {
            let assign28500_ad_e33905: AdValue = AdValue::div_from_scalar(1e-100, AdValue::offset(AdValue::mul(AdValue::sub_from_scalar((-230.25850929940458), AdValue::sub(scratch.ad_value(1379), scratch.ad_value(1341))), AdValue::offset(AdValue::scale(AdValue::mul(AdValue::sub_from_scalar((-230.25850929940458), AdValue::sub(scratch.ad_value(1379), scratch.ad_value(1341))), AdValue::offset(AdValue::scale(AdValue::sub_from_scalar((-230.25850929940458), AdValue::sub(scratch.ad_value(1379), scratch.ad_value(1341))), 0.3333333333333333), 1.0)), 0.5), 1.0)), 1.0));
            scratch.store_ad(1359, &assign28500_ad_e33905);
        }

        if ((((scratch.values[1323] != 0.0) && (scratch.values[1340] != 0.0)) && (!(scratch.values[1720] != 0.0))) && (!(scratch.values[1725] != 0.0))) {
            scratch.store_ad(1343, &AdValue::mul(AdValue::add(AdValue::add(AdValue::scale(scratch.ad_value(1342), 0.29214664), AdValue::scale(AdValue::square(scratch.ad_value(1342)), scratch.values[422])), AdValue::scale(AdValue::mul(AdValue::square(scratch.ad_value(1342)), scratch.ad_value(1342)), scratch.values[423])), scratch.ad_value(1359)));
        }

        scratch.values[1729] = if (scratch.values[1380] > 0.0) { 1.0 } else { 0.0 };

        if (((((scratch.values[1323] != 0.0) && (scratch.values[1340] != 0.0)) && (!(scratch.values[1720] != 0.0))) && (!(scratch.values[1725] != 0.0))) && (scratch.values[1729] != 0.0)) {
            scratch.values[1381] = scratch.values[1343];
            scratch.node_derivatives[1381] = scratch.node_derivatives[1343];
            scratch.branch_derivatives[1381] = scratch.branch_derivatives[1343];
        }

        scratch.values[1730] = if (scratch.values[1379] > (-230.25850929940458)) { 1.0 } else { 0.0 };

        if ((((((scratch.values[1323] != 0.0) && (scratch.values[1340] != 0.0)) && (!(scratch.values[1720] != 0.0))) && (!(scratch.values[1725] != 0.0))) && (!(scratch.values[1729] != 0.0))) && (scratch.values[1730] != 0.0)) {
            scratch.store_ad(1359, &AdValue::exp(scratch.ad_value(1379)));
        }

        if ((((((scratch.values[1323] != 0.0) && (scratch.values[1340] != 0.0)) && (!(scratch.values[1720] != 0.0))) && (!(scratch.values[1725] != 0.0))) && (!(scratch.values[1729] != 0.0))) && (!(scratch.values[1730] != 0.0))) {
            scratch.store_ad(1359, &AdValue::div_from_scalar(1e-100, AdValue::offset(AdValue::mul(AdValue::sub_from_scalar((-230.25850929940458), scratch.ad_value(1379)), AdValue::offset(AdValue::scale(AdValue::mul(AdValue::sub_from_scalar((-230.25850929940458), scratch.ad_value(1379)), AdValue::offset(AdValue::scale(AdValue::sub_from_scalar((-230.25850929940458), scratch.ad_value(1379)), 0.3333333333333333), 1.0)), 0.5), 1.0)), 1.0)));
        }

        if (((((scratch.values[1323] != 0.0) && (scratch.values[1340] != 0.0)) && (!(scratch.values[1720] != 0.0))) && (!(scratch.values[1725] != 0.0))) && (!(scratch.values[1729] != 0.0))) {
            scratch.store_ad(1381, &AdValue::sub(AdValue::scale(scratch.ad_value(1359), 2.0), scratch.ad_value(1343)));
        }

        if ((((scratch.values[1323] != 0.0) && (scratch.values[1340] != 0.0)) && (!(scratch.values[1720] != 0.0))) && (!(scratch.values[1725] != 0.0))) {
            scratch.store_ad(1382, &AdValue::scale(AdValue::div(AdValue::mul(scratch.ad_value(628), scratch.ad_value(1381)), scratch.ad_value(1377)), (1.772453850905516 * 0.5)));
        }

        if ((((scratch.values[1323] != 0.0) && (scratch.values[1340] != 0.0)) && (!(scratch.values[1720] != 0.0))) && (!(scratch.values[1725] != 0.0))) {
            scratch.store_ad(1368, &AdValue::mul(scratch.ad_value(557), AdValue::mul(AdValue::mul(scratch.ad_value(1367), scratch.ad_value(1382)), scratch.ad_value(1376))));
        }

        scratch.values[1731] = if (scratch.values[563] == 0.0) { 1.0 } else { 0.0 };

        if ((((scratch.values[1323] != 0.0) && (scratch.values[1340] != 0.0)) && (!(scratch.values[1720] != 0.0))) && (scratch.values[1731] != 0.0)) {
            scratch.values[1383] = 0.0;
            scratch.node_derivatives[1383] = [0.0; Instance::NODE_COUNT];
            scratch.branch_derivatives[1383] = [0.0; Instance::BRANCH_COUNT];
        }

        scratch.values[1732] = if (scratch.values[543] == 0.5) { 1.0 } else { 0.0 };

        if (((((scratch.values[1323] != 0.0) && (scratch.values[1340] != 0.0)) && (!(scratch.values[1720] != 0.0))) && (!(scratch.values[1731] != 0.0))) && (scratch.values[1732] != 0.0)) {
            scratch.store_ad(1359, &AdValue::sqrt(AdValue::mul(AdValue::sub(scratch.ad_value(540), scratch.ad_value(1357)), scratch.ad_value(622))));
        }

        if (((((scratch.values[1323] != 0.0) && (scratch.values[1340] != 0.0)) && (!(scratch.values[1720] != 0.0))) && (!(scratch.values[1731] != 0.0))) && (!(scratch.values[1732] != 0.0))) {
            scratch.store_ad(1359, &AdValue::pow(AdValue::mul(AdValue::sub(scratch.ad_value(540), scratch.ad_value(1357)), scratch.ad_value(622)), scratch.ad_value(543)));
        }

        if ((((scratch.values[1323] != 0.0) && (scratch.values[1340] != 0.0)) && (!(scratch.values[1720] != 0.0))) && (!(scratch.values[1731] != 0.0))) {
            scratch.store_ad(1384, &AdValue::mul(scratch.ad_value(604), AdValue::div(AdValue::mul(AdValue::sub(scratch.ad_value(540), scratch.ad_value(1357)), scratch.ad_value(619)), scratch.ad_value(1359))));
        }

        scratch.values[1733] = if (((((-scratch.values[634]) / scratch.values[1384])) as f64).abs() < 230.25850929940458) { 1.0 } else { 0.0 };

        if (((((scratch.values[1323] != 0.0) && (scratch.values[1340] != 0.0)) && (!(scratch.values[1720] != 0.0))) && (!(scratch.values[1731] != 0.0))) && (scratch.values[1733] != 0.0)) {
            scratch.store_ad(1359, &AdValue::exp(AdValue::div(AdValue::neg(scratch.ad_value(634)), scratch.ad_value(1384))));
        }

        scratch.values[1734] = if (((-scratch.values[634]) / scratch.values[1384]) < 0.0) { 1.0 } else { 0.0 };

        if ((((((scratch.values[1323] != 0.0) && (scratch.values[1340] != 0.0)) && (!(scratch.values[1720] != 0.0))) && (!(scratch.values[1731] != 0.0))) && (!(scratch.values[1733] != 0.0))) && (scratch.values[1734] != 0.0)) {
            let assign28690_ad_e34232: AdValue = AdValue::offset(AdValue::mul(AdValue::sub_from_scalar((-230.25850929940458), AdValue::div(AdValue::neg(scratch.ad_value(634)), scratch.ad_value(1384))), AdValue::offset(AdValue::scale(AdValue::mul(AdValue::sub_from_scalar((-230.25850929940458), AdValue::div(AdValue::neg(scratch.ad_value(634)), scratch.ad_value(1384))), AdValue::offset(AdValue::scale(AdValue::sub_from_scalar((-230.25850929940458), AdValue::div(AdValue::neg(scratch.ad_value(634)), scratch.ad_value(1384))), 0.3333333333333333), 1.0)), 0.5), 1.0)), 1.0);
            scratch.store_ad(1359, &AdValue::div_from_scalar(1e-100, assign28690_ad_e34232));
        }

        if ((((((scratch.values[1323] != 0.0) && (scratch.values[1340] != 0.0)) && (!(scratch.values[1720] != 0.0))) && (!(scratch.values[1731] != 0.0))) && (!(scratch.values[1733] != 0.0))) && (!(scratch.values[1734] != 0.0))) {
            let assign28700_ad_e34282: AdValue = AdValue::scale(AdValue::offset(AdValue::mul(AdValue::offset(AdValue::div(AdValue::neg(scratch.ad_value(634)), scratch.ad_value(1384)), (-230.25850929940458)), AdValue::offset(AdValue::scale(AdValue::mul(AdValue::offset(AdValue::div(AdValue::neg(scratch.ad_value(634)), scratch.ad_value(1384)), (-230.25850929940458)), AdValue::offset(AdValue::scale(AdValue::offset(AdValue::div(AdValue::neg(scratch.ad_value(634)), scratch.ad_value(1384)), (-230.25850929940458)), 0.3333333333333333), 1.0)), 0.5), 1.0)), 1.0), 1e100);
            scratch.store_ad(1359, &assign28700_ad_e34282);
        }

        if ((((scratch.values[1323] != 0.0) && (scratch.values[1340] != 0.0)) && (!(scratch.values[1720] != 0.0))) && (!(scratch.values[1731] != 0.0))) {
            scratch.store_ad(1383, &AdValue::mul(scratch.ad_value(563), AdValue::mul(AdValue::mul(AdValue::mul(scratch.ad_value(516), scratch.ad_value(1384)), scratch.ad_value(1384)), scratch.ad_value(1359))));
        }

        scratch.values[1735] = if (scratch.values[572] > 1000.0) { 1.0 } else { 0.0 };

        if ((((scratch.values[1323] != 0.0) && (scratch.values[1340] != 0.0)) && (!(scratch.values[1720] != 0.0))) && (scratch.values[1735] != 0.0)) {
            scratch.values[1385] = 1.0;
            scratch.node_derivatives[1385] = [0.0; Instance::NODE_COUNT];
            scratch.branch_derivatives[1385] = [0.0; Instance::BRANCH_COUNT];
        }

        scratch.values[1736] = if (scratch.values[1358] > ((-scratch.values[493]) * scratch.values[572])) { 1.0 } else { 0.0 };

        scratch.values[1737] = if (scratch.values[575] == 4.0) { 1.0 } else { 0.0 };

        if ((((((scratch.values[1323] != 0.0) && (scratch.values[1340] != 0.0)) && (!(scratch.values[1720] != 0.0))) && (!(scratch.values[1735] != 0.0))) && (scratch.values[1736] != 0.0)) && (scratch.values[1737] != 0.0)) {
            scratch.store_ad(1359, &AdValue::mul(AdValue::mul(AdValue::mul(AdValue::mul(scratch.ad_value(1358), scratch.ad_value(640)), AdValue::mul(scratch.ad_value(1358), scratch.ad_value(640))), AdValue::mul(scratch.ad_value(1358), scratch.ad_value(640))), AdValue::mul(scratch.ad_value(1358), scratch.ad_value(640))));
        }

        if ((((((scratch.values[1323] != 0.0) && (scratch.values[1340] != 0.0)) && (!(scratch.values[1720] != 0.0))) && (!(scratch.values[1735] != 0.0))) && (scratch.values[1736] != 0.0)) && (!(scratch.values[1737] != 0.0))) {
            scratch.store_ad(1359, &AdValue::pow(AdValue::abs(AdValue::mul(scratch.ad_value(1358), scratch.ad_value(640))), scratch.ad_value(575)));
        }

    }

    pub(super) fn stamp_transient_block_22(
        &mut self,
        ctx: &GeneratedEvalContext<'_>,
        stamper: &mut GeneratedStamper<'_>,
        scratch: &mut Scratch,
    ) {
        let _ = stamper;
        if (((((scratch.values[1323] != 0.0) && (scratch.values[1340] != 0.0)) && (!(scratch.values[1720] != 0.0))) && (!(scratch.values[1735] != 0.0))) && (scratch.values[1736] != 0.0)) {
            scratch.store_ad(1385, &AdValue::div_from_scalar(1.0, AdValue::sub_from_scalar(1.0, scratch.ad_value(1359))));
        }

        if (((((scratch.values[1323] != 0.0) && (scratch.values[1340] != 0.0)) && (!(scratch.values[1720] != 0.0))) && (!(scratch.values[1735] != 0.0))) && (!(scratch.values[1736] != 0.0))) {
            scratch.store_ad(1385, &AdValue::add(scratch.ad_value(637), AdValue::mul(AdValue::add(scratch.ad_value(1358), AdValue::scale(scratch.ad_value(572), scratch.values[493])), scratch.ad_value(643))));
        }

        if (((scratch.values[1323] != 0.0) && (scratch.values[1340] != 0.0)) && (!(scratch.values[1720] != 0.0))) {
            scratch.store_ad(1387, &AdValue::mul(AdValue::add(AdValue::add(AdValue::add(scratch.ad_value(1360), scratch.ad_value(1361)), scratch.ad_value(1368)), scratch.ad_value(1383)), scratch.ad_value(1385)));
        }

        scratch.values[1738] = if (scratch.values[719] == 0.0) { 1.0 } else { 0.0 };

        if (((scratch.values[1323] != 0.0) && (scratch.values[1340] != 0.0)) && (scratch.values[1738] != 0.0)) {
            scratch.values[1388] = 0.0;
            scratch.node_derivatives[1388] = [0.0; Instance::NODE_COUNT];
            scratch.branch_derivatives[1388] = [0.0; Instance::BRANCH_COUNT];
        }

        scratch.values[1739] = if (scratch.values[602] == 0.5) { 1.0 } else { 0.0 };

        if ((((scratch.values[1323] != 0.0) && (scratch.values[1340] != 0.0)) && (!(scratch.values[1738] != 0.0))) && (scratch.values[1739] != 0.0)) {
            scratch.store_ad(1359, &AdValue::sqrt(AdValue::sub_from_scalar(1.0, AdValue::mul(scratch.ad_value(1351), scratch.ad_value(599)))));
        }

        if ((((scratch.values[1323] != 0.0) && (scratch.values[1340] != 0.0)) && (!(scratch.values[1738] != 0.0))) && (!(scratch.values[1739] != 0.0))) {
            scratch.store_ad(1359, &AdValue::pow(AdValue::sub_from_scalar(1.0, AdValue::mul(scratch.ad_value(1351), scratch.ad_value(599))), scratch.ad_value(602)));
        }

        if (((scratch.values[1323] != 0.0) && (scratch.values[1340] != 0.0)) && (!(scratch.values[1738] != 0.0))) {
            scratch.store_ad(1360, &AdValue::mul(scratch.ad_value(590), scratch.ad_value(1350)));
        }

        scratch.values[1740] = if ((scratch.values[555] == 0.0) && (scratch.values[558] == 0.0)) { 1.0 } else { 0.0 };

        if ((((scratch.values[1323] != 0.0) && (scratch.values[1340] != 0.0)) && (!(scratch.values[1738] != 0.0))) && (scratch.values[1740] != 0.0)) {
            scratch.values[1361] = 0.0;
            scratch.node_derivatives[1361] = [0.0; Instance::NODE_COUNT];
            scratch.branch_derivatives[1361] = [0.0; Instance::BRANCH_COUNT];
        }

        if ((((scratch.values[1323] != 0.0) && (scratch.values[1340] != 0.0)) && (!(scratch.values[1738] != 0.0))) && (!(scratch.values[1740] != 0.0))) {
            scratch.store_ad(1362, &AdValue::sub(scratch.ad_value(596), scratch.ad_value(1356)));
        }

        if ((((scratch.values[1323] != 0.0) && (scratch.values[1340] != 0.0)) && (!(scratch.values[1738] != 0.0))) && (!(scratch.values[1740] != 0.0))) {
            scratch.store_ad(1363, &AdValue::sub_from_scalar(1.0, AdValue::sqrt(AdValue::sub_from_scalar(1.0, AdValue::div(scratch.ad_value(1354), scratch.ad_value(1362))))));
        }

        scratch.values[1741] = if (scratch.values[544] == 0.5) { 1.0 } else { 0.0 };

        if (((((scratch.values[1323] != 0.0) && (scratch.values[1340] != 0.0)) && (!(scratch.values[1738] != 0.0))) && (!(scratch.values[1740] != 0.0))) && (scratch.values[1741] != 0.0)) {
            scratch.values[1364] = 0.0;
            scratch.node_derivatives[1364] = [0.0; Instance::NODE_COUNT];
            scratch.branch_derivatives[1364] = [0.0; Instance::BRANCH_COUNT];
        }

        if (((((scratch.values[1323] != 0.0) && (scratch.values[1340] != 0.0)) && (!(scratch.values[1738] != 0.0))) && (!(scratch.values[1740] != 0.0))) && (!(scratch.values[1741] != 0.0))) {
            scratch.store_ad(1364, &AdValue::mul(AdValue::add(AdValue::div(AdValue::mul(AdValue::square(scratch.ad_value(1363)), AdValue::ln(scratch.ad_value(1363))), AdValue::sub_from_scalar(1.0, scratch.ad_value(1363))), scratch.ad_value(1363)), AdValue::sub_from_scalar(1.0, AdValue::scale(scratch.ad_value(544), 2.0))));
        }

        if ((((scratch.values[1323] != 0.0) && (scratch.values[1340] != 0.0)) && (!(scratch.values[1738] != 0.0))) && (!(scratch.values[1740] != 0.0))) {
            scratch.store_ad(1365, &AdValue::add(scratch.ad_value(1363), scratch.ad_value(1364)));
        }

        scratch.values[1742] = if (scratch.values[544] == 0.5) { 1.0 } else { 0.0 };

        if (((((scratch.values[1323] != 0.0) && (scratch.values[1340] != 0.0)) && (!(scratch.values[1738] != 0.0))) && (!(scratch.values[1740] != 0.0))) && (scratch.values[1742] != 0.0)) {
            scratch.store_ad(1359, &AdValue::sqrt(AdValue::mul(scratch.ad_value(1362), scratch.ad_value(623))));
        }

        if (((((scratch.values[1323] != 0.0) && (scratch.values[1340] != 0.0)) && (!(scratch.values[1738] != 0.0))) && (!(scratch.values[1740] != 0.0))) && (!(scratch.values[1742] != 0.0))) {
            scratch.store_ad(1359, &AdValue::pow(AdValue::mul(scratch.ad_value(1362), scratch.ad_value(623)), scratch.ad_value(544)));
        }

        if ((((scratch.values[1323] != 0.0) && (scratch.values[1340] != 0.0)) && (!(scratch.values[1738] != 0.0))) && (!(scratch.values[1740] != 0.0))) {
            scratch.store_ad(1366, &AdValue::mul(scratch.ad_value(617), scratch.ad_value(1359)));
        }

        if ((((scratch.values[1323] != 0.0) && (scratch.values[1340] != 0.0)) && (!(scratch.values[1738] != 0.0))) && (!(scratch.values[1740] != 0.0))) {
            scratch.store_ad(1367, &AdValue::mul(scratch.ad_value(587), AdValue::mul(AdValue::offset(scratch.ad_value(1353), (-1.0)), scratch.ad_value(1366))));
        }

        if ((((scratch.values[1323] != 0.0) && (scratch.values[1340] != 0.0)) && (!(scratch.values[1738] != 0.0))) && (!(scratch.values[1740] != 0.0))) {
            scratch.store_ad(1361, &AdValue::mul(scratch.ad_value(555), AdValue::mul(scratch.ad_value(1367), scratch.ad_value(1365))));
        }

        scratch.values[1743] = if (scratch.values[558] == 0.0) { 1.0 } else { 0.0 };

        if ((((scratch.values[1323] != 0.0) && (scratch.values[1340] != 0.0)) && (!(scratch.values[1738] != 0.0))) && (scratch.values[1743] != 0.0)) {
            scratch.values[1368] = 0.0;
            scratch.node_derivatives[1368] = [0.0; Instance::NODE_COUNT];
            scratch.branch_derivatives[1368] = [0.0; Instance::BRANCH_COUNT];
        }

        if ((((scratch.values[1323] != 0.0) && (scratch.values[1340] != 0.0)) && (!(scratch.values[1738] != 0.0))) && (!(scratch.values[1743] != 0.0))) {
            scratch.store_ad(1369, &AdValue::mul(scratch.ad_value(632), AdValue::div(AdValue::mul(scratch.ad_value(1366), scratch.ad_value(602)), scratch.ad_value(1362))));
        }

        if ((((scratch.values[1323] != 0.0) && (scratch.values[1340] != 0.0)) && (!(scratch.values[1738] != 0.0))) && (!(scratch.values[1743] != 0.0))) {
            scratch.store_ad(1370, &AdValue::div(AdValue::scale(scratch.ad_value(629), 0.666666666666667), scratch.ad_value(1369)));
        }

        if ((((scratch.values[1323] != 0.0) && (scratch.values[1340] != 0.0)) && (!(scratch.values[1738] != 0.0))) && (!(scratch.values[1743] != 0.0))) {
            scratch.store_ad(1371, &AdValue::square(scratch.ad_value(1370)));
        }

        if ((((scratch.values[1323] != 0.0) && (scratch.values[1340] != 0.0)) && (!(scratch.values[1738] != 0.0))) && (!(scratch.values[1743] != 0.0))) {
            scratch.store_ad(1372, &AdValue::sqrt(AdValue::div(AdValue::square(scratch.ad_value(1371)), AdValue::offset(AdValue::square(scratch.ad_value(1371)), 1.0))));
        }

        if ((((scratch.values[1323] != 0.0) && (scratch.values[1340] != 0.0)) && (!(scratch.values[1738] != 0.0))) && (!(scratch.values[1743] != 0.0))) {
            scratch.store_ad(1373, &AdValue::sqrt(AdValue::abs(scratch.ad_value(1372))));
        }

        if ((((scratch.values[1323] != 0.0) && (scratch.values[1340] != 0.0)) && (!(scratch.values[1738] != 0.0))) && (!(scratch.values[1743] != 0.0))) {
            scratch.store_ad(1374, &AdValue::mul(scratch.ad_value(1372), scratch.ad_value(1373)));
        }

        scratch.values[1744] = if (((-scratch.values[544]) * scratch.values[605]) == (-1.0)) { 1.0 } else { 0.0 };

        if (((((scratch.values[1323] != 0.0) && (scratch.values[1340] != 0.0)) && (!(scratch.values[1738] != 0.0))) && (!(scratch.values[1743] != 0.0))) && (scratch.values[1744] != 0.0)) {
            scratch.store_ad(1375, &AdValue::div_from_scalar(1.0, AdValue::offset(AdValue::mul(scratch.ad_value(1369), scratch.ad_value(1374)), 1.0)));
        }

        if (((((scratch.values[1323] != 0.0) && (scratch.values[1340] != 0.0)) && (!(scratch.values[1738] != 0.0))) && (!(scratch.values[1743] != 0.0))) && (!(scratch.values[1744] != 0.0))) {
            scratch.store_ad(1375, &AdValue::pow(AdValue::offset(AdValue::mul(scratch.ad_value(1369), scratch.ad_value(1374)), 1.0), AdValue::mul(AdValue::neg(scratch.ad_value(544)), scratch.ad_value(605))));
        }

        if ((((scratch.values[1323] != 0.0) && (scratch.values[1340] != 0.0)) && (!(scratch.values[1738] != 0.0))) && (!(scratch.values[1743] != 0.0))) {
            scratch.store_ad(1376, &AdValue::div(AdValue::mul(scratch.ad_value(1365), scratch.ad_value(1375)), AdValue::add(scratch.ad_value(1365), scratch.ad_value(1375))));
        }

        if ((((scratch.values[1323] != 0.0) && (scratch.values[1340] != 0.0)) && (!(scratch.values[1738] != 0.0))) && (!(scratch.values[1743] != 0.0))) {
            scratch.store_ad(1377, &AdValue::sqrt(AdValue::scale(AdValue::div(scratch.ad_value(1369), scratch.ad_value(1373)), 0.375)));
        }

        if ((((scratch.values[1323] != 0.0) && (scratch.values[1340] != 0.0)) && (!(scratch.values[1738] != 0.0))) && (!(scratch.values[1743] != 0.0))) {
            scratch.store_ad(1378, &AdValue::sub(AdValue::scale(AdValue::mul(scratch.ad_value(1370), scratch.ad_value(1373)), 2.0), scratch.ad_value(1372)));
        }

        if ((((scratch.values[1323] != 0.0) && (scratch.values[1340] != 0.0)) && (!(scratch.values[1738] != 0.0))) && (!(scratch.values[1743] != 0.0))) {
            scratch.store_ad(1379, &AdValue::add(AdValue::sub(AdValue::mul(AdValue::mul(scratch.ad_value(629), scratch.ad_value(1370)), scratch.ad_value(1373)), AdValue::mul(scratch.ad_value(629), scratch.ad_value(1372))), AdValue::scale(AdValue::mul(scratch.ad_value(1369), scratch.ad_value(1374)), 0.5)));
        }

        if ((((scratch.values[1323] != 0.0) && (scratch.values[1340] != 0.0)) && (!(scratch.values[1738] != 0.0))) && (!(scratch.values[1743] != 0.0))) {
            scratch.store_ad(1380, &AdValue::mul(AdValue::offset(scratch.ad_value(1378), (-1.0)), scratch.ad_value(1377)));
        }

        if ((((scratch.values[1323] != 0.0) && (scratch.values[1340] != 0.0)) && (!(scratch.values[1738] != 0.0))) && (!(scratch.values[1743] != 0.0))) {
            scratch.store_ad(1341, &AdValue::square(scratch.ad_value(1380)));
        }

        scratch.values[1745] = if (scratch.values[1380] > 0.0) { 1.0 } else { 0.0 };

        if (((((scratch.values[1323] != 0.0) && (scratch.values[1340] != 0.0)) && (!(scratch.values[1738] != 0.0))) && (!(scratch.values[1743] != 0.0))) && (scratch.values[1745] != 0.0)) {
            scratch.store_ad(1342, &AdValue::div_from_scalar(1.0, AdValue::offset(AdValue::scale(scratch.ad_value(1380), scratch.values[421]), 1.0)));
        }

        if (((((scratch.values[1323] != 0.0) && (scratch.values[1340] != 0.0)) && (!(scratch.values[1738] != 0.0))) && (!(scratch.values[1743] != 0.0))) && (!(scratch.values[1745] != 0.0))) {
            scratch.store_ad(1342, &AdValue::div_from_scalar(1.0, AdValue::sub_from_scalar(1.0, AdValue::scale(scratch.ad_value(1380), scratch.values[421]))));
        }

        scratch.values[1746] = if (((-scratch.values[1341]) + scratch.values[1379]) > (-230.25850929940458)) { 1.0 } else { 0.0 };

        if (((((scratch.values[1323] != 0.0) && (scratch.values[1340] != 0.0)) && (!(scratch.values[1738] != 0.0))) && (!(scratch.values[1743] != 0.0))) && (scratch.values[1746] != 0.0)) {
            scratch.store_ad(1359, &AdValue::exp(AdValue::sub(scratch.ad_value(1379), scratch.ad_value(1341))));
        }

        if (((((scratch.values[1323] != 0.0) && (scratch.values[1340] != 0.0)) && (!(scratch.values[1738] != 0.0))) && (!(scratch.values[1743] != 0.0))) && (!(scratch.values[1746] != 0.0))) {
            let assign29250_ad_e35111: AdValue = AdValue::div_from_scalar(1e-100, AdValue::offset(AdValue::mul(AdValue::sub_from_scalar((-230.25850929940458), AdValue::sub(scratch.ad_value(1379), scratch.ad_value(1341))), AdValue::offset(AdValue::scale(AdValue::mul(AdValue::sub_from_scalar((-230.25850929940458), AdValue::sub(scratch.ad_value(1379), scratch.ad_value(1341))), AdValue::offset(AdValue::scale(AdValue::sub_from_scalar((-230.25850929940458), AdValue::sub(scratch.ad_value(1379), scratch.ad_value(1341))), 0.3333333333333333), 1.0)), 0.5), 1.0)), 1.0));
            scratch.store_ad(1359, &assign29250_ad_e35111);
        }

        if ((((scratch.values[1323] != 0.0) && (scratch.values[1340] != 0.0)) && (!(scratch.values[1738] != 0.0))) && (!(scratch.values[1743] != 0.0))) {
            scratch.store_ad(1343, &AdValue::mul(AdValue::add(AdValue::add(AdValue::scale(scratch.ad_value(1342), 0.29214664), AdValue::scale(AdValue::square(scratch.ad_value(1342)), scratch.values[422])), AdValue::scale(AdValue::mul(AdValue::square(scratch.ad_value(1342)), scratch.ad_value(1342)), scratch.values[423])), scratch.ad_value(1359)));
        }

        scratch.values[1747] = if (scratch.values[1380] > 0.0) { 1.0 } else { 0.0 };

        if (((((scratch.values[1323] != 0.0) && (scratch.values[1340] != 0.0)) && (!(scratch.values[1738] != 0.0))) && (!(scratch.values[1743] != 0.0))) && (scratch.values[1747] != 0.0)) {
            scratch.values[1381] = scratch.values[1343];
            scratch.node_derivatives[1381] = scratch.node_derivatives[1343];
            scratch.branch_derivatives[1381] = scratch.branch_derivatives[1343];
        }

        scratch.values[1748] = if (scratch.values[1379] > (-230.25850929940458)) { 1.0 } else { 0.0 };

        if ((((((scratch.values[1323] != 0.0) && (scratch.values[1340] != 0.0)) && (!(scratch.values[1738] != 0.0))) && (!(scratch.values[1743] != 0.0))) && (!(scratch.values[1747] != 0.0))) && (scratch.values[1748] != 0.0)) {
            scratch.store_ad(1359, &AdValue::exp(scratch.ad_value(1379)));
        }

        if ((((((scratch.values[1323] != 0.0) && (scratch.values[1340] != 0.0)) && (!(scratch.values[1738] != 0.0))) && (!(scratch.values[1743] != 0.0))) && (!(scratch.values[1747] != 0.0))) && (!(scratch.values[1748] != 0.0))) {
            scratch.store_ad(1359, &AdValue::div_from_scalar(1e-100, AdValue::offset(AdValue::mul(AdValue::sub_from_scalar((-230.25850929940458), scratch.ad_value(1379)), AdValue::offset(AdValue::scale(AdValue::mul(AdValue::sub_from_scalar((-230.25850929940458), scratch.ad_value(1379)), AdValue::offset(AdValue::scale(AdValue::sub_from_scalar((-230.25850929940458), scratch.ad_value(1379)), 0.3333333333333333), 1.0)), 0.5), 1.0)), 1.0)));
        }

        if (((((scratch.values[1323] != 0.0) && (scratch.values[1340] != 0.0)) && (!(scratch.values[1738] != 0.0))) && (!(scratch.values[1743] != 0.0))) && (!(scratch.values[1747] != 0.0))) {
            scratch.store_ad(1381, &AdValue::sub(AdValue::scale(scratch.ad_value(1359), 2.0), scratch.ad_value(1343)));
        }

        if ((((scratch.values[1323] != 0.0) && (scratch.values[1340] != 0.0)) && (!(scratch.values[1738] != 0.0))) && (!(scratch.values[1743] != 0.0))) {
            scratch.store_ad(1382, &AdValue::scale(AdValue::div(AdValue::mul(scratch.ad_value(629), scratch.ad_value(1381)), scratch.ad_value(1377)), (1.772453850905516 * 0.5)));
        }

        if ((((scratch.values[1323] != 0.0) && (scratch.values[1340] != 0.0)) && (!(scratch.values[1738] != 0.0))) && (!(scratch.values[1743] != 0.0))) {
            scratch.store_ad(1368, &AdValue::mul(scratch.ad_value(558), AdValue::mul(AdValue::mul(scratch.ad_value(1367), scratch.ad_value(1382)), scratch.ad_value(1376))));
        }

        scratch.values[1749] = if (scratch.values[564] == 0.0) { 1.0 } else { 0.0 };

        if ((((scratch.values[1323] != 0.0) && (scratch.values[1340] != 0.0)) && (!(scratch.values[1738] != 0.0))) && (scratch.values[1749] != 0.0)) {
            scratch.values[1383] = 0.0;
            scratch.node_derivatives[1383] = [0.0; Instance::NODE_COUNT];
            scratch.branch_derivatives[1383] = [0.0; Instance::BRANCH_COUNT];
        }

        scratch.values[1750] = if (scratch.values[544] == 0.5) { 1.0 } else { 0.0 };

        if (((((scratch.values[1323] != 0.0) && (scratch.values[1340] != 0.0)) && (!(scratch.values[1738] != 0.0))) && (!(scratch.values[1749] != 0.0))) && (scratch.values[1750] != 0.0)) {
            scratch.store_ad(1359, &AdValue::sqrt(AdValue::mul(AdValue::sub(scratch.ad_value(541), scratch.ad_value(1357)), scratch.ad_value(623))));
        }

        if (((((scratch.values[1323] != 0.0) && (scratch.values[1340] != 0.0)) && (!(scratch.values[1738] != 0.0))) && (!(scratch.values[1749] != 0.0))) && (!(scratch.values[1750] != 0.0))) {
            scratch.store_ad(1359, &AdValue::pow(AdValue::mul(AdValue::sub(scratch.ad_value(541), scratch.ad_value(1357)), scratch.ad_value(623)), scratch.ad_value(544)));
        }

        if ((((scratch.values[1323] != 0.0) && (scratch.values[1340] != 0.0)) && (!(scratch.values[1738] != 0.0))) && (!(scratch.values[1749] != 0.0))) {
            scratch.store_ad(1384, &AdValue::mul(scratch.ad_value(605), AdValue::div(AdValue::mul(AdValue::sub(scratch.ad_value(541), scratch.ad_value(1357)), scratch.ad_value(620)), scratch.ad_value(1359))));
        }

        scratch.values[1751] = if (((((-scratch.values[635]) / scratch.values[1384])) as f64).abs() < 230.25850929940458) { 1.0 } else { 0.0 };

        if (((((scratch.values[1323] != 0.0) && (scratch.values[1340] != 0.0)) && (!(scratch.values[1738] != 0.0))) && (!(scratch.values[1749] != 0.0))) && (scratch.values[1751] != 0.0)) {
            scratch.store_ad(1359, &AdValue::exp(AdValue::div(AdValue::neg(scratch.ad_value(635)), scratch.ad_value(1384))));
        }

        scratch.values[1752] = if (((-scratch.values[635]) / scratch.values[1384]) < 0.0) { 1.0 } else { 0.0 };

        if ((((((scratch.values[1323] != 0.0) && (scratch.values[1340] != 0.0)) && (!(scratch.values[1738] != 0.0))) && (!(scratch.values[1749] != 0.0))) && (!(scratch.values[1751] != 0.0))) && (scratch.values[1752] != 0.0)) {
            let assign29440_ad_e35438: AdValue = AdValue::offset(AdValue::mul(AdValue::sub_from_scalar((-230.25850929940458), AdValue::div(AdValue::neg(scratch.ad_value(635)), scratch.ad_value(1384))), AdValue::offset(AdValue::scale(AdValue::mul(AdValue::sub_from_scalar((-230.25850929940458), AdValue::div(AdValue::neg(scratch.ad_value(635)), scratch.ad_value(1384))), AdValue::offset(AdValue::scale(AdValue::sub_from_scalar((-230.25850929940458), AdValue::div(AdValue::neg(scratch.ad_value(635)), scratch.ad_value(1384))), 0.3333333333333333), 1.0)), 0.5), 1.0)), 1.0);
            scratch.store_ad(1359, &AdValue::div_from_scalar(1e-100, assign29440_ad_e35438));
        }

        if ((((((scratch.values[1323] != 0.0) && (scratch.values[1340] != 0.0)) && (!(scratch.values[1738] != 0.0))) && (!(scratch.values[1749] != 0.0))) && (!(scratch.values[1751] != 0.0))) && (!(scratch.values[1752] != 0.0))) {
            let assign29450_ad_e35488: AdValue = AdValue::scale(AdValue::offset(AdValue::mul(AdValue::offset(AdValue::div(AdValue::neg(scratch.ad_value(635)), scratch.ad_value(1384)), (-230.25850929940458)), AdValue::offset(AdValue::scale(AdValue::mul(AdValue::offset(AdValue::div(AdValue::neg(scratch.ad_value(635)), scratch.ad_value(1384)), (-230.25850929940458)), AdValue::offset(AdValue::scale(AdValue::offset(AdValue::div(AdValue::neg(scratch.ad_value(635)), scratch.ad_value(1384)), (-230.25850929940458)), 0.3333333333333333), 1.0)), 0.5), 1.0)), 1.0), 1e100);
            scratch.store_ad(1359, &assign29450_ad_e35488);
        }

        if ((((scratch.values[1323] != 0.0) && (scratch.values[1340] != 0.0)) && (!(scratch.values[1738] != 0.0))) && (!(scratch.values[1749] != 0.0))) {
            scratch.store_ad(1383, &AdValue::mul(scratch.ad_value(564), AdValue::mul(AdValue::mul(AdValue::mul(scratch.ad_value(516), scratch.ad_value(1384)), scratch.ad_value(1384)), scratch.ad_value(1359))));
        }

        scratch.values[1753] = if (scratch.values[573] > 1000.0) { 1.0 } else { 0.0 };

        if ((((scratch.values[1323] != 0.0) && (scratch.values[1340] != 0.0)) && (!(scratch.values[1738] != 0.0))) && (scratch.values[1753] != 0.0)) {
            scratch.values[1385] = 1.0;
            scratch.node_derivatives[1385] = [0.0; Instance::NODE_COUNT];
            scratch.branch_derivatives[1385] = [0.0; Instance::BRANCH_COUNT];
        }

        scratch.values[1754] = if (scratch.values[1358] > ((-scratch.values[493]) * scratch.values[573])) { 1.0 } else { 0.0 };

        scratch.values[1755] = if (scratch.values[576] == 4.0) { 1.0 } else { 0.0 };

        if ((((((scratch.values[1323] != 0.0) && (scratch.values[1340] != 0.0)) && (!(scratch.values[1738] != 0.0))) && (!(scratch.values[1753] != 0.0))) && (scratch.values[1754] != 0.0)) && (scratch.values[1755] != 0.0)) {
            scratch.store_ad(1359, &AdValue::mul(AdValue::mul(AdValue::mul(AdValue::mul(scratch.ad_value(1358), scratch.ad_value(641)), AdValue::mul(scratch.ad_value(1358), scratch.ad_value(641))), AdValue::mul(scratch.ad_value(1358), scratch.ad_value(641))), AdValue::mul(scratch.ad_value(1358), scratch.ad_value(641))));
        }

        if ((((((scratch.values[1323] != 0.0) && (scratch.values[1340] != 0.0)) && (!(scratch.values[1738] != 0.0))) && (!(scratch.values[1753] != 0.0))) && (scratch.values[1754] != 0.0)) && (!(scratch.values[1755] != 0.0))) {
            scratch.store_ad(1359, &AdValue::pow(AdValue::abs(AdValue::mul(scratch.ad_value(1358), scratch.ad_value(641))), scratch.ad_value(576)));
        }

        if (((((scratch.values[1323] != 0.0) && (scratch.values[1340] != 0.0)) && (!(scratch.values[1738] != 0.0))) && (!(scratch.values[1753] != 0.0))) && (scratch.values[1754] != 0.0)) {
            scratch.store_ad(1385, &AdValue::div_from_scalar(1.0, AdValue::sub_from_scalar(1.0, scratch.ad_value(1359))));
        }

        if (((((scratch.values[1323] != 0.0) && (scratch.values[1340] != 0.0)) && (!(scratch.values[1738] != 0.0))) && (!(scratch.values[1753] != 0.0))) && (!(scratch.values[1754] != 0.0))) {
            scratch.store_ad(1385, &AdValue::add(scratch.ad_value(638), AdValue::mul(AdValue::add(scratch.ad_value(1358), AdValue::scale(scratch.ad_value(573), scratch.values[493])), scratch.ad_value(644))));
        }

        if (((scratch.values[1323] != 0.0) && (scratch.values[1340] != 0.0)) && (!(scratch.values[1738] != 0.0))) {
            scratch.store_ad(1388, &AdValue::mul(AdValue::add(AdValue::add(AdValue::add(scratch.ad_value(1360), scratch.ad_value(1361)), scratch.ad_value(1368)), scratch.ad_value(1383)), scratch.ad_value(1385)));
        }

        if ((scratch.values[1323] != 0.0) && (scratch.values[1340] != 0.0)) {
            scratch.store_ad(506, &AdValue::add(AdValue::add(AdValue::mul(scratch.ad_value(717), scratch.ad_value(1386)), AdValue::mul(scratch.ad_value(718), scratch.ad_value(1387))), AdValue::mul(scratch.ad_value(719), scratch.ad_value(1388))));
        }

        if ((scratch.values[1323] != 0.0) && (scratch.values[1340] != 0.0)) {
            scratch.values[1357] = 0.0;
            scratch.node_derivatives[1357] = [0.0; Instance::NODE_COUNT];
            scratch.branch_derivatives[1357] = [0.0; Instance::BRANCH_COUNT];
        }

        if ((scratch.values[1323] != 0.0) && (scratch.values[1340] != 0.0)) {
            scratch.values[1354] = 0.0;
            scratch.node_derivatives[1354] = [0.0; Instance::NODE_COUNT];
            scratch.branch_derivatives[1354] = [0.0; Instance::BRANCH_COUNT];
        }

        scratch.values[1756] = if !(((scratch.values[717] == 0.0) && (scratch.values[718] == 0.0)) && (scratch.values[719] == 0.0)) { 1.0 } else { 0.0 };

        if (((scratch.values[1323] != 0.0) && (scratch.values[1340] != 0.0)) && (scratch.values[1756] != 0.0)) {
            scratch.store_ad(1344, &AdValue::mul(AdValue::scale(scratch.ad_value(728), 4.0), scratch.ad_value(728)));
        }

        if (((scratch.values[1323] != 0.0) && (scratch.values[1340] != 0.0)) && (scratch.values[1756] != 0.0)) {
            scratch.store_ad(1345, &AdValue::div(scratch.ad_value(728), scratch.ad_value(729)));
        }

        if (((scratch.values[1323] != 0.0) && (scratch.values[1340] != 0.0)) && (scratch.values[1756] != 0.0)) {
            scratch.store_ad(1346, &AdValue::add(scratch.ad_value(517), AdValue::mul(scratch.ad_value(728), scratch.ad_value(1345))));
        }

        if (((scratch.values[1323] != 0.0) && (scratch.values[1340] != 0.0)) && (scratch.values[1756] != 0.0)) {
            scratch.store_ad(1347, &AdValue::add(scratch.ad_value(729), scratch.ad_value(1346)));
        }

        if (((scratch.values[1323] != 0.0) && (scratch.values[1340] != 0.0)) && (scratch.values[1756] != 0.0)) {
            scratch.store_ad(1348, &AdValue::sub(scratch.ad_value(729), scratch.ad_value(1346)));
        }

        if (((scratch.values[1323] != 0.0) && (scratch.values[1340] != 0.0)) && (scratch.values[1756] != 0.0)) {
            scratch.store_ad(1349, &AdValue::sqrt(AdValue::add(AdValue::square(scratch.ad_value(1348)), scratch.ad_value(1344))));
        }

        if (((scratch.values[1323] != 0.0) && (scratch.values[1340] != 0.0)) && (scratch.values[1756] != 0.0)) {
            scratch.store_ad(1351, &AdValue::scale(AdValue::div(AdValue::mul(scratch.ad_value(517), scratch.ad_value(729)), AdValue::add(scratch.ad_value(1347), scratch.ad_value(1349))), 2.0));
        }

        scratch.values[1757] = if (scratch.values[517] < scratch.values[725]) { 1.0 } else { 0.0 };

        scratch.values[1758] = if ((((0.5 * (scratch.values[517] * scratch.values[420]))) as f64).abs() < 230.25850929940458) { 1.0 } else { 0.0 };

        if (((((scratch.values[1323] != 0.0) && (scratch.values[1340] != 0.0)) && (scratch.values[1756] != 0.0)) && (scratch.values[1757] != 0.0)) && (scratch.values[1758] != 0.0)) {
            scratch.store_ad(1353, &AdValue::exp(AdValue::scale(scratch.ad_value(517), (scratch.values[420] * 0.5))));
        }

        scratch.values[1759] = if ((0.5 * (scratch.values[517] * scratch.values[420])) < 0.0) { 1.0 } else { 0.0 };

        if ((((((scratch.values[1323] != 0.0) && (scratch.values[1340] != 0.0)) && (scratch.values[1756] != 0.0)) && (scratch.values[1757] != 0.0)) && (!(scratch.values[1758] != 0.0))) && (scratch.values[1759] != 0.0)) {
            let assign29710_ad_e35851: AdValue = AdValue::div_from_scalar(1e-100, AdValue::offset(AdValue::mul(AdValue::sub_from_scalar((-230.25850929940458), AdValue::scale(scratch.ad_value(517), (scratch.values[420] * 0.5))), AdValue::offset(AdValue::scale(AdValue::mul(AdValue::sub_from_scalar((-230.25850929940458), AdValue::scale(scratch.ad_value(517), (scratch.values[420] * 0.5))), AdValue::offset(AdValue::scale(AdValue::sub_from_scalar((-230.25850929940458), AdValue::scale(scratch.ad_value(517), (scratch.values[420] * 0.5))), 0.3333333333333333), 1.0)), 0.5), 1.0)), 1.0));
            scratch.store_ad(1353, &assign29710_ad_e35851);
        }

        if ((((((scratch.values[1323] != 0.0) && (scratch.values[1340] != 0.0)) && (scratch.values[1756] != 0.0)) && (scratch.values[1757] != 0.0)) && (!(scratch.values[1758] != 0.0))) && (!(scratch.values[1759] != 0.0))) {
            scratch.store_ad(1353, &AdValue::scale(AdValue::offset(AdValue::mul(AdValue::offset(AdValue::scale(scratch.ad_value(517), (scratch.values[420] * 0.5)), (-230.25850929940458)), AdValue::offset(AdValue::scale(AdValue::mul(AdValue::offset(AdValue::scale(scratch.ad_value(517), (scratch.values[420] * 0.5)), (-230.25850929940458)), AdValue::offset(AdValue::scale(AdValue::offset(AdValue::scale(scratch.ad_value(517), (scratch.values[420] * 0.5)), (-230.25850929940458)), 0.3333333333333333), 1.0)), 0.5), 1.0)), 1.0), 1e100));
        }

        if ((((scratch.values[1323] != 0.0) && (scratch.values[1340] != 0.0)) && (scratch.values[1756] != 0.0)) && (scratch.values[1757] != 0.0)) {
            scratch.store_ad(1350, &AdValue::square(scratch.ad_value(1353)));
        }

        if ((((scratch.values[1323] != 0.0) && (scratch.values[1340] != 0.0)) && (scratch.values[1756] != 0.0)) && (!(scratch.values[1757] != 0.0))) {
            scratch.store_ad(1350, &AdValue::mul(AdValue::offset(AdValue::scale(AdValue::sub(scratch.ad_value(517), scratch.ad_value(725)), scratch.values[420]), 1.0), scratch.ad_value(726)));
        }

        if ((((scratch.values[1323] != 0.0) && (scratch.values[1340] != 0.0)) && (scratch.values[1756] != 0.0)) && (!(scratch.values[1757] != 0.0))) {
            scratch.store_ad(1353, &AdValue::sqrt(scratch.ad_value(1350)));
        }

        if (((scratch.values[1323] != 0.0) && (scratch.values[1340] != 0.0)) && (scratch.values[1756] != 0.0)) {
            scratch.store_ad(1350, &AdValue::offset(scratch.ad_value(1350), (-1.0)));
        }

        if (((scratch.values[1323] != 0.0) && (scratch.values[1340] != 0.0)) && (scratch.values[1756] != 0.0)) {
            scratch.store_ad(1352, &AdValue::div_from_scalar(1.0, scratch.ad_value(1353)));
        }

        scratch.values[1760] = if (scratch.values[517] > 0.0) { 1.0 } else { 0.0 };

        if ((((scratch.values[1323] != 0.0) && (scratch.values[1340] != 0.0)) && (scratch.values[1756] != 0.0)) && (scratch.values[1760] != 0.0)) {
            scratch.store_ad(1354, &AdValue::scale(AdValue::ln(AdValue::add(AdValue::offset(scratch.ad_value(1352), 2.0), AdValue::sqrt(AdValue::mul(AdValue::offset(scratch.ad_value(1352), 1.0), AdValue::offset(scratch.ad_value(1352), 3.0))))), (scratch.values[419] * 2.0)));
        }

        if ((((scratch.values[1323] != 0.0) && (scratch.values[1340] != 0.0)) && (scratch.values[1756] != 0.0)) && (!(scratch.values[1760] != 0.0))) {
            scratch.store_ad(1354, &AdValue::sub(AdValue::scale(AdValue::ln(AdValue::add(AdValue::offset(AdValue::scale(scratch.ad_value(1353), 2.0), 1.0), AdValue::sqrt(AdValue::mul(AdValue::offset(scratch.ad_value(1353), 1.0), AdValue::offset(AdValue::scale(scratch.ad_value(1353), 3.0), 1.0))))), (scratch.values[419] * 2.0)), scratch.ad_value(517)));
        }

        if (((scratch.values[1323] != 0.0) && (scratch.values[1340] != 0.0)) && (scratch.values[1756] != 0.0)) {
            scratch.store_ad(1355, &AdValue::sub(scratch.ad_value(727), scratch.ad_value(1354)));
        }

        if (((scratch.values[1323] != 0.0) && (scratch.values[1340] != 0.0)) && (scratch.values[1756] != 0.0)) {
            scratch.store_ad(1356, &AdValue::scale(AdValue::sub(AdValue::add(scratch.ad_value(517), scratch.ad_value(1355)), AdValue::sqrt(AdValue::offset(AdValue::mul(AdValue::sub(scratch.ad_value(517), scratch.ad_value(1355)), AdValue::sub(scratch.ad_value(517), scratch.ad_value(1355))), ((4.0 * scratch.values[419]) * scratch.values[419])))), 0.5));
        }

        if (((scratch.values[1323] != 0.0) && (scratch.values[1340] != 0.0)) && (scratch.values[1756] != 0.0)) {
            scratch.store_ad(1357, &AdValue::scale(AdValue::sub(AdValue::add(scratch.ad_value(517), scratch.ad_value(730)), AdValue::sqrt(AdValue::offset(AdValue::mul(AdValue::sub(scratch.ad_value(517), scratch.ad_value(730)), AdValue::sub(scratch.ad_value(517), scratch.ad_value(730))), ((4.0 * scratch.values[417]) * scratch.values[417])))), 0.5));
        }

        if (((scratch.values[1323] != 0.0) && (scratch.values[1340] != 0.0)) && (scratch.values[1756] != 0.0)) {
            scratch.store_ad(1358, &AdValue::scale(AdValue::sub(scratch.ad_value(517), AdValue::sqrt(AdValue::offset(AdValue::mul(scratch.ad_value(517), scratch.ad_value(517)), ((4.0 * 1e-6) * 1e-6)))), 0.5));
        }

        scratch.values[1761] = if (scratch.values[717] == 0.0) { 1.0 } else { 0.0 };

        if (((scratch.values[1323] != 0.0) && (scratch.values[1340] != 0.0)) && (scratch.values[1761] != 0.0)) {
            scratch.values[1386] = 0.0;
            scratch.node_derivatives[1386] = [0.0; Instance::NODE_COUNT];
            scratch.branch_derivatives[1386] = [0.0; Instance::BRANCH_COUNT];
        }

        scratch.values[1762] = if (scratch.values[600] == 0.5) { 1.0 } else { 0.0 };

        if ((((scratch.values[1323] != 0.0) && (scratch.values[1340] != 0.0)) && (!(scratch.values[1761] != 0.0))) && (scratch.values[1762] != 0.0)) {
            scratch.store_ad(1359, &AdValue::sqrt(AdValue::sub_from_scalar(1.0, AdValue::mul(scratch.ad_value(1351), scratch.ad_value(597)))));
        }

        if ((((scratch.values[1323] != 0.0) && (scratch.values[1340] != 0.0)) && (!(scratch.values[1761] != 0.0))) && (!(scratch.values[1762] != 0.0))) {
            scratch.store_ad(1359, &AdValue::pow(AdValue::sub_from_scalar(1.0, AdValue::mul(scratch.ad_value(1351), scratch.ad_value(597))), scratch.ad_value(600)));
        }

        if (((scratch.values[1323] != 0.0) && (scratch.values[1340] != 0.0)) && (!(scratch.values[1761] != 0.0))) {
            scratch.store_ad(1360, &AdValue::mul(scratch.ad_value(588), scratch.ad_value(1350)));
        }

        scratch.values[1763] = if ((scratch.values[553] == 0.0) && (scratch.values[556] == 0.0)) { 1.0 } else { 0.0 };

        if ((((scratch.values[1323] != 0.0) && (scratch.values[1340] != 0.0)) && (!(scratch.values[1761] != 0.0))) && (scratch.values[1763] != 0.0)) {
            scratch.values[1361] = 0.0;
            scratch.node_derivatives[1361] = [0.0; Instance::NODE_COUNT];
            scratch.branch_derivatives[1361] = [0.0; Instance::BRANCH_COUNT];
        }

        if ((((scratch.values[1323] != 0.0) && (scratch.values[1340] != 0.0)) && (!(scratch.values[1761] != 0.0))) && (!(scratch.values[1763] != 0.0))) {
            scratch.store_ad(1362, &AdValue::sub(scratch.ad_value(594), scratch.ad_value(1356)));
        }

        if ((((scratch.values[1323] != 0.0) && (scratch.values[1340] != 0.0)) && (!(scratch.values[1761] != 0.0))) && (!(scratch.values[1763] != 0.0))) {
            scratch.store_ad(1363, &AdValue::sub_from_scalar(1.0, AdValue::sqrt(AdValue::sub_from_scalar(1.0, AdValue::div(scratch.ad_value(1354), scratch.ad_value(1362))))));
        }

        scratch.values[1764] = if (scratch.values[542] == 0.5) { 1.0 } else { 0.0 };

        if (((((scratch.values[1323] != 0.0) && (scratch.values[1340] != 0.0)) && (!(scratch.values[1761] != 0.0))) && (!(scratch.values[1763] != 0.0))) && (scratch.values[1764] != 0.0)) {
            scratch.values[1364] = 0.0;
            scratch.node_derivatives[1364] = [0.0; Instance::NODE_COUNT];
            scratch.branch_derivatives[1364] = [0.0; Instance::BRANCH_COUNT];
        }

        if (((((scratch.values[1323] != 0.0) && (scratch.values[1340] != 0.0)) && (!(scratch.values[1761] != 0.0))) && (!(scratch.values[1763] != 0.0))) && (!(scratch.values[1764] != 0.0))) {
            scratch.store_ad(1364, &AdValue::mul(AdValue::add(AdValue::div(AdValue::mul(AdValue::square(scratch.ad_value(1363)), AdValue::ln(scratch.ad_value(1363))), AdValue::sub_from_scalar(1.0, scratch.ad_value(1363))), scratch.ad_value(1363)), AdValue::sub_from_scalar(1.0, AdValue::scale(scratch.ad_value(542), 2.0))));
        }

        if ((((scratch.values[1323] != 0.0) && (scratch.values[1340] != 0.0)) && (!(scratch.values[1761] != 0.0))) && (!(scratch.values[1763] != 0.0))) {
            scratch.store_ad(1365, &AdValue::add(scratch.ad_value(1363), scratch.ad_value(1364)));
        }

        scratch.values[1765] = if (scratch.values[542] == 0.5) { 1.0 } else { 0.0 };

        if (((((scratch.values[1323] != 0.0) && (scratch.values[1340] != 0.0)) && (!(scratch.values[1761] != 0.0))) && (!(scratch.values[1763] != 0.0))) && (scratch.values[1765] != 0.0)) {
            scratch.store_ad(1359, &AdValue::sqrt(AdValue::mul(scratch.ad_value(1362), scratch.ad_value(621))));
        }

        if (((((scratch.values[1323] != 0.0) && (scratch.values[1340] != 0.0)) && (!(scratch.values[1761] != 0.0))) && (!(scratch.values[1763] != 0.0))) && (!(scratch.values[1765] != 0.0))) {
            scratch.store_ad(1359, &AdValue::pow(AdValue::mul(scratch.ad_value(1362), scratch.ad_value(621)), scratch.ad_value(542)));
        }

        if ((((scratch.values[1323] != 0.0) && (scratch.values[1340] != 0.0)) && (!(scratch.values[1761] != 0.0))) && (!(scratch.values[1763] != 0.0))) {
            scratch.store_ad(1366, &AdValue::mul(scratch.ad_value(615), scratch.ad_value(1359)));
        }

        if ((((scratch.values[1323] != 0.0) && (scratch.values[1340] != 0.0)) && (!(scratch.values[1761] != 0.0))) && (!(scratch.values[1763] != 0.0))) {
            scratch.store_ad(1367, &AdValue::mul(scratch.ad_value(585), AdValue::mul(AdValue::offset(scratch.ad_value(1353), (-1.0)), scratch.ad_value(1366))));
        }

        if ((((scratch.values[1323] != 0.0) && (scratch.values[1340] != 0.0)) && (!(scratch.values[1761] != 0.0))) && (!(scratch.values[1763] != 0.0))) {
            scratch.store_ad(1361, &AdValue::mul(scratch.ad_value(553), AdValue::mul(scratch.ad_value(1367), scratch.ad_value(1365))));
        }

        scratch.values[1766] = if (scratch.values[556] == 0.0) { 1.0 } else { 0.0 };

        if ((((scratch.values[1323] != 0.0) && (scratch.values[1340] != 0.0)) && (!(scratch.values[1761] != 0.0))) && (scratch.values[1766] != 0.0)) {
            scratch.values[1368] = 0.0;
            scratch.node_derivatives[1368] = [0.0; Instance::NODE_COUNT];
            scratch.branch_derivatives[1368] = [0.0; Instance::BRANCH_COUNT];
        }

        if ((((scratch.values[1323] != 0.0) && (scratch.values[1340] != 0.0)) && (!(scratch.values[1761] != 0.0))) && (!(scratch.values[1766] != 0.0))) {
            scratch.store_ad(1369, &AdValue::mul(scratch.ad_value(630), AdValue::div(AdValue::mul(scratch.ad_value(1366), scratch.ad_value(600)), scratch.ad_value(1362))));
        }

        if ((((scratch.values[1323] != 0.0) && (scratch.values[1340] != 0.0)) && (!(scratch.values[1761] != 0.0))) && (!(scratch.values[1766] != 0.0))) {
            scratch.store_ad(1370, &AdValue::div(AdValue::scale(scratch.ad_value(627), 0.666666666666667), scratch.ad_value(1369)));
        }

        if ((((scratch.values[1323] != 0.0) && (scratch.values[1340] != 0.0)) && (!(scratch.values[1761] != 0.0))) && (!(scratch.values[1766] != 0.0))) {
            scratch.store_ad(1371, &AdValue::square(scratch.ad_value(1370)));
        }

        if ((((scratch.values[1323] != 0.0) && (scratch.values[1340] != 0.0)) && (!(scratch.values[1761] != 0.0))) && (!(scratch.values[1766] != 0.0))) {
            scratch.store_ad(1372, &AdValue::sqrt(AdValue::div(AdValue::square(scratch.ad_value(1371)), AdValue::offset(AdValue::square(scratch.ad_value(1371)), 1.0))));
        }

        if ((((scratch.values[1323] != 0.0) && (scratch.values[1340] != 0.0)) && (!(scratch.values[1761] != 0.0))) && (!(scratch.values[1766] != 0.0))) {
            scratch.store_ad(1373, &AdValue::sqrt(AdValue::abs(scratch.ad_value(1372))));
        }

        if ((((scratch.values[1323] != 0.0) && (scratch.values[1340] != 0.0)) && (!(scratch.values[1761] != 0.0))) && (!(scratch.values[1766] != 0.0))) {
            scratch.store_ad(1374, &AdValue::mul(scratch.ad_value(1372), scratch.ad_value(1373)));
        }

        scratch.values[1767] = if (((-scratch.values[542]) * scratch.values[603]) == (-1.0)) { 1.0 } else { 0.0 };

        if (((((scratch.values[1323] != 0.0) && (scratch.values[1340] != 0.0)) && (!(scratch.values[1761] != 0.0))) && (!(scratch.values[1766] != 0.0))) && (scratch.values[1767] != 0.0)) {
            scratch.store_ad(1375, &AdValue::div_from_scalar(1.0, AdValue::offset(AdValue::mul(scratch.ad_value(1369), scratch.ad_value(1374)), 1.0)));
        }

    }

    pub(super) fn stamp_transient_block_23(
        &mut self,
        ctx: &GeneratedEvalContext<'_>,
        stamper: &mut GeneratedStamper<'_>,
        scratch: &mut Scratch,
    ) {
        let _ = stamper;
        if (((((scratch.values[1323] != 0.0) && (scratch.values[1340] != 0.0)) && (!(scratch.values[1761] != 0.0))) && (!(scratch.values[1766] != 0.0))) && (!(scratch.values[1767] != 0.0))) {
            scratch.store_ad(1375, &AdValue::pow(AdValue::offset(AdValue::mul(scratch.ad_value(1369), scratch.ad_value(1374)), 1.0), AdValue::mul(AdValue::neg(scratch.ad_value(542)), scratch.ad_value(603))));
        }

        if ((((scratch.values[1323] != 0.0) && (scratch.values[1340] != 0.0)) && (!(scratch.values[1761] != 0.0))) && (!(scratch.values[1766] != 0.0))) {
            scratch.store_ad(1376, &AdValue::div(AdValue::mul(scratch.ad_value(1365), scratch.ad_value(1375)), AdValue::add(scratch.ad_value(1365), scratch.ad_value(1375))));
        }

        if ((((scratch.values[1323] != 0.0) && (scratch.values[1340] != 0.0)) && (!(scratch.values[1761] != 0.0))) && (!(scratch.values[1766] != 0.0))) {
            scratch.store_ad(1377, &AdValue::sqrt(AdValue::scale(AdValue::div(scratch.ad_value(1369), scratch.ad_value(1373)), 0.375)));
        }

        if ((((scratch.values[1323] != 0.0) && (scratch.values[1340] != 0.0)) && (!(scratch.values[1761] != 0.0))) && (!(scratch.values[1766] != 0.0))) {
            scratch.store_ad(1378, &AdValue::sub(AdValue::scale(AdValue::mul(scratch.ad_value(1370), scratch.ad_value(1373)), 2.0), scratch.ad_value(1372)));
        }

        if ((((scratch.values[1323] != 0.0) && (scratch.values[1340] != 0.0)) && (!(scratch.values[1761] != 0.0))) && (!(scratch.values[1766] != 0.0))) {
            scratch.store_ad(1379, &AdValue::add(AdValue::sub(AdValue::mul(AdValue::mul(scratch.ad_value(627), scratch.ad_value(1370)), scratch.ad_value(1373)), AdValue::mul(scratch.ad_value(627), scratch.ad_value(1372))), AdValue::scale(AdValue::mul(scratch.ad_value(1369), scratch.ad_value(1374)), 0.5)));
        }

        if ((((scratch.values[1323] != 0.0) && (scratch.values[1340] != 0.0)) && (!(scratch.values[1761] != 0.0))) && (!(scratch.values[1766] != 0.0))) {
            scratch.store_ad(1380, &AdValue::mul(AdValue::offset(scratch.ad_value(1378), (-1.0)), scratch.ad_value(1377)));
        }

        if ((((scratch.values[1323] != 0.0) && (scratch.values[1340] != 0.0)) && (!(scratch.values[1761] != 0.0))) && (!(scratch.values[1766] != 0.0))) {
            scratch.store_ad(1341, &AdValue::square(scratch.ad_value(1380)));
        }

        scratch.values[1768] = if (scratch.values[1380] > 0.0) { 1.0 } else { 0.0 };

        if (((((scratch.values[1323] != 0.0) && (scratch.values[1340] != 0.0)) && (!(scratch.values[1761] != 0.0))) && (!(scratch.values[1766] != 0.0))) && (scratch.values[1768] != 0.0)) {
            scratch.store_ad(1342, &AdValue::div_from_scalar(1.0, AdValue::offset(AdValue::scale(scratch.ad_value(1380), scratch.values[421]), 1.0)));
        }

        if (((((scratch.values[1323] != 0.0) && (scratch.values[1340] != 0.0)) && (!(scratch.values[1761] != 0.0))) && (!(scratch.values[1766] != 0.0))) && (!(scratch.values[1768] != 0.0))) {
            scratch.store_ad(1342, &AdValue::div_from_scalar(1.0, AdValue::sub_from_scalar(1.0, AdValue::scale(scratch.ad_value(1380), scratch.values[421]))));
        }

        scratch.values[1769] = if (((-scratch.values[1341]) + scratch.values[1379]) > (-230.25850929940458)) { 1.0 } else { 0.0 };

        if (((((scratch.values[1323] != 0.0) && (scratch.values[1340] != 0.0)) && (!(scratch.values[1761] != 0.0))) && (!(scratch.values[1766] != 0.0))) && (scratch.values[1769] != 0.0)) {
            scratch.store_ad(1359, &AdValue::exp(AdValue::sub(scratch.ad_value(1379), scratch.ad_value(1341))));
        }

        if (((((scratch.values[1323] != 0.0) && (scratch.values[1340] != 0.0)) && (!(scratch.values[1761] != 0.0))) && (!(scratch.values[1766] != 0.0))) && (!(scratch.values[1769] != 0.0))) {
            let assign30290_ad_e36794: AdValue = AdValue::div_from_scalar(1e-100, AdValue::offset(AdValue::mul(AdValue::sub_from_scalar((-230.25850929940458), AdValue::sub(scratch.ad_value(1379), scratch.ad_value(1341))), AdValue::offset(AdValue::scale(AdValue::mul(AdValue::sub_from_scalar((-230.25850929940458), AdValue::sub(scratch.ad_value(1379), scratch.ad_value(1341))), AdValue::offset(AdValue::scale(AdValue::sub_from_scalar((-230.25850929940458), AdValue::sub(scratch.ad_value(1379), scratch.ad_value(1341))), 0.3333333333333333), 1.0)), 0.5), 1.0)), 1.0));
            scratch.store_ad(1359, &assign30290_ad_e36794);
        }

        if ((((scratch.values[1323] != 0.0) && (scratch.values[1340] != 0.0)) && (!(scratch.values[1761] != 0.0))) && (!(scratch.values[1766] != 0.0))) {
            scratch.store_ad(1343, &AdValue::mul(AdValue::add(AdValue::add(AdValue::scale(scratch.ad_value(1342), 0.29214664), AdValue::scale(AdValue::square(scratch.ad_value(1342)), scratch.values[422])), AdValue::scale(AdValue::mul(AdValue::square(scratch.ad_value(1342)), scratch.ad_value(1342)), scratch.values[423])), scratch.ad_value(1359)));
        }

        scratch.values[1770] = if (scratch.values[1380] > 0.0) { 1.0 } else { 0.0 };

        if (((((scratch.values[1323] != 0.0) && (scratch.values[1340] != 0.0)) && (!(scratch.values[1761] != 0.0))) && (!(scratch.values[1766] != 0.0))) && (scratch.values[1770] != 0.0)) {
            scratch.values[1381] = scratch.values[1343];
            scratch.node_derivatives[1381] = scratch.node_derivatives[1343];
            scratch.branch_derivatives[1381] = scratch.branch_derivatives[1343];
        }

        scratch.values[1771] = if (scratch.values[1379] > (-230.25850929940458)) { 1.0 } else { 0.0 };

        if ((((((scratch.values[1323] != 0.0) && (scratch.values[1340] != 0.0)) && (!(scratch.values[1761] != 0.0))) && (!(scratch.values[1766] != 0.0))) && (!(scratch.values[1770] != 0.0))) && (scratch.values[1771] != 0.0)) {
            scratch.store_ad(1359, &AdValue::exp(scratch.ad_value(1379)));
        }

        if ((((((scratch.values[1323] != 0.0) && (scratch.values[1340] != 0.0)) && (!(scratch.values[1761] != 0.0))) && (!(scratch.values[1766] != 0.0))) && (!(scratch.values[1770] != 0.0))) && (!(scratch.values[1771] != 0.0))) {
            scratch.store_ad(1359, &AdValue::div_from_scalar(1e-100, AdValue::offset(AdValue::mul(AdValue::sub_from_scalar((-230.25850929940458), scratch.ad_value(1379)), AdValue::offset(AdValue::scale(AdValue::mul(AdValue::sub_from_scalar((-230.25850929940458), scratch.ad_value(1379)), AdValue::offset(AdValue::scale(AdValue::sub_from_scalar((-230.25850929940458), scratch.ad_value(1379)), 0.3333333333333333), 1.0)), 0.5), 1.0)), 1.0)));
        }

        if (((((scratch.values[1323] != 0.0) && (scratch.values[1340] != 0.0)) && (!(scratch.values[1761] != 0.0))) && (!(scratch.values[1766] != 0.0))) && (!(scratch.values[1770] != 0.0))) {
            scratch.store_ad(1381, &AdValue::sub(AdValue::scale(scratch.ad_value(1359), 2.0), scratch.ad_value(1343)));
        }

        if ((((scratch.values[1323] != 0.0) && (scratch.values[1340] != 0.0)) && (!(scratch.values[1761] != 0.0))) && (!(scratch.values[1766] != 0.0))) {
            scratch.store_ad(1382, &AdValue::scale(AdValue::div(AdValue::mul(scratch.ad_value(627), scratch.ad_value(1381)), scratch.ad_value(1377)), (1.772453850905516 * 0.5)));
        }

        if ((((scratch.values[1323] != 0.0) && (scratch.values[1340] != 0.0)) && (!(scratch.values[1761] != 0.0))) && (!(scratch.values[1766] != 0.0))) {
            scratch.store_ad(1368, &AdValue::mul(scratch.ad_value(556), AdValue::mul(AdValue::mul(scratch.ad_value(1367), scratch.ad_value(1382)), scratch.ad_value(1376))));
        }

        scratch.values[1772] = if (scratch.values[562] == 0.0) { 1.0 } else { 0.0 };

        if ((((scratch.values[1323] != 0.0) && (scratch.values[1340] != 0.0)) && (!(scratch.values[1761] != 0.0))) && (scratch.values[1772] != 0.0)) {
            scratch.values[1383] = 0.0;
            scratch.node_derivatives[1383] = [0.0; Instance::NODE_COUNT];
            scratch.branch_derivatives[1383] = [0.0; Instance::BRANCH_COUNT];
        }

        scratch.values[1773] = if (scratch.values[542] == 0.5) { 1.0 } else { 0.0 };

        if (((((scratch.values[1323] != 0.0) && (scratch.values[1340] != 0.0)) && (!(scratch.values[1761] != 0.0))) && (!(scratch.values[1772] != 0.0))) && (scratch.values[1773] != 0.0)) {
            scratch.store_ad(1359, &AdValue::sqrt(AdValue::mul(AdValue::sub(scratch.ad_value(539), scratch.ad_value(1357)), scratch.ad_value(621))));
        }

        if (((((scratch.values[1323] != 0.0) && (scratch.values[1340] != 0.0)) && (!(scratch.values[1761] != 0.0))) && (!(scratch.values[1772] != 0.0))) && (!(scratch.values[1773] != 0.0))) {
            scratch.store_ad(1359, &AdValue::pow(AdValue::mul(AdValue::sub(scratch.ad_value(539), scratch.ad_value(1357)), scratch.ad_value(621)), scratch.ad_value(542)));
        }

        if ((((scratch.values[1323] != 0.0) && (scratch.values[1340] != 0.0)) && (!(scratch.values[1761] != 0.0))) && (!(scratch.values[1772] != 0.0))) {
            scratch.store_ad(1384, &AdValue::mul(scratch.ad_value(603), AdValue::div(AdValue::mul(AdValue::sub(scratch.ad_value(539), scratch.ad_value(1357)), scratch.ad_value(618)), scratch.ad_value(1359))));
        }

        scratch.values[1774] = if (((((-scratch.values[633]) / scratch.values[1384])) as f64).abs() < 230.25850929940458) { 1.0 } else { 0.0 };

        if (((((scratch.values[1323] != 0.0) && (scratch.values[1340] != 0.0)) && (!(scratch.values[1761] != 0.0))) && (!(scratch.values[1772] != 0.0))) && (scratch.values[1774] != 0.0)) {
            scratch.store_ad(1359, &AdValue::exp(AdValue::div(AdValue::neg(scratch.ad_value(633)), scratch.ad_value(1384))));
        }

        scratch.values[1775] = if (((-scratch.values[633]) / scratch.values[1384]) < 0.0) { 1.0 } else { 0.0 };

        if ((((((scratch.values[1323] != 0.0) && (scratch.values[1340] != 0.0)) && (!(scratch.values[1761] != 0.0))) && (!(scratch.values[1772] != 0.0))) && (!(scratch.values[1774] != 0.0))) && (scratch.values[1775] != 0.0)) {
            let assign30480_ad_e37121: AdValue = AdValue::offset(AdValue::mul(AdValue::sub_from_scalar((-230.25850929940458), AdValue::div(AdValue::neg(scratch.ad_value(633)), scratch.ad_value(1384))), AdValue::offset(AdValue::scale(AdValue::mul(AdValue::sub_from_scalar((-230.25850929940458), AdValue::div(AdValue::neg(scratch.ad_value(633)), scratch.ad_value(1384))), AdValue::offset(AdValue::scale(AdValue::sub_from_scalar((-230.25850929940458), AdValue::div(AdValue::neg(scratch.ad_value(633)), scratch.ad_value(1384))), 0.3333333333333333), 1.0)), 0.5), 1.0)), 1.0);
            scratch.store_ad(1359, &AdValue::div_from_scalar(1e-100, assign30480_ad_e37121));
        }

        if ((((((scratch.values[1323] != 0.0) && (scratch.values[1340] != 0.0)) && (!(scratch.values[1761] != 0.0))) && (!(scratch.values[1772] != 0.0))) && (!(scratch.values[1774] != 0.0))) && (!(scratch.values[1775] != 0.0))) {
            let assign30490_ad_e37171: AdValue = AdValue::scale(AdValue::offset(AdValue::mul(AdValue::offset(AdValue::div(AdValue::neg(scratch.ad_value(633)), scratch.ad_value(1384)), (-230.25850929940458)), AdValue::offset(AdValue::scale(AdValue::mul(AdValue::offset(AdValue::div(AdValue::neg(scratch.ad_value(633)), scratch.ad_value(1384)), (-230.25850929940458)), AdValue::offset(AdValue::scale(AdValue::offset(AdValue::div(AdValue::neg(scratch.ad_value(633)), scratch.ad_value(1384)), (-230.25850929940458)), 0.3333333333333333), 1.0)), 0.5), 1.0)), 1.0), 1e100);
            scratch.store_ad(1359, &assign30490_ad_e37171);
        }

        if ((((scratch.values[1323] != 0.0) && (scratch.values[1340] != 0.0)) && (!(scratch.values[1761] != 0.0))) && (!(scratch.values[1772] != 0.0))) {
            scratch.store_ad(1383, &AdValue::mul(scratch.ad_value(562), AdValue::mul(AdValue::mul(AdValue::mul(scratch.ad_value(517), scratch.ad_value(1384)), scratch.ad_value(1384)), scratch.ad_value(1359))));
        }

        scratch.values[1776] = if (scratch.values[571] > 1000.0) { 1.0 } else { 0.0 };

        if ((((scratch.values[1323] != 0.0) && (scratch.values[1340] != 0.0)) && (!(scratch.values[1761] != 0.0))) && (scratch.values[1776] != 0.0)) {
            scratch.values[1385] = 1.0;
            scratch.node_derivatives[1385] = [0.0; Instance::NODE_COUNT];
            scratch.branch_derivatives[1385] = [0.0; Instance::BRANCH_COUNT];
        }

        scratch.values[1777] = if (scratch.values[1358] > ((-scratch.values[493]) * scratch.values[571])) { 1.0 } else { 0.0 };

        scratch.values[1778] = if (scratch.values[574] == 4.0) { 1.0 } else { 0.0 };

        if ((((((scratch.values[1323] != 0.0) && (scratch.values[1340] != 0.0)) && (!(scratch.values[1761] != 0.0))) && (!(scratch.values[1776] != 0.0))) && (scratch.values[1777] != 0.0)) && (scratch.values[1778] != 0.0)) {
            scratch.store_ad(1359, &AdValue::mul(AdValue::mul(AdValue::mul(AdValue::mul(scratch.ad_value(1358), scratch.ad_value(639)), AdValue::mul(scratch.ad_value(1358), scratch.ad_value(639))), AdValue::mul(scratch.ad_value(1358), scratch.ad_value(639))), AdValue::mul(scratch.ad_value(1358), scratch.ad_value(639))));
        }

        if ((((((scratch.values[1323] != 0.0) && (scratch.values[1340] != 0.0)) && (!(scratch.values[1761] != 0.0))) && (!(scratch.values[1776] != 0.0))) && (scratch.values[1777] != 0.0)) && (!(scratch.values[1778] != 0.0))) {
            scratch.store_ad(1359, &AdValue::pow(AdValue::abs(AdValue::mul(scratch.ad_value(1358), scratch.ad_value(639))), scratch.ad_value(574)));
        }

        if (((((scratch.values[1323] != 0.0) && (scratch.values[1340] != 0.0)) && (!(scratch.values[1761] != 0.0))) && (!(scratch.values[1776] != 0.0))) && (scratch.values[1777] != 0.0)) {
            scratch.store_ad(1385, &AdValue::div_from_scalar(1.0, AdValue::sub_from_scalar(1.0, scratch.ad_value(1359))));
        }

        if (((((scratch.values[1323] != 0.0) && (scratch.values[1340] != 0.0)) && (!(scratch.values[1761] != 0.0))) && (!(scratch.values[1776] != 0.0))) && (!(scratch.values[1777] != 0.0))) {
            scratch.store_ad(1385, &AdValue::add(scratch.ad_value(636), AdValue::mul(AdValue::add(scratch.ad_value(1358), AdValue::scale(scratch.ad_value(571), scratch.values[493])), scratch.ad_value(642))));
        }

        if (((scratch.values[1323] != 0.0) && (scratch.values[1340] != 0.0)) && (!(scratch.values[1761] != 0.0))) {
            scratch.store_ad(1386, &AdValue::mul(AdValue::add(AdValue::add(AdValue::add(scratch.ad_value(1360), scratch.ad_value(1361)), scratch.ad_value(1368)), scratch.ad_value(1383)), scratch.ad_value(1385)));
        }

        scratch.values[1779] = if (scratch.values[718] == 0.0) { 1.0 } else { 0.0 };

        if (((scratch.values[1323] != 0.0) && (scratch.values[1340] != 0.0)) && (scratch.values[1779] != 0.0)) {
            scratch.values[1387] = 0.0;
            scratch.node_derivatives[1387] = [0.0; Instance::NODE_COUNT];
            scratch.branch_derivatives[1387] = [0.0; Instance::BRANCH_COUNT];
        }

        scratch.values[1780] = if (scratch.values[601] == 0.5) { 1.0 } else { 0.0 };

        if ((((scratch.values[1323] != 0.0) && (scratch.values[1340] != 0.0)) && (!(scratch.values[1779] != 0.0))) && (scratch.values[1780] != 0.0)) {
            scratch.store_ad(1359, &AdValue::sqrt(AdValue::sub_from_scalar(1.0, AdValue::mul(scratch.ad_value(1351), scratch.ad_value(598)))));
        }

        if ((((scratch.values[1323] != 0.0) && (scratch.values[1340] != 0.0)) && (!(scratch.values[1779] != 0.0))) && (!(scratch.values[1780] != 0.0))) {
            scratch.store_ad(1359, &AdValue::pow(AdValue::sub_from_scalar(1.0, AdValue::mul(scratch.ad_value(1351), scratch.ad_value(598))), scratch.ad_value(601)));
        }

        if (((scratch.values[1323] != 0.0) && (scratch.values[1340] != 0.0)) && (!(scratch.values[1779] != 0.0))) {
            scratch.store_ad(1360, &AdValue::mul(scratch.ad_value(589), scratch.ad_value(1350)));
        }

        scratch.values[1781] = if ((scratch.values[554] == 0.0) && (scratch.values[557] == 0.0)) { 1.0 } else { 0.0 };

        if ((((scratch.values[1323] != 0.0) && (scratch.values[1340] != 0.0)) && (!(scratch.values[1779] != 0.0))) && (scratch.values[1781] != 0.0)) {
            scratch.values[1361] = 0.0;
            scratch.node_derivatives[1361] = [0.0; Instance::NODE_COUNT];
            scratch.branch_derivatives[1361] = [0.0; Instance::BRANCH_COUNT];
        }

        if ((((scratch.values[1323] != 0.0) && (scratch.values[1340] != 0.0)) && (!(scratch.values[1779] != 0.0))) && (!(scratch.values[1781] != 0.0))) {
            scratch.store_ad(1362, &AdValue::sub(scratch.ad_value(595), scratch.ad_value(1356)));
        }

        if ((((scratch.values[1323] != 0.0) && (scratch.values[1340] != 0.0)) && (!(scratch.values[1779] != 0.0))) && (!(scratch.values[1781] != 0.0))) {
            scratch.store_ad(1363, &AdValue::sub_from_scalar(1.0, AdValue::sqrt(AdValue::sub_from_scalar(1.0, AdValue::div(scratch.ad_value(1354), scratch.ad_value(1362))))));
        }

        scratch.values[1782] = if (scratch.values[543] == 0.5) { 1.0 } else { 0.0 };

        if (((((scratch.values[1323] != 0.0) && (scratch.values[1340] != 0.0)) && (!(scratch.values[1779] != 0.0))) && (!(scratch.values[1781] != 0.0))) && (scratch.values[1782] != 0.0)) {
            scratch.values[1364] = 0.0;
            scratch.node_derivatives[1364] = [0.0; Instance::NODE_COUNT];
            scratch.branch_derivatives[1364] = [0.0; Instance::BRANCH_COUNT];
        }

        if (((((scratch.values[1323] != 0.0) && (scratch.values[1340] != 0.0)) && (!(scratch.values[1779] != 0.0))) && (!(scratch.values[1781] != 0.0))) && (!(scratch.values[1782] != 0.0))) {
            scratch.store_ad(1364, &AdValue::mul(AdValue::add(AdValue::div(AdValue::mul(AdValue::square(scratch.ad_value(1363)), AdValue::ln(scratch.ad_value(1363))), AdValue::sub_from_scalar(1.0, scratch.ad_value(1363))), scratch.ad_value(1363)), AdValue::sub_from_scalar(1.0, AdValue::scale(scratch.ad_value(543), 2.0))));
        }

        if ((((scratch.values[1323] != 0.0) && (scratch.values[1340] != 0.0)) && (!(scratch.values[1779] != 0.0))) && (!(scratch.values[1781] != 0.0))) {
            scratch.store_ad(1365, &AdValue::add(scratch.ad_value(1363), scratch.ad_value(1364)));
        }

        scratch.values[1783] = if (scratch.values[543] == 0.5) { 1.0 } else { 0.0 };

        if (((((scratch.values[1323] != 0.0) && (scratch.values[1340] != 0.0)) && (!(scratch.values[1779] != 0.0))) && (!(scratch.values[1781] != 0.0))) && (scratch.values[1783] != 0.0)) {
            scratch.store_ad(1359, &AdValue::sqrt(AdValue::mul(scratch.ad_value(1362), scratch.ad_value(622))));
        }

        if (((((scratch.values[1323] != 0.0) && (scratch.values[1340] != 0.0)) && (!(scratch.values[1779] != 0.0))) && (!(scratch.values[1781] != 0.0))) && (!(scratch.values[1783] != 0.0))) {
            scratch.store_ad(1359, &AdValue::pow(AdValue::mul(scratch.ad_value(1362), scratch.ad_value(622)), scratch.ad_value(543)));
        }

        if ((((scratch.values[1323] != 0.0) && (scratch.values[1340] != 0.0)) && (!(scratch.values[1779] != 0.0))) && (!(scratch.values[1781] != 0.0))) {
            scratch.store_ad(1366, &AdValue::mul(scratch.ad_value(616), scratch.ad_value(1359)));
        }

        if ((((scratch.values[1323] != 0.0) && (scratch.values[1340] != 0.0)) && (!(scratch.values[1779] != 0.0))) && (!(scratch.values[1781] != 0.0))) {
            scratch.store_ad(1367, &AdValue::mul(scratch.ad_value(586), AdValue::mul(AdValue::offset(scratch.ad_value(1353), (-1.0)), scratch.ad_value(1366))));
        }

        if ((((scratch.values[1323] != 0.0) && (scratch.values[1340] != 0.0)) && (!(scratch.values[1779] != 0.0))) && (!(scratch.values[1781] != 0.0))) {
            scratch.store_ad(1361, &AdValue::mul(scratch.ad_value(554), AdValue::mul(scratch.ad_value(1367), scratch.ad_value(1365))));
        }

        scratch.values[1784] = if (scratch.values[557] == 0.0) { 1.0 } else { 0.0 };

        if ((((scratch.values[1323] != 0.0) && (scratch.values[1340] != 0.0)) && (!(scratch.values[1779] != 0.0))) && (scratch.values[1784] != 0.0)) {
            scratch.values[1368] = 0.0;
            scratch.node_derivatives[1368] = [0.0; Instance::NODE_COUNT];
            scratch.branch_derivatives[1368] = [0.0; Instance::BRANCH_COUNT];
        }

        if ((((scratch.values[1323] != 0.0) && (scratch.values[1340] != 0.0)) && (!(scratch.values[1779] != 0.0))) && (!(scratch.values[1784] != 0.0))) {
            scratch.store_ad(1369, &AdValue::mul(scratch.ad_value(631), AdValue::div(AdValue::mul(scratch.ad_value(1366), scratch.ad_value(601)), scratch.ad_value(1362))));
        }

        if ((((scratch.values[1323] != 0.0) && (scratch.values[1340] != 0.0)) && (!(scratch.values[1779] != 0.0))) && (!(scratch.values[1784] != 0.0))) {
            scratch.store_ad(1370, &AdValue::div(AdValue::scale(scratch.ad_value(628), 0.666666666666667), scratch.ad_value(1369)));
        }

        if ((((scratch.values[1323] != 0.0) && (scratch.values[1340] != 0.0)) && (!(scratch.values[1779] != 0.0))) && (!(scratch.values[1784] != 0.0))) {
            scratch.store_ad(1371, &AdValue::square(scratch.ad_value(1370)));
        }

        if ((((scratch.values[1323] != 0.0) && (scratch.values[1340] != 0.0)) && (!(scratch.values[1779] != 0.0))) && (!(scratch.values[1784] != 0.0))) {
            scratch.store_ad(1372, &AdValue::sqrt(AdValue::div(AdValue::square(scratch.ad_value(1371)), AdValue::offset(AdValue::square(scratch.ad_value(1371)), 1.0))));
        }

        if ((((scratch.values[1323] != 0.0) && (scratch.values[1340] != 0.0)) && (!(scratch.values[1779] != 0.0))) && (!(scratch.values[1784] != 0.0))) {
            scratch.store_ad(1373, &AdValue::sqrt(AdValue::abs(scratch.ad_value(1372))));
        }

        if ((((scratch.values[1323] != 0.0) && (scratch.values[1340] != 0.0)) && (!(scratch.values[1779] != 0.0))) && (!(scratch.values[1784] != 0.0))) {
            scratch.store_ad(1374, &AdValue::mul(scratch.ad_value(1372), scratch.ad_value(1373)));
        }

        scratch.values[1785] = if (((-scratch.values[543]) * scratch.values[604]) == (-1.0)) { 1.0 } else { 0.0 };

        if (((((scratch.values[1323] != 0.0) && (scratch.values[1340] != 0.0)) && (!(scratch.values[1779] != 0.0))) && (!(scratch.values[1784] != 0.0))) && (scratch.values[1785] != 0.0)) {
            scratch.store_ad(1375, &AdValue::div_from_scalar(1.0, AdValue::offset(AdValue::mul(scratch.ad_value(1369), scratch.ad_value(1374)), 1.0)));
        }

        if (((((scratch.values[1323] != 0.0) && (scratch.values[1340] != 0.0)) && (!(scratch.values[1779] != 0.0))) && (!(scratch.values[1784] != 0.0))) && (!(scratch.values[1785] != 0.0))) {
            scratch.store_ad(1375, &AdValue::pow(AdValue::offset(AdValue::mul(scratch.ad_value(1369), scratch.ad_value(1374)), 1.0), AdValue::mul(AdValue::neg(scratch.ad_value(543)), scratch.ad_value(604))));
        }

        if ((((scratch.values[1323] != 0.0) && (scratch.values[1340] != 0.0)) && (!(scratch.values[1779] != 0.0))) && (!(scratch.values[1784] != 0.0))) {
            scratch.store_ad(1376, &AdValue::div(AdValue::mul(scratch.ad_value(1365), scratch.ad_value(1375)), AdValue::add(scratch.ad_value(1365), scratch.ad_value(1375))));
        }

        if ((((scratch.values[1323] != 0.0) && (scratch.values[1340] != 0.0)) && (!(scratch.values[1779] != 0.0))) && (!(scratch.values[1784] != 0.0))) {
            scratch.store_ad(1377, &AdValue::sqrt(AdValue::scale(AdValue::div(scratch.ad_value(1369), scratch.ad_value(1373)), 0.375)));
        }

        if ((((scratch.values[1323] != 0.0) && (scratch.values[1340] != 0.0)) && (!(scratch.values[1779] != 0.0))) && (!(scratch.values[1784] != 0.0))) {
            scratch.store_ad(1378, &AdValue::sub(AdValue::scale(AdValue::mul(scratch.ad_value(1370), scratch.ad_value(1373)), 2.0), scratch.ad_value(1372)));
        }

        if ((((scratch.values[1323] != 0.0) && (scratch.values[1340] != 0.0)) && (!(scratch.values[1779] != 0.0))) && (!(scratch.values[1784] != 0.0))) {
            scratch.store_ad(1379, &AdValue::add(AdValue::sub(AdValue::mul(AdValue::mul(scratch.ad_value(628), scratch.ad_value(1370)), scratch.ad_value(1373)), AdValue::mul(scratch.ad_value(628), scratch.ad_value(1372))), AdValue::scale(AdValue::mul(scratch.ad_value(1369), scratch.ad_value(1374)), 0.5)));
        }

        if ((((scratch.values[1323] != 0.0) && (scratch.values[1340] != 0.0)) && (!(scratch.values[1779] != 0.0))) && (!(scratch.values[1784] != 0.0))) {
            scratch.store_ad(1380, &AdValue::mul(AdValue::offset(scratch.ad_value(1378), (-1.0)), scratch.ad_value(1377)));
        }

        if ((((scratch.values[1323] != 0.0) && (scratch.values[1340] != 0.0)) && (!(scratch.values[1779] != 0.0))) && (!(scratch.values[1784] != 0.0))) {
            scratch.store_ad(1341, &AdValue::square(scratch.ad_value(1380)));
        }

        scratch.values[1786] = if (scratch.values[1380] > 0.0) { 1.0 } else { 0.0 };

        if (((((scratch.values[1323] != 0.0) && (scratch.values[1340] != 0.0)) && (!(scratch.values[1779] != 0.0))) && (!(scratch.values[1784] != 0.0))) && (scratch.values[1786] != 0.0)) {
            scratch.store_ad(1342, &AdValue::div_from_scalar(1.0, AdValue::offset(AdValue::scale(scratch.ad_value(1380), scratch.values[421]), 1.0)));
        }

        if (((((scratch.values[1323] != 0.0) && (scratch.values[1340] != 0.0)) && (!(scratch.values[1779] != 0.0))) && (!(scratch.values[1784] != 0.0))) && (!(scratch.values[1786] != 0.0))) {
            scratch.store_ad(1342, &AdValue::div_from_scalar(1.0, AdValue::sub_from_scalar(1.0, AdValue::scale(scratch.ad_value(1380), scratch.values[421]))));
        }

        scratch.values[1787] = if (((-scratch.values[1341]) + scratch.values[1379]) > (-230.25850929940458)) { 1.0 } else { 0.0 };

        if (((((scratch.values[1323] != 0.0) && (scratch.values[1340] != 0.0)) && (!(scratch.values[1779] != 0.0))) && (!(scratch.values[1784] != 0.0))) && (scratch.values[1787] != 0.0)) {
            scratch.store_ad(1359, &AdValue::exp(AdValue::sub(scratch.ad_value(1379), scratch.ad_value(1341))));
        }

        if (((((scratch.values[1323] != 0.0) && (scratch.values[1340] != 0.0)) && (!(scratch.values[1779] != 0.0))) && (!(scratch.values[1784] != 0.0))) && (!(scratch.values[1787] != 0.0))) {
            let assign31040_ad_e38000: AdValue = AdValue::div_from_scalar(1e-100, AdValue::offset(AdValue::mul(AdValue::sub_from_scalar((-230.25850929940458), AdValue::sub(scratch.ad_value(1379), scratch.ad_value(1341))), AdValue::offset(AdValue::scale(AdValue::mul(AdValue::sub_from_scalar((-230.25850929940458), AdValue::sub(scratch.ad_value(1379), scratch.ad_value(1341))), AdValue::offset(AdValue::scale(AdValue::sub_from_scalar((-230.25850929940458), AdValue::sub(scratch.ad_value(1379), scratch.ad_value(1341))), 0.3333333333333333), 1.0)), 0.5), 1.0)), 1.0));
            scratch.store_ad(1359, &assign31040_ad_e38000);
        }

        if ((((scratch.values[1323] != 0.0) && (scratch.values[1340] != 0.0)) && (!(scratch.values[1779] != 0.0))) && (!(scratch.values[1784] != 0.0))) {
            scratch.store_ad(1343, &AdValue::mul(AdValue::add(AdValue::add(AdValue::scale(scratch.ad_value(1342), 0.29214664), AdValue::scale(AdValue::square(scratch.ad_value(1342)), scratch.values[422])), AdValue::scale(AdValue::mul(AdValue::square(scratch.ad_value(1342)), scratch.ad_value(1342)), scratch.values[423])), scratch.ad_value(1359)));
        }

        scratch.values[1788] = if (scratch.values[1380] > 0.0) { 1.0 } else { 0.0 };

        if (((((scratch.values[1323] != 0.0) && (scratch.values[1340] != 0.0)) && (!(scratch.values[1779] != 0.0))) && (!(scratch.values[1784] != 0.0))) && (scratch.values[1788] != 0.0)) {
            scratch.values[1381] = scratch.values[1343];
            scratch.node_derivatives[1381] = scratch.node_derivatives[1343];
            scratch.branch_derivatives[1381] = scratch.branch_derivatives[1343];
        }

        scratch.values[1789] = if (scratch.values[1379] > (-230.25850929940458)) { 1.0 } else { 0.0 };

        if ((((((scratch.values[1323] != 0.0) && (scratch.values[1340] != 0.0)) && (!(scratch.values[1779] != 0.0))) && (!(scratch.values[1784] != 0.0))) && (!(scratch.values[1788] != 0.0))) && (scratch.values[1789] != 0.0)) {
            scratch.store_ad(1359, &AdValue::exp(scratch.ad_value(1379)));
        }

        if ((((((scratch.values[1323] != 0.0) && (scratch.values[1340] != 0.0)) && (!(scratch.values[1779] != 0.0))) && (!(scratch.values[1784] != 0.0))) && (!(scratch.values[1788] != 0.0))) && (!(scratch.values[1789] != 0.0))) {
            scratch.store_ad(1359, &AdValue::div_from_scalar(1e-100, AdValue::offset(AdValue::mul(AdValue::sub_from_scalar((-230.25850929940458), scratch.ad_value(1379)), AdValue::offset(AdValue::scale(AdValue::mul(AdValue::sub_from_scalar((-230.25850929940458), scratch.ad_value(1379)), AdValue::offset(AdValue::scale(AdValue::sub_from_scalar((-230.25850929940458), scratch.ad_value(1379)), 0.3333333333333333), 1.0)), 0.5), 1.0)), 1.0)));
        }

        if (((((scratch.values[1323] != 0.0) && (scratch.values[1340] != 0.0)) && (!(scratch.values[1779] != 0.0))) && (!(scratch.values[1784] != 0.0))) && (!(scratch.values[1788] != 0.0))) {
            scratch.store_ad(1381, &AdValue::sub(AdValue::scale(scratch.ad_value(1359), 2.0), scratch.ad_value(1343)));
        }

        if ((((scratch.values[1323] != 0.0) && (scratch.values[1340] != 0.0)) && (!(scratch.values[1779] != 0.0))) && (!(scratch.values[1784] != 0.0))) {
            scratch.store_ad(1382, &AdValue::scale(AdValue::div(AdValue::mul(scratch.ad_value(628), scratch.ad_value(1381)), scratch.ad_value(1377)), (1.772453850905516 * 0.5)));
        }

        if ((((scratch.values[1323] != 0.0) && (scratch.values[1340] != 0.0)) && (!(scratch.values[1779] != 0.0))) && (!(scratch.values[1784] != 0.0))) {
            scratch.store_ad(1368, &AdValue::mul(scratch.ad_value(557), AdValue::mul(AdValue::mul(scratch.ad_value(1367), scratch.ad_value(1382)), scratch.ad_value(1376))));
        }

        scratch.values[1790] = if (scratch.values[563] == 0.0) { 1.0 } else { 0.0 };

        if ((((scratch.values[1323] != 0.0) && (scratch.values[1340] != 0.0)) && (!(scratch.values[1779] != 0.0))) && (scratch.values[1790] != 0.0)) {
            scratch.values[1383] = 0.0;
            scratch.node_derivatives[1383] = [0.0; Instance::NODE_COUNT];
            scratch.branch_derivatives[1383] = [0.0; Instance::BRANCH_COUNT];
        }

        scratch.values[1791] = if (scratch.values[543] == 0.5) { 1.0 } else { 0.0 };

        if (((((scratch.values[1323] != 0.0) && (scratch.values[1340] != 0.0)) && (!(scratch.values[1779] != 0.0))) && (!(scratch.values[1790] != 0.0))) && (scratch.values[1791] != 0.0)) {
            scratch.store_ad(1359, &AdValue::sqrt(AdValue::mul(AdValue::sub(scratch.ad_value(540), scratch.ad_value(1357)), scratch.ad_value(622))));
        }

        if (((((scratch.values[1323] != 0.0) && (scratch.values[1340] != 0.0)) && (!(scratch.values[1779] != 0.0))) && (!(scratch.values[1790] != 0.0))) && (!(scratch.values[1791] != 0.0))) {
            scratch.store_ad(1359, &AdValue::pow(AdValue::mul(AdValue::sub(scratch.ad_value(540), scratch.ad_value(1357)), scratch.ad_value(622)), scratch.ad_value(543)));
        }

        if ((((scratch.values[1323] != 0.0) && (scratch.values[1340] != 0.0)) && (!(scratch.values[1779] != 0.0))) && (!(scratch.values[1790] != 0.0))) {
            scratch.store_ad(1384, &AdValue::mul(scratch.ad_value(604), AdValue::div(AdValue::mul(AdValue::sub(scratch.ad_value(540), scratch.ad_value(1357)), scratch.ad_value(619)), scratch.ad_value(1359))));
        }

        scratch.values[1792] = if (((((-scratch.values[634]) / scratch.values[1384])) as f64).abs() < 230.25850929940458) { 1.0 } else { 0.0 };

        if (((((scratch.values[1323] != 0.0) && (scratch.values[1340] != 0.0)) && (!(scratch.values[1779] != 0.0))) && (!(scratch.values[1790] != 0.0))) && (scratch.values[1792] != 0.0)) {
            scratch.store_ad(1359, &AdValue::exp(AdValue::div(AdValue::neg(scratch.ad_value(634)), scratch.ad_value(1384))));
        }

        scratch.values[1793] = if (((-scratch.values[634]) / scratch.values[1384]) < 0.0) { 1.0 } else { 0.0 };

        if ((((((scratch.values[1323] != 0.0) && (scratch.values[1340] != 0.0)) && (!(scratch.values[1779] != 0.0))) && (!(scratch.values[1790] != 0.0))) && (!(scratch.values[1792] != 0.0))) && (scratch.values[1793] != 0.0)) {
            let assign31230_ad_e38327: AdValue = AdValue::offset(AdValue::mul(AdValue::sub_from_scalar((-230.25850929940458), AdValue::div(AdValue::neg(scratch.ad_value(634)), scratch.ad_value(1384))), AdValue::offset(AdValue::scale(AdValue::mul(AdValue::sub_from_scalar((-230.25850929940458), AdValue::div(AdValue::neg(scratch.ad_value(634)), scratch.ad_value(1384))), AdValue::offset(AdValue::scale(AdValue::sub_from_scalar((-230.25850929940458), AdValue::div(AdValue::neg(scratch.ad_value(634)), scratch.ad_value(1384))), 0.3333333333333333), 1.0)), 0.5), 1.0)), 1.0);
            scratch.store_ad(1359, &AdValue::div_from_scalar(1e-100, assign31230_ad_e38327));
        }

        if ((((((scratch.values[1323] != 0.0) && (scratch.values[1340] != 0.0)) && (!(scratch.values[1779] != 0.0))) && (!(scratch.values[1790] != 0.0))) && (!(scratch.values[1792] != 0.0))) && (!(scratch.values[1793] != 0.0))) {
            let assign31240_ad_e38377: AdValue = AdValue::scale(AdValue::offset(AdValue::mul(AdValue::offset(AdValue::div(AdValue::neg(scratch.ad_value(634)), scratch.ad_value(1384)), (-230.25850929940458)), AdValue::offset(AdValue::scale(AdValue::mul(AdValue::offset(AdValue::div(AdValue::neg(scratch.ad_value(634)), scratch.ad_value(1384)), (-230.25850929940458)), AdValue::offset(AdValue::scale(AdValue::offset(AdValue::div(AdValue::neg(scratch.ad_value(634)), scratch.ad_value(1384)), (-230.25850929940458)), 0.3333333333333333), 1.0)), 0.5), 1.0)), 1.0), 1e100);
            scratch.store_ad(1359, &assign31240_ad_e38377);
        }

        if ((((scratch.values[1323] != 0.0) && (scratch.values[1340] != 0.0)) && (!(scratch.values[1779] != 0.0))) && (!(scratch.values[1790] != 0.0))) {
            scratch.store_ad(1383, &AdValue::mul(scratch.ad_value(563), AdValue::mul(AdValue::mul(AdValue::mul(scratch.ad_value(517), scratch.ad_value(1384)), scratch.ad_value(1384)), scratch.ad_value(1359))));
        }

        scratch.values[1794] = if (scratch.values[572] > 1000.0) { 1.0 } else { 0.0 };

        if ((((scratch.values[1323] != 0.0) && (scratch.values[1340] != 0.0)) && (!(scratch.values[1779] != 0.0))) && (scratch.values[1794] != 0.0)) {
            scratch.values[1385] = 1.0;
            scratch.node_derivatives[1385] = [0.0; Instance::NODE_COUNT];
            scratch.branch_derivatives[1385] = [0.0; Instance::BRANCH_COUNT];
        }

        scratch.values[1795] = if (scratch.values[1358] > ((-scratch.values[493]) * scratch.values[572])) { 1.0 } else { 0.0 };

        scratch.values[1796] = if (scratch.values[575] == 4.0) { 1.0 } else { 0.0 };

        if ((((((scratch.values[1323] != 0.0) && (scratch.values[1340] != 0.0)) && (!(scratch.values[1779] != 0.0))) && (!(scratch.values[1794] != 0.0))) && (scratch.values[1795] != 0.0)) && (scratch.values[1796] != 0.0)) {
            scratch.store_ad(1359, &AdValue::mul(AdValue::mul(AdValue::mul(AdValue::mul(scratch.ad_value(1358), scratch.ad_value(640)), AdValue::mul(scratch.ad_value(1358), scratch.ad_value(640))), AdValue::mul(scratch.ad_value(1358), scratch.ad_value(640))), AdValue::mul(scratch.ad_value(1358), scratch.ad_value(640))));
        }

        if ((((((scratch.values[1323] != 0.0) && (scratch.values[1340] != 0.0)) && (!(scratch.values[1779] != 0.0))) && (!(scratch.values[1794] != 0.0))) && (scratch.values[1795] != 0.0)) && (!(scratch.values[1796] != 0.0))) {
            scratch.store_ad(1359, &AdValue::pow(AdValue::abs(AdValue::mul(scratch.ad_value(1358), scratch.ad_value(640))), scratch.ad_value(575)));
        }

        if (((((scratch.values[1323] != 0.0) && (scratch.values[1340] != 0.0)) && (!(scratch.values[1779] != 0.0))) && (!(scratch.values[1794] != 0.0))) && (scratch.values[1795] != 0.0)) {
            scratch.store_ad(1385, &AdValue::div_from_scalar(1.0, AdValue::sub_from_scalar(1.0, scratch.ad_value(1359))));
        }

        if (((((scratch.values[1323] != 0.0) && (scratch.values[1340] != 0.0)) && (!(scratch.values[1779] != 0.0))) && (!(scratch.values[1794] != 0.0))) && (!(scratch.values[1795] != 0.0))) {
            scratch.store_ad(1385, &AdValue::add(scratch.ad_value(637), AdValue::mul(AdValue::add(scratch.ad_value(1358), AdValue::scale(scratch.ad_value(572), scratch.values[493])), scratch.ad_value(643))));
        }

        if (((scratch.values[1323] != 0.0) && (scratch.values[1340] != 0.0)) && (!(scratch.values[1779] != 0.0))) {
            scratch.store_ad(1387, &AdValue::mul(AdValue::add(AdValue::add(AdValue::add(scratch.ad_value(1360), scratch.ad_value(1361)), scratch.ad_value(1368)), scratch.ad_value(1383)), scratch.ad_value(1385)));
        }

        scratch.values[1797] = if (scratch.values[719] == 0.0) { 1.0 } else { 0.0 };

        if (((scratch.values[1323] != 0.0) && (scratch.values[1340] != 0.0)) && (scratch.values[1797] != 0.0)) {
            scratch.values[1388] = 0.0;
            scratch.node_derivatives[1388] = [0.0; Instance::NODE_COUNT];
            scratch.branch_derivatives[1388] = [0.0; Instance::BRANCH_COUNT];
        }

        scratch.values[1798] = if (scratch.values[602] == 0.5) { 1.0 } else { 0.0 };

        if ((((scratch.values[1323] != 0.0) && (scratch.values[1340] != 0.0)) && (!(scratch.values[1797] != 0.0))) && (scratch.values[1798] != 0.0)) {
            scratch.store_ad(1359, &AdValue::sqrt(AdValue::sub_from_scalar(1.0, AdValue::mul(scratch.ad_value(1351), scratch.ad_value(599)))));
        }

        if ((((scratch.values[1323] != 0.0) && (scratch.values[1340] != 0.0)) && (!(scratch.values[1797] != 0.0))) && (!(scratch.values[1798] != 0.0))) {
            scratch.store_ad(1359, &AdValue::pow(AdValue::sub_from_scalar(1.0, AdValue::mul(scratch.ad_value(1351), scratch.ad_value(599))), scratch.ad_value(602)));
        }

        if (((scratch.values[1323] != 0.0) && (scratch.values[1340] != 0.0)) && (!(scratch.values[1797] != 0.0))) {
            scratch.store_ad(1360, &AdValue::mul(scratch.ad_value(590), scratch.ad_value(1350)));
        }

        scratch.values[1799] = if ((scratch.values[555] == 0.0) && (scratch.values[558] == 0.0)) { 1.0 } else { 0.0 };

        if ((((scratch.values[1323] != 0.0) && (scratch.values[1340] != 0.0)) && (!(scratch.values[1797] != 0.0))) && (scratch.values[1799] != 0.0)) {
            scratch.values[1361] = 0.0;
            scratch.node_derivatives[1361] = [0.0; Instance::NODE_COUNT];
            scratch.branch_derivatives[1361] = [0.0; Instance::BRANCH_COUNT];
        }

        if ((((scratch.values[1323] != 0.0) && (scratch.values[1340] != 0.0)) && (!(scratch.values[1797] != 0.0))) && (!(scratch.values[1799] != 0.0))) {
            scratch.store_ad(1362, &AdValue::sub(scratch.ad_value(596), scratch.ad_value(1356)));
        }

        if ((((scratch.values[1323] != 0.0) && (scratch.values[1340] != 0.0)) && (!(scratch.values[1797] != 0.0))) && (!(scratch.values[1799] != 0.0))) {
            scratch.store_ad(1363, &AdValue::sub_from_scalar(1.0, AdValue::sqrt(AdValue::sub_from_scalar(1.0, AdValue::div(scratch.ad_value(1354), scratch.ad_value(1362))))));
        }

        scratch.values[1800] = if (scratch.values[544] == 0.5) { 1.0 } else { 0.0 };

        if (((((scratch.values[1323] != 0.0) && (scratch.values[1340] != 0.0)) && (!(scratch.values[1797] != 0.0))) && (!(scratch.values[1799] != 0.0))) && (scratch.values[1800] != 0.0)) {
            scratch.values[1364] = 0.0;
            scratch.node_derivatives[1364] = [0.0; Instance::NODE_COUNT];
            scratch.branch_derivatives[1364] = [0.0; Instance::BRANCH_COUNT];
        }

        if (((((scratch.values[1323] != 0.0) && (scratch.values[1340] != 0.0)) && (!(scratch.values[1797] != 0.0))) && (!(scratch.values[1799] != 0.0))) && (!(scratch.values[1800] != 0.0))) {
            scratch.store_ad(1364, &AdValue::mul(AdValue::add(AdValue::div(AdValue::mul(AdValue::square(scratch.ad_value(1363)), AdValue::ln(scratch.ad_value(1363))), AdValue::sub_from_scalar(1.0, scratch.ad_value(1363))), scratch.ad_value(1363)), AdValue::sub_from_scalar(1.0, AdValue::scale(scratch.ad_value(544), 2.0))));
        }

        if ((((scratch.values[1323] != 0.0) && (scratch.values[1340] != 0.0)) && (!(scratch.values[1797] != 0.0))) && (!(scratch.values[1799] != 0.0))) {
            scratch.store_ad(1365, &AdValue::add(scratch.ad_value(1363), scratch.ad_value(1364)));
        }

        scratch.values[1801] = if (scratch.values[544] == 0.5) { 1.0 } else { 0.0 };

        if (((((scratch.values[1323] != 0.0) && (scratch.values[1340] != 0.0)) && (!(scratch.values[1797] != 0.0))) && (!(scratch.values[1799] != 0.0))) && (scratch.values[1801] != 0.0)) {
            scratch.store_ad(1359, &AdValue::sqrt(AdValue::mul(scratch.ad_value(1362), scratch.ad_value(623))));
        }

        if (((((scratch.values[1323] != 0.0) && (scratch.values[1340] != 0.0)) && (!(scratch.values[1797] != 0.0))) && (!(scratch.values[1799] != 0.0))) && (!(scratch.values[1801] != 0.0))) {
            scratch.store_ad(1359, &AdValue::pow(AdValue::mul(scratch.ad_value(1362), scratch.ad_value(623)), scratch.ad_value(544)));
        }

        if ((((scratch.values[1323] != 0.0) && (scratch.values[1340] != 0.0)) && (!(scratch.values[1797] != 0.0))) && (!(scratch.values[1799] != 0.0))) {
            scratch.store_ad(1366, &AdValue::mul(scratch.ad_value(617), scratch.ad_value(1359)));
        }

        if ((((scratch.values[1323] != 0.0) && (scratch.values[1340] != 0.0)) && (!(scratch.values[1797] != 0.0))) && (!(scratch.values[1799] != 0.0))) {
            scratch.store_ad(1367, &AdValue::mul(scratch.ad_value(587), AdValue::mul(AdValue::offset(scratch.ad_value(1353), (-1.0)), scratch.ad_value(1366))));
        }

        if ((((scratch.values[1323] != 0.0) && (scratch.values[1340] != 0.0)) && (!(scratch.values[1797] != 0.0))) && (!(scratch.values[1799] != 0.0))) {
            scratch.store_ad(1361, &AdValue::mul(scratch.ad_value(555), AdValue::mul(scratch.ad_value(1367), scratch.ad_value(1365))));
        }

        scratch.values[1802] = if (scratch.values[558] == 0.0) { 1.0 } else { 0.0 };

    }
}
