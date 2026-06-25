#![allow(dead_code, unused_assignments, unused_imports, unused_parens, unused_variables)]

use super::{AdValue, GeneratedEvalContext, GeneratedReactiveStamper, GeneratedStamper, Scratch, THERMAL_VOLTAGE_PER_K};
use super::super::state::Instance;

impl Instance {

    pub(super) fn stamp_transient_block_16(
        &mut self,
        ctx: &GeneratedEvalContext<'_>,
        stamper: &mut GeneratedStamper<'_>,
        scratch: &mut Scratch,
    ) {
        let _ = stamper;
        scratch.values[1544] = if (scratch.values[396] == 0.0) { 1.0 } else { 0.0 };

        if ((((scratch.values[1323] != 0.0) && (scratch.values[1340] != 0.0)) && (!(scratch.values[1533] != 0.0))) && (scratch.values[1544] != 0.0)) {
            scratch.values[1383] = 0.0;
            scratch.node_derivatives[1383] = [0.0; Instance::NODE_COUNT];
            scratch.branch_derivatives[1383] = [0.0; Instance::BRANCH_COUNT];
        }

        scratch.values[1545] = if (scratch.values[376] == 0.5) { 1.0 } else { 0.0 };

        if (((((scratch.values[1323] != 0.0) && (scratch.values[1340] != 0.0)) && (!(scratch.values[1533] != 0.0))) && (!(scratch.values[1544] != 0.0))) && (scratch.values[1545] != 0.0)) {
            scratch.store_ad(1359, &AdValue::sqrt(AdValue::scale(AdValue::sub_from_scalar(scratch.values[373], scratch.ad_value(1357)), scratch.values[479])));
        }

        if (((((scratch.values[1323] != 0.0) && (scratch.values[1340] != 0.0)) && (!(scratch.values[1533] != 0.0))) && (!(scratch.values[1544] != 0.0))) && (!(scratch.values[1545] != 0.0))) {
            scratch.store_ad(1359, &AdValue::powf(AdValue::scale(AdValue::sub_from_scalar(scratch.values[373], scratch.ad_value(1357)), scratch.values[479]), scratch.values[376]));
        }

        if ((((scratch.values[1323] != 0.0) && (scratch.values[1340] != 0.0)) && (!(scratch.values[1533] != 0.0))) && (!(scratch.values[1544] != 0.0))) {
            scratch.store_ad(1384, &AdValue::scale(AdValue::div(AdValue::scale(AdValue::sub_from_scalar(scratch.values[373], scratch.ad_value(1357)), scratch.values[476]), scratch.ad_value(1359)), scratch.values[461]));
        }

        scratch.values[1546] = if (((((-scratch.values[491]) / scratch.values[1384])) as f64).abs() < 230.25850929940458) { 1.0 } else { 0.0 };

        if (((((scratch.values[1323] != 0.0) && (scratch.values[1340] != 0.0)) && (!(scratch.values[1533] != 0.0))) && (!(scratch.values[1544] != 0.0))) && (scratch.values[1546] != 0.0)) {
            scratch.store_ad(1359, &AdValue::exp(AdValue::div(AdValue::neg(scratch.ad_value(491)), scratch.ad_value(1384))));
        }

        scratch.values[1547] = if (((-scratch.values[491]) / scratch.values[1384]) < 0.0) { 1.0 } else { 0.0 };

        if ((((((scratch.values[1323] != 0.0) && (scratch.values[1340] != 0.0)) && (!(scratch.values[1533] != 0.0))) && (!(scratch.values[1544] != 0.0))) && (!(scratch.values[1546] != 0.0))) && (scratch.values[1547] != 0.0)) {
            let assign20540_ad_e21208: AdValue = AdValue::offset(AdValue::mul(AdValue::sub_from_scalar((-230.25850929940458), AdValue::div(AdValue::neg(scratch.ad_value(491)), scratch.ad_value(1384))), AdValue::offset(AdValue::scale(AdValue::mul(AdValue::sub_from_scalar((-230.25850929940458), AdValue::div(AdValue::neg(scratch.ad_value(491)), scratch.ad_value(1384))), AdValue::offset(AdValue::scale(AdValue::sub_from_scalar((-230.25850929940458), AdValue::div(AdValue::neg(scratch.ad_value(491)), scratch.ad_value(1384))), 0.3333333333333333), 1.0)), 0.5), 1.0)), 1.0);
            scratch.store_ad(1359, &AdValue::div_from_scalar(1e-100, assign20540_ad_e21208));
        }

        if ((((((scratch.values[1323] != 0.0) && (scratch.values[1340] != 0.0)) && (!(scratch.values[1533] != 0.0))) && (!(scratch.values[1544] != 0.0))) && (!(scratch.values[1546] != 0.0))) && (!(scratch.values[1547] != 0.0))) {
            let assign20550_ad_e21258: AdValue = AdValue::scale(AdValue::offset(AdValue::mul(AdValue::offset(AdValue::div(AdValue::neg(scratch.ad_value(491)), scratch.ad_value(1384)), (-230.25850929940458)), AdValue::offset(AdValue::scale(AdValue::mul(AdValue::offset(AdValue::div(AdValue::neg(scratch.ad_value(491)), scratch.ad_value(1384)), (-230.25850929940458)), AdValue::offset(AdValue::scale(AdValue::offset(AdValue::div(AdValue::neg(scratch.ad_value(491)), scratch.ad_value(1384)), (-230.25850929940458)), 0.3333333333333333), 1.0)), 0.5), 1.0)), 1.0), 1e100);
            scratch.store_ad(1359, &assign20550_ad_e21258);
        }

        if ((((scratch.values[1323] != 0.0) && (scratch.values[1340] != 0.0)) && (!(scratch.values[1533] != 0.0))) && (!(scratch.values[1544] != 0.0))) {
            scratch.store_ad(1383, &AdValue::scale(AdValue::mul(AdValue::mul(AdValue::mul(scratch.ad_value(518), scratch.ad_value(1384)), scratch.ad_value(1384)), scratch.ad_value(1359)), scratch.values[396]));
        }

        scratch.values[1548] = if (scratch.values[405] > 1000.0) { 1.0 } else { 0.0 };

        if ((((scratch.values[1323] != 0.0) && (scratch.values[1340] != 0.0)) && (!(scratch.values[1533] != 0.0))) && (scratch.values[1548] != 0.0)) {
            scratch.values[1385] = 1.0;
            scratch.node_derivatives[1385] = [0.0; Instance::NODE_COUNT];
            scratch.branch_derivatives[1385] = [0.0; Instance::BRANCH_COUNT];
        }

        scratch.values[1549] = if (scratch.values[1358] > ((-scratch.values[493]) * scratch.values[405])) { 1.0 } else { 0.0 };

        scratch.values[1550] = if (scratch.values[408] == 4.0) { 1.0 } else { 0.0 };

        if ((((((scratch.values[1323] != 0.0) && (scratch.values[1340] != 0.0)) && (!(scratch.values[1533] != 0.0))) && (!(scratch.values[1548] != 0.0))) && (scratch.values[1549] != 0.0)) && (scratch.values[1550] != 0.0)) {
            scratch.store_ad(1359, &AdValue::mul(AdValue::mul(AdValue::mul(AdValue::scale(scratch.ad_value(1358), scratch.values[498]), AdValue::scale(scratch.ad_value(1358), scratch.values[498])), AdValue::scale(scratch.ad_value(1358), scratch.values[498])), AdValue::scale(scratch.ad_value(1358), scratch.values[498])));
        }

        if ((((((scratch.values[1323] != 0.0) && (scratch.values[1340] != 0.0)) && (!(scratch.values[1533] != 0.0))) && (!(scratch.values[1548] != 0.0))) && (scratch.values[1549] != 0.0)) && (!(scratch.values[1550] != 0.0))) {
            scratch.store_ad(1359, &AdValue::powf(AdValue::abs(AdValue::scale(scratch.ad_value(1358), scratch.values[498])), scratch.values[408]));
        }

        if (((((scratch.values[1323] != 0.0) && (scratch.values[1340] != 0.0)) && (!(scratch.values[1533] != 0.0))) && (!(scratch.values[1548] != 0.0))) && (scratch.values[1549] != 0.0)) {
            scratch.store_ad(1385, &AdValue::div_from_scalar(1.0, AdValue::sub_from_scalar(1.0, scratch.ad_value(1359))));
        }

        if (((((scratch.values[1323] != 0.0) && (scratch.values[1340] != 0.0)) && (!(scratch.values[1533] != 0.0))) && (!(scratch.values[1548] != 0.0))) && (!(scratch.values[1549] != 0.0))) {
            scratch.store_ad(1385, &AdValue::offset(AdValue::scale(AdValue::offset(scratch.ad_value(1358), (scratch.values[493] * scratch.values[405])), scratch.values[501]), scratch.values[495]));
        }

        if (((scratch.values[1323] != 0.0) && (scratch.values[1340] != 0.0)) && (!(scratch.values[1533] != 0.0))) {
            scratch.store_ad(1387, &AdValue::mul(AdValue::add(AdValue::add(AdValue::add(scratch.ad_value(1360), scratch.ad_value(1361)), scratch.ad_value(1368)), scratch.ad_value(1383)), scratch.ad_value(1385)));
        }

        scratch.values[1551] = if (scratch.values[692] == 0.0) { 1.0 } else { 0.0 };

        if (((scratch.values[1323] != 0.0) && (scratch.values[1340] != 0.0)) && (scratch.values[1551] != 0.0)) {
            scratch.values[1388] = 0.0;
            scratch.node_derivatives[1388] = [0.0; Instance::NODE_COUNT];
            scratch.branch_derivatives[1388] = [0.0; Instance::BRANCH_COUNT];
        }

        scratch.values[1552] = if (scratch.values[459] == 0.5) { 1.0 } else { 0.0 };

        if ((((scratch.values[1323] != 0.0) && (scratch.values[1340] != 0.0)) && (!(scratch.values[1551] != 0.0))) && (scratch.values[1552] != 0.0)) {
            scratch.store_ad(1359, &AdValue::sqrt(AdValue::sub_from_scalar(1.0, AdValue::scale(scratch.ad_value(1351), scratch.values[456]))));
        }

        if ((((scratch.values[1323] != 0.0) && (scratch.values[1340] != 0.0)) && (!(scratch.values[1551] != 0.0))) && (!(scratch.values[1552] != 0.0))) {
            scratch.store_ad(1359, &AdValue::powf(AdValue::sub_from_scalar(1.0, AdValue::scale(scratch.ad_value(1351), scratch.values[456])), scratch.values[459]));
        }

        if (((scratch.values[1323] != 0.0) && (scratch.values[1340] != 0.0)) && (!(scratch.values[1551] != 0.0))) {
            scratch.store_ad(1360, &AdValue::scale(scratch.ad_value(1350), scratch.values[438]));
        }

        scratch.values[1553] = if ((scratch.values[388] == 0.0) && (scratch.values[391] == 0.0)) { 1.0 } else { 0.0 };

        if ((((scratch.values[1323] != 0.0) && (scratch.values[1340] != 0.0)) && (!(scratch.values[1551] != 0.0))) && (scratch.values[1553] != 0.0)) {
            scratch.values[1361] = 0.0;
            scratch.node_derivatives[1361] = [0.0; Instance::NODE_COUNT];
            scratch.branch_derivatives[1361] = [0.0; Instance::BRANCH_COUNT];
        }

        if ((((scratch.values[1323] != 0.0) && (scratch.values[1340] != 0.0)) && (!(scratch.values[1551] != 0.0))) && (!(scratch.values[1553] != 0.0))) {
            scratch.store_ad(1362, &AdValue::sub_from_scalar(scratch.values[444], scratch.ad_value(1356)));
        }

        if ((((scratch.values[1323] != 0.0) && (scratch.values[1340] != 0.0)) && (!(scratch.values[1551] != 0.0))) && (!(scratch.values[1553] != 0.0))) {
            scratch.store_ad(1363, &AdValue::sub_from_scalar(1.0, AdValue::sqrt(AdValue::sub_from_scalar(1.0, AdValue::div(scratch.ad_value(1354), scratch.ad_value(1362))))));
        }

        scratch.values[1554] = if (scratch.values[377] == 0.5) { 1.0 } else { 0.0 };

        if (((((scratch.values[1323] != 0.0) && (scratch.values[1340] != 0.0)) && (!(scratch.values[1551] != 0.0))) && (!(scratch.values[1553] != 0.0))) && (scratch.values[1554] != 0.0)) {
            scratch.values[1364] = 0.0;
            scratch.node_derivatives[1364] = [0.0; Instance::NODE_COUNT];
            scratch.branch_derivatives[1364] = [0.0; Instance::BRANCH_COUNT];
        }

        if (((((scratch.values[1323] != 0.0) && (scratch.values[1340] != 0.0)) && (!(scratch.values[1551] != 0.0))) && (!(scratch.values[1553] != 0.0))) && (!(scratch.values[1554] != 0.0))) {
            scratch.store_ad(1364, &AdValue::scale(AdValue::add(AdValue::div(AdValue::mul(AdValue::square(scratch.ad_value(1363)), AdValue::ln(scratch.ad_value(1363))), AdValue::sub_from_scalar(1.0, scratch.ad_value(1363))), scratch.ad_value(1363)), (1.0 - (2.0 * scratch.values[377]))));
        }

        if ((((scratch.values[1323] != 0.0) && (scratch.values[1340] != 0.0)) && (!(scratch.values[1551] != 0.0))) && (!(scratch.values[1553] != 0.0))) {
            scratch.store_ad(1365, &AdValue::add(scratch.ad_value(1363), scratch.ad_value(1364)));
        }

        scratch.values[1555] = if (scratch.values[377] == 0.5) { 1.0 } else { 0.0 };

        if (((((scratch.values[1323] != 0.0) && (scratch.values[1340] != 0.0)) && (!(scratch.values[1551] != 0.0))) && (!(scratch.values[1553] != 0.0))) && (scratch.values[1555] != 0.0)) {
            scratch.store_ad(1359, &AdValue::sqrt(AdValue::scale(scratch.ad_value(1362), scratch.values[480])));
        }

        if (((((scratch.values[1323] != 0.0) && (scratch.values[1340] != 0.0)) && (!(scratch.values[1551] != 0.0))) && (!(scratch.values[1553] != 0.0))) && (!(scratch.values[1555] != 0.0))) {
            scratch.store_ad(1359, &AdValue::powf(AdValue::scale(scratch.ad_value(1362), scratch.values[480]), scratch.values[377]));
        }

        if ((((scratch.values[1323] != 0.0) && (scratch.values[1340] != 0.0)) && (!(scratch.values[1551] != 0.0))) && (!(scratch.values[1553] != 0.0))) {
            scratch.store_ad(1366, &AdValue::scale(scratch.ad_value(1359), scratch.values[474]));
        }

        if ((((scratch.values[1323] != 0.0) && (scratch.values[1340] != 0.0)) && (!(scratch.values[1551] != 0.0))) && (!(scratch.values[1553] != 0.0))) {
            scratch.store_ad(1367, &AdValue::scale(AdValue::mul(AdValue::offset(scratch.ad_value(1353), (-1.0)), scratch.ad_value(1366)), scratch.values[435]));
        }

        if ((((scratch.values[1323] != 0.0) && (scratch.values[1340] != 0.0)) && (!(scratch.values[1551] != 0.0))) && (!(scratch.values[1553] != 0.0))) {
            scratch.store_ad(1361, &AdValue::scale(AdValue::mul(scratch.ad_value(1367), scratch.ad_value(1365)), scratch.values[388]));
        }

        scratch.values[1556] = if (scratch.values[391] == 0.0) { 1.0 } else { 0.0 };

        if ((((scratch.values[1323] != 0.0) && (scratch.values[1340] != 0.0)) && (!(scratch.values[1551] != 0.0))) && (scratch.values[1556] != 0.0)) {
            scratch.values[1368] = 0.0;
            scratch.node_derivatives[1368] = [0.0; Instance::NODE_COUNT];
            scratch.branch_derivatives[1368] = [0.0; Instance::BRANCH_COUNT];
        }

        if ((((scratch.values[1323] != 0.0) && (scratch.values[1340] != 0.0)) && (!(scratch.values[1551] != 0.0))) && (!(scratch.values[1556] != 0.0))) {
            scratch.store_ad(1369, &AdValue::scale(AdValue::div(AdValue::scale(scratch.ad_value(1366), scratch.values[459]), scratch.ad_value(1362)), scratch.values[489]));
        }

        if ((((scratch.values[1323] != 0.0) && (scratch.values[1340] != 0.0)) && (!(scratch.values[1551] != 0.0))) && (!(scratch.values[1556] != 0.0))) {
            scratch.store_ad(1370, &AdValue::div_from_scalar((0.666666666666667 * scratch.values[486]), scratch.ad_value(1369)));
        }

        if ((((scratch.values[1323] != 0.0) && (scratch.values[1340] != 0.0)) && (!(scratch.values[1551] != 0.0))) && (!(scratch.values[1556] != 0.0))) {
            scratch.store_ad(1371, &AdValue::square(scratch.ad_value(1370)));
        }

        if ((((scratch.values[1323] != 0.0) && (scratch.values[1340] != 0.0)) && (!(scratch.values[1551] != 0.0))) && (!(scratch.values[1556] != 0.0))) {
            scratch.store_ad(1372, &AdValue::sqrt(AdValue::div(AdValue::square(scratch.ad_value(1371)), AdValue::offset(AdValue::square(scratch.ad_value(1371)), 1.0))));
        }

        if ((((scratch.values[1323] != 0.0) && (scratch.values[1340] != 0.0)) && (!(scratch.values[1551] != 0.0))) && (!(scratch.values[1556] != 0.0))) {
            scratch.store_ad(1373, &AdValue::sqrt(AdValue::abs(scratch.ad_value(1372))));
        }

        if ((((scratch.values[1323] != 0.0) && (scratch.values[1340] != 0.0)) && (!(scratch.values[1551] != 0.0))) && (!(scratch.values[1556] != 0.0))) {
            scratch.store_ad(1374, &AdValue::mul(scratch.ad_value(1372), scratch.ad_value(1373)));
        }

        scratch.values[1557] = if (((-scratch.values[377]) * scratch.values[462]) == (-1.0)) { 1.0 } else { 0.0 };

        if (((((scratch.values[1323] != 0.0) && (scratch.values[1340] != 0.0)) && (!(scratch.values[1551] != 0.0))) && (!(scratch.values[1556] != 0.0))) && (scratch.values[1557] != 0.0)) {
            scratch.store_ad(1375, &AdValue::div_from_scalar(1.0, AdValue::offset(AdValue::mul(scratch.ad_value(1369), scratch.ad_value(1374)), 1.0)));
        }

        if (((((scratch.values[1323] != 0.0) && (scratch.values[1340] != 0.0)) && (!(scratch.values[1551] != 0.0))) && (!(scratch.values[1556] != 0.0))) && (!(scratch.values[1557] != 0.0))) {
            scratch.store_ad(1375, &AdValue::powf(AdValue::offset(AdValue::mul(scratch.ad_value(1369), scratch.ad_value(1374)), 1.0), ((-scratch.values[377]) * scratch.values[462])));
        }

        if ((((scratch.values[1323] != 0.0) && (scratch.values[1340] != 0.0)) && (!(scratch.values[1551] != 0.0))) && (!(scratch.values[1556] != 0.0))) {
            scratch.store_ad(1376, &AdValue::div(AdValue::mul(scratch.ad_value(1365), scratch.ad_value(1375)), AdValue::add(scratch.ad_value(1365), scratch.ad_value(1375))));
        }

        if ((((scratch.values[1323] != 0.0) && (scratch.values[1340] != 0.0)) && (!(scratch.values[1551] != 0.0))) && (!(scratch.values[1556] != 0.0))) {
            scratch.store_ad(1377, &AdValue::sqrt(AdValue::scale(AdValue::div(scratch.ad_value(1369), scratch.ad_value(1373)), 0.375)));
        }

        if ((((scratch.values[1323] != 0.0) && (scratch.values[1340] != 0.0)) && (!(scratch.values[1551] != 0.0))) && (!(scratch.values[1556] != 0.0))) {
            scratch.store_ad(1378, &AdValue::sub(AdValue::scale(AdValue::mul(scratch.ad_value(1370), scratch.ad_value(1373)), 2.0), scratch.ad_value(1372)));
        }

        if ((((scratch.values[1323] != 0.0) && (scratch.values[1340] != 0.0)) && (!(scratch.values[1551] != 0.0))) && (!(scratch.values[1556] != 0.0))) {
            scratch.store_ad(1379, &AdValue::add(AdValue::sub(AdValue::mul(AdValue::scale(scratch.ad_value(1370), scratch.values[486]), scratch.ad_value(1373)), AdValue::scale(scratch.ad_value(1372), scratch.values[486])), AdValue::scale(AdValue::mul(scratch.ad_value(1369), scratch.ad_value(1374)), 0.5)));
        }

        if ((((scratch.values[1323] != 0.0) && (scratch.values[1340] != 0.0)) && (!(scratch.values[1551] != 0.0))) && (!(scratch.values[1556] != 0.0))) {
            scratch.store_ad(1380, &AdValue::mul(AdValue::offset(scratch.ad_value(1378), (-1.0)), scratch.ad_value(1377)));
        }

        if ((((scratch.values[1323] != 0.0) && (scratch.values[1340] != 0.0)) && (!(scratch.values[1551] != 0.0))) && (!(scratch.values[1556] != 0.0))) {
            scratch.store_ad(1341, &AdValue::square(scratch.ad_value(1380)));
        }

        scratch.values[1558] = if (scratch.values[1380] > 0.0) { 1.0 } else { 0.0 };

        if (((((scratch.values[1323] != 0.0) && (scratch.values[1340] != 0.0)) && (!(scratch.values[1551] != 0.0))) && (!(scratch.values[1556] != 0.0))) && (scratch.values[1558] != 0.0)) {
            scratch.store_ad(1342, &AdValue::div_from_scalar(1.0, AdValue::offset(AdValue::scale(scratch.ad_value(1380), scratch.values[421]), 1.0)));
        }

        if (((((scratch.values[1323] != 0.0) && (scratch.values[1340] != 0.0)) && (!(scratch.values[1551] != 0.0))) && (!(scratch.values[1556] != 0.0))) && (!(scratch.values[1558] != 0.0))) {
            scratch.store_ad(1342, &AdValue::div_from_scalar(1.0, AdValue::sub_from_scalar(1.0, AdValue::scale(scratch.ad_value(1380), scratch.values[421]))));
        }

        scratch.values[1559] = if (((-scratch.values[1341]) + scratch.values[1379]) > (-230.25850929940458)) { 1.0 } else { 0.0 };

        if (((((scratch.values[1323] != 0.0) && (scratch.values[1340] != 0.0)) && (!(scratch.values[1551] != 0.0))) && (!(scratch.values[1556] != 0.0))) && (scratch.values[1559] != 0.0)) {
            scratch.store_ad(1359, &AdValue::exp(AdValue::sub(scratch.ad_value(1379), scratch.ad_value(1341))));
        }

        if (((((scratch.values[1323] != 0.0) && (scratch.values[1340] != 0.0)) && (!(scratch.values[1551] != 0.0))) && (!(scratch.values[1556] != 0.0))) && (!(scratch.values[1559] != 0.0))) {
            let assign21100_ad_e22087: AdValue = AdValue::div_from_scalar(1e-100, AdValue::offset(AdValue::mul(AdValue::sub_from_scalar((-230.25850929940458), AdValue::sub(scratch.ad_value(1379), scratch.ad_value(1341))), AdValue::offset(AdValue::scale(AdValue::mul(AdValue::sub_from_scalar((-230.25850929940458), AdValue::sub(scratch.ad_value(1379), scratch.ad_value(1341))), AdValue::offset(AdValue::scale(AdValue::sub_from_scalar((-230.25850929940458), AdValue::sub(scratch.ad_value(1379), scratch.ad_value(1341))), 0.3333333333333333), 1.0)), 0.5), 1.0)), 1.0));
            scratch.store_ad(1359, &assign21100_ad_e22087);
        }

        if ((((scratch.values[1323] != 0.0) && (scratch.values[1340] != 0.0)) && (!(scratch.values[1551] != 0.0))) && (!(scratch.values[1556] != 0.0))) {
            scratch.store_ad(1343, &AdValue::mul(AdValue::add(AdValue::add(AdValue::scale(scratch.ad_value(1342), 0.29214664), AdValue::scale(AdValue::square(scratch.ad_value(1342)), scratch.values[422])), AdValue::scale(AdValue::mul(AdValue::square(scratch.ad_value(1342)), scratch.ad_value(1342)), scratch.values[423])), scratch.ad_value(1359)));
        }

        scratch.values[1560] = if (scratch.values[1380] > 0.0) { 1.0 } else { 0.0 };

        if (((((scratch.values[1323] != 0.0) && (scratch.values[1340] != 0.0)) && (!(scratch.values[1551] != 0.0))) && (!(scratch.values[1556] != 0.0))) && (scratch.values[1560] != 0.0)) {
            scratch.values[1381] = scratch.values[1343];
            scratch.node_derivatives[1381] = scratch.node_derivatives[1343];
            scratch.branch_derivatives[1381] = scratch.branch_derivatives[1343];
        }

        scratch.values[1561] = if (scratch.values[1379] > (-230.25850929940458)) { 1.0 } else { 0.0 };

        if ((((((scratch.values[1323] != 0.0) && (scratch.values[1340] != 0.0)) && (!(scratch.values[1551] != 0.0))) && (!(scratch.values[1556] != 0.0))) && (!(scratch.values[1560] != 0.0))) && (scratch.values[1561] != 0.0)) {
            scratch.store_ad(1359, &AdValue::exp(scratch.ad_value(1379)));
        }

        if ((((((scratch.values[1323] != 0.0) && (scratch.values[1340] != 0.0)) && (!(scratch.values[1551] != 0.0))) && (!(scratch.values[1556] != 0.0))) && (!(scratch.values[1560] != 0.0))) && (!(scratch.values[1561] != 0.0))) {
            scratch.store_ad(1359, &AdValue::div_from_scalar(1e-100, AdValue::offset(AdValue::mul(AdValue::sub_from_scalar((-230.25850929940458), scratch.ad_value(1379)), AdValue::offset(AdValue::scale(AdValue::mul(AdValue::sub_from_scalar((-230.25850929940458), scratch.ad_value(1379)), AdValue::offset(AdValue::scale(AdValue::sub_from_scalar((-230.25850929940458), scratch.ad_value(1379)), 0.3333333333333333), 1.0)), 0.5), 1.0)), 1.0)));
        }

        if (((((scratch.values[1323] != 0.0) && (scratch.values[1340] != 0.0)) && (!(scratch.values[1551] != 0.0))) && (!(scratch.values[1556] != 0.0))) && (!(scratch.values[1560] != 0.0))) {
            scratch.store_ad(1381, &AdValue::sub(AdValue::scale(scratch.ad_value(1359), 2.0), scratch.ad_value(1343)));
        }

        if ((((scratch.values[1323] != 0.0) && (scratch.values[1340] != 0.0)) && (!(scratch.values[1551] != 0.0))) && (!(scratch.values[1556] != 0.0))) {
            scratch.store_ad(1382, &AdValue::scale(AdValue::div(AdValue::scale(scratch.ad_value(1381), scratch.values[486]), scratch.ad_value(1377)), (1.772453850905516 * 0.5)));
        }

        if ((((scratch.values[1323] != 0.0) && (scratch.values[1340] != 0.0)) && (!(scratch.values[1551] != 0.0))) && (!(scratch.values[1556] != 0.0))) {
            scratch.store_ad(1368, &AdValue::scale(AdValue::mul(AdValue::mul(scratch.ad_value(1367), scratch.ad_value(1382)), scratch.ad_value(1376)), scratch.values[391]));
        }

        scratch.values[1562] = if (scratch.values[397] == 0.0) { 1.0 } else { 0.0 };

        if ((((scratch.values[1323] != 0.0) && (scratch.values[1340] != 0.0)) && (!(scratch.values[1551] != 0.0))) && (scratch.values[1562] != 0.0)) {
            scratch.values[1383] = 0.0;
            scratch.node_derivatives[1383] = [0.0; Instance::NODE_COUNT];
            scratch.branch_derivatives[1383] = [0.0; Instance::BRANCH_COUNT];
        }

        scratch.values[1563] = if (scratch.values[377] == 0.5) { 1.0 } else { 0.0 };

        if (((((scratch.values[1323] != 0.0) && (scratch.values[1340] != 0.0)) && (!(scratch.values[1551] != 0.0))) && (!(scratch.values[1562] != 0.0))) && (scratch.values[1563] != 0.0)) {
            scratch.store_ad(1359, &AdValue::sqrt(AdValue::scale(AdValue::sub_from_scalar(scratch.values[374], scratch.ad_value(1357)), scratch.values[480])));
        }

        if (((((scratch.values[1323] != 0.0) && (scratch.values[1340] != 0.0)) && (!(scratch.values[1551] != 0.0))) && (!(scratch.values[1562] != 0.0))) && (!(scratch.values[1563] != 0.0))) {
            scratch.store_ad(1359, &AdValue::powf(AdValue::scale(AdValue::sub_from_scalar(scratch.values[374], scratch.ad_value(1357)), scratch.values[480]), scratch.values[377]));
        }

        if ((((scratch.values[1323] != 0.0) && (scratch.values[1340] != 0.0)) && (!(scratch.values[1551] != 0.0))) && (!(scratch.values[1562] != 0.0))) {
            scratch.store_ad(1384, &AdValue::scale(AdValue::div(AdValue::scale(AdValue::sub_from_scalar(scratch.values[374], scratch.ad_value(1357)), scratch.values[477]), scratch.ad_value(1359)), scratch.values[462]));
        }

        scratch.values[1564] = if (((((-scratch.values[492]) / scratch.values[1384])) as f64).abs() < 230.25850929940458) { 1.0 } else { 0.0 };

        if (((((scratch.values[1323] != 0.0) && (scratch.values[1340] != 0.0)) && (!(scratch.values[1551] != 0.0))) && (!(scratch.values[1562] != 0.0))) && (scratch.values[1564] != 0.0)) {
            scratch.store_ad(1359, &AdValue::exp(AdValue::div(AdValue::neg(scratch.ad_value(492)), scratch.ad_value(1384))));
        }

        scratch.values[1565] = if (((-scratch.values[492]) / scratch.values[1384]) < 0.0) { 1.0 } else { 0.0 };

        if ((((((scratch.values[1323] != 0.0) && (scratch.values[1340] != 0.0)) && (!(scratch.values[1551] != 0.0))) && (!(scratch.values[1562] != 0.0))) && (!(scratch.values[1564] != 0.0))) && (scratch.values[1565] != 0.0)) {
            let assign21290_ad_e22414: AdValue = AdValue::offset(AdValue::mul(AdValue::sub_from_scalar((-230.25850929940458), AdValue::div(AdValue::neg(scratch.ad_value(492)), scratch.ad_value(1384))), AdValue::offset(AdValue::scale(AdValue::mul(AdValue::sub_from_scalar((-230.25850929940458), AdValue::div(AdValue::neg(scratch.ad_value(492)), scratch.ad_value(1384))), AdValue::offset(AdValue::scale(AdValue::sub_from_scalar((-230.25850929940458), AdValue::div(AdValue::neg(scratch.ad_value(492)), scratch.ad_value(1384))), 0.3333333333333333), 1.0)), 0.5), 1.0)), 1.0);
            scratch.store_ad(1359, &AdValue::div_from_scalar(1e-100, assign21290_ad_e22414));
        }

        if ((((((scratch.values[1323] != 0.0) && (scratch.values[1340] != 0.0)) && (!(scratch.values[1551] != 0.0))) && (!(scratch.values[1562] != 0.0))) && (!(scratch.values[1564] != 0.0))) && (!(scratch.values[1565] != 0.0))) {
            let assign21300_ad_e22464: AdValue = AdValue::scale(AdValue::offset(AdValue::mul(AdValue::offset(AdValue::div(AdValue::neg(scratch.ad_value(492)), scratch.ad_value(1384)), (-230.25850929940458)), AdValue::offset(AdValue::scale(AdValue::mul(AdValue::offset(AdValue::div(AdValue::neg(scratch.ad_value(492)), scratch.ad_value(1384)), (-230.25850929940458)), AdValue::offset(AdValue::scale(AdValue::offset(AdValue::div(AdValue::neg(scratch.ad_value(492)), scratch.ad_value(1384)), (-230.25850929940458)), 0.3333333333333333), 1.0)), 0.5), 1.0)), 1.0), 1e100);
            scratch.store_ad(1359, &assign21300_ad_e22464);
        }

        if ((((scratch.values[1323] != 0.0) && (scratch.values[1340] != 0.0)) && (!(scratch.values[1551] != 0.0))) && (!(scratch.values[1562] != 0.0))) {
            scratch.store_ad(1383, &AdValue::scale(AdValue::mul(AdValue::mul(AdValue::mul(scratch.ad_value(518), scratch.ad_value(1384)), scratch.ad_value(1384)), scratch.ad_value(1359)), scratch.values[397]));
        }

        scratch.values[1566] = if (scratch.values[406] > 1000.0) { 1.0 } else { 0.0 };

        if ((((scratch.values[1323] != 0.0) && (scratch.values[1340] != 0.0)) && (!(scratch.values[1551] != 0.0))) && (scratch.values[1566] != 0.0)) {
            scratch.values[1385] = 1.0;
            scratch.node_derivatives[1385] = [0.0; Instance::NODE_COUNT];
            scratch.branch_derivatives[1385] = [0.0; Instance::BRANCH_COUNT];
        }

        scratch.values[1567] = if (scratch.values[1358] > ((-scratch.values[493]) * scratch.values[406])) { 1.0 } else { 0.0 };

        scratch.values[1568] = if (scratch.values[409] == 4.0) { 1.0 } else { 0.0 };

        if ((((((scratch.values[1323] != 0.0) && (scratch.values[1340] != 0.0)) && (!(scratch.values[1551] != 0.0))) && (!(scratch.values[1566] != 0.0))) && (scratch.values[1567] != 0.0)) && (scratch.values[1568] != 0.0)) {
            scratch.store_ad(1359, &AdValue::mul(AdValue::mul(AdValue::mul(AdValue::scale(scratch.ad_value(1358), scratch.values[499]), AdValue::scale(scratch.ad_value(1358), scratch.values[499])), AdValue::scale(scratch.ad_value(1358), scratch.values[499])), AdValue::scale(scratch.ad_value(1358), scratch.values[499])));
        }

        if ((((((scratch.values[1323] != 0.0) && (scratch.values[1340] != 0.0)) && (!(scratch.values[1551] != 0.0))) && (!(scratch.values[1566] != 0.0))) && (scratch.values[1567] != 0.0)) && (!(scratch.values[1568] != 0.0))) {
            scratch.store_ad(1359, &AdValue::powf(AdValue::abs(AdValue::scale(scratch.ad_value(1358), scratch.values[499])), scratch.values[409]));
        }

        if (((((scratch.values[1323] != 0.0) && (scratch.values[1340] != 0.0)) && (!(scratch.values[1551] != 0.0))) && (!(scratch.values[1566] != 0.0))) && (scratch.values[1567] != 0.0)) {
            scratch.store_ad(1385, &AdValue::div_from_scalar(1.0, AdValue::sub_from_scalar(1.0, scratch.ad_value(1359))));
        }

        if (((((scratch.values[1323] != 0.0) && (scratch.values[1340] != 0.0)) && (!(scratch.values[1551] != 0.0))) && (!(scratch.values[1566] != 0.0))) && (!(scratch.values[1567] != 0.0))) {
            scratch.store_ad(1385, &AdValue::offset(AdValue::scale(AdValue::offset(scratch.ad_value(1358), (scratch.values[493] * scratch.values[406])), scratch.values[502]), scratch.values[496]));
        }

        if (((scratch.values[1323] != 0.0) && (scratch.values[1340] != 0.0)) && (!(scratch.values[1551] != 0.0))) {
            scratch.store_ad(1388, &AdValue::mul(AdValue::add(AdValue::add(AdValue::add(scratch.ad_value(1360), scratch.ad_value(1361)), scratch.ad_value(1368)), scratch.ad_value(1383)), scratch.ad_value(1385)));
        }

        if ((scratch.values[1323] != 0.0) && (scratch.values[1340] != 0.0)) {
            scratch.store_ad(508, &AdValue::add(AdValue::add(AdValue::mul(scratch.ad_value(690), scratch.ad_value(1386)), AdValue::mul(scratch.ad_value(691), scratch.ad_value(1387))), AdValue::mul(scratch.ad_value(692), scratch.ad_value(1388))));
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

        scratch.values[1569] = if !(((scratch.values[690] == 0.0) && (scratch.values[691] == 0.0)) && (scratch.values[692] == 0.0)) { 1.0 } else { 0.0 };

        if (((scratch.values[1323] != 0.0) && (scratch.values[1340] != 0.0)) && (scratch.values[1569] != 0.0)) {
            scratch.store_ad(1344, &AdValue::mul(AdValue::scale(scratch.ad_value(701), 4.0), scratch.ad_value(701)));
        }

        if (((scratch.values[1323] != 0.0) && (scratch.values[1340] != 0.0)) && (scratch.values[1569] != 0.0)) {
            scratch.store_ad(1345, &AdValue::div(scratch.ad_value(701), scratch.ad_value(702)));
        }

        if (((scratch.values[1323] != 0.0) && (scratch.values[1340] != 0.0)) && (scratch.values[1569] != 0.0)) {
            scratch.store_ad(1346, &AdValue::add(scratch.ad_value(519), AdValue::mul(scratch.ad_value(701), scratch.ad_value(1345))));
        }

        if (((scratch.values[1323] != 0.0) && (scratch.values[1340] != 0.0)) && (scratch.values[1569] != 0.0)) {
            scratch.store_ad(1347, &AdValue::add(scratch.ad_value(702), scratch.ad_value(1346)));
        }

        if (((scratch.values[1323] != 0.0) && (scratch.values[1340] != 0.0)) && (scratch.values[1569] != 0.0)) {
            scratch.store_ad(1348, &AdValue::sub(scratch.ad_value(702), scratch.ad_value(1346)));
        }

        if (((scratch.values[1323] != 0.0) && (scratch.values[1340] != 0.0)) && (scratch.values[1569] != 0.0)) {
            scratch.store_ad(1349, &AdValue::sqrt(AdValue::add(AdValue::square(scratch.ad_value(1348)), scratch.ad_value(1344))));
        }

        if (((scratch.values[1323] != 0.0) && (scratch.values[1340] != 0.0)) && (scratch.values[1569] != 0.0)) {
            scratch.store_ad(1351, &AdValue::scale(AdValue::div(AdValue::mul(scratch.ad_value(519), scratch.ad_value(702)), AdValue::add(scratch.ad_value(1347), scratch.ad_value(1349))), 2.0));
        }

        scratch.values[1570] = if (scratch.values[519] < scratch.values[698]) { 1.0 } else { 0.0 };

        scratch.values[1571] = if ((((0.5 * (scratch.values[519] * scratch.values[420]))) as f64).abs() < 230.25850929940458) { 1.0 } else { 0.0 };

        if (((((scratch.values[1323] != 0.0) && (scratch.values[1340] != 0.0)) && (scratch.values[1569] != 0.0)) && (scratch.values[1570] != 0.0)) && (scratch.values[1571] != 0.0)) {
            scratch.store_ad(1353, &AdValue::exp(AdValue::scale(scratch.ad_value(519), (scratch.values[420] * 0.5))));
        }

        scratch.values[1572] = if ((0.5 * (scratch.values[519] * scratch.values[420])) < 0.0) { 1.0 } else { 0.0 };

        if ((((((scratch.values[1323] != 0.0) && (scratch.values[1340] != 0.0)) && (scratch.values[1569] != 0.0)) && (scratch.values[1570] != 0.0)) && (!(scratch.values[1571] != 0.0))) && (scratch.values[1572] != 0.0)) {
            let assign21560_ad_e22827: AdValue = AdValue::div_from_scalar(1e-100, AdValue::offset(AdValue::mul(AdValue::sub_from_scalar((-230.25850929940458), AdValue::scale(scratch.ad_value(519), (scratch.values[420] * 0.5))), AdValue::offset(AdValue::scale(AdValue::mul(AdValue::sub_from_scalar((-230.25850929940458), AdValue::scale(scratch.ad_value(519), (scratch.values[420] * 0.5))), AdValue::offset(AdValue::scale(AdValue::sub_from_scalar((-230.25850929940458), AdValue::scale(scratch.ad_value(519), (scratch.values[420] * 0.5))), 0.3333333333333333), 1.0)), 0.5), 1.0)), 1.0));
            scratch.store_ad(1353, &assign21560_ad_e22827);
        }

        if ((((((scratch.values[1323] != 0.0) && (scratch.values[1340] != 0.0)) && (scratch.values[1569] != 0.0)) && (scratch.values[1570] != 0.0)) && (!(scratch.values[1571] != 0.0))) && (!(scratch.values[1572] != 0.0))) {
            scratch.store_ad(1353, &AdValue::scale(AdValue::offset(AdValue::mul(AdValue::offset(AdValue::scale(scratch.ad_value(519), (scratch.values[420] * 0.5)), (-230.25850929940458)), AdValue::offset(AdValue::scale(AdValue::mul(AdValue::offset(AdValue::scale(scratch.ad_value(519), (scratch.values[420] * 0.5)), (-230.25850929940458)), AdValue::offset(AdValue::scale(AdValue::offset(AdValue::scale(scratch.ad_value(519), (scratch.values[420] * 0.5)), (-230.25850929940458)), 0.3333333333333333), 1.0)), 0.5), 1.0)), 1.0), 1e100));
        }

        if ((((scratch.values[1323] != 0.0) && (scratch.values[1340] != 0.0)) && (scratch.values[1569] != 0.0)) && (scratch.values[1570] != 0.0)) {
            scratch.store_ad(1350, &AdValue::square(scratch.ad_value(1353)));
        }

        if ((((scratch.values[1323] != 0.0) && (scratch.values[1340] != 0.0)) && (scratch.values[1569] != 0.0)) && (!(scratch.values[1570] != 0.0))) {
            scratch.store_ad(1350, &AdValue::mul(AdValue::offset(AdValue::scale(AdValue::sub(scratch.ad_value(519), scratch.ad_value(698)), scratch.values[420]), 1.0), scratch.ad_value(699)));
        }

        if ((((scratch.values[1323] != 0.0) && (scratch.values[1340] != 0.0)) && (scratch.values[1569] != 0.0)) && (!(scratch.values[1570] != 0.0))) {
            scratch.store_ad(1353, &AdValue::sqrt(scratch.ad_value(1350)));
        }

        if (((scratch.values[1323] != 0.0) && (scratch.values[1340] != 0.0)) && (scratch.values[1569] != 0.0)) {
            scratch.store_ad(1350, &AdValue::offset(scratch.ad_value(1350), (-1.0)));
        }

        if (((scratch.values[1323] != 0.0) && (scratch.values[1340] != 0.0)) && (scratch.values[1569] != 0.0)) {
            scratch.store_ad(1352, &AdValue::div_from_scalar(1.0, scratch.ad_value(1353)));
        }

        scratch.values[1573] = if (scratch.values[519] > 0.0) { 1.0 } else { 0.0 };

        if ((((scratch.values[1323] != 0.0) && (scratch.values[1340] != 0.0)) && (scratch.values[1569] != 0.0)) && (scratch.values[1573] != 0.0)) {
            scratch.store_ad(1354, &AdValue::scale(AdValue::ln(AdValue::add(AdValue::offset(scratch.ad_value(1352), 2.0), AdValue::sqrt(AdValue::mul(AdValue::offset(scratch.ad_value(1352), 1.0), AdValue::offset(scratch.ad_value(1352), 3.0))))), (scratch.values[419] * 2.0)));
        }

        if ((((scratch.values[1323] != 0.0) && (scratch.values[1340] != 0.0)) && (scratch.values[1569] != 0.0)) && (!(scratch.values[1573] != 0.0))) {
            scratch.store_ad(1354, &AdValue::sub(AdValue::scale(AdValue::ln(AdValue::add(AdValue::offset(AdValue::scale(scratch.ad_value(1353), 2.0), 1.0), AdValue::sqrt(AdValue::mul(AdValue::offset(scratch.ad_value(1353), 1.0), AdValue::offset(AdValue::scale(scratch.ad_value(1353), 3.0), 1.0))))), (scratch.values[419] * 2.0)), scratch.ad_value(519)));
        }

        if (((scratch.values[1323] != 0.0) && (scratch.values[1340] != 0.0)) && (scratch.values[1569] != 0.0)) {
            scratch.store_ad(1355, &AdValue::sub(scratch.ad_value(700), scratch.ad_value(1354)));
        }

        if (((scratch.values[1323] != 0.0) && (scratch.values[1340] != 0.0)) && (scratch.values[1569] != 0.0)) {
            scratch.store_ad(1356, &AdValue::scale(AdValue::sub(AdValue::add(scratch.ad_value(519), scratch.ad_value(1355)), AdValue::sqrt(AdValue::offset(AdValue::mul(AdValue::sub(scratch.ad_value(519), scratch.ad_value(1355)), AdValue::sub(scratch.ad_value(519), scratch.ad_value(1355))), ((4.0 * scratch.values[419]) * scratch.values[419])))), 0.5));
        }

        if (((scratch.values[1323] != 0.0) && (scratch.values[1340] != 0.0)) && (scratch.values[1569] != 0.0)) {
            scratch.store_ad(1357, &AdValue::scale(AdValue::sub(AdValue::add(scratch.ad_value(519), scratch.ad_value(703)), AdValue::sqrt(AdValue::offset(AdValue::mul(AdValue::sub(scratch.ad_value(519), scratch.ad_value(703)), AdValue::sub(scratch.ad_value(519), scratch.ad_value(703))), ((4.0 * scratch.values[417]) * scratch.values[417])))), 0.5));
        }

        if (((scratch.values[1323] != 0.0) && (scratch.values[1340] != 0.0)) && (scratch.values[1569] != 0.0)) {
            scratch.store_ad(1358, &AdValue::scale(AdValue::sub(scratch.ad_value(519), AdValue::sqrt(AdValue::offset(AdValue::mul(scratch.ad_value(519), scratch.ad_value(519)), ((4.0 * 1e-6) * 1e-6)))), 0.5));
        }

        scratch.values[1574] = if (scratch.values[690] == 0.0) { 1.0 } else { 0.0 };

        if (((scratch.values[1323] != 0.0) && (scratch.values[1340] != 0.0)) && (scratch.values[1574] != 0.0)) {
            scratch.values[1386] = 0.0;
            scratch.node_derivatives[1386] = [0.0; Instance::NODE_COUNT];
            scratch.branch_derivatives[1386] = [0.0; Instance::BRANCH_COUNT];
        }

        scratch.values[1575] = if (scratch.values[457] == 0.5) { 1.0 } else { 0.0 };

        if ((((scratch.values[1323] != 0.0) && (scratch.values[1340] != 0.0)) && (!(scratch.values[1574] != 0.0))) && (scratch.values[1575] != 0.0)) {
            scratch.store_ad(1359, &AdValue::sqrt(AdValue::sub_from_scalar(1.0, AdValue::scale(scratch.ad_value(1351), scratch.values[454]))));
        }

        if ((((scratch.values[1323] != 0.0) && (scratch.values[1340] != 0.0)) && (!(scratch.values[1574] != 0.0))) && (!(scratch.values[1575] != 0.0))) {
            scratch.store_ad(1359, &AdValue::powf(AdValue::sub_from_scalar(1.0, AdValue::scale(scratch.ad_value(1351), scratch.values[454])), scratch.values[457]));
        }

        if (((scratch.values[1323] != 0.0) && (scratch.values[1340] != 0.0)) && (!(scratch.values[1574] != 0.0))) {
            scratch.store_ad(1360, &AdValue::scale(scratch.ad_value(1350), scratch.values[436]));
        }

        scratch.values[1576] = if ((scratch.values[386] == 0.0) && (scratch.values[389] == 0.0)) { 1.0 } else { 0.0 };

        if ((((scratch.values[1323] != 0.0) && (scratch.values[1340] != 0.0)) && (!(scratch.values[1574] != 0.0))) && (scratch.values[1576] != 0.0)) {
            scratch.values[1361] = 0.0;
            scratch.node_derivatives[1361] = [0.0; Instance::NODE_COUNT];
            scratch.branch_derivatives[1361] = [0.0; Instance::BRANCH_COUNT];
        }

        if ((((scratch.values[1323] != 0.0) && (scratch.values[1340] != 0.0)) && (!(scratch.values[1574] != 0.0))) && (!(scratch.values[1576] != 0.0))) {
            scratch.store_ad(1362, &AdValue::sub_from_scalar(scratch.values[442], scratch.ad_value(1356)));
        }

        if ((((scratch.values[1323] != 0.0) && (scratch.values[1340] != 0.0)) && (!(scratch.values[1574] != 0.0))) && (!(scratch.values[1576] != 0.0))) {
            scratch.store_ad(1363, &AdValue::sub_from_scalar(1.0, AdValue::sqrt(AdValue::sub_from_scalar(1.0, AdValue::div(scratch.ad_value(1354), scratch.ad_value(1362))))));
        }

        scratch.values[1577] = if (scratch.values[375] == 0.5) { 1.0 } else { 0.0 };

        if (((((scratch.values[1323] != 0.0) && (scratch.values[1340] != 0.0)) && (!(scratch.values[1574] != 0.0))) && (!(scratch.values[1576] != 0.0))) && (scratch.values[1577] != 0.0)) {
            scratch.values[1364] = 0.0;
            scratch.node_derivatives[1364] = [0.0; Instance::NODE_COUNT];
            scratch.branch_derivatives[1364] = [0.0; Instance::BRANCH_COUNT];
        }

        if (((((scratch.values[1323] != 0.0) && (scratch.values[1340] != 0.0)) && (!(scratch.values[1574] != 0.0))) && (!(scratch.values[1576] != 0.0))) && (!(scratch.values[1577] != 0.0))) {
            scratch.store_ad(1364, &AdValue::scale(AdValue::add(AdValue::div(AdValue::mul(AdValue::square(scratch.ad_value(1363)), AdValue::ln(scratch.ad_value(1363))), AdValue::sub_from_scalar(1.0, scratch.ad_value(1363))), scratch.ad_value(1363)), (1.0 - (2.0 * scratch.values[375]))));
        }

    }

    pub(super) fn stamp_transient_block_17(
        &mut self,
        ctx: &GeneratedEvalContext<'_>,
        stamper: &mut GeneratedStamper<'_>,
        scratch: &mut Scratch,
    ) {
        let _ = stamper;
        if ((((scratch.values[1323] != 0.0) && (scratch.values[1340] != 0.0)) && (!(scratch.values[1574] != 0.0))) && (!(scratch.values[1576] != 0.0))) {
            scratch.store_ad(1365, &AdValue::add(scratch.ad_value(1363), scratch.ad_value(1364)));
        }

        scratch.values[1578] = if (scratch.values[375] == 0.5) { 1.0 } else { 0.0 };

        if (((((scratch.values[1323] != 0.0) && (scratch.values[1340] != 0.0)) && (!(scratch.values[1574] != 0.0))) && (!(scratch.values[1576] != 0.0))) && (scratch.values[1578] != 0.0)) {
            scratch.store_ad(1359, &AdValue::sqrt(AdValue::scale(scratch.ad_value(1362), scratch.values[478])));
        }

        if (((((scratch.values[1323] != 0.0) && (scratch.values[1340] != 0.0)) && (!(scratch.values[1574] != 0.0))) && (!(scratch.values[1576] != 0.0))) && (!(scratch.values[1578] != 0.0))) {
            scratch.store_ad(1359, &AdValue::powf(AdValue::scale(scratch.ad_value(1362), scratch.values[478]), scratch.values[375]));
        }

        if ((((scratch.values[1323] != 0.0) && (scratch.values[1340] != 0.0)) && (!(scratch.values[1574] != 0.0))) && (!(scratch.values[1576] != 0.0))) {
            scratch.store_ad(1366, &AdValue::scale(scratch.ad_value(1359), scratch.values[472]));
        }

        if ((((scratch.values[1323] != 0.0) && (scratch.values[1340] != 0.0)) && (!(scratch.values[1574] != 0.0))) && (!(scratch.values[1576] != 0.0))) {
            scratch.store_ad(1367, &AdValue::scale(AdValue::mul(AdValue::offset(scratch.ad_value(1353), (-1.0)), scratch.ad_value(1366)), scratch.values[433]));
        }

        if ((((scratch.values[1323] != 0.0) && (scratch.values[1340] != 0.0)) && (!(scratch.values[1574] != 0.0))) && (!(scratch.values[1576] != 0.0))) {
            scratch.store_ad(1361, &AdValue::scale(AdValue::mul(scratch.ad_value(1367), scratch.ad_value(1365)), scratch.values[386]));
        }

        scratch.values[1579] = if (scratch.values[389] == 0.0) { 1.0 } else { 0.0 };

        if ((((scratch.values[1323] != 0.0) && (scratch.values[1340] != 0.0)) && (!(scratch.values[1574] != 0.0))) && (scratch.values[1579] != 0.0)) {
            scratch.values[1368] = 0.0;
            scratch.node_derivatives[1368] = [0.0; Instance::NODE_COUNT];
            scratch.branch_derivatives[1368] = [0.0; Instance::BRANCH_COUNT];
        }

        if ((((scratch.values[1323] != 0.0) && (scratch.values[1340] != 0.0)) && (!(scratch.values[1574] != 0.0))) && (!(scratch.values[1579] != 0.0))) {
            scratch.store_ad(1369, &AdValue::scale(AdValue::div(AdValue::scale(scratch.ad_value(1366), scratch.values[457]), scratch.ad_value(1362)), scratch.values[487]));
        }

        if ((((scratch.values[1323] != 0.0) && (scratch.values[1340] != 0.0)) && (!(scratch.values[1574] != 0.0))) && (!(scratch.values[1579] != 0.0))) {
            scratch.store_ad(1370, &AdValue::div_from_scalar((0.666666666666667 * scratch.values[484]), scratch.ad_value(1369)));
        }

        if ((((scratch.values[1323] != 0.0) && (scratch.values[1340] != 0.0)) && (!(scratch.values[1574] != 0.0))) && (!(scratch.values[1579] != 0.0))) {
            scratch.store_ad(1371, &AdValue::square(scratch.ad_value(1370)));
        }

        if ((((scratch.values[1323] != 0.0) && (scratch.values[1340] != 0.0)) && (!(scratch.values[1574] != 0.0))) && (!(scratch.values[1579] != 0.0))) {
            scratch.store_ad(1372, &AdValue::sqrt(AdValue::div(AdValue::square(scratch.ad_value(1371)), AdValue::offset(AdValue::square(scratch.ad_value(1371)), 1.0))));
        }

        if ((((scratch.values[1323] != 0.0) && (scratch.values[1340] != 0.0)) && (!(scratch.values[1574] != 0.0))) && (!(scratch.values[1579] != 0.0))) {
            scratch.store_ad(1373, &AdValue::sqrt(AdValue::abs(scratch.ad_value(1372))));
        }

        if ((((scratch.values[1323] != 0.0) && (scratch.values[1340] != 0.0)) && (!(scratch.values[1574] != 0.0))) && (!(scratch.values[1579] != 0.0))) {
            scratch.store_ad(1374, &AdValue::mul(scratch.ad_value(1372), scratch.ad_value(1373)));
        }

        scratch.values[1580] = if (((-scratch.values[375]) * scratch.values[460]) == (-1.0)) { 1.0 } else { 0.0 };

        if (((((scratch.values[1323] != 0.0) && (scratch.values[1340] != 0.0)) && (!(scratch.values[1574] != 0.0))) && (!(scratch.values[1579] != 0.0))) && (scratch.values[1580] != 0.0)) {
            scratch.store_ad(1375, &AdValue::div_from_scalar(1.0, AdValue::offset(AdValue::mul(scratch.ad_value(1369), scratch.ad_value(1374)), 1.0)));
        }

        if (((((scratch.values[1323] != 0.0) && (scratch.values[1340] != 0.0)) && (!(scratch.values[1574] != 0.0))) && (!(scratch.values[1579] != 0.0))) && (!(scratch.values[1580] != 0.0))) {
            scratch.store_ad(1375, &AdValue::powf(AdValue::offset(AdValue::mul(scratch.ad_value(1369), scratch.ad_value(1374)), 1.0), ((-scratch.values[375]) * scratch.values[460])));
        }

        if ((((scratch.values[1323] != 0.0) && (scratch.values[1340] != 0.0)) && (!(scratch.values[1574] != 0.0))) && (!(scratch.values[1579] != 0.0))) {
            scratch.store_ad(1376, &AdValue::div(AdValue::mul(scratch.ad_value(1365), scratch.ad_value(1375)), AdValue::add(scratch.ad_value(1365), scratch.ad_value(1375))));
        }

        if ((((scratch.values[1323] != 0.0) && (scratch.values[1340] != 0.0)) && (!(scratch.values[1574] != 0.0))) && (!(scratch.values[1579] != 0.0))) {
            scratch.store_ad(1377, &AdValue::sqrt(AdValue::scale(AdValue::div(scratch.ad_value(1369), scratch.ad_value(1373)), 0.375)));
        }

        if ((((scratch.values[1323] != 0.0) && (scratch.values[1340] != 0.0)) && (!(scratch.values[1574] != 0.0))) && (!(scratch.values[1579] != 0.0))) {
            scratch.store_ad(1378, &AdValue::sub(AdValue::scale(AdValue::mul(scratch.ad_value(1370), scratch.ad_value(1373)), 2.0), scratch.ad_value(1372)));
        }

        if ((((scratch.values[1323] != 0.0) && (scratch.values[1340] != 0.0)) && (!(scratch.values[1574] != 0.0))) && (!(scratch.values[1579] != 0.0))) {
            scratch.store_ad(1379, &AdValue::add(AdValue::sub(AdValue::mul(AdValue::scale(scratch.ad_value(1370), scratch.values[484]), scratch.ad_value(1373)), AdValue::scale(scratch.ad_value(1372), scratch.values[484])), AdValue::scale(AdValue::mul(scratch.ad_value(1369), scratch.ad_value(1374)), 0.5)));
        }

        if ((((scratch.values[1323] != 0.0) && (scratch.values[1340] != 0.0)) && (!(scratch.values[1574] != 0.0))) && (!(scratch.values[1579] != 0.0))) {
            scratch.store_ad(1380, &AdValue::mul(AdValue::offset(scratch.ad_value(1378), (-1.0)), scratch.ad_value(1377)));
        }

        if ((((scratch.values[1323] != 0.0) && (scratch.values[1340] != 0.0)) && (!(scratch.values[1574] != 0.0))) && (!(scratch.values[1579] != 0.0))) {
            scratch.store_ad(1341, &AdValue::square(scratch.ad_value(1380)));
        }

        scratch.values[1581] = if (scratch.values[1380] > 0.0) { 1.0 } else { 0.0 };

        if (((((scratch.values[1323] != 0.0) && (scratch.values[1340] != 0.0)) && (!(scratch.values[1574] != 0.0))) && (!(scratch.values[1579] != 0.0))) && (scratch.values[1581] != 0.0)) {
            scratch.store_ad(1342, &AdValue::div_from_scalar(1.0, AdValue::offset(AdValue::scale(scratch.ad_value(1380), scratch.values[421]), 1.0)));
        }

        if (((((scratch.values[1323] != 0.0) && (scratch.values[1340] != 0.0)) && (!(scratch.values[1574] != 0.0))) && (!(scratch.values[1579] != 0.0))) && (!(scratch.values[1581] != 0.0))) {
            scratch.store_ad(1342, &AdValue::div_from_scalar(1.0, AdValue::sub_from_scalar(1.0, AdValue::scale(scratch.ad_value(1380), scratch.values[421]))));
        }

        scratch.values[1582] = if (((-scratch.values[1341]) + scratch.values[1379]) > (-230.25850929940458)) { 1.0 } else { 0.0 };

        if (((((scratch.values[1323] != 0.0) && (scratch.values[1340] != 0.0)) && (!(scratch.values[1574] != 0.0))) && (!(scratch.values[1579] != 0.0))) && (scratch.values[1582] != 0.0)) {
            scratch.store_ad(1359, &AdValue::exp(AdValue::sub(scratch.ad_value(1379), scratch.ad_value(1341))));
        }

        if (((((scratch.values[1323] != 0.0) && (scratch.values[1340] != 0.0)) && (!(scratch.values[1574] != 0.0))) && (!(scratch.values[1579] != 0.0))) && (!(scratch.values[1582] != 0.0))) {
            let assign22140_ad_e23770: AdValue = AdValue::div_from_scalar(1e-100, AdValue::offset(AdValue::mul(AdValue::sub_from_scalar((-230.25850929940458), AdValue::sub(scratch.ad_value(1379), scratch.ad_value(1341))), AdValue::offset(AdValue::scale(AdValue::mul(AdValue::sub_from_scalar((-230.25850929940458), AdValue::sub(scratch.ad_value(1379), scratch.ad_value(1341))), AdValue::offset(AdValue::scale(AdValue::sub_from_scalar((-230.25850929940458), AdValue::sub(scratch.ad_value(1379), scratch.ad_value(1341))), 0.3333333333333333), 1.0)), 0.5), 1.0)), 1.0));
            scratch.store_ad(1359, &assign22140_ad_e23770);
        }

        if ((((scratch.values[1323] != 0.0) && (scratch.values[1340] != 0.0)) && (!(scratch.values[1574] != 0.0))) && (!(scratch.values[1579] != 0.0))) {
            scratch.store_ad(1343, &AdValue::mul(AdValue::add(AdValue::add(AdValue::scale(scratch.ad_value(1342), 0.29214664), AdValue::scale(AdValue::square(scratch.ad_value(1342)), scratch.values[422])), AdValue::scale(AdValue::mul(AdValue::square(scratch.ad_value(1342)), scratch.ad_value(1342)), scratch.values[423])), scratch.ad_value(1359)));
        }

        scratch.values[1583] = if (scratch.values[1380] > 0.0) { 1.0 } else { 0.0 };

        if (((((scratch.values[1323] != 0.0) && (scratch.values[1340] != 0.0)) && (!(scratch.values[1574] != 0.0))) && (!(scratch.values[1579] != 0.0))) && (scratch.values[1583] != 0.0)) {
            scratch.values[1381] = scratch.values[1343];
            scratch.node_derivatives[1381] = scratch.node_derivatives[1343];
            scratch.branch_derivatives[1381] = scratch.branch_derivatives[1343];
        }

        scratch.values[1584] = if (scratch.values[1379] > (-230.25850929940458)) { 1.0 } else { 0.0 };

        if ((((((scratch.values[1323] != 0.0) && (scratch.values[1340] != 0.0)) && (!(scratch.values[1574] != 0.0))) && (!(scratch.values[1579] != 0.0))) && (!(scratch.values[1583] != 0.0))) && (scratch.values[1584] != 0.0)) {
            scratch.store_ad(1359, &AdValue::exp(scratch.ad_value(1379)));
        }

        if ((((((scratch.values[1323] != 0.0) && (scratch.values[1340] != 0.0)) && (!(scratch.values[1574] != 0.0))) && (!(scratch.values[1579] != 0.0))) && (!(scratch.values[1583] != 0.0))) && (!(scratch.values[1584] != 0.0))) {
            scratch.store_ad(1359, &AdValue::div_from_scalar(1e-100, AdValue::offset(AdValue::mul(AdValue::sub_from_scalar((-230.25850929940458), scratch.ad_value(1379)), AdValue::offset(AdValue::scale(AdValue::mul(AdValue::sub_from_scalar((-230.25850929940458), scratch.ad_value(1379)), AdValue::offset(AdValue::scale(AdValue::sub_from_scalar((-230.25850929940458), scratch.ad_value(1379)), 0.3333333333333333), 1.0)), 0.5), 1.0)), 1.0)));
        }

        if (((((scratch.values[1323] != 0.0) && (scratch.values[1340] != 0.0)) && (!(scratch.values[1574] != 0.0))) && (!(scratch.values[1579] != 0.0))) && (!(scratch.values[1583] != 0.0))) {
            scratch.store_ad(1381, &AdValue::sub(AdValue::scale(scratch.ad_value(1359), 2.0), scratch.ad_value(1343)));
        }

        if ((((scratch.values[1323] != 0.0) && (scratch.values[1340] != 0.0)) && (!(scratch.values[1574] != 0.0))) && (!(scratch.values[1579] != 0.0))) {
            scratch.store_ad(1382, &AdValue::scale(AdValue::div(AdValue::scale(scratch.ad_value(1381), scratch.values[484]), scratch.ad_value(1377)), (1.772453850905516 * 0.5)));
        }

        if ((((scratch.values[1323] != 0.0) && (scratch.values[1340] != 0.0)) && (!(scratch.values[1574] != 0.0))) && (!(scratch.values[1579] != 0.0))) {
            scratch.store_ad(1368, &AdValue::scale(AdValue::mul(AdValue::mul(scratch.ad_value(1367), scratch.ad_value(1382)), scratch.ad_value(1376)), scratch.values[389]));
        }

        scratch.values[1585] = if (scratch.values[395] == 0.0) { 1.0 } else { 0.0 };

        if ((((scratch.values[1323] != 0.0) && (scratch.values[1340] != 0.0)) && (!(scratch.values[1574] != 0.0))) && (scratch.values[1585] != 0.0)) {
            scratch.values[1383] = 0.0;
            scratch.node_derivatives[1383] = [0.0; Instance::NODE_COUNT];
            scratch.branch_derivatives[1383] = [0.0; Instance::BRANCH_COUNT];
        }

        scratch.values[1586] = if (scratch.values[375] == 0.5) { 1.0 } else { 0.0 };

        if (((((scratch.values[1323] != 0.0) && (scratch.values[1340] != 0.0)) && (!(scratch.values[1574] != 0.0))) && (!(scratch.values[1585] != 0.0))) && (scratch.values[1586] != 0.0)) {
            scratch.store_ad(1359, &AdValue::sqrt(AdValue::scale(AdValue::sub_from_scalar(scratch.values[372], scratch.ad_value(1357)), scratch.values[478])));
        }

        if (((((scratch.values[1323] != 0.0) && (scratch.values[1340] != 0.0)) && (!(scratch.values[1574] != 0.0))) && (!(scratch.values[1585] != 0.0))) && (!(scratch.values[1586] != 0.0))) {
            scratch.store_ad(1359, &AdValue::powf(AdValue::scale(AdValue::sub_from_scalar(scratch.values[372], scratch.ad_value(1357)), scratch.values[478]), scratch.values[375]));
        }

        if ((((scratch.values[1323] != 0.0) && (scratch.values[1340] != 0.0)) && (!(scratch.values[1574] != 0.0))) && (!(scratch.values[1585] != 0.0))) {
            scratch.store_ad(1384, &AdValue::scale(AdValue::div(AdValue::scale(AdValue::sub_from_scalar(scratch.values[372], scratch.ad_value(1357)), scratch.values[475]), scratch.ad_value(1359)), scratch.values[460]));
        }

        scratch.values[1587] = if (((((-scratch.values[490]) / scratch.values[1384])) as f64).abs() < 230.25850929940458) { 1.0 } else { 0.0 };

        if (((((scratch.values[1323] != 0.0) && (scratch.values[1340] != 0.0)) && (!(scratch.values[1574] != 0.0))) && (!(scratch.values[1585] != 0.0))) && (scratch.values[1587] != 0.0)) {
            scratch.store_ad(1359, &AdValue::exp(AdValue::div(AdValue::neg(scratch.ad_value(490)), scratch.ad_value(1384))));
        }

        scratch.values[1588] = if (((-scratch.values[490]) / scratch.values[1384]) < 0.0) { 1.0 } else { 0.0 };

        if ((((((scratch.values[1323] != 0.0) && (scratch.values[1340] != 0.0)) && (!(scratch.values[1574] != 0.0))) && (!(scratch.values[1585] != 0.0))) && (!(scratch.values[1587] != 0.0))) && (scratch.values[1588] != 0.0)) {
            let assign22330_ad_e24097: AdValue = AdValue::offset(AdValue::mul(AdValue::sub_from_scalar((-230.25850929940458), AdValue::div(AdValue::neg(scratch.ad_value(490)), scratch.ad_value(1384))), AdValue::offset(AdValue::scale(AdValue::mul(AdValue::sub_from_scalar((-230.25850929940458), AdValue::div(AdValue::neg(scratch.ad_value(490)), scratch.ad_value(1384))), AdValue::offset(AdValue::scale(AdValue::sub_from_scalar((-230.25850929940458), AdValue::div(AdValue::neg(scratch.ad_value(490)), scratch.ad_value(1384))), 0.3333333333333333), 1.0)), 0.5), 1.0)), 1.0);
            scratch.store_ad(1359, &AdValue::div_from_scalar(1e-100, assign22330_ad_e24097));
        }

        if ((((((scratch.values[1323] != 0.0) && (scratch.values[1340] != 0.0)) && (!(scratch.values[1574] != 0.0))) && (!(scratch.values[1585] != 0.0))) && (!(scratch.values[1587] != 0.0))) && (!(scratch.values[1588] != 0.0))) {
            let assign22340_ad_e24147: AdValue = AdValue::scale(AdValue::offset(AdValue::mul(AdValue::offset(AdValue::div(AdValue::neg(scratch.ad_value(490)), scratch.ad_value(1384)), (-230.25850929940458)), AdValue::offset(AdValue::scale(AdValue::mul(AdValue::offset(AdValue::div(AdValue::neg(scratch.ad_value(490)), scratch.ad_value(1384)), (-230.25850929940458)), AdValue::offset(AdValue::scale(AdValue::offset(AdValue::div(AdValue::neg(scratch.ad_value(490)), scratch.ad_value(1384)), (-230.25850929940458)), 0.3333333333333333), 1.0)), 0.5), 1.0)), 1.0), 1e100);
            scratch.store_ad(1359, &assign22340_ad_e24147);
        }

        if ((((scratch.values[1323] != 0.0) && (scratch.values[1340] != 0.0)) && (!(scratch.values[1574] != 0.0))) && (!(scratch.values[1585] != 0.0))) {
            scratch.store_ad(1383, &AdValue::scale(AdValue::mul(AdValue::mul(AdValue::mul(scratch.ad_value(519), scratch.ad_value(1384)), scratch.ad_value(1384)), scratch.ad_value(1359)), scratch.values[395]));
        }

        scratch.values[1589] = if (scratch.values[404] > 1000.0) { 1.0 } else { 0.0 };

        if ((((scratch.values[1323] != 0.0) && (scratch.values[1340] != 0.0)) && (!(scratch.values[1574] != 0.0))) && (scratch.values[1589] != 0.0)) {
            scratch.values[1385] = 1.0;
            scratch.node_derivatives[1385] = [0.0; Instance::NODE_COUNT];
            scratch.branch_derivatives[1385] = [0.0; Instance::BRANCH_COUNT];
        }

        scratch.values[1590] = if (scratch.values[1358] > ((-scratch.values[493]) * scratch.values[404])) { 1.0 } else { 0.0 };

        scratch.values[1591] = if (scratch.values[407] == 4.0) { 1.0 } else { 0.0 };

        if ((((((scratch.values[1323] != 0.0) && (scratch.values[1340] != 0.0)) && (!(scratch.values[1574] != 0.0))) && (!(scratch.values[1589] != 0.0))) && (scratch.values[1590] != 0.0)) && (scratch.values[1591] != 0.0)) {
            scratch.store_ad(1359, &AdValue::mul(AdValue::mul(AdValue::mul(AdValue::scale(scratch.ad_value(1358), scratch.values[497]), AdValue::scale(scratch.ad_value(1358), scratch.values[497])), AdValue::scale(scratch.ad_value(1358), scratch.values[497])), AdValue::scale(scratch.ad_value(1358), scratch.values[497])));
        }

        if ((((((scratch.values[1323] != 0.0) && (scratch.values[1340] != 0.0)) && (!(scratch.values[1574] != 0.0))) && (!(scratch.values[1589] != 0.0))) && (scratch.values[1590] != 0.0)) && (!(scratch.values[1591] != 0.0))) {
            scratch.store_ad(1359, &AdValue::powf(AdValue::abs(AdValue::scale(scratch.ad_value(1358), scratch.values[497])), scratch.values[407]));
        }

        if (((((scratch.values[1323] != 0.0) && (scratch.values[1340] != 0.0)) && (!(scratch.values[1574] != 0.0))) && (!(scratch.values[1589] != 0.0))) && (scratch.values[1590] != 0.0)) {
            scratch.store_ad(1385, &AdValue::div_from_scalar(1.0, AdValue::sub_from_scalar(1.0, scratch.ad_value(1359))));
        }

        if (((((scratch.values[1323] != 0.0) && (scratch.values[1340] != 0.0)) && (!(scratch.values[1574] != 0.0))) && (!(scratch.values[1589] != 0.0))) && (!(scratch.values[1590] != 0.0))) {
            scratch.store_ad(1385, &AdValue::offset(AdValue::scale(AdValue::offset(scratch.ad_value(1358), (scratch.values[493] * scratch.values[404])), scratch.values[500]), scratch.values[494]));
        }

        if (((scratch.values[1323] != 0.0) && (scratch.values[1340] != 0.0)) && (!(scratch.values[1574] != 0.0))) {
            scratch.store_ad(1386, &AdValue::mul(AdValue::add(AdValue::add(AdValue::add(scratch.ad_value(1360), scratch.ad_value(1361)), scratch.ad_value(1368)), scratch.ad_value(1383)), scratch.ad_value(1385)));
        }

        scratch.values[1592] = if (scratch.values[691] == 0.0) { 1.0 } else { 0.0 };

        if (((scratch.values[1323] != 0.0) && (scratch.values[1340] != 0.0)) && (scratch.values[1592] != 0.0)) {
            scratch.values[1387] = 0.0;
            scratch.node_derivatives[1387] = [0.0; Instance::NODE_COUNT];
            scratch.branch_derivatives[1387] = [0.0; Instance::BRANCH_COUNT];
        }

        scratch.values[1593] = if (scratch.values[458] == 0.5) { 1.0 } else { 0.0 };

        if ((((scratch.values[1323] != 0.0) && (scratch.values[1340] != 0.0)) && (!(scratch.values[1592] != 0.0))) && (scratch.values[1593] != 0.0)) {
            scratch.store_ad(1359, &AdValue::sqrt(AdValue::sub_from_scalar(1.0, AdValue::scale(scratch.ad_value(1351), scratch.values[455]))));
        }

        if ((((scratch.values[1323] != 0.0) && (scratch.values[1340] != 0.0)) && (!(scratch.values[1592] != 0.0))) && (!(scratch.values[1593] != 0.0))) {
            scratch.store_ad(1359, &AdValue::powf(AdValue::sub_from_scalar(1.0, AdValue::scale(scratch.ad_value(1351), scratch.values[455])), scratch.values[458]));
        }

        if (((scratch.values[1323] != 0.0) && (scratch.values[1340] != 0.0)) && (!(scratch.values[1592] != 0.0))) {
            scratch.store_ad(1360, &AdValue::scale(scratch.ad_value(1350), scratch.values[437]));
        }

        scratch.values[1594] = if ((scratch.values[387] == 0.0) && (scratch.values[390] == 0.0)) { 1.0 } else { 0.0 };

        if ((((scratch.values[1323] != 0.0) && (scratch.values[1340] != 0.0)) && (!(scratch.values[1592] != 0.0))) && (scratch.values[1594] != 0.0)) {
            scratch.values[1361] = 0.0;
            scratch.node_derivatives[1361] = [0.0; Instance::NODE_COUNT];
            scratch.branch_derivatives[1361] = [0.0; Instance::BRANCH_COUNT];
        }

        if ((((scratch.values[1323] != 0.0) && (scratch.values[1340] != 0.0)) && (!(scratch.values[1592] != 0.0))) && (!(scratch.values[1594] != 0.0))) {
            scratch.store_ad(1362, &AdValue::sub_from_scalar(scratch.values[443], scratch.ad_value(1356)));
        }

        if ((((scratch.values[1323] != 0.0) && (scratch.values[1340] != 0.0)) && (!(scratch.values[1592] != 0.0))) && (!(scratch.values[1594] != 0.0))) {
            scratch.store_ad(1363, &AdValue::sub_from_scalar(1.0, AdValue::sqrt(AdValue::sub_from_scalar(1.0, AdValue::div(scratch.ad_value(1354), scratch.ad_value(1362))))));
        }

        scratch.values[1595] = if (scratch.values[376] == 0.5) { 1.0 } else { 0.0 };

        if (((((scratch.values[1323] != 0.0) && (scratch.values[1340] != 0.0)) && (!(scratch.values[1592] != 0.0))) && (!(scratch.values[1594] != 0.0))) && (scratch.values[1595] != 0.0)) {
            scratch.values[1364] = 0.0;
            scratch.node_derivatives[1364] = [0.0; Instance::NODE_COUNT];
            scratch.branch_derivatives[1364] = [0.0; Instance::BRANCH_COUNT];
        }

        if (((((scratch.values[1323] != 0.0) && (scratch.values[1340] != 0.0)) && (!(scratch.values[1592] != 0.0))) && (!(scratch.values[1594] != 0.0))) && (!(scratch.values[1595] != 0.0))) {
            scratch.store_ad(1364, &AdValue::scale(AdValue::add(AdValue::div(AdValue::mul(AdValue::square(scratch.ad_value(1363)), AdValue::ln(scratch.ad_value(1363))), AdValue::sub_from_scalar(1.0, scratch.ad_value(1363))), scratch.ad_value(1363)), (1.0 - (2.0 * scratch.values[376]))));
        }

        if ((((scratch.values[1323] != 0.0) && (scratch.values[1340] != 0.0)) && (!(scratch.values[1592] != 0.0))) && (!(scratch.values[1594] != 0.0))) {
            scratch.store_ad(1365, &AdValue::add(scratch.ad_value(1363), scratch.ad_value(1364)));
        }

        scratch.values[1596] = if (scratch.values[376] == 0.5) { 1.0 } else { 0.0 };

        if (((((scratch.values[1323] != 0.0) && (scratch.values[1340] != 0.0)) && (!(scratch.values[1592] != 0.0))) && (!(scratch.values[1594] != 0.0))) && (scratch.values[1596] != 0.0)) {
            scratch.store_ad(1359, &AdValue::sqrt(AdValue::scale(scratch.ad_value(1362), scratch.values[479])));
        }

        if (((((scratch.values[1323] != 0.0) && (scratch.values[1340] != 0.0)) && (!(scratch.values[1592] != 0.0))) && (!(scratch.values[1594] != 0.0))) && (!(scratch.values[1596] != 0.0))) {
            scratch.store_ad(1359, &AdValue::powf(AdValue::scale(scratch.ad_value(1362), scratch.values[479]), scratch.values[376]));
        }

        if ((((scratch.values[1323] != 0.0) && (scratch.values[1340] != 0.0)) && (!(scratch.values[1592] != 0.0))) && (!(scratch.values[1594] != 0.0))) {
            scratch.store_ad(1366, &AdValue::scale(scratch.ad_value(1359), scratch.values[473]));
        }

        if ((((scratch.values[1323] != 0.0) && (scratch.values[1340] != 0.0)) && (!(scratch.values[1592] != 0.0))) && (!(scratch.values[1594] != 0.0))) {
            scratch.store_ad(1367, &AdValue::scale(AdValue::mul(AdValue::offset(scratch.ad_value(1353), (-1.0)), scratch.ad_value(1366)), scratch.values[434]));
        }

        if ((((scratch.values[1323] != 0.0) && (scratch.values[1340] != 0.0)) && (!(scratch.values[1592] != 0.0))) && (!(scratch.values[1594] != 0.0))) {
            scratch.store_ad(1361, &AdValue::scale(AdValue::mul(scratch.ad_value(1367), scratch.ad_value(1365)), scratch.values[387]));
        }

        scratch.values[1597] = if (scratch.values[390] == 0.0) { 1.0 } else { 0.0 };

        if ((((scratch.values[1323] != 0.0) && (scratch.values[1340] != 0.0)) && (!(scratch.values[1592] != 0.0))) && (scratch.values[1597] != 0.0)) {
            scratch.values[1368] = 0.0;
            scratch.node_derivatives[1368] = [0.0; Instance::NODE_COUNT];
            scratch.branch_derivatives[1368] = [0.0; Instance::BRANCH_COUNT];
        }

        if ((((scratch.values[1323] != 0.0) && (scratch.values[1340] != 0.0)) && (!(scratch.values[1592] != 0.0))) && (!(scratch.values[1597] != 0.0))) {
            scratch.store_ad(1369, &AdValue::scale(AdValue::div(AdValue::scale(scratch.ad_value(1366), scratch.values[458]), scratch.ad_value(1362)), scratch.values[488]));
        }

        if ((((scratch.values[1323] != 0.0) && (scratch.values[1340] != 0.0)) && (!(scratch.values[1592] != 0.0))) && (!(scratch.values[1597] != 0.0))) {
            scratch.store_ad(1370, &AdValue::div_from_scalar((0.666666666666667 * scratch.values[485]), scratch.ad_value(1369)));
        }

        if ((((scratch.values[1323] != 0.0) && (scratch.values[1340] != 0.0)) && (!(scratch.values[1592] != 0.0))) && (!(scratch.values[1597] != 0.0))) {
            scratch.store_ad(1371, &AdValue::square(scratch.ad_value(1370)));
        }

        if ((((scratch.values[1323] != 0.0) && (scratch.values[1340] != 0.0)) && (!(scratch.values[1592] != 0.0))) && (!(scratch.values[1597] != 0.0))) {
            scratch.store_ad(1372, &AdValue::sqrt(AdValue::div(AdValue::square(scratch.ad_value(1371)), AdValue::offset(AdValue::square(scratch.ad_value(1371)), 1.0))));
        }

        if ((((scratch.values[1323] != 0.0) && (scratch.values[1340] != 0.0)) && (!(scratch.values[1592] != 0.0))) && (!(scratch.values[1597] != 0.0))) {
            scratch.store_ad(1373, &AdValue::sqrt(AdValue::abs(scratch.ad_value(1372))));
        }

        if ((((scratch.values[1323] != 0.0) && (scratch.values[1340] != 0.0)) && (!(scratch.values[1592] != 0.0))) && (!(scratch.values[1597] != 0.0))) {
            scratch.store_ad(1374, &AdValue::mul(scratch.ad_value(1372), scratch.ad_value(1373)));
        }

        scratch.values[1598] = if (((-scratch.values[376]) * scratch.values[461]) == (-1.0)) { 1.0 } else { 0.0 };

        if (((((scratch.values[1323] != 0.0) && (scratch.values[1340] != 0.0)) && (!(scratch.values[1592] != 0.0))) && (!(scratch.values[1597] != 0.0))) && (scratch.values[1598] != 0.0)) {
            scratch.store_ad(1375, &AdValue::div_from_scalar(1.0, AdValue::offset(AdValue::mul(scratch.ad_value(1369), scratch.ad_value(1374)), 1.0)));
        }

        if (((((scratch.values[1323] != 0.0) && (scratch.values[1340] != 0.0)) && (!(scratch.values[1592] != 0.0))) && (!(scratch.values[1597] != 0.0))) && (!(scratch.values[1598] != 0.0))) {
            scratch.store_ad(1375, &AdValue::powf(AdValue::offset(AdValue::mul(scratch.ad_value(1369), scratch.ad_value(1374)), 1.0), ((-scratch.values[376]) * scratch.values[461])));
        }

        if ((((scratch.values[1323] != 0.0) && (scratch.values[1340] != 0.0)) && (!(scratch.values[1592] != 0.0))) && (!(scratch.values[1597] != 0.0))) {
            scratch.store_ad(1376, &AdValue::div(AdValue::mul(scratch.ad_value(1365), scratch.ad_value(1375)), AdValue::add(scratch.ad_value(1365), scratch.ad_value(1375))));
        }

        if ((((scratch.values[1323] != 0.0) && (scratch.values[1340] != 0.0)) && (!(scratch.values[1592] != 0.0))) && (!(scratch.values[1597] != 0.0))) {
            scratch.store_ad(1377, &AdValue::sqrt(AdValue::scale(AdValue::div(scratch.ad_value(1369), scratch.ad_value(1373)), 0.375)));
        }

        if ((((scratch.values[1323] != 0.0) && (scratch.values[1340] != 0.0)) && (!(scratch.values[1592] != 0.0))) && (!(scratch.values[1597] != 0.0))) {
            scratch.store_ad(1378, &AdValue::sub(AdValue::scale(AdValue::mul(scratch.ad_value(1370), scratch.ad_value(1373)), 2.0), scratch.ad_value(1372)));
        }

        if ((((scratch.values[1323] != 0.0) && (scratch.values[1340] != 0.0)) && (!(scratch.values[1592] != 0.0))) && (!(scratch.values[1597] != 0.0))) {
            scratch.store_ad(1379, &AdValue::add(AdValue::sub(AdValue::mul(AdValue::scale(scratch.ad_value(1370), scratch.values[485]), scratch.ad_value(1373)), AdValue::scale(scratch.ad_value(1372), scratch.values[485])), AdValue::scale(AdValue::mul(scratch.ad_value(1369), scratch.ad_value(1374)), 0.5)));
        }

        if ((((scratch.values[1323] != 0.0) && (scratch.values[1340] != 0.0)) && (!(scratch.values[1592] != 0.0))) && (!(scratch.values[1597] != 0.0))) {
            scratch.store_ad(1380, &AdValue::mul(AdValue::offset(scratch.ad_value(1378), (-1.0)), scratch.ad_value(1377)));
        }

        if ((((scratch.values[1323] != 0.0) && (scratch.values[1340] != 0.0)) && (!(scratch.values[1592] != 0.0))) && (!(scratch.values[1597] != 0.0))) {
            scratch.store_ad(1341, &AdValue::square(scratch.ad_value(1380)));
        }

        scratch.values[1599] = if (scratch.values[1380] > 0.0) { 1.0 } else { 0.0 };

        if (((((scratch.values[1323] != 0.0) && (scratch.values[1340] != 0.0)) && (!(scratch.values[1592] != 0.0))) && (!(scratch.values[1597] != 0.0))) && (scratch.values[1599] != 0.0)) {
            scratch.store_ad(1342, &AdValue::div_from_scalar(1.0, AdValue::offset(AdValue::scale(scratch.ad_value(1380), scratch.values[421]), 1.0)));
        }

        if (((((scratch.values[1323] != 0.0) && (scratch.values[1340] != 0.0)) && (!(scratch.values[1592] != 0.0))) && (!(scratch.values[1597] != 0.0))) && (!(scratch.values[1599] != 0.0))) {
            scratch.store_ad(1342, &AdValue::div_from_scalar(1.0, AdValue::sub_from_scalar(1.0, AdValue::scale(scratch.ad_value(1380), scratch.values[421]))));
        }

        scratch.values[1600] = if (((-scratch.values[1341]) + scratch.values[1379]) > (-230.25850929940458)) { 1.0 } else { 0.0 };

        if (((((scratch.values[1323] != 0.0) && (scratch.values[1340] != 0.0)) && (!(scratch.values[1592] != 0.0))) && (!(scratch.values[1597] != 0.0))) && (scratch.values[1600] != 0.0)) {
            scratch.store_ad(1359, &AdValue::exp(AdValue::sub(scratch.ad_value(1379), scratch.ad_value(1341))));
        }

        if (((((scratch.values[1323] != 0.0) && (scratch.values[1340] != 0.0)) && (!(scratch.values[1592] != 0.0))) && (!(scratch.values[1597] != 0.0))) && (!(scratch.values[1600] != 0.0))) {
            let assign22890_ad_e24976: AdValue = AdValue::div_from_scalar(1e-100, AdValue::offset(AdValue::mul(AdValue::sub_from_scalar((-230.25850929940458), AdValue::sub(scratch.ad_value(1379), scratch.ad_value(1341))), AdValue::offset(AdValue::scale(AdValue::mul(AdValue::sub_from_scalar((-230.25850929940458), AdValue::sub(scratch.ad_value(1379), scratch.ad_value(1341))), AdValue::offset(AdValue::scale(AdValue::sub_from_scalar((-230.25850929940458), AdValue::sub(scratch.ad_value(1379), scratch.ad_value(1341))), 0.3333333333333333), 1.0)), 0.5), 1.0)), 1.0));
            scratch.store_ad(1359, &assign22890_ad_e24976);
        }

        if ((((scratch.values[1323] != 0.0) && (scratch.values[1340] != 0.0)) && (!(scratch.values[1592] != 0.0))) && (!(scratch.values[1597] != 0.0))) {
            scratch.store_ad(1343, &AdValue::mul(AdValue::add(AdValue::add(AdValue::scale(scratch.ad_value(1342), 0.29214664), AdValue::scale(AdValue::square(scratch.ad_value(1342)), scratch.values[422])), AdValue::scale(AdValue::mul(AdValue::square(scratch.ad_value(1342)), scratch.ad_value(1342)), scratch.values[423])), scratch.ad_value(1359)));
        }

        scratch.values[1601] = if (scratch.values[1380] > 0.0) { 1.0 } else { 0.0 };

        if (((((scratch.values[1323] != 0.0) && (scratch.values[1340] != 0.0)) && (!(scratch.values[1592] != 0.0))) && (!(scratch.values[1597] != 0.0))) && (scratch.values[1601] != 0.0)) {
            scratch.values[1381] = scratch.values[1343];
            scratch.node_derivatives[1381] = scratch.node_derivatives[1343];
            scratch.branch_derivatives[1381] = scratch.branch_derivatives[1343];
        }

        scratch.values[1602] = if (scratch.values[1379] > (-230.25850929940458)) { 1.0 } else { 0.0 };

        if ((((((scratch.values[1323] != 0.0) && (scratch.values[1340] != 0.0)) && (!(scratch.values[1592] != 0.0))) && (!(scratch.values[1597] != 0.0))) && (!(scratch.values[1601] != 0.0))) && (scratch.values[1602] != 0.0)) {
            scratch.store_ad(1359, &AdValue::exp(scratch.ad_value(1379)));
        }

        if ((((((scratch.values[1323] != 0.0) && (scratch.values[1340] != 0.0)) && (!(scratch.values[1592] != 0.0))) && (!(scratch.values[1597] != 0.0))) && (!(scratch.values[1601] != 0.0))) && (!(scratch.values[1602] != 0.0))) {
            scratch.store_ad(1359, &AdValue::div_from_scalar(1e-100, AdValue::offset(AdValue::mul(AdValue::sub_from_scalar((-230.25850929940458), scratch.ad_value(1379)), AdValue::offset(AdValue::scale(AdValue::mul(AdValue::sub_from_scalar((-230.25850929940458), scratch.ad_value(1379)), AdValue::offset(AdValue::scale(AdValue::sub_from_scalar((-230.25850929940458), scratch.ad_value(1379)), 0.3333333333333333), 1.0)), 0.5), 1.0)), 1.0)));
        }

        if (((((scratch.values[1323] != 0.0) && (scratch.values[1340] != 0.0)) && (!(scratch.values[1592] != 0.0))) && (!(scratch.values[1597] != 0.0))) && (!(scratch.values[1601] != 0.0))) {
            scratch.store_ad(1381, &AdValue::sub(AdValue::scale(scratch.ad_value(1359), 2.0), scratch.ad_value(1343)));
        }

        if ((((scratch.values[1323] != 0.0) && (scratch.values[1340] != 0.0)) && (!(scratch.values[1592] != 0.0))) && (!(scratch.values[1597] != 0.0))) {
            scratch.store_ad(1382, &AdValue::scale(AdValue::div(AdValue::scale(scratch.ad_value(1381), scratch.values[485]), scratch.ad_value(1377)), (1.772453850905516 * 0.5)));
        }

        if ((((scratch.values[1323] != 0.0) && (scratch.values[1340] != 0.0)) && (!(scratch.values[1592] != 0.0))) && (!(scratch.values[1597] != 0.0))) {
            scratch.store_ad(1368, &AdValue::scale(AdValue::mul(AdValue::mul(scratch.ad_value(1367), scratch.ad_value(1382)), scratch.ad_value(1376)), scratch.values[390]));
        }

        scratch.values[1603] = if (scratch.values[396] == 0.0) { 1.0 } else { 0.0 };

        if ((((scratch.values[1323] != 0.0) && (scratch.values[1340] != 0.0)) && (!(scratch.values[1592] != 0.0))) && (scratch.values[1603] != 0.0)) {
            scratch.values[1383] = 0.0;
            scratch.node_derivatives[1383] = [0.0; Instance::NODE_COUNT];
            scratch.branch_derivatives[1383] = [0.0; Instance::BRANCH_COUNT];
        }

        scratch.values[1604] = if (scratch.values[376] == 0.5) { 1.0 } else { 0.0 };

        if (((((scratch.values[1323] != 0.0) && (scratch.values[1340] != 0.0)) && (!(scratch.values[1592] != 0.0))) && (!(scratch.values[1603] != 0.0))) && (scratch.values[1604] != 0.0)) {
            scratch.store_ad(1359, &AdValue::sqrt(AdValue::scale(AdValue::sub_from_scalar(scratch.values[373], scratch.ad_value(1357)), scratch.values[479])));
        }

        if (((((scratch.values[1323] != 0.0) && (scratch.values[1340] != 0.0)) && (!(scratch.values[1592] != 0.0))) && (!(scratch.values[1603] != 0.0))) && (!(scratch.values[1604] != 0.0))) {
            scratch.store_ad(1359, &AdValue::powf(AdValue::scale(AdValue::sub_from_scalar(scratch.values[373], scratch.ad_value(1357)), scratch.values[479]), scratch.values[376]));
        }

        if ((((scratch.values[1323] != 0.0) && (scratch.values[1340] != 0.0)) && (!(scratch.values[1592] != 0.0))) && (!(scratch.values[1603] != 0.0))) {
            scratch.store_ad(1384, &AdValue::scale(AdValue::div(AdValue::scale(AdValue::sub_from_scalar(scratch.values[373], scratch.ad_value(1357)), scratch.values[476]), scratch.ad_value(1359)), scratch.values[461]));
        }

        scratch.values[1605] = if (((((-scratch.values[491]) / scratch.values[1384])) as f64).abs() < 230.25850929940458) { 1.0 } else { 0.0 };

        if (((((scratch.values[1323] != 0.0) && (scratch.values[1340] != 0.0)) && (!(scratch.values[1592] != 0.0))) && (!(scratch.values[1603] != 0.0))) && (scratch.values[1605] != 0.0)) {
            scratch.store_ad(1359, &AdValue::exp(AdValue::div(AdValue::neg(scratch.ad_value(491)), scratch.ad_value(1384))));
        }

        scratch.values[1606] = if (((-scratch.values[491]) / scratch.values[1384]) < 0.0) { 1.0 } else { 0.0 };

        if ((((((scratch.values[1323] != 0.0) && (scratch.values[1340] != 0.0)) && (!(scratch.values[1592] != 0.0))) && (!(scratch.values[1603] != 0.0))) && (!(scratch.values[1605] != 0.0))) && (scratch.values[1606] != 0.0)) {
            let assign23080_ad_e25303: AdValue = AdValue::offset(AdValue::mul(AdValue::sub_from_scalar((-230.25850929940458), AdValue::div(AdValue::neg(scratch.ad_value(491)), scratch.ad_value(1384))), AdValue::offset(AdValue::scale(AdValue::mul(AdValue::sub_from_scalar((-230.25850929940458), AdValue::div(AdValue::neg(scratch.ad_value(491)), scratch.ad_value(1384))), AdValue::offset(AdValue::scale(AdValue::sub_from_scalar((-230.25850929940458), AdValue::div(AdValue::neg(scratch.ad_value(491)), scratch.ad_value(1384))), 0.3333333333333333), 1.0)), 0.5), 1.0)), 1.0);
            scratch.store_ad(1359, &AdValue::div_from_scalar(1e-100, assign23080_ad_e25303));
        }

        if ((((((scratch.values[1323] != 0.0) && (scratch.values[1340] != 0.0)) && (!(scratch.values[1592] != 0.0))) && (!(scratch.values[1603] != 0.0))) && (!(scratch.values[1605] != 0.0))) && (!(scratch.values[1606] != 0.0))) {
            let assign23090_ad_e25353: AdValue = AdValue::scale(AdValue::offset(AdValue::mul(AdValue::offset(AdValue::div(AdValue::neg(scratch.ad_value(491)), scratch.ad_value(1384)), (-230.25850929940458)), AdValue::offset(AdValue::scale(AdValue::mul(AdValue::offset(AdValue::div(AdValue::neg(scratch.ad_value(491)), scratch.ad_value(1384)), (-230.25850929940458)), AdValue::offset(AdValue::scale(AdValue::offset(AdValue::div(AdValue::neg(scratch.ad_value(491)), scratch.ad_value(1384)), (-230.25850929940458)), 0.3333333333333333), 1.0)), 0.5), 1.0)), 1.0), 1e100);
            scratch.store_ad(1359, &assign23090_ad_e25353);
        }

        if ((((scratch.values[1323] != 0.0) && (scratch.values[1340] != 0.0)) && (!(scratch.values[1592] != 0.0))) && (!(scratch.values[1603] != 0.0))) {
            scratch.store_ad(1383, &AdValue::scale(AdValue::mul(AdValue::mul(AdValue::mul(scratch.ad_value(519), scratch.ad_value(1384)), scratch.ad_value(1384)), scratch.ad_value(1359)), scratch.values[396]));
        }

        scratch.values[1607] = if (scratch.values[405] > 1000.0) { 1.0 } else { 0.0 };

        if ((((scratch.values[1323] != 0.0) && (scratch.values[1340] != 0.0)) && (!(scratch.values[1592] != 0.0))) && (scratch.values[1607] != 0.0)) {
            scratch.values[1385] = 1.0;
            scratch.node_derivatives[1385] = [0.0; Instance::NODE_COUNT];
            scratch.branch_derivatives[1385] = [0.0; Instance::BRANCH_COUNT];
        }

        scratch.values[1608] = if (scratch.values[1358] > ((-scratch.values[493]) * scratch.values[405])) { 1.0 } else { 0.0 };

        scratch.values[1609] = if (scratch.values[408] == 4.0) { 1.0 } else { 0.0 };

        if ((((((scratch.values[1323] != 0.0) && (scratch.values[1340] != 0.0)) && (!(scratch.values[1592] != 0.0))) && (!(scratch.values[1607] != 0.0))) && (scratch.values[1608] != 0.0)) && (scratch.values[1609] != 0.0)) {
            scratch.store_ad(1359, &AdValue::mul(AdValue::mul(AdValue::mul(AdValue::scale(scratch.ad_value(1358), scratch.values[498]), AdValue::scale(scratch.ad_value(1358), scratch.values[498])), AdValue::scale(scratch.ad_value(1358), scratch.values[498])), AdValue::scale(scratch.ad_value(1358), scratch.values[498])));
        }

        if ((((((scratch.values[1323] != 0.0) && (scratch.values[1340] != 0.0)) && (!(scratch.values[1592] != 0.0))) && (!(scratch.values[1607] != 0.0))) && (scratch.values[1608] != 0.0)) && (!(scratch.values[1609] != 0.0))) {
            scratch.store_ad(1359, &AdValue::powf(AdValue::abs(AdValue::scale(scratch.ad_value(1358), scratch.values[498])), scratch.values[408]));
        }

        if (((((scratch.values[1323] != 0.0) && (scratch.values[1340] != 0.0)) && (!(scratch.values[1592] != 0.0))) && (!(scratch.values[1607] != 0.0))) && (scratch.values[1608] != 0.0)) {
            scratch.store_ad(1385, &AdValue::div_from_scalar(1.0, AdValue::sub_from_scalar(1.0, scratch.ad_value(1359))));
        }

        if (((((scratch.values[1323] != 0.0) && (scratch.values[1340] != 0.0)) && (!(scratch.values[1592] != 0.0))) && (!(scratch.values[1607] != 0.0))) && (!(scratch.values[1608] != 0.0))) {
            scratch.store_ad(1385, &AdValue::offset(AdValue::scale(AdValue::offset(scratch.ad_value(1358), (scratch.values[493] * scratch.values[405])), scratch.values[501]), scratch.values[495]));
        }

        if (((scratch.values[1323] != 0.0) && (scratch.values[1340] != 0.0)) && (!(scratch.values[1592] != 0.0))) {
            scratch.store_ad(1387, &AdValue::mul(AdValue::add(AdValue::add(AdValue::add(scratch.ad_value(1360), scratch.ad_value(1361)), scratch.ad_value(1368)), scratch.ad_value(1383)), scratch.ad_value(1385)));
        }

        scratch.values[1610] = if (scratch.values[692] == 0.0) { 1.0 } else { 0.0 };

        if (((scratch.values[1323] != 0.0) && (scratch.values[1340] != 0.0)) && (scratch.values[1610] != 0.0)) {
            scratch.values[1388] = 0.0;
            scratch.node_derivatives[1388] = [0.0; Instance::NODE_COUNT];
            scratch.branch_derivatives[1388] = [0.0; Instance::BRANCH_COUNT];
        }

        scratch.values[1611] = if (scratch.values[459] == 0.5) { 1.0 } else { 0.0 };

        if ((((scratch.values[1323] != 0.0) && (scratch.values[1340] != 0.0)) && (!(scratch.values[1610] != 0.0))) && (scratch.values[1611] != 0.0)) {
            scratch.store_ad(1359, &AdValue::sqrt(AdValue::sub_from_scalar(1.0, AdValue::scale(scratch.ad_value(1351), scratch.values[456]))));
        }

    }

    pub(super) fn stamp_transient_block_18(
        &mut self,
        ctx: &GeneratedEvalContext<'_>,
        stamper: &mut GeneratedStamper<'_>,
        scratch: &mut Scratch,
    ) {
        let _ = stamper;
        if ((((scratch.values[1323] != 0.0) && (scratch.values[1340] != 0.0)) && (!(scratch.values[1610] != 0.0))) && (!(scratch.values[1611] != 0.0))) {
            scratch.store_ad(1359, &AdValue::powf(AdValue::sub_from_scalar(1.0, AdValue::scale(scratch.ad_value(1351), scratch.values[456])), scratch.values[459]));
        }

        if (((scratch.values[1323] != 0.0) && (scratch.values[1340] != 0.0)) && (!(scratch.values[1610] != 0.0))) {
            scratch.store_ad(1360, &AdValue::scale(scratch.ad_value(1350), scratch.values[438]));
        }

        scratch.values[1612] = if ((scratch.values[388] == 0.0) && (scratch.values[391] == 0.0)) { 1.0 } else { 0.0 };

        if ((((scratch.values[1323] != 0.0) && (scratch.values[1340] != 0.0)) && (!(scratch.values[1610] != 0.0))) && (scratch.values[1612] != 0.0)) {
            scratch.values[1361] = 0.0;
            scratch.node_derivatives[1361] = [0.0; Instance::NODE_COUNT];
            scratch.branch_derivatives[1361] = [0.0; Instance::BRANCH_COUNT];
        }

        if ((((scratch.values[1323] != 0.0) && (scratch.values[1340] != 0.0)) && (!(scratch.values[1610] != 0.0))) && (!(scratch.values[1612] != 0.0))) {
            scratch.store_ad(1362, &AdValue::sub_from_scalar(scratch.values[444], scratch.ad_value(1356)));
        }

        if ((((scratch.values[1323] != 0.0) && (scratch.values[1340] != 0.0)) && (!(scratch.values[1610] != 0.0))) && (!(scratch.values[1612] != 0.0))) {
            scratch.store_ad(1363, &AdValue::sub_from_scalar(1.0, AdValue::sqrt(AdValue::sub_from_scalar(1.0, AdValue::div(scratch.ad_value(1354), scratch.ad_value(1362))))));
        }

        scratch.values[1613] = if (scratch.values[377] == 0.5) { 1.0 } else { 0.0 };

        if (((((scratch.values[1323] != 0.0) && (scratch.values[1340] != 0.0)) && (!(scratch.values[1610] != 0.0))) && (!(scratch.values[1612] != 0.0))) && (scratch.values[1613] != 0.0)) {
            scratch.values[1364] = 0.0;
            scratch.node_derivatives[1364] = [0.0; Instance::NODE_COUNT];
            scratch.branch_derivatives[1364] = [0.0; Instance::BRANCH_COUNT];
        }

        if (((((scratch.values[1323] != 0.0) && (scratch.values[1340] != 0.0)) && (!(scratch.values[1610] != 0.0))) && (!(scratch.values[1612] != 0.0))) && (!(scratch.values[1613] != 0.0))) {
            scratch.store_ad(1364, &AdValue::scale(AdValue::add(AdValue::div(AdValue::mul(AdValue::square(scratch.ad_value(1363)), AdValue::ln(scratch.ad_value(1363))), AdValue::sub_from_scalar(1.0, scratch.ad_value(1363))), scratch.ad_value(1363)), (1.0 - (2.0 * scratch.values[377]))));
        }

        if ((((scratch.values[1323] != 0.0) && (scratch.values[1340] != 0.0)) && (!(scratch.values[1610] != 0.0))) && (!(scratch.values[1612] != 0.0))) {
            scratch.store_ad(1365, &AdValue::add(scratch.ad_value(1363), scratch.ad_value(1364)));
        }

        scratch.values[1614] = if (scratch.values[377] == 0.5) { 1.0 } else { 0.0 };

        if (((((scratch.values[1323] != 0.0) && (scratch.values[1340] != 0.0)) && (!(scratch.values[1610] != 0.0))) && (!(scratch.values[1612] != 0.0))) && (scratch.values[1614] != 0.0)) {
            scratch.store_ad(1359, &AdValue::sqrt(AdValue::scale(scratch.ad_value(1362), scratch.values[480])));
        }

        if (((((scratch.values[1323] != 0.0) && (scratch.values[1340] != 0.0)) && (!(scratch.values[1610] != 0.0))) && (!(scratch.values[1612] != 0.0))) && (!(scratch.values[1614] != 0.0))) {
            scratch.store_ad(1359, &AdValue::powf(AdValue::scale(scratch.ad_value(1362), scratch.values[480]), scratch.values[377]));
        }

        if ((((scratch.values[1323] != 0.0) && (scratch.values[1340] != 0.0)) && (!(scratch.values[1610] != 0.0))) && (!(scratch.values[1612] != 0.0))) {
            scratch.store_ad(1366, &AdValue::scale(scratch.ad_value(1359), scratch.values[474]));
        }

        if ((((scratch.values[1323] != 0.0) && (scratch.values[1340] != 0.0)) && (!(scratch.values[1610] != 0.0))) && (!(scratch.values[1612] != 0.0))) {
            scratch.store_ad(1367, &AdValue::scale(AdValue::mul(AdValue::offset(scratch.ad_value(1353), (-1.0)), scratch.ad_value(1366)), scratch.values[435]));
        }

        if ((((scratch.values[1323] != 0.0) && (scratch.values[1340] != 0.0)) && (!(scratch.values[1610] != 0.0))) && (!(scratch.values[1612] != 0.0))) {
            scratch.store_ad(1361, &AdValue::scale(AdValue::mul(scratch.ad_value(1367), scratch.ad_value(1365)), scratch.values[388]));
        }

        scratch.values[1615] = if (scratch.values[391] == 0.0) { 1.0 } else { 0.0 };

        if ((((scratch.values[1323] != 0.0) && (scratch.values[1340] != 0.0)) && (!(scratch.values[1610] != 0.0))) && (scratch.values[1615] != 0.0)) {
            scratch.values[1368] = 0.0;
            scratch.node_derivatives[1368] = [0.0; Instance::NODE_COUNT];
            scratch.branch_derivatives[1368] = [0.0; Instance::BRANCH_COUNT];
        }

        if ((((scratch.values[1323] != 0.0) && (scratch.values[1340] != 0.0)) && (!(scratch.values[1610] != 0.0))) && (!(scratch.values[1615] != 0.0))) {
            scratch.store_ad(1369, &AdValue::scale(AdValue::div(AdValue::scale(scratch.ad_value(1366), scratch.values[459]), scratch.ad_value(1362)), scratch.values[489]));
        }

        if ((((scratch.values[1323] != 0.0) && (scratch.values[1340] != 0.0)) && (!(scratch.values[1610] != 0.0))) && (!(scratch.values[1615] != 0.0))) {
            scratch.store_ad(1370, &AdValue::div_from_scalar((0.666666666666667 * scratch.values[486]), scratch.ad_value(1369)));
        }

        if ((((scratch.values[1323] != 0.0) && (scratch.values[1340] != 0.0)) && (!(scratch.values[1610] != 0.0))) && (!(scratch.values[1615] != 0.0))) {
            scratch.store_ad(1371, &AdValue::square(scratch.ad_value(1370)));
        }

        if ((((scratch.values[1323] != 0.0) && (scratch.values[1340] != 0.0)) && (!(scratch.values[1610] != 0.0))) && (!(scratch.values[1615] != 0.0))) {
            scratch.store_ad(1372, &AdValue::sqrt(AdValue::div(AdValue::square(scratch.ad_value(1371)), AdValue::offset(AdValue::square(scratch.ad_value(1371)), 1.0))));
        }

        if ((((scratch.values[1323] != 0.0) && (scratch.values[1340] != 0.0)) && (!(scratch.values[1610] != 0.0))) && (!(scratch.values[1615] != 0.0))) {
            scratch.store_ad(1373, &AdValue::sqrt(AdValue::abs(scratch.ad_value(1372))));
        }

        if ((((scratch.values[1323] != 0.0) && (scratch.values[1340] != 0.0)) && (!(scratch.values[1610] != 0.0))) && (!(scratch.values[1615] != 0.0))) {
            scratch.store_ad(1374, &AdValue::mul(scratch.ad_value(1372), scratch.ad_value(1373)));
        }

        scratch.values[1616] = if (((-scratch.values[377]) * scratch.values[462]) == (-1.0)) { 1.0 } else { 0.0 };

        if (((((scratch.values[1323] != 0.0) && (scratch.values[1340] != 0.0)) && (!(scratch.values[1610] != 0.0))) && (!(scratch.values[1615] != 0.0))) && (scratch.values[1616] != 0.0)) {
            scratch.store_ad(1375, &AdValue::div_from_scalar(1.0, AdValue::offset(AdValue::mul(scratch.ad_value(1369), scratch.ad_value(1374)), 1.0)));
        }

        if (((((scratch.values[1323] != 0.0) && (scratch.values[1340] != 0.0)) && (!(scratch.values[1610] != 0.0))) && (!(scratch.values[1615] != 0.0))) && (!(scratch.values[1616] != 0.0))) {
            scratch.store_ad(1375, &AdValue::powf(AdValue::offset(AdValue::mul(scratch.ad_value(1369), scratch.ad_value(1374)), 1.0), ((-scratch.values[377]) * scratch.values[462])));
        }

        if ((((scratch.values[1323] != 0.0) && (scratch.values[1340] != 0.0)) && (!(scratch.values[1610] != 0.0))) && (!(scratch.values[1615] != 0.0))) {
            scratch.store_ad(1376, &AdValue::div(AdValue::mul(scratch.ad_value(1365), scratch.ad_value(1375)), AdValue::add(scratch.ad_value(1365), scratch.ad_value(1375))));
        }

        if ((((scratch.values[1323] != 0.0) && (scratch.values[1340] != 0.0)) && (!(scratch.values[1610] != 0.0))) && (!(scratch.values[1615] != 0.0))) {
            scratch.store_ad(1377, &AdValue::sqrt(AdValue::scale(AdValue::div(scratch.ad_value(1369), scratch.ad_value(1373)), 0.375)));
        }

        if ((((scratch.values[1323] != 0.0) && (scratch.values[1340] != 0.0)) && (!(scratch.values[1610] != 0.0))) && (!(scratch.values[1615] != 0.0))) {
            scratch.store_ad(1378, &AdValue::sub(AdValue::scale(AdValue::mul(scratch.ad_value(1370), scratch.ad_value(1373)), 2.0), scratch.ad_value(1372)));
        }

        if ((((scratch.values[1323] != 0.0) && (scratch.values[1340] != 0.0)) && (!(scratch.values[1610] != 0.0))) && (!(scratch.values[1615] != 0.0))) {
            scratch.store_ad(1379, &AdValue::add(AdValue::sub(AdValue::mul(AdValue::scale(scratch.ad_value(1370), scratch.values[486]), scratch.ad_value(1373)), AdValue::scale(scratch.ad_value(1372), scratch.values[486])), AdValue::scale(AdValue::mul(scratch.ad_value(1369), scratch.ad_value(1374)), 0.5)));
        }

        if ((((scratch.values[1323] != 0.0) && (scratch.values[1340] != 0.0)) && (!(scratch.values[1610] != 0.0))) && (!(scratch.values[1615] != 0.0))) {
            scratch.store_ad(1380, &AdValue::mul(AdValue::offset(scratch.ad_value(1378), (-1.0)), scratch.ad_value(1377)));
        }

        if ((((scratch.values[1323] != 0.0) && (scratch.values[1340] != 0.0)) && (!(scratch.values[1610] != 0.0))) && (!(scratch.values[1615] != 0.0))) {
            scratch.store_ad(1341, &AdValue::square(scratch.ad_value(1380)));
        }

        scratch.values[1617] = if (scratch.values[1380] > 0.0) { 1.0 } else { 0.0 };

        if (((((scratch.values[1323] != 0.0) && (scratch.values[1340] != 0.0)) && (!(scratch.values[1610] != 0.0))) && (!(scratch.values[1615] != 0.0))) && (scratch.values[1617] != 0.0)) {
            scratch.store_ad(1342, &AdValue::div_from_scalar(1.0, AdValue::offset(AdValue::scale(scratch.ad_value(1380), scratch.values[421]), 1.0)));
        }

        if (((((scratch.values[1323] != 0.0) && (scratch.values[1340] != 0.0)) && (!(scratch.values[1610] != 0.0))) && (!(scratch.values[1615] != 0.0))) && (!(scratch.values[1617] != 0.0))) {
            scratch.store_ad(1342, &AdValue::div_from_scalar(1.0, AdValue::sub_from_scalar(1.0, AdValue::scale(scratch.ad_value(1380), scratch.values[421]))));
        }

        scratch.values[1618] = if (((-scratch.values[1341]) + scratch.values[1379]) > (-230.25850929940458)) { 1.0 } else { 0.0 };

        if (((((scratch.values[1323] != 0.0) && (scratch.values[1340] != 0.0)) && (!(scratch.values[1610] != 0.0))) && (!(scratch.values[1615] != 0.0))) && (scratch.values[1618] != 0.0)) {
            scratch.store_ad(1359, &AdValue::exp(AdValue::sub(scratch.ad_value(1379), scratch.ad_value(1341))));
        }

        if (((((scratch.values[1323] != 0.0) && (scratch.values[1340] != 0.0)) && (!(scratch.values[1610] != 0.0))) && (!(scratch.values[1615] != 0.0))) && (!(scratch.values[1618] != 0.0))) {
            let assign23640_ad_e26182: AdValue = AdValue::div_from_scalar(1e-100, AdValue::offset(AdValue::mul(AdValue::sub_from_scalar((-230.25850929940458), AdValue::sub(scratch.ad_value(1379), scratch.ad_value(1341))), AdValue::offset(AdValue::scale(AdValue::mul(AdValue::sub_from_scalar((-230.25850929940458), AdValue::sub(scratch.ad_value(1379), scratch.ad_value(1341))), AdValue::offset(AdValue::scale(AdValue::sub_from_scalar((-230.25850929940458), AdValue::sub(scratch.ad_value(1379), scratch.ad_value(1341))), 0.3333333333333333), 1.0)), 0.5), 1.0)), 1.0));
            scratch.store_ad(1359, &assign23640_ad_e26182);
        }

        if ((((scratch.values[1323] != 0.0) && (scratch.values[1340] != 0.0)) && (!(scratch.values[1610] != 0.0))) && (!(scratch.values[1615] != 0.0))) {
            scratch.store_ad(1343, &AdValue::mul(AdValue::add(AdValue::add(AdValue::scale(scratch.ad_value(1342), 0.29214664), AdValue::scale(AdValue::square(scratch.ad_value(1342)), scratch.values[422])), AdValue::scale(AdValue::mul(AdValue::square(scratch.ad_value(1342)), scratch.ad_value(1342)), scratch.values[423])), scratch.ad_value(1359)));
        }

        scratch.values[1619] = if (scratch.values[1380] > 0.0) { 1.0 } else { 0.0 };

        if (((((scratch.values[1323] != 0.0) && (scratch.values[1340] != 0.0)) && (!(scratch.values[1610] != 0.0))) && (!(scratch.values[1615] != 0.0))) && (scratch.values[1619] != 0.0)) {
            scratch.values[1381] = scratch.values[1343];
            scratch.node_derivatives[1381] = scratch.node_derivatives[1343];
            scratch.branch_derivatives[1381] = scratch.branch_derivatives[1343];
        }

        scratch.values[1620] = if (scratch.values[1379] > (-230.25850929940458)) { 1.0 } else { 0.0 };

        if ((((((scratch.values[1323] != 0.0) && (scratch.values[1340] != 0.0)) && (!(scratch.values[1610] != 0.0))) && (!(scratch.values[1615] != 0.0))) && (!(scratch.values[1619] != 0.0))) && (scratch.values[1620] != 0.0)) {
            scratch.store_ad(1359, &AdValue::exp(scratch.ad_value(1379)));
        }

        if ((((((scratch.values[1323] != 0.0) && (scratch.values[1340] != 0.0)) && (!(scratch.values[1610] != 0.0))) && (!(scratch.values[1615] != 0.0))) && (!(scratch.values[1619] != 0.0))) && (!(scratch.values[1620] != 0.0))) {
            scratch.store_ad(1359, &AdValue::div_from_scalar(1e-100, AdValue::offset(AdValue::mul(AdValue::sub_from_scalar((-230.25850929940458), scratch.ad_value(1379)), AdValue::offset(AdValue::scale(AdValue::mul(AdValue::sub_from_scalar((-230.25850929940458), scratch.ad_value(1379)), AdValue::offset(AdValue::scale(AdValue::sub_from_scalar((-230.25850929940458), scratch.ad_value(1379)), 0.3333333333333333), 1.0)), 0.5), 1.0)), 1.0)));
        }

        if (((((scratch.values[1323] != 0.0) && (scratch.values[1340] != 0.0)) && (!(scratch.values[1610] != 0.0))) && (!(scratch.values[1615] != 0.0))) && (!(scratch.values[1619] != 0.0))) {
            scratch.store_ad(1381, &AdValue::sub(AdValue::scale(scratch.ad_value(1359), 2.0), scratch.ad_value(1343)));
        }

        if ((((scratch.values[1323] != 0.0) && (scratch.values[1340] != 0.0)) && (!(scratch.values[1610] != 0.0))) && (!(scratch.values[1615] != 0.0))) {
            scratch.store_ad(1382, &AdValue::scale(AdValue::div(AdValue::scale(scratch.ad_value(1381), scratch.values[486]), scratch.ad_value(1377)), (1.772453850905516 * 0.5)));
        }

        if ((((scratch.values[1323] != 0.0) && (scratch.values[1340] != 0.0)) && (!(scratch.values[1610] != 0.0))) && (!(scratch.values[1615] != 0.0))) {
            scratch.store_ad(1368, &AdValue::scale(AdValue::mul(AdValue::mul(scratch.ad_value(1367), scratch.ad_value(1382)), scratch.ad_value(1376)), scratch.values[391]));
        }

        scratch.values[1621] = if (scratch.values[397] == 0.0) { 1.0 } else { 0.0 };

        if ((((scratch.values[1323] != 0.0) && (scratch.values[1340] != 0.0)) && (!(scratch.values[1610] != 0.0))) && (scratch.values[1621] != 0.0)) {
            scratch.values[1383] = 0.0;
            scratch.node_derivatives[1383] = [0.0; Instance::NODE_COUNT];
            scratch.branch_derivatives[1383] = [0.0; Instance::BRANCH_COUNT];
        }

        scratch.values[1622] = if (scratch.values[377] == 0.5) { 1.0 } else { 0.0 };

        if (((((scratch.values[1323] != 0.0) && (scratch.values[1340] != 0.0)) && (!(scratch.values[1610] != 0.0))) && (!(scratch.values[1621] != 0.0))) && (scratch.values[1622] != 0.0)) {
            scratch.store_ad(1359, &AdValue::sqrt(AdValue::scale(AdValue::sub_from_scalar(scratch.values[374], scratch.ad_value(1357)), scratch.values[480])));
        }

        if (((((scratch.values[1323] != 0.0) && (scratch.values[1340] != 0.0)) && (!(scratch.values[1610] != 0.0))) && (!(scratch.values[1621] != 0.0))) && (!(scratch.values[1622] != 0.0))) {
            scratch.store_ad(1359, &AdValue::powf(AdValue::scale(AdValue::sub_from_scalar(scratch.values[374], scratch.ad_value(1357)), scratch.values[480]), scratch.values[377]));
        }

        if ((((scratch.values[1323] != 0.0) && (scratch.values[1340] != 0.0)) && (!(scratch.values[1610] != 0.0))) && (!(scratch.values[1621] != 0.0))) {
            scratch.store_ad(1384, &AdValue::scale(AdValue::div(AdValue::scale(AdValue::sub_from_scalar(scratch.values[374], scratch.ad_value(1357)), scratch.values[477]), scratch.ad_value(1359)), scratch.values[462]));
        }

        scratch.values[1623] = if (((((-scratch.values[492]) / scratch.values[1384])) as f64).abs() < 230.25850929940458) { 1.0 } else { 0.0 };

        if (((((scratch.values[1323] != 0.0) && (scratch.values[1340] != 0.0)) && (!(scratch.values[1610] != 0.0))) && (!(scratch.values[1621] != 0.0))) && (scratch.values[1623] != 0.0)) {
            scratch.store_ad(1359, &AdValue::exp(AdValue::div(AdValue::neg(scratch.ad_value(492)), scratch.ad_value(1384))));
        }

        scratch.values[1624] = if (((-scratch.values[492]) / scratch.values[1384]) < 0.0) { 1.0 } else { 0.0 };

        if ((((((scratch.values[1323] != 0.0) && (scratch.values[1340] != 0.0)) && (!(scratch.values[1610] != 0.0))) && (!(scratch.values[1621] != 0.0))) && (!(scratch.values[1623] != 0.0))) && (scratch.values[1624] != 0.0)) {
            let assign23830_ad_e26509: AdValue = AdValue::offset(AdValue::mul(AdValue::sub_from_scalar((-230.25850929940458), AdValue::div(AdValue::neg(scratch.ad_value(492)), scratch.ad_value(1384))), AdValue::offset(AdValue::scale(AdValue::mul(AdValue::sub_from_scalar((-230.25850929940458), AdValue::div(AdValue::neg(scratch.ad_value(492)), scratch.ad_value(1384))), AdValue::offset(AdValue::scale(AdValue::sub_from_scalar((-230.25850929940458), AdValue::div(AdValue::neg(scratch.ad_value(492)), scratch.ad_value(1384))), 0.3333333333333333), 1.0)), 0.5), 1.0)), 1.0);
            scratch.store_ad(1359, &AdValue::div_from_scalar(1e-100, assign23830_ad_e26509));
        }

        if ((((((scratch.values[1323] != 0.0) && (scratch.values[1340] != 0.0)) && (!(scratch.values[1610] != 0.0))) && (!(scratch.values[1621] != 0.0))) && (!(scratch.values[1623] != 0.0))) && (!(scratch.values[1624] != 0.0))) {
            let assign23840_ad_e26559: AdValue = AdValue::scale(AdValue::offset(AdValue::mul(AdValue::offset(AdValue::div(AdValue::neg(scratch.ad_value(492)), scratch.ad_value(1384)), (-230.25850929940458)), AdValue::offset(AdValue::scale(AdValue::mul(AdValue::offset(AdValue::div(AdValue::neg(scratch.ad_value(492)), scratch.ad_value(1384)), (-230.25850929940458)), AdValue::offset(AdValue::scale(AdValue::offset(AdValue::div(AdValue::neg(scratch.ad_value(492)), scratch.ad_value(1384)), (-230.25850929940458)), 0.3333333333333333), 1.0)), 0.5), 1.0)), 1.0), 1e100);
            scratch.store_ad(1359, &assign23840_ad_e26559);
        }

        if ((((scratch.values[1323] != 0.0) && (scratch.values[1340] != 0.0)) && (!(scratch.values[1610] != 0.0))) && (!(scratch.values[1621] != 0.0))) {
            scratch.store_ad(1383, &AdValue::scale(AdValue::mul(AdValue::mul(AdValue::mul(scratch.ad_value(519), scratch.ad_value(1384)), scratch.ad_value(1384)), scratch.ad_value(1359)), scratch.values[397]));
        }

        scratch.values[1625] = if (scratch.values[406] > 1000.0) { 1.0 } else { 0.0 };

        if ((((scratch.values[1323] != 0.0) && (scratch.values[1340] != 0.0)) && (!(scratch.values[1610] != 0.0))) && (scratch.values[1625] != 0.0)) {
            scratch.values[1385] = 1.0;
            scratch.node_derivatives[1385] = [0.0; Instance::NODE_COUNT];
            scratch.branch_derivatives[1385] = [0.0; Instance::BRANCH_COUNT];
        }

        scratch.values[1626] = if (scratch.values[1358] > ((-scratch.values[493]) * scratch.values[406])) { 1.0 } else { 0.0 };

        scratch.values[1627] = if (scratch.values[409] == 4.0) { 1.0 } else { 0.0 };

        if ((((((scratch.values[1323] != 0.0) && (scratch.values[1340] != 0.0)) && (!(scratch.values[1610] != 0.0))) && (!(scratch.values[1625] != 0.0))) && (scratch.values[1626] != 0.0)) && (scratch.values[1627] != 0.0)) {
            scratch.store_ad(1359, &AdValue::mul(AdValue::mul(AdValue::mul(AdValue::scale(scratch.ad_value(1358), scratch.values[499]), AdValue::scale(scratch.ad_value(1358), scratch.values[499])), AdValue::scale(scratch.ad_value(1358), scratch.values[499])), AdValue::scale(scratch.ad_value(1358), scratch.values[499])));
        }

        if ((((((scratch.values[1323] != 0.0) && (scratch.values[1340] != 0.0)) && (!(scratch.values[1610] != 0.0))) && (!(scratch.values[1625] != 0.0))) && (scratch.values[1626] != 0.0)) && (!(scratch.values[1627] != 0.0))) {
            scratch.store_ad(1359, &AdValue::powf(AdValue::abs(AdValue::scale(scratch.ad_value(1358), scratch.values[499])), scratch.values[409]));
        }

        if (((((scratch.values[1323] != 0.0) && (scratch.values[1340] != 0.0)) && (!(scratch.values[1610] != 0.0))) && (!(scratch.values[1625] != 0.0))) && (scratch.values[1626] != 0.0)) {
            scratch.store_ad(1385, &AdValue::div_from_scalar(1.0, AdValue::sub_from_scalar(1.0, scratch.ad_value(1359))));
        }

        if (((((scratch.values[1323] != 0.0) && (scratch.values[1340] != 0.0)) && (!(scratch.values[1610] != 0.0))) && (!(scratch.values[1625] != 0.0))) && (!(scratch.values[1626] != 0.0))) {
            scratch.store_ad(1385, &AdValue::offset(AdValue::scale(AdValue::offset(scratch.ad_value(1358), (scratch.values[493] * scratch.values[406])), scratch.values[502]), scratch.values[496]));
        }

        if (((scratch.values[1323] != 0.0) && (scratch.values[1340] != 0.0)) && (!(scratch.values[1610] != 0.0))) {
            scratch.store_ad(1388, &AdValue::mul(AdValue::add(AdValue::add(AdValue::add(scratch.ad_value(1360), scratch.ad_value(1361)), scratch.ad_value(1368)), scratch.ad_value(1383)), scratch.ad_value(1385)));
        }

        if ((scratch.values[1323] != 0.0) && (scratch.values[1340] != 0.0)) {
            scratch.store_ad(509, &AdValue::add(AdValue::add(AdValue::mul(scratch.ad_value(690), scratch.ad_value(1386)), AdValue::mul(scratch.ad_value(691), scratch.ad_value(1387))), AdValue::mul(scratch.ad_value(692), scratch.ad_value(1388))));
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

        scratch.values[1628] = if !(((scratch.values[690] == 0.0) && (scratch.values[691] == 0.0)) && (scratch.values[692] == 0.0)) { 1.0 } else { 0.0 };

        if (((scratch.values[1323] != 0.0) && (scratch.values[1340] != 0.0)) && (scratch.values[1628] != 0.0)) {
            scratch.store_ad(1344, &AdValue::mul(AdValue::scale(scratch.ad_value(701), 4.0), scratch.ad_value(701)));
        }

        if (((scratch.values[1323] != 0.0) && (scratch.values[1340] != 0.0)) && (scratch.values[1628] != 0.0)) {
            scratch.store_ad(1345, &AdValue::div(scratch.ad_value(701), scratch.ad_value(702)));
        }

        if (((scratch.values[1323] != 0.0) && (scratch.values[1340] != 0.0)) && (scratch.values[1628] != 0.0)) {
            scratch.store_ad(1346, &AdValue::add(scratch.ad_value(520), AdValue::mul(scratch.ad_value(701), scratch.ad_value(1345))));
        }

        if (((scratch.values[1323] != 0.0) && (scratch.values[1340] != 0.0)) && (scratch.values[1628] != 0.0)) {
            scratch.store_ad(1347, &AdValue::add(scratch.ad_value(702), scratch.ad_value(1346)));
        }

        if (((scratch.values[1323] != 0.0) && (scratch.values[1340] != 0.0)) && (scratch.values[1628] != 0.0)) {
            scratch.store_ad(1348, &AdValue::sub(scratch.ad_value(702), scratch.ad_value(1346)));
        }

        if (((scratch.values[1323] != 0.0) && (scratch.values[1340] != 0.0)) && (scratch.values[1628] != 0.0)) {
            scratch.store_ad(1349, &AdValue::sqrt(AdValue::add(AdValue::square(scratch.ad_value(1348)), scratch.ad_value(1344))));
        }

        if (((scratch.values[1323] != 0.0) && (scratch.values[1340] != 0.0)) && (scratch.values[1628] != 0.0)) {
            scratch.store_ad(1351, &AdValue::scale(AdValue::div(AdValue::mul(scratch.ad_value(520), scratch.ad_value(702)), AdValue::add(scratch.ad_value(1347), scratch.ad_value(1349))), 2.0));
        }

        scratch.values[1629] = if (scratch.values[520] < scratch.values[698]) { 1.0 } else { 0.0 };

        scratch.values[1630] = if ((((0.5 * (scratch.values[520] * scratch.values[420]))) as f64).abs() < 230.25850929940458) { 1.0 } else { 0.0 };

        if (((((scratch.values[1323] != 0.0) && (scratch.values[1340] != 0.0)) && (scratch.values[1628] != 0.0)) && (scratch.values[1629] != 0.0)) && (scratch.values[1630] != 0.0)) {
            scratch.store_ad(1353, &AdValue::exp(AdValue::scale(scratch.ad_value(520), (scratch.values[420] * 0.5))));
        }

        scratch.values[1631] = if ((0.5 * (scratch.values[520] * scratch.values[420])) < 0.0) { 1.0 } else { 0.0 };

        if ((((((scratch.values[1323] != 0.0) && (scratch.values[1340] != 0.0)) && (scratch.values[1628] != 0.0)) && (scratch.values[1629] != 0.0)) && (!(scratch.values[1630] != 0.0))) && (scratch.values[1631] != 0.0)) {
            let assign24100_ad_e26922: AdValue = AdValue::div_from_scalar(1e-100, AdValue::offset(AdValue::mul(AdValue::sub_from_scalar((-230.25850929940458), AdValue::scale(scratch.ad_value(520), (scratch.values[420] * 0.5))), AdValue::offset(AdValue::scale(AdValue::mul(AdValue::sub_from_scalar((-230.25850929940458), AdValue::scale(scratch.ad_value(520), (scratch.values[420] * 0.5))), AdValue::offset(AdValue::scale(AdValue::sub_from_scalar((-230.25850929940458), AdValue::scale(scratch.ad_value(520), (scratch.values[420] * 0.5))), 0.3333333333333333), 1.0)), 0.5), 1.0)), 1.0));
            scratch.store_ad(1353, &assign24100_ad_e26922);
        }

        if ((((((scratch.values[1323] != 0.0) && (scratch.values[1340] != 0.0)) && (scratch.values[1628] != 0.0)) && (scratch.values[1629] != 0.0)) && (!(scratch.values[1630] != 0.0))) && (!(scratch.values[1631] != 0.0))) {
            scratch.store_ad(1353, &AdValue::scale(AdValue::offset(AdValue::mul(AdValue::offset(AdValue::scale(scratch.ad_value(520), (scratch.values[420] * 0.5)), (-230.25850929940458)), AdValue::offset(AdValue::scale(AdValue::mul(AdValue::offset(AdValue::scale(scratch.ad_value(520), (scratch.values[420] * 0.5)), (-230.25850929940458)), AdValue::offset(AdValue::scale(AdValue::offset(AdValue::scale(scratch.ad_value(520), (scratch.values[420] * 0.5)), (-230.25850929940458)), 0.3333333333333333), 1.0)), 0.5), 1.0)), 1.0), 1e100));
        }

        if ((((scratch.values[1323] != 0.0) && (scratch.values[1340] != 0.0)) && (scratch.values[1628] != 0.0)) && (scratch.values[1629] != 0.0)) {
            scratch.store_ad(1350, &AdValue::square(scratch.ad_value(1353)));
        }

        if ((((scratch.values[1323] != 0.0) && (scratch.values[1340] != 0.0)) && (scratch.values[1628] != 0.0)) && (!(scratch.values[1629] != 0.0))) {
            scratch.store_ad(1350, &AdValue::mul(AdValue::offset(AdValue::scale(AdValue::sub(scratch.ad_value(520), scratch.ad_value(698)), scratch.values[420]), 1.0), scratch.ad_value(699)));
        }

        if ((((scratch.values[1323] != 0.0) && (scratch.values[1340] != 0.0)) && (scratch.values[1628] != 0.0)) && (!(scratch.values[1629] != 0.0))) {
            scratch.store_ad(1353, &AdValue::sqrt(scratch.ad_value(1350)));
        }

        if (((scratch.values[1323] != 0.0) && (scratch.values[1340] != 0.0)) && (scratch.values[1628] != 0.0)) {
            scratch.store_ad(1350, &AdValue::offset(scratch.ad_value(1350), (-1.0)));
        }

        if (((scratch.values[1323] != 0.0) && (scratch.values[1340] != 0.0)) && (scratch.values[1628] != 0.0)) {
            scratch.store_ad(1352, &AdValue::div_from_scalar(1.0, scratch.ad_value(1353)));
        }

        scratch.values[1632] = if (scratch.values[520] > 0.0) { 1.0 } else { 0.0 };

        if ((((scratch.values[1323] != 0.0) && (scratch.values[1340] != 0.0)) && (scratch.values[1628] != 0.0)) && (scratch.values[1632] != 0.0)) {
            scratch.store_ad(1354, &AdValue::scale(AdValue::ln(AdValue::add(AdValue::offset(scratch.ad_value(1352), 2.0), AdValue::sqrt(AdValue::mul(AdValue::offset(scratch.ad_value(1352), 1.0), AdValue::offset(scratch.ad_value(1352), 3.0))))), (scratch.values[419] * 2.0)));
        }

        if ((((scratch.values[1323] != 0.0) && (scratch.values[1340] != 0.0)) && (scratch.values[1628] != 0.0)) && (!(scratch.values[1632] != 0.0))) {
            scratch.store_ad(1354, &AdValue::sub(AdValue::scale(AdValue::ln(AdValue::add(AdValue::offset(AdValue::scale(scratch.ad_value(1353), 2.0), 1.0), AdValue::sqrt(AdValue::mul(AdValue::offset(scratch.ad_value(1353), 1.0), AdValue::offset(AdValue::scale(scratch.ad_value(1353), 3.0), 1.0))))), (scratch.values[419] * 2.0)), scratch.ad_value(520)));
        }

        if (((scratch.values[1323] != 0.0) && (scratch.values[1340] != 0.0)) && (scratch.values[1628] != 0.0)) {
            scratch.store_ad(1355, &AdValue::sub(scratch.ad_value(700), scratch.ad_value(1354)));
        }

        if (((scratch.values[1323] != 0.0) && (scratch.values[1340] != 0.0)) && (scratch.values[1628] != 0.0)) {
            scratch.store_ad(1356, &AdValue::scale(AdValue::sub(AdValue::add(scratch.ad_value(520), scratch.ad_value(1355)), AdValue::sqrt(AdValue::offset(AdValue::mul(AdValue::sub(scratch.ad_value(520), scratch.ad_value(1355)), AdValue::sub(scratch.ad_value(520), scratch.ad_value(1355))), ((4.0 * scratch.values[419]) * scratch.values[419])))), 0.5));
        }

        if (((scratch.values[1323] != 0.0) && (scratch.values[1340] != 0.0)) && (scratch.values[1628] != 0.0)) {
            scratch.store_ad(1357, &AdValue::scale(AdValue::sub(AdValue::add(scratch.ad_value(520), scratch.ad_value(703)), AdValue::sqrt(AdValue::offset(AdValue::mul(AdValue::sub(scratch.ad_value(520), scratch.ad_value(703)), AdValue::sub(scratch.ad_value(520), scratch.ad_value(703))), ((4.0 * scratch.values[417]) * scratch.values[417])))), 0.5));
        }

        if (((scratch.values[1323] != 0.0) && (scratch.values[1340] != 0.0)) && (scratch.values[1628] != 0.0)) {
            scratch.store_ad(1358, &AdValue::scale(AdValue::sub(scratch.ad_value(520), AdValue::sqrt(AdValue::offset(AdValue::mul(scratch.ad_value(520), scratch.ad_value(520)), ((4.0 * 1e-6) * 1e-6)))), 0.5));
        }

        scratch.values[1633] = if (scratch.values[690] == 0.0) { 1.0 } else { 0.0 };

        if (((scratch.values[1323] != 0.0) && (scratch.values[1340] != 0.0)) && (scratch.values[1633] != 0.0)) {
            scratch.values[1386] = 0.0;
            scratch.node_derivatives[1386] = [0.0; Instance::NODE_COUNT];
            scratch.branch_derivatives[1386] = [0.0; Instance::BRANCH_COUNT];
        }

        scratch.values[1634] = if (scratch.values[457] == 0.5) { 1.0 } else { 0.0 };

        if ((((scratch.values[1323] != 0.0) && (scratch.values[1340] != 0.0)) && (!(scratch.values[1633] != 0.0))) && (scratch.values[1634] != 0.0)) {
            scratch.store_ad(1359, &AdValue::sqrt(AdValue::sub_from_scalar(1.0, AdValue::scale(scratch.ad_value(1351), scratch.values[454]))));
        }

        if ((((scratch.values[1323] != 0.0) && (scratch.values[1340] != 0.0)) && (!(scratch.values[1633] != 0.0))) && (!(scratch.values[1634] != 0.0))) {
            scratch.store_ad(1359, &AdValue::powf(AdValue::sub_from_scalar(1.0, AdValue::scale(scratch.ad_value(1351), scratch.values[454])), scratch.values[457]));
        }

        if (((scratch.values[1323] != 0.0) && (scratch.values[1340] != 0.0)) && (!(scratch.values[1633] != 0.0))) {
            scratch.store_ad(1360, &AdValue::scale(scratch.ad_value(1350), scratch.values[436]));
        }

        scratch.values[1635] = if ((scratch.values[386] == 0.0) && (scratch.values[389] == 0.0)) { 1.0 } else { 0.0 };

        if ((((scratch.values[1323] != 0.0) && (scratch.values[1340] != 0.0)) && (!(scratch.values[1633] != 0.0))) && (scratch.values[1635] != 0.0)) {
            scratch.values[1361] = 0.0;
            scratch.node_derivatives[1361] = [0.0; Instance::NODE_COUNT];
            scratch.branch_derivatives[1361] = [0.0; Instance::BRANCH_COUNT];
        }

        if ((((scratch.values[1323] != 0.0) && (scratch.values[1340] != 0.0)) && (!(scratch.values[1633] != 0.0))) && (!(scratch.values[1635] != 0.0))) {
            scratch.store_ad(1362, &AdValue::sub_from_scalar(scratch.values[442], scratch.ad_value(1356)));
        }

        if ((((scratch.values[1323] != 0.0) && (scratch.values[1340] != 0.0)) && (!(scratch.values[1633] != 0.0))) && (!(scratch.values[1635] != 0.0))) {
            scratch.store_ad(1363, &AdValue::sub_from_scalar(1.0, AdValue::sqrt(AdValue::sub_from_scalar(1.0, AdValue::div(scratch.ad_value(1354), scratch.ad_value(1362))))));
        }

        scratch.values[1636] = if (scratch.values[375] == 0.5) { 1.0 } else { 0.0 };

        if (((((scratch.values[1323] != 0.0) && (scratch.values[1340] != 0.0)) && (!(scratch.values[1633] != 0.0))) && (!(scratch.values[1635] != 0.0))) && (scratch.values[1636] != 0.0)) {
            scratch.values[1364] = 0.0;
            scratch.node_derivatives[1364] = [0.0; Instance::NODE_COUNT];
            scratch.branch_derivatives[1364] = [0.0; Instance::BRANCH_COUNT];
        }

        if (((((scratch.values[1323] != 0.0) && (scratch.values[1340] != 0.0)) && (!(scratch.values[1633] != 0.0))) && (!(scratch.values[1635] != 0.0))) && (!(scratch.values[1636] != 0.0))) {
            scratch.store_ad(1364, &AdValue::scale(AdValue::add(AdValue::div(AdValue::mul(AdValue::square(scratch.ad_value(1363)), AdValue::ln(scratch.ad_value(1363))), AdValue::sub_from_scalar(1.0, scratch.ad_value(1363))), scratch.ad_value(1363)), (1.0 - (2.0 * scratch.values[375]))));
        }

        if ((((scratch.values[1323] != 0.0) && (scratch.values[1340] != 0.0)) && (!(scratch.values[1633] != 0.0))) && (!(scratch.values[1635] != 0.0))) {
            scratch.store_ad(1365, &AdValue::add(scratch.ad_value(1363), scratch.ad_value(1364)));
        }

        scratch.values[1637] = if (scratch.values[375] == 0.5) { 1.0 } else { 0.0 };

        if (((((scratch.values[1323] != 0.0) && (scratch.values[1340] != 0.0)) && (!(scratch.values[1633] != 0.0))) && (!(scratch.values[1635] != 0.0))) && (scratch.values[1637] != 0.0)) {
            scratch.store_ad(1359, &AdValue::sqrt(AdValue::scale(scratch.ad_value(1362), scratch.values[478])));
        }

        if (((((scratch.values[1323] != 0.0) && (scratch.values[1340] != 0.0)) && (!(scratch.values[1633] != 0.0))) && (!(scratch.values[1635] != 0.0))) && (!(scratch.values[1637] != 0.0))) {
            scratch.store_ad(1359, &AdValue::powf(AdValue::scale(scratch.ad_value(1362), scratch.values[478]), scratch.values[375]));
        }

        if ((((scratch.values[1323] != 0.0) && (scratch.values[1340] != 0.0)) && (!(scratch.values[1633] != 0.0))) && (!(scratch.values[1635] != 0.0))) {
            scratch.store_ad(1366, &AdValue::scale(scratch.ad_value(1359), scratch.values[472]));
        }

        if ((((scratch.values[1323] != 0.0) && (scratch.values[1340] != 0.0)) && (!(scratch.values[1633] != 0.0))) && (!(scratch.values[1635] != 0.0))) {
            scratch.store_ad(1367, &AdValue::scale(AdValue::mul(AdValue::offset(scratch.ad_value(1353), (-1.0)), scratch.ad_value(1366)), scratch.values[433]));
        }

        if ((((scratch.values[1323] != 0.0) && (scratch.values[1340] != 0.0)) && (!(scratch.values[1633] != 0.0))) && (!(scratch.values[1635] != 0.0))) {
            scratch.store_ad(1361, &AdValue::scale(AdValue::mul(scratch.ad_value(1367), scratch.ad_value(1365)), scratch.values[386]));
        }

        scratch.values[1638] = if (scratch.values[389] == 0.0) { 1.0 } else { 0.0 };

        if ((((scratch.values[1323] != 0.0) && (scratch.values[1340] != 0.0)) && (!(scratch.values[1633] != 0.0))) && (scratch.values[1638] != 0.0)) {
            scratch.values[1368] = 0.0;
            scratch.node_derivatives[1368] = [0.0; Instance::NODE_COUNT];
            scratch.branch_derivatives[1368] = [0.0; Instance::BRANCH_COUNT];
        }

        if ((((scratch.values[1323] != 0.0) && (scratch.values[1340] != 0.0)) && (!(scratch.values[1633] != 0.0))) && (!(scratch.values[1638] != 0.0))) {
            scratch.store_ad(1369, &AdValue::scale(AdValue::div(AdValue::scale(scratch.ad_value(1366), scratch.values[457]), scratch.ad_value(1362)), scratch.values[487]));
        }

        if ((((scratch.values[1323] != 0.0) && (scratch.values[1340] != 0.0)) && (!(scratch.values[1633] != 0.0))) && (!(scratch.values[1638] != 0.0))) {
            scratch.store_ad(1370, &AdValue::div_from_scalar((0.666666666666667 * scratch.values[484]), scratch.ad_value(1369)));
        }

        if ((((scratch.values[1323] != 0.0) && (scratch.values[1340] != 0.0)) && (!(scratch.values[1633] != 0.0))) && (!(scratch.values[1638] != 0.0))) {
            scratch.store_ad(1371, &AdValue::square(scratch.ad_value(1370)));
        }

        if ((((scratch.values[1323] != 0.0) && (scratch.values[1340] != 0.0)) && (!(scratch.values[1633] != 0.0))) && (!(scratch.values[1638] != 0.0))) {
            scratch.store_ad(1372, &AdValue::sqrt(AdValue::div(AdValue::square(scratch.ad_value(1371)), AdValue::offset(AdValue::square(scratch.ad_value(1371)), 1.0))));
        }

        if ((((scratch.values[1323] != 0.0) && (scratch.values[1340] != 0.0)) && (!(scratch.values[1633] != 0.0))) && (!(scratch.values[1638] != 0.0))) {
            scratch.store_ad(1373, &AdValue::sqrt(AdValue::abs(scratch.ad_value(1372))));
        }

        if ((((scratch.values[1323] != 0.0) && (scratch.values[1340] != 0.0)) && (!(scratch.values[1633] != 0.0))) && (!(scratch.values[1638] != 0.0))) {
            scratch.store_ad(1374, &AdValue::mul(scratch.ad_value(1372), scratch.ad_value(1373)));
        }

        scratch.values[1639] = if (((-scratch.values[375]) * scratch.values[460]) == (-1.0)) { 1.0 } else { 0.0 };

        if (((((scratch.values[1323] != 0.0) && (scratch.values[1340] != 0.0)) && (!(scratch.values[1633] != 0.0))) && (!(scratch.values[1638] != 0.0))) && (scratch.values[1639] != 0.0)) {
            scratch.store_ad(1375, &AdValue::div_from_scalar(1.0, AdValue::offset(AdValue::mul(scratch.ad_value(1369), scratch.ad_value(1374)), 1.0)));
        }

        if (((((scratch.values[1323] != 0.0) && (scratch.values[1340] != 0.0)) && (!(scratch.values[1633] != 0.0))) && (!(scratch.values[1638] != 0.0))) && (!(scratch.values[1639] != 0.0))) {
            scratch.store_ad(1375, &AdValue::powf(AdValue::offset(AdValue::mul(scratch.ad_value(1369), scratch.ad_value(1374)), 1.0), ((-scratch.values[375]) * scratch.values[460])));
        }

        if ((((scratch.values[1323] != 0.0) && (scratch.values[1340] != 0.0)) && (!(scratch.values[1633] != 0.0))) && (!(scratch.values[1638] != 0.0))) {
            scratch.store_ad(1376, &AdValue::div(AdValue::mul(scratch.ad_value(1365), scratch.ad_value(1375)), AdValue::add(scratch.ad_value(1365), scratch.ad_value(1375))));
        }

        if ((((scratch.values[1323] != 0.0) && (scratch.values[1340] != 0.0)) && (!(scratch.values[1633] != 0.0))) && (!(scratch.values[1638] != 0.0))) {
            scratch.store_ad(1377, &AdValue::sqrt(AdValue::scale(AdValue::div(scratch.ad_value(1369), scratch.ad_value(1373)), 0.375)));
        }

        if ((((scratch.values[1323] != 0.0) && (scratch.values[1340] != 0.0)) && (!(scratch.values[1633] != 0.0))) && (!(scratch.values[1638] != 0.0))) {
            scratch.store_ad(1378, &AdValue::sub(AdValue::scale(AdValue::mul(scratch.ad_value(1370), scratch.ad_value(1373)), 2.0), scratch.ad_value(1372)));
        }

        if ((((scratch.values[1323] != 0.0) && (scratch.values[1340] != 0.0)) && (!(scratch.values[1633] != 0.0))) && (!(scratch.values[1638] != 0.0))) {
            scratch.store_ad(1379, &AdValue::add(AdValue::sub(AdValue::mul(AdValue::scale(scratch.ad_value(1370), scratch.values[484]), scratch.ad_value(1373)), AdValue::scale(scratch.ad_value(1372), scratch.values[484])), AdValue::scale(AdValue::mul(scratch.ad_value(1369), scratch.ad_value(1374)), 0.5)));
        }

        if ((((scratch.values[1323] != 0.0) && (scratch.values[1340] != 0.0)) && (!(scratch.values[1633] != 0.0))) && (!(scratch.values[1638] != 0.0))) {
            scratch.store_ad(1380, &AdValue::mul(AdValue::offset(scratch.ad_value(1378), (-1.0)), scratch.ad_value(1377)));
        }

        if ((((scratch.values[1323] != 0.0) && (scratch.values[1340] != 0.0)) && (!(scratch.values[1633] != 0.0))) && (!(scratch.values[1638] != 0.0))) {
            scratch.store_ad(1341, &AdValue::square(scratch.ad_value(1380)));
        }

    }

    pub(super) fn stamp_transient_block_19(
        &mut self,
        ctx: &GeneratedEvalContext<'_>,
        stamper: &mut GeneratedStamper<'_>,
        scratch: &mut Scratch,
    ) {
        let _ = stamper;
        scratch.values[1640] = if (scratch.values[1380] > 0.0) { 1.0 } else { 0.0 };

        if (((((scratch.values[1323] != 0.0) && (scratch.values[1340] != 0.0)) && (!(scratch.values[1633] != 0.0))) && (!(scratch.values[1638] != 0.0))) && (scratch.values[1640] != 0.0)) {
            scratch.store_ad(1342, &AdValue::div_from_scalar(1.0, AdValue::offset(AdValue::scale(scratch.ad_value(1380), scratch.values[421]), 1.0)));
        }

        if (((((scratch.values[1323] != 0.0) && (scratch.values[1340] != 0.0)) && (!(scratch.values[1633] != 0.0))) && (!(scratch.values[1638] != 0.0))) && (!(scratch.values[1640] != 0.0))) {
            scratch.store_ad(1342, &AdValue::div_from_scalar(1.0, AdValue::sub_from_scalar(1.0, AdValue::scale(scratch.ad_value(1380), scratch.values[421]))));
        }

        scratch.values[1641] = if (((-scratch.values[1341]) + scratch.values[1379]) > (-230.25850929940458)) { 1.0 } else { 0.0 };

        if (((((scratch.values[1323] != 0.0) && (scratch.values[1340] != 0.0)) && (!(scratch.values[1633] != 0.0))) && (!(scratch.values[1638] != 0.0))) && (scratch.values[1641] != 0.0)) {
            scratch.store_ad(1359, &AdValue::exp(AdValue::sub(scratch.ad_value(1379), scratch.ad_value(1341))));
        }

        if (((((scratch.values[1323] != 0.0) && (scratch.values[1340] != 0.0)) && (!(scratch.values[1633] != 0.0))) && (!(scratch.values[1638] != 0.0))) && (!(scratch.values[1641] != 0.0))) {
            let assign24680_ad_e27865: AdValue = AdValue::div_from_scalar(1e-100, AdValue::offset(AdValue::mul(AdValue::sub_from_scalar((-230.25850929940458), AdValue::sub(scratch.ad_value(1379), scratch.ad_value(1341))), AdValue::offset(AdValue::scale(AdValue::mul(AdValue::sub_from_scalar((-230.25850929940458), AdValue::sub(scratch.ad_value(1379), scratch.ad_value(1341))), AdValue::offset(AdValue::scale(AdValue::sub_from_scalar((-230.25850929940458), AdValue::sub(scratch.ad_value(1379), scratch.ad_value(1341))), 0.3333333333333333), 1.0)), 0.5), 1.0)), 1.0));
            scratch.store_ad(1359, &assign24680_ad_e27865);
        }

        if ((((scratch.values[1323] != 0.0) && (scratch.values[1340] != 0.0)) && (!(scratch.values[1633] != 0.0))) && (!(scratch.values[1638] != 0.0))) {
            scratch.store_ad(1343, &AdValue::mul(AdValue::add(AdValue::add(AdValue::scale(scratch.ad_value(1342), 0.29214664), AdValue::scale(AdValue::square(scratch.ad_value(1342)), scratch.values[422])), AdValue::scale(AdValue::mul(AdValue::square(scratch.ad_value(1342)), scratch.ad_value(1342)), scratch.values[423])), scratch.ad_value(1359)));
        }

        scratch.values[1642] = if (scratch.values[1380] > 0.0) { 1.0 } else { 0.0 };

        if (((((scratch.values[1323] != 0.0) && (scratch.values[1340] != 0.0)) && (!(scratch.values[1633] != 0.0))) && (!(scratch.values[1638] != 0.0))) && (scratch.values[1642] != 0.0)) {
            scratch.values[1381] = scratch.values[1343];
            scratch.node_derivatives[1381] = scratch.node_derivatives[1343];
            scratch.branch_derivatives[1381] = scratch.branch_derivatives[1343];
        }

        scratch.values[1643] = if (scratch.values[1379] > (-230.25850929940458)) { 1.0 } else { 0.0 };

        if ((((((scratch.values[1323] != 0.0) && (scratch.values[1340] != 0.0)) && (!(scratch.values[1633] != 0.0))) && (!(scratch.values[1638] != 0.0))) && (!(scratch.values[1642] != 0.0))) && (scratch.values[1643] != 0.0)) {
            scratch.store_ad(1359, &AdValue::exp(scratch.ad_value(1379)));
        }

        if ((((((scratch.values[1323] != 0.0) && (scratch.values[1340] != 0.0)) && (!(scratch.values[1633] != 0.0))) && (!(scratch.values[1638] != 0.0))) && (!(scratch.values[1642] != 0.0))) && (!(scratch.values[1643] != 0.0))) {
            scratch.store_ad(1359, &AdValue::div_from_scalar(1e-100, AdValue::offset(AdValue::mul(AdValue::sub_from_scalar((-230.25850929940458), scratch.ad_value(1379)), AdValue::offset(AdValue::scale(AdValue::mul(AdValue::sub_from_scalar((-230.25850929940458), scratch.ad_value(1379)), AdValue::offset(AdValue::scale(AdValue::sub_from_scalar((-230.25850929940458), scratch.ad_value(1379)), 0.3333333333333333), 1.0)), 0.5), 1.0)), 1.0)));
        }

        if (((((scratch.values[1323] != 0.0) && (scratch.values[1340] != 0.0)) && (!(scratch.values[1633] != 0.0))) && (!(scratch.values[1638] != 0.0))) && (!(scratch.values[1642] != 0.0))) {
            scratch.store_ad(1381, &AdValue::sub(AdValue::scale(scratch.ad_value(1359), 2.0), scratch.ad_value(1343)));
        }

        if ((((scratch.values[1323] != 0.0) && (scratch.values[1340] != 0.0)) && (!(scratch.values[1633] != 0.0))) && (!(scratch.values[1638] != 0.0))) {
            scratch.store_ad(1382, &AdValue::scale(AdValue::div(AdValue::scale(scratch.ad_value(1381), scratch.values[484]), scratch.ad_value(1377)), (1.772453850905516 * 0.5)));
        }

        if ((((scratch.values[1323] != 0.0) && (scratch.values[1340] != 0.0)) && (!(scratch.values[1633] != 0.0))) && (!(scratch.values[1638] != 0.0))) {
            scratch.store_ad(1368, &AdValue::scale(AdValue::mul(AdValue::mul(scratch.ad_value(1367), scratch.ad_value(1382)), scratch.ad_value(1376)), scratch.values[389]));
        }

        scratch.values[1644] = if (scratch.values[395] == 0.0) { 1.0 } else { 0.0 };

        if ((((scratch.values[1323] != 0.0) && (scratch.values[1340] != 0.0)) && (!(scratch.values[1633] != 0.0))) && (scratch.values[1644] != 0.0)) {
            scratch.values[1383] = 0.0;
            scratch.node_derivatives[1383] = [0.0; Instance::NODE_COUNT];
            scratch.branch_derivatives[1383] = [0.0; Instance::BRANCH_COUNT];
        }

        scratch.values[1645] = if (scratch.values[375] == 0.5) { 1.0 } else { 0.0 };

        if (((((scratch.values[1323] != 0.0) && (scratch.values[1340] != 0.0)) && (!(scratch.values[1633] != 0.0))) && (!(scratch.values[1644] != 0.0))) && (scratch.values[1645] != 0.0)) {
            scratch.store_ad(1359, &AdValue::sqrt(AdValue::scale(AdValue::sub_from_scalar(scratch.values[372], scratch.ad_value(1357)), scratch.values[478])));
        }

        if (((((scratch.values[1323] != 0.0) && (scratch.values[1340] != 0.0)) && (!(scratch.values[1633] != 0.0))) && (!(scratch.values[1644] != 0.0))) && (!(scratch.values[1645] != 0.0))) {
            scratch.store_ad(1359, &AdValue::powf(AdValue::scale(AdValue::sub_from_scalar(scratch.values[372], scratch.ad_value(1357)), scratch.values[478]), scratch.values[375]));
        }

        if ((((scratch.values[1323] != 0.0) && (scratch.values[1340] != 0.0)) && (!(scratch.values[1633] != 0.0))) && (!(scratch.values[1644] != 0.0))) {
            scratch.store_ad(1384, &AdValue::scale(AdValue::div(AdValue::scale(AdValue::sub_from_scalar(scratch.values[372], scratch.ad_value(1357)), scratch.values[475]), scratch.ad_value(1359)), scratch.values[460]));
        }

        scratch.values[1646] = if (((((-scratch.values[490]) / scratch.values[1384])) as f64).abs() < 230.25850929940458) { 1.0 } else { 0.0 };

        if (((((scratch.values[1323] != 0.0) && (scratch.values[1340] != 0.0)) && (!(scratch.values[1633] != 0.0))) && (!(scratch.values[1644] != 0.0))) && (scratch.values[1646] != 0.0)) {
            scratch.store_ad(1359, &AdValue::exp(AdValue::div(AdValue::neg(scratch.ad_value(490)), scratch.ad_value(1384))));
        }

        scratch.values[1647] = if (((-scratch.values[490]) / scratch.values[1384]) < 0.0) { 1.0 } else { 0.0 };

        if ((((((scratch.values[1323] != 0.0) && (scratch.values[1340] != 0.0)) && (!(scratch.values[1633] != 0.0))) && (!(scratch.values[1644] != 0.0))) && (!(scratch.values[1646] != 0.0))) && (scratch.values[1647] != 0.0)) {
            let assign24870_ad_e28192: AdValue = AdValue::offset(AdValue::mul(AdValue::sub_from_scalar((-230.25850929940458), AdValue::div(AdValue::neg(scratch.ad_value(490)), scratch.ad_value(1384))), AdValue::offset(AdValue::scale(AdValue::mul(AdValue::sub_from_scalar((-230.25850929940458), AdValue::div(AdValue::neg(scratch.ad_value(490)), scratch.ad_value(1384))), AdValue::offset(AdValue::scale(AdValue::sub_from_scalar((-230.25850929940458), AdValue::div(AdValue::neg(scratch.ad_value(490)), scratch.ad_value(1384))), 0.3333333333333333), 1.0)), 0.5), 1.0)), 1.0);
            scratch.store_ad(1359, &AdValue::div_from_scalar(1e-100, assign24870_ad_e28192));
        }

        if ((((((scratch.values[1323] != 0.0) && (scratch.values[1340] != 0.0)) && (!(scratch.values[1633] != 0.0))) && (!(scratch.values[1644] != 0.0))) && (!(scratch.values[1646] != 0.0))) && (!(scratch.values[1647] != 0.0))) {
            let assign24880_ad_e28242: AdValue = AdValue::scale(AdValue::offset(AdValue::mul(AdValue::offset(AdValue::div(AdValue::neg(scratch.ad_value(490)), scratch.ad_value(1384)), (-230.25850929940458)), AdValue::offset(AdValue::scale(AdValue::mul(AdValue::offset(AdValue::div(AdValue::neg(scratch.ad_value(490)), scratch.ad_value(1384)), (-230.25850929940458)), AdValue::offset(AdValue::scale(AdValue::offset(AdValue::div(AdValue::neg(scratch.ad_value(490)), scratch.ad_value(1384)), (-230.25850929940458)), 0.3333333333333333), 1.0)), 0.5), 1.0)), 1.0), 1e100);
            scratch.store_ad(1359, &assign24880_ad_e28242);
        }

        if ((((scratch.values[1323] != 0.0) && (scratch.values[1340] != 0.0)) && (!(scratch.values[1633] != 0.0))) && (!(scratch.values[1644] != 0.0))) {
            scratch.store_ad(1383, &AdValue::scale(AdValue::mul(AdValue::mul(AdValue::mul(scratch.ad_value(520), scratch.ad_value(1384)), scratch.ad_value(1384)), scratch.ad_value(1359)), scratch.values[395]));
        }

        scratch.values[1648] = if (scratch.values[404] > 1000.0) { 1.0 } else { 0.0 };

        if ((((scratch.values[1323] != 0.0) && (scratch.values[1340] != 0.0)) && (!(scratch.values[1633] != 0.0))) && (scratch.values[1648] != 0.0)) {
            scratch.values[1385] = 1.0;
            scratch.node_derivatives[1385] = [0.0; Instance::NODE_COUNT];
            scratch.branch_derivatives[1385] = [0.0; Instance::BRANCH_COUNT];
        }

        scratch.values[1649] = if (scratch.values[1358] > ((-scratch.values[493]) * scratch.values[404])) { 1.0 } else { 0.0 };

        scratch.values[1650] = if (scratch.values[407] == 4.0) { 1.0 } else { 0.0 };

        if ((((((scratch.values[1323] != 0.0) && (scratch.values[1340] != 0.0)) && (!(scratch.values[1633] != 0.0))) && (!(scratch.values[1648] != 0.0))) && (scratch.values[1649] != 0.0)) && (scratch.values[1650] != 0.0)) {
            scratch.store_ad(1359, &AdValue::mul(AdValue::mul(AdValue::mul(AdValue::scale(scratch.ad_value(1358), scratch.values[497]), AdValue::scale(scratch.ad_value(1358), scratch.values[497])), AdValue::scale(scratch.ad_value(1358), scratch.values[497])), AdValue::scale(scratch.ad_value(1358), scratch.values[497])));
        }

        if ((((((scratch.values[1323] != 0.0) && (scratch.values[1340] != 0.0)) && (!(scratch.values[1633] != 0.0))) && (!(scratch.values[1648] != 0.0))) && (scratch.values[1649] != 0.0)) && (!(scratch.values[1650] != 0.0))) {
            scratch.store_ad(1359, &AdValue::powf(AdValue::abs(AdValue::scale(scratch.ad_value(1358), scratch.values[497])), scratch.values[407]));
        }

        if (((((scratch.values[1323] != 0.0) && (scratch.values[1340] != 0.0)) && (!(scratch.values[1633] != 0.0))) && (!(scratch.values[1648] != 0.0))) && (scratch.values[1649] != 0.0)) {
            scratch.store_ad(1385, &AdValue::div_from_scalar(1.0, AdValue::sub_from_scalar(1.0, scratch.ad_value(1359))));
        }

        if (((((scratch.values[1323] != 0.0) && (scratch.values[1340] != 0.0)) && (!(scratch.values[1633] != 0.0))) && (!(scratch.values[1648] != 0.0))) && (!(scratch.values[1649] != 0.0))) {
            scratch.store_ad(1385, &AdValue::offset(AdValue::scale(AdValue::offset(scratch.ad_value(1358), (scratch.values[493] * scratch.values[404])), scratch.values[500]), scratch.values[494]));
        }

        if (((scratch.values[1323] != 0.0) && (scratch.values[1340] != 0.0)) && (!(scratch.values[1633] != 0.0))) {
            scratch.store_ad(1386, &AdValue::mul(AdValue::add(AdValue::add(AdValue::add(scratch.ad_value(1360), scratch.ad_value(1361)), scratch.ad_value(1368)), scratch.ad_value(1383)), scratch.ad_value(1385)));
        }

        scratch.values[1651] = if (scratch.values[691] == 0.0) { 1.0 } else { 0.0 };

        if (((scratch.values[1323] != 0.0) && (scratch.values[1340] != 0.0)) && (scratch.values[1651] != 0.0)) {
            scratch.values[1387] = 0.0;
            scratch.node_derivatives[1387] = [0.0; Instance::NODE_COUNT];
            scratch.branch_derivatives[1387] = [0.0; Instance::BRANCH_COUNT];
        }

        scratch.values[1652] = if (scratch.values[458] == 0.5) { 1.0 } else { 0.0 };

        if ((((scratch.values[1323] != 0.0) && (scratch.values[1340] != 0.0)) && (!(scratch.values[1651] != 0.0))) && (scratch.values[1652] != 0.0)) {
            scratch.store_ad(1359, &AdValue::sqrt(AdValue::sub_from_scalar(1.0, AdValue::scale(scratch.ad_value(1351), scratch.values[455]))));
        }

        if ((((scratch.values[1323] != 0.0) && (scratch.values[1340] != 0.0)) && (!(scratch.values[1651] != 0.0))) && (!(scratch.values[1652] != 0.0))) {
            scratch.store_ad(1359, &AdValue::powf(AdValue::sub_from_scalar(1.0, AdValue::scale(scratch.ad_value(1351), scratch.values[455])), scratch.values[458]));
        }

        if (((scratch.values[1323] != 0.0) && (scratch.values[1340] != 0.0)) && (!(scratch.values[1651] != 0.0))) {
            scratch.store_ad(1360, &AdValue::scale(scratch.ad_value(1350), scratch.values[437]));
        }

        scratch.values[1653] = if ((scratch.values[387] == 0.0) && (scratch.values[390] == 0.0)) { 1.0 } else { 0.0 };

        if ((((scratch.values[1323] != 0.0) && (scratch.values[1340] != 0.0)) && (!(scratch.values[1651] != 0.0))) && (scratch.values[1653] != 0.0)) {
            scratch.values[1361] = 0.0;
            scratch.node_derivatives[1361] = [0.0; Instance::NODE_COUNT];
            scratch.branch_derivatives[1361] = [0.0; Instance::BRANCH_COUNT];
        }

        if ((((scratch.values[1323] != 0.0) && (scratch.values[1340] != 0.0)) && (!(scratch.values[1651] != 0.0))) && (!(scratch.values[1653] != 0.0))) {
            scratch.store_ad(1362, &AdValue::sub_from_scalar(scratch.values[443], scratch.ad_value(1356)));
        }

        if ((((scratch.values[1323] != 0.0) && (scratch.values[1340] != 0.0)) && (!(scratch.values[1651] != 0.0))) && (!(scratch.values[1653] != 0.0))) {
            scratch.store_ad(1363, &AdValue::sub_from_scalar(1.0, AdValue::sqrt(AdValue::sub_from_scalar(1.0, AdValue::div(scratch.ad_value(1354), scratch.ad_value(1362))))));
        }

        scratch.values[1654] = if (scratch.values[376] == 0.5) { 1.0 } else { 0.0 };

        if (((((scratch.values[1323] != 0.0) && (scratch.values[1340] != 0.0)) && (!(scratch.values[1651] != 0.0))) && (!(scratch.values[1653] != 0.0))) && (scratch.values[1654] != 0.0)) {
            scratch.values[1364] = 0.0;
            scratch.node_derivatives[1364] = [0.0; Instance::NODE_COUNT];
            scratch.branch_derivatives[1364] = [0.0; Instance::BRANCH_COUNT];
        }

        if (((((scratch.values[1323] != 0.0) && (scratch.values[1340] != 0.0)) && (!(scratch.values[1651] != 0.0))) && (!(scratch.values[1653] != 0.0))) && (!(scratch.values[1654] != 0.0))) {
            scratch.store_ad(1364, &AdValue::scale(AdValue::add(AdValue::div(AdValue::mul(AdValue::square(scratch.ad_value(1363)), AdValue::ln(scratch.ad_value(1363))), AdValue::sub_from_scalar(1.0, scratch.ad_value(1363))), scratch.ad_value(1363)), (1.0 - (2.0 * scratch.values[376]))));
        }

        if ((((scratch.values[1323] != 0.0) && (scratch.values[1340] != 0.0)) && (!(scratch.values[1651] != 0.0))) && (!(scratch.values[1653] != 0.0))) {
            scratch.store_ad(1365, &AdValue::add(scratch.ad_value(1363), scratch.ad_value(1364)));
        }

        scratch.values[1655] = if (scratch.values[376] == 0.5) { 1.0 } else { 0.0 };

        if (((((scratch.values[1323] != 0.0) && (scratch.values[1340] != 0.0)) && (!(scratch.values[1651] != 0.0))) && (!(scratch.values[1653] != 0.0))) && (scratch.values[1655] != 0.0)) {
            scratch.store_ad(1359, &AdValue::sqrt(AdValue::scale(scratch.ad_value(1362), scratch.values[479])));
        }

        if (((((scratch.values[1323] != 0.0) && (scratch.values[1340] != 0.0)) && (!(scratch.values[1651] != 0.0))) && (!(scratch.values[1653] != 0.0))) && (!(scratch.values[1655] != 0.0))) {
            scratch.store_ad(1359, &AdValue::powf(AdValue::scale(scratch.ad_value(1362), scratch.values[479]), scratch.values[376]));
        }

        if ((((scratch.values[1323] != 0.0) && (scratch.values[1340] != 0.0)) && (!(scratch.values[1651] != 0.0))) && (!(scratch.values[1653] != 0.0))) {
            scratch.store_ad(1366, &AdValue::scale(scratch.ad_value(1359), scratch.values[473]));
        }

        if ((((scratch.values[1323] != 0.0) && (scratch.values[1340] != 0.0)) && (!(scratch.values[1651] != 0.0))) && (!(scratch.values[1653] != 0.0))) {
            scratch.store_ad(1367, &AdValue::scale(AdValue::mul(AdValue::offset(scratch.ad_value(1353), (-1.0)), scratch.ad_value(1366)), scratch.values[434]));
        }

        if ((((scratch.values[1323] != 0.0) && (scratch.values[1340] != 0.0)) && (!(scratch.values[1651] != 0.0))) && (!(scratch.values[1653] != 0.0))) {
            scratch.store_ad(1361, &AdValue::scale(AdValue::mul(scratch.ad_value(1367), scratch.ad_value(1365)), scratch.values[387]));
        }

        scratch.values[1656] = if (scratch.values[390] == 0.0) { 1.0 } else { 0.0 };

        if ((((scratch.values[1323] != 0.0) && (scratch.values[1340] != 0.0)) && (!(scratch.values[1651] != 0.0))) && (scratch.values[1656] != 0.0)) {
            scratch.values[1368] = 0.0;
            scratch.node_derivatives[1368] = [0.0; Instance::NODE_COUNT];
            scratch.branch_derivatives[1368] = [0.0; Instance::BRANCH_COUNT];
        }

        if ((((scratch.values[1323] != 0.0) && (scratch.values[1340] != 0.0)) && (!(scratch.values[1651] != 0.0))) && (!(scratch.values[1656] != 0.0))) {
            scratch.store_ad(1369, &AdValue::scale(AdValue::div(AdValue::scale(scratch.ad_value(1366), scratch.values[458]), scratch.ad_value(1362)), scratch.values[488]));
        }

        if ((((scratch.values[1323] != 0.0) && (scratch.values[1340] != 0.0)) && (!(scratch.values[1651] != 0.0))) && (!(scratch.values[1656] != 0.0))) {
            scratch.store_ad(1370, &AdValue::div_from_scalar((0.666666666666667 * scratch.values[485]), scratch.ad_value(1369)));
        }

        if ((((scratch.values[1323] != 0.0) && (scratch.values[1340] != 0.0)) && (!(scratch.values[1651] != 0.0))) && (!(scratch.values[1656] != 0.0))) {
            scratch.store_ad(1371, &AdValue::square(scratch.ad_value(1370)));
        }

        if ((((scratch.values[1323] != 0.0) && (scratch.values[1340] != 0.0)) && (!(scratch.values[1651] != 0.0))) && (!(scratch.values[1656] != 0.0))) {
            scratch.store_ad(1372, &AdValue::sqrt(AdValue::div(AdValue::square(scratch.ad_value(1371)), AdValue::offset(AdValue::square(scratch.ad_value(1371)), 1.0))));
        }

        if ((((scratch.values[1323] != 0.0) && (scratch.values[1340] != 0.0)) && (!(scratch.values[1651] != 0.0))) && (!(scratch.values[1656] != 0.0))) {
            scratch.store_ad(1373, &AdValue::sqrt(AdValue::abs(scratch.ad_value(1372))));
        }

        if ((((scratch.values[1323] != 0.0) && (scratch.values[1340] != 0.0)) && (!(scratch.values[1651] != 0.0))) && (!(scratch.values[1656] != 0.0))) {
            scratch.store_ad(1374, &AdValue::mul(scratch.ad_value(1372), scratch.ad_value(1373)));
        }

        scratch.values[1657] = if (((-scratch.values[376]) * scratch.values[461]) == (-1.0)) { 1.0 } else { 0.0 };

        if (((((scratch.values[1323] != 0.0) && (scratch.values[1340] != 0.0)) && (!(scratch.values[1651] != 0.0))) && (!(scratch.values[1656] != 0.0))) && (scratch.values[1657] != 0.0)) {
            scratch.store_ad(1375, &AdValue::div_from_scalar(1.0, AdValue::offset(AdValue::mul(scratch.ad_value(1369), scratch.ad_value(1374)), 1.0)));
        }

        if (((((scratch.values[1323] != 0.0) && (scratch.values[1340] != 0.0)) && (!(scratch.values[1651] != 0.0))) && (!(scratch.values[1656] != 0.0))) && (!(scratch.values[1657] != 0.0))) {
            scratch.store_ad(1375, &AdValue::powf(AdValue::offset(AdValue::mul(scratch.ad_value(1369), scratch.ad_value(1374)), 1.0), ((-scratch.values[376]) * scratch.values[461])));
        }

        if ((((scratch.values[1323] != 0.0) && (scratch.values[1340] != 0.0)) && (!(scratch.values[1651] != 0.0))) && (!(scratch.values[1656] != 0.0))) {
            scratch.store_ad(1376, &AdValue::div(AdValue::mul(scratch.ad_value(1365), scratch.ad_value(1375)), AdValue::add(scratch.ad_value(1365), scratch.ad_value(1375))));
        }

        if ((((scratch.values[1323] != 0.0) && (scratch.values[1340] != 0.0)) && (!(scratch.values[1651] != 0.0))) && (!(scratch.values[1656] != 0.0))) {
            scratch.store_ad(1377, &AdValue::sqrt(AdValue::scale(AdValue::div(scratch.ad_value(1369), scratch.ad_value(1373)), 0.375)));
        }

        if ((((scratch.values[1323] != 0.0) && (scratch.values[1340] != 0.0)) && (!(scratch.values[1651] != 0.0))) && (!(scratch.values[1656] != 0.0))) {
            scratch.store_ad(1378, &AdValue::sub(AdValue::scale(AdValue::mul(scratch.ad_value(1370), scratch.ad_value(1373)), 2.0), scratch.ad_value(1372)));
        }

        if ((((scratch.values[1323] != 0.0) && (scratch.values[1340] != 0.0)) && (!(scratch.values[1651] != 0.0))) && (!(scratch.values[1656] != 0.0))) {
            scratch.store_ad(1379, &AdValue::add(AdValue::sub(AdValue::mul(AdValue::scale(scratch.ad_value(1370), scratch.values[485]), scratch.ad_value(1373)), AdValue::scale(scratch.ad_value(1372), scratch.values[485])), AdValue::scale(AdValue::mul(scratch.ad_value(1369), scratch.ad_value(1374)), 0.5)));
        }

        if ((((scratch.values[1323] != 0.0) && (scratch.values[1340] != 0.0)) && (!(scratch.values[1651] != 0.0))) && (!(scratch.values[1656] != 0.0))) {
            scratch.store_ad(1380, &AdValue::mul(AdValue::offset(scratch.ad_value(1378), (-1.0)), scratch.ad_value(1377)));
        }

        if ((((scratch.values[1323] != 0.0) && (scratch.values[1340] != 0.0)) && (!(scratch.values[1651] != 0.0))) && (!(scratch.values[1656] != 0.0))) {
            scratch.store_ad(1341, &AdValue::square(scratch.ad_value(1380)));
        }

        scratch.values[1658] = if (scratch.values[1380] > 0.0) { 1.0 } else { 0.0 };

        if (((((scratch.values[1323] != 0.0) && (scratch.values[1340] != 0.0)) && (!(scratch.values[1651] != 0.0))) && (!(scratch.values[1656] != 0.0))) && (scratch.values[1658] != 0.0)) {
            scratch.store_ad(1342, &AdValue::div_from_scalar(1.0, AdValue::offset(AdValue::scale(scratch.ad_value(1380), scratch.values[421]), 1.0)));
        }

        if (((((scratch.values[1323] != 0.0) && (scratch.values[1340] != 0.0)) && (!(scratch.values[1651] != 0.0))) && (!(scratch.values[1656] != 0.0))) && (!(scratch.values[1658] != 0.0))) {
            scratch.store_ad(1342, &AdValue::div_from_scalar(1.0, AdValue::sub_from_scalar(1.0, AdValue::scale(scratch.ad_value(1380), scratch.values[421]))));
        }

        scratch.values[1659] = if (((-scratch.values[1341]) + scratch.values[1379]) > (-230.25850929940458)) { 1.0 } else { 0.0 };

        if (((((scratch.values[1323] != 0.0) && (scratch.values[1340] != 0.0)) && (!(scratch.values[1651] != 0.0))) && (!(scratch.values[1656] != 0.0))) && (scratch.values[1659] != 0.0)) {
            scratch.store_ad(1359, &AdValue::exp(AdValue::sub(scratch.ad_value(1379), scratch.ad_value(1341))));
        }

        if (((((scratch.values[1323] != 0.0) && (scratch.values[1340] != 0.0)) && (!(scratch.values[1651] != 0.0))) && (!(scratch.values[1656] != 0.0))) && (!(scratch.values[1659] != 0.0))) {
            let assign25430_ad_e29071: AdValue = AdValue::div_from_scalar(1e-100, AdValue::offset(AdValue::mul(AdValue::sub_from_scalar((-230.25850929940458), AdValue::sub(scratch.ad_value(1379), scratch.ad_value(1341))), AdValue::offset(AdValue::scale(AdValue::mul(AdValue::sub_from_scalar((-230.25850929940458), AdValue::sub(scratch.ad_value(1379), scratch.ad_value(1341))), AdValue::offset(AdValue::scale(AdValue::sub_from_scalar((-230.25850929940458), AdValue::sub(scratch.ad_value(1379), scratch.ad_value(1341))), 0.3333333333333333), 1.0)), 0.5), 1.0)), 1.0));
            scratch.store_ad(1359, &assign25430_ad_e29071);
        }

        if ((((scratch.values[1323] != 0.0) && (scratch.values[1340] != 0.0)) && (!(scratch.values[1651] != 0.0))) && (!(scratch.values[1656] != 0.0))) {
            scratch.store_ad(1343, &AdValue::mul(AdValue::add(AdValue::add(AdValue::scale(scratch.ad_value(1342), 0.29214664), AdValue::scale(AdValue::square(scratch.ad_value(1342)), scratch.values[422])), AdValue::scale(AdValue::mul(AdValue::square(scratch.ad_value(1342)), scratch.ad_value(1342)), scratch.values[423])), scratch.ad_value(1359)));
        }

        scratch.values[1660] = if (scratch.values[1380] > 0.0) { 1.0 } else { 0.0 };

        if (((((scratch.values[1323] != 0.0) && (scratch.values[1340] != 0.0)) && (!(scratch.values[1651] != 0.0))) && (!(scratch.values[1656] != 0.0))) && (scratch.values[1660] != 0.0)) {
            scratch.values[1381] = scratch.values[1343];
            scratch.node_derivatives[1381] = scratch.node_derivatives[1343];
            scratch.branch_derivatives[1381] = scratch.branch_derivatives[1343];
        }

        scratch.values[1661] = if (scratch.values[1379] > (-230.25850929940458)) { 1.0 } else { 0.0 };

        if ((((((scratch.values[1323] != 0.0) && (scratch.values[1340] != 0.0)) && (!(scratch.values[1651] != 0.0))) && (!(scratch.values[1656] != 0.0))) && (!(scratch.values[1660] != 0.0))) && (scratch.values[1661] != 0.0)) {
            scratch.store_ad(1359, &AdValue::exp(scratch.ad_value(1379)));
        }

        if ((((((scratch.values[1323] != 0.0) && (scratch.values[1340] != 0.0)) && (!(scratch.values[1651] != 0.0))) && (!(scratch.values[1656] != 0.0))) && (!(scratch.values[1660] != 0.0))) && (!(scratch.values[1661] != 0.0))) {
            scratch.store_ad(1359, &AdValue::div_from_scalar(1e-100, AdValue::offset(AdValue::mul(AdValue::sub_from_scalar((-230.25850929940458), scratch.ad_value(1379)), AdValue::offset(AdValue::scale(AdValue::mul(AdValue::sub_from_scalar((-230.25850929940458), scratch.ad_value(1379)), AdValue::offset(AdValue::scale(AdValue::sub_from_scalar((-230.25850929940458), scratch.ad_value(1379)), 0.3333333333333333), 1.0)), 0.5), 1.0)), 1.0)));
        }

        if (((((scratch.values[1323] != 0.0) && (scratch.values[1340] != 0.0)) && (!(scratch.values[1651] != 0.0))) && (!(scratch.values[1656] != 0.0))) && (!(scratch.values[1660] != 0.0))) {
            scratch.store_ad(1381, &AdValue::sub(AdValue::scale(scratch.ad_value(1359), 2.0), scratch.ad_value(1343)));
        }

        if ((((scratch.values[1323] != 0.0) && (scratch.values[1340] != 0.0)) && (!(scratch.values[1651] != 0.0))) && (!(scratch.values[1656] != 0.0))) {
            scratch.store_ad(1382, &AdValue::scale(AdValue::div(AdValue::scale(scratch.ad_value(1381), scratch.values[485]), scratch.ad_value(1377)), (1.772453850905516 * 0.5)));
        }

        if ((((scratch.values[1323] != 0.0) && (scratch.values[1340] != 0.0)) && (!(scratch.values[1651] != 0.0))) && (!(scratch.values[1656] != 0.0))) {
            scratch.store_ad(1368, &AdValue::scale(AdValue::mul(AdValue::mul(scratch.ad_value(1367), scratch.ad_value(1382)), scratch.ad_value(1376)), scratch.values[390]));
        }

        scratch.values[1662] = if (scratch.values[396] == 0.0) { 1.0 } else { 0.0 };

        if ((((scratch.values[1323] != 0.0) && (scratch.values[1340] != 0.0)) && (!(scratch.values[1651] != 0.0))) && (scratch.values[1662] != 0.0)) {
            scratch.values[1383] = 0.0;
            scratch.node_derivatives[1383] = [0.0; Instance::NODE_COUNT];
            scratch.branch_derivatives[1383] = [0.0; Instance::BRANCH_COUNT];
        }

        scratch.values[1663] = if (scratch.values[376] == 0.5) { 1.0 } else { 0.0 };

        if (((((scratch.values[1323] != 0.0) && (scratch.values[1340] != 0.0)) && (!(scratch.values[1651] != 0.0))) && (!(scratch.values[1662] != 0.0))) && (scratch.values[1663] != 0.0)) {
            scratch.store_ad(1359, &AdValue::sqrt(AdValue::scale(AdValue::sub_from_scalar(scratch.values[373], scratch.ad_value(1357)), scratch.values[479])));
        }

        if (((((scratch.values[1323] != 0.0) && (scratch.values[1340] != 0.0)) && (!(scratch.values[1651] != 0.0))) && (!(scratch.values[1662] != 0.0))) && (!(scratch.values[1663] != 0.0))) {
            scratch.store_ad(1359, &AdValue::powf(AdValue::scale(AdValue::sub_from_scalar(scratch.values[373], scratch.ad_value(1357)), scratch.values[479]), scratch.values[376]));
        }

        if ((((scratch.values[1323] != 0.0) && (scratch.values[1340] != 0.0)) && (!(scratch.values[1651] != 0.0))) && (!(scratch.values[1662] != 0.0))) {
            scratch.store_ad(1384, &AdValue::scale(AdValue::div(AdValue::scale(AdValue::sub_from_scalar(scratch.values[373], scratch.ad_value(1357)), scratch.values[476]), scratch.ad_value(1359)), scratch.values[461]));
        }

        scratch.values[1664] = if (((((-scratch.values[491]) / scratch.values[1384])) as f64).abs() < 230.25850929940458) { 1.0 } else { 0.0 };

        if (((((scratch.values[1323] != 0.0) && (scratch.values[1340] != 0.0)) && (!(scratch.values[1651] != 0.0))) && (!(scratch.values[1662] != 0.0))) && (scratch.values[1664] != 0.0)) {
            scratch.store_ad(1359, &AdValue::exp(AdValue::div(AdValue::neg(scratch.ad_value(491)), scratch.ad_value(1384))));
        }

        scratch.values[1665] = if (((-scratch.values[491]) / scratch.values[1384]) < 0.0) { 1.0 } else { 0.0 };

        if ((((((scratch.values[1323] != 0.0) && (scratch.values[1340] != 0.0)) && (!(scratch.values[1651] != 0.0))) && (!(scratch.values[1662] != 0.0))) && (!(scratch.values[1664] != 0.0))) && (scratch.values[1665] != 0.0)) {
            let assign25620_ad_e29398: AdValue = AdValue::offset(AdValue::mul(AdValue::sub_from_scalar((-230.25850929940458), AdValue::div(AdValue::neg(scratch.ad_value(491)), scratch.ad_value(1384))), AdValue::offset(AdValue::scale(AdValue::mul(AdValue::sub_from_scalar((-230.25850929940458), AdValue::div(AdValue::neg(scratch.ad_value(491)), scratch.ad_value(1384))), AdValue::offset(AdValue::scale(AdValue::sub_from_scalar((-230.25850929940458), AdValue::div(AdValue::neg(scratch.ad_value(491)), scratch.ad_value(1384))), 0.3333333333333333), 1.0)), 0.5), 1.0)), 1.0);
            scratch.store_ad(1359, &AdValue::div_from_scalar(1e-100, assign25620_ad_e29398));
        }

        if ((((((scratch.values[1323] != 0.0) && (scratch.values[1340] != 0.0)) && (!(scratch.values[1651] != 0.0))) && (!(scratch.values[1662] != 0.0))) && (!(scratch.values[1664] != 0.0))) && (!(scratch.values[1665] != 0.0))) {
            let assign25630_ad_e29448: AdValue = AdValue::scale(AdValue::offset(AdValue::mul(AdValue::offset(AdValue::div(AdValue::neg(scratch.ad_value(491)), scratch.ad_value(1384)), (-230.25850929940458)), AdValue::offset(AdValue::scale(AdValue::mul(AdValue::offset(AdValue::div(AdValue::neg(scratch.ad_value(491)), scratch.ad_value(1384)), (-230.25850929940458)), AdValue::offset(AdValue::scale(AdValue::offset(AdValue::div(AdValue::neg(scratch.ad_value(491)), scratch.ad_value(1384)), (-230.25850929940458)), 0.3333333333333333), 1.0)), 0.5), 1.0)), 1.0), 1e100);
            scratch.store_ad(1359, &assign25630_ad_e29448);
        }

        if ((((scratch.values[1323] != 0.0) && (scratch.values[1340] != 0.0)) && (!(scratch.values[1651] != 0.0))) && (!(scratch.values[1662] != 0.0))) {
            scratch.store_ad(1383, &AdValue::scale(AdValue::mul(AdValue::mul(AdValue::mul(scratch.ad_value(520), scratch.ad_value(1384)), scratch.ad_value(1384)), scratch.ad_value(1359)), scratch.values[396]));
        }

        scratch.values[1666] = if (scratch.values[405] > 1000.0) { 1.0 } else { 0.0 };

        if ((((scratch.values[1323] != 0.0) && (scratch.values[1340] != 0.0)) && (!(scratch.values[1651] != 0.0))) && (scratch.values[1666] != 0.0)) {
            scratch.values[1385] = 1.0;
            scratch.node_derivatives[1385] = [0.0; Instance::NODE_COUNT];
            scratch.branch_derivatives[1385] = [0.0; Instance::BRANCH_COUNT];
        }

        scratch.values[1667] = if (scratch.values[1358] > ((-scratch.values[493]) * scratch.values[405])) { 1.0 } else { 0.0 };

        scratch.values[1668] = if (scratch.values[408] == 4.0) { 1.0 } else { 0.0 };

        if ((((((scratch.values[1323] != 0.0) && (scratch.values[1340] != 0.0)) && (!(scratch.values[1651] != 0.0))) && (!(scratch.values[1666] != 0.0))) && (scratch.values[1667] != 0.0)) && (scratch.values[1668] != 0.0)) {
            scratch.store_ad(1359, &AdValue::mul(AdValue::mul(AdValue::mul(AdValue::scale(scratch.ad_value(1358), scratch.values[498]), AdValue::scale(scratch.ad_value(1358), scratch.values[498])), AdValue::scale(scratch.ad_value(1358), scratch.values[498])), AdValue::scale(scratch.ad_value(1358), scratch.values[498])));
        }

        if ((((((scratch.values[1323] != 0.0) && (scratch.values[1340] != 0.0)) && (!(scratch.values[1651] != 0.0))) && (!(scratch.values[1666] != 0.0))) && (scratch.values[1667] != 0.0)) && (!(scratch.values[1668] != 0.0))) {
            scratch.store_ad(1359, &AdValue::powf(AdValue::abs(AdValue::scale(scratch.ad_value(1358), scratch.values[498])), scratch.values[408]));
        }

        if (((((scratch.values[1323] != 0.0) && (scratch.values[1340] != 0.0)) && (!(scratch.values[1651] != 0.0))) && (!(scratch.values[1666] != 0.0))) && (scratch.values[1667] != 0.0)) {
            scratch.store_ad(1385, &AdValue::div_from_scalar(1.0, AdValue::sub_from_scalar(1.0, scratch.ad_value(1359))));
        }

        if (((((scratch.values[1323] != 0.0) && (scratch.values[1340] != 0.0)) && (!(scratch.values[1651] != 0.0))) && (!(scratch.values[1666] != 0.0))) && (!(scratch.values[1667] != 0.0))) {
            scratch.store_ad(1385, &AdValue::offset(AdValue::scale(AdValue::offset(scratch.ad_value(1358), (scratch.values[493] * scratch.values[405])), scratch.values[501]), scratch.values[495]));
        }

        if (((scratch.values[1323] != 0.0) && (scratch.values[1340] != 0.0)) && (!(scratch.values[1651] != 0.0))) {
            scratch.store_ad(1387, &AdValue::mul(AdValue::add(AdValue::add(AdValue::add(scratch.ad_value(1360), scratch.ad_value(1361)), scratch.ad_value(1368)), scratch.ad_value(1383)), scratch.ad_value(1385)));
        }

        scratch.values[1669] = if (scratch.values[692] == 0.0) { 1.0 } else { 0.0 };

        if (((scratch.values[1323] != 0.0) && (scratch.values[1340] != 0.0)) && (scratch.values[1669] != 0.0)) {
            scratch.values[1388] = 0.0;
            scratch.node_derivatives[1388] = [0.0; Instance::NODE_COUNT];
            scratch.branch_derivatives[1388] = [0.0; Instance::BRANCH_COUNT];
        }

        scratch.values[1670] = if (scratch.values[459] == 0.5) { 1.0 } else { 0.0 };

        if ((((scratch.values[1323] != 0.0) && (scratch.values[1340] != 0.0)) && (!(scratch.values[1669] != 0.0))) && (scratch.values[1670] != 0.0)) {
            scratch.store_ad(1359, &AdValue::sqrt(AdValue::sub_from_scalar(1.0, AdValue::scale(scratch.ad_value(1351), scratch.values[456]))));
        }

        if ((((scratch.values[1323] != 0.0) && (scratch.values[1340] != 0.0)) && (!(scratch.values[1669] != 0.0))) && (!(scratch.values[1670] != 0.0))) {
            scratch.store_ad(1359, &AdValue::powf(AdValue::sub_from_scalar(1.0, AdValue::scale(scratch.ad_value(1351), scratch.values[456])), scratch.values[459]));
        }

        if (((scratch.values[1323] != 0.0) && (scratch.values[1340] != 0.0)) && (!(scratch.values[1669] != 0.0))) {
            scratch.store_ad(1360, &AdValue::scale(scratch.ad_value(1350), scratch.values[438]));
        }

        scratch.values[1671] = if ((scratch.values[388] == 0.0) && (scratch.values[391] == 0.0)) { 1.0 } else { 0.0 };

        if ((((scratch.values[1323] != 0.0) && (scratch.values[1340] != 0.0)) && (!(scratch.values[1669] != 0.0))) && (scratch.values[1671] != 0.0)) {
            scratch.values[1361] = 0.0;
            scratch.node_derivatives[1361] = [0.0; Instance::NODE_COUNT];
            scratch.branch_derivatives[1361] = [0.0; Instance::BRANCH_COUNT];
        }

        if ((((scratch.values[1323] != 0.0) && (scratch.values[1340] != 0.0)) && (!(scratch.values[1669] != 0.0))) && (!(scratch.values[1671] != 0.0))) {
            scratch.store_ad(1362, &AdValue::sub_from_scalar(scratch.values[444], scratch.ad_value(1356)));
        }

        if ((((scratch.values[1323] != 0.0) && (scratch.values[1340] != 0.0)) && (!(scratch.values[1669] != 0.0))) && (!(scratch.values[1671] != 0.0))) {
            scratch.store_ad(1363, &AdValue::sub_from_scalar(1.0, AdValue::sqrt(AdValue::sub_from_scalar(1.0, AdValue::div(scratch.ad_value(1354), scratch.ad_value(1362))))));
        }

        scratch.values[1672] = if (scratch.values[377] == 0.5) { 1.0 } else { 0.0 };

        if (((((scratch.values[1323] != 0.0) && (scratch.values[1340] != 0.0)) && (!(scratch.values[1669] != 0.0))) && (!(scratch.values[1671] != 0.0))) && (scratch.values[1672] != 0.0)) {
            scratch.values[1364] = 0.0;
            scratch.node_derivatives[1364] = [0.0; Instance::NODE_COUNT];
            scratch.branch_derivatives[1364] = [0.0; Instance::BRANCH_COUNT];
        }

        if (((((scratch.values[1323] != 0.0) && (scratch.values[1340] != 0.0)) && (!(scratch.values[1669] != 0.0))) && (!(scratch.values[1671] != 0.0))) && (!(scratch.values[1672] != 0.0))) {
            scratch.store_ad(1364, &AdValue::scale(AdValue::add(AdValue::div(AdValue::mul(AdValue::square(scratch.ad_value(1363)), AdValue::ln(scratch.ad_value(1363))), AdValue::sub_from_scalar(1.0, scratch.ad_value(1363))), scratch.ad_value(1363)), (1.0 - (2.0 * scratch.values[377]))));
        }

        if ((((scratch.values[1323] != 0.0) && (scratch.values[1340] != 0.0)) && (!(scratch.values[1669] != 0.0))) && (!(scratch.values[1671] != 0.0))) {
            scratch.store_ad(1365, &AdValue::add(scratch.ad_value(1363), scratch.ad_value(1364)));
        }

        scratch.values[1673] = if (scratch.values[377] == 0.5) { 1.0 } else { 0.0 };

        if (((((scratch.values[1323] != 0.0) && (scratch.values[1340] != 0.0)) && (!(scratch.values[1669] != 0.0))) && (!(scratch.values[1671] != 0.0))) && (scratch.values[1673] != 0.0)) {
            scratch.store_ad(1359, &AdValue::sqrt(AdValue::scale(scratch.ad_value(1362), scratch.values[480])));
        }

        if (((((scratch.values[1323] != 0.0) && (scratch.values[1340] != 0.0)) && (!(scratch.values[1669] != 0.0))) && (!(scratch.values[1671] != 0.0))) && (!(scratch.values[1673] != 0.0))) {
            scratch.store_ad(1359, &AdValue::powf(AdValue::scale(scratch.ad_value(1362), scratch.values[480]), scratch.values[377]));
        }

        if ((((scratch.values[1323] != 0.0) && (scratch.values[1340] != 0.0)) && (!(scratch.values[1669] != 0.0))) && (!(scratch.values[1671] != 0.0))) {
            scratch.store_ad(1366, &AdValue::scale(scratch.ad_value(1359), scratch.values[474]));
        }

        if ((((scratch.values[1323] != 0.0) && (scratch.values[1340] != 0.0)) && (!(scratch.values[1669] != 0.0))) && (!(scratch.values[1671] != 0.0))) {
            scratch.store_ad(1367, &AdValue::scale(AdValue::mul(AdValue::offset(scratch.ad_value(1353), (-1.0)), scratch.ad_value(1366)), scratch.values[435]));
        }

        if ((((scratch.values[1323] != 0.0) && (scratch.values[1340] != 0.0)) && (!(scratch.values[1669] != 0.0))) && (!(scratch.values[1671] != 0.0))) {
            scratch.store_ad(1361, &AdValue::scale(AdValue::mul(scratch.ad_value(1367), scratch.ad_value(1365)), scratch.values[388]));
        }

        scratch.values[1674] = if (scratch.values[391] == 0.0) { 1.0 } else { 0.0 };

        if ((((scratch.values[1323] != 0.0) && (scratch.values[1340] != 0.0)) && (!(scratch.values[1669] != 0.0))) && (scratch.values[1674] != 0.0)) {
            scratch.values[1368] = 0.0;
            scratch.node_derivatives[1368] = [0.0; Instance::NODE_COUNT];
            scratch.branch_derivatives[1368] = [0.0; Instance::BRANCH_COUNT];
        }

        if ((((scratch.values[1323] != 0.0) && (scratch.values[1340] != 0.0)) && (!(scratch.values[1669] != 0.0))) && (!(scratch.values[1674] != 0.0))) {
            scratch.store_ad(1369, &AdValue::scale(AdValue::div(AdValue::scale(scratch.ad_value(1366), scratch.values[459]), scratch.ad_value(1362)), scratch.values[489]));
        }

        if ((((scratch.values[1323] != 0.0) && (scratch.values[1340] != 0.0)) && (!(scratch.values[1669] != 0.0))) && (!(scratch.values[1674] != 0.0))) {
            scratch.store_ad(1370, &AdValue::div_from_scalar((0.666666666666667 * scratch.values[486]), scratch.ad_value(1369)));
        }

        if ((((scratch.values[1323] != 0.0) && (scratch.values[1340] != 0.0)) && (!(scratch.values[1669] != 0.0))) && (!(scratch.values[1674] != 0.0))) {
            scratch.store_ad(1371, &AdValue::square(scratch.ad_value(1370)));
        }

        if ((((scratch.values[1323] != 0.0) && (scratch.values[1340] != 0.0)) && (!(scratch.values[1669] != 0.0))) && (!(scratch.values[1674] != 0.0))) {
            scratch.store_ad(1372, &AdValue::sqrt(AdValue::div(AdValue::square(scratch.ad_value(1371)), AdValue::offset(AdValue::square(scratch.ad_value(1371)), 1.0))));
        }

        if ((((scratch.values[1323] != 0.0) && (scratch.values[1340] != 0.0)) && (!(scratch.values[1669] != 0.0))) && (!(scratch.values[1674] != 0.0))) {
            scratch.store_ad(1373, &AdValue::sqrt(AdValue::abs(scratch.ad_value(1372))));
        }

        if ((((scratch.values[1323] != 0.0) && (scratch.values[1340] != 0.0)) && (!(scratch.values[1669] != 0.0))) && (!(scratch.values[1674] != 0.0))) {
            scratch.store_ad(1374, &AdValue::mul(scratch.ad_value(1372), scratch.ad_value(1373)));
        }

    }
}
