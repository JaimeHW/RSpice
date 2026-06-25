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
        if (((((scratch.values[1320] != 0.0) && (scratch.values[1337] != 0.0)) && (!(scratch.values[1548] != 0.0))) && (!(scratch.values[1559] != 0.0))) && (scratch.values[1561] != 0.0)) {
            scratch.store_ad(1356, &AdValue::exp(AdValue::div(AdValue::neg(scratch.ad_value(499)), scratch.ad_value(1381))));
        }

        scratch.values[1562] = if (((-scratch.values[499]) / scratch.values[1381]) < 0.0) { 1.0 } else { 0.0 };

        if ((((((scratch.values[1320] != 0.0) && (scratch.values[1337] != 0.0)) && (!(scratch.values[1548] != 0.0))) && (!(scratch.values[1559] != 0.0))) && (!(scratch.values[1561] != 0.0))) && (scratch.values[1562] != 0.0)) {
            let assign20350_ad_e21796: AdValue = AdValue::offset(AdValue::mul(AdValue::sub_from_scalar((-230.25850929940458), AdValue::div(AdValue::neg(scratch.ad_value(499)), scratch.ad_value(1381))), AdValue::offset(AdValue::scale(AdValue::mul(AdValue::sub_from_scalar((-230.25850929940458), AdValue::div(AdValue::neg(scratch.ad_value(499)), scratch.ad_value(1381))), AdValue::offset(AdValue::scale(AdValue::sub_from_scalar((-230.25850929940458), AdValue::div(AdValue::neg(scratch.ad_value(499)), scratch.ad_value(1381))), 0.3333333333333333), 1.0)), 0.5), 1.0)), 1.0);
            scratch.store_ad(1356, &AdValue::div_from_scalar(1e-100, assign20350_ad_e21796));
        }

        if ((((((scratch.values[1320] != 0.0) && (scratch.values[1337] != 0.0)) && (!(scratch.values[1548] != 0.0))) && (!(scratch.values[1559] != 0.0))) && (!(scratch.values[1561] != 0.0))) && (!(scratch.values[1562] != 0.0))) {
            let assign20360_ad_e21846: AdValue = AdValue::scale(AdValue::offset(AdValue::mul(AdValue::offset(AdValue::div(AdValue::neg(scratch.ad_value(499)), scratch.ad_value(1381)), (-230.25850929940458)), AdValue::offset(AdValue::scale(AdValue::mul(AdValue::offset(AdValue::div(AdValue::neg(scratch.ad_value(499)), scratch.ad_value(1381)), (-230.25850929940458)), AdValue::offset(AdValue::scale(AdValue::offset(AdValue::div(AdValue::neg(scratch.ad_value(499)), scratch.ad_value(1381)), (-230.25850929940458)), 0.3333333333333333), 1.0)), 0.5), 1.0)), 1.0), 1e100);
            scratch.store_ad(1356, &assign20360_ad_e21846);
        }

        if ((((scratch.values[1320] != 0.0) && (scratch.values[1337] != 0.0)) && (!(scratch.values[1548] != 0.0))) && (!(scratch.values[1559] != 0.0))) {
            scratch.store_ad(1380, &AdValue::scale(AdValue::mul(AdValue::mul(AdValue::mul(scratch.ad_value(525), scratch.ad_value(1381)), scratch.ad_value(1381)), scratch.ad_value(1356)), scratch.values[404]));
        }

        scratch.values[1563] = if (scratch.values[413] > 1000.0) { 1.0 } else { 0.0 };

        if ((((scratch.values[1320] != 0.0) && (scratch.values[1337] != 0.0)) && (!(scratch.values[1548] != 0.0))) && (scratch.values[1563] != 0.0)) {
            scratch.values[1382] = 1.0;
            scratch.node_derivatives[1382] = [0.0; Instance::NODE_COUNT];
            scratch.branch_derivatives[1382] = [0.0; Instance::BRANCH_COUNT];
        }

        scratch.values[1564] = if (scratch.values[1355] > ((-scratch.values[500]) * scratch.values[413])) { 1.0 } else { 0.0 };

        scratch.values[1565] = if (scratch.values[416] == 4.0) { 1.0 } else { 0.0 };

        if ((((((scratch.values[1320] != 0.0) && (scratch.values[1337] != 0.0)) && (!(scratch.values[1548] != 0.0))) && (!(scratch.values[1563] != 0.0))) && (scratch.values[1564] != 0.0)) && (scratch.values[1565] != 0.0)) {
            scratch.store_ad(1356, &AdValue::mul(AdValue::mul(AdValue::mul(AdValue::scale(scratch.ad_value(1355), scratch.values[506]), AdValue::scale(scratch.ad_value(1355), scratch.values[506])), AdValue::scale(scratch.ad_value(1355), scratch.values[506])), AdValue::scale(scratch.ad_value(1355), scratch.values[506])));
        }

        if ((((((scratch.values[1320] != 0.0) && (scratch.values[1337] != 0.0)) && (!(scratch.values[1548] != 0.0))) && (!(scratch.values[1563] != 0.0))) && (scratch.values[1564] != 0.0)) && (!(scratch.values[1565] != 0.0))) {
            scratch.store_ad(1356, &AdValue::powf(AdValue::abs(AdValue::scale(scratch.ad_value(1355), scratch.values[506])), scratch.values[416]));
        }

        if (((((scratch.values[1320] != 0.0) && (scratch.values[1337] != 0.0)) && (!(scratch.values[1548] != 0.0))) && (!(scratch.values[1563] != 0.0))) && (scratch.values[1564] != 0.0)) {
            scratch.store_ad(1382, &AdValue::div_from_scalar(1.0, AdValue::sub_from_scalar(1.0, scratch.ad_value(1356))));
        }

        if (((((scratch.values[1320] != 0.0) && (scratch.values[1337] != 0.0)) && (!(scratch.values[1548] != 0.0))) && (!(scratch.values[1563] != 0.0))) && (!(scratch.values[1564] != 0.0))) {
            scratch.store_ad(1382, &AdValue::offset(AdValue::scale(AdValue::offset(scratch.ad_value(1355), (scratch.values[500] * scratch.values[413])), scratch.values[509]), scratch.values[503]));
        }

        if (((scratch.values[1320] != 0.0) && (scratch.values[1337] != 0.0)) && (!(scratch.values[1548] != 0.0))) {
            scratch.store_ad(1385, &AdValue::mul(AdValue::add(AdValue::add(AdValue::add(scratch.ad_value(1357), scratch.ad_value(1358)), scratch.ad_value(1365)), scratch.ad_value(1380)), scratch.ad_value(1382)));
        }

        if ((scratch.values[1320] != 0.0) && (scratch.values[1337] != 0.0)) {
            scratch.store_ad(515, &AdValue::add(AdValue::add(AdValue::mul(scratch.ad_value(697), scratch.ad_value(1383)), AdValue::mul(scratch.ad_value(698), scratch.ad_value(1384))), AdValue::mul(scratch.ad_value(699), scratch.ad_value(1385))));
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

        scratch.values[1566] = if !(((scratch.values[697] == 0.0) && (scratch.values[698] == 0.0)) && (scratch.values[699] == 0.0)) { 1.0 } else { 0.0 };

        if (((scratch.values[1320] != 0.0) && (scratch.values[1337] != 0.0)) && (scratch.values[1566] != 0.0)) {
            scratch.store_ad(1341, &AdValue::mul(AdValue::scale(scratch.ad_value(708), 4.0), scratch.ad_value(708)));
        }

        if (((scratch.values[1320] != 0.0) && (scratch.values[1337] != 0.0)) && (scratch.values[1566] != 0.0)) {
            scratch.store_ad(1342, &AdValue::div(scratch.ad_value(708), scratch.ad_value(709)));
        }

        if (((scratch.values[1320] != 0.0) && (scratch.values[1337] != 0.0)) && (scratch.values[1566] != 0.0)) {
            scratch.store_ad(1343, &AdValue::add(scratch.ad_value(526), AdValue::mul(scratch.ad_value(708), scratch.ad_value(1342))));
        }

        if (((scratch.values[1320] != 0.0) && (scratch.values[1337] != 0.0)) && (scratch.values[1566] != 0.0)) {
            scratch.store_ad(1344, &AdValue::add(scratch.ad_value(709), scratch.ad_value(1343)));
        }

        if (((scratch.values[1320] != 0.0) && (scratch.values[1337] != 0.0)) && (scratch.values[1566] != 0.0)) {
            scratch.store_ad(1345, &AdValue::sub(scratch.ad_value(709), scratch.ad_value(1343)));
        }

        if (((scratch.values[1320] != 0.0) && (scratch.values[1337] != 0.0)) && (scratch.values[1566] != 0.0)) {
            scratch.store_ad(1346, &AdValue::sqrt(AdValue::add(AdValue::square(scratch.ad_value(1345)), scratch.ad_value(1341))));
        }

        if (((scratch.values[1320] != 0.0) && (scratch.values[1337] != 0.0)) && (scratch.values[1566] != 0.0)) {
            scratch.store_ad(1348, &AdValue::scale(AdValue::div(AdValue::mul(scratch.ad_value(526), scratch.ad_value(709)), AdValue::add(scratch.ad_value(1344), scratch.ad_value(1346))), 2.0));
        }

        scratch.values[1567] = if (scratch.values[526] < scratch.values[705]) { 1.0 } else { 0.0 };

        scratch.values[1568] = if ((((0.5 * (scratch.values[526] * scratch.values[427]))) as f64).abs() < 230.25850929940458) { 1.0 } else { 0.0 };

        if (((((scratch.values[1320] != 0.0) && (scratch.values[1337] != 0.0)) && (scratch.values[1566] != 0.0)) && (scratch.values[1567] != 0.0)) && (scratch.values[1568] != 0.0)) {
            scratch.store_ad(1350, &AdValue::exp(AdValue::scale(scratch.ad_value(526), (scratch.values[427] * 0.5))));
        }

        scratch.values[1569] = if ((0.5 * (scratch.values[526] * scratch.values[427])) < 0.0) { 1.0 } else { 0.0 };

        if ((((((scratch.values[1320] != 0.0) && (scratch.values[1337] != 0.0)) && (scratch.values[1566] != 0.0)) && (scratch.values[1567] != 0.0)) && (!(scratch.values[1568] != 0.0))) && (scratch.values[1569] != 0.0)) {
            let assign20620_ad_e22209: AdValue = AdValue::div_from_scalar(1e-100, AdValue::offset(AdValue::mul(AdValue::sub_from_scalar((-230.25850929940458), AdValue::scale(scratch.ad_value(526), (scratch.values[427] * 0.5))), AdValue::offset(AdValue::scale(AdValue::mul(AdValue::sub_from_scalar((-230.25850929940458), AdValue::scale(scratch.ad_value(526), (scratch.values[427] * 0.5))), AdValue::offset(AdValue::scale(AdValue::sub_from_scalar((-230.25850929940458), AdValue::scale(scratch.ad_value(526), (scratch.values[427] * 0.5))), 0.3333333333333333), 1.0)), 0.5), 1.0)), 1.0));
            scratch.store_ad(1350, &assign20620_ad_e22209);
        }

        if ((((((scratch.values[1320] != 0.0) && (scratch.values[1337] != 0.0)) && (scratch.values[1566] != 0.0)) && (scratch.values[1567] != 0.0)) && (!(scratch.values[1568] != 0.0))) && (!(scratch.values[1569] != 0.0))) {
            scratch.store_ad(1350, &AdValue::scale(AdValue::offset(AdValue::mul(AdValue::offset(AdValue::scale(scratch.ad_value(526), (scratch.values[427] * 0.5)), (-230.25850929940458)), AdValue::offset(AdValue::scale(AdValue::mul(AdValue::offset(AdValue::scale(scratch.ad_value(526), (scratch.values[427] * 0.5)), (-230.25850929940458)), AdValue::offset(AdValue::scale(AdValue::offset(AdValue::scale(scratch.ad_value(526), (scratch.values[427] * 0.5)), (-230.25850929940458)), 0.3333333333333333), 1.0)), 0.5), 1.0)), 1.0), 1e100));
        }

        if ((((scratch.values[1320] != 0.0) && (scratch.values[1337] != 0.0)) && (scratch.values[1566] != 0.0)) && (scratch.values[1567] != 0.0)) {
            scratch.store_ad(1347, &AdValue::square(scratch.ad_value(1350)));
        }

        if ((((scratch.values[1320] != 0.0) && (scratch.values[1337] != 0.0)) && (scratch.values[1566] != 0.0)) && (!(scratch.values[1567] != 0.0))) {
            scratch.store_ad(1347, &AdValue::mul(AdValue::offset(AdValue::scale(AdValue::sub(scratch.ad_value(526), scratch.ad_value(705)), scratch.values[427]), 1.0), scratch.ad_value(706)));
        }

        if ((((scratch.values[1320] != 0.0) && (scratch.values[1337] != 0.0)) && (scratch.values[1566] != 0.0)) && (!(scratch.values[1567] != 0.0))) {
            scratch.store_ad(1350, &AdValue::sqrt(scratch.ad_value(1347)));
        }

        if (((scratch.values[1320] != 0.0) && (scratch.values[1337] != 0.0)) && (scratch.values[1566] != 0.0)) {
            scratch.store_ad(1347, &AdValue::offset(scratch.ad_value(1347), (-1.0)));
        }

        if (((scratch.values[1320] != 0.0) && (scratch.values[1337] != 0.0)) && (scratch.values[1566] != 0.0)) {
            scratch.store_ad(1349, &AdValue::div_from_scalar(1.0, scratch.ad_value(1350)));
        }

        scratch.values[1570] = if (scratch.values[526] > 0.0) { 1.0 } else { 0.0 };

        if ((((scratch.values[1320] != 0.0) && (scratch.values[1337] != 0.0)) && (scratch.values[1566] != 0.0)) && (scratch.values[1570] != 0.0)) {
            scratch.store_ad(1351, &AdValue::scale(AdValue::ln(AdValue::add(AdValue::offset(scratch.ad_value(1349), 2.0), AdValue::sqrt(AdValue::mul(AdValue::offset(scratch.ad_value(1349), 1.0), AdValue::offset(scratch.ad_value(1349), 3.0))))), (scratch.values[426] * 2.0)));
        }

        if ((((scratch.values[1320] != 0.0) && (scratch.values[1337] != 0.0)) && (scratch.values[1566] != 0.0)) && (!(scratch.values[1570] != 0.0))) {
            scratch.store_ad(1351, &AdValue::sub(AdValue::scale(AdValue::ln(AdValue::add(AdValue::offset(AdValue::scale(scratch.ad_value(1350), 2.0), 1.0), AdValue::sqrt(AdValue::mul(AdValue::offset(scratch.ad_value(1350), 1.0), AdValue::offset(AdValue::scale(scratch.ad_value(1350), 3.0), 1.0))))), (scratch.values[426] * 2.0)), scratch.ad_value(526)));
        }

        if (((scratch.values[1320] != 0.0) && (scratch.values[1337] != 0.0)) && (scratch.values[1566] != 0.0)) {
            scratch.store_ad(1352, &AdValue::sub(scratch.ad_value(707), scratch.ad_value(1351)));
        }

        if (((scratch.values[1320] != 0.0) && (scratch.values[1337] != 0.0)) && (scratch.values[1566] != 0.0)) {
            scratch.store_ad(1353, &AdValue::scale(AdValue::sub(AdValue::add(scratch.ad_value(526), scratch.ad_value(1352)), AdValue::sqrt(AdValue::offset(AdValue::mul(AdValue::sub(scratch.ad_value(526), scratch.ad_value(1352)), AdValue::sub(scratch.ad_value(526), scratch.ad_value(1352))), ((4.0 * scratch.values[426]) * scratch.values[426])))), 0.5));
        }

        if (((scratch.values[1320] != 0.0) && (scratch.values[1337] != 0.0)) && (scratch.values[1566] != 0.0)) {
            scratch.store_ad(1354, &AdValue::scale(AdValue::sub(AdValue::add(scratch.ad_value(526), scratch.ad_value(710)), AdValue::sqrt(AdValue::offset(AdValue::mul(AdValue::sub(scratch.ad_value(526), scratch.ad_value(710)), AdValue::sub(scratch.ad_value(526), scratch.ad_value(710))), ((4.0 * scratch.values[424]) * scratch.values[424])))), 0.5));
        }

        if (((scratch.values[1320] != 0.0) && (scratch.values[1337] != 0.0)) && (scratch.values[1566] != 0.0)) {
            scratch.store_ad(1355, &AdValue::scale(AdValue::sub(scratch.ad_value(526), AdValue::sqrt(AdValue::offset(AdValue::mul(scratch.ad_value(526), scratch.ad_value(526)), ((4.0 * 1e-6) * 1e-6)))), 0.5));
        }

        scratch.values[1571] = if (scratch.values[697] == 0.0) { 1.0 } else { 0.0 };

        if (((scratch.values[1320] != 0.0) && (scratch.values[1337] != 0.0)) && (scratch.values[1571] != 0.0)) {
            scratch.values[1383] = 0.0;
            scratch.node_derivatives[1383] = [0.0; Instance::NODE_COUNT];
            scratch.branch_derivatives[1383] = [0.0; Instance::BRANCH_COUNT];
        }

        scratch.values[1572] = if (scratch.values[464] == 0.5) { 1.0 } else { 0.0 };

        if ((((scratch.values[1320] != 0.0) && (scratch.values[1337] != 0.0)) && (!(scratch.values[1571] != 0.0))) && (scratch.values[1572] != 0.0)) {
            scratch.store_ad(1356, &AdValue::sqrt(AdValue::sub_from_scalar(1.0, AdValue::scale(scratch.ad_value(1348), scratch.values[461]))));
        }

        if ((((scratch.values[1320] != 0.0) && (scratch.values[1337] != 0.0)) && (!(scratch.values[1571] != 0.0))) && (!(scratch.values[1572] != 0.0))) {
            scratch.store_ad(1356, &AdValue::powf(AdValue::sub_from_scalar(1.0, AdValue::scale(scratch.ad_value(1348), scratch.values[461])), scratch.values[464]));
        }

        if (((scratch.values[1320] != 0.0) && (scratch.values[1337] != 0.0)) && (!(scratch.values[1571] != 0.0))) {
            scratch.store_ad(1357, &AdValue::scale(scratch.ad_value(1347), scratch.values[443]));
        }

        scratch.values[1573] = if ((scratch.values[393] == 0.0) && (scratch.values[396] == 0.0)) { 1.0 } else { 0.0 };

        if ((((scratch.values[1320] != 0.0) && (scratch.values[1337] != 0.0)) && (!(scratch.values[1571] != 0.0))) && (scratch.values[1573] != 0.0)) {
            scratch.values[1358] = 0.0;
            scratch.node_derivatives[1358] = [0.0; Instance::NODE_COUNT];
            scratch.branch_derivatives[1358] = [0.0; Instance::BRANCH_COUNT];
        }

        if ((((scratch.values[1320] != 0.0) && (scratch.values[1337] != 0.0)) && (!(scratch.values[1571] != 0.0))) && (!(scratch.values[1573] != 0.0))) {
            scratch.store_ad(1359, &AdValue::sub_from_scalar(scratch.values[449], scratch.ad_value(1353)));
        }

        if ((((scratch.values[1320] != 0.0) && (scratch.values[1337] != 0.0)) && (!(scratch.values[1571] != 0.0))) && (!(scratch.values[1573] != 0.0))) {
            scratch.store_ad(1360, &AdValue::sub_from_scalar(1.0, AdValue::sqrt(AdValue::sub_from_scalar(1.0, AdValue::div(scratch.ad_value(1351), scratch.ad_value(1359))))));
        }

        scratch.values[1574] = if (scratch.values[382] == 0.5) { 1.0 } else { 0.0 };

        if (((((scratch.values[1320] != 0.0) && (scratch.values[1337] != 0.0)) && (!(scratch.values[1571] != 0.0))) && (!(scratch.values[1573] != 0.0))) && (scratch.values[1574] != 0.0)) {
            scratch.values[1361] = 0.0;
            scratch.node_derivatives[1361] = [0.0; Instance::NODE_COUNT];
            scratch.branch_derivatives[1361] = [0.0; Instance::BRANCH_COUNT];
        }

        if (((((scratch.values[1320] != 0.0) && (scratch.values[1337] != 0.0)) && (!(scratch.values[1571] != 0.0))) && (!(scratch.values[1573] != 0.0))) && (!(scratch.values[1574] != 0.0))) {
            scratch.store_ad(1361, &AdValue::scale(AdValue::add(AdValue::div(AdValue::mul(AdValue::square(scratch.ad_value(1360)), AdValue::ln(scratch.ad_value(1360))), AdValue::sub_from_scalar(1.0, scratch.ad_value(1360))), scratch.ad_value(1360)), (1.0 - (2.0 * scratch.values[382]))));
        }

        if ((((scratch.values[1320] != 0.0) && (scratch.values[1337] != 0.0)) && (!(scratch.values[1571] != 0.0))) && (!(scratch.values[1573] != 0.0))) {
            scratch.store_ad(1362, &AdValue::add(scratch.ad_value(1360), scratch.ad_value(1361)));
        }

        scratch.values[1575] = if (scratch.values[382] == 0.5) { 1.0 } else { 0.0 };

        if (((((scratch.values[1320] != 0.0) && (scratch.values[1337] != 0.0)) && (!(scratch.values[1571] != 0.0))) && (!(scratch.values[1573] != 0.0))) && (scratch.values[1575] != 0.0)) {
            scratch.store_ad(1356, &AdValue::sqrt(AdValue::scale(scratch.ad_value(1359), scratch.values[485])));
        }

        if (((((scratch.values[1320] != 0.0) && (scratch.values[1337] != 0.0)) && (!(scratch.values[1571] != 0.0))) && (!(scratch.values[1573] != 0.0))) && (!(scratch.values[1575] != 0.0))) {
            scratch.store_ad(1356, &AdValue::powf(AdValue::scale(scratch.ad_value(1359), scratch.values[485]), scratch.values[382]));
        }

        if ((((scratch.values[1320] != 0.0) && (scratch.values[1337] != 0.0)) && (!(scratch.values[1571] != 0.0))) && (!(scratch.values[1573] != 0.0))) {
            scratch.store_ad(1363, &AdValue::scale(scratch.ad_value(1356), scratch.values[479]));
        }

        if ((((scratch.values[1320] != 0.0) && (scratch.values[1337] != 0.0)) && (!(scratch.values[1571] != 0.0))) && (!(scratch.values[1573] != 0.0))) {
            scratch.store_ad(1364, &AdValue::scale(AdValue::mul(AdValue::offset(scratch.ad_value(1350), (-1.0)), scratch.ad_value(1363)), scratch.values[440]));
        }

        if ((((scratch.values[1320] != 0.0) && (scratch.values[1337] != 0.0)) && (!(scratch.values[1571] != 0.0))) && (!(scratch.values[1573] != 0.0))) {
            scratch.store_ad(1358, &AdValue::scale(AdValue::mul(scratch.ad_value(1364), scratch.ad_value(1362)), scratch.values[393]));
        }

        scratch.values[1576] = if (scratch.values[396] == 0.0) { 1.0 } else { 0.0 };

        if ((((scratch.values[1320] != 0.0) && (scratch.values[1337] != 0.0)) && (!(scratch.values[1571] != 0.0))) && (scratch.values[1576] != 0.0)) {
            scratch.values[1365] = 0.0;
            scratch.node_derivatives[1365] = [0.0; Instance::NODE_COUNT];
            scratch.branch_derivatives[1365] = [0.0; Instance::BRANCH_COUNT];
        }

        if ((((scratch.values[1320] != 0.0) && (scratch.values[1337] != 0.0)) && (!(scratch.values[1571] != 0.0))) && (!(scratch.values[1576] != 0.0))) {
            scratch.store_ad(1366, &AdValue::scale(AdValue::div(AdValue::scale(scratch.ad_value(1363), scratch.values[464]), scratch.ad_value(1359)), scratch.values[494]));
        }

        if ((((scratch.values[1320] != 0.0) && (scratch.values[1337] != 0.0)) && (!(scratch.values[1571] != 0.0))) && (!(scratch.values[1576] != 0.0))) {
            scratch.store_ad(1367, &AdValue::div_from_scalar((0.666666666666667 * scratch.values[491]), scratch.ad_value(1366)));
        }

        if ((((scratch.values[1320] != 0.0) && (scratch.values[1337] != 0.0)) && (!(scratch.values[1571] != 0.0))) && (!(scratch.values[1576] != 0.0))) {
            scratch.store_ad(1368, &AdValue::square(scratch.ad_value(1367)));
        }

        if ((((scratch.values[1320] != 0.0) && (scratch.values[1337] != 0.0)) && (!(scratch.values[1571] != 0.0))) && (!(scratch.values[1576] != 0.0))) {
            scratch.store_ad(1369, &AdValue::sqrt(AdValue::div(AdValue::square(scratch.ad_value(1368)), AdValue::offset(AdValue::square(scratch.ad_value(1368)), 1.0))));
        }

        if ((((scratch.values[1320] != 0.0) && (scratch.values[1337] != 0.0)) && (!(scratch.values[1571] != 0.0))) && (!(scratch.values[1576] != 0.0))) {
            scratch.store_ad(1370, &AdValue::sqrt(AdValue::abs(scratch.ad_value(1369))));
        }

        if ((((scratch.values[1320] != 0.0) && (scratch.values[1337] != 0.0)) && (!(scratch.values[1571] != 0.0))) && (!(scratch.values[1576] != 0.0))) {
            scratch.store_ad(1371, &AdValue::mul(scratch.ad_value(1369), scratch.ad_value(1370)));
        }

        scratch.values[1577] = if (((-scratch.values[382]) * scratch.values[467]) == (-1.0)) { 1.0 } else { 0.0 };

        if (((((scratch.values[1320] != 0.0) && (scratch.values[1337] != 0.0)) && (!(scratch.values[1571] != 0.0))) && (!(scratch.values[1576] != 0.0))) && (scratch.values[1577] != 0.0)) {
            scratch.store_ad(1372, &AdValue::div_from_scalar(1.0, AdValue::offset(AdValue::mul(scratch.ad_value(1366), scratch.ad_value(1371)), 1.0)));
        }

        if (((((scratch.values[1320] != 0.0) && (scratch.values[1337] != 0.0)) && (!(scratch.values[1571] != 0.0))) && (!(scratch.values[1576] != 0.0))) && (!(scratch.values[1577] != 0.0))) {
            scratch.store_ad(1372, &AdValue::powf(AdValue::offset(AdValue::mul(scratch.ad_value(1366), scratch.ad_value(1371)), 1.0), ((-scratch.values[382]) * scratch.values[467])));
        }

        if ((((scratch.values[1320] != 0.0) && (scratch.values[1337] != 0.0)) && (!(scratch.values[1571] != 0.0))) && (!(scratch.values[1576] != 0.0))) {
            scratch.store_ad(1373, &AdValue::div(AdValue::mul(scratch.ad_value(1362), scratch.ad_value(1372)), AdValue::add(scratch.ad_value(1362), scratch.ad_value(1372))));
        }

        if ((((scratch.values[1320] != 0.0) && (scratch.values[1337] != 0.0)) && (!(scratch.values[1571] != 0.0))) && (!(scratch.values[1576] != 0.0))) {
            scratch.store_ad(1374, &AdValue::sqrt(AdValue::scale(AdValue::div(scratch.ad_value(1366), scratch.ad_value(1370)), 0.375)));
        }

        if ((((scratch.values[1320] != 0.0) && (scratch.values[1337] != 0.0)) && (!(scratch.values[1571] != 0.0))) && (!(scratch.values[1576] != 0.0))) {
            scratch.store_ad(1375, &AdValue::sub(AdValue::scale(AdValue::mul(scratch.ad_value(1367), scratch.ad_value(1370)), 2.0), scratch.ad_value(1369)));
        }

        if ((((scratch.values[1320] != 0.0) && (scratch.values[1337] != 0.0)) && (!(scratch.values[1571] != 0.0))) && (!(scratch.values[1576] != 0.0))) {
            scratch.store_ad(1376, &AdValue::add(AdValue::sub(AdValue::mul(AdValue::scale(scratch.ad_value(1367), scratch.values[491]), scratch.ad_value(1370)), AdValue::scale(scratch.ad_value(1369), scratch.values[491])), AdValue::scale(AdValue::mul(scratch.ad_value(1366), scratch.ad_value(1371)), 0.5)));
        }

        if ((((scratch.values[1320] != 0.0) && (scratch.values[1337] != 0.0)) && (!(scratch.values[1571] != 0.0))) && (!(scratch.values[1576] != 0.0))) {
            scratch.store_ad(1377, &AdValue::mul(AdValue::offset(scratch.ad_value(1375), (-1.0)), scratch.ad_value(1374)));
        }

        if ((((scratch.values[1320] != 0.0) && (scratch.values[1337] != 0.0)) && (!(scratch.values[1571] != 0.0))) && (!(scratch.values[1576] != 0.0))) {
            scratch.store_ad(1338, &AdValue::square(scratch.ad_value(1377)));
        }

        scratch.values[1578] = if (scratch.values[1377] > 0.0) { 1.0 } else { 0.0 };

        if (((((scratch.values[1320] != 0.0) && (scratch.values[1337] != 0.0)) && (!(scratch.values[1571] != 0.0))) && (!(scratch.values[1576] != 0.0))) && (scratch.values[1578] != 0.0)) {
            scratch.store_ad(1339, &AdValue::div_from_scalar(1.0, AdValue::offset(AdValue::scale(scratch.ad_value(1377), scratch.values[428]), 1.0)));
        }

        if (((((scratch.values[1320] != 0.0) && (scratch.values[1337] != 0.0)) && (!(scratch.values[1571] != 0.0))) && (!(scratch.values[1576] != 0.0))) && (!(scratch.values[1578] != 0.0))) {
            scratch.store_ad(1339, &AdValue::div_from_scalar(1.0, AdValue::sub_from_scalar(1.0, AdValue::scale(scratch.ad_value(1377), scratch.values[428]))));
        }

        scratch.values[1579] = if (((-scratch.values[1338]) + scratch.values[1376]) > (-230.25850929940458)) { 1.0 } else { 0.0 };

        if (((((scratch.values[1320] != 0.0) && (scratch.values[1337] != 0.0)) && (!(scratch.values[1571] != 0.0))) && (!(scratch.values[1576] != 0.0))) && (scratch.values[1579] != 0.0)) {
            scratch.store_ad(1356, &AdValue::exp(AdValue::sub(scratch.ad_value(1376), scratch.ad_value(1338))));
        }

        if (((((scratch.values[1320] != 0.0) && (scratch.values[1337] != 0.0)) && (!(scratch.values[1571] != 0.0))) && (!(scratch.values[1576] != 0.0))) && (!(scratch.values[1579] != 0.0))) {
            let assign21200_ad_e23152: AdValue = AdValue::div_from_scalar(1e-100, AdValue::offset(AdValue::mul(AdValue::sub_from_scalar((-230.25850929940458), AdValue::sub(scratch.ad_value(1376), scratch.ad_value(1338))), AdValue::offset(AdValue::scale(AdValue::mul(AdValue::sub_from_scalar((-230.25850929940458), AdValue::sub(scratch.ad_value(1376), scratch.ad_value(1338))), AdValue::offset(AdValue::scale(AdValue::sub_from_scalar((-230.25850929940458), AdValue::sub(scratch.ad_value(1376), scratch.ad_value(1338))), 0.3333333333333333), 1.0)), 0.5), 1.0)), 1.0));
            scratch.store_ad(1356, &assign21200_ad_e23152);
        }

        if ((((scratch.values[1320] != 0.0) && (scratch.values[1337] != 0.0)) && (!(scratch.values[1571] != 0.0))) && (!(scratch.values[1576] != 0.0))) {
            scratch.store_ad(1340, &AdValue::mul(AdValue::add(AdValue::add(AdValue::scale(scratch.ad_value(1339), 0.29214664), AdValue::scale(AdValue::square(scratch.ad_value(1339)), scratch.values[429])), AdValue::scale(AdValue::mul(AdValue::square(scratch.ad_value(1339)), scratch.ad_value(1339)), scratch.values[430])), scratch.ad_value(1356)));
        }

        scratch.values[1580] = if (scratch.values[1377] > 0.0) { 1.0 } else { 0.0 };

        if (((((scratch.values[1320] != 0.0) && (scratch.values[1337] != 0.0)) && (!(scratch.values[1571] != 0.0))) && (!(scratch.values[1576] != 0.0))) && (scratch.values[1580] != 0.0)) {
            scratch.values[1378] = scratch.values[1340];
            scratch.node_derivatives[1378] = scratch.node_derivatives[1340];
            scratch.branch_derivatives[1378] = scratch.branch_derivatives[1340];
        }

        scratch.values[1581] = if (scratch.values[1376] > (-230.25850929940458)) { 1.0 } else { 0.0 };

        if ((((((scratch.values[1320] != 0.0) && (scratch.values[1337] != 0.0)) && (!(scratch.values[1571] != 0.0))) && (!(scratch.values[1576] != 0.0))) && (!(scratch.values[1580] != 0.0))) && (scratch.values[1581] != 0.0)) {
            scratch.store_ad(1356, &AdValue::exp(scratch.ad_value(1376)));
        }

        if ((((((scratch.values[1320] != 0.0) && (scratch.values[1337] != 0.0)) && (!(scratch.values[1571] != 0.0))) && (!(scratch.values[1576] != 0.0))) && (!(scratch.values[1580] != 0.0))) && (!(scratch.values[1581] != 0.0))) {
            scratch.store_ad(1356, &AdValue::div_from_scalar(1e-100, AdValue::offset(AdValue::mul(AdValue::sub_from_scalar((-230.25850929940458), scratch.ad_value(1376)), AdValue::offset(AdValue::scale(AdValue::mul(AdValue::sub_from_scalar((-230.25850929940458), scratch.ad_value(1376)), AdValue::offset(AdValue::scale(AdValue::sub_from_scalar((-230.25850929940458), scratch.ad_value(1376)), 0.3333333333333333), 1.0)), 0.5), 1.0)), 1.0)));
        }

        if (((((scratch.values[1320] != 0.0) && (scratch.values[1337] != 0.0)) && (!(scratch.values[1571] != 0.0))) && (!(scratch.values[1576] != 0.0))) && (!(scratch.values[1580] != 0.0))) {
            scratch.store_ad(1378, &AdValue::sub(AdValue::scale(scratch.ad_value(1356), 2.0), scratch.ad_value(1340)));
        }

        if ((((scratch.values[1320] != 0.0) && (scratch.values[1337] != 0.0)) && (!(scratch.values[1571] != 0.0))) && (!(scratch.values[1576] != 0.0))) {
            scratch.store_ad(1379, &AdValue::scale(AdValue::div(AdValue::scale(scratch.ad_value(1378), scratch.values[491]), scratch.ad_value(1374)), (1.772453850905516 * 0.5)));
        }

        if ((((scratch.values[1320] != 0.0) && (scratch.values[1337] != 0.0)) && (!(scratch.values[1571] != 0.0))) && (!(scratch.values[1576] != 0.0))) {
            scratch.store_ad(1365, &AdValue::scale(AdValue::mul(AdValue::mul(scratch.ad_value(1364), scratch.ad_value(1379)), scratch.ad_value(1373)), scratch.values[396]));
        }

        scratch.values[1582] = if (scratch.values[402] == 0.0) { 1.0 } else { 0.0 };

        if ((((scratch.values[1320] != 0.0) && (scratch.values[1337] != 0.0)) && (!(scratch.values[1571] != 0.0))) && (scratch.values[1582] != 0.0)) {
            scratch.values[1380] = 0.0;
            scratch.node_derivatives[1380] = [0.0; Instance::NODE_COUNT];
            scratch.branch_derivatives[1380] = [0.0; Instance::BRANCH_COUNT];
        }

        scratch.values[1583] = if (scratch.values[382] == 0.5) { 1.0 } else { 0.0 };

        if (((((scratch.values[1320] != 0.0) && (scratch.values[1337] != 0.0)) && (!(scratch.values[1571] != 0.0))) && (!(scratch.values[1582] != 0.0))) && (scratch.values[1583] != 0.0)) {
            scratch.store_ad(1356, &AdValue::sqrt(AdValue::scale(AdValue::sub_from_scalar(scratch.values[379], scratch.ad_value(1354)), scratch.values[485])));
        }

        if (((((scratch.values[1320] != 0.0) && (scratch.values[1337] != 0.0)) && (!(scratch.values[1571] != 0.0))) && (!(scratch.values[1582] != 0.0))) && (!(scratch.values[1583] != 0.0))) {
            scratch.store_ad(1356, &AdValue::powf(AdValue::scale(AdValue::sub_from_scalar(scratch.values[379], scratch.ad_value(1354)), scratch.values[485]), scratch.values[382]));
        }

        if ((((scratch.values[1320] != 0.0) && (scratch.values[1337] != 0.0)) && (!(scratch.values[1571] != 0.0))) && (!(scratch.values[1582] != 0.0))) {
            scratch.store_ad(1381, &AdValue::scale(AdValue::div(AdValue::scale(AdValue::sub_from_scalar(scratch.values[379], scratch.ad_value(1354)), scratch.values[482]), scratch.ad_value(1356)), scratch.values[467]));
        }

        scratch.values[1584] = if (((((-scratch.values[497]) / scratch.values[1381])) as f64).abs() < 230.25850929940458) { 1.0 } else { 0.0 };

        if (((((scratch.values[1320] != 0.0) && (scratch.values[1337] != 0.0)) && (!(scratch.values[1571] != 0.0))) && (!(scratch.values[1582] != 0.0))) && (scratch.values[1584] != 0.0)) {
            scratch.store_ad(1356, &AdValue::exp(AdValue::div(AdValue::neg(scratch.ad_value(497)), scratch.ad_value(1381))));
        }

        scratch.values[1585] = if (((-scratch.values[497]) / scratch.values[1381]) < 0.0) { 1.0 } else { 0.0 };

        if ((((((scratch.values[1320] != 0.0) && (scratch.values[1337] != 0.0)) && (!(scratch.values[1571] != 0.0))) && (!(scratch.values[1582] != 0.0))) && (!(scratch.values[1584] != 0.0))) && (scratch.values[1585] != 0.0)) {
            let assign21390_ad_e23479: AdValue = AdValue::offset(AdValue::mul(AdValue::sub_from_scalar((-230.25850929940458), AdValue::div(AdValue::neg(scratch.ad_value(497)), scratch.ad_value(1381))), AdValue::offset(AdValue::scale(AdValue::mul(AdValue::sub_from_scalar((-230.25850929940458), AdValue::div(AdValue::neg(scratch.ad_value(497)), scratch.ad_value(1381))), AdValue::offset(AdValue::scale(AdValue::sub_from_scalar((-230.25850929940458), AdValue::div(AdValue::neg(scratch.ad_value(497)), scratch.ad_value(1381))), 0.3333333333333333), 1.0)), 0.5), 1.0)), 1.0);
            scratch.store_ad(1356, &AdValue::div_from_scalar(1e-100, assign21390_ad_e23479));
        }

        if ((((((scratch.values[1320] != 0.0) && (scratch.values[1337] != 0.0)) && (!(scratch.values[1571] != 0.0))) && (!(scratch.values[1582] != 0.0))) && (!(scratch.values[1584] != 0.0))) && (!(scratch.values[1585] != 0.0))) {
            let assign21400_ad_e23529: AdValue = AdValue::scale(AdValue::offset(AdValue::mul(AdValue::offset(AdValue::div(AdValue::neg(scratch.ad_value(497)), scratch.ad_value(1381)), (-230.25850929940458)), AdValue::offset(AdValue::scale(AdValue::mul(AdValue::offset(AdValue::div(AdValue::neg(scratch.ad_value(497)), scratch.ad_value(1381)), (-230.25850929940458)), AdValue::offset(AdValue::scale(AdValue::offset(AdValue::div(AdValue::neg(scratch.ad_value(497)), scratch.ad_value(1381)), (-230.25850929940458)), 0.3333333333333333), 1.0)), 0.5), 1.0)), 1.0), 1e100);
            scratch.store_ad(1356, &assign21400_ad_e23529);
        }

        if ((((scratch.values[1320] != 0.0) && (scratch.values[1337] != 0.0)) && (!(scratch.values[1571] != 0.0))) && (!(scratch.values[1582] != 0.0))) {
            scratch.store_ad(1380, &AdValue::scale(AdValue::mul(AdValue::mul(AdValue::mul(scratch.ad_value(526), scratch.ad_value(1381)), scratch.ad_value(1381)), scratch.ad_value(1356)), scratch.values[402]));
        }

        scratch.values[1586] = if (scratch.values[411] > 1000.0) { 1.0 } else { 0.0 };

        if ((((scratch.values[1320] != 0.0) && (scratch.values[1337] != 0.0)) && (!(scratch.values[1571] != 0.0))) && (scratch.values[1586] != 0.0)) {
            scratch.values[1382] = 1.0;
            scratch.node_derivatives[1382] = [0.0; Instance::NODE_COUNT];
            scratch.branch_derivatives[1382] = [0.0; Instance::BRANCH_COUNT];
        }

        scratch.values[1587] = if (scratch.values[1355] > ((-scratch.values[500]) * scratch.values[411])) { 1.0 } else { 0.0 };

        scratch.values[1588] = if (scratch.values[414] == 4.0) { 1.0 } else { 0.0 };

        if ((((((scratch.values[1320] != 0.0) && (scratch.values[1337] != 0.0)) && (!(scratch.values[1571] != 0.0))) && (!(scratch.values[1586] != 0.0))) && (scratch.values[1587] != 0.0)) && (scratch.values[1588] != 0.0)) {
            scratch.store_ad(1356, &AdValue::mul(AdValue::mul(AdValue::mul(AdValue::scale(scratch.ad_value(1355), scratch.values[504]), AdValue::scale(scratch.ad_value(1355), scratch.values[504])), AdValue::scale(scratch.ad_value(1355), scratch.values[504])), AdValue::scale(scratch.ad_value(1355), scratch.values[504])));
        }

        if ((((((scratch.values[1320] != 0.0) && (scratch.values[1337] != 0.0)) && (!(scratch.values[1571] != 0.0))) && (!(scratch.values[1586] != 0.0))) && (scratch.values[1587] != 0.0)) && (!(scratch.values[1588] != 0.0))) {
            scratch.store_ad(1356, &AdValue::powf(AdValue::abs(AdValue::scale(scratch.ad_value(1355), scratch.values[504])), scratch.values[414]));
        }

        if (((((scratch.values[1320] != 0.0) && (scratch.values[1337] != 0.0)) && (!(scratch.values[1571] != 0.0))) && (!(scratch.values[1586] != 0.0))) && (scratch.values[1587] != 0.0)) {
            scratch.store_ad(1382, &AdValue::div_from_scalar(1.0, AdValue::sub_from_scalar(1.0, scratch.ad_value(1356))));
        }

        if (((((scratch.values[1320] != 0.0) && (scratch.values[1337] != 0.0)) && (!(scratch.values[1571] != 0.0))) && (!(scratch.values[1586] != 0.0))) && (!(scratch.values[1587] != 0.0))) {
            scratch.store_ad(1382, &AdValue::offset(AdValue::scale(AdValue::offset(scratch.ad_value(1355), (scratch.values[500] * scratch.values[411])), scratch.values[507]), scratch.values[501]));
        }

        if (((scratch.values[1320] != 0.0) && (scratch.values[1337] != 0.0)) && (!(scratch.values[1571] != 0.0))) {
            scratch.store_ad(1383, &AdValue::mul(AdValue::add(AdValue::add(AdValue::add(scratch.ad_value(1357), scratch.ad_value(1358)), scratch.ad_value(1365)), scratch.ad_value(1380)), scratch.ad_value(1382)));
        }

        scratch.values[1589] = if (scratch.values[698] == 0.0) { 1.0 } else { 0.0 };

        if (((scratch.values[1320] != 0.0) && (scratch.values[1337] != 0.0)) && (scratch.values[1589] != 0.0)) {
            scratch.values[1384] = 0.0;
            scratch.node_derivatives[1384] = [0.0; Instance::NODE_COUNT];
            scratch.branch_derivatives[1384] = [0.0; Instance::BRANCH_COUNT];
        }

        scratch.values[1590] = if (scratch.values[465] == 0.5) { 1.0 } else { 0.0 };

        if ((((scratch.values[1320] != 0.0) && (scratch.values[1337] != 0.0)) && (!(scratch.values[1589] != 0.0))) && (scratch.values[1590] != 0.0)) {
            scratch.store_ad(1356, &AdValue::sqrt(AdValue::sub_from_scalar(1.0, AdValue::scale(scratch.ad_value(1348), scratch.values[462]))));
        }

        if ((((scratch.values[1320] != 0.0) && (scratch.values[1337] != 0.0)) && (!(scratch.values[1589] != 0.0))) && (!(scratch.values[1590] != 0.0))) {
            scratch.store_ad(1356, &AdValue::powf(AdValue::sub_from_scalar(1.0, AdValue::scale(scratch.ad_value(1348), scratch.values[462])), scratch.values[465]));
        }

        if (((scratch.values[1320] != 0.0) && (scratch.values[1337] != 0.0)) && (!(scratch.values[1589] != 0.0))) {
            scratch.store_ad(1357, &AdValue::scale(scratch.ad_value(1347), scratch.values[444]));
        }

        scratch.values[1591] = if ((scratch.values[394] == 0.0) && (scratch.values[397] == 0.0)) { 1.0 } else { 0.0 };

        if ((((scratch.values[1320] != 0.0) && (scratch.values[1337] != 0.0)) && (!(scratch.values[1589] != 0.0))) && (scratch.values[1591] != 0.0)) {
            scratch.values[1358] = 0.0;
            scratch.node_derivatives[1358] = [0.0; Instance::NODE_COUNT];
            scratch.branch_derivatives[1358] = [0.0; Instance::BRANCH_COUNT];
        }

        if ((((scratch.values[1320] != 0.0) && (scratch.values[1337] != 0.0)) && (!(scratch.values[1589] != 0.0))) && (!(scratch.values[1591] != 0.0))) {
            scratch.store_ad(1359, &AdValue::sub_from_scalar(scratch.values[450], scratch.ad_value(1353)));
        }

        if ((((scratch.values[1320] != 0.0) && (scratch.values[1337] != 0.0)) && (!(scratch.values[1589] != 0.0))) && (!(scratch.values[1591] != 0.0))) {
            scratch.store_ad(1360, &AdValue::sub_from_scalar(1.0, AdValue::sqrt(AdValue::sub_from_scalar(1.0, AdValue::div(scratch.ad_value(1351), scratch.ad_value(1359))))));
        }

        scratch.values[1592] = if (scratch.values[383] == 0.5) { 1.0 } else { 0.0 };

        if (((((scratch.values[1320] != 0.0) && (scratch.values[1337] != 0.0)) && (!(scratch.values[1589] != 0.0))) && (!(scratch.values[1591] != 0.0))) && (scratch.values[1592] != 0.0)) {
            scratch.values[1361] = 0.0;
            scratch.node_derivatives[1361] = [0.0; Instance::NODE_COUNT];
            scratch.branch_derivatives[1361] = [0.0; Instance::BRANCH_COUNT];
        }

        if (((((scratch.values[1320] != 0.0) && (scratch.values[1337] != 0.0)) && (!(scratch.values[1589] != 0.0))) && (!(scratch.values[1591] != 0.0))) && (!(scratch.values[1592] != 0.0))) {
            scratch.store_ad(1361, &AdValue::scale(AdValue::add(AdValue::div(AdValue::mul(AdValue::square(scratch.ad_value(1360)), AdValue::ln(scratch.ad_value(1360))), AdValue::sub_from_scalar(1.0, scratch.ad_value(1360))), scratch.ad_value(1360)), (1.0 - (2.0 * scratch.values[383]))));
        }

        if ((((scratch.values[1320] != 0.0) && (scratch.values[1337] != 0.0)) && (!(scratch.values[1589] != 0.0))) && (!(scratch.values[1591] != 0.0))) {
            scratch.store_ad(1362, &AdValue::add(scratch.ad_value(1360), scratch.ad_value(1361)));
        }

        scratch.values[1593] = if (scratch.values[383] == 0.5) { 1.0 } else { 0.0 };

        if (((((scratch.values[1320] != 0.0) && (scratch.values[1337] != 0.0)) && (!(scratch.values[1589] != 0.0))) && (!(scratch.values[1591] != 0.0))) && (scratch.values[1593] != 0.0)) {
            scratch.store_ad(1356, &AdValue::sqrt(AdValue::scale(scratch.ad_value(1359), scratch.values[486])));
        }

        if (((((scratch.values[1320] != 0.0) && (scratch.values[1337] != 0.0)) && (!(scratch.values[1589] != 0.0))) && (!(scratch.values[1591] != 0.0))) && (!(scratch.values[1593] != 0.0))) {
            scratch.store_ad(1356, &AdValue::powf(AdValue::scale(scratch.ad_value(1359), scratch.values[486]), scratch.values[383]));
        }

        if ((((scratch.values[1320] != 0.0) && (scratch.values[1337] != 0.0)) && (!(scratch.values[1589] != 0.0))) && (!(scratch.values[1591] != 0.0))) {
            scratch.store_ad(1363, &AdValue::scale(scratch.ad_value(1356), scratch.values[480]));
        }

        if ((((scratch.values[1320] != 0.0) && (scratch.values[1337] != 0.0)) && (!(scratch.values[1589] != 0.0))) && (!(scratch.values[1591] != 0.0))) {
            scratch.store_ad(1364, &AdValue::scale(AdValue::mul(AdValue::offset(scratch.ad_value(1350), (-1.0)), scratch.ad_value(1363)), scratch.values[441]));
        }

        if ((((scratch.values[1320] != 0.0) && (scratch.values[1337] != 0.0)) && (!(scratch.values[1589] != 0.0))) && (!(scratch.values[1591] != 0.0))) {
            scratch.store_ad(1358, &AdValue::scale(AdValue::mul(scratch.ad_value(1364), scratch.ad_value(1362)), scratch.values[394]));
        }

    }

    pub(super) fn stamp_transient_block_17(
        &mut self,
        ctx: &GeneratedEvalContext<'_>,
        stamper: &mut GeneratedStamper<'_>,
        scratch: &mut Scratch,
    ) {
        let _ = stamper;
        scratch.values[1594] = if (scratch.values[397] == 0.0) { 1.0 } else { 0.0 };

        if ((((scratch.values[1320] != 0.0) && (scratch.values[1337] != 0.0)) && (!(scratch.values[1589] != 0.0))) && (scratch.values[1594] != 0.0)) {
            scratch.values[1365] = 0.0;
            scratch.node_derivatives[1365] = [0.0; Instance::NODE_COUNT];
            scratch.branch_derivatives[1365] = [0.0; Instance::BRANCH_COUNT];
        }

        if ((((scratch.values[1320] != 0.0) && (scratch.values[1337] != 0.0)) && (!(scratch.values[1589] != 0.0))) && (!(scratch.values[1594] != 0.0))) {
            scratch.store_ad(1366, &AdValue::scale(AdValue::div(AdValue::scale(scratch.ad_value(1363), scratch.values[465]), scratch.ad_value(1359)), scratch.values[495]));
        }

        if ((((scratch.values[1320] != 0.0) && (scratch.values[1337] != 0.0)) && (!(scratch.values[1589] != 0.0))) && (!(scratch.values[1594] != 0.0))) {
            scratch.store_ad(1367, &AdValue::div_from_scalar((0.666666666666667 * scratch.values[492]), scratch.ad_value(1366)));
        }

        if ((((scratch.values[1320] != 0.0) && (scratch.values[1337] != 0.0)) && (!(scratch.values[1589] != 0.0))) && (!(scratch.values[1594] != 0.0))) {
            scratch.store_ad(1368, &AdValue::square(scratch.ad_value(1367)));
        }

        if ((((scratch.values[1320] != 0.0) && (scratch.values[1337] != 0.0)) && (!(scratch.values[1589] != 0.0))) && (!(scratch.values[1594] != 0.0))) {
            scratch.store_ad(1369, &AdValue::sqrt(AdValue::div(AdValue::square(scratch.ad_value(1368)), AdValue::offset(AdValue::square(scratch.ad_value(1368)), 1.0))));
        }

        if ((((scratch.values[1320] != 0.0) && (scratch.values[1337] != 0.0)) && (!(scratch.values[1589] != 0.0))) && (!(scratch.values[1594] != 0.0))) {
            scratch.store_ad(1370, &AdValue::sqrt(AdValue::abs(scratch.ad_value(1369))));
        }

        if ((((scratch.values[1320] != 0.0) && (scratch.values[1337] != 0.0)) && (!(scratch.values[1589] != 0.0))) && (!(scratch.values[1594] != 0.0))) {
            scratch.store_ad(1371, &AdValue::mul(scratch.ad_value(1369), scratch.ad_value(1370)));
        }

        scratch.values[1595] = if (((-scratch.values[383]) * scratch.values[468]) == (-1.0)) { 1.0 } else { 0.0 };

        if (((((scratch.values[1320] != 0.0) && (scratch.values[1337] != 0.0)) && (!(scratch.values[1589] != 0.0))) && (!(scratch.values[1594] != 0.0))) && (scratch.values[1595] != 0.0)) {
            scratch.store_ad(1372, &AdValue::div_from_scalar(1.0, AdValue::offset(AdValue::mul(scratch.ad_value(1366), scratch.ad_value(1371)), 1.0)));
        }

        if (((((scratch.values[1320] != 0.0) && (scratch.values[1337] != 0.0)) && (!(scratch.values[1589] != 0.0))) && (!(scratch.values[1594] != 0.0))) && (!(scratch.values[1595] != 0.0))) {
            scratch.store_ad(1372, &AdValue::powf(AdValue::offset(AdValue::mul(scratch.ad_value(1366), scratch.ad_value(1371)), 1.0), ((-scratch.values[383]) * scratch.values[468])));
        }

        if ((((scratch.values[1320] != 0.0) && (scratch.values[1337] != 0.0)) && (!(scratch.values[1589] != 0.0))) && (!(scratch.values[1594] != 0.0))) {
            scratch.store_ad(1373, &AdValue::div(AdValue::mul(scratch.ad_value(1362), scratch.ad_value(1372)), AdValue::add(scratch.ad_value(1362), scratch.ad_value(1372))));
        }

        if ((((scratch.values[1320] != 0.0) && (scratch.values[1337] != 0.0)) && (!(scratch.values[1589] != 0.0))) && (!(scratch.values[1594] != 0.0))) {
            scratch.store_ad(1374, &AdValue::sqrt(AdValue::scale(AdValue::div(scratch.ad_value(1366), scratch.ad_value(1370)), 0.375)));
        }

        if ((((scratch.values[1320] != 0.0) && (scratch.values[1337] != 0.0)) && (!(scratch.values[1589] != 0.0))) && (!(scratch.values[1594] != 0.0))) {
            scratch.store_ad(1375, &AdValue::sub(AdValue::scale(AdValue::mul(scratch.ad_value(1367), scratch.ad_value(1370)), 2.0), scratch.ad_value(1369)));
        }

        if ((((scratch.values[1320] != 0.0) && (scratch.values[1337] != 0.0)) && (!(scratch.values[1589] != 0.0))) && (!(scratch.values[1594] != 0.0))) {
            scratch.store_ad(1376, &AdValue::add(AdValue::sub(AdValue::mul(AdValue::scale(scratch.ad_value(1367), scratch.values[492]), scratch.ad_value(1370)), AdValue::scale(scratch.ad_value(1369), scratch.values[492])), AdValue::scale(AdValue::mul(scratch.ad_value(1366), scratch.ad_value(1371)), 0.5)));
        }

        if ((((scratch.values[1320] != 0.0) && (scratch.values[1337] != 0.0)) && (!(scratch.values[1589] != 0.0))) && (!(scratch.values[1594] != 0.0))) {
            scratch.store_ad(1377, &AdValue::mul(AdValue::offset(scratch.ad_value(1375), (-1.0)), scratch.ad_value(1374)));
        }

        if ((((scratch.values[1320] != 0.0) && (scratch.values[1337] != 0.0)) && (!(scratch.values[1589] != 0.0))) && (!(scratch.values[1594] != 0.0))) {
            scratch.store_ad(1338, &AdValue::square(scratch.ad_value(1377)));
        }

        scratch.values[1596] = if (scratch.values[1377] > 0.0) { 1.0 } else { 0.0 };

        if (((((scratch.values[1320] != 0.0) && (scratch.values[1337] != 0.0)) && (!(scratch.values[1589] != 0.0))) && (!(scratch.values[1594] != 0.0))) && (scratch.values[1596] != 0.0)) {
            scratch.store_ad(1339, &AdValue::div_from_scalar(1.0, AdValue::offset(AdValue::scale(scratch.ad_value(1377), scratch.values[428]), 1.0)));
        }

        if (((((scratch.values[1320] != 0.0) && (scratch.values[1337] != 0.0)) && (!(scratch.values[1589] != 0.0))) && (!(scratch.values[1594] != 0.0))) && (!(scratch.values[1596] != 0.0))) {
            scratch.store_ad(1339, &AdValue::div_from_scalar(1.0, AdValue::sub_from_scalar(1.0, AdValue::scale(scratch.ad_value(1377), scratch.values[428]))));
        }

        scratch.values[1597] = if (((-scratch.values[1338]) + scratch.values[1376]) > (-230.25850929940458)) { 1.0 } else { 0.0 };

        if (((((scratch.values[1320] != 0.0) && (scratch.values[1337] != 0.0)) && (!(scratch.values[1589] != 0.0))) && (!(scratch.values[1594] != 0.0))) && (scratch.values[1597] != 0.0)) {
            scratch.store_ad(1356, &AdValue::exp(AdValue::sub(scratch.ad_value(1376), scratch.ad_value(1338))));
        }

        if (((((scratch.values[1320] != 0.0) && (scratch.values[1337] != 0.0)) && (!(scratch.values[1589] != 0.0))) && (!(scratch.values[1594] != 0.0))) && (!(scratch.values[1597] != 0.0))) {
            let assign21950_ad_e24358: AdValue = AdValue::div_from_scalar(1e-100, AdValue::offset(AdValue::mul(AdValue::sub_from_scalar((-230.25850929940458), AdValue::sub(scratch.ad_value(1376), scratch.ad_value(1338))), AdValue::offset(AdValue::scale(AdValue::mul(AdValue::sub_from_scalar((-230.25850929940458), AdValue::sub(scratch.ad_value(1376), scratch.ad_value(1338))), AdValue::offset(AdValue::scale(AdValue::sub_from_scalar((-230.25850929940458), AdValue::sub(scratch.ad_value(1376), scratch.ad_value(1338))), 0.3333333333333333), 1.0)), 0.5), 1.0)), 1.0));
            scratch.store_ad(1356, &assign21950_ad_e24358);
        }

        if ((((scratch.values[1320] != 0.0) && (scratch.values[1337] != 0.0)) && (!(scratch.values[1589] != 0.0))) && (!(scratch.values[1594] != 0.0))) {
            scratch.store_ad(1340, &AdValue::mul(AdValue::add(AdValue::add(AdValue::scale(scratch.ad_value(1339), 0.29214664), AdValue::scale(AdValue::square(scratch.ad_value(1339)), scratch.values[429])), AdValue::scale(AdValue::mul(AdValue::square(scratch.ad_value(1339)), scratch.ad_value(1339)), scratch.values[430])), scratch.ad_value(1356)));
        }

        scratch.values[1598] = if (scratch.values[1377] > 0.0) { 1.0 } else { 0.0 };

        if (((((scratch.values[1320] != 0.0) && (scratch.values[1337] != 0.0)) && (!(scratch.values[1589] != 0.0))) && (!(scratch.values[1594] != 0.0))) && (scratch.values[1598] != 0.0)) {
            scratch.values[1378] = scratch.values[1340];
            scratch.node_derivatives[1378] = scratch.node_derivatives[1340];
            scratch.branch_derivatives[1378] = scratch.branch_derivatives[1340];
        }

        scratch.values[1599] = if (scratch.values[1376] > (-230.25850929940458)) { 1.0 } else { 0.0 };

        if ((((((scratch.values[1320] != 0.0) && (scratch.values[1337] != 0.0)) && (!(scratch.values[1589] != 0.0))) && (!(scratch.values[1594] != 0.0))) && (!(scratch.values[1598] != 0.0))) && (scratch.values[1599] != 0.0)) {
            scratch.store_ad(1356, &AdValue::exp(scratch.ad_value(1376)));
        }

        if ((((((scratch.values[1320] != 0.0) && (scratch.values[1337] != 0.0)) && (!(scratch.values[1589] != 0.0))) && (!(scratch.values[1594] != 0.0))) && (!(scratch.values[1598] != 0.0))) && (!(scratch.values[1599] != 0.0))) {
            scratch.store_ad(1356, &AdValue::div_from_scalar(1e-100, AdValue::offset(AdValue::mul(AdValue::sub_from_scalar((-230.25850929940458), scratch.ad_value(1376)), AdValue::offset(AdValue::scale(AdValue::mul(AdValue::sub_from_scalar((-230.25850929940458), scratch.ad_value(1376)), AdValue::offset(AdValue::scale(AdValue::sub_from_scalar((-230.25850929940458), scratch.ad_value(1376)), 0.3333333333333333), 1.0)), 0.5), 1.0)), 1.0)));
        }

        if (((((scratch.values[1320] != 0.0) && (scratch.values[1337] != 0.0)) && (!(scratch.values[1589] != 0.0))) && (!(scratch.values[1594] != 0.0))) && (!(scratch.values[1598] != 0.0))) {
            scratch.store_ad(1378, &AdValue::sub(AdValue::scale(scratch.ad_value(1356), 2.0), scratch.ad_value(1340)));
        }

        if ((((scratch.values[1320] != 0.0) && (scratch.values[1337] != 0.0)) && (!(scratch.values[1589] != 0.0))) && (!(scratch.values[1594] != 0.0))) {
            scratch.store_ad(1379, &AdValue::scale(AdValue::div(AdValue::scale(scratch.ad_value(1378), scratch.values[492]), scratch.ad_value(1374)), (1.772453850905516 * 0.5)));
        }

        if ((((scratch.values[1320] != 0.0) && (scratch.values[1337] != 0.0)) && (!(scratch.values[1589] != 0.0))) && (!(scratch.values[1594] != 0.0))) {
            scratch.store_ad(1365, &AdValue::scale(AdValue::mul(AdValue::mul(scratch.ad_value(1364), scratch.ad_value(1379)), scratch.ad_value(1373)), scratch.values[397]));
        }

        scratch.values[1600] = if (scratch.values[403] == 0.0) { 1.0 } else { 0.0 };

        if ((((scratch.values[1320] != 0.0) && (scratch.values[1337] != 0.0)) && (!(scratch.values[1589] != 0.0))) && (scratch.values[1600] != 0.0)) {
            scratch.values[1380] = 0.0;
            scratch.node_derivatives[1380] = [0.0; Instance::NODE_COUNT];
            scratch.branch_derivatives[1380] = [0.0; Instance::BRANCH_COUNT];
        }

        scratch.values[1601] = if (scratch.values[383] == 0.5) { 1.0 } else { 0.0 };

        if (((((scratch.values[1320] != 0.0) && (scratch.values[1337] != 0.0)) && (!(scratch.values[1589] != 0.0))) && (!(scratch.values[1600] != 0.0))) && (scratch.values[1601] != 0.0)) {
            scratch.store_ad(1356, &AdValue::sqrt(AdValue::scale(AdValue::sub_from_scalar(scratch.values[380], scratch.ad_value(1354)), scratch.values[486])));
        }

        if (((((scratch.values[1320] != 0.0) && (scratch.values[1337] != 0.0)) && (!(scratch.values[1589] != 0.0))) && (!(scratch.values[1600] != 0.0))) && (!(scratch.values[1601] != 0.0))) {
            scratch.store_ad(1356, &AdValue::powf(AdValue::scale(AdValue::sub_from_scalar(scratch.values[380], scratch.ad_value(1354)), scratch.values[486]), scratch.values[383]));
        }

        if ((((scratch.values[1320] != 0.0) && (scratch.values[1337] != 0.0)) && (!(scratch.values[1589] != 0.0))) && (!(scratch.values[1600] != 0.0))) {
            scratch.store_ad(1381, &AdValue::scale(AdValue::div(AdValue::scale(AdValue::sub_from_scalar(scratch.values[380], scratch.ad_value(1354)), scratch.values[483]), scratch.ad_value(1356)), scratch.values[468]));
        }

        scratch.values[1602] = if (((((-scratch.values[498]) / scratch.values[1381])) as f64).abs() < 230.25850929940458) { 1.0 } else { 0.0 };

        if (((((scratch.values[1320] != 0.0) && (scratch.values[1337] != 0.0)) && (!(scratch.values[1589] != 0.0))) && (!(scratch.values[1600] != 0.0))) && (scratch.values[1602] != 0.0)) {
            scratch.store_ad(1356, &AdValue::exp(AdValue::div(AdValue::neg(scratch.ad_value(498)), scratch.ad_value(1381))));
        }

        scratch.values[1603] = if (((-scratch.values[498]) / scratch.values[1381]) < 0.0) { 1.0 } else { 0.0 };

        if ((((((scratch.values[1320] != 0.0) && (scratch.values[1337] != 0.0)) && (!(scratch.values[1589] != 0.0))) && (!(scratch.values[1600] != 0.0))) && (!(scratch.values[1602] != 0.0))) && (scratch.values[1603] != 0.0)) {
            let assign22140_ad_e24685: AdValue = AdValue::offset(AdValue::mul(AdValue::sub_from_scalar((-230.25850929940458), AdValue::div(AdValue::neg(scratch.ad_value(498)), scratch.ad_value(1381))), AdValue::offset(AdValue::scale(AdValue::mul(AdValue::sub_from_scalar((-230.25850929940458), AdValue::div(AdValue::neg(scratch.ad_value(498)), scratch.ad_value(1381))), AdValue::offset(AdValue::scale(AdValue::sub_from_scalar((-230.25850929940458), AdValue::div(AdValue::neg(scratch.ad_value(498)), scratch.ad_value(1381))), 0.3333333333333333), 1.0)), 0.5), 1.0)), 1.0);
            scratch.store_ad(1356, &AdValue::div_from_scalar(1e-100, assign22140_ad_e24685));
        }

        if ((((((scratch.values[1320] != 0.0) && (scratch.values[1337] != 0.0)) && (!(scratch.values[1589] != 0.0))) && (!(scratch.values[1600] != 0.0))) && (!(scratch.values[1602] != 0.0))) && (!(scratch.values[1603] != 0.0))) {
            let assign22150_ad_e24735: AdValue = AdValue::scale(AdValue::offset(AdValue::mul(AdValue::offset(AdValue::div(AdValue::neg(scratch.ad_value(498)), scratch.ad_value(1381)), (-230.25850929940458)), AdValue::offset(AdValue::scale(AdValue::mul(AdValue::offset(AdValue::div(AdValue::neg(scratch.ad_value(498)), scratch.ad_value(1381)), (-230.25850929940458)), AdValue::offset(AdValue::scale(AdValue::offset(AdValue::div(AdValue::neg(scratch.ad_value(498)), scratch.ad_value(1381)), (-230.25850929940458)), 0.3333333333333333), 1.0)), 0.5), 1.0)), 1.0), 1e100);
            scratch.store_ad(1356, &assign22150_ad_e24735);
        }

        if ((((scratch.values[1320] != 0.0) && (scratch.values[1337] != 0.0)) && (!(scratch.values[1589] != 0.0))) && (!(scratch.values[1600] != 0.0))) {
            scratch.store_ad(1380, &AdValue::scale(AdValue::mul(AdValue::mul(AdValue::mul(scratch.ad_value(526), scratch.ad_value(1381)), scratch.ad_value(1381)), scratch.ad_value(1356)), scratch.values[403]));
        }

        scratch.values[1604] = if (scratch.values[412] > 1000.0) { 1.0 } else { 0.0 };

        if ((((scratch.values[1320] != 0.0) && (scratch.values[1337] != 0.0)) && (!(scratch.values[1589] != 0.0))) && (scratch.values[1604] != 0.0)) {
            scratch.values[1382] = 1.0;
            scratch.node_derivatives[1382] = [0.0; Instance::NODE_COUNT];
            scratch.branch_derivatives[1382] = [0.0; Instance::BRANCH_COUNT];
        }

        scratch.values[1605] = if (scratch.values[1355] > ((-scratch.values[500]) * scratch.values[412])) { 1.0 } else { 0.0 };

        scratch.values[1606] = if (scratch.values[415] == 4.0) { 1.0 } else { 0.0 };

        if ((((((scratch.values[1320] != 0.0) && (scratch.values[1337] != 0.0)) && (!(scratch.values[1589] != 0.0))) && (!(scratch.values[1604] != 0.0))) && (scratch.values[1605] != 0.0)) && (scratch.values[1606] != 0.0)) {
            scratch.store_ad(1356, &AdValue::mul(AdValue::mul(AdValue::mul(AdValue::scale(scratch.ad_value(1355), scratch.values[505]), AdValue::scale(scratch.ad_value(1355), scratch.values[505])), AdValue::scale(scratch.ad_value(1355), scratch.values[505])), AdValue::scale(scratch.ad_value(1355), scratch.values[505])));
        }

        if ((((((scratch.values[1320] != 0.0) && (scratch.values[1337] != 0.0)) && (!(scratch.values[1589] != 0.0))) && (!(scratch.values[1604] != 0.0))) && (scratch.values[1605] != 0.0)) && (!(scratch.values[1606] != 0.0))) {
            scratch.store_ad(1356, &AdValue::powf(AdValue::abs(AdValue::scale(scratch.ad_value(1355), scratch.values[505])), scratch.values[415]));
        }

        if (((((scratch.values[1320] != 0.0) && (scratch.values[1337] != 0.0)) && (!(scratch.values[1589] != 0.0))) && (!(scratch.values[1604] != 0.0))) && (scratch.values[1605] != 0.0)) {
            scratch.store_ad(1382, &AdValue::div_from_scalar(1.0, AdValue::sub_from_scalar(1.0, scratch.ad_value(1356))));
        }

        if (((((scratch.values[1320] != 0.0) && (scratch.values[1337] != 0.0)) && (!(scratch.values[1589] != 0.0))) && (!(scratch.values[1604] != 0.0))) && (!(scratch.values[1605] != 0.0))) {
            scratch.store_ad(1382, &AdValue::offset(AdValue::scale(AdValue::offset(scratch.ad_value(1355), (scratch.values[500] * scratch.values[412])), scratch.values[508]), scratch.values[502]));
        }

        if (((scratch.values[1320] != 0.0) && (scratch.values[1337] != 0.0)) && (!(scratch.values[1589] != 0.0))) {
            scratch.store_ad(1384, &AdValue::mul(AdValue::add(AdValue::add(AdValue::add(scratch.ad_value(1357), scratch.ad_value(1358)), scratch.ad_value(1365)), scratch.ad_value(1380)), scratch.ad_value(1382)));
        }

        scratch.values[1607] = if (scratch.values[699] == 0.0) { 1.0 } else { 0.0 };

        if (((scratch.values[1320] != 0.0) && (scratch.values[1337] != 0.0)) && (scratch.values[1607] != 0.0)) {
            scratch.values[1385] = 0.0;
            scratch.node_derivatives[1385] = [0.0; Instance::NODE_COUNT];
            scratch.branch_derivatives[1385] = [0.0; Instance::BRANCH_COUNT];
        }

        scratch.values[1608] = if (scratch.values[466] == 0.5) { 1.0 } else { 0.0 };

        if ((((scratch.values[1320] != 0.0) && (scratch.values[1337] != 0.0)) && (!(scratch.values[1607] != 0.0))) && (scratch.values[1608] != 0.0)) {
            scratch.store_ad(1356, &AdValue::sqrt(AdValue::sub_from_scalar(1.0, AdValue::scale(scratch.ad_value(1348), scratch.values[463]))));
        }

        if ((((scratch.values[1320] != 0.0) && (scratch.values[1337] != 0.0)) && (!(scratch.values[1607] != 0.0))) && (!(scratch.values[1608] != 0.0))) {
            scratch.store_ad(1356, &AdValue::powf(AdValue::sub_from_scalar(1.0, AdValue::scale(scratch.ad_value(1348), scratch.values[463])), scratch.values[466]));
        }

        if (((scratch.values[1320] != 0.0) && (scratch.values[1337] != 0.0)) && (!(scratch.values[1607] != 0.0))) {
            scratch.store_ad(1357, &AdValue::scale(scratch.ad_value(1347), scratch.values[445]));
        }

        scratch.values[1609] = if ((scratch.values[395] == 0.0) && (scratch.values[398] == 0.0)) { 1.0 } else { 0.0 };

        if ((((scratch.values[1320] != 0.0) && (scratch.values[1337] != 0.0)) && (!(scratch.values[1607] != 0.0))) && (scratch.values[1609] != 0.0)) {
            scratch.values[1358] = 0.0;
            scratch.node_derivatives[1358] = [0.0; Instance::NODE_COUNT];
            scratch.branch_derivatives[1358] = [0.0; Instance::BRANCH_COUNT];
        }

        if ((((scratch.values[1320] != 0.0) && (scratch.values[1337] != 0.0)) && (!(scratch.values[1607] != 0.0))) && (!(scratch.values[1609] != 0.0))) {
            scratch.store_ad(1359, &AdValue::sub_from_scalar(scratch.values[451], scratch.ad_value(1353)));
        }

        if ((((scratch.values[1320] != 0.0) && (scratch.values[1337] != 0.0)) && (!(scratch.values[1607] != 0.0))) && (!(scratch.values[1609] != 0.0))) {
            scratch.store_ad(1360, &AdValue::sub_from_scalar(1.0, AdValue::sqrt(AdValue::sub_from_scalar(1.0, AdValue::div(scratch.ad_value(1351), scratch.ad_value(1359))))));
        }

        scratch.values[1610] = if (scratch.values[384] == 0.5) { 1.0 } else { 0.0 };

        if (((((scratch.values[1320] != 0.0) && (scratch.values[1337] != 0.0)) && (!(scratch.values[1607] != 0.0))) && (!(scratch.values[1609] != 0.0))) && (scratch.values[1610] != 0.0)) {
            scratch.values[1361] = 0.0;
            scratch.node_derivatives[1361] = [0.0; Instance::NODE_COUNT];
            scratch.branch_derivatives[1361] = [0.0; Instance::BRANCH_COUNT];
        }

        if (((((scratch.values[1320] != 0.0) && (scratch.values[1337] != 0.0)) && (!(scratch.values[1607] != 0.0))) && (!(scratch.values[1609] != 0.0))) && (!(scratch.values[1610] != 0.0))) {
            scratch.store_ad(1361, &AdValue::scale(AdValue::add(AdValue::div(AdValue::mul(AdValue::square(scratch.ad_value(1360)), AdValue::ln(scratch.ad_value(1360))), AdValue::sub_from_scalar(1.0, scratch.ad_value(1360))), scratch.ad_value(1360)), (1.0 - (2.0 * scratch.values[384]))));
        }

        if ((((scratch.values[1320] != 0.0) && (scratch.values[1337] != 0.0)) && (!(scratch.values[1607] != 0.0))) && (!(scratch.values[1609] != 0.0))) {
            scratch.store_ad(1362, &AdValue::add(scratch.ad_value(1360), scratch.ad_value(1361)));
        }

        scratch.values[1611] = if (scratch.values[384] == 0.5) { 1.0 } else { 0.0 };

        if (((((scratch.values[1320] != 0.0) && (scratch.values[1337] != 0.0)) && (!(scratch.values[1607] != 0.0))) && (!(scratch.values[1609] != 0.0))) && (scratch.values[1611] != 0.0)) {
            scratch.store_ad(1356, &AdValue::sqrt(AdValue::scale(scratch.ad_value(1359), scratch.values[487])));
        }

        if (((((scratch.values[1320] != 0.0) && (scratch.values[1337] != 0.0)) && (!(scratch.values[1607] != 0.0))) && (!(scratch.values[1609] != 0.0))) && (!(scratch.values[1611] != 0.0))) {
            scratch.store_ad(1356, &AdValue::powf(AdValue::scale(scratch.ad_value(1359), scratch.values[487]), scratch.values[384]));
        }

        if ((((scratch.values[1320] != 0.0) && (scratch.values[1337] != 0.0)) && (!(scratch.values[1607] != 0.0))) && (!(scratch.values[1609] != 0.0))) {
            scratch.store_ad(1363, &AdValue::scale(scratch.ad_value(1356), scratch.values[481]));
        }

        if ((((scratch.values[1320] != 0.0) && (scratch.values[1337] != 0.0)) && (!(scratch.values[1607] != 0.0))) && (!(scratch.values[1609] != 0.0))) {
            scratch.store_ad(1364, &AdValue::scale(AdValue::mul(AdValue::offset(scratch.ad_value(1350), (-1.0)), scratch.ad_value(1363)), scratch.values[442]));
        }

        if ((((scratch.values[1320] != 0.0) && (scratch.values[1337] != 0.0)) && (!(scratch.values[1607] != 0.0))) && (!(scratch.values[1609] != 0.0))) {
            scratch.store_ad(1358, &AdValue::scale(AdValue::mul(scratch.ad_value(1364), scratch.ad_value(1362)), scratch.values[395]));
        }

        scratch.values[1612] = if (scratch.values[398] == 0.0) { 1.0 } else { 0.0 };

        if ((((scratch.values[1320] != 0.0) && (scratch.values[1337] != 0.0)) && (!(scratch.values[1607] != 0.0))) && (scratch.values[1612] != 0.0)) {
            scratch.values[1365] = 0.0;
            scratch.node_derivatives[1365] = [0.0; Instance::NODE_COUNT];
            scratch.branch_derivatives[1365] = [0.0; Instance::BRANCH_COUNT];
        }

        if ((((scratch.values[1320] != 0.0) && (scratch.values[1337] != 0.0)) && (!(scratch.values[1607] != 0.0))) && (!(scratch.values[1612] != 0.0))) {
            scratch.store_ad(1366, &AdValue::scale(AdValue::div(AdValue::scale(scratch.ad_value(1363), scratch.values[466]), scratch.ad_value(1359)), scratch.values[496]));
        }

        if ((((scratch.values[1320] != 0.0) && (scratch.values[1337] != 0.0)) && (!(scratch.values[1607] != 0.0))) && (!(scratch.values[1612] != 0.0))) {
            scratch.store_ad(1367, &AdValue::div_from_scalar((0.666666666666667 * scratch.values[493]), scratch.ad_value(1366)));
        }

        if ((((scratch.values[1320] != 0.0) && (scratch.values[1337] != 0.0)) && (!(scratch.values[1607] != 0.0))) && (!(scratch.values[1612] != 0.0))) {
            scratch.store_ad(1368, &AdValue::square(scratch.ad_value(1367)));
        }

        if ((((scratch.values[1320] != 0.0) && (scratch.values[1337] != 0.0)) && (!(scratch.values[1607] != 0.0))) && (!(scratch.values[1612] != 0.0))) {
            scratch.store_ad(1369, &AdValue::sqrt(AdValue::div(AdValue::square(scratch.ad_value(1368)), AdValue::offset(AdValue::square(scratch.ad_value(1368)), 1.0))));
        }

        if ((((scratch.values[1320] != 0.0) && (scratch.values[1337] != 0.0)) && (!(scratch.values[1607] != 0.0))) && (!(scratch.values[1612] != 0.0))) {
            scratch.store_ad(1370, &AdValue::sqrt(AdValue::abs(scratch.ad_value(1369))));
        }

        if ((((scratch.values[1320] != 0.0) && (scratch.values[1337] != 0.0)) && (!(scratch.values[1607] != 0.0))) && (!(scratch.values[1612] != 0.0))) {
            scratch.store_ad(1371, &AdValue::mul(scratch.ad_value(1369), scratch.ad_value(1370)));
        }

        scratch.values[1613] = if (((-scratch.values[384]) * scratch.values[469]) == (-1.0)) { 1.0 } else { 0.0 };

        if (((((scratch.values[1320] != 0.0) && (scratch.values[1337] != 0.0)) && (!(scratch.values[1607] != 0.0))) && (!(scratch.values[1612] != 0.0))) && (scratch.values[1613] != 0.0)) {
            scratch.store_ad(1372, &AdValue::div_from_scalar(1.0, AdValue::offset(AdValue::mul(scratch.ad_value(1366), scratch.ad_value(1371)), 1.0)));
        }

        if (((((scratch.values[1320] != 0.0) && (scratch.values[1337] != 0.0)) && (!(scratch.values[1607] != 0.0))) && (!(scratch.values[1612] != 0.0))) && (!(scratch.values[1613] != 0.0))) {
            scratch.store_ad(1372, &AdValue::powf(AdValue::offset(AdValue::mul(scratch.ad_value(1366), scratch.ad_value(1371)), 1.0), ((-scratch.values[384]) * scratch.values[469])));
        }

        if ((((scratch.values[1320] != 0.0) && (scratch.values[1337] != 0.0)) && (!(scratch.values[1607] != 0.0))) && (!(scratch.values[1612] != 0.0))) {
            scratch.store_ad(1373, &AdValue::div(AdValue::mul(scratch.ad_value(1362), scratch.ad_value(1372)), AdValue::add(scratch.ad_value(1362), scratch.ad_value(1372))));
        }

        if ((((scratch.values[1320] != 0.0) && (scratch.values[1337] != 0.0)) && (!(scratch.values[1607] != 0.0))) && (!(scratch.values[1612] != 0.0))) {
            scratch.store_ad(1374, &AdValue::sqrt(AdValue::scale(AdValue::div(scratch.ad_value(1366), scratch.ad_value(1370)), 0.375)));
        }

        if ((((scratch.values[1320] != 0.0) && (scratch.values[1337] != 0.0)) && (!(scratch.values[1607] != 0.0))) && (!(scratch.values[1612] != 0.0))) {
            scratch.store_ad(1375, &AdValue::sub(AdValue::scale(AdValue::mul(scratch.ad_value(1367), scratch.ad_value(1370)), 2.0), scratch.ad_value(1369)));
        }

        if ((((scratch.values[1320] != 0.0) && (scratch.values[1337] != 0.0)) && (!(scratch.values[1607] != 0.0))) && (!(scratch.values[1612] != 0.0))) {
            scratch.store_ad(1376, &AdValue::add(AdValue::sub(AdValue::mul(AdValue::scale(scratch.ad_value(1367), scratch.values[493]), scratch.ad_value(1370)), AdValue::scale(scratch.ad_value(1369), scratch.values[493])), AdValue::scale(AdValue::mul(scratch.ad_value(1366), scratch.ad_value(1371)), 0.5)));
        }

        if ((((scratch.values[1320] != 0.0) && (scratch.values[1337] != 0.0)) && (!(scratch.values[1607] != 0.0))) && (!(scratch.values[1612] != 0.0))) {
            scratch.store_ad(1377, &AdValue::mul(AdValue::offset(scratch.ad_value(1375), (-1.0)), scratch.ad_value(1374)));
        }

        if ((((scratch.values[1320] != 0.0) && (scratch.values[1337] != 0.0)) && (!(scratch.values[1607] != 0.0))) && (!(scratch.values[1612] != 0.0))) {
            scratch.store_ad(1338, &AdValue::square(scratch.ad_value(1377)));
        }

        scratch.values[1614] = if (scratch.values[1377] > 0.0) { 1.0 } else { 0.0 };

        if (((((scratch.values[1320] != 0.0) && (scratch.values[1337] != 0.0)) && (!(scratch.values[1607] != 0.0))) && (!(scratch.values[1612] != 0.0))) && (scratch.values[1614] != 0.0)) {
            scratch.store_ad(1339, &AdValue::div_from_scalar(1.0, AdValue::offset(AdValue::scale(scratch.ad_value(1377), scratch.values[428]), 1.0)));
        }

        if (((((scratch.values[1320] != 0.0) && (scratch.values[1337] != 0.0)) && (!(scratch.values[1607] != 0.0))) && (!(scratch.values[1612] != 0.0))) && (!(scratch.values[1614] != 0.0))) {
            scratch.store_ad(1339, &AdValue::div_from_scalar(1.0, AdValue::sub_from_scalar(1.0, AdValue::scale(scratch.ad_value(1377), scratch.values[428]))));
        }

        scratch.values[1615] = if (((-scratch.values[1338]) + scratch.values[1376]) > (-230.25850929940458)) { 1.0 } else { 0.0 };

        if (((((scratch.values[1320] != 0.0) && (scratch.values[1337] != 0.0)) && (!(scratch.values[1607] != 0.0))) && (!(scratch.values[1612] != 0.0))) && (scratch.values[1615] != 0.0)) {
            scratch.store_ad(1356, &AdValue::exp(AdValue::sub(scratch.ad_value(1376), scratch.ad_value(1338))));
        }

        if (((((scratch.values[1320] != 0.0) && (scratch.values[1337] != 0.0)) && (!(scratch.values[1607] != 0.0))) && (!(scratch.values[1612] != 0.0))) && (!(scratch.values[1615] != 0.0))) {
            let assign22700_ad_e25564: AdValue = AdValue::div_from_scalar(1e-100, AdValue::offset(AdValue::mul(AdValue::sub_from_scalar((-230.25850929940458), AdValue::sub(scratch.ad_value(1376), scratch.ad_value(1338))), AdValue::offset(AdValue::scale(AdValue::mul(AdValue::sub_from_scalar((-230.25850929940458), AdValue::sub(scratch.ad_value(1376), scratch.ad_value(1338))), AdValue::offset(AdValue::scale(AdValue::sub_from_scalar((-230.25850929940458), AdValue::sub(scratch.ad_value(1376), scratch.ad_value(1338))), 0.3333333333333333), 1.0)), 0.5), 1.0)), 1.0));
            scratch.store_ad(1356, &assign22700_ad_e25564);
        }

        if ((((scratch.values[1320] != 0.0) && (scratch.values[1337] != 0.0)) && (!(scratch.values[1607] != 0.0))) && (!(scratch.values[1612] != 0.0))) {
            scratch.store_ad(1340, &AdValue::mul(AdValue::add(AdValue::add(AdValue::scale(scratch.ad_value(1339), 0.29214664), AdValue::scale(AdValue::square(scratch.ad_value(1339)), scratch.values[429])), AdValue::scale(AdValue::mul(AdValue::square(scratch.ad_value(1339)), scratch.ad_value(1339)), scratch.values[430])), scratch.ad_value(1356)));
        }

        scratch.values[1616] = if (scratch.values[1377] > 0.0) { 1.0 } else { 0.0 };

        if (((((scratch.values[1320] != 0.0) && (scratch.values[1337] != 0.0)) && (!(scratch.values[1607] != 0.0))) && (!(scratch.values[1612] != 0.0))) && (scratch.values[1616] != 0.0)) {
            scratch.values[1378] = scratch.values[1340];
            scratch.node_derivatives[1378] = scratch.node_derivatives[1340];
            scratch.branch_derivatives[1378] = scratch.branch_derivatives[1340];
        }

        scratch.values[1617] = if (scratch.values[1376] > (-230.25850929940458)) { 1.0 } else { 0.0 };

        if ((((((scratch.values[1320] != 0.0) && (scratch.values[1337] != 0.0)) && (!(scratch.values[1607] != 0.0))) && (!(scratch.values[1612] != 0.0))) && (!(scratch.values[1616] != 0.0))) && (scratch.values[1617] != 0.0)) {
            scratch.store_ad(1356, &AdValue::exp(scratch.ad_value(1376)));
        }

        if ((((((scratch.values[1320] != 0.0) && (scratch.values[1337] != 0.0)) && (!(scratch.values[1607] != 0.0))) && (!(scratch.values[1612] != 0.0))) && (!(scratch.values[1616] != 0.0))) && (!(scratch.values[1617] != 0.0))) {
            scratch.store_ad(1356, &AdValue::div_from_scalar(1e-100, AdValue::offset(AdValue::mul(AdValue::sub_from_scalar((-230.25850929940458), scratch.ad_value(1376)), AdValue::offset(AdValue::scale(AdValue::mul(AdValue::sub_from_scalar((-230.25850929940458), scratch.ad_value(1376)), AdValue::offset(AdValue::scale(AdValue::sub_from_scalar((-230.25850929940458), scratch.ad_value(1376)), 0.3333333333333333), 1.0)), 0.5), 1.0)), 1.0)));
        }

        if (((((scratch.values[1320] != 0.0) && (scratch.values[1337] != 0.0)) && (!(scratch.values[1607] != 0.0))) && (!(scratch.values[1612] != 0.0))) && (!(scratch.values[1616] != 0.0))) {
            scratch.store_ad(1378, &AdValue::sub(AdValue::scale(scratch.ad_value(1356), 2.0), scratch.ad_value(1340)));
        }

        if ((((scratch.values[1320] != 0.0) && (scratch.values[1337] != 0.0)) && (!(scratch.values[1607] != 0.0))) && (!(scratch.values[1612] != 0.0))) {
            scratch.store_ad(1379, &AdValue::scale(AdValue::div(AdValue::scale(scratch.ad_value(1378), scratch.values[493]), scratch.ad_value(1374)), (1.772453850905516 * 0.5)));
        }

        if ((((scratch.values[1320] != 0.0) && (scratch.values[1337] != 0.0)) && (!(scratch.values[1607] != 0.0))) && (!(scratch.values[1612] != 0.0))) {
            scratch.store_ad(1365, &AdValue::scale(AdValue::mul(AdValue::mul(scratch.ad_value(1364), scratch.ad_value(1379)), scratch.ad_value(1373)), scratch.values[398]));
        }

        scratch.values[1618] = if (scratch.values[404] == 0.0) { 1.0 } else { 0.0 };

        if ((((scratch.values[1320] != 0.0) && (scratch.values[1337] != 0.0)) && (!(scratch.values[1607] != 0.0))) && (scratch.values[1618] != 0.0)) {
            scratch.values[1380] = 0.0;
            scratch.node_derivatives[1380] = [0.0; Instance::NODE_COUNT];
            scratch.branch_derivatives[1380] = [0.0; Instance::BRANCH_COUNT];
        }

        scratch.values[1619] = if (scratch.values[384] == 0.5) { 1.0 } else { 0.0 };

        if (((((scratch.values[1320] != 0.0) && (scratch.values[1337] != 0.0)) && (!(scratch.values[1607] != 0.0))) && (!(scratch.values[1618] != 0.0))) && (scratch.values[1619] != 0.0)) {
            scratch.store_ad(1356, &AdValue::sqrt(AdValue::scale(AdValue::sub_from_scalar(scratch.values[381], scratch.ad_value(1354)), scratch.values[487])));
        }

        if (((((scratch.values[1320] != 0.0) && (scratch.values[1337] != 0.0)) && (!(scratch.values[1607] != 0.0))) && (!(scratch.values[1618] != 0.0))) && (!(scratch.values[1619] != 0.0))) {
            scratch.store_ad(1356, &AdValue::powf(AdValue::scale(AdValue::sub_from_scalar(scratch.values[381], scratch.ad_value(1354)), scratch.values[487]), scratch.values[384]));
        }

        if ((((scratch.values[1320] != 0.0) && (scratch.values[1337] != 0.0)) && (!(scratch.values[1607] != 0.0))) && (!(scratch.values[1618] != 0.0))) {
            scratch.store_ad(1381, &AdValue::scale(AdValue::div(AdValue::scale(AdValue::sub_from_scalar(scratch.values[381], scratch.ad_value(1354)), scratch.values[484]), scratch.ad_value(1356)), scratch.values[469]));
        }

        scratch.values[1620] = if (((((-scratch.values[499]) / scratch.values[1381])) as f64).abs() < 230.25850929940458) { 1.0 } else { 0.0 };

        if (((((scratch.values[1320] != 0.0) && (scratch.values[1337] != 0.0)) && (!(scratch.values[1607] != 0.0))) && (!(scratch.values[1618] != 0.0))) && (scratch.values[1620] != 0.0)) {
            scratch.store_ad(1356, &AdValue::exp(AdValue::div(AdValue::neg(scratch.ad_value(499)), scratch.ad_value(1381))));
        }

        scratch.values[1621] = if (((-scratch.values[499]) / scratch.values[1381]) < 0.0) { 1.0 } else { 0.0 };

        if ((((((scratch.values[1320] != 0.0) && (scratch.values[1337] != 0.0)) && (!(scratch.values[1607] != 0.0))) && (!(scratch.values[1618] != 0.0))) && (!(scratch.values[1620] != 0.0))) && (scratch.values[1621] != 0.0)) {
            let assign22890_ad_e25891: AdValue = AdValue::offset(AdValue::mul(AdValue::sub_from_scalar((-230.25850929940458), AdValue::div(AdValue::neg(scratch.ad_value(499)), scratch.ad_value(1381))), AdValue::offset(AdValue::scale(AdValue::mul(AdValue::sub_from_scalar((-230.25850929940458), AdValue::div(AdValue::neg(scratch.ad_value(499)), scratch.ad_value(1381))), AdValue::offset(AdValue::scale(AdValue::sub_from_scalar((-230.25850929940458), AdValue::div(AdValue::neg(scratch.ad_value(499)), scratch.ad_value(1381))), 0.3333333333333333), 1.0)), 0.5), 1.0)), 1.0);
            scratch.store_ad(1356, &AdValue::div_from_scalar(1e-100, assign22890_ad_e25891));
        }

        if ((((((scratch.values[1320] != 0.0) && (scratch.values[1337] != 0.0)) && (!(scratch.values[1607] != 0.0))) && (!(scratch.values[1618] != 0.0))) && (!(scratch.values[1620] != 0.0))) && (!(scratch.values[1621] != 0.0))) {
            let assign22900_ad_e25941: AdValue = AdValue::scale(AdValue::offset(AdValue::mul(AdValue::offset(AdValue::div(AdValue::neg(scratch.ad_value(499)), scratch.ad_value(1381)), (-230.25850929940458)), AdValue::offset(AdValue::scale(AdValue::mul(AdValue::offset(AdValue::div(AdValue::neg(scratch.ad_value(499)), scratch.ad_value(1381)), (-230.25850929940458)), AdValue::offset(AdValue::scale(AdValue::offset(AdValue::div(AdValue::neg(scratch.ad_value(499)), scratch.ad_value(1381)), (-230.25850929940458)), 0.3333333333333333), 1.0)), 0.5), 1.0)), 1.0), 1e100);
            scratch.store_ad(1356, &assign22900_ad_e25941);
        }

        if ((((scratch.values[1320] != 0.0) && (scratch.values[1337] != 0.0)) && (!(scratch.values[1607] != 0.0))) && (!(scratch.values[1618] != 0.0))) {
            scratch.store_ad(1380, &AdValue::scale(AdValue::mul(AdValue::mul(AdValue::mul(scratch.ad_value(526), scratch.ad_value(1381)), scratch.ad_value(1381)), scratch.ad_value(1356)), scratch.values[404]));
        }

        scratch.values[1622] = if (scratch.values[413] > 1000.0) { 1.0 } else { 0.0 };

        if ((((scratch.values[1320] != 0.0) && (scratch.values[1337] != 0.0)) && (!(scratch.values[1607] != 0.0))) && (scratch.values[1622] != 0.0)) {
            scratch.values[1382] = 1.0;
            scratch.node_derivatives[1382] = [0.0; Instance::NODE_COUNT];
            scratch.branch_derivatives[1382] = [0.0; Instance::BRANCH_COUNT];
        }

        scratch.values[1623] = if (scratch.values[1355] > ((-scratch.values[500]) * scratch.values[413])) { 1.0 } else { 0.0 };

        scratch.values[1624] = if (scratch.values[416] == 4.0) { 1.0 } else { 0.0 };

        if ((((((scratch.values[1320] != 0.0) && (scratch.values[1337] != 0.0)) && (!(scratch.values[1607] != 0.0))) && (!(scratch.values[1622] != 0.0))) && (scratch.values[1623] != 0.0)) && (scratch.values[1624] != 0.0)) {
            scratch.store_ad(1356, &AdValue::mul(AdValue::mul(AdValue::mul(AdValue::scale(scratch.ad_value(1355), scratch.values[506]), AdValue::scale(scratch.ad_value(1355), scratch.values[506])), AdValue::scale(scratch.ad_value(1355), scratch.values[506])), AdValue::scale(scratch.ad_value(1355), scratch.values[506])));
        }

        if ((((((scratch.values[1320] != 0.0) && (scratch.values[1337] != 0.0)) && (!(scratch.values[1607] != 0.0))) && (!(scratch.values[1622] != 0.0))) && (scratch.values[1623] != 0.0)) && (!(scratch.values[1624] != 0.0))) {
            scratch.store_ad(1356, &AdValue::powf(AdValue::abs(AdValue::scale(scratch.ad_value(1355), scratch.values[506])), scratch.values[416]));
        }

        if (((((scratch.values[1320] != 0.0) && (scratch.values[1337] != 0.0)) && (!(scratch.values[1607] != 0.0))) && (!(scratch.values[1622] != 0.0))) && (scratch.values[1623] != 0.0)) {
            scratch.store_ad(1382, &AdValue::div_from_scalar(1.0, AdValue::sub_from_scalar(1.0, scratch.ad_value(1356))));
        }

        if (((((scratch.values[1320] != 0.0) && (scratch.values[1337] != 0.0)) && (!(scratch.values[1607] != 0.0))) && (!(scratch.values[1622] != 0.0))) && (!(scratch.values[1623] != 0.0))) {
            scratch.store_ad(1382, &AdValue::offset(AdValue::scale(AdValue::offset(scratch.ad_value(1355), (scratch.values[500] * scratch.values[413])), scratch.values[509]), scratch.values[503]));
        }

        if (((scratch.values[1320] != 0.0) && (scratch.values[1337] != 0.0)) && (!(scratch.values[1607] != 0.0))) {
            scratch.store_ad(1385, &AdValue::mul(AdValue::add(AdValue::add(AdValue::add(scratch.ad_value(1357), scratch.ad_value(1358)), scratch.ad_value(1365)), scratch.ad_value(1380)), scratch.ad_value(1382)));
        }

        if ((scratch.values[1320] != 0.0) && (scratch.values[1337] != 0.0)) {
            scratch.store_ad(516, &AdValue::add(AdValue::add(AdValue::mul(scratch.ad_value(697), scratch.ad_value(1383)), AdValue::mul(scratch.ad_value(698), scratch.ad_value(1384))), AdValue::mul(scratch.ad_value(699), scratch.ad_value(1385))));
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

        scratch.values[1625] = if !(((scratch.values[697] == 0.0) && (scratch.values[698] == 0.0)) && (scratch.values[699] == 0.0)) { 1.0 } else { 0.0 };

        if (((scratch.values[1320] != 0.0) && (scratch.values[1337] != 0.0)) && (scratch.values[1625] != 0.0)) {
            scratch.store_ad(1341, &AdValue::mul(AdValue::scale(scratch.ad_value(708), 4.0), scratch.ad_value(708)));
        }

        if (((scratch.values[1320] != 0.0) && (scratch.values[1337] != 0.0)) && (scratch.values[1625] != 0.0)) {
            scratch.store_ad(1342, &AdValue::div(scratch.ad_value(708), scratch.ad_value(709)));
        }

        if (((scratch.values[1320] != 0.0) && (scratch.values[1337] != 0.0)) && (scratch.values[1625] != 0.0)) {
            scratch.store_ad(1343, &AdValue::add(scratch.ad_value(527), AdValue::mul(scratch.ad_value(708), scratch.ad_value(1342))));
        }

        if (((scratch.values[1320] != 0.0) && (scratch.values[1337] != 0.0)) && (scratch.values[1625] != 0.0)) {
            scratch.store_ad(1344, &AdValue::add(scratch.ad_value(709), scratch.ad_value(1343)));
        }

        if (((scratch.values[1320] != 0.0) && (scratch.values[1337] != 0.0)) && (scratch.values[1625] != 0.0)) {
            scratch.store_ad(1345, &AdValue::sub(scratch.ad_value(709), scratch.ad_value(1343)));
        }

        if (((scratch.values[1320] != 0.0) && (scratch.values[1337] != 0.0)) && (scratch.values[1625] != 0.0)) {
            scratch.store_ad(1346, &AdValue::sqrt(AdValue::add(AdValue::square(scratch.ad_value(1345)), scratch.ad_value(1341))));
        }

    }

    pub(super) fn stamp_transient_block_18(
        &mut self,
        ctx: &GeneratedEvalContext<'_>,
        stamper: &mut GeneratedStamper<'_>,
        scratch: &mut Scratch,
    ) {
        let _ = stamper;
        if (((scratch.values[1320] != 0.0) && (scratch.values[1337] != 0.0)) && (scratch.values[1625] != 0.0)) {
            scratch.store_ad(1348, &AdValue::scale(AdValue::div(AdValue::mul(scratch.ad_value(527), scratch.ad_value(709)), AdValue::add(scratch.ad_value(1344), scratch.ad_value(1346))), 2.0));
        }

        scratch.values[1626] = if (scratch.values[527] < scratch.values[705]) { 1.0 } else { 0.0 };

        scratch.values[1627] = if ((((0.5 * (scratch.values[527] * scratch.values[427]))) as f64).abs() < 230.25850929940458) { 1.0 } else { 0.0 };

        if (((((scratch.values[1320] != 0.0) && (scratch.values[1337] != 0.0)) && (scratch.values[1625] != 0.0)) && (scratch.values[1626] != 0.0)) && (scratch.values[1627] != 0.0)) {
            scratch.store_ad(1350, &AdValue::exp(AdValue::scale(scratch.ad_value(527), (scratch.values[427] * 0.5))));
        }

        scratch.values[1628] = if ((0.5 * (scratch.values[527] * scratch.values[427])) < 0.0) { 1.0 } else { 0.0 };

        if ((((((scratch.values[1320] != 0.0) && (scratch.values[1337] != 0.0)) && (scratch.values[1625] != 0.0)) && (scratch.values[1626] != 0.0)) && (!(scratch.values[1627] != 0.0))) && (scratch.values[1628] != 0.0)) {
            let assign23160_ad_e26304: AdValue = AdValue::div_from_scalar(1e-100, AdValue::offset(AdValue::mul(AdValue::sub_from_scalar((-230.25850929940458), AdValue::scale(scratch.ad_value(527), (scratch.values[427] * 0.5))), AdValue::offset(AdValue::scale(AdValue::mul(AdValue::sub_from_scalar((-230.25850929940458), AdValue::scale(scratch.ad_value(527), (scratch.values[427] * 0.5))), AdValue::offset(AdValue::scale(AdValue::sub_from_scalar((-230.25850929940458), AdValue::scale(scratch.ad_value(527), (scratch.values[427] * 0.5))), 0.3333333333333333), 1.0)), 0.5), 1.0)), 1.0));
            scratch.store_ad(1350, &assign23160_ad_e26304);
        }

        if ((((((scratch.values[1320] != 0.0) && (scratch.values[1337] != 0.0)) && (scratch.values[1625] != 0.0)) && (scratch.values[1626] != 0.0)) && (!(scratch.values[1627] != 0.0))) && (!(scratch.values[1628] != 0.0))) {
            scratch.store_ad(1350, &AdValue::scale(AdValue::offset(AdValue::mul(AdValue::offset(AdValue::scale(scratch.ad_value(527), (scratch.values[427] * 0.5)), (-230.25850929940458)), AdValue::offset(AdValue::scale(AdValue::mul(AdValue::offset(AdValue::scale(scratch.ad_value(527), (scratch.values[427] * 0.5)), (-230.25850929940458)), AdValue::offset(AdValue::scale(AdValue::offset(AdValue::scale(scratch.ad_value(527), (scratch.values[427] * 0.5)), (-230.25850929940458)), 0.3333333333333333), 1.0)), 0.5), 1.0)), 1.0), 1e100));
        }

        if ((((scratch.values[1320] != 0.0) && (scratch.values[1337] != 0.0)) && (scratch.values[1625] != 0.0)) && (scratch.values[1626] != 0.0)) {
            scratch.store_ad(1347, &AdValue::square(scratch.ad_value(1350)));
        }

        if ((((scratch.values[1320] != 0.0) && (scratch.values[1337] != 0.0)) && (scratch.values[1625] != 0.0)) && (!(scratch.values[1626] != 0.0))) {
            scratch.store_ad(1347, &AdValue::mul(AdValue::offset(AdValue::scale(AdValue::sub(scratch.ad_value(527), scratch.ad_value(705)), scratch.values[427]), 1.0), scratch.ad_value(706)));
        }

        if ((((scratch.values[1320] != 0.0) && (scratch.values[1337] != 0.0)) && (scratch.values[1625] != 0.0)) && (!(scratch.values[1626] != 0.0))) {
            scratch.store_ad(1350, &AdValue::sqrt(scratch.ad_value(1347)));
        }

        if (((scratch.values[1320] != 0.0) && (scratch.values[1337] != 0.0)) && (scratch.values[1625] != 0.0)) {
            scratch.store_ad(1347, &AdValue::offset(scratch.ad_value(1347), (-1.0)));
        }

        if (((scratch.values[1320] != 0.0) && (scratch.values[1337] != 0.0)) && (scratch.values[1625] != 0.0)) {
            scratch.store_ad(1349, &AdValue::div_from_scalar(1.0, scratch.ad_value(1350)));
        }

        scratch.values[1629] = if (scratch.values[527] > 0.0) { 1.0 } else { 0.0 };

        if ((((scratch.values[1320] != 0.0) && (scratch.values[1337] != 0.0)) && (scratch.values[1625] != 0.0)) && (scratch.values[1629] != 0.0)) {
            scratch.store_ad(1351, &AdValue::scale(AdValue::ln(AdValue::add(AdValue::offset(scratch.ad_value(1349), 2.0), AdValue::sqrt(AdValue::mul(AdValue::offset(scratch.ad_value(1349), 1.0), AdValue::offset(scratch.ad_value(1349), 3.0))))), (scratch.values[426] * 2.0)));
        }

        if ((((scratch.values[1320] != 0.0) && (scratch.values[1337] != 0.0)) && (scratch.values[1625] != 0.0)) && (!(scratch.values[1629] != 0.0))) {
            scratch.store_ad(1351, &AdValue::sub(AdValue::scale(AdValue::ln(AdValue::add(AdValue::offset(AdValue::scale(scratch.ad_value(1350), 2.0), 1.0), AdValue::sqrt(AdValue::mul(AdValue::offset(scratch.ad_value(1350), 1.0), AdValue::offset(AdValue::scale(scratch.ad_value(1350), 3.0), 1.0))))), (scratch.values[426] * 2.0)), scratch.ad_value(527)));
        }

        if (((scratch.values[1320] != 0.0) && (scratch.values[1337] != 0.0)) && (scratch.values[1625] != 0.0)) {
            scratch.store_ad(1352, &AdValue::sub(scratch.ad_value(707), scratch.ad_value(1351)));
        }

        if (((scratch.values[1320] != 0.0) && (scratch.values[1337] != 0.0)) && (scratch.values[1625] != 0.0)) {
            scratch.store_ad(1353, &AdValue::scale(AdValue::sub(AdValue::add(scratch.ad_value(527), scratch.ad_value(1352)), AdValue::sqrt(AdValue::offset(AdValue::mul(AdValue::sub(scratch.ad_value(527), scratch.ad_value(1352)), AdValue::sub(scratch.ad_value(527), scratch.ad_value(1352))), ((4.0 * scratch.values[426]) * scratch.values[426])))), 0.5));
        }

        if (((scratch.values[1320] != 0.0) && (scratch.values[1337] != 0.0)) && (scratch.values[1625] != 0.0)) {
            scratch.store_ad(1354, &AdValue::scale(AdValue::sub(AdValue::add(scratch.ad_value(527), scratch.ad_value(710)), AdValue::sqrt(AdValue::offset(AdValue::mul(AdValue::sub(scratch.ad_value(527), scratch.ad_value(710)), AdValue::sub(scratch.ad_value(527), scratch.ad_value(710))), ((4.0 * scratch.values[424]) * scratch.values[424])))), 0.5));
        }

        if (((scratch.values[1320] != 0.0) && (scratch.values[1337] != 0.0)) && (scratch.values[1625] != 0.0)) {
            scratch.store_ad(1355, &AdValue::scale(AdValue::sub(scratch.ad_value(527), AdValue::sqrt(AdValue::offset(AdValue::mul(scratch.ad_value(527), scratch.ad_value(527)), ((4.0 * 1e-6) * 1e-6)))), 0.5));
        }

        scratch.values[1630] = if (scratch.values[697] == 0.0) { 1.0 } else { 0.0 };

        if (((scratch.values[1320] != 0.0) && (scratch.values[1337] != 0.0)) && (scratch.values[1630] != 0.0)) {
            scratch.values[1383] = 0.0;
            scratch.node_derivatives[1383] = [0.0; Instance::NODE_COUNT];
            scratch.branch_derivatives[1383] = [0.0; Instance::BRANCH_COUNT];
        }

        scratch.values[1631] = if (scratch.values[464] == 0.5) { 1.0 } else { 0.0 };

        if ((((scratch.values[1320] != 0.0) && (scratch.values[1337] != 0.0)) && (!(scratch.values[1630] != 0.0))) && (scratch.values[1631] != 0.0)) {
            scratch.store_ad(1356, &AdValue::sqrt(AdValue::sub_from_scalar(1.0, AdValue::scale(scratch.ad_value(1348), scratch.values[461]))));
        }

        if ((((scratch.values[1320] != 0.0) && (scratch.values[1337] != 0.0)) && (!(scratch.values[1630] != 0.0))) && (!(scratch.values[1631] != 0.0))) {
            scratch.store_ad(1356, &AdValue::powf(AdValue::sub_from_scalar(1.0, AdValue::scale(scratch.ad_value(1348), scratch.values[461])), scratch.values[464]));
        }

        if (((scratch.values[1320] != 0.0) && (scratch.values[1337] != 0.0)) && (!(scratch.values[1630] != 0.0))) {
            scratch.store_ad(1357, &AdValue::scale(scratch.ad_value(1347), scratch.values[443]));
        }

        scratch.values[1632] = if ((scratch.values[393] == 0.0) && (scratch.values[396] == 0.0)) { 1.0 } else { 0.0 };

        if ((((scratch.values[1320] != 0.0) && (scratch.values[1337] != 0.0)) && (!(scratch.values[1630] != 0.0))) && (scratch.values[1632] != 0.0)) {
            scratch.values[1358] = 0.0;
            scratch.node_derivatives[1358] = [0.0; Instance::NODE_COUNT];
            scratch.branch_derivatives[1358] = [0.0; Instance::BRANCH_COUNT];
        }

        if ((((scratch.values[1320] != 0.0) && (scratch.values[1337] != 0.0)) && (!(scratch.values[1630] != 0.0))) && (!(scratch.values[1632] != 0.0))) {
            scratch.store_ad(1359, &AdValue::sub_from_scalar(scratch.values[449], scratch.ad_value(1353)));
        }

        if ((((scratch.values[1320] != 0.0) && (scratch.values[1337] != 0.0)) && (!(scratch.values[1630] != 0.0))) && (!(scratch.values[1632] != 0.0))) {
            scratch.store_ad(1360, &AdValue::sub_from_scalar(1.0, AdValue::sqrt(AdValue::sub_from_scalar(1.0, AdValue::div(scratch.ad_value(1351), scratch.ad_value(1359))))));
        }

        scratch.values[1633] = if (scratch.values[382] == 0.5) { 1.0 } else { 0.0 };

        if (((((scratch.values[1320] != 0.0) && (scratch.values[1337] != 0.0)) && (!(scratch.values[1630] != 0.0))) && (!(scratch.values[1632] != 0.0))) && (scratch.values[1633] != 0.0)) {
            scratch.values[1361] = 0.0;
            scratch.node_derivatives[1361] = [0.0; Instance::NODE_COUNT];
            scratch.branch_derivatives[1361] = [0.0; Instance::BRANCH_COUNT];
        }

        if (((((scratch.values[1320] != 0.0) && (scratch.values[1337] != 0.0)) && (!(scratch.values[1630] != 0.0))) && (!(scratch.values[1632] != 0.0))) && (!(scratch.values[1633] != 0.0))) {
            scratch.store_ad(1361, &AdValue::scale(AdValue::add(AdValue::div(AdValue::mul(AdValue::square(scratch.ad_value(1360)), AdValue::ln(scratch.ad_value(1360))), AdValue::sub_from_scalar(1.0, scratch.ad_value(1360))), scratch.ad_value(1360)), (1.0 - (2.0 * scratch.values[382]))));
        }

        if ((((scratch.values[1320] != 0.0) && (scratch.values[1337] != 0.0)) && (!(scratch.values[1630] != 0.0))) && (!(scratch.values[1632] != 0.0))) {
            scratch.store_ad(1362, &AdValue::add(scratch.ad_value(1360), scratch.ad_value(1361)));
        }

        scratch.values[1634] = if (scratch.values[382] == 0.5) { 1.0 } else { 0.0 };

        if (((((scratch.values[1320] != 0.0) && (scratch.values[1337] != 0.0)) && (!(scratch.values[1630] != 0.0))) && (!(scratch.values[1632] != 0.0))) && (scratch.values[1634] != 0.0)) {
            scratch.store_ad(1356, &AdValue::sqrt(AdValue::scale(scratch.ad_value(1359), scratch.values[485])));
        }

        if (((((scratch.values[1320] != 0.0) && (scratch.values[1337] != 0.0)) && (!(scratch.values[1630] != 0.0))) && (!(scratch.values[1632] != 0.0))) && (!(scratch.values[1634] != 0.0))) {
            scratch.store_ad(1356, &AdValue::powf(AdValue::scale(scratch.ad_value(1359), scratch.values[485]), scratch.values[382]));
        }

        if ((((scratch.values[1320] != 0.0) && (scratch.values[1337] != 0.0)) && (!(scratch.values[1630] != 0.0))) && (!(scratch.values[1632] != 0.0))) {
            scratch.store_ad(1363, &AdValue::scale(scratch.ad_value(1356), scratch.values[479]));
        }

        if ((((scratch.values[1320] != 0.0) && (scratch.values[1337] != 0.0)) && (!(scratch.values[1630] != 0.0))) && (!(scratch.values[1632] != 0.0))) {
            scratch.store_ad(1364, &AdValue::scale(AdValue::mul(AdValue::offset(scratch.ad_value(1350), (-1.0)), scratch.ad_value(1363)), scratch.values[440]));
        }

        if ((((scratch.values[1320] != 0.0) && (scratch.values[1337] != 0.0)) && (!(scratch.values[1630] != 0.0))) && (!(scratch.values[1632] != 0.0))) {
            scratch.store_ad(1358, &AdValue::scale(AdValue::mul(scratch.ad_value(1364), scratch.ad_value(1362)), scratch.values[393]));
        }

        scratch.values[1635] = if (scratch.values[396] == 0.0) { 1.0 } else { 0.0 };

        if ((((scratch.values[1320] != 0.0) && (scratch.values[1337] != 0.0)) && (!(scratch.values[1630] != 0.0))) && (scratch.values[1635] != 0.0)) {
            scratch.values[1365] = 0.0;
            scratch.node_derivatives[1365] = [0.0; Instance::NODE_COUNT];
            scratch.branch_derivatives[1365] = [0.0; Instance::BRANCH_COUNT];
        }

        if ((((scratch.values[1320] != 0.0) && (scratch.values[1337] != 0.0)) && (!(scratch.values[1630] != 0.0))) && (!(scratch.values[1635] != 0.0))) {
            scratch.store_ad(1366, &AdValue::scale(AdValue::div(AdValue::scale(scratch.ad_value(1363), scratch.values[464]), scratch.ad_value(1359)), scratch.values[494]));
        }

        if ((((scratch.values[1320] != 0.0) && (scratch.values[1337] != 0.0)) && (!(scratch.values[1630] != 0.0))) && (!(scratch.values[1635] != 0.0))) {
            scratch.store_ad(1367, &AdValue::div_from_scalar((0.666666666666667 * scratch.values[491]), scratch.ad_value(1366)));
        }

        if ((((scratch.values[1320] != 0.0) && (scratch.values[1337] != 0.0)) && (!(scratch.values[1630] != 0.0))) && (!(scratch.values[1635] != 0.0))) {
            scratch.store_ad(1368, &AdValue::square(scratch.ad_value(1367)));
        }

        if ((((scratch.values[1320] != 0.0) && (scratch.values[1337] != 0.0)) && (!(scratch.values[1630] != 0.0))) && (!(scratch.values[1635] != 0.0))) {
            scratch.store_ad(1369, &AdValue::sqrt(AdValue::div(AdValue::square(scratch.ad_value(1368)), AdValue::offset(AdValue::square(scratch.ad_value(1368)), 1.0))));
        }

        if ((((scratch.values[1320] != 0.0) && (scratch.values[1337] != 0.0)) && (!(scratch.values[1630] != 0.0))) && (!(scratch.values[1635] != 0.0))) {
            scratch.store_ad(1370, &AdValue::sqrt(AdValue::abs(scratch.ad_value(1369))));
        }

        if ((((scratch.values[1320] != 0.0) && (scratch.values[1337] != 0.0)) && (!(scratch.values[1630] != 0.0))) && (!(scratch.values[1635] != 0.0))) {
            scratch.store_ad(1371, &AdValue::mul(scratch.ad_value(1369), scratch.ad_value(1370)));
        }

        scratch.values[1636] = if (((-scratch.values[382]) * scratch.values[467]) == (-1.0)) { 1.0 } else { 0.0 };

        if (((((scratch.values[1320] != 0.0) && (scratch.values[1337] != 0.0)) && (!(scratch.values[1630] != 0.0))) && (!(scratch.values[1635] != 0.0))) && (scratch.values[1636] != 0.0)) {
            scratch.store_ad(1372, &AdValue::div_from_scalar(1.0, AdValue::offset(AdValue::mul(scratch.ad_value(1366), scratch.ad_value(1371)), 1.0)));
        }

        if (((((scratch.values[1320] != 0.0) && (scratch.values[1337] != 0.0)) && (!(scratch.values[1630] != 0.0))) && (!(scratch.values[1635] != 0.0))) && (!(scratch.values[1636] != 0.0))) {
            scratch.store_ad(1372, &AdValue::powf(AdValue::offset(AdValue::mul(scratch.ad_value(1366), scratch.ad_value(1371)), 1.0), ((-scratch.values[382]) * scratch.values[467])));
        }

        if ((((scratch.values[1320] != 0.0) && (scratch.values[1337] != 0.0)) && (!(scratch.values[1630] != 0.0))) && (!(scratch.values[1635] != 0.0))) {
            scratch.store_ad(1373, &AdValue::div(AdValue::mul(scratch.ad_value(1362), scratch.ad_value(1372)), AdValue::add(scratch.ad_value(1362), scratch.ad_value(1372))));
        }

        if ((((scratch.values[1320] != 0.0) && (scratch.values[1337] != 0.0)) && (!(scratch.values[1630] != 0.0))) && (!(scratch.values[1635] != 0.0))) {
            scratch.store_ad(1374, &AdValue::sqrt(AdValue::scale(AdValue::div(scratch.ad_value(1366), scratch.ad_value(1370)), 0.375)));
        }

        if ((((scratch.values[1320] != 0.0) && (scratch.values[1337] != 0.0)) && (!(scratch.values[1630] != 0.0))) && (!(scratch.values[1635] != 0.0))) {
            scratch.store_ad(1375, &AdValue::sub(AdValue::scale(AdValue::mul(scratch.ad_value(1367), scratch.ad_value(1370)), 2.0), scratch.ad_value(1369)));
        }

        if ((((scratch.values[1320] != 0.0) && (scratch.values[1337] != 0.0)) && (!(scratch.values[1630] != 0.0))) && (!(scratch.values[1635] != 0.0))) {
            scratch.store_ad(1376, &AdValue::add(AdValue::sub(AdValue::mul(AdValue::scale(scratch.ad_value(1367), scratch.values[491]), scratch.ad_value(1370)), AdValue::scale(scratch.ad_value(1369), scratch.values[491])), AdValue::scale(AdValue::mul(scratch.ad_value(1366), scratch.ad_value(1371)), 0.5)));
        }

        if ((((scratch.values[1320] != 0.0) && (scratch.values[1337] != 0.0)) && (!(scratch.values[1630] != 0.0))) && (!(scratch.values[1635] != 0.0))) {
            scratch.store_ad(1377, &AdValue::mul(AdValue::offset(scratch.ad_value(1375), (-1.0)), scratch.ad_value(1374)));
        }

        if ((((scratch.values[1320] != 0.0) && (scratch.values[1337] != 0.0)) && (!(scratch.values[1630] != 0.0))) && (!(scratch.values[1635] != 0.0))) {
            scratch.store_ad(1338, &AdValue::square(scratch.ad_value(1377)));
        }

        scratch.values[1637] = if (scratch.values[1377] > 0.0) { 1.0 } else { 0.0 };

        if (((((scratch.values[1320] != 0.0) && (scratch.values[1337] != 0.0)) && (!(scratch.values[1630] != 0.0))) && (!(scratch.values[1635] != 0.0))) && (scratch.values[1637] != 0.0)) {
            scratch.store_ad(1339, &AdValue::div_from_scalar(1.0, AdValue::offset(AdValue::scale(scratch.ad_value(1377), scratch.values[428]), 1.0)));
        }

        if (((((scratch.values[1320] != 0.0) && (scratch.values[1337] != 0.0)) && (!(scratch.values[1630] != 0.0))) && (!(scratch.values[1635] != 0.0))) && (!(scratch.values[1637] != 0.0))) {
            scratch.store_ad(1339, &AdValue::div_from_scalar(1.0, AdValue::sub_from_scalar(1.0, AdValue::scale(scratch.ad_value(1377), scratch.values[428]))));
        }

        scratch.values[1638] = if (((-scratch.values[1338]) + scratch.values[1376]) > (-230.25850929940458)) { 1.0 } else { 0.0 };

        if (((((scratch.values[1320] != 0.0) && (scratch.values[1337] != 0.0)) && (!(scratch.values[1630] != 0.0))) && (!(scratch.values[1635] != 0.0))) && (scratch.values[1638] != 0.0)) {
            scratch.store_ad(1356, &AdValue::exp(AdValue::sub(scratch.ad_value(1376), scratch.ad_value(1338))));
        }

        if (((((scratch.values[1320] != 0.0) && (scratch.values[1337] != 0.0)) && (!(scratch.values[1630] != 0.0))) && (!(scratch.values[1635] != 0.0))) && (!(scratch.values[1638] != 0.0))) {
            let assign23740_ad_e27247: AdValue = AdValue::div_from_scalar(1e-100, AdValue::offset(AdValue::mul(AdValue::sub_from_scalar((-230.25850929940458), AdValue::sub(scratch.ad_value(1376), scratch.ad_value(1338))), AdValue::offset(AdValue::scale(AdValue::mul(AdValue::sub_from_scalar((-230.25850929940458), AdValue::sub(scratch.ad_value(1376), scratch.ad_value(1338))), AdValue::offset(AdValue::scale(AdValue::sub_from_scalar((-230.25850929940458), AdValue::sub(scratch.ad_value(1376), scratch.ad_value(1338))), 0.3333333333333333), 1.0)), 0.5), 1.0)), 1.0));
            scratch.store_ad(1356, &assign23740_ad_e27247);
        }

        if ((((scratch.values[1320] != 0.0) && (scratch.values[1337] != 0.0)) && (!(scratch.values[1630] != 0.0))) && (!(scratch.values[1635] != 0.0))) {
            scratch.store_ad(1340, &AdValue::mul(AdValue::add(AdValue::add(AdValue::scale(scratch.ad_value(1339), 0.29214664), AdValue::scale(AdValue::square(scratch.ad_value(1339)), scratch.values[429])), AdValue::scale(AdValue::mul(AdValue::square(scratch.ad_value(1339)), scratch.ad_value(1339)), scratch.values[430])), scratch.ad_value(1356)));
        }

        scratch.values[1639] = if (scratch.values[1377] > 0.0) { 1.0 } else { 0.0 };

        if (((((scratch.values[1320] != 0.0) && (scratch.values[1337] != 0.0)) && (!(scratch.values[1630] != 0.0))) && (!(scratch.values[1635] != 0.0))) && (scratch.values[1639] != 0.0)) {
            scratch.values[1378] = scratch.values[1340];
            scratch.node_derivatives[1378] = scratch.node_derivatives[1340];
            scratch.branch_derivatives[1378] = scratch.branch_derivatives[1340];
        }

        scratch.values[1640] = if (scratch.values[1376] > (-230.25850929940458)) { 1.0 } else { 0.0 };

        if ((((((scratch.values[1320] != 0.0) && (scratch.values[1337] != 0.0)) && (!(scratch.values[1630] != 0.0))) && (!(scratch.values[1635] != 0.0))) && (!(scratch.values[1639] != 0.0))) && (scratch.values[1640] != 0.0)) {
            scratch.store_ad(1356, &AdValue::exp(scratch.ad_value(1376)));
        }

        if ((((((scratch.values[1320] != 0.0) && (scratch.values[1337] != 0.0)) && (!(scratch.values[1630] != 0.0))) && (!(scratch.values[1635] != 0.0))) && (!(scratch.values[1639] != 0.0))) && (!(scratch.values[1640] != 0.0))) {
            scratch.store_ad(1356, &AdValue::div_from_scalar(1e-100, AdValue::offset(AdValue::mul(AdValue::sub_from_scalar((-230.25850929940458), scratch.ad_value(1376)), AdValue::offset(AdValue::scale(AdValue::mul(AdValue::sub_from_scalar((-230.25850929940458), scratch.ad_value(1376)), AdValue::offset(AdValue::scale(AdValue::sub_from_scalar((-230.25850929940458), scratch.ad_value(1376)), 0.3333333333333333), 1.0)), 0.5), 1.0)), 1.0)));
        }

        if (((((scratch.values[1320] != 0.0) && (scratch.values[1337] != 0.0)) && (!(scratch.values[1630] != 0.0))) && (!(scratch.values[1635] != 0.0))) && (!(scratch.values[1639] != 0.0))) {
            scratch.store_ad(1378, &AdValue::sub(AdValue::scale(scratch.ad_value(1356), 2.0), scratch.ad_value(1340)));
        }

        if ((((scratch.values[1320] != 0.0) && (scratch.values[1337] != 0.0)) && (!(scratch.values[1630] != 0.0))) && (!(scratch.values[1635] != 0.0))) {
            scratch.store_ad(1379, &AdValue::scale(AdValue::div(AdValue::scale(scratch.ad_value(1378), scratch.values[491]), scratch.ad_value(1374)), (1.772453850905516 * 0.5)));
        }

        if ((((scratch.values[1320] != 0.0) && (scratch.values[1337] != 0.0)) && (!(scratch.values[1630] != 0.0))) && (!(scratch.values[1635] != 0.0))) {
            scratch.store_ad(1365, &AdValue::scale(AdValue::mul(AdValue::mul(scratch.ad_value(1364), scratch.ad_value(1379)), scratch.ad_value(1373)), scratch.values[396]));
        }

        scratch.values[1641] = if (scratch.values[402] == 0.0) { 1.0 } else { 0.0 };

        if ((((scratch.values[1320] != 0.0) && (scratch.values[1337] != 0.0)) && (!(scratch.values[1630] != 0.0))) && (scratch.values[1641] != 0.0)) {
            scratch.values[1380] = 0.0;
            scratch.node_derivatives[1380] = [0.0; Instance::NODE_COUNT];
            scratch.branch_derivatives[1380] = [0.0; Instance::BRANCH_COUNT];
        }

        scratch.values[1642] = if (scratch.values[382] == 0.5) { 1.0 } else { 0.0 };

        if (((((scratch.values[1320] != 0.0) && (scratch.values[1337] != 0.0)) && (!(scratch.values[1630] != 0.0))) && (!(scratch.values[1641] != 0.0))) && (scratch.values[1642] != 0.0)) {
            scratch.store_ad(1356, &AdValue::sqrt(AdValue::scale(AdValue::sub_from_scalar(scratch.values[379], scratch.ad_value(1354)), scratch.values[485])));
        }

        if (((((scratch.values[1320] != 0.0) && (scratch.values[1337] != 0.0)) && (!(scratch.values[1630] != 0.0))) && (!(scratch.values[1641] != 0.0))) && (!(scratch.values[1642] != 0.0))) {
            scratch.store_ad(1356, &AdValue::powf(AdValue::scale(AdValue::sub_from_scalar(scratch.values[379], scratch.ad_value(1354)), scratch.values[485]), scratch.values[382]));
        }

        if ((((scratch.values[1320] != 0.0) && (scratch.values[1337] != 0.0)) && (!(scratch.values[1630] != 0.0))) && (!(scratch.values[1641] != 0.0))) {
            scratch.store_ad(1381, &AdValue::scale(AdValue::div(AdValue::scale(AdValue::sub_from_scalar(scratch.values[379], scratch.ad_value(1354)), scratch.values[482]), scratch.ad_value(1356)), scratch.values[467]));
        }

        scratch.values[1643] = if (((((-scratch.values[497]) / scratch.values[1381])) as f64).abs() < 230.25850929940458) { 1.0 } else { 0.0 };

        if (((((scratch.values[1320] != 0.0) && (scratch.values[1337] != 0.0)) && (!(scratch.values[1630] != 0.0))) && (!(scratch.values[1641] != 0.0))) && (scratch.values[1643] != 0.0)) {
            scratch.store_ad(1356, &AdValue::exp(AdValue::div(AdValue::neg(scratch.ad_value(497)), scratch.ad_value(1381))));
        }

        scratch.values[1644] = if (((-scratch.values[497]) / scratch.values[1381]) < 0.0) { 1.0 } else { 0.0 };

        if ((((((scratch.values[1320] != 0.0) && (scratch.values[1337] != 0.0)) && (!(scratch.values[1630] != 0.0))) && (!(scratch.values[1641] != 0.0))) && (!(scratch.values[1643] != 0.0))) && (scratch.values[1644] != 0.0)) {
            let assign23930_ad_e27574: AdValue = AdValue::offset(AdValue::mul(AdValue::sub_from_scalar((-230.25850929940458), AdValue::div(AdValue::neg(scratch.ad_value(497)), scratch.ad_value(1381))), AdValue::offset(AdValue::scale(AdValue::mul(AdValue::sub_from_scalar((-230.25850929940458), AdValue::div(AdValue::neg(scratch.ad_value(497)), scratch.ad_value(1381))), AdValue::offset(AdValue::scale(AdValue::sub_from_scalar((-230.25850929940458), AdValue::div(AdValue::neg(scratch.ad_value(497)), scratch.ad_value(1381))), 0.3333333333333333), 1.0)), 0.5), 1.0)), 1.0);
            scratch.store_ad(1356, &AdValue::div_from_scalar(1e-100, assign23930_ad_e27574));
        }

        if ((((((scratch.values[1320] != 0.0) && (scratch.values[1337] != 0.0)) && (!(scratch.values[1630] != 0.0))) && (!(scratch.values[1641] != 0.0))) && (!(scratch.values[1643] != 0.0))) && (!(scratch.values[1644] != 0.0))) {
            let assign23940_ad_e27624: AdValue = AdValue::scale(AdValue::offset(AdValue::mul(AdValue::offset(AdValue::div(AdValue::neg(scratch.ad_value(497)), scratch.ad_value(1381)), (-230.25850929940458)), AdValue::offset(AdValue::scale(AdValue::mul(AdValue::offset(AdValue::div(AdValue::neg(scratch.ad_value(497)), scratch.ad_value(1381)), (-230.25850929940458)), AdValue::offset(AdValue::scale(AdValue::offset(AdValue::div(AdValue::neg(scratch.ad_value(497)), scratch.ad_value(1381)), (-230.25850929940458)), 0.3333333333333333), 1.0)), 0.5), 1.0)), 1.0), 1e100);
            scratch.store_ad(1356, &assign23940_ad_e27624);
        }

        if ((((scratch.values[1320] != 0.0) && (scratch.values[1337] != 0.0)) && (!(scratch.values[1630] != 0.0))) && (!(scratch.values[1641] != 0.0))) {
            scratch.store_ad(1380, &AdValue::scale(AdValue::mul(AdValue::mul(AdValue::mul(scratch.ad_value(527), scratch.ad_value(1381)), scratch.ad_value(1381)), scratch.ad_value(1356)), scratch.values[402]));
        }

        scratch.values[1645] = if (scratch.values[411] > 1000.0) { 1.0 } else { 0.0 };

        if ((((scratch.values[1320] != 0.0) && (scratch.values[1337] != 0.0)) && (!(scratch.values[1630] != 0.0))) && (scratch.values[1645] != 0.0)) {
            scratch.values[1382] = 1.0;
            scratch.node_derivatives[1382] = [0.0; Instance::NODE_COUNT];
            scratch.branch_derivatives[1382] = [0.0; Instance::BRANCH_COUNT];
        }

        scratch.values[1646] = if (scratch.values[1355] > ((-scratch.values[500]) * scratch.values[411])) { 1.0 } else { 0.0 };

        scratch.values[1647] = if (scratch.values[414] == 4.0) { 1.0 } else { 0.0 };

        if ((((((scratch.values[1320] != 0.0) && (scratch.values[1337] != 0.0)) && (!(scratch.values[1630] != 0.0))) && (!(scratch.values[1645] != 0.0))) && (scratch.values[1646] != 0.0)) && (scratch.values[1647] != 0.0)) {
            scratch.store_ad(1356, &AdValue::mul(AdValue::mul(AdValue::mul(AdValue::scale(scratch.ad_value(1355), scratch.values[504]), AdValue::scale(scratch.ad_value(1355), scratch.values[504])), AdValue::scale(scratch.ad_value(1355), scratch.values[504])), AdValue::scale(scratch.ad_value(1355), scratch.values[504])));
        }

        if ((((((scratch.values[1320] != 0.0) && (scratch.values[1337] != 0.0)) && (!(scratch.values[1630] != 0.0))) && (!(scratch.values[1645] != 0.0))) && (scratch.values[1646] != 0.0)) && (!(scratch.values[1647] != 0.0))) {
            scratch.store_ad(1356, &AdValue::powf(AdValue::abs(AdValue::scale(scratch.ad_value(1355), scratch.values[504])), scratch.values[414]));
        }

        if (((((scratch.values[1320] != 0.0) && (scratch.values[1337] != 0.0)) && (!(scratch.values[1630] != 0.0))) && (!(scratch.values[1645] != 0.0))) && (scratch.values[1646] != 0.0)) {
            scratch.store_ad(1382, &AdValue::div_from_scalar(1.0, AdValue::sub_from_scalar(1.0, scratch.ad_value(1356))));
        }

        if (((((scratch.values[1320] != 0.0) && (scratch.values[1337] != 0.0)) && (!(scratch.values[1630] != 0.0))) && (!(scratch.values[1645] != 0.0))) && (!(scratch.values[1646] != 0.0))) {
            scratch.store_ad(1382, &AdValue::offset(AdValue::scale(AdValue::offset(scratch.ad_value(1355), (scratch.values[500] * scratch.values[411])), scratch.values[507]), scratch.values[501]));
        }

        if (((scratch.values[1320] != 0.0) && (scratch.values[1337] != 0.0)) && (!(scratch.values[1630] != 0.0))) {
            scratch.store_ad(1383, &AdValue::mul(AdValue::add(AdValue::add(AdValue::add(scratch.ad_value(1357), scratch.ad_value(1358)), scratch.ad_value(1365)), scratch.ad_value(1380)), scratch.ad_value(1382)));
        }

        scratch.values[1648] = if (scratch.values[698] == 0.0) { 1.0 } else { 0.0 };

        if (((scratch.values[1320] != 0.0) && (scratch.values[1337] != 0.0)) && (scratch.values[1648] != 0.0)) {
            scratch.values[1384] = 0.0;
            scratch.node_derivatives[1384] = [0.0; Instance::NODE_COUNT];
            scratch.branch_derivatives[1384] = [0.0; Instance::BRANCH_COUNT];
        }

        scratch.values[1649] = if (scratch.values[465] == 0.5) { 1.0 } else { 0.0 };

        if ((((scratch.values[1320] != 0.0) && (scratch.values[1337] != 0.0)) && (!(scratch.values[1648] != 0.0))) && (scratch.values[1649] != 0.0)) {
            scratch.store_ad(1356, &AdValue::sqrt(AdValue::sub_from_scalar(1.0, AdValue::scale(scratch.ad_value(1348), scratch.values[462]))));
        }

        if ((((scratch.values[1320] != 0.0) && (scratch.values[1337] != 0.0)) && (!(scratch.values[1648] != 0.0))) && (!(scratch.values[1649] != 0.0))) {
            scratch.store_ad(1356, &AdValue::powf(AdValue::sub_from_scalar(1.0, AdValue::scale(scratch.ad_value(1348), scratch.values[462])), scratch.values[465]));
        }

        if (((scratch.values[1320] != 0.0) && (scratch.values[1337] != 0.0)) && (!(scratch.values[1648] != 0.0))) {
            scratch.store_ad(1357, &AdValue::scale(scratch.ad_value(1347), scratch.values[444]));
        }

        scratch.values[1650] = if ((scratch.values[394] == 0.0) && (scratch.values[397] == 0.0)) { 1.0 } else { 0.0 };

        if ((((scratch.values[1320] != 0.0) && (scratch.values[1337] != 0.0)) && (!(scratch.values[1648] != 0.0))) && (scratch.values[1650] != 0.0)) {
            scratch.values[1358] = 0.0;
            scratch.node_derivatives[1358] = [0.0; Instance::NODE_COUNT];
            scratch.branch_derivatives[1358] = [0.0; Instance::BRANCH_COUNT];
        }

        if ((((scratch.values[1320] != 0.0) && (scratch.values[1337] != 0.0)) && (!(scratch.values[1648] != 0.0))) && (!(scratch.values[1650] != 0.0))) {
            scratch.store_ad(1359, &AdValue::sub_from_scalar(scratch.values[450], scratch.ad_value(1353)));
        }

        if ((((scratch.values[1320] != 0.0) && (scratch.values[1337] != 0.0)) && (!(scratch.values[1648] != 0.0))) && (!(scratch.values[1650] != 0.0))) {
            scratch.store_ad(1360, &AdValue::sub_from_scalar(1.0, AdValue::sqrt(AdValue::sub_from_scalar(1.0, AdValue::div(scratch.ad_value(1351), scratch.ad_value(1359))))));
        }

        scratch.values[1651] = if (scratch.values[383] == 0.5) { 1.0 } else { 0.0 };

        if (((((scratch.values[1320] != 0.0) && (scratch.values[1337] != 0.0)) && (!(scratch.values[1648] != 0.0))) && (!(scratch.values[1650] != 0.0))) && (scratch.values[1651] != 0.0)) {
            scratch.values[1361] = 0.0;
            scratch.node_derivatives[1361] = [0.0; Instance::NODE_COUNT];
            scratch.branch_derivatives[1361] = [0.0; Instance::BRANCH_COUNT];
        }

        if (((((scratch.values[1320] != 0.0) && (scratch.values[1337] != 0.0)) && (!(scratch.values[1648] != 0.0))) && (!(scratch.values[1650] != 0.0))) && (!(scratch.values[1651] != 0.0))) {
            scratch.store_ad(1361, &AdValue::scale(AdValue::add(AdValue::div(AdValue::mul(AdValue::square(scratch.ad_value(1360)), AdValue::ln(scratch.ad_value(1360))), AdValue::sub_from_scalar(1.0, scratch.ad_value(1360))), scratch.ad_value(1360)), (1.0 - (2.0 * scratch.values[383]))));
        }

        if ((((scratch.values[1320] != 0.0) && (scratch.values[1337] != 0.0)) && (!(scratch.values[1648] != 0.0))) && (!(scratch.values[1650] != 0.0))) {
            scratch.store_ad(1362, &AdValue::add(scratch.ad_value(1360), scratch.ad_value(1361)));
        }

        scratch.values[1652] = if (scratch.values[383] == 0.5) { 1.0 } else { 0.0 };

        if (((((scratch.values[1320] != 0.0) && (scratch.values[1337] != 0.0)) && (!(scratch.values[1648] != 0.0))) && (!(scratch.values[1650] != 0.0))) && (scratch.values[1652] != 0.0)) {
            scratch.store_ad(1356, &AdValue::sqrt(AdValue::scale(scratch.ad_value(1359), scratch.values[486])));
        }

        if (((((scratch.values[1320] != 0.0) && (scratch.values[1337] != 0.0)) && (!(scratch.values[1648] != 0.0))) && (!(scratch.values[1650] != 0.0))) && (!(scratch.values[1652] != 0.0))) {
            scratch.store_ad(1356, &AdValue::powf(AdValue::scale(scratch.ad_value(1359), scratch.values[486]), scratch.values[383]));
        }

        if ((((scratch.values[1320] != 0.0) && (scratch.values[1337] != 0.0)) && (!(scratch.values[1648] != 0.0))) && (!(scratch.values[1650] != 0.0))) {
            scratch.store_ad(1363, &AdValue::scale(scratch.ad_value(1356), scratch.values[480]));
        }

        if ((((scratch.values[1320] != 0.0) && (scratch.values[1337] != 0.0)) && (!(scratch.values[1648] != 0.0))) && (!(scratch.values[1650] != 0.0))) {
            scratch.store_ad(1364, &AdValue::scale(AdValue::mul(AdValue::offset(scratch.ad_value(1350), (-1.0)), scratch.ad_value(1363)), scratch.values[441]));
        }

        if ((((scratch.values[1320] != 0.0) && (scratch.values[1337] != 0.0)) && (!(scratch.values[1648] != 0.0))) && (!(scratch.values[1650] != 0.0))) {
            scratch.store_ad(1358, &AdValue::scale(AdValue::mul(scratch.ad_value(1364), scratch.ad_value(1362)), scratch.values[394]));
        }

        scratch.values[1653] = if (scratch.values[397] == 0.0) { 1.0 } else { 0.0 };

        if ((((scratch.values[1320] != 0.0) && (scratch.values[1337] != 0.0)) && (!(scratch.values[1648] != 0.0))) && (scratch.values[1653] != 0.0)) {
            scratch.values[1365] = 0.0;
            scratch.node_derivatives[1365] = [0.0; Instance::NODE_COUNT];
            scratch.branch_derivatives[1365] = [0.0; Instance::BRANCH_COUNT];
        }

        if ((((scratch.values[1320] != 0.0) && (scratch.values[1337] != 0.0)) && (!(scratch.values[1648] != 0.0))) && (!(scratch.values[1653] != 0.0))) {
            scratch.store_ad(1366, &AdValue::scale(AdValue::div(AdValue::scale(scratch.ad_value(1363), scratch.values[465]), scratch.ad_value(1359)), scratch.values[495]));
        }

        if ((((scratch.values[1320] != 0.0) && (scratch.values[1337] != 0.0)) && (!(scratch.values[1648] != 0.0))) && (!(scratch.values[1653] != 0.0))) {
            scratch.store_ad(1367, &AdValue::div_from_scalar((0.666666666666667 * scratch.values[492]), scratch.ad_value(1366)));
        }

        if ((((scratch.values[1320] != 0.0) && (scratch.values[1337] != 0.0)) && (!(scratch.values[1648] != 0.0))) && (!(scratch.values[1653] != 0.0))) {
            scratch.store_ad(1368, &AdValue::square(scratch.ad_value(1367)));
        }

        if ((((scratch.values[1320] != 0.0) && (scratch.values[1337] != 0.0)) && (!(scratch.values[1648] != 0.0))) && (!(scratch.values[1653] != 0.0))) {
            scratch.store_ad(1369, &AdValue::sqrt(AdValue::div(AdValue::square(scratch.ad_value(1368)), AdValue::offset(AdValue::square(scratch.ad_value(1368)), 1.0))));
        }

        if ((((scratch.values[1320] != 0.0) && (scratch.values[1337] != 0.0)) && (!(scratch.values[1648] != 0.0))) && (!(scratch.values[1653] != 0.0))) {
            scratch.store_ad(1370, &AdValue::sqrt(AdValue::abs(scratch.ad_value(1369))));
        }

        if ((((scratch.values[1320] != 0.0) && (scratch.values[1337] != 0.0)) && (!(scratch.values[1648] != 0.0))) && (!(scratch.values[1653] != 0.0))) {
            scratch.store_ad(1371, &AdValue::mul(scratch.ad_value(1369), scratch.ad_value(1370)));
        }

        scratch.values[1654] = if (((-scratch.values[383]) * scratch.values[468]) == (-1.0)) { 1.0 } else { 0.0 };

        if (((((scratch.values[1320] != 0.0) && (scratch.values[1337] != 0.0)) && (!(scratch.values[1648] != 0.0))) && (!(scratch.values[1653] != 0.0))) && (scratch.values[1654] != 0.0)) {
            scratch.store_ad(1372, &AdValue::div_from_scalar(1.0, AdValue::offset(AdValue::mul(scratch.ad_value(1366), scratch.ad_value(1371)), 1.0)));
        }

        if (((((scratch.values[1320] != 0.0) && (scratch.values[1337] != 0.0)) && (!(scratch.values[1648] != 0.0))) && (!(scratch.values[1653] != 0.0))) && (!(scratch.values[1654] != 0.0))) {
            scratch.store_ad(1372, &AdValue::powf(AdValue::offset(AdValue::mul(scratch.ad_value(1366), scratch.ad_value(1371)), 1.0), ((-scratch.values[383]) * scratch.values[468])));
        }

        if ((((scratch.values[1320] != 0.0) && (scratch.values[1337] != 0.0)) && (!(scratch.values[1648] != 0.0))) && (!(scratch.values[1653] != 0.0))) {
            scratch.store_ad(1373, &AdValue::div(AdValue::mul(scratch.ad_value(1362), scratch.ad_value(1372)), AdValue::add(scratch.ad_value(1362), scratch.ad_value(1372))));
        }

        if ((((scratch.values[1320] != 0.0) && (scratch.values[1337] != 0.0)) && (!(scratch.values[1648] != 0.0))) && (!(scratch.values[1653] != 0.0))) {
            scratch.store_ad(1374, &AdValue::sqrt(AdValue::scale(AdValue::div(scratch.ad_value(1366), scratch.ad_value(1370)), 0.375)));
        }

        if ((((scratch.values[1320] != 0.0) && (scratch.values[1337] != 0.0)) && (!(scratch.values[1648] != 0.0))) && (!(scratch.values[1653] != 0.0))) {
            scratch.store_ad(1375, &AdValue::sub(AdValue::scale(AdValue::mul(scratch.ad_value(1367), scratch.ad_value(1370)), 2.0), scratch.ad_value(1369)));
        }

        if ((((scratch.values[1320] != 0.0) && (scratch.values[1337] != 0.0)) && (!(scratch.values[1648] != 0.0))) && (!(scratch.values[1653] != 0.0))) {
            scratch.store_ad(1376, &AdValue::add(AdValue::sub(AdValue::mul(AdValue::scale(scratch.ad_value(1367), scratch.values[492]), scratch.ad_value(1370)), AdValue::scale(scratch.ad_value(1369), scratch.values[492])), AdValue::scale(AdValue::mul(scratch.ad_value(1366), scratch.ad_value(1371)), 0.5)));
        }

        if ((((scratch.values[1320] != 0.0) && (scratch.values[1337] != 0.0)) && (!(scratch.values[1648] != 0.0))) && (!(scratch.values[1653] != 0.0))) {
            scratch.store_ad(1377, &AdValue::mul(AdValue::offset(scratch.ad_value(1375), (-1.0)), scratch.ad_value(1374)));
        }

        if ((((scratch.values[1320] != 0.0) && (scratch.values[1337] != 0.0)) && (!(scratch.values[1648] != 0.0))) && (!(scratch.values[1653] != 0.0))) {
            scratch.store_ad(1338, &AdValue::square(scratch.ad_value(1377)));
        }

        scratch.values[1655] = if (scratch.values[1377] > 0.0) { 1.0 } else { 0.0 };

        if (((((scratch.values[1320] != 0.0) && (scratch.values[1337] != 0.0)) && (!(scratch.values[1648] != 0.0))) && (!(scratch.values[1653] != 0.0))) && (scratch.values[1655] != 0.0)) {
            scratch.store_ad(1339, &AdValue::div_from_scalar(1.0, AdValue::offset(AdValue::scale(scratch.ad_value(1377), scratch.values[428]), 1.0)));
        }

        if (((((scratch.values[1320] != 0.0) && (scratch.values[1337] != 0.0)) && (!(scratch.values[1648] != 0.0))) && (!(scratch.values[1653] != 0.0))) && (!(scratch.values[1655] != 0.0))) {
            scratch.store_ad(1339, &AdValue::div_from_scalar(1.0, AdValue::sub_from_scalar(1.0, AdValue::scale(scratch.ad_value(1377), scratch.values[428]))));
        }

        scratch.values[1656] = if (((-scratch.values[1338]) + scratch.values[1376]) > (-230.25850929940458)) { 1.0 } else { 0.0 };

        if (((((scratch.values[1320] != 0.0) && (scratch.values[1337] != 0.0)) && (!(scratch.values[1648] != 0.0))) && (!(scratch.values[1653] != 0.0))) && (scratch.values[1656] != 0.0)) {
            scratch.store_ad(1356, &AdValue::exp(AdValue::sub(scratch.ad_value(1376), scratch.ad_value(1338))));
        }

        if (((((scratch.values[1320] != 0.0) && (scratch.values[1337] != 0.0)) && (!(scratch.values[1648] != 0.0))) && (!(scratch.values[1653] != 0.0))) && (!(scratch.values[1656] != 0.0))) {
            let assign24490_ad_e28453: AdValue = AdValue::div_from_scalar(1e-100, AdValue::offset(AdValue::mul(AdValue::sub_from_scalar((-230.25850929940458), AdValue::sub(scratch.ad_value(1376), scratch.ad_value(1338))), AdValue::offset(AdValue::scale(AdValue::mul(AdValue::sub_from_scalar((-230.25850929940458), AdValue::sub(scratch.ad_value(1376), scratch.ad_value(1338))), AdValue::offset(AdValue::scale(AdValue::sub_from_scalar((-230.25850929940458), AdValue::sub(scratch.ad_value(1376), scratch.ad_value(1338))), 0.3333333333333333), 1.0)), 0.5), 1.0)), 1.0));
            scratch.store_ad(1356, &assign24490_ad_e28453);
        }

        if ((((scratch.values[1320] != 0.0) && (scratch.values[1337] != 0.0)) && (!(scratch.values[1648] != 0.0))) && (!(scratch.values[1653] != 0.0))) {
            scratch.store_ad(1340, &AdValue::mul(AdValue::add(AdValue::add(AdValue::scale(scratch.ad_value(1339), 0.29214664), AdValue::scale(AdValue::square(scratch.ad_value(1339)), scratch.values[429])), AdValue::scale(AdValue::mul(AdValue::square(scratch.ad_value(1339)), scratch.ad_value(1339)), scratch.values[430])), scratch.ad_value(1356)));
        }

        scratch.values[1657] = if (scratch.values[1377] > 0.0) { 1.0 } else { 0.0 };

    }

    pub(super) fn stamp_transient_block_19(
        &mut self,
        ctx: &GeneratedEvalContext<'_>,
        stamper: &mut GeneratedStamper<'_>,
        scratch: &mut Scratch,
    ) {
        let _ = stamper;
        if (((((scratch.values[1320] != 0.0) && (scratch.values[1337] != 0.0)) && (!(scratch.values[1648] != 0.0))) && (!(scratch.values[1653] != 0.0))) && (scratch.values[1657] != 0.0)) {
            scratch.values[1378] = scratch.values[1340];
            scratch.node_derivatives[1378] = scratch.node_derivatives[1340];
            scratch.branch_derivatives[1378] = scratch.branch_derivatives[1340];
        }

        scratch.values[1658] = if (scratch.values[1376] > (-230.25850929940458)) { 1.0 } else { 0.0 };

        if ((((((scratch.values[1320] != 0.0) && (scratch.values[1337] != 0.0)) && (!(scratch.values[1648] != 0.0))) && (!(scratch.values[1653] != 0.0))) && (!(scratch.values[1657] != 0.0))) && (scratch.values[1658] != 0.0)) {
            scratch.store_ad(1356, &AdValue::exp(scratch.ad_value(1376)));
        }

        if ((((((scratch.values[1320] != 0.0) && (scratch.values[1337] != 0.0)) && (!(scratch.values[1648] != 0.0))) && (!(scratch.values[1653] != 0.0))) && (!(scratch.values[1657] != 0.0))) && (!(scratch.values[1658] != 0.0))) {
            scratch.store_ad(1356, &AdValue::div_from_scalar(1e-100, AdValue::offset(AdValue::mul(AdValue::sub_from_scalar((-230.25850929940458), scratch.ad_value(1376)), AdValue::offset(AdValue::scale(AdValue::mul(AdValue::sub_from_scalar((-230.25850929940458), scratch.ad_value(1376)), AdValue::offset(AdValue::scale(AdValue::sub_from_scalar((-230.25850929940458), scratch.ad_value(1376)), 0.3333333333333333), 1.0)), 0.5), 1.0)), 1.0)));
        }

        if (((((scratch.values[1320] != 0.0) && (scratch.values[1337] != 0.0)) && (!(scratch.values[1648] != 0.0))) && (!(scratch.values[1653] != 0.0))) && (!(scratch.values[1657] != 0.0))) {
            scratch.store_ad(1378, &AdValue::sub(AdValue::scale(scratch.ad_value(1356), 2.0), scratch.ad_value(1340)));
        }

        if ((((scratch.values[1320] != 0.0) && (scratch.values[1337] != 0.0)) && (!(scratch.values[1648] != 0.0))) && (!(scratch.values[1653] != 0.0))) {
            scratch.store_ad(1379, &AdValue::scale(AdValue::div(AdValue::scale(scratch.ad_value(1378), scratch.values[492]), scratch.ad_value(1374)), (1.772453850905516 * 0.5)));
        }

        if ((((scratch.values[1320] != 0.0) && (scratch.values[1337] != 0.0)) && (!(scratch.values[1648] != 0.0))) && (!(scratch.values[1653] != 0.0))) {
            scratch.store_ad(1365, &AdValue::scale(AdValue::mul(AdValue::mul(scratch.ad_value(1364), scratch.ad_value(1379)), scratch.ad_value(1373)), scratch.values[397]));
        }

        scratch.values[1659] = if (scratch.values[403] == 0.0) { 1.0 } else { 0.0 };

        if ((((scratch.values[1320] != 0.0) && (scratch.values[1337] != 0.0)) && (!(scratch.values[1648] != 0.0))) && (scratch.values[1659] != 0.0)) {
            scratch.values[1380] = 0.0;
            scratch.node_derivatives[1380] = [0.0; Instance::NODE_COUNT];
            scratch.branch_derivatives[1380] = [0.0; Instance::BRANCH_COUNT];
        }

        scratch.values[1660] = if (scratch.values[383] == 0.5) { 1.0 } else { 0.0 };

        if (((((scratch.values[1320] != 0.0) && (scratch.values[1337] != 0.0)) && (!(scratch.values[1648] != 0.0))) && (!(scratch.values[1659] != 0.0))) && (scratch.values[1660] != 0.0)) {
            scratch.store_ad(1356, &AdValue::sqrt(AdValue::scale(AdValue::sub_from_scalar(scratch.values[380], scratch.ad_value(1354)), scratch.values[486])));
        }

        if (((((scratch.values[1320] != 0.0) && (scratch.values[1337] != 0.0)) && (!(scratch.values[1648] != 0.0))) && (!(scratch.values[1659] != 0.0))) && (!(scratch.values[1660] != 0.0))) {
            scratch.store_ad(1356, &AdValue::powf(AdValue::scale(AdValue::sub_from_scalar(scratch.values[380], scratch.ad_value(1354)), scratch.values[486]), scratch.values[383]));
        }

        if ((((scratch.values[1320] != 0.0) && (scratch.values[1337] != 0.0)) && (!(scratch.values[1648] != 0.0))) && (!(scratch.values[1659] != 0.0))) {
            scratch.store_ad(1381, &AdValue::scale(AdValue::div(AdValue::scale(AdValue::sub_from_scalar(scratch.values[380], scratch.ad_value(1354)), scratch.values[483]), scratch.ad_value(1356)), scratch.values[468]));
        }

        scratch.values[1661] = if (((((-scratch.values[498]) / scratch.values[1381])) as f64).abs() < 230.25850929940458) { 1.0 } else { 0.0 };

        if (((((scratch.values[1320] != 0.0) && (scratch.values[1337] != 0.0)) && (!(scratch.values[1648] != 0.0))) && (!(scratch.values[1659] != 0.0))) && (scratch.values[1661] != 0.0)) {
            scratch.store_ad(1356, &AdValue::exp(AdValue::div(AdValue::neg(scratch.ad_value(498)), scratch.ad_value(1381))));
        }

        scratch.values[1662] = if (((-scratch.values[498]) / scratch.values[1381]) < 0.0) { 1.0 } else { 0.0 };

        if ((((((scratch.values[1320] != 0.0) && (scratch.values[1337] != 0.0)) && (!(scratch.values[1648] != 0.0))) && (!(scratch.values[1659] != 0.0))) && (!(scratch.values[1661] != 0.0))) && (scratch.values[1662] != 0.0)) {
            let assign24680_ad_e28780: AdValue = AdValue::offset(AdValue::mul(AdValue::sub_from_scalar((-230.25850929940458), AdValue::div(AdValue::neg(scratch.ad_value(498)), scratch.ad_value(1381))), AdValue::offset(AdValue::scale(AdValue::mul(AdValue::sub_from_scalar((-230.25850929940458), AdValue::div(AdValue::neg(scratch.ad_value(498)), scratch.ad_value(1381))), AdValue::offset(AdValue::scale(AdValue::sub_from_scalar((-230.25850929940458), AdValue::div(AdValue::neg(scratch.ad_value(498)), scratch.ad_value(1381))), 0.3333333333333333), 1.0)), 0.5), 1.0)), 1.0);
            scratch.store_ad(1356, &AdValue::div_from_scalar(1e-100, assign24680_ad_e28780));
        }

        if ((((((scratch.values[1320] != 0.0) && (scratch.values[1337] != 0.0)) && (!(scratch.values[1648] != 0.0))) && (!(scratch.values[1659] != 0.0))) && (!(scratch.values[1661] != 0.0))) && (!(scratch.values[1662] != 0.0))) {
            let assign24690_ad_e28830: AdValue = AdValue::scale(AdValue::offset(AdValue::mul(AdValue::offset(AdValue::div(AdValue::neg(scratch.ad_value(498)), scratch.ad_value(1381)), (-230.25850929940458)), AdValue::offset(AdValue::scale(AdValue::mul(AdValue::offset(AdValue::div(AdValue::neg(scratch.ad_value(498)), scratch.ad_value(1381)), (-230.25850929940458)), AdValue::offset(AdValue::scale(AdValue::offset(AdValue::div(AdValue::neg(scratch.ad_value(498)), scratch.ad_value(1381)), (-230.25850929940458)), 0.3333333333333333), 1.0)), 0.5), 1.0)), 1.0), 1e100);
            scratch.store_ad(1356, &assign24690_ad_e28830);
        }

        if ((((scratch.values[1320] != 0.0) && (scratch.values[1337] != 0.0)) && (!(scratch.values[1648] != 0.0))) && (!(scratch.values[1659] != 0.0))) {
            scratch.store_ad(1380, &AdValue::scale(AdValue::mul(AdValue::mul(AdValue::mul(scratch.ad_value(527), scratch.ad_value(1381)), scratch.ad_value(1381)), scratch.ad_value(1356)), scratch.values[403]));
        }

        scratch.values[1663] = if (scratch.values[412] > 1000.0) { 1.0 } else { 0.0 };

        if ((((scratch.values[1320] != 0.0) && (scratch.values[1337] != 0.0)) && (!(scratch.values[1648] != 0.0))) && (scratch.values[1663] != 0.0)) {
            scratch.values[1382] = 1.0;
            scratch.node_derivatives[1382] = [0.0; Instance::NODE_COUNT];
            scratch.branch_derivatives[1382] = [0.0; Instance::BRANCH_COUNT];
        }

        scratch.values[1664] = if (scratch.values[1355] > ((-scratch.values[500]) * scratch.values[412])) { 1.0 } else { 0.0 };

        scratch.values[1665] = if (scratch.values[415] == 4.0) { 1.0 } else { 0.0 };

        if ((((((scratch.values[1320] != 0.0) && (scratch.values[1337] != 0.0)) && (!(scratch.values[1648] != 0.0))) && (!(scratch.values[1663] != 0.0))) && (scratch.values[1664] != 0.0)) && (scratch.values[1665] != 0.0)) {
            scratch.store_ad(1356, &AdValue::mul(AdValue::mul(AdValue::mul(AdValue::scale(scratch.ad_value(1355), scratch.values[505]), AdValue::scale(scratch.ad_value(1355), scratch.values[505])), AdValue::scale(scratch.ad_value(1355), scratch.values[505])), AdValue::scale(scratch.ad_value(1355), scratch.values[505])));
        }

        if ((((((scratch.values[1320] != 0.0) && (scratch.values[1337] != 0.0)) && (!(scratch.values[1648] != 0.0))) && (!(scratch.values[1663] != 0.0))) && (scratch.values[1664] != 0.0)) && (!(scratch.values[1665] != 0.0))) {
            scratch.store_ad(1356, &AdValue::powf(AdValue::abs(AdValue::scale(scratch.ad_value(1355), scratch.values[505])), scratch.values[415]));
        }

        if (((((scratch.values[1320] != 0.0) && (scratch.values[1337] != 0.0)) && (!(scratch.values[1648] != 0.0))) && (!(scratch.values[1663] != 0.0))) && (scratch.values[1664] != 0.0)) {
            scratch.store_ad(1382, &AdValue::div_from_scalar(1.0, AdValue::sub_from_scalar(1.0, scratch.ad_value(1356))));
        }

        if (((((scratch.values[1320] != 0.0) && (scratch.values[1337] != 0.0)) && (!(scratch.values[1648] != 0.0))) && (!(scratch.values[1663] != 0.0))) && (!(scratch.values[1664] != 0.0))) {
            scratch.store_ad(1382, &AdValue::offset(AdValue::scale(AdValue::offset(scratch.ad_value(1355), (scratch.values[500] * scratch.values[412])), scratch.values[508]), scratch.values[502]));
        }

        if (((scratch.values[1320] != 0.0) && (scratch.values[1337] != 0.0)) && (!(scratch.values[1648] != 0.0))) {
            scratch.store_ad(1384, &AdValue::mul(AdValue::add(AdValue::add(AdValue::add(scratch.ad_value(1357), scratch.ad_value(1358)), scratch.ad_value(1365)), scratch.ad_value(1380)), scratch.ad_value(1382)));
        }

        scratch.values[1666] = if (scratch.values[699] == 0.0) { 1.0 } else { 0.0 };

        if (((scratch.values[1320] != 0.0) && (scratch.values[1337] != 0.0)) && (scratch.values[1666] != 0.0)) {
            scratch.values[1385] = 0.0;
            scratch.node_derivatives[1385] = [0.0; Instance::NODE_COUNT];
            scratch.branch_derivatives[1385] = [0.0; Instance::BRANCH_COUNT];
        }

        scratch.values[1667] = if (scratch.values[466] == 0.5) { 1.0 } else { 0.0 };

        if ((((scratch.values[1320] != 0.0) && (scratch.values[1337] != 0.0)) && (!(scratch.values[1666] != 0.0))) && (scratch.values[1667] != 0.0)) {
            scratch.store_ad(1356, &AdValue::sqrt(AdValue::sub_from_scalar(1.0, AdValue::scale(scratch.ad_value(1348), scratch.values[463]))));
        }

        if ((((scratch.values[1320] != 0.0) && (scratch.values[1337] != 0.0)) && (!(scratch.values[1666] != 0.0))) && (!(scratch.values[1667] != 0.0))) {
            scratch.store_ad(1356, &AdValue::powf(AdValue::sub_from_scalar(1.0, AdValue::scale(scratch.ad_value(1348), scratch.values[463])), scratch.values[466]));
        }

        if (((scratch.values[1320] != 0.0) && (scratch.values[1337] != 0.0)) && (!(scratch.values[1666] != 0.0))) {
            scratch.store_ad(1357, &AdValue::scale(scratch.ad_value(1347), scratch.values[445]));
        }

        scratch.values[1668] = if ((scratch.values[395] == 0.0) && (scratch.values[398] == 0.0)) { 1.0 } else { 0.0 };

        if ((((scratch.values[1320] != 0.0) && (scratch.values[1337] != 0.0)) && (!(scratch.values[1666] != 0.0))) && (scratch.values[1668] != 0.0)) {
            scratch.values[1358] = 0.0;
            scratch.node_derivatives[1358] = [0.0; Instance::NODE_COUNT];
            scratch.branch_derivatives[1358] = [0.0; Instance::BRANCH_COUNT];
        }

        if ((((scratch.values[1320] != 0.0) && (scratch.values[1337] != 0.0)) && (!(scratch.values[1666] != 0.0))) && (!(scratch.values[1668] != 0.0))) {
            scratch.store_ad(1359, &AdValue::sub_from_scalar(scratch.values[451], scratch.ad_value(1353)));
        }

        if ((((scratch.values[1320] != 0.0) && (scratch.values[1337] != 0.0)) && (!(scratch.values[1666] != 0.0))) && (!(scratch.values[1668] != 0.0))) {
            scratch.store_ad(1360, &AdValue::sub_from_scalar(1.0, AdValue::sqrt(AdValue::sub_from_scalar(1.0, AdValue::div(scratch.ad_value(1351), scratch.ad_value(1359))))));
        }

        scratch.values[1669] = if (scratch.values[384] == 0.5) { 1.0 } else { 0.0 };

        if (((((scratch.values[1320] != 0.0) && (scratch.values[1337] != 0.0)) && (!(scratch.values[1666] != 0.0))) && (!(scratch.values[1668] != 0.0))) && (scratch.values[1669] != 0.0)) {
            scratch.values[1361] = 0.0;
            scratch.node_derivatives[1361] = [0.0; Instance::NODE_COUNT];
            scratch.branch_derivatives[1361] = [0.0; Instance::BRANCH_COUNT];
        }

        if (((((scratch.values[1320] != 0.0) && (scratch.values[1337] != 0.0)) && (!(scratch.values[1666] != 0.0))) && (!(scratch.values[1668] != 0.0))) && (!(scratch.values[1669] != 0.0))) {
            scratch.store_ad(1361, &AdValue::scale(AdValue::add(AdValue::div(AdValue::mul(AdValue::square(scratch.ad_value(1360)), AdValue::ln(scratch.ad_value(1360))), AdValue::sub_from_scalar(1.0, scratch.ad_value(1360))), scratch.ad_value(1360)), (1.0 - (2.0 * scratch.values[384]))));
        }

        if ((((scratch.values[1320] != 0.0) && (scratch.values[1337] != 0.0)) && (!(scratch.values[1666] != 0.0))) && (!(scratch.values[1668] != 0.0))) {
            scratch.store_ad(1362, &AdValue::add(scratch.ad_value(1360), scratch.ad_value(1361)));
        }

        scratch.values[1670] = if (scratch.values[384] == 0.5) { 1.0 } else { 0.0 };

        if (((((scratch.values[1320] != 0.0) && (scratch.values[1337] != 0.0)) && (!(scratch.values[1666] != 0.0))) && (!(scratch.values[1668] != 0.0))) && (scratch.values[1670] != 0.0)) {
            scratch.store_ad(1356, &AdValue::sqrt(AdValue::scale(scratch.ad_value(1359), scratch.values[487])));
        }

        if (((((scratch.values[1320] != 0.0) && (scratch.values[1337] != 0.0)) && (!(scratch.values[1666] != 0.0))) && (!(scratch.values[1668] != 0.0))) && (!(scratch.values[1670] != 0.0))) {
            scratch.store_ad(1356, &AdValue::powf(AdValue::scale(scratch.ad_value(1359), scratch.values[487]), scratch.values[384]));
        }

        if ((((scratch.values[1320] != 0.0) && (scratch.values[1337] != 0.0)) && (!(scratch.values[1666] != 0.0))) && (!(scratch.values[1668] != 0.0))) {
            scratch.store_ad(1363, &AdValue::scale(scratch.ad_value(1356), scratch.values[481]));
        }

        if ((((scratch.values[1320] != 0.0) && (scratch.values[1337] != 0.0)) && (!(scratch.values[1666] != 0.0))) && (!(scratch.values[1668] != 0.0))) {
            scratch.store_ad(1364, &AdValue::scale(AdValue::mul(AdValue::offset(scratch.ad_value(1350), (-1.0)), scratch.ad_value(1363)), scratch.values[442]));
        }

        if ((((scratch.values[1320] != 0.0) && (scratch.values[1337] != 0.0)) && (!(scratch.values[1666] != 0.0))) && (!(scratch.values[1668] != 0.0))) {
            scratch.store_ad(1358, &AdValue::scale(AdValue::mul(scratch.ad_value(1364), scratch.ad_value(1362)), scratch.values[395]));
        }

        scratch.values[1671] = if (scratch.values[398] == 0.0) { 1.0 } else { 0.0 };

        if ((((scratch.values[1320] != 0.0) && (scratch.values[1337] != 0.0)) && (!(scratch.values[1666] != 0.0))) && (scratch.values[1671] != 0.0)) {
            scratch.values[1365] = 0.0;
            scratch.node_derivatives[1365] = [0.0; Instance::NODE_COUNT];
            scratch.branch_derivatives[1365] = [0.0; Instance::BRANCH_COUNT];
        }

        if ((((scratch.values[1320] != 0.0) && (scratch.values[1337] != 0.0)) && (!(scratch.values[1666] != 0.0))) && (!(scratch.values[1671] != 0.0))) {
            scratch.store_ad(1366, &AdValue::scale(AdValue::div(AdValue::scale(scratch.ad_value(1363), scratch.values[466]), scratch.ad_value(1359)), scratch.values[496]));
        }

        if ((((scratch.values[1320] != 0.0) && (scratch.values[1337] != 0.0)) && (!(scratch.values[1666] != 0.0))) && (!(scratch.values[1671] != 0.0))) {
            scratch.store_ad(1367, &AdValue::div_from_scalar((0.666666666666667 * scratch.values[493]), scratch.ad_value(1366)));
        }

        if ((((scratch.values[1320] != 0.0) && (scratch.values[1337] != 0.0)) && (!(scratch.values[1666] != 0.0))) && (!(scratch.values[1671] != 0.0))) {
            scratch.store_ad(1368, &AdValue::square(scratch.ad_value(1367)));
        }

        if ((((scratch.values[1320] != 0.0) && (scratch.values[1337] != 0.0)) && (!(scratch.values[1666] != 0.0))) && (!(scratch.values[1671] != 0.0))) {
            scratch.store_ad(1369, &AdValue::sqrt(AdValue::div(AdValue::square(scratch.ad_value(1368)), AdValue::offset(AdValue::square(scratch.ad_value(1368)), 1.0))));
        }

        if ((((scratch.values[1320] != 0.0) && (scratch.values[1337] != 0.0)) && (!(scratch.values[1666] != 0.0))) && (!(scratch.values[1671] != 0.0))) {
            scratch.store_ad(1370, &AdValue::sqrt(AdValue::abs(scratch.ad_value(1369))));
        }

        if ((((scratch.values[1320] != 0.0) && (scratch.values[1337] != 0.0)) && (!(scratch.values[1666] != 0.0))) && (!(scratch.values[1671] != 0.0))) {
            scratch.store_ad(1371, &AdValue::mul(scratch.ad_value(1369), scratch.ad_value(1370)));
        }

        scratch.values[1672] = if (((-scratch.values[384]) * scratch.values[469]) == (-1.0)) { 1.0 } else { 0.0 };

        if (((((scratch.values[1320] != 0.0) && (scratch.values[1337] != 0.0)) && (!(scratch.values[1666] != 0.0))) && (!(scratch.values[1671] != 0.0))) && (scratch.values[1672] != 0.0)) {
            scratch.store_ad(1372, &AdValue::div_from_scalar(1.0, AdValue::offset(AdValue::mul(scratch.ad_value(1366), scratch.ad_value(1371)), 1.0)));
        }

        if (((((scratch.values[1320] != 0.0) && (scratch.values[1337] != 0.0)) && (!(scratch.values[1666] != 0.0))) && (!(scratch.values[1671] != 0.0))) && (!(scratch.values[1672] != 0.0))) {
            scratch.store_ad(1372, &AdValue::powf(AdValue::offset(AdValue::mul(scratch.ad_value(1366), scratch.ad_value(1371)), 1.0), ((-scratch.values[384]) * scratch.values[469])));
        }

        if ((((scratch.values[1320] != 0.0) && (scratch.values[1337] != 0.0)) && (!(scratch.values[1666] != 0.0))) && (!(scratch.values[1671] != 0.0))) {
            scratch.store_ad(1373, &AdValue::div(AdValue::mul(scratch.ad_value(1362), scratch.ad_value(1372)), AdValue::add(scratch.ad_value(1362), scratch.ad_value(1372))));
        }

        if ((((scratch.values[1320] != 0.0) && (scratch.values[1337] != 0.0)) && (!(scratch.values[1666] != 0.0))) && (!(scratch.values[1671] != 0.0))) {
            scratch.store_ad(1374, &AdValue::sqrt(AdValue::scale(AdValue::div(scratch.ad_value(1366), scratch.ad_value(1370)), 0.375)));
        }

        if ((((scratch.values[1320] != 0.0) && (scratch.values[1337] != 0.0)) && (!(scratch.values[1666] != 0.0))) && (!(scratch.values[1671] != 0.0))) {
            scratch.store_ad(1375, &AdValue::sub(AdValue::scale(AdValue::mul(scratch.ad_value(1367), scratch.ad_value(1370)), 2.0), scratch.ad_value(1369)));
        }

        if ((((scratch.values[1320] != 0.0) && (scratch.values[1337] != 0.0)) && (!(scratch.values[1666] != 0.0))) && (!(scratch.values[1671] != 0.0))) {
            scratch.store_ad(1376, &AdValue::add(AdValue::sub(AdValue::mul(AdValue::scale(scratch.ad_value(1367), scratch.values[493]), scratch.ad_value(1370)), AdValue::scale(scratch.ad_value(1369), scratch.values[493])), AdValue::scale(AdValue::mul(scratch.ad_value(1366), scratch.ad_value(1371)), 0.5)));
        }

        if ((((scratch.values[1320] != 0.0) && (scratch.values[1337] != 0.0)) && (!(scratch.values[1666] != 0.0))) && (!(scratch.values[1671] != 0.0))) {
            scratch.store_ad(1377, &AdValue::mul(AdValue::offset(scratch.ad_value(1375), (-1.0)), scratch.ad_value(1374)));
        }

        if ((((scratch.values[1320] != 0.0) && (scratch.values[1337] != 0.0)) && (!(scratch.values[1666] != 0.0))) && (!(scratch.values[1671] != 0.0))) {
            scratch.store_ad(1338, &AdValue::square(scratch.ad_value(1377)));
        }

        scratch.values[1673] = if (scratch.values[1377] > 0.0) { 1.0 } else { 0.0 };

        if (((((scratch.values[1320] != 0.0) && (scratch.values[1337] != 0.0)) && (!(scratch.values[1666] != 0.0))) && (!(scratch.values[1671] != 0.0))) && (scratch.values[1673] != 0.0)) {
            scratch.store_ad(1339, &AdValue::div_from_scalar(1.0, AdValue::offset(AdValue::scale(scratch.ad_value(1377), scratch.values[428]), 1.0)));
        }

        if (((((scratch.values[1320] != 0.0) && (scratch.values[1337] != 0.0)) && (!(scratch.values[1666] != 0.0))) && (!(scratch.values[1671] != 0.0))) && (!(scratch.values[1673] != 0.0))) {
            scratch.store_ad(1339, &AdValue::div_from_scalar(1.0, AdValue::sub_from_scalar(1.0, AdValue::scale(scratch.ad_value(1377), scratch.values[428]))));
        }

        scratch.values[1674] = if (((-scratch.values[1338]) + scratch.values[1376]) > (-230.25850929940458)) { 1.0 } else { 0.0 };

        if (((((scratch.values[1320] != 0.0) && (scratch.values[1337] != 0.0)) && (!(scratch.values[1666] != 0.0))) && (!(scratch.values[1671] != 0.0))) && (scratch.values[1674] != 0.0)) {
            scratch.store_ad(1356, &AdValue::exp(AdValue::sub(scratch.ad_value(1376), scratch.ad_value(1338))));
        }

        if (((((scratch.values[1320] != 0.0) && (scratch.values[1337] != 0.0)) && (!(scratch.values[1666] != 0.0))) && (!(scratch.values[1671] != 0.0))) && (!(scratch.values[1674] != 0.0))) {
            let assign25240_ad_e29659: AdValue = AdValue::div_from_scalar(1e-100, AdValue::offset(AdValue::mul(AdValue::sub_from_scalar((-230.25850929940458), AdValue::sub(scratch.ad_value(1376), scratch.ad_value(1338))), AdValue::offset(AdValue::scale(AdValue::mul(AdValue::sub_from_scalar((-230.25850929940458), AdValue::sub(scratch.ad_value(1376), scratch.ad_value(1338))), AdValue::offset(AdValue::scale(AdValue::sub_from_scalar((-230.25850929940458), AdValue::sub(scratch.ad_value(1376), scratch.ad_value(1338))), 0.3333333333333333), 1.0)), 0.5), 1.0)), 1.0));
            scratch.store_ad(1356, &assign25240_ad_e29659);
        }

        if ((((scratch.values[1320] != 0.0) && (scratch.values[1337] != 0.0)) && (!(scratch.values[1666] != 0.0))) && (!(scratch.values[1671] != 0.0))) {
            scratch.store_ad(1340, &AdValue::mul(AdValue::add(AdValue::add(AdValue::scale(scratch.ad_value(1339), 0.29214664), AdValue::scale(AdValue::square(scratch.ad_value(1339)), scratch.values[429])), AdValue::scale(AdValue::mul(AdValue::square(scratch.ad_value(1339)), scratch.ad_value(1339)), scratch.values[430])), scratch.ad_value(1356)));
        }

        scratch.values[1675] = if (scratch.values[1377] > 0.0) { 1.0 } else { 0.0 };

        if (((((scratch.values[1320] != 0.0) && (scratch.values[1337] != 0.0)) && (!(scratch.values[1666] != 0.0))) && (!(scratch.values[1671] != 0.0))) && (scratch.values[1675] != 0.0)) {
            scratch.values[1378] = scratch.values[1340];
            scratch.node_derivatives[1378] = scratch.node_derivatives[1340];
            scratch.branch_derivatives[1378] = scratch.branch_derivatives[1340];
        }

        scratch.values[1676] = if (scratch.values[1376] > (-230.25850929940458)) { 1.0 } else { 0.0 };

        if ((((((scratch.values[1320] != 0.0) && (scratch.values[1337] != 0.0)) && (!(scratch.values[1666] != 0.0))) && (!(scratch.values[1671] != 0.0))) && (!(scratch.values[1675] != 0.0))) && (scratch.values[1676] != 0.0)) {
            scratch.store_ad(1356, &AdValue::exp(scratch.ad_value(1376)));
        }

        if ((((((scratch.values[1320] != 0.0) && (scratch.values[1337] != 0.0)) && (!(scratch.values[1666] != 0.0))) && (!(scratch.values[1671] != 0.0))) && (!(scratch.values[1675] != 0.0))) && (!(scratch.values[1676] != 0.0))) {
            scratch.store_ad(1356, &AdValue::div_from_scalar(1e-100, AdValue::offset(AdValue::mul(AdValue::sub_from_scalar((-230.25850929940458), scratch.ad_value(1376)), AdValue::offset(AdValue::scale(AdValue::mul(AdValue::sub_from_scalar((-230.25850929940458), scratch.ad_value(1376)), AdValue::offset(AdValue::scale(AdValue::sub_from_scalar((-230.25850929940458), scratch.ad_value(1376)), 0.3333333333333333), 1.0)), 0.5), 1.0)), 1.0)));
        }

        if (((((scratch.values[1320] != 0.0) && (scratch.values[1337] != 0.0)) && (!(scratch.values[1666] != 0.0))) && (!(scratch.values[1671] != 0.0))) && (!(scratch.values[1675] != 0.0))) {
            scratch.store_ad(1378, &AdValue::sub(AdValue::scale(scratch.ad_value(1356), 2.0), scratch.ad_value(1340)));
        }

        if ((((scratch.values[1320] != 0.0) && (scratch.values[1337] != 0.0)) && (!(scratch.values[1666] != 0.0))) && (!(scratch.values[1671] != 0.0))) {
            scratch.store_ad(1379, &AdValue::scale(AdValue::div(AdValue::scale(scratch.ad_value(1378), scratch.values[493]), scratch.ad_value(1374)), (1.772453850905516 * 0.5)));
        }

        if ((((scratch.values[1320] != 0.0) && (scratch.values[1337] != 0.0)) && (!(scratch.values[1666] != 0.0))) && (!(scratch.values[1671] != 0.0))) {
            scratch.store_ad(1365, &AdValue::scale(AdValue::mul(AdValue::mul(scratch.ad_value(1364), scratch.ad_value(1379)), scratch.ad_value(1373)), scratch.values[398]));
        }

        scratch.values[1677] = if (scratch.values[404] == 0.0) { 1.0 } else { 0.0 };

        if ((((scratch.values[1320] != 0.0) && (scratch.values[1337] != 0.0)) && (!(scratch.values[1666] != 0.0))) && (scratch.values[1677] != 0.0)) {
            scratch.values[1380] = 0.0;
            scratch.node_derivatives[1380] = [0.0; Instance::NODE_COUNT];
            scratch.branch_derivatives[1380] = [0.0; Instance::BRANCH_COUNT];
        }

        scratch.values[1678] = if (scratch.values[384] == 0.5) { 1.0 } else { 0.0 };

        if (((((scratch.values[1320] != 0.0) && (scratch.values[1337] != 0.0)) && (!(scratch.values[1666] != 0.0))) && (!(scratch.values[1677] != 0.0))) && (scratch.values[1678] != 0.0)) {
            scratch.store_ad(1356, &AdValue::sqrt(AdValue::scale(AdValue::sub_from_scalar(scratch.values[381], scratch.ad_value(1354)), scratch.values[487])));
        }

        if (((((scratch.values[1320] != 0.0) && (scratch.values[1337] != 0.0)) && (!(scratch.values[1666] != 0.0))) && (!(scratch.values[1677] != 0.0))) && (!(scratch.values[1678] != 0.0))) {
            scratch.store_ad(1356, &AdValue::powf(AdValue::scale(AdValue::sub_from_scalar(scratch.values[381], scratch.ad_value(1354)), scratch.values[487]), scratch.values[384]));
        }

        if ((((scratch.values[1320] != 0.0) && (scratch.values[1337] != 0.0)) && (!(scratch.values[1666] != 0.0))) && (!(scratch.values[1677] != 0.0))) {
            scratch.store_ad(1381, &AdValue::scale(AdValue::div(AdValue::scale(AdValue::sub_from_scalar(scratch.values[381], scratch.ad_value(1354)), scratch.values[484]), scratch.ad_value(1356)), scratch.values[469]));
        }

        scratch.values[1679] = if (((((-scratch.values[499]) / scratch.values[1381])) as f64).abs() < 230.25850929940458) { 1.0 } else { 0.0 };

        if (((((scratch.values[1320] != 0.0) && (scratch.values[1337] != 0.0)) && (!(scratch.values[1666] != 0.0))) && (!(scratch.values[1677] != 0.0))) && (scratch.values[1679] != 0.0)) {
            scratch.store_ad(1356, &AdValue::exp(AdValue::div(AdValue::neg(scratch.ad_value(499)), scratch.ad_value(1381))));
        }

        scratch.values[1680] = if (((-scratch.values[499]) / scratch.values[1381]) < 0.0) { 1.0 } else { 0.0 };

        if ((((((scratch.values[1320] != 0.0) && (scratch.values[1337] != 0.0)) && (!(scratch.values[1666] != 0.0))) && (!(scratch.values[1677] != 0.0))) && (!(scratch.values[1679] != 0.0))) && (scratch.values[1680] != 0.0)) {
            let assign25430_ad_e29986: AdValue = AdValue::offset(AdValue::mul(AdValue::sub_from_scalar((-230.25850929940458), AdValue::div(AdValue::neg(scratch.ad_value(499)), scratch.ad_value(1381))), AdValue::offset(AdValue::scale(AdValue::mul(AdValue::sub_from_scalar((-230.25850929940458), AdValue::div(AdValue::neg(scratch.ad_value(499)), scratch.ad_value(1381))), AdValue::offset(AdValue::scale(AdValue::sub_from_scalar((-230.25850929940458), AdValue::div(AdValue::neg(scratch.ad_value(499)), scratch.ad_value(1381))), 0.3333333333333333), 1.0)), 0.5), 1.0)), 1.0);
            scratch.store_ad(1356, &AdValue::div_from_scalar(1e-100, assign25430_ad_e29986));
        }

        if ((((((scratch.values[1320] != 0.0) && (scratch.values[1337] != 0.0)) && (!(scratch.values[1666] != 0.0))) && (!(scratch.values[1677] != 0.0))) && (!(scratch.values[1679] != 0.0))) && (!(scratch.values[1680] != 0.0))) {
            let assign25440_ad_e30036: AdValue = AdValue::scale(AdValue::offset(AdValue::mul(AdValue::offset(AdValue::div(AdValue::neg(scratch.ad_value(499)), scratch.ad_value(1381)), (-230.25850929940458)), AdValue::offset(AdValue::scale(AdValue::mul(AdValue::offset(AdValue::div(AdValue::neg(scratch.ad_value(499)), scratch.ad_value(1381)), (-230.25850929940458)), AdValue::offset(AdValue::scale(AdValue::offset(AdValue::div(AdValue::neg(scratch.ad_value(499)), scratch.ad_value(1381)), (-230.25850929940458)), 0.3333333333333333), 1.0)), 0.5), 1.0)), 1.0), 1e100);
            scratch.store_ad(1356, &assign25440_ad_e30036);
        }

        if ((((scratch.values[1320] != 0.0) && (scratch.values[1337] != 0.0)) && (!(scratch.values[1666] != 0.0))) && (!(scratch.values[1677] != 0.0))) {
            scratch.store_ad(1380, &AdValue::scale(AdValue::mul(AdValue::mul(AdValue::mul(scratch.ad_value(527), scratch.ad_value(1381)), scratch.ad_value(1381)), scratch.ad_value(1356)), scratch.values[404]));
        }

        scratch.values[1681] = if (scratch.values[413] > 1000.0) { 1.0 } else { 0.0 };

        if ((((scratch.values[1320] != 0.0) && (scratch.values[1337] != 0.0)) && (!(scratch.values[1666] != 0.0))) && (scratch.values[1681] != 0.0)) {
            scratch.values[1382] = 1.0;
            scratch.node_derivatives[1382] = [0.0; Instance::NODE_COUNT];
            scratch.branch_derivatives[1382] = [0.0; Instance::BRANCH_COUNT];
        }

        scratch.values[1682] = if (scratch.values[1355] > ((-scratch.values[500]) * scratch.values[413])) { 1.0 } else { 0.0 };

        scratch.values[1683] = if (scratch.values[416] == 4.0) { 1.0 } else { 0.0 };

        if ((((((scratch.values[1320] != 0.0) && (scratch.values[1337] != 0.0)) && (!(scratch.values[1666] != 0.0))) && (!(scratch.values[1681] != 0.0))) && (scratch.values[1682] != 0.0)) && (scratch.values[1683] != 0.0)) {
            scratch.store_ad(1356, &AdValue::mul(AdValue::mul(AdValue::mul(AdValue::scale(scratch.ad_value(1355), scratch.values[506]), AdValue::scale(scratch.ad_value(1355), scratch.values[506])), AdValue::scale(scratch.ad_value(1355), scratch.values[506])), AdValue::scale(scratch.ad_value(1355), scratch.values[506])));
        }

        if ((((((scratch.values[1320] != 0.0) && (scratch.values[1337] != 0.0)) && (!(scratch.values[1666] != 0.0))) && (!(scratch.values[1681] != 0.0))) && (scratch.values[1682] != 0.0)) && (!(scratch.values[1683] != 0.0))) {
            scratch.store_ad(1356, &AdValue::powf(AdValue::abs(AdValue::scale(scratch.ad_value(1355), scratch.values[506])), scratch.values[416]));
        }

        if (((((scratch.values[1320] != 0.0) && (scratch.values[1337] != 0.0)) && (!(scratch.values[1666] != 0.0))) && (!(scratch.values[1681] != 0.0))) && (scratch.values[1682] != 0.0)) {
            scratch.store_ad(1382, &AdValue::div_from_scalar(1.0, AdValue::sub_from_scalar(1.0, scratch.ad_value(1356))));
        }

        if (((((scratch.values[1320] != 0.0) && (scratch.values[1337] != 0.0)) && (!(scratch.values[1666] != 0.0))) && (!(scratch.values[1681] != 0.0))) && (!(scratch.values[1682] != 0.0))) {
            scratch.store_ad(1382, &AdValue::offset(AdValue::scale(AdValue::offset(scratch.ad_value(1355), (scratch.values[500] * scratch.values[413])), scratch.values[509]), scratch.values[503]));
        }

        if (((scratch.values[1320] != 0.0) && (scratch.values[1337] != 0.0)) && (!(scratch.values[1666] != 0.0))) {
            scratch.store_ad(1385, &AdValue::mul(AdValue::add(AdValue::add(AdValue::add(scratch.ad_value(1357), scratch.ad_value(1358)), scratch.ad_value(1365)), scratch.ad_value(1380)), scratch.ad_value(1382)));
        }

        if ((scratch.values[1320] != 0.0) && (scratch.values[1337] != 0.0)) {
            scratch.store_ad(517, &AdValue::add(AdValue::add(AdValue::mul(scratch.ad_value(697), scratch.ad_value(1383)), AdValue::mul(scratch.ad_value(698), scratch.ad_value(1384))), AdValue::mul(scratch.ad_value(699), scratch.ad_value(1385))));
        }

        if ((scratch.values[1320] != 0.0) && (scratch.values[1337] != 0.0)) {
            scratch.store_ad(718, &AdValue::add(AdValue::add(AdValue::scale(scratch.ad_value(697), scratch.values[443]), AdValue::scale(scratch.ad_value(698), scratch.values[444])), AdValue::scale(scratch.ad_value(699), scratch.values[445])));
        }

        if ((scratch.values[1320] != 0.0) && (scratch.values[1337] != 0.0)) {
            scratch.store_ad(521, &AdValue::sub(scratch.ad_value(516), AdValue::mul(scratch.ad_value(718), AdValue::offset(AdValue::exp(AdValue::scale(scratch.ad_value(526), (scratch.values[427] * scratch.values[719]))), (-1.0)))));
        }

        if ((scratch.values[1320] != 0.0) && (scratch.values[1337] != 0.0)) {
            scratch.store_ad(522, &AdValue::sub(scratch.ad_value(517), AdValue::mul(scratch.ad_value(718), AdValue::offset(AdValue::exp(AdValue::scale(scratch.ad_value(527), (scratch.values[427] * scratch.values[719]))), (-1.0)))));
        }

        scratch.values[1684] = if !(((scratch.values[697] == 0.0) && (scratch.values[698] == 0.0)) && (scratch.values[699] == 0.0)) { 1.0 } else { 0.0 };

        scratch.values[1685] = if ((scratch.values[516] > 0.0) && (scratch.values[517] > 0.0)) { 1.0 } else { 0.0 };

        scratch.values[1686] = if (((((scratch.values[521] / scratch.values[516]) > 0.001) || ((scratch.values[522] / scratch.values[517]) > 0.001)) && (scratch.values[521] > 0.0)) && (scratch.values[522] > 0.0)) { 1.0 } else { 0.0 };

        if (((((scratch.values[1320] != 0.0) && (scratch.values[1337] != 0.0)) && (scratch.values[1684] != 0.0)) && (scratch.values[1685] != 0.0)) && (scratch.values[1686] != 0.0)) {
            scratch.store_ad(528, &AdValue::div(scratch.ad_value(521), scratch.ad_value(522)));
        }

        if (((((scratch.values[1320] != 0.0) && (scratch.values[1337] != 0.0)) && (scratch.values[1684] != 0.0)) && (scratch.values[1685] != 0.0)) && (scratch.values[1686] != 0.0)) {
            scratch.store_ad(721, &AdValue::div(AdValue::scale(AdValue::ln(scratch.ad_value(528)), scratch.values[426]), AdValue::sub(scratch.ad_value(526), scratch.ad_value(527))));
        }

        if (((((scratch.values[1320] != 0.0) && (scratch.values[1337] != 0.0)) && (scratch.values[1684] != 0.0)) && (scratch.values[1685] != 0.0)) && (scratch.values[1686] != 0.0)) {
            scratch.store_ad(720, &AdValue::div(scratch.ad_value(521), AdValue::offset(AdValue::exp(AdValue::mul(AdValue::scale(scratch.ad_value(526), scratch.values[427]), scratch.ad_value(721))), (-1.0))));
        }

        if (((scratch.values[1320] != 0.0) && (scratch.values[1337] != 0.0)) && (scratch.values[1684] != 0.0)) {
            scratch.store_ad(518, &AdValue::sub(AdValue::sub(scratch.ad_value(513), AdValue::mul(scratch.ad_value(718), AdValue::offset(AdValue::exp(AdValue::scale(scratch.ad_value(523), (scratch.values[427] * scratch.values[719]))), (-1.0)))), AdValue::mul(scratch.ad_value(720), AdValue::offset(AdValue::exp(AdValue::mul(AdValue::scale(scratch.ad_value(523), scratch.values[427]), scratch.ad_value(721))), (-1.0)))));
        }

        if (((scratch.values[1320] != 0.0) && (scratch.values[1337] != 0.0)) && (scratch.values[1684] != 0.0)) {
            scratch.store_ad(519, &AdValue::sub(AdValue::sub(scratch.ad_value(514), AdValue::mul(scratch.ad_value(718), AdValue::offset(AdValue::exp(AdValue::scale(scratch.ad_value(524), (scratch.values[427] * scratch.values[719]))), (-1.0)))), AdValue::mul(scratch.ad_value(720), AdValue::offset(AdValue::exp(AdValue::mul(AdValue::scale(scratch.ad_value(524), scratch.values[427]), scratch.ad_value(721))), (-1.0)))));
        }

        if (((scratch.values[1320] != 0.0) && (scratch.values[1337] != 0.0)) && (scratch.values[1684] != 0.0)) {
            scratch.store_ad(520, &AdValue::sub(AdValue::sub(scratch.ad_value(515), AdValue::mul(scratch.ad_value(718), AdValue::offset(AdValue::exp(AdValue::scale(scratch.ad_value(525), (scratch.values[427] * scratch.values[719]))), (-1.0)))), AdValue::mul(scratch.ad_value(720), AdValue::offset(AdValue::exp(AdValue::mul(AdValue::scale(scratch.ad_value(525), scratch.values[427]), scratch.ad_value(721))), (-1.0)))));
        }

        scratch.values[1687] = if (((scratch.values[513] < 0.0) && (scratch.values[514] < 0.0)) && (scratch.values[515] < 0.0)) { 1.0 } else { 0.0 };

        scratch.values[1688] = if (((((((scratch.values[518] / scratch.values[513]) > 0.001) || ((scratch.values[519] / scratch.values[514]) > 0.001)) || ((scratch.values[520] / scratch.values[515]) > 0.001)) && (scratch.values[518] < 0.0)) && (scratch.values[519] < 0.0)) && (scratch.values[520] < 0.0)) { 1.0 } else { 0.0 };

        if (((((scratch.values[1320] != 0.0) && (scratch.values[1337] != 0.0)) && (scratch.values[1684] != 0.0)) && (scratch.values[1687] != 0.0)) && (scratch.values[1688] != 0.0)) {
            scratch.store_ad(528, &AdValue::div(scratch.ad_value(518), scratch.ad_value(519)));
        }

        if (((((scratch.values[1320] != 0.0) && (scratch.values[1337] != 0.0)) && (scratch.values[1684] != 0.0)) && (scratch.values[1687] != 0.0)) && (scratch.values[1688] != 0.0)) {
            scratch.store_ad(529, &AdValue::div(AdValue::scale(AdValue::ln(scratch.ad_value(528)), (-scratch.values[426])), AdValue::sub(scratch.ad_value(523), scratch.ad_value(524))));
        }

        if (((((scratch.values[1320] != 0.0) && (scratch.values[1337] != 0.0)) && (scratch.values[1684] != 0.0)) && (scratch.values[1687] != 0.0)) && (scratch.values[1688] != 0.0)) {
            scratch.store_ad(531, &AdValue::div(scratch.ad_value(524), AdValue::sub(scratch.ad_value(524), scratch.ad_value(523))));
        }

        if (((((scratch.values[1320] != 0.0) && (scratch.values[1337] != 0.0)) && (scratch.values[1684] != 0.0)) && (scratch.values[1687] != 0.0)) && (scratch.values[1688] != 0.0)) {
            scratch.store_ad(532, &AdValue::mul(AdValue::scale(AdValue::offset(scratch.ad_value(528), (-1.0)), scratch.values[426]), AdValue::offset(AdValue::pow(scratch.ad_value(528), scratch.ad_value(531)), (-1.0))));
        }

        if (((((scratch.values[1320] != 0.0) && (scratch.values[1337] != 0.0)) && (scratch.values[1684] != 0.0)) && (scratch.values[1687] != 0.0)) && (scratch.values[1688] != 0.0)) {
            scratch.store_ad(531, &AdValue::div(scratch.ad_value(523), AdValue::sub(scratch.ad_value(523), scratch.ad_value(524))));
        }

        if (((((scratch.values[1320] != 0.0) && (scratch.values[1337] != 0.0)) && (scratch.values[1684] != 0.0)) && (scratch.values[1687] != 0.0)) && (scratch.values[1688] != 0.0)) {
            scratch.store_ad(533, &AdValue::sub(AdValue::add(AdValue::mul(AdValue::pow(scratch.ad_value(528), scratch.ad_value(531)), AdValue::sub(scratch.ad_value(524), scratch.ad_value(523))), AdValue::mul(scratch.ad_value(528), scratch.ad_value(523))), scratch.ad_value(524)));
        }

        if (((((scratch.values[1320] != 0.0) && (scratch.values[1337] != 0.0)) && (scratch.values[1684] != 0.0)) && (scratch.values[1687] != 0.0)) && (scratch.values[1688] != 0.0)) {
            scratch.store_ad(530, &AdValue::div(scratch.ad_value(532), scratch.ad_value(533)));
        }

        if (((((scratch.values[1320] != 0.0) && (scratch.values[1337] != 0.0)) && (scratch.values[1684] != 0.0)) && (scratch.values[1687] != 0.0)) && (scratch.values[1688] != 0.0)) {
            scratch.store_ad(723, &AdValue::add(scratch.ad_value(529), scratch.ad_value(530)));
        }

        scratch.values[1689] = if (((((scratch.values[525] * scratch.values[427]) * scratch.values[723])) as f64).abs() < 1e-6) { 1.0 } else { 0.0 };

        if ((((((scratch.values[1320] != 0.0) && (scratch.values[1337] != 0.0)) && (scratch.values[1684] != 0.0)) && (scratch.values[1687] != 0.0)) && (scratch.values[1688] != 0.0)) && (scratch.values[1689] != 0.0)) {
            scratch.values[717] = 1.0;
            scratch.node_derivatives[717] = [0.0; Instance::NODE_COUNT];
            scratch.branch_derivatives[717] = [0.0; Instance::BRANCH_COUNT];
        }

        if ((((((scratch.values[1320] != 0.0) && (scratch.values[1337] != 0.0)) && (scratch.values[1684] != 0.0)) && (scratch.values[1687] != 0.0)) && (scratch.values[1688] != 0.0)) && (scratch.values[1689] != 0.0)) {
            scratch.store_ad(722, &AdValue::mul(scratch.ad_value(520), AdValue::add(AdValue::div_from_scalar(1.0, scratch.ad_value(525)), AdValue::scale(scratch.ad_value(723), (0.5 * scratch.values[427])))));
        }

        if ((((((scratch.values[1320] != 0.0) && (scratch.values[1337] != 0.0)) && (scratch.values[1684] != 0.0)) && (scratch.values[1687] != 0.0)) && (scratch.values[1688] != 0.0)) && (scratch.values[1689] != 0.0)) {
            scratch.store_ad(723, &AdValue::div(AdValue::scale(AdValue::mul(AdValue::scale(scratch.ad_value(520), (-0.5)), scratch.ad_value(723)), scratch.values[427]), scratch.ad_value(525)));
        }

        if ((((((scratch.values[1320] != 0.0) && (scratch.values[1337] != 0.0)) && (scratch.values[1684] != 0.0)) && (scratch.values[1687] != 0.0)) && (scratch.values[1688] != 0.0)) && (!(scratch.values[1689] != 0.0))) {
            scratch.values[717] = 0.0;
            scratch.node_derivatives[717] = [0.0; Instance::NODE_COUNT];
            scratch.branch_derivatives[717] = [0.0; Instance::BRANCH_COUNT];
        }

        if ((((((scratch.values[1320] != 0.0) && (scratch.values[1337] != 0.0)) && (scratch.values[1684] != 0.0)) && (scratch.values[1687] != 0.0)) && (scratch.values[1688] != 0.0)) && (!(scratch.values[1689] != 0.0))) {
            scratch.store_ad(722, &AdValue::div(AdValue::neg(scratch.ad_value(520)), AdValue::offset(AdValue::exp(AdValue::mul(AdValue::scale(AdValue::neg(scratch.ad_value(525)), scratch.values[427]), scratch.ad_value(723))), (-1.0))));
        }

        if ((scratch.values[1320] != 0.0) && (scratch.values[1337] != 0.0)) {
            scratch.store_ad(539, &AdValue::scale(AdValue::add(AdValue::add(AdValue::scale(scratch.ad_value(697), scratch.values[470]), AdValue::scale(scratch.ad_value(698), scratch.values[471])), AdValue::scale(scratch.ad_value(699), scratch.values[472])), scratch.values[419]));
        }

        scratch.values[1690] = if ((scratch.values[697] * scratch.values[470]) <= scratch.values[539]) { 1.0 } else { 0.0 };

        if (((scratch.values[1320] != 0.0) && (scratch.values[1337] != 0.0)) && (scratch.values[1690] != 0.0)) {
            scratch.values[702] = 0.0;
            scratch.node_derivatives[702] = [0.0; Instance::NODE_COUNT];
            scratch.branch_derivatives[702] = [0.0; Instance::BRANCH_COUNT];
        }

        scratch.values[1691] = if ((scratch.values[698] * scratch.values[471]) <= scratch.values[539]) { 1.0 } else { 0.0 };

        if (((scratch.values[1320] != 0.0) && (scratch.values[1337] != 0.0)) && (scratch.values[1691] != 0.0)) {
            scratch.values[703] = 0.0;
            scratch.node_derivatives[703] = [0.0; Instance::NODE_COUNT];
            scratch.branch_derivatives[703] = [0.0; Instance::BRANCH_COUNT];
        }

        scratch.values[1692] = if ((scratch.values[699] * scratch.values[472]) <= scratch.values[539]) { 1.0 } else { 0.0 };

    }
}
