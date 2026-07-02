use super::*;

#[derive(Debug, Default, Clone)]
pub struct Inductors {
    pub names: Vec<String>,
    pub node_pos: Vec<NodeId>,
    pub node_neg: Vec<NodeId>,
    pub branch_indices: Vec<NodeId>,
    pub inductances: Vec<Value>,
    /// Previous current (t - dt) for companion model
    pub i_prev: Vec<Value>,
    /// Current from 2 steps ago (t - 2*dt) for Gear2/BDF2
    pub i_prev_prev: Vec<Value>,
    /// Previous voltage for companion model
    pub v_prev: Vec<Value>,
    /// Initial condition current (IC=)
    pub ic: Vec<Option<Value>>,
}

impl Inductors {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn add(
        &mut self,
        name: String,
        node_pos: NodeId,
        node_neg: NodeId,
        branch_idx: NodeId,
        inductance: Value,
    ) {
        self.names.push(name);
        self.node_pos.push(node_pos);
        self.node_neg.push(node_neg);
        self.branch_indices.push(branch_idx);
        self.inductances.push(inductance);
        self.i_prev.push(0.0);
        self.i_prev_prev.push(0.0);
        self.v_prev.push(0.0);
        self.ic.push(None);
    }

    pub fn add_with_ic(
        &mut self,
        name: String,
        node_pos: NodeId,
        node_neg: NodeId,
        branch_idx: NodeId,
        inductance: Value,
        ic: Value,
    ) {
        self.names.push(name);
        self.node_pos.push(node_pos);
        self.node_neg.push(node_neg);
        self.branch_indices.push(branch_idx);
        self.inductances.push(inductance);
        self.i_prev.push(ic); // Initialize i_prev to IC
        self.i_prev_prev.push(ic); // Initialize i_prev_prev to IC as well
        self.v_prev.push(0.0);
        self.ic.push(Some(ic));
    }

    /// Apply initial conditions to i_prev
    pub fn apply_initial_conditions(&mut self) {
        for (i, ic) in self.ic.iter().enumerate() {
            if let Some(current) = ic {
                self.i_prev[i] = *current;
            }
        }
    }

    pub fn len(&self) -> usize {
        self.names.len()
    }

    pub fn is_empty(&self) -> bool {
        self.names.is_empty()
    }

    /// Get equivalent resistance for trapezoidal integration
    #[inline]
    pub fn req(&self, idx: usize, dt: Value) -> Value {
        2.0 * self.inductances[idx] / dt
    }

    /// Stamp inductors for DC operating point.
    ///
    /// At DC, an ideal inductor is a short circuit:
    /// V(np) - V(nn) = 0 with unconstrained branch current.
    #[inline]
    pub fn stamp_dc_short_direct(
        &self,
        matrix: &mut StaticMatrix,
        rhs: &mut [Value],
        num_nodes: usize,
    ) {
        for i in 0..self.names.len() {
            let np = self.node_pos[i];
            let nn = self.node_neg[i];
            let br_ordinal = self.branch_indices[i];
            let br = num_nodes + br_ordinal;

            if np > 0 {
                matrix.add(br - 1, np - 1, 1.0);
                matrix.add(np - 1, br - 1, 1.0);
            }
            if nn > 0 {
                matrix.add(br - 1, nn - 1, -1.0);
                matrix.add(nn - 1, br - 1, -1.0);
            }

            rhs[br - 1] = 0.0;
        }
    }

    /// Stamp inductors for the t=0 transient operating point.
    ///
    /// Xyce treats `IC=` on an inductor as a branch-current constraint during
    /// the operating-point solve that seeds transient analysis: the specified
    /// current participates in KCL, and the inductor voltage is left
    /// unconstrained. Inductors without `IC=` retain ordinary DC-short
    /// operating-point semantics.
    #[inline]
    pub fn stamp_transient_operating_point_direct(
        &self,
        matrix: &mut StaticMatrix,
        rhs: &mut [Value],
        num_nodes: usize,
    ) {
        for i in 0..self.names.len() {
            let np = self.node_pos[i];
            let nn = self.node_neg[i];
            let br_ordinal = self.branch_indices[i];
            let br = num_nodes + br_ordinal;

            if let Some(current) = self.ic[i] {
                if np > 0 {
                    rhs[np - 1] -= current;
                }
                if nn > 0 {
                    rhs[nn - 1] += current;
                }
                matrix.add(br - 1, br - 1, 1.0);
                rhs[br - 1] = current;
                continue;
            }

            if np > 0 {
                matrix.add(br - 1, np - 1, 1.0);
                matrix.add(np - 1, br - 1, 1.0);
            }
            if nn > 0 {
                matrix.add(br - 1, nn - 1, -1.0);
                matrix.add(nn - 1, br - 1, -1.0);
            }

            rhs[br - 1] = 0.0;
        }
    }

    /// Stamp inductors for DC operating point (triplet path).
    #[inline]
    pub fn stamp_dc_short(&self, matrix: &mut TripletMatrix, rhs: &mut [Value], num_nodes: usize) {
        for i in 0..self.names.len() {
            let np = self.node_pos[i];
            let nn = self.node_neg[i];
            let br_ordinal = self.branch_indices[i];
            let br = num_nodes + br_ordinal;

            if np > 0 {
                matrix.push(br - 1, np - 1, 1.0);
                matrix.push(np - 1, br - 1, 1.0);
            }
            if nn > 0 {
                matrix.push(br - 1, nn - 1, -1.0);
                matrix.push(nn - 1, br - 1, -1.0);
            }

            rhs[br - 1] = 0.0;
        }
    }

    /// Stamp all inductors for transient analysis using optimized direct stamping
    ///
    /// This is the unified stamping method for both StaticMatrix (direct) and TripletMatrix
    #[inline]
    pub fn stamp_transient_companion(
        &self,
        matrix: &mut StaticMatrix,
        rhs: &mut [Value],
        dt: Value,
        coeff: &CompanionCoefficients,
        num_nodes: usize,
    ) {
        for i in 0..self.names.len() {
            let np = self.node_pos[i];
            let nn = self.node_neg[i];
            let br_ordinal = self.branch_indices[i];
            let br = num_nodes + br_ordinal;

            let r_eq = coeff.inductor_req(self.inductances[i], dt);
            let v_eq = coeff.inductor_veq(
                self.inductances[i],
                dt,
                self.i_prev[i],
                self.i_prev_prev[i],
                self.v_prev[i],
            );

            // MNA stamp for inductor companion model (V-source branch).
            // Branch row: v(np) - v(nn) - r_eq*i_{n+1} = -v_eq, the dual of the
            // capacitor companion (see CompanionCoefficients::inductor_veq for
            // the per-method expansion). The negation here is load-bearing:
            // stamping +v_eq flips the history feedback sign and the companion
            // recursion diverges on any L in transient.
            if np > 0 {
                matrix.add(br - 1, np - 1, 1.0);
                matrix.add(np - 1, br - 1, 1.0);
            }
            if nn > 0 {
                matrix.add(br - 1, nn - 1, -1.0);
                matrix.add(nn - 1, br - 1, -1.0);
            }
            matrix.add(br - 1, br - 1, -r_eq);
            rhs[br - 1] = -v_eq;
        }
    }

    /// Update inductor state after a successful timestep
    pub fn update_state(
        &mut self,
        solution: &[Value],
        num_nodes: usize,
        _dt: Value,
        _method: IntegrationMethod,
    ) {
        for i in 0..self.names.len() {
            let np = self.node_pos[i];
            let nn = self.node_neg[i];
            let br_idx = num_nodes + self.branch_indices[i] - 1;

            let v_curr = if np == 0 { 0.0 } else { solution[np - 1] }
                - if nn == 0 { 0.0 } else { solution[nn - 1] };
            let i_curr = solution[br_idx];

            // Advance history
            self.i_prev_prev[i] = self.i_prev[i];
            self.i_prev[i] = i_curr;
            self.v_prev[i] = v_curr;
        }
    }

    /// Stamp all inductors (legacy TripletMatrix support)
    #[inline]
    pub fn stamp_all(
        &self,
        matrix: &mut TripletMatrix,
        rhs: &mut [Value],
        num_nodes: usize,
        dt: Value,
    ) {
        for i in 0..self.names.len() {
            let np = self.node_pos[i];
            let nn = self.node_neg[i];
            let br_ordinal = self.branch_indices[i];
            let br = num_nodes + br_ordinal;

            let req = 2.0 * self.inductances[i] / dt;
            // Trapezoidal branch row: v - req*i_{n+1} = -(req*i_n + v_n);
            // same sign convention as stamp_transient_companion above.
            let veq = req * self.i_prev[i] + self.v_prev[i];

            if np > 0 {
                matrix.push(br - 1, np - 1, 1.0);
                matrix.push(np - 1, br - 1, 1.0);
            }
            if nn > 0 {
                matrix.push(br - 1, nn - 1, -1.0);
                matrix.push(nn - 1, br - 1, -1.0);
            }
            matrix.push(br - 1, br - 1, -req);
            rhs[br - 1] = -veq;
        }
    }

    /// Update state after timestep using trapezoidal rule
    pub fn step(&mut self, voltages: &[Value], currents: &[Value], dt: Value) {
        for i in 0..self.names.len() {
            let np = self.node_pos[i];
            let nn = self.node_neg[i];
            let br = self.branch_indices[i];

            let v_pos = if np == 0 {
                0.0
            } else {
                voltages.get(np - 1).copied().unwrap_or(0.0)
            };
            let v_neg = if nn == 0 {
                0.0
            } else {
                voltages.get(nn - 1).copied().unwrap_or(0.0)
            };
            let v = v_pos - v_neg;

            // Update current: i = i_prev + (dt / 2L) * (v + v_prev)
            let l = self.inductances[i];
            self.i_prev[i] += (dt / (2.0 * l)) * (v + self.v_prev[i]);
            self.v_prev[i] = v;

            // Also get branch current from solution for accuracy
            if br > 0
                && let Some(&i_br) = currents.get(br - 1)
            {
                self.i_prev[i] = i_br;
            }
        }
    }
}

//=============================================================================
// Nonlinear Device Storage (SoA)
//=============================================================================
