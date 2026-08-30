use super::*;
use rspice_core::analysis::HbResult;
use rspice_core::netlist::{SourceSpec, XyceHbPreconditioner};
use std::cmp::Ordering;
use std::io::Read as _;

const LABEL: &str = "BUG_340 stepped harmonic-balance wrapper";
const FAMILY_DIRECTORY: &str = "Netlists/Certification_Tests/BUG_340";
const FAMILY_PREFIX: &str = "netlists/certification_tests/bug_340/";
const OWNER_NAME: &str = "stepRC-hb.cir";
const OWNER_PATH: &str = "Netlists/Certification_Tests/BUG_340/stepRC-hb.cir";
const OUTPUT_DIRECTORY: &str = "OutputData/Certification_Tests/BUG_340";
const OUTPUT_NAME: &str = "stepRC-hb.cir.HB.FD.prn";
const OUTPUT_PATH: &str = "OutputData/Certification_Tests/BUG_340/stepRC-hb.cir.HB.FD.prn";
const CONTRACT: &str = "bug340_stepped_hb_fd_wrapper_owner";
const PRETRIM_COMMIT: &str = "80115a9277c0ddb3409acceb3d4e745fd11cddd4";
const UPSTREAM_COMMIT: &str = "d6e278e371ec2f3df1325dcff4552e585bc7ecc1";
const RELEASE_TAG: &str = "Release-7.10.0";
const RELEASE_TAG_OBJECT: &str = "2a339ec3845af0aef99a7e6cc488a41acf64f6ed";
const FUNDAMENTAL_HZ: Value = 100_000.0;
const CAPACITANCE_F: Value = 2.0e-6;
const HARMONICS: usize = 5;
const STEP_RESISTANCES: [Value; 5] = [1.0e3, 2.0e3, 3.0e3, 4.0e3, 5.0e3];
const AC_ABSOLUTE_TOLERANCE: Value = 1.0e-6;
const AC_RELATIVE_TOLERANCE: Value = 1.0e-3;
const AC_ZERO_TOLERANCE: Value = 1.0e-14;
const AC_FREQUENCY_RELATIVE_TOLERANCE: Value = 1.0e-6;

type Bug340RetainedFiles = BTreeMap<String, Vec<u8>>;
type Bug340Provenance = (Bug340RetainedFiles, Vec<u8>);

const HISTORICAL_CONTENT_BYTES: usize = 24_170;
const HISTORICAL_STREAM_BYTES: usize = 1_387;
const HISTORICAL_STREAM_SHA256: &str =
    "e4d2b8311099a27b3837522887d28730e26af8293cc310c0f754aa0f22b4e619";
const HISTORICAL_STREAM_BLAKE3: &str =
    "0abe6ab4e09708d553180d6c44eb16fd4089a586f32a552e6b5a7a6160edccf3";
const HISTORICAL: [(&str, usize, &str); 6] = [
    (
        "Netlists/Certification_Tests/BUG_340/Manifest.txt",
        36,
        "033147e5650077a834c0bc2cce2bec8a3bebb6d3",
    ),
    (OWNER_PATH, 242, "9f31cea8e7573e070a34243b9297a4823eb31e9e"),
    (
        "Netlists/Certification_Tests/BUG_340/stepRC-hb.cir.sh",
        2_339,
        "1f01b0058223b02fcfe74d9d25e88a110b2c0d4c",
    ),
    (
        "Netlists/Certification_Tests/BUG_340/tags",
        50,
        "ea9e5cfc964b84c778731ed54cb86caec6fd1104",
    ),
    (
        "OutputData/Certification_Tests/BUG_340/stepRC-hb.cir.HB.FD.prn",
        7_195,
        "8bdc3ad577aa17edb4da9082ecf9f0c5a76ff7a8",
    ),
    (
        "TestScripts/ACComparator.pl",
        14_308,
        "53f8d082a343273b4d71369e974a482b96e7b1cd",
    ),
];

const RETAINED_STREAM_BYTES: usize = 252;
const RETAINED_STREAM_SHA256: &str =
    "35f91634a4d1883c068de4a487386a27a578e6c27021820e8509ff7a2565cded";
const RETAINED_STREAM_BLAKE3: &str =
    "2fd8013d1362058d5c639d9376c7e9bf70c05417b598bf6db508ea413b0908eb";
const RETAINED: [(&str, usize, &str); 2] = [
    (
        OWNER_PATH,
        242,
        "6281669193af40a959ad8d6cbbe50da7815a0d0bc2d6e2062cb1b2254e91350e",
    ),
    (
        OUTPUT_PATH,
        7_195,
        "1701b615325b36b6c2160e40d07b94cba0f4e9547a99f59820dea32e21fdc7c5",
    ),
];

const OUTPUT_COLUMNS: [&str; 8] = [
    "Index",
    "FREQ",
    "Re(V(1))",
    "Im(V(1))",
    "Re(V(2))",
    "Im(V(2))",
    "Re(I(V1))",
    "Im(I(V1))",
];

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(super) struct Bug340Role;

impl Bug340Role {
    pub(super) fn for_record(relative_path: &str) -> Option<Self> {
        (XyceTestRunner::normalize_manifest_key(relative_path)
            == XyceTestRunner::normalize_manifest_key(OWNER_PATH))
        .then_some(Self)
    }

    pub(super) const fn contract(self) -> &'static str {
        CONTRACT
    }
}

struct Bug340Plan {
    source: String,
    netlist: Netlist,
    print: XycePrintRequest,
}

impl XyceTestRunner {
    fn bug340_tolerance() -> XyceAcComparatorTolerance {
        XyceAcComparatorTolerance::new(
            AC_ABSOLUTE_TOLERANCE,
            AC_RELATIVE_TOLERANCE,
            AC_ZERO_TOLERANCE,
            AC_FREQUENCY_RELATIVE_TOLERANCE,
        )
        .expect("Release-7.10 BUG340 ACComparator tolerances are valid")
    }

    fn validate_bug340_historical_provenance() -> Result<(), String> {
        let bytes = HISTORICAL.iter().map(|record| record.1).sum::<usize>();
        let unique = HISTORICAL
            .iter()
            .map(|record| record.0.to_ascii_lowercase())
            .collect::<BTreeSet<_>>();
        let mut historical_stream = HISTORICAL
            .iter()
            .map(|(path, size, identity)| {
                format!(
                    "{PRETRIM_COMMIT}\t{UPSTREAM_COMMIT}\t{RELEASE_TAG}\t{RELEASE_TAG_OBJECT}\t{path}\t{size}\t{identity}"
                )
            })
            .collect::<Vec<_>>();
        historical_stream.sort();
        let historical_stream = historical_stream.join("\n");
        let historical_sha = format!("{:x}", Sha256::digest(historical_stream.as_bytes()));
        let historical_b3 = blake3::hash(historical_stream.as_bytes())
            .to_hex()
            .to_string();

        let mut retained_stream = RETAINED
            .iter()
            .map(|(path, size, sha)| format!("{path}\t{size}\t{sha}"))
            .collect::<Vec<_>>();
        retained_stream.sort();
        let retained_stream = retained_stream.join("\n");
        let retained_sha = format!("{:x}", Sha256::digest(retained_stream.as_bytes()));
        let retained_b3 = blake3::hash(retained_stream.as_bytes())
            .to_hex()
            .to_string();

        let family_records = HISTORICAL
            .iter()
            .filter(|record| record.0.starts_with(FAMILY_DIRECTORY))
            .count();
        let output_records = HISTORICAL
            .iter()
            .filter(|record| record.0.starts_with(OUTPUT_DIRECTORY))
            .count();
        let identities_are_git_blobs = HISTORICAL.iter().all(|record| {
            record.2.len() == 40 && record.2.bytes().all(|byte| byte.is_ascii_hexdigit())
        });
        if PRETRIM_COMMIT != UPSTREAM_EXCLUSIONS_SOURCE_COMMIT
            || PRETRIM_COMMIT.len() != 40
            || UPSTREAM_COMMIT.len() != 40
            || RELEASE_TAG != "Release-7.10.0"
            || RELEASE_TAG_OBJECT.len() != 40
            || HISTORICAL.len() != 6
            || RETAINED.len() != 2
            || RETAINED[0].0 != OWNER_PATH
            || RETAINED[1].0 != OUTPUT_PATH
            || family_records != 4
            || output_records != 1
            || unique.len() != HISTORICAL.len()
            || bytes != HISTORICAL_CONTENT_BYTES
            || historical_stream.len() != HISTORICAL_STREAM_BYTES
            || historical_sha != HISTORICAL_STREAM_SHA256
            || historical_b3 != HISTORICAL_STREAM_BLAKE3
            || retained_stream.len() != RETAINED_STREAM_BYTES
            || retained_sha != RETAINED_STREAM_SHA256
            || retained_b3 != RETAINED_STREAM_BLAKE3
            || !identities_are_git_blobs
        {
            return Err(format!(
                "{LABEL} provenance changed: historical={}/family={family_records}/output={output_records}/bytes={bytes}/unique={}, historical_stream={}/sha={historical_sha}/b3={historical_b3}, retained_stream={}/sha={retained_sha}/b3={retained_b3}",
                HISTORICAL.len(),
                unique.len(),
                historical_stream.len(),
                retained_stream.len(),
            ));
        }
        Ok(())
    }

    fn read_bug340_directory(
        &self,
        abort: &dyn AbortSignal,
    ) -> Result<BTreeMap<String, Vec<u8>>, String> {
        Self::validate_bug340_historical_provenance()?;
        let directory = self.root.join(FAMILY_DIRECTORY);
        let metadata = fs::symlink_metadata(&directory)
            .map_err(|error| format!("failed to inspect {LABEL} directory: {error}"))?;
        if metadata.file_type().is_symlink() || !metadata.file_type().is_dir() {
            return Err(format!(
                "{LABEL} source must be a regular non-symlink directory"
            ));
        }
        let owner = RETAINED[0];
        let owner_name = Path::new(owner.0)
            .file_name()
            .and_then(|name| name.to_str())
            .ok_or_else(|| format!("{LABEL} retained owner path is not canonical UTF-8"))?;
        if owner_name != OWNER_NAME {
            return Err(format!("{LABEL} retained owner ledger path changed"));
        }
        let expected = [(owner_name, owner.1, owner.2)]
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
            let Some((expected_name, expected_size, expected_sha)) = expected.get(&key).copied()
            else {
                return Err(format!("{LABEL} acquired unexpected member {name:?}"));
            };
            if name != expected_name || observed.contains_key(&key) {
                return Err(format!("{LABEL} member case/census changed: {name:?}"));
            }
            let cap = expected_size
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
            if canonical.len() != expected_size || sha != expected_sha {
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

    fn read_bug340_output(&self, abort: &dyn AbortSignal) -> Result<Vec<u8>, String> {
        if abort.is_aborted() {
            return Err(format!("{LABEL} output census aborted"));
        }
        let directory = self.root.join(OUTPUT_DIRECTORY);
        let metadata = fs::symlink_metadata(&directory)
            .map_err(|error| format!("failed to inspect {LABEL} output directory: {error}"))?;
        if metadata.file_type().is_symlink() || !metadata.file_type().is_dir() {
            return Err(format!(
                "{LABEL} output source must be a regular non-symlink directory"
            ));
        }
        let mut entries = fs::read_dir(&directory)
            .map_err(|error| format!("failed to read {LABEL} output directory: {error}"))?;
        let entry = entries
            .next()
            .transpose()
            .map_err(|error| format!("failed to inspect {LABEL} output member: {error}"))?
            .ok_or_else(|| format!("{LABEL} output census is empty"))?;
        if abort.is_aborted() {
            return Err(format!("{LABEL} output census aborted"));
        }
        if entries
            .next()
            .transpose()
            .map_err(|error| format!("failed to inspect {LABEL} output member: {error}"))?
            .is_some()
        {
            return Err(format!(
                "{LABEL} output census changed: expected exactly one file"
            ));
        }
        if entry.file_name().to_str() != Some(OUTPUT_NAME) {
            return Err(format!(
                "{LABEL} output member case/name changed: {:?}",
                entry.file_name()
            ));
        }
        let path = entry.path();
        let metadata = fs::symlink_metadata(&path)
            .map_err(|error| format!("failed to inspect {LABEL} output: {error}"))?;
        if metadata.file_type().is_symlink() || !metadata.file_type().is_file() {
            return Err(format!("{LABEL} output must be a regular non-symlink file"));
        }
        let expected_size = RETAINED[1].1;
        let cap = expected_size
            .checked_mul(2)
            .and_then(|value| value.checked_add(3))
            .ok_or_else(|| format!("{LABEL} output-size bound overflowed"))?;
        if metadata.len() > cap as u64 {
            return Err(format!("{LABEL} output exceeds its bounded envelope"));
        }
        let mut bytes = Vec::with_capacity((metadata.len() as usize).min(cap));
        fs::File::open(&path)
            .map_err(|error| format!("failed to open {LABEL} output: {error}"))?
            .take((cap + 1) as u64)
            .read_to_end(&mut bytes)
            .map_err(|error| format!("failed to read {LABEL} output: {error}"))?;
        if bytes.len() > cap {
            return Err(format!("{LABEL} bounded output read grew"));
        }
        let canonical = Self::canonical_lf_text_identity(LABEL, &bytes)?;
        let sha = format!("{:x}", Sha256::digest(&canonical));
        if canonical.len() != expected_size || sha != RETAINED[1].2 {
            return Err(format!(
                "{LABEL} output changed: bytes={}, sha={sha}",
                canonical.len()
            ));
        }
        Ok(canonical)
    }

    fn validate_bug340_provenance(
        &self,
        deck: &XyceDeck,
        abort: &dyn AbortSignal,
    ) -> Result<Bug340Provenance, String> {
        if abort.is_aborted() {
            return Err(format!("{LABEL} provenance validation aborted"));
        }
        let record = Self::normalize_manifest_key(OWNER_PATH);
        if deck.section != XyceDeckSection::Netlists
            || Self::normalize_manifest_key(&deck.relative_path) != record
            || Self::normalize_manifest_key(&self.relative_key(&deck.path)) != record
            || !Self::same_path(&deck.path, &self.root.join(OWNER_PATH))
        {
            return Err(format!("{LABEL} recognized owner is not canonical"));
        }
        let owners = Self::load_upstream_wrapper_decks(&self.root)
            .into_iter()
            .filter(|candidate| candidate.starts_with(FAMILY_PREFIX))
            .collect::<BTreeSet<_>>();
        if owners != BTreeSet::from([record.clone()]) {
            return Err(format!("{LABEL} wrapper ownership changed: {owners:?}"));
        }
        let exclusions = Self::load_upstream_exclusions(&self.root)
            .map_err(|error| format!("{LABEL} exclusions invalid: {error}"))?;
        let family_exclusions = exclusions
            .keys()
            .filter(|candidate| candidate.starts_with(FAMILY_PREFIX))
            .cloned()
            .collect::<BTreeSet<_>>();
        if exclusions.contains_key(&record) || !family_exclusions.is_empty() {
            return Err(format!(
                "{LABEL} wrapper owner/family unexpectedly became excluded: {family_exclusions:?}"
            ));
        }
        let members = self.read_bug340_directory(abort)?;
        let output = self.read_bug340_output(abort)?;
        Ok((members, output))
    }

    fn bug340_nodes(actual: &[String], expected: &[&str]) -> bool {
        actual.len() == expected.len()
            && actual
                .iter()
                .zip(expected)
                .all(|(actual, expected)| actual.eq_ignore_ascii_case(expected))
    }

    fn validate_bug340_source_and_plan(source: &str, path: &Path) -> Result<Bug340Plan, String> {
        if Self::contains_control_block(source) {
            return Err(format!(
                "{LABEL} does not admit a simulator scripting block"
            ));
        }
        let requests = Self::print_output_requests(source, "HB")?;
        let [request] = requests.as_slice() else {
            return Err(format!(
                "{LABEL} requires one .PRINT HB request, found {}",
                requests.len()
            ));
        };
        if request.format.is_some()
            || request.file.is_some()
            || request
                .probes
                .iter()
                .map(String::as_str)
                .ne(["v(1)", "v(2)", "i(v1)"])
        {
            return Err(format!("{LABEL} output projection changed: {request:?}"));
        }
        let netlist = Self::parse_xyce_netlist(source, path)
            .map_err(|error| format!("{LABEL} parse failed: {error}"))?;
        let steps = Self::step_commands(&netlist)?;
        let [step] = steps.as_slice() else {
            return Err(format!(
                "{LABEL} requires exactly one .STEP, found {}",
                steps.len()
            ));
        };
        if !matches!(step, StepCommand {
            target: StepTarget::Device,
            name,
            param_name: None,
            sweep: StepSweep::Linear { start, stop, step },
        } if name.eq_ignore_ascii_case("R1")
            && start.to_bits() == 1.0e3f64.to_bits()
            && stop.to_bits() == 5.0e3f64.to_bits()
            && step.to_bits() == 1.0e3f64.to_bits())
        {
            return Err(format!("{LABEL} STEP contract changed: {step:?}"));
        }
        let hb = netlist
            .analyses
            .iter()
            .filter_map(|analysis| match analysis {
                AnalysisCommand::Hb { frequencies } => Some(frequencies),
                _ => None,
            })
            .collect::<Vec<_>>();
        if !matches!(hb.as_slice(), [frequencies]
            if matches!(frequencies.as_slice(), [frequency]
                if frequency.to_bits() == FUNDAMENTAL_HZ.to_bits()))
            || netlist.analyses.len() != 2
            || netlist.title != "simple RC"
            || !netlist.diagnostics.is_empty()
            || netlist.elements.len() != 3
            || !netlist.models.is_empty()
            || !netlist.subcircuits.is_empty()
            || !netlist.veriloga_includes.is_empty()
            || !netlist.spef_includes.is_empty()
            || !netlist.data_tables.is_empty()
            || !netlist.measurements.is_empty()
            || !netlist.initial_conditions.is_empty()
            || netlist.device_initial_conditions.is_some()
            || !netlist.node_sets.is_empty()
            || !netlist.global_nodes.is_empty()
            || netlist.options.hb_num_frequencies != [HARMONICS]
            || netlist.options.hb_save_ic_data != Some(true)
            || netlist.options.linsol_hb_preconditioner != Some(XyceHbPreconditioner::BlockJacobi)
            || netlist.options.device_debug_level != Some(1)
            || netlist.output_requests.len() != 1
        {
            return Err(format!(
                "{LABEL} typed envelope changed: analyses={:?}, options={:?}, diagnostics={:?}",
                netlist.analyses, netlist.options, netlist.diagnostics
            ));
        }
        let output = &netlist.output_requests[0];
        if output.directive != OutputDirectiveKind::Print
            || output.analysis != Some(rspice_core::netlist::OutputAnalysisKind::Hb)
            || output.print_delimiter.as_ref() != Some(&PrintDelimiter::Whitespace)
            || output.print_precision.is_some()
            || output.print_width.is_some()
            || output
                .operands
                .iter()
                .map(String::as_str)
                .ne(["v(1)", "v(2)", "i(v1)"])
        {
            return Err(format!("{LABEL} typed .PRINT request changed: {output:?}"));
        }
        Self::validate_bug340_topology(&netlist, 1.0e3)?;
        Ok(Bug340Plan {
            source: source.to_string(),
            netlist,
            print: XycePrintRequest {
                probes: request.probes.clone(),
            },
        })
    }

    fn validate_bug340_topology(netlist: &Netlist, resistance: Value) -> Result<(), String> {
        if netlist
            .elements
            .iter()
            .any(|element| element.provenance != ElementProvenance::Authored)
        {
            return Err(format!("{LABEL} does not admit generated circuit elements"));
        }
        let element = |name: &str| {
            netlist
                .elements
                .iter()
                .find(|element| element.name.eq_ignore_ascii_case(name))
        };
        let v1 = element("V1").ok_or_else(|| format!("{LABEL} lost V1"))?;
        let r1 = element("R1").ok_or_else(|| format!("{LABEL} lost R1"))?;
        let c1 = element("C1").ok_or_else(|| format!("{LABEL} lost C1"))?;
        if !Self::bug340_nodes(&v1.nodes, &["1", "0"])
            || !Self::bug340_nodes(&r1.nodes, &["1", "2"])
            || !Self::bug340_nodes(&c1.nodes, &["2", "0"])
            || !matches!(&v1.kind, ElementKind::VoltageSource(SourceSpec::Sin {
                offset,
                amplitude,
                frequency,
                delay,
                damping,
                phase,
            }) if offset.to_bits() == 0.0f64.to_bits()
                && amplitude.to_bits() == 1.0f64.to_bits()
                && frequency.to_bits() == FUNDAMENTAL_HZ.to_bits()
                && delay.to_bits() == 0.0f64.to_bits()
                && damping.to_bits() == 0.0f64.to_bits()
                && phase.to_bits() == 0.0f64.to_bits())
            || !matches!(&r1.kind, ElementKind::Resistor {
                value,
                value_expr: None,
                model: None,
                instance_params,
                deferred_params,
            } if value.to_bits() == resistance.to_bits()
                && instance_params.is_empty() && deferred_params.is_empty())
            || !matches!(&c1.kind, ElementKind::Capacitor {
                value,
                value_expr: None,
                initial_voltage: None,
                model: None,
                instance_params,
                deferred_params,
            } if value.to_bits() == CAPACITANCE_F.to_bits()
                && instance_params.is_empty() && deferred_params.is_empty())
        {
            return Err(format!("{LABEL} RC topology or source excitation changed"));
        }
        Ok(())
    }

    fn bug340_expected_phasors(resistance: Value) -> (Complex64, Complex64, Complex64) {
        let source = Complex64::new(0.0, -1.0);
        let omega_rc = 2.0 * std::f64::consts::PI * FUNDAMENTAL_HZ * resistance * CAPACITANCE_F;
        let transfer = Complex64::new(1.0, 0.0) / Complex64::new(1.0, omega_rc);
        let output = source * transfer;
        let current = (output - source) / resistance;
        (source, output, current)
    }

    fn bug340_close_complex(
        label: &str,
        expected: Complex64,
        actual: Complex64,
        absolute: Value,
        relative: Value,
    ) -> Result<(), String> {
        if !expected.re.is_finite()
            || !expected.im.is_finite()
            || !actual.re.is_finite()
            || !actual.im.is_finite()
        {
            return Err(format!("{LABEL} {label} is non-finite"));
        }
        let error = (actual - expected).norm();
        let scale = expected.norm().max(actual.norm()).max(absolute);
        if error > absolute && error / scale > relative {
            return Err(format!(
                "{LABEL} {label} differs: expected={expected:?}, actual={actual:?}, abs={error}, rel={}",
                error / scale
            ));
        }
        Ok(())
    }

    fn bug340_spectrum<'a>(result: &'a HbResult, node: &str) -> Result<&'a [Complex64], String> {
        result
            .spectral_voltages
            .iter()
            .find(|spectrum| spectrum.node_name.eq_ignore_ascii_case(node))
            .map(|spectrum| spectrum.coefficients.as_slice())
            .ok_or_else(|| format!("{LABEL} lost node spectrum {node}"))
    }

    fn bug340_branch_spectrum<'a>(
        result: &'a HbResult,
        device: &str,
    ) -> Result<&'a [Complex64], String> {
        result
            .mna_branch_currents
            .iter()
            .find(|spectrum| spectrum.device_name.eq_ignore_ascii_case(device))
            .map(|spectrum| spectrum.coefficients.as_slice())
            .ok_or_else(|| format!("{LABEL} lost MNA branch-current spectrum {device}"))
    }

    fn bug340_frequency_grid_is_exact(frequencies: &[Value]) -> bool {
        frequencies.len() == HARMONICS + 1
            && frequencies.iter().enumerate().all(|(harmonic, frequency)| {
                frequency.is_finite()
                    && frequency.to_bits() == (harmonic as Value * FUNDAMENTAL_HZ).to_bits()
            })
    }

    fn validate_bug340_physics(result: &HbResult, resistance: Value) -> Result<(), String> {
        if !result.converged
            || !result.is_valid()
            || !result.residual_norm.is_finite()
            || result.residual_norm > 1.0e-9
            || result.fundamental_freq.to_bits() != FUNDAMENTAL_HZ.to_bits()
            || result.num_harmonics != HARMONICS
            || !Self::bug340_frequency_grid_is_exact(&result.harmonic_frequencies)
            || result.spectral_voltages.len() != 2
            || result.node_names.len() != 2
            || result.mna_branch_currents.len() != 1
            || result
                .spectral_voltages
                .iter()
                .zip(&result.node_names)
                .any(|(spectrum, name)| {
                    !spectrum.node_name.eq_ignore_ascii_case(name)
                        || !Self::bug340_frequency_grid_is_exact(&spectrum.frequencies)
                })
            || result
                .mna_branch_currents
                .iter()
                .any(|spectrum| !Self::bug340_frequency_grid_is_exact(&spectrum.frequencies))
        {
            return Err(format!(
                "{LABEL} HB convergence/result envelope changed: {result:?}"
            ));
        }
        let input = Self::bug340_spectrum(result, "1")?;
        let output = Self::bug340_spectrum(result, "2")?;
        let current = Self::bug340_branch_spectrum(result, "V1")?;
        if input.len() != HARMONICS + 1
            || output.len() != HARMONICS + 1
            || current.len() != HARMONICS + 1
        {
            return Err(format!("{LABEL} harmonic spectrum shape changed"));
        }
        let (expected_input, expected_output, expected_current) =
            Self::bug340_expected_phasors(resistance);
        Self::bug340_close_complex("V(1) H1", expected_input, input[1], 1.0e-12, 1.0e-10)?;
        Self::bug340_close_complex("V(2) H1", expected_output, output[1], 1.0e-12, 1.0e-10)?;
        Self::bug340_close_complex("I(V1) H1", expected_current, current[1], 1.0e-12, 1.0e-10)?;
        for (name, spectrum) in [("V(1)", input), ("V(2)", output), ("I(V1)", current)] {
            if spectrum[0].norm() > 1.0e-12
                || spectrum
                    .iter()
                    .skip(2)
                    .any(|coefficient| coefficient.norm() > 1.0e-12)
            {
                return Err(format!(
                    "{LABEL} linear RC {name} acquired DC or higher-harmonic energy"
                ));
            }
        }
        Self::bug340_close_complex(
            "source-current KCL",
            (output[1] - input[1]) / resistance,
            current[1],
            1.0e-12,
            1.0e-10,
        )?;
        Ok(())
    }

    fn bug340_analytic_table(resistance: Value) -> XycePrnTable {
        let (source, output, current) = Self::bug340_expected_phasors(resistance);
        let mut rows = Vec::with_capacity(2 * HARMONICS + 1);
        for (index, signed_harmonic) in (-(HARMONICS as isize)..=HARMONICS as isize).enumerate() {
            let (source_value, output_value, current_value) = match signed_harmonic {
                -1 => (
                    source.conj() / 2.0,
                    output.conj() / 2.0,
                    current.conj() / 2.0,
                ),
                1 => (source / 2.0, output / 2.0, current / 2.0),
                _ => (
                    Complex64::new(0.0, 0.0),
                    Complex64::new(0.0, 0.0),
                    Complex64::new(0.0, 0.0),
                ),
            };
            rows.push(vec![
                index as Value,
                signed_harmonic as Value * FUNDAMENTAL_HZ,
                source_value.re,
                source_value.im,
                output_value.re,
                output_value.im,
                current_value.re,
                current_value.im,
            ]);
        }
        XycePrnTable {
            columns: OUTPUT_COLUMNS
                .iter()
                .map(|column| (*column).to_string())
                .collect(),
            rows,
        }
    }

    fn bug340_join_tables(tables: &[XycePrnTable]) -> Result<XycePrnTable, String> {
        let [first, ..] = tables else {
            return Err(format!("{LABEL} has no STEP tables"));
        };
        if tables.iter().any(|table| table.columns != first.columns) {
            return Err(format!("{LABEL} STEP table schemas differ"));
        }
        let rows = tables
            .iter()
            .flat_map(|table| table.rows.iter().cloned())
            .collect::<Vec<_>>();
        Ok(XycePrnTable {
            columns: first.columns.clone(),
            rows,
        })
    }

    fn validate_bug340_framing(
        request: &rspice_core::netlist::OutputRequest,
        options: &rspice_core::netlist::SimulationOptions,
        tables: &[XycePrnTable],
    ) -> Result<(), String> {
        let output = serialize_xyce_prn_sequence(
            tables,
            request,
            options,
            XycePrnFooter::ParameterSweep,
            XycePrnLimits::new(55, 32 * 1024),
        )
        .map_err(|error| format!("{LABEL} FD STEP PRN serialization failed: {error}"))?;
        let lines = output.lines().collect::<Vec<_>>();
        if lines.len() != 57
            || lines
                .first()
                .map(|line| line.split_whitespace().collect::<Vec<_>>())
                != Some(OUTPUT_COLUMNS.to_vec())
            || lines.last().copied() != Some("End of Xyce(TM) Parameter Sweep")
            || output.matches("Index").count() != 1
            || output.contains("End of Xyce(TM) Simulation")
        {
            return Err(format!("{LABEL} FD STEP PRN framing changed"));
        }
        for (run, table) in tables.iter().enumerate() {
            if table.rows.len() != 11
                || table
                    .rows
                    .iter()
                    .enumerate()
                    .any(|(index, row)| row.first().copied() != Some(index as Value))
            {
                return Err(format!("{LABEL} run {run} lost local Index 0..10 framing"));
            }
        }
        Ok(())
    }

    pub(super) fn validate_bug340_oracle(
        &self,
        deck: &XyceDeck,
        start: Instant,
    ) -> Result<(), String> {
        let abort = DeadlineAbort::new(start, self.config.max_time_per_test_ms.max(1));
        let (members, output) = self.validate_bug340_provenance(deck, &abort)?;
        let source = members
            .get(&OWNER_NAME.to_ascii_lowercase())
            .ok_or_else(|| format!("{LABEL} lost sealed owner source"))?;
        let source = std::str::from_utf8(source)
            .map_err(|error| format!("{LABEL} owner source is not UTF-8: {error}"))?;
        let plan = Self::validate_bug340_source_and_plan(source, &deck.path)?;
        if plan.source.as_bytes() != source.as_bytes() {
            return Err(format!("{LABEL} planner lost the sealed source"));
        }
        let output = std::str::from_utf8(&output)
            .map_err(|error| format!("{LABEL} FD gold is not UTF-8: {error}"))?;
        if output
            .lines()
            .rev()
            .find(|line| !line.trim().is_empty())
            .map(str::trim)
            != Some("End of Xyce(TM) Parameter Sweep")
        {
            return Err(format!("{LABEL} FD gold lost its exact STEP footer"));
        }
        let historical_gold = Self::parse_prn_table(output)
            .map_err(|error| format!("{LABEL} FD gold parse failed: {error}"))?;
        if historical_gold
            .columns
            .iter()
            .map(String::as_str)
            .ne(OUTPUT_COLUMNS)
            || historical_gold.rows.len() != 55
            || historical_gold
                .rows
                .iter()
                .enumerate()
                .any(|(row, values)| {
                    values.first().copied() != Some((row % (2 * HARMONICS + 1)) as Value)
                        || values.get(1).copied()
                            != Some(
                                ((row % (2 * HARMONICS + 1)) as isize - HARMONICS as isize)
                                    as Value
                                    * FUNDAMENTAL_HZ,
                            )
                })
        {
            return Err(format!("{LABEL} FD gold framing/content shape changed"));
        }
        let steps = Self::step_commands(&plan.netlist)?;
        let engine = self.create_xyce_engine();
        let step_plan = engine
            .plan_step_commands_with_abort(&plan.netlist, &steps, xyce_step_plan_limits(), &abort)
            .map_err(|error| format!("{LABEL} STEP planning failed: {error}"))?;
        if step_plan.total_runs() != STEP_RESISTANCES.len()
            || step_plan.bindings_per_run() != 1
            || step_plan.stored_values() != STEP_RESISTANCES.len()
        {
            return Err(format!("{LABEL} bounded STEP plan shape changed"));
        }

        let template = XycePrnTable {
            columns: historical_gold.columns.clone(),
            rows: historical_gold.rows[..2 * HARMONICS + 1].to_vec(),
        };
        let mut actual_tables = Vec::with_capacity(STEP_RESISTANCES.len());
        let mut analytic_tables = Vec::with_capacity(STEP_RESISTANCES.len());
        let hb_config = rspice_core::analysis::HbConfig::new(FUNDAMENTAL_HZ)
            .with_harmonics(HARMONICS)
            .with_collocation_points(2 * HARMONICS + 1);
        let mut output_magnitudes = Vec::with_capacity(STEP_RESISTANCES.len());
        for (run_index, expected_resistance) in STEP_RESISTANCES.iter().copied().enumerate() {
            if abort.is_aborted() {
                return Err(format!("{LABEL} execution exceeded deadline"));
            }
            let (bindings, run_netlist) = engine
                .materialize_step_run_with_abort(&step_plan, run_index, &abort)
                .map_err(|error| format!("{LABEL} STEP run {run_index} failed: {error}"))?
                .into_parts();
            if bindings.as_slice() != [expected_resistance] {
                return Err(format!(
                    "{LABEL} STEP order changed at run {run_index}: {bindings:?}"
                ));
            }
            Self::validate_bug340_topology(&run_netlist, expected_resistance)?;
            let analysis = engine
                .run_hb_with_abort(&run_netlist, hb_config.clone(), &abort)
                .map_err(|error| format!("{LABEL} R1={expected_resistance} HB failed: {error}"))?;
            Self::validate_bug340_physics(&analysis.result, expected_resistance)?;
            output_magnitudes.push(Self::bug340_spectrum(&analysis.result, "2")?[1].norm());
            let table = Self::hb_frequency_result_to_prn_table(
                &template,
                &plan.print,
                &run_netlist,
                &analysis.result,
            )?;
            if table.columns.iter().map(String::as_str).ne(OUTPUT_COLUMNS) || table.rows.len() != 11
            {
                return Err(format!("{LABEL} projected FD table shape changed"));
            }
            actual_tables.push(table);
            analytic_tables.push(Self::bug340_analytic_table(expected_resistance));
        }
        if output_magnitudes
            .windows(2)
            .any(|pair| !matches!(pair[1].partial_cmp(&pair[0]), Some(Ordering::Less)))
        {
            return Err(format!(
                "{LABEL} RC response is not monotonic across R1 STEP"
            ));
        }

        let actual = Self::bug340_join_tables(&actual_tables)?;
        let analytic = Self::bug340_join_tables(&analytic_tables)?;
        let analytic_mismatches = self.compare_ac_comparator_tables_with_tolerance(
            &historical_gold,
            &analytic,
            Self::bug340_tolerance(),
        )?;
        if !analytic_mismatches.is_empty() {
            return Err(format!(
                "{LABEL} historical FD gold diverges from the independent analytic RC oracle with {} mismatch(es): {:?}",
                analytic_mismatches.len(),
                analytic_mismatches.first()
            ));
        }
        let mismatches = self.compare_ac_comparator_tables_with_tolerance(
            &historical_gold,
            &actual,
            Self::bug340_tolerance(),
        )?;
        if !mismatches.is_empty() {
            return Err(format!(
                "{LABEL} historical GOOD-to-TEST FD comparison failed with {} mismatch(es): {:?}",
                mismatches.len(),
                mismatches.first()
            ));
        }
        Self::validate_bug340_framing(
            &plan.netlist.output_requests[0],
            &plan.netlist.options,
            &actual_tables,
        )?;
        self.validate_bug340_provenance(deck, &abort)?;
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

    #[test]
    fn bug340_route_is_exactly_the_wrapper_owner() {
        assert_eq!(Bug340Role::for_record(OWNER_PATH), Some(Bug340Role));
        assert_eq!(
            Bug340Role::for_record("Netlists/Certification_Tests/BUG_340/stepRC-hb.cir.sh"),
            None
        );
        assert_eq!(
            Bug340Role::for_record("Netlists/Certification_Tests/BUG_340/other.cir"),
            None
        );
    }

    #[test]
    fn bug340_historical_and_retained_provenance_is_exact() {
        XyceTestRunner::validate_bug340_historical_provenance().unwrap();
        let runner = runner();
        let abort = DeadlineAbort::new(Instant::now(), 30_000);
        let members = runner.read_bug340_directory(&abort).unwrap();
        assert_eq!(members.len(), 1);
        assert_eq!(runner.read_bug340_output(&abort).unwrap().len(), 7_195);
    }

    #[test]
    fn bug340_checked_in_owner_executes_complete_oracle() {
        let runner = runner();
        let result = runner.run_test(runner.root.join(OWNER_PATH));
        assert!(result.passed, "{}", result.error.unwrap_or_default());
        assert_eq!(result.contract, CONTRACT);
    }

    #[test]
    fn bug340_analytic_oracle_reproduces_release_anchor_values() {
        let one_k = XyceTestRunner::bug340_analytic_table(1.0e3);
        let five_k = XyceTestRunner::bug340_analytic_table(5.0e3);
        let one_k_positive = &one_k.rows[HARMONICS + 1];
        let five_k_positive = &five_k.rows[HARMONICS + 1];
        assert!((one_k_positive[4] - -3.97887106e-4).abs() < 5.0e-13);
        assert!((one_k_positive[5] - -3.16628498e-7).abs() < 5.0e-16);
        assert!((one_k_positive[6] - -3.97887106e-7).abs() < 5.0e-16);
        assert!((one_k_positive[7] - 4.99999683e-4).abs() < 5.0e-13);
        assert!((five_k_positive[4] - -7.95774695e-5).abs() < 5.0e-14);
        assert!((five_k_positive[5] - -1.26651476e-8).abs() < 5.0e-17);
        assert!((five_k_positive[6] - -1.59154939e-8).abs() < 5.0e-17);
        assert!((five_k_positive[7] - 9.99999975e-5).abs() < 5.0e-13);
    }

    #[test]
    fn bug340_ac_comparator_direction_and_threshold_are_fail_closed() {
        let runner = runner();
        let gold = XyceTestRunner::bug340_analytic_table(1.0e3);
        let mut test = gold.clone();
        assert!(
            runner
                .compare_ac_comparator_tables_with_tolerance(
                    &gold,
                    &test,
                    XyceTestRunner::bug340_tolerance(),
                )
                .unwrap()
                .is_empty()
        );
        test.rows[HARMONICS + 1][7] += AC_ABSOLUTE_TOLERANCE * 2.0;
        assert!(
            !runner
                .compare_ac_comparator_tables_with_tolerance(
                    &gold,
                    &test,
                    XyceTestRunner::bug340_tolerance(),
                )
                .unwrap()
                .is_empty()
        );

        let mut negative_frequency = gold.clone();
        negative_frequency.rows[0][1] += 1.0;
        assert!(
            runner
                .compare_ac_comparator_tables_with_tolerance(
                    &gold,
                    &negative_frequency,
                    XyceTestRunner::bug340_tolerance(),
                )
                .unwrap()
                .is_empty(),
            "the BUG340 oracle must preserve release 7.10's signed negative-frequency denominator"
        );

        let mut reversed_gold = gold.clone();
        reversed_gold.rows[HARMONICS + 1][7] = 0.0;
        let forward = runner
            .compare_ac_comparator_tables_with_tolerance(
                &reversed_gold,
                &gold,
                XyceTestRunner::bug340_tolerance(),
            )
            .unwrap();
        let reverse = runner
            .compare_ac_comparator_tables_with_tolerance(
                &gold,
                &reversed_gold,
                XyceTestRunner::bug340_tolerance(),
            )
            .unwrap();
        assert!(!forward.is_empty());
        assert!(!reverse.is_empty());

        let mut asymmetric_gold = gold.clone();
        let mut asymmetric_test = gold.clone();
        asymmetric_gold.rows[HARMONICS + 1][7] = 5.004e-4;
        asymmetric_test.rows[HARMONICS + 1][7] = 4.999e-4;
        assert!(
            runner
                .compare_ac_comparator_tables_with_tolerance(
                    &asymmetric_gold,
                    &asymmetric_test,
                    XyceTestRunner::bug340_tolerance(),
                )
                .unwrap()
                .is_empty(),
            "historical GOOD-to-TEST direction must pass the asymmetric fixture"
        );
        assert!(
            !runner
                .compare_ac_comparator_tables_with_tolerance(
                    &asymmetric_test,
                    &asymmetric_gold,
                    XyceTestRunner::bug340_tolerance(),
                )
                .unwrap()
                .is_empty(),
            "reversing GOOD and TEST must fail the asymmetric fixture"
        );
    }

    #[test]
    fn bug340_shared_zero_cannot_pass_the_independent_physics_gate() {
        let mut result = HbResult::new(FUNDAMENTAL_HZ, 2, HARMONICS);
        result.converged = true;
        result.residual_norm = 0.0;
        result.node_names = vec!["1".to_string(), "2".to_string()];
        result.spectral_voltages = vec![
            rspice_core::analysis::SpectralVoltage::new("1", HARMONICS),
            rspice_core::analysis::SpectralVoltage::new("2", HARMONICS),
        ];
        result.mna_branch_currents = vec![rspice_core::analysis::SpectralBranchCurrent {
            device_name: "V1".to_string(),
            coefficients: vec![Complex64::new(0.0, 0.0); HARMONICS + 1],
            frequencies: (0..=HARMONICS)
                .map(|harmonic| harmonic as Value * FUNDAMENTAL_HZ)
                .collect(),
        }];
        assert!(XyceTestRunner::validate_bug340_physics(&result, 1.0e3).is_err());

        let grid = (0..=HARMONICS)
            .map(|harmonic| harmonic as Value * FUNDAMENTAL_HZ)
            .collect::<Vec<_>>();
        let (input, output, current) = XyceTestRunner::bug340_expected_phasors(1.0e3);
        result.spectral_voltages[0].coefficients[1] = input;
        result.spectral_voltages[1].coefficients[1] = output;
        result.mna_branch_currents[0].coefficients[1] = current;
        result.spectral_voltages[0].frequencies = grid.clone();
        result.spectral_voltages[1].frequencies = grid.clone();
        result.mna_branch_currents[0].frequencies = grid.clone();
        result.residual_norm = 1.0e-3;
        assert!(XyceTestRunner::validate_bug340_physics(&result, 1.0e3).is_err());

        result.residual_norm = 0.0;
        result.spectral_voltages[1].frequencies[1] += 1.0;
        assert!(XyceTestRunner::validate_bug340_physics(&result, 1.0e3).is_err());
    }
}
