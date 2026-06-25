#![allow(dead_code, unused_assignments, unused_imports, unused_parens, unused_variables)]

use super::{AdValue, GeneratedEvalContext, GeneratedReactiveStamper, GeneratedStamper, Scratch, THERMAL_VOLTAGE_PER_K};
use super::super::state::Instance;

impl Instance {

    pub(super) fn stamp_transient_block_24(
        &mut self,
        ctx: &GeneratedEvalContext<'_>,
        stamper: &mut GeneratedStamper<'_>,
        scratch: &mut Scratch,
    ) {
        let _ = stamper;
        if ((((scratch.values[1323] != 0.0) && (scratch.values[1340] != 0.0)) && (!(scratch.values[1797] != 0.0))) && (scratch.values[1802] != 0.0)) {
            scratch.values[1368] = 0.0;
            scratch.node_derivatives[1368] = [0.0; Instance::NODE_COUNT];
            scratch.branch_derivatives[1368] = [0.0; Instance::BRANCH_COUNT];
        }

        if ((((scratch.values[1323] != 0.0) && (scratch.values[1340] != 0.0)) && (!(scratch.values[1797] != 0.0))) && (!(scratch.values[1802] != 0.0))) {
            scratch.store_ad(1369, &AdValue::mul(scratch.ad_value(632), AdValue::div(AdValue::mul(scratch.ad_value(1366), scratch.ad_value(602)), scratch.ad_value(1362))));
        }

        if ((((scratch.values[1323] != 0.0) && (scratch.values[1340] != 0.0)) && (!(scratch.values[1797] != 0.0))) && (!(scratch.values[1802] != 0.0))) {
            scratch.store_ad(1370, &AdValue::div(AdValue::scale(scratch.ad_value(629), 0.666666666666667), scratch.ad_value(1369)));
        }

        if ((((scratch.values[1323] != 0.0) && (scratch.values[1340] != 0.0)) && (!(scratch.values[1797] != 0.0))) && (!(scratch.values[1802] != 0.0))) {
            scratch.store_ad(1371, &AdValue::square(scratch.ad_value(1370)));
        }

        if ((((scratch.values[1323] != 0.0) && (scratch.values[1340] != 0.0)) && (!(scratch.values[1797] != 0.0))) && (!(scratch.values[1802] != 0.0))) {
            scratch.store_ad(1372, &AdValue::sqrt(AdValue::div(AdValue::square(scratch.ad_value(1371)), AdValue::offset(AdValue::square(scratch.ad_value(1371)), 1.0))));
        }

        if ((((scratch.values[1323] != 0.0) && (scratch.values[1340] != 0.0)) && (!(scratch.values[1797] != 0.0))) && (!(scratch.values[1802] != 0.0))) {
            scratch.store_ad(1373, &AdValue::sqrt(AdValue::abs(scratch.ad_value(1372))));
        }

        if ((((scratch.values[1323] != 0.0) && (scratch.values[1340] != 0.0)) && (!(scratch.values[1797] != 0.0))) && (!(scratch.values[1802] != 0.0))) {
            scratch.store_ad(1374, &AdValue::mul(scratch.ad_value(1372), scratch.ad_value(1373)));
        }

        scratch.values[1803] = if (((-scratch.values[544]) * scratch.values[605]) == (-1.0)) { 1.0 } else { 0.0 };

        if (((((scratch.values[1323] != 0.0) && (scratch.values[1340] != 0.0)) && (!(scratch.values[1797] != 0.0))) && (!(scratch.values[1802] != 0.0))) && (scratch.values[1803] != 0.0)) {
            scratch.store_ad(1375, &AdValue::div_from_scalar(1.0, AdValue::offset(AdValue::mul(scratch.ad_value(1369), scratch.ad_value(1374)), 1.0)));
        }

        if (((((scratch.values[1323] != 0.0) && (scratch.values[1340] != 0.0)) && (!(scratch.values[1797] != 0.0))) && (!(scratch.values[1802] != 0.0))) && (!(scratch.values[1803] != 0.0))) {
            scratch.store_ad(1375, &AdValue::pow(AdValue::offset(AdValue::mul(scratch.ad_value(1369), scratch.ad_value(1374)), 1.0), AdValue::mul(AdValue::neg(scratch.ad_value(544)), scratch.ad_value(605))));
        }

        if ((((scratch.values[1323] != 0.0) && (scratch.values[1340] != 0.0)) && (!(scratch.values[1797] != 0.0))) && (!(scratch.values[1802] != 0.0))) {
            scratch.store_ad(1376, &AdValue::div(AdValue::mul(scratch.ad_value(1365), scratch.ad_value(1375)), AdValue::add(scratch.ad_value(1365), scratch.ad_value(1375))));
        }

        if ((((scratch.values[1323] != 0.0) && (scratch.values[1340] != 0.0)) && (!(scratch.values[1797] != 0.0))) && (!(scratch.values[1802] != 0.0))) {
            scratch.store_ad(1377, &AdValue::sqrt(AdValue::scale(AdValue::div(scratch.ad_value(1369), scratch.ad_value(1373)), 0.375)));
        }

        if ((((scratch.values[1323] != 0.0) && (scratch.values[1340] != 0.0)) && (!(scratch.values[1797] != 0.0))) && (!(scratch.values[1802] != 0.0))) {
            scratch.store_ad(1378, &AdValue::sub(AdValue::scale(AdValue::mul(scratch.ad_value(1370), scratch.ad_value(1373)), 2.0), scratch.ad_value(1372)));
        }

        if ((((scratch.values[1323] != 0.0) && (scratch.values[1340] != 0.0)) && (!(scratch.values[1797] != 0.0))) && (!(scratch.values[1802] != 0.0))) {
            scratch.store_ad(1379, &AdValue::add(AdValue::sub(AdValue::mul(AdValue::mul(scratch.ad_value(629), scratch.ad_value(1370)), scratch.ad_value(1373)), AdValue::mul(scratch.ad_value(629), scratch.ad_value(1372))), AdValue::scale(AdValue::mul(scratch.ad_value(1369), scratch.ad_value(1374)), 0.5)));
        }

        if ((((scratch.values[1323] != 0.0) && (scratch.values[1340] != 0.0)) && (!(scratch.values[1797] != 0.0))) && (!(scratch.values[1802] != 0.0))) {
            scratch.store_ad(1380, &AdValue::mul(AdValue::offset(scratch.ad_value(1378), (-1.0)), scratch.ad_value(1377)));
        }

        if ((((scratch.values[1323] != 0.0) && (scratch.values[1340] != 0.0)) && (!(scratch.values[1797] != 0.0))) && (!(scratch.values[1802] != 0.0))) {
            scratch.store_ad(1341, &AdValue::square(scratch.ad_value(1380)));
        }

        scratch.values[1804] = if (scratch.values[1380] > 0.0) { 1.0 } else { 0.0 };

        if (((((scratch.values[1323] != 0.0) && (scratch.values[1340] != 0.0)) && (!(scratch.values[1797] != 0.0))) && (!(scratch.values[1802] != 0.0))) && (scratch.values[1804] != 0.0)) {
            scratch.store_ad(1342, &AdValue::div_from_scalar(1.0, AdValue::offset(AdValue::scale(scratch.ad_value(1380), scratch.values[421]), 1.0)));
        }

        if (((((scratch.values[1323] != 0.0) && (scratch.values[1340] != 0.0)) && (!(scratch.values[1797] != 0.0))) && (!(scratch.values[1802] != 0.0))) && (!(scratch.values[1804] != 0.0))) {
            scratch.store_ad(1342, &AdValue::div_from_scalar(1.0, AdValue::sub_from_scalar(1.0, AdValue::scale(scratch.ad_value(1380), scratch.values[421]))));
        }

        scratch.values[1805] = if (((-scratch.values[1341]) + scratch.values[1379]) > (-230.25850929940458)) { 1.0 } else { 0.0 };

        if (((((scratch.values[1323] != 0.0) && (scratch.values[1340] != 0.0)) && (!(scratch.values[1797] != 0.0))) && (!(scratch.values[1802] != 0.0))) && (scratch.values[1805] != 0.0)) {
            scratch.store_ad(1359, &AdValue::exp(AdValue::sub(scratch.ad_value(1379), scratch.ad_value(1341))));
        }

        if (((((scratch.values[1323] != 0.0) && (scratch.values[1340] != 0.0)) && (!(scratch.values[1797] != 0.0))) && (!(scratch.values[1802] != 0.0))) && (!(scratch.values[1805] != 0.0))) {
            let assign31790_ad_e39206: AdValue = AdValue::div_from_scalar(1e-100, AdValue::offset(AdValue::mul(AdValue::sub_from_scalar((-230.25850929940458), AdValue::sub(scratch.ad_value(1379), scratch.ad_value(1341))), AdValue::offset(AdValue::scale(AdValue::mul(AdValue::sub_from_scalar((-230.25850929940458), AdValue::sub(scratch.ad_value(1379), scratch.ad_value(1341))), AdValue::offset(AdValue::scale(AdValue::sub_from_scalar((-230.25850929940458), AdValue::sub(scratch.ad_value(1379), scratch.ad_value(1341))), 0.3333333333333333), 1.0)), 0.5), 1.0)), 1.0));
            scratch.store_ad(1359, &assign31790_ad_e39206);
        }

        if ((((scratch.values[1323] != 0.0) && (scratch.values[1340] != 0.0)) && (!(scratch.values[1797] != 0.0))) && (!(scratch.values[1802] != 0.0))) {
            scratch.store_ad(1343, &AdValue::mul(AdValue::add(AdValue::add(AdValue::scale(scratch.ad_value(1342), 0.29214664), AdValue::scale(AdValue::square(scratch.ad_value(1342)), scratch.values[422])), AdValue::scale(AdValue::mul(AdValue::square(scratch.ad_value(1342)), scratch.ad_value(1342)), scratch.values[423])), scratch.ad_value(1359)));
        }

        scratch.values[1806] = if (scratch.values[1380] > 0.0) { 1.0 } else { 0.0 };

        if (((((scratch.values[1323] != 0.0) && (scratch.values[1340] != 0.0)) && (!(scratch.values[1797] != 0.0))) && (!(scratch.values[1802] != 0.0))) && (scratch.values[1806] != 0.0)) {
            scratch.values[1381] = scratch.values[1343];
            scratch.node_derivatives[1381] = scratch.node_derivatives[1343];
            scratch.branch_derivatives[1381] = scratch.branch_derivatives[1343];
        }

        scratch.values[1807] = if (scratch.values[1379] > (-230.25850929940458)) { 1.0 } else { 0.0 };

        if ((((((scratch.values[1323] != 0.0) && (scratch.values[1340] != 0.0)) && (!(scratch.values[1797] != 0.0))) && (!(scratch.values[1802] != 0.0))) && (!(scratch.values[1806] != 0.0))) && (scratch.values[1807] != 0.0)) {
            scratch.store_ad(1359, &AdValue::exp(scratch.ad_value(1379)));
        }

        if ((((((scratch.values[1323] != 0.0) && (scratch.values[1340] != 0.0)) && (!(scratch.values[1797] != 0.0))) && (!(scratch.values[1802] != 0.0))) && (!(scratch.values[1806] != 0.0))) && (!(scratch.values[1807] != 0.0))) {
            scratch.store_ad(1359, &AdValue::div_from_scalar(1e-100, AdValue::offset(AdValue::mul(AdValue::sub_from_scalar((-230.25850929940458), scratch.ad_value(1379)), AdValue::offset(AdValue::scale(AdValue::mul(AdValue::sub_from_scalar((-230.25850929940458), scratch.ad_value(1379)), AdValue::offset(AdValue::scale(AdValue::sub_from_scalar((-230.25850929940458), scratch.ad_value(1379)), 0.3333333333333333), 1.0)), 0.5), 1.0)), 1.0)));
        }

        if (((((scratch.values[1323] != 0.0) && (scratch.values[1340] != 0.0)) && (!(scratch.values[1797] != 0.0))) && (!(scratch.values[1802] != 0.0))) && (!(scratch.values[1806] != 0.0))) {
            scratch.store_ad(1381, &AdValue::sub(AdValue::scale(scratch.ad_value(1359), 2.0), scratch.ad_value(1343)));
        }

        if ((((scratch.values[1323] != 0.0) && (scratch.values[1340] != 0.0)) && (!(scratch.values[1797] != 0.0))) && (!(scratch.values[1802] != 0.0))) {
            scratch.store_ad(1382, &AdValue::scale(AdValue::div(AdValue::mul(scratch.ad_value(629), scratch.ad_value(1381)), scratch.ad_value(1377)), (1.772453850905516 * 0.5)));
        }

        if ((((scratch.values[1323] != 0.0) && (scratch.values[1340] != 0.0)) && (!(scratch.values[1797] != 0.0))) && (!(scratch.values[1802] != 0.0))) {
            scratch.store_ad(1368, &AdValue::mul(scratch.ad_value(558), AdValue::mul(AdValue::mul(scratch.ad_value(1367), scratch.ad_value(1382)), scratch.ad_value(1376))));
        }

        scratch.values[1808] = if (scratch.values[564] == 0.0) { 1.0 } else { 0.0 };

        if ((((scratch.values[1323] != 0.0) && (scratch.values[1340] != 0.0)) && (!(scratch.values[1797] != 0.0))) && (scratch.values[1808] != 0.0)) {
            scratch.values[1383] = 0.0;
            scratch.node_derivatives[1383] = [0.0; Instance::NODE_COUNT];
            scratch.branch_derivatives[1383] = [0.0; Instance::BRANCH_COUNT];
        }

        scratch.values[1809] = if (scratch.values[544] == 0.5) { 1.0 } else { 0.0 };

        if (((((scratch.values[1323] != 0.0) && (scratch.values[1340] != 0.0)) && (!(scratch.values[1797] != 0.0))) && (!(scratch.values[1808] != 0.0))) && (scratch.values[1809] != 0.0)) {
            scratch.store_ad(1359, &AdValue::sqrt(AdValue::mul(AdValue::sub(scratch.ad_value(541), scratch.ad_value(1357)), scratch.ad_value(623))));
        }

        if (((((scratch.values[1323] != 0.0) && (scratch.values[1340] != 0.0)) && (!(scratch.values[1797] != 0.0))) && (!(scratch.values[1808] != 0.0))) && (!(scratch.values[1809] != 0.0))) {
            scratch.store_ad(1359, &AdValue::pow(AdValue::mul(AdValue::sub(scratch.ad_value(541), scratch.ad_value(1357)), scratch.ad_value(623)), scratch.ad_value(544)));
        }

        if ((((scratch.values[1323] != 0.0) && (scratch.values[1340] != 0.0)) && (!(scratch.values[1797] != 0.0))) && (!(scratch.values[1808] != 0.0))) {
            scratch.store_ad(1384, &AdValue::mul(scratch.ad_value(605), AdValue::div(AdValue::mul(AdValue::sub(scratch.ad_value(541), scratch.ad_value(1357)), scratch.ad_value(620)), scratch.ad_value(1359))));
        }

        scratch.values[1810] = if (((((-scratch.values[635]) / scratch.values[1384])) as f64).abs() < 230.25850929940458) { 1.0 } else { 0.0 };

        if (((((scratch.values[1323] != 0.0) && (scratch.values[1340] != 0.0)) && (!(scratch.values[1797] != 0.0))) && (!(scratch.values[1808] != 0.0))) && (scratch.values[1810] != 0.0)) {
            scratch.store_ad(1359, &AdValue::exp(AdValue::div(AdValue::neg(scratch.ad_value(635)), scratch.ad_value(1384))));
        }

        scratch.values[1811] = if (((-scratch.values[635]) / scratch.values[1384]) < 0.0) { 1.0 } else { 0.0 };

        if ((((((scratch.values[1323] != 0.0) && (scratch.values[1340] != 0.0)) && (!(scratch.values[1797] != 0.0))) && (!(scratch.values[1808] != 0.0))) && (!(scratch.values[1810] != 0.0))) && (scratch.values[1811] != 0.0)) {
            let assign31980_ad_e39533: AdValue = AdValue::offset(AdValue::mul(AdValue::sub_from_scalar((-230.25850929940458), AdValue::div(AdValue::neg(scratch.ad_value(635)), scratch.ad_value(1384))), AdValue::offset(AdValue::scale(AdValue::mul(AdValue::sub_from_scalar((-230.25850929940458), AdValue::div(AdValue::neg(scratch.ad_value(635)), scratch.ad_value(1384))), AdValue::offset(AdValue::scale(AdValue::sub_from_scalar((-230.25850929940458), AdValue::div(AdValue::neg(scratch.ad_value(635)), scratch.ad_value(1384))), 0.3333333333333333), 1.0)), 0.5), 1.0)), 1.0);
            scratch.store_ad(1359, &AdValue::div_from_scalar(1e-100, assign31980_ad_e39533));
        }

        if ((((((scratch.values[1323] != 0.0) && (scratch.values[1340] != 0.0)) && (!(scratch.values[1797] != 0.0))) && (!(scratch.values[1808] != 0.0))) && (!(scratch.values[1810] != 0.0))) && (!(scratch.values[1811] != 0.0))) {
            let assign31990_ad_e39583: AdValue = AdValue::scale(AdValue::offset(AdValue::mul(AdValue::offset(AdValue::div(AdValue::neg(scratch.ad_value(635)), scratch.ad_value(1384)), (-230.25850929940458)), AdValue::offset(AdValue::scale(AdValue::mul(AdValue::offset(AdValue::div(AdValue::neg(scratch.ad_value(635)), scratch.ad_value(1384)), (-230.25850929940458)), AdValue::offset(AdValue::scale(AdValue::offset(AdValue::div(AdValue::neg(scratch.ad_value(635)), scratch.ad_value(1384)), (-230.25850929940458)), 0.3333333333333333), 1.0)), 0.5), 1.0)), 1.0), 1e100);
            scratch.store_ad(1359, &assign31990_ad_e39583);
        }

        if ((((scratch.values[1323] != 0.0) && (scratch.values[1340] != 0.0)) && (!(scratch.values[1797] != 0.0))) && (!(scratch.values[1808] != 0.0))) {
            scratch.store_ad(1383, &AdValue::mul(scratch.ad_value(564), AdValue::mul(AdValue::mul(AdValue::mul(scratch.ad_value(517), scratch.ad_value(1384)), scratch.ad_value(1384)), scratch.ad_value(1359))));
        }

        scratch.values[1812] = if (scratch.values[573] > 1000.0) { 1.0 } else { 0.0 };

        if ((((scratch.values[1323] != 0.0) && (scratch.values[1340] != 0.0)) && (!(scratch.values[1797] != 0.0))) && (scratch.values[1812] != 0.0)) {
            scratch.values[1385] = 1.0;
            scratch.node_derivatives[1385] = [0.0; Instance::NODE_COUNT];
            scratch.branch_derivatives[1385] = [0.0; Instance::BRANCH_COUNT];
        }

        scratch.values[1813] = if (scratch.values[1358] > ((-scratch.values[493]) * scratch.values[573])) { 1.0 } else { 0.0 };

        scratch.values[1814] = if (scratch.values[576] == 4.0) { 1.0 } else { 0.0 };

        if ((((((scratch.values[1323] != 0.0) && (scratch.values[1340] != 0.0)) && (!(scratch.values[1797] != 0.0))) && (!(scratch.values[1812] != 0.0))) && (scratch.values[1813] != 0.0)) && (scratch.values[1814] != 0.0)) {
            scratch.store_ad(1359, &AdValue::mul(AdValue::mul(AdValue::mul(AdValue::mul(scratch.ad_value(1358), scratch.ad_value(641)), AdValue::mul(scratch.ad_value(1358), scratch.ad_value(641))), AdValue::mul(scratch.ad_value(1358), scratch.ad_value(641))), AdValue::mul(scratch.ad_value(1358), scratch.ad_value(641))));
        }

        if ((((((scratch.values[1323] != 0.0) && (scratch.values[1340] != 0.0)) && (!(scratch.values[1797] != 0.0))) && (!(scratch.values[1812] != 0.0))) && (scratch.values[1813] != 0.0)) && (!(scratch.values[1814] != 0.0))) {
            scratch.store_ad(1359, &AdValue::pow(AdValue::abs(AdValue::mul(scratch.ad_value(1358), scratch.ad_value(641))), scratch.ad_value(576)));
        }

        if (((((scratch.values[1323] != 0.0) && (scratch.values[1340] != 0.0)) && (!(scratch.values[1797] != 0.0))) && (!(scratch.values[1812] != 0.0))) && (scratch.values[1813] != 0.0)) {
            scratch.store_ad(1385, &AdValue::div_from_scalar(1.0, AdValue::sub_from_scalar(1.0, scratch.ad_value(1359))));
        }

        if (((((scratch.values[1323] != 0.0) && (scratch.values[1340] != 0.0)) && (!(scratch.values[1797] != 0.0))) && (!(scratch.values[1812] != 0.0))) && (!(scratch.values[1813] != 0.0))) {
            scratch.store_ad(1385, &AdValue::add(scratch.ad_value(638), AdValue::mul(AdValue::add(scratch.ad_value(1358), AdValue::scale(scratch.ad_value(573), scratch.values[493])), scratch.ad_value(644))));
        }

        if (((scratch.values[1323] != 0.0) && (scratch.values[1340] != 0.0)) && (!(scratch.values[1797] != 0.0))) {
            scratch.store_ad(1388, &AdValue::mul(AdValue::add(AdValue::add(AdValue::add(scratch.ad_value(1360), scratch.ad_value(1361)), scratch.ad_value(1368)), scratch.ad_value(1383)), scratch.ad_value(1385)));
        }

        if ((scratch.values[1323] != 0.0) && (scratch.values[1340] != 0.0)) {
            scratch.store_ad(507, &AdValue::add(AdValue::add(AdValue::mul(scratch.ad_value(717), scratch.ad_value(1386)), AdValue::mul(scratch.ad_value(718), scratch.ad_value(1387))), AdValue::mul(scratch.ad_value(719), scratch.ad_value(1388))));
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

        scratch.values[1815] = if !(((scratch.values[717] == 0.0) && (scratch.values[718] == 0.0)) && (scratch.values[719] == 0.0)) { 1.0 } else { 0.0 };

        if (((scratch.values[1323] != 0.0) && (scratch.values[1340] != 0.0)) && (scratch.values[1815] != 0.0)) {
            scratch.store_ad(1344, &AdValue::mul(AdValue::scale(scratch.ad_value(728), 4.0), scratch.ad_value(728)));
        }

        if (((scratch.values[1323] != 0.0) && (scratch.values[1340] != 0.0)) && (scratch.values[1815] != 0.0)) {
            scratch.store_ad(1345, &AdValue::div(scratch.ad_value(728), scratch.ad_value(729)));
        }

        if (((scratch.values[1323] != 0.0) && (scratch.values[1340] != 0.0)) && (scratch.values[1815] != 0.0)) {
            scratch.store_ad(1346, &AdValue::add(scratch.ad_value(518), AdValue::mul(scratch.ad_value(728), scratch.ad_value(1345))));
        }

        if (((scratch.values[1323] != 0.0) && (scratch.values[1340] != 0.0)) && (scratch.values[1815] != 0.0)) {
            scratch.store_ad(1347, &AdValue::add(scratch.ad_value(729), scratch.ad_value(1346)));
        }

        if (((scratch.values[1323] != 0.0) && (scratch.values[1340] != 0.0)) && (scratch.values[1815] != 0.0)) {
            scratch.store_ad(1348, &AdValue::sub(scratch.ad_value(729), scratch.ad_value(1346)));
        }

        if (((scratch.values[1323] != 0.0) && (scratch.values[1340] != 0.0)) && (scratch.values[1815] != 0.0)) {
            scratch.store_ad(1349, &AdValue::sqrt(AdValue::add(AdValue::square(scratch.ad_value(1348)), scratch.ad_value(1344))));
        }

        if (((scratch.values[1323] != 0.0) && (scratch.values[1340] != 0.0)) && (scratch.values[1815] != 0.0)) {
            scratch.store_ad(1351, &AdValue::scale(AdValue::div(AdValue::mul(scratch.ad_value(518), scratch.ad_value(729)), AdValue::add(scratch.ad_value(1347), scratch.ad_value(1349))), 2.0));
        }

        scratch.values[1816] = if (scratch.values[518] < scratch.values[725]) { 1.0 } else { 0.0 };

        scratch.values[1817] = if ((((0.5 * (scratch.values[518] * scratch.values[420]))) as f64).abs() < 230.25850929940458) { 1.0 } else { 0.0 };

        if (((((scratch.values[1323] != 0.0) && (scratch.values[1340] != 0.0)) && (scratch.values[1815] != 0.0)) && (scratch.values[1816] != 0.0)) && (scratch.values[1817] != 0.0)) {
            scratch.store_ad(1353, &AdValue::exp(AdValue::scale(scratch.ad_value(518), (scratch.values[420] * 0.5))));
        }

        scratch.values[1818] = if ((0.5 * (scratch.values[518] * scratch.values[420])) < 0.0) { 1.0 } else { 0.0 };

        if ((((((scratch.values[1323] != 0.0) && (scratch.values[1340] != 0.0)) && (scratch.values[1815] != 0.0)) && (scratch.values[1816] != 0.0)) && (!(scratch.values[1817] != 0.0))) && (scratch.values[1818] != 0.0)) {
            let assign32250_ad_e39946: AdValue = AdValue::div_from_scalar(1e-100, AdValue::offset(AdValue::mul(AdValue::sub_from_scalar((-230.25850929940458), AdValue::scale(scratch.ad_value(518), (scratch.values[420] * 0.5))), AdValue::offset(AdValue::scale(AdValue::mul(AdValue::sub_from_scalar((-230.25850929940458), AdValue::scale(scratch.ad_value(518), (scratch.values[420] * 0.5))), AdValue::offset(AdValue::scale(AdValue::sub_from_scalar((-230.25850929940458), AdValue::scale(scratch.ad_value(518), (scratch.values[420] * 0.5))), 0.3333333333333333), 1.0)), 0.5), 1.0)), 1.0));
            scratch.store_ad(1353, &assign32250_ad_e39946);
        }

        if ((((((scratch.values[1323] != 0.0) && (scratch.values[1340] != 0.0)) && (scratch.values[1815] != 0.0)) && (scratch.values[1816] != 0.0)) && (!(scratch.values[1817] != 0.0))) && (!(scratch.values[1818] != 0.0))) {
            scratch.store_ad(1353, &AdValue::scale(AdValue::offset(AdValue::mul(AdValue::offset(AdValue::scale(scratch.ad_value(518), (scratch.values[420] * 0.5)), (-230.25850929940458)), AdValue::offset(AdValue::scale(AdValue::mul(AdValue::offset(AdValue::scale(scratch.ad_value(518), (scratch.values[420] * 0.5)), (-230.25850929940458)), AdValue::offset(AdValue::scale(AdValue::offset(AdValue::scale(scratch.ad_value(518), (scratch.values[420] * 0.5)), (-230.25850929940458)), 0.3333333333333333), 1.0)), 0.5), 1.0)), 1.0), 1e100));
        }

        if ((((scratch.values[1323] != 0.0) && (scratch.values[1340] != 0.0)) && (scratch.values[1815] != 0.0)) && (scratch.values[1816] != 0.0)) {
            scratch.store_ad(1350, &AdValue::square(scratch.ad_value(1353)));
        }

        if ((((scratch.values[1323] != 0.0) && (scratch.values[1340] != 0.0)) && (scratch.values[1815] != 0.0)) && (!(scratch.values[1816] != 0.0))) {
            scratch.store_ad(1350, &AdValue::mul(AdValue::offset(AdValue::scale(AdValue::sub(scratch.ad_value(518), scratch.ad_value(725)), scratch.values[420]), 1.0), scratch.ad_value(726)));
        }

        if ((((scratch.values[1323] != 0.0) && (scratch.values[1340] != 0.0)) && (scratch.values[1815] != 0.0)) && (!(scratch.values[1816] != 0.0))) {
            scratch.store_ad(1353, &AdValue::sqrt(scratch.ad_value(1350)));
        }

        if (((scratch.values[1323] != 0.0) && (scratch.values[1340] != 0.0)) && (scratch.values[1815] != 0.0)) {
            scratch.store_ad(1350, &AdValue::offset(scratch.ad_value(1350), (-1.0)));
        }

        if (((scratch.values[1323] != 0.0) && (scratch.values[1340] != 0.0)) && (scratch.values[1815] != 0.0)) {
            scratch.store_ad(1352, &AdValue::div_from_scalar(1.0, scratch.ad_value(1353)));
        }

        scratch.values[1819] = if (scratch.values[518] > 0.0) { 1.0 } else { 0.0 };

        if ((((scratch.values[1323] != 0.0) && (scratch.values[1340] != 0.0)) && (scratch.values[1815] != 0.0)) && (scratch.values[1819] != 0.0)) {
            scratch.store_ad(1354, &AdValue::scale(AdValue::ln(AdValue::add(AdValue::offset(scratch.ad_value(1352), 2.0), AdValue::sqrt(AdValue::mul(AdValue::offset(scratch.ad_value(1352), 1.0), AdValue::offset(scratch.ad_value(1352), 3.0))))), (scratch.values[419] * 2.0)));
        }

        if ((((scratch.values[1323] != 0.0) && (scratch.values[1340] != 0.0)) && (scratch.values[1815] != 0.0)) && (!(scratch.values[1819] != 0.0))) {
            scratch.store_ad(1354, &AdValue::sub(AdValue::scale(AdValue::ln(AdValue::add(AdValue::offset(AdValue::scale(scratch.ad_value(1353), 2.0), 1.0), AdValue::sqrt(AdValue::mul(AdValue::offset(scratch.ad_value(1353), 1.0), AdValue::offset(AdValue::scale(scratch.ad_value(1353), 3.0), 1.0))))), (scratch.values[419] * 2.0)), scratch.ad_value(518)));
        }

        if (((scratch.values[1323] != 0.0) && (scratch.values[1340] != 0.0)) && (scratch.values[1815] != 0.0)) {
            scratch.store_ad(1355, &AdValue::sub(scratch.ad_value(727), scratch.ad_value(1354)));
        }

        if (((scratch.values[1323] != 0.0) && (scratch.values[1340] != 0.0)) && (scratch.values[1815] != 0.0)) {
            scratch.store_ad(1356, &AdValue::scale(AdValue::sub(AdValue::add(scratch.ad_value(518), scratch.ad_value(1355)), AdValue::sqrt(AdValue::offset(AdValue::mul(AdValue::sub(scratch.ad_value(518), scratch.ad_value(1355)), AdValue::sub(scratch.ad_value(518), scratch.ad_value(1355))), ((4.0 * scratch.values[419]) * scratch.values[419])))), 0.5));
        }

        if (((scratch.values[1323] != 0.0) && (scratch.values[1340] != 0.0)) && (scratch.values[1815] != 0.0)) {
            scratch.store_ad(1357, &AdValue::scale(AdValue::sub(AdValue::add(scratch.ad_value(518), scratch.ad_value(730)), AdValue::sqrt(AdValue::offset(AdValue::mul(AdValue::sub(scratch.ad_value(518), scratch.ad_value(730)), AdValue::sub(scratch.ad_value(518), scratch.ad_value(730))), ((4.0 * scratch.values[417]) * scratch.values[417])))), 0.5));
        }

        if (((scratch.values[1323] != 0.0) && (scratch.values[1340] != 0.0)) && (scratch.values[1815] != 0.0)) {
            scratch.store_ad(1358, &AdValue::scale(AdValue::sub(scratch.ad_value(518), AdValue::sqrt(AdValue::offset(AdValue::mul(scratch.ad_value(518), scratch.ad_value(518)), ((4.0 * 1e-6) * 1e-6)))), 0.5));
        }

        scratch.values[1820] = if (scratch.values[717] == 0.0) { 1.0 } else { 0.0 };

        if (((scratch.values[1323] != 0.0) && (scratch.values[1340] != 0.0)) && (scratch.values[1820] != 0.0)) {
            scratch.values[1386] = 0.0;
            scratch.node_derivatives[1386] = [0.0; Instance::NODE_COUNT];
            scratch.branch_derivatives[1386] = [0.0; Instance::BRANCH_COUNT];
        }

        scratch.values[1821] = if (scratch.values[600] == 0.5) { 1.0 } else { 0.0 };

        if ((((scratch.values[1323] != 0.0) && (scratch.values[1340] != 0.0)) && (!(scratch.values[1820] != 0.0))) && (scratch.values[1821] != 0.0)) {
            scratch.store_ad(1359, &AdValue::sqrt(AdValue::sub_from_scalar(1.0, AdValue::mul(scratch.ad_value(1351), scratch.ad_value(597)))));
        }

        if ((((scratch.values[1323] != 0.0) && (scratch.values[1340] != 0.0)) && (!(scratch.values[1820] != 0.0))) && (!(scratch.values[1821] != 0.0))) {
            scratch.store_ad(1359, &AdValue::pow(AdValue::sub_from_scalar(1.0, AdValue::mul(scratch.ad_value(1351), scratch.ad_value(597))), scratch.ad_value(600)));
        }

        if (((scratch.values[1323] != 0.0) && (scratch.values[1340] != 0.0)) && (!(scratch.values[1820] != 0.0))) {
            scratch.store_ad(1360, &AdValue::mul(scratch.ad_value(588), scratch.ad_value(1350)));
        }

        scratch.values[1822] = if ((scratch.values[553] == 0.0) && (scratch.values[556] == 0.0)) { 1.0 } else { 0.0 };

        if ((((scratch.values[1323] != 0.0) && (scratch.values[1340] != 0.0)) && (!(scratch.values[1820] != 0.0))) && (scratch.values[1822] != 0.0)) {
            scratch.values[1361] = 0.0;
            scratch.node_derivatives[1361] = [0.0; Instance::NODE_COUNT];
            scratch.branch_derivatives[1361] = [0.0; Instance::BRANCH_COUNT];
        }

        if ((((scratch.values[1323] != 0.0) && (scratch.values[1340] != 0.0)) && (!(scratch.values[1820] != 0.0))) && (!(scratch.values[1822] != 0.0))) {
            scratch.store_ad(1362, &AdValue::sub(scratch.ad_value(594), scratch.ad_value(1356)));
        }

        if ((((scratch.values[1323] != 0.0) && (scratch.values[1340] != 0.0)) && (!(scratch.values[1820] != 0.0))) && (!(scratch.values[1822] != 0.0))) {
            scratch.store_ad(1363, &AdValue::sub_from_scalar(1.0, AdValue::sqrt(AdValue::sub_from_scalar(1.0, AdValue::div(scratch.ad_value(1354), scratch.ad_value(1362))))));
        }

        scratch.values[1823] = if (scratch.values[542] == 0.5) { 1.0 } else { 0.0 };

        if (((((scratch.values[1323] != 0.0) && (scratch.values[1340] != 0.0)) && (!(scratch.values[1820] != 0.0))) && (!(scratch.values[1822] != 0.0))) && (scratch.values[1823] != 0.0)) {
            scratch.values[1364] = 0.0;
            scratch.node_derivatives[1364] = [0.0; Instance::NODE_COUNT];
            scratch.branch_derivatives[1364] = [0.0; Instance::BRANCH_COUNT];
        }

        if (((((scratch.values[1323] != 0.0) && (scratch.values[1340] != 0.0)) && (!(scratch.values[1820] != 0.0))) && (!(scratch.values[1822] != 0.0))) && (!(scratch.values[1823] != 0.0))) {
            scratch.store_ad(1364, &AdValue::mul(AdValue::add(AdValue::div(AdValue::mul(AdValue::square(scratch.ad_value(1363)), AdValue::ln(scratch.ad_value(1363))), AdValue::sub_from_scalar(1.0, scratch.ad_value(1363))), scratch.ad_value(1363)), AdValue::sub_from_scalar(1.0, AdValue::scale(scratch.ad_value(542), 2.0))));
        }

        if ((((scratch.values[1323] != 0.0) && (scratch.values[1340] != 0.0)) && (!(scratch.values[1820] != 0.0))) && (!(scratch.values[1822] != 0.0))) {
            scratch.store_ad(1365, &AdValue::add(scratch.ad_value(1363), scratch.ad_value(1364)));
        }

        scratch.values[1824] = if (scratch.values[542] == 0.5) { 1.0 } else { 0.0 };

        if (((((scratch.values[1323] != 0.0) && (scratch.values[1340] != 0.0)) && (!(scratch.values[1820] != 0.0))) && (!(scratch.values[1822] != 0.0))) && (scratch.values[1824] != 0.0)) {
            scratch.store_ad(1359, &AdValue::sqrt(AdValue::mul(scratch.ad_value(1362), scratch.ad_value(621))));
        }

        if (((((scratch.values[1323] != 0.0) && (scratch.values[1340] != 0.0)) && (!(scratch.values[1820] != 0.0))) && (!(scratch.values[1822] != 0.0))) && (!(scratch.values[1824] != 0.0))) {
            scratch.store_ad(1359, &AdValue::pow(AdValue::mul(scratch.ad_value(1362), scratch.ad_value(621)), scratch.ad_value(542)));
        }

        if ((((scratch.values[1323] != 0.0) && (scratch.values[1340] != 0.0)) && (!(scratch.values[1820] != 0.0))) && (!(scratch.values[1822] != 0.0))) {
            scratch.store_ad(1366, &AdValue::mul(scratch.ad_value(615), scratch.ad_value(1359)));
        }

        if ((((scratch.values[1323] != 0.0) && (scratch.values[1340] != 0.0)) && (!(scratch.values[1820] != 0.0))) && (!(scratch.values[1822] != 0.0))) {
            scratch.store_ad(1367, &AdValue::mul(scratch.ad_value(585), AdValue::mul(AdValue::offset(scratch.ad_value(1353), (-1.0)), scratch.ad_value(1366))));
        }

        if ((((scratch.values[1323] != 0.0) && (scratch.values[1340] != 0.0)) && (!(scratch.values[1820] != 0.0))) && (!(scratch.values[1822] != 0.0))) {
            scratch.store_ad(1361, &AdValue::mul(scratch.ad_value(553), AdValue::mul(scratch.ad_value(1367), scratch.ad_value(1365))));
        }

        scratch.values[1825] = if (scratch.values[556] == 0.0) { 1.0 } else { 0.0 };

        if ((((scratch.values[1323] != 0.0) && (scratch.values[1340] != 0.0)) && (!(scratch.values[1820] != 0.0))) && (scratch.values[1825] != 0.0)) {
            scratch.values[1368] = 0.0;
            scratch.node_derivatives[1368] = [0.0; Instance::NODE_COUNT];
            scratch.branch_derivatives[1368] = [0.0; Instance::BRANCH_COUNT];
        }

        if ((((scratch.values[1323] != 0.0) && (scratch.values[1340] != 0.0)) && (!(scratch.values[1820] != 0.0))) && (!(scratch.values[1825] != 0.0))) {
            scratch.store_ad(1369, &AdValue::mul(scratch.ad_value(630), AdValue::div(AdValue::mul(scratch.ad_value(1366), scratch.ad_value(600)), scratch.ad_value(1362))));
        }

        if ((((scratch.values[1323] != 0.0) && (scratch.values[1340] != 0.0)) && (!(scratch.values[1820] != 0.0))) && (!(scratch.values[1825] != 0.0))) {
            scratch.store_ad(1370, &AdValue::div(AdValue::scale(scratch.ad_value(627), 0.666666666666667), scratch.ad_value(1369)));
        }

        if ((((scratch.values[1323] != 0.0) && (scratch.values[1340] != 0.0)) && (!(scratch.values[1820] != 0.0))) && (!(scratch.values[1825] != 0.0))) {
            scratch.store_ad(1371, &AdValue::square(scratch.ad_value(1370)));
        }

        if ((((scratch.values[1323] != 0.0) && (scratch.values[1340] != 0.0)) && (!(scratch.values[1820] != 0.0))) && (!(scratch.values[1825] != 0.0))) {
            scratch.store_ad(1372, &AdValue::sqrt(AdValue::div(AdValue::square(scratch.ad_value(1371)), AdValue::offset(AdValue::square(scratch.ad_value(1371)), 1.0))));
        }

        if ((((scratch.values[1323] != 0.0) && (scratch.values[1340] != 0.0)) && (!(scratch.values[1820] != 0.0))) && (!(scratch.values[1825] != 0.0))) {
            scratch.store_ad(1373, &AdValue::sqrt(AdValue::abs(scratch.ad_value(1372))));
        }

        if ((((scratch.values[1323] != 0.0) && (scratch.values[1340] != 0.0)) && (!(scratch.values[1820] != 0.0))) && (!(scratch.values[1825] != 0.0))) {
            scratch.store_ad(1374, &AdValue::mul(scratch.ad_value(1372), scratch.ad_value(1373)));
        }

        scratch.values[1826] = if (((-scratch.values[542]) * scratch.values[603]) == (-1.0)) { 1.0 } else { 0.0 };

        if (((((scratch.values[1323] != 0.0) && (scratch.values[1340] != 0.0)) && (!(scratch.values[1820] != 0.0))) && (!(scratch.values[1825] != 0.0))) && (scratch.values[1826] != 0.0)) {
            scratch.store_ad(1375, &AdValue::div_from_scalar(1.0, AdValue::offset(AdValue::mul(scratch.ad_value(1369), scratch.ad_value(1374)), 1.0)));
        }

        if (((((scratch.values[1323] != 0.0) && (scratch.values[1340] != 0.0)) && (!(scratch.values[1820] != 0.0))) && (!(scratch.values[1825] != 0.0))) && (!(scratch.values[1826] != 0.0))) {
            scratch.store_ad(1375, &AdValue::pow(AdValue::offset(AdValue::mul(scratch.ad_value(1369), scratch.ad_value(1374)), 1.0), AdValue::mul(AdValue::neg(scratch.ad_value(542)), scratch.ad_value(603))));
        }

        if ((((scratch.values[1323] != 0.0) && (scratch.values[1340] != 0.0)) && (!(scratch.values[1820] != 0.0))) && (!(scratch.values[1825] != 0.0))) {
            scratch.store_ad(1376, &AdValue::div(AdValue::mul(scratch.ad_value(1365), scratch.ad_value(1375)), AdValue::add(scratch.ad_value(1365), scratch.ad_value(1375))));
        }

        if ((((scratch.values[1323] != 0.0) && (scratch.values[1340] != 0.0)) && (!(scratch.values[1820] != 0.0))) && (!(scratch.values[1825] != 0.0))) {
            scratch.store_ad(1377, &AdValue::sqrt(AdValue::scale(AdValue::div(scratch.ad_value(1369), scratch.ad_value(1373)), 0.375)));
        }

        if ((((scratch.values[1323] != 0.0) && (scratch.values[1340] != 0.0)) && (!(scratch.values[1820] != 0.0))) && (!(scratch.values[1825] != 0.0))) {
            scratch.store_ad(1378, &AdValue::sub(AdValue::scale(AdValue::mul(scratch.ad_value(1370), scratch.ad_value(1373)), 2.0), scratch.ad_value(1372)));
        }

        if ((((scratch.values[1323] != 0.0) && (scratch.values[1340] != 0.0)) && (!(scratch.values[1820] != 0.0))) && (!(scratch.values[1825] != 0.0))) {
            scratch.store_ad(1379, &AdValue::add(AdValue::sub(AdValue::mul(AdValue::mul(scratch.ad_value(627), scratch.ad_value(1370)), scratch.ad_value(1373)), AdValue::mul(scratch.ad_value(627), scratch.ad_value(1372))), AdValue::scale(AdValue::mul(scratch.ad_value(1369), scratch.ad_value(1374)), 0.5)));
        }

        if ((((scratch.values[1323] != 0.0) && (scratch.values[1340] != 0.0)) && (!(scratch.values[1820] != 0.0))) && (!(scratch.values[1825] != 0.0))) {
            scratch.store_ad(1380, &AdValue::mul(AdValue::offset(scratch.ad_value(1378), (-1.0)), scratch.ad_value(1377)));
        }

        if ((((scratch.values[1323] != 0.0) && (scratch.values[1340] != 0.0)) && (!(scratch.values[1820] != 0.0))) && (!(scratch.values[1825] != 0.0))) {
            scratch.store_ad(1341, &AdValue::square(scratch.ad_value(1380)));
        }

        scratch.values[1827] = if (scratch.values[1380] > 0.0) { 1.0 } else { 0.0 };

        if (((((scratch.values[1323] != 0.0) && (scratch.values[1340] != 0.0)) && (!(scratch.values[1820] != 0.0))) && (!(scratch.values[1825] != 0.0))) && (scratch.values[1827] != 0.0)) {
            scratch.store_ad(1342, &AdValue::div_from_scalar(1.0, AdValue::offset(AdValue::scale(scratch.ad_value(1380), scratch.values[421]), 1.0)));
        }

        if (((((scratch.values[1323] != 0.0) && (scratch.values[1340] != 0.0)) && (!(scratch.values[1820] != 0.0))) && (!(scratch.values[1825] != 0.0))) && (!(scratch.values[1827] != 0.0))) {
            scratch.store_ad(1342, &AdValue::div_from_scalar(1.0, AdValue::sub_from_scalar(1.0, AdValue::scale(scratch.ad_value(1380), scratch.values[421]))));
        }

        scratch.values[1828] = if (((-scratch.values[1341]) + scratch.values[1379]) > (-230.25850929940458)) { 1.0 } else { 0.0 };

        if (((((scratch.values[1323] != 0.0) && (scratch.values[1340] != 0.0)) && (!(scratch.values[1820] != 0.0))) && (!(scratch.values[1825] != 0.0))) && (scratch.values[1828] != 0.0)) {
            scratch.store_ad(1359, &AdValue::exp(AdValue::sub(scratch.ad_value(1379), scratch.ad_value(1341))));
        }

        if (((((scratch.values[1323] != 0.0) && (scratch.values[1340] != 0.0)) && (!(scratch.values[1820] != 0.0))) && (!(scratch.values[1825] != 0.0))) && (!(scratch.values[1828] != 0.0))) {
            let assign32830_ad_e40889: AdValue = AdValue::div_from_scalar(1e-100, AdValue::offset(AdValue::mul(AdValue::sub_from_scalar((-230.25850929940458), AdValue::sub(scratch.ad_value(1379), scratch.ad_value(1341))), AdValue::offset(AdValue::scale(AdValue::mul(AdValue::sub_from_scalar((-230.25850929940458), AdValue::sub(scratch.ad_value(1379), scratch.ad_value(1341))), AdValue::offset(AdValue::scale(AdValue::sub_from_scalar((-230.25850929940458), AdValue::sub(scratch.ad_value(1379), scratch.ad_value(1341))), 0.3333333333333333), 1.0)), 0.5), 1.0)), 1.0));
            scratch.store_ad(1359, &assign32830_ad_e40889);
        }

        if ((((scratch.values[1323] != 0.0) && (scratch.values[1340] != 0.0)) && (!(scratch.values[1820] != 0.0))) && (!(scratch.values[1825] != 0.0))) {
            scratch.store_ad(1343, &AdValue::mul(AdValue::add(AdValue::add(AdValue::scale(scratch.ad_value(1342), 0.29214664), AdValue::scale(AdValue::square(scratch.ad_value(1342)), scratch.values[422])), AdValue::scale(AdValue::mul(AdValue::square(scratch.ad_value(1342)), scratch.ad_value(1342)), scratch.values[423])), scratch.ad_value(1359)));
        }

        scratch.values[1829] = if (scratch.values[1380] > 0.0) { 1.0 } else { 0.0 };

        if (((((scratch.values[1323] != 0.0) && (scratch.values[1340] != 0.0)) && (!(scratch.values[1820] != 0.0))) && (!(scratch.values[1825] != 0.0))) && (scratch.values[1829] != 0.0)) {
            scratch.values[1381] = scratch.values[1343];
            scratch.node_derivatives[1381] = scratch.node_derivatives[1343];
            scratch.branch_derivatives[1381] = scratch.branch_derivatives[1343];
        }

        scratch.values[1830] = if (scratch.values[1379] > (-230.25850929940458)) { 1.0 } else { 0.0 };

        if ((((((scratch.values[1323] != 0.0) && (scratch.values[1340] != 0.0)) && (!(scratch.values[1820] != 0.0))) && (!(scratch.values[1825] != 0.0))) && (!(scratch.values[1829] != 0.0))) && (scratch.values[1830] != 0.0)) {
            scratch.store_ad(1359, &AdValue::exp(scratch.ad_value(1379)));
        }

        if ((((((scratch.values[1323] != 0.0) && (scratch.values[1340] != 0.0)) && (!(scratch.values[1820] != 0.0))) && (!(scratch.values[1825] != 0.0))) && (!(scratch.values[1829] != 0.0))) && (!(scratch.values[1830] != 0.0))) {
            scratch.store_ad(1359, &AdValue::div_from_scalar(1e-100, AdValue::offset(AdValue::mul(AdValue::sub_from_scalar((-230.25850929940458), scratch.ad_value(1379)), AdValue::offset(AdValue::scale(AdValue::mul(AdValue::sub_from_scalar((-230.25850929940458), scratch.ad_value(1379)), AdValue::offset(AdValue::scale(AdValue::sub_from_scalar((-230.25850929940458), scratch.ad_value(1379)), 0.3333333333333333), 1.0)), 0.5), 1.0)), 1.0)));
        }

        if (((((scratch.values[1323] != 0.0) && (scratch.values[1340] != 0.0)) && (!(scratch.values[1820] != 0.0))) && (!(scratch.values[1825] != 0.0))) && (!(scratch.values[1829] != 0.0))) {
            scratch.store_ad(1381, &AdValue::sub(AdValue::scale(scratch.ad_value(1359), 2.0), scratch.ad_value(1343)));
        }

        if ((((scratch.values[1323] != 0.0) && (scratch.values[1340] != 0.0)) && (!(scratch.values[1820] != 0.0))) && (!(scratch.values[1825] != 0.0))) {
            scratch.store_ad(1382, &AdValue::scale(AdValue::div(AdValue::mul(scratch.ad_value(627), scratch.ad_value(1381)), scratch.ad_value(1377)), (1.772453850905516 * 0.5)));
        }

        if ((((scratch.values[1323] != 0.0) && (scratch.values[1340] != 0.0)) && (!(scratch.values[1820] != 0.0))) && (!(scratch.values[1825] != 0.0))) {
            scratch.store_ad(1368, &AdValue::mul(scratch.ad_value(556), AdValue::mul(AdValue::mul(scratch.ad_value(1367), scratch.ad_value(1382)), scratch.ad_value(1376))));
        }

        scratch.values[1831] = if (scratch.values[562] == 0.0) { 1.0 } else { 0.0 };

        if ((((scratch.values[1323] != 0.0) && (scratch.values[1340] != 0.0)) && (!(scratch.values[1820] != 0.0))) && (scratch.values[1831] != 0.0)) {
            scratch.values[1383] = 0.0;
            scratch.node_derivatives[1383] = [0.0; Instance::NODE_COUNT];
            scratch.branch_derivatives[1383] = [0.0; Instance::BRANCH_COUNT];
        }

    }

    pub(super) fn stamp_transient_block_25(
        &mut self,
        ctx: &GeneratedEvalContext<'_>,
        stamper: &mut GeneratedStamper<'_>,
        scratch: &mut Scratch,
    ) {
        let _ = stamper;
        scratch.values[1832] = if (scratch.values[542] == 0.5) { 1.0 } else { 0.0 };

        if (((((scratch.values[1323] != 0.0) && (scratch.values[1340] != 0.0)) && (!(scratch.values[1820] != 0.0))) && (!(scratch.values[1831] != 0.0))) && (scratch.values[1832] != 0.0)) {
            scratch.store_ad(1359, &AdValue::sqrt(AdValue::mul(AdValue::sub(scratch.ad_value(539), scratch.ad_value(1357)), scratch.ad_value(621))));
        }

        if (((((scratch.values[1323] != 0.0) && (scratch.values[1340] != 0.0)) && (!(scratch.values[1820] != 0.0))) && (!(scratch.values[1831] != 0.0))) && (!(scratch.values[1832] != 0.0))) {
            scratch.store_ad(1359, &AdValue::pow(AdValue::mul(AdValue::sub(scratch.ad_value(539), scratch.ad_value(1357)), scratch.ad_value(621)), scratch.ad_value(542)));
        }

        if ((((scratch.values[1323] != 0.0) && (scratch.values[1340] != 0.0)) && (!(scratch.values[1820] != 0.0))) && (!(scratch.values[1831] != 0.0))) {
            scratch.store_ad(1384, &AdValue::mul(scratch.ad_value(603), AdValue::div(AdValue::mul(AdValue::sub(scratch.ad_value(539), scratch.ad_value(1357)), scratch.ad_value(618)), scratch.ad_value(1359))));
        }

        scratch.values[1833] = if (((((-scratch.values[633]) / scratch.values[1384])) as f64).abs() < 230.25850929940458) { 1.0 } else { 0.0 };

        if (((((scratch.values[1323] != 0.0) && (scratch.values[1340] != 0.0)) && (!(scratch.values[1820] != 0.0))) && (!(scratch.values[1831] != 0.0))) && (scratch.values[1833] != 0.0)) {
            scratch.store_ad(1359, &AdValue::exp(AdValue::div(AdValue::neg(scratch.ad_value(633)), scratch.ad_value(1384))));
        }

        scratch.values[1834] = if (((-scratch.values[633]) / scratch.values[1384]) < 0.0) { 1.0 } else { 0.0 };

        if ((((((scratch.values[1323] != 0.0) && (scratch.values[1340] != 0.0)) && (!(scratch.values[1820] != 0.0))) && (!(scratch.values[1831] != 0.0))) && (!(scratch.values[1833] != 0.0))) && (scratch.values[1834] != 0.0)) {
            let assign33020_ad_e41216: AdValue = AdValue::offset(AdValue::mul(AdValue::sub_from_scalar((-230.25850929940458), AdValue::div(AdValue::neg(scratch.ad_value(633)), scratch.ad_value(1384))), AdValue::offset(AdValue::scale(AdValue::mul(AdValue::sub_from_scalar((-230.25850929940458), AdValue::div(AdValue::neg(scratch.ad_value(633)), scratch.ad_value(1384))), AdValue::offset(AdValue::scale(AdValue::sub_from_scalar((-230.25850929940458), AdValue::div(AdValue::neg(scratch.ad_value(633)), scratch.ad_value(1384))), 0.3333333333333333), 1.0)), 0.5), 1.0)), 1.0);
            scratch.store_ad(1359, &AdValue::div_from_scalar(1e-100, assign33020_ad_e41216));
        }

        if ((((((scratch.values[1323] != 0.0) && (scratch.values[1340] != 0.0)) && (!(scratch.values[1820] != 0.0))) && (!(scratch.values[1831] != 0.0))) && (!(scratch.values[1833] != 0.0))) && (!(scratch.values[1834] != 0.0))) {
            let assign33030_ad_e41266: AdValue = AdValue::scale(AdValue::offset(AdValue::mul(AdValue::offset(AdValue::div(AdValue::neg(scratch.ad_value(633)), scratch.ad_value(1384)), (-230.25850929940458)), AdValue::offset(AdValue::scale(AdValue::mul(AdValue::offset(AdValue::div(AdValue::neg(scratch.ad_value(633)), scratch.ad_value(1384)), (-230.25850929940458)), AdValue::offset(AdValue::scale(AdValue::offset(AdValue::div(AdValue::neg(scratch.ad_value(633)), scratch.ad_value(1384)), (-230.25850929940458)), 0.3333333333333333), 1.0)), 0.5), 1.0)), 1.0), 1e100);
            scratch.store_ad(1359, &assign33030_ad_e41266);
        }

        if ((((scratch.values[1323] != 0.0) && (scratch.values[1340] != 0.0)) && (!(scratch.values[1820] != 0.0))) && (!(scratch.values[1831] != 0.0))) {
            scratch.store_ad(1383, &AdValue::mul(scratch.ad_value(562), AdValue::mul(AdValue::mul(AdValue::mul(scratch.ad_value(518), scratch.ad_value(1384)), scratch.ad_value(1384)), scratch.ad_value(1359))));
        }

        scratch.values[1835] = if (scratch.values[571] > 1000.0) { 1.0 } else { 0.0 };

        if ((((scratch.values[1323] != 0.0) && (scratch.values[1340] != 0.0)) && (!(scratch.values[1820] != 0.0))) && (scratch.values[1835] != 0.0)) {
            scratch.values[1385] = 1.0;
            scratch.node_derivatives[1385] = [0.0; Instance::NODE_COUNT];
            scratch.branch_derivatives[1385] = [0.0; Instance::BRANCH_COUNT];
        }

        scratch.values[1836] = if (scratch.values[1358] > ((-scratch.values[493]) * scratch.values[571])) { 1.0 } else { 0.0 };

        scratch.values[1837] = if (scratch.values[574] == 4.0) { 1.0 } else { 0.0 };

        if ((((((scratch.values[1323] != 0.0) && (scratch.values[1340] != 0.0)) && (!(scratch.values[1820] != 0.0))) && (!(scratch.values[1835] != 0.0))) && (scratch.values[1836] != 0.0)) && (scratch.values[1837] != 0.0)) {
            scratch.store_ad(1359, &AdValue::mul(AdValue::mul(AdValue::mul(AdValue::mul(scratch.ad_value(1358), scratch.ad_value(639)), AdValue::mul(scratch.ad_value(1358), scratch.ad_value(639))), AdValue::mul(scratch.ad_value(1358), scratch.ad_value(639))), AdValue::mul(scratch.ad_value(1358), scratch.ad_value(639))));
        }

        if ((((((scratch.values[1323] != 0.0) && (scratch.values[1340] != 0.0)) && (!(scratch.values[1820] != 0.0))) && (!(scratch.values[1835] != 0.0))) && (scratch.values[1836] != 0.0)) && (!(scratch.values[1837] != 0.0))) {
            scratch.store_ad(1359, &AdValue::pow(AdValue::abs(AdValue::mul(scratch.ad_value(1358), scratch.ad_value(639))), scratch.ad_value(574)));
        }

        if (((((scratch.values[1323] != 0.0) && (scratch.values[1340] != 0.0)) && (!(scratch.values[1820] != 0.0))) && (!(scratch.values[1835] != 0.0))) && (scratch.values[1836] != 0.0)) {
            scratch.store_ad(1385, &AdValue::div_from_scalar(1.0, AdValue::sub_from_scalar(1.0, scratch.ad_value(1359))));
        }

        if (((((scratch.values[1323] != 0.0) && (scratch.values[1340] != 0.0)) && (!(scratch.values[1820] != 0.0))) && (!(scratch.values[1835] != 0.0))) && (!(scratch.values[1836] != 0.0))) {
            scratch.store_ad(1385, &AdValue::add(scratch.ad_value(636), AdValue::mul(AdValue::add(scratch.ad_value(1358), AdValue::scale(scratch.ad_value(571), scratch.values[493])), scratch.ad_value(642))));
        }

        if (((scratch.values[1323] != 0.0) && (scratch.values[1340] != 0.0)) && (!(scratch.values[1820] != 0.0))) {
            scratch.store_ad(1386, &AdValue::mul(AdValue::add(AdValue::add(AdValue::add(scratch.ad_value(1360), scratch.ad_value(1361)), scratch.ad_value(1368)), scratch.ad_value(1383)), scratch.ad_value(1385)));
        }

        scratch.values[1838] = if (scratch.values[718] == 0.0) { 1.0 } else { 0.0 };

        if (((scratch.values[1323] != 0.0) && (scratch.values[1340] != 0.0)) && (scratch.values[1838] != 0.0)) {
            scratch.values[1387] = 0.0;
            scratch.node_derivatives[1387] = [0.0; Instance::NODE_COUNT];
            scratch.branch_derivatives[1387] = [0.0; Instance::BRANCH_COUNT];
        }

        scratch.values[1839] = if (scratch.values[601] == 0.5) { 1.0 } else { 0.0 };

        if ((((scratch.values[1323] != 0.0) && (scratch.values[1340] != 0.0)) && (!(scratch.values[1838] != 0.0))) && (scratch.values[1839] != 0.0)) {
            scratch.store_ad(1359, &AdValue::sqrt(AdValue::sub_from_scalar(1.0, AdValue::mul(scratch.ad_value(1351), scratch.ad_value(598)))));
        }

        if ((((scratch.values[1323] != 0.0) && (scratch.values[1340] != 0.0)) && (!(scratch.values[1838] != 0.0))) && (!(scratch.values[1839] != 0.0))) {
            scratch.store_ad(1359, &AdValue::pow(AdValue::sub_from_scalar(1.0, AdValue::mul(scratch.ad_value(1351), scratch.ad_value(598))), scratch.ad_value(601)));
        }

        if (((scratch.values[1323] != 0.0) && (scratch.values[1340] != 0.0)) && (!(scratch.values[1838] != 0.0))) {
            scratch.store_ad(1360, &AdValue::mul(scratch.ad_value(589), scratch.ad_value(1350)));
        }

        scratch.values[1840] = if ((scratch.values[554] == 0.0) && (scratch.values[557] == 0.0)) { 1.0 } else { 0.0 };

        if ((((scratch.values[1323] != 0.0) && (scratch.values[1340] != 0.0)) && (!(scratch.values[1838] != 0.0))) && (scratch.values[1840] != 0.0)) {
            scratch.values[1361] = 0.0;
            scratch.node_derivatives[1361] = [0.0; Instance::NODE_COUNT];
            scratch.branch_derivatives[1361] = [0.0; Instance::BRANCH_COUNT];
        }

        if ((((scratch.values[1323] != 0.0) && (scratch.values[1340] != 0.0)) && (!(scratch.values[1838] != 0.0))) && (!(scratch.values[1840] != 0.0))) {
            scratch.store_ad(1362, &AdValue::sub(scratch.ad_value(595), scratch.ad_value(1356)));
        }

        if ((((scratch.values[1323] != 0.0) && (scratch.values[1340] != 0.0)) && (!(scratch.values[1838] != 0.0))) && (!(scratch.values[1840] != 0.0))) {
            scratch.store_ad(1363, &AdValue::sub_from_scalar(1.0, AdValue::sqrt(AdValue::sub_from_scalar(1.0, AdValue::div(scratch.ad_value(1354), scratch.ad_value(1362))))));
        }

        scratch.values[1841] = if (scratch.values[543] == 0.5) { 1.0 } else { 0.0 };

        if (((((scratch.values[1323] != 0.0) && (scratch.values[1340] != 0.0)) && (!(scratch.values[1838] != 0.0))) && (!(scratch.values[1840] != 0.0))) && (scratch.values[1841] != 0.0)) {
            scratch.values[1364] = 0.0;
            scratch.node_derivatives[1364] = [0.0; Instance::NODE_COUNT];
            scratch.branch_derivatives[1364] = [0.0; Instance::BRANCH_COUNT];
        }

        if (((((scratch.values[1323] != 0.0) && (scratch.values[1340] != 0.0)) && (!(scratch.values[1838] != 0.0))) && (!(scratch.values[1840] != 0.0))) && (!(scratch.values[1841] != 0.0))) {
            scratch.store_ad(1364, &AdValue::mul(AdValue::add(AdValue::div(AdValue::mul(AdValue::square(scratch.ad_value(1363)), AdValue::ln(scratch.ad_value(1363))), AdValue::sub_from_scalar(1.0, scratch.ad_value(1363))), scratch.ad_value(1363)), AdValue::sub_from_scalar(1.0, AdValue::scale(scratch.ad_value(543), 2.0))));
        }

        if ((((scratch.values[1323] != 0.0) && (scratch.values[1340] != 0.0)) && (!(scratch.values[1838] != 0.0))) && (!(scratch.values[1840] != 0.0))) {
            scratch.store_ad(1365, &AdValue::add(scratch.ad_value(1363), scratch.ad_value(1364)));
        }

        scratch.values[1842] = if (scratch.values[543] == 0.5) { 1.0 } else { 0.0 };

        if (((((scratch.values[1323] != 0.0) && (scratch.values[1340] != 0.0)) && (!(scratch.values[1838] != 0.0))) && (!(scratch.values[1840] != 0.0))) && (scratch.values[1842] != 0.0)) {
            scratch.store_ad(1359, &AdValue::sqrt(AdValue::mul(scratch.ad_value(1362), scratch.ad_value(622))));
        }

        if (((((scratch.values[1323] != 0.0) && (scratch.values[1340] != 0.0)) && (!(scratch.values[1838] != 0.0))) && (!(scratch.values[1840] != 0.0))) && (!(scratch.values[1842] != 0.0))) {
            scratch.store_ad(1359, &AdValue::pow(AdValue::mul(scratch.ad_value(1362), scratch.ad_value(622)), scratch.ad_value(543)));
        }

        if ((((scratch.values[1323] != 0.0) && (scratch.values[1340] != 0.0)) && (!(scratch.values[1838] != 0.0))) && (!(scratch.values[1840] != 0.0))) {
            scratch.store_ad(1366, &AdValue::mul(scratch.ad_value(616), scratch.ad_value(1359)));
        }

        if ((((scratch.values[1323] != 0.0) && (scratch.values[1340] != 0.0)) && (!(scratch.values[1838] != 0.0))) && (!(scratch.values[1840] != 0.0))) {
            scratch.store_ad(1367, &AdValue::mul(scratch.ad_value(586), AdValue::mul(AdValue::offset(scratch.ad_value(1353), (-1.0)), scratch.ad_value(1366))));
        }

        if ((((scratch.values[1323] != 0.0) && (scratch.values[1340] != 0.0)) && (!(scratch.values[1838] != 0.0))) && (!(scratch.values[1840] != 0.0))) {
            scratch.store_ad(1361, &AdValue::mul(scratch.ad_value(554), AdValue::mul(scratch.ad_value(1367), scratch.ad_value(1365))));
        }

        scratch.values[1843] = if (scratch.values[557] == 0.0) { 1.0 } else { 0.0 };

        if ((((scratch.values[1323] != 0.0) && (scratch.values[1340] != 0.0)) && (!(scratch.values[1838] != 0.0))) && (scratch.values[1843] != 0.0)) {
            scratch.values[1368] = 0.0;
            scratch.node_derivatives[1368] = [0.0; Instance::NODE_COUNT];
            scratch.branch_derivatives[1368] = [0.0; Instance::BRANCH_COUNT];
        }

        if ((((scratch.values[1323] != 0.0) && (scratch.values[1340] != 0.0)) && (!(scratch.values[1838] != 0.0))) && (!(scratch.values[1843] != 0.0))) {
            scratch.store_ad(1369, &AdValue::mul(scratch.ad_value(631), AdValue::div(AdValue::mul(scratch.ad_value(1366), scratch.ad_value(601)), scratch.ad_value(1362))));
        }

        if ((((scratch.values[1323] != 0.0) && (scratch.values[1340] != 0.0)) && (!(scratch.values[1838] != 0.0))) && (!(scratch.values[1843] != 0.0))) {
            scratch.store_ad(1370, &AdValue::div(AdValue::scale(scratch.ad_value(628), 0.666666666666667), scratch.ad_value(1369)));
        }

        if ((((scratch.values[1323] != 0.0) && (scratch.values[1340] != 0.0)) && (!(scratch.values[1838] != 0.0))) && (!(scratch.values[1843] != 0.0))) {
            scratch.store_ad(1371, &AdValue::square(scratch.ad_value(1370)));
        }

        if ((((scratch.values[1323] != 0.0) && (scratch.values[1340] != 0.0)) && (!(scratch.values[1838] != 0.0))) && (!(scratch.values[1843] != 0.0))) {
            scratch.store_ad(1372, &AdValue::sqrt(AdValue::div(AdValue::square(scratch.ad_value(1371)), AdValue::offset(AdValue::square(scratch.ad_value(1371)), 1.0))));
        }

        if ((((scratch.values[1323] != 0.0) && (scratch.values[1340] != 0.0)) && (!(scratch.values[1838] != 0.0))) && (!(scratch.values[1843] != 0.0))) {
            scratch.store_ad(1373, &AdValue::sqrt(AdValue::abs(scratch.ad_value(1372))));
        }

        if ((((scratch.values[1323] != 0.0) && (scratch.values[1340] != 0.0)) && (!(scratch.values[1838] != 0.0))) && (!(scratch.values[1843] != 0.0))) {
            scratch.store_ad(1374, &AdValue::mul(scratch.ad_value(1372), scratch.ad_value(1373)));
        }

        scratch.values[1844] = if (((-scratch.values[543]) * scratch.values[604]) == (-1.0)) { 1.0 } else { 0.0 };

        if (((((scratch.values[1323] != 0.0) && (scratch.values[1340] != 0.0)) && (!(scratch.values[1838] != 0.0))) && (!(scratch.values[1843] != 0.0))) && (scratch.values[1844] != 0.0)) {
            scratch.store_ad(1375, &AdValue::div_from_scalar(1.0, AdValue::offset(AdValue::mul(scratch.ad_value(1369), scratch.ad_value(1374)), 1.0)));
        }

        if (((((scratch.values[1323] != 0.0) && (scratch.values[1340] != 0.0)) && (!(scratch.values[1838] != 0.0))) && (!(scratch.values[1843] != 0.0))) && (!(scratch.values[1844] != 0.0))) {
            scratch.store_ad(1375, &AdValue::pow(AdValue::offset(AdValue::mul(scratch.ad_value(1369), scratch.ad_value(1374)), 1.0), AdValue::mul(AdValue::neg(scratch.ad_value(543)), scratch.ad_value(604))));
        }

        if ((((scratch.values[1323] != 0.0) && (scratch.values[1340] != 0.0)) && (!(scratch.values[1838] != 0.0))) && (!(scratch.values[1843] != 0.0))) {
            scratch.store_ad(1376, &AdValue::div(AdValue::mul(scratch.ad_value(1365), scratch.ad_value(1375)), AdValue::add(scratch.ad_value(1365), scratch.ad_value(1375))));
        }

        if ((((scratch.values[1323] != 0.0) && (scratch.values[1340] != 0.0)) && (!(scratch.values[1838] != 0.0))) && (!(scratch.values[1843] != 0.0))) {
            scratch.store_ad(1377, &AdValue::sqrt(AdValue::scale(AdValue::div(scratch.ad_value(1369), scratch.ad_value(1373)), 0.375)));
        }

        if ((((scratch.values[1323] != 0.0) && (scratch.values[1340] != 0.0)) && (!(scratch.values[1838] != 0.0))) && (!(scratch.values[1843] != 0.0))) {
            scratch.store_ad(1378, &AdValue::sub(AdValue::scale(AdValue::mul(scratch.ad_value(1370), scratch.ad_value(1373)), 2.0), scratch.ad_value(1372)));
        }

        if ((((scratch.values[1323] != 0.0) && (scratch.values[1340] != 0.0)) && (!(scratch.values[1838] != 0.0))) && (!(scratch.values[1843] != 0.0))) {
            scratch.store_ad(1379, &AdValue::add(AdValue::sub(AdValue::mul(AdValue::mul(scratch.ad_value(628), scratch.ad_value(1370)), scratch.ad_value(1373)), AdValue::mul(scratch.ad_value(628), scratch.ad_value(1372))), AdValue::scale(AdValue::mul(scratch.ad_value(1369), scratch.ad_value(1374)), 0.5)));
        }

        if ((((scratch.values[1323] != 0.0) && (scratch.values[1340] != 0.0)) && (!(scratch.values[1838] != 0.0))) && (!(scratch.values[1843] != 0.0))) {
            scratch.store_ad(1380, &AdValue::mul(AdValue::offset(scratch.ad_value(1378), (-1.0)), scratch.ad_value(1377)));
        }

        if ((((scratch.values[1323] != 0.0) && (scratch.values[1340] != 0.0)) && (!(scratch.values[1838] != 0.0))) && (!(scratch.values[1843] != 0.0))) {
            scratch.store_ad(1341, &AdValue::square(scratch.ad_value(1380)));
        }

        scratch.values[1845] = if (scratch.values[1380] > 0.0) { 1.0 } else { 0.0 };

        if (((((scratch.values[1323] != 0.0) && (scratch.values[1340] != 0.0)) && (!(scratch.values[1838] != 0.0))) && (!(scratch.values[1843] != 0.0))) && (scratch.values[1845] != 0.0)) {
            scratch.store_ad(1342, &AdValue::div_from_scalar(1.0, AdValue::offset(AdValue::scale(scratch.ad_value(1380), scratch.values[421]), 1.0)));
        }

        if (((((scratch.values[1323] != 0.0) && (scratch.values[1340] != 0.0)) && (!(scratch.values[1838] != 0.0))) && (!(scratch.values[1843] != 0.0))) && (!(scratch.values[1845] != 0.0))) {
            scratch.store_ad(1342, &AdValue::div_from_scalar(1.0, AdValue::sub_from_scalar(1.0, AdValue::scale(scratch.ad_value(1380), scratch.values[421]))));
        }

        scratch.values[1846] = if (((-scratch.values[1341]) + scratch.values[1379]) > (-230.25850929940458)) { 1.0 } else { 0.0 };

        if (((((scratch.values[1323] != 0.0) && (scratch.values[1340] != 0.0)) && (!(scratch.values[1838] != 0.0))) && (!(scratch.values[1843] != 0.0))) && (scratch.values[1846] != 0.0)) {
            scratch.store_ad(1359, &AdValue::exp(AdValue::sub(scratch.ad_value(1379), scratch.ad_value(1341))));
        }

        if (((((scratch.values[1323] != 0.0) && (scratch.values[1340] != 0.0)) && (!(scratch.values[1838] != 0.0))) && (!(scratch.values[1843] != 0.0))) && (!(scratch.values[1846] != 0.0))) {
            let assign33580_ad_e42095: AdValue = AdValue::div_from_scalar(1e-100, AdValue::offset(AdValue::mul(AdValue::sub_from_scalar((-230.25850929940458), AdValue::sub(scratch.ad_value(1379), scratch.ad_value(1341))), AdValue::offset(AdValue::scale(AdValue::mul(AdValue::sub_from_scalar((-230.25850929940458), AdValue::sub(scratch.ad_value(1379), scratch.ad_value(1341))), AdValue::offset(AdValue::scale(AdValue::sub_from_scalar((-230.25850929940458), AdValue::sub(scratch.ad_value(1379), scratch.ad_value(1341))), 0.3333333333333333), 1.0)), 0.5), 1.0)), 1.0));
            scratch.store_ad(1359, &assign33580_ad_e42095);
        }

        if ((((scratch.values[1323] != 0.0) && (scratch.values[1340] != 0.0)) && (!(scratch.values[1838] != 0.0))) && (!(scratch.values[1843] != 0.0))) {
            scratch.store_ad(1343, &AdValue::mul(AdValue::add(AdValue::add(AdValue::scale(scratch.ad_value(1342), 0.29214664), AdValue::scale(AdValue::square(scratch.ad_value(1342)), scratch.values[422])), AdValue::scale(AdValue::mul(AdValue::square(scratch.ad_value(1342)), scratch.ad_value(1342)), scratch.values[423])), scratch.ad_value(1359)));
        }

        scratch.values[1847] = if (scratch.values[1380] > 0.0) { 1.0 } else { 0.0 };

        if (((((scratch.values[1323] != 0.0) && (scratch.values[1340] != 0.0)) && (!(scratch.values[1838] != 0.0))) && (!(scratch.values[1843] != 0.0))) && (scratch.values[1847] != 0.0)) {
            scratch.values[1381] = scratch.values[1343];
            scratch.node_derivatives[1381] = scratch.node_derivatives[1343];
            scratch.branch_derivatives[1381] = scratch.branch_derivatives[1343];
        }

        scratch.values[1848] = if (scratch.values[1379] > (-230.25850929940458)) { 1.0 } else { 0.0 };

        if ((((((scratch.values[1323] != 0.0) && (scratch.values[1340] != 0.0)) && (!(scratch.values[1838] != 0.0))) && (!(scratch.values[1843] != 0.0))) && (!(scratch.values[1847] != 0.0))) && (scratch.values[1848] != 0.0)) {
            scratch.store_ad(1359, &AdValue::exp(scratch.ad_value(1379)));
        }

        if ((((((scratch.values[1323] != 0.0) && (scratch.values[1340] != 0.0)) && (!(scratch.values[1838] != 0.0))) && (!(scratch.values[1843] != 0.0))) && (!(scratch.values[1847] != 0.0))) && (!(scratch.values[1848] != 0.0))) {
            scratch.store_ad(1359, &AdValue::div_from_scalar(1e-100, AdValue::offset(AdValue::mul(AdValue::sub_from_scalar((-230.25850929940458), scratch.ad_value(1379)), AdValue::offset(AdValue::scale(AdValue::mul(AdValue::sub_from_scalar((-230.25850929940458), scratch.ad_value(1379)), AdValue::offset(AdValue::scale(AdValue::sub_from_scalar((-230.25850929940458), scratch.ad_value(1379)), 0.3333333333333333), 1.0)), 0.5), 1.0)), 1.0)));
        }

        if (((((scratch.values[1323] != 0.0) && (scratch.values[1340] != 0.0)) && (!(scratch.values[1838] != 0.0))) && (!(scratch.values[1843] != 0.0))) && (!(scratch.values[1847] != 0.0))) {
            scratch.store_ad(1381, &AdValue::sub(AdValue::scale(scratch.ad_value(1359), 2.0), scratch.ad_value(1343)));
        }

        if ((((scratch.values[1323] != 0.0) && (scratch.values[1340] != 0.0)) && (!(scratch.values[1838] != 0.0))) && (!(scratch.values[1843] != 0.0))) {
            scratch.store_ad(1382, &AdValue::scale(AdValue::div(AdValue::mul(scratch.ad_value(628), scratch.ad_value(1381)), scratch.ad_value(1377)), (1.772453850905516 * 0.5)));
        }

        if ((((scratch.values[1323] != 0.0) && (scratch.values[1340] != 0.0)) && (!(scratch.values[1838] != 0.0))) && (!(scratch.values[1843] != 0.0))) {
            scratch.store_ad(1368, &AdValue::mul(scratch.ad_value(557), AdValue::mul(AdValue::mul(scratch.ad_value(1367), scratch.ad_value(1382)), scratch.ad_value(1376))));
        }

        scratch.values[1849] = if (scratch.values[563] == 0.0) { 1.0 } else { 0.0 };

        if ((((scratch.values[1323] != 0.0) && (scratch.values[1340] != 0.0)) && (!(scratch.values[1838] != 0.0))) && (scratch.values[1849] != 0.0)) {
            scratch.values[1383] = 0.0;
            scratch.node_derivatives[1383] = [0.0; Instance::NODE_COUNT];
            scratch.branch_derivatives[1383] = [0.0; Instance::BRANCH_COUNT];
        }

        scratch.values[1850] = if (scratch.values[543] == 0.5) { 1.0 } else { 0.0 };

        if (((((scratch.values[1323] != 0.0) && (scratch.values[1340] != 0.0)) && (!(scratch.values[1838] != 0.0))) && (!(scratch.values[1849] != 0.0))) && (scratch.values[1850] != 0.0)) {
            scratch.store_ad(1359, &AdValue::sqrt(AdValue::mul(AdValue::sub(scratch.ad_value(540), scratch.ad_value(1357)), scratch.ad_value(622))));
        }

        if (((((scratch.values[1323] != 0.0) && (scratch.values[1340] != 0.0)) && (!(scratch.values[1838] != 0.0))) && (!(scratch.values[1849] != 0.0))) && (!(scratch.values[1850] != 0.0))) {
            scratch.store_ad(1359, &AdValue::pow(AdValue::mul(AdValue::sub(scratch.ad_value(540), scratch.ad_value(1357)), scratch.ad_value(622)), scratch.ad_value(543)));
        }

        if ((((scratch.values[1323] != 0.0) && (scratch.values[1340] != 0.0)) && (!(scratch.values[1838] != 0.0))) && (!(scratch.values[1849] != 0.0))) {
            scratch.store_ad(1384, &AdValue::mul(scratch.ad_value(604), AdValue::div(AdValue::mul(AdValue::sub(scratch.ad_value(540), scratch.ad_value(1357)), scratch.ad_value(619)), scratch.ad_value(1359))));
        }

        scratch.values[1851] = if (((((-scratch.values[634]) / scratch.values[1384])) as f64).abs() < 230.25850929940458) { 1.0 } else { 0.0 };

        if (((((scratch.values[1323] != 0.0) && (scratch.values[1340] != 0.0)) && (!(scratch.values[1838] != 0.0))) && (!(scratch.values[1849] != 0.0))) && (scratch.values[1851] != 0.0)) {
            scratch.store_ad(1359, &AdValue::exp(AdValue::div(AdValue::neg(scratch.ad_value(634)), scratch.ad_value(1384))));
        }

        scratch.values[1852] = if (((-scratch.values[634]) / scratch.values[1384]) < 0.0) { 1.0 } else { 0.0 };

        if ((((((scratch.values[1323] != 0.0) && (scratch.values[1340] != 0.0)) && (!(scratch.values[1838] != 0.0))) && (!(scratch.values[1849] != 0.0))) && (!(scratch.values[1851] != 0.0))) && (scratch.values[1852] != 0.0)) {
            let assign33770_ad_e42422: AdValue = AdValue::offset(AdValue::mul(AdValue::sub_from_scalar((-230.25850929940458), AdValue::div(AdValue::neg(scratch.ad_value(634)), scratch.ad_value(1384))), AdValue::offset(AdValue::scale(AdValue::mul(AdValue::sub_from_scalar((-230.25850929940458), AdValue::div(AdValue::neg(scratch.ad_value(634)), scratch.ad_value(1384))), AdValue::offset(AdValue::scale(AdValue::sub_from_scalar((-230.25850929940458), AdValue::div(AdValue::neg(scratch.ad_value(634)), scratch.ad_value(1384))), 0.3333333333333333), 1.0)), 0.5), 1.0)), 1.0);
            scratch.store_ad(1359, &AdValue::div_from_scalar(1e-100, assign33770_ad_e42422));
        }

        if ((((((scratch.values[1323] != 0.0) && (scratch.values[1340] != 0.0)) && (!(scratch.values[1838] != 0.0))) && (!(scratch.values[1849] != 0.0))) && (!(scratch.values[1851] != 0.0))) && (!(scratch.values[1852] != 0.0))) {
            let assign33780_ad_e42472: AdValue = AdValue::scale(AdValue::offset(AdValue::mul(AdValue::offset(AdValue::div(AdValue::neg(scratch.ad_value(634)), scratch.ad_value(1384)), (-230.25850929940458)), AdValue::offset(AdValue::scale(AdValue::mul(AdValue::offset(AdValue::div(AdValue::neg(scratch.ad_value(634)), scratch.ad_value(1384)), (-230.25850929940458)), AdValue::offset(AdValue::scale(AdValue::offset(AdValue::div(AdValue::neg(scratch.ad_value(634)), scratch.ad_value(1384)), (-230.25850929940458)), 0.3333333333333333), 1.0)), 0.5), 1.0)), 1.0), 1e100);
            scratch.store_ad(1359, &assign33780_ad_e42472);
        }

        if ((((scratch.values[1323] != 0.0) && (scratch.values[1340] != 0.0)) && (!(scratch.values[1838] != 0.0))) && (!(scratch.values[1849] != 0.0))) {
            scratch.store_ad(1383, &AdValue::mul(scratch.ad_value(563), AdValue::mul(AdValue::mul(AdValue::mul(scratch.ad_value(518), scratch.ad_value(1384)), scratch.ad_value(1384)), scratch.ad_value(1359))));
        }

        scratch.values[1853] = if (scratch.values[572] > 1000.0) { 1.0 } else { 0.0 };

        if ((((scratch.values[1323] != 0.0) && (scratch.values[1340] != 0.0)) && (!(scratch.values[1838] != 0.0))) && (scratch.values[1853] != 0.0)) {
            scratch.values[1385] = 1.0;
            scratch.node_derivatives[1385] = [0.0; Instance::NODE_COUNT];
            scratch.branch_derivatives[1385] = [0.0; Instance::BRANCH_COUNT];
        }

        scratch.values[1854] = if (scratch.values[1358] > ((-scratch.values[493]) * scratch.values[572])) { 1.0 } else { 0.0 };

        scratch.values[1855] = if (scratch.values[575] == 4.0) { 1.0 } else { 0.0 };

        if ((((((scratch.values[1323] != 0.0) && (scratch.values[1340] != 0.0)) && (!(scratch.values[1838] != 0.0))) && (!(scratch.values[1853] != 0.0))) && (scratch.values[1854] != 0.0)) && (scratch.values[1855] != 0.0)) {
            scratch.store_ad(1359, &AdValue::mul(AdValue::mul(AdValue::mul(AdValue::mul(scratch.ad_value(1358), scratch.ad_value(640)), AdValue::mul(scratch.ad_value(1358), scratch.ad_value(640))), AdValue::mul(scratch.ad_value(1358), scratch.ad_value(640))), AdValue::mul(scratch.ad_value(1358), scratch.ad_value(640))));
        }

        if ((((((scratch.values[1323] != 0.0) && (scratch.values[1340] != 0.0)) && (!(scratch.values[1838] != 0.0))) && (!(scratch.values[1853] != 0.0))) && (scratch.values[1854] != 0.0)) && (!(scratch.values[1855] != 0.0))) {
            scratch.store_ad(1359, &AdValue::pow(AdValue::abs(AdValue::mul(scratch.ad_value(1358), scratch.ad_value(640))), scratch.ad_value(575)));
        }

        if (((((scratch.values[1323] != 0.0) && (scratch.values[1340] != 0.0)) && (!(scratch.values[1838] != 0.0))) && (!(scratch.values[1853] != 0.0))) && (scratch.values[1854] != 0.0)) {
            scratch.store_ad(1385, &AdValue::div_from_scalar(1.0, AdValue::sub_from_scalar(1.0, scratch.ad_value(1359))));
        }

        if (((((scratch.values[1323] != 0.0) && (scratch.values[1340] != 0.0)) && (!(scratch.values[1838] != 0.0))) && (!(scratch.values[1853] != 0.0))) && (!(scratch.values[1854] != 0.0))) {
            scratch.store_ad(1385, &AdValue::add(scratch.ad_value(637), AdValue::mul(AdValue::add(scratch.ad_value(1358), AdValue::scale(scratch.ad_value(572), scratch.values[493])), scratch.ad_value(643))));
        }

        if (((scratch.values[1323] != 0.0) && (scratch.values[1340] != 0.0)) && (!(scratch.values[1838] != 0.0))) {
            scratch.store_ad(1387, &AdValue::mul(AdValue::add(AdValue::add(AdValue::add(scratch.ad_value(1360), scratch.ad_value(1361)), scratch.ad_value(1368)), scratch.ad_value(1383)), scratch.ad_value(1385)));
        }

        scratch.values[1856] = if (scratch.values[719] == 0.0) { 1.0 } else { 0.0 };

        if (((scratch.values[1323] != 0.0) && (scratch.values[1340] != 0.0)) && (scratch.values[1856] != 0.0)) {
            scratch.values[1388] = 0.0;
            scratch.node_derivatives[1388] = [0.0; Instance::NODE_COUNT];
            scratch.branch_derivatives[1388] = [0.0; Instance::BRANCH_COUNT];
        }

        scratch.values[1857] = if (scratch.values[602] == 0.5) { 1.0 } else { 0.0 };

        if ((((scratch.values[1323] != 0.0) && (scratch.values[1340] != 0.0)) && (!(scratch.values[1856] != 0.0))) && (scratch.values[1857] != 0.0)) {
            scratch.store_ad(1359, &AdValue::sqrt(AdValue::sub_from_scalar(1.0, AdValue::mul(scratch.ad_value(1351), scratch.ad_value(599)))));
        }

        if ((((scratch.values[1323] != 0.0) && (scratch.values[1340] != 0.0)) && (!(scratch.values[1856] != 0.0))) && (!(scratch.values[1857] != 0.0))) {
            scratch.store_ad(1359, &AdValue::pow(AdValue::sub_from_scalar(1.0, AdValue::mul(scratch.ad_value(1351), scratch.ad_value(599))), scratch.ad_value(602)));
        }

        if (((scratch.values[1323] != 0.0) && (scratch.values[1340] != 0.0)) && (!(scratch.values[1856] != 0.0))) {
            scratch.store_ad(1360, &AdValue::mul(scratch.ad_value(590), scratch.ad_value(1350)));
        }

        scratch.values[1858] = if ((scratch.values[555] == 0.0) && (scratch.values[558] == 0.0)) { 1.0 } else { 0.0 };

        if ((((scratch.values[1323] != 0.0) && (scratch.values[1340] != 0.0)) && (!(scratch.values[1856] != 0.0))) && (scratch.values[1858] != 0.0)) {
            scratch.values[1361] = 0.0;
            scratch.node_derivatives[1361] = [0.0; Instance::NODE_COUNT];
            scratch.branch_derivatives[1361] = [0.0; Instance::BRANCH_COUNT];
        }

        if ((((scratch.values[1323] != 0.0) && (scratch.values[1340] != 0.0)) && (!(scratch.values[1856] != 0.0))) && (!(scratch.values[1858] != 0.0))) {
            scratch.store_ad(1362, &AdValue::sub(scratch.ad_value(596), scratch.ad_value(1356)));
        }

        if ((((scratch.values[1323] != 0.0) && (scratch.values[1340] != 0.0)) && (!(scratch.values[1856] != 0.0))) && (!(scratch.values[1858] != 0.0))) {
            scratch.store_ad(1363, &AdValue::sub_from_scalar(1.0, AdValue::sqrt(AdValue::sub_from_scalar(1.0, AdValue::div(scratch.ad_value(1354), scratch.ad_value(1362))))));
        }

        scratch.values[1859] = if (scratch.values[544] == 0.5) { 1.0 } else { 0.0 };

        if (((((scratch.values[1323] != 0.0) && (scratch.values[1340] != 0.0)) && (!(scratch.values[1856] != 0.0))) && (!(scratch.values[1858] != 0.0))) && (scratch.values[1859] != 0.0)) {
            scratch.values[1364] = 0.0;
            scratch.node_derivatives[1364] = [0.0; Instance::NODE_COUNT];
            scratch.branch_derivatives[1364] = [0.0; Instance::BRANCH_COUNT];
        }

        if (((((scratch.values[1323] != 0.0) && (scratch.values[1340] != 0.0)) && (!(scratch.values[1856] != 0.0))) && (!(scratch.values[1858] != 0.0))) && (!(scratch.values[1859] != 0.0))) {
            scratch.store_ad(1364, &AdValue::mul(AdValue::add(AdValue::div(AdValue::mul(AdValue::square(scratch.ad_value(1363)), AdValue::ln(scratch.ad_value(1363))), AdValue::sub_from_scalar(1.0, scratch.ad_value(1363))), scratch.ad_value(1363)), AdValue::sub_from_scalar(1.0, AdValue::scale(scratch.ad_value(544), 2.0))));
        }

        if ((((scratch.values[1323] != 0.0) && (scratch.values[1340] != 0.0)) && (!(scratch.values[1856] != 0.0))) && (!(scratch.values[1858] != 0.0))) {
            scratch.store_ad(1365, &AdValue::add(scratch.ad_value(1363), scratch.ad_value(1364)));
        }

        scratch.values[1860] = if (scratch.values[544] == 0.5) { 1.0 } else { 0.0 };

        if (((((scratch.values[1323] != 0.0) && (scratch.values[1340] != 0.0)) && (!(scratch.values[1856] != 0.0))) && (!(scratch.values[1858] != 0.0))) && (scratch.values[1860] != 0.0)) {
            scratch.store_ad(1359, &AdValue::sqrt(AdValue::mul(scratch.ad_value(1362), scratch.ad_value(623))));
        }

        if (((((scratch.values[1323] != 0.0) && (scratch.values[1340] != 0.0)) && (!(scratch.values[1856] != 0.0))) && (!(scratch.values[1858] != 0.0))) && (!(scratch.values[1860] != 0.0))) {
            scratch.store_ad(1359, &AdValue::pow(AdValue::mul(scratch.ad_value(1362), scratch.ad_value(623)), scratch.ad_value(544)));
        }

        if ((((scratch.values[1323] != 0.0) && (scratch.values[1340] != 0.0)) && (!(scratch.values[1856] != 0.0))) && (!(scratch.values[1858] != 0.0))) {
            scratch.store_ad(1366, &AdValue::mul(scratch.ad_value(617), scratch.ad_value(1359)));
        }

        if ((((scratch.values[1323] != 0.0) && (scratch.values[1340] != 0.0)) && (!(scratch.values[1856] != 0.0))) && (!(scratch.values[1858] != 0.0))) {
            scratch.store_ad(1367, &AdValue::mul(scratch.ad_value(587), AdValue::mul(AdValue::offset(scratch.ad_value(1353), (-1.0)), scratch.ad_value(1366))));
        }

        if ((((scratch.values[1323] != 0.0) && (scratch.values[1340] != 0.0)) && (!(scratch.values[1856] != 0.0))) && (!(scratch.values[1858] != 0.0))) {
            scratch.store_ad(1361, &AdValue::mul(scratch.ad_value(555), AdValue::mul(scratch.ad_value(1367), scratch.ad_value(1365))));
        }

        scratch.values[1861] = if (scratch.values[558] == 0.0) { 1.0 } else { 0.0 };

        if ((((scratch.values[1323] != 0.0) && (scratch.values[1340] != 0.0)) && (!(scratch.values[1856] != 0.0))) && (scratch.values[1861] != 0.0)) {
            scratch.values[1368] = 0.0;
            scratch.node_derivatives[1368] = [0.0; Instance::NODE_COUNT];
            scratch.branch_derivatives[1368] = [0.0; Instance::BRANCH_COUNT];
        }

        if ((((scratch.values[1323] != 0.0) && (scratch.values[1340] != 0.0)) && (!(scratch.values[1856] != 0.0))) && (!(scratch.values[1861] != 0.0))) {
            scratch.store_ad(1369, &AdValue::mul(scratch.ad_value(632), AdValue::div(AdValue::mul(scratch.ad_value(1366), scratch.ad_value(602)), scratch.ad_value(1362))));
        }

        if ((((scratch.values[1323] != 0.0) && (scratch.values[1340] != 0.0)) && (!(scratch.values[1856] != 0.0))) && (!(scratch.values[1861] != 0.0))) {
            scratch.store_ad(1370, &AdValue::div(AdValue::scale(scratch.ad_value(629), 0.666666666666667), scratch.ad_value(1369)));
        }

        if ((((scratch.values[1323] != 0.0) && (scratch.values[1340] != 0.0)) && (!(scratch.values[1856] != 0.0))) && (!(scratch.values[1861] != 0.0))) {
            scratch.store_ad(1371, &AdValue::square(scratch.ad_value(1370)));
        }

        if ((((scratch.values[1323] != 0.0) && (scratch.values[1340] != 0.0)) && (!(scratch.values[1856] != 0.0))) && (!(scratch.values[1861] != 0.0))) {
            scratch.store_ad(1372, &AdValue::sqrt(AdValue::div(AdValue::square(scratch.ad_value(1371)), AdValue::offset(AdValue::square(scratch.ad_value(1371)), 1.0))));
        }

        if ((((scratch.values[1323] != 0.0) && (scratch.values[1340] != 0.0)) && (!(scratch.values[1856] != 0.0))) && (!(scratch.values[1861] != 0.0))) {
            scratch.store_ad(1373, &AdValue::sqrt(AdValue::abs(scratch.ad_value(1372))));
        }

        if ((((scratch.values[1323] != 0.0) && (scratch.values[1340] != 0.0)) && (!(scratch.values[1856] != 0.0))) && (!(scratch.values[1861] != 0.0))) {
            scratch.store_ad(1374, &AdValue::mul(scratch.ad_value(1372), scratch.ad_value(1373)));
        }

        scratch.values[1862] = if (((-scratch.values[544]) * scratch.values[605]) == (-1.0)) { 1.0 } else { 0.0 };

        if (((((scratch.values[1323] != 0.0) && (scratch.values[1340] != 0.0)) && (!(scratch.values[1856] != 0.0))) && (!(scratch.values[1861] != 0.0))) && (scratch.values[1862] != 0.0)) {
            scratch.store_ad(1375, &AdValue::div_from_scalar(1.0, AdValue::offset(AdValue::mul(scratch.ad_value(1369), scratch.ad_value(1374)), 1.0)));
        }

        if (((((scratch.values[1323] != 0.0) && (scratch.values[1340] != 0.0)) && (!(scratch.values[1856] != 0.0))) && (!(scratch.values[1861] != 0.0))) && (!(scratch.values[1862] != 0.0))) {
            scratch.store_ad(1375, &AdValue::pow(AdValue::offset(AdValue::mul(scratch.ad_value(1369), scratch.ad_value(1374)), 1.0), AdValue::mul(AdValue::neg(scratch.ad_value(544)), scratch.ad_value(605))));
        }

        if ((((scratch.values[1323] != 0.0) && (scratch.values[1340] != 0.0)) && (!(scratch.values[1856] != 0.0))) && (!(scratch.values[1861] != 0.0))) {
            scratch.store_ad(1376, &AdValue::div(AdValue::mul(scratch.ad_value(1365), scratch.ad_value(1375)), AdValue::add(scratch.ad_value(1365), scratch.ad_value(1375))));
        }

        if ((((scratch.values[1323] != 0.0) && (scratch.values[1340] != 0.0)) && (!(scratch.values[1856] != 0.0))) && (!(scratch.values[1861] != 0.0))) {
            scratch.store_ad(1377, &AdValue::sqrt(AdValue::scale(AdValue::div(scratch.ad_value(1369), scratch.ad_value(1373)), 0.375)));
        }

        if ((((scratch.values[1323] != 0.0) && (scratch.values[1340] != 0.0)) && (!(scratch.values[1856] != 0.0))) && (!(scratch.values[1861] != 0.0))) {
            scratch.store_ad(1378, &AdValue::sub(AdValue::scale(AdValue::mul(scratch.ad_value(1370), scratch.ad_value(1373)), 2.0), scratch.ad_value(1372)));
        }

        if ((((scratch.values[1323] != 0.0) && (scratch.values[1340] != 0.0)) && (!(scratch.values[1856] != 0.0))) && (!(scratch.values[1861] != 0.0))) {
            scratch.store_ad(1379, &AdValue::add(AdValue::sub(AdValue::mul(AdValue::mul(scratch.ad_value(629), scratch.ad_value(1370)), scratch.ad_value(1373)), AdValue::mul(scratch.ad_value(629), scratch.ad_value(1372))), AdValue::scale(AdValue::mul(scratch.ad_value(1369), scratch.ad_value(1374)), 0.5)));
        }

        if ((((scratch.values[1323] != 0.0) && (scratch.values[1340] != 0.0)) && (!(scratch.values[1856] != 0.0))) && (!(scratch.values[1861] != 0.0))) {
            scratch.store_ad(1380, &AdValue::mul(AdValue::offset(scratch.ad_value(1378), (-1.0)), scratch.ad_value(1377)));
        }

        if ((((scratch.values[1323] != 0.0) && (scratch.values[1340] != 0.0)) && (!(scratch.values[1856] != 0.0))) && (!(scratch.values[1861] != 0.0))) {
            scratch.store_ad(1341, &AdValue::square(scratch.ad_value(1380)));
        }

        scratch.values[1863] = if (scratch.values[1380] > 0.0) { 1.0 } else { 0.0 };

        if (((((scratch.values[1323] != 0.0) && (scratch.values[1340] != 0.0)) && (!(scratch.values[1856] != 0.0))) && (!(scratch.values[1861] != 0.0))) && (scratch.values[1863] != 0.0)) {
            scratch.store_ad(1342, &AdValue::div_from_scalar(1.0, AdValue::offset(AdValue::scale(scratch.ad_value(1380), scratch.values[421]), 1.0)));
        }

        if (((((scratch.values[1323] != 0.0) && (scratch.values[1340] != 0.0)) && (!(scratch.values[1856] != 0.0))) && (!(scratch.values[1861] != 0.0))) && (!(scratch.values[1863] != 0.0))) {
            scratch.store_ad(1342, &AdValue::div_from_scalar(1.0, AdValue::sub_from_scalar(1.0, AdValue::scale(scratch.ad_value(1380), scratch.values[421]))));
        }

        scratch.values[1864] = if (((-scratch.values[1341]) + scratch.values[1379]) > (-230.25850929940458)) { 1.0 } else { 0.0 };

        if (((((scratch.values[1323] != 0.0) && (scratch.values[1340] != 0.0)) && (!(scratch.values[1856] != 0.0))) && (!(scratch.values[1861] != 0.0))) && (scratch.values[1864] != 0.0)) {
            scratch.store_ad(1359, &AdValue::exp(AdValue::sub(scratch.ad_value(1379), scratch.ad_value(1341))));
        }

        if (((((scratch.values[1323] != 0.0) && (scratch.values[1340] != 0.0)) && (!(scratch.values[1856] != 0.0))) && (!(scratch.values[1861] != 0.0))) && (!(scratch.values[1864] != 0.0))) {
            let assign34330_ad_e43301: AdValue = AdValue::div_from_scalar(1e-100, AdValue::offset(AdValue::mul(AdValue::sub_from_scalar((-230.25850929940458), AdValue::sub(scratch.ad_value(1379), scratch.ad_value(1341))), AdValue::offset(AdValue::scale(AdValue::mul(AdValue::sub_from_scalar((-230.25850929940458), AdValue::sub(scratch.ad_value(1379), scratch.ad_value(1341))), AdValue::offset(AdValue::scale(AdValue::sub_from_scalar((-230.25850929940458), AdValue::sub(scratch.ad_value(1379), scratch.ad_value(1341))), 0.3333333333333333), 1.0)), 0.5), 1.0)), 1.0));
            scratch.store_ad(1359, &assign34330_ad_e43301);
        }

        if ((((scratch.values[1323] != 0.0) && (scratch.values[1340] != 0.0)) && (!(scratch.values[1856] != 0.0))) && (!(scratch.values[1861] != 0.0))) {
            scratch.store_ad(1343, &AdValue::mul(AdValue::add(AdValue::add(AdValue::scale(scratch.ad_value(1342), 0.29214664), AdValue::scale(AdValue::square(scratch.ad_value(1342)), scratch.values[422])), AdValue::scale(AdValue::mul(AdValue::square(scratch.ad_value(1342)), scratch.ad_value(1342)), scratch.values[423])), scratch.ad_value(1359)));
        }

        scratch.values[1865] = if (scratch.values[1380] > 0.0) { 1.0 } else { 0.0 };

    }

    pub(super) fn stamp_transient_block_26(
        &mut self,
        ctx: &GeneratedEvalContext<'_>,
        stamper: &mut GeneratedStamper<'_>,
        scratch: &mut Scratch,
    ) {
        let _ = stamper;
        if (((((scratch.values[1323] != 0.0) && (scratch.values[1340] != 0.0)) && (!(scratch.values[1856] != 0.0))) && (!(scratch.values[1861] != 0.0))) && (scratch.values[1865] != 0.0)) {
            scratch.values[1381] = scratch.values[1343];
            scratch.node_derivatives[1381] = scratch.node_derivatives[1343];
            scratch.branch_derivatives[1381] = scratch.branch_derivatives[1343];
        }

        scratch.values[1866] = if (scratch.values[1379] > (-230.25850929940458)) { 1.0 } else { 0.0 };

        if ((((((scratch.values[1323] != 0.0) && (scratch.values[1340] != 0.0)) && (!(scratch.values[1856] != 0.0))) && (!(scratch.values[1861] != 0.0))) && (!(scratch.values[1865] != 0.0))) && (scratch.values[1866] != 0.0)) {
            scratch.store_ad(1359, &AdValue::exp(scratch.ad_value(1379)));
        }

        if ((((((scratch.values[1323] != 0.0) && (scratch.values[1340] != 0.0)) && (!(scratch.values[1856] != 0.0))) && (!(scratch.values[1861] != 0.0))) && (!(scratch.values[1865] != 0.0))) && (!(scratch.values[1866] != 0.0))) {
            scratch.store_ad(1359, &AdValue::div_from_scalar(1e-100, AdValue::offset(AdValue::mul(AdValue::sub_from_scalar((-230.25850929940458), scratch.ad_value(1379)), AdValue::offset(AdValue::scale(AdValue::mul(AdValue::sub_from_scalar((-230.25850929940458), scratch.ad_value(1379)), AdValue::offset(AdValue::scale(AdValue::sub_from_scalar((-230.25850929940458), scratch.ad_value(1379)), 0.3333333333333333), 1.0)), 0.5), 1.0)), 1.0)));
        }

        if (((((scratch.values[1323] != 0.0) && (scratch.values[1340] != 0.0)) && (!(scratch.values[1856] != 0.0))) && (!(scratch.values[1861] != 0.0))) && (!(scratch.values[1865] != 0.0))) {
            scratch.store_ad(1381, &AdValue::sub(AdValue::scale(scratch.ad_value(1359), 2.0), scratch.ad_value(1343)));
        }

        if ((((scratch.values[1323] != 0.0) && (scratch.values[1340] != 0.0)) && (!(scratch.values[1856] != 0.0))) && (!(scratch.values[1861] != 0.0))) {
            scratch.store_ad(1382, &AdValue::scale(AdValue::div(AdValue::mul(scratch.ad_value(629), scratch.ad_value(1381)), scratch.ad_value(1377)), (1.772453850905516 * 0.5)));
        }

        if ((((scratch.values[1323] != 0.0) && (scratch.values[1340] != 0.0)) && (!(scratch.values[1856] != 0.0))) && (!(scratch.values[1861] != 0.0))) {
            scratch.store_ad(1368, &AdValue::mul(scratch.ad_value(558), AdValue::mul(AdValue::mul(scratch.ad_value(1367), scratch.ad_value(1382)), scratch.ad_value(1376))));
        }

        scratch.values[1867] = if (scratch.values[564] == 0.0) { 1.0 } else { 0.0 };

        if ((((scratch.values[1323] != 0.0) && (scratch.values[1340] != 0.0)) && (!(scratch.values[1856] != 0.0))) && (scratch.values[1867] != 0.0)) {
            scratch.values[1383] = 0.0;
            scratch.node_derivatives[1383] = [0.0; Instance::NODE_COUNT];
            scratch.branch_derivatives[1383] = [0.0; Instance::BRANCH_COUNT];
        }

        scratch.values[1868] = if (scratch.values[544] == 0.5) { 1.0 } else { 0.0 };

        if (((((scratch.values[1323] != 0.0) && (scratch.values[1340] != 0.0)) && (!(scratch.values[1856] != 0.0))) && (!(scratch.values[1867] != 0.0))) && (scratch.values[1868] != 0.0)) {
            scratch.store_ad(1359, &AdValue::sqrt(AdValue::mul(AdValue::sub(scratch.ad_value(541), scratch.ad_value(1357)), scratch.ad_value(623))));
        }

        if (((((scratch.values[1323] != 0.0) && (scratch.values[1340] != 0.0)) && (!(scratch.values[1856] != 0.0))) && (!(scratch.values[1867] != 0.0))) && (!(scratch.values[1868] != 0.0))) {
            scratch.store_ad(1359, &AdValue::pow(AdValue::mul(AdValue::sub(scratch.ad_value(541), scratch.ad_value(1357)), scratch.ad_value(623)), scratch.ad_value(544)));
        }

        if ((((scratch.values[1323] != 0.0) && (scratch.values[1340] != 0.0)) && (!(scratch.values[1856] != 0.0))) && (!(scratch.values[1867] != 0.0))) {
            scratch.store_ad(1384, &AdValue::mul(scratch.ad_value(605), AdValue::div(AdValue::mul(AdValue::sub(scratch.ad_value(541), scratch.ad_value(1357)), scratch.ad_value(620)), scratch.ad_value(1359))));
        }

        scratch.values[1869] = if (((((-scratch.values[635]) / scratch.values[1384])) as f64).abs() < 230.25850929940458) { 1.0 } else { 0.0 };

        if (((((scratch.values[1323] != 0.0) && (scratch.values[1340] != 0.0)) && (!(scratch.values[1856] != 0.0))) && (!(scratch.values[1867] != 0.0))) && (scratch.values[1869] != 0.0)) {
            scratch.store_ad(1359, &AdValue::exp(AdValue::div(AdValue::neg(scratch.ad_value(635)), scratch.ad_value(1384))));
        }

        scratch.values[1870] = if (((-scratch.values[635]) / scratch.values[1384]) < 0.0) { 1.0 } else { 0.0 };

        if ((((((scratch.values[1323] != 0.0) && (scratch.values[1340] != 0.0)) && (!(scratch.values[1856] != 0.0))) && (!(scratch.values[1867] != 0.0))) && (!(scratch.values[1869] != 0.0))) && (scratch.values[1870] != 0.0)) {
            let assign34520_ad_e43628: AdValue = AdValue::offset(AdValue::mul(AdValue::sub_from_scalar((-230.25850929940458), AdValue::div(AdValue::neg(scratch.ad_value(635)), scratch.ad_value(1384))), AdValue::offset(AdValue::scale(AdValue::mul(AdValue::sub_from_scalar((-230.25850929940458), AdValue::div(AdValue::neg(scratch.ad_value(635)), scratch.ad_value(1384))), AdValue::offset(AdValue::scale(AdValue::sub_from_scalar((-230.25850929940458), AdValue::div(AdValue::neg(scratch.ad_value(635)), scratch.ad_value(1384))), 0.3333333333333333), 1.0)), 0.5), 1.0)), 1.0);
            scratch.store_ad(1359, &AdValue::div_from_scalar(1e-100, assign34520_ad_e43628));
        }

        if ((((((scratch.values[1323] != 0.0) && (scratch.values[1340] != 0.0)) && (!(scratch.values[1856] != 0.0))) && (!(scratch.values[1867] != 0.0))) && (!(scratch.values[1869] != 0.0))) && (!(scratch.values[1870] != 0.0))) {
            let assign34530_ad_e43678: AdValue = AdValue::scale(AdValue::offset(AdValue::mul(AdValue::offset(AdValue::div(AdValue::neg(scratch.ad_value(635)), scratch.ad_value(1384)), (-230.25850929940458)), AdValue::offset(AdValue::scale(AdValue::mul(AdValue::offset(AdValue::div(AdValue::neg(scratch.ad_value(635)), scratch.ad_value(1384)), (-230.25850929940458)), AdValue::offset(AdValue::scale(AdValue::offset(AdValue::div(AdValue::neg(scratch.ad_value(635)), scratch.ad_value(1384)), (-230.25850929940458)), 0.3333333333333333), 1.0)), 0.5), 1.0)), 1.0), 1e100);
            scratch.store_ad(1359, &assign34530_ad_e43678);
        }

        if ((((scratch.values[1323] != 0.0) && (scratch.values[1340] != 0.0)) && (!(scratch.values[1856] != 0.0))) && (!(scratch.values[1867] != 0.0))) {
            scratch.store_ad(1383, &AdValue::mul(scratch.ad_value(564), AdValue::mul(AdValue::mul(AdValue::mul(scratch.ad_value(518), scratch.ad_value(1384)), scratch.ad_value(1384)), scratch.ad_value(1359))));
        }

        scratch.values[1871] = if (scratch.values[573] > 1000.0) { 1.0 } else { 0.0 };

        if ((((scratch.values[1323] != 0.0) && (scratch.values[1340] != 0.0)) && (!(scratch.values[1856] != 0.0))) && (scratch.values[1871] != 0.0)) {
            scratch.values[1385] = 1.0;
            scratch.node_derivatives[1385] = [0.0; Instance::NODE_COUNT];
            scratch.branch_derivatives[1385] = [0.0; Instance::BRANCH_COUNT];
        }

        scratch.values[1872] = if (scratch.values[1358] > ((-scratch.values[493]) * scratch.values[573])) { 1.0 } else { 0.0 };

        scratch.values[1873] = if (scratch.values[576] == 4.0) { 1.0 } else { 0.0 };

        if ((((((scratch.values[1323] != 0.0) && (scratch.values[1340] != 0.0)) && (!(scratch.values[1856] != 0.0))) && (!(scratch.values[1871] != 0.0))) && (scratch.values[1872] != 0.0)) && (scratch.values[1873] != 0.0)) {
            scratch.store_ad(1359, &AdValue::mul(AdValue::mul(AdValue::mul(AdValue::mul(scratch.ad_value(1358), scratch.ad_value(641)), AdValue::mul(scratch.ad_value(1358), scratch.ad_value(641))), AdValue::mul(scratch.ad_value(1358), scratch.ad_value(641))), AdValue::mul(scratch.ad_value(1358), scratch.ad_value(641))));
        }

        if ((((((scratch.values[1323] != 0.0) && (scratch.values[1340] != 0.0)) && (!(scratch.values[1856] != 0.0))) && (!(scratch.values[1871] != 0.0))) && (scratch.values[1872] != 0.0)) && (!(scratch.values[1873] != 0.0))) {
            scratch.store_ad(1359, &AdValue::pow(AdValue::abs(AdValue::mul(scratch.ad_value(1358), scratch.ad_value(641))), scratch.ad_value(576)));
        }

        if (((((scratch.values[1323] != 0.0) && (scratch.values[1340] != 0.0)) && (!(scratch.values[1856] != 0.0))) && (!(scratch.values[1871] != 0.0))) && (scratch.values[1872] != 0.0)) {
            scratch.store_ad(1385, &AdValue::div_from_scalar(1.0, AdValue::sub_from_scalar(1.0, scratch.ad_value(1359))));
        }

        if (((((scratch.values[1323] != 0.0) && (scratch.values[1340] != 0.0)) && (!(scratch.values[1856] != 0.0))) && (!(scratch.values[1871] != 0.0))) && (!(scratch.values[1872] != 0.0))) {
            scratch.store_ad(1385, &AdValue::add(scratch.ad_value(638), AdValue::mul(AdValue::add(scratch.ad_value(1358), AdValue::scale(scratch.ad_value(573), scratch.values[493])), scratch.ad_value(644))));
        }

        if (((scratch.values[1323] != 0.0) && (scratch.values[1340] != 0.0)) && (!(scratch.values[1856] != 0.0))) {
            scratch.store_ad(1388, &AdValue::mul(AdValue::add(AdValue::add(AdValue::add(scratch.ad_value(1360), scratch.ad_value(1361)), scratch.ad_value(1368)), scratch.ad_value(1383)), scratch.ad_value(1385)));
        }

        if ((scratch.values[1323] != 0.0) && (scratch.values[1340] != 0.0)) {
            scratch.store_ad(508, &AdValue::add(AdValue::add(AdValue::mul(scratch.ad_value(717), scratch.ad_value(1386)), AdValue::mul(scratch.ad_value(718), scratch.ad_value(1387))), AdValue::mul(scratch.ad_value(719), scratch.ad_value(1388))));
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

        scratch.values[1874] = if !(((scratch.values[717] == 0.0) && (scratch.values[718] == 0.0)) && (scratch.values[719] == 0.0)) { 1.0 } else { 0.0 };

        if (((scratch.values[1323] != 0.0) && (scratch.values[1340] != 0.0)) && (scratch.values[1874] != 0.0)) {
            scratch.store_ad(1344, &AdValue::mul(AdValue::scale(scratch.ad_value(728), 4.0), scratch.ad_value(728)));
        }

        if (((scratch.values[1323] != 0.0) && (scratch.values[1340] != 0.0)) && (scratch.values[1874] != 0.0)) {
            scratch.store_ad(1345, &AdValue::div(scratch.ad_value(728), scratch.ad_value(729)));
        }

        if (((scratch.values[1323] != 0.0) && (scratch.values[1340] != 0.0)) && (scratch.values[1874] != 0.0)) {
            scratch.store_ad(1346, &AdValue::add(scratch.ad_value(519), AdValue::mul(scratch.ad_value(728), scratch.ad_value(1345))));
        }

        if (((scratch.values[1323] != 0.0) && (scratch.values[1340] != 0.0)) && (scratch.values[1874] != 0.0)) {
            scratch.store_ad(1347, &AdValue::add(scratch.ad_value(729), scratch.ad_value(1346)));
        }

        if (((scratch.values[1323] != 0.0) && (scratch.values[1340] != 0.0)) && (scratch.values[1874] != 0.0)) {
            scratch.store_ad(1348, &AdValue::sub(scratch.ad_value(729), scratch.ad_value(1346)));
        }

        if (((scratch.values[1323] != 0.0) && (scratch.values[1340] != 0.0)) && (scratch.values[1874] != 0.0)) {
            scratch.store_ad(1349, &AdValue::sqrt(AdValue::add(AdValue::square(scratch.ad_value(1348)), scratch.ad_value(1344))));
        }

        if (((scratch.values[1323] != 0.0) && (scratch.values[1340] != 0.0)) && (scratch.values[1874] != 0.0)) {
            scratch.store_ad(1351, &AdValue::scale(AdValue::div(AdValue::mul(scratch.ad_value(519), scratch.ad_value(729)), AdValue::add(scratch.ad_value(1347), scratch.ad_value(1349))), 2.0));
        }

        scratch.values[1875] = if (scratch.values[519] < scratch.values[725]) { 1.0 } else { 0.0 };

        scratch.values[1876] = if ((((0.5 * (scratch.values[519] * scratch.values[420]))) as f64).abs() < 230.25850929940458) { 1.0 } else { 0.0 };

        if (((((scratch.values[1323] != 0.0) && (scratch.values[1340] != 0.0)) && (scratch.values[1874] != 0.0)) && (scratch.values[1875] != 0.0)) && (scratch.values[1876] != 0.0)) {
            scratch.store_ad(1353, &AdValue::exp(AdValue::scale(scratch.ad_value(519), (scratch.values[420] * 0.5))));
        }

        scratch.values[1877] = if ((0.5 * (scratch.values[519] * scratch.values[420])) < 0.0) { 1.0 } else { 0.0 };

        if ((((((scratch.values[1323] != 0.0) && (scratch.values[1340] != 0.0)) && (scratch.values[1874] != 0.0)) && (scratch.values[1875] != 0.0)) && (!(scratch.values[1876] != 0.0))) && (scratch.values[1877] != 0.0)) {
            let assign34790_ad_e44041: AdValue = AdValue::div_from_scalar(1e-100, AdValue::offset(AdValue::mul(AdValue::sub_from_scalar((-230.25850929940458), AdValue::scale(scratch.ad_value(519), (scratch.values[420] * 0.5))), AdValue::offset(AdValue::scale(AdValue::mul(AdValue::sub_from_scalar((-230.25850929940458), AdValue::scale(scratch.ad_value(519), (scratch.values[420] * 0.5))), AdValue::offset(AdValue::scale(AdValue::sub_from_scalar((-230.25850929940458), AdValue::scale(scratch.ad_value(519), (scratch.values[420] * 0.5))), 0.3333333333333333), 1.0)), 0.5), 1.0)), 1.0));
            scratch.store_ad(1353, &assign34790_ad_e44041);
        }

        if ((((((scratch.values[1323] != 0.0) && (scratch.values[1340] != 0.0)) && (scratch.values[1874] != 0.0)) && (scratch.values[1875] != 0.0)) && (!(scratch.values[1876] != 0.0))) && (!(scratch.values[1877] != 0.0))) {
            scratch.store_ad(1353, &AdValue::scale(AdValue::offset(AdValue::mul(AdValue::offset(AdValue::scale(scratch.ad_value(519), (scratch.values[420] * 0.5)), (-230.25850929940458)), AdValue::offset(AdValue::scale(AdValue::mul(AdValue::offset(AdValue::scale(scratch.ad_value(519), (scratch.values[420] * 0.5)), (-230.25850929940458)), AdValue::offset(AdValue::scale(AdValue::offset(AdValue::scale(scratch.ad_value(519), (scratch.values[420] * 0.5)), (-230.25850929940458)), 0.3333333333333333), 1.0)), 0.5), 1.0)), 1.0), 1e100));
        }

        if ((((scratch.values[1323] != 0.0) && (scratch.values[1340] != 0.0)) && (scratch.values[1874] != 0.0)) && (scratch.values[1875] != 0.0)) {
            scratch.store_ad(1350, &AdValue::square(scratch.ad_value(1353)));
        }

        if ((((scratch.values[1323] != 0.0) && (scratch.values[1340] != 0.0)) && (scratch.values[1874] != 0.0)) && (!(scratch.values[1875] != 0.0))) {
            scratch.store_ad(1350, &AdValue::mul(AdValue::offset(AdValue::scale(AdValue::sub(scratch.ad_value(519), scratch.ad_value(725)), scratch.values[420]), 1.0), scratch.ad_value(726)));
        }

        if ((((scratch.values[1323] != 0.0) && (scratch.values[1340] != 0.0)) && (scratch.values[1874] != 0.0)) && (!(scratch.values[1875] != 0.0))) {
            scratch.store_ad(1353, &AdValue::sqrt(scratch.ad_value(1350)));
        }

        if (((scratch.values[1323] != 0.0) && (scratch.values[1340] != 0.0)) && (scratch.values[1874] != 0.0)) {
            scratch.store_ad(1350, &AdValue::offset(scratch.ad_value(1350), (-1.0)));
        }

        if (((scratch.values[1323] != 0.0) && (scratch.values[1340] != 0.0)) && (scratch.values[1874] != 0.0)) {
            scratch.store_ad(1352, &AdValue::div_from_scalar(1.0, scratch.ad_value(1353)));
        }

        scratch.values[1878] = if (scratch.values[519] > 0.0) { 1.0 } else { 0.0 };

        if ((((scratch.values[1323] != 0.0) && (scratch.values[1340] != 0.0)) && (scratch.values[1874] != 0.0)) && (scratch.values[1878] != 0.0)) {
            scratch.store_ad(1354, &AdValue::scale(AdValue::ln(AdValue::add(AdValue::offset(scratch.ad_value(1352), 2.0), AdValue::sqrt(AdValue::mul(AdValue::offset(scratch.ad_value(1352), 1.0), AdValue::offset(scratch.ad_value(1352), 3.0))))), (scratch.values[419] * 2.0)));
        }

        if ((((scratch.values[1323] != 0.0) && (scratch.values[1340] != 0.0)) && (scratch.values[1874] != 0.0)) && (!(scratch.values[1878] != 0.0))) {
            scratch.store_ad(1354, &AdValue::sub(AdValue::scale(AdValue::ln(AdValue::add(AdValue::offset(AdValue::scale(scratch.ad_value(1353), 2.0), 1.0), AdValue::sqrt(AdValue::mul(AdValue::offset(scratch.ad_value(1353), 1.0), AdValue::offset(AdValue::scale(scratch.ad_value(1353), 3.0), 1.0))))), (scratch.values[419] * 2.0)), scratch.ad_value(519)));
        }

        if (((scratch.values[1323] != 0.0) && (scratch.values[1340] != 0.0)) && (scratch.values[1874] != 0.0)) {
            scratch.store_ad(1355, &AdValue::sub(scratch.ad_value(727), scratch.ad_value(1354)));
        }

        if (((scratch.values[1323] != 0.0) && (scratch.values[1340] != 0.0)) && (scratch.values[1874] != 0.0)) {
            scratch.store_ad(1356, &AdValue::scale(AdValue::sub(AdValue::add(scratch.ad_value(519), scratch.ad_value(1355)), AdValue::sqrt(AdValue::offset(AdValue::mul(AdValue::sub(scratch.ad_value(519), scratch.ad_value(1355)), AdValue::sub(scratch.ad_value(519), scratch.ad_value(1355))), ((4.0 * scratch.values[419]) * scratch.values[419])))), 0.5));
        }

        if (((scratch.values[1323] != 0.0) && (scratch.values[1340] != 0.0)) && (scratch.values[1874] != 0.0)) {
            scratch.store_ad(1357, &AdValue::scale(AdValue::sub(AdValue::add(scratch.ad_value(519), scratch.ad_value(730)), AdValue::sqrt(AdValue::offset(AdValue::mul(AdValue::sub(scratch.ad_value(519), scratch.ad_value(730)), AdValue::sub(scratch.ad_value(519), scratch.ad_value(730))), ((4.0 * scratch.values[417]) * scratch.values[417])))), 0.5));
        }

        if (((scratch.values[1323] != 0.0) && (scratch.values[1340] != 0.0)) && (scratch.values[1874] != 0.0)) {
            scratch.store_ad(1358, &AdValue::scale(AdValue::sub(scratch.ad_value(519), AdValue::sqrt(AdValue::offset(AdValue::mul(scratch.ad_value(519), scratch.ad_value(519)), ((4.0 * 1e-6) * 1e-6)))), 0.5));
        }

        scratch.values[1879] = if (scratch.values[717] == 0.0) { 1.0 } else { 0.0 };

        if (((scratch.values[1323] != 0.0) && (scratch.values[1340] != 0.0)) && (scratch.values[1879] != 0.0)) {
            scratch.values[1386] = 0.0;
            scratch.node_derivatives[1386] = [0.0; Instance::NODE_COUNT];
            scratch.branch_derivatives[1386] = [0.0; Instance::BRANCH_COUNT];
        }

        scratch.values[1880] = if (scratch.values[600] == 0.5) { 1.0 } else { 0.0 };

        if ((((scratch.values[1323] != 0.0) && (scratch.values[1340] != 0.0)) && (!(scratch.values[1879] != 0.0))) && (scratch.values[1880] != 0.0)) {
            scratch.store_ad(1359, &AdValue::sqrt(AdValue::sub_from_scalar(1.0, AdValue::mul(scratch.ad_value(1351), scratch.ad_value(597)))));
        }

        if ((((scratch.values[1323] != 0.0) && (scratch.values[1340] != 0.0)) && (!(scratch.values[1879] != 0.0))) && (!(scratch.values[1880] != 0.0))) {
            scratch.store_ad(1359, &AdValue::pow(AdValue::sub_from_scalar(1.0, AdValue::mul(scratch.ad_value(1351), scratch.ad_value(597))), scratch.ad_value(600)));
        }

        if (((scratch.values[1323] != 0.0) && (scratch.values[1340] != 0.0)) && (!(scratch.values[1879] != 0.0))) {
            scratch.store_ad(1360, &AdValue::mul(scratch.ad_value(588), scratch.ad_value(1350)));
        }

        scratch.values[1881] = if ((scratch.values[553] == 0.0) && (scratch.values[556] == 0.0)) { 1.0 } else { 0.0 };

        if ((((scratch.values[1323] != 0.0) && (scratch.values[1340] != 0.0)) && (!(scratch.values[1879] != 0.0))) && (scratch.values[1881] != 0.0)) {
            scratch.values[1361] = 0.0;
            scratch.node_derivatives[1361] = [0.0; Instance::NODE_COUNT];
            scratch.branch_derivatives[1361] = [0.0; Instance::BRANCH_COUNT];
        }

        if ((((scratch.values[1323] != 0.0) && (scratch.values[1340] != 0.0)) && (!(scratch.values[1879] != 0.0))) && (!(scratch.values[1881] != 0.0))) {
            scratch.store_ad(1362, &AdValue::sub(scratch.ad_value(594), scratch.ad_value(1356)));
        }

        if ((((scratch.values[1323] != 0.0) && (scratch.values[1340] != 0.0)) && (!(scratch.values[1879] != 0.0))) && (!(scratch.values[1881] != 0.0))) {
            scratch.store_ad(1363, &AdValue::sub_from_scalar(1.0, AdValue::sqrt(AdValue::sub_from_scalar(1.0, AdValue::div(scratch.ad_value(1354), scratch.ad_value(1362))))));
        }

        scratch.values[1882] = if (scratch.values[542] == 0.5) { 1.0 } else { 0.0 };

        if (((((scratch.values[1323] != 0.0) && (scratch.values[1340] != 0.0)) && (!(scratch.values[1879] != 0.0))) && (!(scratch.values[1881] != 0.0))) && (scratch.values[1882] != 0.0)) {
            scratch.values[1364] = 0.0;
            scratch.node_derivatives[1364] = [0.0; Instance::NODE_COUNT];
            scratch.branch_derivatives[1364] = [0.0; Instance::BRANCH_COUNT];
        }

        if (((((scratch.values[1323] != 0.0) && (scratch.values[1340] != 0.0)) && (!(scratch.values[1879] != 0.0))) && (!(scratch.values[1881] != 0.0))) && (!(scratch.values[1882] != 0.0))) {
            scratch.store_ad(1364, &AdValue::mul(AdValue::add(AdValue::div(AdValue::mul(AdValue::square(scratch.ad_value(1363)), AdValue::ln(scratch.ad_value(1363))), AdValue::sub_from_scalar(1.0, scratch.ad_value(1363))), scratch.ad_value(1363)), AdValue::sub_from_scalar(1.0, AdValue::scale(scratch.ad_value(542), 2.0))));
        }

        if ((((scratch.values[1323] != 0.0) && (scratch.values[1340] != 0.0)) && (!(scratch.values[1879] != 0.0))) && (!(scratch.values[1881] != 0.0))) {
            scratch.store_ad(1365, &AdValue::add(scratch.ad_value(1363), scratch.ad_value(1364)));
        }

        scratch.values[1883] = if (scratch.values[542] == 0.5) { 1.0 } else { 0.0 };

        if (((((scratch.values[1323] != 0.0) && (scratch.values[1340] != 0.0)) && (!(scratch.values[1879] != 0.0))) && (!(scratch.values[1881] != 0.0))) && (scratch.values[1883] != 0.0)) {
            scratch.store_ad(1359, &AdValue::sqrt(AdValue::mul(scratch.ad_value(1362), scratch.ad_value(621))));
        }

        if (((((scratch.values[1323] != 0.0) && (scratch.values[1340] != 0.0)) && (!(scratch.values[1879] != 0.0))) && (!(scratch.values[1881] != 0.0))) && (!(scratch.values[1883] != 0.0))) {
            scratch.store_ad(1359, &AdValue::pow(AdValue::mul(scratch.ad_value(1362), scratch.ad_value(621)), scratch.ad_value(542)));
        }

        if ((((scratch.values[1323] != 0.0) && (scratch.values[1340] != 0.0)) && (!(scratch.values[1879] != 0.0))) && (!(scratch.values[1881] != 0.0))) {
            scratch.store_ad(1366, &AdValue::mul(scratch.ad_value(615), scratch.ad_value(1359)));
        }

        if ((((scratch.values[1323] != 0.0) && (scratch.values[1340] != 0.0)) && (!(scratch.values[1879] != 0.0))) && (!(scratch.values[1881] != 0.0))) {
            scratch.store_ad(1367, &AdValue::mul(scratch.ad_value(585), AdValue::mul(AdValue::offset(scratch.ad_value(1353), (-1.0)), scratch.ad_value(1366))));
        }

        if ((((scratch.values[1323] != 0.0) && (scratch.values[1340] != 0.0)) && (!(scratch.values[1879] != 0.0))) && (!(scratch.values[1881] != 0.0))) {
            scratch.store_ad(1361, &AdValue::mul(scratch.ad_value(553), AdValue::mul(scratch.ad_value(1367), scratch.ad_value(1365))));
        }

        scratch.values[1884] = if (scratch.values[556] == 0.0) { 1.0 } else { 0.0 };

        if ((((scratch.values[1323] != 0.0) && (scratch.values[1340] != 0.0)) && (!(scratch.values[1879] != 0.0))) && (scratch.values[1884] != 0.0)) {
            scratch.values[1368] = 0.0;
            scratch.node_derivatives[1368] = [0.0; Instance::NODE_COUNT];
            scratch.branch_derivatives[1368] = [0.0; Instance::BRANCH_COUNT];
        }

        if ((((scratch.values[1323] != 0.0) && (scratch.values[1340] != 0.0)) && (!(scratch.values[1879] != 0.0))) && (!(scratch.values[1884] != 0.0))) {
            scratch.store_ad(1369, &AdValue::mul(scratch.ad_value(630), AdValue::div(AdValue::mul(scratch.ad_value(1366), scratch.ad_value(600)), scratch.ad_value(1362))));
        }

        if ((((scratch.values[1323] != 0.0) && (scratch.values[1340] != 0.0)) && (!(scratch.values[1879] != 0.0))) && (!(scratch.values[1884] != 0.0))) {
            scratch.store_ad(1370, &AdValue::div(AdValue::scale(scratch.ad_value(627), 0.666666666666667), scratch.ad_value(1369)));
        }

        if ((((scratch.values[1323] != 0.0) && (scratch.values[1340] != 0.0)) && (!(scratch.values[1879] != 0.0))) && (!(scratch.values[1884] != 0.0))) {
            scratch.store_ad(1371, &AdValue::square(scratch.ad_value(1370)));
        }

        if ((((scratch.values[1323] != 0.0) && (scratch.values[1340] != 0.0)) && (!(scratch.values[1879] != 0.0))) && (!(scratch.values[1884] != 0.0))) {
            scratch.store_ad(1372, &AdValue::sqrt(AdValue::div(AdValue::square(scratch.ad_value(1371)), AdValue::offset(AdValue::square(scratch.ad_value(1371)), 1.0))));
        }

        if ((((scratch.values[1323] != 0.0) && (scratch.values[1340] != 0.0)) && (!(scratch.values[1879] != 0.0))) && (!(scratch.values[1884] != 0.0))) {
            scratch.store_ad(1373, &AdValue::sqrt(AdValue::abs(scratch.ad_value(1372))));
        }

        if ((((scratch.values[1323] != 0.0) && (scratch.values[1340] != 0.0)) && (!(scratch.values[1879] != 0.0))) && (!(scratch.values[1884] != 0.0))) {
            scratch.store_ad(1374, &AdValue::mul(scratch.ad_value(1372), scratch.ad_value(1373)));
        }

        scratch.values[1885] = if (((-scratch.values[542]) * scratch.values[603]) == (-1.0)) { 1.0 } else { 0.0 };

        if (((((scratch.values[1323] != 0.0) && (scratch.values[1340] != 0.0)) && (!(scratch.values[1879] != 0.0))) && (!(scratch.values[1884] != 0.0))) && (scratch.values[1885] != 0.0)) {
            scratch.store_ad(1375, &AdValue::div_from_scalar(1.0, AdValue::offset(AdValue::mul(scratch.ad_value(1369), scratch.ad_value(1374)), 1.0)));
        }

        if (((((scratch.values[1323] != 0.0) && (scratch.values[1340] != 0.0)) && (!(scratch.values[1879] != 0.0))) && (!(scratch.values[1884] != 0.0))) && (!(scratch.values[1885] != 0.0))) {
            scratch.store_ad(1375, &AdValue::pow(AdValue::offset(AdValue::mul(scratch.ad_value(1369), scratch.ad_value(1374)), 1.0), AdValue::mul(AdValue::neg(scratch.ad_value(542)), scratch.ad_value(603))));
        }

        if ((((scratch.values[1323] != 0.0) && (scratch.values[1340] != 0.0)) && (!(scratch.values[1879] != 0.0))) && (!(scratch.values[1884] != 0.0))) {
            scratch.store_ad(1376, &AdValue::div(AdValue::mul(scratch.ad_value(1365), scratch.ad_value(1375)), AdValue::add(scratch.ad_value(1365), scratch.ad_value(1375))));
        }

        if ((((scratch.values[1323] != 0.0) && (scratch.values[1340] != 0.0)) && (!(scratch.values[1879] != 0.0))) && (!(scratch.values[1884] != 0.0))) {
            scratch.store_ad(1377, &AdValue::sqrt(AdValue::scale(AdValue::div(scratch.ad_value(1369), scratch.ad_value(1373)), 0.375)));
        }

        if ((((scratch.values[1323] != 0.0) && (scratch.values[1340] != 0.0)) && (!(scratch.values[1879] != 0.0))) && (!(scratch.values[1884] != 0.0))) {
            scratch.store_ad(1378, &AdValue::sub(AdValue::scale(AdValue::mul(scratch.ad_value(1370), scratch.ad_value(1373)), 2.0), scratch.ad_value(1372)));
        }

        if ((((scratch.values[1323] != 0.0) && (scratch.values[1340] != 0.0)) && (!(scratch.values[1879] != 0.0))) && (!(scratch.values[1884] != 0.0))) {
            scratch.store_ad(1379, &AdValue::add(AdValue::sub(AdValue::mul(AdValue::mul(scratch.ad_value(627), scratch.ad_value(1370)), scratch.ad_value(1373)), AdValue::mul(scratch.ad_value(627), scratch.ad_value(1372))), AdValue::scale(AdValue::mul(scratch.ad_value(1369), scratch.ad_value(1374)), 0.5)));
        }

        if ((((scratch.values[1323] != 0.0) && (scratch.values[1340] != 0.0)) && (!(scratch.values[1879] != 0.0))) && (!(scratch.values[1884] != 0.0))) {
            scratch.store_ad(1380, &AdValue::mul(AdValue::offset(scratch.ad_value(1378), (-1.0)), scratch.ad_value(1377)));
        }

        if ((((scratch.values[1323] != 0.0) && (scratch.values[1340] != 0.0)) && (!(scratch.values[1879] != 0.0))) && (!(scratch.values[1884] != 0.0))) {
            scratch.store_ad(1341, &AdValue::square(scratch.ad_value(1380)));
        }

        scratch.values[1886] = if (scratch.values[1380] > 0.0) { 1.0 } else { 0.0 };

        if (((((scratch.values[1323] != 0.0) && (scratch.values[1340] != 0.0)) && (!(scratch.values[1879] != 0.0))) && (!(scratch.values[1884] != 0.0))) && (scratch.values[1886] != 0.0)) {
            scratch.store_ad(1342, &AdValue::div_from_scalar(1.0, AdValue::offset(AdValue::scale(scratch.ad_value(1380), scratch.values[421]), 1.0)));
        }

        if (((((scratch.values[1323] != 0.0) && (scratch.values[1340] != 0.0)) && (!(scratch.values[1879] != 0.0))) && (!(scratch.values[1884] != 0.0))) && (!(scratch.values[1886] != 0.0))) {
            scratch.store_ad(1342, &AdValue::div_from_scalar(1.0, AdValue::sub_from_scalar(1.0, AdValue::scale(scratch.ad_value(1380), scratch.values[421]))));
        }

        scratch.values[1887] = if (((-scratch.values[1341]) + scratch.values[1379]) > (-230.25850929940458)) { 1.0 } else { 0.0 };

        if (((((scratch.values[1323] != 0.0) && (scratch.values[1340] != 0.0)) && (!(scratch.values[1879] != 0.0))) && (!(scratch.values[1884] != 0.0))) && (scratch.values[1887] != 0.0)) {
            scratch.store_ad(1359, &AdValue::exp(AdValue::sub(scratch.ad_value(1379), scratch.ad_value(1341))));
        }

        if (((((scratch.values[1323] != 0.0) && (scratch.values[1340] != 0.0)) && (!(scratch.values[1879] != 0.0))) && (!(scratch.values[1884] != 0.0))) && (!(scratch.values[1887] != 0.0))) {
            let assign35370_ad_e44984: AdValue = AdValue::div_from_scalar(1e-100, AdValue::offset(AdValue::mul(AdValue::sub_from_scalar((-230.25850929940458), AdValue::sub(scratch.ad_value(1379), scratch.ad_value(1341))), AdValue::offset(AdValue::scale(AdValue::mul(AdValue::sub_from_scalar((-230.25850929940458), AdValue::sub(scratch.ad_value(1379), scratch.ad_value(1341))), AdValue::offset(AdValue::scale(AdValue::sub_from_scalar((-230.25850929940458), AdValue::sub(scratch.ad_value(1379), scratch.ad_value(1341))), 0.3333333333333333), 1.0)), 0.5), 1.0)), 1.0));
            scratch.store_ad(1359, &assign35370_ad_e44984);
        }

        if ((((scratch.values[1323] != 0.0) && (scratch.values[1340] != 0.0)) && (!(scratch.values[1879] != 0.0))) && (!(scratch.values[1884] != 0.0))) {
            scratch.store_ad(1343, &AdValue::mul(AdValue::add(AdValue::add(AdValue::scale(scratch.ad_value(1342), 0.29214664), AdValue::scale(AdValue::square(scratch.ad_value(1342)), scratch.values[422])), AdValue::scale(AdValue::mul(AdValue::square(scratch.ad_value(1342)), scratch.ad_value(1342)), scratch.values[423])), scratch.ad_value(1359)));
        }

        scratch.values[1888] = if (scratch.values[1380] > 0.0) { 1.0 } else { 0.0 };

        if (((((scratch.values[1323] != 0.0) && (scratch.values[1340] != 0.0)) && (!(scratch.values[1879] != 0.0))) && (!(scratch.values[1884] != 0.0))) && (scratch.values[1888] != 0.0)) {
            scratch.values[1381] = scratch.values[1343];
            scratch.node_derivatives[1381] = scratch.node_derivatives[1343];
            scratch.branch_derivatives[1381] = scratch.branch_derivatives[1343];
        }

        scratch.values[1889] = if (scratch.values[1379] > (-230.25850929940458)) { 1.0 } else { 0.0 };

        if ((((((scratch.values[1323] != 0.0) && (scratch.values[1340] != 0.0)) && (!(scratch.values[1879] != 0.0))) && (!(scratch.values[1884] != 0.0))) && (!(scratch.values[1888] != 0.0))) && (scratch.values[1889] != 0.0)) {
            scratch.store_ad(1359, &AdValue::exp(scratch.ad_value(1379)));
        }

        if ((((((scratch.values[1323] != 0.0) && (scratch.values[1340] != 0.0)) && (!(scratch.values[1879] != 0.0))) && (!(scratch.values[1884] != 0.0))) && (!(scratch.values[1888] != 0.0))) && (!(scratch.values[1889] != 0.0))) {
            scratch.store_ad(1359, &AdValue::div_from_scalar(1e-100, AdValue::offset(AdValue::mul(AdValue::sub_from_scalar((-230.25850929940458), scratch.ad_value(1379)), AdValue::offset(AdValue::scale(AdValue::mul(AdValue::sub_from_scalar((-230.25850929940458), scratch.ad_value(1379)), AdValue::offset(AdValue::scale(AdValue::sub_from_scalar((-230.25850929940458), scratch.ad_value(1379)), 0.3333333333333333), 1.0)), 0.5), 1.0)), 1.0)));
        }

        if (((((scratch.values[1323] != 0.0) && (scratch.values[1340] != 0.0)) && (!(scratch.values[1879] != 0.0))) && (!(scratch.values[1884] != 0.0))) && (!(scratch.values[1888] != 0.0))) {
            scratch.store_ad(1381, &AdValue::sub(AdValue::scale(scratch.ad_value(1359), 2.0), scratch.ad_value(1343)));
        }

        if ((((scratch.values[1323] != 0.0) && (scratch.values[1340] != 0.0)) && (!(scratch.values[1879] != 0.0))) && (!(scratch.values[1884] != 0.0))) {
            scratch.store_ad(1382, &AdValue::scale(AdValue::div(AdValue::mul(scratch.ad_value(627), scratch.ad_value(1381)), scratch.ad_value(1377)), (1.772453850905516 * 0.5)));
        }

        if ((((scratch.values[1323] != 0.0) && (scratch.values[1340] != 0.0)) && (!(scratch.values[1879] != 0.0))) && (!(scratch.values[1884] != 0.0))) {
            scratch.store_ad(1368, &AdValue::mul(scratch.ad_value(556), AdValue::mul(AdValue::mul(scratch.ad_value(1367), scratch.ad_value(1382)), scratch.ad_value(1376))));
        }

        scratch.values[1890] = if (scratch.values[562] == 0.0) { 1.0 } else { 0.0 };

        if ((((scratch.values[1323] != 0.0) && (scratch.values[1340] != 0.0)) && (!(scratch.values[1879] != 0.0))) && (scratch.values[1890] != 0.0)) {
            scratch.values[1383] = 0.0;
            scratch.node_derivatives[1383] = [0.0; Instance::NODE_COUNT];
            scratch.branch_derivatives[1383] = [0.0; Instance::BRANCH_COUNT];
        }

        scratch.values[1891] = if (scratch.values[542] == 0.5) { 1.0 } else { 0.0 };

        if (((((scratch.values[1323] != 0.0) && (scratch.values[1340] != 0.0)) && (!(scratch.values[1879] != 0.0))) && (!(scratch.values[1890] != 0.0))) && (scratch.values[1891] != 0.0)) {
            scratch.store_ad(1359, &AdValue::sqrt(AdValue::mul(AdValue::sub(scratch.ad_value(539), scratch.ad_value(1357)), scratch.ad_value(621))));
        }

        if (((((scratch.values[1323] != 0.0) && (scratch.values[1340] != 0.0)) && (!(scratch.values[1879] != 0.0))) && (!(scratch.values[1890] != 0.0))) && (!(scratch.values[1891] != 0.0))) {
            scratch.store_ad(1359, &AdValue::pow(AdValue::mul(AdValue::sub(scratch.ad_value(539), scratch.ad_value(1357)), scratch.ad_value(621)), scratch.ad_value(542)));
        }

        if ((((scratch.values[1323] != 0.0) && (scratch.values[1340] != 0.0)) && (!(scratch.values[1879] != 0.0))) && (!(scratch.values[1890] != 0.0))) {
            scratch.store_ad(1384, &AdValue::mul(scratch.ad_value(603), AdValue::div(AdValue::mul(AdValue::sub(scratch.ad_value(539), scratch.ad_value(1357)), scratch.ad_value(618)), scratch.ad_value(1359))));
        }

        scratch.values[1892] = if (((((-scratch.values[633]) / scratch.values[1384])) as f64).abs() < 230.25850929940458) { 1.0 } else { 0.0 };

        if (((((scratch.values[1323] != 0.0) && (scratch.values[1340] != 0.0)) && (!(scratch.values[1879] != 0.0))) && (!(scratch.values[1890] != 0.0))) && (scratch.values[1892] != 0.0)) {
            scratch.store_ad(1359, &AdValue::exp(AdValue::div(AdValue::neg(scratch.ad_value(633)), scratch.ad_value(1384))));
        }

        scratch.values[1893] = if (((-scratch.values[633]) / scratch.values[1384]) < 0.0) { 1.0 } else { 0.0 };

        if ((((((scratch.values[1323] != 0.0) && (scratch.values[1340] != 0.0)) && (!(scratch.values[1879] != 0.0))) && (!(scratch.values[1890] != 0.0))) && (!(scratch.values[1892] != 0.0))) && (scratch.values[1893] != 0.0)) {
            let assign35560_ad_e45311: AdValue = AdValue::offset(AdValue::mul(AdValue::sub_from_scalar((-230.25850929940458), AdValue::div(AdValue::neg(scratch.ad_value(633)), scratch.ad_value(1384))), AdValue::offset(AdValue::scale(AdValue::mul(AdValue::sub_from_scalar((-230.25850929940458), AdValue::div(AdValue::neg(scratch.ad_value(633)), scratch.ad_value(1384))), AdValue::offset(AdValue::scale(AdValue::sub_from_scalar((-230.25850929940458), AdValue::div(AdValue::neg(scratch.ad_value(633)), scratch.ad_value(1384))), 0.3333333333333333), 1.0)), 0.5), 1.0)), 1.0);
            scratch.store_ad(1359, &AdValue::div_from_scalar(1e-100, assign35560_ad_e45311));
        }

        if ((((((scratch.values[1323] != 0.0) && (scratch.values[1340] != 0.0)) && (!(scratch.values[1879] != 0.0))) && (!(scratch.values[1890] != 0.0))) && (!(scratch.values[1892] != 0.0))) && (!(scratch.values[1893] != 0.0))) {
            let assign35570_ad_e45361: AdValue = AdValue::scale(AdValue::offset(AdValue::mul(AdValue::offset(AdValue::div(AdValue::neg(scratch.ad_value(633)), scratch.ad_value(1384)), (-230.25850929940458)), AdValue::offset(AdValue::scale(AdValue::mul(AdValue::offset(AdValue::div(AdValue::neg(scratch.ad_value(633)), scratch.ad_value(1384)), (-230.25850929940458)), AdValue::offset(AdValue::scale(AdValue::offset(AdValue::div(AdValue::neg(scratch.ad_value(633)), scratch.ad_value(1384)), (-230.25850929940458)), 0.3333333333333333), 1.0)), 0.5), 1.0)), 1.0), 1e100);
            scratch.store_ad(1359, &assign35570_ad_e45361);
        }

        if ((((scratch.values[1323] != 0.0) && (scratch.values[1340] != 0.0)) && (!(scratch.values[1879] != 0.0))) && (!(scratch.values[1890] != 0.0))) {
            scratch.store_ad(1383, &AdValue::mul(scratch.ad_value(562), AdValue::mul(AdValue::mul(AdValue::mul(scratch.ad_value(519), scratch.ad_value(1384)), scratch.ad_value(1384)), scratch.ad_value(1359))));
        }

        scratch.values[1894] = if (scratch.values[571] > 1000.0) { 1.0 } else { 0.0 };

        if ((((scratch.values[1323] != 0.0) && (scratch.values[1340] != 0.0)) && (!(scratch.values[1879] != 0.0))) && (scratch.values[1894] != 0.0)) {
            scratch.values[1385] = 1.0;
            scratch.node_derivatives[1385] = [0.0; Instance::NODE_COUNT];
            scratch.branch_derivatives[1385] = [0.0; Instance::BRANCH_COUNT];
        }

        scratch.values[1895] = if (scratch.values[1358] > ((-scratch.values[493]) * scratch.values[571])) { 1.0 } else { 0.0 };

        scratch.values[1896] = if (scratch.values[574] == 4.0) { 1.0 } else { 0.0 };

        if ((((((scratch.values[1323] != 0.0) && (scratch.values[1340] != 0.0)) && (!(scratch.values[1879] != 0.0))) && (!(scratch.values[1894] != 0.0))) && (scratch.values[1895] != 0.0)) && (scratch.values[1896] != 0.0)) {
            scratch.store_ad(1359, &AdValue::mul(AdValue::mul(AdValue::mul(AdValue::mul(scratch.ad_value(1358), scratch.ad_value(639)), AdValue::mul(scratch.ad_value(1358), scratch.ad_value(639))), AdValue::mul(scratch.ad_value(1358), scratch.ad_value(639))), AdValue::mul(scratch.ad_value(1358), scratch.ad_value(639))));
        }

        if ((((((scratch.values[1323] != 0.0) && (scratch.values[1340] != 0.0)) && (!(scratch.values[1879] != 0.0))) && (!(scratch.values[1894] != 0.0))) && (scratch.values[1895] != 0.0)) && (!(scratch.values[1896] != 0.0))) {
            scratch.store_ad(1359, &AdValue::pow(AdValue::abs(AdValue::mul(scratch.ad_value(1358), scratch.ad_value(639))), scratch.ad_value(574)));
        }

        if (((((scratch.values[1323] != 0.0) && (scratch.values[1340] != 0.0)) && (!(scratch.values[1879] != 0.0))) && (!(scratch.values[1894] != 0.0))) && (scratch.values[1895] != 0.0)) {
            scratch.store_ad(1385, &AdValue::div_from_scalar(1.0, AdValue::sub_from_scalar(1.0, scratch.ad_value(1359))));
        }

        if (((((scratch.values[1323] != 0.0) && (scratch.values[1340] != 0.0)) && (!(scratch.values[1879] != 0.0))) && (!(scratch.values[1894] != 0.0))) && (!(scratch.values[1895] != 0.0))) {
            scratch.store_ad(1385, &AdValue::add(scratch.ad_value(636), AdValue::mul(AdValue::add(scratch.ad_value(1358), AdValue::scale(scratch.ad_value(571), scratch.values[493])), scratch.ad_value(642))));
        }

        if (((scratch.values[1323] != 0.0) && (scratch.values[1340] != 0.0)) && (!(scratch.values[1879] != 0.0))) {
            scratch.store_ad(1386, &AdValue::mul(AdValue::add(AdValue::add(AdValue::add(scratch.ad_value(1360), scratch.ad_value(1361)), scratch.ad_value(1368)), scratch.ad_value(1383)), scratch.ad_value(1385)));
        }

        scratch.values[1897] = if (scratch.values[718] == 0.0) { 1.0 } else { 0.0 };

        if (((scratch.values[1323] != 0.0) && (scratch.values[1340] != 0.0)) && (scratch.values[1897] != 0.0)) {
            scratch.values[1387] = 0.0;
            scratch.node_derivatives[1387] = [0.0; Instance::NODE_COUNT];
            scratch.branch_derivatives[1387] = [0.0; Instance::BRANCH_COUNT];
        }

        scratch.values[1898] = if (scratch.values[601] == 0.5) { 1.0 } else { 0.0 };

        if ((((scratch.values[1323] != 0.0) && (scratch.values[1340] != 0.0)) && (!(scratch.values[1897] != 0.0))) && (scratch.values[1898] != 0.0)) {
            scratch.store_ad(1359, &AdValue::sqrt(AdValue::sub_from_scalar(1.0, AdValue::mul(scratch.ad_value(1351), scratch.ad_value(598)))));
        }

        if ((((scratch.values[1323] != 0.0) && (scratch.values[1340] != 0.0)) && (!(scratch.values[1897] != 0.0))) && (!(scratch.values[1898] != 0.0))) {
            scratch.store_ad(1359, &AdValue::pow(AdValue::sub_from_scalar(1.0, AdValue::mul(scratch.ad_value(1351), scratch.ad_value(598))), scratch.ad_value(601)));
        }

        if (((scratch.values[1323] != 0.0) && (scratch.values[1340] != 0.0)) && (!(scratch.values[1897] != 0.0))) {
            scratch.store_ad(1360, &AdValue::mul(scratch.ad_value(589), scratch.ad_value(1350)));
        }

        scratch.values[1899] = if ((scratch.values[554] == 0.0) && (scratch.values[557] == 0.0)) { 1.0 } else { 0.0 };

    }

    pub(super) fn stamp_transient_block_27(
        &mut self,
        ctx: &GeneratedEvalContext<'_>,
        stamper: &mut GeneratedStamper<'_>,
        scratch: &mut Scratch,
    ) {
        let _ = stamper;
        if ((((scratch.values[1323] != 0.0) && (scratch.values[1340] != 0.0)) && (!(scratch.values[1897] != 0.0))) && (scratch.values[1899] != 0.0)) {
            scratch.values[1361] = 0.0;
            scratch.node_derivatives[1361] = [0.0; Instance::NODE_COUNT];
            scratch.branch_derivatives[1361] = [0.0; Instance::BRANCH_COUNT];
        }

        if ((((scratch.values[1323] != 0.0) && (scratch.values[1340] != 0.0)) && (!(scratch.values[1897] != 0.0))) && (!(scratch.values[1899] != 0.0))) {
            scratch.store_ad(1362, &AdValue::sub(scratch.ad_value(595), scratch.ad_value(1356)));
        }

        if ((((scratch.values[1323] != 0.0) && (scratch.values[1340] != 0.0)) && (!(scratch.values[1897] != 0.0))) && (!(scratch.values[1899] != 0.0))) {
            scratch.store_ad(1363, &AdValue::sub_from_scalar(1.0, AdValue::sqrt(AdValue::sub_from_scalar(1.0, AdValue::div(scratch.ad_value(1354), scratch.ad_value(1362))))));
        }

        scratch.values[1900] = if (scratch.values[543] == 0.5) { 1.0 } else { 0.0 };

        if (((((scratch.values[1323] != 0.0) && (scratch.values[1340] != 0.0)) && (!(scratch.values[1897] != 0.0))) && (!(scratch.values[1899] != 0.0))) && (scratch.values[1900] != 0.0)) {
            scratch.values[1364] = 0.0;
            scratch.node_derivatives[1364] = [0.0; Instance::NODE_COUNT];
            scratch.branch_derivatives[1364] = [0.0; Instance::BRANCH_COUNT];
        }

        if (((((scratch.values[1323] != 0.0) && (scratch.values[1340] != 0.0)) && (!(scratch.values[1897] != 0.0))) && (!(scratch.values[1899] != 0.0))) && (!(scratch.values[1900] != 0.0))) {
            scratch.store_ad(1364, &AdValue::mul(AdValue::add(AdValue::div(AdValue::mul(AdValue::square(scratch.ad_value(1363)), AdValue::ln(scratch.ad_value(1363))), AdValue::sub_from_scalar(1.0, scratch.ad_value(1363))), scratch.ad_value(1363)), AdValue::sub_from_scalar(1.0, AdValue::scale(scratch.ad_value(543), 2.0))));
        }

        if ((((scratch.values[1323] != 0.0) && (scratch.values[1340] != 0.0)) && (!(scratch.values[1897] != 0.0))) && (!(scratch.values[1899] != 0.0))) {
            scratch.store_ad(1365, &AdValue::add(scratch.ad_value(1363), scratch.ad_value(1364)));
        }

        scratch.values[1901] = if (scratch.values[543] == 0.5) { 1.0 } else { 0.0 };

        if (((((scratch.values[1323] != 0.0) && (scratch.values[1340] != 0.0)) && (!(scratch.values[1897] != 0.0))) && (!(scratch.values[1899] != 0.0))) && (scratch.values[1901] != 0.0)) {
            scratch.store_ad(1359, &AdValue::sqrt(AdValue::mul(scratch.ad_value(1362), scratch.ad_value(622))));
        }

        if (((((scratch.values[1323] != 0.0) && (scratch.values[1340] != 0.0)) && (!(scratch.values[1897] != 0.0))) && (!(scratch.values[1899] != 0.0))) && (!(scratch.values[1901] != 0.0))) {
            scratch.store_ad(1359, &AdValue::pow(AdValue::mul(scratch.ad_value(1362), scratch.ad_value(622)), scratch.ad_value(543)));
        }

        if ((((scratch.values[1323] != 0.0) && (scratch.values[1340] != 0.0)) && (!(scratch.values[1897] != 0.0))) && (!(scratch.values[1899] != 0.0))) {
            scratch.store_ad(1366, &AdValue::mul(scratch.ad_value(616), scratch.ad_value(1359)));
        }

        if ((((scratch.values[1323] != 0.0) && (scratch.values[1340] != 0.0)) && (!(scratch.values[1897] != 0.0))) && (!(scratch.values[1899] != 0.0))) {
            scratch.store_ad(1367, &AdValue::mul(scratch.ad_value(586), AdValue::mul(AdValue::offset(scratch.ad_value(1353), (-1.0)), scratch.ad_value(1366))));
        }

        if ((((scratch.values[1323] != 0.0) && (scratch.values[1340] != 0.0)) && (!(scratch.values[1897] != 0.0))) && (!(scratch.values[1899] != 0.0))) {
            scratch.store_ad(1361, &AdValue::mul(scratch.ad_value(554), AdValue::mul(scratch.ad_value(1367), scratch.ad_value(1365))));
        }

        scratch.values[1902] = if (scratch.values[557] == 0.0) { 1.0 } else { 0.0 };

        if ((((scratch.values[1323] != 0.0) && (scratch.values[1340] != 0.0)) && (!(scratch.values[1897] != 0.0))) && (scratch.values[1902] != 0.0)) {
            scratch.values[1368] = 0.0;
            scratch.node_derivatives[1368] = [0.0; Instance::NODE_COUNT];
            scratch.branch_derivatives[1368] = [0.0; Instance::BRANCH_COUNT];
        }

        if ((((scratch.values[1323] != 0.0) && (scratch.values[1340] != 0.0)) && (!(scratch.values[1897] != 0.0))) && (!(scratch.values[1902] != 0.0))) {
            scratch.store_ad(1369, &AdValue::mul(scratch.ad_value(631), AdValue::div(AdValue::mul(scratch.ad_value(1366), scratch.ad_value(601)), scratch.ad_value(1362))));
        }

        if ((((scratch.values[1323] != 0.0) && (scratch.values[1340] != 0.0)) && (!(scratch.values[1897] != 0.0))) && (!(scratch.values[1902] != 0.0))) {
            scratch.store_ad(1370, &AdValue::div(AdValue::scale(scratch.ad_value(628), 0.666666666666667), scratch.ad_value(1369)));
        }

        if ((((scratch.values[1323] != 0.0) && (scratch.values[1340] != 0.0)) && (!(scratch.values[1897] != 0.0))) && (!(scratch.values[1902] != 0.0))) {
            scratch.store_ad(1371, &AdValue::square(scratch.ad_value(1370)));
        }

        if ((((scratch.values[1323] != 0.0) && (scratch.values[1340] != 0.0)) && (!(scratch.values[1897] != 0.0))) && (!(scratch.values[1902] != 0.0))) {
            scratch.store_ad(1372, &AdValue::sqrt(AdValue::div(AdValue::square(scratch.ad_value(1371)), AdValue::offset(AdValue::square(scratch.ad_value(1371)), 1.0))));
        }

        if ((((scratch.values[1323] != 0.0) && (scratch.values[1340] != 0.0)) && (!(scratch.values[1897] != 0.0))) && (!(scratch.values[1902] != 0.0))) {
            scratch.store_ad(1373, &AdValue::sqrt(AdValue::abs(scratch.ad_value(1372))));
        }

        if ((((scratch.values[1323] != 0.0) && (scratch.values[1340] != 0.0)) && (!(scratch.values[1897] != 0.0))) && (!(scratch.values[1902] != 0.0))) {
            scratch.store_ad(1374, &AdValue::mul(scratch.ad_value(1372), scratch.ad_value(1373)));
        }

        scratch.values[1903] = if (((-scratch.values[543]) * scratch.values[604]) == (-1.0)) { 1.0 } else { 0.0 };

        if (((((scratch.values[1323] != 0.0) && (scratch.values[1340] != 0.0)) && (!(scratch.values[1897] != 0.0))) && (!(scratch.values[1902] != 0.0))) && (scratch.values[1903] != 0.0)) {
            scratch.store_ad(1375, &AdValue::div_from_scalar(1.0, AdValue::offset(AdValue::mul(scratch.ad_value(1369), scratch.ad_value(1374)), 1.0)));
        }

        if (((((scratch.values[1323] != 0.0) && (scratch.values[1340] != 0.0)) && (!(scratch.values[1897] != 0.0))) && (!(scratch.values[1902] != 0.0))) && (!(scratch.values[1903] != 0.0))) {
            scratch.store_ad(1375, &AdValue::pow(AdValue::offset(AdValue::mul(scratch.ad_value(1369), scratch.ad_value(1374)), 1.0), AdValue::mul(AdValue::neg(scratch.ad_value(543)), scratch.ad_value(604))));
        }

        if ((((scratch.values[1323] != 0.0) && (scratch.values[1340] != 0.0)) && (!(scratch.values[1897] != 0.0))) && (!(scratch.values[1902] != 0.0))) {
            scratch.store_ad(1376, &AdValue::div(AdValue::mul(scratch.ad_value(1365), scratch.ad_value(1375)), AdValue::add(scratch.ad_value(1365), scratch.ad_value(1375))));
        }

        if ((((scratch.values[1323] != 0.0) && (scratch.values[1340] != 0.0)) && (!(scratch.values[1897] != 0.0))) && (!(scratch.values[1902] != 0.0))) {
            scratch.store_ad(1377, &AdValue::sqrt(AdValue::scale(AdValue::div(scratch.ad_value(1369), scratch.ad_value(1373)), 0.375)));
        }

        if ((((scratch.values[1323] != 0.0) && (scratch.values[1340] != 0.0)) && (!(scratch.values[1897] != 0.0))) && (!(scratch.values[1902] != 0.0))) {
            scratch.store_ad(1378, &AdValue::sub(AdValue::scale(AdValue::mul(scratch.ad_value(1370), scratch.ad_value(1373)), 2.0), scratch.ad_value(1372)));
        }

        if ((((scratch.values[1323] != 0.0) && (scratch.values[1340] != 0.0)) && (!(scratch.values[1897] != 0.0))) && (!(scratch.values[1902] != 0.0))) {
            scratch.store_ad(1379, &AdValue::add(AdValue::sub(AdValue::mul(AdValue::mul(scratch.ad_value(628), scratch.ad_value(1370)), scratch.ad_value(1373)), AdValue::mul(scratch.ad_value(628), scratch.ad_value(1372))), AdValue::scale(AdValue::mul(scratch.ad_value(1369), scratch.ad_value(1374)), 0.5)));
        }

        if ((((scratch.values[1323] != 0.0) && (scratch.values[1340] != 0.0)) && (!(scratch.values[1897] != 0.0))) && (!(scratch.values[1902] != 0.0))) {
            scratch.store_ad(1380, &AdValue::mul(AdValue::offset(scratch.ad_value(1378), (-1.0)), scratch.ad_value(1377)));
        }

        if ((((scratch.values[1323] != 0.0) && (scratch.values[1340] != 0.0)) && (!(scratch.values[1897] != 0.0))) && (!(scratch.values[1902] != 0.0))) {
            scratch.store_ad(1341, &AdValue::square(scratch.ad_value(1380)));
        }

        scratch.values[1904] = if (scratch.values[1380] > 0.0) { 1.0 } else { 0.0 };

        if (((((scratch.values[1323] != 0.0) && (scratch.values[1340] != 0.0)) && (!(scratch.values[1897] != 0.0))) && (!(scratch.values[1902] != 0.0))) && (scratch.values[1904] != 0.0)) {
            scratch.store_ad(1342, &AdValue::div_from_scalar(1.0, AdValue::offset(AdValue::scale(scratch.ad_value(1380), scratch.values[421]), 1.0)));
        }

        if (((((scratch.values[1323] != 0.0) && (scratch.values[1340] != 0.0)) && (!(scratch.values[1897] != 0.0))) && (!(scratch.values[1902] != 0.0))) && (!(scratch.values[1904] != 0.0))) {
            scratch.store_ad(1342, &AdValue::div_from_scalar(1.0, AdValue::sub_from_scalar(1.0, AdValue::scale(scratch.ad_value(1380), scratch.values[421]))));
        }

        scratch.values[1905] = if (((-scratch.values[1341]) + scratch.values[1379]) > (-230.25850929940458)) { 1.0 } else { 0.0 };

        if (((((scratch.values[1323] != 0.0) && (scratch.values[1340] != 0.0)) && (!(scratch.values[1897] != 0.0))) && (!(scratch.values[1902] != 0.0))) && (scratch.values[1905] != 0.0)) {
            scratch.store_ad(1359, &AdValue::exp(AdValue::sub(scratch.ad_value(1379), scratch.ad_value(1341))));
        }

        if (((((scratch.values[1323] != 0.0) && (scratch.values[1340] != 0.0)) && (!(scratch.values[1897] != 0.0))) && (!(scratch.values[1902] != 0.0))) && (!(scratch.values[1905] != 0.0))) {
            let assign36120_ad_e46190: AdValue = AdValue::div_from_scalar(1e-100, AdValue::offset(AdValue::mul(AdValue::sub_from_scalar((-230.25850929940458), AdValue::sub(scratch.ad_value(1379), scratch.ad_value(1341))), AdValue::offset(AdValue::scale(AdValue::mul(AdValue::sub_from_scalar((-230.25850929940458), AdValue::sub(scratch.ad_value(1379), scratch.ad_value(1341))), AdValue::offset(AdValue::scale(AdValue::sub_from_scalar((-230.25850929940458), AdValue::sub(scratch.ad_value(1379), scratch.ad_value(1341))), 0.3333333333333333), 1.0)), 0.5), 1.0)), 1.0));
            scratch.store_ad(1359, &assign36120_ad_e46190);
        }

        if ((((scratch.values[1323] != 0.0) && (scratch.values[1340] != 0.0)) && (!(scratch.values[1897] != 0.0))) && (!(scratch.values[1902] != 0.0))) {
            scratch.store_ad(1343, &AdValue::mul(AdValue::add(AdValue::add(AdValue::scale(scratch.ad_value(1342), 0.29214664), AdValue::scale(AdValue::square(scratch.ad_value(1342)), scratch.values[422])), AdValue::scale(AdValue::mul(AdValue::square(scratch.ad_value(1342)), scratch.ad_value(1342)), scratch.values[423])), scratch.ad_value(1359)));
        }

        scratch.values[1906] = if (scratch.values[1380] > 0.0) { 1.0 } else { 0.0 };

        if (((((scratch.values[1323] != 0.0) && (scratch.values[1340] != 0.0)) && (!(scratch.values[1897] != 0.0))) && (!(scratch.values[1902] != 0.0))) && (scratch.values[1906] != 0.0)) {
            scratch.values[1381] = scratch.values[1343];
            scratch.node_derivatives[1381] = scratch.node_derivatives[1343];
            scratch.branch_derivatives[1381] = scratch.branch_derivatives[1343];
        }

        scratch.values[1907] = if (scratch.values[1379] > (-230.25850929940458)) { 1.0 } else { 0.0 };

        if ((((((scratch.values[1323] != 0.0) && (scratch.values[1340] != 0.0)) && (!(scratch.values[1897] != 0.0))) && (!(scratch.values[1902] != 0.0))) && (!(scratch.values[1906] != 0.0))) && (scratch.values[1907] != 0.0)) {
            scratch.store_ad(1359, &AdValue::exp(scratch.ad_value(1379)));
        }

        if ((((((scratch.values[1323] != 0.0) && (scratch.values[1340] != 0.0)) && (!(scratch.values[1897] != 0.0))) && (!(scratch.values[1902] != 0.0))) && (!(scratch.values[1906] != 0.0))) && (!(scratch.values[1907] != 0.0))) {
            scratch.store_ad(1359, &AdValue::div_from_scalar(1e-100, AdValue::offset(AdValue::mul(AdValue::sub_from_scalar((-230.25850929940458), scratch.ad_value(1379)), AdValue::offset(AdValue::scale(AdValue::mul(AdValue::sub_from_scalar((-230.25850929940458), scratch.ad_value(1379)), AdValue::offset(AdValue::scale(AdValue::sub_from_scalar((-230.25850929940458), scratch.ad_value(1379)), 0.3333333333333333), 1.0)), 0.5), 1.0)), 1.0)));
        }

        if (((((scratch.values[1323] != 0.0) && (scratch.values[1340] != 0.0)) && (!(scratch.values[1897] != 0.0))) && (!(scratch.values[1902] != 0.0))) && (!(scratch.values[1906] != 0.0))) {
            scratch.store_ad(1381, &AdValue::sub(AdValue::scale(scratch.ad_value(1359), 2.0), scratch.ad_value(1343)));
        }

        if ((((scratch.values[1323] != 0.0) && (scratch.values[1340] != 0.0)) && (!(scratch.values[1897] != 0.0))) && (!(scratch.values[1902] != 0.0))) {
            scratch.store_ad(1382, &AdValue::scale(AdValue::div(AdValue::mul(scratch.ad_value(628), scratch.ad_value(1381)), scratch.ad_value(1377)), (1.772453850905516 * 0.5)));
        }

        if ((((scratch.values[1323] != 0.0) && (scratch.values[1340] != 0.0)) && (!(scratch.values[1897] != 0.0))) && (!(scratch.values[1902] != 0.0))) {
            scratch.store_ad(1368, &AdValue::mul(scratch.ad_value(557), AdValue::mul(AdValue::mul(scratch.ad_value(1367), scratch.ad_value(1382)), scratch.ad_value(1376))));
        }

        scratch.values[1908] = if (scratch.values[563] == 0.0) { 1.0 } else { 0.0 };

        if ((((scratch.values[1323] != 0.0) && (scratch.values[1340] != 0.0)) && (!(scratch.values[1897] != 0.0))) && (scratch.values[1908] != 0.0)) {
            scratch.values[1383] = 0.0;
            scratch.node_derivatives[1383] = [0.0; Instance::NODE_COUNT];
            scratch.branch_derivatives[1383] = [0.0; Instance::BRANCH_COUNT];
        }

        scratch.values[1909] = if (scratch.values[543] == 0.5) { 1.0 } else { 0.0 };

        if (((((scratch.values[1323] != 0.0) && (scratch.values[1340] != 0.0)) && (!(scratch.values[1897] != 0.0))) && (!(scratch.values[1908] != 0.0))) && (scratch.values[1909] != 0.0)) {
            scratch.store_ad(1359, &AdValue::sqrt(AdValue::mul(AdValue::sub(scratch.ad_value(540), scratch.ad_value(1357)), scratch.ad_value(622))));
        }

        if (((((scratch.values[1323] != 0.0) && (scratch.values[1340] != 0.0)) && (!(scratch.values[1897] != 0.0))) && (!(scratch.values[1908] != 0.0))) && (!(scratch.values[1909] != 0.0))) {
            scratch.store_ad(1359, &AdValue::pow(AdValue::mul(AdValue::sub(scratch.ad_value(540), scratch.ad_value(1357)), scratch.ad_value(622)), scratch.ad_value(543)));
        }

        if ((((scratch.values[1323] != 0.0) && (scratch.values[1340] != 0.0)) && (!(scratch.values[1897] != 0.0))) && (!(scratch.values[1908] != 0.0))) {
            scratch.store_ad(1384, &AdValue::mul(scratch.ad_value(604), AdValue::div(AdValue::mul(AdValue::sub(scratch.ad_value(540), scratch.ad_value(1357)), scratch.ad_value(619)), scratch.ad_value(1359))));
        }

        scratch.values[1910] = if (((((-scratch.values[634]) / scratch.values[1384])) as f64).abs() < 230.25850929940458) { 1.0 } else { 0.0 };

        if (((((scratch.values[1323] != 0.0) && (scratch.values[1340] != 0.0)) && (!(scratch.values[1897] != 0.0))) && (!(scratch.values[1908] != 0.0))) && (scratch.values[1910] != 0.0)) {
            scratch.store_ad(1359, &AdValue::exp(AdValue::div(AdValue::neg(scratch.ad_value(634)), scratch.ad_value(1384))));
        }

        scratch.values[1911] = if (((-scratch.values[634]) / scratch.values[1384]) < 0.0) { 1.0 } else { 0.0 };

        if ((((((scratch.values[1323] != 0.0) && (scratch.values[1340] != 0.0)) && (!(scratch.values[1897] != 0.0))) && (!(scratch.values[1908] != 0.0))) && (!(scratch.values[1910] != 0.0))) && (scratch.values[1911] != 0.0)) {
            let assign36310_ad_e46517: AdValue = AdValue::offset(AdValue::mul(AdValue::sub_from_scalar((-230.25850929940458), AdValue::div(AdValue::neg(scratch.ad_value(634)), scratch.ad_value(1384))), AdValue::offset(AdValue::scale(AdValue::mul(AdValue::sub_from_scalar((-230.25850929940458), AdValue::div(AdValue::neg(scratch.ad_value(634)), scratch.ad_value(1384))), AdValue::offset(AdValue::scale(AdValue::sub_from_scalar((-230.25850929940458), AdValue::div(AdValue::neg(scratch.ad_value(634)), scratch.ad_value(1384))), 0.3333333333333333), 1.0)), 0.5), 1.0)), 1.0);
            scratch.store_ad(1359, &AdValue::div_from_scalar(1e-100, assign36310_ad_e46517));
        }

        if ((((((scratch.values[1323] != 0.0) && (scratch.values[1340] != 0.0)) && (!(scratch.values[1897] != 0.0))) && (!(scratch.values[1908] != 0.0))) && (!(scratch.values[1910] != 0.0))) && (!(scratch.values[1911] != 0.0))) {
            let assign36320_ad_e46567: AdValue = AdValue::scale(AdValue::offset(AdValue::mul(AdValue::offset(AdValue::div(AdValue::neg(scratch.ad_value(634)), scratch.ad_value(1384)), (-230.25850929940458)), AdValue::offset(AdValue::scale(AdValue::mul(AdValue::offset(AdValue::div(AdValue::neg(scratch.ad_value(634)), scratch.ad_value(1384)), (-230.25850929940458)), AdValue::offset(AdValue::scale(AdValue::offset(AdValue::div(AdValue::neg(scratch.ad_value(634)), scratch.ad_value(1384)), (-230.25850929940458)), 0.3333333333333333), 1.0)), 0.5), 1.0)), 1.0), 1e100);
            scratch.store_ad(1359, &assign36320_ad_e46567);
        }

        if ((((scratch.values[1323] != 0.0) && (scratch.values[1340] != 0.0)) && (!(scratch.values[1897] != 0.0))) && (!(scratch.values[1908] != 0.0))) {
            scratch.store_ad(1383, &AdValue::mul(scratch.ad_value(563), AdValue::mul(AdValue::mul(AdValue::mul(scratch.ad_value(519), scratch.ad_value(1384)), scratch.ad_value(1384)), scratch.ad_value(1359))));
        }

        scratch.values[1912] = if (scratch.values[572] > 1000.0) { 1.0 } else { 0.0 };

        if ((((scratch.values[1323] != 0.0) && (scratch.values[1340] != 0.0)) && (!(scratch.values[1897] != 0.0))) && (scratch.values[1912] != 0.0)) {
            scratch.values[1385] = 1.0;
            scratch.node_derivatives[1385] = [0.0; Instance::NODE_COUNT];
            scratch.branch_derivatives[1385] = [0.0; Instance::BRANCH_COUNT];
        }

        scratch.values[1913] = if (scratch.values[1358] > ((-scratch.values[493]) * scratch.values[572])) { 1.0 } else { 0.0 };

        scratch.values[1914] = if (scratch.values[575] == 4.0) { 1.0 } else { 0.0 };

        if ((((((scratch.values[1323] != 0.0) && (scratch.values[1340] != 0.0)) && (!(scratch.values[1897] != 0.0))) && (!(scratch.values[1912] != 0.0))) && (scratch.values[1913] != 0.0)) && (scratch.values[1914] != 0.0)) {
            scratch.store_ad(1359, &AdValue::mul(AdValue::mul(AdValue::mul(AdValue::mul(scratch.ad_value(1358), scratch.ad_value(640)), AdValue::mul(scratch.ad_value(1358), scratch.ad_value(640))), AdValue::mul(scratch.ad_value(1358), scratch.ad_value(640))), AdValue::mul(scratch.ad_value(1358), scratch.ad_value(640))));
        }

        if ((((((scratch.values[1323] != 0.0) && (scratch.values[1340] != 0.0)) && (!(scratch.values[1897] != 0.0))) && (!(scratch.values[1912] != 0.0))) && (scratch.values[1913] != 0.0)) && (!(scratch.values[1914] != 0.0))) {
            scratch.store_ad(1359, &AdValue::pow(AdValue::abs(AdValue::mul(scratch.ad_value(1358), scratch.ad_value(640))), scratch.ad_value(575)));
        }

        if (((((scratch.values[1323] != 0.0) && (scratch.values[1340] != 0.0)) && (!(scratch.values[1897] != 0.0))) && (!(scratch.values[1912] != 0.0))) && (scratch.values[1913] != 0.0)) {
            scratch.store_ad(1385, &AdValue::div_from_scalar(1.0, AdValue::sub_from_scalar(1.0, scratch.ad_value(1359))));
        }

        if (((((scratch.values[1323] != 0.0) && (scratch.values[1340] != 0.0)) && (!(scratch.values[1897] != 0.0))) && (!(scratch.values[1912] != 0.0))) && (!(scratch.values[1913] != 0.0))) {
            scratch.store_ad(1385, &AdValue::add(scratch.ad_value(637), AdValue::mul(AdValue::add(scratch.ad_value(1358), AdValue::scale(scratch.ad_value(572), scratch.values[493])), scratch.ad_value(643))));
        }

        if (((scratch.values[1323] != 0.0) && (scratch.values[1340] != 0.0)) && (!(scratch.values[1897] != 0.0))) {
            scratch.store_ad(1387, &AdValue::mul(AdValue::add(AdValue::add(AdValue::add(scratch.ad_value(1360), scratch.ad_value(1361)), scratch.ad_value(1368)), scratch.ad_value(1383)), scratch.ad_value(1385)));
        }

        scratch.values[1915] = if (scratch.values[719] == 0.0) { 1.0 } else { 0.0 };

        if (((scratch.values[1323] != 0.0) && (scratch.values[1340] != 0.0)) && (scratch.values[1915] != 0.0)) {
            scratch.values[1388] = 0.0;
            scratch.node_derivatives[1388] = [0.0; Instance::NODE_COUNT];
            scratch.branch_derivatives[1388] = [0.0; Instance::BRANCH_COUNT];
        }

        scratch.values[1916] = if (scratch.values[602] == 0.5) { 1.0 } else { 0.0 };

        if ((((scratch.values[1323] != 0.0) && (scratch.values[1340] != 0.0)) && (!(scratch.values[1915] != 0.0))) && (scratch.values[1916] != 0.0)) {
            scratch.store_ad(1359, &AdValue::sqrt(AdValue::sub_from_scalar(1.0, AdValue::mul(scratch.ad_value(1351), scratch.ad_value(599)))));
        }

        if ((((scratch.values[1323] != 0.0) && (scratch.values[1340] != 0.0)) && (!(scratch.values[1915] != 0.0))) && (!(scratch.values[1916] != 0.0))) {
            scratch.store_ad(1359, &AdValue::pow(AdValue::sub_from_scalar(1.0, AdValue::mul(scratch.ad_value(1351), scratch.ad_value(599))), scratch.ad_value(602)));
        }

        if (((scratch.values[1323] != 0.0) && (scratch.values[1340] != 0.0)) && (!(scratch.values[1915] != 0.0))) {
            scratch.store_ad(1360, &AdValue::mul(scratch.ad_value(590), scratch.ad_value(1350)));
        }

        scratch.values[1917] = if ((scratch.values[555] == 0.0) && (scratch.values[558] == 0.0)) { 1.0 } else { 0.0 };

        if ((((scratch.values[1323] != 0.0) && (scratch.values[1340] != 0.0)) && (!(scratch.values[1915] != 0.0))) && (scratch.values[1917] != 0.0)) {
            scratch.values[1361] = 0.0;
            scratch.node_derivatives[1361] = [0.0; Instance::NODE_COUNT];
            scratch.branch_derivatives[1361] = [0.0; Instance::BRANCH_COUNT];
        }

        if ((((scratch.values[1323] != 0.0) && (scratch.values[1340] != 0.0)) && (!(scratch.values[1915] != 0.0))) && (!(scratch.values[1917] != 0.0))) {
            scratch.store_ad(1362, &AdValue::sub(scratch.ad_value(596), scratch.ad_value(1356)));
        }

        if ((((scratch.values[1323] != 0.0) && (scratch.values[1340] != 0.0)) && (!(scratch.values[1915] != 0.0))) && (!(scratch.values[1917] != 0.0))) {
            scratch.store_ad(1363, &AdValue::sub_from_scalar(1.0, AdValue::sqrt(AdValue::sub_from_scalar(1.0, AdValue::div(scratch.ad_value(1354), scratch.ad_value(1362))))));
        }

        scratch.values[1918] = if (scratch.values[544] == 0.5) { 1.0 } else { 0.0 };

        if (((((scratch.values[1323] != 0.0) && (scratch.values[1340] != 0.0)) && (!(scratch.values[1915] != 0.0))) && (!(scratch.values[1917] != 0.0))) && (scratch.values[1918] != 0.0)) {
            scratch.values[1364] = 0.0;
            scratch.node_derivatives[1364] = [0.0; Instance::NODE_COUNT];
            scratch.branch_derivatives[1364] = [0.0; Instance::BRANCH_COUNT];
        }

        if (((((scratch.values[1323] != 0.0) && (scratch.values[1340] != 0.0)) && (!(scratch.values[1915] != 0.0))) && (!(scratch.values[1917] != 0.0))) && (!(scratch.values[1918] != 0.0))) {
            scratch.store_ad(1364, &AdValue::mul(AdValue::add(AdValue::div(AdValue::mul(AdValue::square(scratch.ad_value(1363)), AdValue::ln(scratch.ad_value(1363))), AdValue::sub_from_scalar(1.0, scratch.ad_value(1363))), scratch.ad_value(1363)), AdValue::sub_from_scalar(1.0, AdValue::scale(scratch.ad_value(544), 2.0))));
        }

        if ((((scratch.values[1323] != 0.0) && (scratch.values[1340] != 0.0)) && (!(scratch.values[1915] != 0.0))) && (!(scratch.values[1917] != 0.0))) {
            scratch.store_ad(1365, &AdValue::add(scratch.ad_value(1363), scratch.ad_value(1364)));
        }

        scratch.values[1919] = if (scratch.values[544] == 0.5) { 1.0 } else { 0.0 };

        if (((((scratch.values[1323] != 0.0) && (scratch.values[1340] != 0.0)) && (!(scratch.values[1915] != 0.0))) && (!(scratch.values[1917] != 0.0))) && (scratch.values[1919] != 0.0)) {
            scratch.store_ad(1359, &AdValue::sqrt(AdValue::mul(scratch.ad_value(1362), scratch.ad_value(623))));
        }

        if (((((scratch.values[1323] != 0.0) && (scratch.values[1340] != 0.0)) && (!(scratch.values[1915] != 0.0))) && (!(scratch.values[1917] != 0.0))) && (!(scratch.values[1919] != 0.0))) {
            scratch.store_ad(1359, &AdValue::pow(AdValue::mul(scratch.ad_value(1362), scratch.ad_value(623)), scratch.ad_value(544)));
        }

        if ((((scratch.values[1323] != 0.0) && (scratch.values[1340] != 0.0)) && (!(scratch.values[1915] != 0.0))) && (!(scratch.values[1917] != 0.0))) {
            scratch.store_ad(1366, &AdValue::mul(scratch.ad_value(617), scratch.ad_value(1359)));
        }

        if ((((scratch.values[1323] != 0.0) && (scratch.values[1340] != 0.0)) && (!(scratch.values[1915] != 0.0))) && (!(scratch.values[1917] != 0.0))) {
            scratch.store_ad(1367, &AdValue::mul(scratch.ad_value(587), AdValue::mul(AdValue::offset(scratch.ad_value(1353), (-1.0)), scratch.ad_value(1366))));
        }

        if ((((scratch.values[1323] != 0.0) && (scratch.values[1340] != 0.0)) && (!(scratch.values[1915] != 0.0))) && (!(scratch.values[1917] != 0.0))) {
            scratch.store_ad(1361, &AdValue::mul(scratch.ad_value(555), AdValue::mul(scratch.ad_value(1367), scratch.ad_value(1365))));
        }

        scratch.values[1920] = if (scratch.values[558] == 0.0) { 1.0 } else { 0.0 };

        if ((((scratch.values[1323] != 0.0) && (scratch.values[1340] != 0.0)) && (!(scratch.values[1915] != 0.0))) && (scratch.values[1920] != 0.0)) {
            scratch.values[1368] = 0.0;
            scratch.node_derivatives[1368] = [0.0; Instance::NODE_COUNT];
            scratch.branch_derivatives[1368] = [0.0; Instance::BRANCH_COUNT];
        }

        if ((((scratch.values[1323] != 0.0) && (scratch.values[1340] != 0.0)) && (!(scratch.values[1915] != 0.0))) && (!(scratch.values[1920] != 0.0))) {
            scratch.store_ad(1369, &AdValue::mul(scratch.ad_value(632), AdValue::div(AdValue::mul(scratch.ad_value(1366), scratch.ad_value(602)), scratch.ad_value(1362))));
        }

        if ((((scratch.values[1323] != 0.0) && (scratch.values[1340] != 0.0)) && (!(scratch.values[1915] != 0.0))) && (!(scratch.values[1920] != 0.0))) {
            scratch.store_ad(1370, &AdValue::div(AdValue::scale(scratch.ad_value(629), 0.666666666666667), scratch.ad_value(1369)));
        }

        if ((((scratch.values[1323] != 0.0) && (scratch.values[1340] != 0.0)) && (!(scratch.values[1915] != 0.0))) && (!(scratch.values[1920] != 0.0))) {
            scratch.store_ad(1371, &AdValue::square(scratch.ad_value(1370)));
        }

        if ((((scratch.values[1323] != 0.0) && (scratch.values[1340] != 0.0)) && (!(scratch.values[1915] != 0.0))) && (!(scratch.values[1920] != 0.0))) {
            scratch.store_ad(1372, &AdValue::sqrt(AdValue::div(AdValue::square(scratch.ad_value(1371)), AdValue::offset(AdValue::square(scratch.ad_value(1371)), 1.0))));
        }

        if ((((scratch.values[1323] != 0.0) && (scratch.values[1340] != 0.0)) && (!(scratch.values[1915] != 0.0))) && (!(scratch.values[1920] != 0.0))) {
            scratch.store_ad(1373, &AdValue::sqrt(AdValue::abs(scratch.ad_value(1372))));
        }

        if ((((scratch.values[1323] != 0.0) && (scratch.values[1340] != 0.0)) && (!(scratch.values[1915] != 0.0))) && (!(scratch.values[1920] != 0.0))) {
            scratch.store_ad(1374, &AdValue::mul(scratch.ad_value(1372), scratch.ad_value(1373)));
        }

        scratch.values[1921] = if (((-scratch.values[544]) * scratch.values[605]) == (-1.0)) { 1.0 } else { 0.0 };

        if (((((scratch.values[1323] != 0.0) && (scratch.values[1340] != 0.0)) && (!(scratch.values[1915] != 0.0))) && (!(scratch.values[1920] != 0.0))) && (scratch.values[1921] != 0.0)) {
            scratch.store_ad(1375, &AdValue::div_from_scalar(1.0, AdValue::offset(AdValue::mul(scratch.ad_value(1369), scratch.ad_value(1374)), 1.0)));
        }

        if (((((scratch.values[1323] != 0.0) && (scratch.values[1340] != 0.0)) && (!(scratch.values[1915] != 0.0))) && (!(scratch.values[1920] != 0.0))) && (!(scratch.values[1921] != 0.0))) {
            scratch.store_ad(1375, &AdValue::pow(AdValue::offset(AdValue::mul(scratch.ad_value(1369), scratch.ad_value(1374)), 1.0), AdValue::mul(AdValue::neg(scratch.ad_value(544)), scratch.ad_value(605))));
        }

        if ((((scratch.values[1323] != 0.0) && (scratch.values[1340] != 0.0)) && (!(scratch.values[1915] != 0.0))) && (!(scratch.values[1920] != 0.0))) {
            scratch.store_ad(1376, &AdValue::div(AdValue::mul(scratch.ad_value(1365), scratch.ad_value(1375)), AdValue::add(scratch.ad_value(1365), scratch.ad_value(1375))));
        }

        if ((((scratch.values[1323] != 0.0) && (scratch.values[1340] != 0.0)) && (!(scratch.values[1915] != 0.0))) && (!(scratch.values[1920] != 0.0))) {
            scratch.store_ad(1377, &AdValue::sqrt(AdValue::scale(AdValue::div(scratch.ad_value(1369), scratch.ad_value(1373)), 0.375)));
        }

        if ((((scratch.values[1323] != 0.0) && (scratch.values[1340] != 0.0)) && (!(scratch.values[1915] != 0.0))) && (!(scratch.values[1920] != 0.0))) {
            scratch.store_ad(1378, &AdValue::sub(AdValue::scale(AdValue::mul(scratch.ad_value(1370), scratch.ad_value(1373)), 2.0), scratch.ad_value(1372)));
        }

        if ((((scratch.values[1323] != 0.0) && (scratch.values[1340] != 0.0)) && (!(scratch.values[1915] != 0.0))) && (!(scratch.values[1920] != 0.0))) {
            scratch.store_ad(1379, &AdValue::add(AdValue::sub(AdValue::mul(AdValue::mul(scratch.ad_value(629), scratch.ad_value(1370)), scratch.ad_value(1373)), AdValue::mul(scratch.ad_value(629), scratch.ad_value(1372))), AdValue::scale(AdValue::mul(scratch.ad_value(1369), scratch.ad_value(1374)), 0.5)));
        }

        if ((((scratch.values[1323] != 0.0) && (scratch.values[1340] != 0.0)) && (!(scratch.values[1915] != 0.0))) && (!(scratch.values[1920] != 0.0))) {
            scratch.store_ad(1380, &AdValue::mul(AdValue::offset(scratch.ad_value(1378), (-1.0)), scratch.ad_value(1377)));
        }

        if ((((scratch.values[1323] != 0.0) && (scratch.values[1340] != 0.0)) && (!(scratch.values[1915] != 0.0))) && (!(scratch.values[1920] != 0.0))) {
            scratch.store_ad(1341, &AdValue::square(scratch.ad_value(1380)));
        }

        scratch.values[1922] = if (scratch.values[1380] > 0.0) { 1.0 } else { 0.0 };

        if (((((scratch.values[1323] != 0.0) && (scratch.values[1340] != 0.0)) && (!(scratch.values[1915] != 0.0))) && (!(scratch.values[1920] != 0.0))) && (scratch.values[1922] != 0.0)) {
            scratch.store_ad(1342, &AdValue::div_from_scalar(1.0, AdValue::offset(AdValue::scale(scratch.ad_value(1380), scratch.values[421]), 1.0)));
        }

        if (((((scratch.values[1323] != 0.0) && (scratch.values[1340] != 0.0)) && (!(scratch.values[1915] != 0.0))) && (!(scratch.values[1920] != 0.0))) && (!(scratch.values[1922] != 0.0))) {
            scratch.store_ad(1342, &AdValue::div_from_scalar(1.0, AdValue::sub_from_scalar(1.0, AdValue::scale(scratch.ad_value(1380), scratch.values[421]))));
        }

        scratch.values[1923] = if (((-scratch.values[1341]) + scratch.values[1379]) > (-230.25850929940458)) { 1.0 } else { 0.0 };

        if (((((scratch.values[1323] != 0.0) && (scratch.values[1340] != 0.0)) && (!(scratch.values[1915] != 0.0))) && (!(scratch.values[1920] != 0.0))) && (scratch.values[1923] != 0.0)) {
            scratch.store_ad(1359, &AdValue::exp(AdValue::sub(scratch.ad_value(1379), scratch.ad_value(1341))));
        }

        if (((((scratch.values[1323] != 0.0) && (scratch.values[1340] != 0.0)) && (!(scratch.values[1915] != 0.0))) && (!(scratch.values[1920] != 0.0))) && (!(scratch.values[1923] != 0.0))) {
            let assign36870_ad_e47396: AdValue = AdValue::div_from_scalar(1e-100, AdValue::offset(AdValue::mul(AdValue::sub_from_scalar((-230.25850929940458), AdValue::sub(scratch.ad_value(1379), scratch.ad_value(1341))), AdValue::offset(AdValue::scale(AdValue::mul(AdValue::sub_from_scalar((-230.25850929940458), AdValue::sub(scratch.ad_value(1379), scratch.ad_value(1341))), AdValue::offset(AdValue::scale(AdValue::sub_from_scalar((-230.25850929940458), AdValue::sub(scratch.ad_value(1379), scratch.ad_value(1341))), 0.3333333333333333), 1.0)), 0.5), 1.0)), 1.0));
            scratch.store_ad(1359, &assign36870_ad_e47396);
        }

        if ((((scratch.values[1323] != 0.0) && (scratch.values[1340] != 0.0)) && (!(scratch.values[1915] != 0.0))) && (!(scratch.values[1920] != 0.0))) {
            scratch.store_ad(1343, &AdValue::mul(AdValue::add(AdValue::add(AdValue::scale(scratch.ad_value(1342), 0.29214664), AdValue::scale(AdValue::square(scratch.ad_value(1342)), scratch.values[422])), AdValue::scale(AdValue::mul(AdValue::square(scratch.ad_value(1342)), scratch.ad_value(1342)), scratch.values[423])), scratch.ad_value(1359)));
        }

        scratch.values[1924] = if (scratch.values[1380] > 0.0) { 1.0 } else { 0.0 };

        if (((((scratch.values[1323] != 0.0) && (scratch.values[1340] != 0.0)) && (!(scratch.values[1915] != 0.0))) && (!(scratch.values[1920] != 0.0))) && (scratch.values[1924] != 0.0)) {
            scratch.values[1381] = scratch.values[1343];
            scratch.node_derivatives[1381] = scratch.node_derivatives[1343];
            scratch.branch_derivatives[1381] = scratch.branch_derivatives[1343];
        }

        scratch.values[1925] = if (scratch.values[1379] > (-230.25850929940458)) { 1.0 } else { 0.0 };

        if ((((((scratch.values[1323] != 0.0) && (scratch.values[1340] != 0.0)) && (!(scratch.values[1915] != 0.0))) && (!(scratch.values[1920] != 0.0))) && (!(scratch.values[1924] != 0.0))) && (scratch.values[1925] != 0.0)) {
            scratch.store_ad(1359, &AdValue::exp(scratch.ad_value(1379)));
        }

        if ((((((scratch.values[1323] != 0.0) && (scratch.values[1340] != 0.0)) && (!(scratch.values[1915] != 0.0))) && (!(scratch.values[1920] != 0.0))) && (!(scratch.values[1924] != 0.0))) && (!(scratch.values[1925] != 0.0))) {
            scratch.store_ad(1359, &AdValue::div_from_scalar(1e-100, AdValue::offset(AdValue::mul(AdValue::sub_from_scalar((-230.25850929940458), scratch.ad_value(1379)), AdValue::offset(AdValue::scale(AdValue::mul(AdValue::sub_from_scalar((-230.25850929940458), scratch.ad_value(1379)), AdValue::offset(AdValue::scale(AdValue::sub_from_scalar((-230.25850929940458), scratch.ad_value(1379)), 0.3333333333333333), 1.0)), 0.5), 1.0)), 1.0)));
        }

        if (((((scratch.values[1323] != 0.0) && (scratch.values[1340] != 0.0)) && (!(scratch.values[1915] != 0.0))) && (!(scratch.values[1920] != 0.0))) && (!(scratch.values[1924] != 0.0))) {
            scratch.store_ad(1381, &AdValue::sub(AdValue::scale(scratch.ad_value(1359), 2.0), scratch.ad_value(1343)));
        }

        if ((((scratch.values[1323] != 0.0) && (scratch.values[1340] != 0.0)) && (!(scratch.values[1915] != 0.0))) && (!(scratch.values[1920] != 0.0))) {
            scratch.store_ad(1382, &AdValue::scale(AdValue::div(AdValue::mul(scratch.ad_value(629), scratch.ad_value(1381)), scratch.ad_value(1377)), (1.772453850905516 * 0.5)));
        }

        if ((((scratch.values[1323] != 0.0) && (scratch.values[1340] != 0.0)) && (!(scratch.values[1915] != 0.0))) && (!(scratch.values[1920] != 0.0))) {
            scratch.store_ad(1368, &AdValue::mul(scratch.ad_value(558), AdValue::mul(AdValue::mul(scratch.ad_value(1367), scratch.ad_value(1382)), scratch.ad_value(1376))));
        }

        scratch.values[1926] = if (scratch.values[564] == 0.0) { 1.0 } else { 0.0 };

        if ((((scratch.values[1323] != 0.0) && (scratch.values[1340] != 0.0)) && (!(scratch.values[1915] != 0.0))) && (scratch.values[1926] != 0.0)) {
            scratch.values[1383] = 0.0;
            scratch.node_derivatives[1383] = [0.0; Instance::NODE_COUNT];
            scratch.branch_derivatives[1383] = [0.0; Instance::BRANCH_COUNT];
        }

        scratch.values[1927] = if (scratch.values[544] == 0.5) { 1.0 } else { 0.0 };

        if (((((scratch.values[1323] != 0.0) && (scratch.values[1340] != 0.0)) && (!(scratch.values[1915] != 0.0))) && (!(scratch.values[1926] != 0.0))) && (scratch.values[1927] != 0.0)) {
            scratch.store_ad(1359, &AdValue::sqrt(AdValue::mul(AdValue::sub(scratch.ad_value(541), scratch.ad_value(1357)), scratch.ad_value(623))));
        }

        if (((((scratch.values[1323] != 0.0) && (scratch.values[1340] != 0.0)) && (!(scratch.values[1915] != 0.0))) && (!(scratch.values[1926] != 0.0))) && (!(scratch.values[1927] != 0.0))) {
            scratch.store_ad(1359, &AdValue::pow(AdValue::mul(AdValue::sub(scratch.ad_value(541), scratch.ad_value(1357)), scratch.ad_value(623)), scratch.ad_value(544)));
        }

        if ((((scratch.values[1323] != 0.0) && (scratch.values[1340] != 0.0)) && (!(scratch.values[1915] != 0.0))) && (!(scratch.values[1926] != 0.0))) {
            scratch.store_ad(1384, &AdValue::mul(scratch.ad_value(605), AdValue::div(AdValue::mul(AdValue::sub(scratch.ad_value(541), scratch.ad_value(1357)), scratch.ad_value(620)), scratch.ad_value(1359))));
        }

        scratch.values[1928] = if (((((-scratch.values[635]) / scratch.values[1384])) as f64).abs() < 230.25850929940458) { 1.0 } else { 0.0 };

        if (((((scratch.values[1323] != 0.0) && (scratch.values[1340] != 0.0)) && (!(scratch.values[1915] != 0.0))) && (!(scratch.values[1926] != 0.0))) && (scratch.values[1928] != 0.0)) {
            scratch.store_ad(1359, &AdValue::exp(AdValue::div(AdValue::neg(scratch.ad_value(635)), scratch.ad_value(1384))));
        }

        scratch.values[1929] = if (((-scratch.values[635]) / scratch.values[1384]) < 0.0) { 1.0 } else { 0.0 };

        if ((((((scratch.values[1323] != 0.0) && (scratch.values[1340] != 0.0)) && (!(scratch.values[1915] != 0.0))) && (!(scratch.values[1926] != 0.0))) && (!(scratch.values[1928] != 0.0))) && (scratch.values[1929] != 0.0)) {
            let assign37060_ad_e47723: AdValue = AdValue::offset(AdValue::mul(AdValue::sub_from_scalar((-230.25850929940458), AdValue::div(AdValue::neg(scratch.ad_value(635)), scratch.ad_value(1384))), AdValue::offset(AdValue::scale(AdValue::mul(AdValue::sub_from_scalar((-230.25850929940458), AdValue::div(AdValue::neg(scratch.ad_value(635)), scratch.ad_value(1384))), AdValue::offset(AdValue::scale(AdValue::sub_from_scalar((-230.25850929940458), AdValue::div(AdValue::neg(scratch.ad_value(635)), scratch.ad_value(1384))), 0.3333333333333333), 1.0)), 0.5), 1.0)), 1.0);
            scratch.store_ad(1359, &AdValue::div_from_scalar(1e-100, assign37060_ad_e47723));
        }

        if ((((((scratch.values[1323] != 0.0) && (scratch.values[1340] != 0.0)) && (!(scratch.values[1915] != 0.0))) && (!(scratch.values[1926] != 0.0))) && (!(scratch.values[1928] != 0.0))) && (!(scratch.values[1929] != 0.0))) {
            let assign37070_ad_e47773: AdValue = AdValue::scale(AdValue::offset(AdValue::mul(AdValue::offset(AdValue::div(AdValue::neg(scratch.ad_value(635)), scratch.ad_value(1384)), (-230.25850929940458)), AdValue::offset(AdValue::scale(AdValue::mul(AdValue::offset(AdValue::div(AdValue::neg(scratch.ad_value(635)), scratch.ad_value(1384)), (-230.25850929940458)), AdValue::offset(AdValue::scale(AdValue::offset(AdValue::div(AdValue::neg(scratch.ad_value(635)), scratch.ad_value(1384)), (-230.25850929940458)), 0.3333333333333333), 1.0)), 0.5), 1.0)), 1.0), 1e100);
            scratch.store_ad(1359, &assign37070_ad_e47773);
        }

        if ((((scratch.values[1323] != 0.0) && (scratch.values[1340] != 0.0)) && (!(scratch.values[1915] != 0.0))) && (!(scratch.values[1926] != 0.0))) {
            scratch.store_ad(1383, &AdValue::mul(scratch.ad_value(564), AdValue::mul(AdValue::mul(AdValue::mul(scratch.ad_value(519), scratch.ad_value(1384)), scratch.ad_value(1384)), scratch.ad_value(1359))));
        }

        scratch.values[1930] = if (scratch.values[573] > 1000.0) { 1.0 } else { 0.0 };

        if ((((scratch.values[1323] != 0.0) && (scratch.values[1340] != 0.0)) && (!(scratch.values[1915] != 0.0))) && (scratch.values[1930] != 0.0)) {
            scratch.values[1385] = 1.0;
            scratch.node_derivatives[1385] = [0.0; Instance::NODE_COUNT];
            scratch.branch_derivatives[1385] = [0.0; Instance::BRANCH_COUNT];
        }

        scratch.values[1931] = if (scratch.values[1358] > ((-scratch.values[493]) * scratch.values[573])) { 1.0 } else { 0.0 };

        scratch.values[1932] = if (scratch.values[576] == 4.0) { 1.0 } else { 0.0 };

        if ((((((scratch.values[1323] != 0.0) && (scratch.values[1340] != 0.0)) && (!(scratch.values[1915] != 0.0))) && (!(scratch.values[1930] != 0.0))) && (scratch.values[1931] != 0.0)) && (scratch.values[1932] != 0.0)) {
            scratch.store_ad(1359, &AdValue::mul(AdValue::mul(AdValue::mul(AdValue::mul(scratch.ad_value(1358), scratch.ad_value(641)), AdValue::mul(scratch.ad_value(1358), scratch.ad_value(641))), AdValue::mul(scratch.ad_value(1358), scratch.ad_value(641))), AdValue::mul(scratch.ad_value(1358), scratch.ad_value(641))));
        }

        if ((((((scratch.values[1323] != 0.0) && (scratch.values[1340] != 0.0)) && (!(scratch.values[1915] != 0.0))) && (!(scratch.values[1930] != 0.0))) && (scratch.values[1931] != 0.0)) && (!(scratch.values[1932] != 0.0))) {
            scratch.store_ad(1359, &AdValue::pow(AdValue::abs(AdValue::mul(scratch.ad_value(1358), scratch.ad_value(641))), scratch.ad_value(576)));
        }

    }
}
