use super::*;
use rspice_core::analysis::evaluate_tran_measurements;
use rspice_core::netlist::SourceSpec;
use rspice_core::netlist::measure::{
    EdgeType, EventOccurrence, MeasureOperand, MeasurePrintPolicy, MeasureStatement, MeasureType,
};
use std::io::Read as _;

const LABEL: &str = "BUG_412_SON CROSS measurement wrapper";
const FAMILY_DIRECTORY: &str = "Netlists/Certification_Tests/BUG_412_SON";
const FAMILY_PREFIX: &str = "netlists/certification_tests/bug_412_son/";
const OUTPUT_DIRECTORY: &str = "OutputData/Certification_Tests/BUG_412_SON";
const OWNER_NAME: &str = "bug412.cir";
const OWNER_PATH: &str = "Netlists/Certification_Tests/BUG_412_SON/bug412.cir";
const OWNER_RECORD: &str = "netlists/certification_tests/bug_412_son/bug412.cir";
const CONTRACT: &str = "bug412_transient_cross_measurement_wrapper_owner";

const PRETRIM_COMMIT: &str = "80115a9277c0ddb3409acceb3d4e745fd11cddd4";
const PRETRIM_NETLISTS_TREE: &str = "3e34bfaafa890cb2e4457137b6a0e325c8c1e87d";
const UPSTREAM_COMMIT: &str = "d6e278e371ec2f3df1325dcff4552e585bc7ecc1";
const RELEASE_TAG: &str = "Release-7.10.0";
const RELEASE_TAG_OBJECT: &str = "2a339ec3845af0aef99a7e6cc488a41acf64f6ed";

const HISTORICAL_CONTENT_BYTES: usize = 76_022;
const HISTORICAL_STREAM_BYTES: usize = 1_363;
const HISTORICAL_STREAM_SHA256: &str =
    "dae50b004d81845a112bfefa0ef730bcfb261627dfaa74b89bb6854d19945b33";
const HISTORICAL_STREAM_BLAKE3: &str =
    "0890581d7010e377a2220c3a0e0db212c245bde6d312e932682cea29873f309c";
const HISTORICAL: [(&str, usize, &str); 5] = [
    (
        "Netlists/Certification_Tests/BUG_412_SON/Manifest.txt",
        30,
        "862a3f4d908ec5113af165b8bb33d473b35b71b6",
    ),
    (OWNER_PATH, 207, "d786fac736f0c10254664132c7ef39f554cd073f"),
    (
        "Netlists/Certification_Tests/BUG_412_SON/bug412.cir.sh",
        7_346,
        "3f10e72222199f34db0bcbd6c7309330da596297",
    ),
    (
        "Netlists/Certification_Tests/BUG_412_SON/tags",
        35,
        "d643646891ff73ac208f0db17a80388855c82adf",
    ),
    (
        "TestScripts/XyceRegression/Tools.pm",
        68_404,
        "17def57575eb3fd703978fd0634a58d6e679a3de",
    ),
];

const RETAINED_STREAM_BYTES: usize = 120;
const RETAINED_STREAM_SHA256: &str =
    "1fb91255c004ba1bc1517c5bcc15efb9777da8b2c8105fa4c387a62f3076f517";
const RETAINED_STREAM_BLAKE3: &str =
    "72a043449706039eb584880f7bf10c5414f741159c13b6afe9c38feb7f5615c0";
const RETAINED: [(&str, usize, &str); 1] = [(
    OWNER_NAME,
    207,
    "bf35f97b1719e92e44385649d354118ee20317c0f9ba29c65dfcb6e02870dd0f",
)];

const TRAN_STEP: Value = 1.0e-6;
const TRAN_STOP: Value = 10.0e-3;
const SOURCE_OFFSET: Value = 0.5;
const SOURCE_AMPLITUDE: Value = 0.5;
const SOURCE_FREQUENCY: Value = 1.0e3;
const THRESHOLD: Value = 0.5;
const FIRST_CROSSING: Value = 0.5e-3;
const THIRD_CROSSING: Value = 1.5e-3;
const HALF_PERIOD: Value = 0.5e-3;
const EXPECTED_CROSSING_COUNT: usize = 19;
const WRAPPER_ABS_TOL: Value = 1.0e-4;
const WRAPPER_REL_TOL: Value = 0.02;
const WRAPPER_ZERO_TOL: Value = 1.0e-5;
const ANALYTIC_TIME_TOL: Value = 5.0e-9;
const MAX_STREAM_ROWS: usize = 20_000;
const MAX_STREAM_BYTES: usize = 2_000_000;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(super) struct Bug412Role;

impl Bug412Role {
    pub(super) fn for_record(record: &str) -> Option<Self> {
        (XyceTestRunner::normalize_manifest_key(record) == OWNER_RECORD).then_some(Self)
    }

    pub(super) const fn contract(self) -> &'static str {
        CONTRACT
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Bug412CrossDirection {
    Rising,
    Falling,
}

#[derive(Debug, Clone, Copy)]
struct Bug412Crossing {
    time: Value,
    direction: Bug412CrossDirection,
}

impl XyceTestRunner {
    fn bug412_record_stream_identities() -> ((usize, String, String), (usize, String, String)) {
        let historical = HISTORICAL
            .iter()
            .map(|(path, bytes, blob)| {
                format!(
                    "{PRETRIM_COMMIT}\t{PRETRIM_NETLISTS_TREE}\t{UPSTREAM_COMMIT}\t{RELEASE_TAG}\t{RELEASE_TAG_OBJECT}\t{path}\t{bytes}\t{blob}"
                )
            })
            .collect::<Vec<_>>()
            .join("\n");
        let retained = RETAINED
            .iter()
            .map(|(name, bytes, sha)| format!("{FAMILY_DIRECTORY}/{name}\t{bytes}\t{sha}"))
            .collect::<Vec<_>>()
            .join("\n");
        (
            (
                historical.len(),
                format!("{:x}", Sha256::digest(historical.as_bytes())),
                blake3::hash(historical.as_bytes()).to_hex().to_string(),
            ),
            (
                retained.len(),
                format!("{:x}", Sha256::digest(retained.as_bytes())),
                blake3::hash(retained.as_bytes()).to_hex().to_string(),
            ),
        )
    }

    fn validate_bug412_record_streams() -> Result<(), String> {
        let content_bytes = HISTORICAL.iter().map(|record| record.1).sum::<usize>();
        let unique = HISTORICAL
            .iter()
            .map(|record| record.0.to_ascii_lowercase())
            .collect::<BTreeSet<_>>();
        let (historical, retained) = Self::bug412_record_stream_identities();
        if PRETRIM_COMMIT != UPSTREAM_EXCLUSIONS_SOURCE_COMMIT
            || PRETRIM_NETLISTS_TREE != UPSTREAM_EXCLUSIONS_SOURCE_NETLISTS_TREE
            || HISTORICAL.len() != 5
            || unique.len() != HISTORICAL.len()
            || content_bytes != HISTORICAL_CONTENT_BYTES
            || !HISTORICAL.iter().all(|record| {
                record.2.len() == 40 && record.2.bytes().all(|byte| byte.is_ascii_hexdigit())
            })
            || historical.0 != HISTORICAL_STREAM_BYTES
            || historical.1 != HISTORICAL_STREAM_SHA256
            || historical.2 != HISTORICAL_STREAM_BLAKE3
            || RETAINED.len() != 1
            || retained.0 != RETAINED_STREAM_BYTES
            || retained.1 != RETAINED_STREAM_SHA256
            || retained.2 != RETAINED_STREAM_BLAKE3
        {
            return Err(format!(
                "{LABEL} provenance changed: historical={}/{content_bytes}/{:?}; retained={}/{:?}",
                HISTORICAL.len(),
                historical,
                RETAINED.len(),
                retained,
            ));
        }
        Ok(())
    }

    fn read_bug412_directory(
        &self,
        abort: &dyn AbortSignal,
    ) -> Result<BTreeMap<String, Vec<u8>>, String> {
        Self::validate_bug412_record_streams()?;
        let directory = self.root.join(FAMILY_DIRECTORY);
        let metadata = fs::symlink_metadata(&directory)
            .map_err(|error| format!("failed to inspect {LABEL} directory: {error}"))?;
        if metadata.file_type().is_symlink() || !metadata.file_type().is_dir() {
            return Err(format!(
                "{LABEL} directory must be a regular non-symlink directory"
            ));
        }
        let expected = RETAINED
            .into_iter()
            .map(|record| (record.0.to_ascii_lowercase(), record))
            .collect::<BTreeMap<_, _>>();
        let mut observed = BTreeMap::new();
        for entry in fs::read_dir(&directory)
            .map_err(|error| format!("failed to read {LABEL} directory: {error}"))?
        {
            if abort.is_aborted() {
                return Err(format!("{LABEL} source census aborted"));
            }
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
            let Some((expected_name, expected_bytes, expected_sha)) = expected.get(&key).copied()
            else {
                return Err(format!("{LABEL} acquired unexpected member {name:?}"));
            };
            if name != expected_name || observed.contains_key(&key) {
                return Err(format!("{LABEL} member case/census changed: {name:?}"));
            }
            let cap = expected_bytes
                .checked_mul(2)
                .and_then(|value| value.checked_add(3))
                .ok_or_else(|| format!("{LABEL} source-size bound overflowed"))?;
            if metadata.len() > cap as u64 {
                return Err(format!(
                    "{LABEL} member {name:?} exceeds its bounded envelope"
                ));
            }
            let mut bytes = Vec::with_capacity((metadata.len() as usize).min(cap));
            fs::File::open(&path)
                .map_err(|error| format!("failed to open {LABEL} member: {error}"))?
                .take((cap + 1) as u64)
                .read_to_end(&mut bytes)
                .map_err(|error| format!("failed to read {LABEL} member: {error}"))?;
            if bytes.len() > cap {
                return Err(format!("{LABEL} bounded source read grew"));
            }
            let canonical = Self::canonical_lf_text_identity(LABEL, &bytes)?;
            let sha = format!("{:x}", Sha256::digest(&canonical));
            if canonical.len() != expected_bytes || sha != expected_sha {
                return Err(format!(
                    "{LABEL} member {name:?} changed: bytes={}, sha={sha}",
                    canonical.len()
                ));
            }
            observed.insert(key, canonical);
        }
        if observed.len() != expected.len() {
            return Err(format!(
                "{LABEL} retained census changed: expected {}, got {}",
                expected.len(),
                observed.len()
            ));
        }
        Ok(observed)
    }

    fn validate_bug412_provenance(
        &self,
        deck: &XyceDeck,
        abort: &dyn AbortSignal,
    ) -> Result<BTreeMap<String, Vec<u8>>, String> {
        if abort.is_aborted() {
            return Err(format!("{LABEL} provenance validation aborted"));
        }
        if deck.section != XyceDeckSection::Netlists
            || Self::normalize_manifest_key(&deck.relative_path) != OWNER_RECORD
            || Self::normalize_manifest_key(&self.relative_key(&deck.path)) != OWNER_RECORD
            || !Self::same_path(&deck.path, &self.root.join(OWNER_PATH))
        {
            return Err(format!("{LABEL} recognized owner is not canonical"));
        }
        let owners = Self::load_upstream_wrapper_decks(&self.root)
            .into_iter()
            .filter(|record| record.starts_with(FAMILY_PREFIX))
            .collect::<BTreeSet<_>>();
        if owners != BTreeSet::from([OWNER_RECORD.to_string()]) {
            return Err(format!("{LABEL} wrapper ownership changed: {owners:?}"));
        }
        let exclusions = Self::load_upstream_exclusions(&self.root)
            .map_err(|error| format!("{LABEL} exclusions invalid: {error}"))?;
        let family_exclusions = exclusions
            .keys()
            .filter(|record| record.starts_with(FAMILY_PREFIX))
            .collect::<Vec<_>>();
        if !family_exclusions.is_empty() {
            return Err(format!(
                "{LABEL} owner family must not be excluded: {family_exclusions:?}"
            ));
        }
        match fs::symlink_metadata(self.root.join(OUTPUT_DIRECTORY)) {
            Err(error) if error.kind() == std::io::ErrorKind::NotFound => {}
            Err(error) => return Err(format!("failed to inspect {LABEL} OutputData: {error}")),
            Ok(_) => return Err(format!("{LABEL} must not acquire invented numerical gold")),
        }
        self.reject_wrapper_output_artifacts(&self.root.join(OWNER_PATH))
            .map_err(|error| format!("{LABEL} {OWNER_PATH} {error}"))?;
        self.read_bug412_directory(abort)
    }

    fn bug412_nodes_match(actual: &[String], expected: &[&str]) -> bool {
        actual.len() == expected.len()
            && actual
                .iter()
                .zip(expected)
                .all(|(actual, expected)| actual.eq_ignore_ascii_case(expected))
    }

    fn validate_bug412_measure(
        statement: &MeasureStatement,
        expected_name: &str,
        expected_occurrence: isize,
    ) -> bool {
        statement.name.eq_ignore_ascii_case(expected_name)
            && statement.analysis.eq_ignore_ascii_case("TRAN")
            && statement.goal.is_none()
            && statement.tolerance.is_none()
            && statement.default_value.is_none()
            && statement.print_policy == MeasurePrintPolicy::All
            && matches!(&statement.measure_type, MeasureType::When {
                condition,
                from: None,
                to: None,
                td: None,
                minval,
            } if condition.left.eq_ignore_ascii_case("V(1)")
                && matches!(condition.right, MeasureOperand::Constant(value)
                    if value.to_bits() == THRESHOLD.to_bits())
                && condition.occurrence == EventOccurrence {
                    edge: EdgeType::Cross,
                    number: expected_occurrence,
                }
                && minval.to_bits() == 1.0e-12f64.to_bits())
    }

    fn validate_bug412_measure_output(
        output: &rspice_core::netlist::OutputRequest,
        path: &Path,
        expected_name: &str,
        expected_line: usize,
    ) -> bool {
        output.directive == OutputDirectiveKind::Measure
            && output.analysis == Some(OutputAnalysisKind::Tran)
            && output
                .name
                .as_deref()
                .is_some_and(|name| name.eq_ignore_ascii_case(expected_name))
            && output.origin.line == expected_line
            && output
                .origin
                .path
                .as_deref()
                .is_some_and(|origin| Self::same_path(origin, path))
            && output.print_delimiter.is_none()
            && output.print_precision.is_none()
            && output.print_width.is_none()
            && output.operands.is_empty()
            && output.expressions.is_empty()
            && output.dependencies.len() == 1
            && output.dependencies[0].kind == OutputSymbolKind::Node
            && output.dependencies[0].operator.eq_ignore_ascii_case("V")
            && output.dependencies[0].symbol.eq_ignore_ascii_case("1")
            && !output.dependencies[0].expression
    }

    fn validate_bug412_source(
        &self,
        source: &str,
        path: &Path,
    ) -> Result<(XyceStaticTranPlan, Netlist), String> {
        if Self::contains_control_block(source) {
            return Err(format!(
                "{LABEL} does not admit a simulator scripting block"
            ));
        }
        Self::reject_unsupported_source_directives(source)?;
        let netlist = Self::parse_xyce_netlist(source, path)
            .map_err(|error| format!("{LABEL} owner no longer parses: {error}"))?;
        if netlist.elements.len() != 2
            || netlist.title != "Test to see if .measure ... when... cross=x actually works"
            || netlist.analyses.len() != 1
            || netlist.lin_analysis.is_some()
            || !netlist.fft_analyses.is_empty()
            || netlist.output_requests.len() != 3
            || netlist.measurements.len() != 2
            || !netlist.models.is_empty()
            || !netlist.subcircuits.is_empty()
            || !netlist.diagnostics.is_empty()
            || !netlist.data_tables.is_empty()
            || !netlist.initial_conditions.is_empty()
            || netlist.device_initial_conditions.is_some()
            || !netlist.node_sets.is_empty()
            || !netlist.global_nodes.is_empty()
            || !netlist.veriloga_includes.is_empty()
            || !netlist.spef_includes.is_empty()
            || !netlist.control_dispositions.is_empty()
            || netlist.source_text.as_deref() != Some(source)
            || netlist.source_path.as_deref() != Some(path)
            || format!("{:?}", netlist.options)
                != format!("{:?}", rspice_core::netlist::SimulationOptions::default())
            || !netlist.params.all_string_params().is_empty()
            || !netlist.params.all_functions().is_empty()
            || !netlist.params.all_parameter_expressions().is_empty()
            || !netlist.params.all_global_expressions().is_empty()
        {
            return Err(format!(
                "{LABEL} typed envelope changed: elements={:?}, analyses={:?}, outputs={:?}, measures={:?}, diagnostics={:?}",
                netlist.elements,
                netlist.analyses,
                netlist.output_requests,
                netlist.measurements,
                netlist.diagnostics
            ));
        }
        if !matches!(netlist.analyses.as_slice(), [AnalysisCommand::Tran {
            step,
            stop,
            start: None,
            max_step: None,
            uic: false,
            ..
        }] if step.to_bits() == TRAN_STEP.to_bits() && stop.to_bits() == TRAN_STOP.to_bits())
        {
            return Err(format!(
                "{LABEL} typed .TRAN changed: {:?}",
                netlist.analyses
            ));
        }
        let output = &netlist.output_requests[0];
        if output.directive != OutputDirectiveKind::Print
            || output.analysis != Some(OutputAnalysisKind::Tran)
            || output.print_delimiter.as_ref() != Some(&PrintDelimiter::Whitespace)
            || output.print_precision.is_some()
            || output.print_width.is_some()
            || output.name.is_some()
            || output.operands.len() != 1
            || !output.operands[0].eq_ignore_ascii_case("V(1)")
            || !output.expressions.is_empty()
            || output.dependencies.len() != 1
            || output.dependencies[0].kind != OutputSymbolKind::Node
            || !output.dependencies[0].operator.eq_ignore_ascii_case("V")
            || !output.dependencies[0].symbol.eq_ignore_ascii_case("1")
            || output.dependencies[0].expression
            || !matches!(netlist.saves.signals.as_slice(),
                [rspice_core::netlist::SaveSignal::Voltage(node)] if node == "1")
        {
            return Err(format!("{LABEL} typed .PRINT changed: {output:?}"));
        }
        if !Self::validate_bug412_measure(&netlist.measurements[0], "T1", 1)
            || !Self::validate_bug412_measure(&netlist.measurements[1], "T2", 3)
            || !Self::validate_bug412_measure_output(&netlist.output_requests[1], path, "T1", 8)
            || !Self::validate_bug412_measure_output(&netlist.output_requests[2], path, "T2", 9)
        {
            return Err(format!(
                "{LABEL} typed .MEASURE CROSS contract changed: {:?}",
                netlist.measurements
            ));
        }
        Self::validate_bug412_topology(&netlist)?;
        validate_output_symbols(&netlist)
            .map_err(|error| format!("{LABEL} output symbols do not resolve: {error}"))?;
        let tran = Self::single_tran_analysis(&netlist)?;
        let print = Self::single_tran_print_output_request(source)?;
        let plan = XyceStaticTranPlan {
            deck_path: path.to_path_buf(),
            oracle: XyceStaticTranOracle::None,
            source: source.to_string(),
            print: Some(XycePrintRequest {
                probes: print.probes,
            }),
            output_override: false,
            timeint_conststep: false,
            tran,
            steps: Vec::new(),
            contract: XyceStaticTranContract::WrapperStatic,
            wrapper_tolerance: None,
            comparison_mode: XyceStaticTranComparisonMode::Pointwise,
        };
        if plan.deck_path != path
            || plan.source != source
            || plan.print.as_ref().is_none_or(|print| {
                print.probes.len() != 1 || !print.probes[0].eq_ignore_ascii_case("V(1)")
            })
            || plan.tran.step.to_bits() != TRAN_STEP.to_bits()
            || plan.tran.stop.to_bits() != TRAN_STOP.to_bits()
            || plan.tran.start.is_some()
            || plan.tran.max_step.is_some()
            || plan.tran.uic
        {
            return Err(format!("{LABEL} native transient plan changed: {plan:?}"));
        }
        Ok((plan, netlist))
    }

    fn validate_bug412_topology(netlist: &Netlist) -> Result<(), String> {
        let [resistor, source] = netlist.elements.as_slice() else {
            return Err(format!("{LABEL} requires exactly R1 and V1"));
        };
        if resistor.provenance != ElementProvenance::Authored
            || !resistor.name.eq_ignore_ascii_case("R1")
            || !Self::bug412_nodes_match(&resistor.nodes, &["1", "0"])
            || !matches!(&resistor.kind, ElementKind::Resistor {
                value,
                value_expr: None,
                model: None,
                instance_params,
                deferred_params,
            } if value.to_bits() == 1.0f64.to_bits()
                && instance_params.is_empty()
                && deferred_params.is_empty())
            || source.provenance != ElementProvenance::Authored
            || !source.name.eq_ignore_ascii_case("V1")
            || !Self::bug412_nodes_match(&source.nodes, &["1", "0"])
            || !matches!(&source.kind, ElementKind::VoltageSource(SourceSpec::Sin {
                offset,
                amplitude,
                frequency,
                delay,
                damping,
                phase,
            }) if offset.to_bits() == SOURCE_OFFSET.to_bits()
                && amplitude.to_bits() == SOURCE_AMPLITUDE.to_bits()
                && frequency.to_bits() == SOURCE_FREQUENCY.to_bits()
                && delay.to_bits() == 0.0f64.to_bits()
                && damping.to_bits() == 0.0f64.to_bits()
                && phase.to_bits() == 0.0f64.to_bits())
        {
            return Err(format!(
                "{LABEL} exact authored R/SIN topology changed: {:?}",
                netlist.elements
            ));
        }
        Ok(())
    }

    fn validate_bug412_native_circuit(circuit: &rspice_core::CircuitData) -> Result<(), String> {
        let nodes = circuit.node_names_sorted();
        let branches = circuit.branch_names_sorted();
        let sources = circuit.independent_source_names();
        let report = circuit.device_op_report();
        if circuit.has_generated_veriloga_devices()
            || circuit.num_nodes() != 1
            || circuit.matrix_size() != 2
            || circuit.device_count() != 2
            || circuit.get_node_by_name("1") != Some(1)
            || circuit.get_node_by_name("0") != Some(0)
            || circuit.get_branch_by_name("V1") != Some(1)
            || circuit.get_branch_matrix_index(1) != 2
            || nodes.iter().map(String::as_str).ne(["1"])
            || branches.iter().map(String::as_str).ne(["V1"])
            || sources.iter().map(String::as_str).ne(["V1"])
            || !report.entries.is_empty()
            || !report.labels_resolve()
        {
            return Err(format!(
                "{LABEL} production circuit census changed: nodes={nodes:?}/{}, matrix={}, devices={}, branches={branches:?}, sources={sources:?}, report={report:?}, generated_verilog={}",
                circuit.num_nodes(),
                circuit.matrix_size(),
                circuit.device_count(),
                circuit.has_generated_veriloga_devices(),
            ));
        }
        Ok(())
    }

    fn bug412_strict_crossings(
        time: &[Value],
        voltage: &[Value],
    ) -> Result<Vec<Bug412Crossing>, String> {
        if time.len() != voltage.len() || time.len() < 2 {
            return Err(format!("{LABEL} crossing input shape is invalid"));
        }
        let mut crossings = Vec::new();
        for index in 1..time.len() {
            let (t0, t1) = (time[index - 1], time[index]);
            let (v0, v1) = (voltage[index - 1], voltage[index]);
            if !t0.is_finite() || !t1.is_finite() || !v0.is_finite() || !v1.is_finite() || t1 < t0 {
                return Err(format!(
                    "{LABEL} non-finite or unordered crossing row {index}"
                ));
            }
            let direction = if v0 < THRESHOLD && v1 > THRESHOLD {
                Some(Bug412CrossDirection::Rising)
            } else if v0 > THRESHOLD && v1 < THRESHOLD {
                Some(Bug412CrossDirection::Falling)
            } else {
                None
            };
            if let Some(direction) = direction {
                let crossing = if t1 == t0 {
                    t1
                } else {
                    t1 - (t1 - t0) / (v1 - v0) * (v1 - THRESHOLD)
                };
                if !crossing.is_finite() || (t1 > t0 && (crossing <= t0 || crossing >= t1)) {
                    return Err(format!(
                        "{LABEL} invalid strict interpolation at row {index}"
                    ));
                }
                crossings.push(Bug412Crossing {
                    time: crossing,
                    direction,
                });
            }
        }
        Ok(crossings)
    }

    fn validate_bug412_crossing_census(crossings: &[Bug412Crossing]) -> Result<(), String> {
        if crossings.len() != EXPECTED_CROSSING_COUNT {
            return Err(format!(
                "{LABEL} strict crossing census changed: expected {EXPECTED_CROSSING_COUNT}, got {}",
                crossings.len()
            ));
        }
        for (index, crossing) in crossings.iter().enumerate() {
            let expected_time = (index + 1) as Value * HALF_PERIOD;
            let expected_direction = if index % 2 == 0 {
                Bug412CrossDirection::Falling
            } else {
                Bug412CrossDirection::Rising
            };
            if crossing.direction != expected_direction
                || (crossing.time - expected_time).abs() > ANALYTIC_TIME_TOL
            {
                return Err(format!(
                    "{LABEL} crossing {} changed: expected {expected_direction:?} at {expected_time:e}, got {crossing:?}",
                    index + 1
                ));
            }
        }
        Ok(())
    }

    fn validate_bug412_production_measures(
        measurements: &[rspice_core::analysis::MeasureResult],
    ) -> Result<[Value; 2], String> {
        if measurements.len() != 2 {
            return Err(format!(
                "{LABEL} production measure census changed: {:?}",
                measurements
            ));
        }
        let expected = [("T1", FIRST_CROSSING), ("T2", THIRD_CROSSING)];
        let mut values = [0.0; 2];
        for (index, (measurement, (name, analytic))) in
            measurements.iter().zip(expected).enumerate()
        {
            let value = measurement.value.ok_or_else(|| {
                format!("{LABEL} production measure {name} has no value: {measurement:?}")
            })?;
            let event_axis = measurement.event_axis.ok_or_else(|| {
                format!("{LABEL} production measure {name} lost event-axis provenance")
            })?;
            if !measurement.name.eq_ignore_ascii_case(name)
                || !measurement.passed
                || measurement.error.is_some()
                || measurement.expected.is_some()
                || measurement.tolerance.is_some()
                || !value.is_finite()
                || value.to_bits() != event_axis.to_bits()
                || (value - analytic).abs() > ANALYTIC_TIME_TOL
            {
                return Err(format!(
                    "{LABEL} production measure {name} changed: {measurement:?}, analytic={analytic:e}"
                ));
            }
            values[index] = value;
        }
        if (values[1] - values[0] - 1.0e-3).abs() > ANALYTIC_TIME_TOL {
            return Err(format!(
                "{LABEL} CROSS=1/CROSS=3 spacing changed: {:?}",
                values
            ));
        }
        Ok(values)
    }

    fn bug412_wrapper_accepts(measured: Value, calculated: Value) -> bool {
        if !measured.is_finite() || !calculated.is_finite() {
            return false;
        }
        let absolute_error = (measured - calculated).abs();
        let relative_error = (calculated != 0.0).then(|| absolute_error / calculated.abs());
        absolute_error <= WRAPPER_ABS_TOL
            && (measured <= WRAPPER_ZERO_TOL
                || relative_error.is_none_or(|error| error <= WRAPPER_REL_TOL))
    }

    fn bug412_mt0_roundtrip(value: Value) -> Result<Value, String> {
        if !value.is_finite() {
            return Err(format!("{LABEL} cannot serialize a non-finite .mt0 value"));
        }
        let printed = format!("{value:.6e}");
        printed
            .parse::<Value>()
            .map_err(|error| format!("{LABEL} failed to parse serialized .mt0 value: {error}"))
    }

    fn validate_bug412_serialized_wrapper(
        table: &XycePrnTable,
        measured: [Value; 2],
    ) -> Result<Vec<Bug412Crossing>, String> {
        if table.columns.len() != 3
            || table.columns[0] != "Index"
            || !table.columns[1].eq_ignore_ascii_case("TIME")
            || !table.columns[2].eq_ignore_ascii_case("V(1)")
            || table.rows.len() < 100
        {
            return Err(format!(
                "{LABEL} serialized PRN schema changed: {:?}/{}",
                table.columns,
                table.rows.len()
            ));
        }
        let mut time = Vec::with_capacity(table.rows.len());
        let mut voltage = Vec::with_capacity(table.rows.len());
        for (index, row) in table.rows.iter().enumerate() {
            if row.len() != 3 || row.iter().any(|value| !value.is_finite()) {
                return Err(format!("{LABEL} malformed PRN row {index}: {row:?}"));
            }
            if row[0].to_bits() != (index as Value).to_bits() {
                return Err(format!("{LABEL} serialized Index changed at row {index}"));
            }
            time.push(row[1]);
            voltage.push(row[2]);
        }
        if time.first().is_none_or(|value| value.abs() > 1.0e-15)
            || time
                .last()
                .is_none_or(|value| (*value - TRAN_STOP).abs() > 1.0e-10)
            || voltage.iter().copied().fold(Value::INFINITY, Value::min) > 1.0e-3
            || voltage
                .iter()
                .copied()
                .fold(Value::NEG_INFINITY, Value::max)
                < 0.999
        {
            return Err(format!("{LABEL} serialized PRN is incomplete or vacuous"));
        }
        let crossings = Self::bug412_strict_crossings(&time, &voltage)?;
        Self::validate_bug412_crossing_census(&crossings)?;
        for (measurement, crossing) in [measured[0], measured[1]]
            .into_iter()
            .zip([crossings[0], crossings[2]])
        {
            if !Self::bug412_wrapper_accepts(measurement, crossing.time) {
                return Err(format!(
                    "{LABEL} historical strict wrapper rejected measured={measurement:e}, calculated={:e}",
                    crossing.time
                ));
            }
        }
        Ok(crossings)
    }

    fn validate_bug412_physics(result: &TransientResult) -> Result<(), String> {
        if result.time.len() < 100
            || result.step_sizes.len() != result.time.len()
            || result.node_names.len() != 1
            || !result.node_names[0].eq_ignore_ascii_case("1")
            || result.voltages.len() != 1
            || result.voltages[0].len() != result.time.len()
            || result.branch_names.len() != 2
            || !result.branch_names[0].eq_ignore_ascii_case("V1")
            || !result.branch_names[1].eq_ignore_ascii_case("R1")
            || result.branch_currents.len() != 2
            || result
                .branch_currents
                .iter()
                .any(|trace| trace.len() != result.time.len())
            || result
                .time
                .iter()
                .chain(&result.voltages[0])
                .any(|value| !value.is_finite())
            || result.time.windows(2).any(|pair| pair[1] <= pair[0])
        {
            return Err(format!("{LABEL} native transient result shape changed"));
        }
        let mut max_error = 0.0;
        for (&time, &actual) in result.time.iter().zip(&result.voltages[0]) {
            let expected = SOURCE_OFFSET
                + SOURCE_AMPLITUDE * (std::f64::consts::TAU * SOURCE_FREQUENCY * time).sin();
            max_error = Value::max(max_error, (actual - expected).abs());
        }
        if max_error > 2.0e-9 {
            return Err(format!(
                "{LABEL} ideal source waveform detached from analytic sine: max error={max_error:e}"
            ));
        }
        let crossings = Self::bug412_strict_crossings(&result.time, &result.voltages[0])?;
        Self::validate_bug412_crossing_census(&crossings)
    }

    fn validate_bug412_branch_kcl(result: &TransientResult) -> Result<(), String> {
        let source_index = result
            .branch_names
            .iter()
            .position(|name| name.eq_ignore_ascii_case("V1"))
            .ok_or_else(|| format!("{LABEL} lost V1 transient branch current"))?;
        let resistor_index = result
            .branch_names
            .iter()
            .position(|name| name.eq_ignore_ascii_case("R1"))
            .ok_or_else(|| format!("{LABEL} lost R1 transient branch current"))?;
        let source = result
            .branch_currents
            .get(source_index)
            .ok_or_else(|| format!("{LABEL} V1 branch index is invalid"))?;
        let resistor = result
            .branch_currents
            .get(resistor_index)
            .ok_or_else(|| format!("{LABEL} R1 branch index is invalid"))?;
        if source.len() != result.time.len() || resistor.len() != result.time.len() {
            return Err(format!("{LABEL} branch-current traces were not retained"));
        }
        let mut max_kcl: Value = 0.0;
        let mut max_constitutive: Value = 0.0;
        for index in 0..result.time.len() {
            let voltage = result.voltages[0][index];
            max_kcl = max_kcl.max((source[index] + resistor[index]).abs());
            max_constitutive = max_constitutive.max((resistor[index] - voltage).abs());
        }
        if max_kcl > 2.0e-9 || max_constitutive > 2.0e-9 {
            return Err(format!(
                "{LABEL} transient KCL/Ohm law failed: kcl={max_kcl:e}, constitutive={max_constitutive:e}"
            ));
        }
        Ok(())
    }

    fn validate_bug412_fresh_dc_op(
        &self,
        netlist: &Netlist,
        abort: &dyn AbortSignal,
    ) -> Result<(), String> {
        let engine = self.create_xyce_engine();
        let (dc, report) = engine
            .run_dc_op_with_report_and_abort(netlist, abort)
            .map_err(|error| format!("{LABEL} fresh DC operating point failed: {error}"))?;
        let node = dc
            .node_names
            .iter()
            .position(|name| name.eq_ignore_ascii_case("1"))
            .and_then(|index| dc.node_voltages.get(index).copied())
            .ok_or_else(|| format!("{LABEL} fresh DC operating point lost node 1"))?;
        let source = dc
            .branch_names
            .iter()
            .position(|name| name.eq_ignore_ascii_case("V1"))
            .and_then(|index| dc.branch_currents.get(index).copied())
            .ok_or_else(|| format!("{LABEL} fresh DC operating point lost I(V1)"))?;
        if !report.entries.is_empty()
            || !report.labels_resolve()
            || (node - SOURCE_OFFSET).abs() > 2.0e-12
            || (source + SOURCE_OFFSET).abs() > 2.0e-12
            || (source + node).abs() > 2.0e-12
        {
            return Err(format!(
                "{LABEL} fresh DC physics changed: V(1)={node:e}, I(V1)={source:e}"
            ));
        }
        Ok(())
    }

    fn validate_bug412_convergence_quality(engine: &Engine) -> Result<(), String> {
        let quality = engine.convergence_quality();
        if quality.gmin_stepping_count != 0
            || quality.source_stepping_count != 0
            || quality.force_accepted_points != 0
            || !quality.force_accepted_indices.is_empty()
            || quality.failure_diagnostic.is_some()
        {
            return Err(format!(
                "{LABEL} returned a numerically unqualified transient: {quality:?}"
            ));
        }
        Ok(())
    }

    fn run_bug412_native(
        &self,
        plan: &XyceStaticTranPlan,
        authored: &Netlist,
        start: Instant,
        abort: &dyn AbortSignal,
    ) -> Result<(), String> {
        let engine = self
            .create_xyce_static_tran_engine(None, Self::xyce_initial_timestep_for_tran(&plan.tran));
        let circuit = engine
            .build_circuit(authored)
            .map_err(|error| format!("{LABEL} production assembly failed: {error}"))?;
        Self::validate_bug412_native_circuit(&circuit)?;
        let max_step = Self::transient_family_max_step(authored, &plan.tran)?;
        let result = engine
            .run_tran_with_startup_mode_and_abort(
                authored,
                plan.tran.stop,
                max_step,
                TransientStartupMode::OperatingPoint,
                abort,
            )
            .map_err(|error| format!("{LABEL} native transient failed: {error}"))?;
        Self::validate_bug412_convergence_quality(&engine)?;
        Self::validate_bug412_physics(&result)?;
        Self::validate_bug412_branch_kcl(&result)?;

        let measurements = evaluate_tran_measurements(authored, &result);
        let measured = Self::validate_bug412_production_measures(&measurements)?;
        let table = Self::transient_family_result_to_prn_table(plan, authored, &result)
            .map_err(|error| format!("{LABEL} PRN projection failed: {error}"))?;
        let raw = serialize_xyce_prn_sequence(
            std::slice::from_ref(&table),
            &authored.output_requests[0],
            &authored.options,
            XycePrnFooter::Simulation,
            XycePrnLimits::new(MAX_STREAM_ROWS, MAX_STREAM_BYTES),
        )
        .map_err(|error| format!("{LABEL} production PRN serialization failed: {error}"))?;
        let folded_raw = raw.to_ascii_uppercase();
        if raw.len() > MAX_STREAM_BYTES
            || !folded_raw.contains("TIME")
            || !folded_raw.contains("V(1)")
            || !raw.ends_with("End of Xyce(TM) Simulation\n")
        {
            return Err(format!("{LABEL} production PRN framing changed"));
        }
        let parsed = Self::parse_prn_table(&raw)
            .map_err(|error| format!("{LABEL} serialized PRN failed to parse: {error}"))?;
        let serialized_measured = [
            Self::bug412_mt0_roundtrip(measured[0])?,
            Self::bug412_mt0_roundtrip(measured[1])?,
        ];
        Self::validate_bug412_serialized_wrapper(&parsed, serialized_measured)?;
        self.validate_bug412_fresh_dc_op(authored, abort)?;
        if start.elapsed().as_millis() > self.config.max_time_per_test_ms.max(1) {
            return Err(format!("{LABEL} execution exceeded deadline"));
        }
        Ok(())
    }

    pub(super) fn validate_bug412_oracle(
        &self,
        deck: &XyceDeck,
        start: Instant,
    ) -> Result<(), String> {
        let abort = DeadlineAbort::new(start, self.config.max_time_per_test_ms.max(1));
        let before = self.validate_bug412_provenance(deck, &abort)?;
        let owner = before
            .get(&OWNER_NAME.to_ascii_lowercase())
            .ok_or_else(|| format!("{LABEL} lost sealed owner"))?;
        let source = std::str::from_utf8(owner)
            .map_err(|error| format!("{LABEL} owner is not UTF-8: {error}"))?;
        let (plan, netlist) = self.validate_bug412_source(source, &deck.path)?;
        self.run_bug412_native(&plan, &netlist, start, &abort)?;
        if abort.is_aborted() {
            return Err(format!("{LABEL} execution exceeded deadline"));
        }
        let after = self.validate_bug412_provenance(deck, &abort)?;
        if before != after {
            return Err(format!("{LABEL} sealed sources changed during execution"));
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

    fn runner() -> XyceTestRunner {
        XyceTestRunner::new(corpus_root(), XyceRunnerConfig::default())
    }

    fn owner_source() -> String {
        fs::read_to_string(corpus_root().join(OWNER_PATH))
            .unwrap()
            .replace("\r\n", "\n")
    }

    #[test]
    fn bug412_route_is_owner_only() {
        assert_eq!(Bug412Role::for_record(OWNER_PATH), Some(Bug412Role));
        for rejected in [
            "Netlists/Certification_Tests/BUG_412_SON/bug412.cir.sh",
            "Netlists/Certification_Tests/BUG_412_SON/control.cir",
            "Netlists/Certification_Tests/BUG_412_SON/sibling.cir",
            "OutputData/Certification_Tests/BUG_412_SON/bug412.cir.prn",
        ] {
            assert_eq!(
                Bug412Role::for_record(rejected),
                None,
                "accepted {rejected}"
            );
        }
    }

    #[test]
    fn bug412_historical_and_retained_record_streams_are_sealed() {
        XyceTestRunner::validate_bug412_record_streams().unwrap();
        let abort = DeadlineAbort::new(Instant::now(), 30_000);
        assert_eq!(runner().read_bug412_directory(&abort).unwrap().len(), 1);
    }

    #[test]
    fn bug412_typed_source_counterfactuals_fail_closed() {
        let runner = runner();
        let path = corpus_root().join(OWNER_PATH);
        let source = owner_source();
        runner.validate_bug412_source(&source, &path).unwrap();
        for changed in [
            source.replacen("cross=3", "cross=1", 1),
            source.replacen("cross=3", "cross=2", 1),
            source.replacen("v(1)=.5", "v(1)=.4", 1),
            source.replacen("1K)", "2K)", 1),
            source.replacen(".tran 1u 10m", ".tran 2u 10m", 1),
            source.replacen(".print tran v(1)", ".print tran v(1) i(v1)", 1),
            source.replacen(".print tran v(1)", ".print dc v(1)", 1),
            source.replacen("R1 1 0 1", "R1 1 0 2", 1),
            source.replacen(
                ".measure tran t1 when v(1)=.5 cross=1\n.measure tran t2 when v(1)=.5 cross=3",
                ".measure tran t2 when v(1)=.5 cross=3\n.measure tran t1 when v(1)=.5 cross=1",
                1,
            ),
            source.replacen(".end", ".options reltol=1e-4\n.end", 1),
            source.replacen(".end", "Cextra 1 0 1p\n.end", 1),
        ] {
            assert!(runner.validate_bug412_source(&changed, &path).is_err());
        }
    }

    #[test]
    fn bug412_strict_crossing_semantics_cover_ties_directions_and_equal_endpoints() {
        let tied =
            XyceTestRunner::bug412_strict_crossings(&[0.0, 1.0, 1.0, 2.0], &[0.4, 0.4, 0.4, 0.6])
                .unwrap();
        assert_eq!(tied.len(), 1);
        assert_eq!(tied[0].direction, Bug412CrossDirection::Rising);
        assert!(
            XyceTestRunner::bug412_strict_crossings(&[0.0, 1.0, 0.5], &[0.4, 0.4, 0.6]).is_err()
        );
        let tied_straddle =
            XyceTestRunner::bug412_strict_crossings(&[1.0, 1.0], &[0.4, 0.6]).unwrap();
        assert_eq!(tied_straddle.len(), 1);
        assert_eq!(tied_straddle[0].time.to_bits(), 1.0f64.to_bits());
        assert_eq!(tied_straddle[0].direction, Bug412CrossDirection::Rising);

        let directions =
            XyceTestRunner::bug412_strict_crossings(&[0.0, 1.0, 2.0], &[0.4, 0.6, 0.4]).unwrap();
        assert_eq!(directions.len(), 2);
        assert_eq!(directions[0].direction, Bug412CrossDirection::Rising);
        assert_eq!(directions[1].direction, Bug412CrossDirection::Falling);
        assert!(XyceTestRunner::bug412_strict_crossings(
            &[0.0, 1.0, 2.0],
            &[THRESHOLD, 0.6, THRESHOLD],
        )
        .unwrap()
        .is_empty());
    }

    #[test]
    fn bug412_measure_result_counterfactuals_fail_closed() {
        let result = |name: &str, value: Value| rspice_core::analysis::MeasureResult {
            name: name.to_string(),
            value: Some(value),
            error: None,
            passed: true,
            expected: None,
            tolerance: None,
            event_axis: Some(value),
        };
        let valid = [result("T1", FIRST_CROSSING), result("T2", THIRD_CROSSING)];
        XyceTestRunner::validate_bug412_production_measures(&valid).unwrap();
        assert!(
            XyceTestRunner::validate_bug412_production_measures(&[
                result("T1", FIRST_CROSSING),
                result("T2", FIRST_CROSSING),
            ])
            .is_err()
        );
        let mut initial_equality = valid.clone();
        initial_equality[0].value = Some(0.0);
        initial_equality[0].event_axis = Some(0.0);
        initial_equality[1].value = Some(1.0e-3);
        initial_equality[1].event_axis = Some(1.0e-3);
        assert!(XyceTestRunner::validate_bug412_production_measures(&initial_equality).is_err());
        let mut failed = valid.clone();
        failed[0].passed = false;
        failed[0].error = Some("failed".into());
        assert!(XyceTestRunner::validate_bug412_production_measures(&failed).is_err());
        let mut detached = valid;
        detached[1].event_axis = Some(THIRD_CROSSING + 1.0e-6);
        assert!(XyceTestRunner::validate_bug412_production_measures(&detached).is_err());
    }

    #[test]
    fn bug412_historical_wrapper_consumes_the_serialized_prn_roundtrip() {
        let runner = runner();
        let source = owner_source();
        let path = corpus_root().join(OWNER_PATH);
        let (_, netlist) = runner.validate_bug412_source(&source, &path).unwrap();
        let mut rows = Vec::new();
        rows.push(vec![0.0, 0.0, THRESHOLD]);
        let mut time = 5.0e-6;
        while time < TRAN_STOP {
            let voltage = SOURCE_OFFSET
                + SOURCE_AMPLITUDE * (std::f64::consts::TAU * SOURCE_FREQUENCY * time).sin();
            rows.push(vec![rows.len() as Value, time, voltage]);
            time += 1.0e-5;
        }
        rows.push(vec![rows.len() as Value, TRAN_STOP, THRESHOLD]);
        let table = XycePrnTable {
            columns: vec!["Index".into(), "TIME".into(), "V(1)".into()],
            rows,
        };
        let serialize = |table: &XycePrnTable| {
            serialize_xyce_prn_sequence(
                std::slice::from_ref(table),
                &netlist.output_requests[0],
                &netlist.options,
                XycePrnFooter::Simulation,
                XycePrnLimits::new(MAX_STREAM_ROWS, MAX_STREAM_BYTES),
            )
            .unwrap()
        };
        let raw = serialize(&table);
        let parsed = XyceTestRunner::parse_prn_table(&raw).unwrap();
        XyceTestRunner::validate_bug412_serialized_wrapper(
            &parsed,
            [FIRST_CROSSING, THIRD_CROSSING],
        )
        .unwrap();

        let first_straddle = parsed
            .rows
            .windows(2)
            .position(|pair| pair[0][2] > THRESHOLD && pair[1][2] < THRESHOLD)
            .unwrap()
            + 1;
        let mut tied_straddle = parsed.clone();
        tied_straddle.rows[first_straddle - 1][1] = FIRST_CROSSING;
        tied_straddle.rows[first_straddle][1] = FIRST_CROSSING;
        XyceTestRunner::validate_bug412_serialized_wrapper(
            &tied_straddle,
            [FIRST_CROSSING, THIRD_CROSSING],
        )
        .unwrap();

        let mut rounding_erased = table.clone();
        rounding_erased.rows[first_straddle - 1][2] = THRESHOLD + 1.0e-12;
        rounding_erased.rows[first_straddle][2] = THRESHOLD - 1.0e-12;
        let rounding_erased =
            XyceTestRunner::parse_prn_table(&serialize(&rounding_erased)).unwrap();
        assert!(
            XyceTestRunner::validate_bug412_serialized_wrapper(
                &rounding_erased,
                [FIRST_CROSSING, THIRD_CROSSING],
            )
            .is_err()
        );

        let mut wrong_header = parsed.clone();
        wrong_header.columns[2] = "V(2)".into();
        assert!(
            XyceTestRunner::validate_bug412_serialized_wrapper(
                &wrong_header,
                [FIRST_CROSSING, THIRD_CROSSING],
            )
            .is_err()
        );
        let mut decreasing = parsed.clone();
        decreasing.rows[20][1] = decreasing.rows[19][1] - 1.0e-6;
        assert!(
            XyceTestRunner::validate_bug412_serialized_wrapper(
                &decreasing,
                [FIRST_CROSSING, THIRD_CROSSING],
            )
            .is_err()
        );
        let mut vacuous = parsed;
        for row in &mut vacuous.rows {
            row[2] = THRESHOLD;
        }
        assert!(
            XyceTestRunner::validate_bug412_serialized_wrapper(
                &vacuous,
                [FIRST_CROSSING, THIRD_CROSSING],
            )
            .is_err()
        );
    }

    #[test]
    fn bug412_historical_wrapper_predicate_is_strict_and_nonvacuous() {
        assert!(XyceTestRunner::bug412_wrapper_accepts(
            THIRD_CROSSING,
            THIRD_CROSSING
        ));
        assert!(XyceTestRunner::bug412_wrapper_accepts(
            -WRAPPER_ABS_TOL,
            0.0
        ));
        let just_over_absolute = -Value::from_bits(WRAPPER_ABS_TOL.to_bits() + 1);
        assert!(!XyceTestRunner::bug412_wrapper_accepts(
            just_over_absolute,
            0.0
        ));
        let relative_calculated = 1.0e-5;
        let relative_boundary = relative_calculated * (1.0 + WRAPPER_REL_TOL);
        assert!(
            (relative_boundary - relative_calculated).abs() / relative_calculated
                <= WRAPPER_REL_TOL
        );
        assert!(XyceTestRunner::bug412_wrapper_accepts(
            relative_boundary,
            relative_calculated
        ));
        let just_over_relative = Value::from_bits(relative_boundary.to_bits() + 1);
        assert!(!XyceTestRunner::bug412_wrapper_accepts(
            just_over_relative,
            relative_calculated
        ));
        assert!(XyceTestRunner::bug412_wrapper_accepts(5.0e-5, 0.0));
        assert!(XyceTestRunner::bug412_wrapper_accepts(-4.0e-5, 5.0e-5));
        assert!(!XyceTestRunner::bug412_wrapper_accepts(5.0e-5, -4.0e-5));
        assert!(!XyceTestRunner::bug412_wrapper_accepts(
            Value::NAN,
            THIRD_CROSSING
        ));
        assert_eq!(
            XyceTestRunner::bug412_mt0_roundtrip(1.23456789e-3).unwrap(),
            1.234568e-3
        );
    }

    #[test]
    fn bug412_checked_in_owner_executes_complete_oracle() {
        let runner = runner();
        let result = runner.run_test(runner.root.join(OWNER_PATH));
        assert!(result.passed, "BUG412 failed: {:?}", result.error);
        assert_eq!(result.contract, CONTRACT);
    }
}
