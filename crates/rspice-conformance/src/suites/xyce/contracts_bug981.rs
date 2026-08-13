use super::*;

const LABEL: &str = "BUG_981_SON OUTPUTTIMEPOINTS/BREAKPOINTS relation";
const REQUESTED_TIMES: [Value; 4] = [1.0e-3, 2.0e-3, 3.0e-3, 4.0e-3];
const EXPECTED_VALUES: [&str; 5] = [
    "5.16060279e0",
    "6.32332358e0",
    "6.75106466e0",
    "6.90842181e0",
    "6.96631027e0",
];

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub(super) enum Bug981Role {
    WrapperOwner,
    OutputTimePoints,
    Breakpoints,
}

impl Bug981Role {
    const ALL: [Self; 3] = [
        Self::WrapperOwner,
        Self::OutputTimePoints,
        Self::Breakpoints,
    ];
    const WORKERS: [Self; 2] = [Self::OutputTimePoints, Self::Breakpoints];

    pub(super) fn for_record(record: &str) -> Option<Self> {
        let record = XyceTestRunner::normalize_manifest_key(record);
        Self::ALL.into_iter().find(|role| role.record() == record)
    }

    pub(super) fn contract(self) -> &'static str {
        match self {
            Self::WrapperOwner => XYCE_BUG981_OWNER_CONTRACT,
            Self::OutputTimePoints => XYCE_BUG981_OUTPUT_CONTRACT,
            Self::Breakpoints => XYCE_BUG981_BREAKPOINT_CONTRACT,
        }
    }

    fn path(self) -> &'static str {
        match self {
            Self::WrapperOwner => XYCE_BUG981_OWNER_PATH,
            Self::OutputTimePoints => XYCE_BUG981_OUTPUT_PATH,
            Self::Breakpoints => XYCE_BUG981_BREAKPOINT_PATH,
        }
    }

    fn record(self) -> &'static str {
        match self {
            Self::WrapperOwner => XYCE_BUG981_OWNER_RECORD,
            Self::OutputTimePoints => XYCE_BUG981_OUTPUT_RECORD,
            Self::Breakpoints => XYCE_BUG981_BREAKPOINT_RECORD,
        }
    }

    fn file_name(self) -> &'static str {
        self.path()
            .rsplit('/')
            .next()
            .expect("BUG981 path has file name")
    }

    fn label(self) -> &'static str {
        match self {
            Self::WrapperOwner => "wrapper owner",
            Self::OutputTimePoints => "OUTPUTTIMEPOINTS control",
            Self::Breakpoints => "BREAKPOINTS control",
        }
    }
}

impl XyceTestRunner {
    pub(super) fn bug981_historical_oracle_provenance_records() -> Vec<String> {
        let mut records = XYCE_BUG981_HISTORICAL_ARTIFACTS
            .into_iter()
            .map(|(path, bytes, sha256, content_blake3)| {
                format!(
                    "{XYCE_BUG981_UPSTREAM_REGRESSION_COMMIT}\t{XYCE_BUG981_UPSTREAM_RELEASE_TAG}\t{path}\t{bytes}\t{sha256}\t{content_blake3}"
                )
            })
            .collect::<Vec<_>>();
        records.sort();
        records
    }

    pub(super) fn validate_bug981_historical_oracle_provenance() -> Result<(), String> {
        let records = Self::bug981_historical_oracle_provenance_records();
        let stream = records.join("\n");
        let sha256 = format!("{:x}", Sha256::digest(stream.as_bytes()));
        let content_blake3 = blake3::hash(stream.as_bytes()).to_hex().to_string();
        if records.len() != XYCE_BUG981_HISTORICAL_RECORD_COUNT
            || stream.len() != XYCE_BUG981_HISTORICAL_RECORD_BYTES
            || sha256 != XYCE_BUG981_HISTORICAL_RECORDS_SHA256
            || content_blake3 != XYCE_BUG981_HISTORICAL_RECORDS_BLAKE3
        {
            return Err(format!(
                "{LABEL} Release-7.10 provenance changed: records={}/{}, sha256={sha256}, blake3={content_blake3}",
                records.len(),
                stream.len()
            ));
        }
        Ok(())
    }

    fn validate_bug981_directory(&self) -> Result<BTreeMap<String, Vec<u8>>, String> {
        let directory = self.root.join("Netlists/Certification_Tests/BUG_981_SON");
        let metadata = fs::symlink_metadata(&directory)
            .map_err(|error| format!("failed to inspect {LABEL} family: {error}"))?;
        if metadata.file_type().is_symlink() || !metadata.file_type().is_dir() {
            return Err(format!(
                "{LABEL} family must be a regular non-symlink directory"
            ));
        }
        let expected = XYCE_BUG981_RETAINED_ARTIFACTS
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
            let bytes = fs::read(&path)
                .map_err(|error| format!("failed to read {LABEL} member {name:?}: {error}"))?;
            let canonical = Self::canonical_lf_text_identity(LABEL, &bytes)?;
            let sha256 = format!("{:x}", Sha256::digest(&canonical));
            let content_blake3 = blake3::hash(&canonical).to_hex().to_string();
            if canonical.len() != expected_bytes
                || sha256 != expected_sha256
                || content_blake3 != expected_blake3
            {
                return Err(format!("{LABEL} member {name:?} changed"));
            }
            observed.insert(key, bytes);
        }
        if observed.len() != expected.len() {
            return Err(format!("{LABEL} retained family census changed"));
        }
        Ok(observed)
    }

    fn validate_bug981_provenance(
        &self,
        deck: &XyceDeck,
        role: Bug981Role,
    ) -> Result<BTreeMap<String, Vec<u8>>, String> {
        Self::validate_bug981_historical_oracle_provenance()?;
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

        let family_prefix = "netlists/certification_tests/bug_981_son/";
        let wrapper_records = Self::load_upstream_wrapper_decks(&self.root);
        let owners = wrapper_records
            .iter()
            .filter(|record| record.starts_with(family_prefix))
            .map(String::as_str)
            .collect::<BTreeSet<_>>();
        if owners != BTreeSet::from([XYCE_BUG981_OWNER_RECORD]) {
            return Err(format!("{LABEL} wrapper ownership changed: {owners:?}"));
        }
        let exclusions = Self::load_upstream_exclusions(&self.root)
            .map_err(|error| format!("{LABEL} exclusion manifest is invalid: {error}"))?;
        if exclusions.contains_key(XYCE_BUG981_OWNER_RECORD) {
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
        for worker in Bug981Role::WORKERS {
            let exclusion = family_exclusions
                .get(&worker.record().to_string())
                .copied()
                .ok_or_else(|| format!("{LABEL} lost {} qualification", worker.file_name()))?;
            if exclusion.source != XYCE_BUG981_EXCLUSION_SOURCE
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

        let members = self.validate_bug981_directory()?;
        let patfile = members
            .get("patfile")
            .ok_or_else(|| format!("{LABEL} lost patfile"))?;
        let canonical_patfile = Self::canonical_lf_text_identity("BUG981 patfile", patfile)?;
        if canonical_patfile.as_slice()
            != b"1.00000000e-03\n2.00000000e-03\n3.00000000e-03\n4.00000000e-03\n5.00000000e-03\nTIME\nEnd\n"
        {
            return Err(format!("{LABEL} grep pattern contract changed"));
        }
        self.reject_wrapper_output_artifacts(&self.root.join(XYCE_BUG981_OWNER_PATH))
            .map_err(|error| format!("{LABEL} owner {error}"))?;
        let output_family = self.root.join("OutputData/Certification_Tests/BUG_981_SON");
        match fs::symlink_metadata(&output_family) {
            Err(error) if error.kind() == std::io::ErrorKind::NotFound => {}
            Err(error) => return Err(format!("failed to inspect {LABEL} OutputData: {error}")),
            Ok(_) => return Err(format!("{LABEL} must not acquire invented numerical gold")),
        }
        Ok(members)
    }

    fn bug981_nodes_match(actual: &[String], expected: &[&str]) -> bool {
        actual.len() == expected.len()
            && actual
                .iter()
                .zip(expected)
                .all(|(actual, expected)| actual.eq_ignore_ascii_case(expected))
    }

    fn validate_bug981_worker(
        &self,
        role: Bug981Role,
        source: &str,
        path: &Path,
    ) -> Result<XyceStaticTranPlan, String> {
        if !Bug981Role::WORKERS.contains(&role) {
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
            if request.format.as_deref().is_some_and(|format| format.eq_ignore_ascii_case("NOINDEX"))
                && request.file.is_none()
                && matches!(request.probes.as_slice(), [probe] if probe.eq_ignore_ascii_case("V(1)")))
        {
            return Err(format!(
                "{LABEL} {} authored NOINDEX output changed",
                role.label()
            ));
        }

        let netlist = Self::parse_xyce_netlist(source, path)
            .map_err(|error| format!("{LABEL} {} no longer parses: {error}", role.label()))?;
        if netlist.title != "testing breakpoints "
            || netlist.elements.len() != 4
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
                "{LABEL} {} typed envelope changed: title={:?}, elements={:?}, analyses={:?}, outputs={:?}, diagnostics={:?}, options={:?}",
                role.label(),
                netlist.title,
                netlist.elements,
                netlist.analyses,
                netlist.output_requests,
                netlist.diagnostics,
                netlist.options
            ));
        }

        let mut remaining_options = netlist.options.clone();
        let output_time_points = std::mem::take(&mut remaining_options.output_time_points);
        let timeint_breakpoints = std::mem::take(&mut remaining_options.timeint_breakpoints);
        let schedule_matches = match role {
            Bug981Role::OutputTimePoints => {
                output_time_points == REQUESTED_TIMES && timeint_breakpoints.is_empty()
            }
            Bug981Role::Breakpoints => {
                timeint_breakpoints == REQUESTED_TIMES && output_time_points.is_empty()
            }
            Bug981Role::WrapperOwner => false,
        };
        if !schedule_matches
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

        let [offset, exponential, resistor, capacitor] = netlist.elements.as_slice() else {
            unreachable!("BUG981 element count was checked")
        };
        if offset.provenance != ElementProvenance::Authored
            || !offset.name.eq_ignore_ascii_case("VOFF")
            || !Self::bug981_nodes_match(&offset.nodes, &["A", "0"])
            || !matches!(&offset.kind, ElementKind::VoltageSource(
                rspice_core::netlist::SourceSpec::Dc(value)
            ) if value.to_bits() == 2.0f64.to_bits())
            || exponential.provenance != ElementProvenance::Authored
            || !exponential.name.eq_ignore_ascii_case("VEXP")
            || !Self::bug981_nodes_match(&exponential.nodes, &["1", "A"])
            || !matches!(&exponential.kind, ElementKind::VoltageSource(
                rspice_core::netlist::SourceSpec::Exp { v1, v2, td1, tau1, td2, tau2 }
            ) if v1.to_bits() == 0.0f64.to_bits()
                && v2.to_bits() == 5.0f64.to_bits()
                && td1.to_bits() == 0.0f64.to_bits()
                && tau1.to_bits() == 1.0e-3f64.to_bits()
                && td2.to_bits() == 1.0f64.to_bits()
                && tau2.is_nan())
            || resistor.provenance != ElementProvenance::Authored
            || !resistor.name.eq_ignore_ascii_case("R1")
            || !Self::bug981_nodes_match(&resistor.nodes, &["1", "2"])
            || !matches!(&resistor.kind, ElementKind::Resistor {
                value,
                value_expr: None,
                model: None,
                instance_params,
                deferred_params,
            } if value.to_bits() == 1.0f64.to_bits()
                && instance_params.is_empty()
                && deferred_params.is_empty())
            || capacitor.provenance != ElementProvenance::Authored
            || !capacitor.name.eq_ignore_ascii_case("C1")
            || !Self::bug981_nodes_match(&capacitor.nodes, &["2", "0"])
            || !matches!(&capacitor.kind, ElementKind::Capacitor {
                value,
                value_expr: None,
                initial_voltage: None,
                model: None,
                instance_params,
                deferred_params,
            } if value.to_bits() == 1.0f64.to_bits()
                && instance_params.is_empty()
                && deferred_params.is_empty())
        {
            return Err(format!("{LABEL} {} topology changed", role.label()));
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
        let expected_line = if role == Bug981Role::OutputTimePoints {
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

    fn validate_bug981_table(role: Bug981Role, table: &XycePrnTable) -> Result<(), String> {
        if table.columns.len() != 3
            || table.columns[0] != "Index"
            || !table.columns[1].eq_ignore_ascii_case("TIME")
            || !table.columns[2].eq_ignore_ascii_case("V(1)")
            || table.rows.is_empty()
            || table.rows.iter().enumerate().any(|(index, row)| {
                row.len() != 3
                    || row[0].to_bits() != (index as Value).to_bits()
                    || row.iter().any(|value| !value.is_finite())
            })
            || table.rows.windows(2).any(|rows| rows[0][1] >= rows[1][1])
            || table
                .rows
                .last()
                .is_none_or(|row| row[1].to_bits() != 5.0e-3f64.to_bits())
        {
            return Err(format!(
                "{LABEL} {} produced malformed output",
                role.label()
            ));
        }
        match role {
            Bug981Role::OutputTimePoints
                if table.rows.len() != 5 || table.rows[0][1].to_bits() != 1.0e-3f64.to_bits() =>
            {
                return Err(format!("{LABEL} OUTPUTTIMEPOINTS projection changed"));
            }
            Bug981Role::Breakpoints
                if table.rows.len() <= 6 || table.rows[0][1].to_bits() != 0.0f64.to_bits() =>
            {
                return Err(format!("{LABEL} BREAKPOINTS must preserve adaptive output"));
            }
            _ => {}
        }
        for (expected_time, expected_value) in REQUESTED_TIMES
            .into_iter()
            .chain([5.0e-3])
            .zip(EXPECTED_VALUES)
        {
            let row = table
                .rows
                .iter()
                .find(|row| row[1].to_bits() == expected_time.to_bits())
                .ok_or_else(|| format!("{LABEL} {} missed {expected_time}", role.label()))?;
            let actual = Self::xyce_default_prn_text(row[2])?;
            if actual != expected_value {
                return Err(format!(
                    "{LABEL} {} has V(1)={actual} at {expected_time}, expected {expected_value}",
                    role.label()
                ));
            }
        }
        Ok(())
    }

    fn bug981_noindex_table(table: &XycePrnTable) -> Result<XycePrnTable, String> {
        if table.columns.first().is_none_or(|column| column != "Index") {
            return Err(format!("{LABEL} indexed observation lost Index"));
        }
        Ok(XycePrnTable {
            columns: table.columns[1..].to_vec(),
            rows: table
                .rows
                .iter()
                .map(|row| {
                    row.get(1..)
                        .map(<[Value]>::to_vec)
                        .ok_or_else(|| format!("{LABEL} row lost its no-index projection"))
                })
                .collect::<Result<_, _>>()?,
        })
    }

    fn validate_bug981_relation(
        output: &XycePrnTable,
        breakpoints: &XycePrnTable,
    ) -> Result<(), String> {
        let output = Self::bug981_noindex_table(output)?;
        let targets = REQUESTED_TIMES
            .into_iter()
            .chain([5.0e-3])
            .map(Self::xyce_default_prn_text)
            .collect::<Result<BTreeSet<_>, _>>()?;
        let filtered = XycePrnTable {
            columns: breakpoints.columns[1..].to_vec(),
            rows: breakpoints
                .rows
                .iter()
                .filter(|row| {
                    row.get(1)
                        .and_then(|time| Self::xyce_default_prn_text(*time).ok())
                        .is_some_and(|time| targets.contains(&time))
                })
                .map(|row| row[1..].to_vec())
                .collect(),
        };
        if filtered.rows.len() != 5 {
            return Err(format!(
                "{LABEL} patfile selection found {} data rows",
                filtered.rows.len()
            ));
        }
        let output_text = Self::xyce_prn_text_with_delimiter(&output, &PrintDelimiter::Whitespace)?;
        let filtered_text =
            Self::xyce_prn_text_with_delimiter(&filtered, &PrintDelimiter::Whitespace)?;
        if output_text != filtered_text {
            return Err(format!("{LABEL} Release grep/diff relation changed"));
        }
        Ok(())
    }

    fn run_bug981_worker(
        &self,
        role: Bug981Role,
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
        Self::validate_bug981_table(role, &table)?;
        Ok(table)
    }

    pub(super) fn validate_bug981_oracle(
        &self,
        deck: &XyceDeck,
        role: Bug981Role,
        start: Instant,
    ) -> Result<(), String> {
        let abort = DeadlineAbort::new(start, self.config.max_time_per_test_ms.max(1));
        if abort.is_aborted() {
            return Err(format!("{LABEL} deadline expired before provenance"));
        }
        let members = self.validate_bug981_provenance(deck, role)?;
        let mut plans = BTreeMap::new();
        for worker in Bug981Role::WORKERS {
            let bytes = members
                .get(&worker.file_name().to_ascii_lowercase())
                .ok_or_else(|| format!("{LABEL} lost {}", worker.file_name()))?;
            let source = std::str::from_utf8(bytes)
                .map_err(|error| format!("{LABEL} {} is not UTF-8: {error}", worker.file_name()))?;
            plans.insert(
                worker,
                self.validate_bug981_worker(worker, source, &self.root.join(worker.path()))?,
            );
        }
        if abort.is_aborted() {
            return Err(format!("{LABEL} validation exceeded deadline"));
        }
        let output = self.run_bug981_worker(
            Bug981Role::OutputTimePoints,
            plans
                .get(&Bug981Role::OutputTimePoints)
                .expect("output plan"),
            start,
        )?;
        let breakpoints = self.run_bug981_worker(
            Bug981Role::Breakpoints,
            plans
                .get(&Bug981Role::Breakpoints)
                .expect("breakpoint plan"),
            start,
        )?;
        Self::validate_bug981_relation(&output, &breakpoints)?;
        self.validate_bug981_provenance(deck, role)?;
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
            .prefix(&format!("rspice-xyce-bug981-{label}-"))
            .tempdir()
            .expect("create BUG981 fixture");
        let root = temporary.path();
        let family = root.join("Netlists/Certification_Tests/BUG_981_SON");
        fs::create_dir_all(&family).expect("create BUG981 family");
        let canonical = corpus_root().join("Netlists/Certification_Tests/BUG_981_SON");
        for (name, ..) in XYCE_BUG981_RETAINED_ARTIFACTS {
            fs::copy(canonical.join(name), family.join(name)).expect("copy BUG981 member");
        }
        fs::write(
            root.join(HARNESS_MANIFEST_FILE),
            format!(
                "{}\t{}\n",
                XYCE_BUG981_OWNER_PATH, REQUIRES_UPSTREAM_WRAPPER_CONTRACT
            ),
        )
        .expect("write wrapper manifest");
        fs::write(
            root.join(UPSTREAM_EXCLUSIONS_MANIFEST_FILE),
            format!(
                "schema_version\t{UPSTREAM_EXCLUSIONS_SCHEMA_VERSION}\nsource_commit\t{UPSTREAM_EXCLUSIONS_SOURCE_COMMIT}\nsource_netlists_tree\t{UPSTREAM_EXCLUSIONS_SOURCE_NETLISTS_TREE}\n{}\t{}\t{}\t{}\n{}\t{}\t{}\t{}\n",
                XYCE_BUG981_OUTPUT_PATH,
                XYCE_BUG981_EXCLUSION_SOURCE,
                RSPICE_INDEPENDENTLY_QUALIFIED_DISPOSITION,
                XYCE_BUG981_OUTPUT_CONTRACT,
                XYCE_BUG981_BREAKPOINT_PATH,
                XYCE_BUG981_EXCLUSION_SOURCE,
                RSPICE_INDEPENDENTLY_QUALIFIED_DISPOSITION,
                XYCE_BUG981_BREAKPOINT_CONTRACT,
            ),
        )
        .expect("write exclusion manifest");
        let runner = XyceTestRunner::new(root, XyceRunnerConfig::default());
        let deck = XyceDeck {
            path: root.join(XYCE_BUG981_OWNER_PATH),
            section: XyceDeckSection::Netlists,
            relative_path: XYCE_BUG981_OWNER_PATH.to_string(),
        };
        (temporary, deck, runner)
    }

    #[test]
    fn bug981_historical_provenance_is_exact() {
        XyceTestRunner::validate_bug981_historical_oracle_provenance()
            .expect("BUG981 historical provenance");
    }

    #[test]
    fn bug981_workers_have_exact_typed_contracts() {
        let runner = XyceTestRunner::new(corpus_root(), XyceRunnerConfig::default());
        for role in Bug981Role::WORKERS {
            let path = runner.root.join(role.path());
            let source = fs::read_to_string(&path).expect("read BUG981 worker");
            runner
                .validate_bug981_worker(role, &source, &path)
                .expect("validate BUG981 worker");
        }
    }

    #[test]
    fn bug981_oracle_executes_adaptive_schedule_relation() {
        let runner = XyceTestRunner::new(corpus_root(), XyceRunnerConfig::default());
        let deck = XyceDeck {
            path: runner.root.join(XYCE_BUG981_OWNER_PATH),
            section: XyceDeckSection::Netlists,
            relative_path: XYCE_BUG981_OWNER_PATH.to_string(),
        };
        runner
            .validate_bug981_oracle(&deck, Bug981Role::WrapperOwner, Instant::now())
            .expect("execute BUG981 relation");
    }

    #[test]
    fn bug981_typed_contract_rejects_schedule_and_stale_source_mutations() {
        let temporary = tempfile::tempdir().expect("create BUG981 worker fixture");
        let path = temporary.path().join(XYCE_BUG981_OUTPUT_PATH);
        fs::create_dir_all(path.parent().expect("worker parent")).expect("create worker family");
        let canonical = fs::read_to_string(corpus_root().join(XYCE_BUG981_OUTPUT_PATH))
            .expect("read BUG981 control");
        let mutated = canonical.replace("4ms", "4.5ms");
        fs::write(&path, &mutated).expect("write schedule mutation");
        let runner = XyceTestRunner::new(temporary.path(), XyceRunnerConfig::default());
        assert!(
            runner
                .validate_bug981_worker(Bug981Role::OutputTimePoints, &mutated, &path)
                .is_err()
        );
        fs::write(&path, &canonical).expect("restore worker");
        assert!(
            runner
                .validate_bug981_worker(Bug981Role::OutputTimePoints, &mutated, &path)
                .is_err(),
            "planner source must match the caller's provenance-bound bytes"
        );
    }

    #[test]
    fn bug981_relation_rejects_selected_row_value_and_census_mutations() {
        let output = XycePrnTable {
            columns: vec!["Index".into(), "TIME".into(), "V(1)".into()],
            rows: REQUESTED_TIMES
                .into_iter()
                .chain([5.0e-3])
                .zip([1.0, 2.0, 3.0, 4.0, 5.0])
                .enumerate()
                .map(|(index, (time, value))| vec![index as Value, time, value])
                .collect(),
        };
        let mut breakpoints = XycePrnTable {
            columns: output.columns.clone(),
            rows: std::iter::once(vec![0.0, 0.0, 0.0])
                .chain(
                    output
                        .rows
                        .iter()
                        .enumerate()
                        .map(|(index, row)| vec![(index + 1) as Value, row[1], row[2]]),
                )
                .collect(),
        };
        XyceTestRunner::validate_bug981_relation(&output, &breakpoints)
            .expect("canonical structural grep/diff relation");

        breakpoints.rows[2][2] += 1.0e-4;
        assert!(XyceTestRunner::validate_bug981_relation(&output, &breakpoints).is_err());
        breakpoints.rows[2][2] -= 1.0e-4;
        breakpoints.rows.remove(3);
        assert!(XyceTestRunner::validate_bug981_relation(&output, &breakpoints).is_err());
    }

    #[test]
    fn bug981_provenance_rejects_source_and_role_drift() {
        let (_temporary, deck, runner) = fixture("drift");
        runner
            .validate_bug981_provenance(&deck, Bug981Role::WrapperOwner)
            .expect("canonical BUG981 fixture");
        fs::write(runner.root.join(XYCE_BUG981_OUTPUT_PATH), "* mutated\n").expect("mutate worker");
        assert!(
            runner
                .validate_bug981_provenance(&deck, Bug981Role::WrapperOwner)
                .is_err()
        );

        let (_temporary, deck, runner) = fixture("role");
        let manifest = runner.root.join(UPSTREAM_EXCLUSIONS_MANIFEST_FILE);
        let text = fs::read_to_string(&manifest).expect("read exclusions");
        fs::write(
            &manifest,
            text.replace(XYCE_BUG981_OUTPUT_CONTRACT, XYCE_BUG981_BREAKPOINT_CONTRACT),
        )
        .expect("mutate qualification role");
        assert!(
            runner
                .validate_bug981_provenance(&deck, Bug981Role::WrapperOwner)
                .is_err(),
            "worker qualification-role drift must fail closed"
        );

        let (_temporary, deck, runner) = fixture("owner");
        fs::write(runner.root.join(HARNESS_MANIFEST_FILE), "")
            .expect("remove wrapper ownership after runner construction");
        assert!(
            runner
                .validate_bug981_provenance(&deck, Bug981Role::WrapperOwner)
                .is_err(),
            "post-construction wrapper-manifest drift must not be hidden by the runner cache"
        );
    }

    #[test]
    fn bug981_oracle_rejects_expired_shared_deadline() {
        let runner = XyceTestRunner::new(corpus_root(), XyceRunnerConfig::default());
        let deck = XyceDeck {
            path: runner.root.join(XYCE_BUG981_OWNER_PATH),
            section: XyceDeckSection::Netlists,
            relative_path: XYCE_BUG981_OWNER_PATH.to_string(),
        };
        let expired = Instant::now()
            - Duration::from_millis(
                u64::try_from(runner.config.max_time_per_test_ms.max(1) + 1)
                    .expect("timeout fits u64"),
            );
        assert!(
            runner
                .validate_bug981_oracle(&deck, Bug981Role::WrapperOwner, expired)
                .is_err()
        );
    }
}
