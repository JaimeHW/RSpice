use super::*;
use rspice_core::netlist::SourceSpec;
use std::io::Read as _;

const LABEL: &str = "BUG_1455 model-parameter punctuation relation";

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub(super) enum Bug1455Role {
    EqualsOwner,
    WhitespaceReference,
}

impl Bug1455Role {
    const ALL: [Self; 2] = [Self::EqualsOwner, Self::WhitespaceReference];

    pub(super) fn for_record(record: &str) -> Option<Self> {
        let normalized = XyceTestRunner::normalize_manifest_key(record);
        Self::ALL
            .into_iter()
            .find(|role| role.record() == normalized)
    }

    pub(super) fn contract(self) -> &'static str {
        match self {
            Self::EqualsOwner => XYCE_BUG1455_OWNER_CONTRACT,
            Self::WhitespaceReference => XYCE_BUG1455_REFERENCE_CONTRACT,
        }
    }

    fn path(self) -> &'static str {
        match self {
            Self::EqualsOwner => XYCE_BUG1455_OWNER_PATH,
            Self::WhitespaceReference => XYCE_BUG1455_REFERENCE_PATH,
        }
    }

    fn record(self) -> &'static str {
        match self {
            Self::EqualsOwner => XYCE_BUG1455_OWNER_RECORD,
            Self::WhitespaceReference => XYCE_BUG1455_REFERENCE_RECORD,
        }
    }

    fn file_name(self) -> &'static str {
        self.path()
            .rsplit('/')
            .next()
            .expect("BUG1455 path has a file name")
    }
}

impl XyceTestRunner {
    pub(super) fn bug1455_historical_oracle_provenance_records() -> Vec<String> {
        let mut records = XYCE_BUG1455_HISTORICAL_ARTIFACTS
            .into_iter()
            .map(|(path, bytes, sha256, blake3)| {
                format!(
                    "{XYCE_BUG1455_UPSTREAM_REGRESSION_COMMIT}\t{XYCE_BUG1455_UPSTREAM_RELEASE_TAG}\t{path}\t{bytes}\t{sha256}\t{blake3}"
                )
            })
            .collect::<Vec<_>>();
        records.sort();
        records
    }

    pub(super) fn validate_bug1455_historical_oracle_provenance() -> Result<(), String> {
        let records = Self::bug1455_historical_oracle_provenance_records();
        let stream = records.join("\n");
        let sha256 = format!("{:x}", Sha256::digest(stream.as_bytes()));
        let blake3 = blake3::hash(stream.as_bytes()).to_hex().to_string();
        if records.len() != XYCE_BUG1455_HISTORICAL_RECORD_COUNT
            || stream.len() != XYCE_BUG1455_HISTORICAL_RECORD_BYTES
            || sha256 != XYCE_BUG1455_HISTORICAL_RECORDS_SHA256
            || blake3 != XYCE_BUG1455_HISTORICAL_RECORDS_BLAKE3
        {
            return Err(format!(
                "{LABEL} Release provenance changed: records={}/{}, sha256={sha256}, blake3={blake3}",
                records.len(),
                stream.len()
            ));
        }
        Ok(())
    }

    fn validate_bug1455_directory(&self) -> Result<BTreeMap<String, Vec<u8>>, String> {
        let directory = self.root.join("Netlists/Certification_Tests/BUG_1455");
        let metadata = fs::symlink_metadata(&directory)
            .map_err(|error| format!("failed to inspect {LABEL} family: {error}"))?;
        if metadata.file_type().is_symlink() || !metadata.file_type().is_dir() {
            return Err(format!(
                "{LABEL} family must be a regular non-symlink directory"
            ));
        }
        let expected = XYCE_BUG1455_RETAINED_ARTIFACTS
            .into_iter()
            .map(|artifact| (artifact.0.to_ascii_lowercase(), artifact))
            .collect::<BTreeMap<_, _>>();
        let mut observed = BTreeMap::new();
        let mut records = Vec::new();
        for entry in fs::read_dir(&directory)
            .map_err(|error| format!("failed to read {LABEL} family: {error}"))?
        {
            let entry = entry.map_err(|error| format!("failed to inspect {LABEL}: {error}"))?;
            let path = entry.path();
            let metadata = fs::symlink_metadata(&path)
                .map_err(|error| format!("failed to inspect {}: {error}", path.display()))?;
            if metadata.file_type().is_symlink() || !metadata.file_type().is_file() {
                return Err(format!(
                    "{LABEL} member {} is not a regular file",
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
                return Err(format!("{LABEL} contains a case collision for {name:?}"));
            }
            let Some((expected_name, expected_bytes, expected_sha256, expected_blake3)) =
                expected.get(&key).copied()
            else {
                return Err(format!("{LABEL} acquired unexpected member {name:?}"));
            };
            if name != expected_name {
                return Err(format!("{LABEL} member case changed: {name:?}"));
            }
            let physical_cap = expected_bytes
                .checked_mul(2)
                .and_then(|value| value.checked_add(1))
                .ok_or_else(|| format!("{LABEL} member bound overflow"))?;
            if metadata.len() > physical_cap as u64 {
                return Err(format!("{LABEL} retained member {name:?} is oversized"));
            }
            let file = fs::File::open(&path)
                .map_err(|error| format!("failed to open {LABEL} {name:?}: {error}"))?;
            let mut bytes = Vec::with_capacity(metadata.len() as usize);
            file.take((physical_cap + 1) as u64)
                .read_to_end(&mut bytes)
                .map_err(|error| format!("failed to read {LABEL} {name:?}: {error}"))?;
            if bytes.len() > physical_cap {
                return Err(format!("{LABEL} retained member {name:?} is oversized"));
            }
            let canonical = Self::canonical_lf_text_identity(LABEL, &bytes)?;
            let sha256 = format!("{:x}", Sha256::digest(&canonical));
            let content_blake3 = blake3::hash(&canonical).to_hex().to_string();
            if canonical.len() != expected_bytes
                || sha256 != expected_sha256
                || content_blake3 != expected_blake3
            {
                return Err(format!("{LABEL} retained member {name:?} changed"));
            }
            records.push(format!(
                "{expected_name}\t{expected_bytes}\t{expected_sha256}\t{expected_blake3}"
            ));
            observed.insert(key, bytes);
        }
        records.sort();
        let stream = records.join("\n");
        let sha256 = format!("{:x}", Sha256::digest(stream.as_bytes()));
        let content_blake3 = blake3::hash(stream.as_bytes()).to_hex().to_string();
        if records.len() != XYCE_BUG1455_RETAINED_RECORD_COUNT
            || stream.len() != XYCE_BUG1455_RETAINED_RECORD_BYTES
            || sha256 != XYCE_BUG1455_RETAINED_RECORDS_SHA256
            || content_blake3 != XYCE_BUG1455_RETAINED_RECORDS_BLAKE3
        {
            return Err(format!("{LABEL} retained family census changed"));
        }
        Ok(observed)
    }

    fn validate_bug1455_provenance(
        &self,
        deck: &XyceDeck,
        role: Bug1455Role,
    ) -> Result<BTreeMap<String, Vec<u8>>, String> {
        Self::validate_bug1455_historical_oracle_provenance()?;
        if deck.section != XyceDeckSection::Netlists
            || Self::normalize_manifest_key(&deck.relative_path) != role.record()
            || Self::normalize_manifest_key(&self.relative_key(&deck.path)) != role.record()
            || !Self::same_path(&deck.path, &self.root.join(role.path()))
        {
            return Err(format!(
                "recognized {LABEL} role is not at its canonical path"
            ));
        }
        let prefix = "netlists/certification_tests/bug_1455/";
        let owners = Self::load_upstream_wrapper_decks(&self.root)
            .into_iter()
            .filter(|record| record.starts_with(prefix))
            .collect::<BTreeSet<_>>();
        if owners != BTreeSet::from([XYCE_BUG1455_OWNER_RECORD.to_string()]) {
            return Err(format!("{LABEL} wrapper ownership changed: {owners:?}"));
        }
        let exclusions = Self::load_upstream_exclusions(&self.root)
            .map_err(|error| format!("{LABEL} exclusion manifest is invalid: {error}"))?;
        if exclusions.contains_key(XYCE_BUG1455_OWNER_RECORD) {
            return Err(format!("{LABEL} owner must not be excluded"));
        }
        let family = exclusions
            .iter()
            .filter(|(record, _)| record.starts_with(prefix))
            .collect::<BTreeMap<_, _>>();
        let row = family
            .get(&XYCE_BUG1455_REFERENCE_RECORD.to_string())
            .copied()
            .ok_or_else(|| format!("{LABEL} lost its whitespace reference row"))?;
        if family.len() != 1
            || row.source != XYCE_BUG1455_EXCLUSION_SOURCE
            || !matches!(&row.disposition,
                XyceUpstreamExclusionDisposition::RspiceIndependentlyQualified { expected_contract }
                    if expected_contract == XYCE_BUG1455_REFERENCE_CONTRACT)
        {
            return Err(format!("{LABEL} reference qualification changed"));
        }
        let members = self.validate_bug1455_directory()?;
        for role in Bug1455Role::ALL {
            self.reject_wrapper_output_artifacts(&self.root.join(role.path()))
                .map_err(|error| format!("{LABEL} {} {error}", role.file_name()))?;
        }
        let output = self.root.join("OutputData/Certification_Tests/BUG_1455");
        match fs::symlink_metadata(&output) {
            Err(error) if error.kind() == std::io::ErrorKind::NotFound => {}
            Err(error) => return Err(format!("failed to inspect {LABEL} OutputData: {error}")),
            Ok(_) => return Err(format!("{LABEL} must not acquire invented numerical gold")),
        }
        Ok(members)
    }

    fn validate_bug1455_plan(role: Bug1455Role, plan: &XyceStaticDcPlan) -> Result<(), String> {
        let probes = plan
            .print
            .probes
            .iter()
            .map(|probe| Self::normalize_probe(probe))
            .collect::<Vec<_>>();
        if plan.execution_dir.is_some()
            || plan.dc_data.is_some()
            || plan.print_format.is_some()
            || !plan.steps.is_empty()
            || plan.dc.sweep2.is_some()
            || !matches!(plan.dc.mode, DcSweepMode::Linear)
            || !plan.dc.source.eq_ignore_ascii_case("VDD")
            || plan.dc.start.to_bits() != 0.0f64.to_bits()
            || plan.dc.stop.to_bits() != 5.0f64.to_bits()
            || plan.dc.step.to_bits() != 1.0f64.to_bits()
            || probes != ["v(2)", "i(vmon)", "v(2,3)", "v(2,1)"]
            || !plan.diagnostics.is_empty()
        {
            return Err(format!(
                "{LABEL} {} DC plan changed: {plan:?}",
                role.file_name()
            ));
        }
        Ok(())
    }

    fn bug1455_nodes_match(actual: &[String], expected: &[&str]) -> bool {
        actual.len() == expected.len()
            && actual
                .iter()
                .zip(expected)
                .all(|(actual, expected)| actual.eq_ignore_ascii_case(expected))
    }

    fn exact_params(actual: &[(String, Value)], expected: &[(&str, Value)]) -> bool {
        if actual.len() != expected.len() {
            return false;
        }
        expected.iter().all(|(name, value)| {
            actual.iter().any(|(actual_name, actual_value)| {
                actual_name.eq_ignore_ascii_case(name) && actual_value.to_bits() == value.to_bits()
            })
        })
    }

    fn validate_resistor(
        element: &rspice_core::netlist::Element,
        name: &str,
        nodes: [&str; 2],
        resistance: Value,
    ) -> Result<(), String> {
        if element.provenance != ElementProvenance::Authored
            || !element.name.eq_ignore_ascii_case(name)
            || !Self::bug1455_nodes_match(&element.nodes, &nodes)
            || !matches!(&element.kind, ElementKind::Resistor {
                value,
                value_expr: None,
                model: None,
                instance_params,
                deferred_params,
            } if value.to_bits() == resistance.to_bits()
                && instance_params.is_empty()
                && deferred_params.is_empty())
        {
            return Err(format!("{LABEL} resistor {name} changed: {element:?}"));
        }
        Ok(())
    }

    fn validate_bug1455_netlist(role: Bug1455Role, netlist: &Netlist) -> Result<(), String> {
        if netlist.elements.len() != 6
            || netlist.models.len() != 1
            || netlist.output_requests.len() != 1
            || !matches!(netlist.analyses.as_slice(), [AnalysisCommand::Dc {
                source,
                start,
                stop,
                step,
                mode: DcSweepMode::Linear,
                sweep2: None,
            }] if source.eq_ignore_ascii_case("VDD")
                && start.to_bits() == 0.0f64.to_bits()
                && stop.to_bits() == 5.0f64.to_bits()
                && step.to_bits() == 1.0f64.to_bits())
            || !netlist.subcircuits.is_empty()
            || !netlist.data_tables.is_empty()
            || !netlist.measurements.is_empty()
            || !netlist.initial_conditions.is_empty()
            || netlist.device_initial_conditions.is_some()
            || !netlist.node_sets.is_empty()
            || !netlist.global_nodes.is_empty()
            || !netlist.veriloga_includes.is_empty()
            || !netlist.spef_includes.is_empty()
            || !netlist.diagnostics.is_empty()
            || !netlist.params.all_params().is_empty()
            || !netlist.params.all_string_params().is_empty()
            || !netlist.params.all_functions().is_empty()
        {
            return Err(format!(
                "{LABEL} {} typed envelope changed",
                role.file_name()
            ));
        }
        let model = &netlist.models[0];
        if !model.name.eq_ignore_ascii_case("PFET")
            || !model.model_type.eq_ignore_ascii_case("PMOS")
            || !Self::exact_params(
                &model.params,
                &[("LEVEL", 1.0), ("KP", 25.0 * 1e-6), ("VTO", -0.8)],
            )
            || !model.expr_params.is_empty()
            || !model.string_params.is_empty()
            || !model.string_vector_params.is_empty()
            || !model.real_vector_params.is_empty()
            || !model.real_vector_expr_params.is_empty()
            || !model.integer_vector_params.is_empty()
        {
            return Err(format!("{LABEL} PFET model changed: {model:?}"));
        }

        let elements = netlist
            .elements
            .iter()
            .map(|element| (element.name.to_ascii_lowercase(), element))
            .collect::<BTreeMap<_, _>>();
        if elements.keys().map(String::as_str).collect::<BTreeSet<_>>()
            != BTreeSet::from(["vdd", "r1", "r2", "rd", "vmon", "m1"])
        {
            return Err(format!("{LABEL} element inventory changed"));
        }
        Self::validate_resistor(elements["r1"], "R1", ["2", "1"], 50e3)?;
        Self::validate_resistor(elements["r2"], "R2", ["1", "0"], 50e3)?;
        Self::validate_resistor(elements["rd"], "RD", ["4", "0"], 7.5e3)?;
        for (name, nodes, expected) in [("vdd", ["2", "0"], 5.0f64), ("vmon", ["3", "4"], 0.0f64)] {
            let source = elements[name];
            if source.provenance != ElementProvenance::Authored
                || !Self::bug1455_nodes_match(&source.nodes, &nodes)
                || !matches!(source.kind, ElementKind::VoltageSource(SourceSpec::Dc(value))
                    if value.to_bits() == expected.to_bits())
            {
                return Err(format!("{LABEL} voltage source {name} changed"));
            }
        }
        let mosfet = elements["m1"];
        let ElementKind::Mosfet {
            model,
            mos_type: _,
            compact_syntax: false,
            instance_params,
            deferred_params,
        } = &mosfet.kind
        else {
            return Err(format!("{LABEL} M1 kind changed: {mosfet:?}"));
        };
        if mosfet.provenance != ElementProvenance::Authored
            || !Self::bug1455_nodes_match(&mosfet.nodes, &["3", "1", "2", "2"])
            || !model.eq_ignore_ascii_case("PFET")
            || !Self::exact_params(instance_params, &[("L", 10.0 * 1e-6), ("W", 160.0 * 1e-6)])
            || !deferred_params.is_empty()
        {
            return Err(format!("{LABEL} M1 topology or geometry changed"));
        }

        let request = &netlist.output_requests[0];
        if request.directive != OutputDirectiveKind::Print
            || request.analysis != Some(rspice_core::netlist::OutputAnalysisKind::Dc)
            || request.name.is_some()
            || request.print_delimiter != Some(PrintDelimiter::Whitespace)
            || !request.expressions.is_empty()
            || request
                .dependencies
                .iter()
                .any(|dependency| dependency.expression)
        {
            return Err(format!("{LABEL} typed .PRINT request changed: {request:?}"));
        }
        let devices = request
            .dependencies
            .iter()
            .filter(|dependency| dependency.kind == OutputSymbolKind::Device)
            .collect::<Vec<_>>();
        let nodes = request
            .dependencies
            .iter()
            .filter(|dependency| dependency.kind == OutputSymbolKind::Node)
            .map(|dependency| dependency.symbol.to_ascii_lowercase())
            .collect::<BTreeSet<_>>();
        if devices.len() != 1
            || !devices[0].operator.eq_ignore_ascii_case("I")
            || !devices[0].symbol.eq_ignore_ascii_case("VMON")
            || nodes != BTreeSet::from(["1".to_string(), "2".to_string(), "3".to_string()])
        {
            return Err(format!(
                "{LABEL} typed .PRINT dependencies changed: {request:?}"
            ));
        }
        Ok(())
    }

    fn bug1455_analytic_row(vdd: Value) -> [Value; 5] {
        let vsg = vdd * 0.5;
        let overdrive = (vsg - 0.8).max(0.0);
        let beta = (25.0 * 1e-6) * ((160.0 * 1e-6) / (10.0 * 1e-6));
        let rd = 7.5e3;
        let (current, vsd) = if overdrive == 0.0 {
            (0.0, vdd)
        } else {
            let saturation_current = 0.5 * beta * overdrive * overdrive;
            let saturation_vsd = vdd - rd * saturation_current;
            if saturation_vsd >= overdrive {
                (saturation_current, saturation_vsd)
            } else {
                let scaled = rd * beta;
                let linear = scaled * overdrive + 1.0;
                let discriminant = linear * linear - 2.0 * scaled * vdd;
                let vsd = (linear - discriminant.sqrt()) / scaled;
                ((vdd - vsd) / rd, vsd)
            }
        };
        [vdd, current, vsd, vdd * 0.5, vdd - vsd]
    }

    fn validate_bug1455_table(role: Bug1455Role, table: &XycePrnTable) -> Result<(), String> {
        const COLUMNS: [&str; 5] = ["Index", "V(2)", "I(VMON)", "V(2,3)", "V(2,1)"];
        if table.columns.len() != COLUMNS.len()
            || table
                .columns
                .iter()
                .zip(COLUMNS)
                .any(|(actual, expected)| !actual.eq_ignore_ascii_case(expected))
            || table.rows.len() != 6
        {
            return Err(format!(
                "{LABEL} {} table shape changed: {table:?}",
                role.file_name()
            ));
        }
        for (index, row) in table.rows.iter().enumerate() {
            if row.len() != COLUMNS.len()
                || row.iter().any(|value| !value.is_finite())
                || row[0].to_bits() != (index as Value).to_bits()
            {
                return Err(format!(
                    "{LABEL} {} row {index} is malformed: {row:?}",
                    role.file_name()
                ));
            }
            let expected = Self::bug1455_analytic_row(index as Value);
            for (column, (actual, expected)) in
                row[1..].iter().zip(expected[..4].iter()).enumerate()
            {
                let absolute = if column == 1 { 5e-10 } else { 5e-6 };
                let relative = expected.abs() * 2e-5;
                if (*actual - *expected).abs() > absolute + relative {
                    return Err(format!(
                        "{LABEL} {} row {index} column {} violated MOS1/load-line oracle: actual={actual}, expected={expected}",
                        role.file_name(),
                        column + 1,
                    ));
                }
            }
        }
        Ok(())
    }

    fn bug1455_plan(&self, role: Bug1455Role) -> Result<XyceStaticDcPlan, String> {
        let plan =
            self.static_dc_plan_for_path(&self.root.join(role.path()), ExpressionDialect::Xyce)?;
        Self::validate_bug1455_plan(role, &plan)?;
        Ok(plan)
    }

    pub(super) fn validate_bug1455_oracle(
        &self,
        deck: &XyceDeck,
        role: Bug1455Role,
        start: Instant,
    ) -> Result<(), String> {
        let abort = DeadlineAbort::new(start, self.config.max_time_per_test_ms.max(1));
        if abort.is_aborted() {
            return Err(format!(
                "{LABEL} exceeded its shared deadline before provenance"
            ));
        }
        let members = self.validate_bug1455_provenance(deck, role)?;
        let run = |member_role: Bug1455Role| {
            let plan = self.bug1455_plan(member_role)?;
            let source = members
                .get(&member_role.file_name().to_ascii_lowercase())
                .ok_or_else(|| format!("{LABEL} lost {}", member_role.file_name()))?;
            if plan.source.as_bytes() != source.as_slice() {
                return Err(format!(
                    "{LABEL} {} source changed between reads",
                    member_role.file_name()
                ));
            }
            let (netlist, results) = self.run_static_dc_results(&plan, start).map_err(|error| {
                format!(
                    "{LABEL} {} execution failed: {error}",
                    member_role.file_name()
                )
            })?;
            Self::validate_bug1455_netlist(member_role, &netlist)?;
            let table = self.dc_results_to_prn_table(&plan, &netlist, &results)?;
            Self::validate_bug1455_table(member_role, &table)?;
            Ok::<_, String>(table)
        };

        let owner = run(Bug1455Role::EqualsOwner)?;
        if abort.is_aborted() {
            return Err(format!(
                "{LABEL} exceeded its shared deadline between independent runs"
            ));
        }
        let reference = run(Bug1455Role::WhitespaceReference)?;
        let mismatches =
            self.compare_serialized_default_prn_tables_case_insensitive(&owner, &reference)?;
        if !mismatches.is_empty() {
            return Err(format!(
                "{LABEL} failed its case-insensitive byte relation: {mismatches:?}"
            ));
        }
        self.validate_bug1455_provenance(deck, role)?;
        if abort.is_aborted() {
            return Err(format!("{LABEL} exceeded its shared deadline"));
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
            .prefix(&format!("rspice-xyce-bug1455-{label}-"))
            .tempdir()
            .expect("create BUG1455 fixture");
        let root = temporary.path();
        let family = root.join("Netlists/Certification_Tests/BUG_1455");
        fs::create_dir_all(&family).expect("create BUG1455 family");
        let canonical = corpus_root().join("Netlists/Certification_Tests/BUG_1455");
        for (name, ..) in XYCE_BUG1455_RETAINED_ARTIFACTS {
            fs::copy(canonical.join(name), family.join(name)).expect("copy BUG1455 member");
        }
        fs::write(
            root.join(HARNESS_MANIFEST_FILE),
            format!(
                "{}\t{}\n",
                XYCE_BUG1455_OWNER_PATH, REQUIRES_UPSTREAM_WRAPPER_CONTRACT
            ),
        )
        .expect("write wrapper manifest");
        fs::write(
            root.join(UPSTREAM_EXCLUSIONS_MANIFEST_FILE),
            format!(
                "schema_version\t{UPSTREAM_EXCLUSIONS_SCHEMA_VERSION}\nsource_commit\t{UPSTREAM_EXCLUSIONS_SOURCE_COMMIT}\nsource_netlists_tree\t{UPSTREAM_EXCLUSIONS_SOURCE_NETLISTS_TREE}\n{}\t{}\t{}\t{}\n",
                XYCE_BUG1455_REFERENCE_PATH,
                XYCE_BUG1455_EXCLUSION_SOURCE,
                RSPICE_INDEPENDENTLY_QUALIFIED_DISPOSITION,
                XYCE_BUG1455_REFERENCE_CONTRACT,
            ),
        )
        .expect("write exclusion manifest");
        let runner = XyceTestRunner::new(root, XyceRunnerConfig::default());
        let deck = XyceDeck {
            path: root.join(XYCE_BUG1455_OWNER_PATH),
            section: XyceDeckSection::Netlists,
            relative_path: XYCE_BUG1455_OWNER_PATH.to_string(),
        };
        (temporary, deck, runner)
    }

    #[test]
    fn bug1455_historical_provenance_is_exact() {
        XyceTestRunner::validate_bug1455_historical_oracle_provenance()
            .expect("BUG1455 Release provenance remains exact");
    }

    #[test]
    fn bug1455_both_roles_execute_the_model_punctuation_relation() {
        let root = corpus_root();
        let runner = XyceTestRunner::new(&root, XyceRunnerConfig::default());
        for role in Bug1455Role::ALL {
            let deck = XyceDeck {
                section: XyceDeckSection::Netlists,
                path: root.join(role.path()),
                relative_path: role.path().to_string(),
            };
            runner
                .validate_bug1455_oracle(&deck, role, Instant::now())
                .unwrap_or_else(|error| panic!("{} failed: {error}", role.file_name()));
        }
    }

    #[test]
    fn bug1455_analytic_oracle_rejects_counterfactuals() {
        let mut table = XycePrnTable {
            columns: vec![
                "Index".into(),
                "V(2)".into(),
                "I(VMON)".into(),
                "V(2,3)".into(),
                "V(2,1)".into(),
            ],
            rows: (0..=5)
                .map(|index| {
                    let expected = XyceTestRunner::bug1455_analytic_row(index as Value);
                    vec![
                        index as Value,
                        expected[0],
                        expected[1],
                        expected[2],
                        expected[3],
                    ]
                })
                .collect(),
        };
        XyceTestRunner::validate_bug1455_table(Bug1455Role::EqualsOwner, &table)
            .expect("canonical analytic table");
        table.rows[5][2] *= 0.5;
        assert!(XyceTestRunner::validate_bug1455_table(Bug1455Role::EqualsOwner, &table).is_err());
    }

    #[test]
    fn bug1455_source_mutations_fail_typed_admission() {
        let source = fs::read_to_string(corpus_root().join(XYCE_BUG1455_OWNER_PATH))
            .expect("read BUG1455 owner");
        for mutated in [
            source.replace("KP=25U", "KP=20U"),
            source.replace("VTO=-0.8V", "VTO=-0.7V"),
            source.replace("RD 4 0 7.5K", "RD 4 0 8K"),
            source.replace("M1 3 1 2 2", "M1 4 1 2 2"),
            source.replace("W=160U", "W=80U"),
            source.replace(".dc VDD 0 5 1", ".dc VDD 0 4 1"),
            source.replace("I(VMON)", "I(VDD)"),
        ] {
            let netlist = XyceTestRunner::parse_xyce_netlist(
                &mutated,
                &corpus_root().join(XYCE_BUG1455_OWNER_PATH),
            )
            .expect("mutated BUG1455 source remains parseable");
            assert!(
                XyceTestRunner::validate_bug1455_netlist(Bug1455Role::EqualsOwner, &netlist)
                    .is_err(),
                "mutation escaped typed admission: {mutated}"
            );
        }
    }

    #[test]
    fn bug1455_expired_deadline_fails_closed() {
        let root = corpus_root();
        let runner = XyceTestRunner::new(
            &root,
            XyceRunnerConfig {
                max_time_per_test_ms: 1,
                ..XyceRunnerConfig::default()
            },
        );
        let deck = XyceDeck {
            section: XyceDeckSection::Netlists,
            path: root.join(XYCE_BUG1455_OWNER_PATH),
            relative_path: XYCE_BUG1455_OWNER_PATH.to_string(),
        };
        assert!(
            runner
                .validate_bug1455_oracle(
                    &deck,
                    Bug1455Role::EqualsOwner,
                    Instant::now() - Duration::from_secs(1),
                )
                .is_err()
        );
    }

    #[test]
    fn bug1455_provenance_rejects_source_role_output_and_census_drift() {
        let (_temporary, deck, runner) = fixture("source");
        runner
            .validate_bug1455_provenance(&deck, Bug1455Role::EqualsOwner)
            .expect("canonical BUG1455 fixture");
        fs::write(runner.root.join(XYCE_BUG1455_REFERENCE_PATH), "* changed\n")
            .expect("mutate reference");
        assert!(
            runner
                .validate_bug1455_provenance(&deck, Bug1455Role::EqualsOwner)
                .is_err()
        );

        let (_temporary, deck, runner) = fixture("owner");
        fs::write(runner.root.join(HARNESS_MANIFEST_FILE), "").expect("remove wrapper owner");
        assert!(
            runner
                .validate_bug1455_provenance(&deck, Bug1455Role::EqualsOwner)
                .is_err()
        );

        let (_temporary, deck, runner) = fixture("exclusion");
        let manifest = runner.root.join(UPSTREAM_EXCLUSIONS_MANIFEST_FILE);
        let text = fs::read_to_string(&manifest).expect("read exclusions");
        fs::write(
            &manifest,
            text.replace(
                RSPICE_INDEPENDENTLY_QUALIFIED_DISPOSITION,
                UPSTREAM_EXCLUDED_DISPOSITION,
            ),
        )
        .expect("change exclusion disposition");
        assert!(
            runner
                .validate_bug1455_provenance(&deck, Bug1455Role::EqualsOwner)
                .is_err()
        );

        let (_temporary, deck, runner) = fixture("extra");
        fs::write(
            runner
                .root
                .join("Netlists/Certification_Tests/BUG_1455/extra.cir"),
            ".end\n",
        )
        .expect("add family member");
        assert!(
            runner
                .validate_bug1455_provenance(&deck, Bug1455Role::EqualsOwner)
                .is_err()
        );

        let (_temporary, deck, runner) = fixture("output");
        fs::create_dir_all(runner.root.join("OutputData/Certification_Tests/BUG_1455"))
            .expect("invent output directory");
        assert!(
            runner
                .validate_bug1455_provenance(&deck, Bug1455Role::EqualsOwner)
                .is_err()
        );
    }
}
