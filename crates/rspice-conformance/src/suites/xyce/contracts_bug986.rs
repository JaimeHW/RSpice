use super::*;
use rspice_core::engine::{SimulationConfigOverrides, resolve_simulation_config};
use rspice_core::numerics::integration::TransientErrorControl;
use std::io::Read as _;

const LABEL: &str = "BUG_986_SON ERROPTION breakpoint-source relation";
const SCHEDULE: [Value; 5] = [1.0e-3, 2.0e-3, 3.0e-3, 4.0e-3, 5.0e-3];
// BUG981's Release patfile/default-PRN oracle independently corroborates these
// five source values. The analytic comparison below derives them directly;
// these strings additionally bind the default eight-digit PRN serialization.
const BUG981_CORROBORATED_PRN_VALUES: [&str; 5] = [
    "5.16060279e0",
    "6.32332358e0",
    "6.75106466e0",
    "6.90842181e0",
    "6.96631027e0",
];
const DEFAULT_PRN_HALF_QUANTUM: Value = 5.0e-9;

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub(super) enum Bug986Role {
    WrapperOwner,
    ExplicitBreakpoints,
    DisconnectedPwlBreakpoints,
}

impl Bug986Role {
    const ALL: [Self; 3] = [
        Self::WrapperOwner,
        Self::ExplicitBreakpoints,
        Self::DisconnectedPwlBreakpoints,
    ];
    const WORKERS: [Self; 2] = [Self::ExplicitBreakpoints, Self::DisconnectedPwlBreakpoints];

    pub(super) fn for_record(record: &str) -> Option<Self> {
        let record = XyceTestRunner::normalize_manifest_key(record);
        Self::ALL.into_iter().find(|role| role.record() == record)
    }

    pub(super) fn contract(self) -> &'static str {
        match self {
            Self::WrapperOwner => XYCE_BUG986_OWNER_CONTRACT,
            Self::ExplicitBreakpoints => XYCE_BUG986_BREAKPOINT_CONTRACT,
            Self::DisconnectedPwlBreakpoints => XYCE_BUG986_PWL_CONTRACT,
        }
    }

    fn path(self) -> &'static str {
        match self {
            Self::WrapperOwner => XYCE_BUG986_OWNER_PATH,
            Self::ExplicitBreakpoints => XYCE_BUG986_BREAKPOINT_PATH,
            Self::DisconnectedPwlBreakpoints => XYCE_BUG986_PWL_PATH,
        }
    }

    fn record(self) -> &'static str {
        match self {
            Self::WrapperOwner => XYCE_BUG986_OWNER_RECORD,
            Self::ExplicitBreakpoints => XYCE_BUG986_BREAKPOINT_RECORD,
            Self::DisconnectedPwlBreakpoints => XYCE_BUG986_PWL_RECORD,
        }
    }

    fn file_name(self) -> &'static str {
        self.path()
            .rsplit('/')
            .next()
            .expect("BUG986 path has file name")
    }

    fn label(self) -> &'static str {
        match self {
            Self::WrapperOwner => "wrapper owner",
            Self::ExplicitBreakpoints => "explicit BREAKPOINTS worker",
            Self::DisconnectedPwlBreakpoints => "disconnected PWL worker",
        }
    }
}

impl XyceTestRunner {
    pub(super) fn bug986_historical_oracle_provenance_records() -> Vec<String> {
        let mut records = XYCE_BUG986_HISTORICAL_ARTIFACTS
            .into_iter()
            .map(|(path, bytes, sha256, content_blake3)| {
                format!(
                    "{XYCE_BUG986_UPSTREAM_REGRESSION_COMMIT}\t{XYCE_BUG986_UPSTREAM_RELEASE_TAG}\t{path}\t{bytes}\t{sha256}\t{content_blake3}"
                )
            })
            .collect::<Vec<_>>();
        records.sort();
        records
    }

    pub(super) fn validate_bug986_historical_oracle_provenance() -> Result<(), String> {
        let records = Self::bug986_historical_oracle_provenance_records();
        let stream = records.join("\n");
        let sha256 = format!("{:x}", Sha256::digest(stream.as_bytes()));
        let content_blake3 = blake3::hash(stream.as_bytes()).to_hex().to_string();
        if records.len() != XYCE_BUG986_HISTORICAL_RECORD_COUNT
            || stream.len() != XYCE_BUG986_HISTORICAL_RECORD_BYTES
            || sha256 != XYCE_BUG986_HISTORICAL_RECORDS_SHA256
            || content_blake3 != XYCE_BUG986_HISTORICAL_RECORDS_BLAKE3
        {
            return Err(format!(
                "{LABEL} Release-7.10 provenance changed: records={}/{}, sha256={sha256}, blake3={content_blake3}",
                records.len(),
                stream.len()
            ));
        }
        Ok(())
    }

    fn validate_bug986_directory(&self) -> Result<BTreeMap<String, Vec<u8>>, String> {
        let directory = self.root.join("Netlists/Certification_Tests/BUG_986_SON");
        let metadata = fs::symlink_metadata(&directory)
            .map_err(|error| format!("failed to inspect {LABEL} family: {error}"))?;
        if metadata.file_type().is_symlink() || !metadata.file_type().is_dir() {
            return Err(format!(
                "{LABEL} family must be a regular non-symlink directory"
            ));
        }
        let expected = XYCE_BUG986_RETAINED_ARTIFACTS
            .into_iter()
            .map(|artifact| (artifact.0.to_ascii_lowercase(), artifact))
            .collect::<BTreeMap<_, _>>();
        let mut observed = BTreeMap::new();
        for entry in fs::read_dir(&directory)
            .map_err(|error| format!("failed to read {LABEL} family: {error}"))?
        {
            let entry =
                entry.map_err(|error| format!("failed to inspect {LABEL} member: {error}"))?;
            let path = entry.path();
            let metadata = fs::symlink_metadata(&path)
                .map_err(|error| format!("failed to inspect {}: {error}", path.display()))?;
            if metadata.file_type().is_symlink() || !metadata.file_type().is_file() {
                return Err(format!(
                    "{LABEL} member {} must be a regular file",
                    path.display()
                ));
            }
            let name = entry
                .file_name()
                .to_str()
                .ok_or_else(|| format!("{LABEL} member name is not UTF-8"))?
                .to_string();
            let key = name.to_ascii_lowercase();
            if observed.contains_key(&key) {
                return Err(format!("{LABEL} contains case-colliding member {name:?}"));
            }
            let Some((expected_name, expected_bytes, expected_sha256, expected_blake3)) =
                expected.get(&key).copied()
            else {
                return Err(format!("{LABEL} acquired unexpected member {name:?}"));
            };
            if name != expected_name {
                return Err(format!("{LABEL} member case changed: {name:?}"));
            }
            // A canonical LF byte can occupy at most two physical bytes in a
            // CRLF checkout. Reserve three further bytes for a possible UTF-8
            // BOM; canonical identity below still rejects it by hash.
            let maximum_raw_bytes = expected_bytes
                .checked_mul(2)
                .and_then(|value| value.checked_add(3))
                .ok_or_else(|| format!("{LABEL} retained-size bound overflowed"))?;
            if metadata.len() > maximum_raw_bytes as u64 {
                return Err(format!(
                    "{LABEL} member {name:?} exceeds its bounded read envelope: {} > {maximum_raw_bytes}",
                    metadata.len()
                ));
            }
            let mut bytes = Vec::with_capacity((metadata.len() as usize).min(maximum_raw_bytes));
            fs::File::open(&path)
                .map_err(|error| format!("failed to open {LABEL} member {name:?}: {error}"))?
                .take((maximum_raw_bytes + 1) as u64)
                .read_to_end(&mut bytes)
                .map_err(|error| format!("failed to read {LABEL} member {name:?}: {error}"))?;
            if bytes.len() > maximum_raw_bytes {
                return Err(format!(
                    "{LABEL} member {name:?} grew beyond its bounded read envelope"
                ));
            }
            let canonical = Self::canonical_lf_text_identity(LABEL, &bytes)?;
            let sha256 = format!("{:x}", Sha256::digest(&canonical));
            let content_blake3 = blake3::hash(&canonical).to_hex().to_string();
            if canonical.len() != expected_bytes
                || sha256 != expected_sha256
                || content_blake3 != expected_blake3
            {
                return Err(format!(
                    "{LABEL} member {name:?} changed: bytes={}, sha256={sha256}, blake3={content_blake3}",
                    canonical.len()
                ));
            }
            observed.insert(key, bytes);
        }
        if observed.len() != expected.len() {
            return Err(format!("{LABEL} retained family census changed"));
        }
        Ok(observed)
    }

    fn validate_bug986_provenance(
        &self,
        deck: &XyceDeck,
        role: Bug986Role,
    ) -> Result<BTreeMap<String, Vec<u8>>, String> {
        Self::validate_bug986_historical_oracle_provenance()?;
        if deck.section != XyceDeckSection::Netlists
            || Self::normalize_manifest_key(&deck.relative_path) != role.record()
            || Self::normalize_manifest_key(&self.relative_key(&deck.path)) != role.record()
            || !Self::same_path(&deck.path, &self.root.join(role.path()))
        {
            return Err(format!(
                "recognized {LABEL} {} is not at its canonical path",
                role.label()
            ));
        }

        let family_prefix = "netlists/certification_tests/bug_986_son/";
        let wrapper_records = Self::load_upstream_wrapper_decks(&self.root);
        let owners = wrapper_records
            .iter()
            .filter(|record| record.starts_with(family_prefix))
            .map(String::as_str)
            .collect::<BTreeSet<_>>();
        if owners != BTreeSet::from([XYCE_BUG986_OWNER_RECORD]) {
            return Err(format!("{LABEL} wrapper ownership changed: {owners:?}"));
        }
        let exclusions = Self::load_upstream_exclusions(&self.root)
            .map_err(|error| format!("{LABEL} exclusion manifest is invalid: {error}"))?;
        if exclusions.contains_key(XYCE_BUG986_OWNER_RECORD) {
            return Err(format!("{LABEL} owner must not be excluded"));
        }
        let family_exclusions = exclusions
            .iter()
            .filter(|(record, _)| record.starts_with(family_prefix))
            .collect::<BTreeMap<_, _>>();
        if family_exclusions.len() != 2 {
            return Err(format!(
                "{LABEL} requires exactly two worker qualification rows"
            ));
        }
        for worker in Bug986Role::WORKERS {
            let exclusion = family_exclusions
                .get(&worker.record().to_string())
                .copied()
                .ok_or_else(|| format!("{LABEL} lost {} qualification", worker.file_name()))?;
            if exclusion.source != XYCE_BUG986_EXCLUSION_SOURCE
                || !matches!(&exclusion.disposition,
                    XyceUpstreamExclusionDisposition::RspiceIndependentlyQualified { expected_contract }
                        if expected_contract == worker.contract())
            {
                return Err(format!(
                    "{LABEL} {} qualification changed",
                    worker.file_name()
                ));
            }
        }

        let members = self.validate_bug986_directory()?;
        self.reject_wrapper_output_artifacts(&self.root.join(XYCE_BUG986_OWNER_PATH))
            .map_err(|error| format!("{LABEL} owner {error}"))?;
        let output_family = self.root.join("OutputData/Certification_Tests/BUG_986_SON");
        match fs::symlink_metadata(&output_family) {
            Err(error) if error.kind() == std::io::ErrorKind::NotFound => {}
            Err(error) => return Err(format!("failed to inspect {LABEL} OutputData: {error}")),
            Ok(_) => return Err(format!("{LABEL} must not acquire invented numerical gold")),
        }
        Ok(members)
    }

    fn bug986_nodes_match(actual: &[String], expected: &[&str]) -> bool {
        actual.len() == expected.len()
            && actual
                .iter()
                .zip(expected)
                .all(|(actual, expected)| actual.eq_ignore_ascii_case(expected))
    }

    fn bug986_common_topology_matches(elements: &[rspice_core::netlist::Element]) -> bool {
        let [offset, exponential, resistor, capacitor, ..] = elements else {
            return false;
        };
        offset.provenance == ElementProvenance::Authored
            && offset.name.eq_ignore_ascii_case("VOFF")
            && Self::bug986_nodes_match(&offset.nodes, &["A", "0"])
            && matches!(&offset.kind, ElementKind::VoltageSource(
                rspice_core::netlist::SourceSpec::Dc(value)
            ) if value.to_bits() == 2.0f64.to_bits())
            && exponential.provenance == ElementProvenance::Authored
            && exponential.name.eq_ignore_ascii_case("VEXP")
            && Self::bug986_nodes_match(&exponential.nodes, &["1", "A"])
            && matches!(&exponential.kind, ElementKind::VoltageSource(
                rspice_core::netlist::SourceSpec::Exp { v1, v2, td1, tau1, td2, tau2 }
            ) if v1.to_bits() == 0.0f64.to_bits()
                && v2.to_bits() == 5.0f64.to_bits()
                && td1.to_bits() == 0.0f64.to_bits()
                && tau1.to_bits() == 1.0e-3f64.to_bits()
                && td2.to_bits() == 1.0f64.to_bits()
                && tau2.is_nan())
            && resistor.provenance == ElementProvenance::Authored
            && resistor.name.eq_ignore_ascii_case("R1")
            && Self::bug986_nodes_match(&resistor.nodes, &["1", "2"])
            && matches!(&resistor.kind, ElementKind::Resistor {
                value,
                value_expr: None,
                model: None,
                instance_params,
                deferred_params,
            } if value.to_bits() == 1.0f64.to_bits()
                && instance_params.is_empty()
                && deferred_params.is_empty())
            && capacitor.provenance == ElementProvenance::Authored
            && capacitor.name.eq_ignore_ascii_case("C1")
            && Self::bug986_nodes_match(&capacitor.nodes, &["2", "0"])
            && matches!(&capacitor.kind, ElementKind::Capacitor {
                value,
                value_expr: None,
                initial_voltage: None,
                model: None,
                instance_params,
                deferred_params,
            } if value.to_bits() == 1.0f64.to_bits()
                && instance_params.is_empty()
                && deferred_params.is_empty())
    }

    fn validate_bug986_worker(
        &self,
        role: Bug986Role,
        source: &str,
        path: &Path,
    ) -> Result<XyceStaticTranPlan, String> {
        if !Bug986Role::WORKERS.contains(&role) {
            return Err(format!("{LABEL} owner is not executable"));
        }
        let plan = self.static_tran_plan_for_path_with_purpose(
            path,
            XyceStaticTranPlanPurpose::RelationalFamily,
        )?;
        if plan.deck_path != path
            || plan.source.as_bytes() != source.as_bytes()
            || !matches!(plan.oracle, XyceStaticTranOracle::None)
            || plan.contract != XyceStaticTranContract::PlainStatic
            || plan.comparison_mode != XyceStaticTranComparisonMode::Pointwise
            || !plan.steps.is_empty()
            || plan.output_override
            || plan.timeint_conststep
            || plan.wrapper_tolerance.is_some()
            || plan.print.as_ref().is_none_or(|print| {
                print.probes.len() != 1 || !print.probes[0].eq_ignore_ascii_case("V(1)")
            })
            || plan.tran.step.to_bits() != 1.0e-3f64.to_bits()
            || plan.tran.stop.to_bits() != 5.0e-3f64.to_bits()
            || plan.tran.start.is_some()
            || plan.tran.max_step.is_some()
            || plan.tran.uic
        {
            return Err(format!("{LABEL} {} plan changed: {plan:?}", role.label()));
        }
        let print_outputs = Self::print_output_requests(source, "TRAN")?;
        if !matches!(print_outputs.as_slice(), [request]
            if request.format.is_none()
                && request.file.is_none()
                && matches!(request.probes.as_slice(), [probe] if probe.eq_ignore_ascii_case("V(1)")))
        {
            return Err(format!(
                "{LABEL} {} authored default PRN output changed",
                role.label()
            ));
        }

        let netlist = Self::parse_xyce_netlist(source, path)
            .map_err(|error| format!("{LABEL} {} no longer parses: {error}", role.label()))?;
        let expected_elements = if role == Bug986Role::ExplicitBreakpoints {
            4
        } else {
            6
        };
        if netlist.title != "testing breakpoints "
            || netlist.elements.len() != expected_elements
            || netlist.analyses.len() != 1
            || netlist.output_requests.len() != 1
            || !netlist.models.is_empty()
            || !netlist.subcircuits.is_empty()
            || !netlist.diagnostics.is_empty()
            || !netlist.data_tables.is_empty()
            || !netlist.measurements.is_empty()
            || !netlist.initial_conditions.is_empty()
            || netlist.device_initial_conditions.is_some()
            || !netlist.node_sets.is_empty()
            || !netlist.global_nodes.is_empty()
            || !netlist.veriloga_includes.is_empty()
            || !netlist.spef_includes.is_empty()
            || !netlist.params.all_params().is_empty()
            || !netlist.params.all_string_params().is_empty()
            || !netlist.params.all_functions().is_empty()
            || !netlist.params.all_parameter_expressions().is_empty()
            || !netlist.params.all_global_expressions().is_empty()
        {
            return Err(format!(
                "{LABEL} {} typed envelope changed: elements={:?}, diagnostics={:?}, options={:?}",
                role.label(),
                netlist.elements,
                netlist.diagnostics,
                netlist.options
            ));
        }

        let mut remaining_options = netlist.options.clone();
        let breakpoints = std::mem::take(&mut remaining_options.timeint_breakpoints);
        let error_control = std::mem::take(&mut remaining_options.timeint_error_control);
        let min_steps =
            std::mem::take(&mut remaining_options.timeint_min_steps_between_breakpoints);
        let nlmin = std::mem::take(&mut remaining_options.timeint_nlmin);
        let nlmax = std::mem::take(&mut remaining_options.timeint_nlmax);
        let min_order = std::mem::take(&mut remaining_options.timeint_min_order);
        let max_order = std::mem::take(&mut remaining_options.timeint_max_order);
        let reversal = std::mem::take(&mut remaining_options.timeint_timesteps_reversal);
        let schedule_matches = match role {
            Bug986Role::ExplicitBreakpoints => breakpoints == SCHEDULE[..4],
            Bug986Role::DisconnectedPwlBreakpoints => breakpoints.is_empty(),
            Bug986Role::WrapperOwner => false,
        };
        if !schedule_matches
            || error_control != Some(TransientErrorControl::NonlinearIterations)
            || min_steps.is_some()
            || nlmin.is_some()
            || nlmax.is_some()
            || min_order.is_some()
            || max_order.is_some()
            || reversal.is_some()
            || !Self::analytic_timeint_only_options_match(
                &remaining_options,
                None,
                None,
                None,
                None,
            )
        {
            return Err(format!("{LABEL} {} option state changed", role.label()));
        }
        let resolved = resolve_simulation_config(
            &self.xyce_engine_config(None),
            Some(&netlist.options),
            &SimulationConfigOverrides::default(),
        );
        if resolved.transient_error_control != TransientErrorControl::NonlinearIterations
            || resolved.transient_min_steps_between_breakpoints.is_some()
            || resolved.effective_transient_min_steps_between_breakpoints() != Some(10)
        {
            return Err(format!(
                "{LABEL} {} did not resolve implicit ERROPTION=1/MINTIMESTEPSBP=10",
                role.label()
            ));
        }

        if !Self::bug986_common_topology_matches(&netlist.elements) {
            return Err(format!("{LABEL} {} common topology changed", role.label()));
        }
        if role == Bug986Role::DisconnectedPwlBreakpoints {
            let [.., source, resistor] = netlist.elements.as_slice() else {
                unreachable!("BUG986B element count was checked")
            };
            let expected_points: [(Value, Value); 5] = [
                (0.0, 0.0),
                (1.0e-3, 0.0),
                (2.0e-3, 0.0),
                (3.0e-3, 0.0),
                (4.0e-3, 0.0),
            ];
            if source.provenance != ElementProvenance::Authored
                || !source.name.eq_ignore_ascii_case("Vtmp")
                || !Self::bug986_nodes_match(&source.nodes, &["E", "0"])
                || !matches!(&source.kind, ElementKind::VoltageSource(
                    rspice_core::netlist::SourceSpec::Pwl { points, delay, repeat_from }
                ) if delay.to_bits() == 0.0f64.to_bits()
                    && repeat_from.is_none()
                    && points.len() == expected_points.len()
                    && points.iter().zip(expected_points).all(|((time, value), (expected_time, expected_value))|
                        time.to_bits() == expected_time.to_bits()
                            && value.to_bits() == expected_value.to_bits()))
                || resistor.provenance != ElementProvenance::Authored
                || !resistor.name.eq_ignore_ascii_case("Rtmp")
                || !Self::bug986_nodes_match(&resistor.nodes, &["E", "0"])
                || !matches!(&resistor.kind, ElementKind::Resistor {
                    value,
                    value_expr: None,
                    model: None,
                    instance_params,
                    deferred_params,
                } if value.to_bits() == 1.0f64.to_bits()
                    && instance_params.is_empty()
                    && deferred_params.is_empty())
            {
                return Err(format!("{LABEL} disconnected PWL topology changed"));
            }
        }

        if !matches!(&netlist.analyses[0], AnalysisCommand::Tran {
            step,
            stop,
            start: None,
            max_step: None,
            uic: false,
        } if step.to_bits() == 1.0e-3f64.to_bits()
            && stop.to_bits() == 5.0e-3f64.to_bits())
        {
            return Err(format!("{LABEL} {} typed TRAN changed", role.label()));
        }
        let request = &netlist.output_requests[0];
        let expected_line = if role == Bug986Role::ExplicitBreakpoints {
            9
        } else {
            10
        };
        if request.directive != OutputDirectiveKind::Print
            || request.analysis != Some(rspice_core::netlist::OutputAnalysisKind::Tran)
            || request.name.is_some()
            || request.print_delimiter.as_ref() != Some(&PrintDelimiter::Whitespace)
            || !request.expressions.is_empty()
            || request.origin.line != expected_line
            || request
                .origin
                .path
                .as_deref()
                .is_none_or(|origin_path| !Self::same_path(origin_path, path))
            || request.dependencies.len() != 1
            || request.dependencies[0].kind != OutputSymbolKind::Node
            || request.dependencies[0].expression
            || !request.dependencies[0].operator.eq_ignore_ascii_case("V")
            || !request.dependencies[0].symbol.eq_ignore_ascii_case("1")
        {
            return Err(format!(
                "{LABEL} {} PRINT request changed: {request:?}",
                role.label()
            ));
        }
        Ok(plan)
    }

    fn validate_bug986_table(role: Bug986Role, table: &XycePrnTable) -> Result<(), String> {
        if table.columns.as_slice() != ["Index", "TIME", "v(1)"]
            || table.rows.is_empty()
            || table.rows.iter().enumerate().any(|(index, row)| {
                row.len() != 3
                    || row[0].to_bits() != (index as Value).to_bits()
                    || row.iter().any(|value| !value.is_finite())
            })
            || table.rows.windows(2).any(|rows| rows[0][1] >= rows[1][1])
            || table
                .rows
                .first()
                .is_none_or(|row| row[1].to_bits() != 0.0f64.to_bits())
            || table
                .rows
                .last()
                .is_none_or(|row| row[1].to_bits() != 5.0e-3f64.to_bits())
        {
            return Err(format!(
                "{LABEL} {} produced malformed transient output with columns {:?}",
                role.label(),
                table.columns
            ));
        }
        for ((expected_time, expected_text), index) in SCHEDULE
            .into_iter()
            .zip(BUG981_CORROBORATED_PRN_VALUES)
            .zip(1usize..)
        {
            let row = table
                .rows
                .iter()
                .find(|row| row[1].to_bits() == expected_time.to_bits())
                .ok_or_else(|| format!("{LABEL} {} missed {expected_time}", role.label()))?;
            let expected = 2.0 + 5.0 * (1.0 - (-expected_time / 1.0e-3).exp());
            let error = (row[2] - expected).abs();
            let tolerance = DEFAULT_PRN_HALF_QUANTUM + 8.0 * Value::EPSILON * expected.abs();
            if error > tolerance {
                return Err(format!(
                    "{LABEL} {} has V(1)={} at {expected_time}, analytic {expected} ± {tolerance}",
                    role.label(),
                    row[2]
                ));
            }
            let actual_text = Self::xyce_default_prn_text(row[2])?;
            if actual_text != expected_text {
                return Err(format!(
                    "{LABEL} {} default PRN value {index} is {actual_text}, expected BUG981-corroborated {expected_text}",
                    role.label()
                ));
            }
        }
        Ok(())
    }

    fn validate_bug986_relation(explicit: &XycePrnTable, pwl: &XycePrnTable) -> Result<(), String> {
        Self::validate_bug986_table(Bug986Role::ExplicitBreakpoints, explicit)?;
        Self::validate_bug986_table(Bug986Role::DisconnectedPwlBreakpoints, pwl)?;
        let explicit_text =
            Self::xyce_prn_text_with_delimiter(explicit, &PrintDelimiter::Whitespace)?;
        let pwl_text = Self::xyce_prn_text_with_delimiter(pwl, &PrintDelimiter::Whitespace)?;
        if explicit_text.is_empty() || pwl_text.is_empty() || explicit_text != pwl_text {
            return Err(format!(
                "{LABEL} Release default-PRN byte comparison changed"
            ));
        }
        Ok(())
    }

    fn run_bug986_worker(
        &self,
        role: Bug986Role,
        plan: &XyceStaticTranPlan,
        start: Instant,
    ) -> Result<XycePrnTable, String> {
        let (netlist, result) = self
            .run_transient_family_plan(plan, start, None, None)
            .map_err(|error| match error {
                SimulationError::Aborted => format!("{LABEL} {} exceeded deadline", role.label()),
                other => format!("{LABEL} {} failed: {other}", role.label()),
            })?;
        let table = Self::transient_family_result_to_prn_table(plan, &netlist, &result)
            .map_err(|error| format!("{LABEL} {} PRN failed: {error}", role.label()))?;
        Self::validate_bug986_table(role, &table)?;
        Ok(table)
    }

    pub(super) fn validate_bug986_oracle(
        &self,
        deck: &XyceDeck,
        role: Bug986Role,
        start: Instant,
    ) -> Result<(), String> {
        let abort = DeadlineAbort::new(start, self.config.max_time_per_test_ms.max(1));
        if abort.is_aborted() {
            return Err(format!("{LABEL} deadline expired before provenance"));
        }
        let members = self.validate_bug986_provenance(deck, role)?;
        let mut plans = BTreeMap::new();
        for worker in Bug986Role::WORKERS {
            let bytes = members
                .get(&worker.file_name().to_ascii_lowercase())
                .ok_or_else(|| format!("{LABEL} lost {}", worker.file_name()))?;
            let source = std::str::from_utf8(bytes)
                .map_err(|error| format!("{LABEL} {} is not UTF-8: {error}", worker.file_name()))?;
            plans.insert(
                worker,
                self.validate_bug986_worker(worker, source, &self.root.join(worker.path()))?,
            );
        }
        if abort.is_aborted() {
            return Err(format!("{LABEL} validation exceeded deadline"));
        }
        let explicit = self.run_bug986_worker(
            Bug986Role::ExplicitBreakpoints,
            plans
                .get(&Bug986Role::ExplicitBreakpoints)
                .expect("explicit breakpoint plan"),
            start,
        )?;
        let pwl = self.run_bug986_worker(
            Bug986Role::DisconnectedPwlBreakpoints,
            plans
                .get(&Bug986Role::DisconnectedPwlBreakpoints)
                .expect("disconnected PWL plan"),
            start,
        )?;
        Self::validate_bug986_relation(&explicit, &pwl)?;
        self.validate_bug986_provenance(deck, role)?;
        if abort.is_aborted() {
            return Err(format!("{LABEL} final provenance exceeded deadline"));
        }
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn corpus_root() -> PathBuf {
        Path::new(env!("CARGO_MANIFEST_DIR"))
            .join("../..")
            .join("tests/xyce")
    }

    fn fixture(label: &str) -> (tempfile::TempDir, XyceDeck, XyceTestRunner) {
        let temporary = tempfile::Builder::new()
            .prefix(&format!("rspice-xyce-bug986-{label}-"))
            .tempdir()
            .expect("create BUG986 fixture");
        let root = temporary.path();
        let family = root.join("Netlists/Certification_Tests/BUG_986_SON");
        fs::create_dir_all(&family).expect("create BUG986 family");
        let canonical = corpus_root().join("Netlists/Certification_Tests/BUG_986_SON");
        for (name, ..) in XYCE_BUG986_RETAINED_ARTIFACTS {
            fs::copy(canonical.join(name), family.join(name)).expect("copy BUG986 member");
        }
        fs::write(
            root.join(HARNESS_MANIFEST_FILE),
            format!(
                "{}\t{}\n",
                XYCE_BUG986_OWNER_PATH, REQUIRES_UPSTREAM_WRAPPER_CONTRACT
            ),
        )
        .expect("write wrapper manifest");
        fs::write(
            root.join(UPSTREAM_EXCLUSIONS_MANIFEST_FILE),
            format!(
                "schema_version\t{UPSTREAM_EXCLUSIONS_SCHEMA_VERSION}\nsource_commit\t{UPSTREAM_EXCLUSIONS_SOURCE_COMMIT}\nsource_netlists_tree\t{UPSTREAM_EXCLUSIONS_SOURCE_NETLISTS_TREE}\n{}\t{}\t{}\t{}\n{}\t{}\t{}\t{}\n",
                XYCE_BUG986_BREAKPOINT_PATH,
                XYCE_BUG986_EXCLUSION_SOURCE,
                RSPICE_INDEPENDENTLY_QUALIFIED_DISPOSITION,
                XYCE_BUG986_BREAKPOINT_CONTRACT,
                XYCE_BUG986_PWL_PATH,
                XYCE_BUG986_EXCLUSION_SOURCE,
                RSPICE_INDEPENDENTLY_QUALIFIED_DISPOSITION,
                XYCE_BUG986_PWL_CONTRACT,
            ),
        )
        .expect("write exclusion manifest");
        let runner = XyceTestRunner::new(root, XyceRunnerConfig::default());
        let deck = XyceDeck {
            path: root.join(XYCE_BUG986_OWNER_PATH),
            section: XyceDeckSection::Netlists,
            relative_path: XYCE_BUG986_OWNER_PATH.to_string(),
        };
        (temporary, deck, runner)
    }

    #[test]
    fn bug986_historical_provenance_is_exact() {
        XyceTestRunner::validate_bug986_historical_oracle_provenance()
            .expect("BUG986 historical provenance");
    }

    #[test]
    fn bug986_workers_have_exact_typed_erroption_contracts() {
        let runner = XyceTestRunner::new(corpus_root(), XyceRunnerConfig::default());
        for role in Bug986Role::WORKERS {
            let path = runner.root.join(role.path());
            let source = fs::read_to_string(&path).expect("read BUG986 worker");
            runner
                .validate_bug986_worker(role, &source, &path)
                .expect("validate BUG986 worker");
        }
    }

    #[test]
    fn bug986_oracle_executes_exact_default_prn_relation() {
        let runner = XyceTestRunner::new(corpus_root(), XyceRunnerConfig::default());
        let deck = XyceDeck {
            path: runner.root.join(XYCE_BUG986_OWNER_PATH),
            section: XyceDeckSection::Netlists,
            relative_path: XYCE_BUG986_OWNER_PATH.to_string(),
        };
        runner
            .validate_bug986_oracle(&deck, Bug986Role::WrapperOwner, Instant::now())
            .expect("execute BUG986 relation");
    }

    #[test]
    fn bug986_explicit_breakpoint_counterfactuals_fail_closed() {
        let temporary = tempfile::tempdir().expect("create BUG986 worker fixture");
        let path = temporary.path().join(XYCE_BUG986_BREAKPOINT_PATH);
        fs::create_dir_all(path.parent().expect("worker parent")).expect("create worker family");
        let canonical = fs::read_to_string(corpus_root().join(XYCE_BUG986_BREAKPOINT_PATH))
            .expect("read BUG986 explicit control");
        for mutated in [
            canonical.replace("4ms", "4.5ms"),
            canonical.replace(",4ms", ""),
        ] {
            fs::write(&path, &mutated).expect("write breakpoint mutation");
            let runner = XyceTestRunner::new(temporary.path(), XyceRunnerConfig::default());
            assert!(
                runner
                    .validate_bug986_worker(Bug986Role::ExplicitBreakpoints, &mutated, &path)
                    .is_err(),
                "moving or removing the fourth explicit breakpoint must fail"
            );
        }
    }

    #[test]
    fn bug986_disconnected_pwl_middle_knot_counterfactuals_fail_closed() {
        let temporary = tempfile::tempdir().expect("create BUG986 PWL worker fixture");
        let path = temporary.path().join(XYCE_BUG986_PWL_PATH);
        fs::create_dir_all(path.parent().expect("worker parent")).expect("create worker family");
        let canonical = fs::read_to_string(corpus_root().join(XYCE_BUG986_PWL_PATH))
            .expect("read BUG986 PWL control");
        for mutated in [
            canonical.replace("2ms 0.0 3ms", "2.5ms 0.0 3ms"),
            canonical.replace("2ms 0.0 ", ""),
        ] {
            fs::write(&path, &mutated).expect("write PWL-knot mutation");
            let runner = XyceTestRunner::new(temporary.path(), XyceRunnerConfig::default());
            assert!(
                runner
                    .validate_bug986_worker(
                        Bug986Role::DisconnectedPwlBreakpoints,
                        &mutated,
                        &path,
                    )
                    .is_err(),
                "moving or removing the middle disconnected PWL knot must fail"
            );
        }
    }

    #[test]
    fn bug986_erroption_counterfactuals_fail_closed() {
        let temporary = tempfile::tempdir().expect("create BUG986 ERROPTION fixture");
        let path = temporary.path().join(XYCE_BUG986_BREAKPOINT_PATH);
        fs::create_dir_all(path.parent().expect("worker parent")).expect("create worker family");
        let canonical = fs::read_to_string(corpus_root().join(XYCE_BUG986_BREAKPOINT_PATH))
            .expect("read BUG986 explicit control");
        for mutated in [
            canonical.replace("erroption=1 ", ""),
            canonical.replace("erroption=1", "erroption=0"),
        ] {
            fs::write(&path, &mutated).expect("write ERROPTION mutation");
            let runner = XyceTestRunner::new(temporary.path(), XyceRunnerConfig::default());
            assert!(
                runner
                    .validate_bug986_worker(Bug986Role::ExplicitBreakpoints, &mutated, &path)
                    .is_err(),
                "removing or changing ERROPTION=1 must fail"
            );
        }
    }

    #[test]
    fn bug986_relation_rejects_schedule_and_byte_drift() {
        let rows = std::iter::once((0.0, 2.0))
            .chain(
                SCHEDULE
                    .into_iter()
                    .map(|time| (time, 2.0 + 5.0 * (1.0 - (-time / 1.0e-3).exp()))),
            )
            .enumerate()
            .map(|(index, (time, value))| vec![index as Value, time, value])
            .collect::<Vec<_>>();
        let explicit = XycePrnTable {
            columns: vec!["Index".into(), "TIME".into(), "v(1)".into()],
            rows,
        };
        let mut pwl = explicit.clone();
        XyceTestRunner::validate_bug986_relation(&explicit, &pwl)
            .expect("canonical BUG986 relation");
        pwl.rows[3][2] += 1.0e-8;
        assert!(XyceTestRunner::validate_bug986_relation(&explicit, &pwl).is_err());
        pwl = explicit.clone();
        pwl.rows.remove(4);
        for (index, row) in pwl.rows.iter_mut().enumerate() {
            row[0] = index as Value;
        }
        assert!(XyceTestRunner::validate_bug986_relation(&explicit, &pwl).is_err());
        pwl = explicit.clone();
        pwl.columns[1] = "Time".to_string();
        assert!(
            XyceTestRunner::validate_bug986_relation(&explicit, &pwl).is_err(),
            "default PRN column case must remain exact"
        );
    }

    #[test]
    fn bug986_provenance_rejects_source_role_census_and_owner_drift() {
        let (_temporary, deck, runner) = fixture("source");
        runner
            .validate_bug986_provenance(&deck, Bug986Role::WrapperOwner)
            .expect("canonical BUG986 fixture");
        fs::write(runner.root.join(XYCE_BUG986_BREAKPOINT_PATH), "* mutated\n")
            .expect("mutate worker");
        assert!(
            runner
                .validate_bug986_provenance(&deck, Bug986Role::WrapperOwner)
                .is_err()
        );

        let (_temporary, deck, runner) = fixture("role");
        let manifest = runner.root.join(UPSTREAM_EXCLUSIONS_MANIFEST_FILE);
        let text = fs::read_to_string(&manifest).expect("read exclusions");
        fs::write(
            &manifest,
            text.replace(XYCE_BUG986_BREAKPOINT_CONTRACT, XYCE_BUG986_PWL_CONTRACT),
        )
        .expect("mutate qualification role");
        assert!(
            runner
                .validate_bug986_provenance(&deck, Bug986Role::WrapperOwner)
                .is_err()
        );

        let (_temporary, deck, runner) = fixture("census");
        fs::write(
            runner
                .root
                .join("Netlists/Certification_Tests/BUG_986_SON/extra.txt"),
            "unexpected\n",
        )
        .expect("add unexpected member");
        assert!(
            runner
                .validate_bug986_provenance(&deck, Bug986Role::WrapperOwner)
                .is_err()
        );

        let (_temporary, deck, runner) = fixture("owner");
        fs::write(runner.root.join(HARNESS_MANIFEST_FILE), "").expect("remove wrapper ownership");
        assert!(
            runner
                .validate_bug986_provenance(&deck, Bug986Role::WrapperOwner)
                .is_err()
        );
    }

    #[test]
    fn bug986_provenance_rejects_oversized_member_before_reading_it() {
        let (_temporary, deck, runner) = fixture("oversized");
        let expected_canonical_bytes = XYCE_BUG986_RETAINED_ARTIFACTS[0].1;
        let maximum_raw_bytes = expected_canonical_bytes
            .checked_mul(2)
            .and_then(|value| value.checked_add(3))
            .expect("test size envelope");
        fs::write(
            runner.root.join(XYCE_BUG986_OWNER_PATH),
            vec![b'x'; maximum_raw_bytes + 1],
        )
        .expect("write oversized owner");
        let error = runner
            .validate_bug986_provenance(&deck, Bug986Role::WrapperOwner)
            .expect_err("oversized retained member must fail closed");
        assert!(
            error.contains("bounded read envelope"),
            "oversized member should fail at the pre-read envelope: {error}"
        );
    }

    #[test]
    fn bug986_oracle_rejects_expired_shared_deadline() {
        let runner = XyceTestRunner::new(corpus_root(), XyceRunnerConfig::default());
        let deck = XyceDeck {
            path: runner.root.join(XYCE_BUG986_OWNER_PATH),
            section: XyceDeckSection::Netlists,
            relative_path: XYCE_BUG986_OWNER_PATH.to_string(),
        };
        let expired = Instant::now()
            - Duration::from_millis(
                u64::try_from(runner.config.max_time_per_test_ms.max(1) + 1)
                    .expect("timeout fits u64"),
            );
        assert!(
            runner
                .validate_bug986_oracle(&deck, Bug986Role::WrapperOwner, expired)
                .is_err()
        );
    }
}
