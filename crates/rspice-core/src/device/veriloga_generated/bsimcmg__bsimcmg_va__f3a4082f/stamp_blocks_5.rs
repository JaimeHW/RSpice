#![allow(dead_code, unused_assignments, unused_imports, unused_parens, unused_variables)]

use super::{A, GeneratedDerivative, GeneratedEvalContext, GeneratedReactiveStamper, GeneratedStamper, Scratch, THERMAL_VOLTAGE_PER_K};
use super::super::state::Instance;

impl Instance {

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
        let (eq23_e2154, eq23_e2154_d_n0, eq23_e2154_d_n1, eq23_e2154_d_n2, eq23_e2154_d_n3, eq23_e2154_d_n4, eq23_e2154_d_n5, eq23_e2154_d_n6, eq23_e2154_d_n7, eq23_e2154_d_n8, eq23_e2154_d_n9, eq23_e2154_d_n10, eq23_e2154_d_n11, eq23_e2154_d_n12, eq23_e2154_d_n13, eq23_e2154_d_n14, eq23_e2154_d_n15, eq23_e2154_d_n16,) = {
    if (((!(s.v[1698] != 0.0)) && (s.v[1701] != 0.0)) && (s.v[1702] != 0.0)) {
        let eq23_e2151: f64 = (s.v[476] + s.v[488]);
        let eq23_e2151_d_n0: f64 = (s.dn[476][0] + s.dn[488][0]);
        let eq23_e2151_d_n1: f64 = (s.dn[476][1] + s.dn[488][1]);
        let eq23_e2151_d_n2: f64 = (s.dn[476][2] + s.dn[488][2]);
        let eq23_e2151_d_n3: f64 = (s.dn[476][3] + s.dn[488][3]);
        let eq23_e2151_d_n4: f64 = (s.dn[476][4] + s.dn[488][4]);
        let eq23_e2151_d_n5: f64 = (s.dn[476][5] + s.dn[488][5]);
        let eq23_e2151_d_n6: f64 = (s.dn[476][6] + s.dn[488][6]);
        let eq23_e2151_d_n7: f64 = (s.dn[476][7] + s.dn[488][7]);
        let eq23_e2151_d_n8: f64 = (s.dn[476][8] + s.dn[488][8]);
        let eq23_e2151_d_n9: f64 = (s.dn[476][9] + s.dn[488][9]);
        let eq23_e2151_d_n10: f64 = (s.dn[476][10] + s.dn[488][10]);
        let eq23_e2151_d_n11: f64 = (s.dn[476][11] + s.dn[488][11]);
        let eq23_e2151_d_n12: f64 = (s.dn[476][12] + s.dn[488][12]);
        let eq23_e2151_d_n13: f64 = (s.dn[476][13] + s.dn[488][13]);
        let eq23_e2151_d_n14: f64 = (s.dn[476][14] + s.dn[488][14]);
        let eq23_e2151_d_n15: f64 = (s.dn[476][15] + s.dn[488][15]);
        let eq23_e2151_d_n16: f64 = (s.dn[476][16] + s.dn[488][16]);
        let eq23_e2152: f64 = (s.v[114] * eq23_e2151);
        let eq23_e2152_d_n0: f64 = ((s.dn[114][0] * eq23_e2151) + (s.v[114] * eq23_e2151_d_n0));
        let eq23_e2152_d_n1: f64 = ((s.dn[114][1] * eq23_e2151) + (s.v[114] * eq23_e2151_d_n1));
        let eq23_e2152_d_n2: f64 = ((s.dn[114][2] * eq23_e2151) + (s.v[114] * eq23_e2151_d_n2));
        let eq23_e2152_d_n3: f64 = ((s.dn[114][3] * eq23_e2151) + (s.v[114] * eq23_e2151_d_n3));
        let eq23_e2152_d_n4: f64 = ((s.dn[114][4] * eq23_e2151) + (s.v[114] * eq23_e2151_d_n4));
        let eq23_e2152_d_n5: f64 = ((s.dn[114][5] * eq23_e2151) + (s.v[114] * eq23_e2151_d_n5));
        let eq23_e2152_d_n6: f64 = ((s.dn[114][6] * eq23_e2151) + (s.v[114] * eq23_e2151_d_n6));
        let eq23_e2152_d_n7: f64 = ((s.dn[114][7] * eq23_e2151) + (s.v[114] * eq23_e2151_d_n7));
        let eq23_e2152_d_n8: f64 = ((s.dn[114][8] * eq23_e2151) + (s.v[114] * eq23_e2151_d_n8));
        let eq23_e2152_d_n9: f64 = ((s.dn[114][9] * eq23_e2151) + (s.v[114] * eq23_e2151_d_n9));
        let eq23_e2152_d_n10: f64 = ((s.dn[114][10] * eq23_e2151) + (s.v[114] * eq23_e2151_d_n10));
        let eq23_e2152_d_n11: f64 = ((s.dn[114][11] * eq23_e2151) + (s.v[114] * eq23_e2151_d_n11));
        let eq23_e2152_d_n12: f64 = ((s.dn[114][12] * eq23_e2151) + (s.v[114] * eq23_e2151_d_n12));
        let eq23_e2152_d_n13: f64 = ((s.dn[114][13] * eq23_e2151) + (s.v[114] * eq23_e2151_d_n13));
        let eq23_e2152_d_n14: f64 = ((s.dn[114][14] * eq23_e2151) + (s.v[114] * eq23_e2151_d_n14));
        let eq23_e2152_d_n15: f64 = ((s.dn[114][15] * eq23_e2151) + (s.v[114] * eq23_e2151_d_n15));
        let eq23_e2152_d_n16: f64 = ((s.dn[114][16] * eq23_e2151) + (s.v[114] * eq23_e2151_d_n16));
        (eq23_e2152, eq23_e2152_d_n0, eq23_e2152_d_n1, eq23_e2152_d_n2, eq23_e2152_d_n3, eq23_e2152_d_n4, eq23_e2152_d_n5, eq23_e2152_d_n6, eq23_e2152_d_n7, eq23_e2152_d_n8, eq23_e2152_d_n9, eq23_e2152_d_n10, eq23_e2152_d_n11, eq23_e2152_d_n12, eq23_e2152_d_n13, eq23_e2152_d_n14, eq23_e2152_d_n15, eq23_e2152_d_n16,)
    } else {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    }
};
        let eq23_value: f64 = eq23_e2154;
        let eq23_node_derivatives: [f64; 17] = [eq23_e2154_d_n0, eq23_e2154_d_n1, eq23_e2154_d_n2, eq23_e2154_d_n3, eq23_e2154_d_n4, eq23_e2154_d_n5, eq23_e2154_d_n6, eq23_e2154_d_n7, eq23_e2154_d_n8, eq23_e2154_d_n9, eq23_e2154_d_n10, eq23_e2154_d_n11, eq23_e2154_d_n12, eq23_e2154_d_n13, eq23_e2154_d_n14, eq23_e2154_d_n15, eq23_e2154_d_n16];
        let eq23_branch_derivatives: [f64; 0] = [];
        stamper.stamp_current_dense(
            Some(nodes[6]),
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
        let (eq24_e2165, eq24_e2165_d_n0, eq24_e2165_d_n1, eq24_e2165_d_n2, eq24_e2165_d_n3, eq24_e2165_d_n4, eq24_e2165_d_n5, eq24_e2165_d_n6, eq24_e2165_d_n7, eq24_e2165_d_n8, eq24_e2165_d_n9, eq24_e2165_d_n10, eq24_e2165_d_n11, eq24_e2165_d_n12, eq24_e2165_d_n13, eq24_e2165_d_n14, eq24_e2165_d_n15, eq24_e2165_d_n16,) = {
    if (((!(s.v[1698] != 0.0)) && (s.v[1701] != 0.0)) && (s.v[1702] != 0.0)) {
        let eq24_e2163: f64 = (s.v[114] * s.v[475]);
        let eq24_e2163_d_n0: f64 = ((s.dn[114][0] * s.v[475]) + (s.v[114] * s.dn[475][0]));
        let eq24_e2163_d_n1: f64 = ((s.dn[114][1] * s.v[475]) + (s.v[114] * s.dn[475][1]));
        let eq24_e2163_d_n2: f64 = ((s.dn[114][2] * s.v[475]) + (s.v[114] * s.dn[475][2]));
        let eq24_e2163_d_n3: f64 = ((s.dn[114][3] * s.v[475]) + (s.v[114] * s.dn[475][3]));
        let eq24_e2163_d_n4: f64 = ((s.dn[114][4] * s.v[475]) + (s.v[114] * s.dn[475][4]));
        let eq24_e2163_d_n5: f64 = ((s.dn[114][5] * s.v[475]) + (s.v[114] * s.dn[475][5]));
        let eq24_e2163_d_n6: f64 = ((s.dn[114][6] * s.v[475]) + (s.v[114] * s.dn[475][6]));
        let eq24_e2163_d_n7: f64 = ((s.dn[114][7] * s.v[475]) + (s.v[114] * s.dn[475][7]));
        let eq24_e2163_d_n8: f64 = ((s.dn[114][8] * s.v[475]) + (s.v[114] * s.dn[475][8]));
        let eq24_e2163_d_n9: f64 = ((s.dn[114][9] * s.v[475]) + (s.v[114] * s.dn[475][9]));
        let eq24_e2163_d_n10: f64 = ((s.dn[114][10] * s.v[475]) + (s.v[114] * s.dn[475][10]));
        let eq24_e2163_d_n11: f64 = ((s.dn[114][11] * s.v[475]) + (s.v[114] * s.dn[475][11]));
        let eq24_e2163_d_n12: f64 = ((s.dn[114][12] * s.v[475]) + (s.v[114] * s.dn[475][12]));
        let eq24_e2163_d_n13: f64 = ((s.dn[114][13] * s.v[475]) + (s.v[114] * s.dn[475][13]));
        let eq24_e2163_d_n14: f64 = ((s.dn[114][14] * s.v[475]) + (s.v[114] * s.dn[475][14]));
        let eq24_e2163_d_n15: f64 = ((s.dn[114][15] * s.v[475]) + (s.v[114] * s.dn[475][15]));
        let eq24_e2163_d_n16: f64 = ((s.dn[114][16] * s.v[475]) + (s.v[114] * s.dn[475][16]));
        (eq24_e2163, eq24_e2163_d_n0, eq24_e2163_d_n1, eq24_e2163_d_n2, eq24_e2163_d_n3, eq24_e2163_d_n4, eq24_e2163_d_n5, eq24_e2163_d_n6, eq24_e2163_d_n7, eq24_e2163_d_n8, eq24_e2163_d_n9, eq24_e2163_d_n10, eq24_e2163_d_n11, eq24_e2163_d_n12, eq24_e2163_d_n13, eq24_e2163_d_n14, eq24_e2163_d_n15, eq24_e2163_d_n16,)
    } else {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    }
};
        let eq24_value: f64 = eq24_e2165;
        let eq24_node_derivatives: [f64; 17] = [eq24_e2165_d_n0, eq24_e2165_d_n1, eq24_e2165_d_n2, eq24_e2165_d_n3, eq24_e2165_d_n4, eq24_e2165_d_n5, eq24_e2165_d_n6, eq24_e2165_d_n7, eq24_e2165_d_n8, eq24_e2165_d_n9, eq24_e2165_d_n10, eq24_e2165_d_n11, eq24_e2165_d_n12, eq24_e2165_d_n13, eq24_e2165_d_n14, eq24_e2165_d_n15, eq24_e2165_d_n16];
        let eq24_branch_derivatives: [f64; 0] = [];
        stamper.stamp_current_dense(
            Some(nodes[5]),
            Some(nodes[6]),
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
        let (eq25_e2176, eq25_e2176_d_n0, eq25_e2176_d_n1, eq25_e2176_d_n2, eq25_e2176_d_n3, eq25_e2176_d_n4, eq25_e2176_d_n5, eq25_e2176_d_n6, eq25_e2176_d_n7, eq25_e2176_d_n8, eq25_e2176_d_n9, eq25_e2176_d_n10, eq25_e2176_d_n11, eq25_e2176_d_n12, eq25_e2176_d_n13, eq25_e2176_d_n14, eq25_e2176_d_n15, eq25_e2176_d_n16,) = {
    if (((!(s.v[1698] != 0.0)) && (s.v[1701] != 0.0)) && (s.v[1702] != 0.0)) {
        let eq25_e2174: f64 = (s.v[114] * s.v[478]);
        let eq25_e2174_d_n0: f64 = ((s.dn[114][0] * s.v[478]) + (s.v[114] * s.dn[478][0]));
        let eq25_e2174_d_n1: f64 = ((s.dn[114][1] * s.v[478]) + (s.v[114] * s.dn[478][1]));
        let eq25_e2174_d_n2: f64 = ((s.dn[114][2] * s.v[478]) + (s.v[114] * s.dn[478][2]));
        let eq25_e2174_d_n3: f64 = ((s.dn[114][3] * s.v[478]) + (s.v[114] * s.dn[478][3]));
        let eq25_e2174_d_n4: f64 = ((s.dn[114][4] * s.v[478]) + (s.v[114] * s.dn[478][4]));
        let eq25_e2174_d_n5: f64 = ((s.dn[114][5] * s.v[478]) + (s.v[114] * s.dn[478][5]));
        let eq25_e2174_d_n6: f64 = ((s.dn[114][6] * s.v[478]) + (s.v[114] * s.dn[478][6]));
        let eq25_e2174_d_n7: f64 = ((s.dn[114][7] * s.v[478]) + (s.v[114] * s.dn[478][7]));
        let eq25_e2174_d_n8: f64 = ((s.dn[114][8] * s.v[478]) + (s.v[114] * s.dn[478][8]));
        let eq25_e2174_d_n9: f64 = ((s.dn[114][9] * s.v[478]) + (s.v[114] * s.dn[478][9]));
        let eq25_e2174_d_n10: f64 = ((s.dn[114][10] * s.v[478]) + (s.v[114] * s.dn[478][10]));
        let eq25_e2174_d_n11: f64 = ((s.dn[114][11] * s.v[478]) + (s.v[114] * s.dn[478][11]));
        let eq25_e2174_d_n12: f64 = ((s.dn[114][12] * s.v[478]) + (s.v[114] * s.dn[478][12]));
        let eq25_e2174_d_n13: f64 = ((s.dn[114][13] * s.v[478]) + (s.v[114] * s.dn[478][13]));
        let eq25_e2174_d_n14: f64 = ((s.dn[114][14] * s.v[478]) + (s.v[114] * s.dn[478][14]));
        let eq25_e2174_d_n15: f64 = ((s.dn[114][15] * s.v[478]) + (s.v[114] * s.dn[478][15]));
        let eq25_e2174_d_n16: f64 = ((s.dn[114][16] * s.v[478]) + (s.v[114] * s.dn[478][16]));
        (eq25_e2174, eq25_e2174_d_n0, eq25_e2174_d_n1, eq25_e2174_d_n2, eq25_e2174_d_n3, eq25_e2174_d_n4, eq25_e2174_d_n5, eq25_e2174_d_n6, eq25_e2174_d_n7, eq25_e2174_d_n8, eq25_e2174_d_n9, eq25_e2174_d_n10, eq25_e2174_d_n11, eq25_e2174_d_n12, eq25_e2174_d_n13, eq25_e2174_d_n14, eq25_e2174_d_n15, eq25_e2174_d_n16,)
    } else {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    }
};
        let eq25_value: f64 = eq25_e2176;
        let eq25_node_derivatives: [f64; 17] = [eq25_e2176_d_n0, eq25_e2176_d_n1, eq25_e2176_d_n2, eq25_e2176_d_n3, eq25_e2176_d_n4, eq25_e2176_d_n5, eq25_e2176_d_n6, eq25_e2176_d_n7, eq25_e2176_d_n8, eq25_e2176_d_n9, eq25_e2176_d_n10, eq25_e2176_d_n11, eq25_e2176_d_n12, eq25_e2176_d_n13, eq25_e2176_d_n14, eq25_e2176_d_n15, eq25_e2176_d_n16];
        let eq25_branch_derivatives: [f64; 0] = [];
        stamper.stamp_current_dense(
            Some(nodes[6]),
            Some(nodes[3]),
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
        let (eq26_e2187, eq26_e2187_d_n0, eq26_e2187_d_n1, eq26_e2187_d_n2, eq26_e2187_d_n3, eq26_e2187_d_n4, eq26_e2187_d_n5, eq26_e2187_d_n6, eq26_e2187_d_n7, eq26_e2187_d_n8, eq26_e2187_d_n9, eq26_e2187_d_n10, eq26_e2187_d_n11, eq26_e2187_d_n12, eq26_e2187_d_n13, eq26_e2187_d_n14, eq26_e2187_d_n15, eq26_e2187_d_n16,) = {
    if (((!(s.v[1698] != 0.0)) && (s.v[1701] != 0.0)) && (s.v[1702] != 0.0)) {
        let eq26_e2185: f64 = (s.v[114] * s.v[477]);
        let eq26_e2185_d_n0: f64 = ((s.dn[114][0] * s.v[477]) + (s.v[114] * s.dn[477][0]));
        let eq26_e2185_d_n1: f64 = ((s.dn[114][1] * s.v[477]) + (s.v[114] * s.dn[477][1]));
        let eq26_e2185_d_n2: f64 = ((s.dn[114][2] * s.v[477]) + (s.v[114] * s.dn[477][2]));
        let eq26_e2185_d_n3: f64 = ((s.dn[114][3] * s.v[477]) + (s.v[114] * s.dn[477][3]));
        let eq26_e2185_d_n4: f64 = ((s.dn[114][4] * s.v[477]) + (s.v[114] * s.dn[477][4]));
        let eq26_e2185_d_n5: f64 = ((s.dn[114][5] * s.v[477]) + (s.v[114] * s.dn[477][5]));
        let eq26_e2185_d_n6: f64 = ((s.dn[114][6] * s.v[477]) + (s.v[114] * s.dn[477][6]));
        let eq26_e2185_d_n7: f64 = ((s.dn[114][7] * s.v[477]) + (s.v[114] * s.dn[477][7]));
        let eq26_e2185_d_n8: f64 = ((s.dn[114][8] * s.v[477]) + (s.v[114] * s.dn[477][8]));
        let eq26_e2185_d_n9: f64 = ((s.dn[114][9] * s.v[477]) + (s.v[114] * s.dn[477][9]));
        let eq26_e2185_d_n10: f64 = ((s.dn[114][10] * s.v[477]) + (s.v[114] * s.dn[477][10]));
        let eq26_e2185_d_n11: f64 = ((s.dn[114][11] * s.v[477]) + (s.v[114] * s.dn[477][11]));
        let eq26_e2185_d_n12: f64 = ((s.dn[114][12] * s.v[477]) + (s.v[114] * s.dn[477][12]));
        let eq26_e2185_d_n13: f64 = ((s.dn[114][13] * s.v[477]) + (s.v[114] * s.dn[477][13]));
        let eq26_e2185_d_n14: f64 = ((s.dn[114][14] * s.v[477]) + (s.v[114] * s.dn[477][14]));
        let eq26_e2185_d_n15: f64 = ((s.dn[114][15] * s.v[477]) + (s.v[114] * s.dn[477][15]));
        let eq26_e2185_d_n16: f64 = ((s.dn[114][16] * s.v[477]) + (s.v[114] * s.dn[477][16]));
        (eq26_e2185, eq26_e2185_d_n0, eq26_e2185_d_n1, eq26_e2185_d_n2, eq26_e2185_d_n3, eq26_e2185_d_n4, eq26_e2185_d_n5, eq26_e2185_d_n6, eq26_e2185_d_n7, eq26_e2185_d_n8, eq26_e2185_d_n9, eq26_e2185_d_n10, eq26_e2185_d_n11, eq26_e2185_d_n12, eq26_e2185_d_n13, eq26_e2185_d_n14, eq26_e2185_d_n15, eq26_e2185_d_n16,)
    } else {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    }
};
        let eq26_value: f64 = eq26_e2187;
        let eq26_node_derivatives: [f64; 17] = [eq26_e2187_d_n0, eq26_e2187_d_n1, eq26_e2187_d_n2, eq26_e2187_d_n3, eq26_e2187_d_n4, eq26_e2187_d_n5, eq26_e2187_d_n6, eq26_e2187_d_n7, eq26_e2187_d_n8, eq26_e2187_d_n9, eq26_e2187_d_n10, eq26_e2187_d_n11, eq26_e2187_d_n12, eq26_e2187_d_n13, eq26_e2187_d_n14, eq26_e2187_d_n15, eq26_e2187_d_n16];
        let eq26_branch_derivatives: [f64; 0] = [];
        stamper.stamp_current_dense(
            Some(nodes[5]),
            Some(nodes[3]),
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
        let (eq27_e2201, eq27_e2201_d_n0, eq27_e2201_d_n1, eq27_e2201_d_n2, eq27_e2201_d_n3, eq27_e2201_d_n4, eq27_e2201_d_n5, eq27_e2201_d_n6, eq27_e2201_d_n7, eq27_e2201_d_n8, eq27_e2201_d_n9, eq27_e2201_d_n10, eq27_e2201_d_n11, eq27_e2201_d_n12, eq27_e2201_d_n13, eq27_e2201_d_n14, eq27_e2201_d_n15, eq27_e2201_d_n16,) = {
    if (((!(s.v[1698] != 0.0)) && (s.v[1701] != 0.0)) && (!(s.v[1702] != 0.0))) {
        let eq27_e2198: f64 = (s.v[476] + s.v[488]);
        let eq27_e2198_d_n0: f64 = (s.dn[476][0] + s.dn[488][0]);
        let eq27_e2198_d_n1: f64 = (s.dn[476][1] + s.dn[488][1]);
        let eq27_e2198_d_n2: f64 = (s.dn[476][2] + s.dn[488][2]);
        let eq27_e2198_d_n3: f64 = (s.dn[476][3] + s.dn[488][3]);
        let eq27_e2198_d_n4: f64 = (s.dn[476][4] + s.dn[488][4]);
        let eq27_e2198_d_n5: f64 = (s.dn[476][5] + s.dn[488][5]);
        let eq27_e2198_d_n6: f64 = (s.dn[476][6] + s.dn[488][6]);
        let eq27_e2198_d_n7: f64 = (s.dn[476][7] + s.dn[488][7]);
        let eq27_e2198_d_n8: f64 = (s.dn[476][8] + s.dn[488][8]);
        let eq27_e2198_d_n9: f64 = (s.dn[476][9] + s.dn[488][9]);
        let eq27_e2198_d_n10: f64 = (s.dn[476][10] + s.dn[488][10]);
        let eq27_e2198_d_n11: f64 = (s.dn[476][11] + s.dn[488][11]);
        let eq27_e2198_d_n12: f64 = (s.dn[476][12] + s.dn[488][12]);
        let eq27_e2198_d_n13: f64 = (s.dn[476][13] + s.dn[488][13]);
        let eq27_e2198_d_n14: f64 = (s.dn[476][14] + s.dn[488][14]);
        let eq27_e2198_d_n15: f64 = (s.dn[476][15] + s.dn[488][15]);
        let eq27_e2198_d_n16: f64 = (s.dn[476][16] + s.dn[488][16]);
        let eq27_e2199: f64 = (s.v[114] * eq27_e2198);
        let eq27_e2199_d_n0: f64 = ((s.dn[114][0] * eq27_e2198) + (s.v[114] * eq27_e2198_d_n0));
        let eq27_e2199_d_n1: f64 = ((s.dn[114][1] * eq27_e2198) + (s.v[114] * eq27_e2198_d_n1));
        let eq27_e2199_d_n2: f64 = ((s.dn[114][2] * eq27_e2198) + (s.v[114] * eq27_e2198_d_n2));
        let eq27_e2199_d_n3: f64 = ((s.dn[114][3] * eq27_e2198) + (s.v[114] * eq27_e2198_d_n3));
        let eq27_e2199_d_n4: f64 = ((s.dn[114][4] * eq27_e2198) + (s.v[114] * eq27_e2198_d_n4));
        let eq27_e2199_d_n5: f64 = ((s.dn[114][5] * eq27_e2198) + (s.v[114] * eq27_e2198_d_n5));
        let eq27_e2199_d_n6: f64 = ((s.dn[114][6] * eq27_e2198) + (s.v[114] * eq27_e2198_d_n6));
        let eq27_e2199_d_n7: f64 = ((s.dn[114][7] * eq27_e2198) + (s.v[114] * eq27_e2198_d_n7));
        let eq27_e2199_d_n8: f64 = ((s.dn[114][8] * eq27_e2198) + (s.v[114] * eq27_e2198_d_n8));
        let eq27_e2199_d_n9: f64 = ((s.dn[114][9] * eq27_e2198) + (s.v[114] * eq27_e2198_d_n9));
        let eq27_e2199_d_n10: f64 = ((s.dn[114][10] * eq27_e2198) + (s.v[114] * eq27_e2198_d_n10));
        let eq27_e2199_d_n11: f64 = ((s.dn[114][11] * eq27_e2198) + (s.v[114] * eq27_e2198_d_n11));
        let eq27_e2199_d_n12: f64 = ((s.dn[114][12] * eq27_e2198) + (s.v[114] * eq27_e2198_d_n12));
        let eq27_e2199_d_n13: f64 = ((s.dn[114][13] * eq27_e2198) + (s.v[114] * eq27_e2198_d_n13));
        let eq27_e2199_d_n14: f64 = ((s.dn[114][14] * eq27_e2198) + (s.v[114] * eq27_e2198_d_n14));
        let eq27_e2199_d_n15: f64 = ((s.dn[114][15] * eq27_e2198) + (s.v[114] * eq27_e2198_d_n15));
        let eq27_e2199_d_n16: f64 = ((s.dn[114][16] * eq27_e2198) + (s.v[114] * eq27_e2198_d_n16));
        (eq27_e2199, eq27_e2199_d_n0, eq27_e2199_d_n1, eq27_e2199_d_n2, eq27_e2199_d_n3, eq27_e2199_d_n4, eq27_e2199_d_n5, eq27_e2199_d_n6, eq27_e2199_d_n7, eq27_e2199_d_n8, eq27_e2199_d_n9, eq27_e2199_d_n10, eq27_e2199_d_n11, eq27_e2199_d_n12, eq27_e2199_d_n13, eq27_e2199_d_n14, eq27_e2199_d_n15, eq27_e2199_d_n16,)
    } else {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    }
};
        let eq27_value: f64 = eq27_e2201;
        let eq27_node_derivatives: [f64; 17] = [eq27_e2201_d_n0, eq27_e2201_d_n1, eq27_e2201_d_n2, eq27_e2201_d_n3, eq27_e2201_d_n4, eq27_e2201_d_n5, eq27_e2201_d_n6, eq27_e2201_d_n7, eq27_e2201_d_n8, eq27_e2201_d_n9, eq27_e2201_d_n10, eq27_e2201_d_n11, eq27_e2201_d_n12, eq27_e2201_d_n13, eq27_e2201_d_n14, eq27_e2201_d_n15, eq27_e2201_d_n16];
        let eq27_branch_derivatives: [f64; 0] = [];
        stamper.stamp_current_dense(
            Some(nodes[6]),
            Some(nodes[3]),
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
        let (eq28_e2213, eq28_e2213_d_n0, eq28_e2213_d_n1, eq28_e2213_d_n2, eq28_e2213_d_n3, eq28_e2213_d_n4, eq28_e2213_d_n5, eq28_e2213_d_n6, eq28_e2213_d_n7, eq28_e2213_d_n8, eq28_e2213_d_n9, eq28_e2213_d_n10, eq28_e2213_d_n11, eq28_e2213_d_n12, eq28_e2213_d_n13, eq28_e2213_d_n14, eq28_e2213_d_n15, eq28_e2213_d_n16,) = {
    if (((!(s.v[1698] != 0.0)) && (s.v[1701] != 0.0)) && (!(s.v[1702] != 0.0))) {
        let eq28_e2211: f64 = (s.v[114] * s.v[475]);
        let eq28_e2211_d_n0: f64 = ((s.dn[114][0] * s.v[475]) + (s.v[114] * s.dn[475][0]));
        let eq28_e2211_d_n1: f64 = ((s.dn[114][1] * s.v[475]) + (s.v[114] * s.dn[475][1]));
        let eq28_e2211_d_n2: f64 = ((s.dn[114][2] * s.v[475]) + (s.v[114] * s.dn[475][2]));
        let eq28_e2211_d_n3: f64 = ((s.dn[114][3] * s.v[475]) + (s.v[114] * s.dn[475][3]));
        let eq28_e2211_d_n4: f64 = ((s.dn[114][4] * s.v[475]) + (s.v[114] * s.dn[475][4]));
        let eq28_e2211_d_n5: f64 = ((s.dn[114][5] * s.v[475]) + (s.v[114] * s.dn[475][5]));
        let eq28_e2211_d_n6: f64 = ((s.dn[114][6] * s.v[475]) + (s.v[114] * s.dn[475][6]));
        let eq28_e2211_d_n7: f64 = ((s.dn[114][7] * s.v[475]) + (s.v[114] * s.dn[475][7]));
        let eq28_e2211_d_n8: f64 = ((s.dn[114][8] * s.v[475]) + (s.v[114] * s.dn[475][8]));
        let eq28_e2211_d_n9: f64 = ((s.dn[114][9] * s.v[475]) + (s.v[114] * s.dn[475][9]));
        let eq28_e2211_d_n10: f64 = ((s.dn[114][10] * s.v[475]) + (s.v[114] * s.dn[475][10]));
        let eq28_e2211_d_n11: f64 = ((s.dn[114][11] * s.v[475]) + (s.v[114] * s.dn[475][11]));
        let eq28_e2211_d_n12: f64 = ((s.dn[114][12] * s.v[475]) + (s.v[114] * s.dn[475][12]));
        let eq28_e2211_d_n13: f64 = ((s.dn[114][13] * s.v[475]) + (s.v[114] * s.dn[475][13]));
        let eq28_e2211_d_n14: f64 = ((s.dn[114][14] * s.v[475]) + (s.v[114] * s.dn[475][14]));
        let eq28_e2211_d_n15: f64 = ((s.dn[114][15] * s.v[475]) + (s.v[114] * s.dn[475][15]));
        let eq28_e2211_d_n16: f64 = ((s.dn[114][16] * s.v[475]) + (s.v[114] * s.dn[475][16]));
        (eq28_e2211, eq28_e2211_d_n0, eq28_e2211_d_n1, eq28_e2211_d_n2, eq28_e2211_d_n3, eq28_e2211_d_n4, eq28_e2211_d_n5, eq28_e2211_d_n6, eq28_e2211_d_n7, eq28_e2211_d_n8, eq28_e2211_d_n9, eq28_e2211_d_n10, eq28_e2211_d_n11, eq28_e2211_d_n12, eq28_e2211_d_n13, eq28_e2211_d_n14, eq28_e2211_d_n15, eq28_e2211_d_n16,)
    } else {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    }
};
        let eq28_value: f64 = eq28_e2213;
        let eq28_node_derivatives: [f64; 17] = [eq28_e2213_d_n0, eq28_e2213_d_n1, eq28_e2213_d_n2, eq28_e2213_d_n3, eq28_e2213_d_n4, eq28_e2213_d_n5, eq28_e2213_d_n6, eq28_e2213_d_n7, eq28_e2213_d_n8, eq28_e2213_d_n9, eq28_e2213_d_n10, eq28_e2213_d_n11, eq28_e2213_d_n12, eq28_e2213_d_n13, eq28_e2213_d_n14, eq28_e2213_d_n15, eq28_e2213_d_n16];
        let eq28_branch_derivatives: [f64; 0] = [];
        stamper.stamp_current_dense(
            Some(nodes[5]),
            Some(nodes[3]),
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
        let (eq29_e2224, eq29_e2224_d_n0, eq29_e2224_d_n1, eq29_e2224_d_n2, eq29_e2224_d_n3, eq29_e2224_d_n4, eq29_e2224_d_n5, eq29_e2224_d_n6, eq29_e2224_d_n7, eq29_e2224_d_n8, eq29_e2224_d_n9, eq29_e2224_d_n10, eq29_e2224_d_n11, eq29_e2224_d_n12, eq29_e2224_d_n13, eq29_e2224_d_n14, eq29_e2224_d_n15, eq29_e2224_d_n16,) = {
    if ((!(s.v[1698] != 0.0)) && (s.v[1701] != 0.0)) {
        let eq29_e2221: f64 = (s.v[461] + s.v[469]);
        let eq29_e2221_d_n0: f64 = (s.dn[461][0] + s.dn[469][0]);
        let eq29_e2221_d_n1: f64 = (s.dn[461][1] + s.dn[469][1]);
        let eq29_e2221_d_n2: f64 = (s.dn[461][2] + s.dn[469][2]);
        let eq29_e2221_d_n3: f64 = (s.dn[461][3] + s.dn[469][3]);
        let eq29_e2221_d_n4: f64 = (s.dn[461][4] + s.dn[469][4]);
        let eq29_e2221_d_n5: f64 = (s.dn[461][5] + s.dn[469][5]);
        let eq29_e2221_d_n6: f64 = (s.dn[461][6] + s.dn[469][6]);
        let eq29_e2221_d_n7: f64 = (s.dn[461][7] + s.dn[469][7]);
        let eq29_e2221_d_n8: f64 = (s.dn[461][8] + s.dn[469][8]);
        let eq29_e2221_d_n9: f64 = (s.dn[461][9] + s.dn[469][9]);
        let eq29_e2221_d_n10: f64 = (s.dn[461][10] + s.dn[469][10]);
        let eq29_e2221_d_n11: f64 = (s.dn[461][11] + s.dn[469][11]);
        let eq29_e2221_d_n12: f64 = (s.dn[461][12] + s.dn[469][12]);
        let eq29_e2221_d_n13: f64 = (s.dn[461][13] + s.dn[469][13]);
        let eq29_e2221_d_n14: f64 = (s.dn[461][14] + s.dn[469][14]);
        let eq29_e2221_d_n15: f64 = (s.dn[461][15] + s.dn[469][15]);
        let eq29_e2221_d_n16: f64 = (s.dn[461][16] + s.dn[469][16]);
        let eq29_e2222: f64 = (s.v[114] * eq29_e2221);
        let eq29_e2222_d_n0: f64 = ((s.dn[114][0] * eq29_e2221) + (s.v[114] * eq29_e2221_d_n0));
        let eq29_e2222_d_n1: f64 = ((s.dn[114][1] * eq29_e2221) + (s.v[114] * eq29_e2221_d_n1));
        let eq29_e2222_d_n2: f64 = ((s.dn[114][2] * eq29_e2221) + (s.v[114] * eq29_e2221_d_n2));
        let eq29_e2222_d_n3: f64 = ((s.dn[114][3] * eq29_e2221) + (s.v[114] * eq29_e2221_d_n3));
        let eq29_e2222_d_n4: f64 = ((s.dn[114][4] * eq29_e2221) + (s.v[114] * eq29_e2221_d_n4));
        let eq29_e2222_d_n5: f64 = ((s.dn[114][5] * eq29_e2221) + (s.v[114] * eq29_e2221_d_n5));
        let eq29_e2222_d_n6: f64 = ((s.dn[114][6] * eq29_e2221) + (s.v[114] * eq29_e2221_d_n6));
        let eq29_e2222_d_n7: f64 = ((s.dn[114][7] * eq29_e2221) + (s.v[114] * eq29_e2221_d_n7));
        let eq29_e2222_d_n8: f64 = ((s.dn[114][8] * eq29_e2221) + (s.v[114] * eq29_e2221_d_n8));
        let eq29_e2222_d_n9: f64 = ((s.dn[114][9] * eq29_e2221) + (s.v[114] * eq29_e2221_d_n9));
        let eq29_e2222_d_n10: f64 = ((s.dn[114][10] * eq29_e2221) + (s.v[114] * eq29_e2221_d_n10));
        let eq29_e2222_d_n11: f64 = ((s.dn[114][11] * eq29_e2221) + (s.v[114] * eq29_e2221_d_n11));
        let eq29_e2222_d_n12: f64 = ((s.dn[114][12] * eq29_e2221) + (s.v[114] * eq29_e2221_d_n12));
        let eq29_e2222_d_n13: f64 = ((s.dn[114][13] * eq29_e2221) + (s.v[114] * eq29_e2221_d_n13));
        let eq29_e2222_d_n14: f64 = ((s.dn[114][14] * eq29_e2221) + (s.v[114] * eq29_e2221_d_n14));
        let eq29_e2222_d_n15: f64 = ((s.dn[114][15] * eq29_e2221) + (s.v[114] * eq29_e2221_d_n15));
        let eq29_e2222_d_n16: f64 = ((s.dn[114][16] * eq29_e2221) + (s.v[114] * eq29_e2221_d_n16));
        (eq29_e2222, eq29_e2222_d_n0, eq29_e2222_d_n1, eq29_e2222_d_n2, eq29_e2222_d_n3, eq29_e2222_d_n4, eq29_e2222_d_n5, eq29_e2222_d_n6, eq29_e2222_d_n7, eq29_e2222_d_n8, eq29_e2222_d_n9, eq29_e2222_d_n10, eq29_e2222_d_n11, eq29_e2222_d_n12, eq29_e2222_d_n13, eq29_e2222_d_n14, eq29_e2222_d_n15, eq29_e2222_d_n16,)
    } else {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    }
};
        let eq29_value: f64 = eq29_e2224;
        let eq29_node_derivatives: [f64; 17] = [eq29_e2224_d_n0, eq29_e2224_d_n1, eq29_e2224_d_n2, eq29_e2224_d_n3, eq29_e2224_d_n4, eq29_e2224_d_n5, eq29_e2224_d_n6, eq29_e2224_d_n7, eq29_e2224_d_n8, eq29_e2224_d_n9, eq29_e2224_d_n10, eq29_e2224_d_n11, eq29_e2224_d_n12, eq29_e2224_d_n13, eq29_e2224_d_n14, eq29_e2224_d_n15, eq29_e2224_d_n16];
        let eq29_branch_derivatives: [f64; 0] = [];
        stamper.stamp_current_dense(
            Some(nodes[11]),
            Some(nodes[3]),
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
        let (eq30_e2236, eq30_e2236_d_n0, eq30_e2236_d_n1, eq30_e2236_d_n2, eq30_e2236_d_n3, eq30_e2236_d_n4, eq30_e2236_d_n5, eq30_e2236_d_n6, eq30_e2236_d_n7, eq30_e2236_d_n8, eq30_e2236_d_n9, eq30_e2236_d_n10, eq30_e2236_d_n11, eq30_e2236_d_n12, eq30_e2236_d_n13, eq30_e2236_d_n14, eq30_e2236_d_n15, eq30_e2236_d_n16,) = {
    if ((!(s.v[1698] != 0.0)) && (!(s.v[1701] != 0.0))) {
        let eq30_e2233: f64 = (s.v[476] + s.v[488]);
        let eq30_e2233_d_n0: f64 = (s.dn[476][0] + s.dn[488][0]);
        let eq30_e2233_d_n1: f64 = (s.dn[476][1] + s.dn[488][1]);
        let eq30_e2233_d_n2: f64 = (s.dn[476][2] + s.dn[488][2]);
        let eq30_e2233_d_n3: f64 = (s.dn[476][3] + s.dn[488][3]);
        let eq30_e2233_d_n4: f64 = (s.dn[476][4] + s.dn[488][4]);
        let eq30_e2233_d_n5: f64 = (s.dn[476][5] + s.dn[488][5]);
        let eq30_e2233_d_n6: f64 = (s.dn[476][6] + s.dn[488][6]);
        let eq30_e2233_d_n7: f64 = (s.dn[476][7] + s.dn[488][7]);
        let eq30_e2233_d_n8: f64 = (s.dn[476][8] + s.dn[488][8]);
        let eq30_e2233_d_n9: f64 = (s.dn[476][9] + s.dn[488][9]);
        let eq30_e2233_d_n10: f64 = (s.dn[476][10] + s.dn[488][10]);
        let eq30_e2233_d_n11: f64 = (s.dn[476][11] + s.dn[488][11]);
        let eq30_e2233_d_n12: f64 = (s.dn[476][12] + s.dn[488][12]);
        let eq30_e2233_d_n13: f64 = (s.dn[476][13] + s.dn[488][13]);
        let eq30_e2233_d_n14: f64 = (s.dn[476][14] + s.dn[488][14]);
        let eq30_e2233_d_n15: f64 = (s.dn[476][15] + s.dn[488][15]);
        let eq30_e2233_d_n16: f64 = (s.dn[476][16] + s.dn[488][16]);
        let eq30_e2234: f64 = (s.v[114] * eq30_e2233);
        let eq30_e2234_d_n0: f64 = ((s.dn[114][0] * eq30_e2233) + (s.v[114] * eq30_e2233_d_n0));
        let eq30_e2234_d_n1: f64 = ((s.dn[114][1] * eq30_e2233) + (s.v[114] * eq30_e2233_d_n1));
        let eq30_e2234_d_n2: f64 = ((s.dn[114][2] * eq30_e2233) + (s.v[114] * eq30_e2233_d_n2));
        let eq30_e2234_d_n3: f64 = ((s.dn[114][3] * eq30_e2233) + (s.v[114] * eq30_e2233_d_n3));
        let eq30_e2234_d_n4: f64 = ((s.dn[114][4] * eq30_e2233) + (s.v[114] * eq30_e2233_d_n4));
        let eq30_e2234_d_n5: f64 = ((s.dn[114][5] * eq30_e2233) + (s.v[114] * eq30_e2233_d_n5));
        let eq30_e2234_d_n6: f64 = ((s.dn[114][6] * eq30_e2233) + (s.v[114] * eq30_e2233_d_n6));
        let eq30_e2234_d_n7: f64 = ((s.dn[114][7] * eq30_e2233) + (s.v[114] * eq30_e2233_d_n7));
        let eq30_e2234_d_n8: f64 = ((s.dn[114][8] * eq30_e2233) + (s.v[114] * eq30_e2233_d_n8));
        let eq30_e2234_d_n9: f64 = ((s.dn[114][9] * eq30_e2233) + (s.v[114] * eq30_e2233_d_n9));
        let eq30_e2234_d_n10: f64 = ((s.dn[114][10] * eq30_e2233) + (s.v[114] * eq30_e2233_d_n10));
        let eq30_e2234_d_n11: f64 = ((s.dn[114][11] * eq30_e2233) + (s.v[114] * eq30_e2233_d_n11));
        let eq30_e2234_d_n12: f64 = ((s.dn[114][12] * eq30_e2233) + (s.v[114] * eq30_e2233_d_n12));
        let eq30_e2234_d_n13: f64 = ((s.dn[114][13] * eq30_e2233) + (s.v[114] * eq30_e2233_d_n13));
        let eq30_e2234_d_n14: f64 = ((s.dn[114][14] * eq30_e2233) + (s.v[114] * eq30_e2233_d_n14));
        let eq30_e2234_d_n15: f64 = ((s.dn[114][15] * eq30_e2233) + (s.v[114] * eq30_e2233_d_n15));
        let eq30_e2234_d_n16: f64 = ((s.dn[114][16] * eq30_e2233) + (s.v[114] * eq30_e2233_d_n16));
        (eq30_e2234, eq30_e2234_d_n0, eq30_e2234_d_n1, eq30_e2234_d_n2, eq30_e2234_d_n3, eq30_e2234_d_n4, eq30_e2234_d_n5, eq30_e2234_d_n6, eq30_e2234_d_n7, eq30_e2234_d_n8, eq30_e2234_d_n9, eq30_e2234_d_n10, eq30_e2234_d_n11, eq30_e2234_d_n12, eq30_e2234_d_n13, eq30_e2234_d_n14, eq30_e2234_d_n15, eq30_e2234_d_n16,)
    } else {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    }
};
        let eq30_value: f64 = eq30_e2236;
        let eq30_node_derivatives: [f64; 17] = [eq30_e2236_d_n0, eq30_e2236_d_n1, eq30_e2236_d_n2, eq30_e2236_d_n3, eq30_e2236_d_n4, eq30_e2236_d_n5, eq30_e2236_d_n6, eq30_e2236_d_n7, eq30_e2236_d_n8, eq30_e2236_d_n9, eq30_e2236_d_n10, eq30_e2236_d_n11, eq30_e2236_d_n12, eq30_e2236_d_n13, eq30_e2236_d_n14, eq30_e2236_d_n15, eq30_e2236_d_n16];
        let eq30_branch_derivatives: [f64; 0] = [];
        stamper.stamp_current_dense(
            Some(nodes[6]),
            Some(nodes[5]),
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
        let (eq31_e2246, eq31_e2246_d_n0, eq31_e2246_d_n1, eq31_e2246_d_n2, eq31_e2246_d_n3, eq31_e2246_d_n4, eq31_e2246_d_n5, eq31_e2246_d_n6, eq31_e2246_d_n7, eq31_e2246_d_n8, eq31_e2246_d_n9, eq31_e2246_d_n10, eq31_e2246_d_n11, eq31_e2246_d_n12, eq31_e2246_d_n13, eq31_e2246_d_n14, eq31_e2246_d_n15, eq31_e2246_d_n16,) = {
    if ((!(s.v[1698] != 0.0)) && (!(s.v[1701] != 0.0))) {
        let eq31_e2244: f64 = (s.v[114] * s.v[475]);
        let eq31_e2244_d_n0: f64 = ((s.dn[114][0] * s.v[475]) + (s.v[114] * s.dn[475][0]));
        let eq31_e2244_d_n1: f64 = ((s.dn[114][1] * s.v[475]) + (s.v[114] * s.dn[475][1]));
        let eq31_e2244_d_n2: f64 = ((s.dn[114][2] * s.v[475]) + (s.v[114] * s.dn[475][2]));
        let eq31_e2244_d_n3: f64 = ((s.dn[114][3] * s.v[475]) + (s.v[114] * s.dn[475][3]));
        let eq31_e2244_d_n4: f64 = ((s.dn[114][4] * s.v[475]) + (s.v[114] * s.dn[475][4]));
        let eq31_e2244_d_n5: f64 = ((s.dn[114][5] * s.v[475]) + (s.v[114] * s.dn[475][5]));
        let eq31_e2244_d_n6: f64 = ((s.dn[114][6] * s.v[475]) + (s.v[114] * s.dn[475][6]));
        let eq31_e2244_d_n7: f64 = ((s.dn[114][7] * s.v[475]) + (s.v[114] * s.dn[475][7]));
        let eq31_e2244_d_n8: f64 = ((s.dn[114][8] * s.v[475]) + (s.v[114] * s.dn[475][8]));
        let eq31_e2244_d_n9: f64 = ((s.dn[114][9] * s.v[475]) + (s.v[114] * s.dn[475][9]));
        let eq31_e2244_d_n10: f64 = ((s.dn[114][10] * s.v[475]) + (s.v[114] * s.dn[475][10]));
        let eq31_e2244_d_n11: f64 = ((s.dn[114][11] * s.v[475]) + (s.v[114] * s.dn[475][11]));
        let eq31_e2244_d_n12: f64 = ((s.dn[114][12] * s.v[475]) + (s.v[114] * s.dn[475][12]));
        let eq31_e2244_d_n13: f64 = ((s.dn[114][13] * s.v[475]) + (s.v[114] * s.dn[475][13]));
        let eq31_e2244_d_n14: f64 = ((s.dn[114][14] * s.v[475]) + (s.v[114] * s.dn[475][14]));
        let eq31_e2244_d_n15: f64 = ((s.dn[114][15] * s.v[475]) + (s.v[114] * s.dn[475][15]));
        let eq31_e2244_d_n16: f64 = ((s.dn[114][16] * s.v[475]) + (s.v[114] * s.dn[475][16]));
        (eq31_e2244, eq31_e2244_d_n0, eq31_e2244_d_n1, eq31_e2244_d_n2, eq31_e2244_d_n3, eq31_e2244_d_n4, eq31_e2244_d_n5, eq31_e2244_d_n6, eq31_e2244_d_n7, eq31_e2244_d_n8, eq31_e2244_d_n9, eq31_e2244_d_n10, eq31_e2244_d_n11, eq31_e2244_d_n12, eq31_e2244_d_n13, eq31_e2244_d_n14, eq31_e2244_d_n15, eq31_e2244_d_n16,)
    } else {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    }
};
        let eq31_value: f64 = eq31_e2246;
        let eq31_node_derivatives: [f64; 17] = [eq31_e2246_d_n0, eq31_e2246_d_n1, eq31_e2246_d_n2, eq31_e2246_d_n3, eq31_e2246_d_n4, eq31_e2246_d_n5, eq31_e2246_d_n6, eq31_e2246_d_n7, eq31_e2246_d_n8, eq31_e2246_d_n9, eq31_e2246_d_n10, eq31_e2246_d_n11, eq31_e2246_d_n12, eq31_e2246_d_n13, eq31_e2246_d_n14, eq31_e2246_d_n15, eq31_e2246_d_n16];
        let eq31_branch_derivatives: [f64; 0] = [];
        stamper.stamp_current_dense(
            Some(nodes[5]),
            Some(nodes[6]),
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
        let (eq32_e2252, eq32_e2252_d_n0, eq32_e2252_d_n1, eq32_e2252_d_n2, eq32_e2252_d_n3, eq32_e2252_d_n4, eq32_e2252_d_n5, eq32_e2252_d_n6, eq32_e2252_d_n7, eq32_e2252_d_n8, eq32_e2252_d_n9, eq32_e2252_d_n10, eq32_e2252_d_n11, eq32_e2252_d_n12, eq32_e2252_d_n13, eq32_e2252_d_n14, eq32_e2252_d_n15, eq32_e2252_d_n16,) = {
    if (s.v[1703] != 0.0) {
        let eq32_e2250: f64 = (s.v[114] * s.v[464]);
        let eq32_e2250_d_n0: f64 = ((s.dn[114][0] * s.v[464]) + (s.v[114] * s.dn[464][0]));
        let eq32_e2250_d_n1: f64 = ((s.dn[114][1] * s.v[464]) + (s.v[114] * s.dn[464][1]));
        let eq32_e2250_d_n2: f64 = ((s.dn[114][2] * s.v[464]) + (s.v[114] * s.dn[464][2]));
        let eq32_e2250_d_n3: f64 = ((s.dn[114][3] * s.v[464]) + (s.v[114] * s.dn[464][3]));
        let eq32_e2250_d_n4: f64 = ((s.dn[114][4] * s.v[464]) + (s.v[114] * s.dn[464][4]));
        let eq32_e2250_d_n5: f64 = ((s.dn[114][5] * s.v[464]) + (s.v[114] * s.dn[464][5]));
        let eq32_e2250_d_n6: f64 = ((s.dn[114][6] * s.v[464]) + (s.v[114] * s.dn[464][6]));
        let eq32_e2250_d_n7: f64 = ((s.dn[114][7] * s.v[464]) + (s.v[114] * s.dn[464][7]));
        let eq32_e2250_d_n8: f64 = ((s.dn[114][8] * s.v[464]) + (s.v[114] * s.dn[464][8]));
        let eq32_e2250_d_n9: f64 = ((s.dn[114][9] * s.v[464]) + (s.v[114] * s.dn[464][9]));
        let eq32_e2250_d_n10: f64 = ((s.dn[114][10] * s.v[464]) + (s.v[114] * s.dn[464][10]));
        let eq32_e2250_d_n11: f64 = ((s.dn[114][11] * s.v[464]) + (s.v[114] * s.dn[464][11]));
        let eq32_e2250_d_n12: f64 = ((s.dn[114][12] * s.v[464]) + (s.v[114] * s.dn[464][12]));
        let eq32_e2250_d_n13: f64 = ((s.dn[114][13] * s.v[464]) + (s.v[114] * s.dn[464][13]));
        let eq32_e2250_d_n14: f64 = ((s.dn[114][14] * s.v[464]) + (s.v[114] * s.dn[464][14]));
        let eq32_e2250_d_n15: f64 = ((s.dn[114][15] * s.v[464]) + (s.v[114] * s.dn[464][15]));
        let eq32_e2250_d_n16: f64 = ((s.dn[114][16] * s.v[464]) + (s.v[114] * s.dn[464][16]));
        (eq32_e2250, eq32_e2250_d_n0, eq32_e2250_d_n1, eq32_e2250_d_n2, eq32_e2250_d_n3, eq32_e2250_d_n4, eq32_e2250_d_n5, eq32_e2250_d_n6, eq32_e2250_d_n7, eq32_e2250_d_n8, eq32_e2250_d_n9, eq32_e2250_d_n10, eq32_e2250_d_n11, eq32_e2250_d_n12, eq32_e2250_d_n13, eq32_e2250_d_n14, eq32_e2250_d_n15, eq32_e2250_d_n16,)
    } else {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    }
};
        let eq32_value: f64 = eq32_e2252;
        let eq32_node_derivatives: [f64; 17] = [eq32_e2252_d_n0, eq32_e2252_d_n1, eq32_e2252_d_n2, eq32_e2252_d_n3, eq32_e2252_d_n4, eq32_e2252_d_n5, eq32_e2252_d_n6, eq32_e2252_d_n7, eq32_e2252_d_n8, eq32_e2252_d_n9, eq32_e2252_d_n10, eq32_e2252_d_n11, eq32_e2252_d_n12, eq32_e2252_d_n13, eq32_e2252_d_n14, eq32_e2252_d_n15, eq32_e2252_d_n16];
        let eq32_branch_derivatives: [f64; 0] = [];
        stamper.stamp_current_dense(
            Some(nodes[11]),
            Some(nodes[6]),
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
        let (eq33_e2258, eq33_e2258_d_n0, eq33_e2258_d_n1, eq33_e2258_d_n2, eq33_e2258_d_n3, eq33_e2258_d_n4, eq33_e2258_d_n5, eq33_e2258_d_n6, eq33_e2258_d_n7, eq33_e2258_d_n8, eq33_e2258_d_n9, eq33_e2258_d_n10, eq33_e2258_d_n11, eq33_e2258_d_n12, eq33_e2258_d_n13, eq33_e2258_d_n14, eq33_e2258_d_n15, eq33_e2258_d_n16,) = {
    if (s.v[1703] != 0.0) {
        let eq33_e2256: f64 = (s.v[114] * s.v[465]);
        let eq33_e2256_d_n0: f64 = ((s.dn[114][0] * s.v[465]) + (s.v[114] * s.dn[465][0]));
        let eq33_e2256_d_n1: f64 = ((s.dn[114][1] * s.v[465]) + (s.v[114] * s.dn[465][1]));
        let eq33_e2256_d_n2: f64 = ((s.dn[114][2] * s.v[465]) + (s.v[114] * s.dn[465][2]));
        let eq33_e2256_d_n3: f64 = ((s.dn[114][3] * s.v[465]) + (s.v[114] * s.dn[465][3]));
        let eq33_e2256_d_n4: f64 = ((s.dn[114][4] * s.v[465]) + (s.v[114] * s.dn[465][4]));
        let eq33_e2256_d_n5: f64 = ((s.dn[114][5] * s.v[465]) + (s.v[114] * s.dn[465][5]));
        let eq33_e2256_d_n6: f64 = ((s.dn[114][6] * s.v[465]) + (s.v[114] * s.dn[465][6]));
        let eq33_e2256_d_n7: f64 = ((s.dn[114][7] * s.v[465]) + (s.v[114] * s.dn[465][7]));
        let eq33_e2256_d_n8: f64 = ((s.dn[114][8] * s.v[465]) + (s.v[114] * s.dn[465][8]));
        let eq33_e2256_d_n9: f64 = ((s.dn[114][9] * s.v[465]) + (s.v[114] * s.dn[465][9]));
        let eq33_e2256_d_n10: f64 = ((s.dn[114][10] * s.v[465]) + (s.v[114] * s.dn[465][10]));
        let eq33_e2256_d_n11: f64 = ((s.dn[114][11] * s.v[465]) + (s.v[114] * s.dn[465][11]));
        let eq33_e2256_d_n12: f64 = ((s.dn[114][12] * s.v[465]) + (s.v[114] * s.dn[465][12]));
        let eq33_e2256_d_n13: f64 = ((s.dn[114][13] * s.v[465]) + (s.v[114] * s.dn[465][13]));
        let eq33_e2256_d_n14: f64 = ((s.dn[114][14] * s.v[465]) + (s.v[114] * s.dn[465][14]));
        let eq33_e2256_d_n15: f64 = ((s.dn[114][15] * s.v[465]) + (s.v[114] * s.dn[465][15]));
        let eq33_e2256_d_n16: f64 = ((s.dn[114][16] * s.v[465]) + (s.v[114] * s.dn[465][16]));
        (eq33_e2256, eq33_e2256_d_n0, eq33_e2256_d_n1, eq33_e2256_d_n2, eq33_e2256_d_n3, eq33_e2256_d_n4, eq33_e2256_d_n5, eq33_e2256_d_n6, eq33_e2256_d_n7, eq33_e2256_d_n8, eq33_e2256_d_n9, eq33_e2256_d_n10, eq33_e2256_d_n11, eq33_e2256_d_n12, eq33_e2256_d_n13, eq33_e2256_d_n14, eq33_e2256_d_n15, eq33_e2256_d_n16,)
    } else {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    }
};
        let eq33_value: f64 = eq33_e2258;
        let eq33_node_derivatives: [f64; 17] = [eq33_e2258_d_n0, eq33_e2258_d_n1, eq33_e2258_d_n2, eq33_e2258_d_n3, eq33_e2258_d_n4, eq33_e2258_d_n5, eq33_e2258_d_n6, eq33_e2258_d_n7, eq33_e2258_d_n8, eq33_e2258_d_n9, eq33_e2258_d_n10, eq33_e2258_d_n11, eq33_e2258_d_n12, eq33_e2258_d_n13, eq33_e2258_d_n14, eq33_e2258_d_n15, eq33_e2258_d_n16];
        let eq33_branch_derivatives: [f64; 0] = [];
        stamper.stamp_current_dense(
            Some(nodes[11]),
            Some(nodes[5]),
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
        let nv3 = ctx.node_voltage(nodes[3]);
        let nv6 = ctx.node_voltage(nodes[6]);
        let (eq34_e2268, eq34_e2268_d_n0, eq34_e2268_d_n1, eq34_e2268_d_n2, eq34_e2268_d_n3, eq34_e2268_d_n4, eq34_e2268_d_n5, eq34_e2268_d_n6, eq34_e2268_d_n7, eq34_e2268_d_n8, eq34_e2268_d_n9, eq34_e2268_d_n10, eq34_e2268_d_n11, eq34_e2268_d_n12, eq34_e2268_d_n13, eq34_e2268_d_n14, eq34_e2268_d_n15, eq34_e2268_d_n16,) = {
    if (s.v[1704] != 0.0) {
        let eq34_e2262: f64 = (s.v[114] * s.v[519]);
        let eq34_e2262_d_n0: f64 = ((s.dn[114][0] * s.v[519]) + (s.v[114] * s.dn[519][0]));
        let eq34_e2262_d_n1: f64 = ((s.dn[114][1] * s.v[519]) + (s.v[114] * s.dn[519][1]));
        let eq34_e2262_d_n2: f64 = ((s.dn[114][2] * s.v[519]) + (s.v[114] * s.dn[519][2]));
        let eq34_e2262_d_n3: f64 = ((s.dn[114][3] * s.v[519]) + (s.v[114] * s.dn[519][3]));
        let eq34_e2262_d_n4: f64 = ((s.dn[114][4] * s.v[519]) + (s.v[114] * s.dn[519][4]));
        let eq34_e2262_d_n5: f64 = ((s.dn[114][5] * s.v[519]) + (s.v[114] * s.dn[519][5]));
        let eq34_e2262_d_n6: f64 = ((s.dn[114][6] * s.v[519]) + (s.v[114] * s.dn[519][6]));
        let eq34_e2262_d_n7: f64 = ((s.dn[114][7] * s.v[519]) + (s.v[114] * s.dn[519][7]));
        let eq34_e2262_d_n8: f64 = ((s.dn[114][8] * s.v[519]) + (s.v[114] * s.dn[519][8]));
        let eq34_e2262_d_n9: f64 = ((s.dn[114][9] * s.v[519]) + (s.v[114] * s.dn[519][9]));
        let eq34_e2262_d_n10: f64 = ((s.dn[114][10] * s.v[519]) + (s.v[114] * s.dn[519][10]));
        let eq34_e2262_d_n11: f64 = ((s.dn[114][11] * s.v[519]) + (s.v[114] * s.dn[519][11]));
        let eq34_e2262_d_n12: f64 = ((s.dn[114][12] * s.v[519]) + (s.v[114] * s.dn[519][12]));
        let eq34_e2262_d_n13: f64 = ((s.dn[114][13] * s.v[519]) + (s.v[114] * s.dn[519][13]));
        let eq34_e2262_d_n14: f64 = ((s.dn[114][14] * s.v[519]) + (s.v[114] * s.dn[519][14]));
        let eq34_e2262_d_n15: f64 = ((s.dn[114][15] * s.v[519]) + (s.v[114] * s.dn[519][15]));
        let eq34_e2262_d_n16: f64 = ((s.dn[114][16] * s.v[519]) + (s.v[114] * s.dn[519][16]));
        let eq34_e2265: f64 = ((nv3 - nv6) * s.v[1052]);
        let eq34_e2265_d_n3: f64 = s.v[1052];
        let eq34_e2265_d_n6: f64 = (-s.v[1052]);
        let eq34_e2266: f64 = (eq34_e2262 + eq34_e2265);
        let eq34_e2266_d_n3: f64 = (eq34_e2262_d_n3 + eq34_e2265_d_n3);
        let eq34_e2266_d_n6: f64 = (eq34_e2262_d_n6 + eq34_e2265_d_n6);
        (eq34_e2266, eq34_e2262_d_n0, eq34_e2262_d_n1, eq34_e2262_d_n2, eq34_e2266_d_n3, eq34_e2262_d_n4, eq34_e2262_d_n5, eq34_e2266_d_n6, eq34_e2262_d_n7, eq34_e2262_d_n8, eq34_e2262_d_n9, eq34_e2262_d_n10, eq34_e2262_d_n11, eq34_e2262_d_n12, eq34_e2262_d_n13, eq34_e2262_d_n14, eq34_e2262_d_n15, eq34_e2262_d_n16,)
    } else {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    }
};
        let eq34_value: f64 = eq34_e2268;
        let eq34_node_derivatives: [f64; 17] = [eq34_e2268_d_n0, eq34_e2268_d_n1, eq34_e2268_d_n2, eq34_e2268_d_n3, eq34_e2268_d_n4, eq34_e2268_d_n5, eq34_e2268_d_n6, eq34_e2268_d_n7, eq34_e2268_d_n8, eq34_e2268_d_n9, eq34_e2268_d_n10, eq34_e2268_d_n11, eq34_e2268_d_n12, eq34_e2268_d_n13, eq34_e2268_d_n14, eq34_e2268_d_n15, eq34_e2268_d_n16];
        let eq34_branch_derivatives: [f64; 0] = [];
        stamper.stamp_current_dense(
            Some(nodes[3]),
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
        let nv3 = ctx.node_voltage(nodes[3]);
        let nv5 = ctx.node_voltage(nodes[5]);
        let (eq35_e2278, eq35_e2278_d_n0, eq35_e2278_d_n1, eq35_e2278_d_n2, eq35_e2278_d_n3, eq35_e2278_d_n4, eq35_e2278_d_n5, eq35_e2278_d_n6, eq35_e2278_d_n7, eq35_e2278_d_n8, eq35_e2278_d_n9, eq35_e2278_d_n10, eq35_e2278_d_n11, eq35_e2278_d_n12, eq35_e2278_d_n13, eq35_e2278_d_n14, eq35_e2278_d_n15, eq35_e2278_d_n16,) = {
    if (s.v[1704] != 0.0) {
        let eq35_e2272: f64 = (s.v[114] * s.v[520]);
        let eq35_e2272_d_n0: f64 = ((s.dn[114][0] * s.v[520]) + (s.v[114] * s.dn[520][0]));
        let eq35_e2272_d_n1: f64 = ((s.dn[114][1] * s.v[520]) + (s.v[114] * s.dn[520][1]));
        let eq35_e2272_d_n2: f64 = ((s.dn[114][2] * s.v[520]) + (s.v[114] * s.dn[520][2]));
        let eq35_e2272_d_n3: f64 = ((s.dn[114][3] * s.v[520]) + (s.v[114] * s.dn[520][3]));
        let eq35_e2272_d_n4: f64 = ((s.dn[114][4] * s.v[520]) + (s.v[114] * s.dn[520][4]));
        let eq35_e2272_d_n5: f64 = ((s.dn[114][5] * s.v[520]) + (s.v[114] * s.dn[520][5]));
        let eq35_e2272_d_n6: f64 = ((s.dn[114][6] * s.v[520]) + (s.v[114] * s.dn[520][6]));
        let eq35_e2272_d_n7: f64 = ((s.dn[114][7] * s.v[520]) + (s.v[114] * s.dn[520][7]));
        let eq35_e2272_d_n8: f64 = ((s.dn[114][8] * s.v[520]) + (s.v[114] * s.dn[520][8]));
        let eq35_e2272_d_n9: f64 = ((s.dn[114][9] * s.v[520]) + (s.v[114] * s.dn[520][9]));
        let eq35_e2272_d_n10: f64 = ((s.dn[114][10] * s.v[520]) + (s.v[114] * s.dn[520][10]));
        let eq35_e2272_d_n11: f64 = ((s.dn[114][11] * s.v[520]) + (s.v[114] * s.dn[520][11]));
        let eq35_e2272_d_n12: f64 = ((s.dn[114][12] * s.v[520]) + (s.v[114] * s.dn[520][12]));
        let eq35_e2272_d_n13: f64 = ((s.dn[114][13] * s.v[520]) + (s.v[114] * s.dn[520][13]));
        let eq35_e2272_d_n14: f64 = ((s.dn[114][14] * s.v[520]) + (s.v[114] * s.dn[520][14]));
        let eq35_e2272_d_n15: f64 = ((s.dn[114][15] * s.v[520]) + (s.v[114] * s.dn[520][15]));
        let eq35_e2272_d_n16: f64 = ((s.dn[114][16] * s.v[520]) + (s.v[114] * s.dn[520][16]));
        let eq35_e2275: f64 = ((nv3 - nv5) * s.v[1052]);
        let eq35_e2275_d_n3: f64 = s.v[1052];
        let eq35_e2275_d_n5: f64 = (-s.v[1052]);
        let eq35_e2276: f64 = (eq35_e2272 + eq35_e2275);
        let eq35_e2276_d_n3: f64 = (eq35_e2272_d_n3 + eq35_e2275_d_n3);
        let eq35_e2276_d_n5: f64 = (eq35_e2272_d_n5 + eq35_e2275_d_n5);
        (eq35_e2276, eq35_e2272_d_n0, eq35_e2272_d_n1, eq35_e2272_d_n2, eq35_e2276_d_n3, eq35_e2272_d_n4, eq35_e2276_d_n5, eq35_e2272_d_n6, eq35_e2272_d_n7, eq35_e2272_d_n8, eq35_e2272_d_n9, eq35_e2272_d_n10, eq35_e2272_d_n11, eq35_e2272_d_n12, eq35_e2272_d_n13, eq35_e2272_d_n14, eq35_e2272_d_n15, eq35_e2272_d_n16,)
    } else {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    }
};
        let eq35_value: f64 = eq35_e2278;
        let eq35_node_derivatives: [f64; 17] = [eq35_e2278_d_n0, eq35_e2278_d_n1, eq35_e2278_d_n2, eq35_e2278_d_n3, eq35_e2278_d_n4, eq35_e2278_d_n5, eq35_e2278_d_n6, eq35_e2278_d_n7, eq35_e2278_d_n8, eq35_e2278_d_n9, eq35_e2278_d_n10, eq35_e2278_d_n11, eq35_e2278_d_n12, eq35_e2278_d_n13, eq35_e2278_d_n14, eq35_e2278_d_n15, eq35_e2278_d_n16];
        let eq35_branch_derivatives: [f64; 0] = [];
        stamper.stamp_current_dense(
            Some(nodes[3]),
            Some(nodes[5]),
            self.multiplicity * (eq35_value),
            &nodes,
            &eq35_node_derivatives,
            &branches,
            &eq35_branch_derivatives,
            self.multiplicity,
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
        let eq36_e2281: f64 = self.eval_ddt(2, s.v[507]);
        let eq36_e2281_d_n0: f64 = self.ddt_jacobian(s.dn[507][0]);
        let eq36_e2281_d_n1: f64 = self.ddt_jacobian(s.dn[507][1]);
        let eq36_e2281_d_n2: f64 = self.ddt_jacobian(s.dn[507][2]);
        let eq36_e2281_d_n3: f64 = self.ddt_jacobian(s.dn[507][3]);
        let eq36_e2281_d_n4: f64 = self.ddt_jacobian(s.dn[507][4]);
        let eq36_e2281_d_n5: f64 = self.ddt_jacobian(s.dn[507][5]);
        let eq36_e2281_d_n6: f64 = self.ddt_jacobian(s.dn[507][6]);
        let eq36_e2281_d_n7: f64 = self.ddt_jacobian(s.dn[507][7]);
        let eq36_e2281_d_n8: f64 = self.ddt_jacobian(s.dn[507][8]);
        let eq36_e2281_d_n9: f64 = self.ddt_jacobian(s.dn[507][9]);
        let eq36_e2281_d_n10: f64 = self.ddt_jacobian(s.dn[507][10]);
        let eq36_e2281_d_n11: f64 = self.ddt_jacobian(s.dn[507][11]);
        let eq36_e2281_d_n12: f64 = self.ddt_jacobian(s.dn[507][12]);
        let eq36_e2281_d_n13: f64 = self.ddt_jacobian(s.dn[507][13]);
        let eq36_e2281_d_n14: f64 = self.ddt_jacobian(s.dn[507][14]);
        let eq36_e2281_d_n15: f64 = self.ddt_jacobian(s.dn[507][15]);
        let eq36_e2281_d_n16: f64 = self.ddt_jacobian(s.dn[507][16]);
        let eq36_e2282: f64 = (s.v[114] * eq36_e2281);
        let eq36_e2282_d_n0: f64 = ((s.dn[114][0] * eq36_e2281) + (s.v[114] * eq36_e2281_d_n0));
        let eq36_e2282_d_n1: f64 = ((s.dn[114][1] * eq36_e2281) + (s.v[114] * eq36_e2281_d_n1));
        let eq36_e2282_d_n2: f64 = ((s.dn[114][2] * eq36_e2281) + (s.v[114] * eq36_e2281_d_n2));
        let eq36_e2282_d_n3: f64 = ((s.dn[114][3] * eq36_e2281) + (s.v[114] * eq36_e2281_d_n3));
        let eq36_e2282_d_n4: f64 = ((s.dn[114][4] * eq36_e2281) + (s.v[114] * eq36_e2281_d_n4));
        let eq36_e2282_d_n5: f64 = ((s.dn[114][5] * eq36_e2281) + (s.v[114] * eq36_e2281_d_n5));
        let eq36_e2282_d_n6: f64 = ((s.dn[114][6] * eq36_e2281) + (s.v[114] * eq36_e2281_d_n6));
        let eq36_e2282_d_n7: f64 = ((s.dn[114][7] * eq36_e2281) + (s.v[114] * eq36_e2281_d_n7));
        let eq36_e2282_d_n8: f64 = ((s.dn[114][8] * eq36_e2281) + (s.v[114] * eq36_e2281_d_n8));
        let eq36_e2282_d_n9: f64 = ((s.dn[114][9] * eq36_e2281) + (s.v[114] * eq36_e2281_d_n9));
        let eq36_e2282_d_n10: f64 = ((s.dn[114][10] * eq36_e2281) + (s.v[114] * eq36_e2281_d_n10));
        let eq36_e2282_d_n11: f64 = ((s.dn[114][11] * eq36_e2281) + (s.v[114] * eq36_e2281_d_n11));
        let eq36_e2282_d_n12: f64 = ((s.dn[114][12] * eq36_e2281) + (s.v[114] * eq36_e2281_d_n12));
        let eq36_e2282_d_n13: f64 = ((s.dn[114][13] * eq36_e2281) + (s.v[114] * eq36_e2281_d_n13));
        let eq36_e2282_d_n14: f64 = ((s.dn[114][14] * eq36_e2281) + (s.v[114] * eq36_e2281_d_n14));
        let eq36_e2282_d_n15: f64 = ((s.dn[114][15] * eq36_e2281) + (s.v[114] * eq36_e2281_d_n15));
        let eq36_e2282_d_n16: f64 = ((s.dn[114][16] * eq36_e2281) + (s.v[114] * eq36_e2281_d_n16));
        let eq36_value: f64 = eq36_e2282;
        let eq36_node_derivatives: [f64; 17] = [eq36_e2282_d_n0, eq36_e2282_d_n1, eq36_e2282_d_n2, eq36_e2282_d_n3, eq36_e2282_d_n4, eq36_e2282_d_n5, eq36_e2282_d_n6, eq36_e2282_d_n7, eq36_e2282_d_n8, eq36_e2282_d_n9, eq36_e2282_d_n10, eq36_e2282_d_n11, eq36_e2282_d_n12, eq36_e2282_d_n13, eq36_e2282_d_n14, eq36_e2282_d_n15, eq36_e2282_d_n16];
        let eq36_branch_derivatives: [f64; 0] = [];
        stamper.stamp_current_dense(
            Some(nodes[3]),
            Some(nodes[6]),
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
        let eq37_e2285: f64 = self.eval_ddt(3, s.v[508]);
        let eq37_e2285_d_n0: f64 = self.ddt_jacobian(s.dn[508][0]);
        let eq37_e2285_d_n1: f64 = self.ddt_jacobian(s.dn[508][1]);
        let eq37_e2285_d_n2: f64 = self.ddt_jacobian(s.dn[508][2]);
        let eq37_e2285_d_n3: f64 = self.ddt_jacobian(s.dn[508][3]);
        let eq37_e2285_d_n4: f64 = self.ddt_jacobian(s.dn[508][4]);
        let eq37_e2285_d_n5: f64 = self.ddt_jacobian(s.dn[508][5]);
        let eq37_e2285_d_n6: f64 = self.ddt_jacobian(s.dn[508][6]);
        let eq37_e2285_d_n7: f64 = self.ddt_jacobian(s.dn[508][7]);
        let eq37_e2285_d_n8: f64 = self.ddt_jacobian(s.dn[508][8]);
        let eq37_e2285_d_n9: f64 = self.ddt_jacobian(s.dn[508][9]);
        let eq37_e2285_d_n10: f64 = self.ddt_jacobian(s.dn[508][10]);
        let eq37_e2285_d_n11: f64 = self.ddt_jacobian(s.dn[508][11]);
        let eq37_e2285_d_n12: f64 = self.ddt_jacobian(s.dn[508][12]);
        let eq37_e2285_d_n13: f64 = self.ddt_jacobian(s.dn[508][13]);
        let eq37_e2285_d_n14: f64 = self.ddt_jacobian(s.dn[508][14]);
        let eq37_e2285_d_n15: f64 = self.ddt_jacobian(s.dn[508][15]);
        let eq37_e2285_d_n16: f64 = self.ddt_jacobian(s.dn[508][16]);
        let eq37_e2286: f64 = (s.v[114] * eq37_e2285);
        let eq37_e2286_d_n0: f64 = ((s.dn[114][0] * eq37_e2285) + (s.v[114] * eq37_e2285_d_n0));
        let eq37_e2286_d_n1: f64 = ((s.dn[114][1] * eq37_e2285) + (s.v[114] * eq37_e2285_d_n1));
        let eq37_e2286_d_n2: f64 = ((s.dn[114][2] * eq37_e2285) + (s.v[114] * eq37_e2285_d_n2));
        let eq37_e2286_d_n3: f64 = ((s.dn[114][3] * eq37_e2285) + (s.v[114] * eq37_e2285_d_n3));
        let eq37_e2286_d_n4: f64 = ((s.dn[114][4] * eq37_e2285) + (s.v[114] * eq37_e2285_d_n4));
        let eq37_e2286_d_n5: f64 = ((s.dn[114][5] * eq37_e2285) + (s.v[114] * eq37_e2285_d_n5));
        let eq37_e2286_d_n6: f64 = ((s.dn[114][6] * eq37_e2285) + (s.v[114] * eq37_e2285_d_n6));
        let eq37_e2286_d_n7: f64 = ((s.dn[114][7] * eq37_e2285) + (s.v[114] * eq37_e2285_d_n7));
        let eq37_e2286_d_n8: f64 = ((s.dn[114][8] * eq37_e2285) + (s.v[114] * eq37_e2285_d_n8));
        let eq37_e2286_d_n9: f64 = ((s.dn[114][9] * eq37_e2285) + (s.v[114] * eq37_e2285_d_n9));
        let eq37_e2286_d_n10: f64 = ((s.dn[114][10] * eq37_e2285) + (s.v[114] * eq37_e2285_d_n10));
        let eq37_e2286_d_n11: f64 = ((s.dn[114][11] * eq37_e2285) + (s.v[114] * eq37_e2285_d_n11));
        let eq37_e2286_d_n12: f64 = ((s.dn[114][12] * eq37_e2285) + (s.v[114] * eq37_e2285_d_n12));
        let eq37_e2286_d_n13: f64 = ((s.dn[114][13] * eq37_e2285) + (s.v[114] * eq37_e2285_d_n13));
        let eq37_e2286_d_n14: f64 = ((s.dn[114][14] * eq37_e2285) + (s.v[114] * eq37_e2285_d_n14));
        let eq37_e2286_d_n15: f64 = ((s.dn[114][15] * eq37_e2285) + (s.v[114] * eq37_e2285_d_n15));
        let eq37_e2286_d_n16: f64 = ((s.dn[114][16] * eq37_e2285) + (s.v[114] * eq37_e2285_d_n16));
        let eq37_value: f64 = eq37_e2286;
        let eq37_node_derivatives: [f64; 17] = [eq37_e2286_d_n0, eq37_e2286_d_n1, eq37_e2286_d_n2, eq37_e2286_d_n3, eq37_e2286_d_n4, eq37_e2286_d_n5, eq37_e2286_d_n6, eq37_e2286_d_n7, eq37_e2286_d_n8, eq37_e2286_d_n9, eq37_e2286_d_n10, eq37_e2286_d_n11, eq37_e2286_d_n12, eq37_e2286_d_n13, eq37_e2286_d_n14, eq37_e2286_d_n15, eq37_e2286_d_n16];
        let eq37_branch_derivatives: [f64; 0] = [];
        stamper.stamp_current_dense(
            Some(nodes[3]),
            Some(nodes[5]),
            self.multiplicity * (eq37_value),
            &nodes,
            &eq37_node_derivatives,
            &branches,
            &eq37_branch_derivatives,
            self.multiplicity,
        );
    }

    pub(super) fn stamp_transient_equation_38_block_0(
        &mut self,
        ctx: &GeneratedEvalContext<'_>,
        stamper: &mut GeneratedStamper<'_>,
        s: &mut Scratch,
    ) {
        let _ = stamper;
        let p = self.params;
        let nodes = self.nodes;
        let branches = self.branches;
        let eq38_e2289: f64 = self.eval_ddt(4, s.v[509]);
        let eq38_e2289_d_n0: f64 = self.ddt_jacobian(s.dn[509][0]);
        let eq38_e2289_d_n1: f64 = self.ddt_jacobian(s.dn[509][1]);
        let eq38_e2289_d_n2: f64 = self.ddt_jacobian(s.dn[509][2]);
        let eq38_e2289_d_n3: f64 = self.ddt_jacobian(s.dn[509][3]);
        let eq38_e2289_d_n4: f64 = self.ddt_jacobian(s.dn[509][4]);
        let eq38_e2289_d_n5: f64 = self.ddt_jacobian(s.dn[509][5]);
        let eq38_e2289_d_n6: f64 = self.ddt_jacobian(s.dn[509][6]);
        let eq38_e2289_d_n7: f64 = self.ddt_jacobian(s.dn[509][7]);
        let eq38_e2289_d_n8: f64 = self.ddt_jacobian(s.dn[509][8]);
        let eq38_e2289_d_n9: f64 = self.ddt_jacobian(s.dn[509][9]);
        let eq38_e2289_d_n10: f64 = self.ddt_jacobian(s.dn[509][10]);
        let eq38_e2289_d_n11: f64 = self.ddt_jacobian(s.dn[509][11]);
        let eq38_e2289_d_n12: f64 = self.ddt_jacobian(s.dn[509][12]);
        let eq38_e2289_d_n13: f64 = self.ddt_jacobian(s.dn[509][13]);
        let eq38_e2289_d_n14: f64 = self.ddt_jacobian(s.dn[509][14]);
        let eq38_e2289_d_n15: f64 = self.ddt_jacobian(s.dn[509][15]);
        let eq38_e2289_d_n16: f64 = self.ddt_jacobian(s.dn[509][16]);
        let eq38_e2290: f64 = (s.v[114] * eq38_e2289);
        let eq38_e2290_d_n0: f64 = ((s.dn[114][0] * eq38_e2289) + (s.v[114] * eq38_e2289_d_n0));
        let eq38_e2290_d_n1: f64 = ((s.dn[114][1] * eq38_e2289) + (s.v[114] * eq38_e2289_d_n1));
        let eq38_e2290_d_n2: f64 = ((s.dn[114][2] * eq38_e2289) + (s.v[114] * eq38_e2289_d_n2));
        let eq38_e2290_d_n3: f64 = ((s.dn[114][3] * eq38_e2289) + (s.v[114] * eq38_e2289_d_n3));
        let eq38_e2290_d_n4: f64 = ((s.dn[114][4] * eq38_e2289) + (s.v[114] * eq38_e2289_d_n4));
        let eq38_e2290_d_n5: f64 = ((s.dn[114][5] * eq38_e2289) + (s.v[114] * eq38_e2289_d_n5));
        let eq38_e2290_d_n6: f64 = ((s.dn[114][6] * eq38_e2289) + (s.v[114] * eq38_e2289_d_n6));
        let eq38_e2290_d_n7: f64 = ((s.dn[114][7] * eq38_e2289) + (s.v[114] * eq38_e2289_d_n7));
        let eq38_e2290_d_n8: f64 = ((s.dn[114][8] * eq38_e2289) + (s.v[114] * eq38_e2289_d_n8));
        let eq38_e2290_d_n9: f64 = ((s.dn[114][9] * eq38_e2289) + (s.v[114] * eq38_e2289_d_n9));
        let eq38_e2290_d_n10: f64 = ((s.dn[114][10] * eq38_e2289) + (s.v[114] * eq38_e2289_d_n10));
        let eq38_e2290_d_n11: f64 = ((s.dn[114][11] * eq38_e2289) + (s.v[114] * eq38_e2289_d_n11));
        let eq38_e2290_d_n12: f64 = ((s.dn[114][12] * eq38_e2289) + (s.v[114] * eq38_e2289_d_n12));
        let eq38_e2290_d_n13: f64 = ((s.dn[114][13] * eq38_e2289) + (s.v[114] * eq38_e2289_d_n13));
        let eq38_e2290_d_n14: f64 = ((s.dn[114][14] * eq38_e2289) + (s.v[114] * eq38_e2289_d_n14));
        let eq38_e2290_d_n15: f64 = ((s.dn[114][15] * eq38_e2289) + (s.v[114] * eq38_e2289_d_n15));
        let eq38_e2290_d_n16: f64 = ((s.dn[114][16] * eq38_e2289) + (s.v[114] * eq38_e2289_d_n16));
        let eq38_value: f64 = eq38_e2290;
        let eq38_node_derivatives: [f64; 17] = [eq38_e2290_d_n0, eq38_e2290_d_n1, eq38_e2290_d_n2, eq38_e2290_d_n3, eq38_e2290_d_n4, eq38_e2290_d_n5, eq38_e2290_d_n6, eq38_e2290_d_n7, eq38_e2290_d_n8, eq38_e2290_d_n9, eq38_e2290_d_n10, eq38_e2290_d_n11, eq38_e2290_d_n12, eq38_e2290_d_n13, eq38_e2290_d_n14, eq38_e2290_d_n15, eq38_e2290_d_n16];
        let eq38_branch_derivatives: [f64; 0] = [];
        stamper.stamp_current_dense(
            Some(nodes[3]),
            Some(nodes[10]),
            self.multiplicity * (eq38_value),
            &nodes,
            &eq38_node_derivatives,
            &branches,
            &eq38_branch_derivatives,
            self.multiplicity,
        );
    }
}
