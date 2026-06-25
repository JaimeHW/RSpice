#![allow(dead_code, unused_assignments, unused_imports, unused_parens, unused_variables)]

use super::{AdValue, GeneratedEvalContext, GeneratedReactiveStamper, GeneratedStamper, Scratch, THERMAL_VOLTAGE_PER_K};
use super::super::state::Instance;

impl Instance {

    pub(super) fn stamp_transient_block_12(
        &mut self,
        ctx: &GeneratedEvalContext<'_>,
        stamper: &mut GeneratedStamper<'_>,
        scratch: &mut Scratch,
    ) {
        let _ = stamper;
        if (((((scratch.values[1320] != 0.0) && (scratch.values[1337] != 0.0)) && (!(scratch.values[1430] != 0.0))) && (!(scratch.values[1432] != 0.0))) && (scratch.values[1433] != 0.0)) {
            scratch.values[1361] = 0.0;
            scratch.node_derivatives[1361] = [0.0; Instance::NODE_COUNT];
            scratch.branch_derivatives[1361] = [0.0; Instance::BRANCH_COUNT];
        }

        if (((((scratch.values[1320] != 0.0) && (scratch.values[1337] != 0.0)) && (!(scratch.values[1430] != 0.0))) && (!(scratch.values[1432] != 0.0))) && (!(scratch.values[1433] != 0.0))) {
            scratch.store_ad(1361, &AdValue::scale(AdValue::add(AdValue::div(AdValue::mul(AdValue::square(scratch.ad_value(1360)), AdValue::ln(scratch.ad_value(1360))), AdValue::sub_from_scalar(1.0, scratch.ad_value(1360))), scratch.ad_value(1360)), (1.0 - (2.0 * scratch.values[384]))));
        }

        if ((((scratch.values[1320] != 0.0) && (scratch.values[1337] != 0.0)) && (!(scratch.values[1430] != 0.0))) && (!(scratch.values[1432] != 0.0))) {
            scratch.store_ad(1362, &AdValue::add(scratch.ad_value(1360), scratch.ad_value(1361)));
        }

        scratch.values[1434] = if (scratch.values[384] == 0.5) { 1.0 } else { 0.0 };

        if (((((scratch.values[1320] != 0.0) && (scratch.values[1337] != 0.0)) && (!(scratch.values[1430] != 0.0))) && (!(scratch.values[1432] != 0.0))) && (scratch.values[1434] != 0.0)) {
            scratch.store_ad(1356, &AdValue::sqrt(AdValue::scale(scratch.ad_value(1359), scratch.values[487])));
        }

        if (((((scratch.values[1320] != 0.0) && (scratch.values[1337] != 0.0)) && (!(scratch.values[1430] != 0.0))) && (!(scratch.values[1432] != 0.0))) && (!(scratch.values[1434] != 0.0))) {
            scratch.store_ad(1356, &AdValue::powf(AdValue::scale(scratch.ad_value(1359), scratch.values[487]), scratch.values[384]));
        }

        if ((((scratch.values[1320] != 0.0) && (scratch.values[1337] != 0.0)) && (!(scratch.values[1430] != 0.0))) && (!(scratch.values[1432] != 0.0))) {
            scratch.store_ad(1363, &AdValue::scale(scratch.ad_value(1356), scratch.values[481]));
        }

        if ((((scratch.values[1320] != 0.0) && (scratch.values[1337] != 0.0)) && (!(scratch.values[1430] != 0.0))) && (!(scratch.values[1432] != 0.0))) {
            scratch.store_ad(1364, &AdValue::scale(AdValue::mul(AdValue::offset(scratch.ad_value(1350), (-1.0)), scratch.ad_value(1363)), scratch.values[442]));
        }

        if ((((scratch.values[1320] != 0.0) && (scratch.values[1337] != 0.0)) && (!(scratch.values[1430] != 0.0))) && (!(scratch.values[1432] != 0.0))) {
            scratch.store_ad(1358, &AdValue::scale(AdValue::mul(scratch.ad_value(1364), scratch.ad_value(1362)), scratch.values[395]));
        }

        scratch.values[1435] = if (scratch.values[398] == 0.0) { 1.0 } else { 0.0 };

        if ((((scratch.values[1320] != 0.0) && (scratch.values[1337] != 0.0)) && (!(scratch.values[1430] != 0.0))) && (scratch.values[1435] != 0.0)) {
            scratch.values[1365] = 0.0;
            scratch.node_derivatives[1365] = [0.0; Instance::NODE_COUNT];
            scratch.branch_derivatives[1365] = [0.0; Instance::BRANCH_COUNT];
        }

        if ((((scratch.values[1320] != 0.0) && (scratch.values[1337] != 0.0)) && (!(scratch.values[1430] != 0.0))) && (!(scratch.values[1435] != 0.0))) {
            scratch.store_ad(1366, &AdValue::scale(AdValue::div(AdValue::scale(scratch.ad_value(1363), scratch.values[466]), scratch.ad_value(1359)), scratch.values[496]));
        }

        if ((((scratch.values[1320] != 0.0) && (scratch.values[1337] != 0.0)) && (!(scratch.values[1430] != 0.0))) && (!(scratch.values[1435] != 0.0))) {
            scratch.store_ad(1367, &AdValue::div_from_scalar((0.666666666666667 * scratch.values[493]), scratch.ad_value(1366)));
        }

        if ((((scratch.values[1320] != 0.0) && (scratch.values[1337] != 0.0)) && (!(scratch.values[1430] != 0.0))) && (!(scratch.values[1435] != 0.0))) {
            scratch.store_ad(1368, &AdValue::square(scratch.ad_value(1367)));
        }

        if ((((scratch.values[1320] != 0.0) && (scratch.values[1337] != 0.0)) && (!(scratch.values[1430] != 0.0))) && (!(scratch.values[1435] != 0.0))) {
            scratch.store_ad(1369, &AdValue::sqrt(AdValue::div(AdValue::square(scratch.ad_value(1368)), AdValue::offset(AdValue::square(scratch.ad_value(1368)), 1.0))));
        }

        if ((((scratch.values[1320] != 0.0) && (scratch.values[1337] != 0.0)) && (!(scratch.values[1430] != 0.0))) && (!(scratch.values[1435] != 0.0))) {
            scratch.store_ad(1370, &AdValue::sqrt(AdValue::abs(scratch.ad_value(1369))));
        }

        if ((((scratch.values[1320] != 0.0) && (scratch.values[1337] != 0.0)) && (!(scratch.values[1430] != 0.0))) && (!(scratch.values[1435] != 0.0))) {
            scratch.store_ad(1371, &AdValue::mul(scratch.ad_value(1369), scratch.ad_value(1370)));
        }

        scratch.values[1436] = if (((-scratch.values[384]) * scratch.values[469]) == (-1.0)) { 1.0 } else { 0.0 };

        if (((((scratch.values[1320] != 0.0) && (scratch.values[1337] != 0.0)) && (!(scratch.values[1430] != 0.0))) && (!(scratch.values[1435] != 0.0))) && (scratch.values[1436] != 0.0)) {
            scratch.store_ad(1372, &AdValue::div_from_scalar(1.0, AdValue::offset(AdValue::mul(scratch.ad_value(1366), scratch.ad_value(1371)), 1.0)));
        }

        if (((((scratch.values[1320] != 0.0) && (scratch.values[1337] != 0.0)) && (!(scratch.values[1430] != 0.0))) && (!(scratch.values[1435] != 0.0))) && (!(scratch.values[1436] != 0.0))) {
            scratch.store_ad(1372, &AdValue::powf(AdValue::offset(AdValue::mul(scratch.ad_value(1366), scratch.ad_value(1371)), 1.0), ((-scratch.values[384]) * scratch.values[469])));
        }

        if ((((scratch.values[1320] != 0.0) && (scratch.values[1337] != 0.0)) && (!(scratch.values[1430] != 0.0))) && (!(scratch.values[1435] != 0.0))) {
            scratch.store_ad(1373, &AdValue::div(AdValue::mul(scratch.ad_value(1362), scratch.ad_value(1372)), AdValue::add(scratch.ad_value(1362), scratch.ad_value(1372))));
        }

        if ((((scratch.values[1320] != 0.0) && (scratch.values[1337] != 0.0)) && (!(scratch.values[1430] != 0.0))) && (!(scratch.values[1435] != 0.0))) {
            scratch.store_ad(1374, &AdValue::sqrt(AdValue::scale(AdValue::div(scratch.ad_value(1366), scratch.ad_value(1370)), 0.375)));
        }

        if ((((scratch.values[1320] != 0.0) && (scratch.values[1337] != 0.0)) && (!(scratch.values[1430] != 0.0))) && (!(scratch.values[1435] != 0.0))) {
            scratch.store_ad(1375, &AdValue::sub(AdValue::scale(AdValue::mul(scratch.ad_value(1367), scratch.ad_value(1370)), 2.0), scratch.ad_value(1369)));
        }

        if ((((scratch.values[1320] != 0.0) && (scratch.values[1337] != 0.0)) && (!(scratch.values[1430] != 0.0))) && (!(scratch.values[1435] != 0.0))) {
            scratch.store_ad(1376, &AdValue::add(AdValue::sub(AdValue::mul(AdValue::scale(scratch.ad_value(1367), scratch.values[493]), scratch.ad_value(1370)), AdValue::scale(scratch.ad_value(1369), scratch.values[493])), AdValue::scale(AdValue::mul(scratch.ad_value(1366), scratch.ad_value(1371)), 0.5)));
        }

        if ((((scratch.values[1320] != 0.0) && (scratch.values[1337] != 0.0)) && (!(scratch.values[1430] != 0.0))) && (!(scratch.values[1435] != 0.0))) {
            scratch.store_ad(1377, &AdValue::mul(AdValue::offset(scratch.ad_value(1375), (-1.0)), scratch.ad_value(1374)));
        }

        if ((((scratch.values[1320] != 0.0) && (scratch.values[1337] != 0.0)) && (!(scratch.values[1430] != 0.0))) && (!(scratch.values[1435] != 0.0))) {
            scratch.store_ad(1338, &AdValue::square(scratch.ad_value(1377)));
        }

        scratch.values[1437] = if (scratch.values[1377] > 0.0) { 1.0 } else { 0.0 };

        if (((((scratch.values[1320] != 0.0) && (scratch.values[1337] != 0.0)) && (!(scratch.values[1430] != 0.0))) && (!(scratch.values[1435] != 0.0))) && (scratch.values[1437] != 0.0)) {
            scratch.store_ad(1339, &AdValue::div_from_scalar(1.0, AdValue::offset(AdValue::scale(scratch.ad_value(1377), scratch.values[428]), 1.0)));
        }

        if (((((scratch.values[1320] != 0.0) && (scratch.values[1337] != 0.0)) && (!(scratch.values[1430] != 0.0))) && (!(scratch.values[1435] != 0.0))) && (!(scratch.values[1437] != 0.0))) {
            scratch.store_ad(1339, &AdValue::div_from_scalar(1.0, AdValue::sub_from_scalar(1.0, AdValue::scale(scratch.ad_value(1377), scratch.values[428]))));
        }

        scratch.values[1438] = if (((-scratch.values[1338]) + scratch.values[1376]) > (-230.25850929940458)) { 1.0 } else { 0.0 };

        if (((((scratch.values[1320] != 0.0) && (scratch.values[1337] != 0.0)) && (!(scratch.values[1430] != 0.0))) && (!(scratch.values[1435] != 0.0))) && (scratch.values[1438] != 0.0)) {
            scratch.store_ad(1356, &AdValue::exp(AdValue::sub(scratch.ad_value(1376), scratch.ad_value(1338))));
        }

        if (((((scratch.values[1320] != 0.0) && (scratch.values[1337] != 0.0)) && (!(scratch.values[1430] != 0.0))) && (!(scratch.values[1435] != 0.0))) && (!(scratch.values[1438] != 0.0))) {
            let assign15080_ad_e13279: AdValue = AdValue::div_from_scalar(1e-100, AdValue::offset(AdValue::mul(AdValue::sub_from_scalar((-230.25850929940458), AdValue::sub(scratch.ad_value(1376), scratch.ad_value(1338))), AdValue::offset(AdValue::scale(AdValue::mul(AdValue::sub_from_scalar((-230.25850929940458), AdValue::sub(scratch.ad_value(1376), scratch.ad_value(1338))), AdValue::offset(AdValue::scale(AdValue::sub_from_scalar((-230.25850929940458), AdValue::sub(scratch.ad_value(1376), scratch.ad_value(1338))), 0.3333333333333333), 1.0)), 0.5), 1.0)), 1.0));
            scratch.store_ad(1356, &assign15080_ad_e13279);
        }

        if ((((scratch.values[1320] != 0.0) && (scratch.values[1337] != 0.0)) && (!(scratch.values[1430] != 0.0))) && (!(scratch.values[1435] != 0.0))) {
            scratch.store_ad(1340, &AdValue::mul(AdValue::add(AdValue::add(AdValue::scale(scratch.ad_value(1339), 0.29214664), AdValue::scale(AdValue::square(scratch.ad_value(1339)), scratch.values[429])), AdValue::scale(AdValue::mul(AdValue::square(scratch.ad_value(1339)), scratch.ad_value(1339)), scratch.values[430])), scratch.ad_value(1356)));
        }

        scratch.values[1439] = if (scratch.values[1377] > 0.0) { 1.0 } else { 0.0 };

        if (((((scratch.values[1320] != 0.0) && (scratch.values[1337] != 0.0)) && (!(scratch.values[1430] != 0.0))) && (!(scratch.values[1435] != 0.0))) && (scratch.values[1439] != 0.0)) {
            scratch.values[1378] = scratch.values[1340];
            scratch.node_derivatives[1378] = scratch.node_derivatives[1340];
            scratch.branch_derivatives[1378] = scratch.branch_derivatives[1340];
        }

        scratch.values[1440] = if (scratch.values[1376] > (-230.25850929940458)) { 1.0 } else { 0.0 };

        if ((((((scratch.values[1320] != 0.0) && (scratch.values[1337] != 0.0)) && (!(scratch.values[1430] != 0.0))) && (!(scratch.values[1435] != 0.0))) && (!(scratch.values[1439] != 0.0))) && (scratch.values[1440] != 0.0)) {
            scratch.store_ad(1356, &AdValue::exp(scratch.ad_value(1376)));
        }

        if ((((((scratch.values[1320] != 0.0) && (scratch.values[1337] != 0.0)) && (!(scratch.values[1430] != 0.0))) && (!(scratch.values[1435] != 0.0))) && (!(scratch.values[1439] != 0.0))) && (!(scratch.values[1440] != 0.0))) {
            scratch.store_ad(1356, &AdValue::div_from_scalar(1e-100, AdValue::offset(AdValue::mul(AdValue::sub_from_scalar((-230.25850929940458), scratch.ad_value(1376)), AdValue::offset(AdValue::scale(AdValue::mul(AdValue::sub_from_scalar((-230.25850929940458), scratch.ad_value(1376)), AdValue::offset(AdValue::scale(AdValue::sub_from_scalar((-230.25850929940458), scratch.ad_value(1376)), 0.3333333333333333), 1.0)), 0.5), 1.0)), 1.0)));
        }

        if (((((scratch.values[1320] != 0.0) && (scratch.values[1337] != 0.0)) && (!(scratch.values[1430] != 0.0))) && (!(scratch.values[1435] != 0.0))) && (!(scratch.values[1439] != 0.0))) {
            scratch.store_ad(1378, &AdValue::sub(AdValue::scale(scratch.ad_value(1356), 2.0), scratch.ad_value(1340)));
        }

        if ((((scratch.values[1320] != 0.0) && (scratch.values[1337] != 0.0)) && (!(scratch.values[1430] != 0.0))) && (!(scratch.values[1435] != 0.0))) {
            scratch.store_ad(1379, &AdValue::scale(AdValue::div(AdValue::scale(scratch.ad_value(1378), scratch.values[493]), scratch.ad_value(1374)), (1.772453850905516 * 0.5)));
        }

        if ((((scratch.values[1320] != 0.0) && (scratch.values[1337] != 0.0)) && (!(scratch.values[1430] != 0.0))) && (!(scratch.values[1435] != 0.0))) {
            scratch.store_ad(1365, &AdValue::scale(AdValue::mul(AdValue::mul(scratch.ad_value(1364), scratch.ad_value(1379)), scratch.ad_value(1373)), scratch.values[398]));
        }

        scratch.values[1441] = if (scratch.values[404] == 0.0) { 1.0 } else { 0.0 };

        if ((((scratch.values[1320] != 0.0) && (scratch.values[1337] != 0.0)) && (!(scratch.values[1430] != 0.0))) && (scratch.values[1441] != 0.0)) {
            scratch.values[1380] = 0.0;
            scratch.node_derivatives[1380] = [0.0; Instance::NODE_COUNT];
            scratch.branch_derivatives[1380] = [0.0; Instance::BRANCH_COUNT];
        }

        scratch.values[1442] = if (scratch.values[384] == 0.5) { 1.0 } else { 0.0 };

        if (((((scratch.values[1320] != 0.0) && (scratch.values[1337] != 0.0)) && (!(scratch.values[1430] != 0.0))) && (!(scratch.values[1441] != 0.0))) && (scratch.values[1442] != 0.0)) {
            scratch.store_ad(1356, &AdValue::sqrt(AdValue::scale(AdValue::sub_from_scalar(scratch.values[381], scratch.ad_value(1354)), scratch.values[487])));
        }

        if (((((scratch.values[1320] != 0.0) && (scratch.values[1337] != 0.0)) && (!(scratch.values[1430] != 0.0))) && (!(scratch.values[1441] != 0.0))) && (!(scratch.values[1442] != 0.0))) {
            scratch.store_ad(1356, &AdValue::powf(AdValue::scale(AdValue::sub_from_scalar(scratch.values[381], scratch.ad_value(1354)), scratch.values[487]), scratch.values[384]));
        }

        if ((((scratch.values[1320] != 0.0) && (scratch.values[1337] != 0.0)) && (!(scratch.values[1430] != 0.0))) && (!(scratch.values[1441] != 0.0))) {
            scratch.store_ad(1381, &AdValue::scale(AdValue::div(AdValue::scale(AdValue::sub_from_scalar(scratch.values[381], scratch.ad_value(1354)), scratch.values[484]), scratch.ad_value(1356)), scratch.values[469]));
        }

        scratch.values[1443] = if (((((-scratch.values[499]) / scratch.values[1381])) as f64).abs() < 230.25850929940458) { 1.0 } else { 0.0 };

        if (((((scratch.values[1320] != 0.0) && (scratch.values[1337] != 0.0)) && (!(scratch.values[1430] != 0.0))) && (!(scratch.values[1441] != 0.0))) && (scratch.values[1443] != 0.0)) {
            scratch.store_ad(1356, &AdValue::exp(AdValue::div(AdValue::neg(scratch.ad_value(499)), scratch.ad_value(1381))));
        }

        scratch.values[1444] = if (((-scratch.values[499]) / scratch.values[1381]) < 0.0) { 1.0 } else { 0.0 };

        if ((((((scratch.values[1320] != 0.0) && (scratch.values[1337] != 0.0)) && (!(scratch.values[1430] != 0.0))) && (!(scratch.values[1441] != 0.0))) && (!(scratch.values[1443] != 0.0))) && (scratch.values[1444] != 0.0)) {
            let assign15270_ad_e13606: AdValue = AdValue::offset(AdValue::mul(AdValue::sub_from_scalar((-230.25850929940458), AdValue::div(AdValue::neg(scratch.ad_value(499)), scratch.ad_value(1381))), AdValue::offset(AdValue::scale(AdValue::mul(AdValue::sub_from_scalar((-230.25850929940458), AdValue::div(AdValue::neg(scratch.ad_value(499)), scratch.ad_value(1381))), AdValue::offset(AdValue::scale(AdValue::sub_from_scalar((-230.25850929940458), AdValue::div(AdValue::neg(scratch.ad_value(499)), scratch.ad_value(1381))), 0.3333333333333333), 1.0)), 0.5), 1.0)), 1.0);
            scratch.store_ad(1356, &AdValue::div_from_scalar(1e-100, assign15270_ad_e13606));
        }

        if ((((((scratch.values[1320] != 0.0) && (scratch.values[1337] != 0.0)) && (!(scratch.values[1430] != 0.0))) && (!(scratch.values[1441] != 0.0))) && (!(scratch.values[1443] != 0.0))) && (!(scratch.values[1444] != 0.0))) {
            let assign15280_ad_e13656: AdValue = AdValue::scale(AdValue::offset(AdValue::mul(AdValue::offset(AdValue::div(AdValue::neg(scratch.ad_value(499)), scratch.ad_value(1381)), (-230.25850929940458)), AdValue::offset(AdValue::scale(AdValue::mul(AdValue::offset(AdValue::div(AdValue::neg(scratch.ad_value(499)), scratch.ad_value(1381)), (-230.25850929940458)), AdValue::offset(AdValue::scale(AdValue::offset(AdValue::div(AdValue::neg(scratch.ad_value(499)), scratch.ad_value(1381)), (-230.25850929940458)), 0.3333333333333333), 1.0)), 0.5), 1.0)), 1.0), 1e100);
            scratch.store_ad(1356, &assign15280_ad_e13656);
        }

        if ((((scratch.values[1320] != 0.0) && (scratch.values[1337] != 0.0)) && (!(scratch.values[1430] != 0.0))) && (!(scratch.values[1441] != 0.0))) {
            scratch.store_ad(1380, &AdValue::scale(AdValue::mul(AdValue::mul(AdValue::mul(scratch.ad_value(523), scratch.ad_value(1381)), scratch.ad_value(1381)), scratch.ad_value(1356)), scratch.values[404]));
        }

        scratch.values[1445] = if (scratch.values[413] > 1000.0) { 1.0 } else { 0.0 };

        if ((((scratch.values[1320] != 0.0) && (scratch.values[1337] != 0.0)) && (!(scratch.values[1430] != 0.0))) && (scratch.values[1445] != 0.0)) {
            scratch.values[1382] = 1.0;
            scratch.node_derivatives[1382] = [0.0; Instance::NODE_COUNT];
            scratch.branch_derivatives[1382] = [0.0; Instance::BRANCH_COUNT];
        }

        scratch.values[1446] = if (scratch.values[1355] > ((-scratch.values[500]) * scratch.values[413])) { 1.0 } else { 0.0 };

        scratch.values[1447] = if (scratch.values[416] == 4.0) { 1.0 } else { 0.0 };

        if ((((((scratch.values[1320] != 0.0) && (scratch.values[1337] != 0.0)) && (!(scratch.values[1430] != 0.0))) && (!(scratch.values[1445] != 0.0))) && (scratch.values[1446] != 0.0)) && (scratch.values[1447] != 0.0)) {
            scratch.store_ad(1356, &AdValue::mul(AdValue::mul(AdValue::mul(AdValue::scale(scratch.ad_value(1355), scratch.values[506]), AdValue::scale(scratch.ad_value(1355), scratch.values[506])), AdValue::scale(scratch.ad_value(1355), scratch.values[506])), AdValue::scale(scratch.ad_value(1355), scratch.values[506])));
        }

        if ((((((scratch.values[1320] != 0.0) && (scratch.values[1337] != 0.0)) && (!(scratch.values[1430] != 0.0))) && (!(scratch.values[1445] != 0.0))) && (scratch.values[1446] != 0.0)) && (!(scratch.values[1447] != 0.0))) {
            scratch.store_ad(1356, &AdValue::powf(AdValue::abs(AdValue::scale(scratch.ad_value(1355), scratch.values[506])), scratch.values[416]));
        }

        if (((((scratch.values[1320] != 0.0) && (scratch.values[1337] != 0.0)) && (!(scratch.values[1430] != 0.0))) && (!(scratch.values[1445] != 0.0))) && (scratch.values[1446] != 0.0)) {
            scratch.store_ad(1382, &AdValue::div_from_scalar(1.0, AdValue::sub_from_scalar(1.0, scratch.ad_value(1356))));
        }

        if (((((scratch.values[1320] != 0.0) && (scratch.values[1337] != 0.0)) && (!(scratch.values[1430] != 0.0))) && (!(scratch.values[1445] != 0.0))) && (!(scratch.values[1446] != 0.0))) {
            scratch.store_ad(1382, &AdValue::offset(AdValue::scale(AdValue::offset(scratch.ad_value(1355), (scratch.values[500] * scratch.values[413])), scratch.values[509]), scratch.values[503]));
        }

        if (((scratch.values[1320] != 0.0) && (scratch.values[1337] != 0.0)) && (!(scratch.values[1430] != 0.0))) {
            scratch.store_ad(1385, &AdValue::mul(AdValue::add(AdValue::add(AdValue::add(scratch.ad_value(1357), scratch.ad_value(1358)), scratch.ad_value(1365)), scratch.ad_value(1380)), scratch.ad_value(1382)));
        }

        if ((scratch.values[1320] != 0.0) && (scratch.values[1337] != 0.0)) {
            scratch.store_ad(513, &AdValue::add(AdValue::add(AdValue::mul(scratch.ad_value(697), scratch.ad_value(1383)), AdValue::mul(scratch.ad_value(698), scratch.ad_value(1384))), AdValue::mul(scratch.ad_value(699), scratch.ad_value(1385))));
        }

        if ((scratch.values[1320] != 0.0) && (scratch.values[1337] != 0.0)) {
            scratch.values[1354] = 0.0;
            scratch.node_derivatives[1354] = [0.0; Instance::NODE_COUNT];
            scratch.branch_derivatives[1354] = [0.0; Instance::BRANCH_COUNT];
        }

        if ((scratch.values[1320] != 0.0) && (scratch.values[1337] != 0.0)) {
            scratch.values[1351] = 0.0;
            scratch.node_derivatives[1351] = [0.0; Instance::NODE_COUNT];
            scratch.branch_derivatives[1351] = [0.0; Instance::BRANCH_COUNT];
        }

        scratch.values[1448] = if !(((scratch.values[697] == 0.0) && (scratch.values[698] == 0.0)) && (scratch.values[699] == 0.0)) { 1.0 } else { 0.0 };

        if (((scratch.values[1320] != 0.0) && (scratch.values[1337] != 0.0)) && (scratch.values[1448] != 0.0)) {
            scratch.store_ad(1341, &AdValue::mul(AdValue::scale(scratch.ad_value(708), 4.0), scratch.ad_value(708)));
        }

        if (((scratch.values[1320] != 0.0) && (scratch.values[1337] != 0.0)) && (scratch.values[1448] != 0.0)) {
            scratch.store_ad(1342, &AdValue::div(scratch.ad_value(708), scratch.ad_value(709)));
        }

        if (((scratch.values[1320] != 0.0) && (scratch.values[1337] != 0.0)) && (scratch.values[1448] != 0.0)) {
            scratch.store_ad(1343, &AdValue::add(scratch.ad_value(524), AdValue::mul(scratch.ad_value(708), scratch.ad_value(1342))));
        }

        if (((scratch.values[1320] != 0.0) && (scratch.values[1337] != 0.0)) && (scratch.values[1448] != 0.0)) {
            scratch.store_ad(1344, &AdValue::add(scratch.ad_value(709), scratch.ad_value(1343)));
        }

        if (((scratch.values[1320] != 0.0) && (scratch.values[1337] != 0.0)) && (scratch.values[1448] != 0.0)) {
            scratch.store_ad(1345, &AdValue::sub(scratch.ad_value(709), scratch.ad_value(1343)));
        }

        if (((scratch.values[1320] != 0.0) && (scratch.values[1337] != 0.0)) && (scratch.values[1448] != 0.0)) {
            scratch.store_ad(1346, &AdValue::sqrt(AdValue::add(AdValue::square(scratch.ad_value(1345)), scratch.ad_value(1341))));
        }

        if (((scratch.values[1320] != 0.0) && (scratch.values[1337] != 0.0)) && (scratch.values[1448] != 0.0)) {
            scratch.store_ad(1348, &AdValue::scale(AdValue::div(AdValue::mul(scratch.ad_value(524), scratch.ad_value(709)), AdValue::add(scratch.ad_value(1344), scratch.ad_value(1346))), 2.0));
        }

        scratch.values[1449] = if (scratch.values[524] < scratch.values[705]) { 1.0 } else { 0.0 };

        scratch.values[1450] = if ((((0.5 * (scratch.values[524] * scratch.values[427]))) as f64).abs() < 230.25850929940458) { 1.0 } else { 0.0 };

        if (((((scratch.values[1320] != 0.0) && (scratch.values[1337] != 0.0)) && (scratch.values[1448] != 0.0)) && (scratch.values[1449] != 0.0)) && (scratch.values[1450] != 0.0)) {
            scratch.store_ad(1350, &AdValue::exp(AdValue::scale(scratch.ad_value(524), (scratch.values[427] * 0.5))));
        }

        scratch.values[1451] = if ((0.5 * (scratch.values[524] * scratch.values[427])) < 0.0) { 1.0 } else { 0.0 };

        if ((((((scratch.values[1320] != 0.0) && (scratch.values[1337] != 0.0)) && (scratch.values[1448] != 0.0)) && (scratch.values[1449] != 0.0)) && (!(scratch.values[1450] != 0.0))) && (scratch.values[1451] != 0.0)) {
            let assign15540_ad_e14019: AdValue = AdValue::div_from_scalar(1e-100, AdValue::offset(AdValue::mul(AdValue::sub_from_scalar((-230.25850929940458), AdValue::scale(scratch.ad_value(524), (scratch.values[427] * 0.5))), AdValue::offset(AdValue::scale(AdValue::mul(AdValue::sub_from_scalar((-230.25850929940458), AdValue::scale(scratch.ad_value(524), (scratch.values[427] * 0.5))), AdValue::offset(AdValue::scale(AdValue::sub_from_scalar((-230.25850929940458), AdValue::scale(scratch.ad_value(524), (scratch.values[427] * 0.5))), 0.3333333333333333), 1.0)), 0.5), 1.0)), 1.0));
            scratch.store_ad(1350, &assign15540_ad_e14019);
        }

        if ((((((scratch.values[1320] != 0.0) && (scratch.values[1337] != 0.0)) && (scratch.values[1448] != 0.0)) && (scratch.values[1449] != 0.0)) && (!(scratch.values[1450] != 0.0))) && (!(scratch.values[1451] != 0.0))) {
            scratch.store_ad(1350, &AdValue::scale(AdValue::offset(AdValue::mul(AdValue::offset(AdValue::scale(scratch.ad_value(524), (scratch.values[427] * 0.5)), (-230.25850929940458)), AdValue::offset(AdValue::scale(AdValue::mul(AdValue::offset(AdValue::scale(scratch.ad_value(524), (scratch.values[427] * 0.5)), (-230.25850929940458)), AdValue::offset(AdValue::scale(AdValue::offset(AdValue::scale(scratch.ad_value(524), (scratch.values[427] * 0.5)), (-230.25850929940458)), 0.3333333333333333), 1.0)), 0.5), 1.0)), 1.0), 1e100));
        }

        if ((((scratch.values[1320] != 0.0) && (scratch.values[1337] != 0.0)) && (scratch.values[1448] != 0.0)) && (scratch.values[1449] != 0.0)) {
            scratch.store_ad(1347, &AdValue::square(scratch.ad_value(1350)));
        }

        if ((((scratch.values[1320] != 0.0) && (scratch.values[1337] != 0.0)) && (scratch.values[1448] != 0.0)) && (!(scratch.values[1449] != 0.0))) {
            scratch.store_ad(1347, &AdValue::mul(AdValue::offset(AdValue::scale(AdValue::sub(scratch.ad_value(524), scratch.ad_value(705)), scratch.values[427]), 1.0), scratch.ad_value(706)));
        }

        if ((((scratch.values[1320] != 0.0) && (scratch.values[1337] != 0.0)) && (scratch.values[1448] != 0.0)) && (!(scratch.values[1449] != 0.0))) {
            scratch.store_ad(1350, &AdValue::sqrt(scratch.ad_value(1347)));
        }

        if (((scratch.values[1320] != 0.0) && (scratch.values[1337] != 0.0)) && (scratch.values[1448] != 0.0)) {
            scratch.store_ad(1347, &AdValue::offset(scratch.ad_value(1347), (-1.0)));
        }

        if (((scratch.values[1320] != 0.0) && (scratch.values[1337] != 0.0)) && (scratch.values[1448] != 0.0)) {
            scratch.store_ad(1349, &AdValue::div_from_scalar(1.0, scratch.ad_value(1350)));
        }

        scratch.values[1452] = if (scratch.values[524] > 0.0) { 1.0 } else { 0.0 };

        if ((((scratch.values[1320] != 0.0) && (scratch.values[1337] != 0.0)) && (scratch.values[1448] != 0.0)) && (scratch.values[1452] != 0.0)) {
            scratch.store_ad(1351, &AdValue::scale(AdValue::ln(AdValue::add(AdValue::offset(scratch.ad_value(1349), 2.0), AdValue::sqrt(AdValue::mul(AdValue::offset(scratch.ad_value(1349), 1.0), AdValue::offset(scratch.ad_value(1349), 3.0))))), (scratch.values[426] * 2.0)));
        }

        if ((((scratch.values[1320] != 0.0) && (scratch.values[1337] != 0.0)) && (scratch.values[1448] != 0.0)) && (!(scratch.values[1452] != 0.0))) {
            scratch.store_ad(1351, &AdValue::sub(AdValue::scale(AdValue::ln(AdValue::add(AdValue::offset(AdValue::scale(scratch.ad_value(1350), 2.0), 1.0), AdValue::sqrt(AdValue::mul(AdValue::offset(scratch.ad_value(1350), 1.0), AdValue::offset(AdValue::scale(scratch.ad_value(1350), 3.0), 1.0))))), (scratch.values[426] * 2.0)), scratch.ad_value(524)));
        }

        if (((scratch.values[1320] != 0.0) && (scratch.values[1337] != 0.0)) && (scratch.values[1448] != 0.0)) {
            scratch.store_ad(1352, &AdValue::sub(scratch.ad_value(707), scratch.ad_value(1351)));
        }

        if (((scratch.values[1320] != 0.0) && (scratch.values[1337] != 0.0)) && (scratch.values[1448] != 0.0)) {
            scratch.store_ad(1353, &AdValue::scale(AdValue::sub(AdValue::add(scratch.ad_value(524), scratch.ad_value(1352)), AdValue::sqrt(AdValue::offset(AdValue::mul(AdValue::sub(scratch.ad_value(524), scratch.ad_value(1352)), AdValue::sub(scratch.ad_value(524), scratch.ad_value(1352))), ((4.0 * scratch.values[426]) * scratch.values[426])))), 0.5));
        }

        if (((scratch.values[1320] != 0.0) && (scratch.values[1337] != 0.0)) && (scratch.values[1448] != 0.0)) {
            scratch.store_ad(1354, &AdValue::scale(AdValue::sub(AdValue::add(scratch.ad_value(524), scratch.ad_value(710)), AdValue::sqrt(AdValue::offset(AdValue::mul(AdValue::sub(scratch.ad_value(524), scratch.ad_value(710)), AdValue::sub(scratch.ad_value(524), scratch.ad_value(710))), ((4.0 * scratch.values[424]) * scratch.values[424])))), 0.5));
        }

        if (((scratch.values[1320] != 0.0) && (scratch.values[1337] != 0.0)) && (scratch.values[1448] != 0.0)) {
            scratch.store_ad(1355, &AdValue::scale(AdValue::sub(scratch.ad_value(524), AdValue::sqrt(AdValue::offset(AdValue::mul(scratch.ad_value(524), scratch.ad_value(524)), ((4.0 * 1e-6) * 1e-6)))), 0.5));
        }

        scratch.values[1453] = if (scratch.values[697] == 0.0) { 1.0 } else { 0.0 };

        if (((scratch.values[1320] != 0.0) && (scratch.values[1337] != 0.0)) && (scratch.values[1453] != 0.0)) {
            scratch.values[1383] = 0.0;
            scratch.node_derivatives[1383] = [0.0; Instance::NODE_COUNT];
            scratch.branch_derivatives[1383] = [0.0; Instance::BRANCH_COUNT];
        }

        scratch.values[1454] = if (scratch.values[464] == 0.5) { 1.0 } else { 0.0 };

        if ((((scratch.values[1320] != 0.0) && (scratch.values[1337] != 0.0)) && (!(scratch.values[1453] != 0.0))) && (scratch.values[1454] != 0.0)) {
            scratch.store_ad(1356, &AdValue::sqrt(AdValue::sub_from_scalar(1.0, AdValue::scale(scratch.ad_value(1348), scratch.values[461]))));
        }

        if ((((scratch.values[1320] != 0.0) && (scratch.values[1337] != 0.0)) && (!(scratch.values[1453] != 0.0))) && (!(scratch.values[1454] != 0.0))) {
            scratch.store_ad(1356, &AdValue::powf(AdValue::sub_from_scalar(1.0, AdValue::scale(scratch.ad_value(1348), scratch.values[461])), scratch.values[464]));
        }

        if (((scratch.values[1320] != 0.0) && (scratch.values[1337] != 0.0)) && (!(scratch.values[1453] != 0.0))) {
            scratch.store_ad(1357, &AdValue::scale(scratch.ad_value(1347), scratch.values[443]));
        }

        scratch.values[1455] = if ((scratch.values[393] == 0.0) && (scratch.values[396] == 0.0)) { 1.0 } else { 0.0 };

        if ((((scratch.values[1320] != 0.0) && (scratch.values[1337] != 0.0)) && (!(scratch.values[1453] != 0.0))) && (scratch.values[1455] != 0.0)) {
            scratch.values[1358] = 0.0;
            scratch.node_derivatives[1358] = [0.0; Instance::NODE_COUNT];
            scratch.branch_derivatives[1358] = [0.0; Instance::BRANCH_COUNT];
        }

        if ((((scratch.values[1320] != 0.0) && (scratch.values[1337] != 0.0)) && (!(scratch.values[1453] != 0.0))) && (!(scratch.values[1455] != 0.0))) {
            scratch.store_ad(1359, &AdValue::sub_from_scalar(scratch.values[449], scratch.ad_value(1353)));
        }

        if ((((scratch.values[1320] != 0.0) && (scratch.values[1337] != 0.0)) && (!(scratch.values[1453] != 0.0))) && (!(scratch.values[1455] != 0.0))) {
            scratch.store_ad(1360, &AdValue::sub_from_scalar(1.0, AdValue::sqrt(AdValue::sub_from_scalar(1.0, AdValue::div(scratch.ad_value(1351), scratch.ad_value(1359))))));
        }

        scratch.values[1456] = if (scratch.values[382] == 0.5) { 1.0 } else { 0.0 };

        if (((((scratch.values[1320] != 0.0) && (scratch.values[1337] != 0.0)) && (!(scratch.values[1453] != 0.0))) && (!(scratch.values[1455] != 0.0))) && (scratch.values[1456] != 0.0)) {
            scratch.values[1361] = 0.0;
            scratch.node_derivatives[1361] = [0.0; Instance::NODE_COUNT];
            scratch.branch_derivatives[1361] = [0.0; Instance::BRANCH_COUNT];
        }

        if (((((scratch.values[1320] != 0.0) && (scratch.values[1337] != 0.0)) && (!(scratch.values[1453] != 0.0))) && (!(scratch.values[1455] != 0.0))) && (!(scratch.values[1456] != 0.0))) {
            scratch.store_ad(1361, &AdValue::scale(AdValue::add(AdValue::div(AdValue::mul(AdValue::square(scratch.ad_value(1360)), AdValue::ln(scratch.ad_value(1360))), AdValue::sub_from_scalar(1.0, scratch.ad_value(1360))), scratch.ad_value(1360)), (1.0 - (2.0 * scratch.values[382]))));
        }

        if ((((scratch.values[1320] != 0.0) && (scratch.values[1337] != 0.0)) && (!(scratch.values[1453] != 0.0))) && (!(scratch.values[1455] != 0.0))) {
            scratch.store_ad(1362, &AdValue::add(scratch.ad_value(1360), scratch.ad_value(1361)));
        }

        scratch.values[1457] = if (scratch.values[382] == 0.5) { 1.0 } else { 0.0 };

        if (((((scratch.values[1320] != 0.0) && (scratch.values[1337] != 0.0)) && (!(scratch.values[1453] != 0.0))) && (!(scratch.values[1455] != 0.0))) && (scratch.values[1457] != 0.0)) {
            scratch.store_ad(1356, &AdValue::sqrt(AdValue::scale(scratch.ad_value(1359), scratch.values[485])));
        }

        if (((((scratch.values[1320] != 0.0) && (scratch.values[1337] != 0.0)) && (!(scratch.values[1453] != 0.0))) && (!(scratch.values[1455] != 0.0))) && (!(scratch.values[1457] != 0.0))) {
            scratch.store_ad(1356, &AdValue::powf(AdValue::scale(scratch.ad_value(1359), scratch.values[485]), scratch.values[382]));
        }

        if ((((scratch.values[1320] != 0.0) && (scratch.values[1337] != 0.0)) && (!(scratch.values[1453] != 0.0))) && (!(scratch.values[1455] != 0.0))) {
            scratch.store_ad(1363, &AdValue::scale(scratch.ad_value(1356), scratch.values[479]));
        }

        if ((((scratch.values[1320] != 0.0) && (scratch.values[1337] != 0.0)) && (!(scratch.values[1453] != 0.0))) && (!(scratch.values[1455] != 0.0))) {
            scratch.store_ad(1364, &AdValue::scale(AdValue::mul(AdValue::offset(scratch.ad_value(1350), (-1.0)), scratch.ad_value(1363)), scratch.values[440]));
        }

        if ((((scratch.values[1320] != 0.0) && (scratch.values[1337] != 0.0)) && (!(scratch.values[1453] != 0.0))) && (!(scratch.values[1455] != 0.0))) {
            scratch.store_ad(1358, &AdValue::scale(AdValue::mul(scratch.ad_value(1364), scratch.ad_value(1362)), scratch.values[393]));
        }

        scratch.values[1458] = if (scratch.values[396] == 0.0) { 1.0 } else { 0.0 };

        if ((((scratch.values[1320] != 0.0) && (scratch.values[1337] != 0.0)) && (!(scratch.values[1453] != 0.0))) && (scratch.values[1458] != 0.0)) {
            scratch.values[1365] = 0.0;
            scratch.node_derivatives[1365] = [0.0; Instance::NODE_COUNT];
            scratch.branch_derivatives[1365] = [0.0; Instance::BRANCH_COUNT];
        }

        if ((((scratch.values[1320] != 0.0) && (scratch.values[1337] != 0.0)) && (!(scratch.values[1453] != 0.0))) && (!(scratch.values[1458] != 0.0))) {
            scratch.store_ad(1366, &AdValue::scale(AdValue::div(AdValue::scale(scratch.ad_value(1363), scratch.values[464]), scratch.ad_value(1359)), scratch.values[494]));
        }

        if ((((scratch.values[1320] != 0.0) && (scratch.values[1337] != 0.0)) && (!(scratch.values[1453] != 0.0))) && (!(scratch.values[1458] != 0.0))) {
            scratch.store_ad(1367, &AdValue::div_from_scalar((0.666666666666667 * scratch.values[491]), scratch.ad_value(1366)));
        }

        if ((((scratch.values[1320] != 0.0) && (scratch.values[1337] != 0.0)) && (!(scratch.values[1453] != 0.0))) && (!(scratch.values[1458] != 0.0))) {
            scratch.store_ad(1368, &AdValue::square(scratch.ad_value(1367)));
        }

        if ((((scratch.values[1320] != 0.0) && (scratch.values[1337] != 0.0)) && (!(scratch.values[1453] != 0.0))) && (!(scratch.values[1458] != 0.0))) {
            scratch.store_ad(1369, &AdValue::sqrt(AdValue::div(AdValue::square(scratch.ad_value(1368)), AdValue::offset(AdValue::square(scratch.ad_value(1368)), 1.0))));
        }

        if ((((scratch.values[1320] != 0.0) && (scratch.values[1337] != 0.0)) && (!(scratch.values[1453] != 0.0))) && (!(scratch.values[1458] != 0.0))) {
            scratch.store_ad(1370, &AdValue::sqrt(AdValue::abs(scratch.ad_value(1369))));
        }

        if ((((scratch.values[1320] != 0.0) && (scratch.values[1337] != 0.0)) && (!(scratch.values[1453] != 0.0))) && (!(scratch.values[1458] != 0.0))) {
            scratch.store_ad(1371, &AdValue::mul(scratch.ad_value(1369), scratch.ad_value(1370)));
        }

        scratch.values[1459] = if (((-scratch.values[382]) * scratch.values[467]) == (-1.0)) { 1.0 } else { 0.0 };

        if (((((scratch.values[1320] != 0.0) && (scratch.values[1337] != 0.0)) && (!(scratch.values[1453] != 0.0))) && (!(scratch.values[1458] != 0.0))) && (scratch.values[1459] != 0.0)) {
            scratch.store_ad(1372, &AdValue::div_from_scalar(1.0, AdValue::offset(AdValue::mul(scratch.ad_value(1366), scratch.ad_value(1371)), 1.0)));
        }

        if (((((scratch.values[1320] != 0.0) && (scratch.values[1337] != 0.0)) && (!(scratch.values[1453] != 0.0))) && (!(scratch.values[1458] != 0.0))) && (!(scratch.values[1459] != 0.0))) {
            scratch.store_ad(1372, &AdValue::powf(AdValue::offset(AdValue::mul(scratch.ad_value(1366), scratch.ad_value(1371)), 1.0), ((-scratch.values[382]) * scratch.values[467])));
        }

        if ((((scratch.values[1320] != 0.0) && (scratch.values[1337] != 0.0)) && (!(scratch.values[1453] != 0.0))) && (!(scratch.values[1458] != 0.0))) {
            scratch.store_ad(1373, &AdValue::div(AdValue::mul(scratch.ad_value(1362), scratch.ad_value(1372)), AdValue::add(scratch.ad_value(1362), scratch.ad_value(1372))));
        }

        if ((((scratch.values[1320] != 0.0) && (scratch.values[1337] != 0.0)) && (!(scratch.values[1453] != 0.0))) && (!(scratch.values[1458] != 0.0))) {
            scratch.store_ad(1374, &AdValue::sqrt(AdValue::scale(AdValue::div(scratch.ad_value(1366), scratch.ad_value(1370)), 0.375)));
        }

        if ((((scratch.values[1320] != 0.0) && (scratch.values[1337] != 0.0)) && (!(scratch.values[1453] != 0.0))) && (!(scratch.values[1458] != 0.0))) {
            scratch.store_ad(1375, &AdValue::sub(AdValue::scale(AdValue::mul(scratch.ad_value(1367), scratch.ad_value(1370)), 2.0), scratch.ad_value(1369)));
        }

        if ((((scratch.values[1320] != 0.0) && (scratch.values[1337] != 0.0)) && (!(scratch.values[1453] != 0.0))) && (!(scratch.values[1458] != 0.0))) {
            scratch.store_ad(1376, &AdValue::add(AdValue::sub(AdValue::mul(AdValue::scale(scratch.ad_value(1367), scratch.values[491]), scratch.ad_value(1370)), AdValue::scale(scratch.ad_value(1369), scratch.values[491])), AdValue::scale(AdValue::mul(scratch.ad_value(1366), scratch.ad_value(1371)), 0.5)));
        }

        if ((((scratch.values[1320] != 0.0) && (scratch.values[1337] != 0.0)) && (!(scratch.values[1453] != 0.0))) && (!(scratch.values[1458] != 0.0))) {
            scratch.store_ad(1377, &AdValue::mul(AdValue::offset(scratch.ad_value(1375), (-1.0)), scratch.ad_value(1374)));
        }

        if ((((scratch.values[1320] != 0.0) && (scratch.values[1337] != 0.0)) && (!(scratch.values[1453] != 0.0))) && (!(scratch.values[1458] != 0.0))) {
            scratch.store_ad(1338, &AdValue::square(scratch.ad_value(1377)));
        }

        scratch.values[1460] = if (scratch.values[1377] > 0.0) { 1.0 } else { 0.0 };

        if (((((scratch.values[1320] != 0.0) && (scratch.values[1337] != 0.0)) && (!(scratch.values[1453] != 0.0))) && (!(scratch.values[1458] != 0.0))) && (scratch.values[1460] != 0.0)) {
            scratch.store_ad(1339, &AdValue::div_from_scalar(1.0, AdValue::offset(AdValue::scale(scratch.ad_value(1377), scratch.values[428]), 1.0)));
        }

        if (((((scratch.values[1320] != 0.0) && (scratch.values[1337] != 0.0)) && (!(scratch.values[1453] != 0.0))) && (!(scratch.values[1458] != 0.0))) && (!(scratch.values[1460] != 0.0))) {
            scratch.store_ad(1339, &AdValue::div_from_scalar(1.0, AdValue::sub_from_scalar(1.0, AdValue::scale(scratch.ad_value(1377), scratch.values[428]))));
        }

        scratch.values[1461] = if (((-scratch.values[1338]) + scratch.values[1376]) > (-230.25850929940458)) { 1.0 } else { 0.0 };

        if (((((scratch.values[1320] != 0.0) && (scratch.values[1337] != 0.0)) && (!(scratch.values[1453] != 0.0))) && (!(scratch.values[1458] != 0.0))) && (scratch.values[1461] != 0.0)) {
            scratch.store_ad(1356, &AdValue::exp(AdValue::sub(scratch.ad_value(1376), scratch.ad_value(1338))));
        }

        if (((((scratch.values[1320] != 0.0) && (scratch.values[1337] != 0.0)) && (!(scratch.values[1453] != 0.0))) && (!(scratch.values[1458] != 0.0))) && (!(scratch.values[1461] != 0.0))) {
            let assign16120_ad_e14962: AdValue = AdValue::div_from_scalar(1e-100, AdValue::offset(AdValue::mul(AdValue::sub_from_scalar((-230.25850929940458), AdValue::sub(scratch.ad_value(1376), scratch.ad_value(1338))), AdValue::offset(AdValue::scale(AdValue::mul(AdValue::sub_from_scalar((-230.25850929940458), AdValue::sub(scratch.ad_value(1376), scratch.ad_value(1338))), AdValue::offset(AdValue::scale(AdValue::sub_from_scalar((-230.25850929940458), AdValue::sub(scratch.ad_value(1376), scratch.ad_value(1338))), 0.3333333333333333), 1.0)), 0.5), 1.0)), 1.0));
            scratch.store_ad(1356, &assign16120_ad_e14962);
        }

        if ((((scratch.values[1320] != 0.0) && (scratch.values[1337] != 0.0)) && (!(scratch.values[1453] != 0.0))) && (!(scratch.values[1458] != 0.0))) {
            scratch.store_ad(1340, &AdValue::mul(AdValue::add(AdValue::add(AdValue::scale(scratch.ad_value(1339), 0.29214664), AdValue::scale(AdValue::square(scratch.ad_value(1339)), scratch.values[429])), AdValue::scale(AdValue::mul(AdValue::square(scratch.ad_value(1339)), scratch.ad_value(1339)), scratch.values[430])), scratch.ad_value(1356)));
        }

    }

    pub(super) fn stamp_transient_block_13(
        &mut self,
        ctx: &GeneratedEvalContext<'_>,
        stamper: &mut GeneratedStamper<'_>,
        scratch: &mut Scratch,
    ) {
        let _ = stamper;
        scratch.values[1462] = if (scratch.values[1377] > 0.0) { 1.0 } else { 0.0 };

        if (((((scratch.values[1320] != 0.0) && (scratch.values[1337] != 0.0)) && (!(scratch.values[1453] != 0.0))) && (!(scratch.values[1458] != 0.0))) && (scratch.values[1462] != 0.0)) {
            scratch.values[1378] = scratch.values[1340];
            scratch.node_derivatives[1378] = scratch.node_derivatives[1340];
            scratch.branch_derivatives[1378] = scratch.branch_derivatives[1340];
        }

        scratch.values[1463] = if (scratch.values[1376] > (-230.25850929940458)) { 1.0 } else { 0.0 };

        if ((((((scratch.values[1320] != 0.0) && (scratch.values[1337] != 0.0)) && (!(scratch.values[1453] != 0.0))) && (!(scratch.values[1458] != 0.0))) && (!(scratch.values[1462] != 0.0))) && (scratch.values[1463] != 0.0)) {
            scratch.store_ad(1356, &AdValue::exp(scratch.ad_value(1376)));
        }

        if ((((((scratch.values[1320] != 0.0) && (scratch.values[1337] != 0.0)) && (!(scratch.values[1453] != 0.0))) && (!(scratch.values[1458] != 0.0))) && (!(scratch.values[1462] != 0.0))) && (!(scratch.values[1463] != 0.0))) {
            scratch.store_ad(1356, &AdValue::div_from_scalar(1e-100, AdValue::offset(AdValue::mul(AdValue::sub_from_scalar((-230.25850929940458), scratch.ad_value(1376)), AdValue::offset(AdValue::scale(AdValue::mul(AdValue::sub_from_scalar((-230.25850929940458), scratch.ad_value(1376)), AdValue::offset(AdValue::scale(AdValue::sub_from_scalar((-230.25850929940458), scratch.ad_value(1376)), 0.3333333333333333), 1.0)), 0.5), 1.0)), 1.0)));
        }

        if (((((scratch.values[1320] != 0.0) && (scratch.values[1337] != 0.0)) && (!(scratch.values[1453] != 0.0))) && (!(scratch.values[1458] != 0.0))) && (!(scratch.values[1462] != 0.0))) {
            scratch.store_ad(1378, &AdValue::sub(AdValue::scale(scratch.ad_value(1356), 2.0), scratch.ad_value(1340)));
        }

        if ((((scratch.values[1320] != 0.0) && (scratch.values[1337] != 0.0)) && (!(scratch.values[1453] != 0.0))) && (!(scratch.values[1458] != 0.0))) {
            scratch.store_ad(1379, &AdValue::scale(AdValue::div(AdValue::scale(scratch.ad_value(1378), scratch.values[491]), scratch.ad_value(1374)), (1.772453850905516 * 0.5)));
        }

        if ((((scratch.values[1320] != 0.0) && (scratch.values[1337] != 0.0)) && (!(scratch.values[1453] != 0.0))) && (!(scratch.values[1458] != 0.0))) {
            scratch.store_ad(1365, &AdValue::scale(AdValue::mul(AdValue::mul(scratch.ad_value(1364), scratch.ad_value(1379)), scratch.ad_value(1373)), scratch.values[396]));
        }

        scratch.values[1464] = if (scratch.values[402] == 0.0) { 1.0 } else { 0.0 };

        if ((((scratch.values[1320] != 0.0) && (scratch.values[1337] != 0.0)) && (!(scratch.values[1453] != 0.0))) && (scratch.values[1464] != 0.0)) {
            scratch.values[1380] = 0.0;
            scratch.node_derivatives[1380] = [0.0; Instance::NODE_COUNT];
            scratch.branch_derivatives[1380] = [0.0; Instance::BRANCH_COUNT];
        }

        scratch.values[1465] = if (scratch.values[382] == 0.5) { 1.0 } else { 0.0 };

        if (((((scratch.values[1320] != 0.0) && (scratch.values[1337] != 0.0)) && (!(scratch.values[1453] != 0.0))) && (!(scratch.values[1464] != 0.0))) && (scratch.values[1465] != 0.0)) {
            scratch.store_ad(1356, &AdValue::sqrt(AdValue::scale(AdValue::sub_from_scalar(scratch.values[379], scratch.ad_value(1354)), scratch.values[485])));
        }

        if (((((scratch.values[1320] != 0.0) && (scratch.values[1337] != 0.0)) && (!(scratch.values[1453] != 0.0))) && (!(scratch.values[1464] != 0.0))) && (!(scratch.values[1465] != 0.0))) {
            scratch.store_ad(1356, &AdValue::powf(AdValue::scale(AdValue::sub_from_scalar(scratch.values[379], scratch.ad_value(1354)), scratch.values[485]), scratch.values[382]));
        }

        if ((((scratch.values[1320] != 0.0) && (scratch.values[1337] != 0.0)) && (!(scratch.values[1453] != 0.0))) && (!(scratch.values[1464] != 0.0))) {
            scratch.store_ad(1381, &AdValue::scale(AdValue::div(AdValue::scale(AdValue::sub_from_scalar(scratch.values[379], scratch.ad_value(1354)), scratch.values[482]), scratch.ad_value(1356)), scratch.values[467]));
        }

        scratch.values[1466] = if (((((-scratch.values[497]) / scratch.values[1381])) as f64).abs() < 230.25850929940458) { 1.0 } else { 0.0 };

        if (((((scratch.values[1320] != 0.0) && (scratch.values[1337] != 0.0)) && (!(scratch.values[1453] != 0.0))) && (!(scratch.values[1464] != 0.0))) && (scratch.values[1466] != 0.0)) {
            scratch.store_ad(1356, &AdValue::exp(AdValue::div(AdValue::neg(scratch.ad_value(497)), scratch.ad_value(1381))));
        }

        scratch.values[1467] = if (((-scratch.values[497]) / scratch.values[1381]) < 0.0) { 1.0 } else { 0.0 };

        if ((((((scratch.values[1320] != 0.0) && (scratch.values[1337] != 0.0)) && (!(scratch.values[1453] != 0.0))) && (!(scratch.values[1464] != 0.0))) && (!(scratch.values[1466] != 0.0))) && (scratch.values[1467] != 0.0)) {
            let assign16310_ad_e15289: AdValue = AdValue::offset(AdValue::mul(AdValue::sub_from_scalar((-230.25850929940458), AdValue::div(AdValue::neg(scratch.ad_value(497)), scratch.ad_value(1381))), AdValue::offset(AdValue::scale(AdValue::mul(AdValue::sub_from_scalar((-230.25850929940458), AdValue::div(AdValue::neg(scratch.ad_value(497)), scratch.ad_value(1381))), AdValue::offset(AdValue::scale(AdValue::sub_from_scalar((-230.25850929940458), AdValue::div(AdValue::neg(scratch.ad_value(497)), scratch.ad_value(1381))), 0.3333333333333333), 1.0)), 0.5), 1.0)), 1.0);
            scratch.store_ad(1356, &AdValue::div_from_scalar(1e-100, assign16310_ad_e15289));
        }

        if ((((((scratch.values[1320] != 0.0) && (scratch.values[1337] != 0.0)) && (!(scratch.values[1453] != 0.0))) && (!(scratch.values[1464] != 0.0))) && (!(scratch.values[1466] != 0.0))) && (!(scratch.values[1467] != 0.0))) {
            let assign16320_ad_e15339: AdValue = AdValue::scale(AdValue::offset(AdValue::mul(AdValue::offset(AdValue::div(AdValue::neg(scratch.ad_value(497)), scratch.ad_value(1381)), (-230.25850929940458)), AdValue::offset(AdValue::scale(AdValue::mul(AdValue::offset(AdValue::div(AdValue::neg(scratch.ad_value(497)), scratch.ad_value(1381)), (-230.25850929940458)), AdValue::offset(AdValue::scale(AdValue::offset(AdValue::div(AdValue::neg(scratch.ad_value(497)), scratch.ad_value(1381)), (-230.25850929940458)), 0.3333333333333333), 1.0)), 0.5), 1.0)), 1.0), 1e100);
            scratch.store_ad(1356, &assign16320_ad_e15339);
        }

        if ((((scratch.values[1320] != 0.0) && (scratch.values[1337] != 0.0)) && (!(scratch.values[1453] != 0.0))) && (!(scratch.values[1464] != 0.0))) {
            scratch.store_ad(1380, &AdValue::scale(AdValue::mul(AdValue::mul(AdValue::mul(scratch.ad_value(524), scratch.ad_value(1381)), scratch.ad_value(1381)), scratch.ad_value(1356)), scratch.values[402]));
        }

        scratch.values[1468] = if (scratch.values[411] > 1000.0) { 1.0 } else { 0.0 };

        if ((((scratch.values[1320] != 0.0) && (scratch.values[1337] != 0.0)) && (!(scratch.values[1453] != 0.0))) && (scratch.values[1468] != 0.0)) {
            scratch.values[1382] = 1.0;
            scratch.node_derivatives[1382] = [0.0; Instance::NODE_COUNT];
            scratch.branch_derivatives[1382] = [0.0; Instance::BRANCH_COUNT];
        }

        scratch.values[1469] = if (scratch.values[1355] > ((-scratch.values[500]) * scratch.values[411])) { 1.0 } else { 0.0 };

        scratch.values[1470] = if (scratch.values[414] == 4.0) { 1.0 } else { 0.0 };

        if ((((((scratch.values[1320] != 0.0) && (scratch.values[1337] != 0.0)) && (!(scratch.values[1453] != 0.0))) && (!(scratch.values[1468] != 0.0))) && (scratch.values[1469] != 0.0)) && (scratch.values[1470] != 0.0)) {
            scratch.store_ad(1356, &AdValue::mul(AdValue::mul(AdValue::mul(AdValue::scale(scratch.ad_value(1355), scratch.values[504]), AdValue::scale(scratch.ad_value(1355), scratch.values[504])), AdValue::scale(scratch.ad_value(1355), scratch.values[504])), AdValue::scale(scratch.ad_value(1355), scratch.values[504])));
        }

        if ((((((scratch.values[1320] != 0.0) && (scratch.values[1337] != 0.0)) && (!(scratch.values[1453] != 0.0))) && (!(scratch.values[1468] != 0.0))) && (scratch.values[1469] != 0.0)) && (!(scratch.values[1470] != 0.0))) {
            scratch.store_ad(1356, &AdValue::powf(AdValue::abs(AdValue::scale(scratch.ad_value(1355), scratch.values[504])), scratch.values[414]));
        }

        if (((((scratch.values[1320] != 0.0) && (scratch.values[1337] != 0.0)) && (!(scratch.values[1453] != 0.0))) && (!(scratch.values[1468] != 0.0))) && (scratch.values[1469] != 0.0)) {
            scratch.store_ad(1382, &AdValue::div_from_scalar(1.0, AdValue::sub_from_scalar(1.0, scratch.ad_value(1356))));
        }

        if (((((scratch.values[1320] != 0.0) && (scratch.values[1337] != 0.0)) && (!(scratch.values[1453] != 0.0))) && (!(scratch.values[1468] != 0.0))) && (!(scratch.values[1469] != 0.0))) {
            scratch.store_ad(1382, &AdValue::offset(AdValue::scale(AdValue::offset(scratch.ad_value(1355), (scratch.values[500] * scratch.values[411])), scratch.values[507]), scratch.values[501]));
        }

        if (((scratch.values[1320] != 0.0) && (scratch.values[1337] != 0.0)) && (!(scratch.values[1453] != 0.0))) {
            scratch.store_ad(1383, &AdValue::mul(AdValue::add(AdValue::add(AdValue::add(scratch.ad_value(1357), scratch.ad_value(1358)), scratch.ad_value(1365)), scratch.ad_value(1380)), scratch.ad_value(1382)));
        }

        scratch.values[1471] = if (scratch.values[698] == 0.0) { 1.0 } else { 0.0 };

        if (((scratch.values[1320] != 0.0) && (scratch.values[1337] != 0.0)) && (scratch.values[1471] != 0.0)) {
            scratch.values[1384] = 0.0;
            scratch.node_derivatives[1384] = [0.0; Instance::NODE_COUNT];
            scratch.branch_derivatives[1384] = [0.0; Instance::BRANCH_COUNT];
        }

        scratch.values[1472] = if (scratch.values[465] == 0.5) { 1.0 } else { 0.0 };

        if ((((scratch.values[1320] != 0.0) && (scratch.values[1337] != 0.0)) && (!(scratch.values[1471] != 0.0))) && (scratch.values[1472] != 0.0)) {
            scratch.store_ad(1356, &AdValue::sqrt(AdValue::sub_from_scalar(1.0, AdValue::scale(scratch.ad_value(1348), scratch.values[462]))));
        }

        if ((((scratch.values[1320] != 0.0) && (scratch.values[1337] != 0.0)) && (!(scratch.values[1471] != 0.0))) && (!(scratch.values[1472] != 0.0))) {
            scratch.store_ad(1356, &AdValue::powf(AdValue::sub_from_scalar(1.0, AdValue::scale(scratch.ad_value(1348), scratch.values[462])), scratch.values[465]));
        }

        if (((scratch.values[1320] != 0.0) && (scratch.values[1337] != 0.0)) && (!(scratch.values[1471] != 0.0))) {
            scratch.store_ad(1357, &AdValue::scale(scratch.ad_value(1347), scratch.values[444]));
        }

        scratch.values[1473] = if ((scratch.values[394] == 0.0) && (scratch.values[397] == 0.0)) { 1.0 } else { 0.0 };

        if ((((scratch.values[1320] != 0.0) && (scratch.values[1337] != 0.0)) && (!(scratch.values[1471] != 0.0))) && (scratch.values[1473] != 0.0)) {
            scratch.values[1358] = 0.0;
            scratch.node_derivatives[1358] = [0.0; Instance::NODE_COUNT];
            scratch.branch_derivatives[1358] = [0.0; Instance::BRANCH_COUNT];
        }

        if ((((scratch.values[1320] != 0.0) && (scratch.values[1337] != 0.0)) && (!(scratch.values[1471] != 0.0))) && (!(scratch.values[1473] != 0.0))) {
            scratch.store_ad(1359, &AdValue::sub_from_scalar(scratch.values[450], scratch.ad_value(1353)));
        }

        if ((((scratch.values[1320] != 0.0) && (scratch.values[1337] != 0.0)) && (!(scratch.values[1471] != 0.0))) && (!(scratch.values[1473] != 0.0))) {
            scratch.store_ad(1360, &AdValue::sub_from_scalar(1.0, AdValue::sqrt(AdValue::sub_from_scalar(1.0, AdValue::div(scratch.ad_value(1351), scratch.ad_value(1359))))));
        }

        scratch.values[1474] = if (scratch.values[383] == 0.5) { 1.0 } else { 0.0 };

        if (((((scratch.values[1320] != 0.0) && (scratch.values[1337] != 0.0)) && (!(scratch.values[1471] != 0.0))) && (!(scratch.values[1473] != 0.0))) && (scratch.values[1474] != 0.0)) {
            scratch.values[1361] = 0.0;
            scratch.node_derivatives[1361] = [0.0; Instance::NODE_COUNT];
            scratch.branch_derivatives[1361] = [0.0; Instance::BRANCH_COUNT];
        }

        if (((((scratch.values[1320] != 0.0) && (scratch.values[1337] != 0.0)) && (!(scratch.values[1471] != 0.0))) && (!(scratch.values[1473] != 0.0))) && (!(scratch.values[1474] != 0.0))) {
            scratch.store_ad(1361, &AdValue::scale(AdValue::add(AdValue::div(AdValue::mul(AdValue::square(scratch.ad_value(1360)), AdValue::ln(scratch.ad_value(1360))), AdValue::sub_from_scalar(1.0, scratch.ad_value(1360))), scratch.ad_value(1360)), (1.0 - (2.0 * scratch.values[383]))));
        }

        if ((((scratch.values[1320] != 0.0) && (scratch.values[1337] != 0.0)) && (!(scratch.values[1471] != 0.0))) && (!(scratch.values[1473] != 0.0))) {
            scratch.store_ad(1362, &AdValue::add(scratch.ad_value(1360), scratch.ad_value(1361)));
        }

        scratch.values[1475] = if (scratch.values[383] == 0.5) { 1.0 } else { 0.0 };

        if (((((scratch.values[1320] != 0.0) && (scratch.values[1337] != 0.0)) && (!(scratch.values[1471] != 0.0))) && (!(scratch.values[1473] != 0.0))) && (scratch.values[1475] != 0.0)) {
            scratch.store_ad(1356, &AdValue::sqrt(AdValue::scale(scratch.ad_value(1359), scratch.values[486])));
        }

        if (((((scratch.values[1320] != 0.0) && (scratch.values[1337] != 0.0)) && (!(scratch.values[1471] != 0.0))) && (!(scratch.values[1473] != 0.0))) && (!(scratch.values[1475] != 0.0))) {
            scratch.store_ad(1356, &AdValue::powf(AdValue::scale(scratch.ad_value(1359), scratch.values[486]), scratch.values[383]));
        }

        if ((((scratch.values[1320] != 0.0) && (scratch.values[1337] != 0.0)) && (!(scratch.values[1471] != 0.0))) && (!(scratch.values[1473] != 0.0))) {
            scratch.store_ad(1363, &AdValue::scale(scratch.ad_value(1356), scratch.values[480]));
        }

        if ((((scratch.values[1320] != 0.0) && (scratch.values[1337] != 0.0)) && (!(scratch.values[1471] != 0.0))) && (!(scratch.values[1473] != 0.0))) {
            scratch.store_ad(1364, &AdValue::scale(AdValue::mul(AdValue::offset(scratch.ad_value(1350), (-1.0)), scratch.ad_value(1363)), scratch.values[441]));
        }

        if ((((scratch.values[1320] != 0.0) && (scratch.values[1337] != 0.0)) && (!(scratch.values[1471] != 0.0))) && (!(scratch.values[1473] != 0.0))) {
            scratch.store_ad(1358, &AdValue::scale(AdValue::mul(scratch.ad_value(1364), scratch.ad_value(1362)), scratch.values[394]));
        }

        scratch.values[1476] = if (scratch.values[397] == 0.0) { 1.0 } else { 0.0 };

        if ((((scratch.values[1320] != 0.0) && (scratch.values[1337] != 0.0)) && (!(scratch.values[1471] != 0.0))) && (scratch.values[1476] != 0.0)) {
            scratch.values[1365] = 0.0;
            scratch.node_derivatives[1365] = [0.0; Instance::NODE_COUNT];
            scratch.branch_derivatives[1365] = [0.0; Instance::BRANCH_COUNT];
        }

        if ((((scratch.values[1320] != 0.0) && (scratch.values[1337] != 0.0)) && (!(scratch.values[1471] != 0.0))) && (!(scratch.values[1476] != 0.0))) {
            scratch.store_ad(1366, &AdValue::scale(AdValue::div(AdValue::scale(scratch.ad_value(1363), scratch.values[465]), scratch.ad_value(1359)), scratch.values[495]));
        }

        if ((((scratch.values[1320] != 0.0) && (scratch.values[1337] != 0.0)) && (!(scratch.values[1471] != 0.0))) && (!(scratch.values[1476] != 0.0))) {
            scratch.store_ad(1367, &AdValue::div_from_scalar((0.666666666666667 * scratch.values[492]), scratch.ad_value(1366)));
        }

        if ((((scratch.values[1320] != 0.0) && (scratch.values[1337] != 0.0)) && (!(scratch.values[1471] != 0.0))) && (!(scratch.values[1476] != 0.0))) {
            scratch.store_ad(1368, &AdValue::square(scratch.ad_value(1367)));
        }

        if ((((scratch.values[1320] != 0.0) && (scratch.values[1337] != 0.0)) && (!(scratch.values[1471] != 0.0))) && (!(scratch.values[1476] != 0.0))) {
            scratch.store_ad(1369, &AdValue::sqrt(AdValue::div(AdValue::square(scratch.ad_value(1368)), AdValue::offset(AdValue::square(scratch.ad_value(1368)), 1.0))));
        }

        if ((((scratch.values[1320] != 0.0) && (scratch.values[1337] != 0.0)) && (!(scratch.values[1471] != 0.0))) && (!(scratch.values[1476] != 0.0))) {
            scratch.store_ad(1370, &AdValue::sqrt(AdValue::abs(scratch.ad_value(1369))));
        }

        if ((((scratch.values[1320] != 0.0) && (scratch.values[1337] != 0.0)) && (!(scratch.values[1471] != 0.0))) && (!(scratch.values[1476] != 0.0))) {
            scratch.store_ad(1371, &AdValue::mul(scratch.ad_value(1369), scratch.ad_value(1370)));
        }

        scratch.values[1477] = if (((-scratch.values[383]) * scratch.values[468]) == (-1.0)) { 1.0 } else { 0.0 };

        if (((((scratch.values[1320] != 0.0) && (scratch.values[1337] != 0.0)) && (!(scratch.values[1471] != 0.0))) && (!(scratch.values[1476] != 0.0))) && (scratch.values[1477] != 0.0)) {
            scratch.store_ad(1372, &AdValue::div_from_scalar(1.0, AdValue::offset(AdValue::mul(scratch.ad_value(1366), scratch.ad_value(1371)), 1.0)));
        }

        if (((((scratch.values[1320] != 0.0) && (scratch.values[1337] != 0.0)) && (!(scratch.values[1471] != 0.0))) && (!(scratch.values[1476] != 0.0))) && (!(scratch.values[1477] != 0.0))) {
            scratch.store_ad(1372, &AdValue::powf(AdValue::offset(AdValue::mul(scratch.ad_value(1366), scratch.ad_value(1371)), 1.0), ((-scratch.values[383]) * scratch.values[468])));
        }

        if ((((scratch.values[1320] != 0.0) && (scratch.values[1337] != 0.0)) && (!(scratch.values[1471] != 0.0))) && (!(scratch.values[1476] != 0.0))) {
            scratch.store_ad(1373, &AdValue::div(AdValue::mul(scratch.ad_value(1362), scratch.ad_value(1372)), AdValue::add(scratch.ad_value(1362), scratch.ad_value(1372))));
        }

        if ((((scratch.values[1320] != 0.0) && (scratch.values[1337] != 0.0)) && (!(scratch.values[1471] != 0.0))) && (!(scratch.values[1476] != 0.0))) {
            scratch.store_ad(1374, &AdValue::sqrt(AdValue::scale(AdValue::div(scratch.ad_value(1366), scratch.ad_value(1370)), 0.375)));
        }

        if ((((scratch.values[1320] != 0.0) && (scratch.values[1337] != 0.0)) && (!(scratch.values[1471] != 0.0))) && (!(scratch.values[1476] != 0.0))) {
            scratch.store_ad(1375, &AdValue::sub(AdValue::scale(AdValue::mul(scratch.ad_value(1367), scratch.ad_value(1370)), 2.0), scratch.ad_value(1369)));
        }

        if ((((scratch.values[1320] != 0.0) && (scratch.values[1337] != 0.0)) && (!(scratch.values[1471] != 0.0))) && (!(scratch.values[1476] != 0.0))) {
            scratch.store_ad(1376, &AdValue::add(AdValue::sub(AdValue::mul(AdValue::scale(scratch.ad_value(1367), scratch.values[492]), scratch.ad_value(1370)), AdValue::scale(scratch.ad_value(1369), scratch.values[492])), AdValue::scale(AdValue::mul(scratch.ad_value(1366), scratch.ad_value(1371)), 0.5)));
        }

        if ((((scratch.values[1320] != 0.0) && (scratch.values[1337] != 0.0)) && (!(scratch.values[1471] != 0.0))) && (!(scratch.values[1476] != 0.0))) {
            scratch.store_ad(1377, &AdValue::mul(AdValue::offset(scratch.ad_value(1375), (-1.0)), scratch.ad_value(1374)));
        }

        if ((((scratch.values[1320] != 0.0) && (scratch.values[1337] != 0.0)) && (!(scratch.values[1471] != 0.0))) && (!(scratch.values[1476] != 0.0))) {
            scratch.store_ad(1338, &AdValue::square(scratch.ad_value(1377)));
        }

        scratch.values[1478] = if (scratch.values[1377] > 0.0) { 1.0 } else { 0.0 };

        if (((((scratch.values[1320] != 0.0) && (scratch.values[1337] != 0.0)) && (!(scratch.values[1471] != 0.0))) && (!(scratch.values[1476] != 0.0))) && (scratch.values[1478] != 0.0)) {
            scratch.store_ad(1339, &AdValue::div_from_scalar(1.0, AdValue::offset(AdValue::scale(scratch.ad_value(1377), scratch.values[428]), 1.0)));
        }

        if (((((scratch.values[1320] != 0.0) && (scratch.values[1337] != 0.0)) && (!(scratch.values[1471] != 0.0))) && (!(scratch.values[1476] != 0.0))) && (!(scratch.values[1478] != 0.0))) {
            scratch.store_ad(1339, &AdValue::div_from_scalar(1.0, AdValue::sub_from_scalar(1.0, AdValue::scale(scratch.ad_value(1377), scratch.values[428]))));
        }

        scratch.values[1479] = if (((-scratch.values[1338]) + scratch.values[1376]) > (-230.25850929940458)) { 1.0 } else { 0.0 };

        if (((((scratch.values[1320] != 0.0) && (scratch.values[1337] != 0.0)) && (!(scratch.values[1471] != 0.0))) && (!(scratch.values[1476] != 0.0))) && (scratch.values[1479] != 0.0)) {
            scratch.store_ad(1356, &AdValue::exp(AdValue::sub(scratch.ad_value(1376), scratch.ad_value(1338))));
        }

        if (((((scratch.values[1320] != 0.0) && (scratch.values[1337] != 0.0)) && (!(scratch.values[1471] != 0.0))) && (!(scratch.values[1476] != 0.0))) && (!(scratch.values[1479] != 0.0))) {
            let assign16870_ad_e16168: AdValue = AdValue::div_from_scalar(1e-100, AdValue::offset(AdValue::mul(AdValue::sub_from_scalar((-230.25850929940458), AdValue::sub(scratch.ad_value(1376), scratch.ad_value(1338))), AdValue::offset(AdValue::scale(AdValue::mul(AdValue::sub_from_scalar((-230.25850929940458), AdValue::sub(scratch.ad_value(1376), scratch.ad_value(1338))), AdValue::offset(AdValue::scale(AdValue::sub_from_scalar((-230.25850929940458), AdValue::sub(scratch.ad_value(1376), scratch.ad_value(1338))), 0.3333333333333333), 1.0)), 0.5), 1.0)), 1.0));
            scratch.store_ad(1356, &assign16870_ad_e16168);
        }

        if ((((scratch.values[1320] != 0.0) && (scratch.values[1337] != 0.0)) && (!(scratch.values[1471] != 0.0))) && (!(scratch.values[1476] != 0.0))) {
            scratch.store_ad(1340, &AdValue::mul(AdValue::add(AdValue::add(AdValue::scale(scratch.ad_value(1339), 0.29214664), AdValue::scale(AdValue::square(scratch.ad_value(1339)), scratch.values[429])), AdValue::scale(AdValue::mul(AdValue::square(scratch.ad_value(1339)), scratch.ad_value(1339)), scratch.values[430])), scratch.ad_value(1356)));
        }

        scratch.values[1480] = if (scratch.values[1377] > 0.0) { 1.0 } else { 0.0 };

        if (((((scratch.values[1320] != 0.0) && (scratch.values[1337] != 0.0)) && (!(scratch.values[1471] != 0.0))) && (!(scratch.values[1476] != 0.0))) && (scratch.values[1480] != 0.0)) {
            scratch.values[1378] = scratch.values[1340];
            scratch.node_derivatives[1378] = scratch.node_derivatives[1340];
            scratch.branch_derivatives[1378] = scratch.branch_derivatives[1340];
        }

        scratch.values[1481] = if (scratch.values[1376] > (-230.25850929940458)) { 1.0 } else { 0.0 };

        if ((((((scratch.values[1320] != 0.0) && (scratch.values[1337] != 0.0)) && (!(scratch.values[1471] != 0.0))) && (!(scratch.values[1476] != 0.0))) && (!(scratch.values[1480] != 0.0))) && (scratch.values[1481] != 0.0)) {
            scratch.store_ad(1356, &AdValue::exp(scratch.ad_value(1376)));
        }

        if ((((((scratch.values[1320] != 0.0) && (scratch.values[1337] != 0.0)) && (!(scratch.values[1471] != 0.0))) && (!(scratch.values[1476] != 0.0))) && (!(scratch.values[1480] != 0.0))) && (!(scratch.values[1481] != 0.0))) {
            scratch.store_ad(1356, &AdValue::div_from_scalar(1e-100, AdValue::offset(AdValue::mul(AdValue::sub_from_scalar((-230.25850929940458), scratch.ad_value(1376)), AdValue::offset(AdValue::scale(AdValue::mul(AdValue::sub_from_scalar((-230.25850929940458), scratch.ad_value(1376)), AdValue::offset(AdValue::scale(AdValue::sub_from_scalar((-230.25850929940458), scratch.ad_value(1376)), 0.3333333333333333), 1.0)), 0.5), 1.0)), 1.0)));
        }

        if (((((scratch.values[1320] != 0.0) && (scratch.values[1337] != 0.0)) && (!(scratch.values[1471] != 0.0))) && (!(scratch.values[1476] != 0.0))) && (!(scratch.values[1480] != 0.0))) {
            scratch.store_ad(1378, &AdValue::sub(AdValue::scale(scratch.ad_value(1356), 2.0), scratch.ad_value(1340)));
        }

        if ((((scratch.values[1320] != 0.0) && (scratch.values[1337] != 0.0)) && (!(scratch.values[1471] != 0.0))) && (!(scratch.values[1476] != 0.0))) {
            scratch.store_ad(1379, &AdValue::scale(AdValue::div(AdValue::scale(scratch.ad_value(1378), scratch.values[492]), scratch.ad_value(1374)), (1.772453850905516 * 0.5)));
        }

        if ((((scratch.values[1320] != 0.0) && (scratch.values[1337] != 0.0)) && (!(scratch.values[1471] != 0.0))) && (!(scratch.values[1476] != 0.0))) {
            scratch.store_ad(1365, &AdValue::scale(AdValue::mul(AdValue::mul(scratch.ad_value(1364), scratch.ad_value(1379)), scratch.ad_value(1373)), scratch.values[397]));
        }

        scratch.values[1482] = if (scratch.values[403] == 0.0) { 1.0 } else { 0.0 };

        if ((((scratch.values[1320] != 0.0) && (scratch.values[1337] != 0.0)) && (!(scratch.values[1471] != 0.0))) && (scratch.values[1482] != 0.0)) {
            scratch.values[1380] = 0.0;
            scratch.node_derivatives[1380] = [0.0; Instance::NODE_COUNT];
            scratch.branch_derivatives[1380] = [0.0; Instance::BRANCH_COUNT];
        }

        scratch.values[1483] = if (scratch.values[383] == 0.5) { 1.0 } else { 0.0 };

        if (((((scratch.values[1320] != 0.0) && (scratch.values[1337] != 0.0)) && (!(scratch.values[1471] != 0.0))) && (!(scratch.values[1482] != 0.0))) && (scratch.values[1483] != 0.0)) {
            scratch.store_ad(1356, &AdValue::sqrt(AdValue::scale(AdValue::sub_from_scalar(scratch.values[380], scratch.ad_value(1354)), scratch.values[486])));
        }

        if (((((scratch.values[1320] != 0.0) && (scratch.values[1337] != 0.0)) && (!(scratch.values[1471] != 0.0))) && (!(scratch.values[1482] != 0.0))) && (!(scratch.values[1483] != 0.0))) {
            scratch.store_ad(1356, &AdValue::powf(AdValue::scale(AdValue::sub_from_scalar(scratch.values[380], scratch.ad_value(1354)), scratch.values[486]), scratch.values[383]));
        }

        if ((((scratch.values[1320] != 0.0) && (scratch.values[1337] != 0.0)) && (!(scratch.values[1471] != 0.0))) && (!(scratch.values[1482] != 0.0))) {
            scratch.store_ad(1381, &AdValue::scale(AdValue::div(AdValue::scale(AdValue::sub_from_scalar(scratch.values[380], scratch.ad_value(1354)), scratch.values[483]), scratch.ad_value(1356)), scratch.values[468]));
        }

        scratch.values[1484] = if (((((-scratch.values[498]) / scratch.values[1381])) as f64).abs() < 230.25850929940458) { 1.0 } else { 0.0 };

        if (((((scratch.values[1320] != 0.0) && (scratch.values[1337] != 0.0)) && (!(scratch.values[1471] != 0.0))) && (!(scratch.values[1482] != 0.0))) && (scratch.values[1484] != 0.0)) {
            scratch.store_ad(1356, &AdValue::exp(AdValue::div(AdValue::neg(scratch.ad_value(498)), scratch.ad_value(1381))));
        }

        scratch.values[1485] = if (((-scratch.values[498]) / scratch.values[1381]) < 0.0) { 1.0 } else { 0.0 };

        if ((((((scratch.values[1320] != 0.0) && (scratch.values[1337] != 0.0)) && (!(scratch.values[1471] != 0.0))) && (!(scratch.values[1482] != 0.0))) && (!(scratch.values[1484] != 0.0))) && (scratch.values[1485] != 0.0)) {
            let assign17060_ad_e16495: AdValue = AdValue::offset(AdValue::mul(AdValue::sub_from_scalar((-230.25850929940458), AdValue::div(AdValue::neg(scratch.ad_value(498)), scratch.ad_value(1381))), AdValue::offset(AdValue::scale(AdValue::mul(AdValue::sub_from_scalar((-230.25850929940458), AdValue::div(AdValue::neg(scratch.ad_value(498)), scratch.ad_value(1381))), AdValue::offset(AdValue::scale(AdValue::sub_from_scalar((-230.25850929940458), AdValue::div(AdValue::neg(scratch.ad_value(498)), scratch.ad_value(1381))), 0.3333333333333333), 1.0)), 0.5), 1.0)), 1.0);
            scratch.store_ad(1356, &AdValue::div_from_scalar(1e-100, assign17060_ad_e16495));
        }

        if ((((((scratch.values[1320] != 0.0) && (scratch.values[1337] != 0.0)) && (!(scratch.values[1471] != 0.0))) && (!(scratch.values[1482] != 0.0))) && (!(scratch.values[1484] != 0.0))) && (!(scratch.values[1485] != 0.0))) {
            let assign17070_ad_e16545: AdValue = AdValue::scale(AdValue::offset(AdValue::mul(AdValue::offset(AdValue::div(AdValue::neg(scratch.ad_value(498)), scratch.ad_value(1381)), (-230.25850929940458)), AdValue::offset(AdValue::scale(AdValue::mul(AdValue::offset(AdValue::div(AdValue::neg(scratch.ad_value(498)), scratch.ad_value(1381)), (-230.25850929940458)), AdValue::offset(AdValue::scale(AdValue::offset(AdValue::div(AdValue::neg(scratch.ad_value(498)), scratch.ad_value(1381)), (-230.25850929940458)), 0.3333333333333333), 1.0)), 0.5), 1.0)), 1.0), 1e100);
            scratch.store_ad(1356, &assign17070_ad_e16545);
        }

        if ((((scratch.values[1320] != 0.0) && (scratch.values[1337] != 0.0)) && (!(scratch.values[1471] != 0.0))) && (!(scratch.values[1482] != 0.0))) {
            scratch.store_ad(1380, &AdValue::scale(AdValue::mul(AdValue::mul(AdValue::mul(scratch.ad_value(524), scratch.ad_value(1381)), scratch.ad_value(1381)), scratch.ad_value(1356)), scratch.values[403]));
        }

        scratch.values[1486] = if (scratch.values[412] > 1000.0) { 1.0 } else { 0.0 };

        if ((((scratch.values[1320] != 0.0) && (scratch.values[1337] != 0.0)) && (!(scratch.values[1471] != 0.0))) && (scratch.values[1486] != 0.0)) {
            scratch.values[1382] = 1.0;
            scratch.node_derivatives[1382] = [0.0; Instance::NODE_COUNT];
            scratch.branch_derivatives[1382] = [0.0; Instance::BRANCH_COUNT];
        }

        scratch.values[1487] = if (scratch.values[1355] > ((-scratch.values[500]) * scratch.values[412])) { 1.0 } else { 0.0 };

        scratch.values[1488] = if (scratch.values[415] == 4.0) { 1.0 } else { 0.0 };

        if ((((((scratch.values[1320] != 0.0) && (scratch.values[1337] != 0.0)) && (!(scratch.values[1471] != 0.0))) && (!(scratch.values[1486] != 0.0))) && (scratch.values[1487] != 0.0)) && (scratch.values[1488] != 0.0)) {
            scratch.store_ad(1356, &AdValue::mul(AdValue::mul(AdValue::mul(AdValue::scale(scratch.ad_value(1355), scratch.values[505]), AdValue::scale(scratch.ad_value(1355), scratch.values[505])), AdValue::scale(scratch.ad_value(1355), scratch.values[505])), AdValue::scale(scratch.ad_value(1355), scratch.values[505])));
        }

        if ((((((scratch.values[1320] != 0.0) && (scratch.values[1337] != 0.0)) && (!(scratch.values[1471] != 0.0))) && (!(scratch.values[1486] != 0.0))) && (scratch.values[1487] != 0.0)) && (!(scratch.values[1488] != 0.0))) {
            scratch.store_ad(1356, &AdValue::powf(AdValue::abs(AdValue::scale(scratch.ad_value(1355), scratch.values[505])), scratch.values[415]));
        }

        if (((((scratch.values[1320] != 0.0) && (scratch.values[1337] != 0.0)) && (!(scratch.values[1471] != 0.0))) && (!(scratch.values[1486] != 0.0))) && (scratch.values[1487] != 0.0)) {
            scratch.store_ad(1382, &AdValue::div_from_scalar(1.0, AdValue::sub_from_scalar(1.0, scratch.ad_value(1356))));
        }

        if (((((scratch.values[1320] != 0.0) && (scratch.values[1337] != 0.0)) && (!(scratch.values[1471] != 0.0))) && (!(scratch.values[1486] != 0.0))) && (!(scratch.values[1487] != 0.0))) {
            scratch.store_ad(1382, &AdValue::offset(AdValue::scale(AdValue::offset(scratch.ad_value(1355), (scratch.values[500] * scratch.values[412])), scratch.values[508]), scratch.values[502]));
        }

        if (((scratch.values[1320] != 0.0) && (scratch.values[1337] != 0.0)) && (!(scratch.values[1471] != 0.0))) {
            scratch.store_ad(1384, &AdValue::mul(AdValue::add(AdValue::add(AdValue::add(scratch.ad_value(1357), scratch.ad_value(1358)), scratch.ad_value(1365)), scratch.ad_value(1380)), scratch.ad_value(1382)));
        }

        scratch.values[1489] = if (scratch.values[699] == 0.0) { 1.0 } else { 0.0 };

        if (((scratch.values[1320] != 0.0) && (scratch.values[1337] != 0.0)) && (scratch.values[1489] != 0.0)) {
            scratch.values[1385] = 0.0;
            scratch.node_derivatives[1385] = [0.0; Instance::NODE_COUNT];
            scratch.branch_derivatives[1385] = [0.0; Instance::BRANCH_COUNT];
        }

        scratch.values[1490] = if (scratch.values[466] == 0.5) { 1.0 } else { 0.0 };

        if ((((scratch.values[1320] != 0.0) && (scratch.values[1337] != 0.0)) && (!(scratch.values[1489] != 0.0))) && (scratch.values[1490] != 0.0)) {
            scratch.store_ad(1356, &AdValue::sqrt(AdValue::sub_from_scalar(1.0, AdValue::scale(scratch.ad_value(1348), scratch.values[463]))));
        }

        if ((((scratch.values[1320] != 0.0) && (scratch.values[1337] != 0.0)) && (!(scratch.values[1489] != 0.0))) && (!(scratch.values[1490] != 0.0))) {
            scratch.store_ad(1356, &AdValue::powf(AdValue::sub_from_scalar(1.0, AdValue::scale(scratch.ad_value(1348), scratch.values[463])), scratch.values[466]));
        }

        if (((scratch.values[1320] != 0.0) && (scratch.values[1337] != 0.0)) && (!(scratch.values[1489] != 0.0))) {
            scratch.store_ad(1357, &AdValue::scale(scratch.ad_value(1347), scratch.values[445]));
        }

        scratch.values[1491] = if ((scratch.values[395] == 0.0) && (scratch.values[398] == 0.0)) { 1.0 } else { 0.0 };

        if ((((scratch.values[1320] != 0.0) && (scratch.values[1337] != 0.0)) && (!(scratch.values[1489] != 0.0))) && (scratch.values[1491] != 0.0)) {
            scratch.values[1358] = 0.0;
            scratch.node_derivatives[1358] = [0.0; Instance::NODE_COUNT];
            scratch.branch_derivatives[1358] = [0.0; Instance::BRANCH_COUNT];
        }

        if ((((scratch.values[1320] != 0.0) && (scratch.values[1337] != 0.0)) && (!(scratch.values[1489] != 0.0))) && (!(scratch.values[1491] != 0.0))) {
            scratch.store_ad(1359, &AdValue::sub_from_scalar(scratch.values[451], scratch.ad_value(1353)));
        }

        if ((((scratch.values[1320] != 0.0) && (scratch.values[1337] != 0.0)) && (!(scratch.values[1489] != 0.0))) && (!(scratch.values[1491] != 0.0))) {
            scratch.store_ad(1360, &AdValue::sub_from_scalar(1.0, AdValue::sqrt(AdValue::sub_from_scalar(1.0, AdValue::div(scratch.ad_value(1351), scratch.ad_value(1359))))));
        }

        scratch.values[1492] = if (scratch.values[384] == 0.5) { 1.0 } else { 0.0 };

        if (((((scratch.values[1320] != 0.0) && (scratch.values[1337] != 0.0)) && (!(scratch.values[1489] != 0.0))) && (!(scratch.values[1491] != 0.0))) && (scratch.values[1492] != 0.0)) {
            scratch.values[1361] = 0.0;
            scratch.node_derivatives[1361] = [0.0; Instance::NODE_COUNT];
            scratch.branch_derivatives[1361] = [0.0; Instance::BRANCH_COUNT];
        }

        if (((((scratch.values[1320] != 0.0) && (scratch.values[1337] != 0.0)) && (!(scratch.values[1489] != 0.0))) && (!(scratch.values[1491] != 0.0))) && (!(scratch.values[1492] != 0.0))) {
            scratch.store_ad(1361, &AdValue::scale(AdValue::add(AdValue::div(AdValue::mul(AdValue::square(scratch.ad_value(1360)), AdValue::ln(scratch.ad_value(1360))), AdValue::sub_from_scalar(1.0, scratch.ad_value(1360))), scratch.ad_value(1360)), (1.0 - (2.0 * scratch.values[384]))));
        }

        if ((((scratch.values[1320] != 0.0) && (scratch.values[1337] != 0.0)) && (!(scratch.values[1489] != 0.0))) && (!(scratch.values[1491] != 0.0))) {
            scratch.store_ad(1362, &AdValue::add(scratch.ad_value(1360), scratch.ad_value(1361)));
        }

        scratch.values[1493] = if (scratch.values[384] == 0.5) { 1.0 } else { 0.0 };

        if (((((scratch.values[1320] != 0.0) && (scratch.values[1337] != 0.0)) && (!(scratch.values[1489] != 0.0))) && (!(scratch.values[1491] != 0.0))) && (scratch.values[1493] != 0.0)) {
            scratch.store_ad(1356, &AdValue::sqrt(AdValue::scale(scratch.ad_value(1359), scratch.values[487])));
        }

        if (((((scratch.values[1320] != 0.0) && (scratch.values[1337] != 0.0)) && (!(scratch.values[1489] != 0.0))) && (!(scratch.values[1491] != 0.0))) && (!(scratch.values[1493] != 0.0))) {
            scratch.store_ad(1356, &AdValue::powf(AdValue::scale(scratch.ad_value(1359), scratch.values[487]), scratch.values[384]));
        }

        if ((((scratch.values[1320] != 0.0) && (scratch.values[1337] != 0.0)) && (!(scratch.values[1489] != 0.0))) && (!(scratch.values[1491] != 0.0))) {
            scratch.store_ad(1363, &AdValue::scale(scratch.ad_value(1356), scratch.values[481]));
        }

        if ((((scratch.values[1320] != 0.0) && (scratch.values[1337] != 0.0)) && (!(scratch.values[1489] != 0.0))) && (!(scratch.values[1491] != 0.0))) {
            scratch.store_ad(1364, &AdValue::scale(AdValue::mul(AdValue::offset(scratch.ad_value(1350), (-1.0)), scratch.ad_value(1363)), scratch.values[442]));
        }

        if ((((scratch.values[1320] != 0.0) && (scratch.values[1337] != 0.0)) && (!(scratch.values[1489] != 0.0))) && (!(scratch.values[1491] != 0.0))) {
            scratch.store_ad(1358, &AdValue::scale(AdValue::mul(scratch.ad_value(1364), scratch.ad_value(1362)), scratch.values[395]));
        }

        scratch.values[1494] = if (scratch.values[398] == 0.0) { 1.0 } else { 0.0 };

        if ((((scratch.values[1320] != 0.0) && (scratch.values[1337] != 0.0)) && (!(scratch.values[1489] != 0.0))) && (scratch.values[1494] != 0.0)) {
            scratch.values[1365] = 0.0;
            scratch.node_derivatives[1365] = [0.0; Instance::NODE_COUNT];
            scratch.branch_derivatives[1365] = [0.0; Instance::BRANCH_COUNT];
        }

        if ((((scratch.values[1320] != 0.0) && (scratch.values[1337] != 0.0)) && (!(scratch.values[1489] != 0.0))) && (!(scratch.values[1494] != 0.0))) {
            scratch.store_ad(1366, &AdValue::scale(AdValue::div(AdValue::scale(scratch.ad_value(1363), scratch.values[466]), scratch.ad_value(1359)), scratch.values[496]));
        }

        if ((((scratch.values[1320] != 0.0) && (scratch.values[1337] != 0.0)) && (!(scratch.values[1489] != 0.0))) && (!(scratch.values[1494] != 0.0))) {
            scratch.store_ad(1367, &AdValue::div_from_scalar((0.666666666666667 * scratch.values[493]), scratch.ad_value(1366)));
        }

        if ((((scratch.values[1320] != 0.0) && (scratch.values[1337] != 0.0)) && (!(scratch.values[1489] != 0.0))) && (!(scratch.values[1494] != 0.0))) {
            scratch.store_ad(1368, &AdValue::square(scratch.ad_value(1367)));
        }

        if ((((scratch.values[1320] != 0.0) && (scratch.values[1337] != 0.0)) && (!(scratch.values[1489] != 0.0))) && (!(scratch.values[1494] != 0.0))) {
            scratch.store_ad(1369, &AdValue::sqrt(AdValue::div(AdValue::square(scratch.ad_value(1368)), AdValue::offset(AdValue::square(scratch.ad_value(1368)), 1.0))));
        }

        if ((((scratch.values[1320] != 0.0) && (scratch.values[1337] != 0.0)) && (!(scratch.values[1489] != 0.0))) && (!(scratch.values[1494] != 0.0))) {
            scratch.store_ad(1370, &AdValue::sqrt(AdValue::abs(scratch.ad_value(1369))));
        }

        if ((((scratch.values[1320] != 0.0) && (scratch.values[1337] != 0.0)) && (!(scratch.values[1489] != 0.0))) && (!(scratch.values[1494] != 0.0))) {
            scratch.store_ad(1371, &AdValue::mul(scratch.ad_value(1369), scratch.ad_value(1370)));
        }

        scratch.values[1495] = if (((-scratch.values[384]) * scratch.values[469]) == (-1.0)) { 1.0 } else { 0.0 };

        if (((((scratch.values[1320] != 0.0) && (scratch.values[1337] != 0.0)) && (!(scratch.values[1489] != 0.0))) && (!(scratch.values[1494] != 0.0))) && (scratch.values[1495] != 0.0)) {
            scratch.store_ad(1372, &AdValue::div_from_scalar(1.0, AdValue::offset(AdValue::mul(scratch.ad_value(1366), scratch.ad_value(1371)), 1.0)));
        }

        if (((((scratch.values[1320] != 0.0) && (scratch.values[1337] != 0.0)) && (!(scratch.values[1489] != 0.0))) && (!(scratch.values[1494] != 0.0))) && (!(scratch.values[1495] != 0.0))) {
            scratch.store_ad(1372, &AdValue::powf(AdValue::offset(AdValue::mul(scratch.ad_value(1366), scratch.ad_value(1371)), 1.0), ((-scratch.values[384]) * scratch.values[469])));
        }

        if ((((scratch.values[1320] != 0.0) && (scratch.values[1337] != 0.0)) && (!(scratch.values[1489] != 0.0))) && (!(scratch.values[1494] != 0.0))) {
            scratch.store_ad(1373, &AdValue::div(AdValue::mul(scratch.ad_value(1362), scratch.ad_value(1372)), AdValue::add(scratch.ad_value(1362), scratch.ad_value(1372))));
        }

        if ((((scratch.values[1320] != 0.0) && (scratch.values[1337] != 0.0)) && (!(scratch.values[1489] != 0.0))) && (!(scratch.values[1494] != 0.0))) {
            scratch.store_ad(1374, &AdValue::sqrt(AdValue::scale(AdValue::div(scratch.ad_value(1366), scratch.ad_value(1370)), 0.375)));
        }

        if ((((scratch.values[1320] != 0.0) && (scratch.values[1337] != 0.0)) && (!(scratch.values[1489] != 0.0))) && (!(scratch.values[1494] != 0.0))) {
            scratch.store_ad(1375, &AdValue::sub(AdValue::scale(AdValue::mul(scratch.ad_value(1367), scratch.ad_value(1370)), 2.0), scratch.ad_value(1369)));
        }

    }

    pub(super) fn stamp_transient_block_14(
        &mut self,
        ctx: &GeneratedEvalContext<'_>,
        stamper: &mut GeneratedStamper<'_>,
        scratch: &mut Scratch,
    ) {
        let _ = stamper;
        if ((((scratch.values[1320] != 0.0) && (scratch.values[1337] != 0.0)) && (!(scratch.values[1489] != 0.0))) && (!(scratch.values[1494] != 0.0))) {
            scratch.store_ad(1376, &AdValue::add(AdValue::sub(AdValue::mul(AdValue::scale(scratch.ad_value(1367), scratch.values[493]), scratch.ad_value(1370)), AdValue::scale(scratch.ad_value(1369), scratch.values[493])), AdValue::scale(AdValue::mul(scratch.ad_value(1366), scratch.ad_value(1371)), 0.5)));
        }

        if ((((scratch.values[1320] != 0.0) && (scratch.values[1337] != 0.0)) && (!(scratch.values[1489] != 0.0))) && (!(scratch.values[1494] != 0.0))) {
            scratch.store_ad(1377, &AdValue::mul(AdValue::offset(scratch.ad_value(1375), (-1.0)), scratch.ad_value(1374)));
        }

        if ((((scratch.values[1320] != 0.0) && (scratch.values[1337] != 0.0)) && (!(scratch.values[1489] != 0.0))) && (!(scratch.values[1494] != 0.0))) {
            scratch.store_ad(1338, &AdValue::square(scratch.ad_value(1377)));
        }

        scratch.values[1496] = if (scratch.values[1377] > 0.0) { 1.0 } else { 0.0 };

        if (((((scratch.values[1320] != 0.0) && (scratch.values[1337] != 0.0)) && (!(scratch.values[1489] != 0.0))) && (!(scratch.values[1494] != 0.0))) && (scratch.values[1496] != 0.0)) {
            scratch.store_ad(1339, &AdValue::div_from_scalar(1.0, AdValue::offset(AdValue::scale(scratch.ad_value(1377), scratch.values[428]), 1.0)));
        }

        if (((((scratch.values[1320] != 0.0) && (scratch.values[1337] != 0.0)) && (!(scratch.values[1489] != 0.0))) && (!(scratch.values[1494] != 0.0))) && (!(scratch.values[1496] != 0.0))) {
            scratch.store_ad(1339, &AdValue::div_from_scalar(1.0, AdValue::sub_from_scalar(1.0, AdValue::scale(scratch.ad_value(1377), scratch.values[428]))));
        }

        scratch.values[1497] = if (((-scratch.values[1338]) + scratch.values[1376]) > (-230.25850929940458)) { 1.0 } else { 0.0 };

        if (((((scratch.values[1320] != 0.0) && (scratch.values[1337] != 0.0)) && (!(scratch.values[1489] != 0.0))) && (!(scratch.values[1494] != 0.0))) && (scratch.values[1497] != 0.0)) {
            scratch.store_ad(1356, &AdValue::exp(AdValue::sub(scratch.ad_value(1376), scratch.ad_value(1338))));
        }

        if (((((scratch.values[1320] != 0.0) && (scratch.values[1337] != 0.0)) && (!(scratch.values[1489] != 0.0))) && (!(scratch.values[1494] != 0.0))) && (!(scratch.values[1497] != 0.0))) {
            let assign17620_ad_e17374: AdValue = AdValue::div_from_scalar(1e-100, AdValue::offset(AdValue::mul(AdValue::sub_from_scalar((-230.25850929940458), AdValue::sub(scratch.ad_value(1376), scratch.ad_value(1338))), AdValue::offset(AdValue::scale(AdValue::mul(AdValue::sub_from_scalar((-230.25850929940458), AdValue::sub(scratch.ad_value(1376), scratch.ad_value(1338))), AdValue::offset(AdValue::scale(AdValue::sub_from_scalar((-230.25850929940458), AdValue::sub(scratch.ad_value(1376), scratch.ad_value(1338))), 0.3333333333333333), 1.0)), 0.5), 1.0)), 1.0));
            scratch.store_ad(1356, &assign17620_ad_e17374);
        }

        if ((((scratch.values[1320] != 0.0) && (scratch.values[1337] != 0.0)) && (!(scratch.values[1489] != 0.0))) && (!(scratch.values[1494] != 0.0))) {
            scratch.store_ad(1340, &AdValue::mul(AdValue::add(AdValue::add(AdValue::scale(scratch.ad_value(1339), 0.29214664), AdValue::scale(AdValue::square(scratch.ad_value(1339)), scratch.values[429])), AdValue::scale(AdValue::mul(AdValue::square(scratch.ad_value(1339)), scratch.ad_value(1339)), scratch.values[430])), scratch.ad_value(1356)));
        }

        scratch.values[1498] = if (scratch.values[1377] > 0.0) { 1.0 } else { 0.0 };

        if (((((scratch.values[1320] != 0.0) && (scratch.values[1337] != 0.0)) && (!(scratch.values[1489] != 0.0))) && (!(scratch.values[1494] != 0.0))) && (scratch.values[1498] != 0.0)) {
            scratch.values[1378] = scratch.values[1340];
            scratch.node_derivatives[1378] = scratch.node_derivatives[1340];
            scratch.branch_derivatives[1378] = scratch.branch_derivatives[1340];
        }

        scratch.values[1499] = if (scratch.values[1376] > (-230.25850929940458)) { 1.0 } else { 0.0 };

        if ((((((scratch.values[1320] != 0.0) && (scratch.values[1337] != 0.0)) && (!(scratch.values[1489] != 0.0))) && (!(scratch.values[1494] != 0.0))) && (!(scratch.values[1498] != 0.0))) && (scratch.values[1499] != 0.0)) {
            scratch.store_ad(1356, &AdValue::exp(scratch.ad_value(1376)));
        }

        if ((((((scratch.values[1320] != 0.0) && (scratch.values[1337] != 0.0)) && (!(scratch.values[1489] != 0.0))) && (!(scratch.values[1494] != 0.0))) && (!(scratch.values[1498] != 0.0))) && (!(scratch.values[1499] != 0.0))) {
            scratch.store_ad(1356, &AdValue::div_from_scalar(1e-100, AdValue::offset(AdValue::mul(AdValue::sub_from_scalar((-230.25850929940458), scratch.ad_value(1376)), AdValue::offset(AdValue::scale(AdValue::mul(AdValue::sub_from_scalar((-230.25850929940458), scratch.ad_value(1376)), AdValue::offset(AdValue::scale(AdValue::sub_from_scalar((-230.25850929940458), scratch.ad_value(1376)), 0.3333333333333333), 1.0)), 0.5), 1.0)), 1.0)));
        }

        if (((((scratch.values[1320] != 0.0) && (scratch.values[1337] != 0.0)) && (!(scratch.values[1489] != 0.0))) && (!(scratch.values[1494] != 0.0))) && (!(scratch.values[1498] != 0.0))) {
            scratch.store_ad(1378, &AdValue::sub(AdValue::scale(scratch.ad_value(1356), 2.0), scratch.ad_value(1340)));
        }

        if ((((scratch.values[1320] != 0.0) && (scratch.values[1337] != 0.0)) && (!(scratch.values[1489] != 0.0))) && (!(scratch.values[1494] != 0.0))) {
            scratch.store_ad(1379, &AdValue::scale(AdValue::div(AdValue::scale(scratch.ad_value(1378), scratch.values[493]), scratch.ad_value(1374)), (1.772453850905516 * 0.5)));
        }

        if ((((scratch.values[1320] != 0.0) && (scratch.values[1337] != 0.0)) && (!(scratch.values[1489] != 0.0))) && (!(scratch.values[1494] != 0.0))) {
            scratch.store_ad(1365, &AdValue::scale(AdValue::mul(AdValue::mul(scratch.ad_value(1364), scratch.ad_value(1379)), scratch.ad_value(1373)), scratch.values[398]));
        }

        scratch.values[1500] = if (scratch.values[404] == 0.0) { 1.0 } else { 0.0 };

        if ((((scratch.values[1320] != 0.0) && (scratch.values[1337] != 0.0)) && (!(scratch.values[1489] != 0.0))) && (scratch.values[1500] != 0.0)) {
            scratch.values[1380] = 0.0;
            scratch.node_derivatives[1380] = [0.0; Instance::NODE_COUNT];
            scratch.branch_derivatives[1380] = [0.0; Instance::BRANCH_COUNT];
        }

        scratch.values[1501] = if (scratch.values[384] == 0.5) { 1.0 } else { 0.0 };

        if (((((scratch.values[1320] != 0.0) && (scratch.values[1337] != 0.0)) && (!(scratch.values[1489] != 0.0))) && (!(scratch.values[1500] != 0.0))) && (scratch.values[1501] != 0.0)) {
            scratch.store_ad(1356, &AdValue::sqrt(AdValue::scale(AdValue::sub_from_scalar(scratch.values[381], scratch.ad_value(1354)), scratch.values[487])));
        }

        if (((((scratch.values[1320] != 0.0) && (scratch.values[1337] != 0.0)) && (!(scratch.values[1489] != 0.0))) && (!(scratch.values[1500] != 0.0))) && (!(scratch.values[1501] != 0.0))) {
            scratch.store_ad(1356, &AdValue::powf(AdValue::scale(AdValue::sub_from_scalar(scratch.values[381], scratch.ad_value(1354)), scratch.values[487]), scratch.values[384]));
        }

        if ((((scratch.values[1320] != 0.0) && (scratch.values[1337] != 0.0)) && (!(scratch.values[1489] != 0.0))) && (!(scratch.values[1500] != 0.0))) {
            scratch.store_ad(1381, &AdValue::scale(AdValue::div(AdValue::scale(AdValue::sub_from_scalar(scratch.values[381], scratch.ad_value(1354)), scratch.values[484]), scratch.ad_value(1356)), scratch.values[469]));
        }

        scratch.values[1502] = if (((((-scratch.values[499]) / scratch.values[1381])) as f64).abs() < 230.25850929940458) { 1.0 } else { 0.0 };

        if (((((scratch.values[1320] != 0.0) && (scratch.values[1337] != 0.0)) && (!(scratch.values[1489] != 0.0))) && (!(scratch.values[1500] != 0.0))) && (scratch.values[1502] != 0.0)) {
            scratch.store_ad(1356, &AdValue::exp(AdValue::div(AdValue::neg(scratch.ad_value(499)), scratch.ad_value(1381))));
        }

        scratch.values[1503] = if (((-scratch.values[499]) / scratch.values[1381]) < 0.0) { 1.0 } else { 0.0 };

        if ((((((scratch.values[1320] != 0.0) && (scratch.values[1337] != 0.0)) && (!(scratch.values[1489] != 0.0))) && (!(scratch.values[1500] != 0.0))) && (!(scratch.values[1502] != 0.0))) && (scratch.values[1503] != 0.0)) {
            let assign17810_ad_e17701: AdValue = AdValue::offset(AdValue::mul(AdValue::sub_from_scalar((-230.25850929940458), AdValue::div(AdValue::neg(scratch.ad_value(499)), scratch.ad_value(1381))), AdValue::offset(AdValue::scale(AdValue::mul(AdValue::sub_from_scalar((-230.25850929940458), AdValue::div(AdValue::neg(scratch.ad_value(499)), scratch.ad_value(1381))), AdValue::offset(AdValue::scale(AdValue::sub_from_scalar((-230.25850929940458), AdValue::div(AdValue::neg(scratch.ad_value(499)), scratch.ad_value(1381))), 0.3333333333333333), 1.0)), 0.5), 1.0)), 1.0);
            scratch.store_ad(1356, &AdValue::div_from_scalar(1e-100, assign17810_ad_e17701));
        }

        if ((((((scratch.values[1320] != 0.0) && (scratch.values[1337] != 0.0)) && (!(scratch.values[1489] != 0.0))) && (!(scratch.values[1500] != 0.0))) && (!(scratch.values[1502] != 0.0))) && (!(scratch.values[1503] != 0.0))) {
            let assign17820_ad_e17751: AdValue = AdValue::scale(AdValue::offset(AdValue::mul(AdValue::offset(AdValue::div(AdValue::neg(scratch.ad_value(499)), scratch.ad_value(1381)), (-230.25850929940458)), AdValue::offset(AdValue::scale(AdValue::mul(AdValue::offset(AdValue::div(AdValue::neg(scratch.ad_value(499)), scratch.ad_value(1381)), (-230.25850929940458)), AdValue::offset(AdValue::scale(AdValue::offset(AdValue::div(AdValue::neg(scratch.ad_value(499)), scratch.ad_value(1381)), (-230.25850929940458)), 0.3333333333333333), 1.0)), 0.5), 1.0)), 1.0), 1e100);
            scratch.store_ad(1356, &assign17820_ad_e17751);
        }

        if ((((scratch.values[1320] != 0.0) && (scratch.values[1337] != 0.0)) && (!(scratch.values[1489] != 0.0))) && (!(scratch.values[1500] != 0.0))) {
            scratch.store_ad(1380, &AdValue::scale(AdValue::mul(AdValue::mul(AdValue::mul(scratch.ad_value(524), scratch.ad_value(1381)), scratch.ad_value(1381)), scratch.ad_value(1356)), scratch.values[404]));
        }

        scratch.values[1504] = if (scratch.values[413] > 1000.0) { 1.0 } else { 0.0 };

        if ((((scratch.values[1320] != 0.0) && (scratch.values[1337] != 0.0)) && (!(scratch.values[1489] != 0.0))) && (scratch.values[1504] != 0.0)) {
            scratch.values[1382] = 1.0;
            scratch.node_derivatives[1382] = [0.0; Instance::NODE_COUNT];
            scratch.branch_derivatives[1382] = [0.0; Instance::BRANCH_COUNT];
        }

        scratch.values[1505] = if (scratch.values[1355] > ((-scratch.values[500]) * scratch.values[413])) { 1.0 } else { 0.0 };

        scratch.values[1506] = if (scratch.values[416] == 4.0) { 1.0 } else { 0.0 };

        if ((((((scratch.values[1320] != 0.0) && (scratch.values[1337] != 0.0)) && (!(scratch.values[1489] != 0.0))) && (!(scratch.values[1504] != 0.0))) && (scratch.values[1505] != 0.0)) && (scratch.values[1506] != 0.0)) {
            scratch.store_ad(1356, &AdValue::mul(AdValue::mul(AdValue::mul(AdValue::scale(scratch.ad_value(1355), scratch.values[506]), AdValue::scale(scratch.ad_value(1355), scratch.values[506])), AdValue::scale(scratch.ad_value(1355), scratch.values[506])), AdValue::scale(scratch.ad_value(1355), scratch.values[506])));
        }

        if ((((((scratch.values[1320] != 0.0) && (scratch.values[1337] != 0.0)) && (!(scratch.values[1489] != 0.0))) && (!(scratch.values[1504] != 0.0))) && (scratch.values[1505] != 0.0)) && (!(scratch.values[1506] != 0.0))) {
            scratch.store_ad(1356, &AdValue::powf(AdValue::abs(AdValue::scale(scratch.ad_value(1355), scratch.values[506])), scratch.values[416]));
        }

        if (((((scratch.values[1320] != 0.0) && (scratch.values[1337] != 0.0)) && (!(scratch.values[1489] != 0.0))) && (!(scratch.values[1504] != 0.0))) && (scratch.values[1505] != 0.0)) {
            scratch.store_ad(1382, &AdValue::div_from_scalar(1.0, AdValue::sub_from_scalar(1.0, scratch.ad_value(1356))));
        }

        if (((((scratch.values[1320] != 0.0) && (scratch.values[1337] != 0.0)) && (!(scratch.values[1489] != 0.0))) && (!(scratch.values[1504] != 0.0))) && (!(scratch.values[1505] != 0.0))) {
            scratch.store_ad(1382, &AdValue::offset(AdValue::scale(AdValue::offset(scratch.ad_value(1355), (scratch.values[500] * scratch.values[413])), scratch.values[509]), scratch.values[503]));
        }

        if (((scratch.values[1320] != 0.0) && (scratch.values[1337] != 0.0)) && (!(scratch.values[1489] != 0.0))) {
            scratch.store_ad(1385, &AdValue::mul(AdValue::add(AdValue::add(AdValue::add(scratch.ad_value(1357), scratch.ad_value(1358)), scratch.ad_value(1365)), scratch.ad_value(1380)), scratch.ad_value(1382)));
        }

        if ((scratch.values[1320] != 0.0) && (scratch.values[1337] != 0.0)) {
            scratch.store_ad(514, &AdValue::add(AdValue::add(AdValue::mul(scratch.ad_value(697), scratch.ad_value(1383)), AdValue::mul(scratch.ad_value(698), scratch.ad_value(1384))), AdValue::mul(scratch.ad_value(699), scratch.ad_value(1385))));
        }

        if ((scratch.values[1320] != 0.0) && (scratch.values[1337] != 0.0)) {
            scratch.values[1354] = 0.0;
            scratch.node_derivatives[1354] = [0.0; Instance::NODE_COUNT];
            scratch.branch_derivatives[1354] = [0.0; Instance::BRANCH_COUNT];
        }

        if ((scratch.values[1320] != 0.0) && (scratch.values[1337] != 0.0)) {
            scratch.values[1351] = 0.0;
            scratch.node_derivatives[1351] = [0.0; Instance::NODE_COUNT];
            scratch.branch_derivatives[1351] = [0.0; Instance::BRANCH_COUNT];
        }

        scratch.values[1507] = if !(((scratch.values[697] == 0.0) && (scratch.values[698] == 0.0)) && (scratch.values[699] == 0.0)) { 1.0 } else { 0.0 };

        if (((scratch.values[1320] != 0.0) && (scratch.values[1337] != 0.0)) && (scratch.values[1507] != 0.0)) {
            scratch.store_ad(1341, &AdValue::mul(AdValue::scale(scratch.ad_value(708), 4.0), scratch.ad_value(708)));
        }

        if (((scratch.values[1320] != 0.0) && (scratch.values[1337] != 0.0)) && (scratch.values[1507] != 0.0)) {
            scratch.store_ad(1342, &AdValue::div(scratch.ad_value(708), scratch.ad_value(709)));
        }

        if (((scratch.values[1320] != 0.0) && (scratch.values[1337] != 0.0)) && (scratch.values[1507] != 0.0)) {
            scratch.store_ad(1343, &AdValue::add(scratch.ad_value(525), AdValue::mul(scratch.ad_value(708), scratch.ad_value(1342))));
        }

        if (((scratch.values[1320] != 0.0) && (scratch.values[1337] != 0.0)) && (scratch.values[1507] != 0.0)) {
            scratch.store_ad(1344, &AdValue::add(scratch.ad_value(709), scratch.ad_value(1343)));
        }

        if (((scratch.values[1320] != 0.0) && (scratch.values[1337] != 0.0)) && (scratch.values[1507] != 0.0)) {
            scratch.store_ad(1345, &AdValue::sub(scratch.ad_value(709), scratch.ad_value(1343)));
        }

        if (((scratch.values[1320] != 0.0) && (scratch.values[1337] != 0.0)) && (scratch.values[1507] != 0.0)) {
            scratch.store_ad(1346, &AdValue::sqrt(AdValue::add(AdValue::square(scratch.ad_value(1345)), scratch.ad_value(1341))));
        }

        if (((scratch.values[1320] != 0.0) && (scratch.values[1337] != 0.0)) && (scratch.values[1507] != 0.0)) {
            scratch.store_ad(1348, &AdValue::scale(AdValue::div(AdValue::mul(scratch.ad_value(525), scratch.ad_value(709)), AdValue::add(scratch.ad_value(1344), scratch.ad_value(1346))), 2.0));
        }

        scratch.values[1508] = if (scratch.values[525] < scratch.values[705]) { 1.0 } else { 0.0 };

        scratch.values[1509] = if ((((0.5 * (scratch.values[525] * scratch.values[427]))) as f64).abs() < 230.25850929940458) { 1.0 } else { 0.0 };

        if (((((scratch.values[1320] != 0.0) && (scratch.values[1337] != 0.0)) && (scratch.values[1507] != 0.0)) && (scratch.values[1508] != 0.0)) && (scratch.values[1509] != 0.0)) {
            scratch.store_ad(1350, &AdValue::exp(AdValue::scale(scratch.ad_value(525), (scratch.values[427] * 0.5))));
        }

        scratch.values[1510] = if ((0.5 * (scratch.values[525] * scratch.values[427])) < 0.0) { 1.0 } else { 0.0 };

        if ((((((scratch.values[1320] != 0.0) && (scratch.values[1337] != 0.0)) && (scratch.values[1507] != 0.0)) && (scratch.values[1508] != 0.0)) && (!(scratch.values[1509] != 0.0))) && (scratch.values[1510] != 0.0)) {
            let assign18080_ad_e18114: AdValue = AdValue::div_from_scalar(1e-100, AdValue::offset(AdValue::mul(AdValue::sub_from_scalar((-230.25850929940458), AdValue::scale(scratch.ad_value(525), (scratch.values[427] * 0.5))), AdValue::offset(AdValue::scale(AdValue::mul(AdValue::sub_from_scalar((-230.25850929940458), AdValue::scale(scratch.ad_value(525), (scratch.values[427] * 0.5))), AdValue::offset(AdValue::scale(AdValue::sub_from_scalar((-230.25850929940458), AdValue::scale(scratch.ad_value(525), (scratch.values[427] * 0.5))), 0.3333333333333333), 1.0)), 0.5), 1.0)), 1.0));
            scratch.store_ad(1350, &assign18080_ad_e18114);
        }

        if ((((((scratch.values[1320] != 0.0) && (scratch.values[1337] != 0.0)) && (scratch.values[1507] != 0.0)) && (scratch.values[1508] != 0.0)) && (!(scratch.values[1509] != 0.0))) && (!(scratch.values[1510] != 0.0))) {
            scratch.store_ad(1350, &AdValue::scale(AdValue::offset(AdValue::mul(AdValue::offset(AdValue::scale(scratch.ad_value(525), (scratch.values[427] * 0.5)), (-230.25850929940458)), AdValue::offset(AdValue::scale(AdValue::mul(AdValue::offset(AdValue::scale(scratch.ad_value(525), (scratch.values[427] * 0.5)), (-230.25850929940458)), AdValue::offset(AdValue::scale(AdValue::offset(AdValue::scale(scratch.ad_value(525), (scratch.values[427] * 0.5)), (-230.25850929940458)), 0.3333333333333333), 1.0)), 0.5), 1.0)), 1.0), 1e100));
        }

        if ((((scratch.values[1320] != 0.0) && (scratch.values[1337] != 0.0)) && (scratch.values[1507] != 0.0)) && (scratch.values[1508] != 0.0)) {
            scratch.store_ad(1347, &AdValue::square(scratch.ad_value(1350)));
        }

        if ((((scratch.values[1320] != 0.0) && (scratch.values[1337] != 0.0)) && (scratch.values[1507] != 0.0)) && (!(scratch.values[1508] != 0.0))) {
            scratch.store_ad(1347, &AdValue::mul(AdValue::offset(AdValue::scale(AdValue::sub(scratch.ad_value(525), scratch.ad_value(705)), scratch.values[427]), 1.0), scratch.ad_value(706)));
        }

        if ((((scratch.values[1320] != 0.0) && (scratch.values[1337] != 0.0)) && (scratch.values[1507] != 0.0)) && (!(scratch.values[1508] != 0.0))) {
            scratch.store_ad(1350, &AdValue::sqrt(scratch.ad_value(1347)));
        }

        if (((scratch.values[1320] != 0.0) && (scratch.values[1337] != 0.0)) && (scratch.values[1507] != 0.0)) {
            scratch.store_ad(1347, &AdValue::offset(scratch.ad_value(1347), (-1.0)));
        }

        if (((scratch.values[1320] != 0.0) && (scratch.values[1337] != 0.0)) && (scratch.values[1507] != 0.0)) {
            scratch.store_ad(1349, &AdValue::div_from_scalar(1.0, scratch.ad_value(1350)));
        }

        scratch.values[1511] = if (scratch.values[525] > 0.0) { 1.0 } else { 0.0 };

        if ((((scratch.values[1320] != 0.0) && (scratch.values[1337] != 0.0)) && (scratch.values[1507] != 0.0)) && (scratch.values[1511] != 0.0)) {
            scratch.store_ad(1351, &AdValue::scale(AdValue::ln(AdValue::add(AdValue::offset(scratch.ad_value(1349), 2.0), AdValue::sqrt(AdValue::mul(AdValue::offset(scratch.ad_value(1349), 1.0), AdValue::offset(scratch.ad_value(1349), 3.0))))), (scratch.values[426] * 2.0)));
        }

        if ((((scratch.values[1320] != 0.0) && (scratch.values[1337] != 0.0)) && (scratch.values[1507] != 0.0)) && (!(scratch.values[1511] != 0.0))) {
            scratch.store_ad(1351, &AdValue::sub(AdValue::scale(AdValue::ln(AdValue::add(AdValue::offset(AdValue::scale(scratch.ad_value(1350), 2.0), 1.0), AdValue::sqrt(AdValue::mul(AdValue::offset(scratch.ad_value(1350), 1.0), AdValue::offset(AdValue::scale(scratch.ad_value(1350), 3.0), 1.0))))), (scratch.values[426] * 2.0)), scratch.ad_value(525)));
        }

        if (((scratch.values[1320] != 0.0) && (scratch.values[1337] != 0.0)) && (scratch.values[1507] != 0.0)) {
            scratch.store_ad(1352, &AdValue::sub(scratch.ad_value(707), scratch.ad_value(1351)));
        }

        if (((scratch.values[1320] != 0.0) && (scratch.values[1337] != 0.0)) && (scratch.values[1507] != 0.0)) {
            scratch.store_ad(1353, &AdValue::scale(AdValue::sub(AdValue::add(scratch.ad_value(525), scratch.ad_value(1352)), AdValue::sqrt(AdValue::offset(AdValue::mul(AdValue::sub(scratch.ad_value(525), scratch.ad_value(1352)), AdValue::sub(scratch.ad_value(525), scratch.ad_value(1352))), ((4.0 * scratch.values[426]) * scratch.values[426])))), 0.5));
        }

        if (((scratch.values[1320] != 0.0) && (scratch.values[1337] != 0.0)) && (scratch.values[1507] != 0.0)) {
            scratch.store_ad(1354, &AdValue::scale(AdValue::sub(AdValue::add(scratch.ad_value(525), scratch.ad_value(710)), AdValue::sqrt(AdValue::offset(AdValue::mul(AdValue::sub(scratch.ad_value(525), scratch.ad_value(710)), AdValue::sub(scratch.ad_value(525), scratch.ad_value(710))), ((4.0 * scratch.values[424]) * scratch.values[424])))), 0.5));
        }

        if (((scratch.values[1320] != 0.0) && (scratch.values[1337] != 0.0)) && (scratch.values[1507] != 0.0)) {
            scratch.store_ad(1355, &AdValue::scale(AdValue::sub(scratch.ad_value(525), AdValue::sqrt(AdValue::offset(AdValue::mul(scratch.ad_value(525), scratch.ad_value(525)), ((4.0 * 1e-6) * 1e-6)))), 0.5));
        }

        scratch.values[1512] = if (scratch.values[697] == 0.0) { 1.0 } else { 0.0 };

        if (((scratch.values[1320] != 0.0) && (scratch.values[1337] != 0.0)) && (scratch.values[1512] != 0.0)) {
            scratch.values[1383] = 0.0;
            scratch.node_derivatives[1383] = [0.0; Instance::NODE_COUNT];
            scratch.branch_derivatives[1383] = [0.0; Instance::BRANCH_COUNT];
        }

        scratch.values[1513] = if (scratch.values[464] == 0.5) { 1.0 } else { 0.0 };

        if ((((scratch.values[1320] != 0.0) && (scratch.values[1337] != 0.0)) && (!(scratch.values[1512] != 0.0))) && (scratch.values[1513] != 0.0)) {
            scratch.store_ad(1356, &AdValue::sqrt(AdValue::sub_from_scalar(1.0, AdValue::scale(scratch.ad_value(1348), scratch.values[461]))));
        }

        if ((((scratch.values[1320] != 0.0) && (scratch.values[1337] != 0.0)) && (!(scratch.values[1512] != 0.0))) && (!(scratch.values[1513] != 0.0))) {
            scratch.store_ad(1356, &AdValue::powf(AdValue::sub_from_scalar(1.0, AdValue::scale(scratch.ad_value(1348), scratch.values[461])), scratch.values[464]));
        }

        if (((scratch.values[1320] != 0.0) && (scratch.values[1337] != 0.0)) && (!(scratch.values[1512] != 0.0))) {
            scratch.store_ad(1357, &AdValue::scale(scratch.ad_value(1347), scratch.values[443]));
        }

        scratch.values[1514] = if ((scratch.values[393] == 0.0) && (scratch.values[396] == 0.0)) { 1.0 } else { 0.0 };

        if ((((scratch.values[1320] != 0.0) && (scratch.values[1337] != 0.0)) && (!(scratch.values[1512] != 0.0))) && (scratch.values[1514] != 0.0)) {
            scratch.values[1358] = 0.0;
            scratch.node_derivatives[1358] = [0.0; Instance::NODE_COUNT];
            scratch.branch_derivatives[1358] = [0.0; Instance::BRANCH_COUNT];
        }

        if ((((scratch.values[1320] != 0.0) && (scratch.values[1337] != 0.0)) && (!(scratch.values[1512] != 0.0))) && (!(scratch.values[1514] != 0.0))) {
            scratch.store_ad(1359, &AdValue::sub_from_scalar(scratch.values[449], scratch.ad_value(1353)));
        }

        if ((((scratch.values[1320] != 0.0) && (scratch.values[1337] != 0.0)) && (!(scratch.values[1512] != 0.0))) && (!(scratch.values[1514] != 0.0))) {
            scratch.store_ad(1360, &AdValue::sub_from_scalar(1.0, AdValue::sqrt(AdValue::sub_from_scalar(1.0, AdValue::div(scratch.ad_value(1351), scratch.ad_value(1359))))));
        }

        scratch.values[1515] = if (scratch.values[382] == 0.5) { 1.0 } else { 0.0 };

        if (((((scratch.values[1320] != 0.0) && (scratch.values[1337] != 0.0)) && (!(scratch.values[1512] != 0.0))) && (!(scratch.values[1514] != 0.0))) && (scratch.values[1515] != 0.0)) {
            scratch.values[1361] = 0.0;
            scratch.node_derivatives[1361] = [0.0; Instance::NODE_COUNT];
            scratch.branch_derivatives[1361] = [0.0; Instance::BRANCH_COUNT];
        }

        if (((((scratch.values[1320] != 0.0) && (scratch.values[1337] != 0.0)) && (!(scratch.values[1512] != 0.0))) && (!(scratch.values[1514] != 0.0))) && (!(scratch.values[1515] != 0.0))) {
            scratch.store_ad(1361, &AdValue::scale(AdValue::add(AdValue::div(AdValue::mul(AdValue::square(scratch.ad_value(1360)), AdValue::ln(scratch.ad_value(1360))), AdValue::sub_from_scalar(1.0, scratch.ad_value(1360))), scratch.ad_value(1360)), (1.0 - (2.0 * scratch.values[382]))));
        }

        if ((((scratch.values[1320] != 0.0) && (scratch.values[1337] != 0.0)) && (!(scratch.values[1512] != 0.0))) && (!(scratch.values[1514] != 0.0))) {
            scratch.store_ad(1362, &AdValue::add(scratch.ad_value(1360), scratch.ad_value(1361)));
        }

        scratch.values[1516] = if (scratch.values[382] == 0.5) { 1.0 } else { 0.0 };

        if (((((scratch.values[1320] != 0.0) && (scratch.values[1337] != 0.0)) && (!(scratch.values[1512] != 0.0))) && (!(scratch.values[1514] != 0.0))) && (scratch.values[1516] != 0.0)) {
            scratch.store_ad(1356, &AdValue::sqrt(AdValue::scale(scratch.ad_value(1359), scratch.values[485])));
        }

        if (((((scratch.values[1320] != 0.0) && (scratch.values[1337] != 0.0)) && (!(scratch.values[1512] != 0.0))) && (!(scratch.values[1514] != 0.0))) && (!(scratch.values[1516] != 0.0))) {
            scratch.store_ad(1356, &AdValue::powf(AdValue::scale(scratch.ad_value(1359), scratch.values[485]), scratch.values[382]));
        }

        if ((((scratch.values[1320] != 0.0) && (scratch.values[1337] != 0.0)) && (!(scratch.values[1512] != 0.0))) && (!(scratch.values[1514] != 0.0))) {
            scratch.store_ad(1363, &AdValue::scale(scratch.ad_value(1356), scratch.values[479]));
        }

        if ((((scratch.values[1320] != 0.0) && (scratch.values[1337] != 0.0)) && (!(scratch.values[1512] != 0.0))) && (!(scratch.values[1514] != 0.0))) {
            scratch.store_ad(1364, &AdValue::scale(AdValue::mul(AdValue::offset(scratch.ad_value(1350), (-1.0)), scratch.ad_value(1363)), scratch.values[440]));
        }

        if ((((scratch.values[1320] != 0.0) && (scratch.values[1337] != 0.0)) && (!(scratch.values[1512] != 0.0))) && (!(scratch.values[1514] != 0.0))) {
            scratch.store_ad(1358, &AdValue::scale(AdValue::mul(scratch.ad_value(1364), scratch.ad_value(1362)), scratch.values[393]));
        }

        scratch.values[1517] = if (scratch.values[396] == 0.0) { 1.0 } else { 0.0 };

        if ((((scratch.values[1320] != 0.0) && (scratch.values[1337] != 0.0)) && (!(scratch.values[1512] != 0.0))) && (scratch.values[1517] != 0.0)) {
            scratch.values[1365] = 0.0;
            scratch.node_derivatives[1365] = [0.0; Instance::NODE_COUNT];
            scratch.branch_derivatives[1365] = [0.0; Instance::BRANCH_COUNT];
        }

        if ((((scratch.values[1320] != 0.0) && (scratch.values[1337] != 0.0)) && (!(scratch.values[1512] != 0.0))) && (!(scratch.values[1517] != 0.0))) {
            scratch.store_ad(1366, &AdValue::scale(AdValue::div(AdValue::scale(scratch.ad_value(1363), scratch.values[464]), scratch.ad_value(1359)), scratch.values[494]));
        }

        if ((((scratch.values[1320] != 0.0) && (scratch.values[1337] != 0.0)) && (!(scratch.values[1512] != 0.0))) && (!(scratch.values[1517] != 0.0))) {
            scratch.store_ad(1367, &AdValue::div_from_scalar((0.666666666666667 * scratch.values[491]), scratch.ad_value(1366)));
        }

        if ((((scratch.values[1320] != 0.0) && (scratch.values[1337] != 0.0)) && (!(scratch.values[1512] != 0.0))) && (!(scratch.values[1517] != 0.0))) {
            scratch.store_ad(1368, &AdValue::square(scratch.ad_value(1367)));
        }

        if ((((scratch.values[1320] != 0.0) && (scratch.values[1337] != 0.0)) && (!(scratch.values[1512] != 0.0))) && (!(scratch.values[1517] != 0.0))) {
            scratch.store_ad(1369, &AdValue::sqrt(AdValue::div(AdValue::square(scratch.ad_value(1368)), AdValue::offset(AdValue::square(scratch.ad_value(1368)), 1.0))));
        }

        if ((((scratch.values[1320] != 0.0) && (scratch.values[1337] != 0.0)) && (!(scratch.values[1512] != 0.0))) && (!(scratch.values[1517] != 0.0))) {
            scratch.store_ad(1370, &AdValue::sqrt(AdValue::abs(scratch.ad_value(1369))));
        }

        if ((((scratch.values[1320] != 0.0) && (scratch.values[1337] != 0.0)) && (!(scratch.values[1512] != 0.0))) && (!(scratch.values[1517] != 0.0))) {
            scratch.store_ad(1371, &AdValue::mul(scratch.ad_value(1369), scratch.ad_value(1370)));
        }

        scratch.values[1518] = if (((-scratch.values[382]) * scratch.values[467]) == (-1.0)) { 1.0 } else { 0.0 };

        if (((((scratch.values[1320] != 0.0) && (scratch.values[1337] != 0.0)) && (!(scratch.values[1512] != 0.0))) && (!(scratch.values[1517] != 0.0))) && (scratch.values[1518] != 0.0)) {
            scratch.store_ad(1372, &AdValue::div_from_scalar(1.0, AdValue::offset(AdValue::mul(scratch.ad_value(1366), scratch.ad_value(1371)), 1.0)));
        }

        if (((((scratch.values[1320] != 0.0) && (scratch.values[1337] != 0.0)) && (!(scratch.values[1512] != 0.0))) && (!(scratch.values[1517] != 0.0))) && (!(scratch.values[1518] != 0.0))) {
            scratch.store_ad(1372, &AdValue::powf(AdValue::offset(AdValue::mul(scratch.ad_value(1366), scratch.ad_value(1371)), 1.0), ((-scratch.values[382]) * scratch.values[467])));
        }

        if ((((scratch.values[1320] != 0.0) && (scratch.values[1337] != 0.0)) && (!(scratch.values[1512] != 0.0))) && (!(scratch.values[1517] != 0.0))) {
            scratch.store_ad(1373, &AdValue::div(AdValue::mul(scratch.ad_value(1362), scratch.ad_value(1372)), AdValue::add(scratch.ad_value(1362), scratch.ad_value(1372))));
        }

        if ((((scratch.values[1320] != 0.0) && (scratch.values[1337] != 0.0)) && (!(scratch.values[1512] != 0.0))) && (!(scratch.values[1517] != 0.0))) {
            scratch.store_ad(1374, &AdValue::sqrt(AdValue::scale(AdValue::div(scratch.ad_value(1366), scratch.ad_value(1370)), 0.375)));
        }

        if ((((scratch.values[1320] != 0.0) && (scratch.values[1337] != 0.0)) && (!(scratch.values[1512] != 0.0))) && (!(scratch.values[1517] != 0.0))) {
            scratch.store_ad(1375, &AdValue::sub(AdValue::scale(AdValue::mul(scratch.ad_value(1367), scratch.ad_value(1370)), 2.0), scratch.ad_value(1369)));
        }

        if ((((scratch.values[1320] != 0.0) && (scratch.values[1337] != 0.0)) && (!(scratch.values[1512] != 0.0))) && (!(scratch.values[1517] != 0.0))) {
            scratch.store_ad(1376, &AdValue::add(AdValue::sub(AdValue::mul(AdValue::scale(scratch.ad_value(1367), scratch.values[491]), scratch.ad_value(1370)), AdValue::scale(scratch.ad_value(1369), scratch.values[491])), AdValue::scale(AdValue::mul(scratch.ad_value(1366), scratch.ad_value(1371)), 0.5)));
        }

        if ((((scratch.values[1320] != 0.0) && (scratch.values[1337] != 0.0)) && (!(scratch.values[1512] != 0.0))) && (!(scratch.values[1517] != 0.0))) {
            scratch.store_ad(1377, &AdValue::mul(AdValue::offset(scratch.ad_value(1375), (-1.0)), scratch.ad_value(1374)));
        }

        if ((((scratch.values[1320] != 0.0) && (scratch.values[1337] != 0.0)) && (!(scratch.values[1512] != 0.0))) && (!(scratch.values[1517] != 0.0))) {
            scratch.store_ad(1338, &AdValue::square(scratch.ad_value(1377)));
        }

        scratch.values[1519] = if (scratch.values[1377] > 0.0) { 1.0 } else { 0.0 };

        if (((((scratch.values[1320] != 0.0) && (scratch.values[1337] != 0.0)) && (!(scratch.values[1512] != 0.0))) && (!(scratch.values[1517] != 0.0))) && (scratch.values[1519] != 0.0)) {
            scratch.store_ad(1339, &AdValue::div_from_scalar(1.0, AdValue::offset(AdValue::scale(scratch.ad_value(1377), scratch.values[428]), 1.0)));
        }

        if (((((scratch.values[1320] != 0.0) && (scratch.values[1337] != 0.0)) && (!(scratch.values[1512] != 0.0))) && (!(scratch.values[1517] != 0.0))) && (!(scratch.values[1519] != 0.0))) {
            scratch.store_ad(1339, &AdValue::div_from_scalar(1.0, AdValue::sub_from_scalar(1.0, AdValue::scale(scratch.ad_value(1377), scratch.values[428]))));
        }

        scratch.values[1520] = if (((-scratch.values[1338]) + scratch.values[1376]) > (-230.25850929940458)) { 1.0 } else { 0.0 };

        if (((((scratch.values[1320] != 0.0) && (scratch.values[1337] != 0.0)) && (!(scratch.values[1512] != 0.0))) && (!(scratch.values[1517] != 0.0))) && (scratch.values[1520] != 0.0)) {
            scratch.store_ad(1356, &AdValue::exp(AdValue::sub(scratch.ad_value(1376), scratch.ad_value(1338))));
        }

        if (((((scratch.values[1320] != 0.0) && (scratch.values[1337] != 0.0)) && (!(scratch.values[1512] != 0.0))) && (!(scratch.values[1517] != 0.0))) && (!(scratch.values[1520] != 0.0))) {
            let assign18660_ad_e19057: AdValue = AdValue::div_from_scalar(1e-100, AdValue::offset(AdValue::mul(AdValue::sub_from_scalar((-230.25850929940458), AdValue::sub(scratch.ad_value(1376), scratch.ad_value(1338))), AdValue::offset(AdValue::scale(AdValue::mul(AdValue::sub_from_scalar((-230.25850929940458), AdValue::sub(scratch.ad_value(1376), scratch.ad_value(1338))), AdValue::offset(AdValue::scale(AdValue::sub_from_scalar((-230.25850929940458), AdValue::sub(scratch.ad_value(1376), scratch.ad_value(1338))), 0.3333333333333333), 1.0)), 0.5), 1.0)), 1.0));
            scratch.store_ad(1356, &assign18660_ad_e19057);
        }

        if ((((scratch.values[1320] != 0.0) && (scratch.values[1337] != 0.0)) && (!(scratch.values[1512] != 0.0))) && (!(scratch.values[1517] != 0.0))) {
            scratch.store_ad(1340, &AdValue::mul(AdValue::add(AdValue::add(AdValue::scale(scratch.ad_value(1339), 0.29214664), AdValue::scale(AdValue::square(scratch.ad_value(1339)), scratch.values[429])), AdValue::scale(AdValue::mul(AdValue::square(scratch.ad_value(1339)), scratch.ad_value(1339)), scratch.values[430])), scratch.ad_value(1356)));
        }

        scratch.values[1521] = if (scratch.values[1377] > 0.0) { 1.0 } else { 0.0 };

        if (((((scratch.values[1320] != 0.0) && (scratch.values[1337] != 0.0)) && (!(scratch.values[1512] != 0.0))) && (!(scratch.values[1517] != 0.0))) && (scratch.values[1521] != 0.0)) {
            scratch.values[1378] = scratch.values[1340];
            scratch.node_derivatives[1378] = scratch.node_derivatives[1340];
            scratch.branch_derivatives[1378] = scratch.branch_derivatives[1340];
        }

        scratch.values[1522] = if (scratch.values[1376] > (-230.25850929940458)) { 1.0 } else { 0.0 };

        if ((((((scratch.values[1320] != 0.0) && (scratch.values[1337] != 0.0)) && (!(scratch.values[1512] != 0.0))) && (!(scratch.values[1517] != 0.0))) && (!(scratch.values[1521] != 0.0))) && (scratch.values[1522] != 0.0)) {
            scratch.store_ad(1356, &AdValue::exp(scratch.ad_value(1376)));
        }

        if ((((((scratch.values[1320] != 0.0) && (scratch.values[1337] != 0.0)) && (!(scratch.values[1512] != 0.0))) && (!(scratch.values[1517] != 0.0))) && (!(scratch.values[1521] != 0.0))) && (!(scratch.values[1522] != 0.0))) {
            scratch.store_ad(1356, &AdValue::div_from_scalar(1e-100, AdValue::offset(AdValue::mul(AdValue::sub_from_scalar((-230.25850929940458), scratch.ad_value(1376)), AdValue::offset(AdValue::scale(AdValue::mul(AdValue::sub_from_scalar((-230.25850929940458), scratch.ad_value(1376)), AdValue::offset(AdValue::scale(AdValue::sub_from_scalar((-230.25850929940458), scratch.ad_value(1376)), 0.3333333333333333), 1.0)), 0.5), 1.0)), 1.0)));
        }

        if (((((scratch.values[1320] != 0.0) && (scratch.values[1337] != 0.0)) && (!(scratch.values[1512] != 0.0))) && (!(scratch.values[1517] != 0.0))) && (!(scratch.values[1521] != 0.0))) {
            scratch.store_ad(1378, &AdValue::sub(AdValue::scale(scratch.ad_value(1356), 2.0), scratch.ad_value(1340)));
        }

        if ((((scratch.values[1320] != 0.0) && (scratch.values[1337] != 0.0)) && (!(scratch.values[1512] != 0.0))) && (!(scratch.values[1517] != 0.0))) {
            scratch.store_ad(1379, &AdValue::scale(AdValue::div(AdValue::scale(scratch.ad_value(1378), scratch.values[491]), scratch.ad_value(1374)), (1.772453850905516 * 0.5)));
        }

        if ((((scratch.values[1320] != 0.0) && (scratch.values[1337] != 0.0)) && (!(scratch.values[1512] != 0.0))) && (!(scratch.values[1517] != 0.0))) {
            scratch.store_ad(1365, &AdValue::scale(AdValue::mul(AdValue::mul(scratch.ad_value(1364), scratch.ad_value(1379)), scratch.ad_value(1373)), scratch.values[396]));
        }

        scratch.values[1523] = if (scratch.values[402] == 0.0) { 1.0 } else { 0.0 };

        if ((((scratch.values[1320] != 0.0) && (scratch.values[1337] != 0.0)) && (!(scratch.values[1512] != 0.0))) && (scratch.values[1523] != 0.0)) {
            scratch.values[1380] = 0.0;
            scratch.node_derivatives[1380] = [0.0; Instance::NODE_COUNT];
            scratch.branch_derivatives[1380] = [0.0; Instance::BRANCH_COUNT];
        }

        scratch.values[1524] = if (scratch.values[382] == 0.5) { 1.0 } else { 0.0 };

        if (((((scratch.values[1320] != 0.0) && (scratch.values[1337] != 0.0)) && (!(scratch.values[1512] != 0.0))) && (!(scratch.values[1523] != 0.0))) && (scratch.values[1524] != 0.0)) {
            scratch.store_ad(1356, &AdValue::sqrt(AdValue::scale(AdValue::sub_from_scalar(scratch.values[379], scratch.ad_value(1354)), scratch.values[485])));
        }

        if (((((scratch.values[1320] != 0.0) && (scratch.values[1337] != 0.0)) && (!(scratch.values[1512] != 0.0))) && (!(scratch.values[1523] != 0.0))) && (!(scratch.values[1524] != 0.0))) {
            scratch.store_ad(1356, &AdValue::powf(AdValue::scale(AdValue::sub_from_scalar(scratch.values[379], scratch.ad_value(1354)), scratch.values[485]), scratch.values[382]));
        }

        if ((((scratch.values[1320] != 0.0) && (scratch.values[1337] != 0.0)) && (!(scratch.values[1512] != 0.0))) && (!(scratch.values[1523] != 0.0))) {
            scratch.store_ad(1381, &AdValue::scale(AdValue::div(AdValue::scale(AdValue::sub_from_scalar(scratch.values[379], scratch.ad_value(1354)), scratch.values[482]), scratch.ad_value(1356)), scratch.values[467]));
        }

        scratch.values[1525] = if (((((-scratch.values[497]) / scratch.values[1381])) as f64).abs() < 230.25850929940458) { 1.0 } else { 0.0 };

        if (((((scratch.values[1320] != 0.0) && (scratch.values[1337] != 0.0)) && (!(scratch.values[1512] != 0.0))) && (!(scratch.values[1523] != 0.0))) && (scratch.values[1525] != 0.0)) {
            scratch.store_ad(1356, &AdValue::exp(AdValue::div(AdValue::neg(scratch.ad_value(497)), scratch.ad_value(1381))));
        }

        scratch.values[1526] = if (((-scratch.values[497]) / scratch.values[1381]) < 0.0) { 1.0 } else { 0.0 };

        if ((((((scratch.values[1320] != 0.0) && (scratch.values[1337] != 0.0)) && (!(scratch.values[1512] != 0.0))) && (!(scratch.values[1523] != 0.0))) && (!(scratch.values[1525] != 0.0))) && (scratch.values[1526] != 0.0)) {
            let assign18850_ad_e19384: AdValue = AdValue::offset(AdValue::mul(AdValue::sub_from_scalar((-230.25850929940458), AdValue::div(AdValue::neg(scratch.ad_value(497)), scratch.ad_value(1381))), AdValue::offset(AdValue::scale(AdValue::mul(AdValue::sub_from_scalar((-230.25850929940458), AdValue::div(AdValue::neg(scratch.ad_value(497)), scratch.ad_value(1381))), AdValue::offset(AdValue::scale(AdValue::sub_from_scalar((-230.25850929940458), AdValue::div(AdValue::neg(scratch.ad_value(497)), scratch.ad_value(1381))), 0.3333333333333333), 1.0)), 0.5), 1.0)), 1.0);
            scratch.store_ad(1356, &AdValue::div_from_scalar(1e-100, assign18850_ad_e19384));
        }

        if ((((((scratch.values[1320] != 0.0) && (scratch.values[1337] != 0.0)) && (!(scratch.values[1512] != 0.0))) && (!(scratch.values[1523] != 0.0))) && (!(scratch.values[1525] != 0.0))) && (!(scratch.values[1526] != 0.0))) {
            let assign18860_ad_e19434: AdValue = AdValue::scale(AdValue::offset(AdValue::mul(AdValue::offset(AdValue::div(AdValue::neg(scratch.ad_value(497)), scratch.ad_value(1381)), (-230.25850929940458)), AdValue::offset(AdValue::scale(AdValue::mul(AdValue::offset(AdValue::div(AdValue::neg(scratch.ad_value(497)), scratch.ad_value(1381)), (-230.25850929940458)), AdValue::offset(AdValue::scale(AdValue::offset(AdValue::div(AdValue::neg(scratch.ad_value(497)), scratch.ad_value(1381)), (-230.25850929940458)), 0.3333333333333333), 1.0)), 0.5), 1.0)), 1.0), 1e100);
            scratch.store_ad(1356, &assign18860_ad_e19434);
        }

        if ((((scratch.values[1320] != 0.0) && (scratch.values[1337] != 0.0)) && (!(scratch.values[1512] != 0.0))) && (!(scratch.values[1523] != 0.0))) {
            scratch.store_ad(1380, &AdValue::scale(AdValue::mul(AdValue::mul(AdValue::mul(scratch.ad_value(525), scratch.ad_value(1381)), scratch.ad_value(1381)), scratch.ad_value(1356)), scratch.values[402]));
        }

        scratch.values[1527] = if (scratch.values[411] > 1000.0) { 1.0 } else { 0.0 };

        if ((((scratch.values[1320] != 0.0) && (scratch.values[1337] != 0.0)) && (!(scratch.values[1512] != 0.0))) && (scratch.values[1527] != 0.0)) {
            scratch.values[1382] = 1.0;
            scratch.node_derivatives[1382] = [0.0; Instance::NODE_COUNT];
            scratch.branch_derivatives[1382] = [0.0; Instance::BRANCH_COUNT];
        }

        scratch.values[1528] = if (scratch.values[1355] > ((-scratch.values[500]) * scratch.values[411])) { 1.0 } else { 0.0 };

        scratch.values[1529] = if (scratch.values[414] == 4.0) { 1.0 } else { 0.0 };

        if ((((((scratch.values[1320] != 0.0) && (scratch.values[1337] != 0.0)) && (!(scratch.values[1512] != 0.0))) && (!(scratch.values[1527] != 0.0))) && (scratch.values[1528] != 0.0)) && (scratch.values[1529] != 0.0)) {
            scratch.store_ad(1356, &AdValue::mul(AdValue::mul(AdValue::mul(AdValue::scale(scratch.ad_value(1355), scratch.values[504]), AdValue::scale(scratch.ad_value(1355), scratch.values[504])), AdValue::scale(scratch.ad_value(1355), scratch.values[504])), AdValue::scale(scratch.ad_value(1355), scratch.values[504])));
        }

    }

    pub(super) fn stamp_transient_block_15(
        &mut self,
        ctx: &GeneratedEvalContext<'_>,
        stamper: &mut GeneratedStamper<'_>,
        scratch: &mut Scratch,
    ) {
        let _ = stamper;
        if ((((((scratch.values[1320] != 0.0) && (scratch.values[1337] != 0.0)) && (!(scratch.values[1512] != 0.0))) && (!(scratch.values[1527] != 0.0))) && (scratch.values[1528] != 0.0)) && (!(scratch.values[1529] != 0.0))) {
            scratch.store_ad(1356, &AdValue::powf(AdValue::abs(AdValue::scale(scratch.ad_value(1355), scratch.values[504])), scratch.values[414]));
        }

        if (((((scratch.values[1320] != 0.0) && (scratch.values[1337] != 0.0)) && (!(scratch.values[1512] != 0.0))) && (!(scratch.values[1527] != 0.0))) && (scratch.values[1528] != 0.0)) {
            scratch.store_ad(1382, &AdValue::div_from_scalar(1.0, AdValue::sub_from_scalar(1.0, scratch.ad_value(1356))));
        }

        if (((((scratch.values[1320] != 0.0) && (scratch.values[1337] != 0.0)) && (!(scratch.values[1512] != 0.0))) && (!(scratch.values[1527] != 0.0))) && (!(scratch.values[1528] != 0.0))) {
            scratch.store_ad(1382, &AdValue::offset(AdValue::scale(AdValue::offset(scratch.ad_value(1355), (scratch.values[500] * scratch.values[411])), scratch.values[507]), scratch.values[501]));
        }

        if (((scratch.values[1320] != 0.0) && (scratch.values[1337] != 0.0)) && (!(scratch.values[1512] != 0.0))) {
            scratch.store_ad(1383, &AdValue::mul(AdValue::add(AdValue::add(AdValue::add(scratch.ad_value(1357), scratch.ad_value(1358)), scratch.ad_value(1365)), scratch.ad_value(1380)), scratch.ad_value(1382)));
        }

        scratch.values[1530] = if (scratch.values[698] == 0.0) { 1.0 } else { 0.0 };

        if (((scratch.values[1320] != 0.0) && (scratch.values[1337] != 0.0)) && (scratch.values[1530] != 0.0)) {
            scratch.values[1384] = 0.0;
            scratch.node_derivatives[1384] = [0.0; Instance::NODE_COUNT];
            scratch.branch_derivatives[1384] = [0.0; Instance::BRANCH_COUNT];
        }

        scratch.values[1531] = if (scratch.values[465] == 0.5) { 1.0 } else { 0.0 };

        if ((((scratch.values[1320] != 0.0) && (scratch.values[1337] != 0.0)) && (!(scratch.values[1530] != 0.0))) && (scratch.values[1531] != 0.0)) {
            scratch.store_ad(1356, &AdValue::sqrt(AdValue::sub_from_scalar(1.0, AdValue::scale(scratch.ad_value(1348), scratch.values[462]))));
        }

        if ((((scratch.values[1320] != 0.0) && (scratch.values[1337] != 0.0)) && (!(scratch.values[1530] != 0.0))) && (!(scratch.values[1531] != 0.0))) {
            scratch.store_ad(1356, &AdValue::powf(AdValue::sub_from_scalar(1.0, AdValue::scale(scratch.ad_value(1348), scratch.values[462])), scratch.values[465]));
        }

        if (((scratch.values[1320] != 0.0) && (scratch.values[1337] != 0.0)) && (!(scratch.values[1530] != 0.0))) {
            scratch.store_ad(1357, &AdValue::scale(scratch.ad_value(1347), scratch.values[444]));
        }

        scratch.values[1532] = if ((scratch.values[394] == 0.0) && (scratch.values[397] == 0.0)) { 1.0 } else { 0.0 };

        if ((((scratch.values[1320] != 0.0) && (scratch.values[1337] != 0.0)) && (!(scratch.values[1530] != 0.0))) && (scratch.values[1532] != 0.0)) {
            scratch.values[1358] = 0.0;
            scratch.node_derivatives[1358] = [0.0; Instance::NODE_COUNT];
            scratch.branch_derivatives[1358] = [0.0; Instance::BRANCH_COUNT];
        }

        if ((((scratch.values[1320] != 0.0) && (scratch.values[1337] != 0.0)) && (!(scratch.values[1530] != 0.0))) && (!(scratch.values[1532] != 0.0))) {
            scratch.store_ad(1359, &AdValue::sub_from_scalar(scratch.values[450], scratch.ad_value(1353)));
        }

        if ((((scratch.values[1320] != 0.0) && (scratch.values[1337] != 0.0)) && (!(scratch.values[1530] != 0.0))) && (!(scratch.values[1532] != 0.0))) {
            scratch.store_ad(1360, &AdValue::sub_from_scalar(1.0, AdValue::sqrt(AdValue::sub_from_scalar(1.0, AdValue::div(scratch.ad_value(1351), scratch.ad_value(1359))))));
        }

        scratch.values[1533] = if (scratch.values[383] == 0.5) { 1.0 } else { 0.0 };

        if (((((scratch.values[1320] != 0.0) && (scratch.values[1337] != 0.0)) && (!(scratch.values[1530] != 0.0))) && (!(scratch.values[1532] != 0.0))) && (scratch.values[1533] != 0.0)) {
            scratch.values[1361] = 0.0;
            scratch.node_derivatives[1361] = [0.0; Instance::NODE_COUNT];
            scratch.branch_derivatives[1361] = [0.0; Instance::BRANCH_COUNT];
        }

        if (((((scratch.values[1320] != 0.0) && (scratch.values[1337] != 0.0)) && (!(scratch.values[1530] != 0.0))) && (!(scratch.values[1532] != 0.0))) && (!(scratch.values[1533] != 0.0))) {
            scratch.store_ad(1361, &AdValue::scale(AdValue::add(AdValue::div(AdValue::mul(AdValue::square(scratch.ad_value(1360)), AdValue::ln(scratch.ad_value(1360))), AdValue::sub_from_scalar(1.0, scratch.ad_value(1360))), scratch.ad_value(1360)), (1.0 - (2.0 * scratch.values[383]))));
        }

        if ((((scratch.values[1320] != 0.0) && (scratch.values[1337] != 0.0)) && (!(scratch.values[1530] != 0.0))) && (!(scratch.values[1532] != 0.0))) {
            scratch.store_ad(1362, &AdValue::add(scratch.ad_value(1360), scratch.ad_value(1361)));
        }

        scratch.values[1534] = if (scratch.values[383] == 0.5) { 1.0 } else { 0.0 };

        if (((((scratch.values[1320] != 0.0) && (scratch.values[1337] != 0.0)) && (!(scratch.values[1530] != 0.0))) && (!(scratch.values[1532] != 0.0))) && (scratch.values[1534] != 0.0)) {
            scratch.store_ad(1356, &AdValue::sqrt(AdValue::scale(scratch.ad_value(1359), scratch.values[486])));
        }

        if (((((scratch.values[1320] != 0.0) && (scratch.values[1337] != 0.0)) && (!(scratch.values[1530] != 0.0))) && (!(scratch.values[1532] != 0.0))) && (!(scratch.values[1534] != 0.0))) {
            scratch.store_ad(1356, &AdValue::powf(AdValue::scale(scratch.ad_value(1359), scratch.values[486]), scratch.values[383]));
        }

        if ((((scratch.values[1320] != 0.0) && (scratch.values[1337] != 0.0)) && (!(scratch.values[1530] != 0.0))) && (!(scratch.values[1532] != 0.0))) {
            scratch.store_ad(1363, &AdValue::scale(scratch.ad_value(1356), scratch.values[480]));
        }

        if ((((scratch.values[1320] != 0.0) && (scratch.values[1337] != 0.0)) && (!(scratch.values[1530] != 0.0))) && (!(scratch.values[1532] != 0.0))) {
            scratch.store_ad(1364, &AdValue::scale(AdValue::mul(AdValue::offset(scratch.ad_value(1350), (-1.0)), scratch.ad_value(1363)), scratch.values[441]));
        }

        if ((((scratch.values[1320] != 0.0) && (scratch.values[1337] != 0.0)) && (!(scratch.values[1530] != 0.0))) && (!(scratch.values[1532] != 0.0))) {
            scratch.store_ad(1358, &AdValue::scale(AdValue::mul(scratch.ad_value(1364), scratch.ad_value(1362)), scratch.values[394]));
        }

        scratch.values[1535] = if (scratch.values[397] == 0.0) { 1.0 } else { 0.0 };

        if ((((scratch.values[1320] != 0.0) && (scratch.values[1337] != 0.0)) && (!(scratch.values[1530] != 0.0))) && (scratch.values[1535] != 0.0)) {
            scratch.values[1365] = 0.0;
            scratch.node_derivatives[1365] = [0.0; Instance::NODE_COUNT];
            scratch.branch_derivatives[1365] = [0.0; Instance::BRANCH_COUNT];
        }

        if ((((scratch.values[1320] != 0.0) && (scratch.values[1337] != 0.0)) && (!(scratch.values[1530] != 0.0))) && (!(scratch.values[1535] != 0.0))) {
            scratch.store_ad(1366, &AdValue::scale(AdValue::div(AdValue::scale(scratch.ad_value(1363), scratch.values[465]), scratch.ad_value(1359)), scratch.values[495]));
        }

        if ((((scratch.values[1320] != 0.0) && (scratch.values[1337] != 0.0)) && (!(scratch.values[1530] != 0.0))) && (!(scratch.values[1535] != 0.0))) {
            scratch.store_ad(1367, &AdValue::div_from_scalar((0.666666666666667 * scratch.values[492]), scratch.ad_value(1366)));
        }

        if ((((scratch.values[1320] != 0.0) && (scratch.values[1337] != 0.0)) && (!(scratch.values[1530] != 0.0))) && (!(scratch.values[1535] != 0.0))) {
            scratch.store_ad(1368, &AdValue::square(scratch.ad_value(1367)));
        }

        if ((((scratch.values[1320] != 0.0) && (scratch.values[1337] != 0.0)) && (!(scratch.values[1530] != 0.0))) && (!(scratch.values[1535] != 0.0))) {
            scratch.store_ad(1369, &AdValue::sqrt(AdValue::div(AdValue::square(scratch.ad_value(1368)), AdValue::offset(AdValue::square(scratch.ad_value(1368)), 1.0))));
        }

        if ((((scratch.values[1320] != 0.0) && (scratch.values[1337] != 0.0)) && (!(scratch.values[1530] != 0.0))) && (!(scratch.values[1535] != 0.0))) {
            scratch.store_ad(1370, &AdValue::sqrt(AdValue::abs(scratch.ad_value(1369))));
        }

        if ((((scratch.values[1320] != 0.0) && (scratch.values[1337] != 0.0)) && (!(scratch.values[1530] != 0.0))) && (!(scratch.values[1535] != 0.0))) {
            scratch.store_ad(1371, &AdValue::mul(scratch.ad_value(1369), scratch.ad_value(1370)));
        }

        scratch.values[1536] = if (((-scratch.values[383]) * scratch.values[468]) == (-1.0)) { 1.0 } else { 0.0 };

        if (((((scratch.values[1320] != 0.0) && (scratch.values[1337] != 0.0)) && (!(scratch.values[1530] != 0.0))) && (!(scratch.values[1535] != 0.0))) && (scratch.values[1536] != 0.0)) {
            scratch.store_ad(1372, &AdValue::div_from_scalar(1.0, AdValue::offset(AdValue::mul(scratch.ad_value(1366), scratch.ad_value(1371)), 1.0)));
        }

        if (((((scratch.values[1320] != 0.0) && (scratch.values[1337] != 0.0)) && (!(scratch.values[1530] != 0.0))) && (!(scratch.values[1535] != 0.0))) && (!(scratch.values[1536] != 0.0))) {
            scratch.store_ad(1372, &AdValue::powf(AdValue::offset(AdValue::mul(scratch.ad_value(1366), scratch.ad_value(1371)), 1.0), ((-scratch.values[383]) * scratch.values[468])));
        }

        if ((((scratch.values[1320] != 0.0) && (scratch.values[1337] != 0.0)) && (!(scratch.values[1530] != 0.0))) && (!(scratch.values[1535] != 0.0))) {
            scratch.store_ad(1373, &AdValue::div(AdValue::mul(scratch.ad_value(1362), scratch.ad_value(1372)), AdValue::add(scratch.ad_value(1362), scratch.ad_value(1372))));
        }

        if ((((scratch.values[1320] != 0.0) && (scratch.values[1337] != 0.0)) && (!(scratch.values[1530] != 0.0))) && (!(scratch.values[1535] != 0.0))) {
            scratch.store_ad(1374, &AdValue::sqrt(AdValue::scale(AdValue::div(scratch.ad_value(1366), scratch.ad_value(1370)), 0.375)));
        }

        if ((((scratch.values[1320] != 0.0) && (scratch.values[1337] != 0.0)) && (!(scratch.values[1530] != 0.0))) && (!(scratch.values[1535] != 0.0))) {
            scratch.store_ad(1375, &AdValue::sub(AdValue::scale(AdValue::mul(scratch.ad_value(1367), scratch.ad_value(1370)), 2.0), scratch.ad_value(1369)));
        }

        if ((((scratch.values[1320] != 0.0) && (scratch.values[1337] != 0.0)) && (!(scratch.values[1530] != 0.0))) && (!(scratch.values[1535] != 0.0))) {
            scratch.store_ad(1376, &AdValue::add(AdValue::sub(AdValue::mul(AdValue::scale(scratch.ad_value(1367), scratch.values[492]), scratch.ad_value(1370)), AdValue::scale(scratch.ad_value(1369), scratch.values[492])), AdValue::scale(AdValue::mul(scratch.ad_value(1366), scratch.ad_value(1371)), 0.5)));
        }

        if ((((scratch.values[1320] != 0.0) && (scratch.values[1337] != 0.0)) && (!(scratch.values[1530] != 0.0))) && (!(scratch.values[1535] != 0.0))) {
            scratch.store_ad(1377, &AdValue::mul(AdValue::offset(scratch.ad_value(1375), (-1.0)), scratch.ad_value(1374)));
        }

        if ((((scratch.values[1320] != 0.0) && (scratch.values[1337] != 0.0)) && (!(scratch.values[1530] != 0.0))) && (!(scratch.values[1535] != 0.0))) {
            scratch.store_ad(1338, &AdValue::square(scratch.ad_value(1377)));
        }

        scratch.values[1537] = if (scratch.values[1377] > 0.0) { 1.0 } else { 0.0 };

        if (((((scratch.values[1320] != 0.0) && (scratch.values[1337] != 0.0)) && (!(scratch.values[1530] != 0.0))) && (!(scratch.values[1535] != 0.0))) && (scratch.values[1537] != 0.0)) {
            scratch.store_ad(1339, &AdValue::div_from_scalar(1.0, AdValue::offset(AdValue::scale(scratch.ad_value(1377), scratch.values[428]), 1.0)));
        }

        if (((((scratch.values[1320] != 0.0) && (scratch.values[1337] != 0.0)) && (!(scratch.values[1530] != 0.0))) && (!(scratch.values[1535] != 0.0))) && (!(scratch.values[1537] != 0.0))) {
            scratch.store_ad(1339, &AdValue::div_from_scalar(1.0, AdValue::sub_from_scalar(1.0, AdValue::scale(scratch.ad_value(1377), scratch.values[428]))));
        }

        scratch.values[1538] = if (((-scratch.values[1338]) + scratch.values[1376]) > (-230.25850929940458)) { 1.0 } else { 0.0 };

        if (((((scratch.values[1320] != 0.0) && (scratch.values[1337] != 0.0)) && (!(scratch.values[1530] != 0.0))) && (!(scratch.values[1535] != 0.0))) && (scratch.values[1538] != 0.0)) {
            scratch.store_ad(1356, &AdValue::exp(AdValue::sub(scratch.ad_value(1376), scratch.ad_value(1338))));
        }

        if (((((scratch.values[1320] != 0.0) && (scratch.values[1337] != 0.0)) && (!(scratch.values[1530] != 0.0))) && (!(scratch.values[1535] != 0.0))) && (!(scratch.values[1538] != 0.0))) {
            let assign19410_ad_e20263: AdValue = AdValue::div_from_scalar(1e-100, AdValue::offset(AdValue::mul(AdValue::sub_from_scalar((-230.25850929940458), AdValue::sub(scratch.ad_value(1376), scratch.ad_value(1338))), AdValue::offset(AdValue::scale(AdValue::mul(AdValue::sub_from_scalar((-230.25850929940458), AdValue::sub(scratch.ad_value(1376), scratch.ad_value(1338))), AdValue::offset(AdValue::scale(AdValue::sub_from_scalar((-230.25850929940458), AdValue::sub(scratch.ad_value(1376), scratch.ad_value(1338))), 0.3333333333333333), 1.0)), 0.5), 1.0)), 1.0));
            scratch.store_ad(1356, &assign19410_ad_e20263);
        }

        if ((((scratch.values[1320] != 0.0) && (scratch.values[1337] != 0.0)) && (!(scratch.values[1530] != 0.0))) && (!(scratch.values[1535] != 0.0))) {
            scratch.store_ad(1340, &AdValue::mul(AdValue::add(AdValue::add(AdValue::scale(scratch.ad_value(1339), 0.29214664), AdValue::scale(AdValue::square(scratch.ad_value(1339)), scratch.values[429])), AdValue::scale(AdValue::mul(AdValue::square(scratch.ad_value(1339)), scratch.ad_value(1339)), scratch.values[430])), scratch.ad_value(1356)));
        }

        scratch.values[1539] = if (scratch.values[1377] > 0.0) { 1.0 } else { 0.0 };

        if (((((scratch.values[1320] != 0.0) && (scratch.values[1337] != 0.0)) && (!(scratch.values[1530] != 0.0))) && (!(scratch.values[1535] != 0.0))) && (scratch.values[1539] != 0.0)) {
            scratch.values[1378] = scratch.values[1340];
            scratch.node_derivatives[1378] = scratch.node_derivatives[1340];
            scratch.branch_derivatives[1378] = scratch.branch_derivatives[1340];
        }

        scratch.values[1540] = if (scratch.values[1376] > (-230.25850929940458)) { 1.0 } else { 0.0 };

        if ((((((scratch.values[1320] != 0.0) && (scratch.values[1337] != 0.0)) && (!(scratch.values[1530] != 0.0))) && (!(scratch.values[1535] != 0.0))) && (!(scratch.values[1539] != 0.0))) && (scratch.values[1540] != 0.0)) {
            scratch.store_ad(1356, &AdValue::exp(scratch.ad_value(1376)));
        }

        if ((((((scratch.values[1320] != 0.0) && (scratch.values[1337] != 0.0)) && (!(scratch.values[1530] != 0.0))) && (!(scratch.values[1535] != 0.0))) && (!(scratch.values[1539] != 0.0))) && (!(scratch.values[1540] != 0.0))) {
            scratch.store_ad(1356, &AdValue::div_from_scalar(1e-100, AdValue::offset(AdValue::mul(AdValue::sub_from_scalar((-230.25850929940458), scratch.ad_value(1376)), AdValue::offset(AdValue::scale(AdValue::mul(AdValue::sub_from_scalar((-230.25850929940458), scratch.ad_value(1376)), AdValue::offset(AdValue::scale(AdValue::sub_from_scalar((-230.25850929940458), scratch.ad_value(1376)), 0.3333333333333333), 1.0)), 0.5), 1.0)), 1.0)));
        }

        if (((((scratch.values[1320] != 0.0) && (scratch.values[1337] != 0.0)) && (!(scratch.values[1530] != 0.0))) && (!(scratch.values[1535] != 0.0))) && (!(scratch.values[1539] != 0.0))) {
            scratch.store_ad(1378, &AdValue::sub(AdValue::scale(scratch.ad_value(1356), 2.0), scratch.ad_value(1340)));
        }

        if ((((scratch.values[1320] != 0.0) && (scratch.values[1337] != 0.0)) && (!(scratch.values[1530] != 0.0))) && (!(scratch.values[1535] != 0.0))) {
            scratch.store_ad(1379, &AdValue::scale(AdValue::div(AdValue::scale(scratch.ad_value(1378), scratch.values[492]), scratch.ad_value(1374)), (1.772453850905516 * 0.5)));
        }

        if ((((scratch.values[1320] != 0.0) && (scratch.values[1337] != 0.0)) && (!(scratch.values[1530] != 0.0))) && (!(scratch.values[1535] != 0.0))) {
            scratch.store_ad(1365, &AdValue::scale(AdValue::mul(AdValue::mul(scratch.ad_value(1364), scratch.ad_value(1379)), scratch.ad_value(1373)), scratch.values[397]));
        }

        scratch.values[1541] = if (scratch.values[403] == 0.0) { 1.0 } else { 0.0 };

        if ((((scratch.values[1320] != 0.0) && (scratch.values[1337] != 0.0)) && (!(scratch.values[1530] != 0.0))) && (scratch.values[1541] != 0.0)) {
            scratch.values[1380] = 0.0;
            scratch.node_derivatives[1380] = [0.0; Instance::NODE_COUNT];
            scratch.branch_derivatives[1380] = [0.0; Instance::BRANCH_COUNT];
        }

        scratch.values[1542] = if (scratch.values[383] == 0.5) { 1.0 } else { 0.0 };

        if (((((scratch.values[1320] != 0.0) && (scratch.values[1337] != 0.0)) && (!(scratch.values[1530] != 0.0))) && (!(scratch.values[1541] != 0.0))) && (scratch.values[1542] != 0.0)) {
            scratch.store_ad(1356, &AdValue::sqrt(AdValue::scale(AdValue::sub_from_scalar(scratch.values[380], scratch.ad_value(1354)), scratch.values[486])));
        }

        if (((((scratch.values[1320] != 0.0) && (scratch.values[1337] != 0.0)) && (!(scratch.values[1530] != 0.0))) && (!(scratch.values[1541] != 0.0))) && (!(scratch.values[1542] != 0.0))) {
            scratch.store_ad(1356, &AdValue::powf(AdValue::scale(AdValue::sub_from_scalar(scratch.values[380], scratch.ad_value(1354)), scratch.values[486]), scratch.values[383]));
        }

        if ((((scratch.values[1320] != 0.0) && (scratch.values[1337] != 0.0)) && (!(scratch.values[1530] != 0.0))) && (!(scratch.values[1541] != 0.0))) {
            scratch.store_ad(1381, &AdValue::scale(AdValue::div(AdValue::scale(AdValue::sub_from_scalar(scratch.values[380], scratch.ad_value(1354)), scratch.values[483]), scratch.ad_value(1356)), scratch.values[468]));
        }

        scratch.values[1543] = if (((((-scratch.values[498]) / scratch.values[1381])) as f64).abs() < 230.25850929940458) { 1.0 } else { 0.0 };

        if (((((scratch.values[1320] != 0.0) && (scratch.values[1337] != 0.0)) && (!(scratch.values[1530] != 0.0))) && (!(scratch.values[1541] != 0.0))) && (scratch.values[1543] != 0.0)) {
            scratch.store_ad(1356, &AdValue::exp(AdValue::div(AdValue::neg(scratch.ad_value(498)), scratch.ad_value(1381))));
        }

        scratch.values[1544] = if (((-scratch.values[498]) / scratch.values[1381]) < 0.0) { 1.0 } else { 0.0 };

        if ((((((scratch.values[1320] != 0.0) && (scratch.values[1337] != 0.0)) && (!(scratch.values[1530] != 0.0))) && (!(scratch.values[1541] != 0.0))) && (!(scratch.values[1543] != 0.0))) && (scratch.values[1544] != 0.0)) {
            let assign19600_ad_e20590: AdValue = AdValue::offset(AdValue::mul(AdValue::sub_from_scalar((-230.25850929940458), AdValue::div(AdValue::neg(scratch.ad_value(498)), scratch.ad_value(1381))), AdValue::offset(AdValue::scale(AdValue::mul(AdValue::sub_from_scalar((-230.25850929940458), AdValue::div(AdValue::neg(scratch.ad_value(498)), scratch.ad_value(1381))), AdValue::offset(AdValue::scale(AdValue::sub_from_scalar((-230.25850929940458), AdValue::div(AdValue::neg(scratch.ad_value(498)), scratch.ad_value(1381))), 0.3333333333333333), 1.0)), 0.5), 1.0)), 1.0);
            scratch.store_ad(1356, &AdValue::div_from_scalar(1e-100, assign19600_ad_e20590));
        }

        if ((((((scratch.values[1320] != 0.0) && (scratch.values[1337] != 0.0)) && (!(scratch.values[1530] != 0.0))) && (!(scratch.values[1541] != 0.0))) && (!(scratch.values[1543] != 0.0))) && (!(scratch.values[1544] != 0.0))) {
            let assign19610_ad_e20640: AdValue = AdValue::scale(AdValue::offset(AdValue::mul(AdValue::offset(AdValue::div(AdValue::neg(scratch.ad_value(498)), scratch.ad_value(1381)), (-230.25850929940458)), AdValue::offset(AdValue::scale(AdValue::mul(AdValue::offset(AdValue::div(AdValue::neg(scratch.ad_value(498)), scratch.ad_value(1381)), (-230.25850929940458)), AdValue::offset(AdValue::scale(AdValue::offset(AdValue::div(AdValue::neg(scratch.ad_value(498)), scratch.ad_value(1381)), (-230.25850929940458)), 0.3333333333333333), 1.0)), 0.5), 1.0)), 1.0), 1e100);
            scratch.store_ad(1356, &assign19610_ad_e20640);
        }

        if ((((scratch.values[1320] != 0.0) && (scratch.values[1337] != 0.0)) && (!(scratch.values[1530] != 0.0))) && (!(scratch.values[1541] != 0.0))) {
            scratch.store_ad(1380, &AdValue::scale(AdValue::mul(AdValue::mul(AdValue::mul(scratch.ad_value(525), scratch.ad_value(1381)), scratch.ad_value(1381)), scratch.ad_value(1356)), scratch.values[403]));
        }

        scratch.values[1545] = if (scratch.values[412] > 1000.0) { 1.0 } else { 0.0 };

        if ((((scratch.values[1320] != 0.0) && (scratch.values[1337] != 0.0)) && (!(scratch.values[1530] != 0.0))) && (scratch.values[1545] != 0.0)) {
            scratch.values[1382] = 1.0;
            scratch.node_derivatives[1382] = [0.0; Instance::NODE_COUNT];
            scratch.branch_derivatives[1382] = [0.0; Instance::BRANCH_COUNT];
        }

        scratch.values[1546] = if (scratch.values[1355] > ((-scratch.values[500]) * scratch.values[412])) { 1.0 } else { 0.0 };

        scratch.values[1547] = if (scratch.values[415] == 4.0) { 1.0 } else { 0.0 };

        if ((((((scratch.values[1320] != 0.0) && (scratch.values[1337] != 0.0)) && (!(scratch.values[1530] != 0.0))) && (!(scratch.values[1545] != 0.0))) && (scratch.values[1546] != 0.0)) && (scratch.values[1547] != 0.0)) {
            scratch.store_ad(1356, &AdValue::mul(AdValue::mul(AdValue::mul(AdValue::scale(scratch.ad_value(1355), scratch.values[505]), AdValue::scale(scratch.ad_value(1355), scratch.values[505])), AdValue::scale(scratch.ad_value(1355), scratch.values[505])), AdValue::scale(scratch.ad_value(1355), scratch.values[505])));
        }

        if ((((((scratch.values[1320] != 0.0) && (scratch.values[1337] != 0.0)) && (!(scratch.values[1530] != 0.0))) && (!(scratch.values[1545] != 0.0))) && (scratch.values[1546] != 0.0)) && (!(scratch.values[1547] != 0.0))) {
            scratch.store_ad(1356, &AdValue::powf(AdValue::abs(AdValue::scale(scratch.ad_value(1355), scratch.values[505])), scratch.values[415]));
        }

        if (((((scratch.values[1320] != 0.0) && (scratch.values[1337] != 0.0)) && (!(scratch.values[1530] != 0.0))) && (!(scratch.values[1545] != 0.0))) && (scratch.values[1546] != 0.0)) {
            scratch.store_ad(1382, &AdValue::div_from_scalar(1.0, AdValue::sub_from_scalar(1.0, scratch.ad_value(1356))));
        }

        if (((((scratch.values[1320] != 0.0) && (scratch.values[1337] != 0.0)) && (!(scratch.values[1530] != 0.0))) && (!(scratch.values[1545] != 0.0))) && (!(scratch.values[1546] != 0.0))) {
            scratch.store_ad(1382, &AdValue::offset(AdValue::scale(AdValue::offset(scratch.ad_value(1355), (scratch.values[500] * scratch.values[412])), scratch.values[508]), scratch.values[502]));
        }

        if (((scratch.values[1320] != 0.0) && (scratch.values[1337] != 0.0)) && (!(scratch.values[1530] != 0.0))) {
            scratch.store_ad(1384, &AdValue::mul(AdValue::add(AdValue::add(AdValue::add(scratch.ad_value(1357), scratch.ad_value(1358)), scratch.ad_value(1365)), scratch.ad_value(1380)), scratch.ad_value(1382)));
        }

        scratch.values[1548] = if (scratch.values[699] == 0.0) { 1.0 } else { 0.0 };

        if (((scratch.values[1320] != 0.0) && (scratch.values[1337] != 0.0)) && (scratch.values[1548] != 0.0)) {
            scratch.values[1385] = 0.0;
            scratch.node_derivatives[1385] = [0.0; Instance::NODE_COUNT];
            scratch.branch_derivatives[1385] = [0.0; Instance::BRANCH_COUNT];
        }

        scratch.values[1549] = if (scratch.values[466] == 0.5) { 1.0 } else { 0.0 };

        if ((((scratch.values[1320] != 0.0) && (scratch.values[1337] != 0.0)) && (!(scratch.values[1548] != 0.0))) && (scratch.values[1549] != 0.0)) {
            scratch.store_ad(1356, &AdValue::sqrt(AdValue::sub_from_scalar(1.0, AdValue::scale(scratch.ad_value(1348), scratch.values[463]))));
        }

        if ((((scratch.values[1320] != 0.0) && (scratch.values[1337] != 0.0)) && (!(scratch.values[1548] != 0.0))) && (!(scratch.values[1549] != 0.0))) {
            scratch.store_ad(1356, &AdValue::powf(AdValue::sub_from_scalar(1.0, AdValue::scale(scratch.ad_value(1348), scratch.values[463])), scratch.values[466]));
        }

        if (((scratch.values[1320] != 0.0) && (scratch.values[1337] != 0.0)) && (!(scratch.values[1548] != 0.0))) {
            scratch.store_ad(1357, &AdValue::scale(scratch.ad_value(1347), scratch.values[445]));
        }

        scratch.values[1550] = if ((scratch.values[395] == 0.0) && (scratch.values[398] == 0.0)) { 1.0 } else { 0.0 };

        if ((((scratch.values[1320] != 0.0) && (scratch.values[1337] != 0.0)) && (!(scratch.values[1548] != 0.0))) && (scratch.values[1550] != 0.0)) {
            scratch.values[1358] = 0.0;
            scratch.node_derivatives[1358] = [0.0; Instance::NODE_COUNT];
            scratch.branch_derivatives[1358] = [0.0; Instance::BRANCH_COUNT];
        }

        if ((((scratch.values[1320] != 0.0) && (scratch.values[1337] != 0.0)) && (!(scratch.values[1548] != 0.0))) && (!(scratch.values[1550] != 0.0))) {
            scratch.store_ad(1359, &AdValue::sub_from_scalar(scratch.values[451], scratch.ad_value(1353)));
        }

        if ((((scratch.values[1320] != 0.0) && (scratch.values[1337] != 0.0)) && (!(scratch.values[1548] != 0.0))) && (!(scratch.values[1550] != 0.0))) {
            scratch.store_ad(1360, &AdValue::sub_from_scalar(1.0, AdValue::sqrt(AdValue::sub_from_scalar(1.0, AdValue::div(scratch.ad_value(1351), scratch.ad_value(1359))))));
        }

        scratch.values[1551] = if (scratch.values[384] == 0.5) { 1.0 } else { 0.0 };

        if (((((scratch.values[1320] != 0.0) && (scratch.values[1337] != 0.0)) && (!(scratch.values[1548] != 0.0))) && (!(scratch.values[1550] != 0.0))) && (scratch.values[1551] != 0.0)) {
            scratch.values[1361] = 0.0;
            scratch.node_derivatives[1361] = [0.0; Instance::NODE_COUNT];
            scratch.branch_derivatives[1361] = [0.0; Instance::BRANCH_COUNT];
        }

        if (((((scratch.values[1320] != 0.0) && (scratch.values[1337] != 0.0)) && (!(scratch.values[1548] != 0.0))) && (!(scratch.values[1550] != 0.0))) && (!(scratch.values[1551] != 0.0))) {
            scratch.store_ad(1361, &AdValue::scale(AdValue::add(AdValue::div(AdValue::mul(AdValue::square(scratch.ad_value(1360)), AdValue::ln(scratch.ad_value(1360))), AdValue::sub_from_scalar(1.0, scratch.ad_value(1360))), scratch.ad_value(1360)), (1.0 - (2.0 * scratch.values[384]))));
        }

        if ((((scratch.values[1320] != 0.0) && (scratch.values[1337] != 0.0)) && (!(scratch.values[1548] != 0.0))) && (!(scratch.values[1550] != 0.0))) {
            scratch.store_ad(1362, &AdValue::add(scratch.ad_value(1360), scratch.ad_value(1361)));
        }

        scratch.values[1552] = if (scratch.values[384] == 0.5) { 1.0 } else { 0.0 };

        if (((((scratch.values[1320] != 0.0) && (scratch.values[1337] != 0.0)) && (!(scratch.values[1548] != 0.0))) && (!(scratch.values[1550] != 0.0))) && (scratch.values[1552] != 0.0)) {
            scratch.store_ad(1356, &AdValue::sqrt(AdValue::scale(scratch.ad_value(1359), scratch.values[487])));
        }

        if (((((scratch.values[1320] != 0.0) && (scratch.values[1337] != 0.0)) && (!(scratch.values[1548] != 0.0))) && (!(scratch.values[1550] != 0.0))) && (!(scratch.values[1552] != 0.0))) {
            scratch.store_ad(1356, &AdValue::powf(AdValue::scale(scratch.ad_value(1359), scratch.values[487]), scratch.values[384]));
        }

        if ((((scratch.values[1320] != 0.0) && (scratch.values[1337] != 0.0)) && (!(scratch.values[1548] != 0.0))) && (!(scratch.values[1550] != 0.0))) {
            scratch.store_ad(1363, &AdValue::scale(scratch.ad_value(1356), scratch.values[481]));
        }

        if ((((scratch.values[1320] != 0.0) && (scratch.values[1337] != 0.0)) && (!(scratch.values[1548] != 0.0))) && (!(scratch.values[1550] != 0.0))) {
            scratch.store_ad(1364, &AdValue::scale(AdValue::mul(AdValue::offset(scratch.ad_value(1350), (-1.0)), scratch.ad_value(1363)), scratch.values[442]));
        }

        if ((((scratch.values[1320] != 0.0) && (scratch.values[1337] != 0.0)) && (!(scratch.values[1548] != 0.0))) && (!(scratch.values[1550] != 0.0))) {
            scratch.store_ad(1358, &AdValue::scale(AdValue::mul(scratch.ad_value(1364), scratch.ad_value(1362)), scratch.values[395]));
        }

        scratch.values[1553] = if (scratch.values[398] == 0.0) { 1.0 } else { 0.0 };

        if ((((scratch.values[1320] != 0.0) && (scratch.values[1337] != 0.0)) && (!(scratch.values[1548] != 0.0))) && (scratch.values[1553] != 0.0)) {
            scratch.values[1365] = 0.0;
            scratch.node_derivatives[1365] = [0.0; Instance::NODE_COUNT];
            scratch.branch_derivatives[1365] = [0.0; Instance::BRANCH_COUNT];
        }

        if ((((scratch.values[1320] != 0.0) && (scratch.values[1337] != 0.0)) && (!(scratch.values[1548] != 0.0))) && (!(scratch.values[1553] != 0.0))) {
            scratch.store_ad(1366, &AdValue::scale(AdValue::div(AdValue::scale(scratch.ad_value(1363), scratch.values[466]), scratch.ad_value(1359)), scratch.values[496]));
        }

        if ((((scratch.values[1320] != 0.0) && (scratch.values[1337] != 0.0)) && (!(scratch.values[1548] != 0.0))) && (!(scratch.values[1553] != 0.0))) {
            scratch.store_ad(1367, &AdValue::div_from_scalar((0.666666666666667 * scratch.values[493]), scratch.ad_value(1366)));
        }

        if ((((scratch.values[1320] != 0.0) && (scratch.values[1337] != 0.0)) && (!(scratch.values[1548] != 0.0))) && (!(scratch.values[1553] != 0.0))) {
            scratch.store_ad(1368, &AdValue::square(scratch.ad_value(1367)));
        }

        if ((((scratch.values[1320] != 0.0) && (scratch.values[1337] != 0.0)) && (!(scratch.values[1548] != 0.0))) && (!(scratch.values[1553] != 0.0))) {
            scratch.store_ad(1369, &AdValue::sqrt(AdValue::div(AdValue::square(scratch.ad_value(1368)), AdValue::offset(AdValue::square(scratch.ad_value(1368)), 1.0))));
        }

        if ((((scratch.values[1320] != 0.0) && (scratch.values[1337] != 0.0)) && (!(scratch.values[1548] != 0.0))) && (!(scratch.values[1553] != 0.0))) {
            scratch.store_ad(1370, &AdValue::sqrt(AdValue::abs(scratch.ad_value(1369))));
        }

        if ((((scratch.values[1320] != 0.0) && (scratch.values[1337] != 0.0)) && (!(scratch.values[1548] != 0.0))) && (!(scratch.values[1553] != 0.0))) {
            scratch.store_ad(1371, &AdValue::mul(scratch.ad_value(1369), scratch.ad_value(1370)));
        }

        scratch.values[1554] = if (((-scratch.values[384]) * scratch.values[469]) == (-1.0)) { 1.0 } else { 0.0 };

        if (((((scratch.values[1320] != 0.0) && (scratch.values[1337] != 0.0)) && (!(scratch.values[1548] != 0.0))) && (!(scratch.values[1553] != 0.0))) && (scratch.values[1554] != 0.0)) {
            scratch.store_ad(1372, &AdValue::div_from_scalar(1.0, AdValue::offset(AdValue::mul(scratch.ad_value(1366), scratch.ad_value(1371)), 1.0)));
        }

        if (((((scratch.values[1320] != 0.0) && (scratch.values[1337] != 0.0)) && (!(scratch.values[1548] != 0.0))) && (!(scratch.values[1553] != 0.0))) && (!(scratch.values[1554] != 0.0))) {
            scratch.store_ad(1372, &AdValue::powf(AdValue::offset(AdValue::mul(scratch.ad_value(1366), scratch.ad_value(1371)), 1.0), ((-scratch.values[384]) * scratch.values[469])));
        }

        if ((((scratch.values[1320] != 0.0) && (scratch.values[1337] != 0.0)) && (!(scratch.values[1548] != 0.0))) && (!(scratch.values[1553] != 0.0))) {
            scratch.store_ad(1373, &AdValue::div(AdValue::mul(scratch.ad_value(1362), scratch.ad_value(1372)), AdValue::add(scratch.ad_value(1362), scratch.ad_value(1372))));
        }

        if ((((scratch.values[1320] != 0.0) && (scratch.values[1337] != 0.0)) && (!(scratch.values[1548] != 0.0))) && (!(scratch.values[1553] != 0.0))) {
            scratch.store_ad(1374, &AdValue::sqrt(AdValue::scale(AdValue::div(scratch.ad_value(1366), scratch.ad_value(1370)), 0.375)));
        }

        if ((((scratch.values[1320] != 0.0) && (scratch.values[1337] != 0.0)) && (!(scratch.values[1548] != 0.0))) && (!(scratch.values[1553] != 0.0))) {
            scratch.store_ad(1375, &AdValue::sub(AdValue::scale(AdValue::mul(scratch.ad_value(1367), scratch.ad_value(1370)), 2.0), scratch.ad_value(1369)));
        }

        if ((((scratch.values[1320] != 0.0) && (scratch.values[1337] != 0.0)) && (!(scratch.values[1548] != 0.0))) && (!(scratch.values[1553] != 0.0))) {
            scratch.store_ad(1376, &AdValue::add(AdValue::sub(AdValue::mul(AdValue::scale(scratch.ad_value(1367), scratch.values[493]), scratch.ad_value(1370)), AdValue::scale(scratch.ad_value(1369), scratch.values[493])), AdValue::scale(AdValue::mul(scratch.ad_value(1366), scratch.ad_value(1371)), 0.5)));
        }

        if ((((scratch.values[1320] != 0.0) && (scratch.values[1337] != 0.0)) && (!(scratch.values[1548] != 0.0))) && (!(scratch.values[1553] != 0.0))) {
            scratch.store_ad(1377, &AdValue::mul(AdValue::offset(scratch.ad_value(1375), (-1.0)), scratch.ad_value(1374)));
        }

        if ((((scratch.values[1320] != 0.0) && (scratch.values[1337] != 0.0)) && (!(scratch.values[1548] != 0.0))) && (!(scratch.values[1553] != 0.0))) {
            scratch.store_ad(1338, &AdValue::square(scratch.ad_value(1377)));
        }

        scratch.values[1555] = if (scratch.values[1377] > 0.0) { 1.0 } else { 0.0 };

        if (((((scratch.values[1320] != 0.0) && (scratch.values[1337] != 0.0)) && (!(scratch.values[1548] != 0.0))) && (!(scratch.values[1553] != 0.0))) && (scratch.values[1555] != 0.0)) {
            scratch.store_ad(1339, &AdValue::div_from_scalar(1.0, AdValue::offset(AdValue::scale(scratch.ad_value(1377), scratch.values[428]), 1.0)));
        }

        if (((((scratch.values[1320] != 0.0) && (scratch.values[1337] != 0.0)) && (!(scratch.values[1548] != 0.0))) && (!(scratch.values[1553] != 0.0))) && (!(scratch.values[1555] != 0.0))) {
            scratch.store_ad(1339, &AdValue::div_from_scalar(1.0, AdValue::sub_from_scalar(1.0, AdValue::scale(scratch.ad_value(1377), scratch.values[428]))));
        }

        scratch.values[1556] = if (((-scratch.values[1338]) + scratch.values[1376]) > (-230.25850929940458)) { 1.0 } else { 0.0 };

        if (((((scratch.values[1320] != 0.0) && (scratch.values[1337] != 0.0)) && (!(scratch.values[1548] != 0.0))) && (!(scratch.values[1553] != 0.0))) && (scratch.values[1556] != 0.0)) {
            scratch.store_ad(1356, &AdValue::exp(AdValue::sub(scratch.ad_value(1376), scratch.ad_value(1338))));
        }

        if (((((scratch.values[1320] != 0.0) && (scratch.values[1337] != 0.0)) && (!(scratch.values[1548] != 0.0))) && (!(scratch.values[1553] != 0.0))) && (!(scratch.values[1556] != 0.0))) {
            let assign20160_ad_e21469: AdValue = AdValue::div_from_scalar(1e-100, AdValue::offset(AdValue::mul(AdValue::sub_from_scalar((-230.25850929940458), AdValue::sub(scratch.ad_value(1376), scratch.ad_value(1338))), AdValue::offset(AdValue::scale(AdValue::mul(AdValue::sub_from_scalar((-230.25850929940458), AdValue::sub(scratch.ad_value(1376), scratch.ad_value(1338))), AdValue::offset(AdValue::scale(AdValue::sub_from_scalar((-230.25850929940458), AdValue::sub(scratch.ad_value(1376), scratch.ad_value(1338))), 0.3333333333333333), 1.0)), 0.5), 1.0)), 1.0));
            scratch.store_ad(1356, &assign20160_ad_e21469);
        }

        if ((((scratch.values[1320] != 0.0) && (scratch.values[1337] != 0.0)) && (!(scratch.values[1548] != 0.0))) && (!(scratch.values[1553] != 0.0))) {
            scratch.store_ad(1340, &AdValue::mul(AdValue::add(AdValue::add(AdValue::scale(scratch.ad_value(1339), 0.29214664), AdValue::scale(AdValue::square(scratch.ad_value(1339)), scratch.values[429])), AdValue::scale(AdValue::mul(AdValue::square(scratch.ad_value(1339)), scratch.ad_value(1339)), scratch.values[430])), scratch.ad_value(1356)));
        }

        scratch.values[1557] = if (scratch.values[1377] > 0.0) { 1.0 } else { 0.0 };

        if (((((scratch.values[1320] != 0.0) && (scratch.values[1337] != 0.0)) && (!(scratch.values[1548] != 0.0))) && (!(scratch.values[1553] != 0.0))) && (scratch.values[1557] != 0.0)) {
            scratch.values[1378] = scratch.values[1340];
            scratch.node_derivatives[1378] = scratch.node_derivatives[1340];
            scratch.branch_derivatives[1378] = scratch.branch_derivatives[1340];
        }

        scratch.values[1558] = if (scratch.values[1376] > (-230.25850929940458)) { 1.0 } else { 0.0 };

        if ((((((scratch.values[1320] != 0.0) && (scratch.values[1337] != 0.0)) && (!(scratch.values[1548] != 0.0))) && (!(scratch.values[1553] != 0.0))) && (!(scratch.values[1557] != 0.0))) && (scratch.values[1558] != 0.0)) {
            scratch.store_ad(1356, &AdValue::exp(scratch.ad_value(1376)));
        }

        if ((((((scratch.values[1320] != 0.0) && (scratch.values[1337] != 0.0)) && (!(scratch.values[1548] != 0.0))) && (!(scratch.values[1553] != 0.0))) && (!(scratch.values[1557] != 0.0))) && (!(scratch.values[1558] != 0.0))) {
            scratch.store_ad(1356, &AdValue::div_from_scalar(1e-100, AdValue::offset(AdValue::mul(AdValue::sub_from_scalar((-230.25850929940458), scratch.ad_value(1376)), AdValue::offset(AdValue::scale(AdValue::mul(AdValue::sub_from_scalar((-230.25850929940458), scratch.ad_value(1376)), AdValue::offset(AdValue::scale(AdValue::sub_from_scalar((-230.25850929940458), scratch.ad_value(1376)), 0.3333333333333333), 1.0)), 0.5), 1.0)), 1.0)));
        }

        if (((((scratch.values[1320] != 0.0) && (scratch.values[1337] != 0.0)) && (!(scratch.values[1548] != 0.0))) && (!(scratch.values[1553] != 0.0))) && (!(scratch.values[1557] != 0.0))) {
            scratch.store_ad(1378, &AdValue::sub(AdValue::scale(scratch.ad_value(1356), 2.0), scratch.ad_value(1340)));
        }

        if ((((scratch.values[1320] != 0.0) && (scratch.values[1337] != 0.0)) && (!(scratch.values[1548] != 0.0))) && (!(scratch.values[1553] != 0.0))) {
            scratch.store_ad(1379, &AdValue::scale(AdValue::div(AdValue::scale(scratch.ad_value(1378), scratch.values[493]), scratch.ad_value(1374)), (1.772453850905516 * 0.5)));
        }

        if ((((scratch.values[1320] != 0.0) && (scratch.values[1337] != 0.0)) && (!(scratch.values[1548] != 0.0))) && (!(scratch.values[1553] != 0.0))) {
            scratch.store_ad(1365, &AdValue::scale(AdValue::mul(AdValue::mul(scratch.ad_value(1364), scratch.ad_value(1379)), scratch.ad_value(1373)), scratch.values[398]));
        }

        scratch.values[1559] = if (scratch.values[404] == 0.0) { 1.0 } else { 0.0 };

        if ((((scratch.values[1320] != 0.0) && (scratch.values[1337] != 0.0)) && (!(scratch.values[1548] != 0.0))) && (scratch.values[1559] != 0.0)) {
            scratch.values[1380] = 0.0;
            scratch.node_derivatives[1380] = [0.0; Instance::NODE_COUNT];
            scratch.branch_derivatives[1380] = [0.0; Instance::BRANCH_COUNT];
        }

        scratch.values[1560] = if (scratch.values[384] == 0.5) { 1.0 } else { 0.0 };

        if (((((scratch.values[1320] != 0.0) && (scratch.values[1337] != 0.0)) && (!(scratch.values[1548] != 0.0))) && (!(scratch.values[1559] != 0.0))) && (scratch.values[1560] != 0.0)) {
            scratch.store_ad(1356, &AdValue::sqrt(AdValue::scale(AdValue::sub_from_scalar(scratch.values[381], scratch.ad_value(1354)), scratch.values[487])));
        }

        if (((((scratch.values[1320] != 0.0) && (scratch.values[1337] != 0.0)) && (!(scratch.values[1548] != 0.0))) && (!(scratch.values[1559] != 0.0))) && (!(scratch.values[1560] != 0.0))) {
            scratch.store_ad(1356, &AdValue::powf(AdValue::scale(AdValue::sub_from_scalar(scratch.values[381], scratch.ad_value(1354)), scratch.values[487]), scratch.values[384]));
        }

        if ((((scratch.values[1320] != 0.0) && (scratch.values[1337] != 0.0)) && (!(scratch.values[1548] != 0.0))) && (!(scratch.values[1559] != 0.0))) {
            scratch.store_ad(1381, &AdValue::scale(AdValue::div(AdValue::scale(AdValue::sub_from_scalar(scratch.values[381], scratch.ad_value(1354)), scratch.values[484]), scratch.ad_value(1356)), scratch.values[469]));
        }

        scratch.values[1561] = if (((((-scratch.values[499]) / scratch.values[1381])) as f64).abs() < 230.25850929940458) { 1.0 } else { 0.0 };

    }
}
