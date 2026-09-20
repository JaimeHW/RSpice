use super::*;
use rspice_core::netlist::{OutputSymbolKind, SaveSignal, SourceSpec};
use std::io::Read as _;

const LABEL: &str = "BUG_1957 multi-winding mutual-inductor relation";
const FAMILY_PATH: &str = "Netlists/Certification_Tests/BUG_1957";
const OUTPUT_PATH: &str = "OutputData/Certification_Tests/BUG_1957";
const EXCLUSION_SOURCE: &str = "Netlists/Certification_Tests/BUG_1957/exclude";
const UPSTREAM_COMMIT: &str = "d6e278e371ec2f3df1325dcff4552e585bc7ecc1";
const UPSTREAM_TAG: &str = "Release-7.10.0";

const HISTORICAL_RECORD_COUNT: usize = 19;
const HISTORICAL_RECORD_BYTES: usize = 4_590;
const HISTORICAL_RECORDS_SHA256: &str =
    "5b10c21973af222b48a08e92142a89dcfae78ff6e66ad1b6ab5d06923e39c080";
const HISTORICAL_RECORDS_BLAKE3: &str =
    "05d1b8e18549660784e11fde42e306d3e5fb7c6b976af60dc13645b970c77489";
const HISTORICAL_ARTIFACTS: [(&str, usize, &str, &str); 19] = [
    (
        "Netlists/Certification_Tests/BUG_1957/CMakeLists.txt",
        3_101,
        "0bac6593e93e5f75d8462b2d5952eabdfb4292f9c439e55133ab17b6a01f3e5f",
        "a5687a70ea3224cd8d91a2da2107043c178f670379aafbcc4f7693ba1a83378b",
    ),
    (
        "Netlists/Certification_Tests/BUG_1957/Manifest.txt",
        244,
        "3efb914c72bc164f2b5751f5d1b20288d8eaff943251bc68fedfd215a33c6691",
        "7905db090c70ec5c18d4dafff76f31aaa3fe2a58a9496532c2305e4873dd69a2",
    ),
    (
        EXCLUSION_SOURCE,
        90,
        "f9ec3e6ad18f333dba6895b4371ccdcc53600940bb835b611bf213f76bd6e59c",
        "3978c3ff8bb94a3f01c589a07f1fe45e666a88ca3417c271773ed0f634b1a18b",
    ),
    (
        "Netlists/Certification_Tests/BUG_1957/mutindlin_even.cir",
        0,
        "e3b0c44298fc1c149afbf4c8996fb92427ae41e4649b934ca495991b7852b855",
        "af1349b9f5f9a1a6a0404dea36dcc9499bcb25c9adc112b7cc9a93cae41f3262",
    ),
    (
        "Netlists/Certification_Tests/BUG_1957/mutindlin_even.cir.sh",
        3_331,
        "7530da31841a8b1bdef2fb794d3b667ab5d99e9c72efdc0d04aed1839a0523fd",
        "452c373a9a269a77013cc12f3edb17407be56064d8c191bd5fbced862a33d8f8",
    ),
    (
        "Netlists/Certification_Tests/BUG_1957/mutindlin_even_ps.cir",
        327,
        "4004f2c10faed0aecdbb2d829149897534c756039422486e5dbe6946d2988978",
        "0f4512315a09e41cd8323630d5d99b675b543947bb4449e51ebfc7493df7e5d3",
    ),
    (
        "Netlists/Certification_Tests/BUG_1957/mutindlin_even_s3f5.cir",
        390,
        "3e47dbae28980555baca6737d88b77cbace82671076f5fb45d3b8fc72364dee3",
        "18bf22eeb84b4197fe35d608c2dbfe8be927399d516effd831f6fd0aa1b10544",
    ),
    (
        "Netlists/Certification_Tests/BUG_1957/mutindlin_even_s3f5.cir.prn.gs",
        17_355,
        "f32dc48efd6096afec5ad4fbcbe5f6657e334693a722a9d87c27aa27c5aaa405",
        "40252b8d95ea575a7a0d6aa510d5acb8534a1f4d1169bf53b27d347ed5b3b2ef",
    ),
    (
        "Netlists/Certification_Tests/BUG_1957/mutindlin_odd.cir",
        0,
        "e3b0c44298fc1c149afbf4c8996fb92427ae41e4649b934ca495991b7852b855",
        "af1349b9f5f9a1a6a0404dea36dcc9499bcb25c9adc112b7cc9a93cae41f3262",
    ),
    (
        "Netlists/Certification_Tests/BUG_1957/mutindlin_odd.cir.sh",
        3_764,
        "14ad06799e8739f09722e60e30cddacee0246b7d34b85f730e60722fff252504",
        "c9078003cb2fe40d786a616f09c74f13dc097c6902a86ed8467791c19ab6f9e1",
    ),
    (
        "Netlists/Certification_Tests/BUG_1957/mutindlin_odd_ps.cir",
        377,
        "1900d7a004fbd31cf3138fdfef1946e667896ffe476f49ff1d5a08d9229215f3",
        "40ad118cc93649b03e210d8d5a7fd2f416b50f8738f5ac0baba13c686860ac2e",
    ),
    (
        "Netlists/Certification_Tests/BUG_1957/mutindlin_odd_s3f5.cir",
        495,
        "9539ff92354d35a2fdad7a44061eb6f87376f7c7976afc0d3171ae59322f3353",
        "7cf4ce52cd53dc69f6c525eab15d584b84445785a635eb4cfe75b57c332f3fef",
    ),
    (
        "Netlists/Certification_Tests/BUG_1957/mutindlin_odd_s3f5.cir.prn.gs",
        17_187,
        "509dbaf17df86d9167d4ec3b4ed4e494ce81a27b37863a17c68ca3ea0f95afb8",
        "089f935371c8fb34de4d49147f1561f8203f030a813660c38836ee3990588cbe",
    ),
    (
        "Netlists/Certification_Tests/BUG_1957/tags",
        48,
        "e1f21975a1970324bb4dfb455b7c36e31a0ee9df3da307e9db7f0eb1bb5dc17b",
        "7d55956df6e2847fbc1dc4553fa9acc2305bb7592ac27b78aebdda3db62ed477",
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
const RETAINED_RECORD_BYTES: usize = 1_259;
const RETAINED_RECORDS_SHA256: &str =
    "fb17b7aabde70a00b4010335082f6c20b5dc12ac7535a83c244e821acc2918fa";
const RETAINED_RECORDS_BLAKE3: &str =
    "01572e44ee2263e77a5727e90f49d565b856511e82059b714bcb674083f08224";
const RETAINED_ARTIFACTS: [(&str, usize, &str, &str); 8] = [
    (
        "mutindlin_even.cir",
        0,
        "e3b0c44298fc1c149afbf4c8996fb92427ae41e4649b934ca495991b7852b855",
        "af1349b9f5f9a1a6a0404dea36dcc9499bcb25c9adc112b7cc9a93cae41f3262",
    ),
    (
        "mutindlin_even_ps.cir",
        327,
        "4004f2c10faed0aecdbb2d829149897534c756039422486e5dbe6946d2988978",
        "0f4512315a09e41cd8323630d5d99b675b543947bb4449e51ebfc7493df7e5d3",
    ),
    (
        "mutindlin_even_s3f5.cir",
        390,
        "3e47dbae28980555baca6737d88b77cbace82671076f5fb45d3b8fc72364dee3",
        "18bf22eeb84b4197fe35d608c2dbfe8be927399d516effd831f6fd0aa1b10544",
    ),
    (
        "mutindlin_even_s3f5.cir.prn.gs",
        17_355,
        "f32dc48efd6096afec5ad4fbcbe5f6657e334693a722a9d87c27aa27c5aaa405",
        "40252b8d95ea575a7a0d6aa510d5acb8534a1f4d1169bf53b27d347ed5b3b2ef",
    ),
    (
        "mutindlin_odd.cir",
        0,
        "e3b0c44298fc1c149afbf4c8996fb92427ae41e4649b934ca495991b7852b855",
        "af1349b9f5f9a1a6a0404dea36dcc9499bcb25c9adc112b7cc9a93cae41f3262",
    ),
    (
        "mutindlin_odd_ps.cir",
        377,
        "1900d7a004fbd31cf3138fdfef1946e667896ffe476f49ff1d5a08d9229215f3",
        "40ad118cc93649b03e210d8d5a7fd2f416b50f8738f5ac0baba13c686860ac2e",
    ),
    (
        "mutindlin_odd_s3f5.cir",
        495,
        "9539ff92354d35a2fdad7a44061eb6f87376f7c7976afc0d3171ae59322f3353",
        "7cf4ce52cd53dc69f6c525eab15d584b84445785a635eb4cfe75b57c332f3fef",
    ),
    (
        "mutindlin_odd_s3f5.cir.prn.gs",
        17_187,
        "509dbaf17df86d9167d4ec3b4ed4e494ce81a27b37863a17c68ca3ea0f95afb8",
        "089f935371c8fb34de4d49147f1561f8203f030a813660c38836ee3990588cbe",
    ),
];

type CapturedMembers = BTreeMap<String, Vec<u8>>;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Bug1957Family {
    Even,
    Odd,
}

impl Bug1957Family {
    fn stem(self) -> &'static str {
        match self {
            Self::Even => "mutindlin_even",
            Self::Odd => "mutindlin_odd",
        }
    }

    fn winding_count(self) -> usize {
        match self {
            Self::Even => 4,
            Self::Odd => 5,
        }
    }

    fn gold_file(self) -> String {
        format!("{}_s3f5.cir.prn.gs", self.stem())
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Bug1957Representation {
    Anchor,
    Multiwinding,
    Pairwise,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(super) struct Bug1957Role {
    family: Bug1957Family,
    representation: Bug1957Representation,
}

impl Bug1957Role {
    const ALL: [Self; 6] = [
        Self::new(Bug1957Family::Even, Bug1957Representation::Anchor),
        Self::new(Bug1957Family::Even, Bug1957Representation::Multiwinding),
        Self::new(Bug1957Family::Even, Bug1957Representation::Pairwise),
        Self::new(Bug1957Family::Odd, Bug1957Representation::Anchor),
        Self::new(Bug1957Family::Odd, Bug1957Representation::Multiwinding),
        Self::new(Bug1957Family::Odd, Bug1957Representation::Pairwise),
    ];

    const fn new(family: Bug1957Family, representation: Bug1957Representation) -> Self {
        Self {
            family,
            representation,
        }
    }

    pub(super) fn for_record(record: &str) -> Option<Self> {
        let normalized = XyceTestRunner::normalize_manifest_key(record);
        Self::ALL
            .into_iter()
            .find(|role| role.record() == normalized)
    }

    fn file_name(self) -> String {
        let suffix = match self.representation {
            Bug1957Representation::Anchor => ".cir",
            Bug1957Representation::Multiwinding => "_ps.cir",
            Bug1957Representation::Pairwise => "_s3f5.cir",
        };
        format!("{}{suffix}", self.family.stem())
    }

    fn path(self) -> String {
        format!("{FAMILY_PATH}/{}", self.file_name())
    }

    fn record(self) -> String {
        Self::normalize_path(&self.path())
    }

    fn normalize_path(path: &str) -> String {
        path.replace('\\', "/").to_ascii_lowercase()
    }

    pub(super) fn contract(self) -> &'static str {
        match (self.family, self.representation) {
            (Bug1957Family::Even, Bug1957Representation::Anchor) => {
                "bug1957_even_mutual_inductor_wrapper_owner"
            }
            (Bug1957Family::Even, Bug1957Representation::Multiwinding) => {
                "bug1957_even_multiwinding_worker"
            }
            (Bug1957Family::Even, Bug1957Representation::Pairwise) => {
                "bug1957_even_pairwise_worker"
            }
            (Bug1957Family::Odd, Bug1957Representation::Anchor) => {
                "bug1957_odd_mutual_inductor_wrapper_owner"
            }
            (Bug1957Family::Odd, Bug1957Representation::Multiwinding) => {
                "bug1957_odd_multiwinding_worker"
            }
            (Bug1957Family::Odd, Bug1957Representation::Pairwise) => "bug1957_odd_pairwise_worker",
        }
    }

    fn worker(family: Bug1957Family, representation: Bug1957Representation) -> Self {
        debug_assert!(representation != Bug1957Representation::Anchor);
        Self::new(family, representation)
    }
}

impl XyceTestRunner {
    pub(super) fn bug1957_historical_oracle_provenance_records() -> Vec<String> {
        let mut records = HISTORICAL_ARTIFACTS
            .into_iter()
            .map(|(path, bytes, sha256, blake3)| {
                format!("{UPSTREAM_COMMIT}\t{UPSTREAM_TAG}\t{path}\t{bytes}\t{sha256}\t{blake3}")
            })
            .collect::<Vec<_>>();
        records.sort();
        records
    }

    pub(super) fn validate_bug1957_historical_oracle_provenance() -> Result<(), String> {
        let records = Self::bug1957_historical_oracle_provenance_records();
        let stream = records.join("\n");
        if records.len() != HISTORICAL_RECORD_COUNT
            || stream.len() != HISTORICAL_RECORD_BYTES
            || format!("{:x}", Sha256::digest(stream.as_bytes())) != HISTORICAL_RECORDS_SHA256
            || blake3::hash(stream.as_bytes()).to_hex().as_str() != HISTORICAL_RECORDS_BLAKE3
        {
            return Err(format!("{LABEL} Release provenance changed"));
        }
        Ok(())
    }

    fn validate_bug1957_directory(&self) -> Result<CapturedMembers, String> {
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
            if members.contains_key(&key) {
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
            let physical_cap = expected_bytes
                .checked_mul(2)
                .and_then(|value| value.checked_add(1))
                .ok_or_else(|| format!("{LABEL} retained-member bound overflow"))?;
            if metadata.len() > physical_cap as u64 {
                return Err(format!("{LABEL} member {name:?} is oversized"));
            }
            let file = fs::File::open(&path)
                .map_err(|error| format!("failed to open {LABEL} {name:?}: {error}"))?;
            let mut bytes = Vec::with_capacity(metadata.len() as usize);
            file.take((physical_cap + 1) as u64)
                .read_to_end(&mut bytes)
                .map_err(|error| format!("failed to read {LABEL} {name:?}: {error}"))?;
            if bytes.len() > physical_cap {
                return Err(format!("{LABEL} member {name:?} is oversized"));
            }
            let canonical = Self::canonical_lf_text_identity(LABEL, &bytes)?;
            if canonical.len() != expected_bytes
                || format!("{:x}", Sha256::digest(&canonical)) != expected_sha256
                || blake3::hash(&canonical).to_hex().as_str() != expected_blake3
            {
                return Err(format!("{LABEL} retained member {name:?} changed"));
            }
            records.push(format!(
                "{expected_name}\t{expected_bytes}\t{expected_sha256}\t{expected_blake3}"
            ));
            members.insert(key, bytes);
        }
        if members.len() != expected.len() {
            return Err(format!("{LABEL} lost a retained member"));
        }
        records.sort();
        let stream = records.join("\n");
        if records.len() != RETAINED_RECORD_COUNT
            || stream.len() != RETAINED_RECORD_BYTES
            || format!("{:x}", Sha256::digest(stream.as_bytes())) != RETAINED_RECORDS_SHA256
            || blake3::hash(stream.as_bytes()).to_hex().as_str() != RETAINED_RECORDS_BLAKE3
        {
            return Err(format!("{LABEL} retained census changed"));
        }
        Ok(members)
    }

    fn validate_bug1957_provenance(
        &self,
        deck: &XyceDeck,
        role: Bug1957Role,
    ) -> Result<CapturedMembers, String> {
        Self::validate_bug1957_historical_oracle_provenance()?;
        let path = role.path();
        if deck.section != XyceDeckSection::Netlists
            || Self::normalize_manifest_key(&deck.relative_path) != role.record()
            || Self::normalize_manifest_key(&self.relative_key(&deck.path)) != role.record()
            || !Self::same_path(&deck.path, &self.root.join(&path))
        {
            return Err(format!(
                "recognized {LABEL} role is not at its canonical path"
            ));
        }

        let prefix = "netlists/certification_tests/bug_1957/";
        let owners = Self::load_upstream_wrapper_decks(&self.root)
            .into_iter()
            .filter(|record| record.starts_with(prefix))
            .collect::<BTreeSet<_>>();
        let expected_owners = [Bug1957Family::Even, Bug1957Family::Odd]
            .into_iter()
            .map(|family| Bug1957Role::new(family, Bug1957Representation::Anchor).record())
            .collect::<BTreeSet<_>>();
        if owners != expected_owners {
            return Err(format!("{LABEL} wrapper ownership changed: {owners:?}"));
        }

        let exclusions = Self::load_upstream_exclusions(&self.root)
            .map_err(|error| format!("{LABEL} exclusion manifest is invalid: {error}"))?;
        let family_rows = exclusions
            .iter()
            .filter(|(record, _)| record.starts_with(prefix))
            .collect::<BTreeMap<_, _>>();
        let expected_workers = Bug1957Role::ALL
            .into_iter()
            .filter(|role| role.representation != Bug1957Representation::Anchor)
            .collect::<Vec<_>>();
        let expected_records = expected_workers
            .iter()
            .map(|role| role.record())
            .collect::<BTreeSet<_>>();
        if family_rows
            .keys()
            .map(|record| record.to_string())
            .collect::<BTreeSet<_>>()
            != expected_records
        {
            return Err(format!("{LABEL} exclusion census changed"));
        }
        for worker in expected_workers {
            let row = family_rows
                .get(&worker.record())
                .ok_or_else(|| format!("{LABEL} lost {} exclusion row", worker.file_name()))?;
            if row.source != EXCLUSION_SOURCE
                || !matches!(&row.disposition,
                    XyceUpstreamExclusionDisposition::RspiceIndependentlyQualified { expected_contract }
                        if expected_contract == worker.contract())
            {
                return Err(format!(
                    "{LABEL} {} qualification changed",
                    worker.file_name()
                ));
            }
        }
        match fs::symlink_metadata(self.root.join(OUTPUT_PATH)) {
            Err(error) if error.kind() == std::io::ErrorKind::NotFound => {}
            Ok(_) => return Err(format!("{LABEL} must not acquire an OutputData family")),
            Err(error) => {
                return Err(format!(
                    "failed to inspect {LABEL} OutputData family: {error}"
                ));
            }
        }
        self.validate_bug1957_directory()
    }

    fn bug1957_expected_probes(family: Bug1957Family) -> Vec<String> {
        let mut probes = vec!["i(vs)".to_string()];
        probes.extend((2..=family.winding_count() + 1).map(|node| format!("v({node})")));
        probes
    }

    fn validate_bug1957_plan(role: Bug1957Role, plan: &XyceStaticTranPlan) -> Result<(), String> {
        let print = plan
            .print
            .as_ref()
            .ok_or_else(|| format!("{LABEL} {} lost its PRINT", role.file_name()))?;
        let probes = print
            .probes
            .iter()
            .map(|probe| Self::normalize_probe(probe))
            .collect::<Vec<_>>();
        if role.representation == Bug1957Representation::Anchor
            || !matches!(plan.oracle, XyceStaticTranOracle::None)
            || plan.output_override
            || plan.timeint_conststep
            || !plan.steps.is_empty()
            || plan.wrapper_tolerance.is_some()
            || plan.contract != XyceStaticTranContract::PlainStatic
            || plan.comparison_mode != XyceStaticTranComparisonMode::Pointwise
            || probes != Self::bug1957_expected_probes(role.family)
            || plan.tran.step.to_bits() != (100.0f64 * 1.0e-6).to_bits()
            || plan.tran.stop.to_bits() != 25.0e-3f64.to_bits()
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

    fn bug1957_nodes_match(nodes: &[String], expected: &[&str]) -> bool {
        nodes.len() == expected.len()
            && nodes
                .iter()
                .zip(expected)
                .all(|(actual, expected)| actual.eq_ignore_ascii_case(expected))
    }

    fn validate_bug1957_passive(
        element: &rspice_core::netlist::Element,
        name: &str,
        nodes: [&str; 2],
        value: Value,
        inductor: bool,
    ) -> Result<(), String> {
        let kind_matches = if inductor {
            matches!(&element.kind, ElementKind::Inductor {
                value: actual, value_expr: None, initial_current: None, model: None,
                instance_params, deferred_params,
            } if actual.to_bits() == value.to_bits()
                && instance_params.is_empty() && deferred_params.is_empty())
        } else {
            matches!(&element.kind, ElementKind::Resistor {
                value: actual, value_expr: None, model: None, instance_params, deferred_params,
            } if actual.to_bits() == value.to_bits()
                && instance_params.is_empty() && deferred_params.is_empty())
        };
        if !element.name.eq_ignore_ascii_case(name)
            || element.provenance != ElementProvenance::Authored
            || !Self::bug1957_nodes_match(&element.nodes, &nodes)
            || !kind_matches
        {
            return Err(format!("{LABEL} passive {name} changed: {element:?}"));
        }
        Ok(())
    }

    fn bug1957_pair_set(netlist: &Netlist) -> Result<BTreeSet<(String, String)>, String> {
        let mut pairs = BTreeSet::new();
        for element in &netlist.elements {
            let ElementKind::Coupling {
                inductors,
                coefficient,
                model,
            } = &element.kind
            else {
                continue;
            };
            if element.provenance != ElementProvenance::Authored
                || !element.nodes.is_empty()
                || coefficient.to_bits() != 0.75f64.to_bits()
                || model.is_some()
                || inductors.len() < 2
            {
                return Err(format!("{LABEL} coupling {} changed", element.name));
            }
            for left in 0..inductors.len() {
                for right in left + 1..inductors.len() {
                    let mut pair = [
                        inductors[left].to_ascii_lowercase(),
                        inductors[right].to_ascii_lowercase(),
                    ];
                    pair.sort();
                    if !pairs.insert((pair[0].clone(), pair[1].clone())) {
                        return Err(format!("{LABEL} duplicate effective coupling {pair:?}"));
                    }
                }
            }
        }
        Ok(pairs)
    }

    fn validate_bug1957_netlist(role: Bug1957Role, netlist: &Netlist) -> Result<(), String> {
        let winding_count = role.family.winding_count();
        let pair_count = winding_count * (winding_count - 1) / 2;
        let coupling_count = if role.representation == Bug1957Representation::Multiwinding {
            1
        } else {
            pair_count
        };
        if role.representation == Bug1957Representation::Anchor
            || netlist.elements.len() != 1 + 2 * winding_count + coupling_count
            || netlist.analyses.len() != 1
            || netlist.lin_analysis.is_some()
            || !netlist.fft_analyses.is_empty()
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
            || !netlist.params.all_params().is_empty()
            || !netlist.params.all_string_params().is_empty()
            || !netlist.params.all_functions().is_empty()
            || !netlist.params.all_parameter_expressions().is_empty()
            || !Self::analytic_timeint_only_options_match(&netlist.options, None, None, None, None)
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
        let source = elements
            .get("vs")
            .ok_or_else(|| format!("{LABEL} lost VS"))?;
        if source.provenance != ElementProvenance::Authored
            || !Self::bug1957_nodes_match(&source.nodes, &["1", "0"])
            || !matches!(&source.kind, ElementKind::VoltageSource(SourceSpec::Sin {
                offset, amplitude, frequency, delay, damping, phase,
            }) if offset.to_bits() == 0.0f64.to_bits()
                && amplitude.to_bits() == 169.7f64.to_bits()
                && frequency.to_bits() == 60.0f64.to_bits()
                && delay.to_bits() == 0.0f64.to_bits()
                && damping.to_bits() == 0.0f64.to_bits()
                && phase.to_bits() == 0.0f64.to_bits())
        {
            return Err(format!("{LABEL} sinusoidal source changed: {source:?}"));
        }
        for index in 0..winding_count {
            let resistor = format!("r{}", index + 1);
            let inductor = if index == 0 {
                "lp".to_string()
            } else {
                format!("l{index}")
            };
            let node = (index + 2).to_string();
            let resistor_nodes = if index == 0 {
                ["1", node.as_str()]
            } else {
                [node.as_str(), "0"]
            };
            Self::validate_bug1957_passive(
                elements
                    .get(&resistor)
                    .ok_or_else(|| format!("{LABEL} lost {resistor}"))?,
                &resistor,
                resistor_nodes,
                1.0e3,
                false,
            )?;
            Self::validate_bug1957_passive(
                elements
                    .get(&inductor)
                    .ok_or_else(|| format!("{LABEL} lost {inductor}"))?,
                &inductor,
                [node.as_str(), "0"],
                1.0e-3,
                true,
            )?;
        }

        let couplings = netlist
            .elements
            .iter()
            .filter(|element| matches!(element.kind, ElementKind::Coupling { .. }))
            .collect::<Vec<_>>();
        if couplings.len() != coupling_count {
            return Err(format!("{LABEL} coupling count changed"));
        }
        if role.representation == Bug1957Representation::Multiwinding {
            let expected = (0..winding_count)
                .map(|index| {
                    if index == 0 {
                        "lp".to_string()
                    } else {
                        format!("l{index}")
                    }
                })
                .collect::<Vec<_>>();
            if !couplings[0].name.eq_ignore_ascii_case("K1")
                || !matches!(&couplings[0].kind, ElementKind::Coupling { inductors, .. }
                    if inductors.iter().map(|name| name.to_ascii_lowercase()).collect::<Vec<_>>() == expected)
            {
                return Err(format!("{LABEL} multi-winding syntax changed"));
            }
        }
        let pairs = Self::bug1957_pair_set(netlist)?;
        if pairs.len() != pair_count {
            return Err(format!(
                "{LABEL} effective coupling topology changed: {pairs:?}"
            ));
        }

        if !matches!(&netlist.analyses[0], AnalysisCommand::Tran {
            step, stop, start: None, max_step: None, uic: false,
        } if step.to_bits() == (100.0f64 * 1.0e-6).to_bits()
            && stop.to_bits() == 25.0e-3f64.to_bits())
        {
            return Err(format!("{LABEL} typed transient changed"));
        }
        let request = &netlist.output_requests[0];
        let expected_probes = Self::bug1957_expected_probes(role.family);
        if request.directive != OutputDirectiveKind::Print
            || request.analysis != Some(rspice_core::netlist::OutputAnalysisKind::Tran)
            || request.name.is_some()
            || request.print_delimiter.as_ref() != Some(&PrintDelimiter::Whitespace)
            || !request.expressions.is_empty()
            || request.dependencies.len() != expected_probes.len()
            || request
                .dependencies
                .iter()
                .enumerate()
                .any(|(index, dependency)| {
                    dependency.expression
                        || if index == 0 {
                            dependency.kind != OutputSymbolKind::Device
                                || !dependency.operator.eq_ignore_ascii_case("I")
                                || !dependency.symbol.eq_ignore_ascii_case("VS")
                        } else {
                            dependency.kind != OutputSymbolKind::Node
                                || !dependency.operator.eq_ignore_ascii_case("V")
                                || dependency.symbol != (index + 1).to_string()
                        }
                })
        {
            return Err(format!("{LABEL} typed PRINT request changed: {request:?}"));
        }
        let saves = netlist
            .saves
            .signals
            .iter()
            .map(|signal| match signal {
                SaveSignal::Current(name) => format!("i({})", name.to_ascii_lowercase()),
                SaveSignal::Voltage(node) => format!("v({})", node.to_ascii_lowercase()),
                other => format!("{other:?}"),
            })
            .collect::<Vec<_>>();
        if saves != expected_probes {
            return Err(format!("{LABEL} SaveSet changed: {saves:?}"));
        }
        Ok(())
    }

    fn validate_bug1957_table(family: Bug1957Family, table: &XycePrnTable) -> Result<(), String> {
        let mut expected = vec!["Index".to_string(), "TIME".to_string()];
        expected.extend(Self::bug1957_expected_probes(family));
        if table.columns.len() != expected.len()
            || table
                .columns
                .iter()
                .zip(&expected)
                .any(|(actual, expected)| !actual.eq_ignore_ascii_case(expected))
            || table.rows.len() < 3
        {
            return Err(format!("{LABEL} output shape changed: {table:?}"));
        }
        let mut prior_time = None;
        for (index, row) in table.rows.iter().enumerate() {
            if row.len() != expected.len()
                || row.iter().any(|value| !value.is_finite())
                || row[0].to_bits() != (index as Value).to_bits()
                || prior_time.is_some_and(|prior| row[1] <= prior)
            {
                return Err(format!("{LABEL} output row {index} is malformed"));
            }
            prior_time = Some(row[1]);
        }
        let first = &table.rows[0];
        let last = table.rows.last().expect("BUG1957 table is nonempty");
        if first[1].to_bits() != 0.0f64.to_bits()
            || first[2..]
                .iter()
                .any(|value| value.to_bits() != 0.0f64.to_bits())
            || (last[1] - 25.0e-3).abs() > 1.0e-14
            || table
                .rows
                .iter()
                .all(|row| row[2..].iter().all(|value| value.abs() < 1.0e-8))
        {
            return Err(format!("{LABEL} output domain became vacuous"));
        }
        Ok(())
    }

    fn bug1957_plan(&self, role: Bug1957Role) -> Result<XyceStaticTranPlan, String> {
        let plan = self.static_tran_plan_for_path_with_purpose(
            &self.root.join(role.path()),
            XyceStaticTranPlanPurpose::RelationalFamily,
        )?;
        Self::validate_bug1957_plan(role, &plan)?;
        Ok(plan)
    }

    pub(super) fn validate_bug1957_oracle(
        &self,
        deck: &XyceDeck,
        role: Bug1957Role,
        start: Instant,
    ) -> Result<(), String> {
        let abort = DeadlineAbort::new(start, self.config.max_time_per_test_ms.max(1));
        if abort.is_aborted() {
            return Err(format!("{LABEL} deadline expired before provenance"));
        }
        let members = self.validate_bug1957_provenance(deck, role)?;
        let run = |representation: Bug1957Representation| {
            let member_role = Bug1957Role::worker(role.family, representation);
            let plan = self.bug1957_plan(member_role)?;
            let source = members
                .get(&member_role.file_name().to_ascii_lowercase())
                .ok_or_else(|| format!("{LABEL} lost {}", member_role.file_name()))?;
            if plan.source.as_bytes() != source.as_slice() {
                return Err(format!(
                    "{LABEL} {} changed between reads",
                    member_role.file_name()
                ));
            }
            let (netlist, result) = self
                .run_transient_family_plan(&plan, start, None, None)
                .map_err(|error| match error {
                    SimulationError::Aborted => format!("{LABEL} execution exceeded deadline"),
                    other => format!("{LABEL} execution failed: {other}"),
                })?;
            Self::validate_bug1957_netlist(member_role, &netlist)?;
            let table = Self::transient_family_result_to_prn_table(&plan, &netlist, &result)?;
            Self::validate_bug1957_table(role.family, &table)?;
            Ok::<_, String>((plan, netlist, table))
        };

        // Preserve the Release wrapper's independent-run order.
        let (pairwise_plan, pairwise_netlist, pairwise_table) =
            run(Bug1957Representation::Pairwise)?;
        if abort.is_aborted() {
            return Err(format!("{LABEL} deadline expired between independent runs"));
        }
        let (multi_plan, multi_netlist, multi_table) = run(Bug1957Representation::Multiwinding)?;
        if Self::bug1957_pair_set(&pairwise_netlist)? != Self::bug1957_pair_set(&multi_netlist)? {
            return Err(format!(
                "{LABEL} representations no longer expand to the same topology"
            ));
        }
        let relational =
            self.compare_serialized_default_prn_tables(&pairwise_table, &multi_table)?;
        if !relational.is_empty() {
            return Err(format!("{LABEL} syntax relation failed: {relational:?}"));
        }

        let gold_name = role.family.gold_file();
        let gold_bytes = members
            .get(&gold_name.to_ascii_lowercase())
            .ok_or_else(|| format!("{LABEL} lost {gold_name}"))?;
        let gold_source = std::str::from_utf8(gold_bytes)
            .map_err(|error| format!("{LABEL} gold is not UTF-8: {error}"))?;
        let gold = Self::parse_prn_table(gold_source)?;
        Self::validate_bug1957_table(role.family, &gold)?;
        for (plan, table, representation) in [
            (&pairwise_plan, &pairwise_table, "pairwise"),
            (&multi_plan, &multi_table, "multi-winding"),
        ] {
            let print = plan.print.as_ref().expect("BUG1957 plan has PRINT");
            let tolerances = Self::xyce_verify_comp_tolerances(&plan.source, &print.probes)?;
            let mismatches = self.compare_xyce_verify_transient_tables_with_probe_tolerances(
                &gold,
                table,
                &tolerances,
                XYCE_DEFAULT_PRN_SCIENTIFIC_PRECISION,
            )?;
            if !mismatches.is_empty() {
                return Err(format!(
                    "{LABEL} {representation} absolute Release oracle failed: {mismatches:?}"
                ));
            }
        }
        self.validate_bug1957_provenance(deck, role)?;
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

    fn fixture(label: &str) -> (tempfile::TempDir, XyceDeck, XyceTestRunner) {
        let temporary = tempfile::Builder::new()
            .prefix(&format!("rspice-xyce-bug1957-{label}-"))
            .tempdir()
            .expect("create BUG1957 fixture");
        let root = temporary.path();
        let family = root.join(FAMILY_PATH);
        fs::create_dir_all(&family).expect("create BUG1957 family");
        let canonical = corpus_root().join(FAMILY_PATH);
        for (name, ..) in RETAINED_ARTIFACTS {
            fs::copy(canonical.join(name), family.join(name)).expect("copy BUG1957 member");
        }
        fs::write(
            root.join(HARNESS_MANIFEST_FILE),
            format!(
                "{FAMILY_PATH}/mutindlin_even.cir\t{REQUIRES_UPSTREAM_WRAPPER_CONTRACT}\n{FAMILY_PATH}/mutindlin_odd.cir\t{REQUIRES_UPSTREAM_WRAPPER_CONTRACT}\n"
            ),
        )
        .expect("write BUG1957 wrapper manifest");
        let exclusions = Bug1957Role::ALL
            .into_iter()
            .filter(|role| role.representation != Bug1957Representation::Anchor)
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
        .expect("write BUG1957 exclusion manifest");
        let role = Bug1957Role::new(Bug1957Family::Even, Bug1957Representation::Anchor);
        let deck = XyceDeck {
            path: root.join(role.path()),
            section: XyceDeckSection::Netlists,
            relative_path: role.path(),
        };
        let runner = XyceTestRunner::new(root, XyceRunnerConfig::default());
        (temporary, deck, runner)
    }

    #[test]
    fn bug1957_historical_provenance_is_exact() {
        XyceTestRunner::validate_bug1957_historical_oracle_provenance()
            .expect("BUG1957 Release provenance remains exact");
    }

    #[test]
    fn bug1957_all_roles_execute_the_absolute_and_relational_oracles() {
        let root = corpus_root();
        let runner = XyceTestRunner::new(&root, XyceRunnerConfig::default());
        for role in Bug1957Role::ALL {
            let deck = XyceDeck {
                path: root.join(role.path()),
                section: XyceDeckSection::Netlists,
                relative_path: role.path(),
            };
            runner
                .validate_bug1957_oracle(&deck, role, Instant::now())
                .unwrap_or_else(|error| panic!("{}: {error}", role.file_name()));
        }
    }

    #[test]
    fn bug1957_typed_and_numeric_counterfactuals_fail_closed() {
        let root = corpus_root();
        let runner = XyceTestRunner::new(&root, XyceRunnerConfig::default());
        let role = Bug1957Role::worker(Bug1957Family::Even, Bug1957Representation::Pairwise);
        let mut plan = runner.bug1957_plan(role).expect("canonical BUG1957 plan");
        let netlist = XyceTestRunner::parse_xyce_netlist(&plan.source, &plan.deck_path)
            .expect("canonical BUG1957 netlist");
        XyceTestRunner::validate_bug1957_netlist(role, &netlist)
            .expect("canonical BUG1957 topology");
        let changed_coupling = plan.source.replacen(".75", ".70", 1);
        let changed_netlist =
            XyceTestRunner::parse_xyce_netlist(&changed_coupling, &plan.deck_path)
                .expect("mutated BUG1957 coupling still parses");
        assert!(XyceTestRunner::validate_bug1957_netlist(role, &changed_netlist).is_err());
        plan.tran.stop = 24.0e-3;
        assert!(XyceTestRunner::validate_bug1957_plan(role, &plan).is_err());

        let gold =
            XyceTestRunner::parse_prn_file(&root.join(FAMILY_PATH).join(role.family.gold_file()))
                .expect("parse BUG1957 gold");
        let mut wrong = gold.clone();
        wrong.rows[5][3] += 1.0;
        let tolerances = XyceTestRunner::xyce_verify_comp_tolerances(
            &runner.bug1957_plan(role).expect("BUG1957 plan").source,
            &runner
                .bug1957_plan(role)
                .expect("BUG1957 plan")
                .print
                .expect("BUG1957 print")
                .probes,
        )
        .expect("BUG1957 tolerances");
        assert!(
            !runner
                .compare_xyce_verify_transient_tables_with_probe_tolerances(
                    &gold,
                    &wrong,
                    &tolerances,
                    XYCE_DEFAULT_PRN_SCIENTIFIC_PRECISION,
                )
                .expect("counterfactual comparison")
                .is_empty()
        );
    }

    #[test]
    fn bug1957_provenance_mutations_fail_closed() {
        let (_temporary, deck, runner) = fixture("provenance");
        runner
            .validate_bug1957_provenance(
                &deck,
                Bug1957Role::new(Bug1957Family::Even, Bug1957Representation::Anchor),
            )
            .expect("canonical BUG1957 fixture");
        fs::write(
            runner.root.join(FAMILY_PATH).join("unexpected.cir"),
            "unexpected\n",
        )
        .expect("write extra BUG1957 member");
        assert!(
            runner
                .validate_bug1957_provenance(
                    &deck,
                    Bug1957Role::new(Bug1957Family::Even, Bug1957Representation::Anchor),
                )
                .is_err()
        );
        fs::remove_file(runner.root.join(FAMILY_PATH).join("unexpected.cir"))
            .expect("remove extra BUG1957 member");
        fs::create_dir_all(runner.root.join(OUTPUT_PATH)).expect("create invented BUG1957 output");
        assert!(
            runner
                .validate_bug1957_provenance(
                    &deck,
                    Bug1957Role::new(Bug1957Family::Even, Bug1957Representation::Anchor),
                )
                .is_err()
        );

        let (_owner_fixture, owner_deck, owner_runner) = fixture("owner-drift");
        fs::write(
            owner_runner.root.join(HARNESS_MANIFEST_FILE),
            format!("{FAMILY_PATH}/mutindlin_odd.cir\t{REQUIRES_UPSTREAM_WRAPPER_CONTRACT}\n"),
        )
        .expect("mutate BUG1957 wrapper ownership");
        assert!(
            owner_runner
                .validate_bug1957_provenance(
                    &owner_deck,
                    Bug1957Role::new(Bug1957Family::Even, Bug1957Representation::Anchor),
                )
                .is_err()
        );

        let (exclusion_fixture, exclusion_deck, exclusion_runner) = fixture("exclusion-drift");
        let exclusion_path = exclusion_fixture
            .path()
            .join(UPSTREAM_EXCLUSIONS_MANIFEST_FILE);
        let exclusions = fs::read_to_string(&exclusion_path).expect("read BUG1957 exclusions");
        fs::write(
            &exclusion_path,
            exclusions.replacen(
                "bug1957_even_multiwinding_worker",
                "bug1957_even_pairwise_worker",
                1,
            ),
        )
        .expect("mutate BUG1957 worker qualification");
        assert!(
            exclusion_runner
                .validate_bug1957_provenance(
                    &exclusion_deck,
                    Bug1957Role::new(Bug1957Family::Even, Bug1957Representation::Anchor),
                )
                .is_err()
        );

        let (source_fixture, source_deck, source_runner) = fixture("source-drift");
        fs::write(
            source_fixture
                .path()
                .join(FAMILY_PATH)
                .join("mutindlin_even_ps.cir"),
            "changed\n",
        )
        .expect("mutate BUG1957 retained source");
        assert!(
            source_runner
                .validate_bug1957_provenance(
                    &source_deck,
                    Bug1957Role::new(Bug1957Family::Even, Bug1957Representation::Anchor),
                )
                .is_err()
        );
    }

    #[test]
    fn bug1957_expired_deadline_rejects_before_execution() {
        let root = corpus_root();
        let runner = XyceTestRunner::new(
            &root,
            XyceRunnerConfig {
                max_time_per_test_ms: 1,
                ..XyceRunnerConfig::default()
            },
        );
        let role = Bug1957Role::new(Bug1957Family::Even, Bug1957Representation::Anchor);
        let deck = XyceDeck {
            path: root.join(role.path()),
            section: XyceDeckSection::Netlists,
            relative_path: role.path(),
        };
        assert!(
            runner
                .validate_bug1957_oracle(&deck, role, Instant::now() - Duration::from_millis(10),)
                .is_err()
        );
    }
}
