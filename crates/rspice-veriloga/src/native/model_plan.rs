//! Architecture-neutral native model plan.
//!
//! Canonical lowering produces this complete set of typed programs and
//! dependency metadata exactly once. Machine backends only decide how those
//! entries are encoded, laid out, verified, and published.

#![cfg_attr(not(feature = "native"), allow(dead_code))]

use super::assignment::NativeAssignment;
use super::current_dependencies::JitCurrentDependencies;
use super::plan_program::PlanProgram;
use super::{JitError, JitResult};
use crate::codegen::CompiledModel;

/// The complete set of typed programs and dependency metadata one model
/// compiles to.
///
/// Every value entry is a [`PlanProgram`]: canonical lowering has two routes to
/// an expression and the plan carries whichever form the route produced, so a
/// backend dispatches on the form rather than the plan pretending there is only
/// one. Since W-F3c the default plan's `stamp_values`, `jacobians` and
/// `reactive_jacobians` are the CFG route's block form on every module that
/// route builds; the rest stay postfix.
///
/// The assignment passes are deliberately not part of that: they carry
/// `NativeProgram` directly, and the CFG route recomputes what they write
/// inline rather than reading their variable slots, so leaving them postfix is
/// what keeps both routes filling `variables` by the same code.
#[derive(Debug)]
pub(crate) struct NativeModelPlan {
    pub(crate) assignments: Vec<NativeAssignment>,
    pub(crate) post_assignments: Vec<NativeAssignment>,
    pub(crate) parameter_defaults: Vec<Option<PlanProgram>>,
    pub(crate) static_conditions: Vec<Option<PlanProgram>>,
    pub(crate) stamp_values: Vec<PlanProgram>,
    pub(crate) jacobians: Vec<Vec<PlanProgram>>,
    pub(crate) reactive_jacobians: Vec<Vec<PlanProgram>>,
    pub(crate) noise_psd: Vec<PlanProgram>,
    pub(crate) noise_exponents: Vec<Option<PlanProgram>>,
    pub(crate) published_current_pairs: Vec<Option<(usize, usize)>>,
    pub(crate) current_dependencies: JitCurrentDependencies,
}

impl NativeModelPlan {
    pub(crate) fn validate_shape(&self, model: &CompiledModel) -> JitResult<()> {
        let stamp_count = model.stamp_programs.len();
        let noise_count = model.noise_sources.len();
        if self.parameter_defaults.len() != model.parameters.len() {
            return Err(shape_error(format!(
                "native plan has {} parameter defaults for {} parameters",
                self.parameter_defaults.len(),
                model.parameters.len()
            )));
        }
        for (name, actual) in [
            ("static conditions", self.static_conditions.len()),
            ("stamp values", self.stamp_values.len()),
            ("Jacobian rows", self.jacobians.len()),
            ("reactive-Jacobian rows", self.reactive_jacobians.len()),
            (
                "published current-pair rows",
                self.published_current_pairs.len(),
            ),
        ] {
            if actual != stamp_count {
                return Err(shape_error(format!(
                    "native plan has {actual} {name} for {stamp_count} stamps"
                )));
            }
        }
        for (name, actual) in [
            ("noise PSD entries", self.noise_psd.len()),
            ("noise exponent entries", self.noise_exponents.len()),
        ] {
            if actual != noise_count {
                return Err(shape_error(format!(
                    "native plan has {actual} {name} for {noise_count} noise sources"
                )));
            }
        }
        for (stamp_index, (planned, compiled)) in
            self.jacobians.iter().zip(&model.stamp_programs).enumerate()
        {
            if planned.len() != compiled.jacobian_programs.len() {
                return Err(shape_error(format!(
                    "native plan stamp {stamp_index} has {} Jacobians for {} compiled entries",
                    planned.len(),
                    compiled.jacobian_programs.len()
                )));
            }
        }
        for (stamp_index, (planned, compiled)) in self
            .reactive_jacobians
            .iter()
            .zip(&model.stamp_programs)
            .enumerate()
        {
            if planned.len() != compiled.reactive_jacobians.len() {
                return Err(shape_error(format!(
                    "native plan stamp {stamp_index} has {} reactive Jacobians for {} compiled entries",
                    planned.len(),
                    compiled.reactive_jacobians.len()
                )));
            }
        }
        Ok(())
    }
}

fn shape_error(detail: impl Into<String>) -> JitError {
    JitError::InternalCompilerError {
        model: "native-model-plan".into(),
        detail: detail.into().into(),
    }
}
