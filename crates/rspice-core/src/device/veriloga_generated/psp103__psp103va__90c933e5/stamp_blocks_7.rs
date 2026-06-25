#![allow(dead_code, unused_assignments, unused_imports, unused_parens, unused_variables)]

use super::{AdValue, GeneratedEvalContext, GeneratedReactiveStamper, GeneratedStamper, Scratch, THERMAL_VOLTAGE_PER_K};
use super::super::state::Instance;

impl Instance {

    pub(super) fn stamp_transient_block_28(
        &mut self,
        ctx: &GeneratedEvalContext<'_>,
        stamper: &mut GeneratedStamper<'_>,
        scratch: &mut Scratch,
    ) {
        let _ = stamper;
        if (((((scratch.values[1323] != 0.0) && (scratch.values[1340] != 0.0)) && (!(scratch.values[1915] != 0.0))) && (!(scratch.values[1930] != 0.0))) && (scratch.values[1931] != 0.0)) {
            scratch.store_ad(1385, &AdValue::div_from_scalar(1.0, AdValue::sub_from_scalar(1.0, scratch.ad_value(1359))));
        }

        if (((((scratch.values[1323] != 0.0) && (scratch.values[1340] != 0.0)) && (!(scratch.values[1915] != 0.0))) && (!(scratch.values[1930] != 0.0))) && (!(scratch.values[1931] != 0.0))) {
            scratch.store_ad(1385, &AdValue::add(scratch.ad_value(638), AdValue::mul(AdValue::add(scratch.ad_value(1358), AdValue::scale(scratch.ad_value(573), scratch.values[493])), scratch.ad_value(644))));
        }

        if (((scratch.values[1323] != 0.0) && (scratch.values[1340] != 0.0)) && (!(scratch.values[1915] != 0.0))) {
            scratch.store_ad(1388, &AdValue::mul(AdValue::add(AdValue::add(AdValue::add(scratch.ad_value(1360), scratch.ad_value(1361)), scratch.ad_value(1368)), scratch.ad_value(1383)), scratch.ad_value(1385)));
        }

        if ((scratch.values[1323] != 0.0) && (scratch.values[1340] != 0.0)) {
            scratch.store_ad(509, &AdValue::add(AdValue::add(AdValue::mul(scratch.ad_value(717), scratch.ad_value(1386)), AdValue::mul(scratch.ad_value(718), scratch.ad_value(1387))), AdValue::mul(scratch.ad_value(719), scratch.ad_value(1388))));
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

        scratch.values[1933] = if !(((scratch.values[717] == 0.0) && (scratch.values[718] == 0.0)) && (scratch.values[719] == 0.0)) { 1.0 } else { 0.0 };

        if (((scratch.values[1323] != 0.0) && (scratch.values[1340] != 0.0)) && (scratch.values[1933] != 0.0)) {
            scratch.store_ad(1344, &AdValue::mul(AdValue::scale(scratch.ad_value(728), 4.0), scratch.ad_value(728)));
        }

        if (((scratch.values[1323] != 0.0) && (scratch.values[1340] != 0.0)) && (scratch.values[1933] != 0.0)) {
            scratch.store_ad(1345, &AdValue::div(scratch.ad_value(728), scratch.ad_value(729)));
        }

        if (((scratch.values[1323] != 0.0) && (scratch.values[1340] != 0.0)) && (scratch.values[1933] != 0.0)) {
            scratch.store_ad(1346, &AdValue::add(scratch.ad_value(520), AdValue::mul(scratch.ad_value(728), scratch.ad_value(1345))));
        }

        if (((scratch.values[1323] != 0.0) && (scratch.values[1340] != 0.0)) && (scratch.values[1933] != 0.0)) {
            scratch.store_ad(1347, &AdValue::add(scratch.ad_value(729), scratch.ad_value(1346)));
        }

        if (((scratch.values[1323] != 0.0) && (scratch.values[1340] != 0.0)) && (scratch.values[1933] != 0.0)) {
            scratch.store_ad(1348, &AdValue::sub(scratch.ad_value(729), scratch.ad_value(1346)));
        }

        if (((scratch.values[1323] != 0.0) && (scratch.values[1340] != 0.0)) && (scratch.values[1933] != 0.0)) {
            scratch.store_ad(1349, &AdValue::sqrt(AdValue::add(AdValue::square(scratch.ad_value(1348)), scratch.ad_value(1344))));
        }

        if (((scratch.values[1323] != 0.0) && (scratch.values[1340] != 0.0)) && (scratch.values[1933] != 0.0)) {
            scratch.store_ad(1351, &AdValue::scale(AdValue::div(AdValue::mul(scratch.ad_value(520), scratch.ad_value(729)), AdValue::add(scratch.ad_value(1347), scratch.ad_value(1349))), 2.0));
        }

        scratch.values[1934] = if (scratch.values[520] < scratch.values[725]) { 1.0 } else { 0.0 };

        scratch.values[1935] = if ((((0.5 * (scratch.values[520] * scratch.values[420]))) as f64).abs() < 230.25850929940458) { 1.0 } else { 0.0 };

        if (((((scratch.values[1323] != 0.0) && (scratch.values[1340] != 0.0)) && (scratch.values[1933] != 0.0)) && (scratch.values[1934] != 0.0)) && (scratch.values[1935] != 0.0)) {
            scratch.store_ad(1353, &AdValue::exp(AdValue::scale(scratch.ad_value(520), (scratch.values[420] * 0.5))));
        }

        scratch.values[1936] = if ((0.5 * (scratch.values[520] * scratch.values[420])) < 0.0) { 1.0 } else { 0.0 };

        if ((((((scratch.values[1323] != 0.0) && (scratch.values[1340] != 0.0)) && (scratch.values[1933] != 0.0)) && (scratch.values[1934] != 0.0)) && (!(scratch.values[1935] != 0.0))) && (scratch.values[1936] != 0.0)) {
            let assign37330_ad_e48136: AdValue = AdValue::div_from_scalar(1e-100, AdValue::offset(AdValue::mul(AdValue::sub_from_scalar((-230.25850929940458), AdValue::scale(scratch.ad_value(520), (scratch.values[420] * 0.5))), AdValue::offset(AdValue::scale(AdValue::mul(AdValue::sub_from_scalar((-230.25850929940458), AdValue::scale(scratch.ad_value(520), (scratch.values[420] * 0.5))), AdValue::offset(AdValue::scale(AdValue::sub_from_scalar((-230.25850929940458), AdValue::scale(scratch.ad_value(520), (scratch.values[420] * 0.5))), 0.3333333333333333), 1.0)), 0.5), 1.0)), 1.0));
            scratch.store_ad(1353, &assign37330_ad_e48136);
        }

        if ((((((scratch.values[1323] != 0.0) && (scratch.values[1340] != 0.0)) && (scratch.values[1933] != 0.0)) && (scratch.values[1934] != 0.0)) && (!(scratch.values[1935] != 0.0))) && (!(scratch.values[1936] != 0.0))) {
            scratch.store_ad(1353, &AdValue::scale(AdValue::offset(AdValue::mul(AdValue::offset(AdValue::scale(scratch.ad_value(520), (scratch.values[420] * 0.5)), (-230.25850929940458)), AdValue::offset(AdValue::scale(AdValue::mul(AdValue::offset(AdValue::scale(scratch.ad_value(520), (scratch.values[420] * 0.5)), (-230.25850929940458)), AdValue::offset(AdValue::scale(AdValue::offset(AdValue::scale(scratch.ad_value(520), (scratch.values[420] * 0.5)), (-230.25850929940458)), 0.3333333333333333), 1.0)), 0.5), 1.0)), 1.0), 1e100));
        }

        if ((((scratch.values[1323] != 0.0) && (scratch.values[1340] != 0.0)) && (scratch.values[1933] != 0.0)) && (scratch.values[1934] != 0.0)) {
            scratch.store_ad(1350, &AdValue::square(scratch.ad_value(1353)));
        }

        if ((((scratch.values[1323] != 0.0) && (scratch.values[1340] != 0.0)) && (scratch.values[1933] != 0.0)) && (!(scratch.values[1934] != 0.0))) {
            scratch.store_ad(1350, &AdValue::mul(AdValue::offset(AdValue::scale(AdValue::sub(scratch.ad_value(520), scratch.ad_value(725)), scratch.values[420]), 1.0), scratch.ad_value(726)));
        }

        if ((((scratch.values[1323] != 0.0) && (scratch.values[1340] != 0.0)) && (scratch.values[1933] != 0.0)) && (!(scratch.values[1934] != 0.0))) {
            scratch.store_ad(1353, &AdValue::sqrt(scratch.ad_value(1350)));
        }

        if (((scratch.values[1323] != 0.0) && (scratch.values[1340] != 0.0)) && (scratch.values[1933] != 0.0)) {
            scratch.store_ad(1350, &AdValue::offset(scratch.ad_value(1350), (-1.0)));
        }

        if (((scratch.values[1323] != 0.0) && (scratch.values[1340] != 0.0)) && (scratch.values[1933] != 0.0)) {
            scratch.store_ad(1352, &AdValue::div_from_scalar(1.0, scratch.ad_value(1353)));
        }

        scratch.values[1937] = if (scratch.values[520] > 0.0) { 1.0 } else { 0.0 };

        if ((((scratch.values[1323] != 0.0) && (scratch.values[1340] != 0.0)) && (scratch.values[1933] != 0.0)) && (scratch.values[1937] != 0.0)) {
            scratch.store_ad(1354, &AdValue::scale(AdValue::ln(AdValue::add(AdValue::offset(scratch.ad_value(1352), 2.0), AdValue::sqrt(AdValue::mul(AdValue::offset(scratch.ad_value(1352), 1.0), AdValue::offset(scratch.ad_value(1352), 3.0))))), (scratch.values[419] * 2.0)));
        }

        if ((((scratch.values[1323] != 0.0) && (scratch.values[1340] != 0.0)) && (scratch.values[1933] != 0.0)) && (!(scratch.values[1937] != 0.0))) {
            scratch.store_ad(1354, &AdValue::sub(AdValue::scale(AdValue::ln(AdValue::add(AdValue::offset(AdValue::scale(scratch.ad_value(1353), 2.0), 1.0), AdValue::sqrt(AdValue::mul(AdValue::offset(scratch.ad_value(1353), 1.0), AdValue::offset(AdValue::scale(scratch.ad_value(1353), 3.0), 1.0))))), (scratch.values[419] * 2.0)), scratch.ad_value(520)));
        }

        if (((scratch.values[1323] != 0.0) && (scratch.values[1340] != 0.0)) && (scratch.values[1933] != 0.0)) {
            scratch.store_ad(1355, &AdValue::sub(scratch.ad_value(727), scratch.ad_value(1354)));
        }

        if (((scratch.values[1323] != 0.0) && (scratch.values[1340] != 0.0)) && (scratch.values[1933] != 0.0)) {
            scratch.store_ad(1356, &AdValue::scale(AdValue::sub(AdValue::add(scratch.ad_value(520), scratch.ad_value(1355)), AdValue::sqrt(AdValue::offset(AdValue::mul(AdValue::sub(scratch.ad_value(520), scratch.ad_value(1355)), AdValue::sub(scratch.ad_value(520), scratch.ad_value(1355))), ((4.0 * scratch.values[419]) * scratch.values[419])))), 0.5));
        }

        if (((scratch.values[1323] != 0.0) && (scratch.values[1340] != 0.0)) && (scratch.values[1933] != 0.0)) {
            scratch.store_ad(1357, &AdValue::scale(AdValue::sub(AdValue::add(scratch.ad_value(520), scratch.ad_value(730)), AdValue::sqrt(AdValue::offset(AdValue::mul(AdValue::sub(scratch.ad_value(520), scratch.ad_value(730)), AdValue::sub(scratch.ad_value(520), scratch.ad_value(730))), ((4.0 * scratch.values[417]) * scratch.values[417])))), 0.5));
        }

        if (((scratch.values[1323] != 0.0) && (scratch.values[1340] != 0.0)) && (scratch.values[1933] != 0.0)) {
            scratch.store_ad(1358, &AdValue::scale(AdValue::sub(scratch.ad_value(520), AdValue::sqrt(AdValue::offset(AdValue::mul(scratch.ad_value(520), scratch.ad_value(520)), ((4.0 * 1e-6) * 1e-6)))), 0.5));
        }

        scratch.values[1938] = if (scratch.values[717] == 0.0) { 1.0 } else { 0.0 };

        if (((scratch.values[1323] != 0.0) && (scratch.values[1340] != 0.0)) && (scratch.values[1938] != 0.0)) {
            scratch.values[1386] = 0.0;
            scratch.node_derivatives[1386] = [0.0; Instance::NODE_COUNT];
            scratch.branch_derivatives[1386] = [0.0; Instance::BRANCH_COUNT];
        }

        scratch.values[1939] = if (scratch.values[600] == 0.5) { 1.0 } else { 0.0 };

        if ((((scratch.values[1323] != 0.0) && (scratch.values[1340] != 0.0)) && (!(scratch.values[1938] != 0.0))) && (scratch.values[1939] != 0.0)) {
            scratch.store_ad(1359, &AdValue::sqrt(AdValue::sub_from_scalar(1.0, AdValue::mul(scratch.ad_value(1351), scratch.ad_value(597)))));
        }

        if ((((scratch.values[1323] != 0.0) && (scratch.values[1340] != 0.0)) && (!(scratch.values[1938] != 0.0))) && (!(scratch.values[1939] != 0.0))) {
            scratch.store_ad(1359, &AdValue::pow(AdValue::sub_from_scalar(1.0, AdValue::mul(scratch.ad_value(1351), scratch.ad_value(597))), scratch.ad_value(600)));
        }

        if (((scratch.values[1323] != 0.0) && (scratch.values[1340] != 0.0)) && (!(scratch.values[1938] != 0.0))) {
            scratch.store_ad(1360, &AdValue::mul(scratch.ad_value(588), scratch.ad_value(1350)));
        }

        scratch.values[1940] = if ((scratch.values[553] == 0.0) && (scratch.values[556] == 0.0)) { 1.0 } else { 0.0 };

        if ((((scratch.values[1323] != 0.0) && (scratch.values[1340] != 0.0)) && (!(scratch.values[1938] != 0.0))) && (scratch.values[1940] != 0.0)) {
            scratch.values[1361] = 0.0;
            scratch.node_derivatives[1361] = [0.0; Instance::NODE_COUNT];
            scratch.branch_derivatives[1361] = [0.0; Instance::BRANCH_COUNT];
        }

        if ((((scratch.values[1323] != 0.0) && (scratch.values[1340] != 0.0)) && (!(scratch.values[1938] != 0.0))) && (!(scratch.values[1940] != 0.0))) {
            scratch.store_ad(1362, &AdValue::sub(scratch.ad_value(594), scratch.ad_value(1356)));
        }

        if ((((scratch.values[1323] != 0.0) && (scratch.values[1340] != 0.0)) && (!(scratch.values[1938] != 0.0))) && (!(scratch.values[1940] != 0.0))) {
            scratch.store_ad(1363, &AdValue::sub_from_scalar(1.0, AdValue::sqrt(AdValue::sub_from_scalar(1.0, AdValue::div(scratch.ad_value(1354), scratch.ad_value(1362))))));
        }

        scratch.values[1941] = if (scratch.values[542] == 0.5) { 1.0 } else { 0.0 };

        if (((((scratch.values[1323] != 0.0) && (scratch.values[1340] != 0.0)) && (!(scratch.values[1938] != 0.0))) && (!(scratch.values[1940] != 0.0))) && (scratch.values[1941] != 0.0)) {
            scratch.values[1364] = 0.0;
            scratch.node_derivatives[1364] = [0.0; Instance::NODE_COUNT];
            scratch.branch_derivatives[1364] = [0.0; Instance::BRANCH_COUNT];
        }

        if (((((scratch.values[1323] != 0.0) && (scratch.values[1340] != 0.0)) && (!(scratch.values[1938] != 0.0))) && (!(scratch.values[1940] != 0.0))) && (!(scratch.values[1941] != 0.0))) {
            scratch.store_ad(1364, &AdValue::mul(AdValue::add(AdValue::div(AdValue::mul(AdValue::square(scratch.ad_value(1363)), AdValue::ln(scratch.ad_value(1363))), AdValue::sub_from_scalar(1.0, scratch.ad_value(1363))), scratch.ad_value(1363)), AdValue::sub_from_scalar(1.0, AdValue::scale(scratch.ad_value(542), 2.0))));
        }

        if ((((scratch.values[1323] != 0.0) && (scratch.values[1340] != 0.0)) && (!(scratch.values[1938] != 0.0))) && (!(scratch.values[1940] != 0.0))) {
            scratch.store_ad(1365, &AdValue::add(scratch.ad_value(1363), scratch.ad_value(1364)));
        }

        scratch.values[1942] = if (scratch.values[542] == 0.5) { 1.0 } else { 0.0 };

        if (((((scratch.values[1323] != 0.0) && (scratch.values[1340] != 0.0)) && (!(scratch.values[1938] != 0.0))) && (!(scratch.values[1940] != 0.0))) && (scratch.values[1942] != 0.0)) {
            scratch.store_ad(1359, &AdValue::sqrt(AdValue::mul(scratch.ad_value(1362), scratch.ad_value(621))));
        }

        if (((((scratch.values[1323] != 0.0) && (scratch.values[1340] != 0.0)) && (!(scratch.values[1938] != 0.0))) && (!(scratch.values[1940] != 0.0))) && (!(scratch.values[1942] != 0.0))) {
            scratch.store_ad(1359, &AdValue::pow(AdValue::mul(scratch.ad_value(1362), scratch.ad_value(621)), scratch.ad_value(542)));
        }

        if ((((scratch.values[1323] != 0.0) && (scratch.values[1340] != 0.0)) && (!(scratch.values[1938] != 0.0))) && (!(scratch.values[1940] != 0.0))) {
            scratch.store_ad(1366, &AdValue::mul(scratch.ad_value(615), scratch.ad_value(1359)));
        }

        if ((((scratch.values[1323] != 0.0) && (scratch.values[1340] != 0.0)) && (!(scratch.values[1938] != 0.0))) && (!(scratch.values[1940] != 0.0))) {
            scratch.store_ad(1367, &AdValue::mul(scratch.ad_value(585), AdValue::mul(AdValue::offset(scratch.ad_value(1353), (-1.0)), scratch.ad_value(1366))));
        }

        if ((((scratch.values[1323] != 0.0) && (scratch.values[1340] != 0.0)) && (!(scratch.values[1938] != 0.0))) && (!(scratch.values[1940] != 0.0))) {
            scratch.store_ad(1361, &AdValue::mul(scratch.ad_value(553), AdValue::mul(scratch.ad_value(1367), scratch.ad_value(1365))));
        }

        scratch.values[1943] = if (scratch.values[556] == 0.0) { 1.0 } else { 0.0 };

        if ((((scratch.values[1323] != 0.0) && (scratch.values[1340] != 0.0)) && (!(scratch.values[1938] != 0.0))) && (scratch.values[1943] != 0.0)) {
            scratch.values[1368] = 0.0;
            scratch.node_derivatives[1368] = [0.0; Instance::NODE_COUNT];
            scratch.branch_derivatives[1368] = [0.0; Instance::BRANCH_COUNT];
        }

        if ((((scratch.values[1323] != 0.0) && (scratch.values[1340] != 0.0)) && (!(scratch.values[1938] != 0.0))) && (!(scratch.values[1943] != 0.0))) {
            scratch.store_ad(1369, &AdValue::mul(scratch.ad_value(630), AdValue::div(AdValue::mul(scratch.ad_value(1366), scratch.ad_value(600)), scratch.ad_value(1362))));
        }

        if ((((scratch.values[1323] != 0.0) && (scratch.values[1340] != 0.0)) && (!(scratch.values[1938] != 0.0))) && (!(scratch.values[1943] != 0.0))) {
            scratch.store_ad(1370, &AdValue::div(AdValue::scale(scratch.ad_value(627), 0.666666666666667), scratch.ad_value(1369)));
        }

        if ((((scratch.values[1323] != 0.0) && (scratch.values[1340] != 0.0)) && (!(scratch.values[1938] != 0.0))) && (!(scratch.values[1943] != 0.0))) {
            scratch.store_ad(1371, &AdValue::square(scratch.ad_value(1370)));
        }

        if ((((scratch.values[1323] != 0.0) && (scratch.values[1340] != 0.0)) && (!(scratch.values[1938] != 0.0))) && (!(scratch.values[1943] != 0.0))) {
            scratch.store_ad(1372, &AdValue::sqrt(AdValue::div(AdValue::square(scratch.ad_value(1371)), AdValue::offset(AdValue::square(scratch.ad_value(1371)), 1.0))));
        }

        if ((((scratch.values[1323] != 0.0) && (scratch.values[1340] != 0.0)) && (!(scratch.values[1938] != 0.0))) && (!(scratch.values[1943] != 0.0))) {
            scratch.store_ad(1373, &AdValue::sqrt(AdValue::abs(scratch.ad_value(1372))));
        }

        if ((((scratch.values[1323] != 0.0) && (scratch.values[1340] != 0.0)) && (!(scratch.values[1938] != 0.0))) && (!(scratch.values[1943] != 0.0))) {
            scratch.store_ad(1374, &AdValue::mul(scratch.ad_value(1372), scratch.ad_value(1373)));
        }

        scratch.values[1944] = if (((-scratch.values[542]) * scratch.values[603]) == (-1.0)) { 1.0 } else { 0.0 };

        if (((((scratch.values[1323] != 0.0) && (scratch.values[1340] != 0.0)) && (!(scratch.values[1938] != 0.0))) && (!(scratch.values[1943] != 0.0))) && (scratch.values[1944] != 0.0)) {
            scratch.store_ad(1375, &AdValue::div_from_scalar(1.0, AdValue::offset(AdValue::mul(scratch.ad_value(1369), scratch.ad_value(1374)), 1.0)));
        }

        if (((((scratch.values[1323] != 0.0) && (scratch.values[1340] != 0.0)) && (!(scratch.values[1938] != 0.0))) && (!(scratch.values[1943] != 0.0))) && (!(scratch.values[1944] != 0.0))) {
            scratch.store_ad(1375, &AdValue::pow(AdValue::offset(AdValue::mul(scratch.ad_value(1369), scratch.ad_value(1374)), 1.0), AdValue::mul(AdValue::neg(scratch.ad_value(542)), scratch.ad_value(603))));
        }

        if ((((scratch.values[1323] != 0.0) && (scratch.values[1340] != 0.0)) && (!(scratch.values[1938] != 0.0))) && (!(scratch.values[1943] != 0.0))) {
            scratch.store_ad(1376, &AdValue::div(AdValue::mul(scratch.ad_value(1365), scratch.ad_value(1375)), AdValue::add(scratch.ad_value(1365), scratch.ad_value(1375))));
        }

        if ((((scratch.values[1323] != 0.0) && (scratch.values[1340] != 0.0)) && (!(scratch.values[1938] != 0.0))) && (!(scratch.values[1943] != 0.0))) {
            scratch.store_ad(1377, &AdValue::sqrt(AdValue::scale(AdValue::div(scratch.ad_value(1369), scratch.ad_value(1373)), 0.375)));
        }

        if ((((scratch.values[1323] != 0.0) && (scratch.values[1340] != 0.0)) && (!(scratch.values[1938] != 0.0))) && (!(scratch.values[1943] != 0.0))) {
            scratch.store_ad(1378, &AdValue::sub(AdValue::scale(AdValue::mul(scratch.ad_value(1370), scratch.ad_value(1373)), 2.0), scratch.ad_value(1372)));
        }

        if ((((scratch.values[1323] != 0.0) && (scratch.values[1340] != 0.0)) && (!(scratch.values[1938] != 0.0))) && (!(scratch.values[1943] != 0.0))) {
            scratch.store_ad(1379, &AdValue::add(AdValue::sub(AdValue::mul(AdValue::mul(scratch.ad_value(627), scratch.ad_value(1370)), scratch.ad_value(1373)), AdValue::mul(scratch.ad_value(627), scratch.ad_value(1372))), AdValue::scale(AdValue::mul(scratch.ad_value(1369), scratch.ad_value(1374)), 0.5)));
        }

        if ((((scratch.values[1323] != 0.0) && (scratch.values[1340] != 0.0)) && (!(scratch.values[1938] != 0.0))) && (!(scratch.values[1943] != 0.0))) {
            scratch.store_ad(1380, &AdValue::mul(AdValue::offset(scratch.ad_value(1378), (-1.0)), scratch.ad_value(1377)));
        }

        if ((((scratch.values[1323] != 0.0) && (scratch.values[1340] != 0.0)) && (!(scratch.values[1938] != 0.0))) && (!(scratch.values[1943] != 0.0))) {
            scratch.store_ad(1341, &AdValue::square(scratch.ad_value(1380)));
        }

        scratch.values[1945] = if (scratch.values[1380] > 0.0) { 1.0 } else { 0.0 };

        if (((((scratch.values[1323] != 0.0) && (scratch.values[1340] != 0.0)) && (!(scratch.values[1938] != 0.0))) && (!(scratch.values[1943] != 0.0))) && (scratch.values[1945] != 0.0)) {
            scratch.store_ad(1342, &AdValue::div_from_scalar(1.0, AdValue::offset(AdValue::scale(scratch.ad_value(1380), scratch.values[421]), 1.0)));
        }

        if (((((scratch.values[1323] != 0.0) && (scratch.values[1340] != 0.0)) && (!(scratch.values[1938] != 0.0))) && (!(scratch.values[1943] != 0.0))) && (!(scratch.values[1945] != 0.0))) {
            scratch.store_ad(1342, &AdValue::div_from_scalar(1.0, AdValue::sub_from_scalar(1.0, AdValue::scale(scratch.ad_value(1380), scratch.values[421]))));
        }

        scratch.values[1946] = if (((-scratch.values[1341]) + scratch.values[1379]) > (-230.25850929940458)) { 1.0 } else { 0.0 };

        if (((((scratch.values[1323] != 0.0) && (scratch.values[1340] != 0.0)) && (!(scratch.values[1938] != 0.0))) && (!(scratch.values[1943] != 0.0))) && (scratch.values[1946] != 0.0)) {
            scratch.store_ad(1359, &AdValue::exp(AdValue::sub(scratch.ad_value(1379), scratch.ad_value(1341))));
        }

        if (((((scratch.values[1323] != 0.0) && (scratch.values[1340] != 0.0)) && (!(scratch.values[1938] != 0.0))) && (!(scratch.values[1943] != 0.0))) && (!(scratch.values[1946] != 0.0))) {
            let assign37910_ad_e49079: AdValue = AdValue::div_from_scalar(1e-100, AdValue::offset(AdValue::mul(AdValue::sub_from_scalar((-230.25850929940458), AdValue::sub(scratch.ad_value(1379), scratch.ad_value(1341))), AdValue::offset(AdValue::scale(AdValue::mul(AdValue::sub_from_scalar((-230.25850929940458), AdValue::sub(scratch.ad_value(1379), scratch.ad_value(1341))), AdValue::offset(AdValue::scale(AdValue::sub_from_scalar((-230.25850929940458), AdValue::sub(scratch.ad_value(1379), scratch.ad_value(1341))), 0.3333333333333333), 1.0)), 0.5), 1.0)), 1.0));
            scratch.store_ad(1359, &assign37910_ad_e49079);
        }

        if ((((scratch.values[1323] != 0.0) && (scratch.values[1340] != 0.0)) && (!(scratch.values[1938] != 0.0))) && (!(scratch.values[1943] != 0.0))) {
            scratch.store_ad(1343, &AdValue::mul(AdValue::add(AdValue::add(AdValue::scale(scratch.ad_value(1342), 0.29214664), AdValue::scale(AdValue::square(scratch.ad_value(1342)), scratch.values[422])), AdValue::scale(AdValue::mul(AdValue::square(scratch.ad_value(1342)), scratch.ad_value(1342)), scratch.values[423])), scratch.ad_value(1359)));
        }

        scratch.values[1947] = if (scratch.values[1380] > 0.0) { 1.0 } else { 0.0 };

        if (((((scratch.values[1323] != 0.0) && (scratch.values[1340] != 0.0)) && (!(scratch.values[1938] != 0.0))) && (!(scratch.values[1943] != 0.0))) && (scratch.values[1947] != 0.0)) {
            scratch.values[1381] = scratch.values[1343];
            scratch.node_derivatives[1381] = scratch.node_derivatives[1343];
            scratch.branch_derivatives[1381] = scratch.branch_derivatives[1343];
        }

        scratch.values[1948] = if (scratch.values[1379] > (-230.25850929940458)) { 1.0 } else { 0.0 };

        if ((((((scratch.values[1323] != 0.0) && (scratch.values[1340] != 0.0)) && (!(scratch.values[1938] != 0.0))) && (!(scratch.values[1943] != 0.0))) && (!(scratch.values[1947] != 0.0))) && (scratch.values[1948] != 0.0)) {
            scratch.store_ad(1359, &AdValue::exp(scratch.ad_value(1379)));
        }

        if ((((((scratch.values[1323] != 0.0) && (scratch.values[1340] != 0.0)) && (!(scratch.values[1938] != 0.0))) && (!(scratch.values[1943] != 0.0))) && (!(scratch.values[1947] != 0.0))) && (!(scratch.values[1948] != 0.0))) {
            scratch.store_ad(1359, &AdValue::div_from_scalar(1e-100, AdValue::offset(AdValue::mul(AdValue::sub_from_scalar((-230.25850929940458), scratch.ad_value(1379)), AdValue::offset(AdValue::scale(AdValue::mul(AdValue::sub_from_scalar((-230.25850929940458), scratch.ad_value(1379)), AdValue::offset(AdValue::scale(AdValue::sub_from_scalar((-230.25850929940458), scratch.ad_value(1379)), 0.3333333333333333), 1.0)), 0.5), 1.0)), 1.0)));
        }

        if (((((scratch.values[1323] != 0.0) && (scratch.values[1340] != 0.0)) && (!(scratch.values[1938] != 0.0))) && (!(scratch.values[1943] != 0.0))) && (!(scratch.values[1947] != 0.0))) {
            scratch.store_ad(1381, &AdValue::sub(AdValue::scale(scratch.ad_value(1359), 2.0), scratch.ad_value(1343)));
        }

        if ((((scratch.values[1323] != 0.0) && (scratch.values[1340] != 0.0)) && (!(scratch.values[1938] != 0.0))) && (!(scratch.values[1943] != 0.0))) {
            scratch.store_ad(1382, &AdValue::scale(AdValue::div(AdValue::mul(scratch.ad_value(627), scratch.ad_value(1381)), scratch.ad_value(1377)), (1.772453850905516 * 0.5)));
        }

        if ((((scratch.values[1323] != 0.0) && (scratch.values[1340] != 0.0)) && (!(scratch.values[1938] != 0.0))) && (!(scratch.values[1943] != 0.0))) {
            scratch.store_ad(1368, &AdValue::mul(scratch.ad_value(556), AdValue::mul(AdValue::mul(scratch.ad_value(1367), scratch.ad_value(1382)), scratch.ad_value(1376))));
        }

        scratch.values[1949] = if (scratch.values[562] == 0.0) { 1.0 } else { 0.0 };

        if ((((scratch.values[1323] != 0.0) && (scratch.values[1340] != 0.0)) && (!(scratch.values[1938] != 0.0))) && (scratch.values[1949] != 0.0)) {
            scratch.values[1383] = 0.0;
            scratch.node_derivatives[1383] = [0.0; Instance::NODE_COUNT];
            scratch.branch_derivatives[1383] = [0.0; Instance::BRANCH_COUNT];
        }

        scratch.values[1950] = if (scratch.values[542] == 0.5) { 1.0 } else { 0.0 };

        if (((((scratch.values[1323] != 0.0) && (scratch.values[1340] != 0.0)) && (!(scratch.values[1938] != 0.0))) && (!(scratch.values[1949] != 0.0))) && (scratch.values[1950] != 0.0)) {
            scratch.store_ad(1359, &AdValue::sqrt(AdValue::mul(AdValue::sub(scratch.ad_value(539), scratch.ad_value(1357)), scratch.ad_value(621))));
        }

        if (((((scratch.values[1323] != 0.0) && (scratch.values[1340] != 0.0)) && (!(scratch.values[1938] != 0.0))) && (!(scratch.values[1949] != 0.0))) && (!(scratch.values[1950] != 0.0))) {
            scratch.store_ad(1359, &AdValue::pow(AdValue::mul(AdValue::sub(scratch.ad_value(539), scratch.ad_value(1357)), scratch.ad_value(621)), scratch.ad_value(542)));
        }

        if ((((scratch.values[1323] != 0.0) && (scratch.values[1340] != 0.0)) && (!(scratch.values[1938] != 0.0))) && (!(scratch.values[1949] != 0.0))) {
            scratch.store_ad(1384, &AdValue::mul(scratch.ad_value(603), AdValue::div(AdValue::mul(AdValue::sub(scratch.ad_value(539), scratch.ad_value(1357)), scratch.ad_value(618)), scratch.ad_value(1359))));
        }

        scratch.values[1951] = if (((((-scratch.values[633]) / scratch.values[1384])) as f64).abs() < 230.25850929940458) { 1.0 } else { 0.0 };

        if (((((scratch.values[1323] != 0.0) && (scratch.values[1340] != 0.0)) && (!(scratch.values[1938] != 0.0))) && (!(scratch.values[1949] != 0.0))) && (scratch.values[1951] != 0.0)) {
            scratch.store_ad(1359, &AdValue::exp(AdValue::div(AdValue::neg(scratch.ad_value(633)), scratch.ad_value(1384))));
        }

        scratch.values[1952] = if (((-scratch.values[633]) / scratch.values[1384]) < 0.0) { 1.0 } else { 0.0 };

        if ((((((scratch.values[1323] != 0.0) && (scratch.values[1340] != 0.0)) && (!(scratch.values[1938] != 0.0))) && (!(scratch.values[1949] != 0.0))) && (!(scratch.values[1951] != 0.0))) && (scratch.values[1952] != 0.0)) {
            let assign38100_ad_e49406: AdValue = AdValue::offset(AdValue::mul(AdValue::sub_from_scalar((-230.25850929940458), AdValue::div(AdValue::neg(scratch.ad_value(633)), scratch.ad_value(1384))), AdValue::offset(AdValue::scale(AdValue::mul(AdValue::sub_from_scalar((-230.25850929940458), AdValue::div(AdValue::neg(scratch.ad_value(633)), scratch.ad_value(1384))), AdValue::offset(AdValue::scale(AdValue::sub_from_scalar((-230.25850929940458), AdValue::div(AdValue::neg(scratch.ad_value(633)), scratch.ad_value(1384))), 0.3333333333333333), 1.0)), 0.5), 1.0)), 1.0);
            scratch.store_ad(1359, &AdValue::div_from_scalar(1e-100, assign38100_ad_e49406));
        }

        if ((((((scratch.values[1323] != 0.0) && (scratch.values[1340] != 0.0)) && (!(scratch.values[1938] != 0.0))) && (!(scratch.values[1949] != 0.0))) && (!(scratch.values[1951] != 0.0))) && (!(scratch.values[1952] != 0.0))) {
            let assign38110_ad_e49456: AdValue = AdValue::scale(AdValue::offset(AdValue::mul(AdValue::offset(AdValue::div(AdValue::neg(scratch.ad_value(633)), scratch.ad_value(1384)), (-230.25850929940458)), AdValue::offset(AdValue::scale(AdValue::mul(AdValue::offset(AdValue::div(AdValue::neg(scratch.ad_value(633)), scratch.ad_value(1384)), (-230.25850929940458)), AdValue::offset(AdValue::scale(AdValue::offset(AdValue::div(AdValue::neg(scratch.ad_value(633)), scratch.ad_value(1384)), (-230.25850929940458)), 0.3333333333333333), 1.0)), 0.5), 1.0)), 1.0), 1e100);
            scratch.store_ad(1359, &assign38110_ad_e49456);
        }

        if ((((scratch.values[1323] != 0.0) && (scratch.values[1340] != 0.0)) && (!(scratch.values[1938] != 0.0))) && (!(scratch.values[1949] != 0.0))) {
            scratch.store_ad(1383, &AdValue::mul(scratch.ad_value(562), AdValue::mul(AdValue::mul(AdValue::mul(scratch.ad_value(520), scratch.ad_value(1384)), scratch.ad_value(1384)), scratch.ad_value(1359))));
        }

        scratch.values[1953] = if (scratch.values[571] > 1000.0) { 1.0 } else { 0.0 };

        if ((((scratch.values[1323] != 0.0) && (scratch.values[1340] != 0.0)) && (!(scratch.values[1938] != 0.0))) && (scratch.values[1953] != 0.0)) {
            scratch.values[1385] = 1.0;
            scratch.node_derivatives[1385] = [0.0; Instance::NODE_COUNT];
            scratch.branch_derivatives[1385] = [0.0; Instance::BRANCH_COUNT];
        }

        scratch.values[1954] = if (scratch.values[1358] > ((-scratch.values[493]) * scratch.values[571])) { 1.0 } else { 0.0 };

        scratch.values[1955] = if (scratch.values[574] == 4.0) { 1.0 } else { 0.0 };

        if ((((((scratch.values[1323] != 0.0) && (scratch.values[1340] != 0.0)) && (!(scratch.values[1938] != 0.0))) && (!(scratch.values[1953] != 0.0))) && (scratch.values[1954] != 0.0)) && (scratch.values[1955] != 0.0)) {
            scratch.store_ad(1359, &AdValue::mul(AdValue::mul(AdValue::mul(AdValue::mul(scratch.ad_value(1358), scratch.ad_value(639)), AdValue::mul(scratch.ad_value(1358), scratch.ad_value(639))), AdValue::mul(scratch.ad_value(1358), scratch.ad_value(639))), AdValue::mul(scratch.ad_value(1358), scratch.ad_value(639))));
        }

        if ((((((scratch.values[1323] != 0.0) && (scratch.values[1340] != 0.0)) && (!(scratch.values[1938] != 0.0))) && (!(scratch.values[1953] != 0.0))) && (scratch.values[1954] != 0.0)) && (!(scratch.values[1955] != 0.0))) {
            scratch.store_ad(1359, &AdValue::pow(AdValue::abs(AdValue::mul(scratch.ad_value(1358), scratch.ad_value(639))), scratch.ad_value(574)));
        }

        if (((((scratch.values[1323] != 0.0) && (scratch.values[1340] != 0.0)) && (!(scratch.values[1938] != 0.0))) && (!(scratch.values[1953] != 0.0))) && (scratch.values[1954] != 0.0)) {
            scratch.store_ad(1385, &AdValue::div_from_scalar(1.0, AdValue::sub_from_scalar(1.0, scratch.ad_value(1359))));
        }

        if (((((scratch.values[1323] != 0.0) && (scratch.values[1340] != 0.0)) && (!(scratch.values[1938] != 0.0))) && (!(scratch.values[1953] != 0.0))) && (!(scratch.values[1954] != 0.0))) {
            scratch.store_ad(1385, &AdValue::add(scratch.ad_value(636), AdValue::mul(AdValue::add(scratch.ad_value(1358), AdValue::scale(scratch.ad_value(571), scratch.values[493])), scratch.ad_value(642))));
        }

        if (((scratch.values[1323] != 0.0) && (scratch.values[1340] != 0.0)) && (!(scratch.values[1938] != 0.0))) {
            scratch.store_ad(1386, &AdValue::mul(AdValue::add(AdValue::add(AdValue::add(scratch.ad_value(1360), scratch.ad_value(1361)), scratch.ad_value(1368)), scratch.ad_value(1383)), scratch.ad_value(1385)));
        }

        scratch.values[1956] = if (scratch.values[718] == 0.0) { 1.0 } else { 0.0 };

        if (((scratch.values[1323] != 0.0) && (scratch.values[1340] != 0.0)) && (scratch.values[1956] != 0.0)) {
            scratch.values[1387] = 0.0;
            scratch.node_derivatives[1387] = [0.0; Instance::NODE_COUNT];
            scratch.branch_derivatives[1387] = [0.0; Instance::BRANCH_COUNT];
        }

        scratch.values[1957] = if (scratch.values[601] == 0.5) { 1.0 } else { 0.0 };

        if ((((scratch.values[1323] != 0.0) && (scratch.values[1340] != 0.0)) && (!(scratch.values[1956] != 0.0))) && (scratch.values[1957] != 0.0)) {
            scratch.store_ad(1359, &AdValue::sqrt(AdValue::sub_from_scalar(1.0, AdValue::mul(scratch.ad_value(1351), scratch.ad_value(598)))));
        }

        if ((((scratch.values[1323] != 0.0) && (scratch.values[1340] != 0.0)) && (!(scratch.values[1956] != 0.0))) && (!(scratch.values[1957] != 0.0))) {
            scratch.store_ad(1359, &AdValue::pow(AdValue::sub_from_scalar(1.0, AdValue::mul(scratch.ad_value(1351), scratch.ad_value(598))), scratch.ad_value(601)));
        }

        if (((scratch.values[1323] != 0.0) && (scratch.values[1340] != 0.0)) && (!(scratch.values[1956] != 0.0))) {
            scratch.store_ad(1360, &AdValue::mul(scratch.ad_value(589), scratch.ad_value(1350)));
        }

        scratch.values[1958] = if ((scratch.values[554] == 0.0) && (scratch.values[557] == 0.0)) { 1.0 } else { 0.0 };

        if ((((scratch.values[1323] != 0.0) && (scratch.values[1340] != 0.0)) && (!(scratch.values[1956] != 0.0))) && (scratch.values[1958] != 0.0)) {
            scratch.values[1361] = 0.0;
            scratch.node_derivatives[1361] = [0.0; Instance::NODE_COUNT];
            scratch.branch_derivatives[1361] = [0.0; Instance::BRANCH_COUNT];
        }

        if ((((scratch.values[1323] != 0.0) && (scratch.values[1340] != 0.0)) && (!(scratch.values[1956] != 0.0))) && (!(scratch.values[1958] != 0.0))) {
            scratch.store_ad(1362, &AdValue::sub(scratch.ad_value(595), scratch.ad_value(1356)));
        }

        if ((((scratch.values[1323] != 0.0) && (scratch.values[1340] != 0.0)) && (!(scratch.values[1956] != 0.0))) && (!(scratch.values[1958] != 0.0))) {
            scratch.store_ad(1363, &AdValue::sub_from_scalar(1.0, AdValue::sqrt(AdValue::sub_from_scalar(1.0, AdValue::div(scratch.ad_value(1354), scratch.ad_value(1362))))));
        }

        scratch.values[1959] = if (scratch.values[543] == 0.5) { 1.0 } else { 0.0 };

        if (((((scratch.values[1323] != 0.0) && (scratch.values[1340] != 0.0)) && (!(scratch.values[1956] != 0.0))) && (!(scratch.values[1958] != 0.0))) && (scratch.values[1959] != 0.0)) {
            scratch.values[1364] = 0.0;
            scratch.node_derivatives[1364] = [0.0; Instance::NODE_COUNT];
            scratch.branch_derivatives[1364] = [0.0; Instance::BRANCH_COUNT];
        }

        if (((((scratch.values[1323] != 0.0) && (scratch.values[1340] != 0.0)) && (!(scratch.values[1956] != 0.0))) && (!(scratch.values[1958] != 0.0))) && (!(scratch.values[1959] != 0.0))) {
            scratch.store_ad(1364, &AdValue::mul(AdValue::add(AdValue::div(AdValue::mul(AdValue::square(scratch.ad_value(1363)), AdValue::ln(scratch.ad_value(1363))), AdValue::sub_from_scalar(1.0, scratch.ad_value(1363))), scratch.ad_value(1363)), AdValue::sub_from_scalar(1.0, AdValue::scale(scratch.ad_value(543), 2.0))));
        }

        if ((((scratch.values[1323] != 0.0) && (scratch.values[1340] != 0.0)) && (!(scratch.values[1956] != 0.0))) && (!(scratch.values[1958] != 0.0))) {
            scratch.store_ad(1365, &AdValue::add(scratch.ad_value(1363), scratch.ad_value(1364)));
        }

        scratch.values[1960] = if (scratch.values[543] == 0.5) { 1.0 } else { 0.0 };

        if (((((scratch.values[1323] != 0.0) && (scratch.values[1340] != 0.0)) && (!(scratch.values[1956] != 0.0))) && (!(scratch.values[1958] != 0.0))) && (scratch.values[1960] != 0.0)) {
            scratch.store_ad(1359, &AdValue::sqrt(AdValue::mul(scratch.ad_value(1362), scratch.ad_value(622))));
        }

        if (((((scratch.values[1323] != 0.0) && (scratch.values[1340] != 0.0)) && (!(scratch.values[1956] != 0.0))) && (!(scratch.values[1958] != 0.0))) && (!(scratch.values[1960] != 0.0))) {
            scratch.store_ad(1359, &AdValue::pow(AdValue::mul(scratch.ad_value(1362), scratch.ad_value(622)), scratch.ad_value(543)));
        }

        if ((((scratch.values[1323] != 0.0) && (scratch.values[1340] != 0.0)) && (!(scratch.values[1956] != 0.0))) && (!(scratch.values[1958] != 0.0))) {
            scratch.store_ad(1366, &AdValue::mul(scratch.ad_value(616), scratch.ad_value(1359)));
        }

        if ((((scratch.values[1323] != 0.0) && (scratch.values[1340] != 0.0)) && (!(scratch.values[1956] != 0.0))) && (!(scratch.values[1958] != 0.0))) {
            scratch.store_ad(1367, &AdValue::mul(scratch.ad_value(586), AdValue::mul(AdValue::offset(scratch.ad_value(1353), (-1.0)), scratch.ad_value(1366))));
        }

        if ((((scratch.values[1323] != 0.0) && (scratch.values[1340] != 0.0)) && (!(scratch.values[1956] != 0.0))) && (!(scratch.values[1958] != 0.0))) {
            scratch.store_ad(1361, &AdValue::mul(scratch.ad_value(554), AdValue::mul(scratch.ad_value(1367), scratch.ad_value(1365))));
        }

        scratch.values[1961] = if (scratch.values[557] == 0.0) { 1.0 } else { 0.0 };

        if ((((scratch.values[1323] != 0.0) && (scratch.values[1340] != 0.0)) && (!(scratch.values[1956] != 0.0))) && (scratch.values[1961] != 0.0)) {
            scratch.values[1368] = 0.0;
            scratch.node_derivatives[1368] = [0.0; Instance::NODE_COUNT];
            scratch.branch_derivatives[1368] = [0.0; Instance::BRANCH_COUNT];
        }

        if ((((scratch.values[1323] != 0.0) && (scratch.values[1340] != 0.0)) && (!(scratch.values[1956] != 0.0))) && (!(scratch.values[1961] != 0.0))) {
            scratch.store_ad(1369, &AdValue::mul(scratch.ad_value(631), AdValue::div(AdValue::mul(scratch.ad_value(1366), scratch.ad_value(601)), scratch.ad_value(1362))));
        }

        if ((((scratch.values[1323] != 0.0) && (scratch.values[1340] != 0.0)) && (!(scratch.values[1956] != 0.0))) && (!(scratch.values[1961] != 0.0))) {
            scratch.store_ad(1370, &AdValue::div(AdValue::scale(scratch.ad_value(628), 0.666666666666667), scratch.ad_value(1369)));
        }

        if ((((scratch.values[1323] != 0.0) && (scratch.values[1340] != 0.0)) && (!(scratch.values[1956] != 0.0))) && (!(scratch.values[1961] != 0.0))) {
            scratch.store_ad(1371, &AdValue::square(scratch.ad_value(1370)));
        }

        if ((((scratch.values[1323] != 0.0) && (scratch.values[1340] != 0.0)) && (!(scratch.values[1956] != 0.0))) && (!(scratch.values[1961] != 0.0))) {
            scratch.store_ad(1372, &AdValue::sqrt(AdValue::div(AdValue::square(scratch.ad_value(1371)), AdValue::offset(AdValue::square(scratch.ad_value(1371)), 1.0))));
        }

        if ((((scratch.values[1323] != 0.0) && (scratch.values[1340] != 0.0)) && (!(scratch.values[1956] != 0.0))) && (!(scratch.values[1961] != 0.0))) {
            scratch.store_ad(1373, &AdValue::sqrt(AdValue::abs(scratch.ad_value(1372))));
        }

        if ((((scratch.values[1323] != 0.0) && (scratch.values[1340] != 0.0)) && (!(scratch.values[1956] != 0.0))) && (!(scratch.values[1961] != 0.0))) {
            scratch.store_ad(1374, &AdValue::mul(scratch.ad_value(1372), scratch.ad_value(1373)));
        }

        scratch.values[1962] = if (((-scratch.values[543]) * scratch.values[604]) == (-1.0)) { 1.0 } else { 0.0 };

        if (((((scratch.values[1323] != 0.0) && (scratch.values[1340] != 0.0)) && (!(scratch.values[1956] != 0.0))) && (!(scratch.values[1961] != 0.0))) && (scratch.values[1962] != 0.0)) {
            scratch.store_ad(1375, &AdValue::div_from_scalar(1.0, AdValue::offset(AdValue::mul(scratch.ad_value(1369), scratch.ad_value(1374)), 1.0)));
        }

    }

    pub(super) fn stamp_transient_block_29(
        &mut self,
        ctx: &GeneratedEvalContext<'_>,
        stamper: &mut GeneratedStamper<'_>,
        scratch: &mut Scratch,
    ) {
        let _ = stamper;
        if (((((scratch.values[1323] != 0.0) && (scratch.values[1340] != 0.0)) && (!(scratch.values[1956] != 0.0))) && (!(scratch.values[1961] != 0.0))) && (!(scratch.values[1962] != 0.0))) {
            scratch.store_ad(1375, &AdValue::pow(AdValue::offset(AdValue::mul(scratch.ad_value(1369), scratch.ad_value(1374)), 1.0), AdValue::mul(AdValue::neg(scratch.ad_value(543)), scratch.ad_value(604))));
        }

        if ((((scratch.values[1323] != 0.0) && (scratch.values[1340] != 0.0)) && (!(scratch.values[1956] != 0.0))) && (!(scratch.values[1961] != 0.0))) {
            scratch.store_ad(1376, &AdValue::div(AdValue::mul(scratch.ad_value(1365), scratch.ad_value(1375)), AdValue::add(scratch.ad_value(1365), scratch.ad_value(1375))));
        }

        if ((((scratch.values[1323] != 0.0) && (scratch.values[1340] != 0.0)) && (!(scratch.values[1956] != 0.0))) && (!(scratch.values[1961] != 0.0))) {
            scratch.store_ad(1377, &AdValue::sqrt(AdValue::scale(AdValue::div(scratch.ad_value(1369), scratch.ad_value(1373)), 0.375)));
        }

        if ((((scratch.values[1323] != 0.0) && (scratch.values[1340] != 0.0)) && (!(scratch.values[1956] != 0.0))) && (!(scratch.values[1961] != 0.0))) {
            scratch.store_ad(1378, &AdValue::sub(AdValue::scale(AdValue::mul(scratch.ad_value(1370), scratch.ad_value(1373)), 2.0), scratch.ad_value(1372)));
        }

        if ((((scratch.values[1323] != 0.0) && (scratch.values[1340] != 0.0)) && (!(scratch.values[1956] != 0.0))) && (!(scratch.values[1961] != 0.0))) {
            scratch.store_ad(1379, &AdValue::add(AdValue::sub(AdValue::mul(AdValue::mul(scratch.ad_value(628), scratch.ad_value(1370)), scratch.ad_value(1373)), AdValue::mul(scratch.ad_value(628), scratch.ad_value(1372))), AdValue::scale(AdValue::mul(scratch.ad_value(1369), scratch.ad_value(1374)), 0.5)));
        }

        if ((((scratch.values[1323] != 0.0) && (scratch.values[1340] != 0.0)) && (!(scratch.values[1956] != 0.0))) && (!(scratch.values[1961] != 0.0))) {
            scratch.store_ad(1380, &AdValue::mul(AdValue::offset(scratch.ad_value(1378), (-1.0)), scratch.ad_value(1377)));
        }

        if ((((scratch.values[1323] != 0.0) && (scratch.values[1340] != 0.0)) && (!(scratch.values[1956] != 0.0))) && (!(scratch.values[1961] != 0.0))) {
            scratch.store_ad(1341, &AdValue::square(scratch.ad_value(1380)));
        }

        scratch.values[1963] = if (scratch.values[1380] > 0.0) { 1.0 } else { 0.0 };

        if (((((scratch.values[1323] != 0.0) && (scratch.values[1340] != 0.0)) && (!(scratch.values[1956] != 0.0))) && (!(scratch.values[1961] != 0.0))) && (scratch.values[1963] != 0.0)) {
            scratch.store_ad(1342, &AdValue::div_from_scalar(1.0, AdValue::offset(AdValue::scale(scratch.ad_value(1380), scratch.values[421]), 1.0)));
        }

        if (((((scratch.values[1323] != 0.0) && (scratch.values[1340] != 0.0)) && (!(scratch.values[1956] != 0.0))) && (!(scratch.values[1961] != 0.0))) && (!(scratch.values[1963] != 0.0))) {
            scratch.store_ad(1342, &AdValue::div_from_scalar(1.0, AdValue::sub_from_scalar(1.0, AdValue::scale(scratch.ad_value(1380), scratch.values[421]))));
        }

        scratch.values[1964] = if (((-scratch.values[1341]) + scratch.values[1379]) > (-230.25850929940458)) { 1.0 } else { 0.0 };

        if (((((scratch.values[1323] != 0.0) && (scratch.values[1340] != 0.0)) && (!(scratch.values[1956] != 0.0))) && (!(scratch.values[1961] != 0.0))) && (scratch.values[1964] != 0.0)) {
            scratch.store_ad(1359, &AdValue::exp(AdValue::sub(scratch.ad_value(1379), scratch.ad_value(1341))));
        }

        if (((((scratch.values[1323] != 0.0) && (scratch.values[1340] != 0.0)) && (!(scratch.values[1956] != 0.0))) && (!(scratch.values[1961] != 0.0))) && (!(scratch.values[1964] != 0.0))) {
            let assign38660_ad_e50285: AdValue = AdValue::div_from_scalar(1e-100, AdValue::offset(AdValue::mul(AdValue::sub_from_scalar((-230.25850929940458), AdValue::sub(scratch.ad_value(1379), scratch.ad_value(1341))), AdValue::offset(AdValue::scale(AdValue::mul(AdValue::sub_from_scalar((-230.25850929940458), AdValue::sub(scratch.ad_value(1379), scratch.ad_value(1341))), AdValue::offset(AdValue::scale(AdValue::sub_from_scalar((-230.25850929940458), AdValue::sub(scratch.ad_value(1379), scratch.ad_value(1341))), 0.3333333333333333), 1.0)), 0.5), 1.0)), 1.0));
            scratch.store_ad(1359, &assign38660_ad_e50285);
        }

        if ((((scratch.values[1323] != 0.0) && (scratch.values[1340] != 0.0)) && (!(scratch.values[1956] != 0.0))) && (!(scratch.values[1961] != 0.0))) {
            scratch.store_ad(1343, &AdValue::mul(AdValue::add(AdValue::add(AdValue::scale(scratch.ad_value(1342), 0.29214664), AdValue::scale(AdValue::square(scratch.ad_value(1342)), scratch.values[422])), AdValue::scale(AdValue::mul(AdValue::square(scratch.ad_value(1342)), scratch.ad_value(1342)), scratch.values[423])), scratch.ad_value(1359)));
        }

        scratch.values[1965] = if (scratch.values[1380] > 0.0) { 1.0 } else { 0.0 };

        if (((((scratch.values[1323] != 0.0) && (scratch.values[1340] != 0.0)) && (!(scratch.values[1956] != 0.0))) && (!(scratch.values[1961] != 0.0))) && (scratch.values[1965] != 0.0)) {
            scratch.values[1381] = scratch.values[1343];
            scratch.node_derivatives[1381] = scratch.node_derivatives[1343];
            scratch.branch_derivatives[1381] = scratch.branch_derivatives[1343];
        }

        scratch.values[1966] = if (scratch.values[1379] > (-230.25850929940458)) { 1.0 } else { 0.0 };

        if ((((((scratch.values[1323] != 0.0) && (scratch.values[1340] != 0.0)) && (!(scratch.values[1956] != 0.0))) && (!(scratch.values[1961] != 0.0))) && (!(scratch.values[1965] != 0.0))) && (scratch.values[1966] != 0.0)) {
            scratch.store_ad(1359, &AdValue::exp(scratch.ad_value(1379)));
        }

        if ((((((scratch.values[1323] != 0.0) && (scratch.values[1340] != 0.0)) && (!(scratch.values[1956] != 0.0))) && (!(scratch.values[1961] != 0.0))) && (!(scratch.values[1965] != 0.0))) && (!(scratch.values[1966] != 0.0))) {
            scratch.store_ad(1359, &AdValue::div_from_scalar(1e-100, AdValue::offset(AdValue::mul(AdValue::sub_from_scalar((-230.25850929940458), scratch.ad_value(1379)), AdValue::offset(AdValue::scale(AdValue::mul(AdValue::sub_from_scalar((-230.25850929940458), scratch.ad_value(1379)), AdValue::offset(AdValue::scale(AdValue::sub_from_scalar((-230.25850929940458), scratch.ad_value(1379)), 0.3333333333333333), 1.0)), 0.5), 1.0)), 1.0)));
        }

        if (((((scratch.values[1323] != 0.0) && (scratch.values[1340] != 0.0)) && (!(scratch.values[1956] != 0.0))) && (!(scratch.values[1961] != 0.0))) && (!(scratch.values[1965] != 0.0))) {
            scratch.store_ad(1381, &AdValue::sub(AdValue::scale(scratch.ad_value(1359), 2.0), scratch.ad_value(1343)));
        }

        if ((((scratch.values[1323] != 0.0) && (scratch.values[1340] != 0.0)) && (!(scratch.values[1956] != 0.0))) && (!(scratch.values[1961] != 0.0))) {
            scratch.store_ad(1382, &AdValue::scale(AdValue::div(AdValue::mul(scratch.ad_value(628), scratch.ad_value(1381)), scratch.ad_value(1377)), (1.772453850905516 * 0.5)));
        }

        if ((((scratch.values[1323] != 0.0) && (scratch.values[1340] != 0.0)) && (!(scratch.values[1956] != 0.0))) && (!(scratch.values[1961] != 0.0))) {
            scratch.store_ad(1368, &AdValue::mul(scratch.ad_value(557), AdValue::mul(AdValue::mul(scratch.ad_value(1367), scratch.ad_value(1382)), scratch.ad_value(1376))));
        }

        scratch.values[1967] = if (scratch.values[563] == 0.0) { 1.0 } else { 0.0 };

        if ((((scratch.values[1323] != 0.0) && (scratch.values[1340] != 0.0)) && (!(scratch.values[1956] != 0.0))) && (scratch.values[1967] != 0.0)) {
            scratch.values[1383] = 0.0;
            scratch.node_derivatives[1383] = [0.0; Instance::NODE_COUNT];
            scratch.branch_derivatives[1383] = [0.0; Instance::BRANCH_COUNT];
        }

        scratch.values[1968] = if (scratch.values[543] == 0.5) { 1.0 } else { 0.0 };

        if (((((scratch.values[1323] != 0.0) && (scratch.values[1340] != 0.0)) && (!(scratch.values[1956] != 0.0))) && (!(scratch.values[1967] != 0.0))) && (scratch.values[1968] != 0.0)) {
            scratch.store_ad(1359, &AdValue::sqrt(AdValue::mul(AdValue::sub(scratch.ad_value(540), scratch.ad_value(1357)), scratch.ad_value(622))));
        }

        if (((((scratch.values[1323] != 0.0) && (scratch.values[1340] != 0.0)) && (!(scratch.values[1956] != 0.0))) && (!(scratch.values[1967] != 0.0))) && (!(scratch.values[1968] != 0.0))) {
            scratch.store_ad(1359, &AdValue::pow(AdValue::mul(AdValue::sub(scratch.ad_value(540), scratch.ad_value(1357)), scratch.ad_value(622)), scratch.ad_value(543)));
        }

        if ((((scratch.values[1323] != 0.0) && (scratch.values[1340] != 0.0)) && (!(scratch.values[1956] != 0.0))) && (!(scratch.values[1967] != 0.0))) {
            scratch.store_ad(1384, &AdValue::mul(scratch.ad_value(604), AdValue::div(AdValue::mul(AdValue::sub(scratch.ad_value(540), scratch.ad_value(1357)), scratch.ad_value(619)), scratch.ad_value(1359))));
        }

        scratch.values[1969] = if (((((-scratch.values[634]) / scratch.values[1384])) as f64).abs() < 230.25850929940458) { 1.0 } else { 0.0 };

        if (((((scratch.values[1323] != 0.0) && (scratch.values[1340] != 0.0)) && (!(scratch.values[1956] != 0.0))) && (!(scratch.values[1967] != 0.0))) && (scratch.values[1969] != 0.0)) {
            scratch.store_ad(1359, &AdValue::exp(AdValue::div(AdValue::neg(scratch.ad_value(634)), scratch.ad_value(1384))));
        }

        scratch.values[1970] = if (((-scratch.values[634]) / scratch.values[1384]) < 0.0) { 1.0 } else { 0.0 };

        if ((((((scratch.values[1323] != 0.0) && (scratch.values[1340] != 0.0)) && (!(scratch.values[1956] != 0.0))) && (!(scratch.values[1967] != 0.0))) && (!(scratch.values[1969] != 0.0))) && (scratch.values[1970] != 0.0)) {
            let assign38850_ad_e50612: AdValue = AdValue::offset(AdValue::mul(AdValue::sub_from_scalar((-230.25850929940458), AdValue::div(AdValue::neg(scratch.ad_value(634)), scratch.ad_value(1384))), AdValue::offset(AdValue::scale(AdValue::mul(AdValue::sub_from_scalar((-230.25850929940458), AdValue::div(AdValue::neg(scratch.ad_value(634)), scratch.ad_value(1384))), AdValue::offset(AdValue::scale(AdValue::sub_from_scalar((-230.25850929940458), AdValue::div(AdValue::neg(scratch.ad_value(634)), scratch.ad_value(1384))), 0.3333333333333333), 1.0)), 0.5), 1.0)), 1.0);
            scratch.store_ad(1359, &AdValue::div_from_scalar(1e-100, assign38850_ad_e50612));
        }

        if ((((((scratch.values[1323] != 0.0) && (scratch.values[1340] != 0.0)) && (!(scratch.values[1956] != 0.0))) && (!(scratch.values[1967] != 0.0))) && (!(scratch.values[1969] != 0.0))) && (!(scratch.values[1970] != 0.0))) {
            let assign38860_ad_e50662: AdValue = AdValue::scale(AdValue::offset(AdValue::mul(AdValue::offset(AdValue::div(AdValue::neg(scratch.ad_value(634)), scratch.ad_value(1384)), (-230.25850929940458)), AdValue::offset(AdValue::scale(AdValue::mul(AdValue::offset(AdValue::div(AdValue::neg(scratch.ad_value(634)), scratch.ad_value(1384)), (-230.25850929940458)), AdValue::offset(AdValue::scale(AdValue::offset(AdValue::div(AdValue::neg(scratch.ad_value(634)), scratch.ad_value(1384)), (-230.25850929940458)), 0.3333333333333333), 1.0)), 0.5), 1.0)), 1.0), 1e100);
            scratch.store_ad(1359, &assign38860_ad_e50662);
        }

        if ((((scratch.values[1323] != 0.0) && (scratch.values[1340] != 0.0)) && (!(scratch.values[1956] != 0.0))) && (!(scratch.values[1967] != 0.0))) {
            scratch.store_ad(1383, &AdValue::mul(scratch.ad_value(563), AdValue::mul(AdValue::mul(AdValue::mul(scratch.ad_value(520), scratch.ad_value(1384)), scratch.ad_value(1384)), scratch.ad_value(1359))));
        }

        scratch.values[1971] = if (scratch.values[572] > 1000.0) { 1.0 } else { 0.0 };

        if ((((scratch.values[1323] != 0.0) && (scratch.values[1340] != 0.0)) && (!(scratch.values[1956] != 0.0))) && (scratch.values[1971] != 0.0)) {
            scratch.values[1385] = 1.0;
            scratch.node_derivatives[1385] = [0.0; Instance::NODE_COUNT];
            scratch.branch_derivatives[1385] = [0.0; Instance::BRANCH_COUNT];
        }

        scratch.values[1972] = if (scratch.values[1358] > ((-scratch.values[493]) * scratch.values[572])) { 1.0 } else { 0.0 };

        scratch.values[1973] = if (scratch.values[575] == 4.0) { 1.0 } else { 0.0 };

        if ((((((scratch.values[1323] != 0.0) && (scratch.values[1340] != 0.0)) && (!(scratch.values[1956] != 0.0))) && (!(scratch.values[1971] != 0.0))) && (scratch.values[1972] != 0.0)) && (scratch.values[1973] != 0.0)) {
            scratch.store_ad(1359, &AdValue::mul(AdValue::mul(AdValue::mul(AdValue::mul(scratch.ad_value(1358), scratch.ad_value(640)), AdValue::mul(scratch.ad_value(1358), scratch.ad_value(640))), AdValue::mul(scratch.ad_value(1358), scratch.ad_value(640))), AdValue::mul(scratch.ad_value(1358), scratch.ad_value(640))));
        }

        if ((((((scratch.values[1323] != 0.0) && (scratch.values[1340] != 0.0)) && (!(scratch.values[1956] != 0.0))) && (!(scratch.values[1971] != 0.0))) && (scratch.values[1972] != 0.0)) && (!(scratch.values[1973] != 0.0))) {
            scratch.store_ad(1359, &AdValue::pow(AdValue::abs(AdValue::mul(scratch.ad_value(1358), scratch.ad_value(640))), scratch.ad_value(575)));
        }

        if (((((scratch.values[1323] != 0.0) && (scratch.values[1340] != 0.0)) && (!(scratch.values[1956] != 0.0))) && (!(scratch.values[1971] != 0.0))) && (scratch.values[1972] != 0.0)) {
            scratch.store_ad(1385, &AdValue::div_from_scalar(1.0, AdValue::sub_from_scalar(1.0, scratch.ad_value(1359))));
        }

        if (((((scratch.values[1323] != 0.0) && (scratch.values[1340] != 0.0)) && (!(scratch.values[1956] != 0.0))) && (!(scratch.values[1971] != 0.0))) && (!(scratch.values[1972] != 0.0))) {
            scratch.store_ad(1385, &AdValue::add(scratch.ad_value(637), AdValue::mul(AdValue::add(scratch.ad_value(1358), AdValue::scale(scratch.ad_value(572), scratch.values[493])), scratch.ad_value(643))));
        }

        if (((scratch.values[1323] != 0.0) && (scratch.values[1340] != 0.0)) && (!(scratch.values[1956] != 0.0))) {
            scratch.store_ad(1387, &AdValue::mul(AdValue::add(AdValue::add(AdValue::add(scratch.ad_value(1360), scratch.ad_value(1361)), scratch.ad_value(1368)), scratch.ad_value(1383)), scratch.ad_value(1385)));
        }

        scratch.values[1974] = if (scratch.values[719] == 0.0) { 1.0 } else { 0.0 };

        if (((scratch.values[1323] != 0.0) && (scratch.values[1340] != 0.0)) && (scratch.values[1974] != 0.0)) {
            scratch.values[1388] = 0.0;
            scratch.node_derivatives[1388] = [0.0; Instance::NODE_COUNT];
            scratch.branch_derivatives[1388] = [0.0; Instance::BRANCH_COUNT];
        }

        scratch.values[1975] = if (scratch.values[602] == 0.5) { 1.0 } else { 0.0 };

        if ((((scratch.values[1323] != 0.0) && (scratch.values[1340] != 0.0)) && (!(scratch.values[1974] != 0.0))) && (scratch.values[1975] != 0.0)) {
            scratch.store_ad(1359, &AdValue::sqrt(AdValue::sub_from_scalar(1.0, AdValue::mul(scratch.ad_value(1351), scratch.ad_value(599)))));
        }

        if ((((scratch.values[1323] != 0.0) && (scratch.values[1340] != 0.0)) && (!(scratch.values[1974] != 0.0))) && (!(scratch.values[1975] != 0.0))) {
            scratch.store_ad(1359, &AdValue::pow(AdValue::sub_from_scalar(1.0, AdValue::mul(scratch.ad_value(1351), scratch.ad_value(599))), scratch.ad_value(602)));
        }

        if (((scratch.values[1323] != 0.0) && (scratch.values[1340] != 0.0)) && (!(scratch.values[1974] != 0.0))) {
            scratch.store_ad(1360, &AdValue::mul(scratch.ad_value(590), scratch.ad_value(1350)));
        }

        scratch.values[1976] = if ((scratch.values[555] == 0.0) && (scratch.values[558] == 0.0)) { 1.0 } else { 0.0 };

        if ((((scratch.values[1323] != 0.0) && (scratch.values[1340] != 0.0)) && (!(scratch.values[1974] != 0.0))) && (scratch.values[1976] != 0.0)) {
            scratch.values[1361] = 0.0;
            scratch.node_derivatives[1361] = [0.0; Instance::NODE_COUNT];
            scratch.branch_derivatives[1361] = [0.0; Instance::BRANCH_COUNT];
        }

        if ((((scratch.values[1323] != 0.0) && (scratch.values[1340] != 0.0)) && (!(scratch.values[1974] != 0.0))) && (!(scratch.values[1976] != 0.0))) {
            scratch.store_ad(1362, &AdValue::sub(scratch.ad_value(596), scratch.ad_value(1356)));
        }

        if ((((scratch.values[1323] != 0.0) && (scratch.values[1340] != 0.0)) && (!(scratch.values[1974] != 0.0))) && (!(scratch.values[1976] != 0.0))) {
            scratch.store_ad(1363, &AdValue::sub_from_scalar(1.0, AdValue::sqrt(AdValue::sub_from_scalar(1.0, AdValue::div(scratch.ad_value(1354), scratch.ad_value(1362))))));
        }

        scratch.values[1977] = if (scratch.values[544] == 0.5) { 1.0 } else { 0.0 };

        if (((((scratch.values[1323] != 0.0) && (scratch.values[1340] != 0.0)) && (!(scratch.values[1974] != 0.0))) && (!(scratch.values[1976] != 0.0))) && (scratch.values[1977] != 0.0)) {
            scratch.values[1364] = 0.0;
            scratch.node_derivatives[1364] = [0.0; Instance::NODE_COUNT];
            scratch.branch_derivatives[1364] = [0.0; Instance::BRANCH_COUNT];
        }

        if (((((scratch.values[1323] != 0.0) && (scratch.values[1340] != 0.0)) && (!(scratch.values[1974] != 0.0))) && (!(scratch.values[1976] != 0.0))) && (!(scratch.values[1977] != 0.0))) {
            scratch.store_ad(1364, &AdValue::mul(AdValue::add(AdValue::div(AdValue::mul(AdValue::square(scratch.ad_value(1363)), AdValue::ln(scratch.ad_value(1363))), AdValue::sub_from_scalar(1.0, scratch.ad_value(1363))), scratch.ad_value(1363)), AdValue::sub_from_scalar(1.0, AdValue::scale(scratch.ad_value(544), 2.0))));
        }

        if ((((scratch.values[1323] != 0.0) && (scratch.values[1340] != 0.0)) && (!(scratch.values[1974] != 0.0))) && (!(scratch.values[1976] != 0.0))) {
            scratch.store_ad(1365, &AdValue::add(scratch.ad_value(1363), scratch.ad_value(1364)));
        }

        scratch.values[1978] = if (scratch.values[544] == 0.5) { 1.0 } else { 0.0 };

        if (((((scratch.values[1323] != 0.0) && (scratch.values[1340] != 0.0)) && (!(scratch.values[1974] != 0.0))) && (!(scratch.values[1976] != 0.0))) && (scratch.values[1978] != 0.0)) {
            scratch.store_ad(1359, &AdValue::sqrt(AdValue::mul(scratch.ad_value(1362), scratch.ad_value(623))));
        }

        if (((((scratch.values[1323] != 0.0) && (scratch.values[1340] != 0.0)) && (!(scratch.values[1974] != 0.0))) && (!(scratch.values[1976] != 0.0))) && (!(scratch.values[1978] != 0.0))) {
            scratch.store_ad(1359, &AdValue::pow(AdValue::mul(scratch.ad_value(1362), scratch.ad_value(623)), scratch.ad_value(544)));
        }

        if ((((scratch.values[1323] != 0.0) && (scratch.values[1340] != 0.0)) && (!(scratch.values[1974] != 0.0))) && (!(scratch.values[1976] != 0.0))) {
            scratch.store_ad(1366, &AdValue::mul(scratch.ad_value(617), scratch.ad_value(1359)));
        }

        if ((((scratch.values[1323] != 0.0) && (scratch.values[1340] != 0.0)) && (!(scratch.values[1974] != 0.0))) && (!(scratch.values[1976] != 0.0))) {
            scratch.store_ad(1367, &AdValue::mul(scratch.ad_value(587), AdValue::mul(AdValue::offset(scratch.ad_value(1353), (-1.0)), scratch.ad_value(1366))));
        }

        if ((((scratch.values[1323] != 0.0) && (scratch.values[1340] != 0.0)) && (!(scratch.values[1974] != 0.0))) && (!(scratch.values[1976] != 0.0))) {
            scratch.store_ad(1361, &AdValue::mul(scratch.ad_value(555), AdValue::mul(scratch.ad_value(1367), scratch.ad_value(1365))));
        }

        scratch.values[1979] = if (scratch.values[558] == 0.0) { 1.0 } else { 0.0 };

        if ((((scratch.values[1323] != 0.0) && (scratch.values[1340] != 0.0)) && (!(scratch.values[1974] != 0.0))) && (scratch.values[1979] != 0.0)) {
            scratch.values[1368] = 0.0;
            scratch.node_derivatives[1368] = [0.0; Instance::NODE_COUNT];
            scratch.branch_derivatives[1368] = [0.0; Instance::BRANCH_COUNT];
        }

        if ((((scratch.values[1323] != 0.0) && (scratch.values[1340] != 0.0)) && (!(scratch.values[1974] != 0.0))) && (!(scratch.values[1979] != 0.0))) {
            scratch.store_ad(1369, &AdValue::mul(scratch.ad_value(632), AdValue::div(AdValue::mul(scratch.ad_value(1366), scratch.ad_value(602)), scratch.ad_value(1362))));
        }

        if ((((scratch.values[1323] != 0.0) && (scratch.values[1340] != 0.0)) && (!(scratch.values[1974] != 0.0))) && (!(scratch.values[1979] != 0.0))) {
            scratch.store_ad(1370, &AdValue::div(AdValue::scale(scratch.ad_value(629), 0.666666666666667), scratch.ad_value(1369)));
        }

        if ((((scratch.values[1323] != 0.0) && (scratch.values[1340] != 0.0)) && (!(scratch.values[1974] != 0.0))) && (!(scratch.values[1979] != 0.0))) {
            scratch.store_ad(1371, &AdValue::square(scratch.ad_value(1370)));
        }

        if ((((scratch.values[1323] != 0.0) && (scratch.values[1340] != 0.0)) && (!(scratch.values[1974] != 0.0))) && (!(scratch.values[1979] != 0.0))) {
            scratch.store_ad(1372, &AdValue::sqrt(AdValue::div(AdValue::square(scratch.ad_value(1371)), AdValue::offset(AdValue::square(scratch.ad_value(1371)), 1.0))));
        }

        if ((((scratch.values[1323] != 0.0) && (scratch.values[1340] != 0.0)) && (!(scratch.values[1974] != 0.0))) && (!(scratch.values[1979] != 0.0))) {
            scratch.store_ad(1373, &AdValue::sqrt(AdValue::abs(scratch.ad_value(1372))));
        }

        if ((((scratch.values[1323] != 0.0) && (scratch.values[1340] != 0.0)) && (!(scratch.values[1974] != 0.0))) && (!(scratch.values[1979] != 0.0))) {
            scratch.store_ad(1374, &AdValue::mul(scratch.ad_value(1372), scratch.ad_value(1373)));
        }

        scratch.values[1980] = if (((-scratch.values[544]) * scratch.values[605]) == (-1.0)) { 1.0 } else { 0.0 };

        if (((((scratch.values[1323] != 0.0) && (scratch.values[1340] != 0.0)) && (!(scratch.values[1974] != 0.0))) && (!(scratch.values[1979] != 0.0))) && (scratch.values[1980] != 0.0)) {
            scratch.store_ad(1375, &AdValue::div_from_scalar(1.0, AdValue::offset(AdValue::mul(scratch.ad_value(1369), scratch.ad_value(1374)), 1.0)));
        }

        if (((((scratch.values[1323] != 0.0) && (scratch.values[1340] != 0.0)) && (!(scratch.values[1974] != 0.0))) && (!(scratch.values[1979] != 0.0))) && (!(scratch.values[1980] != 0.0))) {
            scratch.store_ad(1375, &AdValue::pow(AdValue::offset(AdValue::mul(scratch.ad_value(1369), scratch.ad_value(1374)), 1.0), AdValue::mul(AdValue::neg(scratch.ad_value(544)), scratch.ad_value(605))));
        }

        if ((((scratch.values[1323] != 0.0) && (scratch.values[1340] != 0.0)) && (!(scratch.values[1974] != 0.0))) && (!(scratch.values[1979] != 0.0))) {
            scratch.store_ad(1376, &AdValue::div(AdValue::mul(scratch.ad_value(1365), scratch.ad_value(1375)), AdValue::add(scratch.ad_value(1365), scratch.ad_value(1375))));
        }

        if ((((scratch.values[1323] != 0.0) && (scratch.values[1340] != 0.0)) && (!(scratch.values[1974] != 0.0))) && (!(scratch.values[1979] != 0.0))) {
            scratch.store_ad(1377, &AdValue::sqrt(AdValue::scale(AdValue::div(scratch.ad_value(1369), scratch.ad_value(1373)), 0.375)));
        }

        if ((((scratch.values[1323] != 0.0) && (scratch.values[1340] != 0.0)) && (!(scratch.values[1974] != 0.0))) && (!(scratch.values[1979] != 0.0))) {
            scratch.store_ad(1378, &AdValue::sub(AdValue::scale(AdValue::mul(scratch.ad_value(1370), scratch.ad_value(1373)), 2.0), scratch.ad_value(1372)));
        }

        if ((((scratch.values[1323] != 0.0) && (scratch.values[1340] != 0.0)) && (!(scratch.values[1974] != 0.0))) && (!(scratch.values[1979] != 0.0))) {
            scratch.store_ad(1379, &AdValue::add(AdValue::sub(AdValue::mul(AdValue::mul(scratch.ad_value(629), scratch.ad_value(1370)), scratch.ad_value(1373)), AdValue::mul(scratch.ad_value(629), scratch.ad_value(1372))), AdValue::scale(AdValue::mul(scratch.ad_value(1369), scratch.ad_value(1374)), 0.5)));
        }

        if ((((scratch.values[1323] != 0.0) && (scratch.values[1340] != 0.0)) && (!(scratch.values[1974] != 0.0))) && (!(scratch.values[1979] != 0.0))) {
            scratch.store_ad(1380, &AdValue::mul(AdValue::offset(scratch.ad_value(1378), (-1.0)), scratch.ad_value(1377)));
        }

        if ((((scratch.values[1323] != 0.0) && (scratch.values[1340] != 0.0)) && (!(scratch.values[1974] != 0.0))) && (!(scratch.values[1979] != 0.0))) {
            scratch.store_ad(1341, &AdValue::square(scratch.ad_value(1380)));
        }

        scratch.values[1981] = if (scratch.values[1380] > 0.0) { 1.0 } else { 0.0 };

        if (((((scratch.values[1323] != 0.0) && (scratch.values[1340] != 0.0)) && (!(scratch.values[1974] != 0.0))) && (!(scratch.values[1979] != 0.0))) && (scratch.values[1981] != 0.0)) {
            scratch.store_ad(1342, &AdValue::div_from_scalar(1.0, AdValue::offset(AdValue::scale(scratch.ad_value(1380), scratch.values[421]), 1.0)));
        }

        if (((((scratch.values[1323] != 0.0) && (scratch.values[1340] != 0.0)) && (!(scratch.values[1974] != 0.0))) && (!(scratch.values[1979] != 0.0))) && (!(scratch.values[1981] != 0.0))) {
            scratch.store_ad(1342, &AdValue::div_from_scalar(1.0, AdValue::sub_from_scalar(1.0, AdValue::scale(scratch.ad_value(1380), scratch.values[421]))));
        }

        scratch.values[1982] = if (((-scratch.values[1341]) + scratch.values[1379]) > (-230.25850929940458)) { 1.0 } else { 0.0 };

        if (((((scratch.values[1323] != 0.0) && (scratch.values[1340] != 0.0)) && (!(scratch.values[1974] != 0.0))) && (!(scratch.values[1979] != 0.0))) && (scratch.values[1982] != 0.0)) {
            scratch.store_ad(1359, &AdValue::exp(AdValue::sub(scratch.ad_value(1379), scratch.ad_value(1341))));
        }

        if (((((scratch.values[1323] != 0.0) && (scratch.values[1340] != 0.0)) && (!(scratch.values[1974] != 0.0))) && (!(scratch.values[1979] != 0.0))) && (!(scratch.values[1982] != 0.0))) {
            let assign39410_ad_e51491: AdValue = AdValue::div_from_scalar(1e-100, AdValue::offset(AdValue::mul(AdValue::sub_from_scalar((-230.25850929940458), AdValue::sub(scratch.ad_value(1379), scratch.ad_value(1341))), AdValue::offset(AdValue::scale(AdValue::mul(AdValue::sub_from_scalar((-230.25850929940458), AdValue::sub(scratch.ad_value(1379), scratch.ad_value(1341))), AdValue::offset(AdValue::scale(AdValue::sub_from_scalar((-230.25850929940458), AdValue::sub(scratch.ad_value(1379), scratch.ad_value(1341))), 0.3333333333333333), 1.0)), 0.5), 1.0)), 1.0));
            scratch.store_ad(1359, &assign39410_ad_e51491);
        }

        if ((((scratch.values[1323] != 0.0) && (scratch.values[1340] != 0.0)) && (!(scratch.values[1974] != 0.0))) && (!(scratch.values[1979] != 0.0))) {
            scratch.store_ad(1343, &AdValue::mul(AdValue::add(AdValue::add(AdValue::scale(scratch.ad_value(1342), 0.29214664), AdValue::scale(AdValue::square(scratch.ad_value(1342)), scratch.values[422])), AdValue::scale(AdValue::mul(AdValue::square(scratch.ad_value(1342)), scratch.ad_value(1342)), scratch.values[423])), scratch.ad_value(1359)));
        }

        scratch.values[1983] = if (scratch.values[1380] > 0.0) { 1.0 } else { 0.0 };

        if (((((scratch.values[1323] != 0.0) && (scratch.values[1340] != 0.0)) && (!(scratch.values[1974] != 0.0))) && (!(scratch.values[1979] != 0.0))) && (scratch.values[1983] != 0.0)) {
            scratch.values[1381] = scratch.values[1343];
            scratch.node_derivatives[1381] = scratch.node_derivatives[1343];
            scratch.branch_derivatives[1381] = scratch.branch_derivatives[1343];
        }

        scratch.values[1984] = if (scratch.values[1379] > (-230.25850929940458)) { 1.0 } else { 0.0 };

        if ((((((scratch.values[1323] != 0.0) && (scratch.values[1340] != 0.0)) && (!(scratch.values[1974] != 0.0))) && (!(scratch.values[1979] != 0.0))) && (!(scratch.values[1983] != 0.0))) && (scratch.values[1984] != 0.0)) {
            scratch.store_ad(1359, &AdValue::exp(scratch.ad_value(1379)));
        }

        if ((((((scratch.values[1323] != 0.0) && (scratch.values[1340] != 0.0)) && (!(scratch.values[1974] != 0.0))) && (!(scratch.values[1979] != 0.0))) && (!(scratch.values[1983] != 0.0))) && (!(scratch.values[1984] != 0.0))) {
            scratch.store_ad(1359, &AdValue::div_from_scalar(1e-100, AdValue::offset(AdValue::mul(AdValue::sub_from_scalar((-230.25850929940458), scratch.ad_value(1379)), AdValue::offset(AdValue::scale(AdValue::mul(AdValue::sub_from_scalar((-230.25850929940458), scratch.ad_value(1379)), AdValue::offset(AdValue::scale(AdValue::sub_from_scalar((-230.25850929940458), scratch.ad_value(1379)), 0.3333333333333333), 1.0)), 0.5), 1.0)), 1.0)));
        }

        if (((((scratch.values[1323] != 0.0) && (scratch.values[1340] != 0.0)) && (!(scratch.values[1974] != 0.0))) && (!(scratch.values[1979] != 0.0))) && (!(scratch.values[1983] != 0.0))) {
            scratch.store_ad(1381, &AdValue::sub(AdValue::scale(scratch.ad_value(1359), 2.0), scratch.ad_value(1343)));
        }

        if ((((scratch.values[1323] != 0.0) && (scratch.values[1340] != 0.0)) && (!(scratch.values[1974] != 0.0))) && (!(scratch.values[1979] != 0.0))) {
            scratch.store_ad(1382, &AdValue::scale(AdValue::div(AdValue::mul(scratch.ad_value(629), scratch.ad_value(1381)), scratch.ad_value(1377)), (1.772453850905516 * 0.5)));
        }

        if ((((scratch.values[1323] != 0.0) && (scratch.values[1340] != 0.0)) && (!(scratch.values[1974] != 0.0))) && (!(scratch.values[1979] != 0.0))) {
            scratch.store_ad(1368, &AdValue::mul(scratch.ad_value(558), AdValue::mul(AdValue::mul(scratch.ad_value(1367), scratch.ad_value(1382)), scratch.ad_value(1376))));
        }

        scratch.values[1985] = if (scratch.values[564] == 0.0) { 1.0 } else { 0.0 };

        if ((((scratch.values[1323] != 0.0) && (scratch.values[1340] != 0.0)) && (!(scratch.values[1974] != 0.0))) && (scratch.values[1985] != 0.0)) {
            scratch.values[1383] = 0.0;
            scratch.node_derivatives[1383] = [0.0; Instance::NODE_COUNT];
            scratch.branch_derivatives[1383] = [0.0; Instance::BRANCH_COUNT];
        }

        scratch.values[1986] = if (scratch.values[544] == 0.5) { 1.0 } else { 0.0 };

        if (((((scratch.values[1323] != 0.0) && (scratch.values[1340] != 0.0)) && (!(scratch.values[1974] != 0.0))) && (!(scratch.values[1985] != 0.0))) && (scratch.values[1986] != 0.0)) {
            scratch.store_ad(1359, &AdValue::sqrt(AdValue::mul(AdValue::sub(scratch.ad_value(541), scratch.ad_value(1357)), scratch.ad_value(623))));
        }

        if (((((scratch.values[1323] != 0.0) && (scratch.values[1340] != 0.0)) && (!(scratch.values[1974] != 0.0))) && (!(scratch.values[1985] != 0.0))) && (!(scratch.values[1986] != 0.0))) {
            scratch.store_ad(1359, &AdValue::pow(AdValue::mul(AdValue::sub(scratch.ad_value(541), scratch.ad_value(1357)), scratch.ad_value(623)), scratch.ad_value(544)));
        }

        if ((((scratch.values[1323] != 0.0) && (scratch.values[1340] != 0.0)) && (!(scratch.values[1974] != 0.0))) && (!(scratch.values[1985] != 0.0))) {
            scratch.store_ad(1384, &AdValue::mul(scratch.ad_value(605), AdValue::div(AdValue::mul(AdValue::sub(scratch.ad_value(541), scratch.ad_value(1357)), scratch.ad_value(620)), scratch.ad_value(1359))));
        }

        scratch.values[1987] = if (((((-scratch.values[635]) / scratch.values[1384])) as f64).abs() < 230.25850929940458) { 1.0 } else { 0.0 };

        if (((((scratch.values[1323] != 0.0) && (scratch.values[1340] != 0.0)) && (!(scratch.values[1974] != 0.0))) && (!(scratch.values[1985] != 0.0))) && (scratch.values[1987] != 0.0)) {
            scratch.store_ad(1359, &AdValue::exp(AdValue::div(AdValue::neg(scratch.ad_value(635)), scratch.ad_value(1384))));
        }

        scratch.values[1988] = if (((-scratch.values[635]) / scratch.values[1384]) < 0.0) { 1.0 } else { 0.0 };

        if ((((((scratch.values[1323] != 0.0) && (scratch.values[1340] != 0.0)) && (!(scratch.values[1974] != 0.0))) && (!(scratch.values[1985] != 0.0))) && (!(scratch.values[1987] != 0.0))) && (scratch.values[1988] != 0.0)) {
            let assign39600_ad_e51818: AdValue = AdValue::offset(AdValue::mul(AdValue::sub_from_scalar((-230.25850929940458), AdValue::div(AdValue::neg(scratch.ad_value(635)), scratch.ad_value(1384))), AdValue::offset(AdValue::scale(AdValue::mul(AdValue::sub_from_scalar((-230.25850929940458), AdValue::div(AdValue::neg(scratch.ad_value(635)), scratch.ad_value(1384))), AdValue::offset(AdValue::scale(AdValue::sub_from_scalar((-230.25850929940458), AdValue::div(AdValue::neg(scratch.ad_value(635)), scratch.ad_value(1384))), 0.3333333333333333), 1.0)), 0.5), 1.0)), 1.0);
            scratch.store_ad(1359, &AdValue::div_from_scalar(1e-100, assign39600_ad_e51818));
        }

        if ((((((scratch.values[1323] != 0.0) && (scratch.values[1340] != 0.0)) && (!(scratch.values[1974] != 0.0))) && (!(scratch.values[1985] != 0.0))) && (!(scratch.values[1987] != 0.0))) && (!(scratch.values[1988] != 0.0))) {
            let assign39610_ad_e51868: AdValue = AdValue::scale(AdValue::offset(AdValue::mul(AdValue::offset(AdValue::div(AdValue::neg(scratch.ad_value(635)), scratch.ad_value(1384)), (-230.25850929940458)), AdValue::offset(AdValue::scale(AdValue::mul(AdValue::offset(AdValue::div(AdValue::neg(scratch.ad_value(635)), scratch.ad_value(1384)), (-230.25850929940458)), AdValue::offset(AdValue::scale(AdValue::offset(AdValue::div(AdValue::neg(scratch.ad_value(635)), scratch.ad_value(1384)), (-230.25850929940458)), 0.3333333333333333), 1.0)), 0.5), 1.0)), 1.0), 1e100);
            scratch.store_ad(1359, &assign39610_ad_e51868);
        }

        if ((((scratch.values[1323] != 0.0) && (scratch.values[1340] != 0.0)) && (!(scratch.values[1974] != 0.0))) && (!(scratch.values[1985] != 0.0))) {
            scratch.store_ad(1383, &AdValue::mul(scratch.ad_value(564), AdValue::mul(AdValue::mul(AdValue::mul(scratch.ad_value(520), scratch.ad_value(1384)), scratch.ad_value(1384)), scratch.ad_value(1359))));
        }

        scratch.values[1989] = if (scratch.values[573] > 1000.0) { 1.0 } else { 0.0 };

        if ((((scratch.values[1323] != 0.0) && (scratch.values[1340] != 0.0)) && (!(scratch.values[1974] != 0.0))) && (scratch.values[1989] != 0.0)) {
            scratch.values[1385] = 1.0;
            scratch.node_derivatives[1385] = [0.0; Instance::NODE_COUNT];
            scratch.branch_derivatives[1385] = [0.0; Instance::BRANCH_COUNT];
        }

        scratch.values[1990] = if (scratch.values[1358] > ((-scratch.values[493]) * scratch.values[573])) { 1.0 } else { 0.0 };

        scratch.values[1991] = if (scratch.values[576] == 4.0) { 1.0 } else { 0.0 };

        if ((((((scratch.values[1323] != 0.0) && (scratch.values[1340] != 0.0)) && (!(scratch.values[1974] != 0.0))) && (!(scratch.values[1989] != 0.0))) && (scratch.values[1990] != 0.0)) && (scratch.values[1991] != 0.0)) {
            scratch.store_ad(1359, &AdValue::mul(AdValue::mul(AdValue::mul(AdValue::mul(scratch.ad_value(1358), scratch.ad_value(641)), AdValue::mul(scratch.ad_value(1358), scratch.ad_value(641))), AdValue::mul(scratch.ad_value(1358), scratch.ad_value(641))), AdValue::mul(scratch.ad_value(1358), scratch.ad_value(641))));
        }

        if ((((((scratch.values[1323] != 0.0) && (scratch.values[1340] != 0.0)) && (!(scratch.values[1974] != 0.0))) && (!(scratch.values[1989] != 0.0))) && (scratch.values[1990] != 0.0)) && (!(scratch.values[1991] != 0.0))) {
            scratch.store_ad(1359, &AdValue::pow(AdValue::abs(AdValue::mul(scratch.ad_value(1358), scratch.ad_value(641))), scratch.ad_value(576)));
        }

        if (((((scratch.values[1323] != 0.0) && (scratch.values[1340] != 0.0)) && (!(scratch.values[1974] != 0.0))) && (!(scratch.values[1989] != 0.0))) && (scratch.values[1990] != 0.0)) {
            scratch.store_ad(1385, &AdValue::div_from_scalar(1.0, AdValue::sub_from_scalar(1.0, scratch.ad_value(1359))));
        }

        if (((((scratch.values[1323] != 0.0) && (scratch.values[1340] != 0.0)) && (!(scratch.values[1974] != 0.0))) && (!(scratch.values[1989] != 0.0))) && (!(scratch.values[1990] != 0.0))) {
            scratch.store_ad(1385, &AdValue::add(scratch.ad_value(638), AdValue::mul(AdValue::add(scratch.ad_value(1358), AdValue::scale(scratch.ad_value(573), scratch.values[493])), scratch.ad_value(644))));
        }

        if (((scratch.values[1323] != 0.0) && (scratch.values[1340] != 0.0)) && (!(scratch.values[1974] != 0.0))) {
            scratch.store_ad(1388, &AdValue::mul(AdValue::add(AdValue::add(AdValue::add(scratch.ad_value(1360), scratch.ad_value(1361)), scratch.ad_value(1368)), scratch.ad_value(1383)), scratch.ad_value(1385)));
        }

        if ((scratch.values[1323] != 0.0) && (scratch.values[1340] != 0.0)) {
            scratch.store_ad(510, &AdValue::add(AdValue::add(AdValue::mul(scratch.ad_value(717), scratch.ad_value(1386)), AdValue::mul(scratch.ad_value(718), scratch.ad_value(1387))), AdValue::mul(scratch.ad_value(719), scratch.ad_value(1388))));
        }

        if ((scratch.values[1323] != 0.0) && (scratch.values[1340] != 0.0)) {
            scratch.store_ad(738, &AdValue::add(AdValue::add(AdValue::mul(scratch.ad_value(717), scratch.ad_value(588)), AdValue::mul(scratch.ad_value(718), scratch.ad_value(589))), AdValue::mul(scratch.ad_value(719), scratch.ad_value(590))));
        }

        if ((scratch.values[1323] != 0.0) && (scratch.values[1340] != 0.0)) {
            scratch.store_ad(514, &AdValue::sub(scratch.ad_value(509), AdValue::mul(scratch.ad_value(738), AdValue::offset(AdValue::exp(AdValue::scale(scratch.ad_value(519), (scratch.values[420] * scratch.values[739]))), (-1.0)))));
        }

        if ((scratch.values[1323] != 0.0) && (scratch.values[1340] != 0.0)) {
            scratch.store_ad(515, &AdValue::sub(scratch.ad_value(510), AdValue::mul(scratch.ad_value(738), AdValue::offset(AdValue::exp(AdValue::scale(scratch.ad_value(520), (scratch.values[420] * scratch.values[739]))), (-1.0)))));
        }

        scratch.values[1992] = if !(((scratch.values[717] == 0.0) && (scratch.values[718] == 0.0)) && (scratch.values[719] == 0.0)) { 1.0 } else { 0.0 };

        scratch.values[1993] = if ((scratch.values[509] > 0.0) && (scratch.values[510] > 0.0)) { 1.0 } else { 0.0 };

        scratch.values[1994] = if (((((scratch.values[514] / scratch.values[509]) > 0.001) || ((scratch.values[515] / scratch.values[510]) > 0.001)) && (scratch.values[514] > 0.0)) && (scratch.values[515] > 0.0)) { 1.0 } else { 0.0 };

        if (((((scratch.values[1323] != 0.0) && (scratch.values[1340] != 0.0)) && (scratch.values[1992] != 0.0)) && (scratch.values[1993] != 0.0)) && (scratch.values[1994] != 0.0)) {
            scratch.store_ad(521, &AdValue::div(scratch.ad_value(514), scratch.ad_value(515)));
        }

        if (((((scratch.values[1323] != 0.0) && (scratch.values[1340] != 0.0)) && (scratch.values[1992] != 0.0)) && (scratch.values[1993] != 0.0)) && (scratch.values[1994] != 0.0)) {
            scratch.store_ad(741, &AdValue::div(AdValue::scale(AdValue::ln(scratch.ad_value(521)), scratch.values[419]), AdValue::sub(scratch.ad_value(519), scratch.ad_value(520))));
        }

        if (((((scratch.values[1323] != 0.0) && (scratch.values[1340] != 0.0)) && (scratch.values[1992] != 0.0)) && (scratch.values[1993] != 0.0)) && (scratch.values[1994] != 0.0)) {
            scratch.store_ad(740, &AdValue::div(scratch.ad_value(514), AdValue::offset(AdValue::exp(AdValue::mul(AdValue::scale(scratch.ad_value(519), scratch.values[420]), scratch.ad_value(741))), (-1.0))));
        }

        if (((scratch.values[1323] != 0.0) && (scratch.values[1340] != 0.0)) && (scratch.values[1992] != 0.0)) {
            scratch.store_ad(511, &AdValue::sub(AdValue::sub(scratch.ad_value(506), AdValue::mul(scratch.ad_value(738), AdValue::offset(AdValue::exp(AdValue::scale(scratch.ad_value(516), (scratch.values[420] * scratch.values[739]))), (-1.0)))), AdValue::mul(scratch.ad_value(740), AdValue::offset(AdValue::exp(AdValue::mul(AdValue::scale(scratch.ad_value(516), scratch.values[420]), scratch.ad_value(741))), (-1.0)))));
        }

        if (((scratch.values[1323] != 0.0) && (scratch.values[1340] != 0.0)) && (scratch.values[1992] != 0.0)) {
            scratch.store_ad(512, &AdValue::sub(AdValue::sub(scratch.ad_value(507), AdValue::mul(scratch.ad_value(738), AdValue::offset(AdValue::exp(AdValue::scale(scratch.ad_value(517), (scratch.values[420] * scratch.values[739]))), (-1.0)))), AdValue::mul(scratch.ad_value(740), AdValue::offset(AdValue::exp(AdValue::mul(AdValue::scale(scratch.ad_value(517), scratch.values[420]), scratch.ad_value(741))), (-1.0)))));
        }

        if (((scratch.values[1323] != 0.0) && (scratch.values[1340] != 0.0)) && (scratch.values[1992] != 0.0)) {
            scratch.store_ad(513, &AdValue::sub(AdValue::sub(scratch.ad_value(508), AdValue::mul(scratch.ad_value(738), AdValue::offset(AdValue::exp(AdValue::scale(scratch.ad_value(518), (scratch.values[420] * scratch.values[739]))), (-1.0)))), AdValue::mul(scratch.ad_value(740), AdValue::offset(AdValue::exp(AdValue::mul(AdValue::scale(scratch.ad_value(518), scratch.values[420]), scratch.ad_value(741))), (-1.0)))));
        }

        scratch.values[1995] = if (((scratch.values[506] < 0.0) && (scratch.values[507] < 0.0)) && (scratch.values[508] < 0.0)) { 1.0 } else { 0.0 };

        scratch.values[1996] = if (((((((scratch.values[511] / scratch.values[506]) > 0.001) || ((scratch.values[512] / scratch.values[507]) > 0.001)) || ((scratch.values[513] / scratch.values[508]) > 0.001)) && (scratch.values[511] < 0.0)) && (scratch.values[512] < 0.0)) && (scratch.values[513] < 0.0)) { 1.0 } else { 0.0 };

        if (((((scratch.values[1323] != 0.0) && (scratch.values[1340] != 0.0)) && (scratch.values[1992] != 0.0)) && (scratch.values[1995] != 0.0)) && (scratch.values[1996] != 0.0)) {
            scratch.store_ad(521, &AdValue::div(scratch.ad_value(511), scratch.ad_value(512)));
        }

        if (((((scratch.values[1323] != 0.0) && (scratch.values[1340] != 0.0)) && (scratch.values[1992] != 0.0)) && (scratch.values[1995] != 0.0)) && (scratch.values[1996] != 0.0)) {
            scratch.store_ad(522, &AdValue::div(AdValue::scale(AdValue::ln(scratch.ad_value(521)), (-scratch.values[419])), AdValue::sub(scratch.ad_value(516), scratch.ad_value(517))));
        }

        if (((((scratch.values[1323] != 0.0) && (scratch.values[1340] != 0.0)) && (scratch.values[1992] != 0.0)) && (scratch.values[1995] != 0.0)) && (scratch.values[1996] != 0.0)) {
            scratch.store_ad(524, &AdValue::div(scratch.ad_value(517), AdValue::sub(scratch.ad_value(517), scratch.ad_value(516))));
        }

        if (((((scratch.values[1323] != 0.0) && (scratch.values[1340] != 0.0)) && (scratch.values[1992] != 0.0)) && (scratch.values[1995] != 0.0)) && (scratch.values[1996] != 0.0)) {
            scratch.store_ad(525, &AdValue::mul(AdValue::scale(AdValue::offset(scratch.ad_value(521), (-1.0)), scratch.values[419]), AdValue::offset(AdValue::pow(scratch.ad_value(521), scratch.ad_value(524)), (-1.0))));
        }

        if (((((scratch.values[1323] != 0.0) && (scratch.values[1340] != 0.0)) && (scratch.values[1992] != 0.0)) && (scratch.values[1995] != 0.0)) && (scratch.values[1996] != 0.0)) {
            scratch.store_ad(524, &AdValue::div(scratch.ad_value(516), AdValue::sub(scratch.ad_value(516), scratch.ad_value(517))));
        }

        if (((((scratch.values[1323] != 0.0) && (scratch.values[1340] != 0.0)) && (scratch.values[1992] != 0.0)) && (scratch.values[1995] != 0.0)) && (scratch.values[1996] != 0.0)) {
            scratch.store_ad(526, &AdValue::sub(AdValue::add(AdValue::mul(AdValue::pow(scratch.ad_value(521), scratch.ad_value(524)), AdValue::sub(scratch.ad_value(517), scratch.ad_value(516))), AdValue::mul(scratch.ad_value(521), scratch.ad_value(516))), scratch.ad_value(517)));
        }

        if (((((scratch.values[1323] != 0.0) && (scratch.values[1340] != 0.0)) && (scratch.values[1992] != 0.0)) && (scratch.values[1995] != 0.0)) && (scratch.values[1996] != 0.0)) {
            scratch.store_ad(523, &AdValue::div(scratch.ad_value(525), scratch.ad_value(526)));
        }

    }

    pub(super) fn stamp_transient_block_30(
        &mut self,
        ctx: &GeneratedEvalContext<'_>,
        stamper: &mut GeneratedStamper<'_>,
        scratch: &mut Scratch,
    ) {
        let _ = stamper;
        if (((((scratch.values[1323] != 0.0) && (scratch.values[1340] != 0.0)) && (scratch.values[1992] != 0.0)) && (scratch.values[1995] != 0.0)) && (scratch.values[1996] != 0.0)) {
            scratch.store_ad(743, &AdValue::add(scratch.ad_value(522), scratch.ad_value(523)));
        }

        scratch.values[1997] = if (((((scratch.values[518] * scratch.values[420]) * scratch.values[743])) as f64).abs() < 1e-6) { 1.0 } else { 0.0 };

        if ((((((scratch.values[1323] != 0.0) && (scratch.values[1340] != 0.0)) && (scratch.values[1992] != 0.0)) && (scratch.values[1995] != 0.0)) && (scratch.values[1996] != 0.0)) && (scratch.values[1997] != 0.0)) {
            scratch.values[737] = 1.0;
            scratch.node_derivatives[737] = [0.0; Instance::NODE_COUNT];
            scratch.branch_derivatives[737] = [0.0; Instance::BRANCH_COUNT];
        }

        if ((((((scratch.values[1323] != 0.0) && (scratch.values[1340] != 0.0)) && (scratch.values[1992] != 0.0)) && (scratch.values[1995] != 0.0)) && (scratch.values[1996] != 0.0)) && (scratch.values[1997] != 0.0)) {
            scratch.store_ad(742, &AdValue::mul(scratch.ad_value(513), AdValue::add(AdValue::div_from_scalar(1.0, scratch.ad_value(518)), AdValue::scale(scratch.ad_value(743), (0.5 * scratch.values[420])))));
        }

        if ((((((scratch.values[1323] != 0.0) && (scratch.values[1340] != 0.0)) && (scratch.values[1992] != 0.0)) && (scratch.values[1995] != 0.0)) && (scratch.values[1996] != 0.0)) && (scratch.values[1997] != 0.0)) {
            scratch.store_ad(743, &AdValue::div(AdValue::scale(AdValue::mul(AdValue::scale(scratch.ad_value(513), (-0.5)), scratch.ad_value(743)), scratch.values[420]), scratch.ad_value(518)));
        }

        if ((((((scratch.values[1323] != 0.0) && (scratch.values[1340] != 0.0)) && (scratch.values[1992] != 0.0)) && (scratch.values[1995] != 0.0)) && (scratch.values[1996] != 0.0)) && (!(scratch.values[1997] != 0.0))) {
            scratch.values[737] = 0.0;
            scratch.node_derivatives[737] = [0.0; Instance::NODE_COUNT];
            scratch.branch_derivatives[737] = [0.0; Instance::BRANCH_COUNT];
        }

        if ((((((scratch.values[1323] != 0.0) && (scratch.values[1340] != 0.0)) && (scratch.values[1992] != 0.0)) && (scratch.values[1995] != 0.0)) && (scratch.values[1996] != 0.0)) && (!(scratch.values[1997] != 0.0))) {
            scratch.store_ad(742, &AdValue::div(AdValue::neg(scratch.ad_value(513)), AdValue::offset(AdValue::exp(AdValue::mul(AdValue::scale(AdValue::neg(scratch.ad_value(518)), scratch.values[420]), scratch.ad_value(743))), (-1.0))));
        }

        if ((scratch.values[1323] != 0.0) && (scratch.values[1340] != 0.0)) {
            scratch.store_ad(532, &AdValue::mul(scratch.ad_value(578), AdValue::add(AdValue::add(AdValue::mul(scratch.ad_value(717), scratch.ad_value(606)), AdValue::mul(scratch.ad_value(718), scratch.ad_value(607))), AdValue::mul(scratch.ad_value(719), scratch.ad_value(608)))));
        }

        scratch.values[1998] = if ((scratch.values[717] * scratch.values[606]) <= scratch.values[532]) { 1.0 } else { 0.0 };

        if (((scratch.values[1323] != 0.0) && (scratch.values[1340] != 0.0)) && (scratch.values[1998] != 0.0)) {
            scratch.values[722] = 0.0;
            scratch.node_derivatives[722] = [0.0; Instance::NODE_COUNT];
            scratch.branch_derivatives[722] = [0.0; Instance::BRANCH_COUNT];
        }

        scratch.values[1999] = if ((scratch.values[718] * scratch.values[607]) <= scratch.values[532]) { 1.0 } else { 0.0 };

        if (((scratch.values[1323] != 0.0) && (scratch.values[1340] != 0.0)) && (scratch.values[1999] != 0.0)) {
            scratch.values[723] = 0.0;
            scratch.node_derivatives[723] = [0.0; Instance::NODE_COUNT];
            scratch.branch_derivatives[723] = [0.0; Instance::BRANCH_COUNT];
        }

        scratch.values[2000] = if ((scratch.values[719] * scratch.values[608]) <= scratch.values[532]) { 1.0 } else { 0.0 };

        if (((scratch.values[1323] != 0.0) && (scratch.values[1340] != 0.0)) && (scratch.values[2000] != 0.0)) {
            scratch.values[724] = 0.0;
            scratch.node_derivatives[724] = [0.0; Instance::NODE_COUNT];
            scratch.branch_derivatives[724] = [0.0; Instance::BRANCH_COUNT];
        }

        scratch.values[2001] = if !(((scratch.values[717] == 0.0) && (scratch.values[718] == 0.0)) && (scratch.values[719] == 0.0)) { 1.0 } else { 0.0 };

        if (((scratch.values[1323] != 0.0) && (scratch.values[1340] != 0.0)) && (scratch.values[2001] != 0.0)) {
            scratch.store_ad(731, &AdValue::ln(AdValue::div_from_scalar((0.5 * scratch.values[367]), AdValue::offset(scratch.ad_value(738), 1e-21))));
        }

        if (((scratch.values[1323] != 0.0) && (scratch.values[1340] != 0.0)) && (scratch.values[2001] != 0.0)) {
            scratch.store_ad(733, &AdValue::ln(AdValue::div_from_scalar((0.5 * scratch.values[367]), AdValue::offset(scratch.ad_value(740), 1e-21))));
        }

        if (((scratch.values[1323] != 0.0) && (scratch.values[1340] != 0.0)) && (scratch.values[2001] != 0.0)) {
            scratch.store_ad(735, &AdValue::ln(AdValue::div_from_scalar((0.5 * scratch.values[367]), AdValue::offset(AdValue::abs(scratch.ad_value(742)), 1e-21))));
        }

        if ((scratch.values[1323] != 0.0) && (scratch.values[1340] != 0.0)) {
            scratch.store_ad(731, &AdValue::min_with_scalar(scratch.ad_value(731), 230.25850929940458));
        }

        if ((scratch.values[1323] != 0.0) && (scratch.values[1340] != 0.0)) {
            scratch.store_ad(732, &AdValue::exp(scratch.ad_value(731)));
        }

        if ((scratch.values[1323] != 0.0) && (scratch.values[1340] != 0.0)) {
            scratch.store_ad(733, &AdValue::min_with_scalar(scratch.ad_value(733), 230.25850929940458));
        }

        if ((scratch.values[1323] != 0.0) && (scratch.values[1340] != 0.0)) {
            scratch.store_ad(734, &AdValue::exp(scratch.ad_value(733)));
        }

        if ((scratch.values[1323] != 0.0) && (scratch.values[1340] != 0.0)) {
            scratch.store_ad(735, &AdValue::min_with_scalar(scratch.ad_value(735), 230.25850929940458));
        }

        if ((scratch.values[1323] != 0.0) && (scratch.values[1340] != 0.0)) {
            scratch.store_ad(736, &AdValue::exp(scratch.ad_value(735)));
        }

        scratch.values[2076] = 0.0;

        scratch.values[2077] = 0.0;

        scratch.values[2078] = 0.0;

        scratch.values[943] = 0.0;

        scratch.values[891] = 0.0;

        scratch.values[892] = 0.0;

        scratch.values[893] = 0.0;

        scratch.values[894] = 0.0;

        scratch.values[2022] = 0.0;

        scratch.values[2023] = 0.0;

        scratch.values[2079] = if (scratch.values[0] == 1.0) { 1.0 } else { 0.0 };

        if (scratch.values[2079] != 0.0) {
            scratch.store_ad(853, &AdValue::voltage(ctx, &self.nodes, Some(5), Some(6)));
        }

        if (scratch.values[2079] != 0.0) {
            scratch.store_ad(854, &AdValue::voltage(ctx, &self.nodes, Some(7), Some(6)));
        }

        if (scratch.values[2079] != 0.0) {
            scratch.store_ad(855, &AdValue::voltage(ctx, &self.nodes, Some(6), Some(8)));
        }

        if (scratch.values[2079] != 0.0) {
            scratch.store_ad(858, &AdValue::neg(AdValue::voltage(ctx, &self.nodes, Some(6), Some(10))));
        }

        if (scratch.values[2079] != 0.0) {
            scratch.store_ad(859, &AdValue::neg(AdValue::voltage(ctx, &self.nodes, Some(7), Some(11))));
        }

        if (!(scratch.values[2079] != 0.0)) {
            scratch.store_ad(853, &AdValue::neg(AdValue::voltage(ctx, &self.nodes, Some(5), Some(6))));
        }

        if (!(scratch.values[2079] != 0.0)) {
            scratch.store_ad(854, &AdValue::neg(AdValue::voltage(ctx, &self.nodes, Some(7), Some(6))));
        }

        if (!(scratch.values[2079] != 0.0)) {
            scratch.store_ad(855, &AdValue::neg(AdValue::voltage(ctx, &self.nodes, Some(6), Some(8))));
        }

        if (!(scratch.values[2079] != 0.0)) {
            scratch.store_ad(858, &AdValue::voltage(ctx, &self.nodes, Some(6), Some(10)));
        }

        if (!(scratch.values[2079] != 0.0)) {
            scratch.store_ad(859, &AdValue::voltage(ctx, &self.nodes, Some(7), Some(11)));
        }

        scratch.values[860] = scratch.values[853];
        scratch.node_derivatives[860] = scratch.node_derivatives[853];
        scratch.branch_derivatives[860] = scratch.branch_derivatives[853];

        scratch.values[861] = scratch.values[855];
        scratch.node_derivatives[861] = scratch.node_derivatives[855];
        scratch.branch_derivatives[861] = scratch.branch_derivatives[855];

        scratch.store_ad(862, &AdValue::add(scratch.ad_value(854), scratch.ad_value(855)));

        scratch.store_ad(863, &AdValue::sub(scratch.ad_value(853), scratch.ad_value(854)));

        scratch.store_ad(871, &AdValue::scale(AdValue::neg(scratch.ad_value(860)), scratch.values[357]));

        scratch.store_ad(872, &AdValue::scale(AdValue::neg(scratch.ad_value(863)), scratch.values[357]));

        scratch.values[2002] = 1.0;

        scratch.values[2087] = if (scratch.values[854] < 0.0) { 1.0 } else { 0.0 };

        if (scratch.values[2087] != 0.0) {
            scratch.values[2002] = (-1.0);
            scratch.node_derivatives[2002] = [0.0; Instance::NODE_COUNT];
            scratch.branch_derivatives[2002] = [0.0; Instance::BRANCH_COUNT];
        }

        if (scratch.values[2087] != 0.0) {
            scratch.store_ad(853, &AdValue::sub(scratch.ad_value(853), scratch.ad_value(854)));
        }

        if (scratch.values[2087] != 0.0) {
            scratch.store_ad(855, &AdValue::add(scratch.ad_value(855), scratch.ad_value(854)));
        }

        if (scratch.values[2087] != 0.0) {
            scratch.store_ad(854, &AdValue::neg(scratch.ad_value(854)));
        }

        scratch.store_ad(856, &AdValue::add(scratch.ad_value(854), scratch.ad_value(855)));

        scratch.store_ad(864, &AdValue::div(AdValue::square(scratch.ad_value(854)), AdValue::offset(AdValue::sqrt(AdValue::offset(AdValue::square(scratch.ad_value(854)), 0.01)), 0.1)));

        scratch.store_ad(2076, &AdValue::add(AdValue::scale(AdValue::sub(AdValue::add(scratch.ad_value(856), scratch.ad_value(855)), AdValue::sqrt(AdValue::add(AdValue::mul(AdValue::sub(scratch.ad_value(856), scratch.ad_value(855)), AdValue::sub(scratch.ad_value(856), scratch.ad_value(855))), scratch.ad_value(782)))), 0.5), scratch.ad_value(780)));

        scratch.store_ad(873, &AdValue::add(AdValue::sub(scratch.ad_value(855), AdValue::scale(AdValue::sub(scratch.ad_value(2076), AdValue::sqrt(AdValue::add(AdValue::mul(scratch.ad_value(2076), scratch.ad_value(2076)), scratch.ad_value(781)))), 0.5)), scratch.ad_value(784)));

        scratch.values[874] = scratch.values[873];
        scratch.node_derivatives[874] = scratch.node_derivatives[873];
        scratch.branch_derivatives[874] = scratch.branch_derivatives[873];

        scratch.values[2208] = if ((scratch.values[7] != 0.0) && (scratch.values[197] != 1.0)) { 1.0 } else { 0.0 };

        if (scratch.values[2208] != 0.0) {
            scratch.store_ad(875, &AdValue::add(scratch.ad_value(873), AdValue::scale(AdValue::sub(scratch.ad_value(854), scratch.ad_value(864)), 0.5)));
        }

        if (scratch.values[2208] != 0.0) {
            scratch.store_ad(876, &AdValue::sub(AdValue::sqrt(AdValue::add(scratch.ad_value(875), scratch.ad_value(771))), scratch.ad_value(779)));
        }

        if (scratch.values[2208] != 0.0) {
            scratch.store_ad(2076, &AdValue::offset(AdValue::div(AdValue::scale(AdValue::sub(scratch.ad_value(876), scratch.ad_value(786)), 2.0), scratch.ad_value(787)), (-1.0)));
        }

        if (scratch.values[2208] != 0.0) {
            scratch.store_ad(877, &AdValue::sub(scratch.ad_value(876), AdValue::mul(AdValue::mul(AdValue::scale(AdValue::sub_from_scalar(1.0, scratch.ad_value(197)), 0.25), scratch.ad_value(787)), AdValue::add(scratch.ad_value(2076), AdValue::sqrt(AdValue::offset(AdValue::square(scratch.ad_value(2076)), 0.4804530139182))))));
        }

        if (scratch.values[2208] != 0.0) {
            scratch.store_ad(878, &AdValue::add(AdValue::square(scratch.ad_value(877)), AdValue::mul(AdValue::scale(scratch.ad_value(779), 2.0), scratch.ad_value(877))));
        }

        if (scratch.values[2208] != 0.0) {
            scratch.store_ad(873, &AdValue::sub(scratch.ad_value(878), AdValue::scale(AdValue::sub(scratch.ad_value(854), scratch.ad_value(864)), 0.5)));
        }

        scratch.values[2088] = scratch.values[771];
        scratch.node_derivatives[2088] = scratch.node_derivatives[771];
        scratch.branch_derivatives[2088] = scratch.branch_derivatives[771];

        scratch.values[2089] = scratch.values[772];
        scratch.node_derivatives[2089] = scratch.node_derivatives[772];
        scratch.branch_derivatives[2089] = scratch.branch_derivatives[772];

        scratch.values[2090] = scratch.values[873];
        scratch.node_derivatives[2090] = scratch.node_derivatives[873];
        scratch.branch_derivatives[2090] = scratch.branch_derivatives[873];

        scratch.values[2202] = 1.0;

        scratch.values[2153] = 0.0;

        scratch.values[2169] = 1.0;

        scratch.values[2168] = 0.0;

        scratch.values[2164] = 0.0;

        scratch.values[2141] = 0.0;

        scratch.values[2162] = 0.0;

        scratch.values[2163] = 0.0;

        scratch.values[2176] = 1.0;

        scratch.values[2167] = 0.0;

        scratch.values[2151] = 1.0;

        scratch.values[2172] = 1.0;

        scratch.values[2173] = 1.0;

        scratch.values[2198] = 0.0;

        scratch.values[2103] = 0.0;

        scratch.values[2152] = 0.0;

        scratch.values[2120] = 0.0;

        scratch.values[2115] = 0.0;

        scratch.values[2119] = 1.0;

        scratch.values[2170] = 1.0;

        scratch.values[2133] = 0.0;

        scratch.values[2122] = 0.0;

        scratch.values[2150] = 0.0;

        scratch.store_ad(865, &AdValue::add(scratch.ad_value(853), scratch.ad_value(2090)));

        scratch.store_ad(866, &AdValue::sub(scratch.ad_value(865), scratch.ad_value(744)));

        scratch.store_ad(2091, &AdValue::add(scratch.ad_value(2090), AdValue::scale(AdValue::sub(scratch.ad_value(854), scratch.ad_value(864)), 0.5)));

        scratch.values[2209] = if (scratch.values[214] < 1e-10) { 1.0 } else { 0.0 };

        if (scratch.values[2209] != 0.0) {
            scratch.values[867] = scratch.values[864];
            scratch.node_derivatives[867] = scratch.node_derivatives[864];
            scratch.branch_derivatives[867] = scratch.branch_derivatives[864];
        }

        if (!(scratch.values[2209] != 0.0)) {
            scratch.store_ad(867, &AdValue::div(AdValue::scale(AdValue::offset(AdValue::sqrt(AdValue::offset(AdValue::mul(scratch.ad_value(214), scratch.ad_value(864)), 1.0)), (-1.0)), 2.0), scratch.ad_value(214)));
        }

        scratch.store_ad(868, &AdValue::mul(AdValue::mul(scratch.ad_value(213), scratch.ad_value(867)), AdValue::offset(AdValue::mul(scratch.ad_value(215), scratch.ad_value(2091)), 1.0)));

        scratch.store_ad(2003, &AdValue::mul(AdValue::mul(scratch.ad_value(216), AdValue::offset(AdValue::mul(scratch.ad_value(218), scratch.ad_value(864)), 1.0)), AdValue::offset(AdValue::mul(scratch.ad_value(217), scratch.ad_value(2091)), 1.0)));

        scratch.store_ad(866, &AdValue::add(scratch.ad_value(866), scratch.ad_value(868)));

        scratch.values[2210] = if (scratch.values[202] > 0.0) { 1.0 } else { 0.0 };

        if (scratch.values[2210] != 0.0) {
            scratch.store_ad(2093, &AdValue::mul(AdValue::scale(scratch.ad_value(202), 0.5), AdValue::add(AdValue::sub(AdValue::add(scratch.ad_value(853), scratch.ad_value(855)), scratch.ad_value(200)), AdValue::sqrt(AdValue::add(AdValue::mul(AdValue::neg(AdValue::sub(AdValue::add(scratch.ad_value(853), scratch.ad_value(855)), scratch.ad_value(200))), AdValue::neg(AdValue::sub(AdValue::add(scratch.ad_value(853), scratch.ad_value(855)), scratch.ad_value(200)))), scratch.ad_value(201))))));
        }

        if (scratch.values[2210] != 0.0) {
            scratch.store_ad(2094, &AdValue::mul(scratch.ad_value(2089), AdValue::sqrt(AdValue::offset(scratch.ad_value(2093), 1.0))));
        }

        if (!(scratch.values[2210] != 0.0)) {
            scratch.values[2094] = scratch.values[2089];
            scratch.node_derivatives[2094] = scratch.node_derivatives[2089];
            scratch.branch_derivatives[2094] = scratch.branch_derivatives[2089];
        }

        scratch.store_ad(2095, &AdValue::square(scratch.ad_value(2094)));

        scratch.store_ad(2096, &AdValue::div_from_scalar(1.0, scratch.ad_value(2095)));

        scratch.values[2009] = 1.0;

        scratch.values[2211] = if (scratch.values[207] > 0.0) { 1.0 } else { 0.0 };

        if (scratch.values[2211] != 0.0) {
            scratch.store_ad(2004, &AdValue::scale(scratch.ad_value(866), (2.0 * scratch.values[363])));
        }

        if (scratch.values[2211] != 0.0) {
            scratch.store_ad(2077, &AdValue::add(scratch.ad_value(2095), scratch.ad_value(2004)));
        }

        if (scratch.values[2211] != 0.0) {
            scratch.store_ad(2078, &AdValue::scale(AdValue::add(AdValue::add(scratch.ad_value(2077), scratch.ad_value(2004)), AdValue::sqrt(AdValue::offset(AdValue::mul(AdValue::add(scratch.ad_value(2077), scratch.ad_value(2004)), AdValue::add(scratch.ad_value(2077), scratch.ad_value(2004))), 5.0))), 0.5));
        }

        if (scratch.values[2211] != 0.0) {
            scratch.store_ad(2005, &AdValue::scale(AdValue::sub(scratch.ad_value(2077), AdValue::mul(scratch.ad_value(2094), AdValue::sqrt(scratch.ad_value(2078)))), 0.5));
        }

        if (scratch.values[2211] != 0.0) {
            scratch.store_ad(2006, &AdValue::scale(scratch.ad_value(2088), scratch.values[363]));
        }

        if (scratch.values[2211] != 0.0) {
            scratch.store_ad(2007, &AdValue::scale(scratch.ad_value(2091), scratch.values[363]));
        }

        if (scratch.values[2211] != 0.0) {
            scratch.store_ad(2077, &AdValue::offset(AdValue::add(scratch.ad_value(2006), scratch.ad_value(2007)), 2.0));
        }

        if (scratch.values[2211] != 0.0) {
            scratch.store_ad(2008, &AdValue::scale(AdValue::sub(AdValue::add(scratch.ad_value(2005), scratch.ad_value(2077)), AdValue::sqrt(AdValue::offset(AdValue::mul(AdValue::sub(scratch.ad_value(2005), scratch.ad_value(2077)), AdValue::sub(scratch.ad_value(2005), scratch.ad_value(2077))), 5.0))), 0.5));
        }

        if (scratch.values[2211] != 0.0) {
            scratch.store_ad(2078, &AdValue::mul(scratch.ad_value(746), AdValue::sub(scratch.ad_value(2008), AdValue::mul(AdValue::offset(scratch.ad_value(208), 1.0), AdValue::add(AdValue::scale(scratch.ad_value(2006), 0.5), scratch.ad_value(2007))))));
        }

        scratch.values[2212] = if (scratch.values[2078] > (-230.25850929940458)) { 1.0 } else { 0.0 };

        if ((scratch.values[2211] != 0.0) && (scratch.values[2212] != 0.0)) {
            scratch.store_ad(2009, &AdValue::exp(scratch.ad_value(2078)));
        }

        if ((scratch.values[2211] != 0.0) && (!(scratch.values[2212] != 0.0))) {
            scratch.store_ad(2009, &AdValue::div_from_scalar(1e-100, AdValue::offset(AdValue::mul(AdValue::sub_from_scalar((-230.25850929940458), scratch.ad_value(2078)), AdValue::offset(AdValue::scale(AdValue::mul(AdValue::sub_from_scalar((-230.25850929940458), scratch.ad_value(2078)), AdValue::offset(AdValue::scale(AdValue::sub_from_scalar((-230.25850929940458), scratch.ad_value(2078)), 0.3333333333333333), 1.0)), 0.5), 1.0)), 1.0)));
        }

        scratch.store_ad(2010, &AdValue::offset(AdValue::mul(scratch.ad_value(745), scratch.ad_value(2009)), 1.0));

        scratch.store_ad(2011, &AdValue::mul(AdValue::scale(scratch.ad_value(2010), scratch.values[759]), AdValue::offset(scratch.ad_value(2003), 1.0)));

        scratch.store_ad(2012, &AdValue::div_from_scalar(1.0, scratch.ad_value(2011)));

        scratch.store_ad(2092, &AdValue::mul(scratch.ad_value(866), scratch.ad_value(2012)));

        scratch.store_ad(2097, &AdValue::offset(AdValue::scale(scratch.ad_value(2094), 0.7071067811865475), 1.0));

        scratch.store_ad(2098, &AdValue::div_from_scalar(1.0, scratch.ad_value(2097)));

        scratch.store_ad(2099, &AdValue::mul(scratch.ad_value(2090), scratch.ad_value(2012)));

        scratch.store_ad(2100, &AdValue::add(AdValue::mul(scratch.ad_value(2088), scratch.ad_value(2012)), scratch.ad_value(2099)));

        scratch.values[2213] = if (scratch.values[2100] < 460.51701859880916) { 1.0 } else { 0.0 };

        if (scratch.values[2213] != 0.0) {
            scratch.store_ad(2101, &AdValue::exp(AdValue::neg(scratch.ad_value(2100))));
        }

        if (!(scratch.values[2213] != 0.0)) {
            scratch.store_ad(2101, &AdValue::div_from_scalar(1e-200, AdValue::offset(AdValue::mul(AdValue::offset(scratch.ad_value(2100), (-460.51701859880916)), AdValue::offset(AdValue::scale(AdValue::mul(AdValue::offset(scratch.ad_value(2100), (-460.51701859880916)), AdValue::offset(AdValue::scale(AdValue::offset(scratch.ad_value(2100), (-460.51701859880916)), 0.3333333333333333), 1.0)), 0.5), 1.0)), 1.0)));
        }

        scratch.store_ad(2102, &AdValue::scale(scratch.ad_value(2097), 1e-5));

        scratch.values[2214] = if (((scratch.values[2092]) as f64).abs() <= scratch.values[2102]) { 1.0 } else { 0.0 };

        if (scratch.values[2214] != 0.0) {
            scratch.store_ad(2178, &AdValue::scale(AdValue::square(scratch.ad_value(2098)), (0.16666666666666666 * 0.7071067811865475)));
        }

        if (scratch.values[2214] != 0.0) {
            scratch.store_ad(2103, &AdValue::mul(AdValue::mul(scratch.ad_value(2092), scratch.ad_value(2098)), AdValue::offset(AdValue::mul(AdValue::mul(AdValue::mul(scratch.ad_value(2092), AdValue::sub_from_scalar(1.0, scratch.ad_value(2101))), scratch.ad_value(2094)), scratch.ad_value(2178)), 1.0)));
        }

        scratch.values[2215] = if (scratch.values[2092] < (-scratch.values[2102])) { 1.0 } else { 0.0 };

        if ((!(scratch.values[2214] != 0.0)) && (scratch.values[2215] != 0.0)) {
            scratch.store_ad(2180, &AdValue::neg(scratch.ad_value(2092)));
        }

        if ((!(scratch.values[2214] != 0.0)) && (scratch.values[2215] != 0.0)) {
            scratch.store_ad(2181, &AdValue::scale(AdValue::mul(scratch.ad_value(2180), scratch.ad_value(2098)), 1.25));
        }

        if ((!(scratch.values[2214] != 0.0)) && (scratch.values[2215] != 0.0)) {
            scratch.store_ad(2182, &AdValue::scale(AdValue::sub(AdValue::offset(scratch.ad_value(2181), 10.0), AdValue::sqrt(AdValue::offset(AdValue::mul(AdValue::offset(scratch.ad_value(2181), (-6.0)), AdValue::offset(scratch.ad_value(2181), (-6.0))), 64.0))), 0.5));
        }

        if ((!(scratch.values[2214] != 0.0)) && (scratch.values[2215] != 0.0)) {
            scratch.store_ad(2177, &AdValue::sub(scratch.ad_value(2180), scratch.ad_value(2182)));
        }

        if ((!(scratch.values[2214] != 0.0)) && (scratch.values[2215] != 0.0)) {
            scratch.store_ad(2183, &AdValue::add(AdValue::square(scratch.ad_value(2177)), AdValue::mul(scratch.ad_value(2095), AdValue::offset(scratch.ad_value(2182), 1.0))));
        }

        if ((!(scratch.values[2214] != 0.0)) && (scratch.values[2215] != 0.0)) {
            scratch.store_ad(2184, &AdValue::sub(AdValue::scale(scratch.ad_value(2177), 2.0), scratch.ad_value(2095)));
        }

        if ((!(scratch.values[2214] != 0.0)) && (scratch.values[2215] != 0.0)) {
            scratch.store_ad(2185, &AdValue::sub(AdValue::ln(AdValue::mul(scratch.ad_value(2183), scratch.ad_value(2096))), scratch.ad_value(2182)));
        }

        if ((!(scratch.values[2214] != 0.0)) && (scratch.values[2215] != 0.0)) {
            scratch.store_ad(966, &AdValue::add(scratch.ad_value(2183), scratch.ad_value(2184)));
        }

        if ((!(scratch.values[2214] != 0.0)) && (scratch.values[2215] != 0.0)) {
            scratch.store_ad(965, &AdValue::add(AdValue::square(scratch.ad_value(966)), AdValue::mul(scratch.ad_value(2185), AdValue::sub(AdValue::scale(AdValue::square(scratch.ad_value(2184)), 0.5), scratch.ad_value(2183)))));
        }

        if ((!(scratch.values[2214] != 0.0)) && (scratch.values[2215] != 0.0)) {
            scratch.store_ad(2186, &AdValue::add(scratch.ad_value(2182), AdValue::div(AdValue::mul(AdValue::mul(scratch.ad_value(2183), scratch.ad_value(966)), scratch.ad_value(2185)), AdValue::add(scratch.ad_value(965), AdValue::mul(AdValue::mul(AdValue::mul(AdValue::mul(AdValue::div(scratch.ad_value(966), scratch.ad_value(965)), scratch.ad_value(2185)), scratch.ad_value(2185)), scratch.ad_value(2184)), AdValue::sub(AdValue::scale(AdValue::square(scratch.ad_value(2184)), 0.3333333333333333), scratch.ad_value(2183)))))));
        }

        scratch.values[2216] = if (scratch.values[2186] < 230.25850929940458) { 1.0 } else { 0.0 };

        if (((!(scratch.values[2214] != 0.0)) && (scratch.values[2215] != 0.0)) && (scratch.values[2216] != 0.0)) {
            scratch.store_ad(2187, &AdValue::exp(scratch.ad_value(2186)));
        }

        if (((!(scratch.values[2214] != 0.0)) && (scratch.values[2215] != 0.0)) && (!(scratch.values[2216] != 0.0))) {
            scratch.store_ad(2187, &AdValue::scale(AdValue::offset(AdValue::mul(AdValue::offset(scratch.ad_value(2186), (-230.25850929940458)), AdValue::offset(AdValue::scale(AdValue::mul(AdValue::offset(scratch.ad_value(2186), (-230.25850929940458)), AdValue::offset(AdValue::scale(AdValue::offset(scratch.ad_value(2186), (-230.25850929940458)), 0.3333333333333333), 1.0)), 0.5), 1.0)), 1.0), 1e100));
        }

        if ((!(scratch.values[2214] != 0.0)) && (scratch.values[2215] != 0.0)) {
            scratch.store_ad(2188, &AdValue::div_from_scalar(1.0, scratch.ad_value(2187)));
        }

        if ((!(scratch.values[2214] != 0.0)) && (scratch.values[2215] != 0.0)) {
            scratch.store_ad(2177, &AdValue::div_from_scalar(1.0, AdValue::offset(AdValue::square(scratch.ad_value(2186)), 2.0)));
        }

        if ((!(scratch.values[2214] != 0.0)) && (scratch.values[2215] != 0.0)) {
            scratch.store_ad(2189, &AdValue::mul(AdValue::square(scratch.ad_value(2186)), scratch.ad_value(2177)));
        }

        if ((!(scratch.values[2214] != 0.0)) && (scratch.values[2215] != 0.0)) {
            scratch.store_ad(2190, &AdValue::scale(AdValue::mul(AdValue::mul(scratch.ad_value(2186), scratch.ad_value(2177)), scratch.ad_value(2177)), 4.0));
        }

        if ((!(scratch.values[2214] != 0.0)) && (scratch.values[2215] != 0.0)) {
            scratch.store_ad(2191, &AdValue::mul(AdValue::mul(AdValue::sub(AdValue::scale(scratch.ad_value(2177), 8.0), AdValue::scale(scratch.ad_value(2189), 12.0)), scratch.ad_value(2177)), scratch.ad_value(2177)));
        }

        if ((!(scratch.values[2214] != 0.0)) && (scratch.values[2215] != 0.0)) {
            scratch.store_ad(2177, &AdValue::sub(scratch.ad_value(2180), scratch.ad_value(2186)));
        }

        if ((!(scratch.values[2214] != 0.0)) && (scratch.values[2215] != 0.0)) {
            scratch.store_ad(2178, &AdValue::mul(scratch.ad_value(2101), scratch.ad_value(2188)));
        }

        if ((!(scratch.values[2214] != 0.0)) && (scratch.values[2215] != 0.0)) {
            scratch.store_ad(2192, &AdValue::add(AdValue::scale(scratch.ad_value(2177), 2.0), AdValue::mul(scratch.ad_value(2095), AdValue::add(AdValue::sub(AdValue::offset(scratch.ad_value(2187), (-1.0)), scratch.ad_value(2178)), AdValue::mul(scratch.ad_value(2101), AdValue::sub_from_scalar(1.0, scratch.ad_value(2190)))))));
        }

        if ((!(scratch.values[2214] != 0.0)) && (scratch.values[2215] != 0.0)) {
            scratch.store_ad(2193, &AdValue::sub(AdValue::square(scratch.ad_value(2177)), AdValue::mul(scratch.ad_value(2095), AdValue::add(AdValue::add(AdValue::offset(AdValue::sub(scratch.ad_value(2187), scratch.ad_value(2186)), (-1.0)), scratch.ad_value(2178)), AdValue::mul(scratch.ad_value(2101), AdValue::sub(AdValue::offset(scratch.ad_value(2186), (-1.0)), scratch.ad_value(2189)))))));
        }

    }

    pub(super) fn stamp_transient_block_31(
        &mut self,
        ctx: &GeneratedEvalContext<'_>,
        stamper: &mut GeneratedStamper<'_>,
        scratch: &mut Scratch,
    ) {
        let _ = stamper;
        if ((!(scratch.values[2214] != 0.0)) && (scratch.values[2215] != 0.0)) {
            scratch.store_ad(2177, &AdValue::sub_from_scalar(2.0, AdValue::mul(scratch.ad_value(2095), AdValue::sub(AdValue::add(scratch.ad_value(2187), scratch.ad_value(2178)), AdValue::mul(scratch.ad_value(2101), scratch.ad_value(2191))))));
        }

        if ((!(scratch.values[2214] != 0.0)) && (scratch.values[2215] != 0.0)) {
            scratch.store_ad(2177, &AdValue::sub(AdValue::square(scratch.ad_value(2192)), AdValue::scale(AdValue::mul(scratch.ad_value(2193), scratch.ad_value(2177)), 2.0)));
        }

        if ((!(scratch.values[2214] != 0.0)) && (scratch.values[2215] != 0.0)) {
            scratch.store_ad(2103, &AdValue::sub(AdValue::neg(scratch.ad_value(2186)), AdValue::scale(AdValue::div(scratch.ad_value(2193), AdValue::add(scratch.ad_value(2192), AdValue::sqrt(scratch.ad_value(2177)))), 2.0)));
        }

        if ((!(scratch.values[2214] != 0.0)) && (!(scratch.values[2215] != 0.0))) {
            scratch.store_ad(2194, &AdValue::div_from_scalar(1.0, AdValue::offset(AdValue::scale(scratch.ad_value(2094), 0.7324648775608221), 1.25)));
        }

        if ((!(scratch.values[2214] != 0.0)) && (!(scratch.values[2215] != 0.0))) {
            scratch.store_ad(2195, &AdValue::mul(AdValue::offset(AdValue::mul(AdValue::scale(scratch.ad_value(2097), 1.25), scratch.ad_value(2194)), (-1.0)), scratch.ad_value(2194)));
        }

        if ((!(scratch.values[2214] != 0.0)) && (!(scratch.values[2215] != 0.0))) {
            scratch.store_ad(2196, &AdValue::mul(AdValue::mul(scratch.ad_value(2092), scratch.ad_value(2098)), AdValue::offset(AdValue::mul(scratch.ad_value(2195), scratch.ad_value(2092)), 1.0)));
        }

        scratch.values[2217] = if ((-scratch.values[2196]) > (-230.25850929940458)) { 1.0 } else { 0.0 };

        if (((!(scratch.values[2214] != 0.0)) && (!(scratch.values[2215] != 0.0))) && (scratch.values[2217] != 0.0)) {
            scratch.store_ad(2177, &AdValue::exp(AdValue::neg(scratch.ad_value(2196))));
        }

        if (((!(scratch.values[2214] != 0.0)) && (!(scratch.values[2215] != 0.0))) && (!(scratch.values[2217] != 0.0))) {
            scratch.store_ad(2177, &AdValue::div_from_scalar(1e-100, AdValue::offset(AdValue::mul(AdValue::sub_from_scalar((-230.25850929940458), AdValue::neg(scratch.ad_value(2196))), AdValue::offset(AdValue::scale(AdValue::mul(AdValue::sub_from_scalar((-230.25850929940458), AdValue::neg(scratch.ad_value(2196))), AdValue::offset(AdValue::scale(AdValue::sub_from_scalar((-230.25850929940458), AdValue::neg(scratch.ad_value(2196))), 0.3333333333333333), 1.0)), 0.5), 1.0)), 1.0)));
        }

        if ((!(scratch.values[2214] != 0.0)) && (!(scratch.values[2215] != 0.0))) {
            scratch.store_ad(2197, &AdValue::sub_from_scalar(1.0, scratch.ad_value(2177)));
        }

        if ((!(scratch.values[2214] != 0.0)) && (!(scratch.values[2215] != 0.0))) {
            scratch.store_ad(2198, &AdValue::sub(AdValue::add(scratch.ad_value(2092), AdValue::scale(scratch.ad_value(2095), 0.5)), AdValue::mul(scratch.ad_value(2094), AdValue::sqrt(AdValue::sub(AdValue::add(scratch.ad_value(2092), AdValue::scale(scratch.ad_value(2095), 0.25)), scratch.ad_value(2197))))));
        }

        if ((!(scratch.values[2214] != 0.0)) && (!(scratch.values[2215] != 0.0))) {
            scratch.store_ad(2199, &AdValue::offset(scratch.ad_value(2100), 3.0));
        }

        if ((!(scratch.values[2214] != 0.0)) && (!(scratch.values[2215] != 0.0))) {
            scratch.store_ad(2182, &AdValue::sub(AdValue::scale(AdValue::sub(AdValue::add(scratch.ad_value(2198), scratch.ad_value(2199)), AdValue::sqrt(AdValue::offset(AdValue::mul(AdValue::sub(scratch.ad_value(2198), scratch.ad_value(2199)), AdValue::sub(scratch.ad_value(2198), scratch.ad_value(2199))), 5.0))), 0.5), AdValue::scale(AdValue::sub(scratch.ad_value(2199), AdValue::sqrt(AdValue::offset(AdValue::square(scratch.ad_value(2199)), 5.0))), 0.5)));
        }

        if ((!(scratch.values[2214] != 0.0)) && (!(scratch.values[2215] != 0.0))) {
            scratch.store_ad(2177, &AdValue::sub(scratch.ad_value(2092), scratch.ad_value(2182)));
        }

        if ((!(scratch.values[2214] != 0.0)) && (!(scratch.values[2215] != 0.0))) {
            scratch.store_ad(2178, &AdValue::exp(AdValue::neg(scratch.ad_value(2182))));
        }

        if ((!(scratch.values[2214] != 0.0)) && (!(scratch.values[2215] != 0.0))) {
            scratch.store_ad(2179, &AdValue::div_from_scalar(1.0, AdValue::offset(AdValue::square(scratch.ad_value(2182)), 2.0)));
        }

        if ((!(scratch.values[2214] != 0.0)) && (!(scratch.values[2215] != 0.0))) {
            scratch.store_ad(2189, &AdValue::mul(AdValue::square(scratch.ad_value(2182)), scratch.ad_value(2179)));
        }

        if ((!(scratch.values[2214] != 0.0)) && (!(scratch.values[2215] != 0.0))) {
            scratch.store_ad(2190, &AdValue::scale(AdValue::mul(AdValue::mul(scratch.ad_value(2182), scratch.ad_value(2179)), scratch.ad_value(2179)), 4.0));
        }

        if ((!(scratch.values[2214] != 0.0)) && (!(scratch.values[2215] != 0.0))) {
            scratch.store_ad(2191, &AdValue::mul(AdValue::mul(AdValue::sub(AdValue::scale(scratch.ad_value(2179), 8.0), AdValue::scale(scratch.ad_value(2189), 12.0)), scratch.ad_value(2179)), scratch.ad_value(2179)));
        }

        if ((!(scratch.values[2214] != 0.0)) && (!(scratch.values[2215] != 0.0))) {
            scratch.store_ad(2183, &AdValue::max_from_scalar(1e-40, AdValue::sub(AdValue::square(scratch.ad_value(2177)), AdValue::mul(scratch.ad_value(2095), AdValue::sub(AdValue::offset(AdValue::add(scratch.ad_value(2178), scratch.ad_value(2182)), (-1.0)), AdValue::mul(scratch.ad_value(2101), AdValue::add(AdValue::offset(scratch.ad_value(2182), 1.0), scratch.ad_value(2189))))))));
        }

        if ((!(scratch.values[2214] != 0.0)) && (!(scratch.values[2215] != 0.0))) {
            scratch.store_ad(2200, &AdValue::sub_from_scalar(1.0, AdValue::scale(AdValue::mul(scratch.ad_value(2095), AdValue::sub(scratch.ad_value(2178), AdValue::mul(scratch.ad_value(2101), scratch.ad_value(2191)))), 0.5)));
        }

        if ((!(scratch.values[2214] != 0.0)) && (!(scratch.values[2215] != 0.0))) {
            scratch.store_ad(2184, &AdValue::add(AdValue::scale(scratch.ad_value(2177), 2.0), AdValue::mul(scratch.ad_value(2095), AdValue::sub(AdValue::sub_from_scalar(1.0, scratch.ad_value(2178)), AdValue::mul(scratch.ad_value(2101), AdValue::offset(scratch.ad_value(2190), 1.0))))));
        }

        if ((!(scratch.values[2214] != 0.0)) && (!(scratch.values[2215] != 0.0))) {
            scratch.store_ad(2185, &AdValue::add(AdValue::sub(scratch.ad_value(2100), scratch.ad_value(2182)), AdValue::ln(AdValue::div(scratch.ad_value(2183), scratch.ad_value(2095)))));
        }

        if ((!(scratch.values[2214] != 0.0)) && (!(scratch.values[2215] != 0.0))) {
            scratch.store_ad(966, &AdValue::add(scratch.ad_value(2183), scratch.ad_value(2184)));
        }

        if ((!(scratch.values[2214] != 0.0)) && (!(scratch.values[2215] != 0.0))) {
            scratch.store_ad(965, &AdValue::add(AdValue::square(scratch.ad_value(966)), AdValue::mul(scratch.ad_value(2185), AdValue::sub(AdValue::scale(AdValue::square(scratch.ad_value(2184)), 0.5), AdValue::mul(scratch.ad_value(2183), scratch.ad_value(2200))))));
        }

        if ((!(scratch.values[2214] != 0.0)) && (!(scratch.values[2215] != 0.0))) {
            let assign41880_ad_e54176: AdValue = AdValue::add(scratch.ad_value(2182), AdValue::div(AdValue::mul(AdValue::mul(scratch.ad_value(2183), scratch.ad_value(966)), scratch.ad_value(2185)), AdValue::add(scratch.ad_value(965), AdValue::mul(AdValue::mul(AdValue::mul(AdValue::mul(AdValue::div(scratch.ad_value(966), scratch.ad_value(965)), scratch.ad_value(2185)), scratch.ad_value(2185)), scratch.ad_value(2184)), AdValue::sub(AdValue::scale(AdValue::square(scratch.ad_value(2184)), 0.3333333333333333), AdValue::mul(scratch.ad_value(2183), scratch.ad_value(2200)))))));
            scratch.store_ad(2201, &assign41880_ad_e54176);
        }

        scratch.values[2218] = if (scratch.values[2201] < 230.25850929940458) { 1.0 } else { 0.0 };

        if (((!(scratch.values[2214] != 0.0)) && (!(scratch.values[2215] != 0.0))) && (scratch.values[2218] != 0.0)) {
            scratch.store_ad(2187, &AdValue::exp(scratch.ad_value(2201)));
        }

        if (((!(scratch.values[2214] != 0.0)) && (!(scratch.values[2215] != 0.0))) && (scratch.values[2218] != 0.0)) {
            scratch.store_ad(2188, &AdValue::div_from_scalar(1.0, scratch.ad_value(2187)));
        }

        if (((!(scratch.values[2214] != 0.0)) && (!(scratch.values[2215] != 0.0))) && (scratch.values[2218] != 0.0)) {
            scratch.store_ad(2187, &AdValue::mul(scratch.ad_value(2101), scratch.ad_value(2187)));
        }

        scratch.values[2219] = if (scratch.values[2201] > (scratch.values[2100] - 230.25850929940458)) { 1.0 } else { 0.0 };

        if ((((!(scratch.values[2214] != 0.0)) && (!(scratch.values[2215] != 0.0))) && (!(scratch.values[2218] != 0.0))) && (scratch.values[2219] != 0.0)) {
            scratch.store_ad(2187, &AdValue::exp(AdValue::sub(scratch.ad_value(2201), scratch.ad_value(2100))));
        }

        if ((((!(scratch.values[2214] != 0.0)) && (!(scratch.values[2215] != 0.0))) && (!(scratch.values[2218] != 0.0))) && (scratch.values[2219] != 0.0)) {
            scratch.store_ad(2188, &AdValue::div(scratch.ad_value(2101), scratch.ad_value(2187)));
        }

        if ((((!(scratch.values[2214] != 0.0)) && (!(scratch.values[2215] != 0.0))) && (!(scratch.values[2218] != 0.0))) && (!(scratch.values[2219] != 0.0))) {
            scratch.store_ad(2187, &AdValue::div_from_scalar(1e-100, AdValue::offset(AdValue::mul(AdValue::offset(AdValue::sub(scratch.ad_value(2100), scratch.ad_value(2201)), (-230.25850929940458)), AdValue::offset(AdValue::scale(AdValue::mul(AdValue::offset(AdValue::sub(scratch.ad_value(2100), scratch.ad_value(2201)), (-230.25850929940458)), AdValue::offset(AdValue::scale(AdValue::offset(AdValue::sub(scratch.ad_value(2100), scratch.ad_value(2201)), (-230.25850929940458)), 0.3333333333333333), 1.0)), 0.5), 1.0)), 1.0)));
        }

        if ((((!(scratch.values[2214] != 0.0)) && (!(scratch.values[2215] != 0.0))) && (!(scratch.values[2218] != 0.0))) && (!(scratch.values[2219] != 0.0))) {
            scratch.store_ad(2188, &AdValue::div_from_scalar(1e-100, AdValue::offset(AdValue::mul(AdValue::offset(scratch.ad_value(2201), (-230.25850929940458)), AdValue::offset(AdValue::scale(AdValue::mul(AdValue::offset(scratch.ad_value(2201), (-230.25850929940458)), AdValue::offset(AdValue::scale(AdValue::offset(scratch.ad_value(2201), (-230.25850929940458)), 0.3333333333333333), 1.0)), 0.5), 1.0)), 1.0)));
        }

        if ((!(scratch.values[2214] != 0.0)) && (!(scratch.values[2215] != 0.0))) {
            scratch.store_ad(2177, &AdValue::div_from_scalar(1.0, AdValue::offset(AdValue::square(scratch.ad_value(2201)), 2.0)));
        }

        if ((!(scratch.values[2214] != 0.0)) && (!(scratch.values[2215] != 0.0))) {
            scratch.store_ad(2189, &AdValue::mul(AdValue::square(scratch.ad_value(2201)), scratch.ad_value(2177)));
        }

        if ((!(scratch.values[2214] != 0.0)) && (!(scratch.values[2215] != 0.0))) {
            scratch.store_ad(2190, &AdValue::scale(AdValue::mul(AdValue::mul(scratch.ad_value(2201), scratch.ad_value(2177)), scratch.ad_value(2177)), 4.0));
        }

        if ((!(scratch.values[2214] != 0.0)) && (!(scratch.values[2215] != 0.0))) {
            scratch.store_ad(2191, &AdValue::mul(AdValue::mul(AdValue::sub(AdValue::scale(scratch.ad_value(2177), 8.0), AdValue::scale(scratch.ad_value(2189), 12.0)), scratch.ad_value(2177)), scratch.ad_value(2177)));
        }

        if ((!(scratch.values[2214] != 0.0)) && (!(scratch.values[2215] != 0.0))) {
            scratch.store_ad(2177, &AdValue::sub(scratch.ad_value(2092), scratch.ad_value(2201)));
        }

        if ((!(scratch.values[2214] != 0.0)) && (!(scratch.values[2215] != 0.0))) {
            scratch.store_ad(2192, &AdValue::add(AdValue::scale(scratch.ad_value(2177), 2.0), AdValue::mul(scratch.ad_value(2095), AdValue::sub(AdValue::add(AdValue::sub_from_scalar(1.0, scratch.ad_value(2188)), scratch.ad_value(2187)), AdValue::mul(scratch.ad_value(2101), AdValue::offset(scratch.ad_value(2190), 1.0))))));
        }

        if ((!(scratch.values[2214] != 0.0)) && (!(scratch.values[2215] != 0.0))) {
            scratch.store_ad(2193, &AdValue::sub(AdValue::square(scratch.ad_value(2177)), AdValue::mul(scratch.ad_value(2095), AdValue::sub(AdValue::add(AdValue::offset(AdValue::add(scratch.ad_value(2188), scratch.ad_value(2201)), (-1.0)), scratch.ad_value(2187)), AdValue::mul(scratch.ad_value(2101), AdValue::add(AdValue::offset(scratch.ad_value(2201), 1.0), scratch.ad_value(2189)))))));
        }

        if ((!(scratch.values[2214] != 0.0)) && (!(scratch.values[2215] != 0.0))) {
            scratch.store_ad(2177, &AdValue::sub_from_scalar(2.0, AdValue::mul(scratch.ad_value(2095), AdValue::sub(AdValue::add(scratch.ad_value(2188), scratch.ad_value(2187)), AdValue::mul(scratch.ad_value(2101), scratch.ad_value(2191))))));
        }

        if ((!(scratch.values[2214] != 0.0)) && (!(scratch.values[2215] != 0.0))) {
            scratch.store_ad(2177, &AdValue::sub(AdValue::square(scratch.ad_value(2192)), AdValue::scale(AdValue::mul(scratch.ad_value(2193), scratch.ad_value(2177)), 2.0)));
        }

        if ((!(scratch.values[2214] != 0.0)) && (!(scratch.values[2215] != 0.0))) {
            scratch.store_ad(2103, &AdValue::add(scratch.ad_value(2201), AdValue::scale(AdValue::div(scratch.ad_value(2193), AdValue::add(scratch.ad_value(2192), AdValue::sqrt(scratch.ad_value(2177)))), 2.0)));
        }

        scratch.values[2137] = scratch.values[2103];
        scratch.node_derivatives[2137] = scratch.node_derivatives[2103];
        scratch.branch_derivatives[2137] = scratch.branch_derivatives[2103];

        scratch.values[2145] = scratch.values[2103];
        scratch.node_derivatives[2145] = scratch.node_derivatives[2103];
        scratch.branch_derivatives[2145] = scratch.branch_derivatives[2103];

        scratch.values[2138] = 0.0;

        scratch.store_ad(869, &AdValue::scale(scratch.ad_value(2011), 3.912023005));

        scratch.values[2220] = if (scratch.values[2092] <= 0.0) { 1.0 } else { 0.0 };

        if (scratch.values[2220] != 0.0) {
            scratch.values[2113] = 0.0;
            scratch.node_derivatives[2113] = [0.0; Instance::NODE_COUNT];
            scratch.branch_derivatives[2113] = [0.0; Instance::BRANCH_COUNT];
        }

        if (scratch.values[2220] != 0.0) {
            scratch.store_ad(2150, &AdValue::sub(scratch.ad_value(2092), scratch.ad_value(2103)));
        }

        if (scratch.values[2220] != 0.0) {
            scratch.store_ad(2174, &AdValue::mul(scratch.ad_value(2150), scratch.ad_value(2011)));
        }

        if (scratch.values[2220] != 0.0) {
            scratch.values[2166] = scratch.values[2174];
            scratch.node_derivatives[2166] = scratch.node_derivatives[2174];
            scratch.branch_derivatives[2166] = scratch.branch_derivatives[2174];
        }

        if (scratch.values[2220] != 0.0) {
            scratch.values[870] = scratch.values[869];
            scratch.node_derivatives[870] = scratch.node_derivatives[869];
            scratch.branch_derivatives[870] = scratch.branch_derivatives[869];
        }

        if (scratch.values[2220] != 0.0) {
            scratch.values[2132] = scratch.values[854];
            scratch.node_derivatives[2132] = scratch.node_derivatives[854];
            scratch.branch_derivatives[2132] = scratch.branch_derivatives[854];
        }

        if (!(scratch.values[2220] != 0.0)) {
            scratch.values[2104] = 0.0;
            scratch.node_derivatives[2104] = [0.0; Instance::NODE_COUNT];
            scratch.branch_derivatives[2104] = [0.0; Instance::BRANCH_COUNT];
        }

        if (!(scratch.values[2220] != 0.0)) {
            scratch.store_ad(2076, &AdValue::div_from_scalar(1.0, AdValue::offset(AdValue::square(scratch.ad_value(2103)), 2.0)));
        }

        if (!(scratch.values[2220] != 0.0)) {
            scratch.store_ad(2105, &AdValue::mul(AdValue::square(scratch.ad_value(2103)), scratch.ad_value(2076)));
        }

        if (!(scratch.values[2220] != 0.0)) {
            scratch.store_ad(2106, &AdValue::scale(AdValue::mul(AdValue::mul(scratch.ad_value(2103), scratch.ad_value(2076)), scratch.ad_value(2076)), 4.0));
        }

        if (!(scratch.values[2220] != 0.0)) {
            scratch.store_ad(2107, &AdValue::mul(AdValue::mul(AdValue::sub(AdValue::scale(scratch.ad_value(2076), 8.0), AdValue::scale(scratch.ad_value(2105), 12.0)), scratch.ad_value(2076)), scratch.ad_value(2076)));
        }

        scratch.values[2221] = if (scratch.values[2103] < 230.25850929940458) { 1.0 } else { 0.0 };

        if ((!(scratch.values[2220] != 0.0)) && (scratch.values[2221] != 0.0)) {
            scratch.store_ad(2104, &AdValue::exp(scratch.ad_value(2103)));
        }

        if ((!(scratch.values[2220] != 0.0)) && (scratch.values[2221] != 0.0)) {
            scratch.store_ad(2108, &AdValue::div_from_scalar(1.0, scratch.ad_value(2104)));
        }

        if ((!(scratch.values[2220] != 0.0)) && (scratch.values[2221] != 0.0)) {
            scratch.store_ad(2104, &AdValue::mul(scratch.ad_value(2101), scratch.ad_value(2104)));
        }

        scratch.values[2222] = if (scratch.values[2103] > (scratch.values[2100] - 230.25850929940458)) { 1.0 } else { 0.0 };

        if (((!(scratch.values[2220] != 0.0)) && (!(scratch.values[2221] != 0.0))) && (scratch.values[2222] != 0.0)) {
            scratch.store_ad(2104, &AdValue::exp(AdValue::sub(scratch.ad_value(2103), scratch.ad_value(2100))));
        }

        if (((!(scratch.values[2220] != 0.0)) && (!(scratch.values[2221] != 0.0))) && (scratch.values[2222] != 0.0)) {
            scratch.store_ad(2108, &AdValue::div(scratch.ad_value(2101), scratch.ad_value(2104)));
        }

        if (((!(scratch.values[2220] != 0.0)) && (!(scratch.values[2221] != 0.0))) && (!(scratch.values[2222] != 0.0))) {
            scratch.store_ad(2104, &AdValue::div_from_scalar(1e-100, AdValue::offset(AdValue::mul(AdValue::offset(AdValue::sub(scratch.ad_value(2100), scratch.ad_value(2103)), (-230.25850929940458)), AdValue::offset(AdValue::scale(AdValue::mul(AdValue::offset(AdValue::sub(scratch.ad_value(2100), scratch.ad_value(2103)), (-230.25850929940458)), AdValue::offset(AdValue::scale(AdValue::offset(AdValue::sub(scratch.ad_value(2100), scratch.ad_value(2103)), (-230.25850929940458)), 0.3333333333333333), 1.0)), 0.5), 1.0)), 1.0)));
        }

        if (((!(scratch.values[2220] != 0.0)) && (!(scratch.values[2221] != 0.0))) && (!(scratch.values[2222] != 0.0))) {
            scratch.store_ad(2108, &AdValue::div_from_scalar(1e-100, AdValue::offset(AdValue::mul(AdValue::offset(scratch.ad_value(2103), (-230.25850929940458)), AdValue::offset(AdValue::scale(AdValue::mul(AdValue::offset(scratch.ad_value(2103), (-230.25850929940458)), AdValue::offset(AdValue::scale(AdValue::offset(scratch.ad_value(2103), (-230.25850929940458)), 0.3333333333333333), 1.0)), 0.5), 1.0)), 1.0)));
        }

        if (!(scratch.values[2220] != 0.0)) {
            scratch.store_ad(2109, &AdValue::sub(scratch.ad_value(2104), AdValue::mul(scratch.ad_value(2101), AdValue::add(AdValue::offset(scratch.ad_value(2103), 1.0), scratch.ad_value(2105)))));
        }

        scratch.values[2223] = if (scratch.values[2103] < 1e-5) { 1.0 } else { 0.0 };

        if ((!(scratch.values[2220] != 0.0)) && (scratch.values[2223] != 0.0)) {
            scratch.store_ad(2110, &AdValue::scale(AdValue::mul(AdValue::square(scratch.ad_value(2103)), AdValue::sub_from_scalar(1.0, AdValue::scale(AdValue::mul(scratch.ad_value(2103), AdValue::sub_from_scalar(1.0, AdValue::scale(scratch.ad_value(2103), 0.25))), 0.3333333333333333))), 0.5));
        }

        if ((!(scratch.values[2220] != 0.0)) && (scratch.values[2223] != 0.0)) {
            scratch.store_ad(2109, &AdValue::scale(AdValue::mul(AdValue::mul(AdValue::mul(AdValue::mul(scratch.ad_value(2101), scratch.ad_value(2103)), scratch.ad_value(2103)), scratch.ad_value(2103)), AdValue::offset(AdValue::scale(scratch.ad_value(2103), 1.75), 1.0)), 0.16666666666666666));
        }

        if ((!(scratch.values[2220] != 0.0)) && (scratch.values[2223] != 0.0)) {
            scratch.store_ad(2076, &AdValue::sqrt(AdValue::sub_from_scalar(1.0, AdValue::scale(AdValue::mul(scratch.ad_value(2103), AdValue::sub_from_scalar(1.0, AdValue::scale(scratch.ad_value(2103), 0.25))), 0.3333333333333333))));
        }

        if ((!(scratch.values[2220] != 0.0)) && (scratch.values[2223] != 0.0)) {
            scratch.store_ad(2152, &AdValue::scale(AdValue::mul(scratch.ad_value(2103), scratch.ad_value(2076)), 0.7071067811865475));
        }

        if ((!(scratch.values[2220] != 0.0)) && (scratch.values[2223] != 0.0)) {
            scratch.store_ad(2153, &AdValue::offset(AdValue::div(AdValue::mul(AdValue::scale(scratch.ad_value(2094), 0.7071067811865475), AdValue::add(AdValue::sub_from_scalar(1.0, AdValue::scale(scratch.ad_value(2103), 0.5)), AdValue::scale(AdValue::square(scratch.ad_value(2103)), 0.16666666666666666))), scratch.ad_value(2076)), 1.0));
        }

        if ((!(scratch.values[2220] != 0.0)) && (!(scratch.values[2223] != 0.0))) {
            scratch.store_ad(2110, &AdValue::add(AdValue::offset(scratch.ad_value(2103), (-1.0)), scratch.ad_value(2108)));
        }

        if ((!(scratch.values[2220] != 0.0)) && (!(scratch.values[2223] != 0.0))) {
            scratch.store_ad(2152, &AdValue::sqrt(scratch.ad_value(2110)));
        }

        if ((!(scratch.values[2220] != 0.0)) && (!(scratch.values[2223] != 0.0))) {
            scratch.store_ad(2153, &AdValue::offset(AdValue::scale(AdValue::div(AdValue::mul(scratch.ad_value(2094), AdValue::sub_from_scalar(1.0, scratch.ad_value(2108))), scratch.ad_value(2152)), 0.5), 1.0));
        }

        if (!(scratch.values[2220] != 0.0)) {
            scratch.values[2146] = scratch.values[2108];
            scratch.node_derivatives[2146] = scratch.node_derivatives[2108];
            scratch.branch_derivatives[2146] = scratch.branch_derivatives[2108];
        }

        if (!(scratch.values[2220] != 0.0)) {
            scratch.values[2143] = scratch.values[2146];
            scratch.node_derivatives[2143] = scratch.node_derivatives[2146];
            scratch.branch_derivatives[2143] = scratch.branch_derivatives[2146];
        }

        if (!(scratch.values[2220] != 0.0)) {
            scratch.values[2148] = scratch.values[2109];
            scratch.node_derivatives[2148] = scratch.node_derivatives[2109];
            scratch.branch_derivatives[2148] = scratch.branch_derivatives[2109];
        }

        if (!(scratch.values[2220] != 0.0)) {
            scratch.values[2144] = scratch.values[2148];
            scratch.node_derivatives[2144] = scratch.node_derivatives[2148];
            scratch.branch_derivatives[2144] = scratch.branch_derivatives[2148];
        }

        if (!(scratch.values[2220] != 0.0)) {
            scratch.store_ad(2111, &AdValue::div(AdValue::offset(AdValue::mul(AdValue::scale(scratch.ad_value(752), 0.2), scratch.ad_value(2091)), 1.0), AdValue::offset(AdValue::mul(scratch.ad_value(752), scratch.ad_value(2091)), 1.0)));
        }

        scratch.values[2224] = if (scratch.values[2109] > 1e-100) { 1.0 } else { 0.0 };

        if ((!(scratch.values[2220] != 0.0)) && (scratch.values[2224] != 0.0)) {
            scratch.store_ad(2112, &AdValue::mul(scratch.ad_value(2094), AdValue::sqrt(AdValue::add(scratch.ad_value(2110), scratch.ad_value(2109)))));
        }

        if ((!(scratch.values[2220] != 0.0)) && (scratch.values[2224] != 0.0)) {
            scratch.store_ad(2113, &AdValue::div(AdValue::mul(AdValue::mul(scratch.ad_value(2095), scratch.ad_value(2109)), scratch.ad_value(2011)), AdValue::add(scratch.ad_value(2112), AdValue::mul(scratch.ad_value(2094), scratch.ad_value(2152)))));
        }

        if ((!(scratch.values[2220] != 0.0)) && (scratch.values[2224] != 0.0)) {
            scratch.store_ad(2114, &AdValue::mul(AdValue::mul(scratch.ad_value(2152), scratch.ad_value(2094)), scratch.ad_value(2011)));
        }

        scratch.values[2225] = if (scratch.values[234] < 0.0) { 1.0 } else { 0.0 };

        if (((!(scratch.values[2220] != 0.0)) && (scratch.values[2224] != 0.0)) && (scratch.values[2225] != 0.0)) {
            scratch.store_ad(2115, &AdValue::div_from_scalar(1.0, AdValue::sub_from_scalar(1.0, AdValue::mul(scratch.ad_value(234), scratch.ad_value(2091)))));
        }

        if (((!(scratch.values[2220] != 0.0)) && (scratch.values[2224] != 0.0)) && (!(scratch.values[2225] != 0.0))) {
            scratch.store_ad(2115, &AdValue::offset(AdValue::mul(scratch.ad_value(234), scratch.ad_value(2091)), 1.0));
        }

        scratch.values[2226] = if (scratch.values[235] < 0.0) { 1.0 } else { 0.0 };

        if (((!(scratch.values[2220] != 0.0)) && (scratch.values[2224] != 0.0)) && (scratch.values[2226] != 0.0)) {
            scratch.store_ad(2076, &AdValue::sub_from_scalar(1.0, AdValue::mul(scratch.ad_value(235), scratch.ad_value(2113))));
        }

        if (((!(scratch.values[2220] != 0.0)) && (scratch.values[2224] != 0.0)) && (!(scratch.values[2226] != 0.0))) {
            scratch.store_ad(2076, &AdValue::div_from_scalar(1.0, AdValue::offset(AdValue::mul(scratch.ad_value(235), scratch.ad_value(2113)), 1.0)));
        }

        if ((!(scratch.values[2220] != 0.0)) && (scratch.values[2224] != 0.0)) {
            scratch.store_ad(2116, &AdValue::mul(scratch.ad_value(800), AdValue::mul(AdValue::mul(scratch.ad_value(2115), scratch.ad_value(2076)), scratch.ad_value(2113))));
        }

        if ((!(scratch.values[2220] != 0.0)) && (scratch.values[2224] != 0.0)) {
            scratch.store_ad(2117, &AdValue::mul(scratch.ad_value(817), AdValue::add(scratch.ad_value(2114), AdValue::mul(scratch.ad_value(818), scratch.ad_value(2113)))));
        }

        if ((!(scratch.values[2220] != 0.0)) && (scratch.values[2224] != 0.0)) {
            scratch.store_ad(2077, &AdValue::ln(AdValue::div(scratch.ad_value(2110), AdValue::offset(AdValue::add(scratch.ad_value(2110), scratch.ad_value(2109)), 1e-14))));
        }

        if ((!(scratch.values[2220] != 0.0)) && (scratch.values[2224] != 0.0)) {
            scratch.store_ad(2118, &AdValue::add(AdValue::pow(AdValue::mul(scratch.ad_value(2117), scratch.ad_value(748)), scratch.ad_value(749)), AdValue::mul(scratch.ad_value(750), AdValue::exp(AdValue::mul(AdValue::scale(scratch.ad_value(751), 0.5), scratch.ad_value(2077))))));
        }

        if ((!(scratch.values[2220] != 0.0)) && (scratch.values[2224] != 0.0)) {
            scratch.store_ad(2119, &AdValue::mul(AdValue::add(AdValue::offset(scratch.ad_value(2118), 1.0), scratch.ad_value(2116)), scratch.ad_value(2111)));
        }

        scratch.values[2227] = if (scratch.values[238] < 0.0) { 1.0 } else { 0.0 };

        if (((!(scratch.values[2220] != 0.0)) && (scratch.values[2224] != 0.0)) && (scratch.values[2227] != 0.0)) {
            scratch.store_ad(2120, &AdValue::div_from_scalar(1.0, AdValue::sub_from_scalar(1.0, AdValue::mul(scratch.ad_value(238), scratch.ad_value(2091)))));
        }

        if (((!(scratch.values[2220] != 0.0)) && (scratch.values[2224] != 0.0)) && (!(scratch.values[2227] != 0.0))) {
            scratch.store_ad(2120, &AdValue::offset(AdValue::mul(scratch.ad_value(238), scratch.ad_value(2091)), 1.0));
        }

        if ((!(scratch.values[2220] != 0.0)) && (scratch.values[2224] != 0.0)) {
            scratch.store_ad(2078, &AdValue::mul(scratch.ad_value(2113), scratch.ad_value(2120)));
        }

        if ((!(scratch.values[2220] != 0.0)) && (scratch.values[2224] != 0.0)) {
            scratch.store_ad(2121, &AdValue::scale(AdValue::div(scratch.ad_value(2078), AdValue::offset(scratch.ad_value(2078), 100.0)), 100.0));
        }

        scratch.values[2228] = if (scratch.values[239] < 0.0) { 1.0 } else { 0.0 };

        if (((!(scratch.values[2220] != 0.0)) && (scratch.values[2224] != 0.0)) && (scratch.values[2228] != 0.0)) {
            scratch.store_ad(2076, &AdValue::div_from_scalar(1.0, AdValue::sub_from_scalar(1.0, AdValue::mul(scratch.ad_value(239), scratch.ad_value(2121)))));
        }

        if (((!(scratch.values[2220] != 0.0)) && (scratch.values[2224] != 0.0)) && (!(scratch.values[2228] != 0.0))) {
            scratch.store_ad(2076, &AdValue::offset(AdValue::mul(scratch.ad_value(239), scratch.ad_value(2121)), 1.0));
        }

        if ((!(scratch.values[2220] != 0.0)) && (scratch.values[2224] != 0.0)) {
            scratch.store_ad(2122, &AdValue::mul(scratch.ad_value(764), AdValue::div(scratch.ad_value(2076), scratch.ad_value(2119))));
        }

        if ((!(scratch.values[2220] != 0.0)) && (scratch.values[2224] != 0.0)) {
            scratch.store_ad(2123, &AdValue::add(AdValue::div(scratch.ad_value(2113), scratch.ad_value(2153)), scratch.ad_value(2011)));
        }

        if ((!(scratch.values[2220] != 0.0)) && (scratch.values[2224] != 0.0)) {
            scratch.store_ad(2124, &AdValue::scale(AdValue::mul(scratch.ad_value(2122), scratch.ad_value(2123)), 0.7071067811865475));
        }

        scratch.values[2229] = if (scratch.values[0] == (-1.0)) { 1.0 } else { 0.0 };

        if (((!(scratch.values[2220] != 0.0)) && (scratch.values[2224] != 0.0)) && (scratch.values[2229] != 0.0)) {
            scratch.store_ad(2124, &AdValue::div(scratch.ad_value(2124), AdValue::sqrt(AdValue::offset(scratch.ad_value(2124), 1.0))));
        }

        if ((!(scratch.values[2220] != 0.0)) && (scratch.values[2224] != 0.0)) {
            scratch.store_ad(2125, &AdValue::div_from_scalar(2.0, AdValue::offset(AdValue::sqrt(AdValue::offset(AdValue::scale(scratch.ad_value(2124), 4.0), 1.0)), 1.0)));
        }

        if ((!(scratch.values[2220] != 0.0)) && (scratch.values[2224] != 0.0)) {
            scratch.store_ad(2077, &AdValue::mul(scratch.ad_value(2125), scratch.ad_value(2124)));
        }

        if ((!(scratch.values[2220] != 0.0)) && (scratch.values[2224] != 0.0)) {
            scratch.store_ad(2126, &AdValue::mul(AdValue::mul(scratch.ad_value(2123), scratch.ad_value(2125)), AdValue::offset(AdValue::scale(AdValue::div(AdValue::mul(scratch.ad_value(2077), AdValue::sub_from_scalar(1.0, AdValue::mul(scratch.ad_value(2077), scratch.ad_value(2125)))), AdValue::offset(AdValue::scale(AdValue::mul(AdValue::square(scratch.ad_value(2077)), scratch.ad_value(2125)), 4.0), 1.0)), 0.86), 1.0)));
        }

        if ((!(scratch.values[2220] != 0.0)) && (scratch.values[2224] != 0.0)) {
            scratch.store_ad(2127, &AdValue::add(scratch.ad_value(2112), AdValue::scale(scratch.ad_value(2095), 0.5)));
        }

        if ((!(scratch.values[2220] != 0.0)) && (scratch.values[2224] != 0.0)) {
            scratch.store_ad(2128, &AdValue::scale(AdValue::div(AdValue::mul(AdValue::mul(scratch.ad_value(2095), scratch.ad_value(2109)), scratch.ad_value(2011)), AdValue::add(scratch.ad_value(2127), AdValue::sqrt(AdValue::sub(AdValue::square(scratch.ad_value(2127)), AdValue::scale(AdValue::mul(scratch.ad_value(2095), scratch.ad_value(2109)), 0.98))))), 0.98));
        }

        if ((!(scratch.values[2220] != 0.0)) && (scratch.values[2224] != 0.0)) {
            scratch.store_ad(2129, &AdValue::add(scratch.ad_value(2126), scratch.ad_value(2128)));
        }

        if ((!(scratch.values[2220] != 0.0)) && (scratch.values[2224] != 0.0)) {
            scratch.store_ad(2130, &AdValue::scale(AdValue::mul(scratch.ad_value(2126), scratch.ad_value(2128)), 2.0));
        }

        if ((!(scratch.values[2220] != 0.0)) && (scratch.values[2224] != 0.0)) {
            scratch.store_ad(2131, &AdValue::div(scratch.ad_value(2130), AdValue::add(scratch.ad_value(2129), AdValue::sqrt(AdValue::sub(AdValue::square(scratch.ad_value(2129)), AdValue::scale(scratch.ad_value(2130), 1.98))))));
        }

        if ((!(scratch.values[2220] != 0.0)) && (scratch.values[2224] != 0.0)) {
            scratch.store_ad(870, &AdValue::sub(scratch.ad_value(2131), AdValue::mul(scratch.ad_value(2011), AdValue::ln(AdValue::offset(AdValue::div(AdValue::mul(AdValue::mul(scratch.ad_value(2131), AdValue::sub(scratch.ad_value(2131), AdValue::mul(AdValue::scale(scratch.ad_value(2127), 2.0), scratch.ad_value(2011)))), scratch.ad_value(2096)), AdValue::mul(AdValue::square(scratch.ad_value(2011)), scratch.ad_value(2109))), 1.0)))));
        }

        if ((!(scratch.values[2220] != 0.0)) && (!(scratch.values[2224] != 0.0))) {
            scratch.values[870] = scratch.values[869];
            scratch.node_derivatives[870] = scratch.node_derivatives[869];
            scratch.branch_derivatives[870] = scratch.branch_derivatives[869];
        }

        if (!(scratch.values[2220] != 0.0)) {
            scratch.store_ad(2076, &AdValue::pow(AdValue::div(scratch.ad_value(854), scratch.ad_value(870)), scratch.ad_value(240)));
        }

        if (!(scratch.values[2220] != 0.0)) {
            scratch.store_ad(2132, &AdValue::mul(scratch.ad_value(854), AdValue::pow(AdValue::offset(scratch.ad_value(2076), 1.0), AdValue::neg(scratch.ad_value(820)))));
        }

        if (!(scratch.values[2220] != 0.0)) {
            scratch.store_ad(2133, &AdValue::mul(scratch.ad_value(2132), scratch.ad_value(2012)));
        }

        if (!(scratch.values[2220] != 0.0)) {
            scratch.store_ad(2134, &AdValue::add(scratch.ad_value(2100), scratch.ad_value(2133)));
        }

        scratch.values[2230] = if (scratch.values[2133] < 460.51701859880916) { 1.0 } else { 0.0 };

        if ((!(scratch.values[2220] != 0.0)) && (scratch.values[2230] != 0.0)) {
            scratch.store_ad(2135, &AdValue::exp(AdValue::neg(scratch.ad_value(2133))));
        }

        if ((!(scratch.values[2220] != 0.0)) && (!(scratch.values[2230] != 0.0))) {
            scratch.store_ad(2135, &AdValue::div_from_scalar(1e-200, AdValue::offset(AdValue::mul(AdValue::offset(scratch.ad_value(2133), (-460.51701859880916)), AdValue::offset(AdValue::scale(AdValue::mul(AdValue::offset(scratch.ad_value(2133), (-460.51701859880916)), AdValue::offset(AdValue::scale(AdValue::offset(scratch.ad_value(2133), (-460.51701859880916)), 0.3333333333333333), 1.0)), 0.5), 1.0)), 1.0)));
        }

    }
}
