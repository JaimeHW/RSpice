use super::*;
use std::io::Read as _;

const LABEL: &str = "BUG_907_SON nested library equivalence";
const FAMILY_DIRECTORY: &str = "Netlists/Certification_Tests/BUG_907_SON";
const FAMILY_PREFIX: &str = "netlists/certification_tests/bug_907_son/";
const EXCLUSION_SOURCE: &str = "Netlists/Certification_Tests/BUG_907_SON/exclude";
const OWNER_CONTRACT: &str = "bug907_nested_library_relational_wrapper_owner";
const REFERENCE_CONTRACT: &str = "bug907_flat_release_reference";
const UPSTREAM_COMMIT: &str = "d6e278e371ec2f3df1325dcff4552e585bc7ecc1";
const RELEASE_TAG: &str = "Release-7.10.0";

const HISTORICAL: [(&str, usize, &str, &str); 14] = [
    (
        "Netlists/Certification_Tests/BUG_907_SON/CMakeLists.txt",
        3623,
        "da8ebe7ad7624151ec11a13bc389a1f45b8375230ecf3757247687afb6afd483",
        "bb9922527afcc8e0cb898cbf2dbf84fedc2f376e12cfa22d3d14353a8f4a1097",
    ),
    (
        "Netlists/Certification_Tests/BUG_907_SON/Manifest.txt",
        142,
        "298372de9c2cfbe087da95ca52910440ef25ff0ad7613da088e0b5b80cc5e46c",
        "fc163d7f98a487ecaa56a5c4b9f4836093892d4c2c601decf9bd88bb27650fe4",
    ),
    (
        EXCLUSION_SOURCE,
        17,
        "84b9c3988d02b58b6dece2a8bebcedeb9e8315ef60f327256bfbaf6414ea4c76",
        "a65f141cc1bc523a558aea6f1a24eba08d854a5eff8af83b1fd331df9f821566",
    ),
    (
        "Netlists/Certification_Tests/BUG_907_SON/lib3.lib",
        133,
        "6e8bb8dd56a47ab03ec44746eebda82d2a8a424941ecb9f1cf2b61b14d5c1f32",
        "7b1c7770d93918a2e7fe9c5e520a5424a3663089d1f6cba3e6ad98d9add356b3",
    ),
    (
        "Netlists/Certification_Tests/BUG_907_SON/lib4.lib",
        42,
        "12953beb5c67d498da40cb3cac09db93324199847b4a990784c124ebff74ba2a",
        "9075886c18a61d2c240180d65c9dcd49c5ab13111caa3b93d2881d81063fe0ec",
    ),
    (
        "Netlists/Certification_Tests/BUG_907_SON/rlc.cir",
        334,
        "7de49960cfd82210ced63a3ea5617807d240e916f955605d5470f887d6467b96",
        "50d1e6e9361c30b4b5ef47be565c0af5b308f6d7d8845bb62588b308cfc2965a",
    ),
    (
        "Netlists/Certification_Tests/BUG_907_SON/rlc.cir.options",
        14,
        "3c1884f038f4d82fa6e743801d2012ba27c36254c681a9b98373847a856d08b6",
        "b1d67968e7446e26800d83b2f63ab18f63fd84b5b602758b2b2327bbdf15ef3b",
    ),
    (
        "Netlists/Certification_Tests/BUG_907_SON/rlc.cir.sh",
        1475,
        "c9fd3438962eb71aedea247fe944df015e3b9359763cc70d58888508350ac9fb",
        "2dd18713b421c0e0a9704460d0fe22dfe90c47c441252ad7003864ccac1d908e",
    ),
    (
        "Netlists/Certification_Tests/BUG_907_SON/rlc.cir.tags",
        47,
        "2bcd4fda891ee9b05d8807b3db23af0696ecb05b6420e7d02862a2bc91d048fc",
        "af89b368b02e089beb48ac174e19d9a6b51337a9d91eabb5acdf14d57d18937f",
    ),
    (
        "Netlists/Certification_Tests/BUG_907_SON/rlc.lib",
        108,
        "6f136a26e7fccc8aa819511df1060a870c4919be35381176c48fa0bdf1a8dda3",
        "74ab35e853f92938f0a5e6f10779b4151738de42a160e25ce8719dbbcea5b098",
    ),
    (
        "Netlists/Certification_Tests/BUG_907_SON/tags",
        36,
        "88c3499397fa9212893679f70a94925ad87323cb6a2d78802a260ae095b7e1a4",
        "a95e7c82874522350905590332266330955299adf34b4eac14a8d06bdcb417b7",
    ),
    (
        "Netlists/Certification_Tests/BUG_907_SON/test2.cir",
        185,
        "7603160467bb3310461122f23eb734d400864243af44d081d1edebfad4ef6960",
        "bee92e279ec393b9b38848743d0d64aabab28e7ba7dbe165d967c3a68fe638a8",
    ),
    (
        "Netlists/Certification_Tests/BUG_907_SON/test2.cir.sh",
        1553,
        "d8de897a4ea5bf0f7c7760f5dfdd37e8dcbf9f5bba6e7cc1683e066323645ba9",
        "1ff1a753a7db26a1050137d810939020b718fdefd51b8209ebe35527aece7047",
    ),
    (
        "Netlists/Certification_Tests/BUG_907_SON/test2_nolib.cir",
        208,
        "ad23ec0f44242ae5db108e6d6c723b4ef598874174ae9a46a97b6de4d7ba2d1f",
        "dcaa6c0e3215e69645248a44cbaf6c8e42a5bf3295c7c7c7798434e83effab06",
    ),
];
const HISTORICAL_STREAM_BYTES: usize = 3_385;
const HISTORICAL_STREAM_SHA256: &str =
    "d24ba68b72b1d8dc642450593ab23b929655ab5e629349c9d1316d1dde8c771a";
const HISTORICAL_STREAM_BLAKE3: &str =
    "53d37f1ff7fcab564397e5fa690aa7d0cbde9d6ce463e13f01ff38ebe6598ceb";

const RETAINED: [(&str, usize, &str, &str); 7] = [
    (
        "lib3.lib",
        124,
        "c8d8df07d181af693daab3d9894124704d7f6907dff2a04690e3d0e4e87f112d",
        "36e6d919e6daa9cef94e0e226486eb80da2c029b7d527df6ddbd461fac08e96f",
    ),
    (
        "lib4.lib",
        37,
        "dfe8ed277afafdd0f3cc8b38386835437cfb38264e6e76c17f8131440995bb6b",
        "98674e7f4a16a868b0b02df5adfe075c529371f748ec10558eadcd0701334121",
    ),
    (
        "rlc.cir",
        322,
        "a701dd772a64ef40495792bdc5baebff0c72e7a3fca7563c7830aac8e7a14585",
        "6cfcc5dc7b69a73abb002f2e14c3f7b13ddb63be2d12a88cb69b602d93b9eed0",
    ),
    (
        "rlc.cir.options",
        13,
        "381cd29ca4d9097c73fccc5f46cea0c37bd3e71da803e56ccad41d8270de9c0e",
        "8e9c4c362e6a201344f7fd4b55680c6db23a1ba99121d41b9dae7573cff78b81",
    ),
    (
        "rlc.lib",
        102,
        "77bf80b3d4af862d3ba9902e9b2dfecf6a0467cc0c920cb0ec1ddfba217ee72c",
        "1b2356bf2612f88172f07e509fb6206de44bae6293bd454c9a92f42626c65820",
    ),
    (
        "test2.cir",
        168,
        "09840208e3acfdd55d8b6635f587880bd815787b84cec60ec01ac760b4c43011",
        "c778993d45eed958708d307e5a241a2c9c7ea68009f9b50e0b51ed5f0776083a",
    ),
    (
        "test2_nolib.cir",
        190,
        "ff5c1b53df7eb7a58b147bb08bd2301872fecd5ccb0ff82d147d32600acddc43",
        "376cc189dfdc33ff6c1c82f2b3a39790ea11eb02f0e371f36150020b88553fef",
    ),
];

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub(super) enum Bug907SonRole {
    HierarchicalOwner,
    FlatReference,
}

impl Bug907SonRole {
    const ALL: [Self; 2] = [Self::HierarchicalOwner, Self::FlatReference];

    pub(super) fn for_record(record: &str) -> Option<Self> {
        let record = XyceTestRunner::normalize_manifest_key(record);
        Self::ALL.into_iter().find(|role| role.record() == record)
    }

    fn path(self) -> &'static str {
        match self {
            Self::HierarchicalOwner => "Netlists/Certification_Tests/BUG_907_SON/test2.cir",
            Self::FlatReference => "Netlists/Certification_Tests/BUG_907_SON/test2_nolib.cir",
        }
    }

    fn record(self) -> &'static str {
        match self {
            Self::HierarchicalOwner => "netlists/certification_tests/bug_907_son/test2.cir",
            Self::FlatReference => "netlists/certification_tests/bug_907_son/test2_nolib.cir",
        }
    }

    pub(super) fn contract(self) -> &'static str {
        match self {
            Self::HierarchicalOwner => OWNER_CONTRACT,
            Self::FlatReference => REFERENCE_CONTRACT,
        }
    }
}

impl XyceTestRunner {
    fn bug907_member_source<'a>(
        members: &'a BTreeMap<String, Vec<u8>>,
        name: &str,
    ) -> Result<&'a str, String> {
        std::str::from_utf8(
            members
                .get(&name.to_ascii_lowercase())
                .ok_or_else(|| format!("{LABEL} authenticated member {name:?} is missing"))?,
        )
        .map_err(|error| format!("{LABEL} member {name:?} is not UTF-8: {error}"))
    }

    fn bug907_sealed_sources(
        &self,
        members: &BTreeMap<String, Vec<u8>>,
        role: Bug907SonRole,
    ) -> Result<SealedSourceBundle, String> {
        let path = |name: &str| self.root.join(FAMILY_DIRECTORY).join(name);
        let root_name = Path::new(role.path())
            .file_name()
            .and_then(|name| name.to_str())
            .ok_or_else(|| format!("{LABEL} role path is invalid"))?;
        let mut sources = vec![(
            path(root_name),
            Self::bug907_member_source(members, root_name)?.to_string(),
        )];
        let mut edges = Vec::new();
        if role == Bug907SonRole::HierarchicalOwner {
            sources.extend(
                ["lib3.lib", "lib4.lib"]
                    .into_iter()
                    .map(|name| {
                        Ok((
                            path(name),
                            Self::bug907_member_source(members, name)?.to_string(),
                        ))
                    })
                    .collect::<Result<Vec<_>, String>>()?,
            );
            edges.extend([
                SealedSourceEdge {
                    owner: path("test2.cir"),
                    requested_path: "lib3.lib".into(),
                    target: path("lib3.lib"),
                },
                SealedSourceEdge {
                    owner: path("lib3.lib"),
                    requested_path: "lib4.lib".into(),
                    target: path("lib4.lib"),
                },
                SealedSourceEdge {
                    owner: path("lib3.lib"),
                    requested_path: "lib3.lib".into(),
                    target: path("lib3.lib"),
                },
            ]);
        }
        SealedSourceBundle::try_new_with_edges(sources, edges)
            .map_err(|error| format!("{LABEL} sealed source closure is invalid: {error}"))
    }

    fn validate_bug907_historical_provenance() -> Result<(), String> {
        let mut records = HISTORICAL
            .into_iter()
            .map(|(path, bytes, sha, b3)| {
                format!("{UPSTREAM_COMMIT}\t{RELEASE_TAG}\t{path}\t{bytes}\t{sha}\t{b3}")
            })
            .collect::<Vec<_>>();
        records.sort();
        let stream = records.join("\n");
        let sha = format!("{:x}", Sha256::digest(stream.as_bytes()));
        let b3 = blake3::hash(stream.as_bytes()).to_hex().to_string();
        if stream.len() != HISTORICAL_STREAM_BYTES
            || sha != HISTORICAL_STREAM_SHA256
            || b3 != HISTORICAL_STREAM_BLAKE3
        {
            return Err(format!(
                "{LABEL} historical provenance changed: records={}, bytes={}, sha={sha}, b3={b3}",
                records.len(),
                stream.len()
            ));
        }
        Ok(())
    }

    #[cfg(test)]
    fn validate_bug907_directory(&self) -> Result<BTreeMap<String, Vec<u8>>, String> {
        self.validate_bug907_directory_with_abort(&rspice_core::abort_signal::NoAbort)
    }

    fn validate_bug907_directory_with_abort(
        &self,
        abort: &dyn AbortSignal,
    ) -> Result<BTreeMap<String, Vec<u8>>, String> {
        if abort.is_aborted() {
            return Err(format!("{LABEL} provenance validation aborted"));
        }
        let directory = self.root.join(FAMILY_DIRECTORY);
        let metadata = fs::symlink_metadata(&directory)
            .map_err(|error| format!("failed to inspect {LABEL}: {error}"))?;
        if metadata.file_type().is_symlink() || !metadata.file_type().is_dir() {
            return Err(format!("{LABEL} directory is not a regular directory"));
        }
        let expected = RETAINED
            .into_iter()
            .map(|record| (record.0.to_ascii_lowercase(), record))
            .collect::<BTreeMap<_, _>>();
        let mut observed = BTreeMap::new();
        for entry in fs::read_dir(&directory)
            .map_err(|error| format!("failed to enumerate {LABEL}: {error}"))?
        {
            if abort.is_aborted() {
                return Err(format!("{LABEL} provenance validation aborted"));
            }
            let entry =
                entry.map_err(|error| format!("failed to inspect {LABEL} member: {error}"))?;
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
            let Some((expected_name, expected_bytes, expected_sha, expected_b3)) =
                expected.get(&key).copied()
            else {
                return Err(format!("{LABEL} acquired unexpected member {name:?}"));
            };
            if name != expected_name || observed.contains_key(&key) {
                return Err(format!("{LABEL} member case/census changed: {name:?}"));
            }
            let maximum = expected_bytes
                .checked_mul(2)
                .and_then(|n| n.checked_add(3))
                .ok_or_else(|| format!("{LABEL} size bound overflow"))?;
            if metadata.len() > maximum as u64 {
                return Err(format!(
                    "{LABEL} member {name:?} exceeds bounded read envelope"
                ));
            }
            let mut bytes = Vec::with_capacity((metadata.len() as usize).min(maximum));
            fs::File::open(&path)
                .map_err(|error| format!("failed to open {name:?}: {error}"))?
                .take((maximum + 1) as u64)
                .read_to_end(&mut bytes)
                .map_err(|error| format!("failed to read {name:?}: {error}"))?;
            if bytes.len() > maximum {
                return Err(format!("{LABEL} member {name:?} grew during bounded read"));
            }
            if abort.is_aborted() {
                return Err(format!("{LABEL} provenance validation aborted"));
            }
            let canonical = Self::canonical_lf_text_identity(LABEL, &bytes)?;
            let sha = format!("{:x}", Sha256::digest(&canonical));
            let b3 = blake3::hash(&canonical).to_hex().to_string();
            if canonical.len() != expected_bytes || sha != expected_sha || b3 != expected_b3 {
                return Err(format!(
                    "{LABEL} member {name:?} changed: bytes={}, sha={sha}, b3={b3}",
                    canonical.len()
                ));
            }
            observed.insert(key, bytes);
        }
        if observed.len() != expected.len() {
            return Err(format!("{LABEL} retained seven-member census changed"));
        }
        Ok(observed)
    }

    fn validate_bug907_provenance(
        &self,
        deck: &XyceDeck,
        role: Bug907SonRole,
        abort: &dyn AbortSignal,
    ) -> Result<BTreeMap<String, Vec<u8>>, String> {
        if abort.is_aborted() {
            return Err(format!("{LABEL} provenance validation aborted"));
        }
        Self::validate_bug907_historical_provenance()?;
        if deck.section != XyceDeckSection::Netlists
            || Self::normalize_manifest_key(&deck.relative_path) != role.record()
            || Self::normalize_manifest_key(&self.relative_key(&deck.path)) != role.record()
            || !Self::same_path(&deck.path, &self.root.join(role.path()))
        {
            return Err(format!("recognized {LABEL} role is not canonical"));
        }
        let owners = self
            .upstream_wrapper_decks
            .iter()
            .filter(|record| record.starts_with(FAMILY_PREFIX))
            .map(String::as_str)
            .collect::<BTreeSet<_>>();
        if owners
            != BTreeSet::from([
                "netlists/certification_tests/bug_907_son/rlc.cir",
                Bug907SonRole::HierarchicalOwner.record(),
            ])
        {
            return Err(format!("{LABEL} owner census changed: {owners:?}"));
        }
        let exclusions = Self::load_upstream_exclusions(&self.root)
            .map_err(|error| format!("{LABEL} exclusions invalid: {error}"))?;
        if abort.is_aborted() {
            return Err(format!("{LABEL} provenance validation aborted"));
        }
        if exclusions.contains_key(Bug907SonRole::HierarchicalOwner.record())
            || exclusions.contains_key("netlists/certification_tests/bug_907_son/rlc.cir")
        {
            return Err(format!("{LABEL} wrapper owner became excluded"));
        }
        let family = exclusions
            .iter()
            .filter(|(record, _)| record.starts_with(FAMILY_PREFIX))
            .collect::<Vec<_>>();
        if !matches!(family.as_slice(), [(record, q)]
            if record.as_str() == Bug907SonRole::FlatReference.record()
                && q.source == EXCLUSION_SOURCE
                && matches!(&q.disposition, XyceUpstreamExclusionDisposition::RspiceIndependentlyQualified { expected_contract } if expected_contract == REFERENCE_CONTRACT))
        {
            return Err(format!("{LABEL} exclusion disposition changed: {family:?}"));
        }
        let members = self.validate_bug907_directory_with_abort(abort)?;
        for role in Bug907SonRole::ALL {
            self.reject_wrapper_output_artifacts(&self.root.join(role.path()))?;
        }
        if self
            .root
            .join("OutputData/Certification_Tests/BUG_907_SON")
            .exists()
        {
            return Err(format!("{LABEL} must not acquire invented numerical gold"));
        }
        Ok(members)
    }

    fn bug907_nodes(actual: &[String], expected: &[&str]) -> bool {
        actual.len() == expected.len()
            && actual
                .iter()
                .zip(expected)
                .all(|(a, e)| a.eq_ignore_ascii_case(e))
    }

    fn validate_bug907_worker(
        &self,
        role: Bug907SonRole,
        source: &str,
        path: &Path,
        sealed_sources: SealedSourceBundle,
        abort: &dyn AbortSignal,
    ) -> Result<XyceStaticDcPlan, String> {
        let plan = self.static_dc_plan_for_source_with_sealed_sources_and_abort(
            path,
            source.to_string(),
            ExpressionDialect::Xyce,
            sealed_sources.clone(),
            abort,
        )?;
        if plan.deck_path != path
            || plan.execution_dir.is_some()
            || plan.sealed_sources.is_none()
            || plan.expression_dialect != ExpressionDialect::Xyce
            || plan.parameter_redefinition_policy != ParameterRedefinitionPolicy::UseLast
            || !plan.diagnostics.is_empty()
            || plan.print_format.is_some()
            || plan.dc_data.is_some()
            || !plan.steps.is_empty()
            || !plan.dc.source.eq_ignore_ascii_case("VG")
            || plan.dc.mode != DcSweepMode::Linear
            || plan.dc.start.to_bits() != 0.0f64.to_bits()
            || plan.dc.stop.to_bits() != 1.8f64.to_bits()
            || plan.dc.step.to_bits() != 0.01f64.to_bits()
            || plan.dc.sweep2.is_some()
            || !plan
                .print
                .probes
                .iter()
                .map(String::as_str)
                .eq(["i(vd)", "i(vf)"])
        {
            return Err(format!("{LABEL} {role:?} static plan changed: {plan:?}"));
        }
        let options = NetlistParseOptions {
            statistical_mode: StatisticalParamMode::Nominal,
            expression_dialect: ExpressionDialect::Xyce,
            parameter_redefinition_policy: ParameterRedefinitionPolicy::UseLast,
            parameter_redefinition_diagnostic_policy:
                rspice_core::netlist::ParameterRedefinitionDiagnosticPolicy::Silent,
            ..NetlistParseOptions::default()
        };
        let netlist = Netlist::parse_with_path_and_sealed_sources_and_options_and_abort(
            &plan.source,
            path,
            sealed_sources,
            options,
            abort,
        )
        .map_err(|error| format!("{LABEL} {role:?} Xyce parse failed: {error}"))?;
        if abort.is_aborted() {
            return Err(format!("{LABEL} {role:?} preparation exceeded deadline"));
        }
        if netlist.params.get("X").map(Value::to_bits) != Some(1.0f64.to_bits())
            || netlist.params.get("X2").map(Value::to_bits) != Some((1.0f64 / 3.0).to_bits())
            || netlist.params.all_params().len() != 2
            || !netlist.params.all_parameter_expressions().is_empty()
        {
            return Err(format!(
                "{LABEL} {role:?} parameter snapshot changed: {:?}",
                netlist.params.all_params()
            ));
        }
        if netlist.elements.len() != 5
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
            || !netlist.params.all_global_expressions().is_empty()
            || !netlist.params.all_string_params().is_empty()
            || !netlist.params.all_functions().is_empty()
        {
            return Err(format!("{LABEL} {role:?} typed envelope changed"));
        }
        let find = |name: &str| {
            let mut matches = netlist
                .elements
                .iter()
                .filter(|element| element.name.eq_ignore_ascii_case(name));
            let first = matches.next();
            if matches.next().is_some() {
                None
            } else {
                first
            }
        };
        let voltage = |name: &str, nodes: &[&str], value: Value| {
            find(name).is_some_and(|element|
            element.provenance == ElementProvenance::Authored && Self::bug907_nodes(&element.nodes, nodes)
                && matches!(&element.kind, ElementKind::VoltageSource(rspice_core::netlist::SourceSpec::Dc(actual)) if actual.to_bits() == value.to_bits()))
        };
        let resistor = |name: &str, nodes: &[&str], value: Value| {
            find(name).is_some_and(|element|
            element.provenance == ElementProvenance::Authored && Self::bug907_nodes(&element.nodes, nodes)
                && matches!(&element.kind, ElementKind::Resistor { value: actual, value_expr: None, model: None, instance_params, deferred_params }
                    if actual.to_bits() == value.to_bits() && instance_params.is_empty() && deferred_params.is_empty()))
        };
        if !voltage("VD", &["D", "0"], 1.8)
            || !voltage("VG", &["G", "0"], 0.0)
            || !voltage("VF", &["F", "0"], 1.5)
            || !resistor("R1", &["G", "D"], 1_000.0)
            || !resistor("R2", &["G", "F"], 2_000.0)
        {
            return Err(format!(
                "{LABEL} {role:?} exact topology changed: {:?}",
                netlist.elements
            ));
        }
        if role == Bug907SonRole::HierarchicalOwner
            && (!source.contains(".param x2  = '(x)/3.0'")
                || !source.contains(".lib lib3.lib lib4_top"))
        {
            return Err(format!(
                "{LABEL} owner lost deferred selected-LIB representation"
            ));
        }
        if !matches!(&netlist.analyses[0], AnalysisCommand::Dc { source, start, stop, step, mode: DcSweepMode::Linear, sweep2: None }
            if source.eq_ignore_ascii_case("VG") && start.to_bits() == 0.0f64.to_bits()
                && stop.to_bits() == 1.8f64.to_bits() && step.to_bits() == 0.01f64.to_bits())
        {
            return Err(format!("{LABEL} typed DC command changed"));
        }
        let request = &netlist.output_requests[0];
        if request.directive != OutputDirectiveKind::Print
            || request.analysis != Some(rspice_core::netlist::OutputAnalysisKind::Dc)
            || request.name.is_some()
            || !request.expressions.is_empty()
            || request.dependencies.len() != 2
            || request
                .dependencies
                .iter()
                .zip(["VD", "VF"])
                .any(|(dependency, name)| {
                    dependency.kind != OutputSymbolKind::Device
                        || !dependency.operator.eq_ignore_ascii_case("I")
                        || !dependency.symbol.eq_ignore_ascii_case(name)
                })
        {
            return Err(format!("{LABEL} {role:?} typed PRINT changed: {request:?}"));
        }
        Ok(plan)
    }

    fn validate_bug907_table(role: Bug907SonRole, table: &XycePrnTable) -> Result<(), String> {
        if table.columns != ["Index", "i(vd)", "i(vf)"] || table.rows.len() != 181 {
            return Err(format!(
                "{LABEL} {role:?} PRN shape changed: {:?}/{}",
                table.columns,
                table.rows.len()
            ));
        }
        for (index, row) in table.rows.iter().enumerate() {
            let sweep = index as Value * 0.01;
            if row.len() != 3
                || row.iter().any(|value| !value.is_finite())
                || row[0].to_bits() != (index as Value).to_bits()
                || (row[1] - (sweep - 1.8) / 1_000.0).abs() > 5e-13
                || (row[2] - (sweep - 1.5) / 2_000.0).abs() > 5e-13
            {
                return Err(format!(
                    "{LABEL} {role:?} row {index} violates analytic current gate: {row:?}"
                ));
            }
        }
        Ok(())
    }

    fn run_bug907_worker(
        &self,
        role: Bug907SonRole,
        plan: &XyceStaticDcPlan,
        abort: &DeadlineAbort,
    ) -> Result<XycePrnTable, String> {
        let (netlist, results) = self
            .run_static_dc_results_with_abort(plan, abort)
            .map_err(|error| format!("{LABEL} {role:?} failed: {error}"))?;
        let table = self.dc_results_to_prn_table(plan, &netlist, &results)?;
        Self::validate_bug907_table(role, &table)?;
        Ok(table)
    }

    fn validate_bug907_relation(flat: &XycePrnTable, owner: &XycePrnTable) -> Result<(), String> {
        Self::validate_bug907_table(Bug907SonRole::FlatReference, flat)?;
        Self::validate_bug907_table(Bug907SonRole::HierarchicalOwner, owner)?;
        let flat = Self::xyce_prn_text_with_delimiter(flat, &PrintDelimiter::Whitespace)?;
        let owner = Self::xyce_prn_text_with_delimiter(owner, &PrintDelimiter::Whitespace)?;
        if flat.as_bytes() != owner.as_bytes() {
            return Err(format!("{LABEL} Release wrapper byte diff failed"));
        }
        Ok(())
    }

    pub(super) fn validate_bug907_son_oracle(
        &self,
        deck: &XyceDeck,
        role: Bug907SonRole,
        start: Instant,
    ) -> Result<(), String> {
        let abort = DeadlineAbort::new(start, self.config.max_time_per_test_ms.max(1));
        if abort.is_aborted() {
            return Err(format!("{LABEL} deadline expired before validation"));
        }
        let members = self.validate_bug907_provenance(deck, role, &abort)?;
        let mut plans = BTreeMap::new();
        for worker in Bug907SonRole::ALL {
            let name = Path::new(worker.path())
                .file_name()
                .and_then(|name| name.to_str())
                .ok_or_else(|| format!("{LABEL} lost {worker:?}"))?;
            let source = Self::bug907_member_source(&members, name)?;
            let sealed_sources = self.bug907_sealed_sources(&members, worker)?;
            plans.insert(
                worker,
                self.validate_bug907_worker(
                    worker,
                    source,
                    &self.root.join(worker.path()),
                    sealed_sources,
                    &abort,
                )?,
            );
        }
        if abort.is_aborted() {
            return Err(format!("{LABEL} validation exceeded deadline"));
        }
        let flat = self.run_bug907_worker(
            Bug907SonRole::FlatReference,
            plans.get(&Bug907SonRole::FlatReference).unwrap(),
            &abort,
        )?;
        let owner = self.run_bug907_worker(
            Bug907SonRole::HierarchicalOwner,
            plans.get(&Bug907SonRole::HierarchicalOwner).unwrap(),
            &abort,
        )?;
        Self::validate_bug907_relation(&flat, &owner)?;
        self.validate_bug907_provenance(deck, role, &abort)?;
        if abort.is_aborted() {
            return Err(format!("{LABEL} final provenance exceeded deadline"));
        }
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn canonical_runner() -> XyceTestRunner {
        let root = Path::new(env!("CARGO_MANIFEST_DIR"))
            .join("../..")
            .join("tests/xyce");
        XyceTestRunner::new(root, XyceRunnerConfig::default())
    }

    fn copied_family() -> (tempfile::TempDir, XyceTestRunner) {
        let canonical = canonical_runner();
        let temporary = tempfile::tempdir().unwrap();
        let family = temporary.path().join(FAMILY_DIRECTORY);
        fs::create_dir_all(&family).unwrap();
        for (name, ..) in RETAINED {
            fs::copy(
                canonical.root.join(FAMILY_DIRECTORY).join(name),
                family.join(name),
            )
            .unwrap();
        }
        let runner = XyceTestRunner::new(temporary.path(), XyceRunnerConfig::default());
        (temporary, runner)
    }

    #[test]
    fn bug907_xyce_file_parse_resolves_later_nested_library_parameter() {
        let runner = canonical_runner();
        let members = runner.validate_bug907_directory().unwrap();
        let abort = DeadlineAbort::new(Instant::now(), 30_000);
        for role in Bug907SonRole::ALL {
            let path = runner.root.join(role.path());
            let name = path.file_name().unwrap().to_str().unwrap();
            let source = XyceTestRunner::bug907_member_source(&members, name).unwrap();
            let sealed = runner.bug907_sealed_sources(&members, role).unwrap();
            runner
                .validate_bug907_worker(role, source, &path, sealed, &abort)
                .expect("sealed Xyce parse resolves X/X2 through selected nested LIB");
        }
    }

    #[test]
    fn bug907_historical_provenance_is_exact() {
        XyceTestRunner::validate_bug907_historical_provenance().unwrap();
    }

    #[test]
    fn bug907_nested_library_and_topology_counterfactuals_fail_closed() {
        let runner = canonical_runner();
        let path = runner.root.join(Bug907SonRole::HierarchicalOwner.path());
        let members = runner.validate_bug907_directory().unwrap();
        let source = XyceTestRunner::bug907_member_source(&members, "test2.cir").unwrap();
        for mutation in [
            source.replacen("lib4_top", "lib4_B", 1),
            source.replacen("'(x)/3.0'", "'(x)/4.0'", 1),
            source.replacen("r1 g d 1k", "r1 g d 2k", 1),
        ] {
            let mut mutated_members = members.clone();
            mutated_members.insert("test2.cir".into(), mutation.as_bytes().to_vec());
            let sealed = runner
                .bug907_sealed_sources(&mutated_members, Bug907SonRole::HierarchicalOwner)
                .unwrap();
            assert!(
                runner
                    .validate_bug907_worker(
                        Bug907SonRole::HierarchicalOwner,
                        &mutation,
                        &path,
                        sealed,
                        &DeadlineAbort::new(Instant::now(), 30_000),
                    )
                    .is_err()
            );
        }
    }

    #[test]
    fn bug907_exact_prn_relation_rejects_shared_wrong_and_byte_drift() {
        let rows = (0..=180)
            .map(|index| {
                let sweep = index as Value * 0.01;
                vec![
                    index as Value,
                    (sweep - 1.8) / 1_000.0,
                    (sweep - 1.5) / 2_000.0,
                ]
            })
            .collect::<Vec<_>>();
        let baseline = XycePrnTable {
            columns: vec!["Index".into(), "i(vd)".into(), "i(vf)".into()],
            rows,
        };
        XyceTestRunner::validate_bug907_relation(&baseline, &baseline).unwrap();
        let mut shared_wrong = baseline.clone();
        shared_wrong.rows[50][1] += 1e-5;
        assert!(XyceTestRunner::validate_bug907_relation(&shared_wrong, &shared_wrong).is_err());
        let mut byte_drift = baseline.clone();
        byte_drift.rows[180][1] += 1e-13;
        XyceTestRunner::validate_bug907_table(Bug907SonRole::HierarchicalOwner, &byte_drift)
            .expect("counterfactual remains inside the independent analytic tolerance");
        assert!(XyceTestRunner::validate_bug907_relation(&baseline, &byte_drift).is_err());
    }

    #[test]
    fn bug907_retained_census_and_bounded_read_fail_closed() {
        let (_temporary, runner) = copied_family();
        let family = runner.root.join(FAMILY_DIRECTORY);
        runner.validate_bug907_directory().unwrap();
        fs::write(family.join("lib4.lib"), vec![b'X'; 88]).unwrap();
        assert!(runner.validate_bug907_directory().is_err());

        let (_temporary, runner) = copied_family();
        let family = runner.root.join(FAMILY_DIRECTORY);
        fs::remove_file(family.join("lib4.lib")).unwrap();
        assert!(runner.validate_bug907_directory().is_err());

        let (_temporary, runner) = copied_family();
        let family = runner.root.join(FAMILY_DIRECTORY);
        fs::write(family.join("unexpected.txt"), b"not retained\n").unwrap();
        assert!(runner.validate_bug907_directory().is_err());

        let (_temporary, runner) = copied_family();
        let family = runner.root.join(FAMILY_DIRECTORY);
        fs::rename(family.join("lib4.lib"), family.join("LIB4.lib")).unwrap();
        assert!(runner.validate_bug907_directory().is_err());

        let (_temporary, runner) = copied_family();
        let family = runner.root.join(FAMILY_DIRECTORY);
        fs::remove_file(family.join("lib4.lib")).unwrap();
        #[cfg(unix)]
        {
            std::os::unix::fs::symlink("lib3.lib", family.join("lib4.lib")).unwrap();
            assert!(runner.validate_bug907_directory().is_err());
        }
        #[cfg(windows)]
        if std::os::windows::fs::symlink_file("lib3.lib", family.join("lib4.lib")).is_ok() {
            assert!(runner.validate_bug907_directory().is_err());
        }
    }

    #[test]
    fn bug907_sealed_plan_never_executes_live_library_drift() {
        let (_temporary, runner) = copied_family();
        let family = runner.root.join(FAMILY_DIRECTORY);
        let members = runner.validate_bug907_directory().unwrap();
        let path = family.join("test2.cir");
        let source = XyceTestRunner::bug907_member_source(&members, "test2.cir").unwrap();
        let sealed = runner
            .bug907_sealed_sources(&members, Bug907SonRole::HierarchicalOwner)
            .unwrap();
        fs::write(
            family.join("lib4.lib"),
            ".lib lib4_A\nr2 g f 9k\n.endl lib4_A\n",
        )
        .unwrap();
        let start = Instant::now();
        let plan = runner
            .validate_bug907_worker(
                Bug907SonRole::HierarchicalOwner,
                source,
                &path,
                sealed,
                &DeadlineAbort::new(start, 30_000),
            )
            .expect("authenticated bundle, not changed live library, prepares the plan");
        runner
            .run_bug907_worker(
                Bug907SonRole::HierarchicalOwner,
                &plan,
                &DeadlineAbort::new(start, 30_000),
            )
            .expect("authenticated bundle, not changed live library, executes");
        assert!(
            runner.validate_bug907_directory().is_err(),
            "persistent live drift remains rejected by final provenance"
        );
    }

    #[test]
    fn bug907_expired_deadline_aborts_sealed_preparation() {
        let runner = canonical_runner();
        let members = runner.validate_bug907_directory().unwrap();
        let path = runner.root.join(Bug907SonRole::HierarchicalOwner.path());
        let source = XyceTestRunner::bug907_member_source(&members, "test2.cir").unwrap();
        let sealed = runner
            .bug907_sealed_sources(&members, Bug907SonRole::HierarchicalOwner)
            .unwrap();
        let error = runner
            .validate_bug907_worker(
                Bug907SonRole::HierarchicalOwner,
                source,
                &path,
                sealed,
                &DeadlineAbort::new(Instant::now() - Duration::from_secs(1), 1),
            )
            .expect_err("expired shared deadline must abort during sealed preparation");
        assert!(
            error.contains("aborted"),
            "unexpected deadline error: {error}"
        );
    }

    #[test]
    fn bug907_executes_flat_then_nested_and_matches_exact_prn() {
        let root = Path::new(env!("CARGO_MANIFEST_DIR"))
            .join("../..")
            .join("tests/xyce");
        let runner = XyceTestRunner::new(&root, XyceRunnerConfig::default());
        let deck = XyceDeck {
            path: root.join(Bug907SonRole::HierarchicalOwner.path()),
            section: XyceDeckSection::Netlists,
            relative_path: Bug907SonRole::HierarchicalOwner.path().to_string(),
        };
        runner
            .validate_bug907_son_oracle(&deck, Bug907SonRole::HierarchicalOwner, Instant::now())
            .unwrap();
    }

    #[test]
    fn bug907_shared_deadline_fails_closed_before_execution() {
        let root = Path::new(env!("CARGO_MANIFEST_DIR"))
            .join("../..")
            .join("tests/xyce");
        let runner = XyceTestRunner::new(
            &root,
            XyceRunnerConfig {
                max_time_per_test_ms: 1,
                ..Default::default()
            },
        );
        let deck = XyceDeck {
            path: root.join(Bug907SonRole::HierarchicalOwner.path()),
            section: XyceDeckSection::Netlists,
            relative_path: Bug907SonRole::HierarchicalOwner.path().into(),
        };
        assert!(
            runner
                .validate_bug907_son_oracle(
                    &deck,
                    Bug907SonRole::HierarchicalOwner,
                    Instant::now() - Duration::from_secs(1)
                )
                .is_err()
        );
    }
}
