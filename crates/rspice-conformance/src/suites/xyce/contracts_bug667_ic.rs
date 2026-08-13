use super::*;
use std::io::Read as _;

const LABEL: &str = "BUG_667_SON hierarchical IC equivalence";
const PREFIX: &str = "netlists/certification_tests/bug_667_son/";
const OWNER_PATH: &str = "Netlists/Certification_Tests/BUG_667_SON/ic_in_subckt.cir";
const REFERENCE_PATH: &str = "Netlists/Certification_Tests/BUG_667_SON/ic_not_in_subckt.cir";
const OWNER_RECORD: &str = "netlists/certification_tests/bug_667_son/ic_in_subckt.cir";
const REFERENCE_RECORD: &str = "netlists/certification_tests/bug_667_son/ic_not_in_subckt.cir";
const WARNING_RECORD: &str = "netlists/certification_tests/bug_667_son/ic_in_subckt_warning.cir";
const NODESET_OWNER_RECORD: &str = "netlists/certification_tests/bug_667_son/nodeset_in_subckt.cir";
const NODESET_REFERENCE_RECORD: &str =
    "netlists/certification_tests/bug_667_son/nodeset_not_in_subckt.cir";
const EXCLUSION_SOURCE: &str = "Netlists/Certification_Tests/BUG_667_SON/exclude";
const OWNER_CONTRACT: &str = "bug667_ic_relational_wrapper_owner";
const REFERENCE_CONTRACT: &str = "bug667_ic_relational_wrapper_explicit_reference";
const NODESET_REFERENCE_CONTRACT: &str = "bug667_nodeset_relational_wrapper_explicit_reference";

const RELEASE_COMMIT: &str = "d6e278e371ec2f3df1325dcff4552e585bc7ecc1";
const RELEASE_TAG: &str = "Release-7.10.0";
const HISTORICAL_RECORD_COUNT: usize = 7;
const HISTORICAL_RECORD_BYTES: usize = 1_715;
const HISTORICAL_RECORDS_SHA256: &str =
    "eac94b27b6c8f41c304195f2adb2ad44210bd7800dd0edcc8bb0441322984878";
const HISTORICAL_RECORDS_BLAKE3: &str =
    "3814622e15ce792da3895988aa4d0350969d9b6038a42531cb2e8b2cb1ab0345";
const HISTORICAL_ARTIFACTS: [(&str, usize, &str, &str); 7] = [
    (
        "Netlists/Certification_Tests/BUG_667_SON/CMakeLists.txt",
        4_924,
        "f3c645edf7ec411cc60c6e6323ce9319d24e5350634477fab65d7e91f9693739",
        "9dc23e2edcc3ca62d2b71f0b10b5dac1c3080a67e2acc9ff8de6eedf28f34909",
    ),
    (
        "Netlists/Certification_Tests/BUG_667_SON/Manifest.txt",
        197,
        "73874cfb7c1e745fcadcabdaa9343c2b805d5a114ef2f8f20ec52132cd6fd752",
        "bc825f8558bcd993bb93063c1b05c614e53ce0b646446724c7747ec285f518f0",
    ),
    (
        "Netlists/Certification_Tests/BUG_667_SON/exclude",
        47,
        "304313504bdb88fdeb83e92eb6fcffdc30cb6a17bc98cbf2e71fd6f9da820752",
        "c227e23bc38c94dd8c6a39f35c2a3f20a42f683303ad63d8385e46f18d3c1a70",
    ),
    (
        OWNER_PATH,
        851,
        "113c1338c5bff5288e82c0a6b04785191c7ed291e288713d24dea4cd4462b086",
        "972acc03b538e5a1fd34c5507ede76c9cf1ad029eeb8b60edd55f87466555b56",
    ),
    (
        "Netlists/Certification_Tests/BUG_667_SON/ic_in_subckt.cir.sh",
        1_216,
        "0d4dce0817b4038b5639c91daa9a40701b8575782ab309ee203b7c971125d1b6",
        "aa02d40d3398541d21657ec307751995b4eefef59993625199cf04a1bdd60303",
    ),
    (
        REFERENCE_PATH,
        820,
        "eff70f142d1976c72f4b56c16be8cd3b05dfa319f2d0cc5bf48708728cf0f6aa",
        "3b0721e4dcadcadb8d6c76f4a58c13315eb019bf67e5aa79fc3e878e13b4470e",
    ),
    (
        "Netlists/Certification_Tests/BUG_667_SON/tags",
        26,
        "1e6b928df7201ec2ef59e9058e3fd4fb216589d8245520003d5603a8f13b4c5b",
        "734e1696605bc661d095ac763edd46b3c764489f5166f85f03781e02239c8cba",
    ),
];

const RETAINED_RECORD_COUNT: usize = 5;
const RETAINED_RECORD_BYTES: usize = 780;
const RETAINED_RECORDS_SHA256: &str =
    "c1b74fcf45920bbdf70b783501f97405e8098c91cb8faff4c0ae958c8cce5db9";
const RETAINED_RECORDS_BLAKE3: &str =
    "271ed7f8648e10d6a99f7bbd2475bb7c5e7b3e92e9901881ef41b10f51e7a071";
const RETAINED_ARTIFACTS: [(&str, usize, &str, &str); 5] = [
    (
        "ic_in_subckt.cir",
        851,
        "113c1338c5bff5288e82c0a6b04785191c7ed291e288713d24dea4cd4462b086",
        "972acc03b538e5a1fd34c5507ede76c9cf1ad029eeb8b60edd55f87466555b56",
    ),
    (
        "ic_in_subckt_warning.cir",
        930,
        "e5534fd350fbfae0a085c5f06758e3876f0986c57d5b0385dee14256f93271b3",
        "15a1cc0a12119fe2cacc830357789ffad883cc7b769eb617ccd8f6b8e7ad3f2f",
    ),
    (
        "ic_not_in_subckt.cir",
        820,
        "eff70f142d1976c72f4b56c16be8cd3b05dfa319f2d0cc5bf48708728cf0f6aa",
        "3b0721e4dcadcadb8d6c76f4a58c13315eb019bf67e5aa79fc3e878e13b4470e",
    ),
    (
        "nodeset_in_subckt.cir",
        873,
        "4eebfecae321f974c6dd231c01c61a8ba0349221f06eca80bd400d4d498661a7",
        "7be6145e070d7a41332dbe5324afbf4aea951b324cbc5b92c733d64c51ee702c",
    ),
    (
        "nodeset_not_in_subckt.cir",
        842,
        "66e74ccb83f67183a65ddfc27624b54f8672848740612e1a475f8f6163bab665",
        "d5d34b0ef3a4255a2ba8c75a92c14dc9a5bf3c5c47b645517d8d0172aebc4525",
    ),
];

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(super) enum Bug667IcRole {
    ScopedOwner,
    ExplicitHierarchicalReference,
}

impl Bug667IcRole {
    #[cfg(test)]
    const ALL: [Self; 2] = [Self::ScopedOwner, Self::ExplicitHierarchicalReference];

    pub(super) fn for_record(record: &str) -> Option<Self> {
        match XyceTestRunner::normalize_manifest_key(record).as_str() {
            OWNER_RECORD => Some(Self::ScopedOwner),
            REFERENCE_RECORD => Some(Self::ExplicitHierarchicalReference),
            _ => None,
        }
    }

    pub(super) fn contract(self) -> &'static str {
        match self {
            Self::ScopedOwner => OWNER_CONTRACT,
            Self::ExplicitHierarchicalReference => REFERENCE_CONTRACT,
        }
    }

    fn path(self) -> &'static str {
        match self {
            Self::ScopedOwner => OWNER_PATH,
            Self::ExplicitHierarchicalReference => REFERENCE_PATH,
        }
    }

    fn record(self) -> &'static str {
        match self {
            Self::ScopedOwner => OWNER_RECORD,
            Self::ExplicitHierarchicalReference => REFERENCE_RECORD,
        }
    }

    fn file_name(self) -> &'static str {
        self.path()
            .rsplit('/')
            .next()
            .expect("BUG667 IC path has name")
    }
}

impl XyceTestRunner {
    pub(super) fn bug667_ic_historical_provenance_records() -> Vec<String> {
        let mut records = HISTORICAL_ARTIFACTS
            .into_iter()
            .map(|(path, bytes, sha256, blake3)| {
                format!("{RELEASE_COMMIT}\t{RELEASE_TAG}\t{path}\t{bytes}\t{sha256}\t{blake3}")
            })
            .collect::<Vec<_>>();
        records.sort();
        records
    }

    pub(super) fn validate_bug667_ic_historical_provenance() -> Result<(), String> {
        let records = Self::bug667_ic_historical_provenance_records();
        let stream = records.join("\n");
        if records.len() != HISTORICAL_RECORD_COUNT
            || stream.len() != HISTORICAL_RECORD_BYTES
            || format!("{:x}", Sha256::digest(stream.as_bytes())) != HISTORICAL_RECORDS_SHA256
            || blake3::hash(stream.as_bytes()).to_hex().as_str() != HISTORICAL_RECORDS_BLAKE3
        {
            return Err(format!(
                "{LABEL} Release provenance changed: records={}/{}, sha256={:x}, blake3={}",
                records.len(),
                stream.len(),
                Sha256::digest(stream.as_bytes()),
                blake3::hash(stream.as_bytes()).to_hex()
            ));
        }
        Ok(())
    }

    fn validate_bug667_ic_directory(&self) -> Result<BTreeMap<String, Vec<u8>>, String> {
        let directory = self.root.join("Netlists/Certification_Tests/BUG_667_SON");
        let metadata = fs::symlink_metadata(&directory)
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
            let raw_limit = expected_bytes
                .checked_mul(2)
                .and_then(|limit| limit.checked_add(1))
                .ok_or_else(|| format!("{LABEL} {name:?} size bound overflowed"))?;
            if metadata.len() > raw_limit as u64 {
                return Err(format!(
                    "{LABEL} retained member {name:?} is too large: {} > {raw_limit}",
                    metadata.len()
                ));
            }
            let mut bytes = Vec::with_capacity((metadata.len() as usize).min(raw_limit));
            fs::File::open(&path)
                .map_err(|error| format!("failed to open {LABEL} {name:?}: {error}"))?
                .take(raw_limit as u64 + 1)
                .read_to_end(&mut bytes)
                .map_err(|error| format!("failed to read {LABEL} {name:?}: {error}"))?;
            if bytes.len() > raw_limit {
                return Err(format!(
                    "{LABEL} retained member {name:?} grew while reading"
                ));
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
        if records.len() != RETAINED_RECORD_COUNT
            || stream.len() != RETAINED_RECORD_BYTES
            || format!("{:x}", Sha256::digest(stream.as_bytes())) != RETAINED_RECORDS_SHA256
            || blake3::hash(stream.as_bytes()).to_hex().as_str() != RETAINED_RECORDS_BLAKE3
        {
            return Err(format!("{LABEL} retained family census changed"));
        }
        Ok(observed)
    }

    fn validate_bug667_ic_provenance(
        &self,
        deck: &XyceDeck,
        role: Bug667IcRole,
    ) -> Result<BTreeMap<String, Vec<u8>>, String> {
        Self::validate_bug667_ic_historical_provenance()?;
        if deck.section != XyceDeckSection::Netlists
            || Self::normalize_manifest_key(&deck.relative_path) != role.record()
            || Self::normalize_manifest_key(&self.relative_key(&deck.path)) != role.record()
            || !Self::same_path(&deck.path, &self.root.join(role.path()))
        {
            return Err(format!(
                "recognized {LABEL} role is not at its canonical path"
            ));
        }
        let owners = Self::load_upstream_wrapper_decks(&self.root)
            .into_iter()
            .filter(|record| record.starts_with(PREFIX))
            .collect::<BTreeSet<_>>();
        if owners
            != BTreeSet::from([
                OWNER_RECORD.to_string(),
                WARNING_RECORD.to_string(),
                NODESET_OWNER_RECORD.to_string(),
            ])
        {
            return Err(format!("{LABEL} wrapper ownership changed: {owners:?}"));
        }
        let exclusions = Self::load_upstream_exclusions(&self.root)
            .map_err(|error| format!("{LABEL} exclusion manifest is invalid: {error}"))?;
        let family = exclusions
            .iter()
            .filter(|(record, _)| record.starts_with(PREFIX))
            .collect::<BTreeMap<_, _>>();
        if family.len() != 2 {
            return Err(format!("{LABEL} requires exactly two promoted controls"));
        }
        for (record, contract) in [
            (REFERENCE_RECORD, REFERENCE_CONTRACT),
            (NODESET_REFERENCE_RECORD, NODESET_REFERENCE_CONTRACT),
        ] {
            let row = family
                .get(&record.to_string())
                .copied()
                .ok_or_else(|| format!("{LABEL} lost promoted control {record}"))?;
            if row.source != EXCLUSION_SOURCE
                || !matches!(&row.disposition,
                    XyceUpstreamExclusionDisposition::RspiceIndependentlyQualified { expected_contract }
                        if expected_contract == contract)
            {
                return Err(format!(
                    "{LABEL} control qualification changed for {record}"
                ));
            }
        }
        for owner in [OWNER_RECORD, WARNING_RECORD, NODESET_OWNER_RECORD] {
            if exclusions.contains_key(owner) {
                return Err(format!(
                    "{LABEL} wrapper owner {owner} must not be excluded"
                ));
            }
        }
        let members = self.validate_bug667_ic_directory()?;
        for (name, ..) in RETAINED_ARTIFACTS {
            self.reject_wrapper_output_artifacts(
                &self
                    .root
                    .join("Netlists/Certification_Tests/BUG_667_SON")
                    .join(name),
            )
            .map_err(|error| format!("{LABEL} {name} {error}"))?;
        }
        let output = self.root.join("OutputData/Certification_Tests/BUG_667_SON");
        match fs::symlink_metadata(&output) {
            Err(error) if error.kind() == std::io::ErrorKind::NotFound => {}
            Err(error) => return Err(format!("failed to inspect {LABEL} OutputData: {error}")),
            Ok(_) => return Err(format!("{LABEL} must not acquire invented numerical gold")),
        }
        Ok(members)
    }

    fn validate_bug667_ic_plan(
        role: Bug667IcRole,
        plan: &XyceStaticTranPlan,
    ) -> Result<(), String> {
        let expected_contract = match role {
            Bug667IcRole::ScopedOwner => XyceStaticTranContract::WrapperStatic,
            Bug667IcRole::ExplicitHierarchicalReference => XyceStaticTranContract::PlainStatic,
        };
        let expected_probes = [
            "V(N15206)",
            "V(N15971)",
            "V(N15554)",
            "V(N15997)",
            "V(N16554)",
            "V(N16997)",
        ];
        if plan.contract != expected_contract
            || plan.output_override
            || plan.timeint_conststep
            || !plan.steps.is_empty()
            || plan.wrapper_tolerance.is_some()
            || plan.comparison_mode != XyceStaticTranComparisonMode::Pointwise
            || !matches!(plan.oracle, XyceStaticTranOracle::None)
            || plan.require_print(LABEL)?.probes != expected_probes
            || plan.tran.step.to_bits() != 0.0f64.to_bits()
            || plan.tran.stop.to_bits() != 10e-3f64.to_bits()
            || plan.tran.start.is_some()
            || plan.tran.max_step.is_some()
            || plan.tran.uic
        {
            return Err(format!(
                "{LABEL} {} plan changed: {plan:?}",
                role.file_name()
            ));
        }
        Ok(())
    }

    fn validate_bug667_ic_netlist(role: Bug667IcRole, netlist: &Netlist) -> Result<(), String> {
        if netlist.title.trim() != "*Analysis directives:"
            || netlist.elements.len() != 14
            || netlist.subcircuits.len() != 1
            || netlist.output_requests.len() != 1
            || !netlist.models.is_empty()
            || !netlist.data_tables.is_empty()
            || netlist.device_initial_conditions.is_some()
            || !netlist.node_sets.is_empty()
            || !netlist.global_nodes.is_empty()
            || !netlist.measurements.is_empty()
            || !netlist.veriloga_includes.is_empty()
            || !netlist.spef_includes.is_empty()
            || !netlist.diagnostics.is_empty()
            || !matches!(netlist.analyses.as_slice(), [AnalysisCommand::Tran {
                step,
                stop,
                start: None,
                max_step: None,
                uic: false,
            }] if step.to_bits() == 0.0f64.to_bits() && stop.to_bits() == 10e-3f64.to_bits())
            || !netlist.params.all_params().is_empty()
            || !netlist.params.all_string_params().is_empty()
            || !netlist.params.all_functions().is_empty()
        {
            return Err(format!(
                "{LABEL} {} typed envelope changed",
                role.file_name()
            ));
        }

        let mut elements = BTreeMap::new();
        for element in &netlist.elements {
            let name = element.name.to_ascii_lowercase();
            if elements.insert(name.clone(), element).is_some() {
                return Err(format!("{LABEL} duplicate element {name}"));
            }
        }
        let expected_names = [
            "r_r1", "r_r2", "c_c1", "r_r3", "v_v1", "v_v2", "r_r4", "r_r5", "c_c2", "r_r6", "v_v3",
            "r_r7", "x_x1", "r_r8",
        ]
        .into_iter()
        .map(str::to_string)
        .collect::<BTreeSet<_>>();
        if elements.keys().cloned().collect::<BTreeSet<_>>() != expected_names {
            return Err(format!("{LABEL} top-level element inventory changed"));
        }
        for (name, nodes, resistance) in [
            ("r_r1", ["N15206", "N15971"], 1e3),
            ("r_r2", ["N15975", "N15971"], 10.0),
            ("r_r3", ["N16095", "0"], 10.0),
            ("r_r4", ["N15554", "N15997"], 1e3),
            ("r_r5", ["N15967", "N15997"], 10.0),
            ("r_r6", ["N16112", "0"], 10.0),
            ("r_r7", ["N16554", "N16997"], 1e3),
            ("r_r8", ["N17112", "0"], 10.0),
        ] {
            Self::validate_bug667_scalar_resistor(elements[name], nodes, resistance)?;
        }
        for (name, nodes) in [
            ("c_c1", ["N16095", "N15975"]),
            ("c_c2", ["N16112", "N15967"]),
        ] {
            Self::validate_bug667_scalar_capacitor(elements[name], nodes, 1e-6)?;
        }
        for (name, node) in [("v_v1", "N15206"), ("v_v2", "N15554"), ("v_v3", "N16554")] {
            let source = elements[name];
            if source.nodes != [node, "0"]
                || !matches!(source.kind,
                    ElementKind::VoltageSource(rspice_core::netlist::SourceSpec::Pulse {
                        v1, v2, delay, rise, fall, width, period, phase,
                        width_defaults_to_zero: false,
                    }) if v1.to_bits() == 0.0f64.to_bits()
                        && v2.to_bits() == 1.0f64.to_bits()
                        && delay.to_bits() == 0.0f64.to_bits()
                        && rise.to_bits() == 1e-3f64.to_bits()
                        && fall.to_bits() == 1e-3f64.to_bits()
                        && width.to_bits() == 5e-3f64.to_bits()
                        && period.to_bits() == 1.0f64.to_bits()
                        && phase.to_bits() == 0.0f64.to_bits())
            {
                return Err(format!("{LABEL} {name} topology or PULSE changed"));
            }
        }

        let instance = elements["x_x1"];
        let ElementKind::Subcircuit {
            subckt_name,
            params,
        } = &instance.kind
        else {
            return Err(format!("{LABEL} X_X1 is not a subcircuit instance"));
        };
        if instance.nodes != ["N16997", "N17112"] || !subckt_name.eq_ignore_ascii_case("IC_Subckt")
        {
            return Err(format!("{LABEL} X_X1 topology changed"));
        }
        match role {
            Bug667IcRole::ScopedOwner => match params.as_slice() {
                [(name, ParametricValue::Resolved(value))]
                    if name.eq_ignore_ascii_case("vmid") && value.to_bits() == 0.5f64.to_bits() => {
                }
                _ => return Err(format!("{LABEL} owner lost vmid=0.5 instance override")),
            },
            Bug667IcRole::ExplicitHierarchicalReference if params.is_empty() => {}
            Bug667IcRole::ExplicitHierarchicalReference => {
                return Err(format!(
                    "{LABEL} explicit reference gained instance parameters"
                ));
            }
        }

        let subcircuit = &netlist.subcircuits[0];
        if !subcircuit.name.eq_ignore_ascii_case("IC_Subckt")
            || subcircuit.ports.len() != 2
            || !subcircuit.ports[0].eq_ignore_ascii_case("in")
            || !subcircuit.ports[1].eq_ignore_ascii_case("out")
            || subcircuit.elements.len() != 2
            || !subcircuit.node_sets.is_empty()
            || !subcircuit.body_params.is_empty()
            || !subcircuit.expr_params.is_empty()
            || !subcircuit.string_params.is_empty()
            || !subcircuit.body_expr_params.is_empty()
            || !subcircuit.body_string_params.is_empty()
            || !subcircuit.body_functions.is_empty()
            || !subcircuit.local_options.is_empty()
            || subcircuit.library_ref.is_some()
            || !subcircuit.nested_subcircuits.is_empty()
        {
            return Err(format!("{LABEL} IC_Subckt shape changed: {subcircuit:?}"));
        }
        let sub_resistor = subcircuit
            .elements
            .iter()
            .find(|element| element.name.eq_ignore_ascii_case("R1"))
            .ok_or_else(|| format!("{LABEL} IC_Subckt lost R1"))?;
        Self::validate_bug667_scalar_resistor(sub_resistor, ["in", "mid"], 10.0)?;
        let sub_capacitor = subcircuit
            .elements
            .iter()
            .find(|element| element.name.eq_ignore_ascii_case("C1"))
            .ok_or_else(|| format!("{LABEL} IC_Subckt lost C1"))?;
        Self::validate_bug667_scalar_capacitor(sub_capacitor, ["mid", "out"], 1e-6)?;

        match role {
            Bug667IcRole::ScopedOwner => {
                if subcircuit.params.len() != 1
                    || !subcircuit.params[0].0.eq_ignore_ascii_case("vmid")
                    || subcircuit.params[0].1.to_bits() != 5.0f64.to_bits()
                    || netlist.initial_conditions.len() != 1
                    || subcircuit.initial_conditions.len() != 1
                {
                    return Err(format!("{LABEL} scoped IC parameter structure changed"));
                }
                let top = &netlist.initial_conditions[0];
                let local = &subcircuit.initial_conditions[0];
                if !top.node.eq_ignore_ascii_case("N15967")
                    || top.voltage.to_bits() != 0.5f64.to_bits()
                    || top.voltage_expr.is_some()
                    || !local.node.eq_ignore_ascii_case("mid")
                    || local.voltage.to_bits() != 5.0f64.to_bits()
                    || local
                        .voltage_expr
                        .as_deref()
                        .is_none_or(|expression| !expression.eq_ignore_ascii_case("vmid"))
                {
                    return Err(format!("{LABEL} scoped IC representation changed"));
                }
            }
            Bug667IcRole::ExplicitHierarchicalReference => {
                if !subcircuit.params.is_empty()
                    || !subcircuit.initial_conditions.is_empty()
                    || netlist.initial_conditions.len() != 2
                {
                    return Err(format!("{LABEL} explicit IC representation changed"));
                }
                let raw = netlist
                    .initial_conditions
                    .iter()
                    .map(|ic| {
                        (
                            ic.node.to_ascii_lowercase().replace(':', "."),
                            ic.voltage.to_bits(),
                            ic.voltage_expr.is_none(),
                        )
                    })
                    .collect::<BTreeSet<_>>();
                if raw
                    != BTreeSet::from([
                        ("n15967".to_string(), 0.5f64.to_bits(), true),
                        ("x_x1.mid".to_string(), 0.5f64.to_bits(), true),
                    ])
                {
                    return Err(format!("{LABEL} explicit top-level ICs changed: {raw:?}"));
                }
            }
        }

        let request = &netlist.output_requests[0];
        let expected_dependencies = ["N15206", "N15971", "N15554", "N15997", "N16554", "N16997"];
        if request.directive != OutputDirectiveKind::Print
            || request.analysis != Some(rspice_core::netlist::OutputAnalysisKind::Tran)
            || request.name.is_some()
            || request.print_delimiter != Some(PrintDelimiter::Whitespace)
            || !request.expressions.is_empty()
            || request.origin.line != 3
            || request.dependencies.len() != expected_dependencies.len()
            || request
                .dependencies
                .iter()
                .zip(expected_dependencies)
                .any(|(actual, symbol)| {
                    actual.expression
                        || actual.kind != OutputSymbolKind::Node
                        || !actual.operator.eq_ignore_ascii_case("V")
                        || !actual.symbol.eq_ignore_ascii_case(symbol)
                })
        {
            return Err(format!("{LABEL} typed .PRINT request changed: {request:?}"));
        }
        Ok(())
    }

    fn bug667_effective_ic_map(netlist: &Netlist) -> Result<BTreeMap<String, u64>, String> {
        let flattened = flatten_netlist_with_models(netlist)
            .map_err(|error| format!("{LABEL} hierarchy flattening failed: {error}"))?;
        let mut conditions = BTreeMap::new();
        for ic in netlist
            .initial_conditions
            .iter()
            .chain(flattened.scoped_initial_conditions.iter())
        {
            if !ic.voltage.is_finite() || ic.voltage_expr.is_some() {
                return Err(format!("{LABEL} effective IC '{}' is unresolved", ic.node));
            }
            let node = if ic.node.contains([':', '.']) {
                Engine::resolve_hierarchical_node_name(netlist, &ic.node).ok_or_else(|| {
                    format!(
                        "{LABEL} hierarchical IC target '{}' does not resolve",
                        ic.node
                    )
                })?
            } else {
                ic.node.clone()
            }
            .replace(':', ".")
            .to_ascii_lowercase();
            if conditions
                .insert(node.clone(), ic.voltage.to_bits())
                .is_some()
            {
                return Err(format!(
                    "{LABEL} contains duplicate effective IC node {node}"
                ));
            }
        }
        Ok(conditions)
    }

    fn validate_bug667_ic_table(table: &XycePrnTable, role: Bug667IcRole) -> Result<(), String> {
        const COLUMNS: [&str; 8] = [
            "Index",
            "TIME",
            "V(N15206)",
            "V(N15971)",
            "V(N15554)",
            "V(N15997)",
            "V(N16554)",
            "V(N16997)",
        ];
        if table.columns != COLUMNS || table.rows.len() < 2 {
            return Err(format!("{LABEL} {} table shape changed", role.file_name()));
        }
        let first = &table.rows[0];
        if first.len() != COLUMNS.len() || first.iter().any(|value| !value.is_finite()) {
            return Err(format!(
                "{LABEL} {} first row is malformed",
                role.file_name()
            ));
        }
        let expected_ic_output = 0.5 * 1_000.0 / 1_010.0;
        for (column, expected, name) in [
            (0, 0.0, "Index"),
            (1, 0.0, "TIME"),
            (2, 0.0, "baseline source"),
            (3, 0.0, "no-IC output"),
            (4, 0.0, "direct-IC source"),
            (5, expected_ic_output, "direct-IC output"),
            (6, 0.0, "hierarchical-IC source"),
            (7, expected_ic_output, "hierarchical-IC output"),
        ] {
            let actual = Self::xyce_default_prn_roundtrip(first[column])?;
            let expected = Self::xyce_default_prn_roundtrip(expected)?;
            if actual != expected {
                return Err(format!(
                    "{LABEL} {} {name} startup value changed: {actual} != {expected}",
                    role.file_name()
                ));
            }
        }
        if Self::xyce_default_prn_roundtrip(first[5])?
            == Self::xyce_default_prn_roundtrip(first[3])?
            || Self::xyce_default_prn_roundtrip(first[7])?
                == Self::xyce_default_prn_roundtrip(first[3])?
        {
            return Err(format!("{LABEL} IC causality became vacuous"));
        }
        let mut previous_time = f64::NEG_INFINITY;
        for (index, row) in table.rows.iter().enumerate() {
            if row.len() != COLUMNS.len()
                || row.iter().any(|value| !value.is_finite())
                || row[0].to_bits() != (index as Value).to_bits()
                || row[1] < previous_time
            {
                return Err(format!(
                    "{LABEL} {} row {index} changed: {row:?}",
                    role.file_name()
                ));
            }
            previous_time = row[1];
        }
        if Self::xyce_default_prn_roundtrip(previous_time)?
            != Self::xyce_default_prn_roundtrip(10e-3)?
        {
            return Err(format!("{LABEL} {} did not reach 10ms", role.file_name()));
        }

        // The wrapper's exact owner/reference diff alone would also pass if
        // both IC paths lost their capacitor history after startup. The
        // authored three-branch network supplies its own no-IC control in
        // column 3. For every positive-time row, both IC-output deltas must
        // follow the first-order discharge through 1020 ohms and 1 uF.
        let tau = 1_020.0e-6;
        let initial_delta = 0.5 * 1_000.0 / 1_020.0;
        let mut observed_near_tau = false;
        for (index, row) in table.rows.iter().enumerate().skip(1) {
            let time = Self::xyce_default_prn_roundtrip(row[1])?;
            if time <= 0.0 {
                continue;
            }
            let direct = Self::xyce_default_prn_roundtrip(row[5] - row[3])?;
            let scoped = Self::xyce_default_prn_roundtrip(row[7] - row[3])?;
            if direct != scoped {
                return Err(format!(
                    "{LABEL} {} direct/scoped IC histories diverged at row {index}: {direct} != {scoped}",
                    role.file_name()
                ));
            }
            let expected = Self::xyce_default_prn_roundtrip(initial_delta * (-time / tau).exp())?;
            let tolerance = expected.abs() * 1.5e-2 + 2.0e-6;
            if (direct - expected).abs() > tolerance {
                return Err(format!(
                    "{LABEL} {} IC decay changed at row {index}, t={time}: actual={direct}, expected={expected}, tolerance={tolerance}",
                    role.file_name()
                ));
            }
            if (0.5 * tau..=1.5 * tau).contains(&time) {
                observed_near_tau = true;
            }
        }
        if !observed_near_tau {
            return Err(format!(
                "{LABEL} {} did not retain a positive-time sample near the IC decay constant",
                role.file_name()
            ));
        }
        Ok(())
    }

    fn validate_bug667_no_ic_counterfactual(
        full: &XycePrnTable,
        no_ic: &XycePrnTable,
    ) -> Result<(), String> {
        if no_ic.columns != full.columns || no_ic.rows.len() < 2 {
            return Err(format!("{LABEL} no-IC counterfactual table is malformed"));
        }
        let first = &no_ic.rows[0];
        if first.len() != full.columns.len()
            || first.iter().any(|value| !value.is_finite())
            || Self::xyce_default_prn_roundtrip(first[0])? != 0.0
            || Self::xyce_default_prn_roundtrip(first[1])? != 0.0
        {
            return Err(format!(
                "{LABEL} no-IC counterfactual startup row is malformed"
            ));
        }
        let baseline = Self::xyce_default_prn_roundtrip(first[3])?;
        if Self::xyce_default_prn_roundtrip(first[5])? != baseline
            || Self::xyce_default_prn_roundtrip(first[7])? != baseline
        {
            return Err(format!(
                "{LABEL} no-IC counterfactual retained an unexplained startup response"
            ));
        }
        let full_first = &full.rows[0];
        if Self::xyce_default_prn_roundtrip(full_first[5])? == baseline
            || Self::xyce_default_prn_roundtrip(full_first[7])? == baseline
        {
            return Err(format!(
                "{LABEL} removing all ICs did not causally change both startup branches"
            ));
        }
        let final_time = no_ic
            .rows
            .last()
            .and_then(|row| row.get(1))
            .copied()
            .ok_or_else(|| format!("{LABEL} no-IC counterfactual lost its final time"))?;
        if Self::xyce_default_prn_roundtrip(final_time)? != Self::xyce_default_prn_roundtrip(10e-3)?
        {
            return Err(format!(
                "{LABEL} no-IC counterfactual did not reach the complete domain"
            ));
        }
        Ok(())
    }

    pub(super) fn validate_bug667_ic_oracle(
        &self,
        deck: &XyceDeck,
        role: Bug667IcRole,
        start: Instant,
    ) -> Result<(), String> {
        let abort = DeadlineAbort::new(start, self.config.max_time_per_test_ms.max(1));
        if abort.is_aborted() {
            return Err(format!(
                "{LABEL} exceeded its shared deadline before provenance"
            ));
        }
        let members = self.validate_bug667_ic_provenance(deck, role)?;
        let build = |member_role: Bug667IcRole| {
            let purpose = match member_role {
                Bug667IcRole::ScopedOwner => {
                    XyceStaticTranPlanPurpose::GeneratedReferenceRelationalFamily
                }
                Bug667IcRole::ExplicitHierarchicalReference => {
                    XyceStaticTranPlanPurpose::RelationalFamily
                }
            };
            let path = self.root.join(member_role.path());
            let plan = self.static_tran_plan_for_path_with_purpose(&path, purpose)?;
            let source = members
                .get(&member_role.file_name().to_ascii_lowercase())
                .ok_or_else(|| format!("{LABEL} lost {}", member_role.file_name()))?;
            if plan.source.as_bytes() != source.as_slice() {
                return Err(format!(
                    "{LABEL} {} changed between reads",
                    member_role.file_name()
                ));
            }
            Self::validate_bug667_ic_plan(member_role, &plan)?;
            let netlist = Self::parse_xyce_netlist(&plan.source, &path).map_err(|error| {
                format!("{LABEL} {} parse failed: {error}", member_role.file_name())
            })?;
            Self::validate_bug667_ic_netlist(member_role, &netlist)?;
            let effective = Self::bug667_effective_ic_map(&netlist)?;
            Ok::<_, String>((plan, netlist, effective))
        };
        let (owner_plan, owner_netlist, owner_ic) = build(Bug667IcRole::ScopedOwner)?;
        let (reference_plan, reference_netlist, reference_ic) =
            build(Bug667IcRole::ExplicitHierarchicalReference)?;
        let expected_ic = BTreeMap::from([
            ("n15967".to_string(), 0.5f64.to_bits()),
            ("x_x1.mid".to_string(), 0.5f64.to_bits()),
        ]);
        if owner_ic != expected_ic || reference_ic != expected_ic || owner_ic != reference_ic {
            return Err(format!(
                "{LABEL} effective IC maps changed: owner={owner_ic:?}, reference={reference_ic:?}"
            ));
        }

        let run = |member_role: Bug667IcRole, plan: &XyceStaticTranPlan, netlist: &Netlist| {
            let result = self
                .run_transient_family_netlist(plan, netlist, start, None, None)
                .map_err(|error| {
                    format!(
                        "{LABEL} {} execution failed: {error}",
                        member_role.file_name()
                    )
                })?;
            let table = Self::transient_family_result_to_prn_table(plan, netlist, &result)
                .map_err(|error| {
                    format!(
                        "{LABEL} {} PRN generation failed: {error}",
                        member_role.file_name()
                    )
                })?;
            Self::validate_bug667_ic_table(&table, member_role)?;
            Ok::<_, String>(table)
        };
        let owner_table = run(Bug667IcRole::ScopedOwner, &owner_plan, &owner_netlist)?;
        if abort.is_aborted() {
            return Err(format!(
                "{LABEL} exceeded its shared deadline after owner execution"
            ));
        }
        let reference_table = run(
            Bug667IcRole::ExplicitHierarchicalReference,
            &reference_plan,
            &reference_netlist,
        )?;
        let mismatches =
            self.compare_serialized_default_prn_tables(&owner_table, &reference_table)?;
        if !mismatches.is_empty() {
            return Err(format!(
                "{LABEL} raw default-PRN diff found {} mismatch(es): {mismatches:?}",
                mismatches.len()
            ));
        }
        if abort.is_aborted() {
            return Err(format!(
                "{LABEL} exceeded its shared deadline before the no-IC counterfactual"
            ));
        }
        let mut no_ic_netlist = owner_netlist.clone();
        no_ic_netlist.retain_startup_kinds(false, true);
        if !Self::bug667_effective_ic_map(&no_ic_netlist)?.is_empty() {
            return Err(format!(
                "{LABEL} failed to remove all direct and scoped ICs from its counterfactual"
            ));
        }
        let no_ic_result = self
            .run_transient_family_netlist(&owner_plan, &no_ic_netlist, start, None, None)
            .map_err(|error| format!("{LABEL} no-IC counterfactual execution failed: {error}"))?;
        let no_ic_table =
            Self::transient_family_result_to_prn_table(&owner_plan, &no_ic_netlist, &no_ic_result)
                .map_err(|error| {
                    format!("{LABEL} no-IC counterfactual PRN generation failed: {error}")
                })?;
        Self::validate_bug667_no_ic_counterfactual(&owner_table, &no_ic_table)?;
        self.validate_bug667_ic_provenance(deck, role)?;
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

    fn canonical_deck(root: &Path, role: Bug667IcRole) -> XyceDeck {
        XyceDeck {
            path: root.join(role.path()),
            section: XyceDeckSection::Netlists,
            relative_path: role.path().to_string(),
        }
    }

    fn fixture(label: &str) -> (tempfile::TempDir, XyceDeck, XyceTestRunner) {
        let temporary = tempfile::Builder::new()
            .prefix(&format!("rspice-xyce-bug667-ic-{label}-"))
            .tempdir()
            .expect("create BUG667 IC fixture");
        let root = temporary.path();
        let family = root.join("Netlists/Certification_Tests/BUG_667_SON");
        fs::create_dir_all(&family).expect("create BUG667 IC family");
        let canonical = corpus_root().join("Netlists/Certification_Tests/BUG_667_SON");
        for (name, ..) in RETAINED_ARTIFACTS {
            fs::copy(canonical.join(name), family.join(name)).expect("copy BUG667 member");
        }
        fs::write(
            root.join(HARNESS_MANIFEST_FILE),
            format!(
                "{OWNER_PATH}\t{REQUIRES_UPSTREAM_WRAPPER_CONTRACT}\n\
                 Netlists/Certification_Tests/BUG_667_SON/ic_in_subckt_warning.cir\t{REQUIRES_UPSTREAM_WRAPPER_CONTRACT}\n\
                 Netlists/Certification_Tests/BUG_667_SON/nodeset_in_subckt.cir\t{REQUIRES_UPSTREAM_WRAPPER_CONTRACT}\n"
            ),
        )
        .expect("write BUG667 wrapper manifest");
        fs::write(
            root.join(UPSTREAM_EXCLUSIONS_MANIFEST_FILE),
            format!(
                "schema_version\t{UPSTREAM_EXCLUSIONS_SCHEMA_VERSION}\n\
                 source_commit\t{UPSTREAM_EXCLUSIONS_SOURCE_COMMIT}\n\
                 source_netlists_tree\t{UPSTREAM_EXCLUSIONS_SOURCE_NETLISTS_TREE}\n\
                 {REFERENCE_PATH}\t{EXCLUSION_SOURCE}\t{RSPICE_INDEPENDENTLY_QUALIFIED_DISPOSITION}\t{REFERENCE_CONTRACT}\n\
                 Netlists/Certification_Tests/BUG_667_SON/nodeset_not_in_subckt.cir\t{EXCLUSION_SOURCE}\t{RSPICE_INDEPENDENTLY_QUALIFIED_DISPOSITION}\t{NODESET_REFERENCE_CONTRACT}\n"
            ),
        )
        .expect("write BUG667 exclusion manifest");
        let runner = XyceTestRunner::new(root, XyceRunnerConfig::default());
        let deck = canonical_deck(root, Bug667IcRole::ScopedOwner);
        (temporary, deck, runner)
    }

    #[test]
    fn bug667_ic_historical_provenance_is_exact() {
        XyceTestRunner::validate_bug667_ic_historical_provenance()
            .expect("BUG667 IC Release provenance remains exact");
    }

    #[test]
    fn bug667_ic_roles_execute_with_nonvacuous_startup_causality() {
        let root = corpus_root();
        let runner = XyceTestRunner::new(
            &root,
            XyceRunnerConfig {
                max_time_per_test_ms: 30_000,
                ..XyceRunnerConfig::default()
            },
        );
        for role in Bug667IcRole::ALL {
            runner
                .validate_bug667_ic_oracle(&canonical_deck(&root, role), role, Instant::now())
                .unwrap_or_else(|error| panic!("{} failed: {error}", role.file_name()));
        }
    }

    #[test]
    fn bug667_ic_startup_counterfactuals_fail_closed() {
        let expected = 0.5 * 1_000.0 / 1_010.0;
        let tau = 1_020.0e-6;
        let initial = 0.5 * 1_000.0 / 1_020.0;
        let baseline = XycePrnTable {
            columns: vec![
                "Index".into(),
                "TIME".into(),
                "V(N15206)".into(),
                "V(N15971)".into(),
                "V(N15554)".into(),
                "V(N15997)".into(),
                "V(N16554)".into(),
                "V(N16997)".into(),
            ],
            rows: vec![
                vec![0.0, 0.0, 0.0, 0.0, 0.0, expected, 0.0, expected],
                vec![
                    1.0,
                    tau,
                    1.0,
                    0.0,
                    1.0,
                    initial * (-1.0f64).exp(),
                    1.0,
                    initial * (-1.0f64).exp(),
                ],
                vec![
                    2.0,
                    10e-3,
                    1.0,
                    0.0,
                    1.0,
                    initial * (-10e-3 / tau).exp(),
                    1.0,
                    initial * (-10e-3 / tau).exp(),
                ],
            ],
        };
        XyceTestRunner::validate_bug667_ic_table(&baseline, Bug667IcRole::ScopedOwner)
            .expect("canonical causal table passes");
        for column in [5, 7] {
            let mut changed = baseline.clone();
            changed.rows[0][column] = 0.0;
            assert!(
                XyceTestRunner::validate_bug667_ic_table(&changed, Bug667IcRole::ScopedOwner)
                    .is_err(),
                "ignored IC column {column} must fail"
            );
        }
        let mut history_lost = baseline.clone();
        history_lost.rows[1][5] = 0.0;
        history_lost.rows[1][7] = 0.0;
        assert!(
            XyceTestRunner::validate_bug667_ic_table(&history_lost, Bug667IcRole::ScopedOwner)
                .is_err(),
            "shared loss of positive-time IC history must fail"
        );
        let mut malformed = baseline.clone();
        malformed.columns[7] = "V(WRONG)".into();
        assert!(
            XyceTestRunner::validate_bug667_ic_table(&malformed, Bug667IcRole::ScopedOwner)
                .is_err()
        );
        let mut malformed = baseline.clone();
        malformed.rows[1][0] = 7.0;
        assert!(
            XyceTestRunner::validate_bug667_ic_table(&malformed, Bug667IcRole::ScopedOwner)
                .is_err()
        );
        let mut malformed = baseline.clone();
        malformed.rows[1][5] = Value::NAN;
        assert!(
            XyceTestRunner::validate_bug667_ic_table(&malformed, Bug667IcRole::ScopedOwner)
                .is_err()
        );
        let mut malformed = baseline;
        malformed.rows.pop();
        assert!(
            XyceTestRunner::validate_bug667_ic_table(&malformed, Bug667IcRole::ScopedOwner)
                .is_err()
        );
    }

    #[test]
    fn bug667_ic_typed_contract_rejects_startup_and_topology_mutations() {
        for role in Bug667IcRole::ALL {
            let path = corpus_root().join(role.path());
            let canonical = fs::read_to_string(&path).expect("read canonical BUG667 IC deck");
            let mutations = match role {
                Bug667IcRole::ScopedOwner => vec![
                    canonical.replace("vmid=0.5", "vmid=0.6"),
                    canonical.replace("V(N15967) =0.5", "V(N15967) =0.4"),
                    canonical.replace("V(mid)={vmid}", "V(mid)={vmid+1}"),
                    canonical.replace(".IC         V(mid)", ".NODESET    V(mid)"),
                    canonical.replace("R1          in  mid 10", "R1          in  mid 11"),
                    canonical.replace(".TRAN  0 10ms", ".TRAN  0 9ms"),
                ],
                Bug667IcRole::ExplicitHierarchicalReference => vec![
                    canonical.replace("X_X1:mid", "X_X1:missing"),
                    canonical.replace("V(X_X1:mid )=0.5", "V(X_X1:mid )=0.4"),
                    canonical.replace(".IC         V(X_X1:mid", ".NODESET    V(X_X1:mid"),
                    canonical.replace("R1          in  mid 10", "R1          in  mid 11"),
                    canonical.replace(".TRAN  0 10ms", ".TRAN  0 9ms"),
                ],
            };
            for mutation in mutations {
                let rejected = match XyceTestRunner::parse_xyce_netlist(&mutation, &path) {
                    Ok(netlist) => {
                        XyceTestRunner::validate_bug667_ic_netlist(role, &netlist).is_err()
                    }
                    Err(_) => true,
                };
                assert!(
                    rejected,
                    "{} mutation escaped the typed contract",
                    role.file_name()
                );
            }
        }
    }

    #[test]
    fn bug667_ic_provenance_rejects_source_role_and_census_drift() {
        let (_temporary, deck, runner) = fixture("canonical");
        runner
            .validate_bug667_ic_provenance(&deck, Bug667IcRole::ScopedOwner)
            .expect("canonical BUG667 IC fixture");

        let owner = runner.root.join(OWNER_PATH);
        fs::write(&owner, vec![b'x'; 2_000]).expect("replace owner with oversized member");
        assert!(
            runner
                .validate_bug667_ic_provenance(&deck, Bug667IcRole::ScopedOwner)
                .is_err(),
            "oversized retained member must fail before an unbounded read"
        );

        let (_temporary, deck, runner) = fixture("extra-member");
        fs::write(
            runner
                .root
                .join("Netlists/Certification_Tests/BUG_667_SON/extra.cir"),
            "* unexpected\n",
        )
        .expect("write unexpected family member");
        assert!(
            runner
                .validate_bug667_ic_provenance(&deck, Bug667IcRole::ScopedOwner)
                .is_err()
        );

        let (_temporary, deck, runner) = fixture("owner-role");
        fs::write(runner.root.join(HARNESS_MANIFEST_FILE), "")
            .expect("remove wrapper ownership after runner construction");
        assert!(
            runner
                .validate_bug667_ic_provenance(&deck, Bug667IcRole::ScopedOwner)
                .is_err(),
            "live wrapper ownership drift must fail"
        );

        let (_temporary, deck, runner) = fixture("exclusion-role");
        let exclusions = runner.root.join(UPSTREAM_EXCLUSIONS_MANIFEST_FILE);
        let text = fs::read_to_string(&exclusions).expect("read exclusions");
        fs::write(
            &exclusions,
            text.replace(REFERENCE_CONTRACT, NODESET_REFERENCE_CONTRACT),
        )
        .expect("mutate reference qualification contract");
        assert!(
            runner
                .validate_bug667_ic_provenance(&deck, Bug667IcRole::ScopedOwner)
                .is_err()
        );

        let (_temporary, deck, runner) = fixture("invented-gold");
        fs::create_dir_all(
            runner
                .root
                .join("OutputData/Certification_Tests/BUG_667_SON"),
        )
        .expect("invent BUG667 output family");
        assert!(
            runner
                .validate_bug667_ic_provenance(&deck, Bug667IcRole::ScopedOwner)
                .is_err()
        );
    }

    #[test]
    fn bug667_ic_expired_deadline_fails_closed() {
        let root = corpus_root();
        let runner = XyceTestRunner::new(
            &root,
            XyceRunnerConfig {
                max_time_per_test_ms: 1,
                ..XyceRunnerConfig::default()
            },
        );
        assert!(
            runner
                .validate_bug667_ic_oracle(
                    &canonical_deck(&root, Bug667IcRole::ScopedOwner),
                    Bug667IcRole::ScopedOwner,
                    Instant::now() - Duration::from_millis(2),
                )
                .is_err()
        );
    }
}
