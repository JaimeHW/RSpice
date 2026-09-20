use super::*;

const BUG159_LABEL: &str = "BUG_159 BJT TNOM default equivalence";

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
enum Bug159WorkerRole {
    ExplicitTnom,
    ImplicitDefault,
}

impl Bug159WorkerRole {
    const ALL: [Self; 2] = [Self::ExplicitTnom, Self::ImplicitDefault];

    fn path(self) -> &'static str {
        match self {
            Self::ExplicitTnom => XYCE_BUG159_EXPLICIT_PATH,
            Self::ImplicitDefault => XYCE_BUG159_IMPLICIT_PATH,
        }
    }

    fn record(self) -> &'static str {
        match self {
            Self::ExplicitTnom => XYCE_BUG159_EXPLICIT_RECORD,
            Self::ImplicitDefault => XYCE_BUG159_IMPLICIT_RECORD,
        }
    }

    fn source_file_name(self) -> &'static str {
        Path::new(self.path())
            .file_name()
            .and_then(|name| name.to_str())
            .expect("BUG159 constants use UTF-8 file names")
    }

    fn reference_file_name(self) -> &'static str {
        match self {
            Self::ExplicitTnom => "bug_159_1.cir.prn",
            Self::ImplicitDefault => "bug_159_2.cir.prn",
        }
    }

    fn label(self) -> &'static str {
        match self {
            Self::ExplicitTnom => "explicit TNOM=27 worker",
            Self::ImplicitDefault => "implicit default-TNOM worker",
        }
    }
}

#[derive(Debug)]
struct Bug159Provenance {
    sources: BTreeMap<String, Vec<u8>>,
    references: BTreeMap<String, PathBuf>,
}

impl XyceTestRunner {
    pub(super) fn bug159_historical_oracle_provenance_records() -> Vec<String> {
        let mut records = XYCE_BUG159_HISTORICAL_ARTIFACTS
            .into_iter()
            .map(|(path, bytes, sha256, content_blake3)| {
                format!(
                    "{XYCE_BUG159_UPSTREAM_REGRESSION_COMMIT}\t{XYCE_BUG159_UPSTREAM_RELEASE_TAG}\t{path}\t{bytes}\t{sha256}\t{content_blake3}"
                )
            })
            .collect::<Vec<_>>();
        records.sort();
        records
    }

    pub(super) fn validate_bug159_historical_oracle_provenance() -> Result<(), String> {
        let records = Self::bug159_historical_oracle_provenance_records();
        let stream = records.join("\n");
        let sha256 = format!("{:x}", Sha256::digest(stream.as_bytes()));
        let content_blake3 = blake3::hash(stream.as_bytes()).to_hex().to_string();
        if records.len() != XYCE_BUG159_HISTORICAL_RECORD_COUNT
            || stream.len() != XYCE_BUG159_HISTORICAL_RECORD_BYTES
            || sha256 != XYCE_BUG159_HISTORICAL_RECORDS_SHA256
            || content_blake3 != XYCE_BUG159_HISTORICAL_RECORDS_BLAKE3
        {
            return Err(format!(
                "BUG_159 Release-7.10 wrapper provenance changed: records={}/{}, sha256={sha256}, blake3={content_blake3}",
                records.len(),
                stream.len()
            ));
        }
        Ok(())
    }

    fn validate_bug159_directory(
        directory: &Path,
        expected_artifacts: &[(&str, usize, &str, &str)],
        label: &str,
    ) -> Result<BTreeMap<String, Vec<u8>>, String> {
        let metadata = fs::symlink_metadata(directory)
            .map_err(|error| format!("failed to inspect {label} directory: {error}"))?;
        if metadata.file_type().is_symlink() || !metadata.file_type().is_dir() {
            return Err(format!(
                "{label} directory {} must be a regular non-symlink directory",
                directory.display()
            ));
        }
        let expected = expected_artifacts
            .iter()
            .copied()
            .map(|artifact| (artifact.0.to_ascii_lowercase(), artifact))
            .collect::<BTreeMap<_, _>>();
        let mut observed = BTreeMap::new();
        for entry in fs::read_dir(directory)
            .map_err(|error| format!("failed to read {label} directory: {error}"))?
        {
            let entry =
                entry.map_err(|error| format!("failed to inspect {label} member: {error}"))?;
            let path = entry.path();
            let metadata = fs::symlink_metadata(&path)
                .map_err(|error| format!("failed to inspect {}: {error}", path.display()))?;
            if metadata.file_type().is_symlink() || !metadata.file_type().is_file() {
                return Err(format!(
                    "{label} member {} must be a regular non-symlink file",
                    path.display()
                ));
            }
            let name = entry
                .file_name()
                .to_str()
                .ok_or_else(|| format!("{label} member name is not UTF-8"))?
                .to_string();
            let key = name.to_ascii_lowercase();
            if observed.contains_key(&key) {
                return Err(format!(
                    "{label} directory contains case-colliding member {name:?}"
                ));
            }
            let Some((expected_name, expected_bytes, expected_sha256, expected_blake3)) =
                expected.get(&key).copied()
            else {
                return Err(format!(
                    "{label} directory acquired unexpected member {name:?}"
                ));
            };
            if name != expected_name {
                return Err(format!(
                    "{label} member case changed: expected {expected_name:?}, got {name:?}"
                ));
            }
            let bytes = fs::read(&path)
                .map_err(|error| format!("failed to read {label} member {name:?}: {error}"))?;
            let canonical = Self::canonical_lf_text_identity(label, &bytes)?;
            let sha256 = format!("{:x}", Sha256::digest(&canonical));
            let content_blake3 = blake3::hash(&canonical).to_hex().to_string();
            if canonical.len() != expected_bytes
                || sha256 != expected_sha256
                || content_blake3 != expected_blake3
            {
                return Err(format!(
                    "{label} member {name:?} changed: expected {expected_bytes}/{expected_sha256}/{expected_blake3}, got {}/{sha256}/{content_blake3}",
                    canonical.len()
                ));
            }
            observed.insert(key, bytes);
        }
        if observed.len() != expected.len() {
            return Err(format!(
                "{label} directory census changed: expected {} members, got {}",
                expected.len(),
                observed.len()
            ));
        }
        Ok(observed)
    }

    fn validate_bug159_provenance(&self, deck: &XyceDeck) -> Result<Bug159Provenance, String> {
        Self::validate_bug159_historical_oracle_provenance()?;
        if deck.section != XyceDeckSection::Netlists
            || Self::normalize_manifest_key(&deck.relative_path) != XYCE_BUG159_OWNER_RECORD
            || Self::normalize_manifest_key(&self.relative_key(&deck.path))
                != XYCE_BUG159_OWNER_RECORD
            || !Self::same_path(&deck.path, &self.root.join(XYCE_BUG159_OWNER_PATH))
        {
            return Err(format!(
                "recognized {BUG159_LABEL} record '{}' is not backed by its canonical owner path",
                deck.relative_path
            ));
        }
        if !self.requires_upstream_wrapper(&deck.relative_path) {
            return Err(format!("{BUG159_LABEL} lost removed-wrapper ownership"));
        }
        let family_prefix = "netlists/certification_tests/bug_159/";
        let owners = self
            .upstream_wrapper_decks
            .iter()
            .filter(|record| record.starts_with(family_prefix))
            .collect::<Vec<_>>();
        if owners.len() != 1 || owners[0].as_str() != XYCE_BUG159_OWNER_RECORD {
            return Err(format!(
                "{BUG159_LABEL} requires the empty owner as its sole wrapper record, found {owners:?}"
            ));
        }

        let exclusions = Self::load_upstream_exclusions(&self.root)
            .map_err(|error| format!("{BUG159_LABEL} exclusion manifest is invalid: {error}"))?;
        if exclusions.contains_key(XYCE_BUG159_OWNER_RECORD) {
            return Err(format!(
                "{BUG159_LABEL} owner must not be classified by an upstream exclude sentinel"
            ));
        }
        let family_exclusions = exclusions
            .iter()
            .filter(|(record, _)| record.starts_with(family_prefix))
            .collect::<BTreeMap<_, _>>();
        if family_exclusions.len() != 2 {
            return Err(format!(
                "{BUG159_LABEL} requires exactly two independently-qualified workers, found {:?}",
                family_exclusions.keys().collect::<Vec<_>>()
            ));
        }
        for role in Bug159WorkerRole::ALL {
            let exclusion = exclusions.get(role.record()).ok_or_else(|| {
                format!(
                    "{BUG159_LABEL} {} lost its upstream-exclusion qualification",
                    role.label()
                )
            })?;
            if exclusion.source != XYCE_BUG159_EXCLUSION_SOURCE
                || !matches!(&exclusion.disposition,
                    XyceUpstreamExclusionDisposition::RspiceIndependentlyQualified { expected_contract }
                        if expected_contract == "static_prn_dc")
            {
                return Err(format!(
                    "{BUG159_LABEL} {} exclusion provenance changed: {exclusion:?}",
                    role.label()
                ));
            }
        }

        let family_dir = deck
            .path
            .parent()
            .ok_or_else(|| format!("{BUG159_LABEL} owner has no family directory"))?;
        let retained = Self::validate_bug159_directory(
            family_dir,
            &XYCE_BUG159_RETAINED_ARTIFACTS,
            "BUG_159 retained source family",
        )?;
        if retained
            .get("bug_159.cir")
            .is_none_or(|owner| !owner.is_empty())
        {
            return Err(format!(
                "{BUG159_LABEL} owner must remain an exact zero-byte wrapper placeholder"
            ));
        }

        self.reject_wrapper_output_artifacts(&deck.path)
            .map_err(|error| format!("{BUG159_LABEL} owner {error}"))?;
        let output_dir = self.root.join("OutputData/Certification_Tests/BUG_159");
        Self::validate_bug159_directory(
            &output_dir,
            &XYCE_BUG159_RETAINED_OUTPUTS,
            "BUG_159 retained output family",
        )?;

        let mut sources = BTreeMap::new();
        let mut references = BTreeMap::new();
        for role in Bug159WorkerRole::ALL {
            let key = role.source_file_name().to_ascii_lowercase();
            let bytes = retained
                .get(&key)
                .cloned()
                .ok_or_else(|| format!("{BUG159_LABEL} retained family lost {}", role.path()))?;
            let source_path = self.root.join(role.path());
            if !Self::same_path(&source_path, &family_dir.join(role.source_file_name())) {
                return Err(format!(
                    "{BUG159_LABEL} {} path mapping changed",
                    role.label()
                ));
            }
            let reference_path = output_dir.join(role.reference_file_name());
            references.insert(role.record().to_string(), reference_path);
            sources.insert(role.record().to_string(), bytes);
        }
        Ok(Bug159Provenance {
            sources,
            references,
        })
    }

    fn validate_bug159_worker_contract(
        &self,
        role: Bug159WorkerRole,
        source: &str,
        path: &Path,
    ) -> Result<XyceStaticDcPlan, String> {
        let plan = self.static_dc_plan_for_source_with_execution_dir(
            path,
            source.to_string(),
            ExpressionDialect::Xyce,
            None,
        )?;
        let expected_probes = ["V(4)", "I(VMON1)", "I(VMON2)", "V(1)", "V(2)"];
        if plan.deck_path != path
            || plan.execution_dir.is_some()
            || plan.expression_dialect != ExpressionDialect::Xyce
            || plan.parameter_redefinition_policy != ParameterRedefinitionPolicy::UseLast
            || !plan.diagnostics.is_empty()
            || plan.print_format.is_some()
            || plan.dc_data.is_some()
            || !plan.steps.is_empty()
            || !plan.dc.source.eq_ignore_ascii_case("VCC")
            || plan.dc.mode != DcSweepMode::Linear
            || plan.dc.start.to_bits() != 12.0f64.to_bits()
            || plan.dc.stop.to_bits() != 12.0f64.to_bits()
            || plan.dc.step.to_bits() != 1.0f64.to_bits()
            || plan.dc.sweep2.is_some()
            || plan.print.probes != expected_probes
        {
            return Err(format!(
                "{BUG159_LABEL} {} static DC plan changed: {plan:?}",
                role.label()
            ));
        }

        let netlist = Self::parse_xyce_netlist(source, path).map_err(|error| {
            format!("{BUG159_LABEL} {} no longer parses: {error}", role.label())
        })?;
        if !netlist.diagnostics.is_empty()
            || netlist.elements.len() != 6
            || netlist.models.len() != 1
            || netlist.analyses.len() != 1
            || netlist.output_requests.len() != 1
            || !netlist.subcircuits.is_empty()
            || !netlist.data_tables.is_empty()
            || !netlist.measurements.is_empty()
            || !netlist.initial_conditions.is_empty()
            || !netlist.node_sets.is_empty()
            || !netlist.veriloga_includes.is_empty()
            || !netlist.spef_includes.is_empty()
            || !netlist.global_nodes.is_empty()
            || !netlist.params.numeric_parameters().is_empty()
            || !netlist.params.all_string_params().is_empty()
            || !netlist.params.all_functions().is_empty()
            || netlist.options.tnom.is_some()
        {
            return Err(format!(
                "{BUG159_LABEL} {} typed envelope changed",
                role.label()
            ));
        }

        let exact_voltage_source = |element: &rspice_core::netlist::Element,
                                    name: &str,
                                    nodes: [&str; 2],
                                    expected: Value|
         -> bool {
            element.name.eq_ignore_ascii_case(name)
                && element.nodes == nodes
                && element.provenance == ElementProvenance::Authored
                && matches!(&element.kind,
                    ElementKind::VoltageSource(rspice_core::netlist::SourceSpec::Dc(value))
                        if value.to_bits() == expected.to_bits())
        };
        let exact_resistor = |element: &rspice_core::netlist::Element,
                              name: &str,
                              nodes: [&str; 2],
                              expected: Value|
         -> bool {
            element.name.eq_ignore_ascii_case(name)
                && element.nodes == nodes
                && element.provenance == ElementProvenance::Authored
                && matches!(&element.kind, ElementKind::Resistor {
                    value,
                    value_expr: None,
                    model: None,
                    instance_params,
                    deferred_params,
                } if value.to_bits() == expected.to_bits()
                    && instance_params.is_empty()
                    && deferred_params.is_empty())
        };
        if !exact_voltage_source(&netlist.elements[0], "VCC", ["4", "0"], 12.0)
            || !exact_resistor(&netlist.elements[1], "RC", ["3", "4"], 2_000.0)
            || !exact_resistor(&netlist.elements[2], "RB", ["4", "5"], 377_000.0)
            || !exact_voltage_source(&netlist.elements[3], "VMON1", ["5", "1"], 0.0)
            || !exact_voltage_source(&netlist.elements[4], "VMON2", ["3", "2"], 0.0)
        {
            return Err(format!(
                "{BUG159_LABEL} {} source/resistor topology changed",
                role.label()
            ));
        }
        let transistor = &netlist.elements[5];
        if !transistor.name.eq_ignore_ascii_case("Q")
            || transistor.nodes != ["2", "1", "0"]
            || transistor.provenance != ElementProvenance::Authored
            || !matches!(&transistor.kind, ElementKind::Bjt {
                model,
                bjt_type: rspice_core::netlist::BjtType::Npn,
                instance_params,
                deferred_params,
            } if model.eq_ignore_ascii_case("NBJT")
                && instance_params.is_empty()
                && deferred_params.is_empty())
        {
            return Err(format!(
                "{BUG159_LABEL} {} exact three-terminal NPN instance changed",
                role.label()
            ));
        }

        const EXPLICIT_MODEL_PARAMS: [(&str, Value); 2] = [("BF", 100.0), ("TNOM", 27.0)];
        const IMPLICIT_MODEL_PARAMS: [(&str, Value); 1] = [("BF", 100.0)];
        let model = &netlist.models[0];
        let expected_params: &[(&str, Value)] = match role {
            Bug159WorkerRole::ExplicitTnom => &EXPLICIT_MODEL_PARAMS,
            Bug159WorkerRole::ImplicitDefault => &IMPLICIT_MODEL_PARAMS,
        };
        let params_match = model.params.len() == expected_params.len()
            && model.params.iter().zip(expected_params).all(
                |((name, value), (expected_name, expected_value))| {
                    name.eq_ignore_ascii_case(expected_name)
                        && value.to_bits() == (*expected_value).to_bits()
                },
            );
        if !model.name.eq_ignore_ascii_case("NBJT")
            || !model.model_type.eq_ignore_ascii_case("NPN")
            || !params_match
            || !model.expr_params.is_empty()
            || !model.string_params.is_empty()
            || !model.string_vector_params.is_empty()
            || !model.real_vector_params.is_empty()
            || !model.real_vector_expr_params.is_empty()
            || !model.integer_vector_params.is_empty()
        {
            return Err(format!(
                "{BUG159_LABEL} {} exact NBJT model representation changed",
                role.label()
            ));
        }
        if !matches!(&netlist.analyses[0], AnalysisCommand::Dc {
            source,
            start,
            stop,
            step,
            mode: DcSweepMode::Linear,
            sweep2: None,
        } if source.eq_ignore_ascii_case("VCC")
            && start.to_bits() == 12.0f64.to_bits()
            && stop.to_bits() == 12.0f64.to_bits()
            && step.to_bits() == 1.0f64.to_bits())
        {
            return Err(format!(
                "{BUG159_LABEL} {} typed one-point DC command changed",
                role.label()
            ));
        }
        let request = &netlist.output_requests[0];
        if request.directive != OutputDirectiveKind::Print
            || request.analysis != Some(rspice_core::netlist::OutputAnalysisKind::Dc)
            || request.name.is_some()
            || request.dependencies.len() != 5
        {
            return Err(format!(
                "{BUG159_LABEL} {} typed .PRINT request changed",
                role.label()
            ));
        }
        Ok(plan)
    }

    fn run_bug159_worker(
        &self,
        role: Bug159WorkerRole,
        plan: &XyceStaticDcPlan,
        start: Instant,
    ) -> Result<XycePrnTable, String> {
        let (netlist, results) = self
            .run_static_dc_results(plan, start)
            .map_err(|error| format!("{BUG159_LABEL} {} failed: {error}", role.label()))?;
        let [point] = results.as_slice() else {
            return Err(format!(
                "{BUG159_LABEL} {} produced {} DC points instead of one",
                role.label(),
                results.len()
            ));
        };
        let finite = point.sweep_value.is_finite()
            && point
                .result
                .node_voltages
                .iter()
                .chain(point.result.branch_currents.iter())
                .all(|value| value.is_finite())
            && point
                .result
                .dc_observables
                .iter()
                .all(|(_, value)| value.is_finite())
            && point
                .device_op_report
                .entries
                .iter()
                .flat_map(|entry| entry.params.iter().map(|(_, value)| value))
                .all(|value| value.is_finite());
        if point.sweep_value.to_bits() != 12.0f64.to_bits()
            || !finite
            || !point.device_op_report.labels_resolve()
        {
            return Err(format!(
                "{BUG159_LABEL} {} finite one-point observation changed",
                role.label()
            ));
        }
        self.dc_results_to_prn_table(plan, &netlist, &results)
            .map_err(|error| {
                format!(
                    "{BUG159_LABEL} {} default PRN generation failed: {error}",
                    role.label()
                )
            })
    }

    fn require_bug159_serialized_match(
        &self,
        expected_label: &str,
        expected: &XycePrnTable,
        actual_label: &str,
        actual: &XycePrnTable,
    ) -> Result<(), String> {
        let mismatches = self
            .compare_serialized_default_prn_tables(expected, actual)
            .map_err(|error| {
                format!(
                    "{BUG159_LABEL} {expected_label} versus {actual_label} layout comparison failed: {error}"
                )
            })?;
        if !mismatches.is_empty() {
            return Err(format!(
                "{BUG159_LABEL} {expected_label} versus {actual_label} default PRN differs: {mismatches:?}"
            ));
        }
        Ok(())
    }

    pub(super) fn validate_bug159_oracle(
        &self,
        deck: &XyceDeck,
        start: Instant,
    ) -> Result<(), String> {
        let abort = DeadlineAbort::new(start, self.config.max_time_per_test_ms.max(1));
        if abort.is_aborted() {
            return Err(format!(
                "{BUG159_LABEL} deadline expired before provenance validation"
            ));
        }
        let provenance = self.validate_bug159_provenance(deck)?;
        let mut generated = BTreeMap::new();
        for role in Bug159WorkerRole::ALL {
            let source_bytes = provenance
                .sources
                .get(role.record())
                .ok_or_else(|| format!("{BUG159_LABEL} lost {} source", role.label()))?;
            let source = std::str::from_utf8(source_bytes).map_err(|error| {
                format!(
                    "{BUG159_LABEL} {} source is not UTF-8: {error}",
                    role.label()
                )
            })?;
            let path = self.root.join(role.path());
            let plan = self.validate_bug159_worker_contract(role, source, &path)?;
            let table = self.run_bug159_worker(role, &plan, start)?;
            let reference_path = provenance
                .references
                .get(role.record())
                .ok_or_else(|| format!("{BUG159_LABEL} lost {} reference", role.label()))?;
            let reference = Self::parse_prn_file(reference_path).map_err(|error| {
                format!(
                    "{BUG159_LABEL} {} retained reference is invalid: {error}",
                    role.label()
                )
            })?;
            self.require_bug159_serialized_match(
                &format!("{} retained gold", role.label()),
                &reference,
                &format!("{} generated output", role.label()),
                &table,
            )?;
            generated.insert(role, table);
        }
        let explicit = generated
            .get(&Bug159WorkerRole::ExplicitTnom)
            .expect("both BUG159 worker roles are generated");
        let implicit = generated
            .get(&Bug159WorkerRole::ImplicitDefault)
            .expect("both BUG159 worker roles are generated");
        self.require_bug159_serialized_match(
            "explicit TNOM=27 worker",
            explicit,
            "implicit default-TNOM worker",
            implicit,
        )?;
        if abort.is_aborted() {
            return Err(format!(
                "{BUG159_LABEL} execution exceeded timeout ({}ms)",
                self.config.max_time_per_test_ms
            ));
        }
        self.validate_bug159_provenance(deck)?;
        if abort.is_aborted() {
            return Err(format!(
                "{BUG159_LABEL} final provenance exceeded timeout ({}ms)",
                self.config.max_time_per_test_ms
            ));
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

    fn bug159_fixture(label: &str) -> (tempfile::TempDir, XyceDeck, XyceTestRunner) {
        let temporary = tempfile::Builder::new()
            .prefix(&format!("rspice-xyce-bug159-{label}-"))
            .tempdir()
            .expect("create BUG159 fixture root");
        let root = temporary.path().to_path_buf();
        let source_family = root.join("Netlists/Certification_Tests/BUG_159");
        let output_family = root.join("OutputData/Certification_Tests/BUG_159");
        fs::create_dir_all(&source_family).expect("create BUG159 source family");
        fs::create_dir_all(&output_family).expect("create BUG159 output family");
        let canonical_root = corpus_root();
        for (name, ..) in XYCE_BUG159_RETAINED_ARTIFACTS {
            fs::copy(
                canonical_root
                    .join("Netlists/Certification_Tests/BUG_159")
                    .join(name),
                source_family.join(name),
            )
            .expect("copy canonical BUG159 source member");
        }
        for (name, ..) in XYCE_BUG159_RETAINED_OUTPUTS {
            fs::copy(
                canonical_root
                    .join("OutputData/Certification_Tests/BUG_159")
                    .join(name),
                output_family.join(name),
            )
            .expect("copy canonical BUG159 output member");
        }
        fs::write(
            root.join(HARNESS_MANIFEST_FILE),
            format!("{XYCE_BUG159_OWNER_PATH}\t{REQUIRES_UPSTREAM_WRAPPER_CONTRACT}\n"),
        )
        .expect("write BUG159 wrapper manifest");
        fs::write(
            root.join(UPSTREAM_EXCLUSIONS_MANIFEST_FILE),
            format!(
                "schema_version\t{UPSTREAM_EXCLUSIONS_SCHEMA_VERSION}\nsource_commit\t{UPSTREAM_EXCLUSIONS_SOURCE_COMMIT}\nsource_netlists_tree\t{UPSTREAM_EXCLUSIONS_SOURCE_NETLISTS_TREE}\n{XYCE_BUG159_EXPLICIT_PATH}\t{XYCE_BUG159_EXCLUSION_SOURCE}\t{RSPICE_INDEPENDENTLY_QUALIFIED_DISPOSITION}\tstatic_prn_dc\n{XYCE_BUG159_IMPLICIT_PATH}\t{XYCE_BUG159_EXCLUSION_SOURCE}\t{RSPICE_INDEPENDENTLY_QUALIFIED_DISPOSITION}\tstatic_prn_dc\n"
            ),
        )
        .expect("write BUG159 exclusion manifest");
        let deck = XyceDeck {
            path: root.join(XYCE_BUG159_OWNER_PATH),
            section: XyceDeckSection::Netlists,
            relative_path: XYCE_BUG159_OWNER_PATH.to_string(),
        };
        let runner = XyceTestRunner::new(&root, XyceRunnerConfig::default());
        (temporary, deck, runner)
    }

    #[test]
    fn bug159_historical_provenance_is_exact() {
        XyceTestRunner::validate_bug159_historical_oracle_provenance()
            .expect("Release-7.10 BUG159 provenance remains exact");
    }

    #[test]
    fn bug159_workers_preserve_exact_explicit_and_implicit_tnom_contracts() {
        let root = corpus_root();
        let runner = XyceTestRunner::new(&root, XyceRunnerConfig::default());
        for role in Bug159WorkerRole::ALL {
            let path = root.join(role.path());
            let source = fs::read_to_string(&path).expect("read canonical BUG159 worker");
            runner
                .validate_bug159_worker_contract(role, &source, &path)
                .expect("canonical BUG159 worker contract passes");
        }

        let explicit_path = root.join(XYCE_BUG159_EXPLICIT_PATH);
        let explicit = fs::read_to_string(&explicit_path).expect("read explicit BUG159 worker");
        let explicit_mutation = explicit.replace("TNOM=27", "TNOM=28");
        assert!(
            runner
                .validate_bug159_worker_contract(
                    Bug159WorkerRole::ExplicitTnom,
                    &explicit_mutation,
                    &explicit_path,
                )
                .is_err(),
            "a non-default explicit TNOM must fail the BUG159 contract"
        );

        let implicit_path = root.join(XYCE_BUG159_IMPLICIT_PATH);
        let implicit = fs::read_to_string(&implicit_path).expect("read implicit BUG159 worker");
        for mutation in [
            implicit.replace("BF=100", "BF=101"),
            implicit.replace("V(4) I(VMON1)", "I(VMON1) V(4)"),
        ] {
            assert!(
                runner
                    .validate_bug159_worker_contract(
                        Bug159WorkerRole::ImplicitDefault,
                        &mutation,
                        &implicit_path,
                    )
                    .is_err(),
                "model and print-order mutations must fail the BUG159 contract"
            );
        }
    }

    #[test]
    fn bug159_provenance_rejects_family_and_reference_drift() {
        let (temporary, deck, runner) = bug159_fixture("drift");
        runner
            .validate_bug159_provenance(&deck)
            .expect("canonical BUG159 fixture provenance passes");
        fs::write(
            deck.path
                .parent()
                .expect("BUG159 deck has parent")
                .join("unexpected.out"),
            "stale wrapper output\n",
        )
        .expect("write unexpected BUG159 member");
        assert!(runner.validate_bug159_provenance(&deck).is_err());
        drop(temporary);

        let (_temporary, deck, runner) = bug159_fixture("reference-drift");
        fs::write(
            runner
                .root
                .join("OutputData/Certification_Tests/BUG_159/bug_159_1.cir.prn"),
            "mutated reference\n",
        )
        .expect("mutate BUG159 reference");
        assert!(runner.validate_bug159_provenance(&deck).is_err());
    }

    #[test]
    fn bug159_provenance_requires_both_worker_qualifications() {
        let (_temporary, deck, runner) = bug159_fixture("qualification-drift");
        fs::write(
            runner.root.join(UPSTREAM_EXCLUSIONS_MANIFEST_FILE),
            format!(
                "schema_version\t{UPSTREAM_EXCLUSIONS_SCHEMA_VERSION}\nsource_commit\t{UPSTREAM_EXCLUSIONS_SOURCE_COMMIT}\nsource_netlists_tree\t{UPSTREAM_EXCLUSIONS_SOURCE_NETLISTS_TREE}\n{XYCE_BUG159_EXPLICIT_PATH}\t{XYCE_BUG159_EXCLUSION_SOURCE}\t{RSPICE_INDEPENDENTLY_QUALIFIED_DISPOSITION}\tstatic_prn_dc\n"
            ),
        )
        .expect("remove implicit BUG159 qualification");
        assert!(runner.validate_bug159_provenance(&deck).is_err());
    }

    #[test]
    fn bug159_oracle_rejects_an_expired_deadline() {
        let (_temporary, deck, mut runner) = bug159_fixture("deadline");
        runner.config.max_time_per_test_ms = 1;
        let expired_start = Instant::now()
            .checked_sub(Duration::from_millis(2))
            .expect("construct expired BUG159 deadline");
        let error = runner
            .validate_bug159_oracle(&deck, expired_start)
            .expect_err("an expired BUG159 deadline must fail closed");
        assert!(error.contains("deadline"));
    }
}
