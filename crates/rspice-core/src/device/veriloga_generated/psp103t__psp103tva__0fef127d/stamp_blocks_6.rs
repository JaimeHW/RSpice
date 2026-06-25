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
        if (((scratch.values[1320] != 0.0) && (scratch.values[1337] != 0.0)) && (scratch.values[1812] != 0.0)) {
            scratch.store_ad(1354, &AdValue::scale(AdValue::sub(AdValue::add(scratch.ad_value(525), scratch.ad_value(737)), AdValue::sqrt(AdValue::offset(AdValue::mul(AdValue::sub(scratch.ad_value(525), scratch.ad_value(737)), AdValue::sub(scratch.ad_value(525), scratch.ad_value(737))), ((4.0 * scratch.values[424]) * scratch.values[424])))), 0.5));
        }

        if (((scratch.values[1320] != 0.0) && (scratch.values[1337] != 0.0)) && (scratch.values[1812] != 0.0)) {
            scratch.store_ad(1355, &AdValue::scale(AdValue::sub(scratch.ad_value(525), AdValue::sqrt(AdValue::offset(AdValue::mul(scratch.ad_value(525), scratch.ad_value(525)), ((4.0 * 1e-6) * 1e-6)))), 0.5));
        }

        scratch.values[1817] = if (scratch.values[724] == 0.0) { 1.0 } else { 0.0 };

        if (((scratch.values[1320] != 0.0) && (scratch.values[1337] != 0.0)) && (scratch.values[1817] != 0.0)) {
            scratch.values[1383] = 0.0;
            scratch.node_derivatives[1383] = [0.0; Instance::NODE_COUNT];
            scratch.branch_derivatives[1383] = [0.0; Instance::BRANCH_COUNT];
        }

        scratch.values[1818] = if (scratch.values[607] == 0.5) { 1.0 } else { 0.0 };

        if ((((scratch.values[1320] != 0.0) && (scratch.values[1337] != 0.0)) && (!(scratch.values[1817] != 0.0))) && (scratch.values[1818] != 0.0)) {
            scratch.store_ad(1356, &AdValue::sqrt(AdValue::sub_from_scalar(1.0, AdValue::mul(scratch.ad_value(1348), scratch.ad_value(604)))));
        }

        if ((((scratch.values[1320] != 0.0) && (scratch.values[1337] != 0.0)) && (!(scratch.values[1817] != 0.0))) && (!(scratch.values[1818] != 0.0))) {
            scratch.store_ad(1356, &AdValue::pow(AdValue::sub_from_scalar(1.0, AdValue::mul(scratch.ad_value(1348), scratch.ad_value(604))), scratch.ad_value(607)));
        }

        if (((scratch.values[1320] != 0.0) && (scratch.values[1337] != 0.0)) && (!(scratch.values[1817] != 0.0))) {
            scratch.store_ad(1357, &AdValue::mul(scratch.ad_value(595), scratch.ad_value(1347)));
        }

        scratch.values[1819] = if ((scratch.values[560] == 0.0) && (scratch.values[563] == 0.0)) { 1.0 } else { 0.0 };

        if ((((scratch.values[1320] != 0.0) && (scratch.values[1337] != 0.0)) && (!(scratch.values[1817] != 0.0))) && (scratch.values[1819] != 0.0)) {
            scratch.values[1358] = 0.0;
            scratch.node_derivatives[1358] = [0.0; Instance::NODE_COUNT];
            scratch.branch_derivatives[1358] = [0.0; Instance::BRANCH_COUNT];
        }

        if ((((scratch.values[1320] != 0.0) && (scratch.values[1337] != 0.0)) && (!(scratch.values[1817] != 0.0))) && (!(scratch.values[1819] != 0.0))) {
            scratch.store_ad(1359, &AdValue::sub(scratch.ad_value(601), scratch.ad_value(1353)));
        }

        if ((((scratch.values[1320] != 0.0) && (scratch.values[1337] != 0.0)) && (!(scratch.values[1817] != 0.0))) && (!(scratch.values[1819] != 0.0))) {
            scratch.store_ad(1360, &AdValue::sub_from_scalar(1.0, AdValue::sqrt(AdValue::sub_from_scalar(1.0, AdValue::div(scratch.ad_value(1351), scratch.ad_value(1359))))));
        }

        scratch.values[1820] = if (scratch.values[549] == 0.5) { 1.0 } else { 0.0 };

        if (((((scratch.values[1320] != 0.0) && (scratch.values[1337] != 0.0)) && (!(scratch.values[1817] != 0.0))) && (!(scratch.values[1819] != 0.0))) && (scratch.values[1820] != 0.0)) {
            scratch.values[1361] = 0.0;
            scratch.node_derivatives[1361] = [0.0; Instance::NODE_COUNT];
            scratch.branch_derivatives[1361] = [0.0; Instance::BRANCH_COUNT];
        }

        if (((((scratch.values[1320] != 0.0) && (scratch.values[1337] != 0.0)) && (!(scratch.values[1817] != 0.0))) && (!(scratch.values[1819] != 0.0))) && (!(scratch.values[1820] != 0.0))) {
            scratch.store_ad(1361, &AdValue::mul(AdValue::add(AdValue::div(AdValue::mul(AdValue::square(scratch.ad_value(1360)), AdValue::ln(scratch.ad_value(1360))), AdValue::sub_from_scalar(1.0, scratch.ad_value(1360))), scratch.ad_value(1360)), AdValue::sub_from_scalar(1.0, AdValue::scale(scratch.ad_value(549), 2.0))));
        }

        if ((((scratch.values[1320] != 0.0) && (scratch.values[1337] != 0.0)) && (!(scratch.values[1817] != 0.0))) && (!(scratch.values[1819] != 0.0))) {
            scratch.store_ad(1362, &AdValue::add(scratch.ad_value(1360), scratch.ad_value(1361)));
        }

        scratch.values[1821] = if (scratch.values[549] == 0.5) { 1.0 } else { 0.0 };

        if (((((scratch.values[1320] != 0.0) && (scratch.values[1337] != 0.0)) && (!(scratch.values[1817] != 0.0))) && (!(scratch.values[1819] != 0.0))) && (scratch.values[1821] != 0.0)) {
            scratch.store_ad(1356, &AdValue::sqrt(AdValue::mul(scratch.ad_value(1359), scratch.ad_value(628))));
        }

        if (((((scratch.values[1320] != 0.0) && (scratch.values[1337] != 0.0)) && (!(scratch.values[1817] != 0.0))) && (!(scratch.values[1819] != 0.0))) && (!(scratch.values[1821] != 0.0))) {
            scratch.store_ad(1356, &AdValue::pow(AdValue::mul(scratch.ad_value(1359), scratch.ad_value(628)), scratch.ad_value(549)));
        }

        if ((((scratch.values[1320] != 0.0) && (scratch.values[1337] != 0.0)) && (!(scratch.values[1817] != 0.0))) && (!(scratch.values[1819] != 0.0))) {
            scratch.store_ad(1363, &AdValue::mul(scratch.ad_value(622), scratch.ad_value(1356)));
        }

        if ((((scratch.values[1320] != 0.0) && (scratch.values[1337] != 0.0)) && (!(scratch.values[1817] != 0.0))) && (!(scratch.values[1819] != 0.0))) {
            scratch.store_ad(1364, &AdValue::mul(scratch.ad_value(592), AdValue::mul(AdValue::offset(scratch.ad_value(1350), (-1.0)), scratch.ad_value(1363))));
        }

        if ((((scratch.values[1320] != 0.0) && (scratch.values[1337] != 0.0)) && (!(scratch.values[1817] != 0.0))) && (!(scratch.values[1819] != 0.0))) {
            scratch.store_ad(1358, &AdValue::mul(scratch.ad_value(560), AdValue::mul(scratch.ad_value(1364), scratch.ad_value(1362))));
        }

        scratch.values[1822] = if (scratch.values[563] == 0.0) { 1.0 } else { 0.0 };

        if ((((scratch.values[1320] != 0.0) && (scratch.values[1337] != 0.0)) && (!(scratch.values[1817] != 0.0))) && (scratch.values[1822] != 0.0)) {
            scratch.values[1365] = 0.0;
            scratch.node_derivatives[1365] = [0.0; Instance::NODE_COUNT];
            scratch.branch_derivatives[1365] = [0.0; Instance::BRANCH_COUNT];
        }

        if ((((scratch.values[1320] != 0.0) && (scratch.values[1337] != 0.0)) && (!(scratch.values[1817] != 0.0))) && (!(scratch.values[1822] != 0.0))) {
            scratch.store_ad(1366, &AdValue::mul(scratch.ad_value(637), AdValue::div(AdValue::mul(scratch.ad_value(1363), scratch.ad_value(607)), scratch.ad_value(1359))));
        }

        if ((((scratch.values[1320] != 0.0) && (scratch.values[1337] != 0.0)) && (!(scratch.values[1817] != 0.0))) && (!(scratch.values[1822] != 0.0))) {
            scratch.store_ad(1367, &AdValue::div(AdValue::scale(scratch.ad_value(634), 0.666666666666667), scratch.ad_value(1366)));
        }

        if ((((scratch.values[1320] != 0.0) && (scratch.values[1337] != 0.0)) && (!(scratch.values[1817] != 0.0))) && (!(scratch.values[1822] != 0.0))) {
            scratch.store_ad(1368, &AdValue::square(scratch.ad_value(1367)));
        }

        if ((((scratch.values[1320] != 0.0) && (scratch.values[1337] != 0.0)) && (!(scratch.values[1817] != 0.0))) && (!(scratch.values[1822] != 0.0))) {
            scratch.store_ad(1369, &AdValue::sqrt(AdValue::div(AdValue::square(scratch.ad_value(1368)), AdValue::offset(AdValue::square(scratch.ad_value(1368)), 1.0))));
        }

        if ((((scratch.values[1320] != 0.0) && (scratch.values[1337] != 0.0)) && (!(scratch.values[1817] != 0.0))) && (!(scratch.values[1822] != 0.0))) {
            scratch.store_ad(1370, &AdValue::sqrt(AdValue::abs(scratch.ad_value(1369))));
        }

        if ((((scratch.values[1320] != 0.0) && (scratch.values[1337] != 0.0)) && (!(scratch.values[1817] != 0.0))) && (!(scratch.values[1822] != 0.0))) {
            scratch.store_ad(1371, &AdValue::mul(scratch.ad_value(1369), scratch.ad_value(1370)));
        }

        scratch.values[1823] = if (((-scratch.values[549]) * scratch.values[610]) == (-1.0)) { 1.0 } else { 0.0 };

        if (((((scratch.values[1320] != 0.0) && (scratch.values[1337] != 0.0)) && (!(scratch.values[1817] != 0.0))) && (!(scratch.values[1822] != 0.0))) && (scratch.values[1823] != 0.0)) {
            scratch.store_ad(1372, &AdValue::div_from_scalar(1.0, AdValue::offset(AdValue::mul(scratch.ad_value(1366), scratch.ad_value(1371)), 1.0)));
        }

        if (((((scratch.values[1320] != 0.0) && (scratch.values[1337] != 0.0)) && (!(scratch.values[1817] != 0.0))) && (!(scratch.values[1822] != 0.0))) && (!(scratch.values[1823] != 0.0))) {
            scratch.store_ad(1372, &AdValue::pow(AdValue::offset(AdValue::mul(scratch.ad_value(1366), scratch.ad_value(1371)), 1.0), AdValue::mul(AdValue::neg(scratch.ad_value(549)), scratch.ad_value(610))));
        }

        if ((((scratch.values[1320] != 0.0) && (scratch.values[1337] != 0.0)) && (!(scratch.values[1817] != 0.0))) && (!(scratch.values[1822] != 0.0))) {
            scratch.store_ad(1373, &AdValue::div(AdValue::mul(scratch.ad_value(1362), scratch.ad_value(1372)), AdValue::add(scratch.ad_value(1362), scratch.ad_value(1372))));
        }

        if ((((scratch.values[1320] != 0.0) && (scratch.values[1337] != 0.0)) && (!(scratch.values[1817] != 0.0))) && (!(scratch.values[1822] != 0.0))) {
            scratch.store_ad(1374, &AdValue::sqrt(AdValue::scale(AdValue::div(scratch.ad_value(1366), scratch.ad_value(1370)), 0.375)));
        }

        if ((((scratch.values[1320] != 0.0) && (scratch.values[1337] != 0.0)) && (!(scratch.values[1817] != 0.0))) && (!(scratch.values[1822] != 0.0))) {
            scratch.store_ad(1375, &AdValue::sub(AdValue::scale(AdValue::mul(scratch.ad_value(1367), scratch.ad_value(1370)), 2.0), scratch.ad_value(1369)));
        }

        if ((((scratch.values[1320] != 0.0) && (scratch.values[1337] != 0.0)) && (!(scratch.values[1817] != 0.0))) && (!(scratch.values[1822] != 0.0))) {
            scratch.store_ad(1376, &AdValue::add(AdValue::sub(AdValue::mul(AdValue::mul(scratch.ad_value(634), scratch.ad_value(1367)), scratch.ad_value(1370)), AdValue::mul(scratch.ad_value(634), scratch.ad_value(1369))), AdValue::scale(AdValue::mul(scratch.ad_value(1366), scratch.ad_value(1371)), 0.5)));
        }

        if ((((scratch.values[1320] != 0.0) && (scratch.values[1337] != 0.0)) && (!(scratch.values[1817] != 0.0))) && (!(scratch.values[1822] != 0.0))) {
            scratch.store_ad(1377, &AdValue::mul(AdValue::offset(scratch.ad_value(1375), (-1.0)), scratch.ad_value(1374)));
        }

        if ((((scratch.values[1320] != 0.0) && (scratch.values[1337] != 0.0)) && (!(scratch.values[1817] != 0.0))) && (!(scratch.values[1822] != 0.0))) {
            scratch.store_ad(1338, &AdValue::square(scratch.ad_value(1377)));
        }

        scratch.values[1824] = if (scratch.values[1377] > 0.0) { 1.0 } else { 0.0 };

        if (((((scratch.values[1320] != 0.0) && (scratch.values[1337] != 0.0)) && (!(scratch.values[1817] != 0.0))) && (!(scratch.values[1822] != 0.0))) && (scratch.values[1824] != 0.0)) {
            scratch.store_ad(1339, &AdValue::div_from_scalar(1.0, AdValue::offset(AdValue::scale(scratch.ad_value(1377), scratch.values[428]), 1.0)));
        }

        if (((((scratch.values[1320] != 0.0) && (scratch.values[1337] != 0.0)) && (!(scratch.values[1817] != 0.0))) && (!(scratch.values[1822] != 0.0))) && (!(scratch.values[1824] != 0.0))) {
            scratch.store_ad(1339, &AdValue::div_from_scalar(1.0, AdValue::sub_from_scalar(1.0, AdValue::scale(scratch.ad_value(1377), scratch.values[428]))));
        }

        scratch.values[1825] = if (((-scratch.values[1338]) + scratch.values[1376]) > (-230.25850929940458)) { 1.0 } else { 0.0 };

        if (((((scratch.values[1320] != 0.0) && (scratch.values[1337] != 0.0)) && (!(scratch.values[1817] != 0.0))) && (!(scratch.values[1822] != 0.0))) && (scratch.values[1825] != 0.0)) {
            scratch.store_ad(1356, &AdValue::exp(AdValue::sub(scratch.ad_value(1376), scratch.ad_value(1338))));
        }

        if (((((scratch.values[1320] != 0.0) && (scratch.values[1337] != 0.0)) && (!(scratch.values[1817] != 0.0))) && (!(scratch.values[1822] != 0.0))) && (!(scratch.values[1825] != 0.0))) {
            let assign31890_ad_e40271: AdValue = AdValue::div_from_scalar(1e-100, AdValue::offset(AdValue::mul(AdValue::sub_from_scalar((-230.25850929940458), AdValue::sub(scratch.ad_value(1376), scratch.ad_value(1338))), AdValue::offset(AdValue::scale(AdValue::mul(AdValue::sub_from_scalar((-230.25850929940458), AdValue::sub(scratch.ad_value(1376), scratch.ad_value(1338))), AdValue::offset(AdValue::scale(AdValue::sub_from_scalar((-230.25850929940458), AdValue::sub(scratch.ad_value(1376), scratch.ad_value(1338))), 0.3333333333333333), 1.0)), 0.5), 1.0)), 1.0));
            scratch.store_ad(1356, &assign31890_ad_e40271);
        }

        if ((((scratch.values[1320] != 0.0) && (scratch.values[1337] != 0.0)) && (!(scratch.values[1817] != 0.0))) && (!(scratch.values[1822] != 0.0))) {
            scratch.store_ad(1340, &AdValue::mul(AdValue::add(AdValue::add(AdValue::scale(scratch.ad_value(1339), 0.29214664), AdValue::scale(AdValue::square(scratch.ad_value(1339)), scratch.values[429])), AdValue::scale(AdValue::mul(AdValue::square(scratch.ad_value(1339)), scratch.ad_value(1339)), scratch.values[430])), scratch.ad_value(1356)));
        }

        scratch.values[1826] = if (scratch.values[1377] > 0.0) { 1.0 } else { 0.0 };

        if (((((scratch.values[1320] != 0.0) && (scratch.values[1337] != 0.0)) && (!(scratch.values[1817] != 0.0))) && (!(scratch.values[1822] != 0.0))) && (scratch.values[1826] != 0.0)) {
            scratch.values[1378] = scratch.values[1340];
            scratch.node_derivatives[1378] = scratch.node_derivatives[1340];
            scratch.branch_derivatives[1378] = scratch.branch_derivatives[1340];
        }

        scratch.values[1827] = if (scratch.values[1376] > (-230.25850929940458)) { 1.0 } else { 0.0 };

        if ((((((scratch.values[1320] != 0.0) && (scratch.values[1337] != 0.0)) && (!(scratch.values[1817] != 0.0))) && (!(scratch.values[1822] != 0.0))) && (!(scratch.values[1826] != 0.0))) && (scratch.values[1827] != 0.0)) {
            scratch.store_ad(1356, &AdValue::exp(scratch.ad_value(1376)));
        }

        if ((((((scratch.values[1320] != 0.0) && (scratch.values[1337] != 0.0)) && (!(scratch.values[1817] != 0.0))) && (!(scratch.values[1822] != 0.0))) && (!(scratch.values[1826] != 0.0))) && (!(scratch.values[1827] != 0.0))) {
            scratch.store_ad(1356, &AdValue::div_from_scalar(1e-100, AdValue::offset(AdValue::mul(AdValue::sub_from_scalar((-230.25850929940458), scratch.ad_value(1376)), AdValue::offset(AdValue::scale(AdValue::mul(AdValue::sub_from_scalar((-230.25850929940458), scratch.ad_value(1376)), AdValue::offset(AdValue::scale(AdValue::sub_from_scalar((-230.25850929940458), scratch.ad_value(1376)), 0.3333333333333333), 1.0)), 0.5), 1.0)), 1.0)));
        }

        if (((((scratch.values[1320] != 0.0) && (scratch.values[1337] != 0.0)) && (!(scratch.values[1817] != 0.0))) && (!(scratch.values[1822] != 0.0))) && (!(scratch.values[1826] != 0.0))) {
            scratch.store_ad(1378, &AdValue::sub(AdValue::scale(scratch.ad_value(1356), 2.0), scratch.ad_value(1340)));
        }

        if ((((scratch.values[1320] != 0.0) && (scratch.values[1337] != 0.0)) && (!(scratch.values[1817] != 0.0))) && (!(scratch.values[1822] != 0.0))) {
            scratch.store_ad(1379, &AdValue::scale(AdValue::div(AdValue::mul(scratch.ad_value(634), scratch.ad_value(1378)), scratch.ad_value(1374)), (1.772453850905516 * 0.5)));
        }

        if ((((scratch.values[1320] != 0.0) && (scratch.values[1337] != 0.0)) && (!(scratch.values[1817] != 0.0))) && (!(scratch.values[1822] != 0.0))) {
            scratch.store_ad(1365, &AdValue::mul(scratch.ad_value(563), AdValue::mul(AdValue::mul(scratch.ad_value(1364), scratch.ad_value(1379)), scratch.ad_value(1373))));
        }

        scratch.values[1828] = if (scratch.values[569] == 0.0) { 1.0 } else { 0.0 };

        if ((((scratch.values[1320] != 0.0) && (scratch.values[1337] != 0.0)) && (!(scratch.values[1817] != 0.0))) && (scratch.values[1828] != 0.0)) {
            scratch.values[1380] = 0.0;
            scratch.node_derivatives[1380] = [0.0; Instance::NODE_COUNT];
            scratch.branch_derivatives[1380] = [0.0; Instance::BRANCH_COUNT];
        }

        scratch.values[1829] = if (scratch.values[549] == 0.5) { 1.0 } else { 0.0 };

        if (((((scratch.values[1320] != 0.0) && (scratch.values[1337] != 0.0)) && (!(scratch.values[1817] != 0.0))) && (!(scratch.values[1828] != 0.0))) && (scratch.values[1829] != 0.0)) {
            scratch.store_ad(1356, &AdValue::sqrt(AdValue::mul(AdValue::sub(scratch.ad_value(546), scratch.ad_value(1354)), scratch.ad_value(628))));
        }

        if (((((scratch.values[1320] != 0.0) && (scratch.values[1337] != 0.0)) && (!(scratch.values[1817] != 0.0))) && (!(scratch.values[1828] != 0.0))) && (!(scratch.values[1829] != 0.0))) {
            scratch.store_ad(1356, &AdValue::pow(AdValue::mul(AdValue::sub(scratch.ad_value(546), scratch.ad_value(1354)), scratch.ad_value(628)), scratch.ad_value(549)));
        }

        if ((((scratch.values[1320] != 0.0) && (scratch.values[1337] != 0.0)) && (!(scratch.values[1817] != 0.0))) && (!(scratch.values[1828] != 0.0))) {
            scratch.store_ad(1381, &AdValue::mul(scratch.ad_value(610), AdValue::div(AdValue::mul(AdValue::sub(scratch.ad_value(546), scratch.ad_value(1354)), scratch.ad_value(625)), scratch.ad_value(1356))));
        }

        scratch.values[1830] = if (((((-scratch.values[640]) / scratch.values[1381])) as f64).abs() < 230.25850929940458) { 1.0 } else { 0.0 };

        if (((((scratch.values[1320] != 0.0) && (scratch.values[1337] != 0.0)) && (!(scratch.values[1817] != 0.0))) && (!(scratch.values[1828] != 0.0))) && (scratch.values[1830] != 0.0)) {
            scratch.store_ad(1356, &AdValue::exp(AdValue::div(AdValue::neg(scratch.ad_value(640)), scratch.ad_value(1381))));
        }

        scratch.values[1831] = if (((-scratch.values[640]) / scratch.values[1381]) < 0.0) { 1.0 } else { 0.0 };

        if ((((((scratch.values[1320] != 0.0) && (scratch.values[1337] != 0.0)) && (!(scratch.values[1817] != 0.0))) && (!(scratch.values[1828] != 0.0))) && (!(scratch.values[1830] != 0.0))) && (scratch.values[1831] != 0.0)) {
            let assign32080_ad_e40598: AdValue = AdValue::offset(AdValue::mul(AdValue::sub_from_scalar((-230.25850929940458), AdValue::div(AdValue::neg(scratch.ad_value(640)), scratch.ad_value(1381))), AdValue::offset(AdValue::scale(AdValue::mul(AdValue::sub_from_scalar((-230.25850929940458), AdValue::div(AdValue::neg(scratch.ad_value(640)), scratch.ad_value(1381))), AdValue::offset(AdValue::scale(AdValue::sub_from_scalar((-230.25850929940458), AdValue::div(AdValue::neg(scratch.ad_value(640)), scratch.ad_value(1381))), 0.3333333333333333), 1.0)), 0.5), 1.0)), 1.0);
            scratch.store_ad(1356, &AdValue::div_from_scalar(1e-100, assign32080_ad_e40598));
        }

        if ((((((scratch.values[1320] != 0.0) && (scratch.values[1337] != 0.0)) && (!(scratch.values[1817] != 0.0))) && (!(scratch.values[1828] != 0.0))) && (!(scratch.values[1830] != 0.0))) && (!(scratch.values[1831] != 0.0))) {
            let assign32090_ad_e40648: AdValue = AdValue::scale(AdValue::offset(AdValue::mul(AdValue::offset(AdValue::div(AdValue::neg(scratch.ad_value(640)), scratch.ad_value(1381)), (-230.25850929940458)), AdValue::offset(AdValue::scale(AdValue::mul(AdValue::offset(AdValue::div(AdValue::neg(scratch.ad_value(640)), scratch.ad_value(1381)), (-230.25850929940458)), AdValue::offset(AdValue::scale(AdValue::offset(AdValue::div(AdValue::neg(scratch.ad_value(640)), scratch.ad_value(1381)), (-230.25850929940458)), 0.3333333333333333), 1.0)), 0.5), 1.0)), 1.0), 1e100);
            scratch.store_ad(1356, &assign32090_ad_e40648);
        }

        if ((((scratch.values[1320] != 0.0) && (scratch.values[1337] != 0.0)) && (!(scratch.values[1817] != 0.0))) && (!(scratch.values[1828] != 0.0))) {
            scratch.store_ad(1380, &AdValue::mul(scratch.ad_value(569), AdValue::mul(AdValue::mul(AdValue::mul(scratch.ad_value(525), scratch.ad_value(1381)), scratch.ad_value(1381)), scratch.ad_value(1356))));
        }

        scratch.values[1832] = if (scratch.values[578] > 1000.0) { 1.0 } else { 0.0 };

        if ((((scratch.values[1320] != 0.0) && (scratch.values[1337] != 0.0)) && (!(scratch.values[1817] != 0.0))) && (scratch.values[1832] != 0.0)) {
            scratch.values[1382] = 1.0;
            scratch.node_derivatives[1382] = [0.0; Instance::NODE_COUNT];
            scratch.branch_derivatives[1382] = [0.0; Instance::BRANCH_COUNT];
        }

        scratch.values[1833] = if (scratch.values[1355] > ((-scratch.values[500]) * scratch.values[578])) { 1.0 } else { 0.0 };

        scratch.values[1834] = if (scratch.values[581] == 4.0) { 1.0 } else { 0.0 };

        if ((((((scratch.values[1320] != 0.0) && (scratch.values[1337] != 0.0)) && (!(scratch.values[1817] != 0.0))) && (!(scratch.values[1832] != 0.0))) && (scratch.values[1833] != 0.0)) && (scratch.values[1834] != 0.0)) {
            scratch.store_ad(1356, &AdValue::mul(AdValue::mul(AdValue::mul(AdValue::mul(scratch.ad_value(1355), scratch.ad_value(646)), AdValue::mul(scratch.ad_value(1355), scratch.ad_value(646))), AdValue::mul(scratch.ad_value(1355), scratch.ad_value(646))), AdValue::mul(scratch.ad_value(1355), scratch.ad_value(646))));
        }

        if ((((((scratch.values[1320] != 0.0) && (scratch.values[1337] != 0.0)) && (!(scratch.values[1817] != 0.0))) && (!(scratch.values[1832] != 0.0))) && (scratch.values[1833] != 0.0)) && (!(scratch.values[1834] != 0.0))) {
            scratch.store_ad(1356, &AdValue::pow(AdValue::abs(AdValue::mul(scratch.ad_value(1355), scratch.ad_value(646))), scratch.ad_value(581)));
        }

        if (((((scratch.values[1320] != 0.0) && (scratch.values[1337] != 0.0)) && (!(scratch.values[1817] != 0.0))) && (!(scratch.values[1832] != 0.0))) && (scratch.values[1833] != 0.0)) {
            scratch.store_ad(1382, &AdValue::div_from_scalar(1.0, AdValue::sub_from_scalar(1.0, scratch.ad_value(1356))));
        }

        if (((((scratch.values[1320] != 0.0) && (scratch.values[1337] != 0.0)) && (!(scratch.values[1817] != 0.0))) && (!(scratch.values[1832] != 0.0))) && (!(scratch.values[1833] != 0.0))) {
            scratch.store_ad(1382, &AdValue::add(scratch.ad_value(643), AdValue::mul(AdValue::add(scratch.ad_value(1355), AdValue::scale(scratch.ad_value(578), scratch.values[500])), scratch.ad_value(649))));
        }

        if (((scratch.values[1320] != 0.0) && (scratch.values[1337] != 0.0)) && (!(scratch.values[1817] != 0.0))) {
            scratch.store_ad(1383, &AdValue::mul(AdValue::add(AdValue::add(AdValue::add(scratch.ad_value(1357), scratch.ad_value(1358)), scratch.ad_value(1365)), scratch.ad_value(1380)), scratch.ad_value(1382)));
        }

        scratch.values[1835] = if (scratch.values[725] == 0.0) { 1.0 } else { 0.0 };

        if (((scratch.values[1320] != 0.0) && (scratch.values[1337] != 0.0)) && (scratch.values[1835] != 0.0)) {
            scratch.values[1384] = 0.0;
            scratch.node_derivatives[1384] = [0.0; Instance::NODE_COUNT];
            scratch.branch_derivatives[1384] = [0.0; Instance::BRANCH_COUNT];
        }

        scratch.values[1836] = if (scratch.values[608] == 0.5) { 1.0 } else { 0.0 };

        if ((((scratch.values[1320] != 0.0) && (scratch.values[1337] != 0.0)) && (!(scratch.values[1835] != 0.0))) && (scratch.values[1836] != 0.0)) {
            scratch.store_ad(1356, &AdValue::sqrt(AdValue::sub_from_scalar(1.0, AdValue::mul(scratch.ad_value(1348), scratch.ad_value(605)))));
        }

        if ((((scratch.values[1320] != 0.0) && (scratch.values[1337] != 0.0)) && (!(scratch.values[1835] != 0.0))) && (!(scratch.values[1836] != 0.0))) {
            scratch.store_ad(1356, &AdValue::pow(AdValue::sub_from_scalar(1.0, AdValue::mul(scratch.ad_value(1348), scratch.ad_value(605))), scratch.ad_value(608)));
        }

        if (((scratch.values[1320] != 0.0) && (scratch.values[1337] != 0.0)) && (!(scratch.values[1835] != 0.0))) {
            scratch.store_ad(1357, &AdValue::mul(scratch.ad_value(596), scratch.ad_value(1347)));
        }

        scratch.values[1837] = if ((scratch.values[561] == 0.0) && (scratch.values[564] == 0.0)) { 1.0 } else { 0.0 };

        if ((((scratch.values[1320] != 0.0) && (scratch.values[1337] != 0.0)) && (!(scratch.values[1835] != 0.0))) && (scratch.values[1837] != 0.0)) {
            scratch.values[1358] = 0.0;
            scratch.node_derivatives[1358] = [0.0; Instance::NODE_COUNT];
            scratch.branch_derivatives[1358] = [0.0; Instance::BRANCH_COUNT];
        }

        if ((((scratch.values[1320] != 0.0) && (scratch.values[1337] != 0.0)) && (!(scratch.values[1835] != 0.0))) && (!(scratch.values[1837] != 0.0))) {
            scratch.store_ad(1359, &AdValue::sub(scratch.ad_value(602), scratch.ad_value(1353)));
        }

        if ((((scratch.values[1320] != 0.0) && (scratch.values[1337] != 0.0)) && (!(scratch.values[1835] != 0.0))) && (!(scratch.values[1837] != 0.0))) {
            scratch.store_ad(1360, &AdValue::sub_from_scalar(1.0, AdValue::sqrt(AdValue::sub_from_scalar(1.0, AdValue::div(scratch.ad_value(1351), scratch.ad_value(1359))))));
        }

        scratch.values[1838] = if (scratch.values[550] == 0.5) { 1.0 } else { 0.0 };

        if (((((scratch.values[1320] != 0.0) && (scratch.values[1337] != 0.0)) && (!(scratch.values[1835] != 0.0))) && (!(scratch.values[1837] != 0.0))) && (scratch.values[1838] != 0.0)) {
            scratch.values[1361] = 0.0;
            scratch.node_derivatives[1361] = [0.0; Instance::NODE_COUNT];
            scratch.branch_derivatives[1361] = [0.0; Instance::BRANCH_COUNT];
        }

        if (((((scratch.values[1320] != 0.0) && (scratch.values[1337] != 0.0)) && (!(scratch.values[1835] != 0.0))) && (!(scratch.values[1837] != 0.0))) && (!(scratch.values[1838] != 0.0))) {
            scratch.store_ad(1361, &AdValue::mul(AdValue::add(AdValue::div(AdValue::mul(AdValue::square(scratch.ad_value(1360)), AdValue::ln(scratch.ad_value(1360))), AdValue::sub_from_scalar(1.0, scratch.ad_value(1360))), scratch.ad_value(1360)), AdValue::sub_from_scalar(1.0, AdValue::scale(scratch.ad_value(550), 2.0))));
        }

        if ((((scratch.values[1320] != 0.0) && (scratch.values[1337] != 0.0)) && (!(scratch.values[1835] != 0.0))) && (!(scratch.values[1837] != 0.0))) {
            scratch.store_ad(1362, &AdValue::add(scratch.ad_value(1360), scratch.ad_value(1361)));
        }

        scratch.values[1839] = if (scratch.values[550] == 0.5) { 1.0 } else { 0.0 };

        if (((((scratch.values[1320] != 0.0) && (scratch.values[1337] != 0.0)) && (!(scratch.values[1835] != 0.0))) && (!(scratch.values[1837] != 0.0))) && (scratch.values[1839] != 0.0)) {
            scratch.store_ad(1356, &AdValue::sqrt(AdValue::mul(scratch.ad_value(1359), scratch.ad_value(629))));
        }

        if (((((scratch.values[1320] != 0.0) && (scratch.values[1337] != 0.0)) && (!(scratch.values[1835] != 0.0))) && (!(scratch.values[1837] != 0.0))) && (!(scratch.values[1839] != 0.0))) {
            scratch.store_ad(1356, &AdValue::pow(AdValue::mul(scratch.ad_value(1359), scratch.ad_value(629)), scratch.ad_value(550)));
        }

        if ((((scratch.values[1320] != 0.0) && (scratch.values[1337] != 0.0)) && (!(scratch.values[1835] != 0.0))) && (!(scratch.values[1837] != 0.0))) {
            scratch.store_ad(1363, &AdValue::mul(scratch.ad_value(623), scratch.ad_value(1356)));
        }

        if ((((scratch.values[1320] != 0.0) && (scratch.values[1337] != 0.0)) && (!(scratch.values[1835] != 0.0))) && (!(scratch.values[1837] != 0.0))) {
            scratch.store_ad(1364, &AdValue::mul(scratch.ad_value(593), AdValue::mul(AdValue::offset(scratch.ad_value(1350), (-1.0)), scratch.ad_value(1363))));
        }

        if ((((scratch.values[1320] != 0.0) && (scratch.values[1337] != 0.0)) && (!(scratch.values[1835] != 0.0))) && (!(scratch.values[1837] != 0.0))) {
            scratch.store_ad(1358, &AdValue::mul(scratch.ad_value(561), AdValue::mul(scratch.ad_value(1364), scratch.ad_value(1362))));
        }

        scratch.values[1840] = if (scratch.values[564] == 0.0) { 1.0 } else { 0.0 };

        if ((((scratch.values[1320] != 0.0) && (scratch.values[1337] != 0.0)) && (!(scratch.values[1835] != 0.0))) && (scratch.values[1840] != 0.0)) {
            scratch.values[1365] = 0.0;
            scratch.node_derivatives[1365] = [0.0; Instance::NODE_COUNT];
            scratch.branch_derivatives[1365] = [0.0; Instance::BRANCH_COUNT];
        }

        if ((((scratch.values[1320] != 0.0) && (scratch.values[1337] != 0.0)) && (!(scratch.values[1835] != 0.0))) && (!(scratch.values[1840] != 0.0))) {
            scratch.store_ad(1366, &AdValue::mul(scratch.ad_value(638), AdValue::div(AdValue::mul(scratch.ad_value(1363), scratch.ad_value(608)), scratch.ad_value(1359))));
        }

        if ((((scratch.values[1320] != 0.0) && (scratch.values[1337] != 0.0)) && (!(scratch.values[1835] != 0.0))) && (!(scratch.values[1840] != 0.0))) {
            scratch.store_ad(1367, &AdValue::div(AdValue::scale(scratch.ad_value(635), 0.666666666666667), scratch.ad_value(1366)));
        }

        if ((((scratch.values[1320] != 0.0) && (scratch.values[1337] != 0.0)) && (!(scratch.values[1835] != 0.0))) && (!(scratch.values[1840] != 0.0))) {
            scratch.store_ad(1368, &AdValue::square(scratch.ad_value(1367)));
        }

        if ((((scratch.values[1320] != 0.0) && (scratch.values[1337] != 0.0)) && (!(scratch.values[1835] != 0.0))) && (!(scratch.values[1840] != 0.0))) {
            scratch.store_ad(1369, &AdValue::sqrt(AdValue::div(AdValue::square(scratch.ad_value(1368)), AdValue::offset(AdValue::square(scratch.ad_value(1368)), 1.0))));
        }

        if ((((scratch.values[1320] != 0.0) && (scratch.values[1337] != 0.0)) && (!(scratch.values[1835] != 0.0))) && (!(scratch.values[1840] != 0.0))) {
            scratch.store_ad(1370, &AdValue::sqrt(AdValue::abs(scratch.ad_value(1369))));
        }

        if ((((scratch.values[1320] != 0.0) && (scratch.values[1337] != 0.0)) && (!(scratch.values[1835] != 0.0))) && (!(scratch.values[1840] != 0.0))) {
            scratch.store_ad(1371, &AdValue::mul(scratch.ad_value(1369), scratch.ad_value(1370)));
        }

        scratch.values[1841] = if (((-scratch.values[550]) * scratch.values[611]) == (-1.0)) { 1.0 } else { 0.0 };

        if (((((scratch.values[1320] != 0.0) && (scratch.values[1337] != 0.0)) && (!(scratch.values[1835] != 0.0))) && (!(scratch.values[1840] != 0.0))) && (scratch.values[1841] != 0.0)) {
            scratch.store_ad(1372, &AdValue::div_from_scalar(1.0, AdValue::offset(AdValue::mul(scratch.ad_value(1366), scratch.ad_value(1371)), 1.0)));
        }

        if (((((scratch.values[1320] != 0.0) && (scratch.values[1337] != 0.0)) && (!(scratch.values[1835] != 0.0))) && (!(scratch.values[1840] != 0.0))) && (!(scratch.values[1841] != 0.0))) {
            scratch.store_ad(1372, &AdValue::pow(AdValue::offset(AdValue::mul(scratch.ad_value(1366), scratch.ad_value(1371)), 1.0), AdValue::mul(AdValue::neg(scratch.ad_value(550)), scratch.ad_value(611))));
        }

        if ((((scratch.values[1320] != 0.0) && (scratch.values[1337] != 0.0)) && (!(scratch.values[1835] != 0.0))) && (!(scratch.values[1840] != 0.0))) {
            scratch.store_ad(1373, &AdValue::div(AdValue::mul(scratch.ad_value(1362), scratch.ad_value(1372)), AdValue::add(scratch.ad_value(1362), scratch.ad_value(1372))));
        }

        if ((((scratch.values[1320] != 0.0) && (scratch.values[1337] != 0.0)) && (!(scratch.values[1835] != 0.0))) && (!(scratch.values[1840] != 0.0))) {
            scratch.store_ad(1374, &AdValue::sqrt(AdValue::scale(AdValue::div(scratch.ad_value(1366), scratch.ad_value(1370)), 0.375)));
        }

        if ((((scratch.values[1320] != 0.0) && (scratch.values[1337] != 0.0)) && (!(scratch.values[1835] != 0.0))) && (!(scratch.values[1840] != 0.0))) {
            scratch.store_ad(1375, &AdValue::sub(AdValue::scale(AdValue::mul(scratch.ad_value(1367), scratch.ad_value(1370)), 2.0), scratch.ad_value(1369)));
        }

        if ((((scratch.values[1320] != 0.0) && (scratch.values[1337] != 0.0)) && (!(scratch.values[1835] != 0.0))) && (!(scratch.values[1840] != 0.0))) {
            scratch.store_ad(1376, &AdValue::add(AdValue::sub(AdValue::mul(AdValue::mul(scratch.ad_value(635), scratch.ad_value(1367)), scratch.ad_value(1370)), AdValue::mul(scratch.ad_value(635), scratch.ad_value(1369))), AdValue::scale(AdValue::mul(scratch.ad_value(1366), scratch.ad_value(1371)), 0.5)));
        }

        if ((((scratch.values[1320] != 0.0) && (scratch.values[1337] != 0.0)) && (!(scratch.values[1835] != 0.0))) && (!(scratch.values[1840] != 0.0))) {
            scratch.store_ad(1377, &AdValue::mul(AdValue::offset(scratch.ad_value(1375), (-1.0)), scratch.ad_value(1374)));
        }

        if ((((scratch.values[1320] != 0.0) && (scratch.values[1337] != 0.0)) && (!(scratch.values[1835] != 0.0))) && (!(scratch.values[1840] != 0.0))) {
            scratch.store_ad(1338, &AdValue::square(scratch.ad_value(1377)));
        }

        scratch.values[1842] = if (scratch.values[1377] > 0.0) { 1.0 } else { 0.0 };

        if (((((scratch.values[1320] != 0.0) && (scratch.values[1337] != 0.0)) && (!(scratch.values[1835] != 0.0))) && (!(scratch.values[1840] != 0.0))) && (scratch.values[1842] != 0.0)) {
            scratch.store_ad(1339, &AdValue::div_from_scalar(1.0, AdValue::offset(AdValue::scale(scratch.ad_value(1377), scratch.values[428]), 1.0)));
        }

        if (((((scratch.values[1320] != 0.0) && (scratch.values[1337] != 0.0)) && (!(scratch.values[1835] != 0.0))) && (!(scratch.values[1840] != 0.0))) && (!(scratch.values[1842] != 0.0))) {
            scratch.store_ad(1339, &AdValue::div_from_scalar(1.0, AdValue::sub_from_scalar(1.0, AdValue::scale(scratch.ad_value(1377), scratch.values[428]))));
        }

        scratch.values[1843] = if (((-scratch.values[1338]) + scratch.values[1376]) > (-230.25850929940458)) { 1.0 } else { 0.0 };

        if (((((scratch.values[1320] != 0.0) && (scratch.values[1337] != 0.0)) && (!(scratch.values[1835] != 0.0))) && (!(scratch.values[1840] != 0.0))) && (scratch.values[1843] != 0.0)) {
            scratch.store_ad(1356, &AdValue::exp(AdValue::sub(scratch.ad_value(1376), scratch.ad_value(1338))));
        }

        if (((((scratch.values[1320] != 0.0) && (scratch.values[1337] != 0.0)) && (!(scratch.values[1835] != 0.0))) && (!(scratch.values[1840] != 0.0))) && (!(scratch.values[1843] != 0.0))) {
            let assign32640_ad_e41477: AdValue = AdValue::div_from_scalar(1e-100, AdValue::offset(AdValue::mul(AdValue::sub_from_scalar((-230.25850929940458), AdValue::sub(scratch.ad_value(1376), scratch.ad_value(1338))), AdValue::offset(AdValue::scale(AdValue::mul(AdValue::sub_from_scalar((-230.25850929940458), AdValue::sub(scratch.ad_value(1376), scratch.ad_value(1338))), AdValue::offset(AdValue::scale(AdValue::sub_from_scalar((-230.25850929940458), AdValue::sub(scratch.ad_value(1376), scratch.ad_value(1338))), 0.3333333333333333), 1.0)), 0.5), 1.0)), 1.0));
            scratch.store_ad(1356, &assign32640_ad_e41477);
        }

        if ((((scratch.values[1320] != 0.0) && (scratch.values[1337] != 0.0)) && (!(scratch.values[1835] != 0.0))) && (!(scratch.values[1840] != 0.0))) {
            scratch.store_ad(1340, &AdValue::mul(AdValue::add(AdValue::add(AdValue::scale(scratch.ad_value(1339), 0.29214664), AdValue::scale(AdValue::square(scratch.ad_value(1339)), scratch.values[429])), AdValue::scale(AdValue::mul(AdValue::square(scratch.ad_value(1339)), scratch.ad_value(1339)), scratch.values[430])), scratch.ad_value(1356)));
        }

        scratch.values[1844] = if (scratch.values[1377] > 0.0) { 1.0 } else { 0.0 };

        if (((((scratch.values[1320] != 0.0) && (scratch.values[1337] != 0.0)) && (!(scratch.values[1835] != 0.0))) && (!(scratch.values[1840] != 0.0))) && (scratch.values[1844] != 0.0)) {
            scratch.values[1378] = scratch.values[1340];
            scratch.node_derivatives[1378] = scratch.node_derivatives[1340];
            scratch.branch_derivatives[1378] = scratch.branch_derivatives[1340];
        }

        scratch.values[1845] = if (scratch.values[1376] > (-230.25850929940458)) { 1.0 } else { 0.0 };

        if ((((((scratch.values[1320] != 0.0) && (scratch.values[1337] != 0.0)) && (!(scratch.values[1835] != 0.0))) && (!(scratch.values[1840] != 0.0))) && (!(scratch.values[1844] != 0.0))) && (scratch.values[1845] != 0.0)) {
            scratch.store_ad(1356, &AdValue::exp(scratch.ad_value(1376)));
        }

        if ((((((scratch.values[1320] != 0.0) && (scratch.values[1337] != 0.0)) && (!(scratch.values[1835] != 0.0))) && (!(scratch.values[1840] != 0.0))) && (!(scratch.values[1844] != 0.0))) && (!(scratch.values[1845] != 0.0))) {
            scratch.store_ad(1356, &AdValue::div_from_scalar(1e-100, AdValue::offset(AdValue::mul(AdValue::sub_from_scalar((-230.25850929940458), scratch.ad_value(1376)), AdValue::offset(AdValue::scale(AdValue::mul(AdValue::sub_from_scalar((-230.25850929940458), scratch.ad_value(1376)), AdValue::offset(AdValue::scale(AdValue::sub_from_scalar((-230.25850929940458), scratch.ad_value(1376)), 0.3333333333333333), 1.0)), 0.5), 1.0)), 1.0)));
        }

        if (((((scratch.values[1320] != 0.0) && (scratch.values[1337] != 0.0)) && (!(scratch.values[1835] != 0.0))) && (!(scratch.values[1840] != 0.0))) && (!(scratch.values[1844] != 0.0))) {
            scratch.store_ad(1378, &AdValue::sub(AdValue::scale(scratch.ad_value(1356), 2.0), scratch.ad_value(1340)));
        }

        if ((((scratch.values[1320] != 0.0) && (scratch.values[1337] != 0.0)) && (!(scratch.values[1835] != 0.0))) && (!(scratch.values[1840] != 0.0))) {
            scratch.store_ad(1379, &AdValue::scale(AdValue::div(AdValue::mul(scratch.ad_value(635), scratch.ad_value(1378)), scratch.ad_value(1374)), (1.772453850905516 * 0.5)));
        }

        if ((((scratch.values[1320] != 0.0) && (scratch.values[1337] != 0.0)) && (!(scratch.values[1835] != 0.0))) && (!(scratch.values[1840] != 0.0))) {
            scratch.store_ad(1365, &AdValue::mul(scratch.ad_value(564), AdValue::mul(AdValue::mul(scratch.ad_value(1364), scratch.ad_value(1379)), scratch.ad_value(1373))));
        }

        scratch.values[1846] = if (scratch.values[570] == 0.0) { 1.0 } else { 0.0 };

        if ((((scratch.values[1320] != 0.0) && (scratch.values[1337] != 0.0)) && (!(scratch.values[1835] != 0.0))) && (scratch.values[1846] != 0.0)) {
            scratch.values[1380] = 0.0;
            scratch.node_derivatives[1380] = [0.0; Instance::NODE_COUNT];
            scratch.branch_derivatives[1380] = [0.0; Instance::BRANCH_COUNT];
        }

        scratch.values[1847] = if (scratch.values[550] == 0.5) { 1.0 } else { 0.0 };

        if (((((scratch.values[1320] != 0.0) && (scratch.values[1337] != 0.0)) && (!(scratch.values[1835] != 0.0))) && (!(scratch.values[1846] != 0.0))) && (scratch.values[1847] != 0.0)) {
            scratch.store_ad(1356, &AdValue::sqrt(AdValue::mul(AdValue::sub(scratch.ad_value(547), scratch.ad_value(1354)), scratch.ad_value(629))));
        }

        if (((((scratch.values[1320] != 0.0) && (scratch.values[1337] != 0.0)) && (!(scratch.values[1835] != 0.0))) && (!(scratch.values[1846] != 0.0))) && (!(scratch.values[1847] != 0.0))) {
            scratch.store_ad(1356, &AdValue::pow(AdValue::mul(AdValue::sub(scratch.ad_value(547), scratch.ad_value(1354)), scratch.ad_value(629)), scratch.ad_value(550)));
        }

        if ((((scratch.values[1320] != 0.0) && (scratch.values[1337] != 0.0)) && (!(scratch.values[1835] != 0.0))) && (!(scratch.values[1846] != 0.0))) {
            scratch.store_ad(1381, &AdValue::mul(scratch.ad_value(611), AdValue::div(AdValue::mul(AdValue::sub(scratch.ad_value(547), scratch.ad_value(1354)), scratch.ad_value(626)), scratch.ad_value(1356))));
        }

        scratch.values[1848] = if (((((-scratch.values[641]) / scratch.values[1381])) as f64).abs() < 230.25850929940458) { 1.0 } else { 0.0 };

        if (((((scratch.values[1320] != 0.0) && (scratch.values[1337] != 0.0)) && (!(scratch.values[1835] != 0.0))) && (!(scratch.values[1846] != 0.0))) && (scratch.values[1848] != 0.0)) {
            scratch.store_ad(1356, &AdValue::exp(AdValue::div(AdValue::neg(scratch.ad_value(641)), scratch.ad_value(1381))));
        }

        scratch.values[1849] = if (((-scratch.values[641]) / scratch.values[1381]) < 0.0) { 1.0 } else { 0.0 };

    }

    pub(super) fn stamp_transient_block_25(
        &mut self,
        ctx: &GeneratedEvalContext<'_>,
        stamper: &mut GeneratedStamper<'_>,
        scratch: &mut Scratch,
    ) {
        let _ = stamper;
        if ((((((scratch.values[1320] != 0.0) && (scratch.values[1337] != 0.0)) && (!(scratch.values[1835] != 0.0))) && (!(scratch.values[1846] != 0.0))) && (!(scratch.values[1848] != 0.0))) && (scratch.values[1849] != 0.0)) {
            let assign32830_ad_e41804: AdValue = AdValue::offset(AdValue::mul(AdValue::sub_from_scalar((-230.25850929940458), AdValue::div(AdValue::neg(scratch.ad_value(641)), scratch.ad_value(1381))), AdValue::offset(AdValue::scale(AdValue::mul(AdValue::sub_from_scalar((-230.25850929940458), AdValue::div(AdValue::neg(scratch.ad_value(641)), scratch.ad_value(1381))), AdValue::offset(AdValue::scale(AdValue::sub_from_scalar((-230.25850929940458), AdValue::div(AdValue::neg(scratch.ad_value(641)), scratch.ad_value(1381))), 0.3333333333333333), 1.0)), 0.5), 1.0)), 1.0);
            scratch.store_ad(1356, &AdValue::div_from_scalar(1e-100, assign32830_ad_e41804));
        }

        if ((((((scratch.values[1320] != 0.0) && (scratch.values[1337] != 0.0)) && (!(scratch.values[1835] != 0.0))) && (!(scratch.values[1846] != 0.0))) && (!(scratch.values[1848] != 0.0))) && (!(scratch.values[1849] != 0.0))) {
            let assign32840_ad_e41854: AdValue = AdValue::scale(AdValue::offset(AdValue::mul(AdValue::offset(AdValue::div(AdValue::neg(scratch.ad_value(641)), scratch.ad_value(1381)), (-230.25850929940458)), AdValue::offset(AdValue::scale(AdValue::mul(AdValue::offset(AdValue::div(AdValue::neg(scratch.ad_value(641)), scratch.ad_value(1381)), (-230.25850929940458)), AdValue::offset(AdValue::scale(AdValue::offset(AdValue::div(AdValue::neg(scratch.ad_value(641)), scratch.ad_value(1381)), (-230.25850929940458)), 0.3333333333333333), 1.0)), 0.5), 1.0)), 1.0), 1e100);
            scratch.store_ad(1356, &assign32840_ad_e41854);
        }

        if ((((scratch.values[1320] != 0.0) && (scratch.values[1337] != 0.0)) && (!(scratch.values[1835] != 0.0))) && (!(scratch.values[1846] != 0.0))) {
            scratch.store_ad(1380, &AdValue::mul(scratch.ad_value(570), AdValue::mul(AdValue::mul(AdValue::mul(scratch.ad_value(525), scratch.ad_value(1381)), scratch.ad_value(1381)), scratch.ad_value(1356))));
        }

        scratch.values[1850] = if (scratch.values[579] > 1000.0) { 1.0 } else { 0.0 };

        if ((((scratch.values[1320] != 0.0) && (scratch.values[1337] != 0.0)) && (!(scratch.values[1835] != 0.0))) && (scratch.values[1850] != 0.0)) {
            scratch.values[1382] = 1.0;
            scratch.node_derivatives[1382] = [0.0; Instance::NODE_COUNT];
            scratch.branch_derivatives[1382] = [0.0; Instance::BRANCH_COUNT];
        }

        scratch.values[1851] = if (scratch.values[1355] > ((-scratch.values[500]) * scratch.values[579])) { 1.0 } else { 0.0 };

        scratch.values[1852] = if (scratch.values[582] == 4.0) { 1.0 } else { 0.0 };

        if ((((((scratch.values[1320] != 0.0) && (scratch.values[1337] != 0.0)) && (!(scratch.values[1835] != 0.0))) && (!(scratch.values[1850] != 0.0))) && (scratch.values[1851] != 0.0)) && (scratch.values[1852] != 0.0)) {
            scratch.store_ad(1356, &AdValue::mul(AdValue::mul(AdValue::mul(AdValue::mul(scratch.ad_value(1355), scratch.ad_value(647)), AdValue::mul(scratch.ad_value(1355), scratch.ad_value(647))), AdValue::mul(scratch.ad_value(1355), scratch.ad_value(647))), AdValue::mul(scratch.ad_value(1355), scratch.ad_value(647))));
        }

        if ((((((scratch.values[1320] != 0.0) && (scratch.values[1337] != 0.0)) && (!(scratch.values[1835] != 0.0))) && (!(scratch.values[1850] != 0.0))) && (scratch.values[1851] != 0.0)) && (!(scratch.values[1852] != 0.0))) {
            scratch.store_ad(1356, &AdValue::pow(AdValue::abs(AdValue::mul(scratch.ad_value(1355), scratch.ad_value(647))), scratch.ad_value(582)));
        }

        if (((((scratch.values[1320] != 0.0) && (scratch.values[1337] != 0.0)) && (!(scratch.values[1835] != 0.0))) && (!(scratch.values[1850] != 0.0))) && (scratch.values[1851] != 0.0)) {
            scratch.store_ad(1382, &AdValue::div_from_scalar(1.0, AdValue::sub_from_scalar(1.0, scratch.ad_value(1356))));
        }

        if (((((scratch.values[1320] != 0.0) && (scratch.values[1337] != 0.0)) && (!(scratch.values[1835] != 0.0))) && (!(scratch.values[1850] != 0.0))) && (!(scratch.values[1851] != 0.0))) {
            scratch.store_ad(1382, &AdValue::add(scratch.ad_value(644), AdValue::mul(AdValue::add(scratch.ad_value(1355), AdValue::scale(scratch.ad_value(579), scratch.values[500])), scratch.ad_value(650))));
        }

        if (((scratch.values[1320] != 0.0) && (scratch.values[1337] != 0.0)) && (!(scratch.values[1835] != 0.0))) {
            scratch.store_ad(1384, &AdValue::mul(AdValue::add(AdValue::add(AdValue::add(scratch.ad_value(1357), scratch.ad_value(1358)), scratch.ad_value(1365)), scratch.ad_value(1380)), scratch.ad_value(1382)));
        }

        scratch.values[1853] = if (scratch.values[726] == 0.0) { 1.0 } else { 0.0 };

        if (((scratch.values[1320] != 0.0) && (scratch.values[1337] != 0.0)) && (scratch.values[1853] != 0.0)) {
            scratch.values[1385] = 0.0;
            scratch.node_derivatives[1385] = [0.0; Instance::NODE_COUNT];
            scratch.branch_derivatives[1385] = [0.0; Instance::BRANCH_COUNT];
        }

        scratch.values[1854] = if (scratch.values[609] == 0.5) { 1.0 } else { 0.0 };

        if ((((scratch.values[1320] != 0.0) && (scratch.values[1337] != 0.0)) && (!(scratch.values[1853] != 0.0))) && (scratch.values[1854] != 0.0)) {
            scratch.store_ad(1356, &AdValue::sqrt(AdValue::sub_from_scalar(1.0, AdValue::mul(scratch.ad_value(1348), scratch.ad_value(606)))));
        }

        if ((((scratch.values[1320] != 0.0) && (scratch.values[1337] != 0.0)) && (!(scratch.values[1853] != 0.0))) && (!(scratch.values[1854] != 0.0))) {
            scratch.store_ad(1356, &AdValue::pow(AdValue::sub_from_scalar(1.0, AdValue::mul(scratch.ad_value(1348), scratch.ad_value(606))), scratch.ad_value(609)));
        }

        if (((scratch.values[1320] != 0.0) && (scratch.values[1337] != 0.0)) && (!(scratch.values[1853] != 0.0))) {
            scratch.store_ad(1357, &AdValue::mul(scratch.ad_value(597), scratch.ad_value(1347)));
        }

        scratch.values[1855] = if ((scratch.values[562] == 0.0) && (scratch.values[565] == 0.0)) { 1.0 } else { 0.0 };

        if ((((scratch.values[1320] != 0.0) && (scratch.values[1337] != 0.0)) && (!(scratch.values[1853] != 0.0))) && (scratch.values[1855] != 0.0)) {
            scratch.values[1358] = 0.0;
            scratch.node_derivatives[1358] = [0.0; Instance::NODE_COUNT];
            scratch.branch_derivatives[1358] = [0.0; Instance::BRANCH_COUNT];
        }

        if ((((scratch.values[1320] != 0.0) && (scratch.values[1337] != 0.0)) && (!(scratch.values[1853] != 0.0))) && (!(scratch.values[1855] != 0.0))) {
            scratch.store_ad(1359, &AdValue::sub(scratch.ad_value(603), scratch.ad_value(1353)));
        }

        if ((((scratch.values[1320] != 0.0) && (scratch.values[1337] != 0.0)) && (!(scratch.values[1853] != 0.0))) && (!(scratch.values[1855] != 0.0))) {
            scratch.store_ad(1360, &AdValue::sub_from_scalar(1.0, AdValue::sqrt(AdValue::sub_from_scalar(1.0, AdValue::div(scratch.ad_value(1351), scratch.ad_value(1359))))));
        }

        scratch.values[1856] = if (scratch.values[551] == 0.5) { 1.0 } else { 0.0 };

        if (((((scratch.values[1320] != 0.0) && (scratch.values[1337] != 0.0)) && (!(scratch.values[1853] != 0.0))) && (!(scratch.values[1855] != 0.0))) && (scratch.values[1856] != 0.0)) {
            scratch.values[1361] = 0.0;
            scratch.node_derivatives[1361] = [0.0; Instance::NODE_COUNT];
            scratch.branch_derivatives[1361] = [0.0; Instance::BRANCH_COUNT];
        }

        if (((((scratch.values[1320] != 0.0) && (scratch.values[1337] != 0.0)) && (!(scratch.values[1853] != 0.0))) && (!(scratch.values[1855] != 0.0))) && (!(scratch.values[1856] != 0.0))) {
            scratch.store_ad(1361, &AdValue::mul(AdValue::add(AdValue::div(AdValue::mul(AdValue::square(scratch.ad_value(1360)), AdValue::ln(scratch.ad_value(1360))), AdValue::sub_from_scalar(1.0, scratch.ad_value(1360))), scratch.ad_value(1360)), AdValue::sub_from_scalar(1.0, AdValue::scale(scratch.ad_value(551), 2.0))));
        }

        if ((((scratch.values[1320] != 0.0) && (scratch.values[1337] != 0.0)) && (!(scratch.values[1853] != 0.0))) && (!(scratch.values[1855] != 0.0))) {
            scratch.store_ad(1362, &AdValue::add(scratch.ad_value(1360), scratch.ad_value(1361)));
        }

        scratch.values[1857] = if (scratch.values[551] == 0.5) { 1.0 } else { 0.0 };

        if (((((scratch.values[1320] != 0.0) && (scratch.values[1337] != 0.0)) && (!(scratch.values[1853] != 0.0))) && (!(scratch.values[1855] != 0.0))) && (scratch.values[1857] != 0.0)) {
            scratch.store_ad(1356, &AdValue::sqrt(AdValue::mul(scratch.ad_value(1359), scratch.ad_value(630))));
        }

        if (((((scratch.values[1320] != 0.0) && (scratch.values[1337] != 0.0)) && (!(scratch.values[1853] != 0.0))) && (!(scratch.values[1855] != 0.0))) && (!(scratch.values[1857] != 0.0))) {
            scratch.store_ad(1356, &AdValue::pow(AdValue::mul(scratch.ad_value(1359), scratch.ad_value(630)), scratch.ad_value(551)));
        }

        if ((((scratch.values[1320] != 0.0) && (scratch.values[1337] != 0.0)) && (!(scratch.values[1853] != 0.0))) && (!(scratch.values[1855] != 0.0))) {
            scratch.store_ad(1363, &AdValue::mul(scratch.ad_value(624), scratch.ad_value(1356)));
        }

        if ((((scratch.values[1320] != 0.0) && (scratch.values[1337] != 0.0)) && (!(scratch.values[1853] != 0.0))) && (!(scratch.values[1855] != 0.0))) {
            scratch.store_ad(1364, &AdValue::mul(scratch.ad_value(594), AdValue::mul(AdValue::offset(scratch.ad_value(1350), (-1.0)), scratch.ad_value(1363))));
        }

        if ((((scratch.values[1320] != 0.0) && (scratch.values[1337] != 0.0)) && (!(scratch.values[1853] != 0.0))) && (!(scratch.values[1855] != 0.0))) {
            scratch.store_ad(1358, &AdValue::mul(scratch.ad_value(562), AdValue::mul(scratch.ad_value(1364), scratch.ad_value(1362))));
        }

        scratch.values[1858] = if (scratch.values[565] == 0.0) { 1.0 } else { 0.0 };

        if ((((scratch.values[1320] != 0.0) && (scratch.values[1337] != 0.0)) && (!(scratch.values[1853] != 0.0))) && (scratch.values[1858] != 0.0)) {
            scratch.values[1365] = 0.0;
            scratch.node_derivatives[1365] = [0.0; Instance::NODE_COUNT];
            scratch.branch_derivatives[1365] = [0.0; Instance::BRANCH_COUNT];
        }

        if ((((scratch.values[1320] != 0.0) && (scratch.values[1337] != 0.0)) && (!(scratch.values[1853] != 0.0))) && (!(scratch.values[1858] != 0.0))) {
            scratch.store_ad(1366, &AdValue::mul(scratch.ad_value(639), AdValue::div(AdValue::mul(scratch.ad_value(1363), scratch.ad_value(609)), scratch.ad_value(1359))));
        }

        if ((((scratch.values[1320] != 0.0) && (scratch.values[1337] != 0.0)) && (!(scratch.values[1853] != 0.0))) && (!(scratch.values[1858] != 0.0))) {
            scratch.store_ad(1367, &AdValue::div(AdValue::scale(scratch.ad_value(636), 0.666666666666667), scratch.ad_value(1366)));
        }

        if ((((scratch.values[1320] != 0.0) && (scratch.values[1337] != 0.0)) && (!(scratch.values[1853] != 0.0))) && (!(scratch.values[1858] != 0.0))) {
            scratch.store_ad(1368, &AdValue::square(scratch.ad_value(1367)));
        }

        if ((((scratch.values[1320] != 0.0) && (scratch.values[1337] != 0.0)) && (!(scratch.values[1853] != 0.0))) && (!(scratch.values[1858] != 0.0))) {
            scratch.store_ad(1369, &AdValue::sqrt(AdValue::div(AdValue::square(scratch.ad_value(1368)), AdValue::offset(AdValue::square(scratch.ad_value(1368)), 1.0))));
        }

        if ((((scratch.values[1320] != 0.0) && (scratch.values[1337] != 0.0)) && (!(scratch.values[1853] != 0.0))) && (!(scratch.values[1858] != 0.0))) {
            scratch.store_ad(1370, &AdValue::sqrt(AdValue::abs(scratch.ad_value(1369))));
        }

        if ((((scratch.values[1320] != 0.0) && (scratch.values[1337] != 0.0)) && (!(scratch.values[1853] != 0.0))) && (!(scratch.values[1858] != 0.0))) {
            scratch.store_ad(1371, &AdValue::mul(scratch.ad_value(1369), scratch.ad_value(1370)));
        }

        scratch.values[1859] = if (((-scratch.values[551]) * scratch.values[612]) == (-1.0)) { 1.0 } else { 0.0 };

        if (((((scratch.values[1320] != 0.0) && (scratch.values[1337] != 0.0)) && (!(scratch.values[1853] != 0.0))) && (!(scratch.values[1858] != 0.0))) && (scratch.values[1859] != 0.0)) {
            scratch.store_ad(1372, &AdValue::div_from_scalar(1.0, AdValue::offset(AdValue::mul(scratch.ad_value(1366), scratch.ad_value(1371)), 1.0)));
        }

        if (((((scratch.values[1320] != 0.0) && (scratch.values[1337] != 0.0)) && (!(scratch.values[1853] != 0.0))) && (!(scratch.values[1858] != 0.0))) && (!(scratch.values[1859] != 0.0))) {
            scratch.store_ad(1372, &AdValue::pow(AdValue::offset(AdValue::mul(scratch.ad_value(1366), scratch.ad_value(1371)), 1.0), AdValue::mul(AdValue::neg(scratch.ad_value(551)), scratch.ad_value(612))));
        }

        if ((((scratch.values[1320] != 0.0) && (scratch.values[1337] != 0.0)) && (!(scratch.values[1853] != 0.0))) && (!(scratch.values[1858] != 0.0))) {
            scratch.store_ad(1373, &AdValue::div(AdValue::mul(scratch.ad_value(1362), scratch.ad_value(1372)), AdValue::add(scratch.ad_value(1362), scratch.ad_value(1372))));
        }

        if ((((scratch.values[1320] != 0.0) && (scratch.values[1337] != 0.0)) && (!(scratch.values[1853] != 0.0))) && (!(scratch.values[1858] != 0.0))) {
            scratch.store_ad(1374, &AdValue::sqrt(AdValue::scale(AdValue::div(scratch.ad_value(1366), scratch.ad_value(1370)), 0.375)));
        }

        if ((((scratch.values[1320] != 0.0) && (scratch.values[1337] != 0.0)) && (!(scratch.values[1853] != 0.0))) && (!(scratch.values[1858] != 0.0))) {
            scratch.store_ad(1375, &AdValue::sub(AdValue::scale(AdValue::mul(scratch.ad_value(1367), scratch.ad_value(1370)), 2.0), scratch.ad_value(1369)));
        }

        if ((((scratch.values[1320] != 0.0) && (scratch.values[1337] != 0.0)) && (!(scratch.values[1853] != 0.0))) && (!(scratch.values[1858] != 0.0))) {
            scratch.store_ad(1376, &AdValue::add(AdValue::sub(AdValue::mul(AdValue::mul(scratch.ad_value(636), scratch.ad_value(1367)), scratch.ad_value(1370)), AdValue::mul(scratch.ad_value(636), scratch.ad_value(1369))), AdValue::scale(AdValue::mul(scratch.ad_value(1366), scratch.ad_value(1371)), 0.5)));
        }

        if ((((scratch.values[1320] != 0.0) && (scratch.values[1337] != 0.0)) && (!(scratch.values[1853] != 0.0))) && (!(scratch.values[1858] != 0.0))) {
            scratch.store_ad(1377, &AdValue::mul(AdValue::offset(scratch.ad_value(1375), (-1.0)), scratch.ad_value(1374)));
        }

        if ((((scratch.values[1320] != 0.0) && (scratch.values[1337] != 0.0)) && (!(scratch.values[1853] != 0.0))) && (!(scratch.values[1858] != 0.0))) {
            scratch.store_ad(1338, &AdValue::square(scratch.ad_value(1377)));
        }

        scratch.values[1860] = if (scratch.values[1377] > 0.0) { 1.0 } else { 0.0 };

        if (((((scratch.values[1320] != 0.0) && (scratch.values[1337] != 0.0)) && (!(scratch.values[1853] != 0.0))) && (!(scratch.values[1858] != 0.0))) && (scratch.values[1860] != 0.0)) {
            scratch.store_ad(1339, &AdValue::div_from_scalar(1.0, AdValue::offset(AdValue::scale(scratch.ad_value(1377), scratch.values[428]), 1.0)));
        }

        if (((((scratch.values[1320] != 0.0) && (scratch.values[1337] != 0.0)) && (!(scratch.values[1853] != 0.0))) && (!(scratch.values[1858] != 0.0))) && (!(scratch.values[1860] != 0.0))) {
            scratch.store_ad(1339, &AdValue::div_from_scalar(1.0, AdValue::sub_from_scalar(1.0, AdValue::scale(scratch.ad_value(1377), scratch.values[428]))));
        }

        scratch.values[1861] = if (((-scratch.values[1338]) + scratch.values[1376]) > (-230.25850929940458)) { 1.0 } else { 0.0 };

        if (((((scratch.values[1320] != 0.0) && (scratch.values[1337] != 0.0)) && (!(scratch.values[1853] != 0.0))) && (!(scratch.values[1858] != 0.0))) && (scratch.values[1861] != 0.0)) {
            scratch.store_ad(1356, &AdValue::exp(AdValue::sub(scratch.ad_value(1376), scratch.ad_value(1338))));
        }

        if (((((scratch.values[1320] != 0.0) && (scratch.values[1337] != 0.0)) && (!(scratch.values[1853] != 0.0))) && (!(scratch.values[1858] != 0.0))) && (!(scratch.values[1861] != 0.0))) {
            let assign33390_ad_e42683: AdValue = AdValue::div_from_scalar(1e-100, AdValue::offset(AdValue::mul(AdValue::sub_from_scalar((-230.25850929940458), AdValue::sub(scratch.ad_value(1376), scratch.ad_value(1338))), AdValue::offset(AdValue::scale(AdValue::mul(AdValue::sub_from_scalar((-230.25850929940458), AdValue::sub(scratch.ad_value(1376), scratch.ad_value(1338))), AdValue::offset(AdValue::scale(AdValue::sub_from_scalar((-230.25850929940458), AdValue::sub(scratch.ad_value(1376), scratch.ad_value(1338))), 0.3333333333333333), 1.0)), 0.5), 1.0)), 1.0));
            scratch.store_ad(1356, &assign33390_ad_e42683);
        }

        if ((((scratch.values[1320] != 0.0) && (scratch.values[1337] != 0.0)) && (!(scratch.values[1853] != 0.0))) && (!(scratch.values[1858] != 0.0))) {
            scratch.store_ad(1340, &AdValue::mul(AdValue::add(AdValue::add(AdValue::scale(scratch.ad_value(1339), 0.29214664), AdValue::scale(AdValue::square(scratch.ad_value(1339)), scratch.values[429])), AdValue::scale(AdValue::mul(AdValue::square(scratch.ad_value(1339)), scratch.ad_value(1339)), scratch.values[430])), scratch.ad_value(1356)));
        }

        scratch.values[1862] = if (scratch.values[1377] > 0.0) { 1.0 } else { 0.0 };

        if (((((scratch.values[1320] != 0.0) && (scratch.values[1337] != 0.0)) && (!(scratch.values[1853] != 0.0))) && (!(scratch.values[1858] != 0.0))) && (scratch.values[1862] != 0.0)) {
            scratch.values[1378] = scratch.values[1340];
            scratch.node_derivatives[1378] = scratch.node_derivatives[1340];
            scratch.branch_derivatives[1378] = scratch.branch_derivatives[1340];
        }

        scratch.values[1863] = if (scratch.values[1376] > (-230.25850929940458)) { 1.0 } else { 0.0 };

        if ((((((scratch.values[1320] != 0.0) && (scratch.values[1337] != 0.0)) && (!(scratch.values[1853] != 0.0))) && (!(scratch.values[1858] != 0.0))) && (!(scratch.values[1862] != 0.0))) && (scratch.values[1863] != 0.0)) {
            scratch.store_ad(1356, &AdValue::exp(scratch.ad_value(1376)));
        }

        if ((((((scratch.values[1320] != 0.0) && (scratch.values[1337] != 0.0)) && (!(scratch.values[1853] != 0.0))) && (!(scratch.values[1858] != 0.0))) && (!(scratch.values[1862] != 0.0))) && (!(scratch.values[1863] != 0.0))) {
            scratch.store_ad(1356, &AdValue::div_from_scalar(1e-100, AdValue::offset(AdValue::mul(AdValue::sub_from_scalar((-230.25850929940458), scratch.ad_value(1376)), AdValue::offset(AdValue::scale(AdValue::mul(AdValue::sub_from_scalar((-230.25850929940458), scratch.ad_value(1376)), AdValue::offset(AdValue::scale(AdValue::sub_from_scalar((-230.25850929940458), scratch.ad_value(1376)), 0.3333333333333333), 1.0)), 0.5), 1.0)), 1.0)));
        }

        if (((((scratch.values[1320] != 0.0) && (scratch.values[1337] != 0.0)) && (!(scratch.values[1853] != 0.0))) && (!(scratch.values[1858] != 0.0))) && (!(scratch.values[1862] != 0.0))) {
            scratch.store_ad(1378, &AdValue::sub(AdValue::scale(scratch.ad_value(1356), 2.0), scratch.ad_value(1340)));
        }

        if ((((scratch.values[1320] != 0.0) && (scratch.values[1337] != 0.0)) && (!(scratch.values[1853] != 0.0))) && (!(scratch.values[1858] != 0.0))) {
            scratch.store_ad(1379, &AdValue::scale(AdValue::div(AdValue::mul(scratch.ad_value(636), scratch.ad_value(1378)), scratch.ad_value(1374)), (1.772453850905516 * 0.5)));
        }

        if ((((scratch.values[1320] != 0.0) && (scratch.values[1337] != 0.0)) && (!(scratch.values[1853] != 0.0))) && (!(scratch.values[1858] != 0.0))) {
            scratch.store_ad(1365, &AdValue::mul(scratch.ad_value(565), AdValue::mul(AdValue::mul(scratch.ad_value(1364), scratch.ad_value(1379)), scratch.ad_value(1373))));
        }

        scratch.values[1864] = if (scratch.values[571] == 0.0) { 1.0 } else { 0.0 };

        if ((((scratch.values[1320] != 0.0) && (scratch.values[1337] != 0.0)) && (!(scratch.values[1853] != 0.0))) && (scratch.values[1864] != 0.0)) {
            scratch.values[1380] = 0.0;
            scratch.node_derivatives[1380] = [0.0; Instance::NODE_COUNT];
            scratch.branch_derivatives[1380] = [0.0; Instance::BRANCH_COUNT];
        }

        scratch.values[1865] = if (scratch.values[551] == 0.5) { 1.0 } else { 0.0 };

        if (((((scratch.values[1320] != 0.0) && (scratch.values[1337] != 0.0)) && (!(scratch.values[1853] != 0.0))) && (!(scratch.values[1864] != 0.0))) && (scratch.values[1865] != 0.0)) {
            scratch.store_ad(1356, &AdValue::sqrt(AdValue::mul(AdValue::sub(scratch.ad_value(548), scratch.ad_value(1354)), scratch.ad_value(630))));
        }

        if (((((scratch.values[1320] != 0.0) && (scratch.values[1337] != 0.0)) && (!(scratch.values[1853] != 0.0))) && (!(scratch.values[1864] != 0.0))) && (!(scratch.values[1865] != 0.0))) {
            scratch.store_ad(1356, &AdValue::pow(AdValue::mul(AdValue::sub(scratch.ad_value(548), scratch.ad_value(1354)), scratch.ad_value(630)), scratch.ad_value(551)));
        }

        if ((((scratch.values[1320] != 0.0) && (scratch.values[1337] != 0.0)) && (!(scratch.values[1853] != 0.0))) && (!(scratch.values[1864] != 0.0))) {
            scratch.store_ad(1381, &AdValue::mul(scratch.ad_value(612), AdValue::div(AdValue::mul(AdValue::sub(scratch.ad_value(548), scratch.ad_value(1354)), scratch.ad_value(627)), scratch.ad_value(1356))));
        }

        scratch.values[1866] = if (((((-scratch.values[642]) / scratch.values[1381])) as f64).abs() < 230.25850929940458) { 1.0 } else { 0.0 };

        if (((((scratch.values[1320] != 0.0) && (scratch.values[1337] != 0.0)) && (!(scratch.values[1853] != 0.0))) && (!(scratch.values[1864] != 0.0))) && (scratch.values[1866] != 0.0)) {
            scratch.store_ad(1356, &AdValue::exp(AdValue::div(AdValue::neg(scratch.ad_value(642)), scratch.ad_value(1381))));
        }

        scratch.values[1867] = if (((-scratch.values[642]) / scratch.values[1381]) < 0.0) { 1.0 } else { 0.0 };

        if ((((((scratch.values[1320] != 0.0) && (scratch.values[1337] != 0.0)) && (!(scratch.values[1853] != 0.0))) && (!(scratch.values[1864] != 0.0))) && (!(scratch.values[1866] != 0.0))) && (scratch.values[1867] != 0.0)) {
            let assign33580_ad_e43010: AdValue = AdValue::offset(AdValue::mul(AdValue::sub_from_scalar((-230.25850929940458), AdValue::div(AdValue::neg(scratch.ad_value(642)), scratch.ad_value(1381))), AdValue::offset(AdValue::scale(AdValue::mul(AdValue::sub_from_scalar((-230.25850929940458), AdValue::div(AdValue::neg(scratch.ad_value(642)), scratch.ad_value(1381))), AdValue::offset(AdValue::scale(AdValue::sub_from_scalar((-230.25850929940458), AdValue::div(AdValue::neg(scratch.ad_value(642)), scratch.ad_value(1381))), 0.3333333333333333), 1.0)), 0.5), 1.0)), 1.0);
            scratch.store_ad(1356, &AdValue::div_from_scalar(1e-100, assign33580_ad_e43010));
        }

        if ((((((scratch.values[1320] != 0.0) && (scratch.values[1337] != 0.0)) && (!(scratch.values[1853] != 0.0))) && (!(scratch.values[1864] != 0.0))) && (!(scratch.values[1866] != 0.0))) && (!(scratch.values[1867] != 0.0))) {
            let assign33590_ad_e43060: AdValue = AdValue::scale(AdValue::offset(AdValue::mul(AdValue::offset(AdValue::div(AdValue::neg(scratch.ad_value(642)), scratch.ad_value(1381)), (-230.25850929940458)), AdValue::offset(AdValue::scale(AdValue::mul(AdValue::offset(AdValue::div(AdValue::neg(scratch.ad_value(642)), scratch.ad_value(1381)), (-230.25850929940458)), AdValue::offset(AdValue::scale(AdValue::offset(AdValue::div(AdValue::neg(scratch.ad_value(642)), scratch.ad_value(1381)), (-230.25850929940458)), 0.3333333333333333), 1.0)), 0.5), 1.0)), 1.0), 1e100);
            scratch.store_ad(1356, &assign33590_ad_e43060);
        }

        if ((((scratch.values[1320] != 0.0) && (scratch.values[1337] != 0.0)) && (!(scratch.values[1853] != 0.0))) && (!(scratch.values[1864] != 0.0))) {
            scratch.store_ad(1380, &AdValue::mul(scratch.ad_value(571), AdValue::mul(AdValue::mul(AdValue::mul(scratch.ad_value(525), scratch.ad_value(1381)), scratch.ad_value(1381)), scratch.ad_value(1356))));
        }

        scratch.values[1868] = if (scratch.values[580] > 1000.0) { 1.0 } else { 0.0 };

        if ((((scratch.values[1320] != 0.0) && (scratch.values[1337] != 0.0)) && (!(scratch.values[1853] != 0.0))) && (scratch.values[1868] != 0.0)) {
            scratch.values[1382] = 1.0;
            scratch.node_derivatives[1382] = [0.0; Instance::NODE_COUNT];
            scratch.branch_derivatives[1382] = [0.0; Instance::BRANCH_COUNT];
        }

        scratch.values[1869] = if (scratch.values[1355] > ((-scratch.values[500]) * scratch.values[580])) { 1.0 } else { 0.0 };

        scratch.values[1870] = if (scratch.values[583] == 4.0) { 1.0 } else { 0.0 };

        if ((((((scratch.values[1320] != 0.0) && (scratch.values[1337] != 0.0)) && (!(scratch.values[1853] != 0.0))) && (!(scratch.values[1868] != 0.0))) && (scratch.values[1869] != 0.0)) && (scratch.values[1870] != 0.0)) {
            scratch.store_ad(1356, &AdValue::mul(AdValue::mul(AdValue::mul(AdValue::mul(scratch.ad_value(1355), scratch.ad_value(648)), AdValue::mul(scratch.ad_value(1355), scratch.ad_value(648))), AdValue::mul(scratch.ad_value(1355), scratch.ad_value(648))), AdValue::mul(scratch.ad_value(1355), scratch.ad_value(648))));
        }

        if ((((((scratch.values[1320] != 0.0) && (scratch.values[1337] != 0.0)) && (!(scratch.values[1853] != 0.0))) && (!(scratch.values[1868] != 0.0))) && (scratch.values[1869] != 0.0)) && (!(scratch.values[1870] != 0.0))) {
            scratch.store_ad(1356, &AdValue::pow(AdValue::abs(AdValue::mul(scratch.ad_value(1355), scratch.ad_value(648))), scratch.ad_value(583)));
        }

        if (((((scratch.values[1320] != 0.0) && (scratch.values[1337] != 0.0)) && (!(scratch.values[1853] != 0.0))) && (!(scratch.values[1868] != 0.0))) && (scratch.values[1869] != 0.0)) {
            scratch.store_ad(1382, &AdValue::div_from_scalar(1.0, AdValue::sub_from_scalar(1.0, scratch.ad_value(1356))));
        }

        if (((((scratch.values[1320] != 0.0) && (scratch.values[1337] != 0.0)) && (!(scratch.values[1853] != 0.0))) && (!(scratch.values[1868] != 0.0))) && (!(scratch.values[1869] != 0.0))) {
            scratch.store_ad(1382, &AdValue::add(scratch.ad_value(645), AdValue::mul(AdValue::add(scratch.ad_value(1355), AdValue::scale(scratch.ad_value(580), scratch.values[500])), scratch.ad_value(651))));
        }

        if (((scratch.values[1320] != 0.0) && (scratch.values[1337] != 0.0)) && (!(scratch.values[1853] != 0.0))) {
            scratch.store_ad(1385, &AdValue::mul(AdValue::add(AdValue::add(AdValue::add(scratch.ad_value(1357), scratch.ad_value(1358)), scratch.ad_value(1365)), scratch.ad_value(1380)), scratch.ad_value(1382)));
        }

        if ((scratch.values[1320] != 0.0) && (scratch.values[1337] != 0.0)) {
            scratch.store_ad(515, &AdValue::add(AdValue::add(AdValue::mul(scratch.ad_value(724), scratch.ad_value(1383)), AdValue::mul(scratch.ad_value(725), scratch.ad_value(1384))), AdValue::mul(scratch.ad_value(726), scratch.ad_value(1385))));
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

        scratch.values[1871] = if !(((scratch.values[724] == 0.0) && (scratch.values[725] == 0.0)) && (scratch.values[726] == 0.0)) { 1.0 } else { 0.0 };

        if (((scratch.values[1320] != 0.0) && (scratch.values[1337] != 0.0)) && (scratch.values[1871] != 0.0)) {
            scratch.store_ad(1341, &AdValue::mul(AdValue::scale(scratch.ad_value(735), 4.0), scratch.ad_value(735)));
        }

        if (((scratch.values[1320] != 0.0) && (scratch.values[1337] != 0.0)) && (scratch.values[1871] != 0.0)) {
            scratch.store_ad(1342, &AdValue::div(scratch.ad_value(735), scratch.ad_value(736)));
        }

        if (((scratch.values[1320] != 0.0) && (scratch.values[1337] != 0.0)) && (scratch.values[1871] != 0.0)) {
            scratch.store_ad(1343, &AdValue::add(scratch.ad_value(526), AdValue::mul(scratch.ad_value(735), scratch.ad_value(1342))));
        }

        if (((scratch.values[1320] != 0.0) && (scratch.values[1337] != 0.0)) && (scratch.values[1871] != 0.0)) {
            scratch.store_ad(1344, &AdValue::add(scratch.ad_value(736), scratch.ad_value(1343)));
        }

        if (((scratch.values[1320] != 0.0) && (scratch.values[1337] != 0.0)) && (scratch.values[1871] != 0.0)) {
            scratch.store_ad(1345, &AdValue::sub(scratch.ad_value(736), scratch.ad_value(1343)));
        }

        if (((scratch.values[1320] != 0.0) && (scratch.values[1337] != 0.0)) && (scratch.values[1871] != 0.0)) {
            scratch.store_ad(1346, &AdValue::sqrt(AdValue::add(AdValue::square(scratch.ad_value(1345)), scratch.ad_value(1341))));
        }

        if (((scratch.values[1320] != 0.0) && (scratch.values[1337] != 0.0)) && (scratch.values[1871] != 0.0)) {
            scratch.store_ad(1348, &AdValue::scale(AdValue::div(AdValue::mul(scratch.ad_value(526), scratch.ad_value(736)), AdValue::add(scratch.ad_value(1344), scratch.ad_value(1346))), 2.0));
        }

        scratch.values[1872] = if (scratch.values[526] < scratch.values[732]) { 1.0 } else { 0.0 };

        scratch.values[1873] = if ((((0.5 * (scratch.values[526] * scratch.values[427]))) as f64).abs() < 230.25850929940458) { 1.0 } else { 0.0 };

        if (((((scratch.values[1320] != 0.0) && (scratch.values[1337] != 0.0)) && (scratch.values[1871] != 0.0)) && (scratch.values[1872] != 0.0)) && (scratch.values[1873] != 0.0)) {
            scratch.store_ad(1350, &AdValue::exp(AdValue::scale(scratch.ad_value(526), (scratch.values[427] * 0.5))));
        }

        scratch.values[1874] = if ((0.5 * (scratch.values[526] * scratch.values[427])) < 0.0) { 1.0 } else { 0.0 };

        if ((((((scratch.values[1320] != 0.0) && (scratch.values[1337] != 0.0)) && (scratch.values[1871] != 0.0)) && (scratch.values[1872] != 0.0)) && (!(scratch.values[1873] != 0.0))) && (scratch.values[1874] != 0.0)) {
            let assign33850_ad_e43423: AdValue = AdValue::div_from_scalar(1e-100, AdValue::offset(AdValue::mul(AdValue::sub_from_scalar((-230.25850929940458), AdValue::scale(scratch.ad_value(526), (scratch.values[427] * 0.5))), AdValue::offset(AdValue::scale(AdValue::mul(AdValue::sub_from_scalar((-230.25850929940458), AdValue::scale(scratch.ad_value(526), (scratch.values[427] * 0.5))), AdValue::offset(AdValue::scale(AdValue::sub_from_scalar((-230.25850929940458), AdValue::scale(scratch.ad_value(526), (scratch.values[427] * 0.5))), 0.3333333333333333), 1.0)), 0.5), 1.0)), 1.0));
            scratch.store_ad(1350, &assign33850_ad_e43423);
        }

        if ((((((scratch.values[1320] != 0.0) && (scratch.values[1337] != 0.0)) && (scratch.values[1871] != 0.0)) && (scratch.values[1872] != 0.0)) && (!(scratch.values[1873] != 0.0))) && (!(scratch.values[1874] != 0.0))) {
            scratch.store_ad(1350, &AdValue::scale(AdValue::offset(AdValue::mul(AdValue::offset(AdValue::scale(scratch.ad_value(526), (scratch.values[427] * 0.5)), (-230.25850929940458)), AdValue::offset(AdValue::scale(AdValue::mul(AdValue::offset(AdValue::scale(scratch.ad_value(526), (scratch.values[427] * 0.5)), (-230.25850929940458)), AdValue::offset(AdValue::scale(AdValue::offset(AdValue::scale(scratch.ad_value(526), (scratch.values[427] * 0.5)), (-230.25850929940458)), 0.3333333333333333), 1.0)), 0.5), 1.0)), 1.0), 1e100));
        }

        if ((((scratch.values[1320] != 0.0) && (scratch.values[1337] != 0.0)) && (scratch.values[1871] != 0.0)) && (scratch.values[1872] != 0.0)) {
            scratch.store_ad(1347, &AdValue::square(scratch.ad_value(1350)));
        }

        if ((((scratch.values[1320] != 0.0) && (scratch.values[1337] != 0.0)) && (scratch.values[1871] != 0.0)) && (!(scratch.values[1872] != 0.0))) {
            scratch.store_ad(1347, &AdValue::mul(AdValue::offset(AdValue::scale(AdValue::sub(scratch.ad_value(526), scratch.ad_value(732)), scratch.values[427]), 1.0), scratch.ad_value(733)));
        }

        if ((((scratch.values[1320] != 0.0) && (scratch.values[1337] != 0.0)) && (scratch.values[1871] != 0.0)) && (!(scratch.values[1872] != 0.0))) {
            scratch.store_ad(1350, &AdValue::sqrt(scratch.ad_value(1347)));
        }

        if (((scratch.values[1320] != 0.0) && (scratch.values[1337] != 0.0)) && (scratch.values[1871] != 0.0)) {
            scratch.store_ad(1347, &AdValue::offset(scratch.ad_value(1347), (-1.0)));
        }

        if (((scratch.values[1320] != 0.0) && (scratch.values[1337] != 0.0)) && (scratch.values[1871] != 0.0)) {
            scratch.store_ad(1349, &AdValue::div_from_scalar(1.0, scratch.ad_value(1350)));
        }

        scratch.values[1875] = if (scratch.values[526] > 0.0) { 1.0 } else { 0.0 };

        if ((((scratch.values[1320] != 0.0) && (scratch.values[1337] != 0.0)) && (scratch.values[1871] != 0.0)) && (scratch.values[1875] != 0.0)) {
            scratch.store_ad(1351, &AdValue::scale(AdValue::ln(AdValue::add(AdValue::offset(scratch.ad_value(1349), 2.0), AdValue::sqrt(AdValue::mul(AdValue::offset(scratch.ad_value(1349), 1.0), AdValue::offset(scratch.ad_value(1349), 3.0))))), (scratch.values[426] * 2.0)));
        }

        if ((((scratch.values[1320] != 0.0) && (scratch.values[1337] != 0.0)) && (scratch.values[1871] != 0.0)) && (!(scratch.values[1875] != 0.0))) {
            scratch.store_ad(1351, &AdValue::sub(AdValue::scale(AdValue::ln(AdValue::add(AdValue::offset(AdValue::scale(scratch.ad_value(1350), 2.0), 1.0), AdValue::sqrt(AdValue::mul(AdValue::offset(scratch.ad_value(1350), 1.0), AdValue::offset(AdValue::scale(scratch.ad_value(1350), 3.0), 1.0))))), (scratch.values[426] * 2.0)), scratch.ad_value(526)));
        }

        if (((scratch.values[1320] != 0.0) && (scratch.values[1337] != 0.0)) && (scratch.values[1871] != 0.0)) {
            scratch.store_ad(1352, &AdValue::sub(scratch.ad_value(734), scratch.ad_value(1351)));
        }

        if (((scratch.values[1320] != 0.0) && (scratch.values[1337] != 0.0)) && (scratch.values[1871] != 0.0)) {
            scratch.store_ad(1353, &AdValue::scale(AdValue::sub(AdValue::add(scratch.ad_value(526), scratch.ad_value(1352)), AdValue::sqrt(AdValue::offset(AdValue::mul(AdValue::sub(scratch.ad_value(526), scratch.ad_value(1352)), AdValue::sub(scratch.ad_value(526), scratch.ad_value(1352))), ((4.0 * scratch.values[426]) * scratch.values[426])))), 0.5));
        }

        if (((scratch.values[1320] != 0.0) && (scratch.values[1337] != 0.0)) && (scratch.values[1871] != 0.0)) {
            scratch.store_ad(1354, &AdValue::scale(AdValue::sub(AdValue::add(scratch.ad_value(526), scratch.ad_value(737)), AdValue::sqrt(AdValue::offset(AdValue::mul(AdValue::sub(scratch.ad_value(526), scratch.ad_value(737)), AdValue::sub(scratch.ad_value(526), scratch.ad_value(737))), ((4.0 * scratch.values[424]) * scratch.values[424])))), 0.5));
        }

        if (((scratch.values[1320] != 0.0) && (scratch.values[1337] != 0.0)) && (scratch.values[1871] != 0.0)) {
            scratch.store_ad(1355, &AdValue::scale(AdValue::sub(scratch.ad_value(526), AdValue::sqrt(AdValue::offset(AdValue::mul(scratch.ad_value(526), scratch.ad_value(526)), ((4.0 * 1e-6) * 1e-6)))), 0.5));
        }

        scratch.values[1876] = if (scratch.values[724] == 0.0) { 1.0 } else { 0.0 };

        if (((scratch.values[1320] != 0.0) && (scratch.values[1337] != 0.0)) && (scratch.values[1876] != 0.0)) {
            scratch.values[1383] = 0.0;
            scratch.node_derivatives[1383] = [0.0; Instance::NODE_COUNT];
            scratch.branch_derivatives[1383] = [0.0; Instance::BRANCH_COUNT];
        }

        scratch.values[1877] = if (scratch.values[607] == 0.5) { 1.0 } else { 0.0 };

        if ((((scratch.values[1320] != 0.0) && (scratch.values[1337] != 0.0)) && (!(scratch.values[1876] != 0.0))) && (scratch.values[1877] != 0.0)) {
            scratch.store_ad(1356, &AdValue::sqrt(AdValue::sub_from_scalar(1.0, AdValue::mul(scratch.ad_value(1348), scratch.ad_value(604)))));
        }

        if ((((scratch.values[1320] != 0.0) && (scratch.values[1337] != 0.0)) && (!(scratch.values[1876] != 0.0))) && (!(scratch.values[1877] != 0.0))) {
            scratch.store_ad(1356, &AdValue::pow(AdValue::sub_from_scalar(1.0, AdValue::mul(scratch.ad_value(1348), scratch.ad_value(604))), scratch.ad_value(607)));
        }

        if (((scratch.values[1320] != 0.0) && (scratch.values[1337] != 0.0)) && (!(scratch.values[1876] != 0.0))) {
            scratch.store_ad(1357, &AdValue::mul(scratch.ad_value(595), scratch.ad_value(1347)));
        }

        scratch.values[1878] = if ((scratch.values[560] == 0.0) && (scratch.values[563] == 0.0)) { 1.0 } else { 0.0 };

        if ((((scratch.values[1320] != 0.0) && (scratch.values[1337] != 0.0)) && (!(scratch.values[1876] != 0.0))) && (scratch.values[1878] != 0.0)) {
            scratch.values[1358] = 0.0;
            scratch.node_derivatives[1358] = [0.0; Instance::NODE_COUNT];
            scratch.branch_derivatives[1358] = [0.0; Instance::BRANCH_COUNT];
        }

        if ((((scratch.values[1320] != 0.0) && (scratch.values[1337] != 0.0)) && (!(scratch.values[1876] != 0.0))) && (!(scratch.values[1878] != 0.0))) {
            scratch.store_ad(1359, &AdValue::sub(scratch.ad_value(601), scratch.ad_value(1353)));
        }

        if ((((scratch.values[1320] != 0.0) && (scratch.values[1337] != 0.0)) && (!(scratch.values[1876] != 0.0))) && (!(scratch.values[1878] != 0.0))) {
            scratch.store_ad(1360, &AdValue::sub_from_scalar(1.0, AdValue::sqrt(AdValue::sub_from_scalar(1.0, AdValue::div(scratch.ad_value(1351), scratch.ad_value(1359))))));
        }

        scratch.values[1879] = if (scratch.values[549] == 0.5) { 1.0 } else { 0.0 };

        if (((((scratch.values[1320] != 0.0) && (scratch.values[1337] != 0.0)) && (!(scratch.values[1876] != 0.0))) && (!(scratch.values[1878] != 0.0))) && (scratch.values[1879] != 0.0)) {
            scratch.values[1361] = 0.0;
            scratch.node_derivatives[1361] = [0.0; Instance::NODE_COUNT];
            scratch.branch_derivatives[1361] = [0.0; Instance::BRANCH_COUNT];
        }

        if (((((scratch.values[1320] != 0.0) && (scratch.values[1337] != 0.0)) && (!(scratch.values[1876] != 0.0))) && (!(scratch.values[1878] != 0.0))) && (!(scratch.values[1879] != 0.0))) {
            scratch.store_ad(1361, &AdValue::mul(AdValue::add(AdValue::div(AdValue::mul(AdValue::square(scratch.ad_value(1360)), AdValue::ln(scratch.ad_value(1360))), AdValue::sub_from_scalar(1.0, scratch.ad_value(1360))), scratch.ad_value(1360)), AdValue::sub_from_scalar(1.0, AdValue::scale(scratch.ad_value(549), 2.0))));
        }

        if ((((scratch.values[1320] != 0.0) && (scratch.values[1337] != 0.0)) && (!(scratch.values[1876] != 0.0))) && (!(scratch.values[1878] != 0.0))) {
            scratch.store_ad(1362, &AdValue::add(scratch.ad_value(1360), scratch.ad_value(1361)));
        }

        scratch.values[1880] = if (scratch.values[549] == 0.5) { 1.0 } else { 0.0 };

        if (((((scratch.values[1320] != 0.0) && (scratch.values[1337] != 0.0)) && (!(scratch.values[1876] != 0.0))) && (!(scratch.values[1878] != 0.0))) && (scratch.values[1880] != 0.0)) {
            scratch.store_ad(1356, &AdValue::sqrt(AdValue::mul(scratch.ad_value(1359), scratch.ad_value(628))));
        }

        if (((((scratch.values[1320] != 0.0) && (scratch.values[1337] != 0.0)) && (!(scratch.values[1876] != 0.0))) && (!(scratch.values[1878] != 0.0))) && (!(scratch.values[1880] != 0.0))) {
            scratch.store_ad(1356, &AdValue::pow(AdValue::mul(scratch.ad_value(1359), scratch.ad_value(628)), scratch.ad_value(549)));
        }

        if ((((scratch.values[1320] != 0.0) && (scratch.values[1337] != 0.0)) && (!(scratch.values[1876] != 0.0))) && (!(scratch.values[1878] != 0.0))) {
            scratch.store_ad(1363, &AdValue::mul(scratch.ad_value(622), scratch.ad_value(1356)));
        }

        if ((((scratch.values[1320] != 0.0) && (scratch.values[1337] != 0.0)) && (!(scratch.values[1876] != 0.0))) && (!(scratch.values[1878] != 0.0))) {
            scratch.store_ad(1364, &AdValue::mul(scratch.ad_value(592), AdValue::mul(AdValue::offset(scratch.ad_value(1350), (-1.0)), scratch.ad_value(1363))));
        }

        if ((((scratch.values[1320] != 0.0) && (scratch.values[1337] != 0.0)) && (!(scratch.values[1876] != 0.0))) && (!(scratch.values[1878] != 0.0))) {
            scratch.store_ad(1358, &AdValue::mul(scratch.ad_value(560), AdValue::mul(scratch.ad_value(1364), scratch.ad_value(1362))));
        }

        scratch.values[1881] = if (scratch.values[563] == 0.0) { 1.0 } else { 0.0 };

    }

    pub(super) fn stamp_transient_block_26(
        &mut self,
        ctx: &GeneratedEvalContext<'_>,
        stamper: &mut GeneratedStamper<'_>,
        scratch: &mut Scratch,
    ) {
        let _ = stamper;
        if ((((scratch.values[1320] != 0.0) && (scratch.values[1337] != 0.0)) && (!(scratch.values[1876] != 0.0))) && (scratch.values[1881] != 0.0)) {
            scratch.values[1365] = 0.0;
            scratch.node_derivatives[1365] = [0.0; Instance::NODE_COUNT];
            scratch.branch_derivatives[1365] = [0.0; Instance::BRANCH_COUNT];
        }

        if ((((scratch.values[1320] != 0.0) && (scratch.values[1337] != 0.0)) && (!(scratch.values[1876] != 0.0))) && (!(scratch.values[1881] != 0.0))) {
            scratch.store_ad(1366, &AdValue::mul(scratch.ad_value(637), AdValue::div(AdValue::mul(scratch.ad_value(1363), scratch.ad_value(607)), scratch.ad_value(1359))));
        }

        if ((((scratch.values[1320] != 0.0) && (scratch.values[1337] != 0.0)) && (!(scratch.values[1876] != 0.0))) && (!(scratch.values[1881] != 0.0))) {
            scratch.store_ad(1367, &AdValue::div(AdValue::scale(scratch.ad_value(634), 0.666666666666667), scratch.ad_value(1366)));
        }

        if ((((scratch.values[1320] != 0.0) && (scratch.values[1337] != 0.0)) && (!(scratch.values[1876] != 0.0))) && (!(scratch.values[1881] != 0.0))) {
            scratch.store_ad(1368, &AdValue::square(scratch.ad_value(1367)));
        }

        if ((((scratch.values[1320] != 0.0) && (scratch.values[1337] != 0.0)) && (!(scratch.values[1876] != 0.0))) && (!(scratch.values[1881] != 0.0))) {
            scratch.store_ad(1369, &AdValue::sqrt(AdValue::div(AdValue::square(scratch.ad_value(1368)), AdValue::offset(AdValue::square(scratch.ad_value(1368)), 1.0))));
        }

        if ((((scratch.values[1320] != 0.0) && (scratch.values[1337] != 0.0)) && (!(scratch.values[1876] != 0.0))) && (!(scratch.values[1881] != 0.0))) {
            scratch.store_ad(1370, &AdValue::sqrt(AdValue::abs(scratch.ad_value(1369))));
        }

        if ((((scratch.values[1320] != 0.0) && (scratch.values[1337] != 0.0)) && (!(scratch.values[1876] != 0.0))) && (!(scratch.values[1881] != 0.0))) {
            scratch.store_ad(1371, &AdValue::mul(scratch.ad_value(1369), scratch.ad_value(1370)));
        }

        scratch.values[1882] = if (((-scratch.values[549]) * scratch.values[610]) == (-1.0)) { 1.0 } else { 0.0 };

        if (((((scratch.values[1320] != 0.0) && (scratch.values[1337] != 0.0)) && (!(scratch.values[1876] != 0.0))) && (!(scratch.values[1881] != 0.0))) && (scratch.values[1882] != 0.0)) {
            scratch.store_ad(1372, &AdValue::div_from_scalar(1.0, AdValue::offset(AdValue::mul(scratch.ad_value(1366), scratch.ad_value(1371)), 1.0)));
        }

        if (((((scratch.values[1320] != 0.0) && (scratch.values[1337] != 0.0)) && (!(scratch.values[1876] != 0.0))) && (!(scratch.values[1881] != 0.0))) && (!(scratch.values[1882] != 0.0))) {
            scratch.store_ad(1372, &AdValue::pow(AdValue::offset(AdValue::mul(scratch.ad_value(1366), scratch.ad_value(1371)), 1.0), AdValue::mul(AdValue::neg(scratch.ad_value(549)), scratch.ad_value(610))));
        }

        if ((((scratch.values[1320] != 0.0) && (scratch.values[1337] != 0.0)) && (!(scratch.values[1876] != 0.0))) && (!(scratch.values[1881] != 0.0))) {
            scratch.store_ad(1373, &AdValue::div(AdValue::mul(scratch.ad_value(1362), scratch.ad_value(1372)), AdValue::add(scratch.ad_value(1362), scratch.ad_value(1372))));
        }

        if ((((scratch.values[1320] != 0.0) && (scratch.values[1337] != 0.0)) && (!(scratch.values[1876] != 0.0))) && (!(scratch.values[1881] != 0.0))) {
            scratch.store_ad(1374, &AdValue::sqrt(AdValue::scale(AdValue::div(scratch.ad_value(1366), scratch.ad_value(1370)), 0.375)));
        }

        if ((((scratch.values[1320] != 0.0) && (scratch.values[1337] != 0.0)) && (!(scratch.values[1876] != 0.0))) && (!(scratch.values[1881] != 0.0))) {
            scratch.store_ad(1375, &AdValue::sub(AdValue::scale(AdValue::mul(scratch.ad_value(1367), scratch.ad_value(1370)), 2.0), scratch.ad_value(1369)));
        }

        if ((((scratch.values[1320] != 0.0) && (scratch.values[1337] != 0.0)) && (!(scratch.values[1876] != 0.0))) && (!(scratch.values[1881] != 0.0))) {
            scratch.store_ad(1376, &AdValue::add(AdValue::sub(AdValue::mul(AdValue::mul(scratch.ad_value(634), scratch.ad_value(1367)), scratch.ad_value(1370)), AdValue::mul(scratch.ad_value(634), scratch.ad_value(1369))), AdValue::scale(AdValue::mul(scratch.ad_value(1366), scratch.ad_value(1371)), 0.5)));
        }

        if ((((scratch.values[1320] != 0.0) && (scratch.values[1337] != 0.0)) && (!(scratch.values[1876] != 0.0))) && (!(scratch.values[1881] != 0.0))) {
            scratch.store_ad(1377, &AdValue::mul(AdValue::offset(scratch.ad_value(1375), (-1.0)), scratch.ad_value(1374)));
        }

        if ((((scratch.values[1320] != 0.0) && (scratch.values[1337] != 0.0)) && (!(scratch.values[1876] != 0.0))) && (!(scratch.values[1881] != 0.0))) {
            scratch.store_ad(1338, &AdValue::square(scratch.ad_value(1377)));
        }

        scratch.values[1883] = if (scratch.values[1377] > 0.0) { 1.0 } else { 0.0 };

        if (((((scratch.values[1320] != 0.0) && (scratch.values[1337] != 0.0)) && (!(scratch.values[1876] != 0.0))) && (!(scratch.values[1881] != 0.0))) && (scratch.values[1883] != 0.0)) {
            scratch.store_ad(1339, &AdValue::div_from_scalar(1.0, AdValue::offset(AdValue::scale(scratch.ad_value(1377), scratch.values[428]), 1.0)));
        }

        if (((((scratch.values[1320] != 0.0) && (scratch.values[1337] != 0.0)) && (!(scratch.values[1876] != 0.0))) && (!(scratch.values[1881] != 0.0))) && (!(scratch.values[1883] != 0.0))) {
            scratch.store_ad(1339, &AdValue::div_from_scalar(1.0, AdValue::sub_from_scalar(1.0, AdValue::scale(scratch.ad_value(1377), scratch.values[428]))));
        }

        scratch.values[1884] = if (((-scratch.values[1338]) + scratch.values[1376]) > (-230.25850929940458)) { 1.0 } else { 0.0 };

        if (((((scratch.values[1320] != 0.0) && (scratch.values[1337] != 0.0)) && (!(scratch.values[1876] != 0.0))) && (!(scratch.values[1881] != 0.0))) && (scratch.values[1884] != 0.0)) {
            scratch.store_ad(1356, &AdValue::exp(AdValue::sub(scratch.ad_value(1376), scratch.ad_value(1338))));
        }

        if (((((scratch.values[1320] != 0.0) && (scratch.values[1337] != 0.0)) && (!(scratch.values[1876] != 0.0))) && (!(scratch.values[1881] != 0.0))) && (!(scratch.values[1884] != 0.0))) {
            let assign34430_ad_e44366: AdValue = AdValue::div_from_scalar(1e-100, AdValue::offset(AdValue::mul(AdValue::sub_from_scalar((-230.25850929940458), AdValue::sub(scratch.ad_value(1376), scratch.ad_value(1338))), AdValue::offset(AdValue::scale(AdValue::mul(AdValue::sub_from_scalar((-230.25850929940458), AdValue::sub(scratch.ad_value(1376), scratch.ad_value(1338))), AdValue::offset(AdValue::scale(AdValue::sub_from_scalar((-230.25850929940458), AdValue::sub(scratch.ad_value(1376), scratch.ad_value(1338))), 0.3333333333333333), 1.0)), 0.5), 1.0)), 1.0));
            scratch.store_ad(1356, &assign34430_ad_e44366);
        }

        if ((((scratch.values[1320] != 0.0) && (scratch.values[1337] != 0.0)) && (!(scratch.values[1876] != 0.0))) && (!(scratch.values[1881] != 0.0))) {
            scratch.store_ad(1340, &AdValue::mul(AdValue::add(AdValue::add(AdValue::scale(scratch.ad_value(1339), 0.29214664), AdValue::scale(AdValue::square(scratch.ad_value(1339)), scratch.values[429])), AdValue::scale(AdValue::mul(AdValue::square(scratch.ad_value(1339)), scratch.ad_value(1339)), scratch.values[430])), scratch.ad_value(1356)));
        }

        scratch.values[1885] = if (scratch.values[1377] > 0.0) { 1.0 } else { 0.0 };

        if (((((scratch.values[1320] != 0.0) && (scratch.values[1337] != 0.0)) && (!(scratch.values[1876] != 0.0))) && (!(scratch.values[1881] != 0.0))) && (scratch.values[1885] != 0.0)) {
            scratch.values[1378] = scratch.values[1340];
            scratch.node_derivatives[1378] = scratch.node_derivatives[1340];
            scratch.branch_derivatives[1378] = scratch.branch_derivatives[1340];
        }

        scratch.values[1886] = if (scratch.values[1376] > (-230.25850929940458)) { 1.0 } else { 0.0 };

        if ((((((scratch.values[1320] != 0.0) && (scratch.values[1337] != 0.0)) && (!(scratch.values[1876] != 0.0))) && (!(scratch.values[1881] != 0.0))) && (!(scratch.values[1885] != 0.0))) && (scratch.values[1886] != 0.0)) {
            scratch.store_ad(1356, &AdValue::exp(scratch.ad_value(1376)));
        }

        if ((((((scratch.values[1320] != 0.0) && (scratch.values[1337] != 0.0)) && (!(scratch.values[1876] != 0.0))) && (!(scratch.values[1881] != 0.0))) && (!(scratch.values[1885] != 0.0))) && (!(scratch.values[1886] != 0.0))) {
            scratch.store_ad(1356, &AdValue::div_from_scalar(1e-100, AdValue::offset(AdValue::mul(AdValue::sub_from_scalar((-230.25850929940458), scratch.ad_value(1376)), AdValue::offset(AdValue::scale(AdValue::mul(AdValue::sub_from_scalar((-230.25850929940458), scratch.ad_value(1376)), AdValue::offset(AdValue::scale(AdValue::sub_from_scalar((-230.25850929940458), scratch.ad_value(1376)), 0.3333333333333333), 1.0)), 0.5), 1.0)), 1.0)));
        }

        if (((((scratch.values[1320] != 0.0) && (scratch.values[1337] != 0.0)) && (!(scratch.values[1876] != 0.0))) && (!(scratch.values[1881] != 0.0))) && (!(scratch.values[1885] != 0.0))) {
            scratch.store_ad(1378, &AdValue::sub(AdValue::scale(scratch.ad_value(1356), 2.0), scratch.ad_value(1340)));
        }

        if ((((scratch.values[1320] != 0.0) && (scratch.values[1337] != 0.0)) && (!(scratch.values[1876] != 0.0))) && (!(scratch.values[1881] != 0.0))) {
            scratch.store_ad(1379, &AdValue::scale(AdValue::div(AdValue::mul(scratch.ad_value(634), scratch.ad_value(1378)), scratch.ad_value(1374)), (1.772453850905516 * 0.5)));
        }

        if ((((scratch.values[1320] != 0.0) && (scratch.values[1337] != 0.0)) && (!(scratch.values[1876] != 0.0))) && (!(scratch.values[1881] != 0.0))) {
            scratch.store_ad(1365, &AdValue::mul(scratch.ad_value(563), AdValue::mul(AdValue::mul(scratch.ad_value(1364), scratch.ad_value(1379)), scratch.ad_value(1373))));
        }

        scratch.values[1887] = if (scratch.values[569] == 0.0) { 1.0 } else { 0.0 };

        if ((((scratch.values[1320] != 0.0) && (scratch.values[1337] != 0.0)) && (!(scratch.values[1876] != 0.0))) && (scratch.values[1887] != 0.0)) {
            scratch.values[1380] = 0.0;
            scratch.node_derivatives[1380] = [0.0; Instance::NODE_COUNT];
            scratch.branch_derivatives[1380] = [0.0; Instance::BRANCH_COUNT];
        }

        scratch.values[1888] = if (scratch.values[549] == 0.5) { 1.0 } else { 0.0 };

        if (((((scratch.values[1320] != 0.0) && (scratch.values[1337] != 0.0)) && (!(scratch.values[1876] != 0.0))) && (!(scratch.values[1887] != 0.0))) && (scratch.values[1888] != 0.0)) {
            scratch.store_ad(1356, &AdValue::sqrt(AdValue::mul(AdValue::sub(scratch.ad_value(546), scratch.ad_value(1354)), scratch.ad_value(628))));
        }

        if (((((scratch.values[1320] != 0.0) && (scratch.values[1337] != 0.0)) && (!(scratch.values[1876] != 0.0))) && (!(scratch.values[1887] != 0.0))) && (!(scratch.values[1888] != 0.0))) {
            scratch.store_ad(1356, &AdValue::pow(AdValue::mul(AdValue::sub(scratch.ad_value(546), scratch.ad_value(1354)), scratch.ad_value(628)), scratch.ad_value(549)));
        }

        if ((((scratch.values[1320] != 0.0) && (scratch.values[1337] != 0.0)) && (!(scratch.values[1876] != 0.0))) && (!(scratch.values[1887] != 0.0))) {
            scratch.store_ad(1381, &AdValue::mul(scratch.ad_value(610), AdValue::div(AdValue::mul(AdValue::sub(scratch.ad_value(546), scratch.ad_value(1354)), scratch.ad_value(625)), scratch.ad_value(1356))));
        }

        scratch.values[1889] = if (((((-scratch.values[640]) / scratch.values[1381])) as f64).abs() < 230.25850929940458) { 1.0 } else { 0.0 };

        if (((((scratch.values[1320] != 0.0) && (scratch.values[1337] != 0.0)) && (!(scratch.values[1876] != 0.0))) && (!(scratch.values[1887] != 0.0))) && (scratch.values[1889] != 0.0)) {
            scratch.store_ad(1356, &AdValue::exp(AdValue::div(AdValue::neg(scratch.ad_value(640)), scratch.ad_value(1381))));
        }

        scratch.values[1890] = if (((-scratch.values[640]) / scratch.values[1381]) < 0.0) { 1.0 } else { 0.0 };

        if ((((((scratch.values[1320] != 0.0) && (scratch.values[1337] != 0.0)) && (!(scratch.values[1876] != 0.0))) && (!(scratch.values[1887] != 0.0))) && (!(scratch.values[1889] != 0.0))) && (scratch.values[1890] != 0.0)) {
            let assign34620_ad_e44693: AdValue = AdValue::offset(AdValue::mul(AdValue::sub_from_scalar((-230.25850929940458), AdValue::div(AdValue::neg(scratch.ad_value(640)), scratch.ad_value(1381))), AdValue::offset(AdValue::scale(AdValue::mul(AdValue::sub_from_scalar((-230.25850929940458), AdValue::div(AdValue::neg(scratch.ad_value(640)), scratch.ad_value(1381))), AdValue::offset(AdValue::scale(AdValue::sub_from_scalar((-230.25850929940458), AdValue::div(AdValue::neg(scratch.ad_value(640)), scratch.ad_value(1381))), 0.3333333333333333), 1.0)), 0.5), 1.0)), 1.0);
            scratch.store_ad(1356, &AdValue::div_from_scalar(1e-100, assign34620_ad_e44693));
        }

        if ((((((scratch.values[1320] != 0.0) && (scratch.values[1337] != 0.0)) && (!(scratch.values[1876] != 0.0))) && (!(scratch.values[1887] != 0.0))) && (!(scratch.values[1889] != 0.0))) && (!(scratch.values[1890] != 0.0))) {
            let assign34630_ad_e44743: AdValue = AdValue::scale(AdValue::offset(AdValue::mul(AdValue::offset(AdValue::div(AdValue::neg(scratch.ad_value(640)), scratch.ad_value(1381)), (-230.25850929940458)), AdValue::offset(AdValue::scale(AdValue::mul(AdValue::offset(AdValue::div(AdValue::neg(scratch.ad_value(640)), scratch.ad_value(1381)), (-230.25850929940458)), AdValue::offset(AdValue::scale(AdValue::offset(AdValue::div(AdValue::neg(scratch.ad_value(640)), scratch.ad_value(1381)), (-230.25850929940458)), 0.3333333333333333), 1.0)), 0.5), 1.0)), 1.0), 1e100);
            scratch.store_ad(1356, &assign34630_ad_e44743);
        }

        if ((((scratch.values[1320] != 0.0) && (scratch.values[1337] != 0.0)) && (!(scratch.values[1876] != 0.0))) && (!(scratch.values[1887] != 0.0))) {
            scratch.store_ad(1380, &AdValue::mul(scratch.ad_value(569), AdValue::mul(AdValue::mul(AdValue::mul(scratch.ad_value(526), scratch.ad_value(1381)), scratch.ad_value(1381)), scratch.ad_value(1356))));
        }

        scratch.values[1891] = if (scratch.values[578] > 1000.0) { 1.0 } else { 0.0 };

        if ((((scratch.values[1320] != 0.0) && (scratch.values[1337] != 0.0)) && (!(scratch.values[1876] != 0.0))) && (scratch.values[1891] != 0.0)) {
            scratch.values[1382] = 1.0;
            scratch.node_derivatives[1382] = [0.0; Instance::NODE_COUNT];
            scratch.branch_derivatives[1382] = [0.0; Instance::BRANCH_COUNT];
        }

        scratch.values[1892] = if (scratch.values[1355] > ((-scratch.values[500]) * scratch.values[578])) { 1.0 } else { 0.0 };

        scratch.values[1893] = if (scratch.values[581] == 4.0) { 1.0 } else { 0.0 };

        if ((((((scratch.values[1320] != 0.0) && (scratch.values[1337] != 0.0)) && (!(scratch.values[1876] != 0.0))) && (!(scratch.values[1891] != 0.0))) && (scratch.values[1892] != 0.0)) && (scratch.values[1893] != 0.0)) {
            scratch.store_ad(1356, &AdValue::mul(AdValue::mul(AdValue::mul(AdValue::mul(scratch.ad_value(1355), scratch.ad_value(646)), AdValue::mul(scratch.ad_value(1355), scratch.ad_value(646))), AdValue::mul(scratch.ad_value(1355), scratch.ad_value(646))), AdValue::mul(scratch.ad_value(1355), scratch.ad_value(646))));
        }

        if ((((((scratch.values[1320] != 0.0) && (scratch.values[1337] != 0.0)) && (!(scratch.values[1876] != 0.0))) && (!(scratch.values[1891] != 0.0))) && (scratch.values[1892] != 0.0)) && (!(scratch.values[1893] != 0.0))) {
            scratch.store_ad(1356, &AdValue::pow(AdValue::abs(AdValue::mul(scratch.ad_value(1355), scratch.ad_value(646))), scratch.ad_value(581)));
        }

        if (((((scratch.values[1320] != 0.0) && (scratch.values[1337] != 0.0)) && (!(scratch.values[1876] != 0.0))) && (!(scratch.values[1891] != 0.0))) && (scratch.values[1892] != 0.0)) {
            scratch.store_ad(1382, &AdValue::div_from_scalar(1.0, AdValue::sub_from_scalar(1.0, scratch.ad_value(1356))));
        }

        if (((((scratch.values[1320] != 0.0) && (scratch.values[1337] != 0.0)) && (!(scratch.values[1876] != 0.0))) && (!(scratch.values[1891] != 0.0))) && (!(scratch.values[1892] != 0.0))) {
            scratch.store_ad(1382, &AdValue::add(scratch.ad_value(643), AdValue::mul(AdValue::add(scratch.ad_value(1355), AdValue::scale(scratch.ad_value(578), scratch.values[500])), scratch.ad_value(649))));
        }

        if (((scratch.values[1320] != 0.0) && (scratch.values[1337] != 0.0)) && (!(scratch.values[1876] != 0.0))) {
            scratch.store_ad(1383, &AdValue::mul(AdValue::add(AdValue::add(AdValue::add(scratch.ad_value(1357), scratch.ad_value(1358)), scratch.ad_value(1365)), scratch.ad_value(1380)), scratch.ad_value(1382)));
        }

        scratch.values[1894] = if (scratch.values[725] == 0.0) { 1.0 } else { 0.0 };

        if (((scratch.values[1320] != 0.0) && (scratch.values[1337] != 0.0)) && (scratch.values[1894] != 0.0)) {
            scratch.values[1384] = 0.0;
            scratch.node_derivatives[1384] = [0.0; Instance::NODE_COUNT];
            scratch.branch_derivatives[1384] = [0.0; Instance::BRANCH_COUNT];
        }

        scratch.values[1895] = if (scratch.values[608] == 0.5) { 1.0 } else { 0.0 };

        if ((((scratch.values[1320] != 0.0) && (scratch.values[1337] != 0.0)) && (!(scratch.values[1894] != 0.0))) && (scratch.values[1895] != 0.0)) {
            scratch.store_ad(1356, &AdValue::sqrt(AdValue::sub_from_scalar(1.0, AdValue::mul(scratch.ad_value(1348), scratch.ad_value(605)))));
        }

        if ((((scratch.values[1320] != 0.0) && (scratch.values[1337] != 0.0)) && (!(scratch.values[1894] != 0.0))) && (!(scratch.values[1895] != 0.0))) {
            scratch.store_ad(1356, &AdValue::pow(AdValue::sub_from_scalar(1.0, AdValue::mul(scratch.ad_value(1348), scratch.ad_value(605))), scratch.ad_value(608)));
        }

        if (((scratch.values[1320] != 0.0) && (scratch.values[1337] != 0.0)) && (!(scratch.values[1894] != 0.0))) {
            scratch.store_ad(1357, &AdValue::mul(scratch.ad_value(596), scratch.ad_value(1347)));
        }

        scratch.values[1896] = if ((scratch.values[561] == 0.0) && (scratch.values[564] == 0.0)) { 1.0 } else { 0.0 };

        if ((((scratch.values[1320] != 0.0) && (scratch.values[1337] != 0.0)) && (!(scratch.values[1894] != 0.0))) && (scratch.values[1896] != 0.0)) {
            scratch.values[1358] = 0.0;
            scratch.node_derivatives[1358] = [0.0; Instance::NODE_COUNT];
            scratch.branch_derivatives[1358] = [0.0; Instance::BRANCH_COUNT];
        }

        if ((((scratch.values[1320] != 0.0) && (scratch.values[1337] != 0.0)) && (!(scratch.values[1894] != 0.0))) && (!(scratch.values[1896] != 0.0))) {
            scratch.store_ad(1359, &AdValue::sub(scratch.ad_value(602), scratch.ad_value(1353)));
        }

        if ((((scratch.values[1320] != 0.0) && (scratch.values[1337] != 0.0)) && (!(scratch.values[1894] != 0.0))) && (!(scratch.values[1896] != 0.0))) {
            scratch.store_ad(1360, &AdValue::sub_from_scalar(1.0, AdValue::sqrt(AdValue::sub_from_scalar(1.0, AdValue::div(scratch.ad_value(1351), scratch.ad_value(1359))))));
        }

        scratch.values[1897] = if (scratch.values[550] == 0.5) { 1.0 } else { 0.0 };

        if (((((scratch.values[1320] != 0.0) && (scratch.values[1337] != 0.0)) && (!(scratch.values[1894] != 0.0))) && (!(scratch.values[1896] != 0.0))) && (scratch.values[1897] != 0.0)) {
            scratch.values[1361] = 0.0;
            scratch.node_derivatives[1361] = [0.0; Instance::NODE_COUNT];
            scratch.branch_derivatives[1361] = [0.0; Instance::BRANCH_COUNT];
        }

        if (((((scratch.values[1320] != 0.0) && (scratch.values[1337] != 0.0)) && (!(scratch.values[1894] != 0.0))) && (!(scratch.values[1896] != 0.0))) && (!(scratch.values[1897] != 0.0))) {
            scratch.store_ad(1361, &AdValue::mul(AdValue::add(AdValue::div(AdValue::mul(AdValue::square(scratch.ad_value(1360)), AdValue::ln(scratch.ad_value(1360))), AdValue::sub_from_scalar(1.0, scratch.ad_value(1360))), scratch.ad_value(1360)), AdValue::sub_from_scalar(1.0, AdValue::scale(scratch.ad_value(550), 2.0))));
        }

        if ((((scratch.values[1320] != 0.0) && (scratch.values[1337] != 0.0)) && (!(scratch.values[1894] != 0.0))) && (!(scratch.values[1896] != 0.0))) {
            scratch.store_ad(1362, &AdValue::add(scratch.ad_value(1360), scratch.ad_value(1361)));
        }

        scratch.values[1898] = if (scratch.values[550] == 0.5) { 1.0 } else { 0.0 };

        if (((((scratch.values[1320] != 0.0) && (scratch.values[1337] != 0.0)) && (!(scratch.values[1894] != 0.0))) && (!(scratch.values[1896] != 0.0))) && (scratch.values[1898] != 0.0)) {
            scratch.store_ad(1356, &AdValue::sqrt(AdValue::mul(scratch.ad_value(1359), scratch.ad_value(629))));
        }

        if (((((scratch.values[1320] != 0.0) && (scratch.values[1337] != 0.0)) && (!(scratch.values[1894] != 0.0))) && (!(scratch.values[1896] != 0.0))) && (!(scratch.values[1898] != 0.0))) {
            scratch.store_ad(1356, &AdValue::pow(AdValue::mul(scratch.ad_value(1359), scratch.ad_value(629)), scratch.ad_value(550)));
        }

        if ((((scratch.values[1320] != 0.0) && (scratch.values[1337] != 0.0)) && (!(scratch.values[1894] != 0.0))) && (!(scratch.values[1896] != 0.0))) {
            scratch.store_ad(1363, &AdValue::mul(scratch.ad_value(623), scratch.ad_value(1356)));
        }

        if ((((scratch.values[1320] != 0.0) && (scratch.values[1337] != 0.0)) && (!(scratch.values[1894] != 0.0))) && (!(scratch.values[1896] != 0.0))) {
            scratch.store_ad(1364, &AdValue::mul(scratch.ad_value(593), AdValue::mul(AdValue::offset(scratch.ad_value(1350), (-1.0)), scratch.ad_value(1363))));
        }

        if ((((scratch.values[1320] != 0.0) && (scratch.values[1337] != 0.0)) && (!(scratch.values[1894] != 0.0))) && (!(scratch.values[1896] != 0.0))) {
            scratch.store_ad(1358, &AdValue::mul(scratch.ad_value(561), AdValue::mul(scratch.ad_value(1364), scratch.ad_value(1362))));
        }

        scratch.values[1899] = if (scratch.values[564] == 0.0) { 1.0 } else { 0.0 };

        if ((((scratch.values[1320] != 0.0) && (scratch.values[1337] != 0.0)) && (!(scratch.values[1894] != 0.0))) && (scratch.values[1899] != 0.0)) {
            scratch.values[1365] = 0.0;
            scratch.node_derivatives[1365] = [0.0; Instance::NODE_COUNT];
            scratch.branch_derivatives[1365] = [0.0; Instance::BRANCH_COUNT];
        }

        if ((((scratch.values[1320] != 0.0) && (scratch.values[1337] != 0.0)) && (!(scratch.values[1894] != 0.0))) && (!(scratch.values[1899] != 0.0))) {
            scratch.store_ad(1366, &AdValue::mul(scratch.ad_value(638), AdValue::div(AdValue::mul(scratch.ad_value(1363), scratch.ad_value(608)), scratch.ad_value(1359))));
        }

        if ((((scratch.values[1320] != 0.0) && (scratch.values[1337] != 0.0)) && (!(scratch.values[1894] != 0.0))) && (!(scratch.values[1899] != 0.0))) {
            scratch.store_ad(1367, &AdValue::div(AdValue::scale(scratch.ad_value(635), 0.666666666666667), scratch.ad_value(1366)));
        }

        if ((((scratch.values[1320] != 0.0) && (scratch.values[1337] != 0.0)) && (!(scratch.values[1894] != 0.0))) && (!(scratch.values[1899] != 0.0))) {
            scratch.store_ad(1368, &AdValue::square(scratch.ad_value(1367)));
        }

        if ((((scratch.values[1320] != 0.0) && (scratch.values[1337] != 0.0)) && (!(scratch.values[1894] != 0.0))) && (!(scratch.values[1899] != 0.0))) {
            scratch.store_ad(1369, &AdValue::sqrt(AdValue::div(AdValue::square(scratch.ad_value(1368)), AdValue::offset(AdValue::square(scratch.ad_value(1368)), 1.0))));
        }

        if ((((scratch.values[1320] != 0.0) && (scratch.values[1337] != 0.0)) && (!(scratch.values[1894] != 0.0))) && (!(scratch.values[1899] != 0.0))) {
            scratch.store_ad(1370, &AdValue::sqrt(AdValue::abs(scratch.ad_value(1369))));
        }

        if ((((scratch.values[1320] != 0.0) && (scratch.values[1337] != 0.0)) && (!(scratch.values[1894] != 0.0))) && (!(scratch.values[1899] != 0.0))) {
            scratch.store_ad(1371, &AdValue::mul(scratch.ad_value(1369), scratch.ad_value(1370)));
        }

        scratch.values[1900] = if (((-scratch.values[550]) * scratch.values[611]) == (-1.0)) { 1.0 } else { 0.0 };

        if (((((scratch.values[1320] != 0.0) && (scratch.values[1337] != 0.0)) && (!(scratch.values[1894] != 0.0))) && (!(scratch.values[1899] != 0.0))) && (scratch.values[1900] != 0.0)) {
            scratch.store_ad(1372, &AdValue::div_from_scalar(1.0, AdValue::offset(AdValue::mul(scratch.ad_value(1366), scratch.ad_value(1371)), 1.0)));
        }

        if (((((scratch.values[1320] != 0.0) && (scratch.values[1337] != 0.0)) && (!(scratch.values[1894] != 0.0))) && (!(scratch.values[1899] != 0.0))) && (!(scratch.values[1900] != 0.0))) {
            scratch.store_ad(1372, &AdValue::pow(AdValue::offset(AdValue::mul(scratch.ad_value(1366), scratch.ad_value(1371)), 1.0), AdValue::mul(AdValue::neg(scratch.ad_value(550)), scratch.ad_value(611))));
        }

        if ((((scratch.values[1320] != 0.0) && (scratch.values[1337] != 0.0)) && (!(scratch.values[1894] != 0.0))) && (!(scratch.values[1899] != 0.0))) {
            scratch.store_ad(1373, &AdValue::div(AdValue::mul(scratch.ad_value(1362), scratch.ad_value(1372)), AdValue::add(scratch.ad_value(1362), scratch.ad_value(1372))));
        }

        if ((((scratch.values[1320] != 0.0) && (scratch.values[1337] != 0.0)) && (!(scratch.values[1894] != 0.0))) && (!(scratch.values[1899] != 0.0))) {
            scratch.store_ad(1374, &AdValue::sqrt(AdValue::scale(AdValue::div(scratch.ad_value(1366), scratch.ad_value(1370)), 0.375)));
        }

        if ((((scratch.values[1320] != 0.0) && (scratch.values[1337] != 0.0)) && (!(scratch.values[1894] != 0.0))) && (!(scratch.values[1899] != 0.0))) {
            scratch.store_ad(1375, &AdValue::sub(AdValue::scale(AdValue::mul(scratch.ad_value(1367), scratch.ad_value(1370)), 2.0), scratch.ad_value(1369)));
        }

        if ((((scratch.values[1320] != 0.0) && (scratch.values[1337] != 0.0)) && (!(scratch.values[1894] != 0.0))) && (!(scratch.values[1899] != 0.0))) {
            scratch.store_ad(1376, &AdValue::add(AdValue::sub(AdValue::mul(AdValue::mul(scratch.ad_value(635), scratch.ad_value(1367)), scratch.ad_value(1370)), AdValue::mul(scratch.ad_value(635), scratch.ad_value(1369))), AdValue::scale(AdValue::mul(scratch.ad_value(1366), scratch.ad_value(1371)), 0.5)));
        }

        if ((((scratch.values[1320] != 0.0) && (scratch.values[1337] != 0.0)) && (!(scratch.values[1894] != 0.0))) && (!(scratch.values[1899] != 0.0))) {
            scratch.store_ad(1377, &AdValue::mul(AdValue::offset(scratch.ad_value(1375), (-1.0)), scratch.ad_value(1374)));
        }

        if ((((scratch.values[1320] != 0.0) && (scratch.values[1337] != 0.0)) && (!(scratch.values[1894] != 0.0))) && (!(scratch.values[1899] != 0.0))) {
            scratch.store_ad(1338, &AdValue::square(scratch.ad_value(1377)));
        }

        scratch.values[1901] = if (scratch.values[1377] > 0.0) { 1.0 } else { 0.0 };

        if (((((scratch.values[1320] != 0.0) && (scratch.values[1337] != 0.0)) && (!(scratch.values[1894] != 0.0))) && (!(scratch.values[1899] != 0.0))) && (scratch.values[1901] != 0.0)) {
            scratch.store_ad(1339, &AdValue::div_from_scalar(1.0, AdValue::offset(AdValue::scale(scratch.ad_value(1377), scratch.values[428]), 1.0)));
        }

        if (((((scratch.values[1320] != 0.0) && (scratch.values[1337] != 0.0)) && (!(scratch.values[1894] != 0.0))) && (!(scratch.values[1899] != 0.0))) && (!(scratch.values[1901] != 0.0))) {
            scratch.store_ad(1339, &AdValue::div_from_scalar(1.0, AdValue::sub_from_scalar(1.0, AdValue::scale(scratch.ad_value(1377), scratch.values[428]))));
        }

        scratch.values[1902] = if (((-scratch.values[1338]) + scratch.values[1376]) > (-230.25850929940458)) { 1.0 } else { 0.0 };

        if (((((scratch.values[1320] != 0.0) && (scratch.values[1337] != 0.0)) && (!(scratch.values[1894] != 0.0))) && (!(scratch.values[1899] != 0.0))) && (scratch.values[1902] != 0.0)) {
            scratch.store_ad(1356, &AdValue::exp(AdValue::sub(scratch.ad_value(1376), scratch.ad_value(1338))));
        }

        if (((((scratch.values[1320] != 0.0) && (scratch.values[1337] != 0.0)) && (!(scratch.values[1894] != 0.0))) && (!(scratch.values[1899] != 0.0))) && (!(scratch.values[1902] != 0.0))) {
            let assign35180_ad_e45572: AdValue = AdValue::div_from_scalar(1e-100, AdValue::offset(AdValue::mul(AdValue::sub_from_scalar((-230.25850929940458), AdValue::sub(scratch.ad_value(1376), scratch.ad_value(1338))), AdValue::offset(AdValue::scale(AdValue::mul(AdValue::sub_from_scalar((-230.25850929940458), AdValue::sub(scratch.ad_value(1376), scratch.ad_value(1338))), AdValue::offset(AdValue::scale(AdValue::sub_from_scalar((-230.25850929940458), AdValue::sub(scratch.ad_value(1376), scratch.ad_value(1338))), 0.3333333333333333), 1.0)), 0.5), 1.0)), 1.0));
            scratch.store_ad(1356, &assign35180_ad_e45572);
        }

        if ((((scratch.values[1320] != 0.0) && (scratch.values[1337] != 0.0)) && (!(scratch.values[1894] != 0.0))) && (!(scratch.values[1899] != 0.0))) {
            scratch.store_ad(1340, &AdValue::mul(AdValue::add(AdValue::add(AdValue::scale(scratch.ad_value(1339), 0.29214664), AdValue::scale(AdValue::square(scratch.ad_value(1339)), scratch.values[429])), AdValue::scale(AdValue::mul(AdValue::square(scratch.ad_value(1339)), scratch.ad_value(1339)), scratch.values[430])), scratch.ad_value(1356)));
        }

        scratch.values[1903] = if (scratch.values[1377] > 0.0) { 1.0 } else { 0.0 };

        if (((((scratch.values[1320] != 0.0) && (scratch.values[1337] != 0.0)) && (!(scratch.values[1894] != 0.0))) && (!(scratch.values[1899] != 0.0))) && (scratch.values[1903] != 0.0)) {
            scratch.values[1378] = scratch.values[1340];
            scratch.node_derivatives[1378] = scratch.node_derivatives[1340];
            scratch.branch_derivatives[1378] = scratch.branch_derivatives[1340];
        }

        scratch.values[1904] = if (scratch.values[1376] > (-230.25850929940458)) { 1.0 } else { 0.0 };

        if ((((((scratch.values[1320] != 0.0) && (scratch.values[1337] != 0.0)) && (!(scratch.values[1894] != 0.0))) && (!(scratch.values[1899] != 0.0))) && (!(scratch.values[1903] != 0.0))) && (scratch.values[1904] != 0.0)) {
            scratch.store_ad(1356, &AdValue::exp(scratch.ad_value(1376)));
        }

        if ((((((scratch.values[1320] != 0.0) && (scratch.values[1337] != 0.0)) && (!(scratch.values[1894] != 0.0))) && (!(scratch.values[1899] != 0.0))) && (!(scratch.values[1903] != 0.0))) && (!(scratch.values[1904] != 0.0))) {
            scratch.store_ad(1356, &AdValue::div_from_scalar(1e-100, AdValue::offset(AdValue::mul(AdValue::sub_from_scalar((-230.25850929940458), scratch.ad_value(1376)), AdValue::offset(AdValue::scale(AdValue::mul(AdValue::sub_from_scalar((-230.25850929940458), scratch.ad_value(1376)), AdValue::offset(AdValue::scale(AdValue::sub_from_scalar((-230.25850929940458), scratch.ad_value(1376)), 0.3333333333333333), 1.0)), 0.5), 1.0)), 1.0)));
        }

        if (((((scratch.values[1320] != 0.0) && (scratch.values[1337] != 0.0)) && (!(scratch.values[1894] != 0.0))) && (!(scratch.values[1899] != 0.0))) && (!(scratch.values[1903] != 0.0))) {
            scratch.store_ad(1378, &AdValue::sub(AdValue::scale(scratch.ad_value(1356), 2.0), scratch.ad_value(1340)));
        }

        if ((((scratch.values[1320] != 0.0) && (scratch.values[1337] != 0.0)) && (!(scratch.values[1894] != 0.0))) && (!(scratch.values[1899] != 0.0))) {
            scratch.store_ad(1379, &AdValue::scale(AdValue::div(AdValue::mul(scratch.ad_value(635), scratch.ad_value(1378)), scratch.ad_value(1374)), (1.772453850905516 * 0.5)));
        }

        if ((((scratch.values[1320] != 0.0) && (scratch.values[1337] != 0.0)) && (!(scratch.values[1894] != 0.0))) && (!(scratch.values[1899] != 0.0))) {
            scratch.store_ad(1365, &AdValue::mul(scratch.ad_value(564), AdValue::mul(AdValue::mul(scratch.ad_value(1364), scratch.ad_value(1379)), scratch.ad_value(1373))));
        }

        scratch.values[1905] = if (scratch.values[570] == 0.0) { 1.0 } else { 0.0 };

        if ((((scratch.values[1320] != 0.0) && (scratch.values[1337] != 0.0)) && (!(scratch.values[1894] != 0.0))) && (scratch.values[1905] != 0.0)) {
            scratch.values[1380] = 0.0;
            scratch.node_derivatives[1380] = [0.0; Instance::NODE_COUNT];
            scratch.branch_derivatives[1380] = [0.0; Instance::BRANCH_COUNT];
        }

        scratch.values[1906] = if (scratch.values[550] == 0.5) { 1.0 } else { 0.0 };

        if (((((scratch.values[1320] != 0.0) && (scratch.values[1337] != 0.0)) && (!(scratch.values[1894] != 0.0))) && (!(scratch.values[1905] != 0.0))) && (scratch.values[1906] != 0.0)) {
            scratch.store_ad(1356, &AdValue::sqrt(AdValue::mul(AdValue::sub(scratch.ad_value(547), scratch.ad_value(1354)), scratch.ad_value(629))));
        }

        if (((((scratch.values[1320] != 0.0) && (scratch.values[1337] != 0.0)) && (!(scratch.values[1894] != 0.0))) && (!(scratch.values[1905] != 0.0))) && (!(scratch.values[1906] != 0.0))) {
            scratch.store_ad(1356, &AdValue::pow(AdValue::mul(AdValue::sub(scratch.ad_value(547), scratch.ad_value(1354)), scratch.ad_value(629)), scratch.ad_value(550)));
        }

        if ((((scratch.values[1320] != 0.0) && (scratch.values[1337] != 0.0)) && (!(scratch.values[1894] != 0.0))) && (!(scratch.values[1905] != 0.0))) {
            scratch.store_ad(1381, &AdValue::mul(scratch.ad_value(611), AdValue::div(AdValue::mul(AdValue::sub(scratch.ad_value(547), scratch.ad_value(1354)), scratch.ad_value(626)), scratch.ad_value(1356))));
        }

        scratch.values[1907] = if (((((-scratch.values[641]) / scratch.values[1381])) as f64).abs() < 230.25850929940458) { 1.0 } else { 0.0 };

        if (((((scratch.values[1320] != 0.0) && (scratch.values[1337] != 0.0)) && (!(scratch.values[1894] != 0.0))) && (!(scratch.values[1905] != 0.0))) && (scratch.values[1907] != 0.0)) {
            scratch.store_ad(1356, &AdValue::exp(AdValue::div(AdValue::neg(scratch.ad_value(641)), scratch.ad_value(1381))));
        }

        scratch.values[1908] = if (((-scratch.values[641]) / scratch.values[1381]) < 0.0) { 1.0 } else { 0.0 };

        if ((((((scratch.values[1320] != 0.0) && (scratch.values[1337] != 0.0)) && (!(scratch.values[1894] != 0.0))) && (!(scratch.values[1905] != 0.0))) && (!(scratch.values[1907] != 0.0))) && (scratch.values[1908] != 0.0)) {
            let assign35370_ad_e45899: AdValue = AdValue::offset(AdValue::mul(AdValue::sub_from_scalar((-230.25850929940458), AdValue::div(AdValue::neg(scratch.ad_value(641)), scratch.ad_value(1381))), AdValue::offset(AdValue::scale(AdValue::mul(AdValue::sub_from_scalar((-230.25850929940458), AdValue::div(AdValue::neg(scratch.ad_value(641)), scratch.ad_value(1381))), AdValue::offset(AdValue::scale(AdValue::sub_from_scalar((-230.25850929940458), AdValue::div(AdValue::neg(scratch.ad_value(641)), scratch.ad_value(1381))), 0.3333333333333333), 1.0)), 0.5), 1.0)), 1.0);
            scratch.store_ad(1356, &AdValue::div_from_scalar(1e-100, assign35370_ad_e45899));
        }

        if ((((((scratch.values[1320] != 0.0) && (scratch.values[1337] != 0.0)) && (!(scratch.values[1894] != 0.0))) && (!(scratch.values[1905] != 0.0))) && (!(scratch.values[1907] != 0.0))) && (!(scratch.values[1908] != 0.0))) {
            let assign35380_ad_e45949: AdValue = AdValue::scale(AdValue::offset(AdValue::mul(AdValue::offset(AdValue::div(AdValue::neg(scratch.ad_value(641)), scratch.ad_value(1381)), (-230.25850929940458)), AdValue::offset(AdValue::scale(AdValue::mul(AdValue::offset(AdValue::div(AdValue::neg(scratch.ad_value(641)), scratch.ad_value(1381)), (-230.25850929940458)), AdValue::offset(AdValue::scale(AdValue::offset(AdValue::div(AdValue::neg(scratch.ad_value(641)), scratch.ad_value(1381)), (-230.25850929940458)), 0.3333333333333333), 1.0)), 0.5), 1.0)), 1.0), 1e100);
            scratch.store_ad(1356, &assign35380_ad_e45949);
        }

        if ((((scratch.values[1320] != 0.0) && (scratch.values[1337] != 0.0)) && (!(scratch.values[1894] != 0.0))) && (!(scratch.values[1905] != 0.0))) {
            scratch.store_ad(1380, &AdValue::mul(scratch.ad_value(570), AdValue::mul(AdValue::mul(AdValue::mul(scratch.ad_value(526), scratch.ad_value(1381)), scratch.ad_value(1381)), scratch.ad_value(1356))));
        }

        scratch.values[1909] = if (scratch.values[579] > 1000.0) { 1.0 } else { 0.0 };

        if ((((scratch.values[1320] != 0.0) && (scratch.values[1337] != 0.0)) && (!(scratch.values[1894] != 0.0))) && (scratch.values[1909] != 0.0)) {
            scratch.values[1382] = 1.0;
            scratch.node_derivatives[1382] = [0.0; Instance::NODE_COUNT];
            scratch.branch_derivatives[1382] = [0.0; Instance::BRANCH_COUNT];
        }

        scratch.values[1910] = if (scratch.values[1355] > ((-scratch.values[500]) * scratch.values[579])) { 1.0 } else { 0.0 };

        scratch.values[1911] = if (scratch.values[582] == 4.0) { 1.0 } else { 0.0 };

        if ((((((scratch.values[1320] != 0.0) && (scratch.values[1337] != 0.0)) && (!(scratch.values[1894] != 0.0))) && (!(scratch.values[1909] != 0.0))) && (scratch.values[1910] != 0.0)) && (scratch.values[1911] != 0.0)) {
            scratch.store_ad(1356, &AdValue::mul(AdValue::mul(AdValue::mul(AdValue::mul(scratch.ad_value(1355), scratch.ad_value(647)), AdValue::mul(scratch.ad_value(1355), scratch.ad_value(647))), AdValue::mul(scratch.ad_value(1355), scratch.ad_value(647))), AdValue::mul(scratch.ad_value(1355), scratch.ad_value(647))));
        }

        if ((((((scratch.values[1320] != 0.0) && (scratch.values[1337] != 0.0)) && (!(scratch.values[1894] != 0.0))) && (!(scratch.values[1909] != 0.0))) && (scratch.values[1910] != 0.0)) && (!(scratch.values[1911] != 0.0))) {
            scratch.store_ad(1356, &AdValue::pow(AdValue::abs(AdValue::mul(scratch.ad_value(1355), scratch.ad_value(647))), scratch.ad_value(582)));
        }

        if (((((scratch.values[1320] != 0.0) && (scratch.values[1337] != 0.0)) && (!(scratch.values[1894] != 0.0))) && (!(scratch.values[1909] != 0.0))) && (scratch.values[1910] != 0.0)) {
            scratch.store_ad(1382, &AdValue::div_from_scalar(1.0, AdValue::sub_from_scalar(1.0, scratch.ad_value(1356))));
        }

        if (((((scratch.values[1320] != 0.0) && (scratch.values[1337] != 0.0)) && (!(scratch.values[1894] != 0.0))) && (!(scratch.values[1909] != 0.0))) && (!(scratch.values[1910] != 0.0))) {
            scratch.store_ad(1382, &AdValue::add(scratch.ad_value(644), AdValue::mul(AdValue::add(scratch.ad_value(1355), AdValue::scale(scratch.ad_value(579), scratch.values[500])), scratch.ad_value(650))));
        }

        if (((scratch.values[1320] != 0.0) && (scratch.values[1337] != 0.0)) && (!(scratch.values[1894] != 0.0))) {
            scratch.store_ad(1384, &AdValue::mul(AdValue::add(AdValue::add(AdValue::add(scratch.ad_value(1357), scratch.ad_value(1358)), scratch.ad_value(1365)), scratch.ad_value(1380)), scratch.ad_value(1382)));
        }

        scratch.values[1912] = if (scratch.values[726] == 0.0) { 1.0 } else { 0.0 };

        if (((scratch.values[1320] != 0.0) && (scratch.values[1337] != 0.0)) && (scratch.values[1912] != 0.0)) {
            scratch.values[1385] = 0.0;
            scratch.node_derivatives[1385] = [0.0; Instance::NODE_COUNT];
            scratch.branch_derivatives[1385] = [0.0; Instance::BRANCH_COUNT];
        }

        scratch.values[1913] = if (scratch.values[609] == 0.5) { 1.0 } else { 0.0 };

        if ((((scratch.values[1320] != 0.0) && (scratch.values[1337] != 0.0)) && (!(scratch.values[1912] != 0.0))) && (scratch.values[1913] != 0.0)) {
            scratch.store_ad(1356, &AdValue::sqrt(AdValue::sub_from_scalar(1.0, AdValue::mul(scratch.ad_value(1348), scratch.ad_value(606)))));
        }

        if ((((scratch.values[1320] != 0.0) && (scratch.values[1337] != 0.0)) && (!(scratch.values[1912] != 0.0))) && (!(scratch.values[1913] != 0.0))) {
            scratch.store_ad(1356, &AdValue::pow(AdValue::sub_from_scalar(1.0, AdValue::mul(scratch.ad_value(1348), scratch.ad_value(606))), scratch.ad_value(609)));
        }

        if (((scratch.values[1320] != 0.0) && (scratch.values[1337] != 0.0)) && (!(scratch.values[1912] != 0.0))) {
            scratch.store_ad(1357, &AdValue::mul(scratch.ad_value(597), scratch.ad_value(1347)));
        }

        scratch.values[1914] = if ((scratch.values[562] == 0.0) && (scratch.values[565] == 0.0)) { 1.0 } else { 0.0 };

        if ((((scratch.values[1320] != 0.0) && (scratch.values[1337] != 0.0)) && (!(scratch.values[1912] != 0.0))) && (scratch.values[1914] != 0.0)) {
            scratch.values[1358] = 0.0;
            scratch.node_derivatives[1358] = [0.0; Instance::NODE_COUNT];
            scratch.branch_derivatives[1358] = [0.0; Instance::BRANCH_COUNT];
        }

        if ((((scratch.values[1320] != 0.0) && (scratch.values[1337] != 0.0)) && (!(scratch.values[1912] != 0.0))) && (!(scratch.values[1914] != 0.0))) {
            scratch.store_ad(1359, &AdValue::sub(scratch.ad_value(603), scratch.ad_value(1353)));
        }

        if ((((scratch.values[1320] != 0.0) && (scratch.values[1337] != 0.0)) && (!(scratch.values[1912] != 0.0))) && (!(scratch.values[1914] != 0.0))) {
            scratch.store_ad(1360, &AdValue::sub_from_scalar(1.0, AdValue::sqrt(AdValue::sub_from_scalar(1.0, AdValue::div(scratch.ad_value(1351), scratch.ad_value(1359))))));
        }

        scratch.values[1915] = if (scratch.values[551] == 0.5) { 1.0 } else { 0.0 };

    }

    pub(super) fn stamp_transient_block_27(
        &mut self,
        ctx: &GeneratedEvalContext<'_>,
        stamper: &mut GeneratedStamper<'_>,
        scratch: &mut Scratch,
    ) {
        let _ = stamper;
        if (((((scratch.values[1320] != 0.0) && (scratch.values[1337] != 0.0)) && (!(scratch.values[1912] != 0.0))) && (!(scratch.values[1914] != 0.0))) && (scratch.values[1915] != 0.0)) {
            scratch.values[1361] = 0.0;
            scratch.node_derivatives[1361] = [0.0; Instance::NODE_COUNT];
            scratch.branch_derivatives[1361] = [0.0; Instance::BRANCH_COUNT];
        }

        if (((((scratch.values[1320] != 0.0) && (scratch.values[1337] != 0.0)) && (!(scratch.values[1912] != 0.0))) && (!(scratch.values[1914] != 0.0))) && (!(scratch.values[1915] != 0.0))) {
            scratch.store_ad(1361, &AdValue::mul(AdValue::add(AdValue::div(AdValue::mul(AdValue::square(scratch.ad_value(1360)), AdValue::ln(scratch.ad_value(1360))), AdValue::sub_from_scalar(1.0, scratch.ad_value(1360))), scratch.ad_value(1360)), AdValue::sub_from_scalar(1.0, AdValue::scale(scratch.ad_value(551), 2.0))));
        }

        if ((((scratch.values[1320] != 0.0) && (scratch.values[1337] != 0.0)) && (!(scratch.values[1912] != 0.0))) && (!(scratch.values[1914] != 0.0))) {
            scratch.store_ad(1362, &AdValue::add(scratch.ad_value(1360), scratch.ad_value(1361)));
        }

        scratch.values[1916] = if (scratch.values[551] == 0.5) { 1.0 } else { 0.0 };

        if (((((scratch.values[1320] != 0.0) && (scratch.values[1337] != 0.0)) && (!(scratch.values[1912] != 0.0))) && (!(scratch.values[1914] != 0.0))) && (scratch.values[1916] != 0.0)) {
            scratch.store_ad(1356, &AdValue::sqrt(AdValue::mul(scratch.ad_value(1359), scratch.ad_value(630))));
        }

        if (((((scratch.values[1320] != 0.0) && (scratch.values[1337] != 0.0)) && (!(scratch.values[1912] != 0.0))) && (!(scratch.values[1914] != 0.0))) && (!(scratch.values[1916] != 0.0))) {
            scratch.store_ad(1356, &AdValue::pow(AdValue::mul(scratch.ad_value(1359), scratch.ad_value(630)), scratch.ad_value(551)));
        }

        if ((((scratch.values[1320] != 0.0) && (scratch.values[1337] != 0.0)) && (!(scratch.values[1912] != 0.0))) && (!(scratch.values[1914] != 0.0))) {
            scratch.store_ad(1363, &AdValue::mul(scratch.ad_value(624), scratch.ad_value(1356)));
        }

        if ((((scratch.values[1320] != 0.0) && (scratch.values[1337] != 0.0)) && (!(scratch.values[1912] != 0.0))) && (!(scratch.values[1914] != 0.0))) {
            scratch.store_ad(1364, &AdValue::mul(scratch.ad_value(594), AdValue::mul(AdValue::offset(scratch.ad_value(1350), (-1.0)), scratch.ad_value(1363))));
        }

        if ((((scratch.values[1320] != 0.0) && (scratch.values[1337] != 0.0)) && (!(scratch.values[1912] != 0.0))) && (!(scratch.values[1914] != 0.0))) {
            scratch.store_ad(1358, &AdValue::mul(scratch.ad_value(562), AdValue::mul(scratch.ad_value(1364), scratch.ad_value(1362))));
        }

        scratch.values[1917] = if (scratch.values[565] == 0.0) { 1.0 } else { 0.0 };

        if ((((scratch.values[1320] != 0.0) && (scratch.values[1337] != 0.0)) && (!(scratch.values[1912] != 0.0))) && (scratch.values[1917] != 0.0)) {
            scratch.values[1365] = 0.0;
            scratch.node_derivatives[1365] = [0.0; Instance::NODE_COUNT];
            scratch.branch_derivatives[1365] = [0.0; Instance::BRANCH_COUNT];
        }

        if ((((scratch.values[1320] != 0.0) && (scratch.values[1337] != 0.0)) && (!(scratch.values[1912] != 0.0))) && (!(scratch.values[1917] != 0.0))) {
            scratch.store_ad(1366, &AdValue::mul(scratch.ad_value(639), AdValue::div(AdValue::mul(scratch.ad_value(1363), scratch.ad_value(609)), scratch.ad_value(1359))));
        }

        if ((((scratch.values[1320] != 0.0) && (scratch.values[1337] != 0.0)) && (!(scratch.values[1912] != 0.0))) && (!(scratch.values[1917] != 0.0))) {
            scratch.store_ad(1367, &AdValue::div(AdValue::scale(scratch.ad_value(636), 0.666666666666667), scratch.ad_value(1366)));
        }

        if ((((scratch.values[1320] != 0.0) && (scratch.values[1337] != 0.0)) && (!(scratch.values[1912] != 0.0))) && (!(scratch.values[1917] != 0.0))) {
            scratch.store_ad(1368, &AdValue::square(scratch.ad_value(1367)));
        }

        if ((((scratch.values[1320] != 0.0) && (scratch.values[1337] != 0.0)) && (!(scratch.values[1912] != 0.0))) && (!(scratch.values[1917] != 0.0))) {
            scratch.store_ad(1369, &AdValue::sqrt(AdValue::div(AdValue::square(scratch.ad_value(1368)), AdValue::offset(AdValue::square(scratch.ad_value(1368)), 1.0))));
        }

        if ((((scratch.values[1320] != 0.0) && (scratch.values[1337] != 0.0)) && (!(scratch.values[1912] != 0.0))) && (!(scratch.values[1917] != 0.0))) {
            scratch.store_ad(1370, &AdValue::sqrt(AdValue::abs(scratch.ad_value(1369))));
        }

        if ((((scratch.values[1320] != 0.0) && (scratch.values[1337] != 0.0)) && (!(scratch.values[1912] != 0.0))) && (!(scratch.values[1917] != 0.0))) {
            scratch.store_ad(1371, &AdValue::mul(scratch.ad_value(1369), scratch.ad_value(1370)));
        }

        scratch.values[1918] = if (((-scratch.values[551]) * scratch.values[612]) == (-1.0)) { 1.0 } else { 0.0 };

        if (((((scratch.values[1320] != 0.0) && (scratch.values[1337] != 0.0)) && (!(scratch.values[1912] != 0.0))) && (!(scratch.values[1917] != 0.0))) && (scratch.values[1918] != 0.0)) {
            scratch.store_ad(1372, &AdValue::div_from_scalar(1.0, AdValue::offset(AdValue::mul(scratch.ad_value(1366), scratch.ad_value(1371)), 1.0)));
        }

        if (((((scratch.values[1320] != 0.0) && (scratch.values[1337] != 0.0)) && (!(scratch.values[1912] != 0.0))) && (!(scratch.values[1917] != 0.0))) && (!(scratch.values[1918] != 0.0))) {
            scratch.store_ad(1372, &AdValue::pow(AdValue::offset(AdValue::mul(scratch.ad_value(1366), scratch.ad_value(1371)), 1.0), AdValue::mul(AdValue::neg(scratch.ad_value(551)), scratch.ad_value(612))));
        }

        if ((((scratch.values[1320] != 0.0) && (scratch.values[1337] != 0.0)) && (!(scratch.values[1912] != 0.0))) && (!(scratch.values[1917] != 0.0))) {
            scratch.store_ad(1373, &AdValue::div(AdValue::mul(scratch.ad_value(1362), scratch.ad_value(1372)), AdValue::add(scratch.ad_value(1362), scratch.ad_value(1372))));
        }

        if ((((scratch.values[1320] != 0.0) && (scratch.values[1337] != 0.0)) && (!(scratch.values[1912] != 0.0))) && (!(scratch.values[1917] != 0.0))) {
            scratch.store_ad(1374, &AdValue::sqrt(AdValue::scale(AdValue::div(scratch.ad_value(1366), scratch.ad_value(1370)), 0.375)));
        }

        if ((((scratch.values[1320] != 0.0) && (scratch.values[1337] != 0.0)) && (!(scratch.values[1912] != 0.0))) && (!(scratch.values[1917] != 0.0))) {
            scratch.store_ad(1375, &AdValue::sub(AdValue::scale(AdValue::mul(scratch.ad_value(1367), scratch.ad_value(1370)), 2.0), scratch.ad_value(1369)));
        }

        if ((((scratch.values[1320] != 0.0) && (scratch.values[1337] != 0.0)) && (!(scratch.values[1912] != 0.0))) && (!(scratch.values[1917] != 0.0))) {
            scratch.store_ad(1376, &AdValue::add(AdValue::sub(AdValue::mul(AdValue::mul(scratch.ad_value(636), scratch.ad_value(1367)), scratch.ad_value(1370)), AdValue::mul(scratch.ad_value(636), scratch.ad_value(1369))), AdValue::scale(AdValue::mul(scratch.ad_value(1366), scratch.ad_value(1371)), 0.5)));
        }

        if ((((scratch.values[1320] != 0.0) && (scratch.values[1337] != 0.0)) && (!(scratch.values[1912] != 0.0))) && (!(scratch.values[1917] != 0.0))) {
            scratch.store_ad(1377, &AdValue::mul(AdValue::offset(scratch.ad_value(1375), (-1.0)), scratch.ad_value(1374)));
        }

        if ((((scratch.values[1320] != 0.0) && (scratch.values[1337] != 0.0)) && (!(scratch.values[1912] != 0.0))) && (!(scratch.values[1917] != 0.0))) {
            scratch.store_ad(1338, &AdValue::square(scratch.ad_value(1377)));
        }

        scratch.values[1919] = if (scratch.values[1377] > 0.0) { 1.0 } else { 0.0 };

        if (((((scratch.values[1320] != 0.0) && (scratch.values[1337] != 0.0)) && (!(scratch.values[1912] != 0.0))) && (!(scratch.values[1917] != 0.0))) && (scratch.values[1919] != 0.0)) {
            scratch.store_ad(1339, &AdValue::div_from_scalar(1.0, AdValue::offset(AdValue::scale(scratch.ad_value(1377), scratch.values[428]), 1.0)));
        }

        if (((((scratch.values[1320] != 0.0) && (scratch.values[1337] != 0.0)) && (!(scratch.values[1912] != 0.0))) && (!(scratch.values[1917] != 0.0))) && (!(scratch.values[1919] != 0.0))) {
            scratch.store_ad(1339, &AdValue::div_from_scalar(1.0, AdValue::sub_from_scalar(1.0, AdValue::scale(scratch.ad_value(1377), scratch.values[428]))));
        }

        scratch.values[1920] = if (((-scratch.values[1338]) + scratch.values[1376]) > (-230.25850929940458)) { 1.0 } else { 0.0 };

        if (((((scratch.values[1320] != 0.0) && (scratch.values[1337] != 0.0)) && (!(scratch.values[1912] != 0.0))) && (!(scratch.values[1917] != 0.0))) && (scratch.values[1920] != 0.0)) {
            scratch.store_ad(1356, &AdValue::exp(AdValue::sub(scratch.ad_value(1376), scratch.ad_value(1338))));
        }

        if (((((scratch.values[1320] != 0.0) && (scratch.values[1337] != 0.0)) && (!(scratch.values[1912] != 0.0))) && (!(scratch.values[1917] != 0.0))) && (!(scratch.values[1920] != 0.0))) {
            let assign35930_ad_e46778: AdValue = AdValue::div_from_scalar(1e-100, AdValue::offset(AdValue::mul(AdValue::sub_from_scalar((-230.25850929940458), AdValue::sub(scratch.ad_value(1376), scratch.ad_value(1338))), AdValue::offset(AdValue::scale(AdValue::mul(AdValue::sub_from_scalar((-230.25850929940458), AdValue::sub(scratch.ad_value(1376), scratch.ad_value(1338))), AdValue::offset(AdValue::scale(AdValue::sub_from_scalar((-230.25850929940458), AdValue::sub(scratch.ad_value(1376), scratch.ad_value(1338))), 0.3333333333333333), 1.0)), 0.5), 1.0)), 1.0));
            scratch.store_ad(1356, &assign35930_ad_e46778);
        }

        if ((((scratch.values[1320] != 0.0) && (scratch.values[1337] != 0.0)) && (!(scratch.values[1912] != 0.0))) && (!(scratch.values[1917] != 0.0))) {
            scratch.store_ad(1340, &AdValue::mul(AdValue::add(AdValue::add(AdValue::scale(scratch.ad_value(1339), 0.29214664), AdValue::scale(AdValue::square(scratch.ad_value(1339)), scratch.values[429])), AdValue::scale(AdValue::mul(AdValue::square(scratch.ad_value(1339)), scratch.ad_value(1339)), scratch.values[430])), scratch.ad_value(1356)));
        }

        scratch.values[1921] = if (scratch.values[1377] > 0.0) { 1.0 } else { 0.0 };

        if (((((scratch.values[1320] != 0.0) && (scratch.values[1337] != 0.0)) && (!(scratch.values[1912] != 0.0))) && (!(scratch.values[1917] != 0.0))) && (scratch.values[1921] != 0.0)) {
            scratch.values[1378] = scratch.values[1340];
            scratch.node_derivatives[1378] = scratch.node_derivatives[1340];
            scratch.branch_derivatives[1378] = scratch.branch_derivatives[1340];
        }

        scratch.values[1922] = if (scratch.values[1376] > (-230.25850929940458)) { 1.0 } else { 0.0 };

        if ((((((scratch.values[1320] != 0.0) && (scratch.values[1337] != 0.0)) && (!(scratch.values[1912] != 0.0))) && (!(scratch.values[1917] != 0.0))) && (!(scratch.values[1921] != 0.0))) && (scratch.values[1922] != 0.0)) {
            scratch.store_ad(1356, &AdValue::exp(scratch.ad_value(1376)));
        }

        if ((((((scratch.values[1320] != 0.0) && (scratch.values[1337] != 0.0)) && (!(scratch.values[1912] != 0.0))) && (!(scratch.values[1917] != 0.0))) && (!(scratch.values[1921] != 0.0))) && (!(scratch.values[1922] != 0.0))) {
            scratch.store_ad(1356, &AdValue::div_from_scalar(1e-100, AdValue::offset(AdValue::mul(AdValue::sub_from_scalar((-230.25850929940458), scratch.ad_value(1376)), AdValue::offset(AdValue::scale(AdValue::mul(AdValue::sub_from_scalar((-230.25850929940458), scratch.ad_value(1376)), AdValue::offset(AdValue::scale(AdValue::sub_from_scalar((-230.25850929940458), scratch.ad_value(1376)), 0.3333333333333333), 1.0)), 0.5), 1.0)), 1.0)));
        }

        if (((((scratch.values[1320] != 0.0) && (scratch.values[1337] != 0.0)) && (!(scratch.values[1912] != 0.0))) && (!(scratch.values[1917] != 0.0))) && (!(scratch.values[1921] != 0.0))) {
            scratch.store_ad(1378, &AdValue::sub(AdValue::scale(scratch.ad_value(1356), 2.0), scratch.ad_value(1340)));
        }

        if ((((scratch.values[1320] != 0.0) && (scratch.values[1337] != 0.0)) && (!(scratch.values[1912] != 0.0))) && (!(scratch.values[1917] != 0.0))) {
            scratch.store_ad(1379, &AdValue::scale(AdValue::div(AdValue::mul(scratch.ad_value(636), scratch.ad_value(1378)), scratch.ad_value(1374)), (1.772453850905516 * 0.5)));
        }

        if ((((scratch.values[1320] != 0.0) && (scratch.values[1337] != 0.0)) && (!(scratch.values[1912] != 0.0))) && (!(scratch.values[1917] != 0.0))) {
            scratch.store_ad(1365, &AdValue::mul(scratch.ad_value(565), AdValue::mul(AdValue::mul(scratch.ad_value(1364), scratch.ad_value(1379)), scratch.ad_value(1373))));
        }

        scratch.values[1923] = if (scratch.values[571] == 0.0) { 1.0 } else { 0.0 };

        if ((((scratch.values[1320] != 0.0) && (scratch.values[1337] != 0.0)) && (!(scratch.values[1912] != 0.0))) && (scratch.values[1923] != 0.0)) {
            scratch.values[1380] = 0.0;
            scratch.node_derivatives[1380] = [0.0; Instance::NODE_COUNT];
            scratch.branch_derivatives[1380] = [0.0; Instance::BRANCH_COUNT];
        }

        scratch.values[1924] = if (scratch.values[551] == 0.5) { 1.0 } else { 0.0 };

        if (((((scratch.values[1320] != 0.0) && (scratch.values[1337] != 0.0)) && (!(scratch.values[1912] != 0.0))) && (!(scratch.values[1923] != 0.0))) && (scratch.values[1924] != 0.0)) {
            scratch.store_ad(1356, &AdValue::sqrt(AdValue::mul(AdValue::sub(scratch.ad_value(548), scratch.ad_value(1354)), scratch.ad_value(630))));
        }

        if (((((scratch.values[1320] != 0.0) && (scratch.values[1337] != 0.0)) && (!(scratch.values[1912] != 0.0))) && (!(scratch.values[1923] != 0.0))) && (!(scratch.values[1924] != 0.0))) {
            scratch.store_ad(1356, &AdValue::pow(AdValue::mul(AdValue::sub(scratch.ad_value(548), scratch.ad_value(1354)), scratch.ad_value(630)), scratch.ad_value(551)));
        }

        if ((((scratch.values[1320] != 0.0) && (scratch.values[1337] != 0.0)) && (!(scratch.values[1912] != 0.0))) && (!(scratch.values[1923] != 0.0))) {
            scratch.store_ad(1381, &AdValue::mul(scratch.ad_value(612), AdValue::div(AdValue::mul(AdValue::sub(scratch.ad_value(548), scratch.ad_value(1354)), scratch.ad_value(627)), scratch.ad_value(1356))));
        }

        scratch.values[1925] = if (((((-scratch.values[642]) / scratch.values[1381])) as f64).abs() < 230.25850929940458) { 1.0 } else { 0.0 };

        if (((((scratch.values[1320] != 0.0) && (scratch.values[1337] != 0.0)) && (!(scratch.values[1912] != 0.0))) && (!(scratch.values[1923] != 0.0))) && (scratch.values[1925] != 0.0)) {
            scratch.store_ad(1356, &AdValue::exp(AdValue::div(AdValue::neg(scratch.ad_value(642)), scratch.ad_value(1381))));
        }

        scratch.values[1926] = if (((-scratch.values[642]) / scratch.values[1381]) < 0.0) { 1.0 } else { 0.0 };

        if ((((((scratch.values[1320] != 0.0) && (scratch.values[1337] != 0.0)) && (!(scratch.values[1912] != 0.0))) && (!(scratch.values[1923] != 0.0))) && (!(scratch.values[1925] != 0.0))) && (scratch.values[1926] != 0.0)) {
            let assign36120_ad_e47105: AdValue = AdValue::offset(AdValue::mul(AdValue::sub_from_scalar((-230.25850929940458), AdValue::div(AdValue::neg(scratch.ad_value(642)), scratch.ad_value(1381))), AdValue::offset(AdValue::scale(AdValue::mul(AdValue::sub_from_scalar((-230.25850929940458), AdValue::div(AdValue::neg(scratch.ad_value(642)), scratch.ad_value(1381))), AdValue::offset(AdValue::scale(AdValue::sub_from_scalar((-230.25850929940458), AdValue::div(AdValue::neg(scratch.ad_value(642)), scratch.ad_value(1381))), 0.3333333333333333), 1.0)), 0.5), 1.0)), 1.0);
            scratch.store_ad(1356, &AdValue::div_from_scalar(1e-100, assign36120_ad_e47105));
        }

        if ((((((scratch.values[1320] != 0.0) && (scratch.values[1337] != 0.0)) && (!(scratch.values[1912] != 0.0))) && (!(scratch.values[1923] != 0.0))) && (!(scratch.values[1925] != 0.0))) && (!(scratch.values[1926] != 0.0))) {
            let assign36130_ad_e47155: AdValue = AdValue::scale(AdValue::offset(AdValue::mul(AdValue::offset(AdValue::div(AdValue::neg(scratch.ad_value(642)), scratch.ad_value(1381)), (-230.25850929940458)), AdValue::offset(AdValue::scale(AdValue::mul(AdValue::offset(AdValue::div(AdValue::neg(scratch.ad_value(642)), scratch.ad_value(1381)), (-230.25850929940458)), AdValue::offset(AdValue::scale(AdValue::offset(AdValue::div(AdValue::neg(scratch.ad_value(642)), scratch.ad_value(1381)), (-230.25850929940458)), 0.3333333333333333), 1.0)), 0.5), 1.0)), 1.0), 1e100);
            scratch.store_ad(1356, &assign36130_ad_e47155);
        }

        if ((((scratch.values[1320] != 0.0) && (scratch.values[1337] != 0.0)) && (!(scratch.values[1912] != 0.0))) && (!(scratch.values[1923] != 0.0))) {
            scratch.store_ad(1380, &AdValue::mul(scratch.ad_value(571), AdValue::mul(AdValue::mul(AdValue::mul(scratch.ad_value(526), scratch.ad_value(1381)), scratch.ad_value(1381)), scratch.ad_value(1356))));
        }

        scratch.values[1927] = if (scratch.values[580] > 1000.0) { 1.0 } else { 0.0 };

        if ((((scratch.values[1320] != 0.0) && (scratch.values[1337] != 0.0)) && (!(scratch.values[1912] != 0.0))) && (scratch.values[1927] != 0.0)) {
            scratch.values[1382] = 1.0;
            scratch.node_derivatives[1382] = [0.0; Instance::NODE_COUNT];
            scratch.branch_derivatives[1382] = [0.0; Instance::BRANCH_COUNT];
        }

        scratch.values[1928] = if (scratch.values[1355] > ((-scratch.values[500]) * scratch.values[580])) { 1.0 } else { 0.0 };

        scratch.values[1929] = if (scratch.values[583] == 4.0) { 1.0 } else { 0.0 };

        if ((((((scratch.values[1320] != 0.0) && (scratch.values[1337] != 0.0)) && (!(scratch.values[1912] != 0.0))) && (!(scratch.values[1927] != 0.0))) && (scratch.values[1928] != 0.0)) && (scratch.values[1929] != 0.0)) {
            scratch.store_ad(1356, &AdValue::mul(AdValue::mul(AdValue::mul(AdValue::mul(scratch.ad_value(1355), scratch.ad_value(648)), AdValue::mul(scratch.ad_value(1355), scratch.ad_value(648))), AdValue::mul(scratch.ad_value(1355), scratch.ad_value(648))), AdValue::mul(scratch.ad_value(1355), scratch.ad_value(648))));
        }

        if ((((((scratch.values[1320] != 0.0) && (scratch.values[1337] != 0.0)) && (!(scratch.values[1912] != 0.0))) && (!(scratch.values[1927] != 0.0))) && (scratch.values[1928] != 0.0)) && (!(scratch.values[1929] != 0.0))) {
            scratch.store_ad(1356, &AdValue::pow(AdValue::abs(AdValue::mul(scratch.ad_value(1355), scratch.ad_value(648))), scratch.ad_value(583)));
        }

        if (((((scratch.values[1320] != 0.0) && (scratch.values[1337] != 0.0)) && (!(scratch.values[1912] != 0.0))) && (!(scratch.values[1927] != 0.0))) && (scratch.values[1928] != 0.0)) {
            scratch.store_ad(1382, &AdValue::div_from_scalar(1.0, AdValue::sub_from_scalar(1.0, scratch.ad_value(1356))));
        }

        if (((((scratch.values[1320] != 0.0) && (scratch.values[1337] != 0.0)) && (!(scratch.values[1912] != 0.0))) && (!(scratch.values[1927] != 0.0))) && (!(scratch.values[1928] != 0.0))) {
            scratch.store_ad(1382, &AdValue::add(scratch.ad_value(645), AdValue::mul(AdValue::add(scratch.ad_value(1355), AdValue::scale(scratch.ad_value(580), scratch.values[500])), scratch.ad_value(651))));
        }

        if (((scratch.values[1320] != 0.0) && (scratch.values[1337] != 0.0)) && (!(scratch.values[1912] != 0.0))) {
            scratch.store_ad(1385, &AdValue::mul(AdValue::add(AdValue::add(AdValue::add(scratch.ad_value(1357), scratch.ad_value(1358)), scratch.ad_value(1365)), scratch.ad_value(1380)), scratch.ad_value(1382)));
        }

        if ((scratch.values[1320] != 0.0) && (scratch.values[1337] != 0.0)) {
            scratch.store_ad(516, &AdValue::add(AdValue::add(AdValue::mul(scratch.ad_value(724), scratch.ad_value(1383)), AdValue::mul(scratch.ad_value(725), scratch.ad_value(1384))), AdValue::mul(scratch.ad_value(726), scratch.ad_value(1385))));
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

        scratch.values[1930] = if !(((scratch.values[724] == 0.0) && (scratch.values[725] == 0.0)) && (scratch.values[726] == 0.0)) { 1.0 } else { 0.0 };

        if (((scratch.values[1320] != 0.0) && (scratch.values[1337] != 0.0)) && (scratch.values[1930] != 0.0)) {
            scratch.store_ad(1341, &AdValue::mul(AdValue::scale(scratch.ad_value(735), 4.0), scratch.ad_value(735)));
        }

        if (((scratch.values[1320] != 0.0) && (scratch.values[1337] != 0.0)) && (scratch.values[1930] != 0.0)) {
            scratch.store_ad(1342, &AdValue::div(scratch.ad_value(735), scratch.ad_value(736)));
        }

        if (((scratch.values[1320] != 0.0) && (scratch.values[1337] != 0.0)) && (scratch.values[1930] != 0.0)) {
            scratch.store_ad(1343, &AdValue::add(scratch.ad_value(527), AdValue::mul(scratch.ad_value(735), scratch.ad_value(1342))));
        }

        if (((scratch.values[1320] != 0.0) && (scratch.values[1337] != 0.0)) && (scratch.values[1930] != 0.0)) {
            scratch.store_ad(1344, &AdValue::add(scratch.ad_value(736), scratch.ad_value(1343)));
        }

        if (((scratch.values[1320] != 0.0) && (scratch.values[1337] != 0.0)) && (scratch.values[1930] != 0.0)) {
            scratch.store_ad(1345, &AdValue::sub(scratch.ad_value(736), scratch.ad_value(1343)));
        }

        if (((scratch.values[1320] != 0.0) && (scratch.values[1337] != 0.0)) && (scratch.values[1930] != 0.0)) {
            scratch.store_ad(1346, &AdValue::sqrt(AdValue::add(AdValue::square(scratch.ad_value(1345)), scratch.ad_value(1341))));
        }

        if (((scratch.values[1320] != 0.0) && (scratch.values[1337] != 0.0)) && (scratch.values[1930] != 0.0)) {
            scratch.store_ad(1348, &AdValue::scale(AdValue::div(AdValue::mul(scratch.ad_value(527), scratch.ad_value(736)), AdValue::add(scratch.ad_value(1344), scratch.ad_value(1346))), 2.0));
        }

        scratch.values[1931] = if (scratch.values[527] < scratch.values[732]) { 1.0 } else { 0.0 };

        scratch.values[1932] = if ((((0.5 * (scratch.values[527] * scratch.values[427]))) as f64).abs() < 230.25850929940458) { 1.0 } else { 0.0 };

        if (((((scratch.values[1320] != 0.0) && (scratch.values[1337] != 0.0)) && (scratch.values[1930] != 0.0)) && (scratch.values[1931] != 0.0)) && (scratch.values[1932] != 0.0)) {
            scratch.store_ad(1350, &AdValue::exp(AdValue::scale(scratch.ad_value(527), (scratch.values[427] * 0.5))));
        }

        scratch.values[1933] = if ((0.5 * (scratch.values[527] * scratch.values[427])) < 0.0) { 1.0 } else { 0.0 };

        if ((((((scratch.values[1320] != 0.0) && (scratch.values[1337] != 0.0)) && (scratch.values[1930] != 0.0)) && (scratch.values[1931] != 0.0)) && (!(scratch.values[1932] != 0.0))) && (scratch.values[1933] != 0.0)) {
            let assign36390_ad_e47518: AdValue = AdValue::div_from_scalar(1e-100, AdValue::offset(AdValue::mul(AdValue::sub_from_scalar((-230.25850929940458), AdValue::scale(scratch.ad_value(527), (scratch.values[427] * 0.5))), AdValue::offset(AdValue::scale(AdValue::mul(AdValue::sub_from_scalar((-230.25850929940458), AdValue::scale(scratch.ad_value(527), (scratch.values[427] * 0.5))), AdValue::offset(AdValue::scale(AdValue::sub_from_scalar((-230.25850929940458), AdValue::scale(scratch.ad_value(527), (scratch.values[427] * 0.5))), 0.3333333333333333), 1.0)), 0.5), 1.0)), 1.0));
            scratch.store_ad(1350, &assign36390_ad_e47518);
        }

        if ((((((scratch.values[1320] != 0.0) && (scratch.values[1337] != 0.0)) && (scratch.values[1930] != 0.0)) && (scratch.values[1931] != 0.0)) && (!(scratch.values[1932] != 0.0))) && (!(scratch.values[1933] != 0.0))) {
            scratch.store_ad(1350, &AdValue::scale(AdValue::offset(AdValue::mul(AdValue::offset(AdValue::scale(scratch.ad_value(527), (scratch.values[427] * 0.5)), (-230.25850929940458)), AdValue::offset(AdValue::scale(AdValue::mul(AdValue::offset(AdValue::scale(scratch.ad_value(527), (scratch.values[427] * 0.5)), (-230.25850929940458)), AdValue::offset(AdValue::scale(AdValue::offset(AdValue::scale(scratch.ad_value(527), (scratch.values[427] * 0.5)), (-230.25850929940458)), 0.3333333333333333), 1.0)), 0.5), 1.0)), 1.0), 1e100));
        }

        if ((((scratch.values[1320] != 0.0) && (scratch.values[1337] != 0.0)) && (scratch.values[1930] != 0.0)) && (scratch.values[1931] != 0.0)) {
            scratch.store_ad(1347, &AdValue::square(scratch.ad_value(1350)));
        }

        if ((((scratch.values[1320] != 0.0) && (scratch.values[1337] != 0.0)) && (scratch.values[1930] != 0.0)) && (!(scratch.values[1931] != 0.0))) {
            scratch.store_ad(1347, &AdValue::mul(AdValue::offset(AdValue::scale(AdValue::sub(scratch.ad_value(527), scratch.ad_value(732)), scratch.values[427]), 1.0), scratch.ad_value(733)));
        }

        if ((((scratch.values[1320] != 0.0) && (scratch.values[1337] != 0.0)) && (scratch.values[1930] != 0.0)) && (!(scratch.values[1931] != 0.0))) {
            scratch.store_ad(1350, &AdValue::sqrt(scratch.ad_value(1347)));
        }

        if (((scratch.values[1320] != 0.0) && (scratch.values[1337] != 0.0)) && (scratch.values[1930] != 0.0)) {
            scratch.store_ad(1347, &AdValue::offset(scratch.ad_value(1347), (-1.0)));
        }

        if (((scratch.values[1320] != 0.0) && (scratch.values[1337] != 0.0)) && (scratch.values[1930] != 0.0)) {
            scratch.store_ad(1349, &AdValue::div_from_scalar(1.0, scratch.ad_value(1350)));
        }

        scratch.values[1934] = if (scratch.values[527] > 0.0) { 1.0 } else { 0.0 };

        if ((((scratch.values[1320] != 0.0) && (scratch.values[1337] != 0.0)) && (scratch.values[1930] != 0.0)) && (scratch.values[1934] != 0.0)) {
            scratch.store_ad(1351, &AdValue::scale(AdValue::ln(AdValue::add(AdValue::offset(scratch.ad_value(1349), 2.0), AdValue::sqrt(AdValue::mul(AdValue::offset(scratch.ad_value(1349), 1.0), AdValue::offset(scratch.ad_value(1349), 3.0))))), (scratch.values[426] * 2.0)));
        }

        if ((((scratch.values[1320] != 0.0) && (scratch.values[1337] != 0.0)) && (scratch.values[1930] != 0.0)) && (!(scratch.values[1934] != 0.0))) {
            scratch.store_ad(1351, &AdValue::sub(AdValue::scale(AdValue::ln(AdValue::add(AdValue::offset(AdValue::scale(scratch.ad_value(1350), 2.0), 1.0), AdValue::sqrt(AdValue::mul(AdValue::offset(scratch.ad_value(1350), 1.0), AdValue::offset(AdValue::scale(scratch.ad_value(1350), 3.0), 1.0))))), (scratch.values[426] * 2.0)), scratch.ad_value(527)));
        }

        if (((scratch.values[1320] != 0.0) && (scratch.values[1337] != 0.0)) && (scratch.values[1930] != 0.0)) {
            scratch.store_ad(1352, &AdValue::sub(scratch.ad_value(734), scratch.ad_value(1351)));
        }

        if (((scratch.values[1320] != 0.0) && (scratch.values[1337] != 0.0)) && (scratch.values[1930] != 0.0)) {
            scratch.store_ad(1353, &AdValue::scale(AdValue::sub(AdValue::add(scratch.ad_value(527), scratch.ad_value(1352)), AdValue::sqrt(AdValue::offset(AdValue::mul(AdValue::sub(scratch.ad_value(527), scratch.ad_value(1352)), AdValue::sub(scratch.ad_value(527), scratch.ad_value(1352))), ((4.0 * scratch.values[426]) * scratch.values[426])))), 0.5));
        }

        if (((scratch.values[1320] != 0.0) && (scratch.values[1337] != 0.0)) && (scratch.values[1930] != 0.0)) {
            scratch.store_ad(1354, &AdValue::scale(AdValue::sub(AdValue::add(scratch.ad_value(527), scratch.ad_value(737)), AdValue::sqrt(AdValue::offset(AdValue::mul(AdValue::sub(scratch.ad_value(527), scratch.ad_value(737)), AdValue::sub(scratch.ad_value(527), scratch.ad_value(737))), ((4.0 * scratch.values[424]) * scratch.values[424])))), 0.5));
        }

        if (((scratch.values[1320] != 0.0) && (scratch.values[1337] != 0.0)) && (scratch.values[1930] != 0.0)) {
            scratch.store_ad(1355, &AdValue::scale(AdValue::sub(scratch.ad_value(527), AdValue::sqrt(AdValue::offset(AdValue::mul(scratch.ad_value(527), scratch.ad_value(527)), ((4.0 * 1e-6) * 1e-6)))), 0.5));
        }

        scratch.values[1935] = if (scratch.values[724] == 0.0) { 1.0 } else { 0.0 };

        if (((scratch.values[1320] != 0.0) && (scratch.values[1337] != 0.0)) && (scratch.values[1935] != 0.0)) {
            scratch.values[1383] = 0.0;
            scratch.node_derivatives[1383] = [0.0; Instance::NODE_COUNT];
            scratch.branch_derivatives[1383] = [0.0; Instance::BRANCH_COUNT];
        }

        scratch.values[1936] = if (scratch.values[607] == 0.5) { 1.0 } else { 0.0 };

        if ((((scratch.values[1320] != 0.0) && (scratch.values[1337] != 0.0)) && (!(scratch.values[1935] != 0.0))) && (scratch.values[1936] != 0.0)) {
            scratch.store_ad(1356, &AdValue::sqrt(AdValue::sub_from_scalar(1.0, AdValue::mul(scratch.ad_value(1348), scratch.ad_value(604)))));
        }

        if ((((scratch.values[1320] != 0.0) && (scratch.values[1337] != 0.0)) && (!(scratch.values[1935] != 0.0))) && (!(scratch.values[1936] != 0.0))) {
            scratch.store_ad(1356, &AdValue::pow(AdValue::sub_from_scalar(1.0, AdValue::mul(scratch.ad_value(1348), scratch.ad_value(604))), scratch.ad_value(607)));
        }

        if (((scratch.values[1320] != 0.0) && (scratch.values[1337] != 0.0)) && (!(scratch.values[1935] != 0.0))) {
            scratch.store_ad(1357, &AdValue::mul(scratch.ad_value(595), scratch.ad_value(1347)));
        }

        scratch.values[1937] = if ((scratch.values[560] == 0.0) && (scratch.values[563] == 0.0)) { 1.0 } else { 0.0 };

        if ((((scratch.values[1320] != 0.0) && (scratch.values[1337] != 0.0)) && (!(scratch.values[1935] != 0.0))) && (scratch.values[1937] != 0.0)) {
            scratch.values[1358] = 0.0;
            scratch.node_derivatives[1358] = [0.0; Instance::NODE_COUNT];
            scratch.branch_derivatives[1358] = [0.0; Instance::BRANCH_COUNT];
        }

        if ((((scratch.values[1320] != 0.0) && (scratch.values[1337] != 0.0)) && (!(scratch.values[1935] != 0.0))) && (!(scratch.values[1937] != 0.0))) {
            scratch.store_ad(1359, &AdValue::sub(scratch.ad_value(601), scratch.ad_value(1353)));
        }

        if ((((scratch.values[1320] != 0.0) && (scratch.values[1337] != 0.0)) && (!(scratch.values[1935] != 0.0))) && (!(scratch.values[1937] != 0.0))) {
            scratch.store_ad(1360, &AdValue::sub_from_scalar(1.0, AdValue::sqrt(AdValue::sub_from_scalar(1.0, AdValue::div(scratch.ad_value(1351), scratch.ad_value(1359))))));
        }

        scratch.values[1938] = if (scratch.values[549] == 0.5) { 1.0 } else { 0.0 };

        if (((((scratch.values[1320] != 0.0) && (scratch.values[1337] != 0.0)) && (!(scratch.values[1935] != 0.0))) && (!(scratch.values[1937] != 0.0))) && (scratch.values[1938] != 0.0)) {
            scratch.values[1361] = 0.0;
            scratch.node_derivatives[1361] = [0.0; Instance::NODE_COUNT];
            scratch.branch_derivatives[1361] = [0.0; Instance::BRANCH_COUNT];
        }

        if (((((scratch.values[1320] != 0.0) && (scratch.values[1337] != 0.0)) && (!(scratch.values[1935] != 0.0))) && (!(scratch.values[1937] != 0.0))) && (!(scratch.values[1938] != 0.0))) {
            scratch.store_ad(1361, &AdValue::mul(AdValue::add(AdValue::div(AdValue::mul(AdValue::square(scratch.ad_value(1360)), AdValue::ln(scratch.ad_value(1360))), AdValue::sub_from_scalar(1.0, scratch.ad_value(1360))), scratch.ad_value(1360)), AdValue::sub_from_scalar(1.0, AdValue::scale(scratch.ad_value(549), 2.0))));
        }

        if ((((scratch.values[1320] != 0.0) && (scratch.values[1337] != 0.0)) && (!(scratch.values[1935] != 0.0))) && (!(scratch.values[1937] != 0.0))) {
            scratch.store_ad(1362, &AdValue::add(scratch.ad_value(1360), scratch.ad_value(1361)));
        }

        scratch.values[1939] = if (scratch.values[549] == 0.5) { 1.0 } else { 0.0 };

        if (((((scratch.values[1320] != 0.0) && (scratch.values[1337] != 0.0)) && (!(scratch.values[1935] != 0.0))) && (!(scratch.values[1937] != 0.0))) && (scratch.values[1939] != 0.0)) {
            scratch.store_ad(1356, &AdValue::sqrt(AdValue::mul(scratch.ad_value(1359), scratch.ad_value(628))));
        }

        if (((((scratch.values[1320] != 0.0) && (scratch.values[1337] != 0.0)) && (!(scratch.values[1935] != 0.0))) && (!(scratch.values[1937] != 0.0))) && (!(scratch.values[1939] != 0.0))) {
            scratch.store_ad(1356, &AdValue::pow(AdValue::mul(scratch.ad_value(1359), scratch.ad_value(628)), scratch.ad_value(549)));
        }

        if ((((scratch.values[1320] != 0.0) && (scratch.values[1337] != 0.0)) && (!(scratch.values[1935] != 0.0))) && (!(scratch.values[1937] != 0.0))) {
            scratch.store_ad(1363, &AdValue::mul(scratch.ad_value(622), scratch.ad_value(1356)));
        }

        if ((((scratch.values[1320] != 0.0) && (scratch.values[1337] != 0.0)) && (!(scratch.values[1935] != 0.0))) && (!(scratch.values[1937] != 0.0))) {
            scratch.store_ad(1364, &AdValue::mul(scratch.ad_value(592), AdValue::mul(AdValue::offset(scratch.ad_value(1350), (-1.0)), scratch.ad_value(1363))));
        }

        if ((((scratch.values[1320] != 0.0) && (scratch.values[1337] != 0.0)) && (!(scratch.values[1935] != 0.0))) && (!(scratch.values[1937] != 0.0))) {
            scratch.store_ad(1358, &AdValue::mul(scratch.ad_value(560), AdValue::mul(scratch.ad_value(1364), scratch.ad_value(1362))));
        }

        scratch.values[1940] = if (scratch.values[563] == 0.0) { 1.0 } else { 0.0 };

        if ((((scratch.values[1320] != 0.0) && (scratch.values[1337] != 0.0)) && (!(scratch.values[1935] != 0.0))) && (scratch.values[1940] != 0.0)) {
            scratch.values[1365] = 0.0;
            scratch.node_derivatives[1365] = [0.0; Instance::NODE_COUNT];
            scratch.branch_derivatives[1365] = [0.0; Instance::BRANCH_COUNT];
        }

        if ((((scratch.values[1320] != 0.0) && (scratch.values[1337] != 0.0)) && (!(scratch.values[1935] != 0.0))) && (!(scratch.values[1940] != 0.0))) {
            scratch.store_ad(1366, &AdValue::mul(scratch.ad_value(637), AdValue::div(AdValue::mul(scratch.ad_value(1363), scratch.ad_value(607)), scratch.ad_value(1359))));
        }

        if ((((scratch.values[1320] != 0.0) && (scratch.values[1337] != 0.0)) && (!(scratch.values[1935] != 0.0))) && (!(scratch.values[1940] != 0.0))) {
            scratch.store_ad(1367, &AdValue::div(AdValue::scale(scratch.ad_value(634), 0.666666666666667), scratch.ad_value(1366)));
        }

        if ((((scratch.values[1320] != 0.0) && (scratch.values[1337] != 0.0)) && (!(scratch.values[1935] != 0.0))) && (!(scratch.values[1940] != 0.0))) {
            scratch.store_ad(1368, &AdValue::square(scratch.ad_value(1367)));
        }

        if ((((scratch.values[1320] != 0.0) && (scratch.values[1337] != 0.0)) && (!(scratch.values[1935] != 0.0))) && (!(scratch.values[1940] != 0.0))) {
            scratch.store_ad(1369, &AdValue::sqrt(AdValue::div(AdValue::square(scratch.ad_value(1368)), AdValue::offset(AdValue::square(scratch.ad_value(1368)), 1.0))));
        }

        if ((((scratch.values[1320] != 0.0) && (scratch.values[1337] != 0.0)) && (!(scratch.values[1935] != 0.0))) && (!(scratch.values[1940] != 0.0))) {
            scratch.store_ad(1370, &AdValue::sqrt(AdValue::abs(scratch.ad_value(1369))));
        }

        if ((((scratch.values[1320] != 0.0) && (scratch.values[1337] != 0.0)) && (!(scratch.values[1935] != 0.0))) && (!(scratch.values[1940] != 0.0))) {
            scratch.store_ad(1371, &AdValue::mul(scratch.ad_value(1369), scratch.ad_value(1370)));
        }

        scratch.values[1941] = if (((-scratch.values[549]) * scratch.values[610]) == (-1.0)) { 1.0 } else { 0.0 };

        if (((((scratch.values[1320] != 0.0) && (scratch.values[1337] != 0.0)) && (!(scratch.values[1935] != 0.0))) && (!(scratch.values[1940] != 0.0))) && (scratch.values[1941] != 0.0)) {
            scratch.store_ad(1372, &AdValue::div_from_scalar(1.0, AdValue::offset(AdValue::mul(scratch.ad_value(1366), scratch.ad_value(1371)), 1.0)));
        }

        if (((((scratch.values[1320] != 0.0) && (scratch.values[1337] != 0.0)) && (!(scratch.values[1935] != 0.0))) && (!(scratch.values[1940] != 0.0))) && (!(scratch.values[1941] != 0.0))) {
            scratch.store_ad(1372, &AdValue::pow(AdValue::offset(AdValue::mul(scratch.ad_value(1366), scratch.ad_value(1371)), 1.0), AdValue::mul(AdValue::neg(scratch.ad_value(549)), scratch.ad_value(610))));
        }

        if ((((scratch.values[1320] != 0.0) && (scratch.values[1337] != 0.0)) && (!(scratch.values[1935] != 0.0))) && (!(scratch.values[1940] != 0.0))) {
            scratch.store_ad(1373, &AdValue::div(AdValue::mul(scratch.ad_value(1362), scratch.ad_value(1372)), AdValue::add(scratch.ad_value(1362), scratch.ad_value(1372))));
        }

        if ((((scratch.values[1320] != 0.0) && (scratch.values[1337] != 0.0)) && (!(scratch.values[1935] != 0.0))) && (!(scratch.values[1940] != 0.0))) {
            scratch.store_ad(1374, &AdValue::sqrt(AdValue::scale(AdValue::div(scratch.ad_value(1366), scratch.ad_value(1370)), 0.375)));
        }

        if ((((scratch.values[1320] != 0.0) && (scratch.values[1337] != 0.0)) && (!(scratch.values[1935] != 0.0))) && (!(scratch.values[1940] != 0.0))) {
            scratch.store_ad(1375, &AdValue::sub(AdValue::scale(AdValue::mul(scratch.ad_value(1367), scratch.ad_value(1370)), 2.0), scratch.ad_value(1369)));
        }

        if ((((scratch.values[1320] != 0.0) && (scratch.values[1337] != 0.0)) && (!(scratch.values[1935] != 0.0))) && (!(scratch.values[1940] != 0.0))) {
            scratch.store_ad(1376, &AdValue::add(AdValue::sub(AdValue::mul(AdValue::mul(scratch.ad_value(634), scratch.ad_value(1367)), scratch.ad_value(1370)), AdValue::mul(scratch.ad_value(634), scratch.ad_value(1369))), AdValue::scale(AdValue::mul(scratch.ad_value(1366), scratch.ad_value(1371)), 0.5)));
        }

        if ((((scratch.values[1320] != 0.0) && (scratch.values[1337] != 0.0)) && (!(scratch.values[1935] != 0.0))) && (!(scratch.values[1940] != 0.0))) {
            scratch.store_ad(1377, &AdValue::mul(AdValue::offset(scratch.ad_value(1375), (-1.0)), scratch.ad_value(1374)));
        }

        if ((((scratch.values[1320] != 0.0) && (scratch.values[1337] != 0.0)) && (!(scratch.values[1935] != 0.0))) && (!(scratch.values[1940] != 0.0))) {
            scratch.store_ad(1338, &AdValue::square(scratch.ad_value(1377)));
        }

        scratch.values[1942] = if (scratch.values[1377] > 0.0) { 1.0 } else { 0.0 };

        if (((((scratch.values[1320] != 0.0) && (scratch.values[1337] != 0.0)) && (!(scratch.values[1935] != 0.0))) && (!(scratch.values[1940] != 0.0))) && (scratch.values[1942] != 0.0)) {
            scratch.store_ad(1339, &AdValue::div_from_scalar(1.0, AdValue::offset(AdValue::scale(scratch.ad_value(1377), scratch.values[428]), 1.0)));
        }

        if (((((scratch.values[1320] != 0.0) && (scratch.values[1337] != 0.0)) && (!(scratch.values[1935] != 0.0))) && (!(scratch.values[1940] != 0.0))) && (!(scratch.values[1942] != 0.0))) {
            scratch.store_ad(1339, &AdValue::div_from_scalar(1.0, AdValue::sub_from_scalar(1.0, AdValue::scale(scratch.ad_value(1377), scratch.values[428]))));
        }

        scratch.values[1943] = if (((-scratch.values[1338]) + scratch.values[1376]) > (-230.25850929940458)) { 1.0 } else { 0.0 };

        if (((((scratch.values[1320] != 0.0) && (scratch.values[1337] != 0.0)) && (!(scratch.values[1935] != 0.0))) && (!(scratch.values[1940] != 0.0))) && (scratch.values[1943] != 0.0)) {
            scratch.store_ad(1356, &AdValue::exp(AdValue::sub(scratch.ad_value(1376), scratch.ad_value(1338))));
        }

        if (((((scratch.values[1320] != 0.0) && (scratch.values[1337] != 0.0)) && (!(scratch.values[1935] != 0.0))) && (!(scratch.values[1940] != 0.0))) && (!(scratch.values[1943] != 0.0))) {
            let assign36970_ad_e48461: AdValue = AdValue::div_from_scalar(1e-100, AdValue::offset(AdValue::mul(AdValue::sub_from_scalar((-230.25850929940458), AdValue::sub(scratch.ad_value(1376), scratch.ad_value(1338))), AdValue::offset(AdValue::scale(AdValue::mul(AdValue::sub_from_scalar((-230.25850929940458), AdValue::sub(scratch.ad_value(1376), scratch.ad_value(1338))), AdValue::offset(AdValue::scale(AdValue::sub_from_scalar((-230.25850929940458), AdValue::sub(scratch.ad_value(1376), scratch.ad_value(1338))), 0.3333333333333333), 1.0)), 0.5), 1.0)), 1.0));
            scratch.store_ad(1356, &assign36970_ad_e48461);
        }

        if ((((scratch.values[1320] != 0.0) && (scratch.values[1337] != 0.0)) && (!(scratch.values[1935] != 0.0))) && (!(scratch.values[1940] != 0.0))) {
            scratch.store_ad(1340, &AdValue::mul(AdValue::add(AdValue::add(AdValue::scale(scratch.ad_value(1339), 0.29214664), AdValue::scale(AdValue::square(scratch.ad_value(1339)), scratch.values[429])), AdValue::scale(AdValue::mul(AdValue::square(scratch.ad_value(1339)), scratch.ad_value(1339)), scratch.values[430])), scratch.ad_value(1356)));
        }

    }
}
