#![allow(dead_code, unused_assignments, unused_imports, unused_parens, unused_variables)]

use super::{A, ddt_jacobian, eval_ddt, eval_idt, GeneratedDerivative, GeneratedEvalContext, GeneratedReactiveStamper, GeneratedStamper, idt_jacobian, ReactiveScratch, Scratch, LIMEXP_MAX, THERMAL_VOLTAGE_PER_K};
use super::super::state::{Instance, Parameters};

impl Instance {

    pub(super) fn stamp_transient_block_16(
        s: &mut Scratch,
    ) {
        if s.b[1679] {
            s.copy_ad(1410, 1414);
            s.store_scaled_sub(1179, 1162, 1409, 1.0 / (s.v[367]));
        }

        s.b[1686] = (s.v[1179] > 100.0);
        s.v[1686] = if s.b[1686] { 1.0 } else { 0.0 };

        if (s.b[1679] && s.b[1686]) {
            s.store_scaled_offset_ad(1180, A::offset(s.ad_value(1179), 1.0), (-100.0), 2.688117142e43);
        }

        s.b[1687] = (s.v[1179] < (-100.0));
        s.v[1687] = if s.b[1687] { 1.0 } else { 0.0 };

        if ((s.b[1679] && (!s.b[1686])) && s.b[1687]) {
            s.store_scalar(1180, 3.720075976e-44);
        }

        if ((s.b[1679] && (!s.b[1686])) && (!s.b[1687])) {
            s.store_exp(1180, 1179);
        }

        if s.b[1679] {
            s.store_scaled_ln_ad(1412, A::offset(s.ad_value(1180), 1.0), s.v[367]);
        }

        s.b[1688] = (s.v[370] != 0.0);
        s.v[1688] = if s.b[1688] { 1.0 } else { 0.0 };

        if (s.b[1679] && s.b[1688]) {
            s.store_sub_from_scalar_ad(1179, 1.0, A::scale(s.ad_value(1410), 1.0 / (s.v[370])));
        }

        if (s.b[1679] && (!s.b[1688])) {
            s.store_scalar(1179, 1.0);
        }

        s.b[1689] = (s.v[1179] < 0.01);
        s.v[1689] = if s.b[1689] { 1.0 } else { 0.0 };

        if (s.b[1679] && s.b[1689]) {
            s.store_scalar(1179, 0.01);
        }

        if s.b[1679] {
            s.store_mul_ad_product_lhs(1180, A::offset(A::scale(s.ad_value(1228), (s.v[1227] * 1.0 / (s.v[59]))), (s.v[64] / s.v[39])), s.ad_value(786), 1411);
            s.store_scale(1181, 787, s.v[357]);
            s.copy_ad(1182, 611);
            s.copy_ad(1183, 612);
            s.store_div_ad_lhs(1185, A::mul(s.ad_value(1181), A::sub(s.ad_value(1182), A::mul(s.ad_value(1183), s.ad_value(1410)))), 1179);
        }

        s.b[1690] = (s.v[1185] > 100.0);
        s.v[1690] = if s.b[1690] { 1.0 } else { 0.0 };

        if (s.b[1679] && s.b[1690]) {
            s.store_scaled_offset_ad(1184, A::offset(s.ad_value(1185), 1.0), (-100.0), 2.688117142e43);
        }

        s.b[1691] = (s.v[1185] < (-100.0));
        s.v[1691] = if s.b[1691] { 1.0 } else { 0.0 };

        if ((s.b[1679] && (!s.b[1690])) && s.b[1691]) {
            s.store_scalar(1184, 3.720075976e-44);
        }

        if ((s.b[1679] && (!s.b[1690])) && (!s.b[1691])) {
            s.store_exp(1184, 1185);
        }

        if s.b[1679] {
            s.store_mul_ad_product_lhs(1418, A::mul(s.ad_value(1180), s.ad_value(1409)), s.ad_value(1412), 1184);
        }

        s.b[1692] = (s.v[1409] >= 0.0);
        s.v[1692] = if s.b[1692] { 1.0 } else { 0.0 };

        if (s.b[1679] && s.b[1692]) {
            s.copy_ad(1413, 1417);
        }

        if (s.b[1679] && (!s.b[1692])) {
            s.copy_ad(1413, 1418);
        }

        if s.b[1679] {
            s.store_add(1460, 1162, 781);
        }

        if (!s.b[1679]) {
            s.store_scalar(1413, 0.0);
        }

        s.store_scale(412, 1413, s.v[36]);

        s.b[1693] = (((((s.v[355] != 0.0) && (s.v[57] != 2.0)) && (s.v[760] != 0.0)) && (s.v[63] > 0.0)) && (s.v[1447] < s.v[1460]));
        s.v[1693] = if s.b[1693] { 1.0 } else { 0.0 };

        if s.b[1693] {
            s.store_sub(1179, 1447, 1460);
            s.store_sqrt_square_offset(1180, 1179, 0.0001);
            s.store_offset_scaled_sub(1446, 1180, 1179, 0.5, (((-0.01)) * (0.5)));
        }

        if s.b[1693] {
            s.store_ad_value(1190, {
                if (s.v[36] == 1.0) {
                    s.ad_value(788)
                } else {
                    s.ad_value(789)
                }
            });
        }

        if s.b[1693] {
            s.store_ad_value(1191, {
                if (s.v[36] == 1.0) {
                    s.ad_value(790)
                } else {
                    s.ad_value(791)
                }
            });
        }

        if s.b[1693] {
            s.store_mul(1181, 1447, 1446);
            s.store_sub_ad_lhs(1182, A::mul(s.ad_value(613), s.ad_value(615)), 614);
            s.store_mul(1183, 614, 615);
            s.store_mul_scaled_ad_rhs(1184, 1191, (-s.v[357]), A::sub(A::add(s.ad_value(613), A::mul(s.ad_value(1182), s.ad_value(1446))), A::mul(A::mul(s.ad_value(1183), s.ad_value(1446)), s.ad_value(1446))));
        }

        s.b[1694] = (s.v[1184] > 100.0);
        s.v[1694] = if s.b[1694] { 1.0 } else { 0.0 };

        if (s.b[1693] && s.b[1694]) {
            s.store_scalar(1185, 2.688117142e43);
        }

        s.b[1695] = (s.v[1184] < (-100.0));
        s.v[1695] = if s.b[1695] { 1.0 } else { 0.0 };

        if ((s.b[1693] && (!s.b[1694])) && s.b[1695]) {
            s.store_scalar(1185, 3.720075976e-44);
        }

        if ((s.b[1693] && (!s.b[1694])) && (!s.b[1695])) {
            s.store_exp(1185, 1184);
        }

        if s.b[1693] {
            s.store_scale(1190, 1190, (s.v[63] * s.v[706]));
            s.store_mul3_lhs(1445, 1190, 1181, 1185);
        }

        if (!s.b[1693]) {
            s.store_scalar(1445, 0.0);
        }

        s.store_scale(417, 1445, s.v[36]);

        s.b[1696] = (s.v[57] != 2.0);
        s.v[1696] = if s.b[1696] { 1.0 } else { 0.0 };

        s.b[1697] = (s.v[71] == 0.0);
        s.v[1697] = if s.b[1697] { 1.0 } else { 0.0 };

        s.b[1698] = (s.v[570] <= 0.0);
        s.v[1698] = if s.b[1698] { 1.0 } else { 0.0 };

        if ((s.b[1696] && s.b[1697]) && s.b[1698]) {
            s.store_scalar(1243, 0.0);
        }

        if ((s.b[1696] && s.b[1697]) && (!s.b[1698])) {
            s.store_sub_scaled_ad_lhs(1301, A::mul(s.ad_value(638), A::offset(A::scale(A::offset(s.ad_value(771), (-1.0)), s.v[289]), 1.0)), 639, 1.0 / (s.v[1227]));
            s.store_scale(1179, 640, s.v[1227]);
            s.store_div_ad(1180, A::mul(s.ad_value(641), s.ad_value(1179)), A::offset(s.ad_value(1179), 1.0));
            s.store_div_from_scalar_offset_ad(1179, 1.0, A::mul(s.ad_value(642), s.ad_value(1210)), 1.0);
            s.store_add(1182, 1179, 643);
            s.store_mul(1181, 1166, 1182);
            s.store_div_from_scalar_offset_ad(1182, 1.0, A::mul(s.ad_value(644), s.ad_value(1158)), 1.0);
            s.store_mul3_lhs(1302, 1180, 1181, 1182);
            s.store_add(1256, 1301, 1302);
            s.store_sub(1304, 1158, 1256);
            s.store_add_ad(1179, A::add(s.ad_value(637), A::mul(s.ad_value(636), s.ad_value(1304))), A::mul(A::mul(s.ad_value(571), s.ad_value(1304)), s.ad_value(1304)));
        }

        s.b[1699] = (s.v[1179] < 1e-5);
        s.v[1699] = if s.b[1699] { 1.0 } else { 0.0 };

        if (((s.b[1696] && s.b[1697]) && (!s.b[1698])) && s.b[1699]) {
            s.store_scalar(1179, 1e-5);
        }

        s.b[1700] = ((s.v[1179] < (s.v[1304] / 100.0)) && (s.v[1304] > 0.0));
        s.v[1700] = if s.b[1700] { 1.0 } else { 0.0 };

        if (((s.b[1696] && s.b[1697]) && (!s.b[1698])) && s.b[1700]) {
            s.store_scale(1303, 570, 2.688117142e43);
        }

        s.b[1701] = ((s.v[1179] < ((-s.v[1304]) / 100.0)) && (s.v[1304] < 0.0));
        s.v[1701] = if s.b[1701] { 1.0 } else { 0.0 };

        if ((((s.b[1696] && s.b[1697]) && (!s.b[1698])) && (!s.b[1700])) && s.b[1701]) {
            s.store_scale(1303, 570, 3.720075976e-44);
        }

        if ((((s.b[1696] && s.b[1697]) && (!s.b[1698])) && (!s.b[1700])) && (!s.b[1701])) {
            s.store_mul_exp_ad_rhs(1303, 570, A::div(s.ad_value(1304), s.ad_value(1179)));
        }

        s.b[1702] = (s.v[1303] > 10.0);
        s.v[1702] = if s.b[1702] { 1.0 } else { 0.0 };

        if (((s.b[1696] && s.b[1697]) && (!s.b[1698])) && s.b[1702]) {
            s.store_scalar(1303, 10.0);
        }

        if ((s.b[1696] && s.b[1697]) && (!s.b[1698])) {
            s.store_add_ad_rhs(1179, 1220, A::mul(A::mul(s.ad_value(630), s.ad_value(759)), s.ad_value(1268)));
            s.store_mul(1243, 1303, 1179);
        }

        s.b[1703] = (s.v[570] <= 0.0);
        s.v[1703] = if s.b[1703] { 1.0 } else { 0.0 };

        if ((s.b[1696] && (!s.b[1697])) && s.b[1703]) {
            s.store_scalar(1439, 0.0);
        }

        if ((s.b[1696] && (!s.b[1697])) && (!s.b[1703])) {
            s.store_sub_scaled_ad_lhs(1301, A::mul(s.ad_value(638), A::offset(A::scale(A::offset(s.ad_value(771), (-1.0)), s.v[289]), 1.0)), 639, 1.0 / (s.v[1227]));
            s.store_scale(1179, 640, s.v[1227]);
            s.store_div_ad(1180, A::mul(s.ad_value(641), s.ad_value(1179)), A::offset(s.ad_value(1179), 1.0));
            s.store_div_from_scalar_offset_ad(1179, 1.0, A::mul(s.ad_value(642), s.ad_value(1210)), 1.0);
            s.store_add(1182, 1179, 643);
            s.store_mul(1181, 1166, 1182);
            s.store_div_from_scalar_offset_ad(1182, 1.0, A::mul(s.ad_value(644), s.ad_value(1158)), 1.0);
            s.store_mul3_lhs(1302, 1180, 1181, 1182);
            s.store_add(1256, 1301, 1302);
            s.store_sub(1304, 1158, 1256);
            s.store_add_ad(1179, A::add(s.ad_value(637), A::mul(s.ad_value(636), s.ad_value(1304))), A::mul(A::mul(s.ad_value(571), s.ad_value(1304)), s.ad_value(1304)));
        }

        s.b[1704] = (s.v[1179] < 1e-5);
        s.v[1704] = if s.b[1704] { 1.0 } else { 0.0 };

        if (((s.b[1696] && (!s.b[1697])) && (!s.b[1703])) && s.b[1704]) {
            s.store_scalar(1179, 1e-5);
        }

        s.b[1705] = ((s.v[1179] < (s.v[1304] / 100.0)) && (s.v[1304] > 0.0));
        s.v[1705] = if s.b[1705] { 1.0 } else { 0.0 };

        if (((s.b[1696] && (!s.b[1697])) && (!s.b[1703])) && s.b[1705]) {
            s.store_scale(1303, 570, 2.688117142e43);
        }

        s.b[1706] = ((s.v[1179] < ((-s.v[1304]) / 100.0)) && (s.v[1304] < 0.0));
        s.v[1706] = if s.b[1706] { 1.0 } else { 0.0 };

        if ((((s.b[1696] && (!s.b[1697])) && (!s.b[1703])) && (!s.b[1705])) && s.b[1706]) {
            s.store_scale(1303, 570, 3.720075976e-44);
        }

        if ((((s.b[1696] && (!s.b[1697])) && (!s.b[1703])) && (!s.b[1705])) && (!s.b[1706])) {
            s.store_mul_exp_ad_rhs(1303, 570, A::div(s.ad_value(1304), s.ad_value(1179)));
        }

        s.b[1707] = (s.v[1303] > 10.0);
        s.v[1707] = if s.b[1707] { 1.0 } else { 0.0 };

        if (((s.b[1696] && (!s.b[1697])) && (!s.b[1703])) && s.b[1707]) {
            s.store_scalar(1303, 10.0);
        }

        if ((s.b[1696] && (!s.b[1697])) && (!s.b[1703])) {
            s.copy_ad(1179, 1220);
            s.store_mul(1439, 1303, 1179);
        }

        if (s.b[1696] && (!s.b[1697])) {
            s.store_add_scaled_inputs(1179, 632, 1.0 / (s.v[1227]), 631, ((s.v[1227]) * (1.0 / (s.v[1227]))));
            s.store_mul_offset_ad_rhs(1438, 633, A::scale(A::offset(s.ad_value(771), (-1.0)), s.v[301]), 1.0);
        }

        s.b[1708] = (s.v[759] > 0.0);
        s.v[1708] = if s.b[1708] { 1.0 } else { 0.0 };

        if ((s.b[1696] && (!s.b[1697])) && s.b[1708]) {
            s.store_sub(1180, 1438, 1422);
        }

        if ((s.b[1696] && (!s.b[1697])) && (!s.b[1708])) {
            s.store_sub(1180, 1438, 1421);
        }

        if (s.b[1696] && (!s.b[1697])) {
            s.store_offset(1181, 635, (-1.0));
        }

        s.b[1709] = (s.v[1180] <= 0.0);
        s.v[1709] = if s.b[1709] { 1.0 } else { 0.0 };

        if ((s.b[1696] && (!s.b[1697])) && s.b[1709]) {
            s.store_scalar(1182, 0.0);
        }

        if ((s.b[1696] && (!s.b[1697])) && (!s.b[1709])) {
            s.store_mul_scaled_ad_rhs(1182, 634, -1.0, A::pow(s.ad_value(1180), s.ad_value(1181)));
        }

        s.b[1710] = (s.v[1182] > 100.0);
        s.v[1710] = if s.b[1710] { 1.0 } else { 0.0 };

        if ((s.b[1696] && (!s.b[1697])) && s.b[1710]) {
            s.store_scalar(1183, 2.688117142e43);
        }

        s.b[1711] = (s.v[1182] < (-100.0));
        s.v[1711] = if s.b[1711] { 1.0 } else { 0.0 };

        if (((s.b[1696] && (!s.b[1697])) && (!s.b[1710])) && s.b[1711]) {
            s.store_scalar(1183, 3.720075976e-44);
        }

        if (((s.b[1696] && (!s.b[1697])) && (!s.b[1710])) && (!s.b[1711])) {
            s.store_exp(1183, 1182);
        }

        if (s.b[1696] && (!s.b[1697])) {
            s.store_mul_ad_product_lhs(1440, A::mul(A::mul(s.ad_value(1179), s.ad_value(759)), s.ad_value(1268)), s.ad_value(1180), 1183);
            s.store_add(1243, 1439, 1440);
        }

        s.b[1712] = ((s.v[760] == 0.0) || (s.v[760] == 2.0));
        s.v[1712] = if s.b[1712] { 1.0 } else { 0.0 };

        if (s.b[1696] && s.b[1712]) {
            s.store_scalar(1242, 0.0);
        }

        s.b[1713] = (s.v[526] < 0.001);
        s.v[1713] = if s.b[1713] { 1.0 } else { 0.0 };

        s.b[1714] = (s.v[427] <= 0.001);
        s.v[1714] = if s.b[1714] { 1.0 } else { 0.0 };

        if (((s.b[1696] && (!s.b[1712])) && s.b[1713]) && s.b[1714]) {
            s.store_scalar(1179, (1.0 / 0.001));
        }

        if (((s.b[1696] && (!s.b[1712])) && s.b[1713]) && (!s.b[1714])) {
            s.store_scalar(1179, (1.0 / s.v[427]));
        }

        if ((s.b[1696] && (!s.b[1712])) && s.b[1713]) {
            s.store_mul(1242, 1234, 1179);
        }

        if ((s.b[1696] && (!s.b[1712])) && (!s.b[1713])) {
            s.store_div_ad_rhs(1242, 1234, A::offset(s.ad_value(526), s.v[427]));
        }

        if (!s.b[1696]) {
            s.store_scalar(1243, 0.0);
            s.store_scalar(1242, 0.0);
        }

        s.b[1715] = (s.v[66] > 1.0);
        s.v[1715] = if s.b[1715] { 1.0 } else { 0.0 };

        if s.b[1715] {
            s.store_mul(1188, 596, 409);
            s.store_mul(1179, 1188, 1215);
            s.store_mul_add_rhs(413, 595, 1179, 1420);
        }

        s.b[1716] = (s.v[39] != 1.0);
        s.v[1716] = if s.b[1716] { 1.0 } else { 0.0 };

        if (s.b[1715] && s.b[1716]) {
            s.store_scale(413, 413, s.v[39]);
        }

        s.b[1717] = (s.v[66] == 2.0);
        s.v[1717] = if s.b[1717] { 1.0 } else { 0.0 };

        if (s.b[1715] && s.b[1717]) {
            s.store_add(1190, 421, 413);
            s.store_div_ad_lhs(413, A::mul(s.ad_value(421), s.ad_value(413)), 1190);
        }

    }

    pub(super) fn stamp_transient_block_17(
        s: &mut Scratch,
    ) {
        if (!s.b[1715]) {
            s.store_scalar(413, 0.0);
        }

        s.b[1718] = (s.v[403] == 1.0);
        s.v[1718] = if s.b[1718] { 1.0 } else { 0.0 };

        if s.b[1718] {
            s.store_scalar(1222, 0.0);
            s.store_sub(1179, 1157, 736);
            s.store_sqrt_square_offset(1180, 1179, 0.0001);
            s.store_scaled_add(1360, 1179, 1180, 0.5);
            s.store_offset_mul(1179, 553, 1360, 1.0);
            s.store_mul_neg_lhs(1180, 554, 1154);
            s.store_add_ad_lhs(1181, A::div_from_scalar(1.0, s.ad_value(1179)), 1180);
            s.store_add_ad_rhs(1182, 1181, A::sqrt(A::offset(A::square(s.ad_value(1181)), 0.01)));
            s.store_scale(1183, 1430, 0.5);
            s.store_add_ad_lhs(1434, A::add(s.ad_value(1432), A::mul(s.ad_value(1182), s.ad_value(1183))), 422);
            s.store_sub(1179, 1156, 736);
            s.store_sqrt_square_offset(1180, 1179, 0.0001);
            s.store_scaled_add(1361, 1179, 1180, 0.5);
            s.store_offset_mul(1179, 553, 1361, 1.0);
            s.store_mul_neg_lhs(1180, 554, 1153);
            s.store_add_ad_lhs(1181, A::div_from_scalar(1.0, s.ad_value(1179)), 1180);
            s.store_add_ad_rhs(1182, 1181, A::sqrt(A::offset(A::square(s.ad_value(1181)), 0.01)));
            s.store_scale(1183, 1429, 0.5);
            s.store_add_ad_lhs(1433, A::add(s.ad_value(1431), A::mul(s.ad_value(1182), s.ad_value(1183))), 423);
        }

        if (!s.b[1718]) {
            s.copy_ad(1434, 422);
            s.copy_ad(1433, 423);
        }

        s.b[1719] = (s.v[403] == 2.0);
        s.v[1719] = if s.b[1719] { 1.0 } else { 0.0 };

        if s.b[1719] {
            s.store_scalar(1434, 0.0);
            s.store_scalar(1433, 0.0);
        }

        s.store_mul_sub_from_scalar_ad_rhs(1180, 1210, 1.0, A::div(A::mul(A::scale(s.ad_value(1195), 0.5), s.ad_value(1211)), s.ad_value(1225)));

        s.store_mul_scale_ad_lhs(438, A::neg(s.ad_value(757)), (s.v[689] * (s.v[39] * s.v[1227])), 1180);

        s.b[1720] = (s.v[39] != 1.0);
        s.v[1720] = if s.b[1720] { 1.0 } else { 0.0 };

        if s.b[1720] {
            s.store_scale(1220, 1220, s.v[39]);
            s.store_scale(1268, 1268, s.v[39]);
            s.store_scale(454, 454, s.v[39]);
            s.store_scale(1269, 1269, s.v[39]);
            s.store_scale(1270, 1270, s.v[39]);
            s.store_scale(1358, 1358, s.v[39]);
            s.store_scale(1359, 1359, s.v[39]);
            s.store_scale(1356, 1356, s.v[39]);
            s.store_scale(1357, 1357, s.v[39]);
            s.store_scale(1243, 1243, s.v[39]);
            s.store_scale(412, 412, s.v[39]);
            s.store_scale(1240, 1240, s.v[39]);
            s.store_scale(1241, 1241, s.v[39]);
        }

        s.store_scalar(439, (A::ddx_projection(&s.ad_value(1220), Some(9), None) * s.v[36]));

        s.b[1721] = (s.v[759] > 0.0);
        s.v[1721] = if s.b[1721] { 1.0 } else { 0.0 };

        if s.b[1721] {
            s.store_scalar(440, (A::ddx_projection(&s.ad_value(1220), Some(7), None) * s.v[36]));
        }

        if (!s.b[1721]) {
            s.store_scalar(440, (A::ddx_projection(&s.ad_value(1220), Some(8), None) * s.v[36]));
        }

        s.store_scalar(441, (A::ddx_projection(&s.ad_value(1220), Some(5), None) * s.v[36]));

        s.store_scale(1178, 757, ((((s.v[693] / s.v[59]) * s.v[39]) * s.v[692]) + s.v[62]));

        s.store_scale(1316, 757, (s.v[342] * ((((s.v[693] / s.v[59]) * s.v[39]) * s.v[726]) + s.v[62])));

        s.store_scale(1448, 757, s.v[63]);

        s.store_scale(1449, 757, (s.v[342] * s.v[63]));

        s.store_sub(1166, 1161, 1407);

        s.store_mul(1189, 1393, 1168);

        s.store_div_ad_lhs(1145, A::mul(s.ad_value(745), s.ad_value(1166)), 1189);

        s.store_mul3_lhs(1351, 1393, 724, 1168);

        s.store_mul3_lhs(1352, 1393, 725, 1168);

        s.b[1722] = (s.v[69] == 0.0);
        s.v[1722] = if s.b[1722] { 1.0 } else { 0.0 };

        s.b[1723] = ((s.v[1145] > (-100.0)) && (s.v[1145] < 100.0));
        s.v[1723] = if s.b[1723] { 1.0 } else { 0.0 };

        if (s.b[1722] && s.b[1723]) {
            s.store_mul_ad(1146, A::exp(s.ad_value(1145)), A::exp(s.ad_value(1145)));
            s.store_mul_exp_ad_rhs(1146, 1146, A::neg(A::div(s.ad_value(685), s.ad_value(1351))));
        }

        if (s.b[1722] && s.b[1723]) {
            s.store_mul_ad_rhs(1210, 1351, {
                if ((1.0 + s.v[1146]) > 1e-38) {
                    A::ln(A::offset(s.ad_value(1146), 1.0))
                } else {
                    A::neg(A::constant(87.49823353377374))
                }
            });
        }

        s.b[1724] = (s.v[63] > 0.0);
        s.v[1724] = if s.b[1724] { 1.0 } else { 0.0 };

        if ((s.b[1722] && s.b[1723]) && s.b[1724]) {
            s.store_mul_exp_ad_rhs(1450, 1146, A::div(A::div(A::neg(s.ad_value(781)), s.ad_value(1352)), A::square(s.ad_value(1168))));
        }

        if ((s.b[1722] && s.b[1723]) && s.b[1724]) {
            s.store_mul_ad_rhs(1451, 1352, {
                if ((1.0 + s.v[1450]) > 1e-38) {
                    A::ln(A::offset(s.ad_value(1450), 1.0))
                } else {
                    A::neg(A::constant(87.49823353377374))
                }
            });
        }

        s.b[1725] = (s.v[69] == 1.0);
        s.v[1725] = if s.b[1725] { 1.0 } else { 0.0 };

        s.b[1726] = ((s.v[1145] > (-100.0)) && (s.v[1145] < 100.0));
        s.v[1726] = if s.b[1726] { 1.0 } else { 0.0 };

        if (((!s.b[1722]) && s.b[1725]) && s.b[1726]) {
            s.store_exp_ad(1146, A::div(s.ad_value(1145), A::mul(s.ad_value(745), s.ad_value(724))));
            s.store_mul_exp_ad_rhs(1146, 1146, A::neg(A::div(s.ad_value(685), s.ad_value(1351))));
        }

        if (((!s.b[1722]) && s.b[1725]) && s.b[1726]) {
            s.store_mul_ad_rhs(1210, 1351, {
                if ((1.0 + s.v[1146]) > 1e-38) {
                    A::ln(A::offset(s.ad_value(1146), 1.0))
                } else {
                    A::neg(A::constant(87.49823353377374))
                }
            });
        }

        s.b[1727] = (s.v[63] > 0.0);
        s.v[1727] = if s.b[1727] { 1.0 } else { 0.0 };

        if ((((!s.b[1722]) && s.b[1725]) && s.b[1726]) && s.b[1727]) {
            s.store_mul_exp_ad_rhs(1450, 1146, A::div(A::div(A::neg(s.ad_value(781)), s.ad_value(1352)), A::square(s.ad_value(1168))));
        }

        if ((((!s.b[1722]) && s.b[1725]) && s.b[1726]) && s.b[1727]) {
            s.store_mul_ad_rhs(1451, 1352, {
                if ((1.0 + s.v[1450]) > 1e-38) {
                    A::ln(A::offset(s.ad_value(1450), 1.0))
                } else {
                    A::neg(A::constant(87.49823353377374))
                }
            });
        }

        if ((!s.b[1722]) && (!s.b[1725])) {
            s.store_div_ad_lhs(1145, A::mul(s.ad_value(749), A::sub(s.ad_value(1166), s.ad_value(685))), 1351);
            s.store_div_ad_lhs(1169, A::sub(s.ad_value(751), A::mul(A::sub_from_scalar(1.0, s.ad_value(749)), A::sub(s.ad_value(1166), s.ad_value(685)))), 1351);
        }

        s.b[1728] = (s.v[1145] > 100.0);
        s.v[1728] = if s.b[1728] { 1.0 } else { 0.0 };

        if (((!s.b[1722]) && (!s.b[1725])) && s.b[1728]) {
            s.store_sub(1210, 1166, 685);
        }

        s.b[1729] = (s.v[1169] > 100.0);
        s.v[1729] = if s.b[1729] { 1.0 } else { 0.0 };

        if ((((!s.b[1722]) && (!s.b[1725])) && (!s.b[1728])) && s.b[1729]) {
            s.store_div_ad_lhs(1179, A::sub(A::sub(s.ad_value(1166), s.ad_value(685)), s.ad_value(751)), 1351);
            s.store_exp(1146, 1179);
            s.store_mul_div_ad_lhs(1210, A::mul(s.ad_value(1168), s.ad_value(1473)), s.ad_value(757), 1146);
        }

        if ((((!s.b[1722]) && (!s.b[1725])) && (!s.b[1728])) && (!s.b[1729])) {
            s.store_exp(1146, 1145);
        }

        if ((((!s.b[1722]) && (!s.b[1725])) && (!s.b[1728])) && (!s.b[1729])) {
            s.store_mul_ad_rhs(1180, 1351, {
                if ((1.0 + s.v[1146]) > 1e-38) {
                    A::ln(A::offset(s.ad_value(1146), 1.0))
                } else {
                    A::neg(A::constant(87.49823353377374))
                }
            });
        }

        if ((((!s.b[1722]) && (!s.b[1725])) && (!s.b[1728])) && (!s.b[1729])) {
            s.store_mul_ad(1192, A::mul(A::div(A::neg(s.ad_value(757)), A::mul(s.ad_value(1168), s.ad_value(1473))), A::exp(s.ad_value(1169))), A::sub_from_scalar(1.0, s.ad_value(749)));
            s.store_sub_ad_rhs(1181, 749, A::div(A::mul(s.ad_value(1351), s.ad_value(1192)), A::sub_from_scalar(1.0, s.ad_value(749))));
            s.store_div(1210, 1180, 1181);
        }

        s.b[1730] = (s.v[63] > 0.0);
        s.v[1730] = if s.b[1730] { 1.0 } else { 0.0 };

        if (((!s.b[1722]) && (!s.b[1725])) && s.b[1730]) {
            s.store_div_ad_lhs(1452, A::mul(s.ad_value(749), A::sub(A::sub(s.ad_value(1166), s.ad_value(685)), s.ad_value(781))), 1352);
            s.store_div_ad_lhs(1453, A::sub(s.ad_value(751), A::mul(A::sub_from_scalar(1.0, s.ad_value(749)), A::sub(A::sub(s.ad_value(1166), s.ad_value(685)), s.ad_value(781)))), 1352);
        }

        s.b[1731] = (s.v[1452] > 100.0);
        s.v[1731] = if s.b[1731] { 1.0 } else { 0.0 };

        if ((((!s.b[1722]) && (!s.b[1725])) && s.b[1730]) && s.b[1731]) {
            s.store_sub_ad_lhs(1451, A::sub(s.ad_value(1166), s.ad_value(685)), 781);
        }

        s.b[1732] = (s.v[1453] > 100.0);
        s.v[1732] = if s.b[1732] { 1.0 } else { 0.0 };

        if (((((!s.b[1722]) && (!s.b[1725])) && s.b[1730]) && (!s.b[1731])) && s.b[1732]) {
            s.store_div_ad_lhs(1179, A::sub(A::sub(A::sub(s.ad_value(1166), s.ad_value(685)), s.ad_value(751)), s.ad_value(781)), 1352);
            s.store_exp(1450, 1179);
            s.store_mul_div_ad_lhs(1451, A::mul(s.ad_value(1168), s.ad_value(1473)), s.ad_value(757), 1450);
        }

        if (((((!s.b[1722]) && (!s.b[1725])) && s.b[1730]) && (!s.b[1731])) && (!s.b[1732])) {
            s.store_exp(1450, 1452);
        }

        if (((((!s.b[1722]) && (!s.b[1725])) && s.b[1730]) && (!s.b[1731])) && (!s.b[1732])) {
            s.store_mul_ad_rhs(1180, 1352, {
                if ((1.0 + s.v[1450]) > 1e-38) {
                    A::ln(A::offset(s.ad_value(1450), 1.0))
                } else {
                    A::neg(A::constant(87.49823353377374))
                }
            });
        }

        if (((((!s.b[1722]) && (!s.b[1725])) && s.b[1730]) && (!s.b[1731])) && (!s.b[1732])) {
            s.store_mul_ad(1192, A::mul(A::div(A::neg(s.ad_value(757)), A::mul(s.ad_value(1168), s.ad_value(1473))), A::exp(s.ad_value(1453))), A::sub_from_scalar(1.0, s.ad_value(749)));
            s.store_sub_ad_rhs(1181, 749, A::div(A::mul(s.ad_value(1352), s.ad_value(1192)), A::sub_from_scalar(1.0, s.ad_value(749))));
            s.store_div(1451, 1180, 1181);
        }

        s.copy_ad(1165, 1407);

        s.copy_ad(1164, 1388);

        s.copy_ad(1177, 1378);

        s.b[1733] = (s.v[88] == 2.0);
        s.v[1733] = if s.b[1733] { 1.0 } else { 0.0 };

        s.b[1734] = (s.v[57] == 2.0);
        s.v[1734] = if s.b[1734] { 1.0 } else { 0.0 };

        if (s.b[1733] && s.b[1734]) {
            s.store_scalar(1273, 0.0);
            s.store_scalar(1272, 0.0);
        }

        if (s.b[1733] && (!s.b[1734])) {
            s.store_add_ad_lhs(1162, A::sub(A::sub(s.ad_value(1165), s.ad_value(1277)), A::mul(s.ad_value(707), s.ad_value(1164))), 685);
            s.store_offset_add_ad(1149, A::sub(s.ad_value(1162), s.ad_value(1161)), s.ad_value(1177), (-0.08));
        }

        s.b[1735] = (s.v[1162] <= 0.0);
        s.v[1735] = if s.b[1735] { 1.0 } else { 0.0 };

        if ((s.b[1733] && (!s.b[1734])) && s.b[1735]) {
            s.store_sqrt_sub_ad(1179, A::square(s.ad_value(1149)), A::scale(s.ad_value(1162), (4.0 * 0.08)));
        }

        if ((s.b[1733] && (!s.b[1734])) && (!s.b[1735])) {
            s.store_sqrt_add_ad(1179, A::square(s.ad_value(1149)), A::scale(s.ad_value(1162), (4.0 * 0.08)));
        }

        if (s.b[1733] && (!s.b[1734])) {
            s.store_sub_ad_rhs(1148, 1162, A::scale(A::add(s.ad_value(1149), s.ad_value(1179)), 0.5));
            s.store_mul_sub_rhs(1273, 1316, 1148, 1162);
        }

        s.b[1736] = (((s.v[57] != 2.0) && (s.v[760] != 0.0)) && (s.v[63] > 0.0));
        s.v[1736] = if s.b[1736] { 1.0 } else { 0.0 };

        if ((s.b[1733] && (!s.b[1734])) && s.b[1736]) {
            s.store_add(1460, 1162, 781);
            s.store_scalar(1472, 0.08);
            s.store_sub_ad_lhs(1149, A::add(A::sub(s.ad_value(1460), s.ad_value(1458)), s.ad_value(1177)), 1472);
        }

        s.b[1737] = (s.v[1460] <= 0.0);
        s.v[1737] = if s.b[1737] { 1.0 } else { 0.0 };

        if (((s.b[1733] && (!s.b[1734])) && s.b[1736]) && s.b[1737]) {
            s.store_sqrt_sub_ad(1179, A::square(s.ad_value(1149)), A::mul(A::scale(s.ad_value(1472), 100.0), s.ad_value(1460)));
        }

        if (((s.b[1733] && (!s.b[1734])) && s.b[1736]) && (!s.b[1737])) {
            s.store_sqrt_add_ad(1179, A::square(s.ad_value(1149)), A::mul(A::scale(s.ad_value(1472), 100.0), s.ad_value(1460)));
        }

        if ((s.b[1733] && (!s.b[1734])) && s.b[1736]) {
            s.store_sub_ad_rhs(1461, 1460, A::scale(A::add(s.ad_value(1149), s.ad_value(1179)), 0.5));
            s.store_add_ad_rhs(1273, 1273, A::mul(s.ad_value(1449), A::sub(s.ad_value(1461), s.ad_value(1460))));
        }

        if (s.b[1733] && (!s.b[1734])) {
            s.store_scale(1179, 737, 0.5);
            s.store_sub_ad_lhs(1182, A::sub(A::sub(s.ad_value(1161), s.ad_value(1148)), s.ad_value(1177)), 1210);
        }

        s.b[1738] = (s.v[737] == 0.0);
        s.v[1738] = if s.b[1738] { 1.0 } else { 0.0 };

        if ((s.b[1733] && (!s.b[1734])) && s.b[1738]) {
            s.store_scalar(1180, 0.0);
        }

        s.b[1739] = (s.v[1182] < 0.0);
        s.v[1739] = if s.b[1739] { 1.0 } else { 0.0 };

        if (((s.b[1733] && (!s.b[1734])) && (!s.b[1738])) && s.b[1739]) {
            s.store_add_ad_rhs(1180, 1179, A::div(s.ad_value(1182), s.ad_value(737)));
        }

        if (((s.b[1733] && (!s.b[1734])) && (!s.b[1738])) && (!s.b[1739])) {
            s.store_sqrt_square_add(1180, 1179, 1182);
        }

        if (s.b[1733] && (!s.b[1734])) {
            s.store_mul_ad(1272, A::mul(s.ad_value(1316), s.ad_value(737)), A::sub(s.ad_value(1180), s.ad_value(1179)));
        }

        s.b[1740] = (((s.v[57] != 2.0) && (s.v[760] != 0.0)) && (s.v[63] > 0.0));
        s.v[1740] = if s.b[1740] { 1.0 } else { 0.0 };

        if ((s.b[1733] && (!s.b[1734])) && s.b[1740]) {
            s.store_sub_ad_lhs(1182, A::sub(A::sub(s.ad_value(1458), s.ad_value(1461)), s.ad_value(1177)), 1451);
        }

        s.b[1741] = (s.v[1182] < 0.0);
        s.v[1741] = if s.b[1741] { 1.0 } else { 0.0 };

    }

    pub(super) fn stamp_transient_block_18(
        s: &mut Scratch,
        p: &Parameters,
    ) {
        if (((s.b[1733] && (!s.b[1734])) && s.b[1740]) && s.b[1741]) {
            s.store_add_ad_rhs(1180, 1179, A::div(s.ad_value(1182), s.ad_value(737)));
        }

        if (((s.b[1733] && (!s.b[1734])) && s.b[1740]) && (!s.b[1741])) {
            s.store_sqrt_square_add(1180, 1179, 1182);
        }

        if ((s.b[1733] && (!s.b[1734])) && s.b[1740]) {
            s.store_add_ad_rhs(1272, 1272, A::mul(A::mul(s.ad_value(1449), s.ad_value(737)), A::sub(s.ad_value(1180), s.ad_value(1179))));
        }

        if s.b[1733] {
            s.store_scale(1229, 1196, s.v[694]);
            s.store_div(1226, 1210, 1229);
            s.store_offset_sub(1150, 1226, 1158, (-0.02));
            s.store_sqrt_add_ad(1179, A::square(s.ad_value(1150)), A::scale(s.ad_value(1226), (4.0 * 0.02)));
            s.store_sub_ad_rhs(1212, 1226, A::scale(A::add(s.ad_value(1150), s.ad_value(1179)), 0.5));
        }

        s.b[1742] = (s.v[63] > 0.0);
        s.v[1742] = if s.b[1742] { 1.0 } else { 0.0 };

        if (s.b[1733] && s.b[1742]) {
            s.store_div(1462, 1451, 1229);
            s.store_offset_sub(1150, 1462, 1158, (-0.02));
            s.store_sqrt_add_ad(1179, A::square(s.ad_value(1150)), A::scale(s.ad_value(1462), (4.0 * 0.02)));
            s.store_sub_ad_rhs(1463, 1462, A::scale(A::add(s.ad_value(1150), s.ad_value(1179)), 0.5));
        }

        s.b[1743] = (s.v[57] == 2.0);
        s.v[1743] = if s.b[1743] { 1.0 } else { 0.0 };

        if (s.b[1733] && s.b[1743]) {
            s.store_scalar(1341, 0.0);
        }

        if (s.b[1733] && (!s.b[1743])) {
            s.store_mul(1179, 1229, 1212);
            s.store_scaled_offset_ad(1180, A::sub(s.ad_value(1210), A::scale(s.ad_value(1179), 0.5)), 1e-20, 12.0);
            s.store_div(1181, 1212, 1180);
            s.store_mul(1182, 1179, 1181);
            s.store_sub_from_scalar(1186, 1.0, 1229);
            s.store_mul_ad(1341, A::mul(s.ad_value(1316), s.ad_value(1186)), A::sub(A::scale(s.ad_value(1212), 0.5), s.ad_value(1182)));
        }

        s.b[1744] = (((s.v[57] != 2.0) && (s.v[760] != 0.0)) && (s.v[63] > 0.0));
        s.v[1744] = if s.b[1744] { 1.0 } else { 0.0 };

        if ((s.b[1733] && (!s.b[1743])) && s.b[1744]) {
            s.store_mul(1179, 1229, 1463);
            s.store_scaled_offset_ad(1180, A::sub(s.ad_value(1451), A::scale(s.ad_value(1179), 0.5)), 1e-20, 12.0);
            s.store_div(1181, 1463, 1180);
            s.store_mul(1182, 1179, 1181);
            s.store_sub_from_scalar(1186, 1.0, 1229);
            s.store_add_ad_rhs(1341, 1341, A::mul(A::mul(s.ad_value(1449), s.ad_value(1186)), A::sub(A::scale(s.ad_value(1463), 0.5), s.ad_value(1182))));
        }

        if s.b[1733] {
            s.store_mul(1179, 1229, 1212);
            s.store_scaled_offset_ad(1180, A::sub(s.ad_value(1210), A::scale(s.ad_value(1179), 0.5)), 1e-20, 12.0);
            s.store_div(1181, 1179, 1180);
            s.store_mul(1182, 1179, 1181);
            s.store_mul_add_ad_rhs(1250, 1178, A::sub(s.ad_value(1210), A::scale(s.ad_value(1179), 0.5)), s.ad_value(1182));
            s.store_neg(438, 1250);
        }

        s.b[1745] = (((s.v[57] != 2.0) && (s.v[760] != 0.0)) && (s.v[63] > 0.0));
        s.v[1745] = if s.b[1745] { 1.0 } else { 0.0 };

        if (s.b[1733] && s.b[1745]) {
            s.store_mul(1454, 1229, 1463);
            s.store_scaled_offset_ad(1191, A::sub(s.ad_value(1451), A::scale(s.ad_value(1454), 0.5)), 1e-20, 12.0);
            s.store_div(1181, 1454, 1191);
            s.store_mul(1182, 1454, 1181);
            s.store_add_ad_rhs(1250, 1250, A::mul(s.ad_value(1448), A::add(A::sub(s.ad_value(1451), A::scale(s.ad_value(1454), 0.5)), s.ad_value(1182))));
            s.store_neg(438, 1250);
        }

        s.b[1746] = (s.v[153] > 0.5);
        s.v[1746] = if s.b[1746] { 1.0 } else { 0.0 };

        if (s.b[1733] && s.b[1746]) {
            s.store_scale(1180, 1180, 2.0);
            s.store_mul_scaled_ad_rhs(1254, 1178, -1.0, A::sub(A::add(A::scale(s.ad_value(1210), 0.5), A::scale(s.ad_value(1179), 0.25)), A::div(A::square(s.ad_value(1179)), s.ad_value(1180))));
        }

        s.b[1747] = (((s.v[57] != 2.0) && (s.v[760] != 0.0)) && (s.v[63] > 0.0));
        s.v[1747] = if s.b[1747] { 1.0 } else { 0.0 };

        if ((s.b[1733] && s.b[1746]) && s.b[1747]) {
            s.store_scale(1191, 1191, 2.0);
            s.store_sub_ad_rhs(1254, 1254, A::mul(s.ad_value(1448), A::sub(A::add(A::scale(s.ad_value(1451), 0.5), A::scale(s.ad_value(1454), 0.25)), A::div(A::square(s.ad_value(1454)), s.ad_value(1191)))));
        }

        s.b[1748] = (s.v[153] < 0.5);
        s.v[1748] = if s.b[1748] { 1.0 } else { 0.0 };

        if ((s.b[1733] && (!s.b[1746])) && s.b[1748]) {
            s.store_scale(1180, 1180, 0.08333333333333333);
            s.store_div_ad(1181, A::scale(s.ad_value(1178), 0.5), A::square(s.ad_value(1180)));
            s.store_sub_ad(1182, A::mul(s.ad_value(1210), A::add(A::scale(A::mul(A::scale(s.ad_value(1179), 2.0), s.ad_value(1179)), 0.3333333333333333), A::mul(s.ad_value(1210), A::sub(s.ad_value(1210), A::scale(s.ad_value(1179), (4.0 * 0.3333333333333333)))))), A::scale(A::mul(A::mul(A::scale(s.ad_value(1179), 2.0), s.ad_value(1179)), s.ad_value(1179)), 0.06666666666666667));
            s.store_mul_neg_lhs(1254, 1181, 1182);
        }

        s.b[1749] = (((s.v[57] != 2.0) && (s.v[760] != 0.0)) && (s.v[63] > 0.0));
        s.v[1749] = if s.b[1749] { 1.0 } else { 0.0 };

        if (((s.b[1733] && (!s.b[1746])) && s.b[1748]) && s.b[1749]) {
            s.store_scale(1191, 1191, 0.08333333333333333);
            s.store_div_ad(1181, A::scale(s.ad_value(1448), 0.5), A::square(s.ad_value(1191)));
            s.store_sub_ad(1182, A::mul(s.ad_value(1451), A::add(A::scale(A::mul(A::scale(s.ad_value(1454), 2.0), s.ad_value(1454)), 0.3333333333333333), A::mul(s.ad_value(1451), A::sub(s.ad_value(1451), A::scale(s.ad_value(1454), (4.0 * 0.3333333333333333)))))), A::scale(A::mul(A::mul(A::scale(s.ad_value(1454), 2.0), s.ad_value(1454)), s.ad_value(1454)), 0.06666666666666667));
            s.store_mul_neg_lhs(1470, 1181, 1182);
            s.store_add(1254, 1254, 1470);
        }

        if ((s.b[1733] && (!s.b[1746])) && (!s.b[1748])) {
            s.store_scaled_add(1254, 1250, 1341, (-0.5));
        }

        s.b[1750] = (s.v[57] == 2.0);
        s.v[1750] = if s.b[1750] { 1.0 } else { 0.0 };

        if (s.b[1733] && s.b[1750]) {
            s.store_scalar(1274, 0.0);
        }

        if (s.b[1733] && (!s.b[1750])) {
            s.store_scale(1249, 626, (s.v[342] * (s.v[1248] * ((((s.v[693] / s.v[59]) * s.v[39]) * s.v[727]) + s.v[65]))));
            s.store_mul_sub_rhs(1274, 1249, 1237, 1160);
        }

        if s.b[1733] {
            s.store_add_ad_lhs(1251, A::add(s.ad_value(1250), s.ad_value(1273)), 1272);
            s.store_sub_ad_lhs(1252, A::sub(A::sub(s.ad_value(1341), s.ad_value(1273)), s.ad_value(1272)), 1274);
            s.copy_ad(1255, 1274);
            s.store_neg_ad(1253, A::add(A::add(A::add(s.ad_value(1251), s.ad_value(1254)), s.ad_value(1252)), s.ad_value(1255)));
        }

        s.b[1751] = (s.v[88] == 3.0);
        s.v[1751] = if s.b[1751] { 1.0 } else { 0.0 };

        s.b[1752] = (s.v[68] == 0.0);
        s.v[1752] = if s.b[1752] { 1.0 } else { 0.0 };

        if (((!s.b[1733]) && s.b[1751]) && s.b[1752]) {
            s.store_div_from_scalar(1332, 3.453133e-11, 92);
        }

        if (((!s.b[1733]) && s.b[1751]) && (!s.b[1752])) {
            s.store_scaled_div(1332, 777, 92, 8.85418e-12);
        }

        if ((!s.b[1733]) && s.b[1751]) {
            s.store_div_ad_lhs(1178, A::mul(s.ad_value(1178), s.ad_value(776)), 92);
            s.store_scaled_div(1316, 1316, 92, s.v[91]);
            s.store_scale(1333, 92, 100000000.0);
        }

        s.b[1753] = (s.v[63] > 0.0);
        s.v[1753] = if s.b[1753] { 1.0 } else { 0.0 };

        if (((!s.b[1733]) && s.b[1751]) && s.b[1753]) {
            s.store_scaled_div(1448, 1448, 92, s.v[91]);
            s.store_scaled_div(1449, 1449, 92, s.v[91]);
        }

        s.b[1754] = (s.v[57] == 2.0);
        s.v[1754] = if s.b[1754] { 1.0 } else { 0.0 };

        if (((!s.b[1733]) && s.b[1751]) && s.b[1754]) {
            s.store_scalar(1273, 0.0);
            s.store_scalar(1272, 0.0);
            s.store_scalar(1350, 0.0);
        }

        s.b[1755] = ((p.p33 == 1.0) && (p.p16 != 0.0));
        s.v[1755] = if s.b[1755] { 1.0 } else { 0.0 };

        if ((((!s.b[1733]) && s.b[1751]) && (!s.b[1754])) && s.b[1755]) {
            s.store_add_ad_lhs(1350, A::sub(A::sub(s.ad_value(1349), s.ad_value(1277)), A::mul(s.ad_value(707), s.ad_value(1278))), 685);
        }

        if ((((!s.b[1733]) && s.b[1751]) && (!s.b[1754])) && (!s.b[1755])) {
            s.store_add(1350, 424, 685);
        }

        if (((!s.b[1733]) && s.b[1751]) && (!s.b[1754])) {
            s.store_offset_add_ad(1149, A::sub(s.ad_value(1350), s.ad_value(1161)), s.ad_value(1177), (-0.02));
        }

        s.b[1756] = (s.v[1350] <= 0.0);
        s.v[1756] = if s.b[1756] { 1.0 } else { 0.0 };

        if ((((!s.b[1733]) && s.b[1751]) && (!s.b[1754])) && s.b[1756]) {
            s.store_sqrt_sub_ad(1179, A::square(s.ad_value(1149)), A::scale(s.ad_value(1350), (4.0 * 0.02)));
        }

        if ((((!s.b[1733]) && s.b[1751]) && (!s.b[1754])) && (!s.b[1756])) {
            s.store_sqrt_add_ad(1179, A::square(s.ad_value(1149)), A::scale(s.ad_value(1350), (4.0 * 0.02)));
        }

        if (((!s.b[1733]) && s.b[1751]) && (!s.b[1754])) {
            s.store_sub_ad_rhs(1148, 1350, A::scale(A::add(s.ad_value(1149), s.ad_value(1179)), 0.5));
        }

        s.b[1757] = (s.v[63] > 0.0);
        s.v[1757] = if s.b[1757] { 1.0 } else { 0.0 };

        if ((((!s.b[1733]) && s.b[1751]) && (!s.b[1754])) && s.b[1757]) {
            s.store_add(1459, 1350, 781);
            s.store_offset_add_ad(1149, A::sub(s.ad_value(1459), s.ad_value(1458)), s.ad_value(1177), (-0.02));
        }

        s.b[1758] = (s.v[1459] <= 0.0);
        s.v[1758] = if s.b[1758] { 1.0 } else { 0.0 };

        if (((((!s.b[1733]) && s.b[1751]) && (!s.b[1754])) && s.b[1757]) && s.b[1758]) {
            s.store_sqrt_sub_ad(1179, A::square(s.ad_value(1149)), A::scale(s.ad_value(1459), (100.0 * 0.02)));
        }

        if (((((!s.b[1733]) && s.b[1751]) && (!s.b[1754])) && s.b[1757]) && (!s.b[1758])) {
            s.store_sqrt_add_ad(1179, A::square(s.ad_value(1149)), A::scale(s.ad_value(1459), (100.0 * 0.02)));
        }

        if ((((!s.b[1733]) && s.b[1751]) && (!s.b[1754])) && s.b[1757]) {
            s.store_sub_ad_rhs(1461, 1459, A::scale(A::add(s.ad_value(1149), s.ad_value(1179)), 0.5));
        }

        if (((!s.b[1733]) && s.b[1751]) && (!s.b[1754])) {
            s.store_div_ad_lhs(1179, A::sub(A::sub(s.ad_value(1161), s.ad_value(1177)), s.ad_value(1350)), 1333);
            s.store_mul(1194, 1179, 722);
        }

        s.b[1759] = (((-100.0) < s.v[1194]) && (s.v[1194] < 100.0));
        s.v[1759] = if s.b[1759] { 1.0 } else { 0.0 };

        if ((((!s.b[1733]) && s.b[1751]) && (!s.b[1754])) && s.b[1759]) {
            s.store_mul_exp_rhs(1334, 721, 1194);
        }

        s.b[1760] = (s.v[1194] <= (-100.0));
        s.v[1760] = if s.b[1760] { 1.0 } else { 0.0 };

        if (((((!s.b[1733]) && s.b[1751]) && (!s.b[1754])) && (!s.b[1759])) && s.b[1760]) {
            s.store_scale(1334, 721, 3.720075976e-44);
        }

        if (((((!s.b[1733]) && s.b[1751]) && (!s.b[1754])) && (!s.b[1759])) && (!s.b[1760])) {
            s.store_scale(1334, 721, 2.688117142e43);
        }

        if (((!s.b[1733]) && s.b[1751]) && (!s.b[1754])) {
            s.store_scale(1335, 92, 0.001);
            s.store_sub_ad_lhs(1149, A::sub(s.ad_value(721), s.ad_value(1334)), 1335);
            s.store_sqrt_add_ad(1150, A::square(s.ad_value(1149)), A::mul(A::scale(s.ad_value(1335), 4.0), s.ad_value(721)));
            s.store_sub_ad_rhs(1334, 721, A::scale(A::add(s.ad_value(1149), s.ad_value(1150)), 0.5));
        }

        s.b[1761] = (s.v[1334] < 1e-15);
        s.v[1761] = if s.b[1761] { 1.0 } else { 0.0 };

        if ((((!s.b[1733]) && s.b[1751]) && (!s.b[1754])) && s.b[1761]) {
            s.store_scalar(1334, 1e-15);
        }

        s.b[1762] = (s.v[63] > 0.0);
        s.v[1762] = if s.b[1762] { 1.0 } else { 0.0 };

        if ((((!s.b[1733]) && s.b[1751]) && (!s.b[1754])) && s.b[1762]) {
            s.store_div_ad_lhs(1179, A::sub(A::sub(s.ad_value(1458), s.ad_value(1177)), s.ad_value(1459)), 1333);
            s.store_mul(1194, 1179, 722);
        }

        s.b[1763] = (((-100.0) < s.v[1194]) && (s.v[1194] < 100.0));
        s.v[1763] = if s.b[1763] { 1.0 } else { 0.0 };

        if (((((!s.b[1733]) && s.b[1751]) && (!s.b[1754])) && s.b[1762]) && s.b[1763]) {
            s.store_mul_exp_rhs(1464, 721, 1194);
        }

        s.b[1764] = (s.v[1194] <= (-100.0));
        s.v[1764] = if s.b[1764] { 1.0 } else { 0.0 };

        if ((((((!s.b[1733]) && s.b[1751]) && (!s.b[1754])) && s.b[1762]) && (!s.b[1763])) && s.b[1764]) {
            s.store_scale(1464, 721, 3.720075976e-44);
        }

        if ((((((!s.b[1733]) && s.b[1751]) && (!s.b[1754])) && s.b[1762]) && (!s.b[1763])) && (!s.b[1764])) {
            s.store_scale(1464, 721, 2.688117142e43);
        }

        if ((((!s.b[1733]) && s.b[1751]) && (!s.b[1754])) && s.b[1762]) {
            s.store_sub_ad_lhs(1149, A::sub(s.ad_value(721), s.ad_value(1464)), 1335);
            s.store_sqrt_add_ad(1150, A::square(s.ad_value(1149)), A::mul(A::scale(s.ad_value(1335), 4.0), s.ad_value(721)));
            s.store_sub_ad_rhs(1464, 721, A::scale(A::add(s.ad_value(1149), s.ad_value(1150)), 0.5));
        }

        s.b[1765] = (s.v[1464] < 1e-15);
        s.v[1765] = if s.b[1765] { 1.0 } else { 0.0 };

        if (((((!s.b[1733]) && s.b[1751]) && (!s.b[1754])) && s.b[1762]) && s.b[1765]) {
            s.store_scalar(1464, 1e-15);
        }

        if (((!s.b[1733]) && s.b[1751]) && (!s.b[1754])) {
            s.store_div(1336, 778, 1334);
            s.store_div_ad_rhs(1181, 1332, A::add(s.ad_value(1332), s.ad_value(1336)));
            s.store_mul(1337, 1181, 1336);
        }

        s.b[1766] = (((s.v[57] != 2.0) && (s.v[760] != 0.0)) && (s.v[63] > 0.0));
        s.v[1766] = if s.b[1766] { 1.0 } else { 0.0 };

        if ((((!s.b[1733]) && s.b[1751]) && (!s.b[1754])) && s.b[1766]) {
            s.store_div(1465, 778, 1464);
            s.store_div_ad_rhs(1181, 1332, A::add(s.ad_value(1332), s.ad_value(1465)));
            s.store_mul(1466, 1181, 1465);
        }

        if (((!s.b[1733]) && s.b[1751]) && (!s.b[1754])) {
            s.store_div_ad_lhs(1317, A::mul(s.ad_value(1316), s.ad_value(1337)), 1332);
        }

        s.b[1767] = (s.v[63] > 0.0);
        s.v[1767] = if s.b[1767] { 1.0 } else { 0.0 };

        if ((((!s.b[1733]) && s.b[1751]) && (!s.b[1754])) && s.b[1767]) {
            s.store_div_ad_lhs(1468, A::mul(s.ad_value(1449), s.ad_value(1466)), 1332);
        }

        if (((!s.b[1733]) && s.b[1751]) && (!s.b[1754])) {
            s.store_mul_sub_rhs(1273, 1317, 1148, 1350);
        }

        s.b[1768] = (((s.v[57] != 2.0) && (s.v[760] != 0.0)) && (s.v[63] > 0.0));
        s.v[1768] = if s.b[1768] { 1.0 } else { 0.0 };

    }

    pub(super) fn stamp_transient_block_19(
        s: &mut Scratch,
    ) {
        if ((((!s.b[1733]) && s.b[1751]) && (!s.b[1754])) && s.b[1768]) {
            s.store_mul_sub_rhs(1456, 1468, 1461, 1459);
            s.store_add(1273, 1273, 1456);
        }

        if (((!s.b[1733]) && s.b[1751]) && (!s.b[1754])) {
            s.store_scale(1179, 737, 0.5);
            s.store_sub_ad_lhs(1182, A::sub(A::sub(s.ad_value(1161), s.ad_value(1148)), s.ad_value(1177)), 1210);
        }

        s.b[1769] = (s.v[737] == 0.0);
        s.v[1769] = if s.b[1769] { 1.0 } else { 0.0 };

        if ((((!s.b[1733]) && s.b[1751]) && (!s.b[1754])) && s.b[1769]) {
            s.store_scalar(1180, 0.0);
        }

        s.b[1770] = (s.v[1182] < 0.0);
        s.v[1770] = if s.b[1770] { 1.0 } else { 0.0 };

        if (((((!s.b[1733]) && s.b[1751]) && (!s.b[1754])) && (!s.b[1769])) && s.b[1770]) {
            s.store_add_ad_rhs(1180, 1179, A::div(s.ad_value(1182), s.ad_value(737)));
        }

        if (((((!s.b[1733]) && s.b[1751]) && (!s.b[1754])) && (!s.b[1769])) && (!s.b[1770])) {
            s.store_sqrt_square_add(1180, 1179, 1182);
        }

        if (((!s.b[1733]) && s.b[1751]) && (!s.b[1754])) {
            s.store_mul_ad(1272, A::mul(s.ad_value(1317), s.ad_value(737)), A::sub(s.ad_value(1180), s.ad_value(1179)));
        }

        s.b[1771] = (((s.v[57] != 2.0) && (s.v[760] != 0.0)) && (s.v[63] > 0.0));
        s.v[1771] = if s.b[1771] { 1.0 } else { 0.0 };

        if ((((!s.b[1733]) && s.b[1751]) && (!s.b[1754])) && s.b[1771]) {
            s.store_sub_ad_lhs(1182, A::sub(A::sub(s.ad_value(1458), s.ad_value(1461)), s.ad_value(1177)), 1451);
        }

        s.b[1772] = (s.v[737] == 0.0);
        s.v[1772] = if s.b[1772] { 1.0 } else { 0.0 };

        if (((((!s.b[1733]) && s.b[1751]) && (!s.b[1754])) && s.b[1771]) && s.b[1772]) {
            s.store_scalar(1180, 0.0);
        }

        s.b[1773] = (s.v[1182] < 0.0);
        s.v[1773] = if s.b[1773] { 1.0 } else { 0.0 };

        if ((((((!s.b[1733]) && s.b[1751]) && (!s.b[1754])) && s.b[1771]) && (!s.b[1772])) && s.b[1773]) {
            s.store_add_ad_rhs(1180, 1179, A::div(s.ad_value(1182), s.ad_value(737)));
        }

        if ((((((!s.b[1733]) && s.b[1751]) && (!s.b[1754])) && s.b[1771]) && (!s.b[1772])) && (!s.b[1773])) {
            s.store_sqrt_square_add(1180, 1179, 1182);
        }

        if ((((!s.b[1733]) && s.b[1751]) && (!s.b[1754])) && s.b[1771]) {
            s.store_mul_ad(1457, A::mul(s.ad_value(1468), s.ad_value(737)), A::sub(s.ad_value(1180), s.ad_value(1179)));
            s.store_add(1272, 1272, 1457);
        }

        s.b[1774] = (s.v[737] <= 0.0);
        s.v[1774] = if s.b[1774] { 1.0 } else { 0.0 };

        if (((!s.b[1733]) && s.b[1751]) && s.b[1774]) {
            s.store_scaled_mul(1271, 723, 1168, 0.25);
            s.store_scale(1179, 700, 0.5);
        }

        if (((!s.b[1733]) && s.b[1751]) && (!s.b[1774])) {
            s.store_mul_ad_product_lhs(1271, A::mul(s.ad_value(723), s.ad_value(1168)), s.ad_value(737), 737);
            s.store_mul(1179, 737, 700);
        }

        if ((!s.b[1733]) && s.b[1751]) {
            s.store_add_ad_lhs(1180, A::scale(s.ad_value(1179), 2.0), 1210);
        }

        if ((!s.b[1733]) && s.b[1751]) {
            s.store_mul_ad_rhs(1339, 1168, {
                if ((1.0 + ((s.v[1180] * s.v[1210]) / s.v[1271])) > 1e-38) {
                    A::ln(A::offset(A::div(A::mul(s.ad_value(1180), s.ad_value(1210)), s.ad_value(1271)), 1.0))
                } else {
                    A::neg(A::constant(87.49823353377374))
                }
            });
        }

        s.b[1775] = (s.v[63] > 0.0);
        s.v[1775] = if s.b[1775] { 1.0 } else { 0.0 };

        if (((!s.b[1733]) && s.b[1751]) && s.b[1775]) {
            s.store_add_ad_lhs(1180, A::scale(s.ad_value(1179), 2.0), 1451);
        }

        if (((!s.b[1733]) && s.b[1751]) && s.b[1775]) {
            s.store_mul_ad_rhs(1469, 1168, {
                if ((1.0 + ((s.v[1180] * s.v[1451]) / s.v[1271])) > 1e-38) {
                    A::ln(A::offset(A::div(A::mul(s.ad_value(1180), s.ad_value(1451)), s.ad_value(1271)), 1.0))
                } else {
                    A::neg(A::constant(87.49823353377374))
                }
            });
        }

        if ((!s.b[1733]) && s.b[1751]) {
            s.store_scaled_sub_ad_lhs(1182, A::sub(s.ad_value(1165), s.ad_value(1350)), 1277, 4.0);
            s.store_sqrt_square_offset(1181, 1182, 0.0001);
            s.store_scaled_add(1183, 1182, 1181, 0.5);
            s.store_scale(1333, 1333, 2.0);
            s.store_div_ad_lhs(1179, A::add(s.ad_value(1210), s.ad_value(1183)), 1333);
        }

        if ((!s.b[1733]) && s.b[1751]) {
            s.store_exp_ad(1194, A::scale({
                if (s.v[1179] > 1e-38) {
                    A::ln(s.ad_value(1179))
                } else {
                    A::neg(A::constant(87.49823353377374))
                }
            }, (s.v[86] * 0.7)));
        }

        if ((!s.b[1733]) && s.b[1751]) {
            s.store_offset(1180, 1194, 1.0);
            s.store_div_from_scalar(1334, (s.v[85] * 1.9e-9), 1180);
            s.store_div(1336, 778, 1334);
            s.store_div_ad_rhs(1179, 1332, A::add(s.ad_value(1332), s.ad_value(1336)));
            s.store_mul(1337, 1179, 1336);
            s.store_div_ad_lhs(1338, A::mul(s.ad_value(1178), s.ad_value(1337)), 1332);
            s.store_div_ad_lhs(1317, A::mul(s.ad_value(1316), s.ad_value(1337)), 1332);
        }

        s.b[1776] = (((s.v[57] != 2.0) && (s.v[760] != 0.0)) && (s.v[63] > 0.0));
        s.v[1776] = if s.b[1776] { 1.0 } else { 0.0 };

        if (((!s.b[1733]) && s.b[1751]) && s.b[1776]) {
            s.store_scaled_sub_ad_lhs(1182, A::sub(A::add(s.ad_value(1165), s.ad_value(781)), s.ad_value(1459)), 1277, 4.0);
            s.store_sqrt_square_offset(1181, 1182, 0.0001);
            s.store_scaled_add(1183, 1182, 1181, 0.5);
            s.store_div_ad_lhs(1179, A::add(s.ad_value(1451), s.ad_value(1183)), 1333);
        }

        if (((!s.b[1733]) && s.b[1751]) && s.b[1776]) {
            s.store_exp_ad(1194, A::scale({
                if (s.v[1179] > 1e-38) {
                    A::ln(s.ad_value(1179))
                } else {
                    A::neg(A::constant(87.49823353377374))
                }
            }, (s.v[86] * 0.7)));
        }

        if (((!s.b[1733]) && s.b[1751]) && s.b[1776]) {
            s.store_offset(1180, 1194, 1.0);
            s.store_div_from_scalar(1464, (s.v[85] * 1.9e-9), 1180);
            s.store_div(1465, 778, 1464);
            s.store_div_ad_rhs(1179, 1332, A::add(s.ad_value(1332), s.ad_value(1465)));
            s.store_mul(1466, 1179, 1465);
            s.store_div_ad_lhs(1467, A::mul(s.ad_value(1448), s.ad_value(1466)), 1332);
            s.store_div_ad_lhs(1468, A::mul(s.ad_value(1449), s.ad_value(1466)), 1332);
        }

        if ((!s.b[1733]) && s.b[1751]) {
            s.store_sub(1180, 1210, 1339);
            s.store_scale(1229, 1196, s.v[694]);
            s.store_div(1226, 1180, 1229);
            s.store_offset_sub(1150, 1226, 1158, (-0.02));
            s.store_sqrt_add_ad(1179, A::square(s.ad_value(1150)), A::scale(s.ad_value(1226), (4.0 * 0.02)));
            s.store_sub_ad_rhs(1212, 1226, A::scale(A::add(s.ad_value(1150), s.ad_value(1179)), 0.5));
            s.store_mul(1179, 1229, 1212);
            s.store_scaled_offset_ad(1181, A::sub(s.ad_value(1180), A::scale(s.ad_value(1179), 0.5)), 1e-20, 12.0);
            s.store_div(1182, 1179, 1181);
            s.store_mul_sub_ad_rhs(1250, 1338, s.ad_value(1180), A::mul(s.ad_value(1179), A::sub_from_scalar(0.5, s.ad_value(1182))));
            s.copy_ad(1340, 1250);
            s.copy_ad(1251, 1250);
        }

        s.b[1777] = (((s.v[57] != 2.0) && (s.v[760] != 0.0)) && (s.v[63] > 0.0));
        s.v[1777] = if s.b[1777] { 1.0 } else { 0.0 };

        if (((!s.b[1733]) && s.b[1751]) && s.b[1777]) {
            s.store_sub(1191, 1451, 1469);
            s.store_div(1462, 1191, 1229);
            s.store_offset_sub(1150, 1462, 1158, (-0.02));
            s.store_sqrt_add_ad(1454, A::square(s.ad_value(1150)), A::scale(s.ad_value(1462), (4.0 * 0.02)));
            s.store_sub_ad_rhs(1463, 1462, A::scale(A::add(s.ad_value(1150), s.ad_value(1454)), 0.5));
            s.store_mul(1454, 1229, 1463);
            s.store_scaled_offset_ad(1455, A::sub(s.ad_value(1191), A::scale(s.ad_value(1454), 0.5)), 1e-20, 12.0);
            s.store_div(1182, 1454, 1455);
            s.store_mul_sub_ad_rhs(1186, 1467, s.ad_value(1191), A::mul(s.ad_value(1454), A::sub_from_scalar(0.5, s.ad_value(1182))));
            s.store_add(1250, 1250, 1186);
            s.copy_ad(1340, 1250);
            s.copy_ad(1251, 1250);
        }

        s.b[1778] = (s.v[57] == 2.0);
        s.v[1778] = if s.b[1778] { 1.0 } else { 0.0 };

        if (((!s.b[1733]) && s.b[1751]) && s.b[1778]) {
            s.store_scalar(1341, 0.0);
        }

        if (((!s.b[1733]) && s.b[1751]) && (!s.b[1778])) {
            s.store_sub_from_scalar(1186, 1.0, 1229);
            s.store_mul_ad(1341, A::mul(s.ad_value(1317), s.ad_value(1186)), A::sub(A::scale(s.ad_value(1212), 0.5), A::div(A::mul(s.ad_value(1179), s.ad_value(1212)), s.ad_value(1181))));
        }

        s.b[1779] = (((s.v[57] != 2.0) && (s.v[760] != 0.0)) && (s.v[63] > 0.0));
        s.v[1779] = if s.b[1779] { 1.0 } else { 0.0 };

        if ((((!s.b[1733]) && s.b[1751]) && (!s.b[1778])) && s.b[1779]) {
            s.store_mul_ad(1471, A::mul(s.ad_value(1468), s.ad_value(1186)), A::sub(A::scale(s.ad_value(1463), 0.5), A::div(A::mul(s.ad_value(1454), s.ad_value(1463)), s.ad_value(1455))));
            s.store_add(1341, 1341, 1471);
        }

        s.b[1780] = (s.v[153] > 0.5);
        s.v[1780] = if s.b[1780] { 1.0 } else { 0.0 };

        if (((!s.b[1733]) && s.b[1751]) && s.b[1780]) {
            s.store_mul_scaled_ad_rhs(1254, 1338, -1.0, A::sub(A::add(A::scale(s.ad_value(1180), 0.5), A::scale(s.ad_value(1179), 0.25)), A::div(A::mul(A::scale(s.ad_value(1179), 0.5), s.ad_value(1179)), s.ad_value(1181))));
        }

        s.b[1781] = (((s.v[57] != 2.0) && (s.v[760] != 0.0)) && (s.v[63] > 0.0));
        s.v[1781] = if s.b[1781] { 1.0 } else { 0.0 };

        if ((((!s.b[1733]) && s.b[1751]) && s.b[1780]) && s.b[1781]) {
            s.store_mul_scaled_ad_rhs(1470, 1467, -1.0, A::sub(A::add(A::scale(A::sub(s.ad_value(1451), s.ad_value(1469)), 0.5), A::scale(s.ad_value(1454), 0.25)), A::div(A::mul(A::scale(s.ad_value(1454), 0.5), s.ad_value(1454)), s.ad_value(1455))));
            s.store_add(1254, 1254, 1470);
        }

        s.b[1782] = (s.v[153] < 0.5);
        s.v[1782] = if s.b[1782] { 1.0 } else { 0.0 };

        if ((((!s.b[1733]) && s.b[1751]) && (!s.b[1780])) && s.b[1782]) {
            s.store_scale(1181, 1181, 0.08333333333333333);
            s.store_div_ad(1182, A::scale(s.ad_value(1338), 0.5), A::square(s.ad_value(1181)));
            s.store_sub_ad(1183, A::mul(s.ad_value(1180), A::add(A::scale(A::mul(A::scale(s.ad_value(1179), 2.0), s.ad_value(1179)), 0.3333333333333333), A::mul(s.ad_value(1180), A::sub(s.ad_value(1180), A::scale(s.ad_value(1179), (4.0 * 0.3333333333333333)))))), A::scale(A::mul(A::mul(A::scale(s.ad_value(1179), 2.0), s.ad_value(1179)), s.ad_value(1179)), 0.06666666666666667));
            s.store_mul_neg_lhs(1254, 1182, 1183);
        }

        s.b[1783] = (((s.v[57] != 2.0) && (s.v[760] != 0.0)) && (s.v[63] > 0.0));
        s.v[1783] = if s.b[1783] { 1.0 } else { 0.0 };

        if (((((!s.b[1733]) && s.b[1751]) && (!s.b[1780])) && s.b[1782]) && s.b[1783]) {
            s.store_scale(1455, 1455, 0.08333333333333333);
            s.store_div_ad(1182, A::scale(s.ad_value(1467), 0.5), A::square(s.ad_value(1455)));
            s.store_sub_ad(1183, A::mul(s.ad_value(1191), A::add(A::scale(A::mul(A::scale(s.ad_value(1454), 2.0), s.ad_value(1454)), 0.3333333333333333), A::mul(s.ad_value(1191), A::sub(s.ad_value(1191), A::scale(s.ad_value(1454), (4.0 * 0.3333333333333333)))))), A::scale(A::mul(A::mul(A::scale(s.ad_value(1454), 2.0), s.ad_value(1454)), s.ad_value(1454)), 0.06666666666666667));
            s.store_mul_neg_lhs(1470, 1182, 1183);
            s.store_add(1254, 1254, 1470);
        }

        if ((((!s.b[1733]) && s.b[1751]) && (!s.b[1780])) && (!s.b[1782])) {
            s.store_scale(1254, 1251, (-0.5));
        }

        s.b[1784] = (s.v[57] == 2.0);
        s.v[1784] = if s.b[1784] { 1.0 } else { 0.0 };

        if (((!s.b[1733]) && s.b[1751]) && s.b[1784]) {
            s.store_scalar(1274, 0.0);
        }

        if (((!s.b[1733]) && s.b[1751]) && (!s.b[1784])) {
            s.store_scale(1249, 626, (s.v[342] * (s.v[1248] * ((((s.v[693] / s.v[59]) * s.v[39]) * s.v[727]) + s.v[65]))));
            s.store_mul_sub_rhs(1274, 1249, 1237, 1160);
        }

        if ((!s.b[1733]) && s.b[1751]) {
            s.store_sub_ad_lhs(1251, A::add(A::add(s.ad_value(1251), s.ad_value(1273)), s.ad_value(1272)), 1341);
            s.store_sub_ad_lhs(1252, A::sub(A::sub(s.ad_value(1341), s.ad_value(1273)), s.ad_value(1272)), 1274);
            s.copy_ad(1255, 1274);
            s.store_neg_ad(1253, A::add(A::add(A::add(s.ad_value(1251), s.ad_value(1252)), s.ad_value(1255)), s.ad_value(1254)));
            s.store_neg(438, 1340);
        }

        if ((!s.b[1733]) && (!s.b[1751])) {
            s.store_scalar(1273, 0.0);
            s.store_scalar(1272, 0.0);
            s.store_scalar(1255, 0.0);
            s.store_scalar(1252, 0.0);
            s.store_scalar(1254, 0.0);
            s.store_scalar(1253, 0.0);
            s.store_scalar(1251, 0.0);
        }

        s.b[1785] = (s.v[57] == 2.0);
        s.v[1785] = if s.b[1785] { 1.0 } else { 0.0 };

        if s.b[1785] {
            s.store_scalar(1244, 0.0);
            s.store_scalar(1245, 0.0);
        }

        if (!s.b[1785]) {
            s.copy_ad(1151, 200);
            s.store_scalar(1315, (-s.v[344]));
            s.store_add_ad_rhs(1151, 1151, A::mul(s.ad_value(1315), A::offset(s.ad_value(769), (-s.v[150]))));
        }

    }

    pub(super) fn stamp_transient_block_20(
        s: &mut Scratch,
        p: &Parameters,
    ) {
        if (!s.b[1785]) {
            s.copy_ad(1152, 202);
            s.store_scalar(1311, ((((s.v[204] * s.v[711]) * s.v[174]) * s.v[39]) / 1e-7));
            s.store_scale(1314, 1311, s.v[343]);
            s.store_add_ad_rhs(1311, 1311, A::mul(s.ad_value(1314), A::offset(s.ad_value(769), (-s.v[150]))));
            s.store_scalar(1312, ((((s.v[205] * s.v[710]) * s.v[174]) * s.v[39]) / 1e-7));
            s.store_scale(1313, 1312, s.v[345]);
            s.store_add_ad_rhs(1312, 1312, A::mul(s.ad_value(1313), A::offset(s.ad_value(769), (-s.v[150]))));
            s.store_scale(1329, 1151, 0.9);
        }

        if (!s.b[1785]) {
            s.store_sub_from_scalar_ad(1147, 1.0, A::div({
                if (s.v[1421] > s.v[1329]) {
                    s.ad_value(1329)
                } else {
                    s.ad_value(1421)
                }
            }, s.ad_value(1151)));
        }

        s.b[1786] = (p.p173 == 0.5);
        s.v[1786] = if s.b[1786] { 1.0 } else { 0.0 };

        if ((!s.b[1785]) && s.b[1786]) {
            s.store_div_from_scalar_sqrt_ad(1193, 1.0, s.ad_value(1147));
        }

        if ((!s.b[1785]) && (!s.b[1786])) {
            s.store_exp_ad(1193, A::scale({
                if (s.v[1147] > 1e-38) {
                    A::ln(s.ad_value(1147))
                } else {
                    A::neg(A::constant(87.49823353377374))
                }
            }, (-p.p173)));
        }

        if (!s.b[1785]) {
            s.store_mul_scaled_ad_lhs(1182, A::sub_from_scalar(1.0, A::mul(s.ad_value(1147), s.ad_value(1193))), 1151, 1.0 / ((1.0 - p.p173)));
        }

        s.b[1787] = (s.v[1421] > s.v[1329]);
        s.v[1787] = if s.b[1787] { 1.0 } else { 0.0 };

        if ((!s.b[1785]) && s.b[1787]) {
            s.store_add_ad_rhs(1182, 1182, A::mul(s.ad_value(1193), A::sub(s.ad_value(1421), s.ad_value(1329))));
        }

        if (!s.b[1785]) {
            s.store_add_scaled_ad_lhs(1245, A::mul(s.ad_value(1311), s.ad_value(1182)), 1322, (s.v[332] * s.v[39]));
            s.copy_ad(1151, 201);
            s.store_scalar(1315, (-s.v[346]));
            s.store_add_ad_rhs(1151, 1151, A::mul(s.ad_value(1315), A::offset(s.ad_value(769), (-s.v[150]))));
            s.store_scalar(1152, s.v[203]);
            s.store_scale(1329, 1151, 0.9);
        }

        if (!s.b[1785]) {
            s.store_sub_from_scalar_ad(1147, 1.0, A::div({
                if (s.v[1422] > s.v[1329]) {
                    s.ad_value(1329)
                } else {
                    s.ad_value(1422)
                }
            }, s.ad_value(1151)));
        }

        s.b[1788] = (p.p173 == 0.5);
        s.v[1788] = if s.b[1788] { 1.0 } else { 0.0 };

        if ((!s.b[1785]) && s.b[1788]) {
            s.store_div_from_scalar_sqrt_ad(1193, 1.0, s.ad_value(1147));
        }

        if ((!s.b[1785]) && (!s.b[1788])) {
            s.store_exp_ad(1193, A::scale({
                if (s.v[1147] > 1e-38) {
                    A::ln(s.ad_value(1147))
                } else {
                    A::neg(A::constant(87.49823353377374))
                }
            }, (-p.p173)));
        }

        if (!s.b[1785]) {
            s.store_mul_scaled_ad_lhs(1182, A::sub_from_scalar(1.0, A::mul(s.ad_value(1147), s.ad_value(1193))), 1151, 1.0 / ((1.0 - p.p173)));
        }

        s.b[1789] = (s.v[1422] > s.v[1329]);
        s.v[1789] = if s.b[1789] { 1.0 } else { 0.0 };

        if ((!s.b[1785]) && s.b[1789]) {
            s.store_add_ad_rhs(1182, 1182, A::mul(s.ad_value(1193), A::sub(s.ad_value(1422), s.ad_value(1329))));
        }

        if (!s.b[1785]) {
            s.store_add_scaled_ad_lhs(1244, A::mul(s.ad_value(1312), s.ad_value(1182)), 1323, (s.v[332] * s.v[39]));
        }

        s.store_scale(1189, 1232, (-s.v[36]));

        s.store_scaled_sub(1190, 1155, 1232, s.v[36]);

        s.b[1790] = (s.v[336] != 0.0);
        s.v[1790] = if s.b[1790] { 1.0 } else { 0.0 };

        s.b[1791] = (((s.v[479] > 0.0) && (s.v[36] > 0.0)) || ((s.v[479] < 0.0) && (s.v[36] < 0.0)));
        s.v[1791] = if s.b[1791] { 1.0 } else { 0.0 };

        s.b[1792] = (s.v[1189] < s.v[683]);
        s.v[1792] = if s.b[1792] { 1.0 } else { 0.0 };

        if ((s.b[1790] && s.b[1791]) && s.b[1792]) {
            s.store_scaled_sub(448, 1189, 683, s.v[430]);
        }

        s.b[1793] = (s.v[1189] < s.v[545]);
        s.v[1793] = if s.b[1793] { 1.0 } else { 0.0 };

        if (((s.b[1790] && s.b[1791]) && (!s.b[1792])) && s.b[1793]) {
            s.store_sub(1179, 1189, 683);
            s.store_square(1180, 1179);
            s.store_mul_sub_from_scalar_ad_rhs(448, 1179, s.v[430], A::mul(A::scale(s.ad_value(546), 0.3333333333333333), s.ad_value(1180)));
        }

        s.b[1794] = (s.v[1189] < s.v[684]);
        s.v[1794] = if s.b[1794] { 1.0 } else { 0.0 };

        if ((((s.b[1790] && s.b[1791]) && (!s.b[1792])) && (!s.b[1793])) && s.b[1794]) {
            s.store_sub(1179, 1189, 684);
            s.store_square(1180, 1179);
            s.store_add_ad(448, A::add(A::mul(s.ad_value(432), s.ad_value(1189)), s.ad_value(434)), A::mul(A::mul(A::scale(s.ad_value(547), 0.3333333333333333), s.ad_value(1179)), s.ad_value(1180)));
        }

        if ((((s.b[1790] && s.b[1791]) && (!s.b[1792])) && (!s.b[1793])) && (!s.b[1794])) {
            s.store_add_ad_lhs(448, A::mul(s.ad_value(432), s.ad_value(1189)), 434);
        }

        s.b[1795] = (s.v[1189] < s.v[684]);
        s.v[1795] = if s.b[1795] { 1.0 } else { 0.0 };

        if ((s.b[1790] && (!s.b[1791])) && s.b[1795]) {
            s.store_mul_sub_rhs(448, 432, 1189, 684);
        }

        s.b[1796] = (s.v[1189] < s.v[545]);
        s.v[1796] = if s.b[1796] { 1.0 } else { 0.0 };

        if (((s.b[1790] && (!s.b[1791])) && (!s.b[1795])) && s.b[1796]) {
            s.store_sub(1179, 1189, 684);
            s.store_square(1180, 1179);
            s.store_mul_sub_ad_rhs(448, 1179, s.ad_value(432), A::mul(A::scale(s.ad_value(546), 0.3333333333333333), s.ad_value(1180)));
        }

        s.b[1797] = (s.v[1189] < s.v[683]);
        s.v[1797] = if s.b[1797] { 1.0 } else { 0.0 };

        if ((((s.b[1790] && (!s.b[1791])) && (!s.b[1795])) && (!s.b[1796])) && s.b[1797]) {
            s.store_sub(1179, 1189, 683);
            s.store_square(1180, 1179);
            s.store_add_ad(448, A::add(A::scale(s.ad_value(1189), s.v[430]), s.ad_value(434)), A::mul(A::mul(A::scale(s.ad_value(547), 0.3333333333333333), s.ad_value(1179)), s.ad_value(1180)));
        }

        if ((((s.b[1790] && (!s.b[1791])) && (!s.b[1795])) && (!s.b[1796])) && (!s.b[1797])) {
            s.store_add_ad_lhs(448, A::scale(s.ad_value(1189), s.v[430]), 434);
        }

        s.b[1798] = (((s.v[479] > 0.0) && (s.v[36] > 0.0)) || ((s.v[479] < 0.0) && (s.v[36] < 0.0)));
        s.v[1798] = if s.b[1798] { 1.0 } else { 0.0 };

        s.b[1799] = (s.v[1190] < s.v[683]);
        s.v[1799] = if s.b[1799] { 1.0 } else { 0.0 };

        if ((s.b[1790] && s.b[1798]) && s.b[1799]) {
            s.store_scaled_sub(449, 1190, 683, s.v[431]);
        }

        s.b[1800] = (s.v[1190] < s.v[545]);
        s.v[1800] = if s.b[1800] { 1.0 } else { 0.0 };

        if (((s.b[1790] && s.b[1798]) && (!s.b[1799])) && s.b[1800]) {
            s.store_sub(1179, 1190, 683);
            s.store_square(1180, 1179);
            s.store_mul_sub_from_scalar_ad_rhs(449, 1179, s.v[431], A::mul(A::scale(s.ad_value(548), 0.3333333333333333), s.ad_value(1180)));
        }

        s.b[1801] = (s.v[1190] < s.v[684]);
        s.v[1801] = if s.b[1801] { 1.0 } else { 0.0 };

        if ((((s.b[1790] && s.b[1798]) && (!s.b[1799])) && (!s.b[1800])) && s.b[1801]) {
            s.store_sub(1179, 1190, 684);
            s.store_square(1180, 1179);
            s.store_add_ad(449, A::add(A::mul(s.ad_value(433), s.ad_value(1190)), s.ad_value(435)), A::mul(A::mul(A::scale(s.ad_value(549), 0.3333333333333333), s.ad_value(1179)), s.ad_value(1180)));
        }

        if ((((s.b[1790] && s.b[1798]) && (!s.b[1799])) && (!s.b[1800])) && (!s.b[1801])) {
            s.store_add_ad_lhs(449, A::mul(s.ad_value(433), s.ad_value(1190)), 435);
        }

        s.b[1802] = (s.v[1190] < s.v[684]);
        s.v[1802] = if s.b[1802] { 1.0 } else { 0.0 };

        if ((s.b[1790] && (!s.b[1798])) && s.b[1802]) {
            s.store_mul_sub_rhs(449, 433, 1190, 684);
        }

        s.b[1803] = (s.v[1190] < s.v[545]);
        s.v[1803] = if s.b[1803] { 1.0 } else { 0.0 };

        if (((s.b[1790] && (!s.b[1798])) && (!s.b[1802])) && s.b[1803]) {
            s.store_sub(1179, 1190, 684);
            s.store_square(1180, 1179);
            s.store_mul_sub_ad_rhs(449, 1179, s.ad_value(433), A::mul(A::scale(s.ad_value(548), 0.3333333333333333), s.ad_value(1180)));
        }

        s.b[1804] = (s.v[1190] < s.v[683]);
        s.v[1804] = if s.b[1804] { 1.0 } else { 0.0 };

        if ((((s.b[1790] && (!s.b[1798])) && (!s.b[1802])) && (!s.b[1803])) && s.b[1804]) {
            s.store_sub(1179, 1190, 683);
            s.store_square(1180, 1179);
            s.store_add_ad(449, A::add(A::scale(s.ad_value(1190), s.v[431]), s.ad_value(435)), A::mul(A::mul(A::scale(s.ad_value(549), 0.3333333333333333), s.ad_value(1179)), s.ad_value(1180)));
        }

        if ((((s.b[1790] && (!s.b[1798])) && (!s.b[1802])) && (!s.b[1803])) && (!s.b[1804])) {
            s.store_add_ad_lhs(449, A::scale(s.ad_value(1190), s.v[431]), 435);
        }

        if (!s.b[1790]) {
            s.store_scale(448, 1189, s.v[430]);
            s.store_scale(449, 1190, s.v[431]);
        }

        s.store_add_ad_rhs(448, 448, A::mul(s.ad_value(428), s.ad_value(1189)));

        s.store_add_ad_rhs(449, 449, A::mul(s.ad_value(429), s.ad_value(1190)));

        s.b[1805] = (s.v[66] == 3.0);
        s.v[1805] = if s.b[1805] { 1.0 } else { 0.0 };

        if s.b[1805] {
            s.store_offset(1179, 1354, 0.02);
        }

        if (!s.b[1805]) {
            s.store_offset(1179, 1156, 0.02);
        }

        s.store_sqrt_square_offset(1180, 1179, (4.0 * 0.02));

        s.store_scaled_sub(1181, 1179, 1180, 0.5);

        s.store_scale(1182, 603, s.v[710]);

        s.store_sqrt_sub_from_scalar_ad(1183, 1.0, A::div(A::scale(s.ad_value(1181), 4.0), s.ad_value(604)));

        s.b[1806] = (s.v[66] == 3.0);
        s.v[1806] = if s.b[1806] { 1.0 } else { 0.0 };

        if s.b[1806] {
            s.store_sub_ad(1230, A::mul(A::add(s.ad_value(696), s.ad_value(1182)), s.ad_value(1354)), A::mul(s.ad_value(1182), A::add(s.ad_value(1181), A::mul(A::scale(s.ad_value(604), 0.5), A::offset(s.ad_value(1183), (-1.0))))));
        }

        if (!s.b[1806]) {
            s.store_sub_ad(1230, A::mul(A::add(s.ad_value(696), s.ad_value(1182)), s.ad_value(1156)), A::mul(s.ad_value(1182), A::add(s.ad_value(1181), A::mul(A::scale(s.ad_value(604), 0.5), A::offset(s.ad_value(1183), (-1.0))))));
        }

        s.b[1807] = (s.v[66] == 3.0);
        s.v[1807] = if s.b[1807] { 1.0 } else { 0.0 };

        if s.b[1807] {
            s.store_offset(1179, 1353, 0.02);
        }

        if (!s.b[1807]) {
            s.store_offset(1179, 1157, 0.02);
        }

        s.store_sqrt_square_offset(1180, 1179, (4.0 * 0.02));

        s.store_scaled_sub(1181, 1179, 1180, 0.5);

        s.store_scale(1182, 602, s.v[711]);

        s.store_sqrt_sub_from_scalar_ad(1183, 1.0, A::div(A::scale(s.ad_value(1181), 4.0), s.ad_value(604)));

        s.b[1808] = (s.v[66] == 3.0);
        s.v[1808] = if s.b[1808] { 1.0 } else { 0.0 };

        if s.b[1808] {
            s.store_sub_ad(1231, A::mul(A::add(s.ad_value(695), s.ad_value(1182)), s.ad_value(1353)), A::mul(s.ad_value(1182), A::add(s.ad_value(1181), A::mul(A::scale(s.ad_value(604), 0.5), A::offset(s.ad_value(1183), (-1.0))))));
        }

        if (!s.b[1808]) {
            s.store_sub_ad(1231, A::mul(A::add(s.ad_value(695), s.ad_value(1182)), s.ad_value(1157)), A::mul(s.ad_value(1182), A::add(s.ad_value(1181), A::mul(A::scale(s.ad_value(604), 0.5), A::offset(s.ad_value(1183), (-1.0))))));
        }

        s.b[1809] = (s.v[39] != 1.0);
        s.v[1809] = if s.b[1809] { 1.0 } else { 0.0 };

        if s.b[1809] {
            s.store_scale(1230, 1230, s.v[39]);
            s.store_scale(1231, 1231, s.v[39]);
        }

        s.copy_ad(798, 1251);

        s.store_add(797, 1231, 1230);

        s.store_add(1251, 798, 797);

        s.b[1821] = (s.v[759] > 0.0);
        s.v[1821] = if s.b[1821] { 1.0 } else { 0.0 };

        if s.b[1821] {
            s.store_abs_ad(1810, A::add(A::add(A::sub(A::add(s.ad_value(1220), s.ad_value(1268)), s.ad_value(1270)), s.ad_value(1243)), s.ad_value(1240)));
        }

        if (!s.b[1821]) {
            s.store_abs_ad(1810, A::add(A::add(A::sub(A::sub(s.ad_value(1220), s.ad_value(1268)), s.ad_value(1269)), s.ad_value(1243)), s.ad_value(1240)));
        }

        s.store_scale(773, 418, (4.0 * 1.3806503e-23));

        s.b[1822] = (s.v[403] != 2.0);
        s.v[1822] = if s.b[1822] { 1.0 } else { 0.0 };

        if s.b[1822] {
            s.store_div_from_scalar(774, 1.0, 1433);
            s.store_div_from_scalar(775, 1.0, 1434);
        }

        if (!s.b[1822]) {
            s.store_scalar(774, 0.0);
            s.store_scalar(775, 0.0);
        }

        s.b[1823] = (p.p213 == 0.0);
        s.v[1823] = if s.b[1823] { 1.0 } else { 0.0 };

        s.b[1824] = (p.p213 == 1.0);
        s.v[1824] = if s.b[1824] { 1.0 } else { 0.0 };

        s.b[1825] = (p.p213 == 3.0);
        s.v[1825] = if s.b[1825] { 1.0 } else { 0.0 };

        s.b[1826] = (p.p213 == 2.0);
        s.v[1826] = if s.b[1826] { 1.0 } else { 0.0 };

        if s.b[1823] {
            s.store_mul_scaled_ad_rhs(1815, 410, s.v[245], A::abs(A::div(s.ad_value(438), A::offset(A::mul(A::mul(s.ad_value(410), A::abs(s.ad_value(438))), s.ad_value(450)), (s.v[688] * s.v[688])))));
        }

        if (s.b[1824] && (!s.b[1823])) {
            s.store_add_ad_lhs(1179, A::add(s.ad_value(439), s.ad_value(440)), 441);
            s.store_square(1179, 1179);
            s.store_scaled_div(1817, 1281, 410, 2.0);
            s.store_scaled_div(1184, 451, 1817, (1.0 / (s.v[688])));
            s.store_square(1184, 1184);
            s.store_offset_scaled(1818, 1184, (((s.v[241] * s.v[688])) * (s.v[243])), s.v[243]);
            s.store_offset_scaled(1819, 1184, (((s.v[242] * s.v[688])) * (s.v[244])), s.v[244]);
        }

        s.b[1827] = (s.v[1819] > 0.9);
        s.v[1827] = if s.b[1827] { 1.0 } else { 0.0 };

        if ((s.b[1824] && (!s.b[1823])) && s.b[1827]) {
            s.store_scalar(1819, 0.9);
        }

        s.b[1828] = (s.v[1819] > (0.9 * s.v[1818]));
        s.v[1828] = if s.b[1828] { 1.0 } else { 0.0 };

        if ((s.b[1824] && (!s.b[1823])) && s.b[1828]) {
            s.store_scale(1819, 1818, 0.9);
        }

        if (s.b[1824] && (!s.b[1823])) {
            s.store_div_ad_lhs(1820, A::mul(A::square(s.ad_value(1819)), s.ad_value(1179)), 454);
            s.store_add_ad_lhs(1180, A::mul(s.ad_value(1818), A::add(s.ad_value(439), s.ad_value(441))), 440);
            s.store_div_ad_lhs(1181, A::square(s.ad_value(1180)), 454);
            s.store_sub(1815, 1181, 1820);
        }

        s.b[1829] = (s.v[759] > 0.0);
        s.v[1829] = if s.b[1829] { 1.0 } else { 0.0 };

    }

    pub(super) fn stamp_transient_block_21(
        s: &mut Scratch,
        p: &Parameters,
    ) {
        if ((s.b[1824] && (!s.b[1823])) && s.b[1829]) {
            s.store_mul_offset_ad_rhs(775, 775, A::div(A::mul(A::square(s.ad_value(1819)), s.ad_value(775)), s.ad_value(454)), 1.0);
        }

        if ((s.b[1824] && (!s.b[1823])) && (!s.b[1829])) {
            s.store_mul_offset_ad_rhs(774, 774, A::div(A::mul(A::square(s.ad_value(1819)), s.ad_value(774)), s.ad_value(454)), 1.0);
        }

        if (s.b[1826] && (!((s.b[1823] || s.b[1824]) || s.b[1825]))) {
            s.store_scaled_abs_ad(1815, A::add(A::add(s.ad_value(439), s.ad_value(440)), s.ad_value(441)), ((2.0 / 3.0) * s.v[245]));
        }

        s.v[1813] = (s.v[39] * s.v[689]);

        s.b[1830] = (s.v[264] == 1.0);
        s.v[1830] = if s.b[1830] { 1.0 } else { 0.0 };

        if s.b[1830] {
            s.store_scale(1814, 757, s.v[688]);
        }

        s.b[1831] = (s.v[264] == 2.0);
        s.v[1831] = if s.b[1831] { 1.0 } else { 0.0 };

        if ((!s.b[1830]) && s.b[1831]) {
            s.store_scale(1814, 757, (s.v[688] * s.v[688]));
        }

        if ((!s.b[1830]) && (!s.b[1831])) {
            s.store_scale(1814, 757, ((s.v[688]) as f64).powf(s.v[264]));
        }

        s.b[1832] = (p.p212 == 0.0);
        s.v[1832] = if s.b[1832] { 1.0 } else { 0.0 };

        s.b[1833] = (s.v[265] > 0.0);
        s.v[1833] = if s.b[1833] { 1.0 } else { 0.0 };

        if (s.b[1832] && s.b[1833]) {
            s.store_scale_ad(1811, A::scale(s.ad_value(1810), 1.0 / (s.v[1813])), s.v[265]);
        }

        s.b[1834] = (s.v[1811] < 1e-38);
        s.v[1834] = if s.b[1834] { 1.0 } else { 0.0 };

        if ((s.b[1832] && s.b[1833]) && s.b[1834]) {
            s.store_scalar(1811, 1e-38);
        }

        if (s.b[1832] && s.b[1833]) {
            s.store_ln(1812, 1811);
            s.store_div_ad_lhs(1816, A::scale(A::exp(A::scale(s.ad_value(1812), s.v[278])), ((s.v[1813] / s.v[265]) * s.v[279])), 1814);
        }

        s.b[1835] = (s.v[1810] < 1e-38);
        s.v[1835] = if s.b[1835] { 1.0 } else { 0.0 };

        if ((s.b[1832] && (!s.b[1833])) && s.b[1835]) {
            s.store_scalar(1811, 1e-38);
        }

        if ((s.b[1832] && (!s.b[1833])) && (!s.b[1835])) {
            s.copy_ad(1811, 1810);
        }

        if (s.b[1832] && (!s.b[1833])) {
            s.store_ln(1812, 1811);
            s.store_div_ad_lhs(1816, A::scale(A::exp(A::scale(s.ad_value(1812), s.v[278])), s.v[279]), 1814);
        }

        if (!s.b[1832]) {
            s.store_scalar(1841, ((1e-38) as f64).ln());
            s.store_scaled_div(1836, 1281, 410, 2.0);
        }

        s.b[1856] = (s.v[276] <= 0.0);
        s.v[1856] = if s.b[1856] { 1.0 } else { 0.0 };

        if ((!s.b[1832]) && s.b[1856]) {
            s.store_scalar(1837, 0.0);
        }

        if ((!s.b[1832]) && (!s.b[1856])) {
            s.store_div_ad_lhs(1842, A::offset(A::div(A::sub(s.ad_value(1158), s.ad_value(452)), s.ad_value(489)), s.v[276]), 1836);
        }

        s.b[1857] = (s.v[1842] < 1e-38);
        s.v[1857] = if s.b[1857] { 1.0 } else { 0.0 };

        if (((!s.b[1832]) && (!s.b[1856])) && s.b[1857]) {
            s.store_mul(1837, 489, 1841);
        }

        if (((!s.b[1832]) && (!s.b[1856])) && (!s.b[1857])) {
            s.store_mul_ln_rhs(1837, 489, 1842);
        }

        if (!s.b[1832]) {
            s.store_mul_ad_product_lhs(1843, A::scale(s.ad_value(1810), ((1.60219e-19 * 1.60219e-19) * 1.3806503e-23)), s.ad_value(418), 410);
            s.store_scaled_mul(1844, 437, 757, ((10000000000.0) * ((s.v[688] * s.v[688]))));
            s.store_scaled_mul(1838, 757, 451, 6.241457005723417e18);
            s.store_scaled_mul_ad(1839, A::mul(s.ad_value(757), s.ad_value(451)), A::sub_from_scalar(1.0, A::mul(s.ad_value(453), s.ad_value(452))), 6.241457005723417e18);
            s.store_div_ad(1840, A::add(s.ad_value(1838), s.ad_value(436)), A::add(s.ad_value(1839), s.ad_value(436)));
        }

        s.b[1858] = (s.v[1840] < 1e-38);
        s.v[1858] = if s.b[1858] { 1.0 } else { 0.0 };

        if ((!s.b[1832]) && s.b[1858]) {
            s.store_scale(1845, 1841, s.v[238]);
        }

        if ((!s.b[1832]) && (!s.b[1858])) {
            s.store_scaled_ln(1845, 1840, s.v[238]);
        }

        if (!s.b[1832]) {
            s.store_scaled_sub(1846, 1838, 1839, s.v[239]);
            s.store_scaled_sub_ad(1847, A::square(s.ad_value(1838)), A::square(s.ad_value(1839)), (s.v[240] * 0.5));
            s.store_mul_ad_product_lhs(1848, A::scale(s.ad_value(418), 1.3806503e-23), s.ad_value(1810), 1810);
            s.store_scalar(1849, (((10000000000.0 * s.v[688]) * s.v[688]) * s.v[1813]));
            s.store_add_ad(1850, A::offset(A::scale(s.ad_value(1839), s.v[239]), s.v[238]), A::mul(A::scale(s.ad_value(1839), s.v[240]), s.ad_value(1839)));
            s.store_mul_ad(1851, A::add(s.ad_value(1839), s.ad_value(436)), A::add(s.ad_value(1839), s.ad_value(436)));
            s.store_add_ad(1854, A::mul(A::div(s.ad_value(1843), s.ad_value(1844)), A::add(A::add(s.ad_value(1845), s.ad_value(1846)), s.ad_value(1847))), A::div(A::mul(A::mul(A::div(s.ad_value(1848), s.ad_value(1849)), s.ad_value(1837)), s.ad_value(1850)), s.ad_value(1851)));
            s.store_scale(1852, 418, (s.v[238] * 1.3806503e-23));
            s.store_scaled_square(1853, 436, ((s.v[1813] * s.v[688]) * 10000000000.0));
            s.store_mul_ad_product_lhs(1855, A::div(s.ad_value(1852), s.ad_value(1853)), s.ad_value(1810), 1810);
            s.store_add(1843, 1855, 1854);
        }

        s.b[1859] = (((s.v[1843] > 0.0) && (s.v[1854] > 0.0)) && (s.v[1855] > 0.0));
        s.v[1859] = if s.b[1859] { 1.0 } else { 0.0 };

        if ((!s.b[1832]) && s.b[1859]) {
            s.store_div_ad_lhs(1816, A::mul(s.ad_value(1854), s.ad_value(1855)), 1843);
        }

        if ((!s.b[1832]) && (!s.b[1859])) {
            s.store_scalar(1816, 0.0);
        }

        s.b[1860] = (s.v[403] != 2.0);
        s.v[1860] = if s.b[1860] { 1.0 } else { 0.0 };

        s.b[1861] = (s.v[759] > 0.0);
        s.v[1861] = if s.b[1861] { 1.0 } else { 0.0 };

        if s.b[1861] {
            s.store_scale(419, 1240, s.v[36]);
            s.store_scale(420, 1241, s.v[36]);
            s.store_scale(443, 1359, s.v[36]);
            s.store_scale(442, 1358, s.v[36]);
            s.store_scale(446, 1253, s.v[36]);
            s.store_scale(447, 1254, s.v[36]);
        }

        if (!s.b[1861]) {
            s.store_scale(420, 1240, s.v[36]);
            s.store_scale(419, 1241, s.v[36]);
            s.store_scale(442, 1359, s.v[36]);
            s.store_scale(443, 1358, s.v[36]);
            s.store_scale(447, 1253, s.v[36]);
            s.store_scale(446, 1254, s.v[36]);
        }

        s.store_scale(445, 1357, s.v[36]);

        s.store_scale(444, 1356, s.v[36]);

        s.b[1862] = ((s.v[760] == 0.0) || (s.v[760] == 2.0));
        s.v[1862] = if s.b[1862] { 1.0 } else { 0.0 };

        s.b[1863] = (p.p37 == 3.0);
        s.v[1863] = if s.b[1863] { 1.0 } else { 0.0 };

        s.b[1864] = ((p.p37 == 0.0) || (p.p37 == 2.0));
        s.v[1864] = if s.b[1864] { 1.0 } else { 0.0 };

        s.b[1865] = ((p.p37 == 0.0) || (p.p37 == 1.0));
        s.v[1865] = if s.b[1865] { 1.0 } else { 0.0 };

        s.b[1866] = (p.p37 == 2.0);
        s.v[1866] = if s.b[1866] { 1.0 } else { 0.0 };

        if ((!s.b[1865]) && s.b[1866]) {
            s.store_offset_div(1867, 421, 413, 1.0);
        }

        s.b[1868] = (s.v[57] == 2.0);
        s.v[1868] = if s.b[1868] { 1.0 } else { 0.0 };

        s.b[1869] = ((p.p33 == 1.0) && (p.p16 != 0.0));
        s.v[1869] = if s.b[1869] { 1.0 } else { 0.0 };

    }

    pub(super) fn stamp_reactive_block_0(
        ctx: &GeneratedEvalContext<'_>,
        s: &mut ReactiveScratch,
        p: &Parameters,
    ) {
        let ctx_temp = ctx.temperature();
        s.v[769] = (ctx_temp + p.p0);

        s.v[36] = p.p34;

        s.v[37] = p.p1;

        s.v[38] = p.p2;

        s.v[39] = p.p3;

        s.v[40] = p.p4;

        s.v[41] = p.p5;

        s.v[42] = p.p6;

        s.v[43] = p.p7;

        s.v[44] = p.p8;

        s.v[45] = p.p9;

        s.v[46] = p.p10;

        s.v[47] = p.p11;

        s.v[48] = p.p12;

        s.v[49] = p.p14;

        s.v[51] = p.p17;

        s.v[52] = p.p18;

        s.v[53] = p.p19;

        s.v[56] = p.p22;

        s.v[57] = p.p23;

        s.v[58] = p.p24;

        s.v[59] = p.p25;

        s.v[60] = p.p26;

        s.v[61] = p.p27;

        s.v[62] = p.p28;

        s.v[63] = p.p29;

        s.v[64] = p.p30;

        s.v[65] = p.p31;

        s.v[66] = p.p37;

        s.v[68] = p.p39;

        s.v[69] = p.p40;

        s.v[70] = p.p41;

        s.v[71] = p.p42;

        s.v[72] = p.p43;

        s.v[73] = p.p44;

        s.v[74] = p.p45;

        s.v[75] = p.p46;

        s.v[76] = p.p47;

        s.v[77] = p.p48;

        s.v[78] = p.p49;

        s.v[79] = p.p50;

        s.v[80] = p.p51;

        s.v[81] = p.p52;

        s.v[82] = p.p53;

        s.v[83] = p.p54;

        s.v[84] = p.p55;

        s.v[85] = p.p56;

        s.v[86] = p.p57;

        s.v[87] = p.p58;

        s.v[88] = p.p59;

        s.v[89] = p.p60;

        s.v[90] = p.p63;

        s.v[91] = p.p64;

        s.v[93] = p.p66;

        s.v[94] = p.p67;

        s.v[95] = p.p68;

        s.v[96] = p.p69;

        s.v[97] = p.p70;

        s.v[98] = p.p71;

        s.v[99] = p.p72;

        s.v[100] = p.p73;

        s.v[101] = p.p74;

        s.v[102] = p.p75;

        s.v[103] = p.p76;

        s.v[104] = p.p77;

        s.v[105] = p.p78;

        s.v[106] = p.p79;

        s.v[107] = p.p80;

        s.v[108] = p.p81;

        s.v[109] = p.p82;

        s.v[110] = p.p83;

        s.v[111] = p.p84;

        s.v[112] = p.p85;

        s.v[113] = p.p86;

        s.v[114] = p.p87;

        s.v[115] = p.p88;

        s.v[116] = p.p89;

        s.v[117] = p.p90;

        s.v[118] = p.p91;

        s.v[119] = p.p92;

        s.v[120] = p.p93;

        s.v[121] = p.p94;

        s.v[122] = p.p95;

        s.v[123] = p.p96;

        s.v[124] = p.p973;

        s.v[125] = p.p97;

        s.v[126] = p.p98;

        s.v[127] = p.p99;

        s.v[128] = p.p100;

        s.v[129] = p.p101;

        s.v[130] = p.p102;

        s.v[131] = p.p103;

        s.v[132] = p.p104;

        s.v[133] = p.p105;

        s.v[134] = p.p107;

        s.v[135] = p.p108;

        s.v[136] = p.p109;

        s.v[137] = p.p110;

        s.v[138] = p.p111;

        s.v[139] = p.p112;

        s.v[140] = p.p113;

        s.v[141] = p.p114;

        s.v[142] = p.p115;

        s.v[143] = p.p116;

        s.v[144] = p.p117;

        s.v[145] = p.p118;

        s.v[146] = p.p119;

        s.v[147] = p.p120;

        s.v[148] = p.p121;

        s.v[149] = p.p122;

        s.v[150] = (p.p123 + 273.15);

        s.v[153] = p.p126;

        s.v[154] = p.p127;

        s.v[155] = p.p128;

        s.v[156] = p.p129;

        s.v[157] = p.p130;

        s.v[158] = p.p131;

        s.v[159] = p.p132;

        s.v[160] = p.p133;

        s.v[161] = p.p134;

        s.v[162] = p.p135;

        s.v[163] = p.p136;

        s.v[164] = p.p137;

        s.v[165] = p.p138;

        s.v[166] = p.p139;

        s.v[167] = p.p140;

        s.v[168] = p.p141;

        s.v[169] = p.p142;

        s.v[170] = p.p143;

        s.v[171] = p.p144;

        s.v[172] = p.p145;

        s.v[173] = p.p146;

        s.v[174] = p.p147;

        s.v[175] = p.p148;

        s.v[176] = p.p149;

        s.v[177] = p.p974;

        s.v[178] = p.p150;

        s.v[179] = p.p151;

        s.v[180] = p.p152;

        s.v[181] = p.p153;

        s.v[182] = p.p154;

        s.v[183] = p.p155;

        s.v[184] = p.p975;

        s.v[185] = p.p156;

        s.v[186] = p.p157;

        s.v[187] = p.p158;

        s.v[188] = p.p159;

        s.v[189] = p.p160;

        s.v[190] = p.p161;

        s.v[191] = p.p162;

        s.v[192] = p.p163;

        s.v[193] = p.p164;

        s.v[194] = p.p165;

        s.v[195] = p.p166;

        s.v[196] = p.p167;

        s.v[197] = p.p168;

        s.v[198] = p.p169;

        s.v[199] = p.p170;

        s.v[200] = p.p171;

        s.v[201] = p.p172;

        s.copy_ad(202, 1152);

        s.v[203] = p.p174;

        s.v[204] = p.p175;

        s.v[205] = p.p176;

        s.v[206] = p.p177;

        s.v[207] = p.p178;

        s.v[208] = p.p179;

        s.v[209] = p.p180;

        s.v[210] = p.p181;

        s.v[211] = p.p182;

        s.v[212] = p.p183;

        s.v[213] = p.p184;

        s.v[214] = p.p185;

        s.v[215] = p.p186;

        s.v[216] = p.p187;

        s.v[217] = p.p188;

        s.v[218] = p.p189;

        s.v[219] = p.p190;

        s.v[220] = p.p191;

        s.v[221] = p.p192;

        s.v[222] = p.p193;

        s.v[223] = p.p194;

        s.v[224] = p.p195;

        s.v[225] = p.p196;

        s.v[226] = p.p197;

        s.v[227] = p.p198;

        s.v[228] = p.p199;

        s.v[229] = p.p200;

        s.v[230] = p.p201;

        s.v[231] = p.p202;

        s.v[233] = p.p204;

        s.v[234] = p.p205;

        s.v[235] = p.p206;

        s.v[236] = p.p207;

        s.v[237] = p.p208;

        s.v[241] = p.p214;

        s.v[243] = p.p216;

        s.v[246] = p.p219;

        s.v[247] = p.p220;

        s.v[248] = p.p221;

        s.v[249] = p.p222;

        s.v[250] = p.p223;

        s.v[251] = p.p224;

        s.v[252] = p.p225;

        s.v[253] = p.p226;

        s.v[254] = p.p227;

        s.v[255] = p.p228;

        s.v[256] = p.p229;

        s.v[257] = p.p236;

        s.v[258] = p.p237;

        s.v[259] = p.p238;

        s.v[260] = p.p239;

        s.v[261] = p.p240;

        s.v[262] = p.p241;

        s.v[266] = p.p245;

        s.v[267] = p.p249;

        s.v[268] = p.p253;

        s.v[269] = p.p257;

        s.v[270] = p.p261;

        s.v[271] = p.p265;

        s.v[272] = p.p269;

        s.v[273] = p.p270;

        s.v[274] = p.p271;

        s.v[275] = p.p272;

        s.v[281] = p.p287;

        s.v[282] = p.p288;

        s.v[283] = p.p289;

        s.v[284] = p.p290;

        s.v[285] = p.p291;

        s.v[286] = p.p292;

        s.v[287] = p.p293;

        s.v[288] = p.p294;

        s.v[289] = p.p295;

        s.v[290] = p.p296;

        s.v[291] = p.p297;

        s.v[292] = p.p298;

        s.v[293] = p.p299;

        s.v[294] = p.p300;

        s.v[295] = p.p301;

        s.v[296] = p.p302;

        s.v[297] = p.p303;

        s.v[298] = p.p304;

        s.v[299] = p.p305;

        s.v[300] = p.p306;

        s.v[301] = p.p307;

        s.v[302] = p.p308;

        s.v[303] = p.p309;

        s.v[304] = p.p310;

        s.v[305] = p.p311;

        s.v[306] = p.p312;

        s.v[307] = p.p313;

        s.v[308] = p.p314;

        s.v[309] = p.p315;

        s.v[310] = p.p316;

        s.v[311] = p.p317;

        s.v[312] = p.p318;

    }

    pub(super) fn stamp_reactive_block_1(
        s: &mut ReactiveScratch,
        p: &Parameters,
        param_given: &[bool; Instance::PARAMETER_COUNT],
    ) {
        s.v[313] = p.p319;

        s.v[314] = p.p320;

        s.v[315] = p.p321;

        s.v[316] = p.p322;

        s.v[317] = p.p323;

        s.v[318] = p.p324;

        s.v[319] = p.p325;

        s.v[320] = p.p326;

        s.v[321] = p.p327;

        s.v[322] = p.p328;

        s.v[323] = p.p329;

        s.v[324] = p.p330;

        s.v[325] = p.p331;

        s.v[326] = p.p332;

        s.v[327] = p.p333;

        s.v[328] = p.p334;

        s.v[329] = p.p335;

        s.v[330] = p.p336;

        s.v[331] = p.p337;

        s.v[332] = p.p338;

        s.v[333] = p.p339;

        s.v[334] = p.p340;

        s.v[335] = p.p341;

        s.v[336] = p.p342;

        s.v[337] = p.p343;

        s.v[338] = p.p344;

        s.v[339] = p.p345;

        s.v[340] = p.p346;

        s.v[341] = p.p347;

        s.v[342] = p.p348;

        s.v[343] = p.p349;

        s.v[344] = p.p350;

        s.v[345] = p.p351;

        s.v[346] = p.p352;

        s.v[347] = p.p353;

        s.v[348] = p.p354;

        s.v[349] = p.p355;

        s.v[350] = p.p356;

        s.v[351] = p.p357;

        s.v[352] = p.p358;

        s.v[353] = p.p359;

        s.v[354] = p.p360;

        s.v[355] = p.p362;

        s.v[356] = p.p363;

        s.v[357] = p.p364;

        s.v[358] = p.p365;

        s.v[359] = p.p366;

        s.v[360] = p.p367;

        s.v[361] = p.p368;

        s.v[362] = p.p369;

        s.v[363] = p.p370;

        s.v[364] = p.p371;

        s.v[365] = p.p372;

        s.v[366] = p.p373;

        s.v[367] = p.p374;

        s.v[368] = p.p375;

        s.v[369] = p.p376;

        s.v[370] = p.p377;

        s.v[371] = p.p378;

        s.v[372] = p.p379;

        s.v[373] = p.p380;

        s.v[374] = p.p381;

        s.v[375] = p.p382;

        s.v[376] = p.p383;

        s.v[377] = p.p384;

        s.v[378] = p.p385;

        s.v[379] = p.p386;

        s.v[380] = p.p387;

        s.v[381] = p.p388;

        s.v[382] = p.p389;

        s.v[383] = p.p390;

        s.v[384] = p.p391;

        s.v[385] = p.p392;

        s.v[388] = p.p395;

        s.v[389] = p.p396;

        s.v[390] = p.p397;

        s.v[391] = p.p398;

        s.v[392] = p.p399;

        s.v[393] = p.p400;

        s.v[394] = p.p401;

        s.v[395] = p.p402;

        s.v[396] = p.p403;

        s.v[386] = p.p393;

        s.v[387] = p.p394;

        s.v[397] = p.p404;

        s.v[398] = p.p405;

        s.v[399] = p.p406;

        s.v[400] = p.p407;

        s.v[401] = p.p408;

        s.v[402] = p.p409;

        s.v[403] = p.p410;

        s.v[404] = p.p411;

        s.v[405] = p.p412;

        s.v[406] = p.p413;

        s.v[407] = p.p414;

        s.v[408] = p.p418;

        s.v[455] = p.p985;

        s.v[456] = p.p986;

        s.v[457] = p.p987;

        s.v[458] = p.p988;

        s.v[459] = p.p989;

        s.v[460] = p.p990;

        s.v[461] = p.p991;

        s.v[462] = p.p992;

        s.v[463] = p.p993;

        s.v[464] = p.p994;

        s.v[465] = p.p995;

        if (s.v[68] != 0.0) {
            s.store_scalar(777, 3.9);
            s.store_scalar(776, s.v[72]);
            s.store_scalar(778, (8.85418e-12 * s.v[74]));
            s.store_sqrt_scaled_input(780, 778, (2000000.0 * 1.60219e-19));
            s.store_scaled_div(757, 777, 776, 8.85418e-12);
            s.store_scalar(781, s.v[455]);
            s.store_scalar(782, s.v[456]);
            s.store_scalar(784, s.v[457]);
            s.store_scalar(785, s.v[458]);
            s.store_scalar(786, s.v[459]);
            s.store_scalar(787, s.v[460]);
            s.store_scalar(788, s.v[461]);
            s.store_scalar(789, s.v[462]);
            s.store_scalar(790, s.v[463]);
            s.store_scalar(791, s.v[464]);
        }

        if (s.v[68] == 0.0) {
            s.store_scalar(777, s.v[73]);
            s.store_scalar(776, s.v[91]);
            s.store_scalar(778, 1.03594e-10);
            s.store_scalar(780, 5.753e-12);
            s.store_scalar(757, (3.453133e-11 / s.v[91]));
            s.store_scalar(781, s.v[455]);
            s.store_scalar(782, s.v[456]);
            s.store_scalar(784, s.v[457]);
            s.store_scalar(785, s.v[458]);
            s.store_scalar(786, s.v[459]);
            s.store_scalar(787, s.v[460]);
            s.store_scalar(788, s.v[461]);
            s.store_scalar(789, s.v[462]);
            s.store_scalar(790, s.v[463]);
            s.store_scalar(791, s.v[464]);
        }

        s.v[760] = 0.0;

        s.b[807] = param_given[203];
        s.v[807] = if s.b[807] { 1.0 } else { 0.0 };

        if s.b[807] {
            s.store_scalar(232, p.p203);
        }

        if (!s.b[807]) {
            s.store_scalar(232, (((2.0 * 3.453133e-11) / 3.141592653589793) * (((1.0 + (4e-7 / s.v[91]))) as f64).ln()));
        }

        s.b[808] = param_given[125];
        s.v[808] = if s.b[808] { 1.0 } else { 0.0 };

        if s.b[808] {
            s.store_scalar(152, p.p125);
        }

        s.b[809] = (param_given[207] && (s.v[236] > 0.0));
        s.v[809] = if s.b[809] { 1.0 } else { 0.0 };

        if ((!s.b[808]) && s.b[809]) {
            s.store_offset_scaled(152, 757, s.v[236], (-s.v[230]));
        }

        if ((!s.b[808]) && (!s.b[809])) {
            s.store_scale(152, 757, (0.6 * s.v[176]));
        }

        s.b[810] = param_given[124];
        s.v[810] = if s.b[810] { 1.0 } else { 0.0 };

        if s.b[810] {
            s.store_scalar(151, p.p124);
        }

        s.b[811] = (param_given[207] && (s.v[236] > 0.0));
        s.v[811] = if s.b[811] { 1.0 } else { 0.0 };

        if ((!s.b[810]) && s.b[811]) {
            s.store_offset_scaled(151, 757, s.v[236], (-s.v[229]));
        }

        if ((!s.b[810]) && (!s.b[811])) {
            s.store_scale(151, 757, (0.6 * s.v[176]));
        }

        s.b[885] = (s.v[200] < 0.1);
        s.v[885] = if s.b[885] { 1.0 } else { 0.0 };

        if s.b[885] {
            s.store_scalar(200, 0.1);
        }

        s.b[886] = (s.v[201] < 0.1);
        s.v[886] = if s.b[886] { 1.0 } else { 0.0 };

        if s.b[886] {
            s.store_scalar(201, 0.1);
        }

        s.v[832] = s.v[150];

        s.v[827] = (s.v[769] / s.v[832]);

        if (s.v[68] != 0.0) {
            s.store_sqrt_mul_ad(758, A::div(s.ad_value(778), A::scale(s.ad_value(777), 8.85418e-12)), s.ad_value(776));
        }

        if (s.v[68] == 0.0) {
            s.store_scalar(758, ((((1.03594e-10 / 3.453133e-11) * s.v[91])) as f64).sqrt());
        }

        s.v[783] = s.v[465];

        s.b[887] = (s.v[68] == 0.0);
        s.v[887] = if s.b[887] { 1.0 } else { 0.0 };

        if s.b[887] {
            s.store_scalar(831, (8.617087e-5 * s.v[832]));
            s.store_scalar(816, (1.16 - (((0.000702 * s.v[832]) * s.v[832]) / (s.v[832] + 1108.0))));
            s.copy_ad(755, 816);
            s.store_scalar(409, (8.617087e-5 * s.v[769]));
            s.store_scalar(815, (1.16 - (((0.000702 * s.v[769]) * s.v[769]) / (s.v[769] + 1108.0))));
            s.copy_ad(756, 815);
            s.store_scaled_exp_ad(817, A::sub_from_scalar(21.5565981, A::div(s.ad_value(815), A::scale(s.ad_value(409), 2.0))), ((14500000000.0 * (s.v[769] / 300.15)) * (((s.v[769] / 300.15)) as f64).sqrt()));
        }

        if (!s.b[887]) {
            s.store_scalar(831, (8.617087e-5 * s.v[832]));
            s.store_scalar(816, (s.v[76] - (((s.v[77] * s.v[832]) * s.v[832]) / (s.v[832] + s.v[78]))));
            s.copy_ad(755, 816);
            s.store_scalar(409, (8.617087e-5 * s.v[769]));
            s.store_scalar(815, (s.v[76] - (((s.v[77] * s.v[769]) * s.v[769]) / (s.v[769] + s.v[78]))));
            s.copy_ad(756, 815);
            s.store_scaled_exp_ad(817, A::sub(A::div(s.ad_value(816), A::scale(s.ad_value(831), 2.0)), A::div(s.ad_value(815), A::scale(s.ad_value(409), 2.0))), ((s.v[75] * (s.v[769] / s.v[832])) * (((s.v[769] / s.v[832])) as f64).sqrt()));
        }

        s.v[427] = (s.v[52] * s.v[330]);

        s.v[825] = s.v[37];

        s.v[826] = (s.v[38] / s.v[39]);

        s.v[818] = ((s.v[825]) as f64).powf(s.v[209]);

        s.v[819] = ((s.v[826]) as f64).powf(s.v[212]);

        s.v[813] = (((s.v[207] / s.v[818]) + (s.v[210] / s.v[819])) + (s.v[213] / (s.v[818] * s.v[819])));

        s.v[687] = (s.v[206] + s.v[813]);

        s.v[813] = (((s.v[208] / s.v[818]) + (s.v[211] / s.v[819])) + (s.v[214] / (s.v[818] * s.v[819])));

        s.v[691] = (s.v[236] + s.v[813]);

        s.v[581] = (s.v[385] + s.v[813]);

        s.b[888] = (s.v[581] < 0.0);
        s.v[888] = if s.b[888] { 1.0 } else { 0.0 };

        if s.b[888] {
            s.store_scalar(581, 0.0);
        }

        s.v[820] = ((s.v[825]) as f64).powf(s.v[221]);

        s.v[821] = ((s.v[826]) as f64).powf(s.v[224]);

        s.v[814] = (((s.v[219] / s.v[820]) + (s.v[222] / s.v[821])) + (s.v[225] / (s.v[820] * s.v[821])));

        s.v[686] = (s.v[216] + s.v[814]);

        s.v[814] = (((s.v[220] / s.v[820]) + (s.v[223] / s.v[821])) + (s.v[226] / (s.v[820] * s.v[821])));

        s.v[690] = (s.v[235] + s.v[814]);

        s.v[688] = (s.v[37] - (2.0 * s.v[687]));

    }

    pub(super) fn stamp_reactive_block_2(
        s: &mut ReactiveScratch,
        p: &Parameters,
    ) {
        s.v[689] = (((s.v[38] / s.v[39]) - (s.v[58] * s.v[284])) - ((2.0 - s.v[58]) * s.v[686]));

        s.v[709] = ((s.v[689] / s.v[59]) + s.v[60]);

        s.v[708] = ((s.v[689] / s.v[59]) + s.v[61]);

        s.v[692] = (s.v[37] - (2.0 * s.v[691]));

        s.v[693] = (((s.v[38] / s.v[39]) - (s.v[58] * s.v[284])) - ((2.0 - s.v[58]) * s.v[690]));

        s.v[710] = ((s.v[693] / s.v[59]) + s.v[60]);

        s.v[711] = ((s.v[693] / s.v[59]) + s.v[61]);

        s.v[726] = ((s.v[37] - (2.0 * s.v[691])) - s.v[341]);

        s.v[727] = (s.v[726] + (2.0 * s.v[353]));

        s.v[482] = s.v[111];

        s.v[483] = s.v[112];

        s.v[484] = s.v[113];

        s.v[486] = s.v[114];

        s.v[487] = s.v[115];

        s.copy_ad(605, 232);

        s.v[606] = s.v[233];

        s.v[607] = s.v[234];

        s.v[694] = (1.0 + (((s.v[606] / s.v[688])) as f64).powf(s.v[607]));

        s.b[895] = (s.v[90] == 1.0);
        s.v[895] = if s.b[895] { 1.0 } else { 0.0 };

        if s.b[895] {
            s.store_scalar(828, (1e-6 / s.v[688]));
            s.store_scalar(829, (1e-6 / s.v[689]));
            s.store_scalar(830, (1e-12 / (s.v[688] * s.v[689])));
        }

        if (!s.b[895]) {
            s.store_scalar(828, (1.0 / s.v[688]));
            s.store_scalar(829, (1.0 / s.v[689]));
            s.store_scalar(830, (1.0 / (s.v[688] * s.v[689])));
        }

        s.store_add_scaled_ad_lhs(478, A::add(A::offset(A::scale(s.ad_value(828), p.p461), s.v[108]), A::scale(s.ad_value(829), p.p642)), 830, p.p823);

        s.store_add_scaled_ad_lhs(479, A::add(A::offset(A::scale(s.ad_value(828), p.p462), s.v[107]), A::scale(s.ad_value(829), p.p643)), 830, p.p824);

        s.store_add_scaled_ad_lhs(480, A::add(A::offset(A::scale(s.ad_value(828), p.p463), s.v[109]), A::scale(s.ad_value(829), p.p644)), 830, p.p826);

        s.store_add_scaled_ad_lhs(481, A::add(A::offset(A::scale(s.ad_value(828), p.p464), s.v[110]), A::scale(s.ad_value(829), p.p645)), 830, p.p825);

        s.store_add_scaled_ad_lhs(507, A::add(A::offset(A::scale(s.ad_value(828), p.p465), s.v[134]), A::scale(s.ad_value(829), p.p646)), 830, p.p827);

        s.store_add_scaled_ad_lhs(522, A::add(A::offset(A::scale(s.ad_value(828), p.p466), s.v[135]), A::scale(s.ad_value(829), p.p647)), 830, p.p828);

        s.store_add_scaled_ad_lhs(490, A::add(A::offset(A::scale(s.ad_value(828), p.p467), s.v[116]), A::scale(s.ad_value(829), p.p648)), 830, p.p829);

        s.store_add_scaled_ad_lhs(494, A::add(A::offset(A::scale(s.ad_value(828), p.p470), s.v[120]), A::scale(s.ad_value(829), p.p651)), 830, p.p832);

        s.store_add_scaled_ad_lhs(627, A::add(A::offset(A::scale(s.ad_value(828), p.p468), s.v[281]), A::scale(s.ad_value(829), p.p649)), 830, p.p830);

        s.store_add_scaled_ad_lhs(628, A::add(A::offset(A::scale(s.ad_value(828), p.p469), s.v[282]), A::scale(s.ad_value(829), p.p650)), 830, p.p831);

        s.store_add_scaled_ad_lhs(495, A::add(A::offset(A::scale(s.ad_value(828), p.p471), s.v[121]), A::scale(s.ad_value(829), p.p652)), 830, p.p833);

        s.store_add_scaled_ad_lhs(496, A::add(A::offset(A::scale(s.ad_value(828), p.p472), s.v[122]), A::scale(s.ad_value(829), p.p653)), 830, p.p834);

        s.store_add_scaled_ad_lhs(626, A::add(A::offset(A::scale(s.ad_value(828), p.p473), s.v[352]), A::scale(s.ad_value(829), p.p654)), 830, p.p835);

        s.store_add_scaled_ad_lhs(497, A::add(A::offset(A::scale(s.ad_value(828), p.p474), s.v[123]), A::scale(s.ad_value(829), p.p655)), 830, p.p836);

        s.store_add_scaled_ad_lhs(498, A::add(A::offset(A::scale(s.ad_value(828), p.p976), s.v[124]), A::scale(s.ad_value(829), p.p979)), 830, p.p982);

        s.store_add_scaled_ad_lhs(738, A::add(A::offset(A::scale(s.ad_value(828), p.p475), s.v[125]), A::scale(s.ad_value(829), p.p656)), 830, p.p837);

        s.store_add_scaled_ad_lhs(499, A::add(A::offset(A::scale(s.ad_value(828), p.p476), s.v[126]), A::scale(s.ad_value(829), p.p657)), 830, p.p838);

        s.store_add_scaled_ad_lhs(500, A::add(A::offset(A::scale(s.ad_value(828), p.p477), s.v[127]), A::scale(s.ad_value(829), p.p658)), 830, p.p839);

        s.store_add_scaled_ad_lhs(501, A::add(A::offset(A::scale(s.ad_value(828), p.p478), s.v[128]), A::scale(s.ad_value(829), p.p659)), 830, p.p840);

        s.store_add_scaled_ad_lhs(502, A::add(A::offset(A::scale(s.ad_value(828), p.p479), s.v[129]), A::scale(s.ad_value(829), p.p660)), 830, p.p841);

        s.store_add_scaled_ad_lhs(503, A::add(A::offset(A::scale(s.ad_value(828), p.p480), s.v[130]), A::scale(s.ad_value(829), p.p661)), 830, p.p842);

        s.store_add_scaled_ad_lhs(504, A::add(A::offset(A::scale(s.ad_value(828), p.p481), s.v[131]), A::scale(s.ad_value(829), p.p662)), 830, p.p843);

        s.store_add_scaled_ad_lhs(514, A::add(A::offset(A::scale(s.ad_value(828), p.p482), s.v[142]), A::scale(s.ad_value(829), p.p663)), 830, p.p844);

        s.store_add_scaled_ad_lhs(508, A::add(A::offset(A::scale(s.ad_value(828), p.p484), s.v[136]), A::scale(s.ad_value(829), p.p665)), 830, p.p846);

        s.store_add_scaled_ad_lhs(510, A::add(A::offset(A::scale(s.ad_value(828), p.p485), s.v[138]), A::scale(s.ad_value(829), p.p666)), 830, p.p847);

        s.store_add_scaled_ad_lhs(512, A::add(A::offset(A::scale(s.ad_value(828), p.p486), s.v[140]), A::scale(s.ad_value(829), p.p667)), 830, p.p848);

        s.store_add_scaled_ad_lhs(471, A::add(A::offset(A::scale(s.ad_value(828), p.p491), s.v[100]), A::scale(s.ad_value(829), p.p672)), 830, p.p853);

        s.store_add_scaled_ad_lhs(473, A::add(A::offset(A::scale(s.ad_value(828), p.p492), s.v[102]), A::scale(s.ad_value(829), p.p673)), 830, p.p854);

        s.store_add_scaled_ad_lhs(474, A::add(A::offset(A::scale(s.ad_value(828), p.p493), s.v[103]), A::scale(s.ad_value(829), p.p674)), 830, p.p855);

        s.store_add_scaled_ad_lhs(568, A::add(A::offset(A::scale(s.ad_value(828), p.p494), s.v[227]), A::scale(s.ad_value(829), p.p675)), 830, p.p856);

        s.store_add_scaled_ad_lhs(569, A::add(A::offset(A::scale(s.ad_value(828), p.p495), s.v[228]), A::scale(s.ad_value(829), p.p676)), 830, p.p857);

        s.store_add_scaled_ad_lhs(477, A::add(A::offset(A::scale(s.ad_value(828), p.p496), s.v[106]), A::scale(s.ad_value(829), p.p677)), 830, p.p858);

        s.store_add_scaled_ad_lhs(629, A::add(A::offset(A::scale(s.ad_value(828), p.p497), s.v[283]), A::scale(s.ad_value(829), p.p678)), 830, p.p859);

        s.store_add_scaled_ad_lhs(475, A::add(A::offset(A::scale(s.ad_value(828), p.p498), s.v[104]), A::scale(s.ad_value(829), p.p679)), 830, p.p860);

        s.store_add_scaled_ad_lhs(476, A::add(A::offset(A::scale(s.ad_value(828), p.p499), s.v[105]), A::scale(s.ad_value(829), p.p680)), 830, p.p861);

        s.store_add_scaled_ad_lhs(551, A::add(A::offset(A::scale(s.ad_value(828), p.p500), s.v[156]), A::scale(s.ad_value(829), p.p681)), 830, p.p862);

        s.store_add_scaled_ad_lhs(540, A::add(A::offset(A::scale(s.ad_value(828), p.p501), s.v[157]), A::scale(s.ad_value(829), p.p682)), 830, p.p863);

        s.store_add_scaled_ad_lhs(539, A::add(A::offset(A::scale(s.ad_value(828), p.p502), s.v[158]), A::scale(s.ad_value(829), p.p683)), 830, p.p864);

        s.store_add_scaled_ad_lhs(554, A::add(A::offset(A::scale(s.ad_value(828), p.p503), s.v[162]), A::scale(s.ad_value(829), p.p684)), 830, p.p865);

        s.store_add_scaled_ad_lhs(553, A::add(A::offset(A::scale(s.ad_value(828), p.p504), s.v[161]), A::scale(s.ad_value(829), p.p685)), 830, p.p866);

        s.store_add_scaled_ad_lhs(565, A::add(A::offset(A::scale(s.ad_value(828), p.p505), s.v[215]), A::scale(s.ad_value(829), p.p686)), 830, p.p867);

        s.store_add_scaled_ad_lhs(470, A::add(A::offset(A::scale(s.ad_value(828), p.p506), s.v[99]), A::scale(s.ad_value(829), p.p687)), 830, p.p868);

        s.store_add_scaled_ad_lhs(566, A::add(A::offset(A::scale(s.ad_value(828), p.p507), s.v[217]), A::scale(s.ad_value(829), p.p688)), 830, p.p869);

        s.store_add_scaled_ad_lhs(567, A::add(A::offset(A::scale(s.ad_value(828), p.p508), s.v[218]), A::scale(s.ad_value(829), p.p689)), 830, p.p870);

        s.store_add_scaled_ad_lhs(521, A::add(A::offset(A::scale(s.ad_value(828), p.p509), s.v[149]), A::scale(s.ad_value(829), p.p690)), 830, p.p871);

        s.store_add_scaled_ad_lhs(556, A::add(A::offset(A::scale(s.ad_value(828), p.p510), s.v[164]), A::scale(s.ad_value(829), p.p691)), 830, p.p872);

        s.store_add_scaled_ad_lhs(557, A::add(A::offset(A::scale(s.ad_value(828), p.p511), s.v[165]), A::scale(s.ad_value(829), p.p692)), 830, p.p873);

        s.store_add_scaled_ad_lhs(558, A::add(A::offset(A::scale(s.ad_value(828), p.p512), s.v[166]), A::scale(s.ad_value(829), p.p693)), 830, p.p874);

        s.store_add_scaled_ad_lhs(559, A::add(A::offset(A::scale(s.ad_value(828), p.p513), s.v[167]), A::scale(s.ad_value(829), p.p694)), 830, p.p875);

        s.store_add_scaled_ad_lhs(506, A::add(A::offset(A::scale(s.ad_value(828), p.p514), s.v[133]), A::scale(s.ad_value(829), p.p695)), 830, p.p876);

        s.store_add_scaled_ad_lhs(469, A::add(A::offset(A::scale(s.ad_value(828), p.p515), s.v[98]), A::scale(s.ad_value(829), p.p696)), 830, p.p877);

        s.store_add_scaled_ad_lhs(466, A::add(A::offset(A::scale(s.ad_value(828), p.p516), s.v[95]), A::scale(s.ad_value(829), p.p697)), 830, p.p878);

        s.store_add_scaled_ad_lhs(467, A::add(A::offset(A::scale(s.ad_value(828), p.p517), s.v[96]), A::scale(s.ad_value(829), p.p698)), 830, p.p879);

        s.store_add_scaled_ad_lhs(468, A::add(A::offset(A::scale(s.ad_value(828), p.p518), s.v[97]), A::scale(s.ad_value(829), p.p699)), 830, p.p880);

        s.store_add_scaled_ad_lhs(560, A::add(A::offset(A::scale(s.ad_value(828), p.p519), s.v[168]), A::scale(s.ad_value(829), p.p700)), 830, p.p881);

        s.store_add_scaled_ad_lhs(561, A::add(A::offset(A::scale(s.ad_value(828), p.p520), s.v[169]), A::scale(s.ad_value(829), p.p701)), 830, p.p882);

        s.store_add_scaled_ad_lhs(562, A::add(A::offset(A::scale(s.ad_value(828), p.p521), s.v[170]), A::scale(s.ad_value(829), p.p702)), 830, p.p883);

        s.store_add_scaled_ad_lhs(563, A::add(A::offset(A::scale(s.ad_value(828), p.p522), s.v[171]), A::scale(s.ad_value(829), p.p703)), 830, p.p884);

        s.store_add_scaled_ad_lhs(505, A::add(A::offset(A::scale(s.ad_value(828), p.p523), s.v[132]), A::scale(s.ad_value(829), p.p704)), 830, p.p885);

        s.store_add_scaled_ad_lhs(564, A::add(A::offset(A::scale(s.ad_value(828), p.p524), s.v[172]), A::scale(s.ad_value(829), p.p705)), 830, p.p886);

        s.store_add_scaled_ad_lhs(550, A::add(A::offset(A::scale(s.ad_value(828), p.p525), s.v[154]), A::scale(s.ad_value(829), p.p706)), 830, p.p887);

        s.store_add_scaled_ad_lhs(570, A::add(A::offset(A::scale(s.ad_value(828), p.p526), s.v[237]), A::scale(s.ad_value(829), p.p707)), 830, p.p888);

        s.store_add_scaled_ad_lhs(630, A::add(A::offset(A::scale(s.ad_value(828), p.p527), s.v[295]), A::scale(s.ad_value(829), p.p708)), 830, p.p889);

        s.store_add_scaled_ad_lhs(631, A::add(A::offset(A::scale(s.ad_value(828), p.p530), s.v[296]), A::scale(s.ad_value(829), p.p711)), 830, p.p892);

        s.store_add_scaled_ad_lhs(632, A::add(A::offset(A::scale(s.ad_value(828), p.p529), s.v[297]), A::scale(s.ad_value(829), p.p710)), 830, p.p891);

        s.store_add_scaled_ad_lhs(633, A::add(A::offset(A::scale(s.ad_value(828), p.p532), s.v[298]), A::scale(s.ad_value(829), p.p713)), 830, p.p894);

        s.store_add_scaled_ad_lhs(634, A::add(A::offset(A::scale(s.ad_value(828), p.p528), s.v[299]), A::scale(s.ad_value(829), p.p709)), 830, p.p890);

        s.store_add_scaled_ad_lhs(635, A::add(A::offset(A::scale(s.ad_value(828), p.p531), s.v[300]), A::scale(s.ad_value(829), p.p712)), 830, p.p893);

        s.store_add_scaled_ad_lhs(571, A::add(A::offset(A::scale(s.ad_value(828), p.p533), s.v[285]), A::scale(s.ad_value(829), p.p714)), 830, p.p895);

        s.store_add_scaled_ad_lhs(636, A::add(A::offset(A::scale(s.ad_value(828), p.p534), s.v[286]), A::scale(s.ad_value(829), p.p715)), 830, p.p896);

        s.store_add_scaled_ad_lhs(637, A::add(A::offset(A::scale(s.ad_value(828), p.p535), s.v[287]), A::scale(s.ad_value(829), p.p716)), 830, p.p897);

        s.store_add_scaled_ad_lhs(638, A::add(A::offset(A::scale(s.ad_value(828), p.p536), s.v[288]), A::scale(s.ad_value(829), p.p717)), 830, p.p898);

        s.store_add_scaled_ad_lhs(639, A::add(A::offset(A::scale(s.ad_value(828), p.p537), s.v[290]), A::scale(s.ad_value(829), p.p718)), 830, p.p899);

        s.store_add_scaled_ad_lhs(640, A::add(A::offset(A::scale(s.ad_value(828), p.p538), s.v[302]), A::scale(s.ad_value(829), p.p719)), 830, p.p900);

        s.store_add_scaled_ad_lhs(641, A::add(A::offset(A::scale(s.ad_value(828), p.p539), s.v[291]), A::scale(s.ad_value(829), p.p720)), 830, p.p901);

        s.store_add_scaled_ad_lhs(642, A::add(A::offset(A::scale(s.ad_value(828), p.p540), s.v[292]), A::scale(s.ad_value(829), p.p721)), 830, p.p902);

        s.store_add_scaled_ad_lhs(643, A::add(A::offset(A::scale(s.ad_value(828), p.p541), s.v[293]), A::scale(s.ad_value(829), p.p722)), 830, p.p903);

        s.store_add_scaled_ad_lhs(644, A::add(A::offset(A::scale(s.ad_value(828), p.p542), s.v[294]), A::scale(s.ad_value(829), p.p723)), 830, p.p904);

        s.store_add_scaled_ad_lhs(645, A::add(A::offset(A::scale(s.ad_value(828), p.p543), s.v[178]), A::scale(s.ad_value(829), p.p724)), 830, p.p905);

        s.store_add_scaled_ad_lhs(646, A::add(A::offset(A::scale(s.ad_value(828), p.p544), s.v[179]), A::scale(s.ad_value(829), p.p725)), 830, p.p906);

        s.store_add_scaled_ad_lhs(647, A::add(A::offset(A::scale(s.ad_value(828), p.p545), s.v[180]), A::scale(s.ad_value(829), p.p726)), 830, p.p907);

        s.store_add_scaled_ad_lhs(648, A::add(A::offset(A::scale(s.ad_value(828), p.p977), s.v[177]), A::scale(s.ad_value(829), p.p980)), 830, p.p983);

        s.store_add_scaled_ad_lhs(649, A::add(A::offset(A::scale(s.ad_value(828), p.p546), s.v[181]), A::scale(s.ad_value(829), p.p727)), 830, p.p908);

        s.store_add_scaled_ad_lhs(650, A::add(A::offset(A::scale(s.ad_value(828), p.p547), s.v[182]), A::scale(s.ad_value(829), p.p728)), 830, p.p909);

        s.store_add_scaled_ad_lhs(651, A::add(A::offset(A::scale(s.ad_value(828), p.p548), s.v[183]), A::scale(s.ad_value(829), p.p729)), 830, p.p910);

        s.store_add_scaled_ad_lhs(652, A::add(A::offset(A::scale(s.ad_value(828), p.p549), s.v[185]), A::scale(s.ad_value(829), p.p730)), 830, p.p911);

        s.store_add_scaled_ad_lhs(653, A::add(A::offset(A::scale(s.ad_value(828), p.p550), s.v[186]), A::scale(s.ad_value(829), p.p731)), 830, p.p912);

        s.store_add_scaled_ad_lhs(654, A::add(A::offset(A::scale(s.ad_value(828), p.p551), s.v[187]), A::scale(s.ad_value(829), p.p732)), 830, p.p913);

        s.store_add_scaled_ad_lhs(655, A::add(A::offset(A::scale(s.ad_value(828), p.p978), s.v[184]), A::scale(s.ad_value(829), p.p981)), 830, p.p984);

        s.store_add_scaled_ad_lhs(656, A::add(A::offset(A::scale(s.ad_value(828), p.p552), s.v[188]), A::scale(s.ad_value(829), p.p733)), 830, p.p914);

        s.store_add_scaled_ad_lhs(657, A::add(A::offset(A::scale(s.ad_value(828), p.p553), s.v[189]), A::scale(s.ad_value(829), p.p734)), 830, p.p915);

        s.store_add_scaled_ad_lhs(658, A::add(A::offset(A::scale(s.ad_value(828), p.p554), s.v[190]), A::scale(s.ad_value(829), p.p735)), 830, p.p916);

        s.store_add_scaled_ad_lhs(659, A::add(A::offset(A::scale(s.ad_value(828), p.p555), s.v[303]), A::scale(s.ad_value(829), p.p736)), 830, p.p917);

        s.store_add_scaled_ad_lhs(660, A::add(A::offset(A::scale(s.ad_value(828), p.p556), s.v[304]), A::scale(s.ad_value(829), p.p737)), 830, p.p918);

        s.store_add_scaled_ad_lhs(661, A::add(A::offset(A::scale(s.ad_value(828), p.p557), s.v[191]), A::scale(s.ad_value(829), p.p738)), 830, p.p919);

        s.store_add_scaled_ad_lhs(662, A::add(A::offset(A::scale(s.ad_value(828), p.p558), s.v[192]), A::scale(s.ad_value(829), p.p739)), 830, p.p920);

        s.store_add_scaled_ad_lhs(663, A::add(A::offset(A::scale(s.ad_value(828), p.p559), s.v[305]), A::scale(s.ad_value(829), p.p740)), 830, p.p921);

        s.store_add_scaled_ad_lhs(664, A::add(A::offset(A::scale(s.ad_value(828), p.p560), s.v[306]), A::scale(s.ad_value(829), p.p741)), 830, p.p922);

        s.store_add_scaled_ad_lhs(665, A::add(A::offset(A::scale(s.ad_value(828), p.p561), s.v[307]), A::scale(s.ad_value(829), p.p742)), 830, p.p923);

        s.store_add_scaled_ad_lhs(666, A::add(A::offset(A::scale(s.ad_value(828), p.p562), s.v[308]), A::scale(s.ad_value(829), p.p743)), 830, p.p924);

        s.store_add_scaled_ad_lhs(667, A::add(A::offset(A::scale(s.ad_value(828), p.p563), s.v[309]), A::scale(s.ad_value(829), p.p744)), 830, p.p925);

        s.store_add_scaled_ad_lhs(668, A::add(A::offset(A::scale(s.ad_value(828), p.p564), s.v[310]), A::scale(s.ad_value(829), p.p745)), 830, p.p926);

        s.store_add_scaled_ad_lhs(669, A::add(A::offset(A::scale(s.ad_value(828), p.p565), s.v[311]), A::scale(s.ad_value(829), p.p746)), 830, p.p927);

        s.store_add_scaled_ad_lhs(670, A::add(A::offset(A::scale(s.ad_value(828), p.p566), s.v[312]), A::scale(s.ad_value(829), p.p747)), 830, p.p928);

        s.store_add_scaled_ad_lhs(671, A::add(A::offset(A::scale(s.ad_value(828), p.p567), s.v[313]), A::scale(s.ad_value(829), p.p748)), 830, p.p929);

        s.store_add_scaled_ad_lhs(673, A::add(A::offset(A::scale(s.ad_value(828), p.p569), s.v[315]), A::scale(s.ad_value(829), p.p750)), 830, p.p931);

        s.store_add_scaled_ad_lhs(672, A::add(A::offset(A::scale(s.ad_value(828), p.p568), s.v[314]), A::scale(s.ad_value(829), p.p749)), 830, p.p930);

        s.store_add_scaled_ad_lhs(674, A::add(A::offset(A::scale(s.ad_value(828), p.p570), s.v[316]), A::scale(s.ad_value(829), p.p751)), 830, p.p932);

        s.store_add_scaled_ad_lhs(675, A::add(A::offset(A::scale(s.ad_value(828), p.p571), s.v[318]), A::scale(s.ad_value(829), p.p752)), 830, p.p933);

        s.store_add_scaled_ad_lhs(676, A::add(A::offset(A::scale(s.ad_value(828), p.p572), s.v[319]), A::scale(s.ad_value(829), p.p753)), 830, p.p934);

        s.store_add_scaled_ad_lhs(677, A::add(A::offset(A::scale(s.ad_value(828), p.p573), s.v[320]), A::scale(s.ad_value(829), p.p754)), 830, p.p935);

        s.store_add_scaled_ad_lhs(678, A::add(A::offset(A::scale(s.ad_value(828), p.p574), s.v[321]), A::scale(s.ad_value(829), p.p755)), 830, p.p936);

        s.store_add_scaled_ad_lhs(679, A::add(A::offset(A::scale(s.ad_value(828), p.p575), s.v[322]), A::scale(s.ad_value(829), p.p756)), 830, p.p937);

        s.store_add_scaled_ad_lhs(680, A::add(A::offset(A::scale(s.ad_value(828), p.p576), s.v[323]), A::scale(s.ad_value(829), p.p757)), 830, p.p938);

        s.store_add_scaled_ad_lhs(681, A::add(A::offset(A::scale(s.ad_value(828), p.p577), s.v[325]), A::scale(s.ad_value(829), p.p758)), 830, p.p939);

        s.store_add_scaled_ad_lhs(682, A::add(A::offset(A::scale(s.ad_value(828), p.p578), s.v[326]), A::scale(s.ad_value(829), p.p759)), 830, p.p940);

        s.store_add_scaled_ad_lhs(716, A::add(A::offset(A::scale(s.ad_value(828), p.p579), s.v[327]), A::scale(s.ad_value(829), p.p760)), 830, p.p941);

        s.store_add_scaled_ad_lhs(717, A::add(A::offset(A::scale(s.ad_value(828), p.p580), s.v[328]), A::scale(s.ad_value(829), p.p761)), 830, p.p942);

        s.store_add_scaled_ad_lhs(608, A::add(A::offset(A::scale(s.ad_value(828), p.p422), s.v[176]), A::scale(s.ad_value(829), p.p603)), 830, p.p784);

        s.store_add_scaled_ad_lhs(609, A::add(A::offset(A::scale(s.ad_value(828), p.p423), s.v[364]), A::scale(s.ad_value(829), p.p604)), 830, p.p785);

        s.store_add_scaled_ad_lhs(611, A::add(A::offset(A::scale(s.ad_value(828), p.p425), s.v[368]), A::scale(s.ad_value(829), p.p606)), 830, p.p787);

        s.store_add_scaled_ad_lhs(610, A::add(A::offset(A::scale(s.ad_value(828), p.p424), s.v[365]), A::scale(s.ad_value(829), p.p605)), 830, p.p786);

        s.store_add_scaled_ad_lhs(612, A::add(A::offset(A::scale(s.ad_value(828), p.p426), s.v[369]), A::scale(s.ad_value(829), p.p607)), 830, p.p788);

        s.store_add_scaled_ad_lhs(616, A::add(A::offset(A::scale(s.ad_value(828), p.p433), s.v[333]), A::scale(s.ad_value(829), p.p614)), 830, p.p795);

        s.store_add_scaled_ad_lhs(617, A::add(A::offset(A::scale(s.ad_value(828), p.p443), s.v[339]), A::scale(s.ad_value(829), p.p624)), 830, p.p805);

        s.store_add_scaled_ad_lhs(618, A::add(A::offset(A::scale(s.ad_value(828), p.p444), s.v[340]), A::scale(s.ad_value(829), p.p625)), 830, p.p806);

        s.store_add_scaled_ad_lhs(619, A::add(A::offset(A::scale(s.ad_value(828), p.p445), s.v[193]), A::scale(s.ad_value(829), p.p626)), 830, p.p807);

        s.store_add_scaled_ad_lhs(620, A::add(A::offset(A::scale(s.ad_value(828), p.p446), s.v[194]), A::scale(s.ad_value(829), p.p627)), 830, p.p808);

        s.store_add_scaled_ad_lhs(621, A::add(A::offset(A::scale(s.ad_value(828), p.p447), s.v[195]), A::scale(s.ad_value(829), p.p628)), 830, p.p809);

        s.store_add_scaled_ad_lhs(622, A::add(A::offset(A::scale(s.ad_value(828), p.p448), s.v[196]), A::scale(s.ad_value(829), p.p629)), 830, p.p810);

        s.store_add_scaled_ad_lhs(623, A::add(A::offset(A::scale(s.ad_value(828), p.p449), s.v[197]), A::scale(s.ad_value(829), p.p630)), 830, p.p811);

        s.store_add_scaled_ad_lhs(624, A::add(A::offset(A::scale(s.ad_value(828), p.p450), s.v[198]), A::scale(s.ad_value(829), p.p631)), 830, p.p812);

        s.store_add_scaled_ad_lhs(625, A::add(A::offset(A::scale(s.ad_value(828), p.p451), s.v[199]), A::scale(s.ad_value(829), p.p632)), 830, p.p813);

        s.store_add_scaled_ad_lhs(603, A::add(A::offset(A::scale(s.ad_value(828), p.p431), s.v[230]), A::scale(s.ad_value(829), p.p612)), 830, p.p793);

        s.store_add_scaled_ad_lhs(602, A::add(A::offset(A::scale(s.ad_value(828), p.p430), s.v[229]), A::scale(s.ad_value(829), p.p611)), 830, p.p792);

        s.store_add_scaled_ad_lhs(604, A::add(A::offset(A::scale(s.ad_value(828), p.p432), s.v[231]), A::scale(s.ad_value(829), p.p613)), 830, p.p794);

        s.store_add_scaled_ad_lhs(515, A::add(A::offset(A::scale(s.ad_value(828), p.p434), s.v[144]), A::scale(s.ad_value(829), p.p615)), 830, p.p796);

        s.store_add_scaled_ad_lhs(516, A::add(A::offset(A::scale(s.ad_value(828), p.p487), s.v[147]), A::scale(s.ad_value(829), p.p668)), 830, p.p849);

        s.store_add_scaled_ad_lhs(517, A::add(A::offset(A::scale(s.ad_value(828), p.p488), s.v[148]), A::scale(s.ad_value(829), p.p669)), 830, p.p850);

        s.store_add_scaled_ad_lhs(518, A::add(A::offset(A::scale(s.ad_value(828), p.p483), s.v[143]), A::scale(s.ad_value(829), p.p664)), 830, p.p845);

        s.store_add_scaled_ad_lhs(519, A::add(A::offset(A::scale(s.ad_value(828), p.p490), s.v[145]), A::scale(s.ad_value(829), p.p671)), 830, p.p852);

        s.store_add_scaled_ad_lhs(520, A::add(A::offset(A::scale(s.ad_value(828), p.p489), s.v[146]), A::scale(s.ad_value(829), p.p670)), 830, p.p851);

        s.store_add_scaled_ad_lhs(491, A::add(A::offset(A::scale(s.ad_value(828), p.p435), s.v[117]), A::scale(s.ad_value(829), p.p616)), 830, p.p797);

        s.store_add_scaled_ad_lhs(493, A::add(A::offset(A::scale(s.ad_value(828), p.p437), s.v[119]), A::scale(s.ad_value(829), p.p618)), 830, p.p799);

        s.store_add_scaled_ad_lhs(492, A::add(A::offset(A::scale(s.ad_value(828), p.p436), s.v[118]), A::scale(s.ad_value(829), p.p617)), 830, p.p798);

        s.store_add_scaled_ad_lhs(509, A::add(A::offset(A::scale(s.ad_value(828), p.p438), s.v[137]), A::scale(s.ad_value(829), p.p619)), 830, p.p800);

        s.store_add_scaled_ad_lhs(511, A::add(A::offset(A::scale(s.ad_value(828), p.p439), s.v[139]), A::scale(s.ad_value(829), p.p620)), 830, p.p801);

        s.store_add_scaled_ad_lhs(513, A::add(A::offset(A::scale(s.ad_value(828), p.p440), s.v[141]), A::scale(s.ad_value(829), p.p621)), 830, p.p802);

        s.store_add_scaled_ad_lhs(472, A::add(A::offset(A::scale(s.ad_value(828), p.p441), s.v[101]), A::scale(s.ad_value(829), p.p622)), 830, p.p803);

        s.store_add_scaled_ad_lhs(555, A::add(A::offset(A::scale(s.ad_value(828), p.p442), s.v[163]), A::scale(s.ad_value(829), p.p623)), 830, p.p804);

        s.store_add_scaled_ad_lhs(578, A::add(A::offset(A::scale(s.ad_value(828), p.p458), s.v[382]), A::scale(s.ad_value(829), p.p639)), 830, p.p820);

        s.store_add_scaled_ad_lhs(572, A::add(A::offset(A::scale(s.ad_value(828), p.p452), s.v[376]), A::scale(s.ad_value(829), p.p633)), 830, p.p814);

        s.store_add_scaled_ad_lhs(573, A::add(A::offset(A::scale(s.ad_value(828), p.p453), s.v[377]), A::scale(s.ad_value(829), p.p634)), 830, p.p815);

        s.store_add_scaled_ad_lhs(574, A::add(A::offset(A::scale(s.ad_value(828), p.p454), s.v[378]), A::scale(s.ad_value(829), p.p635)), 830, p.p816);

        s.store_add_scaled_ad_lhs(575, A::add(A::offset(A::scale(s.ad_value(828), p.p455), s.v[379]), A::scale(s.ad_value(829), p.p636)), 830, p.p817);

        s.store_add_scaled_ad_lhs(576, A::add(A::offset(A::scale(s.ad_value(828), p.p456), s.v[380]), A::scale(s.ad_value(829), p.p637)), 830, p.p818);

        s.store_add_scaled_ad_lhs(577, A::add(A::offset(A::scale(s.ad_value(828), p.p457), s.v[381]), A::scale(s.ad_value(829), p.p638)), 830, p.p819);

        s.store_add_scaled_ad_lhs(579, A::add(A::offset(A::scale(s.ad_value(828), p.p459), s.v[383]), A::scale(s.ad_value(829), p.p640)), 830, p.p821);

        s.store_add_scaled_ad_lhs(580, A::add(A::offset(A::scale(s.ad_value(828), p.p460), s.v[384]), A::scale(s.ad_value(829), p.p641)), 830, p.p822);

        s.store_add_scaled_ad_lhs(595, A::add(A::offset(A::scale(s.ad_value(828), p.p588), s.v[397]), A::scale(s.ad_value(829), p.p769)), 830, p.p950);

        s.store_add_scaled_ad_lhs(596, A::add(A::offset(A::scale(s.ad_value(828), p.p589), s.v[398]), A::scale(s.ad_value(829), p.p770)), 830, p.p951);

        s.store_add_scaled_ad_lhs(582, A::add(A::offset(A::scale(s.ad_value(828), p.p590), s.v[388]), A::scale(s.ad_value(829), p.p771)), 830, p.p952);

        s.store_add_scaled_ad_lhs(583, A::add(A::offset(A::scale(s.ad_value(828), p.p591), s.v[405]), A::scale(s.ad_value(829), p.p772)), 830, p.p953);

        s.store_add_scaled_ad_lhs(584, A::add(A::offset(A::scale(s.ad_value(828), p.p592), s.v[406]), A::scale(s.ad_value(829), p.p773)), 830, p.p954);

        s.store_add_scaled_ad_lhs(585, A::add(A::offset(A::scale(s.ad_value(828), p.p593), s.v[389]), A::scale(s.ad_value(829), p.p774)), 830, p.p955);

        s.store_add_scaled_ad_lhs(586, A::add(A::offset(A::scale(s.ad_value(828), p.p594), s.v[390]), A::scale(s.ad_value(829), p.p775)), 830, p.p956);

        s.store_add_scaled_ad_lhs(587, A::add(A::offset(A::scale(s.ad_value(828), p.p595), s.v[391]), A::scale(s.ad_value(829), p.p776)), 830, p.p957);

        s.store_add_scaled_ad_lhs(588, A::add(A::offset(A::scale(s.ad_value(828), p.p596), s.v[392]), A::scale(s.ad_value(829), p.p777)), 830, p.p958);

        s.store_add_scaled_ad_lhs(589, A::add(A::offset(A::scale(s.ad_value(828), p.p597), s.v[393]), A::scale(s.ad_value(829), p.p778)), 830, p.p959);

        s.store_add_scaled_ad_lhs(590, A::add(A::offset(A::scale(s.ad_value(828), p.p598), s.v[394]), A::scale(s.ad_value(829), p.p779)), 830, p.p960);

        s.store_add_scaled_ad_lhs(591, A::add(A::offset(A::scale(s.ad_value(828), p.p599), s.v[395]), A::scale(s.ad_value(829), p.p780)), 830, p.p961);

        s.store_add_scaled_ad_lhs(592, A::add(A::offset(A::scale(s.ad_value(828), p.p600), s.v[396]), A::scale(s.ad_value(829), p.p781)), 830, p.p962);

        s.store_add_scaled_ad_lhs(593, A::add(A::offset(A::scale(s.ad_value(828), p.p601), s.v[386]), A::scale(s.ad_value(829), p.p782)), 830, p.p963);

        s.store_add_scaled_ad_lhs(594, A::add(A::offset(A::scale(s.ad_value(828), p.p602), s.v[387]), A::scale(s.ad_value(829), p.p783)), 830, p.p964);

        s.store_add_scaled_ad_lhs(683, A::add(A::offset(A::scale(s.ad_value(828), p.p581), s.v[334]), A::scale(s.ad_value(829), p.p762)), 830, p.p943);

        s.store_add_scaled_ad_lhs(684, A::add(A::offset(A::scale(s.ad_value(828), p.p582), s.v[335]), A::scale(s.ad_value(829), p.p763)), 830, p.p944);

        s.store_add_scaled_ad_lhs(685, A::add(A::offset(A::scale(s.ad_value(828), p.p583), s.v[351]), A::scale(s.ad_value(829), p.p764)), 830, p.p945);

        s.store_add_scaled_ad_lhs(722, A::add(A::offset(A::scale(s.ad_value(828), p.p584), s.v[347]), A::scale(s.ad_value(829), p.p765)), 830, p.p946);

        s.store_mul_powf_ad_rhs(722, 722, A::scale(s.ad_value(478), 5e-17), (-0.25));

        s.store_add_scaled_ad_lhs(723, A::add(A::offset(A::scale(s.ad_value(828), p.p585), s.v[348]), A::scale(s.ad_value(829), p.p766)), 830, p.p947);

        s.store_add_scaled_ad_lhs(724, A::add(A::offset(A::scale(s.ad_value(828), p.p586), s.v[349]), A::scale(s.ad_value(829), p.p767)), 830, p.p948);

        s.store_add_scaled_ad_lhs(725, A::add(A::offset(A::scale(s.ad_value(828), p.p587), s.v[350]), A::scale(s.ad_value(829), p.p768)), 830, p.p949);

        s.store_add_scaled_ad_lhs(739, A::add(A::offset(A::scale(s.ad_value(828), p.p246), s.v[266]), A::scale(s.ad_value(829), p.p247)), 830, p.p248);

        s.store_add_scaled_ad_lhs(740, A::add(A::offset(A::scale(s.ad_value(828), p.p250), s.v[267]), A::scale(s.ad_value(829), p.p251)), 830, p.p252);

        s.store_add_scaled_ad_lhs(741, A::add(A::offset(A::scale(s.ad_value(828), p.p254), s.v[268]), A::scale(s.ad_value(829), p.p255)), 830, p.p256);

        s.store_add_scaled_ad_lhs(742, A::add(A::offset(A::scale(s.ad_value(828), p.p258), s.v[269]), A::scale(s.ad_value(829), p.p259)), 830, p.p260);

        s.store_add_scaled_ad_lhs(743, A::add(A::offset(A::scale(s.ad_value(828), p.p262), s.v[270]), A::scale(s.ad_value(829), p.p263)), 830, p.p264);

        s.store_add_scaled_ad_lhs(744, A::add(A::offset(A::scale(s.ad_value(828), p.p266), s.v[271]), A::scale(s.ad_value(829), p.p267)), 830, p.p268);

        s.store_add_scaled_ad_lhs(750, A::add(A::offset(A::scale(s.ad_value(828), p.p415), s.v[407]), A::scale(s.ad_value(829), p.p416)), 830, p.p417);

        s.store_add_scaled_ad_lhs(751, A::add(A::offset(A::scale(s.ad_value(828), p.p419), s.v[408]), A::scale(s.ad_value(829), p.p420)), 830, p.p421);

        s.store_add_scaled_ad_lhs(746, A::add(A::offset(A::scale(s.ad_value(828), p.p273), s.v[275]), A::scale(s.ad_value(829), p.p276)), 830, p.p279);

        s.store_add_scaled_ad_lhs(747, A::add(A::offset(A::scale(s.ad_value(828), p.p274), s.v[272]), A::scale(s.ad_value(829), p.p277)), 830, p.p280);

        s.store_add_scaled_ad_lhs(748, A::add(A::offset(A::scale(s.ad_value(828), p.p275), s.v[274]), A::scale(s.ad_value(829), p.p278)), 830, p.p281);

        s.store_add_scaled_ad_lhs(613, A::add(A::offset(A::scale(s.ad_value(828), p.p427), s.v[371]), A::scale(s.ad_value(829), p.p608)), 830, p.p789);

        s.store_add_scaled_ad_lhs(614, A::add(A::offset(A::scale(s.ad_value(828), p.p428), s.v[372]), A::scale(s.ad_value(829), p.p609)), 830, p.p790);

        s.store_add_scaled_ad_lhs(615, A::add(A::offset(A::scale(s.ad_value(828), p.p429), s.v[373]), A::scale(s.ad_value(829), p.p610)), 830, p.p791);

        s.store_offset_scaled_ad(745, A::atan(s.ad_value(744)), 0.3183098861837907, 0.5);

        s.store_offset_scaled_ad(749, A::atan(s.ad_value(750)), 0.3183098861837907, 0.5);

        s.v[818] = (s.v[827] - 1.0);

        s.copy_ad(523, 508);

        s.copy_ad(524, 510);

        s.copy_ad(525, 512);

        s.store_pow_from_scalar_ad(529, (s.v[689] * 1000000.0), s.ad_value(565));

        s.v[528] = ((s.v[51] * (s.v[39] * (s.v[689] + s.v[358]))) / s.v[59]);

        s.b[897] = (s.v[329] == 0.0);
        s.v[897] = if s.b[897] { 1.0 } else { 0.0 };

        if s.b[897] {
            s.store_scalar(526, 0.0);
        }

        if (!s.b[897]) {
            s.store_scalar(526, ((((((s.v[53] * s.v[329]) * s.v[359]) / ((2.0 * s.v[329]) + (s.v[359] * s.v[688]))) * s.v[689]) / s.v[59]) / s.v[39]));
        }

        s.v[706] = (((((s.v[361] / s.v[357])) as f64).powf(s.v[360]) / s.v[357]) / s.v[357]);

        s.store_add_ad_rhs(508, 508, A::scale(s.ad_value(509), s.v[818]));

        s.store_add_ad_rhs(510, 510, A::scale(s.ad_value(511), s.v[818]));

        s.store_add_ad_rhs(512, 512, A::scale(s.ad_value(513), s.v[818]));

        s.b[898] = (s.v[514] > 1.0);
        s.v[898] = if s.b[898] { 1.0 } else { 0.0 };

        if s.b[898] {
            s.store_scale(514, 514, 0.0001);
        }

        s.store_mul_ad_rhs(698, 514, A::pow_from_scalar(s.v[827], s.ad_value(515)));

        s.store_sub_ad_rhs(699, 471, A::scale(s.ad_value(472), s.v[818]));

        s.store_div_ad_lhs(552, A::add(s.ad_value(551), A::scale(s.ad_value(555), s.v[818])), 529);

        s.b[899] = (s.v[403] == 1.0);
        s.v[899] = if s.b[899] { 1.0 } else { 0.0 };

        if s.b[899] {
            s.store_scale(848, 529, s.v[39]);
        }

    }

    pub(super) fn stamp_reactive_block_3(
        s: &mut ReactiveScratch,
        param_given: &[bool; Instance::PARAMETER_COUNT],
    ) {
        if s.b[899] {
            s.store_scale(849, 555, s.v[818]);
            s.store_add(819, 539, 849);
            s.store_offset(820, 849, s.v[160]);
        }

        s.b[900] = (s.v[819] < 0.0);
        s.v[900] = if s.b[900] { 1.0 } else { 0.0 };

        if (s.b[899] && s.b[900]) {
            s.store_scalar(819, 0.0);
        }

        s.b[901] = (s.v[820] < 0.0);
        s.v[901] = if s.b[901] { 1.0 } else { 0.0 };

        if (s.b[899] && s.b[901]) {
            s.store_scalar(820, 0.0);
        }

        if s.b[899] {
            s.store_div(543, 819, 848);
            s.store_add(821, 540, 849);
            s.store_offset(822, 849, s.v[159]);
        }

        s.b[902] = (s.v[821] < 0.0);
        s.v[902] = if s.b[902] { 1.0 } else { 0.0 };

        if (s.b[899] && s.b[902]) {
            s.store_scalar(821, 0.0);
        }

        s.b[903] = (s.v[822] < 0.0);
        s.v[903] = if s.b[903] { 1.0 } else { 0.0 };

        if (s.b[899] && s.b[903]) {
            s.store_scalar(822, 0.0);
        }

        if s.b[899] {
            s.store_div(544, 821, 848);
        }

        if (!s.b[899]) {
            s.store_scalar(543, 0.0);
            s.store_scalar(544, 0.0);
        }

        s.b[904] = (s.v[152] < 0.0);
        s.v[904] = if s.b[904] { 1.0 } else { 0.0 };

        if s.b[904] {
            s.store_scalar(152, 0.0);
        }

        s.b[905] = (s.v[151] < 0.0);
        s.v[905] = if s.b[905] { 1.0 } else { 0.0 };

        if s.b[905] {
            s.store_scalar(151, 0.0);
        }

        s.b[906] = (s.v[331] < 0.0);
        s.v[906] = if s.b[906] { 1.0 } else { 0.0 };

        if s.b[906] {
            s.store_scalar(331, 0.0);
        }

        s.store_scaled_add(696, 152, 605, s.v[710]);

        s.store_scaled_add(695, 151, 605, s.v[711]);

        s.store_scale(697, 331, (s.v[692] * s.v[39]));

        s.b[907] = ((!param_given[81]) && param_given[84]);
        s.v[907] = if s.b[907] { 1.0 } else { 0.0 };

        if s.b[907] {
            s.store_scale(818, 757, s.v[482]);
            s.store_scaled_square(478, 818, 3.021e22);
        }

        s.b[908] = (s.v[57] == 2.0);
        s.v[908] = if s.b[908] { 1.0 } else { 0.0 };

        if (s.b[908] && (s.v[68] != 0.0)) {
            s.store_scale(794, 778, ((((s.v[76] - 0.1) / 1.60219e-19) * 2e-6) * 1.0 / ((s.v[175] * s.v[175]))));
        }

        s.b[909] = (s.v[478] > s.v[794]);
        s.v[909] = if s.b[909] { 1.0 } else { 0.0 };

        if ((s.b[908] && (s.v[68] != 0.0)) && s.b[909]) {
            s.copy_ad(478, 794);
        }

        if (s.b[908] && (s.v[68] == 0.0)) {
            s.store_scale(794, 778, ((((1.12 - 0.1) / 1.60219e-19) * 2e-6) * 1.0 / ((s.v[174] * s.v[174]))));
        }

        s.b[910] = (s.v[478] > s.v[794]);
        s.v[910] = if s.b[910] { 1.0 } else { 0.0 };

        if ((s.b[908] && (s.v[68] == 0.0)) && s.b[910]) {
            s.copy_ad(478, 794);
        }

        s.v[753] = (3.453133e-11 / s.v[173]);

        if (s.v[68] != 0.0) {
            s.store_scalar(754, (1.03594e-10 / s.v[175]));
        }

        if (s.v[68] == 0.0) {
            s.store_scalar(754, (1.03594e-10 / s.v[174]));
        }

        if (s.v[68] != 0.0) {
            s.store_scale(792, 478, (1.60219e-19 * ((1.0 + (s.v[124] / s.v[37])) * (1000000.0 * s.v[175]))));
        }

        if (s.v[68] == 0.0) {
            s.store_scale(792, 478, (1.60219e-19 * ((1.0 + (s.v[124] / s.v[37])) * (1000000.0 * s.v[174]))));
        }

        s.store_add_ad_lhs(793, A::sub_from_scalar(0.8, A::div(A::scale(s.ad_value(792), 0.5), s.ad_value(754))), 582);

        s.b[911] = (s.v[57] == 3.0);
        s.v[911] = if s.b[911] { 1.0 } else { 0.0 };

        s.b[912] = (s.v[793] > s.v[594]);
        s.v[912] = if s.b[912] { 1.0 } else { 0.0 };

        if (s.b[911] && s.b[912]) {
            s.store_scalar(57, 2.0);
        }

        s.b[913] = (s.v[793] < s.v[593]);
        s.v[913] = if s.b[913] { 1.0 } else { 0.0 };

        if ((s.b[911] && (!s.b[912])) && s.b[913]) {
            s.store_scalar(57, 0.0);
        }

        if ((s.b[911] && (!s.b[912])) && (!s.b[913])) {
            s.store_scalar(57, 1.0);
        }

        s.store_scale_ad(822, A::div_from_scalar(1.115, s.ad_value(409)), (s.v[827] - 1.0));

        s.store_div_ad_lhs(884, A::mul(s.ad_value(619), s.ad_value(822)), 661);

        s.b[914] = (s.v[884] > 100.0);
        s.v[914] = if s.b[914] { 1.0 } else { 0.0 };

        if s.b[914] {
            s.store_scaled_offset_ad(818, A::offset(s.ad_value(884), 1.0), (-100.0), 2.688117142e43);
        }

        s.b[915] = (s.v[884] < (-100.0));
        s.v[915] = if s.b[915] { 1.0 } else { 0.0 };

        if ((!s.b[914]) && s.b[915]) {
            s.store_scalar(818, 3.720075976e-44);
        }

        if ((!s.b[914]) && (!s.b[915])) {
            s.store_exp(818, 884);
        }

        s.store_div_ad_lhs(884, A::mul(s.ad_value(620), s.ad_value(822)), 661);

        s.b[916] = (s.v[884] > 100.0);
        s.v[916] = if s.b[916] { 1.0 } else { 0.0 };

        if s.b[916] {
            s.store_scaled_offset_ad(819, A::offset(s.ad_value(884), 1.0), (-100.0), 2.688117142e43);
        }

        s.b[917] = (s.v[884] < (-100.0));
        s.v[917] = if s.b[917] { 1.0 } else { 0.0 };

        if ((!s.b[916]) && s.b[917]) {
            s.store_scalar(819, 3.720075976e-44);
        }

        if ((!s.b[916]) && (!s.b[917])) {
            s.store_exp(819, 884);
        }

        s.store_div_ad_lhs(884, A::mul(s.ad_value(621), s.ad_value(822)), 663);

        s.b[918] = (s.v[884] > 100.0);
        s.v[918] = if s.b[918] { 1.0 } else { 0.0 };

        if s.b[918] {
            s.store_scaled_offset_ad(820, A::offset(s.ad_value(884), 1.0), (-100.0), 2.688117142e43);
        }

        s.b[919] = (s.v[884] < (-100.0));
        s.v[919] = if s.b[919] { 1.0 } else { 0.0 };

        if ((!s.b[918]) && s.b[919]) {
            s.store_scalar(820, 3.720075976e-44);
        }

        if ((!s.b[918]) && (!s.b[919])) {
            s.store_exp(820, 884);
        }

        s.store_mul(718, 716, 818);

        s.store_mul(531, 667, 818);

        s.store_mul(533, 669, 819);

        s.store_mul(535, 671, 820);

        s.store_scale(884, 622, (s.v[827] - 1.0));

        s.b[920] = (s.v[884] > 100.0);
        s.v[920] = if s.b[920] { 1.0 } else { 0.0 };

        if s.b[920] {
            s.store_scaled_offset_ad(818, A::offset(s.ad_value(884), 1.0), (-100.0), 2.688117142e43);
        }

        s.b[921] = (s.v[884] < (-100.0));
        s.v[921] = if s.b[921] { 1.0 } else { 0.0 };

        if ((!s.b[920]) && s.b[921]) {
            s.store_scalar(818, 3.720075976e-44);
        }

        if ((!s.b[920]) && (!s.b[921])) {
            s.store_exp(818, 884);
        }

        s.store_mul(537, 673, 818);

        s.store_div_ad_lhs(884, A::mul(s.ad_value(619), s.ad_value(822)), 662);

        s.b[922] = (s.v[884] > 100.0);
        s.v[922] = if s.b[922] { 1.0 } else { 0.0 };

        if s.b[922] {
            s.store_scaled_offset_ad(818, A::offset(s.ad_value(884), 1.0), (-100.0), 2.688117142e43);
        }

        s.b[923] = (s.v[884] < (-100.0));
        s.v[923] = if s.b[923] { 1.0 } else { 0.0 };

        if ((!s.b[922]) && s.b[923]) {
            s.store_scalar(818, 3.720075976e-44);
        }

        if ((!s.b[922]) && (!s.b[923])) {
            s.store_exp(818, 884);
        }

        s.store_div_ad_lhs(884, A::mul(s.ad_value(623), s.ad_value(822)), 662);

        s.b[924] = (s.v[884] > 100.0);
        s.v[924] = if s.b[924] { 1.0 } else { 0.0 };

        if s.b[924] {
            s.store_scaled_offset_ad(819, A::offset(s.ad_value(884), 1.0), (-100.0), 2.688117142e43);
        }

        s.b[925] = (s.v[884] < (-100.0));
        s.v[925] = if s.b[925] { 1.0 } else { 0.0 };

        if ((!s.b[924]) && s.b[925]) {
            s.store_scalar(819, 3.720075976e-44);
        }

        if ((!s.b[924]) && (!s.b[925])) {
            s.store_exp(819, 884);
        }

        s.store_div_ad_lhs(884, A::mul(s.ad_value(624), s.ad_value(822)), 664);

        s.b[926] = (s.v[884] > 100.0);
        s.v[926] = if s.b[926] { 1.0 } else { 0.0 };

        if s.b[926] {
            s.store_scaled_offset_ad(820, A::offset(s.ad_value(884), 1.0), (-100.0), 2.688117142e43);
        }

        s.b[927] = (s.v[884] < (-100.0));
        s.v[927] = if s.b[927] { 1.0 } else { 0.0 };

        if ((!s.b[926]) && s.b[927]) {
            s.store_scalar(820, 3.720075976e-44);
        }

        if ((!s.b[926]) && (!s.b[927])) {
            s.store_exp(820, 884);
        }

        s.store_mul(719, 717, 818);

        s.store_mul(532, 668, 818);

        s.store_mul(534, 670, 819);

        s.store_mul(536, 672, 820);

        s.store_scale(884, 625, (s.v[827] - 1.0));

        s.b[928] = (s.v[884] > 100.0);
        s.v[928] = if s.b[928] { 1.0 } else { 0.0 };

        if s.b[928] {
            s.store_scaled_offset_ad(818, A::offset(s.ad_value(884), 1.0), (-100.0), 2.688117142e43);
        }

        s.b[929] = (s.v[884] < (-100.0));
        s.v[929] = if s.b[929] { 1.0 } else { 0.0 };

        if ((!s.b[928]) && s.b[929]) {
            s.store_scalar(818, 3.720075976e-44);
        }

        if ((!s.b[928]) && (!s.b[929])) {
            s.store_exp(818, 884);
        }

        s.store_mul(538, 674, 818);

        s.b[930] = (s.v[479] > 0.0);
        s.v[930] = if s.b[930] { 1.0 } else { 0.0 };

        if s.b[930] {
            s.store_mul_scaled_ad_rhs(530, 409, (-s.v[36]), {
                if ((s.v[478] / s.v[479]) > 1e-38) {
                    A::ln(A::div(s.ad_value(478), s.ad_value(479)))
                } else {
                    A::neg(A::constant(87.49823353377374))
                }
            });
        }

        if (!s.b[930]) {
            s.store_mul_scaled_ad_rhs(530, 409, (-s.v[36]), {
                if (((((-s.v[478]) * s.v[479]) / s.v[817]) / s.v[817]) > 1e-38) {
                    A::ln(A::div(A::div(A::mul(A::neg(s.ad_value(478)), s.ad_value(479)), s.ad_value(817)), s.ad_value(817)))
                } else {
                    A::neg(A::constant(87.49823353377374))
                }
            });
        }

        s.b[931] = (!param_given[340]);
        s.v[931] = if s.b[931] { 1.0 } else { 0.0 };

        s.b[932] = (s.v[479] > 0.0);
        s.v[932] = if s.b[932] { 1.0 } else { 0.0 };

        if (s.b[931] && s.b[932]) {
            s.store_scaled_offset_ad(683, A::mul(s.ad_value(409), {
                if ((((1e20 * s.v[479]) / s.v[817]) / s.v[817]) > 1e-38) {
                    A::ln(A::div(A::div(A::scale(s.ad_value(479), 1e20), s.ad_value(817)), s.ad_value(817)))
                } else {
                    A::neg(A::constant(87.49823353377374))
                }
            }), (-0.3), (-s.v[36]));
        }

        s.b[933] = (s.v[479] < 0.0);
        s.v[933] = if s.b[933] { 1.0 } else { 0.0 };

        if ((s.b[931] && (!s.b[932])) && s.b[933]) {
            s.store_scaled_offset_ad(683, A::mul(s.ad_value(409), {
                if (((-1e20) / s.v[479]) > 1e-38) {
                    A::ln(A::div_from_scalar((-1e20), s.ad_value(479)))
                } else {
                    A::neg(A::constant(87.49823353377374))
                }
            }), 0.3, (-s.v[36]));
        }

        s.store_mul_scaled_ad_rhs(833, 409, 2.0, {
            if ((((s.v[479]) as f64).abs() / s.v[817]) > 1e-38) {
                A::ln(A::div(A::abs(s.ad_value(479)), s.ad_value(817)))
            } else {
                A::neg(A::constant(87.49823353377374))
            }
        });

        s.store_mul_scaled_ad_rhs(834, 780, 1.0 / (s.v[753]), A::sqrt(A::abs(s.ad_value(479))));

        s.b[934] = (!param_given[341]);
        s.v[934] = if s.b[934] { 1.0 } else { 0.0 };

        s.b[935] = (((s.v[479] > 0.0) && (s.v[36] > 0.0)) || ((s.v[479] < 0.0) && (s.v[36] < 0.0)));
        s.v[935] = if s.b[935] { 1.0 } else { 0.0 };

        if (s.b[934] && s.b[935]) {
            s.store_add_ad(684, A::add(s.ad_value(683), s.ad_value(833)), A::mul(s.ad_value(834), A::sqrt(s.ad_value(833))));
        }

        if (s.b[934] && (!s.b[935])) {
            s.store_sub_ad(684, A::sub(s.ad_value(683), s.ad_value(833)), A::mul(s.ad_value(834), A::sqrt(s.ad_value(833))));
        }

        s.b[936] = (!param_given[342]);
        s.v[936] = if s.b[936] { 1.0 } else { 0.0 };

        if s.b[936] {
            s.store_sqrt_div_ad(812, A::mul(A::scale(s.ad_value(778), 2.0), s.ad_value(833)), A::scale(A::abs(s.ad_value(479)), (1.60219e-19 * 1000000.0)));
            s.store_div(813, 778, 812);
            s.store_div_ad(336, A::scale(s.ad_value(813), s.v[753]), A::offset(s.ad_value(813), s.v[753]));
        }

        s.store_mul_scaled_ad_rhs(488, 409, 2.0, {
            if ((s.v[478] / s.v[817]) > 1e-38) {
                A::ln(A::div(s.ad_value(478), s.ad_value(817)))
            } else {
                A::neg(A::constant(87.49823353377374))
            }
        });

        s.store_sqrt(700, 488);

        s.store_mul_sqrt_ad_lhs(701, A::div(A::scale(s.ad_value(778), 2.0), A::scale(s.ad_value(478), (1.60219e-19 * 1000000.0))), 700);

        s.store_sqrt(702, 701);

        s.b[937] = (s.v[68] == 0.0);
        s.v[937] = if s.b[937] { 1.0 } else { 0.0 };

        if s.b[937] {
            s.store_sqrt_scaled_ad(489, A::mul(A::div_from_scalar((3.0 * 3.9), s.ad_value(777)), s.ad_value(608)), s.v[91]);
        }

        if (!s.b[937]) {
            s.store_sqrt_div_ad(489, A::mul(A::mul(s.ad_value(778), s.ad_value(608)), s.ad_value(776)), A::scale(s.ad_value(777), 8.85418e-12));
        }

        s.store_mul_ad_rhs(485, 409, {
            if (((1e20 * s.v[478]) / (s.v[817] * s.v[817])) > 1e-38) {
                A::ln(A::div(A::scale(s.ad_value(478), 1e20), A::square(s.ad_value(817))))
            } else {
                A::neg(A::constant(87.49823353377374))
            }
        });

        s.store_sqrt_div_ad(728, A::scale(A::mul(A::scale(s.ad_value(778), 1.60219e-19), s.ad_value(478)), (1000000.0 * 0.5)), s.ad_value(488));

        s.b[938] = (s.v[68] == 0.0);
        s.v[938] = if s.b[938] { 1.0 } else { 0.0 };

        s.b[939] = (s.v[480] > 0.0);
        s.v[939] = if s.b[939] { 1.0 } else { 0.0 };

        if (s.b[938] && s.b[939]) {
            s.store_mul_ad_rhs(736, 831, {
                if ((s.v[480] / 1e20) > 1e-38) {
                    A::ln(A::scale(s.ad_value(480), 1e-20))
                } else {
                    A::neg(A::constant(87.49823353377374))
                }
            });
        }

        if (s.b[938] && (!s.b[939])) {
            s.store_scalar(736, 0.0);
        }

        if (!s.b[938]) {
            s.store_mul_ad_rhs(818, 831, {
                if ((s.v[481] / s.v[817]) > 1e-38) {
                    A::ln(A::div(s.ad_value(481), s.ad_value(817)))
                } else {
                    A::neg(A::constant(87.49823353377374))
                }
            });
        }

        if (!s.b[938]) {
            s.store_scale(819, 816, 0.5);
        }

    }

    pub(super) fn stamp_reactive_block_4(
        s: &mut ReactiveScratch,
        p: &Parameters,
        param_given: &[bool; Instance::PARAMETER_COUNT],
    ) {
        s.b[940] = (s.v[818] > s.v[819]);
        s.v[940] = if s.b[940] { 1.0 } else { 0.0 };

        if ((!s.b[938]) && s.b[940]) {
            s.copy_ad(818, 819);
        }

        if (!s.b[938]) {
            s.store_sub_scaled_ad_lhs(820, A::offset(s.ad_value(819), s.v[80]), 818, s.v[36]);
            s.store_sub_from_scalar(736, s.v[79], 820);
        }

        s.v[729] = (((((s.v[360] * (if ((s.v[361] / s.v[357]) > 1e-38) { (((s.v[361] / s.v[357])) as f64).ln() } else { (-87.49823353377374) }))) as f64).exp() / s.v[357]) / s.v[357]);

        s.store_div_ad_lhs(732, A::div(A::scale(A::scale(A::exp(A::scale({
            if ((s.v[361] / (s.v[357] * s.v[580])) > 1e-38) {
                A::ln(A::div_from_scalar(s.v[361], A::scale(s.ad_value(580), s.v[357])))
            } else {
                A::neg(A::constant(87.49823353377374))
            }
        }, s.v[360])), 1.0 / (s.v[357])), 1.0 / (s.v[357])), s.ad_value(580)), 580);

        if (s.v[36] == 1.0) {
            s.copy_ad(730, 789);
        } else {
            s.copy_ad(730, 788);
        }

        if (s.v[36] == 1.0) {
            s.copy_ad(731, 791);
        } else {
            s.copy_ad(731, 790);
        }

        s.store_mul_ad_product_lhs(733, A::scale(s.ad_value(730), ((s.v[689] / s.v[59]) + s.v[61])), s.ad_value(581), 732);

        s.store_mul_ad_product_lhs(734, A::scale(s.ad_value(730), ((s.v[689] / s.v[59]) + s.v[60])), s.ad_value(581), 732);

        s.store_mul_scale_ad_lhs(735, A::neg(s.ad_value(731)), s.v[357], 580);

        s.store_scale(730, 730, (s.v[729] * (((s.v[689] / s.v[59]) * s.v[688]) + (s.v[64] / s.v[39]))));

        s.store_scale(731, 731, (-s.v[357]));

        s.b[941] = (param_given[89] || param_given[93]);
        s.v[941] = if s.b[941] { 1.0 } else { 0.0 };

        s.b[942] = (!param_given[89]);
        s.v[942] = if s.b[942] { 1.0 } else { 0.0 };

        if (s.b[941] && s.b[942]) {
            s.store_scalar(490, 0.53);
        }

        s.b[943] = (!param_given[93]);
        s.v[943] = if s.b[943] { 1.0 } else { 0.0 };

        if (s.b[941] && s.b[943]) {
            s.store_scalar(494, (-0.0186));
        }

        s.b[949] = (!param_given[86]);
        s.v[949] = if s.b[949] { 1.0 } else { 0.0 };

        if (((!s.b[941]) && s.b[949]) && (s.v[68] != 0.0)) {
            s.store_scaled_div_from_scalar_ad(818, 1.60219e-19, A::scale(s.ad_value(778), 2.0), 1000000.0);
        }

        if (((!s.b[941]) && s.b[949]) && (s.v[68] == 0.0)) {
            s.store_scalar(818, 0.00077348);
        }

        if ((!s.b[941]) && s.b[949]) {
            s.store_sub_ad_rhs(484, 488, A::scale(A::mul(s.ad_value(818), s.ad_value(478)), (s.v[487] * s.v[487])));
        }

        s.b[950] = (s.v[484] > 0.0);
        s.v[950] = if s.b[950] { 1.0 } else { 0.0 };

        if ((!s.b[941]) && s.b[950]) {
            s.store_neg(484, 484);
        }

        s.b[951] = (s.v[486] > 0.0);
        s.v[951] = if s.b[951] { 1.0 } else { 0.0 };

        if ((!s.b[941]) && s.b[951]) {
            s.store_scalar(486, (-s.v[486]));
        }

        s.b[952] = (!param_given[84]);
        s.v[952] = if s.b[952] { 1.0 } else { 0.0 };

        if ((!s.b[941]) && s.b[952]) {
            s.store_div_ad_lhs(482, A::mul(s.ad_value(780), A::sqrt(s.ad_value(478))), 757);
        }

        s.b[953] = (!param_given[85]);
        s.v[953] = if s.b[953] { 1.0 } else { 0.0 };

        if ((!s.b[941]) && s.b[953]) {
            s.store_div_ad_lhs(483, A::mul(s.ad_value(780), A::sqrt(s.ad_value(479))), 757);
        }

        if (!s.b[941]) {
            s.store_sub(818, 482, 483);
            s.store_sub_ad_lhs(819, A::sqrt(A::sub(s.ad_value(488), s.ad_value(484))), 700);
            s.store_mul_sub_ad_rhs(820, 700, A::sqrt(A::sub(s.ad_value(488), s.ad_value(486))), s.ad_value(700));
            s.store_div_ad(494, A::mul(s.ad_value(818), s.ad_value(819)), A::add(A::scale(s.ad_value(820), 2.0), s.ad_value(486)));
            s.store_sub_ad_rhs(490, 483, A::mul(A::scale(s.ad_value(494), 2.0), A::sqrt(A::sub(s.ad_value(488), s.ad_value(486)))));
        }

        s.store_offset(818, 628, s.v[689]);

        s.b[954] = (s.v[818] < 1e-8);
        s.v[954] = if s.b[954] { 1.0 } else { 0.0 };

        if s.b[954] {
            s.store_scalar(818, 1e-8);
        }

        s.store_mul_offset_ad_rhs(707, 490, A::div(s.ad_value(627), s.ad_value(818)), 1.0);

        s.b[955] = (!param_given[108]);
        s.v[955] = if s.b[955] { 1.0 } else { 0.0 };

        s.b[956] = (param_given[107] || param_given[106]);
        s.v[956] = if s.b[956] { 1.0 } else { 0.0 };

        if (s.b[955] && s.b[956]) {
            s.store_sub_ad(522, A::sub(A::scale(s.ad_value(507), s.v[36]), s.ad_value(488)), A::mul(s.ad_value(707), s.ad_value(700)));
        }

        if (s.b[955] && (!s.b[956])) {
            s.store_scalar(522, (-1.0));
        }

        s.b[957] = (!param_given[107]);
        s.v[957] = if s.b[957] { 1.0 } else { 0.0 };

        if s.b[957] {
            s.store_scaled_add_ad(507, A::add(s.ad_value(522), s.ad_value(488)), A::mul(s.ad_value(707), s.ad_value(700)), s.v[36]);
        }

        s.store_scale(737, 707, (s.v[91] * 1.0 / (s.v[93])));

        s.store_mul(819, 758, 702);

        s.store_exp_ad(818, A::div(A::scale(s.ad_value(506), ((-0.5) * s.v[688])), s.ad_value(819)));

        s.store_add_ad_rhs(703, 818, A::mul(A::scale(s.ad_value(818), 2.0), s.ad_value(818)));

        s.store_exp_ad(818, A::div(A::scale(s.ad_value(505), ((-0.5) * s.v[688])), s.ad_value(819)));

        s.store_add_ad_rhs(820, 818, A::mul(A::scale(s.ad_value(818), 2.0), s.ad_value(818)));

        s.store_add_ad_lhs(704, A::mul(s.ad_value(561), s.ad_value(820)), 562);

        s.store_div_ad_rhs(752, 741, A::exp(A::scale(s.ad_value(742), (if (s.v[688] > 1e-38) { ((s.v[688]) as f64).ln() } else { (-87.49823353377374) }))));

        s.b[958] = (s.v[248] < 0.0);
        s.v[958] = if s.b[958] { 1.0 } else { 0.0 };

        if s.b[958] {
            s.store_scalar(248, 0.0);
        }

        s.v[818] = ((s.v[825]) as f64).powf(s.v[253]);

        s.store_offset(841, 248, s.v[826]);

        s.store_powf(819, 841, s.v[254]);

        s.store_add_ad(813, A::offset(A::div_from_scalar(p.p231, s.ad_value(819)), (p.p230 / s.v[818])), A::div_from_scalar(p.p232, A::scale(s.ad_value(819), s.v[818])));

        s.store_offset(597, 813, 1.0);

        s.v[818] = ((s.v[825]) as f64).powf(s.v[255]);

        s.store_powf(819, 841, s.v[256]);

        s.store_add_ad(813, A::offset(A::div_from_scalar(p.p234, s.ad_value(819)), (p.p233 / s.v[818])), A::div_from_scalar(p.p235, A::scale(s.ad_value(819), s.v[818])));

        s.store_offset(598, 813, 1.0);

        s.store_sqrt_square_offset(598, 598, 1e-9);

        s.v[818] = (s.v[827] - 1.0);

        s.store_offset_scaled(599, 597, (1.0 + (s.v[252] * s.v[818])), 1e-9);

        s.v[835] = (1.0 / (s.v[246] + (0.5 * s.v[825])));

        s.v[836] = (1.0 / (s.v[247] + (0.5 * s.v[825])));

        s.v[601] = (s.v[835] + s.v[836]);

        s.store_scale_ad(600, A::div_from_scalar(s.v[249], s.ad_value(599)), s.v[601]);

        s.b[959] = (((s.v[40] > 0.0) && (s.v[41] > 0.0)) && ((s.v[39] == 1.0) || ((s.v[39] > 1.0) && (s.v[42] > 0.0))));
        s.v[959] = if s.b[959] { 1.0 } else { 0.0 };

        if s.b[959] {
            s.store_scalar(837, 0.0);
            s.store_scalar(838, 0.0);
        }

        s.b[960] = (s.v[250] < (-1.0));
        s.v[960] = if s.b[960] { 1.0 } else { 0.0 };

        if (s.b[959] && s.b[960]) {
            s.store_scalar(250, (-1.0));
        }

        s.b[961] = (s.v[250] > 1.0);
        s.v[961] = if s.b[961] { 1.0 } else { 0.0 };

        if ((s.b[959] && (!s.b[960])) && s.b[961]) {
            s.store_scalar(250, 1.0);
        }

        if ((s.b[959] && (!s.b[960])) && (!s.b[961])) {
        }

        if s.b[959] {
            s.store_scalar(847, 0.0);
        }

        let mut assign9560_loop_guard: usize = 0;
        while {
            let assign9560_cond_e6904: f64 = if (s.b[959] && (s.v[847] < s.v[39])) { 1.0 } else { 0.0 };
            assign9560_cond_e6904 != 0.0
        } {
            assign9560_loop_guard += 1;
            assert!(assign9560_loop_guard <= Self::MAX_ANALOG_LOOP_ITERATIONS, "generated Verilog-A analog loop exceeded iteration guard");
            if s.b[959] {
                s.store_div_from_scalar_offset_scaled_input(962, (1.0 / s.v[39]), 847, (s.v[42] + s.v[825]), (s.v[40] + (0.5 * s.v[825])));
                s.store_div_from_scalar_offset_scaled_input(963, (1.0 / s.v[39]), 847, (s.v[42] + s.v[825]), (s.v[41] + (0.5 * s.v[825])));
                s.store_add(837, 837, 962);
                s.store_add(838, 838, 963);
                s.store_offset(847, 847, 1.0);
            }
        }

        if s.b[959] {
            s.store_add(842, 837, 838);
            s.copy_ad(414, 842);
            s.store_mul_div_from_scalar_lhs(839, s.v[249], 599, 842);
            s.store_div_ad(818, A::offset(s.ad_value(839), 1.0), A::offset(s.ad_value(600), 1.0));
            s.store_mul(765, 698, 818);
            s.store_div_ad(819, A::offset(A::mul(s.ad_value(250), s.ad_value(839)), 1.0), A::offset(A::mul(s.ad_value(250), s.ad_value(600)), 1.0));
            s.store_mul(767, 699, 819);
            s.store_offset(843, 842, (-s.v[601]));
            s.store_mul_div_from_scalar_lhs(840, s.v[251], 598, 843);
            s.store_mul_div_from_scalar_ad_lhs(844, s.v[257], A::powf(s.ad_value(598), s.v[258]), 843);
            s.store_mul_div_from_scalar_ad_lhs(845, s.v[259], A::powf(s.ad_value(598), s.v[260]), 843);
            s.store_mul_div_from_scalar_ad_lhs(846, s.v[261], A::powf(s.ad_value(598), s.v[262]), 843);
            s.store_add(768, 507, 840);
            s.store_add(763, 494, 844);
            s.store_add(761, 556, 845);
            s.store_add(762, 558, 846);
        }

        if (!s.b[959]) {
            s.copy_ad(765, 698);
            s.copy_ad(768, 507);
            s.copy_ad(767, 699);
            s.copy_ad(763, 494);
            s.copy_ad(761, 556);
            s.copy_ad(762, 558);
            s.store_scalar(414, 0.0);
            s.store_scalar(601, 0.0);
            s.store_scalar(250, 0.0);
        }

        s.store_scale(764, 763, (s.v[91] * 1.0 / (s.v[93])));

        s.store_offset(768, 768, s.v[56]);

        s.store_offset(766, 522, (s.v[36] * s.v[56]));

        s.v[430] = (s.v[753] * s.v[44]);

        s.store_scale(432, 336, s.v[44]);

        s.v[431] = (s.v[753] * s.v[43]);

        s.store_scale(433, 336, s.v[43]);

        s.b[964] = (s.v[336] > 0.0);
        s.v[964] = if s.b[964] { 1.0 } else { 0.0 };

        s.b[965] = (((s.v[479] > 0.0) && (s.v[36] > 0.0)) || ((s.v[479] < 0.0) && (s.v[36] < 0.0)));
        s.v[965] = if s.b[965] { 1.0 } else { 0.0 };

        if (s.b[964] && s.b[965]) {
            s.store_sub(818, 684, 683);
            s.store_add_ad_rhs(545, 683, A::scale(s.ad_value(818), s.v[337]));
            s.store_sub_from_scalar(819, s.v[430], 432);
            s.store_div_ad_lhs(820, A::div(s.ad_value(819), s.ad_value(818)), 818);
            s.store_scale(546, 820, 1.0 / (s.v[337]));
            s.store_scale(547, 820, 1.0 / ((1.0 - s.v[337])));
            s.store_sub_ad(434, A::scale(A::mul(s.ad_value(818), s.ad_value(819)), ((1.0 + s.v[337]) * 0.3333333333333333)), A::mul(s.ad_value(432), s.ad_value(683)));
            s.store_sub_from_scalar(819, s.v[431], 433);
            s.store_div_ad_lhs(820, A::div(s.ad_value(819), s.ad_value(818)), 818);
            s.store_scale(548, 820, 1.0 / (s.v[337]));
            s.store_scale(549, 820, 1.0 / ((1.0 - s.v[337])));
            s.store_sub_ad(435, A::scale(A::mul(s.ad_value(818), s.ad_value(819)), ((1.0 + s.v[337]) * 0.3333333333333333)), A::mul(s.ad_value(433), s.ad_value(683)));
        }

        if (s.b[964] && (!s.b[965])) {
            s.store_sub(818, 683, 684);
            s.store_add_ad_rhs(545, 684, A::scale(s.ad_value(818), s.v[337]));
            s.store_offset(819, 432, (-s.v[430]));
            s.store_div_ad_lhs(820, A::div(s.ad_value(819), s.ad_value(818)), 818);
            s.store_scale(546, 820, 1.0 / (s.v[337]));
            s.store_scale(547, 820, 1.0 / ((1.0 - s.v[337])));
            s.store_sub_scaled_ad_lhs(434, A::scale(A::mul(s.ad_value(818), s.ad_value(819)), ((1.0 + s.v[337]) * 0.3333333333333333)), 684, s.v[430]);
            s.store_offset(819, 433, (-s.v[431]));
            s.store_div_ad_lhs(820, A::div(s.ad_value(819), s.ad_value(818)), 818);
            s.store_scale(548, 820, 1.0 / (s.v[337]));
            s.store_scale(549, 820, 1.0 / ((1.0 - s.v[337])));
            s.store_sub_scaled_ad_lhs(435, A::scale(A::mul(s.ad_value(818), s.ad_value(819)), ((1.0 + s.v[337]) * 0.3333333333333333)), 684, s.v[431]);
        }

        if (!s.b[964]) {
            s.store_scalar(545, 0.0);
            s.store_scalar(546, 0.0);
            s.store_scalar(547, 0.0);
            s.store_scalar(434, 0.0);
            s.store_scalar(548, 0.0);
            s.store_scalar(549, 0.0);
        }

    }

    pub(super) fn stamp_reactive_block_5(
        s: &mut ReactiveScratch,
    ) {
        if (!s.b[964]) {
            s.store_scalar(435, 0.0);
        }

        s.b[966] = ((s.v[354] < 1.0) || (s.v[354] > 2.0));
        s.v[966] = if s.b[966] { 1.0 } else { 0.0 };

        if s.b[966] {
            s.store_scalar(354, 1.0);
        }

        s.store_scale_ad(818, {
            if ((s.v[354] * (1.0 + (s.v[174] / s.v[173]))) > 1e-38) {
                A::ln(A::scale(s.ad_value(354), (1.0 + (s.v[174] / s.v[173]))))
            } else {
                A::neg(A::constant(87.49823353377374))
            }
        }, s.v[338]);

        s.v[819] = (s.v[46] - s.v[38]);

        s.b[967] = (s.v[819] > 0.0);
        s.v[967] = if s.b[967] { 1.0 } else { 0.0 };

        if s.b[967] {
            s.store_scale(428, 818, s.v[819]);
        }

        if (!s.b[967]) {
            s.store_scalar(428, 0.0);
        }

        s.v[819] = (s.v[45] - s.v[38]);

        s.b[968] = (s.v[819] > 0.0);
        s.v[968] = if s.b[968] { 1.0 } else { 0.0 };

        if s.b[968] {
            s.store_scale(429, 818, s.v[819]);
        }

        if (!s.b[968]) {
            s.store_scalar(429, 0.0);
        }

        s.v[423] = (s.v[155] * s.v[47]);

        s.b[969] = (s.v[423] <= 0.001);
        s.v[969] = if s.b[969] { 1.0 } else { 0.0 };

        if s.b[969] {
            s.store_scalar(423, 0.001);
        }

        s.v[422] = (s.v[155] * s.v[48]);

        s.b[970] = (s.v[422] <= 0.001);
        s.v[970] = if s.b[970] { 1.0 } else { 0.0 };

        if s.b[970] {
            s.store_scalar(422, 0.001);
        }

        s.b[971] = (s.v[317] < 1e-15);
        s.v[971] = if s.b[971] { 1.0 } else { 0.0 };

        if s.b[971] {
            s.store_scalar(317, 1e-15);
        }

        s.store_div_ad_lhs(818, A::div_from_scalar((((-0.5) * s.v[688]) * s.v[688]), s.ad_value(317)), 317);

        s.b[972] = (s.v[818] > 100.0);
        s.v[972] = if s.b[972] { 1.0 } else { 0.0 };

        if s.b[972] {
            s.store_scaled_offset_ad(819, A::offset(s.ad_value(818), 1.0), (-100.0), 2.688117142e43);
        }

        s.b[973] = (s.v[818] < (-100.0));
        s.v[973] = if s.b[973] { 1.0 } else { 0.0 };

        if ((!s.b[972]) && s.b[973]) {
            s.store_scalar(819, 3.720075976e-44);
        }

        if ((!s.b[972]) && (!s.b[973])) {
            s.store_exp(819, 818);
        }

        s.copy_ad(712, 819);

        s.store_mul_offset_ad_rhs(818, 680, A::div_from_scalar(1.0, s.ad_value(317)), (1.0 / s.v[688]));

        s.store_pow_ad(713, s.ad_value(818), s.ad_value(679));

        s.store_offset_scaled_ad(714, A::pow(s.ad_value(818), s.ad_value(616)), s.v[324], 1.0);

        s.store_add_ad_rhs(715, 681, A::scale(s.ad_value(682), s.v[688]));

        s.b[974] = (s.v[715] < 1.0);
        s.v[974] = if s.b[974] { 1.0 } else { 0.0 };

        if s.b[974] {
            s.store_scalar(715, 1.0);
        }

        s.b[975] = (s.v[68] == 0.0);
        s.v[975] = if s.b[975] { 1.0 } else { 0.0 };

        if s.b[975] {
            s.store_scalar(92, (s.v[91] - s.v[94]));
        }

        if (!s.b[975]) {
            s.store_scalar(850, (8.617087e-5 * s.v[84]));
            s.copy_ad(851, 850);
        }

        if (!s.b[975]) {
            s.store_mul_ad_rhs(852, 850, {
                if (((1e20 * s.v[478]) / (s.v[817] * s.v[817])) > 1e-38) {
                    A::ln(A::div(A::scale(s.ad_value(478), 1e20), A::square(s.ad_value(817))))
                } else {
                    A::neg(A::constant(87.49823353377374))
                }
            });
        }

        if (!s.b[975]) {
            s.store_mul_scaled_ad_rhs(853, 850, 2.0, {
                if ((s.v[478] / s.v[817]) > 1e-38) {
                    A::ln(A::div(s.ad_value(478), s.ad_value(817)))
                } else {
                    A::neg(A::constant(87.49823353377374))
                }
            });
        }

        if (!s.b[975]) {
            s.store_sqrt(854, 853);
            s.store_add(814, 766, 853);
            s.store_scalar(855, (s.v[36] * s.v[83]));
            s.store_scalar(818, (s.v[87] * 8.85418e-12));
        }

        s.b[976] = ((((s.v[480] > 1e18) && (s.v[480] < 1e25)) && (s.v[855] > s.v[814])) && (s.v[818] != 0.0));
        s.v[976] = if s.b[976] { 1.0 } else { 0.0 };

        if ((!s.b[975]) && s.b[976]) {
            s.store_div_ad(819, A::mul(A::scale(s.ad_value(778), (1000000.0 * 1.60219e-19)), s.ad_value(480)), A::square(s.ad_value(757)));
            s.store_sqrt_offset_ad(822, A::div(A::scale(A::sub(s.ad_value(855), s.ad_value(818)), 2.0), s.ad_value(819)), 1.0);
            s.store_mul_offset_rhs(820, 819, 822, (-1.0));
            s.store_div_ad_lhs(821, A::mul(A::scale(s.ad_value(820), 0.5), s.ad_value(820)), 819);
            s.store_offset_sub(884, 782, 821, (-0.05));
            s.store_sqrt_square_offset(824, 884, 0.224);
            s.store_sub_ad_rhs(823, 782, A::scale(A::add(s.ad_value(884), s.ad_value(824)), 0.5));
            s.store_sub(856, 855, 823);
        }

        if ((!s.b[975]) && (!s.b[976])) {
            s.copy_ad(856, 855);
        }

        if (!s.b[975]) {
            s.store_sub(858, 852, 853);
            s.copy_ad(821, 702);
            s.store_mul(861, 758, 821);
            s.store_mul(862, 758, 821);
            s.store_scaled_div(818, 500, 861, ((-0.5) * s.v[81]));
        }

        s.b[977] = (s.v[818] > (-100.0));
        s.v[977] = if s.b[977] { 1.0 } else { 0.0 };

        if ((!s.b[975]) && s.b[977]) {
            s.store_exp(819, 818);
            s.store_mul_offset_ad_rhs(875, 819, A::scale(s.ad_value(819), 2.0), 1.0);
        }

        if ((!s.b[975]) && (!s.b[977])) {
            s.store_scalar(819, 3.720075976e-44);
            s.store_mul_offset_ad_rhs(875, 819, A::scale(s.ad_value(819), 2.0), 1.0);
        }

        if (!s.b[975]) {
            s.store_div_ad_lhs(820, A::mul(s.ad_value(470), s.ad_value(778)), 701);
            s.copy_ad(821, 466);
            s.store_div_ad_lhs(822, A::add(A::add(s.ad_value(820), A::mul(s.ad_value(821), s.ad_value(875))), s.ad_value(469)), 757);
        }

        s.b[978] = (s.v[822] >= (-0.5));
        s.v[978] = if s.b[978] { 1.0 } else { 0.0 };

        if ((!s.b[975]) && s.b[978]) {
            s.store_offset(864, 822, 1.0);
        }

        if ((!s.b[975]) && (!s.b[978])) {
            s.store_div_from_scalar_offset_scaled_input(818, 1.0, 822, 8.0, 3.0);
            s.store_mul_offset_ad_lhs(864, A::scale(s.ad_value(822), 3.0), 1.0, 818);
        }

        s.b[979] = (s.v[739] > 0.0);
        s.v[979] = if s.b[979] { 1.0 } else { 0.0 };

        if ((!s.b[975]) && s.b[979]) {
            s.store_offset_scaled(821, 739, 2.0, s.v[81]);
        }

        if ((!s.b[975]) && s.b[979]) {
            s.store_mul_ad_rhs(822, 851, {
                if ((s.v[81] / s.v[821]) > 1e-38) {
                    A::ln(A::div_from_scalar(s.v[81], s.ad_value(821)))
                } else {
                    A::neg(A::constant(87.49823353377374))
                }
            });
        }

        if ((!s.b[975]) && s.b[979]) {
            s.store_mul(872, 864, 822);
        }

        if ((!s.b[975]) && (!s.b[979])) {
            s.store_scalar(872, 0.0);
        }

        if (!s.b[975]) {
            s.store_mul(411, 499, 875);
            s.store_mul(876, 411, 858);
            s.store_scaled_div(818, 503, 862, ((-0.5) * (s.v[82] * s.v[81])));
        }

        s.b[980] = (s.v[818] > (-100.0));
        s.v[980] = if s.b[980] { 1.0 } else { 0.0 };

        if ((!s.b[975]) && s.b[980]) {
            s.store_exp(819, 818);
            s.store_mul_offset_ad_rhs(820, 819, A::scale(s.ad_value(819), 2.0), 1.0);
        }

        if ((!s.b[975]) && (!s.b[980])) {
            s.store_scalar(819, 3.720075976e-44);
            s.store_mul_offset_ad_rhs(820, 819, A::scale(s.ad_value(819), 2.0), 1.0);
        }

        if (!s.b[975]) {
            s.store_mul(818, 502, 820);
            s.store_mul(877, 818, 858);
            s.store_scalar(863, ((s.v[84] / s.v[150]) - 1.0));
            s.store_sqrt_offset_scaled_input(818, 498, 1.0 / (s.v[81]), 1.0);
            s.store_add_ad_rhs(819, 491, A::scale(s.ad_value(492), 1.0 / (s.v[81])));
            s.store_add_ad(873, A::mul(A::mul(s.ad_value(737), A::offset(s.ad_value(818), (-1.0))), s.ad_value(854)), A::mul(s.ad_value(819), s.ad_value(863)));
            s.store_div_ad(814, A::mul(s.ad_value(776), s.ad_value(853)), A::offset(s.ad_value(497), s.v[82]));
            s.store_scalar(870, 0.0);
            s.store_scalar(874, 0.0);
            s.store_sqrt_offset_scaled_input(871, 738, 1.0 / (s.v[81]), 1.0);
            s.copy_ad(867, 854);
        }

        if (!s.b[975]) {
            let assign11150_ad_e8258: A = A::sub(A::sub(A::sub(A::add(A::add(A::sub(A::sub(A::add(A::scale(s.ad_value(768), s.v[36]), A::mul(A::sub(A::mul(s.ad_value(737), s.ad_value(867)), A::mul(s.ad_value(707), s.ad_value(854))), s.ad_value(871))), s.ad_value(876)), s.ad_value(877)), A::mul(s.ad_value(495), s.ad_value(814))), s.ad_value(873)), s.ad_value(870)), s.ad_value(872)), s.ad_value(874));
            s.store_ad_value(859, assign11150_ad_e8258);
        }

        if (!s.b[975]) {
            s.store_sub(860, 856, 859);
            s.store_mul(849, 864, 851);
            s.store_div_ad_lhs(865, A::mul(s.ad_value(745), s.ad_value(860)), 849);
            s.store_div_ad_lhs(866, A::sub(s.ad_value(521), A::mul(A::sub_from_scalar(1.0, s.ad_value(745)), s.ad_value(860))), 849);
        }

        s.b[981] = (s.v[865] > 100.0);
        s.v[981] = if s.b[981] { 1.0 } else { 0.0 };

        if ((!s.b[975]) && s.b[981]) {
            s.copy_ad(857, 860);
        }

        s.b[982] = (s.v[866] > 100.0);
        s.v[982] = if s.b[982] { 1.0 } else { 0.0 };

        if (((!s.b[975]) && (!s.b[981])) && s.b[982]) {
            s.store_div_ad(818, A::sub(s.ad_value(860), s.ad_value(521)), A::mul(s.ad_value(864), s.ad_value(851)));
            s.store_exp(868, 818);
            s.store_mul_div_ad_lhs(857, A::mul(s.ad_value(851), s.ad_value(728)), s.ad_value(757), 868);
        }

        if (((!s.b[975]) && (!s.b[981])) && (!s.b[982])) {
            s.store_exp(868, 865);
        }

        if (((!s.b[975]) && (!s.b[981])) && (!s.b[982])) {
            s.store_mul_ad_rhs(819, 849, {
                if ((1.0 + s.v[868]) > 1e-38) {
                    A::ln(A::offset(s.ad_value(868), 1.0))
                } else {
                    A::neg(A::constant(87.49823353377374))
                }
            });
        }

        if (((!s.b[975]) && (!s.b[981])) && (!s.b[982])) {
            s.store_mul_ad(822, A::mul(A::div(A::neg(s.ad_value(757)), A::mul(s.ad_value(850), s.ad_value(728))), A::exp(s.ad_value(866))), A::sub_from_scalar(1.0, s.ad_value(745)));
            s.store_sub_ad_rhs(820, 745, A::div(A::mul(s.ad_value(849), s.ad_value(822)), A::sub_from_scalar(1.0, s.ad_value(745))));
            s.store_div(857, 819, 820);
        }

        if (!s.b[975]) {
            s.store_sub_ad_lhs(821, A::sub(A::scale(s.ad_value(768), s.v[36]), s.ad_value(766)), 853);
            s.store_scale(869, 821, 4.0);
        }

        s.b[983] = (s.v[869] < 0.0);
        s.v[983] = if s.b[983] { 1.0 } else { 0.0 };

        if ((!s.b[975]) && s.b[983]) {
            s.store_scalar(869, 0.0);
        }

        if (!s.b[975]) {
            s.store_scalar(878, 0.0);
            s.copy_ad(879, 776);
            s.store_scalar(880, 1000000.0);
        }

    }

    pub(super) fn stamp_reactive_block_6(
        ctx: &GeneratedEvalContext<'_>,
        s: &mut ReactiveScratch,
        p: &Parameters,
        nodes: &[usize; Instance::NODE_COUNT],
    ) {
        let mut assign11380_loop_guard: usize = 0;
        while {
            let assign11380_cond_e8494: f64 = (s.v[879] - s.v[880]);
            let assign11380_cond_e8494_d_n0: f64 = (s.dn[879][0] - s.dn[880][0]);
            let assign11380_cond_e8494_d_n1: f64 = (s.dn[879][1] - s.dn[880][1]);
            let assign11380_cond_e8494_d_n2: f64 = (s.dn[879][2] - s.dn[880][2]);
            let assign11380_cond_e8494_d_n3: f64 = (s.dn[879][3] - s.dn[880][3]);
            let assign11380_cond_e8494_d_n4: f64 = (s.dn[879][4] - s.dn[880][4]);
            let assign11380_cond_e8494_d_n5: f64 = (s.dn[879][5] - s.dn[880][5]);
            let assign11380_cond_e8494_d_n6: f64 = (s.dn[879][6] - s.dn[880][6]);
            let assign11380_cond_e8494_d_n7: f64 = (s.dn[879][7] - s.dn[880][7]);
            let assign11380_cond_e8494_d_n8: f64 = (s.dn[879][8] - s.dn[880][8]);
            let assign11380_cond_e8494_d_n9: f64 = (s.dn[879][9] - s.dn[880][9]);
            let assign11380_cond_e8494_d_n10: f64 = (s.dn[879][10] - s.dn[880][10]);
            let assign11380_cond_e8494_d_n11: f64 = (s.dn[879][11] - s.dn[880][11]);
            let assign11380_cond_e8494_d_n12: f64 = (s.dn[879][12] - s.dn[880][12]);
            let assign11380_cond_e8494_d_b0: f64 = (s.db[879][0] - s.db[880][0]);
            let assign11380_cond_e8494_d_b1: f64 = (s.db[879][1] - s.db[880][1]);
            let assign11380_cond_e8494_d_b2: f64 = (s.db[879][2] - s.db[880][2]);
            let assign11380_cond_e8494_d_b3: f64 = (s.db[879][3] - s.db[880][3]);
            let assign11380_cond_e8494_d_b4: f64 = (s.db[879][4] - s.db[880][4]);
            let assign11380_cond_e8494_d_b5: f64 = (s.db[879][5] - s.db[880][5]);
            let assign11380_cond_e8494_d_b6: f64 = (s.db[879][6] - s.db[880][6]);
            let assign11380_cond_e8494_d_b7: f64 = (s.db[879][7] - s.db[880][7]);
            let assign11380_cond_e8494_d_b8: f64 = (s.db[879][8] - s.db[880][8]);
            let assign11380_cond_e8495: f64 = (assign11380_cond_e8494).abs();
            let assign11380_cond_e8495_d_n0: f64 = if assign11380_cond_e8494 >= 0.0 { assign11380_cond_e8494_d_n0 } else { (-assign11380_cond_e8494_d_n0) };
            let assign11380_cond_e8495_d_n1: f64 = if assign11380_cond_e8494 >= 0.0 { assign11380_cond_e8494_d_n1 } else { (-assign11380_cond_e8494_d_n1) };
            let assign11380_cond_e8495_d_n2: f64 = if assign11380_cond_e8494 >= 0.0 { assign11380_cond_e8494_d_n2 } else { (-assign11380_cond_e8494_d_n2) };
            let assign11380_cond_e8495_d_n3: f64 = if assign11380_cond_e8494 >= 0.0 { assign11380_cond_e8494_d_n3 } else { (-assign11380_cond_e8494_d_n3) };
            let assign11380_cond_e8495_d_n4: f64 = if assign11380_cond_e8494 >= 0.0 { assign11380_cond_e8494_d_n4 } else { (-assign11380_cond_e8494_d_n4) };
            let assign11380_cond_e8495_d_n5: f64 = if assign11380_cond_e8494 >= 0.0 { assign11380_cond_e8494_d_n5 } else { (-assign11380_cond_e8494_d_n5) };
            let assign11380_cond_e8495_d_n6: f64 = if assign11380_cond_e8494 >= 0.0 { assign11380_cond_e8494_d_n6 } else { (-assign11380_cond_e8494_d_n6) };
            let assign11380_cond_e8495_d_n7: f64 = if assign11380_cond_e8494 >= 0.0 { assign11380_cond_e8494_d_n7 } else { (-assign11380_cond_e8494_d_n7) };
            let assign11380_cond_e8495_d_n8: f64 = if assign11380_cond_e8494 >= 0.0 { assign11380_cond_e8494_d_n8 } else { (-assign11380_cond_e8494_d_n8) };
            let assign11380_cond_e8495_d_n9: f64 = if assign11380_cond_e8494 >= 0.0 { assign11380_cond_e8494_d_n9 } else { (-assign11380_cond_e8494_d_n9) };
            let assign11380_cond_e8495_d_n10: f64 = if assign11380_cond_e8494 >= 0.0 { assign11380_cond_e8494_d_n10 } else { (-assign11380_cond_e8494_d_n10) };
            let assign11380_cond_e8495_d_n11: f64 = if assign11380_cond_e8494 >= 0.0 { assign11380_cond_e8494_d_n11 } else { (-assign11380_cond_e8494_d_n11) };
            let assign11380_cond_e8495_d_n12: f64 = if assign11380_cond_e8494 >= 0.0 { assign11380_cond_e8494_d_n12 } else { (-assign11380_cond_e8494_d_n12) };
            let assign11380_cond_e8495_d_b0: f64 = if assign11380_cond_e8494 >= 0.0 { assign11380_cond_e8494_d_b0 } else { (-assign11380_cond_e8494_d_b0) };
            let assign11380_cond_e8495_d_b1: f64 = if assign11380_cond_e8494 >= 0.0 { assign11380_cond_e8494_d_b1 } else { (-assign11380_cond_e8494_d_b1) };
            let assign11380_cond_e8495_d_b2: f64 = if assign11380_cond_e8494 >= 0.0 { assign11380_cond_e8494_d_b2 } else { (-assign11380_cond_e8494_d_b2) };
            let assign11380_cond_e8495_d_b3: f64 = if assign11380_cond_e8494 >= 0.0 { assign11380_cond_e8494_d_b3 } else { (-assign11380_cond_e8494_d_b3) };
            let assign11380_cond_e8495_d_b4: f64 = if assign11380_cond_e8494 >= 0.0 { assign11380_cond_e8494_d_b4 } else { (-assign11380_cond_e8494_d_b4) };
            let assign11380_cond_e8495_d_b5: f64 = if assign11380_cond_e8494 >= 0.0 { assign11380_cond_e8494_d_b5 } else { (-assign11380_cond_e8494_d_b5) };
            let assign11380_cond_e8495_d_b6: f64 = if assign11380_cond_e8494 >= 0.0 { assign11380_cond_e8494_d_b6 } else { (-assign11380_cond_e8494_d_b6) };
            let assign11380_cond_e8495_d_b7: f64 = if assign11380_cond_e8494 >= 0.0 { assign11380_cond_e8494_d_b7 } else { (-assign11380_cond_e8494_d_b7) };
            let assign11380_cond_e8495_d_b8: f64 = if assign11380_cond_e8494 >= 0.0 { assign11380_cond_e8494_d_b8 } else { (-assign11380_cond_e8494_d_b8) };
            let assign11380_cond_e8499: f64 = if ((!s.b[975]) && ((s.v[878] <= 4.0) && (assign11380_cond_e8495 > 1e-12))) { 1.0 } else { 0.0 };
            assign11380_cond_e8499 != 0.0
        } {
            assign11380_loop_guard += 1;
            assert!(assign11380_loop_guard <= Self::MAX_ANALOG_LOOP_ITERATIONS, "generated Verilog-A analog loop exceeded iteration guard");
            if (!s.b[975]) {
                s.copy_ad(880, 879);
                s.store_scale(814, 879, 200000000.0);
                s.store_div_ad_lhs(984, A::add(s.ad_value(857), s.ad_value(869)), 814);
            }
            if (!s.b[975]) {
                s.store_offset_exp_ad(985, A::scale({
                    if (s.v[984] > 1e-38) {
                        A::ln(s.ad_value(984))
                    } else {
                        A::neg(A::constant(87.49823353377374))
                    }
                }, (s.v[86] * 0.7)), 1.0);
            }
            if (!s.b[975]) {
                s.store_div_from_scalar(881, (s.v[85] * 1.9e-9), 985);
                s.store_sub_ad_rhs(879, 776, A::mul(A::scale(s.ad_value(777), 1.0 / (s.v[74])), s.ad_value(881)));
                s.store_offset(878, 878, 1.0);
            }
        }

        if (!s.b[975]) {
            s.copy_ad(92, 879);
        }

        s.copy_ad(812, 702);

        s.store_sub(813, 485, 488);

        s.store_mul(814, 758, 812);

        s.store_scaled_div(818, 503, 814, ((-0.5) * (s.v[689] * s.v[688])));

        s.b[986] = (s.v[818] > (-100.0));
        s.v[986] = if s.b[986] { 1.0 } else { 0.0 };

        if s.b[986] {
            s.store_exp(819, 818);
            s.store_mul_offset_ad_rhs(820, 819, A::scale(s.ad_value(819), 2.0), 1.0);
        }

        if (!s.b[986]) {
            s.store_scalar(819, 3.720075976e-44);
            s.store_mul_offset_ad_rhs(820, 819, A::scale(s.ad_value(819), 2.0), 1.0);
        }

        s.store_mul(818, 502, 820);

        s.store_mul(820, 818, 813);

        s.store_scaled_div(818, 500, 814, ((-0.5) * s.v[688]));

        s.b[987] = (s.v[818] > (-100.0));
        s.v[987] = if s.b[987] { 1.0 } else { 0.0 };

        if s.b[987] {
            s.store_exp(819, 818);
            s.store_mul_offset_ad_rhs(821, 819, A::scale(s.ad_value(819), 2.0), 1.0);
        }

        if (!s.b[987]) {
            s.store_scalar(819, 3.720075976e-44);
            s.store_mul_offset_ad_rhs(821, 819, A::scale(s.ad_value(819), 2.0), 1.0);
        }

        s.store_mul3_lhs(821, 499, 821, 813);

        s.store_div_ad(822, A::mul(s.ad_value(92), s.ad_value(488)), A::offset(s.ad_value(497), s.v[689]));

        s.store_sqrt_offset_scaled_input(818, 498, 1.0 / (s.v[688]), 1.0);

        s.store_add_ad(823, A::mul(A::mul(s.ad_value(737), A::offset(s.ad_value(818), (-1.0))), s.ad_value(700)), A::scale(A::add(s.ad_value(491), A::scale(s.ad_value(492), 1.0 / (s.v[688]))), (s.v[827] - 1.0)));

        s.store_add_ad_lhs(883, A::add(A::sub(A::sub(A::scale(s.ad_value(507), s.v[36]), s.ad_value(820)), s.ad_value(821)), A::mul(s.ad_value(495), s.ad_value(822))), 823);

        s.store_sub_ad(720, A::sub(s.ad_value(883), s.ad_value(488)), A::mul(s.ad_value(490), s.ad_value(700)));

        s.store_mul_scaled_ad_rhs(705, 478, ((1.60219e-19) * ((1000000.0 * s.v[174]))), A::offset(A::scale(s.ad_value(498), 1.0 / (s.v[688])), 1.0));

        s.v[421] = ((s.v[399] * (s.v[401] + (((s.v[689] / s.v[59]) / 3.0) / s.v[400]))) / ((s.v[400] * s.v[39]) * (s.v[37] - s.v[402])));

        s.b[988] = (s.v[421] > 0.0);
        s.v[988] = if s.b[988] { 1.0 } else { 0.0 };

        if s.b[988] {
            s.store_scalar(421, (1.0 / s.v[421]));
        }

        if (!s.b[988]) {
            s.store_scalar(421, 1000.0);
        }

        s.store_offset(424, 720, (s.v[36] * s.v[56]));

        s.store_scaled_sqrt_ad(721, A::div(A::mul(s.ad_value(778), s.ad_value(831)), A::scale(s.ad_value(478), (1.60219e-19 * 1000000.0))), 0.3333333333333333);

        s.store_sub_ad_lhs(819, A::sub(A::scale(s.ad_value(768), s.v[36]), s.ad_value(766)), 488);

        s.store_scale(820, 819, 2.0);

        s.store_scale(821, 819, 2.5);

        if (s.v[36] == 1.0) {
            s.copy_ad(425, 820);
        } else {
            s.copy_ad(425, 821);
        }

        s.b[992] = (s.v[425] < 0.0);
        s.v[992] = if s.b[992] { 1.0 } else { 0.0 };

        if s.b[992] {
            s.store_scalar(425, 0.0);
        }

        s.b[993] = (s.v[89] == 4.0);
        s.v[993] = if s.b[993] { 1.0 } else { 0.0 };

        if s.b[993] {
            s.store_mul(861, 758, 702);
            s.store_scaled_div(818, 500, 861, s.v[688]);
        }

        s.b[994] = (s.v[818] < 100.0);
        s.v[994] = if s.b[994] { 1.0 } else { 0.0 };

        if (s.b[993] && s.b[994]) {
            s.store_exp(819, 818);
            s.store_offset(820, 819, (-1.0));
            s.store_square(821, 820);
            s.store_add_ad_rhs(822, 821, A::scale(s.ad_value(819), (2.0 * 3.720075976e-44)));
            s.store_div(875, 819, 822);
        }

        if (s.b[993] && (!s.b[994])) {
            s.store_scalar(875, (1.0 / (2.688117142e43 - 2.0)));
        }

        if s.b[993] {
            s.store_div(813, 778, 701);
            s.store_mul(814, 470, 813);
            s.store_div_ad_lhs(883, A::add(A::add(s.ad_value(814), A::mul(s.ad_value(466), s.ad_value(875))), s.ad_value(469)), 757);
        }

        s.b[995] = (s.v[883] >= (-0.5));
        s.v[995] = if s.b[995] { 1.0 } else { 0.0 };

        if (s.b[993] && s.b[995]) {
            s.store_offset(882, 883, 1.0);
        }

        if (s.b[993] && (!s.b[995])) {
            s.store_div_from_scalar_offset_scaled_input(818, 1.0, 883, 8.0, 3.0);
            s.store_mul_offset_ad_lhs(882, A::scale(s.ad_value(883), 3.0), 1.0, 818);
        }

        if s.b[993] {
            s.store_mul(818, 882, 831);
            s.copy_ad(819, 521);
            s.store_div(820, 819, 818);
        }

        s.b[996] = (s.v[820] < (-100.0));
        s.v[996] = if s.b[996] { 1.0 } else { 0.0 };

        if (s.b[993] && s.b[996]) {
            s.store_scaled_div(821, 757, 728, 3.720075976e-44);
            s.store_add_ad_rhs(822, 745, A::mul(s.ad_value(821), s.ad_value(882)));
        }

        s.b[997] = (s.v[820] > 100.0);
        s.v[997] = if s.b[997] { 1.0 } else { 0.0 };

        if ((s.b[993] && (!s.b[996])) && s.b[997]) {
            s.store_scaled_div(821, 757, 728, 2.688117142e43);
            s.store_add_ad_rhs(822, 745, A::mul(s.ad_value(821), s.ad_value(882)));
        }

        if ((s.b[993] && (!s.b[996])) && (!s.b[997])) {
            s.store_div_ad_lhs(821, A::mul(A::exp(s.ad_value(820)), s.ad_value(757)), 728);
            s.store_add_ad_rhs(822, 745, A::mul(s.ad_value(821), s.ad_value(882)));
        }

        if s.b[993] {
            s.store_scaled_div(426, 818, 822, 0.6931471805599453);
        }

        if (!s.b[993]) {
            s.store_scalar(426, 0.0);
        }

        s.b[1050] = ((p.p35 >= 4.4) || (p.p61 != 0.0));
        s.v[1050] = if s.b[1050] { 1.0 } else { 0.0 };

        s.b[1051] = (s.v[476] < 0.01);
        s.v[1051] = if s.b[1051] { 1.0 } else { 0.0 };

        if (s.b[1050] && s.b[1051]) {
            s.store_scalar(476, 0.01);
        }

        s.b[1052] = (s.v[476] > 1.0);
        s.v[1052] = if s.b[1052] { 1.0 } else { 0.0 };

        if ((s.b[1050] && (!s.b[1051])) && s.b[1052]) {
            s.store_scalar(476, 1.0);
            s.store_scalar(475, 0.0);
        }

        s.b[1053] = (s.v[551] < 0.0);
        s.v[1053] = if s.b[1053] { 1.0 } else { 0.0 };

        if s.b[1053] {
            s.store_scalar(551, 0.0);
            s.store_scalar(552, 0.0);
        }

        s.b[1054] = ((s.v[552] < 0.001) && (s.v[552] != 0.0));
        s.v[1054] = if s.b[1054] { 1.0 } else { 0.0 };

        if ((!s.b[1053]) && s.b[1054]) {
            s.store_scalar(552, 0.0);
        }

        s.v[770] = 0.0;

        s.b[1144] = ((p.p33 == 1.0) && (p.p16 != 0.0));
        s.v[1144] = if s.b[1144] { 1.0 } else { 0.0 };

        if s.b[1144] {
            s.store_voltage(770, ctx, nodes, Some(6), None);
        }

        if (!s.b[1144]) {
            s.store_scalar(770, 0.0);
        }

        s.store_offset(769, 770, s.v[769]);

        s.store_scale(771, 769, 1.0 / (s.v[150]));

        s.store_offset_scaled(772, 769, 1.0 / (s.v[150]), (-1.0));

        s.v[1466] = 0.0;

        s.v[1467] = 0.0;

        s.v[1468] = 0.0;

        s.v[1469] = 0.0;

        s.v[1464] = 0.0;

        s.v[1454] = 0.0;

        s.v[1191] = 0.0;

        s.v[1455] = 0.0;

        s.v[1463] = 0.0;

        s.v[1460] = 0.0;

        s.v[1461] = 0.0;

        s.v[1459] = 0.0;

        s.v[1451] = 0.0;

        s.copy_ad(1290, 552);

        s.copy_ad(1429, 543);

        s.copy_ad(1430, 544);

        s.b[1492] = ((p.p33 == 1.0) && (p.p16 != 0.0));
        s.v[1492] = if s.b[1492] { 1.0 } else { 0.0 };

        s.b[1493] = (s.v[68] == 0.0);
        s.v[1493] = if s.b[1493] { 1.0 } else { 0.0 };

        if (s.b[1492] && s.b[1493]) {
            s.store_scale(1168, 769, 8.617087e-5);
            s.store_offset(1179, 769, 1108.0);
            s.store_square(1184, 769);
            s.store_sub_from_scalar_ad(1247, 1.16, A::div(A::scale(s.ad_value(1184), 0.000702), s.ad_value(1179)));
            s.store_scalar(1181, 0.00019230584);
            s.store_sqrt(1184, 769);
            s.store_mul_ad_product_lhs(1182, A::scale(s.ad_value(769), 14500000000.0), s.ad_value(1184), 1181);
            s.store_sub_from_scalar_ad(1185, 21.5565981, A::div(s.ad_value(1247), A::scale(s.ad_value(1168), 2.0)));
        }

        s.b[1494] = (s.v[1185] > (-100.0));
        s.v[1494] = if s.b[1494] { 1.0 } else { 0.0 };

        if ((s.b[1492] && s.b[1493]) && s.b[1494]) {
            s.store_exp(1183, 1185);
        }

        if ((s.b[1492] && s.b[1493]) && (!s.b[1494])) {
            s.store_scalar(1183, (((-100.0)) as f64).exp());
        }

        if (s.b[1492] && s.b[1493]) {
            s.store_mul(1246, 1182, 1183);
        }

        if (s.b[1492] && s.b[1493]) {
            s.store_ad_value(1179, {
                if (((1e20 * s.v[478]) / (s.v[1246] * s.v[1246])) > 1e-38) {
                    A::ln(A::div(A::scale(s.ad_value(478), 1e20), A::square(s.ad_value(1246))))
                } else {
                    A::neg(A::constant(87.49823353377374))
                }
            });
        }

        if (s.b[1492] && s.b[1493]) {
            s.store_mul(1275, 1168, 1179);
        }

        if (s.b[1492] && (!s.b[1493])) {
            s.store_scalar(1435, s.v[150]);
            s.store_scale(1168, 769, 8.617087e-5);
            s.store_scale(1437, 1435, 8.617087e-5);
            s.copy_ad(1436, 755);
            s.store_sub_from_scalar_ad(1247, s.v[76], A::div(A::mul(A::scale(s.ad_value(769), s.v[77]), s.ad_value(769)), A::offset(s.ad_value(769), s.v[78])));
            s.store_div_from_scalar_sqrt_ad(1181, 1.0, A::mul(A::square(s.ad_value(1435)), s.ad_value(1435)));
            s.store_sqrt(1184, 769);
            s.store_mul_ad_product_lhs(1182, A::scale(s.ad_value(769), s.v[75]), s.ad_value(1184), 1181);
            s.store_exp_ad(1183, A::sub(A::div(s.ad_value(1436), A::scale(s.ad_value(1437), 2.0)), A::div(s.ad_value(1247), A::scale(s.ad_value(1168), 2.0))));
            s.store_mul(1246, 1182, 1183);
        }

        if (s.b[1492] && (!s.b[1493])) {
            s.store_ad_value(1179, {
                if (((1e20 * s.v[478]) / (s.v[1246] * s.v[1246])) > 1e-38) {
                    A::ln(A::div(A::scale(s.ad_value(478), 1e20), A::square(s.ad_value(1246))))
                } else {
                    A::neg(A::constant(87.49823353377374))
                }
            });
        }

    }

    pub(super) fn stamp_reactive_block_7(
        s: &mut ReactiveScratch,
        p: &Parameters,
    ) {
        if (s.b[1492] && (!s.b[1493])) {
            s.store_mul(1275, 1168, 1179);
        }

        s.b[1495] = (s.v[479] > 0.0);
        s.v[1495] = if s.b[1495] { 1.0 } else { 0.0 };

        if (s.b[1492] && s.b[1495]) {
            s.store_ad_value(1179, {
                if ((s.v[478] / s.v[479]) > 1e-38) {
                    A::ln(A::div(s.ad_value(478), s.ad_value(479)))
                } else {
                    A::neg(A::constant(87.49823353377374))
                }
            });
        }

        if (s.b[1492] && s.b[1495]) {
            s.store_scaled_mul(1276, 1168, 1179, (-s.v[36]));
        }

        if (s.b[1492] && (!s.b[1495])) {
            s.store_ad_value(1179, {
                if (((((-s.v[478]) * s.v[479]) / s.v[1246]) / s.v[1246]) > 1e-38) {
                    A::ln(A::div(A::div(A::mul(A::neg(s.ad_value(478)), s.ad_value(479)), s.ad_value(1246)), s.ad_value(1246)))
                } else {
                    A::neg(A::constant(87.49823353377374))
                }
            });
        }

        if (s.b[1492] && (!s.b[1495])) {
            s.store_scaled_mul(1276, 1168, 1179, (-s.v[36]));
        }

        if s.b[1492] {
            s.store_mul_scaled_ad_rhs(1277, 1168, 2.0, {
                if ((s.v[478] / s.v[1246]) > 1e-38) {
                    A::ln(A::div(s.ad_value(478), s.ad_value(1246)))
                } else {
                    A::neg(A::constant(87.49823353377374))
                }
            });
        }

        if s.b[1492] {
            s.store_sqrt(1278, 1277);
            s.store_mul_sqrt_ad_lhs(1279, A::div(A::scale(s.ad_value(778), 2.0), A::scale(s.ad_value(478), (1.60219e-19 * 1000000.0))), 1278);
            s.store_div_ad_lhs(1473, A::sqrt(A::scale(A::mul(A::scale(s.ad_value(778), 1.60219e-19), s.ad_value(478)), (1000000.0 * 0.5))), 1278);
            s.store_sqrt_mul_ad(1180, A::mul(A::div(s.ad_value(778), A::scale(s.ad_value(777), 8.85418e-12)), s.ad_value(776)), s.ad_value(1279));
            s.store_exp_ad(1179, A::div(A::scale(s.ad_value(506), ((-0.5) * s.v[688])), s.ad_value(1180)));
            s.store_add_ad_rhs(1474, 1179, A::mul(A::scale(s.ad_value(1179), 2.0), s.ad_value(1179)));
            s.store_exp_ad(1179, A::div(A::scale(s.ad_value(505), ((-0.5) * s.v[688])), s.ad_value(1180)));
            s.store_add_ad_rhs(1181, 1179, A::mul(A::scale(s.ad_value(1179), 2.0), s.ad_value(1179)));
            s.store_add_ad_lhs(1475, A::mul(s.ad_value(561), s.ad_value(1181)), 562);
            s.copy_ad(409, 1168);
            s.store_offset(1182, 771, (-1.0));
            s.store_mul_div_from_scalar_lhs(1183, 1.115, 1168, 1182);
            s.store_div_ad_lhs(1186, A::mul(s.ad_value(619), s.ad_value(1183)), 661);
        }

        s.b[1496] = (s.v[1186] > 100.0);
        s.v[1496] = if s.b[1496] { 1.0 } else { 0.0 };

        if (s.b[1492] && s.b[1496]) {
            s.store_scaled_offset_ad(1179, A::offset(s.ad_value(1186), 1.0), (-100.0), 2.688117142e43);
        }

        s.b[1497] = (s.v[1186] < (-100.0));
        s.v[1497] = if s.b[1497] { 1.0 } else { 0.0 };

        if ((s.b[1492] && (!s.b[1496])) && s.b[1497]) {
            s.store_scalar(1179, 3.720075976e-44);
        }

        if ((s.b[1492] && (!s.b[1496])) && (!s.b[1497])) {
            s.store_exp(1179, 1186);
        }

        s.b[1498] = (s.v[619] == s.v[620]);
        s.v[1498] = if s.b[1498] { 1.0 } else { 0.0 };

        if (s.b[1492] && s.b[1498]) {
            s.copy_ad(1180, 1179);
        }

        if (s.b[1492] && (!s.b[1498])) {
            s.store_div_ad_lhs(1186, A::mul(s.ad_value(620), s.ad_value(1183)), 661);
        }

        s.b[1499] = (s.v[1186] > 100.0);
        s.v[1499] = if s.b[1499] { 1.0 } else { 0.0 };

        if ((s.b[1492] && (!s.b[1498])) && s.b[1499]) {
            s.store_scaled_offset_ad(1180, A::offset(s.ad_value(1186), 1.0), (-100.0), 2.688117142e43);
        }

        s.b[1500] = (s.v[1186] < (-100.0));
        s.v[1500] = if s.b[1500] { 1.0 } else { 0.0 };

        if (((s.b[1492] && (!s.b[1498])) && (!s.b[1499])) && s.b[1500]) {
            s.store_scalar(1180, 3.720075976e-44);
        }

        if (((s.b[1492] && (!s.b[1498])) && (!s.b[1499])) && (!s.b[1500])) {
            s.store_exp(1180, 1186);
        }

        if s.b[1492] {
            s.store_div_ad_lhs(1186, A::mul(s.ad_value(621), s.ad_value(1183)), 663);
        }

        s.b[1501] = (s.v[1186] > 100.0);
        s.v[1501] = if s.b[1501] { 1.0 } else { 0.0 };

        if (s.b[1492] && s.b[1501]) {
            s.store_scaled_offset_ad(1181, A::offset(s.ad_value(1186), 1.0), (-100.0), 2.688117142e43);
        }

        s.b[1502] = (s.v[1186] < (-100.0));
        s.v[1502] = if s.b[1502] { 1.0 } else { 0.0 };

        if ((s.b[1492] && (!s.b[1501])) && s.b[1502]) {
            s.store_scalar(1181, 3.720075976e-44);
        }

        if ((s.b[1492] && (!s.b[1501])) && (!s.b[1502])) {
            s.store_exp(1181, 1186);
        }

        if s.b[1492] {
            s.store_mul(1307, 716, 1179);
            s.store_mul(1284, 667, 1179);
            s.store_mul(1282, 669, 1180);
            s.store_mul(1286, 671, 1181);
            s.store_mul(1186, 622, 1182);
        }

        s.b[1503] = (s.v[1186] > 100.0);
        s.v[1503] = if s.b[1503] { 1.0 } else { 0.0 };

        if (s.b[1492] && s.b[1503]) {
            s.store_scaled_offset_ad(1179, A::offset(s.ad_value(1186), 1.0), (-100.0), 2.688117142e43);
        }

        s.b[1504] = (s.v[1186] < (-100.0));
        s.v[1504] = if s.b[1504] { 1.0 } else { 0.0 };

        if ((s.b[1492] && (!s.b[1503])) && s.b[1504]) {
            s.store_scalar(1179, 3.720075976e-44);
        }

        if ((s.b[1492] && (!s.b[1503])) && (!s.b[1504])) {
            s.store_exp(1179, 1186);
        }

        if s.b[1492] {
            s.store_mul(1288, 673, 1179);
            s.store_div_ad_lhs(1186, A::mul(s.ad_value(619), s.ad_value(1183)), 662);
        }

        s.b[1505] = (s.v[1186] > 100.0);
        s.v[1505] = if s.b[1505] { 1.0 } else { 0.0 };

        if (s.b[1492] && s.b[1505]) {
            s.store_scaled_offset_ad(1179, A::offset(s.ad_value(1186), 1.0), (-100.0), 2.688117142e43);
        }

        s.b[1506] = (s.v[1186] < (-100.0));
        s.v[1506] = if s.b[1506] { 1.0 } else { 0.0 };

        if ((s.b[1492] && (!s.b[1505])) && s.b[1506]) {
            s.store_scalar(1179, 3.720075976e-44);
        }

        if ((s.b[1492] && (!s.b[1505])) && (!s.b[1506])) {
            s.store_exp(1179, 1186);
        }

        s.b[1507] = (s.v[619] == s.v[623]);
        s.v[1507] = if s.b[1507] { 1.0 } else { 0.0 };

        if (s.b[1492] && s.b[1507]) {
            s.copy_ad(1180, 1179);
        }

        if (s.b[1492] && (!s.b[1507])) {
            s.store_div_ad_lhs(1186, A::mul(s.ad_value(623), s.ad_value(1183)), 662);
        }

        s.b[1508] = (s.v[1186] > 100.0);
        s.v[1508] = if s.b[1508] { 1.0 } else { 0.0 };

        if ((s.b[1492] && (!s.b[1507])) && s.b[1508]) {
            s.store_scaled_offset_ad(1180, A::offset(s.ad_value(1186), 1.0), (-100.0), 2.688117142e43);
        }

        s.b[1509] = (s.v[1186] < (-100.0));
        s.v[1509] = if s.b[1509] { 1.0 } else { 0.0 };

        if (((s.b[1492] && (!s.b[1507])) && (!s.b[1508])) && s.b[1509]) {
            s.store_scalar(1180, 3.720075976e-44);
        }

        if (((s.b[1492] && (!s.b[1507])) && (!s.b[1508])) && (!s.b[1509])) {
            s.store_exp(1180, 1186);
        }

        if s.b[1492] {
            s.store_div_ad_lhs(1186, A::mul(s.ad_value(624), s.ad_value(1183)), 664);
        }

        s.b[1510] = (s.v[1186] > 100.0);
        s.v[1510] = if s.b[1510] { 1.0 } else { 0.0 };

        if (s.b[1492] && s.b[1510]) {
            s.store_scaled_offset_ad(1181, A::offset(s.ad_value(1186), 1.0), (-100.0), 2.688117142e43);
        }

        s.b[1511] = (s.v[1186] < (-100.0));
        s.v[1511] = if s.b[1511] { 1.0 } else { 0.0 };

        if ((s.b[1492] && (!s.b[1510])) && s.b[1511]) {
            s.store_scalar(1181, 3.720075976e-44);
        }

        if ((s.b[1492] && (!s.b[1510])) && (!s.b[1511])) {
            s.store_exp(1181, 1186);
        }

        if s.b[1492] {
            s.store_mul(1308, 717, 1179);
            s.store_mul(1285, 668, 1179);
            s.store_mul(1283, 670, 1180);
            s.store_mul(1287, 672, 1181);
            s.store_mul(1186, 625, 1182);
        }

        s.b[1512] = (s.v[1186] > 100.0);
        s.v[1512] = if s.b[1512] { 1.0 } else { 0.0 };

        if (s.b[1492] && s.b[1512]) {
            s.store_scaled_offset_ad(1179, A::offset(s.ad_value(1186), 1.0), (-100.0), 2.688117142e43);
        }

        s.b[1513] = (s.v[1186] < (-100.0));
        s.v[1513] = if s.b[1513] { 1.0 } else { 0.0 };

        if ((s.b[1492] && (!s.b[1512])) && s.b[1513]) {
            s.store_scalar(1179, 3.720075976e-44);
        }

        if ((s.b[1492] && (!s.b[1512])) && (!s.b[1513])) {
            s.store_exp(1179, 1186);
        }

        if s.b[1492] {
            s.store_mul(1289, 674, 1179);
            s.store_mul_pow_ad_rhs(1280, 514, s.ad_value(771), s.ad_value(515));
        }

        s.b[1514] = (p.p35 < 4.2);
        s.v[1514] = if s.b[1514] { 1.0 } else { 0.0 };

        if (s.b[1492] && s.b[1514]) {
            s.store_offset_mul_ad(1296, s.ad_value(597), A::offset(A::scale(s.ad_value(771), s.v[252]), 1.0), 1e-9);
        }

        if (s.b[1492] && (!s.b[1514])) {
            s.store_offset_mul_ad(1296, s.ad_value(597), A::offset(A::scale(s.ad_value(1182), s.v[252]), 1.0), 1e-9);
        }

        if s.b[1492] {
            s.store_scale(1186, 601, s.v[249]);
            s.store_div(1295, 1186, 1296);
            s.store_scale(1183, 414, s.v[249]);
            s.store_div(1294, 1183, 1296);
            s.store_offset(1181, 1294, 1.0);
            s.store_offset(1186, 1295, 1.0);
            s.store_div(1179, 1181, 1186);
            s.store_mul(1280, 1280, 1179);
            s.store_sub_ad_rhs(1281, 471, A::mul(s.ad_value(472), s.ad_value(1182)));
            s.store_offset_mul(1181, 250, 1294, 1.0);
            s.store_offset_mul(1186, 250, 1295, 1.0);
            s.store_div(1179, 1181, 1186);
            s.store_mul(1281, 1281, 1179);
        }

        s.b[1515] = (s.v[403] != 1.0);
        s.v[1515] = if s.b[1515] { 1.0 } else { 0.0 };

        if (s.b[1492] && s.b[1515]) {
            s.store_div_ad_lhs(1290, A::add(s.ad_value(551), A::mul(s.ad_value(555), s.ad_value(1182))), 529);
            s.store_scalar(1429, 0.0);
            s.store_scalar(1430, 0.0);
        }

        if (s.b[1492] && (!s.b[1515])) {
            s.store_scalar(1290, 0.0);
            s.store_scale(1428, 529, s.v[39]);
            s.store_mul(1189, 555, 1182);
            s.store_add(1180, 539, 1189);
            s.store_offset(1181, 1189, s.v[160]);
            s.store_div(1429, 1180, 1428);
            s.store_add(1186, 540, 1189);
            s.store_offset(1183, 1189, s.v[159]);
            s.store_div(1430, 1186, 1428);
        }

        if s.b[1492] {
            s.store_add_ad_rhs(1291, 523, A::mul(s.ad_value(509), s.ad_value(1182)));
            s.store_add_ad_rhs(1292, 524, A::mul(s.ad_value(511), s.ad_value(1182)));
            s.store_add_ad_rhs(1293, 525, A::mul(s.ad_value(513), s.ad_value(1182)));
        }

        if (!s.b[1492]) {
            s.copy_ad(1275, 485);
            s.copy_ad(1276, 530);
            s.copy_ad(1277, 488);
            s.copy_ad(1278, 700);
            s.copy_ad(1279, 701);
            s.copy_ad(1247, 756);
            s.copy_ad(1473, 728);
            s.copy_ad(1474, 703);
            s.copy_ad(1475, 704);
            s.copy_ad(1284, 531);
            s.copy_ad(1285, 532);
            s.copy_ad(1282, 533);
            s.copy_ad(1283, 534);
            s.copy_ad(1286, 535);
        }

    }

    pub(super) fn stamp_reactive_block_8(
        ctx: &GeneratedEvalContext<'_>,
        s: &mut ReactiveScratch,
        p: &Parameters,
        nodes: &[usize; Instance::NODE_COUNT],
        param_given: &[bool; Instance::PARAMETER_COUNT],
    ) {
        if (!s.b[1492]) {
            s.copy_ad(1287, 536);
            s.copy_ad(1288, 537);
            s.copy_ad(1289, 538);
            s.copy_ad(1307, 718);
            s.copy_ad(1308, 719);
            s.copy_ad(1280, 765);
            s.copy_ad(1281, 767);
            s.copy_ad(1291, 508);
            s.copy_ad(1292, 510);
            s.copy_ad(1293, 512);
        }

        s.b[1516] = (param_given[89] || param_given[93]);
        s.v[1516] = if s.b[1516] { 1.0 } else { 0.0 };

        s.b[1517] = (!param_given[89]);
        s.v[1517] = if s.b[1517] { 1.0 } else { 0.0 };

        if (s.b[1516] && s.b[1517]) {
            s.store_scalar(490, 0.53);
        }

        s.b[1518] = (!param_given[93]);
        s.v[1518] = if s.b[1518] { 1.0 } else { 0.0 };

        if (s.b[1516] && s.b[1518]) {
            s.store_scalar(494, (-0.0186));
        }

        s.b[1524] = (!param_given[86]);
        s.v[1524] = if s.b[1524] { 1.0 } else { 0.0 };

        if (((!s.b[1516]) && s.b[1524]) && (s.v[68] != 0.0)) {
            s.store_scaled_div_from_scalar_ad(1179, 1.60219e-19, A::scale(s.ad_value(778), 2.0), 1000000.0);
        }

        if (((!s.b[1516]) && s.b[1524]) && (s.v[68] == 0.0)) {
            s.store_scalar(1179, 0.00077348);
        }

        if ((!s.b[1516]) && s.b[1524]) {
            s.store_sub_ad_rhs(484, 1277, A::scale(A::mul(s.ad_value(1179), s.ad_value(478)), (s.v[487] * s.v[487])));
        }

        s.b[1525] = (s.v[484] > 0.0);
        s.v[1525] = if s.b[1525] { 1.0 } else { 0.0 };

        if ((!s.b[1516]) && s.b[1525]) {
            s.store_neg(484, 484);
        }

        s.b[1526] = (s.v[486] > 0.0);
        s.v[1526] = if s.b[1526] { 1.0 } else { 0.0 };

        if ((!s.b[1516]) && s.b[1526]) {
            s.store_neg(486, 486);
        }

        s.b[1527] = (!param_given[84]);
        s.v[1527] = if s.b[1527] { 1.0 } else { 0.0 };

        if ((!s.b[1516]) && s.b[1527]) {
            s.store_div_ad_lhs(482, A::mul(s.ad_value(780), A::sqrt(s.ad_value(478))), 757);
        }

        s.b[1528] = (!param_given[85]);
        s.v[1528] = if s.b[1528] { 1.0 } else { 0.0 };

        if ((!s.b[1516]) && s.b[1528]) {
            s.store_div_ad_lhs(483, A::mul(s.ad_value(780), A::sqrt(s.ad_value(479))), 757);
        }

        if (!s.b[1516]) {
            s.store_sub(1179, 482, 483);
            s.store_sub_ad_lhs(1180, A::sqrt(A::sub(s.ad_value(1277), s.ad_value(484))), 1278);
            s.store_mul_sub_ad_rhs(1181, 1278, A::sqrt(A::sub(s.ad_value(1277), s.ad_value(486))), s.ad_value(1278));
            s.store_div_ad(1182, A::mul(s.ad_value(1179), s.ad_value(1180)), A::add(A::scale(s.ad_value(1181), 2.0), s.ad_value(486)));
            s.store_add_ad_lhs(763, A::sub(s.ad_value(763), s.ad_value(494)), 1182);
            s.store_sub_ad_rhs(490, 483, A::mul(A::scale(s.ad_value(763), 2.0), A::sqrt(A::sub(s.ad_value(1277), s.ad_value(486)))));
        }

        s.store_offset(1179, 628, s.v[689]);

        s.b[1529] = (s.v[1179] < 1e-8);
        s.v[1529] = if s.b[1529] { 1.0 } else { 0.0 };

        if s.b[1529] {
            s.store_scalar(1179, 1e-8);
        }

        s.store_mul_offset_ad_rhs(707, 490, A::div(s.ad_value(627), s.ad_value(1179)), 1.0);

        s.b[1530] = (!param_given[108]);
        s.v[1530] = if s.b[1530] { 1.0 } else { 0.0 };

        s.b[1531] = (param_given[107] || param_given[106]);
        s.v[1531] = if s.b[1531] { 1.0 } else { 0.0 };

        if (s.b[1530] && s.b[1531]) {
            s.store_sub_ad(766, A::sub(A::add(A::sub(s.ad_value(766), s.ad_value(522)), A::scale(s.ad_value(768), s.v[36])), s.ad_value(1277)), A::mul(s.ad_value(707), s.ad_value(1278)));
        }

        if (s.b[1530] && (!s.b[1531])) {
        }

        s.b[1532] = (!param_given[107]);
        s.v[1532] = if s.b[1532] { 1.0 } else { 0.0 };

        if s.b[1532] {
            s.store_scaled_add_ad(768, A::add(s.ad_value(766), s.ad_value(1277)), A::mul(s.ad_value(707), s.ad_value(1278)), s.v[36]);
        }

        s.b[1533] = (p.p35 < 4.2);
        s.v[1533] = if s.b[1533] { 1.0 } else { 0.0 };

        if s.b[1533] {
            s.copy_ad(1429, 543);
            s.copy_ad(1473, 728);
            s.copy_ad(1474, 703);
            s.copy_ad(1475, 704);
        }

        s.b[1534] = (s.v[89] == 4.0);
        s.v[1534] = if s.b[1534] { 1.0 } else { 0.0 };

        if (s.b[1533] && s.b[1534]) {
            s.copy_ad(1291, 508);
            s.copy_ad(1293, 512);
        }

        s.store_scaled_voltage(1155, ctx, nodes, Some(7), Some(8), s.v[36]);

        s.store_scaled_voltage(1154, ctx, nodes, Some(5), Some(8), s.v[36]);

        s.store_scaled_voltage(1157, ctx, nodes, Some(9), Some(8), s.v[36]);

        s.store_scaled_voltage(1232, ctx, nodes, Some(3), Some(8), s.v[36]);

        s.store_scaled_voltage(1447, ctx, nodes, Some(9), Some(4), s.v[36]);

        s.store_scaled_voltage(1421, ctx, nodes, Some(11), Some(8), s.v[36]);

        s.store_scaled_voltage(1422, ctx, nodes, Some(12), Some(7), s.v[36]);

        s.store_scaled_voltage(1353, ctx, nodes, Some(10), Some(8), s.v[36]);

        s.store_sub(1153, 1154, 1155);

        s.store_sub(1156, 1157, 1155);

        s.store_sub(1233, 1232, 1155);

        s.store_sub(1354, 1353, 1155);

        s.b[1535] = (s.v[1155] >= 0.0);
        s.v[1535] = if s.b[1535] { 1.0 } else { 0.0 };

        if s.b[1535] {
            s.store_scalar(759, 1.0);
            s.copy_ad(1158, 1155);
            s.copy_ad(1159, 1157);
            s.copy_ad(1160, 1154);
            s.copy_ad(1235, 1153);
            s.copy_ad(1236, 1232);
            s.copy_ad(1443, 1156);
            s.copy_ad(1476, 645);
            s.copy_ad(1477, 646);
            s.copy_ad(1478, 647);
            s.copy_ad(1479, 648);
            s.copy_ad(1480, 649);
            s.copy_ad(1481, 650);
            s.copy_ad(1482, 651);
            s.copy_ad(1483, 652);
            s.copy_ad(1484, 653);
            s.copy_ad(1485, 654);
            s.copy_ad(1486, 655);
            s.copy_ad(1487, 656);
            s.copy_ad(1488, 657);
            s.copy_ad(1489, 658);
        }

        if (!s.b[1535]) {
            s.store_scalar(759, (-1.0));
            s.store_neg(1158, 1155);
            s.copy_ad(1159, 1156);
            s.copy_ad(1160, 1153);
            s.copy_ad(1235, 1154);
            s.copy_ad(1236, 1233);
            s.copy_ad(1443, 1157);
            s.copy_ad(1476, 652);
            s.copy_ad(1477, 653);
            s.copy_ad(1478, 654);
            s.copy_ad(1479, 655);
            s.copy_ad(1480, 656);
            s.copy_ad(1481, 657);
            s.copy_ad(1482, 658);
            s.copy_ad(1483, 645);
            s.copy_ad(1484, 646);
            s.copy_ad(1485, 647);
            s.copy_ad(1486, 648);
            s.copy_ad(1487, 649);
            s.copy_ad(1488, 650);
            s.copy_ad(1489, 651);
        }

        s.store_sub(1237, 1236, 1276);

        s.v[1248] = s.v[753];

        s.store_add(1179, 766, 1277);

        s.b[1536] = (s.v[68] == 0.0);
        s.v[1536] = if s.b[1536] { 1.0 } else { 0.0 };

        if s.b[1536] {
            s.copy_ad(779, 778);
        }

        if (!s.b[1536]) {
            s.store_scalar(779, (s.v[87] * 8.85418e-12));
        }

        s.b[1537] = ((((s.v[480] > 1e18) && (s.v[480] < 1e25)) && (s.v[1159] > s.v[1179])) && (s.v[779] != 0.0));
        s.v[1537] = if s.b[1537] { 1.0 } else { 0.0 };

        if s.b[1537] {
            s.store_div_ad(1180, A::mul(A::scale(s.ad_value(779), (1000000.0 * 1.60219e-19)), s.ad_value(480)), A::square(s.ad_value(757)));
            s.store_sqrt_offset_ad(1183, A::div(A::scale(A::sub(s.ad_value(1159), s.ad_value(1179)), 2.0), s.ad_value(1180)), 1.0);
            s.store_mul_offset_rhs(1181, 1180, 1183, (-1.0));
            s.store_div_ad_lhs(1182, A::mul(A::scale(s.ad_value(1181), 0.5), s.ad_value(1181)), 1180);
            s.store_offset_sub(1186, 782, 1182, (-0.05));
            s.store_sqrt_square_offset(1185, 1186, 0.224);
            s.store_sub_ad_rhs(1184, 782, A::scale(A::add(s.ad_value(1186), s.ad_value(1185)), 0.5));
            s.store_sub(1161, 1159, 1184);
        }

        if (!s.b[1537]) {
            s.copy_ad(1161, 1159);
        }

        s.b[1538] = ((((s.v[480] > 1e18) && (s.v[480] < 1e25)) && (s.v[1443] > s.v[1179])) && (s.v[779] != 0.0));
        s.v[1538] = if s.b[1538] { 1.0 } else { 0.0 };

        if s.b[1538] {
            s.store_div_ad(1180, A::mul(A::scale(s.ad_value(779), (1000000.0 * 1.60219e-19)), s.ad_value(480)), A::square(s.ad_value(757)));
            s.store_sqrt_offset_ad(1183, A::div(A::scale(A::sub(s.ad_value(1443), s.ad_value(1179)), 2.0), s.ad_value(1180)), 1.0);
            s.store_mul_offset_rhs(1181, 1180, 1183, (-1.0));
            s.store_div_ad_lhs(1182, A::mul(A::scale(s.ad_value(1181), 0.5), s.ad_value(1181)), 1180);
            s.store_offset_sub(1186, 782, 1182, (-0.05));
            s.store_sqrt_square_offset(1185, 1186, 0.224);
            s.store_sub_ad_rhs(1184, 782, A::scale(A::add(s.ad_value(1186), s.ad_value(1185)), 0.5));
            s.store_sub(1444, 1443, 1184);
        }

        if (!s.b[1538]) {
            s.copy_ad(1444, 1443);
        }

        s.copy_ad(1458, 1159);

        s.v[1227] = s.v[688];

        s.b[1539] = ((p.p33 == 1.0) && (p.p16 != 0.0));
        s.v[1539] = if s.b[1539] { 1.0 } else { 0.0 };

        if s.b[1539] {
            s.store_scale(1168, 769, 8.617087e-5);
        }

        if (!s.b[1539]) {
            s.copy_ad(1168, 409);
        }

        s.store_sub(1170, 1275, 1277);

        s.b[1540] = (s.v[57] == 0.0);
        s.v[1540] = if s.b[1540] { 1.0 } else { 0.0 };

        if s.b[1540] {
            s.copy_ad(1367, 1160);
            s.copy_ad(1382, 1160);
        }

        s.b[1541] = (s.v[404] == 0.0);
        s.v[1541] = if s.b[1541] { 1.0 } else { 0.0 };

        if ((!s.b[1540]) && s.b[1541]) {
            s.store_div_ad_lhs(1179, A::scale(A::neg(s.ad_value(591)), s.v[688]), 489);
        }

    }

    pub(super) fn stamp_reactive_block_9(
        s: &mut ReactiveScratch,
    ) {
        if ((!s.b[1540]) && s.b[1541]) {
            s.store_mul_add_ad_rhs(1180, 590, A::exp(A::scale(s.ad_value(1179), 0.5)), A::scale(A::exp(s.ad_value(1179)), 2.0));
            s.store_mul_sub_rhs(1181, 1180, 1275, 1277);
            s.store_scaled_div(1182, 705, 754, 0.5);
            s.store_add_ad_lhs(1370, A::add(A::sub(s.ad_value(1277), s.ad_value(1182)), s.ad_value(582)), 1181);
            s.store_offset_scaled(1179, 754, 1.0 / (s.v[1248]), 1.0);
            s.store_div_ad_lhs(1182, A::scale(A::neg(s.ad_value(589)), s.v[688]), 489);
            s.store_mul_add_ad_rhs(1184, 588, A::exp(A::scale(s.ad_value(1182), 0.5)), A::scale(A::exp(s.ad_value(1182)), 2.0));
            s.store_div_ad_lhs(1180, A::sub(s.ad_value(587), s.ad_value(1184)), 1179);
            s.store_mul(1181, 1180, 1237);
            s.store_div_from_scalar_offset_ad(1183, 1.0, A::div_from_scalar(s.v[1248], s.ad_value(754)), 1.0);
            s.store_add_ad_lhs(1365, A::mul(s.ad_value(1183), s.ad_value(1370)), 1181);
        }

        if ((!s.b[1540]) && (!s.b[1541])) {
            s.store_div_from_scalar_add_ad(1179, 1.0, A::offset(s.ad_value(754), s.v[1248]), s.ad_value(584));
            s.store_div_ad_lhs(1180, A::scale(A::neg(s.ad_value(591)), s.v[688]), 489);
            s.store_mul_add_ad_rhs(1181, 590, A::exp(A::scale(s.ad_value(1180), 0.5)), A::scale(A::exp(s.ad_value(1180)), 2.0));
            s.store_mul_add_rhs(1182, 1181, 1158, 583);
            s.store_scaled_div(1183, 705, 754, 0.5);
            s.store_mul_ad(1184, A::mul(s.ad_value(754), s.ad_value(1179)), A::add(A::sub(s.ad_value(1277), s.ad_value(1183)), s.ad_value(582)));
            s.store_mul3_lhs(1185, 584, 1179, 1182);
            s.store_add(1370, 1184, 1185);
            s.store_scaled_mul(1186, 1179, 1237, s.v[1248]);
            s.store_add(1365, 1370, 1186);
        }

        if (!s.b[1540]) {
            s.store_offset_sub(1180, 1370, 1365, (-0.005));
            s.store_sqrt_square_offset(1181, 1180, 2.5e-5);
            s.store_scaled_add(1182, 1180, 1181, 0.5);
            s.store_div_ad_lhs(1183, A::mul(s.ad_value(1182), s.ad_value(754)), 705);
            s.store_sub_ad_rhs(1366, 1365, A::mul(A::scale(s.ad_value(1182), 0.5), s.ad_value(1183)));
            s.store_offset(1180, 1277, (-0.02));
            s.store_offset_sub(1181, 1180, 1366, (-0.005));
            s.store_sqrt_square_offset(1182, 1181, (4.0 * 0.005));
            s.store_sub_ad_rhs(1366, 1180, A::scale(A::add(s.ad_value(1181), s.ad_value(1182)), 0.5));
            s.store_sub(1163, 1277, 1366);
            s.store_sqrt(1164, 1163);
            s.store_div_ad_lhs(1199, A::mul(s.ad_value(1279), s.ad_value(1164)), 1278);
            s.store_sqrt(1182, 1199);
            s.store_mul(1179, 501, 1366);
        }

        s.b[1542] = (s.v[1179] >= (-0.5));
        s.v[1542] = if s.b[1542] { 1.0 } else { 0.0 };

        if ((!s.b[1540]) && s.b[1542]) {
            s.store_offset(1180, 1179, 1.0);
        }

        if ((!s.b[1540]) && (!s.b[1542])) {
            s.store_div_from_scalar_offset_scaled_input(1183, 1.0, 1179, 8.0, 3.0);
            s.store_mul_offset_ad_lhs(1180, A::scale(s.ad_value(1179), 3.0), 1.0, 1183);
        }

        if (!s.b[1540]) {
            s.store_mul3_lhs(1200, 758, 1182, 1180);
            s.store_mul(1179, 504, 1366);
        }

        s.b[1543] = (s.v[1179] >= (-0.5));
        s.v[1543] = if s.b[1543] { 1.0 } else { 0.0 };

        if ((!s.b[1540]) && s.b[1543]) {
            s.store_offset(1180, 1179, 1.0);
        }

        if ((!s.b[1540]) && (!s.b[1543])) {
            s.store_div_from_scalar_offset_scaled_input(1183, 1.0, 1179, 8.0, 3.0);
            s.store_mul_offset_ad_lhs(1180, A::scale(s.ad_value(1179), 3.0), 1.0, 1183);
        }

        if (!s.b[1540]) {
            s.store_mul3_lhs(1201, 758, 1182, 1180);
            s.store_scaled_div(1179, 500, 1200, ((-0.5) * s.v[1227]));
        }

        s.b[1544] = (s.v[1179] > (-100.0));
        s.v[1544] = if s.b[1544] { 1.0 } else { 0.0 };

        if ((!s.b[1540]) && s.b[1544]) {
            s.store_exp(1180, 1179);
            s.store_mul_offset_ad_rhs(1203, 1180, A::scale(s.ad_value(1180), 2.0), 1.0);
        }

        if ((!s.b[1540]) && (!s.b[1544])) {
            s.store_scalar(1180, 3.720075976e-44);
            s.store_mul_offset_ad_rhs(1203, 1180, A::scale(s.ad_value(1180), 2.0), 1.0);
        }

        if (!s.b[1540]) {
            s.store_div_ad_lhs(1181, A::mul(s.ad_value(470), s.ad_value(778)), 1199);
            s.store_add_ad(1182, A::add(s.ad_value(466), A::mul(s.ad_value(467), s.ad_value(1366))), A::mul(s.ad_value(468), s.ad_value(1158)));
            s.store_div_ad_lhs(1183, A::add(A::add(s.ad_value(1181), A::mul(s.ad_value(1182), s.ad_value(1203))), s.ad_value(469)), 757);
        }

        s.b[1545] = (s.v[1183] >= (-0.5));
        s.v[1545] = if s.b[1545] { 1.0 } else { 0.0 };

        if ((!s.b[1540]) && s.b[1545]) {
            s.store_offset(1167, 1183, 1.0);
        }

        if ((!s.b[1540]) && (!s.b[1545])) {
            s.store_div_from_scalar_offset_scaled_input(1179, 1.0, 1183, 8.0, 3.0);
            s.store_mul_offset_ad_lhs(1167, A::scale(s.ad_value(1183), 3.0), 1.0, 1179);
        }

        s.b[1546] = (s.v[739] > 0.0);
        s.v[1546] = if s.b[1546] { 1.0 } else { 0.0 };

        if ((!s.b[1540]) && s.b[1546]) {
            s.store_mul_neg_lhs(1179, 740, 1158);
        }

        s.b[1547] = (s.v[1179] < (-100.0));
        s.v[1547] = if s.b[1547] { 1.0 } else { 0.0 };

        if (((!s.b[1540]) && s.b[1546]) && s.b[1547]) {
            s.store_scalar(1181, 3.720075976e-44);
        }

        if (((!s.b[1540]) && s.b[1546]) && (!s.b[1547])) {
            s.store_exp(1181, 1179);
        }

        if ((!s.b[1540]) && s.b[1546]) {
            s.store_offset_mul_ad(1182, s.ad_value(739), A::offset(s.ad_value(1181), 1.0), s.v[1227]);
        }

        if ((!s.b[1540]) && s.b[1546]) {
            s.store_mul_ad_rhs(1183, 1168, {
                if ((s.v[1227] / s.v[1182]) > 1e-38) {
                    A::ln(A::div_from_scalar(s.v[1227], s.ad_value(1182)))
                } else {
                    A::neg(A::constant(87.49823353377374))
                }
            });
        }

        if ((!s.b[1540]) && s.b[1546]) {
            s.store_mul(1424, 1167, 1183);
        }

        if ((!s.b[1540]) && (!s.b[1546])) {
            s.store_scalar(1424, 0.0);
        }

        if (!s.b[1540]) {
            s.store_mul(411, 499, 1203);
            s.store_mul(1202, 411, 1170);
            s.store_scaled_div(1179, 503, 1201, ((-0.5) * (s.v[689] * s.v[1227])));
        }

        s.b[1548] = (s.v[1179] > (-100.0));
        s.v[1548] = if s.b[1548] { 1.0 } else { 0.0 };

        if ((!s.b[1540]) && s.b[1548]) {
            s.store_exp(1180, 1179);
            s.store_mul_offset_ad_rhs(1181, 1180, A::scale(s.ad_value(1180), 2.0), 1.0);
        }

        if ((!s.b[1540]) && (!s.b[1548])) {
            s.store_scalar(1180, 3.720075976e-44);
            s.store_mul_offset_ad_rhs(1181, 1180, A::scale(s.ad_value(1180), 2.0), 1.0);
        }

        if (!s.b[1540]) {
            s.store_mul(1179, 502, 1181);
            s.store_mul(1239, 1179, 1170);
            s.store_sqrt_offset_scaled_input(1179, 498, 1.0 / (s.v[1227]), 1.0);
            s.store_add_ad(1180, A::add(s.ad_value(491), A::scale(s.ad_value(492), 1.0 / (s.v[1227]))), A::mul(s.ad_value(493), s.ad_value(1366)));
            s.store_add_ad(1238, A::mul(A::mul(s.ad_value(737), A::offset(s.ad_value(1179), (-1.0))), s.ad_value(1278)), A::mul(s.ad_value(1180), s.ad_value(772)));
            s.store_div_ad(1205, A::mul(s.ad_value(776), s.ad_value(1277)), A::offset(s.ad_value(497), s.v[689]));
            s.store_add_ad_rhs(1182, 761, A::mul(s.ad_value(557), s.ad_value(1366)));
        }

        s.b[1549] = (s.v[1182] < 0.0001);
        s.v[1549] = if s.b[1549] { 1.0 } else { 0.0 };

        if ((!s.b[1540]) && s.b[1549]) {
            s.store_div_from_scalar_sub_from_scalar_ad(1188, 1.0, 3.0, A::scale(s.ad_value(1182), 20000.0));
            s.store_mul_sub_from_scalar_lhs(1182, 0.0002, 1182, 1188);
        }

        if (!s.b[1540]) {
            s.store_mul3_lhs(1208, 1182, 1474, 1158);
            s.store_add_ad_rhs(1182, 762, A::mul(s.ad_value(559), s.ad_value(1366)));
        }

        s.b[1550] = (s.v[1182] < 0.0001);
        s.v[1550] = if s.b[1550] { 1.0 } else { 0.0 };

        if ((!s.b[1540]) && s.b[1550]) {
            s.store_div_from_scalar_sub_from_scalar_ad(1188, 1.0, 3.0, A::scale(s.ad_value(1182), 20000.0));
            s.store_mul_sub_from_scalar_lhs(1182, 0.0002, 1182, 1188);
        }

        if (!s.b[1540]) {
            s.store_mul3_lhs(1404, 1182, 1474, 1158);
            s.store_sqrt_offset_scaled_input(1423, 738, 1.0 / (s.v[1227]), 1.0);
            s.store_exp_ad(1179, A::mul(A::scale(s.ad_value(743), 2.0), s.ad_value(1158)));
            s.store_div_ad(1425, A::mul(s.ad_value(752), A::offset(s.ad_value(1179), (-1.0))), A::offset(s.ad_value(1179), 1.0));
        }

        if (!s.b[1540]) {
            let assign18380_ad_e13058: A = A::add(A::sub(A::sub(A::sub(A::add(A::scale(s.ad_value(768), s.v[36]), A::mul(A::sub(A::mul(s.ad_value(737), s.ad_value(1164)), A::mul(s.ad_value(707), s.ad_value(1278))), s.ad_value(1423))), A::mul(s.ad_value(764), s.ad_value(1366))), s.ad_value(1202)), s.ad_value(1239)), A::mul(A::add(s.ad_value(495), A::mul(s.ad_value(496), s.ad_value(1366))), s.ad_value(1205)));
            s.store_sub_ad_lhs(1371, A::sub(A::sub(A::add(assign18380_ad_e13058, s.ad_value(1238)), s.ad_value(1208)), s.ad_value(1424)), 1425);
        }

        if (!s.b[1540]) {
            let assign18390_ad_e13099: A = A::add(A::sub(A::sub(A::sub(A::add(A::scale(s.ad_value(768), s.v[36]), A::mul(A::sub(A::mul(s.ad_value(737), s.ad_value(1164)), A::mul(s.ad_value(707), s.ad_value(1278))), s.ad_value(1423))), A::mul(s.ad_value(764), s.ad_value(1366))), s.ad_value(1202)), s.ad_value(1239)), A::mul(A::add(s.ad_value(495), A::mul(s.ad_value(496), s.ad_value(1366))), s.ad_value(1205)));
            s.store_sub_ad_lhs(1386, A::sub(A::sub(A::add(assign18390_ad_e13099, s.ad_value(1238)), s.ad_value(1404)), s.ad_value(1424)), 1425);
        }

        if (!s.b[1540]) {
            s.store_sub(1372, 1371, 1161);
            s.store_mul(1189, 585, 1168);
        }

        s.b[1551] = (((s.v[1372] - s.v[586]) / s.v[1189]) > 100.0);
        s.v[1551] = if s.b[1551] { 1.0 } else { 0.0 };

        if ((!s.b[1540]) && s.b[1551]) {
            s.store_scaled_offset_ad(1373, A::offset(A::div(A::sub(s.ad_value(1372), s.ad_value(586)), s.ad_value(1189)), 1.0), (-100.0), 2.688117142e43);
        }

        s.b[1552] = (((s.v[1372] - s.v[586]) / s.v[1189]) < (-100.0));
        s.v[1552] = if s.b[1552] { 1.0 } else { 0.0 };

        if (((!s.b[1540]) && (!s.b[1551])) && s.b[1552]) {
            s.store_scalar(1373, 3.720075976e-44);
        }

        if (((!s.b[1540]) && (!s.b[1551])) && (!s.b[1552])) {
            s.store_exp_ad(1373, A::div(A::sub(s.ad_value(1372), s.ad_value(586)), s.ad_value(1189)));
        }

        if (!s.b[1540]) {
            s.store_mul_ln_ad_rhs(1376, 1189, A::offset(s.ad_value(1373), 1.0));
            s.store_sub(1374, 1161, 1371);
        }

        s.b[1553] = (((s.v[1374] - s.v[586]) / s.v[1189]) > 100.0);
        s.v[1553] = if s.b[1553] { 1.0 } else { 0.0 };

        if ((!s.b[1540]) && s.b[1553]) {
            s.store_scaled_offset_ad(1375, A::offset(A::div(A::sub(s.ad_value(1374), s.ad_value(586)), s.ad_value(1189)), 1.0), (-100.0), 2.688117142e43);
        }

        s.b[1554] = (((s.v[1374] - s.v[586]) / s.v[1189]) < (-100.0));
        s.v[1554] = if s.b[1554] { 1.0 } else { 0.0 };

        if (((!s.b[1540]) && (!s.b[1553])) && s.b[1554]) {
            s.store_scalar(1375, 3.720075976e-44);
        }

        if (((!s.b[1540]) && (!s.b[1553])) && (!s.b[1554])) {
            s.store_exp_ad(1375, A::div(A::sub(s.ad_value(1374), s.ad_value(586)), s.ad_value(1189)));
        }

        if (!s.b[1540]) {
            s.store_mul_ln_ad_rhs(1377, 1189, A::offset(s.ad_value(1375), 1.0));
            s.store_mul_ad_product_lhs(1180, A::mul(s.ad_value(592), s.ad_value(737)), s.ad_value(1168), 1168);
            s.store_add_ad_rhs(1181, 1377, A::mul(A::scale(s.ad_value(707), 2.0), A::sqrt(s.ad_value(1277))));
            s.store_offset_div_ad(1179, A::mul(s.ad_value(1377), s.ad_value(1181)), s.ad_value(1180), 1.0);
        }

        if (!s.b[1540]) {
            s.store_add_ad_rhs(1368, 1277, A::mul(s.ad_value(1168), {
                if (s.v[1179] > 1e-38) {
                    A::ln(s.ad_value(1179))
                } else {
                    A::neg(A::constant(87.49823353377374))
                }
            }));
        }

        if (!s.b[1540]) {
            s.store_div_ad_rhs(1179, 757, A::add(s.ad_value(757), A::div_from_scalar(1.0, A::offset(A::div_from_scalar(1.0, s.ad_value(754)), (1.0 / s.v[1248])))));
            s.store_sub_ad_rhs(1369, 1368, A::mul(s.ad_value(1179), s.ad_value(1376)));
        }

        s.b[1555] = (s.v[404] == 0.0);
        s.v[1555] = if s.b[1555] { 1.0 } else { 0.0 };

        if ((!s.b[1540]) && s.b[1555]) {
            s.store_div_ad_lhs(1179, A::scale(A::neg(s.ad_value(591)), s.v[688]), 489);
            s.store_mul_add_ad_rhs(1180, 590, A::exp(A::scale(s.ad_value(1179), 0.5)), A::scale(A::exp(s.ad_value(1179)), 2.0));
            s.store_mul_sub_rhs(1181, 1180, 1275, 1277);
            s.store_scaled_div(1182, 705, 754, 0.5);
            s.store_add_ad_lhs(1370, A::add(A::sub(s.ad_value(1369), s.ad_value(1182)), s.ad_value(582)), 1181);
            s.store_offset_scaled(1179, 754, 1.0 / (s.v[1248]), 1.0);
            s.store_div_ad_lhs(1182, A::scale(A::neg(s.ad_value(589)), s.v[688]), 489);
            s.store_mul_add_ad_rhs(1184, 588, A::exp(A::scale(s.ad_value(1182), 0.5)), A::scale(A::exp(s.ad_value(1182)), 2.0));
            s.store_div_ad_lhs(1180, A::sub(s.ad_value(587), s.ad_value(1184)), 1179);
        }

    }
}
