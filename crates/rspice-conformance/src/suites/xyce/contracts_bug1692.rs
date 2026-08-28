use super::*;

const LABEL: &str = "BUG_1692 expression-equivalence relational family";
const FAMILY_PREFIX: &str = "netlists/certification_tests/bug_1692/";
const FAMILY_DIRECTORY: &str = "Netlists/Certification_Tests/BUG_1692";
const EXCLUSION_SOURCE: &str = "Netlists/Certification_Tests/BUG_1692/exclude";

pub(super) const BUG1692_OWNER_CONTRACT: &str = "bug1692_expression_equivalence_wrapper_owner";
pub(super) const BUG1692_WORKER_CONTRACT: &str = "bug1692_expression_equivalence_worker";

const UPSTREAM_REGRESSION_COMMIT: &str = "d6e278e371ec2f3df1325dcff4552e585bc7ecc1";
const UPSTREAM_RELEASE_TAG: &str = "Release-7.10.0";
const HISTORICAL_RECORD_COUNT: usize = 5;
const HISTORICAL_RECORD_BYTES: usize = 1_171;
const HISTORICAL_RECORDS_SHA256: &str =
    "8a2fa516b8b2f15c2321d6837ce615f1fc5a24d3f38ff416f95745fa6f9c88c7";
const HISTORICAL_RECORDS_BLAKE3: &str =
    "87afaa06305da2d9a1823dc09d9d41a97e51e5c78d635592aef373d7b9244b49";
const HISTORICAL_ARTIFACTS: [(&str, usize, &str, &str); HISTORICAL_RECORD_COUNT] = [
    (
        "Netlists/Certification_Tests/BUG_1692/Manifest.txt",
        131,
        "2e1c7960c6934218941867d14e3d4ef2a68fe524883df4c789ab4634dc80b897",
        "c7b79490786f77827785b7020c72cd99ec1165030c85622f638c740d56fd6ef1",
    ),
    (
        "Netlists/Certification_Tests/BUG_1692/bug_1692.cir.sh",
        2_076,
        "3d98820d4c3177afd12e14893015013686e4ee6b26ce029785ecacafc7e28705",
        "46f35e0b57a119a88da5c686db4075eb97d9cd80d66a08c94bb161e359076489",
    ),
    (
        EXCLUSION_SOURCE,
        82,
        "9e2ca895cea37ed530b5620fe5d1861c45a8c27f32ee9df156b00cd3657d058a",
        "1823eab345a4bc0a1735871771b3319ac6c4fb0720c3cfd6a3a3e0a84941d6ad",
    ),
    (
        "Netlists/Certification_Tests/BUG_1692/tags",
        23,
        "680b3f8ee2109c4d7b793a14af9c7296ee93bbb5832d4a984e0d4dcc0523a083",
        "cdd7361907b14f9fdaf2bdd56d84b4eae85944d19d08ff7291cf8c26dcaa2623",
    ),
    (
        "TestScripts/xyce_verify.pl",
        59_566,
        "6e5f84b1646b30d0e12879848d7653584b39472d640a14916ae8fda6e1df12b3",
        "5eadb6dab06ed3091ea114146bd4a574de83784f87be9843ad7b721b0a793665",
    ),
];

const RETAINED_RECORD_COUNT: usize = 7;
const RETAINED_RECORD_BYTES: usize = 1_308;
const RETAINED_RECORDS_SHA256: &str =
    "09eb3353d6c05161d5217e966a4b408df2da203bd8289601f3810d90dc22704b";
const RETAINED_RECORDS_BLAKE3: &str =
    "e5a225f3542230e5d50b8c657b4581973ecbf939def1d05e5b67e262cf1c5cc7";
const RETAINED_ARTIFACTS: [(&str, usize, &str, &str); RETAINED_RECORD_COUNT] = [
    (
        "README",
        941,
        "549c61dafe15162306cb6f59db5847d3f6cb66e2e9e2473a710ea8b3488f3d34",
        "1d86e5e8f1be79b3fe9bbdbb2fab4803c31088f3ce3326d16f70314e71aec550",
    ),
    (
        "bug_1692.cir",
        0,
        "e3b0c44298fc1c149afbf4c8996fb92427ae41e4649b934ca495991b7852b855",
        "af1349b9f5f9a1a6a0404dea36dcc9499bcb25c9adc112b7cc9a93cae41f3262",
    ),
    (
        "bug_1692_0.cir",
        8_142,
        "b80062cc4bde8d3075497dc0677795cefd1511fa844e664e3bd27457a9a9261e",
        "81cc1272e5bd2e4e1a0160f925e16dc6fb3ef45712c49cc275c49bf4a77df80f",
    ),
    (
        "bug_1692_1.cir",
        8_126,
        "83c187ae9115a929b24b151a8f624e00cbbf4052a93a860798845d82a9313dac",
        "2b8589c3460ec16d7493a31c7b93e71cda3a95b6bb77abfbff82e59474335fe4",
    ),
    (
        "bug_1692_2.cir",
        8_142,
        "909133bc1f81fb57938636a65fa362b40d35cffd0257c1fc36a8e0d3bec7e64c",
        "d346de44b0f175fdb267373e78c7ba6f74682158adad095220ddd9bc09b8185f",
    ),
    (
        "bug_1692_3.cir",
        8_314,
        "7d201b8826d036501639654deb467f2b2ed8ee96525909a1e232fa5cb1c85dd1",
        "8eb3b6fa548a6c013299939a56a3b67dbe76a38b1d232335dee57dabafd53034",
    ),
    (
        "bug_1692_noworkee.cir",
        8_238,
        "fa78f802272e932b4f8ff7d57048b8a4bf73c1eaae573193ef623e9e474f8029",
        "a8d8b07d017fff5538c957f271f4f5299e852ad11fa7bbc8e1ee21163b77bbbb",
    ),
];

#[cfg(test)]
const NOWORKEE_PATH: &str = "Netlists/Certification_Tests/BUG_1692/bug_1692_noworkee.cir";
const NOWORKEE_RECORD: &str = "netlists/certification_tests/bug_1692/bug_1692_noworkee.cir";

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub(super) enum Bug1692Role {
    WrapperOwner,
    Braced,
    Naked,
    Quoted,
    Parameterized,
}

impl Bug1692Role {
    pub(super) const ALL: [Self; 5] = [
        Self::WrapperOwner,
        Self::Braced,
        Self::Naked,
        Self::Quoted,
        Self::Parameterized,
    ];
    const WORKERS: [Self; 4] = [Self::Braced, Self::Naked, Self::Quoted, Self::Parameterized];

    pub(super) fn for_record(record: &str) -> Option<Self> {
        let record = XyceTestRunner::normalize_manifest_key(record);
        Self::ALL.into_iter().find(|role| role.record() == record)
    }

    pub(super) const fn contract(self) -> &'static str {
        match self {
            Self::WrapperOwner => BUG1692_OWNER_CONTRACT,
            _ => BUG1692_WORKER_CONTRACT,
        }
    }

    pub(super) const fn path(self) -> &'static str {
        match self {
            Self::WrapperOwner => "Netlists/Certification_Tests/BUG_1692/bug_1692.cir",
            Self::Braced => "Netlists/Certification_Tests/BUG_1692/bug_1692_0.cir",
            Self::Naked => "Netlists/Certification_Tests/BUG_1692/bug_1692_1.cir",
            Self::Quoted => "Netlists/Certification_Tests/BUG_1692/bug_1692_2.cir",
            Self::Parameterized => "Netlists/Certification_Tests/BUG_1692/bug_1692_3.cir",
        }
    }

    const fn file_name(self) -> &'static str {
        match self {
            Self::WrapperOwner => "bug_1692.cir",
            Self::Braced => "bug_1692_0.cir",
            Self::Naked => "bug_1692_1.cir",
            Self::Quoted => "bug_1692_2.cir",
            Self::Parameterized => "bug_1692_3.cir",
        }
    }

    fn record(self) -> &'static str {
        match self {
            Self::WrapperOwner => "netlists/certification_tests/bug_1692/bug_1692.cir",
            Self::Braced => "netlists/certification_tests/bug_1692/bug_1692_0.cir",
            Self::Naked => "netlists/certification_tests/bug_1692/bug_1692_1.cir",
            Self::Quoted => "netlists/certification_tests/bug_1692/bug_1692_2.cir",
            Self::Parameterized => "netlists/certification_tests/bug_1692/bug_1692_3.cir",
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
struct Bug1692Geometry {
    name: String,
    nodes: Vec<String>,
    model: String,
    values: BTreeMap<String, u64>,
}

impl XyceTestRunner {
    pub(super) fn validate_bug1692_historical_provenance() -> Result<(), String> {
        let mut records = HISTORICAL_ARTIFACTS
            .into_iter()
            .map(|(path, bytes, sha256, content_blake3)| {
                format!(
                    "{UPSTREAM_REGRESSION_COMMIT}\t{UPSTREAM_RELEASE_TAG}\t{path}\t{bytes}\t{sha256}\t{content_blake3}"
                )
            })
            .collect::<Vec<_>>();
        records.sort();
        let stream = records.join("\n");
        let sha256 = format!("{:x}", Sha256::digest(stream.as_bytes()));
        let content_blake3 = blake3::hash(stream.as_bytes()).to_hex().to_string();
        if records.len() != HISTORICAL_RECORD_COUNT
            || stream.len() != HISTORICAL_RECORD_BYTES
            || sha256 != HISTORICAL_RECORDS_SHA256
            || content_blake3 != HISTORICAL_RECORDS_BLAKE3
        {
            return Err(format!(
                "{LABEL} historical Release-7.10 provenance changed: records={}/{HISTORICAL_RECORD_COUNT}, bytes={}/{HISTORICAL_RECORD_BYTES}, sha256={sha256}, blake3={content_blake3}",
                records.len(),
                stream.len(),
            ));
        }
        Ok(())
    }

    fn validate_bug1692_provenance(
        &self,
        deck: &XyceDeck,
        role: Bug1692Role,
    ) -> Result<BTreeMap<Bug1692Role, Vec<u8>>, String> {
        Self::validate_bug1692_historical_provenance()?;
        if deck.section != XyceDeckSection::Netlists
            || Self::normalize_manifest_key(&deck.relative_path) != role.record()
            || Self::normalize_manifest_key(&self.relative_key(&deck.path)) != role.record()
            || !Self::same_path(&deck.path, &self.root.join(role.path()))
        {
            return Err(format!("recognized {LABEL} member is not canonical"));
        }

        let family = self.root.join(FAMILY_DIRECTORY);
        let metadata = fs::symlink_metadata(&family)
            .map_err(|error| format!("failed to inspect {LABEL} family: {error}"))?;
        if metadata.file_type().is_symlink() || !metadata.file_type().is_dir() {
            return Err(format!(
                "{LABEL} family must be a regular non-symlink directory"
            ));
        }
        let expected = RETAINED_ARTIFACTS
            .into_iter()
            .map(|artifact| (artifact.0.to_ascii_lowercase(), artifact))
            .collect::<BTreeMap<_, _>>();
        let mut observed_names = BTreeSet::new();
        let mut records = Vec::new();
        let mut sources = BTreeMap::new();
        for entry in fs::read_dir(&family)
            .map_err(|error| format!("failed to enumerate {LABEL}: {error}"))?
        {
            let entry = entry.map_err(|error| format!("failed to read {LABEL} member: {error}"))?;
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
            if !observed_names.insert(key.clone()) {
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
            if canonical.len() != expected_bytes
                || format!("{:x}", Sha256::digest(&canonical)) != expected_sha256
                || blake3::hash(&canonical).to_hex().as_str() != expected_blake3
            {
                return Err(format!("{LABEL} retained member {name:?} changed"));
            }
            records.push(format!(
                "{FAMILY_DIRECTORY}/{expected_name}\t{expected_bytes}\t{expected_sha256}\t{expected_blake3}"
            ));
            if let Some(member_role) =
                Bug1692Role::for_record(&format!("{FAMILY_DIRECTORY}/{expected_name}"))
            {
                self.reject_wrapper_output_artifacts(&path)
                    .map_err(|error| format!("{LABEL} {expected_name} {error}"))?;
                sources.insert(member_role, bytes);
            } else if expected_name.eq_ignore_ascii_case("bug_1692_noworkee.cir") {
                self.reject_wrapper_output_artifacts(&path)
                    .map_err(|error| format!("{LABEL} {expected_name} {error}"))?;
            }
        }
        records.sort();
        let stream = records.join("\n");
        let sha256 = format!("{:x}", Sha256::digest(stream.as_bytes()));
        let content_blake3 = blake3::hash(stream.as_bytes()).to_hex().to_string();
        if observed_names != expected.keys().cloned().collect()
            || sources.len() != Bug1692Role::ALL.len()
            || records.len() != RETAINED_RECORD_COUNT
            || stream.len() != RETAINED_RECORD_BYTES
            || sha256 != RETAINED_RECORDS_SHA256
            || content_blake3 != RETAINED_RECORDS_BLAKE3
        {
            return Err(format!(
                "{LABEL} retained census changed: records={}/{RETAINED_RECORD_COUNT}, bytes={}/{RETAINED_RECORD_BYTES}, sha256={sha256}, blake3={content_blake3}",
                records.len(),
                stream.len(),
            ));
        }

        let owners = Self::load_upstream_wrapper_decks(&self.root)
            .into_iter()
            .filter(|record| record.starts_with(FAMILY_PREFIX))
            .collect::<BTreeSet<_>>();
        if owners != BTreeSet::from([Bug1692Role::WrapperOwner.record().to_string()]) {
            return Err(format!("{LABEL} wrapper ownership changed: {owners:?}"));
        }
        let exclusions = Self::load_upstream_exclusions(&self.root)
            .map_err(|error| format!("{LABEL} exclusion manifest is invalid: {error}"))?;
        let family_rows = exclusions
            .iter()
            .filter(|(record, _)| record.starts_with(FAMILY_PREFIX))
            .collect::<BTreeMap<_, _>>();
        let expected_excluded = Bug1692Role::WORKERS
            .into_iter()
            .map(|worker| worker.record().to_string())
            .chain([NOWORKEE_RECORD.to_string()])
            .collect::<BTreeSet<_>>();
        if family_rows
            .keys()
            .map(|record| record.to_string())
            .collect::<BTreeSet<_>>()
            != expected_excluded
        {
            return Err(format!("{LABEL} exclusion census changed"));
        }
        for worker in Bug1692Role::WORKERS {
            let row = family_rows
                .get(&worker.record().to_string())
                .ok_or_else(|| format!("{LABEL} lost {} qualification", worker.file_name()))?;
            if row.source != EXCLUSION_SOURCE
                || !matches!(&row.disposition,
                    XyceUpstreamExclusionDisposition::RspiceIndependentlyQualified { expected_contract }
                        if expected_contract == BUG1692_WORKER_CONTRACT)
            {
                return Err(format!(
                    "{LABEL} {} qualification changed",
                    worker.file_name()
                ));
            }
        }
        let noworkee = family_rows
            .get(&NOWORKEE_RECORD.to_string())
            .ok_or_else(|| format!("{LABEL} lost noworkee exclusion"))?;
        if noworkee.source != EXCLUSION_SOURCE
            || !matches!(
                noworkee.disposition,
                XyceUpstreamExclusionDisposition::Excluded
            )
        {
            return Err(format!("{LABEL} noworkee must remain ordinarily excluded"));
        }
        match fs::symlink_metadata(self.root.join("OutputData/Certification_Tests/BUG_1692")) {
            Err(error) if error.kind() == std::io::ErrorKind::NotFound => {}
            Ok(_) => {
                return Err(format!(
                    "{LABEL} must not acquire an invented numerical gold"
                ));
            }
            Err(error) => return Err(format!("failed to inspect {LABEL} OutputData: {error}")),
        }
        Ok(sources)
    }

    fn validate_bug1692_unique_effective_param(
        params: &[(String, Value)],
        model_name: &str,
        param_name: &str,
        expected: Value,
    ) -> Result<(), String> {
        let authored = params
            .iter()
            .filter(|(candidate, _)| candidate.eq_ignore_ascii_case(param_name))
            .map(|(_, value)| *value)
            .collect::<Vec<_>>();
        if authored.len() != 1 {
            return Err(format!(
                "{LABEL} {model_name} must contain exactly one {param_name}; found {}",
                authored.len()
            ));
        }
        let effective = Self::numeric_param_value(params, param_name)
            .ok_or_else(|| format!("{LABEL} {model_name} lost effective {param_name} semantics"))?;
        if !authored[0].is_finite()
            || authored[0].to_bits() != expected.to_bits()
            || effective.to_bits() != expected.to_bits()
        {
            return Err(format!(
                "{LABEL} {model_name} effective {param_name} must remain {expected}; authored={:?}, effective={effective}",
                authored[0]
            ));
        }
        Ok(())
    }

    fn validate_bug1692_model(
        netlist: &Netlist,
        name: &str,
        model_type: &str,
    ) -> Result<(), String> {
        let model = netlist
            .models
            .iter()
            .find(|model| model.name.eq_ignore_ascii_case(name))
            .ok_or_else(|| format!("{LABEL} lost {name} model"))?;
        if !model.model_type.eq_ignore_ascii_case(model_type) {
            return Err(format!(
                "{LABEL} {name} must remain native BSIMSOI LEVEL=10/SOIMOD=0 (B3SOIPD route)"
            ));
        }
        Self::validate_bug1692_unique_effective_param(&model.params, name, "LEVEL", 10.0)?;
        Self::validate_bug1692_unique_effective_param(&model.params, name, "SOIMOD", 0.0)?;
        if !Self::model_is_native_b3soi_mosfet(model, &[]) {
            return Err(format!(
                "{LABEL} {name} effective model semantics no longer select native BSIMSOI"
            ));
        }
        Ok(())
    }

    fn validate_bug1692_assembled_native_route(
        &self,
        netlist: &Netlist,
        source_name: &str,
    ) -> Result<(), String> {
        let circuit = self
            .create_xyce_engine()
            .build_circuit(netlist)
            .map_err(|error| format!("{LABEL} {source_name} circuit assembly failed: {error}"))?;
        if circuit.has_generated_veriloga_devices() {
            return Err(format!(
                "{LABEL} {source_name} assembled through a generated Verilog-A device"
            ));
        }
        let report = circuit.device_op_report();
        let observed = report
            .entries
            .iter()
            .map(|entry| format!("{}:{}", entry.name, entry.device_kind))
            .collect::<Vec<_>>();
        let expected = ["x1.mn1", "x1.mp1"];
        if report.entries.len() != expected.len() {
            return Err(format!(
                "{LABEL} {source_name} must assemble exactly two semiconductor devices; observed {observed:?}"
            ));
        }
        for expected_name in expected {
            let matches = report
                .entries
                .iter()
                .filter(|entry| Self::device_instance_names_match(&entry.name, expected_name))
                .collect::<Vec<_>>();
            if matches.len() != 1 || !matches[0].device_kind.eq_ignore_ascii_case("B3SOIPD") {
                return Err(format!(
                    "{LABEL} {source_name} {expected_name} must assemble uniquely through native B3SOIPD; observed {observed:?}"
                ));
            }
        }
        Ok(())
    }

    fn bug1692_geometry(netlist: &Netlist) -> Result<Vec<Bug1692Geometry>, String> {
        let flattened = flatten_netlist_with_models(netlist)
            .map_err(|error| format!("{LABEL} hierarchy no longer flattens: {error}"))?;
        if !flattened.scoped_models.is_empty()
            || !flattened.scoped_initial_conditions.is_empty()
            || !flattened.scoped_node_sets.is_empty()
            || !flattened.scoped_startup_directives.is_empty()
            || !flattened.xspice_auto_bridge_node_hints.is_empty()
        {
            return Err(format!(
                "{LABEL} acquired scoped/generated flattening state"
            ));
        }
        let mut geometry = Vec::new();
        for element in &flattened.elements {
            let ElementKind::Mosfet {
                model,
                compact_syntax,
                instance_params,
                deferred_params,
                ..
            } = &element.kind
            else {
                continue;
            };
            if *compact_syntax
                || !deferred_params.is_empty()
                || element.provenance != ElementProvenance::Authored
                || instance_params.len() != 7
            {
                return Err(format!("{LABEL} flattened MOS shape changed: {element:?}"));
            }
            let values = instance_params
                .iter()
                .map(|(name, value)| (name.to_ascii_uppercase(), value.to_bits()))
                .collect::<BTreeMap<_, _>>();
            if values.len() != 7
                || !["L", "W", "M", "AD", "AS", "PD", "PS"]
                    .into_iter()
                    .all(|name| values.contains_key(name))
            {
                return Err(format!("{LABEL} flattened MOS geometry keys changed"));
            }
            geometry.push(Bug1692Geometry {
                name: element.name.to_ascii_lowercase(),
                nodes: element
                    .nodes
                    .iter()
                    .map(|node| node.to_ascii_lowercase())
                    .collect(),
                model: model.to_ascii_lowercase(),
                values,
            });
        }
        geometry.sort_by(|left, right| left.name.cmp(&right.name));
        if geometry.len() != 2 {
            return Err(format!(
                "{LABEL} must flatten to exactly two inverter MOSFETs"
            ));
        }
        Ok(geometry)
    }

    fn validate_bug1692_geometry(geometry: &[Bug1692Geometry]) -> Result<(), String> {
        let expect = |actual: &Bug1692Geometry,
                      name: &str,
                      nodes: &[&str],
                      model: &str,
                      l: Value,
                      w: Value,
                      ad: Value,
                      perimeter: Value|
         -> Result<(), String> {
            let expected = BTreeMap::from([
                ("L".to_string(), l.to_bits()),
                ("W".to_string(), w.to_bits()),
                ("M".to_string(), 1.0f64.to_bits()),
                ("AD".to_string(), ad.to_bits()),
                ("AS".to_string(), ad.to_bits()),
                ("PD".to_string(), perimeter.to_bits()),
                ("PS".to_string(), perimeter.to_bits()),
            ]);
            if actual.name != name
                || actual
                    .nodes
                    .iter()
                    .map(String::as_str)
                    .ne(nodes.iter().copied())
                || actual.model != model
                || actual.values != expected
            {
                return Err(format!(
                    "{LABEL} flattened {name} geometry changed: {actual:?}"
                ));
            }
            Ok(())
        };
        expect(
            &geometry[0],
            "x1.mn1",
            &["out1", "in1", "vss1", "0", "nbulk1"],
            "mn",
            0.35 * 1.0e-6,
            2.0 * 1.0e-6,
            (0.95 * 1.0e-6) * (2.0 * 1.0e-6),
            2.0 * ((0.95 * 1.0e-6) + (2.0 * 1.0e-6)),
        )?;
        expect(
            &geometry[1],
            "x1.mp1",
            &["out1", "in1", "vdd1", "0", "pbulk1"],
            "mp",
            0.45 * 1.0e-6,
            4.0 * 1.0e-6,
            (0.95 * 1.0e-6) * (4.0 * 1.0e-6),
            2.0 * ((0.95 * 1.0e-6) + (4.0 * 1.0e-6)),
        )
    }

    fn validate_bug1692_worker(
        &self,
        role: Bug1692Role,
        source: &str,
        path: &Path,
    ) -> Result<(XyceStaticTranPlan, Vec<Bug1692Geometry>), String> {
        if role == Bug1692Role::WrapperOwner {
            return Err(format!("{LABEL} owner is not an executable worker"));
        }
        if source
            .lines()
            .any(|line| line.trim_start().to_ascii_uppercase().starts_with("*COMP"))
        {
            return Err(format!(
                "{LABEL} {} acquired a *COMP override",
                role.file_name()
            ));
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
            || plan.output_override
            || plan.timeint_conststep
            || !plan.steps.is_empty()
            || plan.wrapper_tolerance.is_some()
            || plan.tran.step.to_bits() != (20.0f64 * 1.0e-9).to_bits()
            || plan.tran.stop.to_bits() != (30.0f64 * 1.0e-6).to_bits()
            || plan.tran.start.is_some()
            || plan.tran.max_step.is_some()
            || plan.tran.uic
            || plan.print.as_ref().is_none_or(|print| {
                print
                    .probes
                    .iter()
                    .map(|probe| Self::normalize_probe(probe))
                    .ne(["v(in1)".to_string(), "v(out1)".to_string()])
            })
        {
            return Err(format!(
                "{LABEL} {} plan changed: {plan:?}",
                role.file_name()
            ));
        }
        let netlist = Self::parse_xyce_netlist(source, path)
            .map_err(|error| format!("{LABEL} {} no longer parses: {error}", role.file_name()))?;
        if !netlist.diagnostics.is_empty()
            || netlist.elements.len() != 8
            || netlist.models.len() != 2
            || netlist.subcircuits.len() != 1
            || netlist.analyses.len() != 1
            || netlist.output_requests.len() != 1
            || !netlist.veriloga_includes.is_empty()
            || !netlist.spef_includes.is_empty()
            || !netlist.data_tables.is_empty()
            || !netlist.measurements.is_empty()
            || !netlist.initial_conditions.is_empty()
            || netlist.device_initial_conditions.is_some()
            || !netlist.node_sets.is_empty()
            || !netlist.global_nodes.is_empty()
            || !matches!(&netlist.analyses[0], AnalysisCommand::Tran {
                step, stop, start: None, max_step: None, uic: false,
            } if step.to_bits() == (20.0f64 * 1.0e-9).to_bits()
                && stop.to_bits() == (30.0f64 * 1.0e-6).to_bits())
        {
            return Err(format!(
                "{LABEL} {} typed envelope changed",
                role.file_name()
            ));
        }
        Self::validate_bug1692_model(&netlist, "MN", "NMOS")?;
        Self::validate_bug1692_model(&netlist, "MP", "PMOS")?;
        let request = &netlist.output_requests[0];
        if request.directive != OutputDirectiveKind::Print
            || request.analysis != Some(rspice_core::netlist::OutputAnalysisKind::Tran)
            || request.name.is_some()
            || request.print_delimiter.as_ref() != Some(&PrintDelimiter::Whitespace)
            || !request.expressions.is_empty()
            || request.dependencies.len() != 2
            || request
                .dependencies
                .iter()
                .zip(["in1", "out1"])
                .any(|(dependency, node)| {
                    dependency.kind != OutputSymbolKind::Node
                        || dependency.expression
                        || !dependency.operator.eq_ignore_ascii_case("V")
                        || !dependency.symbol.eq_ignore_ascii_case(node)
                })
        {
            return Err(format!(
                "{LABEL} {} typed PRINT changed: {request:?}",
                role.file_name()
            ));
        }
        let geometry = Self::bug1692_geometry(&netlist)?;
        Self::validate_bug1692_geometry(&geometry)?;
        for mos in &geometry {
            if !Self::netlist_device_is_native_b3soi_mosfet(&netlist, &mos.name) {
                return Err(format!("{LABEL} {} lost native B3SOIPD routing", mos.name));
            }
        }
        self.validate_bug1692_assembled_native_route(&netlist, role.file_name())?;
        Ok((plan, geometry))
    }

    fn validate_bug1692_table(role: Bug1692Role, table: &XycePrnTable) -> Result<(), String> {
        let expected = ["Index", "TIME", "V(in1)", "V(out1)"];
        if table.columns.len() != expected.len()
            || table
                .columns
                .iter()
                .zip(expected)
                .any(|(actual, expected)| !actual.eq_ignore_ascii_case(expected))
            || table.rows.len() < 2
        {
            return Err(format!(
                "{LABEL} {} output schema changed: columns={:?}, rows={}",
                role.file_name(),
                table.columns,
                table.rows.len()
            ));
        }
        let mut previous_time = None;
        for (index, row) in table.rows.iter().enumerate() {
            if row.len() != expected.len()
                || row.iter().any(|value| !value.is_finite())
                || row[0].to_bits() != (index as Value).to_bits()
                || previous_time.is_some_and(|previous| row[1] <= previous)
            {
                return Err(format!(
                    "{LABEL} {} malformed row {index}",
                    role.file_name()
                ));
            }
            previous_time = Some(row[1]);
        }
        let first = table.rows.first().expect("checked nonempty");
        let last = table.rows.last().expect("checked nonempty");
        let range = |column: usize| {
            let minimum = table
                .rows
                .iter()
                .map(|row| row[column])
                .fold(Value::INFINITY, Value::min);
            let maximum = table
                .rows
                .iter()
                .map(|row| row[column])
                .fold(Value::NEG_INFINITY, Value::max);
            maximum - minimum
        };
        if first[1].abs() > 1.0e-18
            || (last[1] - 30.0e-6).abs() > 1.0e-14
            || range(2) < 1.0e-3
            || range(3) < 1.0e-3
        {
            return Err(format!(
                "{LABEL} {} output domain became trivial",
                role.file_name()
            ));
        }
        Ok(())
    }

    fn compare_bug1692_relation(
        &self,
        worker: Bug1692Role,
        good: &XycePrnTable,
        test: &XycePrnTable,
    ) -> Result<(), String> {
        let exact_diagnostic = match self.compare_serialized_default_prn_tables(good, test) {
            Ok(mismatches) if mismatches.is_empty() => return Ok(()),
            Ok(mismatches) => format!("{mismatches:?}"),
            Err(error) => format!("comparator error: {error}"),
        };
        let fallback = self.compare_xyce_verify_transient_tables_with_uniform_tolerance(
            good,
            test,
            XyceVerifyTransientTolerance::release_7_10_default(),
            XYCE_DEFAULT_PRN_SCIENTIFIC_PRECISION,
        )
        .map_err(|error| {
            format!(
                "{LABEL} {} exact PRN differed ({exact_diagnostic}) and directional Release-7.10 xyce_verify errored: {error}",
                worker.file_name()
            )
        })?;
        if fallback.is_empty() {
            Ok(())
        } else {
            Err(format!(
                "{LABEL} {} failed both exact PRN and directional Release-7.10 xyce_verify: exact={exact_diagnostic}, fallback={fallback:?}",
                worker.file_name()
            ))
        }
    }

    pub(super) fn validate_bug1692_oracle(
        &self,
        deck: &XyceDeck,
        role: Bug1692Role,
        start: Instant,
    ) -> Result<(), String> {
        let abort = DeadlineAbort::new(start, self.config.max_time_per_test_ms.max(1));
        if abort.is_aborted() {
            return Err(format!("{LABEL} deadline expired before provenance"));
        }
        let members = self.validate_bug1692_provenance(deck, role)?;
        let mut baseline_geometry = None;
        let mut tables = BTreeMap::new();
        for worker in Bug1692Role::WORKERS {
            let bytes = members
                .get(&worker)
                .ok_or_else(|| format!("{LABEL} lost {}", worker.file_name()))?;
            let source = std::str::from_utf8(bytes)
                .map_err(|error| format!("{LABEL} {} is not UTF-8: {error}", worker.file_name()))?;
            let path = self.root.join(worker.path());
            let (plan, geometry) = self.validate_bug1692_worker(worker, source, &path)?;
            if let Some(baseline) = &baseline_geometry {
                if baseline != &geometry {
                    return Err(format!(
                        "{LABEL} {} no longer flattens to baseline geometry",
                        worker.file_name()
                    ));
                }
            } else {
                baseline_geometry = Some(geometry);
            }
            let (netlist, result) = self
                .run_transient_family_plan(&plan, start, None, None)
                .map_err(|error| match error {
                    SimulationError::Aborted => format!("{LABEL} execution exceeded deadline"),
                    other => format!("{LABEL} {} execution failed: {other}", worker.file_name()),
                })?;
            let executed_geometry = Self::bug1692_geometry(&netlist)?;
            if !netlist.veriloga_includes.is_empty()
                || baseline_geometry.as_ref() != Some(&executed_geometry)
            {
                return Err(format!(
                    "{LABEL} {} execution geometry changed",
                    worker.file_name()
                ));
            }
            let table = Self::transient_family_result_to_prn_table(&plan, &netlist, &result)
                .map_err(|error| {
                    format!(
                        "{LABEL} {} PRN rendering failed: {error}",
                        worker.file_name()
                    )
                })?;
            Self::validate_bug1692_table(worker, &table)?;
            tables.insert(worker, table);
            if abort.is_aborted() {
                return Err(format!("{LABEL} deadline expired between independent runs"));
            }
        }

        let good = tables.get(&Bug1692Role::Braced).expect("baseline ran");
        for worker in [
            Bug1692Role::Naked,
            Bug1692Role::Quoted,
            Bug1692Role::Parameterized,
        ] {
            let test = tables.get(&worker).expect("worker ran");
            self.compare_bug1692_relation(worker, good, test)?;
        }
        self.validate_bug1692_provenance(deck, role)?;
        if abort.is_aborted() {
            return Err(format!("{LABEL} post-provenance exceeded deadline"));
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

    fn deck(root: &Path, role: Bug1692Role) -> XyceDeck {
        XyceDeck {
            path: root.join(role.path()),
            section: XyceDeckSection::Netlists,
            relative_path: role.path().to_string(),
        }
    }

    fn fixture(label: &str) -> (tempfile::TempDir, XyceDeck) {
        let temporary = tempfile::Builder::new()
            .prefix(&format!("rspice-xyce-bug1692-{label}-"))
            .tempdir()
            .expect("create BUG1692 fixture");
        let root = temporary.path();
        let family = root.join(FAMILY_DIRECTORY);
        fs::create_dir_all(&family).expect("create BUG1692 family");
        let canonical = corpus_root().join(FAMILY_DIRECTORY);
        for (name, ..) in RETAINED_ARTIFACTS {
            fs::copy(canonical.join(name), family.join(name)).expect("copy BUG1692 member");
        }
        fs::write(
            root.join(HARNESS_MANIFEST_FILE),
            format!(
                "{}\t{REQUIRES_UPSTREAM_WRAPPER_CONTRACT}\n",
                Bug1692Role::WrapperOwner.path()
            ),
        )
        .expect("write BUG1692 harness manifest");
        let mut rows = vec![
            format!("schema_version\t{UPSTREAM_EXCLUSIONS_SCHEMA_VERSION}"),
            format!("source_commit\t{UPSTREAM_EXCLUSIONS_SOURCE_COMMIT}"),
            format!("source_netlists_tree\t{UPSTREAM_EXCLUSIONS_SOURCE_NETLISTS_TREE}"),
        ];
        rows.extend(Bug1692Role::WORKERS.map(|worker| {
            format!(
                "{}\t{EXCLUSION_SOURCE}\t{RSPICE_INDEPENDENTLY_QUALIFIED_DISPOSITION}\t{BUG1692_WORKER_CONTRACT}",
                worker.path()
            )
        }));
        rows.push(format!(
            "{NOWORKEE_PATH}\t{EXCLUSION_SOURCE}\t{UPSTREAM_EXCLUDED_DISPOSITION}"
        ));
        rows[3..].sort();
        fs::write(
            root.join(UPSTREAM_EXCLUSIONS_MANIFEST_FILE),
            rows.join("\n") + "\n",
        )
        .expect("write BUG1692 exclusions");
        let owner = deck(root, Bug1692Role::WrapperOwner);
        (temporary, owner)
    }

    #[test]
    fn bug1692_roles_and_historical_provenance_are_exact() {
        XyceTestRunner::validate_bug1692_historical_provenance()
            .expect("BUG1692 historical provenance");
        let records = Bug1692Role::ALL
            .into_iter()
            .map(Bug1692Role::record)
            .collect::<BTreeSet<_>>();
        assert_eq!(records.len(), Bug1692Role::ALL.len());
        assert_eq!(Bug1692Role::for_record(NOWORKEE_RECORD), None);
    }

    #[test]
    fn bug1692_workers_have_exact_typed_flattened_geometry_and_native_route() {
        let runner = XyceTestRunner::new(corpus_root(), XyceRunnerConfig::default());
        let mut baseline = None;
        for role in Bug1692Role::WORKERS {
            let path = runner.root.join(role.path());
            let source = fs::read_to_string(&path).expect("read BUG1692 worker");
            let (_, geometry) = runner
                .validate_bug1692_worker(role, &source, &path)
                .unwrap_or_else(|error| panic!("{role:?}: {error}"));
            if let Some(expected) = &baseline {
                assert_eq!(expected, &geometry);
            } else {
                baseline = Some(geometry);
            }
        }
    }

    #[test]
    fn bug1692_models_reject_duplicate_level_and_soimod() {
        let path = corpus_root().join(Bug1692Role::Braced.path());
        let source = fs::read_to_string(&path).expect("read BUG1692 worker");
        for (needle, replacement, param_name) in [
            ("LEVEL   = 10", "LEVEL   = 10 LEVEL = 10", "LEVEL"),
            ("SOIMOD  = 0", "SOIMOD  = 0 SOIMOD = 0", "SOIMOD"),
        ] {
            let duplicate = source.replacen(needle, replacement, 1);
            assert_ne!(duplicate, source, "fixture contains {param_name}");
            let netlist = XyceTestRunner::parse_xyce_netlist(&duplicate, &path)
                .unwrap_or_else(|error| panic!("parse duplicate {param_name}: {error}"));
            let error = XyceTestRunner::validate_bug1692_model(&netlist, "MP", "PMOS")
                .expect_err("duplicate model parameter must fail closed");
            assert!(
                error.contains(&format!("exactly one {param_name}")),
                "unexpected {param_name} diagnostic: {error}"
            );
        }
    }

    #[test]
    fn bug1692_effective_soimod_and_assembled_route_must_remain_pd() {
        let root = corpus_root();
        let runner = XyceTestRunner::new(&root, XyceRunnerConfig::default());
        let path = root.join(Bug1692Role::Braced.path());
        let source = fs::read_to_string(&path).expect("read BUG1692 worker");
        let non_pd = source.replace("SOIMOD  = 0", "SOIMOD  = 1");
        assert_eq!(non_pd.matches("SOIMOD  = 1").count(), 2);
        let netlist = XyceTestRunner::parse_xyce_netlist(&non_pd, &path)
            .expect("parse non-PD BUG1692 worker");
        for (model_name, model_type) in [("MN", "NMOS"), ("MP", "PMOS")] {
            let error = XyceTestRunner::validate_bug1692_model(&netlist, model_name, model_type)
                .expect_err("effective SOIMOD=1 must fail closed");
            assert!(
                error.contains("effective SOIMOD must remain 0"),
                "unexpected effective-semantics diagnostic: {error}"
            );
        }
        let error = runner
            .validate_bug1692_assembled_native_route(&netlist, "SOIMOD=1 fixture")
            .expect_err("assembled B3SOIDD route must fail closed");
        assert!(
            error.contains("B3SOIPD"),
            "unexpected route diagnostic: {error}"
        );
        assert!(
            error.contains("B3SOIDD"),
            "route proof was not assembled: {error}"
        );
    }

    #[test]
    fn bug1692_provenance_rejects_source_role_and_noworkee_drift() {
        let (temporary, owner) = fixture("canonical");
        XyceTestRunner::new(temporary.path(), XyceRunnerConfig::default())
            .validate_bug1692_provenance(&owner, Bug1692Role::WrapperOwner)
            .expect("canonical BUG1692 fixture");

        let (temporary, owner) = fixture("source");
        fs::write(
            temporary.path().join(Bug1692Role::Naked.path()),
            "* changed\n",
        )
        .expect("mutate worker");
        assert!(
            XyceTestRunner::new(temporary.path(), XyceRunnerConfig::default())
                .validate_bug1692_provenance(&owner, Bug1692Role::WrapperOwner)
                .is_err()
        );

        let (temporary, owner) = fixture("noworkee");
        let exclusions = temporary.path().join(UPSTREAM_EXCLUSIONS_MANIFEST_FILE);
        let text = fs::read_to_string(&exclusions).expect("read exclusions");
        fs::write(
            &exclusions,
            text.replace(
                &format!("{NOWORKEE_PATH}\t{EXCLUSION_SOURCE}\t{UPSTREAM_EXCLUDED_DISPOSITION}"),
                &format!("{NOWORKEE_PATH}\t{EXCLUSION_SOURCE}\t{RSPICE_INDEPENDENTLY_QUALIFIED_DISPOSITION}\t{BUG1692_WORKER_CONTRACT}"),
            ),
        )
        .expect("promote noworkee incorrectly");
        assert!(
            XyceTestRunner::new(temporary.path(), XyceRunnerConfig::default())
                .validate_bug1692_provenance(&owner, Bug1692Role::WrapperOwner)
                .is_err()
        );
    }

    #[test]
    fn bug1692_oracle_executes_all_four_representations() {
        let root = corpus_root();
        let runner = XyceTestRunner::new(&root, XyceRunnerConfig::default());
        runner
            .validate_bug1692_oracle(
                &deck(&root, Bug1692Role::WrapperOwner),
                Bug1692Role::WrapperOwner,
                Instant::now(),
            )
            .expect("execute BUG1692 relation");
    }

    #[test]
    fn bug1692_oracle_rejects_expired_deadline() {
        let root = corpus_root();
        let runner = XyceTestRunner::new(&root, XyceRunnerConfig::default());
        let expired = Instant::now()
            - Duration::from_millis(
                u64::try_from(runner.config.max_time_per_test_ms.max(1) + 1)
                    .expect("timeout fits u64"),
            );
        assert!(
            runner
                .validate_bug1692_oracle(
                    &deck(&root, Bug1692Role::WrapperOwner),
                    Bug1692Role::WrapperOwner,
                    expired,
                )
                .is_err()
        );
    }

    #[test]
    fn bug1692_directional_xyce_verify_fallback_interpolates_good_to_test_grid() {
        let runner = XyceTestRunner::new(corpus_root(), XyceRunnerConfig::default());
        let columns = ["Index", "TIME", "V(in1)", "V(out1)"]
            .map(str::to_string)
            .to_vec();
        let good = XycePrnTable {
            columns: columns.clone(),
            rows: vec![
                vec![0.0, 0.0, 0.0, 4.0],
                vec![1.0, 1.0, 2.0, 2.0],
                vec![2.0, 2.0, 4.0, 0.0],
            ],
        };
        let test = XycePrnTable {
            columns,
            rows: vec![
                vec![0.0, 0.0, 0.0, 4.0],
                vec![1.0, 0.5, 1.0, 3.0],
                vec![2.0, 1.0, 2.0, 2.0],
                vec![3.0, 1.5, 3.0, 1.0],
                vec![4.0, 2.0, 4.0, 0.0],
            ],
        };
        runner
            .compare_bug1692_relation(Bug1692Role::Naked, &good, &test)
            .expect("historical directional fallback accepts equivalent test grid");
        let mut wrong = test;
        wrong.rows[2][3] = 8.0;
        assert!(
            runner
                .compare_bug1692_relation(Bug1692Role::Naked, &good, &wrong)
                .is_err()
        );
    }
}
