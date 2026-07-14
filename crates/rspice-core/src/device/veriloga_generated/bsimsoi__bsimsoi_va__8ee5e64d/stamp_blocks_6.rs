#![allow(dead_code, non_snake_case, unused_assignments, unused_imports, unused_parens, unused_variables)]
use super::{A, ddt_jacobian, eval_ddt, eval_idt, GeneratedDerivative, GeneratedEvalContext, GeneratedReactiveStamper, GeneratedStamper, idt_jacobian, ReactiveScratch, Scratch, LIMEXP_MAX, THERMAL_VOLTAGE_PER_K};
use super::super::state::{Instance, Parameters};
impl Instance {
    #[inline(never)]
    pub(super) fn stamp_reactive_block_42(
        s: &mut ReactiveScratch,
    ) {
        if ((s.b[1722] && s.b[1723]) && s.b[1724]) {s.store_mul_exp_mixed_ia(1450, 1146, A::div_scaled_value_by_product(s.ad_value(781), -1.0, s.ad_value(1352), A::square(s.ad_value(1168)), 1.0));}
        if ((s.b[1722] && s.b[1723]) && s.b[1724]) {
            s.store_mul_mixed_ia(1451, 1352, {
                            if ((1.0 + s.v[1450]) > 1e-38) {
                                A::ln(A::offset(s.ad_value(1450), 1.0))
                            } else {
                                A::neg(A::constant(87.49823353377374))
                            }
                        });
        }
        s.b[1725] = (s.v[69] == 1.0);s.store_scalar(1725, if s.b[1725] { 1.0 } else { 0.0 });s.b[1726] = ((s.v[1145] > (-100.0)) && (s.v[1145] < 100.0));s.store_scalar(1726, if s.b[1726] { 1.0 } else { 0.0 });
        if (((!s.b[1722]) && s.b[1725]) && s.b[1726]) {s.store_exp_ad(1146, A::div(s.ad_value(1145), A::mul(s.ad_value(745), s.ad_value(724))));s.store_mul_mixed_ia(1146, 1146, A::exp_scaled_input(A::div(s.ad_value(685), s.ad_value(1351)), -1.0));}
        if (((!s.b[1722]) && s.b[1725]) && s.b[1726]) {
            s.store_mul_mixed_ia(1210, 1351, {
                            if ((1.0 + s.v[1146]) > 1e-38) {
                                A::ln(A::offset(s.ad_value(1146), 1.0))
                            } else {
                                A::neg(A::constant(87.49823353377374))
                            }
                        });
        }
        s.b[1727] = (s.v[63] > 0.0);s.store_scalar(1727, if s.b[1727] { 1.0 } else { 0.0 });
        if ((((!s.b[1722]) && s.b[1725]) && s.b[1726]) && s.b[1727]) {s.store_mul_exp_mixed_ia(1450, 1146, A::div_scaled_value_by_product(s.ad_value(781), -1.0, s.ad_value(1352), A::square(s.ad_value(1168)), 1.0));}
        if ((((!s.b[1722]) && s.b[1725]) && s.b[1726]) && s.b[1727]) {
            s.store_mul_mixed_ia(1451, 1352, {
                            if ((1.0 + s.v[1450]) > 1e-38) {
                                A::ln(A::offset(s.ad_value(1450), 1.0))
                            } else {
                                A::neg(A::constant(87.49823353377374))
                            }
                        });
        }
        if ((!s.b[1722]) && (!s.b[1725])) {s.store_div_scaled_product_mixed_iai(1145, 749, A::sub(s.ad_value(1166), s.ad_value(685)), 1.0, 1351, 1.0);s.store_div_scaled_inputs2_mixed_iai(1169, 751, 1.0, A::mul_sub_from_scalar_lhs(1.0, s.ad_value(749), A::sub(s.ad_value(1166), s.ad_value(685))), (-1.0), 1351, 1.0);}
        s.b[1728] = (s.v[1145] > 100.0);s.store_scalar(1728, if s.b[1728] { 1.0 } else { 0.0 });
        if (((!s.b[1722]) && (!s.b[1725])) && s.b[1728]) {s.store_sub(1210, 1166, 685);}
        s.b[1729] = (s.v[1169] > 100.0);s.store_scalar(1729, if s.b[1729] { 1.0 } else { 0.0 });
        if ((((!s.b[1722]) && (!s.b[1725])) && (!s.b[1728])) && s.b[1729]) {s.store_div_scaled_inputs3_indices(1179, 1166, 1.0, 685, (-1.0), 751, -1.0, 1351, 1.0);s.store_exp(1146, 1179);s.store_mul_div_scaled_product_indices(1210, 1146, 1168, 1473, 1.0, 757, 1.0);}
        if ((((!s.b[1722]) && (!s.b[1725])) && (!s.b[1728])) && (!s.b[1729])) {s.store_exp(1146, 1145);}
        if ((((!s.b[1722]) && (!s.b[1725])) && (!s.b[1728])) && (!s.b[1729])) {
            s.store_mul_mixed_ia(1180, 1351, {
                            if ((1.0 + s.v[1146]) > 1e-38) {
                                A::ln(A::offset(s.ad_value(1146), 1.0))
                            } else {
                                A::neg(A::constant(87.49823353377374))
                            }
                        });
        }
        if ((((!s.b[1722]) && (!s.b[1725])) && (!s.b[1728])) && (!s.b[1729])) {s.store_mul3_ad(1192, A::div_scaled_inputs(s.ad_value(757), -1.0, A::mul(s.ad_value(1168), s.ad_value(1473)), 1.0), A::exp(s.ad_value(1169)), A::sub_from_scalar(1.0, s.ad_value(749)));s.store_sub_mixed_ia(1181, 749, A::div_scaled_product(s.ad_value(1351), s.ad_value(1192), 1.0, A::sub_from_scalar(1.0, s.ad_value(749)), 1.0));s.store_div(1210, 1180, 1181);}
        s.b[1730] = (s.v[63] > 0.0);s.store_scalar(1730, if s.b[1730] { 1.0 } else { 0.0 });
        if (((!s.b[1722]) && (!s.b[1725])) && s.b[1730]) {s.store_div_scaled_product_mixed_iai(1452, 749, A::add_scaled_inputs3(s.ad_value(1166), 1.0, s.ad_value(685), (-1.0), s.ad_value(781), -1.0), 1.0, 1352, 1.0);}
    }
    #[inline(never)]
    pub(super) fn stamp_reactive_block_43(
        s: &mut ReactiveScratch,
    ) {
        if (((!s.b[1722]) && (!s.b[1725])) && s.b[1730]) {s.store_div_scaled_inputs2_mixed_iai(1453, 751, 1.0, A::mul_sub_from_scalar_lhs(1.0, s.ad_value(749), A::add_scaled_inputs3(s.ad_value(1166), 1.0, s.ad_value(685), (-1.0), s.ad_value(781), -1.0)), (-1.0), 1352, 1.0);}
        s.b[1731] = (s.v[1452] > 100.0);s.store_scalar(1731, if s.b[1731] { 1.0 } else { 0.0 });
        if ((((!s.b[1722]) && (!s.b[1725])) && s.b[1730]) && s.b[1731]) {s.store_add_scaled_inputs3_indices(1451, 1166, 1.0, 685, (-1.0), 781, -1.0);}
        s.b[1732] = (s.v[1453] > 100.0);s.store_scalar(1732, if s.b[1732] { 1.0 } else { 0.0 });
        if (((((!s.b[1722]) && (!s.b[1725])) && s.b[1730]) && (!s.b[1731])) && s.b[1732]) {s.store_div_scaled_inputs4_indices(1179, 1166, 1.0, 685, (-1.0), 751, -1.0, 781, -1.0, 1352, 1.0);s.store_exp(1450, 1179);s.store_mul_div_scaled_product_indices(1451, 1450, 1168, 1473, 1.0, 757, 1.0);}
        if (((((!s.b[1722]) && (!s.b[1725])) && s.b[1730]) && (!s.b[1731])) && (!s.b[1732])) {s.store_exp(1450, 1452);}
        if (((((!s.b[1722]) && (!s.b[1725])) && s.b[1730]) && (!s.b[1731])) && (!s.b[1732])) {
            s.store_mul_mixed_ia(1180, 1352, {
                            if ((1.0 + s.v[1450]) > 1e-38) {
                                A::ln(A::offset(s.ad_value(1450), 1.0))
                            } else {
                                A::neg(A::constant(87.49823353377374))
                            }
                        });
        }
        if (((((!s.b[1722]) && (!s.b[1725])) && s.b[1730]) && (!s.b[1731])) && (!s.b[1732])) {s.store_mul3_ad(1192, A::div_scaled_inputs(s.ad_value(757), -1.0, A::mul(s.ad_value(1168), s.ad_value(1473)), 1.0), A::exp(s.ad_value(1453)), A::sub_from_scalar(1.0, s.ad_value(749)));s.store_sub_mixed_ia(1181, 749, A::div_scaled_product(s.ad_value(1352), s.ad_value(1192), 1.0, A::sub_from_scalar(1.0, s.ad_value(749)), 1.0));s.store_div(1451, 1180, 1181);}
        s.copy_ad(1165, 1407);s.copy_ad(1164, 1388);s.copy_ad(1177, 1378);s.b[1733] = (s.v[88] == 2.0);s.store_scalar(1733, if s.b[1733] { 1.0 } else { 0.0 });s.b[1734] = (s.v[57] == 2.0);s.store_scalar(1734, if s.b[1734] { 1.0 } else { 0.0 });
        if (s.b[1733] && s.b[1734]) {s.store_scalar(1273, 0.0);s.store_scalar(1272, 0.0);}
        if (s.b[1733] && (!s.b[1734])) {s.store_add_mixed_ai(1162, A::add_scaled_inputs_product(s.ad_value(1165), 1.0, s.ad_value(1277), (-1.0), s.ad_value(707), s.ad_value(1164), (-1.0)), 685);s.store_add_scaled_inputs3_offset_indices(1149, 1162, 1.0, 1161, (-1.0), 1177, 1.0, (-0.08));}
        s.b[1735] = (s.v[1162] <= 0.0);s.store_scalar(1735, if s.b[1735] { 1.0 } else { 0.0 });
        if ((s.b[1733] && (!s.b[1734])) && s.b[1735]) {s.store_sqrt_add_scaled_square_input(1179, 1149, 1.0, 1162, (-(4.0 * 0.08)));}
        if ((s.b[1733] && (!s.b[1734])) && (!s.b[1735])) {s.store_sqrt_add_scaled_square_input(1179, 1149, 1.0, 1162, (4.0 * 0.08));}
        if (s.b[1733] && (!s.b[1734])) {s.store_add_scaled_inputs3_indices(1148, 1162, 1.0, 1149, (-0.5), 1179, (-0.5));s.store_mul_sub_rhs(1273, 1316, 1148, 1162);}
        s.b[1736] = (((s.v[57] != 2.0) && (s.v[760] != 0.0)) && (s.v[63] > 0.0));s.store_scalar(1736, if s.b[1736] { 1.0 } else { 0.0 });
        if ((s.b[1733] && (!s.b[1734])) && s.b[1736]) {s.store_add(1460, 1162, 781);s.store_scalar(1472, 0.08);s.store_add_scaled_inputs4_indices(1149, 1460, 1.0, 1458, (-1.0), 1177, 1.0, 1472, -1.0);}
        s.b[1737] = (s.v[1460] <= 0.0);s.store_scalar(1737, if s.b[1737] { 1.0 } else { 0.0 });
        if (((s.b[1733] && (!s.b[1734])) && s.b[1736]) && s.b[1737]) {s.store_sqrt_add_scaled_square_product(1179, 1149, 1.0, 1472, 1460, (-100.0));}
        if (((s.b[1733] && (!s.b[1734])) && s.b[1736]) && (!s.b[1737])) {s.store_sqrt_add_scaled_square_product(1179, 1149, 1.0, 1472, 1460, 100.0);}
        if ((s.b[1733] && (!s.b[1734])) && s.b[1736]) {s.store_add_scaled_inputs3_indices(1461, 1460, 1.0, 1149, (-0.5), 1179, (-0.5));s.store_add_scaled_product_right_sub(1273, 1273, 1.0, 1449, 1461, 1460, 1.0);}
        if (s.b[1733] && (!s.b[1734])) {s.store_scale(1179, 737, 0.5);s.store_add_scaled_inputs4_indices(1182, 1161, 1.0, 1148, (-1.0), 1177, -1.0, 1210, -1.0);}
        s.b[1738] = (s.v[737] == 0.0);s.store_scalar(1738, if s.b[1738] { 1.0 } else { 0.0 });
        if ((s.b[1733] && (!s.b[1734])) && s.b[1738]) {s.store_scalar(1180, 0.0);}
        s.b[1739] = (s.v[1182] < 0.0);s.store_scalar(1739, if s.b[1739] { 1.0 } else { 0.0 });
        if (((s.b[1733] && (!s.b[1734])) && (!s.b[1738])) && s.b[1739]) {s.store_add_div_rhs_indices(1180, 1179, 1182, 737);}
        if (((s.b[1733] && (!s.b[1734])) && (!s.b[1738])) && (!s.b[1739])) {s.store_sqrt_square_add(1180, 1179, 1182);}
        if (s.b[1733] && (!s.b[1734])) {s.store_mul_ad_product_rhs_mixed_ia(1272, 1316, 737, A::sub(s.ad_value(1180), s.ad_value(1179)));}
        s.b[1740] = (((s.v[57] != 2.0) && (s.v[760] != 0.0)) && (s.v[63] > 0.0));s.store_scalar(1740, if s.b[1740] { 1.0 } else { 0.0 });
        if ((s.b[1733] && (!s.b[1734])) && s.b[1740]) {s.store_add_scaled_inputs4_indices(1182, 1458, 1.0, 1461, (-1.0), 1177, -1.0, 1451, -1.0);}
        s.b[1741] = (s.v[1182] < 0.0);s.store_scalar(1741, if s.b[1741] { 1.0 } else { 0.0 });
        if (((s.b[1733] && (!s.b[1734])) && s.b[1740]) && s.b[1741]) {s.store_add_div_rhs_indices(1180, 1179, 1182, 737);}
        if (((s.b[1733] && (!s.b[1734])) && s.b[1740]) && (!s.b[1741])) {s.store_sqrt_square_add(1180, 1179, 1182);}
    }
    #[inline(never)]
    pub(super) fn stamp_reactive_block_44(
        s: &mut ReactiveScratch,
    ) {
        if ((s.b[1733] && (!s.b[1734])) && s.b[1740]) {s.store_add_product3_rhs_mixed_iia(1272, 1272, 1449, 737, A::sub(s.ad_value(1180), s.ad_value(1179)), 1.0);}
        if s.b[1733] {s.store_scale(1229, 1196, s.v[694]);s.store_div(1226, 1210, 1229);s.store_offset_sub(1150, 1226, 1158, (-0.02));s.store_sqrt_add_scaled_square_input(1179, 1150, 1.0, 1226, (4.0 * 0.02));s.store_add_scaled_inputs3_indices(1212, 1226, 1.0, 1150, (-0.5), 1179, (-0.5));}
        s.b[1742] = (s.v[63] > 0.0);s.store_scalar(1742, if s.b[1742] { 1.0 } else { 0.0 });
        if (s.b[1733] && s.b[1742]) {s.store_div(1462, 1451, 1229);s.store_offset_sub(1150, 1462, 1158, (-0.02));s.store_sqrt_add_scaled_square_input(1179, 1150, 1.0, 1462, (4.0 * 0.02));s.store_add_scaled_inputs3_indices(1463, 1462, 1.0, 1150, (-0.5), 1179, (-0.5));}
        s.b[1743] = (s.v[57] == 2.0);s.store_scalar(1743, if s.b[1743] { 1.0 } else { 0.0 });
        if (s.b[1733] && s.b[1743]) {s.store_scalar(1341, 0.0);}
        if (s.b[1733] && (!s.b[1743])) {s.store_mul(1179, 1229, 1212);s.store_scaled_offset_ad(1180, A::sub_scaled_inputs(s.ad_value(1210), 1.0, s.ad_value(1179), 0.5), 1e-20, 12.0);s.store_div(1181, 1212, 1180);s.store_mul(1182, 1179, 1181);s.store_sub_from_scalar(1186, 1.0, 1229);s.store_mul_ad_product_rhs_mixed_ia(1341, 1316, 1186, A::sub_scaled_inputs(s.ad_value(1212), 0.5, s.ad_value(1182), 1.0));}
        s.b[1744] = (((s.v[57] != 2.0) && (s.v[760] != 0.0)) && (s.v[63] > 0.0));s.store_scalar(1744, if s.b[1744] { 1.0 } else { 0.0 });
        if ((s.b[1733] && (!s.b[1743])) && s.b[1744]) {s.store_mul(1179, 1229, 1463);s.store_scaled_offset_ad(1180, A::sub_scaled_inputs(s.ad_value(1451), 1.0, s.ad_value(1179), 0.5), 1e-20, 12.0);s.store_div(1181, 1463, 1180);s.store_mul(1182, 1179, 1181);s.store_sub_from_scalar(1186, 1.0, 1229);s.store_add_product3_rhs_mixed_iia(1341, 1341, 1449, 1186, A::sub_scaled_inputs(s.ad_value(1463), 0.5, s.ad_value(1182), 1.0), 1.0);}
        if s.b[1733] {s.store_mul(1179, 1229, 1212);s.store_scaled_offset_ad(1180, A::sub_scaled_inputs(s.ad_value(1210), 1.0, s.ad_value(1179), 0.5), 1e-20, 12.0);s.store_div(1181, 1179, 1180);s.store_mul(1182, 1179, 1181);s.store_mul_add_scaled_inputs3_offset_rhs_indices(1250, 1178, 1210, 1.0, 1179, (-0.5), 1182, 1.0, 0.0);}
        s.b[1745] = (((s.v[57] != 2.0) && (s.v[760] != 0.0)) && (s.v[63] > 0.0));s.store_scalar(1745, if s.b[1745] { 1.0 } else { 0.0 });
        if (s.b[1733] && s.b[1745]) {s.store_mul(1454, 1229, 1463);s.store_scaled_offset_ad(1191, A::sub_scaled_inputs(s.ad_value(1451), 1.0, s.ad_value(1454), 0.5), 1e-20, 12.0);s.store_div(1181, 1454, 1191);s.store_mul(1182, 1454, 1181);s.store_add_scaled_product_mixed_iia(1250, 1250, 1.0, 1448, A::add_scaled_inputs3(s.ad_value(1451), 1.0, s.ad_value(1454), (-0.5), s.ad_value(1182), 1.0), 1.0);}
        s.b[1746] = (s.v[153] > 0.5);s.store_scalar(1746, if s.b[1746] { 1.0 } else { 0.0 });
        if (s.b[1733] && s.b[1746]) {s.store_scale(1180, 1180, 2.0);s.store_mul_add_scaled_inputs3_offset_rhs_mixed_iia(1254, 1178, 1210, ((0.5) * (-1.0)), 1179, ((0.25) * (-1.0)), A::div_scaled_product(s.ad_value(1179), s.ad_value(1179), 1.0, s.ad_value(1180), 1.0), ((-1.0) * (-1.0)), 0.0);}
        s.b[1747] = (((s.v[57] != 2.0) && (s.v[760] != 0.0)) && (s.v[63] > 0.0));s.store_scalar(1747, if s.b[1747] { 1.0 } else { 0.0 });
        if ((s.b[1733] && s.b[1746]) && s.b[1747]) {s.store_scale(1191, 1191, 2.0);s.store_add_scaled_product_mixed_iia(1254, 1254, 1.0, 1448, A::add_scaled_inputs3(s.ad_value(1451), 0.5, s.ad_value(1454), 0.25, A::div_scaled_product(s.ad_value(1454), s.ad_value(1454), 1.0, s.ad_value(1191), 1.0), -1.0), (-1.0));}
        s.b[1748] = (s.v[153] < 0.5);s.store_scalar(1748, if s.b[1748] { 1.0 } else { 0.0 });
        if ((s.b[1733] && (!s.b[1746])) && s.b[1748]) {s.store_scale(1180, 1180, 0.08333333333333333);s.store_div_scaled_inputs_square_rhs(1181, 1178, 0.5, 1180, 1.0);}
    }
    #[inline(never)]
    pub(super) fn stamp_reactive_block_45(
        s: &mut ReactiveScratch,
        p: &Parameters,
    ) {
        if ((s.b[1733] && (!s.b[1746])) && s.b[1748]) {s.store_add_scaled_product_mixed_aia(1182, A::mul3_scaled_output(s.ad_value(1179), s.ad_value(1179), s.ad_value(1179), (2.0 * 0.06666666666666667)), (-1.0), 1210, A::add_scaled_products(s.ad_value(1179), s.ad_value(1179), (2.0 * 0.3333333333333333), s.ad_value(1210), A::sub_scaled_inputs(s.ad_value(1210), 1.0, s.ad_value(1179), (4.0 * 0.3333333333333333)), 1.0), 1.0);s.store_mul_scale_offset_indices(1254, 1182, 1181, -1.0, 0.0);}
        s.b[1749] = (((s.v[57] != 2.0) && (s.v[760] != 0.0)) && (s.v[63] > 0.0));s.store_scalar(1749, if s.b[1749] { 1.0 } else { 0.0 });
        if (((s.b[1733] && (!s.b[1746])) && s.b[1748]) && s.b[1749]) {s.store_scale(1191, 1191, 0.08333333333333333);s.store_div_scaled_inputs_square_rhs(1181, 1448, 0.5, 1191, 1.0);s.store_add_scaled_product_mixed_aia(1182, A::mul3_scaled_output(s.ad_value(1454), s.ad_value(1454), s.ad_value(1454), (2.0 * 0.06666666666666667)), (-1.0), 1451, A::add_scaled_products(s.ad_value(1454), s.ad_value(1454), (2.0 * 0.3333333333333333), s.ad_value(1451), A::sub_scaled_inputs(s.ad_value(1451), 1.0, s.ad_value(1454), (4.0 * 0.3333333333333333)), 1.0), 1.0);s.store_mul_scale_offset_indices(1470, 1182, 1181, -1.0, 0.0);s.store_add(1254, 1254, 1470);}
        if ((s.b[1733] && (!s.b[1746])) && (!s.b[1748])) {s.store_scaled_add(1254, 1250, 1341, (-0.5));}
        s.b[1750] = (s.v[57] == 2.0);s.store_scalar(1750, if s.b[1750] { 1.0 } else { 0.0 });
        if (s.b[1733] && s.b[1750]) {s.store_scalar(1274, 0.0);}
        if (s.b[1733] && (!s.b[1750])) {s.store_scale(1249, 626, (s.v[342] * (s.v[1248] * ((((s.v[693] / s.v[59]) * s.v[39]) * s.v[727]) + s.v[65]))));s.store_mul_sub_rhs(1274, 1249, 1237, 1160);}
        if s.b[1733] {s.store_add_scaled_inputs3_indices(1251, 1250, 1.0, 1273, 1.0, 1272, 1.0);s.store_add_scaled_inputs4_indices(1252, 1341, 1.0, 1273, (-1.0), 1272, -1.0, 1274, -1.0);s.copy_ad(1255, 1274);s.store_add_scaled_inputs4_indices(1253, 1251, (-1.0), 1254, (-1.0), 1252, (-1.0), 1255, (-1.0));}
        s.b[1751] = (s.v[88] == 3.0);s.store_scalar(1751, if s.b[1751] { 1.0 } else { 0.0 });s.b[1752] = (s.v[68] == 0.0);s.store_scalar(1752, if s.b[1752] { 1.0 } else { 0.0 });
        if (((!s.b[1733]) && s.b[1751]) && s.b[1752]) {s.store_div_from_scalar(1332, 3.453133e-11, 92);}
        if (((!s.b[1733]) && s.b[1751]) && (!s.b[1752])) {s.store_div_scaled_inputs_indices(1332, 777, 8.85418e-12, 92, 1.0);}
        if ((!s.b[1733]) && s.b[1751]) {s.store_div_scaled_product_indices(1178, 1178, 776, 1.0, 92, 1.0);s.store_div_scaled_inputs_indices(1316, 1316, s.v[91], 92, 1.0);s.store_scale(1333, 92, 100000000.0);}
        s.b[1753] = (s.v[63] > 0.0);s.store_scalar(1753, if s.b[1753] { 1.0 } else { 0.0 });
        if (((!s.b[1733]) && s.b[1751]) && s.b[1753]) {s.store_div_scaled_inputs_indices(1448, 1448, s.v[91], 92, 1.0);s.store_div_scaled_inputs_indices(1449, 1449, s.v[91], 92, 1.0);}
        s.b[1754] = (s.v[57] == 2.0);s.store_scalar(1754, if s.b[1754] { 1.0 } else { 0.0 });
        if (((!s.b[1733]) && s.b[1751]) && s.b[1754]) {s.store_scalar(1273, 0.0);s.store_scalar(1272, 0.0);s.store_scalar(1350, 0.0);}
        s.b[1755] = ((p.p33 == 1.0) && (p.p16 != 0.0));s.store_scalar(1755, if s.b[1755] { 1.0 } else { 0.0 });
        if ((((!s.b[1733]) && s.b[1751]) && (!s.b[1754])) && s.b[1755]) {s.store_add_mixed_ai(1350, A::add_scaled_inputs_product(s.ad_value(1349), 1.0, s.ad_value(1277), (-1.0), s.ad_value(707), s.ad_value(1278), (-1.0)), 685);}
        if ((((!s.b[1733]) && s.b[1751]) && (!s.b[1754])) && (!s.b[1755])) {s.store_add(1350, 424, 685);}
        if (((!s.b[1733]) && s.b[1751]) && (!s.b[1754])) {s.store_add_scaled_inputs3_offset_indices(1149, 1350, 1.0, 1161, (-1.0), 1177, 1.0, (-0.02));}
        s.b[1756] = (s.v[1350] <= 0.0);s.store_scalar(1756, if s.b[1756] { 1.0 } else { 0.0 });
        if ((((!s.b[1733]) && s.b[1751]) && (!s.b[1754])) && s.b[1756]) {s.store_sqrt_add_scaled_square_input(1179, 1149, 1.0, 1350, (-(4.0 * 0.02)));}
        if ((((!s.b[1733]) && s.b[1751]) && (!s.b[1754])) && (!s.b[1756])) {s.store_sqrt_add_scaled_square_input(1179, 1149, 1.0, 1350, (4.0 * 0.02));}
        if (((!s.b[1733]) && s.b[1751]) && (!s.b[1754])) {s.store_add_scaled_inputs3_indices(1148, 1350, 1.0, 1149, (-0.5), 1179, (-0.5));}
        s.b[1757] = (s.v[63] > 0.0);s.store_scalar(1757, if s.b[1757] { 1.0 } else { 0.0 });
        if ((((!s.b[1733]) && s.b[1751]) && (!s.b[1754])) && s.b[1757]) {s.store_add(1459, 1350, 781);s.store_add_scaled_inputs3_offset_indices(1149, 1459, 1.0, 1458, (-1.0), 1177, 1.0, (-0.02));}
        s.b[1758] = (s.v[1459] <= 0.0);s.store_scalar(1758, if s.b[1758] { 1.0 } else { 0.0 });
        if (((((!s.b[1733]) && s.b[1751]) && (!s.b[1754])) && s.b[1757]) && s.b[1758]) {s.store_sqrt_add_scaled_square_input(1179, 1149, 1.0, 1459, (-(100.0 * 0.02)));}
        if (((((!s.b[1733]) && s.b[1751]) && (!s.b[1754])) && s.b[1757]) && (!s.b[1758])) {s.store_sqrt_add_scaled_square_input(1179, 1149, 1.0, 1459, (100.0 * 0.02));}
        if ((((!s.b[1733]) && s.b[1751]) && (!s.b[1754])) && s.b[1757]) {s.store_add_scaled_inputs3_indices(1461, 1459, 1.0, 1149, (-0.5), 1179, (-0.5));}
        if (((!s.b[1733]) && s.b[1751]) && (!s.b[1754])) {s.store_div_scaled_inputs3_indices(1179, 1161, 1.0, 1177, (-1.0), 1350, -1.0, 1333, 1.0);s.store_mul(1194, 1179, 722);}
        s.b[1759] = (((-100.0) < s.v[1194]) && (s.v[1194] < 100.0));s.store_scalar(1759, if s.b[1759] { 1.0 } else { 0.0 });
        if ((((!s.b[1733]) && s.b[1751]) && (!s.b[1754])) && s.b[1759]) {s.store_mul_exp_rhs(1334, 721, 1194);}
        s.b[1760] = (s.v[1194] <= (-100.0));s.store_scalar(1760, if s.b[1760] { 1.0 } else { 0.0 });
        if (((((!s.b[1733]) && s.b[1751]) && (!s.b[1754])) && (!s.b[1759])) && s.b[1760]) {s.store_scale(1334, 721, 3.720075976e-44);}
        if (((((!s.b[1733]) && s.b[1751]) && (!s.b[1754])) && (!s.b[1759])) && (!s.b[1760])) {s.store_scale(1334, 721, 2.688117142e43);}
        if (((!s.b[1733]) && s.b[1751]) && (!s.b[1754])) {s.store_scale(1335, 92, 0.001);s.store_add_scaled_inputs3_indices(1149, 721, 1.0, 1334, (-1.0), 1335, -1.0);s.store_sqrt_add_scaled_square_product(1150, 1149, 1.0, 1335, 721, 4.0);s.store_add_scaled_inputs3_indices(1334, 721, 1.0, 1149, (-0.5), 1150, (-0.5));}
    }
    #[inline(never)]
    pub(super) fn stamp_reactive_block_46(
        s: &mut ReactiveScratch,
    ) {
        s.b[1761] = (s.v[1334] < 1e-15);s.store_scalar(1761, if s.b[1761] { 1.0 } else { 0.0 });
        if ((((!s.b[1733]) && s.b[1751]) && (!s.b[1754])) && s.b[1761]) {s.store_scalar(1334, 1e-15);}
        s.b[1762] = (s.v[63] > 0.0);s.store_scalar(1762, if s.b[1762] { 1.0 } else { 0.0 });
        if ((((!s.b[1733]) && s.b[1751]) && (!s.b[1754])) && s.b[1762]) {s.store_div_scaled_inputs3_indices(1179, 1458, 1.0, 1177, (-1.0), 1459, -1.0, 1333, 1.0);s.store_mul(1194, 1179, 722);}
        s.b[1763] = (((-100.0) < s.v[1194]) && (s.v[1194] < 100.0));s.store_scalar(1763, if s.b[1763] { 1.0 } else { 0.0 });
        if (((((!s.b[1733]) && s.b[1751]) && (!s.b[1754])) && s.b[1762]) && s.b[1763]) {s.store_mul_exp_rhs(1464, 721, 1194);}
        s.b[1764] = (s.v[1194] <= (-100.0));s.store_scalar(1764, if s.b[1764] { 1.0 } else { 0.0 });
        if ((((((!s.b[1733]) && s.b[1751]) && (!s.b[1754])) && s.b[1762]) && (!s.b[1763])) && s.b[1764]) {s.store_scale(1464, 721, 3.720075976e-44);}
        if ((((((!s.b[1733]) && s.b[1751]) && (!s.b[1754])) && s.b[1762]) && (!s.b[1763])) && (!s.b[1764])) {s.store_scale(1464, 721, 2.688117142e43);}
        if ((((!s.b[1733]) && s.b[1751]) && (!s.b[1754])) && s.b[1762]) {s.store_add_scaled_inputs3_indices(1149, 721, 1.0, 1464, (-1.0), 1335, -1.0);s.store_sqrt_add_scaled_square_product(1150, 1149, 1.0, 1335, 721, 4.0);s.store_add_scaled_inputs3_indices(1464, 721, 1.0, 1149, (-0.5), 1150, (-0.5));}
        s.b[1765] = (s.v[1464] < 1e-15);s.store_scalar(1765, if s.b[1765] { 1.0 } else { 0.0 });
        if (((((!s.b[1733]) && s.b[1751]) && (!s.b[1754])) && s.b[1762]) && s.b[1765]) {s.store_scalar(1464, 1e-15);}
        if (((!s.b[1733]) && s.b[1751]) && (!s.b[1754])) {s.store_div(1336, 778, 1334);s.store_div_add_scaled_inputs_rhs_indices(1181, 1332, 1332, 1.0, 1336, 1.0);s.store_mul(1337, 1181, 1336);}
        s.b[1766] = (((s.v[57] != 2.0) && (s.v[760] != 0.0)) && (s.v[63] > 0.0));s.store_scalar(1766, if s.b[1766] { 1.0 } else { 0.0 });
        if ((((!s.b[1733]) && s.b[1751]) && (!s.b[1754])) && s.b[1766]) {s.store_div(1465, 778, 1464);s.store_div_add_scaled_inputs_rhs_indices(1181, 1332, 1332, 1.0, 1465, 1.0);s.store_mul(1466, 1181, 1465);}
        if (((!s.b[1733]) && s.b[1751]) && (!s.b[1754])) {s.store_div_scaled_product_indices(1317, 1316, 1337, 1.0, 1332, 1.0);}
        s.b[1767] = (s.v[63] > 0.0);s.store_scalar(1767, if s.b[1767] { 1.0 } else { 0.0 });
        if ((((!s.b[1733]) && s.b[1751]) && (!s.b[1754])) && s.b[1767]) {s.store_div_scaled_product_indices(1468, 1449, 1466, 1.0, 1332, 1.0);}
        if (((!s.b[1733]) && s.b[1751]) && (!s.b[1754])) {s.store_mul_sub_rhs(1273, 1317, 1148, 1350);}
        s.b[1768] = (((s.v[57] != 2.0) && (s.v[760] != 0.0)) && (s.v[63] > 0.0));s.store_scalar(1768, if s.b[1768] { 1.0 } else { 0.0 });
        if ((((!s.b[1733]) && s.b[1751]) && (!s.b[1754])) && s.b[1768]) {s.store_mul_sub_rhs(1456, 1468, 1461, 1459);s.store_add(1273, 1273, 1456);}
        if (((!s.b[1733]) && s.b[1751]) && (!s.b[1754])) {s.store_scale(1179, 737, 0.5);s.store_add_scaled_inputs4_indices(1182, 1161, 1.0, 1148, (-1.0), 1177, -1.0, 1210, -1.0);}
        s.b[1769] = (s.v[737] == 0.0);s.store_scalar(1769, if s.b[1769] { 1.0 } else { 0.0 });
        if ((((!s.b[1733]) && s.b[1751]) && (!s.b[1754])) && s.b[1769]) {s.store_scalar(1180, 0.0);}
        s.b[1770] = (s.v[1182] < 0.0);s.store_scalar(1770, if s.b[1770] { 1.0 } else { 0.0 });
        if (((((!s.b[1733]) && s.b[1751]) && (!s.b[1754])) && (!s.b[1769])) && s.b[1770]) {s.store_add_div_rhs_indices(1180, 1179, 1182, 737);}
        if (((((!s.b[1733]) && s.b[1751]) && (!s.b[1754])) && (!s.b[1769])) && (!s.b[1770])) {s.store_sqrt_square_add(1180, 1179, 1182);}
        if (((!s.b[1733]) && s.b[1751]) && (!s.b[1754])) {s.store_mul_ad_product_rhs_mixed_ia(1272, 1317, 737, A::sub(s.ad_value(1180), s.ad_value(1179)));}
        s.b[1771] = (((s.v[57] != 2.0) && (s.v[760] != 0.0)) && (s.v[63] > 0.0));s.store_scalar(1771, if s.b[1771] { 1.0 } else { 0.0 });
        if ((((!s.b[1733]) && s.b[1751]) && (!s.b[1754])) && s.b[1771]) {s.store_add_scaled_inputs4_indices(1182, 1458, 1.0, 1461, (-1.0), 1177, -1.0, 1451, -1.0);}
        s.b[1772] = (s.v[737] == 0.0);s.store_scalar(1772, if s.b[1772] { 1.0 } else { 0.0 });
        if (((((!s.b[1733]) && s.b[1751]) && (!s.b[1754])) && s.b[1771]) && s.b[1772]) {s.store_scalar(1180, 0.0);}
        s.b[1773] = (s.v[1182] < 0.0);s.store_scalar(1773, if s.b[1773] { 1.0 } else { 0.0 });
        if ((((((!s.b[1733]) && s.b[1751]) && (!s.b[1754])) && s.b[1771]) && (!s.b[1772])) && s.b[1773]) {s.store_add_div_rhs_indices(1180, 1179, 1182, 737);}
        if ((((((!s.b[1733]) && s.b[1751]) && (!s.b[1754])) && s.b[1771]) && (!s.b[1772])) && (!s.b[1773])) {s.store_sqrt_square_add(1180, 1179, 1182);}
        if ((((!s.b[1733]) && s.b[1751]) && (!s.b[1754])) && s.b[1771]) {s.store_mul_ad_product_rhs_mixed_ia(1457, 1468, 737, A::sub(s.ad_value(1180), s.ad_value(1179)));s.store_add(1272, 1272, 1457);}
        s.b[1774] = (s.v[737] <= 0.0);s.store_scalar(1774, if s.b[1774] { 1.0 } else { 0.0 });
        if (((!s.b[1733]) && s.b[1751]) && s.b[1774]) {s.store_scaled_mul(1271, 723, 1168, 0.25);s.store_scale(1179, 700, 0.5);}
        if (((!s.b[1733]) && s.b[1751]) && (!s.b[1774])) {s.store_mul_product3_indices(1271, 737, 723, 1168, 737, 1.0);s.store_mul(1179, 737, 700);}
        if ((!s.b[1733]) && s.b[1751]) {s.store_add_scaled_inputs(1180, 1179, 2.0, 1210, 1.0);}
        if ((!s.b[1733]) && s.b[1751]) {
            s.store_mul_mixed_ia(1339, 1168, {
                            if ((1.0 + ((s.v[1180] * s.v[1210]) / s.v[1271])) > 1e-38) {
                                A::ln(A::offset(A::div_scaled_product(s.ad_value(1180), s.ad_value(1210), 1.0, s.ad_value(1271), 1.0), 1.0))
                            } else {
                                A::neg(A::constant(87.49823353377374))
                            }
                        });
        }
        s.b[1775] = (s.v[63] > 0.0);s.store_scalar(1775, if s.b[1775] { 1.0 } else { 0.0 });
        if (((!s.b[1733]) && s.b[1751]) && s.b[1775]) {s.store_add_scaled_inputs(1180, 1179, 2.0, 1451, 1.0);}
        if (((!s.b[1733]) && s.b[1751]) && s.b[1775]) {
            s.store_mul_mixed_ia(1469, 1168, {
                            if ((1.0 + ((s.v[1180] * s.v[1451]) / s.v[1271])) > 1e-38) {
                                A::ln(A::offset(A::div_scaled_product(s.ad_value(1180), s.ad_value(1451), 1.0, s.ad_value(1271), 1.0), 1.0))
                            } else {
                                A::neg(A::constant(87.49823353377374))
                            }
                        });
        }
        if ((!s.b[1733]) && s.b[1751]) {s.store_add_scaled_inputs3_indices(1182, 1165, 4.0, 1350, ((-1.0) * 4.0), 1277, (-4.0));s.store_sqrt_square_offset(1181, 1182, 0.0001);s.store_scaled_add(1183, 1182, 1181, 0.5);}
    }
    #[inline(never)]
    pub(super) fn stamp_reactive_block_47(
        s: &mut ReactiveScratch,
    ) {
        if ((!s.b[1733]) && s.b[1751]) {s.store_scale(1333, 1333, 2.0);s.store_div_scaled_inputs2_indices(1179, 1210, 1.0, 1183, 1.0, 1333, 1.0);}
        if ((!s.b[1733]) && s.b[1751]) {
            s.store_exp_scaled_input_ad(1194, {
                if (s.v[1179] > 1e-38) {
                    A::ln(s.ad_value(1179))
                } else {
                    A::neg(A::constant(87.49823353377374))
                }
            }, (s.v[86] * 0.7));
        }
        if ((!s.b[1733]) && s.b[1751]) {s.store_offset(1180, 1194, 1.0);s.store_div_from_scalar(1334, (s.v[85] * 1.9e-9), 1180);s.store_div(1336, 778, 1334);s.store_div_add_scaled_inputs_rhs_indices(1179, 1332, 1332, 1.0, 1336, 1.0);s.store_mul(1337, 1179, 1336);s.store_div_scaled_product_indices(1338, 1178, 1337, 1.0, 1332, 1.0);s.store_div_scaled_product_indices(1317, 1316, 1337, 1.0, 1332, 1.0);}
        s.b[1776] = (((s.v[57] != 2.0) && (s.v[760] != 0.0)) && (s.v[63] > 0.0));s.store_scalar(1776, if s.b[1776] { 1.0 } else { 0.0 });
        if (((!s.b[1733]) && s.b[1751]) && s.b[1776]) {s.store_add_scaled_inputs4_indices(1182, 1165, 4.0, 781, 4.0, 1459, (-4.0), 1277, (-4.0));s.store_sqrt_square_offset(1181, 1182, 0.0001);s.store_scaled_add(1183, 1182, 1181, 0.5);s.store_div_scaled_inputs2_indices(1179, 1451, 1.0, 1183, 1.0, 1333, 1.0);}
        if (((!s.b[1733]) && s.b[1751]) && s.b[1776]) {
            s.store_exp_scaled_input_ad(1194, {
                if (s.v[1179] > 1e-38) {
                    A::ln(s.ad_value(1179))
                } else {
                    A::neg(A::constant(87.49823353377374))
                }
            }, (s.v[86] * 0.7));
        }
        if (((!s.b[1733]) && s.b[1751]) && s.b[1776]) {s.store_offset(1180, 1194, 1.0);s.store_div_from_scalar(1464, (s.v[85] * 1.9e-9), 1180);s.store_div(1465, 778, 1464);s.store_div_add_scaled_inputs_rhs_indices(1179, 1332, 1332, 1.0, 1465, 1.0);s.store_mul(1466, 1179, 1465);s.store_div_scaled_product_indices(1467, 1448, 1466, 1.0, 1332, 1.0);s.store_div_scaled_product_indices(1468, 1449, 1466, 1.0, 1332, 1.0);}
        if ((!s.b[1733]) && s.b[1751]) {s.store_sub(1180, 1210, 1339);s.store_scale(1229, 1196, s.v[694]);s.store_div(1226, 1180, 1229);s.store_offset_sub(1150, 1226, 1158, (-0.02));s.store_sqrt_add_scaled_square_input(1179, 1150, 1.0, 1226, (4.0 * 0.02));s.store_add_scaled_inputs3_indices(1212, 1226, 1.0, 1150, (-0.5), 1179, (-0.5));s.store_mul(1179, 1229, 1212);s.store_scaled_offset_ad(1181, A::sub_scaled_inputs(s.ad_value(1180), 1.0, s.ad_value(1179), 0.5), 1e-20, 12.0);s.store_div(1182, 1179, 1181);s.store_mul_sub_mixed_iia(1250, 1338, 1180, A::mul_sub_from_scalar_rhs(s.ad_value(1179), 0.5, s.ad_value(1182)));s.copy_ad(1251, 1250);}
        s.b[1777] = (((s.v[57] != 2.0) && (s.v[760] != 0.0)) && (s.v[63] > 0.0));s.store_scalar(1777, if s.b[1777] { 1.0 } else { 0.0 });
        if (((!s.b[1733]) && s.b[1751]) && s.b[1777]) {s.store_sub(1191, 1451, 1469);s.store_div(1462, 1191, 1229);s.store_offset_sub(1150, 1462, 1158, (-0.02));s.store_sqrt_add_scaled_square_input(1454, 1150, 1.0, 1462, (4.0 * 0.02));s.store_add_scaled_inputs3_indices(1463, 1462, 1.0, 1150, (-0.5), 1454, (-0.5));s.store_mul(1454, 1229, 1463);s.store_scaled_offset_ad(1455, A::sub_scaled_inputs(s.ad_value(1191), 1.0, s.ad_value(1454), 0.5), 1e-20, 12.0);s.store_div(1182, 1454, 1455);s.store_mul_sub_mixed_iia(1186, 1467, 1191, A::mul_sub_from_scalar_rhs(s.ad_value(1454), 0.5, s.ad_value(1182)));s.store_add(1250, 1250, 1186);s.copy_ad(1251, 1250);}
        s.b[1778] = (s.v[57] == 2.0);s.store_scalar(1778, if s.b[1778] { 1.0 } else { 0.0 });
        if (((!s.b[1733]) && s.b[1751]) && s.b[1778]) {s.store_scalar(1341, 0.0);}
        if (((!s.b[1733]) && s.b[1751]) && (!s.b[1778])) {s.store_sub_from_scalar(1186, 1.0, 1229);s.store_mul_ad_product_rhs_mixed_ia(1341, 1317, 1186, A::sub_scaled_inputs(s.ad_value(1212), 0.5, A::div_scaled_product(s.ad_value(1179), s.ad_value(1212), 1.0, s.ad_value(1181), 1.0), 1.0));}
        s.b[1779] = (((s.v[57] != 2.0) && (s.v[760] != 0.0)) && (s.v[63] > 0.0));s.store_scalar(1779, if s.b[1779] { 1.0 } else { 0.0 });
        if ((((!s.b[1733]) && s.b[1751]) && (!s.b[1778])) && s.b[1779]) {s.store_mul_ad_product_rhs_mixed_ia(1471, 1468, 1186, A::sub_scaled_inputs(s.ad_value(1463), 0.5, A::div_scaled_product(s.ad_value(1454), s.ad_value(1463), 1.0, s.ad_value(1455), 1.0), 1.0));s.store_add(1341, 1341, 1471);}
        s.b[1780] = (s.v[153] > 0.5);s.store_scalar(1780, if s.b[1780] { 1.0 } else { 0.0 });
        if (((!s.b[1733]) && s.b[1751]) && s.b[1780]) {s.store_mul_add_scaled_inputs3_offset_rhs_mixed_iia(1254, 1338, 1180, ((0.5) * (-1.0)), 1179, ((0.25) * (-1.0)), A::div_scaled_product(s.ad_value(1179), s.ad_value(1179), 0.5, s.ad_value(1181), 1.0), ((-1.0) * (-1.0)), 0.0);}
    }
    #[inline(never)]
    pub(super) fn stamp_reactive_block_48(
        s: &mut ReactiveScratch,
        p: &Parameters,
    ) {
        s.b[1781] = (((s.v[57] != 2.0) && (s.v[760] != 0.0)) && (s.v[63] > 0.0));s.store_scalar(1781, if s.b[1781] { 1.0 } else { 0.0 });
        if ((((!s.b[1733]) && s.b[1751]) && s.b[1780]) && s.b[1781]) {s.store_mul_add_scaled_inputs4_rhs_mixed_iiia(1470, 1467, 1451, ((0.5) * (-1.0)), 1469, (((-0.5)) * (-1.0)), 1454, ((0.25) * (-1.0)), A::div_scaled_product(s.ad_value(1454), s.ad_value(1454), 0.5, s.ad_value(1455), 1.0), ((-1.0) * (-1.0)));s.store_add(1254, 1254, 1470);}
        s.b[1782] = (s.v[153] < 0.5);s.store_scalar(1782, if s.b[1782] { 1.0 } else { 0.0 });
        if ((((!s.b[1733]) && s.b[1751]) && (!s.b[1780])) && s.b[1782]) {s.store_scale(1181, 1181, 0.08333333333333333);s.store_div_scaled_inputs_square_rhs(1182, 1338, 0.5, 1181, 1.0);s.store_add_scaled_product_mixed_aia(1183, A::mul3_scaled_output(s.ad_value(1179), s.ad_value(1179), s.ad_value(1179), (2.0 * 0.06666666666666667)), (-1.0), 1180, A::add_scaled_products(s.ad_value(1179), s.ad_value(1179), (2.0 * 0.3333333333333333), s.ad_value(1180), A::sub_scaled_inputs(s.ad_value(1180), 1.0, s.ad_value(1179), (4.0 * 0.3333333333333333)), 1.0), 1.0);s.store_mul_scale_offset_indices(1254, 1183, 1182, -1.0, 0.0);}
        s.b[1783] = (((s.v[57] != 2.0) && (s.v[760] != 0.0)) && (s.v[63] > 0.0));s.store_scalar(1783, if s.b[1783] { 1.0 } else { 0.0 });
        if (((((!s.b[1733]) && s.b[1751]) && (!s.b[1780])) && s.b[1782]) && s.b[1783]) {s.store_scale(1455, 1455, 0.08333333333333333);s.store_div_scaled_inputs_square_rhs(1182, 1467, 0.5, 1455, 1.0);s.store_add_scaled_product_mixed_aia(1183, A::mul3_scaled_output(s.ad_value(1454), s.ad_value(1454), s.ad_value(1454), (2.0 * 0.06666666666666667)), (-1.0), 1191, A::add_scaled_products(s.ad_value(1454), s.ad_value(1454), (2.0 * 0.3333333333333333), s.ad_value(1191), A::sub_scaled_inputs(s.ad_value(1191), 1.0, s.ad_value(1454), (4.0 * 0.3333333333333333)), 1.0), 1.0);s.store_mul_scale_offset_indices(1470, 1183, 1182, -1.0, 0.0);s.store_add(1254, 1254, 1470);}
        if ((((!s.b[1733]) && s.b[1751]) && (!s.b[1780])) && (!s.b[1782])) {s.store_scale(1254, 1251, (-0.5));}
        s.b[1784] = (s.v[57] == 2.0);s.store_scalar(1784, if s.b[1784] { 1.0 } else { 0.0 });
        if (((!s.b[1733]) && s.b[1751]) && s.b[1784]) {s.store_scalar(1274, 0.0);}
        if (((!s.b[1733]) && s.b[1751]) && (!s.b[1784])) {s.store_scale(1249, 626, (s.v[342] * (s.v[1248] * ((((s.v[693] / s.v[59]) * s.v[39]) * s.v[727]) + s.v[65]))));s.store_mul_sub_rhs(1274, 1249, 1237, 1160);}
        if ((!s.b[1733]) && s.b[1751]) {s.store_add_scaled_inputs4_indices(1251, 1251, 1.0, 1273, 1.0, 1272, 1.0, 1341, -1.0);s.store_add_scaled_inputs4_indices(1252, 1341, 1.0, 1273, (-1.0), 1272, -1.0, 1274, -1.0);s.copy_ad(1255, 1274);s.store_add_scaled_inputs4_indices(1253, 1251, (-1.0), 1252, (-1.0), 1255, (-1.0), 1254, (-1.0));}
        if ((!s.b[1733]) && (!s.b[1751])) {s.store_scalar(1273, 0.0);s.store_scalar(1272, 0.0);s.store_scalar(1255, 0.0);s.store_scalar(1252, 0.0);s.store_scalar(1254, 0.0);s.store_scalar(1253, 0.0);s.store_scalar(1251, 0.0);}
        s.b[1785] = (s.v[57] == 2.0);s.store_scalar(1785, if s.b[1785] { 1.0 } else { 0.0 });
        if s.b[1785] {s.store_scalar(1244, 0.0);s.store_scalar(1245, 0.0);}
        if (!s.b[1785]) {s.copy_ad(1151, 200);s.store_scalar(1315, (-s.v[344]));s.store_add_scaled_offset_product_rhs(1151, 1151, 1.0, 1315, 769, (-s.v[150]), 1.0);s.copy_ad(1152, 202);s.store_scalar(1311, ((((s.v[204] * s.v[711]) * s.v[174]) * s.v[39]) / 1e-7));s.store_scale(1314, 1311, s.v[343]);s.store_add_scaled_offset_product_rhs(1311, 1311, 1.0, 1314, 769, (-s.v[150]), 1.0);s.store_scalar(1312, ((((s.v[205] * s.v[710]) * s.v[174]) * s.v[39]) / 1e-7));s.store_scale(1313, 1312, s.v[345]);s.store_add_scaled_offset_product_rhs(1312, 1312, 1.0, 1313, 769, (-s.v[150]), 1.0);s.store_scale(1329, 1151, 0.9);}
        if (!s.b[1785]) {
            s.store_sub_from_scalar_div_mixed_ai(1147, 1.0, {
                if (s.v[1421] > s.v[1329]) {
                    s.ad_value(1329)
                } else {
                    s.ad_value(1421)
                }
            }, 1151);
        }
        s.b[1786] = (p.p173 == 0.5);s.store_scalar(1786, if s.b[1786] { 1.0 } else { 0.0 });
        if ((!s.b[1785]) && s.b[1786]) {s.store_div_from_scalar_sqrt_ad(1193, 1.0, s.ad_value(1147));}
        if ((!s.b[1785]) && (!s.b[1786])) {
            s.store_exp_scaled_input_ad(1193, {
                if (s.v[1147] > 1e-38) {
                    A::ln(s.ad_value(1147))
                } else {
                    A::neg(A::constant(87.49823353377374))
                }
            }, (-p.p173));
        }
        if (!s.b[1785]) {s.store_mul_scale_offset_mixed_ia(1182, 1151, A::mul(s.ad_value(1147), s.ad_value(1193)), -(1.0 / ((1.0 - p.p173))), (1.0) * (1.0 / ((1.0 - p.p173))));}
        s.b[1787] = (s.v[1421] > s.v[1329]);s.store_scalar(1787, if s.b[1787] { 1.0 } else { 0.0 });
        if ((!s.b[1785]) && s.b[1787]) {s.store_add_scaled_product_right_sub(1182, 1182, 1.0, 1193, 1421, 1329, 1.0);}
        if (!s.b[1785]) {s.store_add_scaled_product_indices(1245, 1322, (s.v[332] * s.v[39]), 1311, 1182, 1.0);s.copy_ad(1151, 201);}
    }
    #[inline(never)]
    pub(super) fn stamp_reactive_block_49(
        s: &mut ReactiveScratch,
        p: &Parameters,
    ) {
        if (!s.b[1785]) {s.store_scalar(1315, (-s.v[346]));s.store_add_scaled_offset_product_rhs(1151, 1151, 1.0, 1315, 769, (-s.v[150]), 1.0);s.store_scalar(1152, s.v[203]);s.store_scale(1329, 1151, 0.9);}
        if (!s.b[1785]) {
            s.store_sub_from_scalar_div_mixed_ai(1147, 1.0, {
                if (s.v[1422] > s.v[1329]) {
                    s.ad_value(1329)
                } else {
                    s.ad_value(1422)
                }
            }, 1151);
        }
        s.b[1788] = (p.p173 == 0.5);s.store_scalar(1788, if s.b[1788] { 1.0 } else { 0.0 });
        if ((!s.b[1785]) && s.b[1788]) {s.store_div_from_scalar_sqrt_ad(1193, 1.0, s.ad_value(1147));}
        if ((!s.b[1785]) && (!s.b[1788])) {
            s.store_exp_scaled_input_ad(1193, {
                if (s.v[1147] > 1e-38) {
                    A::ln(s.ad_value(1147))
                } else {
                    A::neg(A::constant(87.49823353377374))
                }
            }, (-p.p173));
        }
        if (!s.b[1785]) {s.store_mul_scale_offset_mixed_ia(1182, 1151, A::mul(s.ad_value(1147), s.ad_value(1193)), -(1.0 / ((1.0 - p.p173))), (1.0) * (1.0 / ((1.0 - p.p173))));}
        s.b[1789] = (s.v[1422] > s.v[1329]);s.store_scalar(1789, if s.b[1789] { 1.0 } else { 0.0 });
        if ((!s.b[1785]) && s.b[1789]) {s.store_add_scaled_product_right_sub(1182, 1182, 1.0, 1193, 1422, 1329, 1.0);}
        if (!s.b[1785]) {s.store_add_scaled_product_indices(1244, 1323, (s.v[332] * s.v[39]), 1312, 1182, 1.0);}
        s.store_scale(1189, 1232, (-s.v[36]));s.store_scaled_sub(1190, 1155, 1232, s.v[36]);s.b[1790] = (s.v[336] != 0.0);s.store_scalar(1790, if s.b[1790] { 1.0 } else { 0.0 });s.b[1791] = (((s.v[479] > 0.0) && (s.v[36] > 0.0)) || ((s.v[479] < 0.0) && (s.v[36] < 0.0)));s.store_scalar(1791, if s.b[1791] { 1.0 } else { 0.0 });s.b[1792] = (s.v[1189] < s.v[683]);s.store_scalar(1792, if s.b[1792] { 1.0 } else { 0.0 });
        if ((s.b[1790] && s.b[1791]) && s.b[1792]) {s.store_scaled_sub(448, 1189, 683, s.v[430]);}
        s.b[1793] = (s.v[1189] < s.v[545]);s.store_scalar(1793, if s.b[1793] { 1.0 } else { 0.0 });
        if (((s.b[1790] && s.b[1791]) && (!s.b[1792])) && s.b[1793]) {s.store_sub(1179, 1189, 683);s.store_square(1180, 1179);s.store_mul_scale_offset_mixed_ia(448, 1179, A::mul_scaled_lhs(s.ad_value(546), 1.0 / (3.0), s.ad_value(1180)), -1.0, s.v[430]);}
        s.b[1794] = (s.v[1189] < s.v[684]);s.store_scalar(1794, if s.b[1794] { 1.0 } else { 0.0 });
        if ((((s.b[1790] && s.b[1791]) && (!s.b[1792])) && (!s.b[1793])) && s.b[1794]) {s.store_sub(1179, 1189, 684);s.store_square(1180, 1179);s.store_add_ad(448, A::add_scaled_product(s.ad_value(434), 1.0, s.ad_value(432), s.ad_value(1189), 1.0), A::mul3_scaled_output(s.ad_value(547), s.ad_value(1179), s.ad_value(1180), 1.0 / (3.0)));}
        if ((((s.b[1790] && s.b[1791]) && (!s.b[1792])) && (!s.b[1793])) && (!s.b[1794])) {s.store_add_scaled_product_indices(448, 434, 1.0, 432, 1189, 1.0);}
        s.b[1795] = (s.v[1189] < s.v[684]);s.store_scalar(1795, if s.b[1795] { 1.0 } else { 0.0 });
        if ((s.b[1790] && (!s.b[1791])) && s.b[1795]) {s.store_mul_sub_rhs(448, 432, 1189, 684);}
        s.b[1796] = (s.v[1189] < s.v[545]);s.store_scalar(1796, if s.b[1796] { 1.0 } else { 0.0 });
        if (((s.b[1790] && (!s.b[1791])) && (!s.b[1795])) && s.b[1796]) {s.store_sub(1179, 1189, 684);s.store_square(1180, 1179);s.store_mul_add_scaled_product_rhs_indices(448, 1179, 432, 1.0, 546, 1180, (-1.0 / (3.0)));}
        s.b[1797] = (s.v[1189] < s.v[683]);s.store_scalar(1797, if s.b[1797] { 1.0 } else { 0.0 });
        if ((((s.b[1790] && (!s.b[1791])) && (!s.b[1795])) && (!s.b[1796])) && s.b[1797]) {s.store_sub(1179, 1189, 683);s.store_square(1180, 1179);s.store_add_scaled_inputs3_mixed_iia(448, 1189, s.v[430], 434, 1.0, A::mul3_scaled_output(s.ad_value(547), s.ad_value(1179), s.ad_value(1180), 1.0 / (3.0)), 1.0);}
        if ((((s.b[1790] && (!s.b[1791])) && (!s.b[1795])) && (!s.b[1796])) && (!s.b[1797])) {s.store_add_scaled_inputs(448, 1189, s.v[430], 434, 1.0);}
        s.b[1798] = (((s.v[479] > 0.0) && (s.v[36] > 0.0)) || ((s.v[479] < 0.0) && (s.v[36] < 0.0)));s.store_scalar(1798, if s.b[1798] { 1.0 } else { 0.0 });s.b[1799] = (s.v[1190] < s.v[683]);s.store_scalar(1799, if s.b[1799] { 1.0 } else { 0.0 });
        if ((s.b[1790] && s.b[1798]) && s.b[1799]) {s.store_scaled_sub(449, 1190, 683, s.v[431]);}
        s.b[1800] = (s.v[1190] < s.v[545]);s.store_scalar(1800, if s.b[1800] { 1.0 } else { 0.0 });
        if (((s.b[1790] && s.b[1798]) && (!s.b[1799])) && s.b[1800]) {s.store_sub(1179, 1190, 683);s.store_square(1180, 1179);s.store_mul_scale_offset_mixed_ia(449, 1179, A::mul_scaled_lhs(s.ad_value(548), 1.0 / (3.0), s.ad_value(1180)), -1.0, s.v[431]);}
        s.b[1801] = (s.v[1190] < s.v[684]);s.store_scalar(1801, if s.b[1801] { 1.0 } else { 0.0 });
        if ((((s.b[1790] && s.b[1798]) && (!s.b[1799])) && (!s.b[1800])) && s.b[1801]) {s.store_sub(1179, 1190, 684);s.store_square(1180, 1179);}
    }
    #[inline(never)]
    pub(super) fn stamp_reactive_block_50(
        s: &mut ReactiveScratch,
    ) {
        if ((((s.b[1790] && s.b[1798]) && (!s.b[1799])) && (!s.b[1800])) && s.b[1801]) {s.store_add_ad(449, A::add_scaled_product(s.ad_value(435), 1.0, s.ad_value(433), s.ad_value(1190), 1.0), A::mul3_scaled_output(s.ad_value(549), s.ad_value(1179), s.ad_value(1180), 1.0 / (3.0)));}
        if ((((s.b[1790] && s.b[1798]) && (!s.b[1799])) && (!s.b[1800])) && (!s.b[1801])) {s.store_add_scaled_product_indices(449, 435, 1.0, 433, 1190, 1.0);}
        s.b[1802] = (s.v[1190] < s.v[684]);s.store_scalar(1802, if s.b[1802] { 1.0 } else { 0.0 });
        if ((s.b[1790] && (!s.b[1798])) && s.b[1802]) {s.store_mul_sub_rhs(449, 433, 1190, 684);}
        s.b[1803] = (s.v[1190] < s.v[545]);s.store_scalar(1803, if s.b[1803] { 1.0 } else { 0.0 });
        if (((s.b[1790] && (!s.b[1798])) && (!s.b[1802])) && s.b[1803]) {s.store_sub(1179, 1190, 684);s.store_square(1180, 1179);s.store_mul_add_scaled_product_rhs_indices(449, 1179, 433, 1.0, 548, 1180, (-1.0 / (3.0)));}
        s.b[1804] = (s.v[1190] < s.v[683]);s.store_scalar(1804, if s.b[1804] { 1.0 } else { 0.0 });
        if ((((s.b[1790] && (!s.b[1798])) && (!s.b[1802])) && (!s.b[1803])) && s.b[1804]) {s.store_sub(1179, 1190, 683);s.store_square(1180, 1179);s.store_add_scaled_inputs3_mixed_iia(449, 1190, s.v[431], 435, 1.0, A::mul3_scaled_output(s.ad_value(549), s.ad_value(1179), s.ad_value(1180), 1.0 / (3.0)), 1.0);}
        if ((((s.b[1790] && (!s.b[1798])) && (!s.b[1802])) && (!s.b[1803])) && (!s.b[1804])) {s.store_add_scaled_inputs(449, 1190, s.v[431], 435, 1.0);}
        if (!s.b[1790]) {s.store_scale(448, 1189, s.v[430]);s.store_scale(449, 1190, s.v[431]);}
        s.store_add_scaled_product_indices(448, 448, 1.0, 428, 1189, 1.0);s.store_add_scaled_product_indices(449, 449, 1.0, 429, 1190, 1.0);s.b[1805] = (s.v[66] == 3.0);s.store_scalar(1805, if s.b[1805] { 1.0 } else { 0.0 });
        if s.b[1805] {s.store_offset(1179, 1354, 0.02);}
        if (!s.b[1805]) {s.store_offset(1179, 1156, 0.02);}
        s.store_sqrt_square_offset(1180, 1179, (4.0 * 0.02));s.store_scaled_sub(1181, 1179, 1180, 0.5);s.store_scale(1182, 603, s.v[710]);s.store_sqrt_sub_from_scalar_ad(1183, 1.0, A::div_scaled_inputs(s.ad_value(1181), 4.0, s.ad_value(604), 1.0));s.b[1806] = (s.v[66] == 3.0);s.store_scalar(1806, if s.b[1806] { 1.0 } else { 0.0 });
        if s.b[1806] {s.store_add_scaled_products_mixed_aiia(1230, A::add(s.ad_value(696), s.ad_value(1182)), 1354, 1.0, 1182, A::add_scaled_offset_product_rhs(s.ad_value(1181), 1.0, s.ad_value(604), s.ad_value(1183), (-1.0), 0.5), (-1.0));}
        if (!s.b[1806]) {s.store_add_scaled_products_mixed_aiia(1230, A::add(s.ad_value(696), s.ad_value(1182)), 1156, 1.0, 1182, A::add_scaled_offset_product_rhs(s.ad_value(1181), 1.0, s.ad_value(604), s.ad_value(1183), (-1.0), 0.5), (-1.0));}
        s.b[1807] = (s.v[66] == 3.0);s.store_scalar(1807, if s.b[1807] { 1.0 } else { 0.0 });
        if s.b[1807] {s.store_offset(1179, 1353, 0.02);}
        if (!s.b[1807]) {s.store_offset(1179, 1157, 0.02);}
        s.store_sqrt_square_offset(1180, 1179, (4.0 * 0.02));s.store_scaled_sub(1181, 1179, 1180, 0.5);s.store_scale(1182, 602, s.v[711]);s.store_sqrt_sub_from_scalar_ad(1183, 1.0, A::div_scaled_inputs(s.ad_value(1181), 4.0, s.ad_value(604), 1.0));s.b[1808] = (s.v[66] == 3.0);s.store_scalar(1808, if s.b[1808] { 1.0 } else { 0.0 });
        if s.b[1808] {s.store_add_scaled_products_mixed_aiia(1231, A::add(s.ad_value(695), s.ad_value(1182)), 1353, 1.0, 1182, A::add_scaled_offset_product_rhs(s.ad_value(1181), 1.0, s.ad_value(604), s.ad_value(1183), (-1.0), 0.5), (-1.0));}
        if (!s.b[1808]) {s.store_add_scaled_products_mixed_aiia(1231, A::add(s.ad_value(695), s.ad_value(1182)), 1157, 1.0, 1182, A::add_scaled_offset_product_rhs(s.ad_value(1181), 1.0, s.ad_value(604), s.ad_value(1183), (-1.0), 0.5), (-1.0));}
        s.b[1809] = (s.v[39] != 1.0);s.store_scalar(1809, if s.b[1809] { 1.0 } else { 0.0 });
        if s.b[1809] {s.store_scale(1230, 1230, s.v[39]);s.store_scale(1231, 1231, s.v[39]);}
        s.copy_ad(798, 1251);s.store_add(797, 1231, 1230);
    }
    #[inline(never)]
    pub(super) fn stamp_reactive_block_51(
        s: &mut ReactiveScratch,
        p: &Parameters,
    ) {
        s.store_add(1251, 798, 797);s.b[1823] = (p.p213 == 0.0);s.store_scalar(1823, if s.b[1823] { 1.0 } else { 0.0 });s.b[1824] = (p.p213 == 1.0);s.store_scalar(1824, if s.b[1824] { 1.0 } else { 0.0 });
        if (s.b[1824] && (!s.b[1823])) {s.store_add_scaled_inputs3_indices(1179, 439, 1.0, 440, 1.0, 441, 1.0);s.store_square(1179, 1179);s.store_div_scaled_inputs_indices(1817, 1281, 2.0, 410, 1.0);s.store_div_scaled_inputs_indices(1184, 451, 1.0, 1817, s.v[688]);s.store_square(1184, 1184);s.store_offset_scaled(1818, 1184, (((s.v[241] * s.v[688])) * (s.v[243])), s.v[243]);s.store_add_scaled_product_mixed_iia(1180, 440, 1.0, 1818, A::add(s.ad_value(439), s.ad_value(441)), 1.0);s.store_div_scaled_product_indices(1181, 1180, 1180, 1.0, 454, 1.0);}
        s.b[1861] = (s.v[759] > 0.0);s.store_scalar(1861, if s.b[1861] { 1.0 } else { 0.0 });
        if s.b[1861] {s.store_scale(446, 1253, s.v[36]);s.store_scale(447, 1254, s.v[36]);}
        if (!s.b[1861]) {s.store_scale(447, 1253, s.v[36]);s.store_scale(446, 1254, s.v[36]);}
        s.b[1863] = (p.p37 == 3.0);s.store_scalar(1863, if s.b[1863] { 1.0 } else { 0.0 });s.b[1869] = ((p.p33 == 1.0) && (p.p16 != 0.0));s.store_scalar(1869, if s.b[1869] { 1.0 } else { 0.0 });
    }
    #[inline(never)]
    pub(super) fn stamp_transient_equations_block_0(
        ctx: &GeneratedEvalContext<'_>,
        stamper: &mut GeneratedStamper<'_>,
        s: &mut Scratch,
        nodes: &[usize; Instance::NODE_COUNT],
        multiplicity: f64,
    ) {
        let nv0 = ctx.node_voltage(nodes[0]);let nv2 = ctx.node_voltage(nodes[2]);let nv7 = ctx.node_voltage(nodes[7]);let nv8 = ctx.node_voltage(nodes[8]);
        let (eq4_e1143, eq4_e1143_d_n0, eq4_e1143_d_n1, eq4_e1143_d_n2, eq4_e1143_d_n3, eq4_e1143_d_n4, eq4_e1143_d_n5, eq4_e1143_d_n6, eq4_e1143_d_n7, eq4_e1143_d_n8, eq4_e1143_d_n9, eq4_e1143_d_n10, eq4_e1143_d_n11, eq4_e1143_d_n12, eq4_e1143_d_b0, eq4_e1143_d_b1, eq4_e1143_d_b2, eq4_e1143_d_b3, eq4_e1143_d_b4, eq4_e1143_d_b5, eq4_e1143_d_b6, eq4_e1143_d_b7, eq4_e1143_d_b8,) = {
    if s.b[1860] {
        let eq4_e1141: f64 = ((nv0 - nv7) / s.v[1433]);let eq4_e1141_d_n0: f64 = ((s.v[1433] - ((nv0 - nv7) * s.dn[1433][0])) / (s.v[1433] * s.v[1433]));let eq4_e1141_d_n1: f64 = (-(((nv0 - nv7) * s.dn[1433][1]) / (s.v[1433] * s.v[1433])));let eq4_e1141_d_n2: f64 = (-(((nv0 - nv7) * s.dn[1433][2]) / (s.v[1433] * s.v[1433])));let eq4_e1141_d_n3: f64 = (-(((nv0 - nv7) * s.dn[1433][3]) / (s.v[1433] * s.v[1433])));let eq4_e1141_d_n4: f64 = (-(((nv0 - nv7) * s.dn[1433][4]) / (s.v[1433] * s.v[1433])));let eq4_e1141_d_n5: f64 = (-(((nv0 - nv7) * s.dn[1433][5]) / (s.v[1433] * s.v[1433])));let eq4_e1141_d_n6: f64 = (-(((nv0 - nv7) * s.dn[1433][6]) / (s.v[1433] * s.v[1433])));let eq4_e1141_d_n7: f64 = (((-s.v[1433]) - ((nv0 - nv7) * s.dn[1433][7])) / (s.v[1433] * s.v[1433]));let eq4_e1141_d_n8: f64 = (-(((nv0 - nv7) * s.dn[1433][8]) / (s.v[1433] * s.v[1433])));let eq4_e1141_d_n9: f64 = (-(((nv0 - nv7) * s.dn[1433][9]) / (s.v[1433] * s.v[1433])));let eq4_e1141_d_n10: f64 = (-(((nv0 - nv7) * s.dn[1433][10]) / (s.v[1433] * s.v[1433])));let eq4_e1141_d_n11: f64 = (-(((nv0 - nv7) * s.dn[1433][11]) / (s.v[1433] * s.v[1433])));let eq4_e1141_d_n12: f64 = (-(((nv0 - nv7) * s.dn[1433][12]) / (s.v[1433] * s.v[1433])));let eq4_e1141_d_b0: f64 = (-(((nv0 - nv7) * s.db[1433][0]) / (s.v[1433] * s.v[1433])));let eq4_e1141_d_b1: f64 = (-(((nv0 - nv7) * s.db[1433][1]) / (s.v[1433] * s.v[1433])));let eq4_e1141_d_b2: f64 = (-(((nv0 - nv7) * s.db[1433][2]) / (s.v[1433] * s.v[1433])));let eq4_e1141_d_b3: f64 = (-(((nv0 - nv7) * s.db[1433][3]) / (s.v[1433] * s.v[1433])));let eq4_e1141_d_b4: f64 = (-(((nv0 - nv7) * s.db[1433][4]) / (s.v[1433] * s.v[1433])));let eq4_e1141_d_b5: f64 = (-(((nv0 - nv7) * s.db[1433][5]) / (s.v[1433] * s.v[1433])));let eq4_e1141_d_b6: f64 = (-(((nv0 - nv7) * s.db[1433][6]) / (s.v[1433] * s.v[1433])));let eq4_e1141_d_b7: f64 = (-(((nv0 - nv7) * s.db[1433][7]) / (s.v[1433] * s.v[1433])));let eq4_e1141_d_b8: f64 = (-(((nv0 - nv7) * s.db[1433][8]) / (s.v[1433] * s.v[1433])));
        (eq4_e1141, eq4_e1141_d_n0, eq4_e1141_d_n1, eq4_e1141_d_n2, eq4_e1141_d_n3, eq4_e1141_d_n4, eq4_e1141_d_n5, eq4_e1141_d_n6, eq4_e1141_d_n7, eq4_e1141_d_n8, eq4_e1141_d_n9, eq4_e1141_d_n10, eq4_e1141_d_n11, eq4_e1141_d_n12, eq4_e1141_d_b0, eq4_e1141_d_b1, eq4_e1141_d_b2, eq4_e1141_d_b3, eq4_e1141_d_b4, eq4_e1141_d_b5, eq4_e1141_d_b6, eq4_e1141_d_b7, eq4_e1141_d_b8,)
    } else {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    }
};
        let eq4_value: f64 = eq4_e1143;let eq4_node_derivatives: [f64; 13] = [eq4_e1143_d_n0, eq4_e1143_d_n1, eq4_e1143_d_n2, eq4_e1143_d_n3, eq4_e1143_d_n4, eq4_e1143_d_n5, eq4_e1143_d_n6, eq4_e1143_d_n7, eq4_e1143_d_n8, eq4_e1143_d_n9, eq4_e1143_d_n10, eq4_e1143_d_n11, eq4_e1143_d_n12];let eq4_branch_derivatives: [f64; 9] = [eq4_e1143_d_b0, eq4_e1143_d_b1, eq4_e1143_d_b2, eq4_e1143_d_b3, eq4_e1143_d_b4, eq4_e1143_d_b5, eq4_e1143_d_b6, eq4_e1143_d_b7, eq4_e1143_d_b8];
        stamper.stamp_current_dense_local(
            Some(0),
            Some(7),
            multiplicity * (eq4_value),
            &eq4_node_derivatives,
            &eq4_branch_derivatives,
            multiplicity,
        );
        let (eq6_e1158, eq6_e1158_d_n0, eq6_e1158_d_n1, eq6_e1158_d_n2, eq6_e1158_d_n3, eq6_e1158_d_n4, eq6_e1158_d_n5, eq6_e1158_d_n6, eq6_e1158_d_n7, eq6_e1158_d_n8, eq6_e1158_d_n9, eq6_e1158_d_n10, eq6_e1158_d_n11, eq6_e1158_d_n12, eq6_e1158_d_b0, eq6_e1158_d_b1, eq6_e1158_d_b2, eq6_e1158_d_b3, eq6_e1158_d_b4, eq6_e1158_d_b5, eq6_e1158_d_b6, eq6_e1158_d_b7, eq6_e1158_d_b8,) = {
    if s.b[1860] {
        let eq6_e1156: f64 = ((nv2 - nv8) / s.v[1434]);let eq6_e1156_d_n0: f64 = (-(((nv2 - nv8) * s.dn[1434][0]) / (s.v[1434] * s.v[1434])));let eq6_e1156_d_n1: f64 = (-(((nv2 - nv8) * s.dn[1434][1]) / (s.v[1434] * s.v[1434])));let eq6_e1156_d_n2: f64 = ((s.v[1434] - ((nv2 - nv8) * s.dn[1434][2])) / (s.v[1434] * s.v[1434]));let eq6_e1156_d_n3: f64 = (-(((nv2 - nv8) * s.dn[1434][3]) / (s.v[1434] * s.v[1434])));let eq6_e1156_d_n4: f64 = (-(((nv2 - nv8) * s.dn[1434][4]) / (s.v[1434] * s.v[1434])));let eq6_e1156_d_n5: f64 = (-(((nv2 - nv8) * s.dn[1434][5]) / (s.v[1434] * s.v[1434])));let eq6_e1156_d_n6: f64 = (-(((nv2 - nv8) * s.dn[1434][6]) / (s.v[1434] * s.v[1434])));let eq6_e1156_d_n7: f64 = (-(((nv2 - nv8) * s.dn[1434][7]) / (s.v[1434] * s.v[1434])));let eq6_e1156_d_n8: f64 = (((-s.v[1434]) - ((nv2 - nv8) * s.dn[1434][8])) / (s.v[1434] * s.v[1434]));let eq6_e1156_d_n9: f64 = (-(((nv2 - nv8) * s.dn[1434][9]) / (s.v[1434] * s.v[1434])));let eq6_e1156_d_n10: f64 = (-(((nv2 - nv8) * s.dn[1434][10]) / (s.v[1434] * s.v[1434])));let eq6_e1156_d_n11: f64 = (-(((nv2 - nv8) * s.dn[1434][11]) / (s.v[1434] * s.v[1434])));let eq6_e1156_d_n12: f64 = (-(((nv2 - nv8) * s.dn[1434][12]) / (s.v[1434] * s.v[1434])));let eq6_e1156_d_b0: f64 = (-(((nv2 - nv8) * s.db[1434][0]) / (s.v[1434] * s.v[1434])));let eq6_e1156_d_b1: f64 = (-(((nv2 - nv8) * s.db[1434][1]) / (s.v[1434] * s.v[1434])));let eq6_e1156_d_b2: f64 = (-(((nv2 - nv8) * s.db[1434][2]) / (s.v[1434] * s.v[1434])));let eq6_e1156_d_b3: f64 = (-(((nv2 - nv8) * s.db[1434][3]) / (s.v[1434] * s.v[1434])));let eq6_e1156_d_b4: f64 = (-(((nv2 - nv8) * s.db[1434][4]) / (s.v[1434] * s.v[1434])));let eq6_e1156_d_b5: f64 = (-(((nv2 - nv8) * s.db[1434][5]) / (s.v[1434] * s.v[1434])));let eq6_e1156_d_b6: f64 = (-(((nv2 - nv8) * s.db[1434][6]) / (s.v[1434] * s.v[1434])));let eq6_e1156_d_b7: f64 = (-(((nv2 - nv8) * s.db[1434][7]) / (s.v[1434] * s.v[1434])));let eq6_e1156_d_b8: f64 = (-(((nv2 - nv8) * s.db[1434][8]) / (s.v[1434] * s.v[1434])));
        (eq6_e1156, eq6_e1156_d_n0, eq6_e1156_d_n1, eq6_e1156_d_n2, eq6_e1156_d_n3, eq6_e1156_d_n4, eq6_e1156_d_n5, eq6_e1156_d_n6, eq6_e1156_d_n7, eq6_e1156_d_n8, eq6_e1156_d_n9, eq6_e1156_d_n10, eq6_e1156_d_n11, eq6_e1156_d_n12, eq6_e1156_d_b0, eq6_e1156_d_b1, eq6_e1156_d_b2, eq6_e1156_d_b3, eq6_e1156_d_b4, eq6_e1156_d_b5, eq6_e1156_d_b6, eq6_e1156_d_b7, eq6_e1156_d_b8,)
    } else {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    }
};
        let eq6_value: f64 = eq6_e1158;let eq6_node_derivatives: [f64; 13] = [eq6_e1158_d_n0, eq6_e1158_d_n1, eq6_e1158_d_n2, eq6_e1158_d_n3, eq6_e1158_d_n4, eq6_e1158_d_n5, eq6_e1158_d_n6, eq6_e1158_d_n7, eq6_e1158_d_n8, eq6_e1158_d_n9, eq6_e1158_d_n10, eq6_e1158_d_n11, eq6_e1158_d_n12];let eq6_branch_derivatives: [f64; 9] = [eq6_e1158_d_b0, eq6_e1158_d_b1, eq6_e1158_d_b2, eq6_e1158_d_b3, eq6_e1158_d_b4, eq6_e1158_d_b5, eq6_e1158_d_b6, eq6_e1158_d_b7, eq6_e1158_d_b8];
        stamper.stamp_current_dense_local(
            Some(2),
            Some(8),
            multiplicity * (eq6_value),
            &eq6_node_derivatives,
            &eq6_branch_derivatives,
            multiplicity,
        );
        let (eq8_e1172,) = {
    if (!s.b[1860]) {
        (0.0,)
    } else {
        (0.0,)
    }
};
        let eq8_value: f64 = eq8_e1172;
        stamper.stamp_potential_const_local(
            0,
            eq8_value,
        );
        let (eq9_e1177,) = {
    if (!s.b[1860]) {
        (0.0,)
    } else {
        (0.0,)
    }
};
        let eq9_value: f64 = eq9_e1177;
        stamper.stamp_potential_const_local(
            1,
            eq9_value,
        );
    }
    #[inline(never)]
    pub(super) fn stamp_transient_equations_block_1(
        stamper: &mut GeneratedStamper<'_>,
        s: &mut Scratch,
        multiplicity: f64,
    ) {
        let (eq10_e1185, eq10_e1185_d_n0, eq10_e1185_d_n1, eq10_e1185_d_n2, eq10_e1185_d_n3, eq10_e1185_d_n4, eq10_e1185_d_n5, eq10_e1185_d_n6, eq10_e1185_d_n7, eq10_e1185_d_n8, eq10_e1185_d_n9, eq10_e1185_d_n10, eq10_e1185_d_n11, eq10_e1185_d_n12, eq10_e1185_d_b0, eq10_e1185_d_b1, eq10_e1185_d_b2, eq10_e1185_d_b3, eq10_e1185_d_b4, eq10_e1185_d_b5, eq10_e1185_d_b6, eq10_e1185_d_b7, eq10_e1185_d_b8,) = {
    if s.b[1861] {
        let eq10_e1182: f64 = (s.v[1220] + s.v[1268]);let eq10_e1182_d_n0: f64 = (s.dn[1220][0] + s.dn[1268][0]);let eq10_e1182_d_n1: f64 = (s.dn[1220][1] + s.dn[1268][1]);let eq10_e1182_d_n2: f64 = (s.dn[1220][2] + s.dn[1268][2]);let eq10_e1182_d_n3: f64 = (s.dn[1220][3] + s.dn[1268][3]);let eq10_e1182_d_n4: f64 = (s.dn[1220][4] + s.dn[1268][4]);let eq10_e1182_d_n5: f64 = (s.dn[1220][5] + s.dn[1268][5]);let eq10_e1182_d_n6: f64 = (s.dn[1220][6] + s.dn[1268][6]);let eq10_e1182_d_n7: f64 = (s.dn[1220][7] + s.dn[1268][7]);let eq10_e1182_d_n8: f64 = (s.dn[1220][8] + s.dn[1268][8]);let eq10_e1182_d_n9: f64 = (s.dn[1220][9] + s.dn[1268][9]);let eq10_e1182_d_n10: f64 = (s.dn[1220][10] + s.dn[1268][10]);let eq10_e1182_d_n11: f64 = (s.dn[1220][11] + s.dn[1268][11]);let eq10_e1182_d_n12: f64 = (s.dn[1220][12] + s.dn[1268][12]);let eq10_e1182_d_b0: f64 = (s.db[1220][0] + s.db[1268][0]);let eq10_e1182_d_b1: f64 = (s.db[1220][1] + s.db[1268][1]);let eq10_e1182_d_b2: f64 = (s.db[1220][2] + s.db[1268][2]);let eq10_e1182_d_b3: f64 = (s.db[1220][3] + s.db[1268][3]);let eq10_e1182_d_b4: f64 = (s.db[1220][4] + s.db[1268][4]);let eq10_e1182_d_b5: f64 = (s.db[1220][5] + s.db[1268][5]);let eq10_e1182_d_b6: f64 = (s.db[1220][6] + s.db[1268][6]);let eq10_e1182_d_b7: f64 = (s.db[1220][7] + s.db[1268][7]);let eq10_e1182_d_b8: f64 = (s.db[1220][8] + s.db[1268][8]);let eq10_e1183: f64 = (s.v[36] * eq10_e1182);let eq10_e1183_d_n0: f64 = (s.v[36] * eq10_e1182_d_n0);let eq10_e1183_d_n1: f64 = (s.v[36] * eq10_e1182_d_n1);let eq10_e1183_d_n2: f64 = (s.v[36] * eq10_e1182_d_n2);let eq10_e1183_d_n3: f64 = (s.v[36] * eq10_e1182_d_n3);let eq10_e1183_d_n4: f64 = (s.v[36] * eq10_e1182_d_n4);let eq10_e1183_d_n5: f64 = (s.v[36] * eq10_e1182_d_n5);let eq10_e1183_d_n6: f64 = (s.v[36] * eq10_e1182_d_n6);let eq10_e1183_d_n7: f64 = (s.v[36] * eq10_e1182_d_n7);let eq10_e1183_d_n8: f64 = (s.v[36] * eq10_e1182_d_n8);let eq10_e1183_d_n9: f64 = (s.v[36] * eq10_e1182_d_n9);let eq10_e1183_d_n10: f64 = (s.v[36] * eq10_e1182_d_n10);let eq10_e1183_d_n11: f64 = (s.v[36] * eq10_e1182_d_n11);let eq10_e1183_d_n12: f64 = (s.v[36] * eq10_e1182_d_n12);let eq10_e1183_d_b0: f64 = (s.v[36] * eq10_e1182_d_b0);let eq10_e1183_d_b1: f64 = (s.v[36] * eq10_e1182_d_b1);let eq10_e1183_d_b2: f64 = (s.v[36] * eq10_e1182_d_b2);let eq10_e1183_d_b3: f64 = (s.v[36] * eq10_e1182_d_b3);let eq10_e1183_d_b4: f64 = (s.v[36] * eq10_e1182_d_b4);let eq10_e1183_d_b5: f64 = (s.v[36] * eq10_e1182_d_b5);let eq10_e1183_d_b6: f64 = (s.v[36] * eq10_e1182_d_b6);let eq10_e1183_d_b7: f64 = (s.v[36] * eq10_e1182_d_b7);let eq10_e1183_d_b8: f64 = (s.v[36] * eq10_e1182_d_b8);
        (eq10_e1183, eq10_e1183_d_n0, eq10_e1183_d_n1, eq10_e1183_d_n2, eq10_e1183_d_n3, eq10_e1183_d_n4, eq10_e1183_d_n5, eq10_e1183_d_n6, eq10_e1183_d_n7, eq10_e1183_d_n8, eq10_e1183_d_n9, eq10_e1183_d_n10, eq10_e1183_d_n11, eq10_e1183_d_n12, eq10_e1183_d_b0, eq10_e1183_d_b1, eq10_e1183_d_b2, eq10_e1183_d_b3, eq10_e1183_d_b4, eq10_e1183_d_b5, eq10_e1183_d_b6, eq10_e1183_d_b7, eq10_e1183_d_b8,)
    } else {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    }
};
        let eq10_value: f64 = eq10_e1185;let eq10_node_derivatives: [f64; 13] = [eq10_e1185_d_n0, eq10_e1185_d_n1, eq10_e1185_d_n2, eq10_e1185_d_n3, eq10_e1185_d_n4, eq10_e1185_d_n5, eq10_e1185_d_n6, eq10_e1185_d_n7, eq10_e1185_d_n8, eq10_e1185_d_n9, eq10_e1185_d_n10, eq10_e1185_d_n11, eq10_e1185_d_n12];let eq10_branch_derivatives: [f64; 9] = [eq10_e1185_d_b0, eq10_e1185_d_b1, eq10_e1185_d_b2, eq10_e1185_d_b3, eq10_e1185_d_b4, eq10_e1185_d_b5, eq10_e1185_d_b6, eq10_e1185_d_b7, eq10_e1185_d_b8];
        stamper.stamp_current_dense_local(
            Some(7),
            Some(8),
            multiplicity * (eq10_value),
            &eq10_node_derivatives,
            &eq10_branch_derivatives,
            multiplicity,
        );
        let (eq11_e1191, eq11_e1191_d_n0, eq11_e1191_d_n1, eq11_e1191_d_n2, eq11_e1191_d_n3, eq11_e1191_d_n4, eq11_e1191_d_n5, eq11_e1191_d_n6, eq11_e1191_d_n7, eq11_e1191_d_n8, eq11_e1191_d_n9, eq11_e1191_d_n10, eq11_e1191_d_n11, eq11_e1191_d_n12, eq11_e1191_d_b0, eq11_e1191_d_b1, eq11_e1191_d_b2, eq11_e1191_d_b3, eq11_e1191_d_b4, eq11_e1191_d_b5, eq11_e1191_d_b6, eq11_e1191_d_b7, eq11_e1191_d_b8,) = {
    if s.b[1861] {
        let eq11_e1189: f64 = (s.v[36] * s.v[1243]);
        (eq11_e1189, (s.v[36] * s.dn[1243][0]), (s.v[36] * s.dn[1243][1]), (s.v[36] * s.dn[1243][2]), (s.v[36] * s.dn[1243][3]), (s.v[36] * s.dn[1243][4]), (s.v[36] * s.dn[1243][5]), (s.v[36] * s.dn[1243][6]), (s.v[36] * s.dn[1243][7]), (s.v[36] * s.dn[1243][8]), (s.v[36] * s.dn[1243][9]), (s.v[36] * s.dn[1243][10]), (s.v[36] * s.dn[1243][11]), (s.v[36] * s.dn[1243][12]), (s.v[36] * s.db[1243][0]), (s.v[36] * s.db[1243][1]), (s.v[36] * s.db[1243][2]), (s.v[36] * s.db[1243][3]), (s.v[36] * s.db[1243][4]), (s.v[36] * s.db[1243][5]), (s.v[36] * s.db[1243][6]), (s.v[36] * s.db[1243][7]), (s.v[36] * s.db[1243][8]),)
    } else {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    }
};
        let eq11_value: f64 = eq11_e1191;let eq11_node_derivatives: [f64; 13] = [eq11_e1191_d_n0, eq11_e1191_d_n1, eq11_e1191_d_n2, eq11_e1191_d_n3, eq11_e1191_d_n4, eq11_e1191_d_n5, eq11_e1191_d_n6, eq11_e1191_d_n7, eq11_e1191_d_n8, eq11_e1191_d_n9, eq11_e1191_d_n10, eq11_e1191_d_n11, eq11_e1191_d_n12];let eq11_branch_derivatives: [f64; 9] = [eq11_e1191_d_b0, eq11_e1191_d_b1, eq11_e1191_d_b2, eq11_e1191_d_b3, eq11_e1191_d_b4, eq11_e1191_d_b5, eq11_e1191_d_b6, eq11_e1191_d_b7, eq11_e1191_d_b8];
        stamper.stamp_current_dense_local(
            Some(7),
            Some(5),
            multiplicity * (eq11_value),
            &eq11_node_derivatives,
            &eq11_branch_derivatives,
            multiplicity,
        );
    }
    #[inline(never)]
    pub(super) fn stamp_transient_equations_block_2(
        stamper: &mut GeneratedStamper<'_>,
        s: &mut Scratch,
        multiplicity: f64,
    ) {
        let (eq12_e1200, eq12_e1200_d_n0, eq12_e1200_d_n1, eq12_e1200_d_n2, eq12_e1200_d_n3, eq12_e1200_d_n4, eq12_e1200_d_n5, eq12_e1200_d_n6, eq12_e1200_d_n7, eq12_e1200_d_n8, eq12_e1200_d_n9, eq12_e1200_d_n10, eq12_e1200_d_n11, eq12_e1200_d_n12, eq12_e1200_d_b0, eq12_e1200_d_b1, eq12_e1200_d_b2, eq12_e1200_d_b3, eq12_e1200_d_b4, eq12_e1200_d_b5, eq12_e1200_d_b6, eq12_e1200_d_b7, eq12_e1200_d_b8,) = {
    if (!s.b[1861]) {
        let eq12_e1197: f64 = (s.v[1220] - s.v[1268]);let eq12_e1197_d_n0: f64 = (s.dn[1220][0] - s.dn[1268][0]);let eq12_e1197_d_n1: f64 = (s.dn[1220][1] - s.dn[1268][1]);let eq12_e1197_d_n2: f64 = (s.dn[1220][2] - s.dn[1268][2]);let eq12_e1197_d_n3: f64 = (s.dn[1220][3] - s.dn[1268][3]);let eq12_e1197_d_n4: f64 = (s.dn[1220][4] - s.dn[1268][4]);let eq12_e1197_d_n5: f64 = (s.dn[1220][5] - s.dn[1268][5]);let eq12_e1197_d_n6: f64 = (s.dn[1220][6] - s.dn[1268][6]);let eq12_e1197_d_n7: f64 = (s.dn[1220][7] - s.dn[1268][7]);let eq12_e1197_d_n8: f64 = (s.dn[1220][8] - s.dn[1268][8]);let eq12_e1197_d_n9: f64 = (s.dn[1220][9] - s.dn[1268][9]);let eq12_e1197_d_n10: f64 = (s.dn[1220][10] - s.dn[1268][10]);let eq12_e1197_d_n11: f64 = (s.dn[1220][11] - s.dn[1268][11]);let eq12_e1197_d_n12: f64 = (s.dn[1220][12] - s.dn[1268][12]);let eq12_e1197_d_b0: f64 = (s.db[1220][0] - s.db[1268][0]);let eq12_e1197_d_b1: f64 = (s.db[1220][1] - s.db[1268][1]);let eq12_e1197_d_b2: f64 = (s.db[1220][2] - s.db[1268][2]);let eq12_e1197_d_b3: f64 = (s.db[1220][3] - s.db[1268][3]);let eq12_e1197_d_b4: f64 = (s.db[1220][4] - s.db[1268][4]);let eq12_e1197_d_b5: f64 = (s.db[1220][5] - s.db[1268][5]);let eq12_e1197_d_b6: f64 = (s.db[1220][6] - s.db[1268][6]);let eq12_e1197_d_b7: f64 = (s.db[1220][7] - s.db[1268][7]);let eq12_e1197_d_b8: f64 = (s.db[1220][8] - s.db[1268][8]);let eq12_e1198: f64 = (s.v[36] * eq12_e1197);let eq12_e1198_d_n0: f64 = (s.v[36] * eq12_e1197_d_n0);let eq12_e1198_d_n1: f64 = (s.v[36] * eq12_e1197_d_n1);let eq12_e1198_d_n2: f64 = (s.v[36] * eq12_e1197_d_n2);let eq12_e1198_d_n3: f64 = (s.v[36] * eq12_e1197_d_n3);let eq12_e1198_d_n4: f64 = (s.v[36] * eq12_e1197_d_n4);let eq12_e1198_d_n5: f64 = (s.v[36] * eq12_e1197_d_n5);let eq12_e1198_d_n6: f64 = (s.v[36] * eq12_e1197_d_n6);let eq12_e1198_d_n7: f64 = (s.v[36] * eq12_e1197_d_n7);let eq12_e1198_d_n8: f64 = (s.v[36] * eq12_e1197_d_n8);let eq12_e1198_d_n9: f64 = (s.v[36] * eq12_e1197_d_n9);let eq12_e1198_d_n10: f64 = (s.v[36] * eq12_e1197_d_n10);let eq12_e1198_d_n11: f64 = (s.v[36] * eq12_e1197_d_n11);let eq12_e1198_d_n12: f64 = (s.v[36] * eq12_e1197_d_n12);let eq12_e1198_d_b0: f64 = (s.v[36] * eq12_e1197_d_b0);let eq12_e1198_d_b1: f64 = (s.v[36] * eq12_e1197_d_b1);let eq12_e1198_d_b2: f64 = (s.v[36] * eq12_e1197_d_b2);let eq12_e1198_d_b3: f64 = (s.v[36] * eq12_e1197_d_b3);let eq12_e1198_d_b4: f64 = (s.v[36] * eq12_e1197_d_b4);let eq12_e1198_d_b5: f64 = (s.v[36] * eq12_e1197_d_b5);let eq12_e1198_d_b6: f64 = (s.v[36] * eq12_e1197_d_b6);let eq12_e1198_d_b7: f64 = (s.v[36] * eq12_e1197_d_b7);let eq12_e1198_d_b8: f64 = (s.v[36] * eq12_e1197_d_b8);
        (eq12_e1198, eq12_e1198_d_n0, eq12_e1198_d_n1, eq12_e1198_d_n2, eq12_e1198_d_n3, eq12_e1198_d_n4, eq12_e1198_d_n5, eq12_e1198_d_n6, eq12_e1198_d_n7, eq12_e1198_d_n8, eq12_e1198_d_n9, eq12_e1198_d_n10, eq12_e1198_d_n11, eq12_e1198_d_n12, eq12_e1198_d_b0, eq12_e1198_d_b1, eq12_e1198_d_b2, eq12_e1198_d_b3, eq12_e1198_d_b4, eq12_e1198_d_b5, eq12_e1198_d_b6, eq12_e1198_d_b7, eq12_e1198_d_b8,)
    } else {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    }
};
        let eq12_value: f64 = eq12_e1200;let eq12_node_derivatives: [f64; 13] = [eq12_e1200_d_n0, eq12_e1200_d_n1, eq12_e1200_d_n2, eq12_e1200_d_n3, eq12_e1200_d_n4, eq12_e1200_d_n5, eq12_e1200_d_n6, eq12_e1200_d_n7, eq12_e1200_d_n8, eq12_e1200_d_n9, eq12_e1200_d_n10, eq12_e1200_d_n11, eq12_e1200_d_n12];let eq12_branch_derivatives: [f64; 9] = [eq12_e1200_d_b0, eq12_e1200_d_b1, eq12_e1200_d_b2, eq12_e1200_d_b3, eq12_e1200_d_b4, eq12_e1200_d_b5, eq12_e1200_d_b6, eq12_e1200_d_b7, eq12_e1200_d_b8];
        stamper.stamp_current_dense_local(
            Some(8),
            Some(7),
            multiplicity * (eq12_value),
            &eq12_node_derivatives,
            &eq12_branch_derivatives,
            multiplicity,
        );
        let (eq13_e1207, eq13_e1207_d_n0, eq13_e1207_d_n1, eq13_e1207_d_n2, eq13_e1207_d_n3, eq13_e1207_d_n4, eq13_e1207_d_n5, eq13_e1207_d_n6, eq13_e1207_d_n7, eq13_e1207_d_n8, eq13_e1207_d_n9, eq13_e1207_d_n10, eq13_e1207_d_n11, eq13_e1207_d_n12, eq13_e1207_d_b0, eq13_e1207_d_b1, eq13_e1207_d_b2, eq13_e1207_d_b3, eq13_e1207_d_b4, eq13_e1207_d_b5, eq13_e1207_d_b6, eq13_e1207_d_b7, eq13_e1207_d_b8,) = {
    if (!s.b[1861]) {
        let eq13_e1205: f64 = (s.v[36] * s.v[1243]);
        (eq13_e1205, (s.v[36] * s.dn[1243][0]), (s.v[36] * s.dn[1243][1]), (s.v[36] * s.dn[1243][2]), (s.v[36] * s.dn[1243][3]), (s.v[36] * s.dn[1243][4]), (s.v[36] * s.dn[1243][5]), (s.v[36] * s.dn[1243][6]), (s.v[36] * s.dn[1243][7]), (s.v[36] * s.dn[1243][8]), (s.v[36] * s.dn[1243][9]), (s.v[36] * s.dn[1243][10]), (s.v[36] * s.dn[1243][11]), (s.v[36] * s.dn[1243][12]), (s.v[36] * s.db[1243][0]), (s.v[36] * s.db[1243][1]), (s.v[36] * s.db[1243][2]), (s.v[36] * s.db[1243][3]), (s.v[36] * s.db[1243][4]), (s.v[36] * s.db[1243][5]), (s.v[36] * s.db[1243][6]), (s.v[36] * s.db[1243][7]), (s.v[36] * s.db[1243][8]),)
    } else {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    }
};
        let eq13_value: f64 = eq13_e1207;let eq13_node_derivatives: [f64; 13] = [eq13_e1207_d_n0, eq13_e1207_d_n1, eq13_e1207_d_n2, eq13_e1207_d_n3, eq13_e1207_d_n4, eq13_e1207_d_n5, eq13_e1207_d_n6, eq13_e1207_d_n7, eq13_e1207_d_n8, eq13_e1207_d_n9, eq13_e1207_d_n10, eq13_e1207_d_n11, eq13_e1207_d_n12];let eq13_branch_derivatives: [f64; 9] = [eq13_e1207_d_b0, eq13_e1207_d_b1, eq13_e1207_d_b2, eq13_e1207_d_b3, eq13_e1207_d_b4, eq13_e1207_d_b5, eq13_e1207_d_b6, eq13_e1207_d_b7, eq13_e1207_d_b8];
        stamper.stamp_current_dense_local(
            Some(8),
            Some(5),
            multiplicity * (eq13_value),
            &eq13_node_derivatives,
            &eq13_branch_derivatives,
            multiplicity,
        );let eq14_value: f64 = s.v[419];
        stamper.stamp_current_dense_local(
            Some(7),
            Some(5),
            multiplicity * (eq14_value),
            &s.dn[419],
            &s.db[419],
            multiplicity,
        );let eq15_value: f64 = s.v[420];
        stamper.stamp_current_dense_local(
            Some(8),
            Some(5),
            multiplicity * (eq15_value),
            &s.dn[420],
            &s.db[420],
            multiplicity,
        );let eq16_e1212: f64 = (s.v[36] * s.v[1270]);let eq16_value: f64 = eq16_e1212;
        stamper.stamp_current_dense_local(
            Some(12),
            Some(7),
            multiplicity * (eq16_value),
            &s.dn[1270],
            &s.db[1270],
            (multiplicity) * (s.v[36]),
        );let eq17_e1215: f64 = (s.v[36] * s.v[1269]);let eq17_value: f64 = eq17_e1215;
        stamper.stamp_current_dense_local(
            Some(11),
            Some(8),
            multiplicity * (eq17_value),
            &s.dn[1269],
            &s.db[1269],
            (multiplicity) * (s.v[36]),
        );
    }
    #[inline(never)]
    pub(super) fn stamp_transient_equations_block_3(
        stamper: &mut GeneratedStamper<'_>,
        s: &mut Scratch,
        multiplicity: f64,
        ddt_active: bool,
        ddt_scale: f64,
        ddt_previous_value_scale: f64,
        ddt_older_value_scale: f64,
        ddt_previous_derivative_scale: f64,
        ddt_state_current: &mut [f64; Instance::DDT_STATE_COUNT],
        ddt_state_previous: &mut [f64; Instance::DDT_STATE_COUNT],
        ddt_state_older: &mut [f64; Instance::DDT_STATE_COUNT],
        ddt_state_initialized: &mut [bool; Instance::DDT_STATE_COUNT],
        ddt_derivative_current: &mut [f64; Instance::DDT_STATE_COUNT],
        ddt_derivative_previous: &mut [f64; Instance::DDT_STATE_COUNT],
    ) {
        let eq18_e1218: f64 = (s.v[445] + s.v[443]);let eq18_e1218_d_n0: f64 = (s.dn[445][0] + s.dn[443][0]);let eq18_e1218_d_n1: f64 = (s.dn[445][1] + s.dn[443][1]);let eq18_e1218_d_n2: f64 = (s.dn[445][2] + s.dn[443][2]);let eq18_e1218_d_n3: f64 = (s.dn[445][3] + s.dn[443][3]);let eq18_e1218_d_n4: f64 = (s.dn[445][4] + s.dn[443][4]);let eq18_e1218_d_n5: f64 = (s.dn[445][5] + s.dn[443][5]);let eq18_e1218_d_n6: f64 = (s.dn[445][6] + s.dn[443][6]);let eq18_e1218_d_n7: f64 = (s.dn[445][7] + s.dn[443][7]);let eq18_e1218_d_n8: f64 = (s.dn[445][8] + s.dn[443][8]);let eq18_e1218_d_n9: f64 = (s.dn[445][9] + s.dn[443][9]);let eq18_e1218_d_n10: f64 = (s.dn[445][10] + s.dn[443][10]);let eq18_e1218_d_n11: f64 = (s.dn[445][11] + s.dn[443][11]);let eq18_e1218_d_n12: f64 = (s.dn[445][12] + s.dn[443][12]);let eq18_e1218_d_b0: f64 = (s.db[445][0] + s.db[443][0]);let eq18_e1218_d_b1: f64 = (s.db[445][1] + s.db[443][1]);let eq18_e1218_d_b2: f64 = (s.db[445][2] + s.db[443][2]);let eq18_e1218_d_b3: f64 = (s.db[445][3] + s.db[443][3]);let eq18_e1218_d_b4: f64 = (s.db[445][4] + s.db[443][4]);let eq18_e1218_d_b5: f64 = (s.db[445][5] + s.db[443][5]);let eq18_e1218_d_b6: f64 = (s.db[445][6] + s.db[443][6]);let eq18_e1218_d_b7: f64 = (s.db[445][7] + s.db[443][7]);let eq18_e1218_d_b8: f64 = (s.db[445][8] + s.db[443][8]);let eq18_value: f64 = eq18_e1218;let eq18_node_derivatives: [f64; 13] = [eq18_e1218_d_n0, eq18_e1218_d_n1, eq18_e1218_d_n2, eq18_e1218_d_n3, eq18_e1218_d_n4, eq18_e1218_d_n5, eq18_e1218_d_n6, eq18_e1218_d_n7, eq18_e1218_d_n8, eq18_e1218_d_n9, eq18_e1218_d_n10, eq18_e1218_d_n11, eq18_e1218_d_n12];let eq18_branch_derivatives: [f64; 9] = [eq18_e1218_d_b0, eq18_e1218_d_b1, eq18_e1218_d_b2, eq18_e1218_d_b3, eq18_e1218_d_b4, eq18_e1218_d_b5, eq18_e1218_d_b6, eq18_e1218_d_b7, eq18_e1218_d_b8];
        stamper.stamp_current_dense_local(
            Some(9),
            Some(7),
            multiplicity * (eq18_value),
            &eq18_node_derivatives,
            &eq18_branch_derivatives,
            multiplicity,
        );let eq19_e1221: f64 = (s.v[444] + s.v[442]);let eq19_e1221_d_n0: f64 = (s.dn[444][0] + s.dn[442][0]);let eq19_e1221_d_n1: f64 = (s.dn[444][1] + s.dn[442][1]);let eq19_e1221_d_n2: f64 = (s.dn[444][2] + s.dn[442][2]);let eq19_e1221_d_n3: f64 = (s.dn[444][3] + s.dn[442][3]);let eq19_e1221_d_n4: f64 = (s.dn[444][4] + s.dn[442][4]);let eq19_e1221_d_n5: f64 = (s.dn[444][5] + s.dn[442][5]);let eq19_e1221_d_n6: f64 = (s.dn[444][6] + s.dn[442][6]);let eq19_e1221_d_n7: f64 = (s.dn[444][7] + s.dn[442][7]);let eq19_e1221_d_n8: f64 = (s.dn[444][8] + s.dn[442][8]);let eq19_e1221_d_n9: f64 = (s.dn[444][9] + s.dn[442][9]);let eq19_e1221_d_n10: f64 = (s.dn[444][10] + s.dn[442][10]);let eq19_e1221_d_n11: f64 = (s.dn[444][11] + s.dn[442][11]);let eq19_e1221_d_n12: f64 = (s.dn[444][12] + s.dn[442][12]);let eq19_e1221_d_b0: f64 = (s.db[444][0] + s.db[442][0]);let eq19_e1221_d_b1: f64 = (s.db[444][1] + s.db[442][1]);let eq19_e1221_d_b2: f64 = (s.db[444][2] + s.db[442][2]);let eq19_e1221_d_b3: f64 = (s.db[444][3] + s.db[442][3]);let eq19_e1221_d_b4: f64 = (s.db[444][4] + s.db[442][4]);let eq19_e1221_d_b5: f64 = (s.db[444][5] + s.db[442][5]);let eq19_e1221_d_b6: f64 = (s.db[444][6] + s.db[442][6]);let eq19_e1221_d_b7: f64 = (s.db[444][7] + s.db[442][7]);let eq19_e1221_d_b8: f64 = (s.db[444][8] + s.db[442][8]);let eq19_value: f64 = eq19_e1221;let eq19_node_derivatives: [f64; 13] = [eq19_e1221_d_n0, eq19_e1221_d_n1, eq19_e1221_d_n2, eq19_e1221_d_n3, eq19_e1221_d_n4, eq19_e1221_d_n5, eq19_e1221_d_n6, eq19_e1221_d_n7, eq19_e1221_d_n8, eq19_e1221_d_n9, eq19_e1221_d_n10, eq19_e1221_d_n11, eq19_e1221_d_n12];let eq19_branch_derivatives: [f64; 9] = [eq19_e1221_d_b0, eq19_e1221_d_b1, eq19_e1221_d_b2, eq19_e1221_d_b3, eq19_e1221_d_b4, eq19_e1221_d_b5, eq19_e1221_d_b6, eq19_e1221_d_b7, eq19_e1221_d_b8];
        stamper.stamp_current_dense_local(
            Some(9),
            Some(8),
            multiplicity * (eq19_value),
            &eq19_node_derivatives,
            &eq19_branch_derivatives,
            multiplicity,
        );let eq20_value: f64 = s.v[412];
        stamper.stamp_current_dense_local(
            Some(9),
            Some(5),
            multiplicity * (eq20_value),
            &s.dn[412],
            &s.db[412],
            multiplicity,
        );let eq21_value: f64 = s.v[417];
        stamper.stamp_current_dense_local(
            Some(9),
            Some(4),
            multiplicity * (eq21_value),
            &s.dn[417],
            &s.db[417],
            multiplicity,
        );
        let (eq22_e1227,) = {
    if s.b[1862] {
        (0.0,)
    } else {
        (0.0,)
    }
};
        let eq22_value: f64 = eq22_e1227;
        stamper.stamp_potential_const_local(
            2,
            eq22_value,
        );
        let (eq23_e1234, eq23_e1234_d_n0, eq23_e1234_d_n1, eq23_e1234_d_n2, eq23_e1234_d_n3, eq23_e1234_d_n4, eq23_e1234_d_n5, eq23_e1234_d_n6, eq23_e1234_d_n7, eq23_e1234_d_n8, eq23_e1234_d_n9, eq23_e1234_d_n10, eq23_e1234_d_n11, eq23_e1234_d_n12, eq23_e1234_d_b0, eq23_e1234_d_b1, eq23_e1234_d_b2, eq23_e1234_d_b3, eq23_e1234_d_b4, eq23_e1234_d_b5, eq23_e1234_d_b6, eq23_e1234_d_b7, eq23_e1234_d_b8,) = {
    if (!s.b[1862]) {
        let eq23_e1232: f64 = (s.v[36] * s.v[1242]);
        (eq23_e1232, (s.v[36] * s.dn[1242][0]), (s.v[36] * s.dn[1242][1]), (s.v[36] * s.dn[1242][2]), (s.v[36] * s.dn[1242][3]), (s.v[36] * s.dn[1242][4]), (s.v[36] * s.dn[1242][5]), (s.v[36] * s.dn[1242][6]), (s.v[36] * s.dn[1242][7]), (s.v[36] * s.dn[1242][8]), (s.v[36] * s.dn[1242][9]), (s.v[36] * s.dn[1242][10]), (s.v[36] * s.dn[1242][11]), (s.v[36] * s.dn[1242][12]), (s.v[36] * s.db[1242][0]), (s.v[36] * s.db[1242][1]), (s.v[36] * s.db[1242][2]), (s.v[36] * s.db[1242][3]), (s.v[36] * s.db[1242][4]), (s.v[36] * s.db[1242][5]), (s.v[36] * s.db[1242][6]), (s.v[36] * s.db[1242][7]), (s.v[36] * s.db[1242][8]),)
    } else {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    }
};
        let eq23_value: f64 = eq23_e1234;let eq23_node_derivatives: [f64; 13] = [eq23_e1234_d_n0, eq23_e1234_d_n1, eq23_e1234_d_n2, eq23_e1234_d_n3, eq23_e1234_d_n4, eq23_e1234_d_n5, eq23_e1234_d_n6, eq23_e1234_d_n7, eq23_e1234_d_n8, eq23_e1234_d_n9, eq23_e1234_d_n10, eq23_e1234_d_n11, eq23_e1234_d_n12];let eq23_branch_derivatives: [f64; 9] = [eq23_e1234_d_b0, eq23_e1234_d_b1, eq23_e1234_d_b2, eq23_e1234_d_b3, eq23_e1234_d_b4, eq23_e1234_d_b5, eq23_e1234_d_b6, eq23_e1234_d_b7, eq23_e1234_d_b8];
        stamper.stamp_current_dense_local(
            Some(5),
            Some(4),
            multiplicity * (eq23_value),
            &eq23_node_derivatives,
            &eq23_branch_derivatives,
            multiplicity,
        );let eq30_e1299: f64 = eval_ddt(ddt_state_current, ddt_state_previous, ddt_state_older, ddt_state_initialized, ddt_derivative_current, ddt_derivative_previous, ddt_active, ddt_scale, ddt_previous_value_scale, ddt_older_value_scale, ddt_previous_derivative_scale, 0, s.v[446]);let eq30_value: f64 = eq30_e1299;
        stamper.stamp_current_dense_local(
            Some(7),
            Some(5),
            multiplicity * (eq30_value),
            &s.dn[446],
            &s.db[446],
            (multiplicity) * (ddt_scale),
        );let eq31_e1301: f64 = eval_ddt(ddt_state_current, ddt_state_previous, ddt_state_older, ddt_state_initialized, ddt_derivative_current, ddt_derivative_previous, ddt_active, ddt_scale, ddt_previous_value_scale, ddt_older_value_scale, ddt_previous_derivative_scale, 1, s.v[447]);let eq31_value: f64 = eq31_e1301;
        stamper.stamp_current_dense_local(
            Some(8),
            Some(5),
            multiplicity * (eq31_value),
            &s.dn[447],
            &s.db[447],
            (multiplicity) * (ddt_scale),
        );
    }
    #[inline(never)]
    pub(super) fn stamp_transient_equations_block_4(
        stamper: &mut GeneratedStamper<'_>,
        s: &mut Scratch,
        multiplicity: f64,
        ddt_active: bool,
        ddt_scale: f64,
        ddt_previous_value_scale: f64,
        ddt_older_value_scale: f64,
        ddt_previous_derivative_scale: f64,
        ddt_state_current: &mut [f64; Instance::DDT_STATE_COUNT],
        ddt_state_previous: &mut [f64; Instance::DDT_STATE_COUNT],
        ddt_state_older: &mut [f64; Instance::DDT_STATE_COUNT],
        ddt_state_initialized: &mut [bool; Instance::DDT_STATE_COUNT],
        ddt_derivative_current: &mut [f64; Instance::DDT_STATE_COUNT],
        ddt_derivative_previous: &mut [f64; Instance::DDT_STATE_COUNT],
    ) {
        let eq32_e1304: f64 = eval_ddt(ddt_state_current, ddt_state_previous, ddt_state_older, ddt_state_initialized, ddt_derivative_current, ddt_derivative_previous, ddt_active, ddt_scale, ddt_previous_value_scale, ddt_older_value_scale, ddt_previous_derivative_scale, 2, s.v[1251]);let eq32_e1305: f64 = (s.v[36] * eq32_e1304);let eq32_e1305_d_n0: f64 = (s.v[36] * (s.dn[1251][0] * ddt_scale));let eq32_e1305_d_n1: f64 = (s.v[36] * (s.dn[1251][1] * ddt_scale));let eq32_e1305_d_n2: f64 = (s.v[36] * (s.dn[1251][2] * ddt_scale));let eq32_e1305_d_n3: f64 = (s.v[36] * (s.dn[1251][3] * ddt_scale));let eq32_e1305_d_n4: f64 = (s.v[36] * (s.dn[1251][4] * ddt_scale));let eq32_e1305_d_n5: f64 = (s.v[36] * (s.dn[1251][5] * ddt_scale));let eq32_e1305_d_n6: f64 = (s.v[36] * (s.dn[1251][6] * ddt_scale));let eq32_e1305_d_n7: f64 = (s.v[36] * (s.dn[1251][7] * ddt_scale));let eq32_e1305_d_n8: f64 = (s.v[36] * (s.dn[1251][8] * ddt_scale));let eq32_e1305_d_n9: f64 = (s.v[36] * (s.dn[1251][9] * ddt_scale));let eq32_e1305_d_n10: f64 = (s.v[36] * (s.dn[1251][10] * ddt_scale));let eq32_e1305_d_n11: f64 = (s.v[36] * (s.dn[1251][11] * ddt_scale));let eq32_e1305_d_n12: f64 = (s.v[36] * (s.dn[1251][12] * ddt_scale));let eq32_e1305_d_b0: f64 = (s.v[36] * (s.db[1251][0] * ddt_scale));let eq32_e1305_d_b1: f64 = (s.v[36] * (s.db[1251][1] * ddt_scale));let eq32_e1305_d_b2: f64 = (s.v[36] * (s.db[1251][2] * ddt_scale));let eq32_e1305_d_b3: f64 = (s.v[36] * (s.db[1251][3] * ddt_scale));let eq32_e1305_d_b4: f64 = (s.v[36] * (s.db[1251][4] * ddt_scale));let eq32_e1305_d_b5: f64 = (s.v[36] * (s.db[1251][5] * ddt_scale));let eq32_e1305_d_b6: f64 = (s.v[36] * (s.db[1251][6] * ddt_scale));let eq32_e1305_d_b7: f64 = (s.v[36] * (s.db[1251][7] * ddt_scale));let eq32_e1305_d_b8: f64 = (s.v[36] * (s.db[1251][8] * ddt_scale));let eq32_value: f64 = eq32_e1305;let eq32_node_derivatives: [f64; 13] = [eq32_e1305_d_n0, eq32_e1305_d_n1, eq32_e1305_d_n2, eq32_e1305_d_n3, eq32_e1305_d_n4, eq32_e1305_d_n5, eq32_e1305_d_n6, eq32_e1305_d_n7, eq32_e1305_d_n8, eq32_e1305_d_n9, eq32_e1305_d_n10, eq32_e1305_d_n11, eq32_e1305_d_n12];let eq32_branch_derivatives: [f64; 9] = [eq32_e1305_d_b0, eq32_e1305_d_b1, eq32_e1305_d_b2, eq32_e1305_d_b3, eq32_e1305_d_b4, eq32_e1305_d_b5, eq32_e1305_d_b6, eq32_e1305_d_b7, eq32_e1305_d_b8];
        stamper.stamp_current_dense_local(
            Some(9),
            Some(5),
            multiplicity * (eq32_value),
            &eq32_node_derivatives,
            &eq32_branch_derivatives,
            multiplicity,
        );let eq33_e1308: f64 = eval_ddt(ddt_state_current, ddt_state_previous, ddt_state_older, ddt_state_initialized, ddt_derivative_current, ddt_derivative_previous, ddt_active, ddt_scale, ddt_previous_value_scale, ddt_older_value_scale, ddt_previous_derivative_scale, 3, s.v[1255]);let eq33_e1309: f64 = (s.v[36] * eq33_e1308);let eq33_e1309_d_n0: f64 = (s.v[36] * (s.dn[1255][0] * ddt_scale));let eq33_e1309_d_n1: f64 = (s.v[36] * (s.dn[1255][1] * ddt_scale));let eq33_e1309_d_n2: f64 = (s.v[36] * (s.dn[1255][2] * ddt_scale));let eq33_e1309_d_n3: f64 = (s.v[36] * (s.dn[1255][3] * ddt_scale));let eq33_e1309_d_n4: f64 = (s.v[36] * (s.dn[1255][4] * ddt_scale));let eq33_e1309_d_n5: f64 = (s.v[36] * (s.dn[1255][5] * ddt_scale));let eq33_e1309_d_n6: f64 = (s.v[36] * (s.dn[1255][6] * ddt_scale));let eq33_e1309_d_n7: f64 = (s.v[36] * (s.dn[1255][7] * ddt_scale));let eq33_e1309_d_n8: f64 = (s.v[36] * (s.dn[1255][8] * ddt_scale));let eq33_e1309_d_n9: f64 = (s.v[36] * (s.dn[1255][9] * ddt_scale));let eq33_e1309_d_n10: f64 = (s.v[36] * (s.dn[1255][10] * ddt_scale));let eq33_e1309_d_n11: f64 = (s.v[36] * (s.dn[1255][11] * ddt_scale));let eq33_e1309_d_n12: f64 = (s.v[36] * (s.dn[1255][12] * ddt_scale));let eq33_e1309_d_b0: f64 = (s.v[36] * (s.db[1255][0] * ddt_scale));let eq33_e1309_d_b1: f64 = (s.v[36] * (s.db[1255][1] * ddt_scale));let eq33_e1309_d_b2: f64 = (s.v[36] * (s.db[1255][2] * ddt_scale));let eq33_e1309_d_b3: f64 = (s.v[36] * (s.db[1255][3] * ddt_scale));let eq33_e1309_d_b4: f64 = (s.v[36] * (s.db[1255][4] * ddt_scale));let eq33_e1309_d_b5: f64 = (s.v[36] * (s.db[1255][5] * ddt_scale));let eq33_e1309_d_b6: f64 = (s.v[36] * (s.db[1255][6] * ddt_scale));let eq33_e1309_d_b7: f64 = (s.v[36] * (s.db[1255][7] * ddt_scale));let eq33_e1309_d_b8: f64 = (s.v[36] * (s.db[1255][8] * ddt_scale));let eq33_value: f64 = eq33_e1309;let eq33_node_derivatives: [f64; 13] = [eq33_e1309_d_n0, eq33_e1309_d_n1, eq33_e1309_d_n2, eq33_e1309_d_n3, eq33_e1309_d_n4, eq33_e1309_d_n5, eq33_e1309_d_n6, eq33_e1309_d_n7, eq33_e1309_d_n8, eq33_e1309_d_n9, eq33_e1309_d_n10, eq33_e1309_d_n11, eq33_e1309_d_n12];let eq33_branch_derivatives: [f64; 9] = [eq33_e1309_d_b0, eq33_e1309_d_b1, eq33_e1309_d_b2, eq33_e1309_d_b3, eq33_e1309_d_b4, eq33_e1309_d_b5, eq33_e1309_d_b6, eq33_e1309_d_b7, eq33_e1309_d_b8];
        stamper.stamp_current_dense_local(
            Some(3),
            Some(5),
            multiplicity * (eq33_value),
            &eq33_node_derivatives,
            &eq33_branch_derivatives,
            multiplicity,
        );let eq34_e1312: f64 = eval_ddt(ddt_state_current, ddt_state_previous, ddt_state_older, ddt_state_initialized, ddt_derivative_current, ddt_derivative_previous, ddt_active, ddt_scale, ddt_previous_value_scale, ddt_older_value_scale, ddt_previous_derivative_scale, 4, s.v[1244]);let eq34_e1313: f64 = (s.v[36] * eq34_e1312);let eq34_e1313_d_n0: f64 = (s.v[36] * (s.dn[1244][0] * ddt_scale));let eq34_e1313_d_n1: f64 = (s.v[36] * (s.dn[1244][1] * ddt_scale));let eq34_e1313_d_n2: f64 = (s.v[36] * (s.dn[1244][2] * ddt_scale));let eq34_e1313_d_n3: f64 = (s.v[36] * (s.dn[1244][3] * ddt_scale));let eq34_e1313_d_n4: f64 = (s.v[36] * (s.dn[1244][4] * ddt_scale));let eq34_e1313_d_n5: f64 = (s.v[36] * (s.dn[1244][5] * ddt_scale));let eq34_e1313_d_n6: f64 = (s.v[36] * (s.dn[1244][6] * ddt_scale));let eq34_e1313_d_n7: f64 = (s.v[36] * (s.dn[1244][7] * ddt_scale));let eq34_e1313_d_n8: f64 = (s.v[36] * (s.dn[1244][8] * ddt_scale));let eq34_e1313_d_n9: f64 = (s.v[36] * (s.dn[1244][9] * ddt_scale));let eq34_e1313_d_n10: f64 = (s.v[36] * (s.dn[1244][10] * ddt_scale));let eq34_e1313_d_n11: f64 = (s.v[36] * (s.dn[1244][11] * ddt_scale));let eq34_e1313_d_n12: f64 = (s.v[36] * (s.dn[1244][12] * ddt_scale));let eq34_e1313_d_b0: f64 = (s.v[36] * (s.db[1244][0] * ddt_scale));let eq34_e1313_d_b1: f64 = (s.v[36] * (s.db[1244][1] * ddt_scale));let eq34_e1313_d_b2: f64 = (s.v[36] * (s.db[1244][2] * ddt_scale));let eq34_e1313_d_b3: f64 = (s.v[36] * (s.db[1244][3] * ddt_scale));let eq34_e1313_d_b4: f64 = (s.v[36] * (s.db[1244][4] * ddt_scale));let eq34_e1313_d_b5: f64 = (s.v[36] * (s.db[1244][5] * ddt_scale));let eq34_e1313_d_b6: f64 = (s.v[36] * (s.db[1244][6] * ddt_scale));let eq34_e1313_d_b7: f64 = (s.v[36] * (s.db[1244][7] * ddt_scale));let eq34_e1313_d_b8: f64 = (s.v[36] * (s.db[1244][8] * ddt_scale));let eq34_value: f64 = eq34_e1313;let eq34_node_derivatives: [f64; 13] = [eq34_e1313_d_n0, eq34_e1313_d_n1, eq34_e1313_d_n2, eq34_e1313_d_n3, eq34_e1313_d_n4, eq34_e1313_d_n5, eq34_e1313_d_n6, eq34_e1313_d_n7, eq34_e1313_d_n8, eq34_e1313_d_n9, eq34_e1313_d_n10, eq34_e1313_d_n11, eq34_e1313_d_n12];let eq34_branch_derivatives: [f64; 9] = [eq34_e1313_d_b0, eq34_e1313_d_b1, eq34_e1313_d_b2, eq34_e1313_d_b3, eq34_e1313_d_b4, eq34_e1313_d_b5, eq34_e1313_d_b6, eq34_e1313_d_b7, eq34_e1313_d_b8];
        stamper.stamp_current_dense_local(
            Some(12),
            Some(7),
            multiplicity * (eq34_value),
            &eq34_node_derivatives,
            &eq34_branch_derivatives,
            multiplicity,
        );
    }
    #[inline(never)]
    pub(super) fn stamp_transient_equations_block_5(
        stamper: &mut GeneratedStamper<'_>,
        s: &mut Scratch,
        multiplicity: f64,
        ddt_active: bool,
        ddt_scale: f64,
        ddt_previous_value_scale: f64,
        ddt_older_value_scale: f64,
        ddt_previous_derivative_scale: f64,
        ddt_state_current: &mut [f64; Instance::DDT_STATE_COUNT],
        ddt_state_previous: &mut [f64; Instance::DDT_STATE_COUNT],
        ddt_state_older: &mut [f64; Instance::DDT_STATE_COUNT],
        ddt_state_initialized: &mut [bool; Instance::DDT_STATE_COUNT],
        ddt_derivative_current: &mut [f64; Instance::DDT_STATE_COUNT],
        ddt_derivative_previous: &mut [f64; Instance::DDT_STATE_COUNT],
    ) {
        let eq35_e1316: f64 = eval_ddt(ddt_state_current, ddt_state_previous, ddt_state_older, ddt_state_initialized, ddt_derivative_current, ddt_derivative_previous, ddt_active, ddt_scale, ddt_previous_value_scale, ddt_older_value_scale, ddt_previous_derivative_scale, 5, s.v[1245]);let eq35_e1317: f64 = (s.v[36] * eq35_e1316);let eq35_e1317_d_n0: f64 = (s.v[36] * (s.dn[1245][0] * ddt_scale));let eq35_e1317_d_n1: f64 = (s.v[36] * (s.dn[1245][1] * ddt_scale));let eq35_e1317_d_n2: f64 = (s.v[36] * (s.dn[1245][2] * ddt_scale));let eq35_e1317_d_n3: f64 = (s.v[36] * (s.dn[1245][3] * ddt_scale));let eq35_e1317_d_n4: f64 = (s.v[36] * (s.dn[1245][4] * ddt_scale));let eq35_e1317_d_n5: f64 = (s.v[36] * (s.dn[1245][5] * ddt_scale));let eq35_e1317_d_n6: f64 = (s.v[36] * (s.dn[1245][6] * ddt_scale));let eq35_e1317_d_n7: f64 = (s.v[36] * (s.dn[1245][7] * ddt_scale));let eq35_e1317_d_n8: f64 = (s.v[36] * (s.dn[1245][8] * ddt_scale));let eq35_e1317_d_n9: f64 = (s.v[36] * (s.dn[1245][9] * ddt_scale));let eq35_e1317_d_n10: f64 = (s.v[36] * (s.dn[1245][10] * ddt_scale));let eq35_e1317_d_n11: f64 = (s.v[36] * (s.dn[1245][11] * ddt_scale));let eq35_e1317_d_n12: f64 = (s.v[36] * (s.dn[1245][12] * ddt_scale));let eq35_e1317_d_b0: f64 = (s.v[36] * (s.db[1245][0] * ddt_scale));let eq35_e1317_d_b1: f64 = (s.v[36] * (s.db[1245][1] * ddt_scale));let eq35_e1317_d_b2: f64 = (s.v[36] * (s.db[1245][2] * ddt_scale));let eq35_e1317_d_b3: f64 = (s.v[36] * (s.db[1245][3] * ddt_scale));let eq35_e1317_d_b4: f64 = (s.v[36] * (s.db[1245][4] * ddt_scale));let eq35_e1317_d_b5: f64 = (s.v[36] * (s.db[1245][5] * ddt_scale));let eq35_e1317_d_b6: f64 = (s.v[36] * (s.db[1245][6] * ddt_scale));let eq35_e1317_d_b7: f64 = (s.v[36] * (s.db[1245][7] * ddt_scale));let eq35_e1317_d_b8: f64 = (s.v[36] * (s.db[1245][8] * ddt_scale));let eq35_value: f64 = eq35_e1317;let eq35_node_derivatives: [f64; 13] = [eq35_e1317_d_n0, eq35_e1317_d_n1, eq35_e1317_d_n2, eq35_e1317_d_n3, eq35_e1317_d_n4, eq35_e1317_d_n5, eq35_e1317_d_n6, eq35_e1317_d_n7, eq35_e1317_d_n8, eq35_e1317_d_n9, eq35_e1317_d_n10, eq35_e1317_d_n11, eq35_e1317_d_n12];let eq35_branch_derivatives: [f64; 9] = [eq35_e1317_d_b0, eq35_e1317_d_b1, eq35_e1317_d_b2, eq35_e1317_d_b3, eq35_e1317_d_b4, eq35_e1317_d_b5, eq35_e1317_d_b6, eq35_e1317_d_b7, eq35_e1317_d_b8];
        stamper.stamp_current_dense_local(
            Some(11),
            Some(8),
            multiplicity * (eq35_value),
            &eq35_node_derivatives,
            &eq35_branch_derivatives,
            multiplicity,
        );
        let (eq36_e1324, eq36_e1324_d_n0, eq36_e1324_d_n1, eq36_e1324_d_n2, eq36_e1324_d_n3, eq36_e1324_d_n4, eq36_e1324_d_n5, eq36_e1324_d_n6, eq36_e1324_d_n7, eq36_e1324_d_n8, eq36_e1324_d_n9, eq36_e1324_d_n10, eq36_e1324_d_n11, eq36_e1324_d_n12, eq36_e1324_d_b0, eq36_e1324_d_b1, eq36_e1324_d_b2, eq36_e1324_d_b3, eq36_e1324_d_b4, eq36_e1324_d_b5, eq36_e1324_d_b6, eq36_e1324_d_b7, eq36_e1324_d_b8,) = {
    if s.b[1863] {
        let eq36_e1321: f64 = eval_ddt(ddt_state_current, ddt_state_previous, ddt_state_older, ddt_state_initialized, ddt_derivative_current, ddt_derivative_previous, ddt_active, ddt_scale, ddt_previous_value_scale, ddt_older_value_scale, ddt_previous_derivative_scale, 6, s.v[1230]);let eq36_e1322: f64 = (s.v[36] * eq36_e1321);let eq36_e1322_d_n0: f64 = (s.v[36] * (s.dn[1230][0] * ddt_scale));let eq36_e1322_d_n1: f64 = (s.v[36] * (s.dn[1230][1] * ddt_scale));let eq36_e1322_d_n2: f64 = (s.v[36] * (s.dn[1230][2] * ddt_scale));let eq36_e1322_d_n3: f64 = (s.v[36] * (s.dn[1230][3] * ddt_scale));let eq36_e1322_d_n4: f64 = (s.v[36] * (s.dn[1230][4] * ddt_scale));let eq36_e1322_d_n5: f64 = (s.v[36] * (s.dn[1230][5] * ddt_scale));let eq36_e1322_d_n6: f64 = (s.v[36] * (s.dn[1230][6] * ddt_scale));let eq36_e1322_d_n7: f64 = (s.v[36] * (s.dn[1230][7] * ddt_scale));let eq36_e1322_d_n8: f64 = (s.v[36] * (s.dn[1230][8] * ddt_scale));let eq36_e1322_d_n9: f64 = (s.v[36] * (s.dn[1230][9] * ddt_scale));let eq36_e1322_d_n10: f64 = (s.v[36] * (s.dn[1230][10] * ddt_scale));let eq36_e1322_d_n11: f64 = (s.v[36] * (s.dn[1230][11] * ddt_scale));let eq36_e1322_d_n12: f64 = (s.v[36] * (s.dn[1230][12] * ddt_scale));let eq36_e1322_d_b0: f64 = (s.v[36] * (s.db[1230][0] * ddt_scale));let eq36_e1322_d_b1: f64 = (s.v[36] * (s.db[1230][1] * ddt_scale));let eq36_e1322_d_b2: f64 = (s.v[36] * (s.db[1230][2] * ddt_scale));let eq36_e1322_d_b3: f64 = (s.v[36] * (s.db[1230][3] * ddt_scale));let eq36_e1322_d_b4: f64 = (s.v[36] * (s.db[1230][4] * ddt_scale));let eq36_e1322_d_b5: f64 = (s.v[36] * (s.db[1230][5] * ddt_scale));let eq36_e1322_d_b6: f64 = (s.v[36] * (s.db[1230][6] * ddt_scale));let eq36_e1322_d_b7: f64 = (s.v[36] * (s.db[1230][7] * ddt_scale));let eq36_e1322_d_b8: f64 = (s.v[36] * (s.db[1230][8] * ddt_scale));
        (eq36_e1322, eq36_e1322_d_n0, eq36_e1322_d_n1, eq36_e1322_d_n2, eq36_e1322_d_n3, eq36_e1322_d_n4, eq36_e1322_d_n5, eq36_e1322_d_n6, eq36_e1322_d_n7, eq36_e1322_d_n8, eq36_e1322_d_n9, eq36_e1322_d_n10, eq36_e1322_d_n11, eq36_e1322_d_n12, eq36_e1322_d_b0, eq36_e1322_d_b1, eq36_e1322_d_b2, eq36_e1322_d_b3, eq36_e1322_d_b4, eq36_e1322_d_b5, eq36_e1322_d_b6, eq36_e1322_d_b7, eq36_e1322_d_b8,)
    } else {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    }
};
        let eq36_value: f64 = eq36_e1324;let eq36_node_derivatives: [f64; 13] = [eq36_e1324_d_n0, eq36_e1324_d_n1, eq36_e1324_d_n2, eq36_e1324_d_n3, eq36_e1324_d_n4, eq36_e1324_d_n5, eq36_e1324_d_n6, eq36_e1324_d_n7, eq36_e1324_d_n8, eq36_e1324_d_n9, eq36_e1324_d_n10, eq36_e1324_d_n11, eq36_e1324_d_n12];let eq36_branch_derivatives: [f64; 9] = [eq36_e1324_d_b0, eq36_e1324_d_b1, eq36_e1324_d_b2, eq36_e1324_d_b3, eq36_e1324_d_b4, eq36_e1324_d_b5, eq36_e1324_d_b6, eq36_e1324_d_b7, eq36_e1324_d_b8];
        stamper.stamp_current_dense_local(
            Some(10),
            Some(7),
            multiplicity * (eq36_value),
            &eq36_node_derivatives,
            &eq36_branch_derivatives,
            multiplicity,
        );
        let (eq37_e1331, eq37_e1331_d_n0, eq37_e1331_d_n1, eq37_e1331_d_n2, eq37_e1331_d_n3, eq37_e1331_d_n4, eq37_e1331_d_n5, eq37_e1331_d_n6, eq37_e1331_d_n7, eq37_e1331_d_n8, eq37_e1331_d_n9, eq37_e1331_d_n10, eq37_e1331_d_n11, eq37_e1331_d_n12, eq37_e1331_d_b0, eq37_e1331_d_b1, eq37_e1331_d_b2, eq37_e1331_d_b3, eq37_e1331_d_b4, eq37_e1331_d_b5, eq37_e1331_d_b6, eq37_e1331_d_b7, eq37_e1331_d_b8,) = {
    if s.b[1863] {
        let eq37_e1328: f64 = eval_ddt(ddt_state_current, ddt_state_previous, ddt_state_older, ddt_state_initialized, ddt_derivative_current, ddt_derivative_previous, ddt_active, ddt_scale, ddt_previous_value_scale, ddt_older_value_scale, ddt_previous_derivative_scale, 7, s.v[1231]);let eq37_e1329: f64 = (s.v[36] * eq37_e1328);let eq37_e1329_d_n0: f64 = (s.v[36] * (s.dn[1231][0] * ddt_scale));let eq37_e1329_d_n1: f64 = (s.v[36] * (s.dn[1231][1] * ddt_scale));let eq37_e1329_d_n2: f64 = (s.v[36] * (s.dn[1231][2] * ddt_scale));let eq37_e1329_d_n3: f64 = (s.v[36] * (s.dn[1231][3] * ddt_scale));let eq37_e1329_d_n4: f64 = (s.v[36] * (s.dn[1231][4] * ddt_scale));let eq37_e1329_d_n5: f64 = (s.v[36] * (s.dn[1231][5] * ddt_scale));let eq37_e1329_d_n6: f64 = (s.v[36] * (s.dn[1231][6] * ddt_scale));let eq37_e1329_d_n7: f64 = (s.v[36] * (s.dn[1231][7] * ddt_scale));let eq37_e1329_d_n8: f64 = (s.v[36] * (s.dn[1231][8] * ddt_scale));let eq37_e1329_d_n9: f64 = (s.v[36] * (s.dn[1231][9] * ddt_scale));let eq37_e1329_d_n10: f64 = (s.v[36] * (s.dn[1231][10] * ddt_scale));let eq37_e1329_d_n11: f64 = (s.v[36] * (s.dn[1231][11] * ddt_scale));let eq37_e1329_d_n12: f64 = (s.v[36] * (s.dn[1231][12] * ddt_scale));let eq37_e1329_d_b0: f64 = (s.v[36] * (s.db[1231][0] * ddt_scale));let eq37_e1329_d_b1: f64 = (s.v[36] * (s.db[1231][1] * ddt_scale));let eq37_e1329_d_b2: f64 = (s.v[36] * (s.db[1231][2] * ddt_scale));let eq37_e1329_d_b3: f64 = (s.v[36] * (s.db[1231][3] * ddt_scale));let eq37_e1329_d_b4: f64 = (s.v[36] * (s.db[1231][4] * ddt_scale));let eq37_e1329_d_b5: f64 = (s.v[36] * (s.db[1231][5] * ddt_scale));let eq37_e1329_d_b6: f64 = (s.v[36] * (s.db[1231][6] * ddt_scale));let eq37_e1329_d_b7: f64 = (s.v[36] * (s.db[1231][7] * ddt_scale));let eq37_e1329_d_b8: f64 = (s.v[36] * (s.db[1231][8] * ddt_scale));
        (eq37_e1329, eq37_e1329_d_n0, eq37_e1329_d_n1, eq37_e1329_d_n2, eq37_e1329_d_n3, eq37_e1329_d_n4, eq37_e1329_d_n5, eq37_e1329_d_n6, eq37_e1329_d_n7, eq37_e1329_d_n8, eq37_e1329_d_n9, eq37_e1329_d_n10, eq37_e1329_d_n11, eq37_e1329_d_n12, eq37_e1329_d_b0, eq37_e1329_d_b1, eq37_e1329_d_b2, eq37_e1329_d_b3, eq37_e1329_d_b4, eq37_e1329_d_b5, eq37_e1329_d_b6, eq37_e1329_d_b7, eq37_e1329_d_b8,)
    } else {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    }
};
        let eq37_value: f64 = eq37_e1331;let eq37_node_derivatives: [f64; 13] = [eq37_e1331_d_n0, eq37_e1331_d_n1, eq37_e1331_d_n2, eq37_e1331_d_n3, eq37_e1331_d_n4, eq37_e1331_d_n5, eq37_e1331_d_n6, eq37_e1331_d_n7, eq37_e1331_d_n8, eq37_e1331_d_n9, eq37_e1331_d_n10, eq37_e1331_d_n11, eq37_e1331_d_n12];let eq37_branch_derivatives: [f64; 9] = [eq37_e1331_d_b0, eq37_e1331_d_b1, eq37_e1331_d_b2, eq37_e1331_d_b3, eq37_e1331_d_b4, eq37_e1331_d_b5, eq37_e1331_d_b6, eq37_e1331_d_b7, eq37_e1331_d_b8];
        stamper.stamp_current_dense_local(
            Some(10),
            Some(8),
            multiplicity * (eq37_value),
            &eq37_node_derivatives,
            &eq37_branch_derivatives,
            multiplicity,
        );
    }
}
