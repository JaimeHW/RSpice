use super::*;

const LABEL: &str = "TIA/TRAP passive analytic RC wrappers";
const UPSTREAM_COMMIT: &str = "d6e278e371ec2f3df1325dcff4552e585bc7ecc1";
const UPSTREAM_TAG: &str = "Release-7.10.0";
const HISTORICAL_RECORD_COUNT: usize = 19;
const HISTORICAL_RECORD_BYTES: usize = 4_410;
const HISTORICAL_RECORDS_SHA256: &str =
    "fca506f9b29831616bcbe6b649671d952db925d72376f50cd7e6d1caa3eae4f9";
const HISTORICAL_RECORDS_BLAKE3: &str =
    "9fe50bad856c7f6e3bccf7f25fb025ef70f499404e11f3ccbdeee0addedfc5df";

const HISTORICAL_ARTIFACTS: [(&str, usize, &str, &str); 19] = [
    (
        "Netlists/TIA/TRAP/CAPACITOR/CMakeLists.txt",
        2_682,
        "f577b52017ccf1d0d51b2bfd39d5acf112633544e5e42187f848d2fac981bc7c",
        "49e20a6ded53683fedfc5a7b512ebacd82faf2757c24177d3068a6cdf03e23f8",
    ),
    (
        "Netlists/TIA/TRAP/CAPACITOR/Manifest.txt",
        129,
        "3ec8a9c997322cb4ace1617b294d5a5dc9ad1e10d44956390294b0e97a60cc6d",
        "6ffe1093c96df23d81a33b8ac53368f338bad85a78882aee7cb86fb8ea611ef0",
    ),
    (
        "Netlists/TIA/TRAP/CAPACITOR/capacitor.cir",
        1_280,
        "d8ce29f005843f2f3efd7ff2b798e674cbd54dc3ada232c8700e09bd41ee7ef5",
        "f36ed76968921c6c6d5c5a8a5ce351e55d05d5fb13167ce84b6b4bccdb3a0308",
    ),
    (
        "Netlists/TIA/TRAP/CAPACITOR/capacitor.cir.prn.gs.pl",
        367,
        "b56529ea01c865f9beac05da6c94a19add68725d175992e0ef22a99c2efda882",
        "7727ad358505c9df29d85b96d027d2e4a9b672f5edacba59a88e806528bd6533",
    ),
    (
        "Netlists/TIA/TRAP/CAPACITOR/capacitor.cir.sh",
        1_342,
        "2bb6e2fc7068486aee219f6ec6ea9ec643f888f0c90ecc2b78b09d09299a69c1",
        "4586c6bb166c67ee4a3e33f16b20abf6e6a6d8ea03bbfe2abd0cf7ef3549a2ad",
    ),
    (
        "Netlists/TIA/TRAP/CAPACITOR/exclude",
        16,
        "6f9cc8bee018222501909351b354839778354132d16493f60dda6cd480ed7797",
        "d3e52792bd83f0b16d6e50460f09e5c3b75cd2f33b155e9581df4824d8423293",
    ),
    (
        "Netlists/TIA/TRAP/CAPACITOR/tags",
        26,
        "1d1984e592cfc74228c8d4eece0d61620a466439b5ed1bf453fa0b20610eb821",
        "fd0fa63b01ce48971bbc4e0ad6b49aa07f42c7bb5fc734629656559d6b11b6f7",
    ),
    (
        "Netlists/TIA/TRAP/IC_AND_NODESET/CMakeLists.txt",
        6_812,
        "b1f289667fac6a82e49391719aac7ec3c87adbb2c8a0bd843909e4ee3b398311",
        "17375ed4c18007eb1905b5862312b539b1dfc69ad09e1e517edb6fdab667e993",
    ),
    (
        "Netlists/TIA/TRAP/IC_AND_NODESET/Manifest.txt",
        213,
        "942e7d3f7999e369785ae1074366c62ebd57dcb34db8c43f8acefb20d19608d8",
        "90634732a88e8057e94d2ba797b68a6930afcc933f934d7f356bc2a57a594c26",
    ),
    (
        "Netlists/TIA/TRAP/IC_AND_NODESET/exclude",
        24,
        "994ed79befdf67eb81d5b94104e1f8abefbd7d3a8b235cccf3ce3616baea2d91",
        "f3d97685b515e3d6b9a0ace3f0e363910d57404dbc31af3fd447b42259a46aef",
    ),
    (
        "Netlists/TIA/TRAP/IC_AND_NODESET/ic_cap2.cir",
        966,
        "3dada86ba38092d963cab254b5343aaceb68b5154500028df4af0dcd9b56b5b3",
        "9f4b40b859902b9c2de7b47546158d56311937918f2c7b40734f903380e7e240",
    ),
    (
        "Netlists/TIA/TRAP/IC_AND_NODESET/ic_cap2.cir.prn.gs.pl",
        363,
        "de9c79302cb02b920bf8a93a542b31c2bcd74d7c5e90f615be56855824b1e461",
        "f97d4457cda1e9ada650ffa1521dd1244aa4d56f6d01d0f2c293063fb74e6b5d",
    ),
    (
        "Netlists/TIA/TRAP/IC_AND_NODESET/ic_cap2.cir.sh",
        1_336,
        "07aec856c8ef6c004f69b889a320d1e09afb349cdcc42f9d213d31cf02fb3711",
        "66dd92b3d2a086e45cbac633ddbcc17fbe4c4a2aa14cda54d380e821ecf0fc9b",
    ),
    (
        "Netlists/TIA/TRAP/IC_AND_NODESET/ic_cap2.cir.tags",
        30,
        "604d3cb23e0ede97a72e4ed1a4e2df53cd3284f2ad65e1a5e395f72bd7fd08ad",
        "5495a67c7606cc3fbe496186efb83be3511b1533f0bb075a675a5910e9ff7f82",
    ),
    (
        "Netlists/TIA/TRAP/IC_AND_NODESET/tags",
        36,
        "52b1d1526b1512b357f9b96420ef209e8832cb3b0ed68cd7d85cbdd9e8ae9c6e",
        "c149bd907147e667c2fe3284b14680b3f0fb5f8a0794cd49fab9e10b15c9b5d1",
    ),
    (
        "TestScripts/XyceVerify/DCSources.pm",
        2_739,
        "b2ddcab5ad5a89c428b9b4430190fa27ef7106da7e7afeb31452c81890a9a006",
        "0905f9dc79d7c5bdbe17e3c2360cd063d6fcbf41823a410f98b236783d109ad7",
    ),
    (
        "TestScripts/XyceVerify/DCSweep.pm",
        9_301,
        "2246da2374e6cce3ea516a50e472fb07f7481e8b0effb20d4a650e6b6cb1eda0",
        "b9cc7d905d001ebe2ace44936b9631e4bdcbf42bca4d4b34c5866262cd11d9a3",
    ),
    (
        "TestScripts/XyceVerify/StepSweep.pm",
        8_731,
        "84b2d485c1848f2e456463de8a5015205d87c3db8a6d070547d6f9464618fed6",
        "db1b142ab3ae9163bbe02bd68b5b3a6311436adbf27c06d71a5c05df9b6973e7",
    ),
    (
        "TestScripts/xyce_verify.pl",
        59_566,
        "6e5f84b1646b30d0e12879848d7653584b39472d640a14916ae8fda6e1df12b3",
        "5eadb6dab06ed3091ea114146bd4a574de83784f87be9843ad7b721b0a793665",
    ),
];

const CAPACITOR_RETAINED: [(&str, usize, &str, &str); 3] = [
    (
        "capacitor.cir",
        1_280,
        "d8ce29f005843f2f3efd7ff2b798e674cbd54dc3ada232c8700e09bd41ee7ef5",
        "f36ed76968921c6c6d5c5a8a5ce351e55d05d5fb13167ce84b6b4bccdb3a0308",
    ),
    (
        "capacitor1.cir",
        1_286,
        "4e26278fc336e26b265aad6cde14d4d10d9f36134ce0296d9343ce3e8d4eb35d",
        "50925caf2d37b70d0bc6a013946388785d28f1231b8e5aea53d29f1d96b7df9f",
    ),
    (
        "rc_osc.cir",
        205,
        "add8ca9d2654398a4c394bc9232809677b479fe35172c66bbb8e2ad9bf9410a4",
        "c9003d09478539483d993a4ad820043d58595a0c29b147d939bf46e593614666",
    ),
];

const IC_RETAINED: [(&str, usize, &str, &str); 6] = [
    (
        "ic_cap2.cir",
        966,
        "3dada86ba38092d963cab254b5343aaceb68b5154500028df4af0dcd9b56b5b3",
        "9f4b40b859902b9c2de7b47546158d56311937918f2c7b40734f903380e7e240",
    ),
    (
        "pierce.cir",
        1_876,
        "cdebfe4588570b9cb1fb4791aec36b27ad43dd61805e94fdd06e0b9c816d610f",
        "22d01c9aafc6a5e794dd92a9acea9a8aa3e0b8834a6032787606148309397237",
    ),
    (
        "pierce1.cir",
        1_875,
        "06b2f356095d24dd1dd67cd6d6185fdffc3f2ab0ef409494919f9b7e4f6081ae",
        "0863c5eb07f665e92181a01756e6581f3cfec380c98e22d04e7c021b9030c293",
    ),
    (
        "pierce2.cir",
        1_876,
        "cdebfe4588570b9cb1fb4791aec36b27ad43dd61805e94fdd06e0b9c816d610f",
        "22d01c9aafc6a5e794dd92a9acea9a8aa3e0b8834a6032787606148309397237",
    ),
    (
        "pierce3.cir",
        1_928,
        "01e9feb35012345dc9fb0dac3f1b7c84d4a35cfec4c2df52fd28b316b370ddbf",
        "7f37b768bae32d43dc10b4326b24e009bf1442d968be065defa8392a5bf18a5b",
    ),
    (
        "pierce_noop.cir",
        1_658,
        "41ae42306163d5ccf27ae30a397ebbf7a7abb4dc6dd116371496f6f5f9c35362",
        "e376b255e216b5c802a6807734782a4334dfee2a7256e3042924b7c5c5c90355",
    ),
];

const IC_OUTPUT_RETAINED: [(&str, usize, &str, &str); 1] = [(
    "pierce.cir.prn",
    334_282,
    "35458378a4dc2062e64663904e9d7e9041e90cf289474f0cd63e797521e0fdde",
    "3dd9d7d0a7c527e22f66b2c89f464e35b0ee1a11f9c7aa670eb78d42cfcbda62",
)];

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub(super) enum XyceTiaPassiveAnalyticKind {
    CapacitorInstanceIc,
    PositionalIc,
}

impl XyceTiaPassiveAnalyticKind {
    const ALL: [Self; 2] = [Self::CapacitorInstanceIc, Self::PositionalIc];

    pub(super) fn for_record(record: &str) -> Option<Self> {
        let record = XyceTestRunner::normalize_manifest_key(record);
        Self::ALL.into_iter().find(|kind| kind.record() == record)
    }

    pub(super) fn contract(self) -> &'static str {
        match self {
            Self::CapacitorInstanceIc => "tia_trap_capacitor_instance_ic_analytic_wrapper",
            Self::PositionalIc => "tia_trap_positional_ic_analytic_wrapper",
        }
    }

    fn path(self) -> &'static str {
        match self {
            Self::CapacitorInstanceIc => "Netlists/TIA/TRAP/CAPACITOR/capacitor.cir",
            Self::PositionalIc => "Netlists/TIA/TRAP/IC_AND_NODESET/ic_cap2.cir",
        }
    }

    fn record(self) -> &'static str {
        match self {
            Self::CapacitorInstanceIc => "netlists/tia/trap/capacitor/capacitor.cir",
            Self::PositionalIc => "netlists/tia/trap/ic_and_nodeset/ic_cap2.cir",
        }
    }

    fn file_name(self) -> &'static str {
        match self {
            Self::CapacitorInstanceIc => "capacitor.cir",
            Self::PositionalIc => "ic_cap2.cir",
        }
    }
}

impl XyceTestRunner {
    pub(super) fn tia_passive_analytic_historical_records() -> Vec<String> {
        let mut records = HISTORICAL_ARTIFACTS
            .into_iter()
            .map(|(path, bytes, sha256, content_blake3)| {
                format!(
                    "{UPSTREAM_COMMIT}\t{UPSTREAM_TAG}\t{path}\t{bytes}\t{sha256}\t{content_blake3}"
                )
            })
            .collect::<Vec<_>>();
        records.sort();
        records
    }

    pub(super) fn validate_tia_passive_analytic_historical_provenance() -> Result<(), String> {
        let records = Self::tia_passive_analytic_historical_records();
        let stream = records.join("\n");
        let sha256 = format!("{:x}", Sha256::digest(stream.as_bytes()));
        let content_blake3 = blake3::hash(stream.as_bytes()).to_hex().to_string();
        if records.len() != HISTORICAL_RECORD_COUNT
            || stream.len() != HISTORICAL_RECORD_BYTES
            || sha256 != HISTORICAL_RECORDS_SHA256
            || content_blake3 != HISTORICAL_RECORDS_BLAKE3
        {
            return Err(format!(
                "{LABEL} Release-7.10 provenance changed: records={}/{}, sha256={sha256}, blake3={content_blake3}",
                records.len(),
                stream.len()
            ));
        }
        Ok(())
    }

    fn validate_tia_artifact_directory(
        directory: &Path,
        expected: &[(&str, usize, &str, &str)],
        label: &str,
    ) -> Result<BTreeMap<String, Vec<u8>>, String> {
        let metadata = fs::symlink_metadata(directory)
            .map_err(|error| format!("failed to inspect {label}: {error}"))?;
        if metadata.file_type().is_symlink() || !metadata.file_type().is_dir() {
            return Err(format!("{label} must be a regular non-symlink directory"));
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
                    "{label} member {} is not a regular file",
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
                return Err(format!("{label} member case changed: {name:?}"));
            }
            let bytes = fs::read(&path)
                .map_err(|error| format!("failed to read {}: {error}", path.display()))?;
            let canonical = Self::canonical_lf_text_identity(label, &bytes)?;
            let sha256 = format!("{:x}", Sha256::digest(&canonical));
            let content_blake3 = blake3::hash(&canonical).to_hex().to_string();
            if canonical.len() != expected_bytes
                || sha256 != expected_sha256
                || content_blake3 != expected_blake3
            {
                return Err(format!("{label} member {name:?} changed"));
            }
            observed.insert(key, bytes);
        }
        if observed.len() != expected.len() {
            return Err(format!("{label} member census changed"));
        }
        Ok(observed)
    }

    fn validate_tia_passive_analytic_provenance(
        &self,
        deck: &XyceDeck,
        kind: XyceTiaPassiveAnalyticKind,
    ) -> Result<Vec<u8>, String> {
        Self::validate_tia_passive_analytic_historical_provenance()?;
        if deck.section != XyceDeckSection::Netlists
            || Self::normalize_manifest_key(&deck.relative_path) != kind.record()
            || Self::normalize_manifest_key(&self.relative_key(&deck.path)) != kind.record()
            || !Self::same_path(&deck.path, &self.root.join(kind.path()))
        {
            return Err(format!("recognized {LABEL} member is not canonical"));
        }

        let capacitor_prefix = "netlists/tia/trap/capacitor/";
        let capacitor_owners = self
            .upstream_wrapper_decks
            .iter()
            .filter(|record| record.starts_with(capacitor_prefix))
            .map(String::as_str)
            .collect::<BTreeSet<_>>();
        if capacitor_owners
            != BTreeSet::from([
                "netlists/tia/trap/capacitor/capacitor.cir",
                "netlists/tia/trap/capacitor/rc_osc.cir",
            ])
        {
            return Err(format!("{LABEL} CAPACITOR wrapper ownership changed"));
        }
        let ic_prefix = "netlists/tia/trap/ic_and_nodeset/";
        let ic_owners = self
            .upstream_wrapper_decks
            .iter()
            .filter(|record| record.starts_with(ic_prefix))
            .map(String::as_str)
            .collect::<BTreeSet<_>>();
        if ic_owners
            != BTreeSet::from([
                "netlists/tia/trap/ic_and_nodeset/ic_cap2.cir",
                "netlists/tia/trap/ic_and_nodeset/pierce3.cir",
                "netlists/tia/trap/ic_and_nodeset/pierce_noop.cir",
            ])
        {
            return Err(format!("{LABEL} IC_AND_NODESET wrapper ownership changed"));
        }
        let exclusions = Self::load_upstream_exclusions(&self.root)
            .map_err(|error| format!("{LABEL} exclusion manifest is invalid: {error}"))?;
        let capacitor_exclusions = exclusions
            .iter()
            .filter(|(record, _)| record.starts_with(capacitor_prefix))
            .map(|(record, exclusion)| {
                (
                    record.as_str(),
                    exclusion.source.as_str(),
                    matches!(
                        &exclusion.disposition,
                        XyceUpstreamExclusionDisposition::Excluded
                    ),
                )
            })
            .collect::<BTreeSet<_>>();
        if capacitor_exclusions
            != BTreeSet::from([(
                "netlists/tia/trap/capacitor/capacitor1.cir",
                "Netlists/TIA/TRAP/CAPACITOR/exclude",
                true,
            )])
        {
            return Err(format!("{LABEL} CAPACITOR exclusions changed"));
        }
        let ic_exclusions = exclusions
            .iter()
            .filter(|(record, _)| record.starts_with(ic_prefix))
            .map(|(record, exclusion)| {
                (
                    record.as_str(),
                    exclusion.source.as_str(),
                    matches!(
                        &exclusion.disposition,
                        XyceUpstreamExclusionDisposition::Excluded
                    ),
                )
            })
            .collect::<BTreeSet<_>>();
        if ic_exclusions
            != BTreeSet::from([
                (
                    "netlists/tia/trap/ic_and_nodeset/pierce1.cir",
                    "Netlists/TIA/TRAP/IC_AND_NODESET/exclude",
                    true,
                ),
                (
                    "netlists/tia/trap/ic_and_nodeset/pierce2.cir",
                    "Netlists/TIA/TRAP/IC_AND_NODESET/exclude",
                    true,
                ),
            ])
        {
            return Err(format!("{LABEL} IC_AND_NODESET exclusions changed"));
        }
        if exclusions.contains_key(kind.record()) {
            return Err(format!("{LABEL} candidate must not be excluded"));
        }

        let capacitor_sources = Self::validate_tia_artifact_directory(
            &self.root.join("Netlists/TIA/TRAP/CAPACITOR"),
            &CAPACITOR_RETAINED,
            "TIA/TRAP/CAPACITOR retained source family",
        )?;
        let ic_sources = Self::validate_tia_artifact_directory(
            &self.root.join("Netlists/TIA/TRAP/IC_AND_NODESET"),
            &IC_RETAINED,
            "TIA/TRAP/IC_AND_NODESET retained source family",
        )?;
        let capacitor_output = self.root.join("OutputData/TIA/TRAP/CAPACITOR");
        match fs::symlink_metadata(&capacitor_output) {
            Err(error) if error.kind() == std::io::ErrorKind::NotFound => {}
            Err(error) => return Err(format!("failed to inspect CAPACITOR output: {error}")),
            Ok(_) => return Err(format!("{LABEL} CAPACITOR must not acquire numerical gold")),
        }
        Self::validate_tia_artifact_directory(
            &self.root.join("OutputData/TIA/TRAP/IC_AND_NODESET"),
            &IC_OUTPUT_RETAINED,
            "TIA/TRAP/IC_AND_NODESET retained output family",
        )?;
        for candidate in XyceTiaPassiveAnalyticKind::ALL {
            self.reject_wrapper_output_artifacts(&self.root.join(candidate.path()))
                .map_err(|error| format!("{LABEL} {} {error}", candidate.file_name()))?;
        }
        match kind {
            XyceTiaPassiveAnalyticKind::CapacitorInstanceIc => capacitor_sources,
            XyceTiaPassiveAnalyticKind::PositionalIc => ic_sources,
        }
        .remove(&kind.file_name().to_ascii_lowercase())
        .ok_or_else(|| format!("{LABEL} lost {}", kind.file_name()))
    }

    fn tia_nodes_match(actual: &[String], expected: &[&str]) -> bool {
        actual.len() == expected.len()
            && actual
                .iter()
                .zip(expected)
                .all(|(actual, expected)| actual.eq_ignore_ascii_case(expected))
    }

    fn validate_tia_passive_analytic_typed_contract(
        &self,
        kind: XyceTiaPassiveAnalyticKind,
        source: &str,
        path: &Path,
    ) -> Result<(XyceStaticTranPlan, XyceAnalyticRcSpecification), String> {
        let plan = self.static_tran_plan_for_path_with_purpose(
            path,
            XyceStaticTranPlanPurpose::AnalyticOracle,
        )?;
        if plan.deck_path != path
            || plan.source.as_bytes() != source.as_bytes()
            || !matches!(plan.oracle, XyceStaticTranOracle::None)
            || plan.contract != XyceStaticTranContract::WrapperStatic
            || !plan.steps.is_empty()
            || plan.output_override
            || plan.timeint_conststep
            || plan.wrapper_tolerance.is_some()
            || plan.tran.step.to_bits() != 0.0f64.to_bits()
            || plan.tran.stop.to_bits() != 5.0e-3f64.to_bits()
            || plan.tran.start.is_some()
            || plan.tran.max_step.is_some()
            || plan.tran.uic
            || plan.print.as_ref().is_none_or(|print| {
                print.probes.len() != 1 || !print.probes[0].eq_ignore_ascii_case("V(1)")
            })
        {
            return Err(format!(
                "{LABEL} {} plan changed: {plan:?}",
                kind.file_name()
            ));
        }
        let netlist = Self::parse_xyce_netlist(source, path)
            .map_err(|error| format!("{LABEL} {} no longer parses: {error}", kind.file_name()))?;
        if netlist.elements.len() != 3
            || netlist.analyses.len() != 1
            || netlist.output_requests.len() != 1
            || !netlist.models.is_empty()
            || !netlist.subcircuits.is_empty()
            || !netlist.diagnostics.is_empty()
            || !netlist.data_tables.is_empty()
            || !netlist.measurements.is_empty()
            || netlist.device_initial_conditions.is_some()
            || !netlist.node_sets.is_empty()
            || !netlist.global_nodes.is_empty()
            || !netlist.veriloga_includes.is_empty()
            || !netlist.spef_includes.is_empty()
            || !netlist.params.all_params().is_empty()
            || !netlist.params.all_string_params().is_empty()
            || !netlist.params.all_functions().is_empty()
        {
            return Err(format!(
                "{LABEL} {} typed envelope changed",
                kind.file_name()
            ));
        }
        let expected_lte = match kind {
            XyceTiaPassiveAnalyticKind::CapacitorInstanceIc => {
                Some(TransientLteReference::PointGlobal)
            }
            XyceTiaPassiveAnalyticKind::PositionalIc => None,
        };
        if !Self::analytic_timeint_only_options_match(
            &netlist.options,
            None,
            None,
            Some("7"),
            expected_lte,
        ) {
            return Err(format!("{LABEL} {} option state changed", kind.file_name()));
        }
        let [capacitor, resistor, voltage] = netlist.elements.as_slice() else {
            unreachable!("TIA passive element count was checked");
        };
        if !capacitor.name.eq_ignore_ascii_case("C1")
            || capacitor.provenance != ElementProvenance::Authored
            || !Self::tia_nodes_match(&capacitor.nodes, &["1", "0"])
            || !matches!(&capacitor.kind, ElementKind::Capacitor {
                value,
                value_expr: None,
                initial_voltage,
                model: None,
                instance_params,
                deferred_params,
            } if value.to_bits() == 1.0e-6f64.to_bits()
                && initial_voltage.map(Value::to_bits) == match kind {
                    XyceTiaPassiveAnalyticKind::CapacitorInstanceIc => Some(1.0f64.to_bits()),
                    XyceTiaPassiveAnalyticKind::PositionalIc => None,
                }
                && instance_params.is_empty()
                && deferred_params.is_empty())
            || !resistor.name.eq_ignore_ascii_case("R1")
            || resistor.provenance != ElementProvenance::Authored
            || !Self::tia_nodes_match(&resistor.nodes, &["1", "2"])
            || !matches!(&resistor.kind, ElementKind::Resistor {
                value,
                value_expr: None,
                model: None,
                instance_params,
                deferred_params,
            } if value.to_bits() == 1.0e3f64.to_bits()
                && instance_params.is_empty()
                && deferred_params.is_empty())
            || !voltage.name.eq_ignore_ascii_case("V1")
            || voltage.provenance != ElementProvenance::Authored
            || !Self::tia_nodes_match(&voltage.nodes, &["2", "0"])
            || !matches!(&voltage.kind, ElementKind::VoltageSource(
                rspice_core::netlist::SourceSpec::Dc(value)
            ) if value.to_bits() == 0.0f64.to_bits())
        {
            return Err(format!(
                "{LABEL} {} passive topology changed",
                kind.file_name()
            ));
        }
        match kind {
            XyceTiaPassiveAnalyticKind::CapacitorInstanceIc
                if !netlist.initial_conditions.is_empty() =>
            {
                return Err(format!(
                    "{LABEL} capacitor-local IC acquired a .IC directive"
                ));
            }
            XyceTiaPassiveAnalyticKind::PositionalIc
                if !matches!(netlist.initial_conditions.as_slice(), [condition]
                    if condition.node.eq_ignore_ascii_case("1")
                        && condition.voltage.to_bits() == 1.0f64.to_bits()
                        && condition.voltage_expr.is_none()) =>
            {
                return Err(format!("{LABEL} positional .IC changed"));
            }
            _ => {}
        }
        if !matches!(&netlist.analyses[0], AnalysisCommand::Tran {
            step,
            stop,
            start: None,
            max_step: None,
            uic: false,
        } if step.to_bits() == 0.0f64.to_bits()
            && stop.to_bits() == 5.0e-3f64.to_bits())
        {
            return Err(format!("{LABEL} {} typed TRAN changed", kind.file_name()));
        }
        let request = &netlist.output_requests[0];
        if request.directive != OutputDirectiveKind::Print
            || request.analysis != Some(rspice_core::netlist::OutputAnalysisKind::Tran)
            || request.name.is_some()
            || request.print_delimiter.as_ref() != Some(&PrintDelimiter::Whitespace)
            || !request.expressions.is_empty()
            || request.dependencies.len() != 1
            || request.dependencies[0].kind != OutputSymbolKind::Node
            || request.dependencies[0].expression
            || !request.dependencies[0].operator.eq_ignore_ascii_case("V")
            || !request.dependencies[0].symbol.eq_ignore_ascii_case("1")
        {
            return Err(format!(
                "{LABEL} {} PRINT request changed",
                kind.file_name()
            ));
        }
        let effective_capacitance = Self::effective_capacitor_value(&netlist, "C1")
            .filter(|value| value.to_bits() == 1.0e-6f64.to_bits())
            .ok_or_else(|| format!("{LABEL} capacitance did not resolve exactly"))?;
        let effective_resistance = Self::effective_resistor_value(&netlist, "R1")?
            .filter(|value| value.to_bits() == 1.0e3f64.to_bits())
            .ok_or_else(|| format!("{LABEL} resistance did not resolve exactly"))?;
        let time_constant = effective_resistance * effective_capacitance;
        if time_constant.to_bits() != 1.0e-3f64.to_bits() {
            return Err(format!("{LABEL} time constant changed to {time_constant}"));
        }
        Ok((
            plan,
            XyceAnalyticRcSpecification {
                output_node: "1".to_string(),
                source_value: 0.0,
                initial_voltage: 1.0,
                resistance: effective_resistance,
                capacitance: effective_capacitance,
                time_constant,
            },
        ))
    }

    pub(super) fn validate_tia_passive_analytic_oracle(
        &self,
        deck: &XyceDeck,
        kind: XyceTiaPassiveAnalyticKind,
        start: Instant,
    ) -> Result<(), String> {
        let abort = DeadlineAbort::new(start, self.config.max_time_per_test_ms.max(1));
        if abort.is_aborted() {
            return Err(format!("{LABEL} deadline expired before provenance"));
        }
        let bytes = self.validate_tia_passive_analytic_provenance(deck, kind)?;
        let source = std::str::from_utf8(&bytes)
            .map_err(|error| format!("{LABEL} {} is not UTF-8: {error}", kind.file_name()))?;
        let (plan, specification) =
            self.validate_tia_passive_analytic_typed_contract(kind, source, &deck.path)?;
        if abort.is_aborted() {
            return Err(format!("{LABEL} validation exceeded deadline"));
        }
        let (netlist, result) = self
            .run_transient_family_plan(&plan, start, None, None)
            .map_err(|error| match error {
                SimulationError::Aborted => format!("{LABEL} execution exceeded deadline"),
                other => format!("{LABEL} execution failed: {other}"),
            })?;
        let actual = Self::transient_family_result_to_prn_table(&plan, &netlist, &result)
            .map_err(|error| format!("{LABEL} output observation failed: {error}"))?;
        Self::validate_analytic_rc_initial_sample(&actual, &specification)?;
        Self::validate_analytic_rc_complete_time_domain(&actual, plan.tran.stop)?;
        let reference = Self::analytic_rc_reference_table(&actual, &specification)?;
        let mismatches = self.compare_xyce_verify_transient_tables(&reference, &actual)?;
        if !mismatches.is_empty() {
            return Err(format!(
                "{LABEL} {} produced {} xyce_verify mismatch(es): {mismatches:?}",
                kind.file_name(),
                mismatches.len()
            ));
        }
        self.validate_tia_passive_analytic_provenance(deck, kind)?;
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

    fn canonical_deck(root: &Path, kind: XyceTiaPassiveAnalyticKind) -> XyceDeck {
        XyceDeck {
            path: root.join(kind.path()),
            section: XyceDeckSection::Netlists,
            relative_path: kind.path().to_string(),
        }
    }

    fn typed_fixture(
        label: &str,
        kind: XyceTiaPassiveAnalyticKind,
        source: &str,
    ) -> (tempfile::TempDir, XyceDeck, XyceTestRunner) {
        let temporary = tempfile::Builder::new()
            .prefix(&format!("rspice-tia-passive-typed-{label}-"))
            .tempdir()
            .expect("create typed fixture");
        let root = temporary.path();
        let deck = canonical_deck(root, kind);
        fs::create_dir_all(deck.path.parent().expect("candidate parent"))
            .expect("create candidate parent");
        fs::write(&deck.path, source).expect("write typed fixture source");
        fs::write(
            root.join(HARNESS_MANIFEST_FILE),
            format!("{}\t{REQUIRES_UPSTREAM_WRAPPER_CONTRACT}\n", kind.path()),
        )
        .expect("write typed fixture wrapper manifest");
        let runner = XyceTestRunner::new(root, XyceRunnerConfig::default());
        (temporary, deck, runner)
    }

    fn copy_regular_family(source: &Path, destination: &Path) {
        fs::create_dir_all(destination).expect("create fixture family");
        for entry in fs::read_dir(source).expect("read source family") {
            let entry = entry.expect("read source member");
            if entry.file_type().expect("inspect source member").is_file() {
                fs::copy(entry.path(), destination.join(entry.file_name()))
                    .expect("copy source member");
            }
        }
    }

    fn provenance_fixture(
        label: &str,
        kind: XyceTiaPassiveAnalyticKind,
    ) -> (tempfile::TempDir, XyceDeck, XyceTestRunner) {
        let source_root = corpus_root();
        let temporary = tempfile::Builder::new()
            .prefix(&format!("rspice-tia-passive-provenance-{label}-"))
            .tempdir()
            .expect("create provenance fixture");
        let root = temporary.path();
        copy_regular_family(
            &source_root.join("Netlists/TIA/TRAP/CAPACITOR"),
            &root.join("Netlists/TIA/TRAP/CAPACITOR"),
        );
        copy_regular_family(
            &source_root.join("Netlists/TIA/TRAP/IC_AND_NODESET"),
            &root.join("Netlists/TIA/TRAP/IC_AND_NODESET"),
        );
        copy_regular_family(
            &source_root.join("OutputData/TIA/TRAP/IC_AND_NODESET"),
            &root.join("OutputData/TIA/TRAP/IC_AND_NODESET"),
        );
        fs::write(
            root.join(HARNESS_MANIFEST_FILE),
            [
                "Netlists/TIA/TRAP/CAPACITOR/capacitor.cir\trequires_upstream_wrapper",
                "Netlists/TIA/TRAP/CAPACITOR/rc_osc.cir\trequires_upstream_wrapper",
                "Netlists/TIA/TRAP/IC_AND_NODESET/ic_cap2.cir\trequires_upstream_wrapper",
                "Netlists/TIA/TRAP/IC_AND_NODESET/pierce3.cir\trequires_upstream_wrapper",
                "Netlists/TIA/TRAP/IC_AND_NODESET/pierce_noop.cir\trequires_upstream_wrapper",
            ]
            .join("\n")
                + "\n",
        )
        .expect("write provenance wrapper manifest");
        fs::write(
            root.join(UPSTREAM_EXCLUSIONS_MANIFEST_FILE),
            format!(
                "schema_version\t{UPSTREAM_EXCLUSIONS_SCHEMA_VERSION}\nsource_commit\t{UPSTREAM_EXCLUSIONS_SOURCE_COMMIT}\nsource_netlists_tree\t{UPSTREAM_EXCLUSIONS_SOURCE_NETLISTS_TREE}\nNetlists/TIA/TRAP/CAPACITOR/capacitor1.cir\tNetlists/TIA/TRAP/CAPACITOR/exclude\tupstream_excluded\nNetlists/TIA/TRAP/IC_AND_NODESET/pierce1.cir\tNetlists/TIA/TRAP/IC_AND_NODESET/exclude\tupstream_excluded\nNetlists/TIA/TRAP/IC_AND_NODESET/pierce2.cir\tNetlists/TIA/TRAP/IC_AND_NODESET/exclude\tupstream_excluded\n"
            ),
        )
        .expect("write provenance exclusion manifest");
        let deck = canonical_deck(root, kind);
        let runner = XyceTestRunner::new(root, XyceRunnerConfig::default());
        (temporary, deck, runner)
    }

    #[test]
    fn tia_passive_historical_provenance_is_exact() {
        XyceTestRunner::validate_tia_passive_analytic_historical_provenance()
            .expect("Release-7.10 TIA passive provenance remains exact");
    }

    #[test]
    fn tia_passive_candidates_have_exact_typed_contracts() {
        let root = corpus_root();
        let runner = XyceTestRunner::new(&root, XyceRunnerConfig::default());
        for kind in XyceTiaPassiveAnalyticKind::ALL {
            let path = root.join(kind.path());
            let source = fs::read_to_string(&path).expect("read TIA passive source");
            runner
                .validate_tia_passive_analytic_typed_contract(kind, &source, &path)
                .expect("canonical TIA passive typed contract passes");
        }
    }

    #[test]
    fn tia_passive_typed_mutations_fail_closed() {
        let root = corpus_root();
        for kind in XyceTiaPassiveAnalyticKind::ALL {
            let canonical = fs::read_to_string(root.join(kind.path()))
                .expect("read canonical TIA passive source");
            let mutations = match kind {
                XyceTiaPassiveAnalyticKind::CapacitorInstanceIc => vec![
                    canonical.replacen("IC=1", "IC=2", 1),
                    canonical.replacen("method=7", "method=8", 1),
                    canonical.replacen("newlte=1", "newlte=2", 1),
                    canonical.replacen("R1 1 2 1K", "R1 1 2 2K", 1),
                    canonical.replacen(".print tran v(1)", ".print tran v(2)", 1),
                    canonical.replacen(".tran 0 5ms", ".tran 0 6ms", 1),
                ],
                XyceTiaPassiveAnalyticKind::PositionalIc => vec![
                    canonical.replacen(".ic 1 1.0", ".ic 2 1.0", 1),
                    canonical.replacen(".ic 1 1.0", ".ic 1 2.0", 1),
                    canonical.replacen("method=7", "method=8", 1),
                    canonical.replacen("R1 1 2 1K", "R1 1 2 2K", 1),
                    canonical.replacen(".print tran v(1)", ".print tran v(2)", 1),
                    canonical.replacen(".tran 0 5ms", ".tran 0 6ms", 1),
                ],
            };
            for (index, mutation) in mutations.into_iter().enumerate() {
                let (_temporary, deck, runner) =
                    typed_fixture(&format!("{kind:?}-{index}"), kind, &mutation);
                assert!(
                    runner
                        .validate_tia_passive_analytic_typed_contract(kind, &mutation, &deck.path,)
                        .is_err(),
                    "{kind:?} mutation {index} must fail closed"
                );
            }
        }
    }

    #[test]
    fn tia_passive_provenance_rejects_source_census_and_role_drift() {
        let (_temporary, deck, runner) =
            provenance_fixture("canonical", XyceTiaPassiveAnalyticKind::CapacitorInstanceIc);
        runner
            .validate_tia_passive_analytic_provenance(
                &deck,
                XyceTiaPassiveAnalyticKind::CapacitorInstanceIc,
            )
            .expect("canonical provenance fixture passes");

        let (_temporary, deck, runner) = provenance_fixture(
            "source-drift",
            XyceTiaPassiveAnalyticKind::CapacitorInstanceIc,
        );
        fs::write(&deck.path, "changed\n").expect("mutate candidate source");
        assert!(
            runner
                .validate_tia_passive_analytic_provenance(
                    &deck,
                    XyceTiaPassiveAnalyticKind::CapacitorInstanceIc,
                )
                .is_err()
        );

        let (_temporary, deck, runner) =
            provenance_fixture("extra-member", XyceTiaPassiveAnalyticKind::PositionalIc);
        fs::write(
            deck.path
                .parent()
                .expect("IC family parent")
                .join("extra.cir"),
            "extra\n",
        )
        .expect("add unexpected source member");
        assert!(
            runner
                .validate_tia_passive_analytic_provenance(
                    &deck,
                    XyceTiaPassiveAnalyticKind::PositionalIc,
                )
                .is_err()
        );

        let (temporary, deck, _runner) =
            provenance_fixture("role-drift", XyceTiaPassiveAnalyticKind::PositionalIc);
        let root = temporary.path();
        fs::write(
            root.join(HARNESS_MANIFEST_FILE),
            "Netlists/TIA/TRAP/CAPACITOR/capacitor.cir\trequires_upstream_wrapper\n",
        )
        .expect("mutate wrapper roles");
        let runner = XyceTestRunner::new(root, XyceRunnerConfig::default());
        assert!(
            runner
                .validate_tia_passive_analytic_provenance(
                    &deck,
                    XyceTiaPassiveAnalyticKind::PositionalIc,
                )
                .is_err()
        );

        let (temporary, deck, _runner) = provenance_fixture(
            "exclusion-source-drift",
            XyceTiaPassiveAnalyticKind::CapacitorInstanceIc,
        );
        let root = temporary.path();
        fs::write(
            root.join(UPSTREAM_EXCLUSIONS_MANIFEST_FILE),
            format!(
                "schema_version\t{UPSTREAM_EXCLUSIONS_SCHEMA_VERSION}\nsource_commit\t{UPSTREAM_EXCLUSIONS_SOURCE_COMMIT}\nsource_netlists_tree\t{UPSTREAM_EXCLUSIONS_SOURCE_NETLISTS_TREE}\nNetlists/TIA/TRAP/CAPACITOR/capacitor1.cir\tNetlists/TIA/TRAP/CAPACITOR/exclude\tupstream_excluded\nNetlists/TIA/TRAP/IC_AND_NODESET/pierce1.cir\tnetlists/tia/trap/ic_and_nodeset/exclude\tupstream_excluded\nNetlists/TIA/TRAP/IC_AND_NODESET/pierce2.cir\tNetlists/TIA/TRAP/IC_AND_NODESET/exclude\tupstream_excluded\n"
            ),
        )
        .expect("mutate exclusion source case");
        let runner = XyceTestRunner::new(root, XyceRunnerConfig::default());
        assert!(
            runner
                .validate_tia_passive_analytic_provenance(
                    &deck,
                    XyceTiaPassiveAnalyticKind::CapacitorInstanceIc,
                )
                .is_err()
        );

        let (temporary, deck, _runner) = provenance_fixture(
            "exclusion-disposition-drift",
            XyceTiaPassiveAnalyticKind::PositionalIc,
        );
        let root = temporary.path();
        fs::write(
            root.join(UPSTREAM_EXCLUSIONS_MANIFEST_FILE),
            format!(
                "schema_version\t{UPSTREAM_EXCLUSIONS_SCHEMA_VERSION}\nsource_commit\t{UPSTREAM_EXCLUSIONS_SOURCE_COMMIT}\nsource_netlists_tree\t{UPSTREAM_EXCLUSIONS_SOURCE_NETLISTS_TREE}\nNetlists/TIA/TRAP/CAPACITOR/capacitor1.cir\tNetlists/TIA/TRAP/CAPACITOR/exclude\tupstream_excluded\nNetlists/TIA/TRAP/IC_AND_NODESET/pierce1.cir\tNetlists/TIA/TRAP/IC_AND_NODESET/exclude\trspice_independently_qualified\tstatic_prn_tran\nNetlists/TIA/TRAP/IC_AND_NODESET/pierce2.cir\tNetlists/TIA/TRAP/IC_AND_NODESET/exclude\tupstream_excluded\n"
            ),
        )
        .expect("mutate exclusion disposition");
        let runner = XyceTestRunner::new(root, XyceRunnerConfig::default());
        assert!(
            runner
                .validate_tia_passive_analytic_provenance(
                    &deck,
                    XyceTiaPassiveAnalyticKind::PositionalIc,
                )
                .is_err()
        );
    }

    #[test]
    fn tia_passive_oracles_execute_both_initial_condition_forms() {
        let root = corpus_root();
        let runner = XyceTestRunner::new(&root, XyceRunnerConfig::default());
        for kind in XyceTiaPassiveAnalyticKind::ALL {
            let deck = canonical_deck(&root, kind);
            runner
                .validate_tia_passive_analytic_oracle(&deck, kind, Instant::now())
                .expect("canonical TIA passive oracle passes");
        }
    }

    #[test]
    fn tia_passive_oracle_rejects_expired_deadline() {
        let root = corpus_root();
        let mut config = XyceRunnerConfig::default();
        config.max_time_per_test_ms = 1;
        let runner = XyceTestRunner::new(&root, config);
        let deck = canonical_deck(&root, XyceTiaPassiveAnalyticKind::PositionalIc);
        assert!(
            runner
                .validate_tia_passive_analytic_oracle(
                    &deck,
                    XyceTiaPassiveAnalyticKind::PositionalIc,
                    Instant::now() - Duration::from_millis(10),
                )
                .is_err()
        );
    }
}
