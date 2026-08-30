use super::*;
use std::io::Read as _;

const LABEL: &str = "BUG_1035_SON simple RC AC DATA equivalence";
const FAMILY_DIR: &str = "Netlists/Certification_Tests/BUG_1035_SON";
const FAMILY_PREFIX: &str = "netlists/certification_tests/bug_1035_son/";
const EXCLUDE: &str = "Netlists/Certification_Tests/BUG_1035_SON/exclude";
const OWNER_CONTRACT: &str = "bug1035_simple_rc_ac_data_relational_wrapper_owner";
const REFERENCE_CONTRACT: &str = "bug1035_simple_rc_ac_data_baseline_reference";
const RELEASE_COMMIT: &str = "d6e278e371ec2f3df1325dcff4552e585bc7ecc1";
const RELEASE_TAG: &str = "Release-7.10.0";
const PRETRIM_COMMIT: &str = "80115a9277c0ddb3409acceb3d4e745fd11cddd4";
const HISTORICAL_RECORD_BYTES: usize = 1_688;
const HISTORICAL_RECORDS_SHA256: &str =
    "0c7d9143cfc8f77c145bae0853218353490e9574dabc5b1fd90d6139ea3e6e3b";
const HISTORICAL_RECORDS_BLAKE3: &str =
    "dc7189f5b58ed6fbeb754ee30b9609778a2692020d73813c0ccdd2727f9ea624";
const GRID: [Value; 9] = [1.0, 10.0, 100.0, 1e3, 1e4, 1e5, 1e6, 1e7, 1e8];

const HISTORICAL: [(&str, usize, &str, &str); 6] = [
    (
        "Netlists/Certification_Tests/BUG_1035_SON/CMakeLists.txt",
        7_473,
        "88128826a9c50a6cd669bf076415a5b10e72d132736e4914e42fc8be9f976d64",
        "7df29c8d6b496a2cfeda94aab236ddf1f79cb804d351cee86028520befca6fbe",
    ),
    (
        "Netlists/Certification_Tests/BUG_1035_SON/Manifest.txt",
        459,
        "273a423961441cf89c8cf98c31eb1e0835f6ebe72d5161f9b2ddbebf13818006",
        "51cb680f01e2c37f4ae352d3ce9c97886a81eaea62f53bb8cb2ce1a71d0fc11f",
    ),
    (
        "Netlists/Certification_Tests/BUG_1035_SON/RC_simple_data.cir.sh",
        2734,
        "68650dcc91b976c436d5d059964ea4ff4c9c05997c105aa581a8baa784960e35",
        "2dfb4af224b301aba14ec41d1da0e772d5160175c6dd6369a11e827eef7f2f4b",
    ),
    (
        EXCLUDE,
        95,
        "f658fcac04e41e9be8cb8d924e890aee2a2792d09ef0390a90f169eab4028f04",
        "8ae9cbdb88e75a0bdeaf375891b05ef7bc3a54b7ec9fbdda56edf660c4a0958c",
    ),
    (
        "Netlists/Certification_Tests/BUG_1035_SON/tags",
        33,
        "ccbb25126be8e50cee3f06c6391ce5884415fe6dd0c33a17fb1774f2cf62d133",
        "5f511e323047ac3efe4b479178c4b882d9f56387787480307a4ad89ffc4ad959",
    ),
    (
        "TestScripts/ACComparator.pl",
        14308,
        "265c0c24ac886ad44bf3827f2cbe0c0f1c75c80971d5bdb3429e8048b36e1571",
        "6a1c8fdfa65116f6729343a759d172be939eb81a8617cea5a52f3572577ba926",
    ),
];

const RETAINED: [(&str, usize, &str); 10] = [
    (
        "RC_AC_data_expr.cir",
        516,
        "eaaa23b9fb4705405f2b502aaf81d7b4326c7bc9fc46d3443f1a051edb5e1f17",
    ),
    (
        "RC_AC_data_expr2.cir",
        495,
        "51064609bb8cad6f478fde863895e770d54c144bcf51e904d555d0eb49c73d1a",
    ),
    (
        "RC_AC_data_expr3.cir",
        403,
        "9e87671007c4d5bc23c438e02c660380c1109734cecd33d4143c85900d602d0a",
    ),
    (
        "RC_AC_data_expr4_baseline.cir",
        498,
        "200873cf6d53cd7804de6b7987e7e726bb08334c333731ef34254d65bf2e0914",
    ),
    (
        "RC_AC_data_expr4.cir",
        459,
        "0d298e016b3da4782d50b0329e0ccd11041f9504f4fee56b601ddfdc9aeff4e3",
    ),
    (
        "RC_AC_data_exprAlone.cir",
        516,
        "eaaa23b9fb4705405f2b502aaf81d7b4326c7bc9fc46d3443f1a051edb5e1f17",
    ),
    (
        "RC_simple_baseline.cir",
        138,
        "8429901b2d71d1f91f0c6ac9b5cf6bb1e22a723f502a31af47cf8b5757151151",
    ),
    (
        "RC_simple_data.cir",
        314,
        "487b45ef4faea9b126a194eea6211e675a4b64746b330e3ae48ce573c95da596",
    ),
    (
        "RC_simple.cir",
        356,
        "fd3917e0a106b6d27b486dd7eb1875683af604d74c2e3bd2d7ee979b3ecfd4d3",
    ),
    (
        "transLineDataGlobal_AC.cir",
        514,
        "69912d6e59c4a4430cebc8686449a291bfdb12488b99ee7625b89ccbb3260b8c",
    ),
];

const RETAINED_OUTPUTS: [(&str, usize, &str); 3] = [
    (
        "RC_AC_data_exprAlone.cir.FD.prn",
        951,
        "b1d2fa65652988b5e32a869c7568cec1b7a32336fac98f38bdfa996e4a1f2d2d",
    ),
    (
        "RC_simple.cir.FD.prn",
        627,
        "1d594ff856abf3b2ba7e8b01c55e8c249534fb810259b7df529dac3ed32820ae",
    ),
    (
        "transLineDataGlobal_AC.cir.FD.prn",
        819,
        "f0ba5ca0fbedfa70b9c60c80649e40217c028f976da263de045445576e372ca8",
    ),
];

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub(super) enum Bug1035SonRole {
    DataOwner,
    BaselineReference,
}

impl Bug1035SonRole {
    const ALL: [Self; 2] = [Self::DataOwner, Self::BaselineReference];

    pub(super) fn for_record(record: &str) -> Option<Self> {
        let record = XyceTestRunner::normalize_manifest_key(record);
        Self::ALL.into_iter().find(|role| record == role.record())
    }

    fn path(self) -> &'static str {
        match self {
            Self::DataOwner => "Netlists/Certification_Tests/BUG_1035_SON/RC_simple_data.cir",
            Self::BaselineReference => {
                "Netlists/Certification_Tests/BUG_1035_SON/RC_simple_baseline.cir"
            }
        }
    }

    fn record(self) -> &'static str {
        match self {
            Self::DataOwner => "netlists/certification_tests/bug_1035_son/rc_simple_data.cir",
            Self::BaselineReference => {
                "netlists/certification_tests/bug_1035_son/rc_simple_baseline.cir"
            }
        }
    }

    pub(super) fn contract(self) -> &'static str {
        match self {
            Self::DataOwner => OWNER_CONTRACT,
            Self::BaselineReference => REFERENCE_CONTRACT,
        }
    }
}

#[derive(Debug)]
struct Bug1035Worker {
    plan: XyceRelationalAcPlan,
    netlist: Netlist,
}

impl XyceTestRunner {
    fn validate_bug1035_historical(records: &[(&str, usize, &str, &str)]) -> Result<(), String> {
        let mut canonical = records
            .iter()
            .map(|(path, bytes, sha, content_blake3)| {
                format!("{PRETRIM_COMMIT}\t{RELEASE_COMMIT}\t{RELEASE_TAG}\t{path}\t{bytes}\t{sha}\t{content_blake3}")
            })
            .collect::<Vec<_>>();
        canonical.sort();
        let stream = canonical.join("\n");
        let sha = format!("{:x}", Sha256::digest(stream.as_bytes()));
        let b3 = blake3::hash(stream.as_bytes()).to_hex().to_string();
        if PRETRIM_COMMIT != UPSTREAM_EXCLUSIONS_SOURCE_COMMIT
            || RELEASE_COMMIT != "d6e278e371ec2f3df1325dcff4552e585bc7ecc1"
            || RELEASE_TAG != "Release-7.10.0"
            || records.len() != HISTORICAL.len()
            || stream.len() != HISTORICAL_RECORD_BYTES
            || sha != HISTORICAL_RECORDS_SHA256
            || b3 != HISTORICAL_RECORDS_BLAKE3
        {
            return Err(format!(
                "{LABEL} Release wrapper/ACComparator provenance changed: bytes={}, sha={sha}, b3={b3}",
                stream.len()
            ));
        }
        Ok(())
    }

    fn read_bug1035_directory(
        &self,
        abort: &dyn AbortSignal,
    ) -> Result<BTreeMap<String, Vec<u8>>, String> {
        if abort.is_aborted() {
            return Err(format!("{LABEL} retained census aborted"));
        }
        let directory = self.root.join(FAMILY_DIR);
        let metadata = fs::symlink_metadata(&directory)
            .map_err(|error| format!("{LABEL} directory inspection failed: {error}"))?;
        if metadata.file_type().is_symlink() || !metadata.file_type().is_dir() {
            return Err(format!(
                "{LABEL} source directory is not a regular directory"
            ));
        }
        let expected = RETAINED
            .into_iter()
            .map(|record| (record.0.to_ascii_lowercase(), record))
            .collect::<BTreeMap<_, _>>();
        let mut observed = BTreeMap::new();
        for entry in
            fs::read_dir(&directory).map_err(|error| format!("{LABEL} census failed: {error}"))?
        {
            if abort.is_aborted() {
                return Err(format!("{LABEL} retained census aborted"));
            }
            let entry = entry.map_err(|error| format!("{LABEL} member failed: {error}"))?;
            let path = entry.path();
            let metadata = fs::symlink_metadata(&path)
                .map_err(|error| format!("{LABEL} member inspection failed: {error}"))?;
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
            let limit = expected_bytes
                .checked_mul(2)
                .and_then(|n| n.checked_add(3))
                .ok_or_else(|| format!("{LABEL} size bound overflow"))?;
            if metadata.len() > limit as u64 {
                return Err(format!("{LABEL} member {name:?} exceeds bounded envelope"));
            }
            let mut raw = Vec::with_capacity((metadata.len() as usize).min(limit));
            fs::File::open(&path)
                .map_err(|error| format!("{LABEL} open failed: {error}"))?
                .take((limit + 1) as u64)
                .read_to_end(&mut raw)
                .map_err(|error| format!("{LABEL} bounded read failed: {error}"))?;
            if raw.len() > limit || abort.is_aborted() {
                return Err(format!("{LABEL} bounded read grew or aborted"));
            }
            let canonical = Self::canonical_lf_text_identity(LABEL, &raw)?;
            let sha = format!("{:x}", Sha256::digest(&canonical));
            if canonical.len() != expected_bytes || sha != expected_sha {
                return Err(format!(
                    "{LABEL} member {name:?} changed: bytes={}, sha={sha}",
                    canonical.len()
                ));
            }
            observed.insert(key, raw);
        }
        if observed.len() != expected.len() {
            return Err(format!("{LABEL} retained ten-member census changed"));
        }
        Ok(observed)
    }

    fn validate_bug1035_output_directory(&self, abort: &dyn AbortSignal) -> Result<(), String> {
        let directory = self
            .root
            .join("OutputData/Certification_Tests/BUG_1035_SON");
        let metadata = fs::symlink_metadata(&directory)
            .map_err(|error| format!("{LABEL} output directory inspection failed: {error}"))?;
        if metadata.file_type().is_symlink() || !metadata.file_type().is_dir() {
            return Err(format!(
                "{LABEL} output directory is not a regular directory"
            ));
        }
        let expected = RETAINED_OUTPUTS
            .into_iter()
            .map(|record| (record.0.to_ascii_lowercase(), record))
            .collect::<BTreeMap<_, _>>();
        let mut observed = BTreeSet::new();
        for entry in fs::read_dir(&directory)
            .map_err(|error| format!("{LABEL} output census failed: {error}"))?
        {
            if abort.is_aborted() {
                return Err(format!("{LABEL} output census aborted"));
            }
            let entry = entry.map_err(|error| format!("{LABEL} output member failed: {error}"))?;
            let path = entry.path();
            let metadata = fs::symlink_metadata(&path)
                .map_err(|error| format!("{LABEL} output inspection failed: {error}"))?;
            if metadata.file_type().is_symlink() || !metadata.file_type().is_file() {
                return Err(format!(
                    "{LABEL} output member {} is not a regular file",
                    path.display()
                ));
            }
            let name = entry
                .file_name()
                .to_str()
                .ok_or_else(|| format!("{LABEL} output name is not UTF-8"))?
                .to_string();
            let key = name.to_ascii_lowercase();
            let Some((expected_name, expected_bytes, expected_sha)) = expected.get(&key).copied()
            else {
                return Err(format!("{LABEL} acquired unexpected output {name:?}"));
            };
            if name != expected_name || !observed.insert(key) {
                return Err(format!("{LABEL} output case/census changed: {name:?}"));
            }
            let limit = expected_bytes
                .checked_mul(2)
                .and_then(|n| n.checked_add(3))
                .ok_or_else(|| format!("{LABEL} output size bound overflow"))?;
            if metadata.len() > limit as u64 {
                return Err(format!("{LABEL} output {name:?} exceeds bounded envelope"));
            }
            let mut raw = Vec::with_capacity((metadata.len() as usize).min(limit));
            fs::File::open(&path)
                .map_err(|error| format!("{LABEL} output open failed: {error}"))?
                .take((limit + 1) as u64)
                .read_to_end(&mut raw)
                .map_err(|error| format!("{LABEL} bounded output read failed: {error}"))?;
            if raw.len() > limit || abort.is_aborted() {
                return Err(format!("{LABEL} bounded output read grew or aborted"));
            }
            let canonical = Self::canonical_lf_text_identity(LABEL, &raw)?;
            let sha = format!("{:x}", Sha256::digest(&canonical));
            if canonical.len() != expected_bytes || sha != expected_sha {
                return Err(format!(
                    "{LABEL} output {name:?} changed: bytes={}, sha={sha}",
                    canonical.len()
                ));
            }
        }
        if observed.len() != expected.len() {
            return Err(format!("{LABEL} retained three-output census changed"));
        }
        Ok(())
    }

    fn validate_bug1035_provenance(
        &self,
        deck: &XyceDeck,
        role: Bug1035SonRole,
        abort: &dyn AbortSignal,
    ) -> Result<BTreeMap<String, Vec<u8>>, String> {
        Self::validate_bug1035_historical(&HISTORICAL)?;
        if deck.section != XyceDeckSection::Netlists
            || Self::normalize_manifest_key(&deck.relative_path) != role.record()
            || Self::normalize_manifest_key(&self.relative_key(&deck.path)) != role.record()
            || !Self::same_path(&deck.path, &self.root.join(role.path()))
        {
            return Err(format!("{LABEL} recognized role is not canonical"));
        }
        let owners = self
            .upstream_wrapper_decks
            .iter()
            .filter(|record| record.starts_with(FAMILY_PREFIX))
            .map(String::as_str)
            .collect::<BTreeSet<_>>();
        if owners
            != BTreeSet::from([
                "netlists/certification_tests/bug_1035_son/rc_ac_data_expr.cir",
                "netlists/certification_tests/bug_1035_son/rc_ac_data_expr4.cir",
                "netlists/certification_tests/bug_1035_son/rc_ac_data_expralone.cir",
                "netlists/certification_tests/bug_1035_son/rc_simple.cir",
                Bug1035SonRole::DataOwner.record(),
                "netlists/certification_tests/bug_1035_son/translinedataglobal_ac.cir",
            ])
        {
            return Err(format!("{LABEL} wrapper owner census changed: {owners:?}"));
        }
        let exclusions = Self::load_upstream_exclusions(&self.root)
            .map_err(|error| format!("{LABEL} exclusions invalid: {error}"))?;
        let family = exclusions
            .iter()
            .filter(|(record, _)| record.starts_with(FAMILY_PREFIX))
            .collect::<BTreeMap<_, _>>();
        let baseline = family
            .get(&Bug1035SonRole::BaselineReference.record().to_string())
            .copied();
        if family.len() != 4 || baseline.is_none_or(|entry| entry.source != EXCLUDE || !matches!(&entry.disposition, XyceUpstreamExclusionDisposition::RspiceIndependentlyQualified { expected_contract } if expected_contract == REFERENCE_CONTRACT)) {
            return Err(format!("{LABEL} exclusion classification changed: {family:?}"));
        }
        for record in [
            "netlists/certification_tests/bug_1035_son/rc_ac_data_expr2.cir",
            "netlists/certification_tests/bug_1035_son/rc_ac_data_expr3.cir",
            "netlists/certification_tests/bug_1035_son/rc_ac_data_expr4_baseline.cir",
        ] {
            let Some(entry) = family.get(&record.to_string()) else {
                return Err(format!("{LABEL} lost sibling exclusion"));
            };
            if entry.source != EXCLUDE
                || entry.disposition != XyceUpstreamExclusionDisposition::Excluded
            {
                return Err(format!("{LABEL} sibling exclusion changed"));
            }
        }
        if exclusions.contains_key(Bug1035SonRole::DataOwner.record()) {
            return Err(format!("{LABEL} owner became excluded"));
        }
        let members = self.read_bug1035_directory(abort)?;
        self.validate_bug1035_output_directory(abort)?;
        for role in Bug1035SonRole::ALL {
            self.reject_wrapper_output_artifacts(&self.root.join(role.path()))?;
        }
        Ok(members)
    }

    fn bug1035_worker(
        &self,
        role: Bug1035SonRole,
        raw: &[u8],
        abort: &dyn AbortSignal,
    ) -> Result<Bug1035Worker, String> {
        let source = std::str::from_utf8(raw)
            .map_err(|error| format!("{LABEL} source is not UTF-8: {error}"))?
            .to_string();
        if source.lines().any(|line| {
            let line = Self::strip_netlist_comment(line)
                .trim()
                .to_ascii_lowercase();
            line.starts_with(".inc") || line.starts_with(".include") || line.starts_with(".lib")
        }) {
            return Err(format!(
                "{LABEL} unexpectedly acquired a filesystem dependency"
            ));
        }
        let path = self.root.join(role.path());
        let output = Self::canonical_print_output_request(&source, "AC", false)?
            .ok_or_else(|| format!("{LABEL} lost PRINT AC"))?;
        if output.format.is_some() || output.file.is_some() || output.probes != ["v(1)"] {
            return Err(format!("{LABEL} {role:?} PRINT schema changed"));
        }
        let options = NetlistParseOptions {
            statistical_mode: StatisticalParamMode::Nominal,
            expression_dialect: ExpressionDialect::Xyce,
            ..Default::default()
        };
        let netlist =
            Netlist::parse_with_path_and_options_and_abort(&source, &path, options, abort)
                .map_err(|error| format!("{LABEL} {role:?} parse failed: {error}"))?;
        let ac = Self::single_ac_analysis(&netlist)?;
        if !Self::step_commands(&netlist)?.is_empty() {
            return Err(format!("{LABEL} {role:?} acquired a STEP analysis"));
        }
        let plan = XyceRelationalAcPlan {
            deck_path: path,
            source,
            print: XycePrintRequest {
                probes: output.probes,
            },
            ac,
            frequency_bound: false,
        };
        Self::validate_bug1035_worker(role, &plan, &netlist)?;
        Ok(Bug1035Worker { plan, netlist })
    }

    fn validate_bug1035_worker(
        role: Bug1035SonRole,
        plan: &XyceRelationalAcPlan,
        netlist: &Netlist,
    ) -> Result<(), String> {
        if plan.print.probes != ["v(1)"]
            || plan.ac.frequencies.len() != GRID.len()
            || plan
                .ac
                .frequencies
                .iter()
                .zip(GRID)
                .any(|(a, b)| (*a - b).abs() > b * 64.0 * f64::EPSILON)
        {
            return Err(format!("{LABEL} {role:?} plan/grid changed"));
        }
        if netlist.elements.len() != 3
            || netlist.output_requests.len() != 1
            || netlist.analyses.len() != 1
            || !netlist.models.is_empty()
            || !netlist.subcircuits.is_empty()
            || !netlist.params.all_params().is_empty()
            || !netlist.diagnostics.is_empty()
        {
            return Err(format!("{LABEL} {role:?} typed envelope changed"));
        }
        let find = |name: &str| {
            netlist
                .elements
                .iter()
                .find(|element| element.name.eq_ignore_ascii_case(name))
        };
        let nodes = |element: &rspice_core::netlist::Element, expected: &[&str]| {
            element
                .nodes
                .iter()
                .map(String::as_str)
                .eq(expected.iter().copied())
        };
        let source = find("Isrc");
        let resistor = find("R1");
        let capacitor = find("C1");
        if !source.is_some_and(|e| e.provenance == ElementProvenance::Authored && nodes(e, &["1","0"]) && matches!(&e.kind, ElementKind::CurrentSource(rspice_core::netlist::SourceSpec::Ac { magnitude, phase }) if magnitude.to_bits()==1.0f64.to_bits() && phase.to_bits()==0.0f64.to_bits()))
            || !resistor.is_some_and(|e| e.provenance == ElementProvenance::Authored && nodes(e, &["1","0"]) && matches!(&e.kind, ElementKind::Resistor { value, value_expr: None, model: None, instance_params, deferred_params } if value.to_bits()==1000.0f64.to_bits() && instance_params.is_empty() && deferred_params.is_empty()))
            || !capacitor.is_some_and(|e| e.provenance == ElementProvenance::Authored && nodes(e, &["1","0"]) && matches!(&e.kind, ElementKind::Capacitor { value, value_expr: None, initial_voltage: None, model: None, instance_params, deferred_params } if value.to_bits()==2e-6f64.to_bits() && instance_params.is_empty() && deferred_params.is_empty())) {
            return Err(format!("{LABEL} {role:?} topology changed"));
        }
        match role {
            Bug1035SonRole::BaselineReference if plan.ac.data_points().is_none() && netlist.data_tables.is_empty() => {}
            Bug1035SonRole::DataOwner if plan.ac.data_points().is_some_and(|points| points.len()==9 && points.iter().zip(GRID).all(|(point, frequency)| point.frequency.to_bits()==frequency.to_bits() && matches!(point.overrides.as_slice(), [(name, value)] if name.eq_ignore_ascii_case("FREQ") && value.to_bits()==frequency.to_bits()))) && matches!(netlist.data_tables.as_slice(), [table] if table.name.eq_ignore_ascii_case("eric") && table.params == ["FREQ"] && table.rows.len()==9 && table.rows.iter().zip(GRID).all(|(row, f)| row.as_slice()==[f])) => {}
            _ => return Err(format!(
                "{LABEL} {role:?} AC DATA representation changed: points={:?}, tables={:?}",
                plan.ac.data_points(), netlist.data_tables
            )),
        }
        Ok(())
    }

    fn run_bug1035_worker(
        &self,
        role: Bug1035SonRole,
        worker: &Bug1035Worker,
        abort: &dyn AbortSignal,
    ) -> Result<XycePrnTable, String> {
        if abort.is_aborted() {
            return Err(format!("{LABEL} deadline expired before {role:?}"));
        }
        let engine = self.create_xyce_engine();
        let table = match role {
            Bug1035SonRole::BaselineReference => {
                let results = engine
                    .run_ac_with_abort(&worker.netlist, &worker.plan.ac.frequencies, abort)
                    .map_err(|error| format!("{LABEL} baseline failed: {error}"))?;
                Self::ac_family_result_to_prn_table(&worker.plan.print, &worker.netlist, &results)?
            }
            Bug1035SonRole::DataOwner => {
                let (rows, results) = engine
                    .run_ac_data_with_abort(&worker.netlist, "eric", abort)
                    .map_err(|error| format!("{LABEL} DATA owner failed: {error}"))?;
                let points = rows
                    .into_iter()
                    .zip(results)
                    .map(|(netlist, result)| XyceAcDataPointResult { netlist, result })
                    .collect::<Vec<_>>();
                Self::ac_family_data_points_to_prn_table(&worker.plan.print, &points)?
            }
        };
        Self::validate_bug1035_table(role, &table)?;
        Ok(table)
    }

    fn validate_bug1035_table(role: Bug1035SonRole, table: &XycePrnTable) -> Result<(), String> {
        if table.columns != ["Index", "FREQ", "Re(v(1))", "Im(v(1))"] || table.rows.len() != 9 {
            return Err(format!(
                "{LABEL} {role:?} table schema changed: {:?}/{}",
                table.columns,
                table.rows.len()
            ));
        }
        for (index, (row, frequency)) in table.rows.iter().zip(GRID).enumerate() {
            let b = std::f64::consts::TAU * frequency * 2e-6;
            let denominator = 1e-6 + b * b;
            let expected_re = -0.001 / denominator;
            let expected_im = b / denominator;
            // The relational surface is the historical default PRN stream,
            // whose scientific fields carry eight fractional digits. Bind
            // the independently derived complex response within one
            // conservative final printed digit, far inside ACComparator.
            if row.len() != 4
                || row.iter().any(|v| !v.is_finite())
                || row[0].to_bits() != (index as Value).to_bits()
                || (row[1] - frequency).abs() > frequency * 64.0 * f64::EPSILON
                || (row[2] - expected_re).abs() > 5.1e-9 * expected_re.abs()
                || (row[3] - expected_im).abs() > 5.1e-9 * expected_im.abs()
            {
                return Err(format!(
                    "{LABEL} {role:?} analytic row {index} failed: {row:?}"
                ));
            }
        }
        Ok(())
    }

    fn validate_bug1035_relation(
        &self,
        baseline: &XycePrnTable,
        owner: &XycePrnTable,
    ) -> Result<(), String> {
        Self::validate_bug1035_table(Bug1035SonRole::BaselineReference, baseline)?;
        Self::validate_bug1035_table(Bug1035SonRole::DataOwner, owner)?;
        let good =
            Self::xyce_legacy_compact_prn_for_comparison(baseline, &PrintDelimiter::Whitespace)?;
        let test =
            Self::xyce_legacy_compact_prn_for_comparison(owner, &PrintDelimiter::Whitespace)?;
        if good.as_bytes() == test.as_bytes() {
            return Ok(());
        }
        let tolerance = XyceAcComparatorTolerance::new(6e-5, 1e-4, 1e-6, 1e-6)?;
        let mismatches =
            self.compare_ac_comparator_tables_with_tolerance(baseline, owner, tolerance)?;
        if mismatches.is_empty() {
            Ok(())
        } else {
            Err(format!(
                "{LABEL} directional ACComparator found {} mismatch(es)",
                mismatches.len()
            ))
        }
    }

    pub(super) fn validate_bug1035_son_oracle(
        &self,
        deck: &XyceDeck,
        role: Bug1035SonRole,
        start: Instant,
    ) -> Result<(), String> {
        let abort = DeadlineAbort::new(start, self.config.max_time_per_test_ms.max(1));
        let members = self.validate_bug1035_provenance(deck, role, &abort)?;
        let source = |role: Bug1035SonRole| -> Result<&[u8], String> {
            let name = Path::new(role.path())
                .file_name()
                .unwrap()
                .to_string_lossy()
                .to_ascii_lowercase();
            members
                .get(&name)
                .map(Vec::as_slice)
                .ok_or_else(|| format!("{LABEL} lost role {role:?}"))
        };
        let baseline = self.bug1035_worker(
            Bug1035SonRole::BaselineReference,
            source(Bug1035SonRole::BaselineReference)?,
            &abort,
        )?;
        let owner = self.bug1035_worker(
            Bug1035SonRole::DataOwner,
            source(Bug1035SonRole::DataOwner)?,
            &abort,
        )?;
        let baseline_table =
            self.run_bug1035_worker(Bug1035SonRole::BaselineReference, &baseline, &abort)?;
        let owner_table = self.run_bug1035_worker(Bug1035SonRole::DataOwner, &owner, &abort)?;
        self.validate_bug1035_relation(&baseline_table, &owner_table)?;
        self.validate_bug1035_provenance(deck, role, &abort)?;
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    fn runner() -> XyceTestRunner {
        XyceTestRunner::new(
            Path::new(env!("CARGO_MANIFEST_DIR"))
                .join("../..")
                .join("tests/xyce"),
            Default::default(),
        )
    }
    fn deck(role: Bug1035SonRole) -> XyceDeck {
        let runner = runner();
        XyceDeck {
            path: runner.root.join(role.path()),
            section: XyceDeckSection::Netlists,
            relative_path: role.path().into(),
        }
    }
    fn copied_runner() -> (tempfile::TempDir, XyceTestRunner) {
        let canonical = runner();
        let temporary = tempfile::tempdir().unwrap();
        let family = temporary.path().join(FAMILY_DIR);
        fs::create_dir_all(&family).unwrap();
        for (name, ..) in RETAINED {
            fs::copy(
                canonical.root.join(FAMILY_DIR).join(name),
                family.join(name),
            )
            .unwrap();
        }
        let outputs = temporary
            .path()
            .join("OutputData/Certification_Tests/BUG_1035_SON");
        fs::create_dir_all(&outputs).unwrap();
        for (name, ..) in RETAINED_OUTPUTS {
            fs::copy(
                canonical
                    .root
                    .join("OutputData/Certification_Tests/BUG_1035_SON")
                    .join(name),
                outputs.join(name),
            )
            .unwrap();
        }
        let copied = XyceTestRunner::new(temporary.path(), Default::default());
        (temporary, copied)
    }

    #[test]
    fn bug1035_historical_provenance_is_exact() {
        XyceTestRunner::validate_bug1035_historical(&HISTORICAL).unwrap();
        let mut changed = HISTORICAL;
        changed[0].1 += 1;
        assert!(XyceTestRunner::validate_bug1035_historical(&changed).is_err());
    }
    #[test]
    fn bug1035_exact_pair_executes_and_matches_analytic_oracle() {
        let runner = runner();
        runner
            .validate_bug1035_son_oracle(
                &deck(Bug1035SonRole::DataOwner),
                Bug1035SonRole::DataOwner,
                Instant::now(),
            )
            .unwrap();
    }
    #[test]
    fn bug1035_baseline_role_executes_same_pair() {
        let runner = runner();
        runner
            .validate_bug1035_son_oracle(
                &deck(Bug1035SonRole::BaselineReference),
                Bug1035SonRole::BaselineReference,
                Instant::now(),
            )
            .unwrap();
    }
    #[test]
    fn bug1035_data_grid_and_topology_mutations_fail() {
        let runner = runner();
        let abort = DeadlineAbort::new(Instant::now(), 30000);
        let raw = fs::read(runner.root.join(Bug1035SonRole::DataOwner.path())).unwrap();
        let text = String::from_utf8(raw).unwrap();
        for changed in [
            text.replacen("1.00000000e+04", "2.00000000e+04", 1),
            text.replacen("C1 1 0 2e-6", "C1 1 0 3e-6", 1),
            text.replacen("v(1)", "v(0)", 1),
        ] {
            assert!(
                runner
                    .bug1035_worker(Bug1035SonRole::DataOwner, changed.as_bytes(), &abort)
                    .is_err()
            );
        }
    }
    #[test]
    fn bug1035_relation_uses_directional_comparator_fallback_but_rejects_shared_wrong() {
        let runner = runner();
        let rows = GRID
            .into_iter()
            .enumerate()
            .map(|(i, f)| {
                let b = std::f64::consts::TAU * f * 2e-6;
                let d = 1e-6 + b * b;
                vec![i as Value, f, -0.001 / d, b / d]
            })
            .collect();
        let baseline = XycePrnTable {
            columns: vec![
                "Index".into(),
                "FREQ".into(),
                "Re(v(1))".into(),
                "Im(v(1))".into(),
            ],
            rows,
        };
        let mut close = baseline.clone();
        close.rows[8][2] += 1e-18;
        let baseline_bytes = XyceTestRunner::xyce_legacy_compact_prn_for_comparison(
            &baseline,
            &PrintDelimiter::Whitespace,
        )
        .unwrap();
        let close_bytes = XyceTestRunner::xyce_legacy_compact_prn_for_comparison(
            &close,
            &PrintDelimiter::Whitespace,
        )
        .unwrap();
        assert_ne!(
            baseline_bytes, close_bytes,
            "counterfactual must reach historical comparator fallback"
        );
        runner.validate_bug1035_relation(&baseline, &close).unwrap();
        let tolerance = XyceAcComparatorTolerance::new(6e-5, 1e-4, 1e-6, 1e-6).unwrap();
        let mut directional_test = baseline.clone();
        directional_test.rows[8][3] = baseline.rows[8][3] * (1.0 - 0.000_099_995);
        assert!(
            runner
                .compare_ac_comparator_tables_with_tolerance(
                    &baseline,
                    &directional_test,
                    tolerance,
                )
                .unwrap()
                .is_empty(),
            "Release baseline-GOOD, DATA-TEST ordering must pass"
        );
        assert!(
            !runner
                .compare_ac_comparator_tables_with_tolerance(
                    &directional_test,
                    &baseline,
                    tolerance,
                )
                .unwrap()
                .is_empty(),
            "reversing ACComparator's directional denominator must fail"
        );
        let mut wrong = baseline.clone();
        wrong.rows[2][2] += 1.0;
        assert!(runner.validate_bug1035_relation(&wrong, &wrong).is_err());
    }
    #[test]
    fn bug1035_release_comparator_boundaries_are_exact() {
        let runner = runner();
        let table = |frequency: Value, value: Value| XycePrnTable {
            columns: vec!["Index".into(), "FREQ".into(), "VALUE".into()],
            rows: vec![vec![0.0, frequency, value]],
        };

        let absolute = XyceAcComparatorTolerance::new(0.125, 0.25, 0.0, 0.125).unwrap();
        assert!(
            !runner
                .compare_ac_comparator_tables_with_tolerance(
                    &table(1.0, 1.0),
                    &table(1.0, 1.125),
                    absolute,
                )
                .unwrap()
                .is_empty()
        );

        let relative = XyceAcComparatorTolerance::new(0.5, 0.25, 0.0, 0.125).unwrap();
        assert!(
            !runner
                .compare_ac_comparator_tables_with_tolerance(
                    &table(1.0, 1.0),
                    &table(1.0, 1.25),
                    relative,
                )
                .unwrap()
                .is_empty()
        );

        let zero = XyceAcComparatorTolerance::new(0.125, 1.0, 0.5, 0.125).unwrap();
        assert!(
            runner
                .compare_ac_comparator_tables_with_tolerance(
                    &table(1.0, 0.25),
                    &table(1.0, 0.5 - f64::EPSILON),
                    zero,
                )
                .unwrap()
                .is_empty()
        );
        assert!(
            !runner
                .compare_ac_comparator_tables_with_tolerance(
                    &table(1.0, 0.25),
                    &table(1.0, 0.5),
                    zero,
                )
                .unwrap()
                .is_empty()
        );

        let frequency = XyceAcComparatorTolerance::new(0.125, 0.25, 0.0, 0.125).unwrap();
        assert!(
            runner
                .compare_ac_comparator_tables_with_tolerance(
                    &table(1.0, 1.0),
                    &table(1.125, 1.0),
                    frequency,
                )
                .unwrap()
                .is_empty()
        );
        assert!(
            !runner
                .compare_ac_comparator_tables_with_tolerance(
                    &table(1.0, 1.0),
                    &table(f64::from_bits(1.125f64.to_bits() + 1), 1.0),
                    frequency,
                )
                .unwrap()
                .is_empty()
        );
        assert!(
            runner
                .compare_ac_comparator_tables_with_tolerance(
                    &table(0.0, 1.0),
                    &table(-1.0, 1.0),
                    frequency,
                )
                .unwrap()
                .is_empty()
        );
        assert!(
            !runner
                .compare_ac_comparator_tables_with_tolerance(
                    &table(0.0, 1.0),
                    &table(0.25, 1.0),
                    frequency,
                )
                .unwrap()
                .is_empty()
        );
    }
    #[test]
    fn bug1035_data_execution_honors_abort_during_engine_entry() {
        struct AlwaysAbort;
        impl AbortSignal for AlwaysAbort {
            fn is_aborted(&self) -> bool {
                true
            }
        }
        let runner = runner();
        let raw = fs::read(runner.root.join(Bug1035SonRole::DataOwner.path())).unwrap();
        let worker = runner
            .bug1035_worker(
                Bug1035SonRole::DataOwner,
                &raw,
                &rspice_core::abort_signal::NoAbort,
            )
            .unwrap();
        assert!(matches!(
            runner.create_xyce_engine().run_ac_data_with_abort(
                &worker.netlist,
                "eric",
                &AlwaysAbort
            ),
            Err(SimulationError::Aborted)
        ));
    }
    #[test]
    fn bug1035_retained_census_is_exact_and_bounded() {
        let (_temp, copied) = copied_runner();
        copied
            .read_bug1035_directory(&rspice_core::abort_signal::NoAbort)
            .unwrap();
        copied
            .validate_bug1035_output_directory(&rspice_core::abort_signal::NoAbort)
            .unwrap();
        fs::write(copied.root.join(FAMILY_DIR).join("unexpected.cir"), b"x\n").unwrap();
        assert!(
            copied
                .read_bug1035_directory(&rspice_core::abort_signal::NoAbort)
                .is_err()
        );
        let (_temp, copied) = copied_runner();
        fs::write(
            copied
                .root
                .join("OutputData/Certification_Tests/BUG_1035_SON/RC_simple.cir.FD.prn"),
            b"drift\n",
        )
        .unwrap();
        assert!(
            copied
                .validate_bug1035_output_directory(&rspice_core::abort_signal::NoAbort)
                .is_err()
        );
        let (_temp, copied) = copied_runner();
        fs::write(
            copied.root.join(FAMILY_DIR).join("RC_simple_data.cir"),
            vec![b'x'; 1000],
        )
        .unwrap();
        assert!(
            copied
                .read_bug1035_directory(&rspice_core::abort_signal::NoAbort)
                .is_err()
        );
        let (_temp, copied) = copied_runner();
        fs::remove_file(copied.root.join(FAMILY_DIR).join("RC_simple_baseline.cir")).unwrap();
        assert!(
            copied
                .read_bug1035_directory(&rspice_core::abort_signal::NoAbort)
                .is_err()
        );
    }
    #[test]
    fn bug1035_expired_deadline_fails_before_preparation() {
        let canonical = runner();
        let runner = XyceTestRunner::new(
            &canonical.root,
            XyceRunnerConfig {
                max_time_per_test_ms: 1,
                ..Default::default()
            },
        );
        assert!(
            runner
                .validate_bug1035_son_oracle(
                    &deck(Bug1035SonRole::DataOwner),
                    Bug1035SonRole::DataOwner,
                    Instant::now() - Duration::from_secs(1)
                )
                .is_err()
        );
    }
}
