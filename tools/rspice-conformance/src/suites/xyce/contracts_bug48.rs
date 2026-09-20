use super::*;

impl XyceTestRunner {
    pub(super) fn bug48_historical_oracle_provenance_records() -> Vec<String> {
        let mut records = XYCE_BUG48_HISTORICAL_ARTIFACTS
            .into_iter()
            .map(|(path, bytes, sha256, content_blake3)| {
                format!(
                    "{XYCE_BUG48_UPSTREAM_REGRESSION_COMMIT}\t{XYCE_BUG48_UPSTREAM_RELEASE_TAG}\t{path}\t{bytes}\t{sha256}\t{content_blake3}"
                )
            })
            .collect::<Vec<_>>();
        records.sort();
        records
    }

    pub(super) fn validate_bug48_historical_oracle_provenance() -> Result<(), String> {
        let records = Self::bug48_historical_oracle_provenance_records();
        let stream = records.join("\n");
        let sha256 = format!("{:x}", Sha256::digest(stream.as_bytes()));
        let content_blake3 = blake3::hash(stream.as_bytes()).to_hex().to_string();
        if records.len() != XYCE_BUG48_HISTORICAL_RECORD_COUNT
            || stream.len() != XYCE_BUG48_HISTORICAL_RECORD_BYTES
            || sha256 != XYCE_BUG48_HISTORICAL_RECORDS_SHA256
            || content_blake3 != XYCE_BUG48_HISTORICAL_RECORDS_BLAKE3
        {
            return Err(format!(
                "BUG_48_SON Release-7.10 wrapper provenance changed: records={}/{}, sha256={sha256}, blake3={content_blake3}",
                records.len(),
                stream.len()
            ));
        }
        Ok(())
    }

    pub(super) fn validate_bug48_provenance(&self, deck: &XyceDeck) -> Result<Vec<u8>, String> {
        const LABEL: &str = "BUG_48_SON LEVEL=54 BSIM4 alias";
        Self::validate_bug48_historical_oracle_provenance()?;
        if deck.section != XyceDeckSection::Netlists
            || Self::normalize_manifest_key(&deck.relative_path) != XYCE_BUG48_RECORD
            || Self::normalize_manifest_key(&self.relative_key(&deck.path)) != XYCE_BUG48_RECORD
            || !Self::same_path(&deck.path, &self.root.join(XYCE_BUG48_PATH))
        {
            return Err(format!(
                "recognized {LABEL} record '{}' is not backed by its exact canonical Netlists path",
                deck.relative_path
            ));
        }
        if !self.requires_upstream_wrapper(&deck.relative_path) {
            return Err(format!("{LABEL} lost removed-wrapper ownership"));
        }
        let family_prefix = "netlists/certification_tests/bug_48_son/";
        let owners = self
            .upstream_wrapper_decks
            .iter()
            .filter(|record| record.starts_with(family_prefix))
            .collect::<Vec<_>>();
        if owners.len() != 1 || owners[0].as_str() != XYCE_BUG48_RECORD {
            return Err(format!(
                "{LABEL} requires its exact single manifest owner, found {owners:?}"
            ));
        }
        let exclusions = Self::load_upstream_exclusions(&self.root)
            .map_err(|error| format!("{LABEL} exclusion manifest is invalid: {error}"))?;
        if exclusions.contains_key(XYCE_BUG48_RECORD) {
            return Err(format!(
                "{LABEL} must not be classified by an upstream exclude sentinel"
            ));
        }

        let family_dir = deck
            .path
            .parent()
            .ok_or_else(|| format!("{LABEL} record has no family directory"))?;
        let family_metadata = fs::symlink_metadata(family_dir)
            .map_err(|error| format!("failed to inspect {LABEL} family: {error}"))?;
        if !family_metadata.file_type().is_dir() || family_metadata.file_type().is_symlink() {
            return Err(format!(
                "{LABEL} family must be a regular non-symlink directory"
            ));
        }
        let expected = XYCE_BUG48_RETAINED_ARTIFACTS
            .into_iter()
            .map(|artifact| (artifact.0.to_ascii_lowercase(), artifact))
            .collect::<BTreeMap<_, _>>();
        let mut observed = BTreeSet::new();
        let mut source = None;
        for entry in fs::read_dir(family_dir)
            .map_err(|error| format!("failed to read {LABEL} family: {error}"))?
        {
            let entry =
                entry.map_err(|error| format!("failed to inspect {LABEL} member: {error}"))?;
            let path = entry.path();
            let metadata = fs::symlink_metadata(&path)
                .map_err(|error| format!("failed to inspect {}: {error}", path.display()))?;
            if !metadata.file_type().is_file() || metadata.file_type().is_symlink() {
                return Err(format!(
                    "{LABEL} member {} must be a regular non-symlink file",
                    path.display()
                ));
            }
            let name = entry
                .file_name()
                .to_str()
                .ok_or_else(|| format!("{LABEL} member name is not UTF-8"))?
                .to_string();
            let key = name.to_ascii_lowercase();
            if !observed.insert(key.clone()) {
                return Err(format!(
                    "{LABEL} family has a case-colliding member {name:?}"
                ));
            }
            let Some((expected_name, expected_bytes, expected_sha256, expected_blake3)) =
                expected.get(&key).copied()
            else {
                return Err(format!(
                    "{LABEL} family acquired unexpected member {name:?}"
                ));
            };
            if name != expected_name {
                return Err(format!(
                    "{LABEL} member case changed: expected {expected_name:?}, got {name:?}"
                ));
            }
            let bytes = fs::read(&path)
                .map_err(|error| format!("failed to read {LABEL} member {name:?}: {error}"))?;
            let canonical = Self::canonical_lf_text_identity(LABEL, &bytes)?;
            let sha256 = format!("{:x}", Sha256::digest(&canonical));
            let content_blake3 = blake3::hash(&canonical).to_hex().to_string();
            if canonical.len() != expected_bytes
                || sha256 != expected_sha256
                || content_blake3 != expected_blake3
            {
                return Err(format!(
                    "{LABEL} retained member {name:?} changed: expected {expected_bytes}/{expected_sha256}/{expected_blake3}, got {}/{sha256}/{content_blake3}",
                    canonical.len()
                ));
            }
            if name == "test.cir" {
                source = Some(bytes);
            }
        }
        if observed.len() != expected.len() || observed.len() != 2 {
            return Err(format!(
                "{LABEL} retained family census changed: expected 2 members, got {}",
                observed.len()
            ));
        }
        self.reject_wrapper_output_artifacts(&deck.path)
            .map_err(|error| format!("{LABEL} {error}"))?;
        let output_anchor = self
            .static_output_reference_path(&deck.path, "prn")
            .ok_or_else(|| format!("{LABEL} cannot be mapped into OutputData"))?;
        let output_family = output_anchor
            .parent()
            .ok_or_else(|| format!("{LABEL} OutputData anchor has no parent"))?;
        match fs::symlink_metadata(output_family) {
            Err(error) if error.kind() == std::io::ErrorKind::NotFound => {}
            Err(error) => {
                return Err(format!(
                    "failed to verify absence of {LABEL} OutputData family: {error}"
                ));
            }
            Ok(_) => {
                return Err(format!(
                    "{LABEL} must not acquire a checked-in numerical oracle at {}",
                    output_family.display()
                ));
            }
        }
        Ok(source.expect("exact retained family includes the source record"))
    }

    pub(super) fn validate_bug48_typed_contract(
        &self,
        source: &str,
        deck_path: &Path,
        start: Instant,
    ) -> Result<(), String> {
        const LABEL: &str = "BUG_48_SON LEVEL=54 BSIM4 alias";
        let plan = self.static_dc_plan_for_source_with_execution_dir(
            deck_path,
            source.to_string(),
            ExpressionDialect::Xyce,
            None,
        )?;
        if plan.deck_path != deck_path
            || plan.execution_dir.is_some()
            || plan.expression_dialect != ExpressionDialect::Xyce
            || plan.parameter_redefinition_policy != ParameterRedefinitionPolicy::UseLast
            || !plan.diagnostics.is_empty()
            || plan.print_format.is_some()
            || plan.dc_data.is_some()
            || !plan.steps.is_empty()
            || !plan.dc.source.eq_ignore_ascii_case("VIN")
            || plan.dc.mode != DcSweepMode::Linear
            || plan.dc.start.to_bits() != 1.0f64.to_bits()
            || plan.dc.stop.to_bits() != 1.0f64.to_bits()
            || plan.dc.step.to_bits() != 0.1f64.to_bits()
            || plan.dc.sweep2.is_some()
            || plan.print.probes != ["V(1)"]
        {
            return Err(format!("{LABEL} static DC plan changed: {plan:?}"));
        }

        let netlist = Self::parse_xyce_netlist(source, deck_path)
            .map_err(|error| format!("{LABEL} source no longer parses: {error}"))?;
        if !netlist.diagnostics.is_empty()
            || netlist.elements.len() != 3
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
        {
            return Err(format!(
                "{LABEL} typed envelope changed: diagnostics={}, elements={}, models={}, analyses={}, outputs={}",
                netlist.diagnostics.len(),
                netlist.elements.len(),
                netlist.models.len(),
                netlist.analyses.len(),
                netlist.output_requests.len()
            ));
        }
        let exact_source = |element: &rspice_core::netlist::Element,
                            name: &str,
                            nodes: [&str; 2],
                            expected: Value|
         -> bool {
            element.name.eq_ignore_ascii_case(name)
                && element.nodes == nodes
                && matches!(&element.kind, ElementKind::VoltageSource(rspice_core::netlist::SourceSpec::Dc(value))
                    if value.to_bits() == expected.to_bits())
                && element.provenance == ElementProvenance::Authored
        };
        if !exact_source(&netlist.elements[0], "VIN", ["1", "0"], 1.0)
            || !exact_source(&netlist.elements[1], "VDD", ["2", "0"], 5.0)
        {
            return Err(format!("{LABEL} exact independent-source topology changed"));
        }
        let mos = &netlist.elements[2];
        if !mos.name.eq_ignore_ascii_case("M1")
            || mos.nodes != ["2", "1", "0", "0"]
            || mos.provenance != ElementProvenance::Authored
            || !matches!(&mos.kind, ElementKind::Mosfet {
                model,
                mos_type: rspice_core::netlist::MosType::Nmos,
                compact_syntax: false,
                instance_params,
                deferred_params,
            } if model.eq_ignore_ascii_case("NMOD")
                && instance_params.is_empty()
                && deferred_params.is_empty())
        {
            return Err(format!("{LABEL} exact four-terminal NMOS instance changed"));
        }
        let model = &netlist.models[0];
        if !model.name.eq_ignore_ascii_case("NMOD")
            || !model.model_type.eq_ignore_ascii_case("NMOS")
            || !matches!(model.params.as_slice(), [(name, value)]
                if name.eq_ignore_ascii_case("LEVEL") && value.to_bits() == 54.0f64.to_bits())
            || !model.expr_params.is_empty()
            || !model.string_params.is_empty()
            || !model.string_vector_params.is_empty()
            || !model.real_vector_params.is_empty()
            || !model.real_vector_expr_params.is_empty()
            || !model.integer_vector_params.is_empty()
        {
            return Err(format!("{LABEL} exact NMOD NMOS LEVEL=54 model changed"));
        }
        if !matches!(&netlist.analyses[0], AnalysisCommand::Dc {
            source,
            start,
            stop,
            step,
            mode: DcSweepMode::Linear,
            sweep2: None,
        } if source.eq_ignore_ascii_case("VIN")
            && start.to_bits() == 1.0f64.to_bits()
            && stop.to_bits() == 1.0f64.to_bits()
            && step.to_bits() == 0.1f64.to_bits())
        {
            return Err(format!("{LABEL} typed one-point DC command changed"));
        }
        let request = &netlist.output_requests[0];
        if request.directive != OutputDirectiveKind::Print
            || request.analysis != Some(rspice_core::netlist::OutputAnalysisKind::Dc)
            || request.name.is_some()
            || !matches!(request.dependencies.as_slice(), [dependency]
                if dependency.kind == OutputSymbolKind::Node
                    && dependency.operator.eq_ignore_ascii_case("V")
                    && dependency.symbol == "1"
                    && !dependency.expression)
        {
            return Err(format!("{LABEL} exact .PRINT DC V(1) request changed"));
        }

        let abort = DeadlineAbort::new(start, self.config.max_time_per_test_ms.max(1));
        let engine = self.create_dc_engine();
        let circuit =
            engine
                .build_circuit_with_abort(&netlist, &abort)
                .map_err(|error| match error {
                    SimulationError::Aborted => format!(
                        "{LABEL} native-device construction exceeded timeout ({}ms)",
                        self.config.max_time_per_test_ms
                    ),
                    other => format!("{LABEL} native-device construction failed: {other}"),
                })?;
        if !circuit.has_bsim4v8_devices() {
            return Err(format!(
                "{LABEL} did not construct its native BSIM4v8 device"
            ));
        }
        let results = engine
            .run_dc_sweep2_spec_with_report_and_abort(
                &netlist,
                &plan.dc.source,
                &plan.dc.primary_spec(),
                None,
                &abort,
            )
            .map_err(|error| match error {
                SimulationError::Aborted => format!(
                    "{LABEL} DC execution exceeded timeout ({}ms)",
                    self.config.max_time_per_test_ms
                ),
                other => format!("{LABEL} DC execution failed: {other}"),
            })?;
        let [point] = results.as_slice() else {
            return Err(format!(
                "{LABEL} expected exactly one DC point, got {}",
                results.len()
            ));
        };
        let voltage_1 = point
            .result
            .try_voltage_named("1")
            .ok_or_else(|| format!("{LABEL} result lost node 1"))?;
        let voltage_2 = point
            .result
            .try_voltage_named("2")
            .ok_or_else(|| format!("{LABEL} result lost node 2"))?;
        let all_finite = point.sweep_value.is_finite()
            && point
                .result
                .node_voltages
                .iter()
                .all(|value| value.is_finite())
            && point
                .result
                .branch_currents
                .iter()
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
        let bsim4_entries = point
            .device_op_report
            .entries
            .iter()
            .filter(|entry| entry.device_kind == "BSIM4")
            .collect::<Vec<_>>();
        if point.sweep_value.to_bits() != 1.0f64.to_bits()
            || !all_finite
            || (voltage_1 - 1.0).abs() > 1.0e-12
            || (voltage_2 - 5.0).abs() > 1.0e-12
            || !point.device_op_report.labels_resolve()
            || bsim4_entries.len() != 1
            || !bsim4_entries[0].name.eq_ignore_ascii_case("M1")
        {
            return Err(format!(
                "{LABEL} finite native BSIM4 DC observation changed: sweep={}, V(1)={voltage_1}, V(2)={voltage_2}, BSIM4 entries={}",
                point.sweep_value,
                bsim4_entries.len()
            ));
        }
        if abort.is_aborted() {
            return Err(format!(
                "{LABEL} observation exceeded timeout ({}ms)",
                self.config.max_time_per_test_ms
            ));
        }
        Ok(())
    }

    pub(super) fn validate_bug48_oracle(
        &self,
        deck: &XyceDeck,
        start: Instant,
    ) -> Result<(), String> {
        let source_bytes = self.validate_bug48_provenance(deck)?;
        let source = std::str::from_utf8(&source_bytes)
            .map_err(|error| format!("BUG_48_SON source is not UTF-8: {error}"))?;
        self.validate_bug48_typed_contract(source, &deck.path, start)?;
        self.validate_bug48_provenance(deck)?;
        let abort = DeadlineAbort::new(start, self.config.max_time_per_test_ms.max(1));
        if abort.is_aborted() {
            return Err(format!(
                "BUG_48_SON post-execution provenance exceeded timeout ({}ms)",
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

    fn bug48_fixture(label: &str) -> (tempfile::TempDir, XyceDeck, XyceTestRunner) {
        let source_root = corpus_root();
        let temporary = tempfile::Builder::new()
            .prefix(&format!("rspice-xyce-bug48-{label}-"))
            .tempdir()
            .expect("create BUG48 fixture root");
        let root = temporary.path();
        let family = root.join("Netlists/Certification_Tests/BUG_48_SON");
        fs::create_dir_all(&family).expect("create BUG48 fixture family");
        for name in ["README", "test.cir"] {
            fs::copy(
                source_root
                    .join("Netlists/Certification_Tests/BUG_48_SON")
                    .join(name),
                family.join(name),
            )
            .expect("copy canonical BUG48 retained member");
        }
        fs::write(
            root.join(HARNESS_MANIFEST_FILE),
            format!("{XYCE_BUG48_PATH}\t{REQUIRES_UPSTREAM_WRAPPER_CONTRACT}\n"),
        )
        .expect("write BUG48 wrapper provenance");
        fs::write(
            root.join(UPSTREAM_EXCLUSIONS_MANIFEST_FILE),
            format!(
                "schema_version\t{UPSTREAM_EXCLUSIONS_SCHEMA_VERSION}\nsource_commit\t{UPSTREAM_EXCLUSIONS_SOURCE_COMMIT}\nsource_netlists_tree\t{UPSTREAM_EXCLUSIONS_SOURCE_NETLISTS_TREE}\n"
            ),
        )
        .expect("write empty BUG48 exclusion provenance");
        let deck = XyceDeck {
            path: root.join(XYCE_BUG48_PATH),
            section: XyceDeckSection::Netlists,
            relative_path: XYCE_BUG48_PATH.to_string(),
        };
        let runner = XyceTestRunner::new(root, XyceRunnerConfig::default());
        (temporary, deck, runner)
    }

    #[test]
    fn bug48_historical_provenance_is_exact() {
        XyceTestRunner::validate_bug48_historical_oracle_provenance()
            .expect("Release-7.10 BUG48 provenance remains exact");
    }

    #[test]
    fn bug48_typed_contract_requires_the_level54_native_bsim4_route() {
        let root = corpus_root();
        let path = root.join(XYCE_BUG48_PATH);
        let source = fs::read_to_string(&path).expect("read canonical BUG48 source");
        let runner = XyceTestRunner::new(&root, XyceRunnerConfig::default());
        runner
            .validate_bug48_typed_contract(&source, &path, Instant::now())
            .expect("canonical BUG48 typed native BSIM4 contract passes");

        for mutated in [
            source.replace("level=54", "level=14"),
            source.replace("level=54", "level=53"),
            source.replace("NMOS level=54", "PMOS level=54"),
            source.replace("M1 2 1 0 0 NMOD", "M1 2 1 0 1 NMOD"),
            source.replace(".DC VIN 1 1 0.1", ".DC VIN 0 1 0.1"),
            source.replace(".print DC V(1)", ".print DC V(2)"),
        ] {
            assert!(
                runner
                    .validate_bug48_typed_contract(&mutated, &path, Instant::now())
                    .is_err(),
                "semantic mutation must fail closed"
            );
        }
    }

    #[test]
    fn bug48_oracle_rejects_an_expired_deadline() {
        let root = corpus_root();
        let runner = XyceTestRunner::new(&root, XyceRunnerConfig::default());
        let deck = XyceDeck {
            path: root.join(XYCE_BUG48_PATH),
            section: XyceDeckSection::Netlists,
            relative_path: XYCE_BUG48_PATH.to_string(),
        };
        let expired_start = Instant::now()
            - Duration::from_millis(runner.config.max_time_per_test_ms.max(1) as u64 + 1);
        let error = runner
            .validate_bug48_oracle(&deck, expired_start)
            .expect_err("an expired BUG48 deadline must fail closed");
        assert!(
            error.contains("timeout"),
            "unexpected deadline error: {error}"
        );
    }

    #[test]
    fn bug48_provenance_rejects_family_and_output_drift() {
        let (temporary, deck, runner) = bug48_fixture("family-drift");
        runner
            .validate_bug48_provenance(&deck)
            .expect("canonical BUG48 fixture provenance passes");
        fs::write(
            deck.path
                .parent()
                .expect("BUG48 deck has parent")
                .join("extra"),
            "drift\n",
        )
        .expect("write unexpected BUG48 family member");
        assert!(
            runner.validate_bug48_provenance(&deck).is_err(),
            "unexpected retained family member must fail closed"
        );
        drop(temporary);

        let (_temporary, deck, runner) = bug48_fixture("output-drift");
        let output = runner
            .static_output_reference_path(&deck.path, "prn")
            .expect("map BUG48 fixture into OutputData");
        fs::create_dir_all(output.parent().expect("BUG48 output has parent"))
            .expect("create forbidden BUG48 OutputData family");
        fs::write(&output, "invented gold\n").expect("write forbidden BUG48 gold");
        assert!(
            runner.validate_bug48_provenance(&deck).is_err(),
            "invented BUG48 numerical gold must fail closed"
        );
    }
}
