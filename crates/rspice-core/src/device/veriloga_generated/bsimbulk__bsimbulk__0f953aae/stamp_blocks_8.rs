#![allow(dead_code, unused_assignments, unused_imports, unused_parens, unused_variables)]

use super::{A, GeneratedDerivative, GeneratedEvalContext, GeneratedReactiveStamper, GeneratedStamper, ReactiveScratch, THERMAL_VOLTAGE_PER_K};
use super::super::state::Instance;

impl Instance {

    pub(super) fn stamp_reactive_equation_11_block_0(
        &mut self,
        ctx: &GeneratedEvalContext<'_>,
        stamper: &mut GeneratedReactiveStamper<'_>,
        s: &mut ReactiveScratch,
    ) {
        let _ = stamper;
        let p = self.params;
        let nodes = self.nodes;
        let branches = self.branches;
        let nv15 = ctx.node_voltage(nodes[15]);
        let (eq11_e1344, eq11_e1344_d_n0, eq11_e1344_d_n1, eq11_e1344_d_n2, eq11_e1344_d_n3, eq11_e1344_d_n4, eq11_e1344_d_n5, eq11_e1344_d_n6, eq11_e1344_d_n7, eq11_e1344_d_n8, eq11_e1344_d_n9, eq11_e1344_d_n10, eq11_e1344_d_n11, eq11_e1344_d_n12, eq11_e1344_d_n13, eq11_e1344_d_n14, eq11_e1344_d_n15, eq11_e1344_d_n16, eq11_e1344_q, eq11_e1344_q_d_n0, eq11_e1344_q_d_n1, eq11_e1344_q_d_n2, eq11_e1344_q_d_n3, eq11_e1344_q_d_n4, eq11_e1344_q_d_n5, eq11_e1344_q_d_n6, eq11_e1344_q_d_n7, eq11_e1344_q_d_n8, eq11_e1344_q_d_n9, eq11_e1344_q_d_n10, eq11_e1344_q_d_n11, eq11_e1344_q_d_n12, eq11_e1344_q_d_n13, eq11_e1344_q_d_n14, eq11_e1344_q_d_n15, eq11_e1344_q_d_n16,) = {
    if ((s.v[1556] != 0.0) && (!(s.v[1555] != 0.0))) {
        let eq11_e1327: f64 = (1.0 + s.v[57]);
        let eq11_e1329: f64 = (eq11_e1327 * s.v[378]);
        let eq11_e1329_d_n0: f64 = ((s.dn[57][0] * s.v[378]) + (eq11_e1327 * s.dn[378][0]));
        let eq11_e1329_d_n1: f64 = ((s.dn[57][1] * s.v[378]) + (eq11_e1327 * s.dn[378][1]));
        let eq11_e1329_d_n2: f64 = ((s.dn[57][2] * s.v[378]) + (eq11_e1327 * s.dn[378][2]));
        let eq11_e1329_d_n3: f64 = ((s.dn[57][3] * s.v[378]) + (eq11_e1327 * s.dn[378][3]));
        let eq11_e1329_d_n4: f64 = ((s.dn[57][4] * s.v[378]) + (eq11_e1327 * s.dn[378][4]));
        let eq11_e1329_d_n5: f64 = ((s.dn[57][5] * s.v[378]) + (eq11_e1327 * s.dn[378][5]));
        let eq11_e1329_d_n6: f64 = ((s.dn[57][6] * s.v[378]) + (eq11_e1327 * s.dn[378][6]));
        let eq11_e1329_d_n7: f64 = ((s.dn[57][7] * s.v[378]) + (eq11_e1327 * s.dn[378][7]));
        let eq11_e1329_d_n8: f64 = ((s.dn[57][8] * s.v[378]) + (eq11_e1327 * s.dn[378][8]));
        let eq11_e1329_d_n9: f64 = ((s.dn[57][9] * s.v[378]) + (eq11_e1327 * s.dn[378][9]));
        let eq11_e1329_d_n10: f64 = ((s.dn[57][10] * s.v[378]) + (eq11_e1327 * s.dn[378][10]));
        let eq11_e1329_d_n11: f64 = ((s.dn[57][11] * s.v[378]) + (eq11_e1327 * s.dn[378][11]));
        let eq11_e1329_d_n12: f64 = ((s.dn[57][12] * s.v[378]) + (eq11_e1327 * s.dn[378][12]));
        let eq11_e1329_d_n13: f64 = ((s.dn[57][13] * s.v[378]) + (eq11_e1327 * s.dn[378][13]));
        let eq11_e1329_d_n14: f64 = ((s.dn[57][14] * s.v[378]) + (eq11_e1327 * s.dn[378][14]));
        let eq11_e1329_d_n15: f64 = ((s.dn[57][15] * s.v[378]) + (eq11_e1327 * s.dn[378][15]));
        let eq11_e1329_d_n16: f64 = ((s.dn[57][16] * s.v[378]) + (eq11_e1327 * s.dn[378][16]));
        let eq11_e1331: f64 = (eq11_e1329 * s.v[46]);
        let eq11_e1331_d_n0: f64 = (eq11_e1329_d_n0 * s.v[46]);
        let eq11_e1331_d_n1: f64 = (eq11_e1329_d_n1 * s.v[46]);
        let eq11_e1331_d_n2: f64 = (eq11_e1329_d_n2 * s.v[46]);
        let eq11_e1331_d_n3: f64 = (eq11_e1329_d_n3 * s.v[46]);
        let eq11_e1331_d_n4: f64 = (eq11_e1329_d_n4 * s.v[46]);
        let eq11_e1331_d_n5: f64 = (eq11_e1329_d_n5 * s.v[46]);
        let eq11_e1331_d_n6: f64 = (eq11_e1329_d_n6 * s.v[46]);
        let eq11_e1331_d_n7: f64 = (eq11_e1329_d_n7 * s.v[46]);
        let eq11_e1331_d_n8: f64 = (eq11_e1329_d_n8 * s.v[46]);
        let eq11_e1331_d_n9: f64 = (eq11_e1329_d_n9 * s.v[46]);
        let eq11_e1331_d_n10: f64 = (eq11_e1329_d_n10 * s.v[46]);
        let eq11_e1331_d_n11: f64 = (eq11_e1329_d_n11 * s.v[46]);
        let eq11_e1331_d_n12: f64 = (eq11_e1329_d_n12 * s.v[46]);
        let eq11_e1331_d_n13: f64 = (eq11_e1329_d_n13 * s.v[46]);
        let eq11_e1331_d_n14: f64 = (eq11_e1329_d_n14 * s.v[46]);
        let eq11_e1331_d_n15: f64 = (eq11_e1329_d_n15 * s.v[46]);
        let eq11_e1331_d_n16: f64 = (eq11_e1329_d_n16 * s.v[46]);
        let eq11_e1333: f64 = (eq11_e1331 * s.v[29]);
        let eq11_e1333_d_n0: f64 = (eq11_e1331_d_n0 * s.v[29]);
        let eq11_e1333_d_n1: f64 = (eq11_e1331_d_n1 * s.v[29]);
        let eq11_e1333_d_n2: f64 = (eq11_e1331_d_n2 * s.v[29]);
        let eq11_e1333_d_n3: f64 = (eq11_e1331_d_n3 * s.v[29]);
        let eq11_e1333_d_n4: f64 = (eq11_e1331_d_n4 * s.v[29]);
        let eq11_e1333_d_n5: f64 = (eq11_e1331_d_n5 * s.v[29]);
        let eq11_e1333_d_n6: f64 = (eq11_e1331_d_n6 * s.v[29]);
        let eq11_e1333_d_n7: f64 = (eq11_e1331_d_n7 * s.v[29]);
        let eq11_e1333_d_n8: f64 = (eq11_e1331_d_n8 * s.v[29]);
        let eq11_e1333_d_n9: f64 = (eq11_e1331_d_n9 * s.v[29]);
        let eq11_e1333_d_n10: f64 = (eq11_e1331_d_n10 * s.v[29]);
        let eq11_e1333_d_n11: f64 = (eq11_e1331_d_n11 * s.v[29]);
        let eq11_e1333_d_n12: f64 = (eq11_e1331_d_n12 * s.v[29]);
        let eq11_e1333_d_n13: f64 = (eq11_e1331_d_n13 * s.v[29]);
        let eq11_e1333_d_n14: f64 = (eq11_e1331_d_n14 * s.v[29]);
        let eq11_e1333_d_n15: f64 = (eq11_e1331_d_n15 * s.v[29]);
        let eq11_e1333_d_n16: f64 = (eq11_e1331_d_n16 * s.v[29]);
        let eq11_e1335: f64 = (eq11_e1333 * p.p2);
        let eq11_e1335_d_n0: f64 = (eq11_e1333_d_n0 * p.p2);
        let eq11_e1335_d_n1: f64 = (eq11_e1333_d_n1 * p.p2);
        let eq11_e1335_d_n2: f64 = (eq11_e1333_d_n2 * p.p2);
        let eq11_e1335_d_n3: f64 = (eq11_e1333_d_n3 * p.p2);
        let eq11_e1335_d_n4: f64 = (eq11_e1333_d_n4 * p.p2);
        let eq11_e1335_d_n5: f64 = (eq11_e1333_d_n5 * p.p2);
        let eq11_e1335_d_n6: f64 = (eq11_e1333_d_n6 * p.p2);
        let eq11_e1335_d_n7: f64 = (eq11_e1333_d_n7 * p.p2);
        let eq11_e1335_d_n8: f64 = (eq11_e1333_d_n8 * p.p2);
        let eq11_e1335_d_n9: f64 = (eq11_e1333_d_n9 * p.p2);
        let eq11_e1335_d_n10: f64 = (eq11_e1333_d_n10 * p.p2);
        let eq11_e1335_d_n11: f64 = (eq11_e1333_d_n11 * p.p2);
        let eq11_e1335_d_n12: f64 = (eq11_e1333_d_n12 * p.p2);
        let eq11_e1335_d_n13: f64 = (eq11_e1333_d_n13 * p.p2);
        let eq11_e1335_d_n14: f64 = (eq11_e1333_d_n14 * p.p2);
        let eq11_e1335_d_n15: f64 = (eq11_e1333_d_n15 * p.p2);
        let eq11_e1335_d_n16: f64 = (eq11_e1333_d_n16 * p.p2);
        let eq11_e1337: f64 = (eq11_e1335 * s.v[30]);
        let eq11_e1337_d_n0: f64 = (eq11_e1335_d_n0 * s.v[30]);
        let eq11_e1337_d_n1: f64 = (eq11_e1335_d_n1 * s.v[30]);
        let eq11_e1337_d_n2: f64 = (eq11_e1335_d_n2 * s.v[30]);
        let eq11_e1337_d_n3: f64 = (eq11_e1335_d_n3 * s.v[30]);
        let eq11_e1337_d_n4: f64 = (eq11_e1335_d_n4 * s.v[30]);
        let eq11_e1337_d_n5: f64 = (eq11_e1335_d_n5 * s.v[30]);
        let eq11_e1337_d_n6: f64 = (eq11_e1335_d_n6 * s.v[30]);
        let eq11_e1337_d_n7: f64 = (eq11_e1335_d_n7 * s.v[30]);
        let eq11_e1337_d_n8: f64 = (eq11_e1335_d_n8 * s.v[30]);
        let eq11_e1337_d_n9: f64 = (eq11_e1335_d_n9 * s.v[30]);
        let eq11_e1337_d_n10: f64 = (eq11_e1335_d_n10 * s.v[30]);
        let eq11_e1337_d_n11: f64 = (eq11_e1335_d_n11 * s.v[30]);
        let eq11_e1337_d_n12: f64 = (eq11_e1335_d_n12 * s.v[30]);
        let eq11_e1337_d_n13: f64 = (eq11_e1335_d_n13 * s.v[30]);
        let eq11_e1337_d_n14: f64 = (eq11_e1335_d_n14 * s.v[30]);
        let eq11_e1337_d_n15: f64 = (eq11_e1335_d_n15 * s.v[30]);
        let eq11_e1337_d_n16: f64 = (eq11_e1335_d_n16 * s.v[30]);
        let eq11_e1339: f64 = (eq11_e1337 * (nv15 - 0.0));
        let eq11_e1339_d_n0: f64 = (eq11_e1337_d_n0 * (nv15 - 0.0));
        let eq11_e1339_d_n1: f64 = (eq11_e1337_d_n1 * (nv15 - 0.0));
        let eq11_e1339_d_n2: f64 = (eq11_e1337_d_n2 * (nv15 - 0.0));
        let eq11_e1339_d_n3: f64 = (eq11_e1337_d_n3 * (nv15 - 0.0));
        let eq11_e1339_d_n4: f64 = (eq11_e1337_d_n4 * (nv15 - 0.0));
        let eq11_e1339_d_n5: f64 = (eq11_e1337_d_n5 * (nv15 - 0.0));
        let eq11_e1339_d_n6: f64 = (eq11_e1337_d_n6 * (nv15 - 0.0));
        let eq11_e1339_d_n7: f64 = (eq11_e1337_d_n7 * (nv15 - 0.0));
        let eq11_e1339_d_n8: f64 = (eq11_e1337_d_n8 * (nv15 - 0.0));
        let eq11_e1339_d_n9: f64 = (eq11_e1337_d_n9 * (nv15 - 0.0));
        let eq11_e1339_d_n10: f64 = (eq11_e1337_d_n10 * (nv15 - 0.0));
        let eq11_e1339_d_n11: f64 = (eq11_e1337_d_n11 * (nv15 - 0.0));
        let eq11_e1339_d_n12: f64 = (eq11_e1337_d_n12 * (nv15 - 0.0));
        let eq11_e1339_d_n13: f64 = (eq11_e1337_d_n13 * (nv15 - 0.0));
        let eq11_e1339_d_n14: f64 = (eq11_e1337_d_n14 * (nv15 - 0.0));
        let eq11_e1339_d_n15: f64 = ((eq11_e1337_d_n15 * (nv15 - 0.0)) + eq11_e1337);
        let eq11_e1339_d_n16: f64 = (eq11_e1337_d_n16 * (nv15 - 0.0));
        let eq11_e1340: f64 = (0.5 * eq11_e1339);
        let eq11_e1340_d_n0: f64 = (0.5 * eq11_e1339_d_n0);
        let eq11_e1340_d_n1: f64 = (0.5 * eq11_e1339_d_n1);
        let eq11_e1340_d_n2: f64 = (0.5 * eq11_e1339_d_n2);
        let eq11_e1340_d_n3: f64 = (0.5 * eq11_e1339_d_n3);
        let eq11_e1340_d_n4: f64 = (0.5 * eq11_e1339_d_n4);
        let eq11_e1340_d_n5: f64 = (0.5 * eq11_e1339_d_n5);
        let eq11_e1340_d_n6: f64 = (0.5 * eq11_e1339_d_n6);
        let eq11_e1340_d_n7: f64 = (0.5 * eq11_e1339_d_n7);
        let eq11_e1340_d_n8: f64 = (0.5 * eq11_e1339_d_n8);
        let eq11_e1340_d_n9: f64 = (0.5 * eq11_e1339_d_n9);
        let eq11_e1340_d_n10: f64 = (0.5 * eq11_e1339_d_n10);
        let eq11_e1340_d_n11: f64 = (0.5 * eq11_e1339_d_n11);
        let eq11_e1340_d_n12: f64 = (0.5 * eq11_e1339_d_n12);
        let eq11_e1340_d_n13: f64 = (0.5 * eq11_e1339_d_n13);
        let eq11_e1340_d_n14: f64 = (0.5 * eq11_e1339_d_n14);
        let eq11_e1340_d_n15: f64 = (0.5 * eq11_e1339_d_n15);
        let eq11_e1340_d_n16: f64 = (0.5 * eq11_e1339_d_n16);
        let eq11_e1341_q: f64 = eq11_e1340;
        let eq11_e1342: f64 = (p.p29 * eq11_e1340);
        let eq11_e1342_d_n0: f64 = (p.p29 * eq11_e1340_d_n0);
        let eq11_e1342_d_n1: f64 = (p.p29 * eq11_e1340_d_n1);
        let eq11_e1342_d_n2: f64 = (p.p29 * eq11_e1340_d_n2);
        let eq11_e1342_d_n3: f64 = (p.p29 * eq11_e1340_d_n3);
        let eq11_e1342_d_n4: f64 = (p.p29 * eq11_e1340_d_n4);
        let eq11_e1342_d_n5: f64 = (p.p29 * eq11_e1340_d_n5);
        let eq11_e1342_d_n6: f64 = (p.p29 * eq11_e1340_d_n6);
        let eq11_e1342_d_n7: f64 = (p.p29 * eq11_e1340_d_n7);
        let eq11_e1342_d_n8: f64 = (p.p29 * eq11_e1340_d_n8);
        let eq11_e1342_d_n9: f64 = (p.p29 * eq11_e1340_d_n9);
        let eq11_e1342_d_n10: f64 = (p.p29 * eq11_e1340_d_n10);
        let eq11_e1342_d_n11: f64 = (p.p29 * eq11_e1340_d_n11);
        let eq11_e1342_d_n12: f64 = (p.p29 * eq11_e1340_d_n12);
        let eq11_e1342_d_n13: f64 = (p.p29 * eq11_e1340_d_n13);
        let eq11_e1342_d_n14: f64 = (p.p29 * eq11_e1340_d_n14);
        let eq11_e1342_d_n15: f64 = (p.p29 * eq11_e1340_d_n15);
        let eq11_e1342_d_n16: f64 = (p.p29 * eq11_e1340_d_n16);
        let eq11_e1342_q: f64 = (p.p29 * eq11_e1341_q);
        let eq11_e1342_q_d_n0: f64 = (p.p29 * eq11_e1340_d_n0);
        let eq11_e1342_q_d_n1: f64 = (p.p29 * eq11_e1340_d_n1);
        let eq11_e1342_q_d_n2: f64 = (p.p29 * eq11_e1340_d_n2);
        let eq11_e1342_q_d_n3: f64 = (p.p29 * eq11_e1340_d_n3);
        let eq11_e1342_q_d_n4: f64 = (p.p29 * eq11_e1340_d_n4);
        let eq11_e1342_q_d_n5: f64 = (p.p29 * eq11_e1340_d_n5);
        let eq11_e1342_q_d_n6: f64 = (p.p29 * eq11_e1340_d_n6);
        let eq11_e1342_q_d_n7: f64 = (p.p29 * eq11_e1340_d_n7);
        let eq11_e1342_q_d_n8: f64 = (p.p29 * eq11_e1340_d_n8);
        let eq11_e1342_q_d_n9: f64 = (p.p29 * eq11_e1340_d_n9);
        let eq11_e1342_q_d_n10: f64 = (p.p29 * eq11_e1340_d_n10);
        let eq11_e1342_q_d_n11: f64 = (p.p29 * eq11_e1340_d_n11);
        let eq11_e1342_q_d_n12: f64 = (p.p29 * eq11_e1340_d_n12);
        let eq11_e1342_q_d_n13: f64 = (p.p29 * eq11_e1340_d_n13);
        let eq11_e1342_q_d_n14: f64 = (p.p29 * eq11_e1340_d_n14);
        let eq11_e1342_q_d_n15: f64 = (p.p29 * eq11_e1340_d_n15);
        let eq11_e1342_q_d_n16: f64 = (p.p29 * eq11_e1340_d_n16);
        (eq11_e1342, eq11_e1342_d_n0, eq11_e1342_d_n1, eq11_e1342_d_n2, eq11_e1342_d_n3, eq11_e1342_d_n4, eq11_e1342_d_n5, eq11_e1342_d_n6, eq11_e1342_d_n7, eq11_e1342_d_n8, eq11_e1342_d_n9, eq11_e1342_d_n10, eq11_e1342_d_n11, eq11_e1342_d_n12, eq11_e1342_d_n13, eq11_e1342_d_n14, eq11_e1342_d_n15, eq11_e1342_d_n16, eq11_e1342_q, eq11_e1342_q_d_n0, eq11_e1342_q_d_n1, eq11_e1342_q_d_n2, eq11_e1342_q_d_n3, eq11_e1342_q_d_n4, eq11_e1342_q_d_n5, eq11_e1342_q_d_n6, eq11_e1342_q_d_n7, eq11_e1342_q_d_n8, eq11_e1342_q_d_n9, eq11_e1342_q_d_n10, eq11_e1342_q_d_n11, eq11_e1342_q_d_n12, eq11_e1342_q_d_n13, eq11_e1342_q_d_n14, eq11_e1342_q_d_n15, eq11_e1342_q_d_n16,)
    } else {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    }
};
        let eq11_reactive_node_derivatives: [f64; 17] = [eq11_e1344_q_d_n0, eq11_e1344_q_d_n1, eq11_e1344_q_d_n2, eq11_e1344_q_d_n3, eq11_e1344_q_d_n4, eq11_e1344_q_d_n5, eq11_e1344_q_d_n6, eq11_e1344_q_d_n7, eq11_e1344_q_d_n8, eq11_e1344_q_d_n9, eq11_e1344_q_d_n10, eq11_e1344_q_d_n11, eq11_e1344_q_d_n12, eq11_e1344_q_d_n13, eq11_e1344_q_d_n14, eq11_e1344_q_d_n15, eq11_e1344_q_d_n16];
        let eq11_reactive_branch_derivatives: [f64; 0] = [];
        stamper.stamp_current_reactive_dense(
            Some(nodes[9]),
            Some(nodes[7]),
            &nodes,
            &eq11_reactive_node_derivatives,
            &branches,
            &eq11_reactive_branch_derivatives,
            self.multiplicity,
        );
    }

    pub(super) fn stamp_reactive_equation_12_block_0(
        &mut self,
        ctx: &GeneratedEvalContext<'_>,
        stamper: &mut GeneratedReactiveStamper<'_>,
        s: &mut ReactiveScratch,
    ) {
        let _ = stamper;
        let p = self.params;
        let nodes = self.nodes;
        let branches = self.branches;
        let nv15 = ctx.node_voltage(nodes[15]);
        let (eq12_e1370, eq12_e1370_d_n0, eq12_e1370_d_n1, eq12_e1370_d_n2, eq12_e1370_d_n3, eq12_e1370_d_n4, eq12_e1370_d_n5, eq12_e1370_d_n6, eq12_e1370_d_n7, eq12_e1370_d_n8, eq12_e1370_d_n9, eq12_e1370_d_n10, eq12_e1370_d_n11, eq12_e1370_d_n12, eq12_e1370_d_n13, eq12_e1370_d_n14, eq12_e1370_d_n15, eq12_e1370_d_n16, eq12_e1370_q, eq12_e1370_q_d_n0, eq12_e1370_q_d_n1, eq12_e1370_q_d_n2, eq12_e1370_q_d_n3, eq12_e1370_q_d_n4, eq12_e1370_q_d_n5, eq12_e1370_q_d_n6, eq12_e1370_q_d_n7, eq12_e1370_q_d_n8, eq12_e1370_q_d_n9, eq12_e1370_q_d_n10, eq12_e1370_q_d_n11, eq12_e1370_q_d_n12, eq12_e1370_q_d_n13, eq12_e1370_q_d_n14, eq12_e1370_q_d_n15, eq12_e1370_q_d_n16,) = {
    if ((s.v[1556] != 0.0) && (!(s.v[1555] != 0.0))) {
        let eq12_e1353: f64 = (1.0 - s.v[57]);
        let eq12_e1353_d_n0: f64 = (-s.dn[57][0]);
        let eq12_e1353_d_n1: f64 = (-s.dn[57][1]);
        let eq12_e1353_d_n2: f64 = (-s.dn[57][2]);
        let eq12_e1353_d_n3: f64 = (-s.dn[57][3]);
        let eq12_e1353_d_n4: f64 = (-s.dn[57][4]);
        let eq12_e1353_d_n5: f64 = (-s.dn[57][5]);
        let eq12_e1353_d_n6: f64 = (-s.dn[57][6]);
        let eq12_e1353_d_n7: f64 = (-s.dn[57][7]);
        let eq12_e1353_d_n8: f64 = (-s.dn[57][8]);
        let eq12_e1353_d_n9: f64 = (-s.dn[57][9]);
        let eq12_e1353_d_n10: f64 = (-s.dn[57][10]);
        let eq12_e1353_d_n11: f64 = (-s.dn[57][11]);
        let eq12_e1353_d_n12: f64 = (-s.dn[57][12]);
        let eq12_e1353_d_n13: f64 = (-s.dn[57][13]);
        let eq12_e1353_d_n14: f64 = (-s.dn[57][14]);
        let eq12_e1353_d_n15: f64 = (-s.dn[57][15]);
        let eq12_e1353_d_n16: f64 = (-s.dn[57][16]);
        let eq12_e1355: f64 = (eq12_e1353 * s.v[378]);
        let eq12_e1355_d_n0: f64 = ((eq12_e1353_d_n0 * s.v[378]) + (eq12_e1353 * s.dn[378][0]));
        let eq12_e1355_d_n1: f64 = ((eq12_e1353_d_n1 * s.v[378]) + (eq12_e1353 * s.dn[378][1]));
        let eq12_e1355_d_n2: f64 = ((eq12_e1353_d_n2 * s.v[378]) + (eq12_e1353 * s.dn[378][2]));
        let eq12_e1355_d_n3: f64 = ((eq12_e1353_d_n3 * s.v[378]) + (eq12_e1353 * s.dn[378][3]));
        let eq12_e1355_d_n4: f64 = ((eq12_e1353_d_n4 * s.v[378]) + (eq12_e1353 * s.dn[378][4]));
        let eq12_e1355_d_n5: f64 = ((eq12_e1353_d_n5 * s.v[378]) + (eq12_e1353 * s.dn[378][5]));
        let eq12_e1355_d_n6: f64 = ((eq12_e1353_d_n6 * s.v[378]) + (eq12_e1353 * s.dn[378][6]));
        let eq12_e1355_d_n7: f64 = ((eq12_e1353_d_n7 * s.v[378]) + (eq12_e1353 * s.dn[378][7]));
        let eq12_e1355_d_n8: f64 = ((eq12_e1353_d_n8 * s.v[378]) + (eq12_e1353 * s.dn[378][8]));
        let eq12_e1355_d_n9: f64 = ((eq12_e1353_d_n9 * s.v[378]) + (eq12_e1353 * s.dn[378][9]));
        let eq12_e1355_d_n10: f64 = ((eq12_e1353_d_n10 * s.v[378]) + (eq12_e1353 * s.dn[378][10]));
        let eq12_e1355_d_n11: f64 = ((eq12_e1353_d_n11 * s.v[378]) + (eq12_e1353 * s.dn[378][11]));
        let eq12_e1355_d_n12: f64 = ((eq12_e1353_d_n12 * s.v[378]) + (eq12_e1353 * s.dn[378][12]));
        let eq12_e1355_d_n13: f64 = ((eq12_e1353_d_n13 * s.v[378]) + (eq12_e1353 * s.dn[378][13]));
        let eq12_e1355_d_n14: f64 = ((eq12_e1353_d_n14 * s.v[378]) + (eq12_e1353 * s.dn[378][14]));
        let eq12_e1355_d_n15: f64 = ((eq12_e1353_d_n15 * s.v[378]) + (eq12_e1353 * s.dn[378][15]));
        let eq12_e1355_d_n16: f64 = ((eq12_e1353_d_n16 * s.v[378]) + (eq12_e1353 * s.dn[378][16]));
        let eq12_e1357: f64 = (eq12_e1355 * s.v[46]);
        let eq12_e1357_d_n0: f64 = (eq12_e1355_d_n0 * s.v[46]);
        let eq12_e1357_d_n1: f64 = (eq12_e1355_d_n1 * s.v[46]);
        let eq12_e1357_d_n2: f64 = (eq12_e1355_d_n2 * s.v[46]);
        let eq12_e1357_d_n3: f64 = (eq12_e1355_d_n3 * s.v[46]);
        let eq12_e1357_d_n4: f64 = (eq12_e1355_d_n4 * s.v[46]);
        let eq12_e1357_d_n5: f64 = (eq12_e1355_d_n5 * s.v[46]);
        let eq12_e1357_d_n6: f64 = (eq12_e1355_d_n6 * s.v[46]);
        let eq12_e1357_d_n7: f64 = (eq12_e1355_d_n7 * s.v[46]);
        let eq12_e1357_d_n8: f64 = (eq12_e1355_d_n8 * s.v[46]);
        let eq12_e1357_d_n9: f64 = (eq12_e1355_d_n9 * s.v[46]);
        let eq12_e1357_d_n10: f64 = (eq12_e1355_d_n10 * s.v[46]);
        let eq12_e1357_d_n11: f64 = (eq12_e1355_d_n11 * s.v[46]);
        let eq12_e1357_d_n12: f64 = (eq12_e1355_d_n12 * s.v[46]);
        let eq12_e1357_d_n13: f64 = (eq12_e1355_d_n13 * s.v[46]);
        let eq12_e1357_d_n14: f64 = (eq12_e1355_d_n14 * s.v[46]);
        let eq12_e1357_d_n15: f64 = (eq12_e1355_d_n15 * s.v[46]);
        let eq12_e1357_d_n16: f64 = (eq12_e1355_d_n16 * s.v[46]);
        let eq12_e1359: f64 = (eq12_e1357 * s.v[29]);
        let eq12_e1359_d_n0: f64 = (eq12_e1357_d_n0 * s.v[29]);
        let eq12_e1359_d_n1: f64 = (eq12_e1357_d_n1 * s.v[29]);
        let eq12_e1359_d_n2: f64 = (eq12_e1357_d_n2 * s.v[29]);
        let eq12_e1359_d_n3: f64 = (eq12_e1357_d_n3 * s.v[29]);
        let eq12_e1359_d_n4: f64 = (eq12_e1357_d_n4 * s.v[29]);
        let eq12_e1359_d_n5: f64 = (eq12_e1357_d_n5 * s.v[29]);
        let eq12_e1359_d_n6: f64 = (eq12_e1357_d_n6 * s.v[29]);
        let eq12_e1359_d_n7: f64 = (eq12_e1357_d_n7 * s.v[29]);
        let eq12_e1359_d_n8: f64 = (eq12_e1357_d_n8 * s.v[29]);
        let eq12_e1359_d_n9: f64 = (eq12_e1357_d_n9 * s.v[29]);
        let eq12_e1359_d_n10: f64 = (eq12_e1357_d_n10 * s.v[29]);
        let eq12_e1359_d_n11: f64 = (eq12_e1357_d_n11 * s.v[29]);
        let eq12_e1359_d_n12: f64 = (eq12_e1357_d_n12 * s.v[29]);
        let eq12_e1359_d_n13: f64 = (eq12_e1357_d_n13 * s.v[29]);
        let eq12_e1359_d_n14: f64 = (eq12_e1357_d_n14 * s.v[29]);
        let eq12_e1359_d_n15: f64 = (eq12_e1357_d_n15 * s.v[29]);
        let eq12_e1359_d_n16: f64 = (eq12_e1357_d_n16 * s.v[29]);
        let eq12_e1361: f64 = (eq12_e1359 * p.p2);
        let eq12_e1361_d_n0: f64 = (eq12_e1359_d_n0 * p.p2);
        let eq12_e1361_d_n1: f64 = (eq12_e1359_d_n1 * p.p2);
        let eq12_e1361_d_n2: f64 = (eq12_e1359_d_n2 * p.p2);
        let eq12_e1361_d_n3: f64 = (eq12_e1359_d_n3 * p.p2);
        let eq12_e1361_d_n4: f64 = (eq12_e1359_d_n4 * p.p2);
        let eq12_e1361_d_n5: f64 = (eq12_e1359_d_n5 * p.p2);
        let eq12_e1361_d_n6: f64 = (eq12_e1359_d_n6 * p.p2);
        let eq12_e1361_d_n7: f64 = (eq12_e1359_d_n7 * p.p2);
        let eq12_e1361_d_n8: f64 = (eq12_e1359_d_n8 * p.p2);
        let eq12_e1361_d_n9: f64 = (eq12_e1359_d_n9 * p.p2);
        let eq12_e1361_d_n10: f64 = (eq12_e1359_d_n10 * p.p2);
        let eq12_e1361_d_n11: f64 = (eq12_e1359_d_n11 * p.p2);
        let eq12_e1361_d_n12: f64 = (eq12_e1359_d_n12 * p.p2);
        let eq12_e1361_d_n13: f64 = (eq12_e1359_d_n13 * p.p2);
        let eq12_e1361_d_n14: f64 = (eq12_e1359_d_n14 * p.p2);
        let eq12_e1361_d_n15: f64 = (eq12_e1359_d_n15 * p.p2);
        let eq12_e1361_d_n16: f64 = (eq12_e1359_d_n16 * p.p2);
        let eq12_e1363: f64 = (eq12_e1361 * s.v[30]);
        let eq12_e1363_d_n0: f64 = (eq12_e1361_d_n0 * s.v[30]);
        let eq12_e1363_d_n1: f64 = (eq12_e1361_d_n1 * s.v[30]);
        let eq12_e1363_d_n2: f64 = (eq12_e1361_d_n2 * s.v[30]);
        let eq12_e1363_d_n3: f64 = (eq12_e1361_d_n3 * s.v[30]);
        let eq12_e1363_d_n4: f64 = (eq12_e1361_d_n4 * s.v[30]);
        let eq12_e1363_d_n5: f64 = (eq12_e1361_d_n5 * s.v[30]);
        let eq12_e1363_d_n6: f64 = (eq12_e1361_d_n6 * s.v[30]);
        let eq12_e1363_d_n7: f64 = (eq12_e1361_d_n7 * s.v[30]);
        let eq12_e1363_d_n8: f64 = (eq12_e1361_d_n8 * s.v[30]);
        let eq12_e1363_d_n9: f64 = (eq12_e1361_d_n9 * s.v[30]);
        let eq12_e1363_d_n10: f64 = (eq12_e1361_d_n10 * s.v[30]);
        let eq12_e1363_d_n11: f64 = (eq12_e1361_d_n11 * s.v[30]);
        let eq12_e1363_d_n12: f64 = (eq12_e1361_d_n12 * s.v[30]);
        let eq12_e1363_d_n13: f64 = (eq12_e1361_d_n13 * s.v[30]);
        let eq12_e1363_d_n14: f64 = (eq12_e1361_d_n14 * s.v[30]);
        let eq12_e1363_d_n15: f64 = (eq12_e1361_d_n15 * s.v[30]);
        let eq12_e1363_d_n16: f64 = (eq12_e1361_d_n16 * s.v[30]);
        let eq12_e1365: f64 = (eq12_e1363 * (nv15 - 0.0));
        let eq12_e1365_d_n0: f64 = (eq12_e1363_d_n0 * (nv15 - 0.0));
        let eq12_e1365_d_n1: f64 = (eq12_e1363_d_n1 * (nv15 - 0.0));
        let eq12_e1365_d_n2: f64 = (eq12_e1363_d_n2 * (nv15 - 0.0));
        let eq12_e1365_d_n3: f64 = (eq12_e1363_d_n3 * (nv15 - 0.0));
        let eq12_e1365_d_n4: f64 = (eq12_e1363_d_n4 * (nv15 - 0.0));
        let eq12_e1365_d_n5: f64 = (eq12_e1363_d_n5 * (nv15 - 0.0));
        let eq12_e1365_d_n6: f64 = (eq12_e1363_d_n6 * (nv15 - 0.0));
        let eq12_e1365_d_n7: f64 = (eq12_e1363_d_n7 * (nv15 - 0.0));
        let eq12_e1365_d_n8: f64 = (eq12_e1363_d_n8 * (nv15 - 0.0));
        let eq12_e1365_d_n9: f64 = (eq12_e1363_d_n9 * (nv15 - 0.0));
        let eq12_e1365_d_n10: f64 = (eq12_e1363_d_n10 * (nv15 - 0.0));
        let eq12_e1365_d_n11: f64 = (eq12_e1363_d_n11 * (nv15 - 0.0));
        let eq12_e1365_d_n12: f64 = (eq12_e1363_d_n12 * (nv15 - 0.0));
        let eq12_e1365_d_n13: f64 = (eq12_e1363_d_n13 * (nv15 - 0.0));
        let eq12_e1365_d_n14: f64 = (eq12_e1363_d_n14 * (nv15 - 0.0));
        let eq12_e1365_d_n15: f64 = ((eq12_e1363_d_n15 * (nv15 - 0.0)) + eq12_e1363);
        let eq12_e1365_d_n16: f64 = (eq12_e1363_d_n16 * (nv15 - 0.0));
        let eq12_e1366: f64 = (0.5 * eq12_e1365);
        let eq12_e1366_d_n0: f64 = (0.5 * eq12_e1365_d_n0);
        let eq12_e1366_d_n1: f64 = (0.5 * eq12_e1365_d_n1);
        let eq12_e1366_d_n2: f64 = (0.5 * eq12_e1365_d_n2);
        let eq12_e1366_d_n3: f64 = (0.5 * eq12_e1365_d_n3);
        let eq12_e1366_d_n4: f64 = (0.5 * eq12_e1365_d_n4);
        let eq12_e1366_d_n5: f64 = (0.5 * eq12_e1365_d_n5);
        let eq12_e1366_d_n6: f64 = (0.5 * eq12_e1365_d_n6);
        let eq12_e1366_d_n7: f64 = (0.5 * eq12_e1365_d_n7);
        let eq12_e1366_d_n8: f64 = (0.5 * eq12_e1365_d_n8);
        let eq12_e1366_d_n9: f64 = (0.5 * eq12_e1365_d_n9);
        let eq12_e1366_d_n10: f64 = (0.5 * eq12_e1365_d_n10);
        let eq12_e1366_d_n11: f64 = (0.5 * eq12_e1365_d_n11);
        let eq12_e1366_d_n12: f64 = (0.5 * eq12_e1365_d_n12);
        let eq12_e1366_d_n13: f64 = (0.5 * eq12_e1365_d_n13);
        let eq12_e1366_d_n14: f64 = (0.5 * eq12_e1365_d_n14);
        let eq12_e1366_d_n15: f64 = (0.5 * eq12_e1365_d_n15);
        let eq12_e1366_d_n16: f64 = (0.5 * eq12_e1365_d_n16);
        let eq12_e1367_q: f64 = eq12_e1366;
        let eq12_e1368: f64 = (p.p29 * eq12_e1366);
        let eq12_e1368_d_n0: f64 = (p.p29 * eq12_e1366_d_n0);
        let eq12_e1368_d_n1: f64 = (p.p29 * eq12_e1366_d_n1);
        let eq12_e1368_d_n2: f64 = (p.p29 * eq12_e1366_d_n2);
        let eq12_e1368_d_n3: f64 = (p.p29 * eq12_e1366_d_n3);
        let eq12_e1368_d_n4: f64 = (p.p29 * eq12_e1366_d_n4);
        let eq12_e1368_d_n5: f64 = (p.p29 * eq12_e1366_d_n5);
        let eq12_e1368_d_n6: f64 = (p.p29 * eq12_e1366_d_n6);
        let eq12_e1368_d_n7: f64 = (p.p29 * eq12_e1366_d_n7);
        let eq12_e1368_d_n8: f64 = (p.p29 * eq12_e1366_d_n8);
        let eq12_e1368_d_n9: f64 = (p.p29 * eq12_e1366_d_n9);
        let eq12_e1368_d_n10: f64 = (p.p29 * eq12_e1366_d_n10);
        let eq12_e1368_d_n11: f64 = (p.p29 * eq12_e1366_d_n11);
        let eq12_e1368_d_n12: f64 = (p.p29 * eq12_e1366_d_n12);
        let eq12_e1368_d_n13: f64 = (p.p29 * eq12_e1366_d_n13);
        let eq12_e1368_d_n14: f64 = (p.p29 * eq12_e1366_d_n14);
        let eq12_e1368_d_n15: f64 = (p.p29 * eq12_e1366_d_n15);
        let eq12_e1368_d_n16: f64 = (p.p29 * eq12_e1366_d_n16);
        let eq12_e1368_q: f64 = (p.p29 * eq12_e1367_q);
        let eq12_e1368_q_d_n0: f64 = (p.p29 * eq12_e1366_d_n0);
        let eq12_e1368_q_d_n1: f64 = (p.p29 * eq12_e1366_d_n1);
        let eq12_e1368_q_d_n2: f64 = (p.p29 * eq12_e1366_d_n2);
        let eq12_e1368_q_d_n3: f64 = (p.p29 * eq12_e1366_d_n3);
        let eq12_e1368_q_d_n4: f64 = (p.p29 * eq12_e1366_d_n4);
        let eq12_e1368_q_d_n5: f64 = (p.p29 * eq12_e1366_d_n5);
        let eq12_e1368_q_d_n6: f64 = (p.p29 * eq12_e1366_d_n6);
        let eq12_e1368_q_d_n7: f64 = (p.p29 * eq12_e1366_d_n7);
        let eq12_e1368_q_d_n8: f64 = (p.p29 * eq12_e1366_d_n8);
        let eq12_e1368_q_d_n9: f64 = (p.p29 * eq12_e1366_d_n9);
        let eq12_e1368_q_d_n10: f64 = (p.p29 * eq12_e1366_d_n10);
        let eq12_e1368_q_d_n11: f64 = (p.p29 * eq12_e1366_d_n11);
        let eq12_e1368_q_d_n12: f64 = (p.p29 * eq12_e1366_d_n12);
        let eq12_e1368_q_d_n13: f64 = (p.p29 * eq12_e1366_d_n13);
        let eq12_e1368_q_d_n14: f64 = (p.p29 * eq12_e1366_d_n14);
        let eq12_e1368_q_d_n15: f64 = (p.p29 * eq12_e1366_d_n15);
        let eq12_e1368_q_d_n16: f64 = (p.p29 * eq12_e1366_d_n16);
        (eq12_e1368, eq12_e1368_d_n0, eq12_e1368_d_n1, eq12_e1368_d_n2, eq12_e1368_d_n3, eq12_e1368_d_n4, eq12_e1368_d_n5, eq12_e1368_d_n6, eq12_e1368_d_n7, eq12_e1368_d_n8, eq12_e1368_d_n9, eq12_e1368_d_n10, eq12_e1368_d_n11, eq12_e1368_d_n12, eq12_e1368_d_n13, eq12_e1368_d_n14, eq12_e1368_d_n15, eq12_e1368_d_n16, eq12_e1368_q, eq12_e1368_q_d_n0, eq12_e1368_q_d_n1, eq12_e1368_q_d_n2, eq12_e1368_q_d_n3, eq12_e1368_q_d_n4, eq12_e1368_q_d_n5, eq12_e1368_q_d_n6, eq12_e1368_q_d_n7, eq12_e1368_q_d_n8, eq12_e1368_q_d_n9, eq12_e1368_q_d_n10, eq12_e1368_q_d_n11, eq12_e1368_q_d_n12, eq12_e1368_q_d_n13, eq12_e1368_q_d_n14, eq12_e1368_q_d_n15, eq12_e1368_q_d_n16,)
    } else {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    }
};
        let eq12_reactive_node_derivatives: [f64; 17] = [eq12_e1370_q_d_n0, eq12_e1370_q_d_n1, eq12_e1370_q_d_n2, eq12_e1370_q_d_n3, eq12_e1370_q_d_n4, eq12_e1370_q_d_n5, eq12_e1370_q_d_n6, eq12_e1370_q_d_n7, eq12_e1370_q_d_n8, eq12_e1370_q_d_n9, eq12_e1370_q_d_n10, eq12_e1370_q_d_n11, eq12_e1370_q_d_n12, eq12_e1370_q_d_n13, eq12_e1370_q_d_n14, eq12_e1370_q_d_n15, eq12_e1370_q_d_n16];
        let eq12_reactive_branch_derivatives: [f64; 0] = [];
        stamper.stamp_current_reactive_dense(
            Some(nodes[9]),
            Some(nodes[5]),
            &nodes,
            &eq12_reactive_node_derivatives,
            &branches,
            &eq12_reactive_branch_derivatives,
            self.multiplicity,
        );
    }

    pub(super) fn stamp_reactive_equation_19_block_0(
        &mut self,
        ctx: &GeneratedEvalContext<'_>,
        stamper: &mut GeneratedReactiveStamper<'_>,
        s: &mut ReactiveScratch,
    ) {
        let _ = stamper;
        let p = self.params;
        let nodes = self.nodes;
        let branches = self.branches;
        let eq19_e1428_q: f64 = s.v[787];
        let eq19_reactive_node_derivatives: [f64; 17] = [s.dn[787][0], s.dn[787][1], s.dn[787][2], s.dn[787][3], s.dn[787][4], s.dn[787][5], s.dn[787][6], s.dn[787][7], s.dn[787][8], s.dn[787][9], s.dn[787][10], s.dn[787][11], s.dn[787][12], s.dn[787][13], s.dn[787][14], s.dn[787][15], s.dn[787][16]];
        let eq19_reactive_branch_derivatives: [f64; 0] = [];
        stamper.stamp_current_reactive_dense(
            Some(nodes[9]),
            Some(nodes[11]),
            &nodes,
            &eq19_reactive_node_derivatives,
            &branches,
            &eq19_reactive_branch_derivatives,
            self.multiplicity,
        );
    }

    pub(super) fn stamp_reactive_equation_20_block_0(
        &mut self,
        ctx: &GeneratedEvalContext<'_>,
        stamper: &mut GeneratedReactiveStamper<'_>,
        s: &mut ReactiveScratch,
    ) {
        let _ = stamper;
        let p = self.params;
        let nodes = self.nodes;
        let branches = self.branches;
        let eq20_e1430_q: f64 = s.v[785];
        let eq20_reactive_node_derivatives: [f64; 17] = [s.dn[785][0], s.dn[785][1], s.dn[785][2], s.dn[785][3], s.dn[785][4], s.dn[785][5], s.dn[785][6], s.dn[785][7], s.dn[785][8], s.dn[785][9], s.dn[785][10], s.dn[785][11], s.dn[785][12], s.dn[785][13], s.dn[785][14], s.dn[785][15], s.dn[785][16]];
        let eq20_reactive_branch_derivatives: [f64; 0] = [];
        stamper.stamp_current_reactive_dense(
            Some(nodes[7]),
            Some(nodes[11]),
            &nodes,
            &eq20_reactive_node_derivatives,
            &branches,
            &eq20_reactive_branch_derivatives,
            self.multiplicity,
        );
    }

    pub(super) fn stamp_reactive_equation_21_block_0(
        &mut self,
        ctx: &GeneratedEvalContext<'_>,
        stamper: &mut GeneratedReactiveStamper<'_>,
        s: &mut ReactiveScratch,
    ) {
        let _ = stamper;
        let p = self.params;
        let nodes = self.nodes;
        let branches = self.branches;
        let eq21_e1432_q: f64 = s.v[786];
        let eq21_reactive_node_derivatives: [f64; 17] = [s.dn[786][0], s.dn[786][1], s.dn[786][2], s.dn[786][3], s.dn[786][4], s.dn[786][5], s.dn[786][6], s.dn[786][7], s.dn[786][8], s.dn[786][9], s.dn[786][10], s.dn[786][11], s.dn[786][12], s.dn[786][13], s.dn[786][14], s.dn[786][15], s.dn[786][16]];
        let eq21_reactive_branch_derivatives: [f64; 0] = [];
        stamper.stamp_current_reactive_dense(
            Some(nodes[5]),
            Some(nodes[11]),
            &nodes,
            &eq21_reactive_node_derivatives,
            &branches,
            &eq21_reactive_branch_derivatives,
            self.multiplicity,
        );
    }

    pub(super) fn stamp_reactive_equation_22_block_0(
        &mut self,
        ctx: &GeneratedEvalContext<'_>,
        stamper: &mut GeneratedReactiveStamper<'_>,
        s: &mut ReactiveScratch,
    ) {
        let _ = stamper;
        let p = self.params;
        let nodes = self.nodes;
        let branches = self.branches;
        let eq22_e1435: f64 = (-s.v[187]);
        let eq22_e1435_d_n0: f64 = (-s.dn[187][0]);
        let eq22_e1435_d_n1: f64 = (-s.dn[187][1]);
        let eq22_e1435_d_n2: f64 = (-s.dn[187][2]);
        let eq22_e1435_d_n3: f64 = (-s.dn[187][3]);
        let eq22_e1435_d_n4: f64 = (-s.dn[187][4]);
        let eq22_e1435_d_n5: f64 = (-s.dn[187][5]);
        let eq22_e1435_d_n6: f64 = (-s.dn[187][6]);
        let eq22_e1435_d_n7: f64 = (-s.dn[187][7]);
        let eq22_e1435_d_n8: f64 = (-s.dn[187][8]);
        let eq22_e1435_d_n9: f64 = (-s.dn[187][9]);
        let eq22_e1435_d_n10: f64 = (-s.dn[187][10]);
        let eq22_e1435_d_n11: f64 = (-s.dn[187][11]);
        let eq22_e1435_d_n12: f64 = (-s.dn[187][12]);
        let eq22_e1435_d_n13: f64 = (-s.dn[187][13]);
        let eq22_e1435_d_n14: f64 = (-s.dn[187][14]);
        let eq22_e1435_d_n15: f64 = (-s.dn[187][15]);
        let eq22_e1435_d_n16: f64 = (-s.dn[187][16]);
        let eq22_e1437: f64 = (eq22_e1435 * s.v[223]);
        let eq22_e1437_d_n0: f64 = ((eq22_e1435_d_n0 * s.v[223]) + (eq22_e1435 * s.dn[223][0]));
        let eq22_e1437_d_n1: f64 = ((eq22_e1435_d_n1 * s.v[223]) + (eq22_e1435 * s.dn[223][1]));
        let eq22_e1437_d_n2: f64 = ((eq22_e1435_d_n2 * s.v[223]) + (eq22_e1435 * s.dn[223][2]));
        let eq22_e1437_d_n3: f64 = ((eq22_e1435_d_n3 * s.v[223]) + (eq22_e1435 * s.dn[223][3]));
        let eq22_e1437_d_n4: f64 = ((eq22_e1435_d_n4 * s.v[223]) + (eq22_e1435 * s.dn[223][4]));
        let eq22_e1437_d_n5: f64 = ((eq22_e1435_d_n5 * s.v[223]) + (eq22_e1435 * s.dn[223][5]));
        let eq22_e1437_d_n6: f64 = ((eq22_e1435_d_n6 * s.v[223]) + (eq22_e1435 * s.dn[223][6]));
        let eq22_e1437_d_n7: f64 = ((eq22_e1435_d_n7 * s.v[223]) + (eq22_e1435 * s.dn[223][7]));
        let eq22_e1437_d_n8: f64 = ((eq22_e1435_d_n8 * s.v[223]) + (eq22_e1435 * s.dn[223][8]));
        let eq22_e1437_d_n9: f64 = ((eq22_e1435_d_n9 * s.v[223]) + (eq22_e1435 * s.dn[223][9]));
        let eq22_e1437_d_n10: f64 = ((eq22_e1435_d_n10 * s.v[223]) + (eq22_e1435 * s.dn[223][10]));
        let eq22_e1437_d_n11: f64 = ((eq22_e1435_d_n11 * s.v[223]) + (eq22_e1435 * s.dn[223][11]));
        let eq22_e1437_d_n12: f64 = ((eq22_e1435_d_n12 * s.v[223]) + (eq22_e1435 * s.dn[223][12]));
        let eq22_e1437_d_n13: f64 = ((eq22_e1435_d_n13 * s.v[223]) + (eq22_e1435 * s.dn[223][13]));
        let eq22_e1437_d_n14: f64 = ((eq22_e1435_d_n14 * s.v[223]) + (eq22_e1435 * s.dn[223][14]));
        let eq22_e1437_d_n15: f64 = ((eq22_e1435_d_n15 * s.v[223]) + (eq22_e1435 * s.dn[223][15]));
        let eq22_e1437_d_n16: f64 = ((eq22_e1435_d_n16 * s.v[223]) + (eq22_e1435 * s.dn[223][16]));
        let eq22_e1438_q: f64 = eq22_e1437;
        let eq22_e1439: f64 = (p.p29 * eq22_e1437);
        let eq22_e1439_d_n0: f64 = (p.p29 * eq22_e1437_d_n0);
        let eq22_e1439_d_n1: f64 = (p.p29 * eq22_e1437_d_n1);
        let eq22_e1439_d_n2: f64 = (p.p29 * eq22_e1437_d_n2);
        let eq22_e1439_d_n3: f64 = (p.p29 * eq22_e1437_d_n3);
        let eq22_e1439_d_n4: f64 = (p.p29 * eq22_e1437_d_n4);
        let eq22_e1439_d_n5: f64 = (p.p29 * eq22_e1437_d_n5);
        let eq22_e1439_d_n6: f64 = (p.p29 * eq22_e1437_d_n6);
        let eq22_e1439_d_n7: f64 = (p.p29 * eq22_e1437_d_n7);
        let eq22_e1439_d_n8: f64 = (p.p29 * eq22_e1437_d_n8);
        let eq22_e1439_d_n9: f64 = (p.p29 * eq22_e1437_d_n9);
        let eq22_e1439_d_n10: f64 = (p.p29 * eq22_e1437_d_n10);
        let eq22_e1439_d_n11: f64 = (p.p29 * eq22_e1437_d_n11);
        let eq22_e1439_d_n12: f64 = (p.p29 * eq22_e1437_d_n12);
        let eq22_e1439_d_n13: f64 = (p.p29 * eq22_e1437_d_n13);
        let eq22_e1439_d_n14: f64 = (p.p29 * eq22_e1437_d_n14);
        let eq22_e1439_d_n15: f64 = (p.p29 * eq22_e1437_d_n15);
        let eq22_e1439_d_n16: f64 = (p.p29 * eq22_e1437_d_n16);
        let eq22_e1439_q: f64 = (p.p29 * eq22_e1438_q);
        let eq22_e1439_q_d_n0: f64 = (p.p29 * eq22_e1437_d_n0);
        let eq22_e1439_q_d_n1: f64 = (p.p29 * eq22_e1437_d_n1);
        let eq22_e1439_q_d_n2: f64 = (p.p29 * eq22_e1437_d_n2);
        let eq22_e1439_q_d_n3: f64 = (p.p29 * eq22_e1437_d_n3);
        let eq22_e1439_q_d_n4: f64 = (p.p29 * eq22_e1437_d_n4);
        let eq22_e1439_q_d_n5: f64 = (p.p29 * eq22_e1437_d_n5);
        let eq22_e1439_q_d_n6: f64 = (p.p29 * eq22_e1437_d_n6);
        let eq22_e1439_q_d_n7: f64 = (p.p29 * eq22_e1437_d_n7);
        let eq22_e1439_q_d_n8: f64 = (p.p29 * eq22_e1437_d_n8);
        let eq22_e1439_q_d_n9: f64 = (p.p29 * eq22_e1437_d_n9);
        let eq22_e1439_q_d_n10: f64 = (p.p29 * eq22_e1437_d_n10);
        let eq22_e1439_q_d_n11: f64 = (p.p29 * eq22_e1437_d_n11);
        let eq22_e1439_q_d_n12: f64 = (p.p29 * eq22_e1437_d_n12);
        let eq22_e1439_q_d_n13: f64 = (p.p29 * eq22_e1437_d_n13);
        let eq22_e1439_q_d_n14: f64 = (p.p29 * eq22_e1437_d_n14);
        let eq22_e1439_q_d_n15: f64 = (p.p29 * eq22_e1437_d_n15);
        let eq22_e1439_q_d_n16: f64 = (p.p29 * eq22_e1437_d_n16);
        let eq22_reactive_node_derivatives: [f64; 17] = [eq22_e1439_q_d_n0, eq22_e1439_q_d_n1, eq22_e1439_q_d_n2, eq22_e1439_q_d_n3, eq22_e1439_q_d_n4, eq22_e1439_q_d_n5, eq22_e1439_q_d_n6, eq22_e1439_q_d_n7, eq22_e1439_q_d_n8, eq22_e1439_q_d_n9, eq22_e1439_q_d_n10, eq22_e1439_q_d_n11, eq22_e1439_q_d_n12, eq22_e1439_q_d_n13, eq22_e1439_q_d_n14, eq22_e1439_q_d_n15, eq22_e1439_q_d_n16];
        let eq22_reactive_branch_derivatives: [f64; 0] = [];
        stamper.stamp_current_reactive_dense(
            Some(nodes[10]),
            Some(nodes[7]),
            &nodes,
            &eq22_reactive_node_derivatives,
            &branches,
            &eq22_reactive_branch_derivatives,
            self.multiplicity,
        );
    }

    pub(super) fn stamp_reactive_equation_23_block_0(
        &mut self,
        ctx: &GeneratedEvalContext<'_>,
        stamper: &mut GeneratedReactiveStamper<'_>,
        s: &mut ReactiveScratch,
    ) {
        let _ = stamper;
        let p = self.params;
        let nodes = self.nodes;
        let branches = self.branches;
        let eq23_e1442: f64 = (-s.v[187]);
        let eq23_e1442_d_n0: f64 = (-s.dn[187][0]);
        let eq23_e1442_d_n1: f64 = (-s.dn[187][1]);
        let eq23_e1442_d_n2: f64 = (-s.dn[187][2]);
        let eq23_e1442_d_n3: f64 = (-s.dn[187][3]);
        let eq23_e1442_d_n4: f64 = (-s.dn[187][4]);
        let eq23_e1442_d_n5: f64 = (-s.dn[187][5]);
        let eq23_e1442_d_n6: f64 = (-s.dn[187][6]);
        let eq23_e1442_d_n7: f64 = (-s.dn[187][7]);
        let eq23_e1442_d_n8: f64 = (-s.dn[187][8]);
        let eq23_e1442_d_n9: f64 = (-s.dn[187][9]);
        let eq23_e1442_d_n10: f64 = (-s.dn[187][10]);
        let eq23_e1442_d_n11: f64 = (-s.dn[187][11]);
        let eq23_e1442_d_n12: f64 = (-s.dn[187][12]);
        let eq23_e1442_d_n13: f64 = (-s.dn[187][13]);
        let eq23_e1442_d_n14: f64 = (-s.dn[187][14]);
        let eq23_e1442_d_n15: f64 = (-s.dn[187][15]);
        let eq23_e1442_d_n16: f64 = (-s.dn[187][16]);
        let eq23_e1444: f64 = (eq23_e1442 * s.v[224]);
        let eq23_e1444_d_n0: f64 = ((eq23_e1442_d_n0 * s.v[224]) + (eq23_e1442 * s.dn[224][0]));
        let eq23_e1444_d_n1: f64 = ((eq23_e1442_d_n1 * s.v[224]) + (eq23_e1442 * s.dn[224][1]));
        let eq23_e1444_d_n2: f64 = ((eq23_e1442_d_n2 * s.v[224]) + (eq23_e1442 * s.dn[224][2]));
        let eq23_e1444_d_n3: f64 = ((eq23_e1442_d_n3 * s.v[224]) + (eq23_e1442 * s.dn[224][3]));
        let eq23_e1444_d_n4: f64 = ((eq23_e1442_d_n4 * s.v[224]) + (eq23_e1442 * s.dn[224][4]));
        let eq23_e1444_d_n5: f64 = ((eq23_e1442_d_n5 * s.v[224]) + (eq23_e1442 * s.dn[224][5]));
        let eq23_e1444_d_n6: f64 = ((eq23_e1442_d_n6 * s.v[224]) + (eq23_e1442 * s.dn[224][6]));
        let eq23_e1444_d_n7: f64 = ((eq23_e1442_d_n7 * s.v[224]) + (eq23_e1442 * s.dn[224][7]));
        let eq23_e1444_d_n8: f64 = ((eq23_e1442_d_n8 * s.v[224]) + (eq23_e1442 * s.dn[224][8]));
        let eq23_e1444_d_n9: f64 = ((eq23_e1442_d_n9 * s.v[224]) + (eq23_e1442 * s.dn[224][9]));
        let eq23_e1444_d_n10: f64 = ((eq23_e1442_d_n10 * s.v[224]) + (eq23_e1442 * s.dn[224][10]));
        let eq23_e1444_d_n11: f64 = ((eq23_e1442_d_n11 * s.v[224]) + (eq23_e1442 * s.dn[224][11]));
        let eq23_e1444_d_n12: f64 = ((eq23_e1442_d_n12 * s.v[224]) + (eq23_e1442 * s.dn[224][12]));
        let eq23_e1444_d_n13: f64 = ((eq23_e1442_d_n13 * s.v[224]) + (eq23_e1442 * s.dn[224][13]));
        let eq23_e1444_d_n14: f64 = ((eq23_e1442_d_n14 * s.v[224]) + (eq23_e1442 * s.dn[224][14]));
        let eq23_e1444_d_n15: f64 = ((eq23_e1442_d_n15 * s.v[224]) + (eq23_e1442 * s.dn[224][15]));
        let eq23_e1444_d_n16: f64 = ((eq23_e1442_d_n16 * s.v[224]) + (eq23_e1442 * s.dn[224][16]));
        let eq23_e1445_q: f64 = eq23_e1444;
        let eq23_e1446: f64 = (p.p29 * eq23_e1444);
        let eq23_e1446_d_n0: f64 = (p.p29 * eq23_e1444_d_n0);
        let eq23_e1446_d_n1: f64 = (p.p29 * eq23_e1444_d_n1);
        let eq23_e1446_d_n2: f64 = (p.p29 * eq23_e1444_d_n2);
        let eq23_e1446_d_n3: f64 = (p.p29 * eq23_e1444_d_n3);
        let eq23_e1446_d_n4: f64 = (p.p29 * eq23_e1444_d_n4);
        let eq23_e1446_d_n5: f64 = (p.p29 * eq23_e1444_d_n5);
        let eq23_e1446_d_n6: f64 = (p.p29 * eq23_e1444_d_n6);
        let eq23_e1446_d_n7: f64 = (p.p29 * eq23_e1444_d_n7);
        let eq23_e1446_d_n8: f64 = (p.p29 * eq23_e1444_d_n8);
        let eq23_e1446_d_n9: f64 = (p.p29 * eq23_e1444_d_n9);
        let eq23_e1446_d_n10: f64 = (p.p29 * eq23_e1444_d_n10);
        let eq23_e1446_d_n11: f64 = (p.p29 * eq23_e1444_d_n11);
        let eq23_e1446_d_n12: f64 = (p.p29 * eq23_e1444_d_n12);
        let eq23_e1446_d_n13: f64 = (p.p29 * eq23_e1444_d_n13);
        let eq23_e1446_d_n14: f64 = (p.p29 * eq23_e1444_d_n14);
        let eq23_e1446_d_n15: f64 = (p.p29 * eq23_e1444_d_n15);
        let eq23_e1446_d_n16: f64 = (p.p29 * eq23_e1444_d_n16);
        let eq23_e1446_q: f64 = (p.p29 * eq23_e1445_q);
        let eq23_e1446_q_d_n0: f64 = (p.p29 * eq23_e1444_d_n0);
        let eq23_e1446_q_d_n1: f64 = (p.p29 * eq23_e1444_d_n1);
        let eq23_e1446_q_d_n2: f64 = (p.p29 * eq23_e1444_d_n2);
        let eq23_e1446_q_d_n3: f64 = (p.p29 * eq23_e1444_d_n3);
        let eq23_e1446_q_d_n4: f64 = (p.p29 * eq23_e1444_d_n4);
        let eq23_e1446_q_d_n5: f64 = (p.p29 * eq23_e1444_d_n5);
        let eq23_e1446_q_d_n6: f64 = (p.p29 * eq23_e1444_d_n6);
        let eq23_e1446_q_d_n7: f64 = (p.p29 * eq23_e1444_d_n7);
        let eq23_e1446_q_d_n8: f64 = (p.p29 * eq23_e1444_d_n8);
        let eq23_e1446_q_d_n9: f64 = (p.p29 * eq23_e1444_d_n9);
        let eq23_e1446_q_d_n10: f64 = (p.p29 * eq23_e1444_d_n10);
        let eq23_e1446_q_d_n11: f64 = (p.p29 * eq23_e1444_d_n11);
        let eq23_e1446_q_d_n12: f64 = (p.p29 * eq23_e1444_d_n12);
        let eq23_e1446_q_d_n13: f64 = (p.p29 * eq23_e1444_d_n13);
        let eq23_e1446_q_d_n14: f64 = (p.p29 * eq23_e1444_d_n14);
        let eq23_e1446_q_d_n15: f64 = (p.p29 * eq23_e1444_d_n15);
        let eq23_e1446_q_d_n16: f64 = (p.p29 * eq23_e1444_d_n16);
        let eq23_reactive_node_derivatives: [f64; 17] = [eq23_e1446_q_d_n0, eq23_e1446_q_d_n1, eq23_e1446_q_d_n2, eq23_e1446_q_d_n3, eq23_e1446_q_d_n4, eq23_e1446_q_d_n5, eq23_e1446_q_d_n6, eq23_e1446_q_d_n7, eq23_e1446_q_d_n8, eq23_e1446_q_d_n9, eq23_e1446_q_d_n10, eq23_e1446_q_d_n11, eq23_e1446_q_d_n12, eq23_e1446_q_d_n13, eq23_e1446_q_d_n14, eq23_e1446_q_d_n15, eq23_e1446_q_d_n16];
        let eq23_reactive_branch_derivatives: [f64; 0] = [];
        stamper.stamp_current_reactive_dense(
            Some(nodes[10]),
            Some(nodes[5]),
            &nodes,
            &eq23_reactive_node_derivatives,
            &branches,
            &eq23_reactive_branch_derivatives,
            self.multiplicity,
        );
    }

    pub(super) fn stamp_reactive_equation_24_block_0(
        &mut self,
        ctx: &GeneratedEvalContext<'_>,
        stamper: &mut GeneratedReactiveStamper<'_>,
        s: &mut ReactiveScratch,
    ) {
        let _ = stamper;
        let p = self.params;
        let nodes = self.nodes;
        let branches = self.branches;
        let eq24_e1449: f64 = (-s.v[187]);
        let eq24_e1449_d_n0: f64 = (-s.dn[187][0]);
        let eq24_e1449_d_n1: f64 = (-s.dn[187][1]);
        let eq24_e1449_d_n2: f64 = (-s.dn[187][2]);
        let eq24_e1449_d_n3: f64 = (-s.dn[187][3]);
        let eq24_e1449_d_n4: f64 = (-s.dn[187][4]);
        let eq24_e1449_d_n5: f64 = (-s.dn[187][5]);
        let eq24_e1449_d_n6: f64 = (-s.dn[187][6]);
        let eq24_e1449_d_n7: f64 = (-s.dn[187][7]);
        let eq24_e1449_d_n8: f64 = (-s.dn[187][8]);
        let eq24_e1449_d_n9: f64 = (-s.dn[187][9]);
        let eq24_e1449_d_n10: f64 = (-s.dn[187][10]);
        let eq24_e1449_d_n11: f64 = (-s.dn[187][11]);
        let eq24_e1449_d_n12: f64 = (-s.dn[187][12]);
        let eq24_e1449_d_n13: f64 = (-s.dn[187][13]);
        let eq24_e1449_d_n14: f64 = (-s.dn[187][14]);
        let eq24_e1449_d_n15: f64 = (-s.dn[187][15]);
        let eq24_e1449_d_n16: f64 = (-s.dn[187][16]);
        let eq24_e1451: f64 = (eq24_e1449 * s.v[221]);
        let eq24_e1451_d_n0: f64 = ((eq24_e1449_d_n0 * s.v[221]) + (eq24_e1449 * s.dn[221][0]));
        let eq24_e1451_d_n1: f64 = ((eq24_e1449_d_n1 * s.v[221]) + (eq24_e1449 * s.dn[221][1]));
        let eq24_e1451_d_n2: f64 = ((eq24_e1449_d_n2 * s.v[221]) + (eq24_e1449 * s.dn[221][2]));
        let eq24_e1451_d_n3: f64 = ((eq24_e1449_d_n3 * s.v[221]) + (eq24_e1449 * s.dn[221][3]));
        let eq24_e1451_d_n4: f64 = ((eq24_e1449_d_n4 * s.v[221]) + (eq24_e1449 * s.dn[221][4]));
        let eq24_e1451_d_n5: f64 = ((eq24_e1449_d_n5 * s.v[221]) + (eq24_e1449 * s.dn[221][5]));
        let eq24_e1451_d_n6: f64 = ((eq24_e1449_d_n6 * s.v[221]) + (eq24_e1449 * s.dn[221][6]));
        let eq24_e1451_d_n7: f64 = ((eq24_e1449_d_n7 * s.v[221]) + (eq24_e1449 * s.dn[221][7]));
        let eq24_e1451_d_n8: f64 = ((eq24_e1449_d_n8 * s.v[221]) + (eq24_e1449 * s.dn[221][8]));
        let eq24_e1451_d_n9: f64 = ((eq24_e1449_d_n9 * s.v[221]) + (eq24_e1449 * s.dn[221][9]));
        let eq24_e1451_d_n10: f64 = ((eq24_e1449_d_n10 * s.v[221]) + (eq24_e1449 * s.dn[221][10]));
        let eq24_e1451_d_n11: f64 = ((eq24_e1449_d_n11 * s.v[221]) + (eq24_e1449 * s.dn[221][11]));
        let eq24_e1451_d_n12: f64 = ((eq24_e1449_d_n12 * s.v[221]) + (eq24_e1449 * s.dn[221][12]));
        let eq24_e1451_d_n13: f64 = ((eq24_e1449_d_n13 * s.v[221]) + (eq24_e1449 * s.dn[221][13]));
        let eq24_e1451_d_n14: f64 = ((eq24_e1449_d_n14 * s.v[221]) + (eq24_e1449 * s.dn[221][14]));
        let eq24_e1451_d_n15: f64 = ((eq24_e1449_d_n15 * s.v[221]) + (eq24_e1449 * s.dn[221][15]));
        let eq24_e1451_d_n16: f64 = ((eq24_e1449_d_n16 * s.v[221]) + (eq24_e1449 * s.dn[221][16]));
        let eq24_e1452_q: f64 = eq24_e1451;
        let eq24_e1453: f64 = (p.p29 * eq24_e1451);
        let eq24_e1453_d_n0: f64 = (p.p29 * eq24_e1451_d_n0);
        let eq24_e1453_d_n1: f64 = (p.p29 * eq24_e1451_d_n1);
        let eq24_e1453_d_n2: f64 = (p.p29 * eq24_e1451_d_n2);
        let eq24_e1453_d_n3: f64 = (p.p29 * eq24_e1451_d_n3);
        let eq24_e1453_d_n4: f64 = (p.p29 * eq24_e1451_d_n4);
        let eq24_e1453_d_n5: f64 = (p.p29 * eq24_e1451_d_n5);
        let eq24_e1453_d_n6: f64 = (p.p29 * eq24_e1451_d_n6);
        let eq24_e1453_d_n7: f64 = (p.p29 * eq24_e1451_d_n7);
        let eq24_e1453_d_n8: f64 = (p.p29 * eq24_e1451_d_n8);
        let eq24_e1453_d_n9: f64 = (p.p29 * eq24_e1451_d_n9);
        let eq24_e1453_d_n10: f64 = (p.p29 * eq24_e1451_d_n10);
        let eq24_e1453_d_n11: f64 = (p.p29 * eq24_e1451_d_n11);
        let eq24_e1453_d_n12: f64 = (p.p29 * eq24_e1451_d_n12);
        let eq24_e1453_d_n13: f64 = (p.p29 * eq24_e1451_d_n13);
        let eq24_e1453_d_n14: f64 = (p.p29 * eq24_e1451_d_n14);
        let eq24_e1453_d_n15: f64 = (p.p29 * eq24_e1451_d_n15);
        let eq24_e1453_d_n16: f64 = (p.p29 * eq24_e1451_d_n16);
        let eq24_e1453_q: f64 = (p.p29 * eq24_e1452_q);
        let eq24_e1453_q_d_n0: f64 = (p.p29 * eq24_e1451_d_n0);
        let eq24_e1453_q_d_n1: f64 = (p.p29 * eq24_e1451_d_n1);
        let eq24_e1453_q_d_n2: f64 = (p.p29 * eq24_e1451_d_n2);
        let eq24_e1453_q_d_n3: f64 = (p.p29 * eq24_e1451_d_n3);
        let eq24_e1453_q_d_n4: f64 = (p.p29 * eq24_e1451_d_n4);
        let eq24_e1453_q_d_n5: f64 = (p.p29 * eq24_e1451_d_n5);
        let eq24_e1453_q_d_n6: f64 = (p.p29 * eq24_e1451_d_n6);
        let eq24_e1453_q_d_n7: f64 = (p.p29 * eq24_e1451_d_n7);
        let eq24_e1453_q_d_n8: f64 = (p.p29 * eq24_e1451_d_n8);
        let eq24_e1453_q_d_n9: f64 = (p.p29 * eq24_e1451_d_n9);
        let eq24_e1453_q_d_n10: f64 = (p.p29 * eq24_e1451_d_n10);
        let eq24_e1453_q_d_n11: f64 = (p.p29 * eq24_e1451_d_n11);
        let eq24_e1453_q_d_n12: f64 = (p.p29 * eq24_e1451_d_n12);
        let eq24_e1453_q_d_n13: f64 = (p.p29 * eq24_e1451_d_n13);
        let eq24_e1453_q_d_n14: f64 = (p.p29 * eq24_e1451_d_n14);
        let eq24_e1453_q_d_n15: f64 = (p.p29 * eq24_e1451_d_n15);
        let eq24_e1453_q_d_n16: f64 = (p.p29 * eq24_e1451_d_n16);
        let eq24_reactive_node_derivatives: [f64; 17] = [eq24_e1453_q_d_n0, eq24_e1453_q_d_n1, eq24_e1453_q_d_n2, eq24_e1453_q_d_n3, eq24_e1453_q_d_n4, eq24_e1453_q_d_n5, eq24_e1453_q_d_n6, eq24_e1453_q_d_n7, eq24_e1453_q_d_n8, eq24_e1453_q_d_n9, eq24_e1453_q_d_n10, eq24_e1453_q_d_n11, eq24_e1453_q_d_n12, eq24_e1453_q_d_n13, eq24_e1453_q_d_n14, eq24_e1453_q_d_n15, eq24_e1453_q_d_n16];
        let eq24_reactive_branch_derivatives: [f64; 0] = [];
        stamper.stamp_current_reactive_dense(
            Some(nodes[10]),
            Some(nodes[11]),
            &nodes,
            &eq24_reactive_node_derivatives,
            &branches,
            &eq24_reactive_branch_derivatives,
            self.multiplicity,
        );
    }

    pub(super) fn stamp_reactive_equation_55_block_0(
        &mut self,
        ctx: &GeneratedEvalContext<'_>,
        stamper: &mut GeneratedReactiveStamper<'_>,
        s: &mut ReactiveScratch,
    ) {
        let _ = stamper;
        let p = self.params;
        let nodes = self.nodes;
        let branches = self.branches;
        let (eq55_e1708, eq55_e1708_d_n0, eq55_e1708_d_n1, eq55_e1708_d_n2, eq55_e1708_d_n3, eq55_e1708_d_n4, eq55_e1708_d_n5, eq55_e1708_d_n6, eq55_e1708_d_n7, eq55_e1708_d_n8, eq55_e1708_d_n9, eq55_e1708_d_n10, eq55_e1708_d_n11, eq55_e1708_d_n12, eq55_e1708_d_n13, eq55_e1708_d_n14, eq55_e1708_d_n15, eq55_e1708_d_n16, eq55_e1708_q, eq55_e1708_q_d_n0, eq55_e1708_q_d_n1, eq55_e1708_q_d_n2, eq55_e1708_q_d_n3, eq55_e1708_q_d_n4, eq55_e1708_q_d_n5, eq55_e1708_q_d_n6, eq55_e1708_q_d_n7, eq55_e1708_q_d_n8, eq55_e1708_q_d_n9, eq55_e1708_q_d_n10, eq55_e1708_q_d_n11, eq55_e1708_q_d_n12, eq55_e1708_q_d_n13, eq55_e1708_q_d_n14, eq55_e1708_q_d_n15, eq55_e1708_q_d_n16,) = {
    if (s.v[1621] != 0.0) {
        let eq55_e1699: f64 = (s.v[390] * s.v[747]);
        let eq55_e1699_d_n0: f64 = ((s.dn[390][0] * s.v[747]) + (s.v[390] * s.dn[747][0]));
        let eq55_e1699_d_n1: f64 = ((s.dn[390][1] * s.v[747]) + (s.v[390] * s.dn[747][1]));
        let eq55_e1699_d_n2: f64 = ((s.dn[390][2] * s.v[747]) + (s.v[390] * s.dn[747][2]));
        let eq55_e1699_d_n3: f64 = ((s.dn[390][3] * s.v[747]) + (s.v[390] * s.dn[747][3]));
        let eq55_e1699_d_n4: f64 = ((s.dn[390][4] * s.v[747]) + (s.v[390] * s.dn[747][4]));
        let eq55_e1699_d_n5: f64 = ((s.dn[390][5] * s.v[747]) + (s.v[390] * s.dn[747][5]));
        let eq55_e1699_d_n6: f64 = ((s.dn[390][6] * s.v[747]) + (s.v[390] * s.dn[747][6]));
        let eq55_e1699_d_n7: f64 = ((s.dn[390][7] * s.v[747]) + (s.v[390] * s.dn[747][7]));
        let eq55_e1699_d_n8: f64 = ((s.dn[390][8] * s.v[747]) + (s.v[390] * s.dn[747][8]));
        let eq55_e1699_d_n9: f64 = ((s.dn[390][9] * s.v[747]) + (s.v[390] * s.dn[747][9]));
        let eq55_e1699_d_n10: f64 = ((s.dn[390][10] * s.v[747]) + (s.v[390] * s.dn[747][10]));
        let eq55_e1699_d_n11: f64 = ((s.dn[390][11] * s.v[747]) + (s.v[390] * s.dn[747][11]));
        let eq55_e1699_d_n12: f64 = ((s.dn[390][12] * s.v[747]) + (s.v[390] * s.dn[747][12]));
        let eq55_e1699_d_n13: f64 = ((s.dn[390][13] * s.v[747]) + (s.v[390] * s.dn[747][13]));
        let eq55_e1699_d_n14: f64 = ((s.dn[390][14] * s.v[747]) + (s.v[390] * s.dn[747][14]));
        let eq55_e1699_d_n15: f64 = ((s.dn[390][15] * s.v[747]) + (s.v[390] * s.dn[747][15]));
        let eq55_e1699_d_n16: f64 = ((s.dn[390][16] * s.v[747]) + (s.v[390] * s.dn[747][16]));
        let eq55_e1702: f64 = (s.v[390] * s.v[748]);
        let eq55_e1702_d_n0: f64 = ((s.dn[390][0] * s.v[748]) + (s.v[390] * s.dn[748][0]));
        let eq55_e1702_d_n1: f64 = ((s.dn[390][1] * s.v[748]) + (s.v[390] * s.dn[748][1]));
        let eq55_e1702_d_n2: f64 = ((s.dn[390][2] * s.v[748]) + (s.v[390] * s.dn[748][2]));
        let eq55_e1702_d_n3: f64 = ((s.dn[390][3] * s.v[748]) + (s.v[390] * s.dn[748][3]));
        let eq55_e1702_d_n4: f64 = ((s.dn[390][4] * s.v[748]) + (s.v[390] * s.dn[748][4]));
        let eq55_e1702_d_n5: f64 = ((s.dn[390][5] * s.v[748]) + (s.v[390] * s.dn[748][5]));
        let eq55_e1702_d_n6: f64 = ((s.dn[390][6] * s.v[748]) + (s.v[390] * s.dn[748][6]));
        let eq55_e1702_d_n7: f64 = ((s.dn[390][7] * s.v[748]) + (s.v[390] * s.dn[748][7]));
        let eq55_e1702_d_n8: f64 = ((s.dn[390][8] * s.v[748]) + (s.v[390] * s.dn[748][8]));
        let eq55_e1702_d_n9: f64 = ((s.dn[390][9] * s.v[748]) + (s.v[390] * s.dn[748][9]));
        let eq55_e1702_d_n10: f64 = ((s.dn[390][10] * s.v[748]) + (s.v[390] * s.dn[748][10]));
        let eq55_e1702_d_n11: f64 = ((s.dn[390][11] * s.v[748]) + (s.v[390] * s.dn[748][11]));
        let eq55_e1702_d_n12: f64 = ((s.dn[390][12] * s.v[748]) + (s.v[390] * s.dn[748][12]));
        let eq55_e1702_d_n13: f64 = ((s.dn[390][13] * s.v[748]) + (s.v[390] * s.dn[748][13]));
        let eq55_e1702_d_n14: f64 = ((s.dn[390][14] * s.v[748]) + (s.v[390] * s.dn[748][14]));
        let eq55_e1702_d_n15: f64 = ((s.dn[390][15] * s.v[748]) + (s.v[390] * s.dn[748][15]));
        let eq55_e1702_d_n16: f64 = ((s.dn[390][16] * s.v[748]) + (s.v[390] * s.dn[748][16]));
        let eq55_e1703_q: f64 = eq55_e1702;
        let eq55_e1704: f64 = (eq55_e1699 + eq55_e1702);
        let eq55_e1704_d_n0: f64 = (eq55_e1699_d_n0 + eq55_e1702_d_n0);
        let eq55_e1704_d_n1: f64 = (eq55_e1699_d_n1 + eq55_e1702_d_n1);
        let eq55_e1704_d_n2: f64 = (eq55_e1699_d_n2 + eq55_e1702_d_n2);
        let eq55_e1704_d_n3: f64 = (eq55_e1699_d_n3 + eq55_e1702_d_n3);
        let eq55_e1704_d_n4: f64 = (eq55_e1699_d_n4 + eq55_e1702_d_n4);
        let eq55_e1704_d_n5: f64 = (eq55_e1699_d_n5 + eq55_e1702_d_n5);
        let eq55_e1704_d_n6: f64 = (eq55_e1699_d_n6 + eq55_e1702_d_n6);
        let eq55_e1704_d_n7: f64 = (eq55_e1699_d_n7 + eq55_e1702_d_n7);
        let eq55_e1704_d_n8: f64 = (eq55_e1699_d_n8 + eq55_e1702_d_n8);
        let eq55_e1704_d_n9: f64 = (eq55_e1699_d_n9 + eq55_e1702_d_n9);
        let eq55_e1704_d_n10: f64 = (eq55_e1699_d_n10 + eq55_e1702_d_n10);
        let eq55_e1704_d_n11: f64 = (eq55_e1699_d_n11 + eq55_e1702_d_n11);
        let eq55_e1704_d_n12: f64 = (eq55_e1699_d_n12 + eq55_e1702_d_n12);
        let eq55_e1704_d_n13: f64 = (eq55_e1699_d_n13 + eq55_e1702_d_n13);
        let eq55_e1704_d_n14: f64 = (eq55_e1699_d_n14 + eq55_e1702_d_n14);
        let eq55_e1704_d_n15: f64 = (eq55_e1699_d_n15 + eq55_e1702_d_n15);
        let eq55_e1704_d_n16: f64 = (eq55_e1699_d_n16 + eq55_e1702_d_n16);
        let eq55_e1704_q: f64 = eq55_e1703_q;
        let eq55_e1706: f64 = (eq55_e1704 - s.v[749]);
        let eq55_e1706_d_n0: f64 = (eq55_e1704_d_n0 - s.dn[749][0]);
        let eq55_e1706_d_n1: f64 = (eq55_e1704_d_n1 - s.dn[749][1]);
        let eq55_e1706_d_n2: f64 = (eq55_e1704_d_n2 - s.dn[749][2]);
        let eq55_e1706_d_n3: f64 = (eq55_e1704_d_n3 - s.dn[749][3]);
        let eq55_e1706_d_n4: f64 = (eq55_e1704_d_n4 - s.dn[749][4]);
        let eq55_e1706_d_n5: f64 = (eq55_e1704_d_n5 - s.dn[749][5]);
        let eq55_e1706_d_n6: f64 = (eq55_e1704_d_n6 - s.dn[749][6]);
        let eq55_e1706_d_n7: f64 = (eq55_e1704_d_n7 - s.dn[749][7]);
        let eq55_e1706_d_n8: f64 = (eq55_e1704_d_n8 - s.dn[749][8]);
        let eq55_e1706_d_n9: f64 = (eq55_e1704_d_n9 - s.dn[749][9]);
        let eq55_e1706_d_n10: f64 = (eq55_e1704_d_n10 - s.dn[749][10]);
        let eq55_e1706_d_n11: f64 = (eq55_e1704_d_n11 - s.dn[749][11]);
        let eq55_e1706_d_n12: f64 = (eq55_e1704_d_n12 - s.dn[749][12]);
        let eq55_e1706_d_n13: f64 = (eq55_e1704_d_n13 - s.dn[749][13]);
        let eq55_e1706_d_n14: f64 = (eq55_e1704_d_n14 - s.dn[749][14]);
        let eq55_e1706_d_n15: f64 = (eq55_e1704_d_n15 - s.dn[749][15]);
        let eq55_e1706_d_n16: f64 = (eq55_e1704_d_n16 - s.dn[749][16]);
        let eq55_e1706_q: f64 = eq55_e1704_q;
        (eq55_e1706, eq55_e1706_d_n0, eq55_e1706_d_n1, eq55_e1706_d_n2, eq55_e1706_d_n3, eq55_e1706_d_n4, eq55_e1706_d_n5, eq55_e1706_d_n6, eq55_e1706_d_n7, eq55_e1706_d_n8, eq55_e1706_d_n9, eq55_e1706_d_n10, eq55_e1706_d_n11, eq55_e1706_d_n12, eq55_e1706_d_n13, eq55_e1706_d_n14, eq55_e1706_d_n15, eq55_e1706_d_n16, eq55_e1706_q, eq55_e1702_d_n0, eq55_e1702_d_n1, eq55_e1702_d_n2, eq55_e1702_d_n3, eq55_e1702_d_n4, eq55_e1702_d_n5, eq55_e1702_d_n6, eq55_e1702_d_n7, eq55_e1702_d_n8, eq55_e1702_d_n9, eq55_e1702_d_n10, eq55_e1702_d_n11, eq55_e1702_d_n12, eq55_e1702_d_n13, eq55_e1702_d_n14, eq55_e1702_d_n15, eq55_e1702_d_n16,)
    } else {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    }
};
        let eq55_reactive_node_derivatives: [f64; 17] = [eq55_e1708_q_d_n0, eq55_e1708_q_d_n1, eq55_e1708_q_d_n2, eq55_e1708_q_d_n3, eq55_e1708_q_d_n4, eq55_e1708_q_d_n5, eq55_e1708_q_d_n6, eq55_e1708_q_d_n7, eq55_e1708_q_d_n8, eq55_e1708_q_d_n9, eq55_e1708_q_d_n10, eq55_e1708_q_d_n11, eq55_e1708_q_d_n12, eq55_e1708_q_d_n13, eq55_e1708_q_d_n14, eq55_e1708_q_d_n15, eq55_e1708_q_d_n16];
        let eq55_reactive_branch_derivatives: [f64; 0] = [];
        stamper.stamp_current_reactive_dense(
            Some(nodes[4]),
            None,
            &nodes,
            &eq55_reactive_node_derivatives,
            &branches,
            &eq55_reactive_branch_derivatives,
            self.multiplicity,
        );
    }

    pub(super) fn stamp_reactive_equation_71_block_0(
        &mut self,
        ctx: &GeneratedEvalContext<'_>,
        stamper: &mut GeneratedReactiveStamper<'_>,
        s: &mut ReactiveScratch,
    ) {
        let _ = stamper;
        let p = self.params;
        let nodes = self.nodes;
        let branches = self.branches;
        let (eq71_e1841, eq71_e1841_d_n0, eq71_e1841_d_n1, eq71_e1841_d_n2, eq71_e1841_d_n3, eq71_e1841_d_n4, eq71_e1841_d_n5, eq71_e1841_d_n6, eq71_e1841_d_n7, eq71_e1841_d_n8, eq71_e1841_d_n9, eq71_e1841_d_n10, eq71_e1841_d_n11, eq71_e1841_d_n12, eq71_e1841_d_n13, eq71_e1841_d_n14, eq71_e1841_d_n15, eq71_e1841_d_n16, eq71_e1841_q, eq71_e1841_q_d_n0, eq71_e1841_q_d_n1, eq71_e1841_q_d_n2, eq71_e1841_q_d_n3, eq71_e1841_q_d_n4, eq71_e1841_q_d_n5, eq71_e1841_q_d_n6, eq71_e1841_q_d_n7, eq71_e1841_q_d_n8, eq71_e1841_q_d_n9, eq71_e1841_q_d_n10, eq71_e1841_q_d_n11, eq71_e1841_q_d_n12, eq71_e1841_q_d_n13, eq71_e1841_q_d_n14, eq71_e1841_q_d_n15, eq71_e1841_q_d_n16,) = {
    if (s.v[1627] != 0.0) {
        let eq71_e1837: f64 = (p.p29 * s.v[330]);
        let eq71_e1837_d_n0: f64 = (p.p29 * s.dn[330][0]);
        let eq71_e1837_d_n1: f64 = (p.p29 * s.dn[330][1]);
        let eq71_e1837_d_n2: f64 = (p.p29 * s.dn[330][2]);
        let eq71_e1837_d_n3: f64 = (p.p29 * s.dn[330][3]);
        let eq71_e1837_d_n4: f64 = (p.p29 * s.dn[330][4]);
        let eq71_e1837_d_n5: f64 = (p.p29 * s.dn[330][5]);
        let eq71_e1837_d_n6: f64 = (p.p29 * s.dn[330][6]);
        let eq71_e1837_d_n7: f64 = (p.p29 * s.dn[330][7]);
        let eq71_e1837_d_n8: f64 = (p.p29 * s.dn[330][8]);
        let eq71_e1837_d_n9: f64 = (p.p29 * s.dn[330][9]);
        let eq71_e1837_d_n10: f64 = (p.p29 * s.dn[330][10]);
        let eq71_e1837_d_n11: f64 = (p.p29 * s.dn[330][11]);
        let eq71_e1837_d_n12: f64 = (p.p29 * s.dn[330][12]);
        let eq71_e1837_d_n13: f64 = (p.p29 * s.dn[330][13]);
        let eq71_e1837_d_n14: f64 = (p.p29 * s.dn[330][14]);
        let eq71_e1837_d_n15: f64 = (p.p29 * s.dn[330][15]);
        let eq71_e1837_d_n16: f64 = (p.p29 * s.dn[330][16]);
        let eq71_e1838_q: f64 = eq71_e1837;
        let eq71_e1839: f64 = (s.v[187] * eq71_e1837);
        let eq71_e1839_d_n0: f64 = ((s.dn[187][0] * eq71_e1837) + (s.v[187] * eq71_e1837_d_n0));
        let eq71_e1839_d_n1: f64 = ((s.dn[187][1] * eq71_e1837) + (s.v[187] * eq71_e1837_d_n1));
        let eq71_e1839_d_n2: f64 = ((s.dn[187][2] * eq71_e1837) + (s.v[187] * eq71_e1837_d_n2));
        let eq71_e1839_d_n3: f64 = ((s.dn[187][3] * eq71_e1837) + (s.v[187] * eq71_e1837_d_n3));
        let eq71_e1839_d_n4: f64 = ((s.dn[187][4] * eq71_e1837) + (s.v[187] * eq71_e1837_d_n4));
        let eq71_e1839_d_n5: f64 = ((s.dn[187][5] * eq71_e1837) + (s.v[187] * eq71_e1837_d_n5));
        let eq71_e1839_d_n6: f64 = ((s.dn[187][6] * eq71_e1837) + (s.v[187] * eq71_e1837_d_n6));
        let eq71_e1839_d_n7: f64 = ((s.dn[187][7] * eq71_e1837) + (s.v[187] * eq71_e1837_d_n7));
        let eq71_e1839_d_n8: f64 = ((s.dn[187][8] * eq71_e1837) + (s.v[187] * eq71_e1837_d_n8));
        let eq71_e1839_d_n9: f64 = ((s.dn[187][9] * eq71_e1837) + (s.v[187] * eq71_e1837_d_n9));
        let eq71_e1839_d_n10: f64 = ((s.dn[187][10] * eq71_e1837) + (s.v[187] * eq71_e1837_d_n10));
        let eq71_e1839_d_n11: f64 = ((s.dn[187][11] * eq71_e1837) + (s.v[187] * eq71_e1837_d_n11));
        let eq71_e1839_d_n12: f64 = ((s.dn[187][12] * eq71_e1837) + (s.v[187] * eq71_e1837_d_n12));
        let eq71_e1839_d_n13: f64 = ((s.dn[187][13] * eq71_e1837) + (s.v[187] * eq71_e1837_d_n13));
        let eq71_e1839_d_n14: f64 = ((s.dn[187][14] * eq71_e1837) + (s.v[187] * eq71_e1837_d_n14));
        let eq71_e1839_d_n15: f64 = ((s.dn[187][15] * eq71_e1837) + (s.v[187] * eq71_e1837_d_n15));
        let eq71_e1839_d_n16: f64 = ((s.dn[187][16] * eq71_e1837) + (s.v[187] * eq71_e1837_d_n16));
        let eq71_e1839_q: f64 = (s.v[187] * eq71_e1838_q);
        let eq71_e1839_q_d_n0: f64 = ((s.dn[187][0] * eq71_e1838_q) + (s.v[187] * eq71_e1837_d_n0));
        let eq71_e1839_q_d_n1: f64 = ((s.dn[187][1] * eq71_e1838_q) + (s.v[187] * eq71_e1837_d_n1));
        let eq71_e1839_q_d_n2: f64 = ((s.dn[187][2] * eq71_e1838_q) + (s.v[187] * eq71_e1837_d_n2));
        let eq71_e1839_q_d_n3: f64 = ((s.dn[187][3] * eq71_e1838_q) + (s.v[187] * eq71_e1837_d_n3));
        let eq71_e1839_q_d_n4: f64 = ((s.dn[187][4] * eq71_e1838_q) + (s.v[187] * eq71_e1837_d_n4));
        let eq71_e1839_q_d_n5: f64 = ((s.dn[187][5] * eq71_e1838_q) + (s.v[187] * eq71_e1837_d_n5));
        let eq71_e1839_q_d_n6: f64 = ((s.dn[187][6] * eq71_e1838_q) + (s.v[187] * eq71_e1837_d_n6));
        let eq71_e1839_q_d_n7: f64 = ((s.dn[187][7] * eq71_e1838_q) + (s.v[187] * eq71_e1837_d_n7));
        let eq71_e1839_q_d_n8: f64 = ((s.dn[187][8] * eq71_e1838_q) + (s.v[187] * eq71_e1837_d_n8));
        let eq71_e1839_q_d_n9: f64 = ((s.dn[187][9] * eq71_e1838_q) + (s.v[187] * eq71_e1837_d_n9));
        let eq71_e1839_q_d_n10: f64 = ((s.dn[187][10] * eq71_e1838_q) + (s.v[187] * eq71_e1837_d_n10));
        let eq71_e1839_q_d_n11: f64 = ((s.dn[187][11] * eq71_e1838_q) + (s.v[187] * eq71_e1837_d_n11));
        let eq71_e1839_q_d_n12: f64 = ((s.dn[187][12] * eq71_e1838_q) + (s.v[187] * eq71_e1837_d_n12));
        let eq71_e1839_q_d_n13: f64 = ((s.dn[187][13] * eq71_e1838_q) + (s.v[187] * eq71_e1837_d_n13));
        let eq71_e1839_q_d_n14: f64 = ((s.dn[187][14] * eq71_e1838_q) + (s.v[187] * eq71_e1837_d_n14));
        let eq71_e1839_q_d_n15: f64 = ((s.dn[187][15] * eq71_e1838_q) + (s.v[187] * eq71_e1837_d_n15));
        let eq71_e1839_q_d_n16: f64 = ((s.dn[187][16] * eq71_e1838_q) + (s.v[187] * eq71_e1837_d_n16));
        (eq71_e1839, eq71_e1839_d_n0, eq71_e1839_d_n1, eq71_e1839_d_n2, eq71_e1839_d_n3, eq71_e1839_d_n4, eq71_e1839_d_n5, eq71_e1839_d_n6, eq71_e1839_d_n7, eq71_e1839_d_n8, eq71_e1839_d_n9, eq71_e1839_d_n10, eq71_e1839_d_n11, eq71_e1839_d_n12, eq71_e1839_d_n13, eq71_e1839_d_n14, eq71_e1839_d_n15, eq71_e1839_d_n16, eq71_e1839_q, eq71_e1839_q_d_n0, eq71_e1839_q_d_n1, eq71_e1839_q_d_n2, eq71_e1839_q_d_n3, eq71_e1839_q_d_n4, eq71_e1839_q_d_n5, eq71_e1839_q_d_n6, eq71_e1839_q_d_n7, eq71_e1839_q_d_n8, eq71_e1839_q_d_n9, eq71_e1839_q_d_n10, eq71_e1839_q_d_n11, eq71_e1839_q_d_n12, eq71_e1839_q_d_n13, eq71_e1839_q_d_n14, eq71_e1839_q_d_n15, eq71_e1839_q_d_n16,)
    } else {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    }
};
        let eq71_reactive_node_derivatives: [f64; 17] = [eq71_e1841_q_d_n0, eq71_e1841_q_d_n1, eq71_e1841_q_d_n2, eq71_e1841_q_d_n3, eq71_e1841_q_d_n4, eq71_e1841_q_d_n5, eq71_e1841_q_d_n6, eq71_e1841_q_d_n7, eq71_e1841_q_d_n8, eq71_e1841_q_d_n9, eq71_e1841_q_d_n10, eq71_e1841_q_d_n11, eq71_e1841_q_d_n12, eq71_e1841_q_d_n13, eq71_e1841_q_d_n14, eq71_e1841_q_d_n15, eq71_e1841_q_d_n16];
        let eq71_reactive_branch_derivatives: [f64; 0] = [];
        stamper.stamp_current_reactive_dense(
            Some(nodes[12]),
            Some(nodes[7]),
            &nodes,
            &eq71_reactive_node_derivatives,
            &branches,
            &eq71_reactive_branch_derivatives,
            self.multiplicity,
        );
    }

    pub(super) fn stamp_reactive_equation_73_block_0(
        &mut self,
        ctx: &GeneratedEvalContext<'_>,
        stamper: &mut GeneratedReactiveStamper<'_>,
        s: &mut ReactiveScratch,
    ) {
        let _ = stamper;
        let p = self.params;
        let nodes = self.nodes;
        let branches = self.branches;
        let (eq73_e1868, eq73_e1868_d_n0, eq73_e1868_d_n1, eq73_e1868_d_n2, eq73_e1868_d_n3, eq73_e1868_d_n4, eq73_e1868_d_n5, eq73_e1868_d_n6, eq73_e1868_d_n7, eq73_e1868_d_n8, eq73_e1868_d_n9, eq73_e1868_d_n10, eq73_e1868_d_n11, eq73_e1868_d_n12, eq73_e1868_d_n13, eq73_e1868_d_n14, eq73_e1868_d_n15, eq73_e1868_d_n16, eq73_e1868_q, eq73_e1868_q_d_n0, eq73_e1868_q_d_n1, eq73_e1868_q_d_n2, eq73_e1868_q_d_n3, eq73_e1868_q_d_n4, eq73_e1868_q_d_n5, eq73_e1868_q_d_n6, eq73_e1868_q_d_n7, eq73_e1868_q_d_n8, eq73_e1868_q_d_n9, eq73_e1868_q_d_n10, eq73_e1868_q_d_n11, eq73_e1868_q_d_n12, eq73_e1868_q_d_n13, eq73_e1868_q_d_n14, eq73_e1868_q_d_n15, eq73_e1868_q_d_n16,) = {
    if ((s.v[1627] != 0.0) && (s.v[1628] != 0.0)) {
        let eq73_e1864: f64 = (p.p29 * s.v[334]);
        let eq73_e1864_d_n0: f64 = (p.p29 * s.dn[334][0]);
        let eq73_e1864_d_n1: f64 = (p.p29 * s.dn[334][1]);
        let eq73_e1864_d_n2: f64 = (p.p29 * s.dn[334][2]);
        let eq73_e1864_d_n3: f64 = (p.p29 * s.dn[334][3]);
        let eq73_e1864_d_n4: f64 = (p.p29 * s.dn[334][4]);
        let eq73_e1864_d_n5: f64 = (p.p29 * s.dn[334][5]);
        let eq73_e1864_d_n6: f64 = (p.p29 * s.dn[334][6]);
        let eq73_e1864_d_n7: f64 = (p.p29 * s.dn[334][7]);
        let eq73_e1864_d_n8: f64 = (p.p29 * s.dn[334][8]);
        let eq73_e1864_d_n9: f64 = (p.p29 * s.dn[334][9]);
        let eq73_e1864_d_n10: f64 = (p.p29 * s.dn[334][10]);
        let eq73_e1864_d_n11: f64 = (p.p29 * s.dn[334][11]);
        let eq73_e1864_d_n12: f64 = (p.p29 * s.dn[334][12]);
        let eq73_e1864_d_n13: f64 = (p.p29 * s.dn[334][13]);
        let eq73_e1864_d_n14: f64 = (p.p29 * s.dn[334][14]);
        let eq73_e1864_d_n15: f64 = (p.p29 * s.dn[334][15]);
        let eq73_e1864_d_n16: f64 = (p.p29 * s.dn[334][16]);
        let eq73_e1865_q: f64 = eq73_e1864;
        let eq73_e1866: f64 = (s.v[187] * eq73_e1864);
        let eq73_e1866_d_n0: f64 = ((s.dn[187][0] * eq73_e1864) + (s.v[187] * eq73_e1864_d_n0));
        let eq73_e1866_d_n1: f64 = ((s.dn[187][1] * eq73_e1864) + (s.v[187] * eq73_e1864_d_n1));
        let eq73_e1866_d_n2: f64 = ((s.dn[187][2] * eq73_e1864) + (s.v[187] * eq73_e1864_d_n2));
        let eq73_e1866_d_n3: f64 = ((s.dn[187][3] * eq73_e1864) + (s.v[187] * eq73_e1864_d_n3));
        let eq73_e1866_d_n4: f64 = ((s.dn[187][4] * eq73_e1864) + (s.v[187] * eq73_e1864_d_n4));
        let eq73_e1866_d_n5: f64 = ((s.dn[187][5] * eq73_e1864) + (s.v[187] * eq73_e1864_d_n5));
        let eq73_e1866_d_n6: f64 = ((s.dn[187][6] * eq73_e1864) + (s.v[187] * eq73_e1864_d_n6));
        let eq73_e1866_d_n7: f64 = ((s.dn[187][7] * eq73_e1864) + (s.v[187] * eq73_e1864_d_n7));
        let eq73_e1866_d_n8: f64 = ((s.dn[187][8] * eq73_e1864) + (s.v[187] * eq73_e1864_d_n8));
        let eq73_e1866_d_n9: f64 = ((s.dn[187][9] * eq73_e1864) + (s.v[187] * eq73_e1864_d_n9));
        let eq73_e1866_d_n10: f64 = ((s.dn[187][10] * eq73_e1864) + (s.v[187] * eq73_e1864_d_n10));
        let eq73_e1866_d_n11: f64 = ((s.dn[187][11] * eq73_e1864) + (s.v[187] * eq73_e1864_d_n11));
        let eq73_e1866_d_n12: f64 = ((s.dn[187][12] * eq73_e1864) + (s.v[187] * eq73_e1864_d_n12));
        let eq73_e1866_d_n13: f64 = ((s.dn[187][13] * eq73_e1864) + (s.v[187] * eq73_e1864_d_n13));
        let eq73_e1866_d_n14: f64 = ((s.dn[187][14] * eq73_e1864) + (s.v[187] * eq73_e1864_d_n14));
        let eq73_e1866_d_n15: f64 = ((s.dn[187][15] * eq73_e1864) + (s.v[187] * eq73_e1864_d_n15));
        let eq73_e1866_d_n16: f64 = ((s.dn[187][16] * eq73_e1864) + (s.v[187] * eq73_e1864_d_n16));
        let eq73_e1866_q: f64 = (s.v[187] * eq73_e1865_q);
        let eq73_e1866_q_d_n0: f64 = ((s.dn[187][0] * eq73_e1865_q) + (s.v[187] * eq73_e1864_d_n0));
        let eq73_e1866_q_d_n1: f64 = ((s.dn[187][1] * eq73_e1865_q) + (s.v[187] * eq73_e1864_d_n1));
        let eq73_e1866_q_d_n2: f64 = ((s.dn[187][2] * eq73_e1865_q) + (s.v[187] * eq73_e1864_d_n2));
        let eq73_e1866_q_d_n3: f64 = ((s.dn[187][3] * eq73_e1865_q) + (s.v[187] * eq73_e1864_d_n3));
        let eq73_e1866_q_d_n4: f64 = ((s.dn[187][4] * eq73_e1865_q) + (s.v[187] * eq73_e1864_d_n4));
        let eq73_e1866_q_d_n5: f64 = ((s.dn[187][5] * eq73_e1865_q) + (s.v[187] * eq73_e1864_d_n5));
        let eq73_e1866_q_d_n6: f64 = ((s.dn[187][6] * eq73_e1865_q) + (s.v[187] * eq73_e1864_d_n6));
        let eq73_e1866_q_d_n7: f64 = ((s.dn[187][7] * eq73_e1865_q) + (s.v[187] * eq73_e1864_d_n7));
        let eq73_e1866_q_d_n8: f64 = ((s.dn[187][8] * eq73_e1865_q) + (s.v[187] * eq73_e1864_d_n8));
        let eq73_e1866_q_d_n9: f64 = ((s.dn[187][9] * eq73_e1865_q) + (s.v[187] * eq73_e1864_d_n9));
        let eq73_e1866_q_d_n10: f64 = ((s.dn[187][10] * eq73_e1865_q) + (s.v[187] * eq73_e1864_d_n10));
        let eq73_e1866_q_d_n11: f64 = ((s.dn[187][11] * eq73_e1865_q) + (s.v[187] * eq73_e1864_d_n11));
        let eq73_e1866_q_d_n12: f64 = ((s.dn[187][12] * eq73_e1865_q) + (s.v[187] * eq73_e1864_d_n12));
        let eq73_e1866_q_d_n13: f64 = ((s.dn[187][13] * eq73_e1865_q) + (s.v[187] * eq73_e1864_d_n13));
        let eq73_e1866_q_d_n14: f64 = ((s.dn[187][14] * eq73_e1865_q) + (s.v[187] * eq73_e1864_d_n14));
        let eq73_e1866_q_d_n15: f64 = ((s.dn[187][15] * eq73_e1865_q) + (s.v[187] * eq73_e1864_d_n15));
        let eq73_e1866_q_d_n16: f64 = ((s.dn[187][16] * eq73_e1865_q) + (s.v[187] * eq73_e1864_d_n16));
        (eq73_e1866, eq73_e1866_d_n0, eq73_e1866_d_n1, eq73_e1866_d_n2, eq73_e1866_d_n3, eq73_e1866_d_n4, eq73_e1866_d_n5, eq73_e1866_d_n6, eq73_e1866_d_n7, eq73_e1866_d_n8, eq73_e1866_d_n9, eq73_e1866_d_n10, eq73_e1866_d_n11, eq73_e1866_d_n12, eq73_e1866_d_n13, eq73_e1866_d_n14, eq73_e1866_d_n15, eq73_e1866_d_n16, eq73_e1866_q, eq73_e1866_q_d_n0, eq73_e1866_q_d_n1, eq73_e1866_q_d_n2, eq73_e1866_q_d_n3, eq73_e1866_q_d_n4, eq73_e1866_q_d_n5, eq73_e1866_q_d_n6, eq73_e1866_q_d_n7, eq73_e1866_q_d_n8, eq73_e1866_q_d_n9, eq73_e1866_q_d_n10, eq73_e1866_q_d_n11, eq73_e1866_q_d_n12, eq73_e1866_q_d_n13, eq73_e1866_q_d_n14, eq73_e1866_q_d_n15, eq73_e1866_q_d_n16,)
    } else {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    }
};
        let eq73_reactive_node_derivatives: [f64; 17] = [eq73_e1868_q_d_n0, eq73_e1868_q_d_n1, eq73_e1868_q_d_n2, eq73_e1868_q_d_n3, eq73_e1868_q_d_n4, eq73_e1868_q_d_n5, eq73_e1868_q_d_n6, eq73_e1868_q_d_n7, eq73_e1868_q_d_n8, eq73_e1868_q_d_n9, eq73_e1868_q_d_n10, eq73_e1868_q_d_n11, eq73_e1868_q_d_n12, eq73_e1868_q_d_n13, eq73_e1868_q_d_n14, eq73_e1868_q_d_n15, eq73_e1868_q_d_n16];
        let eq73_reactive_branch_derivatives: [f64; 0] = [];
        stamper.stamp_current_reactive_dense(
            Some(nodes[13]),
            Some(nodes[5]),
            &nodes,
            &eq73_reactive_node_derivatives,
            &branches,
            &eq73_reactive_branch_derivatives,
            self.multiplicity,
        );
    }

    pub(super) fn stamp_reactive_equation_76_block_0(
        &mut self,
        ctx: &GeneratedEvalContext<'_>,
        stamper: &mut GeneratedReactiveStamper<'_>,
        s: &mut ReactiveScratch,
    ) {
        let _ = stamper;
        let p = self.params;
        let nodes = self.nodes;
        let branches = self.branches;
        let (eq76_e1908, eq76_e1908_d_n0, eq76_e1908_d_n1, eq76_e1908_d_n2, eq76_e1908_d_n3, eq76_e1908_d_n4, eq76_e1908_d_n5, eq76_e1908_d_n6, eq76_e1908_d_n7, eq76_e1908_d_n8, eq76_e1908_d_n9, eq76_e1908_d_n10, eq76_e1908_d_n11, eq76_e1908_d_n12, eq76_e1908_d_n13, eq76_e1908_d_n14, eq76_e1908_d_n15, eq76_e1908_d_n16, eq76_e1908_q, eq76_e1908_q_d_n0, eq76_e1908_q_d_n1, eq76_e1908_q_d_n2, eq76_e1908_q_d_n3, eq76_e1908_q_d_n4, eq76_e1908_q_d_n5, eq76_e1908_q_d_n6, eq76_e1908_q_d_n7, eq76_e1908_q_d_n8, eq76_e1908_q_d_n9, eq76_e1908_q_d_n10, eq76_e1908_q_d_n11, eq76_e1908_q_d_n12, eq76_e1908_q_d_n13, eq76_e1908_q_d_n14, eq76_e1908_q_d_n15, eq76_e1908_q_d_n16,) = {
    if (!(s.v[1627] != 0.0)) {
        let eq76_e1904: f64 = (p.p29 * s.v[330]);
        let eq76_e1904_d_n0: f64 = (p.p29 * s.dn[330][0]);
        let eq76_e1904_d_n1: f64 = (p.p29 * s.dn[330][1]);
        let eq76_e1904_d_n2: f64 = (p.p29 * s.dn[330][2]);
        let eq76_e1904_d_n3: f64 = (p.p29 * s.dn[330][3]);
        let eq76_e1904_d_n4: f64 = (p.p29 * s.dn[330][4]);
        let eq76_e1904_d_n5: f64 = (p.p29 * s.dn[330][5]);
        let eq76_e1904_d_n6: f64 = (p.p29 * s.dn[330][6]);
        let eq76_e1904_d_n7: f64 = (p.p29 * s.dn[330][7]);
        let eq76_e1904_d_n8: f64 = (p.p29 * s.dn[330][8]);
        let eq76_e1904_d_n9: f64 = (p.p29 * s.dn[330][9]);
        let eq76_e1904_d_n10: f64 = (p.p29 * s.dn[330][10]);
        let eq76_e1904_d_n11: f64 = (p.p29 * s.dn[330][11]);
        let eq76_e1904_d_n12: f64 = (p.p29 * s.dn[330][12]);
        let eq76_e1904_d_n13: f64 = (p.p29 * s.dn[330][13]);
        let eq76_e1904_d_n14: f64 = (p.p29 * s.dn[330][14]);
        let eq76_e1904_d_n15: f64 = (p.p29 * s.dn[330][15]);
        let eq76_e1904_d_n16: f64 = (p.p29 * s.dn[330][16]);
        let eq76_e1905_q: f64 = eq76_e1904;
        let eq76_e1906: f64 = (s.v[187] * eq76_e1904);
        let eq76_e1906_d_n0: f64 = ((s.dn[187][0] * eq76_e1904) + (s.v[187] * eq76_e1904_d_n0));
        let eq76_e1906_d_n1: f64 = ((s.dn[187][1] * eq76_e1904) + (s.v[187] * eq76_e1904_d_n1));
        let eq76_e1906_d_n2: f64 = ((s.dn[187][2] * eq76_e1904) + (s.v[187] * eq76_e1904_d_n2));
        let eq76_e1906_d_n3: f64 = ((s.dn[187][3] * eq76_e1904) + (s.v[187] * eq76_e1904_d_n3));
        let eq76_e1906_d_n4: f64 = ((s.dn[187][4] * eq76_e1904) + (s.v[187] * eq76_e1904_d_n4));
        let eq76_e1906_d_n5: f64 = ((s.dn[187][5] * eq76_e1904) + (s.v[187] * eq76_e1904_d_n5));
        let eq76_e1906_d_n6: f64 = ((s.dn[187][6] * eq76_e1904) + (s.v[187] * eq76_e1904_d_n6));
        let eq76_e1906_d_n7: f64 = ((s.dn[187][7] * eq76_e1904) + (s.v[187] * eq76_e1904_d_n7));
        let eq76_e1906_d_n8: f64 = ((s.dn[187][8] * eq76_e1904) + (s.v[187] * eq76_e1904_d_n8));
        let eq76_e1906_d_n9: f64 = ((s.dn[187][9] * eq76_e1904) + (s.v[187] * eq76_e1904_d_n9));
        let eq76_e1906_d_n10: f64 = ((s.dn[187][10] * eq76_e1904) + (s.v[187] * eq76_e1904_d_n10));
        let eq76_e1906_d_n11: f64 = ((s.dn[187][11] * eq76_e1904) + (s.v[187] * eq76_e1904_d_n11));
        let eq76_e1906_d_n12: f64 = ((s.dn[187][12] * eq76_e1904) + (s.v[187] * eq76_e1904_d_n12));
        let eq76_e1906_d_n13: f64 = ((s.dn[187][13] * eq76_e1904) + (s.v[187] * eq76_e1904_d_n13));
        let eq76_e1906_d_n14: f64 = ((s.dn[187][14] * eq76_e1904) + (s.v[187] * eq76_e1904_d_n14));
        let eq76_e1906_d_n15: f64 = ((s.dn[187][15] * eq76_e1904) + (s.v[187] * eq76_e1904_d_n15));
        let eq76_e1906_d_n16: f64 = ((s.dn[187][16] * eq76_e1904) + (s.v[187] * eq76_e1904_d_n16));
        let eq76_e1906_q: f64 = (s.v[187] * eq76_e1905_q);
        let eq76_e1906_q_d_n0: f64 = ((s.dn[187][0] * eq76_e1905_q) + (s.v[187] * eq76_e1904_d_n0));
        let eq76_e1906_q_d_n1: f64 = ((s.dn[187][1] * eq76_e1905_q) + (s.v[187] * eq76_e1904_d_n1));
        let eq76_e1906_q_d_n2: f64 = ((s.dn[187][2] * eq76_e1905_q) + (s.v[187] * eq76_e1904_d_n2));
        let eq76_e1906_q_d_n3: f64 = ((s.dn[187][3] * eq76_e1905_q) + (s.v[187] * eq76_e1904_d_n3));
        let eq76_e1906_q_d_n4: f64 = ((s.dn[187][4] * eq76_e1905_q) + (s.v[187] * eq76_e1904_d_n4));
        let eq76_e1906_q_d_n5: f64 = ((s.dn[187][5] * eq76_e1905_q) + (s.v[187] * eq76_e1904_d_n5));
        let eq76_e1906_q_d_n6: f64 = ((s.dn[187][6] * eq76_e1905_q) + (s.v[187] * eq76_e1904_d_n6));
        let eq76_e1906_q_d_n7: f64 = ((s.dn[187][7] * eq76_e1905_q) + (s.v[187] * eq76_e1904_d_n7));
        let eq76_e1906_q_d_n8: f64 = ((s.dn[187][8] * eq76_e1905_q) + (s.v[187] * eq76_e1904_d_n8));
        let eq76_e1906_q_d_n9: f64 = ((s.dn[187][9] * eq76_e1905_q) + (s.v[187] * eq76_e1904_d_n9));
        let eq76_e1906_q_d_n10: f64 = ((s.dn[187][10] * eq76_e1905_q) + (s.v[187] * eq76_e1904_d_n10));
        let eq76_e1906_q_d_n11: f64 = ((s.dn[187][11] * eq76_e1905_q) + (s.v[187] * eq76_e1904_d_n11));
        let eq76_e1906_q_d_n12: f64 = ((s.dn[187][12] * eq76_e1905_q) + (s.v[187] * eq76_e1904_d_n12));
        let eq76_e1906_q_d_n13: f64 = ((s.dn[187][13] * eq76_e1905_q) + (s.v[187] * eq76_e1904_d_n13));
        let eq76_e1906_q_d_n14: f64 = ((s.dn[187][14] * eq76_e1905_q) + (s.v[187] * eq76_e1904_d_n14));
        let eq76_e1906_q_d_n15: f64 = ((s.dn[187][15] * eq76_e1905_q) + (s.v[187] * eq76_e1904_d_n15));
        let eq76_e1906_q_d_n16: f64 = ((s.dn[187][16] * eq76_e1905_q) + (s.v[187] * eq76_e1904_d_n16));
        (eq76_e1906, eq76_e1906_d_n0, eq76_e1906_d_n1, eq76_e1906_d_n2, eq76_e1906_d_n3, eq76_e1906_d_n4, eq76_e1906_d_n5, eq76_e1906_d_n6, eq76_e1906_d_n7, eq76_e1906_d_n8, eq76_e1906_d_n9, eq76_e1906_d_n10, eq76_e1906_d_n11, eq76_e1906_d_n12, eq76_e1906_d_n13, eq76_e1906_d_n14, eq76_e1906_d_n15, eq76_e1906_d_n16, eq76_e1906_q, eq76_e1906_q_d_n0, eq76_e1906_q_d_n1, eq76_e1906_q_d_n2, eq76_e1906_q_d_n3, eq76_e1906_q_d_n4, eq76_e1906_q_d_n5, eq76_e1906_q_d_n6, eq76_e1906_q_d_n7, eq76_e1906_q_d_n8, eq76_e1906_q_d_n9, eq76_e1906_q_d_n10, eq76_e1906_q_d_n11, eq76_e1906_q_d_n12, eq76_e1906_q_d_n13, eq76_e1906_q_d_n14, eq76_e1906_q_d_n15, eq76_e1906_q_d_n16,)
    } else {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    }
};
        let eq76_reactive_node_derivatives: [f64; 17] = [eq76_e1908_q_d_n0, eq76_e1908_q_d_n1, eq76_e1908_q_d_n2, eq76_e1908_q_d_n3, eq76_e1908_q_d_n4, eq76_e1908_q_d_n5, eq76_e1908_q_d_n6, eq76_e1908_q_d_n7, eq76_e1908_q_d_n8, eq76_e1908_q_d_n9, eq76_e1908_q_d_n10, eq76_e1908_q_d_n11, eq76_e1908_q_d_n12, eq76_e1908_q_d_n13, eq76_e1908_q_d_n14, eq76_e1908_q_d_n15, eq76_e1908_q_d_n16];
        let eq76_reactive_branch_derivatives: [f64; 0] = [];
        stamper.stamp_current_reactive_dense(
            Some(nodes[11]),
            Some(nodes[7]),
            &nodes,
            &eq76_reactive_node_derivatives,
            &branches,
            &eq76_reactive_branch_derivatives,
            self.multiplicity,
        );
    }

    pub(super) fn stamp_reactive_equation_77_block_0(
        &mut self,
        ctx: &GeneratedEvalContext<'_>,
        stamper: &mut GeneratedReactiveStamper<'_>,
        s: &mut ReactiveScratch,
    ) {
        let _ = stamper;
        let p = self.params;
        let nodes = self.nodes;
        let branches = self.branches;
        let (eq77_e1918, eq77_e1918_d_n0, eq77_e1918_d_n1, eq77_e1918_d_n2, eq77_e1918_d_n3, eq77_e1918_d_n4, eq77_e1918_d_n5, eq77_e1918_d_n6, eq77_e1918_d_n7, eq77_e1918_d_n8, eq77_e1918_d_n9, eq77_e1918_d_n10, eq77_e1918_d_n11, eq77_e1918_d_n12, eq77_e1918_d_n13, eq77_e1918_d_n14, eq77_e1918_d_n15, eq77_e1918_d_n16, eq77_e1918_q, eq77_e1918_q_d_n0, eq77_e1918_q_d_n1, eq77_e1918_q_d_n2, eq77_e1918_q_d_n3, eq77_e1918_q_d_n4, eq77_e1918_q_d_n5, eq77_e1918_q_d_n6, eq77_e1918_q_d_n7, eq77_e1918_q_d_n8, eq77_e1918_q_d_n9, eq77_e1918_q_d_n10, eq77_e1918_q_d_n11, eq77_e1918_q_d_n12, eq77_e1918_q_d_n13, eq77_e1918_q_d_n14, eq77_e1918_q_d_n15, eq77_e1918_q_d_n16,) = {
    if (!(s.v[1627] != 0.0)) {
        let eq77_e1914: f64 = (p.p29 * s.v[334]);
        let eq77_e1914_d_n0: f64 = (p.p29 * s.dn[334][0]);
        let eq77_e1914_d_n1: f64 = (p.p29 * s.dn[334][1]);
        let eq77_e1914_d_n2: f64 = (p.p29 * s.dn[334][2]);
        let eq77_e1914_d_n3: f64 = (p.p29 * s.dn[334][3]);
        let eq77_e1914_d_n4: f64 = (p.p29 * s.dn[334][4]);
        let eq77_e1914_d_n5: f64 = (p.p29 * s.dn[334][5]);
        let eq77_e1914_d_n6: f64 = (p.p29 * s.dn[334][6]);
        let eq77_e1914_d_n7: f64 = (p.p29 * s.dn[334][7]);
        let eq77_e1914_d_n8: f64 = (p.p29 * s.dn[334][8]);
        let eq77_e1914_d_n9: f64 = (p.p29 * s.dn[334][9]);
        let eq77_e1914_d_n10: f64 = (p.p29 * s.dn[334][10]);
        let eq77_e1914_d_n11: f64 = (p.p29 * s.dn[334][11]);
        let eq77_e1914_d_n12: f64 = (p.p29 * s.dn[334][12]);
        let eq77_e1914_d_n13: f64 = (p.p29 * s.dn[334][13]);
        let eq77_e1914_d_n14: f64 = (p.p29 * s.dn[334][14]);
        let eq77_e1914_d_n15: f64 = (p.p29 * s.dn[334][15]);
        let eq77_e1914_d_n16: f64 = (p.p29 * s.dn[334][16]);
        let eq77_e1915_q: f64 = eq77_e1914;
        let eq77_e1916: f64 = (s.v[187] * eq77_e1914);
        let eq77_e1916_d_n0: f64 = ((s.dn[187][0] * eq77_e1914) + (s.v[187] * eq77_e1914_d_n0));
        let eq77_e1916_d_n1: f64 = ((s.dn[187][1] * eq77_e1914) + (s.v[187] * eq77_e1914_d_n1));
        let eq77_e1916_d_n2: f64 = ((s.dn[187][2] * eq77_e1914) + (s.v[187] * eq77_e1914_d_n2));
        let eq77_e1916_d_n3: f64 = ((s.dn[187][3] * eq77_e1914) + (s.v[187] * eq77_e1914_d_n3));
        let eq77_e1916_d_n4: f64 = ((s.dn[187][4] * eq77_e1914) + (s.v[187] * eq77_e1914_d_n4));
        let eq77_e1916_d_n5: f64 = ((s.dn[187][5] * eq77_e1914) + (s.v[187] * eq77_e1914_d_n5));
        let eq77_e1916_d_n6: f64 = ((s.dn[187][6] * eq77_e1914) + (s.v[187] * eq77_e1914_d_n6));
        let eq77_e1916_d_n7: f64 = ((s.dn[187][7] * eq77_e1914) + (s.v[187] * eq77_e1914_d_n7));
        let eq77_e1916_d_n8: f64 = ((s.dn[187][8] * eq77_e1914) + (s.v[187] * eq77_e1914_d_n8));
        let eq77_e1916_d_n9: f64 = ((s.dn[187][9] * eq77_e1914) + (s.v[187] * eq77_e1914_d_n9));
        let eq77_e1916_d_n10: f64 = ((s.dn[187][10] * eq77_e1914) + (s.v[187] * eq77_e1914_d_n10));
        let eq77_e1916_d_n11: f64 = ((s.dn[187][11] * eq77_e1914) + (s.v[187] * eq77_e1914_d_n11));
        let eq77_e1916_d_n12: f64 = ((s.dn[187][12] * eq77_e1914) + (s.v[187] * eq77_e1914_d_n12));
        let eq77_e1916_d_n13: f64 = ((s.dn[187][13] * eq77_e1914) + (s.v[187] * eq77_e1914_d_n13));
        let eq77_e1916_d_n14: f64 = ((s.dn[187][14] * eq77_e1914) + (s.v[187] * eq77_e1914_d_n14));
        let eq77_e1916_d_n15: f64 = ((s.dn[187][15] * eq77_e1914) + (s.v[187] * eq77_e1914_d_n15));
        let eq77_e1916_d_n16: f64 = ((s.dn[187][16] * eq77_e1914) + (s.v[187] * eq77_e1914_d_n16));
        let eq77_e1916_q: f64 = (s.v[187] * eq77_e1915_q);
        let eq77_e1916_q_d_n0: f64 = ((s.dn[187][0] * eq77_e1915_q) + (s.v[187] * eq77_e1914_d_n0));
        let eq77_e1916_q_d_n1: f64 = ((s.dn[187][1] * eq77_e1915_q) + (s.v[187] * eq77_e1914_d_n1));
        let eq77_e1916_q_d_n2: f64 = ((s.dn[187][2] * eq77_e1915_q) + (s.v[187] * eq77_e1914_d_n2));
        let eq77_e1916_q_d_n3: f64 = ((s.dn[187][3] * eq77_e1915_q) + (s.v[187] * eq77_e1914_d_n3));
        let eq77_e1916_q_d_n4: f64 = ((s.dn[187][4] * eq77_e1915_q) + (s.v[187] * eq77_e1914_d_n4));
        let eq77_e1916_q_d_n5: f64 = ((s.dn[187][5] * eq77_e1915_q) + (s.v[187] * eq77_e1914_d_n5));
        let eq77_e1916_q_d_n6: f64 = ((s.dn[187][6] * eq77_e1915_q) + (s.v[187] * eq77_e1914_d_n6));
        let eq77_e1916_q_d_n7: f64 = ((s.dn[187][7] * eq77_e1915_q) + (s.v[187] * eq77_e1914_d_n7));
        let eq77_e1916_q_d_n8: f64 = ((s.dn[187][8] * eq77_e1915_q) + (s.v[187] * eq77_e1914_d_n8));
        let eq77_e1916_q_d_n9: f64 = ((s.dn[187][9] * eq77_e1915_q) + (s.v[187] * eq77_e1914_d_n9));
        let eq77_e1916_q_d_n10: f64 = ((s.dn[187][10] * eq77_e1915_q) + (s.v[187] * eq77_e1914_d_n10));
        let eq77_e1916_q_d_n11: f64 = ((s.dn[187][11] * eq77_e1915_q) + (s.v[187] * eq77_e1914_d_n11));
        let eq77_e1916_q_d_n12: f64 = ((s.dn[187][12] * eq77_e1915_q) + (s.v[187] * eq77_e1914_d_n12));
        let eq77_e1916_q_d_n13: f64 = ((s.dn[187][13] * eq77_e1915_q) + (s.v[187] * eq77_e1914_d_n13));
        let eq77_e1916_q_d_n14: f64 = ((s.dn[187][14] * eq77_e1915_q) + (s.v[187] * eq77_e1914_d_n14));
        let eq77_e1916_q_d_n15: f64 = ((s.dn[187][15] * eq77_e1915_q) + (s.v[187] * eq77_e1914_d_n15));
        let eq77_e1916_q_d_n16: f64 = ((s.dn[187][16] * eq77_e1915_q) + (s.v[187] * eq77_e1914_d_n16));
        (eq77_e1916, eq77_e1916_d_n0, eq77_e1916_d_n1, eq77_e1916_d_n2, eq77_e1916_d_n3, eq77_e1916_d_n4, eq77_e1916_d_n5, eq77_e1916_d_n6, eq77_e1916_d_n7, eq77_e1916_d_n8, eq77_e1916_d_n9, eq77_e1916_d_n10, eq77_e1916_d_n11, eq77_e1916_d_n12, eq77_e1916_d_n13, eq77_e1916_d_n14, eq77_e1916_d_n15, eq77_e1916_d_n16, eq77_e1916_q, eq77_e1916_q_d_n0, eq77_e1916_q_d_n1, eq77_e1916_q_d_n2, eq77_e1916_q_d_n3, eq77_e1916_q_d_n4, eq77_e1916_q_d_n5, eq77_e1916_q_d_n6, eq77_e1916_q_d_n7, eq77_e1916_q_d_n8, eq77_e1916_q_d_n9, eq77_e1916_q_d_n10, eq77_e1916_q_d_n11, eq77_e1916_q_d_n12, eq77_e1916_q_d_n13, eq77_e1916_q_d_n14, eq77_e1916_q_d_n15, eq77_e1916_q_d_n16,)
    } else {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    }
};
        let eq77_reactive_node_derivatives: [f64; 17] = [eq77_e1918_q_d_n0, eq77_e1918_q_d_n1, eq77_e1918_q_d_n2, eq77_e1918_q_d_n3, eq77_e1918_q_d_n4, eq77_e1918_q_d_n5, eq77_e1918_q_d_n6, eq77_e1918_q_d_n7, eq77_e1918_q_d_n8, eq77_e1918_q_d_n9, eq77_e1918_q_d_n10, eq77_e1918_q_d_n11, eq77_e1918_q_d_n12, eq77_e1918_q_d_n13, eq77_e1918_q_d_n14, eq77_e1918_q_d_n15, eq77_e1918_q_d_n16];
        let eq77_reactive_branch_derivatives: [f64; 0] = [];
        stamper.stamp_current_reactive_dense(
            Some(nodes[11]),
            Some(nodes[5]),
            &nodes,
            &eq77_reactive_node_derivatives,
            &branches,
            &eq77_reactive_branch_derivatives,
            self.multiplicity,
        );
    }

    pub(super) fn stamp_reactive_equation_83_block_0(
        &mut self,
        ctx: &GeneratedEvalContext<'_>,
        stamper: &mut GeneratedReactiveStamper<'_>,
        s: &mut ReactiveScratch,
    ) {
        let _ = stamper;
        let p = self.params;
        let nodes = self.nodes;
        let branches = self.branches;
        let (eq83_e1984, eq83_e1984_d_n0, eq83_e1984_d_n1, eq83_e1984_d_n2, eq83_e1984_d_n3, eq83_e1984_d_n4, eq83_e1984_d_n5, eq83_e1984_d_n6, eq83_e1984_d_n7, eq83_e1984_d_n8, eq83_e1984_d_n9, eq83_e1984_d_n10, eq83_e1984_d_n11, eq83_e1984_d_n12, eq83_e1984_d_n13, eq83_e1984_d_n14, eq83_e1984_d_n15, eq83_e1984_d_n16, eq83_e1984_q, eq83_e1984_q_d_n0, eq83_e1984_q_d_n1, eq83_e1984_q_d_n2, eq83_e1984_q_d_n3, eq83_e1984_q_d_n4, eq83_e1984_q_d_n5, eq83_e1984_q_d_n6, eq83_e1984_q_d_n7, eq83_e1984_q_d_n8, eq83_e1984_q_d_n9, eq83_e1984_q_d_n10, eq83_e1984_q_d_n11, eq83_e1984_q_d_n12, eq83_e1984_q_d_n13, eq83_e1984_q_d_n14, eq83_e1984_q_d_n15, eq83_e1984_q_d_n16,) = {
    if (s.v[1630] != 0.0) {
        let eq83_e1980: f64 = (p.p29 * s.v[334]);
        let eq83_e1980_d_n0: f64 = (p.p29 * s.dn[334][0]);
        let eq83_e1980_d_n1: f64 = (p.p29 * s.dn[334][1]);
        let eq83_e1980_d_n2: f64 = (p.p29 * s.dn[334][2]);
        let eq83_e1980_d_n3: f64 = (p.p29 * s.dn[334][3]);
        let eq83_e1980_d_n4: f64 = (p.p29 * s.dn[334][4]);
        let eq83_e1980_d_n5: f64 = (p.p29 * s.dn[334][5]);
        let eq83_e1980_d_n6: f64 = (p.p29 * s.dn[334][6]);
        let eq83_e1980_d_n7: f64 = (p.p29 * s.dn[334][7]);
        let eq83_e1980_d_n8: f64 = (p.p29 * s.dn[334][8]);
        let eq83_e1980_d_n9: f64 = (p.p29 * s.dn[334][9]);
        let eq83_e1980_d_n10: f64 = (p.p29 * s.dn[334][10]);
        let eq83_e1980_d_n11: f64 = (p.p29 * s.dn[334][11]);
        let eq83_e1980_d_n12: f64 = (p.p29 * s.dn[334][12]);
        let eq83_e1980_d_n13: f64 = (p.p29 * s.dn[334][13]);
        let eq83_e1980_d_n14: f64 = (p.p29 * s.dn[334][14]);
        let eq83_e1980_d_n15: f64 = (p.p29 * s.dn[334][15]);
        let eq83_e1980_d_n16: f64 = (p.p29 * s.dn[334][16]);
        let eq83_e1981_q: f64 = eq83_e1980;
        let eq83_e1982: f64 = (s.v[187] * eq83_e1980);
        let eq83_e1982_d_n0: f64 = ((s.dn[187][0] * eq83_e1980) + (s.v[187] * eq83_e1980_d_n0));
        let eq83_e1982_d_n1: f64 = ((s.dn[187][1] * eq83_e1980) + (s.v[187] * eq83_e1980_d_n1));
        let eq83_e1982_d_n2: f64 = ((s.dn[187][2] * eq83_e1980) + (s.v[187] * eq83_e1980_d_n2));
        let eq83_e1982_d_n3: f64 = ((s.dn[187][3] * eq83_e1980) + (s.v[187] * eq83_e1980_d_n3));
        let eq83_e1982_d_n4: f64 = ((s.dn[187][4] * eq83_e1980) + (s.v[187] * eq83_e1980_d_n4));
        let eq83_e1982_d_n5: f64 = ((s.dn[187][5] * eq83_e1980) + (s.v[187] * eq83_e1980_d_n5));
        let eq83_e1982_d_n6: f64 = ((s.dn[187][6] * eq83_e1980) + (s.v[187] * eq83_e1980_d_n6));
        let eq83_e1982_d_n7: f64 = ((s.dn[187][7] * eq83_e1980) + (s.v[187] * eq83_e1980_d_n7));
        let eq83_e1982_d_n8: f64 = ((s.dn[187][8] * eq83_e1980) + (s.v[187] * eq83_e1980_d_n8));
        let eq83_e1982_d_n9: f64 = ((s.dn[187][9] * eq83_e1980) + (s.v[187] * eq83_e1980_d_n9));
        let eq83_e1982_d_n10: f64 = ((s.dn[187][10] * eq83_e1980) + (s.v[187] * eq83_e1980_d_n10));
        let eq83_e1982_d_n11: f64 = ((s.dn[187][11] * eq83_e1980) + (s.v[187] * eq83_e1980_d_n11));
        let eq83_e1982_d_n12: f64 = ((s.dn[187][12] * eq83_e1980) + (s.v[187] * eq83_e1980_d_n12));
        let eq83_e1982_d_n13: f64 = ((s.dn[187][13] * eq83_e1980) + (s.v[187] * eq83_e1980_d_n13));
        let eq83_e1982_d_n14: f64 = ((s.dn[187][14] * eq83_e1980) + (s.v[187] * eq83_e1980_d_n14));
        let eq83_e1982_d_n15: f64 = ((s.dn[187][15] * eq83_e1980) + (s.v[187] * eq83_e1980_d_n15));
        let eq83_e1982_d_n16: f64 = ((s.dn[187][16] * eq83_e1980) + (s.v[187] * eq83_e1980_d_n16));
        let eq83_e1982_q: f64 = (s.v[187] * eq83_e1981_q);
        let eq83_e1982_q_d_n0: f64 = ((s.dn[187][0] * eq83_e1981_q) + (s.v[187] * eq83_e1980_d_n0));
        let eq83_e1982_q_d_n1: f64 = ((s.dn[187][1] * eq83_e1981_q) + (s.v[187] * eq83_e1980_d_n1));
        let eq83_e1982_q_d_n2: f64 = ((s.dn[187][2] * eq83_e1981_q) + (s.v[187] * eq83_e1980_d_n2));
        let eq83_e1982_q_d_n3: f64 = ((s.dn[187][3] * eq83_e1981_q) + (s.v[187] * eq83_e1980_d_n3));
        let eq83_e1982_q_d_n4: f64 = ((s.dn[187][4] * eq83_e1981_q) + (s.v[187] * eq83_e1980_d_n4));
        let eq83_e1982_q_d_n5: f64 = ((s.dn[187][5] * eq83_e1981_q) + (s.v[187] * eq83_e1980_d_n5));
        let eq83_e1982_q_d_n6: f64 = ((s.dn[187][6] * eq83_e1981_q) + (s.v[187] * eq83_e1980_d_n6));
        let eq83_e1982_q_d_n7: f64 = ((s.dn[187][7] * eq83_e1981_q) + (s.v[187] * eq83_e1980_d_n7));
        let eq83_e1982_q_d_n8: f64 = ((s.dn[187][8] * eq83_e1981_q) + (s.v[187] * eq83_e1980_d_n8));
        let eq83_e1982_q_d_n9: f64 = ((s.dn[187][9] * eq83_e1981_q) + (s.v[187] * eq83_e1980_d_n9));
        let eq83_e1982_q_d_n10: f64 = ((s.dn[187][10] * eq83_e1981_q) + (s.v[187] * eq83_e1980_d_n10));
        let eq83_e1982_q_d_n11: f64 = ((s.dn[187][11] * eq83_e1981_q) + (s.v[187] * eq83_e1980_d_n11));
        let eq83_e1982_q_d_n12: f64 = ((s.dn[187][12] * eq83_e1981_q) + (s.v[187] * eq83_e1980_d_n12));
        let eq83_e1982_q_d_n13: f64 = ((s.dn[187][13] * eq83_e1981_q) + (s.v[187] * eq83_e1980_d_n13));
        let eq83_e1982_q_d_n14: f64 = ((s.dn[187][14] * eq83_e1981_q) + (s.v[187] * eq83_e1980_d_n14));
        let eq83_e1982_q_d_n15: f64 = ((s.dn[187][15] * eq83_e1981_q) + (s.v[187] * eq83_e1980_d_n15));
        let eq83_e1982_q_d_n16: f64 = ((s.dn[187][16] * eq83_e1981_q) + (s.v[187] * eq83_e1980_d_n16));
        (eq83_e1982, eq83_e1982_d_n0, eq83_e1982_d_n1, eq83_e1982_d_n2, eq83_e1982_d_n3, eq83_e1982_d_n4, eq83_e1982_d_n5, eq83_e1982_d_n6, eq83_e1982_d_n7, eq83_e1982_d_n8, eq83_e1982_d_n9, eq83_e1982_d_n10, eq83_e1982_d_n11, eq83_e1982_d_n12, eq83_e1982_d_n13, eq83_e1982_d_n14, eq83_e1982_d_n15, eq83_e1982_d_n16, eq83_e1982_q, eq83_e1982_q_d_n0, eq83_e1982_q_d_n1, eq83_e1982_q_d_n2, eq83_e1982_q_d_n3, eq83_e1982_q_d_n4, eq83_e1982_q_d_n5, eq83_e1982_q_d_n6, eq83_e1982_q_d_n7, eq83_e1982_q_d_n8, eq83_e1982_q_d_n9, eq83_e1982_q_d_n10, eq83_e1982_q_d_n11, eq83_e1982_q_d_n12, eq83_e1982_q_d_n13, eq83_e1982_q_d_n14, eq83_e1982_q_d_n15, eq83_e1982_q_d_n16,)
    } else {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    }
};
        let eq83_reactive_node_derivatives: [f64; 17] = [eq83_e1984_q_d_n0, eq83_e1984_q_d_n1, eq83_e1984_q_d_n2, eq83_e1984_q_d_n3, eq83_e1984_q_d_n4, eq83_e1984_q_d_n5, eq83_e1984_q_d_n6, eq83_e1984_q_d_n7, eq83_e1984_q_d_n8, eq83_e1984_q_d_n9, eq83_e1984_q_d_n10, eq83_e1984_q_d_n11, eq83_e1984_q_d_n12, eq83_e1984_q_d_n13, eq83_e1984_q_d_n14, eq83_e1984_q_d_n15, eq83_e1984_q_d_n16];
        let eq83_reactive_branch_derivatives: [f64; 0] = [];
        stamper.stamp_current_reactive_dense(
            Some(nodes[13]),
            Some(nodes[5]),
            &nodes,
            &eq83_reactive_node_derivatives,
            &branches,
            &eq83_reactive_branch_derivatives,
            self.multiplicity,
        );
    }

    pub(super) fn stamp_reactive_equation_84_block_0(
        &mut self,
        ctx: &GeneratedEvalContext<'_>,
        stamper: &mut GeneratedReactiveStamper<'_>,
        s: &mut ReactiveScratch,
    ) {
        let _ = stamper;
        let p = self.params;
        let nodes = self.nodes;
        let branches = self.branches;
        let (eq84_e1993, eq84_e1993_d_n0, eq84_e1993_d_n1, eq84_e1993_d_n2, eq84_e1993_d_n3, eq84_e1993_d_n4, eq84_e1993_d_n5, eq84_e1993_d_n6, eq84_e1993_d_n7, eq84_e1993_d_n8, eq84_e1993_d_n9, eq84_e1993_d_n10, eq84_e1993_d_n11, eq84_e1993_d_n12, eq84_e1993_d_n13, eq84_e1993_d_n14, eq84_e1993_d_n15, eq84_e1993_d_n16, eq84_e1993_q, eq84_e1993_q_d_n0, eq84_e1993_q_d_n1, eq84_e1993_q_d_n2, eq84_e1993_q_d_n3, eq84_e1993_q_d_n4, eq84_e1993_q_d_n5, eq84_e1993_q_d_n6, eq84_e1993_q_d_n7, eq84_e1993_q_d_n8, eq84_e1993_q_d_n9, eq84_e1993_q_d_n10, eq84_e1993_q_d_n11, eq84_e1993_q_d_n12, eq84_e1993_q_d_n13, eq84_e1993_q_d_n14, eq84_e1993_q_d_n15, eq84_e1993_q_d_n16,) = {
    if (s.v[1630] != 0.0) {
        let eq84_e1989: f64 = (p.p29 * s.v[338]);
        let eq84_e1989_d_n0: f64 = (p.p29 * s.dn[338][0]);
        let eq84_e1989_d_n1: f64 = (p.p29 * s.dn[338][1]);
        let eq84_e1989_d_n2: f64 = (p.p29 * s.dn[338][2]);
        let eq84_e1989_d_n3: f64 = (p.p29 * s.dn[338][3]);
        let eq84_e1989_d_n4: f64 = (p.p29 * s.dn[338][4]);
        let eq84_e1989_d_n5: f64 = (p.p29 * s.dn[338][5]);
        let eq84_e1989_d_n6: f64 = (p.p29 * s.dn[338][6]);
        let eq84_e1989_d_n7: f64 = (p.p29 * s.dn[338][7]);
        let eq84_e1989_d_n8: f64 = (p.p29 * s.dn[338][8]);
        let eq84_e1989_d_n9: f64 = (p.p29 * s.dn[338][9]);
        let eq84_e1989_d_n10: f64 = (p.p29 * s.dn[338][10]);
        let eq84_e1989_d_n11: f64 = (p.p29 * s.dn[338][11]);
        let eq84_e1989_d_n12: f64 = (p.p29 * s.dn[338][12]);
        let eq84_e1989_d_n13: f64 = (p.p29 * s.dn[338][13]);
        let eq84_e1989_d_n14: f64 = (p.p29 * s.dn[338][14]);
        let eq84_e1989_d_n15: f64 = (p.p29 * s.dn[338][15]);
        let eq84_e1989_d_n16: f64 = (p.p29 * s.dn[338][16]);
        let eq84_e1990_q: f64 = eq84_e1989;
        let eq84_e1991: f64 = (s.v[187] * eq84_e1989);
        let eq84_e1991_d_n0: f64 = ((s.dn[187][0] * eq84_e1989) + (s.v[187] * eq84_e1989_d_n0));
        let eq84_e1991_d_n1: f64 = ((s.dn[187][1] * eq84_e1989) + (s.v[187] * eq84_e1989_d_n1));
        let eq84_e1991_d_n2: f64 = ((s.dn[187][2] * eq84_e1989) + (s.v[187] * eq84_e1989_d_n2));
        let eq84_e1991_d_n3: f64 = ((s.dn[187][3] * eq84_e1989) + (s.v[187] * eq84_e1989_d_n3));
        let eq84_e1991_d_n4: f64 = ((s.dn[187][4] * eq84_e1989) + (s.v[187] * eq84_e1989_d_n4));
        let eq84_e1991_d_n5: f64 = ((s.dn[187][5] * eq84_e1989) + (s.v[187] * eq84_e1989_d_n5));
        let eq84_e1991_d_n6: f64 = ((s.dn[187][6] * eq84_e1989) + (s.v[187] * eq84_e1989_d_n6));
        let eq84_e1991_d_n7: f64 = ((s.dn[187][7] * eq84_e1989) + (s.v[187] * eq84_e1989_d_n7));
        let eq84_e1991_d_n8: f64 = ((s.dn[187][8] * eq84_e1989) + (s.v[187] * eq84_e1989_d_n8));
        let eq84_e1991_d_n9: f64 = ((s.dn[187][9] * eq84_e1989) + (s.v[187] * eq84_e1989_d_n9));
        let eq84_e1991_d_n10: f64 = ((s.dn[187][10] * eq84_e1989) + (s.v[187] * eq84_e1989_d_n10));
        let eq84_e1991_d_n11: f64 = ((s.dn[187][11] * eq84_e1989) + (s.v[187] * eq84_e1989_d_n11));
        let eq84_e1991_d_n12: f64 = ((s.dn[187][12] * eq84_e1989) + (s.v[187] * eq84_e1989_d_n12));
        let eq84_e1991_d_n13: f64 = ((s.dn[187][13] * eq84_e1989) + (s.v[187] * eq84_e1989_d_n13));
        let eq84_e1991_d_n14: f64 = ((s.dn[187][14] * eq84_e1989) + (s.v[187] * eq84_e1989_d_n14));
        let eq84_e1991_d_n15: f64 = ((s.dn[187][15] * eq84_e1989) + (s.v[187] * eq84_e1989_d_n15));
        let eq84_e1991_d_n16: f64 = ((s.dn[187][16] * eq84_e1989) + (s.v[187] * eq84_e1989_d_n16));
        let eq84_e1991_q: f64 = (s.v[187] * eq84_e1990_q);
        let eq84_e1991_q_d_n0: f64 = ((s.dn[187][0] * eq84_e1990_q) + (s.v[187] * eq84_e1989_d_n0));
        let eq84_e1991_q_d_n1: f64 = ((s.dn[187][1] * eq84_e1990_q) + (s.v[187] * eq84_e1989_d_n1));
        let eq84_e1991_q_d_n2: f64 = ((s.dn[187][2] * eq84_e1990_q) + (s.v[187] * eq84_e1989_d_n2));
        let eq84_e1991_q_d_n3: f64 = ((s.dn[187][3] * eq84_e1990_q) + (s.v[187] * eq84_e1989_d_n3));
        let eq84_e1991_q_d_n4: f64 = ((s.dn[187][4] * eq84_e1990_q) + (s.v[187] * eq84_e1989_d_n4));
        let eq84_e1991_q_d_n5: f64 = ((s.dn[187][5] * eq84_e1990_q) + (s.v[187] * eq84_e1989_d_n5));
        let eq84_e1991_q_d_n6: f64 = ((s.dn[187][6] * eq84_e1990_q) + (s.v[187] * eq84_e1989_d_n6));
        let eq84_e1991_q_d_n7: f64 = ((s.dn[187][7] * eq84_e1990_q) + (s.v[187] * eq84_e1989_d_n7));
        let eq84_e1991_q_d_n8: f64 = ((s.dn[187][8] * eq84_e1990_q) + (s.v[187] * eq84_e1989_d_n8));
        let eq84_e1991_q_d_n9: f64 = ((s.dn[187][9] * eq84_e1990_q) + (s.v[187] * eq84_e1989_d_n9));
        let eq84_e1991_q_d_n10: f64 = ((s.dn[187][10] * eq84_e1990_q) + (s.v[187] * eq84_e1989_d_n10));
        let eq84_e1991_q_d_n11: f64 = ((s.dn[187][11] * eq84_e1990_q) + (s.v[187] * eq84_e1989_d_n11));
        let eq84_e1991_q_d_n12: f64 = ((s.dn[187][12] * eq84_e1990_q) + (s.v[187] * eq84_e1989_d_n12));
        let eq84_e1991_q_d_n13: f64 = ((s.dn[187][13] * eq84_e1990_q) + (s.v[187] * eq84_e1989_d_n13));
        let eq84_e1991_q_d_n14: f64 = ((s.dn[187][14] * eq84_e1990_q) + (s.v[187] * eq84_e1989_d_n14));
        let eq84_e1991_q_d_n15: f64 = ((s.dn[187][15] * eq84_e1990_q) + (s.v[187] * eq84_e1989_d_n15));
        let eq84_e1991_q_d_n16: f64 = ((s.dn[187][16] * eq84_e1990_q) + (s.v[187] * eq84_e1989_d_n16));
        (eq84_e1991, eq84_e1991_d_n0, eq84_e1991_d_n1, eq84_e1991_d_n2, eq84_e1991_d_n3, eq84_e1991_d_n4, eq84_e1991_d_n5, eq84_e1991_d_n6, eq84_e1991_d_n7, eq84_e1991_d_n8, eq84_e1991_d_n9, eq84_e1991_d_n10, eq84_e1991_d_n11, eq84_e1991_d_n12, eq84_e1991_d_n13, eq84_e1991_d_n14, eq84_e1991_d_n15, eq84_e1991_d_n16, eq84_e1991_q, eq84_e1991_q_d_n0, eq84_e1991_q_d_n1, eq84_e1991_q_d_n2, eq84_e1991_q_d_n3, eq84_e1991_q_d_n4, eq84_e1991_q_d_n5, eq84_e1991_q_d_n6, eq84_e1991_q_d_n7, eq84_e1991_q_d_n8, eq84_e1991_q_d_n9, eq84_e1991_q_d_n10, eq84_e1991_q_d_n11, eq84_e1991_q_d_n12, eq84_e1991_q_d_n13, eq84_e1991_q_d_n14, eq84_e1991_q_d_n15, eq84_e1991_q_d_n16,)
    } else {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    }
};
        let eq84_reactive_node_derivatives: [f64; 17] = [eq84_e1993_q_d_n0, eq84_e1993_q_d_n1, eq84_e1993_q_d_n2, eq84_e1993_q_d_n3, eq84_e1993_q_d_n4, eq84_e1993_q_d_n5, eq84_e1993_q_d_n6, eq84_e1993_q_d_n7, eq84_e1993_q_d_n8, eq84_e1993_q_d_n9, eq84_e1993_q_d_n10, eq84_e1993_q_d_n11, eq84_e1993_q_d_n12, eq84_e1993_q_d_n13, eq84_e1993_q_d_n14, eq84_e1993_q_d_n15, eq84_e1993_q_d_n16];
        let eq84_reactive_branch_derivatives: [f64; 0] = [];
        stamper.stamp_current_reactive_dense(
            Some(nodes[13]),
            Some(nodes[14]),
            &nodes,
            &eq84_reactive_node_derivatives,
            &branches,
            &eq84_reactive_branch_derivatives,
            self.multiplicity,
        );
    }
}
