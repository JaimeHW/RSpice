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
            study,
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
            if let Some(study) = study {
                study.validate(target_years, *enable_hci, *enable_nbti, *enable_em)?;
            }
            Ok(())
        }
        AnalysisSpec::Optimization {
            search,
            variables,
            objective_unit,
            objective_expression,
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
            search.validate()?;
            crate::simulation::optimizer::validate_requested_unit(objective_unit)?;
            if variables.is_empty() {
                return Err("Optimization variables must not be empty".to_string());
            }
            if let Some(expression) = objective_expression {
                crate::services::simulation_runner::validate_optimization_expression(expression)?;
            } else {
                if objective_node.trim().is_empty() {
                    return Err("Optimization objective_node is required".to_string());
                }
                if objective_ref.trim().is_empty() {
                    return Err("Optimization objective_ref is required".to_string());
                }
                if objective_node.eq_ignore_ascii_case(objective_ref) {
                    return Err(
                        "Optimization objective_node and objective_ref must differ".to_string()
                    );
                }
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
            import_model_voltage_ratings,
            observation,
            rules,
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
            observation.validate(*stop_time)?;
            for rule in rules {
                rule.validate()?;
            }
            if !stop_time.is_finite() || *stop_time <= 0.0 {
                return Err("SOA stop_time must be finite and > 0".to_string());
            }
            if !step_time.is_finite() || *step_time <= 0.0 {
                return Err("SOA step_time must be finite and > 0".to_string());
            }
            if step_time > stop_time {
                return Err("SOA step_time must be <= stop_time".to_string());
            }
            if !import_model_voltage_ratings
                && rules.is_empty()
                && !check_vgs_max
                && !check_vds_max
                && !check_vbe_max
                && !check_vce_max
            {
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
            moment_options,
            output_expression,
            sigma_multiplier,
            include_process,
            include_mismatch,
            contribution_threshold,
            ..
        } => {
            moment_options
                .validate()
                .map_err(|error| error.to_string())?;
            // No bound of its own on `contributor_limit`: zero is the card's
            // authored spelling of "list every contributor", so a limit this
            // layer refused would refuse a card the engine accepts.
            if output_expression.trim().is_empty() {
                return Err(dc_mismatch_missing_output());
            }
            if !sigma_multiplier.is_finite() || *sigma_multiplier <= 0.0 {
                return Err(dc_mismatch_sigma_out_of_range(*sigma_multiplier));
            }
            if let Some(threshold) = contribution_threshold
                && (!threshold.is_finite() || !(0.0..=1.0).contains(threshold))
            {
                return Err(dc_mismatch_threshold_out_of_range(*threshold));
            }
            if !include_process && !include_mismatch {
                return Err(dc_mismatch_no_scope());
            }
            Ok(())
        }
        other => Err(super::misrouted_specification("device-level", other)),
    }
}

/// `.DCMATCH` refuses in the engine's own words, and this is where they are
/// written.
///
/// Every sentence below is built from `rspice-core`'s own `AnalysisCard` and
/// `AnalysisCardIssue` rather than spelled again here, so a Studio-authored
/// study and a hand-written card are refused by the same words for the same
/// reason. A reader who sees one of these in the Analyses page and then looks
/// the card up finds the identical sentence.
///
/// The plan draft reaches these through the specification rather than
/// directly: it parses its own text, assembles the specification the run
/// would carry, and asks *that*. So this file stays the only account of what
/// `.DCMATCH` refuses on, and the page cannot disagree with the run.
///
/// The card cannot default a probe, so this is the parser's
/// `MissingField { field: "OUT" }`.
fn dc_mismatch_missing_output() -> String {
    card_refusal(rspice_core::netlist::AnalysisCardIssue::MissingField { field: "OUT" })
}

/// The parser's `SIGMA` range, which is `value > 0.0`.
fn dc_mismatch_sigma_out_of_range(value: f64) -> String {
    card_refusal(rspice_core::netlist::AnalysisCardIssue::InvalidNumber {
        field: "SIGMA",
        value,
        expected: "a positive multiple of sigma",
    })
}

/// The parser's `THRESHOLD` range, which is `[0, 1]`.
fn dc_mismatch_threshold_out_of_range(value: f64) -> String {
    card_refusal(rspice_core::netlist::AnalysisCardIssue::InvalidNumber {
        field: "THRESHOLD",
        value,
        expected: "a variance share in [0, 1]",
    })
}

/// Both scopes off, in the *analysis* layer's words rather than the parser's.
///
/// The parser answers this as an invalid `PROCESS=` keyword, which reads
/// wrong on a form that offers two switches and no keywords. `Engine::
/// run_dc_match` refuses the same card in a sentence that names the scopes,
/// and that is the one a Studio operator gets.
fn dc_mismatch_no_scope() -> String {
    ".DCMATCH has nothing to vary: the card selects neither the mismatch nor the process scope"
        .to_owned()
}

/// `{card} {issue}`: the directive, then the parser's account of the field.
fn card_refusal(issue: rspice_core::netlist::AnalysisCardIssue) -> String {
    format!("{} {issue}", rspice_core::netlist::AnalysisCard::DcMatch)
}

#[cfg(test)]
mod tests {
    use super::*;

    fn refusal(spec: &AnalysisSpec) -> String {
        validate(spec).expect_err("the specification is refused")
    }

    /// The engine's refusal has to carry the card and the issue the Studio
    /// quotes.
    ///
    /// The core parser locates its refusals — `.DCMATCH at line 5: ...` —
    /// while a form has no line to name, so the two are tied by the card and
    /// the issue rather than by one whole string.
    fn tied_to_the_engine(core: &str, studio: &str) {
        assert!(core.contains(".DCMATCH"), "core `{core}` names no card");
        let issue = studio
            .strip_prefix(".DCMATCH ")
            .unwrap_or_else(|| panic!("`{studio}` does not open with the card it refuses"));
        assert!(
            core.contains(issue),
            "the engine says `{core}`, the Studio says `{issue}`"
        );
    }

    fn dc_mismatch(
        output_expression: &str,
        sigma_multiplier: f64,
        scopes: (bool, bool),
    ) -> AnalysisSpec {
        AnalysisSpec::DcMismatch {
            moment_options: Default::default(),
            output_expression: output_expression.to_owned(),
            sigma_multiplier,
            contributor_limit: 10,
            include_mismatch: scopes.0,
            include_process: scopes.1,
            normalized_contributions: true,
            contribution_threshold: None,
        }
    }

    fn with_threshold(threshold: Option<f64>) -> AnalysisSpec {
        let AnalysisSpec::DcMismatch {
            output_expression,
            sigma_multiplier,
            contributor_limit,
            include_mismatch,
            include_process,
            normalized_contributions,
            ..
        } = dc_mismatch("V(out)", 1.0, (true, false))
        else {
            unreachable!("the fixture is a DC mismatch specification");
        };
        AnalysisSpec::DcMismatch {
            moment_options: Default::default(),
            output_expression,
            sigma_multiplier,
            contributor_limit,
            include_mismatch,
            include_process,
            normalized_contributions,
            contribution_threshold: threshold,
        }
    }

    /// Every DC mismatch refusal is the sentence the engine writes, tied to
    /// the engine that writes it.
    ///
    /// Not a string comparison against a second copy of the words: each
    /// Studio sentence is looked for inside the refusal `rspice-core` itself
    /// produces for the equivalent card, so a reworded core refusal fails
    /// here rather than leaving the Analyses page quoting a sentence no
    /// engine says any more.
    #[test]
    fn a_dc_mismatch_refusal_is_the_sentence_the_engine_writes() {
        let deck = |card: &str| {
            format!("dc mismatch refusals\nV1 in 0 1\nR1 in out 1k\nR2 out 0 2k\n{card}\n.end\n")
        };

        // `OUT` cannot be defaulted, and the parser says so by name.
        let core = rspice_core::netlist::Netlist::parse(&deck(".DCMATCH MISMATCH=yes"))
            .expect_err("a card without OUT is refused")
            .to_string();
        tied_to_the_engine(&core, &refusal(&dc_mismatch("   ", 1.0, (true, false))));

        // `SIGMA` must be positive.
        let core = rspice_core::netlist::Netlist::parse(&deck(".DCMATCH OUT=V(out) SIGMA=0"))
            .expect_err("a zero multiplier is refused")
            .to_string();
        tied_to_the_engine(&core, &refusal(&dc_mismatch("V(out)", 0.0, (true, false))));

        // Both scopes off is the analysis layer's refusal, not the parser's:
        // the parser answers it as an invalid `PROCESS=` keyword, which reads
        // wrong on a form that has switches. A card built in code reaches
        // `Engine::run_dc_match` and gets the sentence this form quotes.
        let netlist =
            rspice_core::netlist::Netlist::parse(&deck(".op")).expect("the fixture design parses");
        let card = rspice_core::netlist::DcMatchCard {
            moments: Default::default(),
            output_node: "OUT".to_owned(),
            reference_node: None,
            output_is_current: false,
            mismatch: false,
            process: false,
            contributor_limit: 10,
            threshold: 0.0,
            sigma_multiplier: 1.0,
        };
        let engine =
            rspice_core::engine::Engine::try_new(rspice_core::engine::SimulationConfig::default())
                .expect("the default numerical policy is valid");
        let core = engine
            .run_dc_match(&netlist, &card)
            .expect_err("a card with neither scope is refused")
            .to_string();
        tied_to_the_engine(&core, &refusal(&dc_mismatch("V(out)", 1.0, (false, false))));

        // `THRESHOLD` is a variance share, so it has a closed range.
        let core = rspice_core::netlist::Netlist::parse(&deck(".DCMATCH OUT=V(out) THRESHOLD=1.5"))
            .expect_err("a share above one is refused")
            .to_string();
        tied_to_the_engine(&core, &refusal(&with_threshold(Some(1.5))));
    }

    /// The limit the card calls "all" is not a refusal here either, and the
    /// two share thresholds the card admits are both accepted.
    #[test]
    fn a_contributor_limit_of_zero_is_a_valid_specification() {
        let AnalysisSpec::DcMismatch {
            output_expression,
            sigma_multiplier,
            include_mismatch,
            include_process,
            normalized_contributions,
            contribution_threshold,
            ..
        } = dc_mismatch("V(out)", 1.0, (true, false))
        else {
            unreachable!("the fixture is a DC mismatch specification");
        };
        let spec = AnalysisSpec::DcMismatch {
            moment_options: Default::default(),
            output_expression,
            sigma_multiplier,
            contributor_limit: 0,
            include_mismatch,
            include_process,
            normalized_contributions,
            contribution_threshold,
        };
        assert_eq!(validate(&spec), Ok(()));
        assert_eq!(validate(&with_threshold(None)), Ok(()));
        assert_eq!(validate(&with_threshold(Some(1.0))), Ok(()));
    }
}
