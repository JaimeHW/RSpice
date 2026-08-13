use super::*;

const BUG271_LABEL: &str = "BUG_271_SON tab-comment RLC success wrapper";

impl XyceTestRunner {
    pub(super) fn bug271_historical_oracle_provenance_records() -> Vec<String> {
        let mut records = XYCE_BUG271_HISTORICAL_ARTIFACTS
            .into_iter()
            .map(|(path, bytes, sha256, content_blake3)| {
                format!(
                    "{XYCE_BUG271_UPSTREAM_REGRESSION_COMMIT}\t{XYCE_BUG271_UPSTREAM_RELEASE_TAG}\t{path}\t{bytes}\t{sha256}\t{content_blake3}"
                )
            })
            .collect::<Vec<_>>();
        records.sort();
        records
    }

    pub(super) fn validate_bug271_historical_oracle_provenance() -> Result<(), String> {
        let records = Self::bug271_historical_oracle_provenance_records();
        let stream = records.join("\n");
        let sha256 = format!("{:x}", Sha256::digest(stream.as_bytes()));
        let content_blake3 = blake3::hash(stream.as_bytes()).to_hex().to_string();
        if records.len() != XYCE_BUG271_HISTORICAL_RECORD_COUNT
            || stream.len() != XYCE_BUG271_HISTORICAL_RECORD_BYTES
            || sha256 != XYCE_BUG271_HISTORICAL_RECORDS_SHA256
            || content_blake3 != XYCE_BUG271_HISTORICAL_RECORDS_BLAKE3
        {
            return Err(format!(
                "{BUG271_LABEL} Release-7.10 provenance changed: records={}/{}, sha256={sha256}, blake3={content_blake3}",
                records.len(),
                stream.len()
            ));
        }
        Ok(())
    }

    fn validate_bug271_source_directory(directory: &Path) -> Result<Vec<u8>, String> {
        let metadata = fs::symlink_metadata(directory)
            .map_err(|error| format!("failed to inspect {BUG271_LABEL} directory: {error}"))?;
        if metadata.file_type().is_symlink() || !metadata.file_type().is_dir() {
            return Err(format!(
                "{BUG271_LABEL} source directory must be a regular non-symlink directory"
            ));
        }
        let mut members = fs::read_dir(directory)
            .map_err(|error| format!("failed to read {BUG271_LABEL} directory: {error}"))?
            .collect::<Result<Vec<_>, _>>()
            .map_err(|error| format!("failed to inspect {BUG271_LABEL} member: {error}"))?;
        members.sort_by_key(|entry| entry.file_name());
        let [(expected_name, expected_bytes, expected_sha256, expected_blake3)] =
            XYCE_BUG271_RETAINED_ARTIFACTS;
        if members.len() != 1 {
            return Err(format!(
                "{BUG271_LABEL} source census changed: expected 1 member, got {}",
                members.len()
            ));
        }
        let entry = &members[0];
        let path = entry.path();
        let metadata = fs::symlink_metadata(&path)
            .map_err(|error| format!("failed to inspect {}: {error}", path.display()))?;
        if metadata.file_type().is_symlink() || !metadata.file_type().is_file() {
            return Err(format!(
                "{BUG271_LABEL} member {} must be a regular non-symlink file",
                path.display()
            ));
        }
        let name = entry
            .file_name()
            .to_str()
            .ok_or_else(|| format!("{BUG271_LABEL} member name is not UTF-8"))?
            .to_string();
        if name != expected_name {
            return Err(format!(
                "{BUG271_LABEL} member case/name changed: expected {expected_name:?}, got {name:?}"
            ));
        }
        let bytes = fs::read(&path)
            .map_err(|error| format!("failed to read {}: {error}", path.display()))?;
        let canonical = Self::canonical_lf_text_identity(BUG271_LABEL, &bytes)?;
        let sha256 = format!("{:x}", Sha256::digest(&canonical));
        let content_blake3 = blake3::hash(&canonical).to_hex().to_string();
        if canonical.len() != expected_bytes
            || sha256 != expected_sha256
            || content_blake3 != expected_blake3
        {
            return Err(format!("{BUG271_LABEL} retained source content changed"));
        }
        Ok(bytes)
    }

    fn validate_bug271_provenance(&self, deck: &XyceDeck) -> Result<Vec<u8>, String> {
        Self::validate_bug271_historical_oracle_provenance()?;
        if deck.section != XyceDeckSection::Netlists
            || Self::normalize_manifest_key(&deck.relative_path) != XYCE_BUG271_RECORD
            || Self::normalize_manifest_key(&self.relative_key(&deck.path)) != XYCE_BUG271_RECORD
            || !Self::same_path(&deck.path, &self.root.join(XYCE_BUG271_PATH))
        {
            return Err(format!("recognized {BUG271_LABEL} record is not canonical"));
        }
        let prefix = "netlists/certification_tests/bug_271_son/";
        let owners = self
            .upstream_wrapper_decks
            .iter()
            .filter(|record| record.starts_with(prefix))
            .map(String::as_str)
            .collect::<BTreeSet<_>>();
        if owners != BTreeSet::from([XYCE_BUG271_RECORD]) {
            return Err(format!(
                "{BUG271_LABEL} wrapper ownership changed: {owners:?}"
            ));
        }
        let exclusions = Self::load_upstream_exclusions(&self.root)
            .map_err(|error| format!("{BUG271_LABEL} exclusions invalid: {error}"))?;
        if exclusions.keys().any(|record| record.starts_with(prefix)) {
            return Err(format!("{BUG271_LABEL} must not acquire an exclusion row"));
        }
        let source = Self::validate_bug271_source_directory(
            &self.root.join("Netlists/Certification_Tests/BUG_271_SON"),
        )?;
        self.reject_wrapper_output_artifacts(&deck.path)
            .map_err(|error| format!("{BUG271_LABEL} {error}"))?;
        let output_dir = self.root.join("OutputData/Certification_Tests/BUG_271_SON");
        match fs::symlink_metadata(&output_dir) {
            Err(error) if error.kind() == std::io::ErrorKind::NotFound => {}
            Err(error) => {
                return Err(format!(
                    "failed to inspect {BUG271_LABEL} OutputData: {error}"
                ));
            }
            Ok(_) => return Err(format!("{BUG271_LABEL} must not acquire numerical gold")),
        }
        Ok(source)
    }

    fn bug271_nodes_match(actual: &[String], expected: &[&str]) -> bool {
        actual.len() == expected.len()
            && actual
                .iter()
                .zip(expected)
                .all(|(actual, expected)| actual.eq_ignore_ascii_case(expected))
    }

    fn validate_bug271_typed_contract(
        &self,
        source: &str,
        path: &Path,
    ) -> Result<XyceStaticTranPlan, String> {
        let plan = self.static_tran_plan_for_path_with_purpose(
            path,
            XyceStaticTranPlanPurpose::GeneratedReferenceRelationalFamily,
        )?;
        if plan.deck_path != path
            || plan.source != source
            || !matches!(plan.oracle, XyceStaticTranOracle::None)
            || plan.contract != XyceStaticTranContract::WrapperStatic
            || plan.comparison_mode != XyceStaticTranComparisonMode::Pointwise
            || !plan.steps.is_empty()
            || plan.output_override
            || plan.timeint_conststep
            || plan.wrapper_tolerance.is_some()
            || plan.print.as_ref().is_none_or(|print| {
                print.probes.len() != 2
                    || !print.probes[0].eq_ignore_ascii_case("V(1)")
                    || !print.probes[1]
                        .chars()
                        .filter(|ch| !ch.is_whitespace())
                        .collect::<String>()
                        .eq_ignore_ascii_case("{I(V1)-1.0}")
            })
            || plan.tran.step.to_bits() != 0.01f64.to_bits()
            || plan.tran.stop.to_bits() != 10.0f64.to_bits()
            || plan.tran.start.map(Value::to_bits) != Some(0.0f64.to_bits())
            || plan.tran.max_step.is_some()
            || plan.tran.uic
        {
            return Err(format!("{BUG271_LABEL} transient plan changed: {plan:?}"));
        }
        let netlist = Self::parse_xyce_netlist(source, path)
            .map_err(|error| format!("{BUG271_LABEL} no longer parses: {error}"))?;
        if netlist.elements.len() != 4
            || !netlist.models.is_empty()
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
            || !netlist.params.numeric_parameters().is_empty()
            || !netlist.params.all_parameter_expressions().is_empty()
            || !netlist.params.all_string_params().is_empty()
            || !netlist.params.all_functions().is_empty()
        {
            return Err(format!("{BUG271_LABEL} typed envelope changed"));
        }
        let [resistor, inductor, capacitor, voltage] = netlist.elements.as_slice() else {
            unreachable!("BUG271 typed element count was checked");
        };
        if !resistor.name.eq_ignore_ascii_case("R1")
            || !Self::bug271_nodes_match(&resistor.nodes, &["1", "2"])
            || resistor.provenance != ElementProvenance::Authored
            || !matches!(&resistor.kind, ElementKind::Resistor {
                value,
                value_expr: None,
                model: None,
                instance_params,
                deferred_params,
            } if value.to_bits() == 3.0f64.to_bits()
                && instance_params.is_empty()
                && deferred_params.is_empty())
            || !inductor.name.eq_ignore_ascii_case("L1")
            || !Self::bug271_nodes_match(&inductor.nodes, &["2", "3"])
            || inductor.provenance != ElementProvenance::Authored
            || !matches!(&inductor.kind, ElementKind::Inductor {
                value,
                value_expr: None,
                initial_current: None,
                model: None,
                instance_params,
                deferred_params,
            } if value.to_bits() == 1.0f64.to_bits()
                && instance_params.is_empty()
                && deferred_params.is_empty())
            || !capacitor.name.eq_ignore_ascii_case("C1")
            || !Self::bug271_nodes_match(&capacitor.nodes, &["3", "0"])
            || capacitor.provenance != ElementProvenance::Authored
            || !matches!(&capacitor.kind, ElementKind::Capacitor {
                value,
                value_expr: None,
                initial_voltage: None,
                model: None,
                instance_params,
                deferred_params,
            } if value.to_bits() == 0.5f64.to_bits()
                && instance_params.is_empty()
                && deferred_params.is_empty())
            || !voltage.name.eq_ignore_ascii_case("V1")
            || !Self::bug271_nodes_match(&voltage.nodes, &["1", "0"])
            || voltage.provenance != ElementProvenance::Authored
            || !matches!(&voltage.kind, ElementKind::VoltageSource(
                rspice_core::netlist::SourceSpec::DcTransient { dc_value, transient }
            ) if dc_value.to_bits() == 10.0f64.to_bits()
                && matches!(transient.as_ref(), rspice_core::netlist::SourceSpec::Pulse {
                    v1, v2, delay, rise, fall, width, period, phase,
                    width_defaults_to_zero,
                } if v1.to_bits() == 0.0f64.to_bits()
                    && v2.to_bits() == 10.0f64.to_bits()
                    && delay.to_bits() == 0.0f64.to_bits()
                    && rise.to_bits() == 0.0f64.to_bits()
                    && fall.to_bits() == 0.0f64.to_bits()
                    && width.to_bits() == 10.0f64.to_bits()
                    && period.to_bits() == 10.0f64.to_bits()
                    && phase.to_bits() == 0.0f64.to_bits()
                    && !width_defaults_to_zero))
        {
            return Err(format!("{BUG271_LABEL} exact RLC/PULSE topology changed"));
        }
        Ok(plan)
    }

    pub(super) fn validate_bug271_oracle(
        &self,
        deck: &XyceDeck,
        start: Instant,
    ) -> Result<(), String> {
        let abort = DeadlineAbort::new(start, self.config.max_time_per_test_ms.max(1));
        if abort.is_aborted() {
            return Err(format!("{BUG271_LABEL} deadline expired before validation"));
        }
        let bytes = self.validate_bug271_provenance(deck)?;
        let source = std::str::from_utf8(&bytes)
            .map_err(|error| format!("{BUG271_LABEL} is not UTF-8: {error}"))?;
        let plan = self.validate_bug271_typed_contract(source, &deck.path)?;
        let (netlist, result) = self
            .run_transient_family_plan(&plan, start, None, None)
            .map_err(|error| format!("{BUG271_LABEL} execution failed: {error}"))?;
        let table = Self::transient_family_result_to_prn_table(&plan, &netlist, &result)
            .map_err(|error| format!("{BUG271_LABEL} output observation failed: {error}"))?;
        let voltage_peak = table
            .rows
            .iter()
            .filter_map(|row| row.get(2))
            .copied()
            .fold(0.0f64, Value::max);
        let shifted_current_peak = table
            .rows
            .iter()
            .filter_map(|row| row.get(3))
            .map(|value| value.abs())
            .fold(0.0f64, Value::max);
        if table.columns.len() != 4
            || table.columns[0] != "Index"
            || !table.columns[1].eq_ignore_ascii_case("TIME")
            || table.rows.len() < 100
            || table
                .rows
                .iter()
                .any(|row| row.len() != 4 || row.iter().any(|value| !value.is_finite()))
            || table.rows.first().is_none_or(|row| row[1].abs() > 1.0e-15)
            || table
                .rows
                .last()
                .is_none_or(|row| (row[1] - 10.0).abs() > 1.0e-10)
            || (voltage_peak - 10.0).abs() > 1.0e-10
            || shifted_current_peak < 0.5
        {
            return Err(format!(
                "{BUG271_LABEL} produced an invalid or trivial observation: rows={}, Vpeak={voltage_peak}, shifted-I peak={shifted_current_peak}",
                table.rows.len()
            ));
        }
        if abort.is_aborted() {
            return Err(format!("{BUG271_LABEL} execution exceeded timeout"));
        }
        self.validate_bug271_provenance(deck)?;
        if abort.is_aborted() {
            return Err(format!("{BUG271_LABEL} final provenance exceeded timeout"));
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

    fn canonical_deck(root: &Path) -> XyceDeck {
        XyceDeck {
            path: root.join(XYCE_BUG271_PATH),
            section: XyceDeckSection::Netlists,
            relative_path: XYCE_BUG271_PATH.to_string(),
        }
    }

    fn bug271_fixture(label: &str) -> (tempfile::TempDir, XyceDeck, XyceTestRunner) {
        let source_root = corpus_root();
        let temporary = tempfile::Builder::new()
            .prefix(&format!("rspice-xyce-bug271-{label}-"))
            .tempdir()
            .expect("create BUG271 fixture root");
        let root = temporary.path();
        let family = root.join("Netlists/Certification_Tests/BUG_271_SON");
        fs::create_dir_all(&family).expect("create BUG271 fixture family");
        fs::copy(
            source_root.join(XYCE_BUG271_PATH),
            family.join("bug_271.cir"),
        )
        .expect("copy canonical BUG271 source");
        fs::write(
            root.join(HARNESS_MANIFEST_FILE),
            format!("{XYCE_BUG271_PATH}\t{REQUIRES_UPSTREAM_WRAPPER_CONTRACT}\n"),
        )
        .expect("write BUG271 wrapper manifest");
        fs::write(
            root.join(UPSTREAM_EXCLUSIONS_MANIFEST_FILE),
            format!(
                "schema_version\t{UPSTREAM_EXCLUSIONS_SCHEMA_VERSION}\nsource_commit\t{UPSTREAM_EXCLUSIONS_SOURCE_COMMIT}\nsource_netlists_tree\t{UPSTREAM_EXCLUSIONS_SOURCE_NETLISTS_TREE}\n"
            ),
        )
        .expect("write empty BUG271 exclusion manifest");
        let deck = canonical_deck(root);
        let runner = XyceTestRunner::new(root, XyceRunnerConfig::default());
        (temporary, deck, runner)
    }

    #[test]
    fn bug271_historical_provenance_is_exact() {
        XyceTestRunner::validate_bug271_historical_oracle_provenance()
            .expect("Release-7.10 BUG271 provenance remains exact");
    }

    #[test]
    fn bug271_typed_contract_preserves_tabs_comments_and_rlc_pulse() {
        let root = corpus_root();
        let path = root.join(XYCE_BUG271_PATH);
        let source = fs::read_to_string(&path).expect("read canonical BUG271 source");
        let runner = XyceTestRunner::new(&root, XyceRunnerConfig::default());
        runner
            .validate_bug271_typed_contract(&source, &path)
            .expect("canonical BUG271 typed contract passes");
        for mutation in [
            source.replacen("r1 1 2 3", "r1 1 2 4", 1),
            source.replacen("\tNote:", "Rogue:", 1),
            source.replacen(".tran  0.01 10 0", ".tran 0.02 10 0", 1),
            source.replacen("{i(v1)-1.0}", "{i(v1)-2.0}", 1),
        ] {
            assert!(
                runner
                    .validate_bug271_typed_contract(&mutation, &path)
                    .is_err(),
                "BUG271 semantic/comment mutation must fail closed"
            );
        }
    }

    #[test]
    fn bug271_oracle_executes_the_exact_success_wrapper() {
        let root = corpus_root();
        let runner = XyceTestRunner::new(&root, XyceRunnerConfig::default());
        let deck = XyceDeck {
            path: root.join(XYCE_BUG271_PATH),
            section: XyceDeckSection::Netlists,
            relative_path: XYCE_BUG271_PATH.to_string(),
        };
        runner
            .validate_bug271_oracle(&deck, Instant::now())
            .expect("canonical BUG271 success wrapper passes");
    }

    #[test]
    fn bug271_oracle_rejects_an_expired_deadline() {
        let root = corpus_root();
        let runner = XyceTestRunner::new(
            &root,
            XyceRunnerConfig {
                max_time_per_test_ms: 1,
                ..XyceRunnerConfig::default()
            },
        );
        let deck = XyceDeck {
            path: root.join(XYCE_BUG271_PATH),
            section: XyceDeckSection::Netlists,
            relative_path: XYCE_BUG271_PATH.to_string(),
        };
        let start = Instant::now()
            .checked_sub(Duration::from_millis(2))
            .expect("construct expired BUG271 deadline");
        assert!(runner.validate_bug271_oracle(&deck, start).is_err());
    }

    #[test]
    fn bug271_provenance_rejects_source_manifest_exclusion_and_output_drift() {
        let (_temporary, deck, runner) = bug271_fixture("source-drift");
        let source = fs::read_to_string(&deck.path).expect("read BUG271 fixture source");
        fs::write(&deck.path, source.replace("r1 1 2 3", "r1 1 2 4"))
            .expect("mutate BUG271 retained source");
        assert!(runner.validate_bug271_provenance(&deck).is_err());

        let (_temporary, deck, runner) = bug271_fixture("family-drift");
        runner
            .validate_bug271_provenance(&deck)
            .expect("canonical BUG271 fixture provenance passes");
        fs::write(
            deck.path
                .parent()
                .expect("BUG271 deck has parent")
                .join("unexpected.out"),
            "stale wrapper output\n",
        )
        .expect("write stale BUG271 wrapper output");
        assert!(runner.validate_bug271_provenance(&deck).is_err());

        let (_temporary, deck, runner) = bug271_fixture("output-drift");
        let output = runner
            .static_output_reference_path(&deck.path, "prn")
            .expect("map BUG271 fixture to OutputData");
        fs::create_dir_all(output.parent().expect("BUG271 output has parent"))
            .expect("create forbidden BUG271 OutputData family");
        fs::write(output, "invented gold\n").expect("write forbidden BUG271 gold");
        assert!(runner.validate_bug271_provenance(&deck).is_err());

        let (_temporary, deck, runner) = bug271_fixture("manifest-drift");
        fs::write(runner.root.join(HARNESS_MANIFEST_FILE), "")
            .expect("remove BUG271 wrapper ownership");
        let rebuilt = XyceTestRunner::new(&runner.root, XyceRunnerConfig::default());
        assert!(rebuilt.validate_bug271_provenance(&deck).is_err());

        let (_temporary, deck, runner) = bug271_fixture("exclusion-drift");
        fs::write(
            runner.root.join(UPSTREAM_EXCLUSIONS_MANIFEST_FILE),
            format!(
                "schema_version\t{UPSTREAM_EXCLUSIONS_SCHEMA_VERSION}\nsource_commit\t{UPSTREAM_EXCLUSIONS_SOURCE_COMMIT}\nsource_netlists_tree\t{UPSTREAM_EXCLUSIONS_SOURCE_NETLISTS_TREE}\n{XYCE_BUG271_PATH}\tNetlists/Certification_Tests/BUG_271_SON/exclude\t{UPSTREAM_EXCLUDED_DISPOSITION}\n"
            ),
        )
        .expect("fabricate BUG271 exclusion");
        assert!(runner.validate_bug271_provenance(&deck).is_err());

        let (_temporary, deck, runner) = bug271_fixture("path-drift");
        let wrong = XyceDeck {
            path: deck.path,
            section: XyceDeckSection::Netlists,
            relative_path: "Netlists/Certification_Tests/BUG_271_SON/other.cir".to_string(),
        };
        assert!(runner.validate_bug271_provenance(&wrong).is_err());
    }
}
