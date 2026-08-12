use super::*;

const BUG302_LABEL: &str = "BUG_302 print-delimiter wrapper";
const BUG302_INVALID_WARNING: &str = "Invalid value of DELIMITER in .PRINT statment, ignoring";

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Bug302Analysis {
    Dc,
    Tran,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Bug302Mode {
    Default,
    Comma,
    Tab,
    Invalid,
}

#[derive(Debug, Clone, Copy)]
struct Bug302Worker {
    file_name: &'static str,
    analysis: Bug302Analysis,
    mode: Bug302Mode,
}

impl Bug302Worker {
    const ALL: [Self; 8] = [
        Self {
            file_name: "DC_defaults.cir",
            analysis: Bug302Analysis::Dc,
            mode: Bug302Mode::Default,
        },
        Self {
            file_name: "DC_comma.cir",
            analysis: Bug302Analysis::Dc,
            mode: Bug302Mode::Comma,
        },
        Self {
            file_name: "DC_tab.cir",
            analysis: Bug302Analysis::Dc,
            mode: Bug302Mode::Tab,
        },
        Self {
            file_name: "DC_delimiter_invalid.cir",
            analysis: Bug302Analysis::Dc,
            mode: Bug302Mode::Invalid,
        },
        Self {
            file_name: "transient_defaults.cir",
            analysis: Bug302Analysis::Tran,
            mode: Bug302Mode::Default,
        },
        Self {
            file_name: "transient_comma.cir",
            analysis: Bug302Analysis::Tran,
            mode: Bug302Mode::Comma,
        },
        Self {
            file_name: "transient_tab.cir",
            analysis: Bug302Analysis::Tran,
            mode: Bug302Mode::Tab,
        },
        Self {
            file_name: "transient_delimiter_invalid.cir",
            analysis: Bug302Analysis::Tran,
            mode: Bug302Mode::Invalid,
        },
    ];

    fn delimiter(self) -> PrintDelimiter {
        match self.mode {
            Bug302Mode::Default | Bug302Mode::Invalid => PrintDelimiter::Whitespace,
            Bug302Mode::Comma => PrintDelimiter::Comma,
            Bug302Mode::Tab => PrintDelimiter::Tab,
        }
    }

    fn expected_contract(self) -> &'static str {
        match self.analysis {
            Bug302Analysis::Dc => "static_prn_dc",
            Bug302Analysis::Tran => "static_prn_tran",
        }
    }

    fn normalized_record(self) -> String {
        format!(
            "netlists/certification_tests/bug_302/{}",
            self.file_name.to_ascii_lowercase()
        )
    }
}

struct Bug302RetainedArtifacts {
    source_dir: PathBuf,
    output_dir: PathBuf,
    sources: BTreeMap<String, Vec<u8>>,
}

impl XyceTestRunner {
    pub(super) fn bug302_historical_oracle_provenance_records() -> Vec<String> {
        let mut records = XYCE_BUG302_HISTORICAL_ARTIFACTS
            .into_iter()
            .map(|(path, bytes, sha256, content_blake3)| {
                format!(
                    "{XYCE_BUG302_UPSTREAM_REGRESSION_COMMIT}\t{XYCE_BUG302_UPSTREAM_RELEASE_TAG}\t{path}\t{bytes}\t{sha256}\t{content_blake3}"
                )
            })
            .collect::<Vec<_>>();
        records.sort();
        records
    }

    pub(super) fn validate_bug302_historical_oracle_provenance() -> Result<(), String> {
        let records = Self::bug302_historical_oracle_provenance_records();
        let stream = records.join("\n");
        let sha256 = format!("{:x}", Sha256::digest(stream.as_bytes()));
        let content_blake3 = blake3::hash(stream.as_bytes()).to_hex().to_string();
        if records.len() != XYCE_BUG302_HISTORICAL_RECORD_COUNT
            || stream.len() != XYCE_BUG302_HISTORICAL_RECORD_BYTES
            || sha256 != XYCE_BUG302_HISTORICAL_RECORDS_SHA256
            || content_blake3 != XYCE_BUG302_HISTORICAL_RECORDS_BLAKE3
        {
            return Err(format!(
                "{BUG302_LABEL} Release-7.10 provenance changed: records={}/{}, sha256={sha256}, blake3={content_blake3}",
                records.len(),
                stream.len()
            ));
        }
        Ok(())
    }

    fn validate_bug302_directory<const N: usize>(
        directory: &Path,
        expected: &[(&str, usize, &str, &str); N],
        label: &str,
    ) -> Result<BTreeMap<String, Vec<u8>>, String> {
        let metadata = fs::symlink_metadata(directory)
            .map_err(|error| format!("failed to inspect {label}: {error}"))?;
        if metadata.file_type().is_symlink() || !metadata.file_type().is_dir() {
            return Err(format!(
                "{label} {} must be a regular non-symlink directory",
                directory.display()
            ));
        }
        let expected = expected
            .iter()
            .copied()
            .map(|artifact| (artifact.0.to_ascii_lowercase(), artifact))
            .collect::<BTreeMap<_, _>>();
        let mut observed = BTreeMap::new();
        for entry in
            fs::read_dir(directory).map_err(|error| format!("failed to read {label}: {error}"))?
        {
            let entry = entry.map_err(|error| format!("failed to inspect {label}: {error}"))?;
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
                return Err(format!("{label} contains case-colliding member {name:?}"));
            }
            let Some((expected_name, expected_bytes, expected_sha256, expected_blake3)) =
                expected.get(&key).copied()
            else {
                return Err(format!("{label} acquired unexpected member {name:?}"));
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
                "{label} census changed: expected {} members, got {}",
                expected.len(),
                observed.len()
            ));
        }
        Ok(observed)
    }

    fn validate_bug302_provenance(
        &self,
        deck: &XyceDeck,
    ) -> Result<Bug302RetainedArtifacts, String> {
        Self::validate_bug302_historical_oracle_provenance()?;
        if deck.section != XyceDeckSection::Netlists
            || Self::normalize_manifest_key(&deck.relative_path) != XYCE_BUG302_RECORD
            || Self::normalize_manifest_key(&self.relative_key(&deck.path)) != XYCE_BUG302_RECORD
            || !Self::same_path(&deck.path, &self.root.join(XYCE_BUG302_PATH))
        {
            return Err(format!(
                "recognized {BUG302_LABEL} record '{}' is not backed by its canonical path",
                deck.relative_path
            ));
        }
        let family_prefix = "netlists/certification_tests/bug_302/";
        let owners = self
            .upstream_wrapper_decks
            .iter()
            .filter(|record| record.starts_with(family_prefix))
            .map(String::as_str)
            .collect::<Vec<_>>();
        if owners != [XYCE_BUG302_RECORD] {
            return Err(format!(
                "{BUG302_LABEL} requires its exact sole wrapper owner, found {owners:?}"
            ));
        }

        let exclusions = Self::load_upstream_exclusions(&self.root)
            .map_err(|error| format!("{BUG302_LABEL} exclusion manifest is invalid: {error}"))?;
        if exclusions.contains_key(XYCE_BUG302_RECORD) {
            return Err(format!("{BUG302_LABEL} owner must not be excluded"));
        }
        let family_exclusions = exclusions
            .iter()
            .filter(|(record, _)| record.starts_with(family_prefix))
            .collect::<BTreeMap<_, _>>();
        let expected_records = Bug302Worker::ALL
            .into_iter()
            .map(Bug302Worker::normalized_record)
            .collect::<BTreeSet<_>>();
        if family_exclusions
            .keys()
            .map(|record| record.to_string())
            .collect::<BTreeSet<_>>()
            != expected_records
        {
            return Err(format!(
                "{BUG302_LABEL} exclusion-family census changed: {:?}",
                family_exclusions.keys().collect::<Vec<_>>()
            ));
        }
        for worker in Bug302Worker::ALL {
            let record = worker.normalized_record();
            let exclusion = family_exclusions
                .get(&record)
                .ok_or_else(|| format!("{BUG302_LABEL} lost exclusion for {record}"))?;
            if exclusion.source != XYCE_BUG302_EXCLUSION_SOURCE
                || !matches!(
                    &exclusion.disposition,
                    XyceUpstreamExclusionDisposition::RspiceIndependentlyQualified {
                        expected_contract
                    } if expected_contract == worker.expected_contract()
                )
            {
                return Err(format!(
                    "{BUG302_LABEL} qualification changed for {record}: {exclusion:?}"
                ));
            }
        }

        let source_dir = self.root.join("Netlists/Certification_Tests/BUG_302");
        let sources = Self::validate_bug302_directory(
            &source_dir,
            &XYCE_BUG302_RETAINED_SOURCES,
            "BUG_302 retained source family",
        )?;
        if sources
            .get("bug_302.cir")
            .is_none_or(|source| !source.is_empty())
        {
            return Err(format!("{BUG302_LABEL} owner is no longer exactly empty"));
        }
        let output_dir = self.root.join("OutputData/Certification_Tests/BUG_302");
        Self::validate_bug302_directory(
            &output_dir,
            &XYCE_BUG302_RETAINED_OUTPUTS,
            "BUG_302 retained output family",
        )?;
        self.reject_wrapper_output_artifacts(&deck.path)
            .map_err(|error| format!("{BUG302_LABEL} {error}"))?;
        if output_dir.join("bug_302.cir.prn").exists() {
            return Err(format!(
                "{BUG302_LABEL} must not acquire an invented owner gold"
            ));
        }
        Ok(Bug302RetainedArtifacts {
            source_dir,
            output_dir,
            sources,
        })
    }

    fn bug302_semantic_fingerprint(netlist: &Netlist) -> String {
        [
            format!("{:?}", netlist.title),
            format!("{:?}", netlist.elements),
            format!("{:?}", netlist.models),
            format!("{:?}", netlist.subcircuits),
            format!("{:?}", netlist.analyses),
            format!("{:?}", netlist.lin_analysis),
            format!("{:?}", netlist.fft_analyses),
            format!("{:?}", netlist.data_tables),
            format!("{:?}", netlist.params),
            format!("{:?}", netlist.initial_conditions),
            format!("{:?}", netlist.device_initial_conditions),
            format!("{:?}", netlist.node_sets),
            format!("{:?}", netlist.global_nodes),
            format!("{:?}", netlist.options),
            format!("{:?}", netlist.saves),
            format!("{:?}", netlist.measurements),
            format!("{:?}", netlist.veriloga_includes),
            format!("{:?}", netlist.spef_includes),
            netlist.pspice_chebyshev_source_count().to_string(),
        ]
        .join("\n")
    }

    fn validate_bug302_worker(
        worker: Bug302Worker,
        path: &Path,
        bytes: &[u8],
    ) -> Result<(String, Netlist), String> {
        let source = String::from_utf8(bytes.to_vec()).map_err(|error| {
            format!("{BUG302_LABEL} {} is not UTF-8: {error}", worker.file_name)
        })?;
        let netlist =
            Self::parse_netlist_with_expression_dialect(&source, path, ExpressionDialect::Xyce)
                .map_err(|error| {
                    format!(
                        "{BUG302_LABEL} {} no longer parses: {error}",
                        worker.file_name
                    )
                })?;
        let [request] = netlist.output_requests.as_slice() else {
            return Err(format!(
                "{BUG302_LABEL} {} must retain exactly one output request",
                worker.file_name
            ));
        };
        let expected_analysis = match worker.analysis {
            Bug302Analysis::Dc => rspice_core::netlist::OutputAnalysisKind::Dc,
            Bug302Analysis::Tran => rspice_core::netlist::OutputAnalysisKind::Tran,
        };
        let expected_line = match (worker.analysis, worker.mode) {
            (Bug302Analysis::Dc, Bug302Mode::Default) => 9,
            (Bug302Analysis::Dc, _) => 10,
            (Bug302Analysis::Tran, Bug302Mode::Default) => 8,
            (Bug302Analysis::Tran, _) => 9,
        };
        let expected_dependencies: &[(&str, &str, rspice_core::netlist::OutputSymbolKind)] =
            match worker.analysis {
                Bug302Analysis::Dc => &[
                    ("V", "2", rspice_core::netlist::OutputSymbolKind::Node),
                    ("V", "1", rspice_core::netlist::OutputSymbolKind::Node),
                    ("I", "VDS", rspice_core::netlist::OutputSymbolKind::Device),
                ],
                Bug302Analysis::Tran => &[("V", "1", rspice_core::netlist::OutputSymbolKind::Node)],
            };
        if request.directive != OutputDirectiveKind::Print
            || request.analysis != Some(expected_analysis)
            || request.name.is_some()
            || !request.expressions.is_empty()
            || request.print_delimiter.as_ref() != Some(&worker.delimiter())
            || request.origin.line != expected_line
            || request
                .origin
                .path
                .as_deref()
                .is_none_or(|origin_path| !XyceTestRunner::same_path(origin_path, path))
            || request.dependencies.len() != expected_dependencies.len()
            || request.dependencies.iter().zip(expected_dependencies).any(
                |(actual, (operator, symbol, kind))| {
                    actual.expression
                        || actual.kind != *kind
                        || !actual.operator.eq_ignore_ascii_case(operator)
                        || !actual.symbol.eq_ignore_ascii_case(symbol)
                },
            )
        {
            return Err(format!(
                "{BUG302_LABEL} {} typed output request changed: {request:?}",
                worker.file_name
            ));
        }
        match worker.mode {
            Bug302Mode::Invalid => {
                let [diagnostic] = netlist.diagnostics.as_slice() else {
                    return Err(format!(
                        "{BUG302_LABEL} {} lost its one invalid-delimiter warning",
                        worker.file_name
                    ));
                };
                if diagnostic.code != "xyce-invalid-print-delimiter"
                    || diagnostic.message != BUG302_INVALID_WARNING
                    || diagnostic.line != expected_line
                    || diagnostic.origin.as_ref().is_none_or(|origin| {
                        origin.line != expected_line
                            || origin.path.as_deref().is_none_or(|origin_path| {
                                !XyceTestRunner::same_path(origin_path, path)
                            })
                    })
                {
                    return Err(format!(
                        "{BUG302_LABEL} {} warning changed: {diagnostic:?}",
                        worker.file_name
                    ));
                }
            }
            _ if !netlist.diagnostics.is_empty() => {
                return Err(format!(
                    "{BUG302_LABEL} {} gained diagnostics: {:?}",
                    worker.file_name, netlist.diagnostics
                ));
            }
            _ => {}
        }

        match worker.analysis {
            Bug302Analysis::Dc => {
                let dc = Self::single_dc_sweep(&netlist)?;
                let print = Self::single_dc_print_request(&source)?;
                if netlist.elements.len() != 3
                    || netlist.models.len() != 1
                    || netlist.analyses.len() != 1
                    || !dc.source.eq_ignore_ascii_case("VDS")
                    || dc.mode != DcSweepMode::Linear
                    || dc.start.to_bits() != 0.0f64.to_bits()
                    || dc.stop.to_bits() != 3.5f64.to_bits()
                    || dc.step.to_bits() != 0.05f64.to_bits()
                    || dc.sweep2.as_ref().is_none_or(|sweep| {
                        !sweep.source.eq_ignore_ascii_case("VGS")
                            || sweep.start.to_bits() != 0.0f64.to_bits()
                            || sweep.stop.to_bits() != 3.5f64.to_bits()
                            || sweep.step.to_bits() != 0.5f64.to_bits()
                    })
                    || print.probes != ["v(2)", "v(1)", "i(vds)"]
                    || !netlist.models[0].model_type.eq_ignore_ascii_case("NMOS")
                    || netlist.models[0]
                        .params
                        .iter()
                        .find(|(name, _)| name.eq_ignore_ascii_case("LEVEL"))
                        .is_none_or(|(_, value)| value.to_bits() != 9.0f64.to_bits())
                {
                    return Err(format!(
                        "{BUG302_LABEL} {} DC/model envelope changed",
                        worker.file_name
                    ));
                }
            }
            Bug302Analysis::Tran => {
                let request = Self::single_tran_print_output_request(&source)?;
                let tran = Self::single_tran_analysis(&netlist)?;
                if netlist.elements.len() != 3
                    || !netlist.models.is_empty()
                    || netlist.analyses.len() != 1
                    || tran.step.to_bits() != 0.0f64.to_bits()
                    || tran.stop.to_bits() != 0.005f64.to_bits()
                    || tran.start.is_some()
                    || tran.max_step.is_some()
                    || tran.uic
                    || request.probes != ["v(1)"]
                {
                    return Err(format!(
                        "{BUG302_LABEL} {} transient envelope changed",
                        worker.file_name
                    ));
                }
            }
        }
        Ok((Self::bug302_semantic_fingerprint(&netlist), netlist))
    }

    fn historical_bug302_delimiter_transform(default: &str, separator: &str) -> String {
        let mut output = String::new();
        for line in default.lines() {
            let line = line.trim_end_matches(char::is_whitespace);
            if line.contains("Xyce") {
                output.push_str(line);
            } else {
                output.push_str(&line.split_whitespace().collect::<Vec<_>>().join(separator));
            }
            output.push('\n');
        }
        output
    }

    fn canonical_bug302_output(path: &Path) -> Result<String, String> {
        let bytes = fs::read(path).map_err(|error| {
            format!("failed to read BUG_302 output {}: {error}", path.display())
        })?;
        String::from_utf8(Self::canonical_lf_text_identity(BUG302_LABEL, &bytes)?)
            .map_err(|error| format!("BUG_302 output {} is not UTF-8: {error}", path.display()))
    }

    fn validate_bug302_retained_output_relations(output_dir: &Path) -> Result<(), String> {
        for prefix in ["DC", "transient"] {
            let default = Self::canonical_bug302_output(
                &output_dir.join(format!("{prefix}_defaults.cir.prn")),
            )?;
            let comma =
                Self::canonical_bug302_output(&output_dir.join(format!("{prefix}_comma.cir.prn")))?;
            let tab =
                Self::canonical_bug302_output(&output_dir.join(format!("{prefix}_tab.cir.prn")))?;
            let invalid = Self::canonical_bug302_output(
                &output_dir.join(format!("{prefix}_delimiter_invalid.cir.prn")),
            )?;
            if Self::historical_bug302_delimiter_transform(&default, ",") != comma
                || Self::historical_bug302_delimiter_transform(&default, "\t") != tab
                || default != invalid
            {
                return Err(format!(
                    "{BUG302_LABEL} retained {prefix} delimiter relation changed"
                ));
            }
        }
        Ok(())
    }

    fn validate_bug302_fresh_rendering(table: &XycePrnTable) -> Result<(), String> {
        let default = Self::xyce_prn_text_with_delimiter(table, &PrintDelimiter::Whitespace)?;
        let comma = Self::xyce_prn_text_with_delimiter(table, &PrintDelimiter::Comma)?;
        let tab = Self::xyce_prn_text_with_delimiter(table, &PrintDelimiter::Tab)?;
        let invalid_fallback =
            Self::xyce_prn_text_with_delimiter(table, &PrintDelimiter::Whitespace)?;
        if Self::historical_bug302_delimiter_transform(&default, ",") != comma
            || Self::historical_bug302_delimiter_transform(&default, "\t") != tab
            || default != invalid_fallback
        {
            return Err(format!(
                "{BUG302_LABEL} fresh delimiter serializer violates its exact relation"
            ));
        }
        Ok(())
    }

    fn validate_bug302_dc_execution(
        &self,
        default_path: &Path,
        reference_path: &Path,
        start: Instant,
    ) -> Result<(), String> {
        let plan = self.static_dc_plan_for_path(default_path, ExpressionDialect::Xyce)?;
        let (netlist, results) = self
            .run_static_dc_results(&plan, start)
            .map_err(|error| format!("{BUG302_LABEL} DC execution failed: {error}"))?;
        if results.len() != 568
            || results.iter().any(|point| {
                !point.sweep_value.is_finite()
                    || point
                        .result
                        .node_voltages
                        .iter()
                        .chain(point.result.branch_currents.iter())
                        .any(|value| !value.is_finite())
                    || !point.device_op_report.labels_resolve()
            })
        {
            return Err(format!(
                "{BUG302_LABEL} DC execution lost its finite 568-point grid"
            ));
        }
        let table = self
            .dc_results_to_prn_table(&plan, &netlist, &results)
            .map_err(|error| format!("{BUG302_LABEL} DC PRN materialization failed: {error}"))?;
        if table
            .columns
            .iter()
            .map(|column| column.to_ascii_lowercase())
            .ne(["index", "v(2)", "v(1)", "i(vds)"]
                .into_iter()
                .map(str::to_string))
            || table.rows.len() != 568
        {
            return Err(format!(
                "{BUG302_LABEL} DC PRN schema changed: {:?}/{}",
                table.columns,
                table.rows.len()
            ));
        }
        let reference = Self::parse_prn_file(reference_path)
            .map_err(|error| format!("{BUG302_LABEL} DC default reference is invalid: {error}"))?;
        let mismatches = self
            .compare_dc_prn_reference(
                &reference,
                &plan.print,
                &netlist,
                &plan.source,
                &plan.dc,
                &results,
            )
            .map_err(|error| {
                format!("{BUG302_LABEL} DC default reference comparison failed: {error}")
            })?;
        if !mismatches.is_empty() {
            return Err(format!(
                "{BUG302_LABEL} DC default no longer matches its retained numerical oracle: {mismatches:?}"
            ));
        }
        Self::validate_bug302_fresh_rendering(&table)
    }

    fn validate_bug302_tran_execution(
        &self,
        default_path: &Path,
        reference_path: &Path,
        start: Instant,
    ) -> Result<(), String> {
        let plan = self.static_tran_plan_for_path_with_purpose(
            default_path,
            XyceStaticTranPlanPurpose::RelationalFamily,
        )?;
        let (netlist, result) = self
            .run_transient_family_plan(&plan, start, None, None)
            .map_err(|error| format!("{BUG302_LABEL} transient execution failed: {error}"))?;
        let table = Self::transient_family_result_to_prn_table(&plan, &netlist, &result).map_err(
            |error| format!("{BUG302_LABEL} transient PRN materialization failed: {error}"),
        )?;
        if table
            .columns
            .iter()
            .map(|column| column.to_ascii_lowercase())
            .ne(["index", "time", "v(1)"].into_iter().map(str::to_string))
            || table.rows.is_empty()
            || table.rows.iter().enumerate().any(|(index, row)| {
                row.len() != 3
                    || row[0].to_bits() != (index as Value).to_bits()
                    || row.iter().any(|value| !value.is_finite())
                    || row[1] < 0.0
                    || row[1] > 0.0050000001
                    || (row[2] - (-row[1] / 0.001).exp()).abs() > 5.0e-3
            })
        {
            return Err(format!(
                "{BUG302_LABEL} transient RC response/schema changed: {:?}/{}",
                table.columns,
                table.rows.len()
            ));
        }
        let reference = Self::parse_prn_file(reference_path).map_err(|error| {
            format!("{BUG302_LABEL} transient default reference is invalid: {error}")
        })?;
        let mismatches = self
            .compare_static_tran_primary_reference(&reference, &plan, &netlist, &result)
            .map_err(|error| {
                format!("{BUG302_LABEL} transient default reference comparison failed: {error}")
            })?;
        if !mismatches.is_empty() {
            return Err(format!(
                "{BUG302_LABEL} transient default no longer matches its retained numerical oracle: {mismatches:?}"
            ));
        }
        Self::validate_bug302_fresh_rendering(&table)
    }

    pub(super) fn validate_bug302_oracle(
        &self,
        deck: &XyceDeck,
        start: Instant,
    ) -> Result<(), String> {
        let abort = DeadlineAbort::new(start, self.config.max_time_per_test_ms.max(1));
        if abort.is_aborted() {
            return Err(format!("{BUG302_LABEL} deadline expired before validation"));
        }
        let artifacts = self.validate_bug302_provenance(deck)?;
        let mut dc_fingerprint = None;
        let mut tran_fingerprint = None;
        for worker in Bug302Worker::ALL {
            let key = worker.file_name.to_ascii_lowercase();
            let bytes = artifacts
                .sources
                .get(&key)
                .ok_or_else(|| format!("{BUG302_LABEL} lost {}", worker.file_name))?;
            let path = artifacts.source_dir.join(worker.file_name);
            let (fingerprint, _) = Self::validate_bug302_worker(worker, &path, bytes)?;
            let baseline = match worker.analysis {
                Bug302Analysis::Dc => &mut dc_fingerprint,
                Bug302Analysis::Tran => &mut tran_fingerprint,
            };
            match baseline {
                Some(expected) if expected != &fingerprint => {
                    return Err(format!(
                        "{BUG302_LABEL} {} changed circuit semantics beyond its print delimiter",
                        worker.file_name
                    ));
                }
                None => *baseline = Some(fingerprint),
                _ => {}
            }
        }
        Self::validate_bug302_retained_output_relations(&artifacts.output_dir)?;
        self.validate_bug302_dc_execution(
            &artifacts.source_dir.join("DC_defaults.cir"),
            &artifacts.output_dir.join("DC_defaults.cir.prn"),
            start,
        )?;
        self.validate_bug302_tran_execution(
            &artifacts.source_dir.join("transient_defaults.cir"),
            &artifacts.output_dir.join("transient_defaults.cir.prn"),
            start,
        )?;
        if abort.is_aborted() {
            return Err(format!(
                "{BUG302_LABEL} execution exceeded timeout ({}ms)",
                self.config.max_time_per_test_ms
            ));
        }
        self.validate_bug302_provenance(deck)?;
        if abort.is_aborted() {
            return Err(format!(
                "{BUG302_LABEL} final provenance exceeded timeout ({}ms)",
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

    fn canonical_deck(root: &Path) -> XyceDeck {
        XyceDeck {
            path: root.join(XYCE_BUG302_PATH),
            section: XyceDeckSection::Netlists,
            relative_path: XYCE_BUG302_PATH.to_string(),
        }
    }

    fn bug302_fixture(label: &str) -> (tempfile::TempDir, XyceDeck, XyceTestRunner) {
        let source_root = corpus_root();
        let temporary = tempfile::Builder::new()
            .prefix(&format!("rspice-xyce-bug302-{label}-"))
            .tempdir()
            .expect("create BUG302 fixture root");
        let root = temporary.path();
        let source_dir = root.join("Netlists/Certification_Tests/BUG_302");
        let output_dir = root.join("OutputData/Certification_Tests/BUG_302");
        fs::create_dir_all(&source_dir).expect("create BUG302 fixture source family");
        fs::create_dir_all(&output_dir).expect("create BUG302 fixture output family");
        for (name, ..) in XYCE_BUG302_RETAINED_SOURCES {
            fs::copy(
                source_root
                    .join("Netlists/Certification_Tests/BUG_302")
                    .join(name),
                source_dir.join(name),
            )
            .expect("copy canonical BUG302 source member");
        }
        for (name, ..) in XYCE_BUG302_RETAINED_OUTPUTS {
            fs::copy(
                source_root
                    .join("OutputData/Certification_Tests/BUG_302")
                    .join(name),
                output_dir.join(name),
            )
            .expect("copy canonical BUG302 output member");
        }
        fs::write(
            root.join(HARNESS_MANIFEST_FILE),
            format!("{XYCE_BUG302_PATH}\t{REQUIRES_UPSTREAM_WRAPPER_CONTRACT}\n"),
        )
        .expect("write BUG302 wrapper manifest");
        let mut exclusion_rows = Bug302Worker::ALL
            .into_iter()
            .map(|worker| {
                format!(
                    "Netlists/Certification_Tests/BUG_302/{}\t{XYCE_BUG302_EXCLUSION_SOURCE}\trspice_independently_qualified\t{}",
                    worker.file_name,
                    worker.expected_contract()
                )
            })
            .collect::<Vec<_>>();
        exclusion_rows.sort();
        fs::write(
            root.join(UPSTREAM_EXCLUSIONS_MANIFEST_FILE),
            format!(
                "schema_version\t{UPSTREAM_EXCLUSIONS_SCHEMA_VERSION}\nsource_commit\t{UPSTREAM_EXCLUSIONS_SOURCE_COMMIT}\nsource_netlists_tree\t{UPSTREAM_EXCLUSIONS_SOURCE_NETLISTS_TREE}\n{}\n",
                exclusion_rows.join("\n")
            ),
        )
        .expect("write BUG302 exclusion manifest");
        let deck = canonical_deck(root);
        let runner = XyceTestRunner::new(root, XyceRunnerConfig::default());
        (temporary, deck, runner)
    }

    #[test]
    fn bug302_historical_provenance_and_retained_relations_are_exact() {
        XyceTestRunner::validate_bug302_historical_oracle_provenance()
            .expect("Release-7.10 BUG302 provenance remains exact");
        let root = corpus_root();
        let runner = XyceTestRunner::new(&root, XyceRunnerConfig::default());
        let artifacts = runner
            .validate_bug302_provenance(&canonical_deck(&root))
            .expect("retained BUG302 provenance remains exact");
        XyceTestRunner::validate_bug302_retained_output_relations(&artifacts.output_dir)
            .expect("retained BUG302 delimiter relations remain exact");
    }

    #[test]
    fn bug302_all_workers_retain_one_typed_semantic_cohort_per_analysis() {
        let root = corpus_root();
        let runner = XyceTestRunner::new(&root, XyceRunnerConfig::default());
        let artifacts = runner
            .validate_bug302_provenance(&canonical_deck(&root))
            .expect("canonical BUG302 provenance passes");
        let mut fingerprints = BTreeMap::<&'static str, String>::new();
        for worker in Bug302Worker::ALL {
            let path = artifacts.source_dir.join(worker.file_name);
            let bytes = artifacts
                .sources
                .get(&worker.file_name.to_ascii_lowercase())
                .expect("worker source is retained");
            let (fingerprint, _) = XyceTestRunner::validate_bug302_worker(worker, &path, bytes)
                .expect("canonical BUG302 worker remains typed");
            let analysis = match worker.analysis {
                Bug302Analysis::Dc => "dc",
                Bug302Analysis::Tran => "tran",
            };
            if let Some(expected) = fingerprints.get(analysis) {
                assert_eq!(expected, &fingerprint);
            } else {
                fingerprints.insert(analysis, fingerprint);
            }
        }
    }

    #[test]
    fn bug302_fresh_serializer_reproduces_the_historical_transform() {
        let table = XycePrnTable {
            columns: vec!["Index".into(), "TIME".into(), "V(1)".into()],
            rows: vec![vec![0.0, 0.0, 1.0], vec![1.0, 1.0e-6, 0.999]],
        };
        XyceTestRunner::validate_bug302_fresh_rendering(&table)
            .expect("typed PRN delimiter rendering remains relationally exact");
        let default =
            "Index      TIME   V(1)  \r\n0   0.0   1.0   \r\nEnd of Xyce(TM) Simulation\r\n";
        assert_eq!(
            XyceTestRunner::historical_bug302_delimiter_transform(default, ","),
            "Index,TIME,V(1)\n0,0.0,1.0\nEnd of Xyce(TM) Simulation\n"
        );
        for (delimiter, separator) in [
            (PrintDelimiter::Colon, ":"),
            (PrintDelimiter::Semicolon, ";"),
            (PrintDelimiter::Custom("||".to_string()), "||"),
        ] {
            let rendered = XyceTestRunner::xyce_prn_text_with_delimiter(&table, &delimiter)
                .expect("full typed delimiter domain renders");
            assert!(rendered.starts_with(&format!("Index{separator}TIME{separator}V(1)")));
        }
        assert!(
            XyceTestRunner::xyce_prn_text_with_delimiter(
                &table,
                &PrintDelimiter::Custom(String::new())
            )
            .is_err()
        );
    }

    #[test]
    fn bug302_typed_workers_reject_delimiter_and_circuit_mutations() {
        let root = corpus_root();
        let comma = Bug302Worker::ALL[1];
        let path = root
            .join("Netlists/Certification_Tests/BUG_302")
            .join(comma.file_name);
        let source = fs::read_to_string(&path).expect("read canonical BUG302 DC comma worker");
        for mutation in [
            source.replace("deLIMitEr=coMmA", "deLIMitEr=COLON"),
            source.replace(".dc vds 0 3.5 0.05", ".dc vds 0 3.0 0.05"),
            source.replace("i(vds)", "i(vgs)"),
        ] {
            assert!(
                XyceTestRunner::validate_bug302_worker(comma, &path, mutation.as_bytes()).is_err(),
                "BUG302 typed worker mutation must fail closed"
            );
        }
    }

    #[test]
    fn bug302_provenance_rejects_source_output_manifest_and_path_drift() {
        let (_temporary, deck, runner) = bug302_fixture("source-drift");
        fs::write(&deck.path, "mutated owner\n").expect("mutate BUG302 owner");
        assert!(runner.validate_bug302_provenance(&deck).is_err());

        let (_temporary, deck, runner) = bug302_fixture("output-drift");
        fs::write(
            runner
                .root
                .join("OutputData/Certification_Tests/BUG_302/DC_defaults.cir.prn"),
            "mutated output\n",
        )
        .expect("mutate BUG302 output");
        assert!(runner.validate_bug302_provenance(&deck).is_err());

        let (_temporary, deck, runner) = bug302_fixture("family-drift");
        fs::write(
            deck.path
                .parent()
                .expect("BUG302 deck has parent")
                .join("unexpected.err"),
            "stale artifact\n",
        )
        .expect("add BUG302 source-family member");
        assert!(runner.validate_bug302_provenance(&deck).is_err());

        let (_temporary, deck, runner) = bug302_fixture("owner-drift");
        fs::write(runner.root.join(HARNESS_MANIFEST_FILE), "")
            .expect("remove BUG302 wrapper owner");
        let rebuilt = XyceTestRunner::new(&runner.root, XyceRunnerConfig::default());
        assert!(rebuilt.validate_bug302_provenance(&deck).is_err());

        let (_temporary, deck, runner) = bug302_fixture("exclusion-drift");
        let manifest_path = runner.root.join(UPSTREAM_EXCLUSIONS_MANIFEST_FILE);
        let manifest = fs::read_to_string(&manifest_path).expect("read BUG302 exclusions");
        fs::write(
            &manifest_path,
            manifest.replacen("static_prn_dc", "static_prn_tran", 1),
        )
        .expect("mutate BUG302 worker contract");
        assert!(runner.validate_bug302_provenance(&deck).is_err());

        let (_temporary, mut deck, runner) = bug302_fixture("path-drift");
        deck.relative_path = "Netlists/Certification_Tests/BUG_302/DC_defaults.cir".to_string();
        assert!(runner.validate_bug302_provenance(&deck).is_err());
    }

    #[test]
    fn bug302_owner_oracle_runs_both_default_analyses() {
        let root = corpus_root();
        let runner = XyceTestRunner::new(&root, XyceRunnerConfig::default());
        runner
            .validate_bug302_oracle(&canonical_deck(&root), Instant::now())
            .expect("canonical BUG302 owner oracle passes");
    }

    #[test]
    fn bug302_owner_oracle_rejects_an_expired_deadline() {
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
            .expect("construct expired deadline");
        assert!(
            runner
                .validate_bug302_oracle(&canonical_deck(&root), start)
                .is_err()
        );
    }
}
