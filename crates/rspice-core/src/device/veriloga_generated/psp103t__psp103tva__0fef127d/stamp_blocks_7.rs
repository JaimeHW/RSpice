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
        scratch.values[1944] = if (scratch.values[1377] > 0.0) { 1.0 } else { 0.0 };

        if (((((scratch.values[1320] != 0.0) && (scratch.values[1337] != 0.0)) && (!(scratch.values[1935] != 0.0))) && (!(scratch.values[1940] != 0.0))) && (scratch.values[1944] != 0.0)) {
            scratch.values[1378] = scratch.values[1340];
            scratch.node_derivatives[1378] = scratch.node_derivatives[1340];
            scratch.branch_derivatives[1378] = scratch.branch_derivatives[1340];
        }

        scratch.values[1945] = if (scratch.values[1376] > (-230.25850929940458)) { 1.0 } else { 0.0 };

        if ((((((scratch.values[1320] != 0.0) && (scratch.values[1337] != 0.0)) && (!(scratch.values[1935] != 0.0))) && (!(scratch.values[1940] != 0.0))) && (!(scratch.values[1944] != 0.0))) && (scratch.values[1945] != 0.0)) {
            scratch.store_ad(1356, &AdValue::exp(scratch.ad_value(1376)));
        }

        if ((((((scratch.values[1320] != 0.0) && (scratch.values[1337] != 0.0)) && (!(scratch.values[1935] != 0.0))) && (!(scratch.values[1940] != 0.0))) && (!(scratch.values[1944] != 0.0))) && (!(scratch.values[1945] != 0.0))) {
            scratch.store_ad(1356, &AdValue::div_from_scalar(1e-100, AdValue::offset(AdValue::mul(AdValue::sub_from_scalar((-230.25850929940458), scratch.ad_value(1376)), AdValue::offset(AdValue::scale(AdValue::mul(AdValue::sub_from_scalar((-230.25850929940458), scratch.ad_value(1376)), AdValue::offset(AdValue::scale(AdValue::sub_from_scalar((-230.25850929940458), scratch.ad_value(1376)), 0.3333333333333333), 1.0)), 0.5), 1.0)), 1.0)));
        }

        if (((((scratch.values[1320] != 0.0) && (scratch.values[1337] != 0.0)) && (!(scratch.values[1935] != 0.0))) && (!(scratch.values[1940] != 0.0))) && (!(scratch.values[1944] != 0.0))) {
            scratch.store_ad(1378, &AdValue::sub(AdValue::scale(scratch.ad_value(1356), 2.0), scratch.ad_value(1340)));
        }

        if ((((scratch.values[1320] != 0.0) && (scratch.values[1337] != 0.0)) && (!(scratch.values[1935] != 0.0))) && (!(scratch.values[1940] != 0.0))) {
            scratch.store_ad(1379, &AdValue::scale(AdValue::div(AdValue::mul(scratch.ad_value(634), scratch.ad_value(1378)), scratch.ad_value(1374)), (1.772453850905516 * 0.5)));
        }

        if ((((scratch.values[1320] != 0.0) && (scratch.values[1337] != 0.0)) && (!(scratch.values[1935] != 0.0))) && (!(scratch.values[1940] != 0.0))) {
            scratch.store_ad(1365, &AdValue::mul(scratch.ad_value(563), AdValue::mul(AdValue::mul(scratch.ad_value(1364), scratch.ad_value(1379)), scratch.ad_value(1373))));
        }

        scratch.values[1946] = if (scratch.values[569] == 0.0) { 1.0 } else { 0.0 };

        if ((((scratch.values[1320] != 0.0) && (scratch.values[1337] != 0.0)) && (!(scratch.values[1935] != 0.0))) && (scratch.values[1946] != 0.0)) {
            scratch.values[1380] = 0.0;
            scratch.node_derivatives[1380] = [0.0; Instance::NODE_COUNT];
            scratch.branch_derivatives[1380] = [0.0; Instance::BRANCH_COUNT];
        }

        scratch.values[1947] = if (scratch.values[549] == 0.5) { 1.0 } else { 0.0 };

        if (((((scratch.values[1320] != 0.0) && (scratch.values[1337] != 0.0)) && (!(scratch.values[1935] != 0.0))) && (!(scratch.values[1946] != 0.0))) && (scratch.values[1947] != 0.0)) {
            scratch.store_ad(1356, &AdValue::sqrt(AdValue::mul(AdValue::sub(scratch.ad_value(546), scratch.ad_value(1354)), scratch.ad_value(628))));
        }

        if (((((scratch.values[1320] != 0.0) && (scratch.values[1337] != 0.0)) && (!(scratch.values[1935] != 0.0))) && (!(scratch.values[1946] != 0.0))) && (!(scratch.values[1947] != 0.0))) {
            scratch.store_ad(1356, &AdValue::pow(AdValue::mul(AdValue::sub(scratch.ad_value(546), scratch.ad_value(1354)), scratch.ad_value(628)), scratch.ad_value(549)));
        }

        if ((((scratch.values[1320] != 0.0) && (scratch.values[1337] != 0.0)) && (!(scratch.values[1935] != 0.0))) && (!(scratch.values[1946] != 0.0))) {
            scratch.store_ad(1381, &AdValue::mul(scratch.ad_value(610), AdValue::div(AdValue::mul(AdValue::sub(scratch.ad_value(546), scratch.ad_value(1354)), scratch.ad_value(625)), scratch.ad_value(1356))));
        }

        scratch.values[1948] = if (((((-scratch.values[640]) / scratch.values[1381])) as f64).abs() < 230.25850929940458) { 1.0 } else { 0.0 };

        if (((((scratch.values[1320] != 0.0) && (scratch.values[1337] != 0.0)) && (!(scratch.values[1935] != 0.0))) && (!(scratch.values[1946] != 0.0))) && (scratch.values[1948] != 0.0)) {
            scratch.store_ad(1356, &AdValue::exp(AdValue::div(AdValue::neg(scratch.ad_value(640)), scratch.ad_value(1381))));
        }

        scratch.values[1949] = if (((-scratch.values[640]) / scratch.values[1381]) < 0.0) { 1.0 } else { 0.0 };

        if ((((((scratch.values[1320] != 0.0) && (scratch.values[1337] != 0.0)) && (!(scratch.values[1935] != 0.0))) && (!(scratch.values[1946] != 0.0))) && (!(scratch.values[1948] != 0.0))) && (scratch.values[1949] != 0.0)) {
            let assign37160_ad_e48788: AdValue = AdValue::offset(AdValue::mul(AdValue::sub_from_scalar((-230.25850929940458), AdValue::div(AdValue::neg(scratch.ad_value(640)), scratch.ad_value(1381))), AdValue::offset(AdValue::scale(AdValue::mul(AdValue::sub_from_scalar((-230.25850929940458), AdValue::div(AdValue::neg(scratch.ad_value(640)), scratch.ad_value(1381))), AdValue::offset(AdValue::scale(AdValue::sub_from_scalar((-230.25850929940458), AdValue::div(AdValue::neg(scratch.ad_value(640)), scratch.ad_value(1381))), 0.3333333333333333), 1.0)), 0.5), 1.0)), 1.0);
            scratch.store_ad(1356, &AdValue::div_from_scalar(1e-100, assign37160_ad_e48788));
        }

        if ((((((scratch.values[1320] != 0.0) && (scratch.values[1337] != 0.0)) && (!(scratch.values[1935] != 0.0))) && (!(scratch.values[1946] != 0.0))) && (!(scratch.values[1948] != 0.0))) && (!(scratch.values[1949] != 0.0))) {
            let assign37170_ad_e48838: AdValue = AdValue::scale(AdValue::offset(AdValue::mul(AdValue::offset(AdValue::div(AdValue::neg(scratch.ad_value(640)), scratch.ad_value(1381)), (-230.25850929940458)), AdValue::offset(AdValue::scale(AdValue::mul(AdValue::offset(AdValue::div(AdValue::neg(scratch.ad_value(640)), scratch.ad_value(1381)), (-230.25850929940458)), AdValue::offset(AdValue::scale(AdValue::offset(AdValue::div(AdValue::neg(scratch.ad_value(640)), scratch.ad_value(1381)), (-230.25850929940458)), 0.3333333333333333), 1.0)), 0.5), 1.0)), 1.0), 1e100);
            scratch.store_ad(1356, &assign37170_ad_e48838);
        }

        if ((((scratch.values[1320] != 0.0) && (scratch.values[1337] != 0.0)) && (!(scratch.values[1935] != 0.0))) && (!(scratch.values[1946] != 0.0))) {
            scratch.store_ad(1380, &AdValue::mul(scratch.ad_value(569), AdValue::mul(AdValue::mul(AdValue::mul(scratch.ad_value(527), scratch.ad_value(1381)), scratch.ad_value(1381)), scratch.ad_value(1356))));
        }

        scratch.values[1950] = if (scratch.values[578] > 1000.0) { 1.0 } else { 0.0 };

        if ((((scratch.values[1320] != 0.0) && (scratch.values[1337] != 0.0)) && (!(scratch.values[1935] != 0.0))) && (scratch.values[1950] != 0.0)) {
            scratch.values[1382] = 1.0;
            scratch.node_derivatives[1382] = [0.0; Instance::NODE_COUNT];
            scratch.branch_derivatives[1382] = [0.0; Instance::BRANCH_COUNT];
        }

        scratch.values[1951] = if (scratch.values[1355] > ((-scratch.values[500]) * scratch.values[578])) { 1.0 } else { 0.0 };

        scratch.values[1952] = if (scratch.values[581] == 4.0) { 1.0 } else { 0.0 };

        if ((((((scratch.values[1320] != 0.0) && (scratch.values[1337] != 0.0)) && (!(scratch.values[1935] != 0.0))) && (!(scratch.values[1950] != 0.0))) && (scratch.values[1951] != 0.0)) && (scratch.values[1952] != 0.0)) {
            scratch.store_ad(1356, &AdValue::mul(AdValue::mul(AdValue::mul(AdValue::mul(scratch.ad_value(1355), scratch.ad_value(646)), AdValue::mul(scratch.ad_value(1355), scratch.ad_value(646))), AdValue::mul(scratch.ad_value(1355), scratch.ad_value(646))), AdValue::mul(scratch.ad_value(1355), scratch.ad_value(646))));
        }

        if ((((((scratch.values[1320] != 0.0) && (scratch.values[1337] != 0.0)) && (!(scratch.values[1935] != 0.0))) && (!(scratch.values[1950] != 0.0))) && (scratch.values[1951] != 0.0)) && (!(scratch.values[1952] != 0.0))) {
            scratch.store_ad(1356, &AdValue::pow(AdValue::abs(AdValue::mul(scratch.ad_value(1355), scratch.ad_value(646))), scratch.ad_value(581)));
        }

        if (((((scratch.values[1320] != 0.0) && (scratch.values[1337] != 0.0)) && (!(scratch.values[1935] != 0.0))) && (!(scratch.values[1950] != 0.0))) && (scratch.values[1951] != 0.0)) {
            scratch.store_ad(1382, &AdValue::div_from_scalar(1.0, AdValue::sub_from_scalar(1.0, scratch.ad_value(1356))));
        }

        if (((((scratch.values[1320] != 0.0) && (scratch.values[1337] != 0.0)) && (!(scratch.values[1935] != 0.0))) && (!(scratch.values[1950] != 0.0))) && (!(scratch.values[1951] != 0.0))) {
            scratch.store_ad(1382, &AdValue::add(scratch.ad_value(643), AdValue::mul(AdValue::add(scratch.ad_value(1355), AdValue::scale(scratch.ad_value(578), scratch.values[500])), scratch.ad_value(649))));
        }

        if (((scratch.values[1320] != 0.0) && (scratch.values[1337] != 0.0)) && (!(scratch.values[1935] != 0.0))) {
            scratch.store_ad(1383, &AdValue::mul(AdValue::add(AdValue::add(AdValue::add(scratch.ad_value(1357), scratch.ad_value(1358)), scratch.ad_value(1365)), scratch.ad_value(1380)), scratch.ad_value(1382)));
        }

        scratch.values[1953] = if (scratch.values[725] == 0.0) { 1.0 } else { 0.0 };

        if (((scratch.values[1320] != 0.0) && (scratch.values[1337] != 0.0)) && (scratch.values[1953] != 0.0)) {
            scratch.values[1384] = 0.0;
            scratch.node_derivatives[1384] = [0.0; Instance::NODE_COUNT];
            scratch.branch_derivatives[1384] = [0.0; Instance::BRANCH_COUNT];
        }

        scratch.values[1954] = if (scratch.values[608] == 0.5) { 1.0 } else { 0.0 };

        if ((((scratch.values[1320] != 0.0) && (scratch.values[1337] != 0.0)) && (!(scratch.values[1953] != 0.0))) && (scratch.values[1954] != 0.0)) {
            scratch.store_ad(1356, &AdValue::sqrt(AdValue::sub_from_scalar(1.0, AdValue::mul(scratch.ad_value(1348), scratch.ad_value(605)))));
        }

        if ((((scratch.values[1320] != 0.0) && (scratch.values[1337] != 0.0)) && (!(scratch.values[1953] != 0.0))) && (!(scratch.values[1954] != 0.0))) {
            scratch.store_ad(1356, &AdValue::pow(AdValue::sub_from_scalar(1.0, AdValue::mul(scratch.ad_value(1348), scratch.ad_value(605))), scratch.ad_value(608)));
        }

        if (((scratch.values[1320] != 0.0) && (scratch.values[1337] != 0.0)) && (!(scratch.values[1953] != 0.0))) {
            scratch.store_ad(1357, &AdValue::mul(scratch.ad_value(596), scratch.ad_value(1347)));
        }

        scratch.values[1955] = if ((scratch.values[561] == 0.0) && (scratch.values[564] == 0.0)) { 1.0 } else { 0.0 };

        if ((((scratch.values[1320] != 0.0) && (scratch.values[1337] != 0.0)) && (!(scratch.values[1953] != 0.0))) && (scratch.values[1955] != 0.0)) {
            scratch.values[1358] = 0.0;
            scratch.node_derivatives[1358] = [0.0; Instance::NODE_COUNT];
            scratch.branch_derivatives[1358] = [0.0; Instance::BRANCH_COUNT];
        }

        if ((((scratch.values[1320] != 0.0) && (scratch.values[1337] != 0.0)) && (!(scratch.values[1953] != 0.0))) && (!(scratch.values[1955] != 0.0))) {
            scratch.store_ad(1359, &AdValue::sub(scratch.ad_value(602), scratch.ad_value(1353)));
        }

        if ((((scratch.values[1320] != 0.0) && (scratch.values[1337] != 0.0)) && (!(scratch.values[1953] != 0.0))) && (!(scratch.values[1955] != 0.0))) {
            scratch.store_ad(1360, &AdValue::sub_from_scalar(1.0, AdValue::sqrt(AdValue::sub_from_scalar(1.0, AdValue::div(scratch.ad_value(1351), scratch.ad_value(1359))))));
        }

        scratch.values[1956] = if (scratch.values[550] == 0.5) { 1.0 } else { 0.0 };

        if (((((scratch.values[1320] != 0.0) && (scratch.values[1337] != 0.0)) && (!(scratch.values[1953] != 0.0))) && (!(scratch.values[1955] != 0.0))) && (scratch.values[1956] != 0.0)) {
            scratch.values[1361] = 0.0;
            scratch.node_derivatives[1361] = [0.0; Instance::NODE_COUNT];
            scratch.branch_derivatives[1361] = [0.0; Instance::BRANCH_COUNT];
        }

        if (((((scratch.values[1320] != 0.0) && (scratch.values[1337] != 0.0)) && (!(scratch.values[1953] != 0.0))) && (!(scratch.values[1955] != 0.0))) && (!(scratch.values[1956] != 0.0))) {
            scratch.store_ad(1361, &AdValue::mul(AdValue::add(AdValue::div(AdValue::mul(AdValue::square(scratch.ad_value(1360)), AdValue::ln(scratch.ad_value(1360))), AdValue::sub_from_scalar(1.0, scratch.ad_value(1360))), scratch.ad_value(1360)), AdValue::sub_from_scalar(1.0, AdValue::scale(scratch.ad_value(550), 2.0))));
        }

        if ((((scratch.values[1320] != 0.0) && (scratch.values[1337] != 0.0)) && (!(scratch.values[1953] != 0.0))) && (!(scratch.values[1955] != 0.0))) {
            scratch.store_ad(1362, &AdValue::add(scratch.ad_value(1360), scratch.ad_value(1361)));
        }

        scratch.values[1957] = if (scratch.values[550] == 0.5) { 1.0 } else { 0.0 };

        if (((((scratch.values[1320] != 0.0) && (scratch.values[1337] != 0.0)) && (!(scratch.values[1953] != 0.0))) && (!(scratch.values[1955] != 0.0))) && (scratch.values[1957] != 0.0)) {
            scratch.store_ad(1356, &AdValue::sqrt(AdValue::mul(scratch.ad_value(1359), scratch.ad_value(629))));
        }

        if (((((scratch.values[1320] != 0.0) && (scratch.values[1337] != 0.0)) && (!(scratch.values[1953] != 0.0))) && (!(scratch.values[1955] != 0.0))) && (!(scratch.values[1957] != 0.0))) {
            scratch.store_ad(1356, &AdValue::pow(AdValue::mul(scratch.ad_value(1359), scratch.ad_value(629)), scratch.ad_value(550)));
        }

        if ((((scratch.values[1320] != 0.0) && (scratch.values[1337] != 0.0)) && (!(scratch.values[1953] != 0.0))) && (!(scratch.values[1955] != 0.0))) {
            scratch.store_ad(1363, &AdValue::mul(scratch.ad_value(623), scratch.ad_value(1356)));
        }

        if ((((scratch.values[1320] != 0.0) && (scratch.values[1337] != 0.0)) && (!(scratch.values[1953] != 0.0))) && (!(scratch.values[1955] != 0.0))) {
            scratch.store_ad(1364, &AdValue::mul(scratch.ad_value(593), AdValue::mul(AdValue::offset(scratch.ad_value(1350), (-1.0)), scratch.ad_value(1363))));
        }

        if ((((scratch.values[1320] != 0.0) && (scratch.values[1337] != 0.0)) && (!(scratch.values[1953] != 0.0))) && (!(scratch.values[1955] != 0.0))) {
            scratch.store_ad(1358, &AdValue::mul(scratch.ad_value(561), AdValue::mul(scratch.ad_value(1364), scratch.ad_value(1362))));
        }

        scratch.values[1958] = if (scratch.values[564] == 0.0) { 1.0 } else { 0.0 };

        if ((((scratch.values[1320] != 0.0) && (scratch.values[1337] != 0.0)) && (!(scratch.values[1953] != 0.0))) && (scratch.values[1958] != 0.0)) {
            scratch.values[1365] = 0.0;
            scratch.node_derivatives[1365] = [0.0; Instance::NODE_COUNT];
            scratch.branch_derivatives[1365] = [0.0; Instance::BRANCH_COUNT];
        }

        if ((((scratch.values[1320] != 0.0) && (scratch.values[1337] != 0.0)) && (!(scratch.values[1953] != 0.0))) && (!(scratch.values[1958] != 0.0))) {
            scratch.store_ad(1366, &AdValue::mul(scratch.ad_value(638), AdValue::div(AdValue::mul(scratch.ad_value(1363), scratch.ad_value(608)), scratch.ad_value(1359))));
        }

        if ((((scratch.values[1320] != 0.0) && (scratch.values[1337] != 0.0)) && (!(scratch.values[1953] != 0.0))) && (!(scratch.values[1958] != 0.0))) {
            scratch.store_ad(1367, &AdValue::div(AdValue::scale(scratch.ad_value(635), 0.666666666666667), scratch.ad_value(1366)));
        }

        if ((((scratch.values[1320] != 0.0) && (scratch.values[1337] != 0.0)) && (!(scratch.values[1953] != 0.0))) && (!(scratch.values[1958] != 0.0))) {
            scratch.store_ad(1368, &AdValue::square(scratch.ad_value(1367)));
        }

        if ((((scratch.values[1320] != 0.0) && (scratch.values[1337] != 0.0)) && (!(scratch.values[1953] != 0.0))) && (!(scratch.values[1958] != 0.0))) {
            scratch.store_ad(1369, &AdValue::sqrt(AdValue::div(AdValue::square(scratch.ad_value(1368)), AdValue::offset(AdValue::square(scratch.ad_value(1368)), 1.0))));
        }

        if ((((scratch.values[1320] != 0.0) && (scratch.values[1337] != 0.0)) && (!(scratch.values[1953] != 0.0))) && (!(scratch.values[1958] != 0.0))) {
            scratch.store_ad(1370, &AdValue::sqrt(AdValue::abs(scratch.ad_value(1369))));
        }

        if ((((scratch.values[1320] != 0.0) && (scratch.values[1337] != 0.0)) && (!(scratch.values[1953] != 0.0))) && (!(scratch.values[1958] != 0.0))) {
            scratch.store_ad(1371, &AdValue::mul(scratch.ad_value(1369), scratch.ad_value(1370)));
        }

        scratch.values[1959] = if (((-scratch.values[550]) * scratch.values[611]) == (-1.0)) { 1.0 } else { 0.0 };

        if (((((scratch.values[1320] != 0.0) && (scratch.values[1337] != 0.0)) && (!(scratch.values[1953] != 0.0))) && (!(scratch.values[1958] != 0.0))) && (scratch.values[1959] != 0.0)) {
            scratch.store_ad(1372, &AdValue::div_from_scalar(1.0, AdValue::offset(AdValue::mul(scratch.ad_value(1366), scratch.ad_value(1371)), 1.0)));
        }

        if (((((scratch.values[1320] != 0.0) && (scratch.values[1337] != 0.0)) && (!(scratch.values[1953] != 0.0))) && (!(scratch.values[1958] != 0.0))) && (!(scratch.values[1959] != 0.0))) {
            scratch.store_ad(1372, &AdValue::pow(AdValue::offset(AdValue::mul(scratch.ad_value(1366), scratch.ad_value(1371)), 1.0), AdValue::mul(AdValue::neg(scratch.ad_value(550)), scratch.ad_value(611))));
        }

        if ((((scratch.values[1320] != 0.0) && (scratch.values[1337] != 0.0)) && (!(scratch.values[1953] != 0.0))) && (!(scratch.values[1958] != 0.0))) {
            scratch.store_ad(1373, &AdValue::div(AdValue::mul(scratch.ad_value(1362), scratch.ad_value(1372)), AdValue::add(scratch.ad_value(1362), scratch.ad_value(1372))));
        }

        if ((((scratch.values[1320] != 0.0) && (scratch.values[1337] != 0.0)) && (!(scratch.values[1953] != 0.0))) && (!(scratch.values[1958] != 0.0))) {
            scratch.store_ad(1374, &AdValue::sqrt(AdValue::scale(AdValue::div(scratch.ad_value(1366), scratch.ad_value(1370)), 0.375)));
        }

        if ((((scratch.values[1320] != 0.0) && (scratch.values[1337] != 0.0)) && (!(scratch.values[1953] != 0.0))) && (!(scratch.values[1958] != 0.0))) {
            scratch.store_ad(1375, &AdValue::sub(AdValue::scale(AdValue::mul(scratch.ad_value(1367), scratch.ad_value(1370)), 2.0), scratch.ad_value(1369)));
        }

        if ((((scratch.values[1320] != 0.0) && (scratch.values[1337] != 0.0)) && (!(scratch.values[1953] != 0.0))) && (!(scratch.values[1958] != 0.0))) {
            scratch.store_ad(1376, &AdValue::add(AdValue::sub(AdValue::mul(AdValue::mul(scratch.ad_value(635), scratch.ad_value(1367)), scratch.ad_value(1370)), AdValue::mul(scratch.ad_value(635), scratch.ad_value(1369))), AdValue::scale(AdValue::mul(scratch.ad_value(1366), scratch.ad_value(1371)), 0.5)));
        }

        if ((((scratch.values[1320] != 0.0) && (scratch.values[1337] != 0.0)) && (!(scratch.values[1953] != 0.0))) && (!(scratch.values[1958] != 0.0))) {
            scratch.store_ad(1377, &AdValue::mul(AdValue::offset(scratch.ad_value(1375), (-1.0)), scratch.ad_value(1374)));
        }

        if ((((scratch.values[1320] != 0.0) && (scratch.values[1337] != 0.0)) && (!(scratch.values[1953] != 0.0))) && (!(scratch.values[1958] != 0.0))) {
            scratch.store_ad(1338, &AdValue::square(scratch.ad_value(1377)));
        }

        scratch.values[1960] = if (scratch.values[1377] > 0.0) { 1.0 } else { 0.0 };

        if (((((scratch.values[1320] != 0.0) && (scratch.values[1337] != 0.0)) && (!(scratch.values[1953] != 0.0))) && (!(scratch.values[1958] != 0.0))) && (scratch.values[1960] != 0.0)) {
            scratch.store_ad(1339, &AdValue::div_from_scalar(1.0, AdValue::offset(AdValue::scale(scratch.ad_value(1377), scratch.values[428]), 1.0)));
        }

        if (((((scratch.values[1320] != 0.0) && (scratch.values[1337] != 0.0)) && (!(scratch.values[1953] != 0.0))) && (!(scratch.values[1958] != 0.0))) && (!(scratch.values[1960] != 0.0))) {
            scratch.store_ad(1339, &AdValue::div_from_scalar(1.0, AdValue::sub_from_scalar(1.0, AdValue::scale(scratch.ad_value(1377), scratch.values[428]))));
        }

        scratch.values[1961] = if (((-scratch.values[1338]) + scratch.values[1376]) > (-230.25850929940458)) { 1.0 } else { 0.0 };

        if (((((scratch.values[1320] != 0.0) && (scratch.values[1337] != 0.0)) && (!(scratch.values[1953] != 0.0))) && (!(scratch.values[1958] != 0.0))) && (scratch.values[1961] != 0.0)) {
            scratch.store_ad(1356, &AdValue::exp(AdValue::sub(scratch.ad_value(1376), scratch.ad_value(1338))));
        }

        if (((((scratch.values[1320] != 0.0) && (scratch.values[1337] != 0.0)) && (!(scratch.values[1953] != 0.0))) && (!(scratch.values[1958] != 0.0))) && (!(scratch.values[1961] != 0.0))) {
            let assign37720_ad_e49667: AdValue = AdValue::div_from_scalar(1e-100, AdValue::offset(AdValue::mul(AdValue::sub_from_scalar((-230.25850929940458), AdValue::sub(scratch.ad_value(1376), scratch.ad_value(1338))), AdValue::offset(AdValue::scale(AdValue::mul(AdValue::sub_from_scalar((-230.25850929940458), AdValue::sub(scratch.ad_value(1376), scratch.ad_value(1338))), AdValue::offset(AdValue::scale(AdValue::sub_from_scalar((-230.25850929940458), AdValue::sub(scratch.ad_value(1376), scratch.ad_value(1338))), 0.3333333333333333), 1.0)), 0.5), 1.0)), 1.0));
            scratch.store_ad(1356, &assign37720_ad_e49667);
        }

        if ((((scratch.values[1320] != 0.0) && (scratch.values[1337] != 0.0)) && (!(scratch.values[1953] != 0.0))) && (!(scratch.values[1958] != 0.0))) {
            scratch.store_ad(1340, &AdValue::mul(AdValue::add(AdValue::add(AdValue::scale(scratch.ad_value(1339), 0.29214664), AdValue::scale(AdValue::square(scratch.ad_value(1339)), scratch.values[429])), AdValue::scale(AdValue::mul(AdValue::square(scratch.ad_value(1339)), scratch.ad_value(1339)), scratch.values[430])), scratch.ad_value(1356)));
        }

        scratch.values[1962] = if (scratch.values[1377] > 0.0) { 1.0 } else { 0.0 };

        if (((((scratch.values[1320] != 0.0) && (scratch.values[1337] != 0.0)) && (!(scratch.values[1953] != 0.0))) && (!(scratch.values[1958] != 0.0))) && (scratch.values[1962] != 0.0)) {
            scratch.values[1378] = scratch.values[1340];
            scratch.node_derivatives[1378] = scratch.node_derivatives[1340];
            scratch.branch_derivatives[1378] = scratch.branch_derivatives[1340];
        }

        scratch.values[1963] = if (scratch.values[1376] > (-230.25850929940458)) { 1.0 } else { 0.0 };

        if ((((((scratch.values[1320] != 0.0) && (scratch.values[1337] != 0.0)) && (!(scratch.values[1953] != 0.0))) && (!(scratch.values[1958] != 0.0))) && (!(scratch.values[1962] != 0.0))) && (scratch.values[1963] != 0.0)) {
            scratch.store_ad(1356, &AdValue::exp(scratch.ad_value(1376)));
        }

        if ((((((scratch.values[1320] != 0.0) && (scratch.values[1337] != 0.0)) && (!(scratch.values[1953] != 0.0))) && (!(scratch.values[1958] != 0.0))) && (!(scratch.values[1962] != 0.0))) && (!(scratch.values[1963] != 0.0))) {
            scratch.store_ad(1356, &AdValue::div_from_scalar(1e-100, AdValue::offset(AdValue::mul(AdValue::sub_from_scalar((-230.25850929940458), scratch.ad_value(1376)), AdValue::offset(AdValue::scale(AdValue::mul(AdValue::sub_from_scalar((-230.25850929940458), scratch.ad_value(1376)), AdValue::offset(AdValue::scale(AdValue::sub_from_scalar((-230.25850929940458), scratch.ad_value(1376)), 0.3333333333333333), 1.0)), 0.5), 1.0)), 1.0)));
        }

        if (((((scratch.values[1320] != 0.0) && (scratch.values[1337] != 0.0)) && (!(scratch.values[1953] != 0.0))) && (!(scratch.values[1958] != 0.0))) && (!(scratch.values[1962] != 0.0))) {
            scratch.store_ad(1378, &AdValue::sub(AdValue::scale(scratch.ad_value(1356), 2.0), scratch.ad_value(1340)));
        }

        if ((((scratch.values[1320] != 0.0) && (scratch.values[1337] != 0.0)) && (!(scratch.values[1953] != 0.0))) && (!(scratch.values[1958] != 0.0))) {
            scratch.store_ad(1379, &AdValue::scale(AdValue::div(AdValue::mul(scratch.ad_value(635), scratch.ad_value(1378)), scratch.ad_value(1374)), (1.772453850905516 * 0.5)));
        }

        if ((((scratch.values[1320] != 0.0) && (scratch.values[1337] != 0.0)) && (!(scratch.values[1953] != 0.0))) && (!(scratch.values[1958] != 0.0))) {
            scratch.store_ad(1365, &AdValue::mul(scratch.ad_value(564), AdValue::mul(AdValue::mul(scratch.ad_value(1364), scratch.ad_value(1379)), scratch.ad_value(1373))));
        }

        scratch.values[1964] = if (scratch.values[570] == 0.0) { 1.0 } else { 0.0 };

        if ((((scratch.values[1320] != 0.0) && (scratch.values[1337] != 0.0)) && (!(scratch.values[1953] != 0.0))) && (scratch.values[1964] != 0.0)) {
            scratch.values[1380] = 0.0;
            scratch.node_derivatives[1380] = [0.0; Instance::NODE_COUNT];
            scratch.branch_derivatives[1380] = [0.0; Instance::BRANCH_COUNT];
        }

        scratch.values[1965] = if (scratch.values[550] == 0.5) { 1.0 } else { 0.0 };

        if (((((scratch.values[1320] != 0.0) && (scratch.values[1337] != 0.0)) && (!(scratch.values[1953] != 0.0))) && (!(scratch.values[1964] != 0.0))) && (scratch.values[1965] != 0.0)) {
            scratch.store_ad(1356, &AdValue::sqrt(AdValue::mul(AdValue::sub(scratch.ad_value(547), scratch.ad_value(1354)), scratch.ad_value(629))));
        }

        if (((((scratch.values[1320] != 0.0) && (scratch.values[1337] != 0.0)) && (!(scratch.values[1953] != 0.0))) && (!(scratch.values[1964] != 0.0))) && (!(scratch.values[1965] != 0.0))) {
            scratch.store_ad(1356, &AdValue::pow(AdValue::mul(AdValue::sub(scratch.ad_value(547), scratch.ad_value(1354)), scratch.ad_value(629)), scratch.ad_value(550)));
        }

        if ((((scratch.values[1320] != 0.0) && (scratch.values[1337] != 0.0)) && (!(scratch.values[1953] != 0.0))) && (!(scratch.values[1964] != 0.0))) {
            scratch.store_ad(1381, &AdValue::mul(scratch.ad_value(611), AdValue::div(AdValue::mul(AdValue::sub(scratch.ad_value(547), scratch.ad_value(1354)), scratch.ad_value(626)), scratch.ad_value(1356))));
        }

        scratch.values[1966] = if (((((-scratch.values[641]) / scratch.values[1381])) as f64).abs() < 230.25850929940458) { 1.0 } else { 0.0 };

        if (((((scratch.values[1320] != 0.0) && (scratch.values[1337] != 0.0)) && (!(scratch.values[1953] != 0.0))) && (!(scratch.values[1964] != 0.0))) && (scratch.values[1966] != 0.0)) {
            scratch.store_ad(1356, &AdValue::exp(AdValue::div(AdValue::neg(scratch.ad_value(641)), scratch.ad_value(1381))));
        }

        scratch.values[1967] = if (((-scratch.values[641]) / scratch.values[1381]) < 0.0) { 1.0 } else { 0.0 };

        if ((((((scratch.values[1320] != 0.0) && (scratch.values[1337] != 0.0)) && (!(scratch.values[1953] != 0.0))) && (!(scratch.values[1964] != 0.0))) && (!(scratch.values[1966] != 0.0))) && (scratch.values[1967] != 0.0)) {
            let assign37910_ad_e49994: AdValue = AdValue::offset(AdValue::mul(AdValue::sub_from_scalar((-230.25850929940458), AdValue::div(AdValue::neg(scratch.ad_value(641)), scratch.ad_value(1381))), AdValue::offset(AdValue::scale(AdValue::mul(AdValue::sub_from_scalar((-230.25850929940458), AdValue::div(AdValue::neg(scratch.ad_value(641)), scratch.ad_value(1381))), AdValue::offset(AdValue::scale(AdValue::sub_from_scalar((-230.25850929940458), AdValue::div(AdValue::neg(scratch.ad_value(641)), scratch.ad_value(1381))), 0.3333333333333333), 1.0)), 0.5), 1.0)), 1.0);
            scratch.store_ad(1356, &AdValue::div_from_scalar(1e-100, assign37910_ad_e49994));
        }

        if ((((((scratch.values[1320] != 0.0) && (scratch.values[1337] != 0.0)) && (!(scratch.values[1953] != 0.0))) && (!(scratch.values[1964] != 0.0))) && (!(scratch.values[1966] != 0.0))) && (!(scratch.values[1967] != 0.0))) {
            let assign37920_ad_e50044: AdValue = AdValue::scale(AdValue::offset(AdValue::mul(AdValue::offset(AdValue::div(AdValue::neg(scratch.ad_value(641)), scratch.ad_value(1381)), (-230.25850929940458)), AdValue::offset(AdValue::scale(AdValue::mul(AdValue::offset(AdValue::div(AdValue::neg(scratch.ad_value(641)), scratch.ad_value(1381)), (-230.25850929940458)), AdValue::offset(AdValue::scale(AdValue::offset(AdValue::div(AdValue::neg(scratch.ad_value(641)), scratch.ad_value(1381)), (-230.25850929940458)), 0.3333333333333333), 1.0)), 0.5), 1.0)), 1.0), 1e100);
            scratch.store_ad(1356, &assign37920_ad_e50044);
        }

        if ((((scratch.values[1320] != 0.0) && (scratch.values[1337] != 0.0)) && (!(scratch.values[1953] != 0.0))) && (!(scratch.values[1964] != 0.0))) {
            scratch.store_ad(1380, &AdValue::mul(scratch.ad_value(570), AdValue::mul(AdValue::mul(AdValue::mul(scratch.ad_value(527), scratch.ad_value(1381)), scratch.ad_value(1381)), scratch.ad_value(1356))));
        }

        scratch.values[1968] = if (scratch.values[579] > 1000.0) { 1.0 } else { 0.0 };

        if ((((scratch.values[1320] != 0.0) && (scratch.values[1337] != 0.0)) && (!(scratch.values[1953] != 0.0))) && (scratch.values[1968] != 0.0)) {
            scratch.values[1382] = 1.0;
            scratch.node_derivatives[1382] = [0.0; Instance::NODE_COUNT];
            scratch.branch_derivatives[1382] = [0.0; Instance::BRANCH_COUNT];
        }

        scratch.values[1969] = if (scratch.values[1355] > ((-scratch.values[500]) * scratch.values[579])) { 1.0 } else { 0.0 };

        scratch.values[1970] = if (scratch.values[582] == 4.0) { 1.0 } else { 0.0 };

        if ((((((scratch.values[1320] != 0.0) && (scratch.values[1337] != 0.0)) && (!(scratch.values[1953] != 0.0))) && (!(scratch.values[1968] != 0.0))) && (scratch.values[1969] != 0.0)) && (scratch.values[1970] != 0.0)) {
            scratch.store_ad(1356, &AdValue::mul(AdValue::mul(AdValue::mul(AdValue::mul(scratch.ad_value(1355), scratch.ad_value(647)), AdValue::mul(scratch.ad_value(1355), scratch.ad_value(647))), AdValue::mul(scratch.ad_value(1355), scratch.ad_value(647))), AdValue::mul(scratch.ad_value(1355), scratch.ad_value(647))));
        }

        if ((((((scratch.values[1320] != 0.0) && (scratch.values[1337] != 0.0)) && (!(scratch.values[1953] != 0.0))) && (!(scratch.values[1968] != 0.0))) && (scratch.values[1969] != 0.0)) && (!(scratch.values[1970] != 0.0))) {
            scratch.store_ad(1356, &AdValue::pow(AdValue::abs(AdValue::mul(scratch.ad_value(1355), scratch.ad_value(647))), scratch.ad_value(582)));
        }

        if (((((scratch.values[1320] != 0.0) && (scratch.values[1337] != 0.0)) && (!(scratch.values[1953] != 0.0))) && (!(scratch.values[1968] != 0.0))) && (scratch.values[1969] != 0.0)) {
            scratch.store_ad(1382, &AdValue::div_from_scalar(1.0, AdValue::sub_from_scalar(1.0, scratch.ad_value(1356))));
        }

        if (((((scratch.values[1320] != 0.0) && (scratch.values[1337] != 0.0)) && (!(scratch.values[1953] != 0.0))) && (!(scratch.values[1968] != 0.0))) && (!(scratch.values[1969] != 0.0))) {
            scratch.store_ad(1382, &AdValue::add(scratch.ad_value(644), AdValue::mul(AdValue::add(scratch.ad_value(1355), AdValue::scale(scratch.ad_value(579), scratch.values[500])), scratch.ad_value(650))));
        }

        if (((scratch.values[1320] != 0.0) && (scratch.values[1337] != 0.0)) && (!(scratch.values[1953] != 0.0))) {
            scratch.store_ad(1384, &AdValue::mul(AdValue::add(AdValue::add(AdValue::add(scratch.ad_value(1357), scratch.ad_value(1358)), scratch.ad_value(1365)), scratch.ad_value(1380)), scratch.ad_value(1382)));
        }

        scratch.values[1971] = if (scratch.values[726] == 0.0) { 1.0 } else { 0.0 };

        if (((scratch.values[1320] != 0.0) && (scratch.values[1337] != 0.0)) && (scratch.values[1971] != 0.0)) {
            scratch.values[1385] = 0.0;
            scratch.node_derivatives[1385] = [0.0; Instance::NODE_COUNT];
            scratch.branch_derivatives[1385] = [0.0; Instance::BRANCH_COUNT];
        }

        scratch.values[1972] = if (scratch.values[609] == 0.5) { 1.0 } else { 0.0 };

        if ((((scratch.values[1320] != 0.0) && (scratch.values[1337] != 0.0)) && (!(scratch.values[1971] != 0.0))) && (scratch.values[1972] != 0.0)) {
            scratch.store_ad(1356, &AdValue::sqrt(AdValue::sub_from_scalar(1.0, AdValue::mul(scratch.ad_value(1348), scratch.ad_value(606)))));
        }

        if ((((scratch.values[1320] != 0.0) && (scratch.values[1337] != 0.0)) && (!(scratch.values[1971] != 0.0))) && (!(scratch.values[1972] != 0.0))) {
            scratch.store_ad(1356, &AdValue::pow(AdValue::sub_from_scalar(1.0, AdValue::mul(scratch.ad_value(1348), scratch.ad_value(606))), scratch.ad_value(609)));
        }

        if (((scratch.values[1320] != 0.0) && (scratch.values[1337] != 0.0)) && (!(scratch.values[1971] != 0.0))) {
            scratch.store_ad(1357, &AdValue::mul(scratch.ad_value(597), scratch.ad_value(1347)));
        }

        scratch.values[1973] = if ((scratch.values[562] == 0.0) && (scratch.values[565] == 0.0)) { 1.0 } else { 0.0 };

        if ((((scratch.values[1320] != 0.0) && (scratch.values[1337] != 0.0)) && (!(scratch.values[1971] != 0.0))) && (scratch.values[1973] != 0.0)) {
            scratch.values[1358] = 0.0;
            scratch.node_derivatives[1358] = [0.0; Instance::NODE_COUNT];
            scratch.branch_derivatives[1358] = [0.0; Instance::BRANCH_COUNT];
        }

        if ((((scratch.values[1320] != 0.0) && (scratch.values[1337] != 0.0)) && (!(scratch.values[1971] != 0.0))) && (!(scratch.values[1973] != 0.0))) {
            scratch.store_ad(1359, &AdValue::sub(scratch.ad_value(603), scratch.ad_value(1353)));
        }

        if ((((scratch.values[1320] != 0.0) && (scratch.values[1337] != 0.0)) && (!(scratch.values[1971] != 0.0))) && (!(scratch.values[1973] != 0.0))) {
            scratch.store_ad(1360, &AdValue::sub_from_scalar(1.0, AdValue::sqrt(AdValue::sub_from_scalar(1.0, AdValue::div(scratch.ad_value(1351), scratch.ad_value(1359))))));
        }

        scratch.values[1974] = if (scratch.values[551] == 0.5) { 1.0 } else { 0.0 };

        if (((((scratch.values[1320] != 0.0) && (scratch.values[1337] != 0.0)) && (!(scratch.values[1971] != 0.0))) && (!(scratch.values[1973] != 0.0))) && (scratch.values[1974] != 0.0)) {
            scratch.values[1361] = 0.0;
            scratch.node_derivatives[1361] = [0.0; Instance::NODE_COUNT];
            scratch.branch_derivatives[1361] = [0.0; Instance::BRANCH_COUNT];
        }

        if (((((scratch.values[1320] != 0.0) && (scratch.values[1337] != 0.0)) && (!(scratch.values[1971] != 0.0))) && (!(scratch.values[1973] != 0.0))) && (!(scratch.values[1974] != 0.0))) {
            scratch.store_ad(1361, &AdValue::mul(AdValue::add(AdValue::div(AdValue::mul(AdValue::square(scratch.ad_value(1360)), AdValue::ln(scratch.ad_value(1360))), AdValue::sub_from_scalar(1.0, scratch.ad_value(1360))), scratch.ad_value(1360)), AdValue::sub_from_scalar(1.0, AdValue::scale(scratch.ad_value(551), 2.0))));
        }

        if ((((scratch.values[1320] != 0.0) && (scratch.values[1337] != 0.0)) && (!(scratch.values[1971] != 0.0))) && (!(scratch.values[1973] != 0.0))) {
            scratch.store_ad(1362, &AdValue::add(scratch.ad_value(1360), scratch.ad_value(1361)));
        }

        scratch.values[1975] = if (scratch.values[551] == 0.5) { 1.0 } else { 0.0 };

        if (((((scratch.values[1320] != 0.0) && (scratch.values[1337] != 0.0)) && (!(scratch.values[1971] != 0.0))) && (!(scratch.values[1973] != 0.0))) && (scratch.values[1975] != 0.0)) {
            scratch.store_ad(1356, &AdValue::sqrt(AdValue::mul(scratch.ad_value(1359), scratch.ad_value(630))));
        }

        if (((((scratch.values[1320] != 0.0) && (scratch.values[1337] != 0.0)) && (!(scratch.values[1971] != 0.0))) && (!(scratch.values[1973] != 0.0))) && (!(scratch.values[1975] != 0.0))) {
            scratch.store_ad(1356, &AdValue::pow(AdValue::mul(scratch.ad_value(1359), scratch.ad_value(630)), scratch.ad_value(551)));
        }

        if ((((scratch.values[1320] != 0.0) && (scratch.values[1337] != 0.0)) && (!(scratch.values[1971] != 0.0))) && (!(scratch.values[1973] != 0.0))) {
            scratch.store_ad(1363, &AdValue::mul(scratch.ad_value(624), scratch.ad_value(1356)));
        }

        if ((((scratch.values[1320] != 0.0) && (scratch.values[1337] != 0.0)) && (!(scratch.values[1971] != 0.0))) && (!(scratch.values[1973] != 0.0))) {
            scratch.store_ad(1364, &AdValue::mul(scratch.ad_value(594), AdValue::mul(AdValue::offset(scratch.ad_value(1350), (-1.0)), scratch.ad_value(1363))));
        }

        if ((((scratch.values[1320] != 0.0) && (scratch.values[1337] != 0.0)) && (!(scratch.values[1971] != 0.0))) && (!(scratch.values[1973] != 0.0))) {
            scratch.store_ad(1358, &AdValue::mul(scratch.ad_value(562), AdValue::mul(scratch.ad_value(1364), scratch.ad_value(1362))));
        }

        scratch.values[1976] = if (scratch.values[565] == 0.0) { 1.0 } else { 0.0 };

        if ((((scratch.values[1320] != 0.0) && (scratch.values[1337] != 0.0)) && (!(scratch.values[1971] != 0.0))) && (scratch.values[1976] != 0.0)) {
            scratch.values[1365] = 0.0;
            scratch.node_derivatives[1365] = [0.0; Instance::NODE_COUNT];
            scratch.branch_derivatives[1365] = [0.0; Instance::BRANCH_COUNT];
        }

        if ((((scratch.values[1320] != 0.0) && (scratch.values[1337] != 0.0)) && (!(scratch.values[1971] != 0.0))) && (!(scratch.values[1976] != 0.0))) {
            scratch.store_ad(1366, &AdValue::mul(scratch.ad_value(639), AdValue::div(AdValue::mul(scratch.ad_value(1363), scratch.ad_value(609)), scratch.ad_value(1359))));
        }

        if ((((scratch.values[1320] != 0.0) && (scratch.values[1337] != 0.0)) && (!(scratch.values[1971] != 0.0))) && (!(scratch.values[1976] != 0.0))) {
            scratch.store_ad(1367, &AdValue::div(AdValue::scale(scratch.ad_value(636), 0.666666666666667), scratch.ad_value(1366)));
        }

        if ((((scratch.values[1320] != 0.0) && (scratch.values[1337] != 0.0)) && (!(scratch.values[1971] != 0.0))) && (!(scratch.values[1976] != 0.0))) {
            scratch.store_ad(1368, &AdValue::square(scratch.ad_value(1367)));
        }

        if ((((scratch.values[1320] != 0.0) && (scratch.values[1337] != 0.0)) && (!(scratch.values[1971] != 0.0))) && (!(scratch.values[1976] != 0.0))) {
            scratch.store_ad(1369, &AdValue::sqrt(AdValue::div(AdValue::square(scratch.ad_value(1368)), AdValue::offset(AdValue::square(scratch.ad_value(1368)), 1.0))));
        }

        if ((((scratch.values[1320] != 0.0) && (scratch.values[1337] != 0.0)) && (!(scratch.values[1971] != 0.0))) && (!(scratch.values[1976] != 0.0))) {
            scratch.store_ad(1370, &AdValue::sqrt(AdValue::abs(scratch.ad_value(1369))));
        }

        if ((((scratch.values[1320] != 0.0) && (scratch.values[1337] != 0.0)) && (!(scratch.values[1971] != 0.0))) && (!(scratch.values[1976] != 0.0))) {
            scratch.store_ad(1371, &AdValue::mul(scratch.ad_value(1369), scratch.ad_value(1370)));
        }

        scratch.values[1977] = if (((-scratch.values[551]) * scratch.values[612]) == (-1.0)) { 1.0 } else { 0.0 };

        if (((((scratch.values[1320] != 0.0) && (scratch.values[1337] != 0.0)) && (!(scratch.values[1971] != 0.0))) && (!(scratch.values[1976] != 0.0))) && (scratch.values[1977] != 0.0)) {
            scratch.store_ad(1372, &AdValue::div_from_scalar(1.0, AdValue::offset(AdValue::mul(scratch.ad_value(1366), scratch.ad_value(1371)), 1.0)));
        }

        if (((((scratch.values[1320] != 0.0) && (scratch.values[1337] != 0.0)) && (!(scratch.values[1971] != 0.0))) && (!(scratch.values[1976] != 0.0))) && (!(scratch.values[1977] != 0.0))) {
            scratch.store_ad(1372, &AdValue::pow(AdValue::offset(AdValue::mul(scratch.ad_value(1366), scratch.ad_value(1371)), 1.0), AdValue::mul(AdValue::neg(scratch.ad_value(551)), scratch.ad_value(612))));
        }

        if ((((scratch.values[1320] != 0.0) && (scratch.values[1337] != 0.0)) && (!(scratch.values[1971] != 0.0))) && (!(scratch.values[1976] != 0.0))) {
            scratch.store_ad(1373, &AdValue::div(AdValue::mul(scratch.ad_value(1362), scratch.ad_value(1372)), AdValue::add(scratch.ad_value(1362), scratch.ad_value(1372))));
        }

        if ((((scratch.values[1320] != 0.0) && (scratch.values[1337] != 0.0)) && (!(scratch.values[1971] != 0.0))) && (!(scratch.values[1976] != 0.0))) {
            scratch.store_ad(1374, &AdValue::sqrt(AdValue::scale(AdValue::div(scratch.ad_value(1366), scratch.ad_value(1370)), 0.375)));
        }

        if ((((scratch.values[1320] != 0.0) && (scratch.values[1337] != 0.0)) && (!(scratch.values[1971] != 0.0))) && (!(scratch.values[1976] != 0.0))) {
            scratch.store_ad(1375, &AdValue::sub(AdValue::scale(AdValue::mul(scratch.ad_value(1367), scratch.ad_value(1370)), 2.0), scratch.ad_value(1369)));
        }

    }

    pub(super) fn stamp_transient_block_29(
        &mut self,
        ctx: &GeneratedEvalContext<'_>,
        stamper: &mut GeneratedStamper<'_>,
        scratch: &mut Scratch,
    ) {
        let _ = stamper;
        if ((((scratch.values[1320] != 0.0) && (scratch.values[1337] != 0.0)) && (!(scratch.values[1971] != 0.0))) && (!(scratch.values[1976] != 0.0))) {
            scratch.store_ad(1376, &AdValue::add(AdValue::sub(AdValue::mul(AdValue::mul(scratch.ad_value(636), scratch.ad_value(1367)), scratch.ad_value(1370)), AdValue::mul(scratch.ad_value(636), scratch.ad_value(1369))), AdValue::scale(AdValue::mul(scratch.ad_value(1366), scratch.ad_value(1371)), 0.5)));
        }

        if ((((scratch.values[1320] != 0.0) && (scratch.values[1337] != 0.0)) && (!(scratch.values[1971] != 0.0))) && (!(scratch.values[1976] != 0.0))) {
            scratch.store_ad(1377, &AdValue::mul(AdValue::offset(scratch.ad_value(1375), (-1.0)), scratch.ad_value(1374)));
        }

        if ((((scratch.values[1320] != 0.0) && (scratch.values[1337] != 0.0)) && (!(scratch.values[1971] != 0.0))) && (!(scratch.values[1976] != 0.0))) {
            scratch.store_ad(1338, &AdValue::square(scratch.ad_value(1377)));
        }

        scratch.values[1978] = if (scratch.values[1377] > 0.0) { 1.0 } else { 0.0 };

        if (((((scratch.values[1320] != 0.0) && (scratch.values[1337] != 0.0)) && (!(scratch.values[1971] != 0.0))) && (!(scratch.values[1976] != 0.0))) && (scratch.values[1978] != 0.0)) {
            scratch.store_ad(1339, &AdValue::div_from_scalar(1.0, AdValue::offset(AdValue::scale(scratch.ad_value(1377), scratch.values[428]), 1.0)));
        }

        if (((((scratch.values[1320] != 0.0) && (scratch.values[1337] != 0.0)) && (!(scratch.values[1971] != 0.0))) && (!(scratch.values[1976] != 0.0))) && (!(scratch.values[1978] != 0.0))) {
            scratch.store_ad(1339, &AdValue::div_from_scalar(1.0, AdValue::sub_from_scalar(1.0, AdValue::scale(scratch.ad_value(1377), scratch.values[428]))));
        }

        scratch.values[1979] = if (((-scratch.values[1338]) + scratch.values[1376]) > (-230.25850929940458)) { 1.0 } else { 0.0 };

        if (((((scratch.values[1320] != 0.0) && (scratch.values[1337] != 0.0)) && (!(scratch.values[1971] != 0.0))) && (!(scratch.values[1976] != 0.0))) && (scratch.values[1979] != 0.0)) {
            scratch.store_ad(1356, &AdValue::exp(AdValue::sub(scratch.ad_value(1376), scratch.ad_value(1338))));
        }

        if (((((scratch.values[1320] != 0.0) && (scratch.values[1337] != 0.0)) && (!(scratch.values[1971] != 0.0))) && (!(scratch.values[1976] != 0.0))) && (!(scratch.values[1979] != 0.0))) {
            let assign38470_ad_e50873: AdValue = AdValue::div_from_scalar(1e-100, AdValue::offset(AdValue::mul(AdValue::sub_from_scalar((-230.25850929940458), AdValue::sub(scratch.ad_value(1376), scratch.ad_value(1338))), AdValue::offset(AdValue::scale(AdValue::mul(AdValue::sub_from_scalar((-230.25850929940458), AdValue::sub(scratch.ad_value(1376), scratch.ad_value(1338))), AdValue::offset(AdValue::scale(AdValue::sub_from_scalar((-230.25850929940458), AdValue::sub(scratch.ad_value(1376), scratch.ad_value(1338))), 0.3333333333333333), 1.0)), 0.5), 1.0)), 1.0));
            scratch.store_ad(1356, &assign38470_ad_e50873);
        }

        if ((((scratch.values[1320] != 0.0) && (scratch.values[1337] != 0.0)) && (!(scratch.values[1971] != 0.0))) && (!(scratch.values[1976] != 0.0))) {
            scratch.store_ad(1340, &AdValue::mul(AdValue::add(AdValue::add(AdValue::scale(scratch.ad_value(1339), 0.29214664), AdValue::scale(AdValue::square(scratch.ad_value(1339)), scratch.values[429])), AdValue::scale(AdValue::mul(AdValue::square(scratch.ad_value(1339)), scratch.ad_value(1339)), scratch.values[430])), scratch.ad_value(1356)));
        }

        scratch.values[1980] = if (scratch.values[1377] > 0.0) { 1.0 } else { 0.0 };

        if (((((scratch.values[1320] != 0.0) && (scratch.values[1337] != 0.0)) && (!(scratch.values[1971] != 0.0))) && (!(scratch.values[1976] != 0.0))) && (scratch.values[1980] != 0.0)) {
            scratch.values[1378] = scratch.values[1340];
            scratch.node_derivatives[1378] = scratch.node_derivatives[1340];
            scratch.branch_derivatives[1378] = scratch.branch_derivatives[1340];
        }

        scratch.values[1981] = if (scratch.values[1376] > (-230.25850929940458)) { 1.0 } else { 0.0 };

        if ((((((scratch.values[1320] != 0.0) && (scratch.values[1337] != 0.0)) && (!(scratch.values[1971] != 0.0))) && (!(scratch.values[1976] != 0.0))) && (!(scratch.values[1980] != 0.0))) && (scratch.values[1981] != 0.0)) {
            scratch.store_ad(1356, &AdValue::exp(scratch.ad_value(1376)));
        }

        if ((((((scratch.values[1320] != 0.0) && (scratch.values[1337] != 0.0)) && (!(scratch.values[1971] != 0.0))) && (!(scratch.values[1976] != 0.0))) && (!(scratch.values[1980] != 0.0))) && (!(scratch.values[1981] != 0.0))) {
            scratch.store_ad(1356, &AdValue::div_from_scalar(1e-100, AdValue::offset(AdValue::mul(AdValue::sub_from_scalar((-230.25850929940458), scratch.ad_value(1376)), AdValue::offset(AdValue::scale(AdValue::mul(AdValue::sub_from_scalar((-230.25850929940458), scratch.ad_value(1376)), AdValue::offset(AdValue::scale(AdValue::sub_from_scalar((-230.25850929940458), scratch.ad_value(1376)), 0.3333333333333333), 1.0)), 0.5), 1.0)), 1.0)));
        }

        if (((((scratch.values[1320] != 0.0) && (scratch.values[1337] != 0.0)) && (!(scratch.values[1971] != 0.0))) && (!(scratch.values[1976] != 0.0))) && (!(scratch.values[1980] != 0.0))) {
            scratch.store_ad(1378, &AdValue::sub(AdValue::scale(scratch.ad_value(1356), 2.0), scratch.ad_value(1340)));
        }

        if ((((scratch.values[1320] != 0.0) && (scratch.values[1337] != 0.0)) && (!(scratch.values[1971] != 0.0))) && (!(scratch.values[1976] != 0.0))) {
            scratch.store_ad(1379, &AdValue::scale(AdValue::div(AdValue::mul(scratch.ad_value(636), scratch.ad_value(1378)), scratch.ad_value(1374)), (1.772453850905516 * 0.5)));
        }

        if ((((scratch.values[1320] != 0.0) && (scratch.values[1337] != 0.0)) && (!(scratch.values[1971] != 0.0))) && (!(scratch.values[1976] != 0.0))) {
            scratch.store_ad(1365, &AdValue::mul(scratch.ad_value(565), AdValue::mul(AdValue::mul(scratch.ad_value(1364), scratch.ad_value(1379)), scratch.ad_value(1373))));
        }

        scratch.values[1982] = if (scratch.values[571] == 0.0) { 1.0 } else { 0.0 };

        if ((((scratch.values[1320] != 0.0) && (scratch.values[1337] != 0.0)) && (!(scratch.values[1971] != 0.0))) && (scratch.values[1982] != 0.0)) {
            scratch.values[1380] = 0.0;
            scratch.node_derivatives[1380] = [0.0; Instance::NODE_COUNT];
            scratch.branch_derivatives[1380] = [0.0; Instance::BRANCH_COUNT];
        }

        scratch.values[1983] = if (scratch.values[551] == 0.5) { 1.0 } else { 0.0 };

        if (((((scratch.values[1320] != 0.0) && (scratch.values[1337] != 0.0)) && (!(scratch.values[1971] != 0.0))) && (!(scratch.values[1982] != 0.0))) && (scratch.values[1983] != 0.0)) {
            scratch.store_ad(1356, &AdValue::sqrt(AdValue::mul(AdValue::sub(scratch.ad_value(548), scratch.ad_value(1354)), scratch.ad_value(630))));
        }

        if (((((scratch.values[1320] != 0.0) && (scratch.values[1337] != 0.0)) && (!(scratch.values[1971] != 0.0))) && (!(scratch.values[1982] != 0.0))) && (!(scratch.values[1983] != 0.0))) {
            scratch.store_ad(1356, &AdValue::pow(AdValue::mul(AdValue::sub(scratch.ad_value(548), scratch.ad_value(1354)), scratch.ad_value(630)), scratch.ad_value(551)));
        }

        if ((((scratch.values[1320] != 0.0) && (scratch.values[1337] != 0.0)) && (!(scratch.values[1971] != 0.0))) && (!(scratch.values[1982] != 0.0))) {
            scratch.store_ad(1381, &AdValue::mul(scratch.ad_value(612), AdValue::div(AdValue::mul(AdValue::sub(scratch.ad_value(548), scratch.ad_value(1354)), scratch.ad_value(627)), scratch.ad_value(1356))));
        }

        scratch.values[1984] = if (((((-scratch.values[642]) / scratch.values[1381])) as f64).abs() < 230.25850929940458) { 1.0 } else { 0.0 };

        if (((((scratch.values[1320] != 0.0) && (scratch.values[1337] != 0.0)) && (!(scratch.values[1971] != 0.0))) && (!(scratch.values[1982] != 0.0))) && (scratch.values[1984] != 0.0)) {
            scratch.store_ad(1356, &AdValue::exp(AdValue::div(AdValue::neg(scratch.ad_value(642)), scratch.ad_value(1381))));
        }

        scratch.values[1985] = if (((-scratch.values[642]) / scratch.values[1381]) < 0.0) { 1.0 } else { 0.0 };

        if ((((((scratch.values[1320] != 0.0) && (scratch.values[1337] != 0.0)) && (!(scratch.values[1971] != 0.0))) && (!(scratch.values[1982] != 0.0))) && (!(scratch.values[1984] != 0.0))) && (scratch.values[1985] != 0.0)) {
            let assign38660_ad_e51200: AdValue = AdValue::offset(AdValue::mul(AdValue::sub_from_scalar((-230.25850929940458), AdValue::div(AdValue::neg(scratch.ad_value(642)), scratch.ad_value(1381))), AdValue::offset(AdValue::scale(AdValue::mul(AdValue::sub_from_scalar((-230.25850929940458), AdValue::div(AdValue::neg(scratch.ad_value(642)), scratch.ad_value(1381))), AdValue::offset(AdValue::scale(AdValue::sub_from_scalar((-230.25850929940458), AdValue::div(AdValue::neg(scratch.ad_value(642)), scratch.ad_value(1381))), 0.3333333333333333), 1.0)), 0.5), 1.0)), 1.0);
            scratch.store_ad(1356, &AdValue::div_from_scalar(1e-100, assign38660_ad_e51200));
        }

        if ((((((scratch.values[1320] != 0.0) && (scratch.values[1337] != 0.0)) && (!(scratch.values[1971] != 0.0))) && (!(scratch.values[1982] != 0.0))) && (!(scratch.values[1984] != 0.0))) && (!(scratch.values[1985] != 0.0))) {
            let assign38670_ad_e51250: AdValue = AdValue::scale(AdValue::offset(AdValue::mul(AdValue::offset(AdValue::div(AdValue::neg(scratch.ad_value(642)), scratch.ad_value(1381)), (-230.25850929940458)), AdValue::offset(AdValue::scale(AdValue::mul(AdValue::offset(AdValue::div(AdValue::neg(scratch.ad_value(642)), scratch.ad_value(1381)), (-230.25850929940458)), AdValue::offset(AdValue::scale(AdValue::offset(AdValue::div(AdValue::neg(scratch.ad_value(642)), scratch.ad_value(1381)), (-230.25850929940458)), 0.3333333333333333), 1.0)), 0.5), 1.0)), 1.0), 1e100);
            scratch.store_ad(1356, &assign38670_ad_e51250);
        }

        if ((((scratch.values[1320] != 0.0) && (scratch.values[1337] != 0.0)) && (!(scratch.values[1971] != 0.0))) && (!(scratch.values[1982] != 0.0))) {
            scratch.store_ad(1380, &AdValue::mul(scratch.ad_value(571), AdValue::mul(AdValue::mul(AdValue::mul(scratch.ad_value(527), scratch.ad_value(1381)), scratch.ad_value(1381)), scratch.ad_value(1356))));
        }

        scratch.values[1986] = if (scratch.values[580] > 1000.0) { 1.0 } else { 0.0 };

        if ((((scratch.values[1320] != 0.0) && (scratch.values[1337] != 0.0)) && (!(scratch.values[1971] != 0.0))) && (scratch.values[1986] != 0.0)) {
            scratch.values[1382] = 1.0;
            scratch.node_derivatives[1382] = [0.0; Instance::NODE_COUNT];
            scratch.branch_derivatives[1382] = [0.0; Instance::BRANCH_COUNT];
        }

        scratch.values[1987] = if (scratch.values[1355] > ((-scratch.values[500]) * scratch.values[580])) { 1.0 } else { 0.0 };

        scratch.values[1988] = if (scratch.values[583] == 4.0) { 1.0 } else { 0.0 };

        if ((((((scratch.values[1320] != 0.0) && (scratch.values[1337] != 0.0)) && (!(scratch.values[1971] != 0.0))) && (!(scratch.values[1986] != 0.0))) && (scratch.values[1987] != 0.0)) && (scratch.values[1988] != 0.0)) {
            scratch.store_ad(1356, &AdValue::mul(AdValue::mul(AdValue::mul(AdValue::mul(scratch.ad_value(1355), scratch.ad_value(648)), AdValue::mul(scratch.ad_value(1355), scratch.ad_value(648))), AdValue::mul(scratch.ad_value(1355), scratch.ad_value(648))), AdValue::mul(scratch.ad_value(1355), scratch.ad_value(648))));
        }

        if ((((((scratch.values[1320] != 0.0) && (scratch.values[1337] != 0.0)) && (!(scratch.values[1971] != 0.0))) && (!(scratch.values[1986] != 0.0))) && (scratch.values[1987] != 0.0)) && (!(scratch.values[1988] != 0.0))) {
            scratch.store_ad(1356, &AdValue::pow(AdValue::abs(AdValue::mul(scratch.ad_value(1355), scratch.ad_value(648))), scratch.ad_value(583)));
        }

        if (((((scratch.values[1320] != 0.0) && (scratch.values[1337] != 0.0)) && (!(scratch.values[1971] != 0.0))) && (!(scratch.values[1986] != 0.0))) && (scratch.values[1987] != 0.0)) {
            scratch.store_ad(1382, &AdValue::div_from_scalar(1.0, AdValue::sub_from_scalar(1.0, scratch.ad_value(1356))));
        }

        if (((((scratch.values[1320] != 0.0) && (scratch.values[1337] != 0.0)) && (!(scratch.values[1971] != 0.0))) && (!(scratch.values[1986] != 0.0))) && (!(scratch.values[1987] != 0.0))) {
            scratch.store_ad(1382, &AdValue::add(scratch.ad_value(645), AdValue::mul(AdValue::add(scratch.ad_value(1355), AdValue::scale(scratch.ad_value(580), scratch.values[500])), scratch.ad_value(651))));
        }

        if (((scratch.values[1320] != 0.0) && (scratch.values[1337] != 0.0)) && (!(scratch.values[1971] != 0.0))) {
            scratch.store_ad(1385, &AdValue::mul(AdValue::add(AdValue::add(AdValue::add(scratch.ad_value(1357), scratch.ad_value(1358)), scratch.ad_value(1365)), scratch.ad_value(1380)), scratch.ad_value(1382)));
        }

        if ((scratch.values[1320] != 0.0) && (scratch.values[1337] != 0.0)) {
            scratch.store_ad(517, &AdValue::add(AdValue::add(AdValue::mul(scratch.ad_value(724), scratch.ad_value(1383)), AdValue::mul(scratch.ad_value(725), scratch.ad_value(1384))), AdValue::mul(scratch.ad_value(726), scratch.ad_value(1385))));
        }

        if ((scratch.values[1320] != 0.0) && (scratch.values[1337] != 0.0)) {
            scratch.store_ad(745, &AdValue::add(AdValue::add(AdValue::mul(scratch.ad_value(724), scratch.ad_value(595)), AdValue::mul(scratch.ad_value(725), scratch.ad_value(596))), AdValue::mul(scratch.ad_value(726), scratch.ad_value(597))));
        }

        if ((scratch.values[1320] != 0.0) && (scratch.values[1337] != 0.0)) {
            scratch.store_ad(521, &AdValue::sub(scratch.ad_value(516), AdValue::mul(scratch.ad_value(745), AdValue::offset(AdValue::exp(AdValue::scale(scratch.ad_value(526), (scratch.values[427] * scratch.values[746]))), (-1.0)))));
        }

        if ((scratch.values[1320] != 0.0) && (scratch.values[1337] != 0.0)) {
            scratch.store_ad(522, &AdValue::sub(scratch.ad_value(517), AdValue::mul(scratch.ad_value(745), AdValue::offset(AdValue::exp(AdValue::scale(scratch.ad_value(527), (scratch.values[427] * scratch.values[746]))), (-1.0)))));
        }

        scratch.values[1989] = if !(((scratch.values[724] == 0.0) && (scratch.values[725] == 0.0)) && (scratch.values[726] == 0.0)) { 1.0 } else { 0.0 };

        scratch.values[1990] = if ((scratch.values[516] > 0.0) && (scratch.values[517] > 0.0)) { 1.0 } else { 0.0 };

        scratch.values[1991] = if (((((scratch.values[521] / scratch.values[516]) > 0.001) || ((scratch.values[522] / scratch.values[517]) > 0.001)) && (scratch.values[521] > 0.0)) && (scratch.values[522] > 0.0)) { 1.0 } else { 0.0 };

        if (((((scratch.values[1320] != 0.0) && (scratch.values[1337] != 0.0)) && (scratch.values[1989] != 0.0)) && (scratch.values[1990] != 0.0)) && (scratch.values[1991] != 0.0)) {
            scratch.store_ad(528, &AdValue::div(scratch.ad_value(521), scratch.ad_value(522)));
        }

        if (((((scratch.values[1320] != 0.0) && (scratch.values[1337] != 0.0)) && (scratch.values[1989] != 0.0)) && (scratch.values[1990] != 0.0)) && (scratch.values[1991] != 0.0)) {
            scratch.store_ad(748, &AdValue::div(AdValue::scale(AdValue::ln(scratch.ad_value(528)), scratch.values[426]), AdValue::sub(scratch.ad_value(526), scratch.ad_value(527))));
        }

        if (((((scratch.values[1320] != 0.0) && (scratch.values[1337] != 0.0)) && (scratch.values[1989] != 0.0)) && (scratch.values[1990] != 0.0)) && (scratch.values[1991] != 0.0)) {
            scratch.store_ad(747, &AdValue::div(scratch.ad_value(521), AdValue::offset(AdValue::exp(AdValue::mul(AdValue::scale(scratch.ad_value(526), scratch.values[427]), scratch.ad_value(748))), (-1.0))));
        }

        if (((scratch.values[1320] != 0.0) && (scratch.values[1337] != 0.0)) && (scratch.values[1989] != 0.0)) {
            scratch.store_ad(518, &AdValue::sub(AdValue::sub(scratch.ad_value(513), AdValue::mul(scratch.ad_value(745), AdValue::offset(AdValue::exp(AdValue::scale(scratch.ad_value(523), (scratch.values[427] * scratch.values[746]))), (-1.0)))), AdValue::mul(scratch.ad_value(747), AdValue::offset(AdValue::exp(AdValue::mul(AdValue::scale(scratch.ad_value(523), scratch.values[427]), scratch.ad_value(748))), (-1.0)))));
        }

        if (((scratch.values[1320] != 0.0) && (scratch.values[1337] != 0.0)) && (scratch.values[1989] != 0.0)) {
            scratch.store_ad(519, &AdValue::sub(AdValue::sub(scratch.ad_value(514), AdValue::mul(scratch.ad_value(745), AdValue::offset(AdValue::exp(AdValue::scale(scratch.ad_value(524), (scratch.values[427] * scratch.values[746]))), (-1.0)))), AdValue::mul(scratch.ad_value(747), AdValue::offset(AdValue::exp(AdValue::mul(AdValue::scale(scratch.ad_value(524), scratch.values[427]), scratch.ad_value(748))), (-1.0)))));
        }

        if (((scratch.values[1320] != 0.0) && (scratch.values[1337] != 0.0)) && (scratch.values[1989] != 0.0)) {
            scratch.store_ad(520, &AdValue::sub(AdValue::sub(scratch.ad_value(515), AdValue::mul(scratch.ad_value(745), AdValue::offset(AdValue::exp(AdValue::scale(scratch.ad_value(525), (scratch.values[427] * scratch.values[746]))), (-1.0)))), AdValue::mul(scratch.ad_value(747), AdValue::offset(AdValue::exp(AdValue::mul(AdValue::scale(scratch.ad_value(525), scratch.values[427]), scratch.ad_value(748))), (-1.0)))));
        }

        scratch.values[1992] = if (((scratch.values[513] < 0.0) && (scratch.values[514] < 0.0)) && (scratch.values[515] < 0.0)) { 1.0 } else { 0.0 };

        scratch.values[1993] = if (((((((scratch.values[518] / scratch.values[513]) > 0.001) || ((scratch.values[519] / scratch.values[514]) > 0.001)) || ((scratch.values[520] / scratch.values[515]) > 0.001)) && (scratch.values[518] < 0.0)) && (scratch.values[519] < 0.0)) && (scratch.values[520] < 0.0)) { 1.0 } else { 0.0 };

        if (((((scratch.values[1320] != 0.0) && (scratch.values[1337] != 0.0)) && (scratch.values[1989] != 0.0)) && (scratch.values[1992] != 0.0)) && (scratch.values[1993] != 0.0)) {
            scratch.store_ad(528, &AdValue::div(scratch.ad_value(518), scratch.ad_value(519)));
        }

        if (((((scratch.values[1320] != 0.0) && (scratch.values[1337] != 0.0)) && (scratch.values[1989] != 0.0)) && (scratch.values[1992] != 0.0)) && (scratch.values[1993] != 0.0)) {
            scratch.store_ad(529, &AdValue::div(AdValue::scale(AdValue::ln(scratch.ad_value(528)), (-scratch.values[426])), AdValue::sub(scratch.ad_value(523), scratch.ad_value(524))));
        }

        if (((((scratch.values[1320] != 0.0) && (scratch.values[1337] != 0.0)) && (scratch.values[1989] != 0.0)) && (scratch.values[1992] != 0.0)) && (scratch.values[1993] != 0.0)) {
            scratch.store_ad(531, &AdValue::div(scratch.ad_value(524), AdValue::sub(scratch.ad_value(524), scratch.ad_value(523))));
        }

        if (((((scratch.values[1320] != 0.0) && (scratch.values[1337] != 0.0)) && (scratch.values[1989] != 0.0)) && (scratch.values[1992] != 0.0)) && (scratch.values[1993] != 0.0)) {
            scratch.store_ad(532, &AdValue::mul(AdValue::scale(AdValue::offset(scratch.ad_value(528), (-1.0)), scratch.values[426]), AdValue::offset(AdValue::pow(scratch.ad_value(528), scratch.ad_value(531)), (-1.0))));
        }

        if (((((scratch.values[1320] != 0.0) && (scratch.values[1337] != 0.0)) && (scratch.values[1989] != 0.0)) && (scratch.values[1992] != 0.0)) && (scratch.values[1993] != 0.0)) {
            scratch.store_ad(531, &AdValue::div(scratch.ad_value(523), AdValue::sub(scratch.ad_value(523), scratch.ad_value(524))));
        }

        if (((((scratch.values[1320] != 0.0) && (scratch.values[1337] != 0.0)) && (scratch.values[1989] != 0.0)) && (scratch.values[1992] != 0.0)) && (scratch.values[1993] != 0.0)) {
            scratch.store_ad(533, &AdValue::sub(AdValue::add(AdValue::mul(AdValue::pow(scratch.ad_value(528), scratch.ad_value(531)), AdValue::sub(scratch.ad_value(524), scratch.ad_value(523))), AdValue::mul(scratch.ad_value(528), scratch.ad_value(523))), scratch.ad_value(524)));
        }

        if (((((scratch.values[1320] != 0.0) && (scratch.values[1337] != 0.0)) && (scratch.values[1989] != 0.0)) && (scratch.values[1992] != 0.0)) && (scratch.values[1993] != 0.0)) {
            scratch.store_ad(530, &AdValue::div(scratch.ad_value(532), scratch.ad_value(533)));
        }

        if (((((scratch.values[1320] != 0.0) && (scratch.values[1337] != 0.0)) && (scratch.values[1989] != 0.0)) && (scratch.values[1992] != 0.0)) && (scratch.values[1993] != 0.0)) {
            scratch.store_ad(750, &AdValue::add(scratch.ad_value(529), scratch.ad_value(530)));
        }

        scratch.values[1994] = if (((((scratch.values[525] * scratch.values[427]) * scratch.values[750])) as f64).abs() < 1e-6) { 1.0 } else { 0.0 };

        if ((((((scratch.values[1320] != 0.0) && (scratch.values[1337] != 0.0)) && (scratch.values[1989] != 0.0)) && (scratch.values[1992] != 0.0)) && (scratch.values[1993] != 0.0)) && (scratch.values[1994] != 0.0)) {
            scratch.values[744] = 1.0;
            scratch.node_derivatives[744] = [0.0; Instance::NODE_COUNT];
            scratch.branch_derivatives[744] = [0.0; Instance::BRANCH_COUNT];
        }

        if ((((((scratch.values[1320] != 0.0) && (scratch.values[1337] != 0.0)) && (scratch.values[1989] != 0.0)) && (scratch.values[1992] != 0.0)) && (scratch.values[1993] != 0.0)) && (scratch.values[1994] != 0.0)) {
            scratch.store_ad(749, &AdValue::mul(scratch.ad_value(520), AdValue::add(AdValue::div_from_scalar(1.0, scratch.ad_value(525)), AdValue::scale(scratch.ad_value(750), (0.5 * scratch.values[427])))));
        }

        if ((((((scratch.values[1320] != 0.0) && (scratch.values[1337] != 0.0)) && (scratch.values[1989] != 0.0)) && (scratch.values[1992] != 0.0)) && (scratch.values[1993] != 0.0)) && (scratch.values[1994] != 0.0)) {
            scratch.store_ad(750, &AdValue::div(AdValue::scale(AdValue::mul(AdValue::scale(scratch.ad_value(520), (-0.5)), scratch.ad_value(750)), scratch.values[427]), scratch.ad_value(525)));
        }

        if ((((((scratch.values[1320] != 0.0) && (scratch.values[1337] != 0.0)) && (scratch.values[1989] != 0.0)) && (scratch.values[1992] != 0.0)) && (scratch.values[1993] != 0.0)) && (!(scratch.values[1994] != 0.0))) {
            scratch.values[744] = 0.0;
            scratch.node_derivatives[744] = [0.0; Instance::NODE_COUNT];
            scratch.branch_derivatives[744] = [0.0; Instance::BRANCH_COUNT];
        }

        if ((((((scratch.values[1320] != 0.0) && (scratch.values[1337] != 0.0)) && (scratch.values[1989] != 0.0)) && (scratch.values[1992] != 0.0)) && (scratch.values[1993] != 0.0)) && (!(scratch.values[1994] != 0.0))) {
            scratch.store_ad(749, &AdValue::div(AdValue::neg(scratch.ad_value(520)), AdValue::offset(AdValue::exp(AdValue::mul(AdValue::scale(AdValue::neg(scratch.ad_value(525)), scratch.values[427]), scratch.ad_value(750))), (-1.0))));
        }

        if ((scratch.values[1320] != 0.0) && (scratch.values[1337] != 0.0)) {
            scratch.store_ad(539, &AdValue::mul(scratch.ad_value(585), AdValue::add(AdValue::add(AdValue::mul(scratch.ad_value(724), scratch.ad_value(613)), AdValue::mul(scratch.ad_value(725), scratch.ad_value(614))), AdValue::mul(scratch.ad_value(726), scratch.ad_value(615)))));
        }

        scratch.values[1995] = if ((scratch.values[724] * scratch.values[613]) <= scratch.values[539]) { 1.0 } else { 0.0 };

        if (((scratch.values[1320] != 0.0) && (scratch.values[1337] != 0.0)) && (scratch.values[1995] != 0.0)) {
            scratch.values[729] = 0.0;
            scratch.node_derivatives[729] = [0.0; Instance::NODE_COUNT];
            scratch.branch_derivatives[729] = [0.0; Instance::BRANCH_COUNT];
        }

        scratch.values[1996] = if ((scratch.values[725] * scratch.values[614]) <= scratch.values[539]) { 1.0 } else { 0.0 };

        if (((scratch.values[1320] != 0.0) && (scratch.values[1337] != 0.0)) && (scratch.values[1996] != 0.0)) {
            scratch.values[730] = 0.0;
            scratch.node_derivatives[730] = [0.0; Instance::NODE_COUNT];
            scratch.branch_derivatives[730] = [0.0; Instance::BRANCH_COUNT];
        }

        scratch.values[1997] = if ((scratch.values[726] * scratch.values[615]) <= scratch.values[539]) { 1.0 } else { 0.0 };

        if (((scratch.values[1320] != 0.0) && (scratch.values[1337] != 0.0)) && (scratch.values[1997] != 0.0)) {
            scratch.values[731] = 0.0;
            scratch.node_derivatives[731] = [0.0; Instance::NODE_COUNT];
            scratch.branch_derivatives[731] = [0.0; Instance::BRANCH_COUNT];
        }

        scratch.values[1998] = if !(((scratch.values[724] == 0.0) && (scratch.values[725] == 0.0)) && (scratch.values[726] == 0.0)) { 1.0 } else { 0.0 };

        if (((scratch.values[1320] != 0.0) && (scratch.values[1337] != 0.0)) && (scratch.values[1998] != 0.0)) {
            scratch.store_ad(738, &AdValue::ln(AdValue::div_from_scalar((0.5 * scratch.values[374]), AdValue::offset(scratch.ad_value(745), 1e-21))));
        }

        if (((scratch.values[1320] != 0.0) && (scratch.values[1337] != 0.0)) && (scratch.values[1998] != 0.0)) {
            scratch.store_ad(740, &AdValue::ln(AdValue::div_from_scalar((0.5 * scratch.values[374]), AdValue::offset(scratch.ad_value(747), 1e-21))));
        }

        if (((scratch.values[1320] != 0.0) && (scratch.values[1337] != 0.0)) && (scratch.values[1998] != 0.0)) {
            scratch.store_ad(742, &AdValue::ln(AdValue::div_from_scalar((0.5 * scratch.values[374]), AdValue::offset(AdValue::abs(scratch.ad_value(749)), 1e-21))));
        }

        if ((scratch.values[1320] != 0.0) && (scratch.values[1337] != 0.0)) {
            scratch.store_ad(738, &AdValue::min_with_scalar(scratch.ad_value(738), 230.25850929940458));
        }

        if ((scratch.values[1320] != 0.0) && (scratch.values[1337] != 0.0)) {
            scratch.store_ad(739, &AdValue::exp(scratch.ad_value(738)));
        }

        if ((scratch.values[1320] != 0.0) && (scratch.values[1337] != 0.0)) {
            scratch.store_ad(740, &AdValue::min_with_scalar(scratch.ad_value(740), 230.25850929940458));
        }

        if ((scratch.values[1320] != 0.0) && (scratch.values[1337] != 0.0)) {
            scratch.store_ad(741, &AdValue::exp(scratch.ad_value(740)));
        }

        if ((scratch.values[1320] != 0.0) && (scratch.values[1337] != 0.0)) {
            scratch.store_ad(742, &AdValue::min_with_scalar(scratch.ad_value(742), 230.25850929940458));
        }

        if ((scratch.values[1320] != 0.0) && (scratch.values[1337] != 0.0)) {
            scratch.store_ad(743, &AdValue::exp(scratch.ad_value(742)));
        }

        scratch.values[2086] = 0.0;

        scratch.values[2087] = 0.0;

        scratch.values[2088] = 0.0;

        scratch.store_ad(365, &AdValue::offset(AdValue::voltage(ctx, &self.nodes, Some(4), None), scratch.values[360]));

        scratch.store_ad(366, &AdValue::square(scratch.ad_value(365)));

        scratch.store_ad(367, &AdValue::offset(scratch.ad_value(365), (-scratch.values[359])));

        scratch.store_ad(368, &AdValue::div_from_scalar(scratch.values[359], scratch.ad_value(365)));

        scratch.store_ad(369, &AdValue::ln(scratch.ad_value(368)));

        scratch.store_ad(2074, &AdValue::scale(scratch.ad_value(365), (1.3806505e-23 * 6.241449993689894e18)));

        scratch.store_ad(370, &AdValue::div_from_scalar(1.0, scratch.ad_value(2074)));

        scratch.store_ad(371, &AdValue::sub(AdValue::sub_from_scalar(1.179, AdValue::scale(scratch.ad_value(365), 9.025e-5)), AdValue::scale(scratch.ad_value(366), 3.05e-7)));

        scratch.store_ad(372, &AdValue::scale(AdValue::mul(AdValue::mul(AdValue::offset(AdValue::scale(scratch.ad_value(365), 0.00045), 1.045), AdValue::sub(AdValue::offset(AdValue::scale(scratch.ad_value(365), 0.0014), 0.523), AdValue::scale(scratch.ad_value(366), 1.48e-6))), scratch.ad_value(366)), 1.1111111111111112e-5));

        if !(scratch.values[372] > 0.001) {
            scratch.values[372] = 0.001;
            scratch.node_derivatives[372] = [0.0; Instance::NODE_COUNT];
            scratch.branch_derivatives[372] = [0.0; Instance::BRANCH_COUNT];
        }

        scratch.store_ad(2077, &AdValue::scale(scratch.ad_value(365), (4.0 * 1.3806505e-23)));

        scratch.store_ad(767, &AdValue::add(AdValue::add(scratch.ad_value(371), scratch.ad_value(206)), AdValue::mul(AdValue::scale(scratch.ad_value(2074), 2.0), AdValue::ln(AdValue::scale(AdValue::mul(scratch.ad_value(198), AdValue::powf(scratch.ad_value(372), (-0.75))), 4e-26)))));

        if !(scratch.values[767] > 0.05) {
            scratch.values[767] = 0.05;
            scratch.node_derivatives[767] = [0.0; Instance::NODE_COUNT];
            scratch.branch_derivatives[767] = [0.0; Instance::BRANCH_COUNT];
        }

        scratch.store_ad(768, &AdValue::div(AdValue::sqrt(AdValue::mul(AdValue::scale(scratch.ad_value(198), ((2.0 * 1.6021918e-19) * scratch.values[806])), scratch.ad_value(370))), scratch.ad_value(808)));

        scratch.values[769] = 0.0;

        scratch.values[770] = 0.0;

        scratch.values[2089] = if (scratch.values[208] > 0.0) { 1.0 } else { 0.0 };

        if (scratch.values[2089] != 0.0) {
            scratch.store_ad(771, &AdValue::div_from_scalar(80000000.0, scratch.ad_value(809)));
        }

        if (scratch.values[2089] != 0.0) {
            scratch.store_ad(770, &{
                if (scratch.values[208] > scratch.values[771]) {
                    scratch.ad_value(208)
                } else {
                    scratch.ad_value(771)
                }
            });
        }

        if (scratch.values[2089] != 0.0) {
            scratch.store_ad(770, &{
                if (5e24 > scratch.values[770]) {
                    AdValue::constant(5e24)
                } else {
                    scratch.ad_value(770)
                }
            });
        }

        if (scratch.values[2089] != 0.0) {
            scratch.store_ad(769, &AdValue::div(AdValue::mul(AdValue::mul(AdValue::scale(scratch.ad_value(808), 2.0), scratch.ad_value(808)), scratch.ad_value(2074)), AdValue::scale(scratch.ad_value(770), (1.6021918e-19 * scratch.values[806]))));
        }

        scratch.store_ad(772, &AdValue::mul(AdValue::scale(scratch.ad_value(2074), 100.0), scratch.ad_value(2074)));

        scratch.values[2090] = if (scratch.values[191] > 0.0) { 1.0 } else { 0.0 };

        if (scratch.values[2090] != 0.0) {
            scratch.store_ad(773, &AdValue::sqrt(AdValue::mul(AdValue::mul(AdValue::mul(scratch.ad_value(2074), scratch.ad_value(768)), scratch.ad_value(768)), scratch.ad_value(767))));
        }

        if (scratch.values[2090] != 0.0) {
            scratch.store_ad(774, &AdValue::mul(AdValue::scale(scratch.ad_value(812), 0.75), AdValue::powf(scratch.ad_value(773), 0.6666666666666666)));
        }

        if (scratch.values[2090] != 0.0) {
            scratch.store_ad(767, &AdValue::add(scratch.ad_value(767), scratch.ad_value(774)));
        }

        if (scratch.values[2090] != 0.0) {
            scratch.store_ad(768, &AdValue::mul(scratch.ad_value(768), AdValue::offset(AdValue::div(AdValue::scale(scratch.ad_value(774), (2.0 * 0.6666666666666666)), scratch.ad_value(773)), 1.0)));
        }

        scratch.store_ad(775, &AdValue::sqrt(scratch.ad_value(767)));

        scratch.store_ad(776, &AdValue::scale(scratch.ad_value(767), 0.95));

        scratch.store_ad(777, &AdValue::mul(AdValue::scale(scratch.ad_value(767), 0.0025), scratch.ad_value(767)));

        scratch.values[778] = scratch.values[777];
        scratch.node_derivatives[778] = scratch.node_derivatives[777];
        scratch.branch_derivatives[778] = scratch.branch_derivatives[777];

        scratch.store_ad(779, &AdValue::scale(AdValue::sqrt(scratch.ad_value(778)), 0.5));

        scratch.store_ad(780, &AdValue::scale(AdValue::sub(AdValue::sub(scratch.ad_value(776), scratch.ad_value(779)), AdValue::sqrt(AdValue::add(AdValue::mul(AdValue::sub(scratch.ad_value(776), scratch.ad_value(779)), AdValue::sub(scratch.ad_value(776), scratch.ad_value(779))), scratch.ad_value(777)))), 0.5));

        scratch.store_ad(781, &AdValue::scale(AdValue::add(scratch.ad_value(767), scratch.ad_value(371)), 0.5));

        scratch.store_ad(782, &AdValue::sub(AdValue::sqrt(AdValue::add(scratch.ad_value(201), scratch.ad_value(767))), scratch.ad_value(775)));

        scratch.store_ad(783, &AdValue::sub(AdValue::sub(AdValue::sqrt(AdValue::add(AdValue::add(scratch.ad_value(201), scratch.ad_value(202)), scratch.ad_value(767))), scratch.ad_value(775)), scratch.ad_value(782)));

        scratch.store_ad(784, &AdValue::add(AdValue::add(AdValue::add(scratch.ad_value(371), scratch.ad_value(206)), scratch.ad_value(207)), AdValue::mul(AdValue::scale(scratch.ad_value(2074), 2.0), AdValue::ln(AdValue::scale(AdValue::mul(scratch.ad_value(811), AdValue::powf(scratch.ad_value(372), (-0.75))), 4e-26)))));

        if !(scratch.values[784] > 0.05) {
            scratch.values[784] = 0.05;
            scratch.node_derivatives[784] = [0.0; Instance::NODE_COUNT];
            scratch.branch_derivatives[784] = [0.0; Instance::BRANCH_COUNT];
        }

        scratch.store_ad(785, &AdValue::div(AdValue::sqrt(AdValue::mul(AdValue::scale(scratch.ad_value(811), ((2.0 * 1.6021918e-19) * scratch.values[806])), scratch.ad_value(370))), scratch.ad_value(808)));

        scratch.values[2091] = if (scratch.values[191] > 0.0) { 1.0 } else { 0.0 };

        if (scratch.values[2091] != 0.0) {
            scratch.store_ad(773, &AdValue::sqrt(AdValue::mul(AdValue::mul(AdValue::mul(scratch.ad_value(2074), scratch.ad_value(785)), scratch.ad_value(785)), scratch.ad_value(784))));
        }

        if (scratch.values[2091] != 0.0) {
            scratch.store_ad(774, &AdValue::mul(AdValue::scale(scratch.ad_value(812), 0.75), AdValue::powf(scratch.ad_value(773), 0.6666666666666666)));
        }

        if (scratch.values[2091] != 0.0) {
            scratch.store_ad(784, &AdValue::add(scratch.ad_value(784), scratch.ad_value(774)));
        }

        if (scratch.values[2091] != 0.0) {
            scratch.store_ad(785, &AdValue::mul(scratch.ad_value(785), AdValue::offset(AdValue::div(AdValue::scale(scratch.ad_value(774), (2.0 * 0.6666666666666666)), scratch.ad_value(773)), 1.0)));
        }

        scratch.store_ad(786, &AdValue::scale(scratch.ad_value(784), 0.95));

        scratch.store_ad(787, &AdValue::mul(AdValue::scale(scratch.ad_value(784), 0.0025), scratch.ad_value(784)));

        scratch.values[788] = scratch.values[787];
        scratch.node_derivatives[788] = scratch.node_derivatives[787];
        scratch.branch_derivatives[788] = scratch.branch_derivatives[787];

        scratch.store_ad(779, &AdValue::scale(AdValue::sqrt(scratch.ad_value(788)), 0.5));

        scratch.store_ad(789, &AdValue::scale(AdValue::sub(AdValue::sub(scratch.ad_value(786), scratch.ad_value(779)), AdValue::sqrt(AdValue::add(AdValue::mul(AdValue::sub(scratch.ad_value(786), scratch.ad_value(779)), AdValue::sub(scratch.ad_value(786), scratch.ad_value(779))), scratch.ad_value(787)))), 0.5));

        scratch.store_ad(751, &AdValue::offset(AdValue::add(scratch.ad_value(192), AdValue::mul(AdValue::mul(scratch.ad_value(193), scratch.ad_value(367)), AdValue::offset(AdValue::mul(scratch.ad_value(194), scratch.ad_value(367)), 1.0))), scratch.values[27]));

        scratch.store_ad(790, &AdValue::exp(AdValue::mul(scratch.ad_value(195), scratch.ad_value(369))));

        scratch.store_ad(752, &AdValue::mul(scratch.ad_value(209), scratch.ad_value(790)));

        scratch.store_ad(753, &AdValue::div(scratch.ad_value(210), scratch.ad_value(368)));

        scratch.store_ad(791, &AdValue::exp(AdValue::mul(scratch.ad_value(223), scratch.ad_value(369))));

        scratch.store_ad(754, &AdValue::mul(scratch.ad_value(222), scratch.ad_value(791)));

        scratch.store_ad(2075, &AdValue::mul(AdValue::scale(scratch.ad_value(754), scratch.values[26]), scratch.ad_value(808)));

        scratch.store_ad(756, &AdValue::mul(scratch.ad_value(226), AdValue::exp(AdValue::mul(scratch.ad_value(227), scratch.ad_value(369)))));

        scratch.store_ad(792, &AdValue::exp(AdValue::mul(scratch.ad_value(225), scratch.ad_value(369))));

        scratch.store_ad(755, &AdValue::mul(scratch.ad_value(224), scratch.ad_value(792)));

        scratch.store_ad(758, &AdValue::mul(scratch.ad_value(230), AdValue::exp(AdValue::mul(scratch.ad_value(231), scratch.ad_value(369)))));

        scratch.store_ad(793, &AdValue::exp(AdValue::mul(scratch.ad_value(229), scratch.ad_value(369))));

        scratch.store_ad(757, &AdValue::mul(scratch.ad_value(228), scratch.ad_value(793)));

        scratch.store_ad(794, &AdValue::exp(AdValue::mul(scratch.ad_value(233), scratch.ad_value(369))));

        scratch.store_ad(759, &AdValue::mul(scratch.ad_value(232), scratch.ad_value(794)));

        scratch.store_ad(795, &AdValue::exp(AdValue::mul(scratch.ad_value(236), scratch.ad_value(369))));

        scratch.store_ad(760, &AdValue::mul(scratch.ad_value(235), scratch.ad_value(795)));

        scratch.store_ad(796, &AdValue::mul(AdValue::scale(scratch.ad_value(2075), 2.0), scratch.ad_value(760)));

        scratch.store_ad(797, &AdValue::exp(AdValue::mul(scratch.ad_value(240), scratch.ad_value(369))));

    }

    pub(super) fn stamp_transient_block_30(
        &mut self,
        ctx: &GeneratedEvalContext<'_>,
        stamper: &mut GeneratedStamper<'_>,
        scratch: &mut Scratch,
    ) {
        let _ = stamper;
        scratch.store_ad(2079, &AdValue::mul(scratch.ad_value(239), scratch.ad_value(797)));

        scratch.store_ad(763, &AdValue::mul(scratch.ad_value(249), AdValue::exp(AdValue::mul(AdValue::neg(scratch.ad_value(250)), scratch.ad_value(369)))));

        scratch.store_ad(2078, &AdValue::mul(AdValue::scale(scratch.ad_value(275), (4.0 * 1.3806505e-23)), scratch.ad_value(365)));

        scratch.store_ad(2080, &AdValue::div(AdValue::mul(AdValue::square(scratch.ad_value(2074)), scratch.ad_value(2075)), scratch.ad_value(810)));

        scratch.values[2092] = if ((scratch.values[8] != 0.0) && (scratch.values[286] > 0.0)) { 1.0 } else { 0.0 };

        if (scratch.values[2092] != 0.0) {
            scratch.store_ad(764, &AdValue::offset(AdValue::add(scratch.ad_value(281), AdValue::mul(scratch.ad_value(282), scratch.ad_value(367))), scratch.values[29]));
        }

        if (scratch.values[2092] != 0.0) {
            scratch.store_ad(798, &AdValue::exp(AdValue::mul(scratch.ad_value(287), scratch.ad_value(369))));
        }

        if (scratch.values[2092] != 0.0) {
            scratch.store_ad(765, &AdValue::mul(scratch.ad_value(286), scratch.ad_value(798)));
        }

        if (scratch.values[2092] != 0.0) {
            scratch.store_ad(2076, &AdValue::mul(AdValue::scale(scratch.ad_value(765), scratch.values[28]), scratch.ad_value(808)));
        }

        if (scratch.values[2092] != 0.0) {
            scratch.store_ad(2081, &AdValue::mul(scratch.ad_value(2074), AdValue::offset(AdValue::mul(scratch.ad_value(285), scratch.ad_value(368)), 1.0)));
        }

        if (scratch.values[2092] != 0.0) {
            scratch.store_ad(799, &AdValue::add(AdValue::add(scratch.ad_value(371), scratch.ad_value(283)), AdValue::mul(AdValue::scale(scratch.ad_value(2081), 2.0), AdValue::ln(AdValue::scale(AdValue::mul(scratch.ad_value(284), AdValue::powf(scratch.ad_value(372), (-0.75))), 4e-26)))));
        }

        if (scratch.values[2092] != 0.0) {
            scratch.store_ad(799, &{
                if (scratch.values[799] > 0.05) {
                    scratch.ad_value(799)
                } else {
                    AdValue::constant(0.05)
                }
            });
        }

        if (scratch.values[2092] != 0.0) {
            scratch.store_ad(800, &AdValue::div(AdValue::sqrt(AdValue::mul(AdValue::scale(scratch.ad_value(284), ((2.0 * 1.6021918e-19) * scratch.values[806])), scratch.ad_value(370))), scratch.ad_value(808)));
        }

        if (scratch.values[2092] != 0.0) {
            scratch.store_ad(2082, &AdValue::square(scratch.ad_value(800)));
        }

        if (scratch.values[2092] != 0.0) {
            scratch.store_ad(2083, &AdValue::ln(scratch.ad_value(2082)));
        }

        if (scratch.values[2092] != 0.0) {
            scratch.store_ad(801, &AdValue::scale(scratch.ad_value(799), 0.95));
        }

        if (scratch.values[2092] != 0.0) {
            scratch.store_ad(802, &AdValue::mul(AdValue::scale(scratch.ad_value(799), 0.0025), scratch.ad_value(799)));
        }

        if (scratch.values[2092] != 0.0) {
            scratch.values[803] = scratch.values[802];
            scratch.node_derivatives[803] = scratch.node_derivatives[802];
            scratch.branch_derivatives[803] = scratch.branch_derivatives[802];
        }

        if (scratch.values[2092] != 0.0) {
            scratch.store_ad(804, &AdValue::scale(AdValue::sqrt(scratch.ad_value(803)), 0.5));
        }

        if (scratch.values[2092] != 0.0) {
            scratch.store_ad(805, &AdValue::scale(AdValue::sub(AdValue::sub(scratch.ad_value(801), scratch.ad_value(804)), AdValue::sqrt(AdValue::add(AdValue::mul(AdValue::sub(scratch.ad_value(801), scratch.ad_value(804)), AdValue::sub(scratch.ad_value(801), scratch.ad_value(804))), scratch.ad_value(802)))), 0.5));
        }

        if (scratch.values[2092] != 0.0) {
            scratch.store_ad(2084, &AdValue::div(AdValue::mul(AdValue::square(scratch.ad_value(2074)), scratch.ad_value(2076)), scratch.ad_value(810)));
        }

        if (scratch.values[2092] != 0.0) {
            scratch.store_ad(2085, &AdValue::mul(AdValue::scale(scratch.ad_value(294), (4.0 * 1.3806505e-23)), scratch.ad_value(365)));
        }

        if (!(scratch.values[2092] != 0.0)) {
            scratch.values[764] = 0.0;
            scratch.node_derivatives[764] = [0.0; Instance::NODE_COUNT];
            scratch.branch_derivatives[764] = [0.0; Instance::BRANCH_COUNT];
        }

        if (!(scratch.values[2092] != 0.0)) {
            scratch.values[798] = 1.0;
            scratch.node_derivatives[798] = [0.0; Instance::NODE_COUNT];
            scratch.branch_derivatives[798] = [0.0; Instance::BRANCH_COUNT];
        }

        if (!(scratch.values[2092] != 0.0)) {
            scratch.values[765] = 0.0;
            scratch.node_derivatives[765] = [0.0; Instance::NODE_COUNT];
            scratch.branch_derivatives[765] = [0.0; Instance::BRANCH_COUNT];
        }

        if (!(scratch.values[2092] != 0.0)) {
            scratch.values[2076] = 0.0;
            scratch.node_derivatives[2076] = [0.0; Instance::NODE_COUNT];
            scratch.branch_derivatives[2076] = [0.0; Instance::BRANCH_COUNT];
        }

        if (!(scratch.values[2092] != 0.0)) {
            scratch.values[2081] = scratch.values[2074];
            scratch.node_derivatives[2081] = scratch.node_derivatives[2074];
            scratch.branch_derivatives[2081] = scratch.branch_derivatives[2074];
        }

        if (!(scratch.values[2092] != 0.0)) {
            scratch.values[799] = 0.0;
            scratch.node_derivatives[799] = [0.0; Instance::NODE_COUNT];
            scratch.branch_derivatives[799] = [0.0; Instance::BRANCH_COUNT];
        }

        if (!(scratch.values[2092] != 0.0)) {
            scratch.values[800] = 1.0;
            scratch.node_derivatives[800] = [0.0; Instance::NODE_COUNT];
            scratch.branch_derivatives[800] = [0.0; Instance::BRANCH_COUNT];
        }

        if (!(scratch.values[2092] != 0.0)) {
            scratch.values[2082] = 1.0;
            scratch.node_derivatives[2082] = [0.0; Instance::NODE_COUNT];
            scratch.branch_derivatives[2082] = [0.0; Instance::BRANCH_COUNT];
        }

        if (!(scratch.values[2092] != 0.0)) {
            scratch.values[2083] = 0.0;
            scratch.node_derivatives[2083] = [0.0; Instance::NODE_COUNT];
            scratch.branch_derivatives[2083] = [0.0; Instance::BRANCH_COUNT];
        }

        if (!(scratch.values[2092] != 0.0)) {
            scratch.values[801] = 0.0;
            scratch.node_derivatives[801] = [0.0; Instance::NODE_COUNT];
            scratch.branch_derivatives[801] = [0.0; Instance::BRANCH_COUNT];
        }

        if (!(scratch.values[2092] != 0.0)) {
            scratch.values[802] = 0.0;
            scratch.node_derivatives[802] = [0.0; Instance::NODE_COUNT];
            scratch.branch_derivatives[802] = [0.0; Instance::BRANCH_COUNT];
        }

        if (!(scratch.values[2092] != 0.0)) {
            scratch.values[803] = 0.0;
            scratch.node_derivatives[803] = [0.0; Instance::NODE_COUNT];
            scratch.branch_derivatives[803] = [0.0; Instance::BRANCH_COUNT];
        }

        if (!(scratch.values[2092] != 0.0)) {
            scratch.values[804] = 0.0;
            scratch.node_derivatives[804] = [0.0; Instance::NODE_COUNT];
            scratch.branch_derivatives[804] = [0.0; Instance::BRANCH_COUNT];
        }

        if (!(scratch.values[2092] != 0.0)) {
            scratch.values[805] = 0.0;
            scratch.node_derivatives[805] = [0.0; Instance::NODE_COUNT];
            scratch.branch_derivatives[805] = [0.0; Instance::BRANCH_COUNT];
        }

        if (!(scratch.values[2092] != 0.0)) {
            scratch.values[2084] = 0.0;
            scratch.node_derivatives[2084] = [0.0; Instance::NODE_COUNT];
            scratch.branch_derivatives[2084] = [0.0; Instance::BRANCH_COUNT];
        }

        if (!(scratch.values[2092] != 0.0)) {
            scratch.values[2085] = 1.0;
            scratch.node_derivatives[2085] = [0.0; Instance::NODE_COUNT];
            scratch.branch_derivatives[2085] = [0.0; Instance::BRANCH_COUNT];
        }

        scratch.values[939] = 0.0;

        scratch.values[887] = 0.0;

        scratch.values[888] = 0.0;

        scratch.values[889] = 0.0;

        scratch.values[890] = 0.0;

        scratch.values[2019] = 0.0;

        scratch.values[2020] = 0.0;

        scratch.values[2093] = if (scratch.values[0] == 1.0) { 1.0 } else { 0.0 };

        if (scratch.values[2093] != 0.0) {
            scratch.store_ad(849, &AdValue::voltage(ctx, &self.nodes, Some(6), Some(7)));
        }

        if (scratch.values[2093] != 0.0) {
            scratch.store_ad(850, &AdValue::voltage(ctx, &self.nodes, Some(8), Some(7)));
        }

        if (scratch.values[2093] != 0.0) {
            scratch.store_ad(851, &AdValue::voltage(ctx, &self.nodes, Some(7), Some(9)));
        }

        if (scratch.values[2093] != 0.0) {
            scratch.store_ad(854, &AdValue::neg(AdValue::voltage(ctx, &self.nodes, Some(7), Some(11))));
        }

        if (scratch.values[2093] != 0.0) {
            scratch.store_ad(855, &AdValue::neg(AdValue::voltage(ctx, &self.nodes, Some(8), Some(12))));
        }

        if (!(scratch.values[2093] != 0.0)) {
            scratch.store_ad(849, &AdValue::neg(AdValue::voltage(ctx, &self.nodes, Some(6), Some(7))));
        }

        if (!(scratch.values[2093] != 0.0)) {
            scratch.store_ad(850, &AdValue::neg(AdValue::voltage(ctx, &self.nodes, Some(8), Some(7))));
        }

        if (!(scratch.values[2093] != 0.0)) {
            scratch.store_ad(851, &AdValue::neg(AdValue::voltage(ctx, &self.nodes, Some(7), Some(9))));
        }

        if (!(scratch.values[2093] != 0.0)) {
            scratch.store_ad(854, &AdValue::voltage(ctx, &self.nodes, Some(7), Some(11)));
        }

        if (!(scratch.values[2093] != 0.0)) {
            scratch.store_ad(855, &AdValue::voltage(ctx, &self.nodes, Some(8), Some(12)));
        }

        scratch.values[856] = scratch.values[849];
        scratch.node_derivatives[856] = scratch.node_derivatives[849];
        scratch.branch_derivatives[856] = scratch.branch_derivatives[849];

        scratch.values[857] = scratch.values[851];
        scratch.node_derivatives[857] = scratch.node_derivatives[851];
        scratch.branch_derivatives[857] = scratch.branch_derivatives[851];

        scratch.store_ad(858, &AdValue::add(scratch.ad_value(850), scratch.ad_value(851)));

        scratch.store_ad(859, &AdValue::sub(scratch.ad_value(849), scratch.ad_value(850)));

        scratch.store_ad(867, &AdValue::scale(AdValue::neg(scratch.ad_value(856)), scratch.values[364]));

        scratch.store_ad(868, &AdValue::scale(AdValue::neg(scratch.ad_value(859)), scratch.values[364]));

        scratch.values[1999] = 1.0;

        scratch.values[2101] = if (scratch.values[850] < 0.0) { 1.0 } else { 0.0 };

        if (scratch.values[2101] != 0.0) {
            scratch.values[1999] = (-1.0);
            scratch.node_derivatives[1999] = [0.0; Instance::NODE_COUNT];
            scratch.branch_derivatives[1999] = [0.0; Instance::BRANCH_COUNT];
        }

        if (scratch.values[2101] != 0.0) {
            scratch.store_ad(849, &AdValue::sub(scratch.ad_value(849), scratch.ad_value(850)));
        }

        if (scratch.values[2101] != 0.0) {
            scratch.store_ad(851, &AdValue::add(scratch.ad_value(851), scratch.ad_value(850)));
        }

        if (scratch.values[2101] != 0.0) {
            scratch.store_ad(850, &AdValue::neg(scratch.ad_value(850)));
        }

        scratch.store_ad(852, &AdValue::add(scratch.ad_value(850), scratch.ad_value(851)));

        scratch.store_ad(860, &AdValue::div(AdValue::square(scratch.ad_value(850)), AdValue::offset(AdValue::sqrt(AdValue::offset(AdValue::square(scratch.ad_value(850)), 0.01)), 0.1)));

        scratch.store_ad(2086, &AdValue::add(AdValue::scale(AdValue::sub(AdValue::add(scratch.ad_value(852), scratch.ad_value(851)), AdValue::sqrt(AdValue::add(AdValue::mul(AdValue::sub(scratch.ad_value(852), scratch.ad_value(851)), AdValue::sub(scratch.ad_value(852), scratch.ad_value(851))), scratch.ad_value(778)))), 0.5), scratch.ad_value(776)));

        scratch.store_ad(869, &AdValue::add(AdValue::sub(scratch.ad_value(851), AdValue::scale(AdValue::sub(scratch.ad_value(2086), AdValue::sqrt(AdValue::add(AdValue::mul(scratch.ad_value(2086), scratch.ad_value(2086)), scratch.ad_value(777)))), 0.5)), scratch.ad_value(780)));

        scratch.values[870] = scratch.values[869];
        scratch.node_derivatives[870] = scratch.node_derivatives[869];
        scratch.branch_derivatives[870] = scratch.branch_derivatives[869];

        scratch.values[2222] = if ((scratch.values[7] != 0.0) && (scratch.values[200] != 1.0)) { 1.0 } else { 0.0 };

        if (scratch.values[2222] != 0.0) {
            scratch.store_ad(871, &AdValue::add(scratch.ad_value(869), AdValue::scale(AdValue::sub(scratch.ad_value(850), scratch.ad_value(860)), 0.5)));
        }

        if (scratch.values[2222] != 0.0) {
            scratch.store_ad(872, &AdValue::sub(AdValue::sqrt(AdValue::add(scratch.ad_value(871), scratch.ad_value(767))), scratch.ad_value(775)));
        }

        if (scratch.values[2222] != 0.0) {
            scratch.store_ad(2086, &AdValue::offset(AdValue::div(AdValue::scale(AdValue::sub(scratch.ad_value(872), scratch.ad_value(782)), 2.0), scratch.ad_value(783)), (-1.0)));
        }

        if (scratch.values[2222] != 0.0) {
            scratch.store_ad(873, &AdValue::sub(scratch.ad_value(872), AdValue::mul(AdValue::mul(AdValue::scale(AdValue::sub_from_scalar(1.0, scratch.ad_value(200)), 0.25), scratch.ad_value(783)), AdValue::add(scratch.ad_value(2086), AdValue::sqrt(AdValue::offset(AdValue::square(scratch.ad_value(2086)), 0.4804530139182))))));
        }

        if (scratch.values[2222] != 0.0) {
            scratch.store_ad(874, &AdValue::add(AdValue::square(scratch.ad_value(873)), AdValue::mul(AdValue::scale(scratch.ad_value(775), 2.0), scratch.ad_value(873))));
        }

        if (scratch.values[2222] != 0.0) {
            scratch.store_ad(869, &AdValue::sub(scratch.ad_value(874), AdValue::scale(AdValue::sub(scratch.ad_value(850), scratch.ad_value(860)), 0.5)));
        }

        scratch.values[2102] = scratch.values[767];
        scratch.node_derivatives[2102] = scratch.node_derivatives[767];
        scratch.branch_derivatives[2102] = scratch.branch_derivatives[767];

        scratch.values[2103] = scratch.values[768];
        scratch.node_derivatives[2103] = scratch.node_derivatives[768];
        scratch.branch_derivatives[2103] = scratch.branch_derivatives[768];

        scratch.values[2104] = scratch.values[869];
        scratch.node_derivatives[2104] = scratch.node_derivatives[869];
        scratch.branch_derivatives[2104] = scratch.branch_derivatives[869];

        scratch.values[2216] = 1.0;

        scratch.values[2167] = 0.0;

        scratch.values[2183] = 1.0;

        scratch.values[2182] = 0.0;

        scratch.values[2178] = 0.0;

        scratch.values[2155] = 0.0;

        scratch.values[2176] = 0.0;

        scratch.values[2177] = 0.0;

        scratch.values[2190] = 1.0;

        scratch.values[2181] = 0.0;

        scratch.values[2165] = 1.0;

        scratch.values[2186] = 1.0;

        scratch.values[2187] = 1.0;

        scratch.values[2212] = 0.0;

        scratch.values[2117] = 0.0;

        scratch.values[2166] = 0.0;

        scratch.values[2134] = 0.0;

        scratch.values[2129] = 0.0;

        scratch.values[2133] = 1.0;

        scratch.values[2184] = 1.0;

        scratch.values[2147] = 0.0;

        scratch.values[2136] = 0.0;

        scratch.values[2164] = 0.0;

        scratch.store_ad(861, &AdValue::add(scratch.ad_value(849), scratch.ad_value(2104)));

        scratch.store_ad(862, &AdValue::sub(scratch.ad_value(861), scratch.ad_value(751)));

        scratch.store_ad(2105, &AdValue::add(scratch.ad_value(2104), AdValue::scale(AdValue::sub(scratch.ad_value(850), scratch.ad_value(860)), 0.5)));

        scratch.values[2223] = if (scratch.values[217] < 1e-10) { 1.0 } else { 0.0 };

        if (scratch.values[2223] != 0.0) {
            scratch.values[863] = scratch.values[860];
            scratch.node_derivatives[863] = scratch.node_derivatives[860];
            scratch.branch_derivatives[863] = scratch.branch_derivatives[860];
        }

        if (!(scratch.values[2223] != 0.0)) {
            scratch.store_ad(863, &AdValue::div(AdValue::scale(AdValue::offset(AdValue::sqrt(AdValue::offset(AdValue::mul(scratch.ad_value(217), scratch.ad_value(860)), 1.0)), (-1.0)), 2.0), scratch.ad_value(217)));
        }

        scratch.store_ad(864, &AdValue::mul(AdValue::mul(scratch.ad_value(216), scratch.ad_value(863)), AdValue::offset(AdValue::mul(scratch.ad_value(218), scratch.ad_value(2105)), 1.0)));

        scratch.store_ad(2000, &AdValue::mul(AdValue::mul(scratch.ad_value(219), AdValue::offset(AdValue::mul(scratch.ad_value(221), scratch.ad_value(860)), 1.0)), AdValue::offset(AdValue::mul(scratch.ad_value(220), scratch.ad_value(2105)), 1.0)));

        scratch.store_ad(862, &AdValue::add(scratch.ad_value(862), scratch.ad_value(864)));

        scratch.values[2224] = if (scratch.values[205] > 0.0) { 1.0 } else { 0.0 };

        if (scratch.values[2224] != 0.0) {
            scratch.store_ad(2107, &AdValue::mul(AdValue::scale(scratch.ad_value(205), 0.5), AdValue::add(AdValue::sub(AdValue::add(scratch.ad_value(849), scratch.ad_value(851)), scratch.ad_value(203)), AdValue::sqrt(AdValue::add(AdValue::mul(AdValue::neg(AdValue::sub(AdValue::add(scratch.ad_value(849), scratch.ad_value(851)), scratch.ad_value(203))), AdValue::neg(AdValue::sub(AdValue::add(scratch.ad_value(849), scratch.ad_value(851)), scratch.ad_value(203)))), scratch.ad_value(204))))));
        }

        if (scratch.values[2224] != 0.0) {
            scratch.store_ad(2108, &AdValue::mul(scratch.ad_value(2103), AdValue::sqrt(AdValue::offset(scratch.ad_value(2107), 1.0))));
        }

        if (!(scratch.values[2224] != 0.0)) {
            scratch.values[2108] = scratch.values[2103];
            scratch.node_derivatives[2108] = scratch.node_derivatives[2103];
            scratch.branch_derivatives[2108] = scratch.branch_derivatives[2103];
        }

        scratch.store_ad(2109, &AdValue::square(scratch.ad_value(2108)));

        scratch.store_ad(2110, &AdValue::div_from_scalar(1.0, scratch.ad_value(2109)));

        scratch.values[2006] = 1.0;

        scratch.values[2225] = if (scratch.values[210] > 0.0) { 1.0 } else { 0.0 };

        if (scratch.values[2225] != 0.0) {
            scratch.store_ad(2001, &AdValue::mul(AdValue::scale(scratch.ad_value(862), 2.0), scratch.ad_value(370)));
        }

        if (scratch.values[2225] != 0.0) {
            scratch.store_ad(2087, &AdValue::add(scratch.ad_value(2109), scratch.ad_value(2001)));
        }

        if (scratch.values[2225] != 0.0) {
            scratch.store_ad(2088, &AdValue::scale(AdValue::add(AdValue::add(scratch.ad_value(2087), scratch.ad_value(2001)), AdValue::sqrt(AdValue::offset(AdValue::mul(AdValue::add(scratch.ad_value(2087), scratch.ad_value(2001)), AdValue::add(scratch.ad_value(2087), scratch.ad_value(2001))), 5.0))), 0.5));
        }

        if (scratch.values[2225] != 0.0) {
            scratch.store_ad(2002, &AdValue::scale(AdValue::sub(scratch.ad_value(2087), AdValue::mul(scratch.ad_value(2108), AdValue::sqrt(scratch.ad_value(2088)))), 0.5));
        }

        if (scratch.values[2225] != 0.0) {
            scratch.store_ad(2003, &AdValue::mul(scratch.ad_value(2102), scratch.ad_value(370)));
        }

        if (scratch.values[2225] != 0.0) {
            scratch.store_ad(2004, &AdValue::mul(scratch.ad_value(2105), scratch.ad_value(370)));
        }

        if (scratch.values[2225] != 0.0) {
            scratch.store_ad(2087, &AdValue::offset(AdValue::add(scratch.ad_value(2003), scratch.ad_value(2004)), 2.0));
        }

        if (scratch.values[2225] != 0.0) {
            scratch.store_ad(2005, &AdValue::scale(AdValue::sub(AdValue::add(scratch.ad_value(2002), scratch.ad_value(2087)), AdValue::sqrt(AdValue::offset(AdValue::mul(AdValue::sub(scratch.ad_value(2002), scratch.ad_value(2087)), AdValue::sub(scratch.ad_value(2002), scratch.ad_value(2087))), 5.0))), 0.5));
        }

        if (scratch.values[2225] != 0.0) {
            scratch.store_ad(2088, &AdValue::mul(scratch.ad_value(753), AdValue::sub(scratch.ad_value(2005), AdValue::mul(AdValue::offset(scratch.ad_value(211), 1.0), AdValue::add(AdValue::scale(scratch.ad_value(2003), 0.5), scratch.ad_value(2004))))));
        }

        scratch.values[2226] = if (scratch.values[2088] > (-230.25850929940458)) { 1.0 } else { 0.0 };

        if ((scratch.values[2225] != 0.0) && (scratch.values[2226] != 0.0)) {
            scratch.store_ad(2006, &AdValue::exp(scratch.ad_value(2088)));
        }

        if ((scratch.values[2225] != 0.0) && (!(scratch.values[2226] != 0.0))) {
            scratch.store_ad(2006, &AdValue::div_from_scalar(1e-100, AdValue::offset(AdValue::mul(AdValue::sub_from_scalar((-230.25850929940458), scratch.ad_value(2088)), AdValue::offset(AdValue::scale(AdValue::mul(AdValue::sub_from_scalar((-230.25850929940458), scratch.ad_value(2088)), AdValue::offset(AdValue::scale(AdValue::sub_from_scalar((-230.25850929940458), scratch.ad_value(2088)), 0.3333333333333333), 1.0)), 0.5), 1.0)), 1.0)));
        }

        scratch.store_ad(2007, &AdValue::offset(AdValue::mul(scratch.ad_value(752), scratch.ad_value(2006)), 1.0));

        scratch.store_ad(2008, &AdValue::mul(AdValue::mul(scratch.ad_value(2074), scratch.ad_value(2007)), AdValue::offset(scratch.ad_value(2000), 1.0)));

        scratch.store_ad(2009, &AdValue::div_from_scalar(1.0, scratch.ad_value(2008)));

        scratch.store_ad(2106, &AdValue::mul(scratch.ad_value(862), scratch.ad_value(2009)));

        scratch.store_ad(2111, &AdValue::offset(AdValue::scale(scratch.ad_value(2108), 0.7071067811865475), 1.0));

        scratch.store_ad(2112, &AdValue::div_from_scalar(1.0, scratch.ad_value(2111)));

        scratch.store_ad(2113, &AdValue::mul(scratch.ad_value(2104), scratch.ad_value(2009)));

        scratch.store_ad(2114, &AdValue::add(AdValue::mul(scratch.ad_value(2102), scratch.ad_value(2009)), scratch.ad_value(2113)));

        scratch.values[2227] = if (scratch.values[2114] < 460.51701859880916) { 1.0 } else { 0.0 };

        if (scratch.values[2227] != 0.0) {
            scratch.store_ad(2115, &AdValue::exp(AdValue::neg(scratch.ad_value(2114))));
        }

        if (!(scratch.values[2227] != 0.0)) {
            scratch.store_ad(2115, &AdValue::div_from_scalar(1e-200, AdValue::offset(AdValue::mul(AdValue::offset(scratch.ad_value(2114), (-460.51701859880916)), AdValue::offset(AdValue::scale(AdValue::mul(AdValue::offset(scratch.ad_value(2114), (-460.51701859880916)), AdValue::offset(AdValue::scale(AdValue::offset(scratch.ad_value(2114), (-460.51701859880916)), 0.3333333333333333), 1.0)), 0.5), 1.0)), 1.0)));
        }

        scratch.store_ad(2116, &AdValue::scale(scratch.ad_value(2111), 1e-5));

        scratch.values[2228] = if (((scratch.values[2106]) as f64).abs() <= scratch.values[2116]) { 1.0 } else { 0.0 };

        if (scratch.values[2228] != 0.0) {
            scratch.store_ad(2192, &AdValue::scale(AdValue::square(scratch.ad_value(2112)), (0.16666666666666666 * 0.7071067811865475)));
        }

        if (scratch.values[2228] != 0.0) {
            scratch.store_ad(2117, &AdValue::mul(AdValue::mul(scratch.ad_value(2106), scratch.ad_value(2112)), AdValue::offset(AdValue::mul(AdValue::mul(AdValue::mul(scratch.ad_value(2106), AdValue::sub_from_scalar(1.0, scratch.ad_value(2115))), scratch.ad_value(2108)), scratch.ad_value(2192)), 1.0)));
        }

        scratch.values[2229] = if (scratch.values[2106] < (-scratch.values[2116])) { 1.0 } else { 0.0 };

        if ((!(scratch.values[2228] != 0.0)) && (scratch.values[2229] != 0.0)) {
            scratch.store_ad(2194, &AdValue::neg(scratch.ad_value(2106)));
        }

    }

    pub(super) fn stamp_transient_block_31(
        &mut self,
        ctx: &GeneratedEvalContext<'_>,
        stamper: &mut GeneratedStamper<'_>,
        scratch: &mut Scratch,
    ) {
        let _ = stamper;
        if ((!(scratch.values[2228] != 0.0)) && (scratch.values[2229] != 0.0)) {
            scratch.store_ad(2195, &AdValue::scale(AdValue::mul(scratch.ad_value(2194), scratch.ad_value(2112)), 1.25));
        }

        if ((!(scratch.values[2228] != 0.0)) && (scratch.values[2229] != 0.0)) {
            scratch.store_ad(2196, &AdValue::scale(AdValue::sub(AdValue::offset(scratch.ad_value(2195), 10.0), AdValue::sqrt(AdValue::offset(AdValue::mul(AdValue::offset(scratch.ad_value(2195), (-6.0)), AdValue::offset(scratch.ad_value(2195), (-6.0))), 64.0))), 0.5));
        }

        if ((!(scratch.values[2228] != 0.0)) && (scratch.values[2229] != 0.0)) {
            scratch.store_ad(2191, &AdValue::sub(scratch.ad_value(2194), scratch.ad_value(2196)));
        }

        if ((!(scratch.values[2228] != 0.0)) && (scratch.values[2229] != 0.0)) {
            scratch.store_ad(2197, &AdValue::add(AdValue::square(scratch.ad_value(2191)), AdValue::mul(scratch.ad_value(2109), AdValue::offset(scratch.ad_value(2196), 1.0))));
        }

        if ((!(scratch.values[2228] != 0.0)) && (scratch.values[2229] != 0.0)) {
            scratch.store_ad(2198, &AdValue::sub(AdValue::scale(scratch.ad_value(2191), 2.0), scratch.ad_value(2109)));
        }

        if ((!(scratch.values[2228] != 0.0)) && (scratch.values[2229] != 0.0)) {
            scratch.store_ad(2199, &AdValue::sub(AdValue::ln(AdValue::mul(scratch.ad_value(2197), scratch.ad_value(2110))), scratch.ad_value(2196)));
        }

        if ((!(scratch.values[2228] != 0.0)) && (scratch.values[2229] != 0.0)) {
            scratch.store_ad(962, &AdValue::add(scratch.ad_value(2197), scratch.ad_value(2198)));
        }

        if ((!(scratch.values[2228] != 0.0)) && (scratch.values[2229] != 0.0)) {
            scratch.store_ad(961, &AdValue::add(AdValue::square(scratch.ad_value(962)), AdValue::mul(scratch.ad_value(2199), AdValue::sub(AdValue::scale(AdValue::square(scratch.ad_value(2198)), 0.5), scratch.ad_value(2197)))));
        }

        if ((!(scratch.values[2228] != 0.0)) && (scratch.values[2229] != 0.0)) {
            scratch.store_ad(2200, &AdValue::add(scratch.ad_value(2196), AdValue::div(AdValue::mul(AdValue::mul(scratch.ad_value(2197), scratch.ad_value(962)), scratch.ad_value(2199)), AdValue::add(scratch.ad_value(961), AdValue::mul(AdValue::mul(AdValue::mul(AdValue::mul(AdValue::div(scratch.ad_value(962), scratch.ad_value(961)), scratch.ad_value(2199)), scratch.ad_value(2199)), scratch.ad_value(2198)), AdValue::sub(AdValue::scale(AdValue::square(scratch.ad_value(2198)), 0.3333333333333333), scratch.ad_value(2197)))))));
        }

        scratch.values[2230] = if (scratch.values[2200] < 230.25850929940458) { 1.0 } else { 0.0 };

        if (((!(scratch.values[2228] != 0.0)) && (scratch.values[2229] != 0.0)) && (scratch.values[2230] != 0.0)) {
            scratch.store_ad(2201, &AdValue::exp(scratch.ad_value(2200)));
        }

        if (((!(scratch.values[2228] != 0.0)) && (scratch.values[2229] != 0.0)) && (!(scratch.values[2230] != 0.0))) {
            scratch.store_ad(2201, &AdValue::scale(AdValue::offset(AdValue::mul(AdValue::offset(scratch.ad_value(2200), (-230.25850929940458)), AdValue::offset(AdValue::scale(AdValue::mul(AdValue::offset(scratch.ad_value(2200), (-230.25850929940458)), AdValue::offset(AdValue::scale(AdValue::offset(scratch.ad_value(2200), (-230.25850929940458)), 0.3333333333333333), 1.0)), 0.5), 1.0)), 1.0), 1e100));
        }

        if ((!(scratch.values[2228] != 0.0)) && (scratch.values[2229] != 0.0)) {
            scratch.store_ad(2202, &AdValue::div_from_scalar(1.0, scratch.ad_value(2201)));
        }

        if ((!(scratch.values[2228] != 0.0)) && (scratch.values[2229] != 0.0)) {
            scratch.store_ad(2191, &AdValue::div_from_scalar(1.0, AdValue::offset(AdValue::square(scratch.ad_value(2200)), 2.0)));
        }

        if ((!(scratch.values[2228] != 0.0)) && (scratch.values[2229] != 0.0)) {
            scratch.store_ad(2203, &AdValue::mul(AdValue::square(scratch.ad_value(2200)), scratch.ad_value(2191)));
        }

        if ((!(scratch.values[2228] != 0.0)) && (scratch.values[2229] != 0.0)) {
            scratch.store_ad(2204, &AdValue::scale(AdValue::mul(AdValue::mul(scratch.ad_value(2200), scratch.ad_value(2191)), scratch.ad_value(2191)), 4.0));
        }

        if ((!(scratch.values[2228] != 0.0)) && (scratch.values[2229] != 0.0)) {
            scratch.store_ad(2205, &AdValue::mul(AdValue::mul(AdValue::sub(AdValue::scale(scratch.ad_value(2191), 8.0), AdValue::scale(scratch.ad_value(2203), 12.0)), scratch.ad_value(2191)), scratch.ad_value(2191)));
        }

        if ((!(scratch.values[2228] != 0.0)) && (scratch.values[2229] != 0.0)) {
            scratch.store_ad(2191, &AdValue::sub(scratch.ad_value(2194), scratch.ad_value(2200)));
        }

        if ((!(scratch.values[2228] != 0.0)) && (scratch.values[2229] != 0.0)) {
            scratch.store_ad(2192, &AdValue::mul(scratch.ad_value(2115), scratch.ad_value(2202)));
        }

        if ((!(scratch.values[2228] != 0.0)) && (scratch.values[2229] != 0.0)) {
            scratch.store_ad(2206, &AdValue::add(AdValue::scale(scratch.ad_value(2191), 2.0), AdValue::mul(scratch.ad_value(2109), AdValue::add(AdValue::sub(AdValue::offset(scratch.ad_value(2201), (-1.0)), scratch.ad_value(2192)), AdValue::mul(scratch.ad_value(2115), AdValue::sub_from_scalar(1.0, scratch.ad_value(2204)))))));
        }

        if ((!(scratch.values[2228] != 0.0)) && (scratch.values[2229] != 0.0)) {
            scratch.store_ad(2207, &AdValue::sub(AdValue::square(scratch.ad_value(2191)), AdValue::mul(scratch.ad_value(2109), AdValue::add(AdValue::add(AdValue::offset(AdValue::sub(scratch.ad_value(2201), scratch.ad_value(2200)), (-1.0)), scratch.ad_value(2192)), AdValue::mul(scratch.ad_value(2115), AdValue::sub(AdValue::offset(scratch.ad_value(2200), (-1.0)), scratch.ad_value(2203)))))));
        }

        if ((!(scratch.values[2228] != 0.0)) && (scratch.values[2229] != 0.0)) {
            scratch.store_ad(2191, &AdValue::sub_from_scalar(2.0, AdValue::mul(scratch.ad_value(2109), AdValue::sub(AdValue::add(scratch.ad_value(2201), scratch.ad_value(2192)), AdValue::mul(scratch.ad_value(2115), scratch.ad_value(2205))))));
        }

        if ((!(scratch.values[2228] != 0.0)) && (scratch.values[2229] != 0.0)) {
            scratch.store_ad(2191, &AdValue::sub(AdValue::square(scratch.ad_value(2206)), AdValue::scale(AdValue::mul(scratch.ad_value(2207), scratch.ad_value(2191)), 2.0)));
        }

        if ((!(scratch.values[2228] != 0.0)) && (scratch.values[2229] != 0.0)) {
            scratch.store_ad(2117, &AdValue::sub(AdValue::neg(scratch.ad_value(2200)), AdValue::scale(AdValue::div(scratch.ad_value(2207), AdValue::add(scratch.ad_value(2206), AdValue::sqrt(scratch.ad_value(2191)))), 2.0)));
        }

        if ((!(scratch.values[2228] != 0.0)) && (!(scratch.values[2229] != 0.0))) {
            scratch.store_ad(2208, &AdValue::div_from_scalar(1.0, AdValue::offset(AdValue::scale(scratch.ad_value(2108), 0.7324648775608221), 1.25)));
        }

        if ((!(scratch.values[2228] != 0.0)) && (!(scratch.values[2229] != 0.0))) {
            scratch.store_ad(2209, &AdValue::mul(AdValue::offset(AdValue::mul(AdValue::scale(scratch.ad_value(2111), 1.25), scratch.ad_value(2208)), (-1.0)), scratch.ad_value(2208)));
        }

        if ((!(scratch.values[2228] != 0.0)) && (!(scratch.values[2229] != 0.0))) {
            scratch.store_ad(2210, &AdValue::mul(AdValue::mul(scratch.ad_value(2106), scratch.ad_value(2112)), AdValue::offset(AdValue::mul(scratch.ad_value(2209), scratch.ad_value(2106)), 1.0)));
        }

        scratch.values[2231] = if ((-scratch.values[2210]) > (-230.25850929940458)) { 1.0 } else { 0.0 };

        if (((!(scratch.values[2228] != 0.0)) && (!(scratch.values[2229] != 0.0))) && (scratch.values[2231] != 0.0)) {
            scratch.store_ad(2191, &AdValue::exp(AdValue::neg(scratch.ad_value(2210))));
        }

        if (((!(scratch.values[2228] != 0.0)) && (!(scratch.values[2229] != 0.0))) && (!(scratch.values[2231] != 0.0))) {
            scratch.store_ad(2191, &AdValue::div_from_scalar(1e-100, AdValue::offset(AdValue::mul(AdValue::sub_from_scalar((-230.25850929940458), AdValue::neg(scratch.ad_value(2210))), AdValue::offset(AdValue::scale(AdValue::mul(AdValue::sub_from_scalar((-230.25850929940458), AdValue::neg(scratch.ad_value(2210))), AdValue::offset(AdValue::scale(AdValue::sub_from_scalar((-230.25850929940458), AdValue::neg(scratch.ad_value(2210))), 0.3333333333333333), 1.0)), 0.5), 1.0)), 1.0)));
        }

        if ((!(scratch.values[2228] != 0.0)) && (!(scratch.values[2229] != 0.0))) {
            scratch.store_ad(2211, &AdValue::sub_from_scalar(1.0, scratch.ad_value(2191)));
        }

        if ((!(scratch.values[2228] != 0.0)) && (!(scratch.values[2229] != 0.0))) {
            scratch.store_ad(2212, &AdValue::sub(AdValue::add(scratch.ad_value(2106), AdValue::scale(scratch.ad_value(2109), 0.5)), AdValue::mul(scratch.ad_value(2108), AdValue::sqrt(AdValue::sub(AdValue::add(scratch.ad_value(2106), AdValue::scale(scratch.ad_value(2109), 0.25)), scratch.ad_value(2211))))));
        }

        if ((!(scratch.values[2228] != 0.0)) && (!(scratch.values[2229] != 0.0))) {
            scratch.store_ad(2213, &AdValue::offset(scratch.ad_value(2114), 3.0));
        }

        if ((!(scratch.values[2228] != 0.0)) && (!(scratch.values[2229] != 0.0))) {
            scratch.store_ad(2196, &AdValue::sub(AdValue::scale(AdValue::sub(AdValue::add(scratch.ad_value(2212), scratch.ad_value(2213)), AdValue::sqrt(AdValue::offset(AdValue::mul(AdValue::sub(scratch.ad_value(2212), scratch.ad_value(2213)), AdValue::sub(scratch.ad_value(2212), scratch.ad_value(2213))), 5.0))), 0.5), AdValue::scale(AdValue::sub(scratch.ad_value(2213), AdValue::sqrt(AdValue::offset(AdValue::square(scratch.ad_value(2213)), 5.0))), 0.5)));
        }

        if ((!(scratch.values[2228] != 0.0)) && (!(scratch.values[2229] != 0.0))) {
            scratch.store_ad(2191, &AdValue::sub(scratch.ad_value(2106), scratch.ad_value(2196)));
        }

        if ((!(scratch.values[2228] != 0.0)) && (!(scratch.values[2229] != 0.0))) {
            scratch.store_ad(2192, &AdValue::exp(AdValue::neg(scratch.ad_value(2196))));
        }

        if ((!(scratch.values[2228] != 0.0)) && (!(scratch.values[2229] != 0.0))) {
            scratch.store_ad(2193, &AdValue::div_from_scalar(1.0, AdValue::offset(AdValue::square(scratch.ad_value(2196)), 2.0)));
        }

        if ((!(scratch.values[2228] != 0.0)) && (!(scratch.values[2229] != 0.0))) {
            scratch.store_ad(2203, &AdValue::mul(AdValue::square(scratch.ad_value(2196)), scratch.ad_value(2193)));
        }

        if ((!(scratch.values[2228] != 0.0)) && (!(scratch.values[2229] != 0.0))) {
            scratch.store_ad(2204, &AdValue::scale(AdValue::mul(AdValue::mul(scratch.ad_value(2196), scratch.ad_value(2193)), scratch.ad_value(2193)), 4.0));
        }

        if ((!(scratch.values[2228] != 0.0)) && (!(scratch.values[2229] != 0.0))) {
            scratch.store_ad(2205, &AdValue::mul(AdValue::mul(AdValue::sub(AdValue::scale(scratch.ad_value(2193), 8.0), AdValue::scale(scratch.ad_value(2203), 12.0)), scratch.ad_value(2193)), scratch.ad_value(2193)));
        }

        if ((!(scratch.values[2228] != 0.0)) && (!(scratch.values[2229] != 0.0))) {
            scratch.store_ad(2197, &AdValue::max_from_scalar(1e-40, AdValue::sub(AdValue::square(scratch.ad_value(2191)), AdValue::mul(scratch.ad_value(2109), AdValue::sub(AdValue::offset(AdValue::add(scratch.ad_value(2192), scratch.ad_value(2196)), (-1.0)), AdValue::mul(scratch.ad_value(2115), AdValue::add(AdValue::offset(scratch.ad_value(2196), 1.0), scratch.ad_value(2203))))))));
        }

        if ((!(scratch.values[2228] != 0.0)) && (!(scratch.values[2229] != 0.0))) {
            scratch.store_ad(2214, &AdValue::sub_from_scalar(1.0, AdValue::scale(AdValue::mul(scratch.ad_value(2109), AdValue::sub(scratch.ad_value(2192), AdValue::mul(scratch.ad_value(2115), scratch.ad_value(2205)))), 0.5)));
        }

        if ((!(scratch.values[2228] != 0.0)) && (!(scratch.values[2229] != 0.0))) {
            scratch.store_ad(2198, &AdValue::add(AdValue::scale(scratch.ad_value(2191), 2.0), AdValue::mul(scratch.ad_value(2109), AdValue::sub(AdValue::sub_from_scalar(1.0, scratch.ad_value(2192)), AdValue::mul(scratch.ad_value(2115), AdValue::offset(scratch.ad_value(2204), 1.0))))));
        }

        if ((!(scratch.values[2228] != 0.0)) && (!(scratch.values[2229] != 0.0))) {
            scratch.store_ad(2199, &AdValue::add(AdValue::sub(scratch.ad_value(2114), scratch.ad_value(2196)), AdValue::ln(AdValue::div(scratch.ad_value(2197), scratch.ad_value(2109)))));
        }

        if ((!(scratch.values[2228] != 0.0)) && (!(scratch.values[2229] != 0.0))) {
            scratch.store_ad(962, &AdValue::add(scratch.ad_value(2197), scratch.ad_value(2198)));
        }

        if ((!(scratch.values[2228] != 0.0)) && (!(scratch.values[2229] != 0.0))) {
            scratch.store_ad(961, &AdValue::add(AdValue::square(scratch.ad_value(962)), AdValue::mul(scratch.ad_value(2199), AdValue::sub(AdValue::scale(AdValue::square(scratch.ad_value(2198)), 0.5), AdValue::mul(scratch.ad_value(2197), scratch.ad_value(2214))))));
        }

        if ((!(scratch.values[2228] != 0.0)) && (!(scratch.values[2229] != 0.0))) {
            let assign42000_ad_e54283: AdValue = AdValue::add(scratch.ad_value(2196), AdValue::div(AdValue::mul(AdValue::mul(scratch.ad_value(2197), scratch.ad_value(962)), scratch.ad_value(2199)), AdValue::add(scratch.ad_value(961), AdValue::mul(AdValue::mul(AdValue::mul(AdValue::mul(AdValue::div(scratch.ad_value(962), scratch.ad_value(961)), scratch.ad_value(2199)), scratch.ad_value(2199)), scratch.ad_value(2198)), AdValue::sub(AdValue::scale(AdValue::square(scratch.ad_value(2198)), 0.3333333333333333), AdValue::mul(scratch.ad_value(2197), scratch.ad_value(2214)))))));
            scratch.store_ad(2215, &assign42000_ad_e54283);
        }

        scratch.values[2232] = if (scratch.values[2215] < 230.25850929940458) { 1.0 } else { 0.0 };

        if (((!(scratch.values[2228] != 0.0)) && (!(scratch.values[2229] != 0.0))) && (scratch.values[2232] != 0.0)) {
            scratch.store_ad(2201, &AdValue::exp(scratch.ad_value(2215)));
        }

        if (((!(scratch.values[2228] != 0.0)) && (!(scratch.values[2229] != 0.0))) && (scratch.values[2232] != 0.0)) {
            scratch.store_ad(2202, &AdValue::div_from_scalar(1.0, scratch.ad_value(2201)));
        }

        if (((!(scratch.values[2228] != 0.0)) && (!(scratch.values[2229] != 0.0))) && (scratch.values[2232] != 0.0)) {
            scratch.store_ad(2201, &AdValue::mul(scratch.ad_value(2115), scratch.ad_value(2201)));
        }

        scratch.values[2233] = if (scratch.values[2215] > (scratch.values[2114] - 230.25850929940458)) { 1.0 } else { 0.0 };

        if ((((!(scratch.values[2228] != 0.0)) && (!(scratch.values[2229] != 0.0))) && (!(scratch.values[2232] != 0.0))) && (scratch.values[2233] != 0.0)) {
            scratch.store_ad(2201, &AdValue::exp(AdValue::sub(scratch.ad_value(2215), scratch.ad_value(2114))));
        }

        if ((((!(scratch.values[2228] != 0.0)) && (!(scratch.values[2229] != 0.0))) && (!(scratch.values[2232] != 0.0))) && (scratch.values[2233] != 0.0)) {
            scratch.store_ad(2202, &AdValue::div(scratch.ad_value(2115), scratch.ad_value(2201)));
        }

        if ((((!(scratch.values[2228] != 0.0)) && (!(scratch.values[2229] != 0.0))) && (!(scratch.values[2232] != 0.0))) && (!(scratch.values[2233] != 0.0))) {
            scratch.store_ad(2201, &AdValue::div_from_scalar(1e-100, AdValue::offset(AdValue::mul(AdValue::offset(AdValue::sub(scratch.ad_value(2114), scratch.ad_value(2215)), (-230.25850929940458)), AdValue::offset(AdValue::scale(AdValue::mul(AdValue::offset(AdValue::sub(scratch.ad_value(2114), scratch.ad_value(2215)), (-230.25850929940458)), AdValue::offset(AdValue::scale(AdValue::offset(AdValue::sub(scratch.ad_value(2114), scratch.ad_value(2215)), (-230.25850929940458)), 0.3333333333333333), 1.0)), 0.5), 1.0)), 1.0)));
        }

        if ((((!(scratch.values[2228] != 0.0)) && (!(scratch.values[2229] != 0.0))) && (!(scratch.values[2232] != 0.0))) && (!(scratch.values[2233] != 0.0))) {
            scratch.store_ad(2202, &AdValue::div_from_scalar(1e-100, AdValue::offset(AdValue::mul(AdValue::offset(scratch.ad_value(2215), (-230.25850929940458)), AdValue::offset(AdValue::scale(AdValue::mul(AdValue::offset(scratch.ad_value(2215), (-230.25850929940458)), AdValue::offset(AdValue::scale(AdValue::offset(scratch.ad_value(2215), (-230.25850929940458)), 0.3333333333333333), 1.0)), 0.5), 1.0)), 1.0)));
        }

        if ((!(scratch.values[2228] != 0.0)) && (!(scratch.values[2229] != 0.0))) {
            scratch.store_ad(2191, &AdValue::div_from_scalar(1.0, AdValue::offset(AdValue::square(scratch.ad_value(2215)), 2.0)));
        }

        if ((!(scratch.values[2228] != 0.0)) && (!(scratch.values[2229] != 0.0))) {
            scratch.store_ad(2203, &AdValue::mul(AdValue::square(scratch.ad_value(2215)), scratch.ad_value(2191)));
        }

        if ((!(scratch.values[2228] != 0.0)) && (!(scratch.values[2229] != 0.0))) {
            scratch.store_ad(2204, &AdValue::scale(AdValue::mul(AdValue::mul(scratch.ad_value(2215), scratch.ad_value(2191)), scratch.ad_value(2191)), 4.0));
        }

        if ((!(scratch.values[2228] != 0.0)) && (!(scratch.values[2229] != 0.0))) {
            scratch.store_ad(2205, &AdValue::mul(AdValue::mul(AdValue::sub(AdValue::scale(scratch.ad_value(2191), 8.0), AdValue::scale(scratch.ad_value(2203), 12.0)), scratch.ad_value(2191)), scratch.ad_value(2191)));
        }

        if ((!(scratch.values[2228] != 0.0)) && (!(scratch.values[2229] != 0.0))) {
            scratch.store_ad(2191, &AdValue::sub(scratch.ad_value(2106), scratch.ad_value(2215)));
        }

        if ((!(scratch.values[2228] != 0.0)) && (!(scratch.values[2229] != 0.0))) {
            scratch.store_ad(2206, &AdValue::add(AdValue::scale(scratch.ad_value(2191), 2.0), AdValue::mul(scratch.ad_value(2109), AdValue::sub(AdValue::add(AdValue::sub_from_scalar(1.0, scratch.ad_value(2202)), scratch.ad_value(2201)), AdValue::mul(scratch.ad_value(2115), AdValue::offset(scratch.ad_value(2204), 1.0))))));
        }

        if ((!(scratch.values[2228] != 0.0)) && (!(scratch.values[2229] != 0.0))) {
            scratch.store_ad(2207, &AdValue::sub(AdValue::square(scratch.ad_value(2191)), AdValue::mul(scratch.ad_value(2109), AdValue::sub(AdValue::add(AdValue::offset(AdValue::add(scratch.ad_value(2202), scratch.ad_value(2215)), (-1.0)), scratch.ad_value(2201)), AdValue::mul(scratch.ad_value(2115), AdValue::add(AdValue::offset(scratch.ad_value(2215), 1.0), scratch.ad_value(2203)))))));
        }

        if ((!(scratch.values[2228] != 0.0)) && (!(scratch.values[2229] != 0.0))) {
            scratch.store_ad(2191, &AdValue::sub_from_scalar(2.0, AdValue::mul(scratch.ad_value(2109), AdValue::sub(AdValue::add(scratch.ad_value(2202), scratch.ad_value(2201)), AdValue::mul(scratch.ad_value(2115), scratch.ad_value(2205))))));
        }

        if ((!(scratch.values[2228] != 0.0)) && (!(scratch.values[2229] != 0.0))) {
            scratch.store_ad(2191, &AdValue::sub(AdValue::square(scratch.ad_value(2206)), AdValue::scale(AdValue::mul(scratch.ad_value(2207), scratch.ad_value(2191)), 2.0)));
        }

        if ((!(scratch.values[2228] != 0.0)) && (!(scratch.values[2229] != 0.0))) {
            scratch.store_ad(2117, &AdValue::add(scratch.ad_value(2215), AdValue::scale(AdValue::div(scratch.ad_value(2207), AdValue::add(scratch.ad_value(2206), AdValue::sqrt(scratch.ad_value(2191)))), 2.0)));
        }

        scratch.values[2151] = scratch.values[2117];
        scratch.node_derivatives[2151] = scratch.node_derivatives[2117];
        scratch.branch_derivatives[2151] = scratch.branch_derivatives[2117];

        scratch.values[2159] = scratch.values[2117];
        scratch.node_derivatives[2159] = scratch.node_derivatives[2117];
        scratch.branch_derivatives[2159] = scratch.branch_derivatives[2117];

        scratch.values[2152] = 0.0;

        scratch.store_ad(865, &AdValue::scale(scratch.ad_value(2008), 3.912023005));

        scratch.values[2234] = if (scratch.values[2106] <= 0.0) { 1.0 } else { 0.0 };

        if (scratch.values[2234] != 0.0) {
            scratch.values[2127] = 0.0;
            scratch.node_derivatives[2127] = [0.0; Instance::NODE_COUNT];
            scratch.branch_derivatives[2127] = [0.0; Instance::BRANCH_COUNT];
        }

        if (scratch.values[2234] != 0.0) {
            scratch.store_ad(2164, &AdValue::sub(scratch.ad_value(2106), scratch.ad_value(2117)));
        }

        if (scratch.values[2234] != 0.0) {
            scratch.store_ad(2188, &AdValue::mul(scratch.ad_value(2164), scratch.ad_value(2008)));
        }

        if (scratch.values[2234] != 0.0) {
            scratch.values[2180] = scratch.values[2188];
            scratch.node_derivatives[2180] = scratch.node_derivatives[2188];
            scratch.branch_derivatives[2180] = scratch.branch_derivatives[2188];
        }

        if (scratch.values[2234] != 0.0) {
            scratch.values[866] = scratch.values[865];
            scratch.node_derivatives[866] = scratch.node_derivatives[865];
            scratch.branch_derivatives[866] = scratch.branch_derivatives[865];
        }

        if (scratch.values[2234] != 0.0) {
            scratch.values[2146] = scratch.values[850];
            scratch.node_derivatives[2146] = scratch.node_derivatives[850];
            scratch.branch_derivatives[2146] = scratch.branch_derivatives[850];
        }

        if (!(scratch.values[2234] != 0.0)) {
            scratch.values[2118] = 0.0;
            scratch.node_derivatives[2118] = [0.0; Instance::NODE_COUNT];
            scratch.branch_derivatives[2118] = [0.0; Instance::BRANCH_COUNT];
        }

        if (!(scratch.values[2234] != 0.0)) {
            scratch.store_ad(2086, &AdValue::div_from_scalar(1.0, AdValue::offset(AdValue::square(scratch.ad_value(2117)), 2.0)));
        }

        if (!(scratch.values[2234] != 0.0)) {
            scratch.store_ad(2119, &AdValue::mul(AdValue::square(scratch.ad_value(2117)), scratch.ad_value(2086)));
        }

        if (!(scratch.values[2234] != 0.0)) {
            scratch.store_ad(2120, &AdValue::scale(AdValue::mul(AdValue::mul(scratch.ad_value(2117), scratch.ad_value(2086)), scratch.ad_value(2086)), 4.0));
        }

        if (!(scratch.values[2234] != 0.0)) {
            scratch.store_ad(2121, &AdValue::mul(AdValue::mul(AdValue::sub(AdValue::scale(scratch.ad_value(2086), 8.0), AdValue::scale(scratch.ad_value(2119), 12.0)), scratch.ad_value(2086)), scratch.ad_value(2086)));
        }

        scratch.values[2235] = if (scratch.values[2117] < 230.25850929940458) { 1.0 } else { 0.0 };

        if ((!(scratch.values[2234] != 0.0)) && (scratch.values[2235] != 0.0)) {
            scratch.store_ad(2118, &AdValue::exp(scratch.ad_value(2117)));
        }

        if ((!(scratch.values[2234] != 0.0)) && (scratch.values[2235] != 0.0)) {
            scratch.store_ad(2122, &AdValue::div_from_scalar(1.0, scratch.ad_value(2118)));
        }

        if ((!(scratch.values[2234] != 0.0)) && (scratch.values[2235] != 0.0)) {
            scratch.store_ad(2118, &AdValue::mul(scratch.ad_value(2115), scratch.ad_value(2118)));
        }

        scratch.values[2236] = if (scratch.values[2117] > (scratch.values[2114] - 230.25850929940458)) { 1.0 } else { 0.0 };

        if (((!(scratch.values[2234] != 0.0)) && (!(scratch.values[2235] != 0.0))) && (scratch.values[2236] != 0.0)) {
            scratch.store_ad(2118, &AdValue::exp(AdValue::sub(scratch.ad_value(2117), scratch.ad_value(2114))));
        }

        if (((!(scratch.values[2234] != 0.0)) && (!(scratch.values[2235] != 0.0))) && (scratch.values[2236] != 0.0)) {
            scratch.store_ad(2122, &AdValue::div(scratch.ad_value(2115), scratch.ad_value(2118)));
        }

        if (((!(scratch.values[2234] != 0.0)) && (!(scratch.values[2235] != 0.0))) && (!(scratch.values[2236] != 0.0))) {
            scratch.store_ad(2118, &AdValue::div_from_scalar(1e-100, AdValue::offset(AdValue::mul(AdValue::offset(AdValue::sub(scratch.ad_value(2114), scratch.ad_value(2117)), (-230.25850929940458)), AdValue::offset(AdValue::scale(AdValue::mul(AdValue::offset(AdValue::sub(scratch.ad_value(2114), scratch.ad_value(2117)), (-230.25850929940458)), AdValue::offset(AdValue::scale(AdValue::offset(AdValue::sub(scratch.ad_value(2114), scratch.ad_value(2117)), (-230.25850929940458)), 0.3333333333333333), 1.0)), 0.5), 1.0)), 1.0)));
        }

        if (((!(scratch.values[2234] != 0.0)) && (!(scratch.values[2235] != 0.0))) && (!(scratch.values[2236] != 0.0))) {
            scratch.store_ad(2122, &AdValue::div_from_scalar(1e-100, AdValue::offset(AdValue::mul(AdValue::offset(scratch.ad_value(2117), (-230.25850929940458)), AdValue::offset(AdValue::scale(AdValue::mul(AdValue::offset(scratch.ad_value(2117), (-230.25850929940458)), AdValue::offset(AdValue::scale(AdValue::offset(scratch.ad_value(2117), (-230.25850929940458)), 0.3333333333333333), 1.0)), 0.5), 1.0)), 1.0)));
        }

        if (!(scratch.values[2234] != 0.0)) {
            scratch.store_ad(2123, &AdValue::sub(scratch.ad_value(2118), AdValue::mul(scratch.ad_value(2115), AdValue::add(AdValue::offset(scratch.ad_value(2117), 1.0), scratch.ad_value(2119)))));
        }

        scratch.values[2237] = if (scratch.values[2117] < 1e-5) { 1.0 } else { 0.0 };

        if ((!(scratch.values[2234] != 0.0)) && (scratch.values[2237] != 0.0)) {
            scratch.store_ad(2124, &AdValue::scale(AdValue::mul(AdValue::square(scratch.ad_value(2117)), AdValue::sub_from_scalar(1.0, AdValue::scale(AdValue::mul(scratch.ad_value(2117), AdValue::sub_from_scalar(1.0, AdValue::scale(scratch.ad_value(2117), 0.25))), 0.3333333333333333))), 0.5));
        }

        if ((!(scratch.values[2234] != 0.0)) && (scratch.values[2237] != 0.0)) {
            scratch.store_ad(2123, &AdValue::scale(AdValue::mul(AdValue::mul(AdValue::mul(AdValue::mul(scratch.ad_value(2115), scratch.ad_value(2117)), scratch.ad_value(2117)), scratch.ad_value(2117)), AdValue::offset(AdValue::scale(scratch.ad_value(2117), 1.75), 1.0)), 0.16666666666666666));
        }

        if ((!(scratch.values[2234] != 0.0)) && (scratch.values[2237] != 0.0)) {
            scratch.store_ad(2086, &AdValue::sqrt(AdValue::sub_from_scalar(1.0, AdValue::scale(AdValue::mul(scratch.ad_value(2117), AdValue::sub_from_scalar(1.0, AdValue::scale(scratch.ad_value(2117), 0.25))), 0.3333333333333333))));
        }

        if ((!(scratch.values[2234] != 0.0)) && (scratch.values[2237] != 0.0)) {
            scratch.store_ad(2166, &AdValue::scale(AdValue::mul(scratch.ad_value(2117), scratch.ad_value(2086)), 0.7071067811865475));
        }

        if ((!(scratch.values[2234] != 0.0)) && (scratch.values[2237] != 0.0)) {
            scratch.store_ad(2167, &AdValue::offset(AdValue::div(AdValue::mul(AdValue::scale(scratch.ad_value(2108), 0.7071067811865475), AdValue::add(AdValue::sub_from_scalar(1.0, AdValue::scale(scratch.ad_value(2117), 0.5)), AdValue::scale(AdValue::square(scratch.ad_value(2117)), 0.16666666666666666))), scratch.ad_value(2086)), 1.0));
        }

        if ((!(scratch.values[2234] != 0.0)) && (!(scratch.values[2237] != 0.0))) {
            scratch.store_ad(2124, &AdValue::add(AdValue::offset(scratch.ad_value(2117), (-1.0)), scratch.ad_value(2122)));
        }

        if ((!(scratch.values[2234] != 0.0)) && (!(scratch.values[2237] != 0.0))) {
            scratch.store_ad(2166, &AdValue::sqrt(scratch.ad_value(2124)));
        }

        if ((!(scratch.values[2234] != 0.0)) && (!(scratch.values[2237] != 0.0))) {
            scratch.store_ad(2167, &AdValue::offset(AdValue::scale(AdValue::div(AdValue::mul(scratch.ad_value(2108), AdValue::sub_from_scalar(1.0, scratch.ad_value(2122))), scratch.ad_value(2166)), 0.5), 1.0));
        }

        if (!(scratch.values[2234] != 0.0)) {
            scratch.values[2160] = scratch.values[2122];
            scratch.node_derivatives[2160] = scratch.node_derivatives[2122];
            scratch.branch_derivatives[2160] = scratch.branch_derivatives[2122];
        }

        if (!(scratch.values[2234] != 0.0)) {
            scratch.values[2157] = scratch.values[2160];
            scratch.node_derivatives[2157] = scratch.node_derivatives[2160];
            scratch.branch_derivatives[2157] = scratch.branch_derivatives[2160];
        }

        if (!(scratch.values[2234] != 0.0)) {
            scratch.values[2162] = scratch.values[2123];
            scratch.node_derivatives[2162] = scratch.node_derivatives[2123];
            scratch.branch_derivatives[2162] = scratch.branch_derivatives[2123];
        }

        if (!(scratch.values[2234] != 0.0)) {
            scratch.values[2158] = scratch.values[2162];
            scratch.node_derivatives[2158] = scratch.node_derivatives[2162];
            scratch.branch_derivatives[2158] = scratch.branch_derivatives[2162];
        }

        if (!(scratch.values[2234] != 0.0)) {
            scratch.store_ad(2125, &AdValue::div(AdValue::offset(AdValue::mul(AdValue::scale(scratch.ad_value(759), 0.2), scratch.ad_value(2105)), 1.0), AdValue::offset(AdValue::mul(scratch.ad_value(759), scratch.ad_value(2105)), 1.0)));
        }

        scratch.values[2238] = if (scratch.values[2123] > 1e-100) { 1.0 } else { 0.0 };

        if ((!(scratch.values[2234] != 0.0)) && (scratch.values[2238] != 0.0)) {
            scratch.store_ad(2126, &AdValue::mul(scratch.ad_value(2108), AdValue::sqrt(AdValue::add(scratch.ad_value(2124), scratch.ad_value(2123)))));
        }

        if ((!(scratch.values[2234] != 0.0)) && (scratch.values[2238] != 0.0)) {
            scratch.store_ad(2127, &AdValue::div(AdValue::mul(AdValue::mul(scratch.ad_value(2109), scratch.ad_value(2123)), scratch.ad_value(2008)), AdValue::add(scratch.ad_value(2126), AdValue::mul(scratch.ad_value(2108), scratch.ad_value(2166)))));
        }

        if ((!(scratch.values[2234] != 0.0)) && (scratch.values[2238] != 0.0)) {
            scratch.store_ad(2128, &AdValue::mul(AdValue::mul(scratch.ad_value(2166), scratch.ad_value(2108)), scratch.ad_value(2008)));
        }

        scratch.values[2239] = if (scratch.values[237] < 0.0) { 1.0 } else { 0.0 };

        if (((!(scratch.values[2234] != 0.0)) && (scratch.values[2238] != 0.0)) && (scratch.values[2239] != 0.0)) {
            scratch.store_ad(2129, &AdValue::div_from_scalar(1.0, AdValue::sub_from_scalar(1.0, AdValue::mul(scratch.ad_value(237), scratch.ad_value(2105)))));
        }

        if (((!(scratch.values[2234] != 0.0)) && (scratch.values[2238] != 0.0)) && (!(scratch.values[2239] != 0.0))) {
            scratch.store_ad(2129, &AdValue::offset(AdValue::mul(scratch.ad_value(237), scratch.ad_value(2105)), 1.0));
        }

        scratch.values[2240] = if (scratch.values[238] < 0.0) { 1.0 } else { 0.0 };

        if (((!(scratch.values[2234] != 0.0)) && (scratch.values[2238] != 0.0)) && (scratch.values[2240] != 0.0)) {
            scratch.store_ad(2086, &AdValue::sub_from_scalar(1.0, AdValue::mul(scratch.ad_value(238), scratch.ad_value(2127))));
        }

        if (((!(scratch.values[2234] != 0.0)) && (scratch.values[2238] != 0.0)) && (!(scratch.values[2240] != 0.0))) {
            scratch.store_ad(2086, &AdValue::div_from_scalar(1.0, AdValue::offset(AdValue::mul(scratch.ad_value(238), scratch.ad_value(2127)), 1.0)));
        }

        if ((!(scratch.values[2234] != 0.0)) && (scratch.values[2238] != 0.0)) {
            scratch.store_ad(2130, &AdValue::mul(scratch.ad_value(796), AdValue::mul(AdValue::mul(scratch.ad_value(2129), scratch.ad_value(2086)), scratch.ad_value(2127))));
        }

        if ((!(scratch.values[2234] != 0.0)) && (scratch.values[2238] != 0.0)) {
            scratch.store_ad(2131, &AdValue::mul(scratch.ad_value(813), AdValue::add(scratch.ad_value(2128), AdValue::mul(scratch.ad_value(814), scratch.ad_value(2127)))));
        }

        if ((!(scratch.values[2234] != 0.0)) && (scratch.values[2238] != 0.0)) {
            scratch.store_ad(2087, &AdValue::ln(AdValue::div(scratch.ad_value(2124), AdValue::offset(AdValue::add(scratch.ad_value(2124), scratch.ad_value(2123)), 1e-14))));
        }

        if ((!(scratch.values[2234] != 0.0)) && (scratch.values[2238] != 0.0)) {
            scratch.store_ad(2132, &AdValue::add(AdValue::pow(AdValue::mul(scratch.ad_value(2131), scratch.ad_value(755)), scratch.ad_value(756)), AdValue::mul(scratch.ad_value(757), AdValue::exp(AdValue::mul(AdValue::scale(scratch.ad_value(758), 0.5), scratch.ad_value(2087))))));
        }

        if ((!(scratch.values[2234] != 0.0)) && (scratch.values[2238] != 0.0)) {
            scratch.store_ad(2133, &AdValue::mul(AdValue::add(AdValue::offset(scratch.ad_value(2132), 1.0), scratch.ad_value(2130)), scratch.ad_value(2125)));
        }

        scratch.values[2241] = if (scratch.values[241] < 0.0) { 1.0 } else { 0.0 };

        if (((!(scratch.values[2234] != 0.0)) && (scratch.values[2238] != 0.0)) && (scratch.values[2241] != 0.0)) {
            scratch.store_ad(2134, &AdValue::div_from_scalar(1.0, AdValue::sub_from_scalar(1.0, AdValue::mul(scratch.ad_value(241), scratch.ad_value(2105)))));
        }

        if (((!(scratch.values[2234] != 0.0)) && (scratch.values[2238] != 0.0)) && (!(scratch.values[2241] != 0.0))) {
            scratch.store_ad(2134, &AdValue::offset(AdValue::mul(scratch.ad_value(241), scratch.ad_value(2105)), 1.0));
        }

        if ((!(scratch.values[2234] != 0.0)) && (scratch.values[2238] != 0.0)) {
            scratch.store_ad(2088, &AdValue::mul(scratch.ad_value(2127), scratch.ad_value(2134)));
        }

        if ((!(scratch.values[2234] != 0.0)) && (scratch.values[2238] != 0.0)) {
            scratch.store_ad(2135, &AdValue::scale(AdValue::div(scratch.ad_value(2088), AdValue::offset(scratch.ad_value(2088), 100.0)), 100.0));
        }

        scratch.values[2242] = if (scratch.values[242] < 0.0) { 1.0 } else { 0.0 };

        if (((!(scratch.values[2234] != 0.0)) && (scratch.values[2238] != 0.0)) && (scratch.values[2242] != 0.0)) {
            scratch.store_ad(2086, &AdValue::div_from_scalar(1.0, AdValue::sub_from_scalar(1.0, AdValue::mul(scratch.ad_value(242), scratch.ad_value(2135)))));
        }

        if (((!(scratch.values[2234] != 0.0)) && (scratch.values[2238] != 0.0)) && (!(scratch.values[2242] != 0.0))) {
            scratch.store_ad(2086, &AdValue::offset(AdValue::mul(scratch.ad_value(242), scratch.ad_value(2135)), 1.0));
        }

        if ((!(scratch.values[2234] != 0.0)) && (scratch.values[2238] != 0.0)) {
            scratch.store_ad(2136, &AdValue::mul(scratch.ad_value(2079), AdValue::div(scratch.ad_value(2086), scratch.ad_value(2133))));
        }

    }
}
