//! Numerical fingerprints for the generated Verilog-A built-ins.
//!
//! The backend that produced a Jacobian is the worst possible judge of whether
//! that Jacobian is right. This module supplies the two things a compact-model
//! backend rewrite needs and that no unit test in the crate currently provides:
//! a reproducible per-model record of what the devices actually compute, and an
//! oracle for the derivative half of it that does not share a line of code with
//! the chain rule under test.
//!
//! ## What a record contains
//!
//! [`GoldenRecord`] is the complete observable result of one device evaluation
//! at one bias: the conductance stamps, the right-hand side, the reactive
//! (charge) stamps, and the noise power spectral densities. Nothing is rounded,
//! summarized, or hashed — a rewrite has to be diffable against this, and a
//! digest only tells you that something moved.
//!
//! ## The derivative oracle
//!
//! A generated stamp writes `J = dI/dV` into the matrix and
//! `J*V - I` into the right-hand side, so the device's own current vector is
//! recoverable from its stamp alone:
//!
//! ```text
//! I(V) = J(V) * V - rhs(V)
//! ```
//!
//! [`GoldenHarness::device_currents`] does exactly that, which makes `I` an
//! ordinary function of the unknowns that can be differentiated numerically
//! without the backend's cooperation. [`GoldenHarness::jacobian_audit`] then
//! compares the stamped `J` against a fourth-order central difference of `I`.
//!
//! Finite differences on a compact model are not automatically trustworthy —
//! junction exponentials have derivatives large enough that no step size is
//! simultaneously free of truncation and round-off error. Rather than assert a
//! tolerance and hope, the audit evaluates each entry at two step sizes and
//! reports an entry as checked only where the two agree
//! ([`StampAuditEntry::converged`]). Entries where the difference itself has
//! not converged are reported as unchecked rather than silently passed, and the
//! checked fraction is part of the audit result — a model whose Jacobian cannot
//! be verified is a finding, not a blank.
//!
//! ## Determinism
//!
//! Bias points come from a counter-based generator seeded by the model name, so
//! the same model draws the same points on every machine and every run, and
//! adding a model never perturbs another model's points.

use crate::Value;
use crate::device::veriloga_generated::{
    BuiltinVerilogAInstance, GeneratedAnalysisKind, GeneratedDdtCoefficients,
    GeneratedEvaluationError, GeneratedSimulationParameters, builtins,
};
use crate::solver::{ComplexMatrix, StaticMatrix};

/// Format version of a captured record set.
///
/// Bumped when the shape of what is captured changes, so a stale fixture fails
/// loudly instead of comparing two different things.
pub const GOLDEN_FORMAT_VERSION: u32 = 1;

/// Node bias range for generated probe points, in volts.
///
/// Wide enough to move every model off its exact-zero fast paths and to put
/// junctions on both sides of turn-on, narrow enough that no shipped model
/// overflows its exponentials at the default card.
const PROBE_NODE_MIN: Value = -0.55;
const PROBE_NODE_MAX: Value = 0.85;

/// Branch-unknown seed magnitude, in amperes.
const PROBE_BRANCH_CURRENT: Value = 1.0e-6;

/// Scale a branch-current perturbation is measured against, in amperes.
///
/// Not the seed magnitude. A branch unknown enters a node's current equation
/// with a coefficient near one, so differencing it against a seed of 1e-6 A
/// perturbs a node current by ~1e-9 A — often twelve orders below the current
/// already flowing there, which is pure cancellation and reports as a wildly
/// wrong derivative rather than as a small one. Perturbing on a milliamp scale
/// resolves against the rest of the row.
const PROBE_BRANCH_SCALE: Value = 1.0e-3;

/// Relative steps tried, coarsest first.
///
/// A fourth-order central rule has truncation error `O(h^4)` and round-off
/// `O(eps/h)`; the two cross near `eps^(1/5)` ~ 7e-4 for well-scaled operands.
/// No single step suits a whole compact model, though — a linear branch wants
/// the largest step that avoids round-off while a junction near turn-on wants
/// the smallest step that avoids truncation, and both appear in the same
/// matrix. Each entry is therefore judged on whichever adjacent pair of these
/// agrees best, which is a cheap stand-in for Ridders' method and costs one
/// extra sweep.
const AUDIT_STEP_LADDER: [Value; 3] = [1.0e-3, 5.0e-4, 2.5e-4];

/// Agreement between adjacent steps, relative to the entry scale, below which
/// the difference is treated as converged and the stamp is judged tightly.
const AUDIT_CONVERGENCE_TOLERANCE: Value = 1.0e-6;

/// Above this spread the difference carries no usable information and the entry
/// is reported as a coverage gap rather than compared.
const AUDIT_MAX_USABLE_UNCERTAINTY: Value = 1.0e-2;

/// How far outside its own error bar a difference may sit before disagreement
/// counts as real rather than as noise in the measurement.
const AUDIT_UNCERTAINTY_SLACK: Value = 10.0;

/// An entry is judged against the larger of its own magnitude and this fraction
/// of the largest entry in the same block.
///
/// Judging every entry against itself alone is the obvious approach and it is
/// wrong: a conductance of 1e-18 sitting in a matrix whose largest entry is
/// 1e-3 contributes nothing a solver can observe, but a per-entry relative test
/// demands the difference resolve it to six figures, which no finite difference
/// on a stiff junction can do. Such entries then report as unverifiable and
/// drag the checked fraction down for no reason. Scaling the floor to the block
/// makes the criterion mean "verified to a precision the solver would notice".
const AUDIT_BLOCK_RELATIVE_FLOOR: Value = 1.0e-9;

/// Absolute floor, for the case where the entire block is zero.
const AUDIT_ABSOLUTE_FLOOR: Value = 1.0e-30;

/// Companion scales the charge oracle drives `ddt` with, in inverse seconds.
///
/// A charge has to be recovered by subtracting two current vectors, and the
/// charge term is `scale * q` while the conduction term is whatever the device
/// conducts. Too small a scale and the subtraction is pure cancellation: a
/// picocoulomb against a milliamp at `scale = 1` leaves four significant digits.
/// These correspond to timesteps of 100 ps and 1 ps, which put `scale * q` on
/// the same order as the conduction current for a typical compact model and
/// recover the charge to nearly full precision.
///
/// Two of them, two orders apart, because agreement between them is the test
/// that `ddt` enters the residual linearly — see [`GoldenHarness::charge_audit`].
const CHARGE_RECOVERY_SCALES: [Value; 2] = [1.0e10, 1.0e12];

/// Disagreement between the two recovery scales, relative to the recovered
/// charge, above which that row's charge is treated as unrecoverable.
const CHARGE_LINEARITY_TOLERANCE: Value = 1.0e-7;

#[derive(Debug, Clone, PartialEq)]
pub enum GoldenError {
    UnknownModel(String),
    Setup { model_name: String, detail: String },
    Evaluation { model_name: String, detail: String },
    NoUnknowns { model_name: String },
}

impl std::fmt::Display for GoldenError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::UnknownModel(name) => {
                write!(f, "'{name}' is not a compiled-in generated built-in")
            }
            Self::Setup { model_name, detail } => {
                write!(f, "{model_name}: golden harness setup failed: {detail}")
            }
            Self::Evaluation { model_name, detail } => {
                write!(f, "{model_name}: evaluation failed: {detail}")
            }
            Self::NoUnknowns { model_name } => {
                write!(f, "{model_name}: model exposes no unknowns to probe")
            }
        }
    }
}

impl std::error::Error for GoldenError {}

/// One captured device evaluation.
///
/// Matrix blocks are dense row-major over `size * size` because the whole point
/// is to notice an entry that used to be structurally absent and now is not,
/// which a sparse triplet list would hide.
///
/// `capacitance` is the reactive stamp taken at `omega = 1`, so the recorded
/// numbers are the charge derivatives themselves rather than a frequency
/// scaling of them.
#[derive(Debug, Clone, PartialEq)]
pub struct GoldenRecord {
    pub jacobian: Vec<Value>,
    pub rhs: Vec<Value>,
    pub capacitance: Vec<Value>,
    pub noise: Vec<GoldenNoiseSample>,
}

#[derive(Debug, Clone, PartialEq)]
pub struct GoldenNoiseSample {
    pub mechanism: &'static str,
    pub active: bool,
    pub psd: Value,
    pub exponent: Option<Value>,
}

/// One entry of a stamped-versus-differenced comparison.
///
/// The same shape serves the conduction Jacobian and the reactive block: both
/// are a matrix the device stamped, held against a numerical derivative of a
/// quantity recovered from the device's own residual.
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct StampAuditEntry {
    pub row: u32,
    pub col: u32,
    /// What the device stamped.
    pub stamped: Value,
    /// Best fourth-order central difference found on the step ladder.
    pub numeric: Value,
    /// How much the best-agreeing pair of steps disagreed, relative.
    ///
    /// This is the oracle's own error bar. Demanding that `stamped` match
    /// `numeric` more closely than this would be asking the difference to be
    /// more precise than it is, and every "failure" so produced would be an
    /// artifact.
    pub uncertainty: Value,
    /// `|stamped - numeric|` relative to the larger magnitude, floored.
    pub relative_error: Value,
}

impl StampAuditEntry {
    /// Whether the difference is precise enough to judge the stamp tightly.
    pub fn converged(&self) -> bool {
        self.uncertainty <= AUDIT_CONVERGENCE_TOLERANCE
    }

    /// Whether the difference is too imprecise to say anything at all.
    pub fn unverifiable(&self) -> bool {
        !(self.uncertainty <= AUDIT_MAX_USABLE_UNCERTAINTY)
    }

    /// Agreement this entry must show, given how precise the oracle managed to
    /// be here.
    ///
    /// Tight where the difference converged; widened in proportion to the
    /// oracle's own spread where it did not, because an entry cannot be held to
    /// a precision the measurement never had.
    pub fn required_tolerance(&self, base: Value) -> Value {
        base.max(self.uncertainty * AUDIT_UNCERTAINTY_SLACK)
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct StampAudit {
    pub model_name: &'static str,
    pub entries: Vec<StampAuditEntry>,
    /// Magnitude below which an entry cannot influence a solve of this block.
    ///
    /// Derived from the largest entry the device stamped, so it scales with the
    /// model rather than being a universal constant that would be meaningless
    /// across a resistor and a power MOSFET.
    pub significance_floor: Value,
}

impl StampAudit {
    /// Entries the difference resolved tightly.
    pub fn checked(&self) -> impl Iterator<Item = &StampAuditEntry> {
        self.entries.iter().filter(|entry| entry.converged())
    }

    /// Entries carrying enough information to be compared at all.
    pub fn comparable(&self) -> impl Iterator<Item = &StampAuditEntry> {
        self.entries.iter().filter(|entry| !entry.unverifiable())
    }

    /// Fraction of entries the difference resolved tightly.
    ///
    /// Informational. Most of the shortfall in a stiff model is entries too
    /// small to matter, which is why [`Self::unverified_significant`] rather
    /// than this is the thing worth gating on.
    pub fn checked_fraction(&self) -> f64 {
        if self.entries.is_empty() {
            return 1.0;
        }
        self.checked().count() as f64 / self.entries.len() as f64
    }

    /// Entries the oracle learned nothing about that are nonetheless large
    /// enough to change a solve, largest first.
    ///
    /// This is the real coverage question. A finite difference will always be
    /// imprecise on some entries of a compact model; what matters is whether it
    /// was so imprecise on an entry the solver would notice that the stamp went
    /// entirely unexamined.
    pub fn unverified_significant(&self) -> Vec<StampAuditEntry> {
        let mut significant: Vec<StampAuditEntry> = self
            .entries
            .iter()
            .filter(|entry| {
                entry.unverifiable() && entry.stamped.abs() > self.significance_floor
            })
            .copied()
            .collect();
        significant.sort_by(|left, right| right.stamped.abs().total_cmp(&left.stamped.abs()));
        significant
    }

    /// Worst relative disagreement among the entries the difference resolved
    /// tightly.
    pub fn worst_relative_error(&self) -> Value {
        self.checked()
            .map(|entry| entry.relative_error)
            .fold(0.0, Value::max)
    }

    /// Entries disagreeing by more than the oracle's precision there allows,
    /// worst first.
    ///
    /// `base` is the tolerance demanded where the difference converged; entries
    /// the difference resolved less well are held to their own error bar
    /// instead, so a stiff junction cannot manufacture a failure.
    pub fn failures(&self, base: Value) -> Vec<StampAuditEntry> {
        let mut failures: Vec<StampAuditEntry> = self
            .comparable()
            .copied()
            .filter(|entry| entry.relative_error > entry.required_tolerance(base))
            .collect();
        failures.sort_by(|left, right| right.relative_error.total_cmp(&left.relative_error));
        failures
    }
}

/// A model instantiated once, reusable across bias points.
///
/// Setup is not cheap — the matrix pattern is dense over every unknown and the
/// static stamp cache is linked against it — and a sweep re-does none of it.
pub struct GoldenHarness {
    model_name: &'static str,
    instance: BuiltinVerilogAInstance,
    matrix: StaticMatrix,
    rhs: Vec<Value>,
    node_count: usize,
    branch_count: usize,
}

impl GoldenHarness {
    /// Instantiate `model_name` with `overrides` applied as instance parameters.
    ///
    /// Every terminal is left ungrounded so the Jacobian block is structurally
    /// dense. That is the worst case, and more importantly it is the same case
    /// for every model and every revision, which is what makes captured records
    /// comparable.
    pub fn new(
        model_name: &str,
        overrides: &[(String, Value)],
    ) -> Result<Self, GoldenError> {
        let model_name = builtins::builtin_names()
            .iter()
            .find(|name| name.eq_ignore_ascii_case(model_name))
            .copied()
            .ok_or_else(|| GoldenError::UnknownModel(model_name.to_string()))?;

        let setup_error = |detail: String| GoldenError::Setup {
            model_name: model_name.to_string(),
            detail,
        };

        let node_count = builtins::total_node_count(model_name)
            .ok_or_else(|| setup_error("model exposes no node count".to_string()))?;
        let branch_count = builtins::branch_count(model_name).unwrap_or(0);
        let size = node_count + branch_count;
        if size == 0 {
            return Err(GoldenError::NoUnknowns {
                model_name: model_name.to_string(),
            });
        }

        // Rows 0..node_count are the ungrounded nodes; branch unknowns follow,
        // matching `GeneratedStaticStampCache`'s index convention.
        let nodes: Vec<usize> = (1..=node_count).collect();
        let branches: Vec<usize> = (1..=branch_count).collect();

        let mut triplets = Vec::with_capacity(size * size);
        for row in 0..size {
            for col in 0..size {
                triplets.push((row, col, 0.0));
            }
        }
        let matrix = StaticMatrix::from_triplets(size, size, &triplets)
            .map_err(|error| setup_error(format!("matrix assembly failed: {error}")))?;

        let mut instance = BuiltinVerilogAInstance::standalone(
            model_name,
            format!("xgolden_{model_name}"),
            nodes,
            branches,
            crate::constants::TEMP_REFERENCE,
            overrides,
        )
        .map_err(setup_error)?;
        instance.link_static_stamps(&matrix, node_count);

        Ok(Self {
            model_name,
            instance,
            matrix,
            rhs: vec![0.0; size],
            node_count,
            branch_count,
        })
    }

    pub fn model_name(&self) -> &'static str {
        self.model_name
    }

    pub fn node_count(&self) -> usize {
        self.node_count
    }

    pub fn branch_count(&self) -> usize {
        self.branch_count
    }

    pub fn size(&self) -> usize {
        self.node_count + self.branch_count
    }

    /// Deterministic probe points for this model.
    ///
    /// Seeded by the model name so points are stable across runs and machines,
    /// and so adding a model cannot shift the points another model draws.
    pub fn probe_points(&self, count: usize) -> Vec<Vec<Value>> {
        let size = self.size();
        let mut points = Vec::with_capacity(count + 1);

        // Equilibrium first: every model must be evaluable at zero bias, and it
        // is the one point whose expected behavior is known by inspection.
        points.push(vec![0.0; size]);

        let mut state = seed_for(self.model_name);
        for _ in 1..count.max(1) {
            let mut point = Vec::with_capacity(size);
            for index in 0..size {
                let unit = next_unit(&mut state);
                if index < self.node_count {
                    point.push(PROBE_NODE_MIN + unit * (PROBE_NODE_MAX - PROBE_NODE_MIN));
                } else {
                    // Branch unknowns are currents; span both directions across
                    // a decade so a branch equation is exercised, not pinned.
                    point.push(PROBE_BRANCH_CURRENT * (2.0 * unit - 1.0) * 10.0);
                }
            }
            points.push(point);
        }
        points
    }

    /// Stamp once at `unknowns` and return everything observable.
    pub fn evaluate(&mut self, unknowns: &[Value]) -> Result<GoldenRecord, GoldenError> {
        let (jacobian, rhs) = self.stamp_dense(unknowns)?;
        let capacitance = self.stamp_capacitance_dense(unknowns)?;
        let noise = self.noise_samples(unknowns)?;
        Ok(GoldenRecord {
            jacobian,
            rhs,
            capacitance,
            noise,
        })
    }

    /// The device's own current vector at `unknowns`, recovered as `J*V - rhs`.
    ///
    /// This is the function the derivative oracle differentiates. It is a pure
    /// function of the unknowns only because the probe stamp runs with Newton
    /// limiting disabled; with limiting on, the stamp depends on the previous
    /// iterate and no numerical derivative of it would mean anything.
    pub fn device_currents(&mut self, unknowns: &[Value]) -> Result<Vec<Value>, GoldenError> {
        let size = self.size();
        let (jacobian, rhs) = self.stamp_dense(unknowns)?;
        let mut currents = Vec::with_capacity(size);
        for row in 0..size {
            let mut accumulated = 0.0;
            for col in 0..size {
                accumulated += jacobian[row * size + col] * unknowns[col];
            }
            currents.push(accumulated - rhs[row]);
        }
        Ok(currents)
    }

    /// Compare the stamped Jacobian against a numerical derivative of the
    /// current vector, at two step sizes.
    pub fn jacobian_audit(&mut self, unknowns: &[Value]) -> Result<StampAudit, GoldenError> {
        let (stamped, _) = self.stamp_dense(unknowns)?;

        let mut ladder = Vec::with_capacity(AUDIT_STEP_LADDER.len());
        for step in AUDIT_STEP_LADDER {
            ladder.push(self.difference_jacobian(unknowns, step)?);
        }

        Ok(audit_against_ladder(
            self.model_name,
            self.size(),
            &stamped,
            &ladder,
        ))
    }

    /// Compare the stamped reactive block against a numerical derivative of the
    /// charge vector, recovered from the residual.
    ///
    /// The reactive stamp is produced by the same chain rule as the conduction
    /// stamp and shares the whole differentiated body with it, so comparing the
    /// two says nothing. What does not share code with either is the *residual*:
    /// a `ddt` reaches it through a companion form that this harness controls
    /// the coefficients of. Drive the device once with `ddt` inert and once with
    /// its derivative scale set to `k` and every history weight at zero, and the
    /// operator returns exactly `k * q`, so
    ///
    /// ```text
    /// q(V) = (I_active(V) - I_inert(V)) / k
    /// ```
    ///
    /// is an ordinary function of the unknowns recovered through
    /// [`Self::device_currents`] — the primal path — with no derivative
    /// information involved. Differencing it gives `dq/dV` to hold the reactive
    /// stamp against.
    ///
    /// The recovery assumes the operator's result enters the residual linearly,
    /// which is how a compact model contributes a charge and is not something
    /// the language guarantees: a body is free to compute `f(ddt(q))`. Rather
    /// than assume it, the charge is recovered at two scales two orders apart
    /// and rows where the two disagree are reported as coverage gaps — the same
    /// discipline the step ladder applies to the difference itself.
    pub fn charge_audit(&mut self, unknowns: &[Value]) -> Result<StampAudit, GoldenError> {
        let size = self.size();
        let stamped = self.stamp_capacitance_dense(unknowns)?;

        let [coarse_scale, fine_scale] = CHARGE_RECOVERY_SCALES;
        let coarse = self.device_charges(unknowns, coarse_scale)?;
        let fine = self.device_charges(unknowns, fine_scale)?;

        let charge_floor = fine
            .iter()
            .chain(coarse.iter())
            .filter(|value| value.is_finite())
            .fold(0.0, |worst: Value, value| worst.max(value.abs()))
            * AUDIT_BLOCK_RELATIVE_FLOOR;
        let linear: Vec<bool> = (0..size)
            .map(|row| {
                let (left, right) = (coarse[row], fine[row]);
                left.is_finite()
                    && right.is_finite()
                    && (left - right).abs()
                        / magnitude_scale(left, right, charge_floor.max(AUDIT_ABSOLUTE_FLOOR))
                        <= CHARGE_LINEARITY_TOLERANCE
            })
            .collect();

        let mut ladder = Vec::with_capacity(AUDIT_STEP_LADDER.len());
        for step in AUDIT_STEP_LADDER {
            ladder.push(self.difference_charges(unknowns, step, fine_scale)?);
        }

        let mut audit = audit_against_ladder(self.model_name, size, &stamped, &ladder);

        // A row whose charge could not be recovered has no oracle at all. Its
        // entries are blanked rather than compared, which reports them through
        // `unverified_significant` as the coverage gaps they are instead of
        // manufacturing failures against a meaningless number.
        for entry in &mut audit.entries {
            if !linear[entry.row as usize] {
                entry.numeric = Value::NAN;
                entry.uncertainty = Value::INFINITY;
                entry.relative_error = 0.0;
            }
        }
        Ok(audit)
    }

    /// The charge the device stores at `unknowns`, at the oracle's own scale.
    ///
    /// The quantity the reactive stamp claims to be the derivative of, obtained
    /// without consulting the reactive stamp.
    pub fn stored_charges(&mut self, unknowns: &[Value]) -> Result<Vec<Value>, GoldenError> {
        self.device_charges(unknowns, CHARGE_RECOVERY_SCALES[1])
    }

    /// The charge the device stores at `unknowns`, recovered from its residual.
    ///
    /// `scale` is the companion derivative weight `ddt` is driven with; see
    /// [`Self::charge_audit`] for why the answer should not depend on it.
    pub fn device_charges(
        &mut self,
        unknowns: &[Value],
        scale: Value,
    ) -> Result<Vec<Value>, GoldenError> {
        // Inert first. `rspice_eval_ddt` latches its history on the inert path,
        // so running it first leaves every slot initialised and makes the active
        // pass depend on nothing but the value it is handed.
        self.set_ddt_scale(None);
        let inert = self.device_currents(unknowns)?;
        self.set_ddt_scale(Some(scale));
        let active = self.device_currents(unknowns)?;
        self.set_ddt_scale(None);

        Ok(inert
            .iter()
            .zip(active.iter())
            .map(|(inert, active)| (active - inert) / scale)
            .collect())
    }

    /// Drive `ddt` with a bare derivative weight, or make it inert.
    ///
    /// Every history weight is zero, so the operator reduces to `scale * value`
    /// with no dependence on what a previous timestep left behind. The timestep
    /// is zero for the same reason: it is what `idt` integrates over, and a zero
    /// step holds an integral at its accepted value across both passes so it
    /// subtracts out instead of appearing as charge.
    fn set_ddt_scale(&mut self, scale: Option<Value>) {
        let coefficients = match scale {
            Some(derivative_scale) => GeneratedDdtCoefficients {
                active: true,
                derivative_scale,
                previous_value_scale: 0.0,
                older_value_scale: 0.0,
                previous_derivative_scale: 0.0,
            },
            None => GeneratedDdtCoefficients::inactive(),
        };
        self.instance.set_timepoint(0.0, 0.0, coefficients);
    }

    /// Fourth-order central difference of the current vector, dense row-major.
    ///
    /// `(-f(x+2h) + 8f(x+h) - 8f(x-h) + f(x-2h)) / (12h)`, with `h` scaled to
    /// the operand so a millivolt node and a microamp branch get steps of
    /// comparable relative size.
    fn difference_jacobian(
        &mut self,
        unknowns: &[Value],
        relative_step: Value,
    ) -> Result<Vec<Value>, GoldenError> {
        let size = self.size();
        let mut jacobian = vec![0.0; size * size];
        let mut perturbed = unknowns.to_vec();

        for col in 0..size {
            let scale = if col < self.node_count {
                unknowns[col].abs().max(1.0)
            } else {
                unknowns[col].abs().max(PROBE_BRANCH_SCALE)
            };
            let step = relative_step * scale;

            let mut sampled = [Vec::new(), Vec::new(), Vec::new(), Vec::new()];
            for (slot, multiple) in [-2.0, -1.0, 1.0, 2.0].into_iter().enumerate() {
                perturbed[col] = unknowns[col] + multiple * step;
                sampled[slot] = self.device_currents(&perturbed)?;
            }
            perturbed[col] = unknowns[col];

            for row in 0..size {
                jacobian[row * size + col] = (-sampled[3][row] + 8.0 * sampled[2][row]
                    - 8.0 * sampled[1][row]
                    + sampled[0][row])
                    / (12.0 * step);
            }
        }

        Ok(jacobian)
    }

    /// The same fourth-order rule applied to the recovered charge vector.
    fn difference_charges(
        &mut self,
        unknowns: &[Value],
        relative_step: Value,
        scale: Value,
    ) -> Result<Vec<Value>, GoldenError> {
        let size = self.size();
        let mut derivatives = vec![0.0; size * size];
        let mut perturbed = unknowns.to_vec();

        for col in 0..size {
            let operand_scale = if col < self.node_count {
                unknowns[col].abs().max(1.0)
            } else {
                unknowns[col].abs().max(PROBE_BRANCH_SCALE)
            };
            let step = relative_step * operand_scale;

            let mut sampled = [Vec::new(), Vec::new(), Vec::new(), Vec::new()];
            for (slot, multiple) in [-2.0, -1.0, 1.0, 2.0].into_iter().enumerate() {
                perturbed[col] = unknowns[col] + multiple * step;
                sampled[slot] = self.device_charges(&perturbed, scale)?;
            }
            perturbed[col] = unknowns[col];

            for row in 0..size {
                derivatives[row * size + col] = (-sampled[3][row] + 8.0 * sampled[2][row]
                    - 8.0 * sampled[1][row]
                    + sampled[0][row])
                    / (12.0 * step);
            }
        }

        Ok(derivatives)
    }

    fn stamp_dense(&mut self, unknowns: &[Value]) -> Result<(Vec<Value>, Vec<Value>), GoldenError> {
        self.matrix.clear_values();
        self.rhs.fill(0.0);
        self.instance
            .stamp_probe(
                &mut self.matrix,
                &mut self.rhs,
                unknowns,
                self.node_count,
                GeneratedAnalysisKind::Dc,
                GeneratedSimulationParameters::new(),
            )
            .map_err(|error| self.evaluation_error(error))?;
        Ok((self.dense_matrix(), self.rhs.clone()))
    }

    /// Reactive stamp at `omega = 1`, so the imaginary block is `dQ/dV` itself.
    ///
    /// A generated `stamp_reactive` writes cached charge derivatives that the
    /// conduction stamp computed; it does not re-evaluate the body. So the
    /// conduction stamp is run here at the same bias rather than left to the
    /// caller — otherwise the block returned is the reactive stamp of whatever
    /// bias happened to be evaluated last, which is a silent wrong answer.
    fn stamp_capacitance_dense(&mut self, unknowns: &[Value]) -> Result<Vec<Value>, GoldenError> {
        let size = self.size();
        self.stamp_dense(unknowns)?;
        let mut complex = ComplexMatrix::from_real_structure(&self.matrix);
        complex.clear_values();
        self.instance
            .stamp_reactive(
                &mut complex,
                unknowns,
                self.node_count,
                1.0,
                GeneratedSimulationParameters::new(),
            )
            .map_err(|error| self.evaluation_error(error))?;

        let rows = complex.to_dense_imag();
        let mut dense = vec![0.0; size * size];
        for (row, values) in rows.iter().enumerate().take(size) {
            for (col, value) in values.iter().enumerate().take(size) {
                dense[row * size + col] = *value;
            }
        }
        Ok(dense)
    }

    fn noise_samples(&mut self, unknowns: &[Value]) -> Result<Vec<GoldenNoiseSample>, GoldenError> {
        let evaluated = self
            .instance
            .evaluate_noise_sources(
                unknowns,
                self.node_count,
                GeneratedSimulationParameters::new(),
            )
            .map_err(|error| GoldenError::Evaluation {
                model_name: self.model_name.to_string(),
                detail: error.to_string(),
            })?;
        let descriptors = self.instance.noise_descriptors();
        Ok(evaluated
            .iter()
            .enumerate()
            .map(|(index, source)| GoldenNoiseSample {
                mechanism: descriptors
                    .get(index)
                    .map_or("<missing descriptor>", |descriptor| descriptor.mechanism),
                active: source.evaluation.active,
                psd: source.evaluation.psd,
                exponent: source.evaluation.exponent,
            })
            .collect())
    }

    fn dense_matrix(&mut self) -> Vec<Value> {
        let size = self.size();
        let mut dense = vec![0.0; size * size];
        for row in 0..size {
            for col in 0..size {
                if let Some(index) = self.matrix.get_index(row, col) {
                    dense[row * size + col] = self.matrix.values_mut()[index.0];
                }
            }
        }
        dense
    }

    fn evaluation_error(&self, error: GeneratedEvaluationError) -> GoldenError {
        GoldenError::Evaluation {
            model_name: self.model_name.to_string(),
            detail: error.to_string(),
        }
    }
}

/// Larger of two magnitudes, floored at the block-derived significance level.
/// Hold a stamped block against a ladder of numerical estimates of it.
///
/// Shared by both oracles because the judgement is the same one in both cases:
/// what varies between them is only which quantity was differenced to build the
/// ladder.
fn audit_against_ladder(
    model_name: &'static str,
    size: usize,
    stamped: &[Value],
    ladder: &[Vec<Value>],
) -> StampAudit {
    // Everything is judged relative to the largest entry the device actually
    // stamped, so "verified" means "verified to a precision the solver could act
    // on" rather than "resolved to six figures in isolation".
    let block_scale = stamped
        .iter()
        .chain(ladder.iter().flatten())
        .filter(|value| value.is_finite())
        .fold(0.0, |worst: Value, value| worst.max(value.abs()));
    let floor = (block_scale * AUDIT_BLOCK_RELATIVE_FLOOR).max(AUDIT_ABSOLUTE_FLOOR);

    let mut entries = Vec::with_capacity(size * size);
    for row in 0..size {
        for col in 0..size {
            let index = row * size + col;
            let stamped_value = stamped[index];

            // Best-agreeing adjacent pair on the ladder. The finer member is the
            // estimate, because within a converged pair it carries the smaller
            // truncation error.
            let mut best: Option<(Value, Value)> = None;
            for window in ladder.windows(2) {
                let (coarser, finer) = (window[0][index], window[1][index]);
                if !coarser.is_finite() || !finer.is_finite() {
                    continue;
                }
                let disagreement = (coarser - finer).abs() / magnitude_scale(coarser, finer, floor);
                if best.is_none_or(|(worst, _)| disagreement < worst) {
                    best = Some((disagreement, finer));
                }
            }

            let (uncertainty, numeric) = best.unwrap_or((Value::INFINITY, Value::NAN));
            let relative_error = if numeric.is_finite() {
                (stamped_value - numeric).abs() / magnitude_scale(stamped_value, numeric, floor)
            } else {
                0.0
            };

            entries.push(StampAuditEntry {
                row: row as u32,
                col: col as u32,
                stamped: stamped_value,
                numeric,
                uncertainty,
                relative_error,
            });
        }
    }

    StampAudit {
        model_name,
        entries,
        significance_floor: floor,
    }
}

fn magnitude_scale(left: Value, right: Value, floor: Value) -> Value {
    left.abs().max(right.abs()).max(floor)
}

/// SplitMix64 seeded from the model name.
///
/// Any stable string hash would do; this one is written out rather than taken
/// from `DefaultHasher` because `DefaultHasher`'s output is explicitly not
/// guaranteed stable across Rust releases, and a fixture keyed to it would
/// silently re-randomize on a toolchain bump.
fn seed_for(model_name: &str) -> u64 {
    let mut state: u64 = 0x9e37_79b9_7f4a_7c15;
    for byte in model_name.as_bytes() {
        state ^= u64::from(*byte);
        state = state.wrapping_mul(0x0100_0000_01b3);
    }
    state
}

fn next_u64(state: &mut u64) -> u64 {
    *state = state.wrapping_add(0x9e37_79b9_7f4a_7c15);
    let mut z = *state;
    z = (z ^ (z >> 30)).wrapping_mul(0xbf58_476d_1ce4_e5b9);
    z = (z ^ (z >> 27)).wrapping_mul(0x94d0_49bb_1331_11eb);
    z ^ (z >> 31)
}

/// Uniform in `[0, 1)` from the top 53 bits, which is the only part of a
/// SplitMix64 word with full equidistribution.
fn next_unit(state: &mut u64) -> Value {
    (next_u64(state) >> 11) as Value / (1u64 << 53) as Value
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn probe_points_are_stable_for_a_model_name() {
        let mut first = seed_for("bsimbulk");
        let mut second = seed_for("bsimbulk");
        for _ in 0..8 {
            assert_eq!(next_u64(&mut first), next_u64(&mut second));
        }
    }

    #[test]
    fn probe_points_differ_between_models() {
        let mut left = seed_for("bsimbulk");
        let mut right = seed_for("bsimcmg_va");
        assert_ne!(next_u64(&mut left), next_u64(&mut right));
    }

    #[test]
    fn unit_draws_stay_in_range() {
        let mut state = seed_for("psp104va");
        for _ in 0..4096 {
            let unit = next_unit(&mut state);
            assert!((0.0..1.0).contains(&unit), "draw out of range: {unit}");
        }
    }

    #[test]
    fn magnitude_scale_takes_the_larger_operand_or_the_floor() {
        assert_eq!(magnitude_scale(0.0, 0.0, 1.0e-12), 1.0e-12);
        assert_eq!(magnitude_scale(1.0, -3.0, 1.0e-12), 3.0);
        assert_eq!(magnitude_scale(1.0e-20, 0.0, 1.0e-12), 1.0e-12);
    }

    fn entry(stamped: Value, numeric: Value, uncertainty: Value) -> StampAuditEntry {
        StampAuditEntry {
            row: 0,
            col: 0,
            stamped,
            numeric,
            uncertainty,
            relative_error: (stamped - numeric).abs() / stamped.abs().max(numeric.abs()).max(1e-30),
        }
    }

    #[test]
    fn a_difference_that_learned_nothing_is_a_gap_not_a_pass() {
        let audit = StampAudit {
            model_name: "fixture",
            significance_floor: 1.0e-9,
            entries: vec![
                entry(1.0, 1.0, 0.0),
                entry(1.0, 9.0e9, Value::INFINITY),
            ],
        };
        assert_eq!(audit.checked().count(), 1);
        assert_eq!(audit.checked_fraction(), 0.5);
        // The wild entry must not be reported as a numerical failure — the
        // difference is what failed — but it must surface as a coverage gap.
        assert!(audit.failures(1.0e-6).is_empty());
        assert_eq!(audit.unverified_significant().len(), 1);
    }

    #[test]
    fn negligible_entries_do_not_count_as_coverage_gaps() {
        let audit = StampAudit {
            model_name: "fixture",
            significance_floor: 1.0e-9,
            entries: vec![entry(1.0e-18, Value::NAN, Value::INFINITY)],
        };
        assert!(audit.unverified_significant().is_empty());
    }

    #[test]
    fn an_entry_is_not_held_tighter_than_the_difference_resolved_it() {
        // Stamp and difference agree to 7e-6, and the difference's own spread
        // there was 7e-7. Ten error bars covers it, so this is measurement
        // noise rather than a wrong derivative.
        let noisy = entry(1.0, 1.000_007, 7.0e-7);
        assert!(noisy.relative_error > 1.0e-6);
        assert!(noisy.relative_error <= noisy.required_tolerance(1.0e-6));

        // Same disagreement, but the difference was precise there. Now it is
        // real.
        let precise = entry(1.0, 1.000_007, 1.0e-12);
        assert!(precise.relative_error > precise.required_tolerance(1.0e-6));
    }
}
