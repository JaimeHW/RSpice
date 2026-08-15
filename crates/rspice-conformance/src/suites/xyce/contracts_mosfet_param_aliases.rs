use super::*;
use rspice_core::netlist::{ElementKind, ElementProvenance, SourceSpec};
use std::io::Read as _;

const LABEL: &str = "MOSFET_ParamAliases classic-MOS parameter alias relation";
const FAMILY_PATH: &str = "Netlists/MOSFET_ParamAliases";
const OUTPUT_PATH: &str = "OutputData/MOSFET_ParamAliases";
const EXCLUSION_SOURCE: &str = "Netlists/MOSFET_ParamAliases/exclude";
const UPSTREAM_COMMIT: &str = "d6e278e371ec2f3df1325dcff4552e585bc7ecc1";
const UPSTREAM_TAG: &str = "Release-7.10.0";

const HISTORICAL_RECORD_COUNT: usize = 21;
const HISTORICAL_RECORD_BYTES: usize = 4_853;
const HISTORICAL_RECORDS_SHA256: &str =
    "82f80e233d0eac2218f4b3284f9d1d2dc2e5af36e07cf6b2ae2e08c599f9159b";
const HISTORICAL_RECORDS_BLAKE3: &str =
    "24a8ef83d9ac4ca7c51f111dd83915728553e4c75e72ae9aed44bdad5ba577b8";
const HISTORICAL_ARTIFACTS: [(&str, usize, &str, &str); 21] = [
    (
        "Netlists/MOSFET_ParamAliases/CMakeLists.txt",
        5_817,
        "8a9ecdc6881ecd0ce61fa1fbfe4ad11e103bed19ea7b48271b6ad2d23aba11ee",
        "40f517c57bdfaf6eda8e6afcc20ef7a2bd4cbd4d6adb10373239c8b6ac8ae453",
    ),
    (
        "Netlists/MOSFET_ParamAliases/Manifest.txt",
        172,
        "c3cf1f24c0a5a9877def18a997e0040812a85b357279d3ce591d24c82a49d4af",
        "93af7c5239575a4a62822f7a4c43e75af97f43c079ff29da316bda52773eeab5",
    ),
    (
        EXCLUSION_SOURCE,
        64,
        "9416efcabd1231a0746274534df3abdeeddb0f9e0d429cf89be36056792dec4f",
        "3e788f4982d5f9fe219a2ebf40a279a9db9fa9235c4852fa8124b6eb7ee481fb",
    ),
    (
        "Netlists/MOSFET_ParamAliases/invert1.cir",
        1_165,
        "a25f498f542c6b3e62d5ca10ee4f6aadc6ad05e05ac5b49123c41e54f928abd2",
        "823d03916c79c25d09cd081ca0d7fae114360c8028cd6d04fd659b644aba8d58",
    ),
    (
        "Netlists/MOSFET_ParamAliases/invert1.cir.sh",
        1_422,
        "41a731ab0e30aa7d3cce757c58f168c9253a132645f5462fb42fe59c6dec27bb",
        "9b60d9fe60848803cb1ad4f6ffdecea7f71b1a9d8f6f821e214da07eccfb86b9",
    ),
    (
        "Netlists/MOSFET_ParamAliases/invert1_alt.cir",
        1_274,
        "059f73d7dc3bcc5b5b05698a50d9356c4dcd39eec05b0f2996906111a9d496bb",
        "b8d9628805b728969af37c478ec2f05b2e6b9f56d280880ba677705235119a0f",
    ),
    (
        "Netlists/MOSFET_ParamAliases/invert2.cir",
        1_165,
        "55def2f3c379d38a88a52c542d396f23750bf0c5acc0873f67a6c093b3f8d081",
        "c7041b83136a0614f5e69ad6e6d1579f1205c9bb9ac8fa47726a78053c801fdf",
    ),
    (
        "Netlists/MOSFET_ParamAliases/invert2.cir.sh",
        1_422,
        "353ff5c743055dd555bf80bb08e0906565f7afe484129a9da0fd517de251860c",
        "391935e1dac58247321fe0ad083fbe90b4945d37a028321f62afd1ad8d5b3259",
    ),
    (
        "Netlists/MOSFET_ParamAliases/invert2_alt.cir",
        1_218,
        "a21e82a9a4af97cca4862c84b3fd5eeb9a22a594bc4af871c981eced55f52fac",
        "d31f9fd316f15e2296f0ca18371532ab2269ddf8e9f47ad991d3af49ac091af8",
    ),
    (
        "Netlists/MOSFET_ParamAliases/invert3.cir",
        1_369,
        "ec9a86626e18cf53dab5cb25f003715d4e74b37a0307347cbf290865b14f5dbc",
        "fd490462f347e328e4e6b3821aaba3573513645dd62d8531492877fd7fe0d456",
    ),
    (
        "Netlists/MOSFET_ParamAliases/invert3.cir.sh",
        1_422,
        "ecca593bf063c4b23128985c81dac0b03da03837996456d3c4227b7bb231884b",
        "3b200ed17c8ec7f579ae08e63a070376bf3a4f2218f26316d94c680421c885e1",
    ),
    (
        "Netlists/MOSFET_ParamAliases/invert3_alt.cir",
        1_429,
        "937b489015b214372885d42158765a70c5b7da65850224d644e67fbed136f591",
        "74890d8ce9db7cea3e1bbe0caa5e6743eda0bcee85f1474f72522c32bb527595",
    ),
    (
        "Netlists/MOSFET_ParamAliases/invert6.cir",
        1_165,
        "bacd253a3cb268ff0191a07a0647ec160f396fe4ab8b187cfc1b9ab1d3829d35",
        "527c071ac3f1d888982d29abdc9cd3bf3e5642a58e7408ad18c1998e7116eef7",
    ),
    (
        "Netlists/MOSFET_ParamAliases/invert6.cir.sh",
        1_422,
        "bdf1e3fe4ad72d30c35da6ce9d4c55ce7b62f5c3db802af2f2f079a7b6754c5d",
        "82dca4a48f1f212ee536c435129cedcdffb90ba764786ce16ef502ec2f5cf147",
    ),
    (
        "Netlists/MOSFET_ParamAliases/invert6_alt.cir",
        1_165,
        "8edd8d8c6450e7cff7385d126c38b3f23809f47f186e8d3873a44d4b7c753008",
        "10ecb0fbe5ad35e55e2163465b2dea073193ae17a563393043fdf0859e4fd7a5",
    ),
    (
        "Netlists/MOSFET_ParamAliases/tags",
        45,
        "6a991e0f1896a6a71e99e77944de6174cb4f0726a50a8ce8ce42d1a21d4347e3",
        "460ee68ad6c0504ea51e45a6d125f42bf33e3f2729accbb340bc1e50778edf72",
    ),
    (
        "TestScripts/XyceRegression/Tools.pm",
        68_108,
        "5b5f86c02d46a1f3bdad5292e7e91d25a9e08e71490643d8d5ed7ae20f9d55e3",
        "13bd274632744ddc4b8baee680ddc9770902793ed7ee892ecdedd4dcb3828667",
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

const RETAINED_RECORD_COUNT: usize = 8;
const RETAINED_RECORD_BYTES: usize = 1_191;
const RETAINED_RECORDS_SHA256: &str =
    "535944824832a3cdd65ce6baef26d5aa803f9635014dffca71850211a7334344";
const RETAINED_RECORDS_BLAKE3: &str =
    "1517edbffb70e9b06b566147495ec822a567c86e17cdf63deba40ddc9f9cc0c9";
const RETAINED_ARTIFACTS: [(&str, usize, &str, &str); 8] = [
    (
        "invert1.cir",
        1_165,
        "a25f498f542c6b3e62d5ca10ee4f6aadc6ad05e05ac5b49123c41e54f928abd2",
        "823d03916c79c25d09cd081ca0d7fae114360c8028cd6d04fd659b644aba8d58",
    ),
    (
        "invert1_alt.cir",
        1_274,
        "059f73d7dc3bcc5b5b05698a50d9356c4dcd39eec05b0f2996906111a9d496bb",
        "b8d9628805b728969af37c478ec2f05b2e6b9f56d280880ba677705235119a0f",
    ),
    (
        "invert2.cir",
        1_165,
        "55def2f3c379d38a88a52c542d396f23750bf0c5acc0873f67a6c093b3f8d081",
        "c7041b83136a0614f5e69ad6e6d1579f1205c9bb9ac8fa47726a78053c801fdf",
    ),
    (
        "invert2_alt.cir",
        1_218,
        "a21e82a9a4af97cca4862c84b3fd5eeb9a22a594bc4af871c981eced55f52fac",
        "d31f9fd316f15e2296f0ca18371532ab2269ddf8e9f47ad991d3af49ac091af8",
    ),
    (
        "invert3.cir",
        1_369,
        "ec9a86626e18cf53dab5cb25f003715d4e74b37a0307347cbf290865b14f5dbc",
        "fd490462f347e328e4e6b3821aaba3573513645dd62d8531492877fd7fe0d456",
    ),
    (
        "invert3_alt.cir",
        1_429,
        "937b489015b214372885d42158765a70c5b7da65850224d644e67fbed136f591",
        "74890d8ce9db7cea3e1bbe0caa5e6743eda0bcee85f1474f72522c32bb527595",
    ),
    (
        "invert6.cir",
        1_165,
        "bacd253a3cb268ff0191a07a0647ec160f396fe4ab8b187cfc1b9ab1d3829d35",
        "527c071ac3f1d888982d29abdc9cd3bf3e5642a58e7408ad18c1998e7116eef7",
    ),
    (
        "invert6_alt.cir",
        1_165,
        "8edd8d8c6450e7cff7385d126c38b3f23809f47f186e8d3873a44d4b7c753008",
        "10ecb0fbe5ad35e55e2163465b2dea073193ae17a563393043fdf0859e4fd7a5",
    ),
];

type CapturedMembers = BTreeMap<String, Vec<u8>>;
type NormalizedModels = Vec<(String, String, Vec<(String, u64)>)>;

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub(super) enum MosfetParamAliasRole {
    Level1Owner,
    Level1Alt,
    Level2Owner,
    Level2Alt,
    Level3Owner,
    Level3Alt,
    Level6Owner,
    Level6Alt,
}

impl MosfetParamAliasRole {
    const ALL: [Self; 8] = [
        Self::Level1Owner,
        Self::Level1Alt,
        Self::Level2Owner,
        Self::Level2Alt,
        Self::Level3Owner,
        Self::Level3Alt,
        Self::Level6Owner,
        Self::Level6Alt,
    ];
    const OWNERS: [Self; 4] = [
        Self::Level1Owner,
        Self::Level2Owner,
        Self::Level3Owner,
        Self::Level6Owner,
    ];
    const ALTS: [Self; 4] = [
        Self::Level1Alt,
        Self::Level2Alt,
        Self::Level3Alt,
        Self::Level6Alt,
    ];

    pub(super) fn for_record(record: &str) -> Option<Self> {
        let normalized = XyceTestRunner::normalize_manifest_key(record);
        Self::ALL
            .into_iter()
            .find(|role| role.record() == normalized)
    }

    fn level(self) -> u8 {
        match self {
            Self::Level1Owner | Self::Level1Alt => 1,
            Self::Level2Owner | Self::Level2Alt => 2,
            Self::Level3Owner | Self::Level3Alt => 3,
            Self::Level6Owner | Self::Level6Alt => 6,
        }
    }

    fn is_owner(self) -> bool {
        matches!(
            self,
            Self::Level1Owner | Self::Level2Owner | Self::Level3Owner | Self::Level6Owner
        )
    }

    fn owner(self) -> Self {
        match self.level() {
            1 => Self::Level1Owner,
            2 => Self::Level2Owner,
            3 => Self::Level3Owner,
            6 => Self::Level6Owner,
            _ => unreachable!(),
        }
    }
    fn alt(self) -> Self {
        match self.level() {
            1 => Self::Level1Alt,
            2 => Self::Level2Alt,
            3 => Self::Level3Alt,
            6 => Self::Level6Alt,
            _ => unreachable!(),
        }
    }

    fn file_name(self) -> String {
        if self.is_owner() {
            format!("invert{}.cir", self.level())
        } else {
            format!("invert{}_alt.cir", self.level())
        }
    }
    fn path(self) -> String {
        format!("{FAMILY_PATH}/{}", self.file_name())
    }
    fn record(self) -> String {
        XyceTestRunner::normalize_manifest_key(&self.path())
    }

    pub(super) fn contract(self) -> &'static str {
        match self {
            Self::Level1Owner => "mosfet_param_alias_level1_wrapper_owner",
            Self::Level1Alt => "mosfet_param_alias_level1_alt_worker",
            Self::Level2Owner => "mosfet_param_alias_level2_wrapper_owner",
            Self::Level2Alt => "mosfet_param_alias_level2_alt_worker",
            Self::Level3Owner => "mosfet_param_alias_level3_wrapper_owner",
            Self::Level3Alt => "mosfet_param_alias_level3_alt_worker",
            Self::Level6Owner => "mosfet_param_alias_level6_wrapper_owner",
            Self::Level6Alt => "mosfet_param_alias_level6_alt_worker",
        }
    }
}

impl XyceTestRunner {
    pub(super) fn mosfet_param_alias_historical_records() -> Vec<String> {
        let mut records = HISTORICAL_ARTIFACTS
            .into_iter()
            .map(|(path, bytes, sha, b3)| {
                format!("{UPSTREAM_COMMIT}\t{UPSTREAM_TAG}\t{path}\t{bytes}\t{sha}\t{b3}")
            })
            .collect::<Vec<_>>();
        records.sort();
        records
    }

    pub(super) fn validate_mosfet_param_alias_historical_provenance() -> Result<(), String> {
        let records = Self::mosfet_param_alias_historical_records();
        let stream = records.join("\n");
        if records.len() != HISTORICAL_RECORD_COUNT
            || stream.len() != HISTORICAL_RECORD_BYTES
            || format!("{:x}", Sha256::digest(stream.as_bytes())) != HISTORICAL_RECORDS_SHA256
            || blake3::hash(stream.as_bytes()).to_hex().as_str() != HISTORICAL_RECORDS_BLAKE3
        {
            return Err(format!("{LABEL} Release-7.10 provenance changed"));
        }
        Ok(())
    }

    fn validate_mosfet_param_alias_directory(&self) -> Result<CapturedMembers, String> {
        let directory = self.root.join(FAMILY_PATH);
        let metadata = fs::symlink_metadata(&directory)
            .map_err(|error| format!("failed to inspect {LABEL} directory: {error}"))?;
        if metadata.file_type().is_symlink() || !metadata.file_type().is_dir() {
            return Err(format!(
                "{LABEL} family must be a regular non-symlink directory"
            ));
        }
        let expected = RETAINED_ARTIFACTS
            .iter()
            .copied()
            .map(|artifact| (artifact.0.to_ascii_lowercase(), artifact))
            .collect::<BTreeMap<_, _>>();
        let mut members = BTreeMap::new();
        let mut records = Vec::new();
        for entry in fs::read_dir(&directory)
            .map_err(|error| format!("failed to read {LABEL} directory: {error}"))?
        {
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
            if members.contains_key(&key) {
                return Err(format!("{LABEL} contains a case collision for {name:?}"));
            }
            let Some((expected_name, expected_bytes, expected_sha, expected_b3)) =
                expected.get(&key).copied()
            else {
                return Err(format!("{LABEL} acquired unexpected member {name:?}"));
            };
            if name != expected_name {
                return Err(format!("{LABEL} member case changed: {name:?}"));
            }
            let cap = expected_bytes
                .checked_mul(2)
                .and_then(|value| value.checked_add(1))
                .ok_or_else(|| format!("{LABEL} member bound overflow"))?;
            if metadata.len() > cap as u64 {
                return Err(format!("{LABEL} member {name:?} is oversized"));
            }
            let mut bytes = Vec::with_capacity(metadata.len() as usize);
            fs::File::open(&path)
                .map_err(|error| format!("failed to open {LABEL} {name:?}: {error}"))?
                .take((cap + 1) as u64)
                .read_to_end(&mut bytes)
                .map_err(|error| format!("failed to read {LABEL} {name:?}: {error}"))?;
            if bytes.len() > cap {
                return Err(format!("{LABEL} member {name:?} exceeded its bound"));
            }
            let canonical = Self::canonical_lf_text_identity(LABEL, &bytes)?;
            let sha = format!("{:x}", Sha256::digest(&canonical));
            let b3 = blake3::hash(&canonical).to_hex().to_string();
            if canonical.len() != expected_bytes || sha != expected_sha || b3 != expected_b3 {
                return Err(format!("{LABEL} member {name:?} changed"));
            }
            records.push(format!("{name}\t{}\t{sha}\t{b3}", canonical.len()));
            members.insert(key, bytes);
        }
        records.sort();
        let stream = records.join("\n");
        if members.len() != expected.len()
            || records.len() != RETAINED_RECORD_COUNT
            || stream.len() != RETAINED_RECORD_BYTES
            || format!("{:x}", Sha256::digest(stream.as_bytes())) != RETAINED_RECORDS_SHA256
            || blake3::hash(stream.as_bytes()).to_hex().as_str() != RETAINED_RECORDS_BLAKE3
        {
            return Err(format!("{LABEL} retained family census changed"));
        }
        Ok(members)
    }

    fn validate_mosfet_param_alias_provenance(
        &self,
        deck: &XyceDeck,
        role: MosfetParamAliasRole,
    ) -> Result<CapturedMembers, String> {
        Self::validate_mosfet_param_alias_historical_provenance()?;
        if deck.section != XyceDeckSection::Netlists
            || Self::normalize_manifest_key(&deck.relative_path) != role.record()
            || Self::normalize_manifest_key(&self.relative_key(&deck.path)) != role.record()
            || !Self::same_path(&deck.path, &self.root.join(role.path()))
        {
            return Err(format!(
                "recognized {LABEL} member is not at its canonical path"
            ));
        }
        let owners = Self::load_upstream_wrapper_decks(&self.root)
            .into_iter()
            .filter(|record| record.starts_with("netlists/mosfet_paramaliases/"))
            .collect::<BTreeSet<_>>();
        let expected_owners = MosfetParamAliasRole::OWNERS
            .into_iter()
            .map(MosfetParamAliasRole::record)
            .collect::<BTreeSet<_>>();
        if owners != expected_owners {
            return Err(format!("{LABEL} wrapper-owner census changed"));
        }
        let exclusions = Self::load_upstream_exclusions(&self.root)
            .map_err(|error| format!("{LABEL} exclusion manifest is invalid: {error}"))?;
        for owner in MosfetParamAliasRole::OWNERS {
            if exclusions.contains_key(&owner.record()) {
                return Err(format!(
                    "{LABEL} owner {} must not be excluded",
                    owner.file_name()
                ));
            }
        }
        let family_exclusions = exclusions
            .iter()
            .filter(|(record, _)| record.starts_with("netlists/mosfet_paramaliases/"))
            .collect::<BTreeMap<_, _>>();
        if family_exclusions.len() != 4 {
            return Err(format!(
                "{LABEL} requires exactly four independently qualified controls"
            ));
        }
        for alt in MosfetParamAliasRole::ALTS {
            let row = family_exclusions
                .get(&alt.record())
                .copied()
                .ok_or_else(|| format!("{LABEL} lost {} qualification", alt.file_name()))?;
            if row.source != EXCLUSION_SOURCE
                || !matches!(&row.disposition, XyceUpstreamExclusionDisposition::RspiceIndependentlyQualified { expected_contract } if expected_contract == alt.contract())
            {
                return Err(format!("{LABEL} {} qualification changed", alt.file_name()));
            }
        }
        match fs::symlink_metadata(self.root.join(OUTPUT_PATH)) {
            Err(error) if error.kind() == std::io::ErrorKind::NotFound => {}
            Err(error) => return Err(format!("failed to inspect {LABEL} OutputData: {error}")),
            Ok(_) => {
                return Err(format!(
                    "{LABEL} must not acquire an invented numerical gold"
                ));
            }
        }
        self.validate_mosfet_param_alias_directory()
    }

    fn mosfet_param_alias_nodes_match(actual: &[String], expected: &[&str]) -> bool {
        actual.len() == expected.len()
            && actual
                .iter()
                .zip(expected)
                .all(|(left, right)| left.eq_ignore_ascii_case(right))
    }

    fn mosfet_param_alias_exact_params(
        actual: &[(String, Value)],
        expected: &[(&str, Value)],
    ) -> bool {
        actual.len() == expected.len()
            && actual.iter().zip(expected).all(
                |((actual_name, actual_value), (expected_name, expected_value))| {
                    actual_name.eq_ignore_ascii_case(expected_name)
                        && actual_value.to_bits() == expected_value.to_bits()
                },
            )
    }

    fn normalized_mosfet_param_alias_models(
        netlist: &Netlist,
        role: MosfetParamAliasRole,
    ) -> Result<NormalizedModels, String> {
        let mut normalized = Vec::new();
        for model in &netlist.models {
            if !model.expr_params.is_empty()
                || !model.string_params.is_empty()
                || !model.string_vector_params.is_empty()
                || !model.real_vector_params.is_empty()
                || !model.real_vector_expr_params.is_empty()
                || !model.integer_vector_params.is_empty()
            {
                return Err(format!(
                    "{LABEL} model {} acquired deferred state",
                    model.name
                ));
            }
            let expected_param_count = if role.level() == 3 { 39 } else { 33 };
            if model.params.len() != expected_param_count {
                return Err(format!(
                    "{LABEL} model {} parameter census changed: expected {expected_param_count}, got {}",
                    model.name,
                    model.params.len()
                ));
            }
            let names = model
                .params
                .iter()
                .map(|(name, _)| name.to_ascii_uppercase())
                .collect::<Vec<_>>();
            let count = |name: &str| {
                names
                    .iter()
                    .filter(|candidate| candidate.as_str() == name)
                    .count()
            };
            let expected_mobility = if role.is_owner() { "UO" } else { "U0" };
            let expected_threshold = if role.is_owner() { "VTO" } else { "VT0" };
            if count(expected_mobility) != 1
                || count(expected_threshold) != 1
                || count(if role.is_owner() { "U0" } else { "UO" }) != 0
                || count(if role.is_owner() { "VT0" } else { "VTO" }) != 0
                || count("VTH0") != 0
                || names.iter().collect::<BTreeSet<_>>().len() != names.len()
            {
                return Err(format!(
                    "{LABEL} model {} alias inventory changed",
                    model.name
                ));
            }
            let mut params = model
                .params
                .iter()
                .map(|(name, value)| {
                    let name = match name.to_ascii_uppercase().as_str() {
                        "UO" | "U0" => "U0".to_string(),
                        "VTO" | "VT0" => "VT0".to_string(),
                        other => other.to_string(),
                    };
                    (name, value.to_bits())
                })
                .collect::<Vec<_>>();
            params.sort();
            normalized.push((
                model.name.to_ascii_uppercase(),
                model.model_type.to_ascii_uppercase(),
                params,
            ));
        }
        normalized.sort();
        Ok(normalized)
    }

    fn validate_mosfet_param_alias_typed_plan(
        &self,
        role: MosfetParamAliasRole,
        source: &str,
        path: &Path,
    ) -> Result<(XyceStaticTranPlan, Netlist, NormalizedModels), String> {
        let plan = self.static_tran_plan_for_path_with_purpose(
            path,
            XyceStaticTranPlanPurpose::ClassicMosParameterAliasRelationalFamily,
        )?;
        let expected_contract = if role.is_owner() {
            XyceStaticTranContract::WrapperStatic
        } else {
            XyceStaticTranContract::PlainStatic
        };
        if plan.deck_path != path
            || plan.source.as_bytes() != source.as_bytes()
            || !matches!(plan.oracle, XyceStaticTranOracle::None)
            || plan.contract != expected_contract
            || plan.comparison_mode != XyceStaticTranComparisonMode::Pointwise
            || plan.output_override
            || plan.timeint_conststep
            || !plan.steps.is_empty()
            || plan.wrapper_tolerance.is_some()
            || plan.tran.step.to_bits() != (20.0f64 * 1e-9).to_bits()
            || plan.tran.stop.to_bits() != (30.0f64 * 1e-6).to_bits()
            || plan.tran.start.is_some()
            || plan.tran.max_step.is_some()
            || plan.tran.uic
            || plan.print.as_ref().is_none_or(|print| {
                print.probes.len() != 3
                    || !print.probes[0].eq_ignore_ascii_case("V(VOUT)")
                    || !print.probes[1].eq_ignore_ascii_case("V(IN)")
                    || !print.probes[2].eq_ignore_ascii_case("V(1)")
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
            || netlist.elements.len() != 7
            || netlist.models.len() != 2
            || netlist.analyses.len() != 1
            || netlist.output_requests.len() != 1
            || !netlist.subcircuits.is_empty()
            || !netlist.data_tables.is_empty()
            || !netlist.measurements.is_empty()
            || !netlist.initial_conditions.is_empty()
            || !netlist.node_sets.is_empty()
            || !netlist.global_nodes.is_empty()
            || !netlist.veriloga_includes.is_empty()
            || !netlist.spef_includes.is_empty()
            || !netlist.params.numeric_parameters().is_empty()
            || !netlist.params.all_string_params().is_empty()
            || !netlist.params.all_functions().is_empty()
            || !Self::netlist_is_native_classic_mos_parameter_alias_envelope(&netlist)
        {
            return Err(format!(
                "{LABEL} {} typed envelope changed",
                role.file_name()
            ));
        }
        let elements = netlist
            .elements
            .iter()
            .map(|element| (element.name.to_ascii_lowercase(), element))
            .collect::<BTreeMap<_, _>>();
        if elements.keys().map(String::as_str).collect::<BTreeSet<_>>()
            != BTreeSet::from(["vdddev", "rin", "vin1", "r1", "c2", "mn1", "mp1"])
        {
            return Err(format!(
                "{LABEL} {} element inventory changed",
                role.file_name()
            ));
        }
        let voltage = elements["vdddev"];
        if voltage.provenance != ElementProvenance::Authored
            || !Self::mosfet_param_alias_nodes_match(&voltage.nodes, &["VDD", "0"])
            || !matches!(voltage.kind, ElementKind::VoltageSource(SourceSpec::Dc(value)) if value.to_bits() == 5.0f64.to_bits())
        {
            return Err(format!("{LABEL} VDD source changed"));
        }
        let pulse = elements["vin1"];
        if pulse.provenance != ElementProvenance::Authored
            || !Self::mosfet_param_alias_nodes_match(&pulse.nodes, &["1", "0"])
            || !matches!(&pulse.kind, ElementKind::VoltageSource(SourceSpec::DcTransient { dc_value, transient }) if dc_value.to_bits() == 5.0f64.to_bits() && matches!(transient.as_ref(), SourceSpec::Pulse { v1, v2, delay, rise, fall, width, period, phase, width_defaults_to_zero } if v1.to_bits() == 5.0f64.to_bits() && v2.to_bits() == 0.0f64.to_bits() && delay.to_bits() == (1.5f64 * 1e-6).to_bits() && rise.to_bits() == (5.0f64 * 1e-9).to_bits() && fall.to_bits() == (5.0f64 * 1e-9).to_bits() && width.to_bits() == (1.5f64 * 1e-6).to_bits() && period.to_bits() == (3.0f64 * 1e-6).to_bits() && phase.to_bits() == 0.0f64.to_bits() && !width_defaults_to_zero))
        {
            return Err(format!("{LABEL} pulse source changed: {:?}", pulse.kind));
        }
        for (key, nodes, value) in [
            ("rin", ["IN", "1"], 1_000.0_f64),
            ("r1", ["VOUT", "0"], 10_000.0_f64),
        ] {
            let element = elements[key];
            if element.provenance != ElementProvenance::Authored
                || !Self::mosfet_param_alias_nodes_match(&element.nodes, &nodes)
                || !matches!(&element.kind, ElementKind::Resistor { value: actual, value_expr: None, model: None, instance_params, deferred_params } if actual.to_bits() == value.to_bits() && instance_params.is_empty() && deferred_params.is_empty())
            {
                return Err(format!("{LABEL} resistor {key} changed"));
            }
        }
        let capacitor = elements["c2"];
        if capacitor.provenance != ElementProvenance::Authored
            || !Self::mosfet_param_alias_nodes_match(&capacitor.nodes, &["VOUT", "0"])
            || !matches!(&capacitor.kind, ElementKind::Capacitor { value, value_expr: None, initial_voltage: None, model: None, instance_params, deferred_params } if value.to_bits() == 0.1e-12f64.to_bits() && instance_params.is_empty() && deferred_params.is_empty())
        {
            return Err(format!("{LABEL} output capacitor changed"));
        }
        for (key, nodes, model, l, w) in [
            (
                "mn1",
                ["VOUT", "IN", "0", "0"],
                "CD4012_NMOS",
                5.0 * 1e-6,
                175.0 * 1e-6,
            ),
            (
                "mp1",
                ["VOUT", "IN", "VDD", "VDD"],
                "CD4012_PMOS",
                5.0 * 1e-6,
                270.0 * 1e-6,
            ),
        ] {
            let element = elements[key];
            let ElementKind::Mosfet {
                model: actual_model,
                compact_syntax: false,
                instance_params,
                deferred_params,
                ..
            } = &element.kind
            else {
                return Err(format!("{LABEL} MOSFET {key} changed"));
            };
            if element.provenance != ElementProvenance::Authored
                || !Self::mosfet_param_alias_nodes_match(&element.nodes, &nodes)
                || !actual_model.eq_ignore_ascii_case(model)
                || !Self::mosfet_param_alias_exact_params(instance_params, &[("L", l), ("W", w)])
                || !deferred_params.is_empty()
            {
                return Err(format!(
                    "{LABEL} MOSFET {key} topology changed: {:?}",
                    element.kind
                ));
            }
        }
        for model in &netlist.models {
            if Self::numeric_param_value(&model.params, "LEVEL")
                .is_none_or(|level| level.to_bits() != (role.level() as f64).to_bits())
            {
                return Err(format!("{LABEL} {} model level changed", model.name));
            }
        }
        let tolerances = Self::xyce_verify_comp_tolerances(
            source,
            &plan.print.as_ref().expect("PRINT checked").probes,
        )?;
        let defaults = XyceVerifyTransientTolerance::release_7_10_default();
        let expected = if role.level() == 3 {
            vec![
                XyceVerifyTransientTolerance {
                    relative: 0.02,
                    absolute: 1e-6,
                    ..defaults
                },
                XyceVerifyTransientTolerance {
                    relative: 0.02,
                    absolute: 1e-6,
                    zero: 1e-6,
                    absolute_difference: 1e-6,
                    ..defaults
                },
                defaults,
            ]
        } else {
            vec![
                XyceVerifyTransientTolerance {
                    relative: 0.02,
                    absolute: 5e-7,
                    ..defaults
                },
                XyceVerifyTransientTolerance {
                    relative: 0.02,
                    absolute: 5e-7,
                    zero: 1e-8,
                    ..defaults
                },
                defaults,
            ]
        };
        if tolerances != expected {
            return Err(format!(
                "{LABEL} {} COMP policy changed: {tolerances:?}",
                role.file_name()
            ));
        }
        let models = Self::normalized_mosfet_param_alias_models(&netlist, role)?;
        Ok((plan, netlist, models))
    }

    fn validate_mosfet_param_alias_table(table: &XycePrnTable) -> Result<(), String> {
        if table.columns.len() != 5
            || table.columns[0] != "Index"
            || !table.columns[1].eq_ignore_ascii_case("TIME")
            || !table.columns[2].eq_ignore_ascii_case("V(VOUT)")
            || !table.columns[3].eq_ignore_ascii_case("V(IN)")
            || !table.columns[4].eq_ignore_ascii_case("V(1)")
            || table.rows.len() < 10
            || table
                .rows
                .iter()
                .any(|row| row.len() != 5 || row.iter().any(|value| !value.is_finite()))
        {
            return Err(format!("{LABEL} produced malformed transient output"));
        }
        for (index, row) in table.rows.iter().enumerate() {
            if row[0].to_bits() != (index as f64).to_bits() {
                return Err(format!("{LABEL} output index sequence changed"));
            }
        }
        for rows in table.rows.windows(2) {
            if rows[1][1] <= rows[0][1] {
                return Err(format!("{LABEL} output index/time grid changed"));
            }
        }
        if table.rows.first().is_none_or(|row| row[1].abs() > 1e-18)
            || table
                .rows
                .last()
                .is_none_or(|row| (row[1] - 30.0 * 1e-6).abs() > 1e-12)
        {
            return Err(format!("{LABEL} output domain changed"));
        }
        for column in 2..=4 {
            let min = table
                .rows
                .iter()
                .map(|row| row[column])
                .fold(Value::INFINITY, Value::min);
            let max = table
                .rows
                .iter()
                .map(|row| row[column])
                .fold(Value::NEG_INFINITY, Value::max);
            if max - min < if column == 2 { 0.05 } else { 4.0 } {
                return Err(format!(
                    "{LABEL} output column {} became vacuous",
                    table.columns[column]
                ));
            }
        }
        Ok(())
    }

    pub(super) fn validate_mosfet_param_alias_oracle(
        &self,
        deck: &XyceDeck,
        role: MosfetParamAliasRole,
        start: Instant,
    ) -> Result<(), String> {
        let abort = DeadlineAbort::new(start, self.config.max_time_per_test_ms.max(1));
        if abort.is_aborted() {
            return Err(format!(
                "{LABEL} deadline expired before provenance validation"
            ));
        }
        let members = self.validate_mosfet_param_alias_provenance(deck, role)?;
        let owner = role.owner();
        let alt = role.alt();
        let build = |member: MosfetParamAliasRole| -> Result<(XyceStaticTranPlan, Netlist, NormalizedModels), String> {
            let bytes = members.get(&member.file_name().to_ascii_lowercase()).ok_or_else(|| format!("{LABEL} lost {}", member.file_name()))?;
            let source = std::str::from_utf8(bytes).map_err(|error| format!("{LABEL} {} is not UTF-8: {error}", member.file_name()))?;
            self.validate_mosfet_param_alias_typed_plan(member, source, &self.root.join(member.path()))
        };
        let (owner_plan, _owner_netlist, owner_models) = build(owner)?;
        let (alt_plan, _alt_netlist, alt_models) = build(alt)?;
        if owner_models != alt_models {
            return Err(format!(
                "{LABEL} LEVEL={} normalized model snapshots differ",
                role.level()
            ));
        }
        let run = |member: MosfetParamAliasRole,
                   plan: &XyceStaticTranPlan|
         -> Result<XycePrnTable, String> {
            let (netlist, result) = self
                .run_transient_family_plan(plan, start, None, None)
                .map_err(|error| format!("{LABEL} {} failed: {error}", member.file_name()))?;
            let table = Self::transient_family_result_to_prn_table(plan, &netlist, &result)?;
            Self::validate_mosfet_param_alias_table(&table)?;
            Ok(table)
        };
        let owner_table = run(owner, &owner_plan)?;
        if abort.is_aborted() {
            return Err(format!("{LABEL} deadline expired between pair runs"));
        }
        let alt_table = run(alt, &alt_plan)?;
        let tolerances = Self::xyce_verify_comp_tolerances(
            &owner_plan.source,
            &owner_plan.print.as_ref().expect("PRINT checked").probes,
        )?;
        let mismatches = self.compare_xyce_verify_transient_tables_with_probe_tolerances(
            &owner_table,
            &alt_table,
            &tolerances,
            XYCE_DEFAULT_PRN_SCIENTIFIC_PRECISION,
        )?;
        if !mismatches.is_empty() {
            return Err(format!(
                "{LABEL} LEVEL={} owner-good to alt-test comparison failed: {mismatches:?}",
                role.level()
            ));
        }
        self.validate_mosfet_param_alias_provenance(deck, role)?;
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

    fn deck(root: &Path, role: MosfetParamAliasRole) -> XyceDeck {
        XyceDeck {
            path: root.join(role.path()),
            section: XyceDeckSection::Netlists,
            relative_path: role.path(),
        }
    }

    fn fixture(label: &str) -> (tempfile::TempDir, XyceDeck, XyceTestRunner) {
        let temporary = tempfile::Builder::new()
            .prefix(&format!("rspice-xyce-mos-alias-{label}-"))
            .tempdir()
            .expect("create MOS alias fixture");
        let root = temporary.path();
        let family = root.join(FAMILY_PATH);
        fs::create_dir_all(&family).expect("create MOS alias family");
        let canonical = corpus_root().join(FAMILY_PATH);
        for (name, ..) in RETAINED_ARTIFACTS {
            fs::copy(canonical.join(name), family.join(name)).expect("copy MOS alias member");
        }
        let owners = MosfetParamAliasRole::OWNERS
            .into_iter()
            .map(|role| format!("{}\t{REQUIRES_UPSTREAM_WRAPPER_CONTRACT}", role.path()))
            .collect::<Vec<_>>()
            .join("\n");
        fs::write(root.join(HARNESS_MANIFEST_FILE), format!("{owners}\n"))
            .expect("write MOS alias wrapper manifest");
        let exclusions = MosfetParamAliasRole::ALTS
            .into_iter()
            .map(|role| {
                format!(
                    "{}\t{EXCLUSION_SOURCE}\t{RSPICE_INDEPENDENTLY_QUALIFIED_DISPOSITION}\t{}",
                    role.path(),
                    role.contract()
                )
            })
            .collect::<Vec<_>>()
            .join("\n");
        fs::write(
            root.join(UPSTREAM_EXCLUSIONS_MANIFEST_FILE),
            format!(
                "schema_version\t{UPSTREAM_EXCLUSIONS_SCHEMA_VERSION}\nsource_commit\t{UPSTREAM_EXCLUSIONS_SOURCE_COMMIT}\nsource_netlists_tree\t{UPSTREAM_EXCLUSIONS_SOURCE_NETLISTS_TREE}\n{exclusions}\n"
            ),
        )
        .expect("write MOS alias exclusion manifest");
        let owner = deck(root, MosfetParamAliasRole::Level1Owner);
        let runner = XyceTestRunner::new(root, XyceRunnerConfig::default());
        (temporary, owner, runner)
    }

    #[test]
    fn mosfet_param_alias_release_provenance_is_exact() {
        XyceTestRunner::validate_mosfet_param_alias_historical_provenance()
            .expect("Release provenance remains exact");
    }

    #[test]
    fn mosfet_param_alias_retained_provenance_is_exact() {
        let root = corpus_root();
        let runner = XyceTestRunner::new(&root, XyceRunnerConfig::default());
        let role = MosfetParamAliasRole::Level1Owner;
        let deck = XyceDeck {
            path: root.join(role.path()),
            section: XyceDeckSection::Netlists,
            relative_path: role.path(),
        };
        runner
            .validate_mosfet_param_alias_provenance(&deck, role)
            .expect("retained family remains exact");
    }

    #[test]
    fn mosfet_param_alias_typed_snapshots_match_only_after_alias_normalization() {
        let root = corpus_root();
        let runner = XyceTestRunner::new(&root, XyceRunnerConfig::default());
        for owner in MosfetParamAliasRole::OWNERS {
            let owner_source = fs::read_to_string(root.join(owner.path())).expect("read owner");
            let owner_parsed =
                XyceTestRunner::parse_xyce_netlist(&owner_source, &root.join(owner.path()))
                    .expect("parse owner");
            assert!(
                XyceTestRunner::netlist_is_native_classic_mos_parameter_alias_envelope(
                    &owner_parsed,
                ),
                "LEVEL={} owner is outside native envelope: models={:#?}, elements={:#?}",
                owner.level(),
                owner_parsed.models,
                owner_parsed.elements
            );
            let alt = owner.alt();
            let alt_source = fs::read_to_string(root.join(alt.path())).expect("read alt");
            let (_, _, owner_models) = runner
                .validate_mosfet_param_alias_typed_plan(
                    owner,
                    &owner_source,
                    &root.join(owner.path()),
                )
                .expect("owner plan");
            let (_, _, alt_models) = runner
                .validate_mosfet_param_alias_typed_plan(alt, &alt_source, &root.join(alt.path()))
                .expect("alt plan");
            assert_eq!(owner_models, alt_models);
        }
    }

    #[test]
    fn mosfet_param_alias_mixed_duplicate_and_vth0_cards_fail_closed() {
        let root = corpus_root();
        let role = MosfetParamAliasRole::Level1Owner;
        let source = fs::read_to_string(root.join(role.path())).expect("read owner");
        for mutated in [
            source.replacen("UO = 310", "U0 = 310 UO = 310", 1),
            source.replacen("UO = 310", "UO = 310 UO = 310", 1),
            source.replacen("VTO = -1.6", "VTH0 = -1.6", 1),
        ] {
            let netlist = XyceTestRunner::parse_xyce_netlist(&mutated, &root.join(role.path()))
                .expect("mutation remains syntactically valid");
            assert!(
                XyceTestRunner::normalized_mosfet_param_alias_models(&netlist, role).is_err(),
                "mixed, duplicate, and non-Release aliases must remain outside this contract"
            );
        }
    }

    fn valid_alias_table() -> XycePrnTable {
        let stop = 30.0 * 1e-6;
        XycePrnTable {
            columns: vec![
                "Index".to_string(),
                "TIME".to_string(),
                "V(VOUT)".to_string(),
                "V(IN)".to_string(),
                "V(1)".to_string(),
            ],
            rows: (0..10)
                .map(|index| {
                    let fraction = index as f64 / 9.0;
                    vec![
                        index as f64,
                        stop * fraction,
                        5.0 * fraction,
                        5.0 * (1.0 - fraction),
                        5.0 * (1.0 - fraction),
                    ]
                })
                .collect(),
        }
    }

    #[test]
    fn mosfet_param_alias_table_shape_and_relation_fail_closed() {
        let table = valid_alias_table();
        XyceTestRunner::validate_mosfet_param_alias_table(&table)
            .expect("non-vacuous canonical table");

        let mut malformed = table.clone();
        malformed.rows[4][2] = Value::NAN;
        assert!(XyceTestRunner::validate_mosfet_param_alias_table(&malformed).is_err());

        let root = corpus_root();
        let runner = XyceTestRunner::new(&root, XyceRunnerConfig::default());
        let tolerances = vec![XyceVerifyTransientTolerance::release_7_10_default(); 3];
        let mut changed = table.clone();
        changed.rows[5][2] += 1.0;
        let mismatches = runner
            .compare_xyce_verify_transient_tables_with_probe_tolerances(
                &table,
                &changed,
                &tolerances,
                XYCE_DEFAULT_PRN_SCIENTIFIC_PRECISION,
            )
            .expect("counterfactual comparison remains well formed");
        assert!(!mismatches.is_empty());
    }

    #[test]
    fn mosfet_param_alias_all_four_native_pairs_execute() {
        let root = corpus_root();
        let runner = XyceTestRunner::new(&root, XyceRunnerConfig::default());
        for owner in MosfetParamAliasRole::OWNERS {
            let alt = owner.alt();
            let owner_source = fs::read_to_string(root.join(owner.path())).expect("read owner");
            let alt_source = fs::read_to_string(root.join(alt.path())).expect("read alt");
            let (owner_plan, _, owner_models) = runner
                .validate_mosfet_param_alias_typed_plan(
                    owner,
                    &owner_source,
                    &root.join(owner.path()),
                )
                .expect("owner plan");
            let (alt_plan, _, alt_models) = runner
                .validate_mosfet_param_alias_typed_plan(alt, &alt_source, &root.join(alt.path()))
                .expect("alt plan");
            assert_eq!(owner_models, alt_models);

            let start = Instant::now();
            let (owner_netlist, owner_result) = runner
                .run_transient_family_plan(&owner_plan, start, None, None)
                .expect("owner native run");
            let (alt_netlist, alt_result) = runner
                .run_transient_family_plan(&alt_plan, start, None, None)
                .expect("alt native run");
            let owner_table = XyceTestRunner::transient_family_result_to_prn_table(
                &owner_plan,
                &owner_netlist,
                &owner_result,
            )
            .expect("owner table");
            let alt_table = XyceTestRunner::transient_family_result_to_prn_table(
                &alt_plan,
                &alt_netlist,
                &alt_result,
            )
            .expect("alt table");
            XyceTestRunner::validate_mosfet_param_alias_table(&owner_table)
                .expect("owner table shape");
            XyceTestRunner::validate_mosfet_param_alias_table(&alt_table).expect("alt table shape");
            let tolerances = XyceTestRunner::xyce_verify_comp_tolerances(
                &owner_plan.source,
                &owner_plan.print.as_ref().expect("PRINT checked").probes,
            )
            .expect("COMP policy");
            let mismatches = runner
                .compare_xyce_verify_transient_tables_with_probe_tolerances(
                    &owner_table,
                    &alt_table,
                    &tolerances,
                    XYCE_DEFAULT_PRN_SCIENTIFIC_PRECISION,
                )
                .expect("pair comparison");
            assert!(
                mismatches.is_empty(),
                "LEVEL={} alias pair diverged: {mismatches:?}",
                owner.level()
            );
        }
    }

    #[test]
    fn mosfet_param_alias_expired_shared_deadline_rejects_before_execution() {
        let root = corpus_root();
        let runner = XyceTestRunner::new(
            &root,
            XyceRunnerConfig {
                max_time_per_test_ms: 1,
                ..XyceRunnerConfig::default()
            },
        );
        let role = MosfetParamAliasRole::Level1Owner;
        let deck = XyceDeck {
            path: root.join(role.path()),
            section: XyceDeckSection::Netlists,
            relative_path: role.path(),
        };
        assert!(
            runner
                .validate_mosfet_param_alias_oracle(
                    &deck,
                    role,
                    Instant::now() - Duration::from_millis(10),
                )
                .is_err()
        );
    }

    #[test]
    fn mosfet_param_alias_provenance_mutations_fail_closed() {
        let (_temporary, owner, runner) = fixture("extra");
        runner
            .validate_mosfet_param_alias_provenance(&owner, MosfetParamAliasRole::Level1Owner)
            .expect("canonical MOS alias fixture");
        fs::write(runner.root.join(FAMILY_PATH).join("unexpected.cir"), "x\n")
            .expect("write unexpected MOS alias member");
        assert!(
            runner
                .validate_mosfet_param_alias_provenance(&owner, MosfetParamAliasRole::Level1Owner,)
                .is_err()
        );

        let (_temporary, owner, runner) = fixture("owner");
        fs::write(runner.root.join(HARNESS_MANIFEST_FILE), "").expect("remove MOS alias owners");
        assert!(
            runner
                .validate_mosfet_param_alias_provenance(&owner, MosfetParamAliasRole::Level1Owner,)
                .is_err()
        );

        let (temporary, owner, runner) = fixture("exclusion");
        let path = temporary.path().join(UPSTREAM_EXCLUSIONS_MANIFEST_FILE);
        let content = fs::read_to_string(&path).expect("read MOS alias exclusions");
        fs::write(
            &path,
            content.replacen(
                "mosfet_param_alias_level1_alt_worker",
                "mosfet_param_alias_level2_alt_worker",
                1,
            ),
        )
        .expect("mutate MOS alias qualification");
        assert!(
            runner
                .validate_mosfet_param_alias_provenance(&owner, MosfetParamAliasRole::Level1Owner,)
                .is_err()
        );

        let (temporary, owner, runner) = fixture("source");
        fs::write(
            temporary.path().join(FAMILY_PATH).join("invert6_alt.cir"),
            "changed\n",
        )
        .expect("mutate MOS alias source");
        assert!(
            runner
                .validate_mosfet_param_alias_provenance(&owner, MosfetParamAliasRole::Level1Owner,)
                .is_err()
        );

        let (_temporary, owner, runner) = fixture("output");
        fs::create_dir_all(runner.root.join(OUTPUT_PATH)).expect("invent MOS alias gold");
        assert!(
            runner
                .validate_mosfet_param_alias_provenance(&owner, MosfetParamAliasRole::Level1Owner,)
                .is_err()
        );
    }
}
