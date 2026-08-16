use super::*;

const LABEL: &str = "BUG_1797 BSIM3 LEVEL=9/49 alias relation";

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub(super) enum Bug1797Role {
    WrapperOwner,
    Level9,
    Level49,
}

impl Bug1797Role {
    const ALL: [Self; 3] = [Self::WrapperOwner, Self::Level9, Self::Level49];
    const WORKERS: [Self; 2] = [Self::Level9, Self::Level49];

    pub(super) fn for_record(record: &str) -> Option<Self> {
        let record = XyceTestRunner::normalize_manifest_key(record);
        Self::ALL.into_iter().find(|role| role.record() == record)
    }

    pub(super) fn contract(self) -> &'static str {
        match self {
            Self::WrapperOwner => XYCE_BUG1797_OWNER_CONTRACT,
            Self::Level9 => XYCE_BUG1797_LEVEL9_CONTRACT,
            Self::Level49 => XYCE_BUG1797_LEVEL49_CONTRACT,
        }
    }

    fn path(self) -> &'static str {
        match self {
            Self::WrapperOwner => XYCE_BUG1797_OWNER_PATH,
            Self::Level9 => XYCE_BUG1797_LEVEL9_PATH,
            Self::Level49 => XYCE_BUG1797_LEVEL49_PATH,
        }
    }

    fn record(self) -> &'static str {
        match self {
            Self::WrapperOwner => XYCE_BUG1797_OWNER_RECORD,
            Self::Level9 => XYCE_BUG1797_LEVEL9_RECORD,
            Self::Level49 => XYCE_BUG1797_LEVEL49_RECORD,
        }
    }

    fn file_name(self) -> &'static str {
        match self {
            Self::WrapperOwner => "one-shot.cir",
            Self::Level9 => "one-shot_lev9.cir",
            Self::Level49 => "one-shot_lev49.cir",
        }
    }

    fn level(self) -> Option<Value> {
        match self {
            Self::WrapperOwner => None,
            Self::Level9 => Some(9.0),
            Self::Level49 => Some(49.0),
        }
    }
}

impl XyceTestRunner {
    pub(super) fn bug1797_historical_oracle_provenance_records() -> Vec<String> {
        let mut records = XYCE_BUG1797_HISTORICAL_ARTIFACTS
            .into_iter()
            .map(|(path, bytes, sha256, content_blake3)| {
                format!(
                    "{XYCE_BUG1797_UPSTREAM_REGRESSION_COMMIT}\t{XYCE_BUG1797_UPSTREAM_RELEASE_TAG}\t{path}\t{bytes}\t{sha256}\t{content_blake3}"
                )
            })
            .collect::<Vec<_>>();
        records.sort();
        records
    }

    pub(super) fn validate_bug1797_historical_oracle_provenance() -> Result<(), String> {
        let records = Self::bug1797_historical_oracle_provenance_records();
        let stream = records.join("\n");
        let sha256 = format!("{:x}", Sha256::digest(stream.as_bytes()));
        let content_blake3 = blake3::hash(stream.as_bytes()).to_hex().to_string();
        if records.len() != XYCE_BUG1797_HISTORICAL_RECORD_COUNT
            || stream.len() != XYCE_BUG1797_HISTORICAL_RECORD_BYTES
            || sha256 != XYCE_BUG1797_HISTORICAL_RECORDS_SHA256
            || content_blake3 != XYCE_BUG1797_HISTORICAL_RECORDS_BLAKE3
        {
            return Err(format!(
                "{LABEL} Release-7.10 provenance changed: records={}/{}, sha256={sha256}, blake3={content_blake3}",
                records.len(),
                stream.len()
            ));
        }
        Ok(())
    }

    fn validate_bug1797_provenance(
        &self,
        deck: &XyceDeck,
        role: Bug1797Role,
    ) -> Result<BTreeMap<Bug1797Role, Vec<u8>>, String> {
        Self::validate_bug1797_historical_oracle_provenance()?;
        if deck.section != XyceDeckSection::Netlists
            || Self::normalize_manifest_key(&deck.relative_path) != role.record()
            || Self::normalize_manifest_key(&self.relative_key(&deck.path)) != role.record()
            || !Self::same_path(&deck.path, &self.root.join(role.path()))
        {
            return Err(format!(
                "recognized {LABEL} member is not at its canonical path"
            ));
        }

        let family_prefix = "netlists/certification_tests/bug_1797/";
        let owners = self
            .upstream_wrapper_decks
            .iter()
            .filter(|record| record.starts_with(family_prefix))
            .collect::<Vec<_>>();
        if owners.len() != 1 || owners[0].as_str() != XYCE_BUG1797_OWNER_RECORD {
            return Err(format!(
                "{LABEL} requires its empty anchor as sole wrapper owner"
            ));
        }
        let exclusions = Self::load_upstream_exclusions(&self.root)
            .map_err(|error| format!("{LABEL} exclusion manifest is invalid: {error}"))?;
        if exclusions.contains_key(XYCE_BUG1797_OWNER_RECORD) {
            return Err(format!("{LABEL} wrapper owner must not be excluded"));
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
        for worker in Bug1797Role::WORKERS {
            let exclusion = family_exclusions
                .get(&worker.record().to_string())
                .copied()
                .ok_or_else(|| format!("{LABEL} lost {} qualification", worker.file_name()))?;
            if exclusion.source != XYCE_BUG1797_EXCLUSION_SOURCE
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

        let family_dir = self.root.join("Netlists/Certification_Tests/BUG_1797");
        let metadata = fs::symlink_metadata(&family_dir)
            .map_err(|error| format!("failed to inspect {LABEL} family: {error}"))?;
        if metadata.file_type().is_symlink() || !metadata.file_type().is_dir() {
            return Err(format!(
                "{LABEL} family must be a regular non-symlink directory"
            ));
        }
        let expected = XYCE_BUG1797_RETAINED_ARTIFACTS
            .into_iter()
            .map(|artifact| (artifact.0.to_ascii_lowercase(), artifact))
            .collect::<BTreeMap<_, _>>();
        let mut observed = BTreeMap::new();
        for entry in fs::read_dir(&family_dir)
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
                return Err(format!("{LABEL} contains a case-colliding member {name:?}"));
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
                .map_err(|error| format!("failed to read {LABEL} member: {error}"))?;
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
        for member in Bug1797Role::ALL {
            self.reject_wrapper_output_artifacts(&self.root.join(member.path()))
                .map_err(|error| format!("{LABEL} {} {error}", member.file_name()))?;
        }
        let output_family = self.root.join("OutputData/Certification_Tests/BUG_1797");
        match fs::symlink_metadata(&output_family) {
            Err(error) if error.kind() == std::io::ErrorKind::NotFound => {}
            Err(error) => return Err(format!("failed to inspect {LABEL} OutputData: {error}")),
            Ok(_) => {
                return Err(format!(
                    "{LABEL} must not acquire an invented numerical gold"
                ));
            }
        }
        Bug1797Role::ALL
            .into_iter()
            .map(|member| {
                observed
                    .get(&member.file_name().to_ascii_lowercase())
                    .cloned()
                    .map(|bytes| (member, bytes))
                    .ok_or_else(|| format!("{LABEL} lost {}", member.file_name()))
            })
            .collect()
    }

    fn bug1797_param(params: &[(String, Value)], key: &str) -> Option<Value> {
        params
            .iter()
            .find(|(name, _)| name.eq_ignore_ascii_case(key))
            .map(|(_, value)| *value)
    }

    fn validate_bug1797_worker(
        &self,
        role: Bug1797Role,
        source: &str,
        path: &Path,
    ) -> Result<XyceStaticTranPlan, String> {
        let level = role
            .level()
            .ok_or_else(|| format!("{LABEL} owner is not executable"))?;
        let plan = self.static_tran_plan_for_path_with_purpose(
            path,
            XyceStaticTranPlanPurpose::Bug1797RelationalFamily,
        )?;
        if plan.deck_path != path
            || plan.source.as_bytes() != source.as_bytes()
            || !matches!(plan.oracle, XyceStaticTranOracle::None)
            || plan.contract != XyceStaticTranContract::PlainStatic
            || plan.comparison_mode != XyceStaticTranComparisonMode::Pointwise
            || plan.output_override
            || plan.timeint_conststep
            || !plan.steps.is_empty()
            || plan.wrapper_tolerance.is_some()
            || plan.tran.step.to_bits() != 1.0e-9f64.to_bits()
            || plan.tran.stop.to_bits() != 1.0e-8f64.to_bits()
            || plan.tran.start.is_some()
            || plan.tran.max_step.is_some()
            || plan.tran.uic
            || plan.print.as_ref().is_none_or(|print| {
                print.probes.len() != 2
                    || !print.probes[0].eq_ignore_ascii_case("V(in)")
                    || !print.probes[1].eq_ignore_ascii_case("V(out)")
            })
        {
            return Err(format!(
                "{LABEL} {} plan changed: {plan:?}",
                role.file_name()
            ));
        }
        let netlist = Self::parse_xyce_netlist(source, path)
            .map_err(|error| format!("{LABEL} {} no longer parses: {error}", role.file_name()))?;
        if netlist.title != "One-shot Trigger."
            || !netlist.diagnostics.is_empty()
            || netlist.elements.len() != 21
            || netlist.models.len() != 2
            || netlist.analyses.len() != 1
            || netlist.output_requests.len() != 1
            || !netlist.subcircuits.is_empty()
            || !netlist.data_tables.is_empty()
            || !netlist.measurements.is_empty()
            || !netlist.initial_conditions.is_empty()
            || !netlist.node_sets.is_empty()
            || !netlist.global_nodes.is_empty()
            || !netlist.veriloga_includes.is_empty()
            || !netlist.spef_includes.is_empty()
            || !netlist.params.numeric_parameters().is_empty()
            || !netlist.params.all_functions().is_empty()
            || !Self::netlist_is_native_bug1797_bsim3_envelope(&netlist)
        {
            return Err(format!(
                "{LABEL} {} typed envelope changed",
                role.file_name()
            ));
        }

        let expected_mos: [(&str, [&str; 4], &str, Value, Value); 17] = [
            ("MD1", ["4", "IN", "VDD", "VDD"], "PMOS", 3.6e-6, 1.2e-6),
            ("MD2", ["4", "IN", "0", "0"], "NMOS", 1.8e-6, 1.2e-6),
            ("MD3", ["A", "4", "VDD", "VDD"], "PMOS", 3.6e-6, 1.2e-6),
            ("MD4", ["A", "4", "0", "0"], "NMOS", 1.8e-6, 1.2e-6),
            ("M1", ["ANOT", "A", "VDD", "VDD"], "PMOS", 3.6e-6, 1.2e-6),
            ("M2", ["ANOT", "A", "0", "0"], "NMOS", 1.8e-6, 1.2e-6),
            ("M3", ["BNOT", "IN", "VDD", "VDD"], "PMOS", 3.6e-6, 1.2e-6),
            ("M4", ["BNOT", "IN", "0", "0"], "NMOS", 1.8e-6, 1.2e-6),
            ("M5", ["AORBNOT", "0", "VDD", "VDD"], "PMOS", 1.8e-6, 3.6e-6),
            ("M6", ["AORBNOT", "IN", "1", "0"], "NMOS", 1.8e-6, 1.2e-6),
            ("M7", ["1", "ANOT", "0", "0"], "NMOS", 1.8e-6, 1.2e-6),
            ("M8", ["LNOT", "0", "VDD", "VDD"], "PMOS", 1.8e-6, 3.6e-6),
            ("M9", ["LNOT", "BNOT", "2", "0"], "NMOS", 1.8e-6, 1.2e-6),
            ("M10", ["2", "A", "0", "0"], "NMOS", 1.8e-6, 1.2e-6),
            ("M11", ["OUT", "0", "VDD", "VDD"], "PMOS", 3.6e-6, 3.6e-6),
            ("M12", ["OUT", "AORBNOT", "3", "0"], "NMOS", 1.8e-6, 1.2e-6),
            ("M13", ["3", "LNOT", "0", "0"], "NMOS", 1.8e-6, 1.2e-6),
        ];
        let mosfets = netlist
            .elements
            .iter()
            .filter(|element| matches!(element.kind, ElementKind::Mosfet { .. }))
            .collect::<Vec<_>>();
        for (element, (name, nodes, model_name, width, length)) in
            mosfets.into_iter().zip(expected_mos)
        {
            let ElementKind::Mosfet {
                model,
                compact_syntax,
                instance_params,
                deferred_params,
                ..
            } = &element.kind
            else {
                unreachable!()
            };
            if !element.name.eq_ignore_ascii_case(name)
                || element.nodes.len() != 4
                || element
                    .nodes
                    .iter()
                    .zip(nodes)
                    .any(|(actual, expected)| !actual.eq_ignore_ascii_case(expected))
                || !model.eq_ignore_ascii_case(model_name)
                || *compact_syntax
                || instance_params.len() != 2
                || Self::bug1797_param(instance_params, "W")
                    .is_none_or(|value| value.to_bits() != width.to_bits())
                || Self::bug1797_param(instance_params, "L")
                    .is_none_or(|value| value.to_bits() != length.to_bits())
                || !deferred_params.is_empty()
            {
                return Err(format!(
                    "{LABEL} {} MOSFET {name} changed",
                    role.file_name()
                ));
            }
        }
        for model_name in ["NMOS", "PMOS"] {
            let model = netlist
                .models
                .iter()
                .find(|model| model.name.eq_ignore_ascii_case(model_name))
                .ok_or_else(|| format!("{LABEL} lost {model_name} model"))?;
            if !model.model_type.eq_ignore_ascii_case(model_name)
                || !matches!(model.params.as_slice(), [(name, value)]
                    if name.eq_ignore_ascii_case("LEVEL") && value.to_bits() == level.to_bits())
            {
                return Err(format!(
                    "{LABEL} {} {model_name} model changed",
                    role.file_name()
                ));
            }
        }
        let capacitors = netlist
            .elements
            .iter()
            .filter(|element| matches!(element.kind, ElementKind::Capacitor { .. }))
            .collect::<Vec<_>>();
        if capacitors.len() != 2
            || !matches!(&capacitors[0].kind, ElementKind::Capacitor { value, value_expr: None, initial_voltage: None, model: None, instance_params, deferred_params }
            if capacitors[0].name.eq_ignore_ascii_case("C4") && capacitors[0].nodes == ["4", "0"] && value.to_bits() == (30.0f64 * 1e-15).to_bits() && instance_params.is_empty() && deferred_params.is_empty())
            || !matches!(&capacitors[1].kind, ElementKind::Capacitor { value, value_expr: None, initial_voltage: None, model: None, instance_params, deferred_params }
            if capacitors[1].name.eq_ignore_ascii_case("CA") && capacitors[1].nodes.iter().zip(["A", "0"]).all(|(a,b)| a.eq_ignore_ascii_case(b)) && value.to_bits() == (30.0f64 * 1e-15).to_bits() && instance_params.is_empty() && deferred_params.is_empty())
        {
            return Err(format!("{LABEL} {} capacitors changed", role.file_name()));
        }
        let sources = netlist
            .elements
            .iter()
            .filter(|element| matches!(element.kind, ElementKind::VoltageSource(_)))
            .collect::<Vec<_>>();
        if sources.len() != 2
            || !matches!(&sources[0].kind, ElementKind::VoltageSource(rspice_core::netlist::SourceSpec::Dc(value))
                if sources[0].name.eq_ignore_ascii_case("VCC") && sources[0].nodes.iter().zip(["VDD", "0"]).all(|(a,b)| a.eq_ignore_ascii_case(b)) && value.to_bits() == 5.0f64.to_bits())
            || !matches!(&sources[1].kind, ElementKind::VoltageSource(rspice_core::netlist::SourceSpec::Pulse { v1, v2, delay, rise, fall, width, period, pulse_count, width_defaults_to_zero })
                if sources[1].name.eq_ignore_ascii_case("VIN") && sources[1].nodes.iter().zip(["IN", "0"]).all(|(a,b)| a.eq_ignore_ascii_case(b))
                    && v1.to_bits() == 0.0f64.to_bits() && v2.to_bits() == 5.0f64.to_bits()
                    && delay.to_bits() == 1e-9f64.to_bits() && rise.to_bits() == (0.1f64 * 1e-9).to_bits()
                    && fall.to_bits() == (0.1f64 * 1e-9).to_bits() && width.to_bits() == (0.8f64 * 1e-9).to_bits()
                    && period.to_bits() == 5e-9f64.to_bits() && pulse_count.to_bits() == 0.0f64.to_bits() && !width_defaults_to_zero)
        {
            return Err(format!("{LABEL} {} sources changed", role.file_name()));
        }
        Ok(plan)
    }

    fn run_bug1797_worker(
        &self,
        role: Bug1797Role,
        plan: &XyceStaticTranPlan,
        start: Instant,
    ) -> Result<XycePrnTable, String> {
        let (netlist, result) = self
            .run_transient_family_plan(plan, start, None, None)
            .map_err(|error| format!("{LABEL} {} failed: {error}", role.file_name()))?;
        let table = Self::transient_family_result_to_prn_table(plan, &netlist, &result)
            .map_err(|error| format!("{LABEL} {} PRN failed: {error}", role.file_name()))?;
        if table.columns.len() != 4
            || table.columns[0] != "Index"
            || !table.columns[1].eq_ignore_ascii_case("TIME")
            || !table.columns[2].eq_ignore_ascii_case("V(in)")
            || !table.columns[3].eq_ignore_ascii_case("V(out)")
            || table.rows.len() < 10
            || table
                .rows
                .iter()
                .any(|row| row.len() != 4 || row.iter().any(|value| !value.is_finite()))
            || table.rows.first().is_none_or(|row| row[1].abs() > 1e-18)
            || table
                .rows
                .last()
                .is_none_or(|row| (row[1] - 1e-8).abs() > 1e-15)
            || table
                .rows
                .iter()
                .map(|row| row[2])
                .fold(Value::NEG_INFINITY, Value::max)
                < 4.9
            || table
                .rows
                .iter()
                .map(|row| row[3].abs())
                .fold(0.0, Value::max)
                < 1e-3
        {
            return Err(format!(
                "{LABEL} {} produced an invalid or trivial table",
                role.file_name()
            ));
        }
        Ok(table)
    }

    fn validate_bug1797_intended_relation(
        gold: &XycePrnTable,
        test: &XycePrnTable,
        tolerance: XyceFileCompareTolerance,
    ) -> Result<(), String> {
        if gold.columns != test.columns || gold.rows.len() != test.rows.len() {
            return Err(format!("{LABEL} intended relation schema/grid differs"));
        }
        for (row_index, (gold_row, test_row)) in gold.rows.iter().zip(&test.rows).enumerate() {
            if gold_row.len() != gold.columns.len() || test_row.len() != test.columns.len() {
                return Err(format!("{LABEL} row {row_index} has invalid width"));
            }
            for (column, (&gold_value, &test_value)) in gold_row.iter().zip(test_row).enumerate() {
                let gold_value = Self::xyce_default_prn_roundtrip(gold_value)?;
                let test_value = Self::xyce_default_prn_roundtrip(test_value)?;
                let difference = (test_value - gold_value).abs();
                let pass = test_value == gold_value
                    || (test_value.abs() <= tolerance.zero && gold_value.abs() <= tolerance.zero)
                    || (difference < tolerance.absolute
                        && difference / gold_value.abs() < tolerance.relative);
                if !pass {
                    return Err(format!(
                        "{LABEL} intended relation differs at row {row_index}, column {}: gold={gold_value}, test={test_value}",
                        gold.columns[column]
                    ));
                }
            }
        }
        Ok(())
    }

    pub(super) fn validate_bug1797_oracle(
        &self,
        deck: &XyceDeck,
        role: Bug1797Role,
        start: Instant,
    ) -> Result<(), String> {
        let abort = DeadlineAbort::new(start, self.config.max_time_per_test_ms.max(1));
        if abort.is_aborted() {
            return Err(format!("{LABEL} deadline expired before validation"));
        }
        let sources = self.validate_bug1797_provenance(deck, role)?;
        let mut output = BTreeMap::new();
        for worker in Bug1797Role::WORKERS {
            let bytes = sources
                .get(&worker)
                .ok_or_else(|| format!("{LABEL} lost worker"))?;
            let source = std::str::from_utf8(bytes)
                .map_err(|error| format!("{LABEL} worker is not UTF-8: {error}"))?;
            let path = self.root.join(worker.path());
            let plan = self.validate_bug1797_worker(worker, source, &path)?;
            output.insert(worker, self.run_bug1797_worker(worker, &plan, start)?);
        }
        let level9 = output.get(&Bug1797Role::Level9).expect("both workers ran");
        let level49 = output.get(&Bug1797Role::Level49).expect("both workers ran");
        let tolerance = XyceFileCompareTolerance {
            absolute: 2e-8,
            relative: 1e-7,
            zero: 1e-20,
        };
        let historical =
            self.compare_release_7_10_file_compare_tables(level49, level9, tolerance)?;
        if !historical.is_empty() {
            return Err(format!(
                "{LABEL} historical file_compare failed: {historical:?}"
            ));
        }
        // `file_compare.pl` contains a malformed FFT phase escape that is
        // almost vacuous for voltages below 180.  Preserve that executable
        // result above, then also enforce the README's intended numeric alias
        // relation without the phase escape.
        Self::validate_bug1797_intended_relation(level49, level9, tolerance)?;
        if abort.is_aborted() {
            return Err(format!("{LABEL} execution exceeded deadline"));
        }
        self.validate_bug1797_provenance(deck, role)?;
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
            .prefix(&format!("rspice-xyce-bug1797-{label}-"))
            .tempdir()
            .expect("create BUG1797 fixture");
        let root = temporary.path();
        let family = root.join("Netlists/Certification_Tests/BUG_1797");
        fs::create_dir_all(&family).expect("create BUG1797 family");
        let canonical = corpus_root().join("Netlists/Certification_Tests/BUG_1797");
        for (name, ..) in XYCE_BUG1797_RETAINED_ARTIFACTS {
            fs::copy(canonical.join(name), family.join(name)).expect("copy BUG1797 member");
        }
        fs::write(
            root.join(HARNESS_MANIFEST_FILE),
            format!(
                "{}\t{}\n",
                XYCE_BUG1797_OWNER_PATH, REQUIRES_UPSTREAM_WRAPPER_CONTRACT
            ),
        )
        .expect("write wrapper manifest");
        fs::write(
            root.join(UPSTREAM_EXCLUSIONS_MANIFEST_FILE),
            format!(
                "schema_version\t{UPSTREAM_EXCLUSIONS_SCHEMA_VERSION}\nsource_commit\t{UPSTREAM_EXCLUSIONS_SOURCE_COMMIT}\nsource_netlists_tree\t{UPSTREAM_EXCLUSIONS_SOURCE_NETLISTS_TREE}\n{}\t{}\t{}\t{}\n{}\t{}\t{}\t{}\n",
                XYCE_BUG1797_LEVEL49_PATH,
                XYCE_BUG1797_EXCLUSION_SOURCE,
                RSPICE_INDEPENDENTLY_QUALIFIED_DISPOSITION,
                XYCE_BUG1797_LEVEL49_CONTRACT,
                XYCE_BUG1797_LEVEL9_PATH,
                XYCE_BUG1797_EXCLUSION_SOURCE,
                RSPICE_INDEPENDENTLY_QUALIFIED_DISPOSITION,
                XYCE_BUG1797_LEVEL9_CONTRACT,
            ),
        )
        .expect("write exclusion manifest");
        let runner = XyceTestRunner::new(root, XyceRunnerConfig::default());
        let deck = XyceDeck {
            path: root.join(XYCE_BUG1797_OWNER_PATH),
            section: XyceDeckSection::Netlists,
            relative_path: XYCE_BUG1797_OWNER_PATH.to_string(),
        };
        (temporary, deck, runner)
    }

    #[test]
    fn bug1797_historical_provenance_is_exact() {
        XyceTestRunner::validate_bug1797_historical_oracle_provenance()
            .expect("BUG1797 historical provenance");
    }

    #[test]
    fn bug1797_workers_have_exact_typed_contracts() {
        let runner = XyceTestRunner::new(corpus_root(), XyceRunnerConfig::default());
        for role in Bug1797Role::WORKERS {
            let path = runner.root.join(role.path());
            let source = fs::read_to_string(&path).expect("read BUG1797 worker");
            runner
                .validate_bug1797_worker(role, &source, &path)
                .expect("validate BUG1797 worker");
        }
    }

    #[test]
    fn bug1797_oracle_executes_both_aliases() {
        let runner = XyceTestRunner::new(corpus_root(), XyceRunnerConfig::default());
        let deck = XyceDeck {
            path: runner.root.join(XYCE_BUG1797_OWNER_PATH),
            section: XyceDeckSection::Netlists,
            relative_path: XYCE_BUG1797_OWNER_PATH.to_string(),
        };
        runner
            .validate_bug1797_oracle(&deck, Bug1797Role::WrapperOwner, Instant::now())
            .expect("execute BUG1797 alias relation");
    }

    #[test]
    fn bug1797_typed_contract_rejects_level_mutation_and_stale_source() {
        let temporary = tempfile::Builder::new()
            .prefix("rspice-xyce-bug1797-worker-mutation-")
            .tempdir()
            .expect("create worker fixture");
        let path = temporary.path().join(XYCE_BUG1797_LEVEL9_PATH);
        fs::create_dir_all(path.parent().expect("worker parent")).expect("create worker family");
        let canonical_path = corpus_root().join(XYCE_BUG1797_LEVEL9_PATH);
        let canonical = fs::read_to_string(&canonical_path).expect("read canonical worker");
        let mutated = canonical.replace("level=9", "level=49");
        fs::write(&path, &mutated).expect("write level mutation");
        let runner = XyceTestRunner::new(temporary.path(), XyceRunnerConfig::default());
        assert!(
            runner
                .validate_bug1797_worker(Bug1797Role::Level9, &mutated, &path)
                .is_err()
        );

        fs::write(&path, &canonical).expect("restore canonical worker");
        assert!(
            runner
                .validate_bug1797_worker(Bug1797Role::Level9, &mutated, &path)
                .is_err(),
            "planner source must match the provenance-bound caller bytes"
        );
    }

    #[test]
    fn bug1797_provenance_rejects_source_and_role_drift() {
        let (_temporary, deck, runner) = fixture("drift");
        runner
            .validate_bug1797_provenance(&deck, Bug1797Role::WrapperOwner)
            .expect("canonical fixture provenance");

        fs::write(runner.root.join(XYCE_BUG1797_LEVEL9_PATH), "* mutated\n")
            .expect("mutate worker");
        assert!(
            runner
                .validate_bug1797_provenance(&deck, Bug1797Role::WrapperOwner)
                .is_err()
        );

        let (_temporary, deck, runner) = fixture("role-drift");
        let exclusions = runner.root.join(UPSTREAM_EXCLUSIONS_MANIFEST_FILE);
        let text = fs::read_to_string(&exclusions).expect("read exclusions");
        fs::write(
            &exclusions,
            text.replace(XYCE_BUG1797_LEVEL9_CONTRACT, XYCE_BUG1797_LEVEL49_CONTRACT),
        )
        .expect("mutate worker role");
        let mutated_runner = XyceTestRunner::new(&runner.root, XyceRunnerConfig::default());
        assert!(
            mutated_runner
                .validate_bug1797_provenance(&deck, Bug1797Role::WrapperOwner)
                .is_err()
        );
    }

    #[test]
    fn bug1797_oracle_rejects_expired_shared_deadline() {
        let runner = XyceTestRunner::new(corpus_root(), XyceRunnerConfig::default());
        let deck = XyceDeck {
            path: runner.root.join(XYCE_BUG1797_OWNER_PATH),
            section: XyceDeckSection::Netlists,
            relative_path: XYCE_BUG1797_OWNER_PATH.to_string(),
        };
        let expired = Instant::now()
            - Duration::from_millis(
                u64::try_from(runner.config.max_time_per_test_ms.max(1) + 1)
                    .expect("test timeout fits u64"),
            );
        assert!(
            runner
                .validate_bug1797_oracle(&deck, Bug1797Role::WrapperOwner, expired)
                .is_err()
        );
    }

    #[test]
    fn bug1797_intended_relation_rejects_phase_escape_false_pass() {
        let gold = XycePrnTable {
            columns: vec![
                "Index".into(),
                "TIME".into(),
                "V(in)".into(),
                "V(out)".into(),
            ],
            rows: vec![vec![0.0, 1e-9, 1.0, 1.0]],
        };
        let test = XycePrnTable {
            columns: gold.columns.clone(),
            rows: vec![vec![0.0, 1e-9, 2.0, 1.0]],
        };
        let tolerance = XyceFileCompareTolerance {
            absolute: 2e-8,
            relative: 1e-7,
            zero: 1e-20,
        };
        assert!(
            XyceTestRunner::validate_bug1797_intended_relation(&gold, &test, tolerance).is_err()
        );
    }
}
