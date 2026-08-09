//! Architecture-neutral native model plan.
//!
//! Canonical lowering produces this complete set of typed programs and
//! dependency metadata exactly once. Machine backends only decide how those
//! entries are encoded, laid out, verified, and published.

use super::assignment::NativeAssignment;
use super::expr::NativeProgram;
use super::model::NativeCurrentDependencies;
use super::{JitError, JitResult};
use crate::codegen::CompiledModel;

#[derive(Debug)]
pub(super) struct NativeModelPlan {
    pub(super) assignments: Vec<NativeAssignment>,
    pub(super) post_assignments: Vec<NativeAssignment>,
    pub(super) parameter_defaults: Vec<Option<NativeProgram>>,
    pub(super) static_conditions: Vec<Option<NativeProgram>>,
    pub(super) stamp_values: Vec<NativeProgram>,
    pub(super) jacobians: Vec<Vec<NativeProgram>>,
    pub(super) reactive_jacobians: Vec<Vec<NativeProgram>>,
    pub(super) noise_psd: Vec<NativeProgram>,
    pub(super) noise_exponents: Vec<Option<NativeProgram>>,
    pub(super) published_current_pairs: Vec<Option<(usize, usize)>>,
    pub(super) current_dependencies: NativeCurrentDependencies,
}

impl NativeModelPlan {
    pub(super) fn validate_shape(&self, model: &CompiledModel) -> JitResult<()> {
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
