use super::*;

const BUG1398_LABEL: &str = "BUG_1398 inductor-model equivalence";

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub(super) enum Bug1398Role {
    ModelOwner,
    LiteralControl,
}

impl Bug1398Role {
    const ALL: [Self; 2] = [Self::ModelOwner, Self::LiteralControl];

    pub(super) fn for_record(record: &str) -> Option<Self> {
        let record = XyceTestRunner::normalize_manifest_key(record);
        Self::ALL.into_iter().find(|role| role.record() == record)
    }

    fn path(self) -> &'static str {
        match self {
            Self::ModelOwner => XYCE_BUG1398_OWNER_PATH,
            Self::LiteralControl => XYCE_BUG1398_CONTROL_PATH,
        }
    }

    fn record(self) -> &'static str {
        match self {
            Self::ModelOwner => XYCE_BUG1398_OWNER_RECORD,
            Self::LiteralControl => XYCE_BUG1398_CONTROL_RECORD,
        }
    }

    pub(super) fn contract(self) -> &'static str {
        match self {
            Self::ModelOwner => XYCE_BUG1398_OWNER_CONTRACT,
            Self::LiteralControl => XYCE_BUG1398_CONTROL_CONTRACT,
        }
    }

    fn file_name(self) -> &'static str {
        match self {
            Self::ModelOwner => "RLC.cir",
            Self::LiteralControl => "RLC_simple.cir",
        }
    }

    fn label(self) -> &'static str {
        match self {
            Self::ModelOwner => "model-card owner",
            Self::LiteralControl => "literal control",
        }
    }

    fn plan_purpose(self) -> XyceStaticTranPlanPurpose {
        match self {
            Self::ModelOwner => XyceStaticTranPlanPurpose::GeneratedReferenceRelationalFamily,
            Self::LiteralControl => XyceStaticTranPlanPurpose::RelationalFamily,
        }
    }
}

impl XyceTestRunner {
    fn bug1398_nodes_match(actual: &[String], expected: &[&str]) -> bool {
        actual.len() == expected.len()
            && actual
                .iter()
                .zip(expected)
                .all(|(actual, expected)| actual.eq_ignore_ascii_case(expected))
    }

    pub(super) fn bug1398_historical_oracle_provenance_records() -> Vec<String> {
        let mut records = XYCE_BUG1398_HISTORICAL_ARTIFACTS
            .into_iter()
            .map(|(path, bytes, sha256, content_blake3)| {
                format!(
                    "{XYCE_BUG1398_UPSTREAM_REGRESSION_COMMIT}\t{XYCE_BUG1398_UPSTREAM_RELEASE_TAG}\t{path}\t{bytes}\t{sha256}\t{content_blake3}"
                )
            })
            .collect::<Vec<_>>();
        records.sort();
        records
    }

    pub(super) fn validate_bug1398_historical_oracle_provenance() -> Result<(), String> {
        let records = Self::bug1398_historical_oracle_provenance_records();
        let stream = records.join("\n");
        let sha256 = format!("{:x}", Sha256::digest(stream.as_bytes()));
        let content_blake3 = blake3::hash(stream.as_bytes()).to_hex().to_string();
        if records.len() != XYCE_BUG1398_HISTORICAL_RECORD_COUNT
            || stream.len() != XYCE_BUG1398_HISTORICAL_RECORD_BYTES
            || sha256 != XYCE_BUG1398_HISTORICAL_RECORDS_SHA256
            || content_blake3 != XYCE_BUG1398_HISTORICAL_RECORDS_BLAKE3
        {
            return Err(format!(
                "{BUG1398_LABEL} Release-7.10 provenance changed: records={}/{}, sha256={sha256}, blake3={content_blake3}",
                records.len(),
                stream.len()
            ));
        }
        Ok(())
    }

    fn validate_bug1398_source_directory(
        directory: &Path,
    ) -> Result<BTreeMap<String, Vec<u8>>, String> {
        let metadata = fs::symlink_metadata(directory)
            .map_err(|error| format!("failed to inspect {BUG1398_LABEL} directory: {error}"))?;
        if metadata.file_type().is_symlink() || !metadata.file_type().is_dir() {
            return Err(format!(
                "{BUG1398_LABEL} source directory must be a regular non-symlink directory"
            ));
        }
        let expected = XYCE_BUG1398_RETAINED_ARTIFACTS
            .into_iter()
            .map(|artifact| (artifact.0.to_ascii_lowercase(), artifact))
            .collect::<BTreeMap<_, _>>();
        let mut observed = BTreeMap::new();
        for entry in fs::read_dir(directory)
            .map_err(|error| format!("failed to read {BUG1398_LABEL} directory: {error}"))?
        {
            let entry = entry.map_err(|error| format!("failed to inspect member: {error}"))?;
            let path = entry.path();
            let metadata = fs::symlink_metadata(&path)
                .map_err(|error| format!("failed to inspect {}: {error}", path.display()))?;
            if metadata.file_type().is_symlink() || !metadata.file_type().is_file() {
                return Err(format!(
                    "{BUG1398_LABEL} member {} must be a regular non-symlink file",
                    path.display()
                ));
            }
            let name = entry
                .file_name()
                .to_str()
                .ok_or_else(|| format!("{BUG1398_LABEL} member name is not UTF-8"))?
                .to_string();
            let key = name.to_ascii_lowercase();
            if observed.contains_key(&key) {
                return Err(format!("{BUG1398_LABEL} contains a case collision"));
            }
            let Some((expected_name, expected_bytes, expected_sha256, expected_blake3)) =
                expected.get(&key).copied()
            else {
                return Err(format!(
                    "{BUG1398_LABEL} acquired unexpected source member {name:?}"
                ));
            };
            if name != expected_name {
                return Err(format!(
                    "{BUG1398_LABEL} member case changed: expected {expected_name:?}, got {name:?}"
                ));
            }
            let bytes = fs::read(&path)
                .map_err(|error| format!("failed to read {}: {error}", path.display()))?;
            let canonical = Self::canonical_lf_text_identity(BUG1398_LABEL, &bytes)?;
            let sha256 = format!("{:x}", Sha256::digest(&canonical));
            let content_blake3 = blake3::hash(&canonical).to_hex().to_string();
            if canonical.len() != expected_bytes
                || sha256 != expected_sha256
                || content_blake3 != expected_blake3
            {
                return Err(format!("{BUG1398_LABEL} member {name:?} content changed"));
            }
            observed.insert(key, bytes);
        }
        if observed.len() != expected.len() {
            return Err(format!(
                "{BUG1398_LABEL} source census changed: expected {}, got {}",
                expected.len(),
                observed.len()
            ));
        }
        Ok(observed)
    }

    fn validate_bug1398_provenance(
        &self,
        deck: &XyceDeck,
        role: Bug1398Role,
    ) -> Result<BTreeMap<Bug1398Role, Vec<u8>>, String> {
        Self::validate_bug1398_historical_oracle_provenance()?;
        if deck.section != XyceDeckSection::Netlists
            || Self::normalize_manifest_key(&deck.relative_path) != role.record()
            || Self::normalize_manifest_key(&self.relative_key(&deck.path)) != role.record()
            || !Self::same_path(&deck.path, &self.root.join(role.path()))
        {
            return Err(format!(
                "recognized {BUG1398_LABEL} {} is not canonical",
                role.label()
            ));
        }
        let prefix = "netlists/certification_tests/bug_1398/";
        let owners = self
            .upstream_wrapper_decks
            .iter()
            .filter(|record| record.starts_with(prefix))
            .map(String::as_str)
            .collect::<BTreeSet<_>>();
        if owners != BTreeSet::from([XYCE_BUG1398_OWNER_RECORD]) {
            return Err(format!(
                "{BUG1398_LABEL} wrapper ownership changed: {owners:?}"
            ));
        }
        let exclusions = Self::load_upstream_exclusions(&self.root)
            .map_err(|error| format!("{BUG1398_LABEL} exclusions invalid: {error}"))?;
        if exclusions.contains_key(XYCE_BUG1398_OWNER_RECORD) {
            return Err(format!("{BUG1398_LABEL} owner must not be excluded"));
        }
        let family = exclusions
            .iter()
            .filter(|(record, _)| record.starts_with(prefix))
            .collect::<Vec<_>>();
        if family.len() != 1
            || family[0].0.as_str() != XYCE_BUG1398_CONTROL_RECORD
            || family[0].1.source != XYCE_BUG1398_EXCLUSION_SOURCE
            || !matches!(&family[0].1.disposition,
                XyceUpstreamExclusionDisposition::RspiceIndependentlyQualified { expected_contract }
                    if expected_contract == XYCE_BUG1398_CONTROL_CONTRACT)
        {
            return Err(format!(
                "{BUG1398_LABEL} control qualification changed: {family:?}"
            ));
        }
        let source_dir = self.root.join("Netlists/Certification_Tests/BUG_1398");
        let retained = Self::validate_bug1398_source_directory(&source_dir)?;
        let output_dir = self.root.join("OutputData/Certification_Tests/BUG_1398");
        match fs::symlink_metadata(&output_dir) {
            Err(error) if error.kind() == std::io::ErrorKind::NotFound => {}
            Err(error) => {
                return Err(format!(
                    "failed to inspect {BUG1398_LABEL} OutputData: {error}"
                ));
            }
            Ok(_) => return Err(format!("{BUG1398_LABEL} must not acquire numerical gold")),
        }
        for member_role in Bug1398Role::ALL {
            self.reject_wrapper_output_artifacts(&self.root.join(member_role.path()))
                .map_err(|error| format!("{BUG1398_LABEL} {} {error}", member_role.label()))?;
        }
        Bug1398Role::ALL
            .into_iter()
            .map(|member_role| {
                retained
                    .get(&member_role.file_name().to_ascii_lowercase())
                    .cloned()
                    .map(|bytes| (member_role, bytes))
                    .ok_or_else(|| format!("{BUG1398_LABEL} lost {}", member_role.file_name()))
            })
            .collect()
    }

    fn validate_bug1398_model(model: &rspice_core::netlist::ModelDef) -> bool {
        if !model.model_type.eq_ignore_ascii_case("L")
            || !model.expr_params.is_empty()
            || !model.string_params.is_empty()
            || !model.string_vector_params.is_empty()
            || !model.real_vector_params.is_empty()
            || !model.real_vector_expr_params.is_empty()
            || !model.integer_vector_params.is_empty()
        {
            return false;
        }
        let expected: &[(&str, Value)] = if model.name.eq_ignore_ascii_case("IND_D") {
            &[("L", 1.0)]
        } else if model.name.eq_ignore_ascii_case("IND_E") {
            &[("L", 2.0)]
        } else if model.name.eq_ignore_ascii_case("IND_F") {
            &[("L", 3.0)]
        } else if model.name.eq_ignore_ascii_case("IND_G") {
            &[("TC1", 0.01), ("TC2", 0.001)]
        } else {
            return false;
        };
        model.params.len() == expected.len()
            && expected.iter().all(|(expected_name, expected_value)| {
                model.params.iter().any(|(name, value)| {
                    name.eq_ignore_ascii_case(expected_name)
                        && value.to_bits() == expected_value.to_bits()
                })
            })
    }

    fn validate_bug1398_worker(
        &self,
        role: Bug1398Role,
        source: &str,
        path: &Path,
    ) -> Result<XyceStaticTranPlan, String> {
        let plan = self.static_tran_plan_for_path_with_purpose(path, role.plan_purpose())?;
        let expected_contract = if role == Bug1398Role::ModelOwner {
            XyceStaticTranContract::WrapperStatic
        } else {
            XyceStaticTranContract::PlainStatic
        };
        let expected_probes = [
            "V(2a)", "V(2b)", "V(2c)", "V(2d)", "V(2e)", "V(2f)", "V(2g)",
        ];
        if plan.deck_path != path
            || plan.source != source
            || !matches!(plan.oracle, XyceStaticTranOracle::None)
            || plan.contract != expected_contract
            || plan.comparison_mode != XyceStaticTranComparisonMode::Pointwise
            || !plan.steps.is_empty()
            || plan.output_override
            || plan.timeint_conststep
            || plan.wrapper_tolerance.is_some()
            || plan.print.as_ref().is_none_or(|print| {
                print.probes.len() != expected_probes.len()
                    || print
                        .probes
                        .iter()
                        .zip(expected_probes)
                        .any(|(actual, expected)| !actual.eq_ignore_ascii_case(expected))
            })
            || plan.tran.step.to_bits() != 2.0e-9f64.to_bits()
            || plan.tran.stop.to_bits() != 8.0e-4f64.to_bits()
            || plan.tran.start.is_some()
            || plan.tran.max_step.is_some()
            || plan.tran.uic
        {
            return Err(format!(
                "{BUG1398_LABEL} {} plan changed: {plan:?}",
                role.label()
            ));
        }
        let netlist = Self::parse_xyce_netlist(source, path).map_err(|error| {
            format!("{BUG1398_LABEL} {} no longer parses: {error}", role.label())
        })?;
        if netlist.elements.len() != 28
            || netlist.models.len() != 4
            || !netlist.models.iter().all(Self::validate_bug1398_model)
            || netlist.analyses.len() != 1
            || netlist.output_requests.len() != 1
            || !netlist.subcircuits.is_empty()
            || !netlist.diagnostics.is_empty()
            || !netlist.data_tables.is_empty()
            || !netlist.measurements.is_empty()
            || !netlist.initial_conditions.is_empty()
            || !netlist.node_sets.is_empty()
            || !netlist.global_nodes.is_empty()
            || !netlist.veriloga_includes.is_empty()
            || !netlist.spef_includes.is_empty()
            || netlist.options.temp.map(Value::to_bits) != Some(37.0f64.to_bits())
            || netlist.params.numeric_parameters().len() != 6
            || ![
                ("ML1", 1.0_f64),
                ("ML2", 2.0_f64),
                ("ML3", 3.0_f64),
                ("TEMP", 37.0_f64),
                ("TEMPER", 37.0_f64),
                ("VT", 0.026_726_659_112_543_266_f64),
            ]
            .iter()
            .all(|(name, value)| {
                netlist
                    .params
                    .get(name)
                    .is_some_and(|actual| actual.to_bits() == value.to_bits())
            })
            || !netlist.params.all_string_params().is_empty()
            || !netlist.params.all_functions().is_empty()
        {
            return Err(format!(
                "{BUG1398_LABEL} {} typed envelope changed: elements={}, models={:?}, analyses={}, outputs={}, diagnostics={:?}, temp={:?}, numeric={:?}, expressions={:?}",
                role.label(),
                netlist.elements.len(),
                netlist.models,
                netlist.analyses.len(),
                netlist.output_requests.len(),
                netlist.diagnostics,
                netlist.options.temp,
                netlist.params.numeric_parameters(),
                netlist.params.all_parameter_expressions(),
            ));
        }
        let expected_inductors: &[(&str, Value, Option<&str>)] = match role {
            Bug1398Role::ModelOwner => &[
                ("L1a", 0.005, None),
                ("L1b", 0.01, None),
                ("L1c", 0.015, None),
                ("L1d", 0.005, Some("IND_D")),
                ("L1e", 0.005, Some("IND_E")),
                ("L1f", 0.005, Some("IND_F")),
                ("L1g", 0.005, Some("IND_G")),
            ],
            Bug1398Role::LiteralControl => &[
                ("L1a", 0.005, None),
                ("L1b", 0.01, None),
                ("L1c", 0.015, None),
                ("L1d", 0.005, None),
                ("L1e", 0.01, None),
                ("L1f", 0.015, None),
                ("L1g", 0.006_000_000_000_000_001, None),
            ],
        };
        let pulse_delays: [Value; 7] = [1.0e-5, 1.0e-5, 1.0e-5, 4.0e-4, 4.5e-4, 5.0e-4, 5.0e-4];
        for (branch, ((name, expected_value, expected_model), expected_delay)) in
            expected_inductors.iter().zip(pulse_delays).enumerate()
        {
            let suffix = (b'a' + branch as u8) as char;
            let [source, resistor, element, capacitor] =
                &netlist.elements[branch * 4..branch * 4 + 4]
            else {
                unreachable!("BUG1398 typed element count was checked");
            };
            let source_name = format!("VPULSE{suffix}");
            let node_one = format!("1{suffix}");
            let node_two = format!("2{suffix}");
            if !source.name.eq_ignore_ascii_case(&source_name)
                || !Self::bug1398_nodes_match(
                    &source.nodes,
                    &[node_two.as_str(), node_one.as_str()],
                )
                || source.provenance != ElementProvenance::Authored
                || !matches!(&source.kind, ElementKind::VoltageSource(rspice_core::netlist::SourceSpec::Pulse {
                    v1, v2, delay, rise, fall, width, period, phase, width_defaults_to_zero,
                }) if v1.to_bits() == 0.0f64.to_bits()
                    && v2.to_bits() == 1.0f64.to_bits()
                    && delay.to_bits() == expected_delay.to_bits()
                    && rise.to_bits() == 1.0e-4f64.to_bits()
                    && fall.to_bits() == 1.0e-4f64.to_bits()
                    && width.to_bits() == 1.0e-6f64.to_bits()
                    && period.to_bits() == 1.0e-3f64.to_bits()
                    && phase.to_bits() == 0.0f64.to_bits()
                    && !width_defaults_to_zero)
                || !resistor.name.eq_ignore_ascii_case(&format!("R1{suffix}"))
                || !Self::bug1398_nodes_match(&resistor.nodes, &[node_one.as_str(), "0"])
                || resistor.provenance != ElementProvenance::Authored
                || !matches!(&resistor.kind, ElementKind::Resistor {
                    value,
                    value_expr: None,
                    model: None,
                    instance_params,
                    deferred_params,
                } if value.to_bits() == 400.0f64.to_bits()
                    && instance_params.is_empty()
                    && deferred_params.is_empty())
                || !capacitor.name.eq_ignore_ascii_case(&format!("C1{suffix}"))
                || !Self::bug1398_nodes_match(&capacitor.nodes, &[node_one.as_str(), "0"])
                || capacitor.provenance != ElementProvenance::Authored
                || !matches!(&capacitor.kind, ElementKind::Capacitor {
                    value,
                    value_expr: None,
                    initial_voltage: None,
                    model: None,
                    instance_params,
                    deferred_params,
                } if value.to_bits() == 1.0e-8f64.to_bits()
                    && instance_params.is_empty()
                    && deferred_params.is_empty())
            {
                return Err(format!(
                    "{BUG1398_LABEL} {} branch {suffix} topology changed",
                    role.label()
                ));
            }
            if !matches!(&element.kind, ElementKind::Inductor {
                value,
                value_expr: None,
                initial_current: None,
                model,
                instance_params,
                deferred_params,
            } if value.to_bits() == expected_value.to_bits()
                && match (model.as_deref(), *expected_model) {
                    (Some(actual), Some(expected)) => actual.eq_ignore_ascii_case(expected),
                    (None, None) => true,
                    _ => false,
                }
                && instance_params.is_empty()
                && deferred_params.is_empty())
                || !element.name.eq_ignore_ascii_case(name)
                || !Self::bug1398_nodes_match(&element.nodes, &[node_two.as_str(), "0"])
                || element.provenance != ElementProvenance::Authored
            {
                return Err(format!(
                    "{BUG1398_LABEL} {} {name} changed: {element:?}",
                    role.label()
                ));
            }
        }
        Ok(plan)
    }

    fn run_bug1398_worker(
        &self,
        role: Bug1398Role,
        plan: &XyceStaticTranPlan,
        start: Instant,
    ) -> Result<XycePrnTable, String> {
        let (netlist, result) = self
            .run_transient_family_plan(plan, start, None, None)
            .map_err(|error| format!("{BUG1398_LABEL} {} failed: {error}", role.label()))?;
        let table = Self::transient_family_result_to_prn_table(plan, &netlist, &result)
            .map_err(|error| format!("{BUG1398_LABEL} {} PRN failed: {error}", role.label()))?;
        if table.columns.len() != 9
            || table.columns[0] != "Index"
            || !table.columns[1].eq_ignore_ascii_case("TIME")
            || table.rows.len() < 3
            || table
                .rows
                .iter()
                .any(|row| row.len() != 9 || row.iter().any(|value| !value.is_finite()))
            || table.rows.first().is_none_or(|row| row[1].abs() > 1.0e-15)
            || table
                .rows
                .last()
                .is_none_or(|row| (row[1] - 8.0e-4).abs() > 1.0e-12)
            || table
                .rows
                .iter()
                .flat_map(|row| row[2..].iter())
                .map(|value| value.abs())
                .fold(0.0, Value::max)
                < 1.0e-3
        {
            return Err(format!(
                "{BUG1398_LABEL} {} produced an invalid or trivial table",
                role.label()
            ));
        }
        Ok(table)
    }

    pub(super) fn validate_bug1398_oracle(
        &self,
        deck: &XyceDeck,
        role: Bug1398Role,
        start: Instant,
    ) -> Result<(), String> {
        let abort = DeadlineAbort::new(start, self.config.max_time_per_test_ms.max(1));
        if abort.is_aborted() {
            return Err(format!(
                "{BUG1398_LABEL} deadline expired before validation"
            ));
        }
        let sources = self.validate_bug1398_provenance(deck, role)?;
        let mut outputs = BTreeMap::new();
        for worker_role in Bug1398Role::ALL {
            let bytes = sources
                .get(&worker_role)
                .ok_or_else(|| format!("{BUG1398_LABEL} lost {}", worker_role.label()))?;
            let source = std::str::from_utf8(bytes).map_err(|error| {
                format!(
                    "{BUG1398_LABEL} {} is not UTF-8: {error}",
                    worker_role.label()
                )
            })?;
            let path = self.root.join(worker_role.path());
            let plan = self.validate_bug1398_worker(worker_role, source, &path)?;
            outputs.insert(
                worker_role,
                self.run_bug1398_worker(worker_role, &plan, start)?,
            );
        }
        let owner = outputs
            .get(&Bug1398Role::ModelOwner)
            .expect("both BUG1398 workers ran");
        let control = outputs
            .get(&Bug1398Role::LiteralControl)
            .expect("both BUG1398 workers ran");
        let mismatches = self
            .compare_xyce_verify_transient_tables(control, owner)
            .map_err(|error| format!("{BUG1398_LABEL} xyce_verify relation failed: {error}"))?;
        if !mismatches.is_empty() {
            return Err(format!(
                "{BUG1398_LABEL} owner differs from control: {mismatches:?}"
            ));
        }
        if abort.is_aborted() {
            return Err(format!("{BUG1398_LABEL} execution exceeded timeout"));
        }
        self.validate_bug1398_provenance(deck, role)?;
        if abort.is_aborted() {
            return Err(format!("{BUG1398_LABEL} final provenance exceeded timeout"));
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

    fn bug1398_exclusion_manifest(disposition: &str) -> String {
        format!(
            "schema_version\t{UPSTREAM_EXCLUSIONS_SCHEMA_VERSION}\nsource_commit\t{UPSTREAM_EXCLUSIONS_SOURCE_COMMIT}\nsource_netlists_tree\t{UPSTREAM_EXCLUSIONS_SOURCE_NETLISTS_TREE}\n{XYCE_BUG1398_CONTROL_PATH}\t{XYCE_BUG1398_EXCLUSION_SOURCE}\t{disposition}\n"
        )
    }

    fn bug1398_fixture(label: &str) -> (tempfile::TempDir, XyceDeck, XyceTestRunner) {
        let temporary = tempfile::Builder::new()
            .prefix(&format!("rspice-xyce-bug1398-{label}-"))
            .tempdir()
            .expect("create BUG1398 fixture root");
        let root = temporary.path().to_path_buf();
        let family = root.join("Netlists/Certification_Tests/BUG_1398");
        fs::create_dir_all(&family).expect("create BUG1398 source family");
        let canonical = corpus_root().join("Netlists/Certification_Tests/BUG_1398");
        for (name, ..) in XYCE_BUG1398_RETAINED_ARTIFACTS {
            fs::copy(canonical.join(name), family.join(name))
                .expect("copy canonical BUG1398 source member");
        }
        fs::write(
            root.join(HARNESS_MANIFEST_FILE),
            format!("{XYCE_BUG1398_OWNER_PATH}\t{REQUIRES_UPSTREAM_WRAPPER_CONTRACT}\n"),
        )
        .expect("write BUG1398 wrapper manifest");
        fs::write(
            root.join(UPSTREAM_EXCLUSIONS_MANIFEST_FILE),
            bug1398_exclusion_manifest(&format!(
                "{RSPICE_INDEPENDENTLY_QUALIFIED_DISPOSITION}\t{XYCE_BUG1398_CONTROL_CONTRACT}"
            )),
        )
        .expect("write BUG1398 exclusion manifest");
        let deck = XyceDeck {
            path: root.join(XYCE_BUG1398_OWNER_PATH),
            section: XyceDeckSection::Netlists,
            relative_path: XYCE_BUG1398_OWNER_PATH.to_string(),
        };
        let runner = XyceTestRunner::new(&root, XyceRunnerConfig::default());
        (temporary, deck, runner)
    }

    #[test]
    fn bug1398_historical_provenance_is_exact() {
        XyceTestRunner::validate_bug1398_historical_oracle_provenance()
            .expect("Release-7.10 BUG1398 provenance remains exact");
    }

    #[test]
    fn bug1398_workers_preserve_model_and_literal_inductances() {
        let root = corpus_root();
        let runner = XyceTestRunner::new(&root, XyceRunnerConfig::default());
        for role in Bug1398Role::ALL {
            let path = root.join(role.path());
            let source = fs::read_to_string(&path).expect("read BUG1398 worker");
            runner
                .validate_bug1398_worker(role, &source, &path)
                .unwrap_or_else(|error| panic!("canonical {role:?} failed: {error}"));
            assert!(
                runner
                    .validate_bug1398_worker(role, &source.replacen("mL1=1.0", "mL1=1.1", 1), &path)
                    .is_err()
            );
        }
    }

    #[test]
    fn bug1398_oracle_runs_both_roles() {
        let root = corpus_root();
        let runner = XyceTestRunner::new(&root, XyceRunnerConfig::default());
        let deck = XyceDeck {
            path: root.join(XYCE_BUG1398_OWNER_PATH),
            section: XyceDeckSection::Netlists,
            relative_path: XYCE_BUG1398_OWNER_PATH.to_string(),
        };
        runner
            .validate_bug1398_oracle(&deck, Bug1398Role::ModelOwner, Instant::now())
            .expect("canonical BUG1398 relation passes");
    }

    #[test]
    fn bug1398_oracle_rejects_an_expired_deadline() {
        let root = corpus_root();
        let runner = XyceTestRunner::new(
            &root,
            XyceRunnerConfig {
                max_time_per_test_ms: 1,
                ..XyceRunnerConfig::default()
            },
        );
        let deck = XyceDeck {
            path: root.join(XYCE_BUG1398_OWNER_PATH),
            section: XyceDeckSection::Netlists,
            relative_path: XYCE_BUG1398_OWNER_PATH.to_string(),
        };
        let start = Instant::now()
            .checked_sub(Duration::from_millis(2))
            .expect("construct expired BUG1398 deadline");
        assert!(
            runner
                .validate_bug1398_oracle(&deck, Bug1398Role::ModelOwner, start)
                .is_err()
        );
    }

    #[test]
    fn bug1398_provenance_rejects_source_output_ownership_and_qualification_drift() {
        let (_temporary, deck, runner) = bug1398_fixture("source-drift");
        runner
            .validate_bug1398_provenance(&deck, Bug1398Role::ModelOwner)
            .expect("canonical BUG1398 fixture provenance passes");
        fs::write(
            deck.path
                .parent()
                .expect("BUG1398 owner has parent")
                .join("RLC.cir.out"),
            "stale wrapper output\n",
        )
        .expect("write stale BUG1398 wrapper output");
        assert!(
            runner
                .validate_bug1398_provenance(&deck, Bug1398Role::ModelOwner)
                .is_err()
        );

        let (_temporary, deck, runner) = bug1398_fixture("output-drift");
        let output = runner.root.join("OutputData/Certification_Tests/BUG_1398");
        fs::create_dir_all(&output).expect("create forbidden BUG1398 OutputData");
        assert!(
            runner
                .validate_bug1398_provenance(&deck, Bug1398Role::ModelOwner)
                .is_err()
        );

        let (_temporary, deck, runner) = bug1398_fixture("owner-drift");
        fs::write(runner.root.join(HARNESS_MANIFEST_FILE), "")
            .expect("remove BUG1398 wrapper ownership");
        let refreshed = XyceTestRunner::new(&runner.root, XyceRunnerConfig::default());
        assert!(
            refreshed
                .validate_bug1398_provenance(&deck, Bug1398Role::ModelOwner)
                .is_err()
        );

        let (_temporary, deck, runner) = bug1398_fixture("qualification-drift");
        fs::write(
            runner.root.join(UPSTREAM_EXCLUSIONS_MANIFEST_FILE),
            bug1398_exclusion_manifest(UPSTREAM_EXCLUDED_DISPOSITION),
        )
        .expect("demote BUG1398 literal control");
        let refreshed = XyceTestRunner::new(&runner.root, XyceRunnerConfig::default());
        assert!(
            refreshed
                .validate_bug1398_provenance(&deck, Bug1398Role::ModelOwner)
                .is_err()
        );
    }
}
