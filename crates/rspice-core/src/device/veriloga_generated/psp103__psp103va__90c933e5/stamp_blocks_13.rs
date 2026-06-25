#![allow(dead_code, unused_assignments, unused_imports, unused_parens, unused_variables)]

use super::{AdValue, GeneratedEvalContext, GeneratedReactiveStamper, GeneratedStamper, ReactiveScratch, THERMAL_VOLTAGE_PER_K};
use super::super::state::Instance;

impl Instance {

    pub(super) fn stamp_reactive_block_8(
        &mut self,
        ctx: &GeneratedEvalContext<'_>,
        stamper: &mut GeneratedReactiveStamper<'_>,
        scratch: &mut ReactiveScratch,
    ) {
        let _ = stamper;
        if ((1.0 + (scratch.values[262] * scratch.values[355])) > 0.0) {
            scratch.store_ad(835, &AdValue::offset(AdValue::scale(scratch.ad_value(262), scratch.values[355]), 1.0));
        } else {
            scratch.values[835] = 0.0;
            scratch.node_derivatives[835] = [0.0; Instance::NODE_COUNT];
            scratch.branch_derivatives[835] = [0.0; Instance::BRANCH_COUNT];
        }

        scratch.store_ad(754, &AdValue::mul(scratch.ad_value(260), scratch.ad_value(835)));

        scratch.store_ad(843, &AdValue::scale(AdValue::mul(scratch.ad_value(754), scratch.ad_value(209)), 500000000.0));

        if ((1.0 + (scratch.values[263] * scratch.values[355])) > 0.0) {
            scratch.store_ad(835, &AdValue::offset(AdValue::scale(scratch.ad_value(263), scratch.values[355]), 1.0));
        } else {
            scratch.values[835] = 0.0;
            scratch.node_derivatives[835] = [0.0; Instance::NODE_COUNT];
            scratch.branch_derivatives[835] = [0.0; Instance::BRANCH_COUNT];
        }

        scratch.store_ad(755, &AdValue::mul(scratch.ad_value(261), scratch.ad_value(835)));

        scratch.store_ad(844, &AdValue::scale(AdValue::mul(scratch.ad_value(755), scratch.ad_value(210)), 500000000.0));

        scratch.values[30] = 0.0;

        scratch.values[31] = 0.0;

        scratch.values[32] = 0.0;

        scratch.values[33] = 0.0;

        scratch.values[34] = 0.0;

        scratch.values[35] = 0.0;

        scratch.values[36] = 0.0;

        scratch.values[37] = scratch.values[308];
        scratch.node_derivatives[37] = scratch.node_derivatives[308];
        scratch.branch_derivatives[37] = scratch.branch_derivatives[308];

        scratch.values[1319] = if (scratch.values[1] == 0.0) { 1.0 } else { 0.0 };

        if (scratch.values[1319] != 0.0) {
            scratch.values[37] = (if (scratch.values[20] > 0.0) { scratch.values[20] } else { 0.0 });
            scratch.node_derivatives[37] = [0.0; Instance::NODE_COUNT];
            scratch.branch_derivatives[37] = [0.0; Instance::BRANCH_COUNT];
        }

        scratch.values[1320] = if (scratch.values[5] == 3.0) { 1.0 } else { 0.0 };

        if (scratch.values[1320] != 0.0) {
            scratch.values[36] = 1.0;
            scratch.node_derivatives[36] = [0.0; Instance::NODE_COUNT];
            scratch.branch_derivatives[36] = [0.0; Instance::BRANCH_COUNT];
        }

        scratch.store_ad(30, &AdValue::scale(scratch.ad_value(12), scratch.values[690]));

        scratch.store_ad(31, &AdValue::scale(scratch.ad_value(12), scratch.values[691]));

        scratch.store_ad(32, &AdValue::scale(scratch.ad_value(12), scratch.values[692]));

        scratch.store_ad(33, &AdValue::scale(scratch.ad_value(12), scratch.values[717]));

        scratch.store_ad(34, &AdValue::scale(scratch.ad_value(12), scratch.values[718]));

        scratch.store_ad(35, &AdValue::scale(scratch.ad_value(12), scratch.values[719]));

        scratch.values[1321] = if ((scratch.values[5] == 2.0) || (scratch.values[5] == 3.0)) { 1.0 } else { 0.0 };

        if (scratch.values[1321] != 0.0) {
            scratch.store_ad(30, &AdValue::scale(scratch.ad_value(12), scratch.values[693]));
        }

        if (scratch.values[1321] != 0.0) {
            scratch.store_ad(31, &AdValue::sub(AdValue::scale(scratch.ad_value(12), scratch.values[694]), AdValue::mul(scratch.ad_value(36), scratch.ad_value(37))));
        }

        if (scratch.values[1321] != 0.0) {
            scratch.values[32] = scratch.values[37];
            scratch.node_derivatives[32] = scratch.node_derivatives[37];
            scratch.branch_derivatives[32] = scratch.branch_derivatives[37];
        }

        if (scratch.values[1321] != 0.0) {
            scratch.store_ad(33, &AdValue::scale(scratch.ad_value(12), scratch.values[720]));
        }

        if (scratch.values[1321] != 0.0) {
            scratch.store_ad(34, &AdValue::sub(AdValue::scale(scratch.ad_value(12), scratch.values[721]), AdValue::mul(scratch.ad_value(36), scratch.ad_value(37))));
        }

        if (scratch.values[1321] != 0.0) {
            scratch.values[35] = scratch.values[37];
            scratch.node_derivatives[35] = scratch.node_derivatives[37];
            scratch.branch_derivatives[35] = scratch.branch_derivatives[37];
        }

        scratch.values[1322] = if (((scratch.values[5] == 1.0) || (scratch.values[5] == 2.0)) || (scratch.values[5] == 3.0)) { 1.0 } else { 0.0 };

        if (scratch.values[1322] != 0.0) {
            scratch.store_ad(690, &{
                if (scratch.values[30] > 0.0) {
                    scratch.ad_value(30)
                } else {
                    AdValue::constant(0.0)
                }
            });
        }

        if (scratch.values[1322] != 0.0) {
            scratch.store_ad(691, &{
                if (scratch.values[31] > 0.0) {
                    scratch.ad_value(31)
                } else {
                    AdValue::constant(0.0)
                }
            });
        }

        if (scratch.values[1322] != 0.0) {
            scratch.store_ad(692, &{
                if (scratch.values[32] > 0.0) {
                    scratch.ad_value(32)
                } else {
                    AdValue::constant(0.0)
                }
            });
        }

        if (scratch.values[1322] != 0.0) {
            scratch.store_ad(717, &{
                if (scratch.values[33] > 0.0) {
                    scratch.ad_value(33)
                } else {
                    AdValue::constant(0.0)
                }
            });
        }

        if (scratch.values[1322] != 0.0) {
            scratch.store_ad(718, &{
                if (scratch.values[34] > 0.0) {
                    scratch.ad_value(34)
                } else {
                    AdValue::constant(0.0)
                }
            });
        }

        if (scratch.values[1322] != 0.0) {
            scratch.store_ad(719, &{
                if (scratch.values[35] > 0.0) {
                    scratch.ad_value(35)
                } else {
                    AdValue::constant(0.0)
                }
            });
        }

        if (!(scratch.values[1322] != 0.0)) {
            scratch.values[690] = 0.0;
            scratch.node_derivatives[690] = [0.0; Instance::NODE_COUNT];
            scratch.branch_derivatives[690] = [0.0; Instance::BRANCH_COUNT];
        }

        if (!(scratch.values[1322] != 0.0)) {
            scratch.values[691] = 0.0;
            scratch.node_derivatives[691] = [0.0; Instance::NODE_COUNT];
            scratch.branch_derivatives[691] = [0.0; Instance::BRANCH_COUNT];
        }

        if (!(scratch.values[1322] != 0.0)) {
            scratch.values[692] = 0.0;
            scratch.node_derivatives[692] = [0.0; Instance::NODE_COUNT];
            scratch.branch_derivatives[692] = [0.0; Instance::BRANCH_COUNT];
        }

        if (!(scratch.values[1322] != 0.0)) {
            scratch.values[717] = 0.0;
            scratch.node_derivatives[717] = [0.0; Instance::NODE_COUNT];
            scratch.branch_derivatives[717] = [0.0; Instance::BRANCH_COUNT];
        }

        if (!(scratch.values[1322] != 0.0)) {
            scratch.values[718] = 0.0;
            scratch.node_derivatives[718] = [0.0; Instance::NODE_COUNT];
            scratch.branch_derivatives[718] = [0.0; Instance::BRANCH_COUNT];
        }

        if (!(scratch.values[1322] != 0.0)) {
            scratch.values[719] = 0.0;
            scratch.node_derivatives[719] = [0.0; Instance::NODE_COUNT];
            scratch.branch_derivatives[719] = [0.0; Instance::BRANCH_COUNT];
        }

        scratch.values[700] = 0.0;

        scratch.values[727] = 0.0;

        scratch.values[702] = 0.0;

        scratch.values[729] = 0.0;

        scratch.values[701] = 0.0;

        scratch.values[728] = 0.0;

        scratch.values[703] = 0.0;

        scratch.values[730] = 0.0;

        scratch.values[698] = 0.0;

        scratch.values[725] = 0.0;

        scratch.values[699] = 0.0;

        scratch.values[726] = 0.0;

        scratch.values[695] = 1.0;

        scratch.values[722] = 1.0;

        scratch.values[696] = 1.0;

        scratch.values[723] = 1.0;

        scratch.values[697] = 1.0;

        scratch.values[724] = 1.0;

        scratch.values[532] = 0.0;

        scratch.values[1323] = if (scratch.values[5] > 0.0) { 1.0 } else { 0.0 };

        scratch.values[1324] = if ((scratch.values[436] * scratch.values[690]) > 0.0) { 1.0 } else { 0.0 };

        if ((scratch.values[1323] != 0.0) && (scratch.values[1324] != 0.0)) {
            scratch.store_ad(503, &AdValue::scale(AdValue::ln(AdValue::offset(AdValue::div_from_scalar(scratch.values[367], AdValue::scale(scratch.ad_value(690), scratch.values[436])), 1.0)), scratch.values[419]));
        }

        if ((scratch.values[1323] != 0.0) && (!(scratch.values[1324] != 0.0))) {
            scratch.values[503] = 100000000.0;
            scratch.node_derivatives[503] = [0.0; Instance::NODE_COUNT];
            scratch.branch_derivatives[503] = [0.0; Instance::BRANCH_COUNT];
        }

        scratch.values[1325] = if ((scratch.values[437] * scratch.values[691]) > 0.0) { 1.0 } else { 0.0 };

        if ((scratch.values[1323] != 0.0) && (scratch.values[1325] != 0.0)) {
            scratch.store_ad(504, &AdValue::scale(AdValue::ln(AdValue::offset(AdValue::div_from_scalar(scratch.values[367], AdValue::scale(scratch.ad_value(691), scratch.values[437])), 1.0)), scratch.values[419]));
        }

        if ((scratch.values[1323] != 0.0) && (!(scratch.values[1325] != 0.0))) {
            scratch.values[504] = 100000000.0;
            scratch.node_derivatives[504] = [0.0; Instance::NODE_COUNT];
            scratch.branch_derivatives[504] = [0.0; Instance::BRANCH_COUNT];
        }

        scratch.values[1326] = if ((scratch.values[438] * scratch.values[692]) > 0.0) { 1.0 } else { 0.0 };

        if ((scratch.values[1323] != 0.0) && (scratch.values[1326] != 0.0)) {
            scratch.store_ad(505, &AdValue::scale(AdValue::ln(AdValue::offset(AdValue::div_from_scalar(scratch.values[367], AdValue::scale(scratch.ad_value(692), scratch.values[438])), 1.0)), scratch.values[419]));
        }

        if ((scratch.values[1323] != 0.0) && (!(scratch.values[1326] != 0.0))) {
            scratch.values[505] = 100000000.0;
            scratch.node_derivatives[505] = [0.0; Instance::NODE_COUNT];
            scratch.branch_derivatives[505] = [0.0; Instance::BRANCH_COUNT];
        }

        if (scratch.values[1323] != 0.0) {
            scratch.store_ad(698, &AdValue::min(AdValue::min(scratch.ad_value(503), scratch.ad_value(504)), scratch.ad_value(505)));
        }

        scratch.values[1327] = if ((((scratch.values[698] * scratch.values[420])) as f64).abs() < 230.25850929940458) { 1.0 } else { 0.0 };

        if ((scratch.values[1323] != 0.0) && (scratch.values[1327] != 0.0)) {
            scratch.store_ad(699, &AdValue::exp(AdValue::scale(scratch.ad_value(698), scratch.values[420])));
        }

        scratch.values[1328] = if ((scratch.values[698] * scratch.values[420]) < 0.0) { 1.0 } else { 0.0 };

        if (((scratch.values[1323] != 0.0) && (!(scratch.values[1327] != 0.0))) && (scratch.values[1328] != 0.0)) {
            scratch.store_ad(699, &AdValue::div_from_scalar(1e-100, AdValue::offset(AdValue::mul(AdValue::sub_from_scalar((-230.25850929940458), AdValue::scale(scratch.ad_value(698), scratch.values[420])), AdValue::offset(AdValue::scale(AdValue::mul(AdValue::sub_from_scalar((-230.25850929940458), AdValue::scale(scratch.ad_value(698), scratch.values[420])), AdValue::offset(AdValue::scale(AdValue::sub_from_scalar((-230.25850929940458), AdValue::scale(scratch.ad_value(698), scratch.values[420])), 0.3333333333333333), 1.0)), 0.5), 1.0)), 1.0)));
        }

        if (((scratch.values[1323] != 0.0) && (!(scratch.values[1327] != 0.0))) && (!(scratch.values[1328] != 0.0))) {
            scratch.store_ad(699, &AdValue::scale(AdValue::offset(AdValue::mul(AdValue::offset(AdValue::scale(scratch.ad_value(698), scratch.values[420]), (-230.25850929940458)), AdValue::offset(AdValue::scale(AdValue::mul(AdValue::offset(AdValue::scale(scratch.ad_value(698), scratch.values[420]), (-230.25850929940458)), AdValue::offset(AdValue::scale(AdValue::offset(AdValue::scale(scratch.ad_value(698), scratch.values[420]), (-230.25850929940458)), 0.3333333333333333), 1.0)), 0.5), 1.0)), 1.0), 1e100));
        }

        if (scratch.values[1323] != 0.0) {
            scratch.values[445] = scratch.values[442];
            scratch.node_derivatives[445] = [0.0; Instance::NODE_COUNT];
            scratch.branch_derivatives[445] = [0.0; Instance::BRANCH_COUNT];
        }

        if (scratch.values[1323] != 0.0) {
            scratch.values[446] = scratch.values[443];
            scratch.node_derivatives[446] = [0.0; Instance::NODE_COUNT];
            scratch.branch_derivatives[446] = [0.0; Instance::BRANCH_COUNT];
        }

        if (scratch.values[1323] != 0.0) {
            scratch.values[447] = scratch.values[444];
            scratch.node_derivatives[447] = [0.0; Instance::NODE_COUNT];
            scratch.branch_derivatives[447] = [0.0; Instance::BRANCH_COUNT];
        }

        if (scratch.values[1323] != 0.0) {
            scratch.values[448] = scratch.values[375];
            scratch.node_derivatives[448] = [0.0; Instance::NODE_COUNT];
            scratch.branch_derivatives[448] = [0.0; Instance::BRANCH_COUNT];
        }

        if (scratch.values[1323] != 0.0) {
            scratch.values[449] = scratch.values[376];
            scratch.node_derivatives[449] = [0.0; Instance::NODE_COUNT];
            scratch.branch_derivatives[449] = [0.0; Instance::BRANCH_COUNT];
        }

        if (scratch.values[1323] != 0.0) {
            scratch.values[450] = scratch.values[377];
            scratch.node_derivatives[450] = [0.0; Instance::NODE_COUNT];
            scratch.branch_derivatives[450] = [0.0; Instance::BRANCH_COUNT];
        }

        if (scratch.values[1323] != 0.0) {
            scratch.values[451] = scratch.values[372];
            scratch.node_derivatives[451] = [0.0; Instance::NODE_COUNT];
            scratch.branch_derivatives[451] = [0.0; Instance::BRANCH_COUNT];
        }

        if (scratch.values[1323] != 0.0) {
            scratch.values[452] = scratch.values[373];
            scratch.node_derivatives[452] = [0.0; Instance::NODE_COUNT];
            scratch.branch_derivatives[452] = [0.0; Instance::BRANCH_COUNT];
        }

        if (scratch.values[1323] != 0.0) {
            scratch.values[453] = scratch.values[374];
            scratch.node_derivatives[453] = [0.0; Instance::NODE_COUNT];
            scratch.branch_derivatives[453] = [0.0; Instance::BRANCH_COUNT];
        }

        scratch.values[1329] = if (scratch.values[690] == 0.0) { 1.0 } else { 0.0 };

        if ((scratch.values[1323] != 0.0) && (scratch.values[1329] != 0.0)) {
            scratch.values[445] = (scratch.values[443] + scratch.values[444]);
            scratch.node_derivatives[445] = [0.0; Instance::NODE_COUNT];
            scratch.branch_derivatives[445] = [0.0; Instance::BRANCH_COUNT];
        }

        if ((scratch.values[1323] != 0.0) && (scratch.values[1329] != 0.0)) {
            scratch.values[448] = (0.9 * (scratch.values[376]).min(scratch.values[377]));
            scratch.node_derivatives[448] = [0.0; Instance::NODE_COUNT];
            scratch.branch_derivatives[448] = [0.0; Instance::BRANCH_COUNT];
        }

        if ((scratch.values[1323] != 0.0) && (scratch.values[1329] != 0.0)) {
            scratch.values[451] = (scratch.values[373] + scratch.values[374]);
            scratch.node_derivatives[451] = [0.0; Instance::NODE_COUNT];
            scratch.branch_derivatives[451] = [0.0; Instance::BRANCH_COUNT];
        }

        scratch.values[1330] = if (scratch.values[691] == 0.0) { 1.0 } else { 0.0 };

        if ((scratch.values[1323] != 0.0) && (scratch.values[1330] != 0.0)) {
            scratch.values[446] = (scratch.values[442] + scratch.values[444]);
            scratch.node_derivatives[446] = [0.0; Instance::NODE_COUNT];
            scratch.branch_derivatives[446] = [0.0; Instance::BRANCH_COUNT];
        }

        if ((scratch.values[1323] != 0.0) && (scratch.values[1330] != 0.0)) {
            scratch.values[449] = (0.9 * (scratch.values[375]).min(scratch.values[377]));
            scratch.node_derivatives[449] = [0.0; Instance::NODE_COUNT];
            scratch.branch_derivatives[449] = [0.0; Instance::BRANCH_COUNT];
        }

        if ((scratch.values[1323] != 0.0) && (scratch.values[1330] != 0.0)) {
            scratch.values[452] = (scratch.values[372] + scratch.values[374]);
            scratch.node_derivatives[452] = [0.0; Instance::NODE_COUNT];
            scratch.branch_derivatives[452] = [0.0; Instance::BRANCH_COUNT];
        }

        scratch.values[1331] = if (scratch.values[692] == 0.0) { 1.0 } else { 0.0 };

        if ((scratch.values[1323] != 0.0) && (scratch.values[1331] != 0.0)) {
            scratch.values[447] = (scratch.values[442] + scratch.values[443]);
            scratch.node_derivatives[447] = [0.0; Instance::NODE_COUNT];
            scratch.branch_derivatives[447] = [0.0; Instance::BRANCH_COUNT];
        }

        if ((scratch.values[1323] != 0.0) && (scratch.values[1331] != 0.0)) {
            scratch.values[450] = (0.9 * (scratch.values[375]).min(scratch.values[376]));
            scratch.node_derivatives[450] = [0.0; Instance::NODE_COUNT];
            scratch.branch_derivatives[450] = [0.0; Instance::BRANCH_COUNT];
        }

        if ((scratch.values[1323] != 0.0) && (scratch.values[1331] != 0.0)) {
            scratch.values[453] = (scratch.values[372] + scratch.values[373]);
            scratch.node_derivatives[453] = [0.0; Instance::NODE_COUNT];
            scratch.branch_derivatives[453] = [0.0; Instance::BRANCH_COUNT];
        }

        if (scratch.values[1323] != 0.0) {
            scratch.store_ad(700, &AdValue::min(AdValue::min(scratch.ad_value(445), scratch.ad_value(446)), scratch.ad_value(447)));
        }

        if (scratch.values[1323] != 0.0) {
            scratch.store_ad(701, &AdValue::scale(scratch.ad_value(700), 0.1));
        }

        if (scratch.values[1323] != 0.0) {
            scratch.store_ad(426, &AdValue::max(AdValue::max(scratch.ad_value(448), scratch.ad_value(449)), scratch.ad_value(450)));
        }

        if (scratch.values[1323] != 0.0) {
            scratch.store_ad(702, &AdValue::mul(scratch.ad_value(700), AdValue::sub_from_scalar(1.0, AdValue::pow_from_scalar(2.0, AdValue::div_from_scalar((-1.0), scratch.ad_value(426))))));
        }

        if (scratch.values[1323] != 0.0) {
            scratch.store_ad(703, &AdValue::offset(AdValue::min(AdValue::min(scratch.ad_value(451), scratch.ad_value(452)), scratch.ad_value(453)), (-0.05)));
        }

        scratch.values[1332] = if ((scratch.values[588] * scratch.values[717]) > 0.0) { 1.0 } else { 0.0 };

        if ((scratch.values[1323] != 0.0) && (scratch.values[1332] != 0.0)) {
            scratch.store_ad(503, &AdValue::scale(AdValue::ln(AdValue::offset(AdValue::div_from_scalar(scratch.values[367], AdValue::mul(scratch.ad_value(588), scratch.ad_value(717))), 1.0)), scratch.values[419]));
        }

        if ((scratch.values[1323] != 0.0) && (!(scratch.values[1332] != 0.0))) {
            scratch.values[503] = 100000000.0;
            scratch.node_derivatives[503] = [0.0; Instance::NODE_COUNT];
            scratch.branch_derivatives[503] = [0.0; Instance::BRANCH_COUNT];
        }

        scratch.values[1333] = if ((scratch.values[589] * scratch.values[718]) > 0.0) { 1.0 } else { 0.0 };

        if ((scratch.values[1323] != 0.0) && (scratch.values[1333] != 0.0)) {
            scratch.store_ad(504, &AdValue::scale(AdValue::ln(AdValue::offset(AdValue::div_from_scalar(scratch.values[367], AdValue::mul(scratch.ad_value(589), scratch.ad_value(718))), 1.0)), scratch.values[419]));
        }

        if ((scratch.values[1323] != 0.0) && (!(scratch.values[1333] != 0.0))) {
            scratch.values[504] = 100000000.0;
            scratch.node_derivatives[504] = [0.0; Instance::NODE_COUNT];
            scratch.branch_derivatives[504] = [0.0; Instance::BRANCH_COUNT];
        }

        scratch.values[1334] = if ((scratch.values[590] * scratch.values[719]) > 0.0) { 1.0 } else { 0.0 };

        if ((scratch.values[1323] != 0.0) && (scratch.values[1334] != 0.0)) {
            scratch.store_ad(505, &AdValue::scale(AdValue::ln(AdValue::offset(AdValue::div_from_scalar(scratch.values[367], AdValue::mul(scratch.ad_value(590), scratch.ad_value(719))), 1.0)), scratch.values[419]));
        }

        if ((scratch.values[1323] != 0.0) && (!(scratch.values[1334] != 0.0))) {
            scratch.values[505] = 100000000.0;
            scratch.node_derivatives[505] = [0.0; Instance::NODE_COUNT];
            scratch.branch_derivatives[505] = [0.0; Instance::BRANCH_COUNT];
        }

        if (scratch.values[1323] != 0.0) {
            scratch.store_ad(725, &AdValue::min(AdValue::min(scratch.ad_value(503), scratch.ad_value(504)), scratch.ad_value(505)));
        }

        scratch.values[1335] = if ((((scratch.values[725] * scratch.values[420])) as f64).abs() < 230.25850929940458) { 1.0 } else { 0.0 };

        if ((scratch.values[1323] != 0.0) && (scratch.values[1335] != 0.0)) {
            scratch.store_ad(726, &AdValue::exp(AdValue::scale(scratch.ad_value(725), scratch.values[420])));
        }

        scratch.values[1336] = if ((scratch.values[725] * scratch.values[420]) < 0.0) { 1.0 } else { 0.0 };

        if (((scratch.values[1323] != 0.0) && (!(scratch.values[1335] != 0.0))) && (scratch.values[1336] != 0.0)) {
            scratch.store_ad(726, &AdValue::div_from_scalar(1e-100, AdValue::offset(AdValue::mul(AdValue::sub_from_scalar((-230.25850929940458), AdValue::scale(scratch.ad_value(725), scratch.values[420])), AdValue::offset(AdValue::scale(AdValue::mul(AdValue::sub_from_scalar((-230.25850929940458), AdValue::scale(scratch.ad_value(725), scratch.values[420])), AdValue::offset(AdValue::scale(AdValue::sub_from_scalar((-230.25850929940458), AdValue::scale(scratch.ad_value(725), scratch.values[420])), 0.3333333333333333), 1.0)), 0.5), 1.0)), 1.0)));
        }

        if (((scratch.values[1323] != 0.0) && (!(scratch.values[1335] != 0.0))) && (!(scratch.values[1336] != 0.0))) {
            scratch.store_ad(726, &AdValue::scale(AdValue::offset(AdValue::mul(AdValue::offset(AdValue::scale(scratch.ad_value(725), scratch.values[420]), (-230.25850929940458)), AdValue::offset(AdValue::scale(AdValue::mul(AdValue::offset(AdValue::scale(scratch.ad_value(725), scratch.values[420]), (-230.25850929940458)), AdValue::offset(AdValue::scale(AdValue::offset(AdValue::scale(scratch.ad_value(725), scratch.values[420]), (-230.25850929940458)), 0.3333333333333333), 1.0)), 0.5), 1.0)), 1.0), 1e100));
        }

        if (scratch.values[1323] != 0.0) {
            scratch.values[445] = scratch.values[594];
            scratch.node_derivatives[445] = scratch.node_derivatives[594];
            scratch.branch_derivatives[445] = scratch.branch_derivatives[594];
        }

        if (scratch.values[1323] != 0.0) {
            scratch.values[446] = scratch.values[595];
            scratch.node_derivatives[446] = scratch.node_derivatives[595];
            scratch.branch_derivatives[446] = scratch.branch_derivatives[595];
        }

        if (scratch.values[1323] != 0.0) {
            scratch.values[447] = scratch.values[596];
            scratch.node_derivatives[447] = scratch.node_derivatives[596];
            scratch.branch_derivatives[447] = scratch.branch_derivatives[596];
        }

        if (scratch.values[1323] != 0.0) {
            scratch.values[448] = scratch.values[542];
            scratch.node_derivatives[448] = scratch.node_derivatives[542];
            scratch.branch_derivatives[448] = scratch.branch_derivatives[542];
        }

    }

    pub(super) fn stamp_reactive_block_9(
        &mut self,
        ctx: &GeneratedEvalContext<'_>,
        stamper: &mut GeneratedReactiveStamper<'_>,
        scratch: &mut ReactiveScratch,
    ) {
        let _ = stamper;
        if (scratch.values[1323] != 0.0) {
            scratch.values[449] = scratch.values[543];
            scratch.node_derivatives[449] = scratch.node_derivatives[543];
            scratch.branch_derivatives[449] = scratch.branch_derivatives[543];
        }

        if (scratch.values[1323] != 0.0) {
            scratch.values[450] = scratch.values[544];
            scratch.node_derivatives[450] = scratch.node_derivatives[544];
            scratch.branch_derivatives[450] = scratch.branch_derivatives[544];
        }

        if (scratch.values[1323] != 0.0) {
            scratch.values[451] = scratch.values[539];
            scratch.node_derivatives[451] = scratch.node_derivatives[539];
            scratch.branch_derivatives[451] = scratch.branch_derivatives[539];
        }

        if (scratch.values[1323] != 0.0) {
            scratch.values[452] = scratch.values[540];
            scratch.node_derivatives[452] = scratch.node_derivatives[540];
            scratch.branch_derivatives[452] = scratch.branch_derivatives[540];
        }

        if (scratch.values[1323] != 0.0) {
            scratch.values[453] = scratch.values[541];
            scratch.node_derivatives[453] = scratch.node_derivatives[541];
            scratch.branch_derivatives[453] = scratch.branch_derivatives[541];
        }

        scratch.values[1337] = if (scratch.values[717] == 0.0) { 1.0 } else { 0.0 };

        if ((scratch.values[1323] != 0.0) && (scratch.values[1337] != 0.0)) {
            scratch.store_ad(445, &AdValue::add(scratch.ad_value(595), scratch.ad_value(596)));
        }

        if ((scratch.values[1323] != 0.0) && (scratch.values[1337] != 0.0)) {
            scratch.store_ad(448, &AdValue::scale(AdValue::min(scratch.ad_value(543), scratch.ad_value(544)), 0.9));
        }

        if ((scratch.values[1323] != 0.0) && (scratch.values[1337] != 0.0)) {
            scratch.store_ad(451, &AdValue::add(scratch.ad_value(540), scratch.ad_value(541)));
        }

        scratch.values[1338] = if (scratch.values[718] == 0.0) { 1.0 } else { 0.0 };

        if ((scratch.values[1323] != 0.0) && (scratch.values[1338] != 0.0)) {
            scratch.store_ad(446, &AdValue::add(scratch.ad_value(594), scratch.ad_value(596)));
        }

        if ((scratch.values[1323] != 0.0) && (scratch.values[1338] != 0.0)) {
            scratch.store_ad(449, &AdValue::scale(AdValue::min(scratch.ad_value(542), scratch.ad_value(544)), 0.9));
        }

        if ((scratch.values[1323] != 0.0) && (scratch.values[1338] != 0.0)) {
            scratch.store_ad(452, &AdValue::add(scratch.ad_value(539), scratch.ad_value(541)));
        }

        scratch.values[1339] = if (scratch.values[719] == 0.0) { 1.0 } else { 0.0 };

        if ((scratch.values[1323] != 0.0) && (scratch.values[1339] != 0.0)) {
            scratch.store_ad(447, &AdValue::add(scratch.ad_value(594), scratch.ad_value(595)));
        }

        if ((scratch.values[1323] != 0.0) && (scratch.values[1339] != 0.0)) {
            scratch.store_ad(450, &AdValue::scale(AdValue::min(scratch.ad_value(542), scratch.ad_value(543)), 0.9));
        }

        if ((scratch.values[1323] != 0.0) && (scratch.values[1339] != 0.0)) {
            scratch.store_ad(453, &AdValue::add(scratch.ad_value(539), scratch.ad_value(540)));
        }

        if (scratch.values[1323] != 0.0) {
            scratch.store_ad(727, &AdValue::min(AdValue::min(scratch.ad_value(445), scratch.ad_value(446)), scratch.ad_value(447)));
        }

        if (scratch.values[1323] != 0.0) {
            scratch.store_ad(728, &AdValue::scale(scratch.ad_value(727), 0.1));
        }

        if (scratch.values[1323] != 0.0) {
            scratch.store_ad(426, &AdValue::max(AdValue::max(scratch.ad_value(448), scratch.ad_value(449)), scratch.ad_value(450)));
        }

        if (scratch.values[1323] != 0.0) {
            scratch.store_ad(729, &AdValue::mul(scratch.ad_value(727), AdValue::sub_from_scalar(1.0, AdValue::pow_from_scalar(2.0, AdValue::div_from_scalar((-1.0), scratch.ad_value(426))))));
        }

        if (scratch.values[1323] != 0.0) {
            scratch.store_ad(730, &AdValue::offset(AdValue::min(AdValue::min(scratch.ad_value(451), scratch.ad_value(452)), scratch.ad_value(453)), (-0.05)));
        }

        scratch.values[1340] = if (scratch.values[410] == 1.0) { 1.0 } else { 0.0 };

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

        scratch.values[2076] = 0.0;

        scratch.values[2077] = 0.0;

        scratch.values[2078] = 0.0;

        scratch.values[943] = 0.0;

        scratch.values[891] = 0.0;

        scratch.values[892] = 0.0;

        scratch.values[893] = 0.0;

        scratch.values[894] = 0.0;

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

    }

    pub(super) fn stamp_reactive_block_10(
        &mut self,
        ctx: &GeneratedEvalContext<'_>,
        stamper: &mut GeneratedReactiveStamper<'_>,
        scratch: &mut ReactiveScratch,
    ) {
        let _ = stamper;
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

    }

    pub(super) fn stamp_reactive_block_11(
        &mut self,
        ctx: &GeneratedEvalContext<'_>,
        stamper: &mut GeneratedReactiveStamper<'_>,
        scratch: &mut ReactiveScratch,
    ) {
        let _ = stamper;
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

        if (!(scratch.values[2220] != 0.0)) {
            scratch.store_ad(2136, &AdValue::mul(scratch.ad_value(2101), scratch.ad_value(2135)));
        }

        scratch.values[2231] = if (((scratch.values[2092]) as f64).abs() <= scratch.values[2102]) { 1.0 } else { 0.0 };

        if ((!(scratch.values[2220] != 0.0)) && (scratch.values[2231] != 0.0)) {
            scratch.store_ad(2178, &AdValue::scale(AdValue::square(scratch.ad_value(2098)), (0.16666666666666666 * 0.7071067811865475)));
        }

        if ((!(scratch.values[2220] != 0.0)) && (scratch.values[2231] != 0.0)) {
            scratch.store_ad(2137, &AdValue::mul(AdValue::mul(scratch.ad_value(2092), scratch.ad_value(2098)), AdValue::offset(AdValue::mul(AdValue::mul(AdValue::mul(scratch.ad_value(2092), AdValue::sub_from_scalar(1.0, scratch.ad_value(2136))), scratch.ad_value(2094)), scratch.ad_value(2178)), 1.0)));
        }

        if ((!(scratch.values[2220] != 0.0)) && (!(scratch.values[2231] != 0.0))) {
            scratch.store_ad(2199, &AdValue::offset(scratch.ad_value(2134), 3.0));
        }

        if ((!(scratch.values[2220] != 0.0)) && (!(scratch.values[2231] != 0.0))) {
            scratch.store_ad(2182, &AdValue::sub(AdValue::scale(AdValue::sub(AdValue::add(scratch.ad_value(2198), scratch.ad_value(2199)), AdValue::sqrt(AdValue::offset(AdValue::mul(AdValue::sub(scratch.ad_value(2198), scratch.ad_value(2199)), AdValue::sub(scratch.ad_value(2198), scratch.ad_value(2199))), 5.0))), 0.5), AdValue::scale(AdValue::sub(scratch.ad_value(2199), AdValue::sqrt(AdValue::offset(AdValue::square(scratch.ad_value(2199)), 5.0))), 0.5)));
        }

        if ((!(scratch.values[2220] != 0.0)) && (!(scratch.values[2231] != 0.0))) {
            scratch.store_ad(2177, &AdValue::sub(scratch.ad_value(2092), scratch.ad_value(2182)));
        }

        if ((!(scratch.values[2220] != 0.0)) && (!(scratch.values[2231] != 0.0))) {
            scratch.store_ad(2178, &AdValue::exp(AdValue::neg(scratch.ad_value(2182))));
        }

        if ((!(scratch.values[2220] != 0.0)) && (!(scratch.values[2231] != 0.0))) {
            scratch.store_ad(2179, &AdValue::div_from_scalar(1.0, AdValue::offset(AdValue::square(scratch.ad_value(2182)), 2.0)));
        }

        if ((!(scratch.values[2220] != 0.0)) && (!(scratch.values[2231] != 0.0))) {
            scratch.store_ad(2189, &AdValue::mul(AdValue::square(scratch.ad_value(2182)), scratch.ad_value(2179)));
        }

        if ((!(scratch.values[2220] != 0.0)) && (!(scratch.values[2231] != 0.0))) {
            scratch.store_ad(2190, &AdValue::scale(AdValue::mul(AdValue::mul(scratch.ad_value(2182), scratch.ad_value(2179)), scratch.ad_value(2179)), 4.0));
        }

        if ((!(scratch.values[2220] != 0.0)) && (!(scratch.values[2231] != 0.0))) {
            scratch.store_ad(2191, &AdValue::mul(AdValue::mul(AdValue::sub(AdValue::scale(scratch.ad_value(2179), 8.0), AdValue::scale(scratch.ad_value(2189), 12.0)), scratch.ad_value(2179)), scratch.ad_value(2179)));
        }

        if ((!(scratch.values[2220] != 0.0)) && (!(scratch.values[2231] != 0.0))) {
            scratch.store_ad(2183, &AdValue::max_from_scalar(1e-40, AdValue::sub(AdValue::square(scratch.ad_value(2177)), AdValue::mul(scratch.ad_value(2095), AdValue::sub(AdValue::offset(AdValue::add(scratch.ad_value(2178), scratch.ad_value(2182)), (-1.0)), AdValue::mul(scratch.ad_value(2136), AdValue::add(AdValue::offset(scratch.ad_value(2182), 1.0), scratch.ad_value(2189))))))));
        }

        if ((!(scratch.values[2220] != 0.0)) && (!(scratch.values[2231] != 0.0))) {
            scratch.store_ad(2200, &AdValue::sub_from_scalar(1.0, AdValue::scale(AdValue::mul(scratch.ad_value(2095), AdValue::sub(scratch.ad_value(2178), AdValue::mul(scratch.ad_value(2136), scratch.ad_value(2191)))), 0.5)));
        }

        if ((!(scratch.values[2220] != 0.0)) && (!(scratch.values[2231] != 0.0))) {
            scratch.store_ad(2184, &AdValue::add(AdValue::scale(scratch.ad_value(2177), 2.0), AdValue::mul(scratch.ad_value(2095), AdValue::sub(AdValue::sub_from_scalar(1.0, scratch.ad_value(2178)), AdValue::mul(scratch.ad_value(2136), AdValue::offset(scratch.ad_value(2190), 1.0))))));
        }

        if ((!(scratch.values[2220] != 0.0)) && (!(scratch.values[2231] != 0.0))) {
            scratch.store_ad(2185, &AdValue::add(AdValue::sub(scratch.ad_value(2134), scratch.ad_value(2182)), AdValue::ln(AdValue::div(scratch.ad_value(2183), scratch.ad_value(2095)))));
        }

        if ((!(scratch.values[2220] != 0.0)) && (!(scratch.values[2231] != 0.0))) {
            scratch.store_ad(966, &AdValue::add(scratch.ad_value(2183), scratch.ad_value(2184)));
        }

        if ((!(scratch.values[2220] != 0.0)) && (!(scratch.values[2231] != 0.0))) {
            scratch.store_ad(965, &AdValue::add(AdValue::square(scratch.ad_value(966)), AdValue::mul(scratch.ad_value(2185), AdValue::sub(AdValue::scale(AdValue::square(scratch.ad_value(2184)), 0.5), AdValue::mul(scratch.ad_value(2183), scratch.ad_value(2200))))));
        }

        if ((!(scratch.values[2220] != 0.0)) && (!(scratch.values[2231] != 0.0))) {
            let assign43110_ad_e55789: AdValue = AdValue::add(scratch.ad_value(2182), AdValue::div(AdValue::mul(AdValue::mul(scratch.ad_value(2183), scratch.ad_value(966)), scratch.ad_value(2185)), AdValue::add(scratch.ad_value(965), AdValue::mul(AdValue::mul(AdValue::mul(AdValue::mul(AdValue::div(scratch.ad_value(966), scratch.ad_value(965)), scratch.ad_value(2185)), scratch.ad_value(2185)), scratch.ad_value(2184)), AdValue::sub(AdValue::scale(AdValue::square(scratch.ad_value(2184)), 0.3333333333333333), AdValue::mul(scratch.ad_value(2183), scratch.ad_value(2200)))))));
            scratch.store_ad(2201, &assign43110_ad_e55789);
        }

        scratch.values[2232] = if (scratch.values[2201] < 230.25850929940458) { 1.0 } else { 0.0 };

        if (((!(scratch.values[2220] != 0.0)) && (!(scratch.values[2231] != 0.0))) && (scratch.values[2232] != 0.0)) {
            scratch.store_ad(2187, &AdValue::exp(scratch.ad_value(2201)));
        }

        if (((!(scratch.values[2220] != 0.0)) && (!(scratch.values[2231] != 0.0))) && (scratch.values[2232] != 0.0)) {
            scratch.store_ad(2188, &AdValue::div_from_scalar(1.0, scratch.ad_value(2187)));
        }

        if (((!(scratch.values[2220] != 0.0)) && (!(scratch.values[2231] != 0.0))) && (scratch.values[2232] != 0.0)) {
            scratch.store_ad(2187, &AdValue::mul(scratch.ad_value(2136), scratch.ad_value(2187)));
        }

        scratch.values[2233] = if (scratch.values[2201] > (scratch.values[2134] - 230.25850929940458)) { 1.0 } else { 0.0 };

        if ((((!(scratch.values[2220] != 0.0)) && (!(scratch.values[2231] != 0.0))) && (!(scratch.values[2232] != 0.0))) && (scratch.values[2233] != 0.0)) {
            scratch.store_ad(2187, &AdValue::exp(AdValue::sub(scratch.ad_value(2201), scratch.ad_value(2134))));
        }

        if ((((!(scratch.values[2220] != 0.0)) && (!(scratch.values[2231] != 0.0))) && (!(scratch.values[2232] != 0.0))) && (scratch.values[2233] != 0.0)) {
            scratch.store_ad(2188, &AdValue::div(scratch.ad_value(2136), scratch.ad_value(2187)));
        }

        if ((((!(scratch.values[2220] != 0.0)) && (!(scratch.values[2231] != 0.0))) && (!(scratch.values[2232] != 0.0))) && (!(scratch.values[2233] != 0.0))) {
            scratch.store_ad(2187, &AdValue::div_from_scalar(1e-100, AdValue::offset(AdValue::mul(AdValue::offset(AdValue::sub(scratch.ad_value(2134), scratch.ad_value(2201)), (-230.25850929940458)), AdValue::offset(AdValue::scale(AdValue::mul(AdValue::offset(AdValue::sub(scratch.ad_value(2134), scratch.ad_value(2201)), (-230.25850929940458)), AdValue::offset(AdValue::scale(AdValue::offset(AdValue::sub(scratch.ad_value(2134), scratch.ad_value(2201)), (-230.25850929940458)), 0.3333333333333333), 1.0)), 0.5), 1.0)), 1.0)));
        }

        if ((((!(scratch.values[2220] != 0.0)) && (!(scratch.values[2231] != 0.0))) && (!(scratch.values[2232] != 0.0))) && (!(scratch.values[2233] != 0.0))) {
            scratch.store_ad(2188, &AdValue::div_from_scalar(1e-100, AdValue::offset(AdValue::mul(AdValue::offset(scratch.ad_value(2201), (-230.25850929940458)), AdValue::offset(AdValue::scale(AdValue::mul(AdValue::offset(scratch.ad_value(2201), (-230.25850929940458)), AdValue::offset(AdValue::scale(AdValue::offset(scratch.ad_value(2201), (-230.25850929940458)), 0.3333333333333333), 1.0)), 0.5), 1.0)), 1.0)));
        }

        if ((!(scratch.values[2220] != 0.0)) && (!(scratch.values[2231] != 0.0))) {
            scratch.store_ad(2177, &AdValue::div_from_scalar(1.0, AdValue::offset(AdValue::square(scratch.ad_value(2201)), 2.0)));
        }

        if ((!(scratch.values[2220] != 0.0)) && (!(scratch.values[2231] != 0.0))) {
            scratch.store_ad(2189, &AdValue::mul(AdValue::square(scratch.ad_value(2201)), scratch.ad_value(2177)));
        }

        if ((!(scratch.values[2220] != 0.0)) && (!(scratch.values[2231] != 0.0))) {
            scratch.store_ad(2190, &AdValue::scale(AdValue::mul(AdValue::mul(scratch.ad_value(2201), scratch.ad_value(2177)), scratch.ad_value(2177)), 4.0));
        }

        if ((!(scratch.values[2220] != 0.0)) && (!(scratch.values[2231] != 0.0))) {
            scratch.store_ad(2191, &AdValue::mul(AdValue::mul(AdValue::sub(AdValue::scale(scratch.ad_value(2177), 8.0), AdValue::scale(scratch.ad_value(2189), 12.0)), scratch.ad_value(2177)), scratch.ad_value(2177)));
        }

        if ((!(scratch.values[2220] != 0.0)) && (!(scratch.values[2231] != 0.0))) {
            scratch.store_ad(2177, &AdValue::sub(scratch.ad_value(2092), scratch.ad_value(2201)));
        }

        if ((!(scratch.values[2220] != 0.0)) && (!(scratch.values[2231] != 0.0))) {
            scratch.store_ad(2192, &AdValue::add(AdValue::scale(scratch.ad_value(2177), 2.0), AdValue::mul(scratch.ad_value(2095), AdValue::sub(AdValue::add(AdValue::sub_from_scalar(1.0, scratch.ad_value(2188)), scratch.ad_value(2187)), AdValue::mul(scratch.ad_value(2136), AdValue::offset(scratch.ad_value(2190), 1.0))))));
        }

        if ((!(scratch.values[2220] != 0.0)) && (!(scratch.values[2231] != 0.0))) {
            scratch.store_ad(2193, &AdValue::sub(AdValue::square(scratch.ad_value(2177)), AdValue::mul(scratch.ad_value(2095), AdValue::sub(AdValue::add(AdValue::offset(AdValue::add(scratch.ad_value(2188), scratch.ad_value(2201)), (-1.0)), scratch.ad_value(2187)), AdValue::mul(scratch.ad_value(2136), AdValue::add(AdValue::offset(scratch.ad_value(2201), 1.0), scratch.ad_value(2189)))))));
        }

        if ((!(scratch.values[2220] != 0.0)) && (!(scratch.values[2231] != 0.0))) {
            scratch.store_ad(2177, &AdValue::sub_from_scalar(2.0, AdValue::mul(scratch.ad_value(2095), AdValue::sub(AdValue::add(scratch.ad_value(2188), scratch.ad_value(2187)), AdValue::mul(scratch.ad_value(2136), scratch.ad_value(2191))))));
        }

        if ((!(scratch.values[2220] != 0.0)) && (!(scratch.values[2231] != 0.0))) {
            scratch.store_ad(2177, &AdValue::sub(AdValue::square(scratch.ad_value(2192)), AdValue::scale(AdValue::mul(scratch.ad_value(2193), scratch.ad_value(2177)), 2.0)));
        }

        if ((!(scratch.values[2220] != 0.0)) && (!(scratch.values[2231] != 0.0))) {
            scratch.store_ad(2137, &AdValue::add(scratch.ad_value(2201), AdValue::scale(AdValue::div(scratch.ad_value(2193), AdValue::add(scratch.ad_value(2192), AdValue::sqrt(scratch.ad_value(2177)))), 2.0)));
        }

        if (!(scratch.values[2220] != 0.0)) {
            scratch.store_ad(2138, &AdValue::sub(scratch.ad_value(2137), scratch.ad_value(2103)));
        }

        scratch.values[2234] = if (scratch.values[2138] < 1e-10) { 1.0 } else { 0.0 };

        if ((!(scratch.values[2220] != 0.0)) && (scratch.values[2234] != 0.0)) {
            scratch.store_ad(2139, &AdValue::add(AdValue::scale(AdValue::sub(scratch.ad_value(2092), scratch.ad_value(2103)), 2.0), AdValue::mul(scratch.ad_value(2095), AdValue::sub(AdValue::add(AdValue::sub_from_scalar(1.0, scratch.ad_value(2108)), AdValue::mul(scratch.ad_value(2104), scratch.ad_value(2135))), AdValue::mul(scratch.ad_value(2136), AdValue::offset(scratch.ad_value(2106), 1.0))))));
        }

        if ((!(scratch.values[2220] != 0.0)) && (scratch.values[2234] != 0.0)) {
            scratch.store_ad(2140, &AdValue::mul(AdValue::mul(scratch.ad_value(2095), AdValue::sub_from_scalar(1.0, scratch.ad_value(2135))), scratch.ad_value(2109)));
        }

        if ((!(scratch.values[2220] != 0.0)) && (scratch.values[2234] != 0.0)) {
            scratch.store_ad(2076, &AdValue::sub_from_scalar(2.0, AdValue::mul(scratch.ad_value(2095), AdValue::sub(AdValue::add(scratch.ad_value(2108), AdValue::mul(scratch.ad_value(2104), scratch.ad_value(2135))), AdValue::mul(scratch.ad_value(2136), scratch.ad_value(2107))))));
        }

        if ((!(scratch.values[2220] != 0.0)) && (scratch.values[2234] != 0.0)) {
            scratch.store_ad(2076, &AdValue::sub(AdValue::square(scratch.ad_value(2139)), AdValue::scale(AdValue::mul(scratch.ad_value(2076), scratch.ad_value(2140)), 2.0)));
        }

        if ((!(scratch.values[2220] != 0.0)) && (scratch.values[2234] != 0.0)) {
            scratch.store_ad(2138, &AdValue::scale(AdValue::div(scratch.ad_value(2140), AdValue::add(scratch.ad_value(2139), AdValue::sqrt(scratch.ad_value(2076)))), 2.0));
        }

        if ((!(scratch.values[2220] != 0.0)) && (scratch.values[2234] != 0.0)) {
            scratch.store_ad(2137, &AdValue::add(scratch.ad_value(2103), scratch.ad_value(2138)));
        }

        if (!(scratch.values[2220] != 0.0)) {
            scratch.store_ad(2141, &AdValue::mul(scratch.ad_value(2138), scratch.ad_value(2011)));
        }

        if (!(scratch.values[2220] != 0.0)) {
            scratch.store_ad(2142, &AdValue::div(AdValue::square(scratch.ad_value(2137)), AdValue::offset(AdValue::square(scratch.ad_value(2137)), 2.0)));
        }

        scratch.values[2235] = if (scratch.values[2137] < 230.25850929940458) { 1.0 } else { 0.0 };

        if ((!(scratch.values[2220] != 0.0)) && (scratch.values[2235] != 0.0)) {
            scratch.store_ad(2143, &AdValue::exp(AdValue::neg(scratch.ad_value(2137))));
        }

        scratch.values[2236] = if (scratch.values[2137] < 1e-5) { 1.0 } else { 0.0 };

        if (((!(scratch.values[2220] != 0.0)) && (scratch.values[2235] != 0.0)) && (scratch.values[2236] != 0.0)) {
            scratch.store_ad(2144, &AdValue::mul(AdValue::mul(AdValue::mul(AdValue::mul(AdValue::scale(scratch.ad_value(2136), 0.16666666666666666), scratch.ad_value(2137)), scratch.ad_value(2137)), scratch.ad_value(2137)), AdValue::offset(AdValue::scale(scratch.ad_value(2137), 1.75), 1.0)));
        }

        if (((!(scratch.values[2220] != 0.0)) && (scratch.values[2235] != 0.0)) && (!(scratch.values[2236] != 0.0))) {
            scratch.store_ad(2144, &AdValue::mul(scratch.ad_value(2136), AdValue::sub(AdValue::offset(AdValue::sub(AdValue::div_from_scalar(1.0, scratch.ad_value(2143)), scratch.ad_value(2137)), (-1.0)), scratch.ad_value(2142))));
        }

        scratch.values[2237] = if (scratch.values[2137] > (scratch.values[2134] - 230.25850929940458)) { 1.0 } else { 0.0 };

        if (((!(scratch.values[2220] != 0.0)) && (!(scratch.values[2235] != 0.0))) && (scratch.values[2237] != 0.0)) {
            scratch.store_ad(2076, &AdValue::exp(AdValue::sub(scratch.ad_value(2137), scratch.ad_value(2134))));
        }

        if (((!(scratch.values[2220] != 0.0)) && (!(scratch.values[2235] != 0.0))) && (scratch.values[2237] != 0.0)) {
            scratch.store_ad(2143, &AdValue::div(scratch.ad_value(2136), scratch.ad_value(2076)));
        }

        if (((!(scratch.values[2220] != 0.0)) && (!(scratch.values[2235] != 0.0))) && (scratch.values[2237] != 0.0)) {
            scratch.store_ad(2144, &AdValue::sub(scratch.ad_value(2076), AdValue::mul(scratch.ad_value(2136), AdValue::add(AdValue::offset(scratch.ad_value(2137), 1.0), scratch.ad_value(2142)))));
        }

        if (((!(scratch.values[2220] != 0.0)) && (!(scratch.values[2235] != 0.0))) && (!(scratch.values[2237] != 0.0))) {
            scratch.store_ad(2143, &AdValue::div_from_scalar(1e-100, AdValue::offset(AdValue::mul(AdValue::offset(scratch.ad_value(2137), (-230.25850929940458)), AdValue::offset(AdValue::scale(AdValue::mul(AdValue::offset(scratch.ad_value(2137), (-230.25850929940458)), AdValue::offset(AdValue::scale(AdValue::offset(scratch.ad_value(2137), (-230.25850929940458)), 0.3333333333333333), 1.0)), 0.5), 1.0)), 1.0)));
        }

        if (((!(scratch.values[2220] != 0.0)) && (!(scratch.values[2235] != 0.0))) && (!(scratch.values[2237] != 0.0))) {
            scratch.store_ad(2076, &AdValue::div_from_scalar(1e-100, AdValue::offset(AdValue::mul(AdValue::offset(AdValue::sub(scratch.ad_value(2134), scratch.ad_value(2137)), (-230.25850929940458)), AdValue::offset(AdValue::scale(AdValue::mul(AdValue::offset(AdValue::sub(scratch.ad_value(2134), scratch.ad_value(2137)), (-230.25850929940458)), AdValue::offset(AdValue::scale(AdValue::offset(AdValue::sub(scratch.ad_value(2134), scratch.ad_value(2137)), (-230.25850929940458)), 0.3333333333333333), 1.0)), 0.5), 1.0)), 1.0)));
        }

        if (((!(scratch.values[2220] != 0.0)) && (!(scratch.values[2235] != 0.0))) && (!(scratch.values[2237] != 0.0))) {
            scratch.store_ad(2144, &AdValue::sub(scratch.ad_value(2076), AdValue::mul(scratch.ad_value(2136), AdValue::add(AdValue::offset(scratch.ad_value(2137), 1.0), scratch.ad_value(2142)))));
        }

        if (!(scratch.values[2220] != 0.0)) {
            scratch.store_ad(2145, &AdValue::scale(AdValue::add(scratch.ad_value(2103), scratch.ad_value(2137)), 0.5));
        }

        if (!(scratch.values[2220] != 0.0)) {
            scratch.values[2146] = 0.0;
            scratch.node_derivatives[2146] = [0.0; Instance::NODE_COUNT];
            scratch.branch_derivatives[2146] = [0.0; Instance::BRANCH_COUNT];
        }

        if (!(scratch.values[2220] != 0.0)) {
            scratch.store_ad(2076, &AdValue::mul(scratch.ad_value(2143), scratch.ad_value(2108)));
        }

        scratch.values[2238] = if (scratch.values[2076] > 0.0) { 1.0 } else { 0.0 };

        if ((!(scratch.values[2220] != 0.0)) && (scratch.values[2238] != 0.0)) {
            scratch.store_ad(2146, &AdValue::sqrt(scratch.ad_value(2076)));
        }

        if (!(scratch.values[2220] != 0.0)) {
            scratch.store_ad(2147, &AdValue::scale(AdValue::add(scratch.ad_value(2109), scratch.ad_value(2144)), 0.5));
        }

        if (!(scratch.values[2220] != 0.0)) {
            scratch.store_ad(2148, &AdValue::add(scratch.ad_value(2147), AdValue::scale(AdValue::mul(AdValue::square(scratch.ad_value(2138)), AdValue::sub(scratch.ad_value(2146), AdValue::scale(scratch.ad_value(2096), 2.0))), 0.125)));
        }

        scratch.values[2239] = if (scratch.values[2145] < 1e-5) { 1.0 } else { 0.0 };

        if ((!(scratch.values[2220] != 0.0)) && (scratch.values[2239] != 0.0)) {
            scratch.store_ad(2149, &AdValue::scale(AdValue::mul(AdValue::square(scratch.ad_value(2145)), AdValue::sub_from_scalar(1.0, AdValue::scale(AdValue::mul(scratch.ad_value(2145), AdValue::sub_from_scalar(1.0, AdValue::scale(scratch.ad_value(2145), 0.25))), 0.3333333333333333))), 0.5));
        }

        if ((!(scratch.values[2220] != 0.0)) && (scratch.values[2239] != 0.0)) {
            scratch.store_ad(2150, &AdValue::mul(scratch.ad_value(2094), AdValue::sqrt(AdValue::add(scratch.ad_value(2148), scratch.ad_value(2149)))));
        }

        scratch.values[2240] = if (scratch.values[773] > 0.0) { 1.0 } else { 0.0 };

        if (((!(scratch.values[2220] != 0.0)) && (scratch.values[2239] != 0.0)) && (scratch.values[2240] != 0.0)) {
            scratch.store_ad(2151, &AdValue::div_from_scalar(1.0, AdValue::sqrt(AdValue::offset(AdValue::mul(scratch.ad_value(773), scratch.ad_value(2150)), 1.0))));
        }

        if ((!(scratch.values[2220] != 0.0)) && (scratch.values[2239] != 0.0)) {
            scratch.store_ad(2076, &AdValue::sqrt(AdValue::sub_from_scalar(1.0, AdValue::scale(AdValue::mul(scratch.ad_value(2145), AdValue::sub_from_scalar(1.0, AdValue::scale(scratch.ad_value(2145), 0.25))), 0.3333333333333333))));
        }

        if ((!(scratch.values[2220] != 0.0)) && (scratch.values[2239] != 0.0)) {
            scratch.store_ad(2152, &AdValue::scale(AdValue::mul(scratch.ad_value(2145), scratch.ad_value(2076)), 0.7071067811865475));
        }

        if ((!(scratch.values[2220] != 0.0)) && (scratch.values[2239] != 0.0)) {
            scratch.store_ad(2153, &AdValue::add(scratch.ad_value(2151), AdValue::scale(AdValue::div(AdValue::mul(scratch.ad_value(2094), AdValue::add(AdValue::sub_from_scalar(1.0, AdValue::scale(scratch.ad_value(2145), 0.5)), AdValue::scale(AdValue::square(scratch.ad_value(2145)), 0.16666666666666666))), scratch.ad_value(2076)), 0.7071067811865475)));
        }

        if ((!(scratch.values[2220] != 0.0)) && (!(scratch.values[2239] != 0.0))) {
            scratch.store_ad(2149, &AdValue::add(AdValue::offset(scratch.ad_value(2145), (-1.0)), scratch.ad_value(2146)));
        }

        if ((!(scratch.values[2220] != 0.0)) && (!(scratch.values[2239] != 0.0))) {
            scratch.store_ad(2150, &AdValue::mul(scratch.ad_value(2094), AdValue::sqrt(AdValue::add(scratch.ad_value(2148), scratch.ad_value(2149)))));
        }

        scratch.values[2241] = if (scratch.values[773] > 0.0) { 1.0 } else { 0.0 };

        if (((!(scratch.values[2220] != 0.0)) && (!(scratch.values[2239] != 0.0))) && (scratch.values[2241] != 0.0)) {
            scratch.store_ad(2154, &AdValue::add(AdValue::sub_from_scalar(1.0, scratch.ad_value(2146)), AdValue::scale(AdValue::mul(scratch.ad_value(2150), scratch.ad_value(2096)), 2.0)));
        }

        if (((!(scratch.values[2220] != 0.0)) && (!(scratch.values[2239] != 0.0))) && (scratch.values[2241] != 0.0)) {
            scratch.store_ad(2151, &AdValue::div_from_scalar(1.0, AdValue::sqrt(AdValue::offset(AdValue::mul(scratch.ad_value(773), scratch.ad_value(2150)), 1.0))));
        }

        if (((!(scratch.values[2220] != 0.0)) && (!(scratch.values[2239] != 0.0))) && (scratch.values[2241] != 0.0)) {
            scratch.store_ad(2076, &AdValue::div(scratch.ad_value(2151), AdValue::offset(scratch.ad_value(2151), 1.0)));
        }

        if (((!(scratch.values[2220] != 0.0)) && (!(scratch.values[2239] != 0.0))) && (scratch.values[2241] != 0.0)) {
            scratch.store_ad(2155, &AdValue::mul(scratch.ad_value(773), AdValue::mul(AdValue::mul(AdValue::square(scratch.ad_value(2076)), scratch.ad_value(2095)), scratch.ad_value(2148))));
        }

        if (((!(scratch.values[2220] != 0.0)) && (!(scratch.values[2239] != 0.0))) && (scratch.values[2241] != 0.0)) {
            scratch.store_ad(2156, &AdValue::add(AdValue::scale(AdValue::sub(scratch.ad_value(2150), scratch.ad_value(2155)), 2.0), AdValue::mul(scratch.ad_value(2095), AdValue::add(AdValue::sub_from_scalar(1.0, scratch.ad_value(2146)), scratch.ad_value(2148)))));
        }

        if (((!(scratch.values[2220] != 0.0)) && (!(scratch.values[2239] != 0.0))) && (scratch.values[2241] != 0.0)) {
            scratch.store_ad(2157, &AdValue::mul(scratch.ad_value(2155), AdValue::sub(scratch.ad_value(2155), AdValue::scale(scratch.ad_value(2150), 2.0))));
        }

        if (((!(scratch.values[2220] != 0.0)) && (!(scratch.values[2239] != 0.0))) && (scratch.values[2241] != 0.0)) {
            scratch.store_ad(2158, &AdValue::sub_from_scalar(1.0, AdValue::scale(AdValue::mul(scratch.ad_value(2095), AdValue::add(scratch.ad_value(2146), scratch.ad_value(2148))), 0.5)));
        }

        if (((!(scratch.values[2220] != 0.0)) && (!(scratch.values[2239] != 0.0))) && (scratch.values[2241] != 0.0)) {
            scratch.store_ad(2159, &AdValue::div(AdValue::mul(scratch.ad_value(2157), scratch.ad_value(2156)), AdValue::sub(AdValue::square(scratch.ad_value(2156)), AdValue::mul(scratch.ad_value(2158), scratch.ad_value(2157)))));
        }

        if (((!(scratch.values[2220] != 0.0)) && (!(scratch.values[2239] != 0.0))) && (scratch.values[2241] != 0.0)) {
            scratch.store_ad(2145, &AdValue::add(scratch.ad_value(2145), scratch.ad_value(2159)));
        }

        if (((!(scratch.values[2220] != 0.0)) && (!(scratch.values[2239] != 0.0))) && (scratch.values[2241] != 0.0)) {
            scratch.store_ad(2160, &AdValue::exp(scratch.ad_value(2159)));
        }

        if (((!(scratch.values[2220] != 0.0)) && (!(scratch.values[2239] != 0.0))) && (scratch.values[2241] != 0.0)) {
            scratch.store_ad(2146, &AdValue::div(scratch.ad_value(2146), scratch.ad_value(2160)));
        }

        if (((!(scratch.values[2220] != 0.0)) && (!(scratch.values[2239] != 0.0))) && (scratch.values[2241] != 0.0)) {
            scratch.store_ad(2148, &AdValue::mul(scratch.ad_value(2148), scratch.ad_value(2160)));
        }

        if (((!(scratch.values[2220] != 0.0)) && (!(scratch.values[2239] != 0.0))) && (scratch.values[2241] != 0.0)) {
            scratch.store_ad(2149, &AdValue::add(AdValue::offset(scratch.ad_value(2145), (-1.0)), scratch.ad_value(2146)));
        }

        if (((!(scratch.values[2220] != 0.0)) && (!(scratch.values[2239] != 0.0))) && (scratch.values[2241] != 0.0)) {
            scratch.store_ad(2150, &AdValue::mul(scratch.ad_value(2094), AdValue::sqrt(AdValue::add(scratch.ad_value(2148), scratch.ad_value(2149)))));
        }

        if (((!(scratch.values[2220] != 0.0)) && (!(scratch.values[2239] != 0.0))) && (scratch.values[2241] != 0.0)) {
            scratch.store_ad(2161, &AdValue::add(AdValue::sub_from_scalar(1.0, scratch.ad_value(2146)), AdValue::scale(AdValue::mul(AdValue::mul(scratch.ad_value(2150), scratch.ad_value(2151)), scratch.ad_value(2096)), 2.0)));
        }

        if (((!(scratch.values[2220] != 0.0)) && (!(scratch.values[2239] != 0.0))) && (scratch.values[2241] != 0.0)) {
            scratch.store_ad(2138, &AdValue::div(AdValue::mul(AdValue::mul(scratch.ad_value(2138), scratch.ad_value(2160)), AdValue::add(scratch.ad_value(2154), scratch.ad_value(2147))), AdValue::add(scratch.ad_value(2161), AdValue::mul(scratch.ad_value(2160), scratch.ad_value(2147)))));
        }

        if (((!(scratch.values[2220] != 0.0)) && (!(scratch.values[2239] != 0.0))) && (scratch.values[2241] != 0.0)) {
            scratch.store_ad(2141, &AdValue::mul(scratch.ad_value(2138), scratch.ad_value(2011)));
        }

        if ((!(scratch.values[2220] != 0.0)) && (!(scratch.values[2239] != 0.0))) {
            scratch.store_ad(2152, &AdValue::sqrt(scratch.ad_value(2149)));
        }

        if ((!(scratch.values[2220] != 0.0)) && (!(scratch.values[2239] != 0.0))) {
            scratch.store_ad(2153, &AdValue::add(scratch.ad_value(2151), AdValue::scale(AdValue::div(AdValue::mul(scratch.ad_value(2094), AdValue::sub_from_scalar(1.0, scratch.ad_value(2146))), scratch.ad_value(2152)), 0.5)));
        }

        if (!(scratch.values[2220] != 0.0)) {
            scratch.store_ad(2162, &AdValue::mul(scratch.ad_value(2011), AdValue::div(AdValue::mul(scratch.ad_value(2095), scratch.ad_value(2148)), AdValue::add(scratch.ad_value(2150), AdValue::mul(scratch.ad_value(2094), scratch.ad_value(2152))))));
        }

        if (!(scratch.values[2220] != 0.0)) {
            scratch.store_ad(2163, &AdValue::add(scratch.ad_value(2162), AdValue::mul(scratch.ad_value(2011), scratch.ad_value(2153))));
        }

        if (!(scratch.values[2220] != 0.0)) {
            scratch.store_ad(2164, &AdValue::mul(AdValue::mul(scratch.ad_value(2152), scratch.ad_value(2094)), scratch.ad_value(2011)));
        }

        scratch.values[2242] = if (scratch.values[235] < 0.0) { 1.0 } else { 0.0 };

        if ((!(scratch.values[2220] != 0.0)) && (scratch.values[2242] != 0.0)) {
            scratch.store_ad(2076, &AdValue::sub_from_scalar(1.0, AdValue::mul(scratch.ad_value(235), scratch.ad_value(2162))));
        }

        if ((!(scratch.values[2220] != 0.0)) && (!(scratch.values[2242] != 0.0))) {
            scratch.store_ad(2076, &AdValue::div_from_scalar(1.0, AdValue::offset(AdValue::mul(scratch.ad_value(235), scratch.ad_value(2162)), 1.0)));
        }

        if (!(scratch.values[2220] != 0.0)) {
            scratch.store_ad(2116, &AdValue::mul(scratch.ad_value(800), AdValue::mul(AdValue::mul(scratch.ad_value(2115), scratch.ad_value(2076)), scratch.ad_value(2162))));
        }

        if (!(scratch.values[2220] != 0.0)) {
            scratch.store_ad(2165, &AdValue::add(scratch.ad_value(2164), AdValue::mul(scratch.ad_value(818), scratch.ad_value(2162))));
        }

        if (!(scratch.values[2220] != 0.0)) {
            scratch.store_ad(2166, &AdValue::add(scratch.ad_value(2164), AdValue::mul(scratch.ad_value(819), scratch.ad_value(2162))));
        }

        if (!(scratch.values[2220] != 0.0)) {
            scratch.store_ad(2117, &AdValue::mul(scratch.ad_value(817), scratch.ad_value(2165)));
        }

        if (!(scratch.values[2220] != 0.0)) {
            scratch.store_ad(2077, &AdValue::ln(AdValue::div(scratch.ad_value(2149), AdValue::offset(AdValue::add(scratch.ad_value(2149), scratch.ad_value(2148)), 1e-14))));
        }

        if (!(scratch.values[2220] != 0.0)) {
            scratch.store_ad(2118, &AdValue::add(AdValue::pow(AdValue::mul(scratch.ad_value(2117), scratch.ad_value(748)), scratch.ad_value(749)), AdValue::mul(scratch.ad_value(750), AdValue::exp(AdValue::mul(AdValue::scale(scratch.ad_value(751), 0.5), scratch.ad_value(2077))))));
        }

        if (!(scratch.values[2220] != 0.0)) {
            scratch.store_ad(2119, &AdValue::mul(AdValue::add(AdValue::offset(scratch.ad_value(2118), 1.0), scratch.ad_value(2116)), scratch.ad_value(2111)));
        }

        if (!(scratch.values[2220] != 0.0)) {
            scratch.store_ad(2167, &AdValue::ln(AdValue::div(AdValue::offset(AdValue::mul(AdValue::sub(scratch.ad_value(854), scratch.ad_value(2141)), scratch.ad_value(821)), 1.0), AdValue::offset(AdValue::mul(AdValue::sub(scratch.ad_value(2132), scratch.ad_value(2141)), scratch.ad_value(821)), 1.0))));
        }

        if (!(scratch.values[2220] != 0.0)) {
            scratch.store_ad(2168, &AdValue::mul(scratch.ad_value(241), scratch.ad_value(2167)));
        }

        if (!(scratch.values[2220] != 0.0)) {
            scratch.store_ad(2169, &AdValue::div_from_scalar(1.0, AdValue::add(AdValue::offset(scratch.ad_value(2168), 1.0), AdValue::square(scratch.ad_value(2168)))));
        }

        if (!(scratch.values[2220] != 0.0)) {
            scratch.store_ad(2078, &AdValue::mul(scratch.ad_value(2162), scratch.ad_value(2120)));
        }

        if (!(scratch.values[2220] != 0.0)) {
            scratch.store_ad(2121, &AdValue::scale(AdValue::div(scratch.ad_value(2078), AdValue::offset(scratch.ad_value(2078), 100.0)), 100.0));
        }

        if (!(scratch.values[2220] != 0.0)) {
            scratch.store_ad(2170, &AdValue::mul(scratch.ad_value(2119), scratch.ad_value(2169)));
        }

        scratch.values[2243] = if (scratch.values[239] < 0.0) { 1.0 } else { 0.0 };

        if ((!(scratch.values[2220] != 0.0)) && (scratch.values[2243] != 0.0)) {
            scratch.store_ad(2076, &AdValue::div_from_scalar(1.0, AdValue::sub_from_scalar(1.0, AdValue::mul(scratch.ad_value(239), scratch.ad_value(2121)))));
        }

        if ((!(scratch.values[2220] != 0.0)) && (!(scratch.values[2243] != 0.0))) {
            scratch.store_ad(2076, &AdValue::offset(AdValue::mul(scratch.ad_value(239), scratch.ad_value(2121)), 1.0));
        }

        if (!(scratch.values[2220] != 0.0)) {
            scratch.store_ad(2122, &AdValue::mul(scratch.ad_value(764), AdValue::div(scratch.ad_value(2076), scratch.ad_value(2170))));
        }

        if (!(scratch.values[2220] != 0.0)) {
            scratch.store_ad(2171, &AdValue::mul(AdValue::mul(AdValue::square(scratch.ad_value(2122)), scratch.ad_value(2141)), scratch.ad_value(2141)));
        }

        scratch.values[2244] = if (scratch.values[0] == (-1.0)) { 1.0 } else { 0.0 };

    }
}
