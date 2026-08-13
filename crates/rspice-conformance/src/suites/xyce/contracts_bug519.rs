use super::*;
use rspice_core::io::{RawExporter, RawFormat, RawWaveformData, parse_raw_reader};
use std::io::Cursor;

const BUG519_LABEL: &str = "BUG_519_SON binary/ASCII RAW wrapper";

impl XyceTestRunner {
    pub(super) fn bug519_historical_oracle_provenance_records() -> Vec<String> {
        let mut records = XYCE_BUG519_HISTORICAL_ARTIFACTS
            .into_iter()
            .map(|(path, bytes, sha256, content_blake3)| {
                format!(
                    "{XYCE_BUG519_UPSTREAM_REGRESSION_COMMIT}\t{XYCE_BUG519_UPSTREAM_RELEASE_TAG}\t{path}\t{bytes}\t{sha256}\t{content_blake3}"
                )
            })
            .collect::<Vec<_>>();
        records.sort();
        records
    }

    pub(super) fn validate_bug519_historical_oracle_provenance() -> Result<(), String> {
        let records = Self::bug519_historical_oracle_provenance_records();
        let stream = records.join("\n");
        let sha256 = format!("{:x}", Sha256::digest(stream.as_bytes()));
        let content_blake3 = blake3::hash(stream.as_bytes()).to_hex().to_string();
        if records.len() != XYCE_BUG519_HISTORICAL_RECORD_COUNT
            || stream.len() != XYCE_BUG519_HISTORICAL_RECORD_BYTES
            || sha256 != XYCE_BUG519_HISTORICAL_RECORDS_SHA256
            || content_blake3 != XYCE_BUG519_HISTORICAL_RECORDS_BLAKE3
        {
            return Err(format!(
                "{BUG519_LABEL} Release-7.10 provenance changed: records={}/{}, sha256={sha256}, blake3={content_blake3}",
                records.len(),
                stream.len()
            ));
        }
        Ok(())
    }

    fn validate_bug519_source_directory(directory: &Path) -> Result<Vec<u8>, String> {
        let metadata = fs::symlink_metadata(directory)
            .map_err(|error| format!("failed to inspect {BUG519_LABEL} directory: {error}"))?;
        if metadata.file_type().is_symlink() || !metadata.file_type().is_dir() {
            return Err(format!(
                "{BUG519_LABEL} source directory must be a regular non-symlink directory"
            ));
        }
        let mut members = fs::read_dir(directory)
            .map_err(|error| format!("failed to read {BUG519_LABEL} directory: {error}"))?
            .collect::<Result<Vec<_>, _>>()
            .map_err(|error| format!("failed to inspect {BUG519_LABEL} member: {error}"))?;
        members.sort_by_key(|entry| entry.file_name());
        let [(expected_name, expected_bytes, expected_sha256, expected_blake3)] =
            XYCE_BUG519_RETAINED_ARTIFACTS;
        if members.len() != 1 {
            return Err(format!(
                "{BUG519_LABEL} source census changed: expected 1 member, got {}",
                members.len()
            ));
        }
        let entry = &members[0];
        let path = entry.path();
        let metadata = fs::symlink_metadata(&path)
            .map_err(|error| format!("failed to inspect {}: {error}", path.display()))?;
        if metadata.file_type().is_symlink() || !metadata.file_type().is_file() {
            return Err(format!(
                "{BUG519_LABEL} member {} must be a regular non-symlink file",
                path.display()
            ));
        }
        let file_name = entry.file_name();
        let name = file_name
            .to_str()
            .ok_or_else(|| format!("{BUG519_LABEL} member name is not UTF-8"))?;
        if name != expected_name {
            return Err(format!(
                "{BUG519_LABEL} member case/name changed: expected {expected_name:?}, got {name:?}"
            ));
        }
        let bytes = fs::read(&path)
            .map_err(|error| format!("failed to read {}: {error}", path.display()))?;
        let canonical = Self::canonical_lf_text_identity(BUG519_LABEL, &bytes)?;
        let sha256 = format!("{:x}", Sha256::digest(&canonical));
        let content_blake3 = blake3::hash(&canonical).to_hex().to_string();
        if canonical.len() != expected_bytes
            || sha256 != expected_sha256
            || content_blake3 != expected_blake3
        {
            return Err(format!("{BUG519_LABEL} retained source content changed"));
        }
        Ok(bytes)
    }

    fn validate_bug519_provenance(&self, deck: &XyceDeck) -> Result<Vec<u8>, String> {
        Self::validate_bug519_historical_oracle_provenance()?;
        if deck.section != XyceDeckSection::Netlists
            || Self::normalize_manifest_key(&deck.relative_path) != XYCE_BUG519_RECORD
            || Self::normalize_manifest_key(&self.relative_key(&deck.path)) != XYCE_BUG519_RECORD
            || !Self::same_path(&deck.path, &self.root.join(XYCE_BUG519_PATH))
        {
            return Err(format!("recognized {BUG519_LABEL} record is not canonical"));
        }
        let prefix = "netlists/certification_tests/bug_519_son/";
        let owners = self
            .upstream_wrapper_decks
            .iter()
            .filter(|record| record.starts_with(prefix))
            .map(String::as_str)
            .collect::<BTreeSet<_>>();
        if owners != BTreeSet::from([XYCE_BUG519_RECORD]) {
            return Err(format!(
                "{BUG519_LABEL} wrapper ownership changed: {owners:?}"
            ));
        }
        let exclusions = Self::load_upstream_exclusions(&self.root)
            .map_err(|error| format!("{BUG519_LABEL} exclusions invalid: {error}"))?;
        if exclusions.keys().any(|record| record.starts_with(prefix)) {
            return Err(format!("{BUG519_LABEL} must not acquire an exclusion row"));
        }
        let source = Self::validate_bug519_source_directory(
            &self.root.join("Netlists/Certification_Tests/BUG_519_SON"),
        )?;
        self.reject_wrapper_output_artifacts(&deck.path)
            .map_err(|error| format!("{BUG519_LABEL} {error}"))?;
        let output_dir = self.root.join("OutputData/Certification_Tests/BUG_519_SON");
        match fs::symlink_metadata(&output_dir) {
            Err(error) if error.kind() == std::io::ErrorKind::NotFound => {}
            Err(error) => {
                return Err(format!(
                    "failed to inspect {BUG519_LABEL} OutputData: {error}"
                ));
            }
            Ok(_) => return Err(format!("{BUG519_LABEL} must not acquire numerical gold")),
        }
        Ok(source)
    }

    fn bug519_nodes_match(actual: &[String], expected: &[&str]) -> bool {
        actual.len() == expected.len()
            && actual
                .iter()
                .zip(expected)
                .all(|(actual, expected)| actual.eq_ignore_ascii_case(expected))
    }

    fn validate_bug519_typed_contract(
        &self,
        source: &str,
        path: &Path,
    ) -> Result<XyceStaticTranPlan, String> {
        let requests = Self::print_output_requests(source, "TRAN")?;
        let [request] = requests.as_slice() else {
            return Err(format!(
                "{BUG519_LABEL} must retain exactly one .PRINT TRAN request"
            ));
        };
        if request
            != &(XycePrintOutputRequest {
                format: Some("raw".to_string()),
                file: Some("bug_519_SON.cir.raw".to_string()),
                probes: vec!["v(1)".to_string(), "i(r1)".to_string()],
            })
        {
            return Err(format!(
                "{BUG519_LABEL} exact RAW output request changed: {request:?}"
            ));
        }

        let netlist = Self::parse_xyce_netlist(source, path)
            .map_err(|error| format!("{BUG519_LABEL} no longer parses: {error}"))?;
        if netlist.title != "*test resistor lead current"
            || netlist.elements.len() != 2
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
            || !netlist.params.all_global_expressions().is_empty()
            || !netlist.params.all_string_params().is_empty()
            || !netlist.params.all_functions().is_empty()
        {
            return Err(format!("{BUG519_LABEL} typed envelope changed"));
        }

        let [voltage, resistor] = netlist.elements.as_slice() else {
            unreachable!("BUG519 typed element count was checked");
        };
        let expected_points = [(0.0f64, 0.0f64), (1.0f64, 1.0f64)];
        if !voltage.name.eq_ignore_ascii_case("V1")
            || !Self::bug519_nodes_match(&voltage.nodes, &["1", "0"])
            || voltage.provenance != ElementProvenance::Authored
            || !matches!(&voltage.kind, ElementKind::VoltageSource(
                rspice_core::netlist::SourceSpec::Pwl { points, delay, repeat_from }
            ) if points.len() == expected_points.len()
                && points.iter().zip(expected_points).all(|((time, value), (expected_time, expected_value))|
                    time.to_bits() == expected_time.to_bits()
                        && value.to_bits() == expected_value.to_bits())
                && delay.to_bits() == 0.0f64.to_bits()
                && repeat_from.is_none())
            || !resistor.name.eq_ignore_ascii_case("R1")
            || !Self::bug519_nodes_match(&resistor.nodes, &["1", "0"])
            || resistor.provenance != ElementProvenance::Authored
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
            return Err(format!(
                "{BUG519_LABEL} exact PWL/resistor topology changed"
            ));
        }

        if !matches!(&netlist.analyses[0], AnalysisCommand::Tran {
            step,
            stop,
            start: None,
            max_step: None,
            uic: false,
        } if step.to_bits() == 1.0e-3f64.to_bits()
            && stop.to_bits() == 1.0f64.to_bits())
        {
            return Err(format!("{BUG519_LABEL} typed TRAN command changed"));
        }
        let output = &netlist.output_requests[0];
        if output.directive != OutputDirectiveKind::Print
            || output.analysis != Some(rspice_core::netlist::OutputAnalysisKind::Tran)
            || output.name.is_some()
            || output.print_delimiter.as_ref() != Some(&PrintDelimiter::Whitespace)
            || !output.expressions.is_empty()
            || output.origin.line != 5
            || output
                .origin
                .path
                .as_deref()
                .is_none_or(|origin_path| !Self::same_path(origin_path, path))
            || output.dependencies.len() != 2
            || output
                .dependencies
                .iter()
                .zip([
                    (OutputSymbolKind::Node, "V", "1"),
                    (OutputSymbolKind::Device, "I", "R1"),
                ])
                .any(|(dependency, (kind, operator, symbol))| {
                    dependency.kind != kind
                        || dependency.expression
                        || !dependency.operator.eq_ignore_ascii_case(operator)
                        || !dependency.symbol.eq_ignore_ascii_case(symbol)
                })
        {
            return Err(format!(
                "{BUG519_LABEL} typed .PRINT request changed: {output:?}"
            ));
        }

        Ok(XyceStaticTranPlan {
            deck_path: path.to_path_buf(),
            oracle: XyceStaticTranOracle::None,
            source: source.to_string(),
            print: Some(XycePrintRequest {
                probes: request.probes.clone(),
            }),
            output_override: false,
            timeint_conststep: false,
            tran: Self::single_tran_analysis(&netlist)?,
            steps: Vec::new(),
            contract: XyceStaticTranContract::WrapperStatic,
            wrapper_tolerance: None,
            comparison_mode: XyceStaticTranComparisonMode::Pointwise,
        })
    }

    fn validate_bug519_parsed_raw(
        raw: &RawWaveformData,
        expected_binary: bool,
        table: &XycePrnTable,
        abort: &dyn AbortSignal,
    ) -> Result<(), String> {
        if raw.header.is_binary != expected_binary
            || raw.header.is_complex
            || raw.header.plotname != "Transient Analysis"
            || raw.header.no_variables != 3
            || raw.header.no_points != table.rows.len()
            || raw.variables.len() != 3
            || raw.waveforms.len() != 3
            || raw
                .variables
                .iter()
                .zip([("time", "time"), ("V(1)", "voltage"), ("I(R1)", "current")])
                .any(|(variable, (name, kind))| {
                    !variable.name.eq_ignore_ascii_case(name)
                        || !variable.var_type.eq_ignore_ascii_case(kind)
                })
        {
            return Err(format!(
                "{BUG519_LABEL} parsed RAW schema changed: {:?}",
                raw.header
            ));
        }
        for (row_index, row) in table.rows.iter().enumerate() {
            if row_index % 1_024 == 0 && abort.is_aborted() {
                return Err(format!(
                    "{BUG519_LABEL} deadline expired while validating RAW data"
                ));
            }
            for (waveform_index, table_index) in [(0usize, 1usize), (1, 2), (2, 3)] {
                let waveform = &raw.waveforms[waveform_index];
                let expected_value = if expected_binary {
                    row[table_index]
                } else {
                    Self::xyce_default_prn_roundtrip(row[table_index])?
                };
                let expected_time = if expected_binary {
                    row[1]
                } else {
                    Self::xyce_default_prn_roundtrip(row[1])?
                };
                if waveform.y_imag.is_some()
                    || waveform.y.get(row_index).map(|value| value.to_bits())
                        != Some(expected_value.to_bits())
                    || waveform.x.get(row_index).map(|value| value.to_bits())
                        != Some(expected_time.to_bits())
                {
                    return Err(format!(
                        "{BUG519_LABEL} RAW round-trip changed row {row_index}, waveform {}",
                        waveform.name
                    ));
                }
            }
            let voltage = Self::xyce_default_prn_roundtrip(raw.waveforms[1].y[row_index])?;
            let current = Self::xyce_default_prn_roundtrip(raw.waveforms[2].y[row_index])?;
            if voltage != current {
                return Err(format!(
                    "{BUG519_LABEL} converted RAW columns differ at row {row_index}: {voltage} != {current}"
                ));
            }
        }
        Ok(())
    }

    fn validate_bug519_raw_observation(
        table: &XycePrnTable,
        abort: &dyn AbortSignal,
    ) -> Result<(), String> {
        if table.columns.len() != 4
            || table.columns[0] != "Index"
            || !table.columns[1].eq_ignore_ascii_case("TIME")
            || !table.columns[2].eq_ignore_ascii_case("V(1)")
            || !table.columns[3].eq_ignore_ascii_case("I(R1)")
            || !(2..=100_100).contains(&table.rows.len())
        {
            return Err(format!(
                "{BUG519_LABEL} observation schema changed: columns={:?}, rows={}",
                table.columns,
                table.rows.len()
            ));
        }
        for (row_index, row) in table.rows.iter().enumerate() {
            if row_index % 1_024 == 0 && abort.is_aborted() {
                return Err(format!(
                    "{BUG519_LABEL} deadline expired while validating observation"
                ));
            }
            if row.len() != 4
                || row.iter().any(|value| !value.is_finite())
                || row[0] != row_index as Value
                || (row_index > 0 && row[1] <= table.rows[row_index - 1][1])
            {
                return Err(format!(
                    "{BUG519_LABEL} malformed observation row {row_index}: {row:?}"
                ));
            }
            let time = Self::xyce_default_prn_roundtrip(row[1])?;
            let voltage = Self::xyce_default_prn_roundtrip(row[2])?;
            let current = Self::xyce_default_prn_roundtrip(row[3])?;
            if voltage != current || voltage != time {
                return Err(format!(
                    "{BUG519_LABEL} PWL/one-ohm relation changed at row {row_index}: time={time}, V={voltage}, I={current}"
                ));
            }
        }
        let first = table.rows.first().expect("BUG519 row count was checked");
        let last = table.rows.last().expect("BUG519 row count was checked");
        if first[1].abs() > 1.0e-15
            || Self::xyce_default_prn_roundtrip(first[2])? != Self::xyce_default_prn_roundtrip(0.0)?
            || (last[1] - 1.0).abs() > 1.0e-12
            || Self::xyce_default_prn_roundtrip(last[2])? != Self::xyce_default_prn_roundtrip(1.0)?
        {
            return Err(format!(
                "{BUG519_LABEL} produced an invalid or trivial envelope: first={first:?}, last={last:?}"
            ));
        }

        let times = table.rows.iter().map(|row| row[1]).collect::<Vec<_>>();
        let voltage = table.rows.iter().map(|row| row[2]).collect::<Vec<_>>();
        let current = table.rows.iter().map(|row| row[3]).collect::<Vec<_>>();
        let mut exporter = RawExporter::new_transient("BUG_519_SON transient");
        exporter.add_voltage("1");
        exporter.add_current("R1");
        exporter
            .add_transient_data(&times, &[voltage, current])
            .map_err(|error| format!("{BUG519_LABEL} RAW assembly failed: {error}"))?;

        let mut binary = Vec::new();
        exporter
            .write(&mut binary, RawFormat::Binary)
            .map_err(|error| format!("{BUG519_LABEL} binary RAW export failed: {error}"))?;
        let binary = parse_raw_reader(&mut Cursor::new(binary))
            .map_err(|error| format!("{BUG519_LABEL} binary RAW parse failed: {error}"))?;
        Self::validate_bug519_parsed_raw(&binary, true, table, abort)?;

        if abort.is_aborted() {
            return Err(format!(
                "{BUG519_LABEL} deadline expired between RAW formats"
            ));
        }
        let mut ascii = Vec::new();
        exporter
            .write_xyce_ascii(&mut ascii)
            .map_err(|error| format!("{BUG519_LABEL} ASCII RAW export failed: {error}"))?;
        let ascii = parse_raw_reader(&mut Cursor::new(ascii))
            .map_err(|error| format!("{BUG519_LABEL} ASCII RAW parse failed: {error}"))?;
        Self::validate_bug519_parsed_raw(&ascii, false, table, abort)?;

        for waveform_index in 0..3 {
            if binary.waveforms[waveform_index]
                .y
                .iter()
                .zip(&ascii.waveforms[waveform_index].y)
                .any(
                    |(binary, ascii)| match Self::xyce_default_prn_roundtrip(*binary) {
                        Ok(binary) => binary.to_bits() != ascii.to_bits(),
                        Err(_) => true,
                    },
                )
            {
                return Err(format!(
                    "{BUG519_LABEL} binary and ASCII RAW observations differ for {}",
                    binary.waveforms[waveform_index].name
                ));
            }
        }
        Ok(())
    }

    pub(super) fn validate_bug519_oracle(
        &self,
        deck: &XyceDeck,
        start: Instant,
    ) -> Result<(), String> {
        let abort = DeadlineAbort::new(start, self.config.max_time_per_test_ms.max(1));
        if abort.is_aborted() {
            return Err(format!("{BUG519_LABEL} deadline expired before validation"));
        }
        let bytes = self.validate_bug519_provenance(deck)?;
        let source = std::str::from_utf8(&bytes)
            .map_err(|error| format!("{BUG519_LABEL} is not UTF-8: {error}"))?;
        let plan = self.validate_bug519_typed_contract(source, &deck.path)?;
        let (netlist, result) = self
            .run_transient_family_plan(&plan, start, None, None)
            .map_err(|error| format!("{BUG519_LABEL} execution failed: {error}"))?;
        let table = Self::transient_family_result_to_prn_table(&plan, &netlist, &result)
            .map_err(|error| format!("{BUG519_LABEL} output observation failed: {error}"))?;
        Self::validate_bug519_raw_observation(&table, &abort)?;
        if abort.is_aborted() {
            return Err(format!("{BUG519_LABEL} execution exceeded timeout"));
        }
        self.validate_bug519_provenance(deck)?;
        if abort.is_aborted() {
            return Err(format!("{BUG519_LABEL} final provenance exceeded timeout"));
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
            path: root.join(XYCE_BUG519_PATH),
            section: XyceDeckSection::Netlists,
            relative_path: XYCE_BUG519_PATH.to_string(),
        }
    }

    fn bug519_fixture(label: &str) -> (tempfile::TempDir, XyceDeck, XyceTestRunner) {
        let source_root = corpus_root();
        let temporary = tempfile::Builder::new()
            .prefix(&format!("rspice-xyce-bug519-{label}-"))
            .tempdir()
            .expect("create BUG519 fixture root");
        let root = temporary.path();
        let family = root.join("Netlists/Certification_Tests/BUG_519_SON");
        fs::create_dir_all(&family).expect("create BUG519 fixture family");
        fs::copy(
            source_root.join(XYCE_BUG519_PATH),
            family.join("bug_519_SON.cir"),
        )
        .expect("copy canonical BUG519 source");
        fs::write(
            root.join(HARNESS_MANIFEST_FILE),
            format!("{XYCE_BUG519_PATH}\t{REQUIRES_UPSTREAM_WRAPPER_CONTRACT}\n"),
        )
        .expect("write BUG519 wrapper manifest");
        fs::write(
            root.join(UPSTREAM_EXCLUSIONS_MANIFEST_FILE),
            format!(
                "schema_version\t{UPSTREAM_EXCLUSIONS_SCHEMA_VERSION}\nsource_commit\t{UPSTREAM_EXCLUSIONS_SOURCE_COMMIT}\nsource_netlists_tree\t{UPSTREAM_EXCLUSIONS_SOURCE_NETLISTS_TREE}\n"
            ),
        )
        .expect("write empty BUG519 exclusion manifest");
        let deck = canonical_deck(root);
        let runner = XyceTestRunner::new(root, XyceRunnerConfig::default());
        (temporary, deck, runner)
    }

    #[test]
    fn bug519_historical_provenance_is_exact() {
        XyceTestRunner::validate_bug519_historical_oracle_provenance()
            .expect("Release-7.10 BUG519 provenance remains exact");
    }

    #[test]
    fn bug519_typed_contract_preserves_raw_request_and_one_ohm_relation() {
        let root = corpus_root();
        let path = root.join(XYCE_BUG519_PATH);
        let source = fs::read_to_string(&path).expect("read canonical BUG519 source");
        let runner = XyceTestRunner::new(&root, XyceRunnerConfig::default());
        runner
            .validate_bug519_typed_contract(&source, &path)
            .expect("canonical BUG519 typed contract passes");
        for (index, mutation) in [
            source.replacen("V1 1 0 pwl 0 0 1 1", "V1 1 0 pwl 0 0 1 0.9", 1),
            source.replacen("r1 1 0 1", "r1 1 0 2", 1),
            source.replacen(".tran 1m 1", ".tran 2m 1", 1),
            source.replacen("format=raw", "format=prn", 1),
            source.replacen("file=bug_519_SON.cir.raw", "file=other.raw", 1),
            source.replacen("v(1) i(r1)", "i(r1) v(1)", 1),
        ]
        .into_iter()
        .enumerate()
        {
            let (_temporary, deck, fixture_runner) =
                bug519_fixture(&format!("typed-mutation-{index}"));
            fs::write(&deck.path, &mutation).expect("write BUG519 semantic mutation");
            assert!(
                fixture_runner
                    .validate_bug519_typed_contract(&mutation, &deck.path)
                    .is_err(),
                "BUG519 semantic mutation {index} must fail closed"
            );
        }
    }

    #[test]
    fn bug519_raw_predicate_covers_both_formats_and_rejects_counterfactuals() {
        let abort = DeadlineAbort::new(Instant::now(), 10_000);
        let columns = vec![
            "Index".to_string(),
            "TIME".to_string(),
            "V(1)".to_string(),
            "I(R1)".to_string(),
        ];
        let valid = XycePrnTable {
            columns: columns.clone(),
            rows: vec![
                vec![0.0, 0.0, 0.0, -0.0],
                vec![1.0, 0.5, 0.5, 0.5],
                vec![2.0, 1.0, 1.0, 1.0],
            ],
        };
        XyceTestRunner::validate_bug519_raw_observation(&valid, &abort)
            .expect("canonical binary/ASCII RAW relation passes");

        let unequal = XycePrnTable {
            columns: columns.clone(),
            rows: vec![vec![0.0, 0.0, 0.0, 0.0], vec![1.0, 1.0, 1.0, 0.9]],
        };
        assert!(XyceTestRunner::validate_bug519_raw_observation(&unequal, &abort).is_err());

        let wrong_ramp = XycePrnTable {
            columns,
            rows: vec![vec![0.0, 0.0, 0.0, 0.0], vec![1.0, 1.0, 0.5, 0.5]],
        };
        assert!(XyceTestRunner::validate_bug519_raw_observation(&wrong_ramp, &abort).is_err());
    }

    #[test]
    fn bug519_oracle_executes_binary_and_ascii_raw_relation() {
        let root = corpus_root();
        let runner = XyceTestRunner::new(&root, XyceRunnerConfig::default());
        runner
            .validate_bug519_oracle(&canonical_deck(&root), Instant::now())
            .expect("canonical BUG519 RAW wrapper passes");
    }

    #[test]
    fn bug519_oracle_rejects_an_expired_deadline() {
        let root = corpus_root();
        let runner = XyceTestRunner::new(
            &root,
            XyceRunnerConfig {
                max_time_per_test_ms: 1,
                ..XyceRunnerConfig::default()
            },
        );
        let start = Instant::now()
            .checked_sub(Duration::from_millis(2))
            .expect("construct expired BUG519 deadline");
        assert!(
            runner
                .validate_bug519_oracle(&canonical_deck(&root), start)
                .is_err()
        );
    }

    #[test]
    fn bug519_provenance_rejects_source_output_manifest_and_exclusion_drift() {
        let (_temporary, deck, runner) = bug519_fixture("source-drift");
        let source = fs::read_to_string(&deck.path).expect("read BUG519 fixture source");
        fs::write(&deck.path, source.replace("r1 1 0 1", "r1 1 0 2"))
            .expect("mutate BUG519 retained source");
        assert!(runner.validate_bug519_provenance(&deck).is_err());

        let (_temporary, deck, runner) = bug519_fixture("family-drift");
        fs::write(
            deck.path
                .parent()
                .expect("BUG519 deck has parent")
                .join("unexpected.raw"),
            "stale wrapper output\n",
        )
        .expect("write stale BUG519 output");
        assert!(runner.validate_bug519_provenance(&deck).is_err());

        let (_temporary, deck, runner) = bug519_fixture("output-drift");
        let output = runner
            .static_output_reference_path(&deck.path, "prn")
            .expect("map BUG519 fixture to OutputData");
        fs::create_dir_all(output.parent().expect("BUG519 output has parent"))
            .expect("create forbidden BUG519 OutputData family");
        fs::write(output, "invented gold\n").expect("write forbidden BUG519 gold");
        assert!(runner.validate_bug519_provenance(&deck).is_err());

        let (_temporary, deck, runner) = bug519_fixture("manifest-drift");
        fs::write(runner.root.join(HARNESS_MANIFEST_FILE), "")
            .expect("remove BUG519 wrapper ownership");
        let rebuilt = XyceTestRunner::new(&runner.root, XyceRunnerConfig::default());
        assert!(rebuilt.validate_bug519_provenance(&deck).is_err());

        let (_temporary, deck, runner) = bug519_fixture("exclusion-drift");
        fs::write(
            runner.root.join(UPSTREAM_EXCLUSIONS_MANIFEST_FILE),
            format!(
                "schema_version\t{UPSTREAM_EXCLUSIONS_SCHEMA_VERSION}\nsource_commit\t{UPSTREAM_EXCLUSIONS_SOURCE_COMMIT}\nsource_netlists_tree\t{UPSTREAM_EXCLUSIONS_SOURCE_NETLISTS_TREE}\n{XYCE_BUG519_PATH}\tNetlists/Certification_Tests/BUG_519_SON/exclude\t{UPSTREAM_EXCLUDED_DISPOSITION}\n"
            ),
        )
        .expect("fabricate BUG519 exclusion");
        assert!(runner.validate_bug519_provenance(&deck).is_err());
    }
}
