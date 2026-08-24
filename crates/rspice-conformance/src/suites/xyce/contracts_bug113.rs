use super::*;

const LABEL: &str = "BUG_113 voltage-switch initial junction state";
const FAMILY_PREFIX: &str = "netlists/certification_tests/bug_113/";
const FAMILY_DIRECTORY: &str = "Netlists/Certification_Tests/BUG_113";
const OUTPUT_DIRECTORY: &str = "OutputData/Certification_Tests/BUG_113";
const EXCLUSION_SOURCE: &str = "Netlists/Certification_Tests/BUG_113/exclude";
const OWNER_PATH: &str = "Netlists/Certification_Tests/BUG_113/VSWITCH.cir";
const NOIC_PATH: &str = "Netlists/Certification_Tests/BUG_113/VSWITCH_NOIC.cir";
const EXPLICIT_ON_PATH: &str = "Netlists/Certification_Tests/BUG_113/VSWITCH_ON_OFF.cir";
const OWNER_RECORD: &str = "netlists/certification_tests/bug_113/vswitch.cir";
const NOIC_RECORD: &str = "netlists/certification_tests/bug_113/vswitch_noic.cir";
const EXPLICIT_ON_RECORD: &str = "netlists/certification_tests/bug_113/vswitch_on_off.cir";
const OWNER_CONTRACT: &str = "bug113_switch_initial_state_jacobian_wrapper_owner";
const NOIC_CONTRACT: &str = "bug113_switch_initial_state_noic_baseline";
const EXPLICIT_ON_CONTRACT: &str = "bug113_switch_initial_state_explicit_on_member";
const UPSTREAM_REGRESSION_COMMIT: &str = "d6e278e371ec2f3df1325dcff4552e585bc7ecc1";
const UPSTREAM_RELEASE_TAG: &str = "Release-7.10.0";
const HISTORICAL_RECORD_COUNT: usize = 8;
const HISTORICAL_RECORD_BYTES: usize = 1_908;
const HISTORICAL_RECORDS_SHA256: &str =
    "6abfbcf4c2c480d28004c07f7e50c4e4820b3a585a009db537321e3f514d53bf";
const HISTORICAL_RECORDS_BLAKE3: &str =
    "8719e09867a3ce38d5affac9da1b53059293709b5fc80e61aa0e94468e1a148b";

const HISTORICAL_ARTIFACTS: [(&str, usize, &str, &str); 8] = [
    (
        "Netlists/Certification_Tests/BUG_113/Manifest.txt",
        83,
        "0dc0ba4e4e32d9899ce48b4b52ae6b553ecd1a725ea76af8438aec5969d79c8d",
        "d70c3461613a383b2ac995c10f8e3246f2d91108622e10a9389688661cc8744d",
    ),
    (
        "Netlists/Certification_Tests/BUG_113/README",
        1_978,
        "2f1e9e1aea04753644d6174dc2455b4520fd5adcb4f42050ef71bb08fd6a666e",
        "5be29c081b89bacaacea7e6faa10d0b904cad42547e4a8e96b97d46fb86d829d",
    ),
    (
        OWNER_PATH,
        0,
        "e3b0c44298fc1c149afbf4c8996fb92427ae41e4649b934ca495991b7852b855",
        "af1349b9f5f9a1a6a0404dea36dcc9499bcb25c9adc112b7cc9a93cae41f3262",
    ),
    (
        "Netlists/Certification_Tests/BUG_113/VSWITCH.cir.sh",
        2_068,
        "03ff802cd1e2e1c1e8c3cfc02ff39aa294de52f74fe1cd26499b6b3bcd335462",
        "d1f642faa8f62acb53962e6c77b645f5f2c50405b8ad2fef287fd3c990aaa80e",
    ),
    (
        NOIC_PATH,
        175,
        "76c5e6df3a8509dab7388a51aa0c75e612676d53e667c5630f69c8207cb721e4",
        "ea71ed1c829ccd17810642855190080e60f0069180f1ea81f9e450d4f53c67f6",
    ),
    (
        EXPLICIT_ON_PATH,
        179,
        "b5301a6d953158954eb83941621875df301a1d0b9c221f3156a0496d0c0c976b",
        "200610d226ad9abba86f394892ae90cbc657d2e38b638ef2505ec0a6bba13123",
    ),
    (
        "Netlists/Certification_Tests/BUG_113/exclude",
        97,
        "402a6818ae9949e6e5340584e74f56deae65735b9556448fdeb9dd7fa91bedc0",
        "c13edc074ae1e6ec5ef8a70de8225881af0d7afa304c4827b7084da8c238450f",
    ),
    (
        "Netlists/Certification_Tests/BUG_113/tags",
        24,
        "2023507e6d6e7f2cae7bf0e1f090ef404db8492c713ffba6102bfb38e8514347",
        "46186bb072b1ee352ccef8d1da4f1a0cdb29fd788dd06270dac5625b1f142415",
    ),
];

const RETAINED_ARTIFACTS: [(&str, usize, &str, &str); 4] = [
    (
        "README",
        1_978,
        "2f1e9e1aea04753644d6174dc2455b4520fd5adcb4f42050ef71bb08fd6a666e",
        "5be29c081b89bacaacea7e6faa10d0b904cad42547e4a8e96b97d46fb86d829d",
    ),
    (
        "VSWITCH.cir",
        0,
        "e3b0c44298fc1c149afbf4c8996fb92427ae41e4649b934ca495991b7852b855",
        "af1349b9f5f9a1a6a0404dea36dcc9499bcb25c9adc112b7cc9a93cae41f3262",
    ),
    (
        "VSWITCH_NOIC.cir",
        175,
        "76c5e6df3a8509dab7388a51aa0c75e612676d53e667c5630f69c8207cb721e4",
        "ea71ed1c829ccd17810642855190080e60f0069180f1ea81f9e450d4f53c67f6",
    ),
    (
        "VSWITCH_ON_OFF.cir",
        179,
        "b5301a6d953158954eb83941621875df301a1d0b9c221f3156a0496d0c0c976b",
        "200610d226ad9abba86f394892ae90cbc657d2e38b638ef2505ec0a6bba13123",
    ),
];

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub(super) enum Bug113Role {
    WrapperOwner,
    NoInitialState,
    ExplicitOn,
}

impl Bug113Role {
    const ALL: [Self; 3] = [Self::WrapperOwner, Self::NoInitialState, Self::ExplicitOn];
    const WORKERS: [Self; 2] = [Self::NoInitialState, Self::ExplicitOn];

    pub(super) fn for_record(relative_path: &str) -> Option<Self> {
        match XyceTestRunner::normalize_manifest_key(relative_path).as_str() {
            OWNER_RECORD => Some(Self::WrapperOwner),
            NOIC_RECORD => Some(Self::NoInitialState),
            EXPLICIT_ON_RECORD => Some(Self::ExplicitOn),
            _ => None,
        }
    }

    pub(super) fn contract(self) -> &'static str {
        match self {
            Self::WrapperOwner => OWNER_CONTRACT,
            Self::NoInitialState => NOIC_CONTRACT,
            Self::ExplicitOn => EXPLICIT_ON_CONTRACT,
        }
    }

    fn path(self) -> &'static str {
        match self {
            Self::WrapperOwner => OWNER_PATH,
            Self::NoInitialState => NOIC_PATH,
            Self::ExplicitOn => EXPLICIT_ON_PATH,
        }
    }

    fn record(self) -> &'static str {
        match self {
            Self::WrapperOwner => OWNER_RECORD,
            Self::NoInitialState => NOIC_RECORD,
            Self::ExplicitOn => EXPLICIT_ON_RECORD,
        }
    }

    fn file_name(self) -> &'static str {
        Path::new(self.path())
            .file_name()
            .and_then(|name| name.to_str())
            .expect("BUG113 constants use UTF-8 file names")
    }

    fn label(self) -> &'static str {
        match self {
            Self::WrapperOwner => "empty wrapper owner",
            Self::NoInitialState => "unmarked switch worker",
            Self::ExplicitOn => "explicit-ON switch worker",
        }
    }

    fn expected_initial_state(self) -> Option<rspice_core::netlist::SwitchState> {
        match self {
            Self::NoInitialState => None,
            Self::ExplicitOn => Some(rspice_core::netlist::SwitchState::On),
            Self::WrapperOwner => unreachable!("the empty owner is not executable"),
        }
    }

    fn expected_jacobians(self) -> usize {
        match self {
            Self::NoInitialState => 3,
            Self::ExplicitOn => 2,
            Self::WrapperOwner => unreachable!("the empty owner is not executable"),
        }
    }
}

impl XyceTestRunner {
    pub(super) fn bug113_historical_oracle_provenance_records() -> Vec<String> {
        let mut records = HISTORICAL_ARTIFACTS
            .into_iter()
            .map(|(path, bytes, sha256, content_blake3)| {
                format!(
                    "{UPSTREAM_REGRESSION_COMMIT}\t{UPSTREAM_RELEASE_TAG}\t{path}\t{bytes}\t{sha256}\t{content_blake3}"
                )
            })
            .collect::<Vec<_>>();
        records.sort();
        records
    }

    pub(super) fn validate_bug113_historical_oracle_provenance() -> Result<(), String> {
        let records = Self::bug113_historical_oracle_provenance_records();
        let stream = records.join("\n");
        let sha256 = format!("{:x}", Sha256::digest(stream.as_bytes()));
        let content_blake3 = blake3::hash(stream.as_bytes()).to_hex().to_string();
        if records.len() != HISTORICAL_RECORD_COUNT
            || stream.len() != HISTORICAL_RECORD_BYTES
            || sha256 != HISTORICAL_RECORDS_SHA256
            || content_blake3 != HISTORICAL_RECORDS_BLAKE3
        {
            return Err(format!(
                "{LABEL} Release-7.10 wrapper provenance changed: records={}/{}, sha256={sha256}, blake3={content_blake3}",
                records.len(),
                stream.len()
            ));
        }
        Ok(())
    }

    fn validate_bug113_source_directory(
        directory: &Path,
    ) -> Result<BTreeMap<String, Vec<u8>>, String> {
        let metadata = fs::symlink_metadata(directory)
            .map_err(|error| format!("failed to inspect {LABEL} directory: {error}"))?;
        if metadata.file_type().is_symlink() || !metadata.file_type().is_dir() {
            return Err(format!(
                "{LABEL} source directory must be a regular non-symlink directory"
            ));
        }
        let expected = RETAINED_ARTIFACTS
            .into_iter()
            .map(|artifact| (artifact.0.to_ascii_lowercase(), artifact))
            .collect::<BTreeMap<_, _>>();
        let mut observed = BTreeMap::new();
        for entry in fs::read_dir(directory)
            .map_err(|error| format!("failed to read {LABEL} directory: {error}"))?
        {
            let entry =
                entry.map_err(|error| format!("failed to inspect {LABEL} member: {error}"))?;
            let path = entry.path();
            let metadata = fs::symlink_metadata(&path)
                .map_err(|error| format!("failed to inspect {}: {error}", path.display()))?;
            if metadata.file_type().is_symlink() || !metadata.file_type().is_file() {
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
            if observed.contains_key(&key) {
                return Err(format!("{LABEL} contains a case-colliding member {name:?}"));
            }
            let Some((expected_name, expected_bytes, expected_sha256, expected_blake3)) =
                expected.get(&key).copied()
            else {
                return Err(format!("{LABEL} acquired unexpected member {name:?}"));
            };
            if name != expected_name {
                return Err(format!(
                    "{LABEL} member case changed: expected {expected_name:?}, got {name:?}"
                ));
            }
            let bytes = fs::read(&path)
                .map_err(|error| format!("failed to read {}: {error}", path.display()))?;
            let canonical = Self::canonical_lf_text_identity(LABEL, &bytes)?;
            let sha256 = format!("{:x}", Sha256::digest(&canonical));
            let content_blake3 = blake3::hash(&canonical).to_hex().to_string();
            if canonical.len() != expected_bytes
                || sha256 != expected_sha256
                || content_blake3 != expected_blake3
            {
                return Err(format!("{LABEL} member {name:?} content changed"));
            }
            observed.insert(key, bytes);
        }
        if observed.len() != expected.len() {
            return Err(format!(
                "{LABEL} source census changed: expected {}, got {}",
                expected.len(),
                observed.len()
            ));
        }
        Ok(observed)
    }

    fn validate_bug113_provenance(
        &self,
        deck: &XyceDeck,
        role: Bug113Role,
    ) -> Result<BTreeMap<Bug113Role, Vec<u8>>, String> {
        Self::validate_bug113_historical_oracle_provenance()?;
        if deck.section != XyceDeckSection::Netlists
            || Self::normalize_manifest_key(&deck.relative_path) != role.record()
            || Self::normalize_manifest_key(&self.relative_key(&deck.path)) != role.record()
            || !Self::same_path(&deck.path, &self.root.join(role.path()))
        {
            return Err(format!(
                "recognized {LABEL} {} is not backed by its canonical path",
                role.label()
            ));
        }

        let owners = self
            .upstream_wrapper_decks
            .iter()
            .filter(|record| record.starts_with(FAMILY_PREFIX))
            .map(String::as_str)
            .collect::<BTreeSet<_>>();
        if owners != BTreeSet::from([OWNER_RECORD]) {
            return Err(format!("{LABEL} wrapper ownership changed: {owners:?}"));
        }

        let exclusions = Self::load_upstream_exclusions(&self.root)
            .map_err(|error| format!("{LABEL} exclusion manifest is invalid: {error}"))?;
        if exclusions.contains_key(OWNER_RECORD) {
            return Err(format!("{LABEL} owner must not be upstream-excluded"));
        }
        let family_exclusions = exclusions
            .iter()
            .filter(|(record, _)| record.starts_with(FAMILY_PREFIX))
            .collect::<BTreeMap<_, _>>();
        if family_exclusions.len() != Bug113Role::WORKERS.len() {
            return Err(format!(
                "{LABEL} requires exactly two independently-qualified workers, found {:?}",
                family_exclusions.keys().collect::<Vec<_>>()
            ));
        }
        for worker in Bug113Role::WORKERS {
            let exclusion = exclusions.get(worker.record()).ok_or_else(|| {
                format!(
                    "{LABEL} {} lost its independent qualification",
                    worker.label()
                )
            })?;
            if exclusion.source != EXCLUSION_SOURCE
                || !matches!(
                    &exclusion.disposition,
                    XyceUpstreamExclusionDisposition::RspiceIndependentlyQualified {
                        expected_contract
                    } if expected_contract == worker.contract()
                )
            {
                return Err(format!(
                    "{LABEL} {} exclusion provenance changed: {exclusion:?}",
                    worker.label()
                ));
            }
        }

        let retained = Self::validate_bug113_source_directory(&self.root.join(FAMILY_DIRECTORY))?;
        if retained
            .get("vswitch.cir")
            .is_none_or(|owner| !owner.is_empty())
        {
            return Err(format!(
                "{LABEL} owner must remain an exact zero-byte wrapper placeholder"
            ));
        }
        let output_directory = self.root.join(OUTPUT_DIRECTORY);
        match fs::symlink_metadata(&output_directory) {
            Err(error) if error.kind() == std::io::ErrorKind::NotFound => {}
            Err(error) => return Err(format!("failed to inspect {LABEL} OutputData: {error}")),
            Ok(_) => return Err(format!("{LABEL} must not acquire numerical gold")),
        }
        for member in Bug113Role::ALL {
            self.reject_wrapper_output_artifacts(&self.root.join(member.path()))
                .map_err(|error| format!("{LABEL} {} {error}", member.label()))?;
        }

        Bug113Role::WORKERS
            .into_iter()
            .map(|worker| {
                retained
                    .get(&worker.file_name().to_ascii_lowercase())
                    .cloned()
                    .map(|bytes| (worker, bytes))
                    .ok_or_else(|| format!("{LABEL} lost {}", worker.file_name()))
            })
            .collect()
    }

    fn bug113_nodes_match(actual: &[String], expected: &[&str]) -> bool {
        actual.len() == expected.len()
            && actual
                .iter()
                .zip(expected)
                .all(|(actual, expected)| actual.eq_ignore_ascii_case(expected))
    }

    fn validate_bug113_worker(
        &self,
        role: Bug113Role,
        source: &str,
        path: &Path,
    ) -> Result<Netlist, String> {
        let plan = self.static_dc_plan_for_source_with_execution_dir(
            path,
            source.to_string(),
            ExpressionDialect::Xyce,
            None,
        )?;
        if plan.deck_path != path
            || plan.execution_dir.is_some()
            || plan.expression_dialect != ExpressionDialect::Xyce
            || plan.parameter_redefinition_policy != ParameterRedefinitionPolicy::UseLast
            || !plan.diagnostics.is_empty()
            || plan.print_format.is_some()
            || plan.dc_data.is_some()
            || !plan.steps.is_empty()
            || !plan.dc.source.eq_ignore_ascii_case("V1")
            || plan.dc.mode != DcSweepMode::Linear
            || plan.dc.start.to_bits() != 5.0f64.to_bits()
            || plan.dc.stop.to_bits() != 5.0f64.to_bits()
            || plan.dc.step.to_bits() != 1.0f64.to_bits()
            || plan.dc.sweep2.is_some()
            || plan.print.probes.len() != 1
            || !plan.print.probes[0].eq_ignore_ascii_case("I(V1)")
        {
            return Err(format!(
                "{LABEL} {} static DC plan changed: {plan:?}",
                role.label()
            ));
        }

        let netlist = Self::parse_xyce_netlist(source, path)
            .map_err(|error| format!("{LABEL} {} no longer parses: {error}", role.label()))?;
        if !netlist.diagnostics.is_empty()
            || netlist.elements.len() != 5
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
        {
            return Err(format!("{LABEL} {} typed envelope changed", role.label()));
        }

        let exact_voltage_source = |element: &rspice_core::netlist::Element,
                                    name: &str,
                                    nodes: &[&str],
                                    expected: Value|
         -> bool {
            element.name.eq_ignore_ascii_case(name)
                && Self::bug113_nodes_match(&element.nodes, nodes)
                && element.provenance == ElementProvenance::Authored
                && matches!(
                    &element.kind,
                    ElementKind::VoltageSource(rspice_core::netlist::SourceSpec::Dc(value))
                        if value.to_bits() == expected.to_bits()
                )
        };
        let exact_resistor = |element: &rspice_core::netlist::Element,
                              name: &str,
                              nodes: &[&str],
                              expected: Value|
         -> bool {
            element.name.eq_ignore_ascii_case(name)
                && Self::bug113_nodes_match(&element.nodes, nodes)
                && element.provenance == ElementProvenance::Authored
                && matches!(
                    &element.kind,
                    ElementKind::Resistor {
                        value,
                        value_expr: None,
                        model: None,
                        instance_params,
                        deferred_params,
                    } if value.to_bits() == expected.to_bits()
                        && instance_params.is_empty()
                        && deferred_params.is_empty()
                )
        };
        if !exact_voltage_source(&netlist.elements[0], "V1", &["1", "0"], 5.0)
            || !netlist.elements[1].name.eq_ignore_ascii_case("S1")
            || !Self::bug113_nodes_match(&netlist.elements[1].nodes, &["1", "2"])
            || netlist.elements[1].provenance != ElementProvenance::Authored
            || !matches!(
                &netlist.elements[1].kind,
                ElementKind::VSwitch {
                    control_pos,
                    control_neg,
                    model,
                    initial_state,
                } if control_pos.eq_ignore_ascii_case("3")
                    && control_neg.eq_ignore_ascii_case("0")
                    && model.eq_ignore_ascii_case("SW")
                    && *initial_state == role.expected_initial_state()
            )
            || !exact_resistor(&netlist.elements[2], "R1", &["2", "0"], 100.0)
            || !exact_voltage_source(&netlist.elements[3], "V2", &["3", "0"], 1.0)
            || !exact_resistor(&netlist.elements[4], "R2", &["3", "0"], 100.0)
        {
            return Err(format!("{LABEL} {} exact topology changed", role.label()));
        }

        let expected_model_params: [(&str, Value); 4] = [
            ("RON", 1.0e-6),
            ("ROFF", 1.0e6),
            ("VON", 1.0),
            ("VOFF", 0.0),
        ];
        let model = &netlist.models[0];
        if !model.name.eq_ignore_ascii_case("SW")
            || !model.model_type.eq_ignore_ascii_case("VSWITCH")
            || model.params.len() != expected_model_params.len()
            || expected_model_params
                .iter()
                .any(|(expected_name, expected_value)| {
                    !model.params.iter().any(|(name, value)| {
                        name.eq_ignore_ascii_case(expected_name)
                            && value.to_bits() == expected_value.to_bits()
                    })
                })
            || !model.expr_params.is_empty()
            || !model.string_params.is_empty()
            || !model.string_vector_params.is_empty()
            || !model.real_vector_params.is_empty()
            || !model.real_vector_expr_params.is_empty()
            || !model.integer_vector_params.is_empty()
        {
            return Err(format!("{LABEL} exact VSWITCH model changed"));
        }
        if !matches!(
            &netlist.analyses[0],
            AnalysisCommand::Dc {
                source,
                start,
                stop,
                step,
                mode: DcSweepMode::Linear,
                sweep2: None,
            } if source.eq_ignore_ascii_case("V1")
                && start.to_bits() == 5.0f64.to_bits()
                && stop.to_bits() == 5.0f64.to_bits()
                && step.to_bits() == 1.0f64.to_bits()
        ) {
            return Err(format!("{LABEL} exact one-point DC command changed"));
        }
        let request = &netlist.output_requests[0];
        if request.directive != OutputDirectiveKind::Print
            || request.analysis != Some(rspice_core::netlist::OutputAnalysisKind::Dc)
            || request.name.is_some()
            || request.dependencies.len() != 1
        {
            return Err(format!("{LABEL} exact .PRINT DC request changed"));
        }
        Ok(netlist)
    }

    fn run_bug113_worker(
        &self,
        role: Bug113Role,
        netlist: &Netlist,
        start: Instant,
    ) -> Result<Value, String> {
        let engine = self.create_dc_engine();
        let abort = DeadlineAbort::new(start, self.config.max_time_per_test_ms.max(1));
        let results = engine
            .run_dc_sweep_with_abort(netlist, "V1", 5.0, 5.0, 1.0, &abort)
            .map_err(|error| format!("{LABEL} {} failed: {error}", role.label()))?;
        let [(sweep, result)] = results.as_slice() else {
            return Err(format!(
                "{LABEL} {} produced {} DC points instead of one",
                role.label(),
                results.len()
            ));
        };
        let current = result
            .branch_current_named("V1")
            .ok_or_else(|| format!("{LABEL} {} lost I(V1)", role.label()))?;
        let quality = engine.convergence_quality();
        if sweep.to_bits() != 5.0f64.to_bits()
            || !current.is_finite()
            || quality.total_iterations != role.expected_jacobians()
            || quality.gmin_stepping_count != 0
            || quality.source_stepping_count != 0
            || quality.force_accepted_points != 0
        {
            return Err(format!(
                "{LABEL} {} historical observation changed: sweep={sweep:.17e}, I(V1)={current:.17e}, quality={quality:?}",
                role.label()
            ));
        }
        Ok(current)
    }

    pub(super) fn validate_bug113_oracle(
        &self,
        deck: &XyceDeck,
        role: Bug113Role,
        start: Instant,
    ) -> Result<(), String> {
        let abort = DeadlineAbort::new(start, self.config.max_time_per_test_ms.max(1));
        if abort.is_aborted() {
            return Err(format!(
                "{LABEL} deadline expired before provenance validation"
            ));
        }
        let sources = self.validate_bug113_provenance(deck, role)?;
        let mut currents = BTreeMap::new();
        for worker in Bug113Role::WORKERS {
            let bytes = sources
                .get(&worker)
                .ok_or_else(|| format!("{LABEL} lost {} source", worker.label()))?;
            let source = std::str::from_utf8(bytes)
                .map_err(|error| format!("{LABEL} {} is not UTF-8: {error}", worker.label()))?;
            let path = self.root.join(worker.path());
            let netlist = self.validate_bug113_worker(worker, source, &path)?;
            currents.insert(worker, self.run_bug113_worker(worker, &netlist, start)?);
        }
        let noic = currents[&Bug113Role::NoInitialState];
        let explicit_on = currents[&Bug113Role::ExplicitOn];
        if (noic - explicit_on).abs() > 1.0e-15
            || (noic - -0.05).abs() > 1.0e-9
            || Bug113Role::NoInitialState.expected_jacobians()
                != Bug113Role::ExplicitOn.expected_jacobians() + 1
        {
            return Err(format!(
                "{LABEL} relational result changed: no-initial={noic:.17e}, explicit-ON={explicit_on:.17e}"
            ));
        }
        if abort.is_aborted() {
            return Err(format!(
                "{LABEL} execution exceeded timeout ({}ms)",
                self.config.max_time_per_test_ms
            ));
        }
        self.validate_bug113_provenance(deck, role)?;
        if abort.is_aborted() {
            return Err(format!(
                "{LABEL} final provenance exceeded timeout ({}ms)",
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

    fn bug113_fixture(label: &str) -> (tempfile::TempDir, XyceDeck, XyceTestRunner) {
        let temporary = tempfile::Builder::new()
            .prefix(&format!("rspice-xyce-bug113-{label}-"))
            .tempdir()
            .expect("create BUG113 fixture root");
        let root = temporary.path().to_path_buf();
        let family = root.join(FAMILY_DIRECTORY);
        fs::create_dir_all(&family).expect("create BUG113 family");
        let canonical = corpus_root().join(FAMILY_DIRECTORY);
        for (name, ..) in RETAINED_ARTIFACTS {
            fs::copy(canonical.join(name), family.join(name))
                .expect("copy canonical BUG113 member");
        }
        fs::write(
            root.join(HARNESS_MANIFEST_FILE),
            format!("{OWNER_PATH}\t{REQUIRES_UPSTREAM_WRAPPER_CONTRACT}\n"),
        )
        .expect("write BUG113 wrapper manifest");
        fs::write(
            root.join(UPSTREAM_EXCLUSIONS_MANIFEST_FILE),
            format!(
                "schema_version\t{UPSTREAM_EXCLUSIONS_SCHEMA_VERSION}\nsource_commit\t{UPSTREAM_EXCLUSIONS_SOURCE_COMMIT}\nsource_netlists_tree\t{UPSTREAM_EXCLUSIONS_SOURCE_NETLISTS_TREE}\n{NOIC_PATH}\t{EXCLUSION_SOURCE}\t{RSPICE_INDEPENDENTLY_QUALIFIED_DISPOSITION}\t{NOIC_CONTRACT}\n{EXPLICIT_ON_PATH}\t{EXCLUSION_SOURCE}\t{RSPICE_INDEPENDENTLY_QUALIFIED_DISPOSITION}\t{EXPLICIT_ON_CONTRACT}\n"
            ),
        )
        .expect("write BUG113 exclusion manifest");
        let deck = XyceDeck {
            path: root.join(OWNER_PATH),
            section: XyceDeckSection::Netlists,
            relative_path: OWNER_PATH.to_string(),
        };
        let runner = XyceTestRunner::new(&root, XyceRunnerConfig::default());
        (temporary, deck, runner)
    }

    #[test]
    fn bug113_historical_provenance_is_exact() {
        XyceTestRunner::validate_bug113_historical_oracle_provenance()
            .expect("Release-7.10 BUG113 provenance remains exact");
    }

    #[test]
    fn bug113_workers_preserve_typed_initial_state_and_exact_jacobians() {
        let root = corpus_root();
        let runner = XyceTestRunner::new(&root, XyceRunnerConfig::default());
        let mut currents = BTreeMap::new();
        for role in Bug113Role::WORKERS {
            let path = root.join(role.path());
            let source = fs::read_to_string(&path).expect("read canonical BUG113 worker");
            let netlist = runner
                .validate_bug113_worker(role, &source, &path)
                .expect("canonical BUG113 worker remains typed");
            currents.insert(
                role,
                runner
                    .run_bug113_worker(role, &netlist, Instant::now())
                    .expect("canonical BUG113 worker reproduces Jacobian count"),
            );
        }
        assert!(
            (currents[&Bug113Role::NoInitialState] - currents[&Bug113Role::ExplicitOn]).abs()
                <= 1.0e-15
        );

        let path = root.join(EXPLICIT_ON_PATH);
        let source = fs::read_to_string(&path).expect("read explicit-ON worker");
        for mutation in [
            source.replace("SW  ON", "SW OFF"),
            source.replace("RON=1u", "RON=2u"),
            source.replace("i(v1)", "v(2)"),
        ] {
            assert!(
                runner
                    .validate_bug113_worker(Bug113Role::ExplicitOn, &mutation, &path)
                    .is_err(),
                "initial-state, model, and observation mutations must fail closed"
            );
        }
    }

    #[test]
    fn bug113_provenance_rejects_family_qualification_and_output_drift() {
        let (_temporary, deck, runner) = bug113_fixture("drift");
        runner
            .validate_bug113_provenance(&deck, Bug113Role::WrapperOwner)
            .expect("canonical BUG113 fixture provenance passes");
        fs::write(
            deck.path
                .parent()
                .expect("BUG113 owner has parent")
                .join("unexpected.out"),
            "stale output\n",
        )
        .expect("write unexpected BUG113 member");
        assert!(
            runner
                .validate_bug113_provenance(&deck, Bug113Role::WrapperOwner)
                .is_err()
        );

        let (_temporary, deck, runner) = bug113_fixture("qualification");
        fs::write(
            runner.root.join(UPSTREAM_EXCLUSIONS_MANIFEST_FILE),
            format!(
                "schema_version\t{UPSTREAM_EXCLUSIONS_SCHEMA_VERSION}\nsource_commit\t{UPSTREAM_EXCLUSIONS_SOURCE_COMMIT}\nsource_netlists_tree\t{UPSTREAM_EXCLUSIONS_SOURCE_NETLISTS_TREE}\n{NOIC_PATH}\t{EXCLUSION_SOURCE}\t{RSPICE_INDEPENDENTLY_QUALIFIED_DISPOSITION}\t{NOIC_CONTRACT}\n"
            ),
        )
        .expect("remove explicit-ON qualification");
        assert!(
            runner
                .validate_bug113_provenance(&deck, Bug113Role::WrapperOwner)
                .is_err()
        );

        let (_temporary, deck, runner) = bug113_fixture("output");
        fs::create_dir_all(runner.root.join(OUTPUT_DIRECTORY))
            .expect("create forbidden BUG113 OutputData");
        assert!(
            runner
                .validate_bug113_provenance(&deck, Bug113Role::WrapperOwner)
                .is_err()
        );
    }

    #[test]
    fn bug113_oracle_rejects_an_expired_deadline() {
        let (_temporary, deck, mut runner) = bug113_fixture("deadline");
        runner.config.max_time_per_test_ms = 1;
        let expired_start = Instant::now()
            .checked_sub(Duration::from_millis(2))
            .expect("construct expired BUG113 deadline");
        let error = runner
            .validate_bug113_oracle(&deck, Bug113Role::WrapperOwner, expired_start)
            .expect_err("an expired BUG113 deadline must fail closed");
        assert!(error.contains("deadline"));
    }
}
