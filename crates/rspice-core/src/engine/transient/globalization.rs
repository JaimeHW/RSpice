//! Merit-gated backtracking globalization for the transient Newton loop.
//!
//! SPICE-style transient Newton takes raw (junction-limited) full steps. On
//! an ill-conditioned manifold — e.g. a BJT mirror crossing the saturation
//! boundary, where an epi charge turns on exponentially — the full step
//! overshoots its own linearization and the iterates settle into a limit
//! cycle that no timestep reduction breaks: the cycle is driven by the
//! static nonlinearity, not by stiffness, so every dt cut replays it.
//! Production simulators globalize Newton with a merit line search; this
//! module supplies that for the transient loop, mirroring the Armijo
//! discipline the DC solver already uses.
//!
//! The merit function is the scaled infinity-norm of the true nonlinear
//! residual `F(x) = A(x)·x − b(x)`, available for one matrix-vector product
//! right after the loop stamps `A(x), b(x)` at the current iterate — before
//! the linear solve, at essentially no cost. The previous iterate's merit is
//! retained; when a step grows the merit beyond [`MERIT_GROWTH_GATE`] while
//! the residual is still unconverged, the step is re-tried at geometrically
//! damped fractions until one satisfies an Armijo decrease or the trial
//! budget is exhausted (the best trial seen is then kept). Steps that do not
//! grow the residual an order of magnitude are never touched, so converging
//! decks see bit-identical trajectories.

use super::*;

/// Multiplicative growth of the true-residual merit that marks a Newton step
/// as having left the basin of its linearization. Ordinary rough progress —
/// including the pnjlim wandering through which SPICE iterates legitimately
/// explore a hard junction edge before capture — stays well below this; an
/// eagerness experiment that also damped flat unconverged plateaus strangled
/// that wandering and converged strictly less often, so only genuine
/// blow-ups (and non-finite residuals) engage the search. Iterates the
/// search cannot help (a limiter-pinned non-descent direction) are the
/// gmin-continuation rescue's job, not the line search's.
const MERIT_GROWTH_GATE: Value = 10.0;

/// Armijo sufficient-decrease coefficient, matching the DC line search.
const ARMIJO_C1: Value = 1e-4;

/// Geometric damping factor between trials, matching the DC line search.
const BACKTRACK_FACTOR: Value = 0.5;

/// Trial budget per search (smallest fraction tried is 2^-8). On exhaustion
/// the best trial seen — possibly the original full step — is kept, leaving
/// the outer timestep-cut machinery as the unchanged fallback.
const MAX_TRIALS: usize = 8;

/// Next move for the Newton loop after judging a stamped iterate.
pub(super) enum BacktrackAction {
    /// Keep the just-stamped iterate and proceed to the linear solve.
    Accept,
    /// Re-evaluate devices and restamp at this damped iterate instead.
    Trial(Vec<Value>),
}

/// One active backtracking search along a rejected Newton step.
pub(super) struct NewtonMeritBacktrack {
    base: Vec<Value>,
    base_merit: Value,
    step: Vec<Value>,
    alpha: Value,
    best_alpha: Value,
    best_merit: Value,
    trials: usize,
    concluded: bool,
}

impl NewtonMeritBacktrack {
    /// True when the freshly stamped iterate's merit shows the last Newton
    /// step left the basin of its linearization: order-of-magnitude residual
    /// growth (or a non-finite residual) while the iterate is still
    /// unconverged.
    pub(super) fn step_needs_globalization(
        previous_merit: Value,
        current_merit: Value,
    ) -> bool {
        if !previous_merit.is_finite() {
            return false;
        }
        if !current_merit.is_finite() {
            return true;
        }
        current_merit > 1.0 && current_merit > previous_merit * MERIT_GROWTH_GATE
    }

    /// Start a search along `candidate − base` and return the first damped
    /// trial point. `candidate` (the full step, already junction-limited by
    /// the caller) seeds the best-trial record so exhaustion can fall back
    /// to it honestly when no damped fraction improves on it.
    pub(super) fn engage(
        base: &[Value],
        base_merit: Value,
        candidate: &[Value],
        candidate_merit: Value,
    ) -> (Self, Vec<Value>) {
        let step: Vec<Value> = candidate
            .iter()
            .zip(base)
            .map(|(candidate_v, base_v)| candidate_v - base_v)
            .collect();
        let alpha = BACKTRACK_FACTOR;
        let trial = Self::point_at(base, &step, alpha);
        (
            Self {
                base: base.to_vec(),
                base_merit,
                step,
                alpha,
                best_alpha: 1.0,
                best_merit: candidate_merit,
                trials: 1,
                concluded: false,
            },
            trial,
        )
    }

    /// Judge the merit of the current trial (already stamped by the caller)
    /// and decide whether to keep it or move to another damped fraction.
    pub(super) fn judge(&mut self, trial_merit: Value) -> BacktrackAction {
        if self.concluded {
            // The previous action re-positioned the iterate on the best
            // trial; its restamp is what the caller just measured.
            return BacktrackAction::Accept;
        }

        if trial_merit < self.best_merit {
            self.best_merit = trial_merit;
            self.best_alpha = self.alpha;
        }

        let armijo_ok = trial_merit <= 1.0
            || trial_merit <= self.base_merit * (1.0 - ARMIJO_C1 * self.alpha);
        if armijo_ok {
            return BacktrackAction::Accept;
        }

        if self.trials >= MAX_TRIALS {
            self.concluded = true;
            if self.best_alpha == self.alpha {
                return BacktrackAction::Accept;
            }
            return BacktrackAction::Trial(Self::point_at(
                &self.base,
                &self.step,
                self.best_alpha,
            ));
        }

        self.alpha *= BACKTRACK_FACTOR;
        self.trials += 1;
        BacktrackAction::Trial(Self::point_at(&self.base, &self.step, self.alpha))
    }

    fn point_at(base: &[Value], step: &[Value], alpha: Value) -> Vec<Value> {
        base.iter()
            .zip(step)
            .map(|(base_v, step_v)| base_v + alpha * step_v)
            .collect()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn globalization_gate_engages_only_on_unconverged_order_of_magnitude_growth() {
        // Ordinary progress and mild non-monotonicity never engage; the
        // flat unconverged plateaus of pnjlim wandering are left alone.
        assert!(!NewtonMeritBacktrack::step_needs_globalization(50.0, 49.0));
        assert!(!NewtonMeritBacktrack::step_needs_globalization(50.0, 400.0));
        assert!(!NewtonMeritBacktrack::step_needs_globalization(1.6e3, 1.9e3));
        // Converged-scale residuals never engage regardless of ratio.
        assert!(!NewtonMeritBacktrack::step_needs_globalization(0.05, 0.9));
        // The blow-up signature engages.
        assert!(NewtonMeritBacktrack::step_needs_globalization(50.0, 5000.0));
        // A step into non-finite territory engages.
        assert!(NewtonMeritBacktrack::step_needs_globalization(
            50.0,
            Value::INFINITY
        ));
        assert!(NewtonMeritBacktrack::step_needs_globalization(
            50.0,
            Value::NAN
        ));
        // Without a finite reference there is nothing to judge against.
        assert!(!NewtonMeritBacktrack::step_needs_globalization(
            Value::INFINITY,
            5000.0
        ));
    }

    #[test]
    fn search_accepts_first_trial_with_sufficient_decrease() {
        let base = vec![1.0, 2.0];
        let candidate = vec![3.0, 6.0];
        let (mut search, trial) =
            NewtonMeritBacktrack::engage(&base, 100.0, &candidate, 5000.0);
        assert_eq!(trial, vec![2.0, 4.0]);

        // Half step decreased the merit below the Armijo line: accepted.
        match search.judge(60.0) {
            BacktrackAction::Accept => {}
            BacktrackAction::Trial(_) => panic!("sufficient decrease must be accepted"),
        }
    }

    #[test]
    fn search_halves_until_decrease_then_accepts() {
        let base = vec![0.0];
        let candidate = vec![8.0];
        let (mut search, trial) =
            NewtonMeritBacktrack::engage(&base, 100.0, &candidate, 4000.0);
        assert_eq!(trial, vec![4.0]);

        // Still worse than base at alpha = 1/2 and 1/4.
        let trial = match search.judge(900.0) {
            BacktrackAction::Trial(point) => point,
            BacktrackAction::Accept => panic!("merit above base must keep searching"),
        };
        assert_eq!(trial, vec![2.0]);
        let trial = match search.judge(300.0) {
            BacktrackAction::Trial(point) => point,
            BacktrackAction::Accept => panic!("merit above base must keep searching"),
        };
        assert_eq!(trial, vec![1.0]);

        // alpha = 1/8 satisfies the decrease.
        match search.judge(80.0) {
            BacktrackAction::Accept => {}
            BacktrackAction::Trial(_) => panic!("sufficient decrease must be accepted"),
        }
    }

    #[test]
    fn exhausted_search_repositions_on_best_trial_then_accepts() {
        let base = vec![0.0];
        let candidate = vec![256.0];
        let (mut search, _) = NewtonMeritBacktrack::engage(&base, 100.0, &candidate, 4000.0);

        // Feed monotonically worsening merits except one good-but-not-Armijo
        // trial at alpha = 1/4 (merit above base, so the search continues).
        // Seven judges halve down to the eighth and last trial at 2^-8.
        let merits = [800.0, 150.0, 900.0, 950.0, 1000.0, 1100.0, 1200.0];
        let mut last_action = None;
        for merit in merits {
            last_action = Some(search.judge(merit));
        }
        match last_action.unwrap() {
            BacktrackAction::Trial(point) => assert_eq!(point, vec![1.0]),
            BacktrackAction::Accept => panic!("trial budget not yet exhausted"),
        }

        // Final trial is also bad: budget exhausted, jump back to the best
        // trial seen (alpha = 1/4 -> x = 64).
        match search.judge(1300.0) {
            BacktrackAction::Trial(point) => assert_eq!(point, vec![64.0]),
            BacktrackAction::Accept => panic!("exhaustion must reposition on the best trial"),
        }
        // The repositioned iterate is accepted unconditionally.
        match search.judge(150.0) {
            BacktrackAction::Accept => {}
            BacktrackAction::Trial(_) => panic!("concluded search must accept"),
        }
    }

    #[test]
    fn exhausted_search_keeps_full_step_when_no_damping_helps() {
        let base = vec![0.0];
        let candidate = vec![1.0];
        let (mut search, _) = NewtonMeritBacktrack::engage(&base, 100.0, &candidate, 2000.0);

        // Every damped fraction is worse than the full step: seven judges
        // halve down to the last trial, the eighth exhausts the budget.
        let mut action = search.judge(3000.0);
        for _ in 0..7 {
            action = match action {
                BacktrackAction::Trial(_) => search.judge(3500.0),
                BacktrackAction::Accept => panic!("worsening trials must keep searching"),
            };
        }
        // Budget exhausted with the full step still best: reposition on it.
        match action {
            BacktrackAction::Trial(point) => assert_eq!(point, vec![1.0]),
            BacktrackAction::Accept => panic!("exhaustion must reposition on the full step"),
        }
        match search.judge(2000.0) {
            BacktrackAction::Accept => {}
            BacktrackAction::Trial(_) => panic!("concluded search must accept"),
        }
    }
}
