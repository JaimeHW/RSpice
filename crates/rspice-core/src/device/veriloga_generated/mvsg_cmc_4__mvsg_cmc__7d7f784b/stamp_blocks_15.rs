#![allow(dead_code, unused_assignments, unused_imports, unused_parens, unused_variables)]

use super::{A, GeneratedDerivative, GeneratedEvalContext, GeneratedReactiveStamper, GeneratedStamper, Scratch, THERMAL_VOLTAGE_PER_K};
use super::super::state::Instance;

impl Instance {

    pub(super) fn stamp_transient_equation_180_block_0(
        &mut self,
        ctx: &GeneratedEvalContext<'_>,
        stamper: &mut GeneratedStamper<'_>,
        s: &mut Scratch,
    ) {
        let _ = stamper;
        let p = self.params;
        let nodes = self.nodes;
        let branches = self.branches;
        let (eq180_e1942,) = {
    if (s.v[2686] != 0.0) {
        (0.0,)
    } else {
        (0.0,)
    }
};
        let eq180_value: f64 = eq180_e1942;
        stamper.stamp_current(
            Some(nodes[8]),
            Some(nodes[13]),
            self.multiplicity * (eq180_value),
            &[
            ],
        );
    }

    pub(super) fn stamp_transient_equation_181_block_0(
        &mut self,
        ctx: &GeneratedEvalContext<'_>,
        stamper: &mut GeneratedStamper<'_>,
        s: &mut Scratch,
    ) {
        let _ = stamper;
        let p = self.params;
        let nodes = self.nodes;
        let branches = self.branches;
        let (eq181_e1959,) = {
    if (s.v[2686] != 0.0) {
        (0.0,)
    } else {
        (0.0,)
    }
};
        let eq181_value: f64 = eq181_e1959;
        stamper.stamp_current(
            Some(nodes[8]),
            Some(nodes[17]),
            self.multiplicity * (eq181_value),
            &[
            ],
        );
    }

    pub(super) fn stamp_transient_equation_182_block_0(
        &mut self,
        ctx: &GeneratedEvalContext<'_>,
        stamper: &mut GeneratedStamper<'_>,
        s: &mut Scratch,
    ) {
        let _ = stamper;
        let p = self.params;
        let nodes = self.nodes;
        let branches = self.branches;
        let (eq182_e1966,) = {
    if (s.v[2686] != 0.0) {
        (0.0,)
    } else {
        (0.0,)
    }
};
        let eq182_value: f64 = eq182_e1966;
        stamper.stamp_current(
            Some(nodes[5]),
            Some(nodes[9]),
            self.multiplicity * (eq182_value),
            &[
            ],
        );
    }

    pub(super) fn stamp_transient_equation_183_block_0(
        &mut self,
        ctx: &GeneratedEvalContext<'_>,
        stamper: &mut GeneratedStamper<'_>,
        s: &mut Scratch,
    ) {
        let _ = stamper;
        let p = self.params;
        let nodes = self.nodes;
        let branches = self.branches;
        let (eq183_e1972,) = {
    if (s.v[2686] != 0.0) {
        (0.0,)
    } else {
        (0.0,)
    }
};
        let eq183_value: f64 = eq183_e1972;
        stamper.stamp_current(
            Some(nodes[5]),
            Some(nodes[9]),
            self.multiplicity * (eq183_value),
            &[
            ],
        );
    }

    pub(super) fn stamp_transient_equation_184_block_0(
        &mut self,
        ctx: &GeneratedEvalContext<'_>,
        stamper: &mut GeneratedStamper<'_>,
        s: &mut Scratch,
    ) {
        let _ = stamper;
        let p = self.params;
        let nodes = self.nodes;
        let branches = self.branches;
        let (eq184_e1992,) = {
    if ((s.v[2686] != 0.0) && (s.v[2688] != 0.0)) {
        (0.0,)
    } else {
        (0.0,)
    }
};
        let eq184_value: f64 = eq184_e1992;
        stamper.stamp_current(
            Some(nodes[9]),
            Some(nodes[10]),
            self.multiplicity * (eq184_value),
            &[
            ],
        );
    }

    pub(super) fn stamp_transient_equation_185_block_0(
        &mut self,
        ctx: &GeneratedEvalContext<'_>,
        stamper: &mut GeneratedStamper<'_>,
        s: &mut Scratch,
    ) {
        let _ = stamper;
        let p = self.params;
        let nodes = self.nodes;
        let branches = self.branches;
        let (eq185_e2012,) = {
    if ((s.v[2686] != 0.0) && (s.v[2689] != 0.0)) {
        (0.0,)
    } else {
        (0.0,)
    }
};
        let eq185_value: f64 = eq185_e2012;
        stamper.stamp_current(
            Some(nodes[10]),
            Some(nodes[11]),
            self.multiplicity * (eq185_value),
            &[
            ],
        );
    }

    pub(super) fn stamp_transient_equation_186_block_0(
        &mut self,
        ctx: &GeneratedEvalContext<'_>,
        stamper: &mut GeneratedStamper<'_>,
        s: &mut Scratch,
    ) {
        let _ = stamper;
        let p = self.params;
        let nodes = self.nodes;
        let branches = self.branches;
        let (eq186_e2032,) = {
    if ((s.v[2686] != 0.0) && (s.v[2690] != 0.0)) {
        (0.0,)
    } else {
        (0.0,)
    }
};
        let eq186_value: f64 = eq186_e2032;
        stamper.stamp_current(
            Some(nodes[11]),
            Some(nodes[12]),
            self.multiplicity * (eq186_value),
            &[
            ],
        );
    }

    pub(super) fn stamp_transient_equation_187_block_0(
        &mut self,
        ctx: &GeneratedEvalContext<'_>,
        stamper: &mut GeneratedStamper<'_>,
        s: &mut Scratch,
    ) {
        let _ = stamper;
        let p = self.params;
        let nodes = self.nodes;
        let branches = self.branches;
        let (eq187_e2052,) = {
    if ((s.v[2686] != 0.0) && (s.v[2691] != 0.0)) {
        (0.0,)
    } else {
        (0.0,)
    }
};
        let eq187_value: f64 = eq187_e2052;
        stamper.stamp_current(
            Some(nodes[12]),
            Some(nodes[13]),
            self.multiplicity * (eq187_value),
            &[
            ],
        );
    }

    pub(super) fn stamp_transient_equation_188_block_0(
        &mut self,
        ctx: &GeneratedEvalContext<'_>,
        stamper: &mut GeneratedStamper<'_>,
        s: &mut Scratch,
    ) {
        let _ = stamper;
        let p = self.params;
        let nodes = self.nodes;
        let branches = self.branches;
        let (eq188_e2072,) = {
    if ((s.v[2686] != 0.0) && (s.v[2692] != 0.0)) {
        (0.0,)
    } else {
        (0.0,)
    }
};
        let eq188_value: f64 = eq188_e2072;
        stamper.stamp_current(
            Some(nodes[14]),
            Some(nodes[5]),
            self.multiplicity * (eq188_value),
            &[
            ],
        );
    }

    pub(super) fn stamp_transient_equation_189_block_0(
        &mut self,
        ctx: &GeneratedEvalContext<'_>,
        stamper: &mut GeneratedStamper<'_>,
        s: &mut Scratch,
    ) {
        let _ = stamper;
        let p = self.params;
        let nodes = self.nodes;
        let branches = self.branches;
        let (eq189_e2092,) = {
    if ((s.v[2686] != 0.0) && (s.v[2693] != 0.0)) {
        (0.0,)
    } else {
        (0.0,)
    }
};
        let eq189_value: f64 = eq189_e2092;
        stamper.stamp_current(
            Some(nodes[15]),
            Some(nodes[14]),
            self.multiplicity * (eq189_value),
            &[
            ],
        );
    }

    pub(super) fn stamp_transient_equation_190_block_0(
        &mut self,
        ctx: &GeneratedEvalContext<'_>,
        stamper: &mut GeneratedStamper<'_>,
        s: &mut Scratch,
    ) {
        let _ = stamper;
        let p = self.params;
        let nodes = self.nodes;
        let branches = self.branches;
        let (eq190_e2112,) = {
    if ((s.v[2686] != 0.0) && (s.v[2694] != 0.0)) {
        (0.0,)
    } else {
        (0.0,)
    }
};
        let eq190_value: f64 = eq190_e2112;
        stamper.stamp_current(
            Some(nodes[16]),
            Some(nodes[15]),
            self.multiplicity * (eq190_value),
            &[
            ],
        );
    }

    pub(super) fn stamp_transient_equation_191_block_0(
        &mut self,
        ctx: &GeneratedEvalContext<'_>,
        stamper: &mut GeneratedStamper<'_>,
        s: &mut Scratch,
    ) {
        let _ = stamper;
        let p = self.params;
        let nodes = self.nodes;
        let branches = self.branches;
        let (eq191_e2132,) = {
    if ((s.v[2686] != 0.0) && (s.v[2695] != 0.0)) {
        (0.0,)
    } else {
        (0.0,)
    }
};
        let eq191_value: f64 = eq191_e2132;
        stamper.stamp_current(
            Some(nodes[17]),
            Some(nodes[16]),
            self.multiplicity * (eq191_value),
            &[
            ],
        );
    }

    pub(super) fn stamp_transient_equation_192_block_0(
        &mut self,
        ctx: &GeneratedEvalContext<'_>,
        stamper: &mut GeneratedStamper<'_>,
        s: &mut Scratch,
    ) {
        let _ = stamper;
        let p = self.params;
        let nodes = self.nodes;
        let branches = self.branches;
        let (eq192_e2146,) = {
    if ((s.v[2686] != 0.0) && (s.v[2696] != 0.0)) {
        (0.0,)
    } else {
        (0.0,)
    }
};
        let eq192_value: f64 = eq192_e2146;
        stamper.stamp_current(
            Some(nodes[19]),
            Some(nodes[2]),
            self.multiplicity * (eq192_value),
            &[
            ],
        );
    }

    pub(super) fn stamp_transient_equation_193_block_0(
        &mut self,
        ctx: &GeneratedEvalContext<'_>,
        stamper: &mut GeneratedStamper<'_>,
        s: &mut Scratch,
    ) {
        let _ = stamper;
        let p = self.params;
        let nodes = self.nodes;
        let branches = self.branches;
        let (eq193_e2160,) = {
    if ((s.v[2686] != 0.0) && (s.v[2697] != 0.0)) {
        (0.0,)
    } else {
        (0.0,)
    }
};
        let eq193_value: f64 = eq193_e2160;
        stamper.stamp_current(
            Some(nodes[0]),
            Some(nodes[18]),
            self.multiplicity * (eq193_value),
            &[
            ],
        );
    }

    pub(super) fn stamp_transient_equation_194_block_0(
        &mut self,
        ctx: &GeneratedEvalContext<'_>,
        stamper: &mut GeneratedStamper<'_>,
        s: &mut Scratch,
    ) {
        let _ = stamper;
        let p = self.params;
        let nodes = self.nodes;
        let branches = self.branches;
        let nv4 = ctx.node_voltage(nodes[4]);
        let (eq194_e2167, eq194_e2167_d_n0, eq194_e2167_d_n1, eq194_e2167_d_n2, eq194_e2167_d_n3, eq194_e2167_d_n4, eq194_e2167_d_n5, eq194_e2167_d_n6, eq194_e2167_d_n7, eq194_e2167_d_n8, eq194_e2167_d_n9, eq194_e2167_d_n10, eq194_e2167_d_n11, eq194_e2167_d_n12, eq194_e2167_d_n13, eq194_e2167_d_n14, eq194_e2167_d_n15, eq194_e2167_d_n16, eq194_e2167_d_n17, eq194_e2167_d_n18, eq194_e2167_d_n19, eq194_e2167_d_n20, eq194_e2167_d_n21, eq194_e2167_d_n22, eq194_e2167_d_n23, eq194_e2167_d_n24, eq194_e2167_d_n25, eq194_e2167_d_n26, eq194_e2167_d_n27, eq194_e2167_d_n28, eq194_e2167_d_n29,) = {
    if (s.v[2700] != 0.0) {
        let eq194_e2164: f64 = (p.p321 * (nv4 - 0.0));
        let eq194_e2164_d_n4: f64 = p.p321;
        let eq194_e2165: f64 = self.eval_ddt(145, eq194_e2164);
        let eq194_e2165_d_n0: f64 = self.ddt_jacobian(0.0);
        let eq194_e2165_d_n1: f64 = self.ddt_jacobian(0.0);
        let eq194_e2165_d_n2: f64 = self.ddt_jacobian(0.0);
        let eq194_e2165_d_n3: f64 = self.ddt_jacobian(0.0);
        let eq194_e2165_d_n4: f64 = self.ddt_jacobian(eq194_e2164_d_n4);
        let eq194_e2165_d_n5: f64 = self.ddt_jacobian(0.0);
        let eq194_e2165_d_n6: f64 = self.ddt_jacobian(0.0);
        let eq194_e2165_d_n7: f64 = self.ddt_jacobian(0.0);
        let eq194_e2165_d_n8: f64 = self.ddt_jacobian(0.0);
        let eq194_e2165_d_n9: f64 = self.ddt_jacobian(0.0);
        let eq194_e2165_d_n10: f64 = self.ddt_jacobian(0.0);
        let eq194_e2165_d_n11: f64 = self.ddt_jacobian(0.0);
        let eq194_e2165_d_n12: f64 = self.ddt_jacobian(0.0);
        let eq194_e2165_d_n13: f64 = self.ddt_jacobian(0.0);
        let eq194_e2165_d_n14: f64 = self.ddt_jacobian(0.0);
        let eq194_e2165_d_n15: f64 = self.ddt_jacobian(0.0);
        let eq194_e2165_d_n16: f64 = self.ddt_jacobian(0.0);
        let eq194_e2165_d_n17: f64 = self.ddt_jacobian(0.0);
        let eq194_e2165_d_n18: f64 = self.ddt_jacobian(0.0);
        let eq194_e2165_d_n19: f64 = self.ddt_jacobian(0.0);
        let eq194_e2165_d_n20: f64 = self.ddt_jacobian(0.0);
        let eq194_e2165_d_n21: f64 = self.ddt_jacobian(0.0);
        let eq194_e2165_d_n22: f64 = self.ddt_jacobian(0.0);
        let eq194_e2165_d_n23: f64 = self.ddt_jacobian(0.0);
        let eq194_e2165_d_n24: f64 = self.ddt_jacobian(0.0);
        let eq194_e2165_d_n25: f64 = self.ddt_jacobian(0.0);
        let eq194_e2165_d_n26: f64 = self.ddt_jacobian(0.0);
        let eq194_e2165_d_n27: f64 = self.ddt_jacobian(0.0);
        let eq194_e2165_d_n28: f64 = self.ddt_jacobian(0.0);
        let eq194_e2165_d_n29: f64 = self.ddt_jacobian(0.0);
        (eq194_e2165, eq194_e2165_d_n0, eq194_e2165_d_n1, eq194_e2165_d_n2, eq194_e2165_d_n3, eq194_e2165_d_n4, eq194_e2165_d_n5, eq194_e2165_d_n6, eq194_e2165_d_n7, eq194_e2165_d_n8, eq194_e2165_d_n9, eq194_e2165_d_n10, eq194_e2165_d_n11, eq194_e2165_d_n12, eq194_e2165_d_n13, eq194_e2165_d_n14, eq194_e2165_d_n15, eq194_e2165_d_n16, eq194_e2165_d_n17, eq194_e2165_d_n18, eq194_e2165_d_n19, eq194_e2165_d_n20, eq194_e2165_d_n21, eq194_e2165_d_n22, eq194_e2165_d_n23, eq194_e2165_d_n24, eq194_e2165_d_n25, eq194_e2165_d_n26, eq194_e2165_d_n27, eq194_e2165_d_n28, eq194_e2165_d_n29,)
    } else {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    }
};
        let eq194_value: f64 = eq194_e2167;
        let eq194_node_derivatives: [f64; 30] = [eq194_e2167_d_n0, eq194_e2167_d_n1, eq194_e2167_d_n2, eq194_e2167_d_n3, eq194_e2167_d_n4, eq194_e2167_d_n5, eq194_e2167_d_n6, eq194_e2167_d_n7, eq194_e2167_d_n8, eq194_e2167_d_n9, eq194_e2167_d_n10, eq194_e2167_d_n11, eq194_e2167_d_n12, eq194_e2167_d_n13, eq194_e2167_d_n14, eq194_e2167_d_n15, eq194_e2167_d_n16, eq194_e2167_d_n17, eq194_e2167_d_n18, eq194_e2167_d_n19, eq194_e2167_d_n20, eq194_e2167_d_n21, eq194_e2167_d_n22, eq194_e2167_d_n23, eq194_e2167_d_n24, eq194_e2167_d_n25, eq194_e2167_d_n26, eq194_e2167_d_n27, eq194_e2167_d_n28, eq194_e2167_d_n29];
        let eq194_branch_derivatives: [f64; 0] = [];
        stamper.stamp_current_dense(
            Some(nodes[4]),
            None,
            self.multiplicity * (eq194_value),
            &nodes,
            &eq194_node_derivatives,
            &branches,
            &eq194_branch_derivatives,
            self.multiplicity,
        );
    }

    pub(super) fn stamp_transient_equation_195_block_0(
        &mut self,
        ctx: &GeneratedEvalContext<'_>,
        stamper: &mut GeneratedStamper<'_>,
        s: &mut Scratch,
    ) {
        let _ = stamper;
        let p = self.params;
        let nodes = self.nodes;
        let branches = self.branches;
        let (eq195_e2172, eq195_e2172_d_n0, eq195_e2172_d_n1, eq195_e2172_d_n2, eq195_e2172_d_n3, eq195_e2172_d_n4, eq195_e2172_d_n5, eq195_e2172_d_n6, eq195_e2172_d_n7, eq195_e2172_d_n8, eq195_e2172_d_n9, eq195_e2172_d_n10, eq195_e2172_d_n11, eq195_e2172_d_n12, eq195_e2172_d_n13, eq195_e2172_d_n14, eq195_e2172_d_n15, eq195_e2172_d_n16, eq195_e2172_d_n17, eq195_e2172_d_n18, eq195_e2172_d_n19, eq195_e2172_d_n20, eq195_e2172_d_n21, eq195_e2172_d_n22, eq195_e2172_d_n23, eq195_e2172_d_n24, eq195_e2172_d_n25, eq195_e2172_d_n26, eq195_e2172_d_n27, eq195_e2172_d_n28, eq195_e2172_d_n29,) = {
    if (s.v[2700] != 0.0) {
        let eq195_e2170: f64 = (-s.v[114]);
        let eq195_e2170_d_n0: f64 = (-s.dn[114][0]);
        let eq195_e2170_d_n1: f64 = (-s.dn[114][1]);
        let eq195_e2170_d_n2: f64 = (-s.dn[114][2]);
        let eq195_e2170_d_n3: f64 = (-s.dn[114][3]);
        let eq195_e2170_d_n4: f64 = (-s.dn[114][4]);
        let eq195_e2170_d_n5: f64 = (-s.dn[114][5]);
        let eq195_e2170_d_n6: f64 = (-s.dn[114][6]);
        let eq195_e2170_d_n7: f64 = (-s.dn[114][7]);
        let eq195_e2170_d_n8: f64 = (-s.dn[114][8]);
        let eq195_e2170_d_n9: f64 = (-s.dn[114][9]);
        let eq195_e2170_d_n10: f64 = (-s.dn[114][10]);
        let eq195_e2170_d_n11: f64 = (-s.dn[114][11]);
        let eq195_e2170_d_n12: f64 = (-s.dn[114][12]);
        let eq195_e2170_d_n13: f64 = (-s.dn[114][13]);
        let eq195_e2170_d_n14: f64 = (-s.dn[114][14]);
        let eq195_e2170_d_n15: f64 = (-s.dn[114][15]);
        let eq195_e2170_d_n16: f64 = (-s.dn[114][16]);
        let eq195_e2170_d_n17: f64 = (-s.dn[114][17]);
        let eq195_e2170_d_n18: f64 = (-s.dn[114][18]);
        let eq195_e2170_d_n19: f64 = (-s.dn[114][19]);
        let eq195_e2170_d_n20: f64 = (-s.dn[114][20]);
        let eq195_e2170_d_n21: f64 = (-s.dn[114][21]);
        let eq195_e2170_d_n22: f64 = (-s.dn[114][22]);
        let eq195_e2170_d_n23: f64 = (-s.dn[114][23]);
        let eq195_e2170_d_n24: f64 = (-s.dn[114][24]);
        let eq195_e2170_d_n25: f64 = (-s.dn[114][25]);
        let eq195_e2170_d_n26: f64 = (-s.dn[114][26]);
        let eq195_e2170_d_n27: f64 = (-s.dn[114][27]);
        let eq195_e2170_d_n28: f64 = (-s.dn[114][28]);
        let eq195_e2170_d_n29: f64 = (-s.dn[114][29]);
        (eq195_e2170, eq195_e2170_d_n0, eq195_e2170_d_n1, eq195_e2170_d_n2, eq195_e2170_d_n3, eq195_e2170_d_n4, eq195_e2170_d_n5, eq195_e2170_d_n6, eq195_e2170_d_n7, eq195_e2170_d_n8, eq195_e2170_d_n9, eq195_e2170_d_n10, eq195_e2170_d_n11, eq195_e2170_d_n12, eq195_e2170_d_n13, eq195_e2170_d_n14, eq195_e2170_d_n15, eq195_e2170_d_n16, eq195_e2170_d_n17, eq195_e2170_d_n18, eq195_e2170_d_n19, eq195_e2170_d_n20, eq195_e2170_d_n21, eq195_e2170_d_n22, eq195_e2170_d_n23, eq195_e2170_d_n24, eq195_e2170_d_n25, eq195_e2170_d_n26, eq195_e2170_d_n27, eq195_e2170_d_n28, eq195_e2170_d_n29,)
    } else {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    }
};
        let eq195_value: f64 = eq195_e2172;
        let eq195_node_derivatives: [f64; 30] = [eq195_e2172_d_n0, eq195_e2172_d_n1, eq195_e2172_d_n2, eq195_e2172_d_n3, eq195_e2172_d_n4, eq195_e2172_d_n5, eq195_e2172_d_n6, eq195_e2172_d_n7, eq195_e2172_d_n8, eq195_e2172_d_n9, eq195_e2172_d_n10, eq195_e2172_d_n11, eq195_e2172_d_n12, eq195_e2172_d_n13, eq195_e2172_d_n14, eq195_e2172_d_n15, eq195_e2172_d_n16, eq195_e2172_d_n17, eq195_e2172_d_n18, eq195_e2172_d_n19, eq195_e2172_d_n20, eq195_e2172_d_n21, eq195_e2172_d_n22, eq195_e2172_d_n23, eq195_e2172_d_n24, eq195_e2172_d_n25, eq195_e2172_d_n26, eq195_e2172_d_n27, eq195_e2172_d_n28, eq195_e2172_d_n29];
        let eq195_branch_derivatives: [f64; 0] = [];
        stamper.stamp_current_dense(
            Some(nodes[4]),
            None,
            self.multiplicity * (eq195_value),
            &nodes,
            &eq195_node_derivatives,
            &branches,
            &eq195_branch_derivatives,
            self.multiplicity,
        );
    }
}
