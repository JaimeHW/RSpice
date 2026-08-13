//! Formatting results, mismatches, and suite reports.
//!
//! Part of the Xyce suite's `XyceTestRunner`, split out of a single
//! 96,731-line file. Methods keep `impl XyceTestRunner` so call sites are
//! unchanged; private ones are `pub(super)` so siblings can reach them.

use super::*;

/// `.OPTIONS OUTPUT` schedule: the initial print interval, then the
/// `(until_time, interval)` pairs that supersede it. `None` when the deck
/// declares no schedule at all.
type OutputIntervalSchedule = Result<Option<(Value, Vec<(Value, Value)>)>, String>;

#[derive(Debug, Clone, PartialEq, Eq)]
pub(super) enum XyceGeneratedVbicNoiseIssue {
    ModelUnavailable {
        model_name: &'static str,
        mechanism: String,
    },
    NoiseDescriptorsUnavailable {
        model_name: &'static str,
        mechanism: String,
    },
    UnknownMechanism {
        model_name: &'static str,
        mechanism: String,
    },
}

impl XyceTestRunner {
    pub fn print_summary(results: &[XyceTestResult]) {
        let stats = Self::statistics(results);
        println!("\nXyce corpus summary");
        println!(
            "  total: {} | executed: {} | passed: {} | failed: {} | expected unsupported: {} | upstream excluded: {}",
            stats.total,
            stats.executed,
            stats.passed,
            stats.failed,
            stats.expected_unsupported,
            stats.upstream_excluded
        );
        println!(
            "  executed pass rate: {:.1}% | executed coverage: {:.1}%",
            stats.executed_pass_rate(),
            stats.executed_coverage_rate()
        );

        for result in results.iter().filter(|result| !result.passed).take(20) {
            println!(
                "  FAIL {} [{}]: {}",
                result.relative_path,
                result.contract,
                result.error.as_deref().unwrap_or("unknown failure")
            );
            for mismatch in result.mismatches.iter().take(3) {
                println!(
                    "       row {} {} expected {:.8e}, actual {:.8e}, rel {:.3e}",
                    mismatch.row,
                    mismatch.probe,
                    mismatch.expected,
                    mismatch.actual,
                    mismatch.relative_error
                );
            }
        }
    }

    pub(super) fn reject_xdm_replaceground_output_artifacts(
        &self,
        deck_path: &Path,
    ) -> Result<(), String> {
        let anchor = self
            .static_output_reference_path(deck_path, "anchor")
            .ok_or_else(|| "XDM REPLACEGROUND deck cannot be mapped into OutputData".to_string())?;
        let Some(output_dir) = anchor.parent() else {
            return Err("XDM REPLACEGROUND OutputData anchor has no parent".to_string());
        };
        if !output_dir.exists() {
            return Ok(());
        }
        let metadata = fs::symlink_metadata(output_dir)
            .map_err(|error| format!("failed to inspect XDM REPLACEGROUND OutputData: {error}"))?;
        if !metadata.file_type().is_dir() || metadata.file_type().is_symlink() {
            return Err(format!(
                "XDM REPLACEGROUND OutputData path {} must be a regular non-symlink directory",
                output_dir.display()
            ));
        }
        let deck_name = deck_path
            .file_name()
            .and_then(|name| name.to_str())
            .ok_or_else(|| "XDM REPLACEGROUND deck filename is not UTF-8".to_string())?;
        let prefix = format!("{deck_name}.").to_ascii_lowercase();
        let mut artifacts = Vec::new();
        for entry in fs::read_dir(output_dir).map_err(|error| {
            format!("failed to inspect XDM REPLACEGROUND OutputData entries: {error}")
        })? {
            let entry = entry.map_err(|error| {
                format!("failed to inspect XDM REPLACEGROUND OutputData entry: {error}")
            })?;
            if entry
                .file_name()
                .to_str()
                .is_some_and(|name| name.to_ascii_lowercase().starts_with(&prefix))
            {
                artifacts.push(entry.path());
            }
        }
        artifacts.sort();
        if !artifacts.is_empty() {
            return Err(format!(
                "XDM REPLACEGROUND candidate must not own checked-in OutputData artifacts: {artifacts:?}"
            ));
        }
        Ok(())
    }

    pub(super) fn reject_addresistors_output_artifacts(
        &self,
        deck_path: &Path,
    ) -> Result<(), String> {
        let anchor = self
            .static_output_reference_path(deck_path, "anchor")
            .ok_or_else(|| "ADDRESISTORS deck cannot be mapped into OutputData".to_string())?;
        let Some(output_dir) = anchor.parent() else {
            return Err("ADDRESISTORS OutputData anchor has no parent".to_string());
        };
        if !output_dir.exists() {
            return Ok(());
        }
        let metadata = fs::symlink_metadata(output_dir)
            .map_err(|error| format!("failed to inspect ADDRESISTORS OutputData: {error}"))?;
        if !metadata.file_type().is_dir() || metadata.file_type().is_symlink() {
            return Err(format!(
                "ADDRESISTORS OutputData path {} must be a regular non-symlink directory",
                output_dir.display()
            ));
        }
        let deck_name = deck_path
            .file_name()
            .and_then(|name| name.to_str())
            .ok_or_else(|| "ADDRESISTORS deck filename is not UTF-8".to_string())?;
        let prefix = format!("{deck_name}.").to_ascii_lowercase();
        let mut artifacts = Vec::new();
        for entry in fs::read_dir(output_dir)
            .map_err(|error| format!("failed to inspect ADDRESISTORS OutputData: {error}"))?
        {
            let entry = entry
                .map_err(|error| format!("failed to inspect ADDRESISTORS artifact: {error}"))?;
            if entry
                .file_name()
                .to_str()
                .is_some_and(|name| name.to_ascii_lowercase().starts_with(&prefix))
            {
                artifacts.push(entry.path());
            }
        }
        artifacts.sort();
        if artifacts.is_empty() {
            Ok(())
        } else {
            Err(format!(
                "ADDRESISTORS candidate must not own checked-in OutputData artifacts: {artifacts:?}"
            ))
        }
    }

    pub(super) fn reject_removeunused_output_artifacts(
        &self,
        deck_path: &Path,
    ) -> Result<(), String> {
        let anchor = self
            .static_output_reference_path(deck_path, "anchor")
            .ok_or_else(|| "REMOVEUNUSED deck cannot be mapped into OutputData".to_string())?;
        let Some(output_dir) = anchor.parent() else {
            return Err("REMOVEUNUSED OutputData anchor has no parent".to_string());
        };
        if !output_dir.exists() {
            return Ok(());
        }
        let metadata = fs::symlink_metadata(output_dir)
            .map_err(|error| format!("failed to inspect REMOVEUNUSED OutputData: {error}"))?;
        if !metadata.file_type().is_dir() || metadata.file_type().is_symlink() {
            return Err(format!(
                "REMOVEUNUSED OutputData path {} must be a regular non-symlink directory",
                output_dir.display()
            ));
        }
        let deck_name = deck_path
            .file_name()
            .and_then(|name| name.to_str())
            .ok_or_else(|| "REMOVEUNUSED deck filename is not UTF-8".to_string())?;
        let prefix = format!("{deck_name}.").to_ascii_lowercase();
        let mut artifacts = Vec::new();
        for entry in fs::read_dir(output_dir)
            .map_err(|error| format!("failed to inspect REMOVEUNUSED OutputData: {error}"))?
        {
            let entry = entry
                .map_err(|error| format!("failed to inspect REMOVEUNUSED artifact: {error}"))?;
            if entry
                .file_name()
                .to_str()
                .is_some_and(|name| name.to_ascii_lowercase().starts_with(&prefix))
            {
                artifacts.push(entry.path());
            }
        }
        artifacts.sort();
        if artifacts.is_empty() {
            Ok(())
        } else {
            Err(format!(
                "REMOVEUNUSED candidate must not own checked-in OutputData artifacts: {artifacts:?}"
            ))
        }
    }

    pub(super) fn reject_startup_output_artifacts(&self, deck_path: &Path) -> Result<(), String> {
        let anchor = self
            .static_output_reference_path(deck_path, "anchor")
            .ok_or_else(|| {
                "startup-diagnostic source cannot be mapped into OutputData".to_string()
            })?;
        let output_dir = anchor
            .parent()
            .ok_or_else(|| "startup-diagnostic OutputData path has no parent".to_string())?;
        if !output_dir.exists() {
            return Ok(());
        }
        let metadata = fs::symlink_metadata(output_dir)
            .map_err(|error| format!("failed to inspect startup OutputData: {error}"))?;
        if !metadata.file_type().is_dir() || metadata.file_type().is_symlink() {
            return Err(format!(
                "startup-diagnostic OutputData path {} must be a regular non-symlink directory",
                output_dir.display()
            ));
        }
        let deck_name = deck_path
            .file_name()
            .and_then(|name| name.to_str())
            .ok_or_else(|| "startup-diagnostic filename is not UTF-8".to_string())?;
        let prefix = format!("{deck_name}.").to_ascii_lowercase();
        let mut artifacts = Vec::new();
        for entry in fs::read_dir(output_dir)
            .map_err(|error| format!("failed to inspect startup OutputData entries: {error}"))?
        {
            let entry = entry.map_err(|error| {
                format!(
                    "failed to read startup OutputData entry in {}: {error}",
                    output_dir.display()
                )
            })?;
            if entry
                .file_name()
                .to_str()
                .is_some_and(|name| name.to_ascii_lowercase().starts_with(&prefix))
            {
                artifacts.push(entry.path());
            }
        }
        artifacts.sort();
        if !artifacts.is_empty() {
            return Err(format!(
                "startup-diagnostic source must not own checked-in output artifacts: {artifacts:?}"
            ));
        }
        Ok(())
    }

    /// Return whether an ordinary, precisionless `.PRINT TRAN` card is safe
    /// to compare with the default Release 7.10 integrated-RMS verifier for
    /// a structurally qualified LEVEL=9 deck.  Xyce serializes this form at
    /// the default scientific precision; the stricter helper above remains
    /// responsible for authored `PRECISION=12` cards and all option/error
    /// validation.
    pub(super) fn level9_xyce_verify_default_output(
        source: &str,
        expected_model_count: usize,
    ) -> bool {
        let lines = Self::logical_netlist_lines(source);
        if lines.is_empty() {
            return false;
        }

        let mut model_count = 0usize;
        let mut tran_count = 0usize;
        let mut print_count = 0usize;
        let mut end_count = 0usize;
        let mut saw_probe = false;
        let mut saw_end = false;

        for line in lines.iter().skip(1) {
            let stripped = Self::strip_netlist_comment(line).trim();
            if stripped.is_empty() {
                continue;
            }
            if saw_end {
                return false;
            }
            let Ok(fields) = Self::split_print_fields(stripped) else {
                return false;
            };
            let Some(command) = fields.first() else {
                continue;
            };
            if !command.starts_with('.') {
                continue;
            }
            match command.to_ascii_lowercase().as_str() {
                ".model" => model_count += 1,
                ".tran" if fields.len() == 3 => tran_count += 1,
                ".tran" => return false,
                ".print" => {
                    if fields.len() < 3 || !fields[1].eq_ignore_ascii_case("tran") {
                        return false;
                    }
                    let field_refs = fields.iter().map(String::as_str).collect::<Vec<_>>();
                    if fields[2..].iter().enumerate().any(|(offset, field)| {
                        let index = offset + 2;
                        Self::print_option_assignment(&field_refs, index).is_some()
                            || Self::is_print_option_token(&field.to_ascii_lowercase())
                            || matches!(
                                field.to_ascii_lowercase().as_str(),
                                "file"
                                    | "format"
                                    | "width"
                                    | "precision"
                                    | "delimiter"
                                    | "noindex"
                                    | "index"
                                    | "filter"
                                    | "timescalefactor"
                            )
                    }) {
                        return false;
                    }
                    print_count += 1;
                    saw_probe = true;
                }
                ".end" if fields.len() == 1 => {
                    end_count += 1;
                    saw_end = true;
                }
                ".end" => return false,
                _ => return false,
            }
        }

        model_count == expected_model_count
            && tran_count == 1
            && print_count == 1
            && end_count == 1
            && saw_probe
            && saw_end
    }

    /// Return the schema emitted by Xyce's AC sensitivity outputter for a
    /// `.PRINT SENS FORMAT=` value.  Xyce only has native AC sensitivity
    /// writers for STD, CSV, and TECPLOT; RAW/PROBE/Touchstone/Dakota and
    /// unrecognized PRN-like formats deliberately fall back to standard PRN.
    /// TECPLOT is kept fail-closed here until a native table parser exists.
    pub(super) fn ac_sensitivity_output_schema(
        format: Option<&str>,
    ) -> Result<(XyceAcSensitivityReferenceFormat, bool), String> {
        let normalized = format.unwrap_or("STD").trim();
        if normalized.eq_ignore_ascii_case("CSV") {
            return Ok((XyceAcSensitivityReferenceFormat::Csv, true));
        }
        if normalized.eq_ignore_ascii_case("NOINDEX") {
            return Ok((XyceAcSensitivityReferenceFormat::Prn, true));
        }
        if normalized.eq_ignore_ascii_case("STD") {
            return Ok((XyceAcSensitivityReferenceFormat::Prn, false));
        }
        if matches!(
            normalized.to_ascii_lowercase().as_str(),
            "raw"
                | "raw_ascii"
                | "probe"
                | "dakota"
                | "touchstone"
                | "touchstone2"
                | "ts1"
                | "ts2"
                | "gnuplot"
                | "splot"
        ) {
            return Ok((XyceAcSensitivityReferenceFormat::Prn, false));
        }
        Err(format!(
            "native Xyce AC sensitivity contract does not cover .PRINT SENS FORMAT={normalized}"
        ))
    }

    fn noise_print_generated_vbic_mechanism_requirements(
        print: &XycePrintRequest,
        netlist: &Netlist,
    ) -> Result<Vec<(&'static str, String)>, String> {
        let generated_target = |device: &str| {
            netlist.elements.iter().find_map(|element| {
                if !element.name.eq_ignore_ascii_case(device) {
                    return None;
                }
                let ElementKind::Bjt { model, .. } = &element.kind else {
                    return None;
                };
                let model = Self::find_model(&netlist.models, model)?;
                if !Self::model_is_native_vbic_bjt(model) {
                    return None;
                }
                let level = Self::numeric_param_value(&model.params, "LEVEL")?;
                if (level - 11.0).abs() <= 1.0e-9 {
                    Some(if element.nodes.len() >= 4 {
                        "VBIC13_3T_ET"
                    } else {
                        "VBIC13"
                    })
                } else if [4.0, 9.0, 12.0, 13.0]
                    .iter()
                    .any(|candidate| (level - candidate).abs() <= 1.0e-9)
                {
                    Some("VBIC13_4T")
                } else {
                    None
                }
            })
        };

        let mut requirements = Vec::new();
        for printed_probe in &print.probes {
            let mut contribution_calls = Vec::new();
            if rspice_core::analysis::NoiseContributionProbe::parse(printed_probe).is_ok() {
                contribution_calls.push(printed_probe.clone());
            } else if let Some(expression) = Self::print_expression_inner(printed_probe) {
                let normalized_expression = expression.to_ascii_lowercase();
                if !normalized_expression.contains("dno(")
                    && !normalized_expression.contains("dni(")
                {
                    continue;
                }
                let mut collect = |call: &str| {
                    if call.get(..3).is_some_and(|prefix| {
                        prefix.eq_ignore_ascii_case("dno") || prefix.eq_ignore_ascii_case("dni")
                    }) {
                        contribution_calls.push(call.to_string());
                    }
                    Ok(1.0)
                };
                Self::evaluate_print_expression_with_probe_calls(
                    expression,
                    netlist.params.clone(),
                    &mut collect,
                )
                .map_err(|err| {
                    format!(
                        "failed to inspect NOISE print expression '{printed_probe}' for generated-device mechanisms: {err}"
                    )
                })?;
            }

            for call in contribution_calls {
                let contribution = rspice_core::analysis::NoiseContributionProbe::parse(&call)
                    .map_err(|err| err.to_string())?;
                if let Some(mechanism) = contribution.mechanism
                    && let Some(target) = generated_target(&contribution.device)
                {
                    requirements.push((target, mechanism));
                }
            }
        }
        Ok(requirements)
    }

    #[cfg(test)]
    pub(super) fn noise_print_requires_generated_vbic_mechanisms(
        print: &XycePrintRequest,
        netlist: &Netlist,
    ) -> Result<bool, String> {
        Ok(!Self::noise_print_generated_vbic_mechanism_requirements(print, netlist)?.is_empty())
    }

    /// Ask the linked generated-model registry for the actual noise descriptor
    /// table. The registry currently exposes descriptors on an instance, so a
    /// default instance and its node/branch index vectors are allocated for
    /// this admission-time query. This is intentionally a cold-path probe run
    /// once per candidate deck, never part of frequency-point evaluation.
    fn linked_builtin_noise_descriptors(
        model_name: &str,
    ) -> Result<
        Option<&'static [rspice_core::device::veriloga_builtins::GeneratedNoiseDescriptor]>,
        String,
    > {
        use rspice_core::device::veriloga_builtins::builtins;

        let Some(canonical_name) = builtins::builtin_names()
            .iter()
            .copied()
            .find(|name| name.eq_ignore_ascii_case(model_name))
        else {
            return Ok(None);
        };
        let node_count = builtins::total_node_count(canonical_name).ok_or_else(|| {
            format!("generated registry omitted node metadata for '{canonical_name}'")
        })?;
        let branch_count = builtins::branch_count(canonical_name).ok_or_else(|| {
            format!("generated registry omitted branch metadata for '{canonical_name}'")
        })?;
        let nodes = vec![0; node_count];
        let branches = vec![0; branch_count];
        let device = builtins::instantiate_scoped(canonical_name, &nodes, &branches, &[])
            .map_err(|error| {
                format!(
                    "generated registry could not instantiate '{canonical_name}' for noise capability inspection: {error}"
                )
            })?
            .ok_or_else(|| {
                format!(
                    "generated registry advertised '{canonical_name}' but could not instantiate it"
                )
            })?;
        Ok(Some(device.noise_descriptors()))
    }

    pub(super) fn generated_vbic_noise_issue(
        print: &XycePrintRequest,
        netlist: &Netlist,
    ) -> Result<Option<XyceGeneratedVbicNoiseIssue>, String> {
        let mut descriptor_cache = std::collections::HashMap::new();
        for (model_name, mechanism) in
            Self::noise_print_generated_vbic_mechanism_requirements(print, netlist)?
        {
            let descriptors = match descriptor_cache.entry(model_name) {
                std::collections::hash_map::Entry::Occupied(entry) => *entry.get(),
                std::collections::hash_map::Entry::Vacant(entry) => {
                    *entry.insert(Self::linked_builtin_noise_descriptors(model_name)?)
                }
            };
            let Some(descriptors) = descriptors else {
                return Ok(Some(XyceGeneratedVbicNoiseIssue::ModelUnavailable {
                    model_name,
                    mechanism,
                }));
            };
            if descriptors.is_empty() {
                return Ok(Some(
                    XyceGeneratedVbicNoiseIssue::NoiseDescriptorsUnavailable {
                        model_name,
                        mechanism,
                    },
                ));
            }
            if !descriptors
                .iter()
                .any(|descriptor| descriptor.mechanism.eq_ignore_ascii_case(&mechanism))
            {
                return Ok(Some(XyceGeneratedVbicNoiseIssue::UnknownMechanism {
                    model_name,
                    mechanism,
                }));
            }
        }
        Ok(None)
    }

    pub(super) fn dc_sensitivity_output_schema(
        format: Option<&str>,
    ) -> Result<(XyceDcSensitivityReferenceFormat, bool), String> {
        let normalized = format.unwrap_or("STD").trim();
        if normalized.eq_ignore_ascii_case("CSV") {
            return Ok((XyceDcSensitivityReferenceFormat::Csv, true));
        }
        if normalized.eq_ignore_ascii_case("NOINDEX") {
            return Ok((XyceDcSensitivityReferenceFormat::Prn, true));
        }
        if normalized.eq_ignore_ascii_case("STD")
            || matches!(
                normalized.to_ascii_lowercase().as_str(),
                "raw" | "raw_ascii" | "probe" | "gnuplot" | "splot" | "dakota"
            )
        {
            return Ok((XyceDcSensitivityReferenceFormat::Prn, false));
        }
        Err(format!(
            "native static DC sensitivity contract does not cover .PRINT SENS FORMAT={normalized}"
        ))
    }

    pub(super) fn print_probe_contains_subcircuit_node_voltage_probe(probe: &str) -> bool {
        let expression = Self::print_expression_inner(probe).unwrap_or(probe);
        let normalized = Self::normalize_probe(expression);
        if Self::voltage_probe_targets_subcircuit_node(&normalized) {
            return true;
        }

        let context = rspice_core::netlist::ParamContext::new();
        let mut found = false;
        let _ = Self::rewrite_print_expression_calls_maybe(expression, context, |call| {
            if Self::voltage_probe_targets_subcircuit_node(&Self::normalize_probe(call)) {
                found = true;
            }
            Ok(0.0)
        });
        found
    }

    pub(super) fn source_enables_constant_time_step_output(source: &str) -> bool {
        Self::logical_netlist_lines(source).iter().any(|line| {
            let normalized = Self::strip_netlist_comment(line)
                .chars()
                .filter(|ch| !ch.is_whitespace())
                .collect::<String>()
                .to_ascii_lowercase();
            normalized.starts_with(".options")
                && normalized.contains("timeint")
                && normalized.contains("conststep")
        })
    }

    pub(super) fn source_requests_ac_phase_output_radians(source: &str) -> bool {
        let mut enabled = false;
        for line in Self::logical_netlist_lines(source) {
            let normalized = Self::strip_netlist_comment(&line)
                .chars()
                .filter(|ch| !ch.is_whitespace())
                .collect::<String>()
                .to_ascii_lowercase();
            if !normalized.starts_with(".options") || !normalized.contains("phase_output_radians") {
                continue;
            }
            if normalized.contains("phase_output_radians=false")
                || normalized.contains("phase_output_radians=0")
                || normalized.contains("phase_output_radians=no")
            {
                enabled = false;
                continue;
            }
            enabled = true;
        }
        enabled
    }

    pub(super) fn source_requests_ac_print_headerless(source: &str) -> bool {
        let mut enabled = false;
        for line in Self::logical_netlist_lines(source) {
            let normalized = Self::strip_netlist_comment(&line)
                .chars()
                .filter(|ch| !ch.is_whitespace())
                .collect::<String>()
                .to_ascii_lowercase();
            if !normalized.starts_with(".options") || !normalized.contains("printheader") {
                continue;
            }
            if normalized.contains("printheader=false")
                || normalized.contains("printheader=0")
                || normalized.contains("printheader=no")
                || normalized.contains("printheader=off")
            {
                enabled = true;
                continue;
            }
            if normalized.contains("printheader=true")
                || normalized.contains("printheader=1")
                || normalized.contains("printheader=yes")
                || normalized.contains("printheader=on")
            {
                enabled = false;
            }
        }
        enabled
    }

    pub(super) fn ac_ic_print_format_is_supported(format: &str) -> bool {
        Self::ac_print_format_is_prn_compatible(format)
            || format.eq_ignore_ascii_case("CSV")
            || format.eq_ignore_ascii_case("PROBE")
    }

    pub(super) fn print_option_assignment<'a>(
        tokens: &'a [&'a str],
        index: usize,
    ) -> Option<(&'a str, &'a str, usize)> {
        let token = tokens.get(index).copied()?;
        if let Some((key, value)) = token.split_once('=') {
            if value.is_empty() && token.ends_with('=') {
                return Some((key, tokens.get(index + 1).copied()?, 2));
            }
            return Some((key, value, 1));
        }
        if token.ends_with('=') {
            return Some((
                token.trim_end_matches('='),
                tokens.get(index + 1).copied()?,
                2,
            ));
        }
        if tokens.get(index + 1).copied() == Some("=") {
            return Some((token, tokens.get(index + 2).copied()?, 3));
        }
        None
    }

    pub(super) fn wrapper_source_has_extra_output_analysis(source: &str) -> bool {
        Self::logical_netlist_lines(source).into_iter().any(|line| {
            let trimmed = Self::strip_netlist_comment(&line).trim().to_string();
            let Some(command) = trimmed.split_whitespace().next() else {
                return false;
            };
            Self::is_extra_wrapper_output_analysis_command(command)
        })
    }

    pub(super) fn passed_or_tran_side_output_failure(
        &self,
        deck: &XyceDeck,
        start: Instant,
        contract: &str,
        plan: &XyceStaticTranPlan,
        netlist: &Netlist,
        result: &TransientResult,
    ) -> XyceTestResult {
        match self.compare_tran_side_outputs(plan, netlist, result) {
            Ok(mismatches) if mismatches.is_empty() => self.passed_result(deck, start, contract),
            Ok(mismatches) => self.failure_result(
                deck,
                start,
                contract,
                format!(
                    "{} Xyce transient side-output mismatch(es)",
                    mismatches.len()
                ),
                mismatches,
            ),
            Err(err) => self.failure_result(
                deck,
                start,
                contract,
                format!("transient side-output comparison error: {err}"),
                Vec::new(),
            ),
        }
    }

    pub(super) fn passed_or_step_tran_side_output_failure(
        &self,
        deck: &XyceDeck,
        start: Instant,
        contract: &str,
        plan: &XyceStaticTranPlan,
        step_runs: &[XyceStepRun],
        abort: &dyn AbortSignal,
        locked_time_grid: bool,
    ) -> XyceTestResult {
        match self.compare_step_tran_side_outputs(plan, step_runs, abort, locked_time_grid) {
            Ok(mismatches) if mismatches.is_empty() => self.passed_result(deck, start, contract),
            Ok(mismatches) => self.failure_result(
                deck,
                start,
                contract,
                format!(
                    "{} Xyce stepped transient side-output mismatch(es)",
                    mismatches.len()
                ),
                mismatches,
            ),
            Err(err) => self.failure_result(
                deck,
                start,
                contract,
                format!("stepped transient side-output comparison error: {err}"),
                Vec::new(),
            ),
        }
    }

    pub(super) fn xyce_verify_transient_output_times(
        plan: &XyceStaticTranPlan,
        netlist: &Netlist,
        result: &TransientResult,
    ) -> Result<Vec<Value>, String> {
        let output_start = plan.tran.start.unwrap_or(0.0).max(0.0);
        let time_epsilon = 16.0 * Value::EPSILON * plan.tran.stop.abs().max(1.0);
        if !netlist.options.output_time_points.is_empty() {
            let projection = result.output_projection(
                &netlist.options.output_time_points,
                output_start,
                plan.tran.stop,
            )?;
            return projection.project(&result.time);
        }
        if let Some((initial_interval, transitions)) = Self::output_interval_schedule(&plan.source)?
        {
            let mut times = Vec::new();
            let mut next_time = 0.0;
            let mut interval = initial_interval;
            let mut boundaries = transitions;
            boundaries.push((plan.tran.stop, interval));
            for (boundary, next_interval) in boundaries {
                let segment_end = boundary.min(plan.tran.stop);
                while next_time < segment_end - time_epsilon {
                    if next_time + time_epsilon >= output_start {
                        times.push(next_time);
                        if times.len() > MAX_NATIVE_TRAN_ORACLE_STEPS as usize {
                            return Err(format!(
                                "OUTPUT schedule requests more than {:.0} serialized transient rows",
                                MAX_NATIVE_TRAN_ORACLE_STEPS
                            ));
                        }
                    }
                    next_time += interval;
                }
                if boundary > plan.tran.stop + time_epsilon {
                    break;
                }
                if boundary + time_epsilon >= output_start
                    && times
                        .last()
                        .is_none_or(|last| (boundary - *last).abs() > time_epsilon)
                {
                    times.push(boundary);
                }
                interval = next_interval;
                next_time = boundary + interval;
            }
            if times
                .last()
                .is_none_or(|last| (plan.tran.stop - *last).abs() > time_epsilon)
            {
                times.push(plan.tran.stop);
            }
            if times.is_empty() {
                return Err("transient output schedule produced no times".to_string());
            }
            return Ok(times);
        }

        let times = result
            .time
            .iter()
            .copied()
            .filter(|time| *time + time_epsilon >= output_start)
            .collect::<Vec<_>>();
        if times.is_empty() {
            return Err(format!(
                "transient result has no serialized sample at or after TSTART={output_start}"
            ));
        }
        Ok(times)
    }

    pub(super) fn analytic_sinusoidal_rc_print_expression(
        probe: &str,
    ) -> Result<(String, Value), String> {
        let expression = Self::print_expression_inner(probe).ok_or_else(|| {
            "analytic sinusoidal RC print probe must be one braced expression".to_string()
        })?;
        if expression.chars().any(char::is_whitespace) {
            return Err(
                "analytic sinusoidal RC expression must remain one whitespace-free wrapper token"
                    .to_string(),
            );
        }
        let ast = parse_expression_strict(expression)
            .map_err(|err| format!("could not parse analytic sinusoidal RC expression: {err}"))?;
        let Expr::Binary {
            op: rspice_core::expr::BinaryOp::Add,
            left,
            right,
        } = ast
        else {
            return Err(
                "analytic sinusoidal RC expression must directly add V(output) and an offset"
                    .to_string(),
            );
        };
        let Expr::NodeVoltage(node) = left.as_ref() else {
            return Err(
                "analytic sinusoidal RC expression left operand must be one direct V(output) probe"
                    .to_string(),
            );
        };
        if !Self::is_single_spice_node_token(node) {
            return Err(
                "analytic sinusoidal RC expression voltage node must be one direct node token"
                    .to_string(),
            );
        }
        let Expr::Const(offset) = right.as_ref() else {
            return Err(
                "analytic sinusoidal RC expression offset must be one direct numeric literal"
                    .to_string(),
            );
        };
        if !offset.is_finite() {
            return Err("analytic sinusoidal RC expression offset must be finite".to_string());
        }
        Ok((Self::canonical_passive_primary_node_name(node), *offset))
    }

    pub(super) fn passive_temperature_nonpassive_fingerprint(
        element: &rspice_core::netlist::Element,
        nodes: Vec<String>,
    ) -> Result<XyceRelationalElementFingerprint, String> {
        let (kind, numeric_bits, text) = match &element.kind {
            ElementKind::Resistor {
                value,
                value_expr,
                model,
                instance_params,
                deferred_params,
            } if nodes.len() == 2
                && value.is_finite()
                && *value > 0.0
                && value_expr.is_none()
                && model.is_none()
                && instance_params.is_empty()
                && deferred_params.is_empty() =>
            {
                ("R".to_string(), vec![value.to_bits()], Vec::new())
            }
            ElementKind::VoltageSource(spec) | ElementKind::CurrentSource(spec)
                if nodes.len() == 2 =>
            {
                let (waveform, values) = Self::scoped_model_source_fingerprint(spec)?;
                let prefix = if matches!(&element.kind, ElementKind::VoltageSource(_)) {
                    "V"
                } else {
                    "I"
                };
                (format!("{prefix}:{waveform}"), values, Vec::new())
            }
            ElementKind::BehavioralVoltage {
                expression,
                tc1,
                tc2,
                multiplicity,
            }
            | ElementKind::BehavioralCurrent {
                expression,
                tc1,
                tc2,
                multiplicity,
            } if nodes.len() == 2
                && tc1.to_bits() == 0.0f64.to_bits()
                && tc2.to_bits() == 0.0f64.to_bits()
                && multiplicity.value.to_bits() == 1.0f64.to_bits()
                && multiplicity.value_expr.is_none()
                && !multiplicity.given =>
            {
                let prepared = rspice_core::netlist::expr::prepare_behavioral_expression(
                    expression,
                    &rspice_core::netlist::expr::ParamContext::new(),
                )
                .map_err(|err| {
                    format!(
                        "could not canonicalize passive temperature family source '{}': {err}",
                        element.name
                    )
                })?;
                let kind = if matches!(&element.kind, ElementKind::BehavioralVoltage { .. }) {
                    "BV"
                } else {
                    "BI"
                };
                (
                    kind.to_string(),
                    vec![tc1.to_bits(), tc2.to_bits()],
                    vec![prepared.trim().to_ascii_lowercase()],
                )
            }
            _ => {
                return Err(format!(
                    "passive temperature-coefficient override family contains unqualified non-passive element '{}'",
                    element.name
                ));
            }
        };
        Ok(XyceRelationalElementFingerprint {
            kind,
            nodes,
            numeric_bits,
            text,
        })
    }

    pub(super) fn ac_analysis_output_option_is_footer_suppression(
        statement: &str,
    ) -> Result<bool, String> {
        let fields = Self::split_grouped_whitespace_fields(statement, ".OPTIONS OUTPUT statement")?;
        if fields.len() != 3
            || !fields[0].eq_ignore_ascii_case(".OPTIONS")
            || !fields[1].eq_ignore_ascii_case("OUTPUT")
        {
            return Ok(false);
        }
        let Some((name, value)) = fields[2].split_once('=') else {
            return Ok(false);
        };
        Ok(name.eq_ignore_ascii_case("PRINTFOOTER")
            && matches!(
                value.to_ascii_lowercase().as_str(),
                "false" | "0" | "no" | "off"
            ))
    }

    pub(super) fn expression_ast_fingerprint(
        expression: &rspice_core::netlist::expr::Expr,
    ) -> XyceExpressionAstFingerprint {
        use rspice_core::netlist::expr::Expr as NetlistExpr;
        match expression {
            NetlistExpr::Number(value) => XyceExpressionAstFingerprint::Number(value.to_bits()),
            NetlistExpr::ComplexNumber(value) => {
                XyceExpressionAstFingerprint::Complex(value.re.to_bits(), value.im.to_bits())
            }
            NetlistExpr::StringLiteral(value) => {
                XyceExpressionAstFingerprint::String(value.clone())
            }
            NetlistExpr::Param(name) => {
                XyceExpressionAstFingerprint::Parameter(name.trim().to_ascii_lowercase())
            }
            NetlistExpr::BinOp { op, left, right } => XyceExpressionAstFingerprint::Binary(
                *op,
                Box::new(Self::expression_ast_fingerprint(left)),
                Box::new(Self::expression_ast_fingerprint(right)),
            ),
            NetlistExpr::UnaryOp { op, operand } => XyceExpressionAstFingerprint::Unary(
                *op,
                Box::new(Self::expression_ast_fingerprint(operand)),
            ),
            NetlistExpr::FnCall { name, args } => XyceExpressionAstFingerprint::Function(
                name.trim().to_ascii_lowercase(),
                args.iter().map(Self::expression_ast_fingerprint).collect(),
            ),
        }
    }

    pub(super) fn transient_analysis_source_fingerprint(
        element: &rspice_core::netlist::Element,
        spec: &rspice_core::netlist::SourceSpec,
        nodes: Vec<String>,
    ) -> Result<XyceRelationalElementFingerprint, String> {
        let (waveform, numeric_bits) = match spec {
            rspice_core::netlist::SourceSpec::Dc(value) if value.is_finite() => {
                ("DC", vec![value.to_bits()])
            }
            rspice_core::netlist::SourceSpec::Pulse {
                v1,
                v2,
                delay,
                rise,
                fall,
                width,
                period,
                phase,
                width_defaults_to_zero,
            } if [*v1, *v2, *delay, *rise, *fall, *width, *period, *phase]
                .into_iter()
                .all(Value::is_finite)
                && *delay >= 0.0
                && *rise >= 0.0
                && *fall >= 0.0
                && *width >= 0.0
                && *period > 0.0 =>
            {
                (
                    "PULSE",
                    vec![
                        v1.to_bits(),
                        v2.to_bits(),
                        delay.to_bits(),
                        rise.to_bits(),
                        fall.to_bits(),
                        width.to_bits(),
                        period.to_bits(),
                        phase.to_bits(),
                        u64::from(*width_defaults_to_zero),
                    ],
                )
            }
            rspice_core::netlist::SourceSpec::Sin {
                offset,
                amplitude,
                frequency,
                delay,
                damping,
                phase,
            } if [*offset, *amplitude, *frequency, *delay, *damping, *phase]
                .into_iter()
                .all(Value::is_finite)
                && *frequency > 0.0
                && *delay >= 0.0 =>
            {
                (
                    "SIN",
                    vec![
                        offset.to_bits(),
                        amplitude.to_bits(),
                        frequency.to_bits(),
                        delay.to_bits(),
                        damping.to_bits(),
                        phase.to_bits(),
                    ],
                )
            }
            _ => {
                return Err(format!(
                    "transient-analysis expression source '{}' must be a finite direct DC, PULSE, or SIN source",
                    element.name
                ));
            }
        };
        let prefix = if matches!(&element.kind, ElementKind::VoltageSource(_)) {
            "V"
        } else {
            "I"
        };
        Ok(XyceRelationalElementFingerprint {
            kind: format!("{prefix}:{waveform}"),
            nodes,
            numeric_bits,
            text: Vec::new(),
        })
    }

    pub(super) fn transient_print_requests_linear_capacitor_branch_quantity(
        netlist: &Netlist,
        print: &XycePrintRequest,
    ) -> bool {
        print.probes.iter().any(|probe| {
            Self::transient_probe_requests_linear_capacitor_branch_quantity(netlist, probe)
        })
    }

    pub(super) fn xyce_dc_sensitivity_output_from_spec(
        spec: &XyceAcSensitivityObjectiveSpec,
        result: &rspice_core::solver::SimulationResult,
    ) -> Result<AcSensitivityOutput, String> {
        let node_index = |node: &str| {
            if node.eq_ignore_ascii_case("0") {
                return Ok(0usize);
            }
            result
                .node_names
                .iter()
                .position(|name| Self::normalize_probe(name) == Self::normalize_probe(node))
                .ok_or_else(|| {
                    format!("DC sensitivity objective node '{node}' is not in the solved circuit")
                })
        };
        match spec {
            XyceAcSensitivityObjectiveSpec::Voltage { positive, negative } => {
                let positive = node_index(positive)?;
                let negative = negative.as_deref().map(node_index).transpose()?;
                if positive == 0 {
                    return Err("DC sensitivity objective voltage must not be ground".to_string());
                }
                if negative == Some(positive) {
                    return Err("DC sensitivity objective voltage nodes must differ".to_string());
                }
                Ok(AcSensitivityOutput::Voltage { positive, negative })
            }
            XyceAcSensitivityObjectiveSpec::BranchCurrent(element) => {
                Ok(AcSensitivityOutput::BranchCurrent(element.clone()))
            }
        }
    }

    pub(super) fn xyce_sensitivity_output_from_spec(
        spec: &XyceAcSensitivityObjectiveSpec,
        result: &AcResult,
    ) -> Result<AcSensitivityOutput, String> {
        let node_index = |node: &str| {
            if node.eq_ignore_ascii_case("0") {
                return Ok(0usize);
            }
            result
                .node_names
                .iter()
                .position(|name| Self::normalize_probe(name) == node)
                .map(|index| index + 1)
                .ok_or_else(|| {
                    format!("AC sensitivity objective node '{node}' is not in the solved circuit")
                })
        };
        match spec {
            XyceAcSensitivityObjectiveSpec::Voltage { positive, negative } => {
                let positive = node_index(positive)?;
                let negative = negative.as_deref().map(node_index).transpose()?;
                if positive == 0 {
                    return Err("AC sensitivity objective voltage must not be ground".to_string());
                }
                if negative == Some(positive) {
                    return Err("AC sensitivity objective voltage nodes must differ".to_string());
                }
                Ok(AcSensitivityOutput::Voltage { positive, negative })
            }
            XyceAcSensitivityObjectiveSpec::BranchCurrent(element) => {
                Ok(AcSensitivityOutput::BranchCurrent(element.clone()))
            }
        }
    }

    pub(super) fn noise_measurement_output_traces(
        netlist: &Netlist,
        results: &[rspice_core::analysis::NoiseResult],
        columns: &[XyceAcReferenceColumn],
    ) -> Result<BTreeMap<String, Vec<(Value, Value)>>, String> {
        let requested = columns
            .iter()
            .filter(|column| column.component() == XyceAcProbeComponent::Scalar)
            .map(|column| column.probe_name().to_ascii_uppercase())
            .collect::<BTreeSet<_>>();
        let mut traces = BTreeMap::new();
        for (statement_index, statement) in netlist.measurements.iter().enumerate() {
            let normalized_name = statement.name.to_ascii_uppercase();
            if !requested.contains(&normalized_name)
                || !(statement.analysis.eq_ignore_ascii_case("NOISE")
                    || statement.analysis.eq_ignore_ascii_case("NOISE_CONT"))
            {
                continue;
            }
            // Equation measures have a dedicated point-by-point evaluator
            // whose ordered state can depend on earlier equations. They must
            // not be coerced through the point-event evaluator below.
            if matches!(
                statement.measure_type,
                rspice_core::analysis::MeasureType::Equation { .. }
            ) {
                continue;
            }
            let mut trace_netlist = netlist.clone();
            let trace_statement = &mut trace_netlist.measurements[statement_index];
            trace_statement.analysis = "NOISE_CONT".to_string();
            // A negative qualifier is a rolling "Nth event from the end" in
            // Xyce's live output state.  Evaluate the complete event stream,
            // then lag its values below; evaluating the final scalar result
            // would discard the earlier states printed during the sweep.
            let occurrence = match &mut trace_statement.measure_type {
                rspice_core::analysis::MeasureType::When { condition, .. }
                | rspice_core::analysis::MeasureType::Find {
                    when: Some(condition),
                    ..
                }
                | rspice_core::analysis::MeasureType::Derivative {
                    when: Some(condition),
                    ..
                } => {
                    let number = condition.occurrence.number;
                    if number < 0 {
                        condition.occurrence.number = 1;
                    }
                    Some(number)
                }
                _ => None,
            };
            let result_index = trace_netlist.measurements[..=statement_index]
                .iter()
                .filter(|candidate| candidate.analysis.eq_ignore_ascii_case("NOISE_CONT"))
                .count()
                - 1;
            let evaluated = rspice_core::analysis::evaluate_noise_continuous_measurements(
                &trace_netlist,
                results,
            );
            let measurement = evaluated.get(result_index).ok_or_else(|| {
                format!(
                    "NOISE measurement trace evaluator omitted declaration '{}'",
                    statement.name
                )
            })?;
            measurement.validate_invariants().map_err(|error| {
                format!(
                    "NOISE measurement trace '{}' violates its result invariant: {error}",
                    statement.name
                )
            })?;
            if let Some(failure) = &measurement.failure {
                if measurement.records.is_empty()
                    && Self::continuous_measurement_failure_is_uninitialized(
                        &statement.measure_type,
                        failure,
                    )
                {
                    traces.insert(normalized_name, Vec::new());
                    continue;
                }
                return Err(format!(
                    "NOISE measurement trace '{}' failed: {failure}",
                    statement.name
                ));
            }
            let events = measurement
                .records
                .iter()
                .map(|record| {
                    let activation_axis = record.event_axis.or_else(|| {
                        match (record.trigger_axis, record.target_axis) {
                            (Some(trigger), Some(target)) => Some(trigger.max(target)),
                            (None, Some(target)) => Some(target),
                            (Some(trigger), None) => Some(trigger),
                            (None, None) => None,
                        }
                    });
                    activation_axis
                        .map(|axis| (axis, record.value))
                        .ok_or_else(|| {
                            format!(
                                "NOISE measurement trace '{}' has no activation-axis metadata",
                                statement.name
                            )
                        })
                })
                .collect::<Result<Vec<_>, _>>()?;
            let trace = if occurrence.is_some_and(|number| number < 0) {
                let number = occurrence.expect("negative occurrence was checked");
                let lag = number
                    .checked_abs()
                    .and_then(|number| usize::try_from(number - 1).ok())
                    .ok_or_else(|| {
                        format!(
                            "NOISE measurement '{}' has an unrepresentable occurrence {number}",
                            statement.name
                        )
                    })?;
                events
                    .iter()
                    .skip(lag)
                    .zip(&events)
                    .map(|((activation_axis, _), (_, value))| (*activation_axis, *value))
                    .collect()
            } else if statement.analysis.eq_ignore_ascii_case("NOISE") {
                events.into_iter().take(1).collect()
            } else {
                events
            };
            traces.insert(normalized_name, trace);
        }
        Ok(traces)
    }

    pub(super) fn measurement_output_traces<'a, F>(
        netlist: &Netlist,
        accepted_axis: &[Value],
        requested_names: impl IntoIterator<Item = &'a str>,
        scalar_analysis: &str,
        continuous_analysis: &str,
        segment_starts: &[usize],
        mut evaluate_continuous: F,
    ) -> Result<BTreeMap<String, Vec<(usize, Value)>>, String>
    where
        F: FnMut(&Netlist) -> Vec<rspice_core::analysis::ContinuousMeasureResult>,
    {
        let requested = requested_names
            .into_iter()
            .map(str::to_ascii_uppercase)
            .collect::<BTreeSet<_>>();
        let mut traces = BTreeMap::new();

        for (statement_index, statement) in netlist.measurements.iter().enumerate() {
            let normalized_name = statement.name.to_ascii_uppercase();
            if !requested.contains(&normalized_name)
                || !(statement.analysis.eq_ignore_ascii_case(scalar_analysis)
                    || statement.analysis.eq_ignore_ascii_case(continuous_analysis))
            {
                continue;
            }
            // Equation measures are projected by the caller's dedicated
            // ordered equation-trace evaluator. Coercing them to *_CONT here
            // would manufacture a point-event semantic failure.
            if matches!(
                statement.measure_type,
                rspice_core::analysis::MeasureType::Equation { .. }
            ) {
                continue;
            }

            let mut trace_netlist = netlist.clone();
            let trace_statement = &mut trace_netlist.measurements[statement_index];
            trace_statement.analysis = continuous_analysis.to_string();
            // Xyce's live state for a negative occurrence is a rolling Nth
            // event from the end of the events accepted so far.  Ask the
            // continuous evaluator for the complete stream and apply that
            // lag after mapping events to accepted-point traversal order.
            let occurrence = match &mut trace_statement.measure_type {
                rspice_core::analysis::MeasureType::When { condition, .. }
                | rspice_core::analysis::MeasureType::Find {
                    when: Some(condition),
                    ..
                }
                | rspice_core::analysis::MeasureType::Derivative {
                    when: Some(condition),
                    ..
                } => {
                    let number = condition.occurrence.number;
                    if number < 0 {
                        condition.occurrence.number = 1;
                    }
                    Some(number)
                }
                _ => None,
            };
            let result_index = trace_netlist.measurements[..=statement_index]
                .iter()
                .filter(|candidate| candidate.analysis.eq_ignore_ascii_case(continuous_analysis))
                .count()
                - 1;
            let evaluated = evaluate_continuous(&trace_netlist);
            let measurement = evaluated.get(result_index).ok_or_else(|| {
                format!(
                    "{scalar_analysis} measurement trace evaluator omitted declaration '{}'",
                    statement.name
                )
            })?;
            measurement.validate_invariants().map_err(|error| {
                format!(
                    "{scalar_analysis} measurement trace '{}' violates its result invariant: {error}",
                    statement.name
                )
            })?;
            if let Some(failure) = &measurement.failure {
                // A valid point-event measure that never initializes remains
                // printable as zero. Semantic failures (unsupported measure
                // types, missing signals, malformed qualifiers, invalid data)
                // must fail closed instead of becoming an all-zero trace.
                if measurement.records.is_empty()
                    && Self::continuous_measurement_failure_is_uninitialized(
                        &statement.measure_type,
                        failure,
                    )
                {
                    traces.insert(normalized_name, Vec::new());
                    continue;
                }
                return Err(format!(
                    "{scalar_analysis} measurement trace '{}' failed: {failure}",
                    statement.name
                ));
            }

            let mut minimum_index = 0usize;
            let mut events = Vec::with_capacity(measurement.records.len());
            for record in &measurement.records {
                let activation_index = Self::continuous_record_activation_index(
                    accepted_axis,
                    segment_starts,
                    record,
                    minimum_index,
                )
                .ok_or_else(|| {
                    format!(
                        "{scalar_analysis} measurement trace '{}' event metadata does not map to the accepted-point traversal",
                        statement.name
                    )
                })?;
                events.push((activation_index, record.value));
                minimum_index = activation_index.saturating_add(1);
            }

            let trace = if occurrence.is_some_and(|number| number < 0) {
                let number = occurrence.expect("negative occurrence was checked");
                let lag = number
                    .checked_abs()
                    .and_then(|number| usize::try_from(number - 1).ok())
                    .ok_or_else(|| {
                        format!(
                            "{scalar_analysis} measurement '{}' has an unrepresentable occurrence {number}",
                            statement.name
                        )
                    })?;
                events
                    .iter()
                    .skip(lag)
                    .zip(&events)
                    .map(|((activation_index, _), (_, value))| (*activation_index, *value))
                    .collect()
            } else if statement.analysis.eq_ignore_ascii_case(scalar_analysis) {
                events.into_iter().take(1).collect()
            } else {
                events
            };
            traces.insert(normalized_name, trace);
        }

        Ok(traces)
    }

    pub(super) fn transient_probe_matches_output_interval_corridor(
        &self,
        probe: &str,
        netlist: &Netlist,
        result: &TransientResult,
        reference: &XycePrnTable,
        time_column: usize,
        row_index: usize,
        expected: Value,
        tolerance: XyceComparisonTolerance,
        output_interval: Value,
        time_scale_factor: Value,
    ) -> Result<bool, String> {
        if !expected.is_finite()
            || !output_interval.is_finite()
            || output_interval <= 0.0
            || !time_scale_factor.is_finite()
            || time_scale_factor <= 0.0
        {
            return Ok(false);
        }
        let Some(row) = reference.rows.get(row_index) else {
            return Ok(false);
        };
        let Some(time) = row
            .get(time_column)
            .copied()
            .map(|reference_time| reference_time / time_scale_factor)
        else {
            return Ok(false);
        };
        if !time.is_finite() {
            return Ok(false);
        }

        let lower_time = reference.rows[..row_index]
            .iter()
            .rev()
            .filter_map(|row| row.get(time_column).copied())
            .map(|reference_time| reference_time / time_scale_factor)
            .find(|candidate| candidate.is_finite() && *candidate < time);
        let upper_time = reference.rows[row_index + 1..]
            .iter()
            .filter_map(|row| row.get(time_column).copied())
            .map(|reference_time| reference_time / time_scale_factor)
            .find(|candidate| candidate.is_finite() && *candidate > time);
        let (Some(lower_time), Some(upper_time)) = (lower_time, upper_time) else {
            return Ok(false);
        };
        let window = upper_time - lower_time;
        if !window.is_finite() || window <= 0.0 || window > 2.5 * output_interval {
            return Ok(false);
        }

        let lower_value = Self::evaluate_tran_probe(probe, netlist, result, lower_time)?;
        let center_value = Self::evaluate_tran_probe(probe, netlist, result, time)?;
        let upper_value = Self::evaluate_tran_probe(probe, netlist, result, upper_time)?;
        if [lower_value, center_value, upper_value]
            .into_iter()
            .any(|candidate| {
                candidate.is_finite()
                    && self
                        .value_mismatch(expected, candidate, tolerance)
                        .is_none()
            })
        {
            return Ok(true);
        }

        let mut min_value = lower_value.min(center_value).min(upper_value);
        let mut max_value = lower_value.max(center_value).max(upper_value);
        for &sample_time in result.time.iter() {
            if sample_time <= lower_time || sample_time >= upper_time {
                continue;
            }
            let sample_value = Self::evaluate_tran_probe(probe, netlist, result, sample_time)?;
            if sample_value.is_finite() {
                min_value = min_value.min(sample_value);
                max_value = max_value.max(sample_value);
            }
        }

        Ok(min_value.is_finite()
            && max_value.is_finite()
            && expected >= min_value.min(max_value)
            && expected <= min_value.max(max_value))
    }

    pub(super) fn output_initial_interval(source: &str) -> Result<Option<Value>, String> {
        let mut interval: Option<Value> = None;
        for line in Self::logical_netlist_lines(source) {
            let trimmed = Self::strip_netlist_comment(&line).trim().to_string();
            if trimmed.is_empty() {
                continue;
            }
            let Some(command) = trimmed.split_whitespace().next() else {
                continue;
            };
            if !command.eq_ignore_ascii_case(".options") {
                continue;
            }

            let tokens = Self::split_grouped_whitespace_fields(&trimmed, ".OPTIONS statement")?;
            let token_refs = tokens.iter().map(String::as_str).collect::<Vec<_>>();
            let has_output_package = token_refs
                .iter()
                .skip(1)
                .any(|token| token.eq_ignore_ascii_case("output"));
            if !has_output_package {
                continue;
            }

            let mut index = 1usize;
            while index < token_refs.len() {
                if let Some((raw_key, raw_value, consumed)) =
                    Self::print_option_assignment(&token_refs, index)
                {
                    if raw_key.trim().eq_ignore_ascii_case("initial_interval") {
                        let parsed = rspice_core::netlist::lexer::parse_spice_value(
                            raw_value.trim().trim_matches(['"', '\'']),
                        )
                        .map_err(|err| {
                            format!(
                                "failed to parse OUTPUT INITIAL_INTERVAL value '{}': {err}",
                                raw_value.trim()
                            )
                        })?;
                        if !parsed.is_finite() || parsed <= 0.0 {
                            return Err(format!(
                                "OUTPUT INITIAL_INTERVAL must be positive and finite, got {parsed}"
                            ));
                        }
                        if let Some(existing) = interval {
                            let scale = existing.abs().max(parsed.abs()).max(1.0);
                            if (existing - parsed).abs() > 1.0e-12 * scale {
                                return Err(
                                    "conflicting OUTPUT INITIAL_INTERVAL options are not supported"
                                        .to_string(),
                                );
                            }
                        } else {
                            interval = Some(parsed);
                        }
                    }
                    index += consumed;
                } else {
                    index += 1;
                }
            }
        }

        Ok(interval)
    }

    pub(super) fn output_interval_schedule(source: &str) -> OutputIntervalSchedule {
        let Some(initial_interval) = Self::output_initial_interval(source)? else {
            return Ok(None);
        };
        let mut values = Vec::new();
        for line in Self::logical_netlist_lines(source) {
            let trimmed = Self::strip_netlist_comment(&line).trim();
            let tokens = Self::split_grouped_whitespace_fields(trimmed, ".OPTIONS statement")?;
            let token_refs = tokens.iter().map(String::as_str).collect::<Vec<_>>();
            if !token_refs
                .first()
                .is_some_and(|command| command.eq_ignore_ascii_case(".options"))
                || !token_refs
                    .iter()
                    .skip(1)
                    .any(|token| token.eq_ignore_ascii_case("output"))
            {
                continue;
            }
            let mut index = 1usize;
            while index < token_refs.len() {
                if let Some((_key, _value, consumed)) =
                    Self::print_option_assignment(&token_refs, index)
                {
                    index += consumed;
                    continue;
                }
                let token = token_refs[index].trim().trim_matches(['"', '\'']);
                if !token.eq_ignore_ascii_case("output")
                    && let Ok(value) = rspice_core::netlist::lexer::parse_spice_value(token)
                {
                    values.push(value);
                }
                index += 1;
            }
        }
        if values.len() % 2 != 0 {
            return Err(format!(
                "OUTPUT INITIAL_INTERVAL schedule requires time/interval pairs, found {} trailing numeric value(s)",
                values.len()
            ));
        }
        let mut transitions = Vec::with_capacity(values.len() / 2);
        let mut previous_time = Value::NEG_INFINITY;
        for pair in values.chunks_exact(2) {
            let time = pair[0];
            let interval = pair[1];
            if !time.is_finite() || time < 0.0 || time <= previous_time {
                return Err(format!(
                    "OUTPUT interval transition times must be finite, nonnegative, and strictly increasing, got {time} after {previous_time}"
                ));
            }
            if !interval.is_finite() || interval <= 0.0 {
                return Err(format!(
                    "OUTPUT interval after time {time} must be positive and finite, got {interval}"
                ));
            }
            transitions.push((time, interval));
            previous_time = time;
        }
        Ok(Some((initial_interval, transitions)))
    }

    pub(super) fn tran_print_time_scale_factor(source: &str) -> Result<Value, String> {
        let mut factor: Option<Value> = None;
        for line in Self::logical_netlist_lines(source) {
            let trimmed = Self::strip_netlist_comment(&line).trim().to_string();
            if trimmed.is_empty() {
                continue;
            }
            let Some(command) = trimmed.split_whitespace().next() else {
                continue;
            };
            if !command.eq_ignore_ascii_case(".print") {
                continue;
            }

            let tokens = Self::split_print_fields(&trimmed)?;
            let token_refs = tokens.iter().map(String::as_str).collect::<Vec<_>>();
            if token_refs
                .get(1)
                .is_none_or(|analysis| !analysis.eq_ignore_ascii_case("TRAN"))
            {
                continue;
            }

            let mut index = 2usize;
            let mut line_factor = None;
            while index < token_refs.len() {
                if let Some((raw_key, raw_value, consumed)) =
                    Self::print_option_assignment(&token_refs, index)
                {
                    if raw_key.trim().eq_ignore_ascii_case("TIMESCALEFACTOR") {
                        line_factor = Some(raw_value.trim().trim_matches(['"', '\'']).to_string());
                    }
                    index += consumed;
                } else {
                    break;
                }
            }
            let parsed = match line_factor {
                Some(raw_factor) => rspice_core::netlist::lexer::parse_spice_value(&raw_factor).map_err(
                    |err| {
                        format!(
                            "failed to parse .PRINT TRAN TIMESCALEFACTOR value '{raw_factor}': {err}"
                        )
                    },
                )?,
                None => 1.0,
            };
            if !parsed.is_finite() || parsed <= 0.0 {
                return Err(format!(
                    ".PRINT TRAN TIMESCALEFACTOR must be positive and finite, got {parsed}"
                ));
            }
            if let Some(existing) = factor {
                if existing.to_bits() != parsed.to_bits() {
                    return Err(
                        "different .PRINT TRAN output blocks use conflicting TIMESCALEFACTOR values"
                            .to_string(),
                    );
                }
            } else {
                factor = Some(parsed);
            }
        }

        Ok(factor.unwrap_or(1.0))
    }

    #[cfg(test)]
    pub(super) fn line_declares_output_snapshots(line: &str) -> Result<bool, String> {
        let tokens = Self::split_grouped_whitespace_fields(line, ".OPTIONS statement")?;
        let token_refs = tokens.iter().map(String::as_str).collect::<Vec<_>>();
        if !token_refs
            .iter()
            .skip(1)
            .any(|token| token.eq_ignore_ascii_case("output"))
        {
            return Ok(false);
        }

        let mut index = 1usize;
        while index < token_refs.len() {
            if let Some((raw_key, raw_value, consumed)) =
                Self::print_option_assignment(&token_refs, index)
            {
                if raw_key.trim().eq_ignore_ascii_case("snapshots") {
                    let value = raw_value.trim().trim_matches(['"', '\'']);
                    return Ok(!matches!(
                        value.to_ascii_lowercase().as_str(),
                        "0" | "false" | "no" | "off"
                    ));
                }
                index += consumed;
            } else {
                index += 1;
            }
        }

        Ok(false)
    }

    pub(super) fn print_requests_complex_ac_probe(print: &XycePrintRequest, probe: &str) -> bool {
        let normalized_probe = Self::normalize_ac_expression_probe_key(probe);
        print
            .probes
            .iter()
            .any(|requested| Self::normalize_ac_expression_probe_key(requested) == normalized_probe)
    }

    pub(super) fn print_requests_scalar_ac_probe(print: &XycePrintRequest, column: &str) -> bool {
        let normalized_column = Self::normalize_probe(column);
        let normalized_expression_column = Self::normalize_ac_expression_probe_key(column);
        print.probes.iter().any(|requested| {
            Self::normalize_probe(requested) == normalized_column
                || Self::normalize_ac_expression_probe_key(requested)
                    == normalized_expression_column
        })
    }

    pub(super) fn stateful_tran_print_expression(
        probe: &str,
        netlist: &Netlist,
    ) -> Result<Option<StatefulTranPrintExpression>, String> {
        let Some(expression) = Self::print_expression_inner(probe) else {
            return Ok(None);
        };
        if !Self::print_expression_contains_stateful_sdt(expression, &netlist.params) {
            return Ok(None);
        }
        let (expression, context, _) = Self::rewrite_print_device_parameter_tokens_maybe(
            expression,
            netlist.params.clone(),
            |token| Self::static_transient_device_parameter_value(netlist, token),
        )?;
        let prepared = prepare_behavioral_expression(&expression, &context).map_err(|err| {
            format!("could not prepare stateful .PRINT TRAN expression '{{{expression}}}': {err}")
        })?;
        let ast = parse_expression_strict(&prepared).map_err(|err| {
            format!("could not parse stateful .PRINT TRAN expression '{{{expression}}}': {err}")
        })?;
        if !Self::expression_contains_sdt(&ast) {
            return Ok(None);
        }
        Ok(Some(StatefulTranPrintExpression {
            program: compile(&ast),
            vm: Vm::new(),
        }))
    }

    pub(super) fn evaluate_stateful_tran_print_expression(
        runtime: &mut StatefulTranPrintExpression,
        netlist: &Netlist,
        result: &TransientResult,
        time: Value,
    ) -> Result<Value, String> {
        let mut voltages = vec![0.0; runtime.program.node_map.len()];
        for (name, &index) in &runtime.program.node_map {
            voltages[index] = Self::transient_voltage_named(result, netlist, name, time)?;
        }
        let mut currents = vec![0.0; runtime.program.branch_map.len()];
        for (name, &index) in &runtime.program.branch_map {
            currents[index] = Self::transient_branch_current_named(result, name, time)
                .or_else(|| {
                    Self::evaluate_independent_current_source_probe(netlist, result, name, time)
                })
                .ok_or_else(|| format!("branch current '{name}' not found in transient result"))?;
        }
        let context = Context::transient(&voltages, &currents, time)
            .with_expression_dialect(ExpressionDialect::Xyce);
        let value = runtime.vm.execute(&runtime.program, &context);
        runtime.vm.accept_transient_step(time);
        Ok(value)
    }

    pub(super) fn evaluate_print_expression_with_probe_calls<F>(
        expression: &str,
        context: rspice_core::netlist::ParamContext,
        call_value: &mut F,
    ) -> Result<Value, String>
    where
        F: FnMut(&str) -> Result<Value, String>,
    {
        Self::evaluate_print_expression_internal(expression, context, call_value, None)
    }

    pub(super) fn evaluate_print_expression_internal<F>(
        expression: &str,
        mut context: rspice_core::netlist::ParamContext,
        call_value: &mut F,
        override_probe: Option<(&str, Value)>,
    ) -> Result<Value, String>
    where
        F: FnMut(&str) -> Result<Value, String>,
    {
        if let Some((name, value)) = override_probe
            && let Some(parameter) = Self::parse_scalar_parameter_probe(name)
            && context.has_any_parameter_binding(&parameter)
        {
            context.set(&parameter, value);
        }
        let expression =
            rspice_core::netlist::expr::expand_output_user_functions(expression, &context)?;
        let (rewritten, context) =
            Self::rewrite_print_ddx_calls(&expression, context, call_value, override_probe)?;
        let (rewritten, context, _) =
            Self::rewrite_print_expression_calls_maybe(&rewritten, context, |call| {
                Self::print_probe_value(call, call_value, override_probe)
            })?;
        let (rewritten, context, _) =
            Self::rewrite_print_device_parameter_tokens_maybe(&rewritten, context, |call| {
                Self::print_probe_value(call, call_value, override_probe)
            })?;
        rspice_core::netlist::expr::eval_expression(&rewritten, &context)
            .map_err(|err| err.to_string())
    }

    pub(super) fn print_probe_value<F>(
        call: &str,
        call_value: &mut F,
        override_probe: Option<(&str, Value)>,
    ) -> Result<Value, String>
    where
        F: FnMut(&str) -> Result<Value, String>,
    {
        if let Some((override_name, override_value)) = override_probe
            && Self::normalize_probe(call) == override_name
        {
            return Ok(override_value);
        }
        call_value(call)
    }

    pub(super) fn rewrite_print_ddx_calls<F>(
        expression: &str,
        mut context: rspice_core::netlist::ParamContext,
        call_value: &mut F,
        override_probe: Option<(&str, Value)>,
    ) -> Result<(String, rspice_core::netlist::ParamContext), String>
    where
        F: FnMut(&str) -> Result<Value, String>,
    {
        let mut rewritten = String::with_capacity(expression.len());
        let mut index = 0usize;
        let mut placeholder_index = 0usize;

        while index < expression.len() {
            if expression[index..].starts_with('"') {
                let start = index;
                index += 1;
                let mut escaped = false;
                while index < expression.len() {
                    let character = expression[index..]
                        .chars()
                        .next()
                        .expect("valid char boundary");
                    index += character.len_utf8();
                    if escaped {
                        escaped = false;
                    } else if character == '\\' {
                        escaped = true;
                    } else if character == '"' {
                        break;
                    }
                }
                rewritten.push_str(&expression[start..index]);
                continue;
            }
            if let Some(open_index) = Self::print_ddx_call_open_index(expression, index) {
                let close_index = Self::matching_parenthesis_index(expression, open_index)?;
                let call = &expression[index..=close_index];
                let placeholder = format!("__rspice_ddx_{placeholder_index}");
                let value =
                    Self::evaluate_print_ddx_call(call, &context, call_value, override_probe)?;
                context.set(&placeholder, value);
                rewritten.push_str(&placeholder);
                placeholder_index += 1;
                index = close_index + 1;
                continue;
            }

            let ch = expression[index..]
                .chars()
                .next()
                .expect("valid char boundary");
            match ch {
                '{' => rewritten.push('('),
                '}' => rewritten.push(')'),
                _ => rewritten.push(ch),
            }
            index += ch.len_utf8();
        }

        Ok((rewritten, context))
    }

    pub(super) fn evaluate_print_ddx_call<F>(
        call: &str,
        context: &rspice_core::netlist::ParamContext,
        call_value: &mut F,
        override_probe: Option<(&str, Value)>,
    ) -> Result<Value, String>
    where
        F: FnMut(&str) -> Result<Value, String>,
    {
        let open_index = call
            .find('(')
            .ok_or_else(|| format!("malformed DDX call '{call}'"))?;
        let inner = call[open_index + 1..]
            .strip_suffix(')')
            .ok_or_else(|| format!("malformed DDX call '{call}'"))?;
        let args = Self::split_top_level_args(inner)?;
        if args.len() != 2 {
            return Err(format!(
                "DDX expects exactly two arguments, got {} in '{call}'",
                args.len()
            ));
        }

        let expression = args[0].trim();
        let contained_nested_ddx = Self::print_expression_contains_named_call(expression, "DDX");
        let variable = args[1].trim();
        if Self::parse_voltage_probe(variable).is_none()
            && Self::parse_current_probe(variable).is_none()
            && Self::parse_power_probe(variable).is_none()
            && Self::parse_lead_current_probe(variable).is_none()
            && Self::parse_device_operating_point_probe(variable).is_none()
            && Self::parse_device_parameter_probe(variable).is_none()
            && Self::parse_scalar_parameter_probe(variable).is_none()
        {
            return Err(format!(
                "DDX derivative variable '{variable}' is not a supported print probe"
            ));
        }

        let normalized_variable = Self::normalize_probe(variable);
        let (expression, context) =
            Self::rewrite_print_ddx_calls(expression, context.clone(), call_value, override_probe)?;
        let mut target_probe_placeholders = Vec::new();
        let mut probe_index = 0usize;
        let (expression, context, _) =
            Self::rewrite_print_expression_calls_maybe(&expression, context, |probe| {
                if Self::normalize_probe(probe) == normalized_variable {
                    target_probe_placeholders.push(format!("__rspice_probe_{probe_index}"));
                }
                probe_index = probe_index.saturating_add(1);
                Self::print_probe_value(probe, call_value, override_probe)
            })?;
        let (expression, context, _) =
            Self::rewrite_print_device_parameter_tokens_maybe(&expression, context, |probe| {
                Self::print_probe_value(probe, call_value, override_probe)
            })?;
        let targets = if let Some(parameter) = Self::parse_scalar_parameter_probe(variable) {
            vec![parameter]
        } else {
            if target_probe_placeholders.is_empty() {
                if contained_nested_ddx {
                    return Ok(0.0);
                }
                return Err(format!(
                    "DDX derivative variable '{variable}' is absent from its expression"
                ));
            }
            target_probe_placeholders
        };
        rspice_core::device::behavioral::evaluate_parameter_directional_derivative(
            &expression,
            &context,
            &targets,
        )
    }

    pub(super) fn print_ddx_call_open_index(expression: &str, index: usize) -> Option<usize> {
        if index >= expression.len() || !expression.is_char_boundary(index) {
            return None;
        }
        let tail = &expression[index..];
        if !tail
            .get(..3)
            .is_some_and(|prefix| prefix.eq_ignore_ascii_case("ddx"))
        {
            return None;
        }
        let previous = expression[..index].chars().next_back();
        if previous.is_some_and(Self::print_identifier_char) {
            return None;
        }

        let mut next_index = index + 3;
        while next_index < expression.len() {
            let ch = expression[next_index..].chars().next()?;
            if !ch.is_whitespace() {
                break;
            }
            next_index += ch.len_utf8();
        }
        expression[next_index..]
            .starts_with('(')
            .then_some(next_index)
    }

    pub(super) fn rewrite_print_expression_calls<F>(
        expression: &str,
        context: rspice_core::netlist::ParamContext,
        call_value: F,
    ) -> Result<(String, rspice_core::netlist::ParamContext), String>
    where
        F: FnMut(&str) -> Result<Value, String>,
    {
        let (rewritten, context, placeholder_index) =
            Self::rewrite_print_expression_calls_maybe(expression, context, call_value)?;

        if placeholder_index == 0 {
            return Err(format!(
                ".PRINT DC expression '{{{expression}}}' does not contain a supported voltage, current, power, or device probe"
            ));
        }

        Ok((rewritten, context))
    }

    pub(super) fn rewrite_print_expression_calls_maybe<F>(
        expression: &str,
        mut context: rspice_core::netlist::ParamContext,
        mut call_value: F,
    ) -> Result<(String, rspice_core::netlist::ParamContext, usize), String>
    where
        F: FnMut(&str) -> Result<Value, String>,
    {
        let mut rewritten = String::with_capacity(expression.len());
        let mut index = 0usize;
        let mut placeholder_index = 0usize;

        while index < expression.len() {
            if let Some(open_index) = Self::print_probe_call_open_index(expression, index) {
                let close_index = Self::matching_parenthesis_index(expression, open_index)?;
                let call = &expression[index..=close_index];
                let placeholder = format!("__rspice_probe_{placeholder_index}");
                let value = call_value(call)?;
                context.set(&placeholder, value);
                rewritten.push_str(&placeholder);
                placeholder_index += 1;
                index = close_index + 1;
                continue;
            }

            let ch = expression[index..]
                .chars()
                .next()
                .expect("valid char boundary");
            match ch {
                '{' => rewritten.push('('),
                '}' => rewritten.push(')'),
                _ => rewritten.push(ch),
            }
            index += ch.len_utf8();
        }

        Ok((rewritten, context, placeholder_index))
    }

    pub(super) fn rewrite_ac_print_expression_complex<F>(
        expression: &str,
        mut context: rspice_core::netlist::ParamContext,
        call_value: &mut F,
    ) -> Result<(String, rspice_core::netlist::ParamContext), String>
    where
        F: FnMut(&str) -> Result<ExprComplexValue, String>,
    {
        let expression = Self::print_expression_inner(expression).unwrap_or(expression);
        let mut rewritten = String::with_capacity(expression.len());
        let mut index = 0usize;
        let mut placeholder_index = 0usize;

        while index < expression.len() {
            if let Some(open_index) = Self::print_probe_call_open_index(expression, index) {
                let close_index = Self::matching_parenthesis_index(expression, open_index)?;
                let call = &expression[index..=close_index];
                let placeholder = format!("__rspice_ac_probe_{placeholder_index}");
                let value = call_value(call)?;
                context.set_complex(&placeholder, value);
                rewritten.push_str(&placeholder);
                placeholder_index += 1;
                index = close_index + 1;
                continue;
            }

            let ch = expression[index..]
                .chars()
                .next()
                .expect("valid char boundary");
            match ch {
                '{' => rewritten.push('('),
                '}' => rewritten.push(')'),
                _ => rewritten.push(ch),
            }
            index += ch.len_utf8();
        }

        Ok((rewritten, context))
    }

    pub(super) fn rewrite_print_device_parameter_tokens_maybe<F>(
        expression: &str,
        mut context: rspice_core::netlist::ParamContext,
        mut call_value: F,
    ) -> Result<(String, rspice_core::netlist::ParamContext, usize), String>
    where
        F: FnMut(&str) -> Result<Value, String>,
    {
        let mut rewritten = String::with_capacity(expression.len());
        let mut index = 0usize;
        let mut placeholder_index = 0usize;

        while index < expression.len() {
            if let Some((end_index, token)) =
                Self::print_device_parameter_token_at(expression, index)
            {
                let placeholder = format!("__rspice_param_{placeholder_index}");
                let value = call_value(token)?;
                context.set(&placeholder, value);
                rewritten.push_str(&placeholder);
                placeholder_index += 1;
                index = end_index;
                continue;
            }

            let ch = expression[index..]
                .chars()
                .next()
                .expect("valid char boundary");
            rewritten.push(ch);
            index += ch.len_utf8();
        }

        Ok((rewritten, context, placeholder_index))
    }

    pub(super) fn print_expression_inner(probe: &str) -> Option<&str> {
        let trimmed = probe.trim();
        trimmed
            .strip_prefix('{')
            .and_then(|value| value.strip_suffix('}'))
            .map(str::trim)
            .filter(|value| !value.is_empty())
    }

    pub(super) fn print_expression_contains_probe_call(expression: &str) -> bool {
        let mut index = 0usize;
        while index < expression.len() {
            if Self::print_probe_call_open_index(expression, index).is_some() {
                return true;
            }
            let ch = expression[index..]
                .chars()
                .next()
                .expect("valid char boundary");
            index += ch.len_utf8();
        }
        false
    }

    pub(super) fn print_expression_contains_named_call(expression: &str, name: &str) -> bool {
        let bytes = expression.as_bytes();
        let mut index = 0usize;
        while index < bytes.len() {
            if bytes[index] == b'"' {
                let quote = b'"';
                index += 1;
                while index < bytes.len() && bytes[index] != quote {
                    index += 1;
                }
                index = (index + 1).min(bytes.len());
                continue;
            }
            if bytes[index].is_ascii_alphabetic() || bytes[index] == b'_' {
                let start = index;
                index += 1;
                while index < bytes.len()
                    && (bytes[index].is_ascii_alphanumeric() || bytes[index] == b'_')
                {
                    index += 1;
                }
                let identifier = &expression[start..index];
                while index < bytes.len() && bytes[index].is_ascii_whitespace() {
                    index += 1;
                }
                if identifier.eq_ignore_ascii_case(name)
                    && bytes.get(index).is_some_and(|byte| *byte == b'(')
                {
                    return true;
                }
                continue;
            }
            index += 1;
        }
        false
    }

    pub(super) fn print_expression_contains_stateful_sdt(
        expression: &str,
        params: &rspice_core::netlist::ParamContext,
    ) -> bool {
        if Self::print_expression_contains_named_call(expression, "sdt") {
            return true;
        }

        let functions = params.all_functions();
        let mut stateful_functions: BTreeSet<String> = BTreeSet::new();
        loop {
            let mut changed = false;
            for function in &functions {
                if stateful_functions.contains(&function.name) {
                    continue;
                }
                let contains_sdt =
                    Self::print_expression_contains_named_call(&function.body, "sdt")
                        || stateful_functions.iter().any(|name| {
                            Self::print_expression_contains_named_call(&function.body, name)
                        });
                if contains_sdt {
                    changed |= stateful_functions.insert(function.name.clone());
                }
            }
            if !changed {
                break;
            }
        }

        stateful_functions
            .iter()
            .any(|name| Self::print_expression_contains_named_call(expression, name))
    }

    pub(super) fn print_expression_contains_voltage_accessor_call(expression: &str) -> bool {
        let mut index = 0usize;
        while index < expression.len() {
            if let Some(open_index) = Self::print_probe_call_open_index(expression, index) {
                let call = &expression[index..open_index];
                if XyceVoltageAccessor::from_function_name(call)
                    .is_some_and(|accessor| accessor != XyceVoltageAccessor::Value)
                {
                    return true;
                }
            }
            let ch = expression[index..]
                .chars()
                .next()
                .expect("valid char boundary");
            index += ch.len_utf8();
        }
        false
    }

    pub(super) fn print_probe_call_open_index(expression: &str, index: usize) -> Option<usize> {
        if index >= expression.len() || !expression.is_char_boundary(index) {
            return None;
        }
        let previous = expression[..index].chars().next_back();
        if previous.is_some_and(Self::print_identifier_char) {
            return None;
        }

        let rest = &expression[index..];
        for prefix in [
            "dno", "dni", "idb", "ir", "ii", "im", "ip", "id", "ig", "is", "ib", "ic", "ie", "vdb",
            "vr", "vi", "vm", "vp", "v", "i", "p", "w", "n",
        ] {
            let next_index = index + prefix.len();
            if rest.len() <= prefix.len()
                || !expression.is_char_boundary(next_index)
                || !rest[..prefix.len()].eq_ignore_ascii_case(prefix)
                || !expression[next_index..].starts_with('(')
            {
                continue;
            }
            return Some(next_index);
        }
        None
    }

    pub(super) fn print_device_parameter_token_at(
        expression: &str,
        index: usize,
    ) -> Option<(usize, &str)> {
        if index >= expression.len() || !expression.is_char_boundary(index) {
            return None;
        }
        let previous = expression[..index].chars().next_back();
        if previous.is_some_and(Self::print_device_parameter_token_char) {
            return None;
        }

        let first = expression[index..].chars().next()?;
        if !(first.is_ascii_alphabetic() || first == '_') {
            return None;
        }

        let mut end_index = index;
        let mut has_colon = false;
        while end_index < expression.len() {
            let ch = expression[end_index..].chars().next()?;
            if !Self::print_device_parameter_token_char(ch) {
                break;
            }
            has_colon |= ch == ':';
            end_index += ch.len_utf8();
        }
        if !has_colon {
            return None;
        }

        let token = &expression[index..end_index];
        Self::parse_device_parameter_probe(token).map(|_| (end_index, token))
    }

    pub(super) fn print_device_parameter_token_char(ch: char) -> bool {
        ch.is_ascii_alphanumeric() || matches!(ch, '_' | '.' | '$' | ':')
    }

    pub(super) fn print_identifier_char(ch: char) -> bool {
        ch.is_ascii_alphanumeric() || matches!(ch, '_' | '.' | '$')
    }

    pub(super) fn print_eval_context(
        netlist: &Netlist,
        dc: Option<&XyceDcSweep>,
        sweep_point: Option<XyceDcSweepPoint>,
    ) -> rspice_core::netlist::ParamContext {
        let mut context = netlist.params.clone();
        let temp_c = Self::active_temperature_c(netlist, dc, sweep_point);
        context.set("TEMP", temp_c);
        context.set("TEMPER", temp_c);
        context.set("TNOM", netlist.options.tnom.unwrap_or(27.0));
        context.set("VT", Self::thermal_voltage_celsius(temp_c));
        context.set(
            "GMIN",
            netlist.options.gmin.unwrap_or(rspice_core::constants::GMIN),
        );
        Self::add_runtime_scalar_parameter_bindings(&mut context);
        Self::add_resistor_parameter_bindings(netlist, &mut context);
        Self::add_independent_source_parameter_bindings(netlist, &mut context);
        context
    }

    pub(super) fn print_tran_eval_context(
        netlist: &Netlist,
        time: Value,
    ) -> rspice_core::netlist::ParamContext {
        let mut context = Self::print_eval_context(netlist, None, None);
        context.set("TIME", time);
        Self::add_runtime_scalar_parameter_bindings(&mut context);
        Self::add_runtime_file_table_parameter_bindings(netlist, &mut context, time);
        for measurement in &netlist.measurements {
            if measurement.analysis.eq_ignore_ascii_case("TRAN")
                && matches!(
                    measurement.measure_type,
                    rspice_core::analysis::MeasureType::Equation { .. }
                )
            {
                context.set(
                    &measurement.name,
                    netlist
                        .options
                        .measure_default_value
                        .or(measurement.default_value)
                        .unwrap_or(-1.0),
                );
            }
        }
        context
    }

    #[cfg(test)]
    pub(super) fn primary_print_output_request(
        source: &str,
        analysis: &str,
    ) -> Result<Option<XycePrintOutputRequest>, String> {
        Self::canonical_print_output_request(source, analysis, false)
    }

    pub(super) fn canonical_print_output_request(
        source: &str,
        analysis: &str,
        allow_single_file_output: bool,
    ) -> Result<Option<XycePrintOutputRequest>, String> {
        let requests = Self::aggregate_print_output_requests(
            Self::print_output_requests(source, analysis)?,
            analysis,
        )?;
        let primary_requests = requests
            .iter()
            .filter(|request| request.file.is_none())
            .cloned()
            .collect::<Vec<_>>();

        match primary_requests.len() {
            0 => Ok(None),
            1 => Ok(Some(
                primary_requests.into_iter().next().expect("one request"),
            )),
            _ => Err(format!(
                "deck has multiple primary .PRINT {analysis} output destinations after aggregation"
            )),
        }
        .and_then(|primary| {
            if primary.is_some() || !allow_single_file_output {
                return Ok(primary);
            }
            let file_requests = requests
                .into_iter()
                .filter(|request| request.file.is_some())
                .collect::<Vec<_>>();
            match file_requests.len() {
                0 => Ok(None),
                1 => Ok(file_requests.into_iter().next()),
                _ => Err(format!(
                    "deck has multiple .PRINT {analysis} FILE= outputs and no primary output"
                )),
            }
        })
    }

    pub(super) fn output_override_print_output_request(
        source: &str,
        analysis: &str,
    ) -> Result<Option<XycePrintOutputRequest>, String> {
        let mut probes = Vec::new();
        for request in Self::print_output_requests(source, analysis)? {
            probes.extend(request.probes);
        }
        if probes.is_empty() {
            return Ok(None);
        }
        Ok(Some(XycePrintOutputRequest {
            format: None,
            file: None,
            probes,
        }))
    }

    pub(super) fn aggregate_print_output_requests(
        requests: Vec<XycePrintOutputRequest>,
        analysis: &str,
    ) -> Result<Vec<XycePrintOutputRequest>, String> {
        let mut aggregated: Vec<XycePrintOutputRequest> = Vec::new();
        for request in requests {
            if let Some(existing) = aggregated
                .iter_mut()
                .find(|existing| existing.file == request.file)
            {
                Self::validate_print_output_format_compatible(
                    existing.format.as_deref(),
                    request.format.as_deref(),
                    analysis,
                    request.file.as_deref(),
                )?;
                existing.probes.extend(request.probes);
                if existing.format.is_none() {
                    existing.format = request.format;
                }
            } else {
                aggregated.push(request);
            }
        }
        Ok(aggregated)
    }

    pub(super) fn print_format_key(format: Option<&str>) -> String {
        format.unwrap_or("STD").trim().to_ascii_lowercase()
    }

    pub(super) fn dc_print_output_requests(
        source: &str,
    ) -> Result<Vec<XycePrintOutputRequest>, String> {
        Self::print_output_requests(source, "DC")
    }

    pub(super) fn dc_print_precisions(source: &str) -> Result<HashMap<String, usize>, String> {
        let mut precisions = HashMap::new();
        for line in Self::logical_netlist_lines(source) {
            let trimmed = Self::strip_netlist_comment(&line).trim().to_string();
            if trimmed.is_empty() {
                continue;
            }
            let Some(command) = trimmed.split_whitespace().next() else {
                continue;
            };
            if !command.eq_ignore_ascii_case(".print") {
                continue;
            }

            let tokens = Self::split_print_fields(&trimmed)?;
            let token_refs = tokens.iter().map(String::as_str).collect::<Vec<_>>();
            if token_refs
                .get(1)
                .is_none_or(|analysis| !analysis.eq_ignore_ascii_case("DC"))
            {
                continue;
            }

            let mut precision = None;
            let mut probes = Vec::new();
            let mut index = 2usize;
            while index < token_refs.len() {
                if let Some((raw_key, raw_value, consumed)) =
                    Self::print_option_assignment(&token_refs, index)
                {
                    if raw_key.trim().eq_ignore_ascii_case("precision") {
                        let value = raw_value.trim().trim_matches(['"', '\'']);
                        let parsed = value.parse::<usize>().map_err(|err| {
                            format!(
                                ".PRINT DC PRECISION must be a positive integer, got '{value}': {err}"
                            )
                        })?;
                        if !(1..=XYCE_MAX_IEEE754_PRN_SCIENTIFIC_PRECISION).contains(&parsed) {
                            return Err(format!(
                                ".PRINT DC PRECISION must be between 1 and {XYCE_MAX_IEEE754_PRN_SCIENTIFIC_PRECISION}, got {parsed}"
                            ));
                        }
                        precision = Some(parsed);
                    }
                    index += consumed;
                    continue;
                }
                let normalized = token_refs[index].to_ascii_lowercase();
                if Self::is_print_option_token(&normalized) {
                    index += 1;
                    continue;
                }
                probes.push(Self::canonicalize_single_quoted_print_probe(
                    token_refs[index],
                )?);
                index += 1;
            }

            let effective = precision.unwrap_or(XYCE_DEFAULT_PRN_SCIENTIFIC_PRECISION);
            for probe in probes {
                let normalized = Self::normalize_probe(&probe);
                if let Some(existing) = precisions.insert(normalized, effective)
                    && existing != effective
                {
                    return Err(format!(
                        ".PRINT DC probe '{probe}' has conflicting PRECISION values {existing} and {effective}"
                    ));
                }
            }
        }
        Ok(precisions)
    }

    #[cfg(test)]
    pub(super) fn dc_print_precision_for_probe(
        source: &str,
        probe: &str,
    ) -> Result<Option<usize>, String> {
        Ok(Self::dc_print_precisions(source)?
            .get(&Self::normalize_probe(probe))
            .copied())
    }

    pub(super) fn quantize_dc_print_value_with_precisions(
        precisions: &HashMap<String, usize>,
        probe: &str,
        value: Value,
    ) -> Result<Value, String> {
        match precisions.get(&Self::normalize_probe(probe)).copied() {
            Some(precision) => Self::xyce_prn_scientific_roundtrip(value, precision),
            None => Ok(value),
        }
    }

    pub(super) fn quantize_dc_print_value(
        source: &str,
        probe: &str,
        value: Value,
    ) -> Result<Value, String> {
        let precisions = Self::dc_print_precisions(source)?;
        Self::quantize_dc_print_value_with_precisions(&precisions, probe, value)
    }

    pub(super) fn print_output_requests(
        source: &str,
        expected_analysis: &str,
    ) -> Result<Vec<XycePrintOutputRequest>, String> {
        let mut requests = Vec::new();
        for line in Self::logical_netlist_lines(source) {
            let trimmed = Self::strip_netlist_comment(&line).trim().to_string();
            if trimmed.is_empty() {
                continue;
            }
            let Some(command) = trimmed.split_whitespace().next() else {
                continue;
            };
            if !command.eq_ignore_ascii_case(".print") {
                continue;
            }
            let tokens = Self::split_print_fields(&trimmed)?;
            let token_refs = tokens.iter().map(String::as_str).collect::<Vec<_>>();
            let Some(analysis) = token_refs.get(1).copied() else {
                return Err(".PRINT statement has no analysis type".to_string());
            };
            let analysis_matches = analysis.eq_ignore_ascii_case(expected_analysis)
                || (expected_analysis.eq_ignore_ascii_case("TRAN")
                    && Self::is_tran_analysis_keyword(analysis));
            if !analysis_matches {
                continue;
            }

            let mut format = None;
            let mut file = None;
            let mut probes = Vec::new();
            let mut index = 2usize;
            while index < tokens.len() {
                if let Some((raw_key, raw_value, consumed)) =
                    Self::print_option_assignment(&token_refs, index)
                {
                    let key = raw_key.trim().to_ascii_lowercase();
                    let value = raw_value.trim().trim_matches(['"', '\'']).to_string();
                    match key.as_str() {
                        "format" => format = Some(value),
                        "file" => file = Some(value),
                        _ => {}
                    }
                    index += consumed;
                    continue;
                }

                let normalized = token_refs[index].to_ascii_lowercase();
                if Self::is_print_option_token(&normalized) {
                    index += 1;
                    continue;
                }
                probes.push(Self::canonicalize_single_quoted_print_probe(
                    &tokens[index],
                )?);
                index += 1;
            }

            if probes.is_empty() {
                continue;
            }
            requests.push(XycePrintOutputRequest {
                format,
                file,
                probes,
            });
        }

        Ok(requests)
    }

    pub(super) fn split_print_fields(line: &str) -> Result<Vec<String>, String> {
        Self::split_grouped_whitespace_fields(line, ".PRINT statement")
    }

    pub(super) fn canonicalize_single_quoted_print_probe(probe: &str) -> Result<String, String> {
        let trimmed = probe.trim();
        let starts_quote = trimmed.starts_with('\'');
        let ends_quote = trimmed.ends_with('\'');
        if !starts_quote && !ends_quote {
            return Ok(probe.to_string());
        }
        if !starts_quote || !ends_quote || trimmed.len() < 2 {
            return Err(format!(
                "malformed single-quoted .PRINT expression '{probe}'"
            ));
        }
        let inner = trimmed[1..trimmed.len() - 1].trim();
        if inner.is_empty() || inner.contains('\'') || inner.contains('{') || inner.contains('}') {
            return Err(format!(
                "single-quoted .PRINT expression must be nonempty and must not mix expression delimiters: '{probe}'"
            ));
        }
        Ok(format!("{{{inner}}}"))
    }

    pub(super) fn gnuplot_splot_print_pair(
        source: &str,
    ) -> Result<(XycePrintOutputRequest, XycePrintOutputRequest), String> {
        let requests = Self::dc_print_output_requests(source)?;
        let mut primary = None;
        let mut side = None;
        for request in requests {
            let format = request.format.as_deref().unwrap_or("STD");
            if request.file.is_none() && format.eq_ignore_ascii_case("GNUPLOT") {
                if primary.replace(request).is_some() {
                    return Err(
                        "GNUPLOT/SPLOT contract requires exactly one primary GNUPLOT .PRINT DC"
                            .to_string(),
                    );
                }
                continue;
            }
            if request.file.is_some() && format.eq_ignore_ascii_case("SPLOT") {
                if side.replace(request).is_some() {
                    return Err(
                        "GNUPLOT/SPLOT contract requires exactly one named SPLOT .PRINT DC"
                            .to_string(),
                    );
                }
                continue;
            }
            return Err(format!(
                "GNUPLOT/SPLOT contract does not cover .PRINT DC FORMAT={format} FILE={}",
                request.file.as_deref().unwrap_or("<default>")
            ));
        }

        match (primary, side) {
            (Some(primary), Some(side)) => Ok((primary, side)),
            _ => Err(
                "GNUPLOT/SPLOT contract requires one primary GNUPLOT and one named SPLOT .PRINT DC"
                    .to_string(),
            ),
        }
    }

    pub(super) fn nested_include_subcircuit_fingerprint(
        subcircuit: &SubcircuitDef,
    ) -> Result<XyceNestedIncludeSubcircuitFingerprint, String> {
        let mut elements = BTreeMap::new();
        for element in &subcircuit.elements {
            let fingerprint = match &element.kind {
                ElementKind::Resistor { .. } => {
                    Self::strict_nested_include_resistor_fingerprint(element)?
                }
                ElementKind::Subcircuit {
                    subckt_name,
                    params,
                } if params.is_empty() && element.nodes.len() == 2 => {
                    XyceRelationalElementFingerprint {
                        kind: "X".to_string(),
                        nodes: element
                            .nodes
                            .iter()
                            .map(|node| node.to_ascii_lowercase())
                            .collect(),
                        numeric_bits: Vec::new(),
                        text: vec![subckt_name.to_ascii_lowercase()],
                    }
                }
                _ => {
                    return Err(format!(
                        "subcircuit '{}' contains unqualified element '{}'",
                        subcircuit.name, element.name
                    ));
                }
            };
            if elements
                .insert(element.name.to_ascii_lowercase(), fingerprint)
                .is_some()
            {
                return Err(format!(
                    "subcircuit '{}' contains duplicate element '{}'",
                    subcircuit.name, element.name
                ));
            }
        }
        let mut nested_names = subcircuit
            .nested_subcircuits
            .iter()
            .map(|nested| nested.name.to_ascii_lowercase())
            .collect::<Vec<_>>();
        nested_names.sort();
        Ok(XyceNestedIncludeSubcircuitFingerprint {
            name: subcircuit.name.to_ascii_lowercase(),
            ports: subcircuit
                .ports
                .iter()
                .map(|port| port.to_ascii_lowercase())
                .collect(),
            elements,
            nested_names,
        })
    }

    pub(super) fn reject_wrapper_output_artifacts(&self, deck_path: &Path) -> Result<(), String> {
        let Some(output_path) = self.static_output_reference_path(deck_path, "prn") else {
            return Err("wrapper deck is not rooted under the Xyce Netlists corpus".to_string());
        };
        let Some(parent) = output_path.parent() else {
            return Err("wrapper output path has no parent directory".to_string());
        };
        let parent_metadata = match fs::symlink_metadata(parent) {
            Ok(metadata) => metadata,
            Err(error) if error.kind() == std::io::ErrorKind::NotFound => return Ok(()),
            Err(error) => {
                return Err(format!(
                    "could not inspect wrapper output directory {}: {error}",
                    parent.display()
                ));
            }
        };
        if parent_metadata.file_type().is_symlink() || !parent_metadata.file_type().is_dir() {
            return Err(format!(
                "wrapper output location {} must be a regular non-symlink directory",
                parent.display()
            ));
        }
        let prefix = deck_path
            .file_name()
            .and_then(|name| name.to_str())
            .ok_or_else(|| "wrapper deck filename is not valid UTF-8".to_string())?
            .to_ascii_lowercase()
            + ".";
        let mut artifacts = Vec::new();
        for entry in fs::read_dir(parent)
            .map_err(|err| format!("could not inspect wrapper output directory: {err}"))?
        {
            let entry =
                entry.map_err(|err| format!("could not inspect wrapper output artifact: {err}"))?;
            let name = entry
                .file_name()
                .to_str()
                .ok_or_else(|| {
                    format!(
                        "wrapper output artifact name is not UTF-8: {}",
                        entry.path().display()
                    )
                })?
                .to_ascii_lowercase();
            if name.starts_with(&prefix) {
                // Any path with the deck's output prefix is an artifact. Do
                // not let a symlink, directory, socket, or other non-regular
                // entry bypass the checked-in-output prohibition.
                artifacts.push(entry.path());
            }
        }
        artifacts.sort();
        if artifacts.is_empty() {
            Ok(())
        } else {
            Err(format!(
                "relational/generated wrapper must not have checked-in output artifacts: {}",
                artifacts
                    .iter()
                    .map(|path| self.display_path(path))
                    .collect::<Vec<_>>()
                    .join(", ")
            ))
        }
    }
}
