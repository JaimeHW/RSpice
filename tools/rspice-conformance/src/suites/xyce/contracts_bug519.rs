use super::*;
use rspice_core::io::{RawExporter, RawFormat, RawWaveformData, parse_raw_reader};
use std::io::Cursor;

const BUG519_LABEL: &str = "BUG_519_SON binary/ASCII RAW wrapper";

impl XyceTestRunner {
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
        let source = fs::read_to_string(&deck.path)
            .map_err(|error| format!("failed to read {}: {error}", deck.path.display()))?;
        let plan = self.validate_bug519_typed_contract(&source, &deck.path)?;
        let (netlist, result) = self
            .run_transient_family_plan(&plan, start, None, None)
            .map_err(|error| format!("{BUG519_LABEL} execution failed: {error}"))?;
        let table = Self::transient_family_result_to_prn_table(&plan, &netlist, &result)
            .map_err(|error| format!("{BUG519_LABEL} output observation failed: {error}"))?;
        Self::validate_bug519_raw_observation(&table, &abort)?;
        if abort.is_aborted() {
            return Err(format!("{BUG519_LABEL} execution exceeded timeout"));
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

    #[test]
    fn bug519_typed_contract_preserves_raw_request_and_one_ohm_relation() {
        let root = corpus_root();
        let path = root.join(XYCE_BUG519_PATH);
        let source = fs::read_to_string(&path).expect("read canonical BUG519 source");
        let runner = XyceTestRunner::new(&root, XyceRunnerConfig::default());
        runner
            .validate_bug519_typed_contract(&source, &path)
            .expect("canonical BUG519 typed contract passes");
        let temporary = tempfile::Builder::new()
            .prefix("rspice-xyce-bug519-typed-mutation-")
            .tempdir()
            .expect("create BUG519 mutation root");
        let mutated_path = temporary.path().join(XYCE_BUG519_PATH);
        fs::create_dir_all(mutated_path.parent().expect("BUG519 deck has a family"))
            .expect("create BUG519 mutation family");
        fs::write(&mutated_path, &source).expect("copy canonical BUG519 source");
        fs::write(
            temporary.path().join(HARNESS_MANIFEST_FILE),
            format!("{XYCE_BUG519_PATH}\t{REQUIRES_UPSTREAM_WRAPPER_CONTRACT}\n"),
        )
        .expect("write BUG519 wrapper manifest");
        let fixture_runner = XyceTestRunner::new(temporary.path(), XyceRunnerConfig::default());
        fixture_runner
            .validate_bug519_typed_contract(&source, &mutated_path)
            .expect("canonical BUG519 typed contract passes outside the corpus");
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
            fs::write(&mutated_path, &mutation).expect("write BUG519 semantic mutation");
            assert!(
                fixture_runner
                    .validate_bug519_typed_contract(&mutation, &mutated_path)
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
}
