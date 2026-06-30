#![allow(dead_code, unused_assignments, unused_imports, unused_parens, unused_variables)]

use super::{A, ddt_jacobian, eval_ddt, eval_idt, GeneratedDerivative, GeneratedEvalContext, GeneratedReactiveStamper, GeneratedStamper, idt_jacobian, ReactiveScratch, Scratch, LIMEXP_MAX, THERMAL_VOLTAGE_PER_K};
use super::super::state::{Instance, Parameters};

impl Instance {

    pub(super) fn stamp_reactive_block_19(
        s: &mut ReactiveScratch,
        p: &Parameters,
    ) {
        if ((!s.b[1733]) && (!s.b[1751])) {
            s.store_scalar(1254, 0.0);
            s.store_scalar(1253, 0.0);
            s.store_scalar(1251, 0.0);
        }

        s.b[1785] = (s.v[57] == 2.0);
        s.store_scalar(1785, if s.b[1785] { 1.0 } else { 0.0 });

        if s.b[1785] {
            s.store_scalar(1244, 0.0);
            s.store_scalar(1245, 0.0);
        }

        if (!s.b[1785]) {
            s.copy_ad(1151, 200);
            s.store_scalar(1315, (-s.v[344]));
            s.store_add_scaled_offset_product_rhs(1151, 1151, 1.0, 1315, 769, (-s.v[150]), 1.0);
            s.copy_ad(1152, 202);
            s.store_scalar(1311, ((((s.v[204] * s.v[711]) * s.v[174]) * s.v[39]) / 1e-7));
            s.store_scale(1314, 1311, s.v[343]);
            s.store_add_scaled_offset_product_rhs(1311, 1311, 1.0, 1314, 769, (-s.v[150]), 1.0);
            s.store_scalar(1312, ((((s.v[205] * s.v[710]) * s.v[174]) * s.v[39]) / 1e-7));
            s.store_scale(1313, 1312, s.v[345]);
            s.store_add_scaled_offset_product_rhs(1312, 1312, 1.0, 1313, 769, (-s.v[150]), 1.0);
            s.store_scale(1329, 1151, 0.9);
        }

        if (!s.b[1785]) {
            s.store_sub_from_scalar_div_mixed_ai(1147, 1.0, {
                if (s.v[1421] > s.v[1329]) {
                    s.ad_value(1329)
                } else {
                    s.ad_value(1421)
                }
            }, 1151);
        }

        s.b[1786] = (p.p173 == 0.5);
        s.store_scalar(1786, if s.b[1786] { 1.0 } else { 0.0 });

        if ((!s.b[1785]) && s.b[1786]) {
            s.store_div_from_scalar_sqrt_ad(1193, 1.0, s.ad_value(1147));
        }

        if ((!s.b[1785]) && (!s.b[1786])) {
            s.store_exp_scaled_input_ad(1193, {
                if (s.v[1147] > 1e-38) {
                    A::ln(s.ad_value(1147))
                } else {
                    A::neg(A::constant(87.49823353377374))
                }
            }, (-p.p173));
        }

        if (!s.b[1785]) {
            s.store_mul_sub_from_scalar_lhs_scaled_ad_lhs(1182, 1.0, A::mul(s.ad_value(1147), s.ad_value(1193)), 1151, 1.0 / ((1.0 - p.p173)));
        }

        s.b[1787] = (s.v[1421] > s.v[1329]);
        s.store_scalar(1787, if s.b[1787] { 1.0 } else { 0.0 });

        if ((!s.b[1785]) && s.b[1787]) {
            s.store_add_scaled_product_right_sub(1182, 1182, 1.0, 1193, 1421, 1329, 1.0);
        }

        if (!s.b[1785]) {
            s.store_add_scaled_product_indices(1245, 1322, (s.v[332] * s.v[39]), 1311, 1182, 1.0);
            s.copy_ad(1151, 201);
            s.store_scalar(1315, (-s.v[346]));
            s.store_add_scaled_offset_product_rhs(1151, 1151, 1.0, 1315, 769, (-s.v[150]), 1.0);
            s.store_scalar(1152, s.v[203]);
            s.store_scale(1329, 1151, 0.9);
        }

        if (!s.b[1785]) {
            s.store_sub_from_scalar_div_mixed_ai(1147, 1.0, {
                if (s.v[1422] > s.v[1329]) {
                    s.ad_value(1329)
                } else {
                    s.ad_value(1422)
                }
            }, 1151);
        }

        s.b[1788] = (p.p173 == 0.5);
        s.store_scalar(1788, if s.b[1788] { 1.0 } else { 0.0 });

        if ((!s.b[1785]) && s.b[1788]) {
            s.store_div_from_scalar_sqrt_ad(1193, 1.0, s.ad_value(1147));
        }

        if ((!s.b[1785]) && (!s.b[1788])) {
            s.store_exp_scaled_input_ad(1193, {
                if (s.v[1147] > 1e-38) {
                    A::ln(s.ad_value(1147))
                } else {
                    A::neg(A::constant(87.49823353377374))
                }
            }, (-p.p173));
        }

        if (!s.b[1785]) {
            s.store_mul_sub_from_scalar_lhs_scaled_ad_lhs(1182, 1.0, A::mul(s.ad_value(1147), s.ad_value(1193)), 1151, 1.0 / ((1.0 - p.p173)));
        }

        s.b[1789] = (s.v[1422] > s.v[1329]);
        s.store_scalar(1789, if s.b[1789] { 1.0 } else { 0.0 });

        if ((!s.b[1785]) && s.b[1789]) {
            s.store_add_scaled_product_right_sub(1182, 1182, 1.0, 1193, 1422, 1329, 1.0);
        }

        if (!s.b[1785]) {
            s.store_add_scaled_product_indices(1244, 1323, (s.v[332] * s.v[39]), 1312, 1182, 1.0);
        }

        s.store_scale(1189, 1232, (-s.v[36]));

        s.store_scaled_sub(1190, 1155, 1232, s.v[36]);

        s.b[1790] = (s.v[336] != 0.0);
        s.store_scalar(1790, if s.b[1790] { 1.0 } else { 0.0 });

        s.b[1791] = (((s.v[479] > 0.0) && (s.v[36] > 0.0)) || ((s.v[479] < 0.0) && (s.v[36] < 0.0)));
        s.store_scalar(1791, if s.b[1791] { 1.0 } else { 0.0 });

        s.b[1792] = (s.v[1189] < s.v[683]);
        s.store_scalar(1792, if s.b[1792] { 1.0 } else { 0.0 });

        if ((s.b[1790] && s.b[1791]) && s.b[1792]) {
            s.store_scaled_sub(448, 1189, 683, s.v[430]);
        }

        s.b[1793] = (s.v[1189] < s.v[545]);
        s.store_scalar(1793, if s.b[1793] { 1.0 } else { 0.0 });

        if (((s.b[1790] && s.b[1791]) && (!s.b[1792])) && s.b[1793]) {
            s.store_sub(1179, 1189, 683);
            s.store_square(1180, 1179);
            s.store_mul_sub_from_scalar_ad_rhs(448, 1179, s.v[430], A::mul_scaled_lhs(s.ad_value(546), 1.0 / (3.0), s.ad_value(1180)));
        }

        s.b[1794] = (s.v[1189] < s.v[684]);
        s.store_scalar(1794, if s.b[1794] { 1.0 } else { 0.0 });

        if ((((s.b[1790] && s.b[1791]) && (!s.b[1792])) && (!s.b[1793])) && s.b[1794]) {
            s.store_sub(1179, 1189, 684);
            s.store_square(1180, 1179);
            s.store_add_ad(448, A::add_scaled_product(s.ad_value(434), 1.0, s.ad_value(432), s.ad_value(1189), 1.0), A::mul3_scaled_output(s.ad_value(547), s.ad_value(1179), s.ad_value(1180), 1.0 / (3.0)));
        }

        if ((((s.b[1790] && s.b[1791]) && (!s.b[1792])) && (!s.b[1793])) && (!s.b[1794])) {
            s.store_add_scaled_product_indices(448, 434, 1.0, 432, 1189, 1.0);
        }

        s.b[1795] = (s.v[1189] < s.v[684]);
        s.store_scalar(1795, if s.b[1795] { 1.0 } else { 0.0 });

        if ((s.b[1790] && (!s.b[1791])) && s.b[1795]) {
            s.store_mul_sub_rhs(448, 432, 1189, 684);
        }

        s.b[1796] = (s.v[1189] < s.v[545]);
        s.store_scalar(1796, if s.b[1796] { 1.0 } else { 0.0 });

        if (((s.b[1790] && (!s.b[1791])) && (!s.b[1795])) && s.b[1796]) {
            s.store_sub(1179, 1189, 684);
            s.store_square(1180, 1179);
            s.store_mul_add_scaled_product_rhs(448, 1179, s.ad_value(432), 1.0, s.ad_value(546), s.ad_value(1180), (-1.0 / (3.0)));
        }

        s.b[1797] = (s.v[1189] < s.v[683]);
        s.store_scalar(1797, if s.b[1797] { 1.0 } else { 0.0 });

        if ((((s.b[1790] && (!s.b[1791])) && (!s.b[1795])) && (!s.b[1796])) && s.b[1797]) {
            s.store_sub(1179, 1189, 683);
            s.store_square(1180, 1179);
            s.store_add_scaled_inputs3_mixed_iia(448, 1189, s.v[430], 434, 1.0, A::mul3_scaled_output(s.ad_value(547), s.ad_value(1179), s.ad_value(1180), 1.0 / (3.0)), 1.0);
        }

        if ((((s.b[1790] && (!s.b[1791])) && (!s.b[1795])) && (!s.b[1796])) && (!s.b[1797])) {
            s.store_add_scaled_inputs(448, 1189, s.v[430], 434, 1.0);
        }

        s.b[1798] = (((s.v[479] > 0.0) && (s.v[36] > 0.0)) || ((s.v[479] < 0.0) && (s.v[36] < 0.0)));
        s.store_scalar(1798, if s.b[1798] { 1.0 } else { 0.0 });

        s.b[1799] = (s.v[1190] < s.v[683]);
        s.store_scalar(1799, if s.b[1799] { 1.0 } else { 0.0 });

        if ((s.b[1790] && s.b[1798]) && s.b[1799]) {
            s.store_scaled_sub(449, 1190, 683, s.v[431]);
        }

        s.b[1800] = (s.v[1190] < s.v[545]);
        s.store_scalar(1800, if s.b[1800] { 1.0 } else { 0.0 });

        if (((s.b[1790] && s.b[1798]) && (!s.b[1799])) && s.b[1800]) {
            s.store_sub(1179, 1190, 683);
            s.store_square(1180, 1179);
            s.store_mul_sub_from_scalar_ad_rhs(449, 1179, s.v[431], A::mul_scaled_lhs(s.ad_value(548), 1.0 / (3.0), s.ad_value(1180)));
        }

        s.b[1801] = (s.v[1190] < s.v[684]);
        s.store_scalar(1801, if s.b[1801] { 1.0 } else { 0.0 });

        if ((((s.b[1790] && s.b[1798]) && (!s.b[1799])) && (!s.b[1800])) && s.b[1801]) {
            s.store_sub(1179, 1190, 684);
            s.store_square(1180, 1179);
            s.store_add_ad(449, A::add_scaled_product(s.ad_value(435), 1.0, s.ad_value(433), s.ad_value(1190), 1.0), A::mul3_scaled_output(s.ad_value(549), s.ad_value(1179), s.ad_value(1180), 1.0 / (3.0)));
        }

        if ((((s.b[1790] && s.b[1798]) && (!s.b[1799])) && (!s.b[1800])) && (!s.b[1801])) {
            s.store_add_scaled_product_indices(449, 435, 1.0, 433, 1190, 1.0);
        }

        s.b[1802] = (s.v[1190] < s.v[684]);
        s.store_scalar(1802, if s.b[1802] { 1.0 } else { 0.0 });

        if ((s.b[1790] && (!s.b[1798])) && s.b[1802]) {
            s.store_mul_sub_rhs(449, 433, 1190, 684);
        }

        s.b[1803] = (s.v[1190] < s.v[545]);
        s.store_scalar(1803, if s.b[1803] { 1.0 } else { 0.0 });

        if (((s.b[1790] && (!s.b[1798])) && (!s.b[1802])) && s.b[1803]) {
            s.store_sub(1179, 1190, 684);
            s.store_square(1180, 1179);
            s.store_mul_add_scaled_product_rhs(449, 1179, s.ad_value(433), 1.0, s.ad_value(548), s.ad_value(1180), (-1.0 / (3.0)));
        }

        s.b[1804] = (s.v[1190] < s.v[683]);
        s.store_scalar(1804, if s.b[1804] { 1.0 } else { 0.0 });

        if ((((s.b[1790] && (!s.b[1798])) && (!s.b[1802])) && (!s.b[1803])) && s.b[1804]) {
            s.store_sub(1179, 1190, 683);
            s.store_square(1180, 1179);
            s.store_add_scaled_inputs3_mixed_iia(449, 1190, s.v[431], 435, 1.0, A::mul3_scaled_output(s.ad_value(549), s.ad_value(1179), s.ad_value(1180), 1.0 / (3.0)), 1.0);
        }

        if ((((s.b[1790] && (!s.b[1798])) && (!s.b[1802])) && (!s.b[1803])) && (!s.b[1804])) {
            s.store_add_scaled_inputs(449, 1190, s.v[431], 435, 1.0);
        }

        if (!s.b[1790]) {
            s.store_scale(448, 1189, s.v[430]);
            s.store_scale(449, 1190, s.v[431]);
        }

        s.store_add_scaled_product_indices(448, 448, 1.0, 428, 1189, 1.0);

        s.store_add_scaled_product_indices(449, 449, 1.0, 429, 1190, 1.0);

        s.b[1805] = (s.v[66] == 3.0);
        s.store_scalar(1805, if s.b[1805] { 1.0 } else { 0.0 });

        if s.b[1805] {
            s.store_offset(1179, 1354, 0.02);
        }

        if (!s.b[1805]) {
            s.store_offset(1179, 1156, 0.02);
        }

        s.store_sqrt_square_offset(1180, 1179, (4.0 * 0.02));

        s.store_scaled_sub(1181, 1179, 1180, 0.5);

        s.store_scale(1182, 603, s.v[710]);

        s.store_sqrt_sub_from_scalar_ad(1183, 1.0, A::div_scaled_inputs(s.ad_value(1181), 4.0, s.ad_value(604), 1.0));

        s.b[1806] = (s.v[66] == 3.0);
        s.store_scalar(1806, if s.b[1806] { 1.0 } else { 0.0 });

        if s.b[1806] {
            s.store_add_scaled_products_mixed_aiia(1230, A::add(s.ad_value(696), s.ad_value(1182)), 1354, 1.0, 1182, A::add_scaled_offset_product_rhs(s.ad_value(1181), 1.0, s.ad_value(604), s.ad_value(1183), (-1.0), 0.5), (-1.0));
        }

        if (!s.b[1806]) {
            s.store_add_scaled_products_mixed_aiia(1230, A::add(s.ad_value(696), s.ad_value(1182)), 1156, 1.0, 1182, A::add_scaled_offset_product_rhs(s.ad_value(1181), 1.0, s.ad_value(604), s.ad_value(1183), (-1.0), 0.5), (-1.0));
        }

        s.b[1807] = (s.v[66] == 3.0);
        s.store_scalar(1807, if s.b[1807] { 1.0 } else { 0.0 });

        if s.b[1807] {
            s.store_offset(1179, 1353, 0.02);
        }

        if (!s.b[1807]) {
            s.store_offset(1179, 1157, 0.02);
        }

        s.store_sqrt_square_offset(1180, 1179, (4.0 * 0.02));

        s.store_scaled_sub(1181, 1179, 1180, 0.5);

        s.store_scale(1182, 602, s.v[711]);

        s.store_sqrt_sub_from_scalar_ad(1183, 1.0, A::div_scaled_inputs(s.ad_value(1181), 4.0, s.ad_value(604), 1.0));

        s.b[1808] = (s.v[66] == 3.0);
        s.store_scalar(1808, if s.b[1808] { 1.0 } else { 0.0 });

        if s.b[1808] {
            s.store_add_scaled_products_mixed_aiia(1231, A::add(s.ad_value(695), s.ad_value(1182)), 1353, 1.0, 1182, A::add_scaled_offset_product_rhs(s.ad_value(1181), 1.0, s.ad_value(604), s.ad_value(1183), (-1.0), 0.5), (-1.0));
        }

        if (!s.b[1808]) {
            s.store_add_scaled_products_mixed_aiia(1231, A::add(s.ad_value(695), s.ad_value(1182)), 1157, 1.0, 1182, A::add_scaled_offset_product_rhs(s.ad_value(1181), 1.0, s.ad_value(604), s.ad_value(1183), (-1.0), 0.5), (-1.0));
        }

        s.b[1809] = (s.v[39] != 1.0);
        s.store_scalar(1809, if s.b[1809] { 1.0 } else { 0.0 });

        if s.b[1809] {
            s.store_scale(1230, 1230, s.v[39]);
            s.store_scale(1231, 1231, s.v[39]);
        }

        s.copy_ad(798, 1251);

        s.store_add(797, 1231, 1230);

        s.store_add(1251, 798, 797);

        s.b[1823] = (p.p213 == 0.0);
        s.store_scalar(1823, if s.b[1823] { 1.0 } else { 0.0 });

        s.b[1824] = (p.p213 == 1.0);
        s.store_scalar(1824, if s.b[1824] { 1.0 } else { 0.0 });

        if (s.b[1824] && (!s.b[1823])) {
            s.store_add_scaled_inputs3_indices(1179, 439, 1.0, 440, 1.0, 441, 1.0);
            s.store_square(1179, 1179);
            s.store_div_scaled_inputs_indices(1817, 1281, 2.0, 410, 1.0);
            s.store_div_scaled_inputs_indices(1184, 451, 1.0, 1817, s.v[688]);
            s.store_square(1184, 1184);
            s.store_offset_scaled(1818, 1184, (((s.v[241] * s.v[688])) * (s.v[243])), s.v[243]);
            s.store_add_scaled_product_right_ad(1180, 440, 1.0, 1818, A::add(s.ad_value(439), s.ad_value(441)), 1.0);
            s.store_div_scaled_product_indices(1181, 1180, 1180, 1.0, 454, 1.0);
        }

        s.b[1861] = (s.v[759] > 0.0);
        s.store_scalar(1861, if s.b[1861] { 1.0 } else { 0.0 });

        if s.b[1861] {
            s.store_scale(446, 1253, s.v[36]);
            s.store_scale(447, 1254, s.v[36]);
        }

        if (!s.b[1861]) {
            s.store_scale(447, 1253, s.v[36]);
            s.store_scale(446, 1254, s.v[36]);
        }

        s.b[1863] = (p.p37 == 3.0);
        s.store_scalar(1863, if s.b[1863] { 1.0 } else { 0.0 });

        s.b[1869] = ((p.p33 == 1.0) && (p.p16 != 0.0));
        s.store_scalar(1869, if s.b[1869] { 1.0 } else { 0.0 });

    }

    pub(super) fn stamp_transient_equations_block_0(
        ctx: &GeneratedEvalContext<'_>,
        stamper: &mut GeneratedStamper<'_>,
        nodes: &[usize; Instance::NODE_COUNT],
        multiplicity: f64,
        var_b4soiig: f64,
        var_b4soiig_db0: f64,
        var_b4soiig_db1: f64,
        var_b4soiig_db2: f64,
        var_b4soiig_db3: f64,
        var_b4soiig_db4: f64,
        var_b4soiig_db5: f64,
        var_b4soiig_db6: f64,
        var_b4soiig_db7: f64,
        var_b4soiig_db8: f64,
        var_b4soiig_dn0: f64,
        var_b4soiig_dn1: f64,
        var_b4soiig_dn10: f64,
        var_b4soiig_dn11: f64,
        var_b4soiig_dn12: f64,
        var_b4soiig_dn2: f64,
        var_b4soiig_dn3: f64,
        var_b4soiig_dn4: f64,
        var_b4soiig_dn5: f64,
        var_b4soiig_dn6: f64,
        var_b4soiig_dn7: f64,
        var_b4soiig_dn8: f64,
        var_b4soiig_dn9: f64,
        var_b4soiigcd: f64,
        var_b4soiigcd_db0: f64,
        var_b4soiigcd_db1: f64,
        var_b4soiigcd_db2: f64,
        var_b4soiigcd_db3: f64,
        var_b4soiigcd_db4: f64,
        var_b4soiigcd_db5: f64,
        var_b4soiigcd_db6: f64,
        var_b4soiigcd_db7: f64,
        var_b4soiigcd_db8: f64,
        var_b4soiigcd_dn0: f64,
        var_b4soiigcd_dn1: f64,
        var_b4soiigcd_dn10: f64,
        var_b4soiigcd_dn11: f64,
        var_b4soiigcd_dn12: f64,
        var_b4soiigcd_dn2: f64,
        var_b4soiigcd_dn3: f64,
        var_b4soiigcd_dn4: f64,
        var_b4soiigcd_dn5: f64,
        var_b4soiigcd_dn6: f64,
        var_b4soiigcd_dn7: f64,
        var_b4soiigcd_dn8: f64,
        var_b4soiigcd_dn9: f64,
        var_b4soiigcs: f64,
        var_b4soiigcs_db0: f64,
        var_b4soiigcs_db1: f64,
        var_b4soiigcs_db2: f64,
        var_b4soiigcs_db3: f64,
        var_b4soiigcs_db4: f64,
        var_b4soiigcs_db5: f64,
        var_b4soiigcs_db6: f64,
        var_b4soiigcs_db7: f64,
        var_b4soiigcs_db8: f64,
        var_b4soiigcs_dn0: f64,
        var_b4soiigcs_dn1: f64,
        var_b4soiigcs_dn10: f64,
        var_b4soiigcs_dn11: f64,
        var_b4soiigcs_dn12: f64,
        var_b4soiigcs_dn2: f64,
        var_b4soiigcs_dn3: f64,
        var_b4soiigcs_dn4: f64,
        var_b4soiigcs_dn5: f64,
        var_b4soiigcs_dn6: f64,
        var_b4soiigcs_dn7: f64,
        var_b4soiigcs_dn8: f64,
        var_b4soiigcs_dn9: f64,
        var_b4soiigd: f64,
        var_b4soiigd_db0: f64,
        var_b4soiigd_db1: f64,
        var_b4soiigd_db2: f64,
        var_b4soiigd_db3: f64,
        var_b4soiigd_db4: f64,
        var_b4soiigd_db5: f64,
        var_b4soiigd_db6: f64,
        var_b4soiigd_db7: f64,
        var_b4soiigd_db8: f64,
        var_b4soiigd_dn0: f64,
        var_b4soiigd_dn1: f64,
        var_b4soiigd_dn10: f64,
        var_b4soiigd_dn11: f64,
        var_b4soiigd_dn12: f64,
        var_b4soiigd_dn2: f64,
        var_b4soiigd_dn3: f64,
        var_b4soiigd_dn4: f64,
        var_b4soiigd_dn5: f64,
        var_b4soiigd_dn6: f64,
        var_b4soiigd_dn7: f64,
        var_b4soiigd_dn8: f64,
        var_b4soiigd_dn9: f64,
        var_b4soiigidl: f64,
        var_b4soiigidl_db0: f64,
        var_b4soiigidl_db1: f64,
        var_b4soiigidl_db2: f64,
        var_b4soiigidl_db3: f64,
        var_b4soiigidl_db4: f64,
        var_b4soiigidl_db5: f64,
        var_b4soiigidl_db6: f64,
        var_b4soiigidl_db7: f64,
        var_b4soiigidl_db8: f64,
        var_b4soiigidl_dn0: f64,
        var_b4soiigidl_dn1: f64,
        var_b4soiigidl_dn10: f64,
        var_b4soiigidl_dn11: f64,
        var_b4soiigidl_dn12: f64,
        var_b4soiigidl_dn2: f64,
        var_b4soiigidl_dn3: f64,
        var_b4soiigidl_dn4: f64,
        var_b4soiigidl_dn5: f64,
        var_b4soiigidl_dn6: f64,
        var_b4soiigidl_dn7: f64,
        var_b4soiigidl_dn8: f64,
        var_b4soiigidl_dn9: f64,
        var_b4soiigisl: f64,
        var_b4soiigisl_db0: f64,
        var_b4soiigisl_db1: f64,
        var_b4soiigisl_db2: f64,
        var_b4soiigisl_db3: f64,
        var_b4soiigisl_db4: f64,
        var_b4soiigisl_db5: f64,
        var_b4soiigisl_db6: f64,
        var_b4soiigisl_db7: f64,
        var_b4soiigisl_db8: f64,
        var_b4soiigisl_dn0: f64,
        var_b4soiigisl_dn1: f64,
        var_b4soiigisl_dn10: f64,
        var_b4soiigisl_dn11: f64,
        var_b4soiigisl_dn12: f64,
        var_b4soiigisl_dn2: f64,
        var_b4soiigisl_dn3: f64,
        var_b4soiigisl_dn4: f64,
        var_b4soiigisl_dn5: f64,
        var_b4soiigisl_dn6: f64,
        var_b4soiigisl_dn7: f64,
        var_b4soiigisl_dn8: f64,
        var_b4soiigisl_dn9: f64,
        var_b4soiigp: f64,
        var_b4soiigp_db0: f64,
        var_b4soiigp_db1: f64,
        var_b4soiigp_db2: f64,
        var_b4soiigp_db3: f64,
        var_b4soiigp_db4: f64,
        var_b4soiigp_db5: f64,
        var_b4soiigp_db6: f64,
        var_b4soiigp_db7: f64,
        var_b4soiigp_db8: f64,
        var_b4soiigp_dn0: f64,
        var_b4soiigp_dn1: f64,
        var_b4soiigp_dn10: f64,
        var_b4soiigp_dn11: f64,
        var_b4soiigp_dn12: f64,
        var_b4soiigp_dn2: f64,
        var_b4soiigp_dn3: f64,
        var_b4soiigp_dn4: f64,
        var_b4soiigp_dn5: f64,
        var_b4soiigp_dn6: f64,
        var_b4soiigp_dn7: f64,
        var_b4soiigp_dn8: f64,
        var_b4soiigp_dn9: f64,
        var_b4soiigs: f64,
        var_b4soiigs_db0: f64,
        var_b4soiigs_db1: f64,
        var_b4soiigs_db2: f64,
        var_b4soiigs_db3: f64,
        var_b4soiigs_db4: f64,
        var_b4soiigs_db5: f64,
        var_b4soiigs_db6: f64,
        var_b4soiigs_db7: f64,
        var_b4soiigs_db8: f64,
        var_b4soiigs_dn0: f64,
        var_b4soiigs_dn1: f64,
        var_b4soiigs_dn10: f64,
        var_b4soiigs_dn11: f64,
        var_b4soiigs_dn12: f64,
        var_b4soiigs_dn2: f64,
        var_b4soiigs_dn3: f64,
        var_b4soiigs_dn4: f64,
        var_b4soiigs_dn5: f64,
        var_b4soiigs_dn6: f64,
        var_b4soiigs_dn7: f64,
        var_b4soiigs_dn8: f64,
        var_b4soiigs_dn9: f64,
        var_b4soitype: f64,
        var_guard1825: f64,
        var_guard1826: f64,
        var_ibd_1: f64,
        var_ibd_1_db0: f64,
        var_ibd_1_db1: f64,
        var_ibd_1_db2: f64,
        var_ibd_1_db3: f64,
        var_ibd_1_db4: f64,
        var_ibd_1_db5: f64,
        var_ibd_1_db6: f64,
        var_ibd_1_db7: f64,
        var_ibd_1_db8: f64,
        var_ibd_1_dn0: f64,
        var_ibd_1_dn1: f64,
        var_ibd_1_dn10: f64,
        var_ibd_1_dn11: f64,
        var_ibd_1_dn12: f64,
        var_ibd_1_dn2: f64,
        var_ibd_1_dn3: f64,
        var_ibd_1_dn4: f64,
        var_ibd_1_dn5: f64,
        var_ibd_1_dn6: f64,
        var_ibd_1_dn7: f64,
        var_ibd_1_dn8: f64,
        var_ibd_1_dn9: f64,
        var_ibs_1: f64,
        var_ibs_1_db0: f64,
        var_ibs_1_db1: f64,
        var_ibs_1_db2: f64,
        var_ibs_1_db3: f64,
        var_ibs_1_db4: f64,
        var_ibs_1_db5: f64,
        var_ibs_1_db6: f64,
        var_ibs_1_db7: f64,
        var_ibs_1_db8: f64,
        var_ibs_1_dn0: f64,
        var_ibs_1_dn1: f64,
        var_ibs_1_dn10: f64,
        var_ibs_1_dn11: f64,
        var_ibs_1_dn12: f64,
        var_ibs_1_dn2: f64,
        var_ibs_1_dn3: f64,
        var_ibs_1_dn4: f64,
        var_ibs_1_dn5: f64,
        var_ibs_1_dn6: f64,
        var_ibs_1_dn7: f64,
        var_ibs_1_dn8: f64,
        var_ibs_1_dn9: f64,
        var_ic_1: f64,
        var_ic_1_db0: f64,
        var_ic_1_db1: f64,
        var_ic_1_db2: f64,
        var_ic_1_db3: f64,
        var_ic_1_db4: f64,
        var_ic_1_db5: f64,
        var_ic_1_db6: f64,
        var_ic_1_db7: f64,
        var_ic_1_db8: f64,
        var_ic_1_dn0: f64,
        var_ic_1_dn1: f64,
        var_ic_1_dn10: f64,
        var_ic_1_dn11: f64,
        var_ic_1_dn12: f64,
        var_ic_1_dn2: f64,
        var_ic_1_dn3: f64,
        var_ic_1_dn4: f64,
        var_ic_1_dn5: f64,
        var_ic_1_dn6: f64,
        var_ic_1_dn7: f64,
        var_ic_1_dn8: f64,
        var_ic_1_dn9: f64,
        var_ids_1: f64,
        var_ids_1_db0: f64,
        var_ids_1_db1: f64,
        var_ids_1_db2: f64,
        var_ids_1_db3: f64,
        var_ids_1_db4: f64,
        var_ids_1_db5: f64,
        var_ids_1_db6: f64,
        var_ids_1_db7: f64,
        var_ids_1_db8: f64,
        var_ids_1_dn0: f64,
        var_ids_1_dn1: f64,
        var_ids_1_dn10: f64,
        var_ids_1_dn11: f64,
        var_ids_1_dn12: f64,
        var_ids_1_dn2: f64,
        var_ids_1_dn3: f64,
        var_ids_1_dn4: f64,
        var_ids_1_dn5: f64,
        var_ids_1_dn6: f64,
        var_ids_1_dn7: f64,
        var_ids_1_dn8: f64,
        var_ids_1_dn9: f64,
        var_iii: f64,
        var_iii_db0: f64,
        var_iii_db1: f64,
        var_iii_db2: f64,
        var_iii_db3: f64,
        var_iii_db4: f64,
        var_iii_db5: f64,
        var_iii_db6: f64,
        var_iii_db7: f64,
        var_iii_db8: f64,
        var_iii_dn0: f64,
        var_iii_dn1: f64,
        var_iii_dn10: f64,
        var_iii_dn11: f64,
        var_iii_dn12: f64,
        var_iii_dn2: f64,
        var_iii_dn3: f64,
        var_iii_dn4: f64,
        var_iii_dn5: f64,
        var_iii_dn6: f64,
        var_iii_dn7: f64,
        var_iii_dn8: f64,
        var_iii_dn9: f64,
        var_rd: f64,
        var_rd_db0: f64,
        var_rd_db1: f64,
        var_rd_db2: f64,
        var_rd_db3: f64,
        var_rd_db4: f64,
        var_rd_db5: f64,
        var_rd_db6: f64,
        var_rd_db7: f64,
        var_rd_db8: f64,
        var_rd_dn0: f64,
        var_rd_dn1: f64,
        var_rd_dn10: f64,
        var_rd_dn11: f64,
        var_rd_dn12: f64,
        var_rd_dn2: f64,
        var_rd_dn3: f64,
        var_rd_dn4: f64,
        var_rd_dn5: f64,
        var_rd_dn6: f64,
        var_rd_dn7: f64,
        var_rd_dn8: f64,
        var_rd_dn9: f64,
        var_rs: f64,
        var_rs_db0: f64,
        var_rs_db1: f64,
        var_rs_db2: f64,
        var_rs_db3: f64,
        var_rs_db4: f64,
        var_rs_db5: f64,
        var_rs_db6: f64,
        var_rs_db7: f64,
        var_rs_db8: f64,
        var_rs_dn0: f64,
        var_rs_dn1: f64,
        var_rs_dn10: f64,
        var_rs_dn11: f64,
        var_rs_dn12: f64,
        var_rs_dn2: f64,
        var_rs_dn3: f64,
        var_rs_dn4: f64,
        var_rs_dn5: f64,
        var_rs_dn6: f64,
        var_rs_dn7: f64,
        var_rs_dn8: f64,
        var_rs_dn9: f64,
    ) {
        let nv0 = ctx.node_voltage(nodes[0]);
        let nv2 = ctx.node_voltage(nodes[2]);
        let nv7 = ctx.node_voltage(nodes[7]);
        let nv8 = ctx.node_voltage(nodes[8]);
        let (eq4_e1143, eq4_e1143_d_n0, eq4_e1143_d_n1, eq4_e1143_d_n2, eq4_e1143_d_n3, eq4_e1143_d_n4, eq4_e1143_d_n5, eq4_e1143_d_n6, eq4_e1143_d_n7, eq4_e1143_d_n8, eq4_e1143_d_n9, eq4_e1143_d_n10, eq4_e1143_d_n11, eq4_e1143_d_n12, eq4_e1143_d_b0, eq4_e1143_d_b1, eq4_e1143_d_b2, eq4_e1143_d_b3, eq4_e1143_d_b4, eq4_e1143_d_b5, eq4_e1143_d_b6, eq4_e1143_d_b7, eq4_e1143_d_b8,) = {
    if (var_guard1825 != 0.0) {
        let eq4_e1141: f64 = ((nv0 - nv7) / var_rd);
        let eq4_e1141_d_n0: f64 = ((var_rd - ((nv0 - nv7) * var_rd_dn0)) / (var_rd * var_rd));
        let eq4_e1141_d_n1: f64 = (-(((nv0 - nv7) * var_rd_dn1) / (var_rd * var_rd)));
        let eq4_e1141_d_n2: f64 = (-(((nv0 - nv7) * var_rd_dn2) / (var_rd * var_rd)));
        let eq4_e1141_d_n3: f64 = (-(((nv0 - nv7) * var_rd_dn3) / (var_rd * var_rd)));
        let eq4_e1141_d_n4: f64 = (-(((nv0 - nv7) * var_rd_dn4) / (var_rd * var_rd)));
        let eq4_e1141_d_n5: f64 = (-(((nv0 - nv7) * var_rd_dn5) / (var_rd * var_rd)));
        let eq4_e1141_d_n6: f64 = (-(((nv0 - nv7) * var_rd_dn6) / (var_rd * var_rd)));
        let eq4_e1141_d_n7: f64 = (((-var_rd) - ((nv0 - nv7) * var_rd_dn7)) / (var_rd * var_rd));
        let eq4_e1141_d_n8: f64 = (-(((nv0 - nv7) * var_rd_dn8) / (var_rd * var_rd)));
        let eq4_e1141_d_n9: f64 = (-(((nv0 - nv7) * var_rd_dn9) / (var_rd * var_rd)));
        let eq4_e1141_d_n10: f64 = (-(((nv0 - nv7) * var_rd_dn10) / (var_rd * var_rd)));
        let eq4_e1141_d_n11: f64 = (-(((nv0 - nv7) * var_rd_dn11) / (var_rd * var_rd)));
        let eq4_e1141_d_n12: f64 = (-(((nv0 - nv7) * var_rd_dn12) / (var_rd * var_rd)));
        let eq4_e1141_d_b0: f64 = (-(((nv0 - nv7) * var_rd_db0) / (var_rd * var_rd)));
        let eq4_e1141_d_b1: f64 = (-(((nv0 - nv7) * var_rd_db1) / (var_rd * var_rd)));
        let eq4_e1141_d_b2: f64 = (-(((nv0 - nv7) * var_rd_db2) / (var_rd * var_rd)));
        let eq4_e1141_d_b3: f64 = (-(((nv0 - nv7) * var_rd_db3) / (var_rd * var_rd)));
        let eq4_e1141_d_b4: f64 = (-(((nv0 - nv7) * var_rd_db4) / (var_rd * var_rd)));
        let eq4_e1141_d_b5: f64 = (-(((nv0 - nv7) * var_rd_db5) / (var_rd * var_rd)));
        let eq4_e1141_d_b6: f64 = (-(((nv0 - nv7) * var_rd_db6) / (var_rd * var_rd)));
        let eq4_e1141_d_b7: f64 = (-(((nv0 - nv7) * var_rd_db7) / (var_rd * var_rd)));
        let eq4_e1141_d_b8: f64 = (-(((nv0 - nv7) * var_rd_db8) / (var_rd * var_rd)));
        (eq4_e1141, eq4_e1141_d_n0, eq4_e1141_d_n1, eq4_e1141_d_n2, eq4_e1141_d_n3, eq4_e1141_d_n4, eq4_e1141_d_n5, eq4_e1141_d_n6, eq4_e1141_d_n7, eq4_e1141_d_n8, eq4_e1141_d_n9, eq4_e1141_d_n10, eq4_e1141_d_n11, eq4_e1141_d_n12, eq4_e1141_d_b0, eq4_e1141_d_b1, eq4_e1141_d_b2, eq4_e1141_d_b3, eq4_e1141_d_b4, eq4_e1141_d_b5, eq4_e1141_d_b6, eq4_e1141_d_b7, eq4_e1141_d_b8,)
    } else {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    }
};
        let eq4_value: f64 = eq4_e1143;
        let eq4_node_derivatives: [f64; 13] = [eq4_e1143_d_n0, eq4_e1143_d_n1, eq4_e1143_d_n2, eq4_e1143_d_n3, eq4_e1143_d_n4, eq4_e1143_d_n5, eq4_e1143_d_n6, eq4_e1143_d_n7, eq4_e1143_d_n8, eq4_e1143_d_n9, eq4_e1143_d_n10, eq4_e1143_d_n11, eq4_e1143_d_n12];
        let eq4_branch_derivatives: [f64; 9] = [eq4_e1143_d_b0, eq4_e1143_d_b1, eq4_e1143_d_b2, eq4_e1143_d_b3, eq4_e1143_d_b4, eq4_e1143_d_b5, eq4_e1143_d_b6, eq4_e1143_d_b7, eq4_e1143_d_b8];
        stamper.stamp_current_dense_local(
            Some(0),
            Some(7),
            multiplicity * (eq4_value),
            &eq4_node_derivatives,
            &eq4_branch_derivatives,
            multiplicity,
        );
        let (eq6_e1158, eq6_e1158_d_n0, eq6_e1158_d_n1, eq6_e1158_d_n2, eq6_e1158_d_n3, eq6_e1158_d_n4, eq6_e1158_d_n5, eq6_e1158_d_n6, eq6_e1158_d_n7, eq6_e1158_d_n8, eq6_e1158_d_n9, eq6_e1158_d_n10, eq6_e1158_d_n11, eq6_e1158_d_n12, eq6_e1158_d_b0, eq6_e1158_d_b1, eq6_e1158_d_b2, eq6_e1158_d_b3, eq6_e1158_d_b4, eq6_e1158_d_b5, eq6_e1158_d_b6, eq6_e1158_d_b7, eq6_e1158_d_b8,) = {
    if (var_guard1825 != 0.0) {
        let eq6_e1156: f64 = ((nv2 - nv8) / var_rs);
        let eq6_e1156_d_n0: f64 = (-(((nv2 - nv8) * var_rs_dn0) / (var_rs * var_rs)));
        let eq6_e1156_d_n1: f64 = (-(((nv2 - nv8) * var_rs_dn1) / (var_rs * var_rs)));
        let eq6_e1156_d_n2: f64 = ((var_rs - ((nv2 - nv8) * var_rs_dn2)) / (var_rs * var_rs));
        let eq6_e1156_d_n3: f64 = (-(((nv2 - nv8) * var_rs_dn3) / (var_rs * var_rs)));
        let eq6_e1156_d_n4: f64 = (-(((nv2 - nv8) * var_rs_dn4) / (var_rs * var_rs)));
        let eq6_e1156_d_n5: f64 = (-(((nv2 - nv8) * var_rs_dn5) / (var_rs * var_rs)));
        let eq6_e1156_d_n6: f64 = (-(((nv2 - nv8) * var_rs_dn6) / (var_rs * var_rs)));
        let eq6_e1156_d_n7: f64 = (-(((nv2 - nv8) * var_rs_dn7) / (var_rs * var_rs)));
        let eq6_e1156_d_n8: f64 = (((-var_rs) - ((nv2 - nv8) * var_rs_dn8)) / (var_rs * var_rs));
        let eq6_e1156_d_n9: f64 = (-(((nv2 - nv8) * var_rs_dn9) / (var_rs * var_rs)));
        let eq6_e1156_d_n10: f64 = (-(((nv2 - nv8) * var_rs_dn10) / (var_rs * var_rs)));
        let eq6_e1156_d_n11: f64 = (-(((nv2 - nv8) * var_rs_dn11) / (var_rs * var_rs)));
        let eq6_e1156_d_n12: f64 = (-(((nv2 - nv8) * var_rs_dn12) / (var_rs * var_rs)));
        let eq6_e1156_d_b0: f64 = (-(((nv2 - nv8) * var_rs_db0) / (var_rs * var_rs)));
        let eq6_e1156_d_b1: f64 = (-(((nv2 - nv8) * var_rs_db1) / (var_rs * var_rs)));
        let eq6_e1156_d_b2: f64 = (-(((nv2 - nv8) * var_rs_db2) / (var_rs * var_rs)));
        let eq6_e1156_d_b3: f64 = (-(((nv2 - nv8) * var_rs_db3) / (var_rs * var_rs)));
        let eq6_e1156_d_b4: f64 = (-(((nv2 - nv8) * var_rs_db4) / (var_rs * var_rs)));
        let eq6_e1156_d_b5: f64 = (-(((nv2 - nv8) * var_rs_db5) / (var_rs * var_rs)));
        let eq6_e1156_d_b6: f64 = (-(((nv2 - nv8) * var_rs_db6) / (var_rs * var_rs)));
        let eq6_e1156_d_b7: f64 = (-(((nv2 - nv8) * var_rs_db7) / (var_rs * var_rs)));
        let eq6_e1156_d_b8: f64 = (-(((nv2 - nv8) * var_rs_db8) / (var_rs * var_rs)));
        (eq6_e1156, eq6_e1156_d_n0, eq6_e1156_d_n1, eq6_e1156_d_n2, eq6_e1156_d_n3, eq6_e1156_d_n4, eq6_e1156_d_n5, eq6_e1156_d_n6, eq6_e1156_d_n7, eq6_e1156_d_n8, eq6_e1156_d_n9, eq6_e1156_d_n10, eq6_e1156_d_n11, eq6_e1156_d_n12, eq6_e1156_d_b0, eq6_e1156_d_b1, eq6_e1156_d_b2, eq6_e1156_d_b3, eq6_e1156_d_b4, eq6_e1156_d_b5, eq6_e1156_d_b6, eq6_e1156_d_b7, eq6_e1156_d_b8,)
    } else {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    }
};
        let eq6_value: f64 = eq6_e1158;
        let eq6_node_derivatives: [f64; 13] = [eq6_e1158_d_n0, eq6_e1158_d_n1, eq6_e1158_d_n2, eq6_e1158_d_n3, eq6_e1158_d_n4, eq6_e1158_d_n5, eq6_e1158_d_n6, eq6_e1158_d_n7, eq6_e1158_d_n8, eq6_e1158_d_n9, eq6_e1158_d_n10, eq6_e1158_d_n11, eq6_e1158_d_n12];
        let eq6_branch_derivatives: [f64; 9] = [eq6_e1158_d_b0, eq6_e1158_d_b1, eq6_e1158_d_b2, eq6_e1158_d_b3, eq6_e1158_d_b4, eq6_e1158_d_b5, eq6_e1158_d_b6, eq6_e1158_d_b7, eq6_e1158_d_b8];
        stamper.stamp_current_dense_local(
            Some(2),
            Some(8),
            multiplicity * (eq6_value),
            &eq6_node_derivatives,
            &eq6_branch_derivatives,
            multiplicity,
        );
        let (eq10_e1185, eq10_e1185_d_n0, eq10_e1185_d_n1, eq10_e1185_d_n2, eq10_e1185_d_n3, eq10_e1185_d_n4, eq10_e1185_d_n5, eq10_e1185_d_n6, eq10_e1185_d_n7, eq10_e1185_d_n8, eq10_e1185_d_n9, eq10_e1185_d_n10, eq10_e1185_d_n11, eq10_e1185_d_n12, eq10_e1185_d_b0, eq10_e1185_d_b1, eq10_e1185_d_b2, eq10_e1185_d_b3, eq10_e1185_d_b4, eq10_e1185_d_b5, eq10_e1185_d_b6, eq10_e1185_d_b7, eq10_e1185_d_b8,) = {
    if (var_guard1826 != 0.0) {
        let eq10_e1182: f64 = (var_ids_1 + var_ic_1);
        let eq10_e1182_d_n0: f64 = (var_ids_1_dn0 + var_ic_1_dn0);
        let eq10_e1182_d_n1: f64 = (var_ids_1_dn1 + var_ic_1_dn1);
        let eq10_e1182_d_n2: f64 = (var_ids_1_dn2 + var_ic_1_dn2);
        let eq10_e1182_d_n3: f64 = (var_ids_1_dn3 + var_ic_1_dn3);
        let eq10_e1182_d_n4: f64 = (var_ids_1_dn4 + var_ic_1_dn4);
        let eq10_e1182_d_n5: f64 = (var_ids_1_dn5 + var_ic_1_dn5);
        let eq10_e1182_d_n6: f64 = (var_ids_1_dn6 + var_ic_1_dn6);
        let eq10_e1182_d_n7: f64 = (var_ids_1_dn7 + var_ic_1_dn7);
        let eq10_e1182_d_n8: f64 = (var_ids_1_dn8 + var_ic_1_dn8);
        let eq10_e1182_d_n9: f64 = (var_ids_1_dn9 + var_ic_1_dn9);
        let eq10_e1182_d_n10: f64 = (var_ids_1_dn10 + var_ic_1_dn10);
        let eq10_e1182_d_n11: f64 = (var_ids_1_dn11 + var_ic_1_dn11);
        let eq10_e1182_d_n12: f64 = (var_ids_1_dn12 + var_ic_1_dn12);
        let eq10_e1182_d_b0: f64 = (var_ids_1_db0 + var_ic_1_db0);
        let eq10_e1182_d_b1: f64 = (var_ids_1_db1 + var_ic_1_db1);
        let eq10_e1182_d_b2: f64 = (var_ids_1_db2 + var_ic_1_db2);
        let eq10_e1182_d_b3: f64 = (var_ids_1_db3 + var_ic_1_db3);
        let eq10_e1182_d_b4: f64 = (var_ids_1_db4 + var_ic_1_db4);
        let eq10_e1182_d_b5: f64 = (var_ids_1_db5 + var_ic_1_db5);
        let eq10_e1182_d_b6: f64 = (var_ids_1_db6 + var_ic_1_db6);
        let eq10_e1182_d_b7: f64 = (var_ids_1_db7 + var_ic_1_db7);
        let eq10_e1182_d_b8: f64 = (var_ids_1_db8 + var_ic_1_db8);
        let eq10_e1183: f64 = (var_b4soitype * eq10_e1182);
        let eq10_e1183_d_n0: f64 = (var_b4soitype * eq10_e1182_d_n0);
        let eq10_e1183_d_n1: f64 = (var_b4soitype * eq10_e1182_d_n1);
        let eq10_e1183_d_n2: f64 = (var_b4soitype * eq10_e1182_d_n2);
        let eq10_e1183_d_n3: f64 = (var_b4soitype * eq10_e1182_d_n3);
        let eq10_e1183_d_n4: f64 = (var_b4soitype * eq10_e1182_d_n4);
        let eq10_e1183_d_n5: f64 = (var_b4soitype * eq10_e1182_d_n5);
        let eq10_e1183_d_n6: f64 = (var_b4soitype * eq10_e1182_d_n6);
        let eq10_e1183_d_n7: f64 = (var_b4soitype * eq10_e1182_d_n7);
        let eq10_e1183_d_n8: f64 = (var_b4soitype * eq10_e1182_d_n8);
        let eq10_e1183_d_n9: f64 = (var_b4soitype * eq10_e1182_d_n9);
        let eq10_e1183_d_n10: f64 = (var_b4soitype * eq10_e1182_d_n10);
        let eq10_e1183_d_n11: f64 = (var_b4soitype * eq10_e1182_d_n11);
        let eq10_e1183_d_n12: f64 = (var_b4soitype * eq10_e1182_d_n12);
        let eq10_e1183_d_b0: f64 = (var_b4soitype * eq10_e1182_d_b0);
        let eq10_e1183_d_b1: f64 = (var_b4soitype * eq10_e1182_d_b1);
        let eq10_e1183_d_b2: f64 = (var_b4soitype * eq10_e1182_d_b2);
        let eq10_e1183_d_b3: f64 = (var_b4soitype * eq10_e1182_d_b3);
        let eq10_e1183_d_b4: f64 = (var_b4soitype * eq10_e1182_d_b4);
        let eq10_e1183_d_b5: f64 = (var_b4soitype * eq10_e1182_d_b5);
        let eq10_e1183_d_b6: f64 = (var_b4soitype * eq10_e1182_d_b6);
        let eq10_e1183_d_b7: f64 = (var_b4soitype * eq10_e1182_d_b7);
        let eq10_e1183_d_b8: f64 = (var_b4soitype * eq10_e1182_d_b8);
        (eq10_e1183, eq10_e1183_d_n0, eq10_e1183_d_n1, eq10_e1183_d_n2, eq10_e1183_d_n3, eq10_e1183_d_n4, eq10_e1183_d_n5, eq10_e1183_d_n6, eq10_e1183_d_n7, eq10_e1183_d_n8, eq10_e1183_d_n9, eq10_e1183_d_n10, eq10_e1183_d_n11, eq10_e1183_d_n12, eq10_e1183_d_b0, eq10_e1183_d_b1, eq10_e1183_d_b2, eq10_e1183_d_b3, eq10_e1183_d_b4, eq10_e1183_d_b5, eq10_e1183_d_b6, eq10_e1183_d_b7, eq10_e1183_d_b8,)
    } else {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    }
};
        let eq10_value: f64 = eq10_e1185;
        let eq10_node_derivatives: [f64; 13] = [eq10_e1185_d_n0, eq10_e1185_d_n1, eq10_e1185_d_n2, eq10_e1185_d_n3, eq10_e1185_d_n4, eq10_e1185_d_n5, eq10_e1185_d_n6, eq10_e1185_d_n7, eq10_e1185_d_n8, eq10_e1185_d_n9, eq10_e1185_d_n10, eq10_e1185_d_n11, eq10_e1185_d_n12];
        let eq10_branch_derivatives: [f64; 9] = [eq10_e1185_d_b0, eq10_e1185_d_b1, eq10_e1185_d_b2, eq10_e1185_d_b3, eq10_e1185_d_b4, eq10_e1185_d_b5, eq10_e1185_d_b6, eq10_e1185_d_b7, eq10_e1185_d_b8];
        stamper.stamp_current_dense_local(
            Some(7),
            Some(8),
            multiplicity * (eq10_value),
            &eq10_node_derivatives,
            &eq10_branch_derivatives,
            multiplicity,
        );
        let (eq11_e1191, eq11_e1191_d_n0, eq11_e1191_d_n1, eq11_e1191_d_n2, eq11_e1191_d_n3, eq11_e1191_d_n4, eq11_e1191_d_n5, eq11_e1191_d_n6, eq11_e1191_d_n7, eq11_e1191_d_n8, eq11_e1191_d_n9, eq11_e1191_d_n10, eq11_e1191_d_n11, eq11_e1191_d_n12, eq11_e1191_d_b0, eq11_e1191_d_b1, eq11_e1191_d_b2, eq11_e1191_d_b3, eq11_e1191_d_b4, eq11_e1191_d_b5, eq11_e1191_d_b6, eq11_e1191_d_b7, eq11_e1191_d_b8,) = {
    if (var_guard1826 != 0.0) {
        let eq11_e1189: f64 = (var_b4soitype * var_iii);
        let eq11_e1189_d_n0: f64 = (var_b4soitype * var_iii_dn0);
        let eq11_e1189_d_n1: f64 = (var_b4soitype * var_iii_dn1);
        let eq11_e1189_d_n2: f64 = (var_b4soitype * var_iii_dn2);
        let eq11_e1189_d_n3: f64 = (var_b4soitype * var_iii_dn3);
        let eq11_e1189_d_n4: f64 = (var_b4soitype * var_iii_dn4);
        let eq11_e1189_d_n5: f64 = (var_b4soitype * var_iii_dn5);
        let eq11_e1189_d_n6: f64 = (var_b4soitype * var_iii_dn6);
        let eq11_e1189_d_n7: f64 = (var_b4soitype * var_iii_dn7);
        let eq11_e1189_d_n8: f64 = (var_b4soitype * var_iii_dn8);
        let eq11_e1189_d_n9: f64 = (var_b4soitype * var_iii_dn9);
        let eq11_e1189_d_n10: f64 = (var_b4soitype * var_iii_dn10);
        let eq11_e1189_d_n11: f64 = (var_b4soitype * var_iii_dn11);
        let eq11_e1189_d_n12: f64 = (var_b4soitype * var_iii_dn12);
        let eq11_e1189_d_b0: f64 = (var_b4soitype * var_iii_db0);
        let eq11_e1189_d_b1: f64 = (var_b4soitype * var_iii_db1);
        let eq11_e1189_d_b2: f64 = (var_b4soitype * var_iii_db2);
        let eq11_e1189_d_b3: f64 = (var_b4soitype * var_iii_db3);
        let eq11_e1189_d_b4: f64 = (var_b4soitype * var_iii_db4);
        let eq11_e1189_d_b5: f64 = (var_b4soitype * var_iii_db5);
        let eq11_e1189_d_b6: f64 = (var_b4soitype * var_iii_db6);
        let eq11_e1189_d_b7: f64 = (var_b4soitype * var_iii_db7);
        let eq11_e1189_d_b8: f64 = (var_b4soitype * var_iii_db8);
        (eq11_e1189, eq11_e1189_d_n0, eq11_e1189_d_n1, eq11_e1189_d_n2, eq11_e1189_d_n3, eq11_e1189_d_n4, eq11_e1189_d_n5, eq11_e1189_d_n6, eq11_e1189_d_n7, eq11_e1189_d_n8, eq11_e1189_d_n9, eq11_e1189_d_n10, eq11_e1189_d_n11, eq11_e1189_d_n12, eq11_e1189_d_b0, eq11_e1189_d_b1, eq11_e1189_d_b2, eq11_e1189_d_b3, eq11_e1189_d_b4, eq11_e1189_d_b5, eq11_e1189_d_b6, eq11_e1189_d_b7, eq11_e1189_d_b8,)
    } else {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    }
};
        let eq11_value: f64 = eq11_e1191;
        let eq11_node_derivatives: [f64; 13] = [eq11_e1191_d_n0, eq11_e1191_d_n1, eq11_e1191_d_n2, eq11_e1191_d_n3, eq11_e1191_d_n4, eq11_e1191_d_n5, eq11_e1191_d_n6, eq11_e1191_d_n7, eq11_e1191_d_n8, eq11_e1191_d_n9, eq11_e1191_d_n10, eq11_e1191_d_n11, eq11_e1191_d_n12];
        let eq11_branch_derivatives: [f64; 9] = [eq11_e1191_d_b0, eq11_e1191_d_b1, eq11_e1191_d_b2, eq11_e1191_d_b3, eq11_e1191_d_b4, eq11_e1191_d_b5, eq11_e1191_d_b6, eq11_e1191_d_b7, eq11_e1191_d_b8];
        stamper.stamp_current_dense_local(
            Some(7),
            Some(5),
            multiplicity * (eq11_value),
            &eq11_node_derivatives,
            &eq11_branch_derivatives,
            multiplicity,
        );
        let (eq12_e1200, eq12_e1200_d_n0, eq12_e1200_d_n1, eq12_e1200_d_n2, eq12_e1200_d_n3, eq12_e1200_d_n4, eq12_e1200_d_n5, eq12_e1200_d_n6, eq12_e1200_d_n7, eq12_e1200_d_n8, eq12_e1200_d_n9, eq12_e1200_d_n10, eq12_e1200_d_n11, eq12_e1200_d_n12, eq12_e1200_d_b0, eq12_e1200_d_b1, eq12_e1200_d_b2, eq12_e1200_d_b3, eq12_e1200_d_b4, eq12_e1200_d_b5, eq12_e1200_d_b6, eq12_e1200_d_b7, eq12_e1200_d_b8,) = {
    if (var_guard1826 == 0.0) {
        let eq12_e1197: f64 = (var_ids_1 - var_ic_1);
        let eq12_e1197_d_n0: f64 = (var_ids_1_dn0 - var_ic_1_dn0);
        let eq12_e1197_d_n1: f64 = (var_ids_1_dn1 - var_ic_1_dn1);
        let eq12_e1197_d_n2: f64 = (var_ids_1_dn2 - var_ic_1_dn2);
        let eq12_e1197_d_n3: f64 = (var_ids_1_dn3 - var_ic_1_dn3);
        let eq12_e1197_d_n4: f64 = (var_ids_1_dn4 - var_ic_1_dn4);
        let eq12_e1197_d_n5: f64 = (var_ids_1_dn5 - var_ic_1_dn5);
        let eq12_e1197_d_n6: f64 = (var_ids_1_dn6 - var_ic_1_dn6);
        let eq12_e1197_d_n7: f64 = (var_ids_1_dn7 - var_ic_1_dn7);
        let eq12_e1197_d_n8: f64 = (var_ids_1_dn8 - var_ic_1_dn8);
        let eq12_e1197_d_n9: f64 = (var_ids_1_dn9 - var_ic_1_dn9);
        let eq12_e1197_d_n10: f64 = (var_ids_1_dn10 - var_ic_1_dn10);
        let eq12_e1197_d_n11: f64 = (var_ids_1_dn11 - var_ic_1_dn11);
        let eq12_e1197_d_n12: f64 = (var_ids_1_dn12 - var_ic_1_dn12);
        let eq12_e1197_d_b0: f64 = (var_ids_1_db0 - var_ic_1_db0);
        let eq12_e1197_d_b1: f64 = (var_ids_1_db1 - var_ic_1_db1);
        let eq12_e1197_d_b2: f64 = (var_ids_1_db2 - var_ic_1_db2);
        let eq12_e1197_d_b3: f64 = (var_ids_1_db3 - var_ic_1_db3);
        let eq12_e1197_d_b4: f64 = (var_ids_1_db4 - var_ic_1_db4);
        let eq12_e1197_d_b5: f64 = (var_ids_1_db5 - var_ic_1_db5);
        let eq12_e1197_d_b6: f64 = (var_ids_1_db6 - var_ic_1_db6);
        let eq12_e1197_d_b7: f64 = (var_ids_1_db7 - var_ic_1_db7);
        let eq12_e1197_d_b8: f64 = (var_ids_1_db8 - var_ic_1_db8);
        let eq12_e1198: f64 = (var_b4soitype * eq12_e1197);
        let eq12_e1198_d_n0: f64 = (var_b4soitype * eq12_e1197_d_n0);
        let eq12_e1198_d_n1: f64 = (var_b4soitype * eq12_e1197_d_n1);
        let eq12_e1198_d_n2: f64 = (var_b4soitype * eq12_e1197_d_n2);
        let eq12_e1198_d_n3: f64 = (var_b4soitype * eq12_e1197_d_n3);
        let eq12_e1198_d_n4: f64 = (var_b4soitype * eq12_e1197_d_n4);
        let eq12_e1198_d_n5: f64 = (var_b4soitype * eq12_e1197_d_n5);
        let eq12_e1198_d_n6: f64 = (var_b4soitype * eq12_e1197_d_n6);
        let eq12_e1198_d_n7: f64 = (var_b4soitype * eq12_e1197_d_n7);
        let eq12_e1198_d_n8: f64 = (var_b4soitype * eq12_e1197_d_n8);
        let eq12_e1198_d_n9: f64 = (var_b4soitype * eq12_e1197_d_n9);
        let eq12_e1198_d_n10: f64 = (var_b4soitype * eq12_e1197_d_n10);
        let eq12_e1198_d_n11: f64 = (var_b4soitype * eq12_e1197_d_n11);
        let eq12_e1198_d_n12: f64 = (var_b4soitype * eq12_e1197_d_n12);
        let eq12_e1198_d_b0: f64 = (var_b4soitype * eq12_e1197_d_b0);
        let eq12_e1198_d_b1: f64 = (var_b4soitype * eq12_e1197_d_b1);
        let eq12_e1198_d_b2: f64 = (var_b4soitype * eq12_e1197_d_b2);
        let eq12_e1198_d_b3: f64 = (var_b4soitype * eq12_e1197_d_b3);
        let eq12_e1198_d_b4: f64 = (var_b4soitype * eq12_e1197_d_b4);
        let eq12_e1198_d_b5: f64 = (var_b4soitype * eq12_e1197_d_b5);
        let eq12_e1198_d_b6: f64 = (var_b4soitype * eq12_e1197_d_b6);
        let eq12_e1198_d_b7: f64 = (var_b4soitype * eq12_e1197_d_b7);
        let eq12_e1198_d_b8: f64 = (var_b4soitype * eq12_e1197_d_b8);
        (eq12_e1198, eq12_e1198_d_n0, eq12_e1198_d_n1, eq12_e1198_d_n2, eq12_e1198_d_n3, eq12_e1198_d_n4, eq12_e1198_d_n5, eq12_e1198_d_n6, eq12_e1198_d_n7, eq12_e1198_d_n8, eq12_e1198_d_n9, eq12_e1198_d_n10, eq12_e1198_d_n11, eq12_e1198_d_n12, eq12_e1198_d_b0, eq12_e1198_d_b1, eq12_e1198_d_b2, eq12_e1198_d_b3, eq12_e1198_d_b4, eq12_e1198_d_b5, eq12_e1198_d_b6, eq12_e1198_d_b7, eq12_e1198_d_b8,)
    } else {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    }
};
        let eq12_value: f64 = eq12_e1200;
        let eq12_node_derivatives: [f64; 13] = [eq12_e1200_d_n0, eq12_e1200_d_n1, eq12_e1200_d_n2, eq12_e1200_d_n3, eq12_e1200_d_n4, eq12_e1200_d_n5, eq12_e1200_d_n6, eq12_e1200_d_n7, eq12_e1200_d_n8, eq12_e1200_d_n9, eq12_e1200_d_n10, eq12_e1200_d_n11, eq12_e1200_d_n12];
        let eq12_branch_derivatives: [f64; 9] = [eq12_e1200_d_b0, eq12_e1200_d_b1, eq12_e1200_d_b2, eq12_e1200_d_b3, eq12_e1200_d_b4, eq12_e1200_d_b5, eq12_e1200_d_b6, eq12_e1200_d_b7, eq12_e1200_d_b8];
        stamper.stamp_current_dense_local(
            Some(8),
            Some(7),
            multiplicity * (eq12_value),
            &eq12_node_derivatives,
            &eq12_branch_derivatives,
            multiplicity,
        );
        let (eq13_e1207, eq13_e1207_d_n0, eq13_e1207_d_n1, eq13_e1207_d_n2, eq13_e1207_d_n3, eq13_e1207_d_n4, eq13_e1207_d_n5, eq13_e1207_d_n6, eq13_e1207_d_n7, eq13_e1207_d_n8, eq13_e1207_d_n9, eq13_e1207_d_n10, eq13_e1207_d_n11, eq13_e1207_d_n12, eq13_e1207_d_b0, eq13_e1207_d_b1, eq13_e1207_d_b2, eq13_e1207_d_b3, eq13_e1207_d_b4, eq13_e1207_d_b5, eq13_e1207_d_b6, eq13_e1207_d_b7, eq13_e1207_d_b8,) = {
    if (var_guard1826 == 0.0) {
        let eq13_e1205: f64 = (var_b4soitype * var_iii);
        let eq13_e1205_d_n0: f64 = (var_b4soitype * var_iii_dn0);
        let eq13_e1205_d_n1: f64 = (var_b4soitype * var_iii_dn1);
        let eq13_e1205_d_n2: f64 = (var_b4soitype * var_iii_dn2);
        let eq13_e1205_d_n3: f64 = (var_b4soitype * var_iii_dn3);
        let eq13_e1205_d_n4: f64 = (var_b4soitype * var_iii_dn4);
        let eq13_e1205_d_n5: f64 = (var_b4soitype * var_iii_dn5);
        let eq13_e1205_d_n6: f64 = (var_b4soitype * var_iii_dn6);
        let eq13_e1205_d_n7: f64 = (var_b4soitype * var_iii_dn7);
        let eq13_e1205_d_n8: f64 = (var_b4soitype * var_iii_dn8);
        let eq13_e1205_d_n9: f64 = (var_b4soitype * var_iii_dn9);
        let eq13_e1205_d_n10: f64 = (var_b4soitype * var_iii_dn10);
        let eq13_e1205_d_n11: f64 = (var_b4soitype * var_iii_dn11);
        let eq13_e1205_d_n12: f64 = (var_b4soitype * var_iii_dn12);
        let eq13_e1205_d_b0: f64 = (var_b4soitype * var_iii_db0);
        let eq13_e1205_d_b1: f64 = (var_b4soitype * var_iii_db1);
        let eq13_e1205_d_b2: f64 = (var_b4soitype * var_iii_db2);
        let eq13_e1205_d_b3: f64 = (var_b4soitype * var_iii_db3);
        let eq13_e1205_d_b4: f64 = (var_b4soitype * var_iii_db4);
        let eq13_e1205_d_b5: f64 = (var_b4soitype * var_iii_db5);
        let eq13_e1205_d_b6: f64 = (var_b4soitype * var_iii_db6);
        let eq13_e1205_d_b7: f64 = (var_b4soitype * var_iii_db7);
        let eq13_e1205_d_b8: f64 = (var_b4soitype * var_iii_db8);
        (eq13_e1205, eq13_e1205_d_n0, eq13_e1205_d_n1, eq13_e1205_d_n2, eq13_e1205_d_n3, eq13_e1205_d_n4, eq13_e1205_d_n5, eq13_e1205_d_n6, eq13_e1205_d_n7, eq13_e1205_d_n8, eq13_e1205_d_n9, eq13_e1205_d_n10, eq13_e1205_d_n11, eq13_e1205_d_n12, eq13_e1205_d_b0, eq13_e1205_d_b1, eq13_e1205_d_b2, eq13_e1205_d_b3, eq13_e1205_d_b4, eq13_e1205_d_b5, eq13_e1205_d_b6, eq13_e1205_d_b7, eq13_e1205_d_b8,)
    } else {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    }
};
        let eq13_value: f64 = eq13_e1207;
        let eq13_node_derivatives: [f64; 13] = [eq13_e1207_d_n0, eq13_e1207_d_n1, eq13_e1207_d_n2, eq13_e1207_d_n3, eq13_e1207_d_n4, eq13_e1207_d_n5, eq13_e1207_d_n6, eq13_e1207_d_n7, eq13_e1207_d_n8, eq13_e1207_d_n9, eq13_e1207_d_n10, eq13_e1207_d_n11, eq13_e1207_d_n12];
        let eq13_branch_derivatives: [f64; 9] = [eq13_e1207_d_b0, eq13_e1207_d_b1, eq13_e1207_d_b2, eq13_e1207_d_b3, eq13_e1207_d_b4, eq13_e1207_d_b5, eq13_e1207_d_b6, eq13_e1207_d_b7, eq13_e1207_d_b8];
        stamper.stamp_current_dense_local(
            Some(8),
            Some(5),
            multiplicity * (eq13_value),
            &eq13_node_derivatives,
            &eq13_branch_derivatives,
            multiplicity,
        );
        let eq14_value: f64 = var_b4soiigidl;
        let eq14_node_derivatives: [f64; 13] = [var_b4soiigidl_dn0, var_b4soiigidl_dn1, var_b4soiigidl_dn2, var_b4soiigidl_dn3, var_b4soiigidl_dn4, var_b4soiigidl_dn5, var_b4soiigidl_dn6, var_b4soiigidl_dn7, var_b4soiigidl_dn8, var_b4soiigidl_dn9, var_b4soiigidl_dn10, var_b4soiigidl_dn11, var_b4soiigidl_dn12];
        let eq14_branch_derivatives: [f64; 9] = [var_b4soiigidl_db0, var_b4soiigidl_db1, var_b4soiigidl_db2, var_b4soiigidl_db3, var_b4soiigidl_db4, var_b4soiigidl_db5, var_b4soiigidl_db6, var_b4soiigidl_db7, var_b4soiigidl_db8];
        stamper.stamp_current_dense_local(
            Some(7),
            Some(5),
            multiplicity * (eq14_value),
            &eq14_node_derivatives,
            &eq14_branch_derivatives,
            multiplicity,
        );
        let eq15_value: f64 = var_b4soiigisl;
        let eq15_node_derivatives: [f64; 13] = [var_b4soiigisl_dn0, var_b4soiigisl_dn1, var_b4soiigisl_dn2, var_b4soiigisl_dn3, var_b4soiigisl_dn4, var_b4soiigisl_dn5, var_b4soiigisl_dn6, var_b4soiigisl_dn7, var_b4soiigisl_dn8, var_b4soiigisl_dn9, var_b4soiigisl_dn10, var_b4soiigisl_dn11, var_b4soiigisl_dn12];
        let eq15_branch_derivatives: [f64; 9] = [var_b4soiigisl_db0, var_b4soiigisl_db1, var_b4soiigisl_db2, var_b4soiigisl_db3, var_b4soiigisl_db4, var_b4soiigisl_db5, var_b4soiigisl_db6, var_b4soiigisl_db7, var_b4soiigisl_db8];
        stamper.stamp_current_dense_local(
            Some(8),
            Some(5),
            multiplicity * (eq15_value),
            &eq15_node_derivatives,
            &eq15_branch_derivatives,
            multiplicity,
        );
        let eq16_e1212: f64 = (var_b4soitype * var_ibd_1);
        let eq16_e1212_d_n0: f64 = (var_b4soitype * var_ibd_1_dn0);
        let eq16_e1212_d_n1: f64 = (var_b4soitype * var_ibd_1_dn1);
        let eq16_e1212_d_n2: f64 = (var_b4soitype * var_ibd_1_dn2);
        let eq16_e1212_d_n3: f64 = (var_b4soitype * var_ibd_1_dn3);
        let eq16_e1212_d_n4: f64 = (var_b4soitype * var_ibd_1_dn4);
        let eq16_e1212_d_n5: f64 = (var_b4soitype * var_ibd_1_dn5);
        let eq16_e1212_d_n6: f64 = (var_b4soitype * var_ibd_1_dn6);
        let eq16_e1212_d_n7: f64 = (var_b4soitype * var_ibd_1_dn7);
        let eq16_e1212_d_n8: f64 = (var_b4soitype * var_ibd_1_dn8);
        let eq16_e1212_d_n9: f64 = (var_b4soitype * var_ibd_1_dn9);
        let eq16_e1212_d_n10: f64 = (var_b4soitype * var_ibd_1_dn10);
        let eq16_e1212_d_n11: f64 = (var_b4soitype * var_ibd_1_dn11);
        let eq16_e1212_d_n12: f64 = (var_b4soitype * var_ibd_1_dn12);
        let eq16_e1212_d_b0: f64 = (var_b4soitype * var_ibd_1_db0);
        let eq16_e1212_d_b1: f64 = (var_b4soitype * var_ibd_1_db1);
        let eq16_e1212_d_b2: f64 = (var_b4soitype * var_ibd_1_db2);
        let eq16_e1212_d_b3: f64 = (var_b4soitype * var_ibd_1_db3);
        let eq16_e1212_d_b4: f64 = (var_b4soitype * var_ibd_1_db4);
        let eq16_e1212_d_b5: f64 = (var_b4soitype * var_ibd_1_db5);
        let eq16_e1212_d_b6: f64 = (var_b4soitype * var_ibd_1_db6);
        let eq16_e1212_d_b7: f64 = (var_b4soitype * var_ibd_1_db7);
        let eq16_e1212_d_b8: f64 = (var_b4soitype * var_ibd_1_db8);
        let eq16_value: f64 = eq16_e1212;
        let eq16_node_derivatives: [f64; 13] = [eq16_e1212_d_n0, eq16_e1212_d_n1, eq16_e1212_d_n2, eq16_e1212_d_n3, eq16_e1212_d_n4, eq16_e1212_d_n5, eq16_e1212_d_n6, eq16_e1212_d_n7, eq16_e1212_d_n8, eq16_e1212_d_n9, eq16_e1212_d_n10, eq16_e1212_d_n11, eq16_e1212_d_n12];
        let eq16_branch_derivatives: [f64; 9] = [eq16_e1212_d_b0, eq16_e1212_d_b1, eq16_e1212_d_b2, eq16_e1212_d_b3, eq16_e1212_d_b4, eq16_e1212_d_b5, eq16_e1212_d_b6, eq16_e1212_d_b7, eq16_e1212_d_b8];
        stamper.stamp_current_dense_local(
            Some(12),
            Some(7),
            multiplicity * (eq16_value),
            &eq16_node_derivatives,
            &eq16_branch_derivatives,
            multiplicity,
        );
        let eq17_e1215: f64 = (var_b4soitype * var_ibs_1);
        let eq17_e1215_d_n0: f64 = (var_b4soitype * var_ibs_1_dn0);
        let eq17_e1215_d_n1: f64 = (var_b4soitype * var_ibs_1_dn1);
        let eq17_e1215_d_n2: f64 = (var_b4soitype * var_ibs_1_dn2);
        let eq17_e1215_d_n3: f64 = (var_b4soitype * var_ibs_1_dn3);
        let eq17_e1215_d_n4: f64 = (var_b4soitype * var_ibs_1_dn4);
        let eq17_e1215_d_n5: f64 = (var_b4soitype * var_ibs_1_dn5);
        let eq17_e1215_d_n6: f64 = (var_b4soitype * var_ibs_1_dn6);
        let eq17_e1215_d_n7: f64 = (var_b4soitype * var_ibs_1_dn7);
        let eq17_e1215_d_n8: f64 = (var_b4soitype * var_ibs_1_dn8);
        let eq17_e1215_d_n9: f64 = (var_b4soitype * var_ibs_1_dn9);
        let eq17_e1215_d_n10: f64 = (var_b4soitype * var_ibs_1_dn10);
        let eq17_e1215_d_n11: f64 = (var_b4soitype * var_ibs_1_dn11);
        let eq17_e1215_d_n12: f64 = (var_b4soitype * var_ibs_1_dn12);
        let eq17_e1215_d_b0: f64 = (var_b4soitype * var_ibs_1_db0);
        let eq17_e1215_d_b1: f64 = (var_b4soitype * var_ibs_1_db1);
        let eq17_e1215_d_b2: f64 = (var_b4soitype * var_ibs_1_db2);
        let eq17_e1215_d_b3: f64 = (var_b4soitype * var_ibs_1_db3);
        let eq17_e1215_d_b4: f64 = (var_b4soitype * var_ibs_1_db4);
        let eq17_e1215_d_b5: f64 = (var_b4soitype * var_ibs_1_db5);
        let eq17_e1215_d_b6: f64 = (var_b4soitype * var_ibs_1_db6);
        let eq17_e1215_d_b7: f64 = (var_b4soitype * var_ibs_1_db7);
        let eq17_e1215_d_b8: f64 = (var_b4soitype * var_ibs_1_db8);
        let eq17_value: f64 = eq17_e1215;
        let eq17_node_derivatives: [f64; 13] = [eq17_e1215_d_n0, eq17_e1215_d_n1, eq17_e1215_d_n2, eq17_e1215_d_n3, eq17_e1215_d_n4, eq17_e1215_d_n5, eq17_e1215_d_n6, eq17_e1215_d_n7, eq17_e1215_d_n8, eq17_e1215_d_n9, eq17_e1215_d_n10, eq17_e1215_d_n11, eq17_e1215_d_n12];
        let eq17_branch_derivatives: [f64; 9] = [eq17_e1215_d_b0, eq17_e1215_d_b1, eq17_e1215_d_b2, eq17_e1215_d_b3, eq17_e1215_d_b4, eq17_e1215_d_b5, eq17_e1215_d_b6, eq17_e1215_d_b7, eq17_e1215_d_b8];
        stamper.stamp_current_dense_local(
            Some(11),
            Some(8),
            multiplicity * (eq17_value),
            &eq17_node_derivatives,
            &eq17_branch_derivatives,
            multiplicity,
        );
        let eq18_e1218: f64 = (var_b4soiigd + var_b4soiigcd);
        let eq18_e1218_d_n0: f64 = (var_b4soiigd_dn0 + var_b4soiigcd_dn0);
        let eq18_e1218_d_n1: f64 = (var_b4soiigd_dn1 + var_b4soiigcd_dn1);
        let eq18_e1218_d_n2: f64 = (var_b4soiigd_dn2 + var_b4soiigcd_dn2);
        let eq18_e1218_d_n3: f64 = (var_b4soiigd_dn3 + var_b4soiigcd_dn3);
        let eq18_e1218_d_n4: f64 = (var_b4soiigd_dn4 + var_b4soiigcd_dn4);
        let eq18_e1218_d_n5: f64 = (var_b4soiigd_dn5 + var_b4soiigcd_dn5);
        let eq18_e1218_d_n6: f64 = (var_b4soiigd_dn6 + var_b4soiigcd_dn6);
        let eq18_e1218_d_n7: f64 = (var_b4soiigd_dn7 + var_b4soiigcd_dn7);
        let eq18_e1218_d_n8: f64 = (var_b4soiigd_dn8 + var_b4soiigcd_dn8);
        let eq18_e1218_d_n9: f64 = (var_b4soiigd_dn9 + var_b4soiigcd_dn9);
        let eq18_e1218_d_n10: f64 = (var_b4soiigd_dn10 + var_b4soiigcd_dn10);
        let eq18_e1218_d_n11: f64 = (var_b4soiigd_dn11 + var_b4soiigcd_dn11);
        let eq18_e1218_d_n12: f64 = (var_b4soiigd_dn12 + var_b4soiigcd_dn12);
        let eq18_e1218_d_b0: f64 = (var_b4soiigd_db0 + var_b4soiigcd_db0);
        let eq18_e1218_d_b1: f64 = (var_b4soiigd_db1 + var_b4soiigcd_db1);
        let eq18_e1218_d_b2: f64 = (var_b4soiigd_db2 + var_b4soiigcd_db2);
        let eq18_e1218_d_b3: f64 = (var_b4soiigd_db3 + var_b4soiigcd_db3);
        let eq18_e1218_d_b4: f64 = (var_b4soiigd_db4 + var_b4soiigcd_db4);
        let eq18_e1218_d_b5: f64 = (var_b4soiigd_db5 + var_b4soiigcd_db5);
        let eq18_e1218_d_b6: f64 = (var_b4soiigd_db6 + var_b4soiigcd_db6);
        let eq18_e1218_d_b7: f64 = (var_b4soiigd_db7 + var_b4soiigcd_db7);
        let eq18_e1218_d_b8: f64 = (var_b4soiigd_db8 + var_b4soiigcd_db8);
        let eq18_value: f64 = eq18_e1218;
        let eq18_node_derivatives: [f64; 13] = [eq18_e1218_d_n0, eq18_e1218_d_n1, eq18_e1218_d_n2, eq18_e1218_d_n3, eq18_e1218_d_n4, eq18_e1218_d_n5, eq18_e1218_d_n6, eq18_e1218_d_n7, eq18_e1218_d_n8, eq18_e1218_d_n9, eq18_e1218_d_n10, eq18_e1218_d_n11, eq18_e1218_d_n12];
        let eq18_branch_derivatives: [f64; 9] = [eq18_e1218_d_b0, eq18_e1218_d_b1, eq18_e1218_d_b2, eq18_e1218_d_b3, eq18_e1218_d_b4, eq18_e1218_d_b5, eq18_e1218_d_b6, eq18_e1218_d_b7, eq18_e1218_d_b8];
        stamper.stamp_current_dense_local(
            Some(9),
            Some(7),
            multiplicity * (eq18_value),
            &eq18_node_derivatives,
            &eq18_branch_derivatives,
            multiplicity,
        );
        let eq19_e1221: f64 = (var_b4soiigs + var_b4soiigcs);
        let eq19_e1221_d_n0: f64 = (var_b4soiigs_dn0 + var_b4soiigcs_dn0);
        let eq19_e1221_d_n1: f64 = (var_b4soiigs_dn1 + var_b4soiigcs_dn1);
        let eq19_e1221_d_n2: f64 = (var_b4soiigs_dn2 + var_b4soiigcs_dn2);
        let eq19_e1221_d_n3: f64 = (var_b4soiigs_dn3 + var_b4soiigcs_dn3);
        let eq19_e1221_d_n4: f64 = (var_b4soiigs_dn4 + var_b4soiigcs_dn4);
        let eq19_e1221_d_n5: f64 = (var_b4soiigs_dn5 + var_b4soiigcs_dn5);
        let eq19_e1221_d_n6: f64 = (var_b4soiigs_dn6 + var_b4soiigcs_dn6);
        let eq19_e1221_d_n7: f64 = (var_b4soiigs_dn7 + var_b4soiigcs_dn7);
        let eq19_e1221_d_n8: f64 = (var_b4soiigs_dn8 + var_b4soiigcs_dn8);
        let eq19_e1221_d_n9: f64 = (var_b4soiigs_dn9 + var_b4soiigcs_dn9);
        let eq19_e1221_d_n10: f64 = (var_b4soiigs_dn10 + var_b4soiigcs_dn10);
        let eq19_e1221_d_n11: f64 = (var_b4soiigs_dn11 + var_b4soiigcs_dn11);
        let eq19_e1221_d_n12: f64 = (var_b4soiigs_dn12 + var_b4soiigcs_dn12);
        let eq19_e1221_d_b0: f64 = (var_b4soiigs_db0 + var_b4soiigcs_db0);
        let eq19_e1221_d_b1: f64 = (var_b4soiigs_db1 + var_b4soiigcs_db1);
        let eq19_e1221_d_b2: f64 = (var_b4soiigs_db2 + var_b4soiigcs_db2);
        let eq19_e1221_d_b3: f64 = (var_b4soiigs_db3 + var_b4soiigcs_db3);
        let eq19_e1221_d_b4: f64 = (var_b4soiigs_db4 + var_b4soiigcs_db4);
        let eq19_e1221_d_b5: f64 = (var_b4soiigs_db5 + var_b4soiigcs_db5);
        let eq19_e1221_d_b6: f64 = (var_b4soiigs_db6 + var_b4soiigcs_db6);
        let eq19_e1221_d_b7: f64 = (var_b4soiigs_db7 + var_b4soiigcs_db7);
        let eq19_e1221_d_b8: f64 = (var_b4soiigs_db8 + var_b4soiigcs_db8);
        let eq19_value: f64 = eq19_e1221;
        let eq19_node_derivatives: [f64; 13] = [eq19_e1221_d_n0, eq19_e1221_d_n1, eq19_e1221_d_n2, eq19_e1221_d_n3, eq19_e1221_d_n4, eq19_e1221_d_n5, eq19_e1221_d_n6, eq19_e1221_d_n7, eq19_e1221_d_n8, eq19_e1221_d_n9, eq19_e1221_d_n10, eq19_e1221_d_n11, eq19_e1221_d_n12];
        let eq19_branch_derivatives: [f64; 9] = [eq19_e1221_d_b0, eq19_e1221_d_b1, eq19_e1221_d_b2, eq19_e1221_d_b3, eq19_e1221_d_b4, eq19_e1221_d_b5, eq19_e1221_d_b6, eq19_e1221_d_b7, eq19_e1221_d_b8];
        stamper.stamp_current_dense_local(
            Some(9),
            Some(8),
            multiplicity * (eq19_value),
            &eq19_node_derivatives,
            &eq19_branch_derivatives,
            multiplicity,
        );
        let eq20_value: f64 = var_b4soiig;
        let eq20_node_derivatives: [f64; 13] = [var_b4soiig_dn0, var_b4soiig_dn1, var_b4soiig_dn2, var_b4soiig_dn3, var_b4soiig_dn4, var_b4soiig_dn5, var_b4soiig_dn6, var_b4soiig_dn7, var_b4soiig_dn8, var_b4soiig_dn9, var_b4soiig_dn10, var_b4soiig_dn11, var_b4soiig_dn12];
        let eq20_branch_derivatives: [f64; 9] = [var_b4soiig_db0, var_b4soiig_db1, var_b4soiig_db2, var_b4soiig_db3, var_b4soiig_db4, var_b4soiig_db5, var_b4soiig_db6, var_b4soiig_db7, var_b4soiig_db8];
        stamper.stamp_current_dense_local(
            Some(9),
            Some(5),
            multiplicity * (eq20_value),
            &eq20_node_derivatives,
            &eq20_branch_derivatives,
            multiplicity,
        );
        let eq21_value: f64 = var_b4soiigp;
        let eq21_node_derivatives: [f64; 13] = [var_b4soiigp_dn0, var_b4soiigp_dn1, var_b4soiigp_dn2, var_b4soiigp_dn3, var_b4soiigp_dn4, var_b4soiigp_dn5, var_b4soiigp_dn6, var_b4soiigp_dn7, var_b4soiigp_dn8, var_b4soiigp_dn9, var_b4soiigp_dn10, var_b4soiigp_dn11, var_b4soiigp_dn12];
        let eq21_branch_derivatives: [f64; 9] = [var_b4soiigp_db0, var_b4soiigp_db1, var_b4soiigp_db2, var_b4soiigp_db3, var_b4soiigp_db4, var_b4soiigp_db5, var_b4soiigp_db6, var_b4soiigp_db7, var_b4soiigp_db8];
        stamper.stamp_current_dense_local(
            Some(9),
            Some(4),
            multiplicity * (eq21_value),
            &eq21_node_derivatives,
            &eq21_branch_derivatives,
            multiplicity,
        );
    }

    pub(super) fn stamp_transient_equations_block_1(
        ctx: &GeneratedEvalContext<'_>,
        stamper: &mut GeneratedStamper<'_>,
        s: &mut Scratch,
        nodes: &[usize; Instance::NODE_COUNT],
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
        var_b4soitype: f64,
        var_guard1827: f64,
        var_ibp: f64,
        var_ibp_db0: f64,
        var_ibp_db1: f64,
        var_ibp_db2: f64,
        var_ibp_db3: f64,
        var_ibp_db4: f64,
        var_ibp_db5: f64,
        var_ibp_db6: f64,
        var_ibp_db7: f64,
        var_ibp_db8: f64,
        var_ibp_dn0: f64,
        var_ibp_dn1: f64,
        var_ibp_dn10: f64,
        var_ibp_dn11: f64,
        var_ibp_dn12: f64,
        var_ibp_dn2: f64,
        var_ibp_dn3: f64,
        var_ibp_dn4: f64,
        var_ibp_dn5: f64,
        var_ibp_dn6: f64,
        var_ibp_dn7: f64,
        var_ibp_dn8: f64,
        var_ibp_dn9: f64,
        var_qjd_1: f64,
        var_qjd_1_db0: f64,
        var_qjd_1_db1: f64,
        var_qjd_1_db2: f64,
        var_qjd_1_db3: f64,
        var_qjd_1_db4: f64,
        var_qjd_1_db5: f64,
        var_qjd_1_db6: f64,
        var_qjd_1_db7: f64,
        var_qjd_1_db8: f64,
        var_qjd_1_dn0: f64,
        var_qjd_1_dn1: f64,
        var_qjd_1_dn10: f64,
        var_qjd_1_dn11: f64,
        var_qjd_1_dn12: f64,
        var_qjd_1_dn2: f64,
        var_qjd_1_dn3: f64,
        var_qjd_1_dn4: f64,
        var_qjd_1_dn5: f64,
        var_qjd_1_dn6: f64,
        var_qjd_1_dn7: f64,
        var_qjd_1_dn8: f64,
        var_qjd_1_dn9: f64,
        var_qsub: f64,
        var_qsub_db0: f64,
        var_qsub_db1: f64,
        var_qsub_db2: f64,
        var_qsub_db3: f64,
        var_qsub_db4: f64,
        var_qsub_db5: f64,
        var_qsub_db6: f64,
        var_qsub_db7: f64,
        var_qsub_db8: f64,
        var_qsub_dn0: f64,
        var_qsub_dn1: f64,
        var_qsub_dn10: f64,
        var_qsub_dn11: f64,
        var_qsub_dn12: f64,
        var_qsub_dn2: f64,
        var_qsub_dn3: f64,
        var_qsub_dn4: f64,
        var_qsub_dn5: f64,
        var_qsub_dn6: f64,
        var_qsub_dn7: f64,
        var_qsub_dn8: f64,
        var_qsub_dn9: f64,
    ) {
        let nv3 = ctx.node_voltage(nodes[3]);
        let nv9 = ctx.node_voltage(nodes[9]);
        let nv10 = ctx.node_voltage(nodes[10]);
        let (eq23_e1234, eq23_e1234_d_n0, eq23_e1234_d_n1, eq23_e1234_d_n2, eq23_e1234_d_n3, eq23_e1234_d_n4, eq23_e1234_d_n5, eq23_e1234_d_n6, eq23_e1234_d_n7, eq23_e1234_d_n8, eq23_e1234_d_n9, eq23_e1234_d_n10, eq23_e1234_d_n11, eq23_e1234_d_n12, eq23_e1234_d_b0, eq23_e1234_d_b1, eq23_e1234_d_b2, eq23_e1234_d_b3, eq23_e1234_d_b4, eq23_e1234_d_b5, eq23_e1234_d_b6, eq23_e1234_d_b7, eq23_e1234_d_b8,) = {
    if (var_guard1827 == 0.0) {
        let eq23_e1232: f64 = (var_b4soitype * var_ibp);
        let eq23_e1232_d_n0: f64 = (var_b4soitype * var_ibp_dn0);
        let eq23_e1232_d_n1: f64 = (var_b4soitype * var_ibp_dn1);
        let eq23_e1232_d_n2: f64 = (var_b4soitype * var_ibp_dn2);
        let eq23_e1232_d_n3: f64 = (var_b4soitype * var_ibp_dn3);
        let eq23_e1232_d_n4: f64 = (var_b4soitype * var_ibp_dn4);
        let eq23_e1232_d_n5: f64 = (var_b4soitype * var_ibp_dn5);
        let eq23_e1232_d_n6: f64 = (var_b4soitype * var_ibp_dn6);
        let eq23_e1232_d_n7: f64 = (var_b4soitype * var_ibp_dn7);
        let eq23_e1232_d_n8: f64 = (var_b4soitype * var_ibp_dn8);
        let eq23_e1232_d_n9: f64 = (var_b4soitype * var_ibp_dn9);
        let eq23_e1232_d_n10: f64 = (var_b4soitype * var_ibp_dn10);
        let eq23_e1232_d_n11: f64 = (var_b4soitype * var_ibp_dn11);
        let eq23_e1232_d_n12: f64 = (var_b4soitype * var_ibp_dn12);
        let eq23_e1232_d_b0: f64 = (var_b4soitype * var_ibp_db0);
        let eq23_e1232_d_b1: f64 = (var_b4soitype * var_ibp_db1);
        let eq23_e1232_d_b2: f64 = (var_b4soitype * var_ibp_db2);
        let eq23_e1232_d_b3: f64 = (var_b4soitype * var_ibp_db3);
        let eq23_e1232_d_b4: f64 = (var_b4soitype * var_ibp_db4);
        let eq23_e1232_d_b5: f64 = (var_b4soitype * var_ibp_db5);
        let eq23_e1232_d_b6: f64 = (var_b4soitype * var_ibp_db6);
        let eq23_e1232_d_b7: f64 = (var_b4soitype * var_ibp_db7);
        let eq23_e1232_d_b8: f64 = (var_b4soitype * var_ibp_db8);
        (eq23_e1232, eq23_e1232_d_n0, eq23_e1232_d_n1, eq23_e1232_d_n2, eq23_e1232_d_n3, eq23_e1232_d_n4, eq23_e1232_d_n5, eq23_e1232_d_n6, eq23_e1232_d_n7, eq23_e1232_d_n8, eq23_e1232_d_n9, eq23_e1232_d_n10, eq23_e1232_d_n11, eq23_e1232_d_n12, eq23_e1232_d_b0, eq23_e1232_d_b1, eq23_e1232_d_b2, eq23_e1232_d_b3, eq23_e1232_d_b4, eq23_e1232_d_b5, eq23_e1232_d_b6, eq23_e1232_d_b7, eq23_e1232_d_b8,)
    } else {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    }
};
        let eq23_value: f64 = eq23_e1234;
        let eq23_node_derivatives: [f64; 13] = [eq23_e1234_d_n0, eq23_e1234_d_n1, eq23_e1234_d_n2, eq23_e1234_d_n3, eq23_e1234_d_n4, eq23_e1234_d_n5, eq23_e1234_d_n6, eq23_e1234_d_n7, eq23_e1234_d_n8, eq23_e1234_d_n9, eq23_e1234_d_n10, eq23_e1234_d_n11, eq23_e1234_d_n12];
        let eq23_branch_derivatives: [f64; 9] = [eq23_e1234_d_b0, eq23_e1234_d_b1, eq23_e1234_d_b2, eq23_e1234_d_b3, eq23_e1234_d_b4, eq23_e1234_d_b5, eq23_e1234_d_b6, eq23_e1234_d_b7, eq23_e1234_d_b8];
        stamper.stamp_current_dense_local(
            Some(5),
            Some(4),
            multiplicity * (eq23_value),
            &eq23_node_derivatives,
            &eq23_branch_derivatives,
            multiplicity,
        );
        let eq30_e1299: f64 = eval_ddt(ddt_state_current, ddt_state_previous, ddt_state_older, ddt_state_initialized, ddt_derivative_current, ddt_derivative_previous, ddt_active, ddt_scale, ddt_previous_value_scale, ddt_older_value_scale, ddt_previous_derivative_scale, 0, s.v[446]);
        let eq30_value: f64 = eq30_e1299;
        stamper.stamp_current_dense_local(
            Some(7),
            Some(5),
            multiplicity * (eq30_value),
            &s.dn[446],
            &s.db[446],
            (multiplicity) * (ddt_scale),
        );
        let eq31_e1301: f64 = eval_ddt(ddt_state_current, ddt_state_previous, ddt_state_older, ddt_state_initialized, ddt_derivative_current, ddt_derivative_previous, ddt_active, ddt_scale, ddt_previous_value_scale, ddt_older_value_scale, ddt_previous_derivative_scale, 1, s.v[447]);
        let eq31_value: f64 = eq31_e1301;
        stamper.stamp_current_dense_local(
            Some(8),
            Some(5),
            multiplicity * (eq31_value),
            &s.dn[447],
            &s.db[447],
            (multiplicity) * (ddt_scale),
        );
        let eq32_e1304: f64 = eval_ddt(ddt_state_current, ddt_state_previous, ddt_state_older, ddt_state_initialized, ddt_derivative_current, ddt_derivative_previous, ddt_active, ddt_scale, ddt_previous_value_scale, ddt_older_value_scale, ddt_previous_derivative_scale, 2, s.v[1251]);
        let eq32_e1305: f64 = (var_b4soitype * eq32_e1304);
        let eq32_e1305_d_n0: f64 = (var_b4soitype * (s.dn[1251][0] * ddt_scale));
        let eq32_e1305_d_n1: f64 = (var_b4soitype * (s.dn[1251][1] * ddt_scale));
        let eq32_e1305_d_n2: f64 = (var_b4soitype * (s.dn[1251][2] * ddt_scale));
        let eq32_e1305_d_n3: f64 = (var_b4soitype * (s.dn[1251][3] * ddt_scale));
        let eq32_e1305_d_n4: f64 = (var_b4soitype * (s.dn[1251][4] * ddt_scale));
        let eq32_e1305_d_n5: f64 = (var_b4soitype * (s.dn[1251][5] * ddt_scale));
        let eq32_e1305_d_n6: f64 = (var_b4soitype * (s.dn[1251][6] * ddt_scale));
        let eq32_e1305_d_n7: f64 = (var_b4soitype * (s.dn[1251][7] * ddt_scale));
        let eq32_e1305_d_n8: f64 = (var_b4soitype * (s.dn[1251][8] * ddt_scale));
        let eq32_e1305_d_n9: f64 = (var_b4soitype * (s.dn[1251][9] * ddt_scale));
        let eq32_e1305_d_n10: f64 = (var_b4soitype * (s.dn[1251][10] * ddt_scale));
        let eq32_e1305_d_n11: f64 = (var_b4soitype * (s.dn[1251][11] * ddt_scale));
        let eq32_e1305_d_n12: f64 = (var_b4soitype * (s.dn[1251][12] * ddt_scale));
        let eq32_e1305_d_b0: f64 = (var_b4soitype * (s.db[1251][0] * ddt_scale));
        let eq32_e1305_d_b1: f64 = (var_b4soitype * (s.db[1251][1] * ddt_scale));
        let eq32_e1305_d_b2: f64 = (var_b4soitype * (s.db[1251][2] * ddt_scale));
        let eq32_e1305_d_b3: f64 = (var_b4soitype * (s.db[1251][3] * ddt_scale));
        let eq32_e1305_d_b4: f64 = (var_b4soitype * (s.db[1251][4] * ddt_scale));
        let eq32_e1305_d_b5: f64 = (var_b4soitype * (s.db[1251][5] * ddt_scale));
        let eq32_e1305_d_b6: f64 = (var_b4soitype * (s.db[1251][6] * ddt_scale));
        let eq32_e1305_d_b7: f64 = (var_b4soitype * (s.db[1251][7] * ddt_scale));
        let eq32_e1305_d_b8: f64 = (var_b4soitype * (s.db[1251][8] * ddt_scale));
        let eq32_value: f64 = eq32_e1305;
        let eq32_node_derivatives: [f64; 13] = [eq32_e1305_d_n0, eq32_e1305_d_n1, eq32_e1305_d_n2, eq32_e1305_d_n3, eq32_e1305_d_n4, eq32_e1305_d_n5, eq32_e1305_d_n6, eq32_e1305_d_n7, eq32_e1305_d_n8, eq32_e1305_d_n9, eq32_e1305_d_n10, eq32_e1305_d_n11, eq32_e1305_d_n12];
        let eq32_branch_derivatives: [f64; 9] = [eq32_e1305_d_b0, eq32_e1305_d_b1, eq32_e1305_d_b2, eq32_e1305_d_b3, eq32_e1305_d_b4, eq32_e1305_d_b5, eq32_e1305_d_b6, eq32_e1305_d_b7, eq32_e1305_d_b8];
        stamper.stamp_current_dense_local(
            Some(9),
            Some(5),
            multiplicity * (eq32_value),
            &eq32_node_derivatives,
            &eq32_branch_derivatives,
            multiplicity,
        );
        let eq33_e1308: f64 = eval_ddt(ddt_state_current, ddt_state_previous, ddt_state_older, ddt_state_initialized, ddt_derivative_current, ddt_derivative_previous, ddt_active, ddt_scale, ddt_previous_value_scale, ddt_older_value_scale, ddt_previous_derivative_scale, 3, var_qsub);
        let eq33_e1309: f64 = (var_b4soitype * eq33_e1308);
        let eq33_e1309_d_n0: f64 = (var_b4soitype * (var_qsub_dn0 * ddt_scale));
        let eq33_e1309_d_n1: f64 = (var_b4soitype * (var_qsub_dn1 * ddt_scale));
        let eq33_e1309_d_n2: f64 = (var_b4soitype * (var_qsub_dn2 * ddt_scale));
        let eq33_e1309_d_n3: f64 = (var_b4soitype * (var_qsub_dn3 * ddt_scale));
        let eq33_e1309_d_n4: f64 = (var_b4soitype * (var_qsub_dn4 * ddt_scale));
        let eq33_e1309_d_n5: f64 = (var_b4soitype * (var_qsub_dn5 * ddt_scale));
        let eq33_e1309_d_n6: f64 = (var_b4soitype * (var_qsub_dn6 * ddt_scale));
        let eq33_e1309_d_n7: f64 = (var_b4soitype * (var_qsub_dn7 * ddt_scale));
        let eq33_e1309_d_n8: f64 = (var_b4soitype * (var_qsub_dn8 * ddt_scale));
        let eq33_e1309_d_n9: f64 = (var_b4soitype * (var_qsub_dn9 * ddt_scale));
        let eq33_e1309_d_n10: f64 = (var_b4soitype * (var_qsub_dn10 * ddt_scale));
        let eq33_e1309_d_n11: f64 = (var_b4soitype * (var_qsub_dn11 * ddt_scale));
        let eq33_e1309_d_n12: f64 = (var_b4soitype * (var_qsub_dn12 * ddt_scale));
        let eq33_e1309_d_b0: f64 = (var_b4soitype * (var_qsub_db0 * ddt_scale));
        let eq33_e1309_d_b1: f64 = (var_b4soitype * (var_qsub_db1 * ddt_scale));
        let eq33_e1309_d_b2: f64 = (var_b4soitype * (var_qsub_db2 * ddt_scale));
        let eq33_e1309_d_b3: f64 = (var_b4soitype * (var_qsub_db3 * ddt_scale));
        let eq33_e1309_d_b4: f64 = (var_b4soitype * (var_qsub_db4 * ddt_scale));
        let eq33_e1309_d_b5: f64 = (var_b4soitype * (var_qsub_db5 * ddt_scale));
        let eq33_e1309_d_b6: f64 = (var_b4soitype * (var_qsub_db6 * ddt_scale));
        let eq33_e1309_d_b7: f64 = (var_b4soitype * (var_qsub_db7 * ddt_scale));
        let eq33_e1309_d_b8: f64 = (var_b4soitype * (var_qsub_db8 * ddt_scale));
        let eq33_value: f64 = eq33_e1309;
        let eq33_node_derivatives: [f64; 13] = [eq33_e1309_d_n0, eq33_e1309_d_n1, eq33_e1309_d_n2, eq33_e1309_d_n3, eq33_e1309_d_n4, eq33_e1309_d_n5, eq33_e1309_d_n6, eq33_e1309_d_n7, eq33_e1309_d_n8, eq33_e1309_d_n9, eq33_e1309_d_n10, eq33_e1309_d_n11, eq33_e1309_d_n12];
        let eq33_branch_derivatives: [f64; 9] = [eq33_e1309_d_b0, eq33_e1309_d_b1, eq33_e1309_d_b2, eq33_e1309_d_b3, eq33_e1309_d_b4, eq33_e1309_d_b5, eq33_e1309_d_b6, eq33_e1309_d_b7, eq33_e1309_d_b8];
        stamper.stamp_current_dense_local(
            Some(3),
            Some(5),
            multiplicity * (eq33_value),
            &eq33_node_derivatives,
            &eq33_branch_derivatives,
            multiplicity,
        );
        let eq34_e1312: f64 = eval_ddt(ddt_state_current, ddt_state_previous, ddt_state_older, ddt_state_initialized, ddt_derivative_current, ddt_derivative_previous, ddt_active, ddt_scale, ddt_previous_value_scale, ddt_older_value_scale, ddt_previous_derivative_scale, 4, var_qjd_1);
        let eq34_e1313: f64 = (var_b4soitype * eq34_e1312);
        let eq34_e1313_d_n0: f64 = (var_b4soitype * (var_qjd_1_dn0 * ddt_scale));
        let eq34_e1313_d_n1: f64 = (var_b4soitype * (var_qjd_1_dn1 * ddt_scale));
        let eq34_e1313_d_n2: f64 = (var_b4soitype * (var_qjd_1_dn2 * ddt_scale));
        let eq34_e1313_d_n3: f64 = (var_b4soitype * (var_qjd_1_dn3 * ddt_scale));
        let eq34_e1313_d_n4: f64 = (var_b4soitype * (var_qjd_1_dn4 * ddt_scale));
        let eq34_e1313_d_n5: f64 = (var_b4soitype * (var_qjd_1_dn5 * ddt_scale));
        let eq34_e1313_d_n6: f64 = (var_b4soitype * (var_qjd_1_dn6 * ddt_scale));
        let eq34_e1313_d_n7: f64 = (var_b4soitype * (var_qjd_1_dn7 * ddt_scale));
        let eq34_e1313_d_n8: f64 = (var_b4soitype * (var_qjd_1_dn8 * ddt_scale));
        let eq34_e1313_d_n9: f64 = (var_b4soitype * (var_qjd_1_dn9 * ddt_scale));
        let eq34_e1313_d_n10: f64 = (var_b4soitype * (var_qjd_1_dn10 * ddt_scale));
        let eq34_e1313_d_n11: f64 = (var_b4soitype * (var_qjd_1_dn11 * ddt_scale));
        let eq34_e1313_d_n12: f64 = (var_b4soitype * (var_qjd_1_dn12 * ddt_scale));
        let eq34_e1313_d_b0: f64 = (var_b4soitype * (var_qjd_1_db0 * ddt_scale));
        let eq34_e1313_d_b1: f64 = (var_b4soitype * (var_qjd_1_db1 * ddt_scale));
        let eq34_e1313_d_b2: f64 = (var_b4soitype * (var_qjd_1_db2 * ddt_scale));
        let eq34_e1313_d_b3: f64 = (var_b4soitype * (var_qjd_1_db3 * ddt_scale));
        let eq34_e1313_d_b4: f64 = (var_b4soitype * (var_qjd_1_db4 * ddt_scale));
        let eq34_e1313_d_b5: f64 = (var_b4soitype * (var_qjd_1_db5 * ddt_scale));
        let eq34_e1313_d_b6: f64 = (var_b4soitype * (var_qjd_1_db6 * ddt_scale));
        let eq34_e1313_d_b7: f64 = (var_b4soitype * (var_qjd_1_db7 * ddt_scale));
        let eq34_e1313_d_b8: f64 = (var_b4soitype * (var_qjd_1_db8 * ddt_scale));
        let eq34_value: f64 = eq34_e1313;
        let eq34_node_derivatives: [f64; 13] = [eq34_e1313_d_n0, eq34_e1313_d_n1, eq34_e1313_d_n2, eq34_e1313_d_n3, eq34_e1313_d_n4, eq34_e1313_d_n5, eq34_e1313_d_n6, eq34_e1313_d_n7, eq34_e1313_d_n8, eq34_e1313_d_n9, eq34_e1313_d_n10, eq34_e1313_d_n11, eq34_e1313_d_n12];
        let eq34_branch_derivatives: [f64; 9] = [eq34_e1313_d_b0, eq34_e1313_d_b1, eq34_e1313_d_b2, eq34_e1313_d_b3, eq34_e1313_d_b4, eq34_e1313_d_b5, eq34_e1313_d_b6, eq34_e1313_d_b7, eq34_e1313_d_b8];
        stamper.stamp_current_dense_local(
            Some(12),
            Some(7),
            multiplicity * (eq34_value),
            &eq34_node_derivatives,
            &eq34_branch_derivatives,
            multiplicity,
        );
        let eq35_e1316: f64 = eval_ddt(ddt_state_current, ddt_state_previous, ddt_state_older, ddt_state_initialized, ddt_derivative_current, ddt_derivative_previous, ddt_active, ddt_scale, ddt_previous_value_scale, ddt_older_value_scale, ddt_previous_derivative_scale, 5, s.v[1245]);
        let eq35_e1317: f64 = (var_b4soitype * eq35_e1316);
        let eq35_e1317_d_n0: f64 = (var_b4soitype * (s.dn[1245][0] * ddt_scale));
        let eq35_e1317_d_n1: f64 = (var_b4soitype * (s.dn[1245][1] * ddt_scale));
        let eq35_e1317_d_n2: f64 = (var_b4soitype * (s.dn[1245][2] * ddt_scale));
        let eq35_e1317_d_n3: f64 = (var_b4soitype * (s.dn[1245][3] * ddt_scale));
        let eq35_e1317_d_n4: f64 = (var_b4soitype * (s.dn[1245][4] * ddt_scale));
        let eq35_e1317_d_n5: f64 = (var_b4soitype * (s.dn[1245][5] * ddt_scale));
        let eq35_e1317_d_n6: f64 = (var_b4soitype * (s.dn[1245][6] * ddt_scale));
        let eq35_e1317_d_n7: f64 = (var_b4soitype * (s.dn[1245][7] * ddt_scale));
        let eq35_e1317_d_n8: f64 = (var_b4soitype * (s.dn[1245][8] * ddt_scale));
        let eq35_e1317_d_n9: f64 = (var_b4soitype * (s.dn[1245][9] * ddt_scale));
        let eq35_e1317_d_n10: f64 = (var_b4soitype * (s.dn[1245][10] * ddt_scale));
        let eq35_e1317_d_n11: f64 = (var_b4soitype * (s.dn[1245][11] * ddt_scale));
        let eq35_e1317_d_n12: f64 = (var_b4soitype * (s.dn[1245][12] * ddt_scale));
        let eq35_e1317_d_b0: f64 = (var_b4soitype * (s.db[1245][0] * ddt_scale));
        let eq35_e1317_d_b1: f64 = (var_b4soitype * (s.db[1245][1] * ddt_scale));
        let eq35_e1317_d_b2: f64 = (var_b4soitype * (s.db[1245][2] * ddt_scale));
        let eq35_e1317_d_b3: f64 = (var_b4soitype * (s.db[1245][3] * ddt_scale));
        let eq35_e1317_d_b4: f64 = (var_b4soitype * (s.db[1245][4] * ddt_scale));
        let eq35_e1317_d_b5: f64 = (var_b4soitype * (s.db[1245][5] * ddt_scale));
        let eq35_e1317_d_b6: f64 = (var_b4soitype * (s.db[1245][6] * ddt_scale));
        let eq35_e1317_d_b7: f64 = (var_b4soitype * (s.db[1245][7] * ddt_scale));
        let eq35_e1317_d_b8: f64 = (var_b4soitype * (s.db[1245][8] * ddt_scale));
        let eq35_value: f64 = eq35_e1317;
        let eq35_node_derivatives: [f64; 13] = [eq35_e1317_d_n0, eq35_e1317_d_n1, eq35_e1317_d_n2, eq35_e1317_d_n3, eq35_e1317_d_n4, eq35_e1317_d_n5, eq35_e1317_d_n6, eq35_e1317_d_n7, eq35_e1317_d_n8, eq35_e1317_d_n9, eq35_e1317_d_n10, eq35_e1317_d_n11, eq35_e1317_d_n12];
        let eq35_branch_derivatives: [f64; 9] = [eq35_e1317_d_b0, eq35_e1317_d_b1, eq35_e1317_d_b2, eq35_e1317_d_b3, eq35_e1317_d_b4, eq35_e1317_d_b5, eq35_e1317_d_b6, eq35_e1317_d_b7, eq35_e1317_d_b8];
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
        let eq36_e1321: f64 = eval_ddt(ddt_state_current, ddt_state_previous, ddt_state_older, ddt_state_initialized, ddt_derivative_current, ddt_derivative_previous, ddt_active, ddt_scale, ddt_previous_value_scale, ddt_older_value_scale, ddt_previous_derivative_scale, 6, s.v[1230]);
        let eq36_e1322: f64 = (var_b4soitype * eq36_e1321);
        let eq36_e1322_d_n0: f64 = (var_b4soitype * (s.dn[1230][0] * ddt_scale));
        let eq36_e1322_d_n1: f64 = (var_b4soitype * (s.dn[1230][1] * ddt_scale));
        let eq36_e1322_d_n2: f64 = (var_b4soitype * (s.dn[1230][2] * ddt_scale));
        let eq36_e1322_d_n3: f64 = (var_b4soitype * (s.dn[1230][3] * ddt_scale));
        let eq36_e1322_d_n4: f64 = (var_b4soitype * (s.dn[1230][4] * ddt_scale));
        let eq36_e1322_d_n5: f64 = (var_b4soitype * (s.dn[1230][5] * ddt_scale));
        let eq36_e1322_d_n6: f64 = (var_b4soitype * (s.dn[1230][6] * ddt_scale));
        let eq36_e1322_d_n7: f64 = (var_b4soitype * (s.dn[1230][7] * ddt_scale));
        let eq36_e1322_d_n8: f64 = (var_b4soitype * (s.dn[1230][8] * ddt_scale));
        let eq36_e1322_d_n9: f64 = (var_b4soitype * (s.dn[1230][9] * ddt_scale));
        let eq36_e1322_d_n10: f64 = (var_b4soitype * (s.dn[1230][10] * ddt_scale));
        let eq36_e1322_d_n11: f64 = (var_b4soitype * (s.dn[1230][11] * ddt_scale));
        let eq36_e1322_d_n12: f64 = (var_b4soitype * (s.dn[1230][12] * ddt_scale));
        let eq36_e1322_d_b0: f64 = (var_b4soitype * (s.db[1230][0] * ddt_scale));
        let eq36_e1322_d_b1: f64 = (var_b4soitype * (s.db[1230][1] * ddt_scale));
        let eq36_e1322_d_b2: f64 = (var_b4soitype * (s.db[1230][2] * ddt_scale));
        let eq36_e1322_d_b3: f64 = (var_b4soitype * (s.db[1230][3] * ddt_scale));
        let eq36_e1322_d_b4: f64 = (var_b4soitype * (s.db[1230][4] * ddt_scale));
        let eq36_e1322_d_b5: f64 = (var_b4soitype * (s.db[1230][5] * ddt_scale));
        let eq36_e1322_d_b6: f64 = (var_b4soitype * (s.db[1230][6] * ddt_scale));
        let eq36_e1322_d_b7: f64 = (var_b4soitype * (s.db[1230][7] * ddt_scale));
        let eq36_e1322_d_b8: f64 = (var_b4soitype * (s.db[1230][8] * ddt_scale));
        (eq36_e1322, eq36_e1322_d_n0, eq36_e1322_d_n1, eq36_e1322_d_n2, eq36_e1322_d_n3, eq36_e1322_d_n4, eq36_e1322_d_n5, eq36_e1322_d_n6, eq36_e1322_d_n7, eq36_e1322_d_n8, eq36_e1322_d_n9, eq36_e1322_d_n10, eq36_e1322_d_n11, eq36_e1322_d_n12, eq36_e1322_d_b0, eq36_e1322_d_b1, eq36_e1322_d_b2, eq36_e1322_d_b3, eq36_e1322_d_b4, eq36_e1322_d_b5, eq36_e1322_d_b6, eq36_e1322_d_b7, eq36_e1322_d_b8,)
    } else {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    }
};
        let eq36_value: f64 = eq36_e1324;
        let eq36_node_derivatives: [f64; 13] = [eq36_e1324_d_n0, eq36_e1324_d_n1, eq36_e1324_d_n2, eq36_e1324_d_n3, eq36_e1324_d_n4, eq36_e1324_d_n5, eq36_e1324_d_n6, eq36_e1324_d_n7, eq36_e1324_d_n8, eq36_e1324_d_n9, eq36_e1324_d_n10, eq36_e1324_d_n11, eq36_e1324_d_n12];
        let eq36_branch_derivatives: [f64; 9] = [eq36_e1324_d_b0, eq36_e1324_d_b1, eq36_e1324_d_b2, eq36_e1324_d_b3, eq36_e1324_d_b4, eq36_e1324_d_b5, eq36_e1324_d_b6, eq36_e1324_d_b7, eq36_e1324_d_b8];
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
        let eq37_e1328: f64 = eval_ddt(ddt_state_current, ddt_state_previous, ddt_state_older, ddt_state_initialized, ddt_derivative_current, ddt_derivative_previous, ddt_active, ddt_scale, ddt_previous_value_scale, ddt_older_value_scale, ddt_previous_derivative_scale, 7, s.v[1231]);
        let eq37_e1329: f64 = (var_b4soitype * eq37_e1328);
        let eq37_e1329_d_n0: f64 = (var_b4soitype * (s.dn[1231][0] * ddt_scale));
        let eq37_e1329_d_n1: f64 = (var_b4soitype * (s.dn[1231][1] * ddt_scale));
        let eq37_e1329_d_n2: f64 = (var_b4soitype * (s.dn[1231][2] * ddt_scale));
        let eq37_e1329_d_n3: f64 = (var_b4soitype * (s.dn[1231][3] * ddt_scale));
        let eq37_e1329_d_n4: f64 = (var_b4soitype * (s.dn[1231][4] * ddt_scale));
        let eq37_e1329_d_n5: f64 = (var_b4soitype * (s.dn[1231][5] * ddt_scale));
        let eq37_e1329_d_n6: f64 = (var_b4soitype * (s.dn[1231][6] * ddt_scale));
        let eq37_e1329_d_n7: f64 = (var_b4soitype * (s.dn[1231][7] * ddt_scale));
        let eq37_e1329_d_n8: f64 = (var_b4soitype * (s.dn[1231][8] * ddt_scale));
        let eq37_e1329_d_n9: f64 = (var_b4soitype * (s.dn[1231][9] * ddt_scale));
        let eq37_e1329_d_n10: f64 = (var_b4soitype * (s.dn[1231][10] * ddt_scale));
        let eq37_e1329_d_n11: f64 = (var_b4soitype * (s.dn[1231][11] * ddt_scale));
        let eq37_e1329_d_n12: f64 = (var_b4soitype * (s.dn[1231][12] * ddt_scale));
        let eq37_e1329_d_b0: f64 = (var_b4soitype * (s.db[1231][0] * ddt_scale));
        let eq37_e1329_d_b1: f64 = (var_b4soitype * (s.db[1231][1] * ddt_scale));
        let eq37_e1329_d_b2: f64 = (var_b4soitype * (s.db[1231][2] * ddt_scale));
        let eq37_e1329_d_b3: f64 = (var_b4soitype * (s.db[1231][3] * ddt_scale));
        let eq37_e1329_d_b4: f64 = (var_b4soitype * (s.db[1231][4] * ddt_scale));
        let eq37_e1329_d_b5: f64 = (var_b4soitype * (s.db[1231][5] * ddt_scale));
        let eq37_e1329_d_b6: f64 = (var_b4soitype * (s.db[1231][6] * ddt_scale));
        let eq37_e1329_d_b7: f64 = (var_b4soitype * (s.db[1231][7] * ddt_scale));
        let eq37_e1329_d_b8: f64 = (var_b4soitype * (s.db[1231][8] * ddt_scale));
        (eq37_e1329, eq37_e1329_d_n0, eq37_e1329_d_n1, eq37_e1329_d_n2, eq37_e1329_d_n3, eq37_e1329_d_n4, eq37_e1329_d_n5, eq37_e1329_d_n6, eq37_e1329_d_n7, eq37_e1329_d_n8, eq37_e1329_d_n9, eq37_e1329_d_n10, eq37_e1329_d_n11, eq37_e1329_d_n12, eq37_e1329_d_b0, eq37_e1329_d_b1, eq37_e1329_d_b2, eq37_e1329_d_b3, eq37_e1329_d_b4, eq37_e1329_d_b5, eq37_e1329_d_b6, eq37_e1329_d_b7, eq37_e1329_d_b8,)
    } else {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    }
};
        let eq37_value: f64 = eq37_e1331;
        let eq37_node_derivatives: [f64; 13] = [eq37_e1331_d_n0, eq37_e1331_d_n1, eq37_e1331_d_n2, eq37_e1331_d_n3, eq37_e1331_d_n4, eq37_e1331_d_n5, eq37_e1331_d_n6, eq37_e1331_d_n7, eq37_e1331_d_n8, eq37_e1331_d_n9, eq37_e1331_d_n10, eq37_e1331_d_n11, eq37_e1331_d_n12];
        let eq37_branch_derivatives: [f64; 9] = [eq37_e1331_d_b0, eq37_e1331_d_b1, eq37_e1331_d_b2, eq37_e1331_d_b3, eq37_e1331_d_b4, eq37_e1331_d_b5, eq37_e1331_d_b6, eq37_e1331_d_b7, eq37_e1331_d_b8];
        stamper.stamp_current_dense_local(
            Some(10),
            Some(8),
            multiplicity * (eq37_value),
            &eq37_node_derivatives,
            &eq37_branch_derivatives,
            multiplicity,
        );
        let (eq38_e1338, eq38_e1338_d_n0, eq38_e1338_d_n1, eq38_e1338_d_n2, eq38_e1338_d_n3, eq38_e1338_d_n4, eq38_e1338_d_n5, eq38_e1338_d_n6, eq38_e1338_d_n7, eq38_e1338_d_n8, eq38_e1338_d_n9, eq38_e1338_d_n10, eq38_e1338_d_n11, eq38_e1338_d_n12, eq38_e1338_d_b0, eq38_e1338_d_b1, eq38_e1338_d_b2, eq38_e1338_d_b3, eq38_e1338_d_b4, eq38_e1338_d_b5, eq38_e1338_d_b6, eq38_e1338_d_b7, eq38_e1338_d_b8,) = {
    if s.b[1863] {
        let eq38_e1335: f64 = ((nv10 - nv3) * s.v[697]);
        let eq38_e1335_d_n0: f64 = ((nv10 - nv3) * s.dn[697][0]);
        let eq38_e1335_d_n1: f64 = ((nv10 - nv3) * s.dn[697][1]);
        let eq38_e1335_d_n2: f64 = ((nv10 - nv3) * s.dn[697][2]);
        let eq38_e1335_d_n3: f64 = ((-s.v[697]) + ((nv10 - nv3) * s.dn[697][3]));
        let eq38_e1335_d_n4: f64 = ((nv10 - nv3) * s.dn[697][4]);
        let eq38_e1335_d_n5: f64 = ((nv10 - nv3) * s.dn[697][5]);
        let eq38_e1335_d_n6: f64 = ((nv10 - nv3) * s.dn[697][6]);
        let eq38_e1335_d_n7: f64 = ((nv10 - nv3) * s.dn[697][7]);
        let eq38_e1335_d_n8: f64 = ((nv10 - nv3) * s.dn[697][8]);
        let eq38_e1335_d_n9: f64 = ((nv10 - nv3) * s.dn[697][9]);
        let eq38_e1335_d_n10: f64 = (s.v[697] + ((nv10 - nv3) * s.dn[697][10]));
        let eq38_e1335_d_n11: f64 = ((nv10 - nv3) * s.dn[697][11]);
        let eq38_e1335_d_n12: f64 = ((nv10 - nv3) * s.dn[697][12]);
        let eq38_e1335_d_b0: f64 = ((nv10 - nv3) * s.db[697][0]);
        let eq38_e1335_d_b1: f64 = ((nv10 - nv3) * s.db[697][1]);
        let eq38_e1335_d_b2: f64 = ((nv10 - nv3) * s.db[697][2]);
        let eq38_e1335_d_b3: f64 = ((nv10 - nv3) * s.db[697][3]);
        let eq38_e1335_d_b4: f64 = ((nv10 - nv3) * s.db[697][4]);
        let eq38_e1335_d_b5: f64 = ((nv10 - nv3) * s.db[697][5]);
        let eq38_e1335_d_b6: f64 = ((nv10 - nv3) * s.db[697][6]);
        let eq38_e1335_d_b7: f64 = ((nv10 - nv3) * s.db[697][7]);
        let eq38_e1335_d_b8: f64 = ((nv10 - nv3) * s.db[697][8]);
        let eq38_e1336: f64 = eval_ddt(ddt_state_current, ddt_state_previous, ddt_state_older, ddt_state_initialized, ddt_derivative_current, ddt_derivative_previous, ddt_active, ddt_scale, ddt_previous_value_scale, ddt_older_value_scale, ddt_previous_derivative_scale, 8, eq38_e1335);
        (eq38_e1336, (eq38_e1335_d_n0 * ddt_scale), (eq38_e1335_d_n1 * ddt_scale), (eq38_e1335_d_n2 * ddt_scale), (eq38_e1335_d_n3 * ddt_scale), (eq38_e1335_d_n4 * ddt_scale), (eq38_e1335_d_n5 * ddt_scale), (eq38_e1335_d_n6 * ddt_scale), (eq38_e1335_d_n7 * ddt_scale), (eq38_e1335_d_n8 * ddt_scale), (eq38_e1335_d_n9 * ddt_scale), (eq38_e1335_d_n10 * ddt_scale), (eq38_e1335_d_n11 * ddt_scale), (eq38_e1335_d_n12 * ddt_scale), (eq38_e1335_d_b0 * ddt_scale), (eq38_e1335_d_b1 * ddt_scale), (eq38_e1335_d_b2 * ddt_scale), (eq38_e1335_d_b3 * ddt_scale), (eq38_e1335_d_b4 * ddt_scale), (eq38_e1335_d_b5 * ddt_scale), (eq38_e1335_d_b6 * ddt_scale), (eq38_e1335_d_b7 * ddt_scale), (eq38_e1335_d_b8 * ddt_scale),)
    } else {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    }
};
        let eq38_value: f64 = eq38_e1338;
        let eq38_node_derivatives: [f64; 13] = [eq38_e1338_d_n0, eq38_e1338_d_n1, eq38_e1338_d_n2, eq38_e1338_d_n3, eq38_e1338_d_n4, eq38_e1338_d_n5, eq38_e1338_d_n6, eq38_e1338_d_n7, eq38_e1338_d_n8, eq38_e1338_d_n9, eq38_e1338_d_n10, eq38_e1338_d_n11, eq38_e1338_d_n12];
        let eq38_branch_derivatives: [f64; 9] = [eq38_e1338_d_b0, eq38_e1338_d_b1, eq38_e1338_d_b2, eq38_e1338_d_b3, eq38_e1338_d_b4, eq38_e1338_d_b5, eq38_e1338_d_b6, eq38_e1338_d_b7, eq38_e1338_d_b8];
        stamper.stamp_current_dense_local(
            Some(10),
            Some(3),
            multiplicity * (eq38_value),
            &eq38_node_derivatives,
            &eq38_branch_derivatives,
            multiplicity,
        );
        let (eq39_e1346, eq39_e1346_d_n0, eq39_e1346_d_n1, eq39_e1346_d_n2, eq39_e1346_d_n3, eq39_e1346_d_n4, eq39_e1346_d_n5, eq39_e1346_d_n6, eq39_e1346_d_n7, eq39_e1346_d_n8, eq39_e1346_d_n9, eq39_e1346_d_n10, eq39_e1346_d_n11, eq39_e1346_d_n12, eq39_e1346_d_b0, eq39_e1346_d_b1, eq39_e1346_d_b2, eq39_e1346_d_b3, eq39_e1346_d_b4, eq39_e1346_d_b5, eq39_e1346_d_b6, eq39_e1346_d_b7, eq39_e1346_d_b8,) = {
    if (!s.b[1863]) {
        let eq39_e1343: f64 = eval_ddt(ddt_state_current, ddt_state_previous, ddt_state_older, ddt_state_initialized, ddt_derivative_current, ddt_derivative_previous, ddt_active, ddt_scale, ddt_previous_value_scale, ddt_older_value_scale, ddt_previous_derivative_scale, 9, s.v[1230]);
        let eq39_e1344: f64 = (var_b4soitype * eq39_e1343);
        let eq39_e1344_d_n0: f64 = (var_b4soitype * (s.dn[1230][0] * ddt_scale));
        let eq39_e1344_d_n1: f64 = (var_b4soitype * (s.dn[1230][1] * ddt_scale));
        let eq39_e1344_d_n2: f64 = (var_b4soitype * (s.dn[1230][2] * ddt_scale));
        let eq39_e1344_d_n3: f64 = (var_b4soitype * (s.dn[1230][3] * ddt_scale));
        let eq39_e1344_d_n4: f64 = (var_b4soitype * (s.dn[1230][4] * ddt_scale));
        let eq39_e1344_d_n5: f64 = (var_b4soitype * (s.dn[1230][5] * ddt_scale));
        let eq39_e1344_d_n6: f64 = (var_b4soitype * (s.dn[1230][6] * ddt_scale));
        let eq39_e1344_d_n7: f64 = (var_b4soitype * (s.dn[1230][7] * ddt_scale));
        let eq39_e1344_d_n8: f64 = (var_b4soitype * (s.dn[1230][8] * ddt_scale));
        let eq39_e1344_d_n9: f64 = (var_b4soitype * (s.dn[1230][9] * ddt_scale));
        let eq39_e1344_d_n10: f64 = (var_b4soitype * (s.dn[1230][10] * ddt_scale));
        let eq39_e1344_d_n11: f64 = (var_b4soitype * (s.dn[1230][11] * ddt_scale));
        let eq39_e1344_d_n12: f64 = (var_b4soitype * (s.dn[1230][12] * ddt_scale));
        let eq39_e1344_d_b0: f64 = (var_b4soitype * (s.db[1230][0] * ddt_scale));
        let eq39_e1344_d_b1: f64 = (var_b4soitype * (s.db[1230][1] * ddt_scale));
        let eq39_e1344_d_b2: f64 = (var_b4soitype * (s.db[1230][2] * ddt_scale));
        let eq39_e1344_d_b3: f64 = (var_b4soitype * (s.db[1230][3] * ddt_scale));
        let eq39_e1344_d_b4: f64 = (var_b4soitype * (s.db[1230][4] * ddt_scale));
        let eq39_e1344_d_b5: f64 = (var_b4soitype * (s.db[1230][5] * ddt_scale));
        let eq39_e1344_d_b6: f64 = (var_b4soitype * (s.db[1230][6] * ddt_scale));
        let eq39_e1344_d_b7: f64 = (var_b4soitype * (s.db[1230][7] * ddt_scale));
        let eq39_e1344_d_b8: f64 = (var_b4soitype * (s.db[1230][8] * ddt_scale));
        (eq39_e1344, eq39_e1344_d_n0, eq39_e1344_d_n1, eq39_e1344_d_n2, eq39_e1344_d_n3, eq39_e1344_d_n4, eq39_e1344_d_n5, eq39_e1344_d_n6, eq39_e1344_d_n7, eq39_e1344_d_n8, eq39_e1344_d_n9, eq39_e1344_d_n10, eq39_e1344_d_n11, eq39_e1344_d_n12, eq39_e1344_d_b0, eq39_e1344_d_b1, eq39_e1344_d_b2, eq39_e1344_d_b3, eq39_e1344_d_b4, eq39_e1344_d_b5, eq39_e1344_d_b6, eq39_e1344_d_b7, eq39_e1344_d_b8,)
    } else {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    }
};
        let eq39_value: f64 = eq39_e1346;
        let eq39_node_derivatives: [f64; 13] = [eq39_e1346_d_n0, eq39_e1346_d_n1, eq39_e1346_d_n2, eq39_e1346_d_n3, eq39_e1346_d_n4, eq39_e1346_d_n5, eq39_e1346_d_n6, eq39_e1346_d_n7, eq39_e1346_d_n8, eq39_e1346_d_n9, eq39_e1346_d_n10, eq39_e1346_d_n11, eq39_e1346_d_n12];
        let eq39_branch_derivatives: [f64; 9] = [eq39_e1346_d_b0, eq39_e1346_d_b1, eq39_e1346_d_b2, eq39_e1346_d_b3, eq39_e1346_d_b4, eq39_e1346_d_b5, eq39_e1346_d_b6, eq39_e1346_d_b7, eq39_e1346_d_b8];
        stamper.stamp_current_dense_local(
            Some(9),
            Some(7),
            multiplicity * (eq39_value),
            &eq39_node_derivatives,
            &eq39_branch_derivatives,
            multiplicity,
        );
        let (eq40_e1354, eq40_e1354_d_n0, eq40_e1354_d_n1, eq40_e1354_d_n2, eq40_e1354_d_n3, eq40_e1354_d_n4, eq40_e1354_d_n5, eq40_e1354_d_n6, eq40_e1354_d_n7, eq40_e1354_d_n8, eq40_e1354_d_n9, eq40_e1354_d_n10, eq40_e1354_d_n11, eq40_e1354_d_n12, eq40_e1354_d_b0, eq40_e1354_d_b1, eq40_e1354_d_b2, eq40_e1354_d_b3, eq40_e1354_d_b4, eq40_e1354_d_b5, eq40_e1354_d_b6, eq40_e1354_d_b7, eq40_e1354_d_b8,) = {
    if (!s.b[1863]) {
        let eq40_e1351: f64 = eval_ddt(ddt_state_current, ddt_state_previous, ddt_state_older, ddt_state_initialized, ddt_derivative_current, ddt_derivative_previous, ddt_active, ddt_scale, ddt_previous_value_scale, ddt_older_value_scale, ddt_previous_derivative_scale, 10, s.v[1231]);
        let eq40_e1352: f64 = (var_b4soitype * eq40_e1351);
        let eq40_e1352_d_n0: f64 = (var_b4soitype * (s.dn[1231][0] * ddt_scale));
        let eq40_e1352_d_n1: f64 = (var_b4soitype * (s.dn[1231][1] * ddt_scale));
        let eq40_e1352_d_n2: f64 = (var_b4soitype * (s.dn[1231][2] * ddt_scale));
        let eq40_e1352_d_n3: f64 = (var_b4soitype * (s.dn[1231][3] * ddt_scale));
        let eq40_e1352_d_n4: f64 = (var_b4soitype * (s.dn[1231][4] * ddt_scale));
        let eq40_e1352_d_n5: f64 = (var_b4soitype * (s.dn[1231][5] * ddt_scale));
        let eq40_e1352_d_n6: f64 = (var_b4soitype * (s.dn[1231][6] * ddt_scale));
        let eq40_e1352_d_n7: f64 = (var_b4soitype * (s.dn[1231][7] * ddt_scale));
        let eq40_e1352_d_n8: f64 = (var_b4soitype * (s.dn[1231][8] * ddt_scale));
        let eq40_e1352_d_n9: f64 = (var_b4soitype * (s.dn[1231][9] * ddt_scale));
        let eq40_e1352_d_n10: f64 = (var_b4soitype * (s.dn[1231][10] * ddt_scale));
        let eq40_e1352_d_n11: f64 = (var_b4soitype * (s.dn[1231][11] * ddt_scale));
        let eq40_e1352_d_n12: f64 = (var_b4soitype * (s.dn[1231][12] * ddt_scale));
        let eq40_e1352_d_b0: f64 = (var_b4soitype * (s.db[1231][0] * ddt_scale));
        let eq40_e1352_d_b1: f64 = (var_b4soitype * (s.db[1231][1] * ddt_scale));
        let eq40_e1352_d_b2: f64 = (var_b4soitype * (s.db[1231][2] * ddt_scale));
        let eq40_e1352_d_b3: f64 = (var_b4soitype * (s.db[1231][3] * ddt_scale));
        let eq40_e1352_d_b4: f64 = (var_b4soitype * (s.db[1231][4] * ddt_scale));
        let eq40_e1352_d_b5: f64 = (var_b4soitype * (s.db[1231][5] * ddt_scale));
        let eq40_e1352_d_b6: f64 = (var_b4soitype * (s.db[1231][6] * ddt_scale));
        let eq40_e1352_d_b7: f64 = (var_b4soitype * (s.db[1231][7] * ddt_scale));
        let eq40_e1352_d_b8: f64 = (var_b4soitype * (s.db[1231][8] * ddt_scale));
        (eq40_e1352, eq40_e1352_d_n0, eq40_e1352_d_n1, eq40_e1352_d_n2, eq40_e1352_d_n3, eq40_e1352_d_n4, eq40_e1352_d_n5, eq40_e1352_d_n6, eq40_e1352_d_n7, eq40_e1352_d_n8, eq40_e1352_d_n9, eq40_e1352_d_n10, eq40_e1352_d_n11, eq40_e1352_d_n12, eq40_e1352_d_b0, eq40_e1352_d_b1, eq40_e1352_d_b2, eq40_e1352_d_b3, eq40_e1352_d_b4, eq40_e1352_d_b5, eq40_e1352_d_b6, eq40_e1352_d_b7, eq40_e1352_d_b8,)
    } else {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    }
};
        let eq40_value: f64 = eq40_e1354;
        let eq40_node_derivatives: [f64; 13] = [eq40_e1354_d_n0, eq40_e1354_d_n1, eq40_e1354_d_n2, eq40_e1354_d_n3, eq40_e1354_d_n4, eq40_e1354_d_n5, eq40_e1354_d_n6, eq40_e1354_d_n7, eq40_e1354_d_n8, eq40_e1354_d_n9, eq40_e1354_d_n10, eq40_e1354_d_n11, eq40_e1354_d_n12];
        let eq40_branch_derivatives: [f64; 9] = [eq40_e1354_d_b0, eq40_e1354_d_b1, eq40_e1354_d_b2, eq40_e1354_d_b3, eq40_e1354_d_b4, eq40_e1354_d_b5, eq40_e1354_d_b6, eq40_e1354_d_b7, eq40_e1354_d_b8];
        stamper.stamp_current_dense_local(
            Some(9),
            Some(8),
            multiplicity * (eq40_value),
            &eq40_node_derivatives,
            &eq40_branch_derivatives,
            multiplicity,
        );
        let (eq41_e1362, eq41_e1362_d_n0, eq41_e1362_d_n1, eq41_e1362_d_n2, eq41_e1362_d_n3, eq41_e1362_d_n4, eq41_e1362_d_n5, eq41_e1362_d_n6, eq41_e1362_d_n7, eq41_e1362_d_n8, eq41_e1362_d_n9, eq41_e1362_d_n10, eq41_e1362_d_n11, eq41_e1362_d_n12, eq41_e1362_d_b0, eq41_e1362_d_b1, eq41_e1362_d_b2, eq41_e1362_d_b3, eq41_e1362_d_b4, eq41_e1362_d_b5, eq41_e1362_d_b6, eq41_e1362_d_b7, eq41_e1362_d_b8,) = {
    if (!s.b[1863]) {
        let eq41_e1359: f64 = ((nv9 - nv3) * s.v[697]);
        let eq41_e1359_d_n0: f64 = ((nv9 - nv3) * s.dn[697][0]);
        let eq41_e1359_d_n1: f64 = ((nv9 - nv3) * s.dn[697][1]);
        let eq41_e1359_d_n2: f64 = ((nv9 - nv3) * s.dn[697][2]);
        let eq41_e1359_d_n3: f64 = ((-s.v[697]) + ((nv9 - nv3) * s.dn[697][3]));
        let eq41_e1359_d_n4: f64 = ((nv9 - nv3) * s.dn[697][4]);
        let eq41_e1359_d_n5: f64 = ((nv9 - nv3) * s.dn[697][5]);
        let eq41_e1359_d_n6: f64 = ((nv9 - nv3) * s.dn[697][6]);
        let eq41_e1359_d_n7: f64 = ((nv9 - nv3) * s.dn[697][7]);
        let eq41_e1359_d_n8: f64 = ((nv9 - nv3) * s.dn[697][8]);
        let eq41_e1359_d_n9: f64 = (s.v[697] + ((nv9 - nv3) * s.dn[697][9]));
        let eq41_e1359_d_n10: f64 = ((nv9 - nv3) * s.dn[697][10]);
        let eq41_e1359_d_n11: f64 = ((nv9 - nv3) * s.dn[697][11]);
        let eq41_e1359_d_n12: f64 = ((nv9 - nv3) * s.dn[697][12]);
        let eq41_e1359_d_b0: f64 = ((nv9 - nv3) * s.db[697][0]);
        let eq41_e1359_d_b1: f64 = ((nv9 - nv3) * s.db[697][1]);
        let eq41_e1359_d_b2: f64 = ((nv9 - nv3) * s.db[697][2]);
        let eq41_e1359_d_b3: f64 = ((nv9 - nv3) * s.db[697][3]);
        let eq41_e1359_d_b4: f64 = ((nv9 - nv3) * s.db[697][4]);
        let eq41_e1359_d_b5: f64 = ((nv9 - nv3) * s.db[697][5]);
        let eq41_e1359_d_b6: f64 = ((nv9 - nv3) * s.db[697][6]);
        let eq41_e1359_d_b7: f64 = ((nv9 - nv3) * s.db[697][7]);
        let eq41_e1359_d_b8: f64 = ((nv9 - nv3) * s.db[697][8]);
        let eq41_e1360: f64 = eval_ddt(ddt_state_current, ddt_state_previous, ddt_state_older, ddt_state_initialized, ddt_derivative_current, ddt_derivative_previous, ddt_active, ddt_scale, ddt_previous_value_scale, ddt_older_value_scale, ddt_previous_derivative_scale, 11, eq41_e1359);
        (eq41_e1360, (eq41_e1359_d_n0 * ddt_scale), (eq41_e1359_d_n1 * ddt_scale), (eq41_e1359_d_n2 * ddt_scale), (eq41_e1359_d_n3 * ddt_scale), (eq41_e1359_d_n4 * ddt_scale), (eq41_e1359_d_n5 * ddt_scale), (eq41_e1359_d_n6 * ddt_scale), (eq41_e1359_d_n7 * ddt_scale), (eq41_e1359_d_n8 * ddt_scale), (eq41_e1359_d_n9 * ddt_scale), (eq41_e1359_d_n10 * ddt_scale), (eq41_e1359_d_n11 * ddt_scale), (eq41_e1359_d_n12 * ddt_scale), (eq41_e1359_d_b0 * ddt_scale), (eq41_e1359_d_b1 * ddt_scale), (eq41_e1359_d_b2 * ddt_scale), (eq41_e1359_d_b3 * ddt_scale), (eq41_e1359_d_b4 * ddt_scale), (eq41_e1359_d_b5 * ddt_scale), (eq41_e1359_d_b6 * ddt_scale), (eq41_e1359_d_b7 * ddt_scale), (eq41_e1359_d_b8 * ddt_scale),)
    } else {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    }
};
        let eq41_value: f64 = eq41_e1362;
        let eq41_node_derivatives: [f64; 13] = [eq41_e1362_d_n0, eq41_e1362_d_n1, eq41_e1362_d_n2, eq41_e1362_d_n3, eq41_e1362_d_n4, eq41_e1362_d_n5, eq41_e1362_d_n6, eq41_e1362_d_n7, eq41_e1362_d_n8, eq41_e1362_d_n9, eq41_e1362_d_n10, eq41_e1362_d_n11, eq41_e1362_d_n12];
        let eq41_branch_derivatives: [f64; 9] = [eq41_e1362_d_b0, eq41_e1362_d_b1, eq41_e1362_d_b2, eq41_e1362_d_b3, eq41_e1362_d_b4, eq41_e1362_d_b5, eq41_e1362_d_b6, eq41_e1362_d_b7, eq41_e1362_d_b8];
        stamper.stamp_current_dense_local(
            Some(9),
            Some(3),
            multiplicity * (eq41_value),
            &eq41_node_derivatives,
            &eq41_branch_derivatives,
            multiplicity,
        );
        let eq42_e1364: f64 = eval_ddt(ddt_state_current, ddt_state_previous, ddt_state_older, ddt_state_initialized, ddt_derivative_current, ddt_derivative_previous, ddt_active, ddt_scale, ddt_previous_value_scale, ddt_older_value_scale, ddt_previous_derivative_scale, 12, s.v[449]);
        let eq42_value: f64 = eq42_e1364;
        stamper.stamp_current_dense_local(
            Some(7),
            Some(3),
            multiplicity * (eq42_value),
            &s.dn[449],
            &s.db[449],
            (multiplicity) * (ddt_scale),
        );
        let eq43_e1366: f64 = eval_ddt(ddt_state_current, ddt_state_previous, ddt_state_older, ddt_state_initialized, ddt_derivative_current, ddt_derivative_previous, ddt_active, ddt_scale, ddt_previous_value_scale, ddt_older_value_scale, ddt_previous_derivative_scale, 13, s.v[448]);
        let eq43_value: f64 = eq43_e1366;
        stamper.stamp_current_dense_local(
            Some(8),
            Some(3),
            multiplicity * (eq43_value),
            &s.dn[448],
            &s.db[448],
            (multiplicity) * (ddt_scale),
        );
    }

    pub(super) fn stamp_transient_equations_block_2(
        ctx: &GeneratedEvalContext<'_>,
        stamper: &mut GeneratedStamper<'_>,
        s: &mut Scratch,
        nodes: &[usize; Instance::NODE_COUNT],
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
        var_b4soigcrg: f64,
        var_b4soigcrg_db0: f64,
        var_b4soigcrg_db1: f64,
        var_b4soigcrg_db2: f64,
        var_b4soigcrg_db3: f64,
        var_b4soigcrg_db4: f64,
        var_b4soigcrg_db5: f64,
        var_b4soigcrg_db6: f64,
        var_b4soigcrg_db7: f64,
        var_b4soigcrg_db8: f64,
        var_b4soigcrg_dn0: f64,
        var_b4soigcrg_dn1: f64,
        var_b4soigcrg_dn10: f64,
        var_b4soigcrg_dn11: f64,
        var_b4soigcrg_dn12: f64,
        var_b4soigcrg_dn2: f64,
        var_b4soigcrg_dn3: f64,
        var_b4soigcrg_dn4: f64,
        var_b4soigcrg_dn5: f64,
        var_b4soigcrg_dn6: f64,
        var_b4soigcrg_dn7: f64,
        var_b4soigcrg_dn8: f64,
        var_b4soigcrg_dn9: f64,
        var_deltemp: f64,
        var_deltemp_db0: f64,
        var_deltemp_db1: f64,
        var_deltemp_db2: f64,
        var_deltemp_db3: f64,
        var_deltemp_db4: f64,
        var_deltemp_db5: f64,
        var_deltemp_db6: f64,
        var_deltemp_db7: f64,
        var_deltemp_db8: f64,
        var_deltemp_dn0: f64,
        var_deltemp_dn1: f64,
        var_deltemp_dn10: f64,
        var_deltemp_dn11: f64,
        var_deltemp_dn12: f64,
        var_deltemp_dn2: f64,
        var_deltemp_dn3: f64,
        var_deltemp_dn4: f64,
        var_deltemp_dn5: f64,
        var_deltemp_dn6: f64,
        var_deltemp_dn7: f64,
        var_deltemp_dn8: f64,
        var_deltemp_dn9: f64,
        var_guard1830: f64,
        var_ids_1: f64,
        var_ids_1_db0: f64,
        var_ids_1_db1: f64,
        var_ids_1_db2: f64,
        var_ids_1_db3: f64,
        var_ids_1_db4: f64,
        var_ids_1_db5: f64,
        var_ids_1_db6: f64,
        var_ids_1_db7: f64,
        var_ids_1_db8: f64,
        var_ids_1_dn0: f64,
        var_ids_1_dn1: f64,
        var_ids_1_dn10: f64,
        var_ids_1_dn11: f64,
        var_ids_1_dn12: f64,
        var_ids_1_dn2: f64,
        var_ids_1_dn3: f64,
        var_ids_1_dn4: f64,
        var_ids_1_dn5: f64,
        var_ids_1_dn6: f64,
        var_ids_1_dn7: f64,
        var_ids_1_dn8: f64,
        var_ids_1_dn9: f64,
        var_vds_1: f64,
        var_vds_1_db0: f64,
        var_vds_1_db1: f64,
        var_vds_1_db2: f64,
        var_vds_1_db3: f64,
        var_vds_1_db4: f64,
        var_vds_1_db5: f64,
        var_vds_1_db6: f64,
        var_vds_1_db7: f64,
        var_vds_1_db8: f64,
        var_vds_1_dn0: f64,
        var_vds_1_dn1: f64,
        var_vds_1_dn10: f64,
        var_vds_1_dn11: f64,
        var_vds_1_dn12: f64,
        var_vds_1_dn2: f64,
        var_vds_1_dn3: f64,
        var_vds_1_dn4: f64,
        var_vds_1_dn5: f64,
        var_vds_1_dn6: f64,
        var_vds_1_dn7: f64,
        var_vds_1_dn8: f64,
        var_vds_1_dn9: f64,
    ) {
        let nv9 = ctx.node_voltage(nodes[9]);
        let nv10 = ctx.node_voltage(nodes[10]);
        let (eq48_e1398, eq48_e1398_d_n0, eq48_e1398_d_n1, eq48_e1398_d_n2, eq48_e1398_d_n3, eq48_e1398_d_n4, eq48_e1398_d_n5, eq48_e1398_d_n6, eq48_e1398_d_n7, eq48_e1398_d_n8, eq48_e1398_d_n9, eq48_e1398_d_n10, eq48_e1398_d_n11, eq48_e1398_d_n12, eq48_e1398_d_b0, eq48_e1398_d_b1, eq48_e1398_d_b2, eq48_e1398_d_b3, eq48_e1398_d_b4, eq48_e1398_d_b5, eq48_e1398_d_b6, eq48_e1398_d_b7, eq48_e1398_d_b8,) = {
    if (var_guard1830 == 0.0) {
        let eq48_e1396: f64 = ((nv10 - nv9) * var_b4soigcrg);
        let eq48_e1396_d_n0: f64 = ((nv10 - nv9) * var_b4soigcrg_dn0);
        let eq48_e1396_d_n1: f64 = ((nv10 - nv9) * var_b4soigcrg_dn1);
        let eq48_e1396_d_n2: f64 = ((nv10 - nv9) * var_b4soigcrg_dn2);
        let eq48_e1396_d_n3: f64 = ((nv10 - nv9) * var_b4soigcrg_dn3);
        let eq48_e1396_d_n4: f64 = ((nv10 - nv9) * var_b4soigcrg_dn4);
        let eq48_e1396_d_n5: f64 = ((nv10 - nv9) * var_b4soigcrg_dn5);
        let eq48_e1396_d_n6: f64 = ((nv10 - nv9) * var_b4soigcrg_dn6);
        let eq48_e1396_d_n7: f64 = ((nv10 - nv9) * var_b4soigcrg_dn7);
        let eq48_e1396_d_n8: f64 = ((nv10 - nv9) * var_b4soigcrg_dn8);
        let eq48_e1396_d_n9: f64 = ((-var_b4soigcrg) + ((nv10 - nv9) * var_b4soigcrg_dn9));
        let eq48_e1396_d_n10: f64 = (var_b4soigcrg + ((nv10 - nv9) * var_b4soigcrg_dn10));
        let eq48_e1396_d_n11: f64 = ((nv10 - nv9) * var_b4soigcrg_dn11);
        let eq48_e1396_d_n12: f64 = ((nv10 - nv9) * var_b4soigcrg_dn12);
        let eq48_e1396_d_b0: f64 = ((nv10 - nv9) * var_b4soigcrg_db0);
        let eq48_e1396_d_b1: f64 = ((nv10 - nv9) * var_b4soigcrg_db1);
        let eq48_e1396_d_b2: f64 = ((nv10 - nv9) * var_b4soigcrg_db2);
        let eq48_e1396_d_b3: f64 = ((nv10 - nv9) * var_b4soigcrg_db3);
        let eq48_e1396_d_b4: f64 = ((nv10 - nv9) * var_b4soigcrg_db4);
        let eq48_e1396_d_b5: f64 = ((nv10 - nv9) * var_b4soigcrg_db5);
        let eq48_e1396_d_b6: f64 = ((nv10 - nv9) * var_b4soigcrg_db6);
        let eq48_e1396_d_b7: f64 = ((nv10 - nv9) * var_b4soigcrg_db7);
        let eq48_e1396_d_b8: f64 = ((nv10 - nv9) * var_b4soigcrg_db8);
        (eq48_e1396, eq48_e1396_d_n0, eq48_e1396_d_n1, eq48_e1396_d_n2, eq48_e1396_d_n3, eq48_e1396_d_n4, eq48_e1396_d_n5, eq48_e1396_d_n6, eq48_e1396_d_n7, eq48_e1396_d_n8, eq48_e1396_d_n9, eq48_e1396_d_n10, eq48_e1396_d_n11, eq48_e1396_d_n12, eq48_e1396_d_b0, eq48_e1396_d_b1, eq48_e1396_d_b2, eq48_e1396_d_b3, eq48_e1396_d_b4, eq48_e1396_d_b5, eq48_e1396_d_b6, eq48_e1396_d_b7, eq48_e1396_d_b8,)
    } else {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    }
};
        let eq48_value: f64 = eq48_e1398;
        let eq48_node_derivatives: [f64; 13] = [eq48_e1398_d_n0, eq48_e1398_d_n1, eq48_e1398_d_n2, eq48_e1398_d_n3, eq48_e1398_d_n4, eq48_e1398_d_n5, eq48_e1398_d_n6, eq48_e1398_d_n7, eq48_e1398_d_n8, eq48_e1398_d_n9, eq48_e1398_d_n10, eq48_e1398_d_n11, eq48_e1398_d_n12];
        let eq48_branch_derivatives: [f64; 9] = [eq48_e1398_d_b0, eq48_e1398_d_b1, eq48_e1398_d_b2, eq48_e1398_d_b3, eq48_e1398_d_b4, eq48_e1398_d_b5, eq48_e1398_d_b6, eq48_e1398_d_b7, eq48_e1398_d_b8];
        stamper.stamp_current_dense_local(
            Some(10),
            Some(9),
            multiplicity * (eq48_value),
            &eq48_node_derivatives,
            &eq48_branch_derivatives,
            multiplicity,
        );
        let (eq57_e1469, eq57_e1469_d_n0, eq57_e1469_d_n1, eq57_e1469_d_n2, eq57_e1469_d_n3, eq57_e1469_d_n4, eq57_e1469_d_n5, eq57_e1469_d_n6, eq57_e1469_d_n7, eq57_e1469_d_n8, eq57_e1469_d_n9, eq57_e1469_d_n10, eq57_e1469_d_n11, eq57_e1469_d_n12, eq57_e1469_d_b0, eq57_e1469_d_b1, eq57_e1469_d_b2, eq57_e1469_d_b3, eq57_e1469_d_b4, eq57_e1469_d_b5, eq57_e1469_d_b6, eq57_e1469_d_b7, eq57_e1469_d_b8,) = {
    if s.b[1869] {
        let eq57_e1461: f64 = (-var_ids_1);
        let eq57_e1463: f64 = (eq57_e1461 * var_vds_1);
        let eq57_e1463_d_n0: f64 = (((-var_ids_1_dn0) * var_vds_1) + (eq57_e1461 * var_vds_1_dn0));
        let eq57_e1463_d_n1: f64 = (((-var_ids_1_dn1) * var_vds_1) + (eq57_e1461 * var_vds_1_dn1));
        let eq57_e1463_d_n2: f64 = (((-var_ids_1_dn2) * var_vds_1) + (eq57_e1461 * var_vds_1_dn2));
        let eq57_e1463_d_n3: f64 = (((-var_ids_1_dn3) * var_vds_1) + (eq57_e1461 * var_vds_1_dn3));
        let eq57_e1463_d_n4: f64 = (((-var_ids_1_dn4) * var_vds_1) + (eq57_e1461 * var_vds_1_dn4));
        let eq57_e1463_d_n5: f64 = (((-var_ids_1_dn5) * var_vds_1) + (eq57_e1461 * var_vds_1_dn5));
        let eq57_e1463_d_n6: f64 = (((-var_ids_1_dn6) * var_vds_1) + (eq57_e1461 * var_vds_1_dn6));
        let eq57_e1463_d_n7: f64 = (((-var_ids_1_dn7) * var_vds_1) + (eq57_e1461 * var_vds_1_dn7));
        let eq57_e1463_d_n8: f64 = (((-var_ids_1_dn8) * var_vds_1) + (eq57_e1461 * var_vds_1_dn8));
        let eq57_e1463_d_n9: f64 = (((-var_ids_1_dn9) * var_vds_1) + (eq57_e1461 * var_vds_1_dn9));
        let eq57_e1463_d_n10: f64 = (((-var_ids_1_dn10) * var_vds_1) + (eq57_e1461 * var_vds_1_dn10));
        let eq57_e1463_d_n11: f64 = (((-var_ids_1_dn11) * var_vds_1) + (eq57_e1461 * var_vds_1_dn11));
        let eq57_e1463_d_n12: f64 = (((-var_ids_1_dn12) * var_vds_1) + (eq57_e1461 * var_vds_1_dn12));
        let eq57_e1463_d_b0: f64 = (((-var_ids_1_db0) * var_vds_1) + (eq57_e1461 * var_vds_1_db0));
        let eq57_e1463_d_b1: f64 = (((-var_ids_1_db1) * var_vds_1) + (eq57_e1461 * var_vds_1_db1));
        let eq57_e1463_d_b2: f64 = (((-var_ids_1_db2) * var_vds_1) + (eq57_e1461 * var_vds_1_db2));
        let eq57_e1463_d_b3: f64 = (((-var_ids_1_db3) * var_vds_1) + (eq57_e1461 * var_vds_1_db3));
        let eq57_e1463_d_b4: f64 = (((-var_ids_1_db4) * var_vds_1) + (eq57_e1461 * var_vds_1_db4));
        let eq57_e1463_d_b5: f64 = (((-var_ids_1_db5) * var_vds_1) + (eq57_e1461 * var_vds_1_db5));
        let eq57_e1463_d_b6: f64 = (((-var_ids_1_db6) * var_vds_1) + (eq57_e1461 * var_vds_1_db6));
        let eq57_e1463_d_b7: f64 = (((-var_ids_1_db7) * var_vds_1) + (eq57_e1461 * var_vds_1_db7));
        let eq57_e1463_d_b8: f64 = (((-var_ids_1_db8) * var_vds_1) + (eq57_e1461 * var_vds_1_db8));
        let eq57_e1466: f64 = (var_deltemp / s.v[527]);
        let __rspice_inv_cse_0: f64 = 1.0 / (s.v[527] * s.v[527]);
        let eq57_e1466_d_n0: f64 = (((var_deltemp_dn0 * s.v[527]) - (var_deltemp * s.dn[527][0])) * __rspice_inv_cse_0);
        let eq57_e1466_d_n1: f64 = (((var_deltemp_dn1 * s.v[527]) - (var_deltemp * s.dn[527][1])) * __rspice_inv_cse_0);
        let eq57_e1466_d_n2: f64 = (((var_deltemp_dn2 * s.v[527]) - (var_deltemp * s.dn[527][2])) * __rspice_inv_cse_0);
        let eq57_e1466_d_n3: f64 = (((var_deltemp_dn3 * s.v[527]) - (var_deltemp * s.dn[527][3])) * __rspice_inv_cse_0);
        let eq57_e1466_d_n4: f64 = (((var_deltemp_dn4 * s.v[527]) - (var_deltemp * s.dn[527][4])) * __rspice_inv_cse_0);
        let eq57_e1466_d_n5: f64 = (((var_deltemp_dn5 * s.v[527]) - (var_deltemp * s.dn[527][5])) * __rspice_inv_cse_0);
        let eq57_e1466_d_n6: f64 = (((var_deltemp_dn6 * s.v[527]) - (var_deltemp * s.dn[527][6])) * __rspice_inv_cse_0);
        let eq57_e1466_d_n7: f64 = (((var_deltemp_dn7 * s.v[527]) - (var_deltemp * s.dn[527][7])) * __rspice_inv_cse_0);
        let eq57_e1466_d_n8: f64 = (((var_deltemp_dn8 * s.v[527]) - (var_deltemp * s.dn[527][8])) * __rspice_inv_cse_0);
        let eq57_e1466_d_n9: f64 = (((var_deltemp_dn9 * s.v[527]) - (var_deltemp * s.dn[527][9])) * __rspice_inv_cse_0);
        let eq57_e1466_d_n10: f64 = (((var_deltemp_dn10 * s.v[527]) - (var_deltemp * s.dn[527][10])) * __rspice_inv_cse_0);
        let eq57_e1466_d_n11: f64 = (((var_deltemp_dn11 * s.v[527]) - (var_deltemp * s.dn[527][11])) * __rspice_inv_cse_0);
        let eq57_e1466_d_n12: f64 = (((var_deltemp_dn12 * s.v[527]) - (var_deltemp * s.dn[527][12])) * __rspice_inv_cse_0);
        let eq57_e1466_d_b0: f64 = (((var_deltemp_db0 * s.v[527]) - (var_deltemp * s.db[527][0])) * __rspice_inv_cse_0);
        let eq57_e1466_d_b1: f64 = (((var_deltemp_db1 * s.v[527]) - (var_deltemp * s.db[527][1])) * __rspice_inv_cse_0);
        let eq57_e1466_d_b2: f64 = (((var_deltemp_db2 * s.v[527]) - (var_deltemp * s.db[527][2])) * __rspice_inv_cse_0);
        let eq57_e1466_d_b3: f64 = (((var_deltemp_db3 * s.v[527]) - (var_deltemp * s.db[527][3])) * __rspice_inv_cse_0);
        let eq57_e1466_d_b4: f64 = (((var_deltemp_db4 * s.v[527]) - (var_deltemp * s.db[527][4])) * __rspice_inv_cse_0);
        let eq57_e1466_d_b5: f64 = (((var_deltemp_db5 * s.v[527]) - (var_deltemp * s.db[527][5])) * __rspice_inv_cse_0);
        let eq57_e1466_d_b6: f64 = (((var_deltemp_db6 * s.v[527]) - (var_deltemp * s.db[527][6])) * __rspice_inv_cse_0);
        let eq57_e1466_d_b7: f64 = (((var_deltemp_db7 * s.v[527]) - (var_deltemp * s.db[527][7])) * __rspice_inv_cse_0);
        let eq57_e1466_d_b8: f64 = (((var_deltemp_db8 * s.v[527]) - (var_deltemp * s.db[527][8])) * __rspice_inv_cse_0);
        let eq57_e1467: f64 = (eq57_e1463 + eq57_e1466);
        let eq57_e1467_d_n0: f64 = (eq57_e1463_d_n0 + eq57_e1466_d_n0);
        let eq57_e1467_d_n1: f64 = (eq57_e1463_d_n1 + eq57_e1466_d_n1);
        let eq57_e1467_d_n2: f64 = (eq57_e1463_d_n2 + eq57_e1466_d_n2);
        let eq57_e1467_d_n3: f64 = (eq57_e1463_d_n3 + eq57_e1466_d_n3);
        let eq57_e1467_d_n4: f64 = (eq57_e1463_d_n4 + eq57_e1466_d_n4);
        let eq57_e1467_d_n5: f64 = (eq57_e1463_d_n5 + eq57_e1466_d_n5);
        let eq57_e1467_d_n6: f64 = (eq57_e1463_d_n6 + eq57_e1466_d_n6);
        let eq57_e1467_d_n7: f64 = (eq57_e1463_d_n7 + eq57_e1466_d_n7);
        let eq57_e1467_d_n8: f64 = (eq57_e1463_d_n8 + eq57_e1466_d_n8);
        let eq57_e1467_d_n9: f64 = (eq57_e1463_d_n9 + eq57_e1466_d_n9);
        let eq57_e1467_d_n10: f64 = (eq57_e1463_d_n10 + eq57_e1466_d_n10);
        let eq57_e1467_d_n11: f64 = (eq57_e1463_d_n11 + eq57_e1466_d_n11);
        let eq57_e1467_d_n12: f64 = (eq57_e1463_d_n12 + eq57_e1466_d_n12);
        let eq57_e1467_d_b0: f64 = (eq57_e1463_d_b0 + eq57_e1466_d_b0);
        let eq57_e1467_d_b1: f64 = (eq57_e1463_d_b1 + eq57_e1466_d_b1);
        let eq57_e1467_d_b2: f64 = (eq57_e1463_d_b2 + eq57_e1466_d_b2);
        let eq57_e1467_d_b3: f64 = (eq57_e1463_d_b3 + eq57_e1466_d_b3);
        let eq57_e1467_d_b4: f64 = (eq57_e1463_d_b4 + eq57_e1466_d_b4);
        let eq57_e1467_d_b5: f64 = (eq57_e1463_d_b5 + eq57_e1466_d_b5);
        let eq57_e1467_d_b6: f64 = (eq57_e1463_d_b6 + eq57_e1466_d_b6);
        let eq57_e1467_d_b7: f64 = (eq57_e1463_d_b7 + eq57_e1466_d_b7);
        let eq57_e1467_d_b8: f64 = (eq57_e1463_d_b8 + eq57_e1466_d_b8);
        (eq57_e1467, eq57_e1467_d_n0, eq57_e1467_d_n1, eq57_e1467_d_n2, eq57_e1467_d_n3, eq57_e1467_d_n4, eq57_e1467_d_n5, eq57_e1467_d_n6, eq57_e1467_d_n7, eq57_e1467_d_n8, eq57_e1467_d_n9, eq57_e1467_d_n10, eq57_e1467_d_n11, eq57_e1467_d_n12, eq57_e1467_d_b0, eq57_e1467_d_b1, eq57_e1467_d_b2, eq57_e1467_d_b3, eq57_e1467_d_b4, eq57_e1467_d_b5, eq57_e1467_d_b6, eq57_e1467_d_b7, eq57_e1467_d_b8,)
    } else {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    }
};
        let eq57_value: f64 = eq57_e1469;
        let eq57_node_derivatives: [f64; 13] = [eq57_e1469_d_n0, eq57_e1469_d_n1, eq57_e1469_d_n2, eq57_e1469_d_n3, eq57_e1469_d_n4, eq57_e1469_d_n5, eq57_e1469_d_n6, eq57_e1469_d_n7, eq57_e1469_d_n8, eq57_e1469_d_n9, eq57_e1469_d_n10, eq57_e1469_d_n11, eq57_e1469_d_n12];
        let eq57_branch_derivatives: [f64; 9] = [eq57_e1469_d_b0, eq57_e1469_d_b1, eq57_e1469_d_b2, eq57_e1469_d_b3, eq57_e1469_d_b4, eq57_e1469_d_b5, eq57_e1469_d_b6, eq57_e1469_d_b7, eq57_e1469_d_b8];
        stamper.stamp_current_dense_local(
            Some(6),
            None,
            multiplicity * (eq57_value),
            &eq57_node_derivatives,
            &eq57_branch_derivatives,
            multiplicity,
        );
        let (eq58_e1476, eq58_e1476_d_n0, eq58_e1476_d_n1, eq58_e1476_d_n2, eq58_e1476_d_n3, eq58_e1476_d_n4, eq58_e1476_d_n5, eq58_e1476_d_n6, eq58_e1476_d_n7, eq58_e1476_d_n8, eq58_e1476_d_n9, eq58_e1476_d_n10, eq58_e1476_d_n11, eq58_e1476_d_n12, eq58_e1476_d_b0, eq58_e1476_d_b1, eq58_e1476_d_b2, eq58_e1476_d_b3, eq58_e1476_d_b4, eq58_e1476_d_b5, eq58_e1476_d_b6, eq58_e1476_d_b7, eq58_e1476_d_b8,) = {
    if s.b[1869] {
        let eq58_e1473: f64 = (var_deltemp * s.v[528]);
        let eq58_e1473_d_n0: f64 = ((var_deltemp_dn0 * s.v[528]) + (var_deltemp * s.dn[528][0]));
        let eq58_e1473_d_n1: f64 = ((var_deltemp_dn1 * s.v[528]) + (var_deltemp * s.dn[528][1]));
        let eq58_e1473_d_n2: f64 = ((var_deltemp_dn2 * s.v[528]) + (var_deltemp * s.dn[528][2]));
        let eq58_e1473_d_n3: f64 = ((var_deltemp_dn3 * s.v[528]) + (var_deltemp * s.dn[528][3]));
        let eq58_e1473_d_n4: f64 = ((var_deltemp_dn4 * s.v[528]) + (var_deltemp * s.dn[528][4]));
        let eq58_e1473_d_n5: f64 = ((var_deltemp_dn5 * s.v[528]) + (var_deltemp * s.dn[528][5]));
        let eq58_e1473_d_n6: f64 = ((var_deltemp_dn6 * s.v[528]) + (var_deltemp * s.dn[528][6]));
        let eq58_e1473_d_n7: f64 = ((var_deltemp_dn7 * s.v[528]) + (var_deltemp * s.dn[528][7]));
        let eq58_e1473_d_n8: f64 = ((var_deltemp_dn8 * s.v[528]) + (var_deltemp * s.dn[528][8]));
        let eq58_e1473_d_n9: f64 = ((var_deltemp_dn9 * s.v[528]) + (var_deltemp * s.dn[528][9]));
        let eq58_e1473_d_n10: f64 = ((var_deltemp_dn10 * s.v[528]) + (var_deltemp * s.dn[528][10]));
        let eq58_e1473_d_n11: f64 = ((var_deltemp_dn11 * s.v[528]) + (var_deltemp * s.dn[528][11]));
        let eq58_e1473_d_n12: f64 = ((var_deltemp_dn12 * s.v[528]) + (var_deltemp * s.dn[528][12]));
        let eq58_e1473_d_b0: f64 = ((var_deltemp_db0 * s.v[528]) + (var_deltemp * s.db[528][0]));
        let eq58_e1473_d_b1: f64 = ((var_deltemp_db1 * s.v[528]) + (var_deltemp * s.db[528][1]));
        let eq58_e1473_d_b2: f64 = ((var_deltemp_db2 * s.v[528]) + (var_deltemp * s.db[528][2]));
        let eq58_e1473_d_b3: f64 = ((var_deltemp_db3 * s.v[528]) + (var_deltemp * s.db[528][3]));
        let eq58_e1473_d_b4: f64 = ((var_deltemp_db4 * s.v[528]) + (var_deltemp * s.db[528][4]));
        let eq58_e1473_d_b5: f64 = ((var_deltemp_db5 * s.v[528]) + (var_deltemp * s.db[528][5]));
        let eq58_e1473_d_b6: f64 = ((var_deltemp_db6 * s.v[528]) + (var_deltemp * s.db[528][6]));
        let eq58_e1473_d_b7: f64 = ((var_deltemp_db7 * s.v[528]) + (var_deltemp * s.db[528][7]));
        let eq58_e1473_d_b8: f64 = ((var_deltemp_db8 * s.v[528]) + (var_deltemp * s.db[528][8]));
        let eq58_e1474: f64 = eval_ddt(ddt_state_current, ddt_state_previous, ddt_state_older, ddt_state_initialized, ddt_derivative_current, ddt_derivative_previous, ddt_active, ddt_scale, ddt_previous_value_scale, ddt_older_value_scale, ddt_previous_derivative_scale, 14, eq58_e1473);
        (eq58_e1474, (eq58_e1473_d_n0 * ddt_scale), (eq58_e1473_d_n1 * ddt_scale), (eq58_e1473_d_n2 * ddt_scale), (eq58_e1473_d_n3 * ddt_scale), (eq58_e1473_d_n4 * ddt_scale), (eq58_e1473_d_n5 * ddt_scale), (eq58_e1473_d_n6 * ddt_scale), (eq58_e1473_d_n7 * ddt_scale), (eq58_e1473_d_n8 * ddt_scale), (eq58_e1473_d_n9 * ddt_scale), (eq58_e1473_d_n10 * ddt_scale), (eq58_e1473_d_n11 * ddt_scale), (eq58_e1473_d_n12 * ddt_scale), (eq58_e1473_d_b0 * ddt_scale), (eq58_e1473_d_b1 * ddt_scale), (eq58_e1473_d_b2 * ddt_scale), (eq58_e1473_d_b3 * ddt_scale), (eq58_e1473_d_b4 * ddt_scale), (eq58_e1473_d_b5 * ddt_scale), (eq58_e1473_d_b6 * ddt_scale), (eq58_e1473_d_b7 * ddt_scale), (eq58_e1473_d_b8 * ddt_scale),)
    } else {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    }
};
        let eq58_value: f64 = eq58_e1476;
        let eq58_node_derivatives: [f64; 13] = [eq58_e1476_d_n0, eq58_e1476_d_n1, eq58_e1476_d_n2, eq58_e1476_d_n3, eq58_e1476_d_n4, eq58_e1476_d_n5, eq58_e1476_d_n6, eq58_e1476_d_n7, eq58_e1476_d_n8, eq58_e1476_d_n9, eq58_e1476_d_n10, eq58_e1476_d_n11, eq58_e1476_d_n12];
        let eq58_branch_derivatives: [f64; 9] = [eq58_e1476_d_b0, eq58_e1476_d_b1, eq58_e1476_d_b2, eq58_e1476_d_b3, eq58_e1476_d_b4, eq58_e1476_d_b5, eq58_e1476_d_b6, eq58_e1476_d_b7, eq58_e1476_d_b8];
        stamper.stamp_current_dense_local(
            Some(6),
            None,
            multiplicity * (eq58_value),
            &eq58_node_derivatives,
            &eq58_branch_derivatives,
            multiplicity,
        );
    }

    pub(super) fn stamp_reactive_equations_block_0(
        ctx: &GeneratedEvalContext<'_>,
        stamper: &mut GeneratedReactiveStamper<'_>,
        s: &mut ReactiveScratch,
        nodes: &[usize; Instance::NODE_COUNT],
        branches: &[usize; Instance::BRANCH_COUNT],
        multiplicity: f64,
    ) {
        let nv3 = ctx.node_voltage(nodes[3]);
        let nv9 = ctx.node_voltage(nodes[9]);
        let nv10 = ctx.node_voltage(nodes[10]);
        let eq30_e1299_q: f64 = s.v[446];
        stamper.stamp_current_reactive_dense(
            Some(nodes[7]),
            Some(nodes[5]),
            nodes,
            &s.dn[446],
            branches,
            &s.db[446],
            multiplicity,
        );
        let eq31_e1301_q: f64 = s.v[447];
        stamper.stamp_current_reactive_dense(
            Some(nodes[8]),
            Some(nodes[5]),
            nodes,
            &s.dn[447],
            branches,
            &s.db[447],
            multiplicity,
        );
        let eq32_e1304_q: f64 = s.v[1251];
        let eq32_e1305: f64 = (s.v[36] * s.v[1251]);
        let eq32_e1305_q: f64 = (s.v[36] * eq32_e1304_q);
        stamper.stamp_current_reactive_dense(
            Some(nodes[9]),
            Some(nodes[5]),
            nodes,
            &s.dn[1251],
            branches,
            &s.db[1251],
            (multiplicity) * (s.v[36]),
        );
        let eq33_e1308_q: f64 = s.v[1255];
        let eq33_e1309: f64 = (s.v[36] * s.v[1255]);
        let eq33_e1309_q: f64 = (s.v[36] * eq33_e1308_q);
        stamper.stamp_current_reactive_dense(
            Some(nodes[3]),
            Some(nodes[5]),
            nodes,
            &s.dn[1255],
            branches,
            &s.db[1255],
            (multiplicity) * (s.v[36]),
        );
        let eq34_e1312_q: f64 = s.v[1244];
        let eq34_e1313: f64 = (s.v[36] * s.v[1244]);
        let eq34_e1313_q: f64 = (s.v[36] * eq34_e1312_q);
        stamper.stamp_current_reactive_dense(
            Some(nodes[12]),
            Some(nodes[7]),
            nodes,
            &s.dn[1244],
            branches,
            &s.db[1244],
            (multiplicity) * (s.v[36]),
        );
        let eq35_e1316_q: f64 = s.v[1245];
        let eq35_e1317: f64 = (s.v[36] * s.v[1245]);
        let eq35_e1317_q: f64 = (s.v[36] * eq35_e1316_q);
        stamper.stamp_current_reactive_dense(
            Some(nodes[11]),
            Some(nodes[8]),
            nodes,
            &s.dn[1245],
            branches,
            &s.db[1245],
            (multiplicity) * (s.v[36]),
        );
        let (eq36_e1324, eq36_e1324_d_n0, eq36_e1324_d_n1, eq36_e1324_d_n2, eq36_e1324_d_n3, eq36_e1324_d_n4, eq36_e1324_d_n5, eq36_e1324_d_n6, eq36_e1324_d_n7, eq36_e1324_d_n8, eq36_e1324_d_n9, eq36_e1324_d_n10, eq36_e1324_d_n11, eq36_e1324_d_n12, eq36_e1324_d_b0, eq36_e1324_d_b1, eq36_e1324_d_b2, eq36_e1324_d_b3, eq36_e1324_d_b4, eq36_e1324_d_b5, eq36_e1324_d_b6, eq36_e1324_d_b7, eq36_e1324_d_b8, eq36_e1324_q,) = {
    if s.b[1863] {
        let eq36_e1321_q: f64 = s.v[1230];
        let eq36_e1322: f64 = (s.v[36] * s.v[1230]);
        let eq36_e1322_q: f64 = (s.v[36] * eq36_e1321_q);
        (eq36_e1322, (s.v[36] * s.dn[1230][0]), (s.v[36] * s.dn[1230][1]), (s.v[36] * s.dn[1230][2]), (s.v[36] * s.dn[1230][3]), (s.v[36] * s.dn[1230][4]), (s.v[36] * s.dn[1230][5]), (s.v[36] * s.dn[1230][6]), (s.v[36] * s.dn[1230][7]), (s.v[36] * s.dn[1230][8]), (s.v[36] * s.dn[1230][9]), (s.v[36] * s.dn[1230][10]), (s.v[36] * s.dn[1230][11]), (s.v[36] * s.dn[1230][12]), (s.v[36] * s.db[1230][0]), (s.v[36] * s.db[1230][1]), (s.v[36] * s.db[1230][2]), (s.v[36] * s.db[1230][3]), (s.v[36] * s.db[1230][4]), (s.v[36] * s.db[1230][5]), (s.v[36] * s.db[1230][6]), (s.v[36] * s.db[1230][7]), (s.v[36] * s.db[1230][8]), eq36_e1322_q,)
    } else {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    }
};
        let eq36_reactive_node_derivatives: [f64; 13] = [eq36_e1324_d_n0, eq36_e1324_d_n1, eq36_e1324_d_n2, eq36_e1324_d_n3, eq36_e1324_d_n4, eq36_e1324_d_n5, eq36_e1324_d_n6, eq36_e1324_d_n7, eq36_e1324_d_n8, eq36_e1324_d_n9, eq36_e1324_d_n10, eq36_e1324_d_n11, eq36_e1324_d_n12];
        let eq36_reactive_branch_derivatives: [f64; 9] = [eq36_e1324_d_b0, eq36_e1324_d_b1, eq36_e1324_d_b2, eq36_e1324_d_b3, eq36_e1324_d_b4, eq36_e1324_d_b5, eq36_e1324_d_b6, eq36_e1324_d_b7, eq36_e1324_d_b8];
        stamper.stamp_current_reactive_dense(
            Some(nodes[10]),
            Some(nodes[7]),
            nodes,
            &eq36_reactive_node_derivatives,
            branches,
            &eq36_reactive_branch_derivatives,
            multiplicity,
        );
        let (eq37_e1331, eq37_e1331_d_n0, eq37_e1331_d_n1, eq37_e1331_d_n2, eq37_e1331_d_n3, eq37_e1331_d_n4, eq37_e1331_d_n5, eq37_e1331_d_n6, eq37_e1331_d_n7, eq37_e1331_d_n8, eq37_e1331_d_n9, eq37_e1331_d_n10, eq37_e1331_d_n11, eq37_e1331_d_n12, eq37_e1331_d_b0, eq37_e1331_d_b1, eq37_e1331_d_b2, eq37_e1331_d_b3, eq37_e1331_d_b4, eq37_e1331_d_b5, eq37_e1331_d_b6, eq37_e1331_d_b7, eq37_e1331_d_b8, eq37_e1331_q,) = {
    if s.b[1863] {
        let eq37_e1328_q: f64 = s.v[1231];
        let eq37_e1329: f64 = (s.v[36] * s.v[1231]);
        let eq37_e1329_q: f64 = (s.v[36] * eq37_e1328_q);
        (eq37_e1329, (s.v[36] * s.dn[1231][0]), (s.v[36] * s.dn[1231][1]), (s.v[36] * s.dn[1231][2]), (s.v[36] * s.dn[1231][3]), (s.v[36] * s.dn[1231][4]), (s.v[36] * s.dn[1231][5]), (s.v[36] * s.dn[1231][6]), (s.v[36] * s.dn[1231][7]), (s.v[36] * s.dn[1231][8]), (s.v[36] * s.dn[1231][9]), (s.v[36] * s.dn[1231][10]), (s.v[36] * s.dn[1231][11]), (s.v[36] * s.dn[1231][12]), (s.v[36] * s.db[1231][0]), (s.v[36] * s.db[1231][1]), (s.v[36] * s.db[1231][2]), (s.v[36] * s.db[1231][3]), (s.v[36] * s.db[1231][4]), (s.v[36] * s.db[1231][5]), (s.v[36] * s.db[1231][6]), (s.v[36] * s.db[1231][7]), (s.v[36] * s.db[1231][8]), eq37_e1329_q,)
    } else {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    }
};
        let eq37_reactive_node_derivatives: [f64; 13] = [eq37_e1331_d_n0, eq37_e1331_d_n1, eq37_e1331_d_n2, eq37_e1331_d_n3, eq37_e1331_d_n4, eq37_e1331_d_n5, eq37_e1331_d_n6, eq37_e1331_d_n7, eq37_e1331_d_n8, eq37_e1331_d_n9, eq37_e1331_d_n10, eq37_e1331_d_n11, eq37_e1331_d_n12];
        let eq37_reactive_branch_derivatives: [f64; 9] = [eq37_e1331_d_b0, eq37_e1331_d_b1, eq37_e1331_d_b2, eq37_e1331_d_b3, eq37_e1331_d_b4, eq37_e1331_d_b5, eq37_e1331_d_b6, eq37_e1331_d_b7, eq37_e1331_d_b8];
        stamper.stamp_current_reactive_dense(
            Some(nodes[10]),
            Some(nodes[8]),
            nodes,
            &eq37_reactive_node_derivatives,
            branches,
            &eq37_reactive_branch_derivatives,
            multiplicity,
        );
        let (eq38_e1338, eq38_e1338_d_n0, eq38_e1338_d_n1, eq38_e1338_d_n2, eq38_e1338_d_n3, eq38_e1338_d_n4, eq38_e1338_d_n5, eq38_e1338_d_n6, eq38_e1338_d_n7, eq38_e1338_d_n8, eq38_e1338_d_n9, eq38_e1338_d_n10, eq38_e1338_d_n11, eq38_e1338_d_n12, eq38_e1338_d_b0, eq38_e1338_d_b1, eq38_e1338_d_b2, eq38_e1338_d_b3, eq38_e1338_d_b4, eq38_e1338_d_b5, eq38_e1338_d_b6, eq38_e1338_d_b7, eq38_e1338_d_b8, eq38_e1338_q,) = {
    if s.b[1863] {
        let eq38_e1335: f64 = ((nv10 - nv3) * s.v[697]);
        let eq38_e1335_d_n0: f64 = ((nv10 - nv3) * s.dn[697][0]);
        let eq38_e1335_d_n1: f64 = ((nv10 - nv3) * s.dn[697][1]);
        let eq38_e1335_d_n2: f64 = ((nv10 - nv3) * s.dn[697][2]);
        let eq38_e1335_d_n3: f64 = ((-s.v[697]) + ((nv10 - nv3) * s.dn[697][3]));
        let eq38_e1335_d_n4: f64 = ((nv10 - nv3) * s.dn[697][4]);
        let eq38_e1335_d_n5: f64 = ((nv10 - nv3) * s.dn[697][5]);
        let eq38_e1335_d_n6: f64 = ((nv10 - nv3) * s.dn[697][6]);
        let eq38_e1335_d_n7: f64 = ((nv10 - nv3) * s.dn[697][7]);
        let eq38_e1335_d_n8: f64 = ((nv10 - nv3) * s.dn[697][8]);
        let eq38_e1335_d_n9: f64 = ((nv10 - nv3) * s.dn[697][9]);
        let eq38_e1335_d_n10: f64 = (s.v[697] + ((nv10 - nv3) * s.dn[697][10]));
        let eq38_e1335_d_n11: f64 = ((nv10 - nv3) * s.dn[697][11]);
        let eq38_e1335_d_n12: f64 = ((nv10 - nv3) * s.dn[697][12]);
        let eq38_e1335_d_b0: f64 = ((nv10 - nv3) * s.db[697][0]);
        let eq38_e1335_d_b1: f64 = ((nv10 - nv3) * s.db[697][1]);
        let eq38_e1335_d_b2: f64 = ((nv10 - nv3) * s.db[697][2]);
        let eq38_e1335_d_b3: f64 = ((nv10 - nv3) * s.db[697][3]);
        let eq38_e1335_d_b4: f64 = ((nv10 - nv3) * s.db[697][4]);
        let eq38_e1335_d_b5: f64 = ((nv10 - nv3) * s.db[697][5]);
        let eq38_e1335_d_b6: f64 = ((nv10 - nv3) * s.db[697][6]);
        let eq38_e1335_d_b7: f64 = ((nv10 - nv3) * s.db[697][7]);
        let eq38_e1335_d_b8: f64 = ((nv10 - nv3) * s.db[697][8]);
        let eq38_e1336_q: f64 = eq38_e1335;
        (eq38_e1335, eq38_e1335_d_n0, eq38_e1335_d_n1, eq38_e1335_d_n2, eq38_e1335_d_n3, eq38_e1335_d_n4, eq38_e1335_d_n5, eq38_e1335_d_n6, eq38_e1335_d_n7, eq38_e1335_d_n8, eq38_e1335_d_n9, eq38_e1335_d_n10, eq38_e1335_d_n11, eq38_e1335_d_n12, eq38_e1335_d_b0, eq38_e1335_d_b1, eq38_e1335_d_b2, eq38_e1335_d_b3, eq38_e1335_d_b4, eq38_e1335_d_b5, eq38_e1335_d_b6, eq38_e1335_d_b7, eq38_e1335_d_b8, eq38_e1336_q,)
    } else {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    }
};
        let eq38_reactive_node_derivatives: [f64; 13] = [eq38_e1338_d_n0, eq38_e1338_d_n1, eq38_e1338_d_n2, eq38_e1338_d_n3, eq38_e1338_d_n4, eq38_e1338_d_n5, eq38_e1338_d_n6, eq38_e1338_d_n7, eq38_e1338_d_n8, eq38_e1338_d_n9, eq38_e1338_d_n10, eq38_e1338_d_n11, eq38_e1338_d_n12];
        let eq38_reactive_branch_derivatives: [f64; 9] = [eq38_e1338_d_b0, eq38_e1338_d_b1, eq38_e1338_d_b2, eq38_e1338_d_b3, eq38_e1338_d_b4, eq38_e1338_d_b5, eq38_e1338_d_b6, eq38_e1338_d_b7, eq38_e1338_d_b8];
        stamper.stamp_current_reactive_dense(
            Some(nodes[10]),
            Some(nodes[3]),
            nodes,
            &eq38_reactive_node_derivatives,
            branches,
            &eq38_reactive_branch_derivatives,
            multiplicity,
        );
        let (eq39_e1346, eq39_e1346_d_n0, eq39_e1346_d_n1, eq39_e1346_d_n2, eq39_e1346_d_n3, eq39_e1346_d_n4, eq39_e1346_d_n5, eq39_e1346_d_n6, eq39_e1346_d_n7, eq39_e1346_d_n8, eq39_e1346_d_n9, eq39_e1346_d_n10, eq39_e1346_d_n11, eq39_e1346_d_n12, eq39_e1346_d_b0, eq39_e1346_d_b1, eq39_e1346_d_b2, eq39_e1346_d_b3, eq39_e1346_d_b4, eq39_e1346_d_b5, eq39_e1346_d_b6, eq39_e1346_d_b7, eq39_e1346_d_b8, eq39_e1346_q,) = {
    if (!s.b[1863]) {
        let eq39_e1343_q: f64 = s.v[1230];
        let eq39_e1344: f64 = (s.v[36] * s.v[1230]);
        let eq39_e1344_q: f64 = (s.v[36] * eq39_e1343_q);
        (eq39_e1344, (s.v[36] * s.dn[1230][0]), (s.v[36] * s.dn[1230][1]), (s.v[36] * s.dn[1230][2]), (s.v[36] * s.dn[1230][3]), (s.v[36] * s.dn[1230][4]), (s.v[36] * s.dn[1230][5]), (s.v[36] * s.dn[1230][6]), (s.v[36] * s.dn[1230][7]), (s.v[36] * s.dn[1230][8]), (s.v[36] * s.dn[1230][9]), (s.v[36] * s.dn[1230][10]), (s.v[36] * s.dn[1230][11]), (s.v[36] * s.dn[1230][12]), (s.v[36] * s.db[1230][0]), (s.v[36] * s.db[1230][1]), (s.v[36] * s.db[1230][2]), (s.v[36] * s.db[1230][3]), (s.v[36] * s.db[1230][4]), (s.v[36] * s.db[1230][5]), (s.v[36] * s.db[1230][6]), (s.v[36] * s.db[1230][7]), (s.v[36] * s.db[1230][8]), eq39_e1344_q,)
    } else {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    }
};
        let eq39_reactive_node_derivatives: [f64; 13] = [eq39_e1346_d_n0, eq39_e1346_d_n1, eq39_e1346_d_n2, eq39_e1346_d_n3, eq39_e1346_d_n4, eq39_e1346_d_n5, eq39_e1346_d_n6, eq39_e1346_d_n7, eq39_e1346_d_n8, eq39_e1346_d_n9, eq39_e1346_d_n10, eq39_e1346_d_n11, eq39_e1346_d_n12];
        let eq39_reactive_branch_derivatives: [f64; 9] = [eq39_e1346_d_b0, eq39_e1346_d_b1, eq39_e1346_d_b2, eq39_e1346_d_b3, eq39_e1346_d_b4, eq39_e1346_d_b5, eq39_e1346_d_b6, eq39_e1346_d_b7, eq39_e1346_d_b8];
        stamper.stamp_current_reactive_dense(
            Some(nodes[9]),
            Some(nodes[7]),
            nodes,
            &eq39_reactive_node_derivatives,
            branches,
            &eq39_reactive_branch_derivatives,
            multiplicity,
        );
        let (eq40_e1354, eq40_e1354_d_n0, eq40_e1354_d_n1, eq40_e1354_d_n2, eq40_e1354_d_n3, eq40_e1354_d_n4, eq40_e1354_d_n5, eq40_e1354_d_n6, eq40_e1354_d_n7, eq40_e1354_d_n8, eq40_e1354_d_n9, eq40_e1354_d_n10, eq40_e1354_d_n11, eq40_e1354_d_n12, eq40_e1354_d_b0, eq40_e1354_d_b1, eq40_e1354_d_b2, eq40_e1354_d_b3, eq40_e1354_d_b4, eq40_e1354_d_b5, eq40_e1354_d_b6, eq40_e1354_d_b7, eq40_e1354_d_b8, eq40_e1354_q,) = {
    if (!s.b[1863]) {
        let eq40_e1351_q: f64 = s.v[1231];
        let eq40_e1352: f64 = (s.v[36] * s.v[1231]);
        let eq40_e1352_q: f64 = (s.v[36] * eq40_e1351_q);
        (eq40_e1352, (s.v[36] * s.dn[1231][0]), (s.v[36] * s.dn[1231][1]), (s.v[36] * s.dn[1231][2]), (s.v[36] * s.dn[1231][3]), (s.v[36] * s.dn[1231][4]), (s.v[36] * s.dn[1231][5]), (s.v[36] * s.dn[1231][6]), (s.v[36] * s.dn[1231][7]), (s.v[36] * s.dn[1231][8]), (s.v[36] * s.dn[1231][9]), (s.v[36] * s.dn[1231][10]), (s.v[36] * s.dn[1231][11]), (s.v[36] * s.dn[1231][12]), (s.v[36] * s.db[1231][0]), (s.v[36] * s.db[1231][1]), (s.v[36] * s.db[1231][2]), (s.v[36] * s.db[1231][3]), (s.v[36] * s.db[1231][4]), (s.v[36] * s.db[1231][5]), (s.v[36] * s.db[1231][6]), (s.v[36] * s.db[1231][7]), (s.v[36] * s.db[1231][8]), eq40_e1352_q,)
    } else {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    }
};
        let eq40_reactive_node_derivatives: [f64; 13] = [eq40_e1354_d_n0, eq40_e1354_d_n1, eq40_e1354_d_n2, eq40_e1354_d_n3, eq40_e1354_d_n4, eq40_e1354_d_n5, eq40_e1354_d_n6, eq40_e1354_d_n7, eq40_e1354_d_n8, eq40_e1354_d_n9, eq40_e1354_d_n10, eq40_e1354_d_n11, eq40_e1354_d_n12];
        let eq40_reactive_branch_derivatives: [f64; 9] = [eq40_e1354_d_b0, eq40_e1354_d_b1, eq40_e1354_d_b2, eq40_e1354_d_b3, eq40_e1354_d_b4, eq40_e1354_d_b5, eq40_e1354_d_b6, eq40_e1354_d_b7, eq40_e1354_d_b8];
        stamper.stamp_current_reactive_dense(
            Some(nodes[9]),
            Some(nodes[8]),
            nodes,
            &eq40_reactive_node_derivatives,
            branches,
            &eq40_reactive_branch_derivatives,
            multiplicity,
        );
        let (eq41_e1362, eq41_e1362_d_n0, eq41_e1362_d_n1, eq41_e1362_d_n2, eq41_e1362_d_n3, eq41_e1362_d_n4, eq41_e1362_d_n5, eq41_e1362_d_n6, eq41_e1362_d_n7, eq41_e1362_d_n8, eq41_e1362_d_n9, eq41_e1362_d_n10, eq41_e1362_d_n11, eq41_e1362_d_n12, eq41_e1362_d_b0, eq41_e1362_d_b1, eq41_e1362_d_b2, eq41_e1362_d_b3, eq41_e1362_d_b4, eq41_e1362_d_b5, eq41_e1362_d_b6, eq41_e1362_d_b7, eq41_e1362_d_b8, eq41_e1362_q,) = {
    if (!s.b[1863]) {
        let eq41_e1359: f64 = ((nv9 - nv3) * s.v[697]);
        let eq41_e1359_d_n0: f64 = ((nv9 - nv3) * s.dn[697][0]);
        let eq41_e1359_d_n1: f64 = ((nv9 - nv3) * s.dn[697][1]);
        let eq41_e1359_d_n2: f64 = ((nv9 - nv3) * s.dn[697][2]);
        let eq41_e1359_d_n3: f64 = ((-s.v[697]) + ((nv9 - nv3) * s.dn[697][3]));
        let eq41_e1359_d_n4: f64 = ((nv9 - nv3) * s.dn[697][4]);
        let eq41_e1359_d_n5: f64 = ((nv9 - nv3) * s.dn[697][5]);
        let eq41_e1359_d_n6: f64 = ((nv9 - nv3) * s.dn[697][6]);
        let eq41_e1359_d_n7: f64 = ((nv9 - nv3) * s.dn[697][7]);
        let eq41_e1359_d_n8: f64 = ((nv9 - nv3) * s.dn[697][8]);
        let eq41_e1359_d_n9: f64 = (s.v[697] + ((nv9 - nv3) * s.dn[697][9]));
        let eq41_e1359_d_n10: f64 = ((nv9 - nv3) * s.dn[697][10]);
        let eq41_e1359_d_n11: f64 = ((nv9 - nv3) * s.dn[697][11]);
        let eq41_e1359_d_n12: f64 = ((nv9 - nv3) * s.dn[697][12]);
        let eq41_e1359_d_b0: f64 = ((nv9 - nv3) * s.db[697][0]);
        let eq41_e1359_d_b1: f64 = ((nv9 - nv3) * s.db[697][1]);
        let eq41_e1359_d_b2: f64 = ((nv9 - nv3) * s.db[697][2]);
        let eq41_e1359_d_b3: f64 = ((nv9 - nv3) * s.db[697][3]);
        let eq41_e1359_d_b4: f64 = ((nv9 - nv3) * s.db[697][4]);
        let eq41_e1359_d_b5: f64 = ((nv9 - nv3) * s.db[697][5]);
        let eq41_e1359_d_b6: f64 = ((nv9 - nv3) * s.db[697][6]);
        let eq41_e1359_d_b7: f64 = ((nv9 - nv3) * s.db[697][7]);
        let eq41_e1359_d_b8: f64 = ((nv9 - nv3) * s.db[697][8]);
        let eq41_e1360_q: f64 = eq41_e1359;
        (eq41_e1359, eq41_e1359_d_n0, eq41_e1359_d_n1, eq41_e1359_d_n2, eq41_e1359_d_n3, eq41_e1359_d_n4, eq41_e1359_d_n5, eq41_e1359_d_n6, eq41_e1359_d_n7, eq41_e1359_d_n8, eq41_e1359_d_n9, eq41_e1359_d_n10, eq41_e1359_d_n11, eq41_e1359_d_n12, eq41_e1359_d_b0, eq41_e1359_d_b1, eq41_e1359_d_b2, eq41_e1359_d_b3, eq41_e1359_d_b4, eq41_e1359_d_b5, eq41_e1359_d_b6, eq41_e1359_d_b7, eq41_e1359_d_b8, eq41_e1360_q,)
    } else {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    }
};
        let eq41_reactive_node_derivatives: [f64; 13] = [eq41_e1362_d_n0, eq41_e1362_d_n1, eq41_e1362_d_n2, eq41_e1362_d_n3, eq41_e1362_d_n4, eq41_e1362_d_n5, eq41_e1362_d_n6, eq41_e1362_d_n7, eq41_e1362_d_n8, eq41_e1362_d_n9, eq41_e1362_d_n10, eq41_e1362_d_n11, eq41_e1362_d_n12];
        let eq41_reactive_branch_derivatives: [f64; 9] = [eq41_e1362_d_b0, eq41_e1362_d_b1, eq41_e1362_d_b2, eq41_e1362_d_b3, eq41_e1362_d_b4, eq41_e1362_d_b5, eq41_e1362_d_b6, eq41_e1362_d_b7, eq41_e1362_d_b8];
        stamper.stamp_current_reactive_dense(
            Some(nodes[9]),
            Some(nodes[3]),
            nodes,
            &eq41_reactive_node_derivatives,
            branches,
            &eq41_reactive_branch_derivatives,
            multiplicity,
        );
        let eq42_e1364_q: f64 = s.v[449];
        stamper.stamp_current_reactive_dense(
            Some(nodes[7]),
            Some(nodes[3]),
            nodes,
            &s.dn[449],
            branches,
            &s.db[449],
            multiplicity,
        );
        let eq43_e1366_q: f64 = s.v[448];
        stamper.stamp_current_reactive_dense(
            Some(nodes[8]),
            Some(nodes[3]),
            nodes,
            &s.dn[448],
            branches,
            &s.db[448],
            multiplicity,
        );
        let (eq58_e1476, eq58_e1476_d_n0, eq58_e1476_d_n1, eq58_e1476_d_n2, eq58_e1476_d_n3, eq58_e1476_d_n4, eq58_e1476_d_n5, eq58_e1476_d_n6, eq58_e1476_d_n7, eq58_e1476_d_n8, eq58_e1476_d_n9, eq58_e1476_d_n10, eq58_e1476_d_n11, eq58_e1476_d_n12, eq58_e1476_d_b0, eq58_e1476_d_b1, eq58_e1476_d_b2, eq58_e1476_d_b3, eq58_e1476_d_b4, eq58_e1476_d_b5, eq58_e1476_d_b6, eq58_e1476_d_b7, eq58_e1476_d_b8, eq58_e1476_q,) = {
    if s.b[1869] {
        let eq58_e1473: f64 = (s.v[770] * s.v[528]);
        let eq58_e1474_q: f64 = eq58_e1473;
        (eq58_e1473, (s.dn[770][0] * s.v[528]), (s.dn[770][1] * s.v[528]), (s.dn[770][2] * s.v[528]), (s.dn[770][3] * s.v[528]), (s.dn[770][4] * s.v[528]), (s.dn[770][5] * s.v[528]), (s.dn[770][6] * s.v[528]), (s.dn[770][7] * s.v[528]), (s.dn[770][8] * s.v[528]), (s.dn[770][9] * s.v[528]), (s.dn[770][10] * s.v[528]), (s.dn[770][11] * s.v[528]), (s.dn[770][12] * s.v[528]), (s.db[770][0] * s.v[528]), (s.db[770][1] * s.v[528]), (s.db[770][2] * s.v[528]), (s.db[770][3] * s.v[528]), (s.db[770][4] * s.v[528]), (s.db[770][5] * s.v[528]), (s.db[770][6] * s.v[528]), (s.db[770][7] * s.v[528]), (s.db[770][8] * s.v[528]), eq58_e1474_q,)
    } else {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    }
};
        let eq58_reactive_node_derivatives: [f64; 13] = [eq58_e1476_d_n0, eq58_e1476_d_n1, eq58_e1476_d_n2, eq58_e1476_d_n3, eq58_e1476_d_n4, eq58_e1476_d_n5, eq58_e1476_d_n6, eq58_e1476_d_n7, eq58_e1476_d_n8, eq58_e1476_d_n9, eq58_e1476_d_n10, eq58_e1476_d_n11, eq58_e1476_d_n12];
        let eq58_reactive_branch_derivatives: [f64; 9] = [eq58_e1476_d_b0, eq58_e1476_d_b1, eq58_e1476_d_b2, eq58_e1476_d_b3, eq58_e1476_d_b4, eq58_e1476_d_b5, eq58_e1476_d_b6, eq58_e1476_d_b7, eq58_e1476_d_b8];
        stamper.stamp_current_reactive_dense(
            Some(nodes[6]),
            None,
            nodes,
            &eq58_reactive_node_derivatives,
            branches,
            &eq58_reactive_branch_derivatives,
            multiplicity,
        );
    }
}
