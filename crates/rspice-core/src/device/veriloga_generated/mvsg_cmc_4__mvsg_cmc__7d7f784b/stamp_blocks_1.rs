#![allow(dead_code, unused_assignments, unused_imports, unused_parens, unused_variables)]

use super::{A, GeneratedDerivative, GeneratedEvalContext, GeneratedReactiveStamper, GeneratedStamper, Scratch, THERMAL_VOLTAGE_PER_K};
use super::super::state::Instance;

impl Instance {

    pub(super) fn stamp_transient_block_16(
        &mut self,
        ctx: &GeneratedEvalContext<'_>,
        stamper: &mut GeneratedStamper<'_>,
        s: &mut Scratch,
    ) {
        let _ = stamper;
        let p = self.params;
        let nodes = self.nodes;
        let branches = self.branches;
        if (s.v[1202] != 0.0) {
            s.store_scalar(1303, 0.0);
        }

        if (s.v[1202] != 0.0) {
            s.store_scalar(1304, 0.0);
        }

        if (s.v[1202] != 0.0) {
            s.store_scalar(1305, 0.0);
        }

        if (s.v[1202] != 0.0) {
            s.store_scalar(1306, 0.0);
        }

        if (s.v[1202] != 0.0) {
            s.store_scalar(1307, 0.0);
        }

        if (s.v[1202] != 0.0) {
            s.store_scalar(1308, 0.0);
        }

        if (s.v[1202] != 0.0) {
            s.store_scalar(1309, 0.0);
        }

        if (s.v[1202] != 0.0) {
            s.store_scalar(1310, 0.0);
        }

        if (s.v[1202] != 0.0) {
            s.store_scalar(1311, 0.0);
        }

        if (s.v[1202] != 0.0) {
            s.store_scalar(1312, 0.0);
        }

        if (s.v[1202] != 0.0) {
            s.store_scalar(1313, 0.0);
        }

        if (s.v[1202] != 0.0) {
            s.store_scalar(1314, 0.0);
        }

        if (s.v[1202] != 0.0) {
            s.store_ad(1311, &{
                if (!(p.p52 == 0.0)) {
                    A::mul(s.ad_value(1213), A::tanh(A::scale(s.ad_value(1213), (0.001 / p.p53))))
                } else {
                    {
                        if (p.p52 == 0.0) {
                            A::sqrt(A::offset(A::square(s.ad_value(1213)), p.p53))
                        } else {
                            A::constant(0.0)
                        }
                    }
                }
            });
        }

        if (s.v[1202] != 0.0) {
            s.store_sub(1312, 1212, 1213);
        }

        if (s.v[1202] != 0.0) {
            s.store_mul(1246, 1232, 1220);
        }

        if (s.v[1202] != 0.0) {
            s.store_add_ad(1248, A::div(s.ad_value(1228), A::scale(s.ad_value(1220), 2.302585092994046)), A::mul(s.ad_value(1231), s.ad_value(1311)));
        }

        if (s.v[1202] != 0.0) {
            s.store_add_ad_rhs(1249, 1227, A::mul(s.ad_value(1238), A::sub(s.ad_value(1218), s.ad_value(1219))));
        }

        if (s.v[1202] != 0.0) {
            s.store_ad(1267, &A::pow(A::div(s.ad_value(1218), s.ad_value(1219)), s.ad_value(1240)));
        }

        s.v[1315] = if (s.v[1239] != 0.0) { 1.0 } else { 0.0 };

        if ((s.v[1202] != 0.0) && (s.v[1315] != 0.0)) {
            s.store_div_ad_rhs(1250, 1311, A::pow(A::offset(A::pow(A::div(s.ad_value(1311), s.ad_value(1239)), s.ad_value(1235)), 1.0), A::div_from_scalar(1.0, s.ad_value(1235))));
        }

        if ((s.v[1202] != 0.0) && (!(s.v[1315] != 0.0))) {
            s.store_scalar(1250, 0.0);
        }

        if (s.v[1202] != 0.0) {
            s.store_mul_ad_lhs(1247, A::sub(s.ad_value(1229), A::mul(s.ad_value(1250), s.ad_value(1230))), 1311);
        }

        if (s.v[1202] != 0.0) {
            s.store_sub(1210, 1249, 1247);
        }

        if (s.v[1202] != 0.0) {
            s.store_mul_ad_lhs(1252, A::scale(s.ad_value(1248), 2.0), 1220);
        }

        if (s.v[1202] != 0.0) {
            s.store_mul(1253, 1223, 1252);
        }

        if (s.v[1202] != 0.0) {
            s.store_sub_ad_rhs(1310, 1210, A::scale(s.ad_value(1246), (p.p51 * 0.5)));
        }

        if (s.v[1202] != 0.0) {
            let assign20060_ad_e19059: A = {
                if (!(p.p52 == 0.0)) {
                    A::scale(A::add(A::add(s.ad_value(1212), s.ad_value(1312)), A::mul(A::sub(s.ad_value(1212), s.ad_value(1312)), A::tanh(A::scale(A::sub(s.ad_value(1212), s.ad_value(1312)), (0.001 / p.p53))))), 0.5)
                } else {
                    {
                        if (p.p52 == 0.0) {
                            A::scale(A::add(A::add(s.ad_value(1212), s.ad_value(1312)), A::sqrt(A::offset(A::mul(A::sub(s.ad_value(1212), s.ad_value(1312)), A::sub(s.ad_value(1212), s.ad_value(1312))), p.p53))), 0.5)
                        } else {
                            A::constant(0.0)
                        }
                    }
                }
            };
            s.store_div_ad_lhs(1309, A::sub(assign20060_ad_e19059, s.ad_value(1310)), 1246);
        }

        s.v[1316] = if (s.v[1309] > 50.0) { 1.0 } else { 0.0 };

        if ((s.v[1202] != 0.0) && (s.v[1316] != 0.0)) {
            s.store_scalar(1268, 0.0);
        }

        s.v[1317] = if (s.v[1309] < (-50.0)) { 1.0 } else { 0.0 };

        if (((s.v[1202] != 0.0) && (!(s.v[1316] != 0.0))) && (s.v[1317] != 0.0)) {
            s.store_scalar(1268, 1.0);
        }

        if (((s.v[1202] != 0.0) && (!(s.v[1316] != 0.0))) && (!(s.v[1317] != 0.0))) {
            s.store_div_from_scalar_ad(1268, 1.0, A::offset(A::exp(s.ad_value(1309)), 1.0));
        }

        if (s.v[1202] != 0.0) {
            let assign20120_ad_e19147: A = {
                if (!(p.p52 == 0.0)) {
                    A::scale(A::add(A::add(s.ad_value(1212), s.ad_value(1312)), A::mul(A::sub(s.ad_value(1212), s.ad_value(1312)), A::tanh(A::scale(A::sub(s.ad_value(1212), s.ad_value(1312)), (0.001 / p.p53))))), 0.5)
                } else {
                    {
                        if (p.p52 == 0.0) {
                            A::scale(A::add(A::add(s.ad_value(1212), s.ad_value(1312)), A::sqrt(A::offset(A::mul(A::sub(s.ad_value(1212), s.ad_value(1312)), A::sub(s.ad_value(1212), s.ad_value(1312))), p.p53))), 0.5)
                        } else {
                            A::constant(0.0)
                        }
                    }
                }
            };
            s.store_div_ad_lhs(1269, A::sub(assign20120_ad_e19147, A::sub(s.ad_value(1210), A::mul(A::scale(s.ad_value(1246), (p.p51 * 0.1)), s.ad_value(1268)))), 1252);
        }

        s.v[1318] = if (s.v[1269] > 50.0) { 1.0 } else { 0.0 };

        if ((s.v[1202] != 0.0) && (s.v[1318] != 0.0)) {
            s.store_mul(1270, 1253, 1269);
        }

        s.v[1319] = if (s.v[1269] < (-50.0)) { 1.0 } else { 0.0 };

        if (((s.v[1202] != 0.0) && (!(s.v[1318] != 0.0))) && (s.v[1319] != 0.0)) {
            s.store_mul_ad_rhs(1270, 1253, A::exp(s.ad_value(1269)));
        }

        if (((s.v[1202] != 0.0) && (!(s.v[1318] != 0.0))) && (!(s.v[1319] != 0.0))) {
            s.store_mul_ad_rhs(1270, 1253, A::ln(A::offset(A::exp(s.ad_value(1269)), 1.0)));
        }

        if (s.v[1202] != 0.0) {
            s.store_div_ad_rhs(1256, 1234, A::mul(s.ad_value(1267), A::offset(A::div(A::mul(s.ad_value(1236), s.ad_value(1270)), s.ad_value(1223)), 1.0)));
        }

        if (s.v[1202] != 0.0) {
            s.store_div_ad(1257, A::mul(A::mul(s.ad_value(1233), A::div(A::offset(A::mul(s.ad_value(1241), s.ad_value(1219)), 1.0), A::offset(A::mul(s.ad_value(1241), s.ad_value(1218)), 1.0))), A::offset(A::div(A::mul(s.ad_value(1242), s.ad_value(1311)), s.ad_value(1222)), 1.0)), A::offset(A::div(A::mul(s.ad_value(1237), s.ad_value(1270)), s.ad_value(1223)), 1.0));
        }

        if (s.v[1202] != 0.0) {
            s.store_add_ad(1258, A::div(A::mul(A::mul(A::scale(s.ad_value(1268), 2.0), s.ad_value(1220)), s.ad_value(1256)), s.ad_value(1222)), A::mul(A::sub_from_scalar(1.0, s.ad_value(1268)), s.ad_value(1257)));
        }

        if (s.v[1202] != 0.0) {
            s.store_div_ad_lhs(1274, A::mul(s.ad_value(1257), s.ad_value(1222)), 1256);
        }

        if (s.v[1202] != 0.0) {
            s.store_sub_ad_lhs(1275, A::mul(s.ad_value(1274), A::sqrt(A::offset(A::div(A::div(A::scale(s.ad_value(1270), 2.0), s.ad_value(1223)), s.ad_value(1274)), 1.0))), 1274);
        }

        if (s.v[1202] != 0.0) {
            s.store_add_ad(1276, A::mul(s.ad_value(1274), A::sub_from_scalar(1.0, s.ad_value(1268))), A::mul(s.ad_value(1252), s.ad_value(1268)));
        }

        if (s.v[1202] != 0.0) {
            s.store_add_ad(1211, A::mul(s.ad_value(1275), A::sub_from_scalar(1.0, s.ad_value(1268))), A::mul(s.ad_value(1252), s.ad_value(1268)));
        }

        if (s.v[1202] != 0.0) {
            let assign20250_ad_e19376: A = {
                if (!(p.p52 == 0.0)) {
                    A::scale(A::add(A::div(s.ad_value(1213), s.ad_value(1211)), A::mul(A::neg(A::div(s.ad_value(1213), s.ad_value(1211))), A::tanh(A::scale(A::neg(A::div(s.ad_value(1213), s.ad_value(1211))), (0.001 / p.p53))))), 0.5)
                } else {
                    {
                        if (p.p52 == 0.0) {
                            A::scale(A::add(A::div(s.ad_value(1213), s.ad_value(1211)), A::sqrt(A::offset(A::mul(A::neg(A::div(s.ad_value(1213), s.ad_value(1211))), A::neg(A::div(s.ad_value(1213), s.ad_value(1211)))), p.p53))), 0.5)
                        } else {
                            A::constant(0.0)
                        }
                    }
                }
            };
            s.store_div_from_scalar_ad(1277, 1.0, A::pow(A::offset(A::pow(assign20250_ad_e19376, s.ad_value(1235)), 1.0), A::div_from_scalar(1.0, s.ad_value(1235))));
        }

        if (s.v[1202] != 0.0) {
            s.store_mul(1278, 1213, 1277);
        }

        if (s.v[1202] != 0.0) {
            let assign20270_ad_e19457: A = {
                if (!(p.p52 == 0.0)) {
                    A::scale(A::add(A::div(A::neg(s.ad_value(1213)), s.ad_value(1211)), A::mul(A::neg(A::div(A::neg(s.ad_value(1213)), s.ad_value(1211))), A::tanh(A::scale(A::neg(A::div(A::neg(s.ad_value(1213)), s.ad_value(1211))), (0.001 / p.p53))))), 0.5)
                } else {
                    {
                        if (p.p52 == 0.0) {
                            A::scale(A::add(A::div(A::neg(s.ad_value(1213)), s.ad_value(1211)), A::sqrt(A::offset(A::mul(A::neg(A::div(A::neg(s.ad_value(1213)), s.ad_value(1211))), A::neg(A::div(A::neg(s.ad_value(1213)), s.ad_value(1211)))), p.p53))), 0.5)
                        } else {
                            A::constant(0.0)
                        }
                    }
                }
            };
            s.store_div_from_scalar_ad(1279, 1.0, A::pow(A::offset(A::pow(assign20270_ad_e19457, s.ad_value(1235)), 1.0), A::div_from_scalar(1.0, s.ad_value(1235))));
        }

        if (s.v[1202] != 0.0) {
            s.store_mul_ad_lhs(1280, A::neg(s.ad_value(1213)), 1279);
        }

        if (s.v[1202] != 0.0) {
            s.store_div_ad_lhs(1309, A::sub(s.ad_value(1212), s.ad_value(1310)), 1246);
        }

        s.v[1320] = if (s.v[1309] > 50.0) { 1.0 } else { 0.0 };

        if ((s.v[1202] != 0.0) && (s.v[1320] != 0.0)) {
            s.store_scalar(1251, 0.0);
        }

        s.v[1321] = if (s.v[1309] < (-50.0)) { 1.0 } else { 0.0 };

        if (((s.v[1202] != 0.0) && (!(s.v[1320] != 0.0))) && (s.v[1321] != 0.0)) {
            s.store_scalar(1251, 1.0);
        }

        if (((s.v[1202] != 0.0) && (!(s.v[1320] != 0.0))) && (!(s.v[1321] != 0.0))) {
            s.store_div_from_scalar_ad(1251, 1.0, A::offset(A::exp(s.ad_value(1309)), 1.0));
        }

        if (s.v[1202] != 0.0) {
            s.store_div_ad_lhs(1254, A::sub(A::sub(s.ad_value(1312), s.ad_value(1280)), A::sub(s.ad_value(1210), A::mul(A::scale(s.ad_value(1246), (p.p51 * 0.1)), s.ad_value(1251)))), 1252);
        }

        s.v[1322] = if (s.v[1254] > 50.0) { 1.0 } else { 0.0 };

        if ((s.v[1202] != 0.0) && (s.v[1322] != 0.0)) {
            s.store_mul(1255, 1253, 1254);
        }

        s.v[1323] = if (s.v[1254] < (-50.0)) { 1.0 } else { 0.0 };

        if (((s.v[1202] != 0.0) && (!(s.v[1322] != 0.0))) && (s.v[1323] != 0.0)) {
            s.store_mul_ad_rhs(1255, 1253, A::exp(s.ad_value(1254)));
        }

        if (((s.v[1202] != 0.0) && (!(s.v[1322] != 0.0))) && (!(s.v[1323] != 0.0))) {
            s.store_mul_ad_rhs(1255, 1253, A::ln(A::offset(A::exp(s.ad_value(1254)), 1.0)));
        }

        if (s.v[1202] != 0.0) {
            s.store_div_ad_lhs(1309, A::sub(s.ad_value(1312), s.ad_value(1310)), 1246);
        }

        s.v[1324] = if (s.v[1309] > 50.0) { 1.0 } else { 0.0 };

        if ((s.v[1202] != 0.0) && (s.v[1324] != 0.0)) {
            s.store_scalar(1281, 0.0);
        }

        s.v[1325] = if (s.v[1309] < (-50.0)) { 1.0 } else { 0.0 };

        if (((s.v[1202] != 0.0) && (!(s.v[1324] != 0.0))) && (s.v[1325] != 0.0)) {
            s.store_scalar(1281, 1.0);
        }

        if (((s.v[1202] != 0.0) && (!(s.v[1324] != 0.0))) && (!(s.v[1325] != 0.0))) {
            s.store_div_from_scalar_ad(1281, 1.0, A::offset(A::exp(s.ad_value(1309)), 1.0));
        }

        if (s.v[1202] != 0.0) {
            s.store_div_ad_lhs(1282, A::sub(A::sub(s.ad_value(1212), s.ad_value(1278)), A::sub(s.ad_value(1210), A::mul(A::scale(s.ad_value(1246), (p.p51 * 0.1)), s.ad_value(1281)))), 1252);
        }

        s.v[1326] = if (s.v[1282] > 50.0) { 1.0 } else { 0.0 };

        if ((s.v[1202] != 0.0) && (s.v[1326] != 0.0)) {
            s.store_mul(1283, 1253, 1282);
        }

        s.v[1327] = if (s.v[1282] < (-50.0)) { 1.0 } else { 0.0 };

        if (((s.v[1202] != 0.0) && (!(s.v[1326] != 0.0))) && (s.v[1327] != 0.0)) {
            s.store_mul_ad_rhs(1283, 1253, A::exp(s.ad_value(1282)));
        }

        if (((s.v[1202] != 0.0) && (!(s.v[1326] != 0.0))) && (!(s.v[1327] != 0.0))) {
            s.store_mul_ad_rhs(1283, 1253, A::ln(A::offset(A::exp(s.ad_value(1282)), 1.0)));
        }

        if (s.v[1202] != 0.0) {
            s.store_div_ad_lhs(1284, A::sub(s.ad_value(1255), s.ad_value(1283)), 1223);
        }

        if (s.v[1202] != 0.0) {
            s.store_div(1310, 1284, 1276);
        }

        if (s.v[1202] != 0.0) {
            let assign20550_ad_e19734: A = A::pow(A::offset(A::pow({
                if (!(p.p52 == 0.0)) {
                    A::mul(s.ad_value(1310), A::tanh(A::scale(s.ad_value(1310), (0.001 / p.p53))))
                } else {
                    {
                        if (p.p52 == 0.0) {
                            A::sqrt(A::offset(A::square(s.ad_value(1310)), p.p53))
                        } else {
                            A::constant(0.0)
                        }
                    }
                }
            }, s.ad_value(1235)), 1.0), A::div_from_scalar(1.0, s.ad_value(1235)));
            s.store_div_ad_rhs(1285, 1310, assign20550_ad_e19734);
        }

        if (s.v[1202] != 0.0) {
            s.store_mul(1286, 1258, 1285);
        }

        if (s.v[1202] != 0.0) {
            s.store_mul_ad_lhs(1204, A::mul(A::mul(A::scale(A::mul(A::mul(s.ad_value(1244), s.ad_value(1221)), s.ad_value(1243)), 0.5), A::add(s.ad_value(1255), s.ad_value(1283))), s.ad_value(1286)), 1245);
        }

        if (s.v[1202] != 0.0) {
            s.store_div_ad_rhs(1259, 1228, A::scale(s.ad_value(1220), 2.302585092994046));
        }

        if (s.v[1202] != 0.0) {
            s.store_mul_ad_lhs(1261, A::scale(s.ad_value(1259), 2.0), 1220);
        }

        if (s.v[1202] != 0.0) {
            s.store_mul(1262, 1223, 1261);
        }

        if (s.v[1202] != 0.0) {
            s.store_sub_ad_rhs(1314, 1249, A::scale(s.ad_value(1246), (p.p51 * 0.5)));
        }

        if (s.v[1202] != 0.0) {
            let assign20620_ad_e19838: A = {
                if (!(p.p52 == 0.0)) {
                    A::scale(A::add(A::add(s.ad_value(1212), s.ad_value(1312)), A::mul(A::sub(s.ad_value(1212), s.ad_value(1312)), A::tanh(A::scale(A::sub(s.ad_value(1212), s.ad_value(1312)), (0.001 / p.p53))))), 0.5)
                } else {
                    {
                        if (p.p52 == 0.0) {
                            A::scale(A::add(A::add(s.ad_value(1212), s.ad_value(1312)), A::sqrt(A::offset(A::mul(A::sub(s.ad_value(1212), s.ad_value(1312)), A::sub(s.ad_value(1212), s.ad_value(1312))), p.p53))), 0.5)
                        } else {
                            A::constant(0.0)
                        }
                    }
                }
            };
            s.store_div_ad_lhs(1313, A::sub(assign20620_ad_e19838, s.ad_value(1314)), 1246);
        }

        s.v[1328] = if (s.v[1313] > 50.0) { 1.0 } else { 0.0 };

        if ((s.v[1202] != 0.0) && (s.v[1328] != 0.0)) {
            s.store_scalar(1271, 0.0);
        }

        s.v[1329] = if (s.v[1313] < (-50.0)) { 1.0 } else { 0.0 };

        if (((s.v[1202] != 0.0) && (!(s.v[1328] != 0.0))) && (s.v[1329] != 0.0)) {
            s.store_scalar(1271, 1.0);
        }

        if (((s.v[1202] != 0.0) && (!(s.v[1328] != 0.0))) && (!(s.v[1329] != 0.0))) {
            s.store_div_from_scalar_ad(1271, 1.0, A::offset(A::exp(s.ad_value(1313)), 1.0));
        }

        if (s.v[1202] != 0.0) {
            let assign20680_ad_e19926: A = {
                if (!(p.p52 == 0.0)) {
                    A::scale(A::add(A::add(s.ad_value(1212), s.ad_value(1312)), A::mul(A::sub(s.ad_value(1212), s.ad_value(1312)), A::tanh(A::scale(A::sub(s.ad_value(1212), s.ad_value(1312)), (0.001 / p.p53))))), 0.5)
                } else {
                    {
                        if (p.p52 == 0.0) {
                            A::scale(A::add(A::add(s.ad_value(1212), s.ad_value(1312)), A::sqrt(A::offset(A::mul(A::sub(s.ad_value(1212), s.ad_value(1312)), A::sub(s.ad_value(1212), s.ad_value(1312))), p.p53))), 0.5)
                        } else {
                            A::constant(0.0)
                        }
                    }
                }
            };
            s.store_div_ad_lhs(1272, A::sub(assign20680_ad_e19926, A::sub(s.ad_value(1249), A::mul(A::scale(s.ad_value(1246), (p.p51 * 0.1)), s.ad_value(1271)))), 1261);
        }

        s.v[1330] = if (s.v[1272] > 50.0) { 1.0 } else { 0.0 };

        if ((s.v[1202] != 0.0) && (s.v[1330] != 0.0)) {
            s.store_mul(1273, 1262, 1272);
        }

        s.v[1331] = if (s.v[1272] < (-50.0)) { 1.0 } else { 0.0 };

        if (((s.v[1202] != 0.0) && (!(s.v[1330] != 0.0))) && (s.v[1331] != 0.0)) {
            s.store_mul_ad_rhs(1273, 1262, A::exp(s.ad_value(1272)));
        }

        if (((s.v[1202] != 0.0) && (!(s.v[1330] != 0.0))) && (!(s.v[1331] != 0.0))) {
            s.store_mul_ad_rhs(1273, 1262, A::ln(A::offset(A::exp(s.ad_value(1272)), 1.0)));
        }

        if (s.v[1202] != 0.0) {
            s.store_div(1265, 1234, 1267);
        }

        if (s.v[1202] != 0.0) {
            s.store_mul_ad_rhs(1266, 1233, A::div(A::offset(A::mul(s.ad_value(1241), s.ad_value(1219)), 1.0), A::offset(A::mul(s.ad_value(1241), s.ad_value(1218)), 1.0)));
        }

        if (s.v[1202] != 0.0) {
            s.store_div_ad_lhs(1287, A::mul(s.ad_value(1266), s.ad_value(1222)), 1265);
        }

        if (s.v[1202] != 0.0) {
            s.store_sub_ad_lhs(1288, A::mul(s.ad_value(1287), A::sqrt(A::offset(A::div(A::div(A::scale(s.ad_value(1273), 2.0), s.ad_value(1223)), s.ad_value(1287)), 1.0))), 1287);
        }

        if (s.v[1202] != 0.0) {
            s.store_add_ad(1289, A::mul(s.ad_value(1288), A::sub_from_scalar(1.0, s.ad_value(1271))), A::mul(s.ad_value(1261), s.ad_value(1271)));
        }

        if (s.v[1202] != 0.0) {
            let assign20790_ad_e20101: A = {
                if (!(p.p52 == 0.0)) {
                    A::scale(A::add(A::div(s.ad_value(1213), s.ad_value(1289)), A::mul(A::neg(A::div(s.ad_value(1213), s.ad_value(1289))), A::tanh(A::scale(A::neg(A::div(s.ad_value(1213), s.ad_value(1289))), (0.001 / p.p53))))), 0.5)
                } else {
                    {
                        if (p.p52 == 0.0) {
                            A::scale(A::add(A::div(s.ad_value(1213), s.ad_value(1289)), A::sqrt(A::offset(A::mul(A::neg(A::div(s.ad_value(1213), s.ad_value(1289))), A::neg(A::div(s.ad_value(1213), s.ad_value(1289)))), p.p53))), 0.5)
                        } else {
                            A::constant(0.0)
                        }
                    }
                }
            };
            s.store_div_from_scalar_ad(1290, 1.0, A::pow(A::offset(A::pow(assign20790_ad_e20101, s.ad_value(1235)), 1.0), A::div_from_scalar(1.0, s.ad_value(1235))));
        }

        if (s.v[1202] != 0.0) {
            s.store_mul(1291, 1213, 1290);
        }

        if (s.v[1202] != 0.0) {
            let assign20810_ad_e20182: A = {
                if (!(p.p52 == 0.0)) {
                    A::scale(A::add(A::div(A::neg(s.ad_value(1213)), s.ad_value(1289)), A::mul(A::neg(A::div(A::neg(s.ad_value(1213)), s.ad_value(1289))), A::tanh(A::scale(A::neg(A::div(A::neg(s.ad_value(1213)), s.ad_value(1289))), (0.001 / p.p53))))), 0.5)
                } else {
                    {
                        if (p.p52 == 0.0) {
                            A::scale(A::add(A::div(A::neg(s.ad_value(1213)), s.ad_value(1289)), A::sqrt(A::offset(A::mul(A::neg(A::div(A::neg(s.ad_value(1213)), s.ad_value(1289))), A::neg(A::div(A::neg(s.ad_value(1213)), s.ad_value(1289)))), p.p53))), 0.5)
                        } else {
                            A::constant(0.0)
                        }
                    }
                }
            };
            s.store_div_from_scalar_ad(1292, 1.0, A::pow(A::offset(A::pow(assign20810_ad_e20182, s.ad_value(1235)), 1.0), A::div_from_scalar(1.0, s.ad_value(1235))));
        }

        if (s.v[1202] != 0.0) {
            s.store_mul_ad_lhs(1293, A::neg(s.ad_value(1213)), 1292);
        }

        if (s.v[1202] != 0.0) {
            s.store_div_ad_lhs(1313, A::sub(s.ad_value(1212), s.ad_value(1314)), 1246);
        }

    }

    pub(super) fn stamp_transient_block_17(
        &mut self,
        ctx: &GeneratedEvalContext<'_>,
        stamper: &mut GeneratedStamper<'_>,
        s: &mut Scratch,
    ) {
        let _ = stamper;
        let p = self.params;
        let nodes = self.nodes;
        let branches = self.branches;
        s.v[1332] = if (s.v[1313] > 50.0) { 1.0 } else { 0.0 };

        if ((s.v[1202] != 0.0) && (s.v[1332] != 0.0)) {
            s.store_scalar(1260, 0.0);
        }

        s.v[1333] = if (s.v[1313] < (-50.0)) { 1.0 } else { 0.0 };

        if (((s.v[1202] != 0.0) && (!(s.v[1332] != 0.0))) && (s.v[1333] != 0.0)) {
            s.store_scalar(1260, 1.0);
        }

        if (((s.v[1202] != 0.0) && (!(s.v[1332] != 0.0))) && (!(s.v[1333] != 0.0))) {
            s.store_div_from_scalar_ad(1260, 1.0, A::offset(A::exp(s.ad_value(1313)), 1.0));
        }

        if (s.v[1202] != 0.0) {
            s.store_div_ad_lhs(1263, A::sub(A::sub(s.ad_value(1312), s.ad_value(1293)), A::sub(s.ad_value(1249), A::mul(A::scale(s.ad_value(1246), (p.p51 * 0.1)), s.ad_value(1260)))), 1261);
        }

        s.v[1334] = if (s.v[1263] > 50.0) { 1.0 } else { 0.0 };

        if ((s.v[1202] != 0.0) && (s.v[1334] != 0.0)) {
            s.store_mul(1264, 1262, 1263);
        }

        s.v[1335] = if (s.v[1263] < (-50.0)) { 1.0 } else { 0.0 };

        if (((s.v[1202] != 0.0) && (!(s.v[1334] != 0.0))) && (s.v[1335] != 0.0)) {
            s.store_mul_ad_rhs(1264, 1262, A::exp(s.ad_value(1263)));
        }

        if (((s.v[1202] != 0.0) && (!(s.v[1334] != 0.0))) && (!(s.v[1335] != 0.0))) {
            s.store_mul_ad_rhs(1264, 1262, A::ln(A::offset(A::exp(s.ad_value(1263)), 1.0)));
        }

        if (s.v[1202] != 0.0) {
            s.store_div_ad_lhs(1313, A::sub(s.ad_value(1312), s.ad_value(1314)), 1246);
        }

        s.v[1336] = if (s.v[1313] > 50.0) { 1.0 } else { 0.0 };

        if ((s.v[1202] != 0.0) && (s.v[1336] != 0.0)) {
            s.store_scalar(1294, 0.0);
        }

        s.v[1337] = if (s.v[1313] < (-50.0)) { 1.0 } else { 0.0 };

        if (((s.v[1202] != 0.0) && (!(s.v[1336] != 0.0))) && (s.v[1337] != 0.0)) {
            s.store_scalar(1294, 1.0);
        }

        if (((s.v[1202] != 0.0) && (!(s.v[1336] != 0.0))) && (!(s.v[1337] != 0.0))) {
            s.store_div_from_scalar_ad(1294, 1.0, A::offset(A::exp(s.ad_value(1313)), 1.0));
        }

        if (s.v[1202] != 0.0) {
            s.store_div_ad_lhs(1295, A::sub(A::sub(s.ad_value(1212), s.ad_value(1291)), A::sub(s.ad_value(1249), A::mul(A::scale(s.ad_value(1246), (p.p51 * 0.1)), s.ad_value(1294)))), 1261);
        }

        s.v[1338] = if (s.v[1295] > 50.0) { 1.0 } else { 0.0 };

        if ((s.v[1202] != 0.0) && (s.v[1338] != 0.0)) {
            s.store_mul(1296, 1262, 1295);
        }

        s.v[1339] = if (s.v[1295] < (-50.0)) { 1.0 } else { 0.0 };

        if (((s.v[1202] != 0.0) && (!(s.v[1338] != 0.0))) && (s.v[1339] != 0.0)) {
            s.store_mul_ad_rhs(1296, 1262, A::exp(s.ad_value(1295)));
        }

        if (((s.v[1202] != 0.0) && (!(s.v[1338] != 0.0))) && (!(s.v[1339] != 0.0))) {
            s.store_mul_ad_rhs(1296, 1262, A::ln(A::offset(A::exp(s.ad_value(1295)), 1.0)));
        }

        if (s.v[1202] != 0.0) {
            s.store_offset_ad(1297, A::square(s.ad_value(1264)), 1e-38);
        }

        if (s.v[1202] != 0.0) {
            s.store_offset_ad(1298, A::mul(s.ad_value(1297), s.ad_value(1264)), 1e-57);
        }

        if (s.v[1202] != 0.0) {
            s.store_offset_ad(1299, A::square(s.ad_value(1296)), 1e-38);
        }

        if (s.v[1202] != 0.0) {
            s.store_offset_ad(1300, A::mul(s.ad_value(1299), s.ad_value(1296)), 1e-57);
        }

        if (s.v[1202] != 0.0) {
            s.store_offset_ad(1301, A::mul(s.ad_value(1264), s.ad_value(1296)), 1e-38);
        }

        if (s.v[1202] != 0.0) {
            s.store_div_ad(1302, A::scale(A::add(A::add(s.ad_value(1297), s.ad_value(1299)), s.ad_value(1301)), (2.0 / 3.0)), A::offset(A::add(s.ad_value(1264), s.ad_value(1296)), 2e-19));
        }

        if (s.v[1202] != 0.0) {
            s.store_div_ad(1303, A::scale(A::add(A::add(A::add(A::scale(s.ad_value(1298), 2.0), A::scale(s.ad_value(1300), 3.0)), A::mul(A::scale(s.ad_value(1297), 4.0), s.ad_value(1296))), A::mul(A::scale(s.ad_value(1299), 6.0), s.ad_value(1264))), 2.0), A::scale(A::add(A::add(s.ad_value(1297), s.ad_value(1299)), A::scale(s.ad_value(1301), 2.0)), 15.0));
        }

        if (s.v[1202] != 0.0) {
            s.store_sub(1304, 1302, 1303);
        }

        if (s.v[1202] != 0.0) {
            s.copy_ad(1305, 1303);
        }

        if (s.v[1202] != 0.0) {
            s.store_mul_ad_lhs(1205, A::mul(A::mul(A::mul(A::mul(s.ad_value(1221), s.ad_value(1243)), s.ad_value(1222)), s.ad_value(1244)), s.ad_value(1304)), 1245);
        }

        if (s.v[1202] != 0.0) {
            s.store_mul_ad_lhs(1206, A::mul(A::mul(A::mul(A::mul(s.ad_value(1221), s.ad_value(1243)), s.ad_value(1222)), s.ad_value(1244)), s.ad_value(1305)), 1245);
        }

        s.v[1340] = if (s.v[1214] == 1.0) { 1.0 } else { 0.0 };

        if ((s.v[1202] != 0.0) && (s.v[1340] != 0.0)) {
            s.store_div_ad_lhs(1306, A::sub(s.ad_value(1215), A::sub(s.ad_value(1249), A::scale(s.ad_value(1246), (p.p51 * 0.5)))), 1261);
        }

        s.v[1341] = if (s.v[1306] > 50.0) { 1.0 } else { 0.0 };

        if (((s.v[1202] != 0.0) && (s.v[1340] != 0.0)) && (s.v[1341] != 0.0)) {
            s.copy_ad(1309, 1306);
        }

        s.v[1342] = if (s.v[1306] < (-50.0)) { 1.0 } else { 0.0 };

        if ((((s.v[1202] != 0.0) && (s.v[1340] != 0.0)) && (!(s.v[1341] != 0.0))) && (s.v[1342] != 0.0)) {
            s.store_exp(1309, 1306);
        }

        if ((((s.v[1202] != 0.0) && (s.v[1340] != 0.0)) && (!(s.v[1341] != 0.0))) && (!(s.v[1342] != 0.0))) {
            s.store_ln_ad(1309, A::offset(A::exp(s.ad_value(1306)), 1.0));
        }

        if ((s.v[1202] != 0.0) && (s.v[1340] != 0.0)) {
            s.store_mul_ad_lhs(1207, A::mul(A::mul(A::mul(A::mul(A::mul(s.ad_value(1221), s.ad_value(1243)), s.ad_value(1244)), s.ad_value(1225)), s.ad_value(1261)), s.ad_value(1309)), 1245);
        }

        if ((s.v[1202] != 0.0) && (s.v[1340] != 0.0)) {
            s.store_div_ad_lhs(1307, A::sub(s.ad_value(1216), A::sub(s.ad_value(1249), A::scale(s.ad_value(1246), (p.p51 * 0.5)))), 1261);
        }

        s.v[1343] = if (s.v[1307] > 50.0) { 1.0 } else { 0.0 };

        if (((s.v[1202] != 0.0) && (s.v[1340] != 0.0)) && (s.v[1343] != 0.0)) {
            s.copy_ad(1309, 1307);
        }

        s.v[1344] = if (s.v[1307] < (-50.0)) { 1.0 } else { 0.0 };

        if ((((s.v[1202] != 0.0) && (s.v[1340] != 0.0)) && (!(s.v[1343] != 0.0))) && (s.v[1344] != 0.0)) {
            s.store_exp(1309, 1307);
        }

        if ((((s.v[1202] != 0.0) && (s.v[1340] != 0.0)) && (!(s.v[1343] != 0.0))) && (!(s.v[1344] != 0.0))) {
            s.store_ln_ad(1309, A::offset(A::exp(s.ad_value(1307)), 1.0));
        }

        if ((s.v[1202] != 0.0) && (s.v[1340] != 0.0)) {
            s.store_mul_ad_lhs(1208, A::mul(A::mul(A::mul(A::mul(A::mul(s.ad_value(1221), s.ad_value(1243)), s.ad_value(1244)), s.ad_value(1226)), s.ad_value(1261)), s.ad_value(1309)), 1245);
        }

        if ((s.v[1202] != 0.0) && (!(s.v[1340] != 0.0))) {
            s.store_scalar(1207, 0.0);
        }

        if ((s.v[1202] != 0.0) && (!(s.v[1340] != 0.0))) {
            s.store_scalar(1208, 0.0);
        }

        s.v[1345] = if (s.v[1217] == 1.0) { 1.0 } else { 0.0 };

        if ((s.v[1202] != 0.0) && (s.v[1345] != 0.0)) {
            s.store_div_ad_lhs(1308, A::sub(s.ad_value(1212), A::sub(s.ad_value(1249), A::scale(s.ad_value(1246), (p.p51 * 0.5)))), 1261);
        }

        s.v[1346] = if (s.v[1308] > 50.0) { 1.0 } else { 0.0 };

        if (((s.v[1202] != 0.0) && (s.v[1345] != 0.0)) && (s.v[1346] != 0.0)) {
            s.copy_ad(1309, 1308);
        }

        s.v[1347] = if (s.v[1308] < (-50.0)) { 1.0 } else { 0.0 };

        if ((((s.v[1202] != 0.0) && (s.v[1345] != 0.0)) && (!(s.v[1346] != 0.0))) && (s.v[1347] != 0.0)) {
            s.store_exp(1309, 1308);
        }

        if ((((s.v[1202] != 0.0) && (s.v[1345] != 0.0)) && (!(s.v[1346] != 0.0))) && (!(s.v[1347] != 0.0))) {
            s.store_ln_ad(1309, A::offset(A::exp(s.ad_value(1308)), 1.0));
        }

        if ((s.v[1202] != 0.0) && (s.v[1345] != 0.0)) {
            s.store_mul_ad_lhs(1209, A::mul(A::mul(A::mul(A::mul(A::mul(s.ad_value(1221), s.ad_value(1243)), s.ad_value(1244)), s.ad_value(1224)), s.ad_value(1261)), s.ad_value(1309)), 1245);
        }

        if ((s.v[1202] != 0.0) && (!(s.v[1345] != 0.0))) {
            s.store_scalar(1209, 0.0);
        }

        if (s.v[1202] != 0.0) {
            s.copy_ad(1203, 1204);
        }

        if (s.v[1202] != 0.0) {
            s.copy_ad(178, 1204);
        }

        if (s.v[1202] != 0.0) {
            s.copy_ad(179, 1205);
        }

        if (s.v[1202] != 0.0) {
            s.copy_ad(180, 1206);
        }

        if (s.v[1202] != 0.0) {
            s.copy_ad(181, 1207);
        }

        if (s.v[1202] != 0.0) {
            s.copy_ad(182, 1208);
        }

        if (s.v[1202] != 0.0) {
            s.copy_ad(183, 1209);
        }

        if (s.v[1202] != 0.0) {
            s.copy_ad(178, 1203);
        }

        s.v[1348] = if (p.p122 == 1.0) { 1.0 } else { 0.0 };

        s.v[184] = 0.0;

        s.v[185] = 0.0;

        s.v[186] = 0.0;

        s.v[187] = 0.0;

        s.v[188] = 0.0;

        s.v[189] = 0.0;

        s.v[1349] = if (p.p145 > p.p354) { 1.0 } else { 0.0 };

        if (s.v[1349] != 0.0) {
            s.store_scalar(1350, 0.0);
        }

        if (s.v[1349] != 0.0) {
            s.store_scalar(1351, 0.0);
        }

        if (s.v[1349] != 0.0) {
            s.store_scalar(1352, 0.0);
        }

        if (s.v[1349] != 0.0) {
            s.store_scalar(1353, 0.0);
        }

        if (s.v[1349] != 0.0) {
            s.store_scalar(1354, 0.0);
        }

        if (s.v[1349] != 0.0) {
            s.store_scalar(1355, 0.0);
        }

        if (s.v[1349] != 0.0) {
            s.store_scalar(1356, 0.0);
        }

        if (s.v[1349] != 0.0) {
            s.store_scalar(1357, 0.0);
        }

        if (s.v[1349] != 0.0) {
            s.store_scalar(1358, 0.0);
        }

        if (s.v[1349] != 0.0) {
            s.copy_ad(1359, 78);
        }

        if (s.v[1349] != 0.0) {
            s.copy_ad(1360, 79);
        }

        if (s.v[1349] != 0.0) {
            s.store_scalar(1361, p.p151);
        }

        if (s.v[1349] != 0.0) {
            s.copy_ad(1362, 80);
        }

        if (s.v[1349] != 0.0) {
            s.copy_ad(1363, 81);
        }

        if (s.v[1349] != 0.0) {
            s.store_scalar(1364, p.p149);
        }

        if (s.v[1349] != 0.0) {
            s.copy_ad(1365, 111);
        }

        if (s.v[1349] != 0.0) {
            s.store_scalar(1366, s.v[109]);
        }

        if (s.v[1349] != 0.0) {
            s.copy_ad(1367, 113);
        }

        if (s.v[1349] != 0.0) {
            s.store_scalar(1368, p.p0);
        }

        if (s.v[1349] != 0.0) {
            s.store_scalar(1369, p.p145);
        }

        if (s.v[1349] != 0.0) {
            s.copy_ad(1370, 29);
        }

        if (s.v[1349] != 0.0) {
            s.store_scalar(1371, p.p150);
        }

        if (s.v[1349] != 0.0) {
            s.copy_ad(1372, 30);
        }

        if (s.v[1349] != 0.0) {
            s.copy_ad(1373, 31);
        }

        if (s.v[1349] != 0.0) {
            s.store_scalar(1374, p.p146);
        }

        if (s.v[1349] != 0.0) {
            s.store_scalar(1375, p.p160);
        }

        if (s.v[1349] != 0.0) {
            s.store_scalar(1376, p.p159);
        }

        if (s.v[1349] != 0.0) {
            s.store_scalar(1377, 0.0);
        }

        if (s.v[1349] != 0.0) {
            s.store_scalar(1378, p.p161);
        }

        if (s.v[1349] != 0.0) {
            s.store_scalar(1379, p.p165);
        }

        if (s.v[1349] != 0.0) {
            s.store_scalar(1380, p.p156);
        }

        if (s.v[1349] != 0.0) {
            s.store_scalar(1381, p.p157);
        }

        if (s.v[1349] != 0.0) {
            s.store_scalar(1382, p.p158);
        }

        if (s.v[1349] != 0.0) {
            s.store_scalar(1383, p.p164);
        }

        if (s.v[1349] != 0.0) {
            s.store_scalar(1384, p.p163);
        }

        if (s.v[1349] != 0.0) {
            s.store_scalar(1385, p.p162);
        }

        if (s.v[1349] != 0.0) {
            s.store_scalar(1386, p.p39);
        }

        if (s.v[1349] != 0.0) {
            s.store_scalar(1387, p.p47);
        }

        if (s.v[1349] != 0.0) {
            s.store_scalar(1388, p.p45);
        }

        if (s.v[1349] != 0.0) {
            s.store_scalar(1389, p.p42);
        }

        if (s.v[1349] != 0.0) {
            s.store_scalar(1390, p.p2);
        }

        if (s.v[1349] != 0.0) {
            s.store_scalar(1391, p.p6);
        }

        if (s.v[1349] != 0.0) {
            s.store_scalar(1392, 1.0);
        }

        if (s.v[1349] != 0.0) {
            s.store_scalar(1393, 0.0);
        }

        if (s.v[1349] != 0.0) {
            s.store_scalar(1394, 0.0);
        }

        if (s.v[1349] != 0.0) {
            s.store_scalar(1395, 0.0);
        }

        if (s.v[1349] != 0.0) {
            s.store_scalar(1396, 0.0);
        }

        if (s.v[1349] != 0.0) {
            s.store_scalar(1397, 0.0);
        }

        if (s.v[1349] != 0.0) {
            s.store_scalar(1398, 0.0);
        }

        if (s.v[1349] != 0.0) {
            s.store_scalar(1399, 0.0);
        }

        if (s.v[1349] != 0.0) {
            s.store_scalar(1400, 0.0);
        }

        if (s.v[1349] != 0.0) {
            s.store_scalar(1401, 0.0);
        }

        if (s.v[1349] != 0.0) {
            s.store_scalar(1402, 0.0);
        }

        if (s.v[1349] != 0.0) {
            s.store_scalar(1403, 0.0);
        }

        if (s.v[1349] != 0.0) {
            s.store_scalar(1404, 0.0);
        }

        if (s.v[1349] != 0.0) {
            s.store_scalar(1405, 0.0);
        }

        if (s.v[1349] != 0.0) {
            s.store_scalar(1406, 0.0);
        }

        if (s.v[1349] != 0.0) {
            s.store_scalar(1407, 0.0);
        }

        if (s.v[1349] != 0.0) {
            s.store_scalar(1408, 0.0);
        }

        if (s.v[1349] != 0.0) {
            s.store_scalar(1409, 0.0);
        }

        if (s.v[1349] != 0.0) {
            s.store_scalar(1410, 0.0);
        }

        if (s.v[1349] != 0.0) {
            s.store_scalar(1411, 0.0);
        }

        if (s.v[1349] != 0.0) {
            s.store_scalar(1412, 0.0);
        }

        if (s.v[1349] != 0.0) {
            s.store_scalar(1413, 0.0);
        }

    }

    pub(super) fn stamp_transient_block_18(
        &mut self,
        ctx: &GeneratedEvalContext<'_>,
        stamper: &mut GeneratedStamper<'_>,
        s: &mut Scratch,
    ) {
        let _ = stamper;
        let p = self.params;
        let nodes = self.nodes;
        let branches = self.branches;
        if (s.v[1349] != 0.0) {
            s.store_scalar(1414, 0.0);
        }

        if (s.v[1349] != 0.0) {
            s.store_scalar(1415, 0.0);
        }

        if (s.v[1349] != 0.0) {
            s.store_scalar(1416, 0.0);
        }

        if (s.v[1349] != 0.0) {
            s.store_scalar(1417, 0.0);
        }

        if (s.v[1349] != 0.0) {
            s.store_scalar(1418, 0.0);
        }

        if (s.v[1349] != 0.0) {
            s.store_scalar(1419, 0.0);
        }

        if (s.v[1349] != 0.0) {
            s.store_scalar(1420, 0.0);
        }

        if (s.v[1349] != 0.0) {
            s.store_scalar(1421, 0.0);
        }

        if (s.v[1349] != 0.0) {
            s.store_scalar(1422, 0.0);
        }

        if (s.v[1349] != 0.0) {
            s.store_scalar(1423, 0.0);
        }

        if (s.v[1349] != 0.0) {
            s.store_scalar(1424, 0.0);
        }

        if (s.v[1349] != 0.0) {
            s.store_scalar(1425, 0.0);
        }

        if (s.v[1349] != 0.0) {
            s.store_scalar(1426, 0.0);
        }

        if (s.v[1349] != 0.0) {
            s.store_scalar(1427, 0.0);
        }

        if (s.v[1349] != 0.0) {
            s.store_scalar(1428, 0.0);
        }

        if (s.v[1349] != 0.0) {
            s.store_scalar(1429, 0.0);
        }

        if (s.v[1349] != 0.0) {
            s.store_scalar(1430, 0.0);
        }

        if (s.v[1349] != 0.0) {
            s.store_scalar(1431, 0.0);
        }

        if (s.v[1349] != 0.0) {
            s.store_scalar(1432, 0.0);
        }

        if (s.v[1349] != 0.0) {
            s.store_scalar(1433, 0.0);
        }

        if (s.v[1349] != 0.0) {
            s.store_scalar(1434, 0.0);
        }

        if (s.v[1349] != 0.0) {
            s.store_scalar(1435, 0.0);
        }

        if (s.v[1349] != 0.0) {
            s.store_scalar(1436, 0.0);
        }

        if (s.v[1349] != 0.0) {
            s.store_scalar(1437, 0.0);
        }

        if (s.v[1349] != 0.0) {
            s.store_scalar(1438, 0.0);
        }

        if (s.v[1349] != 0.0) {
            s.store_scalar(1439, 0.0);
        }

        if (s.v[1349] != 0.0) {
            s.store_scalar(1440, 0.0);
        }

        if (s.v[1349] != 0.0) {
            s.store_scalar(1441, 0.0);
        }

        if (s.v[1349] != 0.0) {
            s.store_scalar(1442, 0.0);
        }

        if (s.v[1349] != 0.0) {
            s.store_scalar(1443, 0.0);
        }

        if (s.v[1349] != 0.0) {
            s.store_scalar(1444, 0.0);
        }

        if (s.v[1349] != 0.0) {
            s.store_scalar(1445, 0.0);
        }

        if (s.v[1349] != 0.0) {
            s.store_scalar(1446, 0.0);
        }

        if (s.v[1349] != 0.0) {
            s.store_scalar(1447, 0.0);
        }

        if (s.v[1349] != 0.0) {
            s.store_scalar(1448, 0.0);
        }

        if (s.v[1349] != 0.0) {
            s.store_scalar(1449, 0.0);
        }

        if (s.v[1349] != 0.0) {
            s.store_scalar(1450, 0.0);
        }

        if (s.v[1349] != 0.0) {
            s.store_scalar(1451, 0.0);
        }

        if (s.v[1349] != 0.0) {
            s.store_scalar(1452, 0.0);
        }

        if (s.v[1349] != 0.0) {
            s.store_scalar(1453, 0.0);
        }

        if (s.v[1349] != 0.0) {
            s.store_scalar(1454, 0.0);
        }

        if (s.v[1349] != 0.0) {
            s.store_scalar(1455, 0.0);
        }

        if (s.v[1349] != 0.0) {
            s.store_scalar(1456, 0.0);
        }

        if (s.v[1349] != 0.0) {
            s.store_scalar(1457, 0.0);
        }

        if (s.v[1349] != 0.0) {
            s.store_scalar(1458, 0.0);
        }

        if (s.v[1349] != 0.0) {
            s.store_scalar(1459, 0.0);
        }

        if (s.v[1349] != 0.0) {
            s.store_scalar(1460, 0.0);
        }

        if (s.v[1349] != 0.0) {
            s.store_scalar(1461, 0.0);
        }

        if (s.v[1349] != 0.0) {
            s.store_ad(1458, &{
                if (!(p.p52 == 0.0)) {
                    A::mul(s.ad_value(1360), A::tanh(A::scale(s.ad_value(1360), (0.001 / p.p53))))
                } else {
                    {
                        if (p.p52 == 0.0) {
                            A::sqrt(A::offset(A::square(s.ad_value(1360)), p.p53))
                        } else {
                            A::constant(0.0)
                        }
                    }
                }
            });
        }

        if (s.v[1349] != 0.0) {
            s.store_sub(1459, 1359, 1360);
        }

        if (s.v[1349] != 0.0) {
            s.store_mul(1393, 1379, 1367);
        }

        if (s.v[1349] != 0.0) {
            s.store_add_ad(1395, A::div(s.ad_value(1375), A::scale(s.ad_value(1367), 2.302585092994046)), A::mul(s.ad_value(1378), s.ad_value(1458)));
        }

        if (s.v[1349] != 0.0) {
            s.store_add_ad_rhs(1396, 1374, A::mul(s.ad_value(1385), A::sub(s.ad_value(1365), s.ad_value(1366))));
        }

        if (s.v[1349] != 0.0) {
            s.store_ad(1414, &A::pow(A::div(s.ad_value(1365), s.ad_value(1366)), s.ad_value(1387)));
        }

        s.v[1462] = if (s.v[1386] != 0.0) { 1.0 } else { 0.0 };

        if ((s.v[1349] != 0.0) && (s.v[1462] != 0.0)) {
            s.store_div_ad_rhs(1397, 1458, A::pow(A::offset(A::pow(A::div(s.ad_value(1458), s.ad_value(1386)), s.ad_value(1382)), 1.0), A::div_from_scalar(1.0, s.ad_value(1382))));
        }

        if ((s.v[1349] != 0.0) && (!(s.v[1462] != 0.0))) {
            s.store_scalar(1397, 0.0);
        }

        if (s.v[1349] != 0.0) {
            s.store_mul_ad_lhs(1394, A::sub(s.ad_value(1376), A::mul(s.ad_value(1397), s.ad_value(1377))), 1458);
        }

        if (s.v[1349] != 0.0) {
            s.store_sub(1357, 1396, 1394);
        }

        if (s.v[1349] != 0.0) {
            s.store_mul_ad_lhs(1399, A::scale(s.ad_value(1395), 2.0), 1367);
        }

        if (s.v[1349] != 0.0) {
            s.store_mul(1400, 1370, 1399);
        }

        if (s.v[1349] != 0.0) {
            s.store_sub_ad_rhs(1457, 1357, A::scale(s.ad_value(1393), (p.p51 * 0.5)));
        }

        if (s.v[1349] != 0.0) {
            let assign22900_ad_e21483: A = {
                if (!(p.p52 == 0.0)) {
                    A::scale(A::add(A::add(s.ad_value(1359), s.ad_value(1459)), A::mul(A::sub(s.ad_value(1359), s.ad_value(1459)), A::tanh(A::scale(A::sub(s.ad_value(1359), s.ad_value(1459)), (0.001 / p.p53))))), 0.5)
                } else {
                    {
                        if (p.p52 == 0.0) {
                            A::scale(A::add(A::add(s.ad_value(1359), s.ad_value(1459)), A::sqrt(A::offset(A::mul(A::sub(s.ad_value(1359), s.ad_value(1459)), A::sub(s.ad_value(1359), s.ad_value(1459))), p.p53))), 0.5)
                        } else {
                            A::constant(0.0)
                        }
                    }
                }
            };
            s.store_div_ad_lhs(1456, A::sub(assign22900_ad_e21483, s.ad_value(1457)), 1393);
        }

        s.v[1463] = if (s.v[1456] > 50.0) { 1.0 } else { 0.0 };

        if ((s.v[1349] != 0.0) && (s.v[1463] != 0.0)) {
            s.store_scalar(1415, 0.0);
        }

        s.v[1464] = if (s.v[1456] < (-50.0)) { 1.0 } else { 0.0 };

        if (((s.v[1349] != 0.0) && (!(s.v[1463] != 0.0))) && (s.v[1464] != 0.0)) {
            s.store_scalar(1415, 1.0);
        }

        if (((s.v[1349] != 0.0) && (!(s.v[1463] != 0.0))) && (!(s.v[1464] != 0.0))) {
            s.store_div_from_scalar_ad(1415, 1.0, A::offset(A::exp(s.ad_value(1456)), 1.0));
        }

        if (s.v[1349] != 0.0) {
            let assign22960_ad_e21571: A = {
                if (!(p.p52 == 0.0)) {
                    A::scale(A::add(A::add(s.ad_value(1359), s.ad_value(1459)), A::mul(A::sub(s.ad_value(1359), s.ad_value(1459)), A::tanh(A::scale(A::sub(s.ad_value(1359), s.ad_value(1459)), (0.001 / p.p53))))), 0.5)
                } else {
                    {
                        if (p.p52 == 0.0) {
                            A::scale(A::add(A::add(s.ad_value(1359), s.ad_value(1459)), A::sqrt(A::offset(A::mul(A::sub(s.ad_value(1359), s.ad_value(1459)), A::sub(s.ad_value(1359), s.ad_value(1459))), p.p53))), 0.5)
                        } else {
                            A::constant(0.0)
                        }
                    }
                }
            };
            s.store_div_ad_lhs(1416, A::sub(assign22960_ad_e21571, A::sub(s.ad_value(1357), A::mul(A::scale(s.ad_value(1393), (p.p51 * 0.1)), s.ad_value(1415)))), 1399);
        }

        s.v[1465] = if (s.v[1416] > 50.0) { 1.0 } else { 0.0 };

        if ((s.v[1349] != 0.0) && (s.v[1465] != 0.0)) {
            s.store_mul(1417, 1400, 1416);
        }

        s.v[1466] = if (s.v[1416] < (-50.0)) { 1.0 } else { 0.0 };

        if (((s.v[1349] != 0.0) && (!(s.v[1465] != 0.0))) && (s.v[1466] != 0.0)) {
            s.store_mul_ad_rhs(1417, 1400, A::exp(s.ad_value(1416)));
        }

        if (((s.v[1349] != 0.0) && (!(s.v[1465] != 0.0))) && (!(s.v[1466] != 0.0))) {
            s.store_mul_ad_rhs(1417, 1400, A::ln(A::offset(A::exp(s.ad_value(1416)), 1.0)));
        }

        if (s.v[1349] != 0.0) {
            s.store_div_ad_rhs(1403, 1381, A::mul(s.ad_value(1414), A::offset(A::div(A::mul(s.ad_value(1383), s.ad_value(1417)), s.ad_value(1370)), 1.0)));
        }

        if (s.v[1349] != 0.0) {
            s.store_div_ad(1404, A::mul(A::mul(s.ad_value(1380), A::div(A::offset(A::mul(s.ad_value(1388), s.ad_value(1366)), 1.0), A::offset(A::mul(s.ad_value(1388), s.ad_value(1365)), 1.0))), A::offset(A::div(A::mul(s.ad_value(1389), s.ad_value(1458)), s.ad_value(1369)), 1.0)), A::offset(A::div(A::mul(s.ad_value(1384), s.ad_value(1417)), s.ad_value(1370)), 1.0));
        }

        if (s.v[1349] != 0.0) {
            s.store_add_ad(1405, A::div(A::mul(A::mul(A::scale(s.ad_value(1415), 2.0), s.ad_value(1367)), s.ad_value(1403)), s.ad_value(1369)), A::mul(A::sub_from_scalar(1.0, s.ad_value(1415)), s.ad_value(1404)));
        }

        if (s.v[1349] != 0.0) {
            s.store_div_ad_lhs(1421, A::mul(s.ad_value(1404), s.ad_value(1369)), 1403);
        }

        if (s.v[1349] != 0.0) {
            s.store_sub_ad_lhs(1422, A::mul(s.ad_value(1421), A::sqrt(A::offset(A::div(A::div(A::scale(s.ad_value(1417), 2.0), s.ad_value(1370)), s.ad_value(1421)), 1.0))), 1421);
        }

        if (s.v[1349] != 0.0) {
            s.store_add_ad(1423, A::mul(s.ad_value(1421), A::sub_from_scalar(1.0, s.ad_value(1415))), A::mul(s.ad_value(1399), s.ad_value(1415)));
        }

        if (s.v[1349] != 0.0) {
            s.store_add_ad(1358, A::mul(s.ad_value(1422), A::sub_from_scalar(1.0, s.ad_value(1415))), A::mul(s.ad_value(1399), s.ad_value(1415)));
        }

        if (s.v[1349] != 0.0) {
            let assign23090_ad_e21800: A = {
                if (!(p.p52 == 0.0)) {
                    A::scale(A::add(A::div(s.ad_value(1360), s.ad_value(1358)), A::mul(A::neg(A::div(s.ad_value(1360), s.ad_value(1358))), A::tanh(A::scale(A::neg(A::div(s.ad_value(1360), s.ad_value(1358))), (0.001 / p.p53))))), 0.5)
                } else {
                    {
                        if (p.p52 == 0.0) {
                            A::scale(A::add(A::div(s.ad_value(1360), s.ad_value(1358)), A::sqrt(A::offset(A::mul(A::neg(A::div(s.ad_value(1360), s.ad_value(1358))), A::neg(A::div(s.ad_value(1360), s.ad_value(1358)))), p.p53))), 0.5)
                        } else {
                            A::constant(0.0)
                        }
                    }
                }
            };
            s.store_div_from_scalar_ad(1424, 1.0, A::pow(A::offset(A::pow(assign23090_ad_e21800, s.ad_value(1382)), 1.0), A::div_from_scalar(1.0, s.ad_value(1382))));
        }

        if (s.v[1349] != 0.0) {
            s.store_mul(1425, 1360, 1424);
        }

        if (s.v[1349] != 0.0) {
            let assign23110_ad_e21881: A = {
                if (!(p.p52 == 0.0)) {
                    A::scale(A::add(A::div(A::neg(s.ad_value(1360)), s.ad_value(1358)), A::mul(A::neg(A::div(A::neg(s.ad_value(1360)), s.ad_value(1358))), A::tanh(A::scale(A::neg(A::div(A::neg(s.ad_value(1360)), s.ad_value(1358))), (0.001 / p.p53))))), 0.5)
                } else {
                    {
                        if (p.p52 == 0.0) {
                            A::scale(A::add(A::div(A::neg(s.ad_value(1360)), s.ad_value(1358)), A::sqrt(A::offset(A::mul(A::neg(A::div(A::neg(s.ad_value(1360)), s.ad_value(1358))), A::neg(A::div(A::neg(s.ad_value(1360)), s.ad_value(1358)))), p.p53))), 0.5)
                        } else {
                            A::constant(0.0)
                        }
                    }
                }
            };
            s.store_div_from_scalar_ad(1426, 1.0, A::pow(A::offset(A::pow(assign23110_ad_e21881, s.ad_value(1382)), 1.0), A::div_from_scalar(1.0, s.ad_value(1382))));
        }

        if (s.v[1349] != 0.0) {
            s.store_mul_ad_lhs(1427, A::neg(s.ad_value(1360)), 1426);
        }

        if (s.v[1349] != 0.0) {
            s.store_div_ad_lhs(1456, A::sub(s.ad_value(1359), s.ad_value(1457)), 1393);
        }

        s.v[1467] = if (s.v[1456] > 50.0) { 1.0 } else { 0.0 };

        if ((s.v[1349] != 0.0) && (s.v[1467] != 0.0)) {
            s.store_scalar(1398, 0.0);
        }

        s.v[1468] = if (s.v[1456] < (-50.0)) { 1.0 } else { 0.0 };

        if (((s.v[1349] != 0.0) && (!(s.v[1467] != 0.0))) && (s.v[1468] != 0.0)) {
            s.store_scalar(1398, 1.0);
        }

        if (((s.v[1349] != 0.0) && (!(s.v[1467] != 0.0))) && (!(s.v[1468] != 0.0))) {
            s.store_div_from_scalar_ad(1398, 1.0, A::offset(A::exp(s.ad_value(1456)), 1.0));
        }

        if (s.v[1349] != 0.0) {
            s.store_div_ad_lhs(1401, A::sub(A::sub(s.ad_value(1459), s.ad_value(1427)), A::sub(s.ad_value(1357), A::mul(A::scale(s.ad_value(1393), (p.p51 * 0.1)), s.ad_value(1398)))), 1399);
        }

        s.v[1469] = if (s.v[1401] > 50.0) { 1.0 } else { 0.0 };

        if ((s.v[1349] != 0.0) && (s.v[1469] != 0.0)) {
            s.store_mul(1402, 1400, 1401);
        }

        s.v[1470] = if (s.v[1401] < (-50.0)) { 1.0 } else { 0.0 };

        if (((s.v[1349] != 0.0) && (!(s.v[1469] != 0.0))) && (s.v[1470] != 0.0)) {
            s.store_mul_ad_rhs(1402, 1400, A::exp(s.ad_value(1401)));
        }

        if (((s.v[1349] != 0.0) && (!(s.v[1469] != 0.0))) && (!(s.v[1470] != 0.0))) {
            s.store_mul_ad_rhs(1402, 1400, A::ln(A::offset(A::exp(s.ad_value(1401)), 1.0)));
        }

        if (s.v[1349] != 0.0) {
            s.store_div_ad_lhs(1456, A::sub(s.ad_value(1459), s.ad_value(1457)), 1393);
        }

        s.v[1471] = if (s.v[1456] > 50.0) { 1.0 } else { 0.0 };

        if ((s.v[1349] != 0.0) && (s.v[1471] != 0.0)) {
            s.store_scalar(1428, 0.0);
        }

        s.v[1472] = if (s.v[1456] < (-50.0)) { 1.0 } else { 0.0 };

        if (((s.v[1349] != 0.0) && (!(s.v[1471] != 0.0))) && (s.v[1472] != 0.0)) {
            s.store_scalar(1428, 1.0);
        }

        if (((s.v[1349] != 0.0) && (!(s.v[1471] != 0.0))) && (!(s.v[1472] != 0.0))) {
            s.store_div_from_scalar_ad(1428, 1.0, A::offset(A::exp(s.ad_value(1456)), 1.0));
        }

        if (s.v[1349] != 0.0) {
            s.store_div_ad_lhs(1429, A::sub(A::sub(s.ad_value(1359), s.ad_value(1425)), A::sub(s.ad_value(1357), A::mul(A::scale(s.ad_value(1393), (p.p51 * 0.1)), s.ad_value(1428)))), 1399);
        }

        s.v[1473] = if (s.v[1429] > 50.0) { 1.0 } else { 0.0 };

        if ((s.v[1349] != 0.0) && (s.v[1473] != 0.0)) {
            s.store_mul(1430, 1400, 1429);
        }

        s.v[1474] = if (s.v[1429] < (-50.0)) { 1.0 } else { 0.0 };

        if (((s.v[1349] != 0.0) && (!(s.v[1473] != 0.0))) && (s.v[1474] != 0.0)) {
            s.store_mul_ad_rhs(1430, 1400, A::exp(s.ad_value(1429)));
        }

        if (((s.v[1349] != 0.0) && (!(s.v[1473] != 0.0))) && (!(s.v[1474] != 0.0))) {
            s.store_mul_ad_rhs(1430, 1400, A::ln(A::offset(A::exp(s.ad_value(1429)), 1.0)));
        }

        if (s.v[1349] != 0.0) {
            s.store_div_ad_lhs(1431, A::sub(s.ad_value(1402), s.ad_value(1430)), 1370);
        }

        if (s.v[1349] != 0.0) {
            s.store_div(1457, 1431, 1423);
        }

        if (s.v[1349] != 0.0) {
            let assign23390_ad_e22158: A = A::pow(A::offset(A::pow({
                if (!(p.p52 == 0.0)) {
                    A::mul(s.ad_value(1457), A::tanh(A::scale(s.ad_value(1457), (0.001 / p.p53))))
                } else {
                    {
                        if (p.p52 == 0.0) {
                            A::sqrt(A::offset(A::square(s.ad_value(1457)), p.p53))
                        } else {
                            A::constant(0.0)
                        }
                    }
                }
            }, s.ad_value(1382)), 1.0), A::div_from_scalar(1.0, s.ad_value(1382)));
            s.store_div_ad_rhs(1432, 1457, assign23390_ad_e22158);
        }

        if (s.v[1349] != 0.0) {
            s.store_mul(1433, 1405, 1432);
        }

        if (s.v[1349] != 0.0) {
            s.store_mul_ad_lhs(1351, A::mul(A::mul(A::scale(A::mul(A::mul(s.ad_value(1391), s.ad_value(1368)), s.ad_value(1390)), 0.5), A::add(s.ad_value(1402), s.ad_value(1430))), s.ad_value(1433)), 1392);
        }

        if (s.v[1349] != 0.0) {
            s.store_div_ad_rhs(1406, 1375, A::scale(s.ad_value(1367), 2.302585092994046));
        }

    }

    pub(super) fn stamp_transient_block_19(
        &mut self,
        ctx: &GeneratedEvalContext<'_>,
        stamper: &mut GeneratedStamper<'_>,
        s: &mut Scratch,
    ) {
        let _ = stamper;
        let p = self.params;
        let nodes = self.nodes;
        let branches = self.branches;
        if (s.v[1349] != 0.0) {
            s.store_mul_ad_lhs(1408, A::scale(s.ad_value(1406), 2.0), 1367);
        }

        if (s.v[1349] != 0.0) {
            s.store_mul(1409, 1370, 1408);
        }

        if (s.v[1349] != 0.0) {
            s.store_sub_ad_rhs(1461, 1396, A::scale(s.ad_value(1393), (p.p51 * 0.5)));
        }

        if (s.v[1349] != 0.0) {
            let assign23460_ad_e22262: A = {
                if (!(p.p52 == 0.0)) {
                    A::scale(A::add(A::add(s.ad_value(1359), s.ad_value(1459)), A::mul(A::sub(s.ad_value(1359), s.ad_value(1459)), A::tanh(A::scale(A::sub(s.ad_value(1359), s.ad_value(1459)), (0.001 / p.p53))))), 0.5)
                } else {
                    {
                        if (p.p52 == 0.0) {
                            A::scale(A::add(A::add(s.ad_value(1359), s.ad_value(1459)), A::sqrt(A::offset(A::mul(A::sub(s.ad_value(1359), s.ad_value(1459)), A::sub(s.ad_value(1359), s.ad_value(1459))), p.p53))), 0.5)
                        } else {
                            A::constant(0.0)
                        }
                    }
                }
            };
            s.store_div_ad_lhs(1460, A::sub(assign23460_ad_e22262, s.ad_value(1461)), 1393);
        }

        s.v[1475] = if (s.v[1460] > 50.0) { 1.0 } else { 0.0 };

        if ((s.v[1349] != 0.0) && (s.v[1475] != 0.0)) {
            s.store_scalar(1418, 0.0);
        }

        s.v[1476] = if (s.v[1460] < (-50.0)) { 1.0 } else { 0.0 };

        if (((s.v[1349] != 0.0) && (!(s.v[1475] != 0.0))) && (s.v[1476] != 0.0)) {
            s.store_scalar(1418, 1.0);
        }

        if (((s.v[1349] != 0.0) && (!(s.v[1475] != 0.0))) && (!(s.v[1476] != 0.0))) {
            s.store_div_from_scalar_ad(1418, 1.0, A::offset(A::exp(s.ad_value(1460)), 1.0));
        }

        if (s.v[1349] != 0.0) {
            let assign23520_ad_e22350: A = {
                if (!(p.p52 == 0.0)) {
                    A::scale(A::add(A::add(s.ad_value(1359), s.ad_value(1459)), A::mul(A::sub(s.ad_value(1359), s.ad_value(1459)), A::tanh(A::scale(A::sub(s.ad_value(1359), s.ad_value(1459)), (0.001 / p.p53))))), 0.5)
                } else {
                    {
                        if (p.p52 == 0.0) {
                            A::scale(A::add(A::add(s.ad_value(1359), s.ad_value(1459)), A::sqrt(A::offset(A::mul(A::sub(s.ad_value(1359), s.ad_value(1459)), A::sub(s.ad_value(1359), s.ad_value(1459))), p.p53))), 0.5)
                        } else {
                            A::constant(0.0)
                        }
                    }
                }
            };
            s.store_div_ad_lhs(1419, A::sub(assign23520_ad_e22350, A::sub(s.ad_value(1396), A::mul(A::scale(s.ad_value(1393), (p.p51 * 0.1)), s.ad_value(1418)))), 1408);
        }

        s.v[1477] = if (s.v[1419] > 50.0) { 1.0 } else { 0.0 };

        if ((s.v[1349] != 0.0) && (s.v[1477] != 0.0)) {
            s.store_mul(1420, 1409, 1419);
        }

        s.v[1478] = if (s.v[1419] < (-50.0)) { 1.0 } else { 0.0 };

        if (((s.v[1349] != 0.0) && (!(s.v[1477] != 0.0))) && (s.v[1478] != 0.0)) {
            s.store_mul_ad_rhs(1420, 1409, A::exp(s.ad_value(1419)));
        }

        if (((s.v[1349] != 0.0) && (!(s.v[1477] != 0.0))) && (!(s.v[1478] != 0.0))) {
            s.store_mul_ad_rhs(1420, 1409, A::ln(A::offset(A::exp(s.ad_value(1419)), 1.0)));
        }

        if (s.v[1349] != 0.0) {
            s.store_div(1412, 1381, 1414);
        }

        if (s.v[1349] != 0.0) {
            s.store_mul_ad_rhs(1413, 1380, A::div(A::offset(A::mul(s.ad_value(1388), s.ad_value(1366)), 1.0), A::offset(A::mul(s.ad_value(1388), s.ad_value(1365)), 1.0)));
        }

        if (s.v[1349] != 0.0) {
            s.store_div_ad_lhs(1434, A::mul(s.ad_value(1413), s.ad_value(1369)), 1412);
        }

        if (s.v[1349] != 0.0) {
            s.store_sub_ad_lhs(1435, A::mul(s.ad_value(1434), A::sqrt(A::offset(A::div(A::div(A::scale(s.ad_value(1420), 2.0), s.ad_value(1370)), s.ad_value(1434)), 1.0))), 1434);
        }

        if (s.v[1349] != 0.0) {
            s.store_add_ad(1436, A::mul(s.ad_value(1435), A::sub_from_scalar(1.0, s.ad_value(1418))), A::mul(s.ad_value(1408), s.ad_value(1418)));
        }

        if (s.v[1349] != 0.0) {
            let assign23630_ad_e22525: A = {
                if (!(p.p52 == 0.0)) {
                    A::scale(A::add(A::div(s.ad_value(1360), s.ad_value(1436)), A::mul(A::neg(A::div(s.ad_value(1360), s.ad_value(1436))), A::tanh(A::scale(A::neg(A::div(s.ad_value(1360), s.ad_value(1436))), (0.001 / p.p53))))), 0.5)
                } else {
                    {
                        if (p.p52 == 0.0) {
                            A::scale(A::add(A::div(s.ad_value(1360), s.ad_value(1436)), A::sqrt(A::offset(A::mul(A::neg(A::div(s.ad_value(1360), s.ad_value(1436))), A::neg(A::div(s.ad_value(1360), s.ad_value(1436)))), p.p53))), 0.5)
                        } else {
                            A::constant(0.0)
                        }
                    }
                }
            };
            s.store_div_from_scalar_ad(1437, 1.0, A::pow(A::offset(A::pow(assign23630_ad_e22525, s.ad_value(1382)), 1.0), A::div_from_scalar(1.0, s.ad_value(1382))));
        }

        if (s.v[1349] != 0.0) {
            s.store_mul(1438, 1360, 1437);
        }

        if (s.v[1349] != 0.0) {
            let assign23650_ad_e22606: A = {
                if (!(p.p52 == 0.0)) {
                    A::scale(A::add(A::div(A::neg(s.ad_value(1360)), s.ad_value(1436)), A::mul(A::neg(A::div(A::neg(s.ad_value(1360)), s.ad_value(1436))), A::tanh(A::scale(A::neg(A::div(A::neg(s.ad_value(1360)), s.ad_value(1436))), (0.001 / p.p53))))), 0.5)
                } else {
                    {
                        if (p.p52 == 0.0) {
                            A::scale(A::add(A::div(A::neg(s.ad_value(1360)), s.ad_value(1436)), A::sqrt(A::offset(A::mul(A::neg(A::div(A::neg(s.ad_value(1360)), s.ad_value(1436))), A::neg(A::div(A::neg(s.ad_value(1360)), s.ad_value(1436)))), p.p53))), 0.5)
                        } else {
                            A::constant(0.0)
                        }
                    }
                }
            };
            s.store_div_from_scalar_ad(1439, 1.0, A::pow(A::offset(A::pow(assign23650_ad_e22606, s.ad_value(1382)), 1.0), A::div_from_scalar(1.0, s.ad_value(1382))));
        }

        if (s.v[1349] != 0.0) {
            s.store_mul_ad_lhs(1440, A::neg(s.ad_value(1360)), 1439);
        }

        if (s.v[1349] != 0.0) {
            s.store_div_ad_lhs(1460, A::sub(s.ad_value(1359), s.ad_value(1461)), 1393);
        }

        s.v[1479] = if (s.v[1460] > 50.0) { 1.0 } else { 0.0 };

        if ((s.v[1349] != 0.0) && (s.v[1479] != 0.0)) {
            s.store_scalar(1407, 0.0);
        }

        s.v[1480] = if (s.v[1460] < (-50.0)) { 1.0 } else { 0.0 };

        if (((s.v[1349] != 0.0) && (!(s.v[1479] != 0.0))) && (s.v[1480] != 0.0)) {
            s.store_scalar(1407, 1.0);
        }

        if (((s.v[1349] != 0.0) && (!(s.v[1479] != 0.0))) && (!(s.v[1480] != 0.0))) {
            s.store_div_from_scalar_ad(1407, 1.0, A::offset(A::exp(s.ad_value(1460)), 1.0));
        }

        if (s.v[1349] != 0.0) {
            s.store_div_ad_lhs(1410, A::sub(A::sub(s.ad_value(1459), s.ad_value(1440)), A::sub(s.ad_value(1396), A::mul(A::scale(s.ad_value(1393), (p.p51 * 0.1)), s.ad_value(1407)))), 1408);
        }

        s.v[1481] = if (s.v[1410] > 50.0) { 1.0 } else { 0.0 };

        if ((s.v[1349] != 0.0) && (s.v[1481] != 0.0)) {
            s.store_mul(1411, 1409, 1410);
        }

        s.v[1482] = if (s.v[1410] < (-50.0)) { 1.0 } else { 0.0 };

        if (((s.v[1349] != 0.0) && (!(s.v[1481] != 0.0))) && (s.v[1482] != 0.0)) {
            s.store_mul_ad_rhs(1411, 1409, A::exp(s.ad_value(1410)));
        }

        if (((s.v[1349] != 0.0) && (!(s.v[1481] != 0.0))) && (!(s.v[1482] != 0.0))) {
            s.store_mul_ad_rhs(1411, 1409, A::ln(A::offset(A::exp(s.ad_value(1410)), 1.0)));
        }

        if (s.v[1349] != 0.0) {
            s.store_div_ad_lhs(1460, A::sub(s.ad_value(1459), s.ad_value(1461)), 1393);
        }

        s.v[1483] = if (s.v[1460] > 50.0) { 1.0 } else { 0.0 };

        if ((s.v[1349] != 0.0) && (s.v[1483] != 0.0)) {
            s.store_scalar(1441, 0.0);
        }

        s.v[1484] = if (s.v[1460] < (-50.0)) { 1.0 } else { 0.0 };

        if (((s.v[1349] != 0.0) && (!(s.v[1483] != 0.0))) && (s.v[1484] != 0.0)) {
            s.store_scalar(1441, 1.0);
        }

        if (((s.v[1349] != 0.0) && (!(s.v[1483] != 0.0))) && (!(s.v[1484] != 0.0))) {
            s.store_div_from_scalar_ad(1441, 1.0, A::offset(A::exp(s.ad_value(1460)), 1.0));
        }

        if (s.v[1349] != 0.0) {
            s.store_div_ad_lhs(1442, A::sub(A::sub(s.ad_value(1359), s.ad_value(1438)), A::sub(s.ad_value(1396), A::mul(A::scale(s.ad_value(1393), (p.p51 * 0.1)), s.ad_value(1441)))), 1408);
        }

        s.v[1485] = if (s.v[1442] > 50.0) { 1.0 } else { 0.0 };

        if ((s.v[1349] != 0.0) && (s.v[1485] != 0.0)) {
            s.store_mul(1443, 1409, 1442);
        }

        s.v[1486] = if (s.v[1442] < (-50.0)) { 1.0 } else { 0.0 };

        if (((s.v[1349] != 0.0) && (!(s.v[1485] != 0.0))) && (s.v[1486] != 0.0)) {
            s.store_mul_ad_rhs(1443, 1409, A::exp(s.ad_value(1442)));
        }

        if (((s.v[1349] != 0.0) && (!(s.v[1485] != 0.0))) && (!(s.v[1486] != 0.0))) {
            s.store_mul_ad_rhs(1443, 1409, A::ln(A::offset(A::exp(s.ad_value(1442)), 1.0)));
        }

        if (s.v[1349] != 0.0) {
            s.store_offset_ad(1444, A::square(s.ad_value(1411)), 1e-38);
        }

        if (s.v[1349] != 0.0) {
            s.store_offset_ad(1445, A::mul(s.ad_value(1444), s.ad_value(1411)), 1e-57);
        }

        if (s.v[1349] != 0.0) {
            s.store_offset_ad(1446, A::square(s.ad_value(1443)), 1e-38);
        }

        if (s.v[1349] != 0.0) {
            s.store_offset_ad(1447, A::mul(s.ad_value(1446), s.ad_value(1443)), 1e-57);
        }

        if (s.v[1349] != 0.0) {
            s.store_offset_ad(1448, A::mul(s.ad_value(1411), s.ad_value(1443)), 1e-38);
        }

        if (s.v[1349] != 0.0) {
            s.store_div_ad(1449, A::scale(A::add(A::add(s.ad_value(1444), s.ad_value(1446)), s.ad_value(1448)), (2.0 / 3.0)), A::offset(A::add(s.ad_value(1411), s.ad_value(1443)), 2e-19));
        }

        if (s.v[1349] != 0.0) {
            s.store_div_ad(1450, A::scale(A::add(A::add(A::add(A::scale(s.ad_value(1445), 2.0), A::scale(s.ad_value(1447), 3.0)), A::mul(A::scale(s.ad_value(1444), 4.0), s.ad_value(1443))), A::mul(A::scale(s.ad_value(1446), 6.0), s.ad_value(1411))), 2.0), A::scale(A::add(A::add(s.ad_value(1444), s.ad_value(1446)), A::scale(s.ad_value(1448), 2.0)), 15.0));
        }

        if (s.v[1349] != 0.0) {
            s.store_sub(1451, 1449, 1450);
        }

        if (s.v[1349] != 0.0) {
            s.copy_ad(1452, 1450);
        }

        if (s.v[1349] != 0.0) {
            s.store_mul_ad_lhs(1352, A::mul(A::mul(A::mul(A::mul(s.ad_value(1368), s.ad_value(1390)), s.ad_value(1369)), s.ad_value(1391)), s.ad_value(1451)), 1392);
        }

        if (s.v[1349] != 0.0) {
            s.store_mul_ad_lhs(1353, A::mul(A::mul(A::mul(A::mul(s.ad_value(1368), s.ad_value(1390)), s.ad_value(1369)), s.ad_value(1391)), s.ad_value(1452)), 1392);
        }

        s.v[1487] = if (s.v[1361] == 1.0) { 1.0 } else { 0.0 };

        if ((s.v[1349] != 0.0) && (s.v[1487] != 0.0)) {
            s.store_div_ad_lhs(1453, A::sub(s.ad_value(1362), A::sub(s.ad_value(1396), A::scale(s.ad_value(1393), (p.p51 * 0.5)))), 1408);
        }

        s.v[1488] = if (s.v[1453] > 50.0) { 1.0 } else { 0.0 };

        if (((s.v[1349] != 0.0) && (s.v[1487] != 0.0)) && (s.v[1488] != 0.0)) {
            s.copy_ad(1456, 1453);
        }

        s.v[1489] = if (s.v[1453] < (-50.0)) { 1.0 } else { 0.0 };

        if ((((s.v[1349] != 0.0) && (s.v[1487] != 0.0)) && (!(s.v[1488] != 0.0))) && (s.v[1489] != 0.0)) {
            s.store_exp(1456, 1453);
        }

        if ((((s.v[1349] != 0.0) && (s.v[1487] != 0.0)) && (!(s.v[1488] != 0.0))) && (!(s.v[1489] != 0.0))) {
            s.store_ln_ad(1456, A::offset(A::exp(s.ad_value(1453)), 1.0));
        }

        if ((s.v[1349] != 0.0) && (s.v[1487] != 0.0)) {
            s.store_mul_ad_lhs(1354, A::mul(A::mul(A::mul(A::mul(A::mul(s.ad_value(1368), s.ad_value(1390)), s.ad_value(1391)), s.ad_value(1372)), s.ad_value(1408)), s.ad_value(1456)), 1392);
        }

        if ((s.v[1349] != 0.0) && (s.v[1487] != 0.0)) {
            s.store_div_ad_lhs(1454, A::sub(s.ad_value(1363), A::sub(s.ad_value(1396), A::scale(s.ad_value(1393), (p.p51 * 0.5)))), 1408);
        }

        s.v[1490] = if (s.v[1454] > 50.0) { 1.0 } else { 0.0 };

        if (((s.v[1349] != 0.0) && (s.v[1487] != 0.0)) && (s.v[1490] != 0.0)) {
            s.copy_ad(1456, 1454);
        }

        s.v[1491] = if (s.v[1454] < (-50.0)) { 1.0 } else { 0.0 };

        if ((((s.v[1349] != 0.0) && (s.v[1487] != 0.0)) && (!(s.v[1490] != 0.0))) && (s.v[1491] != 0.0)) {
            s.store_exp(1456, 1454);
        }

        if ((((s.v[1349] != 0.0) && (s.v[1487] != 0.0)) && (!(s.v[1490] != 0.0))) && (!(s.v[1491] != 0.0))) {
            s.store_ln_ad(1456, A::offset(A::exp(s.ad_value(1454)), 1.0));
        }

        if ((s.v[1349] != 0.0) && (s.v[1487] != 0.0)) {
            s.store_mul_ad_lhs(1355, A::mul(A::mul(A::mul(A::mul(A::mul(s.ad_value(1368), s.ad_value(1390)), s.ad_value(1391)), s.ad_value(1373)), s.ad_value(1408)), s.ad_value(1456)), 1392);
        }

        if ((s.v[1349] != 0.0) && (!(s.v[1487] != 0.0))) {
            s.store_scalar(1354, 0.0);
        }

        if ((s.v[1349] != 0.0) && (!(s.v[1487] != 0.0))) {
            s.store_scalar(1355, 0.0);
        }

        s.v[1492] = if (s.v[1364] == 1.0) { 1.0 } else { 0.0 };

        if ((s.v[1349] != 0.0) && (s.v[1492] != 0.0)) {
            s.store_div_ad_lhs(1455, A::sub(s.ad_value(1359), A::sub(s.ad_value(1396), A::scale(s.ad_value(1393), (p.p51 * 0.5)))), 1408);
        }

        s.v[1493] = if (s.v[1455] > 50.0) { 1.0 } else { 0.0 };

        if (((s.v[1349] != 0.0) && (s.v[1492] != 0.0)) && (s.v[1493] != 0.0)) {
            s.copy_ad(1456, 1455);
        }

        s.v[1494] = if (s.v[1455] < (-50.0)) { 1.0 } else { 0.0 };

        if ((((s.v[1349] != 0.0) && (s.v[1492] != 0.0)) && (!(s.v[1493] != 0.0))) && (s.v[1494] != 0.0)) {
            s.store_exp(1456, 1455);
        }

        if ((((s.v[1349] != 0.0) && (s.v[1492] != 0.0)) && (!(s.v[1493] != 0.0))) && (!(s.v[1494] != 0.0))) {
            s.store_ln_ad(1456, A::offset(A::exp(s.ad_value(1455)), 1.0));
        }

        if ((s.v[1349] != 0.0) && (s.v[1492] != 0.0)) {
            s.store_mul_ad_lhs(1356, A::mul(A::mul(A::mul(A::mul(A::mul(s.ad_value(1368), s.ad_value(1390)), s.ad_value(1391)), s.ad_value(1371)), s.ad_value(1408)), s.ad_value(1456)), 1392);
        }

        if ((s.v[1349] != 0.0) && (!(s.v[1492] != 0.0))) {
            s.store_scalar(1356, 0.0);
        }

        if (s.v[1349] != 0.0) {
            s.copy_ad(1350, 1351);
        }

        if (s.v[1349] != 0.0) {
            s.copy_ad(184, 1351);
        }

        if (s.v[1349] != 0.0) {
            s.copy_ad(185, 1352);
        }

        if (s.v[1349] != 0.0) {
            s.copy_ad(186, 1353);
        }

        if (s.v[1349] != 0.0) {
            s.copy_ad(187, 1354);
        }

        if (s.v[1349] != 0.0) {
            s.copy_ad(188, 1355);
        }

        if (s.v[1349] != 0.0) {
            s.copy_ad(189, 1356);
        }

        if (s.v[1349] != 0.0) {
            s.copy_ad(184, 1350);
        }

        s.v[1495] = if (p.p144 == 1.0) { 1.0 } else { 0.0 };

        s.v[154] = 0.0;

        s.v[1496] = if ((p.p50 == 0.0) && (p.p54 > p.p354)) { 1.0 } else { 0.0 };

        if (s.v[1496] != 0.0) {
            s.store_scalar(1497, 0.0);
        }

        if (s.v[1496] != 0.0) {
            s.store_scalar(1498, 0.0);
        }

        if (s.v[1496] != 0.0) {
            s.store_scalar(1504, 0.0);
        }

        if (s.v[1496] != 0.0) {
            s.store_scalar(1505, 0.0);
        }

        if (s.v[1496] != 0.0) {
            s.copy_ad(1506, 52);
        }

        if (s.v[1496] != 0.0) {
            s.copy_ad(1507, 53);
        }

        if (s.v[1496] != 0.0) {
            s.store_scalar(1508, 0.0);
        }

        if (s.v[1496] != 0.0) {
            s.store_scalar(1509, 0.0);
        }

        if (s.v[1496] != 0.0) {
            s.store_scalar(1510, 0.0);
        }

        if (s.v[1496] != 0.0) {
            s.store_scalar(1511, 0.0);
        }

        if (s.v[1496] != 0.0) {
            s.copy_ad(1512, 111);
        }

        if (s.v[1496] != 0.0) {
            s.store_scalar(1513, s.v[109]);
        }

        if (s.v[1496] != 0.0) {
            s.copy_ad(1514, 113);
        }

        if (s.v[1496] != 0.0) {
            s.store_scalar(1515, p.p0);
        }

        if (s.v[1496] != 0.0) {
            s.store_scalar(1516, p.p54);
        }

        if (s.v[1496] != 0.0) {
            s.store_scalar(1517, p.p56);
        }

        if (s.v[1496] != 0.0) {
            s.store_scalar(1521, p.p55);
        }

        if (s.v[1496] != 0.0) {
            s.store_scalar(1522, p.p61);
        }

        if (s.v[1496] != 0.0) {
            s.store_scalar(1523, p.p60);
        }

        if (s.v[1496] != 0.0) {
            s.store_scalar(1524, 0.0);
        }

        if (s.v[1496] != 0.0) {
            s.store_scalar(1525, p.p62);
        }

        if (s.v[1496] != 0.0) {
            s.store_scalar(1526, p.p65);
        }

        if (s.v[1496] != 0.0) {
            s.store_scalar(1527, p.p57);
        }

        if (s.v[1496] != 0.0) {
            s.store_scalar(1528, p.p58);
        }

        if (s.v[1496] != 0.0) {
            s.store_scalar(1529, p.p59);
        }

        if (s.v[1496] != 0.0) {
            s.store_scalar(1530, p.p64);
        }

        if (s.v[1496] != 0.0) {
            s.store_scalar(1531, p.p63);
        }

        if (s.v[1496] != 0.0) {
            s.store_scalar(1532, p.p46);
        }

        if (s.v[1496] != 0.0) {
            s.store_scalar(1533, p.p39);
        }

        if (s.v[1496] != 0.0) {
            s.store_scalar(1534, p.p47);
        }

    }

    pub(super) fn stamp_transient_block_20(
        &mut self,
        ctx: &GeneratedEvalContext<'_>,
        stamper: &mut GeneratedStamper<'_>,
        s: &mut Scratch,
    ) {
        let _ = stamper;
        let p = self.params;
        let nodes = self.nodes;
        let branches = self.branches;
        if (s.v[1496] != 0.0) {
            s.store_scalar(1535, p.p45);
        }

        if (s.v[1496] != 0.0) {
            s.store_scalar(1536, p.p42);
        }

        if (s.v[1496] != 0.0) {
            s.store_scalar(1537, p.p2);
        }

        if (s.v[1496] != 0.0) {
            s.store_scalar(1538, p.p6);
        }

        if (s.v[1496] != 0.0) {
            s.store_scalar(1539, 1.0);
        }

        if (s.v[1496] != 0.0) {
            s.store_scalar(1540, 0.0);
        }

        if (s.v[1496] != 0.0) {
            s.store_scalar(1541, 0.0);
        }

        if (s.v[1496] != 0.0) {
            s.store_scalar(1542, 0.0);
        }

        if (s.v[1496] != 0.0) {
            s.store_scalar(1543, 0.0);
        }

        if (s.v[1496] != 0.0) {
            s.store_scalar(1544, 0.0);
        }

        if (s.v[1496] != 0.0) {
            s.store_scalar(1545, 0.0);
        }

        if (s.v[1496] != 0.0) {
            s.store_scalar(1546, 0.0);
        }

        if (s.v[1496] != 0.0) {
            s.store_scalar(1547, 0.0);
        }

        if (s.v[1496] != 0.0) {
            s.store_scalar(1548, 0.0);
        }

        if (s.v[1496] != 0.0) {
            s.store_scalar(1549, 0.0);
        }

        if (s.v[1496] != 0.0) {
            s.store_scalar(1550, 0.0);
        }

        if (s.v[1496] != 0.0) {
            s.store_scalar(1551, 0.0);
        }

        if (s.v[1496] != 0.0) {
            s.store_scalar(1552, 0.0);
        }

        if (s.v[1496] != 0.0) {
            s.store_scalar(1553, 0.0);
        }

        if (s.v[1496] != 0.0) {
            s.store_scalar(1555, 0.0);
        }

        if (s.v[1496] != 0.0) {
            s.store_scalar(1561, 0.0);
        }

        if (s.v[1496] != 0.0) {
            s.store_scalar(1562, 0.0);
        }

        if (s.v[1496] != 0.0) {
            s.store_scalar(1563, 0.0);
        }

        if (s.v[1496] != 0.0) {
            s.store_scalar(1564, 0.0);
        }

        if (s.v[1496] != 0.0) {
            s.store_scalar(1568, 0.0);
        }

        if (s.v[1496] != 0.0) {
            s.store_scalar(1569, 0.0);
        }

        if (s.v[1496] != 0.0) {
            s.store_scalar(1570, 0.0);
        }

        if (s.v[1496] != 0.0) {
            s.store_scalar(1571, 0.0);
        }

        if (s.v[1496] != 0.0) {
            s.store_scalar(1572, 0.0);
        }

        if (s.v[1496] != 0.0) {
            s.store_scalar(1573, 0.0);
        }

        if (s.v[1496] != 0.0) {
            s.store_scalar(1574, 0.0);
        }

        if (s.v[1496] != 0.0) {
            s.store_scalar(1575, 0.0);
        }

        if (s.v[1496] != 0.0) {
            s.store_scalar(1576, 0.0);
        }

        if (s.v[1496] != 0.0) {
            s.store_scalar(1577, 0.0);
        }

        if (s.v[1496] != 0.0) {
            s.store_scalar(1578, 0.0);
        }

        if (s.v[1496] != 0.0) {
            s.store_scalar(1579, 0.0);
        }

        if (s.v[1496] != 0.0) {
            s.store_scalar(1580, 0.0);
        }

        if (s.v[1496] != 0.0) {
            s.store_scalar(1600, 0.0);
        }

        if (s.v[1496] != 0.0) {
            s.store_scalar(1601, 0.0);
        }

        if (s.v[1496] != 0.0) {
            s.store_scalar(1602, 0.0);
        }

        if (s.v[1496] != 0.0) {
            s.store_scalar(1603, 0.0);
        }

        if (s.v[1496] != 0.0) {
            s.store_scalar(1604, 0.0);
        }

        if (s.v[1496] != 0.0) {
            s.store_scalar(1605, 0.0);
        }

        if (s.v[1496] != 0.0) {
            s.store_scalar(1606, 0.0);
        }

        if (s.v[1496] != 0.0) {
            s.store_ad(1605, &{
                if (!(p.p52 == 0.0)) {
                    A::mul(s.ad_value(1507), A::tanh(A::scale(s.ad_value(1507), (0.001 / p.p53))))
                } else {
                    {
                        if (p.p52 == 0.0) {
                            A::sqrt(A::offset(A::square(s.ad_value(1507)), p.p53))
                        } else {
                            A::constant(0.0)
                        }
                    }
                }
            });
        }

        if (s.v[1496] != 0.0) {
            s.store_sub(1606, 1506, 1507);
        }

        if (s.v[1496] != 0.0) {
            s.store_mul(1540, 1526, 1514);
        }

        if (s.v[1496] != 0.0) {
            s.store_add_ad(1542, A::div(s.ad_value(1522), A::scale(s.ad_value(1514), 2.302585092994046)), A::mul(s.ad_value(1525), s.ad_value(1605)));
        }

        if (s.v[1496] != 0.0) {
            s.store_add_ad_rhs(1543, 1521, A::mul(s.ad_value(1532), A::sub(s.ad_value(1512), s.ad_value(1513))));
        }

        if (s.v[1496] != 0.0) {
            s.store_ad(1561, &A::pow(A::div(s.ad_value(1512), s.ad_value(1513)), s.ad_value(1534)));
        }

        s.v[1609] = if (s.v[1533] != 0.0) { 1.0 } else { 0.0 };

        if ((s.v[1496] != 0.0) && (s.v[1609] != 0.0)) {
            s.store_div_ad_rhs(1544, 1605, A::pow(A::offset(A::pow(A::div(s.ad_value(1605), s.ad_value(1533)), s.ad_value(1529)), 1.0), A::div_from_scalar(1.0, s.ad_value(1529))));
        }

        if ((s.v[1496] != 0.0) && (!(s.v[1609] != 0.0))) {
            s.store_scalar(1544, 0.0);
        }

        if (s.v[1496] != 0.0) {
            s.store_mul_ad_lhs(1541, A::sub(s.ad_value(1523), A::mul(s.ad_value(1544), s.ad_value(1524))), 1605);
        }

        if (s.v[1496] != 0.0) {
            s.store_sub(1504, 1543, 1541);
        }

        if (s.v[1496] != 0.0) {
            s.store_mul_ad_lhs(1546, A::scale(s.ad_value(1542), 2.0), 1514);
        }

        if (s.v[1496] != 0.0) {
            s.store_mul(1547, 1517, 1546);
        }

        if (s.v[1496] != 0.0) {
            s.store_sub_ad_rhs(1604, 1504, A::scale(s.ad_value(1540), (p.p51 * 0.5)));
        }

        if (s.v[1496] != 0.0) {
            let assign25740_ad_e23911: A = {
                if (!(p.p52 == 0.0)) {
                    A::scale(A::add(A::add(s.ad_value(1506), s.ad_value(1606)), A::mul(A::sub(s.ad_value(1506), s.ad_value(1606)), A::tanh(A::scale(A::sub(s.ad_value(1506), s.ad_value(1606)), (0.001 / p.p53))))), 0.5)
                } else {
                    {
                        if (p.p52 == 0.0) {
                            A::scale(A::add(A::add(s.ad_value(1506), s.ad_value(1606)), A::sqrt(A::offset(A::mul(A::sub(s.ad_value(1506), s.ad_value(1606)), A::sub(s.ad_value(1506), s.ad_value(1606))), p.p53))), 0.5)
                        } else {
                            A::constant(0.0)
                        }
                    }
                }
            };
            s.store_div_ad_lhs(1603, A::sub(assign25740_ad_e23911, s.ad_value(1604)), 1540);
        }

        s.v[1610] = if (s.v[1603] > 50.0) { 1.0 } else { 0.0 };

        if ((s.v[1496] != 0.0) && (s.v[1610] != 0.0)) {
            s.store_scalar(1562, 0.0);
        }

        s.v[1611] = if (s.v[1603] < (-50.0)) { 1.0 } else { 0.0 };

        if (((s.v[1496] != 0.0) && (!(s.v[1610] != 0.0))) && (s.v[1611] != 0.0)) {
            s.store_scalar(1562, 1.0);
        }

        if (((s.v[1496] != 0.0) && (!(s.v[1610] != 0.0))) && (!(s.v[1611] != 0.0))) {
            s.store_div_from_scalar_ad(1562, 1.0, A::offset(A::exp(s.ad_value(1603)), 1.0));
        }

        if (s.v[1496] != 0.0) {
            let assign25800_ad_e23999: A = {
                if (!(p.p52 == 0.0)) {
                    A::scale(A::add(A::add(s.ad_value(1506), s.ad_value(1606)), A::mul(A::sub(s.ad_value(1506), s.ad_value(1606)), A::tanh(A::scale(A::sub(s.ad_value(1506), s.ad_value(1606)), (0.001 / p.p53))))), 0.5)
                } else {
                    {
                        if (p.p52 == 0.0) {
                            A::scale(A::add(A::add(s.ad_value(1506), s.ad_value(1606)), A::sqrt(A::offset(A::mul(A::sub(s.ad_value(1506), s.ad_value(1606)), A::sub(s.ad_value(1506), s.ad_value(1606))), p.p53))), 0.5)
                        } else {
                            A::constant(0.0)
                        }
                    }
                }
            };
            s.store_div_ad_lhs(1563, A::sub(assign25800_ad_e23999, A::sub(s.ad_value(1504), A::mul(A::scale(s.ad_value(1540), (p.p51 * 0.1)), s.ad_value(1562)))), 1546);
        }

        s.v[1612] = if (s.v[1563] > 50.0) { 1.0 } else { 0.0 };

        if ((s.v[1496] != 0.0) && (s.v[1612] != 0.0)) {
            s.store_mul(1564, 1547, 1563);
        }

        s.v[1613] = if (s.v[1563] < (-50.0)) { 1.0 } else { 0.0 };

        if (((s.v[1496] != 0.0) && (!(s.v[1612] != 0.0))) && (s.v[1613] != 0.0)) {
            s.store_mul_ad_rhs(1564, 1547, A::exp(s.ad_value(1563)));
        }

        if (((s.v[1496] != 0.0) && (!(s.v[1612] != 0.0))) && (!(s.v[1613] != 0.0))) {
            s.store_mul_ad_rhs(1564, 1547, A::ln(A::offset(A::exp(s.ad_value(1563)), 1.0)));
        }

        if (s.v[1496] != 0.0) {
            s.store_div_ad_rhs(1550, 1528, A::mul(s.ad_value(1561), A::offset(A::div(A::mul(s.ad_value(1530), s.ad_value(1564)), s.ad_value(1517)), 1.0)));
        }

        if (s.v[1496] != 0.0) {
            s.store_div_ad(1551, A::mul(A::mul(s.ad_value(1527), A::div(A::offset(A::mul(s.ad_value(1535), s.ad_value(1513)), 1.0), A::offset(A::mul(s.ad_value(1535), s.ad_value(1512)), 1.0))), A::offset(A::div(A::mul(s.ad_value(1536), s.ad_value(1605)), s.ad_value(1516)), 1.0)), A::offset(A::div(A::mul(s.ad_value(1531), s.ad_value(1564)), s.ad_value(1517)), 1.0));
        }

        if (s.v[1496] != 0.0) {
            s.store_add_ad(1552, A::div(A::mul(A::mul(A::scale(s.ad_value(1562), 2.0), s.ad_value(1514)), s.ad_value(1550)), s.ad_value(1516)), A::mul(A::sub_from_scalar(1.0, s.ad_value(1562)), s.ad_value(1551)));
        }

        if (s.v[1496] != 0.0) {
            s.store_div_ad_lhs(1568, A::mul(s.ad_value(1551), s.ad_value(1516)), 1550);
        }

        if (s.v[1496] != 0.0) {
            s.store_sub_ad_lhs(1569, A::mul(s.ad_value(1568), A::sqrt(A::offset(A::div(A::div(A::scale(s.ad_value(1564), 2.0), s.ad_value(1517)), s.ad_value(1568)), 1.0))), 1568);
        }

        if (s.v[1496] != 0.0) {
            s.store_add_ad(1570, A::mul(s.ad_value(1568), A::sub_from_scalar(1.0, s.ad_value(1562))), A::mul(s.ad_value(1546), s.ad_value(1562)));
        }

        if (s.v[1496] != 0.0) {
            s.store_add_ad(1505, A::mul(s.ad_value(1569), A::sub_from_scalar(1.0, s.ad_value(1562))), A::mul(s.ad_value(1546), s.ad_value(1562)));
        }

        if (s.v[1496] != 0.0) {
            let assign25930_ad_e24228: A = {
                if (!(p.p52 == 0.0)) {
                    A::scale(A::add(A::div(s.ad_value(1507), s.ad_value(1505)), A::mul(A::neg(A::div(s.ad_value(1507), s.ad_value(1505))), A::tanh(A::scale(A::neg(A::div(s.ad_value(1507), s.ad_value(1505))), (0.001 / p.p53))))), 0.5)
                } else {
                    {
                        if (p.p52 == 0.0) {
                            A::scale(A::add(A::div(s.ad_value(1507), s.ad_value(1505)), A::sqrt(A::offset(A::mul(A::neg(A::div(s.ad_value(1507), s.ad_value(1505))), A::neg(A::div(s.ad_value(1507), s.ad_value(1505)))), p.p53))), 0.5)
                        } else {
                            A::constant(0.0)
                        }
                    }
                }
            };
            s.store_div_from_scalar_ad(1571, 1.0, A::pow(A::offset(A::pow(assign25930_ad_e24228, s.ad_value(1529)), 1.0), A::div_from_scalar(1.0, s.ad_value(1529))));
        }

        if (s.v[1496] != 0.0) {
            s.store_mul(1572, 1507, 1571);
        }

        if (s.v[1496] != 0.0) {
            let assign25950_ad_e24309: A = {
                if (!(p.p52 == 0.0)) {
                    A::scale(A::add(A::div(A::neg(s.ad_value(1507)), s.ad_value(1505)), A::mul(A::neg(A::div(A::neg(s.ad_value(1507)), s.ad_value(1505))), A::tanh(A::scale(A::neg(A::div(A::neg(s.ad_value(1507)), s.ad_value(1505))), (0.001 / p.p53))))), 0.5)
                } else {
                    {
                        if (p.p52 == 0.0) {
                            A::scale(A::add(A::div(A::neg(s.ad_value(1507)), s.ad_value(1505)), A::sqrt(A::offset(A::mul(A::neg(A::div(A::neg(s.ad_value(1507)), s.ad_value(1505))), A::neg(A::div(A::neg(s.ad_value(1507)), s.ad_value(1505)))), p.p53))), 0.5)
                        } else {
                            A::constant(0.0)
                        }
                    }
                }
            };
            s.store_div_from_scalar_ad(1573, 1.0, A::pow(A::offset(A::pow(assign25950_ad_e24309, s.ad_value(1529)), 1.0), A::div_from_scalar(1.0, s.ad_value(1529))));
        }

        if (s.v[1496] != 0.0) {
            s.store_mul_ad_lhs(1574, A::neg(s.ad_value(1507)), 1573);
        }

        if (s.v[1496] != 0.0) {
            s.store_div_ad_lhs(1603, A::sub(s.ad_value(1506), s.ad_value(1604)), 1540);
        }

        s.v[1614] = if (s.v[1603] > 50.0) { 1.0 } else { 0.0 };

        if ((s.v[1496] != 0.0) && (s.v[1614] != 0.0)) {
            s.store_scalar(1545, 0.0);
        }

        s.v[1615] = if (s.v[1603] < (-50.0)) { 1.0 } else { 0.0 };

        if (((s.v[1496] != 0.0) && (!(s.v[1614] != 0.0))) && (s.v[1615] != 0.0)) {
            s.store_scalar(1545, 1.0);
        }

        if (((s.v[1496] != 0.0) && (!(s.v[1614] != 0.0))) && (!(s.v[1615] != 0.0))) {
            s.store_div_from_scalar_ad(1545, 1.0, A::offset(A::exp(s.ad_value(1603)), 1.0));
        }

        if (s.v[1496] != 0.0) {
            s.store_div_ad_lhs(1548, A::sub(A::sub(s.ad_value(1606), s.ad_value(1574)), A::sub(s.ad_value(1504), A::mul(A::scale(s.ad_value(1540), (p.p51 * 0.1)), s.ad_value(1545)))), 1546);
        }

        s.v[1616] = if (s.v[1548] > 50.0) { 1.0 } else { 0.0 };

        if ((s.v[1496] != 0.0) && (s.v[1616] != 0.0)) {
            s.store_mul(1549, 1547, 1548);
        }

        s.v[1617] = if (s.v[1548] < (-50.0)) { 1.0 } else { 0.0 };

        if (((s.v[1496] != 0.0) && (!(s.v[1616] != 0.0))) && (s.v[1617] != 0.0)) {
            s.store_mul_ad_rhs(1549, 1547, A::exp(s.ad_value(1548)));
        }

        if (((s.v[1496] != 0.0) && (!(s.v[1616] != 0.0))) && (!(s.v[1617] != 0.0))) {
            s.store_mul_ad_rhs(1549, 1547, A::ln(A::offset(A::exp(s.ad_value(1548)), 1.0)));
        }

        if (s.v[1496] != 0.0) {
            s.store_div_ad_lhs(1603, A::sub(s.ad_value(1606), s.ad_value(1604)), 1540);
        }

        s.v[1618] = if (s.v[1603] > 50.0) { 1.0 } else { 0.0 };

        if ((s.v[1496] != 0.0) && (s.v[1618] != 0.0)) {
            s.store_scalar(1575, 0.0);
        }

        s.v[1619] = if (s.v[1603] < (-50.0)) { 1.0 } else { 0.0 };

        if (((s.v[1496] != 0.0) && (!(s.v[1618] != 0.0))) && (s.v[1619] != 0.0)) {
            s.store_scalar(1575, 1.0);
        }

        if (((s.v[1496] != 0.0) && (!(s.v[1618] != 0.0))) && (!(s.v[1619] != 0.0))) {
            s.store_div_from_scalar_ad(1575, 1.0, A::offset(A::exp(s.ad_value(1603)), 1.0));
        }

        if (s.v[1496] != 0.0) {
            s.store_div_ad_lhs(1576, A::sub(A::sub(s.ad_value(1506), s.ad_value(1572)), A::sub(s.ad_value(1504), A::mul(A::scale(s.ad_value(1540), (p.p51 * 0.1)), s.ad_value(1575)))), 1546);
        }

        s.v[1620] = if (s.v[1576] > 50.0) { 1.0 } else { 0.0 };

        if ((s.v[1496] != 0.0) && (s.v[1620] != 0.0)) {
            s.store_mul(1577, 1547, 1576);
        }

        s.v[1621] = if (s.v[1576] < (-50.0)) { 1.0 } else { 0.0 };

        if (((s.v[1496] != 0.0) && (!(s.v[1620] != 0.0))) && (s.v[1621] != 0.0)) {
            s.store_mul_ad_rhs(1577, 1547, A::exp(s.ad_value(1576)));
        }

        if (((s.v[1496] != 0.0) && (!(s.v[1620] != 0.0))) && (!(s.v[1621] != 0.0))) {
            s.store_mul_ad_rhs(1577, 1547, A::ln(A::offset(A::exp(s.ad_value(1576)), 1.0)));
        }

        if (s.v[1496] != 0.0) {
            s.store_div_ad_lhs(1578, A::sub(s.ad_value(1549), s.ad_value(1577)), 1517);
        }

        if (s.v[1496] != 0.0) {
            s.store_div(1604, 1578, 1570);
        }

        if (s.v[1496] != 0.0) {
            let assign26230_ad_e24586: A = A::pow(A::offset(A::pow({
                if (!(p.p52 == 0.0)) {
                    A::mul(s.ad_value(1604), A::tanh(A::scale(s.ad_value(1604), (0.001 / p.p53))))
                } else {
                    {
                        if (p.p52 == 0.0) {
                            A::sqrt(A::offset(A::square(s.ad_value(1604)), p.p53))
                        } else {
                            A::constant(0.0)
                        }
                    }
                }
            }, s.ad_value(1529)), 1.0), A::div_from_scalar(1.0, s.ad_value(1529)));
            s.store_div_ad_rhs(1579, 1604, assign26230_ad_e24586);
        }

        if (s.v[1496] != 0.0) {
            s.store_mul(1580, 1552, 1579);
        }

        if (s.v[1496] != 0.0) {
            s.store_mul_ad_lhs(1498, A::mul(A::mul(A::scale(A::mul(A::mul(s.ad_value(1538), s.ad_value(1515)), s.ad_value(1537)), 0.5), A::add(s.ad_value(1549), s.ad_value(1577))), s.ad_value(1580)), 1539);
        }

        if (s.v[1496] != 0.0) {
            s.store_div_ad_rhs(1553, 1522, A::scale(s.ad_value(1514), 2.302585092994046));
        }

        if (s.v[1496] != 0.0) {
            s.store_mul_ad_lhs(1555, A::scale(s.ad_value(1553), 2.0), 1514);
        }

        s.v[1634] = if (s.v[1508] == 1.0) { 1.0 } else { 0.0 };

        if ((s.v[1496] != 0.0) && (s.v[1634] != 0.0)) {
            s.store_div_ad_lhs(1600, A::sub(s.ad_value(1509), A::sub(s.ad_value(1543), A::scale(s.ad_value(1540), (p.p51 * 0.5)))), 1555);
        }

        s.v[1635] = if (s.v[1600] > 50.0) { 1.0 } else { 0.0 };

        if (((s.v[1496] != 0.0) && (s.v[1634] != 0.0)) && (s.v[1635] != 0.0)) {
            s.copy_ad(1603, 1600);
        }

    }

    pub(super) fn stamp_transient_block_21(
        &mut self,
        ctx: &GeneratedEvalContext<'_>,
        stamper: &mut GeneratedStamper<'_>,
        s: &mut Scratch,
    ) {
        let _ = stamper;
        let p = self.params;
        let nodes = self.nodes;
        let branches = self.branches;
        s.v[1636] = if (s.v[1600] < (-50.0)) { 1.0 } else { 0.0 };

        if ((((s.v[1496] != 0.0) && (s.v[1634] != 0.0)) && (!(s.v[1635] != 0.0))) && (s.v[1636] != 0.0)) {
            s.store_exp(1603, 1600);
        }

        if ((((s.v[1496] != 0.0) && (s.v[1634] != 0.0)) && (!(s.v[1635] != 0.0))) && (!(s.v[1636] != 0.0))) {
            s.store_ln_ad(1603, A::offset(A::exp(s.ad_value(1600)), 1.0));
        }

        if ((s.v[1496] != 0.0) && (s.v[1634] != 0.0)) {
            s.store_div_ad_lhs(1601, A::sub(s.ad_value(1510), A::sub(s.ad_value(1543), A::scale(s.ad_value(1540), (p.p51 * 0.5)))), 1555);
        }

        s.v[1637] = if (s.v[1601] > 50.0) { 1.0 } else { 0.0 };

        if (((s.v[1496] != 0.0) && (s.v[1634] != 0.0)) && (s.v[1637] != 0.0)) {
            s.copy_ad(1603, 1601);
        }

        s.v[1638] = if (s.v[1601] < (-50.0)) { 1.0 } else { 0.0 };

        if ((((s.v[1496] != 0.0) && (s.v[1634] != 0.0)) && (!(s.v[1637] != 0.0))) && (s.v[1638] != 0.0)) {
            s.store_exp(1603, 1601);
        }

        if ((((s.v[1496] != 0.0) && (s.v[1634] != 0.0)) && (!(s.v[1637] != 0.0))) && (!(s.v[1638] != 0.0))) {
            s.store_ln_ad(1603, A::offset(A::exp(s.ad_value(1601)), 1.0));
        }

        s.v[1639] = if (s.v[1511] == 1.0) { 1.0 } else { 0.0 };

        if ((s.v[1496] != 0.0) && (s.v[1639] != 0.0)) {
            s.store_div_ad_lhs(1602, A::sub(s.ad_value(1506), A::sub(s.ad_value(1543), A::scale(s.ad_value(1540), (p.p51 * 0.5)))), 1555);
        }

        s.v[1640] = if (s.v[1602] > 50.0) { 1.0 } else { 0.0 };

        if (((s.v[1496] != 0.0) && (s.v[1639] != 0.0)) && (s.v[1640] != 0.0)) {
            s.copy_ad(1603, 1602);
        }

        s.v[1641] = if (s.v[1602] < (-50.0)) { 1.0 } else { 0.0 };

        if ((((s.v[1496] != 0.0) && (s.v[1639] != 0.0)) && (!(s.v[1640] != 0.0))) && (s.v[1641] != 0.0)) {
            s.store_exp(1603, 1602);
        }

        if ((((s.v[1496] != 0.0) && (s.v[1639] != 0.0)) && (!(s.v[1640] != 0.0))) && (!(s.v[1641] != 0.0))) {
            s.store_ln_ad(1603, A::offset(A::exp(s.ad_value(1602)), 1.0));
        }

        if (s.v[1496] != 0.0) {
            s.copy_ad(1497, 1498);
        }

        if (s.v[1496] != 0.0) {
            s.copy_ad(154, 1498);
        }

        if (s.v[1496] != 0.0) {
            s.copy_ad(154, 1497);
        }

        s.v[160] = 0.0;

        s.v[1642] = if ((p.p50 == 0.0) && (p.p66 > p.p354)) { 1.0 } else { 0.0 };

        if (s.v[1642] != 0.0) {
            s.store_scalar(1643, 0.0);
        }

        if (s.v[1642] != 0.0) {
            s.store_scalar(1644, 0.0);
        }

        if (s.v[1642] != 0.0) {
            s.store_scalar(1650, 0.0);
        }

        if (s.v[1642] != 0.0) {
            s.store_scalar(1651, 0.0);
        }

        if (s.v[1642] != 0.0) {
            s.copy_ad(1652, 56);
        }

        if (s.v[1642] != 0.0) {
            s.copy_ad(1653, 57);
        }

        if (s.v[1642] != 0.0) {
            s.store_scalar(1654, 0.0);
        }

        if (s.v[1642] != 0.0) {
            s.store_scalar(1655, 0.0);
        }

        if (s.v[1642] != 0.0) {
            s.store_scalar(1656, 0.0);
        }

        if (s.v[1642] != 0.0) {
            s.store_scalar(1657, 0.0);
        }

        if (s.v[1642] != 0.0) {
            s.copy_ad(1658, 111);
        }

        if (s.v[1642] != 0.0) {
            s.store_scalar(1659, s.v[109]);
        }

        if (s.v[1642] != 0.0) {
            s.copy_ad(1660, 113);
        }

        if (s.v[1642] != 0.0) {
            s.store_scalar(1661, p.p0);
        }

        if (s.v[1642] != 0.0) {
            s.store_scalar(1662, p.p66);
        }

        if (s.v[1642] != 0.0) {
            s.store_scalar(1663, p.p68);
        }

        if (s.v[1642] != 0.0) {
            s.store_scalar(1667, p.p67);
        }

        if (s.v[1642] != 0.0) {
            s.store_scalar(1668, p.p73);
        }

        if (s.v[1642] != 0.0) {
            s.store_scalar(1669, p.p72);
        }

        if (s.v[1642] != 0.0) {
            s.store_scalar(1670, 0.0);
        }

        if (s.v[1642] != 0.0) {
            s.store_scalar(1671, p.p74);
        }

        if (s.v[1642] != 0.0) {
            s.store_scalar(1672, p.p77);
        }

        if (s.v[1642] != 0.0) {
            s.store_scalar(1673, p.p69);
        }

        if (s.v[1642] != 0.0) {
            s.store_scalar(1674, p.p70);
        }

        if (s.v[1642] != 0.0) {
            s.store_scalar(1675, p.p71);
        }

        if (s.v[1642] != 0.0) {
            s.store_scalar(1676, p.p76);
        }

        if (s.v[1642] != 0.0) {
            s.store_scalar(1677, p.p75);
        }

        if (s.v[1642] != 0.0) {
            s.store_scalar(1678, p.p46);
        }

        if (s.v[1642] != 0.0) {
            s.store_scalar(1679, p.p39);
        }

        if (s.v[1642] != 0.0) {
            s.store_scalar(1680, p.p47);
        }

        if (s.v[1642] != 0.0) {
            s.store_scalar(1681, p.p45);
        }

        if (s.v[1642] != 0.0) {
            s.store_scalar(1682, p.p42);
        }

        if (s.v[1642] != 0.0) {
            s.store_scalar(1683, p.p2);
        }

        if (s.v[1642] != 0.0) {
            s.store_scalar(1684, p.p6);
        }

        if (s.v[1642] != 0.0) {
            s.store_scalar(1685, 1.0);
        }

        if (s.v[1642] != 0.0) {
            s.store_scalar(1686, 0.0);
        }

        if (s.v[1642] != 0.0) {
            s.store_scalar(1687, 0.0);
        }

        if (s.v[1642] != 0.0) {
            s.store_scalar(1688, 0.0);
        }

        if (s.v[1642] != 0.0) {
            s.store_scalar(1689, 0.0);
        }

        if (s.v[1642] != 0.0) {
            s.store_scalar(1690, 0.0);
        }

        if (s.v[1642] != 0.0) {
            s.store_scalar(1691, 0.0);
        }

        if (s.v[1642] != 0.0) {
            s.store_scalar(1692, 0.0);
        }

        if (s.v[1642] != 0.0) {
            s.store_scalar(1693, 0.0);
        }

        if (s.v[1642] != 0.0) {
            s.store_scalar(1694, 0.0);
        }

        if (s.v[1642] != 0.0) {
            s.store_scalar(1695, 0.0);
        }

        if (s.v[1642] != 0.0) {
            s.store_scalar(1696, 0.0);
        }

        if (s.v[1642] != 0.0) {
            s.store_scalar(1697, 0.0);
        }

        if (s.v[1642] != 0.0) {
            s.store_scalar(1698, 0.0);
        }

        if (s.v[1642] != 0.0) {
            s.store_scalar(1699, 0.0);
        }

        if (s.v[1642] != 0.0) {
            s.store_scalar(1701, 0.0);
        }

        if (s.v[1642] != 0.0) {
            s.store_scalar(1707, 0.0);
        }

        if (s.v[1642] != 0.0) {
            s.store_scalar(1708, 0.0);
        }

        if (s.v[1642] != 0.0) {
            s.store_scalar(1709, 0.0);
        }

        if (s.v[1642] != 0.0) {
            s.store_scalar(1710, 0.0);
        }

        if (s.v[1642] != 0.0) {
            s.store_scalar(1714, 0.0);
        }

        if (s.v[1642] != 0.0) {
            s.store_scalar(1715, 0.0);
        }

        if (s.v[1642] != 0.0) {
            s.store_scalar(1716, 0.0);
        }

        if (s.v[1642] != 0.0) {
            s.store_scalar(1717, 0.0);
        }

        if (s.v[1642] != 0.0) {
            s.store_scalar(1718, 0.0);
        }

        if (s.v[1642] != 0.0) {
            s.store_scalar(1719, 0.0);
        }

        if (s.v[1642] != 0.0) {
            s.store_scalar(1720, 0.0);
        }

        if (s.v[1642] != 0.0) {
            s.store_scalar(1721, 0.0);
        }

        if (s.v[1642] != 0.0) {
            s.store_scalar(1722, 0.0);
        }

        if (s.v[1642] != 0.0) {
            s.store_scalar(1723, 0.0);
        }

        if (s.v[1642] != 0.0) {
            s.store_scalar(1724, 0.0);
        }

        if (s.v[1642] != 0.0) {
            s.store_scalar(1725, 0.0);
        }

        if (s.v[1642] != 0.0) {
            s.store_scalar(1726, 0.0);
        }

        if (s.v[1642] != 0.0) {
            s.store_scalar(1746, 0.0);
        }

        if (s.v[1642] != 0.0) {
            s.store_scalar(1747, 0.0);
        }

        if (s.v[1642] != 0.0) {
            s.store_scalar(1748, 0.0);
        }

        if (s.v[1642] != 0.0) {
            s.store_scalar(1749, 0.0);
        }

        if (s.v[1642] != 0.0) {
            s.store_scalar(1750, 0.0);
        }

        if (s.v[1642] != 0.0) {
            s.store_scalar(1751, 0.0);
        }

        if (s.v[1642] != 0.0) {
            s.store_scalar(1752, 0.0);
        }

        if (s.v[1642] != 0.0) {
            s.store_ad(1751, &{
                if (!(p.p52 == 0.0)) {
                    A::mul(s.ad_value(1653), A::tanh(A::scale(s.ad_value(1653), (0.001 / p.p53))))
                } else {
                    {
                        if (p.p52 == 0.0) {
                            A::sqrt(A::offset(A::square(s.ad_value(1653)), p.p53))
                        } else {
                            A::constant(0.0)
                        }
                    }
                }
            });
        }

        if (s.v[1642] != 0.0) {
            s.store_sub(1752, 1652, 1653);
        }

        if (s.v[1642] != 0.0) {
            s.store_mul(1686, 1672, 1660);
        }

        if (s.v[1642] != 0.0) {
            s.store_add_ad(1688, A::div(s.ad_value(1668), A::scale(s.ad_value(1660), 2.302585092994046)), A::mul(s.ad_value(1671), s.ad_value(1751)));
        }

        if (s.v[1642] != 0.0) {
            s.store_add_ad_rhs(1689, 1667, A::mul(s.ad_value(1678), A::sub(s.ad_value(1658), s.ad_value(1659))));
        }

        if (s.v[1642] != 0.0) {
            s.store_ad(1707, &A::pow(A::div(s.ad_value(1658), s.ad_value(1659)), s.ad_value(1680)));
        }

        s.v[1755] = if (s.v[1679] != 0.0) { 1.0 } else { 0.0 };

        if ((s.v[1642] != 0.0) && (s.v[1755] != 0.0)) {
            s.store_div_ad_rhs(1690, 1751, A::pow(A::offset(A::pow(A::div(s.ad_value(1751), s.ad_value(1679)), s.ad_value(1675)), 1.0), A::div_from_scalar(1.0, s.ad_value(1675))));
        }

        if ((s.v[1642] != 0.0) && (!(s.v[1755] != 0.0))) {
            s.store_scalar(1690, 0.0);
        }

        if (s.v[1642] != 0.0) {
            s.store_mul_ad_lhs(1687, A::sub(s.ad_value(1669), A::mul(s.ad_value(1690), s.ad_value(1670))), 1751);
        }

        if (s.v[1642] != 0.0) {
            s.store_sub(1650, 1689, 1687);
        }

        if (s.v[1642] != 0.0) {
            s.store_mul_ad_lhs(1692, A::scale(s.ad_value(1688), 2.0), 1660);
        }

        if (s.v[1642] != 0.0) {
            s.store_mul(1693, 1663, 1692);
        }

        if (s.v[1642] != 0.0) {
            s.store_sub_ad_rhs(1750, 1650, A::scale(s.ad_value(1686), (p.p51 * 0.5)));
        }

        if (s.v[1642] != 0.0) {
            let assign28570_ad_e26336: A = {
                if (!(p.p52 == 0.0)) {
                    A::scale(A::add(A::add(s.ad_value(1652), s.ad_value(1752)), A::mul(A::sub(s.ad_value(1652), s.ad_value(1752)), A::tanh(A::scale(A::sub(s.ad_value(1652), s.ad_value(1752)), (0.001 / p.p53))))), 0.5)
                } else {
                    {
                        if (p.p52 == 0.0) {
                            A::scale(A::add(A::add(s.ad_value(1652), s.ad_value(1752)), A::sqrt(A::offset(A::mul(A::sub(s.ad_value(1652), s.ad_value(1752)), A::sub(s.ad_value(1652), s.ad_value(1752))), p.p53))), 0.5)
                        } else {
                            A::constant(0.0)
                        }
                    }
                }
            };
            s.store_div_ad_lhs(1749, A::sub(assign28570_ad_e26336, s.ad_value(1750)), 1686);
        }

        s.v[1756] = if (s.v[1749] > 50.0) { 1.0 } else { 0.0 };

        if ((s.v[1642] != 0.0) && (s.v[1756] != 0.0)) {
            s.store_scalar(1708, 0.0);
        }

        s.v[1757] = if (s.v[1749] < (-50.0)) { 1.0 } else { 0.0 };

        if (((s.v[1642] != 0.0) && (!(s.v[1756] != 0.0))) && (s.v[1757] != 0.0)) {
            s.store_scalar(1708, 1.0);
        }

        if (((s.v[1642] != 0.0) && (!(s.v[1756] != 0.0))) && (!(s.v[1757] != 0.0))) {
            s.store_div_from_scalar_ad(1708, 1.0, A::offset(A::exp(s.ad_value(1749)), 1.0));
        }

        if (s.v[1642] != 0.0) {
            let assign28630_ad_e26424: A = {
                if (!(p.p52 == 0.0)) {
                    A::scale(A::add(A::add(s.ad_value(1652), s.ad_value(1752)), A::mul(A::sub(s.ad_value(1652), s.ad_value(1752)), A::tanh(A::scale(A::sub(s.ad_value(1652), s.ad_value(1752)), (0.001 / p.p53))))), 0.5)
                } else {
                    {
                        if (p.p52 == 0.0) {
                            A::scale(A::add(A::add(s.ad_value(1652), s.ad_value(1752)), A::sqrt(A::offset(A::mul(A::sub(s.ad_value(1652), s.ad_value(1752)), A::sub(s.ad_value(1652), s.ad_value(1752))), p.p53))), 0.5)
                        } else {
                            A::constant(0.0)
                        }
                    }
                }
            };
            s.store_div_ad_lhs(1709, A::sub(assign28630_ad_e26424, A::sub(s.ad_value(1650), A::mul(A::scale(s.ad_value(1686), (p.p51 * 0.1)), s.ad_value(1708)))), 1692);
        }

        s.v[1758] = if (s.v[1709] > 50.0) { 1.0 } else { 0.0 };

        if ((s.v[1642] != 0.0) && (s.v[1758] != 0.0)) {
            s.store_mul(1710, 1693, 1709);
        }

        s.v[1759] = if (s.v[1709] < (-50.0)) { 1.0 } else { 0.0 };

        if (((s.v[1642] != 0.0) && (!(s.v[1758] != 0.0))) && (s.v[1759] != 0.0)) {
            s.store_mul_ad_rhs(1710, 1693, A::exp(s.ad_value(1709)));
        }

        if (((s.v[1642] != 0.0) && (!(s.v[1758] != 0.0))) && (!(s.v[1759] != 0.0))) {
            s.store_mul_ad_rhs(1710, 1693, A::ln(A::offset(A::exp(s.ad_value(1709)), 1.0)));
        }

        if (s.v[1642] != 0.0) {
            s.store_div_ad_rhs(1696, 1674, A::mul(s.ad_value(1707), A::offset(A::div(A::mul(s.ad_value(1676), s.ad_value(1710)), s.ad_value(1663)), 1.0)));
        }

        if (s.v[1642] != 0.0) {
            s.store_div_ad(1697, A::mul(A::mul(s.ad_value(1673), A::div(A::offset(A::mul(s.ad_value(1681), s.ad_value(1659)), 1.0), A::offset(A::mul(s.ad_value(1681), s.ad_value(1658)), 1.0))), A::offset(A::div(A::mul(s.ad_value(1682), s.ad_value(1751)), s.ad_value(1662)), 1.0)), A::offset(A::div(A::mul(s.ad_value(1677), s.ad_value(1710)), s.ad_value(1663)), 1.0));
        }

        if (s.v[1642] != 0.0) {
            s.store_add_ad(1698, A::div(A::mul(A::mul(A::scale(s.ad_value(1708), 2.0), s.ad_value(1660)), s.ad_value(1696)), s.ad_value(1662)), A::mul(A::sub_from_scalar(1.0, s.ad_value(1708)), s.ad_value(1697)));
        }

        if (s.v[1642] != 0.0) {
            s.store_div_ad_lhs(1714, A::mul(s.ad_value(1697), s.ad_value(1662)), 1696);
        }

    }

    pub(super) fn stamp_transient_block_22(
        &mut self,
        ctx: &GeneratedEvalContext<'_>,
        stamper: &mut GeneratedStamper<'_>,
        s: &mut Scratch,
    ) {
        let _ = stamper;
        let p = self.params;
        let nodes = self.nodes;
        let branches = self.branches;
        if (s.v[1642] != 0.0) {
            s.store_sub_ad_lhs(1715, A::mul(s.ad_value(1714), A::sqrt(A::offset(A::div(A::div(A::scale(s.ad_value(1710), 2.0), s.ad_value(1663)), s.ad_value(1714)), 1.0))), 1714);
        }

        if (s.v[1642] != 0.0) {
            s.store_add_ad(1716, A::mul(s.ad_value(1714), A::sub_from_scalar(1.0, s.ad_value(1708))), A::mul(s.ad_value(1692), s.ad_value(1708)));
        }

        if (s.v[1642] != 0.0) {
            s.store_add_ad(1651, A::mul(s.ad_value(1715), A::sub_from_scalar(1.0, s.ad_value(1708))), A::mul(s.ad_value(1692), s.ad_value(1708)));
        }

        if (s.v[1642] != 0.0) {
            let assign28760_ad_e26653: A = {
                if (!(p.p52 == 0.0)) {
                    A::scale(A::add(A::div(s.ad_value(1653), s.ad_value(1651)), A::mul(A::neg(A::div(s.ad_value(1653), s.ad_value(1651))), A::tanh(A::scale(A::neg(A::div(s.ad_value(1653), s.ad_value(1651))), (0.001 / p.p53))))), 0.5)
                } else {
                    {
                        if (p.p52 == 0.0) {
                            A::scale(A::add(A::div(s.ad_value(1653), s.ad_value(1651)), A::sqrt(A::offset(A::mul(A::neg(A::div(s.ad_value(1653), s.ad_value(1651))), A::neg(A::div(s.ad_value(1653), s.ad_value(1651)))), p.p53))), 0.5)
                        } else {
                            A::constant(0.0)
                        }
                    }
                }
            };
            s.store_div_from_scalar_ad(1717, 1.0, A::pow(A::offset(A::pow(assign28760_ad_e26653, s.ad_value(1675)), 1.0), A::div_from_scalar(1.0, s.ad_value(1675))));
        }

        if (s.v[1642] != 0.0) {
            s.store_mul(1718, 1653, 1717);
        }

        if (s.v[1642] != 0.0) {
            let assign28780_ad_e26734: A = {
                if (!(p.p52 == 0.0)) {
                    A::scale(A::add(A::div(A::neg(s.ad_value(1653)), s.ad_value(1651)), A::mul(A::neg(A::div(A::neg(s.ad_value(1653)), s.ad_value(1651))), A::tanh(A::scale(A::neg(A::div(A::neg(s.ad_value(1653)), s.ad_value(1651))), (0.001 / p.p53))))), 0.5)
                } else {
                    {
                        if (p.p52 == 0.0) {
                            A::scale(A::add(A::div(A::neg(s.ad_value(1653)), s.ad_value(1651)), A::sqrt(A::offset(A::mul(A::neg(A::div(A::neg(s.ad_value(1653)), s.ad_value(1651))), A::neg(A::div(A::neg(s.ad_value(1653)), s.ad_value(1651)))), p.p53))), 0.5)
                        } else {
                            A::constant(0.0)
                        }
                    }
                }
            };
            s.store_div_from_scalar_ad(1719, 1.0, A::pow(A::offset(A::pow(assign28780_ad_e26734, s.ad_value(1675)), 1.0), A::div_from_scalar(1.0, s.ad_value(1675))));
        }

        if (s.v[1642] != 0.0) {
            s.store_mul_ad_lhs(1720, A::neg(s.ad_value(1653)), 1719);
        }

        if (s.v[1642] != 0.0) {
            s.store_div_ad_lhs(1749, A::sub(s.ad_value(1652), s.ad_value(1750)), 1686);
        }

        s.v[1760] = if (s.v[1749] > 50.0) { 1.0 } else { 0.0 };

        if ((s.v[1642] != 0.0) && (s.v[1760] != 0.0)) {
            s.store_scalar(1691, 0.0);
        }

        s.v[1761] = if (s.v[1749] < (-50.0)) { 1.0 } else { 0.0 };

        if (((s.v[1642] != 0.0) && (!(s.v[1760] != 0.0))) && (s.v[1761] != 0.0)) {
            s.store_scalar(1691, 1.0);
        }

        if (((s.v[1642] != 0.0) && (!(s.v[1760] != 0.0))) && (!(s.v[1761] != 0.0))) {
            s.store_div_from_scalar_ad(1691, 1.0, A::offset(A::exp(s.ad_value(1749)), 1.0));
        }

        if (s.v[1642] != 0.0) {
            s.store_div_ad_lhs(1694, A::sub(A::sub(s.ad_value(1752), s.ad_value(1720)), A::sub(s.ad_value(1650), A::mul(A::scale(s.ad_value(1686), (p.p51 * 0.1)), s.ad_value(1691)))), 1692);
        }

        s.v[1762] = if (s.v[1694] > 50.0) { 1.0 } else { 0.0 };

        if ((s.v[1642] != 0.0) && (s.v[1762] != 0.0)) {
            s.store_mul(1695, 1693, 1694);
        }

        s.v[1763] = if (s.v[1694] < (-50.0)) { 1.0 } else { 0.0 };

        if (((s.v[1642] != 0.0) && (!(s.v[1762] != 0.0))) && (s.v[1763] != 0.0)) {
            s.store_mul_ad_rhs(1695, 1693, A::exp(s.ad_value(1694)));
        }

        if (((s.v[1642] != 0.0) && (!(s.v[1762] != 0.0))) && (!(s.v[1763] != 0.0))) {
            s.store_mul_ad_rhs(1695, 1693, A::ln(A::offset(A::exp(s.ad_value(1694)), 1.0)));
        }

        if (s.v[1642] != 0.0) {
            s.store_div_ad_lhs(1749, A::sub(s.ad_value(1752), s.ad_value(1750)), 1686);
        }

        s.v[1764] = if (s.v[1749] > 50.0) { 1.0 } else { 0.0 };

        if ((s.v[1642] != 0.0) && (s.v[1764] != 0.0)) {
            s.store_scalar(1721, 0.0);
        }

        s.v[1765] = if (s.v[1749] < (-50.0)) { 1.0 } else { 0.0 };

        if (((s.v[1642] != 0.0) && (!(s.v[1764] != 0.0))) && (s.v[1765] != 0.0)) {
            s.store_scalar(1721, 1.0);
        }

        if (((s.v[1642] != 0.0) && (!(s.v[1764] != 0.0))) && (!(s.v[1765] != 0.0))) {
            s.store_div_from_scalar_ad(1721, 1.0, A::offset(A::exp(s.ad_value(1749)), 1.0));
        }

        if (s.v[1642] != 0.0) {
            s.store_div_ad_lhs(1722, A::sub(A::sub(s.ad_value(1652), s.ad_value(1718)), A::sub(s.ad_value(1650), A::mul(A::scale(s.ad_value(1686), (p.p51 * 0.1)), s.ad_value(1721)))), 1692);
        }

        s.v[1766] = if (s.v[1722] > 50.0) { 1.0 } else { 0.0 };

        if ((s.v[1642] != 0.0) && (s.v[1766] != 0.0)) {
            s.store_mul(1723, 1693, 1722);
        }

        s.v[1767] = if (s.v[1722] < (-50.0)) { 1.0 } else { 0.0 };

        if (((s.v[1642] != 0.0) && (!(s.v[1766] != 0.0))) && (s.v[1767] != 0.0)) {
            s.store_mul_ad_rhs(1723, 1693, A::exp(s.ad_value(1722)));
        }

        if (((s.v[1642] != 0.0) && (!(s.v[1766] != 0.0))) && (!(s.v[1767] != 0.0))) {
            s.store_mul_ad_rhs(1723, 1693, A::ln(A::offset(A::exp(s.ad_value(1722)), 1.0)));
        }

        if (s.v[1642] != 0.0) {
            s.store_div_ad_lhs(1724, A::sub(s.ad_value(1695), s.ad_value(1723)), 1663);
        }

        if (s.v[1642] != 0.0) {
            s.store_div(1750, 1724, 1716);
        }

        if (s.v[1642] != 0.0) {
            let assign29060_ad_e27011: A = A::pow(A::offset(A::pow({
                if (!(p.p52 == 0.0)) {
                    A::mul(s.ad_value(1750), A::tanh(A::scale(s.ad_value(1750), (0.001 / p.p53))))
                } else {
                    {
                        if (p.p52 == 0.0) {
                            A::sqrt(A::offset(A::square(s.ad_value(1750)), p.p53))
                        } else {
                            A::constant(0.0)
                        }
                    }
                }
            }, s.ad_value(1675)), 1.0), A::div_from_scalar(1.0, s.ad_value(1675)));
            s.store_div_ad_rhs(1725, 1750, assign29060_ad_e27011);
        }

        if (s.v[1642] != 0.0) {
            s.store_mul(1726, 1698, 1725);
        }

        if (s.v[1642] != 0.0) {
            s.store_mul_ad_lhs(1644, A::mul(A::mul(A::scale(A::mul(A::mul(s.ad_value(1684), s.ad_value(1661)), s.ad_value(1683)), 0.5), A::add(s.ad_value(1695), s.ad_value(1723))), s.ad_value(1726)), 1685);
        }

        if (s.v[1642] != 0.0) {
            s.store_div_ad_rhs(1699, 1668, A::scale(s.ad_value(1660), 2.302585092994046));
        }

        if (s.v[1642] != 0.0) {
            s.store_mul_ad_lhs(1701, A::scale(s.ad_value(1699), 2.0), 1660);
        }

        s.v[1780] = if (s.v[1654] == 1.0) { 1.0 } else { 0.0 };

        if ((s.v[1642] != 0.0) && (s.v[1780] != 0.0)) {
            s.store_div_ad_lhs(1746, A::sub(s.ad_value(1655), A::sub(s.ad_value(1689), A::scale(s.ad_value(1686), (p.p51 * 0.5)))), 1701);
        }

        s.v[1781] = if (s.v[1746] > 50.0) { 1.0 } else { 0.0 };

        if (((s.v[1642] != 0.0) && (s.v[1780] != 0.0)) && (s.v[1781] != 0.0)) {
            s.copy_ad(1749, 1746);
        }

        s.v[1782] = if (s.v[1746] < (-50.0)) { 1.0 } else { 0.0 };

        if ((((s.v[1642] != 0.0) && (s.v[1780] != 0.0)) && (!(s.v[1781] != 0.0))) && (s.v[1782] != 0.0)) {
            s.store_exp(1749, 1746);
        }

        if ((((s.v[1642] != 0.0) && (s.v[1780] != 0.0)) && (!(s.v[1781] != 0.0))) && (!(s.v[1782] != 0.0))) {
            s.store_ln_ad(1749, A::offset(A::exp(s.ad_value(1746)), 1.0));
        }

        if ((s.v[1642] != 0.0) && (s.v[1780] != 0.0)) {
            s.store_div_ad_lhs(1747, A::sub(s.ad_value(1656), A::sub(s.ad_value(1689), A::scale(s.ad_value(1686), (p.p51 * 0.5)))), 1701);
        }

        s.v[1783] = if (s.v[1747] > 50.0) { 1.0 } else { 0.0 };

        if (((s.v[1642] != 0.0) && (s.v[1780] != 0.0)) && (s.v[1783] != 0.0)) {
            s.copy_ad(1749, 1747);
        }

        s.v[1784] = if (s.v[1747] < (-50.0)) { 1.0 } else { 0.0 };

        if ((((s.v[1642] != 0.0) && (s.v[1780] != 0.0)) && (!(s.v[1783] != 0.0))) && (s.v[1784] != 0.0)) {
            s.store_exp(1749, 1747);
        }

        if ((((s.v[1642] != 0.0) && (s.v[1780] != 0.0)) && (!(s.v[1783] != 0.0))) && (!(s.v[1784] != 0.0))) {
            s.store_ln_ad(1749, A::offset(A::exp(s.ad_value(1747)), 1.0));
        }

        s.v[1785] = if (s.v[1657] == 1.0) { 1.0 } else { 0.0 };

        if ((s.v[1642] != 0.0) && (s.v[1785] != 0.0)) {
            s.store_div_ad_lhs(1748, A::sub(s.ad_value(1652), A::sub(s.ad_value(1689), A::scale(s.ad_value(1686), (p.p51 * 0.5)))), 1701);
        }

        s.v[1786] = if (s.v[1748] > 50.0) { 1.0 } else { 0.0 };

        if (((s.v[1642] != 0.0) && (s.v[1785] != 0.0)) && (s.v[1786] != 0.0)) {
            s.copy_ad(1749, 1748);
        }

        s.v[1787] = if (s.v[1748] < (-50.0)) { 1.0 } else { 0.0 };

        if ((((s.v[1642] != 0.0) && (s.v[1785] != 0.0)) && (!(s.v[1786] != 0.0))) && (s.v[1787] != 0.0)) {
            s.store_exp(1749, 1748);
        }

        if ((((s.v[1642] != 0.0) && (s.v[1785] != 0.0)) && (!(s.v[1786] != 0.0))) && (!(s.v[1787] != 0.0))) {
            s.store_ln_ad(1749, A::offset(A::exp(s.ad_value(1748)), 1.0));
        }

        if (s.v[1642] != 0.0) {
            s.copy_ad(1643, 1644);
        }

        if (s.v[1642] != 0.0) {
            s.copy_ad(160, 1644);
        }

        if (s.v[1642] != 0.0) {
            s.copy_ad(160, 1643);
        }

        s.v[1788] = 0.0;

        s.v[1789] = 0.0;

        s.v[1790] = 0.0;

        s.v[1791] = 0.0;

        s.v[1795] = 0.0;

        s.v[1796] = 0.0;

        s.copy_ad(1797, 45);

        s.copy_ad(1798, 44);

        s.v[1799] = 0.0;

        s.v[1800] = 0.0;

        s.v[1801] = 0.0;

        s.v[1802] = 0.0;

        s.copy_ad(1803, 111);

        s.v[1804] = s.v[109];

        s.copy_ad(1805, 113);

        s.v[1806] = p.p0;

        s.v[1807] = p.p1;

        s.copy_ad(1808, 19);

        s.v[1812] = p.p35;

        s.v[1813] = p.p36;

        s.v[1814] = p.p37;

        s.v[1815] = p.p38;

        s.v[1816] = p.p40;

        s.v[1817] = p.p41;

        s.v[1818] = p.p32;

        s.v[1819] = p.p33;

        s.v[1820] = p.p34;

        s.v[1821] = p.p44;

        s.v[1822] = p.p43;

        s.v[1823] = p.p46;

        s.v[1824] = p.p39;

        s.v[1825] = p.p47;

        s.v[1826] = p.p45;

        s.v[1827] = p.p42;

        s.v[1828] = p.p2;

        s.v[1829] = p.p6;

        s.copy_ad(1830, 230);

        s.v[1831] = 0.0;

        s.v[1832] = 0.0;

        s.v[1833] = 0.0;

        s.v[1834] = 0.0;

        s.v[1835] = 0.0;

        s.v[1836] = 0.0;

        s.v[1837] = 0.0;

        s.v[1838] = 0.0;

        s.v[1839] = 0.0;

        s.v[1840] = 0.0;

        s.v[1841] = 0.0;

        s.v[1842] = 0.0;

        s.v[1843] = 0.0;

        s.v[1844] = 0.0;

        s.v[1845] = 0.0;

        s.v[1846] = 0.0;

        s.v[1847] = 0.0;

        s.v[1848] = 0.0;

        s.v[1849] = 0.0;

        s.v[1850] = 0.0;

        s.v[1851] = 0.0;

        s.v[1852] = 0.0;

        s.v[1853] = 0.0;

        s.v[1854] = 0.0;

        s.v[1855] = 0.0;

        s.v[1856] = 0.0;

        s.v[1857] = 0.0;

        s.v[1858] = 0.0;

        s.v[1859] = 0.0;

        s.v[1860] = 0.0;

        s.v[1861] = 0.0;

        s.v[1862] = 0.0;

        s.v[1863] = 0.0;

        s.v[1864] = 0.0;

        s.v[1865] = 0.0;

        s.v[1866] = 0.0;

        s.v[1867] = 0.0;

        s.v[1868] = 0.0;

        s.v[1869] = 0.0;

        s.v[1870] = 0.0;

        s.v[1871] = 0.0;

        s.v[1872] = 0.0;

        s.v[1873] = 0.0;

        s.v[1874] = 0.0;

        s.v[1875] = 0.0;

        s.v[1876] = 0.0;

        s.v[1877] = 0.0;

        s.v[1878] = 0.0;

        s.v[1879] = 0.0;

        s.v[1880] = 0.0;

        s.v[1881] = 0.0;

        s.v[1882] = 0.0;

        s.v[1883] = 0.0;

        s.v[1884] = 0.0;

        s.v[1885] = 0.0;

        s.v[1886] = 0.0;

        s.v[1887] = 0.0;

        s.v[1888] = 0.0;

        s.v[1889] = 0.0;

        s.v[1890] = 0.0;

        s.v[1891] = 0.0;

        s.v[1892] = 0.0;

        s.v[1893] = 0.0;

        s.v[1894] = 0.0;

        s.v[1895] = 0.0;

        s.v[1896] = 0.0;

        s.v[1897] = 0.0;

        s.v[1898] = 0.0;

        s.v[1899] = 0.0;

        if (!(p.p52 == 0.0)) {
            s.store_mul_ad_rhs(1896, 1798, A::tanh(A::scale(s.ad_value(1798), (0.001 / p.p53))));
        } else {
            if (p.p52 == 0.0) {
                s.store_sqrt_ad(1896, A::offset(A::square(s.ad_value(1798)), p.p53));
            } else {
                s.store_scalar(1896, 0.0);
            }
        }

        s.store_sub(1897, 1797, 1798);

        s.store_scale(1831, 1805, s.v[1817]);

        s.store_add_ad(1833, A::div_from_scalar(s.v[1813], A::scale(s.ad_value(1805), 2.302585092994046)), A::scale(s.ad_value(1896), s.v[1816]));

        s.store_offset_ad(1834, A::scale(A::offset(s.ad_value(1803), (-s.v[1804])), s.v[1823]), s.v[1812]);

        s.store_powf_ad(1852, A::scale(s.ad_value(1803), 1.0 / (s.v[1804])), s.v[1825]);

        s.v[1900] = if (s.v[1824] != 0.0) { 1.0 } else { 0.0 };

        if (s.v[1900] != 0.0) {
            s.store_div_ad_rhs(1835, 1896, A::powf(A::offset(A::powf(A::scale(s.ad_value(1896), 1.0 / (s.v[1824])), s.v[1820]), 1.0), (1.0 / s.v[1820])));
        }

        if (!(s.v[1900] != 0.0)) {
            s.store_scalar(1835, 0.0);
        }

        s.store_mul_ad_lhs(1832, A::sub_from_scalar(s.v[1814], A::scale(s.ad_value(1835), s.v[1815])), 1896);

        s.store_sub(1795, 1834, 1832);

        s.store_mul_ad_lhs(1837, A::scale(s.ad_value(1833), 2.0), 1805);

        s.store_mul(1838, 1808, 1837);

        s.store_sub_ad_rhs(1895, 1795, A::scale(s.ad_value(1831), (p.p51 * 0.5)));

    }

    pub(super) fn stamp_transient_block_23(
        &mut self,
        ctx: &GeneratedEvalContext<'_>,
        stamper: &mut GeneratedStamper<'_>,
        s: &mut Scratch,
    ) {
        let _ = stamper;
        let p = self.params;
        let nodes = self.nodes;
        let branches = self.branches;
        let assign31310_ad_e28372: A = {
    if (!(p.p52 == 0.0)) {
        A::scale(A::add(A::add(s.ad_value(1797), s.ad_value(1897)), A::mul(A::sub(s.ad_value(1797), s.ad_value(1897)), A::tanh(A::scale(A::sub(s.ad_value(1797), s.ad_value(1897)), (0.001 / p.p53))))), 0.5)
    } else {
        {
            if (p.p52 == 0.0) {
                A::scale(A::add(A::add(s.ad_value(1797), s.ad_value(1897)), A::sqrt(A::offset(A::mul(A::sub(s.ad_value(1797), s.ad_value(1897)), A::sub(s.ad_value(1797), s.ad_value(1897))), p.p53))), 0.5)
            } else {
                A::constant(0.0)
            }
        }
    }
};
        s.store_div_ad_lhs(1894, A::sub(assign31310_ad_e28372, s.ad_value(1895)), 1831);

        s.v[1901] = if (s.v[1894] > 50.0) { 1.0 } else { 0.0 };

        if (s.v[1901] != 0.0) {
            s.store_scalar(1853, 0.0);
        }

        s.v[1902] = if (s.v[1894] < (-50.0)) { 1.0 } else { 0.0 };

        if ((!(s.v[1901] != 0.0)) && (s.v[1902] != 0.0)) {
            s.store_scalar(1853, 1.0);
        }

        if ((!(s.v[1901] != 0.0)) && (!(s.v[1902] != 0.0))) {
            s.store_div_from_scalar_ad(1853, 1.0, A::offset(A::exp(s.ad_value(1894)), 1.0));
        }

        let assign31370_ad_e28451: A = {
    if (!(p.p52 == 0.0)) {
        A::scale(A::add(A::add(s.ad_value(1797), s.ad_value(1897)), A::mul(A::sub(s.ad_value(1797), s.ad_value(1897)), A::tanh(A::scale(A::sub(s.ad_value(1797), s.ad_value(1897)), (0.001 / p.p53))))), 0.5)
    } else {
        {
            if (p.p52 == 0.0) {
                A::scale(A::add(A::add(s.ad_value(1797), s.ad_value(1897)), A::sqrt(A::offset(A::mul(A::sub(s.ad_value(1797), s.ad_value(1897)), A::sub(s.ad_value(1797), s.ad_value(1897))), p.p53))), 0.5)
            } else {
                A::constant(0.0)
            }
        }
    }
};
        s.store_div_ad_lhs(1854, A::sub(assign31370_ad_e28451, A::sub(s.ad_value(1795), A::mul(A::scale(s.ad_value(1831), (p.p51 * 0.1)), s.ad_value(1853)))), 1837);

        s.v[1903] = if (s.v[1854] > 50.0) { 1.0 } else { 0.0 };

        if (s.v[1903] != 0.0) {
            s.store_mul(1855, 1838, 1854);
        }

        s.v[1904] = if (s.v[1854] < (-50.0)) { 1.0 } else { 0.0 };

        if ((!(s.v[1903] != 0.0)) && (s.v[1904] != 0.0)) {
            s.store_mul_ad_rhs(1855, 1838, A::exp(s.ad_value(1854)));
        }

        if ((!(s.v[1903] != 0.0)) && (!(s.v[1904] != 0.0))) {
            s.store_mul_ad_rhs(1855, 1838, A::ln(A::offset(A::exp(s.ad_value(1854)), 1.0)));
        }

        s.store_div_from_scalar_ad(1841, s.v[1819], A::mul(s.ad_value(1852), A::offset(A::div(A::scale(s.ad_value(1855), s.v[1821]), s.ad_value(1808)), 1.0)));

        s.store_div_ad(1842, A::mul(A::scale(A::div_from_scalar((1.0 + (s.v[1826] * s.v[1804])), A::offset(A::scale(s.ad_value(1803), s.v[1826]), 1.0)), s.v[1818]), A::offset(A::scale(s.ad_value(1896), (s.v[1827] * 1.0 / (s.v[1807]))), 1.0)), A::offset(A::div(A::scale(s.ad_value(1855), s.v[1822]), s.ad_value(1808)), 1.0));

        s.store_add_ad(1843, A::scale(A::mul(A::mul(A::scale(s.ad_value(1853), 2.0), s.ad_value(1805)), s.ad_value(1841)), 1.0 / (s.v[1807])), A::mul(A::sub_from_scalar(1.0, s.ad_value(1853)), s.ad_value(1842)));

        s.store_div_ad_lhs(1859, A::scale(s.ad_value(1842), s.v[1807]), 1841);

        s.store_sub_ad_lhs(1860, A::mul(s.ad_value(1859), A::sqrt(A::offset(A::div(A::div(A::scale(s.ad_value(1855), 2.0), s.ad_value(1808)), s.ad_value(1859)), 1.0))), 1859);

        s.store_add_ad(1861, A::mul(s.ad_value(1859), A::sub_from_scalar(1.0, s.ad_value(1853))), A::mul(s.ad_value(1837), s.ad_value(1853)));

        s.store_add_ad(1796, A::mul(s.ad_value(1860), A::sub_from_scalar(1.0, s.ad_value(1853))), A::mul(s.ad_value(1837), s.ad_value(1853)));

        let assign31500_ad_e28650: A = {
    if (!(p.p52 == 0.0)) {
        A::scale(A::add(A::div(s.ad_value(1798), s.ad_value(1796)), A::mul(A::neg(A::div(s.ad_value(1798), s.ad_value(1796))), A::tanh(A::scale(A::neg(A::div(s.ad_value(1798), s.ad_value(1796))), (0.001 / p.p53))))), 0.5)
    } else {
        {
            if (p.p52 == 0.0) {
                A::scale(A::add(A::div(s.ad_value(1798), s.ad_value(1796)), A::sqrt(A::offset(A::mul(A::neg(A::div(s.ad_value(1798), s.ad_value(1796))), A::neg(A::div(s.ad_value(1798), s.ad_value(1796)))), p.p53))), 0.5)
            } else {
                A::constant(0.0)
            }
        }
    }
};
        s.store_div_from_scalar_ad(1862, 1.0, A::powf(A::offset(A::powf(assign31500_ad_e28650, s.v[1820]), 1.0), (1.0 / s.v[1820])));

        s.store_mul(1863, 1798, 1862);

        let assign31520_ad_e28725: A = {
    if (!(p.p52 == 0.0)) {
        A::scale(A::add(A::div(A::neg(s.ad_value(1798)), s.ad_value(1796)), A::mul(A::neg(A::div(A::neg(s.ad_value(1798)), s.ad_value(1796))), A::tanh(A::scale(A::neg(A::div(A::neg(s.ad_value(1798)), s.ad_value(1796))), (0.001 / p.p53))))), 0.5)
    } else {
        {
            if (p.p52 == 0.0) {
                A::scale(A::add(A::div(A::neg(s.ad_value(1798)), s.ad_value(1796)), A::sqrt(A::offset(A::mul(A::neg(A::div(A::neg(s.ad_value(1798)), s.ad_value(1796))), A::neg(A::div(A::neg(s.ad_value(1798)), s.ad_value(1796)))), p.p53))), 0.5)
            } else {
                A::constant(0.0)
            }
        }
    }
};
        s.store_div_from_scalar_ad(1864, 1.0, A::powf(A::offset(A::powf(assign31520_ad_e28725, s.v[1820]), 1.0), (1.0 / s.v[1820])));

        s.store_mul_ad_lhs(1865, A::neg(s.ad_value(1798)), 1864);

        s.store_div_ad_lhs(1894, A::sub(s.ad_value(1797), s.ad_value(1895)), 1831);

        s.v[1905] = if (s.v[1894] > 50.0) { 1.0 } else { 0.0 };

        if (s.v[1905] != 0.0) {
            s.store_scalar(1836, 0.0);
        }

        s.v[1906] = if (s.v[1894] < (-50.0)) { 1.0 } else { 0.0 };

        if ((!(s.v[1905] != 0.0)) && (s.v[1906] != 0.0)) {
            s.store_scalar(1836, 1.0);
        }

        if ((!(s.v[1905] != 0.0)) && (!(s.v[1906] != 0.0))) {
            s.store_div_from_scalar_ad(1836, 1.0, A::offset(A::exp(s.ad_value(1894)), 1.0));
        }

        s.store_div_ad_lhs(1839, A::sub(A::sub(s.ad_value(1897), s.ad_value(1865)), A::sub(s.ad_value(1795), A::mul(A::scale(s.ad_value(1831), (p.p51 * 0.1)), s.ad_value(1836)))), 1837);

        s.v[1907] = if (s.v[1839] > 50.0) { 1.0 } else { 0.0 };

        if (s.v[1907] != 0.0) {
            s.store_mul(1840, 1838, 1839);
        }

        s.v[1908] = if (s.v[1839] < (-50.0)) { 1.0 } else { 0.0 };

        if ((!(s.v[1907] != 0.0)) && (s.v[1908] != 0.0)) {
            s.store_mul_ad_rhs(1840, 1838, A::exp(s.ad_value(1839)));
        }

        if ((!(s.v[1907] != 0.0)) && (!(s.v[1908] != 0.0))) {
            s.store_mul_ad_rhs(1840, 1838, A::ln(A::offset(A::exp(s.ad_value(1839)), 1.0)));
        }

        s.store_div_ad_lhs(1894, A::sub(s.ad_value(1897), s.ad_value(1895)), 1831);

        s.v[1909] = if (s.v[1894] > 50.0) { 1.0 } else { 0.0 };

        if (s.v[1909] != 0.0) {
            s.store_scalar(1866, 0.0);
        }

        s.v[1910] = if (s.v[1894] < (-50.0)) { 1.0 } else { 0.0 };

        if ((!(s.v[1909] != 0.0)) && (s.v[1910] != 0.0)) {
            s.store_scalar(1866, 1.0);
        }

        if ((!(s.v[1909] != 0.0)) && (!(s.v[1910] != 0.0))) {
            s.store_div_from_scalar_ad(1866, 1.0, A::offset(A::exp(s.ad_value(1894)), 1.0));
        }

        s.store_div_ad_lhs(1867, A::sub(A::sub(s.ad_value(1797), s.ad_value(1863)), A::sub(s.ad_value(1795), A::mul(A::scale(s.ad_value(1831), (p.p51 * 0.1)), s.ad_value(1866)))), 1837);

        s.v[1911] = if (s.v[1867] > 50.0) { 1.0 } else { 0.0 };

        if (s.v[1911] != 0.0) {
            s.store_mul(1868, 1838, 1867);
        }

        s.v[1912] = if (s.v[1867] < (-50.0)) { 1.0 } else { 0.0 };

        if ((!(s.v[1911] != 0.0)) && (s.v[1912] != 0.0)) {
            s.store_mul_ad_rhs(1868, 1838, A::exp(s.ad_value(1867)));
        }

        if ((!(s.v[1911] != 0.0)) && (!(s.v[1912] != 0.0))) {
            s.store_mul_ad_rhs(1868, 1838, A::ln(A::offset(A::exp(s.ad_value(1867)), 1.0)));
        }

        s.store_div_ad_lhs(1869, A::sub(s.ad_value(1840), s.ad_value(1868)), 1808);

        s.store_div(1895, 1869, 1861);

        let assign31800_ad_e28955: A = A::div(s.ad_value(1895), A::powf(A::offset(A::powf({
    if (!(p.p52 == 0.0)) {
        A::mul(s.ad_value(1895), A::tanh(A::scale(s.ad_value(1895), (0.001 / p.p53))))
    } else {
        {
            if (p.p52 == 0.0) {
                A::sqrt(A::offset(A::square(s.ad_value(1895)), p.p53))
            } else {
                A::constant(0.0)
            }
        }
    }
}, s.v[1820]), 1.0), (1.0 / s.v[1820])));
        s.store_ad(1870, &assign31800_ad_e28955);

        s.store_mul(1871, 1843, 1870);

        s.store_mul_ad_lhs(1789, A::mul(A::scale(A::add(s.ad_value(1840), s.ad_value(1868)), (((s.v[1829] * s.v[1806]) * s.v[1828]) * 0.5)), s.ad_value(1871)), 1830);

        s.store_div_from_scalar_ad(1844, s.v[1813], A::scale(s.ad_value(1805), 2.302585092994046));

        s.store_mul_ad_lhs(1846, A::scale(s.ad_value(1844), 2.0), 1805);

        s.store_mul(1847, 1808, 1846);

        s.store_sub_ad_rhs(1899, 1834, A::scale(s.ad_value(1831), (p.p51 * 0.5)));

        let assign31870_ad_e29037: A = {
    if (!(p.p52 == 0.0)) {
        A::scale(A::add(A::add(s.ad_value(1797), s.ad_value(1897)), A::mul(A::sub(s.ad_value(1797), s.ad_value(1897)), A::tanh(A::scale(A::sub(s.ad_value(1797), s.ad_value(1897)), (0.001 / p.p53))))), 0.5)
    } else {
        {
            if (p.p52 == 0.0) {
                A::scale(A::add(A::add(s.ad_value(1797), s.ad_value(1897)), A::sqrt(A::offset(A::mul(A::sub(s.ad_value(1797), s.ad_value(1897)), A::sub(s.ad_value(1797), s.ad_value(1897))), p.p53))), 0.5)
            } else {
                A::constant(0.0)
            }
        }
    }
};
        s.store_div_ad_lhs(1898, A::sub(assign31870_ad_e29037, s.ad_value(1899)), 1831);

        s.v[1913] = if (s.v[1898] > 50.0) { 1.0 } else { 0.0 };

        if (s.v[1913] != 0.0) {
            s.store_scalar(1856, 0.0);
        }

        s.v[1914] = if (s.v[1898] < (-50.0)) { 1.0 } else { 0.0 };

        if ((!(s.v[1913] != 0.0)) && (s.v[1914] != 0.0)) {
            s.store_scalar(1856, 1.0);
        }

        if ((!(s.v[1913] != 0.0)) && (!(s.v[1914] != 0.0))) {
            s.store_div_from_scalar_ad(1856, 1.0, A::offset(A::exp(s.ad_value(1898)), 1.0));
        }

        let assign31930_ad_e29116: A = {
    if (!(p.p52 == 0.0)) {
        A::scale(A::add(A::add(s.ad_value(1797), s.ad_value(1897)), A::mul(A::sub(s.ad_value(1797), s.ad_value(1897)), A::tanh(A::scale(A::sub(s.ad_value(1797), s.ad_value(1897)), (0.001 / p.p53))))), 0.5)
    } else {
        {
            if (p.p52 == 0.0) {
                A::scale(A::add(A::add(s.ad_value(1797), s.ad_value(1897)), A::sqrt(A::offset(A::mul(A::sub(s.ad_value(1797), s.ad_value(1897)), A::sub(s.ad_value(1797), s.ad_value(1897))), p.p53))), 0.5)
            } else {
                A::constant(0.0)
            }
        }
    }
};
        s.store_div_ad_lhs(1857, A::sub(assign31930_ad_e29116, A::sub(s.ad_value(1834), A::mul(A::scale(s.ad_value(1831), (p.p51 * 0.1)), s.ad_value(1856)))), 1846);

        s.v[1915] = if (s.v[1857] > 50.0) { 1.0 } else { 0.0 };

        if (s.v[1915] != 0.0) {
            s.store_mul(1858, 1847, 1857);
        }

        s.v[1916] = if (s.v[1857] < (-50.0)) { 1.0 } else { 0.0 };

        if ((!(s.v[1915] != 0.0)) && (s.v[1916] != 0.0)) {
            s.store_mul_ad_rhs(1858, 1847, A::exp(s.ad_value(1857)));
        }

        if ((!(s.v[1915] != 0.0)) && (!(s.v[1916] != 0.0))) {
            s.store_mul_ad_rhs(1858, 1847, A::ln(A::offset(A::exp(s.ad_value(1857)), 1.0)));
        }

        s.store_div_from_scalar(1850, s.v[1819], 1852);

        s.store_scale_ad(1851, A::div_from_scalar((1.0 + (s.v[1826] * s.v[1804])), A::offset(A::scale(s.ad_value(1803), s.v[1826]), 1.0)), s.v[1818]);

        s.store_div_ad_lhs(1872, A::scale(s.ad_value(1851), s.v[1807]), 1850);

        s.store_sub_ad_lhs(1873, A::mul(s.ad_value(1872), A::sqrt(A::offset(A::div(A::div(A::scale(s.ad_value(1858), 2.0), s.ad_value(1808)), s.ad_value(1872)), 1.0))), 1872);

        s.store_add_ad(1874, A::mul(s.ad_value(1873), A::sub_from_scalar(1.0, s.ad_value(1856))), A::mul(s.ad_value(1846), s.ad_value(1856)));

        let assign32040_ad_e29267: A = {
    if (!(p.p52 == 0.0)) {
        A::scale(A::add(A::div(s.ad_value(1798), s.ad_value(1874)), A::mul(A::neg(A::div(s.ad_value(1798), s.ad_value(1874))), A::tanh(A::scale(A::neg(A::div(s.ad_value(1798), s.ad_value(1874))), (0.001 / p.p53))))), 0.5)
    } else {
        {
            if (p.p52 == 0.0) {
                A::scale(A::add(A::div(s.ad_value(1798), s.ad_value(1874)), A::sqrt(A::offset(A::mul(A::neg(A::div(s.ad_value(1798), s.ad_value(1874))), A::neg(A::div(s.ad_value(1798), s.ad_value(1874)))), p.p53))), 0.5)
            } else {
                A::constant(0.0)
            }
        }
    }
};
        s.store_div_from_scalar_ad(1875, 1.0, A::powf(A::offset(A::powf(assign32040_ad_e29267, s.v[1820]), 1.0), (1.0 / s.v[1820])));

        s.store_mul(1876, 1798, 1875);

        let assign32060_ad_e29342: A = {
    if (!(p.p52 == 0.0)) {
        A::scale(A::add(A::div(A::neg(s.ad_value(1798)), s.ad_value(1874)), A::mul(A::neg(A::div(A::neg(s.ad_value(1798)), s.ad_value(1874))), A::tanh(A::scale(A::neg(A::div(A::neg(s.ad_value(1798)), s.ad_value(1874))), (0.001 / p.p53))))), 0.5)
    } else {
        {
            if (p.p52 == 0.0) {
                A::scale(A::add(A::div(A::neg(s.ad_value(1798)), s.ad_value(1874)), A::sqrt(A::offset(A::mul(A::neg(A::div(A::neg(s.ad_value(1798)), s.ad_value(1874))), A::neg(A::div(A::neg(s.ad_value(1798)), s.ad_value(1874)))), p.p53))), 0.5)
            } else {
                A::constant(0.0)
            }
        }
    }
};
        s.store_div_from_scalar_ad(1877, 1.0, A::powf(A::offset(A::powf(assign32060_ad_e29342, s.v[1820]), 1.0), (1.0 / s.v[1820])));

        s.store_mul_ad_lhs(1878, A::neg(s.ad_value(1798)), 1877);

        s.store_div_ad_lhs(1898, A::sub(s.ad_value(1797), s.ad_value(1899)), 1831);

        s.v[1917] = if (s.v[1898] > 50.0) { 1.0 } else { 0.0 };

        if (s.v[1917] != 0.0) {
            s.store_scalar(1845, 0.0);
        }

        s.v[1918] = if (s.v[1898] < (-50.0)) { 1.0 } else { 0.0 };

        if ((!(s.v[1917] != 0.0)) && (s.v[1918] != 0.0)) {
            s.store_scalar(1845, 1.0);
        }

        if ((!(s.v[1917] != 0.0)) && (!(s.v[1918] != 0.0))) {
            s.store_div_from_scalar_ad(1845, 1.0, A::offset(A::exp(s.ad_value(1898)), 1.0));
        }

        s.store_div_ad_lhs(1848, A::sub(A::sub(s.ad_value(1897), s.ad_value(1878)), A::sub(s.ad_value(1834), A::mul(A::scale(s.ad_value(1831), (p.p51 * 0.1)), s.ad_value(1845)))), 1846);

        s.v[1919] = if (s.v[1848] > 50.0) { 1.0 } else { 0.0 };

        if (s.v[1919] != 0.0) {
            s.store_mul(1849, 1847, 1848);
        }

        s.v[1920] = if (s.v[1848] < (-50.0)) { 1.0 } else { 0.0 };

        if ((!(s.v[1919] != 0.0)) && (s.v[1920] != 0.0)) {
            s.store_mul_ad_rhs(1849, 1847, A::exp(s.ad_value(1848)));
        }

        if ((!(s.v[1919] != 0.0)) && (!(s.v[1920] != 0.0))) {
            s.store_mul_ad_rhs(1849, 1847, A::ln(A::offset(A::exp(s.ad_value(1848)), 1.0)));
        }

        s.store_div_ad_lhs(1898, A::sub(s.ad_value(1897), s.ad_value(1899)), 1831);

        s.v[1921] = if (s.v[1898] > 50.0) { 1.0 } else { 0.0 };

        if (s.v[1921] != 0.0) {
            s.store_scalar(1879, 0.0);
        }

        s.v[1922] = if (s.v[1898] < (-50.0)) { 1.0 } else { 0.0 };

        if ((!(s.v[1921] != 0.0)) && (s.v[1922] != 0.0)) {
            s.store_scalar(1879, 1.0);
        }

        if ((!(s.v[1921] != 0.0)) && (!(s.v[1922] != 0.0))) {
            s.store_div_from_scalar_ad(1879, 1.0, A::offset(A::exp(s.ad_value(1898)), 1.0));
        }

        s.store_div_ad_lhs(1880, A::sub(A::sub(s.ad_value(1797), s.ad_value(1876)), A::sub(s.ad_value(1834), A::mul(A::scale(s.ad_value(1831), (p.p51 * 0.1)), s.ad_value(1879)))), 1846);

        s.v[1923] = if (s.v[1880] > 50.0) { 1.0 } else { 0.0 };

        if (s.v[1923] != 0.0) {
            s.store_mul(1881, 1847, 1880);
        }

        s.v[1924] = if (s.v[1880] < (-50.0)) { 1.0 } else { 0.0 };

        if ((!(s.v[1923] != 0.0)) && (s.v[1924] != 0.0)) {
            s.store_mul_ad_rhs(1881, 1847, A::exp(s.ad_value(1880)));
        }

        if ((!(s.v[1923] != 0.0)) && (!(s.v[1924] != 0.0))) {
            s.store_mul_ad_rhs(1881, 1847, A::ln(A::offset(A::exp(s.ad_value(1880)), 1.0)));
        }

        s.store_offset_ad(1882, A::square(s.ad_value(1849)), 1e-38);

        s.store_offset_ad(1883, A::mul(s.ad_value(1882), s.ad_value(1849)), 1e-57);

        s.store_offset_ad(1884, A::square(s.ad_value(1881)), 1e-38);

        s.store_offset_ad(1885, A::mul(s.ad_value(1884), s.ad_value(1881)), 1e-57);

        s.store_offset_ad(1886, A::mul(s.ad_value(1849), s.ad_value(1881)), 1e-38);

        s.store_div_ad(1887, A::scale(A::add(A::add(s.ad_value(1882), s.ad_value(1884)), s.ad_value(1886)), (2.0 / 3.0)), A::offset(A::add(s.ad_value(1849), s.ad_value(1881)), 2e-19));

        s.store_div_ad(1888, A::scale(A::add(A::add(A::add(A::scale(s.ad_value(1883), 2.0), A::scale(s.ad_value(1885), 3.0)), A::mul(A::scale(s.ad_value(1882), 4.0), s.ad_value(1881))), A::mul(A::scale(s.ad_value(1884), 6.0), s.ad_value(1849))), 2.0), A::scale(A::add(A::add(s.ad_value(1882), s.ad_value(1884)), A::scale(s.ad_value(1886), 2.0)), 15.0));

        s.store_sub(1889, 1887, 1888);

        s.copy_ad(1890, 1888);

        s.store_mul_ad_lhs(1790, A::scale(s.ad_value(1889), (((s.v[1806] * s.v[1828]) * s.v[1807]) * s.v[1829])), 1830);

        s.store_mul_ad_lhs(1791, A::scale(s.ad_value(1890), (((s.v[1806] * s.v[1828]) * s.v[1807]) * s.v[1829])), 1830);

        s.v[1925] = if (s.v[1799] == 1.0) { 1.0 } else { 0.0 };

        if (s.v[1925] != 0.0) {
            s.store_div_ad_lhs(1891, A::sub_from_scalar(s.v[1800], A::sub(s.ad_value(1834), A::scale(s.ad_value(1831), (p.p51 * 0.5)))), 1846);
        }

        s.v[1926] = if (s.v[1891] > 50.0) { 1.0 } else { 0.0 };

        if ((s.v[1925] != 0.0) && (s.v[1926] != 0.0)) {
            s.copy_ad(1894, 1891);
        }

        s.v[1927] = if (s.v[1891] < (-50.0)) { 1.0 } else { 0.0 };

        if (((s.v[1925] != 0.0) && (!(s.v[1926] != 0.0))) && (s.v[1927] != 0.0)) {
            s.store_exp(1894, 1891);
        }

        if (((s.v[1925] != 0.0) && (!(s.v[1926] != 0.0))) && (!(s.v[1927] != 0.0))) {
            s.store_ln_ad(1894, A::offset(A::exp(s.ad_value(1891)), 1.0));
        }

        if (s.v[1925] != 0.0) {
            s.store_div_ad_lhs(1892, A::sub_from_scalar(s.v[1801], A::sub(s.ad_value(1834), A::scale(s.ad_value(1831), (p.p51 * 0.5)))), 1846);
        }

        s.v[1928] = if (s.v[1892] > 50.0) { 1.0 } else { 0.0 };

        if ((s.v[1925] != 0.0) && (s.v[1928] != 0.0)) {
            s.copy_ad(1894, 1892);
        }

        s.v[1929] = if (s.v[1892] < (-50.0)) { 1.0 } else { 0.0 };

        if (((s.v[1925] != 0.0) && (!(s.v[1928] != 0.0))) && (s.v[1929] != 0.0)) {
            s.store_exp(1894, 1892);
        }

        if (((s.v[1925] != 0.0) && (!(s.v[1928] != 0.0))) && (!(s.v[1929] != 0.0))) {
            s.store_ln_ad(1894, A::offset(A::exp(s.ad_value(1892)), 1.0));
        }

        s.v[1930] = if (s.v[1802] == 1.0) { 1.0 } else { 0.0 };

        if (s.v[1930] != 0.0) {
            s.store_div_ad_lhs(1893, A::sub(s.ad_value(1797), A::sub(s.ad_value(1834), A::scale(s.ad_value(1831), (p.p51 * 0.5)))), 1846);
        }

        s.v[1931] = if (s.v[1893] > 50.0) { 1.0 } else { 0.0 };

        if ((s.v[1930] != 0.0) && (s.v[1931] != 0.0)) {
            s.copy_ad(1894, 1893);
        }

        s.v[1932] = if (s.v[1893] < (-50.0)) { 1.0 } else { 0.0 };

        if (((s.v[1930] != 0.0) && (!(s.v[1931] != 0.0))) && (s.v[1932] != 0.0)) {
            s.store_exp(1894, 1893);
        }

        if (((s.v[1930] != 0.0) && (!(s.v[1931] != 0.0))) && (!(s.v[1932] != 0.0))) {
            s.store_ln_ad(1894, A::offset(A::exp(s.ad_value(1893)), 1.0));
        }

        s.copy_ad(1788, 1789);

        s.copy_ad(115, 1789);

        s.copy_ad(117, 1790);

        s.copy_ad(118, 1791);

        s.copy_ad(115, 1788);

        s.store_ad(116, &A::voltage(ctx, &nodes, Some(29), None));

        s.v[1933] = if (p.p322 == 0.0) { 1.0 } else { 0.0 };

        s.v[122] = 0.0;

        s.v[123] = 0.0;

        s.v[124] = 0.0;

        s.v[125] = 0.0;

        s.v[126] = 0.0;

        s.v[127] = 0.0;

        s.v[134] = 0.0;

        s.v[135] = 0.0;

        s.v[128] = 0.0;

        s.v[129] = 0.0;

    }

    pub(super) fn stamp_transient_block_24(
        &mut self,
        ctx: &GeneratedEvalContext<'_>,
        stamper: &mut GeneratedStamper<'_>,
        s: &mut Scratch,
    ) {
        let _ = stamper;
        let p = self.params;
        let nodes = self.nodes;
        let branches = self.branches;
        s.v[130] = 0.0;

        s.v[131] = 0.0;

        s.v[132] = 0.0;

        s.v[133] = 0.0;

        s.v[140] = 0.0;

        s.v[141] = 0.0;

        s.v[1934] = if (p.p254 == 1.0) { 1.0 } else { 0.0 };

        if (s.v[1934] != 0.0) {
            s.store_scalar(1935, 0.0);
        }

        if (s.v[1934] != 0.0) {
            s.store_scalar(1936, 0.0);
        }

        if (s.v[1934] != 0.0) {
            s.store_scalar(1937, 0.0);
        }

        if (s.v[1934] != 0.0) {
            s.store_ad(1938, &A::scale(A::voltage(ctx, &nodes, Some(8), Some(13)), p.p6));
        }

        if (s.v[1934] != 0.0) {
            s.copy_ad(1939, 113);
        }

        if (s.v[1934] != 0.0) {
            s.store_scalar(1940, p.p260);
        }

        if (s.v[1934] != 0.0) {
            s.store_scalar(1941, p.p262);
        }

        if (s.v[1934] != 0.0) {
            s.store_scalar(1942, p.p261);
        }

        if (s.v[1934] != 0.0) {
            s.store_scalar(1943, p.p258);
        }

        if (s.v[1934] != 0.0) {
            s.store_scalar(1944, p.p278);
        }

        if (s.v[1934] != 0.0) {
            s.store_scalar(1945, p.p277);
        }

        if (s.v[1934] != 0.0) {
            s.copy_ad(1946, 112);
        }

        if (s.v[1934] != 0.0) {
            s.store_scalar(1947, p.p0);
        }

        if (s.v[1934] != 0.0) {
            s.store_scalar(1948, p.p2);
        }

        if (s.v[1934] != 0.0) {
            s.store_scalar(1949, ((1.0 - p.p255) * p.p259));
        }

        if (s.v[1934] != 0.0) {
            s.store_scalar(1950, p.p276);
        }

        if (s.v[1934] != 0.0) {
            s.store_scalar(1951, p.p270);
        }

        if (s.v[1934] != 0.0) {
            s.store_scalar(1952, p.p271);
        }

        if (s.v[1934] != 0.0) {
            s.store_scalar(1953, ((1.0 - p.p255) * p.p269));
        }

        if (s.v[1934] != 0.0) {
            s.store_scalar(1954, p.p268);
        }

        if (s.v[1934] != 0.0) {
            s.store_scalar(1955, p.p257);
        }

        if (s.v[1934] != 0.0) {
            s.store_scalar(1956, p.p256);
        }

        if (s.v[1934] != 0.0) {
            s.store_scalar(1957, p.p6);
        }

        if (s.v[1934] != 0.0) {
            s.store_scalar(1958, 0.0);
        }

        if (s.v[1934] != 0.0) {
            s.store_scalar(1959, 0.0);
        }

        if (s.v[1934] != 0.0) {
            s.store_scalar(1960, 0.0);
        }

        if (s.v[1934] != 0.0) {
            s.store_scalar(1961, 0.0);
        }

        if (s.v[1934] != 0.0) {
            s.store_scalar(1962, 0.0);
        }

        if (s.v[1934] != 0.0) {
            s.store_scalar(1963, 0.0);
        }

        if (s.v[1934] != 0.0) {
            s.store_scalar(1964, 0.0);
        }

        if (s.v[1934] != 0.0) {
            s.store_scalar(1965, 0.0);
        }

        if (s.v[1934] != 0.0) {
            s.store_scalar(1966, 0.0);
        }

        if (s.v[1934] != 0.0) {
            s.store_scalar(1967, 0.0);
        }

        if (s.v[1934] != 0.0) {
            s.store_scalar(1968, 0.0);
        }

        if (s.v[1934] != 0.0) {
            s.store_scalar(1969, 0.0);
        }

        if (s.v[1934] != 0.0) {
            s.store_scalar(1970, 0.0);
        }

        if (s.v[1934] != 0.0) {
            s.store_scalar(1971, 0.0);
        }

        if (s.v[1934] != 0.0) {
            s.store_scalar(1972, 0.0);
        }

        if (s.v[1934] != 0.0) {
            s.store_scalar(1973, 0.0);
        }

        if (s.v[1934] != 0.0) {
            s.store_scalar(1974, 0.0);
        }

        if (s.v[1934] != 0.0) {
            s.store_scalar(1975, 0.0);
        }

        if (s.v[1934] != 0.0) {
            s.store_scalar(1976, 0.0);
        }

        if (s.v[1934] != 0.0) {
            s.store_scalar(1977, 0.0);
        }

        if (s.v[1934] != 0.0) {
            s.store_scalar(1978, 0.0);
        }

        if (s.v[1934] != 0.0) {
            s.store_scalar(1979, 0.0);
        }

        if (s.v[1934] != 0.0) {
            s.store_scalar(1980, 0.0);
        }

        if (s.v[1934] != 0.0) {
            s.store_scalar(1981, 0.0);
        }

        if (s.v[1934] != 0.0) {
            s.store_scalar(1982, 0.0);
        }

        if (s.v[1934] != 0.0) {
            s.store_scalar(1983, 0.0);
        }

        if (s.v[1934] != 0.0) {
            s.store_scalar(1984, 0.0);
        }

        if (s.v[1934] != 0.0) {
            s.store_scalar(1985, 0.0);
        }

        if (s.v[1934] != 0.0) {
            s.store_scalar(1986, 0.0);
        }

        if (s.v[1934] != 0.0) {
            s.store_scalar(1987, 0.0);
        }

        if (s.v[1934] != 0.0) {
            s.store_scalar(1988, 0.0);
        }

        if (s.v[1934] != 0.0) {
            s.store_scalar(1989, 0.0);
        }

        if (s.v[1934] != 0.0) {
            s.store_scalar(1990, 0.0);
        }

        if (s.v[1934] != 0.0) {
            s.store_mul_ad(1970, A::div(s.ad_value(1955), s.ad_value(1939)), A::neg(s.ad_value(1956)));
        }

        if (s.v[1934] != 0.0) {
            let assign33630_ad_e30173: A = {
                if ((!(s.v[1970] > 50.0)) && (!(s.v[1970] < (-50.0)))) {
                    A::exp(s.ad_value(1970))
                } else {
                    {
                        if ((!(s.v[1970] > 50.0)) && (s.v[1970] < (-50.0))) {
                            A::exp(A::neg(A::constant(50.0)))
                        } else {
                            {
                                if (s.v[1970] > 50.0) {
                                    A::scale(A::offset(A::offset(s.ad_value(1970), (-50.0)), 1.0), ((50.0) as f64).exp())
                                } else {
                                    A::constant(0.0)
                                }
                            }
                        }
                    }
                }
            };
            s.store_ad(1960, &assign33630_ad_e30173);
        }

        if (s.v[1934] != 0.0) {
            s.store_add_ad_lhs(1966, A::mul(s.ad_value(1944), A::sub(A::neg(s.ad_value(1938)), s.ad_value(1945))), 1970);
        }

        if (s.v[1934] != 0.0) {
            s.store_add_ad_lhs(1967, A::mul(A::neg(s.ad_value(1944)), s.ad_value(1945)), 1970);
        }

        if (s.v[1934] != 0.0) {
            let assign33660_ad_e30235: A = {
                if ((!(s.v[1966] > 50.0)) && (!(s.v[1966] < (-50.0)))) {
                    A::exp(s.ad_value(1966))
                } else {
                    {
                        if ((!(s.v[1966] > 50.0)) && (s.v[1966] < (-50.0))) {
                            A::exp(A::neg(A::constant(50.0)))
                        } else {
                            {
                                if (s.v[1966] > 50.0) {
                                    A::scale(A::offset(A::offset(s.ad_value(1966), (-50.0)), 1.0), ((50.0) as f64).exp())
                                } else {
                                    A::constant(0.0)
                                }
                            }
                        }
                    }
                }
            };
            s.store_ad(1968, &assign33660_ad_e30235);
        }

        if (s.v[1934] != 0.0) {
            let assign33670_ad_e30277: A = {
                if ((!(s.v[1967] > 50.0)) && (!(s.v[1967] < (-50.0)))) {
                    A::exp(s.ad_value(1967))
                } else {
                    {
                        if ((!(s.v[1967] > 50.0)) && (s.v[1967] < (-50.0))) {
                            A::exp(A::neg(A::constant(50.0)))
                        } else {
                            {
                                if (s.v[1967] > 50.0) {
                                    A::scale(A::offset(A::offset(s.ad_value(1967), (-50.0)), 1.0), ((50.0) as f64).exp())
                                } else {
                                    A::constant(0.0)
                                }
                            }
                        }
                    }
                }
            };
            s.store_ad(1969, &assign33670_ad_e30277);
        }

        if (s.v[1934] != 0.0) {
            s.store_sub(1962, 1968, 1969);
        }

        if (s.v[1934] != 0.0) {
            s.store_mul_ad_lhs(1936, A::mul(A::mul(A::mul(s.ad_value(1957), s.ad_value(1947)), s.ad_value(1948)), s.ad_value(1949)), 1946);
        }

        if (s.v[1934] != 0.0) {
            s.store_add_ad_lhs(1972, A::mul(A::div(s.ad_value(1943), s.ad_value(1939)), s.ad_value(1938)), 1970);
        }

        if (s.v[1934] != 0.0) {
            let assign33710_ad_e30347: A = {
                if ((!(s.v[1972] > 50.0)) && (!(s.v[1972] < (-50.0)))) {
                    A::exp(s.ad_value(1972))
                } else {
                    {
                        if ((!(s.v[1972] > 50.0)) && (s.v[1972] < (-50.0))) {
                            A::exp(A::neg(A::constant(50.0)))
                        } else {
                            {
                                if (s.v[1972] > 50.0) {
                                    A::scale(A::offset(A::offset(s.ad_value(1972), (-50.0)), 1.0), ((50.0) as f64).exp())
                                } else {
                                    A::constant(0.0)
                                }
                            }
                        }
                    }
                }
            };
            s.store_ad(1973, &assign33710_ad_e30347);
        }

        s.v[1991] = if (s.v[1942] == 1.0) { 1.0 } else { 0.0 };

        if ((s.v[1934] != 0.0) && (s.v[1991] != 0.0)) {
            s.store_mul_ad_rhs(1963, 1936, A::sub(A::sub(s.ad_value(1973), A::mul(s.ad_value(1950), s.ad_value(1962))), s.ad_value(1960)));
        }

        if ((s.v[1934] != 0.0) && (!(s.v[1991] != 0.0))) {
            s.store_add_ad_lhs(1977, A::mul(s.ad_value(1944), A::sub(A::neg(s.ad_value(1940)), s.ad_value(1945))), 1970);
        }

        if ((s.v[1934] != 0.0) && (!(s.v[1991] != 0.0))) {
            let assign33750_ad_e30423: A = {
                if ((!(s.v[1977] > 50.0)) && (!(s.v[1977] < (-50.0)))) {
                    A::exp(s.ad_value(1977))
                } else {
                    {
                        if ((!(s.v[1977] > 50.0)) && (s.v[1977] < (-50.0))) {
                            A::exp(A::neg(A::constant(50.0)))
                        } else {
                            {
                                if (s.v[1977] > 50.0) {
                                    A::scale(A::offset(A::offset(s.ad_value(1977), (-50.0)), 1.0), ((50.0) as f64).exp())
                                } else {
                                    A::constant(0.0)
                                }
                            }
                        }
                    }
                }
            };
            s.store_ad(1978, &assign33750_ad_e30423);
        }

        if ((s.v[1934] != 0.0) && (!(s.v[1991] != 0.0))) {
            s.store_sub(1979, 1978, 1969);
        }

        if ((s.v[1934] != 0.0) && (!(s.v[1991] != 0.0))) {
            s.store_add_ad_lhs(1980, A::mul(A::div(s.ad_value(1943), s.ad_value(1939)), s.ad_value(1940)), 1970);
        }

        if ((s.v[1934] != 0.0) && (!(s.v[1991] != 0.0))) {
            let assign33780_ad_e30490: A = {
                if ((!(s.v[1980] > 50.0)) && (!(s.v[1980] < (-50.0)))) {
                    A::exp(s.ad_value(1980))
                } else {
                    {
                        if ((!(s.v[1980] > 50.0)) && (s.v[1980] < (-50.0))) {
                            A::exp(A::neg(A::constant(50.0)))
                        } else {
                            {
                                if (s.v[1980] > 50.0) {
                                    A::scale(A::offset(A::offset(s.ad_value(1980), (-50.0)), 1.0), ((50.0) as f64).exp())
                                } else {
                                    A::constant(0.0)
                                }
                            }
                        }
                    }
                }
            };
            s.store_ad(1981, &assign33780_ad_e30490);
        }

        if ((s.v[1934] != 0.0) && (!(s.v[1991] != 0.0))) {
            s.store_sub_ad_lhs(1982, A::sub(s.ad_value(1981), A::mul(s.ad_value(1950), s.ad_value(1979))), 1960);
        }

        if ((s.v[1934] != 0.0) && (!(s.v[1991] != 0.0))) {
            s.store_mul_ad_rhs(1983, 1936, A::sub(A::sub(s.ad_value(1973), A::mul(s.ad_value(1950), s.ad_value(1962))), s.ad_value(1960)));
        }

        s.v[1992] = if (s.v[1942] > 0.0) { 1.0 } else { 0.0 };

        if (((s.v[1934] != 0.0) && (!(s.v[1991] != 0.0))) && (s.v[1992] != 0.0)) {
            s.store_mul(1976, 1942, 1943);
        }

        if (((s.v[1934] != 0.0) && (!(s.v[1991] != 0.0))) && (s.v[1992] != 0.0)) {
            s.store_add_ad_lhs(1984, A::mul(A::div(s.ad_value(1976), s.ad_value(1939)), s.ad_value(1940)), 1970);
        }

        if (((s.v[1934] != 0.0) && (!(s.v[1991] != 0.0))) && (s.v[1992] != 0.0)) {
            let assign33840_ad_e30594: A = {
                if ((!(s.v[1984] > 50.0)) && (!(s.v[1984] < (-50.0)))) {
                    A::exp(s.ad_value(1984))
                } else {
                    {
                        if ((!(s.v[1984] > 50.0)) && (s.v[1984] < (-50.0))) {
                            A::exp(A::neg(A::constant(50.0)))
                        } else {
                            {
                                if (s.v[1984] > 50.0) {
                                    A::scale(A::offset(A::offset(s.ad_value(1984), (-50.0)), 1.0), ((50.0) as f64).exp())
                                } else {
                                    A::constant(0.0)
                                }
                            }
                        }
                    }
                }
            };
            s.store_ad(1985, &assign33840_ad_e30594);
        }

        if (((s.v[1934] != 0.0) && (!(s.v[1991] != 0.0))) && (s.v[1992] != 0.0)) {
            s.store_sub_ad_lhs(1986, A::sub(s.ad_value(1985), A::mul(s.ad_value(1950), s.ad_value(1979))), 1960);
        }

        if (((s.v[1934] != 0.0) && (!(s.v[1991] != 0.0))) && (s.v[1992] != 0.0)) {
            s.store_add_ad_lhs(1987, A::mul(A::div(s.ad_value(1976), s.ad_value(1939)), s.ad_value(1938)), 1970);
        }

        if (((s.v[1934] != 0.0) && (!(s.v[1991] != 0.0))) && (s.v[1992] != 0.0)) {
            let assign33870_ad_e30671: A = {
                if ((!(s.v[1987] > 50.0)) && (!(s.v[1987] < (-50.0)))) {
                    A::exp(s.ad_value(1987))
                } else {
                    {
                        if ((!(s.v[1987] > 50.0)) && (s.v[1987] < (-50.0))) {
                            A::exp(A::neg(A::constant(50.0)))
                        } else {
                            {
                                if (s.v[1987] > 50.0) {
                                    A::scale(A::offset(A::offset(s.ad_value(1987), (-50.0)), 1.0), ((50.0) as f64).exp())
                                } else {
                                    A::constant(0.0)
                                }
                            }
                        }
                    }
                }
            };
            s.store_ad(1988, &assign33870_ad_e30671);
        }

        if (((s.v[1934] != 0.0) && (!(s.v[1991] != 0.0))) && (s.v[1992] != 0.0)) {
            s.store_div_ad_lhs(1989, A::mul(s.ad_value(1936), s.ad_value(1982)), 1986);
        }

        if (((s.v[1934] != 0.0) && (!(s.v[1991] != 0.0))) && (s.v[1992] != 0.0)) {
            s.store_mul_ad_rhs(1990, 1989, A::sub(A::sub(s.ad_value(1988), A::mul(s.ad_value(1950), s.ad_value(1962))), s.ad_value(1960)));
        }

        if (((s.v[1934] != 0.0) && (!(s.v[1991] != 0.0))) && (!(s.v[1992] != 0.0))) {
            s.store_mul(1990, 1936, 1982);
        }

        if ((s.v[1934] != 0.0) && (!(s.v[1991] != 0.0))) {
            s.store_mul_ad_lhs(1959, A::square(s.ad_value(1941)), 1939);
        }

        if ((s.v[1934] != 0.0) && (!(s.v[1991] != 0.0))) {
            s.store_div_ad_lhs(1971, A::sub(s.ad_value(1938), A::sub(s.ad_value(1940), A::scale(s.ad_value(1959), 0.5))), 1959);
        }

        s.v[1993] = if (s.v[1971] > 50.0) { 1.0 } else { 0.0 };

    }

    pub(super) fn stamp_transient_block_25(
        &mut self,
        ctx: &GeneratedEvalContext<'_>,
        stamper: &mut GeneratedStamper<'_>,
        s: &mut Scratch,
    ) {
        let _ = stamper;
        let p = self.params;
        let nodes = self.nodes;
        let branches = self.branches;
        if (((s.v[1934] != 0.0) && (!(s.v[1991] != 0.0))) && (s.v[1993] != 0.0)) {
            s.store_scalar(1961, 0.0);
        }

        s.v[1994] = if (s.v[1971] < (-50.0)) { 1.0 } else { 0.0 };

        if ((((s.v[1934] != 0.0) && (!(s.v[1991] != 0.0))) && (!(s.v[1993] != 0.0))) && (s.v[1994] != 0.0)) {
            s.store_scalar(1961, 1.0);
        }

        if ((((s.v[1934] != 0.0) && (!(s.v[1991] != 0.0))) && (!(s.v[1993] != 0.0))) && (!(s.v[1994] != 0.0))) {
            s.store_div_from_scalar_ad(1961, 1.0, A::offset(A::exp(s.ad_value(1971)), 1.0));
        }

        if ((s.v[1934] != 0.0) && (!(s.v[1991] != 0.0))) {
            s.store_add_ad(1963, A::mul(s.ad_value(1961), s.ad_value(1983)), A::mul(A::sub_from_scalar(1.0, s.ad_value(1961)), s.ad_value(1990)));
        }

        if (s.v[1934] != 0.0) {
            let assign33990_ad_e30838: A = {
                if (!(p.p52 == 0.0)) {
                    A::mul(A::div(s.ad_value(1938), s.ad_value(1951)), A::tanh(A::scale(A::div(s.ad_value(1938), s.ad_value(1951)), (0.001 / p.p53))))
                } else {
                    {
                        if (p.p52 == 0.0) {
                            A::sqrt(A::offset(A::mul(A::div(s.ad_value(1938), s.ad_value(1951)), A::div(s.ad_value(1938), s.ad_value(1951))), p.p53))
                        } else {
                            A::constant(0.0)
                        }
                    }
                }
            };
            s.store_div_ad(1964, A::neg(s.ad_value(1938)), A::pow(A::offset(A::pow(assign33990_ad_e30838, s.ad_value(1952)), 1.0), A::div_from_scalar(1.0, s.ad_value(1952))));
        }

        if (s.v[1934] != 0.0) {
            s.store_mul_ad_lhs(1937, A::mul(A::mul(A::mul(A::neg(s.ad_value(1957)), s.ad_value(1947)), s.ad_value(1948)), s.ad_value(1953)), 1946);
        }

        if (s.v[1934] != 0.0) {
            s.store_mul_ad_lhs(1974, A::div(s.ad_value(1954), s.ad_value(1939)), 1964);
        }

        if (s.v[1934] != 0.0) {
            let assign34020_ad_e30911: A = {
                if ((!(s.v[1974] > 50.0)) && (!(s.v[1974] < (-50.0)))) {
                    A::exp(s.ad_value(1974))
                } else {
                    {
                        if ((!(s.v[1974] > 50.0)) && (s.v[1974] < (-50.0))) {
                            A::exp(A::neg(A::constant(50.0)))
                        } else {
                            {
                                if (s.v[1974] > 50.0) {
                                    A::scale(A::offset(A::offset(s.ad_value(1974), (-50.0)), 1.0), ((50.0) as f64).exp())
                                } else {
                                    A::constant(0.0)
                                }
                            }
                        }
                    }
                }
            };
            s.store_ad(1975, &assign34020_ad_e30911);
        }

        if (s.v[1934] != 0.0) {
            s.store_mul_ad_rhs(1965, 1937, A::offset(s.ad_value(1975), (-1.0)));
        }

        if (s.v[1934] != 0.0) {
            s.store_add(1958, 1963, 1965);
        }

        if (s.v[1934] != 0.0) {
            s.copy_ad(1935, 1958);
        }

        if (s.v[1934] != 0.0) {
            s.copy_ad(124, 1936);
        }

        if (s.v[1934] != 0.0) {
            s.copy_ad(126, 1937);
        }

        if (s.v[1934] != 0.0) {
            s.copy_ad(122, 1935);
        }

        if (s.v[1934] != 0.0) {
            s.store_scalar(1995, 0.0);
        }

        if (s.v[1934] != 0.0) {
            s.store_scalar(1996, 0.0);
        }

        if (s.v[1934] != 0.0) {
            s.store_scalar(1997, 0.0);
        }

        if (s.v[1934] != 0.0) {
            s.store_ad(1998, &A::scale(A::voltage(ctx, &nodes, Some(8), Some(17)), p.p6));
        }

        if (s.v[1934] != 0.0) {
            s.copy_ad(1999, 113);
        }

        if (s.v[1934] != 0.0) {
            s.store_scalar(2000, p.p265);
        }

        if (s.v[1934] != 0.0) {
            s.store_scalar(2001, p.p267);
        }

        if (s.v[1934] != 0.0) {
            s.store_scalar(2002, p.p266);
        }

        if (s.v[1934] != 0.0) {
            s.store_scalar(2003, p.p263);
        }

        if (s.v[1934] != 0.0) {
            s.store_scalar(2004, p.p281);
        }

        if (s.v[1934] != 0.0) {
            s.store_scalar(2005, p.p280);
        }

        if (s.v[1934] != 0.0) {
            s.copy_ad(2006, 112);
        }

        if (s.v[1934] != 0.0) {
            s.store_scalar(2007, p.p0);
        }

        if (s.v[1934] != 0.0) {
            s.store_scalar(2008, p.p2);
        }

        if (s.v[1934] != 0.0) {
            s.store_scalar(2009, ((1.0 - p.p255) * p.p264));
        }

        if (s.v[1934] != 0.0) {
            s.store_scalar(2010, p.p279);
        }

        if (s.v[1934] != 0.0) {
            s.store_scalar(2011, p.p274);
        }

        if (s.v[1934] != 0.0) {
            s.store_scalar(2012, p.p275);
        }

        if (s.v[1934] != 0.0) {
            s.store_scalar(2013, ((1.0 - p.p255) * p.p273));
        }

        if (s.v[1934] != 0.0) {
            s.store_scalar(2014, p.p272);
        }

        if (s.v[1934] != 0.0) {
            s.store_scalar(2015, p.p257);
        }

        if (s.v[1934] != 0.0) {
            s.store_scalar(2016, p.p256);
        }

        if (s.v[1934] != 0.0) {
            s.store_scalar(2017, p.p6);
        }

        if (s.v[1934] != 0.0) {
            s.store_scalar(2018, 0.0);
        }

        if (s.v[1934] != 0.0) {
            s.store_scalar(2019, 0.0);
        }

        if (s.v[1934] != 0.0) {
            s.store_scalar(2020, 0.0);
        }

        if (s.v[1934] != 0.0) {
            s.store_scalar(2021, 0.0);
        }

        if (s.v[1934] != 0.0) {
            s.store_scalar(2022, 0.0);
        }

        if (s.v[1934] != 0.0) {
            s.store_scalar(2023, 0.0);
        }

        if (s.v[1934] != 0.0) {
            s.store_scalar(2024, 0.0);
        }

        if (s.v[1934] != 0.0) {
            s.store_scalar(2025, 0.0);
        }

        if (s.v[1934] != 0.0) {
            s.store_scalar(2026, 0.0);
        }

        if (s.v[1934] != 0.0) {
            s.store_scalar(2027, 0.0);
        }

        if (s.v[1934] != 0.0) {
            s.store_scalar(2028, 0.0);
        }

        if (s.v[1934] != 0.0) {
            s.store_scalar(2029, 0.0);
        }

        if (s.v[1934] != 0.0) {
            s.store_scalar(2030, 0.0);
        }

        if (s.v[1934] != 0.0) {
            s.store_scalar(2031, 0.0);
        }

        if (s.v[1934] != 0.0) {
            s.store_scalar(2032, 0.0);
        }

        if (s.v[1934] != 0.0) {
            s.store_scalar(2033, 0.0);
        }

        if (s.v[1934] != 0.0) {
            s.store_scalar(2034, 0.0);
        }

        if (s.v[1934] != 0.0) {
            s.store_scalar(2035, 0.0);
        }

        if (s.v[1934] != 0.0) {
            s.store_scalar(2036, 0.0);
        }

        if (s.v[1934] != 0.0) {
            s.store_scalar(2037, 0.0);
        }

        if (s.v[1934] != 0.0) {
            s.store_scalar(2038, 0.0);
        }

        if (s.v[1934] != 0.0) {
            s.store_scalar(2039, 0.0);
        }

        if (s.v[1934] != 0.0) {
            s.store_scalar(2040, 0.0);
        }

        if (s.v[1934] != 0.0) {
            s.store_scalar(2041, 0.0);
        }

        if (s.v[1934] != 0.0) {
            s.store_scalar(2042, 0.0);
        }

        if (s.v[1934] != 0.0) {
            s.store_scalar(2043, 0.0);
        }

        if (s.v[1934] != 0.0) {
            s.store_scalar(2044, 0.0);
        }

        if (s.v[1934] != 0.0) {
            s.store_scalar(2045, 0.0);
        }

        if (s.v[1934] != 0.0) {
            s.store_scalar(2046, 0.0);
        }

        if (s.v[1934] != 0.0) {
            s.store_scalar(2047, 0.0);
        }

        if (s.v[1934] != 0.0) {
            s.store_scalar(2048, 0.0);
        }

        if (s.v[1934] != 0.0) {
            s.store_scalar(2049, 0.0);
        }

        if (s.v[1934] != 0.0) {
            s.store_scalar(2050, 0.0);
        }

        if (s.v[1934] != 0.0) {
            s.store_mul_ad(2030, A::div(s.ad_value(2015), s.ad_value(1999)), A::neg(s.ad_value(2016)));
        }

        if (s.v[1934] != 0.0) {
            let assign34660_ad_e31226: A = {
                if ((!(s.v[2030] > 50.0)) && (!(s.v[2030] < (-50.0)))) {
                    A::exp(s.ad_value(2030))
                } else {
                    {
                        if ((!(s.v[2030] > 50.0)) && (s.v[2030] < (-50.0))) {
                            A::exp(A::neg(A::constant(50.0)))
                        } else {
                            {
                                if (s.v[2030] > 50.0) {
                                    A::scale(A::offset(A::offset(s.ad_value(2030), (-50.0)), 1.0), ((50.0) as f64).exp())
                                } else {
                                    A::constant(0.0)
                                }
                            }
                        }
                    }
                }
            };
            s.store_ad(2020, &assign34660_ad_e31226);
        }

        if (s.v[1934] != 0.0) {
            s.store_add_ad_lhs(2026, A::mul(s.ad_value(2004), A::sub(A::neg(s.ad_value(1998)), s.ad_value(2005))), 2030);
        }

        if (s.v[1934] != 0.0) {
            s.store_add_ad_lhs(2027, A::mul(A::neg(s.ad_value(2004)), s.ad_value(2005)), 2030);
        }

        if (s.v[1934] != 0.0) {
            let assign34690_ad_e31288: A = {
                if ((!(s.v[2026] > 50.0)) && (!(s.v[2026] < (-50.0)))) {
                    A::exp(s.ad_value(2026))
                } else {
                    {
                        if ((!(s.v[2026] > 50.0)) && (s.v[2026] < (-50.0))) {
                            A::exp(A::neg(A::constant(50.0)))
                        } else {
                            {
                                if (s.v[2026] > 50.0) {
                                    A::scale(A::offset(A::offset(s.ad_value(2026), (-50.0)), 1.0), ((50.0) as f64).exp())
                                } else {
                                    A::constant(0.0)
                                }
                            }
                        }
                    }
                }
            };
            s.store_ad(2028, &assign34690_ad_e31288);
        }

        if (s.v[1934] != 0.0) {
            let assign34700_ad_e31330: A = {
                if ((!(s.v[2027] > 50.0)) && (!(s.v[2027] < (-50.0)))) {
                    A::exp(s.ad_value(2027))
                } else {
                    {
                        if ((!(s.v[2027] > 50.0)) && (s.v[2027] < (-50.0))) {
                            A::exp(A::neg(A::constant(50.0)))
                        } else {
                            {
                                if (s.v[2027] > 50.0) {
                                    A::scale(A::offset(A::offset(s.ad_value(2027), (-50.0)), 1.0), ((50.0) as f64).exp())
                                } else {
                                    A::constant(0.0)
                                }
                            }
                        }
                    }
                }
            };
            s.store_ad(2029, &assign34700_ad_e31330);
        }

        if (s.v[1934] != 0.0) {
            s.store_sub(2022, 2028, 2029);
        }

        if (s.v[1934] != 0.0) {
            s.store_mul_ad_lhs(1996, A::mul(A::mul(A::mul(s.ad_value(2017), s.ad_value(2007)), s.ad_value(2008)), s.ad_value(2009)), 2006);
        }

        if (s.v[1934] != 0.0) {
            s.store_add_ad_lhs(2032, A::mul(A::div(s.ad_value(2003), s.ad_value(1999)), s.ad_value(1998)), 2030);
        }

        if (s.v[1934] != 0.0) {
            let assign34740_ad_e31400: A = {
                if ((!(s.v[2032] > 50.0)) && (!(s.v[2032] < (-50.0)))) {
                    A::exp(s.ad_value(2032))
                } else {
                    {
                        if ((!(s.v[2032] > 50.0)) && (s.v[2032] < (-50.0))) {
                            A::exp(A::neg(A::constant(50.0)))
                        } else {
                            {
                                if (s.v[2032] > 50.0) {
                                    A::scale(A::offset(A::offset(s.ad_value(2032), (-50.0)), 1.0), ((50.0) as f64).exp())
                                } else {
                                    A::constant(0.0)
                                }
                            }
                        }
                    }
                }
            };
            s.store_ad(2033, &assign34740_ad_e31400);
        }

        s.v[2051] = if (s.v[2002] == 1.0) { 1.0 } else { 0.0 };

        if ((s.v[1934] != 0.0) && (s.v[2051] != 0.0)) {
            s.store_mul_ad_rhs(2023, 1996, A::sub(A::sub(s.ad_value(2033), A::mul(s.ad_value(2010), s.ad_value(2022))), s.ad_value(2020)));
        }

        if ((s.v[1934] != 0.0) && (!(s.v[2051] != 0.0))) {
            s.store_add_ad_lhs(2037, A::mul(s.ad_value(2004), A::sub(A::neg(s.ad_value(2000)), s.ad_value(2005))), 2030);
        }

        if ((s.v[1934] != 0.0) && (!(s.v[2051] != 0.0))) {
            let assign34780_ad_e31476: A = {
                if ((!(s.v[2037] > 50.0)) && (!(s.v[2037] < (-50.0)))) {
                    A::exp(s.ad_value(2037))
                } else {
                    {
                        if ((!(s.v[2037] > 50.0)) && (s.v[2037] < (-50.0))) {
                            A::exp(A::neg(A::constant(50.0)))
                        } else {
                            {
                                if (s.v[2037] > 50.0) {
                                    A::scale(A::offset(A::offset(s.ad_value(2037), (-50.0)), 1.0), ((50.0) as f64).exp())
                                } else {
                                    A::constant(0.0)
                                }
                            }
                        }
                    }
                }
            };
            s.store_ad(2038, &assign34780_ad_e31476);
        }

        if ((s.v[1934] != 0.0) && (!(s.v[2051] != 0.0))) {
            s.store_sub(2039, 2038, 2029);
        }

        if ((s.v[1934] != 0.0) && (!(s.v[2051] != 0.0))) {
            s.store_add_ad_lhs(2040, A::mul(A::div(s.ad_value(2003), s.ad_value(1999)), s.ad_value(2000)), 2030);
        }

        if ((s.v[1934] != 0.0) && (!(s.v[2051] != 0.0))) {
            let assign34810_ad_e31543: A = {
                if ((!(s.v[2040] > 50.0)) && (!(s.v[2040] < (-50.0)))) {
                    A::exp(s.ad_value(2040))
                } else {
                    {
                        if ((!(s.v[2040] > 50.0)) && (s.v[2040] < (-50.0))) {
                            A::exp(A::neg(A::constant(50.0)))
                        } else {
                            {
                                if (s.v[2040] > 50.0) {
                                    A::scale(A::offset(A::offset(s.ad_value(2040), (-50.0)), 1.0), ((50.0) as f64).exp())
                                } else {
                                    A::constant(0.0)
                                }
                            }
                        }
                    }
                }
            };
            s.store_ad(2041, &assign34810_ad_e31543);
        }

        if ((s.v[1934] != 0.0) && (!(s.v[2051] != 0.0))) {
            s.store_sub_ad_lhs(2042, A::sub(s.ad_value(2041), A::mul(s.ad_value(2010), s.ad_value(2039))), 2020);
        }

        if ((s.v[1934] != 0.0) && (!(s.v[2051] != 0.0))) {
            s.store_mul_ad_rhs(2043, 1996, A::sub(A::sub(s.ad_value(2033), A::mul(s.ad_value(2010), s.ad_value(2022))), s.ad_value(2020)));
        }

        s.v[2052] = if (s.v[2002] > 0.0) { 1.0 } else { 0.0 };

        if (((s.v[1934] != 0.0) && (!(s.v[2051] != 0.0))) && (s.v[2052] != 0.0)) {
            s.store_mul(2036, 2002, 2003);
        }

        if (((s.v[1934] != 0.0) && (!(s.v[2051] != 0.0))) && (s.v[2052] != 0.0)) {
            s.store_add_ad_lhs(2044, A::mul(A::div(s.ad_value(2036), s.ad_value(1999)), s.ad_value(2000)), 2030);
        }

    }

    pub(super) fn stamp_transient_block_26(
        &mut self,
        ctx: &GeneratedEvalContext<'_>,
        stamper: &mut GeneratedStamper<'_>,
        s: &mut Scratch,
    ) {
        let _ = stamper;
        let p = self.params;
        let nodes = self.nodes;
        let branches = self.branches;
        if (((s.v[1934] != 0.0) && (!(s.v[2051] != 0.0))) && (s.v[2052] != 0.0)) {
            let assign34870_ad_e31647: A = {
                if ((!(s.v[2044] > 50.0)) && (!(s.v[2044] < (-50.0)))) {
                    A::exp(s.ad_value(2044))
                } else {
                    {
                        if ((!(s.v[2044] > 50.0)) && (s.v[2044] < (-50.0))) {
                            A::exp(A::neg(A::constant(50.0)))
                        } else {
                            {
                                if (s.v[2044] > 50.0) {
                                    A::scale(A::offset(A::offset(s.ad_value(2044), (-50.0)), 1.0), ((50.0) as f64).exp())
                                } else {
                                    A::constant(0.0)
                                }
                            }
                        }
                    }
                }
            };
            s.store_ad(2045, &assign34870_ad_e31647);
        }

        if (((s.v[1934] != 0.0) && (!(s.v[2051] != 0.0))) && (s.v[2052] != 0.0)) {
            s.store_sub_ad_lhs(2046, A::sub(s.ad_value(2045), A::mul(s.ad_value(2010), s.ad_value(2039))), 2020);
        }

        if (((s.v[1934] != 0.0) && (!(s.v[2051] != 0.0))) && (s.v[2052] != 0.0)) {
            s.store_add_ad_lhs(2047, A::mul(A::div(s.ad_value(2036), s.ad_value(1999)), s.ad_value(1998)), 2030);
        }

        if (((s.v[1934] != 0.0) && (!(s.v[2051] != 0.0))) && (s.v[2052] != 0.0)) {
            let assign34900_ad_e31724: A = {
                if ((!(s.v[2047] > 50.0)) && (!(s.v[2047] < (-50.0)))) {
                    A::exp(s.ad_value(2047))
                } else {
                    {
                        if ((!(s.v[2047] > 50.0)) && (s.v[2047] < (-50.0))) {
                            A::exp(A::neg(A::constant(50.0)))
                        } else {
                            {
                                if (s.v[2047] > 50.0) {
                                    A::scale(A::offset(A::offset(s.ad_value(2047), (-50.0)), 1.0), ((50.0) as f64).exp())
                                } else {
                                    A::constant(0.0)
                                }
                            }
                        }
                    }
                }
            };
            s.store_ad(2048, &assign34900_ad_e31724);
        }

        if (((s.v[1934] != 0.0) && (!(s.v[2051] != 0.0))) && (s.v[2052] != 0.0)) {
            s.store_div_ad_lhs(2049, A::mul(s.ad_value(1996), s.ad_value(2042)), 2046);
        }

        if (((s.v[1934] != 0.0) && (!(s.v[2051] != 0.0))) && (s.v[2052] != 0.0)) {
            s.store_mul_ad_rhs(2050, 2049, A::sub(A::sub(s.ad_value(2048), A::mul(s.ad_value(2010), s.ad_value(2022))), s.ad_value(2020)));
        }

        if (((s.v[1934] != 0.0) && (!(s.v[2051] != 0.0))) && (!(s.v[2052] != 0.0))) {
            s.store_mul(2050, 1996, 2042);
        }

        if ((s.v[1934] != 0.0) && (!(s.v[2051] != 0.0))) {
            s.store_mul_ad_lhs(2019, A::square(s.ad_value(2001)), 1999);
        }

        if ((s.v[1934] != 0.0) && (!(s.v[2051] != 0.0))) {
            s.store_div_ad_lhs(2031, A::sub(s.ad_value(1998), A::sub(s.ad_value(2000), A::scale(s.ad_value(2019), 0.5))), 2019);
        }

        s.v[2053] = if (s.v[2031] > 50.0) { 1.0 } else { 0.0 };

        if (((s.v[1934] != 0.0) && (!(s.v[2051] != 0.0))) && (s.v[2053] != 0.0)) {
            s.store_scalar(2021, 0.0);
        }

        s.v[2054] = if (s.v[2031] < (-50.0)) { 1.0 } else { 0.0 };

        if ((((s.v[1934] != 0.0) && (!(s.v[2051] != 0.0))) && (!(s.v[2053] != 0.0))) && (s.v[2054] != 0.0)) {
            s.store_scalar(2021, 1.0);
        }

        if ((((s.v[1934] != 0.0) && (!(s.v[2051] != 0.0))) && (!(s.v[2053] != 0.0))) && (!(s.v[2054] != 0.0))) {
            s.store_div_from_scalar_ad(2021, 1.0, A::offset(A::exp(s.ad_value(2031)), 1.0));
        }

        if ((s.v[1934] != 0.0) && (!(s.v[2051] != 0.0))) {
            s.store_add_ad(2023, A::mul(s.ad_value(2021), s.ad_value(2043)), A::mul(A::sub_from_scalar(1.0, s.ad_value(2021)), s.ad_value(2050)));
        }

        if (s.v[1934] != 0.0) {
            let assign35020_ad_e31891: A = {
                if (!(p.p52 == 0.0)) {
                    A::mul(A::div(s.ad_value(1998), s.ad_value(2011)), A::tanh(A::scale(A::div(s.ad_value(1998), s.ad_value(2011)), (0.001 / p.p53))))
                } else {
                    {
                        if (p.p52 == 0.0) {
                            A::sqrt(A::offset(A::mul(A::div(s.ad_value(1998), s.ad_value(2011)), A::div(s.ad_value(1998), s.ad_value(2011))), p.p53))
                        } else {
                            A::constant(0.0)
                        }
                    }
                }
            };
            s.store_div_ad(2024, A::neg(s.ad_value(1998)), A::pow(A::offset(A::pow(assign35020_ad_e31891, s.ad_value(2012)), 1.0), A::div_from_scalar(1.0, s.ad_value(2012))));
        }

        if (s.v[1934] != 0.0) {
            s.store_mul_ad_lhs(1997, A::mul(A::mul(A::mul(A::neg(s.ad_value(2017)), s.ad_value(2007)), s.ad_value(2008)), s.ad_value(2013)), 2006);
        }

        if (s.v[1934] != 0.0) {
            s.store_mul_ad_lhs(2034, A::div(s.ad_value(2014), s.ad_value(1999)), 2024);
        }

        if (s.v[1934] != 0.0) {
            let assign35050_ad_e31964: A = {
                if ((!(s.v[2034] > 50.0)) && (!(s.v[2034] < (-50.0)))) {
                    A::exp(s.ad_value(2034))
                } else {
                    {
                        if ((!(s.v[2034] > 50.0)) && (s.v[2034] < (-50.0))) {
                            A::exp(A::neg(A::constant(50.0)))
                        } else {
                            {
                                if (s.v[2034] > 50.0) {
                                    A::scale(A::offset(A::offset(s.ad_value(2034), (-50.0)), 1.0), ((50.0) as f64).exp())
                                } else {
                                    A::constant(0.0)
                                }
                            }
                        }
                    }
                }
            };
            s.store_ad(2035, &assign35050_ad_e31964);
        }

        if (s.v[1934] != 0.0) {
            s.store_mul_ad_rhs(2025, 1997, A::offset(s.ad_value(2035), (-1.0)));
        }

        if (s.v[1934] != 0.0) {
            s.store_add(2018, 2023, 2025);
        }

        if (s.v[1934] != 0.0) {
            s.copy_ad(1995, 2018);
        }

        if (s.v[1934] != 0.0) {
            s.copy_ad(125, 1996);
        }

        if (s.v[1934] != 0.0) {
            s.copy_ad(127, 1997);
        }

        if (s.v[1934] != 0.0) {
            s.copy_ad(123, 1995);
        }

        s.v[2055] = if (p.p282 == 1.0) { 1.0 } else { 0.0 };

        if ((s.v[1934] != 0.0) && (s.v[2055] != 0.0)) {
            s.store_scalar(2056, 0.0);
        }

        if ((s.v[1934] != 0.0) && (s.v[2055] != 0.0)) {
            s.store_scalar(2057, 0.0);
        }

        if ((s.v[1934] != 0.0) && (s.v[2055] != 0.0)) {
            s.store_scalar(2058, 0.0);
        }

        if ((s.v[1934] != 0.0) && (s.v[2055] != 0.0)) {
            s.store_ad(2059, &A::scale(A::voltage(ctx, &nodes, Some(8), Some(13)), p.p6));
        }

        if ((s.v[1934] != 0.0) && (s.v[2055] != 0.0)) {
            s.copy_ad(2060, 113);
        }

        if ((s.v[1934] != 0.0) && (s.v[2055] != 0.0)) {
            s.store_scalar(2061, p.p260);
        }

        if ((s.v[1934] != 0.0) && (s.v[2055] != 0.0)) {
            s.store_scalar(2062, p.p262);
        }

        if ((s.v[1934] != 0.0) && (s.v[2055] != 0.0)) {
            s.store_scalar(2063, 1.0);
        }

        if ((s.v[1934] != 0.0) && (s.v[2055] != 0.0)) {
            s.store_scalar(2064, p.p258);
        }

        if ((s.v[1934] != 0.0) && (s.v[2055] != 0.0)) {
            s.store_scalar(2065, p.p278);
        }

        if ((s.v[1934] != 0.0) && (s.v[2055] != 0.0)) {
            s.store_scalar(2066, p.p277);
        }

        if ((s.v[1934] != 0.0) && (s.v[2055] != 0.0)) {
            s.copy_ad(2067, 112);
        }

        if ((s.v[1934] != 0.0) && (s.v[2055] != 0.0)) {
            s.store_scalar(2068, p.p0);
        }

        if ((s.v[1934] != 0.0) && (s.v[2055] != 0.0)) {
            s.store_scalar(2069, p.p2);
        }

        if ((s.v[1934] != 0.0) && (s.v[2055] != 0.0)) {
            s.store_scalar(2070, 0.0);
        }

        if ((s.v[1934] != 0.0) && (s.v[2055] != 0.0)) {
            s.store_scalar(2071, 0.0);
        }

        if ((s.v[1934] != 0.0) && (s.v[2055] != 0.0)) {
            s.store_scalar(2072, p.p285);
        }

        if ((s.v[1934] != 0.0) && (s.v[2055] != 0.0)) {
            s.store_scalar(2073, p.p286);
        }

        if ((s.v[1934] != 0.0) && (s.v[2055] != 0.0)) {
            s.store_scalar(2074, ((1.0 - p.p255) * p.p284));
        }

        if ((s.v[1934] != 0.0) && (s.v[2055] != 0.0)) {
            s.store_scalar(2075, p.p283);
        }

        if ((s.v[1934] != 0.0) && (s.v[2055] != 0.0)) {
            s.store_scalar(2076, p.p257);
        }

        if ((s.v[1934] != 0.0) && (s.v[2055] != 0.0)) {
            s.store_scalar(2077, p.p256);
        }

        if ((s.v[1934] != 0.0) && (s.v[2055] != 0.0)) {
            s.store_scalar(2078, p.p6);
        }

        if ((s.v[1934] != 0.0) && (s.v[2055] != 0.0)) {
            s.store_scalar(2079, 0.0);
        }

        if ((s.v[1934] != 0.0) && (s.v[2055] != 0.0)) {
            s.store_scalar(2080, 0.0);
        }

        if ((s.v[1934] != 0.0) && (s.v[2055] != 0.0)) {
            s.store_scalar(2081, 0.0);
        }

        if ((s.v[1934] != 0.0) && (s.v[2055] != 0.0)) {
            s.store_scalar(2082, 0.0);
        }

        if ((s.v[1934] != 0.0) && (s.v[2055] != 0.0)) {
            s.store_scalar(2083, 0.0);
        }

        if ((s.v[1934] != 0.0) && (s.v[2055] != 0.0)) {
            s.store_scalar(2084, 0.0);
        }

        if ((s.v[1934] != 0.0) && (s.v[2055] != 0.0)) {
            s.store_scalar(2085, 0.0);
        }

        if ((s.v[1934] != 0.0) && (s.v[2055] != 0.0)) {
            s.store_scalar(2086, 0.0);
        }

        if ((s.v[1934] != 0.0) && (s.v[2055] != 0.0)) {
            s.store_scalar(2087, 0.0);
        }

        if ((s.v[1934] != 0.0) && (s.v[2055] != 0.0)) {
            s.store_scalar(2088, 0.0);
        }

        if ((s.v[1934] != 0.0) && (s.v[2055] != 0.0)) {
            s.store_scalar(2089, 0.0);
        }

        if ((s.v[1934] != 0.0) && (s.v[2055] != 0.0)) {
            s.store_scalar(2090, 0.0);
        }

        if ((s.v[1934] != 0.0) && (s.v[2055] != 0.0)) {
            s.store_scalar(2091, 0.0);
        }

        if ((s.v[1934] != 0.0) && (s.v[2055] != 0.0)) {
            s.store_scalar(2092, 0.0);
        }

        if ((s.v[1934] != 0.0) && (s.v[2055] != 0.0)) {
            s.store_scalar(2093, 0.0);
        }

        if ((s.v[1934] != 0.0) && (s.v[2055] != 0.0)) {
            s.store_scalar(2094, 0.0);
        }

        if ((s.v[1934] != 0.0) && (s.v[2055] != 0.0)) {
            s.store_scalar(2095, 0.0);
        }

        if ((s.v[1934] != 0.0) && (s.v[2055] != 0.0)) {
            s.store_scalar(2096, 0.0);
        }

        if ((s.v[1934] != 0.0) && (s.v[2055] != 0.0)) {
            s.store_scalar(2097, 0.0);
        }

        if ((s.v[1934] != 0.0) && (s.v[2055] != 0.0)) {
            s.store_scalar(2098, 0.0);
        }

        if ((s.v[1934] != 0.0) && (s.v[2055] != 0.0)) {
            s.store_scalar(2099, 0.0);
        }

        if ((s.v[1934] != 0.0) && (s.v[2055] != 0.0)) {
            s.store_scalar(2100, 0.0);
        }

        if ((s.v[1934] != 0.0) && (s.v[2055] != 0.0)) {
            s.store_scalar(2101, 0.0);
        }

        if ((s.v[1934] != 0.0) && (s.v[2055] != 0.0)) {
            s.store_scalar(2102, 0.0);
        }

        if ((s.v[1934] != 0.0) && (s.v[2055] != 0.0)) {
            s.store_scalar(2103, 0.0);
        }

        if ((s.v[1934] != 0.0) && (s.v[2055] != 0.0)) {
            s.store_scalar(2104, 0.0);
        }

        if ((s.v[1934] != 0.0) && (s.v[2055] != 0.0)) {
            s.store_scalar(2105, 0.0);
        }

        if ((s.v[1934] != 0.0) && (s.v[2055] != 0.0)) {
            s.store_scalar(2106, 0.0);
        }

        if ((s.v[1934] != 0.0) && (s.v[2055] != 0.0)) {
            s.store_scalar(2107, 0.0);
        }

        if ((s.v[1934] != 0.0) && (s.v[2055] != 0.0)) {
            s.store_scalar(2108, 0.0);
        }

        if ((s.v[1934] != 0.0) && (s.v[2055] != 0.0)) {
            s.store_scalar(2109, 0.0);
        }

        if ((s.v[1934] != 0.0) && (s.v[2055] != 0.0)) {
            s.store_scalar(2110, 0.0);
        }

        if ((s.v[1934] != 0.0) && (s.v[2055] != 0.0)) {
            s.store_scalar(2111, 0.0);
        }

        if ((s.v[1934] != 0.0) && (s.v[2055] != 0.0)) {
            s.store_mul_ad(2091, A::div(s.ad_value(2076), s.ad_value(2060)), A::neg(s.ad_value(2077)));
        }

        if ((s.v[1934] != 0.0) && (s.v[2055] != 0.0)) {
            let assign35700_ad_e32394: A = {
                if ((!(s.v[2091] > 50.0)) && (!(s.v[2091] < (-50.0)))) {
                    A::exp(s.ad_value(2091))
                } else {
                    {
                        if ((!(s.v[2091] > 50.0)) && (s.v[2091] < (-50.0))) {
                            A::exp(A::neg(A::constant(50.0)))
                        } else {
                            {
                                if (s.v[2091] > 50.0) {
                                    A::scale(A::offset(A::offset(s.ad_value(2091), (-50.0)), 1.0), ((50.0) as f64).exp())
                                } else {
                                    A::constant(0.0)
                                }
                            }
                        }
                    }
                }
            };
            s.store_ad(2081, &assign35700_ad_e32394);
        }

        if ((s.v[1934] != 0.0) && (s.v[2055] != 0.0)) {
            s.store_add_ad_lhs(2087, A::mul(s.ad_value(2065), A::sub(A::neg(s.ad_value(2059)), s.ad_value(2066))), 2091);
        }

        if ((s.v[1934] != 0.0) && (s.v[2055] != 0.0)) {
            s.store_add_ad_lhs(2088, A::mul(A::neg(s.ad_value(2065)), s.ad_value(2066)), 2091);
        }

        if ((s.v[1934] != 0.0) && (s.v[2055] != 0.0)) {
            let assign35730_ad_e32462: A = {
                if ((!(s.v[2087] > 50.0)) && (!(s.v[2087] < (-50.0)))) {
                    A::exp(s.ad_value(2087))
                } else {
                    {
                        if ((!(s.v[2087] > 50.0)) && (s.v[2087] < (-50.0))) {
                            A::exp(A::neg(A::constant(50.0)))
                        } else {
                            {
                                if (s.v[2087] > 50.0) {
                                    A::scale(A::offset(A::offset(s.ad_value(2087), (-50.0)), 1.0), ((50.0) as f64).exp())
                                } else {
                                    A::constant(0.0)
                                }
                            }
                        }
                    }
                }
            };
            s.store_ad(2089, &assign35730_ad_e32462);
        }

        if ((s.v[1934] != 0.0) && (s.v[2055] != 0.0)) {
            let assign35740_ad_e32506: A = {
                if ((!(s.v[2088] > 50.0)) && (!(s.v[2088] < (-50.0)))) {
                    A::exp(s.ad_value(2088))
                } else {
                    {
                        if ((!(s.v[2088] > 50.0)) && (s.v[2088] < (-50.0))) {
                            A::exp(A::neg(A::constant(50.0)))
                        } else {
                            {
                                if (s.v[2088] > 50.0) {
                                    A::scale(A::offset(A::offset(s.ad_value(2088), (-50.0)), 1.0), ((50.0) as f64).exp())
                                } else {
                                    A::constant(0.0)
                                }
                            }
                        }
                    }
                }
            };
            s.store_ad(2090, &assign35740_ad_e32506);
        }

        if ((s.v[1934] != 0.0) && (s.v[2055] != 0.0)) {
            s.store_sub(2083, 2089, 2090);
        }

        if ((s.v[1934] != 0.0) && (s.v[2055] != 0.0)) {
            s.store_mul_ad_lhs(2057, A::mul(A::mul(A::mul(s.ad_value(2078), s.ad_value(2068)), s.ad_value(2069)), s.ad_value(2070)), 2067);
        }

        if ((s.v[1934] != 0.0) && (s.v[2055] != 0.0)) {
            s.store_add_ad_lhs(2093, A::mul(A::div(s.ad_value(2064), s.ad_value(2060)), s.ad_value(2059)), 2091);
        }

        if ((s.v[1934] != 0.0) && (s.v[2055] != 0.0)) {
            let assign35780_ad_e32584: A = {
                if ((!(s.v[2093] > 50.0)) && (!(s.v[2093] < (-50.0)))) {
                    A::exp(s.ad_value(2093))
                } else {
                    {
                        if ((!(s.v[2093] > 50.0)) && (s.v[2093] < (-50.0))) {
                            A::exp(A::neg(A::constant(50.0)))
                        } else {
                            {
                                if (s.v[2093] > 50.0) {
                                    A::scale(A::offset(A::offset(s.ad_value(2093), (-50.0)), 1.0), ((50.0) as f64).exp())
                                } else {
                                    A::constant(0.0)
                                }
                            }
                        }
                    }
                }
            };
            s.store_ad(2094, &assign35780_ad_e32584);
        }

        s.v[2112] = if (s.v[2063] == 1.0) { 1.0 } else { 0.0 };

    }

    pub(super) fn stamp_transient_block_27(
        &mut self,
        ctx: &GeneratedEvalContext<'_>,
        stamper: &mut GeneratedStamper<'_>,
        s: &mut Scratch,
    ) {
        let _ = stamper;
        let p = self.params;
        let nodes = self.nodes;
        let branches = self.branches;
        if (((s.v[1934] != 0.0) && (s.v[2055] != 0.0)) && (s.v[2112] != 0.0)) {
            s.store_mul_ad_rhs(2084, 2057, A::sub(A::sub(s.ad_value(2094), A::mul(s.ad_value(2071), s.ad_value(2083))), s.ad_value(2081)));
        }

        if (((s.v[1934] != 0.0) && (s.v[2055] != 0.0)) && (!(s.v[2112] != 0.0))) {
            s.store_add_ad_lhs(2098, A::mul(s.ad_value(2065), A::sub(A::neg(s.ad_value(2061)), s.ad_value(2066))), 2091);
        }

        if (((s.v[1934] != 0.0) && (s.v[2055] != 0.0)) && (!(s.v[2112] != 0.0))) {
            let assign35820_ad_e32666: A = {
                if ((!(s.v[2098] > 50.0)) && (!(s.v[2098] < (-50.0)))) {
                    A::exp(s.ad_value(2098))
                } else {
                    {
                        if ((!(s.v[2098] > 50.0)) && (s.v[2098] < (-50.0))) {
                            A::exp(A::neg(A::constant(50.0)))
                        } else {
                            {
                                if (s.v[2098] > 50.0) {
                                    A::scale(A::offset(A::offset(s.ad_value(2098), (-50.0)), 1.0), ((50.0) as f64).exp())
                                } else {
                                    A::constant(0.0)
                                }
                            }
                        }
                    }
                }
            };
            s.store_ad(2099, &assign35820_ad_e32666);
        }

        if (((s.v[1934] != 0.0) && (s.v[2055] != 0.0)) && (!(s.v[2112] != 0.0))) {
            s.store_sub(2100, 2099, 2090);
        }

        if (((s.v[1934] != 0.0) && (s.v[2055] != 0.0)) && (!(s.v[2112] != 0.0))) {
            s.store_add_ad_lhs(2101, A::mul(A::div(s.ad_value(2064), s.ad_value(2060)), s.ad_value(2061)), 2091);
        }

        if (((s.v[1934] != 0.0) && (s.v[2055] != 0.0)) && (!(s.v[2112] != 0.0))) {
            let assign35850_ad_e32739: A = {
                if ((!(s.v[2101] > 50.0)) && (!(s.v[2101] < (-50.0)))) {
                    A::exp(s.ad_value(2101))
                } else {
                    {
                        if ((!(s.v[2101] > 50.0)) && (s.v[2101] < (-50.0))) {
                            A::exp(A::neg(A::constant(50.0)))
                        } else {
                            {
                                if (s.v[2101] > 50.0) {
                                    A::scale(A::offset(A::offset(s.ad_value(2101), (-50.0)), 1.0), ((50.0) as f64).exp())
                                } else {
                                    A::constant(0.0)
                                }
                            }
                        }
                    }
                }
            };
            s.store_ad(2102, &assign35850_ad_e32739);
        }

        if (((s.v[1934] != 0.0) && (s.v[2055] != 0.0)) && (!(s.v[2112] != 0.0))) {
            s.store_sub_ad_lhs(2103, A::sub(s.ad_value(2102), A::mul(s.ad_value(2071), s.ad_value(2100))), 2081);
        }

        if (((s.v[1934] != 0.0) && (s.v[2055] != 0.0)) && (!(s.v[2112] != 0.0))) {
            s.store_mul_ad_rhs(2104, 2057, A::sub(A::sub(s.ad_value(2094), A::mul(s.ad_value(2071), s.ad_value(2083))), s.ad_value(2081)));
        }

        s.v[2113] = if (s.v[2063] > 0.0) { 1.0 } else { 0.0 };

        if ((((s.v[1934] != 0.0) && (s.v[2055] != 0.0)) && (!(s.v[2112] != 0.0))) && (s.v[2113] != 0.0)) {
            s.store_mul(2097, 2063, 2064);
        }

        if ((((s.v[1934] != 0.0) && (s.v[2055] != 0.0)) && (!(s.v[2112] != 0.0))) && (s.v[2113] != 0.0)) {
            s.store_add_ad_lhs(2105, A::mul(A::div(s.ad_value(2097), s.ad_value(2060)), s.ad_value(2061)), 2091);
        }

        if ((((s.v[1934] != 0.0) && (s.v[2055] != 0.0)) && (!(s.v[2112] != 0.0))) && (s.v[2113] != 0.0)) {
            let assign35910_ad_e32853: A = {
                if ((!(s.v[2105] > 50.0)) && (!(s.v[2105] < (-50.0)))) {
                    A::exp(s.ad_value(2105))
                } else {
                    {
                        if ((!(s.v[2105] > 50.0)) && (s.v[2105] < (-50.0))) {
                            A::exp(A::neg(A::constant(50.0)))
                        } else {
                            {
                                if (s.v[2105] > 50.0) {
                                    A::scale(A::offset(A::offset(s.ad_value(2105), (-50.0)), 1.0), ((50.0) as f64).exp())
                                } else {
                                    A::constant(0.0)
                                }
                            }
                        }
                    }
                }
            };
            s.store_ad(2106, &assign35910_ad_e32853);
        }

        if ((((s.v[1934] != 0.0) && (s.v[2055] != 0.0)) && (!(s.v[2112] != 0.0))) && (s.v[2113] != 0.0)) {
            s.store_sub_ad_lhs(2107, A::sub(s.ad_value(2106), A::mul(s.ad_value(2071), s.ad_value(2100))), 2081);
        }

        if ((((s.v[1934] != 0.0) && (s.v[2055] != 0.0)) && (!(s.v[2112] != 0.0))) && (s.v[2113] != 0.0)) {
            s.store_add_ad_lhs(2108, A::mul(A::div(s.ad_value(2097), s.ad_value(2060)), s.ad_value(2059)), 2091);
        }

        if ((((s.v[1934] != 0.0) && (s.v[2055] != 0.0)) && (!(s.v[2112] != 0.0))) && (s.v[2113] != 0.0)) {
            let assign35940_ad_e32936: A = {
                if ((!(s.v[2108] > 50.0)) && (!(s.v[2108] < (-50.0)))) {
                    A::exp(s.ad_value(2108))
                } else {
                    {
                        if ((!(s.v[2108] > 50.0)) && (s.v[2108] < (-50.0))) {
                            A::exp(A::neg(A::constant(50.0)))
                        } else {
                            {
                                if (s.v[2108] > 50.0) {
                                    A::scale(A::offset(A::offset(s.ad_value(2108), (-50.0)), 1.0), ((50.0) as f64).exp())
                                } else {
                                    A::constant(0.0)
                                }
                            }
                        }
                    }
                }
            };
            s.store_ad(2109, &assign35940_ad_e32936);
        }

        if ((((s.v[1934] != 0.0) && (s.v[2055] != 0.0)) && (!(s.v[2112] != 0.0))) && (s.v[2113] != 0.0)) {
            s.store_div_ad_lhs(2110, A::mul(s.ad_value(2057), s.ad_value(2103)), 2107);
        }

        if ((((s.v[1934] != 0.0) && (s.v[2055] != 0.0)) && (!(s.v[2112] != 0.0))) && (s.v[2113] != 0.0)) {
            s.store_mul_ad_rhs(2111, 2110, A::sub(A::sub(s.ad_value(2109), A::mul(s.ad_value(2071), s.ad_value(2083))), s.ad_value(2081)));
        }

        if ((((s.v[1934] != 0.0) && (s.v[2055] != 0.0)) && (!(s.v[2112] != 0.0))) && (!(s.v[2113] != 0.0))) {
            s.store_mul(2111, 2057, 2103);
        }

        if (((s.v[1934] != 0.0) && (s.v[2055] != 0.0)) && (!(s.v[2112] != 0.0))) {
            s.store_mul_ad_lhs(2080, A::square(s.ad_value(2062)), 2060);
        }

        if (((s.v[1934] != 0.0) && (s.v[2055] != 0.0)) && (!(s.v[2112] != 0.0))) {
            s.store_div_ad_lhs(2092, A::sub(s.ad_value(2059), A::sub(s.ad_value(2061), A::scale(s.ad_value(2080), 0.5))), 2080);
        }

        s.v[2114] = if (s.v[2092] > 50.0) { 1.0 } else { 0.0 };

        if ((((s.v[1934] != 0.0) && (s.v[2055] != 0.0)) && (!(s.v[2112] != 0.0))) && (s.v[2114] != 0.0)) {
            s.store_scalar(2082, 0.0);
        }

        s.v[2115] = if (s.v[2092] < (-50.0)) { 1.0 } else { 0.0 };

        if (((((s.v[1934] != 0.0) && (s.v[2055] != 0.0)) && (!(s.v[2112] != 0.0))) && (!(s.v[2114] != 0.0))) && (s.v[2115] != 0.0)) {
            s.store_scalar(2082, 1.0);
        }

        if (((((s.v[1934] != 0.0) && (s.v[2055] != 0.0)) && (!(s.v[2112] != 0.0))) && (!(s.v[2114] != 0.0))) && (!(s.v[2115] != 0.0))) {
            s.store_div_from_scalar_ad(2082, 1.0, A::offset(A::exp(s.ad_value(2092)), 1.0));
        }

        if (((s.v[1934] != 0.0) && (s.v[2055] != 0.0)) && (!(s.v[2112] != 0.0))) {
            s.store_add_ad(2084, A::mul(s.ad_value(2082), s.ad_value(2104)), A::mul(A::sub_from_scalar(1.0, s.ad_value(2082)), s.ad_value(2111)));
        }

        if ((s.v[1934] != 0.0) && (s.v[2055] != 0.0)) {
            let assign36060_ad_e33123: A = {
                if (!(p.p52 == 0.0)) {
                    A::mul(A::div(s.ad_value(2059), s.ad_value(2072)), A::tanh(A::scale(A::div(s.ad_value(2059), s.ad_value(2072)), (0.001 / p.p53))))
                } else {
                    {
                        if (p.p52 == 0.0) {
                            A::sqrt(A::offset(A::mul(A::div(s.ad_value(2059), s.ad_value(2072)), A::div(s.ad_value(2059), s.ad_value(2072))), p.p53))
                        } else {
                            A::constant(0.0)
                        }
                    }
                }
            };
            s.store_div_ad(2085, A::neg(s.ad_value(2059)), A::pow(A::offset(A::pow(assign36060_ad_e33123, s.ad_value(2073)), 1.0), A::div_from_scalar(1.0, s.ad_value(2073))));
        }

        if ((s.v[1934] != 0.0) && (s.v[2055] != 0.0)) {
            s.store_mul_ad_lhs(2058, A::mul(A::mul(A::mul(A::neg(s.ad_value(2078)), s.ad_value(2068)), s.ad_value(2069)), s.ad_value(2074)), 2067);
        }

        if ((s.v[1934] != 0.0) && (s.v[2055] != 0.0)) {
            s.store_mul_ad_lhs(2095, A::div(s.ad_value(2075), s.ad_value(2060)), 2085);
        }

        if ((s.v[1934] != 0.0) && (s.v[2055] != 0.0)) {
            let assign36090_ad_e33202: A = {
                if ((!(s.v[2095] > 50.0)) && (!(s.v[2095] < (-50.0)))) {
                    A::exp(s.ad_value(2095))
                } else {
                    {
                        if ((!(s.v[2095] > 50.0)) && (s.v[2095] < (-50.0))) {
                            A::exp(A::neg(A::constant(50.0)))
                        } else {
                            {
                                if (s.v[2095] > 50.0) {
                                    A::scale(A::offset(A::offset(s.ad_value(2095), (-50.0)), 1.0), ((50.0) as f64).exp())
                                } else {
                                    A::constant(0.0)
                                }
                            }
                        }
                    }
                }
            };
            s.store_ad(2096, &assign36090_ad_e33202);
        }

        if ((s.v[1934] != 0.0) && (s.v[2055] != 0.0)) {
            s.store_mul_ad_rhs(2086, 2058, A::offset(s.ad_value(2096), (-1.0)));
        }

        if ((s.v[1934] != 0.0) && (s.v[2055] != 0.0)) {
            s.store_add(2079, 2084, 2086);
        }

        if ((s.v[1934] != 0.0) && (s.v[2055] != 0.0)) {
            s.copy_ad(2056, 2079);
        }

        if ((s.v[1934] != 0.0) && (s.v[2055] != 0.0)) {
            s.copy_ad(134, 2056);
        }

        if ((s.v[1934] != 0.0) && (s.v[2055] != 0.0)) {
            s.store_scalar(2116, 0.0);
        }

        if ((s.v[1934] != 0.0) && (s.v[2055] != 0.0)) {
            s.store_scalar(2117, 0.0);
        }

        if ((s.v[1934] != 0.0) && (s.v[2055] != 0.0)) {
            s.store_scalar(2118, 0.0);
        }

        if ((s.v[1934] != 0.0) && (s.v[2055] != 0.0)) {
            s.store_ad(2119, &A::scale(A::voltage(ctx, &nodes, Some(8), Some(17)), p.p6));
        }

        if ((s.v[1934] != 0.0) && (s.v[2055] != 0.0)) {
            s.copy_ad(2120, 113);
        }

        if ((s.v[1934] != 0.0) && (s.v[2055] != 0.0)) {
            s.store_scalar(2121, p.p265);
        }

        if ((s.v[1934] != 0.0) && (s.v[2055] != 0.0)) {
            s.store_scalar(2122, p.p267);
        }

        if ((s.v[1934] != 0.0) && (s.v[2055] != 0.0)) {
            s.store_scalar(2123, 1.0);
        }

        if ((s.v[1934] != 0.0) && (s.v[2055] != 0.0)) {
            s.store_scalar(2124, p.p263);
        }

        if ((s.v[1934] != 0.0) && (s.v[2055] != 0.0)) {
            s.store_scalar(2125, p.p281);
        }

        if ((s.v[1934] != 0.0) && (s.v[2055] != 0.0)) {
            s.store_scalar(2126, p.p280);
        }

        if ((s.v[1934] != 0.0) && (s.v[2055] != 0.0)) {
            s.copy_ad(2127, 112);
        }

        if ((s.v[1934] != 0.0) && (s.v[2055] != 0.0)) {
            s.store_scalar(2128, p.p0);
        }

        if ((s.v[1934] != 0.0) && (s.v[2055] != 0.0)) {
            s.store_scalar(2129, p.p2);
        }

        if ((s.v[1934] != 0.0) && (s.v[2055] != 0.0)) {
            s.store_scalar(2130, 0.0);
        }

        if ((s.v[1934] != 0.0) && (s.v[2055] != 0.0)) {
            s.store_scalar(2131, 0.0);
        }

        if ((s.v[1934] != 0.0) && (s.v[2055] != 0.0)) {
            s.store_scalar(2132, p.p289);
        }

        if ((s.v[1934] != 0.0) && (s.v[2055] != 0.0)) {
            s.store_scalar(2133, p.p290);
        }

        if ((s.v[1934] != 0.0) && (s.v[2055] != 0.0)) {
            s.store_scalar(2134, ((1.0 - p.p255) * p.p288));
        }

        if ((s.v[1934] != 0.0) && (s.v[2055] != 0.0)) {
            s.store_scalar(2135, p.p287);
        }

        if ((s.v[1934] != 0.0) && (s.v[2055] != 0.0)) {
            s.store_scalar(2136, p.p257);
        }

        if ((s.v[1934] != 0.0) && (s.v[2055] != 0.0)) {
            s.store_scalar(2137, p.p256);
        }

        if ((s.v[1934] != 0.0) && (s.v[2055] != 0.0)) {
            s.store_scalar(2138, p.p6);
        }

        if ((s.v[1934] != 0.0) && (s.v[2055] != 0.0)) {
            s.store_scalar(2139, 0.0);
        }

        if ((s.v[1934] != 0.0) && (s.v[2055] != 0.0)) {
            s.store_scalar(2140, 0.0);
        }

        if ((s.v[1934] != 0.0) && (s.v[2055] != 0.0)) {
            s.store_scalar(2141, 0.0);
        }

        if ((s.v[1934] != 0.0) && (s.v[2055] != 0.0)) {
            s.store_scalar(2142, 0.0);
        }

        if ((s.v[1934] != 0.0) && (s.v[2055] != 0.0)) {
            s.store_scalar(2143, 0.0);
        }

        if ((s.v[1934] != 0.0) && (s.v[2055] != 0.0)) {
            s.store_scalar(2144, 0.0);
        }

        if ((s.v[1934] != 0.0) && (s.v[2055] != 0.0)) {
            s.store_scalar(2145, 0.0);
        }

        if ((s.v[1934] != 0.0) && (s.v[2055] != 0.0)) {
            s.store_scalar(2146, 0.0);
        }

        if ((s.v[1934] != 0.0) && (s.v[2055] != 0.0)) {
            s.store_scalar(2147, 0.0);
        }

        if ((s.v[1934] != 0.0) && (s.v[2055] != 0.0)) {
            s.store_scalar(2148, 0.0);
        }

        if ((s.v[1934] != 0.0) && (s.v[2055] != 0.0)) {
            s.store_scalar(2149, 0.0);
        }

        if ((s.v[1934] != 0.0) && (s.v[2055] != 0.0)) {
            s.store_scalar(2150, 0.0);
        }

        if ((s.v[1934] != 0.0) && (s.v[2055] != 0.0)) {
            s.store_scalar(2151, 0.0);
        }

        if ((s.v[1934] != 0.0) && (s.v[2055] != 0.0)) {
            s.store_scalar(2152, 0.0);
        }

        if ((s.v[1934] != 0.0) && (s.v[2055] != 0.0)) {
            s.store_scalar(2153, 0.0);
        }

        if ((s.v[1934] != 0.0) && (s.v[2055] != 0.0)) {
            s.store_scalar(2154, 0.0);
        }

        if ((s.v[1934] != 0.0) && (s.v[2055] != 0.0)) {
            s.store_scalar(2155, 0.0);
        }

        if ((s.v[1934] != 0.0) && (s.v[2055] != 0.0)) {
            s.store_scalar(2156, 0.0);
        }

        if ((s.v[1934] != 0.0) && (s.v[2055] != 0.0)) {
            s.store_scalar(2157, 0.0);
        }

        if ((s.v[1934] != 0.0) && (s.v[2055] != 0.0)) {
            s.store_scalar(2158, 0.0);
        }

        if ((s.v[1934] != 0.0) && (s.v[2055] != 0.0)) {
            s.store_scalar(2159, 0.0);
        }

        if ((s.v[1934] != 0.0) && (s.v[2055] != 0.0)) {
            s.store_scalar(2160, 0.0);
        }

        if ((s.v[1934] != 0.0) && (s.v[2055] != 0.0)) {
            s.store_scalar(2161, 0.0);
        }

        if ((s.v[1934] != 0.0) && (s.v[2055] != 0.0)) {
            s.store_scalar(2162, 0.0);
        }

        if ((s.v[1934] != 0.0) && (s.v[2055] != 0.0)) {
            s.store_scalar(2163, 0.0);
        }

        if ((s.v[1934] != 0.0) && (s.v[2055] != 0.0)) {
            s.store_scalar(2164, 0.0);
        }

        if ((s.v[1934] != 0.0) && (s.v[2055] != 0.0)) {
            s.store_scalar(2165, 0.0);
        }

        if ((s.v[1934] != 0.0) && (s.v[2055] != 0.0)) {
            s.store_scalar(2166, 0.0);
        }

        if ((s.v[1934] != 0.0) && (s.v[2055] != 0.0)) {
            s.store_scalar(2167, 0.0);
        }

        if ((s.v[1934] != 0.0) && (s.v[2055] != 0.0)) {
            s.store_scalar(2168, 0.0);
        }

        if ((s.v[1934] != 0.0) && (s.v[2055] != 0.0)) {
            s.store_scalar(2169, 0.0);
        }

        if ((s.v[1934] != 0.0) && (s.v[2055] != 0.0)) {
            s.store_scalar(2170, 0.0);
        }

        if ((s.v[1934] != 0.0) && (s.v[2055] != 0.0)) {
            s.store_scalar(2171, 0.0);
        }

        if ((s.v[1934] != 0.0) && (s.v[2055] != 0.0)) {
            s.store_mul_ad(2151, A::div(s.ad_value(2136), s.ad_value(2120)), A::neg(s.ad_value(2137)));
        }

        if ((s.v[1934] != 0.0) && (s.v[2055] != 0.0)) {
            let assign36730_ad_e33641: A = {
                if ((!(s.v[2151] > 50.0)) && (!(s.v[2151] < (-50.0)))) {
                    A::exp(s.ad_value(2151))
                } else {
                    {
                        if ((!(s.v[2151] > 50.0)) && (s.v[2151] < (-50.0))) {
                            A::exp(A::neg(A::constant(50.0)))
                        } else {
                            {
                                if (s.v[2151] > 50.0) {
                                    A::scale(A::offset(A::offset(s.ad_value(2151), (-50.0)), 1.0), ((50.0) as f64).exp())
                                } else {
                                    A::constant(0.0)
                                }
                            }
                        }
                    }
                }
            };
            s.store_ad(2141, &assign36730_ad_e33641);
        }

        if ((s.v[1934] != 0.0) && (s.v[2055] != 0.0)) {
            s.store_add_ad_lhs(2147, A::mul(s.ad_value(2125), A::sub(A::neg(s.ad_value(2119)), s.ad_value(2126))), 2151);
        }

        if ((s.v[1934] != 0.0) && (s.v[2055] != 0.0)) {
            s.store_add_ad_lhs(2148, A::mul(A::neg(s.ad_value(2125)), s.ad_value(2126)), 2151);
        }

    }

    pub(super) fn stamp_transient_block_28(
        &mut self,
        ctx: &GeneratedEvalContext<'_>,
        stamper: &mut GeneratedStamper<'_>,
        s: &mut Scratch,
    ) {
        let _ = stamper;
        let p = self.params;
        let nodes = self.nodes;
        let branches = self.branches;
        if ((s.v[1934] != 0.0) && (s.v[2055] != 0.0)) {
            let assign36760_ad_e33709: A = {
                if ((!(s.v[2147] > 50.0)) && (!(s.v[2147] < (-50.0)))) {
                    A::exp(s.ad_value(2147))
                } else {
                    {
                        if ((!(s.v[2147] > 50.0)) && (s.v[2147] < (-50.0))) {
                            A::exp(A::neg(A::constant(50.0)))
                        } else {
                            {
                                if (s.v[2147] > 50.0) {
                                    A::scale(A::offset(A::offset(s.ad_value(2147), (-50.0)), 1.0), ((50.0) as f64).exp())
                                } else {
                                    A::constant(0.0)
                                }
                            }
                        }
                    }
                }
            };
            s.store_ad(2149, &assign36760_ad_e33709);
        }

        if ((s.v[1934] != 0.0) && (s.v[2055] != 0.0)) {
            let assign36770_ad_e33753: A = {
                if ((!(s.v[2148] > 50.0)) && (!(s.v[2148] < (-50.0)))) {
                    A::exp(s.ad_value(2148))
                } else {
                    {
                        if ((!(s.v[2148] > 50.0)) && (s.v[2148] < (-50.0))) {
                            A::exp(A::neg(A::constant(50.0)))
                        } else {
                            {
                                if (s.v[2148] > 50.0) {
                                    A::scale(A::offset(A::offset(s.ad_value(2148), (-50.0)), 1.0), ((50.0) as f64).exp())
                                } else {
                                    A::constant(0.0)
                                }
                            }
                        }
                    }
                }
            };
            s.store_ad(2150, &assign36770_ad_e33753);
        }

        if ((s.v[1934] != 0.0) && (s.v[2055] != 0.0)) {
            s.store_sub(2143, 2149, 2150);
        }

        if ((s.v[1934] != 0.0) && (s.v[2055] != 0.0)) {
            s.store_mul_ad_lhs(2117, A::mul(A::mul(A::mul(s.ad_value(2138), s.ad_value(2128)), s.ad_value(2129)), s.ad_value(2130)), 2127);
        }

        if ((s.v[1934] != 0.0) && (s.v[2055] != 0.0)) {
            s.store_add_ad_lhs(2153, A::mul(A::div(s.ad_value(2124), s.ad_value(2120)), s.ad_value(2119)), 2151);
        }

        if ((s.v[1934] != 0.0) && (s.v[2055] != 0.0)) {
            let assign36810_ad_e33831: A = {
                if ((!(s.v[2153] > 50.0)) && (!(s.v[2153] < (-50.0)))) {
                    A::exp(s.ad_value(2153))
                } else {
                    {
                        if ((!(s.v[2153] > 50.0)) && (s.v[2153] < (-50.0))) {
                            A::exp(A::neg(A::constant(50.0)))
                        } else {
                            {
                                if (s.v[2153] > 50.0) {
                                    A::scale(A::offset(A::offset(s.ad_value(2153), (-50.0)), 1.0), ((50.0) as f64).exp())
                                } else {
                                    A::constant(0.0)
                                }
                            }
                        }
                    }
                }
            };
            s.store_ad(2154, &assign36810_ad_e33831);
        }

        s.v[2172] = if (s.v[2123] == 1.0) { 1.0 } else { 0.0 };

        if (((s.v[1934] != 0.0) && (s.v[2055] != 0.0)) && (s.v[2172] != 0.0)) {
            s.store_mul_ad_rhs(2144, 2117, A::sub(A::sub(s.ad_value(2154), A::mul(s.ad_value(2131), s.ad_value(2143))), s.ad_value(2141)));
        }

        if (((s.v[1934] != 0.0) && (s.v[2055] != 0.0)) && (!(s.v[2172] != 0.0))) {
            s.store_add_ad_lhs(2158, A::mul(s.ad_value(2125), A::sub(A::neg(s.ad_value(2121)), s.ad_value(2126))), 2151);
        }

        if (((s.v[1934] != 0.0) && (s.v[2055] != 0.0)) && (!(s.v[2172] != 0.0))) {
            let assign36850_ad_e33913: A = {
                if ((!(s.v[2158] > 50.0)) && (!(s.v[2158] < (-50.0)))) {
                    A::exp(s.ad_value(2158))
                } else {
                    {
                        if ((!(s.v[2158] > 50.0)) && (s.v[2158] < (-50.0))) {
                            A::exp(A::neg(A::constant(50.0)))
                        } else {
                            {
                                if (s.v[2158] > 50.0) {
                                    A::scale(A::offset(A::offset(s.ad_value(2158), (-50.0)), 1.0), ((50.0) as f64).exp())
                                } else {
                                    A::constant(0.0)
                                }
                            }
                        }
                    }
                }
            };
            s.store_ad(2159, &assign36850_ad_e33913);
        }

        if (((s.v[1934] != 0.0) && (s.v[2055] != 0.0)) && (!(s.v[2172] != 0.0))) {
            s.store_sub(2160, 2159, 2150);
        }

        if (((s.v[1934] != 0.0) && (s.v[2055] != 0.0)) && (!(s.v[2172] != 0.0))) {
            s.store_add_ad_lhs(2161, A::mul(A::div(s.ad_value(2124), s.ad_value(2120)), s.ad_value(2121)), 2151);
        }

        if (((s.v[1934] != 0.0) && (s.v[2055] != 0.0)) && (!(s.v[2172] != 0.0))) {
            let assign36880_ad_e33986: A = {
                if ((!(s.v[2161] > 50.0)) && (!(s.v[2161] < (-50.0)))) {
                    A::exp(s.ad_value(2161))
                } else {
                    {
                        if ((!(s.v[2161] > 50.0)) && (s.v[2161] < (-50.0))) {
                            A::exp(A::neg(A::constant(50.0)))
                        } else {
                            {
                                if (s.v[2161] > 50.0) {
                                    A::scale(A::offset(A::offset(s.ad_value(2161), (-50.0)), 1.0), ((50.0) as f64).exp())
                                } else {
                                    A::constant(0.0)
                                }
                            }
                        }
                    }
                }
            };
            s.store_ad(2162, &assign36880_ad_e33986);
        }

        if (((s.v[1934] != 0.0) && (s.v[2055] != 0.0)) && (!(s.v[2172] != 0.0))) {
            s.store_sub_ad_lhs(2163, A::sub(s.ad_value(2162), A::mul(s.ad_value(2131), s.ad_value(2160))), 2141);
        }

        if (((s.v[1934] != 0.0) && (s.v[2055] != 0.0)) && (!(s.v[2172] != 0.0))) {
            s.store_mul_ad_rhs(2164, 2117, A::sub(A::sub(s.ad_value(2154), A::mul(s.ad_value(2131), s.ad_value(2143))), s.ad_value(2141)));
        }

        s.v[2173] = if (s.v[2123] > 0.0) { 1.0 } else { 0.0 };

        if ((((s.v[1934] != 0.0) && (s.v[2055] != 0.0)) && (!(s.v[2172] != 0.0))) && (s.v[2173] != 0.0)) {
            s.store_mul(2157, 2123, 2124);
        }

        if ((((s.v[1934] != 0.0) && (s.v[2055] != 0.0)) && (!(s.v[2172] != 0.0))) && (s.v[2173] != 0.0)) {
            s.store_add_ad_lhs(2165, A::mul(A::div(s.ad_value(2157), s.ad_value(2120)), s.ad_value(2121)), 2151);
        }

        if ((((s.v[1934] != 0.0) && (s.v[2055] != 0.0)) && (!(s.v[2172] != 0.0))) && (s.v[2173] != 0.0)) {
            let assign36940_ad_e34100: A = {
                if ((!(s.v[2165] > 50.0)) && (!(s.v[2165] < (-50.0)))) {
                    A::exp(s.ad_value(2165))
                } else {
                    {
                        if ((!(s.v[2165] > 50.0)) && (s.v[2165] < (-50.0))) {
                            A::exp(A::neg(A::constant(50.0)))
                        } else {
                            {
                                if (s.v[2165] > 50.0) {
                                    A::scale(A::offset(A::offset(s.ad_value(2165), (-50.0)), 1.0), ((50.0) as f64).exp())
                                } else {
                                    A::constant(0.0)
                                }
                            }
                        }
                    }
                }
            };
            s.store_ad(2166, &assign36940_ad_e34100);
        }

        if ((((s.v[1934] != 0.0) && (s.v[2055] != 0.0)) && (!(s.v[2172] != 0.0))) && (s.v[2173] != 0.0)) {
            s.store_sub_ad_lhs(2167, A::sub(s.ad_value(2166), A::mul(s.ad_value(2131), s.ad_value(2160))), 2141);
        }

        if ((((s.v[1934] != 0.0) && (s.v[2055] != 0.0)) && (!(s.v[2172] != 0.0))) && (s.v[2173] != 0.0)) {
            s.store_add_ad_lhs(2168, A::mul(A::div(s.ad_value(2157), s.ad_value(2120)), s.ad_value(2119)), 2151);
        }

        if ((((s.v[1934] != 0.0) && (s.v[2055] != 0.0)) && (!(s.v[2172] != 0.0))) && (s.v[2173] != 0.0)) {
            let assign36970_ad_e34183: A = {
                if ((!(s.v[2168] > 50.0)) && (!(s.v[2168] < (-50.0)))) {
                    A::exp(s.ad_value(2168))
                } else {
                    {
                        if ((!(s.v[2168] > 50.0)) && (s.v[2168] < (-50.0))) {
                            A::exp(A::neg(A::constant(50.0)))
                        } else {
                            {
                                if (s.v[2168] > 50.0) {
                                    A::scale(A::offset(A::offset(s.ad_value(2168), (-50.0)), 1.0), ((50.0) as f64).exp())
                                } else {
                                    A::constant(0.0)
                                }
                            }
                        }
                    }
                }
            };
            s.store_ad(2169, &assign36970_ad_e34183);
        }

        if ((((s.v[1934] != 0.0) && (s.v[2055] != 0.0)) && (!(s.v[2172] != 0.0))) && (s.v[2173] != 0.0)) {
            s.store_div_ad_lhs(2170, A::mul(s.ad_value(2117), s.ad_value(2163)), 2167);
        }

        if ((((s.v[1934] != 0.0) && (s.v[2055] != 0.0)) && (!(s.v[2172] != 0.0))) && (s.v[2173] != 0.0)) {
            s.store_mul_ad_rhs(2171, 2170, A::sub(A::sub(s.ad_value(2169), A::mul(s.ad_value(2131), s.ad_value(2143))), s.ad_value(2141)));
        }

        if ((((s.v[1934] != 0.0) && (s.v[2055] != 0.0)) && (!(s.v[2172] != 0.0))) && (!(s.v[2173] != 0.0))) {
            s.store_mul(2171, 2117, 2163);
        }

        if (((s.v[1934] != 0.0) && (s.v[2055] != 0.0)) && (!(s.v[2172] != 0.0))) {
            s.store_mul_ad_lhs(2140, A::square(s.ad_value(2122)), 2120);
        }

        if (((s.v[1934] != 0.0) && (s.v[2055] != 0.0)) && (!(s.v[2172] != 0.0))) {
            s.store_div_ad_lhs(2152, A::sub(s.ad_value(2119), A::sub(s.ad_value(2121), A::scale(s.ad_value(2140), 0.5))), 2140);
        }

        s.v[2174] = if (s.v[2152] > 50.0) { 1.0 } else { 0.0 };

        if ((((s.v[1934] != 0.0) && (s.v[2055] != 0.0)) && (!(s.v[2172] != 0.0))) && (s.v[2174] != 0.0)) {
            s.store_scalar(2142, 0.0);
        }

        s.v[2175] = if (s.v[2152] < (-50.0)) { 1.0 } else { 0.0 };

        if (((((s.v[1934] != 0.0) && (s.v[2055] != 0.0)) && (!(s.v[2172] != 0.0))) && (!(s.v[2174] != 0.0))) && (s.v[2175] != 0.0)) {
            s.store_scalar(2142, 1.0);
        }

        if (((((s.v[1934] != 0.0) && (s.v[2055] != 0.0)) && (!(s.v[2172] != 0.0))) && (!(s.v[2174] != 0.0))) && (!(s.v[2175] != 0.0))) {
            s.store_div_from_scalar_ad(2142, 1.0, A::offset(A::exp(s.ad_value(2152)), 1.0));
        }

        if (((s.v[1934] != 0.0) && (s.v[2055] != 0.0)) && (!(s.v[2172] != 0.0))) {
            s.store_add_ad(2144, A::mul(s.ad_value(2142), s.ad_value(2164)), A::mul(A::sub_from_scalar(1.0, s.ad_value(2142)), s.ad_value(2171)));
        }

        if ((s.v[1934] != 0.0) && (s.v[2055] != 0.0)) {
            let assign37090_ad_e34370: A = {
                if (!(p.p52 == 0.0)) {
                    A::mul(A::div(s.ad_value(2119), s.ad_value(2132)), A::tanh(A::scale(A::div(s.ad_value(2119), s.ad_value(2132)), (0.001 / p.p53))))
                } else {
                    {
                        if (p.p52 == 0.0) {
                            A::sqrt(A::offset(A::mul(A::div(s.ad_value(2119), s.ad_value(2132)), A::div(s.ad_value(2119), s.ad_value(2132))), p.p53))
                        } else {
                            A::constant(0.0)
                        }
                    }
                }
            };
            s.store_div_ad(2145, A::neg(s.ad_value(2119)), A::pow(A::offset(A::pow(assign37090_ad_e34370, s.ad_value(2133)), 1.0), A::div_from_scalar(1.0, s.ad_value(2133))));
        }

        if ((s.v[1934] != 0.0) && (s.v[2055] != 0.0)) {
            s.store_mul_ad_lhs(2118, A::mul(A::mul(A::mul(A::neg(s.ad_value(2138)), s.ad_value(2128)), s.ad_value(2129)), s.ad_value(2134)), 2127);
        }

        if ((s.v[1934] != 0.0) && (s.v[2055] != 0.0)) {
            s.store_mul_ad_lhs(2155, A::div(s.ad_value(2135), s.ad_value(2120)), 2145);
        }

        if ((s.v[1934] != 0.0) && (s.v[2055] != 0.0)) {
            let assign37120_ad_e34449: A = {
                if ((!(s.v[2155] > 50.0)) && (!(s.v[2155] < (-50.0)))) {
                    A::exp(s.ad_value(2155))
                } else {
                    {
                        if ((!(s.v[2155] > 50.0)) && (s.v[2155] < (-50.0))) {
                            A::exp(A::neg(A::constant(50.0)))
                        } else {
                            {
                                if (s.v[2155] > 50.0) {
                                    A::scale(A::offset(A::offset(s.ad_value(2155), (-50.0)), 1.0), ((50.0) as f64).exp())
                                } else {
                                    A::constant(0.0)
                                }
                            }
                        }
                    }
                }
            };
            s.store_ad(2156, &assign37120_ad_e34449);
        }

        if ((s.v[1934] != 0.0) && (s.v[2055] != 0.0)) {
            s.store_mul_ad_rhs(2146, 2118, A::offset(s.ad_value(2156), (-1.0)));
        }

        if ((s.v[1934] != 0.0) && (s.v[2055] != 0.0)) {
            s.store_add(2139, 2144, 2146);
        }

        if ((s.v[1934] != 0.0) && (s.v[2055] != 0.0)) {
            s.copy_ad(2116, 2139);
        }

        if ((s.v[1934] != 0.0) && (s.v[2055] != 0.0)) {
            s.copy_ad(135, 2116);
        }

        s.v[2176] = if (p.p255 != 0.0) { 1.0 } else { 0.0 };

        if ((s.v[1934] != 0.0) && (s.v[2176] != 0.0)) {
            s.store_scalar(2177, 0.0);
        }

        if ((s.v[1934] != 0.0) && (s.v[2176] != 0.0)) {
            s.store_scalar(2178, 0.0);
        }

        if ((s.v[1934] != 0.0) && (s.v[2176] != 0.0)) {
            s.store_scalar(2179, 0.0);
        }

        if ((s.v[1934] != 0.0) && (s.v[2176] != 0.0)) {
            s.store_ad(2180, &A::scale(A::voltage(ctx, &nodes, Some(8), Some(9)), p.p6));
        }

        if ((s.v[1934] != 0.0) && (s.v[2176] != 0.0)) {
            s.copy_ad(2181, 113);
        }

        if ((s.v[1934] != 0.0) && (s.v[2176] != 0.0)) {
            s.store_scalar(2182, p.p260);
        }

        if ((s.v[1934] != 0.0) && (s.v[2176] != 0.0)) {
            s.store_scalar(2183, p.p262);
        }

        if ((s.v[1934] != 0.0) && (s.v[2176] != 0.0)) {
            s.store_scalar(2184, p.p261);
        }

        if ((s.v[1934] != 0.0) && (s.v[2176] != 0.0)) {
            s.store_scalar(2185, p.p258);
        }

        if ((s.v[1934] != 0.0) && (s.v[2176] != 0.0)) {
            s.store_scalar(2186, p.p278);
        }

        if ((s.v[1934] != 0.0) && (s.v[2176] != 0.0)) {
            s.store_scalar(2187, p.p277);
        }

        if ((s.v[1934] != 0.0) && (s.v[2176] != 0.0)) {
            s.copy_ad(2188, 112);
        }

        if ((s.v[1934] != 0.0) && (s.v[2176] != 0.0)) {
            s.store_scalar(2189, p.p0);
        }

        if ((s.v[1934] != 0.0) && (s.v[2176] != 0.0)) {
            s.store_scalar(2190, p.p2);
        }

        if ((s.v[1934] != 0.0) && (s.v[2176] != 0.0)) {
            s.store_scalar(2191, (p.p255 * p.p259));
        }

        if ((s.v[1934] != 0.0) && (s.v[2176] != 0.0)) {
            s.store_scalar(2192, p.p276);
        }

        if ((s.v[1934] != 0.0) && (s.v[2176] != 0.0)) {
            s.store_scalar(2193, p.p270);
        }

        if ((s.v[1934] != 0.0) && (s.v[2176] != 0.0)) {
            s.store_scalar(2194, p.p271);
        }

        if ((s.v[1934] != 0.0) && (s.v[2176] != 0.0)) {
            s.store_scalar(2195, (p.p255 * p.p269));
        }

        if ((s.v[1934] != 0.0) && (s.v[2176] != 0.0)) {
            s.store_scalar(2196, p.p268);
        }

        if ((s.v[1934] != 0.0) && (s.v[2176] != 0.0)) {
            s.store_scalar(2197, p.p257);
        }

        if ((s.v[1934] != 0.0) && (s.v[2176] != 0.0)) {
            s.store_scalar(2198, p.p256);
        }

        if ((s.v[1934] != 0.0) && (s.v[2176] != 0.0)) {
            s.store_scalar(2199, p.p6);
        }

        if ((s.v[1934] != 0.0) && (s.v[2176] != 0.0)) {
            s.store_scalar(2200, 0.0);
        }

        if ((s.v[1934] != 0.0) && (s.v[2176] != 0.0)) {
            s.store_scalar(2201, 0.0);
        }

        if ((s.v[1934] != 0.0) && (s.v[2176] != 0.0)) {
            s.store_scalar(2202, 0.0);
        }

        if ((s.v[1934] != 0.0) && (s.v[2176] != 0.0)) {
            s.store_scalar(2203, 0.0);
        }

        if ((s.v[1934] != 0.0) && (s.v[2176] != 0.0)) {
            s.store_scalar(2204, 0.0);
        }

        if ((s.v[1934] != 0.0) && (s.v[2176] != 0.0)) {
            s.store_scalar(2205, 0.0);
        }

        if ((s.v[1934] != 0.0) && (s.v[2176] != 0.0)) {
            s.store_scalar(2206, 0.0);
        }

        if ((s.v[1934] != 0.0) && (s.v[2176] != 0.0)) {
            s.store_scalar(2207, 0.0);
        }

        if ((s.v[1934] != 0.0) && (s.v[2176] != 0.0)) {
            s.store_scalar(2208, 0.0);
        }

        if ((s.v[1934] != 0.0) && (s.v[2176] != 0.0)) {
            s.store_scalar(2209, 0.0);
        }

        if ((s.v[1934] != 0.0) && (s.v[2176] != 0.0)) {
            s.store_scalar(2210, 0.0);
        }

        if ((s.v[1934] != 0.0) && (s.v[2176] != 0.0)) {
            s.store_scalar(2211, 0.0);
        }

        if ((s.v[1934] != 0.0) && (s.v[2176] != 0.0)) {
            s.store_scalar(2212, 0.0);
        }

        if ((s.v[1934] != 0.0) && (s.v[2176] != 0.0)) {
            s.store_scalar(2213, 0.0);
        }

        if ((s.v[1934] != 0.0) && (s.v[2176] != 0.0)) {
            s.store_scalar(2214, 0.0);
        }

        if ((s.v[1934] != 0.0) && (s.v[2176] != 0.0)) {
            s.store_scalar(2215, 0.0);
        }

        if ((s.v[1934] != 0.0) && (s.v[2176] != 0.0)) {
            s.store_scalar(2216, 0.0);
        }

        if ((s.v[1934] != 0.0) && (s.v[2176] != 0.0)) {
            s.store_scalar(2217, 0.0);
        }

        if ((s.v[1934] != 0.0) && (s.v[2176] != 0.0)) {
            s.store_scalar(2218, 0.0);
        }

        if ((s.v[1934] != 0.0) && (s.v[2176] != 0.0)) {
            s.store_scalar(2219, 0.0);
        }

        if ((s.v[1934] != 0.0) && (s.v[2176] != 0.0)) {
            s.store_scalar(2220, 0.0);
        }

        if ((s.v[1934] != 0.0) && (s.v[2176] != 0.0)) {
            s.store_scalar(2221, 0.0);
        }

        if ((s.v[1934] != 0.0) && (s.v[2176] != 0.0)) {
            s.store_scalar(2222, 0.0);
        }

        if ((s.v[1934] != 0.0) && (s.v[2176] != 0.0)) {
            s.store_scalar(2223, 0.0);
        }

    }

    pub(super) fn stamp_transient_block_29(
        &mut self,
        ctx: &GeneratedEvalContext<'_>,
        stamper: &mut GeneratedStamper<'_>,
        s: &mut Scratch,
    ) {
        let _ = stamper;
        let p = self.params;
        let nodes = self.nodes;
        let branches = self.branches;
        if ((s.v[1934] != 0.0) && (s.v[2176] != 0.0)) {
            s.store_scalar(2224, 0.0);
        }

        if ((s.v[1934] != 0.0) && (s.v[2176] != 0.0)) {
            s.store_scalar(2225, 0.0);
        }

        if ((s.v[1934] != 0.0) && (s.v[2176] != 0.0)) {
            s.store_scalar(2226, 0.0);
        }

        if ((s.v[1934] != 0.0) && (s.v[2176] != 0.0)) {
            s.store_scalar(2227, 0.0);
        }

        if ((s.v[1934] != 0.0) && (s.v[2176] != 0.0)) {
            s.store_scalar(2228, 0.0);
        }

        if ((s.v[1934] != 0.0) && (s.v[2176] != 0.0)) {
            s.store_scalar(2229, 0.0);
        }

        if ((s.v[1934] != 0.0) && (s.v[2176] != 0.0)) {
            s.store_scalar(2230, 0.0);
        }

        if ((s.v[1934] != 0.0) && (s.v[2176] != 0.0)) {
            s.store_scalar(2231, 0.0);
        }

        if ((s.v[1934] != 0.0) && (s.v[2176] != 0.0)) {
            s.store_scalar(2232, 0.0);
        }

        if ((s.v[1934] != 0.0) && (s.v[2176] != 0.0)) {
            s.store_mul_ad(2212, A::div(s.ad_value(2197), s.ad_value(2181)), A::neg(s.ad_value(2198)));
        }

        if ((s.v[1934] != 0.0) && (s.v[2176] != 0.0)) {
            let assign37770_ad_e34891: A = {
                if ((!(s.v[2212] > 50.0)) && (!(s.v[2212] < (-50.0)))) {
                    A::exp(s.ad_value(2212))
                } else {
                    {
                        if ((!(s.v[2212] > 50.0)) && (s.v[2212] < (-50.0))) {
                            A::exp(A::neg(A::constant(50.0)))
                        } else {
                            {
                                if (s.v[2212] > 50.0) {
                                    A::scale(A::offset(A::offset(s.ad_value(2212), (-50.0)), 1.0), ((50.0) as f64).exp())
                                } else {
                                    A::constant(0.0)
                                }
                            }
                        }
                    }
                }
            };
            s.store_ad(2202, &assign37770_ad_e34891);
        }

        if ((s.v[1934] != 0.0) && (s.v[2176] != 0.0)) {
            s.store_add_ad_lhs(2208, A::mul(s.ad_value(2186), A::sub(A::neg(s.ad_value(2180)), s.ad_value(2187))), 2212);
        }

        if ((s.v[1934] != 0.0) && (s.v[2176] != 0.0)) {
            s.store_add_ad_lhs(2209, A::mul(A::neg(s.ad_value(2186)), s.ad_value(2187)), 2212);
        }

        if ((s.v[1934] != 0.0) && (s.v[2176] != 0.0)) {
            let assign37800_ad_e34959: A = {
                if ((!(s.v[2208] > 50.0)) && (!(s.v[2208] < (-50.0)))) {
                    A::exp(s.ad_value(2208))
                } else {
                    {
                        if ((!(s.v[2208] > 50.0)) && (s.v[2208] < (-50.0))) {
                            A::exp(A::neg(A::constant(50.0)))
                        } else {
                            {
                                if (s.v[2208] > 50.0) {
                                    A::scale(A::offset(A::offset(s.ad_value(2208), (-50.0)), 1.0), ((50.0) as f64).exp())
                                } else {
                                    A::constant(0.0)
                                }
                            }
                        }
                    }
                }
            };
            s.store_ad(2210, &assign37800_ad_e34959);
        }

        if ((s.v[1934] != 0.0) && (s.v[2176] != 0.0)) {
            let assign37810_ad_e35003: A = {
                if ((!(s.v[2209] > 50.0)) && (!(s.v[2209] < (-50.0)))) {
                    A::exp(s.ad_value(2209))
                } else {
                    {
                        if ((!(s.v[2209] > 50.0)) && (s.v[2209] < (-50.0))) {
                            A::exp(A::neg(A::constant(50.0)))
                        } else {
                            {
                                if (s.v[2209] > 50.0) {
                                    A::scale(A::offset(A::offset(s.ad_value(2209), (-50.0)), 1.0), ((50.0) as f64).exp())
                                } else {
                                    A::constant(0.0)
                                }
                            }
                        }
                    }
                }
            };
            s.store_ad(2211, &assign37810_ad_e35003);
        }

        if ((s.v[1934] != 0.0) && (s.v[2176] != 0.0)) {
            s.store_sub(2204, 2210, 2211);
        }

        if ((s.v[1934] != 0.0) && (s.v[2176] != 0.0)) {
            s.store_mul_ad_lhs(2178, A::mul(A::mul(A::mul(s.ad_value(2199), s.ad_value(2189)), s.ad_value(2190)), s.ad_value(2191)), 2188);
        }

        if ((s.v[1934] != 0.0) && (s.v[2176] != 0.0)) {
            s.store_add_ad_lhs(2214, A::mul(A::div(s.ad_value(2185), s.ad_value(2181)), s.ad_value(2180)), 2212);
        }

        if ((s.v[1934] != 0.0) && (s.v[2176] != 0.0)) {
            let assign37850_ad_e35081: A = {
                if ((!(s.v[2214] > 50.0)) && (!(s.v[2214] < (-50.0)))) {
                    A::exp(s.ad_value(2214))
                } else {
                    {
                        if ((!(s.v[2214] > 50.0)) && (s.v[2214] < (-50.0))) {
                            A::exp(A::neg(A::constant(50.0)))
                        } else {
                            {
                                if (s.v[2214] > 50.0) {
                                    A::scale(A::offset(A::offset(s.ad_value(2214), (-50.0)), 1.0), ((50.0) as f64).exp())
                                } else {
                                    A::constant(0.0)
                                }
                            }
                        }
                    }
                }
            };
            s.store_ad(2215, &assign37850_ad_e35081);
        }

        s.v[2233] = if (s.v[2184] == 1.0) { 1.0 } else { 0.0 };

        if (((s.v[1934] != 0.0) && (s.v[2176] != 0.0)) && (s.v[2233] != 0.0)) {
            s.store_mul_ad_rhs(2205, 2178, A::sub(A::sub(s.ad_value(2215), A::mul(s.ad_value(2192), s.ad_value(2204))), s.ad_value(2202)));
        }

        if (((s.v[1934] != 0.0) && (s.v[2176] != 0.0)) && (!(s.v[2233] != 0.0))) {
            s.store_add_ad_lhs(2219, A::mul(s.ad_value(2186), A::sub(A::neg(s.ad_value(2182)), s.ad_value(2187))), 2212);
        }

        if (((s.v[1934] != 0.0) && (s.v[2176] != 0.0)) && (!(s.v[2233] != 0.0))) {
            let assign37890_ad_e35163: A = {
                if ((!(s.v[2219] > 50.0)) && (!(s.v[2219] < (-50.0)))) {
                    A::exp(s.ad_value(2219))
                } else {
                    {
                        if ((!(s.v[2219] > 50.0)) && (s.v[2219] < (-50.0))) {
                            A::exp(A::neg(A::constant(50.0)))
                        } else {
                            {
                                if (s.v[2219] > 50.0) {
                                    A::scale(A::offset(A::offset(s.ad_value(2219), (-50.0)), 1.0), ((50.0) as f64).exp())
                                } else {
                                    A::constant(0.0)
                                }
                            }
                        }
                    }
                }
            };
            s.store_ad(2220, &assign37890_ad_e35163);
        }

        if (((s.v[1934] != 0.0) && (s.v[2176] != 0.0)) && (!(s.v[2233] != 0.0))) {
            s.store_sub(2221, 2220, 2211);
        }

        if (((s.v[1934] != 0.0) && (s.v[2176] != 0.0)) && (!(s.v[2233] != 0.0))) {
            s.store_add_ad_lhs(2222, A::mul(A::div(s.ad_value(2185), s.ad_value(2181)), s.ad_value(2182)), 2212);
        }

        if (((s.v[1934] != 0.0) && (s.v[2176] != 0.0)) && (!(s.v[2233] != 0.0))) {
            let assign37920_ad_e35236: A = {
                if ((!(s.v[2222] > 50.0)) && (!(s.v[2222] < (-50.0)))) {
                    A::exp(s.ad_value(2222))
                } else {
                    {
                        if ((!(s.v[2222] > 50.0)) && (s.v[2222] < (-50.0))) {
                            A::exp(A::neg(A::constant(50.0)))
                        } else {
                            {
                                if (s.v[2222] > 50.0) {
                                    A::scale(A::offset(A::offset(s.ad_value(2222), (-50.0)), 1.0), ((50.0) as f64).exp())
                                } else {
                                    A::constant(0.0)
                                }
                            }
                        }
                    }
                }
            };
            s.store_ad(2223, &assign37920_ad_e35236);
        }

        if (((s.v[1934] != 0.0) && (s.v[2176] != 0.0)) && (!(s.v[2233] != 0.0))) {
            s.store_sub_ad_lhs(2224, A::sub(s.ad_value(2223), A::mul(s.ad_value(2192), s.ad_value(2221))), 2202);
        }

        if (((s.v[1934] != 0.0) && (s.v[2176] != 0.0)) && (!(s.v[2233] != 0.0))) {
            s.store_mul_ad_rhs(2225, 2178, A::sub(A::sub(s.ad_value(2215), A::mul(s.ad_value(2192), s.ad_value(2204))), s.ad_value(2202)));
        }

        s.v[2234] = if (s.v[2184] > 0.0) { 1.0 } else { 0.0 };

        if ((((s.v[1934] != 0.0) && (s.v[2176] != 0.0)) && (!(s.v[2233] != 0.0))) && (s.v[2234] != 0.0)) {
            s.store_mul(2218, 2184, 2185);
        }

        if ((((s.v[1934] != 0.0) && (s.v[2176] != 0.0)) && (!(s.v[2233] != 0.0))) && (s.v[2234] != 0.0)) {
            s.store_add_ad_lhs(2226, A::mul(A::div(s.ad_value(2218), s.ad_value(2181)), s.ad_value(2182)), 2212);
        }

        if ((((s.v[1934] != 0.0) && (s.v[2176] != 0.0)) && (!(s.v[2233] != 0.0))) && (s.v[2234] != 0.0)) {
            let assign37980_ad_e35350: A = {
                if ((!(s.v[2226] > 50.0)) && (!(s.v[2226] < (-50.0)))) {
                    A::exp(s.ad_value(2226))
                } else {
                    {
                        if ((!(s.v[2226] > 50.0)) && (s.v[2226] < (-50.0))) {
                            A::exp(A::neg(A::constant(50.0)))
                        } else {
                            {
                                if (s.v[2226] > 50.0) {
                                    A::scale(A::offset(A::offset(s.ad_value(2226), (-50.0)), 1.0), ((50.0) as f64).exp())
                                } else {
                                    A::constant(0.0)
                                }
                            }
                        }
                    }
                }
            };
            s.store_ad(2227, &assign37980_ad_e35350);
        }

        if ((((s.v[1934] != 0.0) && (s.v[2176] != 0.0)) && (!(s.v[2233] != 0.0))) && (s.v[2234] != 0.0)) {
            s.store_sub_ad_lhs(2228, A::sub(s.ad_value(2227), A::mul(s.ad_value(2192), s.ad_value(2221))), 2202);
        }

        if ((((s.v[1934] != 0.0) && (s.v[2176] != 0.0)) && (!(s.v[2233] != 0.0))) && (s.v[2234] != 0.0)) {
            s.store_add_ad_lhs(2229, A::mul(A::div(s.ad_value(2218), s.ad_value(2181)), s.ad_value(2180)), 2212);
        }

        if ((((s.v[1934] != 0.0) && (s.v[2176] != 0.0)) && (!(s.v[2233] != 0.0))) && (s.v[2234] != 0.0)) {
            let assign38010_ad_e35433: A = {
                if ((!(s.v[2229] > 50.0)) && (!(s.v[2229] < (-50.0)))) {
                    A::exp(s.ad_value(2229))
                } else {
                    {
                        if ((!(s.v[2229] > 50.0)) && (s.v[2229] < (-50.0))) {
                            A::exp(A::neg(A::constant(50.0)))
                        } else {
                            {
                                if (s.v[2229] > 50.0) {
                                    A::scale(A::offset(A::offset(s.ad_value(2229), (-50.0)), 1.0), ((50.0) as f64).exp())
                                } else {
                                    A::constant(0.0)
                                }
                            }
                        }
                    }
                }
            };
            s.store_ad(2230, &assign38010_ad_e35433);
        }

        if ((((s.v[1934] != 0.0) && (s.v[2176] != 0.0)) && (!(s.v[2233] != 0.0))) && (s.v[2234] != 0.0)) {
            s.store_div_ad_lhs(2231, A::mul(s.ad_value(2178), s.ad_value(2224)), 2228);
        }

        if ((((s.v[1934] != 0.0) && (s.v[2176] != 0.0)) && (!(s.v[2233] != 0.0))) && (s.v[2234] != 0.0)) {
            s.store_mul_ad_rhs(2232, 2231, A::sub(A::sub(s.ad_value(2230), A::mul(s.ad_value(2192), s.ad_value(2204))), s.ad_value(2202)));
        }

        if ((((s.v[1934] != 0.0) && (s.v[2176] != 0.0)) && (!(s.v[2233] != 0.0))) && (!(s.v[2234] != 0.0))) {
            s.store_mul(2232, 2178, 2224);
        }

        if (((s.v[1934] != 0.0) && (s.v[2176] != 0.0)) && (!(s.v[2233] != 0.0))) {
            s.store_mul_ad_lhs(2201, A::square(s.ad_value(2183)), 2181);
        }

        if (((s.v[1934] != 0.0) && (s.v[2176] != 0.0)) && (!(s.v[2233] != 0.0))) {
            s.store_div_ad_lhs(2213, A::sub(s.ad_value(2180), A::sub(s.ad_value(2182), A::scale(s.ad_value(2201), 0.5))), 2201);
        }

        s.v[2235] = if (s.v[2213] > 50.0) { 1.0 } else { 0.0 };

        if ((((s.v[1934] != 0.0) && (s.v[2176] != 0.0)) && (!(s.v[2233] != 0.0))) && (s.v[2235] != 0.0)) {
            s.store_scalar(2203, 0.0);
        }

        s.v[2236] = if (s.v[2213] < (-50.0)) { 1.0 } else { 0.0 };

        if (((((s.v[1934] != 0.0) && (s.v[2176] != 0.0)) && (!(s.v[2233] != 0.0))) && (!(s.v[2235] != 0.0))) && (s.v[2236] != 0.0)) {
            s.store_scalar(2203, 1.0);
        }

        if (((((s.v[1934] != 0.0) && (s.v[2176] != 0.0)) && (!(s.v[2233] != 0.0))) && (!(s.v[2235] != 0.0))) && (!(s.v[2236] != 0.0))) {
            s.store_div_from_scalar_ad(2203, 1.0, A::offset(A::exp(s.ad_value(2213)), 1.0));
        }

        if (((s.v[1934] != 0.0) && (s.v[2176] != 0.0)) && (!(s.v[2233] != 0.0))) {
            s.store_add_ad(2205, A::mul(s.ad_value(2203), s.ad_value(2225)), A::mul(A::sub_from_scalar(1.0, s.ad_value(2203)), s.ad_value(2232)));
        }

        if ((s.v[1934] != 0.0) && (s.v[2176] != 0.0)) {
            let assign38130_ad_e35620: A = {
                if (!(p.p52 == 0.0)) {
                    A::mul(A::div(s.ad_value(2180), s.ad_value(2193)), A::tanh(A::scale(A::div(s.ad_value(2180), s.ad_value(2193)), (0.001 / p.p53))))
                } else {
                    {
                        if (p.p52 == 0.0) {
                            A::sqrt(A::offset(A::mul(A::div(s.ad_value(2180), s.ad_value(2193)), A::div(s.ad_value(2180), s.ad_value(2193))), p.p53))
                        } else {
                            A::constant(0.0)
                        }
                    }
                }
            };
            s.store_div_ad(2206, A::neg(s.ad_value(2180)), A::pow(A::offset(A::pow(assign38130_ad_e35620, s.ad_value(2194)), 1.0), A::div_from_scalar(1.0, s.ad_value(2194))));
        }

        if ((s.v[1934] != 0.0) && (s.v[2176] != 0.0)) {
            s.store_mul_ad_lhs(2179, A::mul(A::mul(A::mul(A::neg(s.ad_value(2199)), s.ad_value(2189)), s.ad_value(2190)), s.ad_value(2195)), 2188);
        }

        if ((s.v[1934] != 0.0) && (s.v[2176] != 0.0)) {
            s.store_mul_ad_lhs(2216, A::div(s.ad_value(2196), s.ad_value(2181)), 2206);
        }

        if ((s.v[1934] != 0.0) && (s.v[2176] != 0.0)) {
            let assign38160_ad_e35699: A = {
                if ((!(s.v[2216] > 50.0)) && (!(s.v[2216] < (-50.0)))) {
                    A::exp(s.ad_value(2216))
                } else {
                    {
                        if ((!(s.v[2216] > 50.0)) && (s.v[2216] < (-50.0))) {
                            A::exp(A::neg(A::constant(50.0)))
                        } else {
                            {
                                if (s.v[2216] > 50.0) {
                                    A::scale(A::offset(A::offset(s.ad_value(2216), (-50.0)), 1.0), ((50.0) as f64).exp())
                                } else {
                                    A::constant(0.0)
                                }
                            }
                        }
                    }
                }
            };
            s.store_ad(2217, &assign38160_ad_e35699);
        }

        if ((s.v[1934] != 0.0) && (s.v[2176] != 0.0)) {
            s.store_mul_ad_rhs(2207, 2179, A::offset(s.ad_value(2217), (-1.0)));
        }

        if ((s.v[1934] != 0.0) && (s.v[2176] != 0.0)) {
            s.store_add(2200, 2205, 2207);
        }

        if ((s.v[1934] != 0.0) && (s.v[2176] != 0.0)) {
            s.copy_ad(2177, 2200);
        }

        if ((s.v[1934] != 0.0) && (s.v[2176] != 0.0)) {
            s.copy_ad(130, 2178);
        }

        if ((s.v[1934] != 0.0) && (s.v[2176] != 0.0)) {
            s.copy_ad(132, 2179);
        }

        if ((s.v[1934] != 0.0) && (s.v[2176] != 0.0)) {
            s.copy_ad(128, 2177);
        }

        if ((s.v[1934] != 0.0) && (s.v[2176] != 0.0)) {
            s.store_scalar(2237, 0.0);
        }

        if ((s.v[1934] != 0.0) && (s.v[2176] != 0.0)) {
            s.store_scalar(2238, 0.0);
        }

        if ((s.v[1934] != 0.0) && (s.v[2176] != 0.0)) {
            s.store_scalar(2239, 0.0);
        }

        if ((s.v[1934] != 0.0) && (s.v[2176] != 0.0)) {
            s.store_ad(2240, &A::scale(A::voltage(ctx, &nodes, Some(8), Some(5)), p.p6));
        }

        if ((s.v[1934] != 0.0) && (s.v[2176] != 0.0)) {
            s.copy_ad(2241, 113);
        }

        if ((s.v[1934] != 0.0) && (s.v[2176] != 0.0)) {
            s.store_scalar(2242, p.p265);
        }

        if ((s.v[1934] != 0.0) && (s.v[2176] != 0.0)) {
            s.store_scalar(2243, p.p267);
        }

        if ((s.v[1934] != 0.0) && (s.v[2176] != 0.0)) {
            s.store_scalar(2244, p.p266);
        }

        if ((s.v[1934] != 0.0) && (s.v[2176] != 0.0)) {
            s.store_scalar(2245, p.p263);
        }

        if ((s.v[1934] != 0.0) && (s.v[2176] != 0.0)) {
            s.store_scalar(2246, p.p281);
        }

        if ((s.v[1934] != 0.0) && (s.v[2176] != 0.0)) {
            s.store_scalar(2247, p.p280);
        }

        if ((s.v[1934] != 0.0) && (s.v[2176] != 0.0)) {
            s.copy_ad(2248, 112);
        }

        if ((s.v[1934] != 0.0) && (s.v[2176] != 0.0)) {
            s.store_scalar(2249, p.p0);
        }

        if ((s.v[1934] != 0.0) && (s.v[2176] != 0.0)) {
            s.store_scalar(2250, p.p2);
        }

        if ((s.v[1934] != 0.0) && (s.v[2176] != 0.0)) {
            s.store_scalar(2251, (p.p255 * p.p264));
        }

        if ((s.v[1934] != 0.0) && (s.v[2176] != 0.0)) {
            s.store_scalar(2252, p.p279);
        }

        if ((s.v[1934] != 0.0) && (s.v[2176] != 0.0)) {
            s.store_scalar(2253, p.p274);
        }

        if ((s.v[1934] != 0.0) && (s.v[2176] != 0.0)) {
            s.store_scalar(2254, p.p275);
        }

        if ((s.v[1934] != 0.0) && (s.v[2176] != 0.0)) {
            s.store_scalar(2255, (p.p255 * p.p273));
        }

        if ((s.v[1934] != 0.0) && (s.v[2176] != 0.0)) {
            s.store_scalar(2256, p.p272);
        }

        if ((s.v[1934] != 0.0) && (s.v[2176] != 0.0)) {
            s.store_scalar(2257, p.p257);
        }

        if ((s.v[1934] != 0.0) && (s.v[2176] != 0.0)) {
            s.store_scalar(2258, p.p256);
        }

        if ((s.v[1934] != 0.0) && (s.v[2176] != 0.0)) {
            s.store_scalar(2259, p.p6);
        }

        if ((s.v[1934] != 0.0) && (s.v[2176] != 0.0)) {
            s.store_scalar(2260, 0.0);
        }

        if ((s.v[1934] != 0.0) && (s.v[2176] != 0.0)) {
            s.store_scalar(2261, 0.0);
        }

        if ((s.v[1934] != 0.0) && (s.v[2176] != 0.0)) {
            s.store_scalar(2262, 0.0);
        }

        if ((s.v[1934] != 0.0) && (s.v[2176] != 0.0)) {
            s.store_scalar(2263, 0.0);
        }

        if ((s.v[1934] != 0.0) && (s.v[2176] != 0.0)) {
            s.store_scalar(2264, 0.0);
        }

    }

    pub(super) fn stamp_transient_block_30(
        &mut self,
        ctx: &GeneratedEvalContext<'_>,
        stamper: &mut GeneratedStamper<'_>,
        s: &mut Scratch,
    ) {
        let _ = stamper;
        let p = self.params;
        let nodes = self.nodes;
        let branches = self.branches;
        if ((s.v[1934] != 0.0) && (s.v[2176] != 0.0)) {
            s.store_scalar(2265, 0.0);
        }

        if ((s.v[1934] != 0.0) && (s.v[2176] != 0.0)) {
            s.store_scalar(2266, 0.0);
        }

        if ((s.v[1934] != 0.0) && (s.v[2176] != 0.0)) {
            s.store_scalar(2267, 0.0);
        }

        if ((s.v[1934] != 0.0) && (s.v[2176] != 0.0)) {
            s.store_scalar(2268, 0.0);
        }

        if ((s.v[1934] != 0.0) && (s.v[2176] != 0.0)) {
            s.store_scalar(2269, 0.0);
        }

        if ((s.v[1934] != 0.0) && (s.v[2176] != 0.0)) {
            s.store_scalar(2270, 0.0);
        }

        if ((s.v[1934] != 0.0) && (s.v[2176] != 0.0)) {
            s.store_scalar(2271, 0.0);
        }

        if ((s.v[1934] != 0.0) && (s.v[2176] != 0.0)) {
            s.store_scalar(2272, 0.0);
        }

        if ((s.v[1934] != 0.0) && (s.v[2176] != 0.0)) {
            s.store_scalar(2273, 0.0);
        }

        if ((s.v[1934] != 0.0) && (s.v[2176] != 0.0)) {
            s.store_scalar(2274, 0.0);
        }

        if ((s.v[1934] != 0.0) && (s.v[2176] != 0.0)) {
            s.store_scalar(2275, 0.0);
        }

        if ((s.v[1934] != 0.0) && (s.v[2176] != 0.0)) {
            s.store_scalar(2276, 0.0);
        }

        if ((s.v[1934] != 0.0) && (s.v[2176] != 0.0)) {
            s.store_scalar(2277, 0.0);
        }

        if ((s.v[1934] != 0.0) && (s.v[2176] != 0.0)) {
            s.store_scalar(2278, 0.0);
        }

        if ((s.v[1934] != 0.0) && (s.v[2176] != 0.0)) {
            s.store_scalar(2279, 0.0);
        }

        if ((s.v[1934] != 0.0) && (s.v[2176] != 0.0)) {
            s.store_scalar(2280, 0.0);
        }

        if ((s.v[1934] != 0.0) && (s.v[2176] != 0.0)) {
            s.store_scalar(2281, 0.0);
        }

        if ((s.v[1934] != 0.0) && (s.v[2176] != 0.0)) {
            s.store_scalar(2282, 0.0);
        }

        if ((s.v[1934] != 0.0) && (s.v[2176] != 0.0)) {
            s.store_scalar(2283, 0.0);
        }

        if ((s.v[1934] != 0.0) && (s.v[2176] != 0.0)) {
            s.store_scalar(2284, 0.0);
        }

        if ((s.v[1934] != 0.0) && (s.v[2176] != 0.0)) {
            s.store_scalar(2285, 0.0);
        }

        if ((s.v[1934] != 0.0) && (s.v[2176] != 0.0)) {
            s.store_scalar(2286, 0.0);
        }

        if ((s.v[1934] != 0.0) && (s.v[2176] != 0.0)) {
            s.store_scalar(2287, 0.0);
        }

        if ((s.v[1934] != 0.0) && (s.v[2176] != 0.0)) {
            s.store_scalar(2288, 0.0);
        }

        if ((s.v[1934] != 0.0) && (s.v[2176] != 0.0)) {
            s.store_scalar(2289, 0.0);
        }

        if ((s.v[1934] != 0.0) && (s.v[2176] != 0.0)) {
            s.store_scalar(2290, 0.0);
        }

        if ((s.v[1934] != 0.0) && (s.v[2176] != 0.0)) {
            s.store_scalar(2291, 0.0);
        }

        if ((s.v[1934] != 0.0) && (s.v[2176] != 0.0)) {
            s.store_scalar(2292, 0.0);
        }

        if ((s.v[1934] != 0.0) && (s.v[2176] != 0.0)) {
            s.store_mul_ad(2272, A::div(s.ad_value(2257), s.ad_value(2241)), A::neg(s.ad_value(2258)));
        }

        if ((s.v[1934] != 0.0) && (s.v[2176] != 0.0)) {
            let assign38800_ad_e36138: A = {
                if ((!(s.v[2272] > 50.0)) && (!(s.v[2272] < (-50.0)))) {
                    A::exp(s.ad_value(2272))
                } else {
                    {
                        if ((!(s.v[2272] > 50.0)) && (s.v[2272] < (-50.0))) {
                            A::exp(A::neg(A::constant(50.0)))
                        } else {
                            {
                                if (s.v[2272] > 50.0) {
                                    A::scale(A::offset(A::offset(s.ad_value(2272), (-50.0)), 1.0), ((50.0) as f64).exp())
                                } else {
                                    A::constant(0.0)
                                }
                            }
                        }
                    }
                }
            };
            s.store_ad(2262, &assign38800_ad_e36138);
        }

        if ((s.v[1934] != 0.0) && (s.v[2176] != 0.0)) {
            s.store_add_ad_lhs(2268, A::mul(s.ad_value(2246), A::sub(A::neg(s.ad_value(2240)), s.ad_value(2247))), 2272);
        }

        if ((s.v[1934] != 0.0) && (s.v[2176] != 0.0)) {
            s.store_add_ad_lhs(2269, A::mul(A::neg(s.ad_value(2246)), s.ad_value(2247)), 2272);
        }

        if ((s.v[1934] != 0.0) && (s.v[2176] != 0.0)) {
            let assign38830_ad_e36206: A = {
                if ((!(s.v[2268] > 50.0)) && (!(s.v[2268] < (-50.0)))) {
                    A::exp(s.ad_value(2268))
                } else {
                    {
                        if ((!(s.v[2268] > 50.0)) && (s.v[2268] < (-50.0))) {
                            A::exp(A::neg(A::constant(50.0)))
                        } else {
                            {
                                if (s.v[2268] > 50.0) {
                                    A::scale(A::offset(A::offset(s.ad_value(2268), (-50.0)), 1.0), ((50.0) as f64).exp())
                                } else {
                                    A::constant(0.0)
                                }
                            }
                        }
                    }
                }
            };
            s.store_ad(2270, &assign38830_ad_e36206);
        }

        if ((s.v[1934] != 0.0) && (s.v[2176] != 0.0)) {
            let assign38840_ad_e36250: A = {
                if ((!(s.v[2269] > 50.0)) && (!(s.v[2269] < (-50.0)))) {
                    A::exp(s.ad_value(2269))
                } else {
                    {
                        if ((!(s.v[2269] > 50.0)) && (s.v[2269] < (-50.0))) {
                            A::exp(A::neg(A::constant(50.0)))
                        } else {
                            {
                                if (s.v[2269] > 50.0) {
                                    A::scale(A::offset(A::offset(s.ad_value(2269), (-50.0)), 1.0), ((50.0) as f64).exp())
                                } else {
                                    A::constant(0.0)
                                }
                            }
                        }
                    }
                }
            };
            s.store_ad(2271, &assign38840_ad_e36250);
        }

        if ((s.v[1934] != 0.0) && (s.v[2176] != 0.0)) {
            s.store_sub(2264, 2270, 2271);
        }

        if ((s.v[1934] != 0.0) && (s.v[2176] != 0.0)) {
            s.store_mul_ad_lhs(2238, A::mul(A::mul(A::mul(s.ad_value(2259), s.ad_value(2249)), s.ad_value(2250)), s.ad_value(2251)), 2248);
        }

        if ((s.v[1934] != 0.0) && (s.v[2176] != 0.0)) {
            s.store_add_ad_lhs(2274, A::mul(A::div(s.ad_value(2245), s.ad_value(2241)), s.ad_value(2240)), 2272);
        }

        if ((s.v[1934] != 0.0) && (s.v[2176] != 0.0)) {
            let assign38880_ad_e36328: A = {
                if ((!(s.v[2274] > 50.0)) && (!(s.v[2274] < (-50.0)))) {
                    A::exp(s.ad_value(2274))
                } else {
                    {
                        if ((!(s.v[2274] > 50.0)) && (s.v[2274] < (-50.0))) {
                            A::exp(A::neg(A::constant(50.0)))
                        } else {
                            {
                                if (s.v[2274] > 50.0) {
                                    A::scale(A::offset(A::offset(s.ad_value(2274), (-50.0)), 1.0), ((50.0) as f64).exp())
                                } else {
                                    A::constant(0.0)
                                }
                            }
                        }
                    }
                }
            };
            s.store_ad(2275, &assign38880_ad_e36328);
        }

        s.v[2293] = if (s.v[2244] == 1.0) { 1.0 } else { 0.0 };

        if (((s.v[1934] != 0.0) && (s.v[2176] != 0.0)) && (s.v[2293] != 0.0)) {
            s.store_mul_ad_rhs(2265, 2238, A::sub(A::sub(s.ad_value(2275), A::mul(s.ad_value(2252), s.ad_value(2264))), s.ad_value(2262)));
        }

        if (((s.v[1934] != 0.0) && (s.v[2176] != 0.0)) && (!(s.v[2293] != 0.0))) {
            s.store_add_ad_lhs(2279, A::mul(s.ad_value(2246), A::sub(A::neg(s.ad_value(2242)), s.ad_value(2247))), 2272);
        }

        if (((s.v[1934] != 0.0) && (s.v[2176] != 0.0)) && (!(s.v[2293] != 0.0))) {
            let assign38920_ad_e36410: A = {
                if ((!(s.v[2279] > 50.0)) && (!(s.v[2279] < (-50.0)))) {
                    A::exp(s.ad_value(2279))
                } else {
                    {
                        if ((!(s.v[2279] > 50.0)) && (s.v[2279] < (-50.0))) {
                            A::exp(A::neg(A::constant(50.0)))
                        } else {
                            {
                                if (s.v[2279] > 50.0) {
                                    A::scale(A::offset(A::offset(s.ad_value(2279), (-50.0)), 1.0), ((50.0) as f64).exp())
                                } else {
                                    A::constant(0.0)
                                }
                            }
                        }
                    }
                }
            };
            s.store_ad(2280, &assign38920_ad_e36410);
        }

        if (((s.v[1934] != 0.0) && (s.v[2176] != 0.0)) && (!(s.v[2293] != 0.0))) {
            s.store_sub(2281, 2280, 2271);
        }

        if (((s.v[1934] != 0.0) && (s.v[2176] != 0.0)) && (!(s.v[2293] != 0.0))) {
            s.store_add_ad_lhs(2282, A::mul(A::div(s.ad_value(2245), s.ad_value(2241)), s.ad_value(2242)), 2272);
        }

        if (((s.v[1934] != 0.0) && (s.v[2176] != 0.0)) && (!(s.v[2293] != 0.0))) {
            let assign38950_ad_e36483: A = {
                if ((!(s.v[2282] > 50.0)) && (!(s.v[2282] < (-50.0)))) {
                    A::exp(s.ad_value(2282))
                } else {
                    {
                        if ((!(s.v[2282] > 50.0)) && (s.v[2282] < (-50.0))) {
                            A::exp(A::neg(A::constant(50.0)))
                        } else {
                            {
                                if (s.v[2282] > 50.0) {
                                    A::scale(A::offset(A::offset(s.ad_value(2282), (-50.0)), 1.0), ((50.0) as f64).exp())
                                } else {
                                    A::constant(0.0)
                                }
                            }
                        }
                    }
                }
            };
            s.store_ad(2283, &assign38950_ad_e36483);
        }

        if (((s.v[1934] != 0.0) && (s.v[2176] != 0.0)) && (!(s.v[2293] != 0.0))) {
            s.store_sub_ad_lhs(2284, A::sub(s.ad_value(2283), A::mul(s.ad_value(2252), s.ad_value(2281))), 2262);
        }

        if (((s.v[1934] != 0.0) && (s.v[2176] != 0.0)) && (!(s.v[2293] != 0.0))) {
            s.store_mul_ad_rhs(2285, 2238, A::sub(A::sub(s.ad_value(2275), A::mul(s.ad_value(2252), s.ad_value(2264))), s.ad_value(2262)));
        }

        s.v[2294] = if (s.v[2244] > 0.0) { 1.0 } else { 0.0 };

        if ((((s.v[1934] != 0.0) && (s.v[2176] != 0.0)) && (!(s.v[2293] != 0.0))) && (s.v[2294] != 0.0)) {
            s.store_mul(2278, 2244, 2245);
        }

        if ((((s.v[1934] != 0.0) && (s.v[2176] != 0.0)) && (!(s.v[2293] != 0.0))) && (s.v[2294] != 0.0)) {
            s.store_add_ad_lhs(2286, A::mul(A::div(s.ad_value(2278), s.ad_value(2241)), s.ad_value(2242)), 2272);
        }

        if ((((s.v[1934] != 0.0) && (s.v[2176] != 0.0)) && (!(s.v[2293] != 0.0))) && (s.v[2294] != 0.0)) {
            let assign39010_ad_e36597: A = {
                if ((!(s.v[2286] > 50.0)) && (!(s.v[2286] < (-50.0)))) {
                    A::exp(s.ad_value(2286))
                } else {
                    {
                        if ((!(s.v[2286] > 50.0)) && (s.v[2286] < (-50.0))) {
                            A::exp(A::neg(A::constant(50.0)))
                        } else {
                            {
                                if (s.v[2286] > 50.0) {
                                    A::scale(A::offset(A::offset(s.ad_value(2286), (-50.0)), 1.0), ((50.0) as f64).exp())
                                } else {
                                    A::constant(0.0)
                                }
                            }
                        }
                    }
                }
            };
            s.store_ad(2287, &assign39010_ad_e36597);
        }

        if ((((s.v[1934] != 0.0) && (s.v[2176] != 0.0)) && (!(s.v[2293] != 0.0))) && (s.v[2294] != 0.0)) {
            s.store_sub_ad_lhs(2288, A::sub(s.ad_value(2287), A::mul(s.ad_value(2252), s.ad_value(2281))), 2262);
        }

        if ((((s.v[1934] != 0.0) && (s.v[2176] != 0.0)) && (!(s.v[2293] != 0.0))) && (s.v[2294] != 0.0)) {
            s.store_add_ad_lhs(2289, A::mul(A::div(s.ad_value(2278), s.ad_value(2241)), s.ad_value(2240)), 2272);
        }

        if ((((s.v[1934] != 0.0) && (s.v[2176] != 0.0)) && (!(s.v[2293] != 0.0))) && (s.v[2294] != 0.0)) {
            let assign39040_ad_e36680: A = {
                if ((!(s.v[2289] > 50.0)) && (!(s.v[2289] < (-50.0)))) {
                    A::exp(s.ad_value(2289))
                } else {
                    {
                        if ((!(s.v[2289] > 50.0)) && (s.v[2289] < (-50.0))) {
                            A::exp(A::neg(A::constant(50.0)))
                        } else {
                            {
                                if (s.v[2289] > 50.0) {
                                    A::scale(A::offset(A::offset(s.ad_value(2289), (-50.0)), 1.0), ((50.0) as f64).exp())
                                } else {
                                    A::constant(0.0)
                                }
                            }
                        }
                    }
                }
            };
            s.store_ad(2290, &assign39040_ad_e36680);
        }

        if ((((s.v[1934] != 0.0) && (s.v[2176] != 0.0)) && (!(s.v[2293] != 0.0))) && (s.v[2294] != 0.0)) {
            s.store_div_ad_lhs(2291, A::mul(s.ad_value(2238), s.ad_value(2284)), 2288);
        }

        if ((((s.v[1934] != 0.0) && (s.v[2176] != 0.0)) && (!(s.v[2293] != 0.0))) && (s.v[2294] != 0.0)) {
            s.store_mul_ad_rhs(2292, 2291, A::sub(A::sub(s.ad_value(2290), A::mul(s.ad_value(2252), s.ad_value(2264))), s.ad_value(2262)));
        }

        if ((((s.v[1934] != 0.0) && (s.v[2176] != 0.0)) && (!(s.v[2293] != 0.0))) && (!(s.v[2294] != 0.0))) {
            s.store_mul(2292, 2238, 2284);
        }

        if (((s.v[1934] != 0.0) && (s.v[2176] != 0.0)) && (!(s.v[2293] != 0.0))) {
            s.store_mul_ad_lhs(2261, A::square(s.ad_value(2243)), 2241);
        }

        if (((s.v[1934] != 0.0) && (s.v[2176] != 0.0)) && (!(s.v[2293] != 0.0))) {
            s.store_div_ad_lhs(2273, A::sub(s.ad_value(2240), A::sub(s.ad_value(2242), A::scale(s.ad_value(2261), 0.5))), 2261);
        }

        s.v[2295] = if (s.v[2273] > 50.0) { 1.0 } else { 0.0 };

        if ((((s.v[1934] != 0.0) && (s.v[2176] != 0.0)) && (!(s.v[2293] != 0.0))) && (s.v[2295] != 0.0)) {
            s.store_scalar(2263, 0.0);
        }

        s.v[2296] = if (s.v[2273] < (-50.0)) { 1.0 } else { 0.0 };

        if (((((s.v[1934] != 0.0) && (s.v[2176] != 0.0)) && (!(s.v[2293] != 0.0))) && (!(s.v[2295] != 0.0))) && (s.v[2296] != 0.0)) {
            s.store_scalar(2263, 1.0);
        }

        if (((((s.v[1934] != 0.0) && (s.v[2176] != 0.0)) && (!(s.v[2293] != 0.0))) && (!(s.v[2295] != 0.0))) && (!(s.v[2296] != 0.0))) {
            s.store_div_from_scalar_ad(2263, 1.0, A::offset(A::exp(s.ad_value(2273)), 1.0));
        }

        if (((s.v[1934] != 0.0) && (s.v[2176] != 0.0)) && (!(s.v[2293] != 0.0))) {
            s.store_add_ad(2265, A::mul(s.ad_value(2263), s.ad_value(2285)), A::mul(A::sub_from_scalar(1.0, s.ad_value(2263)), s.ad_value(2292)));
        }

        if ((s.v[1934] != 0.0) && (s.v[2176] != 0.0)) {
            let assign39160_ad_e36867: A = {
                if (!(p.p52 == 0.0)) {
                    A::mul(A::div(s.ad_value(2240), s.ad_value(2253)), A::tanh(A::scale(A::div(s.ad_value(2240), s.ad_value(2253)), (0.001 / p.p53))))
                } else {
                    {
                        if (p.p52 == 0.0) {
                            A::sqrt(A::offset(A::mul(A::div(s.ad_value(2240), s.ad_value(2253)), A::div(s.ad_value(2240), s.ad_value(2253))), p.p53))
                        } else {
                            A::constant(0.0)
                        }
                    }
                }
            };
            s.store_div_ad(2266, A::neg(s.ad_value(2240)), A::pow(A::offset(A::pow(assign39160_ad_e36867, s.ad_value(2254)), 1.0), A::div_from_scalar(1.0, s.ad_value(2254))));
        }

        if ((s.v[1934] != 0.0) && (s.v[2176] != 0.0)) {
            s.store_mul_ad_lhs(2239, A::mul(A::mul(A::mul(A::neg(s.ad_value(2259)), s.ad_value(2249)), s.ad_value(2250)), s.ad_value(2255)), 2248);
        }

        if ((s.v[1934] != 0.0) && (s.v[2176] != 0.0)) {
            s.store_mul_ad_lhs(2276, A::div(s.ad_value(2256), s.ad_value(2241)), 2266);
        }

        if ((s.v[1934] != 0.0) && (s.v[2176] != 0.0)) {
            let assign39190_ad_e36946: A = {
                if ((!(s.v[2276] > 50.0)) && (!(s.v[2276] < (-50.0)))) {
                    A::exp(s.ad_value(2276))
                } else {
                    {
                        if ((!(s.v[2276] > 50.0)) && (s.v[2276] < (-50.0))) {
                            A::exp(A::neg(A::constant(50.0)))
                        } else {
                            {
                                if (s.v[2276] > 50.0) {
                                    A::scale(A::offset(A::offset(s.ad_value(2276), (-50.0)), 1.0), ((50.0) as f64).exp())
                                } else {
                                    A::constant(0.0)
                                }
                            }
                        }
                    }
                }
            };
            s.store_ad(2277, &assign39190_ad_e36946);
        }

        if ((s.v[1934] != 0.0) && (s.v[2176] != 0.0)) {
            s.store_mul_ad_rhs(2267, 2239, A::offset(s.ad_value(2277), (-1.0)));
        }

        if ((s.v[1934] != 0.0) && (s.v[2176] != 0.0)) {
            s.store_add(2260, 2265, 2267);
        }

        if ((s.v[1934] != 0.0) && (s.v[2176] != 0.0)) {
            s.copy_ad(2237, 2260);
        }

        if ((s.v[1934] != 0.0) && (s.v[2176] != 0.0)) {
            s.copy_ad(131, 2238);
        }

        if ((s.v[1934] != 0.0) && (s.v[2176] != 0.0)) {
            s.copy_ad(133, 2239);
        }

        if ((s.v[1934] != 0.0) && (s.v[2176] != 0.0)) {
            s.copy_ad(129, 2237);
        }

        s.v[2297] = if (p.p282 == 1.0) { 1.0 } else { 0.0 };

        if (((s.v[1934] != 0.0) && (s.v[2176] != 0.0)) && (s.v[2297] != 0.0)) {
            s.store_scalar(2298, 0.0);
        }

        if (((s.v[1934] != 0.0) && (s.v[2176] != 0.0)) && (s.v[2297] != 0.0)) {
            s.store_scalar(2299, 0.0);
        }

        if (((s.v[1934] != 0.0) && (s.v[2176] != 0.0)) && (s.v[2297] != 0.0)) {
            s.store_scalar(2300, 0.0);
        }

        if (((s.v[1934] != 0.0) && (s.v[2176] != 0.0)) && (s.v[2297] != 0.0)) {
            s.store_ad(2301, &A::scale(A::voltage(ctx, &nodes, Some(8), Some(9)), p.p6));
        }

        if (((s.v[1934] != 0.0) && (s.v[2176] != 0.0)) && (s.v[2297] != 0.0)) {
            s.copy_ad(2302, 113);
        }

        if (((s.v[1934] != 0.0) && (s.v[2176] != 0.0)) && (s.v[2297] != 0.0)) {
            s.store_scalar(2303, p.p260);
        }

        if (((s.v[1934] != 0.0) && (s.v[2176] != 0.0)) && (s.v[2297] != 0.0)) {
            s.store_scalar(2304, p.p262);
        }

        if (((s.v[1934] != 0.0) && (s.v[2176] != 0.0)) && (s.v[2297] != 0.0)) {
            s.store_scalar(2305, 1.0);
        }

    }

    pub(super) fn stamp_transient_block_31(
        &mut self,
        ctx: &GeneratedEvalContext<'_>,
        stamper: &mut GeneratedStamper<'_>,
        s: &mut Scratch,
    ) {
        let _ = stamper;
        let p = self.params;
        let nodes = self.nodes;
        let branches = self.branches;
        if (((s.v[1934] != 0.0) && (s.v[2176] != 0.0)) && (s.v[2297] != 0.0)) {
            s.store_scalar(2306, p.p258);
        }

        if (((s.v[1934] != 0.0) && (s.v[2176] != 0.0)) && (s.v[2297] != 0.0)) {
            s.store_scalar(2307, p.p278);
        }

        if (((s.v[1934] != 0.0) && (s.v[2176] != 0.0)) && (s.v[2297] != 0.0)) {
            s.store_scalar(2308, p.p277);
        }

        if (((s.v[1934] != 0.0) && (s.v[2176] != 0.0)) && (s.v[2297] != 0.0)) {
            s.copy_ad(2309, 112);
        }

        if (((s.v[1934] != 0.0) && (s.v[2176] != 0.0)) && (s.v[2297] != 0.0)) {
            s.store_scalar(2310, p.p0);
        }

        if (((s.v[1934] != 0.0) && (s.v[2176] != 0.0)) && (s.v[2297] != 0.0)) {
            s.store_scalar(2311, p.p2);
        }

        if (((s.v[1934] != 0.0) && (s.v[2176] != 0.0)) && (s.v[2297] != 0.0)) {
            s.store_scalar(2312, 0.0);
        }

        if (((s.v[1934] != 0.0) && (s.v[2176] != 0.0)) && (s.v[2297] != 0.0)) {
            s.store_scalar(2313, 0.0);
        }

        if (((s.v[1934] != 0.0) && (s.v[2176] != 0.0)) && (s.v[2297] != 0.0)) {
            s.store_scalar(2314, p.p285);
        }

        if (((s.v[1934] != 0.0) && (s.v[2176] != 0.0)) && (s.v[2297] != 0.0)) {
            s.store_scalar(2315, p.p286);
        }

        if (((s.v[1934] != 0.0) && (s.v[2176] != 0.0)) && (s.v[2297] != 0.0)) {
            s.store_scalar(2316, (p.p255 * p.p284));
        }

        if (((s.v[1934] != 0.0) && (s.v[2176] != 0.0)) && (s.v[2297] != 0.0)) {
            s.store_scalar(2317, p.p283);
        }

        if (((s.v[1934] != 0.0) && (s.v[2176] != 0.0)) && (s.v[2297] != 0.0)) {
            s.store_scalar(2318, p.p257);
        }

        if (((s.v[1934] != 0.0) && (s.v[2176] != 0.0)) && (s.v[2297] != 0.0)) {
            s.store_scalar(2319, p.p256);
        }

        if (((s.v[1934] != 0.0) && (s.v[2176] != 0.0)) && (s.v[2297] != 0.0)) {
            s.store_scalar(2320, p.p6);
        }

        if (((s.v[1934] != 0.0) && (s.v[2176] != 0.0)) && (s.v[2297] != 0.0)) {
            s.store_scalar(2321, 0.0);
        }

        if (((s.v[1934] != 0.0) && (s.v[2176] != 0.0)) && (s.v[2297] != 0.0)) {
            s.store_scalar(2322, 0.0);
        }

        if (((s.v[1934] != 0.0) && (s.v[2176] != 0.0)) && (s.v[2297] != 0.0)) {
            s.store_scalar(2323, 0.0);
        }

        if (((s.v[1934] != 0.0) && (s.v[2176] != 0.0)) && (s.v[2297] != 0.0)) {
            s.store_scalar(2324, 0.0);
        }

        if (((s.v[1934] != 0.0) && (s.v[2176] != 0.0)) && (s.v[2297] != 0.0)) {
            s.store_scalar(2325, 0.0);
        }

        if (((s.v[1934] != 0.0) && (s.v[2176] != 0.0)) && (s.v[2297] != 0.0)) {
            s.store_scalar(2326, 0.0);
        }

        if (((s.v[1934] != 0.0) && (s.v[2176] != 0.0)) && (s.v[2297] != 0.0)) {
            s.store_scalar(2327, 0.0);
        }

        if (((s.v[1934] != 0.0) && (s.v[2176] != 0.0)) && (s.v[2297] != 0.0)) {
            s.store_scalar(2328, 0.0);
        }

        if (((s.v[1934] != 0.0) && (s.v[2176] != 0.0)) && (s.v[2297] != 0.0)) {
            s.store_scalar(2329, 0.0);
        }

        if (((s.v[1934] != 0.0) && (s.v[2176] != 0.0)) && (s.v[2297] != 0.0)) {
            s.store_scalar(2330, 0.0);
        }

        if (((s.v[1934] != 0.0) && (s.v[2176] != 0.0)) && (s.v[2297] != 0.0)) {
            s.store_scalar(2331, 0.0);
        }

        if (((s.v[1934] != 0.0) && (s.v[2176] != 0.0)) && (s.v[2297] != 0.0)) {
            s.store_scalar(2332, 0.0);
        }

        if (((s.v[1934] != 0.0) && (s.v[2176] != 0.0)) && (s.v[2297] != 0.0)) {
            s.store_scalar(2333, 0.0);
        }

        if (((s.v[1934] != 0.0) && (s.v[2176] != 0.0)) && (s.v[2297] != 0.0)) {
            s.store_scalar(2334, 0.0);
        }

        if (((s.v[1934] != 0.0) && (s.v[2176] != 0.0)) && (s.v[2297] != 0.0)) {
            s.store_scalar(2335, 0.0);
        }

        if (((s.v[1934] != 0.0) && (s.v[2176] != 0.0)) && (s.v[2297] != 0.0)) {
            s.store_scalar(2336, 0.0);
        }

        if (((s.v[1934] != 0.0) && (s.v[2176] != 0.0)) && (s.v[2297] != 0.0)) {
            s.store_scalar(2337, 0.0);
        }

        if (((s.v[1934] != 0.0) && (s.v[2176] != 0.0)) && (s.v[2297] != 0.0)) {
            s.store_scalar(2338, 0.0);
        }

        if (((s.v[1934] != 0.0) && (s.v[2176] != 0.0)) && (s.v[2297] != 0.0)) {
            s.store_scalar(2339, 0.0);
        }

        if (((s.v[1934] != 0.0) && (s.v[2176] != 0.0)) && (s.v[2297] != 0.0)) {
            s.store_scalar(2340, 0.0);
        }

        if (((s.v[1934] != 0.0) && (s.v[2176] != 0.0)) && (s.v[2297] != 0.0)) {
            s.store_scalar(2341, 0.0);
        }

        if (((s.v[1934] != 0.0) && (s.v[2176] != 0.0)) && (s.v[2297] != 0.0)) {
            s.store_scalar(2342, 0.0);
        }

        if (((s.v[1934] != 0.0) && (s.v[2176] != 0.0)) && (s.v[2297] != 0.0)) {
            s.store_scalar(2343, 0.0);
        }

        if (((s.v[1934] != 0.0) && (s.v[2176] != 0.0)) && (s.v[2297] != 0.0)) {
            s.store_scalar(2344, 0.0);
        }

        if (((s.v[1934] != 0.0) && (s.v[2176] != 0.0)) && (s.v[2297] != 0.0)) {
            s.store_scalar(2345, 0.0);
        }

        if (((s.v[1934] != 0.0) && (s.v[2176] != 0.0)) && (s.v[2297] != 0.0)) {
            s.store_scalar(2346, 0.0);
        }

        if (((s.v[1934] != 0.0) && (s.v[2176] != 0.0)) && (s.v[2297] != 0.0)) {
            s.store_scalar(2347, 0.0);
        }

        if (((s.v[1934] != 0.0) && (s.v[2176] != 0.0)) && (s.v[2297] != 0.0)) {
            s.store_scalar(2348, 0.0);
        }

        if (((s.v[1934] != 0.0) && (s.v[2176] != 0.0)) && (s.v[2297] != 0.0)) {
            s.store_scalar(2349, 0.0);
        }

        if (((s.v[1934] != 0.0) && (s.v[2176] != 0.0)) && (s.v[2297] != 0.0)) {
            s.store_scalar(2350, 0.0);
        }

        if (((s.v[1934] != 0.0) && (s.v[2176] != 0.0)) && (s.v[2297] != 0.0)) {
            s.store_scalar(2351, 0.0);
        }

        if (((s.v[1934] != 0.0) && (s.v[2176] != 0.0)) && (s.v[2297] != 0.0)) {
            s.store_scalar(2352, 0.0);
        }

        if (((s.v[1934] != 0.0) && (s.v[2176] != 0.0)) && (s.v[2297] != 0.0)) {
            s.store_scalar(2353, 0.0);
        }

        if (((s.v[1934] != 0.0) && (s.v[2176] != 0.0)) && (s.v[2297] != 0.0)) {
            s.store_mul_ad(2333, A::div(s.ad_value(2318), s.ad_value(2302)), A::neg(s.ad_value(2319)));
        }

        if (((s.v[1934] != 0.0) && (s.v[2176] != 0.0)) && (s.v[2297] != 0.0)) {
            let assign39840_ad_e37502: A = {
                if ((!(s.v[2333] > 50.0)) && (!(s.v[2333] < (-50.0)))) {
                    A::exp(s.ad_value(2333))
                } else {
                    {
                        if ((!(s.v[2333] > 50.0)) && (s.v[2333] < (-50.0))) {
                            A::exp(A::neg(A::constant(50.0)))
                        } else {
                            {
                                if (s.v[2333] > 50.0) {
                                    A::scale(A::offset(A::offset(s.ad_value(2333), (-50.0)), 1.0), ((50.0) as f64).exp())
                                } else {
                                    A::constant(0.0)
                                }
                            }
                        }
                    }
                }
            };
            s.store_ad(2323, &assign39840_ad_e37502);
        }

        if (((s.v[1934] != 0.0) && (s.v[2176] != 0.0)) && (s.v[2297] != 0.0)) {
            s.store_add_ad_lhs(2329, A::mul(s.ad_value(2307), A::sub(A::neg(s.ad_value(2301)), s.ad_value(2308))), 2333);
        }

        if (((s.v[1934] != 0.0) && (s.v[2176] != 0.0)) && (s.v[2297] != 0.0)) {
            s.store_add_ad_lhs(2330, A::mul(A::neg(s.ad_value(2307)), s.ad_value(2308)), 2333);
        }

        if (((s.v[1934] != 0.0) && (s.v[2176] != 0.0)) && (s.v[2297] != 0.0)) {
            let assign39870_ad_e37576: A = {
                if ((!(s.v[2329] > 50.0)) && (!(s.v[2329] < (-50.0)))) {
                    A::exp(s.ad_value(2329))
                } else {
                    {
                        if ((!(s.v[2329] > 50.0)) && (s.v[2329] < (-50.0))) {
                            A::exp(A::neg(A::constant(50.0)))
                        } else {
                            {
                                if (s.v[2329] > 50.0) {
                                    A::scale(A::offset(A::offset(s.ad_value(2329), (-50.0)), 1.0), ((50.0) as f64).exp())
                                } else {
                                    A::constant(0.0)
                                }
                            }
                        }
                    }
                }
            };
            s.store_ad(2331, &assign39870_ad_e37576);
        }

        if (((s.v[1934] != 0.0) && (s.v[2176] != 0.0)) && (s.v[2297] != 0.0)) {
            let assign39880_ad_e37622: A = {
                if ((!(s.v[2330] > 50.0)) && (!(s.v[2330] < (-50.0)))) {
                    A::exp(s.ad_value(2330))
                } else {
                    {
                        if ((!(s.v[2330] > 50.0)) && (s.v[2330] < (-50.0))) {
                            A::exp(A::neg(A::constant(50.0)))
                        } else {
                            {
                                if (s.v[2330] > 50.0) {
                                    A::scale(A::offset(A::offset(s.ad_value(2330), (-50.0)), 1.0), ((50.0) as f64).exp())
                                } else {
                                    A::constant(0.0)
                                }
                            }
                        }
                    }
                }
            };
            s.store_ad(2332, &assign39880_ad_e37622);
        }

        if (((s.v[1934] != 0.0) && (s.v[2176] != 0.0)) && (s.v[2297] != 0.0)) {
            s.store_sub(2325, 2331, 2332);
        }

        if (((s.v[1934] != 0.0) && (s.v[2176] != 0.0)) && (s.v[2297] != 0.0)) {
            s.store_mul_ad_lhs(2299, A::mul(A::mul(A::mul(s.ad_value(2320), s.ad_value(2310)), s.ad_value(2311)), s.ad_value(2312)), 2309);
        }

        if (((s.v[1934] != 0.0) && (s.v[2176] != 0.0)) && (s.v[2297] != 0.0)) {
            s.store_add_ad_lhs(2335, A::mul(A::div(s.ad_value(2306), s.ad_value(2302)), s.ad_value(2301)), 2333);
        }

        if (((s.v[1934] != 0.0) && (s.v[2176] != 0.0)) && (s.v[2297] != 0.0)) {
            let assign39920_ad_e37708: A = {
                if ((!(s.v[2335] > 50.0)) && (!(s.v[2335] < (-50.0)))) {
                    A::exp(s.ad_value(2335))
                } else {
                    {
                        if ((!(s.v[2335] > 50.0)) && (s.v[2335] < (-50.0))) {
                            A::exp(A::neg(A::constant(50.0)))
                        } else {
                            {
                                if (s.v[2335] > 50.0) {
                                    A::scale(A::offset(A::offset(s.ad_value(2335), (-50.0)), 1.0), ((50.0) as f64).exp())
                                } else {
                                    A::constant(0.0)
                                }
                            }
                        }
                    }
                }
            };
            s.store_ad(2336, &assign39920_ad_e37708);
        }

        s.v[2354] = if (s.v[2305] == 1.0) { 1.0 } else { 0.0 };

        if ((((s.v[1934] != 0.0) && (s.v[2176] != 0.0)) && (s.v[2297] != 0.0)) && (s.v[2354] != 0.0)) {
            s.store_mul_ad_rhs(2326, 2299, A::sub(A::sub(s.ad_value(2336), A::mul(s.ad_value(2313), s.ad_value(2325))), s.ad_value(2323)));
        }

        if ((((s.v[1934] != 0.0) && (s.v[2176] != 0.0)) && (s.v[2297] != 0.0)) && (!(s.v[2354] != 0.0))) {
            s.store_add_ad_lhs(2340, A::mul(s.ad_value(2307), A::sub(A::neg(s.ad_value(2303)), s.ad_value(2308))), 2333);
        }

        if ((((s.v[1934] != 0.0) && (s.v[2176] != 0.0)) && (s.v[2297] != 0.0)) && (!(s.v[2354] != 0.0))) {
            let assign39960_ad_e37796: A = {
                if ((!(s.v[2340] > 50.0)) && (!(s.v[2340] < (-50.0)))) {
                    A::exp(s.ad_value(2340))
                } else {
                    {
                        if ((!(s.v[2340] > 50.0)) && (s.v[2340] < (-50.0))) {
                            A::exp(A::neg(A::constant(50.0)))
                        } else {
                            {
                                if (s.v[2340] > 50.0) {
                                    A::scale(A::offset(A::offset(s.ad_value(2340), (-50.0)), 1.0), ((50.0) as f64).exp())
                                } else {
                                    A::constant(0.0)
                                }
                            }
                        }
                    }
                }
            };
            s.store_ad(2341, &assign39960_ad_e37796);
        }

        if ((((s.v[1934] != 0.0) && (s.v[2176] != 0.0)) && (s.v[2297] != 0.0)) && (!(s.v[2354] != 0.0))) {
            s.store_sub(2342, 2341, 2332);
        }

        if ((((s.v[1934] != 0.0) && (s.v[2176] != 0.0)) && (s.v[2297] != 0.0)) && (!(s.v[2354] != 0.0))) {
            s.store_add_ad_lhs(2343, A::mul(A::div(s.ad_value(2306), s.ad_value(2302)), s.ad_value(2303)), 2333);
        }

        if ((((s.v[1934] != 0.0) && (s.v[2176] != 0.0)) && (s.v[2297] != 0.0)) && (!(s.v[2354] != 0.0))) {
            let assign39990_ad_e37875: A = {
                if ((!(s.v[2343] > 50.0)) && (!(s.v[2343] < (-50.0)))) {
                    A::exp(s.ad_value(2343))
                } else {
                    {
                        if ((!(s.v[2343] > 50.0)) && (s.v[2343] < (-50.0))) {
                            A::exp(A::neg(A::constant(50.0)))
                        } else {
                            {
                                if (s.v[2343] > 50.0) {
                                    A::scale(A::offset(A::offset(s.ad_value(2343), (-50.0)), 1.0), ((50.0) as f64).exp())
                                } else {
                                    A::constant(0.0)
                                }
                            }
                        }
                    }
                }
            };
            s.store_ad(2344, &assign39990_ad_e37875);
        }

        if ((((s.v[1934] != 0.0) && (s.v[2176] != 0.0)) && (s.v[2297] != 0.0)) && (!(s.v[2354] != 0.0))) {
            s.store_sub_ad_lhs(2345, A::sub(s.ad_value(2344), A::mul(s.ad_value(2313), s.ad_value(2342))), 2323);
        }

        if ((((s.v[1934] != 0.0) && (s.v[2176] != 0.0)) && (s.v[2297] != 0.0)) && (!(s.v[2354] != 0.0))) {
            s.store_mul_ad_rhs(2346, 2299, A::sub(A::sub(s.ad_value(2336), A::mul(s.ad_value(2313), s.ad_value(2325))), s.ad_value(2323)));
        }

        s.v[2355] = if (s.v[2305] > 0.0) { 1.0 } else { 0.0 };

        if (((((s.v[1934] != 0.0) && (s.v[2176] != 0.0)) && (s.v[2297] != 0.0)) && (!(s.v[2354] != 0.0))) && (s.v[2355] != 0.0)) {
            s.store_mul(2339, 2305, 2306);
        }

        if (((((s.v[1934] != 0.0) && (s.v[2176] != 0.0)) && (s.v[2297] != 0.0)) && (!(s.v[2354] != 0.0))) && (s.v[2355] != 0.0)) {
            s.store_add_ad_lhs(2347, A::mul(A::div(s.ad_value(2339), s.ad_value(2302)), s.ad_value(2303)), 2333);
        }

        if (((((s.v[1934] != 0.0) && (s.v[2176] != 0.0)) && (s.v[2297] != 0.0)) && (!(s.v[2354] != 0.0))) && (s.v[2355] != 0.0)) {
            let assign40050_ad_e37999: A = {
                if ((!(s.v[2347] > 50.0)) && (!(s.v[2347] < (-50.0)))) {
                    A::exp(s.ad_value(2347))
                } else {
                    {
                        if ((!(s.v[2347] > 50.0)) && (s.v[2347] < (-50.0))) {
                            A::exp(A::neg(A::constant(50.0)))
                        } else {
                            {
                                if (s.v[2347] > 50.0) {
                                    A::scale(A::offset(A::offset(s.ad_value(2347), (-50.0)), 1.0), ((50.0) as f64).exp())
                                } else {
                                    A::constant(0.0)
                                }
                            }
                        }
                    }
                }
            };
            s.store_ad(2348, &assign40050_ad_e37999);
        }

        if (((((s.v[1934] != 0.0) && (s.v[2176] != 0.0)) && (s.v[2297] != 0.0)) && (!(s.v[2354] != 0.0))) && (s.v[2355] != 0.0)) {
            s.store_sub_ad_lhs(2349, A::sub(s.ad_value(2348), A::mul(s.ad_value(2313), s.ad_value(2342))), 2323);
        }

        if (((((s.v[1934] != 0.0) && (s.v[2176] != 0.0)) && (s.v[2297] != 0.0)) && (!(s.v[2354] != 0.0))) && (s.v[2355] != 0.0)) {
            s.store_add_ad_lhs(2350, A::mul(A::div(s.ad_value(2339), s.ad_value(2302)), s.ad_value(2301)), 2333);
        }

        if (((((s.v[1934] != 0.0) && (s.v[2176] != 0.0)) && (s.v[2297] != 0.0)) && (!(s.v[2354] != 0.0))) && (s.v[2355] != 0.0)) {
            let assign40080_ad_e38088: A = {
                if ((!(s.v[2350] > 50.0)) && (!(s.v[2350] < (-50.0)))) {
                    A::exp(s.ad_value(2350))
                } else {
                    {
                        if ((!(s.v[2350] > 50.0)) && (s.v[2350] < (-50.0))) {
                            A::exp(A::neg(A::constant(50.0)))
                        } else {
                            {
                                if (s.v[2350] > 50.0) {
                                    A::scale(A::offset(A::offset(s.ad_value(2350), (-50.0)), 1.0), ((50.0) as f64).exp())
                                } else {
                                    A::constant(0.0)
                                }
                            }
                        }
                    }
                }
            };
            s.store_ad(2351, &assign40080_ad_e38088);
        }

        if (((((s.v[1934] != 0.0) && (s.v[2176] != 0.0)) && (s.v[2297] != 0.0)) && (!(s.v[2354] != 0.0))) && (s.v[2355] != 0.0)) {
            s.store_div_ad_lhs(2352, A::mul(s.ad_value(2299), s.ad_value(2345)), 2349);
        }

        if (((((s.v[1934] != 0.0) && (s.v[2176] != 0.0)) && (s.v[2297] != 0.0)) && (!(s.v[2354] != 0.0))) && (s.v[2355] != 0.0)) {
            s.store_mul_ad_rhs(2353, 2352, A::sub(A::sub(s.ad_value(2351), A::mul(s.ad_value(2313), s.ad_value(2325))), s.ad_value(2323)));
        }

        if (((((s.v[1934] != 0.0) && (s.v[2176] != 0.0)) && (s.v[2297] != 0.0)) && (!(s.v[2354] != 0.0))) && (!(s.v[2355] != 0.0))) {
            s.store_mul(2353, 2299, 2345);
        }

        if ((((s.v[1934] != 0.0) && (s.v[2176] != 0.0)) && (s.v[2297] != 0.0)) && (!(s.v[2354] != 0.0))) {
            s.store_mul_ad_lhs(2322, A::square(s.ad_value(2304)), 2302);
        }

        if ((((s.v[1934] != 0.0) && (s.v[2176] != 0.0)) && (s.v[2297] != 0.0)) && (!(s.v[2354] != 0.0))) {
            s.store_div_ad_lhs(2334, A::sub(s.ad_value(2301), A::sub(s.ad_value(2303), A::scale(s.ad_value(2322), 0.5))), 2322);
        }

        s.v[2356] = if (s.v[2334] > 50.0) { 1.0 } else { 0.0 };

        if (((((s.v[1934] != 0.0) && (s.v[2176] != 0.0)) && (s.v[2297] != 0.0)) && (!(s.v[2354] != 0.0))) && (s.v[2356] != 0.0)) {
            s.store_scalar(2324, 0.0);
        }

        s.v[2357] = if (s.v[2334] < (-50.0)) { 1.0 } else { 0.0 };

        if ((((((s.v[1934] != 0.0) && (s.v[2176] != 0.0)) && (s.v[2297] != 0.0)) && (!(s.v[2354] != 0.0))) && (!(s.v[2356] != 0.0))) && (s.v[2357] != 0.0)) {
            s.store_scalar(2324, 1.0);
        }

        if ((((((s.v[1934] != 0.0) && (s.v[2176] != 0.0)) && (s.v[2297] != 0.0)) && (!(s.v[2354] != 0.0))) && (!(s.v[2356] != 0.0))) && (!(s.v[2357] != 0.0))) {
            s.store_div_from_scalar_ad(2324, 1.0, A::offset(A::exp(s.ad_value(2334)), 1.0));
        }

        if ((((s.v[1934] != 0.0) && (s.v[2176] != 0.0)) && (s.v[2297] != 0.0)) && (!(s.v[2354] != 0.0))) {
            s.store_add_ad(2326, A::mul(s.ad_value(2324), s.ad_value(2346)), A::mul(A::sub_from_scalar(1.0, s.ad_value(2324)), s.ad_value(2353)));
        }

        if (((s.v[1934] != 0.0) && (s.v[2176] != 0.0)) && (s.v[2297] != 0.0)) {
            let assign40200_ad_e38295: A = {
                if (!(p.p52 == 0.0)) {
                    A::mul(A::div(s.ad_value(2301), s.ad_value(2314)), A::tanh(A::scale(A::div(s.ad_value(2301), s.ad_value(2314)), (0.001 / p.p53))))
                } else {
                    {
                        if (p.p52 == 0.0) {
                            A::sqrt(A::offset(A::mul(A::div(s.ad_value(2301), s.ad_value(2314)), A::div(s.ad_value(2301), s.ad_value(2314))), p.p53))
                        } else {
                            A::constant(0.0)
                        }
                    }
                }
            };
            s.store_div_ad(2327, A::neg(s.ad_value(2301)), A::pow(A::offset(A::pow(assign40200_ad_e38295, s.ad_value(2315)), 1.0), A::div_from_scalar(1.0, s.ad_value(2315))));
        }

        if (((s.v[1934] != 0.0) && (s.v[2176] != 0.0)) && (s.v[2297] != 0.0)) {
            s.store_mul_ad_lhs(2300, A::mul(A::mul(A::mul(A::neg(s.ad_value(2320)), s.ad_value(2310)), s.ad_value(2311)), s.ad_value(2316)), 2309);
        }

        if (((s.v[1934] != 0.0) && (s.v[2176] != 0.0)) && (s.v[2297] != 0.0)) {
            s.store_mul_ad_lhs(2337, A::div(s.ad_value(2317), s.ad_value(2302)), 2327);
        }

    }
}
