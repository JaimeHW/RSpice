#![allow(dead_code, unused_assignments, unused_imports, unused_parens, unused_variables)]

use super::{A, GeneratedDerivative, GeneratedEvalContext, GeneratedReactiveStamper, GeneratedStamper, Scratch, THERMAL_VOLTAGE_PER_K};
use super::super::state::Instance;

impl Instance {

    pub(super) fn stamp_transient_equation_22_block_0(
        &mut self,
        ctx: &GeneratedEvalContext<'_>,
        stamper: &mut GeneratedStamper<'_>,
        s: &mut Scratch,
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
        let eq22_e1438: f64 = self.eval_ddt(6, eq22_e1437);
        let eq22_e1438_d_n0: f64 = self.ddt_jacobian(eq22_e1437_d_n0);
        let eq22_e1438_d_n1: f64 = self.ddt_jacobian(eq22_e1437_d_n1);
        let eq22_e1438_d_n2: f64 = self.ddt_jacobian(eq22_e1437_d_n2);
        let eq22_e1438_d_n3: f64 = self.ddt_jacobian(eq22_e1437_d_n3);
        let eq22_e1438_d_n4: f64 = self.ddt_jacobian(eq22_e1437_d_n4);
        let eq22_e1438_d_n5: f64 = self.ddt_jacobian(eq22_e1437_d_n5);
        let eq22_e1438_d_n6: f64 = self.ddt_jacobian(eq22_e1437_d_n6);
        let eq22_e1438_d_n7: f64 = self.ddt_jacobian(eq22_e1437_d_n7);
        let eq22_e1438_d_n8: f64 = self.ddt_jacobian(eq22_e1437_d_n8);
        let eq22_e1438_d_n9: f64 = self.ddt_jacobian(eq22_e1437_d_n9);
        let eq22_e1438_d_n10: f64 = self.ddt_jacobian(eq22_e1437_d_n10);
        let eq22_e1438_d_n11: f64 = self.ddt_jacobian(eq22_e1437_d_n11);
        let eq22_e1438_d_n12: f64 = self.ddt_jacobian(eq22_e1437_d_n12);
        let eq22_e1438_d_n13: f64 = self.ddt_jacobian(eq22_e1437_d_n13);
        let eq22_e1438_d_n14: f64 = self.ddt_jacobian(eq22_e1437_d_n14);
        let eq22_e1438_d_n15: f64 = self.ddt_jacobian(eq22_e1437_d_n15);
        let eq22_e1438_d_n16: f64 = self.ddt_jacobian(eq22_e1437_d_n16);
        let eq22_e1439: f64 = (p.p29 * eq22_e1438);
        let eq22_e1439_d_n0: f64 = (p.p29 * eq22_e1438_d_n0);
        let eq22_e1439_d_n1: f64 = (p.p29 * eq22_e1438_d_n1);
        let eq22_e1439_d_n2: f64 = (p.p29 * eq22_e1438_d_n2);
        let eq22_e1439_d_n3: f64 = (p.p29 * eq22_e1438_d_n3);
        let eq22_e1439_d_n4: f64 = (p.p29 * eq22_e1438_d_n4);
        let eq22_e1439_d_n5: f64 = (p.p29 * eq22_e1438_d_n5);
        let eq22_e1439_d_n6: f64 = (p.p29 * eq22_e1438_d_n6);
        let eq22_e1439_d_n7: f64 = (p.p29 * eq22_e1438_d_n7);
        let eq22_e1439_d_n8: f64 = (p.p29 * eq22_e1438_d_n8);
        let eq22_e1439_d_n9: f64 = (p.p29 * eq22_e1438_d_n9);
        let eq22_e1439_d_n10: f64 = (p.p29 * eq22_e1438_d_n10);
        let eq22_e1439_d_n11: f64 = (p.p29 * eq22_e1438_d_n11);
        let eq22_e1439_d_n12: f64 = (p.p29 * eq22_e1438_d_n12);
        let eq22_e1439_d_n13: f64 = (p.p29 * eq22_e1438_d_n13);
        let eq22_e1439_d_n14: f64 = (p.p29 * eq22_e1438_d_n14);
        let eq22_e1439_d_n15: f64 = (p.p29 * eq22_e1438_d_n15);
        let eq22_e1439_d_n16: f64 = (p.p29 * eq22_e1438_d_n16);
        let eq22_value: f64 = eq22_e1439;
        let eq22_node_derivatives: [f64; 17] = [eq22_e1439_d_n0, eq22_e1439_d_n1, eq22_e1439_d_n2, eq22_e1439_d_n3, eq22_e1439_d_n4, eq22_e1439_d_n5, eq22_e1439_d_n6, eq22_e1439_d_n7, eq22_e1439_d_n8, eq22_e1439_d_n9, eq22_e1439_d_n10, eq22_e1439_d_n11, eq22_e1439_d_n12, eq22_e1439_d_n13, eq22_e1439_d_n14, eq22_e1439_d_n15, eq22_e1439_d_n16];
        let eq22_branch_derivatives: [f64; 0] = [];
        stamper.stamp_current_dense(
            Some(nodes[10]),
            Some(nodes[7]),
            self.multiplicity * (eq22_value),
            &nodes,
            &eq22_node_derivatives,
            &branches,
            &eq22_branch_derivatives,
            self.multiplicity,
        );
    }

    pub(super) fn stamp_transient_equation_23_block_0(
        &mut self,
        ctx: &GeneratedEvalContext<'_>,
        stamper: &mut GeneratedStamper<'_>,
        s: &mut Scratch,
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
        let eq23_e1445: f64 = self.eval_ddt(7, eq23_e1444);
        let eq23_e1445_d_n0: f64 = self.ddt_jacobian(eq23_e1444_d_n0);
        let eq23_e1445_d_n1: f64 = self.ddt_jacobian(eq23_e1444_d_n1);
        let eq23_e1445_d_n2: f64 = self.ddt_jacobian(eq23_e1444_d_n2);
        let eq23_e1445_d_n3: f64 = self.ddt_jacobian(eq23_e1444_d_n3);
        let eq23_e1445_d_n4: f64 = self.ddt_jacobian(eq23_e1444_d_n4);
        let eq23_e1445_d_n5: f64 = self.ddt_jacobian(eq23_e1444_d_n5);
        let eq23_e1445_d_n6: f64 = self.ddt_jacobian(eq23_e1444_d_n6);
        let eq23_e1445_d_n7: f64 = self.ddt_jacobian(eq23_e1444_d_n7);
        let eq23_e1445_d_n8: f64 = self.ddt_jacobian(eq23_e1444_d_n8);
        let eq23_e1445_d_n9: f64 = self.ddt_jacobian(eq23_e1444_d_n9);
        let eq23_e1445_d_n10: f64 = self.ddt_jacobian(eq23_e1444_d_n10);
        let eq23_e1445_d_n11: f64 = self.ddt_jacobian(eq23_e1444_d_n11);
        let eq23_e1445_d_n12: f64 = self.ddt_jacobian(eq23_e1444_d_n12);
        let eq23_e1445_d_n13: f64 = self.ddt_jacobian(eq23_e1444_d_n13);
        let eq23_e1445_d_n14: f64 = self.ddt_jacobian(eq23_e1444_d_n14);
        let eq23_e1445_d_n15: f64 = self.ddt_jacobian(eq23_e1444_d_n15);
        let eq23_e1445_d_n16: f64 = self.ddt_jacobian(eq23_e1444_d_n16);
        let eq23_e1446: f64 = (p.p29 * eq23_e1445);
        let eq23_e1446_d_n0: f64 = (p.p29 * eq23_e1445_d_n0);
        let eq23_e1446_d_n1: f64 = (p.p29 * eq23_e1445_d_n1);
        let eq23_e1446_d_n2: f64 = (p.p29 * eq23_e1445_d_n2);
        let eq23_e1446_d_n3: f64 = (p.p29 * eq23_e1445_d_n3);
        let eq23_e1446_d_n4: f64 = (p.p29 * eq23_e1445_d_n4);
        let eq23_e1446_d_n5: f64 = (p.p29 * eq23_e1445_d_n5);
        let eq23_e1446_d_n6: f64 = (p.p29 * eq23_e1445_d_n6);
        let eq23_e1446_d_n7: f64 = (p.p29 * eq23_e1445_d_n7);
        let eq23_e1446_d_n8: f64 = (p.p29 * eq23_e1445_d_n8);
        let eq23_e1446_d_n9: f64 = (p.p29 * eq23_e1445_d_n9);
        let eq23_e1446_d_n10: f64 = (p.p29 * eq23_e1445_d_n10);
        let eq23_e1446_d_n11: f64 = (p.p29 * eq23_e1445_d_n11);
        let eq23_e1446_d_n12: f64 = (p.p29 * eq23_e1445_d_n12);
        let eq23_e1446_d_n13: f64 = (p.p29 * eq23_e1445_d_n13);
        let eq23_e1446_d_n14: f64 = (p.p29 * eq23_e1445_d_n14);
        let eq23_e1446_d_n15: f64 = (p.p29 * eq23_e1445_d_n15);
        let eq23_e1446_d_n16: f64 = (p.p29 * eq23_e1445_d_n16);
        let eq23_value: f64 = eq23_e1446;
        let eq23_node_derivatives: [f64; 17] = [eq23_e1446_d_n0, eq23_e1446_d_n1, eq23_e1446_d_n2, eq23_e1446_d_n3, eq23_e1446_d_n4, eq23_e1446_d_n5, eq23_e1446_d_n6, eq23_e1446_d_n7, eq23_e1446_d_n8, eq23_e1446_d_n9, eq23_e1446_d_n10, eq23_e1446_d_n11, eq23_e1446_d_n12, eq23_e1446_d_n13, eq23_e1446_d_n14, eq23_e1446_d_n15, eq23_e1446_d_n16];
        let eq23_branch_derivatives: [f64; 0] = [];
        stamper.stamp_current_dense(
            Some(nodes[10]),
            Some(nodes[5]),
            self.multiplicity * (eq23_value),
            &nodes,
            &eq23_node_derivatives,
            &branches,
            &eq23_branch_derivatives,
            self.multiplicity,
        );
    }

    pub(super) fn stamp_transient_equation_24_block_0(
        &mut self,
        ctx: &GeneratedEvalContext<'_>,
        stamper: &mut GeneratedStamper<'_>,
        s: &mut Scratch,
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
        let eq24_e1452: f64 = self.eval_ddt(8, eq24_e1451);
        let eq24_e1452_d_n0: f64 = self.ddt_jacobian(eq24_e1451_d_n0);
        let eq24_e1452_d_n1: f64 = self.ddt_jacobian(eq24_e1451_d_n1);
        let eq24_e1452_d_n2: f64 = self.ddt_jacobian(eq24_e1451_d_n2);
        let eq24_e1452_d_n3: f64 = self.ddt_jacobian(eq24_e1451_d_n3);
        let eq24_e1452_d_n4: f64 = self.ddt_jacobian(eq24_e1451_d_n4);
        let eq24_e1452_d_n5: f64 = self.ddt_jacobian(eq24_e1451_d_n5);
        let eq24_e1452_d_n6: f64 = self.ddt_jacobian(eq24_e1451_d_n6);
        let eq24_e1452_d_n7: f64 = self.ddt_jacobian(eq24_e1451_d_n7);
        let eq24_e1452_d_n8: f64 = self.ddt_jacobian(eq24_e1451_d_n8);
        let eq24_e1452_d_n9: f64 = self.ddt_jacobian(eq24_e1451_d_n9);
        let eq24_e1452_d_n10: f64 = self.ddt_jacobian(eq24_e1451_d_n10);
        let eq24_e1452_d_n11: f64 = self.ddt_jacobian(eq24_e1451_d_n11);
        let eq24_e1452_d_n12: f64 = self.ddt_jacobian(eq24_e1451_d_n12);
        let eq24_e1452_d_n13: f64 = self.ddt_jacobian(eq24_e1451_d_n13);
        let eq24_e1452_d_n14: f64 = self.ddt_jacobian(eq24_e1451_d_n14);
        let eq24_e1452_d_n15: f64 = self.ddt_jacobian(eq24_e1451_d_n15);
        let eq24_e1452_d_n16: f64 = self.ddt_jacobian(eq24_e1451_d_n16);
        let eq24_e1453: f64 = (p.p29 * eq24_e1452);
        let eq24_e1453_d_n0: f64 = (p.p29 * eq24_e1452_d_n0);
        let eq24_e1453_d_n1: f64 = (p.p29 * eq24_e1452_d_n1);
        let eq24_e1453_d_n2: f64 = (p.p29 * eq24_e1452_d_n2);
        let eq24_e1453_d_n3: f64 = (p.p29 * eq24_e1452_d_n3);
        let eq24_e1453_d_n4: f64 = (p.p29 * eq24_e1452_d_n4);
        let eq24_e1453_d_n5: f64 = (p.p29 * eq24_e1452_d_n5);
        let eq24_e1453_d_n6: f64 = (p.p29 * eq24_e1452_d_n6);
        let eq24_e1453_d_n7: f64 = (p.p29 * eq24_e1452_d_n7);
        let eq24_e1453_d_n8: f64 = (p.p29 * eq24_e1452_d_n8);
        let eq24_e1453_d_n9: f64 = (p.p29 * eq24_e1452_d_n9);
        let eq24_e1453_d_n10: f64 = (p.p29 * eq24_e1452_d_n10);
        let eq24_e1453_d_n11: f64 = (p.p29 * eq24_e1452_d_n11);
        let eq24_e1453_d_n12: f64 = (p.p29 * eq24_e1452_d_n12);
        let eq24_e1453_d_n13: f64 = (p.p29 * eq24_e1452_d_n13);
        let eq24_e1453_d_n14: f64 = (p.p29 * eq24_e1452_d_n14);
        let eq24_e1453_d_n15: f64 = (p.p29 * eq24_e1452_d_n15);
        let eq24_e1453_d_n16: f64 = (p.p29 * eq24_e1452_d_n16);
        let eq24_value: f64 = eq24_e1453;
        let eq24_node_derivatives: [f64; 17] = [eq24_e1453_d_n0, eq24_e1453_d_n1, eq24_e1453_d_n2, eq24_e1453_d_n3, eq24_e1453_d_n4, eq24_e1453_d_n5, eq24_e1453_d_n6, eq24_e1453_d_n7, eq24_e1453_d_n8, eq24_e1453_d_n9, eq24_e1453_d_n10, eq24_e1453_d_n11, eq24_e1453_d_n12, eq24_e1453_d_n13, eq24_e1453_d_n14, eq24_e1453_d_n15, eq24_e1453_d_n16];
        let eq24_branch_derivatives: [f64; 0] = [];
        stamper.stamp_current_dense(
            Some(nodes[10]),
            Some(nodes[11]),
            self.multiplicity * (eq24_value),
            &nodes,
            &eq24_node_derivatives,
            &branches,
            &eq24_branch_derivatives,
            self.multiplicity,
        );
    }

    pub(super) fn stamp_transient_equation_25_block_0(
        &mut self,
        ctx: &GeneratedEvalContext<'_>,
        stamper: &mut GeneratedStamper<'_>,
        s: &mut Scratch,
    ) {
        let _ = stamper;
        let p = self.params;
        let nodes = self.nodes;
        let branches = self.branches;
        let eq25_e1456: f64 = (s.v[187] * p.p28);
        let eq25_e1456_d_n0: f64 = (s.dn[187][0] * p.p28);
        let eq25_e1456_d_n1: f64 = (s.dn[187][1] * p.p28);
        let eq25_e1456_d_n2: f64 = (s.dn[187][2] * p.p28);
        let eq25_e1456_d_n3: f64 = (s.dn[187][3] * p.p28);
        let eq25_e1456_d_n4: f64 = (s.dn[187][4] * p.p28);
        let eq25_e1456_d_n5: f64 = (s.dn[187][5] * p.p28);
        let eq25_e1456_d_n6: f64 = (s.dn[187][6] * p.p28);
        let eq25_e1456_d_n7: f64 = (s.dn[187][7] * p.p28);
        let eq25_e1456_d_n8: f64 = (s.dn[187][8] * p.p28);
        let eq25_e1456_d_n9: f64 = (s.dn[187][9] * p.p28);
        let eq25_e1456_d_n10: f64 = (s.dn[187][10] * p.p28);
        let eq25_e1456_d_n11: f64 = (s.dn[187][11] * p.p28);
        let eq25_e1456_d_n12: f64 = (s.dn[187][12] * p.p28);
        let eq25_e1456_d_n13: f64 = (s.dn[187][13] * p.p28);
        let eq25_e1456_d_n14: f64 = (s.dn[187][14] * p.p28);
        let eq25_e1456_d_n15: f64 = (s.dn[187][15] * p.p28);
        let eq25_e1456_d_n16: f64 = (s.dn[187][16] * p.p28);
        let eq25_e1458: f64 = (eq25_e1456 * s.v[57]);
        let eq25_e1458_d_n0: f64 = ((eq25_e1456_d_n0 * s.v[57]) + (eq25_e1456 * s.dn[57][0]));
        let eq25_e1458_d_n1: f64 = ((eq25_e1456_d_n1 * s.v[57]) + (eq25_e1456 * s.dn[57][1]));
        let eq25_e1458_d_n2: f64 = ((eq25_e1456_d_n2 * s.v[57]) + (eq25_e1456 * s.dn[57][2]));
        let eq25_e1458_d_n3: f64 = ((eq25_e1456_d_n3 * s.v[57]) + (eq25_e1456 * s.dn[57][3]));
        let eq25_e1458_d_n4: f64 = ((eq25_e1456_d_n4 * s.v[57]) + (eq25_e1456 * s.dn[57][4]));
        let eq25_e1458_d_n5: f64 = ((eq25_e1456_d_n5 * s.v[57]) + (eq25_e1456 * s.dn[57][5]));
        let eq25_e1458_d_n6: f64 = ((eq25_e1456_d_n6 * s.v[57]) + (eq25_e1456 * s.dn[57][6]));
        let eq25_e1458_d_n7: f64 = ((eq25_e1456_d_n7 * s.v[57]) + (eq25_e1456 * s.dn[57][7]));
        let eq25_e1458_d_n8: f64 = ((eq25_e1456_d_n8 * s.v[57]) + (eq25_e1456 * s.dn[57][8]));
        let eq25_e1458_d_n9: f64 = ((eq25_e1456_d_n9 * s.v[57]) + (eq25_e1456 * s.dn[57][9]));
        let eq25_e1458_d_n10: f64 = ((eq25_e1456_d_n10 * s.v[57]) + (eq25_e1456 * s.dn[57][10]));
        let eq25_e1458_d_n11: f64 = ((eq25_e1456_d_n11 * s.v[57]) + (eq25_e1456 * s.dn[57][11]));
        let eq25_e1458_d_n12: f64 = ((eq25_e1456_d_n12 * s.v[57]) + (eq25_e1456 * s.dn[57][12]));
        let eq25_e1458_d_n13: f64 = ((eq25_e1456_d_n13 * s.v[57]) + (eq25_e1456 * s.dn[57][13]));
        let eq25_e1458_d_n14: f64 = ((eq25_e1456_d_n14 * s.v[57]) + (eq25_e1456 * s.dn[57][14]));
        let eq25_e1458_d_n15: f64 = ((eq25_e1456_d_n15 * s.v[57]) + (eq25_e1456 * s.dn[57][15]));
        let eq25_e1458_d_n16: f64 = ((eq25_e1456_d_n16 * s.v[57]) + (eq25_e1456 * s.dn[57][16]));
        let eq25_e1460: f64 = (eq25_e1458 * s.v[188]);
        let eq25_e1460_d_n0: f64 = ((eq25_e1458_d_n0 * s.v[188]) + (eq25_e1458 * s.dn[188][0]));
        let eq25_e1460_d_n1: f64 = ((eq25_e1458_d_n1 * s.v[188]) + (eq25_e1458 * s.dn[188][1]));
        let eq25_e1460_d_n2: f64 = ((eq25_e1458_d_n2 * s.v[188]) + (eq25_e1458 * s.dn[188][2]));
        let eq25_e1460_d_n3: f64 = ((eq25_e1458_d_n3 * s.v[188]) + (eq25_e1458 * s.dn[188][3]));
        let eq25_e1460_d_n4: f64 = ((eq25_e1458_d_n4 * s.v[188]) + (eq25_e1458 * s.dn[188][4]));
        let eq25_e1460_d_n5: f64 = ((eq25_e1458_d_n5 * s.v[188]) + (eq25_e1458 * s.dn[188][5]));
        let eq25_e1460_d_n6: f64 = ((eq25_e1458_d_n6 * s.v[188]) + (eq25_e1458 * s.dn[188][6]));
        let eq25_e1460_d_n7: f64 = ((eq25_e1458_d_n7 * s.v[188]) + (eq25_e1458 * s.dn[188][7]));
        let eq25_e1460_d_n8: f64 = ((eq25_e1458_d_n8 * s.v[188]) + (eq25_e1458 * s.dn[188][8]));
        let eq25_e1460_d_n9: f64 = ((eq25_e1458_d_n9 * s.v[188]) + (eq25_e1458 * s.dn[188][9]));
        let eq25_e1460_d_n10: f64 = ((eq25_e1458_d_n10 * s.v[188]) + (eq25_e1458 * s.dn[188][10]));
        let eq25_e1460_d_n11: f64 = ((eq25_e1458_d_n11 * s.v[188]) + (eq25_e1458 * s.dn[188][11]));
        let eq25_e1460_d_n12: f64 = ((eq25_e1458_d_n12 * s.v[188]) + (eq25_e1458 * s.dn[188][12]));
        let eq25_e1460_d_n13: f64 = ((eq25_e1458_d_n13 * s.v[188]) + (eq25_e1458 * s.dn[188][13]));
        let eq25_e1460_d_n14: f64 = ((eq25_e1458_d_n14 * s.v[188]) + (eq25_e1458 * s.dn[188][14]));
        let eq25_e1460_d_n15: f64 = ((eq25_e1458_d_n15 * s.v[188]) + (eq25_e1458 * s.dn[188][15]));
        let eq25_e1460_d_n16: f64 = ((eq25_e1458_d_n16 * s.v[188]) + (eq25_e1458 * s.dn[188][16]));
        let eq25_value: f64 = eq25_e1460;
        let eq25_node_derivatives: [f64; 17] = [eq25_e1460_d_n0, eq25_e1460_d_n1, eq25_e1460_d_n2, eq25_e1460_d_n3, eq25_e1460_d_n4, eq25_e1460_d_n5, eq25_e1460_d_n6, eq25_e1460_d_n7, eq25_e1460_d_n8, eq25_e1460_d_n9, eq25_e1460_d_n10, eq25_e1460_d_n11, eq25_e1460_d_n12, eq25_e1460_d_n13, eq25_e1460_d_n14, eq25_e1460_d_n15, eq25_e1460_d_n16];
        let eq25_branch_derivatives: [f64; 0] = [];
        stamper.stamp_current_dense(
            Some(nodes[5]),
            Some(nodes[7]),
            self.multiplicity * (eq25_value),
            &nodes,
            &eq25_node_derivatives,
            &branches,
            &eq25_branch_derivatives,
            self.multiplicity,
        );
    }

    pub(super) fn stamp_transient_equation_26_block_0(
        &mut self,
        ctx: &GeneratedEvalContext<'_>,
        stamper: &mut GeneratedStamper<'_>,
        s: &mut Scratch,
    ) {
        let _ = stamper;
        let p = self.params;
        let nodes = self.nodes;
        let branches = self.branches;
        let (eq26_e1464, eq26_e1464_d_n0, eq26_e1464_d_n1, eq26_e1464_d_n2, eq26_e1464_d_n3, eq26_e1464_d_n4, eq26_e1464_d_n5, eq26_e1464_d_n6, eq26_e1464_d_n7, eq26_e1464_d_n8, eq26_e1464_d_n9, eq26_e1464_d_n10, eq26_e1464_d_n11, eq26_e1464_d_n12, eq26_e1464_d_n13, eq26_e1464_d_n14, eq26_e1464_d_n15, eq26_e1464_d_n16,) = {
    if (s.v[1609] != 0.0) {
        (s.v[831], s.dn[831][0], s.dn[831][1], s.dn[831][2], s.dn[831][3], s.dn[831][4], s.dn[831][5], s.dn[831][6], s.dn[831][7], s.dn[831][8], s.dn[831][9], s.dn[831][10], s.dn[831][11], s.dn[831][12], s.dn[831][13], s.dn[831][14], s.dn[831][15], s.dn[831][16],)
    } else {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    }
};
        let eq26_value: f64 = eq26_e1464;
        let eq26_node_derivatives: [f64; 17] = [eq26_e1464_d_n0, eq26_e1464_d_n1, eq26_e1464_d_n2, eq26_e1464_d_n3, eq26_e1464_d_n4, eq26_e1464_d_n5, eq26_e1464_d_n6, eq26_e1464_d_n7, eq26_e1464_d_n8, eq26_e1464_d_n9, eq26_e1464_d_n10, eq26_e1464_d_n11, eq26_e1464_d_n12, eq26_e1464_d_n13, eq26_e1464_d_n14, eq26_e1464_d_n15, eq26_e1464_d_n16];
        let eq26_branch_derivatives: [f64; 0] = [];
        stamper.stamp_current_dense(
            Some(nodes[9]),
            Some(nodes[11]),
            self.multiplicity * (eq26_value),
            &nodes,
            &eq26_node_derivatives,
            &branches,
            &eq26_branch_derivatives,
            self.multiplicity,
        );
    }

    pub(super) fn stamp_transient_equation_27_block_0(
        &mut self,
        ctx: &GeneratedEvalContext<'_>,
        stamper: &mut GeneratedStamper<'_>,
        s: &mut Scratch,
    ) {
        let _ = stamper;
        let p = self.params;
        let nodes = self.nodes;
        let branches = self.branches;
        let (eq27_e1470, eq27_e1470_d_n0, eq27_e1470_d_n1, eq27_e1470_d_n2, eq27_e1470_d_n3, eq27_e1470_d_n4, eq27_e1470_d_n5, eq27_e1470_d_n6, eq27_e1470_d_n7, eq27_e1470_d_n8, eq27_e1470_d_n9, eq27_e1470_d_n10, eq27_e1470_d_n11, eq27_e1470_d_n12, eq27_e1470_d_n13, eq27_e1470_d_n14, eq27_e1470_d_n15, eq27_e1470_d_n16,) = {
    if (s.v[1610] != 0.0) {
        let eq27_e1468: f64 = (s.v[827] + s.v[829]);
        let eq27_e1468_d_n0: f64 = (s.dn[827][0] + s.dn[829][0]);
        let eq27_e1468_d_n1: f64 = (s.dn[827][1] + s.dn[829][1]);
        let eq27_e1468_d_n2: f64 = (s.dn[827][2] + s.dn[829][2]);
        let eq27_e1468_d_n3: f64 = (s.dn[827][3] + s.dn[829][3]);
        let eq27_e1468_d_n4: f64 = (s.dn[827][4] + s.dn[829][4]);
        let eq27_e1468_d_n5: f64 = (s.dn[827][5] + s.dn[829][5]);
        let eq27_e1468_d_n6: f64 = (s.dn[827][6] + s.dn[829][6]);
        let eq27_e1468_d_n7: f64 = (s.dn[827][7] + s.dn[829][7]);
        let eq27_e1468_d_n8: f64 = (s.dn[827][8] + s.dn[829][8]);
        let eq27_e1468_d_n9: f64 = (s.dn[827][9] + s.dn[829][9]);
        let eq27_e1468_d_n10: f64 = (s.dn[827][10] + s.dn[829][10]);
        let eq27_e1468_d_n11: f64 = (s.dn[827][11] + s.dn[829][11]);
        let eq27_e1468_d_n12: f64 = (s.dn[827][12] + s.dn[829][12]);
        let eq27_e1468_d_n13: f64 = (s.dn[827][13] + s.dn[829][13]);
        let eq27_e1468_d_n14: f64 = (s.dn[827][14] + s.dn[829][14]);
        let eq27_e1468_d_n15: f64 = (s.dn[827][15] + s.dn[829][15]);
        let eq27_e1468_d_n16: f64 = (s.dn[827][16] + s.dn[829][16]);
        (eq27_e1468, eq27_e1468_d_n0, eq27_e1468_d_n1, eq27_e1468_d_n2, eq27_e1468_d_n3, eq27_e1468_d_n4, eq27_e1468_d_n5, eq27_e1468_d_n6, eq27_e1468_d_n7, eq27_e1468_d_n8, eq27_e1468_d_n9, eq27_e1468_d_n10, eq27_e1468_d_n11, eq27_e1468_d_n12, eq27_e1468_d_n13, eq27_e1468_d_n14, eq27_e1468_d_n15, eq27_e1468_d_n16,)
    } else {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    }
};
        let eq27_value: f64 = eq27_e1470;
        let eq27_node_derivatives: [f64; 17] = [eq27_e1470_d_n0, eq27_e1470_d_n1, eq27_e1470_d_n2, eq27_e1470_d_n3, eq27_e1470_d_n4, eq27_e1470_d_n5, eq27_e1470_d_n6, eq27_e1470_d_n7, eq27_e1470_d_n8, eq27_e1470_d_n9, eq27_e1470_d_n10, eq27_e1470_d_n11, eq27_e1470_d_n12, eq27_e1470_d_n13, eq27_e1470_d_n14, eq27_e1470_d_n15, eq27_e1470_d_n16];
        let eq27_branch_derivatives: [f64; 0] = [];
        stamper.stamp_current_dense(
            Some(nodes[9]),
            Some(nodes[7]),
            self.multiplicity * (eq27_value),
            &nodes,
            &eq27_node_derivatives,
            &branches,
            &eq27_branch_derivatives,
            self.multiplicity,
        );
    }

    pub(super) fn stamp_transient_equation_28_block_0(
        &mut self,
        ctx: &GeneratedEvalContext<'_>,
        stamper: &mut GeneratedStamper<'_>,
        s: &mut Scratch,
    ) {
        let _ = stamper;
        let p = self.params;
        let nodes = self.nodes;
        let branches = self.branches;
        let (eq28_e1476, eq28_e1476_d_n0, eq28_e1476_d_n1, eq28_e1476_d_n2, eq28_e1476_d_n3, eq28_e1476_d_n4, eq28_e1476_d_n5, eq28_e1476_d_n6, eq28_e1476_d_n7, eq28_e1476_d_n8, eq28_e1476_d_n9, eq28_e1476_d_n10, eq28_e1476_d_n11, eq28_e1476_d_n12, eq28_e1476_d_n13, eq28_e1476_d_n14, eq28_e1476_d_n15, eq28_e1476_d_n16,) = {
    if (s.v[1610] != 0.0) {
        let eq28_e1474: f64 = (s.v[828] + s.v[830]);
        let eq28_e1474_d_n0: f64 = (s.dn[828][0] + s.dn[830][0]);
        let eq28_e1474_d_n1: f64 = (s.dn[828][1] + s.dn[830][1]);
        let eq28_e1474_d_n2: f64 = (s.dn[828][2] + s.dn[830][2]);
        let eq28_e1474_d_n3: f64 = (s.dn[828][3] + s.dn[830][3]);
        let eq28_e1474_d_n4: f64 = (s.dn[828][4] + s.dn[830][4]);
        let eq28_e1474_d_n5: f64 = (s.dn[828][5] + s.dn[830][5]);
        let eq28_e1474_d_n6: f64 = (s.dn[828][6] + s.dn[830][6]);
        let eq28_e1474_d_n7: f64 = (s.dn[828][7] + s.dn[830][7]);
        let eq28_e1474_d_n8: f64 = (s.dn[828][8] + s.dn[830][8]);
        let eq28_e1474_d_n9: f64 = (s.dn[828][9] + s.dn[830][9]);
        let eq28_e1474_d_n10: f64 = (s.dn[828][10] + s.dn[830][10]);
        let eq28_e1474_d_n11: f64 = (s.dn[828][11] + s.dn[830][11]);
        let eq28_e1474_d_n12: f64 = (s.dn[828][12] + s.dn[830][12]);
        let eq28_e1474_d_n13: f64 = (s.dn[828][13] + s.dn[830][13]);
        let eq28_e1474_d_n14: f64 = (s.dn[828][14] + s.dn[830][14]);
        let eq28_e1474_d_n15: f64 = (s.dn[828][15] + s.dn[830][15]);
        let eq28_e1474_d_n16: f64 = (s.dn[828][16] + s.dn[830][16]);
        (eq28_e1474, eq28_e1474_d_n0, eq28_e1474_d_n1, eq28_e1474_d_n2, eq28_e1474_d_n3, eq28_e1474_d_n4, eq28_e1474_d_n5, eq28_e1474_d_n6, eq28_e1474_d_n7, eq28_e1474_d_n8, eq28_e1474_d_n9, eq28_e1474_d_n10, eq28_e1474_d_n11, eq28_e1474_d_n12, eq28_e1474_d_n13, eq28_e1474_d_n14, eq28_e1474_d_n15, eq28_e1474_d_n16,)
    } else {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    }
};
        let eq28_value: f64 = eq28_e1476;
        let eq28_node_derivatives: [f64; 17] = [eq28_e1476_d_n0, eq28_e1476_d_n1, eq28_e1476_d_n2, eq28_e1476_d_n3, eq28_e1476_d_n4, eq28_e1476_d_n5, eq28_e1476_d_n6, eq28_e1476_d_n7, eq28_e1476_d_n8, eq28_e1476_d_n9, eq28_e1476_d_n10, eq28_e1476_d_n11, eq28_e1476_d_n12, eq28_e1476_d_n13, eq28_e1476_d_n14, eq28_e1476_d_n15, eq28_e1476_d_n16];
        let eq28_branch_derivatives: [f64; 0] = [];
        stamper.stamp_current_dense(
            Some(nodes[9]),
            Some(nodes[5]),
            self.multiplicity * (eq28_value),
            &nodes,
            &eq28_node_derivatives,
            &branches,
            &eq28_branch_derivatives,
            self.multiplicity,
        );
    }

    pub(super) fn stamp_transient_equation_29_block_0(
        &mut self,
        ctx: &GeneratedEvalContext<'_>,
        stamper: &mut GeneratedStamper<'_>,
        s: &mut Scratch,
    ) {
        let _ = stamper;
        let p = self.params;
        let nodes = self.nodes;
        let branches = self.branches;
        let (eq29_e1482, eq29_e1482_d_n0, eq29_e1482_d_n1, eq29_e1482_d_n2, eq29_e1482_d_n3, eq29_e1482_d_n4, eq29_e1482_d_n5, eq29_e1482_d_n6, eq29_e1482_d_n7, eq29_e1482_d_n8, eq29_e1482_d_n9, eq29_e1482_d_n10, eq29_e1482_d_n11, eq29_e1482_d_n12, eq29_e1482_d_n13, eq29_e1482_d_n14, eq29_e1482_d_n15, eq29_e1482_d_n16,) = {
    if (s.v[1611] != 0.0) {
        let eq29_e1480: f64 = (s.v[824] + s.v[825]);
        let eq29_e1480_d_n0: f64 = (s.dn[824][0] + s.dn[825][0]);
        let eq29_e1480_d_n1: f64 = (s.dn[824][1] + s.dn[825][1]);
        let eq29_e1480_d_n2: f64 = (s.dn[824][2] + s.dn[825][2]);
        let eq29_e1480_d_n3: f64 = (s.dn[824][3] + s.dn[825][3]);
        let eq29_e1480_d_n4: f64 = (s.dn[824][4] + s.dn[825][4]);
        let eq29_e1480_d_n5: f64 = (s.dn[824][5] + s.dn[825][5]);
        let eq29_e1480_d_n6: f64 = (s.dn[824][6] + s.dn[825][6]);
        let eq29_e1480_d_n7: f64 = (s.dn[824][7] + s.dn[825][7]);
        let eq29_e1480_d_n8: f64 = (s.dn[824][8] + s.dn[825][8]);
        let eq29_e1480_d_n9: f64 = (s.dn[824][9] + s.dn[825][9]);
        let eq29_e1480_d_n10: f64 = (s.dn[824][10] + s.dn[825][10]);
        let eq29_e1480_d_n11: f64 = (s.dn[824][11] + s.dn[825][11]);
        let eq29_e1480_d_n12: f64 = (s.dn[824][12] + s.dn[825][12]);
        let eq29_e1480_d_n13: f64 = (s.dn[824][13] + s.dn[825][13]);
        let eq29_e1480_d_n14: f64 = (s.dn[824][14] + s.dn[825][14]);
        let eq29_e1480_d_n15: f64 = (s.dn[824][15] + s.dn[825][15]);
        let eq29_e1480_d_n16: f64 = (s.dn[824][16] + s.dn[825][16]);
        (eq29_e1480, eq29_e1480_d_n0, eq29_e1480_d_n1, eq29_e1480_d_n2, eq29_e1480_d_n3, eq29_e1480_d_n4, eq29_e1480_d_n5, eq29_e1480_d_n6, eq29_e1480_d_n7, eq29_e1480_d_n8, eq29_e1480_d_n9, eq29_e1480_d_n10, eq29_e1480_d_n11, eq29_e1480_d_n12, eq29_e1480_d_n13, eq29_e1480_d_n14, eq29_e1480_d_n15, eq29_e1480_d_n16,)
    } else {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    }
};
        let eq29_value: f64 = eq29_e1482;
        let eq29_node_derivatives: [f64; 17] = [eq29_e1482_d_n0, eq29_e1482_d_n1, eq29_e1482_d_n2, eq29_e1482_d_n3, eq29_e1482_d_n4, eq29_e1482_d_n5, eq29_e1482_d_n6, eq29_e1482_d_n7, eq29_e1482_d_n8, eq29_e1482_d_n9, eq29_e1482_d_n10, eq29_e1482_d_n11, eq29_e1482_d_n12, eq29_e1482_d_n13, eq29_e1482_d_n14, eq29_e1482_d_n15, eq29_e1482_d_n16];
        let eq29_branch_derivatives: [f64; 0] = [];
        stamper.stamp_current_dense(
            Some(nodes[5]),
            Some(nodes[11]),
            self.multiplicity * (eq29_value),
            &nodes,
            &eq29_node_derivatives,
            &branches,
            &eq29_branch_derivatives,
            self.multiplicity,
        );
    }

    pub(super) fn stamp_transient_equation_30_block_0(
        &mut self,
        ctx: &GeneratedEvalContext<'_>,
        stamper: &mut GeneratedStamper<'_>,
        s: &mut Scratch,
    ) {
        let _ = stamper;
        let p = self.params;
        let nodes = self.nodes;
        let branches = self.branches;
        let (eq30_e1490, eq30_e1490_d_n0, eq30_e1490_d_n1, eq30_e1490_d_n2, eq30_e1490_d_n3, eq30_e1490_d_n4, eq30_e1490_d_n5, eq30_e1490_d_n6, eq30_e1490_d_n7, eq30_e1490_d_n8, eq30_e1490_d_n9, eq30_e1490_d_n10, eq30_e1490_d_n11, eq30_e1490_d_n12, eq30_e1490_d_n13, eq30_e1490_d_n14, eq30_e1490_d_n15, eq30_e1490_d_n16,) = {
    if (s.v[1611] != 0.0) {
        let eq30_e1486: f64 = (p.p28 * s.v[187]);
        let eq30_e1486_d_n0: f64 = (p.p28 * s.dn[187][0]);
        let eq30_e1486_d_n1: f64 = (p.p28 * s.dn[187][1]);
        let eq30_e1486_d_n2: f64 = (p.p28 * s.dn[187][2]);
        let eq30_e1486_d_n3: f64 = (p.p28 * s.dn[187][3]);
        let eq30_e1486_d_n4: f64 = (p.p28 * s.dn[187][4]);
        let eq30_e1486_d_n5: f64 = (p.p28 * s.dn[187][5]);
        let eq30_e1486_d_n6: f64 = (p.p28 * s.dn[187][6]);
        let eq30_e1486_d_n7: f64 = (p.p28 * s.dn[187][7]);
        let eq30_e1486_d_n8: f64 = (p.p28 * s.dn[187][8]);
        let eq30_e1486_d_n9: f64 = (p.p28 * s.dn[187][9]);
        let eq30_e1486_d_n10: f64 = (p.p28 * s.dn[187][10]);
        let eq30_e1486_d_n11: f64 = (p.p28 * s.dn[187][11]);
        let eq30_e1486_d_n12: f64 = (p.p28 * s.dn[187][12]);
        let eq30_e1486_d_n13: f64 = (p.p28 * s.dn[187][13]);
        let eq30_e1486_d_n14: f64 = (p.p28 * s.dn[187][14]);
        let eq30_e1486_d_n15: f64 = (p.p28 * s.dn[187][15]);
        let eq30_e1486_d_n16: f64 = (p.p28 * s.dn[187][16]);
        let eq30_e1488: f64 = (eq30_e1486 * s.v[780]);
        let eq30_e1488_d_n0: f64 = ((eq30_e1486_d_n0 * s.v[780]) + (eq30_e1486 * s.dn[780][0]));
        let eq30_e1488_d_n1: f64 = ((eq30_e1486_d_n1 * s.v[780]) + (eq30_e1486 * s.dn[780][1]));
        let eq30_e1488_d_n2: f64 = ((eq30_e1486_d_n2 * s.v[780]) + (eq30_e1486 * s.dn[780][2]));
        let eq30_e1488_d_n3: f64 = ((eq30_e1486_d_n3 * s.v[780]) + (eq30_e1486 * s.dn[780][3]));
        let eq30_e1488_d_n4: f64 = ((eq30_e1486_d_n4 * s.v[780]) + (eq30_e1486 * s.dn[780][4]));
        let eq30_e1488_d_n5: f64 = ((eq30_e1486_d_n5 * s.v[780]) + (eq30_e1486 * s.dn[780][5]));
        let eq30_e1488_d_n6: f64 = ((eq30_e1486_d_n6 * s.v[780]) + (eq30_e1486 * s.dn[780][6]));
        let eq30_e1488_d_n7: f64 = ((eq30_e1486_d_n7 * s.v[780]) + (eq30_e1486 * s.dn[780][7]));
        let eq30_e1488_d_n8: f64 = ((eq30_e1486_d_n8 * s.v[780]) + (eq30_e1486 * s.dn[780][8]));
        let eq30_e1488_d_n9: f64 = ((eq30_e1486_d_n9 * s.v[780]) + (eq30_e1486 * s.dn[780][9]));
        let eq30_e1488_d_n10: f64 = ((eq30_e1486_d_n10 * s.v[780]) + (eq30_e1486 * s.dn[780][10]));
        let eq30_e1488_d_n11: f64 = ((eq30_e1486_d_n11 * s.v[780]) + (eq30_e1486 * s.dn[780][11]));
        let eq30_e1488_d_n12: f64 = ((eq30_e1486_d_n12 * s.v[780]) + (eq30_e1486 * s.dn[780][12]));
        let eq30_e1488_d_n13: f64 = ((eq30_e1486_d_n13 * s.v[780]) + (eq30_e1486 * s.dn[780][13]));
        let eq30_e1488_d_n14: f64 = ((eq30_e1486_d_n14 * s.v[780]) + (eq30_e1486 * s.dn[780][14]));
        let eq30_e1488_d_n15: f64 = ((eq30_e1486_d_n15 * s.v[780]) + (eq30_e1486 * s.dn[780][15]));
        let eq30_e1488_d_n16: f64 = ((eq30_e1486_d_n16 * s.v[780]) + (eq30_e1486 * s.dn[780][16]));
        (eq30_e1488, eq30_e1488_d_n0, eq30_e1488_d_n1, eq30_e1488_d_n2, eq30_e1488_d_n3, eq30_e1488_d_n4, eq30_e1488_d_n5, eq30_e1488_d_n6, eq30_e1488_d_n7, eq30_e1488_d_n8, eq30_e1488_d_n9, eq30_e1488_d_n10, eq30_e1488_d_n11, eq30_e1488_d_n12, eq30_e1488_d_n13, eq30_e1488_d_n14, eq30_e1488_d_n15, eq30_e1488_d_n16,)
    } else {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    }
};
        let eq30_value: f64 = eq30_e1490;
        let eq30_node_derivatives: [f64; 17] = [eq30_e1490_d_n0, eq30_e1490_d_n1, eq30_e1490_d_n2, eq30_e1490_d_n3, eq30_e1490_d_n4, eq30_e1490_d_n5, eq30_e1490_d_n6, eq30_e1490_d_n7, eq30_e1490_d_n8, eq30_e1490_d_n9, eq30_e1490_d_n10, eq30_e1490_d_n11, eq30_e1490_d_n12, eq30_e1490_d_n13, eq30_e1490_d_n14, eq30_e1490_d_n15, eq30_e1490_d_n16];
        let eq30_branch_derivatives: [f64; 0] = [];
        stamper.stamp_current_dense(
            Some(nodes[6]),
            Some(nodes[11]),
            self.multiplicity * (eq30_value),
            &nodes,
            &eq30_node_derivatives,
            &branches,
            &eq30_branch_derivatives,
            self.multiplicity,
        );
    }

    pub(super) fn stamp_transient_equation_31_block_0(
        &mut self,
        ctx: &GeneratedEvalContext<'_>,
        stamper: &mut GeneratedStamper<'_>,
        s: &mut Scratch,
    ) {
        let _ = stamper;
        let p = self.params;
        let nodes = self.nodes;
        let branches = self.branches;
        let (eq31_e1494, eq31_e1494_d_n0, eq31_e1494_d_n1, eq31_e1494_d_n2, eq31_e1494_d_n3, eq31_e1494_d_n4, eq31_e1494_d_n5, eq31_e1494_d_n6, eq31_e1494_d_n7, eq31_e1494_d_n8, eq31_e1494_d_n9, eq31_e1494_d_n10, eq31_e1494_d_n11, eq31_e1494_d_n12, eq31_e1494_d_n13, eq31_e1494_d_n14, eq31_e1494_d_n15, eq31_e1494_d_n16,) = {
    if (s.v[1611] != 0.0) {
        (s.v[826], s.dn[826][0], s.dn[826][1], s.dn[826][2], s.dn[826][3], s.dn[826][4], s.dn[826][5], s.dn[826][6], s.dn[826][7], s.dn[826][8], s.dn[826][9], s.dn[826][10], s.dn[826][11], s.dn[826][12], s.dn[826][13], s.dn[826][14], s.dn[826][15], s.dn[826][16],)
    } else {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    }
};
        let eq31_value: f64 = eq31_e1494;
        let eq31_node_derivatives: [f64; 17] = [eq31_e1494_d_n0, eq31_e1494_d_n1, eq31_e1494_d_n2, eq31_e1494_d_n3, eq31_e1494_d_n4, eq31_e1494_d_n5, eq31_e1494_d_n6, eq31_e1494_d_n7, eq31_e1494_d_n8, eq31_e1494_d_n9, eq31_e1494_d_n10, eq31_e1494_d_n11, eq31_e1494_d_n12, eq31_e1494_d_n13, eq31_e1494_d_n14, eq31_e1494_d_n15, eq31_e1494_d_n16];
        let eq31_branch_derivatives: [f64; 0] = [];
        stamper.stamp_current_dense(
            Some(nodes[7]),
            Some(nodes[11]),
            self.multiplicity * (eq31_value),
            &nodes,
            &eq31_node_derivatives,
            &branches,
            &eq31_branch_derivatives,
            self.multiplicity,
        );
    }

    pub(super) fn stamp_transient_equation_32_block_0(
        &mut self,
        ctx: &GeneratedEvalContext<'_>,
        stamper: &mut GeneratedStamper<'_>,
        s: &mut Scratch,
    ) {
        let _ = stamper;
        let p = self.params;
        let nodes = self.nodes;
        let branches = self.branches;
        let (eq32_e1499, eq32_e1499_d_n0, eq32_e1499_d_n1, eq32_e1499_d_n2, eq32_e1499_d_n3, eq32_e1499_d_n4, eq32_e1499_d_n5, eq32_e1499_d_n6, eq32_e1499_d_n7, eq32_e1499_d_n8, eq32_e1499_d_n9, eq32_e1499_d_n10, eq32_e1499_d_n11, eq32_e1499_d_n12, eq32_e1499_d_n13, eq32_e1499_d_n14, eq32_e1499_d_n15, eq32_e1499_d_n16,) = {
    if (!(s.v[1611] != 0.0)) {
        (s.v[825], s.dn[825][0], s.dn[825][1], s.dn[825][2], s.dn[825][3], s.dn[825][4], s.dn[825][5], s.dn[825][6], s.dn[825][7], s.dn[825][8], s.dn[825][9], s.dn[825][10], s.dn[825][11], s.dn[825][12], s.dn[825][13], s.dn[825][14], s.dn[825][15], s.dn[825][16],)
    } else {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    }
};
        let eq32_value: f64 = eq32_e1499;
        let eq32_node_derivatives: [f64; 17] = [eq32_e1499_d_n0, eq32_e1499_d_n1, eq32_e1499_d_n2, eq32_e1499_d_n3, eq32_e1499_d_n4, eq32_e1499_d_n5, eq32_e1499_d_n6, eq32_e1499_d_n7, eq32_e1499_d_n8, eq32_e1499_d_n9, eq32_e1499_d_n10, eq32_e1499_d_n11, eq32_e1499_d_n12, eq32_e1499_d_n13, eq32_e1499_d_n14, eq32_e1499_d_n15, eq32_e1499_d_n16];
        let eq32_branch_derivatives: [f64; 0] = [];
        stamper.stamp_current_dense(
            Some(nodes[5]),
            Some(nodes[11]),
            self.multiplicity * (eq32_value),
            &nodes,
            &eq32_node_derivatives,
            &branches,
            &eq32_branch_derivatives,
            self.multiplicity,
        );
    }

    pub(super) fn stamp_transient_equation_33_block_0(
        &mut self,
        ctx: &GeneratedEvalContext<'_>,
        stamper: &mut GeneratedStamper<'_>,
        s: &mut Scratch,
    ) {
        let _ = stamper;
        let p = self.params;
        let nodes = self.nodes;
        let branches = self.branches;
        let (eq33_e1506, eq33_e1506_d_n0, eq33_e1506_d_n1, eq33_e1506_d_n2, eq33_e1506_d_n3, eq33_e1506_d_n4, eq33_e1506_d_n5, eq33_e1506_d_n6, eq33_e1506_d_n7, eq33_e1506_d_n8, eq33_e1506_d_n9, eq33_e1506_d_n10, eq33_e1506_d_n11, eq33_e1506_d_n12, eq33_e1506_d_n13, eq33_e1506_d_n14, eq33_e1506_d_n15, eq33_e1506_d_n16,) = {
    if (!(s.v[1611] != 0.0)) {
        let eq33_e1504: f64 = (s.v[824] + s.v[826]);
        let eq33_e1504_d_n0: f64 = (s.dn[824][0] + s.dn[826][0]);
        let eq33_e1504_d_n1: f64 = (s.dn[824][1] + s.dn[826][1]);
        let eq33_e1504_d_n2: f64 = (s.dn[824][2] + s.dn[826][2]);
        let eq33_e1504_d_n3: f64 = (s.dn[824][3] + s.dn[826][3]);
        let eq33_e1504_d_n4: f64 = (s.dn[824][4] + s.dn[826][4]);
        let eq33_e1504_d_n5: f64 = (s.dn[824][5] + s.dn[826][5]);
        let eq33_e1504_d_n6: f64 = (s.dn[824][6] + s.dn[826][6]);
        let eq33_e1504_d_n7: f64 = (s.dn[824][7] + s.dn[826][7]);
        let eq33_e1504_d_n8: f64 = (s.dn[824][8] + s.dn[826][8]);
        let eq33_e1504_d_n9: f64 = (s.dn[824][9] + s.dn[826][9]);
        let eq33_e1504_d_n10: f64 = (s.dn[824][10] + s.dn[826][10]);
        let eq33_e1504_d_n11: f64 = (s.dn[824][11] + s.dn[826][11]);
        let eq33_e1504_d_n12: f64 = (s.dn[824][12] + s.dn[826][12]);
        let eq33_e1504_d_n13: f64 = (s.dn[824][13] + s.dn[826][13]);
        let eq33_e1504_d_n14: f64 = (s.dn[824][14] + s.dn[826][14]);
        let eq33_e1504_d_n15: f64 = (s.dn[824][15] + s.dn[826][15]);
        let eq33_e1504_d_n16: f64 = (s.dn[824][16] + s.dn[826][16]);
        (eq33_e1504, eq33_e1504_d_n0, eq33_e1504_d_n1, eq33_e1504_d_n2, eq33_e1504_d_n3, eq33_e1504_d_n4, eq33_e1504_d_n5, eq33_e1504_d_n6, eq33_e1504_d_n7, eq33_e1504_d_n8, eq33_e1504_d_n9, eq33_e1504_d_n10, eq33_e1504_d_n11, eq33_e1504_d_n12, eq33_e1504_d_n13, eq33_e1504_d_n14, eq33_e1504_d_n15, eq33_e1504_d_n16,)
    } else {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    }
};
        let eq33_value: f64 = eq33_e1506;
        let eq33_node_derivatives: [f64; 17] = [eq33_e1506_d_n0, eq33_e1506_d_n1, eq33_e1506_d_n2, eq33_e1506_d_n3, eq33_e1506_d_n4, eq33_e1506_d_n5, eq33_e1506_d_n6, eq33_e1506_d_n7, eq33_e1506_d_n8, eq33_e1506_d_n9, eq33_e1506_d_n10, eq33_e1506_d_n11, eq33_e1506_d_n12, eq33_e1506_d_n13, eq33_e1506_d_n14, eq33_e1506_d_n15, eq33_e1506_d_n16];
        let eq33_branch_derivatives: [f64; 0] = [];
        stamper.stamp_current_dense(
            Some(nodes[7]),
            Some(nodes[11]),
            self.multiplicity * (eq33_value),
            &nodes,
            &eq33_node_derivatives,
            &branches,
            &eq33_branch_derivatives,
            self.multiplicity,
        );
    }

    pub(super) fn stamp_transient_equation_34_block_0(
        &mut self,
        ctx: &GeneratedEvalContext<'_>,
        stamper: &mut GeneratedStamper<'_>,
        s: &mut Scratch,
    ) {
        let _ = stamper;
        let p = self.params;
        let nodes = self.nodes;
        let branches = self.branches;
        let nv0 = ctx.node_voltage(nodes[0]);
        let nv6 = ctx.node_voltage(nodes[6]);
        let (eq34_e1514, eq34_e1514_d_n0, eq34_e1514_d_n1, eq34_e1514_d_n2, eq34_e1514_d_n3, eq34_e1514_d_n4, eq34_e1514_d_n5, eq34_e1514_d_n6, eq34_e1514_d_n7, eq34_e1514_d_n8, eq34_e1514_d_n9, eq34_e1514_d_n10, eq34_e1514_d_n11, eq34_e1514_d_n12, eq34_e1514_d_n13, eq34_e1514_d_n14, eq34_e1514_d_n15, eq34_e1514_d_n16,) = {
    if (s.v[1612] != 0.0) {
        let eq34_e1510: f64 = (p.p28 * (nv0 - nv6));
        let eq34_e1510_d_n0: f64 = p.p28;
        let eq34_e1510_d_n6: f64 = (-p.p28);
        let eq34_e1512: f64 = (eq34_e1510 * s.v[372]);
        let eq34_e1512_d_n0: f64 = ((eq34_e1510_d_n0 * s.v[372]) + (eq34_e1510 * s.dn[372][0]));
        let eq34_e1512_d_n1: f64 = (eq34_e1510 * s.dn[372][1]);
        let eq34_e1512_d_n2: f64 = (eq34_e1510 * s.dn[372][2]);
        let eq34_e1512_d_n3: f64 = (eq34_e1510 * s.dn[372][3]);
        let eq34_e1512_d_n4: f64 = (eq34_e1510 * s.dn[372][4]);
        let eq34_e1512_d_n5: f64 = (eq34_e1510 * s.dn[372][5]);
        let eq34_e1512_d_n6: f64 = ((eq34_e1510_d_n6 * s.v[372]) + (eq34_e1510 * s.dn[372][6]));
        let eq34_e1512_d_n7: f64 = (eq34_e1510 * s.dn[372][7]);
        let eq34_e1512_d_n8: f64 = (eq34_e1510 * s.dn[372][8]);
        let eq34_e1512_d_n9: f64 = (eq34_e1510 * s.dn[372][9]);
        let eq34_e1512_d_n10: f64 = (eq34_e1510 * s.dn[372][10]);
        let eq34_e1512_d_n11: f64 = (eq34_e1510 * s.dn[372][11]);
        let eq34_e1512_d_n12: f64 = (eq34_e1510 * s.dn[372][12]);
        let eq34_e1512_d_n13: f64 = (eq34_e1510 * s.dn[372][13]);
        let eq34_e1512_d_n14: f64 = (eq34_e1510 * s.dn[372][14]);
        let eq34_e1512_d_n15: f64 = (eq34_e1510 * s.dn[372][15]);
        let eq34_e1512_d_n16: f64 = (eq34_e1510 * s.dn[372][16]);
        (eq34_e1512, eq34_e1512_d_n0, eq34_e1512_d_n1, eq34_e1512_d_n2, eq34_e1512_d_n3, eq34_e1512_d_n4, eq34_e1512_d_n5, eq34_e1512_d_n6, eq34_e1512_d_n7, eq34_e1512_d_n8, eq34_e1512_d_n9, eq34_e1512_d_n10, eq34_e1512_d_n11, eq34_e1512_d_n12, eq34_e1512_d_n13, eq34_e1512_d_n14, eq34_e1512_d_n15, eq34_e1512_d_n16,)
    } else {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    }
};
        let eq34_value: f64 = eq34_e1514;
        let eq34_node_derivatives: [f64; 17] = [eq34_e1514_d_n0, eq34_e1514_d_n1, eq34_e1514_d_n2, eq34_e1514_d_n3, eq34_e1514_d_n4, eq34_e1514_d_n5, eq34_e1514_d_n6, eq34_e1514_d_n7, eq34_e1514_d_n8, eq34_e1514_d_n9, eq34_e1514_d_n10, eq34_e1514_d_n11, eq34_e1514_d_n12, eq34_e1514_d_n13, eq34_e1514_d_n14, eq34_e1514_d_n15, eq34_e1514_d_n16];
        let eq34_branch_derivatives: [f64; 0] = [];
        stamper.stamp_current_dense(
            Some(nodes[0]),
            Some(nodes[6]),
            self.multiplicity * (eq34_value),
            &nodes,
            &eq34_node_derivatives,
            &branches,
            &eq34_branch_derivatives,
            self.multiplicity,
        );
    }

    pub(super) fn stamp_transient_equation_35_block_0(
        &mut self,
        ctx: &GeneratedEvalContext<'_>,
        stamper: &mut GeneratedStamper<'_>,
        s: &mut Scratch,
    ) {
        let _ = stamper;
        let p = self.params;
        let nodes = self.nodes;
        let branches = self.branches;
        let (eq35_e1524,) = {
    if (s.v[1612] != 0.0) {
        (0.0,)
    } else {
        (0.0,)
    }
};
        let eq35_value: f64 = eq35_e1524;
        stamper.stamp_current(
            Some(nodes[0]),
            Some(nodes[6]),
            self.multiplicity * (eq35_value),
            &[
            ],
        );
    }

    pub(super) fn stamp_transient_equation_36_block_0(
        &mut self,
        ctx: &GeneratedEvalContext<'_>,
        stamper: &mut GeneratedStamper<'_>,
        s: &mut Scratch,
    ) {
        let _ = stamper;
        let p = self.params;
        let nodes = self.nodes;
        let branches = self.branches;
        let nv5 = ctx.node_voltage(nodes[5]);
        let nv6 = ctx.node_voltage(nodes[6]);
        let (eq36_e1534, eq36_e1534_d_n0, eq36_e1534_d_n1, eq36_e1534_d_n2, eq36_e1534_d_n3, eq36_e1534_d_n4, eq36_e1534_d_n5, eq36_e1534_d_n6, eq36_e1534_d_n7, eq36_e1534_d_n8, eq36_e1534_d_n9, eq36_e1534_d_n10, eq36_e1534_d_n11, eq36_e1534_d_n12, eq36_e1534_d_n13, eq36_e1534_d_n14, eq36_e1534_d_n15, eq36_e1534_d_n16,) = {
    if ((s.v[1612] != 0.0) && (s.v[1613] != 0.0)) {
        let eq36_e1530: f64 = (p.p28 * (nv6 - nv5));
        let eq36_e1530_d_n5: f64 = (-p.p28);
        let eq36_e1530_d_n6: f64 = p.p28;
        let eq36_e1532: f64 = (eq36_e1530 * s.v[374]);
        let eq36_e1532_d_n0: f64 = (eq36_e1530 * s.dn[374][0]);
        let eq36_e1532_d_n1: f64 = (eq36_e1530 * s.dn[374][1]);
        let eq36_e1532_d_n2: f64 = (eq36_e1530 * s.dn[374][2]);
        let eq36_e1532_d_n3: f64 = (eq36_e1530 * s.dn[374][3]);
        let eq36_e1532_d_n4: f64 = (eq36_e1530 * s.dn[374][4]);
        let eq36_e1532_d_n5: f64 = ((eq36_e1530_d_n5 * s.v[374]) + (eq36_e1530 * s.dn[374][5]));
        let eq36_e1532_d_n6: f64 = ((eq36_e1530_d_n6 * s.v[374]) + (eq36_e1530 * s.dn[374][6]));
        let eq36_e1532_d_n7: f64 = (eq36_e1530 * s.dn[374][7]);
        let eq36_e1532_d_n8: f64 = (eq36_e1530 * s.dn[374][8]);
        let eq36_e1532_d_n9: f64 = (eq36_e1530 * s.dn[374][9]);
        let eq36_e1532_d_n10: f64 = (eq36_e1530 * s.dn[374][10]);
        let eq36_e1532_d_n11: f64 = (eq36_e1530 * s.dn[374][11]);
        let eq36_e1532_d_n12: f64 = (eq36_e1530 * s.dn[374][12]);
        let eq36_e1532_d_n13: f64 = (eq36_e1530 * s.dn[374][13]);
        let eq36_e1532_d_n14: f64 = (eq36_e1530 * s.dn[374][14]);
        let eq36_e1532_d_n15: f64 = (eq36_e1530 * s.dn[374][15]);
        let eq36_e1532_d_n16: f64 = (eq36_e1530 * s.dn[374][16]);
        (eq36_e1532, eq36_e1532_d_n0, eq36_e1532_d_n1, eq36_e1532_d_n2, eq36_e1532_d_n3, eq36_e1532_d_n4, eq36_e1532_d_n5, eq36_e1532_d_n6, eq36_e1532_d_n7, eq36_e1532_d_n8, eq36_e1532_d_n9, eq36_e1532_d_n10, eq36_e1532_d_n11, eq36_e1532_d_n12, eq36_e1532_d_n13, eq36_e1532_d_n14, eq36_e1532_d_n15, eq36_e1532_d_n16,)
    } else {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    }
};
        let eq36_value: f64 = eq36_e1534;
        let eq36_node_derivatives: [f64; 17] = [eq36_e1534_d_n0, eq36_e1534_d_n1, eq36_e1534_d_n2, eq36_e1534_d_n3, eq36_e1534_d_n4, eq36_e1534_d_n5, eq36_e1534_d_n6, eq36_e1534_d_n7, eq36_e1534_d_n8, eq36_e1534_d_n9, eq36_e1534_d_n10, eq36_e1534_d_n11, eq36_e1534_d_n12, eq36_e1534_d_n13, eq36_e1534_d_n14, eq36_e1534_d_n15, eq36_e1534_d_n16];
        let eq36_branch_derivatives: [f64; 0] = [];
        stamper.stamp_current_dense(
            Some(nodes[6]),
            Some(nodes[5]),
            self.multiplicity * (eq36_value),
            &nodes,
            &eq36_node_derivatives,
            &branches,
            &eq36_branch_derivatives,
            self.multiplicity,
        );
    }

    pub(super) fn stamp_transient_equation_37_block_0(
        &mut self,
        ctx: &GeneratedEvalContext<'_>,
        stamper: &mut GeneratedStamper<'_>,
        s: &mut Scratch,
    ) {
        let _ = stamper;
        let p = self.params;
        let nodes = self.nodes;
        let branches = self.branches;
        let (eq37_e1546,) = {
    if ((s.v[1612] != 0.0) && (s.v[1613] != 0.0)) {
        (0.0,)
    } else {
        (0.0,)
    }
};
        let eq37_value: f64 = eq37_e1546;
        stamper.stamp_current(
            Some(nodes[6]),
            Some(nodes[5]),
            self.multiplicity * (eq37_value),
            &[
            ],
        );
    }
}
