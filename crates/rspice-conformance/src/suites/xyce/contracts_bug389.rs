use super::*;
use rspice_core::analysis::HbResult;
use rspice_core::netlist::{SourceSpec, XyceHbTimeDomainMode};
use std::io::Read as _;

const LABEL: &str = "BUG_389_SON global-parameter harmonic-balance wrapper";
const FAMILY_DIRECTORY: &str = "Netlists/Certification_Tests/BUG_389_SON";
const FAMILY_PREFIX: &str = "netlists/certification_tests/bug_389_son/";
const OUTPUT_DIRECTORY: &str = "OutputData/Certification_Tests/BUG_389_SON";
const EXCLUSION_SOURCE: &str = "Netlists/Certification_Tests/BUG_389_SON/exclude";
const OWNER_NAME: &str = "bug389.cir";
const GLOBAL_NAME: &str = "global_param.cir";
const LOCAL_NAME: &str = "local_param.cir";
const OWNER_PATH: &str = "Netlists/Certification_Tests/BUG_389_SON/bug389.cir";
const GLOBAL_PATH: &str = "Netlists/Certification_Tests/BUG_389_SON/global_param.cir";
const LOCAL_PATH: &str = "Netlists/Certification_Tests/BUG_389_SON/local_param.cir";
const OWNER_RECORD: &str = "netlists/certification_tests/bug_389_son/bug389.cir";
const GLOBAL_RECORD: &str = "netlists/certification_tests/bug_389_son/global_param.cir";
const LOCAL_RECORD: &str = "netlists/certification_tests/bug_389_son/local_param.cir";
const CONTRACT: &str = "bug389_global_parameter_hb_wrapper_owner";

const PRETRIM_COMMIT: &str = "80115a9277c0ddb3409acceb3d4e745fd11cddd4";
const PRETRIM_NETLISTS_TREE: &str = "3e34bfaafa890cb2e4457137b6a0e325c8c1e87d";
const UPSTREAM_COMMIT: &str = "d6e278e371ec2f3df1325dcff4552e585bc7ecc1";
const RELEASE_TAG: &str = "Release-7.10.0";
const RELEASE_TAG_OBJECT: &str = "2a339ec3845af0aef99a7e6cc488a41acf64f6ed";

const HISTORICAL_CONTENT_BYTES: usize = 10_811;
// These identities seal the ordered metadata stream below. They are filled
// from the checked-in constants themselves, so changing a path, size, blob,
// release identity, or record order fails closed before any circuit executes.
const HISTORICAL_STREAM_BYTES: usize = 2_187;
const HISTORICAL_STREAM_SHA256: &str =
    "033f60829d2e55485a882886bf3fe349d690b3cd62d8514d4aee643f285c9217";
const HISTORICAL_STREAM_BLAKE3: &str =
    "6e6461f2761ffab649ba0d89561c284fe1fd336d3457b2806229010d78f55865";
const HISTORICAL: [(&str, usize, &str); 8] = [
    (
        "Netlists/Certification_Tests/BUG_389_SON/Manifest.txt",
        71,
        "269e6ca25dfc808990b30ed370a090e994f48575",
    ),
    (OWNER_PATH, 2, "139597f9cb07c5d48bed18984ec4747f4b4f3438"),
    (
        "Netlists/Certification_Tests/BUG_389_SON/bug389.cir.sh",
        2_009,
        "18f11e1b73a144b5482eb3fc801b5e7505f75a7b",
    ),
    (
        EXCLUSION_SOURCE,
        33,
        "7d61b034a3ae4ae4cf1d6f7bdc9b8055524c96e1",
    ),
    (GLOBAL_PATH, 608, "f683860d8b14ef40311bb67e85786f1bef2882f6"),
    (LOCAL_PATH, 573, "e1c0c90d8250d84e7f061abab77f5dd81d3e518b"),
    (
        "Netlists/Certification_Tests/BUG_389_SON/tags",
        50,
        "67332e0dbbc503038edec1d8ecab7fb814f793e6",
    ),
    (
        "TestScripts/file_compare.pl",
        7_465,
        "67670d9b777a258354d7154d44e74742fd9ff5ae",
    ),
];

const RETAINED_STREAM_BYTES: usize = 371;
const RETAINED_STREAM_SHA256: &str =
    "a28409555a828022bb69c7a39833d80c3b25ce0c5e30370ffa5e74e9338ecc21";
const RETAINED_STREAM_BLAKE3: &str =
    "9c3fecd00c9bd116bc401906f36749bba4533ae24067a5b68bf4f2dfaf3e283f";
const RETAINED: [(&str, usize, &str); 3] = [
    (
        OWNER_NAME,
        2,
        "75a11da44c802486bc6f65640aa48a730f0f684c5c07a42ba3cd1735eb3fb070",
    ),
    (
        GLOBAL_NAME,
        608,
        "7093903628ab0cf6388f77ce8d885dd8945768a53991e972e8e3dcba298817c5",
    ),
    (
        LOCAL_NAME,
        573,
        "7a334e358cbfe46973b8e67443556f5c1014bdb5fc4e485d7b3cb83cd7745ae4",
    ),
];

const FUNDAMENTAL_HZ: Value = 1.0e6;
const HARMONICS: usize = 101;
const COLLOCATION_POINTS: usize = 2 * HARMONICS + 1;
const NONLINEAR_MAX_STEPS: usize = 2;
const PRINT_PRECISION: usize = 7;
const SOURCE_RESISTANCE: Value = 50.0;
const CABLE_RESISTANCE: Value = 0.12;
const CABLE_INDUCTANCE: Value = 1.2e-6;
const CABLE_CAPACITANCE: Value = 420.0e-12;
const TD_TOLERANCE: XyceFileCompareTolerance = XyceFileCompareTolerance {
    absolute: 1.0e-6,
    relative: 1.0e-4,
    zero: 1.0e-6,
};
const FD_COLUMNS: [&str; 5] = ["FREQ", "Re(V(1))", "Im(V(1))", "Re(V(1B))", "Im(V(1B))"];
const TD_COLUMNS: [&str; 3] = ["TIME", "V(1)", "V(1B)"];
const RESULT_NODE_NAMES: [&str; 4] = ["1", "1B", "DUT", "X_bigline.3"];

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(super) struct Bug389Role;

impl Bug389Role {
    pub(super) fn for_record(record: &str) -> Option<Self> {
        (XyceTestRunner::normalize_manifest_key(record) == OWNER_RECORD).then_some(Self)
    }

    pub(super) const fn contract(self) -> &'static str {
        CONTRACT
    }
}

struct Bug389Run {
    result: HbResult,
    fd: XycePrnTable,
    td: XycePrnTable,
}

impl XyceTestRunner {
    fn validate_bug389_record_streams() -> Result<(), String> {
        let content_bytes = HISTORICAL.iter().map(|record| record.1).sum::<usize>();
        let historical = HISTORICAL
            .iter()
            .map(|(path, bytes, blob)| {
                format!(
                    "{PRETRIM_COMMIT}\t{PRETRIM_NETLISTS_TREE}\t{UPSTREAM_COMMIT}\t{RELEASE_TAG}\t{RELEASE_TAG_OBJECT}\t{path}\t{bytes}\t{blob}"
                )
            })
            .collect::<Vec<_>>()
            .join("\n");
        let historical_sha = format!("{:x}", Sha256::digest(historical.as_bytes()));
        let historical_b3 = blake3::hash(historical.as_bytes()).to_hex().to_string();
        let retained = RETAINED
            .iter()
            .map(|(name, bytes, sha)| format!("{FAMILY_DIRECTORY}/{name}\t{bytes}\t{sha}"))
            .collect::<Vec<_>>()
            .join("\n");
        let retained_sha = format!("{:x}", Sha256::digest(retained.as_bytes()));
        let retained_b3 = blake3::hash(retained.as_bytes()).to_hex().to_string();
        let unique = HISTORICAL
            .iter()
            .map(|record| record.0.to_ascii_lowercase())
            .collect::<BTreeSet<_>>();

        if PRETRIM_COMMIT != UPSTREAM_EXCLUSIONS_SOURCE_COMMIT
            || PRETRIM_NETLISTS_TREE != UPSTREAM_EXCLUSIONS_SOURCE_NETLISTS_TREE
            || HISTORICAL.len() != 8
            || unique.len() != HISTORICAL.len()
            || content_bytes != HISTORICAL_CONTENT_BYTES
            || !HISTORICAL.iter().all(|record| {
                record.2.len() == 40 && record.2.bytes().all(|byte| byte.is_ascii_hexdigit())
            })
            || historical.len() != HISTORICAL_STREAM_BYTES
            || historical_sha != HISTORICAL_STREAM_SHA256
            || historical_b3 != HISTORICAL_STREAM_BLAKE3
            || RETAINED.len() != 3
            || retained.len() != RETAINED_STREAM_BYTES
            || retained_sha != RETAINED_STREAM_SHA256
            || retained_b3 != RETAINED_STREAM_BLAKE3
        {
            return Err(format!(
                "{LABEL} provenance changed: historical={}/{content_bytes}/{}/sha={historical_sha}/b3={historical_b3}; retained={}/{}/sha={retained_sha}/b3={retained_b3}",
                HISTORICAL.len(),
                historical.len(),
                RETAINED.len(),
                retained.len(),
            ));
        }
        Ok(())
    }

    fn read_bug389_directory(
        &self,
        abort: &dyn AbortSignal,
    ) -> Result<BTreeMap<String, Vec<u8>>, String> {
        Self::validate_bug389_record_streams()?;
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

    fn validate_bug389_provenance(
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
        if exclusions.contains_key(OWNER_RECORD) {
            return Err(format!("{LABEL} owner must not be excluded"));
        }
        let family = exclusions
            .iter()
            .filter(|(record, _)| record.starts_with(FAMILY_PREFIX))
            .collect::<Vec<_>>();
        if family.len() != 2 {
            return Err(format!("{LABEL} exclusion census changed: {family:?}"));
        }
        for expected_record in [GLOBAL_RECORD, LOCAL_RECORD] {
            let Some(qualification) = exclusions.get(expected_record) else {
                return Err(format!("{LABEL} lost control exclusion {expected_record}"));
            };
            if qualification.source != EXCLUSION_SOURCE
                || !matches!(
                    qualification.disposition,
                    XyceUpstreamExclusionDisposition::Excluded
                )
            {
                return Err(format!(
                    "{LABEL} control exclusion changed: {expected_record} {qualification:?}"
                ));
            }
        }
        match fs::symlink_metadata(self.root.join(OUTPUT_DIRECTORY)) {
            Err(error) if error.kind() == std::io::ErrorKind::NotFound => {}
            Err(error) => return Err(format!("failed to inspect {LABEL} OutputData: {error}")),
            Ok(_) => return Err(format!("{LABEL} must not acquire invented numerical gold")),
        }
        for path in [OWNER_PATH, GLOBAL_PATH, LOCAL_PATH] {
            self.reject_wrapper_output_artifacts(&self.root.join(path))
                .map_err(|error| format!("{LABEL} {path} {error}"))?;
        }
        self.read_bug389_directory(abort)
    }

    fn bug389_namespace_transform(global: &str, local: &str) -> Result<(), String> {
        let token = ".global_param";
        let positions = global.match_indices(token).collect::<Vec<_>>();
        if positions.len() != 5
            || positions.iter().any(|(index, _)| {
                let before = global.as_bytes().get(index.wrapping_sub(1)).copied();
                let after = global.as_bytes().get(index + token.len()).copied();
                before.is_some_and(|byte| !byte.is_ascii_whitespace())
                    || after.is_some_and(|byte| !byte.is_ascii_whitespace())
            })
            || local.contains(token)
            || global.replace(token, ".param") != local
        {
            return Err(format!(
                "{LABEL} controls are not the exact five-token .global_param -> .param transform"
            ));
        }
        Ok(())
    }

    fn validate_bug389_source(source: &str, path: &Path) -> Result<Netlist, String> {
        if Self::contains_control_block(source) {
            return Err(format!(
                "{LABEL} does not admit a simulator scripting block"
            ));
        }
        let requests = Self::print_output_requests(source, "HB")?;
        let [request] = requests.as_slice() else {
            return Err(format!("{LABEL} requires exactly one .PRINT HB request"));
        };
        if request.format.as_deref() != Some("tecplot")
            && request.format.as_deref() != Some("TECPLOT")
        {
            return Err(format!("{LABEL} requires authored TECPLOT output"));
        }
        if request.file.is_some()
            || request.probes.len() != 2
            || !request.probes[0].eq_ignore_ascii_case("v(1)")
            || !request.probes[1].eq_ignore_ascii_case("v(1b)")
        {
            return Err(format!("{LABEL} output projection changed: {request:?}"));
        }
        let netlist = Self::parse_xyce_netlist(source, path)
            .map_err(|error| format!("{LABEL} parse failed: {error}"))?;
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
            || netlist.analyses.len() != 1
            || !netlist.diagnostics.is_empty()
            || netlist.elements.len() != 3
            || netlist.subcircuits.len() != 1
            || !netlist.models.is_empty()
            || !netlist.veriloga_includes.is_empty()
            || !netlist.spef_includes.is_empty()
            || !netlist.data_tables.is_empty()
            || !netlist.measurements.is_empty()
            || !netlist.initial_conditions.is_empty()
            || netlist.device_initial_conditions.is_some()
            || !netlist.node_sets.is_empty()
            || !netlist.global_nodes.is_empty()
            || netlist.options.hb_num_frequencies != [HARMONICS]
            || netlist.options.hb_time_domain_mode != Some(XyceHbTimeDomainMode::Direct)
            || netlist.options.nonlin_hb_maxstep != Some(NONLINEAR_MAX_STEPS)
            || !netlist
                .options
                .method
                .as_deref()
                .is_some_and(|method| method.eq_ignore_ascii_case("gear"))
            || netlist.output_requests.len() != 1
        {
            return Err(format!(
                "{LABEL} typed source envelope changed: analyses={:?}, options={:?}, diagnostics={:?}",
                netlist.analyses, netlist.options, netlist.diagnostics
            ));
        }
        let output = &netlist.output_requests[0];
        if output.directive != OutputDirectiveKind::Print
            || output.analysis != Some(OutputAnalysisKind::Hb)
            || output.print_delimiter.as_ref() != Some(&PrintDelimiter::Whitespace)
            || output.print_precision != Some(PRINT_PRECISION as i32)
            || output.print_width.is_some()
            || output.operands.len() != 2
            || !output.operands[0].eq_ignore_ascii_case("v(1)")
            || !output.operands[1].eq_ignore_ascii_case("v(1b)")
        {
            return Err(format!("{LABEL} typed .PRINT request changed: {output:?}"));
        }
        Self::validate_bug389_topology(&netlist)?;
        Ok(netlist)
    }

    fn validate_bug389_topology(netlist: &Netlist) -> Result<(), String> {
        let flattened = flatten_netlist_with_models(netlist)
            .map_err(|error| format!("{LABEL} flatten failed: {error}"))?;
        if flattened.elements.len() != 5
            || !flattened.scoped_models.is_empty()
            || !flattened.scoped_initial_conditions.is_empty()
            || !flattened.scoped_node_sets.is_empty()
            || !flattened.scoped_startup_directives.is_empty()
            || flattened
                .elements
                .iter()
                .any(|element| element.provenance != ElementProvenance::Authored)
        {
            return Err(format!("{LABEL} flattened topology envelope changed"));
        }
        let element = |name: &str| {
            flattened
                .elements
                .iter()
                .find(|element| element.name.eq_ignore_ascii_case(name))
        };
        let nodes_match = |actual: &[String], expected: &[&str]| {
            actual.len() == expected.len()
                && actual
                    .iter()
                    .zip(expected)
                    .all(|(actual, expected)| actual.eq_ignore_ascii_case(expected))
        };
        let source = element("VsigGen").ok_or_else(|| format!("{LABEL} lost VsigGen"))?;
        let source_resistor = element("RsigGen").ok_or_else(|| format!("{LABEL} lost RsigGen"))?;
        let cable_resistor =
            element("X_bigline.R1").ok_or_else(|| format!("{LABEL} lost cable R1"))?;
        let cable_capacitor =
            element("X_bigline.C1").ok_or_else(|| format!("{LABEL} lost cable C1"))?;
        let cable_inductor =
            element("X_bigline.L1").ok_or_else(|| format!("{LABEL} lost cable L1"))?;
        if !nodes_match(&source.nodes, &["1", "0"])
            || !matches!(&source.kind, ElementKind::VoltageSource(SourceSpec::Pulse {
                v1, v2, delay, rise, fall, width, period, pulse_count, width_defaults_to_zero,
            }) if v1.to_bits() == 0.0f64.to_bits()
                && v2.to_bits() == 1.0f64.to_bits()
                && delay.to_bits() == 0.0f64.to_bits()
                && rise.to_bits() == 5.0e-9f64.to_bits()
                && fall.to_bits() == 5.0e-9f64.to_bits()
                && width.to_bits() == 0.49e-6f64.to_bits()
                && period.to_bits() == 1.0e-6f64.to_bits()
                && pulse_count.to_bits() == 0.0f64.to_bits()
                && !width_defaults_to_zero)
            || !nodes_match(&source_resistor.nodes, &["1", "1b"])
            || !matches!(&source_resistor.kind, ElementKind::Resistor { value, .. }
                if value.to_bits() == SOURCE_RESISTANCE.to_bits())
            || !nodes_match(&cable_resistor.nodes, &["X_bigline.3", "1b"])
            || !matches!(&cable_resistor.kind, ElementKind::Resistor { value, .. }
                if value.to_bits() == CABLE_RESISTANCE.to_bits())
            || !nodes_match(&cable_capacitor.nodes, &["1b", "0"])
            || !matches!(&cable_capacitor.kind, ElementKind::Capacitor { value, .. }
                if value.to_bits() == CABLE_CAPACITANCE.to_bits())
            || !nodes_match(&cable_inductor.nodes, &["DUT", "X_bigline.3"])
            || !matches!(&cable_inductor.kind, ElementKind::Inductor { value, .. }
                if value.to_bits() == CABLE_INDUCTANCE.to_bits())
        {
            return Err(format!(
                "{LABEL} exact authored pulse/RC/open-cable topology changed: {:?}",
                flattened.elements
            ));
        }
        let nodes = flattened
            .elements
            .iter()
            .flat_map(|element| element.nodes.iter())
            .filter(|node| node.as_str() != "0")
            .map(|node| node.to_ascii_lowercase())
            .collect::<BTreeSet<_>>();
        let branch_unknowns = flattened
            .elements
            .iter()
            .filter(|element| {
                matches!(
                    element.kind,
                    ElementKind::VoltageSource(_) | ElementKind::Inductor { .. }
                )
            })
            .count();
        if nodes.len() != 4 || branch_unknowns != 2 || nodes.len() + branch_unknowns != 6 {
            return Err(format!(
                "{LABEL} native unknown census changed: nodes={nodes:?}, branches={branch_unknowns}"
            ));
        }
        Ok(())
    }

    fn bug389_spectrum<'a>(result: &'a HbResult, node: &str) -> Result<&'a [Complex64], String> {
        result
            .spectral_voltages
            .iter()
            .find(|spectrum| spectrum.node_name.eq_ignore_ascii_case(node))
            .map(|spectrum| spectrum.coefficients.as_slice())
            .ok_or_else(|| format!("{LABEL} lost node spectrum {node}"))
    }

    fn bug389_pulse_value(time: Value) -> Value {
        let phase = time.rem_euclid(FUNDAMENTAL_HZ.recip());
        if phase < 5.0e-9 {
            phase / 5.0e-9
        } else if phase < 495.0e-9 {
            1.0
        } else if phase < 500.0e-9 {
            (500.0e-9 - phase) / 5.0e-9
        } else {
            0.0
        }
    }

    fn bug389_pulse_dft() -> Vec<Complex64> {
        (0..=HARMONICS)
            .map(|harmonic| {
                let coefficient = (0..COLLOCATION_POINTS)
                    .map(|sample| {
                        let phase =
                            2.0 * std::f64::consts::PI * harmonic as Value * sample as Value
                                / COLLOCATION_POINTS as Value;
                        Complex64::from_polar(
                            Self::bug389_pulse_value(
                                sample as Value / (COLLOCATION_POINTS as Value * FUNDAMENTAL_HZ),
                            ),
                            -phase,
                        )
                    })
                    .sum::<Complex64>()
                    / COLLOCATION_POINTS as Value;
                if harmonic == 0 {
                    coefficient
                } else {
                    coefficient * 2.0
                }
            })
            .collect()
    }

    fn bug389_close(label: &str, actual: Complex64, expected: Complex64) -> Result<(), String> {
        if !actual.re.is_finite()
            || !actual.im.is_finite()
            || !expected.re.is_finite()
            || !expected.im.is_finite()
        {
            return Err(format!("{LABEL} {label} is non-finite"));
        }
        let error = (actual - expected).norm();
        let scale = actual.norm().max(expected.norm()).max(1.0e-12);
        if error > 2.0e-9 && error / scale > 2.0e-8 {
            return Err(format!(
                "{LABEL} {label} differs: actual={actual:?}, expected={expected:?}, error={error}"
            ));
        }
        Ok(())
    }

    fn validate_bug389_physics(result: &HbResult) -> Result<(), String> {
        let frequency_grid_matches = |frequencies: &[Value]| {
            frequencies.len() == result.harmonic_frequencies.len()
                && frequencies
                    .iter()
                    .zip(&result.harmonic_frequencies)
                    .all(|(actual, expected)| actual.to_bits() == expected.to_bits())
        };
        if !result.converged
            || !result.is_valid()
            || !result.residual_norm.is_finite()
            || result.residual_norm > 1.0e-8
            || result.fundamental_freq.to_bits() != FUNDAMENTAL_HZ.to_bits()
            || result.num_harmonics != HARMONICS
            || result.harmonic_frequencies.len() != HARMONICS + 1
            || result
                .harmonic_frequencies
                .iter()
                .enumerate()
                .any(|(harmonic, frequency)| {
                    frequency.to_bits() != (harmonic as Value * FUNDAMENTAL_HZ).to_bits()
                })
            || result.node_names.len() != 4
            || result.spectral_voltages.len() != 4
            || result
                .node_names
                .iter()
                .map(String::as_str)
                .ne(RESULT_NODE_NAMES)
            || result
                .spectral_voltages
                .iter()
                .map(|spectrum| spectrum.node_name.as_str())
                .ne(RESULT_NODE_NAMES)
            || result.spectral_voltages.iter().any(|spectrum| {
                spectrum.coefficients.len() != HARMONICS + 1
                    || !frequency_grid_matches(&spectrum.frequencies)
            })
            || result.mna_branch_currents.len() != 1
            || result.reactive_spectra.len() != 2
            || result.continuation_limitations.as_slice()
                != [rspice_core::analysis::HbContinuationLimitation::InductorDcCurrentUsesShortSurrogate]
        {
            return Err(format!(
                "{LABEL} HB result envelope changed: converged={}/valid={}/residual={}/fundamental={}/harmonics={}/frequencies={}/nodes={:?}/spectra={}/branches={}/reactive={}/limitations={:?}",
                result.converged,
                result.is_valid(),
                result.residual_norm,
                result.fundamental_freq,
                result.num_harmonics,
                result.harmonic_frequencies.len(),
                result.node_names,
                result.spectral_voltages.len(),
                result.mna_branch_currents.len(),
                result.reactive_spectra.len(),
                result.continuation_limitations,
            ));
        }
        let source = Self::bug389_spectrum(result, "1")?;
        let output = Self::bug389_spectrum(result, "1B")?;
        let expected_source = Self::bug389_pulse_dft();
        if source.len() != HARMONICS + 1 || output.len() != HARMONICS + 1 {
            return Err(format!("{LABEL} spectrum shape changed"));
        }
        let hidden = result
            .spectral_voltages
            .iter()
            .filter(|spectrum| {
                !spectrum.node_name.eq_ignore_ascii_case("1")
                    && !spectrum.node_name.eq_ignore_ascii_case("1b")
            })
            .collect::<Vec<_>>();
        if hidden.len() != 2 {
            return Err(format!(
                "{LABEL} hidden-node census changed: {:?}",
                result.node_names
            ));
        }
        let source_current = result
            .mna_branch_currents
            .iter()
            .find(|spectrum| spectrum.device_name.eq_ignore_ascii_case("VsigGen"))
            .ok_or_else(|| format!("{LABEL} lost VsigGen branch-current spectrum"))?;
        let capacitor = result
            .reactive_spectra
            .iter()
            .find(|spectrum| {
                spectrum.device_name.eq_ignore_ascii_case("X_bigline.C1")
                    && spectrum.kind == rspice_core::analysis::HbReactiveKind::Capacitor
            })
            .ok_or_else(|| format!("{LABEL} lost cable capacitor spectrum"))?;
        let inductor = result
            .reactive_spectra
            .iter()
            .find(|spectrum| {
                spectrum.device_name.eq_ignore_ascii_case("X_bigline.L1")
                    && spectrum.kind == rspice_core::analysis::HbReactiveKind::Inductor
            })
            .ok_or_else(|| format!("{LABEL} lost cable inductor spectrum"))?;
        if source_current.coefficients.len() != HARMONICS + 1
            || !frequency_grid_matches(&source_current.frequencies)
            || capacitor.current_coefficients.len() != HARMONICS + 1
            || inductor.current_coefficients.len() != HARMONICS + 1
            || !capacitor.dc_current_is_exact
            || inductor.dc_current_is_exact
        {
            return Err(format!("{LABEL} branch/reactive spectrum shape changed"));
        }
        let mut harmonic_energy = 0.0;
        for harmonic in 0..=HARMONICS {
            Self::bug389_close(
                &format!("pulse DFT H{harmonic}"),
                source[harmonic],
                expected_source[harmonic],
            )?;
            let omega = 2.0 * std::f64::consts::PI * harmonic as Value * FUNDAMENTAL_HZ;
            let transfer = Complex64::new(1.0, 0.0)
                / Complex64::new(1.0, omega * SOURCE_RESISTANCE * CABLE_CAPACITANCE);
            let expected_output = expected_source[harmonic] * transfer;
            Self::bug389_close(
                &format!("RC transfer H{harmonic}"),
                output[harmonic],
                expected_output,
            )?;
            for node in &hidden {
                if node.coefficients.len() != HARMONICS + 1 {
                    return Err(format!("{LABEL} hidden-node spectrum shape changed"));
                }
                Self::bug389_close(
                    &format!("open-cable hidden equality {} H{harmonic}", node.node_name),
                    node.coefficients[harmonic],
                    output[harmonic],
                )?;
            }
            let kcl = (output[harmonic] - source[harmonic]) / SOURCE_RESISTANCE
                + Complex64::new(0.0, omega * CABLE_CAPACITANCE) * output[harmonic];
            Self::bug389_close(
                &format!("V(1B) KCL H{harmonic}"),
                kcl,
                Complex64::new(0.0, 0.0),
            )?;
            Self::bug389_close(
                &format!("VsigGen source-current KCL H{harmonic}"),
                source_current.coefficients[harmonic],
                (output[harmonic] - source[harmonic]) / SOURCE_RESISTANCE,
            )?;
            Self::bug389_close(
                &format!("cable capacitor constitutive current H{harmonic}"),
                capacitor.current_coefficients[harmonic],
                Complex64::new(0.0, omega * CABLE_CAPACITANCE) * output[harmonic],
            )?;
            Self::bug389_close(
                &format!("open cable inductor current H{harmonic}"),
                inductor.current_coefficients[harmonic],
                Complex64::new(0.0, 0.0),
            )?;
            if harmonic > 1 {
                harmonic_energy += source[harmonic].norm_sqr();
            }
        }
        if source[0].re < 0.45
            || source[0].re > 0.55
            || source[1].norm() < 0.5
            || harmonic_energy < 1.0e-3
        {
            return Err(format!("{LABEL} pulse excitation is vacuous"));
        }
        Ok(())
    }

    fn bug389_fd_table(result: &HbResult) -> Result<XycePrnTable, String> {
        let source = Self::bug389_spectrum(result, "1")?;
        let output = Self::bug389_spectrum(result, "1B")?;
        let mut rows = Vec::with_capacity(COLLOCATION_POINTS);
        for signed in -(HARMONICS as isize)..=HARMONICS as isize {
            let harmonic = signed.unsigned_abs();
            let project = |spectrum: &[Complex64]| {
                if signed == 0 {
                    spectrum[0]
                } else if signed < 0 {
                    spectrum[harmonic].conj() / 2.0
                } else {
                    spectrum[harmonic] / 2.0
                }
            };
            let input = project(source);
            let output = project(output);
            rows.push(vec![
                signed as Value * FUNDAMENTAL_HZ,
                input.re,
                input.im,
                output.re,
                output.im,
            ]);
        }
        Ok(XycePrnTable {
            columns: FD_COLUMNS
                .iter()
                .map(|value| (*value).to_string())
                .collect(),
            rows,
        })
    }

    fn bug389_td_table(result: &HbResult) -> Result<XycePrnTable, String> {
        let source = Self::bug389_spectrum(result, "1")?;
        let output = Self::bug389_spectrum(result, "1B")?;
        let mut rows = Vec::with_capacity(COLLOCATION_POINTS);
        for sample in 0..COLLOCATION_POINTS {
            let phase = 2.0 * std::f64::consts::PI * sample as Value / COLLOCATION_POINTS as Value;
            let evaluate = |spectrum: &[Complex64]| {
                spectrum[0].re
                    + spectrum
                        .iter()
                        .enumerate()
                        .skip(1)
                        .map(|(harmonic, coefficient)| {
                            (*coefficient * Complex64::from_polar(1.0, harmonic as Value * phase))
                                .re
                        })
                        .sum::<Value>()
            };
            rows.push(vec![
                sample as Value / (COLLOCATION_POINTS as Value * FUNDAMENTAL_HZ),
                evaluate(source),
                evaluate(output),
            ]);
        }
        Ok(XycePrnTable {
            columns: TD_COLUMNS
                .iter()
                .map(|value| (*value).to_string())
                .collect(),
            rows,
        })
    }

    fn bug389_precision7_table(table: &XycePrnTable) -> Result<XycePrnTable, String> {
        let rows = table
            .rows
            .iter()
            .map(|row| {
                row.iter()
                    .map(|value| {
                        format_xyce_prn_scientific(
                            *value,
                            PRINT_PRECISION,
                            32,
                            XycePrnScientificStyle::Canonical,
                        )
                        .map_err(|error| {
                            format!("{LABEL} precision-7 serialization failed: {error}")
                        })?
                        .parse::<Value>()
                        .map_err(|error| {
                            format!("{LABEL} precision-7 token reparsing failed: {error}")
                        })
                    })
                    .collect::<Result<Vec<_>, _>>()
            })
            .collect::<Result<Vec<_>, _>>()?;
        Ok(XycePrnTable {
            columns: table.columns.clone(),
            rows,
        })
    }

    fn bug389_filtered_tecplot_bytes(table: &XycePrnTable) -> Result<Vec<u8>, String> {
        if table.columns.iter().map(String::as_str).ne(FD_COLUMNS)
            || table.rows.len() != COLLOCATION_POINTS
            || table.rows.iter().any(|row| row.len() != FD_COLUMNS.len())
        {
            return Err(format!("{LABEL} FD TECPLOT shape changed"));
        }
        let mut output = String::new();
        // `tecplotFreqHeader` places the first variable on the VARIABLES line
        // and one subsequent variable on each line. Release 7.10's grep chain
        // removes TITLE/DATASETAUXDATA/ZONE but retains this declaration and
        // the exact completion footer.
        output.push_str("\tVARIABLES = ");
        for column in FD_COLUMNS {
            output.push_str(&format!("\" {column}\" \n"));
        }
        for row in &table.rows {
            for (column, value) in row.iter().enumerate() {
                if column != 0 {
                    output.push(' ');
                }
                let token = format_xyce_prn_scientific(
                    *value,
                    PRINT_PRECISION,
                    32,
                    XycePrnScientificStyle::Canonical,
                )
                .map_err(|error| format!("{LABEL} FD TECPLOT serialization failed: {error}"))?;
                output.push_str(&format!("{token:>17}"));
            }
            output.push('\n');
        }
        output.push_str("End of Xyce(TM) Simulation\n");
        Ok(output.into_bytes())
    }

    fn validate_bug389_native_circuit(circuit: &rspice_core::CircuitData) -> Result<(), String> {
        let nodes = circuit.node_names_sorted();
        let branches = circuit.branch_names_sorted();
        let sources = circuit.independent_source_names();
        let report = circuit.device_op_report();
        if circuit.has_generated_veriloga_devices()
            || circuit.num_nodes() != 4
            || circuit.matrix_size() != 6
            || circuit.device_count() != 5
            || nodes
                .iter()
                .map(String::as_str)
                .ne(["1", "1B", "DUT", "X_bigline.3"])
            || branches
                .iter()
                .map(String::as_str)
                .ne(["VSIGGEN", "X_bigline.L1"])
            || sources.iter().map(String::as_str).ne(["VSIGGEN"])
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

    fn validate_bug389_convergence_quality(engine: &Engine) -> Result<(), String> {
        let quality = engine.convergence_quality();
        if quality.gmin_stepping_count != 0
            || quality.source_stepping_count != 0
            || quality.force_accepted_points != 0
            || !quality.force_accepted_indices.is_empty()
            || quality.failure_diagnostic.is_some()
        {
            return Err(format!(
                "{LABEL} returned a numerically unqualified HB result: {quality:?}"
            ));
        }
        Ok(())
    }

    fn run_bug389_worker(
        &self,
        source: &str,
        path: &Path,
        abort: &dyn AbortSignal,
    ) -> Result<Bug389Run, String> {
        let netlist = Self::validate_bug389_source(source, path)?;
        let hb_config = rspice_core::analysis::HbConfig::new(FUNDAMENTAL_HZ)
            .with_harmonics(HARMONICS)
            .with_collocation_points(COLLOCATION_POINTS);
        // Each relational side owns a newly constructed engine. This is a
        // product execution path, not a reused solver state or copied result.
        let engine = self.create_xyce_engine();
        let circuit = engine
            .build_circuit(&netlist)
            .map_err(|error| format!("{LABEL} production assembly failed: {error}"))?;
        Self::validate_bug389_native_circuit(&circuit)?;
        let analysis = engine
            .run_hb_with_abort(&netlist, hb_config, abort)
            .map_err(|error| format!("{LABEL} clean HB run failed: {error}"))?;
        Self::validate_bug389_convergence_quality(&engine)?;
        if analysis.operating_point.config().max_iterations != NONLINEAR_MAX_STEPS {
            return Err(format!(
                "{LABEL} NONLIN-HB MAXSTEP did not reach the production HB solver: expected {NONLINEAR_MAX_STEPS}, found {}",
                analysis.operating_point.config().max_iterations
            ));
        }
        Self::validate_bug389_physics(&analysis.result)?;
        let fd = Self::bug389_fd_table(&analysis.result)?;
        let td = Self::bug389_td_table(&analysis.result)?;
        Ok(Bug389Run {
            result: analysis.result,
            fd,
            td,
        })
    }

    pub(super) fn validate_bug389_oracle(
        &self,
        deck: &XyceDeck,
        start: Instant,
    ) -> Result<(), String> {
        let abort = DeadlineAbort::new(start, self.config.max_time_per_test_ms.max(1));
        let before = self.validate_bug389_provenance(deck, &abort)?;
        let owner = before
            .get(&OWNER_NAME.to_ascii_lowercase())
            .ok_or_else(|| format!("{LABEL} lost sealed owner"))?;
        if owner.as_slice() != b"\n\n" {
            return Err(format!(
                "{LABEL} owner is no longer the exact blank wrapper sentinel"
            ));
        }
        let global = before
            .get(&GLOBAL_NAME.to_ascii_lowercase())
            .ok_or_else(|| format!("{LABEL} lost global control"))?;
        let local = before
            .get(&LOCAL_NAME.to_ascii_lowercase())
            .ok_or_else(|| format!("{LABEL} lost local control"))?;
        let global_source = std::str::from_utf8(global)
            .map_err(|error| format!("{LABEL} global control is not UTF-8: {error}"))?;
        let local_source = std::str::from_utf8(local)
            .map_err(|error| format!("{LABEL} local control is not UTF-8: {error}"))?;
        Self::bug389_namespace_transform(global_source, local_source)?;

        let global_run =
            self.run_bug389_worker(global_source, &self.root.join(GLOBAL_PATH), &abort)?;
        let local_run =
            self.run_bug389_worker(local_source, &self.root.join(LOCAL_PATH), &abort)?;
        if global_run.result.node_names != local_run.result.node_names {
            return Err(format!(
                "{LABEL} relational runs acquired different namespaces"
            ));
        }

        // Release 7.10 stripped TITLE/DATASETAUXDATA/ZONE and performed an
        // exact byte diff on the remaining precision-7 HB.FD.dat content.
        let global_fd = Self::bug389_filtered_tecplot_bytes(&global_run.fd)?;
        let local_fd = Self::bug389_filtered_tecplot_bytes(&local_run.fd)?;
        if global_fd != local_fd {
            return Err(format!(
                "{LABEL} historical precision-7 FD byte diff failed"
            ));
        }

        // The wrapper passed global as TEST and local as GOOD. Preserve that
        // direction because file_compare divides relative error by |GOOD|.
        let test_global = Self::bug389_precision7_table(&global_run.td)?;
        let good_local = Self::bug389_precision7_table(&local_run.td)?;
        let mismatches =
            self.compare_release_7_10_file_compare_tables(&good_local, &test_global, TD_TOLERANCE)?;
        if !mismatches.is_empty() {
            return Err(format!(
                "{LABEL} historical TD TEST=global GOOD=local comparison failed with {} mismatch(es): {:?}",
                mismatches.len(),
                mismatches.first()
            ));
        }

        if abort.is_aborted() {
            return Err(format!("{LABEL} execution exceeded deadline"));
        }
        let after = self.validate_bug389_provenance(deck, &abort)?;
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

    #[test]
    fn bug389_route_is_owner_only() {
        assert_eq!(Bug389Role::for_record(OWNER_PATH), Some(Bug389Role));
        assert_eq!(Bug389Role::for_record(GLOBAL_PATH), None);
        assert_eq!(Bug389Role::for_record(LOCAL_PATH), None);
    }

    #[test]
    fn bug389_historical_and_retained_record_streams_are_sealed() {
        XyceTestRunner::validate_bug389_record_streams().unwrap();
        let abort = DeadlineAbort::new(Instant::now(), 30_000);
        assert_eq!(runner().read_bug389_directory(&abort).unwrap().len(), 3);
    }

    #[test]
    fn bug389_namespace_transform_is_exact_and_counterfactual_sensitive() {
        let root = corpus_root().join(FAMILY_DIRECTORY);
        let global = fs::read_to_string(root.join(GLOBAL_NAME))
            .unwrap()
            .replace("\r\n", "\n");
        let local = fs::read_to_string(root.join(LOCAL_NAME))
            .unwrap()
            .replace("\r\n", "\n");
        XyceTestRunner::bug389_namespace_transform(&global, &local).unwrap();
        assert!(
            XyceTestRunner::bug389_namespace_transform(&global, &local.replace(".param", ".PARAM"))
                .is_err()
        );
        assert!(
            XyceTestRunner::bug389_namespace_transform(
                &global.replacen(".global_param", ".param", 1),
                &local
            )
            .is_err()
        );
    }

    #[test]
    fn bug389_pulse_dft_is_nonvacuous() {
        let spectrum = XyceTestRunner::bug389_pulse_dft();
        assert_eq!(spectrum.len(), HARMONICS + 1);
        assert!(spectrum[0].re > 0.45 && spectrum[0].re < 0.55);
        assert!(spectrum[1].norm() > 0.5);
        assert!(
            spectrum
                .iter()
                .skip(2)
                .map(Complex64::norm_sqr)
                .sum::<Value>()
                > 1.0e-3
        );
    }

    #[test]
    fn bug389_physics_counterfactuals_fail_closed() {
        let runner = runner();
        let path = corpus_root().join(GLOBAL_PATH);
        let source = fs::read_to_string(&path).unwrap().replace("\r\n", "\n");
        let abort = DeadlineAbort::new(Instant::now(), 30_000);
        let run = runner.run_bug389_worker(&source, &path, &abort).unwrap();

        let mut zero = run.result.clone();
        for spectrum in &mut zero.spectral_voltages {
            spectrum.coefficients.fill(Complex64::new(0.0, 0.0));
        }
        assert!(XyceTestRunner::validate_bug389_physics(&zero).is_err());

        let mut wrong_transfer = run.result.clone();
        let output = wrong_transfer
            .spectral_voltages
            .iter_mut()
            .find(|spectrum| spectrum.node_name.eq_ignore_ascii_case("1b"))
            .unwrap();
        output.coefficients[1] *= 0.9;
        assert!(XyceTestRunner::validate_bug389_physics(&wrong_transfer).is_err());

        let mut wrong_node_names = run.result.clone();
        wrong_node_names.node_names[0] = "wrong".to_string();
        assert!(XyceTestRunner::validate_bug389_physics(&wrong_node_names).is_err());

        let mut duplicate_spectrum = run.result.clone();
        let hidden = duplicate_spectrum
            .spectral_voltages
            .iter_mut()
            .find(|spectrum| spectrum.node_name.eq_ignore_ascii_case("X_bigline.3"))
            .unwrap();
        hidden.node_name = "DUT".to_string();
        assert!(XyceTestRunner::validate_bug389_physics(&duplicate_spectrum).is_err());

        let mut wrong_voltage_grid = run.result.clone();
        wrong_voltage_grid.spectral_voltages[0].frequencies[1] += 1.0;
        assert!(XyceTestRunner::validate_bug389_physics(&wrong_voltage_grid).is_err());

        let mut wrong_branch_grid = run.result.clone();
        wrong_branch_grid.mna_branch_currents[0].frequencies[1] += 1.0;
        assert!(XyceTestRunner::validate_bug389_physics(&wrong_branch_grid).is_err());

        let mut wrong_hidden = run.result;
        let hidden = wrong_hidden
            .spectral_voltages
            .iter_mut()
            .find(|spectrum| spectrum.node_name.eq_ignore_ascii_case("DUT"))
            .unwrap();
        hidden.coefficients[1] += Complex64::new(1.0e-3, 0.0);
        assert!(XyceTestRunner::validate_bug389_physics(&wrong_hidden).is_err());
    }

    #[test]
    fn bug389_typed_hb_options_fail_closed() {
        let path = corpus_root().join(GLOBAL_PATH);
        let source = fs::read_to_string(&path).unwrap().replace("\r\n", "\n");
        assert!(XyceTestRunner::validate_bug389_source(&source, &path).is_ok());
        assert!(
            XyceTestRunner::validate_bug389_source(&source.replace("tahb=0", "tahb=1"), &path)
                .is_err()
        );
        assert!(
            XyceTestRunner::validate_bug389_source(
                &source.replace("maxstep=2", "maxstep=3"),
                &path
            )
            .is_err()
        );
    }

    #[test]
    fn bug389_production_circuit_census_rejects_an_extra_device() {
        let path = corpus_root().join(GLOBAL_PATH);
        let source = fs::read_to_string(&path).unwrap().replace("\r\n", "\n");
        let netlist = XyceTestRunner::parse_xyce_netlist(&source, &path).unwrap();
        let engine = runner().create_xyce_engine();
        let circuit = engine.build_circuit(&netlist).unwrap();
        XyceTestRunner::validate_bug389_native_circuit(&circuit).unwrap();

        let changed = source.replace("\n.hb 1e6", "\nRextra DUT 0 1\n.hb 1e6");
        let changed = XyceTestRunner::parse_xyce_netlist(&changed, &path).unwrap();
        let changed = engine.build_circuit(&changed).unwrap();
        assert!(XyceTestRunner::validate_bug389_native_circuit(&changed).is_err());
    }

    #[test]
    fn bug389_precision7_fd_comparison_detects_a_serialized_change() {
        let table = XycePrnTable {
            columns: FD_COLUMNS
                .iter()
                .map(|column| (*column).to_string())
                .collect(),
            rows: (-(HARMONICS as isize)..=HARMONICS as isize)
                .map(|signed| vec![signed as Value * FUNDAMENTAL_HZ, 0.495, 0.0, 0.494, 0.001])
                .collect(),
        };
        let bytes = XyceTestRunner::bug389_filtered_tecplot_bytes(&table).unwrap();
        let rendered = std::str::from_utf8(&bytes).unwrap();
        assert!(rendered.starts_with("\tVARIABLES = \" FREQ\" \n\" Re(V(1))\" \n"));
        assert!(rendered.ends_with("End of Xyce(TM) Simulation\n"));
        assert!(!rendered.contains("TITLE"));
        assert!(!rendered.contains("DATASETAUXDATA"));
        assert!(!rendered.contains("ZONE"));
        let mut changed = table.clone();
        changed.rows[HARMONICS][1] += 1.0e-7;
        let changed = XyceTestRunner::bug389_filtered_tecplot_bytes(&changed).unwrap();
        assert_ne!(bytes, changed);
    }

    #[test]
    fn bug389_td_file_compare_is_good_asymmetric_and_strict() {
        let runner = runner();
        let table = |value| XycePrnTable {
            columns: vec!["TIME".to_string(), "V(1)".to_string()],
            rows: vec![vec![0.0, value]],
        };
        assert!(
            runner
                .compare_release_7_10_file_compare_tables(
                    &table(200.0),
                    &table(200.0 + 5.0e-7),
                    TD_TOLERANCE
                )
                .unwrap()
                .is_empty()
        );
        assert!(
            !runner
                .compare_release_7_10_file_compare_tables(
                    &table(200.0),
                    &table(200.0 + 2.0e-6),
                    TD_TOLERANCE
                )
                .unwrap()
                .is_empty()
        );
        let asymmetric = XyceFileCompareTolerance {
            absolute: 200.0,
            relative: 0.5,
            zero: 0.0,
        };
        assert!(
            runner
                .compare_release_7_10_file_compare_tables(&table(400.0), &table(210.0), asymmetric)
                .unwrap()
                .is_empty()
        );
        assert!(
            !runner
                .compare_release_7_10_file_compare_tables(&table(210.0), &table(400.0), asymmetric)
                .unwrap()
                .is_empty()
        );
    }

    #[test]
    fn bug389_checked_in_owner_executes_complete_oracle() {
        let runner = runner();
        let result = runner.run_test(runner.root.join(OWNER_PATH));
        assert!(result.passed, "BUG389 failed: {:?}", result.error);
        assert_eq!(result.contract, CONTRACT);
    }
}
