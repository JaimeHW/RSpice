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
        if ((((scratch.values[1323] != 0.0) && (scratch.values[1340] != 0.0)) && (!(scratch.values[1415] != 0.0))) && (scratch.values[1416] != 0.0)) {
            scratch.store_ad(1359, &AdValue::sqrt(AdValue::sub_from_scalar(1.0, AdValue::scale(scratch.ad_value(1351), scratch.values[455]))));
        }

        if ((((scratch.values[1323] != 0.0) && (scratch.values[1340] != 0.0)) && (!(scratch.values[1415] != 0.0))) && (!(scratch.values[1416] != 0.0))) {
            scratch.store_ad(1359, &AdValue::powf(AdValue::sub_from_scalar(1.0, AdValue::scale(scratch.ad_value(1351), scratch.values[455])), scratch.values[458]));
        }

        if (((scratch.values[1323] != 0.0) && (scratch.values[1340] != 0.0)) && (!(scratch.values[1415] != 0.0))) {
            scratch.store_ad(1360, &AdValue::scale(scratch.ad_value(1350), scratch.values[437]));
        }

        scratch.values[1417] = if ((scratch.values[387] == 0.0) && (scratch.values[390] == 0.0)) { 1.0 } else { 0.0 };

        if ((((scratch.values[1323] != 0.0) && (scratch.values[1340] != 0.0)) && (!(scratch.values[1415] != 0.0))) && (scratch.values[1417] != 0.0)) {
            scratch.values[1361] = 0.0;
            scratch.node_derivatives[1361] = [0.0; Instance::NODE_COUNT];
            scratch.branch_derivatives[1361] = [0.0; Instance::BRANCH_COUNT];
        }

        if ((((scratch.values[1323] != 0.0) && (scratch.values[1340] != 0.0)) && (!(scratch.values[1415] != 0.0))) && (!(scratch.values[1417] != 0.0))) {
            scratch.store_ad(1362, &AdValue::sub_from_scalar(scratch.values[443], scratch.ad_value(1356)));
        }

        if ((((scratch.values[1323] != 0.0) && (scratch.values[1340] != 0.0)) && (!(scratch.values[1415] != 0.0))) && (!(scratch.values[1417] != 0.0))) {
            scratch.store_ad(1363, &AdValue::sub_from_scalar(1.0, AdValue::sqrt(AdValue::sub_from_scalar(1.0, AdValue::div(scratch.ad_value(1354), scratch.ad_value(1362))))));
        }

        scratch.values[1418] = if (scratch.values[376] == 0.5) { 1.0 } else { 0.0 };

        if (((((scratch.values[1323] != 0.0) && (scratch.values[1340] != 0.0)) && (!(scratch.values[1415] != 0.0))) && (!(scratch.values[1417] != 0.0))) && (scratch.values[1418] != 0.0)) {
            scratch.values[1364] = 0.0;
            scratch.node_derivatives[1364] = [0.0; Instance::NODE_COUNT];
            scratch.branch_derivatives[1364] = [0.0; Instance::BRANCH_COUNT];
        }

        if (((((scratch.values[1323] != 0.0) && (scratch.values[1340] != 0.0)) && (!(scratch.values[1415] != 0.0))) && (!(scratch.values[1417] != 0.0))) && (!(scratch.values[1418] != 0.0))) {
            scratch.store_ad(1364, &AdValue::scale(AdValue::add(AdValue::div(AdValue::mul(AdValue::square(scratch.ad_value(1363)), AdValue::ln(scratch.ad_value(1363))), AdValue::sub_from_scalar(1.0, scratch.ad_value(1363))), scratch.ad_value(1363)), (1.0 - (2.0 * scratch.values[376]))));
        }

        if ((((scratch.values[1323] != 0.0) && (scratch.values[1340] != 0.0)) && (!(scratch.values[1415] != 0.0))) && (!(scratch.values[1417] != 0.0))) {
            scratch.store_ad(1365, &AdValue::add(scratch.ad_value(1363), scratch.ad_value(1364)));
        }

        scratch.values[1419] = if (scratch.values[376] == 0.5) { 1.0 } else { 0.0 };

        if (((((scratch.values[1323] != 0.0) && (scratch.values[1340] != 0.0)) && (!(scratch.values[1415] != 0.0))) && (!(scratch.values[1417] != 0.0))) && (scratch.values[1419] != 0.0)) {
            scratch.store_ad(1359, &AdValue::sqrt(AdValue::scale(scratch.ad_value(1362), scratch.values[479])));
        }

        if (((((scratch.values[1323] != 0.0) && (scratch.values[1340] != 0.0)) && (!(scratch.values[1415] != 0.0))) && (!(scratch.values[1417] != 0.0))) && (!(scratch.values[1419] != 0.0))) {
            scratch.store_ad(1359, &AdValue::powf(AdValue::scale(scratch.ad_value(1362), scratch.values[479]), scratch.values[376]));
        }

        if ((((scratch.values[1323] != 0.0) && (scratch.values[1340] != 0.0)) && (!(scratch.values[1415] != 0.0))) && (!(scratch.values[1417] != 0.0))) {
            scratch.store_ad(1366, &AdValue::scale(scratch.ad_value(1359), scratch.values[473]));
        }

        if ((((scratch.values[1323] != 0.0) && (scratch.values[1340] != 0.0)) && (!(scratch.values[1415] != 0.0))) && (!(scratch.values[1417] != 0.0))) {
            scratch.store_ad(1367, &AdValue::scale(AdValue::mul(AdValue::offset(scratch.ad_value(1353), (-1.0)), scratch.ad_value(1366)), scratch.values[434]));
        }

        if ((((scratch.values[1323] != 0.0) && (scratch.values[1340] != 0.0)) && (!(scratch.values[1415] != 0.0))) && (!(scratch.values[1417] != 0.0))) {
            scratch.store_ad(1361, &AdValue::scale(AdValue::mul(scratch.ad_value(1367), scratch.ad_value(1365)), scratch.values[387]));
        }

        scratch.values[1420] = if (scratch.values[390] == 0.0) { 1.0 } else { 0.0 };

        if ((((scratch.values[1323] != 0.0) && (scratch.values[1340] != 0.0)) && (!(scratch.values[1415] != 0.0))) && (scratch.values[1420] != 0.0)) {
            scratch.values[1368] = 0.0;
            scratch.node_derivatives[1368] = [0.0; Instance::NODE_COUNT];
            scratch.branch_derivatives[1368] = [0.0; Instance::BRANCH_COUNT];
        }

        if ((((scratch.values[1323] != 0.0) && (scratch.values[1340] != 0.0)) && (!(scratch.values[1415] != 0.0))) && (!(scratch.values[1420] != 0.0))) {
            scratch.store_ad(1369, &AdValue::scale(AdValue::div(AdValue::scale(scratch.ad_value(1366), scratch.values[458]), scratch.ad_value(1362)), scratch.values[488]));
        }

        if ((((scratch.values[1323] != 0.0) && (scratch.values[1340] != 0.0)) && (!(scratch.values[1415] != 0.0))) && (!(scratch.values[1420] != 0.0))) {
            scratch.store_ad(1370, &AdValue::div_from_scalar((0.666666666666667 * scratch.values[485]), scratch.ad_value(1369)));
        }

        if ((((scratch.values[1323] != 0.0) && (scratch.values[1340] != 0.0)) && (!(scratch.values[1415] != 0.0))) && (!(scratch.values[1420] != 0.0))) {
            scratch.store_ad(1371, &AdValue::square(scratch.ad_value(1370)));
        }

        if ((((scratch.values[1323] != 0.0) && (scratch.values[1340] != 0.0)) && (!(scratch.values[1415] != 0.0))) && (!(scratch.values[1420] != 0.0))) {
            scratch.store_ad(1372, &AdValue::sqrt(AdValue::div(AdValue::square(scratch.ad_value(1371)), AdValue::offset(AdValue::square(scratch.ad_value(1371)), 1.0))));
        }

        if ((((scratch.values[1323] != 0.0) && (scratch.values[1340] != 0.0)) && (!(scratch.values[1415] != 0.0))) && (!(scratch.values[1420] != 0.0))) {
            scratch.store_ad(1373, &AdValue::sqrt(AdValue::abs(scratch.ad_value(1372))));
        }

        if ((((scratch.values[1323] != 0.0) && (scratch.values[1340] != 0.0)) && (!(scratch.values[1415] != 0.0))) && (!(scratch.values[1420] != 0.0))) {
            scratch.store_ad(1374, &AdValue::mul(scratch.ad_value(1372), scratch.ad_value(1373)));
        }

        scratch.values[1421] = if (((-scratch.values[376]) * scratch.values[461]) == (-1.0)) { 1.0 } else { 0.0 };

        if (((((scratch.values[1323] != 0.0) && (scratch.values[1340] != 0.0)) && (!(scratch.values[1415] != 0.0))) && (!(scratch.values[1420] != 0.0))) && (scratch.values[1421] != 0.0)) {
            scratch.store_ad(1375, &AdValue::div_from_scalar(1.0, AdValue::offset(AdValue::mul(scratch.ad_value(1369), scratch.ad_value(1374)), 1.0)));
        }

        if (((((scratch.values[1323] != 0.0) && (scratch.values[1340] != 0.0)) && (!(scratch.values[1415] != 0.0))) && (!(scratch.values[1420] != 0.0))) && (!(scratch.values[1421] != 0.0))) {
            scratch.store_ad(1375, &AdValue::powf(AdValue::offset(AdValue::mul(scratch.ad_value(1369), scratch.ad_value(1374)), 1.0), ((-scratch.values[376]) * scratch.values[461])));
        }

        if ((((scratch.values[1323] != 0.0) && (scratch.values[1340] != 0.0)) && (!(scratch.values[1415] != 0.0))) && (!(scratch.values[1420] != 0.0))) {
            scratch.store_ad(1376, &AdValue::div(AdValue::mul(scratch.ad_value(1365), scratch.ad_value(1375)), AdValue::add(scratch.ad_value(1365), scratch.ad_value(1375))));
        }

        if ((((scratch.values[1323] != 0.0) && (scratch.values[1340] != 0.0)) && (!(scratch.values[1415] != 0.0))) && (!(scratch.values[1420] != 0.0))) {
            scratch.store_ad(1377, &AdValue::sqrt(AdValue::scale(AdValue::div(scratch.ad_value(1369), scratch.ad_value(1373)), 0.375)));
        }

        if ((((scratch.values[1323] != 0.0) && (scratch.values[1340] != 0.0)) && (!(scratch.values[1415] != 0.0))) && (!(scratch.values[1420] != 0.0))) {
            scratch.store_ad(1378, &AdValue::sub(AdValue::scale(AdValue::mul(scratch.ad_value(1370), scratch.ad_value(1373)), 2.0), scratch.ad_value(1372)));
        }

        if ((((scratch.values[1323] != 0.0) && (scratch.values[1340] != 0.0)) && (!(scratch.values[1415] != 0.0))) && (!(scratch.values[1420] != 0.0))) {
            scratch.store_ad(1379, &AdValue::add(AdValue::sub(AdValue::mul(AdValue::scale(scratch.ad_value(1370), scratch.values[485]), scratch.ad_value(1373)), AdValue::scale(scratch.ad_value(1372), scratch.values[485])), AdValue::scale(AdValue::mul(scratch.ad_value(1369), scratch.ad_value(1374)), 0.5)));
        }

        if ((((scratch.values[1323] != 0.0) && (scratch.values[1340] != 0.0)) && (!(scratch.values[1415] != 0.0))) && (!(scratch.values[1420] != 0.0))) {
            scratch.store_ad(1380, &AdValue::mul(AdValue::offset(scratch.ad_value(1378), (-1.0)), scratch.ad_value(1377)));
        }

        if ((((scratch.values[1323] != 0.0) && (scratch.values[1340] != 0.0)) && (!(scratch.values[1415] != 0.0))) && (!(scratch.values[1420] != 0.0))) {
            scratch.store_ad(1341, &AdValue::square(scratch.ad_value(1380)));
        }

        scratch.values[1422] = if (scratch.values[1380] > 0.0) { 1.0 } else { 0.0 };

        if (((((scratch.values[1323] != 0.0) && (scratch.values[1340] != 0.0)) && (!(scratch.values[1415] != 0.0))) && (!(scratch.values[1420] != 0.0))) && (scratch.values[1422] != 0.0)) {
            scratch.store_ad(1342, &AdValue::div_from_scalar(1.0, AdValue::offset(AdValue::scale(scratch.ad_value(1380), scratch.values[421]), 1.0)));
        }

        if (((((scratch.values[1323] != 0.0) && (scratch.values[1340] != 0.0)) && (!(scratch.values[1415] != 0.0))) && (!(scratch.values[1420] != 0.0))) && (!(scratch.values[1422] != 0.0))) {
            scratch.store_ad(1342, &AdValue::div_from_scalar(1.0, AdValue::sub_from_scalar(1.0, AdValue::scale(scratch.ad_value(1380), scratch.values[421]))));
        }

        scratch.values[1423] = if (((-scratch.values[1341]) + scratch.values[1379]) > (-230.25850929940458)) { 1.0 } else { 0.0 };

        if (((((scratch.values[1323] != 0.0) && (scratch.values[1340] != 0.0)) && (!(scratch.values[1415] != 0.0))) && (!(scratch.values[1420] != 0.0))) && (scratch.values[1423] != 0.0)) {
            scratch.store_ad(1359, &AdValue::exp(AdValue::sub(scratch.ad_value(1379), scratch.ad_value(1341))));
        }

        if (((((scratch.values[1323] != 0.0) && (scratch.values[1340] != 0.0)) && (!(scratch.values[1415] != 0.0))) && (!(scratch.values[1420] != 0.0))) && (!(scratch.values[1423] != 0.0))) {
            let assign15270_ad_e12691: AdValue = AdValue::div_from_scalar(1e-100, AdValue::offset(AdValue::mul(AdValue::sub_from_scalar((-230.25850929940458), AdValue::sub(scratch.ad_value(1379), scratch.ad_value(1341))), AdValue::offset(AdValue::scale(AdValue::mul(AdValue::sub_from_scalar((-230.25850929940458), AdValue::sub(scratch.ad_value(1379), scratch.ad_value(1341))), AdValue::offset(AdValue::scale(AdValue::sub_from_scalar((-230.25850929940458), AdValue::sub(scratch.ad_value(1379), scratch.ad_value(1341))), 0.3333333333333333), 1.0)), 0.5), 1.0)), 1.0));
            scratch.store_ad(1359, &assign15270_ad_e12691);
        }

        if ((((scratch.values[1323] != 0.0) && (scratch.values[1340] != 0.0)) && (!(scratch.values[1415] != 0.0))) && (!(scratch.values[1420] != 0.0))) {
            scratch.store_ad(1343, &AdValue::mul(AdValue::add(AdValue::add(AdValue::scale(scratch.ad_value(1342), 0.29214664), AdValue::scale(AdValue::square(scratch.ad_value(1342)), scratch.values[422])), AdValue::scale(AdValue::mul(AdValue::square(scratch.ad_value(1342)), scratch.ad_value(1342)), scratch.values[423])), scratch.ad_value(1359)));
        }

        scratch.values[1424] = if (scratch.values[1380] > 0.0) { 1.0 } else { 0.0 };

        if (((((scratch.values[1323] != 0.0) && (scratch.values[1340] != 0.0)) && (!(scratch.values[1415] != 0.0))) && (!(scratch.values[1420] != 0.0))) && (scratch.values[1424] != 0.0)) {
            scratch.values[1381] = scratch.values[1343];
            scratch.node_derivatives[1381] = scratch.node_derivatives[1343];
            scratch.branch_derivatives[1381] = scratch.branch_derivatives[1343];
        }

        scratch.values[1425] = if (scratch.values[1379] > (-230.25850929940458)) { 1.0 } else { 0.0 };

        if ((((((scratch.values[1323] != 0.0) && (scratch.values[1340] != 0.0)) && (!(scratch.values[1415] != 0.0))) && (!(scratch.values[1420] != 0.0))) && (!(scratch.values[1424] != 0.0))) && (scratch.values[1425] != 0.0)) {
            scratch.store_ad(1359, &AdValue::exp(scratch.ad_value(1379)));
        }

        if ((((((scratch.values[1323] != 0.0) && (scratch.values[1340] != 0.0)) && (!(scratch.values[1415] != 0.0))) && (!(scratch.values[1420] != 0.0))) && (!(scratch.values[1424] != 0.0))) && (!(scratch.values[1425] != 0.0))) {
            scratch.store_ad(1359, &AdValue::div_from_scalar(1e-100, AdValue::offset(AdValue::mul(AdValue::sub_from_scalar((-230.25850929940458), scratch.ad_value(1379)), AdValue::offset(AdValue::scale(AdValue::mul(AdValue::sub_from_scalar((-230.25850929940458), scratch.ad_value(1379)), AdValue::offset(AdValue::scale(AdValue::sub_from_scalar((-230.25850929940458), scratch.ad_value(1379)), 0.3333333333333333), 1.0)), 0.5), 1.0)), 1.0)));
        }

        if (((((scratch.values[1323] != 0.0) && (scratch.values[1340] != 0.0)) && (!(scratch.values[1415] != 0.0))) && (!(scratch.values[1420] != 0.0))) && (!(scratch.values[1424] != 0.0))) {
            scratch.store_ad(1381, &AdValue::sub(AdValue::scale(scratch.ad_value(1359), 2.0), scratch.ad_value(1343)));
        }

        if ((((scratch.values[1323] != 0.0) && (scratch.values[1340] != 0.0)) && (!(scratch.values[1415] != 0.0))) && (!(scratch.values[1420] != 0.0))) {
            scratch.store_ad(1382, &AdValue::scale(AdValue::div(AdValue::scale(scratch.ad_value(1381), scratch.values[485]), scratch.ad_value(1377)), (1.772453850905516 * 0.5)));
        }

        if ((((scratch.values[1323] != 0.0) && (scratch.values[1340] != 0.0)) && (!(scratch.values[1415] != 0.0))) && (!(scratch.values[1420] != 0.0))) {
            scratch.store_ad(1368, &AdValue::scale(AdValue::mul(AdValue::mul(scratch.ad_value(1367), scratch.ad_value(1382)), scratch.ad_value(1376)), scratch.values[390]));
        }

        scratch.values[1426] = if (scratch.values[396] == 0.0) { 1.0 } else { 0.0 };

        if ((((scratch.values[1323] != 0.0) && (scratch.values[1340] != 0.0)) && (!(scratch.values[1415] != 0.0))) && (scratch.values[1426] != 0.0)) {
            scratch.values[1383] = 0.0;
            scratch.node_derivatives[1383] = [0.0; Instance::NODE_COUNT];
            scratch.branch_derivatives[1383] = [0.0; Instance::BRANCH_COUNT];
        }

        scratch.values[1427] = if (scratch.values[376] == 0.5) { 1.0 } else { 0.0 };

        if (((((scratch.values[1323] != 0.0) && (scratch.values[1340] != 0.0)) && (!(scratch.values[1415] != 0.0))) && (!(scratch.values[1426] != 0.0))) && (scratch.values[1427] != 0.0)) {
            scratch.store_ad(1359, &AdValue::sqrt(AdValue::scale(AdValue::sub_from_scalar(scratch.values[373], scratch.ad_value(1357)), scratch.values[479])));
        }

        if (((((scratch.values[1323] != 0.0) && (scratch.values[1340] != 0.0)) && (!(scratch.values[1415] != 0.0))) && (!(scratch.values[1426] != 0.0))) && (!(scratch.values[1427] != 0.0))) {
            scratch.store_ad(1359, &AdValue::powf(AdValue::scale(AdValue::sub_from_scalar(scratch.values[373], scratch.ad_value(1357)), scratch.values[479]), scratch.values[376]));
        }

        if ((((scratch.values[1323] != 0.0) && (scratch.values[1340] != 0.0)) && (!(scratch.values[1415] != 0.0))) && (!(scratch.values[1426] != 0.0))) {
            scratch.store_ad(1384, &AdValue::scale(AdValue::div(AdValue::scale(AdValue::sub_from_scalar(scratch.values[373], scratch.ad_value(1357)), scratch.values[476]), scratch.ad_value(1359)), scratch.values[461]));
        }

        scratch.values[1428] = if (((((-scratch.values[491]) / scratch.values[1384])) as f64).abs() < 230.25850929940458) { 1.0 } else { 0.0 };

        if (((((scratch.values[1323] != 0.0) && (scratch.values[1340] != 0.0)) && (!(scratch.values[1415] != 0.0))) && (!(scratch.values[1426] != 0.0))) && (scratch.values[1428] != 0.0)) {
            scratch.store_ad(1359, &AdValue::exp(AdValue::div(AdValue::neg(scratch.ad_value(491)), scratch.ad_value(1384))));
        }

        scratch.values[1429] = if (((-scratch.values[491]) / scratch.values[1384]) < 0.0) { 1.0 } else { 0.0 };

        if ((((((scratch.values[1323] != 0.0) && (scratch.values[1340] != 0.0)) && (!(scratch.values[1415] != 0.0))) && (!(scratch.values[1426] != 0.0))) && (!(scratch.values[1428] != 0.0))) && (scratch.values[1429] != 0.0)) {
            let assign15460_ad_e13018: AdValue = AdValue::offset(AdValue::mul(AdValue::sub_from_scalar((-230.25850929940458), AdValue::div(AdValue::neg(scratch.ad_value(491)), scratch.ad_value(1384))), AdValue::offset(AdValue::scale(AdValue::mul(AdValue::sub_from_scalar((-230.25850929940458), AdValue::div(AdValue::neg(scratch.ad_value(491)), scratch.ad_value(1384))), AdValue::offset(AdValue::scale(AdValue::sub_from_scalar((-230.25850929940458), AdValue::div(AdValue::neg(scratch.ad_value(491)), scratch.ad_value(1384))), 0.3333333333333333), 1.0)), 0.5), 1.0)), 1.0);
            scratch.store_ad(1359, &AdValue::div_from_scalar(1e-100, assign15460_ad_e13018));
        }

        if ((((((scratch.values[1323] != 0.0) && (scratch.values[1340] != 0.0)) && (!(scratch.values[1415] != 0.0))) && (!(scratch.values[1426] != 0.0))) && (!(scratch.values[1428] != 0.0))) && (!(scratch.values[1429] != 0.0))) {
            let assign15470_ad_e13068: AdValue = AdValue::scale(AdValue::offset(AdValue::mul(AdValue::offset(AdValue::div(AdValue::neg(scratch.ad_value(491)), scratch.ad_value(1384)), (-230.25850929940458)), AdValue::offset(AdValue::scale(AdValue::mul(AdValue::offset(AdValue::div(AdValue::neg(scratch.ad_value(491)), scratch.ad_value(1384)), (-230.25850929940458)), AdValue::offset(AdValue::scale(AdValue::offset(AdValue::div(AdValue::neg(scratch.ad_value(491)), scratch.ad_value(1384)), (-230.25850929940458)), 0.3333333333333333), 1.0)), 0.5), 1.0)), 1.0), 1e100);
            scratch.store_ad(1359, &assign15470_ad_e13068);
        }

        if ((((scratch.values[1323] != 0.0) && (scratch.values[1340] != 0.0)) && (!(scratch.values[1415] != 0.0))) && (!(scratch.values[1426] != 0.0))) {
            scratch.store_ad(1383, &AdValue::scale(AdValue::mul(AdValue::mul(AdValue::mul(scratch.ad_value(516), scratch.ad_value(1384)), scratch.ad_value(1384)), scratch.ad_value(1359)), scratch.values[396]));
        }

        scratch.values[1430] = if (scratch.values[405] > 1000.0) { 1.0 } else { 0.0 };

        if ((((scratch.values[1323] != 0.0) && (scratch.values[1340] != 0.0)) && (!(scratch.values[1415] != 0.0))) && (scratch.values[1430] != 0.0)) {
            scratch.values[1385] = 1.0;
            scratch.node_derivatives[1385] = [0.0; Instance::NODE_COUNT];
            scratch.branch_derivatives[1385] = [0.0; Instance::BRANCH_COUNT];
        }

        scratch.values[1431] = if (scratch.values[1358] > ((-scratch.values[493]) * scratch.values[405])) { 1.0 } else { 0.0 };

        scratch.values[1432] = if (scratch.values[408] == 4.0) { 1.0 } else { 0.0 };

        if ((((((scratch.values[1323] != 0.0) && (scratch.values[1340] != 0.0)) && (!(scratch.values[1415] != 0.0))) && (!(scratch.values[1430] != 0.0))) && (scratch.values[1431] != 0.0)) && (scratch.values[1432] != 0.0)) {
            scratch.store_ad(1359, &AdValue::mul(AdValue::mul(AdValue::mul(AdValue::scale(scratch.ad_value(1358), scratch.values[498]), AdValue::scale(scratch.ad_value(1358), scratch.values[498])), AdValue::scale(scratch.ad_value(1358), scratch.values[498])), AdValue::scale(scratch.ad_value(1358), scratch.values[498])));
        }

        if ((((((scratch.values[1323] != 0.0) && (scratch.values[1340] != 0.0)) && (!(scratch.values[1415] != 0.0))) && (!(scratch.values[1430] != 0.0))) && (scratch.values[1431] != 0.0)) && (!(scratch.values[1432] != 0.0))) {
            scratch.store_ad(1359, &AdValue::powf(AdValue::abs(AdValue::scale(scratch.ad_value(1358), scratch.values[498])), scratch.values[408]));
        }

        if (((((scratch.values[1323] != 0.0) && (scratch.values[1340] != 0.0)) && (!(scratch.values[1415] != 0.0))) && (!(scratch.values[1430] != 0.0))) && (scratch.values[1431] != 0.0)) {
            scratch.store_ad(1385, &AdValue::div_from_scalar(1.0, AdValue::sub_from_scalar(1.0, scratch.ad_value(1359))));
        }

        if (((((scratch.values[1323] != 0.0) && (scratch.values[1340] != 0.0)) && (!(scratch.values[1415] != 0.0))) && (!(scratch.values[1430] != 0.0))) && (!(scratch.values[1431] != 0.0))) {
            scratch.store_ad(1385, &AdValue::offset(AdValue::scale(AdValue::offset(scratch.ad_value(1358), (scratch.values[493] * scratch.values[405])), scratch.values[501]), scratch.values[495]));
        }

        if (((scratch.values[1323] != 0.0) && (scratch.values[1340] != 0.0)) && (!(scratch.values[1415] != 0.0))) {
            scratch.store_ad(1387, &AdValue::mul(AdValue::add(AdValue::add(AdValue::add(scratch.ad_value(1360), scratch.ad_value(1361)), scratch.ad_value(1368)), scratch.ad_value(1383)), scratch.ad_value(1385)));
        }

        scratch.values[1433] = if (scratch.values[692] == 0.0) { 1.0 } else { 0.0 };

        if (((scratch.values[1323] != 0.0) && (scratch.values[1340] != 0.0)) && (scratch.values[1433] != 0.0)) {
            scratch.values[1388] = 0.0;
            scratch.node_derivatives[1388] = [0.0; Instance::NODE_COUNT];
            scratch.branch_derivatives[1388] = [0.0; Instance::BRANCH_COUNT];
        }

        scratch.values[1434] = if (scratch.values[459] == 0.5) { 1.0 } else { 0.0 };

        if ((((scratch.values[1323] != 0.0) && (scratch.values[1340] != 0.0)) && (!(scratch.values[1433] != 0.0))) && (scratch.values[1434] != 0.0)) {
            scratch.store_ad(1359, &AdValue::sqrt(AdValue::sub_from_scalar(1.0, AdValue::scale(scratch.ad_value(1351), scratch.values[456]))));
        }

        if ((((scratch.values[1323] != 0.0) && (scratch.values[1340] != 0.0)) && (!(scratch.values[1433] != 0.0))) && (!(scratch.values[1434] != 0.0))) {
            scratch.store_ad(1359, &AdValue::powf(AdValue::sub_from_scalar(1.0, AdValue::scale(scratch.ad_value(1351), scratch.values[456])), scratch.values[459]));
        }

        if (((scratch.values[1323] != 0.0) && (scratch.values[1340] != 0.0)) && (!(scratch.values[1433] != 0.0))) {
            scratch.store_ad(1360, &AdValue::scale(scratch.ad_value(1350), scratch.values[438]));
        }

        scratch.values[1435] = if ((scratch.values[388] == 0.0) && (scratch.values[391] == 0.0)) { 1.0 } else { 0.0 };

        if ((((scratch.values[1323] != 0.0) && (scratch.values[1340] != 0.0)) && (!(scratch.values[1433] != 0.0))) && (scratch.values[1435] != 0.0)) {
            scratch.values[1361] = 0.0;
            scratch.node_derivatives[1361] = [0.0; Instance::NODE_COUNT];
            scratch.branch_derivatives[1361] = [0.0; Instance::BRANCH_COUNT];
        }

        if ((((scratch.values[1323] != 0.0) && (scratch.values[1340] != 0.0)) && (!(scratch.values[1433] != 0.0))) && (!(scratch.values[1435] != 0.0))) {
            scratch.store_ad(1362, &AdValue::sub_from_scalar(scratch.values[444], scratch.ad_value(1356)));
        }

        if ((((scratch.values[1323] != 0.0) && (scratch.values[1340] != 0.0)) && (!(scratch.values[1433] != 0.0))) && (!(scratch.values[1435] != 0.0))) {
            scratch.store_ad(1363, &AdValue::sub_from_scalar(1.0, AdValue::sqrt(AdValue::sub_from_scalar(1.0, AdValue::div(scratch.ad_value(1354), scratch.ad_value(1362))))));
        }

        scratch.values[1436] = if (scratch.values[377] == 0.5) { 1.0 } else { 0.0 };

        if (((((scratch.values[1323] != 0.0) && (scratch.values[1340] != 0.0)) && (!(scratch.values[1433] != 0.0))) && (!(scratch.values[1435] != 0.0))) && (scratch.values[1436] != 0.0)) {
            scratch.values[1364] = 0.0;
            scratch.node_derivatives[1364] = [0.0; Instance::NODE_COUNT];
            scratch.branch_derivatives[1364] = [0.0; Instance::BRANCH_COUNT];
        }

        if (((((scratch.values[1323] != 0.0) && (scratch.values[1340] != 0.0)) && (!(scratch.values[1433] != 0.0))) && (!(scratch.values[1435] != 0.0))) && (!(scratch.values[1436] != 0.0))) {
            scratch.store_ad(1364, &AdValue::scale(AdValue::add(AdValue::div(AdValue::mul(AdValue::square(scratch.ad_value(1363)), AdValue::ln(scratch.ad_value(1363))), AdValue::sub_from_scalar(1.0, scratch.ad_value(1363))), scratch.ad_value(1363)), (1.0 - (2.0 * scratch.values[377]))));
        }

        if ((((scratch.values[1323] != 0.0) && (scratch.values[1340] != 0.0)) && (!(scratch.values[1433] != 0.0))) && (!(scratch.values[1435] != 0.0))) {
            scratch.store_ad(1365, &AdValue::add(scratch.ad_value(1363), scratch.ad_value(1364)));
        }

        scratch.values[1437] = if (scratch.values[377] == 0.5) { 1.0 } else { 0.0 };

        if (((((scratch.values[1323] != 0.0) && (scratch.values[1340] != 0.0)) && (!(scratch.values[1433] != 0.0))) && (!(scratch.values[1435] != 0.0))) && (scratch.values[1437] != 0.0)) {
            scratch.store_ad(1359, &AdValue::sqrt(AdValue::scale(scratch.ad_value(1362), scratch.values[480])));
        }

        if (((((scratch.values[1323] != 0.0) && (scratch.values[1340] != 0.0)) && (!(scratch.values[1433] != 0.0))) && (!(scratch.values[1435] != 0.0))) && (!(scratch.values[1437] != 0.0))) {
            scratch.store_ad(1359, &AdValue::powf(AdValue::scale(scratch.ad_value(1362), scratch.values[480]), scratch.values[377]));
        }

        if ((((scratch.values[1323] != 0.0) && (scratch.values[1340] != 0.0)) && (!(scratch.values[1433] != 0.0))) && (!(scratch.values[1435] != 0.0))) {
            scratch.store_ad(1366, &AdValue::scale(scratch.ad_value(1359), scratch.values[474]));
        }

        if ((((scratch.values[1323] != 0.0) && (scratch.values[1340] != 0.0)) && (!(scratch.values[1433] != 0.0))) && (!(scratch.values[1435] != 0.0))) {
            scratch.store_ad(1367, &AdValue::scale(AdValue::mul(AdValue::offset(scratch.ad_value(1353), (-1.0)), scratch.ad_value(1366)), scratch.values[435]));
        }

        if ((((scratch.values[1323] != 0.0) && (scratch.values[1340] != 0.0)) && (!(scratch.values[1433] != 0.0))) && (!(scratch.values[1435] != 0.0))) {
            scratch.store_ad(1361, &AdValue::scale(AdValue::mul(scratch.ad_value(1367), scratch.ad_value(1365)), scratch.values[388]));
        }

        scratch.values[1438] = if (scratch.values[391] == 0.0) { 1.0 } else { 0.0 };

        if ((((scratch.values[1323] != 0.0) && (scratch.values[1340] != 0.0)) && (!(scratch.values[1433] != 0.0))) && (scratch.values[1438] != 0.0)) {
            scratch.values[1368] = 0.0;
            scratch.node_derivatives[1368] = [0.0; Instance::NODE_COUNT];
            scratch.branch_derivatives[1368] = [0.0; Instance::BRANCH_COUNT];
        }

        if ((((scratch.values[1323] != 0.0) && (scratch.values[1340] != 0.0)) && (!(scratch.values[1433] != 0.0))) && (!(scratch.values[1438] != 0.0))) {
            scratch.store_ad(1369, &AdValue::scale(AdValue::div(AdValue::scale(scratch.ad_value(1366), scratch.values[459]), scratch.ad_value(1362)), scratch.values[489]));
        }

        if ((((scratch.values[1323] != 0.0) && (scratch.values[1340] != 0.0)) && (!(scratch.values[1433] != 0.0))) && (!(scratch.values[1438] != 0.0))) {
            scratch.store_ad(1370, &AdValue::div_from_scalar((0.666666666666667 * scratch.values[486]), scratch.ad_value(1369)));
        }

        if ((((scratch.values[1323] != 0.0) && (scratch.values[1340] != 0.0)) && (!(scratch.values[1433] != 0.0))) && (!(scratch.values[1438] != 0.0))) {
            scratch.store_ad(1371, &AdValue::square(scratch.ad_value(1370)));
        }

        if ((((scratch.values[1323] != 0.0) && (scratch.values[1340] != 0.0)) && (!(scratch.values[1433] != 0.0))) && (!(scratch.values[1438] != 0.0))) {
            scratch.store_ad(1372, &AdValue::sqrt(AdValue::div(AdValue::square(scratch.ad_value(1371)), AdValue::offset(AdValue::square(scratch.ad_value(1371)), 1.0))));
        }

        if ((((scratch.values[1323] != 0.0) && (scratch.values[1340] != 0.0)) && (!(scratch.values[1433] != 0.0))) && (!(scratch.values[1438] != 0.0))) {
            scratch.store_ad(1373, &AdValue::sqrt(AdValue::abs(scratch.ad_value(1372))));
        }

        if ((((scratch.values[1323] != 0.0) && (scratch.values[1340] != 0.0)) && (!(scratch.values[1433] != 0.0))) && (!(scratch.values[1438] != 0.0))) {
            scratch.store_ad(1374, &AdValue::mul(scratch.ad_value(1372), scratch.ad_value(1373)));
        }

        scratch.values[1439] = if (((-scratch.values[377]) * scratch.values[462]) == (-1.0)) { 1.0 } else { 0.0 };

        if (((((scratch.values[1323] != 0.0) && (scratch.values[1340] != 0.0)) && (!(scratch.values[1433] != 0.0))) && (!(scratch.values[1438] != 0.0))) && (scratch.values[1439] != 0.0)) {
            scratch.store_ad(1375, &AdValue::div_from_scalar(1.0, AdValue::offset(AdValue::mul(scratch.ad_value(1369), scratch.ad_value(1374)), 1.0)));
        }

        if (((((scratch.values[1323] != 0.0) && (scratch.values[1340] != 0.0)) && (!(scratch.values[1433] != 0.0))) && (!(scratch.values[1438] != 0.0))) && (!(scratch.values[1439] != 0.0))) {
            scratch.store_ad(1375, &AdValue::powf(AdValue::offset(AdValue::mul(scratch.ad_value(1369), scratch.ad_value(1374)), 1.0), ((-scratch.values[377]) * scratch.values[462])));
        }

        if ((((scratch.values[1323] != 0.0) && (scratch.values[1340] != 0.0)) && (!(scratch.values[1433] != 0.0))) && (!(scratch.values[1438] != 0.0))) {
            scratch.store_ad(1376, &AdValue::div(AdValue::mul(scratch.ad_value(1365), scratch.ad_value(1375)), AdValue::add(scratch.ad_value(1365), scratch.ad_value(1375))));
        }

        if ((((scratch.values[1323] != 0.0) && (scratch.values[1340] != 0.0)) && (!(scratch.values[1433] != 0.0))) && (!(scratch.values[1438] != 0.0))) {
            scratch.store_ad(1377, &AdValue::sqrt(AdValue::scale(AdValue::div(scratch.ad_value(1369), scratch.ad_value(1373)), 0.375)));
        }

        if ((((scratch.values[1323] != 0.0) && (scratch.values[1340] != 0.0)) && (!(scratch.values[1433] != 0.0))) && (!(scratch.values[1438] != 0.0))) {
            scratch.store_ad(1378, &AdValue::sub(AdValue::scale(AdValue::mul(scratch.ad_value(1370), scratch.ad_value(1373)), 2.0), scratch.ad_value(1372)));
        }

        if ((((scratch.values[1323] != 0.0) && (scratch.values[1340] != 0.0)) && (!(scratch.values[1433] != 0.0))) && (!(scratch.values[1438] != 0.0))) {
            scratch.store_ad(1379, &AdValue::add(AdValue::sub(AdValue::mul(AdValue::scale(scratch.ad_value(1370), scratch.values[486]), scratch.ad_value(1373)), AdValue::scale(scratch.ad_value(1372), scratch.values[486])), AdValue::scale(AdValue::mul(scratch.ad_value(1369), scratch.ad_value(1374)), 0.5)));
        }

        if ((((scratch.values[1323] != 0.0) && (scratch.values[1340] != 0.0)) && (!(scratch.values[1433] != 0.0))) && (!(scratch.values[1438] != 0.0))) {
            scratch.store_ad(1380, &AdValue::mul(AdValue::offset(scratch.ad_value(1378), (-1.0)), scratch.ad_value(1377)));
        }

        if ((((scratch.values[1323] != 0.0) && (scratch.values[1340] != 0.0)) && (!(scratch.values[1433] != 0.0))) && (!(scratch.values[1438] != 0.0))) {
            scratch.store_ad(1341, &AdValue::square(scratch.ad_value(1380)));
        }

        scratch.values[1440] = if (scratch.values[1380] > 0.0) { 1.0 } else { 0.0 };

        if (((((scratch.values[1323] != 0.0) && (scratch.values[1340] != 0.0)) && (!(scratch.values[1433] != 0.0))) && (!(scratch.values[1438] != 0.0))) && (scratch.values[1440] != 0.0)) {
            scratch.store_ad(1342, &AdValue::div_from_scalar(1.0, AdValue::offset(AdValue::scale(scratch.ad_value(1380), scratch.values[421]), 1.0)));
        }

        if (((((scratch.values[1323] != 0.0) && (scratch.values[1340] != 0.0)) && (!(scratch.values[1433] != 0.0))) && (!(scratch.values[1438] != 0.0))) && (!(scratch.values[1440] != 0.0))) {
            scratch.store_ad(1342, &AdValue::div_from_scalar(1.0, AdValue::sub_from_scalar(1.0, AdValue::scale(scratch.ad_value(1380), scratch.values[421]))));
        }

        scratch.values[1441] = if (((-scratch.values[1341]) + scratch.values[1379]) > (-230.25850929940458)) { 1.0 } else { 0.0 };

        if (((((scratch.values[1323] != 0.0) && (scratch.values[1340] != 0.0)) && (!(scratch.values[1433] != 0.0))) && (!(scratch.values[1438] != 0.0))) && (scratch.values[1441] != 0.0)) {
            scratch.store_ad(1359, &AdValue::exp(AdValue::sub(scratch.ad_value(1379), scratch.ad_value(1341))));
        }

        if (((((scratch.values[1323] != 0.0) && (scratch.values[1340] != 0.0)) && (!(scratch.values[1433] != 0.0))) && (!(scratch.values[1438] != 0.0))) && (!(scratch.values[1441] != 0.0))) {
            let assign16020_ad_e13897: AdValue = AdValue::div_from_scalar(1e-100, AdValue::offset(AdValue::mul(AdValue::sub_from_scalar((-230.25850929940458), AdValue::sub(scratch.ad_value(1379), scratch.ad_value(1341))), AdValue::offset(AdValue::scale(AdValue::mul(AdValue::sub_from_scalar((-230.25850929940458), AdValue::sub(scratch.ad_value(1379), scratch.ad_value(1341))), AdValue::offset(AdValue::scale(AdValue::sub_from_scalar((-230.25850929940458), AdValue::sub(scratch.ad_value(1379), scratch.ad_value(1341))), 0.3333333333333333), 1.0)), 0.5), 1.0)), 1.0));
            scratch.store_ad(1359, &assign16020_ad_e13897);
        }

        if ((((scratch.values[1323] != 0.0) && (scratch.values[1340] != 0.0)) && (!(scratch.values[1433] != 0.0))) && (!(scratch.values[1438] != 0.0))) {
            scratch.store_ad(1343, &AdValue::mul(AdValue::add(AdValue::add(AdValue::scale(scratch.ad_value(1342), 0.29214664), AdValue::scale(AdValue::square(scratch.ad_value(1342)), scratch.values[422])), AdValue::scale(AdValue::mul(AdValue::square(scratch.ad_value(1342)), scratch.ad_value(1342)), scratch.values[423])), scratch.ad_value(1359)));
        }

        scratch.values[1442] = if (scratch.values[1380] > 0.0) { 1.0 } else { 0.0 };

        if (((((scratch.values[1323] != 0.0) && (scratch.values[1340] != 0.0)) && (!(scratch.values[1433] != 0.0))) && (!(scratch.values[1438] != 0.0))) && (scratch.values[1442] != 0.0)) {
            scratch.values[1381] = scratch.values[1343];
            scratch.node_derivatives[1381] = scratch.node_derivatives[1343];
            scratch.branch_derivatives[1381] = scratch.branch_derivatives[1343];
        }

        scratch.values[1443] = if (scratch.values[1379] > (-230.25850929940458)) { 1.0 } else { 0.0 };

        if ((((((scratch.values[1323] != 0.0) && (scratch.values[1340] != 0.0)) && (!(scratch.values[1433] != 0.0))) && (!(scratch.values[1438] != 0.0))) && (!(scratch.values[1442] != 0.0))) && (scratch.values[1443] != 0.0)) {
            scratch.store_ad(1359, &AdValue::exp(scratch.ad_value(1379)));
        }

        if ((((((scratch.values[1323] != 0.0) && (scratch.values[1340] != 0.0)) && (!(scratch.values[1433] != 0.0))) && (!(scratch.values[1438] != 0.0))) && (!(scratch.values[1442] != 0.0))) && (!(scratch.values[1443] != 0.0))) {
            scratch.store_ad(1359, &AdValue::div_from_scalar(1e-100, AdValue::offset(AdValue::mul(AdValue::sub_from_scalar((-230.25850929940458), scratch.ad_value(1379)), AdValue::offset(AdValue::scale(AdValue::mul(AdValue::sub_from_scalar((-230.25850929940458), scratch.ad_value(1379)), AdValue::offset(AdValue::scale(AdValue::sub_from_scalar((-230.25850929940458), scratch.ad_value(1379)), 0.3333333333333333), 1.0)), 0.5), 1.0)), 1.0)));
        }

        if (((((scratch.values[1323] != 0.0) && (scratch.values[1340] != 0.0)) && (!(scratch.values[1433] != 0.0))) && (!(scratch.values[1438] != 0.0))) && (!(scratch.values[1442] != 0.0))) {
            scratch.store_ad(1381, &AdValue::sub(AdValue::scale(scratch.ad_value(1359), 2.0), scratch.ad_value(1343)));
        }

        if ((((scratch.values[1323] != 0.0) && (scratch.values[1340] != 0.0)) && (!(scratch.values[1433] != 0.0))) && (!(scratch.values[1438] != 0.0))) {
            scratch.store_ad(1382, &AdValue::scale(AdValue::div(AdValue::scale(scratch.ad_value(1381), scratch.values[486]), scratch.ad_value(1377)), (1.772453850905516 * 0.5)));
        }

        if ((((scratch.values[1323] != 0.0) && (scratch.values[1340] != 0.0)) && (!(scratch.values[1433] != 0.0))) && (!(scratch.values[1438] != 0.0))) {
            scratch.store_ad(1368, &AdValue::scale(AdValue::mul(AdValue::mul(scratch.ad_value(1367), scratch.ad_value(1382)), scratch.ad_value(1376)), scratch.values[391]));
        }

        scratch.values[1444] = if (scratch.values[397] == 0.0) { 1.0 } else { 0.0 };

        if ((((scratch.values[1323] != 0.0) && (scratch.values[1340] != 0.0)) && (!(scratch.values[1433] != 0.0))) && (scratch.values[1444] != 0.0)) {
            scratch.values[1383] = 0.0;
            scratch.node_derivatives[1383] = [0.0; Instance::NODE_COUNT];
            scratch.branch_derivatives[1383] = [0.0; Instance::BRANCH_COUNT];
        }

        scratch.values[1445] = if (scratch.values[377] == 0.5) { 1.0 } else { 0.0 };

        if (((((scratch.values[1323] != 0.0) && (scratch.values[1340] != 0.0)) && (!(scratch.values[1433] != 0.0))) && (!(scratch.values[1444] != 0.0))) && (scratch.values[1445] != 0.0)) {
            scratch.store_ad(1359, &AdValue::sqrt(AdValue::scale(AdValue::sub_from_scalar(scratch.values[374], scratch.ad_value(1357)), scratch.values[480])));
        }

        if (((((scratch.values[1323] != 0.0) && (scratch.values[1340] != 0.0)) && (!(scratch.values[1433] != 0.0))) && (!(scratch.values[1444] != 0.0))) && (!(scratch.values[1445] != 0.0))) {
            scratch.store_ad(1359, &AdValue::powf(AdValue::scale(AdValue::sub_from_scalar(scratch.values[374], scratch.ad_value(1357)), scratch.values[480]), scratch.values[377]));
        }

        if ((((scratch.values[1323] != 0.0) && (scratch.values[1340] != 0.0)) && (!(scratch.values[1433] != 0.0))) && (!(scratch.values[1444] != 0.0))) {
            scratch.store_ad(1384, &AdValue::scale(AdValue::div(AdValue::scale(AdValue::sub_from_scalar(scratch.values[374], scratch.ad_value(1357)), scratch.values[477]), scratch.ad_value(1359)), scratch.values[462]));
        }

        scratch.values[1446] = if (((((-scratch.values[492]) / scratch.values[1384])) as f64).abs() < 230.25850929940458) { 1.0 } else { 0.0 };

        if (((((scratch.values[1323] != 0.0) && (scratch.values[1340] != 0.0)) && (!(scratch.values[1433] != 0.0))) && (!(scratch.values[1444] != 0.0))) && (scratch.values[1446] != 0.0)) {
            scratch.store_ad(1359, &AdValue::exp(AdValue::div(AdValue::neg(scratch.ad_value(492)), scratch.ad_value(1384))));
        }

        scratch.values[1447] = if (((-scratch.values[492]) / scratch.values[1384]) < 0.0) { 1.0 } else { 0.0 };

        if ((((((scratch.values[1323] != 0.0) && (scratch.values[1340] != 0.0)) && (!(scratch.values[1433] != 0.0))) && (!(scratch.values[1444] != 0.0))) && (!(scratch.values[1446] != 0.0))) && (scratch.values[1447] != 0.0)) {
            let assign16210_ad_e14224: AdValue = AdValue::offset(AdValue::mul(AdValue::sub_from_scalar((-230.25850929940458), AdValue::div(AdValue::neg(scratch.ad_value(492)), scratch.ad_value(1384))), AdValue::offset(AdValue::scale(AdValue::mul(AdValue::sub_from_scalar((-230.25850929940458), AdValue::div(AdValue::neg(scratch.ad_value(492)), scratch.ad_value(1384))), AdValue::offset(AdValue::scale(AdValue::sub_from_scalar((-230.25850929940458), AdValue::div(AdValue::neg(scratch.ad_value(492)), scratch.ad_value(1384))), 0.3333333333333333), 1.0)), 0.5), 1.0)), 1.0);
            scratch.store_ad(1359, &AdValue::div_from_scalar(1e-100, assign16210_ad_e14224));
        }

        if ((((((scratch.values[1323] != 0.0) && (scratch.values[1340] != 0.0)) && (!(scratch.values[1433] != 0.0))) && (!(scratch.values[1444] != 0.0))) && (!(scratch.values[1446] != 0.0))) && (!(scratch.values[1447] != 0.0))) {
            let assign16220_ad_e14274: AdValue = AdValue::scale(AdValue::offset(AdValue::mul(AdValue::offset(AdValue::div(AdValue::neg(scratch.ad_value(492)), scratch.ad_value(1384)), (-230.25850929940458)), AdValue::offset(AdValue::scale(AdValue::mul(AdValue::offset(AdValue::div(AdValue::neg(scratch.ad_value(492)), scratch.ad_value(1384)), (-230.25850929940458)), AdValue::offset(AdValue::scale(AdValue::offset(AdValue::div(AdValue::neg(scratch.ad_value(492)), scratch.ad_value(1384)), (-230.25850929940458)), 0.3333333333333333), 1.0)), 0.5), 1.0)), 1.0), 1e100);
            scratch.store_ad(1359, &assign16220_ad_e14274);
        }

        if ((((scratch.values[1323] != 0.0) && (scratch.values[1340] != 0.0)) && (!(scratch.values[1433] != 0.0))) && (!(scratch.values[1444] != 0.0))) {
            scratch.store_ad(1383, &AdValue::scale(AdValue::mul(AdValue::mul(AdValue::mul(scratch.ad_value(516), scratch.ad_value(1384)), scratch.ad_value(1384)), scratch.ad_value(1359)), scratch.values[397]));
        }

        scratch.values[1448] = if (scratch.values[406] > 1000.0) { 1.0 } else { 0.0 };

        if ((((scratch.values[1323] != 0.0) && (scratch.values[1340] != 0.0)) && (!(scratch.values[1433] != 0.0))) && (scratch.values[1448] != 0.0)) {
            scratch.values[1385] = 1.0;
            scratch.node_derivatives[1385] = [0.0; Instance::NODE_COUNT];
            scratch.branch_derivatives[1385] = [0.0; Instance::BRANCH_COUNT];
        }

    }

    pub(super) fn stamp_transient_block_13(
        &mut self,
        ctx: &GeneratedEvalContext<'_>,
        stamper: &mut GeneratedStamper<'_>,
        scratch: &mut Scratch,
    ) {
        let _ = stamper;
        scratch.values[1449] = if (scratch.values[1358] > ((-scratch.values[493]) * scratch.values[406])) { 1.0 } else { 0.0 };

        scratch.values[1450] = if (scratch.values[409] == 4.0) { 1.0 } else { 0.0 };

        if ((((((scratch.values[1323] != 0.0) && (scratch.values[1340] != 0.0)) && (!(scratch.values[1433] != 0.0))) && (!(scratch.values[1448] != 0.0))) && (scratch.values[1449] != 0.0)) && (scratch.values[1450] != 0.0)) {
            scratch.store_ad(1359, &AdValue::mul(AdValue::mul(AdValue::mul(AdValue::scale(scratch.ad_value(1358), scratch.values[499]), AdValue::scale(scratch.ad_value(1358), scratch.values[499])), AdValue::scale(scratch.ad_value(1358), scratch.values[499])), AdValue::scale(scratch.ad_value(1358), scratch.values[499])));
        }

        if ((((((scratch.values[1323] != 0.0) && (scratch.values[1340] != 0.0)) && (!(scratch.values[1433] != 0.0))) && (!(scratch.values[1448] != 0.0))) && (scratch.values[1449] != 0.0)) && (!(scratch.values[1450] != 0.0))) {
            scratch.store_ad(1359, &AdValue::powf(AdValue::abs(AdValue::scale(scratch.ad_value(1358), scratch.values[499])), scratch.values[409]));
        }

        if (((((scratch.values[1323] != 0.0) && (scratch.values[1340] != 0.0)) && (!(scratch.values[1433] != 0.0))) && (!(scratch.values[1448] != 0.0))) && (scratch.values[1449] != 0.0)) {
            scratch.store_ad(1385, &AdValue::div_from_scalar(1.0, AdValue::sub_from_scalar(1.0, scratch.ad_value(1359))));
        }

        if (((((scratch.values[1323] != 0.0) && (scratch.values[1340] != 0.0)) && (!(scratch.values[1433] != 0.0))) && (!(scratch.values[1448] != 0.0))) && (!(scratch.values[1449] != 0.0))) {
            scratch.store_ad(1385, &AdValue::offset(AdValue::scale(AdValue::offset(scratch.ad_value(1358), (scratch.values[493] * scratch.values[406])), scratch.values[502]), scratch.values[496]));
        }

        if (((scratch.values[1323] != 0.0) && (scratch.values[1340] != 0.0)) && (!(scratch.values[1433] != 0.0))) {
            scratch.store_ad(1388, &AdValue::mul(AdValue::add(AdValue::add(AdValue::add(scratch.ad_value(1360), scratch.ad_value(1361)), scratch.ad_value(1368)), scratch.ad_value(1383)), scratch.ad_value(1385)));
        }

        if ((scratch.values[1323] != 0.0) && (scratch.values[1340] != 0.0)) {
            scratch.store_ad(506, &AdValue::add(AdValue::add(AdValue::mul(scratch.ad_value(690), scratch.ad_value(1386)), AdValue::mul(scratch.ad_value(691), scratch.ad_value(1387))), AdValue::mul(scratch.ad_value(692), scratch.ad_value(1388))));
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

        scratch.values[1451] = if !(((scratch.values[690] == 0.0) && (scratch.values[691] == 0.0)) && (scratch.values[692] == 0.0)) { 1.0 } else { 0.0 };

        if (((scratch.values[1323] != 0.0) && (scratch.values[1340] != 0.0)) && (scratch.values[1451] != 0.0)) {
            scratch.store_ad(1344, &AdValue::mul(AdValue::scale(scratch.ad_value(701), 4.0), scratch.ad_value(701)));
        }

        if (((scratch.values[1323] != 0.0) && (scratch.values[1340] != 0.0)) && (scratch.values[1451] != 0.0)) {
            scratch.store_ad(1345, &AdValue::div(scratch.ad_value(701), scratch.ad_value(702)));
        }

        if (((scratch.values[1323] != 0.0) && (scratch.values[1340] != 0.0)) && (scratch.values[1451] != 0.0)) {
            scratch.store_ad(1346, &AdValue::add(scratch.ad_value(517), AdValue::mul(scratch.ad_value(701), scratch.ad_value(1345))));
        }

        if (((scratch.values[1323] != 0.0) && (scratch.values[1340] != 0.0)) && (scratch.values[1451] != 0.0)) {
            scratch.store_ad(1347, &AdValue::add(scratch.ad_value(702), scratch.ad_value(1346)));
        }

        if (((scratch.values[1323] != 0.0) && (scratch.values[1340] != 0.0)) && (scratch.values[1451] != 0.0)) {
            scratch.store_ad(1348, &AdValue::sub(scratch.ad_value(702), scratch.ad_value(1346)));
        }

        if (((scratch.values[1323] != 0.0) && (scratch.values[1340] != 0.0)) && (scratch.values[1451] != 0.0)) {
            scratch.store_ad(1349, &AdValue::sqrt(AdValue::add(AdValue::square(scratch.ad_value(1348)), scratch.ad_value(1344))));
        }

        if (((scratch.values[1323] != 0.0) && (scratch.values[1340] != 0.0)) && (scratch.values[1451] != 0.0)) {
            scratch.store_ad(1351, &AdValue::scale(AdValue::div(AdValue::mul(scratch.ad_value(517), scratch.ad_value(702)), AdValue::add(scratch.ad_value(1347), scratch.ad_value(1349))), 2.0));
        }

        scratch.values[1452] = if (scratch.values[517] < scratch.values[698]) { 1.0 } else { 0.0 };

        scratch.values[1453] = if ((((0.5 * (scratch.values[517] * scratch.values[420]))) as f64).abs() < 230.25850929940458) { 1.0 } else { 0.0 };

        if (((((scratch.values[1323] != 0.0) && (scratch.values[1340] != 0.0)) && (scratch.values[1451] != 0.0)) && (scratch.values[1452] != 0.0)) && (scratch.values[1453] != 0.0)) {
            scratch.store_ad(1353, &AdValue::exp(AdValue::scale(scratch.ad_value(517), (scratch.values[420] * 0.5))));
        }

        scratch.values[1454] = if ((0.5 * (scratch.values[517] * scratch.values[420])) < 0.0) { 1.0 } else { 0.0 };

        if ((((((scratch.values[1323] != 0.0) && (scratch.values[1340] != 0.0)) && (scratch.values[1451] != 0.0)) && (scratch.values[1452] != 0.0)) && (!(scratch.values[1453] != 0.0))) && (scratch.values[1454] != 0.0)) {
            let assign16480_ad_e14637: AdValue = AdValue::div_from_scalar(1e-100, AdValue::offset(AdValue::mul(AdValue::sub_from_scalar((-230.25850929940458), AdValue::scale(scratch.ad_value(517), (scratch.values[420] * 0.5))), AdValue::offset(AdValue::scale(AdValue::mul(AdValue::sub_from_scalar((-230.25850929940458), AdValue::scale(scratch.ad_value(517), (scratch.values[420] * 0.5))), AdValue::offset(AdValue::scale(AdValue::sub_from_scalar((-230.25850929940458), AdValue::scale(scratch.ad_value(517), (scratch.values[420] * 0.5))), 0.3333333333333333), 1.0)), 0.5), 1.0)), 1.0));
            scratch.store_ad(1353, &assign16480_ad_e14637);
        }

        if ((((((scratch.values[1323] != 0.0) && (scratch.values[1340] != 0.0)) && (scratch.values[1451] != 0.0)) && (scratch.values[1452] != 0.0)) && (!(scratch.values[1453] != 0.0))) && (!(scratch.values[1454] != 0.0))) {
            scratch.store_ad(1353, &AdValue::scale(AdValue::offset(AdValue::mul(AdValue::offset(AdValue::scale(scratch.ad_value(517), (scratch.values[420] * 0.5)), (-230.25850929940458)), AdValue::offset(AdValue::scale(AdValue::mul(AdValue::offset(AdValue::scale(scratch.ad_value(517), (scratch.values[420] * 0.5)), (-230.25850929940458)), AdValue::offset(AdValue::scale(AdValue::offset(AdValue::scale(scratch.ad_value(517), (scratch.values[420] * 0.5)), (-230.25850929940458)), 0.3333333333333333), 1.0)), 0.5), 1.0)), 1.0), 1e100));
        }

        if ((((scratch.values[1323] != 0.0) && (scratch.values[1340] != 0.0)) && (scratch.values[1451] != 0.0)) && (scratch.values[1452] != 0.0)) {
            scratch.store_ad(1350, &AdValue::square(scratch.ad_value(1353)));
        }

        if ((((scratch.values[1323] != 0.0) && (scratch.values[1340] != 0.0)) && (scratch.values[1451] != 0.0)) && (!(scratch.values[1452] != 0.0))) {
            scratch.store_ad(1350, &AdValue::mul(AdValue::offset(AdValue::scale(AdValue::sub(scratch.ad_value(517), scratch.ad_value(698)), scratch.values[420]), 1.0), scratch.ad_value(699)));
        }

        if ((((scratch.values[1323] != 0.0) && (scratch.values[1340] != 0.0)) && (scratch.values[1451] != 0.0)) && (!(scratch.values[1452] != 0.0))) {
            scratch.store_ad(1353, &AdValue::sqrt(scratch.ad_value(1350)));
        }

        if (((scratch.values[1323] != 0.0) && (scratch.values[1340] != 0.0)) && (scratch.values[1451] != 0.0)) {
            scratch.store_ad(1350, &AdValue::offset(scratch.ad_value(1350), (-1.0)));
        }

        if (((scratch.values[1323] != 0.0) && (scratch.values[1340] != 0.0)) && (scratch.values[1451] != 0.0)) {
            scratch.store_ad(1352, &AdValue::div_from_scalar(1.0, scratch.ad_value(1353)));
        }

        scratch.values[1455] = if (scratch.values[517] > 0.0) { 1.0 } else { 0.0 };

        if ((((scratch.values[1323] != 0.0) && (scratch.values[1340] != 0.0)) && (scratch.values[1451] != 0.0)) && (scratch.values[1455] != 0.0)) {
            scratch.store_ad(1354, &AdValue::scale(AdValue::ln(AdValue::add(AdValue::offset(scratch.ad_value(1352), 2.0), AdValue::sqrt(AdValue::mul(AdValue::offset(scratch.ad_value(1352), 1.0), AdValue::offset(scratch.ad_value(1352), 3.0))))), (scratch.values[419] * 2.0)));
        }

        if ((((scratch.values[1323] != 0.0) && (scratch.values[1340] != 0.0)) && (scratch.values[1451] != 0.0)) && (!(scratch.values[1455] != 0.0))) {
            scratch.store_ad(1354, &AdValue::sub(AdValue::scale(AdValue::ln(AdValue::add(AdValue::offset(AdValue::scale(scratch.ad_value(1353), 2.0), 1.0), AdValue::sqrt(AdValue::mul(AdValue::offset(scratch.ad_value(1353), 1.0), AdValue::offset(AdValue::scale(scratch.ad_value(1353), 3.0), 1.0))))), (scratch.values[419] * 2.0)), scratch.ad_value(517)));
        }

        if (((scratch.values[1323] != 0.0) && (scratch.values[1340] != 0.0)) && (scratch.values[1451] != 0.0)) {
            scratch.store_ad(1355, &AdValue::sub(scratch.ad_value(700), scratch.ad_value(1354)));
        }

        if (((scratch.values[1323] != 0.0) && (scratch.values[1340] != 0.0)) && (scratch.values[1451] != 0.0)) {
            scratch.store_ad(1356, &AdValue::scale(AdValue::sub(AdValue::add(scratch.ad_value(517), scratch.ad_value(1355)), AdValue::sqrt(AdValue::offset(AdValue::mul(AdValue::sub(scratch.ad_value(517), scratch.ad_value(1355)), AdValue::sub(scratch.ad_value(517), scratch.ad_value(1355))), ((4.0 * scratch.values[419]) * scratch.values[419])))), 0.5));
        }

        if (((scratch.values[1323] != 0.0) && (scratch.values[1340] != 0.0)) && (scratch.values[1451] != 0.0)) {
            scratch.store_ad(1357, &AdValue::scale(AdValue::sub(AdValue::add(scratch.ad_value(517), scratch.ad_value(703)), AdValue::sqrt(AdValue::offset(AdValue::mul(AdValue::sub(scratch.ad_value(517), scratch.ad_value(703)), AdValue::sub(scratch.ad_value(517), scratch.ad_value(703))), ((4.0 * scratch.values[417]) * scratch.values[417])))), 0.5));
        }

        if (((scratch.values[1323] != 0.0) && (scratch.values[1340] != 0.0)) && (scratch.values[1451] != 0.0)) {
            scratch.store_ad(1358, &AdValue::scale(AdValue::sub(scratch.ad_value(517), AdValue::sqrt(AdValue::offset(AdValue::mul(scratch.ad_value(517), scratch.ad_value(517)), ((4.0 * 1e-6) * 1e-6)))), 0.5));
        }

        scratch.values[1456] = if (scratch.values[690] == 0.0) { 1.0 } else { 0.0 };

        if (((scratch.values[1323] != 0.0) && (scratch.values[1340] != 0.0)) && (scratch.values[1456] != 0.0)) {
            scratch.values[1386] = 0.0;
            scratch.node_derivatives[1386] = [0.0; Instance::NODE_COUNT];
            scratch.branch_derivatives[1386] = [0.0; Instance::BRANCH_COUNT];
        }

        scratch.values[1457] = if (scratch.values[457] == 0.5) { 1.0 } else { 0.0 };

        if ((((scratch.values[1323] != 0.0) && (scratch.values[1340] != 0.0)) && (!(scratch.values[1456] != 0.0))) && (scratch.values[1457] != 0.0)) {
            scratch.store_ad(1359, &AdValue::sqrt(AdValue::sub_from_scalar(1.0, AdValue::scale(scratch.ad_value(1351), scratch.values[454]))));
        }

        if ((((scratch.values[1323] != 0.0) && (scratch.values[1340] != 0.0)) && (!(scratch.values[1456] != 0.0))) && (!(scratch.values[1457] != 0.0))) {
            scratch.store_ad(1359, &AdValue::powf(AdValue::sub_from_scalar(1.0, AdValue::scale(scratch.ad_value(1351), scratch.values[454])), scratch.values[457]));
        }

        if (((scratch.values[1323] != 0.0) && (scratch.values[1340] != 0.0)) && (!(scratch.values[1456] != 0.0))) {
            scratch.store_ad(1360, &AdValue::scale(scratch.ad_value(1350), scratch.values[436]));
        }

        scratch.values[1458] = if ((scratch.values[386] == 0.0) && (scratch.values[389] == 0.0)) { 1.0 } else { 0.0 };

        if ((((scratch.values[1323] != 0.0) && (scratch.values[1340] != 0.0)) && (!(scratch.values[1456] != 0.0))) && (scratch.values[1458] != 0.0)) {
            scratch.values[1361] = 0.0;
            scratch.node_derivatives[1361] = [0.0; Instance::NODE_COUNT];
            scratch.branch_derivatives[1361] = [0.0; Instance::BRANCH_COUNT];
        }

        if ((((scratch.values[1323] != 0.0) && (scratch.values[1340] != 0.0)) && (!(scratch.values[1456] != 0.0))) && (!(scratch.values[1458] != 0.0))) {
            scratch.store_ad(1362, &AdValue::sub_from_scalar(scratch.values[442], scratch.ad_value(1356)));
        }

        if ((((scratch.values[1323] != 0.0) && (scratch.values[1340] != 0.0)) && (!(scratch.values[1456] != 0.0))) && (!(scratch.values[1458] != 0.0))) {
            scratch.store_ad(1363, &AdValue::sub_from_scalar(1.0, AdValue::sqrt(AdValue::sub_from_scalar(1.0, AdValue::div(scratch.ad_value(1354), scratch.ad_value(1362))))));
        }

        scratch.values[1459] = if (scratch.values[375] == 0.5) { 1.0 } else { 0.0 };

        if (((((scratch.values[1323] != 0.0) && (scratch.values[1340] != 0.0)) && (!(scratch.values[1456] != 0.0))) && (!(scratch.values[1458] != 0.0))) && (scratch.values[1459] != 0.0)) {
            scratch.values[1364] = 0.0;
            scratch.node_derivatives[1364] = [0.0; Instance::NODE_COUNT];
            scratch.branch_derivatives[1364] = [0.0; Instance::BRANCH_COUNT];
        }

        if (((((scratch.values[1323] != 0.0) && (scratch.values[1340] != 0.0)) && (!(scratch.values[1456] != 0.0))) && (!(scratch.values[1458] != 0.0))) && (!(scratch.values[1459] != 0.0))) {
            scratch.store_ad(1364, &AdValue::scale(AdValue::add(AdValue::div(AdValue::mul(AdValue::square(scratch.ad_value(1363)), AdValue::ln(scratch.ad_value(1363))), AdValue::sub_from_scalar(1.0, scratch.ad_value(1363))), scratch.ad_value(1363)), (1.0 - (2.0 * scratch.values[375]))));
        }

        if ((((scratch.values[1323] != 0.0) && (scratch.values[1340] != 0.0)) && (!(scratch.values[1456] != 0.0))) && (!(scratch.values[1458] != 0.0))) {
            scratch.store_ad(1365, &AdValue::add(scratch.ad_value(1363), scratch.ad_value(1364)));
        }

        scratch.values[1460] = if (scratch.values[375] == 0.5) { 1.0 } else { 0.0 };

        if (((((scratch.values[1323] != 0.0) && (scratch.values[1340] != 0.0)) && (!(scratch.values[1456] != 0.0))) && (!(scratch.values[1458] != 0.0))) && (scratch.values[1460] != 0.0)) {
            scratch.store_ad(1359, &AdValue::sqrt(AdValue::scale(scratch.ad_value(1362), scratch.values[478])));
        }

        if (((((scratch.values[1323] != 0.0) && (scratch.values[1340] != 0.0)) && (!(scratch.values[1456] != 0.0))) && (!(scratch.values[1458] != 0.0))) && (!(scratch.values[1460] != 0.0))) {
            scratch.store_ad(1359, &AdValue::powf(AdValue::scale(scratch.ad_value(1362), scratch.values[478]), scratch.values[375]));
        }

        if ((((scratch.values[1323] != 0.0) && (scratch.values[1340] != 0.0)) && (!(scratch.values[1456] != 0.0))) && (!(scratch.values[1458] != 0.0))) {
            scratch.store_ad(1366, &AdValue::scale(scratch.ad_value(1359), scratch.values[472]));
        }

        if ((((scratch.values[1323] != 0.0) && (scratch.values[1340] != 0.0)) && (!(scratch.values[1456] != 0.0))) && (!(scratch.values[1458] != 0.0))) {
            scratch.store_ad(1367, &AdValue::scale(AdValue::mul(AdValue::offset(scratch.ad_value(1353), (-1.0)), scratch.ad_value(1366)), scratch.values[433]));
        }

        if ((((scratch.values[1323] != 0.0) && (scratch.values[1340] != 0.0)) && (!(scratch.values[1456] != 0.0))) && (!(scratch.values[1458] != 0.0))) {
            scratch.store_ad(1361, &AdValue::scale(AdValue::mul(scratch.ad_value(1367), scratch.ad_value(1365)), scratch.values[386]));
        }

        scratch.values[1461] = if (scratch.values[389] == 0.0) { 1.0 } else { 0.0 };

        if ((((scratch.values[1323] != 0.0) && (scratch.values[1340] != 0.0)) && (!(scratch.values[1456] != 0.0))) && (scratch.values[1461] != 0.0)) {
            scratch.values[1368] = 0.0;
            scratch.node_derivatives[1368] = [0.0; Instance::NODE_COUNT];
            scratch.branch_derivatives[1368] = [0.0; Instance::BRANCH_COUNT];
        }

        if ((((scratch.values[1323] != 0.0) && (scratch.values[1340] != 0.0)) && (!(scratch.values[1456] != 0.0))) && (!(scratch.values[1461] != 0.0))) {
            scratch.store_ad(1369, &AdValue::scale(AdValue::div(AdValue::scale(scratch.ad_value(1366), scratch.values[457]), scratch.ad_value(1362)), scratch.values[487]));
        }

        if ((((scratch.values[1323] != 0.0) && (scratch.values[1340] != 0.0)) && (!(scratch.values[1456] != 0.0))) && (!(scratch.values[1461] != 0.0))) {
            scratch.store_ad(1370, &AdValue::div_from_scalar((0.666666666666667 * scratch.values[484]), scratch.ad_value(1369)));
        }

        if ((((scratch.values[1323] != 0.0) && (scratch.values[1340] != 0.0)) && (!(scratch.values[1456] != 0.0))) && (!(scratch.values[1461] != 0.0))) {
            scratch.store_ad(1371, &AdValue::square(scratch.ad_value(1370)));
        }

        if ((((scratch.values[1323] != 0.0) && (scratch.values[1340] != 0.0)) && (!(scratch.values[1456] != 0.0))) && (!(scratch.values[1461] != 0.0))) {
            scratch.store_ad(1372, &AdValue::sqrt(AdValue::div(AdValue::square(scratch.ad_value(1371)), AdValue::offset(AdValue::square(scratch.ad_value(1371)), 1.0))));
        }

        if ((((scratch.values[1323] != 0.0) && (scratch.values[1340] != 0.0)) && (!(scratch.values[1456] != 0.0))) && (!(scratch.values[1461] != 0.0))) {
            scratch.store_ad(1373, &AdValue::sqrt(AdValue::abs(scratch.ad_value(1372))));
        }

        if ((((scratch.values[1323] != 0.0) && (scratch.values[1340] != 0.0)) && (!(scratch.values[1456] != 0.0))) && (!(scratch.values[1461] != 0.0))) {
            scratch.store_ad(1374, &AdValue::mul(scratch.ad_value(1372), scratch.ad_value(1373)));
        }

        scratch.values[1462] = if (((-scratch.values[375]) * scratch.values[460]) == (-1.0)) { 1.0 } else { 0.0 };

        if (((((scratch.values[1323] != 0.0) && (scratch.values[1340] != 0.0)) && (!(scratch.values[1456] != 0.0))) && (!(scratch.values[1461] != 0.0))) && (scratch.values[1462] != 0.0)) {
            scratch.store_ad(1375, &AdValue::div_from_scalar(1.0, AdValue::offset(AdValue::mul(scratch.ad_value(1369), scratch.ad_value(1374)), 1.0)));
        }

        if (((((scratch.values[1323] != 0.0) && (scratch.values[1340] != 0.0)) && (!(scratch.values[1456] != 0.0))) && (!(scratch.values[1461] != 0.0))) && (!(scratch.values[1462] != 0.0))) {
            scratch.store_ad(1375, &AdValue::powf(AdValue::offset(AdValue::mul(scratch.ad_value(1369), scratch.ad_value(1374)), 1.0), ((-scratch.values[375]) * scratch.values[460])));
        }

        if ((((scratch.values[1323] != 0.0) && (scratch.values[1340] != 0.0)) && (!(scratch.values[1456] != 0.0))) && (!(scratch.values[1461] != 0.0))) {
            scratch.store_ad(1376, &AdValue::div(AdValue::mul(scratch.ad_value(1365), scratch.ad_value(1375)), AdValue::add(scratch.ad_value(1365), scratch.ad_value(1375))));
        }

        if ((((scratch.values[1323] != 0.0) && (scratch.values[1340] != 0.0)) && (!(scratch.values[1456] != 0.0))) && (!(scratch.values[1461] != 0.0))) {
            scratch.store_ad(1377, &AdValue::sqrt(AdValue::scale(AdValue::div(scratch.ad_value(1369), scratch.ad_value(1373)), 0.375)));
        }

        if ((((scratch.values[1323] != 0.0) && (scratch.values[1340] != 0.0)) && (!(scratch.values[1456] != 0.0))) && (!(scratch.values[1461] != 0.0))) {
            scratch.store_ad(1378, &AdValue::sub(AdValue::scale(AdValue::mul(scratch.ad_value(1370), scratch.ad_value(1373)), 2.0), scratch.ad_value(1372)));
        }

        if ((((scratch.values[1323] != 0.0) && (scratch.values[1340] != 0.0)) && (!(scratch.values[1456] != 0.0))) && (!(scratch.values[1461] != 0.0))) {
            scratch.store_ad(1379, &AdValue::add(AdValue::sub(AdValue::mul(AdValue::scale(scratch.ad_value(1370), scratch.values[484]), scratch.ad_value(1373)), AdValue::scale(scratch.ad_value(1372), scratch.values[484])), AdValue::scale(AdValue::mul(scratch.ad_value(1369), scratch.ad_value(1374)), 0.5)));
        }

        if ((((scratch.values[1323] != 0.0) && (scratch.values[1340] != 0.0)) && (!(scratch.values[1456] != 0.0))) && (!(scratch.values[1461] != 0.0))) {
            scratch.store_ad(1380, &AdValue::mul(AdValue::offset(scratch.ad_value(1378), (-1.0)), scratch.ad_value(1377)));
        }

        if ((((scratch.values[1323] != 0.0) && (scratch.values[1340] != 0.0)) && (!(scratch.values[1456] != 0.0))) && (!(scratch.values[1461] != 0.0))) {
            scratch.store_ad(1341, &AdValue::square(scratch.ad_value(1380)));
        }

        scratch.values[1463] = if (scratch.values[1380] > 0.0) { 1.0 } else { 0.0 };

        if (((((scratch.values[1323] != 0.0) && (scratch.values[1340] != 0.0)) && (!(scratch.values[1456] != 0.0))) && (!(scratch.values[1461] != 0.0))) && (scratch.values[1463] != 0.0)) {
            scratch.store_ad(1342, &AdValue::div_from_scalar(1.0, AdValue::offset(AdValue::scale(scratch.ad_value(1380), scratch.values[421]), 1.0)));
        }

        if (((((scratch.values[1323] != 0.0) && (scratch.values[1340] != 0.0)) && (!(scratch.values[1456] != 0.0))) && (!(scratch.values[1461] != 0.0))) && (!(scratch.values[1463] != 0.0))) {
            scratch.store_ad(1342, &AdValue::div_from_scalar(1.0, AdValue::sub_from_scalar(1.0, AdValue::scale(scratch.ad_value(1380), scratch.values[421]))));
        }

        scratch.values[1464] = if (((-scratch.values[1341]) + scratch.values[1379]) > (-230.25850929940458)) { 1.0 } else { 0.0 };

        if (((((scratch.values[1323] != 0.0) && (scratch.values[1340] != 0.0)) && (!(scratch.values[1456] != 0.0))) && (!(scratch.values[1461] != 0.0))) && (scratch.values[1464] != 0.0)) {
            scratch.store_ad(1359, &AdValue::exp(AdValue::sub(scratch.ad_value(1379), scratch.ad_value(1341))));
        }

        if (((((scratch.values[1323] != 0.0) && (scratch.values[1340] != 0.0)) && (!(scratch.values[1456] != 0.0))) && (!(scratch.values[1461] != 0.0))) && (!(scratch.values[1464] != 0.0))) {
            let assign17060_ad_e15580: AdValue = AdValue::div_from_scalar(1e-100, AdValue::offset(AdValue::mul(AdValue::sub_from_scalar((-230.25850929940458), AdValue::sub(scratch.ad_value(1379), scratch.ad_value(1341))), AdValue::offset(AdValue::scale(AdValue::mul(AdValue::sub_from_scalar((-230.25850929940458), AdValue::sub(scratch.ad_value(1379), scratch.ad_value(1341))), AdValue::offset(AdValue::scale(AdValue::sub_from_scalar((-230.25850929940458), AdValue::sub(scratch.ad_value(1379), scratch.ad_value(1341))), 0.3333333333333333), 1.0)), 0.5), 1.0)), 1.0));
            scratch.store_ad(1359, &assign17060_ad_e15580);
        }

        if ((((scratch.values[1323] != 0.0) && (scratch.values[1340] != 0.0)) && (!(scratch.values[1456] != 0.0))) && (!(scratch.values[1461] != 0.0))) {
            scratch.store_ad(1343, &AdValue::mul(AdValue::add(AdValue::add(AdValue::scale(scratch.ad_value(1342), 0.29214664), AdValue::scale(AdValue::square(scratch.ad_value(1342)), scratch.values[422])), AdValue::scale(AdValue::mul(AdValue::square(scratch.ad_value(1342)), scratch.ad_value(1342)), scratch.values[423])), scratch.ad_value(1359)));
        }

        scratch.values[1465] = if (scratch.values[1380] > 0.0) { 1.0 } else { 0.0 };

        if (((((scratch.values[1323] != 0.0) && (scratch.values[1340] != 0.0)) && (!(scratch.values[1456] != 0.0))) && (!(scratch.values[1461] != 0.0))) && (scratch.values[1465] != 0.0)) {
            scratch.values[1381] = scratch.values[1343];
            scratch.node_derivatives[1381] = scratch.node_derivatives[1343];
            scratch.branch_derivatives[1381] = scratch.branch_derivatives[1343];
        }

        scratch.values[1466] = if (scratch.values[1379] > (-230.25850929940458)) { 1.0 } else { 0.0 };

        if ((((((scratch.values[1323] != 0.0) && (scratch.values[1340] != 0.0)) && (!(scratch.values[1456] != 0.0))) && (!(scratch.values[1461] != 0.0))) && (!(scratch.values[1465] != 0.0))) && (scratch.values[1466] != 0.0)) {
            scratch.store_ad(1359, &AdValue::exp(scratch.ad_value(1379)));
        }

        if ((((((scratch.values[1323] != 0.0) && (scratch.values[1340] != 0.0)) && (!(scratch.values[1456] != 0.0))) && (!(scratch.values[1461] != 0.0))) && (!(scratch.values[1465] != 0.0))) && (!(scratch.values[1466] != 0.0))) {
            scratch.store_ad(1359, &AdValue::div_from_scalar(1e-100, AdValue::offset(AdValue::mul(AdValue::sub_from_scalar((-230.25850929940458), scratch.ad_value(1379)), AdValue::offset(AdValue::scale(AdValue::mul(AdValue::sub_from_scalar((-230.25850929940458), scratch.ad_value(1379)), AdValue::offset(AdValue::scale(AdValue::sub_from_scalar((-230.25850929940458), scratch.ad_value(1379)), 0.3333333333333333), 1.0)), 0.5), 1.0)), 1.0)));
        }

        if (((((scratch.values[1323] != 0.0) && (scratch.values[1340] != 0.0)) && (!(scratch.values[1456] != 0.0))) && (!(scratch.values[1461] != 0.0))) && (!(scratch.values[1465] != 0.0))) {
            scratch.store_ad(1381, &AdValue::sub(AdValue::scale(scratch.ad_value(1359), 2.0), scratch.ad_value(1343)));
        }

        if ((((scratch.values[1323] != 0.0) && (scratch.values[1340] != 0.0)) && (!(scratch.values[1456] != 0.0))) && (!(scratch.values[1461] != 0.0))) {
            scratch.store_ad(1382, &AdValue::scale(AdValue::div(AdValue::scale(scratch.ad_value(1381), scratch.values[484]), scratch.ad_value(1377)), (1.772453850905516 * 0.5)));
        }

        if ((((scratch.values[1323] != 0.0) && (scratch.values[1340] != 0.0)) && (!(scratch.values[1456] != 0.0))) && (!(scratch.values[1461] != 0.0))) {
            scratch.store_ad(1368, &AdValue::scale(AdValue::mul(AdValue::mul(scratch.ad_value(1367), scratch.ad_value(1382)), scratch.ad_value(1376)), scratch.values[389]));
        }

        scratch.values[1467] = if (scratch.values[395] == 0.0) { 1.0 } else { 0.0 };

        if ((((scratch.values[1323] != 0.0) && (scratch.values[1340] != 0.0)) && (!(scratch.values[1456] != 0.0))) && (scratch.values[1467] != 0.0)) {
            scratch.values[1383] = 0.0;
            scratch.node_derivatives[1383] = [0.0; Instance::NODE_COUNT];
            scratch.branch_derivatives[1383] = [0.0; Instance::BRANCH_COUNT];
        }

        scratch.values[1468] = if (scratch.values[375] == 0.5) { 1.0 } else { 0.0 };

        if (((((scratch.values[1323] != 0.0) && (scratch.values[1340] != 0.0)) && (!(scratch.values[1456] != 0.0))) && (!(scratch.values[1467] != 0.0))) && (scratch.values[1468] != 0.0)) {
            scratch.store_ad(1359, &AdValue::sqrt(AdValue::scale(AdValue::sub_from_scalar(scratch.values[372], scratch.ad_value(1357)), scratch.values[478])));
        }

        if (((((scratch.values[1323] != 0.0) && (scratch.values[1340] != 0.0)) && (!(scratch.values[1456] != 0.0))) && (!(scratch.values[1467] != 0.0))) && (!(scratch.values[1468] != 0.0))) {
            scratch.store_ad(1359, &AdValue::powf(AdValue::scale(AdValue::sub_from_scalar(scratch.values[372], scratch.ad_value(1357)), scratch.values[478]), scratch.values[375]));
        }

        if ((((scratch.values[1323] != 0.0) && (scratch.values[1340] != 0.0)) && (!(scratch.values[1456] != 0.0))) && (!(scratch.values[1467] != 0.0))) {
            scratch.store_ad(1384, &AdValue::scale(AdValue::div(AdValue::scale(AdValue::sub_from_scalar(scratch.values[372], scratch.ad_value(1357)), scratch.values[475]), scratch.ad_value(1359)), scratch.values[460]));
        }

        scratch.values[1469] = if (((((-scratch.values[490]) / scratch.values[1384])) as f64).abs() < 230.25850929940458) { 1.0 } else { 0.0 };

        if (((((scratch.values[1323] != 0.0) && (scratch.values[1340] != 0.0)) && (!(scratch.values[1456] != 0.0))) && (!(scratch.values[1467] != 0.0))) && (scratch.values[1469] != 0.0)) {
            scratch.store_ad(1359, &AdValue::exp(AdValue::div(AdValue::neg(scratch.ad_value(490)), scratch.ad_value(1384))));
        }

        scratch.values[1470] = if (((-scratch.values[490]) / scratch.values[1384]) < 0.0) { 1.0 } else { 0.0 };

        if ((((((scratch.values[1323] != 0.0) && (scratch.values[1340] != 0.0)) && (!(scratch.values[1456] != 0.0))) && (!(scratch.values[1467] != 0.0))) && (!(scratch.values[1469] != 0.0))) && (scratch.values[1470] != 0.0)) {
            let assign17250_ad_e15907: AdValue = AdValue::offset(AdValue::mul(AdValue::sub_from_scalar((-230.25850929940458), AdValue::div(AdValue::neg(scratch.ad_value(490)), scratch.ad_value(1384))), AdValue::offset(AdValue::scale(AdValue::mul(AdValue::sub_from_scalar((-230.25850929940458), AdValue::div(AdValue::neg(scratch.ad_value(490)), scratch.ad_value(1384))), AdValue::offset(AdValue::scale(AdValue::sub_from_scalar((-230.25850929940458), AdValue::div(AdValue::neg(scratch.ad_value(490)), scratch.ad_value(1384))), 0.3333333333333333), 1.0)), 0.5), 1.0)), 1.0);
            scratch.store_ad(1359, &AdValue::div_from_scalar(1e-100, assign17250_ad_e15907));
        }

        if ((((((scratch.values[1323] != 0.0) && (scratch.values[1340] != 0.0)) && (!(scratch.values[1456] != 0.0))) && (!(scratch.values[1467] != 0.0))) && (!(scratch.values[1469] != 0.0))) && (!(scratch.values[1470] != 0.0))) {
            let assign17260_ad_e15957: AdValue = AdValue::scale(AdValue::offset(AdValue::mul(AdValue::offset(AdValue::div(AdValue::neg(scratch.ad_value(490)), scratch.ad_value(1384)), (-230.25850929940458)), AdValue::offset(AdValue::scale(AdValue::mul(AdValue::offset(AdValue::div(AdValue::neg(scratch.ad_value(490)), scratch.ad_value(1384)), (-230.25850929940458)), AdValue::offset(AdValue::scale(AdValue::offset(AdValue::div(AdValue::neg(scratch.ad_value(490)), scratch.ad_value(1384)), (-230.25850929940458)), 0.3333333333333333), 1.0)), 0.5), 1.0)), 1.0), 1e100);
            scratch.store_ad(1359, &assign17260_ad_e15957);
        }

        if ((((scratch.values[1323] != 0.0) && (scratch.values[1340] != 0.0)) && (!(scratch.values[1456] != 0.0))) && (!(scratch.values[1467] != 0.0))) {
            scratch.store_ad(1383, &AdValue::scale(AdValue::mul(AdValue::mul(AdValue::mul(scratch.ad_value(517), scratch.ad_value(1384)), scratch.ad_value(1384)), scratch.ad_value(1359)), scratch.values[395]));
        }

        scratch.values[1471] = if (scratch.values[404] > 1000.0) { 1.0 } else { 0.0 };

        if ((((scratch.values[1323] != 0.0) && (scratch.values[1340] != 0.0)) && (!(scratch.values[1456] != 0.0))) && (scratch.values[1471] != 0.0)) {
            scratch.values[1385] = 1.0;
            scratch.node_derivatives[1385] = [0.0; Instance::NODE_COUNT];
            scratch.branch_derivatives[1385] = [0.0; Instance::BRANCH_COUNT];
        }

        scratch.values[1472] = if (scratch.values[1358] > ((-scratch.values[493]) * scratch.values[404])) { 1.0 } else { 0.0 };

        scratch.values[1473] = if (scratch.values[407] == 4.0) { 1.0 } else { 0.0 };

        if ((((((scratch.values[1323] != 0.0) && (scratch.values[1340] != 0.0)) && (!(scratch.values[1456] != 0.0))) && (!(scratch.values[1471] != 0.0))) && (scratch.values[1472] != 0.0)) && (scratch.values[1473] != 0.0)) {
            scratch.store_ad(1359, &AdValue::mul(AdValue::mul(AdValue::mul(AdValue::scale(scratch.ad_value(1358), scratch.values[497]), AdValue::scale(scratch.ad_value(1358), scratch.values[497])), AdValue::scale(scratch.ad_value(1358), scratch.values[497])), AdValue::scale(scratch.ad_value(1358), scratch.values[497])));
        }

        if ((((((scratch.values[1323] != 0.0) && (scratch.values[1340] != 0.0)) && (!(scratch.values[1456] != 0.0))) && (!(scratch.values[1471] != 0.0))) && (scratch.values[1472] != 0.0)) && (!(scratch.values[1473] != 0.0))) {
            scratch.store_ad(1359, &AdValue::powf(AdValue::abs(AdValue::scale(scratch.ad_value(1358), scratch.values[497])), scratch.values[407]));
        }

        if (((((scratch.values[1323] != 0.0) && (scratch.values[1340] != 0.0)) && (!(scratch.values[1456] != 0.0))) && (!(scratch.values[1471] != 0.0))) && (scratch.values[1472] != 0.0)) {
            scratch.store_ad(1385, &AdValue::div_from_scalar(1.0, AdValue::sub_from_scalar(1.0, scratch.ad_value(1359))));
        }

        if (((((scratch.values[1323] != 0.0) && (scratch.values[1340] != 0.0)) && (!(scratch.values[1456] != 0.0))) && (!(scratch.values[1471] != 0.0))) && (!(scratch.values[1472] != 0.0))) {
            scratch.store_ad(1385, &AdValue::offset(AdValue::scale(AdValue::offset(scratch.ad_value(1358), (scratch.values[493] * scratch.values[404])), scratch.values[500]), scratch.values[494]));
        }

        if (((scratch.values[1323] != 0.0) && (scratch.values[1340] != 0.0)) && (!(scratch.values[1456] != 0.0))) {
            scratch.store_ad(1386, &AdValue::mul(AdValue::add(AdValue::add(AdValue::add(scratch.ad_value(1360), scratch.ad_value(1361)), scratch.ad_value(1368)), scratch.ad_value(1383)), scratch.ad_value(1385)));
        }

        scratch.values[1474] = if (scratch.values[691] == 0.0) { 1.0 } else { 0.0 };

        if (((scratch.values[1323] != 0.0) && (scratch.values[1340] != 0.0)) && (scratch.values[1474] != 0.0)) {
            scratch.values[1387] = 0.0;
            scratch.node_derivatives[1387] = [0.0; Instance::NODE_COUNT];
            scratch.branch_derivatives[1387] = [0.0; Instance::BRANCH_COUNT];
        }

        scratch.values[1475] = if (scratch.values[458] == 0.5) { 1.0 } else { 0.0 };

        if ((((scratch.values[1323] != 0.0) && (scratch.values[1340] != 0.0)) && (!(scratch.values[1474] != 0.0))) && (scratch.values[1475] != 0.0)) {
            scratch.store_ad(1359, &AdValue::sqrt(AdValue::sub_from_scalar(1.0, AdValue::scale(scratch.ad_value(1351), scratch.values[455]))));
        }

        if ((((scratch.values[1323] != 0.0) && (scratch.values[1340] != 0.0)) && (!(scratch.values[1474] != 0.0))) && (!(scratch.values[1475] != 0.0))) {
            scratch.store_ad(1359, &AdValue::powf(AdValue::sub_from_scalar(1.0, AdValue::scale(scratch.ad_value(1351), scratch.values[455])), scratch.values[458]));
        }

        if (((scratch.values[1323] != 0.0) && (scratch.values[1340] != 0.0)) && (!(scratch.values[1474] != 0.0))) {
            scratch.store_ad(1360, &AdValue::scale(scratch.ad_value(1350), scratch.values[437]));
        }

        scratch.values[1476] = if ((scratch.values[387] == 0.0) && (scratch.values[390] == 0.0)) { 1.0 } else { 0.0 };

        if ((((scratch.values[1323] != 0.0) && (scratch.values[1340] != 0.0)) && (!(scratch.values[1474] != 0.0))) && (scratch.values[1476] != 0.0)) {
            scratch.values[1361] = 0.0;
            scratch.node_derivatives[1361] = [0.0; Instance::NODE_COUNT];
            scratch.branch_derivatives[1361] = [0.0; Instance::BRANCH_COUNT];
        }

        if ((((scratch.values[1323] != 0.0) && (scratch.values[1340] != 0.0)) && (!(scratch.values[1474] != 0.0))) && (!(scratch.values[1476] != 0.0))) {
            scratch.store_ad(1362, &AdValue::sub_from_scalar(scratch.values[443], scratch.ad_value(1356)));
        }

        if ((((scratch.values[1323] != 0.0) && (scratch.values[1340] != 0.0)) && (!(scratch.values[1474] != 0.0))) && (!(scratch.values[1476] != 0.0))) {
            scratch.store_ad(1363, &AdValue::sub_from_scalar(1.0, AdValue::sqrt(AdValue::sub_from_scalar(1.0, AdValue::div(scratch.ad_value(1354), scratch.ad_value(1362))))));
        }

        scratch.values[1477] = if (scratch.values[376] == 0.5) { 1.0 } else { 0.0 };

        if (((((scratch.values[1323] != 0.0) && (scratch.values[1340] != 0.0)) && (!(scratch.values[1474] != 0.0))) && (!(scratch.values[1476] != 0.0))) && (scratch.values[1477] != 0.0)) {
            scratch.values[1364] = 0.0;
            scratch.node_derivatives[1364] = [0.0; Instance::NODE_COUNT];
            scratch.branch_derivatives[1364] = [0.0; Instance::BRANCH_COUNT];
        }

        if (((((scratch.values[1323] != 0.0) && (scratch.values[1340] != 0.0)) && (!(scratch.values[1474] != 0.0))) && (!(scratch.values[1476] != 0.0))) && (!(scratch.values[1477] != 0.0))) {
            scratch.store_ad(1364, &AdValue::scale(AdValue::add(AdValue::div(AdValue::mul(AdValue::square(scratch.ad_value(1363)), AdValue::ln(scratch.ad_value(1363))), AdValue::sub_from_scalar(1.0, scratch.ad_value(1363))), scratch.ad_value(1363)), (1.0 - (2.0 * scratch.values[376]))));
        }

        if ((((scratch.values[1323] != 0.0) && (scratch.values[1340] != 0.0)) && (!(scratch.values[1474] != 0.0))) && (!(scratch.values[1476] != 0.0))) {
            scratch.store_ad(1365, &AdValue::add(scratch.ad_value(1363), scratch.ad_value(1364)));
        }

        scratch.values[1478] = if (scratch.values[376] == 0.5) { 1.0 } else { 0.0 };

        if (((((scratch.values[1323] != 0.0) && (scratch.values[1340] != 0.0)) && (!(scratch.values[1474] != 0.0))) && (!(scratch.values[1476] != 0.0))) && (scratch.values[1478] != 0.0)) {
            scratch.store_ad(1359, &AdValue::sqrt(AdValue::scale(scratch.ad_value(1362), scratch.values[479])));
        }

        if (((((scratch.values[1323] != 0.0) && (scratch.values[1340] != 0.0)) && (!(scratch.values[1474] != 0.0))) && (!(scratch.values[1476] != 0.0))) && (!(scratch.values[1478] != 0.0))) {
            scratch.store_ad(1359, &AdValue::powf(AdValue::scale(scratch.ad_value(1362), scratch.values[479]), scratch.values[376]));
        }

        if ((((scratch.values[1323] != 0.0) && (scratch.values[1340] != 0.0)) && (!(scratch.values[1474] != 0.0))) && (!(scratch.values[1476] != 0.0))) {
            scratch.store_ad(1366, &AdValue::scale(scratch.ad_value(1359), scratch.values[473]));
        }

        if ((((scratch.values[1323] != 0.0) && (scratch.values[1340] != 0.0)) && (!(scratch.values[1474] != 0.0))) && (!(scratch.values[1476] != 0.0))) {
            scratch.store_ad(1367, &AdValue::scale(AdValue::mul(AdValue::offset(scratch.ad_value(1353), (-1.0)), scratch.ad_value(1366)), scratch.values[434]));
        }

        if ((((scratch.values[1323] != 0.0) && (scratch.values[1340] != 0.0)) && (!(scratch.values[1474] != 0.0))) && (!(scratch.values[1476] != 0.0))) {
            scratch.store_ad(1361, &AdValue::scale(AdValue::mul(scratch.ad_value(1367), scratch.ad_value(1365)), scratch.values[387]));
        }

        scratch.values[1479] = if (scratch.values[390] == 0.0) { 1.0 } else { 0.0 };

        if ((((scratch.values[1323] != 0.0) && (scratch.values[1340] != 0.0)) && (!(scratch.values[1474] != 0.0))) && (scratch.values[1479] != 0.0)) {
            scratch.values[1368] = 0.0;
            scratch.node_derivatives[1368] = [0.0; Instance::NODE_COUNT];
            scratch.branch_derivatives[1368] = [0.0; Instance::BRANCH_COUNT];
        }

        if ((((scratch.values[1323] != 0.0) && (scratch.values[1340] != 0.0)) && (!(scratch.values[1474] != 0.0))) && (!(scratch.values[1479] != 0.0))) {
            scratch.store_ad(1369, &AdValue::scale(AdValue::div(AdValue::scale(scratch.ad_value(1366), scratch.values[458]), scratch.ad_value(1362)), scratch.values[488]));
        }

        if ((((scratch.values[1323] != 0.0) && (scratch.values[1340] != 0.0)) && (!(scratch.values[1474] != 0.0))) && (!(scratch.values[1479] != 0.0))) {
            scratch.store_ad(1370, &AdValue::div_from_scalar((0.666666666666667 * scratch.values[485]), scratch.ad_value(1369)));
        }

        if ((((scratch.values[1323] != 0.0) && (scratch.values[1340] != 0.0)) && (!(scratch.values[1474] != 0.0))) && (!(scratch.values[1479] != 0.0))) {
            scratch.store_ad(1371, &AdValue::square(scratch.ad_value(1370)));
        }

        if ((((scratch.values[1323] != 0.0) && (scratch.values[1340] != 0.0)) && (!(scratch.values[1474] != 0.0))) && (!(scratch.values[1479] != 0.0))) {
            scratch.store_ad(1372, &AdValue::sqrt(AdValue::div(AdValue::square(scratch.ad_value(1371)), AdValue::offset(AdValue::square(scratch.ad_value(1371)), 1.0))));
        }

        if ((((scratch.values[1323] != 0.0) && (scratch.values[1340] != 0.0)) && (!(scratch.values[1474] != 0.0))) && (!(scratch.values[1479] != 0.0))) {
            scratch.store_ad(1373, &AdValue::sqrt(AdValue::abs(scratch.ad_value(1372))));
        }

    }

    pub(super) fn stamp_transient_block_14(
        &mut self,
        ctx: &GeneratedEvalContext<'_>,
        stamper: &mut GeneratedStamper<'_>,
        scratch: &mut Scratch,
    ) {
        let _ = stamper;
        if ((((scratch.values[1323] != 0.0) && (scratch.values[1340] != 0.0)) && (!(scratch.values[1474] != 0.0))) && (!(scratch.values[1479] != 0.0))) {
            scratch.store_ad(1374, &AdValue::mul(scratch.ad_value(1372), scratch.ad_value(1373)));
        }

        scratch.values[1480] = if (((-scratch.values[376]) * scratch.values[461]) == (-1.0)) { 1.0 } else { 0.0 };

        if (((((scratch.values[1323] != 0.0) && (scratch.values[1340] != 0.0)) && (!(scratch.values[1474] != 0.0))) && (!(scratch.values[1479] != 0.0))) && (scratch.values[1480] != 0.0)) {
            scratch.store_ad(1375, &AdValue::div_from_scalar(1.0, AdValue::offset(AdValue::mul(scratch.ad_value(1369), scratch.ad_value(1374)), 1.0)));
        }

        if (((((scratch.values[1323] != 0.0) && (scratch.values[1340] != 0.0)) && (!(scratch.values[1474] != 0.0))) && (!(scratch.values[1479] != 0.0))) && (!(scratch.values[1480] != 0.0))) {
            scratch.store_ad(1375, &AdValue::powf(AdValue::offset(AdValue::mul(scratch.ad_value(1369), scratch.ad_value(1374)), 1.0), ((-scratch.values[376]) * scratch.values[461])));
        }

        if ((((scratch.values[1323] != 0.0) && (scratch.values[1340] != 0.0)) && (!(scratch.values[1474] != 0.0))) && (!(scratch.values[1479] != 0.0))) {
            scratch.store_ad(1376, &AdValue::div(AdValue::mul(scratch.ad_value(1365), scratch.ad_value(1375)), AdValue::add(scratch.ad_value(1365), scratch.ad_value(1375))));
        }

        if ((((scratch.values[1323] != 0.0) && (scratch.values[1340] != 0.0)) && (!(scratch.values[1474] != 0.0))) && (!(scratch.values[1479] != 0.0))) {
            scratch.store_ad(1377, &AdValue::sqrt(AdValue::scale(AdValue::div(scratch.ad_value(1369), scratch.ad_value(1373)), 0.375)));
        }

        if ((((scratch.values[1323] != 0.0) && (scratch.values[1340] != 0.0)) && (!(scratch.values[1474] != 0.0))) && (!(scratch.values[1479] != 0.0))) {
            scratch.store_ad(1378, &AdValue::sub(AdValue::scale(AdValue::mul(scratch.ad_value(1370), scratch.ad_value(1373)), 2.0), scratch.ad_value(1372)));
        }

        if ((((scratch.values[1323] != 0.0) && (scratch.values[1340] != 0.0)) && (!(scratch.values[1474] != 0.0))) && (!(scratch.values[1479] != 0.0))) {
            scratch.store_ad(1379, &AdValue::add(AdValue::sub(AdValue::mul(AdValue::scale(scratch.ad_value(1370), scratch.values[485]), scratch.ad_value(1373)), AdValue::scale(scratch.ad_value(1372), scratch.values[485])), AdValue::scale(AdValue::mul(scratch.ad_value(1369), scratch.ad_value(1374)), 0.5)));
        }

        if ((((scratch.values[1323] != 0.0) && (scratch.values[1340] != 0.0)) && (!(scratch.values[1474] != 0.0))) && (!(scratch.values[1479] != 0.0))) {
            scratch.store_ad(1380, &AdValue::mul(AdValue::offset(scratch.ad_value(1378), (-1.0)), scratch.ad_value(1377)));
        }

        if ((((scratch.values[1323] != 0.0) && (scratch.values[1340] != 0.0)) && (!(scratch.values[1474] != 0.0))) && (!(scratch.values[1479] != 0.0))) {
            scratch.store_ad(1341, &AdValue::square(scratch.ad_value(1380)));
        }

        scratch.values[1481] = if (scratch.values[1380] > 0.0) { 1.0 } else { 0.0 };

        if (((((scratch.values[1323] != 0.0) && (scratch.values[1340] != 0.0)) && (!(scratch.values[1474] != 0.0))) && (!(scratch.values[1479] != 0.0))) && (scratch.values[1481] != 0.0)) {
            scratch.store_ad(1342, &AdValue::div_from_scalar(1.0, AdValue::offset(AdValue::scale(scratch.ad_value(1380), scratch.values[421]), 1.0)));
        }

        if (((((scratch.values[1323] != 0.0) && (scratch.values[1340] != 0.0)) && (!(scratch.values[1474] != 0.0))) && (!(scratch.values[1479] != 0.0))) && (!(scratch.values[1481] != 0.0))) {
            scratch.store_ad(1342, &AdValue::div_from_scalar(1.0, AdValue::sub_from_scalar(1.0, AdValue::scale(scratch.ad_value(1380), scratch.values[421]))));
        }

        scratch.values[1482] = if (((-scratch.values[1341]) + scratch.values[1379]) > (-230.25850929940458)) { 1.0 } else { 0.0 };

        if (((((scratch.values[1323] != 0.0) && (scratch.values[1340] != 0.0)) && (!(scratch.values[1474] != 0.0))) && (!(scratch.values[1479] != 0.0))) && (scratch.values[1482] != 0.0)) {
            scratch.store_ad(1359, &AdValue::exp(AdValue::sub(scratch.ad_value(1379), scratch.ad_value(1341))));
        }

        if (((((scratch.values[1323] != 0.0) && (scratch.values[1340] != 0.0)) && (!(scratch.values[1474] != 0.0))) && (!(scratch.values[1479] != 0.0))) && (!(scratch.values[1482] != 0.0))) {
            let assign17810_ad_e16786: AdValue = AdValue::div_from_scalar(1e-100, AdValue::offset(AdValue::mul(AdValue::sub_from_scalar((-230.25850929940458), AdValue::sub(scratch.ad_value(1379), scratch.ad_value(1341))), AdValue::offset(AdValue::scale(AdValue::mul(AdValue::sub_from_scalar((-230.25850929940458), AdValue::sub(scratch.ad_value(1379), scratch.ad_value(1341))), AdValue::offset(AdValue::scale(AdValue::sub_from_scalar((-230.25850929940458), AdValue::sub(scratch.ad_value(1379), scratch.ad_value(1341))), 0.3333333333333333), 1.0)), 0.5), 1.0)), 1.0));
            scratch.store_ad(1359, &assign17810_ad_e16786);
        }

        if ((((scratch.values[1323] != 0.0) && (scratch.values[1340] != 0.0)) && (!(scratch.values[1474] != 0.0))) && (!(scratch.values[1479] != 0.0))) {
            scratch.store_ad(1343, &AdValue::mul(AdValue::add(AdValue::add(AdValue::scale(scratch.ad_value(1342), 0.29214664), AdValue::scale(AdValue::square(scratch.ad_value(1342)), scratch.values[422])), AdValue::scale(AdValue::mul(AdValue::square(scratch.ad_value(1342)), scratch.ad_value(1342)), scratch.values[423])), scratch.ad_value(1359)));
        }

        scratch.values[1483] = if (scratch.values[1380] > 0.0) { 1.0 } else { 0.0 };

        if (((((scratch.values[1323] != 0.0) && (scratch.values[1340] != 0.0)) && (!(scratch.values[1474] != 0.0))) && (!(scratch.values[1479] != 0.0))) && (scratch.values[1483] != 0.0)) {
            scratch.values[1381] = scratch.values[1343];
            scratch.node_derivatives[1381] = scratch.node_derivatives[1343];
            scratch.branch_derivatives[1381] = scratch.branch_derivatives[1343];
        }

        scratch.values[1484] = if (scratch.values[1379] > (-230.25850929940458)) { 1.0 } else { 0.0 };

        if ((((((scratch.values[1323] != 0.0) && (scratch.values[1340] != 0.0)) && (!(scratch.values[1474] != 0.0))) && (!(scratch.values[1479] != 0.0))) && (!(scratch.values[1483] != 0.0))) && (scratch.values[1484] != 0.0)) {
            scratch.store_ad(1359, &AdValue::exp(scratch.ad_value(1379)));
        }

        if ((((((scratch.values[1323] != 0.0) && (scratch.values[1340] != 0.0)) && (!(scratch.values[1474] != 0.0))) && (!(scratch.values[1479] != 0.0))) && (!(scratch.values[1483] != 0.0))) && (!(scratch.values[1484] != 0.0))) {
            scratch.store_ad(1359, &AdValue::div_from_scalar(1e-100, AdValue::offset(AdValue::mul(AdValue::sub_from_scalar((-230.25850929940458), scratch.ad_value(1379)), AdValue::offset(AdValue::scale(AdValue::mul(AdValue::sub_from_scalar((-230.25850929940458), scratch.ad_value(1379)), AdValue::offset(AdValue::scale(AdValue::sub_from_scalar((-230.25850929940458), scratch.ad_value(1379)), 0.3333333333333333), 1.0)), 0.5), 1.0)), 1.0)));
        }

        if (((((scratch.values[1323] != 0.0) && (scratch.values[1340] != 0.0)) && (!(scratch.values[1474] != 0.0))) && (!(scratch.values[1479] != 0.0))) && (!(scratch.values[1483] != 0.0))) {
            scratch.store_ad(1381, &AdValue::sub(AdValue::scale(scratch.ad_value(1359), 2.0), scratch.ad_value(1343)));
        }

        if ((((scratch.values[1323] != 0.0) && (scratch.values[1340] != 0.0)) && (!(scratch.values[1474] != 0.0))) && (!(scratch.values[1479] != 0.0))) {
            scratch.store_ad(1382, &AdValue::scale(AdValue::div(AdValue::scale(scratch.ad_value(1381), scratch.values[485]), scratch.ad_value(1377)), (1.772453850905516 * 0.5)));
        }

        if ((((scratch.values[1323] != 0.0) && (scratch.values[1340] != 0.0)) && (!(scratch.values[1474] != 0.0))) && (!(scratch.values[1479] != 0.0))) {
            scratch.store_ad(1368, &AdValue::scale(AdValue::mul(AdValue::mul(scratch.ad_value(1367), scratch.ad_value(1382)), scratch.ad_value(1376)), scratch.values[390]));
        }

        scratch.values[1485] = if (scratch.values[396] == 0.0) { 1.0 } else { 0.0 };

        if ((((scratch.values[1323] != 0.0) && (scratch.values[1340] != 0.0)) && (!(scratch.values[1474] != 0.0))) && (scratch.values[1485] != 0.0)) {
            scratch.values[1383] = 0.0;
            scratch.node_derivatives[1383] = [0.0; Instance::NODE_COUNT];
            scratch.branch_derivatives[1383] = [0.0; Instance::BRANCH_COUNT];
        }

        scratch.values[1486] = if (scratch.values[376] == 0.5) { 1.0 } else { 0.0 };

        if (((((scratch.values[1323] != 0.0) && (scratch.values[1340] != 0.0)) && (!(scratch.values[1474] != 0.0))) && (!(scratch.values[1485] != 0.0))) && (scratch.values[1486] != 0.0)) {
            scratch.store_ad(1359, &AdValue::sqrt(AdValue::scale(AdValue::sub_from_scalar(scratch.values[373], scratch.ad_value(1357)), scratch.values[479])));
        }

        if (((((scratch.values[1323] != 0.0) && (scratch.values[1340] != 0.0)) && (!(scratch.values[1474] != 0.0))) && (!(scratch.values[1485] != 0.0))) && (!(scratch.values[1486] != 0.0))) {
            scratch.store_ad(1359, &AdValue::powf(AdValue::scale(AdValue::sub_from_scalar(scratch.values[373], scratch.ad_value(1357)), scratch.values[479]), scratch.values[376]));
        }

        if ((((scratch.values[1323] != 0.0) && (scratch.values[1340] != 0.0)) && (!(scratch.values[1474] != 0.0))) && (!(scratch.values[1485] != 0.0))) {
            scratch.store_ad(1384, &AdValue::scale(AdValue::div(AdValue::scale(AdValue::sub_from_scalar(scratch.values[373], scratch.ad_value(1357)), scratch.values[476]), scratch.ad_value(1359)), scratch.values[461]));
        }

        scratch.values[1487] = if (((((-scratch.values[491]) / scratch.values[1384])) as f64).abs() < 230.25850929940458) { 1.0 } else { 0.0 };

        if (((((scratch.values[1323] != 0.0) && (scratch.values[1340] != 0.0)) && (!(scratch.values[1474] != 0.0))) && (!(scratch.values[1485] != 0.0))) && (scratch.values[1487] != 0.0)) {
            scratch.store_ad(1359, &AdValue::exp(AdValue::div(AdValue::neg(scratch.ad_value(491)), scratch.ad_value(1384))));
        }

        scratch.values[1488] = if (((-scratch.values[491]) / scratch.values[1384]) < 0.0) { 1.0 } else { 0.0 };

        if ((((((scratch.values[1323] != 0.0) && (scratch.values[1340] != 0.0)) && (!(scratch.values[1474] != 0.0))) && (!(scratch.values[1485] != 0.0))) && (!(scratch.values[1487] != 0.0))) && (scratch.values[1488] != 0.0)) {
            let assign18000_ad_e17113: AdValue = AdValue::offset(AdValue::mul(AdValue::sub_from_scalar((-230.25850929940458), AdValue::div(AdValue::neg(scratch.ad_value(491)), scratch.ad_value(1384))), AdValue::offset(AdValue::scale(AdValue::mul(AdValue::sub_from_scalar((-230.25850929940458), AdValue::div(AdValue::neg(scratch.ad_value(491)), scratch.ad_value(1384))), AdValue::offset(AdValue::scale(AdValue::sub_from_scalar((-230.25850929940458), AdValue::div(AdValue::neg(scratch.ad_value(491)), scratch.ad_value(1384))), 0.3333333333333333), 1.0)), 0.5), 1.0)), 1.0);
            scratch.store_ad(1359, &AdValue::div_from_scalar(1e-100, assign18000_ad_e17113));
        }

        if ((((((scratch.values[1323] != 0.0) && (scratch.values[1340] != 0.0)) && (!(scratch.values[1474] != 0.0))) && (!(scratch.values[1485] != 0.0))) && (!(scratch.values[1487] != 0.0))) && (!(scratch.values[1488] != 0.0))) {
            let assign18010_ad_e17163: AdValue = AdValue::scale(AdValue::offset(AdValue::mul(AdValue::offset(AdValue::div(AdValue::neg(scratch.ad_value(491)), scratch.ad_value(1384)), (-230.25850929940458)), AdValue::offset(AdValue::scale(AdValue::mul(AdValue::offset(AdValue::div(AdValue::neg(scratch.ad_value(491)), scratch.ad_value(1384)), (-230.25850929940458)), AdValue::offset(AdValue::scale(AdValue::offset(AdValue::div(AdValue::neg(scratch.ad_value(491)), scratch.ad_value(1384)), (-230.25850929940458)), 0.3333333333333333), 1.0)), 0.5), 1.0)), 1.0), 1e100);
            scratch.store_ad(1359, &assign18010_ad_e17163);
        }

        if ((((scratch.values[1323] != 0.0) && (scratch.values[1340] != 0.0)) && (!(scratch.values[1474] != 0.0))) && (!(scratch.values[1485] != 0.0))) {
            scratch.store_ad(1383, &AdValue::scale(AdValue::mul(AdValue::mul(AdValue::mul(scratch.ad_value(517), scratch.ad_value(1384)), scratch.ad_value(1384)), scratch.ad_value(1359)), scratch.values[396]));
        }

        scratch.values[1489] = if (scratch.values[405] > 1000.0) { 1.0 } else { 0.0 };

        if ((((scratch.values[1323] != 0.0) && (scratch.values[1340] != 0.0)) && (!(scratch.values[1474] != 0.0))) && (scratch.values[1489] != 0.0)) {
            scratch.values[1385] = 1.0;
            scratch.node_derivatives[1385] = [0.0; Instance::NODE_COUNT];
            scratch.branch_derivatives[1385] = [0.0; Instance::BRANCH_COUNT];
        }

        scratch.values[1490] = if (scratch.values[1358] > ((-scratch.values[493]) * scratch.values[405])) { 1.0 } else { 0.0 };

        scratch.values[1491] = if (scratch.values[408] == 4.0) { 1.0 } else { 0.0 };

        if ((((((scratch.values[1323] != 0.0) && (scratch.values[1340] != 0.0)) && (!(scratch.values[1474] != 0.0))) && (!(scratch.values[1489] != 0.0))) && (scratch.values[1490] != 0.0)) && (scratch.values[1491] != 0.0)) {
            scratch.store_ad(1359, &AdValue::mul(AdValue::mul(AdValue::mul(AdValue::scale(scratch.ad_value(1358), scratch.values[498]), AdValue::scale(scratch.ad_value(1358), scratch.values[498])), AdValue::scale(scratch.ad_value(1358), scratch.values[498])), AdValue::scale(scratch.ad_value(1358), scratch.values[498])));
        }

        if ((((((scratch.values[1323] != 0.0) && (scratch.values[1340] != 0.0)) && (!(scratch.values[1474] != 0.0))) && (!(scratch.values[1489] != 0.0))) && (scratch.values[1490] != 0.0)) && (!(scratch.values[1491] != 0.0))) {
            scratch.store_ad(1359, &AdValue::powf(AdValue::abs(AdValue::scale(scratch.ad_value(1358), scratch.values[498])), scratch.values[408]));
        }

        if (((((scratch.values[1323] != 0.0) && (scratch.values[1340] != 0.0)) && (!(scratch.values[1474] != 0.0))) && (!(scratch.values[1489] != 0.0))) && (scratch.values[1490] != 0.0)) {
            scratch.store_ad(1385, &AdValue::div_from_scalar(1.0, AdValue::sub_from_scalar(1.0, scratch.ad_value(1359))));
        }

        if (((((scratch.values[1323] != 0.0) && (scratch.values[1340] != 0.0)) && (!(scratch.values[1474] != 0.0))) && (!(scratch.values[1489] != 0.0))) && (!(scratch.values[1490] != 0.0))) {
            scratch.store_ad(1385, &AdValue::offset(AdValue::scale(AdValue::offset(scratch.ad_value(1358), (scratch.values[493] * scratch.values[405])), scratch.values[501]), scratch.values[495]));
        }

        if (((scratch.values[1323] != 0.0) && (scratch.values[1340] != 0.0)) && (!(scratch.values[1474] != 0.0))) {
            scratch.store_ad(1387, &AdValue::mul(AdValue::add(AdValue::add(AdValue::add(scratch.ad_value(1360), scratch.ad_value(1361)), scratch.ad_value(1368)), scratch.ad_value(1383)), scratch.ad_value(1385)));
        }

        scratch.values[1492] = if (scratch.values[692] == 0.0) { 1.0 } else { 0.0 };

        if (((scratch.values[1323] != 0.0) && (scratch.values[1340] != 0.0)) && (scratch.values[1492] != 0.0)) {
            scratch.values[1388] = 0.0;
            scratch.node_derivatives[1388] = [0.0; Instance::NODE_COUNT];
            scratch.branch_derivatives[1388] = [0.0; Instance::BRANCH_COUNT];
        }

        scratch.values[1493] = if (scratch.values[459] == 0.5) { 1.0 } else { 0.0 };

        if ((((scratch.values[1323] != 0.0) && (scratch.values[1340] != 0.0)) && (!(scratch.values[1492] != 0.0))) && (scratch.values[1493] != 0.0)) {
            scratch.store_ad(1359, &AdValue::sqrt(AdValue::sub_from_scalar(1.0, AdValue::scale(scratch.ad_value(1351), scratch.values[456]))));
        }

        if ((((scratch.values[1323] != 0.0) && (scratch.values[1340] != 0.0)) && (!(scratch.values[1492] != 0.0))) && (!(scratch.values[1493] != 0.0))) {
            scratch.store_ad(1359, &AdValue::powf(AdValue::sub_from_scalar(1.0, AdValue::scale(scratch.ad_value(1351), scratch.values[456])), scratch.values[459]));
        }

        if (((scratch.values[1323] != 0.0) && (scratch.values[1340] != 0.0)) && (!(scratch.values[1492] != 0.0))) {
            scratch.store_ad(1360, &AdValue::scale(scratch.ad_value(1350), scratch.values[438]));
        }

        scratch.values[1494] = if ((scratch.values[388] == 0.0) && (scratch.values[391] == 0.0)) { 1.0 } else { 0.0 };

        if ((((scratch.values[1323] != 0.0) && (scratch.values[1340] != 0.0)) && (!(scratch.values[1492] != 0.0))) && (scratch.values[1494] != 0.0)) {
            scratch.values[1361] = 0.0;
            scratch.node_derivatives[1361] = [0.0; Instance::NODE_COUNT];
            scratch.branch_derivatives[1361] = [0.0; Instance::BRANCH_COUNT];
        }

        if ((((scratch.values[1323] != 0.0) && (scratch.values[1340] != 0.0)) && (!(scratch.values[1492] != 0.0))) && (!(scratch.values[1494] != 0.0))) {
            scratch.store_ad(1362, &AdValue::sub_from_scalar(scratch.values[444], scratch.ad_value(1356)));
        }

        if ((((scratch.values[1323] != 0.0) && (scratch.values[1340] != 0.0)) && (!(scratch.values[1492] != 0.0))) && (!(scratch.values[1494] != 0.0))) {
            scratch.store_ad(1363, &AdValue::sub_from_scalar(1.0, AdValue::sqrt(AdValue::sub_from_scalar(1.0, AdValue::div(scratch.ad_value(1354), scratch.ad_value(1362))))));
        }

        scratch.values[1495] = if (scratch.values[377] == 0.5) { 1.0 } else { 0.0 };

        if (((((scratch.values[1323] != 0.0) && (scratch.values[1340] != 0.0)) && (!(scratch.values[1492] != 0.0))) && (!(scratch.values[1494] != 0.0))) && (scratch.values[1495] != 0.0)) {
            scratch.values[1364] = 0.0;
            scratch.node_derivatives[1364] = [0.0; Instance::NODE_COUNT];
            scratch.branch_derivatives[1364] = [0.0; Instance::BRANCH_COUNT];
        }

        if (((((scratch.values[1323] != 0.0) && (scratch.values[1340] != 0.0)) && (!(scratch.values[1492] != 0.0))) && (!(scratch.values[1494] != 0.0))) && (!(scratch.values[1495] != 0.0))) {
            scratch.store_ad(1364, &AdValue::scale(AdValue::add(AdValue::div(AdValue::mul(AdValue::square(scratch.ad_value(1363)), AdValue::ln(scratch.ad_value(1363))), AdValue::sub_from_scalar(1.0, scratch.ad_value(1363))), scratch.ad_value(1363)), (1.0 - (2.0 * scratch.values[377]))));
        }

        if ((((scratch.values[1323] != 0.0) && (scratch.values[1340] != 0.0)) && (!(scratch.values[1492] != 0.0))) && (!(scratch.values[1494] != 0.0))) {
            scratch.store_ad(1365, &AdValue::add(scratch.ad_value(1363), scratch.ad_value(1364)));
        }

        scratch.values[1496] = if (scratch.values[377] == 0.5) { 1.0 } else { 0.0 };

        if (((((scratch.values[1323] != 0.0) && (scratch.values[1340] != 0.0)) && (!(scratch.values[1492] != 0.0))) && (!(scratch.values[1494] != 0.0))) && (scratch.values[1496] != 0.0)) {
            scratch.store_ad(1359, &AdValue::sqrt(AdValue::scale(scratch.ad_value(1362), scratch.values[480])));
        }

        if (((((scratch.values[1323] != 0.0) && (scratch.values[1340] != 0.0)) && (!(scratch.values[1492] != 0.0))) && (!(scratch.values[1494] != 0.0))) && (!(scratch.values[1496] != 0.0))) {
            scratch.store_ad(1359, &AdValue::powf(AdValue::scale(scratch.ad_value(1362), scratch.values[480]), scratch.values[377]));
        }

        if ((((scratch.values[1323] != 0.0) && (scratch.values[1340] != 0.0)) && (!(scratch.values[1492] != 0.0))) && (!(scratch.values[1494] != 0.0))) {
            scratch.store_ad(1366, &AdValue::scale(scratch.ad_value(1359), scratch.values[474]));
        }

        if ((((scratch.values[1323] != 0.0) && (scratch.values[1340] != 0.0)) && (!(scratch.values[1492] != 0.0))) && (!(scratch.values[1494] != 0.0))) {
            scratch.store_ad(1367, &AdValue::scale(AdValue::mul(AdValue::offset(scratch.ad_value(1353), (-1.0)), scratch.ad_value(1366)), scratch.values[435]));
        }

        if ((((scratch.values[1323] != 0.0) && (scratch.values[1340] != 0.0)) && (!(scratch.values[1492] != 0.0))) && (!(scratch.values[1494] != 0.0))) {
            scratch.store_ad(1361, &AdValue::scale(AdValue::mul(scratch.ad_value(1367), scratch.ad_value(1365)), scratch.values[388]));
        }

        scratch.values[1497] = if (scratch.values[391] == 0.0) { 1.0 } else { 0.0 };

        if ((((scratch.values[1323] != 0.0) && (scratch.values[1340] != 0.0)) && (!(scratch.values[1492] != 0.0))) && (scratch.values[1497] != 0.0)) {
            scratch.values[1368] = 0.0;
            scratch.node_derivatives[1368] = [0.0; Instance::NODE_COUNT];
            scratch.branch_derivatives[1368] = [0.0; Instance::BRANCH_COUNT];
        }

        if ((((scratch.values[1323] != 0.0) && (scratch.values[1340] != 0.0)) && (!(scratch.values[1492] != 0.0))) && (!(scratch.values[1497] != 0.0))) {
            scratch.store_ad(1369, &AdValue::scale(AdValue::div(AdValue::scale(scratch.ad_value(1366), scratch.values[459]), scratch.ad_value(1362)), scratch.values[489]));
        }

        if ((((scratch.values[1323] != 0.0) && (scratch.values[1340] != 0.0)) && (!(scratch.values[1492] != 0.0))) && (!(scratch.values[1497] != 0.0))) {
            scratch.store_ad(1370, &AdValue::div_from_scalar((0.666666666666667 * scratch.values[486]), scratch.ad_value(1369)));
        }

        if ((((scratch.values[1323] != 0.0) && (scratch.values[1340] != 0.0)) && (!(scratch.values[1492] != 0.0))) && (!(scratch.values[1497] != 0.0))) {
            scratch.store_ad(1371, &AdValue::square(scratch.ad_value(1370)));
        }

        if ((((scratch.values[1323] != 0.0) && (scratch.values[1340] != 0.0)) && (!(scratch.values[1492] != 0.0))) && (!(scratch.values[1497] != 0.0))) {
            scratch.store_ad(1372, &AdValue::sqrt(AdValue::div(AdValue::square(scratch.ad_value(1371)), AdValue::offset(AdValue::square(scratch.ad_value(1371)), 1.0))));
        }

        if ((((scratch.values[1323] != 0.0) && (scratch.values[1340] != 0.0)) && (!(scratch.values[1492] != 0.0))) && (!(scratch.values[1497] != 0.0))) {
            scratch.store_ad(1373, &AdValue::sqrt(AdValue::abs(scratch.ad_value(1372))));
        }

        if ((((scratch.values[1323] != 0.0) && (scratch.values[1340] != 0.0)) && (!(scratch.values[1492] != 0.0))) && (!(scratch.values[1497] != 0.0))) {
            scratch.store_ad(1374, &AdValue::mul(scratch.ad_value(1372), scratch.ad_value(1373)));
        }

        scratch.values[1498] = if (((-scratch.values[377]) * scratch.values[462]) == (-1.0)) { 1.0 } else { 0.0 };

        if (((((scratch.values[1323] != 0.0) && (scratch.values[1340] != 0.0)) && (!(scratch.values[1492] != 0.0))) && (!(scratch.values[1497] != 0.0))) && (scratch.values[1498] != 0.0)) {
            scratch.store_ad(1375, &AdValue::div_from_scalar(1.0, AdValue::offset(AdValue::mul(scratch.ad_value(1369), scratch.ad_value(1374)), 1.0)));
        }

        if (((((scratch.values[1323] != 0.0) && (scratch.values[1340] != 0.0)) && (!(scratch.values[1492] != 0.0))) && (!(scratch.values[1497] != 0.0))) && (!(scratch.values[1498] != 0.0))) {
            scratch.store_ad(1375, &AdValue::powf(AdValue::offset(AdValue::mul(scratch.ad_value(1369), scratch.ad_value(1374)), 1.0), ((-scratch.values[377]) * scratch.values[462])));
        }

        if ((((scratch.values[1323] != 0.0) && (scratch.values[1340] != 0.0)) && (!(scratch.values[1492] != 0.0))) && (!(scratch.values[1497] != 0.0))) {
            scratch.store_ad(1376, &AdValue::div(AdValue::mul(scratch.ad_value(1365), scratch.ad_value(1375)), AdValue::add(scratch.ad_value(1365), scratch.ad_value(1375))));
        }

        if ((((scratch.values[1323] != 0.0) && (scratch.values[1340] != 0.0)) && (!(scratch.values[1492] != 0.0))) && (!(scratch.values[1497] != 0.0))) {
            scratch.store_ad(1377, &AdValue::sqrt(AdValue::scale(AdValue::div(scratch.ad_value(1369), scratch.ad_value(1373)), 0.375)));
        }

        if ((((scratch.values[1323] != 0.0) && (scratch.values[1340] != 0.0)) && (!(scratch.values[1492] != 0.0))) && (!(scratch.values[1497] != 0.0))) {
            scratch.store_ad(1378, &AdValue::sub(AdValue::scale(AdValue::mul(scratch.ad_value(1370), scratch.ad_value(1373)), 2.0), scratch.ad_value(1372)));
        }

        if ((((scratch.values[1323] != 0.0) && (scratch.values[1340] != 0.0)) && (!(scratch.values[1492] != 0.0))) && (!(scratch.values[1497] != 0.0))) {
            scratch.store_ad(1379, &AdValue::add(AdValue::sub(AdValue::mul(AdValue::scale(scratch.ad_value(1370), scratch.values[486]), scratch.ad_value(1373)), AdValue::scale(scratch.ad_value(1372), scratch.values[486])), AdValue::scale(AdValue::mul(scratch.ad_value(1369), scratch.ad_value(1374)), 0.5)));
        }

        if ((((scratch.values[1323] != 0.0) && (scratch.values[1340] != 0.0)) && (!(scratch.values[1492] != 0.0))) && (!(scratch.values[1497] != 0.0))) {
            scratch.store_ad(1380, &AdValue::mul(AdValue::offset(scratch.ad_value(1378), (-1.0)), scratch.ad_value(1377)));
        }

        if ((((scratch.values[1323] != 0.0) && (scratch.values[1340] != 0.0)) && (!(scratch.values[1492] != 0.0))) && (!(scratch.values[1497] != 0.0))) {
            scratch.store_ad(1341, &AdValue::square(scratch.ad_value(1380)));
        }

        scratch.values[1499] = if (scratch.values[1380] > 0.0) { 1.0 } else { 0.0 };

        if (((((scratch.values[1323] != 0.0) && (scratch.values[1340] != 0.0)) && (!(scratch.values[1492] != 0.0))) && (!(scratch.values[1497] != 0.0))) && (scratch.values[1499] != 0.0)) {
            scratch.store_ad(1342, &AdValue::div_from_scalar(1.0, AdValue::offset(AdValue::scale(scratch.ad_value(1380), scratch.values[421]), 1.0)));
        }

        if (((((scratch.values[1323] != 0.0) && (scratch.values[1340] != 0.0)) && (!(scratch.values[1492] != 0.0))) && (!(scratch.values[1497] != 0.0))) && (!(scratch.values[1499] != 0.0))) {
            scratch.store_ad(1342, &AdValue::div_from_scalar(1.0, AdValue::sub_from_scalar(1.0, AdValue::scale(scratch.ad_value(1380), scratch.values[421]))));
        }

        scratch.values[1500] = if (((-scratch.values[1341]) + scratch.values[1379]) > (-230.25850929940458)) { 1.0 } else { 0.0 };

        if (((((scratch.values[1323] != 0.0) && (scratch.values[1340] != 0.0)) && (!(scratch.values[1492] != 0.0))) && (!(scratch.values[1497] != 0.0))) && (scratch.values[1500] != 0.0)) {
            scratch.store_ad(1359, &AdValue::exp(AdValue::sub(scratch.ad_value(1379), scratch.ad_value(1341))));
        }

        if (((((scratch.values[1323] != 0.0) && (scratch.values[1340] != 0.0)) && (!(scratch.values[1492] != 0.0))) && (!(scratch.values[1497] != 0.0))) && (!(scratch.values[1500] != 0.0))) {
            let assign18560_ad_e17992: AdValue = AdValue::div_from_scalar(1e-100, AdValue::offset(AdValue::mul(AdValue::sub_from_scalar((-230.25850929940458), AdValue::sub(scratch.ad_value(1379), scratch.ad_value(1341))), AdValue::offset(AdValue::scale(AdValue::mul(AdValue::sub_from_scalar((-230.25850929940458), AdValue::sub(scratch.ad_value(1379), scratch.ad_value(1341))), AdValue::offset(AdValue::scale(AdValue::sub_from_scalar((-230.25850929940458), AdValue::sub(scratch.ad_value(1379), scratch.ad_value(1341))), 0.3333333333333333), 1.0)), 0.5), 1.0)), 1.0));
            scratch.store_ad(1359, &assign18560_ad_e17992);
        }

        if ((((scratch.values[1323] != 0.0) && (scratch.values[1340] != 0.0)) && (!(scratch.values[1492] != 0.0))) && (!(scratch.values[1497] != 0.0))) {
            scratch.store_ad(1343, &AdValue::mul(AdValue::add(AdValue::add(AdValue::scale(scratch.ad_value(1342), 0.29214664), AdValue::scale(AdValue::square(scratch.ad_value(1342)), scratch.values[422])), AdValue::scale(AdValue::mul(AdValue::square(scratch.ad_value(1342)), scratch.ad_value(1342)), scratch.values[423])), scratch.ad_value(1359)));
        }

        scratch.values[1501] = if (scratch.values[1380] > 0.0) { 1.0 } else { 0.0 };

        if (((((scratch.values[1323] != 0.0) && (scratch.values[1340] != 0.0)) && (!(scratch.values[1492] != 0.0))) && (!(scratch.values[1497] != 0.0))) && (scratch.values[1501] != 0.0)) {
            scratch.values[1381] = scratch.values[1343];
            scratch.node_derivatives[1381] = scratch.node_derivatives[1343];
            scratch.branch_derivatives[1381] = scratch.branch_derivatives[1343];
        }

        scratch.values[1502] = if (scratch.values[1379] > (-230.25850929940458)) { 1.0 } else { 0.0 };

        if ((((((scratch.values[1323] != 0.0) && (scratch.values[1340] != 0.0)) && (!(scratch.values[1492] != 0.0))) && (!(scratch.values[1497] != 0.0))) && (!(scratch.values[1501] != 0.0))) && (scratch.values[1502] != 0.0)) {
            scratch.store_ad(1359, &AdValue::exp(scratch.ad_value(1379)));
        }

        if ((((((scratch.values[1323] != 0.0) && (scratch.values[1340] != 0.0)) && (!(scratch.values[1492] != 0.0))) && (!(scratch.values[1497] != 0.0))) && (!(scratch.values[1501] != 0.0))) && (!(scratch.values[1502] != 0.0))) {
            scratch.store_ad(1359, &AdValue::div_from_scalar(1e-100, AdValue::offset(AdValue::mul(AdValue::sub_from_scalar((-230.25850929940458), scratch.ad_value(1379)), AdValue::offset(AdValue::scale(AdValue::mul(AdValue::sub_from_scalar((-230.25850929940458), scratch.ad_value(1379)), AdValue::offset(AdValue::scale(AdValue::sub_from_scalar((-230.25850929940458), scratch.ad_value(1379)), 0.3333333333333333), 1.0)), 0.5), 1.0)), 1.0)));
        }

        if (((((scratch.values[1323] != 0.0) && (scratch.values[1340] != 0.0)) && (!(scratch.values[1492] != 0.0))) && (!(scratch.values[1497] != 0.0))) && (!(scratch.values[1501] != 0.0))) {
            scratch.store_ad(1381, &AdValue::sub(AdValue::scale(scratch.ad_value(1359), 2.0), scratch.ad_value(1343)));
        }

        if ((((scratch.values[1323] != 0.0) && (scratch.values[1340] != 0.0)) && (!(scratch.values[1492] != 0.0))) && (!(scratch.values[1497] != 0.0))) {
            scratch.store_ad(1382, &AdValue::scale(AdValue::div(AdValue::scale(scratch.ad_value(1381), scratch.values[486]), scratch.ad_value(1377)), (1.772453850905516 * 0.5)));
        }

        if ((((scratch.values[1323] != 0.0) && (scratch.values[1340] != 0.0)) && (!(scratch.values[1492] != 0.0))) && (!(scratch.values[1497] != 0.0))) {
            scratch.store_ad(1368, &AdValue::scale(AdValue::mul(AdValue::mul(scratch.ad_value(1367), scratch.ad_value(1382)), scratch.ad_value(1376)), scratch.values[391]));
        }

        scratch.values[1503] = if (scratch.values[397] == 0.0) { 1.0 } else { 0.0 };

        if ((((scratch.values[1323] != 0.0) && (scratch.values[1340] != 0.0)) && (!(scratch.values[1492] != 0.0))) && (scratch.values[1503] != 0.0)) {
            scratch.values[1383] = 0.0;
            scratch.node_derivatives[1383] = [0.0; Instance::NODE_COUNT];
            scratch.branch_derivatives[1383] = [0.0; Instance::BRANCH_COUNT];
        }

        scratch.values[1504] = if (scratch.values[377] == 0.5) { 1.0 } else { 0.0 };

        if (((((scratch.values[1323] != 0.0) && (scratch.values[1340] != 0.0)) && (!(scratch.values[1492] != 0.0))) && (!(scratch.values[1503] != 0.0))) && (scratch.values[1504] != 0.0)) {
            scratch.store_ad(1359, &AdValue::sqrt(AdValue::scale(AdValue::sub_from_scalar(scratch.values[374], scratch.ad_value(1357)), scratch.values[480])));
        }

        if (((((scratch.values[1323] != 0.0) && (scratch.values[1340] != 0.0)) && (!(scratch.values[1492] != 0.0))) && (!(scratch.values[1503] != 0.0))) && (!(scratch.values[1504] != 0.0))) {
            scratch.store_ad(1359, &AdValue::powf(AdValue::scale(AdValue::sub_from_scalar(scratch.values[374], scratch.ad_value(1357)), scratch.values[480]), scratch.values[377]));
        }

        if ((((scratch.values[1323] != 0.0) && (scratch.values[1340] != 0.0)) && (!(scratch.values[1492] != 0.0))) && (!(scratch.values[1503] != 0.0))) {
            scratch.store_ad(1384, &AdValue::scale(AdValue::div(AdValue::scale(AdValue::sub_from_scalar(scratch.values[374], scratch.ad_value(1357)), scratch.values[477]), scratch.ad_value(1359)), scratch.values[462]));
        }

        scratch.values[1505] = if (((((-scratch.values[492]) / scratch.values[1384])) as f64).abs() < 230.25850929940458) { 1.0 } else { 0.0 };

        if (((((scratch.values[1323] != 0.0) && (scratch.values[1340] != 0.0)) && (!(scratch.values[1492] != 0.0))) && (!(scratch.values[1503] != 0.0))) && (scratch.values[1505] != 0.0)) {
            scratch.store_ad(1359, &AdValue::exp(AdValue::div(AdValue::neg(scratch.ad_value(492)), scratch.ad_value(1384))));
        }

        scratch.values[1506] = if (((-scratch.values[492]) / scratch.values[1384]) < 0.0) { 1.0 } else { 0.0 };

        if ((((((scratch.values[1323] != 0.0) && (scratch.values[1340] != 0.0)) && (!(scratch.values[1492] != 0.0))) && (!(scratch.values[1503] != 0.0))) && (!(scratch.values[1505] != 0.0))) && (scratch.values[1506] != 0.0)) {
            let assign18750_ad_e18319: AdValue = AdValue::offset(AdValue::mul(AdValue::sub_from_scalar((-230.25850929940458), AdValue::div(AdValue::neg(scratch.ad_value(492)), scratch.ad_value(1384))), AdValue::offset(AdValue::scale(AdValue::mul(AdValue::sub_from_scalar((-230.25850929940458), AdValue::div(AdValue::neg(scratch.ad_value(492)), scratch.ad_value(1384))), AdValue::offset(AdValue::scale(AdValue::sub_from_scalar((-230.25850929940458), AdValue::div(AdValue::neg(scratch.ad_value(492)), scratch.ad_value(1384))), 0.3333333333333333), 1.0)), 0.5), 1.0)), 1.0);
            scratch.store_ad(1359, &AdValue::div_from_scalar(1e-100, assign18750_ad_e18319));
        }

        if ((((((scratch.values[1323] != 0.0) && (scratch.values[1340] != 0.0)) && (!(scratch.values[1492] != 0.0))) && (!(scratch.values[1503] != 0.0))) && (!(scratch.values[1505] != 0.0))) && (!(scratch.values[1506] != 0.0))) {
            let assign18760_ad_e18369: AdValue = AdValue::scale(AdValue::offset(AdValue::mul(AdValue::offset(AdValue::div(AdValue::neg(scratch.ad_value(492)), scratch.ad_value(1384)), (-230.25850929940458)), AdValue::offset(AdValue::scale(AdValue::mul(AdValue::offset(AdValue::div(AdValue::neg(scratch.ad_value(492)), scratch.ad_value(1384)), (-230.25850929940458)), AdValue::offset(AdValue::scale(AdValue::offset(AdValue::div(AdValue::neg(scratch.ad_value(492)), scratch.ad_value(1384)), (-230.25850929940458)), 0.3333333333333333), 1.0)), 0.5), 1.0)), 1.0), 1e100);
            scratch.store_ad(1359, &assign18760_ad_e18369);
        }

        if ((((scratch.values[1323] != 0.0) && (scratch.values[1340] != 0.0)) && (!(scratch.values[1492] != 0.0))) && (!(scratch.values[1503] != 0.0))) {
            scratch.store_ad(1383, &AdValue::scale(AdValue::mul(AdValue::mul(AdValue::mul(scratch.ad_value(517), scratch.ad_value(1384)), scratch.ad_value(1384)), scratch.ad_value(1359)), scratch.values[397]));
        }

        scratch.values[1507] = if (scratch.values[406] > 1000.0) { 1.0 } else { 0.0 };

        if ((((scratch.values[1323] != 0.0) && (scratch.values[1340] != 0.0)) && (!(scratch.values[1492] != 0.0))) && (scratch.values[1507] != 0.0)) {
            scratch.values[1385] = 1.0;
            scratch.node_derivatives[1385] = [0.0; Instance::NODE_COUNT];
            scratch.branch_derivatives[1385] = [0.0; Instance::BRANCH_COUNT];
        }

        scratch.values[1508] = if (scratch.values[1358] > ((-scratch.values[493]) * scratch.values[406])) { 1.0 } else { 0.0 };

        scratch.values[1509] = if (scratch.values[409] == 4.0) { 1.0 } else { 0.0 };

        if ((((((scratch.values[1323] != 0.0) && (scratch.values[1340] != 0.0)) && (!(scratch.values[1492] != 0.0))) && (!(scratch.values[1507] != 0.0))) && (scratch.values[1508] != 0.0)) && (scratch.values[1509] != 0.0)) {
            scratch.store_ad(1359, &AdValue::mul(AdValue::mul(AdValue::mul(AdValue::scale(scratch.ad_value(1358), scratch.values[499]), AdValue::scale(scratch.ad_value(1358), scratch.values[499])), AdValue::scale(scratch.ad_value(1358), scratch.values[499])), AdValue::scale(scratch.ad_value(1358), scratch.values[499])));
        }

        if ((((((scratch.values[1323] != 0.0) && (scratch.values[1340] != 0.0)) && (!(scratch.values[1492] != 0.0))) && (!(scratch.values[1507] != 0.0))) && (scratch.values[1508] != 0.0)) && (!(scratch.values[1509] != 0.0))) {
            scratch.store_ad(1359, &AdValue::powf(AdValue::abs(AdValue::scale(scratch.ad_value(1358), scratch.values[499])), scratch.values[409]));
        }

        if (((((scratch.values[1323] != 0.0) && (scratch.values[1340] != 0.0)) && (!(scratch.values[1492] != 0.0))) && (!(scratch.values[1507] != 0.0))) && (scratch.values[1508] != 0.0)) {
            scratch.store_ad(1385, &AdValue::div_from_scalar(1.0, AdValue::sub_from_scalar(1.0, scratch.ad_value(1359))));
        }

        if (((((scratch.values[1323] != 0.0) && (scratch.values[1340] != 0.0)) && (!(scratch.values[1492] != 0.0))) && (!(scratch.values[1507] != 0.0))) && (!(scratch.values[1508] != 0.0))) {
            scratch.store_ad(1385, &AdValue::offset(AdValue::scale(AdValue::offset(scratch.ad_value(1358), (scratch.values[493] * scratch.values[406])), scratch.values[502]), scratch.values[496]));
        }

        if (((scratch.values[1323] != 0.0) && (scratch.values[1340] != 0.0)) && (!(scratch.values[1492] != 0.0))) {
            scratch.store_ad(1388, &AdValue::mul(AdValue::add(AdValue::add(AdValue::add(scratch.ad_value(1360), scratch.ad_value(1361)), scratch.ad_value(1368)), scratch.ad_value(1383)), scratch.ad_value(1385)));
        }

        if ((scratch.values[1323] != 0.0) && (scratch.values[1340] != 0.0)) {
            scratch.store_ad(507, &AdValue::add(AdValue::add(AdValue::mul(scratch.ad_value(690), scratch.ad_value(1386)), AdValue::mul(scratch.ad_value(691), scratch.ad_value(1387))), AdValue::mul(scratch.ad_value(692), scratch.ad_value(1388))));
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

        scratch.values[1510] = if !(((scratch.values[690] == 0.0) && (scratch.values[691] == 0.0)) && (scratch.values[692] == 0.0)) { 1.0 } else { 0.0 };

        if (((scratch.values[1323] != 0.0) && (scratch.values[1340] != 0.0)) && (scratch.values[1510] != 0.0)) {
            scratch.store_ad(1344, &AdValue::mul(AdValue::scale(scratch.ad_value(701), 4.0), scratch.ad_value(701)));
        }

        if (((scratch.values[1323] != 0.0) && (scratch.values[1340] != 0.0)) && (scratch.values[1510] != 0.0)) {
            scratch.store_ad(1345, &AdValue::div(scratch.ad_value(701), scratch.ad_value(702)));
        }

        if (((scratch.values[1323] != 0.0) && (scratch.values[1340] != 0.0)) && (scratch.values[1510] != 0.0)) {
            scratch.store_ad(1346, &AdValue::add(scratch.ad_value(518), AdValue::mul(scratch.ad_value(701), scratch.ad_value(1345))));
        }

        if (((scratch.values[1323] != 0.0) && (scratch.values[1340] != 0.0)) && (scratch.values[1510] != 0.0)) {
            scratch.store_ad(1347, &AdValue::add(scratch.ad_value(702), scratch.ad_value(1346)));
        }

        if (((scratch.values[1323] != 0.0) && (scratch.values[1340] != 0.0)) && (scratch.values[1510] != 0.0)) {
            scratch.store_ad(1348, &AdValue::sub(scratch.ad_value(702), scratch.ad_value(1346)));
        }

        if (((scratch.values[1323] != 0.0) && (scratch.values[1340] != 0.0)) && (scratch.values[1510] != 0.0)) {
            scratch.store_ad(1349, &AdValue::sqrt(AdValue::add(AdValue::square(scratch.ad_value(1348)), scratch.ad_value(1344))));
        }

        if (((scratch.values[1323] != 0.0) && (scratch.values[1340] != 0.0)) && (scratch.values[1510] != 0.0)) {
            scratch.store_ad(1351, &AdValue::scale(AdValue::div(AdValue::mul(scratch.ad_value(518), scratch.ad_value(702)), AdValue::add(scratch.ad_value(1347), scratch.ad_value(1349))), 2.0));
        }

        scratch.values[1511] = if (scratch.values[518] < scratch.values[698]) { 1.0 } else { 0.0 };

        scratch.values[1512] = if ((((0.5 * (scratch.values[518] * scratch.values[420]))) as f64).abs() < 230.25850929940458) { 1.0 } else { 0.0 };

        if (((((scratch.values[1323] != 0.0) && (scratch.values[1340] != 0.0)) && (scratch.values[1510] != 0.0)) && (scratch.values[1511] != 0.0)) && (scratch.values[1512] != 0.0)) {
            scratch.store_ad(1353, &AdValue::exp(AdValue::scale(scratch.ad_value(518), (scratch.values[420] * 0.5))));
        }

        scratch.values[1513] = if ((0.5 * (scratch.values[518] * scratch.values[420])) < 0.0) { 1.0 } else { 0.0 };

        if ((((((scratch.values[1323] != 0.0) && (scratch.values[1340] != 0.0)) && (scratch.values[1510] != 0.0)) && (scratch.values[1511] != 0.0)) && (!(scratch.values[1512] != 0.0))) && (scratch.values[1513] != 0.0)) {
            let assign19020_ad_e18732: AdValue = AdValue::div_from_scalar(1e-100, AdValue::offset(AdValue::mul(AdValue::sub_from_scalar((-230.25850929940458), AdValue::scale(scratch.ad_value(518), (scratch.values[420] * 0.5))), AdValue::offset(AdValue::scale(AdValue::mul(AdValue::sub_from_scalar((-230.25850929940458), AdValue::scale(scratch.ad_value(518), (scratch.values[420] * 0.5))), AdValue::offset(AdValue::scale(AdValue::sub_from_scalar((-230.25850929940458), AdValue::scale(scratch.ad_value(518), (scratch.values[420] * 0.5))), 0.3333333333333333), 1.0)), 0.5), 1.0)), 1.0));
            scratch.store_ad(1353, &assign19020_ad_e18732);
        }

        if ((((((scratch.values[1323] != 0.0) && (scratch.values[1340] != 0.0)) && (scratch.values[1510] != 0.0)) && (scratch.values[1511] != 0.0)) && (!(scratch.values[1512] != 0.0))) && (!(scratch.values[1513] != 0.0))) {
            scratch.store_ad(1353, &AdValue::scale(AdValue::offset(AdValue::mul(AdValue::offset(AdValue::scale(scratch.ad_value(518), (scratch.values[420] * 0.5)), (-230.25850929940458)), AdValue::offset(AdValue::scale(AdValue::mul(AdValue::offset(AdValue::scale(scratch.ad_value(518), (scratch.values[420] * 0.5)), (-230.25850929940458)), AdValue::offset(AdValue::scale(AdValue::offset(AdValue::scale(scratch.ad_value(518), (scratch.values[420] * 0.5)), (-230.25850929940458)), 0.3333333333333333), 1.0)), 0.5), 1.0)), 1.0), 1e100));
        }

        if ((((scratch.values[1323] != 0.0) && (scratch.values[1340] != 0.0)) && (scratch.values[1510] != 0.0)) && (scratch.values[1511] != 0.0)) {
            scratch.store_ad(1350, &AdValue::square(scratch.ad_value(1353)));
        }

    }

    pub(super) fn stamp_transient_block_15(
        &mut self,
        ctx: &GeneratedEvalContext<'_>,
        stamper: &mut GeneratedStamper<'_>,
        scratch: &mut Scratch,
    ) {
        let _ = stamper;
        if ((((scratch.values[1323] != 0.0) && (scratch.values[1340] != 0.0)) && (scratch.values[1510] != 0.0)) && (!(scratch.values[1511] != 0.0))) {
            scratch.store_ad(1350, &AdValue::mul(AdValue::offset(AdValue::scale(AdValue::sub(scratch.ad_value(518), scratch.ad_value(698)), scratch.values[420]), 1.0), scratch.ad_value(699)));
        }

        if ((((scratch.values[1323] != 0.0) && (scratch.values[1340] != 0.0)) && (scratch.values[1510] != 0.0)) && (!(scratch.values[1511] != 0.0))) {
            scratch.store_ad(1353, &AdValue::sqrt(scratch.ad_value(1350)));
        }

        if (((scratch.values[1323] != 0.0) && (scratch.values[1340] != 0.0)) && (scratch.values[1510] != 0.0)) {
            scratch.store_ad(1350, &AdValue::offset(scratch.ad_value(1350), (-1.0)));
        }

        if (((scratch.values[1323] != 0.0) && (scratch.values[1340] != 0.0)) && (scratch.values[1510] != 0.0)) {
            scratch.store_ad(1352, &AdValue::div_from_scalar(1.0, scratch.ad_value(1353)));
        }

        scratch.values[1514] = if (scratch.values[518] > 0.0) { 1.0 } else { 0.0 };

        if ((((scratch.values[1323] != 0.0) && (scratch.values[1340] != 0.0)) && (scratch.values[1510] != 0.0)) && (scratch.values[1514] != 0.0)) {
            scratch.store_ad(1354, &AdValue::scale(AdValue::ln(AdValue::add(AdValue::offset(scratch.ad_value(1352), 2.0), AdValue::sqrt(AdValue::mul(AdValue::offset(scratch.ad_value(1352), 1.0), AdValue::offset(scratch.ad_value(1352), 3.0))))), (scratch.values[419] * 2.0)));
        }

        if ((((scratch.values[1323] != 0.0) && (scratch.values[1340] != 0.0)) && (scratch.values[1510] != 0.0)) && (!(scratch.values[1514] != 0.0))) {
            scratch.store_ad(1354, &AdValue::sub(AdValue::scale(AdValue::ln(AdValue::add(AdValue::offset(AdValue::scale(scratch.ad_value(1353), 2.0), 1.0), AdValue::sqrt(AdValue::mul(AdValue::offset(scratch.ad_value(1353), 1.0), AdValue::offset(AdValue::scale(scratch.ad_value(1353), 3.0), 1.0))))), (scratch.values[419] * 2.0)), scratch.ad_value(518)));
        }

        if (((scratch.values[1323] != 0.0) && (scratch.values[1340] != 0.0)) && (scratch.values[1510] != 0.0)) {
            scratch.store_ad(1355, &AdValue::sub(scratch.ad_value(700), scratch.ad_value(1354)));
        }

        if (((scratch.values[1323] != 0.0) && (scratch.values[1340] != 0.0)) && (scratch.values[1510] != 0.0)) {
            scratch.store_ad(1356, &AdValue::scale(AdValue::sub(AdValue::add(scratch.ad_value(518), scratch.ad_value(1355)), AdValue::sqrt(AdValue::offset(AdValue::mul(AdValue::sub(scratch.ad_value(518), scratch.ad_value(1355)), AdValue::sub(scratch.ad_value(518), scratch.ad_value(1355))), ((4.0 * scratch.values[419]) * scratch.values[419])))), 0.5));
        }

        if (((scratch.values[1323] != 0.0) && (scratch.values[1340] != 0.0)) && (scratch.values[1510] != 0.0)) {
            scratch.store_ad(1357, &AdValue::scale(AdValue::sub(AdValue::add(scratch.ad_value(518), scratch.ad_value(703)), AdValue::sqrt(AdValue::offset(AdValue::mul(AdValue::sub(scratch.ad_value(518), scratch.ad_value(703)), AdValue::sub(scratch.ad_value(518), scratch.ad_value(703))), ((4.0 * scratch.values[417]) * scratch.values[417])))), 0.5));
        }

        if (((scratch.values[1323] != 0.0) && (scratch.values[1340] != 0.0)) && (scratch.values[1510] != 0.0)) {
            scratch.store_ad(1358, &AdValue::scale(AdValue::sub(scratch.ad_value(518), AdValue::sqrt(AdValue::offset(AdValue::mul(scratch.ad_value(518), scratch.ad_value(518)), ((4.0 * 1e-6) * 1e-6)))), 0.5));
        }

        scratch.values[1515] = if (scratch.values[690] == 0.0) { 1.0 } else { 0.0 };

        if (((scratch.values[1323] != 0.0) && (scratch.values[1340] != 0.0)) && (scratch.values[1515] != 0.0)) {
            scratch.values[1386] = 0.0;
            scratch.node_derivatives[1386] = [0.0; Instance::NODE_COUNT];
            scratch.branch_derivatives[1386] = [0.0; Instance::BRANCH_COUNT];
        }

        scratch.values[1516] = if (scratch.values[457] == 0.5) { 1.0 } else { 0.0 };

        if ((((scratch.values[1323] != 0.0) && (scratch.values[1340] != 0.0)) && (!(scratch.values[1515] != 0.0))) && (scratch.values[1516] != 0.0)) {
            scratch.store_ad(1359, &AdValue::sqrt(AdValue::sub_from_scalar(1.0, AdValue::scale(scratch.ad_value(1351), scratch.values[454]))));
        }

        if ((((scratch.values[1323] != 0.0) && (scratch.values[1340] != 0.0)) && (!(scratch.values[1515] != 0.0))) && (!(scratch.values[1516] != 0.0))) {
            scratch.store_ad(1359, &AdValue::powf(AdValue::sub_from_scalar(1.0, AdValue::scale(scratch.ad_value(1351), scratch.values[454])), scratch.values[457]));
        }

        if (((scratch.values[1323] != 0.0) && (scratch.values[1340] != 0.0)) && (!(scratch.values[1515] != 0.0))) {
            scratch.store_ad(1360, &AdValue::scale(scratch.ad_value(1350), scratch.values[436]));
        }

        scratch.values[1517] = if ((scratch.values[386] == 0.0) && (scratch.values[389] == 0.0)) { 1.0 } else { 0.0 };

        if ((((scratch.values[1323] != 0.0) && (scratch.values[1340] != 0.0)) && (!(scratch.values[1515] != 0.0))) && (scratch.values[1517] != 0.0)) {
            scratch.values[1361] = 0.0;
            scratch.node_derivatives[1361] = [0.0; Instance::NODE_COUNT];
            scratch.branch_derivatives[1361] = [0.0; Instance::BRANCH_COUNT];
        }

        if ((((scratch.values[1323] != 0.0) && (scratch.values[1340] != 0.0)) && (!(scratch.values[1515] != 0.0))) && (!(scratch.values[1517] != 0.0))) {
            scratch.store_ad(1362, &AdValue::sub_from_scalar(scratch.values[442], scratch.ad_value(1356)));
        }

        if ((((scratch.values[1323] != 0.0) && (scratch.values[1340] != 0.0)) && (!(scratch.values[1515] != 0.0))) && (!(scratch.values[1517] != 0.0))) {
            scratch.store_ad(1363, &AdValue::sub_from_scalar(1.0, AdValue::sqrt(AdValue::sub_from_scalar(1.0, AdValue::div(scratch.ad_value(1354), scratch.ad_value(1362))))));
        }

        scratch.values[1518] = if (scratch.values[375] == 0.5) { 1.0 } else { 0.0 };

        if (((((scratch.values[1323] != 0.0) && (scratch.values[1340] != 0.0)) && (!(scratch.values[1515] != 0.0))) && (!(scratch.values[1517] != 0.0))) && (scratch.values[1518] != 0.0)) {
            scratch.values[1364] = 0.0;
            scratch.node_derivatives[1364] = [0.0; Instance::NODE_COUNT];
            scratch.branch_derivatives[1364] = [0.0; Instance::BRANCH_COUNT];
        }

        if (((((scratch.values[1323] != 0.0) && (scratch.values[1340] != 0.0)) && (!(scratch.values[1515] != 0.0))) && (!(scratch.values[1517] != 0.0))) && (!(scratch.values[1518] != 0.0))) {
            scratch.store_ad(1364, &AdValue::scale(AdValue::add(AdValue::div(AdValue::mul(AdValue::square(scratch.ad_value(1363)), AdValue::ln(scratch.ad_value(1363))), AdValue::sub_from_scalar(1.0, scratch.ad_value(1363))), scratch.ad_value(1363)), (1.0 - (2.0 * scratch.values[375]))));
        }

        if ((((scratch.values[1323] != 0.0) && (scratch.values[1340] != 0.0)) && (!(scratch.values[1515] != 0.0))) && (!(scratch.values[1517] != 0.0))) {
            scratch.store_ad(1365, &AdValue::add(scratch.ad_value(1363), scratch.ad_value(1364)));
        }

        scratch.values[1519] = if (scratch.values[375] == 0.5) { 1.0 } else { 0.0 };

        if (((((scratch.values[1323] != 0.0) && (scratch.values[1340] != 0.0)) && (!(scratch.values[1515] != 0.0))) && (!(scratch.values[1517] != 0.0))) && (scratch.values[1519] != 0.0)) {
            scratch.store_ad(1359, &AdValue::sqrt(AdValue::scale(scratch.ad_value(1362), scratch.values[478])));
        }

        if (((((scratch.values[1323] != 0.0) && (scratch.values[1340] != 0.0)) && (!(scratch.values[1515] != 0.0))) && (!(scratch.values[1517] != 0.0))) && (!(scratch.values[1519] != 0.0))) {
            scratch.store_ad(1359, &AdValue::powf(AdValue::scale(scratch.ad_value(1362), scratch.values[478]), scratch.values[375]));
        }

        if ((((scratch.values[1323] != 0.0) && (scratch.values[1340] != 0.0)) && (!(scratch.values[1515] != 0.0))) && (!(scratch.values[1517] != 0.0))) {
            scratch.store_ad(1366, &AdValue::scale(scratch.ad_value(1359), scratch.values[472]));
        }

        if ((((scratch.values[1323] != 0.0) && (scratch.values[1340] != 0.0)) && (!(scratch.values[1515] != 0.0))) && (!(scratch.values[1517] != 0.0))) {
            scratch.store_ad(1367, &AdValue::scale(AdValue::mul(AdValue::offset(scratch.ad_value(1353), (-1.0)), scratch.ad_value(1366)), scratch.values[433]));
        }

        if ((((scratch.values[1323] != 0.0) && (scratch.values[1340] != 0.0)) && (!(scratch.values[1515] != 0.0))) && (!(scratch.values[1517] != 0.0))) {
            scratch.store_ad(1361, &AdValue::scale(AdValue::mul(scratch.ad_value(1367), scratch.ad_value(1365)), scratch.values[386]));
        }

        scratch.values[1520] = if (scratch.values[389] == 0.0) { 1.0 } else { 0.0 };

        if ((((scratch.values[1323] != 0.0) && (scratch.values[1340] != 0.0)) && (!(scratch.values[1515] != 0.0))) && (scratch.values[1520] != 0.0)) {
            scratch.values[1368] = 0.0;
            scratch.node_derivatives[1368] = [0.0; Instance::NODE_COUNT];
            scratch.branch_derivatives[1368] = [0.0; Instance::BRANCH_COUNT];
        }

        if ((((scratch.values[1323] != 0.0) && (scratch.values[1340] != 0.0)) && (!(scratch.values[1515] != 0.0))) && (!(scratch.values[1520] != 0.0))) {
            scratch.store_ad(1369, &AdValue::scale(AdValue::div(AdValue::scale(scratch.ad_value(1366), scratch.values[457]), scratch.ad_value(1362)), scratch.values[487]));
        }

        if ((((scratch.values[1323] != 0.0) && (scratch.values[1340] != 0.0)) && (!(scratch.values[1515] != 0.0))) && (!(scratch.values[1520] != 0.0))) {
            scratch.store_ad(1370, &AdValue::div_from_scalar((0.666666666666667 * scratch.values[484]), scratch.ad_value(1369)));
        }

        if ((((scratch.values[1323] != 0.0) && (scratch.values[1340] != 0.0)) && (!(scratch.values[1515] != 0.0))) && (!(scratch.values[1520] != 0.0))) {
            scratch.store_ad(1371, &AdValue::square(scratch.ad_value(1370)));
        }

        if ((((scratch.values[1323] != 0.0) && (scratch.values[1340] != 0.0)) && (!(scratch.values[1515] != 0.0))) && (!(scratch.values[1520] != 0.0))) {
            scratch.store_ad(1372, &AdValue::sqrt(AdValue::div(AdValue::square(scratch.ad_value(1371)), AdValue::offset(AdValue::square(scratch.ad_value(1371)), 1.0))));
        }

        if ((((scratch.values[1323] != 0.0) && (scratch.values[1340] != 0.0)) && (!(scratch.values[1515] != 0.0))) && (!(scratch.values[1520] != 0.0))) {
            scratch.store_ad(1373, &AdValue::sqrt(AdValue::abs(scratch.ad_value(1372))));
        }

        if ((((scratch.values[1323] != 0.0) && (scratch.values[1340] != 0.0)) && (!(scratch.values[1515] != 0.0))) && (!(scratch.values[1520] != 0.0))) {
            scratch.store_ad(1374, &AdValue::mul(scratch.ad_value(1372), scratch.ad_value(1373)));
        }

        scratch.values[1521] = if (((-scratch.values[375]) * scratch.values[460]) == (-1.0)) { 1.0 } else { 0.0 };

        if (((((scratch.values[1323] != 0.0) && (scratch.values[1340] != 0.0)) && (!(scratch.values[1515] != 0.0))) && (!(scratch.values[1520] != 0.0))) && (scratch.values[1521] != 0.0)) {
            scratch.store_ad(1375, &AdValue::div_from_scalar(1.0, AdValue::offset(AdValue::mul(scratch.ad_value(1369), scratch.ad_value(1374)), 1.0)));
        }

        if (((((scratch.values[1323] != 0.0) && (scratch.values[1340] != 0.0)) && (!(scratch.values[1515] != 0.0))) && (!(scratch.values[1520] != 0.0))) && (!(scratch.values[1521] != 0.0))) {
            scratch.store_ad(1375, &AdValue::powf(AdValue::offset(AdValue::mul(scratch.ad_value(1369), scratch.ad_value(1374)), 1.0), ((-scratch.values[375]) * scratch.values[460])));
        }

        if ((((scratch.values[1323] != 0.0) && (scratch.values[1340] != 0.0)) && (!(scratch.values[1515] != 0.0))) && (!(scratch.values[1520] != 0.0))) {
            scratch.store_ad(1376, &AdValue::div(AdValue::mul(scratch.ad_value(1365), scratch.ad_value(1375)), AdValue::add(scratch.ad_value(1365), scratch.ad_value(1375))));
        }

        if ((((scratch.values[1323] != 0.0) && (scratch.values[1340] != 0.0)) && (!(scratch.values[1515] != 0.0))) && (!(scratch.values[1520] != 0.0))) {
            scratch.store_ad(1377, &AdValue::sqrt(AdValue::scale(AdValue::div(scratch.ad_value(1369), scratch.ad_value(1373)), 0.375)));
        }

        if ((((scratch.values[1323] != 0.0) && (scratch.values[1340] != 0.0)) && (!(scratch.values[1515] != 0.0))) && (!(scratch.values[1520] != 0.0))) {
            scratch.store_ad(1378, &AdValue::sub(AdValue::scale(AdValue::mul(scratch.ad_value(1370), scratch.ad_value(1373)), 2.0), scratch.ad_value(1372)));
        }

        if ((((scratch.values[1323] != 0.0) && (scratch.values[1340] != 0.0)) && (!(scratch.values[1515] != 0.0))) && (!(scratch.values[1520] != 0.0))) {
            scratch.store_ad(1379, &AdValue::add(AdValue::sub(AdValue::mul(AdValue::scale(scratch.ad_value(1370), scratch.values[484]), scratch.ad_value(1373)), AdValue::scale(scratch.ad_value(1372), scratch.values[484])), AdValue::scale(AdValue::mul(scratch.ad_value(1369), scratch.ad_value(1374)), 0.5)));
        }

        if ((((scratch.values[1323] != 0.0) && (scratch.values[1340] != 0.0)) && (!(scratch.values[1515] != 0.0))) && (!(scratch.values[1520] != 0.0))) {
            scratch.store_ad(1380, &AdValue::mul(AdValue::offset(scratch.ad_value(1378), (-1.0)), scratch.ad_value(1377)));
        }

        if ((((scratch.values[1323] != 0.0) && (scratch.values[1340] != 0.0)) && (!(scratch.values[1515] != 0.0))) && (!(scratch.values[1520] != 0.0))) {
            scratch.store_ad(1341, &AdValue::square(scratch.ad_value(1380)));
        }

        scratch.values[1522] = if (scratch.values[1380] > 0.0) { 1.0 } else { 0.0 };

        if (((((scratch.values[1323] != 0.0) && (scratch.values[1340] != 0.0)) && (!(scratch.values[1515] != 0.0))) && (!(scratch.values[1520] != 0.0))) && (scratch.values[1522] != 0.0)) {
            scratch.store_ad(1342, &AdValue::div_from_scalar(1.0, AdValue::offset(AdValue::scale(scratch.ad_value(1380), scratch.values[421]), 1.0)));
        }

        if (((((scratch.values[1323] != 0.0) && (scratch.values[1340] != 0.0)) && (!(scratch.values[1515] != 0.0))) && (!(scratch.values[1520] != 0.0))) && (!(scratch.values[1522] != 0.0))) {
            scratch.store_ad(1342, &AdValue::div_from_scalar(1.0, AdValue::sub_from_scalar(1.0, AdValue::scale(scratch.ad_value(1380), scratch.values[421]))));
        }

        scratch.values[1523] = if (((-scratch.values[1341]) + scratch.values[1379]) > (-230.25850929940458)) { 1.0 } else { 0.0 };

        if (((((scratch.values[1323] != 0.0) && (scratch.values[1340] != 0.0)) && (!(scratch.values[1515] != 0.0))) && (!(scratch.values[1520] != 0.0))) && (scratch.values[1523] != 0.0)) {
            scratch.store_ad(1359, &AdValue::exp(AdValue::sub(scratch.ad_value(1379), scratch.ad_value(1341))));
        }

        if (((((scratch.values[1323] != 0.0) && (scratch.values[1340] != 0.0)) && (!(scratch.values[1515] != 0.0))) && (!(scratch.values[1520] != 0.0))) && (!(scratch.values[1523] != 0.0))) {
            let assign19600_ad_e19675: AdValue = AdValue::div_from_scalar(1e-100, AdValue::offset(AdValue::mul(AdValue::sub_from_scalar((-230.25850929940458), AdValue::sub(scratch.ad_value(1379), scratch.ad_value(1341))), AdValue::offset(AdValue::scale(AdValue::mul(AdValue::sub_from_scalar((-230.25850929940458), AdValue::sub(scratch.ad_value(1379), scratch.ad_value(1341))), AdValue::offset(AdValue::scale(AdValue::sub_from_scalar((-230.25850929940458), AdValue::sub(scratch.ad_value(1379), scratch.ad_value(1341))), 0.3333333333333333), 1.0)), 0.5), 1.0)), 1.0));
            scratch.store_ad(1359, &assign19600_ad_e19675);
        }

        if ((((scratch.values[1323] != 0.0) && (scratch.values[1340] != 0.0)) && (!(scratch.values[1515] != 0.0))) && (!(scratch.values[1520] != 0.0))) {
            scratch.store_ad(1343, &AdValue::mul(AdValue::add(AdValue::add(AdValue::scale(scratch.ad_value(1342), 0.29214664), AdValue::scale(AdValue::square(scratch.ad_value(1342)), scratch.values[422])), AdValue::scale(AdValue::mul(AdValue::square(scratch.ad_value(1342)), scratch.ad_value(1342)), scratch.values[423])), scratch.ad_value(1359)));
        }

        scratch.values[1524] = if (scratch.values[1380] > 0.0) { 1.0 } else { 0.0 };

        if (((((scratch.values[1323] != 0.0) && (scratch.values[1340] != 0.0)) && (!(scratch.values[1515] != 0.0))) && (!(scratch.values[1520] != 0.0))) && (scratch.values[1524] != 0.0)) {
            scratch.values[1381] = scratch.values[1343];
            scratch.node_derivatives[1381] = scratch.node_derivatives[1343];
            scratch.branch_derivatives[1381] = scratch.branch_derivatives[1343];
        }

        scratch.values[1525] = if (scratch.values[1379] > (-230.25850929940458)) { 1.0 } else { 0.0 };

        if ((((((scratch.values[1323] != 0.0) && (scratch.values[1340] != 0.0)) && (!(scratch.values[1515] != 0.0))) && (!(scratch.values[1520] != 0.0))) && (!(scratch.values[1524] != 0.0))) && (scratch.values[1525] != 0.0)) {
            scratch.store_ad(1359, &AdValue::exp(scratch.ad_value(1379)));
        }

        if ((((((scratch.values[1323] != 0.0) && (scratch.values[1340] != 0.0)) && (!(scratch.values[1515] != 0.0))) && (!(scratch.values[1520] != 0.0))) && (!(scratch.values[1524] != 0.0))) && (!(scratch.values[1525] != 0.0))) {
            scratch.store_ad(1359, &AdValue::div_from_scalar(1e-100, AdValue::offset(AdValue::mul(AdValue::sub_from_scalar((-230.25850929940458), scratch.ad_value(1379)), AdValue::offset(AdValue::scale(AdValue::mul(AdValue::sub_from_scalar((-230.25850929940458), scratch.ad_value(1379)), AdValue::offset(AdValue::scale(AdValue::sub_from_scalar((-230.25850929940458), scratch.ad_value(1379)), 0.3333333333333333), 1.0)), 0.5), 1.0)), 1.0)));
        }

        if (((((scratch.values[1323] != 0.0) && (scratch.values[1340] != 0.0)) && (!(scratch.values[1515] != 0.0))) && (!(scratch.values[1520] != 0.0))) && (!(scratch.values[1524] != 0.0))) {
            scratch.store_ad(1381, &AdValue::sub(AdValue::scale(scratch.ad_value(1359), 2.0), scratch.ad_value(1343)));
        }

        if ((((scratch.values[1323] != 0.0) && (scratch.values[1340] != 0.0)) && (!(scratch.values[1515] != 0.0))) && (!(scratch.values[1520] != 0.0))) {
            scratch.store_ad(1382, &AdValue::scale(AdValue::div(AdValue::scale(scratch.ad_value(1381), scratch.values[484]), scratch.ad_value(1377)), (1.772453850905516 * 0.5)));
        }

        if ((((scratch.values[1323] != 0.0) && (scratch.values[1340] != 0.0)) && (!(scratch.values[1515] != 0.0))) && (!(scratch.values[1520] != 0.0))) {
            scratch.store_ad(1368, &AdValue::scale(AdValue::mul(AdValue::mul(scratch.ad_value(1367), scratch.ad_value(1382)), scratch.ad_value(1376)), scratch.values[389]));
        }

        scratch.values[1526] = if (scratch.values[395] == 0.0) { 1.0 } else { 0.0 };

        if ((((scratch.values[1323] != 0.0) && (scratch.values[1340] != 0.0)) && (!(scratch.values[1515] != 0.0))) && (scratch.values[1526] != 0.0)) {
            scratch.values[1383] = 0.0;
            scratch.node_derivatives[1383] = [0.0; Instance::NODE_COUNT];
            scratch.branch_derivatives[1383] = [0.0; Instance::BRANCH_COUNT];
        }

        scratch.values[1527] = if (scratch.values[375] == 0.5) { 1.0 } else { 0.0 };

        if (((((scratch.values[1323] != 0.0) && (scratch.values[1340] != 0.0)) && (!(scratch.values[1515] != 0.0))) && (!(scratch.values[1526] != 0.0))) && (scratch.values[1527] != 0.0)) {
            scratch.store_ad(1359, &AdValue::sqrt(AdValue::scale(AdValue::sub_from_scalar(scratch.values[372], scratch.ad_value(1357)), scratch.values[478])));
        }

        if (((((scratch.values[1323] != 0.0) && (scratch.values[1340] != 0.0)) && (!(scratch.values[1515] != 0.0))) && (!(scratch.values[1526] != 0.0))) && (!(scratch.values[1527] != 0.0))) {
            scratch.store_ad(1359, &AdValue::powf(AdValue::scale(AdValue::sub_from_scalar(scratch.values[372], scratch.ad_value(1357)), scratch.values[478]), scratch.values[375]));
        }

        if ((((scratch.values[1323] != 0.0) && (scratch.values[1340] != 0.0)) && (!(scratch.values[1515] != 0.0))) && (!(scratch.values[1526] != 0.0))) {
            scratch.store_ad(1384, &AdValue::scale(AdValue::div(AdValue::scale(AdValue::sub_from_scalar(scratch.values[372], scratch.ad_value(1357)), scratch.values[475]), scratch.ad_value(1359)), scratch.values[460]));
        }

        scratch.values[1528] = if (((((-scratch.values[490]) / scratch.values[1384])) as f64).abs() < 230.25850929940458) { 1.0 } else { 0.0 };

        if (((((scratch.values[1323] != 0.0) && (scratch.values[1340] != 0.0)) && (!(scratch.values[1515] != 0.0))) && (!(scratch.values[1526] != 0.0))) && (scratch.values[1528] != 0.0)) {
            scratch.store_ad(1359, &AdValue::exp(AdValue::div(AdValue::neg(scratch.ad_value(490)), scratch.ad_value(1384))));
        }

        scratch.values[1529] = if (((-scratch.values[490]) / scratch.values[1384]) < 0.0) { 1.0 } else { 0.0 };

        if ((((((scratch.values[1323] != 0.0) && (scratch.values[1340] != 0.0)) && (!(scratch.values[1515] != 0.0))) && (!(scratch.values[1526] != 0.0))) && (!(scratch.values[1528] != 0.0))) && (scratch.values[1529] != 0.0)) {
            let assign19790_ad_e20002: AdValue = AdValue::offset(AdValue::mul(AdValue::sub_from_scalar((-230.25850929940458), AdValue::div(AdValue::neg(scratch.ad_value(490)), scratch.ad_value(1384))), AdValue::offset(AdValue::scale(AdValue::mul(AdValue::sub_from_scalar((-230.25850929940458), AdValue::div(AdValue::neg(scratch.ad_value(490)), scratch.ad_value(1384))), AdValue::offset(AdValue::scale(AdValue::sub_from_scalar((-230.25850929940458), AdValue::div(AdValue::neg(scratch.ad_value(490)), scratch.ad_value(1384))), 0.3333333333333333), 1.0)), 0.5), 1.0)), 1.0);
            scratch.store_ad(1359, &AdValue::div_from_scalar(1e-100, assign19790_ad_e20002));
        }

        if ((((((scratch.values[1323] != 0.0) && (scratch.values[1340] != 0.0)) && (!(scratch.values[1515] != 0.0))) && (!(scratch.values[1526] != 0.0))) && (!(scratch.values[1528] != 0.0))) && (!(scratch.values[1529] != 0.0))) {
            let assign19800_ad_e20052: AdValue = AdValue::scale(AdValue::offset(AdValue::mul(AdValue::offset(AdValue::div(AdValue::neg(scratch.ad_value(490)), scratch.ad_value(1384)), (-230.25850929940458)), AdValue::offset(AdValue::scale(AdValue::mul(AdValue::offset(AdValue::div(AdValue::neg(scratch.ad_value(490)), scratch.ad_value(1384)), (-230.25850929940458)), AdValue::offset(AdValue::scale(AdValue::offset(AdValue::div(AdValue::neg(scratch.ad_value(490)), scratch.ad_value(1384)), (-230.25850929940458)), 0.3333333333333333), 1.0)), 0.5), 1.0)), 1.0), 1e100);
            scratch.store_ad(1359, &assign19800_ad_e20052);
        }

        if ((((scratch.values[1323] != 0.0) && (scratch.values[1340] != 0.0)) && (!(scratch.values[1515] != 0.0))) && (!(scratch.values[1526] != 0.0))) {
            scratch.store_ad(1383, &AdValue::scale(AdValue::mul(AdValue::mul(AdValue::mul(scratch.ad_value(518), scratch.ad_value(1384)), scratch.ad_value(1384)), scratch.ad_value(1359)), scratch.values[395]));
        }

        scratch.values[1530] = if (scratch.values[404] > 1000.0) { 1.0 } else { 0.0 };

        if ((((scratch.values[1323] != 0.0) && (scratch.values[1340] != 0.0)) && (!(scratch.values[1515] != 0.0))) && (scratch.values[1530] != 0.0)) {
            scratch.values[1385] = 1.0;
            scratch.node_derivatives[1385] = [0.0; Instance::NODE_COUNT];
            scratch.branch_derivatives[1385] = [0.0; Instance::BRANCH_COUNT];
        }

        scratch.values[1531] = if (scratch.values[1358] > ((-scratch.values[493]) * scratch.values[404])) { 1.0 } else { 0.0 };

        scratch.values[1532] = if (scratch.values[407] == 4.0) { 1.0 } else { 0.0 };

        if ((((((scratch.values[1323] != 0.0) && (scratch.values[1340] != 0.0)) && (!(scratch.values[1515] != 0.0))) && (!(scratch.values[1530] != 0.0))) && (scratch.values[1531] != 0.0)) && (scratch.values[1532] != 0.0)) {
            scratch.store_ad(1359, &AdValue::mul(AdValue::mul(AdValue::mul(AdValue::scale(scratch.ad_value(1358), scratch.values[497]), AdValue::scale(scratch.ad_value(1358), scratch.values[497])), AdValue::scale(scratch.ad_value(1358), scratch.values[497])), AdValue::scale(scratch.ad_value(1358), scratch.values[497])));
        }

        if ((((((scratch.values[1323] != 0.0) && (scratch.values[1340] != 0.0)) && (!(scratch.values[1515] != 0.0))) && (!(scratch.values[1530] != 0.0))) && (scratch.values[1531] != 0.0)) && (!(scratch.values[1532] != 0.0))) {
            scratch.store_ad(1359, &AdValue::powf(AdValue::abs(AdValue::scale(scratch.ad_value(1358), scratch.values[497])), scratch.values[407]));
        }

        if (((((scratch.values[1323] != 0.0) && (scratch.values[1340] != 0.0)) && (!(scratch.values[1515] != 0.0))) && (!(scratch.values[1530] != 0.0))) && (scratch.values[1531] != 0.0)) {
            scratch.store_ad(1385, &AdValue::div_from_scalar(1.0, AdValue::sub_from_scalar(1.0, scratch.ad_value(1359))));
        }

        if (((((scratch.values[1323] != 0.0) && (scratch.values[1340] != 0.0)) && (!(scratch.values[1515] != 0.0))) && (!(scratch.values[1530] != 0.0))) && (!(scratch.values[1531] != 0.0))) {
            scratch.store_ad(1385, &AdValue::offset(AdValue::scale(AdValue::offset(scratch.ad_value(1358), (scratch.values[493] * scratch.values[404])), scratch.values[500]), scratch.values[494]));
        }

        if (((scratch.values[1323] != 0.0) && (scratch.values[1340] != 0.0)) && (!(scratch.values[1515] != 0.0))) {
            scratch.store_ad(1386, &AdValue::mul(AdValue::add(AdValue::add(AdValue::add(scratch.ad_value(1360), scratch.ad_value(1361)), scratch.ad_value(1368)), scratch.ad_value(1383)), scratch.ad_value(1385)));
        }

        scratch.values[1533] = if (scratch.values[691] == 0.0) { 1.0 } else { 0.0 };

        if (((scratch.values[1323] != 0.0) && (scratch.values[1340] != 0.0)) && (scratch.values[1533] != 0.0)) {
            scratch.values[1387] = 0.0;
            scratch.node_derivatives[1387] = [0.0; Instance::NODE_COUNT];
            scratch.branch_derivatives[1387] = [0.0; Instance::BRANCH_COUNT];
        }

        scratch.values[1534] = if (scratch.values[458] == 0.5) { 1.0 } else { 0.0 };

        if ((((scratch.values[1323] != 0.0) && (scratch.values[1340] != 0.0)) && (!(scratch.values[1533] != 0.0))) && (scratch.values[1534] != 0.0)) {
            scratch.store_ad(1359, &AdValue::sqrt(AdValue::sub_from_scalar(1.0, AdValue::scale(scratch.ad_value(1351), scratch.values[455]))));
        }

        if ((((scratch.values[1323] != 0.0) && (scratch.values[1340] != 0.0)) && (!(scratch.values[1533] != 0.0))) && (!(scratch.values[1534] != 0.0))) {
            scratch.store_ad(1359, &AdValue::powf(AdValue::sub_from_scalar(1.0, AdValue::scale(scratch.ad_value(1351), scratch.values[455])), scratch.values[458]));
        }

        if (((scratch.values[1323] != 0.0) && (scratch.values[1340] != 0.0)) && (!(scratch.values[1533] != 0.0))) {
            scratch.store_ad(1360, &AdValue::scale(scratch.ad_value(1350), scratch.values[437]));
        }

        scratch.values[1535] = if ((scratch.values[387] == 0.0) && (scratch.values[390] == 0.0)) { 1.0 } else { 0.0 };

        if ((((scratch.values[1323] != 0.0) && (scratch.values[1340] != 0.0)) && (!(scratch.values[1533] != 0.0))) && (scratch.values[1535] != 0.0)) {
            scratch.values[1361] = 0.0;
            scratch.node_derivatives[1361] = [0.0; Instance::NODE_COUNT];
            scratch.branch_derivatives[1361] = [0.0; Instance::BRANCH_COUNT];
        }

        if ((((scratch.values[1323] != 0.0) && (scratch.values[1340] != 0.0)) && (!(scratch.values[1533] != 0.0))) && (!(scratch.values[1535] != 0.0))) {
            scratch.store_ad(1362, &AdValue::sub_from_scalar(scratch.values[443], scratch.ad_value(1356)));
        }

        if ((((scratch.values[1323] != 0.0) && (scratch.values[1340] != 0.0)) && (!(scratch.values[1533] != 0.0))) && (!(scratch.values[1535] != 0.0))) {
            scratch.store_ad(1363, &AdValue::sub_from_scalar(1.0, AdValue::sqrt(AdValue::sub_from_scalar(1.0, AdValue::div(scratch.ad_value(1354), scratch.ad_value(1362))))));
        }

        scratch.values[1536] = if (scratch.values[376] == 0.5) { 1.0 } else { 0.0 };

        if (((((scratch.values[1323] != 0.0) && (scratch.values[1340] != 0.0)) && (!(scratch.values[1533] != 0.0))) && (!(scratch.values[1535] != 0.0))) && (scratch.values[1536] != 0.0)) {
            scratch.values[1364] = 0.0;
            scratch.node_derivatives[1364] = [0.0; Instance::NODE_COUNT];
            scratch.branch_derivatives[1364] = [0.0; Instance::BRANCH_COUNT];
        }

        if (((((scratch.values[1323] != 0.0) && (scratch.values[1340] != 0.0)) && (!(scratch.values[1533] != 0.0))) && (!(scratch.values[1535] != 0.0))) && (!(scratch.values[1536] != 0.0))) {
            scratch.store_ad(1364, &AdValue::scale(AdValue::add(AdValue::div(AdValue::mul(AdValue::square(scratch.ad_value(1363)), AdValue::ln(scratch.ad_value(1363))), AdValue::sub_from_scalar(1.0, scratch.ad_value(1363))), scratch.ad_value(1363)), (1.0 - (2.0 * scratch.values[376]))));
        }

        if ((((scratch.values[1323] != 0.0) && (scratch.values[1340] != 0.0)) && (!(scratch.values[1533] != 0.0))) && (!(scratch.values[1535] != 0.0))) {
            scratch.store_ad(1365, &AdValue::add(scratch.ad_value(1363), scratch.ad_value(1364)));
        }

        scratch.values[1537] = if (scratch.values[376] == 0.5) { 1.0 } else { 0.0 };

        if (((((scratch.values[1323] != 0.0) && (scratch.values[1340] != 0.0)) && (!(scratch.values[1533] != 0.0))) && (!(scratch.values[1535] != 0.0))) && (scratch.values[1537] != 0.0)) {
            scratch.store_ad(1359, &AdValue::sqrt(AdValue::scale(scratch.ad_value(1362), scratch.values[479])));
        }

        if (((((scratch.values[1323] != 0.0) && (scratch.values[1340] != 0.0)) && (!(scratch.values[1533] != 0.0))) && (!(scratch.values[1535] != 0.0))) && (!(scratch.values[1537] != 0.0))) {
            scratch.store_ad(1359, &AdValue::powf(AdValue::scale(scratch.ad_value(1362), scratch.values[479]), scratch.values[376]));
        }

        if ((((scratch.values[1323] != 0.0) && (scratch.values[1340] != 0.0)) && (!(scratch.values[1533] != 0.0))) && (!(scratch.values[1535] != 0.0))) {
            scratch.store_ad(1366, &AdValue::scale(scratch.ad_value(1359), scratch.values[473]));
        }

        if ((((scratch.values[1323] != 0.0) && (scratch.values[1340] != 0.0)) && (!(scratch.values[1533] != 0.0))) && (!(scratch.values[1535] != 0.0))) {
            scratch.store_ad(1367, &AdValue::scale(AdValue::mul(AdValue::offset(scratch.ad_value(1353), (-1.0)), scratch.ad_value(1366)), scratch.values[434]));
        }

        if ((((scratch.values[1323] != 0.0) && (scratch.values[1340] != 0.0)) && (!(scratch.values[1533] != 0.0))) && (!(scratch.values[1535] != 0.0))) {
            scratch.store_ad(1361, &AdValue::scale(AdValue::mul(scratch.ad_value(1367), scratch.ad_value(1365)), scratch.values[387]));
        }

        scratch.values[1538] = if (scratch.values[390] == 0.0) { 1.0 } else { 0.0 };

        if ((((scratch.values[1323] != 0.0) && (scratch.values[1340] != 0.0)) && (!(scratch.values[1533] != 0.0))) && (scratch.values[1538] != 0.0)) {
            scratch.values[1368] = 0.0;
            scratch.node_derivatives[1368] = [0.0; Instance::NODE_COUNT];
            scratch.branch_derivatives[1368] = [0.0; Instance::BRANCH_COUNT];
        }

        if ((((scratch.values[1323] != 0.0) && (scratch.values[1340] != 0.0)) && (!(scratch.values[1533] != 0.0))) && (!(scratch.values[1538] != 0.0))) {
            scratch.store_ad(1369, &AdValue::scale(AdValue::div(AdValue::scale(scratch.ad_value(1366), scratch.values[458]), scratch.ad_value(1362)), scratch.values[488]));
        }

        if ((((scratch.values[1323] != 0.0) && (scratch.values[1340] != 0.0)) && (!(scratch.values[1533] != 0.0))) && (!(scratch.values[1538] != 0.0))) {
            scratch.store_ad(1370, &AdValue::div_from_scalar((0.666666666666667 * scratch.values[485]), scratch.ad_value(1369)));
        }

        if ((((scratch.values[1323] != 0.0) && (scratch.values[1340] != 0.0)) && (!(scratch.values[1533] != 0.0))) && (!(scratch.values[1538] != 0.0))) {
            scratch.store_ad(1371, &AdValue::square(scratch.ad_value(1370)));
        }

        if ((((scratch.values[1323] != 0.0) && (scratch.values[1340] != 0.0)) && (!(scratch.values[1533] != 0.0))) && (!(scratch.values[1538] != 0.0))) {
            scratch.store_ad(1372, &AdValue::sqrt(AdValue::div(AdValue::square(scratch.ad_value(1371)), AdValue::offset(AdValue::square(scratch.ad_value(1371)), 1.0))));
        }

        if ((((scratch.values[1323] != 0.0) && (scratch.values[1340] != 0.0)) && (!(scratch.values[1533] != 0.0))) && (!(scratch.values[1538] != 0.0))) {
            scratch.store_ad(1373, &AdValue::sqrt(AdValue::abs(scratch.ad_value(1372))));
        }

        if ((((scratch.values[1323] != 0.0) && (scratch.values[1340] != 0.0)) && (!(scratch.values[1533] != 0.0))) && (!(scratch.values[1538] != 0.0))) {
            scratch.store_ad(1374, &AdValue::mul(scratch.ad_value(1372), scratch.ad_value(1373)));
        }

        scratch.values[1539] = if (((-scratch.values[376]) * scratch.values[461]) == (-1.0)) { 1.0 } else { 0.0 };

        if (((((scratch.values[1323] != 0.0) && (scratch.values[1340] != 0.0)) && (!(scratch.values[1533] != 0.0))) && (!(scratch.values[1538] != 0.0))) && (scratch.values[1539] != 0.0)) {
            scratch.store_ad(1375, &AdValue::div_from_scalar(1.0, AdValue::offset(AdValue::mul(scratch.ad_value(1369), scratch.ad_value(1374)), 1.0)));
        }

        if (((((scratch.values[1323] != 0.0) && (scratch.values[1340] != 0.0)) && (!(scratch.values[1533] != 0.0))) && (!(scratch.values[1538] != 0.0))) && (!(scratch.values[1539] != 0.0))) {
            scratch.store_ad(1375, &AdValue::powf(AdValue::offset(AdValue::mul(scratch.ad_value(1369), scratch.ad_value(1374)), 1.0), ((-scratch.values[376]) * scratch.values[461])));
        }

        if ((((scratch.values[1323] != 0.0) && (scratch.values[1340] != 0.0)) && (!(scratch.values[1533] != 0.0))) && (!(scratch.values[1538] != 0.0))) {
            scratch.store_ad(1376, &AdValue::div(AdValue::mul(scratch.ad_value(1365), scratch.ad_value(1375)), AdValue::add(scratch.ad_value(1365), scratch.ad_value(1375))));
        }

        if ((((scratch.values[1323] != 0.0) && (scratch.values[1340] != 0.0)) && (!(scratch.values[1533] != 0.0))) && (!(scratch.values[1538] != 0.0))) {
            scratch.store_ad(1377, &AdValue::sqrt(AdValue::scale(AdValue::div(scratch.ad_value(1369), scratch.ad_value(1373)), 0.375)));
        }

        if ((((scratch.values[1323] != 0.0) && (scratch.values[1340] != 0.0)) && (!(scratch.values[1533] != 0.0))) && (!(scratch.values[1538] != 0.0))) {
            scratch.store_ad(1378, &AdValue::sub(AdValue::scale(AdValue::mul(scratch.ad_value(1370), scratch.ad_value(1373)), 2.0), scratch.ad_value(1372)));
        }

        if ((((scratch.values[1323] != 0.0) && (scratch.values[1340] != 0.0)) && (!(scratch.values[1533] != 0.0))) && (!(scratch.values[1538] != 0.0))) {
            scratch.store_ad(1379, &AdValue::add(AdValue::sub(AdValue::mul(AdValue::scale(scratch.ad_value(1370), scratch.values[485]), scratch.ad_value(1373)), AdValue::scale(scratch.ad_value(1372), scratch.values[485])), AdValue::scale(AdValue::mul(scratch.ad_value(1369), scratch.ad_value(1374)), 0.5)));
        }

        if ((((scratch.values[1323] != 0.0) && (scratch.values[1340] != 0.0)) && (!(scratch.values[1533] != 0.0))) && (!(scratch.values[1538] != 0.0))) {
            scratch.store_ad(1380, &AdValue::mul(AdValue::offset(scratch.ad_value(1378), (-1.0)), scratch.ad_value(1377)));
        }

        if ((((scratch.values[1323] != 0.0) && (scratch.values[1340] != 0.0)) && (!(scratch.values[1533] != 0.0))) && (!(scratch.values[1538] != 0.0))) {
            scratch.store_ad(1341, &AdValue::square(scratch.ad_value(1380)));
        }

        scratch.values[1540] = if (scratch.values[1380] > 0.0) { 1.0 } else { 0.0 };

        if (((((scratch.values[1323] != 0.0) && (scratch.values[1340] != 0.0)) && (!(scratch.values[1533] != 0.0))) && (!(scratch.values[1538] != 0.0))) && (scratch.values[1540] != 0.0)) {
            scratch.store_ad(1342, &AdValue::div_from_scalar(1.0, AdValue::offset(AdValue::scale(scratch.ad_value(1380), scratch.values[421]), 1.0)));
        }

        if (((((scratch.values[1323] != 0.0) && (scratch.values[1340] != 0.0)) && (!(scratch.values[1533] != 0.0))) && (!(scratch.values[1538] != 0.0))) && (!(scratch.values[1540] != 0.0))) {
            scratch.store_ad(1342, &AdValue::div_from_scalar(1.0, AdValue::sub_from_scalar(1.0, AdValue::scale(scratch.ad_value(1380), scratch.values[421]))));
        }

        scratch.values[1541] = if (((-scratch.values[1341]) + scratch.values[1379]) > (-230.25850929940458)) { 1.0 } else { 0.0 };

        if (((((scratch.values[1323] != 0.0) && (scratch.values[1340] != 0.0)) && (!(scratch.values[1533] != 0.0))) && (!(scratch.values[1538] != 0.0))) && (scratch.values[1541] != 0.0)) {
            scratch.store_ad(1359, &AdValue::exp(AdValue::sub(scratch.ad_value(1379), scratch.ad_value(1341))));
        }

        if (((((scratch.values[1323] != 0.0) && (scratch.values[1340] != 0.0)) && (!(scratch.values[1533] != 0.0))) && (!(scratch.values[1538] != 0.0))) && (!(scratch.values[1541] != 0.0))) {
            let assign20350_ad_e20881: AdValue = AdValue::div_from_scalar(1e-100, AdValue::offset(AdValue::mul(AdValue::sub_from_scalar((-230.25850929940458), AdValue::sub(scratch.ad_value(1379), scratch.ad_value(1341))), AdValue::offset(AdValue::scale(AdValue::mul(AdValue::sub_from_scalar((-230.25850929940458), AdValue::sub(scratch.ad_value(1379), scratch.ad_value(1341))), AdValue::offset(AdValue::scale(AdValue::sub_from_scalar((-230.25850929940458), AdValue::sub(scratch.ad_value(1379), scratch.ad_value(1341))), 0.3333333333333333), 1.0)), 0.5), 1.0)), 1.0));
            scratch.store_ad(1359, &assign20350_ad_e20881);
        }

        if ((((scratch.values[1323] != 0.0) && (scratch.values[1340] != 0.0)) && (!(scratch.values[1533] != 0.0))) && (!(scratch.values[1538] != 0.0))) {
            scratch.store_ad(1343, &AdValue::mul(AdValue::add(AdValue::add(AdValue::scale(scratch.ad_value(1342), 0.29214664), AdValue::scale(AdValue::square(scratch.ad_value(1342)), scratch.values[422])), AdValue::scale(AdValue::mul(AdValue::square(scratch.ad_value(1342)), scratch.ad_value(1342)), scratch.values[423])), scratch.ad_value(1359)));
        }

        scratch.values[1542] = if (scratch.values[1380] > 0.0) { 1.0 } else { 0.0 };

        if (((((scratch.values[1323] != 0.0) && (scratch.values[1340] != 0.0)) && (!(scratch.values[1533] != 0.0))) && (!(scratch.values[1538] != 0.0))) && (scratch.values[1542] != 0.0)) {
            scratch.values[1381] = scratch.values[1343];
            scratch.node_derivatives[1381] = scratch.node_derivatives[1343];
            scratch.branch_derivatives[1381] = scratch.branch_derivatives[1343];
        }

        scratch.values[1543] = if (scratch.values[1379] > (-230.25850929940458)) { 1.0 } else { 0.0 };

        if ((((((scratch.values[1323] != 0.0) && (scratch.values[1340] != 0.0)) && (!(scratch.values[1533] != 0.0))) && (!(scratch.values[1538] != 0.0))) && (!(scratch.values[1542] != 0.0))) && (scratch.values[1543] != 0.0)) {
            scratch.store_ad(1359, &AdValue::exp(scratch.ad_value(1379)));
        }

        if ((((((scratch.values[1323] != 0.0) && (scratch.values[1340] != 0.0)) && (!(scratch.values[1533] != 0.0))) && (!(scratch.values[1538] != 0.0))) && (!(scratch.values[1542] != 0.0))) && (!(scratch.values[1543] != 0.0))) {
            scratch.store_ad(1359, &AdValue::div_from_scalar(1e-100, AdValue::offset(AdValue::mul(AdValue::sub_from_scalar((-230.25850929940458), scratch.ad_value(1379)), AdValue::offset(AdValue::scale(AdValue::mul(AdValue::sub_from_scalar((-230.25850929940458), scratch.ad_value(1379)), AdValue::offset(AdValue::scale(AdValue::sub_from_scalar((-230.25850929940458), scratch.ad_value(1379)), 0.3333333333333333), 1.0)), 0.5), 1.0)), 1.0)));
        }

        if (((((scratch.values[1323] != 0.0) && (scratch.values[1340] != 0.0)) && (!(scratch.values[1533] != 0.0))) && (!(scratch.values[1538] != 0.0))) && (!(scratch.values[1542] != 0.0))) {
            scratch.store_ad(1381, &AdValue::sub(AdValue::scale(scratch.ad_value(1359), 2.0), scratch.ad_value(1343)));
        }

        if ((((scratch.values[1323] != 0.0) && (scratch.values[1340] != 0.0)) && (!(scratch.values[1533] != 0.0))) && (!(scratch.values[1538] != 0.0))) {
            scratch.store_ad(1382, &AdValue::scale(AdValue::div(AdValue::scale(scratch.ad_value(1381), scratch.values[485]), scratch.ad_value(1377)), (1.772453850905516 * 0.5)));
        }

        if ((((scratch.values[1323] != 0.0) && (scratch.values[1340] != 0.0)) && (!(scratch.values[1533] != 0.0))) && (!(scratch.values[1538] != 0.0))) {
            scratch.store_ad(1368, &AdValue::scale(AdValue::mul(AdValue::mul(scratch.ad_value(1367), scratch.ad_value(1382)), scratch.ad_value(1376)), scratch.values[390]));
        }

    }
}
