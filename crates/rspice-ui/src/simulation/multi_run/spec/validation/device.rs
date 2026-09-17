//! Validation for the device-level analyses.
//!
//! The same family `runner/spec/device.rs` dispatches, with the two
//! variation analyses beside it: what a wear-out projection and a mismatch
//! spread refuse on is a device population and a bound on it, not a sweep.

use crate::simulation::multi_run::{AnalysisSpec, OptimizationGoal};

/// Validate one device-level specification.
pub(super) fn validate(spec: &AnalysisSpec) -> Result<(), String> {
    match spec {
        AnalysisSpec::Reliability {
            target_years,
            enable_hci,
            enable_nbti,
            enable_em,
            min_stress_voltage,
        } => {
            if target_years.is_empty() {
                return Err("Reliability target_years must not be empty".to_string());
            }
            if target_years
                .iter()
                .any(|years| !years.is_finite() || *years <= 0.0)
            {
                return Err("Reliability target_years must be finite and > 0".to_string());
            }
            if !enable_hci && !enable_nbti && !enable_em {
                return Err("Reliability requires at least one enabled mechanism".to_string());
            }
            if !min_stress_voltage.is_finite() || *min_stress_voltage < 0.0 {
                return Err("Reliability min_stress_voltage must be finite and >= 0".to_string());
            }
            Ok(())
        }
        AnalysisSpec::Optimization {
            variables,
            objective_node,
            objective_ref,
            goal,
            target,
            max_iterations,
            cost_tolerance,
            fd_step,
            initial_step,
            min_step,
            ..
        } => {
            if variables.is_empty() {
                return Err("Optimization variables must not be empty".to_string());
            }
            if objective_node.trim().is_empty() {
                return Err("Optimization objective_node is required".to_string());
            }
            if objective_ref.trim().is_empty() {
                return Err("Optimization objective_ref is required".to_string());
            }
            if objective_node.eq_ignore_ascii_case(objective_ref) {
                return Err("Optimization objective_node and objective_ref must differ".to_string());
            }
            if *max_iterations == 0 {
                return Err("Optimization max_iterations must be > 0".to_string());
            }
            if !cost_tolerance.is_finite() || *cost_tolerance <= 0.0 {
                return Err("Optimization cost_tolerance must be finite and > 0".to_string());
            }
            if !fd_step.is_finite() || *fd_step <= 0.0 {
                return Err("Optimization fd_step must be finite and > 0".to_string());
            }
            if !initial_step.is_finite() || *initial_step <= 0.0 {
                return Err("Optimization initial_step must be finite and > 0".to_string());
            }
            if !min_step.is_finite() || *min_step <= 0.0 {
                return Err("Optimization min_step must be finite and > 0".to_string());
            }
            if min_step > initial_step {
                return Err("Optimization min_step must be <= initial_step".to_string());
            }
            if *goal == OptimizationGoal::Target {
                if target.is_none() || target.is_some_and(|v| !v.is_finite()) {
                    return Err(
                        "Optimization target goal requires a finite target value".to_string()
                    );
                }
            } else if target.is_some_and(|v| !v.is_finite()) {
                return Err("Optimization target must be finite when provided".to_string());
            }

            let mut seen = std::collections::HashSet::new();
            for var in variables {
                if var.name.trim().is_empty() {
                    return Err("Optimization variable name must not be empty".to_string());
                }
                if !var.min.is_finite() || !var.max.is_finite() || !var.initial.is_finite() {
                    return Err(format!(
                        "Optimization variable '{}' bounds/initial must be finite",
                        var.name
                    ));
                }
                if var.max <= var.min {
                    return Err(format!(
                        "Optimization variable '{}' requires max > min",
                        var.name
                    ));
                }
                if var.initial < var.min || var.initial > var.max {
                    return Err(format!(
                        "Optimization variable '{}' initial must be within [{}, {}]",
                        var.name, var.min, var.max
                    ));
                }
                if !seen.insert(var.name.to_ascii_uppercase()) {
                    return Err(format!(
                        "Optimization variable '{}' is defined more than once",
                        var.name
                    ));
                }
            }
            Ok(())
        }
        AnalysisSpec::Soa {
            stop_time,
            step_time,
            check_vgs_max,
            max_vgs,
            check_vds_max,
            max_vds,
            check_vbe_max,
            max_vbe,
            check_vce_max,
            max_vce,
        } => {
            if !stop_time.is_finite() || *stop_time <= 0.0 {
                return Err("SOA stop_time must be finite and > 0".to_string());
            }
            if !step_time.is_finite() || *step_time <= 0.0 {
                return Err("SOA step_time must be finite and > 0".to_string());
            }
            if step_time > stop_time {
                return Err("SOA step_time must be <= stop_time".to_string());
            }
            if !check_vgs_max && !check_vds_max && !check_vbe_max && !check_vce_max {
                return Err("SOA requires at least one enabled check".to_string());
            }
            if *check_vgs_max && (!max_vgs.is_finite() || *max_vgs <= 0.0) {
                return Err("SOA max_vgs must be finite and > 0 when enabled".to_string());
            }
            if *check_vds_max && (!max_vds.is_finite() || *max_vds <= 0.0) {
                return Err("SOA max_vds must be finite and > 0 when enabled".to_string());
            }
            if *check_vbe_max && (!max_vbe.is_finite() || *max_vbe <= 0.0) {
                return Err("SOA max_vbe must be finite and > 0 when enabled".to_string());
            }
            if *check_vce_max && (!max_vce.is_finite() || *max_vce <= 0.0) {
                return Err("SOA max_vce must be finite and > 0 when enabled".to_string());
            }
            Ok(())
        }
        AnalysisSpec::DcMismatch {
            output_expression,
            sigma_multiplier,
            contributor_limit,
            include_process,
            include_mismatch,
            ..
        } => {
            if output_expression.trim().is_empty() {
                return Err("DCMATCH output_expression is required".to_owned());
            }
            if !sigma_multiplier.is_finite() || *sigma_multiplier <= 0.0 {
                return Err("DCMATCH sigma_multiplier must be finite and > 0".to_owned());
            }
            if *contributor_limit == 0 {
                return Err("DCMATCH contributor_limit must be > 0".to_owned());
            }
            if !include_process && !include_mismatch {
                return Err("DCMATCH requires process or mismatch contributions".to_owned());
            }
            Ok(())
        }
        other => Err(super::misrouted_specification("device-level", other)),
    }
}
