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
        if (((scratch.values[1320] != 0.0) && (scratch.values[1337] != 0.0)) && (scratch.values[1692] != 0.0)) {
            scratch.values[704] = 0.0;
            scratch.node_derivatives[704] = [0.0; Instance::NODE_COUNT];
            scratch.branch_derivatives[704] = [0.0; Instance::BRANCH_COUNT];
        }

        scratch.values[1693] = if !(((scratch.values[697] == 0.0) && (scratch.values[698] == 0.0)) && (scratch.values[699] == 0.0)) { 1.0 } else { 0.0 };

        if (((scratch.values[1320] != 0.0) && (scratch.values[1337] != 0.0)) && (scratch.values[1693] != 0.0)) {
            scratch.store_ad(711, &AdValue::ln(AdValue::div_from_scalar((0.5 * scratch.values[374]), AdValue::offset(scratch.ad_value(718), 1e-21))));
        }

        if (((scratch.values[1320] != 0.0) && (scratch.values[1337] != 0.0)) && (scratch.values[1693] != 0.0)) {
            scratch.store_ad(713, &AdValue::ln(AdValue::div_from_scalar((0.5 * scratch.values[374]), AdValue::offset(scratch.ad_value(720), 1e-21))));
        }

        if (((scratch.values[1320] != 0.0) && (scratch.values[1337] != 0.0)) && (scratch.values[1693] != 0.0)) {
            scratch.store_ad(715, &AdValue::ln(AdValue::div_from_scalar((0.5 * scratch.values[374]), AdValue::offset(AdValue::abs(scratch.ad_value(722)), 1e-21))));
        }

        if ((scratch.values[1320] != 0.0) && (scratch.values[1337] != 0.0)) {
            scratch.store_ad(711, &AdValue::min_with_scalar(scratch.ad_value(711), 230.25850929940458));
        }

        if ((scratch.values[1320] != 0.0) && (scratch.values[1337] != 0.0)) {
            scratch.store_ad(712, &AdValue::exp(scratch.ad_value(711)));
        }

        if ((scratch.values[1320] != 0.0) && (scratch.values[1337] != 0.0)) {
            scratch.store_ad(713, &AdValue::min_with_scalar(scratch.ad_value(713), 230.25850929940458));
        }

        if ((scratch.values[1320] != 0.0) && (scratch.values[1337] != 0.0)) {
            scratch.store_ad(714, &AdValue::exp(scratch.ad_value(713)));
        }

        if ((scratch.values[1320] != 0.0) && (scratch.values[1337] != 0.0)) {
            scratch.store_ad(715, &AdValue::min_with_scalar(scratch.ad_value(715), 230.25850929940458));
        }

        if ((scratch.values[1320] != 0.0) && (scratch.values[1337] != 0.0)) {
            scratch.store_ad(716, &AdValue::exp(scratch.ad_value(715)));
        }

        if ((scratch.values[1320] != 0.0) && (scratch.values[1337] != 0.0)) {
            scratch.values[536] = 0.4;
            scratch.node_derivatives[536] = [0.0; Instance::NODE_COUNT];
            scratch.branch_derivatives[536] = [0.0; Instance::BRANCH_COUNT];
        }

        if ((scratch.values[1320] != 0.0) && (scratch.values[1337] != 0.0)) {
            scratch.values[537] = 0.65;
            scratch.node_derivatives[537] = [0.0; Instance::NODE_COUNT];
            scratch.branch_derivatives[537] = [0.0; Instance::BRANCH_COUNT];
        }

        if ((scratch.values[1320] != 0.0) && (scratch.values[1337] != 0.0)) {
            scratch.values[538] = 0.8;
            scratch.node_derivatives[538] = [0.0; Instance::NODE_COUNT];
            scratch.branch_derivatives[538] = [0.0; Instance::BRANCH_COUNT];
        }

        if ((scratch.values[1320] != 0.0) && (scratch.values[1337] != 0.0)) {
            scratch.store_ad(523, &AdValue::mul(AdValue::neg(scratch.ad_value(536)), scratch.ad_value(584)));
        }

        if ((scratch.values[1320] != 0.0) && (scratch.values[1337] != 0.0)) {
            scratch.store_ad(524, &AdValue::mul(AdValue::neg(scratch.ad_value(537)), scratch.ad_value(584)));
        }

        if ((scratch.values[1320] != 0.0) && (scratch.values[1337] != 0.0)) {
            scratch.store_ad(525, &AdValue::mul(AdValue::neg(scratch.ad_value(538)), scratch.ad_value(584)));
        }

        if ((scratch.values[1320] != 0.0) && (scratch.values[1337] != 0.0)) {
            scratch.values[526] = 0.1;
            scratch.node_derivatives[526] = [0.0; Instance::NODE_COUNT];
            scratch.branch_derivatives[526] = [0.0; Instance::BRANCH_COUNT];
        }

        if ((scratch.values[1320] != 0.0) && (scratch.values[1337] != 0.0)) {
            scratch.values[527] = 0.2;
            scratch.node_derivatives[527] = [0.0; Instance::NODE_COUNT];
            scratch.branch_derivatives[527] = [0.0; Instance::BRANCH_COUNT];
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

        scratch.values[1694] = if !(((scratch.values[724] == 0.0) && (scratch.values[725] == 0.0)) && (scratch.values[726] == 0.0)) { 1.0 } else { 0.0 };

        if (((scratch.values[1320] != 0.0) && (scratch.values[1337] != 0.0)) && (scratch.values[1694] != 0.0)) {
            scratch.store_ad(1341, &AdValue::mul(AdValue::scale(scratch.ad_value(735), 4.0), scratch.ad_value(735)));
        }

        if (((scratch.values[1320] != 0.0) && (scratch.values[1337] != 0.0)) && (scratch.values[1694] != 0.0)) {
            scratch.store_ad(1342, &AdValue::div(scratch.ad_value(735), scratch.ad_value(736)));
        }

        if (((scratch.values[1320] != 0.0) && (scratch.values[1337] != 0.0)) && (scratch.values[1694] != 0.0)) {
            scratch.store_ad(1343, &AdValue::add(scratch.ad_value(523), AdValue::mul(scratch.ad_value(735), scratch.ad_value(1342))));
        }

        if (((scratch.values[1320] != 0.0) && (scratch.values[1337] != 0.0)) && (scratch.values[1694] != 0.0)) {
            scratch.store_ad(1344, &AdValue::add(scratch.ad_value(736), scratch.ad_value(1343)));
        }

        if (((scratch.values[1320] != 0.0) && (scratch.values[1337] != 0.0)) && (scratch.values[1694] != 0.0)) {
            scratch.store_ad(1345, &AdValue::sub(scratch.ad_value(736), scratch.ad_value(1343)));
        }

        if (((scratch.values[1320] != 0.0) && (scratch.values[1337] != 0.0)) && (scratch.values[1694] != 0.0)) {
            scratch.store_ad(1346, &AdValue::sqrt(AdValue::add(AdValue::square(scratch.ad_value(1345)), scratch.ad_value(1341))));
        }

        if (((scratch.values[1320] != 0.0) && (scratch.values[1337] != 0.0)) && (scratch.values[1694] != 0.0)) {
            scratch.store_ad(1348, &AdValue::scale(AdValue::div(AdValue::mul(scratch.ad_value(523), scratch.ad_value(736)), AdValue::add(scratch.ad_value(1344), scratch.ad_value(1346))), 2.0));
        }

        scratch.values[1695] = if (scratch.values[523] < scratch.values[732]) { 1.0 } else { 0.0 };

        scratch.values[1696] = if ((((0.5 * (scratch.values[523] * scratch.values[427]))) as f64).abs() < 230.25850929940458) { 1.0 } else { 0.0 };

        if (((((scratch.values[1320] != 0.0) && (scratch.values[1337] != 0.0)) && (scratch.values[1694] != 0.0)) && (scratch.values[1695] != 0.0)) && (scratch.values[1696] != 0.0)) {
            scratch.store_ad(1350, &AdValue::exp(AdValue::scale(scratch.ad_value(523), (scratch.values[427] * 0.5))));
        }

        scratch.values[1697] = if ((0.5 * (scratch.values[523] * scratch.values[427])) < 0.0) { 1.0 } else { 0.0 };

        if ((((((scratch.values[1320] != 0.0) && (scratch.values[1337] != 0.0)) && (scratch.values[1694] != 0.0)) && (scratch.values[1695] != 0.0)) && (!(scratch.values[1696] != 0.0))) && (scratch.values[1697] != 0.0)) {
            let assign26230_ad_e31138: AdValue = AdValue::div_from_scalar(1e-100, AdValue::offset(AdValue::mul(AdValue::sub_from_scalar((-230.25850929940458), AdValue::scale(scratch.ad_value(523), (scratch.values[427] * 0.5))), AdValue::offset(AdValue::scale(AdValue::mul(AdValue::sub_from_scalar((-230.25850929940458), AdValue::scale(scratch.ad_value(523), (scratch.values[427] * 0.5))), AdValue::offset(AdValue::scale(AdValue::sub_from_scalar((-230.25850929940458), AdValue::scale(scratch.ad_value(523), (scratch.values[427] * 0.5))), 0.3333333333333333), 1.0)), 0.5), 1.0)), 1.0));
            scratch.store_ad(1350, &assign26230_ad_e31138);
        }

        if ((((((scratch.values[1320] != 0.0) && (scratch.values[1337] != 0.0)) && (scratch.values[1694] != 0.0)) && (scratch.values[1695] != 0.0)) && (!(scratch.values[1696] != 0.0))) && (!(scratch.values[1697] != 0.0))) {
            scratch.store_ad(1350, &AdValue::scale(AdValue::offset(AdValue::mul(AdValue::offset(AdValue::scale(scratch.ad_value(523), (scratch.values[427] * 0.5)), (-230.25850929940458)), AdValue::offset(AdValue::scale(AdValue::mul(AdValue::offset(AdValue::scale(scratch.ad_value(523), (scratch.values[427] * 0.5)), (-230.25850929940458)), AdValue::offset(AdValue::scale(AdValue::offset(AdValue::scale(scratch.ad_value(523), (scratch.values[427] * 0.5)), (-230.25850929940458)), 0.3333333333333333), 1.0)), 0.5), 1.0)), 1.0), 1e100));
        }

        if ((((scratch.values[1320] != 0.0) && (scratch.values[1337] != 0.0)) && (scratch.values[1694] != 0.0)) && (scratch.values[1695] != 0.0)) {
            scratch.store_ad(1347, &AdValue::square(scratch.ad_value(1350)));
        }

        if ((((scratch.values[1320] != 0.0) && (scratch.values[1337] != 0.0)) && (scratch.values[1694] != 0.0)) && (!(scratch.values[1695] != 0.0))) {
            scratch.store_ad(1347, &AdValue::mul(AdValue::offset(AdValue::scale(AdValue::sub(scratch.ad_value(523), scratch.ad_value(732)), scratch.values[427]), 1.0), scratch.ad_value(733)));
        }

        if ((((scratch.values[1320] != 0.0) && (scratch.values[1337] != 0.0)) && (scratch.values[1694] != 0.0)) && (!(scratch.values[1695] != 0.0))) {
            scratch.store_ad(1350, &AdValue::sqrt(scratch.ad_value(1347)));
        }

        if (((scratch.values[1320] != 0.0) && (scratch.values[1337] != 0.0)) && (scratch.values[1694] != 0.0)) {
            scratch.store_ad(1347, &AdValue::offset(scratch.ad_value(1347), (-1.0)));
        }

        if (((scratch.values[1320] != 0.0) && (scratch.values[1337] != 0.0)) && (scratch.values[1694] != 0.0)) {
            scratch.store_ad(1349, &AdValue::div_from_scalar(1.0, scratch.ad_value(1350)));
        }

        scratch.values[1698] = if (scratch.values[523] > 0.0) { 1.0 } else { 0.0 };

        if ((((scratch.values[1320] != 0.0) && (scratch.values[1337] != 0.0)) && (scratch.values[1694] != 0.0)) && (scratch.values[1698] != 0.0)) {
            scratch.store_ad(1351, &AdValue::scale(AdValue::ln(AdValue::add(AdValue::offset(scratch.ad_value(1349), 2.0), AdValue::sqrt(AdValue::mul(AdValue::offset(scratch.ad_value(1349), 1.0), AdValue::offset(scratch.ad_value(1349), 3.0))))), (scratch.values[426] * 2.0)));
        }

        if ((((scratch.values[1320] != 0.0) && (scratch.values[1337] != 0.0)) && (scratch.values[1694] != 0.0)) && (!(scratch.values[1698] != 0.0))) {
            scratch.store_ad(1351, &AdValue::sub(AdValue::scale(AdValue::ln(AdValue::add(AdValue::offset(AdValue::scale(scratch.ad_value(1350), 2.0), 1.0), AdValue::sqrt(AdValue::mul(AdValue::offset(scratch.ad_value(1350), 1.0), AdValue::offset(AdValue::scale(scratch.ad_value(1350), 3.0), 1.0))))), (scratch.values[426] * 2.0)), scratch.ad_value(523)));
        }

        if (((scratch.values[1320] != 0.0) && (scratch.values[1337] != 0.0)) && (scratch.values[1694] != 0.0)) {
            scratch.store_ad(1352, &AdValue::sub(scratch.ad_value(734), scratch.ad_value(1351)));
        }

        if (((scratch.values[1320] != 0.0) && (scratch.values[1337] != 0.0)) && (scratch.values[1694] != 0.0)) {
            scratch.store_ad(1353, &AdValue::scale(AdValue::sub(AdValue::add(scratch.ad_value(523), scratch.ad_value(1352)), AdValue::sqrt(AdValue::offset(AdValue::mul(AdValue::sub(scratch.ad_value(523), scratch.ad_value(1352)), AdValue::sub(scratch.ad_value(523), scratch.ad_value(1352))), ((4.0 * scratch.values[426]) * scratch.values[426])))), 0.5));
        }

        if (((scratch.values[1320] != 0.0) && (scratch.values[1337] != 0.0)) && (scratch.values[1694] != 0.0)) {
            scratch.store_ad(1354, &AdValue::scale(AdValue::sub(AdValue::add(scratch.ad_value(523), scratch.ad_value(737)), AdValue::sqrt(AdValue::offset(AdValue::mul(AdValue::sub(scratch.ad_value(523), scratch.ad_value(737)), AdValue::sub(scratch.ad_value(523), scratch.ad_value(737))), ((4.0 * scratch.values[424]) * scratch.values[424])))), 0.5));
        }

        if (((scratch.values[1320] != 0.0) && (scratch.values[1337] != 0.0)) && (scratch.values[1694] != 0.0)) {
            scratch.store_ad(1355, &AdValue::scale(AdValue::sub(scratch.ad_value(523), AdValue::sqrt(AdValue::offset(AdValue::mul(scratch.ad_value(523), scratch.ad_value(523)), ((4.0 * 1e-6) * 1e-6)))), 0.5));
        }

        scratch.values[1699] = if (scratch.values[724] == 0.0) { 1.0 } else { 0.0 };

        if (((scratch.values[1320] != 0.0) && (scratch.values[1337] != 0.0)) && (scratch.values[1699] != 0.0)) {
            scratch.values[1383] = 0.0;
            scratch.node_derivatives[1383] = [0.0; Instance::NODE_COUNT];
            scratch.branch_derivatives[1383] = [0.0; Instance::BRANCH_COUNT];
        }

        scratch.values[1700] = if (scratch.values[607] == 0.5) { 1.0 } else { 0.0 };

        if ((((scratch.values[1320] != 0.0) && (scratch.values[1337] != 0.0)) && (!(scratch.values[1699] != 0.0))) && (scratch.values[1700] != 0.0)) {
            scratch.store_ad(1356, &AdValue::sqrt(AdValue::sub_from_scalar(1.0, AdValue::mul(scratch.ad_value(1348), scratch.ad_value(604)))));
        }

        if ((((scratch.values[1320] != 0.0) && (scratch.values[1337] != 0.0)) && (!(scratch.values[1699] != 0.0))) && (!(scratch.values[1700] != 0.0))) {
            scratch.store_ad(1356, &AdValue::pow(AdValue::sub_from_scalar(1.0, AdValue::mul(scratch.ad_value(1348), scratch.ad_value(604))), scratch.ad_value(607)));
        }

        if (((scratch.values[1320] != 0.0) && (scratch.values[1337] != 0.0)) && (!(scratch.values[1699] != 0.0))) {
            scratch.store_ad(1357, &AdValue::mul(scratch.ad_value(595), scratch.ad_value(1347)));
        }

        scratch.values[1701] = if ((scratch.values[560] == 0.0) && (scratch.values[563] == 0.0)) { 1.0 } else { 0.0 };

        if ((((scratch.values[1320] != 0.0) && (scratch.values[1337] != 0.0)) && (!(scratch.values[1699] != 0.0))) && (scratch.values[1701] != 0.0)) {
            scratch.values[1358] = 0.0;
            scratch.node_derivatives[1358] = [0.0; Instance::NODE_COUNT];
            scratch.branch_derivatives[1358] = [0.0; Instance::BRANCH_COUNT];
        }

        if ((((scratch.values[1320] != 0.0) && (scratch.values[1337] != 0.0)) && (!(scratch.values[1699] != 0.0))) && (!(scratch.values[1701] != 0.0))) {
            scratch.store_ad(1359, &AdValue::sub(scratch.ad_value(601), scratch.ad_value(1353)));
        }

        if ((((scratch.values[1320] != 0.0) && (scratch.values[1337] != 0.0)) && (!(scratch.values[1699] != 0.0))) && (!(scratch.values[1701] != 0.0))) {
            scratch.store_ad(1360, &AdValue::sub_from_scalar(1.0, AdValue::sqrt(AdValue::sub_from_scalar(1.0, AdValue::div(scratch.ad_value(1351), scratch.ad_value(1359))))));
        }

        scratch.values[1702] = if (scratch.values[549] == 0.5) { 1.0 } else { 0.0 };

        if (((((scratch.values[1320] != 0.0) && (scratch.values[1337] != 0.0)) && (!(scratch.values[1699] != 0.0))) && (!(scratch.values[1701] != 0.0))) && (scratch.values[1702] != 0.0)) {
            scratch.values[1361] = 0.0;
            scratch.node_derivatives[1361] = [0.0; Instance::NODE_COUNT];
            scratch.branch_derivatives[1361] = [0.0; Instance::BRANCH_COUNT];
        }

        if (((((scratch.values[1320] != 0.0) && (scratch.values[1337] != 0.0)) && (!(scratch.values[1699] != 0.0))) && (!(scratch.values[1701] != 0.0))) && (!(scratch.values[1702] != 0.0))) {
            scratch.store_ad(1361, &AdValue::mul(AdValue::add(AdValue::div(AdValue::mul(AdValue::square(scratch.ad_value(1360)), AdValue::ln(scratch.ad_value(1360))), AdValue::sub_from_scalar(1.0, scratch.ad_value(1360))), scratch.ad_value(1360)), AdValue::sub_from_scalar(1.0, AdValue::scale(scratch.ad_value(549), 2.0))));
        }

        if ((((scratch.values[1320] != 0.0) && (scratch.values[1337] != 0.0)) && (!(scratch.values[1699] != 0.0))) && (!(scratch.values[1701] != 0.0))) {
            scratch.store_ad(1362, &AdValue::add(scratch.ad_value(1360), scratch.ad_value(1361)));
        }

        scratch.values[1703] = if (scratch.values[549] == 0.5) { 1.0 } else { 0.0 };

        if (((((scratch.values[1320] != 0.0) && (scratch.values[1337] != 0.0)) && (!(scratch.values[1699] != 0.0))) && (!(scratch.values[1701] != 0.0))) && (scratch.values[1703] != 0.0)) {
            scratch.store_ad(1356, &AdValue::sqrt(AdValue::mul(scratch.ad_value(1359), scratch.ad_value(628))));
        }

        if (((((scratch.values[1320] != 0.0) && (scratch.values[1337] != 0.0)) && (!(scratch.values[1699] != 0.0))) && (!(scratch.values[1701] != 0.0))) && (!(scratch.values[1703] != 0.0))) {
            scratch.store_ad(1356, &AdValue::pow(AdValue::mul(scratch.ad_value(1359), scratch.ad_value(628)), scratch.ad_value(549)));
        }

        if ((((scratch.values[1320] != 0.0) && (scratch.values[1337] != 0.0)) && (!(scratch.values[1699] != 0.0))) && (!(scratch.values[1701] != 0.0))) {
            scratch.store_ad(1363, &AdValue::mul(scratch.ad_value(622), scratch.ad_value(1356)));
        }

        if ((((scratch.values[1320] != 0.0) && (scratch.values[1337] != 0.0)) && (!(scratch.values[1699] != 0.0))) && (!(scratch.values[1701] != 0.0))) {
            scratch.store_ad(1364, &AdValue::mul(scratch.ad_value(592), AdValue::mul(AdValue::offset(scratch.ad_value(1350), (-1.0)), scratch.ad_value(1363))));
        }

        if ((((scratch.values[1320] != 0.0) && (scratch.values[1337] != 0.0)) && (!(scratch.values[1699] != 0.0))) && (!(scratch.values[1701] != 0.0))) {
            scratch.store_ad(1358, &AdValue::mul(scratch.ad_value(560), AdValue::mul(scratch.ad_value(1364), scratch.ad_value(1362))));
        }

        scratch.values[1704] = if (scratch.values[563] == 0.0) { 1.0 } else { 0.0 };

        if ((((scratch.values[1320] != 0.0) && (scratch.values[1337] != 0.0)) && (!(scratch.values[1699] != 0.0))) && (scratch.values[1704] != 0.0)) {
            scratch.values[1365] = 0.0;
            scratch.node_derivatives[1365] = [0.0; Instance::NODE_COUNT];
            scratch.branch_derivatives[1365] = [0.0; Instance::BRANCH_COUNT];
        }

        if ((((scratch.values[1320] != 0.0) && (scratch.values[1337] != 0.0)) && (!(scratch.values[1699] != 0.0))) && (!(scratch.values[1704] != 0.0))) {
            scratch.store_ad(1366, &AdValue::mul(scratch.ad_value(637), AdValue::div(AdValue::mul(scratch.ad_value(1363), scratch.ad_value(607)), scratch.ad_value(1359))));
        }

        if ((((scratch.values[1320] != 0.0) && (scratch.values[1337] != 0.0)) && (!(scratch.values[1699] != 0.0))) && (!(scratch.values[1704] != 0.0))) {
            scratch.store_ad(1367, &AdValue::div(AdValue::scale(scratch.ad_value(634), 0.666666666666667), scratch.ad_value(1366)));
        }

        if ((((scratch.values[1320] != 0.0) && (scratch.values[1337] != 0.0)) && (!(scratch.values[1699] != 0.0))) && (!(scratch.values[1704] != 0.0))) {
            scratch.store_ad(1368, &AdValue::square(scratch.ad_value(1367)));
        }

        if ((((scratch.values[1320] != 0.0) && (scratch.values[1337] != 0.0)) && (!(scratch.values[1699] != 0.0))) && (!(scratch.values[1704] != 0.0))) {
            scratch.store_ad(1369, &AdValue::sqrt(AdValue::div(AdValue::square(scratch.ad_value(1368)), AdValue::offset(AdValue::square(scratch.ad_value(1368)), 1.0))));
        }

        if ((((scratch.values[1320] != 0.0) && (scratch.values[1337] != 0.0)) && (!(scratch.values[1699] != 0.0))) && (!(scratch.values[1704] != 0.0))) {
            scratch.store_ad(1370, &AdValue::sqrt(AdValue::abs(scratch.ad_value(1369))));
        }

        if ((((scratch.values[1320] != 0.0) && (scratch.values[1337] != 0.0)) && (!(scratch.values[1699] != 0.0))) && (!(scratch.values[1704] != 0.0))) {
            scratch.store_ad(1371, &AdValue::mul(scratch.ad_value(1369), scratch.ad_value(1370)));
        }

        scratch.values[1705] = if (((-scratch.values[549]) * scratch.values[610]) == (-1.0)) { 1.0 } else { 0.0 };

        if (((((scratch.values[1320] != 0.0) && (scratch.values[1337] != 0.0)) && (!(scratch.values[1699] != 0.0))) && (!(scratch.values[1704] != 0.0))) && (scratch.values[1705] != 0.0)) {
            scratch.store_ad(1372, &AdValue::div_from_scalar(1.0, AdValue::offset(AdValue::mul(scratch.ad_value(1366), scratch.ad_value(1371)), 1.0)));
        }

        if (((((scratch.values[1320] != 0.0) && (scratch.values[1337] != 0.0)) && (!(scratch.values[1699] != 0.0))) && (!(scratch.values[1704] != 0.0))) && (!(scratch.values[1705] != 0.0))) {
            scratch.store_ad(1372, &AdValue::pow(AdValue::offset(AdValue::mul(scratch.ad_value(1366), scratch.ad_value(1371)), 1.0), AdValue::mul(AdValue::neg(scratch.ad_value(549)), scratch.ad_value(610))));
        }

        if ((((scratch.values[1320] != 0.0) && (scratch.values[1337] != 0.0)) && (!(scratch.values[1699] != 0.0))) && (!(scratch.values[1704] != 0.0))) {
            scratch.store_ad(1373, &AdValue::div(AdValue::mul(scratch.ad_value(1362), scratch.ad_value(1372)), AdValue::add(scratch.ad_value(1362), scratch.ad_value(1372))));
        }

        if ((((scratch.values[1320] != 0.0) && (scratch.values[1337] != 0.0)) && (!(scratch.values[1699] != 0.0))) && (!(scratch.values[1704] != 0.0))) {
            scratch.store_ad(1374, &AdValue::sqrt(AdValue::scale(AdValue::div(scratch.ad_value(1366), scratch.ad_value(1370)), 0.375)));
        }

        if ((((scratch.values[1320] != 0.0) && (scratch.values[1337] != 0.0)) && (!(scratch.values[1699] != 0.0))) && (!(scratch.values[1704] != 0.0))) {
            scratch.store_ad(1375, &AdValue::sub(AdValue::scale(AdValue::mul(scratch.ad_value(1367), scratch.ad_value(1370)), 2.0), scratch.ad_value(1369)));
        }

        if ((((scratch.values[1320] != 0.0) && (scratch.values[1337] != 0.0)) && (!(scratch.values[1699] != 0.0))) && (!(scratch.values[1704] != 0.0))) {
            scratch.store_ad(1376, &AdValue::add(AdValue::sub(AdValue::mul(AdValue::mul(scratch.ad_value(634), scratch.ad_value(1367)), scratch.ad_value(1370)), AdValue::mul(scratch.ad_value(634), scratch.ad_value(1369))), AdValue::scale(AdValue::mul(scratch.ad_value(1366), scratch.ad_value(1371)), 0.5)));
        }

        if ((((scratch.values[1320] != 0.0) && (scratch.values[1337] != 0.0)) && (!(scratch.values[1699] != 0.0))) && (!(scratch.values[1704] != 0.0))) {
            scratch.store_ad(1377, &AdValue::mul(AdValue::offset(scratch.ad_value(1375), (-1.0)), scratch.ad_value(1374)));
        }

        if ((((scratch.values[1320] != 0.0) && (scratch.values[1337] != 0.0)) && (!(scratch.values[1699] != 0.0))) && (!(scratch.values[1704] != 0.0))) {
            scratch.store_ad(1338, &AdValue::square(scratch.ad_value(1377)));
        }

        scratch.values[1706] = if (scratch.values[1377] > 0.0) { 1.0 } else { 0.0 };

        if (((((scratch.values[1320] != 0.0) && (scratch.values[1337] != 0.0)) && (!(scratch.values[1699] != 0.0))) && (!(scratch.values[1704] != 0.0))) && (scratch.values[1706] != 0.0)) {
            scratch.store_ad(1339, &AdValue::div_from_scalar(1.0, AdValue::offset(AdValue::scale(scratch.ad_value(1377), scratch.values[428]), 1.0)));
        }

        if (((((scratch.values[1320] != 0.0) && (scratch.values[1337] != 0.0)) && (!(scratch.values[1699] != 0.0))) && (!(scratch.values[1704] != 0.0))) && (!(scratch.values[1706] != 0.0))) {
            scratch.store_ad(1339, &AdValue::div_from_scalar(1.0, AdValue::sub_from_scalar(1.0, AdValue::scale(scratch.ad_value(1377), scratch.values[428]))));
        }

        scratch.values[1707] = if (((-scratch.values[1338]) + scratch.values[1376]) > (-230.25850929940458)) { 1.0 } else { 0.0 };

        if (((((scratch.values[1320] != 0.0) && (scratch.values[1337] != 0.0)) && (!(scratch.values[1699] != 0.0))) && (!(scratch.values[1704] != 0.0))) && (scratch.values[1707] != 0.0)) {
            scratch.store_ad(1356, &AdValue::exp(AdValue::sub(scratch.ad_value(1376), scratch.ad_value(1338))));
        }

        if (((((scratch.values[1320] != 0.0) && (scratch.values[1337] != 0.0)) && (!(scratch.values[1699] != 0.0))) && (!(scratch.values[1704] != 0.0))) && (!(scratch.values[1707] != 0.0))) {
            let assign26810_ad_e32081: AdValue = AdValue::div_from_scalar(1e-100, AdValue::offset(AdValue::mul(AdValue::sub_from_scalar((-230.25850929940458), AdValue::sub(scratch.ad_value(1376), scratch.ad_value(1338))), AdValue::offset(AdValue::scale(AdValue::mul(AdValue::sub_from_scalar((-230.25850929940458), AdValue::sub(scratch.ad_value(1376), scratch.ad_value(1338))), AdValue::offset(AdValue::scale(AdValue::sub_from_scalar((-230.25850929940458), AdValue::sub(scratch.ad_value(1376), scratch.ad_value(1338))), 0.3333333333333333), 1.0)), 0.5), 1.0)), 1.0));
            scratch.store_ad(1356, &assign26810_ad_e32081);
        }

        if ((((scratch.values[1320] != 0.0) && (scratch.values[1337] != 0.0)) && (!(scratch.values[1699] != 0.0))) && (!(scratch.values[1704] != 0.0))) {
            scratch.store_ad(1340, &AdValue::mul(AdValue::add(AdValue::add(AdValue::scale(scratch.ad_value(1339), 0.29214664), AdValue::scale(AdValue::square(scratch.ad_value(1339)), scratch.values[429])), AdValue::scale(AdValue::mul(AdValue::square(scratch.ad_value(1339)), scratch.ad_value(1339)), scratch.values[430])), scratch.ad_value(1356)));
        }

        scratch.values[1708] = if (scratch.values[1377] > 0.0) { 1.0 } else { 0.0 };

        if (((((scratch.values[1320] != 0.0) && (scratch.values[1337] != 0.0)) && (!(scratch.values[1699] != 0.0))) && (!(scratch.values[1704] != 0.0))) && (scratch.values[1708] != 0.0)) {
            scratch.values[1378] = scratch.values[1340];
            scratch.node_derivatives[1378] = scratch.node_derivatives[1340];
            scratch.branch_derivatives[1378] = scratch.branch_derivatives[1340];
        }

        scratch.values[1709] = if (scratch.values[1376] > (-230.25850929940458)) { 1.0 } else { 0.0 };

        if ((((((scratch.values[1320] != 0.0) && (scratch.values[1337] != 0.0)) && (!(scratch.values[1699] != 0.0))) && (!(scratch.values[1704] != 0.0))) && (!(scratch.values[1708] != 0.0))) && (scratch.values[1709] != 0.0)) {
            scratch.store_ad(1356, &AdValue::exp(scratch.ad_value(1376)));
        }

        if ((((((scratch.values[1320] != 0.0) && (scratch.values[1337] != 0.0)) && (!(scratch.values[1699] != 0.0))) && (!(scratch.values[1704] != 0.0))) && (!(scratch.values[1708] != 0.0))) && (!(scratch.values[1709] != 0.0))) {
            scratch.store_ad(1356, &AdValue::div_from_scalar(1e-100, AdValue::offset(AdValue::mul(AdValue::sub_from_scalar((-230.25850929940458), scratch.ad_value(1376)), AdValue::offset(AdValue::scale(AdValue::mul(AdValue::sub_from_scalar((-230.25850929940458), scratch.ad_value(1376)), AdValue::offset(AdValue::scale(AdValue::sub_from_scalar((-230.25850929940458), scratch.ad_value(1376)), 0.3333333333333333), 1.0)), 0.5), 1.0)), 1.0)));
        }

        if (((((scratch.values[1320] != 0.0) && (scratch.values[1337] != 0.0)) && (!(scratch.values[1699] != 0.0))) && (!(scratch.values[1704] != 0.0))) && (!(scratch.values[1708] != 0.0))) {
            scratch.store_ad(1378, &AdValue::sub(AdValue::scale(scratch.ad_value(1356), 2.0), scratch.ad_value(1340)));
        }

        if ((((scratch.values[1320] != 0.0) && (scratch.values[1337] != 0.0)) && (!(scratch.values[1699] != 0.0))) && (!(scratch.values[1704] != 0.0))) {
            scratch.store_ad(1379, &AdValue::scale(AdValue::div(AdValue::mul(scratch.ad_value(634), scratch.ad_value(1378)), scratch.ad_value(1374)), (1.772453850905516 * 0.5)));
        }

        if ((((scratch.values[1320] != 0.0) && (scratch.values[1337] != 0.0)) && (!(scratch.values[1699] != 0.0))) && (!(scratch.values[1704] != 0.0))) {
            scratch.store_ad(1365, &AdValue::mul(scratch.ad_value(563), AdValue::mul(AdValue::mul(scratch.ad_value(1364), scratch.ad_value(1379)), scratch.ad_value(1373))));
        }

        scratch.values[1710] = if (scratch.values[569] == 0.0) { 1.0 } else { 0.0 };

        if ((((scratch.values[1320] != 0.0) && (scratch.values[1337] != 0.0)) && (!(scratch.values[1699] != 0.0))) && (scratch.values[1710] != 0.0)) {
            scratch.values[1380] = 0.0;
            scratch.node_derivatives[1380] = [0.0; Instance::NODE_COUNT];
            scratch.branch_derivatives[1380] = [0.0; Instance::BRANCH_COUNT];
        }

        scratch.values[1711] = if (scratch.values[549] == 0.5) { 1.0 } else { 0.0 };

        if (((((scratch.values[1320] != 0.0) && (scratch.values[1337] != 0.0)) && (!(scratch.values[1699] != 0.0))) && (!(scratch.values[1710] != 0.0))) && (scratch.values[1711] != 0.0)) {
            scratch.store_ad(1356, &AdValue::sqrt(AdValue::mul(AdValue::sub(scratch.ad_value(546), scratch.ad_value(1354)), scratch.ad_value(628))));
        }

        if (((((scratch.values[1320] != 0.0) && (scratch.values[1337] != 0.0)) && (!(scratch.values[1699] != 0.0))) && (!(scratch.values[1710] != 0.0))) && (!(scratch.values[1711] != 0.0))) {
            scratch.store_ad(1356, &AdValue::pow(AdValue::mul(AdValue::sub(scratch.ad_value(546), scratch.ad_value(1354)), scratch.ad_value(628)), scratch.ad_value(549)));
        }

        if ((((scratch.values[1320] != 0.0) && (scratch.values[1337] != 0.0)) && (!(scratch.values[1699] != 0.0))) && (!(scratch.values[1710] != 0.0))) {
            scratch.store_ad(1381, &AdValue::mul(scratch.ad_value(610), AdValue::div(AdValue::mul(AdValue::sub(scratch.ad_value(546), scratch.ad_value(1354)), scratch.ad_value(625)), scratch.ad_value(1356))));
        }

        scratch.values[1712] = if (((((-scratch.values[640]) / scratch.values[1381])) as f64).abs() < 230.25850929940458) { 1.0 } else { 0.0 };

        if (((((scratch.values[1320] != 0.0) && (scratch.values[1337] != 0.0)) && (!(scratch.values[1699] != 0.0))) && (!(scratch.values[1710] != 0.0))) && (scratch.values[1712] != 0.0)) {
            scratch.store_ad(1356, &AdValue::exp(AdValue::div(AdValue::neg(scratch.ad_value(640)), scratch.ad_value(1381))));
        }

        scratch.values[1713] = if (((-scratch.values[640]) / scratch.values[1381]) < 0.0) { 1.0 } else { 0.0 };

        if ((((((scratch.values[1320] != 0.0) && (scratch.values[1337] != 0.0)) && (!(scratch.values[1699] != 0.0))) && (!(scratch.values[1710] != 0.0))) && (!(scratch.values[1712] != 0.0))) && (scratch.values[1713] != 0.0)) {
            let assign27000_ad_e32408: AdValue = AdValue::offset(AdValue::mul(AdValue::sub_from_scalar((-230.25850929940458), AdValue::div(AdValue::neg(scratch.ad_value(640)), scratch.ad_value(1381))), AdValue::offset(AdValue::scale(AdValue::mul(AdValue::sub_from_scalar((-230.25850929940458), AdValue::div(AdValue::neg(scratch.ad_value(640)), scratch.ad_value(1381))), AdValue::offset(AdValue::scale(AdValue::sub_from_scalar((-230.25850929940458), AdValue::div(AdValue::neg(scratch.ad_value(640)), scratch.ad_value(1381))), 0.3333333333333333), 1.0)), 0.5), 1.0)), 1.0);
            scratch.store_ad(1356, &AdValue::div_from_scalar(1e-100, assign27000_ad_e32408));
        }

        if ((((((scratch.values[1320] != 0.0) && (scratch.values[1337] != 0.0)) && (!(scratch.values[1699] != 0.0))) && (!(scratch.values[1710] != 0.0))) && (!(scratch.values[1712] != 0.0))) && (!(scratch.values[1713] != 0.0))) {
            let assign27010_ad_e32458: AdValue = AdValue::scale(AdValue::offset(AdValue::mul(AdValue::offset(AdValue::div(AdValue::neg(scratch.ad_value(640)), scratch.ad_value(1381)), (-230.25850929940458)), AdValue::offset(AdValue::scale(AdValue::mul(AdValue::offset(AdValue::div(AdValue::neg(scratch.ad_value(640)), scratch.ad_value(1381)), (-230.25850929940458)), AdValue::offset(AdValue::scale(AdValue::offset(AdValue::div(AdValue::neg(scratch.ad_value(640)), scratch.ad_value(1381)), (-230.25850929940458)), 0.3333333333333333), 1.0)), 0.5), 1.0)), 1.0), 1e100);
            scratch.store_ad(1356, &assign27010_ad_e32458);
        }

        if ((((scratch.values[1320] != 0.0) && (scratch.values[1337] != 0.0)) && (!(scratch.values[1699] != 0.0))) && (!(scratch.values[1710] != 0.0))) {
            scratch.store_ad(1380, &AdValue::mul(scratch.ad_value(569), AdValue::mul(AdValue::mul(AdValue::mul(scratch.ad_value(523), scratch.ad_value(1381)), scratch.ad_value(1381)), scratch.ad_value(1356))));
        }

        scratch.values[1714] = if (scratch.values[578] > 1000.0) { 1.0 } else { 0.0 };

        if ((((scratch.values[1320] != 0.0) && (scratch.values[1337] != 0.0)) && (!(scratch.values[1699] != 0.0))) && (scratch.values[1714] != 0.0)) {
            scratch.values[1382] = 1.0;
            scratch.node_derivatives[1382] = [0.0; Instance::NODE_COUNT];
            scratch.branch_derivatives[1382] = [0.0; Instance::BRANCH_COUNT];
        }

        scratch.values[1715] = if (scratch.values[1355] > ((-scratch.values[500]) * scratch.values[578])) { 1.0 } else { 0.0 };

        scratch.values[1716] = if (scratch.values[581] == 4.0) { 1.0 } else { 0.0 };

        if ((((((scratch.values[1320] != 0.0) && (scratch.values[1337] != 0.0)) && (!(scratch.values[1699] != 0.0))) && (!(scratch.values[1714] != 0.0))) && (scratch.values[1715] != 0.0)) && (scratch.values[1716] != 0.0)) {
            scratch.store_ad(1356, &AdValue::mul(AdValue::mul(AdValue::mul(AdValue::mul(scratch.ad_value(1355), scratch.ad_value(646)), AdValue::mul(scratch.ad_value(1355), scratch.ad_value(646))), AdValue::mul(scratch.ad_value(1355), scratch.ad_value(646))), AdValue::mul(scratch.ad_value(1355), scratch.ad_value(646))));
        }

        if ((((((scratch.values[1320] != 0.0) && (scratch.values[1337] != 0.0)) && (!(scratch.values[1699] != 0.0))) && (!(scratch.values[1714] != 0.0))) && (scratch.values[1715] != 0.0)) && (!(scratch.values[1716] != 0.0))) {
            scratch.store_ad(1356, &AdValue::pow(AdValue::abs(AdValue::mul(scratch.ad_value(1355), scratch.ad_value(646))), scratch.ad_value(581)));
        }

        if (((((scratch.values[1320] != 0.0) && (scratch.values[1337] != 0.0)) && (!(scratch.values[1699] != 0.0))) && (!(scratch.values[1714] != 0.0))) && (scratch.values[1715] != 0.0)) {
            scratch.store_ad(1382, &AdValue::div_from_scalar(1.0, AdValue::sub_from_scalar(1.0, scratch.ad_value(1356))));
        }

        if (((((scratch.values[1320] != 0.0) && (scratch.values[1337] != 0.0)) && (!(scratch.values[1699] != 0.0))) && (!(scratch.values[1714] != 0.0))) && (!(scratch.values[1715] != 0.0))) {
            scratch.store_ad(1382, &AdValue::add(scratch.ad_value(643), AdValue::mul(AdValue::add(scratch.ad_value(1355), AdValue::scale(scratch.ad_value(578), scratch.values[500])), scratch.ad_value(649))));
        }

        if (((scratch.values[1320] != 0.0) && (scratch.values[1337] != 0.0)) && (!(scratch.values[1699] != 0.0))) {
            scratch.store_ad(1383, &AdValue::mul(AdValue::add(AdValue::add(AdValue::add(scratch.ad_value(1357), scratch.ad_value(1358)), scratch.ad_value(1365)), scratch.ad_value(1380)), scratch.ad_value(1382)));
        }

        scratch.values[1717] = if (scratch.values[725] == 0.0) { 1.0 } else { 0.0 };

        if (((scratch.values[1320] != 0.0) && (scratch.values[1337] != 0.0)) && (scratch.values[1717] != 0.0)) {
            scratch.values[1384] = 0.0;
            scratch.node_derivatives[1384] = [0.0; Instance::NODE_COUNT];
            scratch.branch_derivatives[1384] = [0.0; Instance::BRANCH_COUNT];
        }

        scratch.values[1718] = if (scratch.values[608] == 0.5) { 1.0 } else { 0.0 };

        if ((((scratch.values[1320] != 0.0) && (scratch.values[1337] != 0.0)) && (!(scratch.values[1717] != 0.0))) && (scratch.values[1718] != 0.0)) {
            scratch.store_ad(1356, &AdValue::sqrt(AdValue::sub_from_scalar(1.0, AdValue::mul(scratch.ad_value(1348), scratch.ad_value(605)))));
        }

        if ((((scratch.values[1320] != 0.0) && (scratch.values[1337] != 0.0)) && (!(scratch.values[1717] != 0.0))) && (!(scratch.values[1718] != 0.0))) {
            scratch.store_ad(1356, &AdValue::pow(AdValue::sub_from_scalar(1.0, AdValue::mul(scratch.ad_value(1348), scratch.ad_value(605))), scratch.ad_value(608)));
        }

        if (((scratch.values[1320] != 0.0) && (scratch.values[1337] != 0.0)) && (!(scratch.values[1717] != 0.0))) {
            scratch.store_ad(1357, &AdValue::mul(scratch.ad_value(596), scratch.ad_value(1347)));
        }

        scratch.values[1719] = if ((scratch.values[561] == 0.0) && (scratch.values[564] == 0.0)) { 1.0 } else { 0.0 };

        if ((((scratch.values[1320] != 0.0) && (scratch.values[1337] != 0.0)) && (!(scratch.values[1717] != 0.0))) && (scratch.values[1719] != 0.0)) {
            scratch.values[1358] = 0.0;
            scratch.node_derivatives[1358] = [0.0; Instance::NODE_COUNT];
            scratch.branch_derivatives[1358] = [0.0; Instance::BRANCH_COUNT];
        }

        if ((((scratch.values[1320] != 0.0) && (scratch.values[1337] != 0.0)) && (!(scratch.values[1717] != 0.0))) && (!(scratch.values[1719] != 0.0))) {
            scratch.store_ad(1359, &AdValue::sub(scratch.ad_value(602), scratch.ad_value(1353)));
        }

        if ((((scratch.values[1320] != 0.0) && (scratch.values[1337] != 0.0)) && (!(scratch.values[1717] != 0.0))) && (!(scratch.values[1719] != 0.0))) {
            scratch.store_ad(1360, &AdValue::sub_from_scalar(1.0, AdValue::sqrt(AdValue::sub_from_scalar(1.0, AdValue::div(scratch.ad_value(1351), scratch.ad_value(1359))))));
        }

        scratch.values[1720] = if (scratch.values[550] == 0.5) { 1.0 } else { 0.0 };

        if (((((scratch.values[1320] != 0.0) && (scratch.values[1337] != 0.0)) && (!(scratch.values[1717] != 0.0))) && (!(scratch.values[1719] != 0.0))) && (scratch.values[1720] != 0.0)) {
            scratch.values[1361] = 0.0;
            scratch.node_derivatives[1361] = [0.0; Instance::NODE_COUNT];
            scratch.branch_derivatives[1361] = [0.0; Instance::BRANCH_COUNT];
        }

    }

    pub(super) fn stamp_transient_block_21(
        &mut self,
        ctx: &GeneratedEvalContext<'_>,
        stamper: &mut GeneratedStamper<'_>,
        scratch: &mut Scratch,
    ) {
        let _ = stamper;
        if (((((scratch.values[1320] != 0.0) && (scratch.values[1337] != 0.0)) && (!(scratch.values[1717] != 0.0))) && (!(scratch.values[1719] != 0.0))) && (!(scratch.values[1720] != 0.0))) {
            scratch.store_ad(1361, &AdValue::mul(AdValue::add(AdValue::div(AdValue::mul(AdValue::square(scratch.ad_value(1360)), AdValue::ln(scratch.ad_value(1360))), AdValue::sub_from_scalar(1.0, scratch.ad_value(1360))), scratch.ad_value(1360)), AdValue::sub_from_scalar(1.0, AdValue::scale(scratch.ad_value(550), 2.0))));
        }

        if ((((scratch.values[1320] != 0.0) && (scratch.values[1337] != 0.0)) && (!(scratch.values[1717] != 0.0))) && (!(scratch.values[1719] != 0.0))) {
            scratch.store_ad(1362, &AdValue::add(scratch.ad_value(1360), scratch.ad_value(1361)));
        }

        scratch.values[1721] = if (scratch.values[550] == 0.5) { 1.0 } else { 0.0 };

        if (((((scratch.values[1320] != 0.0) && (scratch.values[1337] != 0.0)) && (!(scratch.values[1717] != 0.0))) && (!(scratch.values[1719] != 0.0))) && (scratch.values[1721] != 0.0)) {
            scratch.store_ad(1356, &AdValue::sqrt(AdValue::mul(scratch.ad_value(1359), scratch.ad_value(629))));
        }

        if (((((scratch.values[1320] != 0.0) && (scratch.values[1337] != 0.0)) && (!(scratch.values[1717] != 0.0))) && (!(scratch.values[1719] != 0.0))) && (!(scratch.values[1721] != 0.0))) {
            scratch.store_ad(1356, &AdValue::pow(AdValue::mul(scratch.ad_value(1359), scratch.ad_value(629)), scratch.ad_value(550)));
        }

        if ((((scratch.values[1320] != 0.0) && (scratch.values[1337] != 0.0)) && (!(scratch.values[1717] != 0.0))) && (!(scratch.values[1719] != 0.0))) {
            scratch.store_ad(1363, &AdValue::mul(scratch.ad_value(623), scratch.ad_value(1356)));
        }

        if ((((scratch.values[1320] != 0.0) && (scratch.values[1337] != 0.0)) && (!(scratch.values[1717] != 0.0))) && (!(scratch.values[1719] != 0.0))) {
            scratch.store_ad(1364, &AdValue::mul(scratch.ad_value(593), AdValue::mul(AdValue::offset(scratch.ad_value(1350), (-1.0)), scratch.ad_value(1363))));
        }

        if ((((scratch.values[1320] != 0.0) && (scratch.values[1337] != 0.0)) && (!(scratch.values[1717] != 0.0))) && (!(scratch.values[1719] != 0.0))) {
            scratch.store_ad(1358, &AdValue::mul(scratch.ad_value(561), AdValue::mul(scratch.ad_value(1364), scratch.ad_value(1362))));
        }

        scratch.values[1722] = if (scratch.values[564] == 0.0) { 1.0 } else { 0.0 };

        if ((((scratch.values[1320] != 0.0) && (scratch.values[1337] != 0.0)) && (!(scratch.values[1717] != 0.0))) && (scratch.values[1722] != 0.0)) {
            scratch.values[1365] = 0.0;
            scratch.node_derivatives[1365] = [0.0; Instance::NODE_COUNT];
            scratch.branch_derivatives[1365] = [0.0; Instance::BRANCH_COUNT];
        }

        if ((((scratch.values[1320] != 0.0) && (scratch.values[1337] != 0.0)) && (!(scratch.values[1717] != 0.0))) && (!(scratch.values[1722] != 0.0))) {
            scratch.store_ad(1366, &AdValue::mul(scratch.ad_value(638), AdValue::div(AdValue::mul(scratch.ad_value(1363), scratch.ad_value(608)), scratch.ad_value(1359))));
        }

        if ((((scratch.values[1320] != 0.0) && (scratch.values[1337] != 0.0)) && (!(scratch.values[1717] != 0.0))) && (!(scratch.values[1722] != 0.0))) {
            scratch.store_ad(1367, &AdValue::div(AdValue::scale(scratch.ad_value(635), 0.666666666666667), scratch.ad_value(1366)));
        }

        if ((((scratch.values[1320] != 0.0) && (scratch.values[1337] != 0.0)) && (!(scratch.values[1717] != 0.0))) && (!(scratch.values[1722] != 0.0))) {
            scratch.store_ad(1368, &AdValue::square(scratch.ad_value(1367)));
        }

        if ((((scratch.values[1320] != 0.0) && (scratch.values[1337] != 0.0)) && (!(scratch.values[1717] != 0.0))) && (!(scratch.values[1722] != 0.0))) {
            scratch.store_ad(1369, &AdValue::sqrt(AdValue::div(AdValue::square(scratch.ad_value(1368)), AdValue::offset(AdValue::square(scratch.ad_value(1368)), 1.0))));
        }

        if ((((scratch.values[1320] != 0.0) && (scratch.values[1337] != 0.0)) && (!(scratch.values[1717] != 0.0))) && (!(scratch.values[1722] != 0.0))) {
            scratch.store_ad(1370, &AdValue::sqrt(AdValue::abs(scratch.ad_value(1369))));
        }

        if ((((scratch.values[1320] != 0.0) && (scratch.values[1337] != 0.0)) && (!(scratch.values[1717] != 0.0))) && (!(scratch.values[1722] != 0.0))) {
            scratch.store_ad(1371, &AdValue::mul(scratch.ad_value(1369), scratch.ad_value(1370)));
        }

        scratch.values[1723] = if (((-scratch.values[550]) * scratch.values[611]) == (-1.0)) { 1.0 } else { 0.0 };

        if (((((scratch.values[1320] != 0.0) && (scratch.values[1337] != 0.0)) && (!(scratch.values[1717] != 0.0))) && (!(scratch.values[1722] != 0.0))) && (scratch.values[1723] != 0.0)) {
            scratch.store_ad(1372, &AdValue::div_from_scalar(1.0, AdValue::offset(AdValue::mul(scratch.ad_value(1366), scratch.ad_value(1371)), 1.0)));
        }

        if (((((scratch.values[1320] != 0.0) && (scratch.values[1337] != 0.0)) && (!(scratch.values[1717] != 0.0))) && (!(scratch.values[1722] != 0.0))) && (!(scratch.values[1723] != 0.0))) {
            scratch.store_ad(1372, &AdValue::pow(AdValue::offset(AdValue::mul(scratch.ad_value(1366), scratch.ad_value(1371)), 1.0), AdValue::mul(AdValue::neg(scratch.ad_value(550)), scratch.ad_value(611))));
        }

        if ((((scratch.values[1320] != 0.0) && (scratch.values[1337] != 0.0)) && (!(scratch.values[1717] != 0.0))) && (!(scratch.values[1722] != 0.0))) {
            scratch.store_ad(1373, &AdValue::div(AdValue::mul(scratch.ad_value(1362), scratch.ad_value(1372)), AdValue::add(scratch.ad_value(1362), scratch.ad_value(1372))));
        }

        if ((((scratch.values[1320] != 0.0) && (scratch.values[1337] != 0.0)) && (!(scratch.values[1717] != 0.0))) && (!(scratch.values[1722] != 0.0))) {
            scratch.store_ad(1374, &AdValue::sqrt(AdValue::scale(AdValue::div(scratch.ad_value(1366), scratch.ad_value(1370)), 0.375)));
        }

        if ((((scratch.values[1320] != 0.0) && (scratch.values[1337] != 0.0)) && (!(scratch.values[1717] != 0.0))) && (!(scratch.values[1722] != 0.0))) {
            scratch.store_ad(1375, &AdValue::sub(AdValue::scale(AdValue::mul(scratch.ad_value(1367), scratch.ad_value(1370)), 2.0), scratch.ad_value(1369)));
        }

        if ((((scratch.values[1320] != 0.0) && (scratch.values[1337] != 0.0)) && (!(scratch.values[1717] != 0.0))) && (!(scratch.values[1722] != 0.0))) {
            scratch.store_ad(1376, &AdValue::add(AdValue::sub(AdValue::mul(AdValue::mul(scratch.ad_value(635), scratch.ad_value(1367)), scratch.ad_value(1370)), AdValue::mul(scratch.ad_value(635), scratch.ad_value(1369))), AdValue::scale(AdValue::mul(scratch.ad_value(1366), scratch.ad_value(1371)), 0.5)));
        }

        if ((((scratch.values[1320] != 0.0) && (scratch.values[1337] != 0.0)) && (!(scratch.values[1717] != 0.0))) && (!(scratch.values[1722] != 0.0))) {
            scratch.store_ad(1377, &AdValue::mul(AdValue::offset(scratch.ad_value(1375), (-1.0)), scratch.ad_value(1374)));
        }

        if ((((scratch.values[1320] != 0.0) && (scratch.values[1337] != 0.0)) && (!(scratch.values[1717] != 0.0))) && (!(scratch.values[1722] != 0.0))) {
            scratch.store_ad(1338, &AdValue::square(scratch.ad_value(1377)));
        }

        scratch.values[1724] = if (scratch.values[1377] > 0.0) { 1.0 } else { 0.0 };

        if (((((scratch.values[1320] != 0.0) && (scratch.values[1337] != 0.0)) && (!(scratch.values[1717] != 0.0))) && (!(scratch.values[1722] != 0.0))) && (scratch.values[1724] != 0.0)) {
            scratch.store_ad(1339, &AdValue::div_from_scalar(1.0, AdValue::offset(AdValue::scale(scratch.ad_value(1377), scratch.values[428]), 1.0)));
        }

        if (((((scratch.values[1320] != 0.0) && (scratch.values[1337] != 0.0)) && (!(scratch.values[1717] != 0.0))) && (!(scratch.values[1722] != 0.0))) && (!(scratch.values[1724] != 0.0))) {
            scratch.store_ad(1339, &AdValue::div_from_scalar(1.0, AdValue::sub_from_scalar(1.0, AdValue::scale(scratch.ad_value(1377), scratch.values[428]))));
        }

        scratch.values[1725] = if (((-scratch.values[1338]) + scratch.values[1376]) > (-230.25850929940458)) { 1.0 } else { 0.0 };

        if (((((scratch.values[1320] != 0.0) && (scratch.values[1337] != 0.0)) && (!(scratch.values[1717] != 0.0))) && (!(scratch.values[1722] != 0.0))) && (scratch.values[1725] != 0.0)) {
            scratch.store_ad(1356, &AdValue::exp(AdValue::sub(scratch.ad_value(1376), scratch.ad_value(1338))));
        }

        if (((((scratch.values[1320] != 0.0) && (scratch.values[1337] != 0.0)) && (!(scratch.values[1717] != 0.0))) && (!(scratch.values[1722] != 0.0))) && (!(scratch.values[1725] != 0.0))) {
            let assign27560_ad_e33287: AdValue = AdValue::div_from_scalar(1e-100, AdValue::offset(AdValue::mul(AdValue::sub_from_scalar((-230.25850929940458), AdValue::sub(scratch.ad_value(1376), scratch.ad_value(1338))), AdValue::offset(AdValue::scale(AdValue::mul(AdValue::sub_from_scalar((-230.25850929940458), AdValue::sub(scratch.ad_value(1376), scratch.ad_value(1338))), AdValue::offset(AdValue::scale(AdValue::sub_from_scalar((-230.25850929940458), AdValue::sub(scratch.ad_value(1376), scratch.ad_value(1338))), 0.3333333333333333), 1.0)), 0.5), 1.0)), 1.0));
            scratch.store_ad(1356, &assign27560_ad_e33287);
        }

        if ((((scratch.values[1320] != 0.0) && (scratch.values[1337] != 0.0)) && (!(scratch.values[1717] != 0.0))) && (!(scratch.values[1722] != 0.0))) {
            scratch.store_ad(1340, &AdValue::mul(AdValue::add(AdValue::add(AdValue::scale(scratch.ad_value(1339), 0.29214664), AdValue::scale(AdValue::square(scratch.ad_value(1339)), scratch.values[429])), AdValue::scale(AdValue::mul(AdValue::square(scratch.ad_value(1339)), scratch.ad_value(1339)), scratch.values[430])), scratch.ad_value(1356)));
        }

        scratch.values[1726] = if (scratch.values[1377] > 0.0) { 1.0 } else { 0.0 };

        if (((((scratch.values[1320] != 0.0) && (scratch.values[1337] != 0.0)) && (!(scratch.values[1717] != 0.0))) && (!(scratch.values[1722] != 0.0))) && (scratch.values[1726] != 0.0)) {
            scratch.values[1378] = scratch.values[1340];
            scratch.node_derivatives[1378] = scratch.node_derivatives[1340];
            scratch.branch_derivatives[1378] = scratch.branch_derivatives[1340];
        }

        scratch.values[1727] = if (scratch.values[1376] > (-230.25850929940458)) { 1.0 } else { 0.0 };

        if ((((((scratch.values[1320] != 0.0) && (scratch.values[1337] != 0.0)) && (!(scratch.values[1717] != 0.0))) && (!(scratch.values[1722] != 0.0))) && (!(scratch.values[1726] != 0.0))) && (scratch.values[1727] != 0.0)) {
            scratch.store_ad(1356, &AdValue::exp(scratch.ad_value(1376)));
        }

        if ((((((scratch.values[1320] != 0.0) && (scratch.values[1337] != 0.0)) && (!(scratch.values[1717] != 0.0))) && (!(scratch.values[1722] != 0.0))) && (!(scratch.values[1726] != 0.0))) && (!(scratch.values[1727] != 0.0))) {
            scratch.store_ad(1356, &AdValue::div_from_scalar(1e-100, AdValue::offset(AdValue::mul(AdValue::sub_from_scalar((-230.25850929940458), scratch.ad_value(1376)), AdValue::offset(AdValue::scale(AdValue::mul(AdValue::sub_from_scalar((-230.25850929940458), scratch.ad_value(1376)), AdValue::offset(AdValue::scale(AdValue::sub_from_scalar((-230.25850929940458), scratch.ad_value(1376)), 0.3333333333333333), 1.0)), 0.5), 1.0)), 1.0)));
        }

        if (((((scratch.values[1320] != 0.0) && (scratch.values[1337] != 0.0)) && (!(scratch.values[1717] != 0.0))) && (!(scratch.values[1722] != 0.0))) && (!(scratch.values[1726] != 0.0))) {
            scratch.store_ad(1378, &AdValue::sub(AdValue::scale(scratch.ad_value(1356), 2.0), scratch.ad_value(1340)));
        }

        if ((((scratch.values[1320] != 0.0) && (scratch.values[1337] != 0.0)) && (!(scratch.values[1717] != 0.0))) && (!(scratch.values[1722] != 0.0))) {
            scratch.store_ad(1379, &AdValue::scale(AdValue::div(AdValue::mul(scratch.ad_value(635), scratch.ad_value(1378)), scratch.ad_value(1374)), (1.772453850905516 * 0.5)));
        }

        if ((((scratch.values[1320] != 0.0) && (scratch.values[1337] != 0.0)) && (!(scratch.values[1717] != 0.0))) && (!(scratch.values[1722] != 0.0))) {
            scratch.store_ad(1365, &AdValue::mul(scratch.ad_value(564), AdValue::mul(AdValue::mul(scratch.ad_value(1364), scratch.ad_value(1379)), scratch.ad_value(1373))));
        }

        scratch.values[1728] = if (scratch.values[570] == 0.0) { 1.0 } else { 0.0 };

        if ((((scratch.values[1320] != 0.0) && (scratch.values[1337] != 0.0)) && (!(scratch.values[1717] != 0.0))) && (scratch.values[1728] != 0.0)) {
            scratch.values[1380] = 0.0;
            scratch.node_derivatives[1380] = [0.0; Instance::NODE_COUNT];
            scratch.branch_derivatives[1380] = [0.0; Instance::BRANCH_COUNT];
        }

        scratch.values[1729] = if (scratch.values[550] == 0.5) { 1.0 } else { 0.0 };

        if (((((scratch.values[1320] != 0.0) && (scratch.values[1337] != 0.0)) && (!(scratch.values[1717] != 0.0))) && (!(scratch.values[1728] != 0.0))) && (scratch.values[1729] != 0.0)) {
            scratch.store_ad(1356, &AdValue::sqrt(AdValue::mul(AdValue::sub(scratch.ad_value(547), scratch.ad_value(1354)), scratch.ad_value(629))));
        }

        if (((((scratch.values[1320] != 0.0) && (scratch.values[1337] != 0.0)) && (!(scratch.values[1717] != 0.0))) && (!(scratch.values[1728] != 0.0))) && (!(scratch.values[1729] != 0.0))) {
            scratch.store_ad(1356, &AdValue::pow(AdValue::mul(AdValue::sub(scratch.ad_value(547), scratch.ad_value(1354)), scratch.ad_value(629)), scratch.ad_value(550)));
        }

        if ((((scratch.values[1320] != 0.0) && (scratch.values[1337] != 0.0)) && (!(scratch.values[1717] != 0.0))) && (!(scratch.values[1728] != 0.0))) {
            scratch.store_ad(1381, &AdValue::mul(scratch.ad_value(611), AdValue::div(AdValue::mul(AdValue::sub(scratch.ad_value(547), scratch.ad_value(1354)), scratch.ad_value(626)), scratch.ad_value(1356))));
        }

        scratch.values[1730] = if (((((-scratch.values[641]) / scratch.values[1381])) as f64).abs() < 230.25850929940458) { 1.0 } else { 0.0 };

        if (((((scratch.values[1320] != 0.0) && (scratch.values[1337] != 0.0)) && (!(scratch.values[1717] != 0.0))) && (!(scratch.values[1728] != 0.0))) && (scratch.values[1730] != 0.0)) {
            scratch.store_ad(1356, &AdValue::exp(AdValue::div(AdValue::neg(scratch.ad_value(641)), scratch.ad_value(1381))));
        }

        scratch.values[1731] = if (((-scratch.values[641]) / scratch.values[1381]) < 0.0) { 1.0 } else { 0.0 };

        if ((((((scratch.values[1320] != 0.0) && (scratch.values[1337] != 0.0)) && (!(scratch.values[1717] != 0.0))) && (!(scratch.values[1728] != 0.0))) && (!(scratch.values[1730] != 0.0))) && (scratch.values[1731] != 0.0)) {
            let assign27750_ad_e33614: AdValue = AdValue::offset(AdValue::mul(AdValue::sub_from_scalar((-230.25850929940458), AdValue::div(AdValue::neg(scratch.ad_value(641)), scratch.ad_value(1381))), AdValue::offset(AdValue::scale(AdValue::mul(AdValue::sub_from_scalar((-230.25850929940458), AdValue::div(AdValue::neg(scratch.ad_value(641)), scratch.ad_value(1381))), AdValue::offset(AdValue::scale(AdValue::sub_from_scalar((-230.25850929940458), AdValue::div(AdValue::neg(scratch.ad_value(641)), scratch.ad_value(1381))), 0.3333333333333333), 1.0)), 0.5), 1.0)), 1.0);
            scratch.store_ad(1356, &AdValue::div_from_scalar(1e-100, assign27750_ad_e33614));
        }

        if ((((((scratch.values[1320] != 0.0) && (scratch.values[1337] != 0.0)) && (!(scratch.values[1717] != 0.0))) && (!(scratch.values[1728] != 0.0))) && (!(scratch.values[1730] != 0.0))) && (!(scratch.values[1731] != 0.0))) {
            let assign27760_ad_e33664: AdValue = AdValue::scale(AdValue::offset(AdValue::mul(AdValue::offset(AdValue::div(AdValue::neg(scratch.ad_value(641)), scratch.ad_value(1381)), (-230.25850929940458)), AdValue::offset(AdValue::scale(AdValue::mul(AdValue::offset(AdValue::div(AdValue::neg(scratch.ad_value(641)), scratch.ad_value(1381)), (-230.25850929940458)), AdValue::offset(AdValue::scale(AdValue::offset(AdValue::div(AdValue::neg(scratch.ad_value(641)), scratch.ad_value(1381)), (-230.25850929940458)), 0.3333333333333333), 1.0)), 0.5), 1.0)), 1.0), 1e100);
            scratch.store_ad(1356, &assign27760_ad_e33664);
        }

        if ((((scratch.values[1320] != 0.0) && (scratch.values[1337] != 0.0)) && (!(scratch.values[1717] != 0.0))) && (!(scratch.values[1728] != 0.0))) {
            scratch.store_ad(1380, &AdValue::mul(scratch.ad_value(570), AdValue::mul(AdValue::mul(AdValue::mul(scratch.ad_value(523), scratch.ad_value(1381)), scratch.ad_value(1381)), scratch.ad_value(1356))));
        }

        scratch.values[1732] = if (scratch.values[579] > 1000.0) { 1.0 } else { 0.0 };

        if ((((scratch.values[1320] != 0.0) && (scratch.values[1337] != 0.0)) && (!(scratch.values[1717] != 0.0))) && (scratch.values[1732] != 0.0)) {
            scratch.values[1382] = 1.0;
            scratch.node_derivatives[1382] = [0.0; Instance::NODE_COUNT];
            scratch.branch_derivatives[1382] = [0.0; Instance::BRANCH_COUNT];
        }

        scratch.values[1733] = if (scratch.values[1355] > ((-scratch.values[500]) * scratch.values[579])) { 1.0 } else { 0.0 };

        scratch.values[1734] = if (scratch.values[582] == 4.0) { 1.0 } else { 0.0 };

        if ((((((scratch.values[1320] != 0.0) && (scratch.values[1337] != 0.0)) && (!(scratch.values[1717] != 0.0))) && (!(scratch.values[1732] != 0.0))) && (scratch.values[1733] != 0.0)) && (scratch.values[1734] != 0.0)) {
            scratch.store_ad(1356, &AdValue::mul(AdValue::mul(AdValue::mul(AdValue::mul(scratch.ad_value(1355), scratch.ad_value(647)), AdValue::mul(scratch.ad_value(1355), scratch.ad_value(647))), AdValue::mul(scratch.ad_value(1355), scratch.ad_value(647))), AdValue::mul(scratch.ad_value(1355), scratch.ad_value(647))));
        }

        if ((((((scratch.values[1320] != 0.0) && (scratch.values[1337] != 0.0)) && (!(scratch.values[1717] != 0.0))) && (!(scratch.values[1732] != 0.0))) && (scratch.values[1733] != 0.0)) && (!(scratch.values[1734] != 0.0))) {
            scratch.store_ad(1356, &AdValue::pow(AdValue::abs(AdValue::mul(scratch.ad_value(1355), scratch.ad_value(647))), scratch.ad_value(582)));
        }

        if (((((scratch.values[1320] != 0.0) && (scratch.values[1337] != 0.0)) && (!(scratch.values[1717] != 0.0))) && (!(scratch.values[1732] != 0.0))) && (scratch.values[1733] != 0.0)) {
            scratch.store_ad(1382, &AdValue::div_from_scalar(1.0, AdValue::sub_from_scalar(1.0, scratch.ad_value(1356))));
        }

        if (((((scratch.values[1320] != 0.0) && (scratch.values[1337] != 0.0)) && (!(scratch.values[1717] != 0.0))) && (!(scratch.values[1732] != 0.0))) && (!(scratch.values[1733] != 0.0))) {
            scratch.store_ad(1382, &AdValue::add(scratch.ad_value(644), AdValue::mul(AdValue::add(scratch.ad_value(1355), AdValue::scale(scratch.ad_value(579), scratch.values[500])), scratch.ad_value(650))));
        }

        if (((scratch.values[1320] != 0.0) && (scratch.values[1337] != 0.0)) && (!(scratch.values[1717] != 0.0))) {
            scratch.store_ad(1384, &AdValue::mul(AdValue::add(AdValue::add(AdValue::add(scratch.ad_value(1357), scratch.ad_value(1358)), scratch.ad_value(1365)), scratch.ad_value(1380)), scratch.ad_value(1382)));
        }

        scratch.values[1735] = if (scratch.values[726] == 0.0) { 1.0 } else { 0.0 };

        if (((scratch.values[1320] != 0.0) && (scratch.values[1337] != 0.0)) && (scratch.values[1735] != 0.0)) {
            scratch.values[1385] = 0.0;
            scratch.node_derivatives[1385] = [0.0; Instance::NODE_COUNT];
            scratch.branch_derivatives[1385] = [0.0; Instance::BRANCH_COUNT];
        }

        scratch.values[1736] = if (scratch.values[609] == 0.5) { 1.0 } else { 0.0 };

        if ((((scratch.values[1320] != 0.0) && (scratch.values[1337] != 0.0)) && (!(scratch.values[1735] != 0.0))) && (scratch.values[1736] != 0.0)) {
            scratch.store_ad(1356, &AdValue::sqrt(AdValue::sub_from_scalar(1.0, AdValue::mul(scratch.ad_value(1348), scratch.ad_value(606)))));
        }

        if ((((scratch.values[1320] != 0.0) && (scratch.values[1337] != 0.0)) && (!(scratch.values[1735] != 0.0))) && (!(scratch.values[1736] != 0.0))) {
            scratch.store_ad(1356, &AdValue::pow(AdValue::sub_from_scalar(1.0, AdValue::mul(scratch.ad_value(1348), scratch.ad_value(606))), scratch.ad_value(609)));
        }

        if (((scratch.values[1320] != 0.0) && (scratch.values[1337] != 0.0)) && (!(scratch.values[1735] != 0.0))) {
            scratch.store_ad(1357, &AdValue::mul(scratch.ad_value(597), scratch.ad_value(1347)));
        }

        scratch.values[1737] = if ((scratch.values[562] == 0.0) && (scratch.values[565] == 0.0)) { 1.0 } else { 0.0 };

        if ((((scratch.values[1320] != 0.0) && (scratch.values[1337] != 0.0)) && (!(scratch.values[1735] != 0.0))) && (scratch.values[1737] != 0.0)) {
            scratch.values[1358] = 0.0;
            scratch.node_derivatives[1358] = [0.0; Instance::NODE_COUNT];
            scratch.branch_derivatives[1358] = [0.0; Instance::BRANCH_COUNT];
        }

        if ((((scratch.values[1320] != 0.0) && (scratch.values[1337] != 0.0)) && (!(scratch.values[1735] != 0.0))) && (!(scratch.values[1737] != 0.0))) {
            scratch.store_ad(1359, &AdValue::sub(scratch.ad_value(603), scratch.ad_value(1353)));
        }

        if ((((scratch.values[1320] != 0.0) && (scratch.values[1337] != 0.0)) && (!(scratch.values[1735] != 0.0))) && (!(scratch.values[1737] != 0.0))) {
            scratch.store_ad(1360, &AdValue::sub_from_scalar(1.0, AdValue::sqrt(AdValue::sub_from_scalar(1.0, AdValue::div(scratch.ad_value(1351), scratch.ad_value(1359))))));
        }

        scratch.values[1738] = if (scratch.values[551] == 0.5) { 1.0 } else { 0.0 };

        if (((((scratch.values[1320] != 0.0) && (scratch.values[1337] != 0.0)) && (!(scratch.values[1735] != 0.0))) && (!(scratch.values[1737] != 0.0))) && (scratch.values[1738] != 0.0)) {
            scratch.values[1361] = 0.0;
            scratch.node_derivatives[1361] = [0.0; Instance::NODE_COUNT];
            scratch.branch_derivatives[1361] = [0.0; Instance::BRANCH_COUNT];
        }

        if (((((scratch.values[1320] != 0.0) && (scratch.values[1337] != 0.0)) && (!(scratch.values[1735] != 0.0))) && (!(scratch.values[1737] != 0.0))) && (!(scratch.values[1738] != 0.0))) {
            scratch.store_ad(1361, &AdValue::mul(AdValue::add(AdValue::div(AdValue::mul(AdValue::square(scratch.ad_value(1360)), AdValue::ln(scratch.ad_value(1360))), AdValue::sub_from_scalar(1.0, scratch.ad_value(1360))), scratch.ad_value(1360)), AdValue::sub_from_scalar(1.0, AdValue::scale(scratch.ad_value(551), 2.0))));
        }

        if ((((scratch.values[1320] != 0.0) && (scratch.values[1337] != 0.0)) && (!(scratch.values[1735] != 0.0))) && (!(scratch.values[1737] != 0.0))) {
            scratch.store_ad(1362, &AdValue::add(scratch.ad_value(1360), scratch.ad_value(1361)));
        }

        scratch.values[1739] = if (scratch.values[551] == 0.5) { 1.0 } else { 0.0 };

        if (((((scratch.values[1320] != 0.0) && (scratch.values[1337] != 0.0)) && (!(scratch.values[1735] != 0.0))) && (!(scratch.values[1737] != 0.0))) && (scratch.values[1739] != 0.0)) {
            scratch.store_ad(1356, &AdValue::sqrt(AdValue::mul(scratch.ad_value(1359), scratch.ad_value(630))));
        }

        if (((((scratch.values[1320] != 0.0) && (scratch.values[1337] != 0.0)) && (!(scratch.values[1735] != 0.0))) && (!(scratch.values[1737] != 0.0))) && (!(scratch.values[1739] != 0.0))) {
            scratch.store_ad(1356, &AdValue::pow(AdValue::mul(scratch.ad_value(1359), scratch.ad_value(630)), scratch.ad_value(551)));
        }

        if ((((scratch.values[1320] != 0.0) && (scratch.values[1337] != 0.0)) && (!(scratch.values[1735] != 0.0))) && (!(scratch.values[1737] != 0.0))) {
            scratch.store_ad(1363, &AdValue::mul(scratch.ad_value(624), scratch.ad_value(1356)));
        }

        if ((((scratch.values[1320] != 0.0) && (scratch.values[1337] != 0.0)) && (!(scratch.values[1735] != 0.0))) && (!(scratch.values[1737] != 0.0))) {
            scratch.store_ad(1364, &AdValue::mul(scratch.ad_value(594), AdValue::mul(AdValue::offset(scratch.ad_value(1350), (-1.0)), scratch.ad_value(1363))));
        }

        if ((((scratch.values[1320] != 0.0) && (scratch.values[1337] != 0.0)) && (!(scratch.values[1735] != 0.0))) && (!(scratch.values[1737] != 0.0))) {
            scratch.store_ad(1358, &AdValue::mul(scratch.ad_value(562), AdValue::mul(scratch.ad_value(1364), scratch.ad_value(1362))));
        }

        scratch.values[1740] = if (scratch.values[565] == 0.0) { 1.0 } else { 0.0 };

        if ((((scratch.values[1320] != 0.0) && (scratch.values[1337] != 0.0)) && (!(scratch.values[1735] != 0.0))) && (scratch.values[1740] != 0.0)) {
            scratch.values[1365] = 0.0;
            scratch.node_derivatives[1365] = [0.0; Instance::NODE_COUNT];
            scratch.branch_derivatives[1365] = [0.0; Instance::BRANCH_COUNT];
        }

        if ((((scratch.values[1320] != 0.0) && (scratch.values[1337] != 0.0)) && (!(scratch.values[1735] != 0.0))) && (!(scratch.values[1740] != 0.0))) {
            scratch.store_ad(1366, &AdValue::mul(scratch.ad_value(639), AdValue::div(AdValue::mul(scratch.ad_value(1363), scratch.ad_value(609)), scratch.ad_value(1359))));
        }

        if ((((scratch.values[1320] != 0.0) && (scratch.values[1337] != 0.0)) && (!(scratch.values[1735] != 0.0))) && (!(scratch.values[1740] != 0.0))) {
            scratch.store_ad(1367, &AdValue::div(AdValue::scale(scratch.ad_value(636), 0.666666666666667), scratch.ad_value(1366)));
        }

        if ((((scratch.values[1320] != 0.0) && (scratch.values[1337] != 0.0)) && (!(scratch.values[1735] != 0.0))) && (!(scratch.values[1740] != 0.0))) {
            scratch.store_ad(1368, &AdValue::square(scratch.ad_value(1367)));
        }

        if ((((scratch.values[1320] != 0.0) && (scratch.values[1337] != 0.0)) && (!(scratch.values[1735] != 0.0))) && (!(scratch.values[1740] != 0.0))) {
            scratch.store_ad(1369, &AdValue::sqrt(AdValue::div(AdValue::square(scratch.ad_value(1368)), AdValue::offset(AdValue::square(scratch.ad_value(1368)), 1.0))));
        }

        if ((((scratch.values[1320] != 0.0) && (scratch.values[1337] != 0.0)) && (!(scratch.values[1735] != 0.0))) && (!(scratch.values[1740] != 0.0))) {
            scratch.store_ad(1370, &AdValue::sqrt(AdValue::abs(scratch.ad_value(1369))));
        }

        if ((((scratch.values[1320] != 0.0) && (scratch.values[1337] != 0.0)) && (!(scratch.values[1735] != 0.0))) && (!(scratch.values[1740] != 0.0))) {
            scratch.store_ad(1371, &AdValue::mul(scratch.ad_value(1369), scratch.ad_value(1370)));
        }

        scratch.values[1741] = if (((-scratch.values[551]) * scratch.values[612]) == (-1.0)) { 1.0 } else { 0.0 };

        if (((((scratch.values[1320] != 0.0) && (scratch.values[1337] != 0.0)) && (!(scratch.values[1735] != 0.0))) && (!(scratch.values[1740] != 0.0))) && (scratch.values[1741] != 0.0)) {
            scratch.store_ad(1372, &AdValue::div_from_scalar(1.0, AdValue::offset(AdValue::mul(scratch.ad_value(1366), scratch.ad_value(1371)), 1.0)));
        }

        if (((((scratch.values[1320] != 0.0) && (scratch.values[1337] != 0.0)) && (!(scratch.values[1735] != 0.0))) && (!(scratch.values[1740] != 0.0))) && (!(scratch.values[1741] != 0.0))) {
            scratch.store_ad(1372, &AdValue::pow(AdValue::offset(AdValue::mul(scratch.ad_value(1366), scratch.ad_value(1371)), 1.0), AdValue::mul(AdValue::neg(scratch.ad_value(551)), scratch.ad_value(612))));
        }

        if ((((scratch.values[1320] != 0.0) && (scratch.values[1337] != 0.0)) && (!(scratch.values[1735] != 0.0))) && (!(scratch.values[1740] != 0.0))) {
            scratch.store_ad(1373, &AdValue::div(AdValue::mul(scratch.ad_value(1362), scratch.ad_value(1372)), AdValue::add(scratch.ad_value(1362), scratch.ad_value(1372))));
        }

        if ((((scratch.values[1320] != 0.0) && (scratch.values[1337] != 0.0)) && (!(scratch.values[1735] != 0.0))) && (!(scratch.values[1740] != 0.0))) {
            scratch.store_ad(1374, &AdValue::sqrt(AdValue::scale(AdValue::div(scratch.ad_value(1366), scratch.ad_value(1370)), 0.375)));
        }

        if ((((scratch.values[1320] != 0.0) && (scratch.values[1337] != 0.0)) && (!(scratch.values[1735] != 0.0))) && (!(scratch.values[1740] != 0.0))) {
            scratch.store_ad(1375, &AdValue::sub(AdValue::scale(AdValue::mul(scratch.ad_value(1367), scratch.ad_value(1370)), 2.0), scratch.ad_value(1369)));
        }

        if ((((scratch.values[1320] != 0.0) && (scratch.values[1337] != 0.0)) && (!(scratch.values[1735] != 0.0))) && (!(scratch.values[1740] != 0.0))) {
            scratch.store_ad(1376, &AdValue::add(AdValue::sub(AdValue::mul(AdValue::mul(scratch.ad_value(636), scratch.ad_value(1367)), scratch.ad_value(1370)), AdValue::mul(scratch.ad_value(636), scratch.ad_value(1369))), AdValue::scale(AdValue::mul(scratch.ad_value(1366), scratch.ad_value(1371)), 0.5)));
        }

        if ((((scratch.values[1320] != 0.0) && (scratch.values[1337] != 0.0)) && (!(scratch.values[1735] != 0.0))) && (!(scratch.values[1740] != 0.0))) {
            scratch.store_ad(1377, &AdValue::mul(AdValue::offset(scratch.ad_value(1375), (-1.0)), scratch.ad_value(1374)));
        }

        if ((((scratch.values[1320] != 0.0) && (scratch.values[1337] != 0.0)) && (!(scratch.values[1735] != 0.0))) && (!(scratch.values[1740] != 0.0))) {
            scratch.store_ad(1338, &AdValue::square(scratch.ad_value(1377)));
        }

        scratch.values[1742] = if (scratch.values[1377] > 0.0) { 1.0 } else { 0.0 };

        if (((((scratch.values[1320] != 0.0) && (scratch.values[1337] != 0.0)) && (!(scratch.values[1735] != 0.0))) && (!(scratch.values[1740] != 0.0))) && (scratch.values[1742] != 0.0)) {
            scratch.store_ad(1339, &AdValue::div_from_scalar(1.0, AdValue::offset(AdValue::scale(scratch.ad_value(1377), scratch.values[428]), 1.0)));
        }

        if (((((scratch.values[1320] != 0.0) && (scratch.values[1337] != 0.0)) && (!(scratch.values[1735] != 0.0))) && (!(scratch.values[1740] != 0.0))) && (!(scratch.values[1742] != 0.0))) {
            scratch.store_ad(1339, &AdValue::div_from_scalar(1.0, AdValue::sub_from_scalar(1.0, AdValue::scale(scratch.ad_value(1377), scratch.values[428]))));
        }

        scratch.values[1743] = if (((-scratch.values[1338]) + scratch.values[1376]) > (-230.25850929940458)) { 1.0 } else { 0.0 };

        if (((((scratch.values[1320] != 0.0) && (scratch.values[1337] != 0.0)) && (!(scratch.values[1735] != 0.0))) && (!(scratch.values[1740] != 0.0))) && (scratch.values[1743] != 0.0)) {
            scratch.store_ad(1356, &AdValue::exp(AdValue::sub(scratch.ad_value(1376), scratch.ad_value(1338))));
        }

        if (((((scratch.values[1320] != 0.0) && (scratch.values[1337] != 0.0)) && (!(scratch.values[1735] != 0.0))) && (!(scratch.values[1740] != 0.0))) && (!(scratch.values[1743] != 0.0))) {
            let assign28310_ad_e34493: AdValue = AdValue::div_from_scalar(1e-100, AdValue::offset(AdValue::mul(AdValue::sub_from_scalar((-230.25850929940458), AdValue::sub(scratch.ad_value(1376), scratch.ad_value(1338))), AdValue::offset(AdValue::scale(AdValue::mul(AdValue::sub_from_scalar((-230.25850929940458), AdValue::sub(scratch.ad_value(1376), scratch.ad_value(1338))), AdValue::offset(AdValue::scale(AdValue::sub_from_scalar((-230.25850929940458), AdValue::sub(scratch.ad_value(1376), scratch.ad_value(1338))), 0.3333333333333333), 1.0)), 0.5), 1.0)), 1.0));
            scratch.store_ad(1356, &assign28310_ad_e34493);
        }

        if ((((scratch.values[1320] != 0.0) && (scratch.values[1337] != 0.0)) && (!(scratch.values[1735] != 0.0))) && (!(scratch.values[1740] != 0.0))) {
            scratch.store_ad(1340, &AdValue::mul(AdValue::add(AdValue::add(AdValue::scale(scratch.ad_value(1339), 0.29214664), AdValue::scale(AdValue::square(scratch.ad_value(1339)), scratch.values[429])), AdValue::scale(AdValue::mul(AdValue::square(scratch.ad_value(1339)), scratch.ad_value(1339)), scratch.values[430])), scratch.ad_value(1356)));
        }

        scratch.values[1744] = if (scratch.values[1377] > 0.0) { 1.0 } else { 0.0 };

        if (((((scratch.values[1320] != 0.0) && (scratch.values[1337] != 0.0)) && (!(scratch.values[1735] != 0.0))) && (!(scratch.values[1740] != 0.0))) && (scratch.values[1744] != 0.0)) {
            scratch.values[1378] = scratch.values[1340];
            scratch.node_derivatives[1378] = scratch.node_derivatives[1340];
            scratch.branch_derivatives[1378] = scratch.branch_derivatives[1340];
        }

        scratch.values[1745] = if (scratch.values[1376] > (-230.25850929940458)) { 1.0 } else { 0.0 };

        if ((((((scratch.values[1320] != 0.0) && (scratch.values[1337] != 0.0)) && (!(scratch.values[1735] != 0.0))) && (!(scratch.values[1740] != 0.0))) && (!(scratch.values[1744] != 0.0))) && (scratch.values[1745] != 0.0)) {
            scratch.store_ad(1356, &AdValue::exp(scratch.ad_value(1376)));
        }

        if ((((((scratch.values[1320] != 0.0) && (scratch.values[1337] != 0.0)) && (!(scratch.values[1735] != 0.0))) && (!(scratch.values[1740] != 0.0))) && (!(scratch.values[1744] != 0.0))) && (!(scratch.values[1745] != 0.0))) {
            scratch.store_ad(1356, &AdValue::div_from_scalar(1e-100, AdValue::offset(AdValue::mul(AdValue::sub_from_scalar((-230.25850929940458), scratch.ad_value(1376)), AdValue::offset(AdValue::scale(AdValue::mul(AdValue::sub_from_scalar((-230.25850929940458), scratch.ad_value(1376)), AdValue::offset(AdValue::scale(AdValue::sub_from_scalar((-230.25850929940458), scratch.ad_value(1376)), 0.3333333333333333), 1.0)), 0.5), 1.0)), 1.0)));
        }

        if (((((scratch.values[1320] != 0.0) && (scratch.values[1337] != 0.0)) && (!(scratch.values[1735] != 0.0))) && (!(scratch.values[1740] != 0.0))) && (!(scratch.values[1744] != 0.0))) {
            scratch.store_ad(1378, &AdValue::sub(AdValue::scale(scratch.ad_value(1356), 2.0), scratch.ad_value(1340)));
        }

        if ((((scratch.values[1320] != 0.0) && (scratch.values[1337] != 0.0)) && (!(scratch.values[1735] != 0.0))) && (!(scratch.values[1740] != 0.0))) {
            scratch.store_ad(1379, &AdValue::scale(AdValue::div(AdValue::mul(scratch.ad_value(636), scratch.ad_value(1378)), scratch.ad_value(1374)), (1.772453850905516 * 0.5)));
        }

        if ((((scratch.values[1320] != 0.0) && (scratch.values[1337] != 0.0)) && (!(scratch.values[1735] != 0.0))) && (!(scratch.values[1740] != 0.0))) {
            scratch.store_ad(1365, &AdValue::mul(scratch.ad_value(565), AdValue::mul(AdValue::mul(scratch.ad_value(1364), scratch.ad_value(1379)), scratch.ad_value(1373))));
        }

        scratch.values[1746] = if (scratch.values[571] == 0.0) { 1.0 } else { 0.0 };

        if ((((scratch.values[1320] != 0.0) && (scratch.values[1337] != 0.0)) && (!(scratch.values[1735] != 0.0))) && (scratch.values[1746] != 0.0)) {
            scratch.values[1380] = 0.0;
            scratch.node_derivatives[1380] = [0.0; Instance::NODE_COUNT];
            scratch.branch_derivatives[1380] = [0.0; Instance::BRANCH_COUNT];
        }

        scratch.values[1747] = if (scratch.values[551] == 0.5) { 1.0 } else { 0.0 };

        if (((((scratch.values[1320] != 0.0) && (scratch.values[1337] != 0.0)) && (!(scratch.values[1735] != 0.0))) && (!(scratch.values[1746] != 0.0))) && (scratch.values[1747] != 0.0)) {
            scratch.store_ad(1356, &AdValue::sqrt(AdValue::mul(AdValue::sub(scratch.ad_value(548), scratch.ad_value(1354)), scratch.ad_value(630))));
        }

        if (((((scratch.values[1320] != 0.0) && (scratch.values[1337] != 0.0)) && (!(scratch.values[1735] != 0.0))) && (!(scratch.values[1746] != 0.0))) && (!(scratch.values[1747] != 0.0))) {
            scratch.store_ad(1356, &AdValue::pow(AdValue::mul(AdValue::sub(scratch.ad_value(548), scratch.ad_value(1354)), scratch.ad_value(630)), scratch.ad_value(551)));
        }

        if ((((scratch.values[1320] != 0.0) && (scratch.values[1337] != 0.0)) && (!(scratch.values[1735] != 0.0))) && (!(scratch.values[1746] != 0.0))) {
            scratch.store_ad(1381, &AdValue::mul(scratch.ad_value(612), AdValue::div(AdValue::mul(AdValue::sub(scratch.ad_value(548), scratch.ad_value(1354)), scratch.ad_value(627)), scratch.ad_value(1356))));
        }

        scratch.values[1748] = if (((((-scratch.values[642]) / scratch.values[1381])) as f64).abs() < 230.25850929940458) { 1.0 } else { 0.0 };

        if (((((scratch.values[1320] != 0.0) && (scratch.values[1337] != 0.0)) && (!(scratch.values[1735] != 0.0))) && (!(scratch.values[1746] != 0.0))) && (scratch.values[1748] != 0.0)) {
            scratch.store_ad(1356, &AdValue::exp(AdValue::div(AdValue::neg(scratch.ad_value(642)), scratch.ad_value(1381))));
        }

        scratch.values[1749] = if (((-scratch.values[642]) / scratch.values[1381]) < 0.0) { 1.0 } else { 0.0 };

        if ((((((scratch.values[1320] != 0.0) && (scratch.values[1337] != 0.0)) && (!(scratch.values[1735] != 0.0))) && (!(scratch.values[1746] != 0.0))) && (!(scratch.values[1748] != 0.0))) && (scratch.values[1749] != 0.0)) {
            let assign28500_ad_e34820: AdValue = AdValue::offset(AdValue::mul(AdValue::sub_from_scalar((-230.25850929940458), AdValue::div(AdValue::neg(scratch.ad_value(642)), scratch.ad_value(1381))), AdValue::offset(AdValue::scale(AdValue::mul(AdValue::sub_from_scalar((-230.25850929940458), AdValue::div(AdValue::neg(scratch.ad_value(642)), scratch.ad_value(1381))), AdValue::offset(AdValue::scale(AdValue::sub_from_scalar((-230.25850929940458), AdValue::div(AdValue::neg(scratch.ad_value(642)), scratch.ad_value(1381))), 0.3333333333333333), 1.0)), 0.5), 1.0)), 1.0);
            scratch.store_ad(1356, &AdValue::div_from_scalar(1e-100, assign28500_ad_e34820));
        }

        if ((((((scratch.values[1320] != 0.0) && (scratch.values[1337] != 0.0)) && (!(scratch.values[1735] != 0.0))) && (!(scratch.values[1746] != 0.0))) && (!(scratch.values[1748] != 0.0))) && (!(scratch.values[1749] != 0.0))) {
            let assign28510_ad_e34870: AdValue = AdValue::scale(AdValue::offset(AdValue::mul(AdValue::offset(AdValue::div(AdValue::neg(scratch.ad_value(642)), scratch.ad_value(1381)), (-230.25850929940458)), AdValue::offset(AdValue::scale(AdValue::mul(AdValue::offset(AdValue::div(AdValue::neg(scratch.ad_value(642)), scratch.ad_value(1381)), (-230.25850929940458)), AdValue::offset(AdValue::scale(AdValue::offset(AdValue::div(AdValue::neg(scratch.ad_value(642)), scratch.ad_value(1381)), (-230.25850929940458)), 0.3333333333333333), 1.0)), 0.5), 1.0)), 1.0), 1e100);
            scratch.store_ad(1356, &assign28510_ad_e34870);
        }

        if ((((scratch.values[1320] != 0.0) && (scratch.values[1337] != 0.0)) && (!(scratch.values[1735] != 0.0))) && (!(scratch.values[1746] != 0.0))) {
            scratch.store_ad(1380, &AdValue::mul(scratch.ad_value(571), AdValue::mul(AdValue::mul(AdValue::mul(scratch.ad_value(523), scratch.ad_value(1381)), scratch.ad_value(1381)), scratch.ad_value(1356))));
        }

        scratch.values[1750] = if (scratch.values[580] > 1000.0) { 1.0 } else { 0.0 };

        if ((((scratch.values[1320] != 0.0) && (scratch.values[1337] != 0.0)) && (!(scratch.values[1735] != 0.0))) && (scratch.values[1750] != 0.0)) {
            scratch.values[1382] = 1.0;
            scratch.node_derivatives[1382] = [0.0; Instance::NODE_COUNT];
            scratch.branch_derivatives[1382] = [0.0; Instance::BRANCH_COUNT];
        }

        scratch.values[1751] = if (scratch.values[1355] > ((-scratch.values[500]) * scratch.values[580])) { 1.0 } else { 0.0 };

        scratch.values[1752] = if (scratch.values[583] == 4.0) { 1.0 } else { 0.0 };

        if ((((((scratch.values[1320] != 0.0) && (scratch.values[1337] != 0.0)) && (!(scratch.values[1735] != 0.0))) && (!(scratch.values[1750] != 0.0))) && (scratch.values[1751] != 0.0)) && (scratch.values[1752] != 0.0)) {
            scratch.store_ad(1356, &AdValue::mul(AdValue::mul(AdValue::mul(AdValue::mul(scratch.ad_value(1355), scratch.ad_value(648)), AdValue::mul(scratch.ad_value(1355), scratch.ad_value(648))), AdValue::mul(scratch.ad_value(1355), scratch.ad_value(648))), AdValue::mul(scratch.ad_value(1355), scratch.ad_value(648))));
        }

        if ((((((scratch.values[1320] != 0.0) && (scratch.values[1337] != 0.0)) && (!(scratch.values[1735] != 0.0))) && (!(scratch.values[1750] != 0.0))) && (scratch.values[1751] != 0.0)) && (!(scratch.values[1752] != 0.0))) {
            scratch.store_ad(1356, &AdValue::pow(AdValue::abs(AdValue::mul(scratch.ad_value(1355), scratch.ad_value(648))), scratch.ad_value(583)));
        }

        if (((((scratch.values[1320] != 0.0) && (scratch.values[1337] != 0.0)) && (!(scratch.values[1735] != 0.0))) && (!(scratch.values[1750] != 0.0))) && (scratch.values[1751] != 0.0)) {
            scratch.store_ad(1382, &AdValue::div_from_scalar(1.0, AdValue::sub_from_scalar(1.0, scratch.ad_value(1356))));
        }

        if (((((scratch.values[1320] != 0.0) && (scratch.values[1337] != 0.0)) && (!(scratch.values[1735] != 0.0))) && (!(scratch.values[1750] != 0.0))) && (!(scratch.values[1751] != 0.0))) {
            scratch.store_ad(1382, &AdValue::add(scratch.ad_value(645), AdValue::mul(AdValue::add(scratch.ad_value(1355), AdValue::scale(scratch.ad_value(580), scratch.values[500])), scratch.ad_value(651))));
        }

        if (((scratch.values[1320] != 0.0) && (scratch.values[1337] != 0.0)) && (!(scratch.values[1735] != 0.0))) {
            scratch.store_ad(1385, &AdValue::mul(AdValue::add(AdValue::add(AdValue::add(scratch.ad_value(1357), scratch.ad_value(1358)), scratch.ad_value(1365)), scratch.ad_value(1380)), scratch.ad_value(1382)));
        }

        if ((scratch.values[1320] != 0.0) && (scratch.values[1337] != 0.0)) {
            scratch.store_ad(513, &AdValue::add(AdValue::add(AdValue::mul(scratch.ad_value(724), scratch.ad_value(1383)), AdValue::mul(scratch.ad_value(725), scratch.ad_value(1384))), AdValue::mul(scratch.ad_value(726), scratch.ad_value(1385))));
        }

        if ((scratch.values[1320] != 0.0) && (scratch.values[1337] != 0.0)) {
            scratch.values[1354] = 0.0;
            scratch.node_derivatives[1354] = [0.0; Instance::NODE_COUNT];
            scratch.branch_derivatives[1354] = [0.0; Instance::BRANCH_COUNT];
        }

    }

    pub(super) fn stamp_transient_block_22(
        &mut self,
        ctx: &GeneratedEvalContext<'_>,
        stamper: &mut GeneratedStamper<'_>,
        scratch: &mut Scratch,
    ) {
        let _ = stamper;
        if ((scratch.values[1320] != 0.0) && (scratch.values[1337] != 0.0)) {
            scratch.values[1351] = 0.0;
            scratch.node_derivatives[1351] = [0.0; Instance::NODE_COUNT];
            scratch.branch_derivatives[1351] = [0.0; Instance::BRANCH_COUNT];
        }

        scratch.values[1753] = if !(((scratch.values[724] == 0.0) && (scratch.values[725] == 0.0)) && (scratch.values[726] == 0.0)) { 1.0 } else { 0.0 };

        if (((scratch.values[1320] != 0.0) && (scratch.values[1337] != 0.0)) && (scratch.values[1753] != 0.0)) {
            scratch.store_ad(1341, &AdValue::mul(AdValue::scale(scratch.ad_value(735), 4.0), scratch.ad_value(735)));
        }

        if (((scratch.values[1320] != 0.0) && (scratch.values[1337] != 0.0)) && (scratch.values[1753] != 0.0)) {
            scratch.store_ad(1342, &AdValue::div(scratch.ad_value(735), scratch.ad_value(736)));
        }

        if (((scratch.values[1320] != 0.0) && (scratch.values[1337] != 0.0)) && (scratch.values[1753] != 0.0)) {
            scratch.store_ad(1343, &AdValue::add(scratch.ad_value(524), AdValue::mul(scratch.ad_value(735), scratch.ad_value(1342))));
        }

        if (((scratch.values[1320] != 0.0) && (scratch.values[1337] != 0.0)) && (scratch.values[1753] != 0.0)) {
            scratch.store_ad(1344, &AdValue::add(scratch.ad_value(736), scratch.ad_value(1343)));
        }

        if (((scratch.values[1320] != 0.0) && (scratch.values[1337] != 0.0)) && (scratch.values[1753] != 0.0)) {
            scratch.store_ad(1345, &AdValue::sub(scratch.ad_value(736), scratch.ad_value(1343)));
        }

        if (((scratch.values[1320] != 0.0) && (scratch.values[1337] != 0.0)) && (scratch.values[1753] != 0.0)) {
            scratch.store_ad(1346, &AdValue::sqrt(AdValue::add(AdValue::square(scratch.ad_value(1345)), scratch.ad_value(1341))));
        }

        if (((scratch.values[1320] != 0.0) && (scratch.values[1337] != 0.0)) && (scratch.values[1753] != 0.0)) {
            scratch.store_ad(1348, &AdValue::scale(AdValue::div(AdValue::mul(scratch.ad_value(524), scratch.ad_value(736)), AdValue::add(scratch.ad_value(1344), scratch.ad_value(1346))), 2.0));
        }

        scratch.values[1754] = if (scratch.values[524] < scratch.values[732]) { 1.0 } else { 0.0 };

        scratch.values[1755] = if ((((0.5 * (scratch.values[524] * scratch.values[427]))) as f64).abs() < 230.25850929940458) { 1.0 } else { 0.0 };

        if (((((scratch.values[1320] != 0.0) && (scratch.values[1337] != 0.0)) && (scratch.values[1753] != 0.0)) && (scratch.values[1754] != 0.0)) && (scratch.values[1755] != 0.0)) {
            scratch.store_ad(1350, &AdValue::exp(AdValue::scale(scratch.ad_value(524), (scratch.values[427] * 0.5))));
        }

        scratch.values[1756] = if ((0.5 * (scratch.values[524] * scratch.values[427])) < 0.0) { 1.0 } else { 0.0 };

        if ((((((scratch.values[1320] != 0.0) && (scratch.values[1337] != 0.0)) && (scratch.values[1753] != 0.0)) && (scratch.values[1754] != 0.0)) && (!(scratch.values[1755] != 0.0))) && (scratch.values[1756] != 0.0)) {
            let assign28770_ad_e35233: AdValue = AdValue::div_from_scalar(1e-100, AdValue::offset(AdValue::mul(AdValue::sub_from_scalar((-230.25850929940458), AdValue::scale(scratch.ad_value(524), (scratch.values[427] * 0.5))), AdValue::offset(AdValue::scale(AdValue::mul(AdValue::sub_from_scalar((-230.25850929940458), AdValue::scale(scratch.ad_value(524), (scratch.values[427] * 0.5))), AdValue::offset(AdValue::scale(AdValue::sub_from_scalar((-230.25850929940458), AdValue::scale(scratch.ad_value(524), (scratch.values[427] * 0.5))), 0.3333333333333333), 1.0)), 0.5), 1.0)), 1.0));
            scratch.store_ad(1350, &assign28770_ad_e35233);
        }

        if ((((((scratch.values[1320] != 0.0) && (scratch.values[1337] != 0.0)) && (scratch.values[1753] != 0.0)) && (scratch.values[1754] != 0.0)) && (!(scratch.values[1755] != 0.0))) && (!(scratch.values[1756] != 0.0))) {
            scratch.store_ad(1350, &AdValue::scale(AdValue::offset(AdValue::mul(AdValue::offset(AdValue::scale(scratch.ad_value(524), (scratch.values[427] * 0.5)), (-230.25850929940458)), AdValue::offset(AdValue::scale(AdValue::mul(AdValue::offset(AdValue::scale(scratch.ad_value(524), (scratch.values[427] * 0.5)), (-230.25850929940458)), AdValue::offset(AdValue::scale(AdValue::offset(AdValue::scale(scratch.ad_value(524), (scratch.values[427] * 0.5)), (-230.25850929940458)), 0.3333333333333333), 1.0)), 0.5), 1.0)), 1.0), 1e100));
        }

        if ((((scratch.values[1320] != 0.0) && (scratch.values[1337] != 0.0)) && (scratch.values[1753] != 0.0)) && (scratch.values[1754] != 0.0)) {
            scratch.store_ad(1347, &AdValue::square(scratch.ad_value(1350)));
        }

        if ((((scratch.values[1320] != 0.0) && (scratch.values[1337] != 0.0)) && (scratch.values[1753] != 0.0)) && (!(scratch.values[1754] != 0.0))) {
            scratch.store_ad(1347, &AdValue::mul(AdValue::offset(AdValue::scale(AdValue::sub(scratch.ad_value(524), scratch.ad_value(732)), scratch.values[427]), 1.0), scratch.ad_value(733)));
        }

        if ((((scratch.values[1320] != 0.0) && (scratch.values[1337] != 0.0)) && (scratch.values[1753] != 0.0)) && (!(scratch.values[1754] != 0.0))) {
            scratch.store_ad(1350, &AdValue::sqrt(scratch.ad_value(1347)));
        }

        if (((scratch.values[1320] != 0.0) && (scratch.values[1337] != 0.0)) && (scratch.values[1753] != 0.0)) {
            scratch.store_ad(1347, &AdValue::offset(scratch.ad_value(1347), (-1.0)));
        }

        if (((scratch.values[1320] != 0.0) && (scratch.values[1337] != 0.0)) && (scratch.values[1753] != 0.0)) {
            scratch.store_ad(1349, &AdValue::div_from_scalar(1.0, scratch.ad_value(1350)));
        }

        scratch.values[1757] = if (scratch.values[524] > 0.0) { 1.0 } else { 0.0 };

        if ((((scratch.values[1320] != 0.0) && (scratch.values[1337] != 0.0)) && (scratch.values[1753] != 0.0)) && (scratch.values[1757] != 0.0)) {
            scratch.store_ad(1351, &AdValue::scale(AdValue::ln(AdValue::add(AdValue::offset(scratch.ad_value(1349), 2.0), AdValue::sqrt(AdValue::mul(AdValue::offset(scratch.ad_value(1349), 1.0), AdValue::offset(scratch.ad_value(1349), 3.0))))), (scratch.values[426] * 2.0)));
        }

        if ((((scratch.values[1320] != 0.0) && (scratch.values[1337] != 0.0)) && (scratch.values[1753] != 0.0)) && (!(scratch.values[1757] != 0.0))) {
            scratch.store_ad(1351, &AdValue::sub(AdValue::scale(AdValue::ln(AdValue::add(AdValue::offset(AdValue::scale(scratch.ad_value(1350), 2.0), 1.0), AdValue::sqrt(AdValue::mul(AdValue::offset(scratch.ad_value(1350), 1.0), AdValue::offset(AdValue::scale(scratch.ad_value(1350), 3.0), 1.0))))), (scratch.values[426] * 2.0)), scratch.ad_value(524)));
        }

        if (((scratch.values[1320] != 0.0) && (scratch.values[1337] != 0.0)) && (scratch.values[1753] != 0.0)) {
            scratch.store_ad(1352, &AdValue::sub(scratch.ad_value(734), scratch.ad_value(1351)));
        }

        if (((scratch.values[1320] != 0.0) && (scratch.values[1337] != 0.0)) && (scratch.values[1753] != 0.0)) {
            scratch.store_ad(1353, &AdValue::scale(AdValue::sub(AdValue::add(scratch.ad_value(524), scratch.ad_value(1352)), AdValue::sqrt(AdValue::offset(AdValue::mul(AdValue::sub(scratch.ad_value(524), scratch.ad_value(1352)), AdValue::sub(scratch.ad_value(524), scratch.ad_value(1352))), ((4.0 * scratch.values[426]) * scratch.values[426])))), 0.5));
        }

        if (((scratch.values[1320] != 0.0) && (scratch.values[1337] != 0.0)) && (scratch.values[1753] != 0.0)) {
            scratch.store_ad(1354, &AdValue::scale(AdValue::sub(AdValue::add(scratch.ad_value(524), scratch.ad_value(737)), AdValue::sqrt(AdValue::offset(AdValue::mul(AdValue::sub(scratch.ad_value(524), scratch.ad_value(737)), AdValue::sub(scratch.ad_value(524), scratch.ad_value(737))), ((4.0 * scratch.values[424]) * scratch.values[424])))), 0.5));
        }

        if (((scratch.values[1320] != 0.0) && (scratch.values[1337] != 0.0)) && (scratch.values[1753] != 0.0)) {
            scratch.store_ad(1355, &AdValue::scale(AdValue::sub(scratch.ad_value(524), AdValue::sqrt(AdValue::offset(AdValue::mul(scratch.ad_value(524), scratch.ad_value(524)), ((4.0 * 1e-6) * 1e-6)))), 0.5));
        }

        scratch.values[1758] = if (scratch.values[724] == 0.0) { 1.0 } else { 0.0 };

        if (((scratch.values[1320] != 0.0) && (scratch.values[1337] != 0.0)) && (scratch.values[1758] != 0.0)) {
            scratch.values[1383] = 0.0;
            scratch.node_derivatives[1383] = [0.0; Instance::NODE_COUNT];
            scratch.branch_derivatives[1383] = [0.0; Instance::BRANCH_COUNT];
        }

        scratch.values[1759] = if (scratch.values[607] == 0.5) { 1.0 } else { 0.0 };

        if ((((scratch.values[1320] != 0.0) && (scratch.values[1337] != 0.0)) && (!(scratch.values[1758] != 0.0))) && (scratch.values[1759] != 0.0)) {
            scratch.store_ad(1356, &AdValue::sqrt(AdValue::sub_from_scalar(1.0, AdValue::mul(scratch.ad_value(1348), scratch.ad_value(604)))));
        }

        if ((((scratch.values[1320] != 0.0) && (scratch.values[1337] != 0.0)) && (!(scratch.values[1758] != 0.0))) && (!(scratch.values[1759] != 0.0))) {
            scratch.store_ad(1356, &AdValue::pow(AdValue::sub_from_scalar(1.0, AdValue::mul(scratch.ad_value(1348), scratch.ad_value(604))), scratch.ad_value(607)));
        }

        if (((scratch.values[1320] != 0.0) && (scratch.values[1337] != 0.0)) && (!(scratch.values[1758] != 0.0))) {
            scratch.store_ad(1357, &AdValue::mul(scratch.ad_value(595), scratch.ad_value(1347)));
        }

        scratch.values[1760] = if ((scratch.values[560] == 0.0) && (scratch.values[563] == 0.0)) { 1.0 } else { 0.0 };

        if ((((scratch.values[1320] != 0.0) && (scratch.values[1337] != 0.0)) && (!(scratch.values[1758] != 0.0))) && (scratch.values[1760] != 0.0)) {
            scratch.values[1358] = 0.0;
            scratch.node_derivatives[1358] = [0.0; Instance::NODE_COUNT];
            scratch.branch_derivatives[1358] = [0.0; Instance::BRANCH_COUNT];
        }

        if ((((scratch.values[1320] != 0.0) && (scratch.values[1337] != 0.0)) && (!(scratch.values[1758] != 0.0))) && (!(scratch.values[1760] != 0.0))) {
            scratch.store_ad(1359, &AdValue::sub(scratch.ad_value(601), scratch.ad_value(1353)));
        }

        if ((((scratch.values[1320] != 0.0) && (scratch.values[1337] != 0.0)) && (!(scratch.values[1758] != 0.0))) && (!(scratch.values[1760] != 0.0))) {
            scratch.store_ad(1360, &AdValue::sub_from_scalar(1.0, AdValue::sqrt(AdValue::sub_from_scalar(1.0, AdValue::div(scratch.ad_value(1351), scratch.ad_value(1359))))));
        }

        scratch.values[1761] = if (scratch.values[549] == 0.5) { 1.0 } else { 0.0 };

        if (((((scratch.values[1320] != 0.0) && (scratch.values[1337] != 0.0)) && (!(scratch.values[1758] != 0.0))) && (!(scratch.values[1760] != 0.0))) && (scratch.values[1761] != 0.0)) {
            scratch.values[1361] = 0.0;
            scratch.node_derivatives[1361] = [0.0; Instance::NODE_COUNT];
            scratch.branch_derivatives[1361] = [0.0; Instance::BRANCH_COUNT];
        }

        if (((((scratch.values[1320] != 0.0) && (scratch.values[1337] != 0.0)) && (!(scratch.values[1758] != 0.0))) && (!(scratch.values[1760] != 0.0))) && (!(scratch.values[1761] != 0.0))) {
            scratch.store_ad(1361, &AdValue::mul(AdValue::add(AdValue::div(AdValue::mul(AdValue::square(scratch.ad_value(1360)), AdValue::ln(scratch.ad_value(1360))), AdValue::sub_from_scalar(1.0, scratch.ad_value(1360))), scratch.ad_value(1360)), AdValue::sub_from_scalar(1.0, AdValue::scale(scratch.ad_value(549), 2.0))));
        }

        if ((((scratch.values[1320] != 0.0) && (scratch.values[1337] != 0.0)) && (!(scratch.values[1758] != 0.0))) && (!(scratch.values[1760] != 0.0))) {
            scratch.store_ad(1362, &AdValue::add(scratch.ad_value(1360), scratch.ad_value(1361)));
        }

        scratch.values[1762] = if (scratch.values[549] == 0.5) { 1.0 } else { 0.0 };

        if (((((scratch.values[1320] != 0.0) && (scratch.values[1337] != 0.0)) && (!(scratch.values[1758] != 0.0))) && (!(scratch.values[1760] != 0.0))) && (scratch.values[1762] != 0.0)) {
            scratch.store_ad(1356, &AdValue::sqrt(AdValue::mul(scratch.ad_value(1359), scratch.ad_value(628))));
        }

        if (((((scratch.values[1320] != 0.0) && (scratch.values[1337] != 0.0)) && (!(scratch.values[1758] != 0.0))) && (!(scratch.values[1760] != 0.0))) && (!(scratch.values[1762] != 0.0))) {
            scratch.store_ad(1356, &AdValue::pow(AdValue::mul(scratch.ad_value(1359), scratch.ad_value(628)), scratch.ad_value(549)));
        }

        if ((((scratch.values[1320] != 0.0) && (scratch.values[1337] != 0.0)) && (!(scratch.values[1758] != 0.0))) && (!(scratch.values[1760] != 0.0))) {
            scratch.store_ad(1363, &AdValue::mul(scratch.ad_value(622), scratch.ad_value(1356)));
        }

        if ((((scratch.values[1320] != 0.0) && (scratch.values[1337] != 0.0)) && (!(scratch.values[1758] != 0.0))) && (!(scratch.values[1760] != 0.0))) {
            scratch.store_ad(1364, &AdValue::mul(scratch.ad_value(592), AdValue::mul(AdValue::offset(scratch.ad_value(1350), (-1.0)), scratch.ad_value(1363))));
        }

        if ((((scratch.values[1320] != 0.0) && (scratch.values[1337] != 0.0)) && (!(scratch.values[1758] != 0.0))) && (!(scratch.values[1760] != 0.0))) {
            scratch.store_ad(1358, &AdValue::mul(scratch.ad_value(560), AdValue::mul(scratch.ad_value(1364), scratch.ad_value(1362))));
        }

        scratch.values[1763] = if (scratch.values[563] == 0.0) { 1.0 } else { 0.0 };

        if ((((scratch.values[1320] != 0.0) && (scratch.values[1337] != 0.0)) && (!(scratch.values[1758] != 0.0))) && (scratch.values[1763] != 0.0)) {
            scratch.values[1365] = 0.0;
            scratch.node_derivatives[1365] = [0.0; Instance::NODE_COUNT];
            scratch.branch_derivatives[1365] = [0.0; Instance::BRANCH_COUNT];
        }

        if ((((scratch.values[1320] != 0.0) && (scratch.values[1337] != 0.0)) && (!(scratch.values[1758] != 0.0))) && (!(scratch.values[1763] != 0.0))) {
            scratch.store_ad(1366, &AdValue::mul(scratch.ad_value(637), AdValue::div(AdValue::mul(scratch.ad_value(1363), scratch.ad_value(607)), scratch.ad_value(1359))));
        }

        if ((((scratch.values[1320] != 0.0) && (scratch.values[1337] != 0.0)) && (!(scratch.values[1758] != 0.0))) && (!(scratch.values[1763] != 0.0))) {
            scratch.store_ad(1367, &AdValue::div(AdValue::scale(scratch.ad_value(634), 0.666666666666667), scratch.ad_value(1366)));
        }

        if ((((scratch.values[1320] != 0.0) && (scratch.values[1337] != 0.0)) && (!(scratch.values[1758] != 0.0))) && (!(scratch.values[1763] != 0.0))) {
            scratch.store_ad(1368, &AdValue::square(scratch.ad_value(1367)));
        }

        if ((((scratch.values[1320] != 0.0) && (scratch.values[1337] != 0.0)) && (!(scratch.values[1758] != 0.0))) && (!(scratch.values[1763] != 0.0))) {
            scratch.store_ad(1369, &AdValue::sqrt(AdValue::div(AdValue::square(scratch.ad_value(1368)), AdValue::offset(AdValue::square(scratch.ad_value(1368)), 1.0))));
        }

        if ((((scratch.values[1320] != 0.0) && (scratch.values[1337] != 0.0)) && (!(scratch.values[1758] != 0.0))) && (!(scratch.values[1763] != 0.0))) {
            scratch.store_ad(1370, &AdValue::sqrt(AdValue::abs(scratch.ad_value(1369))));
        }

        if ((((scratch.values[1320] != 0.0) && (scratch.values[1337] != 0.0)) && (!(scratch.values[1758] != 0.0))) && (!(scratch.values[1763] != 0.0))) {
            scratch.store_ad(1371, &AdValue::mul(scratch.ad_value(1369), scratch.ad_value(1370)));
        }

        scratch.values[1764] = if (((-scratch.values[549]) * scratch.values[610]) == (-1.0)) { 1.0 } else { 0.0 };

        if (((((scratch.values[1320] != 0.0) && (scratch.values[1337] != 0.0)) && (!(scratch.values[1758] != 0.0))) && (!(scratch.values[1763] != 0.0))) && (scratch.values[1764] != 0.0)) {
            scratch.store_ad(1372, &AdValue::div_from_scalar(1.0, AdValue::offset(AdValue::mul(scratch.ad_value(1366), scratch.ad_value(1371)), 1.0)));
        }

        if (((((scratch.values[1320] != 0.0) && (scratch.values[1337] != 0.0)) && (!(scratch.values[1758] != 0.0))) && (!(scratch.values[1763] != 0.0))) && (!(scratch.values[1764] != 0.0))) {
            scratch.store_ad(1372, &AdValue::pow(AdValue::offset(AdValue::mul(scratch.ad_value(1366), scratch.ad_value(1371)), 1.0), AdValue::mul(AdValue::neg(scratch.ad_value(549)), scratch.ad_value(610))));
        }

        if ((((scratch.values[1320] != 0.0) && (scratch.values[1337] != 0.0)) && (!(scratch.values[1758] != 0.0))) && (!(scratch.values[1763] != 0.0))) {
            scratch.store_ad(1373, &AdValue::div(AdValue::mul(scratch.ad_value(1362), scratch.ad_value(1372)), AdValue::add(scratch.ad_value(1362), scratch.ad_value(1372))));
        }

        if ((((scratch.values[1320] != 0.0) && (scratch.values[1337] != 0.0)) && (!(scratch.values[1758] != 0.0))) && (!(scratch.values[1763] != 0.0))) {
            scratch.store_ad(1374, &AdValue::sqrt(AdValue::scale(AdValue::div(scratch.ad_value(1366), scratch.ad_value(1370)), 0.375)));
        }

        if ((((scratch.values[1320] != 0.0) && (scratch.values[1337] != 0.0)) && (!(scratch.values[1758] != 0.0))) && (!(scratch.values[1763] != 0.0))) {
            scratch.store_ad(1375, &AdValue::sub(AdValue::scale(AdValue::mul(scratch.ad_value(1367), scratch.ad_value(1370)), 2.0), scratch.ad_value(1369)));
        }

        if ((((scratch.values[1320] != 0.0) && (scratch.values[1337] != 0.0)) && (!(scratch.values[1758] != 0.0))) && (!(scratch.values[1763] != 0.0))) {
            scratch.store_ad(1376, &AdValue::add(AdValue::sub(AdValue::mul(AdValue::mul(scratch.ad_value(634), scratch.ad_value(1367)), scratch.ad_value(1370)), AdValue::mul(scratch.ad_value(634), scratch.ad_value(1369))), AdValue::scale(AdValue::mul(scratch.ad_value(1366), scratch.ad_value(1371)), 0.5)));
        }

        if ((((scratch.values[1320] != 0.0) && (scratch.values[1337] != 0.0)) && (!(scratch.values[1758] != 0.0))) && (!(scratch.values[1763] != 0.0))) {
            scratch.store_ad(1377, &AdValue::mul(AdValue::offset(scratch.ad_value(1375), (-1.0)), scratch.ad_value(1374)));
        }

        if ((((scratch.values[1320] != 0.0) && (scratch.values[1337] != 0.0)) && (!(scratch.values[1758] != 0.0))) && (!(scratch.values[1763] != 0.0))) {
            scratch.store_ad(1338, &AdValue::square(scratch.ad_value(1377)));
        }

        scratch.values[1765] = if (scratch.values[1377] > 0.0) { 1.0 } else { 0.0 };

        if (((((scratch.values[1320] != 0.0) && (scratch.values[1337] != 0.0)) && (!(scratch.values[1758] != 0.0))) && (!(scratch.values[1763] != 0.0))) && (scratch.values[1765] != 0.0)) {
            scratch.store_ad(1339, &AdValue::div_from_scalar(1.0, AdValue::offset(AdValue::scale(scratch.ad_value(1377), scratch.values[428]), 1.0)));
        }

        if (((((scratch.values[1320] != 0.0) && (scratch.values[1337] != 0.0)) && (!(scratch.values[1758] != 0.0))) && (!(scratch.values[1763] != 0.0))) && (!(scratch.values[1765] != 0.0))) {
            scratch.store_ad(1339, &AdValue::div_from_scalar(1.0, AdValue::sub_from_scalar(1.0, AdValue::scale(scratch.ad_value(1377), scratch.values[428]))));
        }

        scratch.values[1766] = if (((-scratch.values[1338]) + scratch.values[1376]) > (-230.25850929940458)) { 1.0 } else { 0.0 };

        if (((((scratch.values[1320] != 0.0) && (scratch.values[1337] != 0.0)) && (!(scratch.values[1758] != 0.0))) && (!(scratch.values[1763] != 0.0))) && (scratch.values[1766] != 0.0)) {
            scratch.store_ad(1356, &AdValue::exp(AdValue::sub(scratch.ad_value(1376), scratch.ad_value(1338))));
        }

        if (((((scratch.values[1320] != 0.0) && (scratch.values[1337] != 0.0)) && (!(scratch.values[1758] != 0.0))) && (!(scratch.values[1763] != 0.0))) && (!(scratch.values[1766] != 0.0))) {
            let assign29350_ad_e36176: AdValue = AdValue::div_from_scalar(1e-100, AdValue::offset(AdValue::mul(AdValue::sub_from_scalar((-230.25850929940458), AdValue::sub(scratch.ad_value(1376), scratch.ad_value(1338))), AdValue::offset(AdValue::scale(AdValue::mul(AdValue::sub_from_scalar((-230.25850929940458), AdValue::sub(scratch.ad_value(1376), scratch.ad_value(1338))), AdValue::offset(AdValue::scale(AdValue::sub_from_scalar((-230.25850929940458), AdValue::sub(scratch.ad_value(1376), scratch.ad_value(1338))), 0.3333333333333333), 1.0)), 0.5), 1.0)), 1.0));
            scratch.store_ad(1356, &assign29350_ad_e36176);
        }

        if ((((scratch.values[1320] != 0.0) && (scratch.values[1337] != 0.0)) && (!(scratch.values[1758] != 0.0))) && (!(scratch.values[1763] != 0.0))) {
            scratch.store_ad(1340, &AdValue::mul(AdValue::add(AdValue::add(AdValue::scale(scratch.ad_value(1339), 0.29214664), AdValue::scale(AdValue::square(scratch.ad_value(1339)), scratch.values[429])), AdValue::scale(AdValue::mul(AdValue::square(scratch.ad_value(1339)), scratch.ad_value(1339)), scratch.values[430])), scratch.ad_value(1356)));
        }

        scratch.values[1767] = if (scratch.values[1377] > 0.0) { 1.0 } else { 0.0 };

        if (((((scratch.values[1320] != 0.0) && (scratch.values[1337] != 0.0)) && (!(scratch.values[1758] != 0.0))) && (!(scratch.values[1763] != 0.0))) && (scratch.values[1767] != 0.0)) {
            scratch.values[1378] = scratch.values[1340];
            scratch.node_derivatives[1378] = scratch.node_derivatives[1340];
            scratch.branch_derivatives[1378] = scratch.branch_derivatives[1340];
        }

        scratch.values[1768] = if (scratch.values[1376] > (-230.25850929940458)) { 1.0 } else { 0.0 };

        if ((((((scratch.values[1320] != 0.0) && (scratch.values[1337] != 0.0)) && (!(scratch.values[1758] != 0.0))) && (!(scratch.values[1763] != 0.0))) && (!(scratch.values[1767] != 0.0))) && (scratch.values[1768] != 0.0)) {
            scratch.store_ad(1356, &AdValue::exp(scratch.ad_value(1376)));
        }

        if ((((((scratch.values[1320] != 0.0) && (scratch.values[1337] != 0.0)) && (!(scratch.values[1758] != 0.0))) && (!(scratch.values[1763] != 0.0))) && (!(scratch.values[1767] != 0.0))) && (!(scratch.values[1768] != 0.0))) {
            scratch.store_ad(1356, &AdValue::div_from_scalar(1e-100, AdValue::offset(AdValue::mul(AdValue::sub_from_scalar((-230.25850929940458), scratch.ad_value(1376)), AdValue::offset(AdValue::scale(AdValue::mul(AdValue::sub_from_scalar((-230.25850929940458), scratch.ad_value(1376)), AdValue::offset(AdValue::scale(AdValue::sub_from_scalar((-230.25850929940458), scratch.ad_value(1376)), 0.3333333333333333), 1.0)), 0.5), 1.0)), 1.0)));
        }

        if (((((scratch.values[1320] != 0.0) && (scratch.values[1337] != 0.0)) && (!(scratch.values[1758] != 0.0))) && (!(scratch.values[1763] != 0.0))) && (!(scratch.values[1767] != 0.0))) {
            scratch.store_ad(1378, &AdValue::sub(AdValue::scale(scratch.ad_value(1356), 2.0), scratch.ad_value(1340)));
        }

        if ((((scratch.values[1320] != 0.0) && (scratch.values[1337] != 0.0)) && (!(scratch.values[1758] != 0.0))) && (!(scratch.values[1763] != 0.0))) {
            scratch.store_ad(1379, &AdValue::scale(AdValue::div(AdValue::mul(scratch.ad_value(634), scratch.ad_value(1378)), scratch.ad_value(1374)), (1.772453850905516 * 0.5)));
        }

        if ((((scratch.values[1320] != 0.0) && (scratch.values[1337] != 0.0)) && (!(scratch.values[1758] != 0.0))) && (!(scratch.values[1763] != 0.0))) {
            scratch.store_ad(1365, &AdValue::mul(scratch.ad_value(563), AdValue::mul(AdValue::mul(scratch.ad_value(1364), scratch.ad_value(1379)), scratch.ad_value(1373))));
        }

        scratch.values[1769] = if (scratch.values[569] == 0.0) { 1.0 } else { 0.0 };

        if ((((scratch.values[1320] != 0.0) && (scratch.values[1337] != 0.0)) && (!(scratch.values[1758] != 0.0))) && (scratch.values[1769] != 0.0)) {
            scratch.values[1380] = 0.0;
            scratch.node_derivatives[1380] = [0.0; Instance::NODE_COUNT];
            scratch.branch_derivatives[1380] = [0.0; Instance::BRANCH_COUNT];
        }

        scratch.values[1770] = if (scratch.values[549] == 0.5) { 1.0 } else { 0.0 };

        if (((((scratch.values[1320] != 0.0) && (scratch.values[1337] != 0.0)) && (!(scratch.values[1758] != 0.0))) && (!(scratch.values[1769] != 0.0))) && (scratch.values[1770] != 0.0)) {
            scratch.store_ad(1356, &AdValue::sqrt(AdValue::mul(AdValue::sub(scratch.ad_value(546), scratch.ad_value(1354)), scratch.ad_value(628))));
        }

        if (((((scratch.values[1320] != 0.0) && (scratch.values[1337] != 0.0)) && (!(scratch.values[1758] != 0.0))) && (!(scratch.values[1769] != 0.0))) && (!(scratch.values[1770] != 0.0))) {
            scratch.store_ad(1356, &AdValue::pow(AdValue::mul(AdValue::sub(scratch.ad_value(546), scratch.ad_value(1354)), scratch.ad_value(628)), scratch.ad_value(549)));
        }

        if ((((scratch.values[1320] != 0.0) && (scratch.values[1337] != 0.0)) && (!(scratch.values[1758] != 0.0))) && (!(scratch.values[1769] != 0.0))) {
            scratch.store_ad(1381, &AdValue::mul(scratch.ad_value(610), AdValue::div(AdValue::mul(AdValue::sub(scratch.ad_value(546), scratch.ad_value(1354)), scratch.ad_value(625)), scratch.ad_value(1356))));
        }

        scratch.values[1771] = if (((((-scratch.values[640]) / scratch.values[1381])) as f64).abs() < 230.25850929940458) { 1.0 } else { 0.0 };

        if (((((scratch.values[1320] != 0.0) && (scratch.values[1337] != 0.0)) && (!(scratch.values[1758] != 0.0))) && (!(scratch.values[1769] != 0.0))) && (scratch.values[1771] != 0.0)) {
            scratch.store_ad(1356, &AdValue::exp(AdValue::div(AdValue::neg(scratch.ad_value(640)), scratch.ad_value(1381))));
        }

        scratch.values[1772] = if (((-scratch.values[640]) / scratch.values[1381]) < 0.0) { 1.0 } else { 0.0 };

        if ((((((scratch.values[1320] != 0.0) && (scratch.values[1337] != 0.0)) && (!(scratch.values[1758] != 0.0))) && (!(scratch.values[1769] != 0.0))) && (!(scratch.values[1771] != 0.0))) && (scratch.values[1772] != 0.0)) {
            let assign29540_ad_e36503: AdValue = AdValue::offset(AdValue::mul(AdValue::sub_from_scalar((-230.25850929940458), AdValue::div(AdValue::neg(scratch.ad_value(640)), scratch.ad_value(1381))), AdValue::offset(AdValue::scale(AdValue::mul(AdValue::sub_from_scalar((-230.25850929940458), AdValue::div(AdValue::neg(scratch.ad_value(640)), scratch.ad_value(1381))), AdValue::offset(AdValue::scale(AdValue::sub_from_scalar((-230.25850929940458), AdValue::div(AdValue::neg(scratch.ad_value(640)), scratch.ad_value(1381))), 0.3333333333333333), 1.0)), 0.5), 1.0)), 1.0);
            scratch.store_ad(1356, &AdValue::div_from_scalar(1e-100, assign29540_ad_e36503));
        }

        if ((((((scratch.values[1320] != 0.0) && (scratch.values[1337] != 0.0)) && (!(scratch.values[1758] != 0.0))) && (!(scratch.values[1769] != 0.0))) && (!(scratch.values[1771] != 0.0))) && (!(scratch.values[1772] != 0.0))) {
            let assign29550_ad_e36553: AdValue = AdValue::scale(AdValue::offset(AdValue::mul(AdValue::offset(AdValue::div(AdValue::neg(scratch.ad_value(640)), scratch.ad_value(1381)), (-230.25850929940458)), AdValue::offset(AdValue::scale(AdValue::mul(AdValue::offset(AdValue::div(AdValue::neg(scratch.ad_value(640)), scratch.ad_value(1381)), (-230.25850929940458)), AdValue::offset(AdValue::scale(AdValue::offset(AdValue::div(AdValue::neg(scratch.ad_value(640)), scratch.ad_value(1381)), (-230.25850929940458)), 0.3333333333333333), 1.0)), 0.5), 1.0)), 1.0), 1e100);
            scratch.store_ad(1356, &assign29550_ad_e36553);
        }

        if ((((scratch.values[1320] != 0.0) && (scratch.values[1337] != 0.0)) && (!(scratch.values[1758] != 0.0))) && (!(scratch.values[1769] != 0.0))) {
            scratch.store_ad(1380, &AdValue::mul(scratch.ad_value(569), AdValue::mul(AdValue::mul(AdValue::mul(scratch.ad_value(524), scratch.ad_value(1381)), scratch.ad_value(1381)), scratch.ad_value(1356))));
        }

        scratch.values[1773] = if (scratch.values[578] > 1000.0) { 1.0 } else { 0.0 };

        if ((((scratch.values[1320] != 0.0) && (scratch.values[1337] != 0.0)) && (!(scratch.values[1758] != 0.0))) && (scratch.values[1773] != 0.0)) {
            scratch.values[1382] = 1.0;
            scratch.node_derivatives[1382] = [0.0; Instance::NODE_COUNT];
            scratch.branch_derivatives[1382] = [0.0; Instance::BRANCH_COUNT];
        }

        scratch.values[1774] = if (scratch.values[1355] > ((-scratch.values[500]) * scratch.values[578])) { 1.0 } else { 0.0 };

        scratch.values[1775] = if (scratch.values[581] == 4.0) { 1.0 } else { 0.0 };

        if ((((((scratch.values[1320] != 0.0) && (scratch.values[1337] != 0.0)) && (!(scratch.values[1758] != 0.0))) && (!(scratch.values[1773] != 0.0))) && (scratch.values[1774] != 0.0)) && (scratch.values[1775] != 0.0)) {
            scratch.store_ad(1356, &AdValue::mul(AdValue::mul(AdValue::mul(AdValue::mul(scratch.ad_value(1355), scratch.ad_value(646)), AdValue::mul(scratch.ad_value(1355), scratch.ad_value(646))), AdValue::mul(scratch.ad_value(1355), scratch.ad_value(646))), AdValue::mul(scratch.ad_value(1355), scratch.ad_value(646))));
        }

        if ((((((scratch.values[1320] != 0.0) && (scratch.values[1337] != 0.0)) && (!(scratch.values[1758] != 0.0))) && (!(scratch.values[1773] != 0.0))) && (scratch.values[1774] != 0.0)) && (!(scratch.values[1775] != 0.0))) {
            scratch.store_ad(1356, &AdValue::pow(AdValue::abs(AdValue::mul(scratch.ad_value(1355), scratch.ad_value(646))), scratch.ad_value(581)));
        }

        if (((((scratch.values[1320] != 0.0) && (scratch.values[1337] != 0.0)) && (!(scratch.values[1758] != 0.0))) && (!(scratch.values[1773] != 0.0))) && (scratch.values[1774] != 0.0)) {
            scratch.store_ad(1382, &AdValue::div_from_scalar(1.0, AdValue::sub_from_scalar(1.0, scratch.ad_value(1356))));
        }

        if (((((scratch.values[1320] != 0.0) && (scratch.values[1337] != 0.0)) && (!(scratch.values[1758] != 0.0))) && (!(scratch.values[1773] != 0.0))) && (!(scratch.values[1774] != 0.0))) {
            scratch.store_ad(1382, &AdValue::add(scratch.ad_value(643), AdValue::mul(AdValue::add(scratch.ad_value(1355), AdValue::scale(scratch.ad_value(578), scratch.values[500])), scratch.ad_value(649))));
        }

        if (((scratch.values[1320] != 0.0) && (scratch.values[1337] != 0.0)) && (!(scratch.values[1758] != 0.0))) {
            scratch.store_ad(1383, &AdValue::mul(AdValue::add(AdValue::add(AdValue::add(scratch.ad_value(1357), scratch.ad_value(1358)), scratch.ad_value(1365)), scratch.ad_value(1380)), scratch.ad_value(1382)));
        }

        scratch.values[1776] = if (scratch.values[725] == 0.0) { 1.0 } else { 0.0 };

        if (((scratch.values[1320] != 0.0) && (scratch.values[1337] != 0.0)) && (scratch.values[1776] != 0.0)) {
            scratch.values[1384] = 0.0;
            scratch.node_derivatives[1384] = [0.0; Instance::NODE_COUNT];
            scratch.branch_derivatives[1384] = [0.0; Instance::BRANCH_COUNT];
        }

        scratch.values[1777] = if (scratch.values[608] == 0.5) { 1.0 } else { 0.0 };

        if ((((scratch.values[1320] != 0.0) && (scratch.values[1337] != 0.0)) && (!(scratch.values[1776] != 0.0))) && (scratch.values[1777] != 0.0)) {
            scratch.store_ad(1356, &AdValue::sqrt(AdValue::sub_from_scalar(1.0, AdValue::mul(scratch.ad_value(1348), scratch.ad_value(605)))));
        }

        if ((((scratch.values[1320] != 0.0) && (scratch.values[1337] != 0.0)) && (!(scratch.values[1776] != 0.0))) && (!(scratch.values[1777] != 0.0))) {
            scratch.store_ad(1356, &AdValue::pow(AdValue::sub_from_scalar(1.0, AdValue::mul(scratch.ad_value(1348), scratch.ad_value(605))), scratch.ad_value(608)));
        }

        if (((scratch.values[1320] != 0.0) && (scratch.values[1337] != 0.0)) && (!(scratch.values[1776] != 0.0))) {
            scratch.store_ad(1357, &AdValue::mul(scratch.ad_value(596), scratch.ad_value(1347)));
        }

        scratch.values[1778] = if ((scratch.values[561] == 0.0) && (scratch.values[564] == 0.0)) { 1.0 } else { 0.0 };

        if ((((scratch.values[1320] != 0.0) && (scratch.values[1337] != 0.0)) && (!(scratch.values[1776] != 0.0))) && (scratch.values[1778] != 0.0)) {
            scratch.values[1358] = 0.0;
            scratch.node_derivatives[1358] = [0.0; Instance::NODE_COUNT];
            scratch.branch_derivatives[1358] = [0.0; Instance::BRANCH_COUNT];
        }

        if ((((scratch.values[1320] != 0.0) && (scratch.values[1337] != 0.0)) && (!(scratch.values[1776] != 0.0))) && (!(scratch.values[1778] != 0.0))) {
            scratch.store_ad(1359, &AdValue::sub(scratch.ad_value(602), scratch.ad_value(1353)));
        }

        if ((((scratch.values[1320] != 0.0) && (scratch.values[1337] != 0.0)) && (!(scratch.values[1776] != 0.0))) && (!(scratch.values[1778] != 0.0))) {
            scratch.store_ad(1360, &AdValue::sub_from_scalar(1.0, AdValue::sqrt(AdValue::sub_from_scalar(1.0, AdValue::div(scratch.ad_value(1351), scratch.ad_value(1359))))));
        }

        scratch.values[1779] = if (scratch.values[550] == 0.5) { 1.0 } else { 0.0 };

        if (((((scratch.values[1320] != 0.0) && (scratch.values[1337] != 0.0)) && (!(scratch.values[1776] != 0.0))) && (!(scratch.values[1778] != 0.0))) && (scratch.values[1779] != 0.0)) {
            scratch.values[1361] = 0.0;
            scratch.node_derivatives[1361] = [0.0; Instance::NODE_COUNT];
            scratch.branch_derivatives[1361] = [0.0; Instance::BRANCH_COUNT];
        }

        if (((((scratch.values[1320] != 0.0) && (scratch.values[1337] != 0.0)) && (!(scratch.values[1776] != 0.0))) && (!(scratch.values[1778] != 0.0))) && (!(scratch.values[1779] != 0.0))) {
            scratch.store_ad(1361, &AdValue::mul(AdValue::add(AdValue::div(AdValue::mul(AdValue::square(scratch.ad_value(1360)), AdValue::ln(scratch.ad_value(1360))), AdValue::sub_from_scalar(1.0, scratch.ad_value(1360))), scratch.ad_value(1360)), AdValue::sub_from_scalar(1.0, AdValue::scale(scratch.ad_value(550), 2.0))));
        }

        if ((((scratch.values[1320] != 0.0) && (scratch.values[1337] != 0.0)) && (!(scratch.values[1776] != 0.0))) && (!(scratch.values[1778] != 0.0))) {
            scratch.store_ad(1362, &AdValue::add(scratch.ad_value(1360), scratch.ad_value(1361)));
        }

        scratch.values[1780] = if (scratch.values[550] == 0.5) { 1.0 } else { 0.0 };

        if (((((scratch.values[1320] != 0.0) && (scratch.values[1337] != 0.0)) && (!(scratch.values[1776] != 0.0))) && (!(scratch.values[1778] != 0.0))) && (scratch.values[1780] != 0.0)) {
            scratch.store_ad(1356, &AdValue::sqrt(AdValue::mul(scratch.ad_value(1359), scratch.ad_value(629))));
        }

        if (((((scratch.values[1320] != 0.0) && (scratch.values[1337] != 0.0)) && (!(scratch.values[1776] != 0.0))) && (!(scratch.values[1778] != 0.0))) && (!(scratch.values[1780] != 0.0))) {
            scratch.store_ad(1356, &AdValue::pow(AdValue::mul(scratch.ad_value(1359), scratch.ad_value(629)), scratch.ad_value(550)));
        }

        if ((((scratch.values[1320] != 0.0) && (scratch.values[1337] != 0.0)) && (!(scratch.values[1776] != 0.0))) && (!(scratch.values[1778] != 0.0))) {
            scratch.store_ad(1363, &AdValue::mul(scratch.ad_value(623), scratch.ad_value(1356)));
        }

        if ((((scratch.values[1320] != 0.0) && (scratch.values[1337] != 0.0)) && (!(scratch.values[1776] != 0.0))) && (!(scratch.values[1778] != 0.0))) {
            scratch.store_ad(1364, &AdValue::mul(scratch.ad_value(593), AdValue::mul(AdValue::offset(scratch.ad_value(1350), (-1.0)), scratch.ad_value(1363))));
        }

        if ((((scratch.values[1320] != 0.0) && (scratch.values[1337] != 0.0)) && (!(scratch.values[1776] != 0.0))) && (!(scratch.values[1778] != 0.0))) {
            scratch.store_ad(1358, &AdValue::mul(scratch.ad_value(561), AdValue::mul(scratch.ad_value(1364), scratch.ad_value(1362))));
        }

        scratch.values[1781] = if (scratch.values[564] == 0.0) { 1.0 } else { 0.0 };

        if ((((scratch.values[1320] != 0.0) && (scratch.values[1337] != 0.0)) && (!(scratch.values[1776] != 0.0))) && (scratch.values[1781] != 0.0)) {
            scratch.values[1365] = 0.0;
            scratch.node_derivatives[1365] = [0.0; Instance::NODE_COUNT];
            scratch.branch_derivatives[1365] = [0.0; Instance::BRANCH_COUNT];
        }

        if ((((scratch.values[1320] != 0.0) && (scratch.values[1337] != 0.0)) && (!(scratch.values[1776] != 0.0))) && (!(scratch.values[1781] != 0.0))) {
            scratch.store_ad(1366, &AdValue::mul(scratch.ad_value(638), AdValue::div(AdValue::mul(scratch.ad_value(1363), scratch.ad_value(608)), scratch.ad_value(1359))));
        }

        if ((((scratch.values[1320] != 0.0) && (scratch.values[1337] != 0.0)) && (!(scratch.values[1776] != 0.0))) && (!(scratch.values[1781] != 0.0))) {
            scratch.store_ad(1367, &AdValue::div(AdValue::scale(scratch.ad_value(635), 0.666666666666667), scratch.ad_value(1366)));
        }

        if ((((scratch.values[1320] != 0.0) && (scratch.values[1337] != 0.0)) && (!(scratch.values[1776] != 0.0))) && (!(scratch.values[1781] != 0.0))) {
            scratch.store_ad(1368, &AdValue::square(scratch.ad_value(1367)));
        }

        if ((((scratch.values[1320] != 0.0) && (scratch.values[1337] != 0.0)) && (!(scratch.values[1776] != 0.0))) && (!(scratch.values[1781] != 0.0))) {
            scratch.store_ad(1369, &AdValue::sqrt(AdValue::div(AdValue::square(scratch.ad_value(1368)), AdValue::offset(AdValue::square(scratch.ad_value(1368)), 1.0))));
        }

        if ((((scratch.values[1320] != 0.0) && (scratch.values[1337] != 0.0)) && (!(scratch.values[1776] != 0.0))) && (!(scratch.values[1781] != 0.0))) {
            scratch.store_ad(1370, &AdValue::sqrt(AdValue::abs(scratch.ad_value(1369))));
        }

        if ((((scratch.values[1320] != 0.0) && (scratch.values[1337] != 0.0)) && (!(scratch.values[1776] != 0.0))) && (!(scratch.values[1781] != 0.0))) {
            scratch.store_ad(1371, &AdValue::mul(scratch.ad_value(1369), scratch.ad_value(1370)));
        }

        scratch.values[1782] = if (((-scratch.values[550]) * scratch.values[611]) == (-1.0)) { 1.0 } else { 0.0 };

        if (((((scratch.values[1320] != 0.0) && (scratch.values[1337] != 0.0)) && (!(scratch.values[1776] != 0.0))) && (!(scratch.values[1781] != 0.0))) && (scratch.values[1782] != 0.0)) {
            scratch.store_ad(1372, &AdValue::div_from_scalar(1.0, AdValue::offset(AdValue::mul(scratch.ad_value(1366), scratch.ad_value(1371)), 1.0)));
        }

        if (((((scratch.values[1320] != 0.0) && (scratch.values[1337] != 0.0)) && (!(scratch.values[1776] != 0.0))) && (!(scratch.values[1781] != 0.0))) && (!(scratch.values[1782] != 0.0))) {
            scratch.store_ad(1372, &AdValue::pow(AdValue::offset(AdValue::mul(scratch.ad_value(1366), scratch.ad_value(1371)), 1.0), AdValue::mul(AdValue::neg(scratch.ad_value(550)), scratch.ad_value(611))));
        }

        if ((((scratch.values[1320] != 0.0) && (scratch.values[1337] != 0.0)) && (!(scratch.values[1776] != 0.0))) && (!(scratch.values[1781] != 0.0))) {
            scratch.store_ad(1373, &AdValue::div(AdValue::mul(scratch.ad_value(1362), scratch.ad_value(1372)), AdValue::add(scratch.ad_value(1362), scratch.ad_value(1372))));
        }

        if ((((scratch.values[1320] != 0.0) && (scratch.values[1337] != 0.0)) && (!(scratch.values[1776] != 0.0))) && (!(scratch.values[1781] != 0.0))) {
            scratch.store_ad(1374, &AdValue::sqrt(AdValue::scale(AdValue::div(scratch.ad_value(1366), scratch.ad_value(1370)), 0.375)));
        }

        if ((((scratch.values[1320] != 0.0) && (scratch.values[1337] != 0.0)) && (!(scratch.values[1776] != 0.0))) && (!(scratch.values[1781] != 0.0))) {
            scratch.store_ad(1375, &AdValue::sub(AdValue::scale(AdValue::mul(scratch.ad_value(1367), scratch.ad_value(1370)), 2.0), scratch.ad_value(1369)));
        }

        if ((((scratch.values[1320] != 0.0) && (scratch.values[1337] != 0.0)) && (!(scratch.values[1776] != 0.0))) && (!(scratch.values[1781] != 0.0))) {
            scratch.store_ad(1376, &AdValue::add(AdValue::sub(AdValue::mul(AdValue::mul(scratch.ad_value(635), scratch.ad_value(1367)), scratch.ad_value(1370)), AdValue::mul(scratch.ad_value(635), scratch.ad_value(1369))), AdValue::scale(AdValue::mul(scratch.ad_value(1366), scratch.ad_value(1371)), 0.5)));
        }

        if ((((scratch.values[1320] != 0.0) && (scratch.values[1337] != 0.0)) && (!(scratch.values[1776] != 0.0))) && (!(scratch.values[1781] != 0.0))) {
            scratch.store_ad(1377, &AdValue::mul(AdValue::offset(scratch.ad_value(1375), (-1.0)), scratch.ad_value(1374)));
        }

    }

    pub(super) fn stamp_transient_block_23(
        &mut self,
        ctx: &GeneratedEvalContext<'_>,
        stamper: &mut GeneratedStamper<'_>,
        scratch: &mut Scratch,
    ) {
        let _ = stamper;
        if ((((scratch.values[1320] != 0.0) && (scratch.values[1337] != 0.0)) && (!(scratch.values[1776] != 0.0))) && (!(scratch.values[1781] != 0.0))) {
            scratch.store_ad(1338, &AdValue::square(scratch.ad_value(1377)));
        }

        scratch.values[1783] = if (scratch.values[1377] > 0.0) { 1.0 } else { 0.0 };

        if (((((scratch.values[1320] != 0.0) && (scratch.values[1337] != 0.0)) && (!(scratch.values[1776] != 0.0))) && (!(scratch.values[1781] != 0.0))) && (scratch.values[1783] != 0.0)) {
            scratch.store_ad(1339, &AdValue::div_from_scalar(1.0, AdValue::offset(AdValue::scale(scratch.ad_value(1377), scratch.values[428]), 1.0)));
        }

        if (((((scratch.values[1320] != 0.0) && (scratch.values[1337] != 0.0)) && (!(scratch.values[1776] != 0.0))) && (!(scratch.values[1781] != 0.0))) && (!(scratch.values[1783] != 0.0))) {
            scratch.store_ad(1339, &AdValue::div_from_scalar(1.0, AdValue::sub_from_scalar(1.0, AdValue::scale(scratch.ad_value(1377), scratch.values[428]))));
        }

        scratch.values[1784] = if (((-scratch.values[1338]) + scratch.values[1376]) > (-230.25850929940458)) { 1.0 } else { 0.0 };

        if (((((scratch.values[1320] != 0.0) && (scratch.values[1337] != 0.0)) && (!(scratch.values[1776] != 0.0))) && (!(scratch.values[1781] != 0.0))) && (scratch.values[1784] != 0.0)) {
            scratch.store_ad(1356, &AdValue::exp(AdValue::sub(scratch.ad_value(1376), scratch.ad_value(1338))));
        }

        if (((((scratch.values[1320] != 0.0) && (scratch.values[1337] != 0.0)) && (!(scratch.values[1776] != 0.0))) && (!(scratch.values[1781] != 0.0))) && (!(scratch.values[1784] != 0.0))) {
            let assign30100_ad_e37382: AdValue = AdValue::div_from_scalar(1e-100, AdValue::offset(AdValue::mul(AdValue::sub_from_scalar((-230.25850929940458), AdValue::sub(scratch.ad_value(1376), scratch.ad_value(1338))), AdValue::offset(AdValue::scale(AdValue::mul(AdValue::sub_from_scalar((-230.25850929940458), AdValue::sub(scratch.ad_value(1376), scratch.ad_value(1338))), AdValue::offset(AdValue::scale(AdValue::sub_from_scalar((-230.25850929940458), AdValue::sub(scratch.ad_value(1376), scratch.ad_value(1338))), 0.3333333333333333), 1.0)), 0.5), 1.0)), 1.0));
            scratch.store_ad(1356, &assign30100_ad_e37382);
        }

        if ((((scratch.values[1320] != 0.0) && (scratch.values[1337] != 0.0)) && (!(scratch.values[1776] != 0.0))) && (!(scratch.values[1781] != 0.0))) {
            scratch.store_ad(1340, &AdValue::mul(AdValue::add(AdValue::add(AdValue::scale(scratch.ad_value(1339), 0.29214664), AdValue::scale(AdValue::square(scratch.ad_value(1339)), scratch.values[429])), AdValue::scale(AdValue::mul(AdValue::square(scratch.ad_value(1339)), scratch.ad_value(1339)), scratch.values[430])), scratch.ad_value(1356)));
        }

        scratch.values[1785] = if (scratch.values[1377] > 0.0) { 1.0 } else { 0.0 };

        if (((((scratch.values[1320] != 0.0) && (scratch.values[1337] != 0.0)) && (!(scratch.values[1776] != 0.0))) && (!(scratch.values[1781] != 0.0))) && (scratch.values[1785] != 0.0)) {
            scratch.values[1378] = scratch.values[1340];
            scratch.node_derivatives[1378] = scratch.node_derivatives[1340];
            scratch.branch_derivatives[1378] = scratch.branch_derivatives[1340];
        }

        scratch.values[1786] = if (scratch.values[1376] > (-230.25850929940458)) { 1.0 } else { 0.0 };

        if ((((((scratch.values[1320] != 0.0) && (scratch.values[1337] != 0.0)) && (!(scratch.values[1776] != 0.0))) && (!(scratch.values[1781] != 0.0))) && (!(scratch.values[1785] != 0.0))) && (scratch.values[1786] != 0.0)) {
            scratch.store_ad(1356, &AdValue::exp(scratch.ad_value(1376)));
        }

        if ((((((scratch.values[1320] != 0.0) && (scratch.values[1337] != 0.0)) && (!(scratch.values[1776] != 0.0))) && (!(scratch.values[1781] != 0.0))) && (!(scratch.values[1785] != 0.0))) && (!(scratch.values[1786] != 0.0))) {
            scratch.store_ad(1356, &AdValue::div_from_scalar(1e-100, AdValue::offset(AdValue::mul(AdValue::sub_from_scalar((-230.25850929940458), scratch.ad_value(1376)), AdValue::offset(AdValue::scale(AdValue::mul(AdValue::sub_from_scalar((-230.25850929940458), scratch.ad_value(1376)), AdValue::offset(AdValue::scale(AdValue::sub_from_scalar((-230.25850929940458), scratch.ad_value(1376)), 0.3333333333333333), 1.0)), 0.5), 1.0)), 1.0)));
        }

        if (((((scratch.values[1320] != 0.0) && (scratch.values[1337] != 0.0)) && (!(scratch.values[1776] != 0.0))) && (!(scratch.values[1781] != 0.0))) && (!(scratch.values[1785] != 0.0))) {
            scratch.store_ad(1378, &AdValue::sub(AdValue::scale(scratch.ad_value(1356), 2.0), scratch.ad_value(1340)));
        }

        if ((((scratch.values[1320] != 0.0) && (scratch.values[1337] != 0.0)) && (!(scratch.values[1776] != 0.0))) && (!(scratch.values[1781] != 0.0))) {
            scratch.store_ad(1379, &AdValue::scale(AdValue::div(AdValue::mul(scratch.ad_value(635), scratch.ad_value(1378)), scratch.ad_value(1374)), (1.772453850905516 * 0.5)));
        }

        if ((((scratch.values[1320] != 0.0) && (scratch.values[1337] != 0.0)) && (!(scratch.values[1776] != 0.0))) && (!(scratch.values[1781] != 0.0))) {
            scratch.store_ad(1365, &AdValue::mul(scratch.ad_value(564), AdValue::mul(AdValue::mul(scratch.ad_value(1364), scratch.ad_value(1379)), scratch.ad_value(1373))));
        }

        scratch.values[1787] = if (scratch.values[570] == 0.0) { 1.0 } else { 0.0 };

        if ((((scratch.values[1320] != 0.0) && (scratch.values[1337] != 0.0)) && (!(scratch.values[1776] != 0.0))) && (scratch.values[1787] != 0.0)) {
            scratch.values[1380] = 0.0;
            scratch.node_derivatives[1380] = [0.0; Instance::NODE_COUNT];
            scratch.branch_derivatives[1380] = [0.0; Instance::BRANCH_COUNT];
        }

        scratch.values[1788] = if (scratch.values[550] == 0.5) { 1.0 } else { 0.0 };

        if (((((scratch.values[1320] != 0.0) && (scratch.values[1337] != 0.0)) && (!(scratch.values[1776] != 0.0))) && (!(scratch.values[1787] != 0.0))) && (scratch.values[1788] != 0.0)) {
            scratch.store_ad(1356, &AdValue::sqrt(AdValue::mul(AdValue::sub(scratch.ad_value(547), scratch.ad_value(1354)), scratch.ad_value(629))));
        }

        if (((((scratch.values[1320] != 0.0) && (scratch.values[1337] != 0.0)) && (!(scratch.values[1776] != 0.0))) && (!(scratch.values[1787] != 0.0))) && (!(scratch.values[1788] != 0.0))) {
            scratch.store_ad(1356, &AdValue::pow(AdValue::mul(AdValue::sub(scratch.ad_value(547), scratch.ad_value(1354)), scratch.ad_value(629)), scratch.ad_value(550)));
        }

        if ((((scratch.values[1320] != 0.0) && (scratch.values[1337] != 0.0)) && (!(scratch.values[1776] != 0.0))) && (!(scratch.values[1787] != 0.0))) {
            scratch.store_ad(1381, &AdValue::mul(scratch.ad_value(611), AdValue::div(AdValue::mul(AdValue::sub(scratch.ad_value(547), scratch.ad_value(1354)), scratch.ad_value(626)), scratch.ad_value(1356))));
        }

        scratch.values[1789] = if (((((-scratch.values[641]) / scratch.values[1381])) as f64).abs() < 230.25850929940458) { 1.0 } else { 0.0 };

        if (((((scratch.values[1320] != 0.0) && (scratch.values[1337] != 0.0)) && (!(scratch.values[1776] != 0.0))) && (!(scratch.values[1787] != 0.0))) && (scratch.values[1789] != 0.0)) {
            scratch.store_ad(1356, &AdValue::exp(AdValue::div(AdValue::neg(scratch.ad_value(641)), scratch.ad_value(1381))));
        }

        scratch.values[1790] = if (((-scratch.values[641]) / scratch.values[1381]) < 0.0) { 1.0 } else { 0.0 };

        if ((((((scratch.values[1320] != 0.0) && (scratch.values[1337] != 0.0)) && (!(scratch.values[1776] != 0.0))) && (!(scratch.values[1787] != 0.0))) && (!(scratch.values[1789] != 0.0))) && (scratch.values[1790] != 0.0)) {
            let assign30290_ad_e37709: AdValue = AdValue::offset(AdValue::mul(AdValue::sub_from_scalar((-230.25850929940458), AdValue::div(AdValue::neg(scratch.ad_value(641)), scratch.ad_value(1381))), AdValue::offset(AdValue::scale(AdValue::mul(AdValue::sub_from_scalar((-230.25850929940458), AdValue::div(AdValue::neg(scratch.ad_value(641)), scratch.ad_value(1381))), AdValue::offset(AdValue::scale(AdValue::sub_from_scalar((-230.25850929940458), AdValue::div(AdValue::neg(scratch.ad_value(641)), scratch.ad_value(1381))), 0.3333333333333333), 1.0)), 0.5), 1.0)), 1.0);
            scratch.store_ad(1356, &AdValue::div_from_scalar(1e-100, assign30290_ad_e37709));
        }

        if ((((((scratch.values[1320] != 0.0) && (scratch.values[1337] != 0.0)) && (!(scratch.values[1776] != 0.0))) && (!(scratch.values[1787] != 0.0))) && (!(scratch.values[1789] != 0.0))) && (!(scratch.values[1790] != 0.0))) {
            let assign30300_ad_e37759: AdValue = AdValue::scale(AdValue::offset(AdValue::mul(AdValue::offset(AdValue::div(AdValue::neg(scratch.ad_value(641)), scratch.ad_value(1381)), (-230.25850929940458)), AdValue::offset(AdValue::scale(AdValue::mul(AdValue::offset(AdValue::div(AdValue::neg(scratch.ad_value(641)), scratch.ad_value(1381)), (-230.25850929940458)), AdValue::offset(AdValue::scale(AdValue::offset(AdValue::div(AdValue::neg(scratch.ad_value(641)), scratch.ad_value(1381)), (-230.25850929940458)), 0.3333333333333333), 1.0)), 0.5), 1.0)), 1.0), 1e100);
            scratch.store_ad(1356, &assign30300_ad_e37759);
        }

        if ((((scratch.values[1320] != 0.0) && (scratch.values[1337] != 0.0)) && (!(scratch.values[1776] != 0.0))) && (!(scratch.values[1787] != 0.0))) {
            scratch.store_ad(1380, &AdValue::mul(scratch.ad_value(570), AdValue::mul(AdValue::mul(AdValue::mul(scratch.ad_value(524), scratch.ad_value(1381)), scratch.ad_value(1381)), scratch.ad_value(1356))));
        }

        scratch.values[1791] = if (scratch.values[579] > 1000.0) { 1.0 } else { 0.0 };

        if ((((scratch.values[1320] != 0.0) && (scratch.values[1337] != 0.0)) && (!(scratch.values[1776] != 0.0))) && (scratch.values[1791] != 0.0)) {
            scratch.values[1382] = 1.0;
            scratch.node_derivatives[1382] = [0.0; Instance::NODE_COUNT];
            scratch.branch_derivatives[1382] = [0.0; Instance::BRANCH_COUNT];
        }

        scratch.values[1792] = if (scratch.values[1355] > ((-scratch.values[500]) * scratch.values[579])) { 1.0 } else { 0.0 };

        scratch.values[1793] = if (scratch.values[582] == 4.0) { 1.0 } else { 0.0 };

        if ((((((scratch.values[1320] != 0.0) && (scratch.values[1337] != 0.0)) && (!(scratch.values[1776] != 0.0))) && (!(scratch.values[1791] != 0.0))) && (scratch.values[1792] != 0.0)) && (scratch.values[1793] != 0.0)) {
            scratch.store_ad(1356, &AdValue::mul(AdValue::mul(AdValue::mul(AdValue::mul(scratch.ad_value(1355), scratch.ad_value(647)), AdValue::mul(scratch.ad_value(1355), scratch.ad_value(647))), AdValue::mul(scratch.ad_value(1355), scratch.ad_value(647))), AdValue::mul(scratch.ad_value(1355), scratch.ad_value(647))));
        }

        if ((((((scratch.values[1320] != 0.0) && (scratch.values[1337] != 0.0)) && (!(scratch.values[1776] != 0.0))) && (!(scratch.values[1791] != 0.0))) && (scratch.values[1792] != 0.0)) && (!(scratch.values[1793] != 0.0))) {
            scratch.store_ad(1356, &AdValue::pow(AdValue::abs(AdValue::mul(scratch.ad_value(1355), scratch.ad_value(647))), scratch.ad_value(582)));
        }

        if (((((scratch.values[1320] != 0.0) && (scratch.values[1337] != 0.0)) && (!(scratch.values[1776] != 0.0))) && (!(scratch.values[1791] != 0.0))) && (scratch.values[1792] != 0.0)) {
            scratch.store_ad(1382, &AdValue::div_from_scalar(1.0, AdValue::sub_from_scalar(1.0, scratch.ad_value(1356))));
        }

        if (((((scratch.values[1320] != 0.0) && (scratch.values[1337] != 0.0)) && (!(scratch.values[1776] != 0.0))) && (!(scratch.values[1791] != 0.0))) && (!(scratch.values[1792] != 0.0))) {
            scratch.store_ad(1382, &AdValue::add(scratch.ad_value(644), AdValue::mul(AdValue::add(scratch.ad_value(1355), AdValue::scale(scratch.ad_value(579), scratch.values[500])), scratch.ad_value(650))));
        }

        if (((scratch.values[1320] != 0.0) && (scratch.values[1337] != 0.0)) && (!(scratch.values[1776] != 0.0))) {
            scratch.store_ad(1384, &AdValue::mul(AdValue::add(AdValue::add(AdValue::add(scratch.ad_value(1357), scratch.ad_value(1358)), scratch.ad_value(1365)), scratch.ad_value(1380)), scratch.ad_value(1382)));
        }

        scratch.values[1794] = if (scratch.values[726] == 0.0) { 1.0 } else { 0.0 };

        if (((scratch.values[1320] != 0.0) && (scratch.values[1337] != 0.0)) && (scratch.values[1794] != 0.0)) {
            scratch.values[1385] = 0.0;
            scratch.node_derivatives[1385] = [0.0; Instance::NODE_COUNT];
            scratch.branch_derivatives[1385] = [0.0; Instance::BRANCH_COUNT];
        }

        scratch.values[1795] = if (scratch.values[609] == 0.5) { 1.0 } else { 0.0 };

        if ((((scratch.values[1320] != 0.0) && (scratch.values[1337] != 0.0)) && (!(scratch.values[1794] != 0.0))) && (scratch.values[1795] != 0.0)) {
            scratch.store_ad(1356, &AdValue::sqrt(AdValue::sub_from_scalar(1.0, AdValue::mul(scratch.ad_value(1348), scratch.ad_value(606)))));
        }

        if ((((scratch.values[1320] != 0.0) && (scratch.values[1337] != 0.0)) && (!(scratch.values[1794] != 0.0))) && (!(scratch.values[1795] != 0.0))) {
            scratch.store_ad(1356, &AdValue::pow(AdValue::sub_from_scalar(1.0, AdValue::mul(scratch.ad_value(1348), scratch.ad_value(606))), scratch.ad_value(609)));
        }

        if (((scratch.values[1320] != 0.0) && (scratch.values[1337] != 0.0)) && (!(scratch.values[1794] != 0.0))) {
            scratch.store_ad(1357, &AdValue::mul(scratch.ad_value(597), scratch.ad_value(1347)));
        }

        scratch.values[1796] = if ((scratch.values[562] == 0.0) && (scratch.values[565] == 0.0)) { 1.0 } else { 0.0 };

        if ((((scratch.values[1320] != 0.0) && (scratch.values[1337] != 0.0)) && (!(scratch.values[1794] != 0.0))) && (scratch.values[1796] != 0.0)) {
            scratch.values[1358] = 0.0;
            scratch.node_derivatives[1358] = [0.0; Instance::NODE_COUNT];
            scratch.branch_derivatives[1358] = [0.0; Instance::BRANCH_COUNT];
        }

        if ((((scratch.values[1320] != 0.0) && (scratch.values[1337] != 0.0)) && (!(scratch.values[1794] != 0.0))) && (!(scratch.values[1796] != 0.0))) {
            scratch.store_ad(1359, &AdValue::sub(scratch.ad_value(603), scratch.ad_value(1353)));
        }

        if ((((scratch.values[1320] != 0.0) && (scratch.values[1337] != 0.0)) && (!(scratch.values[1794] != 0.0))) && (!(scratch.values[1796] != 0.0))) {
            scratch.store_ad(1360, &AdValue::sub_from_scalar(1.0, AdValue::sqrt(AdValue::sub_from_scalar(1.0, AdValue::div(scratch.ad_value(1351), scratch.ad_value(1359))))));
        }

        scratch.values[1797] = if (scratch.values[551] == 0.5) { 1.0 } else { 0.0 };

        if (((((scratch.values[1320] != 0.0) && (scratch.values[1337] != 0.0)) && (!(scratch.values[1794] != 0.0))) && (!(scratch.values[1796] != 0.0))) && (scratch.values[1797] != 0.0)) {
            scratch.values[1361] = 0.0;
            scratch.node_derivatives[1361] = [0.0; Instance::NODE_COUNT];
            scratch.branch_derivatives[1361] = [0.0; Instance::BRANCH_COUNT];
        }

        if (((((scratch.values[1320] != 0.0) && (scratch.values[1337] != 0.0)) && (!(scratch.values[1794] != 0.0))) && (!(scratch.values[1796] != 0.0))) && (!(scratch.values[1797] != 0.0))) {
            scratch.store_ad(1361, &AdValue::mul(AdValue::add(AdValue::div(AdValue::mul(AdValue::square(scratch.ad_value(1360)), AdValue::ln(scratch.ad_value(1360))), AdValue::sub_from_scalar(1.0, scratch.ad_value(1360))), scratch.ad_value(1360)), AdValue::sub_from_scalar(1.0, AdValue::scale(scratch.ad_value(551), 2.0))));
        }

        if ((((scratch.values[1320] != 0.0) && (scratch.values[1337] != 0.0)) && (!(scratch.values[1794] != 0.0))) && (!(scratch.values[1796] != 0.0))) {
            scratch.store_ad(1362, &AdValue::add(scratch.ad_value(1360), scratch.ad_value(1361)));
        }

        scratch.values[1798] = if (scratch.values[551] == 0.5) { 1.0 } else { 0.0 };

        if (((((scratch.values[1320] != 0.0) && (scratch.values[1337] != 0.0)) && (!(scratch.values[1794] != 0.0))) && (!(scratch.values[1796] != 0.0))) && (scratch.values[1798] != 0.0)) {
            scratch.store_ad(1356, &AdValue::sqrt(AdValue::mul(scratch.ad_value(1359), scratch.ad_value(630))));
        }

        if (((((scratch.values[1320] != 0.0) && (scratch.values[1337] != 0.0)) && (!(scratch.values[1794] != 0.0))) && (!(scratch.values[1796] != 0.0))) && (!(scratch.values[1798] != 0.0))) {
            scratch.store_ad(1356, &AdValue::pow(AdValue::mul(scratch.ad_value(1359), scratch.ad_value(630)), scratch.ad_value(551)));
        }

        if ((((scratch.values[1320] != 0.0) && (scratch.values[1337] != 0.0)) && (!(scratch.values[1794] != 0.0))) && (!(scratch.values[1796] != 0.0))) {
            scratch.store_ad(1363, &AdValue::mul(scratch.ad_value(624), scratch.ad_value(1356)));
        }

        if ((((scratch.values[1320] != 0.0) && (scratch.values[1337] != 0.0)) && (!(scratch.values[1794] != 0.0))) && (!(scratch.values[1796] != 0.0))) {
            scratch.store_ad(1364, &AdValue::mul(scratch.ad_value(594), AdValue::mul(AdValue::offset(scratch.ad_value(1350), (-1.0)), scratch.ad_value(1363))));
        }

        if ((((scratch.values[1320] != 0.0) && (scratch.values[1337] != 0.0)) && (!(scratch.values[1794] != 0.0))) && (!(scratch.values[1796] != 0.0))) {
            scratch.store_ad(1358, &AdValue::mul(scratch.ad_value(562), AdValue::mul(scratch.ad_value(1364), scratch.ad_value(1362))));
        }

        scratch.values[1799] = if (scratch.values[565] == 0.0) { 1.0 } else { 0.0 };

        if ((((scratch.values[1320] != 0.0) && (scratch.values[1337] != 0.0)) && (!(scratch.values[1794] != 0.0))) && (scratch.values[1799] != 0.0)) {
            scratch.values[1365] = 0.0;
            scratch.node_derivatives[1365] = [0.0; Instance::NODE_COUNT];
            scratch.branch_derivatives[1365] = [0.0; Instance::BRANCH_COUNT];
        }

        if ((((scratch.values[1320] != 0.0) && (scratch.values[1337] != 0.0)) && (!(scratch.values[1794] != 0.0))) && (!(scratch.values[1799] != 0.0))) {
            scratch.store_ad(1366, &AdValue::mul(scratch.ad_value(639), AdValue::div(AdValue::mul(scratch.ad_value(1363), scratch.ad_value(609)), scratch.ad_value(1359))));
        }

        if ((((scratch.values[1320] != 0.0) && (scratch.values[1337] != 0.0)) && (!(scratch.values[1794] != 0.0))) && (!(scratch.values[1799] != 0.0))) {
            scratch.store_ad(1367, &AdValue::div(AdValue::scale(scratch.ad_value(636), 0.666666666666667), scratch.ad_value(1366)));
        }

        if ((((scratch.values[1320] != 0.0) && (scratch.values[1337] != 0.0)) && (!(scratch.values[1794] != 0.0))) && (!(scratch.values[1799] != 0.0))) {
            scratch.store_ad(1368, &AdValue::square(scratch.ad_value(1367)));
        }

        if ((((scratch.values[1320] != 0.0) && (scratch.values[1337] != 0.0)) && (!(scratch.values[1794] != 0.0))) && (!(scratch.values[1799] != 0.0))) {
            scratch.store_ad(1369, &AdValue::sqrt(AdValue::div(AdValue::square(scratch.ad_value(1368)), AdValue::offset(AdValue::square(scratch.ad_value(1368)), 1.0))));
        }

        if ((((scratch.values[1320] != 0.0) && (scratch.values[1337] != 0.0)) && (!(scratch.values[1794] != 0.0))) && (!(scratch.values[1799] != 0.0))) {
            scratch.store_ad(1370, &AdValue::sqrt(AdValue::abs(scratch.ad_value(1369))));
        }

        if ((((scratch.values[1320] != 0.0) && (scratch.values[1337] != 0.0)) && (!(scratch.values[1794] != 0.0))) && (!(scratch.values[1799] != 0.0))) {
            scratch.store_ad(1371, &AdValue::mul(scratch.ad_value(1369), scratch.ad_value(1370)));
        }

        scratch.values[1800] = if (((-scratch.values[551]) * scratch.values[612]) == (-1.0)) { 1.0 } else { 0.0 };

        if (((((scratch.values[1320] != 0.0) && (scratch.values[1337] != 0.0)) && (!(scratch.values[1794] != 0.0))) && (!(scratch.values[1799] != 0.0))) && (scratch.values[1800] != 0.0)) {
            scratch.store_ad(1372, &AdValue::div_from_scalar(1.0, AdValue::offset(AdValue::mul(scratch.ad_value(1366), scratch.ad_value(1371)), 1.0)));
        }

        if (((((scratch.values[1320] != 0.0) && (scratch.values[1337] != 0.0)) && (!(scratch.values[1794] != 0.0))) && (!(scratch.values[1799] != 0.0))) && (!(scratch.values[1800] != 0.0))) {
            scratch.store_ad(1372, &AdValue::pow(AdValue::offset(AdValue::mul(scratch.ad_value(1366), scratch.ad_value(1371)), 1.0), AdValue::mul(AdValue::neg(scratch.ad_value(551)), scratch.ad_value(612))));
        }

        if ((((scratch.values[1320] != 0.0) && (scratch.values[1337] != 0.0)) && (!(scratch.values[1794] != 0.0))) && (!(scratch.values[1799] != 0.0))) {
            scratch.store_ad(1373, &AdValue::div(AdValue::mul(scratch.ad_value(1362), scratch.ad_value(1372)), AdValue::add(scratch.ad_value(1362), scratch.ad_value(1372))));
        }

        if ((((scratch.values[1320] != 0.0) && (scratch.values[1337] != 0.0)) && (!(scratch.values[1794] != 0.0))) && (!(scratch.values[1799] != 0.0))) {
            scratch.store_ad(1374, &AdValue::sqrt(AdValue::scale(AdValue::div(scratch.ad_value(1366), scratch.ad_value(1370)), 0.375)));
        }

        if ((((scratch.values[1320] != 0.0) && (scratch.values[1337] != 0.0)) && (!(scratch.values[1794] != 0.0))) && (!(scratch.values[1799] != 0.0))) {
            scratch.store_ad(1375, &AdValue::sub(AdValue::scale(AdValue::mul(scratch.ad_value(1367), scratch.ad_value(1370)), 2.0), scratch.ad_value(1369)));
        }

        if ((((scratch.values[1320] != 0.0) && (scratch.values[1337] != 0.0)) && (!(scratch.values[1794] != 0.0))) && (!(scratch.values[1799] != 0.0))) {
            scratch.store_ad(1376, &AdValue::add(AdValue::sub(AdValue::mul(AdValue::mul(scratch.ad_value(636), scratch.ad_value(1367)), scratch.ad_value(1370)), AdValue::mul(scratch.ad_value(636), scratch.ad_value(1369))), AdValue::scale(AdValue::mul(scratch.ad_value(1366), scratch.ad_value(1371)), 0.5)));
        }

        if ((((scratch.values[1320] != 0.0) && (scratch.values[1337] != 0.0)) && (!(scratch.values[1794] != 0.0))) && (!(scratch.values[1799] != 0.0))) {
            scratch.store_ad(1377, &AdValue::mul(AdValue::offset(scratch.ad_value(1375), (-1.0)), scratch.ad_value(1374)));
        }

        if ((((scratch.values[1320] != 0.0) && (scratch.values[1337] != 0.0)) && (!(scratch.values[1794] != 0.0))) && (!(scratch.values[1799] != 0.0))) {
            scratch.store_ad(1338, &AdValue::square(scratch.ad_value(1377)));
        }

        scratch.values[1801] = if (scratch.values[1377] > 0.0) { 1.0 } else { 0.0 };

        if (((((scratch.values[1320] != 0.0) && (scratch.values[1337] != 0.0)) && (!(scratch.values[1794] != 0.0))) && (!(scratch.values[1799] != 0.0))) && (scratch.values[1801] != 0.0)) {
            scratch.store_ad(1339, &AdValue::div_from_scalar(1.0, AdValue::offset(AdValue::scale(scratch.ad_value(1377), scratch.values[428]), 1.0)));
        }

        if (((((scratch.values[1320] != 0.0) && (scratch.values[1337] != 0.0)) && (!(scratch.values[1794] != 0.0))) && (!(scratch.values[1799] != 0.0))) && (!(scratch.values[1801] != 0.0))) {
            scratch.store_ad(1339, &AdValue::div_from_scalar(1.0, AdValue::sub_from_scalar(1.0, AdValue::scale(scratch.ad_value(1377), scratch.values[428]))));
        }

        scratch.values[1802] = if (((-scratch.values[1338]) + scratch.values[1376]) > (-230.25850929940458)) { 1.0 } else { 0.0 };

        if (((((scratch.values[1320] != 0.0) && (scratch.values[1337] != 0.0)) && (!(scratch.values[1794] != 0.0))) && (!(scratch.values[1799] != 0.0))) && (scratch.values[1802] != 0.0)) {
            scratch.store_ad(1356, &AdValue::exp(AdValue::sub(scratch.ad_value(1376), scratch.ad_value(1338))));
        }

        if (((((scratch.values[1320] != 0.0) && (scratch.values[1337] != 0.0)) && (!(scratch.values[1794] != 0.0))) && (!(scratch.values[1799] != 0.0))) && (!(scratch.values[1802] != 0.0))) {
            let assign30850_ad_e38588: AdValue = AdValue::div_from_scalar(1e-100, AdValue::offset(AdValue::mul(AdValue::sub_from_scalar((-230.25850929940458), AdValue::sub(scratch.ad_value(1376), scratch.ad_value(1338))), AdValue::offset(AdValue::scale(AdValue::mul(AdValue::sub_from_scalar((-230.25850929940458), AdValue::sub(scratch.ad_value(1376), scratch.ad_value(1338))), AdValue::offset(AdValue::scale(AdValue::sub_from_scalar((-230.25850929940458), AdValue::sub(scratch.ad_value(1376), scratch.ad_value(1338))), 0.3333333333333333), 1.0)), 0.5), 1.0)), 1.0));
            scratch.store_ad(1356, &assign30850_ad_e38588);
        }

        if ((((scratch.values[1320] != 0.0) && (scratch.values[1337] != 0.0)) && (!(scratch.values[1794] != 0.0))) && (!(scratch.values[1799] != 0.0))) {
            scratch.store_ad(1340, &AdValue::mul(AdValue::add(AdValue::add(AdValue::scale(scratch.ad_value(1339), 0.29214664), AdValue::scale(AdValue::square(scratch.ad_value(1339)), scratch.values[429])), AdValue::scale(AdValue::mul(AdValue::square(scratch.ad_value(1339)), scratch.ad_value(1339)), scratch.values[430])), scratch.ad_value(1356)));
        }

        scratch.values[1803] = if (scratch.values[1377] > 0.0) { 1.0 } else { 0.0 };

        if (((((scratch.values[1320] != 0.0) && (scratch.values[1337] != 0.0)) && (!(scratch.values[1794] != 0.0))) && (!(scratch.values[1799] != 0.0))) && (scratch.values[1803] != 0.0)) {
            scratch.values[1378] = scratch.values[1340];
            scratch.node_derivatives[1378] = scratch.node_derivatives[1340];
            scratch.branch_derivatives[1378] = scratch.branch_derivatives[1340];
        }

        scratch.values[1804] = if (scratch.values[1376] > (-230.25850929940458)) { 1.0 } else { 0.0 };

        if ((((((scratch.values[1320] != 0.0) && (scratch.values[1337] != 0.0)) && (!(scratch.values[1794] != 0.0))) && (!(scratch.values[1799] != 0.0))) && (!(scratch.values[1803] != 0.0))) && (scratch.values[1804] != 0.0)) {
            scratch.store_ad(1356, &AdValue::exp(scratch.ad_value(1376)));
        }

        if ((((((scratch.values[1320] != 0.0) && (scratch.values[1337] != 0.0)) && (!(scratch.values[1794] != 0.0))) && (!(scratch.values[1799] != 0.0))) && (!(scratch.values[1803] != 0.0))) && (!(scratch.values[1804] != 0.0))) {
            scratch.store_ad(1356, &AdValue::div_from_scalar(1e-100, AdValue::offset(AdValue::mul(AdValue::sub_from_scalar((-230.25850929940458), scratch.ad_value(1376)), AdValue::offset(AdValue::scale(AdValue::mul(AdValue::sub_from_scalar((-230.25850929940458), scratch.ad_value(1376)), AdValue::offset(AdValue::scale(AdValue::sub_from_scalar((-230.25850929940458), scratch.ad_value(1376)), 0.3333333333333333), 1.0)), 0.5), 1.0)), 1.0)));
        }

        if (((((scratch.values[1320] != 0.0) && (scratch.values[1337] != 0.0)) && (!(scratch.values[1794] != 0.0))) && (!(scratch.values[1799] != 0.0))) && (!(scratch.values[1803] != 0.0))) {
            scratch.store_ad(1378, &AdValue::sub(AdValue::scale(scratch.ad_value(1356), 2.0), scratch.ad_value(1340)));
        }

        if ((((scratch.values[1320] != 0.0) && (scratch.values[1337] != 0.0)) && (!(scratch.values[1794] != 0.0))) && (!(scratch.values[1799] != 0.0))) {
            scratch.store_ad(1379, &AdValue::scale(AdValue::div(AdValue::mul(scratch.ad_value(636), scratch.ad_value(1378)), scratch.ad_value(1374)), (1.772453850905516 * 0.5)));
        }

        if ((((scratch.values[1320] != 0.0) && (scratch.values[1337] != 0.0)) && (!(scratch.values[1794] != 0.0))) && (!(scratch.values[1799] != 0.0))) {
            scratch.store_ad(1365, &AdValue::mul(scratch.ad_value(565), AdValue::mul(AdValue::mul(scratch.ad_value(1364), scratch.ad_value(1379)), scratch.ad_value(1373))));
        }

        scratch.values[1805] = if (scratch.values[571] == 0.0) { 1.0 } else { 0.0 };

        if ((((scratch.values[1320] != 0.0) && (scratch.values[1337] != 0.0)) && (!(scratch.values[1794] != 0.0))) && (scratch.values[1805] != 0.0)) {
            scratch.values[1380] = 0.0;
            scratch.node_derivatives[1380] = [0.0; Instance::NODE_COUNT];
            scratch.branch_derivatives[1380] = [0.0; Instance::BRANCH_COUNT];
        }

        scratch.values[1806] = if (scratch.values[551] == 0.5) { 1.0 } else { 0.0 };

        if (((((scratch.values[1320] != 0.0) && (scratch.values[1337] != 0.0)) && (!(scratch.values[1794] != 0.0))) && (!(scratch.values[1805] != 0.0))) && (scratch.values[1806] != 0.0)) {
            scratch.store_ad(1356, &AdValue::sqrt(AdValue::mul(AdValue::sub(scratch.ad_value(548), scratch.ad_value(1354)), scratch.ad_value(630))));
        }

        if (((((scratch.values[1320] != 0.0) && (scratch.values[1337] != 0.0)) && (!(scratch.values[1794] != 0.0))) && (!(scratch.values[1805] != 0.0))) && (!(scratch.values[1806] != 0.0))) {
            scratch.store_ad(1356, &AdValue::pow(AdValue::mul(AdValue::sub(scratch.ad_value(548), scratch.ad_value(1354)), scratch.ad_value(630)), scratch.ad_value(551)));
        }

        if ((((scratch.values[1320] != 0.0) && (scratch.values[1337] != 0.0)) && (!(scratch.values[1794] != 0.0))) && (!(scratch.values[1805] != 0.0))) {
            scratch.store_ad(1381, &AdValue::mul(scratch.ad_value(612), AdValue::div(AdValue::mul(AdValue::sub(scratch.ad_value(548), scratch.ad_value(1354)), scratch.ad_value(627)), scratch.ad_value(1356))));
        }

        scratch.values[1807] = if (((((-scratch.values[642]) / scratch.values[1381])) as f64).abs() < 230.25850929940458) { 1.0 } else { 0.0 };

        if (((((scratch.values[1320] != 0.0) && (scratch.values[1337] != 0.0)) && (!(scratch.values[1794] != 0.0))) && (!(scratch.values[1805] != 0.0))) && (scratch.values[1807] != 0.0)) {
            scratch.store_ad(1356, &AdValue::exp(AdValue::div(AdValue::neg(scratch.ad_value(642)), scratch.ad_value(1381))));
        }

        scratch.values[1808] = if (((-scratch.values[642]) / scratch.values[1381]) < 0.0) { 1.0 } else { 0.0 };

        if ((((((scratch.values[1320] != 0.0) && (scratch.values[1337] != 0.0)) && (!(scratch.values[1794] != 0.0))) && (!(scratch.values[1805] != 0.0))) && (!(scratch.values[1807] != 0.0))) && (scratch.values[1808] != 0.0)) {
            let assign31040_ad_e38915: AdValue = AdValue::offset(AdValue::mul(AdValue::sub_from_scalar((-230.25850929940458), AdValue::div(AdValue::neg(scratch.ad_value(642)), scratch.ad_value(1381))), AdValue::offset(AdValue::scale(AdValue::mul(AdValue::sub_from_scalar((-230.25850929940458), AdValue::div(AdValue::neg(scratch.ad_value(642)), scratch.ad_value(1381))), AdValue::offset(AdValue::scale(AdValue::sub_from_scalar((-230.25850929940458), AdValue::div(AdValue::neg(scratch.ad_value(642)), scratch.ad_value(1381))), 0.3333333333333333), 1.0)), 0.5), 1.0)), 1.0);
            scratch.store_ad(1356, &AdValue::div_from_scalar(1e-100, assign31040_ad_e38915));
        }

        if ((((((scratch.values[1320] != 0.0) && (scratch.values[1337] != 0.0)) && (!(scratch.values[1794] != 0.0))) && (!(scratch.values[1805] != 0.0))) && (!(scratch.values[1807] != 0.0))) && (!(scratch.values[1808] != 0.0))) {
            let assign31050_ad_e38965: AdValue = AdValue::scale(AdValue::offset(AdValue::mul(AdValue::offset(AdValue::div(AdValue::neg(scratch.ad_value(642)), scratch.ad_value(1381)), (-230.25850929940458)), AdValue::offset(AdValue::scale(AdValue::mul(AdValue::offset(AdValue::div(AdValue::neg(scratch.ad_value(642)), scratch.ad_value(1381)), (-230.25850929940458)), AdValue::offset(AdValue::scale(AdValue::offset(AdValue::div(AdValue::neg(scratch.ad_value(642)), scratch.ad_value(1381)), (-230.25850929940458)), 0.3333333333333333), 1.0)), 0.5), 1.0)), 1.0), 1e100);
            scratch.store_ad(1356, &assign31050_ad_e38965);
        }

        if ((((scratch.values[1320] != 0.0) && (scratch.values[1337] != 0.0)) && (!(scratch.values[1794] != 0.0))) && (!(scratch.values[1805] != 0.0))) {
            scratch.store_ad(1380, &AdValue::mul(scratch.ad_value(571), AdValue::mul(AdValue::mul(AdValue::mul(scratch.ad_value(524), scratch.ad_value(1381)), scratch.ad_value(1381)), scratch.ad_value(1356))));
        }

        scratch.values[1809] = if (scratch.values[580] > 1000.0) { 1.0 } else { 0.0 };

        if ((((scratch.values[1320] != 0.0) && (scratch.values[1337] != 0.0)) && (!(scratch.values[1794] != 0.0))) && (scratch.values[1809] != 0.0)) {
            scratch.values[1382] = 1.0;
            scratch.node_derivatives[1382] = [0.0; Instance::NODE_COUNT];
            scratch.branch_derivatives[1382] = [0.0; Instance::BRANCH_COUNT];
        }

        scratch.values[1810] = if (scratch.values[1355] > ((-scratch.values[500]) * scratch.values[580])) { 1.0 } else { 0.0 };

        scratch.values[1811] = if (scratch.values[583] == 4.0) { 1.0 } else { 0.0 };

        if ((((((scratch.values[1320] != 0.0) && (scratch.values[1337] != 0.0)) && (!(scratch.values[1794] != 0.0))) && (!(scratch.values[1809] != 0.0))) && (scratch.values[1810] != 0.0)) && (scratch.values[1811] != 0.0)) {
            scratch.store_ad(1356, &AdValue::mul(AdValue::mul(AdValue::mul(AdValue::mul(scratch.ad_value(1355), scratch.ad_value(648)), AdValue::mul(scratch.ad_value(1355), scratch.ad_value(648))), AdValue::mul(scratch.ad_value(1355), scratch.ad_value(648))), AdValue::mul(scratch.ad_value(1355), scratch.ad_value(648))));
        }

        if ((((((scratch.values[1320] != 0.0) && (scratch.values[1337] != 0.0)) && (!(scratch.values[1794] != 0.0))) && (!(scratch.values[1809] != 0.0))) && (scratch.values[1810] != 0.0)) && (!(scratch.values[1811] != 0.0))) {
            scratch.store_ad(1356, &AdValue::pow(AdValue::abs(AdValue::mul(scratch.ad_value(1355), scratch.ad_value(648))), scratch.ad_value(583)));
        }

        if (((((scratch.values[1320] != 0.0) && (scratch.values[1337] != 0.0)) && (!(scratch.values[1794] != 0.0))) && (!(scratch.values[1809] != 0.0))) && (scratch.values[1810] != 0.0)) {
            scratch.store_ad(1382, &AdValue::div_from_scalar(1.0, AdValue::sub_from_scalar(1.0, scratch.ad_value(1356))));
        }

        if (((((scratch.values[1320] != 0.0) && (scratch.values[1337] != 0.0)) && (!(scratch.values[1794] != 0.0))) && (!(scratch.values[1809] != 0.0))) && (!(scratch.values[1810] != 0.0))) {
            scratch.store_ad(1382, &AdValue::add(scratch.ad_value(645), AdValue::mul(AdValue::add(scratch.ad_value(1355), AdValue::scale(scratch.ad_value(580), scratch.values[500])), scratch.ad_value(651))));
        }

        if (((scratch.values[1320] != 0.0) && (scratch.values[1337] != 0.0)) && (!(scratch.values[1794] != 0.0))) {
            scratch.store_ad(1385, &AdValue::mul(AdValue::add(AdValue::add(AdValue::add(scratch.ad_value(1357), scratch.ad_value(1358)), scratch.ad_value(1365)), scratch.ad_value(1380)), scratch.ad_value(1382)));
        }

        if ((scratch.values[1320] != 0.0) && (scratch.values[1337] != 0.0)) {
            scratch.store_ad(514, &AdValue::add(AdValue::add(AdValue::mul(scratch.ad_value(724), scratch.ad_value(1383)), AdValue::mul(scratch.ad_value(725), scratch.ad_value(1384))), AdValue::mul(scratch.ad_value(726), scratch.ad_value(1385))));
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

        scratch.values[1812] = if !(((scratch.values[724] == 0.0) && (scratch.values[725] == 0.0)) && (scratch.values[726] == 0.0)) { 1.0 } else { 0.0 };

        if (((scratch.values[1320] != 0.0) && (scratch.values[1337] != 0.0)) && (scratch.values[1812] != 0.0)) {
            scratch.store_ad(1341, &AdValue::mul(AdValue::scale(scratch.ad_value(735), 4.0), scratch.ad_value(735)));
        }

        if (((scratch.values[1320] != 0.0) && (scratch.values[1337] != 0.0)) && (scratch.values[1812] != 0.0)) {
            scratch.store_ad(1342, &AdValue::div(scratch.ad_value(735), scratch.ad_value(736)));
        }

        if (((scratch.values[1320] != 0.0) && (scratch.values[1337] != 0.0)) && (scratch.values[1812] != 0.0)) {
            scratch.store_ad(1343, &AdValue::add(scratch.ad_value(525), AdValue::mul(scratch.ad_value(735), scratch.ad_value(1342))));
        }

        if (((scratch.values[1320] != 0.0) && (scratch.values[1337] != 0.0)) && (scratch.values[1812] != 0.0)) {
            scratch.store_ad(1344, &AdValue::add(scratch.ad_value(736), scratch.ad_value(1343)));
        }

        if (((scratch.values[1320] != 0.0) && (scratch.values[1337] != 0.0)) && (scratch.values[1812] != 0.0)) {
            scratch.store_ad(1345, &AdValue::sub(scratch.ad_value(736), scratch.ad_value(1343)));
        }

        if (((scratch.values[1320] != 0.0) && (scratch.values[1337] != 0.0)) && (scratch.values[1812] != 0.0)) {
            scratch.store_ad(1346, &AdValue::sqrt(AdValue::add(AdValue::square(scratch.ad_value(1345)), scratch.ad_value(1341))));
        }

        if (((scratch.values[1320] != 0.0) && (scratch.values[1337] != 0.0)) && (scratch.values[1812] != 0.0)) {
            scratch.store_ad(1348, &AdValue::scale(AdValue::div(AdValue::mul(scratch.ad_value(525), scratch.ad_value(736)), AdValue::add(scratch.ad_value(1344), scratch.ad_value(1346))), 2.0));
        }

        scratch.values[1813] = if (scratch.values[525] < scratch.values[732]) { 1.0 } else { 0.0 };

        scratch.values[1814] = if ((((0.5 * (scratch.values[525] * scratch.values[427]))) as f64).abs() < 230.25850929940458) { 1.0 } else { 0.0 };

        if (((((scratch.values[1320] != 0.0) && (scratch.values[1337] != 0.0)) && (scratch.values[1812] != 0.0)) && (scratch.values[1813] != 0.0)) && (scratch.values[1814] != 0.0)) {
            scratch.store_ad(1350, &AdValue::exp(AdValue::scale(scratch.ad_value(525), (scratch.values[427] * 0.5))));
        }

        scratch.values[1815] = if ((0.5 * (scratch.values[525] * scratch.values[427])) < 0.0) { 1.0 } else { 0.0 };

        if ((((((scratch.values[1320] != 0.0) && (scratch.values[1337] != 0.0)) && (scratch.values[1812] != 0.0)) && (scratch.values[1813] != 0.0)) && (!(scratch.values[1814] != 0.0))) && (scratch.values[1815] != 0.0)) {
            let assign31310_ad_e39328: AdValue = AdValue::div_from_scalar(1e-100, AdValue::offset(AdValue::mul(AdValue::sub_from_scalar((-230.25850929940458), AdValue::scale(scratch.ad_value(525), (scratch.values[427] * 0.5))), AdValue::offset(AdValue::scale(AdValue::mul(AdValue::sub_from_scalar((-230.25850929940458), AdValue::scale(scratch.ad_value(525), (scratch.values[427] * 0.5))), AdValue::offset(AdValue::scale(AdValue::sub_from_scalar((-230.25850929940458), AdValue::scale(scratch.ad_value(525), (scratch.values[427] * 0.5))), 0.3333333333333333), 1.0)), 0.5), 1.0)), 1.0));
            scratch.store_ad(1350, &assign31310_ad_e39328);
        }

        if ((((((scratch.values[1320] != 0.0) && (scratch.values[1337] != 0.0)) && (scratch.values[1812] != 0.0)) && (scratch.values[1813] != 0.0)) && (!(scratch.values[1814] != 0.0))) && (!(scratch.values[1815] != 0.0))) {
            scratch.store_ad(1350, &AdValue::scale(AdValue::offset(AdValue::mul(AdValue::offset(AdValue::scale(scratch.ad_value(525), (scratch.values[427] * 0.5)), (-230.25850929940458)), AdValue::offset(AdValue::scale(AdValue::mul(AdValue::offset(AdValue::scale(scratch.ad_value(525), (scratch.values[427] * 0.5)), (-230.25850929940458)), AdValue::offset(AdValue::scale(AdValue::offset(AdValue::scale(scratch.ad_value(525), (scratch.values[427] * 0.5)), (-230.25850929940458)), 0.3333333333333333), 1.0)), 0.5), 1.0)), 1.0), 1e100));
        }

        if ((((scratch.values[1320] != 0.0) && (scratch.values[1337] != 0.0)) && (scratch.values[1812] != 0.0)) && (scratch.values[1813] != 0.0)) {
            scratch.store_ad(1347, &AdValue::square(scratch.ad_value(1350)));
        }

        if ((((scratch.values[1320] != 0.0) && (scratch.values[1337] != 0.0)) && (scratch.values[1812] != 0.0)) && (!(scratch.values[1813] != 0.0))) {
            scratch.store_ad(1347, &AdValue::mul(AdValue::offset(AdValue::scale(AdValue::sub(scratch.ad_value(525), scratch.ad_value(732)), scratch.values[427]), 1.0), scratch.ad_value(733)));
        }

        if ((((scratch.values[1320] != 0.0) && (scratch.values[1337] != 0.0)) && (scratch.values[1812] != 0.0)) && (!(scratch.values[1813] != 0.0))) {
            scratch.store_ad(1350, &AdValue::sqrt(scratch.ad_value(1347)));
        }

        if (((scratch.values[1320] != 0.0) && (scratch.values[1337] != 0.0)) && (scratch.values[1812] != 0.0)) {
            scratch.store_ad(1347, &AdValue::offset(scratch.ad_value(1347), (-1.0)));
        }

        if (((scratch.values[1320] != 0.0) && (scratch.values[1337] != 0.0)) && (scratch.values[1812] != 0.0)) {
            scratch.store_ad(1349, &AdValue::div_from_scalar(1.0, scratch.ad_value(1350)));
        }

        scratch.values[1816] = if (scratch.values[525] > 0.0) { 1.0 } else { 0.0 };

        if ((((scratch.values[1320] != 0.0) && (scratch.values[1337] != 0.0)) && (scratch.values[1812] != 0.0)) && (scratch.values[1816] != 0.0)) {
            scratch.store_ad(1351, &AdValue::scale(AdValue::ln(AdValue::add(AdValue::offset(scratch.ad_value(1349), 2.0), AdValue::sqrt(AdValue::mul(AdValue::offset(scratch.ad_value(1349), 1.0), AdValue::offset(scratch.ad_value(1349), 3.0))))), (scratch.values[426] * 2.0)));
        }

        if ((((scratch.values[1320] != 0.0) && (scratch.values[1337] != 0.0)) && (scratch.values[1812] != 0.0)) && (!(scratch.values[1816] != 0.0))) {
            scratch.store_ad(1351, &AdValue::sub(AdValue::scale(AdValue::ln(AdValue::add(AdValue::offset(AdValue::scale(scratch.ad_value(1350), 2.0), 1.0), AdValue::sqrt(AdValue::mul(AdValue::offset(scratch.ad_value(1350), 1.0), AdValue::offset(AdValue::scale(scratch.ad_value(1350), 3.0), 1.0))))), (scratch.values[426] * 2.0)), scratch.ad_value(525)));
        }

        if (((scratch.values[1320] != 0.0) && (scratch.values[1337] != 0.0)) && (scratch.values[1812] != 0.0)) {
            scratch.store_ad(1352, &AdValue::sub(scratch.ad_value(734), scratch.ad_value(1351)));
        }

        if (((scratch.values[1320] != 0.0) && (scratch.values[1337] != 0.0)) && (scratch.values[1812] != 0.0)) {
            scratch.store_ad(1353, &AdValue::scale(AdValue::sub(AdValue::add(scratch.ad_value(525), scratch.ad_value(1352)), AdValue::sqrt(AdValue::offset(AdValue::mul(AdValue::sub(scratch.ad_value(525), scratch.ad_value(1352)), AdValue::sub(scratch.ad_value(525), scratch.ad_value(1352))), ((4.0 * scratch.values[426]) * scratch.values[426])))), 0.5));
        }

    }
}
