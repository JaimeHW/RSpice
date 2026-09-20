use super::*;
use rspice_core::netlist::SourceSpec;
use std::io::Read as _;

const LABEL: &str = "BUG_806_SON DC DATA/baseline relation";
const FAMILY_PATH: &str = "Netlists/Certification_Tests/BUG_806_SON";
const OUTPUT_PATH: &str = "OutputData/Certification_Tests/BUG_806_SON";
const OWNER_PATH: &str = "Netlists/Certification_Tests/BUG_806_SON/dc_data.cir";
const BASELINE_PATH: &str = "Netlists/Certification_Tests/BUG_806_SON/dc_baseline.cir";
const OWNER_RECORD: &str = "netlists/certification_tests/bug_806_son/dc_data.cir";
const BASELINE_RECORD: &str = "netlists/certification_tests/bug_806_son/dc_baseline.cir";
const EXCLUSION_SOURCE: &str = "Netlists/Certification_Tests/BUG_806_SON/exclude";
const OWNER_CONTRACT: &str = "bug806_dc_data_relational_wrapper_owner";
const BASELINE_CONTRACT: &str = "bug806_dc_data_relational_wrapper_baseline";
const UPSTREAM_COMMIT: &str = "d6e278e371ec2f3df1325dcff4552e585bc7ecc1";
const UPSTREAM_TAG: &str = "Release-7.10.0";

const HISTORICAL_RECORD_COUNT: usize = 11;
const HISTORICAL_RECORD_BYTES: usize = 2_600;
const HISTORICAL_RECORDS_SHA256: &str =
    "e47dd0b66f9bdc2e6e90e9eb5d62f3aa95d63457403658ac28f8c2c09d41edea";
const HISTORICAL_RECORDS_BLAKE3: &str =
    "b894c6df801e914389af14971acfb5bf2cf5a7690eff644193fced3d6f583197";
const HISTORICAL_ARTIFACTS: [(&str, usize, &str, &str); 11] = [
    (
        "Netlists/Certification_Tests/BUG_806_SON/CMakeLists.txt",
        10_345,
        "8ab1329d2278d1aa83c251a5739c3830e8f4f44269ff9d2d96cf6f6b54e2b23d",
        "415c1948023adf6f217353c5556e12b534a189c7e8930351bfd95be08bc502da",
    ),
    (
        "Netlists/Certification_Tests/BUG_806_SON/Manifest.txt",
        528,
        "306916da311a4955fea746d502d79a1f989f132c224ff0d7d6d6a23af51e16ee",
        "f09a17e3591e8180144a2d2ddbb87297230edf90863a20288043501367446b1f",
    ),
    (
        BASELINE_PATH,
        185,
        "dfee6f8b84a6e3e5f4f01cfd86ffd41fae6193db378cd37455b93e774c661638",
        "079d91cffaf76a44f36e508d63153628d3221a3bb3e1e102e6196291d680d6d8",
    ),
    (
        OWNER_PATH,
        834,
        "16834fa29b3144d3a3474937801c29714501050c08c8bd17d0a844b001a04dba",
        "27f4a0f1156affd671ca01b24d0bd9574a80f1b94d7b064d00fdb6f1f927d995",
    ),
    (
        "Netlists/Certification_Tests/BUG_806_SON/dc_data.cir.sh",
        1_494,
        "40df64b630ae54d0b2a59fbae8fb8ae30a0f26491e5f27537fc092ecbc214d18",
        "f15021a8defe6caa6cd255338bce32a6be350e3a423e7d4e9cc85a106cbd683b",
    ),
    (
        EXCLUSION_SOURCE,
        103,
        "e21dc2afc9a75c28872f81446cce2a0ccfa4ad48c1d41b8f49fa50473957479e",
        "07f98a3ad38c79a297620080fab495a1f2d18f57de453adc8a4a9adea5a7a346",
    ),
    (
        "Netlists/Certification_Tests/BUG_806_SON/tags",
        44,
        "b583524144f523434e0f8a3e85a4d936b96e3a62e0071e1c2741039f25b3f02a",
        "98606d80a78f8c4c5636b86ca534bf913bc54e891e4296d8b1099aa424605155",
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

const RETAINED_RECORD_COUNT: usize = 14;
const RETAINED_RECORD_BYTES: usize = 2_775;
const RETAINED_RECORDS_SHA256: &str =
    "48e719269389951d062c47f552c6905171843836c05b1ac5173cfbc5b8f18e57";
const RETAINED_RECORDS_BLAKE3: &str =
    "339f011a5c2c3c4aee1ef0f9f7d0dfe82c844f196aaecde7be6a71db6d2d658d";
const RETAINED_SOURCE_ARTIFACTS: [(&str, usize, &str, &str); 13] = [
    (
        "dc_baseline.cir",
        185,
        "dfee6f8b84a6e3e5f4f01cfd86ffd41fae6193db378cd37455b93e774c661638",
        "079d91cffaf76a44f36e508d63153628d3221a3bb3e1e102e6196291d680d6d8",
    ),
    (
        "dc_baselineGP.cir",
        185,
        "dfee6f8b84a6e3e5f4f01cfd86ffd41fae6193db378cd37455b93e774c661638",
        "079d91cffaf76a44f36e508d63153628d3221a3bb3e1e102e6196291d680d6d8",
    ),
    (
        "dc_data.cir",
        834,
        "16834fa29b3144d3a3474937801c29714501050c08c8bd17d0a844b001a04dba",
        "27f4a0f1156affd671ca01b24d0bd9574a80f1b94d7b064d00fdb6f1f927d995",
    ),
    (
        "dc_data_two_table.cir",
        678,
        "f8f6a91fa52ad71e5fd9e7b04b417a8166f532ce5f7145f64dac19c49ae4121d",
        "c708b2c634cc5dcc0c4b3b6117f7d202c7fad6d33924f4df089e5692d4d9fd45",
    ),
    (
        "dc_globalparam_data.cir",
        909,
        "288a91df29af49a72abbd574c96140b55b0d1b3b1de9b716ab155c23ed8ed0e9",
        "4e2ac4beed3f0d4373b2cf969b94b1468518de49266605b2b285d260b58d54ed",
    ),
    (
        "mos2_baseline.cir",
        852,
        "bcd3f0f0f7253b15bf68c8ea14ccc74edfc0f2ffb3d8758894d76d24d80cdc00",
        "f7350056e20aec2f54328590699d2bf2e57a35ca956aa9a3c0b3a44acce6927f",
    ),
    (
        "mos2_baseline_forglobal.cir",
        852,
        "bcd3f0f0f7253b15bf68c8ea14ccc74edfc0f2ffb3d8758894d76d24d80cdc00",
        "f7350056e20aec2f54328590699d2bf2e57a35ca956aa9a3c0b3a44acce6927f",
    ),
    (
        "mos2_data_step.cir",
        1_272,
        "c48c63524de5bffa72fe8d283cf731d6b1ac6b125fc3ada374e7402e79d14615",
        "78938c6337bb8f08109a3452703d1cc85a81591f0accb23ecd653b2f11f33cbf",
    ),
    (
        "mos2_data_step_globalparam.cir",
        1_181,
        "e41bed01a4117b11051e98a50bb54a55dfde89fda6e209a01deb92584ebfd0cd",
        "5915f342493a4c081e08e466c7127af4873637e1c3f28e162fca7d7c5b603e1c",
    ),
    (
        "transLineDataGlobal.cir",
        630,
        "d01bd04751780490ec5fcaa88876062f7e7ea4a3cc0ec9939c77c1f4009fae09",
        "f43b4b032cfd1424d4856c5391c8672c5f16f3c22e6e1b74e26a7ade94af6906",
    ),
    (
        "transLineDataGlobal.cir.res.gs",
        556,
        "427a9c9a65f9330788b4a7521724950497d31010f6ee90a73b0e3452e57a0aef",
        "ebc170b898226c0e4117d7c141ea1005c1d453effbee18ce955629311842bc18",
    ),
    (
        "transLine_baseline.cir",
        327,
        "0505001f10e24c027b4fdd7bc7f08e2ec6ec6b0bdaa542fc5cc45f89af63ed45",
        "352a6bd2754fc3bef0fdbab16a18edd8fcba9474a0a153e8159296eab831928a",
    ),
    (
        "transLine_baseline.cir.res.gs",
        556,
        "17cb5e4481706257e4fdb3ff4f3a9cd9357d6822ba72e718bb29eb968fd1ff6d",
        "69c7fa352cc888a71176c23f4526421df6b160751b92c614bcffb41c60c03941",
    ),
];
const RETAINED_OUTPUT_ARTIFACTS: [(&str, usize, &str, &str); 1] = [(
    "dc_data_two_table.cir.prn",
    207,
    "03d2cf7d211b9e1ce8c8a2bea7ec1d36aa52788ef00b3b814b3990624b36447b",
    "0a1a72daf0bdb6268e893e8b5e3ff5602356be2cd8c6160d94e9dc0b7afeba62",
)];

type Bug806CapturedDirectory = (BTreeMap<String, Vec<u8>>, Vec<String>);

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(super) enum Bug806Role {
    DataOwner,
    Baseline,
}

impl Bug806Role {
    const ALL: [Self; 2] = [Self::DataOwner, Self::Baseline];

    pub(super) fn for_record(record: &str) -> Option<Self> {
        let normalized = XyceTestRunner::normalize_manifest_key(record);
        Self::ALL
            .into_iter()
            .find(|role| role.record() == normalized)
    }

    pub(super) fn contract(self) -> &'static str {
        match self {
            Self::DataOwner => OWNER_CONTRACT,
            Self::Baseline => BASELINE_CONTRACT,
        }
    }

    fn path(self) -> &'static str {
        match self {
            Self::DataOwner => OWNER_PATH,
            Self::Baseline => BASELINE_PATH,
        }
    }

    fn record(self) -> &'static str {
        match self {
            Self::DataOwner => OWNER_RECORD,
            Self::Baseline => BASELINE_RECORD,
        }
    }

    fn file_name(self) -> &'static str {
        self.path().rsplit('/').next().expect("BUG806 file name")
    }
}

impl XyceTestRunner {
    pub(super) fn bug806_historical_oracle_provenance_records() -> Vec<String> {
        let mut records = HISTORICAL_ARTIFACTS
            .into_iter()
            .map(|(path, bytes, sha256, blake3)| {
                format!("{UPSTREAM_COMMIT}\t{UPSTREAM_TAG}\t{path}\t{bytes}\t{sha256}\t{blake3}")
            })
            .collect::<Vec<_>>();
        records.sort();
        records
    }

    pub(super) fn validate_bug806_historical_oracle_provenance() -> Result<(), String> {
        let records = Self::bug806_historical_oracle_provenance_records();
        let stream = records.join("\n");
        let sha256 = format!("{:x}", Sha256::digest(stream.as_bytes()));
        let content_blake3 = blake3::hash(stream.as_bytes()).to_hex().to_string();
        if records.len() != HISTORICAL_RECORD_COUNT
            || stream.len() != HISTORICAL_RECORD_BYTES
            || sha256 != HISTORICAL_RECORDS_SHA256
            || content_blake3 != HISTORICAL_RECORDS_BLAKE3
        {
            return Err(format!(
                "{LABEL} Release provenance changed: records={}/{}, sha256={sha256}, blake3={content_blake3}",
                records.len(),
                stream.len()
            ));
        }
        Ok(())
    }

    fn validate_bug806_directory(
        &self,
        directory_path: &str,
        record_prefix: &str,
        expected_artifacts: &[(&str, usize, &str, &str)],
    ) -> Result<Bug806CapturedDirectory, String> {
        let directory = self.root.join(directory_path);
        let metadata = fs::symlink_metadata(&directory)
            .map_err(|error| format!("failed to inspect {LABEL} {directory_path}: {error}"))?;
        if metadata.file_type().is_symlink() || !metadata.file_type().is_dir() {
            return Err(format!(
                "{LABEL} {directory_path} must be a regular non-symlink directory"
            ));
        }
        let expected = expected_artifacts
            .iter()
            .copied()
            .map(|artifact| (artifact.0.to_ascii_lowercase(), artifact))
            .collect::<BTreeMap<_, _>>();
        let mut observed = BTreeMap::new();
        let mut records = Vec::new();
        for entry in fs::read_dir(&directory)
            .map_err(|error| format!("failed to read {LABEL} {directory_path}: {error}"))?
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
            let physical_cap = expected_bytes
                .checked_mul(2)
                .and_then(|value| value.checked_add(1))
                .ok_or_else(|| format!("{LABEL} member bound overflow"))?;
            if metadata.len() > physical_cap as u64 {
                return Err(format!("{LABEL} retained member {name:?} is oversized"));
            }
            let file = fs::File::open(&path)
                .map_err(|error| format!("failed to open {LABEL} {name:?}: {error}"))?;
            let mut bytes = Vec::with_capacity(metadata.len() as usize);
            file.take((physical_cap + 1) as u64)
                .read_to_end(&mut bytes)
                .map_err(|error| format!("failed to read {LABEL} {name:?}: {error}"))?;
            if bytes.len() > physical_cap {
                return Err(format!("{LABEL} retained member {name:?} is oversized"));
            }
            let canonical = Self::canonical_lf_text_identity(LABEL, &bytes)?;
            if canonical.len() != expected_bytes
                || format!("{:x}", Sha256::digest(&canonical)) != expected_sha256
                || blake3::hash(&canonical).to_hex().as_str() != expected_blake3
            {
                return Err(format!("{LABEL} retained member {name:?} changed"));
            }
            records.push(format!(
                "{record_prefix}/{expected_name}\t{expected_bytes}\t{expected_sha256}\t{expected_blake3}"
            ));
            observed.insert(key, bytes);
        }
        if observed.len() != expected.len() {
            return Err(format!("{LABEL} {directory_path} lost a retained member"));
        }
        Ok((observed, records))
    }

    fn validate_bug806_provenance(
        &self,
        deck: &XyceDeck,
        role: Bug806Role,
    ) -> Result<BTreeMap<String, Vec<u8>>, String> {
        Self::validate_bug806_historical_oracle_provenance()?;
        if deck.section != XyceDeckSection::Netlists
            || Self::normalize_manifest_key(&deck.relative_path) != role.record()
            || Self::normalize_manifest_key(&self.relative_key(&deck.path)) != role.record()
            || !Self::same_path(&deck.path, &self.root.join(role.path()))
        {
            return Err(format!(
                "recognized {LABEL} role is not at its canonical path"
            ));
        }

        let prefix = "netlists/certification_tests/bug_806_son/";
        let owners = Self::load_upstream_wrapper_decks(&self.root)
            .into_iter()
            .filter(|record| record.starts_with(prefix))
            .collect::<BTreeSet<_>>();
        let expected_owners = BTreeSet::from([
            OWNER_RECORD.to_string(),
            format!("{prefix}dc_data_two_table.cir"),
            format!("{prefix}dc_globalparam_data.cir"),
            format!("{prefix}mos2_data_step.cir"),
            format!("{prefix}mos2_data_step_globalparam.cir"),
            format!("{prefix}translinedataglobal.cir"),
        ]);
        if owners != expected_owners {
            return Err(format!("{LABEL} wrapper ownership changed: {owners:?}"));
        }

        let exclusions = Self::load_upstream_exclusions(&self.root)
            .map_err(|error| format!("{LABEL} exclusion manifest is invalid: {error}"))?;
        if exclusions.contains_key(OWNER_RECORD) {
            return Err(format!("{LABEL} owner must not be excluded"));
        }
        let family = exclusions
            .iter()
            .filter(|(record, _)| record.starts_with(prefix))
            .collect::<BTreeMap<_, _>>();
        let expected_records = BTreeSet::from([
            BASELINE_RECORD,
            "netlists/certification_tests/bug_806_son/dc_baselinegp.cir",
            "netlists/certification_tests/bug_806_son/mos2_baseline.cir",
            "netlists/certification_tests/bug_806_son/mos2_baseline_forglobal.cir",
            "netlists/certification_tests/bug_806_son/transline_baseline.cir",
        ]);
        if family
            .keys()
            .map(|record| record.as_str())
            .collect::<BTreeSet<_>>()
            != expected_records
        {
            return Err(format!("{LABEL} exclusion census changed"));
        }
        for (record, row) in family {
            if row.source != EXCLUSION_SOURCE {
                return Err(format!("{LABEL} exclusion source changed for {record}"));
            }
            if record == BASELINE_RECORD {
                if !matches!(&row.disposition,
                    XyceUpstreamExclusionDisposition::RspiceIndependentlyQualified { expected_contract }
                        if expected_contract == BASELINE_CONTRACT)
                {
                    return Err(format!("{LABEL} baseline qualification changed"));
                }
            } else if !matches!(row.disposition, XyceUpstreamExclusionDisposition::Excluded) {
                return Err(format!("{LABEL} unrelated exclusion changed for {record}"));
            }
        }

        let (members, mut records) =
            self.validate_bug806_directory(FAMILY_PATH, FAMILY_PATH, &RETAINED_SOURCE_ARTIFACTS)?;
        let (_, output_records) =
            self.validate_bug806_directory(OUTPUT_PATH, OUTPUT_PATH, &RETAINED_OUTPUT_ARTIFACTS)?;
        records.extend(output_records);
        records.sort();
        let stream = records.join("\n");
        if records.len() != RETAINED_RECORD_COUNT
            || stream.len() != RETAINED_RECORD_BYTES
            || format!("{:x}", Sha256::digest(stream.as_bytes())) != RETAINED_RECORDS_SHA256
            || blake3::hash(stream.as_bytes()).to_hex().as_str() != RETAINED_RECORDS_BLAKE3
        {
            return Err(format!("{LABEL} retained source/output census changed"));
        }
        for member_role in Bug806Role::ALL {
            self.reject_wrapper_output_artifacts(&self.root.join(member_role.path()))
                .map_err(|error| format!("{LABEL} {} {error}", member_role.file_name()))?;
        }
        Ok(members)
    }

    fn validate_bug806_plan(role: Bug806Role, plan: &XyceStaticDcPlan) -> Result<(), String> {
        let probes = plan
            .print
            .probes
            .iter()
            .map(|probe| Self::normalize_probe(probe))
            .collect::<Vec<_>>();
        let print_precisions = Self::dc_print_precisions(&plan.source)?;
        if plan.execution_dir.is_some()
            || plan.expression_dialect != ExpressionDialect::Xyce
            || plan.parameter_redefinition_policy != ParameterRedefinitionPolicy::UseLast
            || plan.parameter_redefinition_diagnostic_policy
                != rspice_core::netlist::ParameterRedefinitionDiagnosticPolicy::Silent
            || plan.print_format.is_some()
            || !plan.steps.is_empty()
            || !plan.diagnostics.is_empty()
            || probes != ["{r1:r}", "{r2:r}", "v(4)", "v(5)"]
            || print_precisions.len() != 4
            || print_precisions.values().any(|precision| *precision != 4)
        {
            return Err(format!(
                "{LABEL} {} plan changed: {plan:?}",
                role.file_name()
            ));
        }
        match role {
            Bug806Role::DataOwner => {
                let data = plan
                    .dc_data
                    .as_ref()
                    .ok_or_else(|| format!("{LABEL} owner lost its typed DATA sweep"))?;
                if data.rows.len() != 15
                    || !plan.dc.source.eq_ignore_ascii_case("DATA")
                    || plan.dc.start.to_bits() != 0.0f64.to_bits()
                    || plan.dc.stop.to_bits() != 14.0f64.to_bits()
                    || plan.dc.step.to_bits() != 1.0f64.to_bits()
                    || !matches!(plan.dc.mode, DcSweepMode::Linear)
                    || plan.dc.sweep2.is_some()
                {
                    return Err(format!("{LABEL} DATA sweep changed: {plan:?}"));
                }
                for (row_index, row) in data.rows.iter().enumerate() {
                    let expected_r1 = 8.0 + (row_index % 5) as Value;
                    let expected_r2 = 4.0 + (row_index / 5) as Value;
                    if row.overrides.len() != 2
                        || !matches!(&row.overrides[0], XyceDcDataOverride::Device { name, param_name: None, value }
                            if name.eq_ignore_ascii_case("R1") && value.to_bits() == expected_r1.to_bits())
                        || !matches!(&row.overrides[1], XyceDcDataOverride::Device { name, param_name: None, value }
                            if name.eq_ignore_ascii_case("R2") && value.to_bits() == expected_r2.to_bits())
                    {
                        return Err(format!("{LABEL} DATA row {} changed", row_index + 1));
                    }
                }
            }
            Bug806Role::Baseline => {
                let second = plan.dc.sweep2.as_ref();
                if plan.dc_data.is_some()
                    || !plan.dc.source.eq_ignore_ascii_case("R1:R")
                    || plan.dc.start.to_bits() != 8.0f64.to_bits()
                    || plan.dc.stop.to_bits() != 12.0f64.to_bits()
                    || plan.dc.step.to_bits() != 1.0f64.to_bits()
                    || !matches!(plan.dc.mode, DcSweepMode::Linear)
                    || !matches!(second, Some(second)
                        if second.source.eq_ignore_ascii_case("R2:R")
                            && second.start.to_bits() == 4.0f64.to_bits()
                            && second.stop.to_bits() == 6.0f64.to_bits()
                            && second.step.to_bits() == 1.0f64.to_bits()
                            && matches!(second.mode, DcSweepMode::Linear))
                {
                    return Err(format!("{LABEL} baseline sweep changed: {plan:?}"));
                }
            }
        }
        Ok(())
    }

    fn validate_bug806_resistor(
        element: &rspice_core::netlist::Element,
        name: &str,
        nodes: [&str; 2],
        value_bits: u64,
    ) -> Result<(), String> {
        if !element.name.eq_ignore_ascii_case(name)
            || element.provenance != ElementProvenance::Authored
            || element.nodes.len() != 2
            || element
                .nodes
                .iter()
                .zip(nodes)
                .any(|(actual, expected)| !actual.eq_ignore_ascii_case(expected))
            || !matches!(&element.kind, ElementKind::Resistor {
                value,
                value_expr: None,
                model: None,
                instance_params,
                deferred_params,
            } if value.to_bits() == value_bits
                && instance_params.is_empty()
                && deferred_params.is_empty())
        {
            return Err(format!("{LABEL} resistor {name} changed: {element:?}"));
        }
        Ok(())
    }

    fn validate_bug806_netlist(role: Bug806Role, netlist: &Netlist) -> Result<(), String> {
        if netlist.elements.len() != 3
            || netlist.output_requests.len() != 1
            || !netlist.models.is_empty()
            || !netlist.subcircuits.is_empty()
            || !netlist.measurements.is_empty()
            || !netlist.initial_conditions.is_empty()
            || netlist.device_initial_conditions.is_some()
            || !netlist.node_sets.is_empty()
            || !netlist.global_nodes.is_empty()
            || !netlist.veriloga_includes.is_empty()
            || !netlist.spef_includes.is_empty()
            || !netlist.diagnostics.is_empty()
            || !netlist.params.all_params().is_empty()
            || !netlist.params.all_string_params().is_empty()
            || !netlist.params.all_functions().is_empty()
            || !netlist.params.all_parameter_expressions().is_empty()
            || !netlist.params.all_global_expressions().is_empty()
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
            != BTreeSet::from(["r1", "r2", "vt1"])
        {
            return Err(format!("{LABEL} element inventory changed"));
        }
        let source = elements["vt1"];
        if source.provenance != ElementProvenance::Authored
            || source.nodes.len() != 2
            || !source.nodes[0].eq_ignore_ascii_case("4")
            || !source.nodes[1].eq_ignore_ascii_case("0")
            || !matches!(source.kind, ElementKind::VoltageSource(SourceSpec::Dc(value))
                if value.to_bits() == 10.0f64.to_bits())
        {
            return Err(format!("{LABEL} voltage source changed: {source:?}"));
        }
        Self::validate_bug806_resistor(elements["r1"], "R1", ["4", "5"], 10.0f64.to_bits())?;
        Self::validate_bug806_resistor(elements["r2"], "R2", ["5", "0"], 5.0f64.to_bits())?;

        let request = &netlist.output_requests[0];
        if request.directive != OutputDirectiveKind::Print
            || request.analysis != Some(rspice_core::netlist::OutputAnalysisKind::Dc)
            || request.name.is_some()
            || request.print_delimiter != Some(PrintDelimiter::Whitespace)
        {
            return Err(format!("{LABEL} typed .PRINT request changed: {request:?}"));
        }

        match role {
            Bug806Role::DataOwner => {
                if netlist.data_tables.len() != 1
                    || netlist.data_tables[0].name != "test"
                    || netlist.data_tables[0].params != ["r1", "r2"]
                    || netlist.data_tables[0].rows.len() != 15
                    || !matches!(netlist.analyses.as_slice(), [AnalysisCommand::Dc {
                        source,
                        start,
                        stop,
                        step,
                        mode: DcSweepMode::Linear,
                        sweep2: None,
                    }] if source.eq_ignore_ascii_case("VT1")
                        && start.to_bits() == 10.0f64.to_bits()
                        && stop.to_bits() == 15.0f64.to_bits()
                        && step.to_bits() == 1.0f64.to_bits())
                {
                    return Err(format!("{LABEL} owner DATA/precedence structure changed"));
                }
                for (row_index, row) in netlist.data_tables[0].rows.iter().enumerate() {
                    let expected = [
                        8.0 + (row_index % 5) as Value,
                        4.0 + (row_index / 5) as Value,
                    ];
                    if row.len() != 2
                        || row
                            .iter()
                            .zip(expected)
                            .any(|(actual, expected)| actual.to_bits() != expected.to_bits())
                    {
                        return Err(format!("{LABEL} typed DATA row {} changed", row_index + 1));
                    }
                }
            }
            Bug806Role::Baseline => {
                if !netlist.data_tables.is_empty()
                    || !matches!(netlist.analyses.as_slice(), [AnalysisCommand::Dc {
                        source,
                        start,
                        stop,
                        step,
                        mode: DcSweepMode::Linear,
                        sweep2: Some(second),
                    }] if source.eq_ignore_ascii_case("R1:R")
                        && start.to_bits() == 8.0f64.to_bits()
                        && stop.to_bits() == 12.0f64.to_bits()
                        && step.to_bits() == 1.0f64.to_bits()
                        && second.source.eq_ignore_ascii_case("R2:R")
                        && second.start.to_bits() == 4.0f64.to_bits()
                        && second.stop.to_bits() == 6.0f64.to_bits()
                        && second.step.to_bits() == 1.0f64.to_bits()
                        && matches!(second.mode, DcSweepMode::Linear))
                {
                    return Err(format!("{LABEL} baseline typed sweep changed"));
                }
            }
        }
        Ok(())
    }

    fn bug806_data_results_to_prn_table(
        plan: &XyceStaticDcPlan,
        points: &[XyceDcDataPointResult],
    ) -> Result<XycePrnTable, String> {
        let mut columns = vec!["Index".to_string()];
        columns.extend(plan.print.probes.iter().cloned());
        let mut rows = Vec::with_capacity(points.len());
        for (row_index, point) in points.iter().enumerate() {
            let expected_sweep_value = (row_index + 1) as Value;
            if point.point.sweep_value.to_bits() != expected_sweep_value.to_bits() {
                return Err(format!(
                    "{LABEL} DATA row {} synthetic sweep coordinate changed: {} != {expected_sweep_value}",
                    row_index + 1,
                    point.point.sweep_value
                ));
            }
            let sweep_point = XyceDcSweepPoint {
                primary: point.point.sweep_value,
                secondary: None,
            };
            let mut row = vec![row_index as Value];
            for probe in &plan.print.probes {
                row.push(Self::evaluate_dc_probe(
                    probe,
                    &point.netlist,
                    &plan.dc,
                    sweep_point,
                    &point.point.result,
                    &point.point.device_op_report,
                )?);
            }
            rows.push(row);
        }
        Ok(XycePrnTable { columns, rows })
    }

    fn validate_bug806_table(table: &XycePrnTable) -> Result<(), String> {
        if table.columns.len() != 5
            || table.rows.len() != 15
            || !table.columns[0].eq_ignore_ascii_case("Index")
            || table.columns[1..]
                .iter()
                .map(|column| Self::normalize_probe(column))
                .ne(["{r1:r}", "{r2:r}", "v(4)", "v(5)"])
        {
            return Err(format!("{LABEL} output shape changed: {table:?}"));
        }
        for (row_index, row) in table.rows.iter().enumerate() {
            if row.len() != 5 || row.iter().any(|value| !value.is_finite()) {
                return Err(format!("{LABEL} row {row_index} is malformed"));
            }
            let r1 = 8.0 + (row_index % 5) as Value;
            let r2 = 4.0 + (row_index / 5) as Value;
            let expected = [row_index as Value, r1, r2, 10.0, 10.0 * r2 / (r1 + r2)];
            for (column_index, (actual, expected)) in row.iter().zip(expected).enumerate() {
                let actual = Self::xyce_prn_scientific_roundtrip(*actual, 4)?;
                let expected = Self::xyce_prn_scientific_roundtrip(expected, 4)?;
                if actual != expected {
                    return Err(format!(
                        "{LABEL} analytic row {row_index} column {column_index} changed: {actual} != {expected}"
                    ));
                }
            }
        }
        Ok(())
    }

    fn compare_bug806_precision4_tables(
        expected: &XycePrnTable,
        actual: &XycePrnTable,
    ) -> Result<bool, String> {
        if expected.columns != actual.columns || expected.rows.len() != actual.rows.len() {
            return Ok(false);
        }
        for (row_index, (expected_row, actual_row)) in
            expected.rows.iter().zip(&actual.rows).enumerate()
        {
            if expected_row.len() != expected.columns.len()
                || actual_row.len() != actual.columns.len()
                || expected_row[0].to_bits() != (row_index as Value).to_bits()
                || actual_row[0].to_bits() != (row_index as Value).to_bits()
            {
                return Ok(false);
            }
            for column_index in 1..expected.columns.len() {
                if Self::xyce_prn_scientific_text(expected_row[column_index], 4)?
                    != Self::xyce_prn_scientific_text(actual_row[column_index], 4)?
                {
                    return Ok(false);
                }
            }
        }
        Ok(true)
    }

    fn bug806_plan(&self, role: Bug806Role) -> Result<XyceStaticDcPlan, String> {
        let plan =
            self.static_dc_plan_for_path(&self.root.join(role.path()), ExpressionDialect::Xyce)?;
        Self::validate_bug806_plan(role, &plan)?;
        Ok(plan)
    }

    pub(super) fn validate_bug806_oracle(
        &self,
        deck: &XyceDeck,
        role: Bug806Role,
        start: Instant,
    ) -> Result<(), String> {
        let abort = DeadlineAbort::new(start, self.config.max_time_per_test_ms.max(1));
        if abort.is_aborted() {
            return Err(format!(
                "{LABEL} exceeded its shared deadline before provenance"
            ));
        }
        let members = self.validate_bug806_provenance(deck, role)?;

        let baseline_plan = self.bug806_plan(Bug806Role::Baseline)?;
        if baseline_plan.source.as_bytes()
            != members
                .get(&Bug806Role::Baseline.file_name().to_ascii_lowercase())
                .ok_or_else(|| format!("{LABEL} lost its baseline"))?
                .as_slice()
        {
            return Err(format!("{LABEL} baseline source changed between reads"));
        }
        let (baseline_netlist, baseline_results) = self
            .run_static_dc_results(&baseline_plan, start)
            .map_err(|error| format!("{LABEL} baseline execution failed: {error}"))?;
        Self::validate_bug806_netlist(Bug806Role::Baseline, &baseline_netlist)?;
        let baseline_table =
            self.dc_results_to_prn_table(&baseline_plan, &baseline_netlist, &baseline_results)?;
        Self::validate_bug806_table(&baseline_table)?;

        if abort.is_aborted() {
            return Err(format!("{LABEL} exceeded its shared deadline between runs"));
        }
        let data_plan = self.bug806_plan(Bug806Role::DataOwner)?;
        if data_plan.source.as_bytes()
            != members
                .get(&Bug806Role::DataOwner.file_name().to_ascii_lowercase())
                .ok_or_else(|| format!("{LABEL} lost its DATA owner"))?
                .as_slice()
        {
            return Err(format!("{LABEL} DATA source changed between reads"));
        }
        let data_netlist = Self::parse_xyce_netlist(&data_plan.source, &data_plan.deck_path)
            .map_err(|error| format!("{LABEL} DATA parse failed: {error}"))?;
        Self::validate_bug806_netlist(Bug806Role::DataOwner, &data_netlist)?;
        let data_sweep = data_plan
            .dc_data
            .as_ref()
            .ok_or_else(|| format!("{LABEL} DATA plan lost its sweep"))?;
        let data_results = self
            .run_static_dc_data_results(&data_netlist, data_sweep, start)
            .map_err(|error| format!("{LABEL} DATA execution failed: {error}"))?;
        let data_table = Self::bug806_data_results_to_prn_table(&data_plan, &data_results)?;
        Self::validate_bug806_table(&data_table)?;

        if !Self::compare_bug806_precision4_tables(&baseline_table, &data_table)? {
            let mut serialized_baseline = baseline_table.clone();
            for row in &mut serialized_baseline.rows {
                for value in &mut row[1..] {
                    *value = Self::xyce_prn_scientific_roundtrip(*value, 4)?;
                }
            }
            let fallback = self.compare_dc_data_prn_reference(
                &serialized_baseline,
                &data_plan.print,
                &data_plan.source,
                &data_plan.dc,
                &data_results,
            )?;
            if !fallback.is_empty() {
                return Err(format!(
                    "{LABEL} failed both exact Release diff and xyce_verify fallback: {fallback:?}"
                ));
            }
        }

        self.validate_bug806_provenance(deck, role)?;
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

    fn write_fixture_manifests(root: &Path) {
        let owners = [
            OWNER_PATH,
            "Netlists/Certification_Tests/BUG_806_SON/dc_data_two_table.cir",
            "Netlists/Certification_Tests/BUG_806_SON/dc_globalparam_data.cir",
            "Netlists/Certification_Tests/BUG_806_SON/mos2_data_step.cir",
            "Netlists/Certification_Tests/BUG_806_SON/mos2_data_step_globalparam.cir",
            "Netlists/Certification_Tests/BUG_806_SON/transLineDataGlobal.cir",
        ];
        fs::write(
            root.join(HARNESS_MANIFEST_FILE),
            owners
                .into_iter()
                .map(|path| format!("{path}\t{REQUIRES_UPSTREAM_WRAPPER_CONTRACT}"))
                .collect::<Vec<_>>()
                .join("\n")
                + "\n",
        )
        .expect("write BUG806 wrapper manifest");
        let rows = [
            format!(
                "{BASELINE_PATH}\t{EXCLUSION_SOURCE}\t{RSPICE_INDEPENDENTLY_QUALIFIED_DISPOSITION}\t{BASELINE_CONTRACT}"
            ),
            format!(
                "Netlists/Certification_Tests/BUG_806_SON/dc_baselineGP.cir\t{EXCLUSION_SOURCE}\t{UPSTREAM_EXCLUDED_DISPOSITION}"
            ),
            format!(
                "Netlists/Certification_Tests/BUG_806_SON/mos2_baseline.cir\t{EXCLUSION_SOURCE}\t{UPSTREAM_EXCLUDED_DISPOSITION}"
            ),
            format!(
                "Netlists/Certification_Tests/BUG_806_SON/mos2_baseline_forglobal.cir\t{EXCLUSION_SOURCE}\t{UPSTREAM_EXCLUDED_DISPOSITION}"
            ),
            format!(
                "Netlists/Certification_Tests/BUG_806_SON/transLine_baseline.cir\t{EXCLUSION_SOURCE}\t{UPSTREAM_EXCLUDED_DISPOSITION}"
            ),
        ];
        fs::write(
            root.join(UPSTREAM_EXCLUSIONS_MANIFEST_FILE),
            format!(
                "schema_version\t{UPSTREAM_EXCLUSIONS_SCHEMA_VERSION}\nsource_commit\t{UPSTREAM_EXCLUSIONS_SOURCE_COMMIT}\nsource_netlists_tree\t{UPSTREAM_EXCLUSIONS_SOURCE_NETLISTS_TREE}\n{}\n",
                rows.join("\n")
            ),
        )
        .expect("write BUG806 exclusion manifest");
    }

    fn fixture(label: &str) -> (tempfile::TempDir, XyceDeck, XyceTestRunner) {
        let temporary = tempfile::Builder::new()
            .prefix(&format!("rspice-xyce-bug806-{label}-"))
            .tempdir()
            .expect("create BUG806 fixture");
        let root = temporary.path();
        let source_dir = root.join(FAMILY_PATH);
        let output_dir = root.join(OUTPUT_PATH);
        fs::create_dir_all(&source_dir).expect("create BUG806 source family");
        fs::create_dir_all(&output_dir).expect("create BUG806 output family");
        let canonical = corpus_root();
        for (name, ..) in RETAINED_SOURCE_ARTIFACTS {
            fs::copy(
                canonical.join(FAMILY_PATH).join(name),
                source_dir.join(name),
            )
            .expect("copy BUG806 source member");
        }
        for (name, ..) in RETAINED_OUTPUT_ARTIFACTS {
            fs::copy(
                canonical.join(OUTPUT_PATH).join(name),
                output_dir.join(name),
            )
            .expect("copy BUG806 output member");
        }
        write_fixture_manifests(root);
        let runner = XyceTestRunner::new(root, XyceRunnerConfig::default());
        let deck = XyceDeck {
            path: root.join(OWNER_PATH),
            section: XyceDeckSection::Netlists,
            relative_path: OWNER_PATH.to_string(),
        };
        (temporary, deck, runner)
    }

    #[test]
    fn bug806_historical_provenance_is_exact() {
        XyceTestRunner::validate_bug806_historical_oracle_provenance()
            .expect("BUG806 Release provenance remains exact");
    }

    #[test]
    fn bug806_typed_plans_and_native_oracle_pass_for_both_roles() {
        let root = corpus_root();
        let runner = XyceTestRunner::new(&root, XyceRunnerConfig::default());
        for role in Bug806Role::ALL {
            let deck = XyceDeck {
                path: root.join(role.path()),
                section: XyceDeckSection::Netlists,
                relative_path: role.path().to_string(),
            };
            runner
                .validate_bug806_oracle(&deck, role, Instant::now())
                .expect("BUG806 role executes its exact native relation");
        }
    }

    #[test]
    fn bug806_analytic_oracle_rejects_a_shared_wrong_divider() {
        let mut table = XycePrnTable {
            columns: vec![
                "Index".to_string(),
                "{R1:R}".to_string(),
                "{R2:R}".to_string(),
                "V(4)".to_string(),
                "V(5)".to_string(),
            ],
            rows: (0..15)
                .map(|row_index| {
                    let r1 = 8.0 + (row_index % 5) as Value;
                    let r2 = 4.0 + (row_index / 5) as Value;
                    vec![row_index as Value, r1, r2, 10.0, 10.0 * r2 / (r1 + r2)]
                })
                .collect(),
        };
        XyceTestRunner::validate_bug806_table(&table).expect("analytic fixture passes");
        table.rows[7][4] += 0.1;
        assert!(XyceTestRunner::validate_bug806_table(&table).is_err());
    }

    #[test]
    fn bug806_typed_mutations_fail_closed() {
        let root = corpus_root();
        let runner = XyceTestRunner::new(&root, XyceRunnerConfig::default());
        let owner_path = root.join(OWNER_PATH);
        let source = fs::read_to_string(&owner_path).expect("read BUG806 owner");

        let changed_row = source.replacen("1.0000e+01  5.0000e+00", "1.0000e+01  5.5000e+00", 1);
        let plan = runner
            .static_dc_plan_for_source_with_execution_dir(
                &owner_path,
                changed_row,
                ExpressionDialect::Xyce,
                None,
            )
            .expect("mutated DATA deck still plans");
        assert!(XyceTestRunner::validate_bug806_plan(Bug806Role::DataOwner, &plan).is_err());

        let changed_precedence = source.replacen(".DC VT1 10 15 1", ".DC VT1 10 14 1", 1);
        let netlist = XyceTestRunner::parse_xyce_netlist(&changed_precedence, &owner_path)
            .expect("mutated precedence deck still parses");
        assert!(XyceTestRunner::validate_bug806_netlist(Bug806Role::DataOwner, &netlist).is_err());
    }

    #[test]
    fn bug806_provenance_reloads_and_rejects_family_drift() {
        let (temporary, deck, runner) = fixture("provenance");
        runner
            .validate_bug806_provenance(&deck, Bug806Role::DataOwner)
            .expect("canonical fixture provenance passes");

        fs::write(
            temporary.path().join(HARNESS_MANIFEST_FILE),
            "Netlists/Certification_Tests/BUG_806_SON/dc_data_two_table.cir\trequires_upstream_wrapper\n",
        )
        .expect("mutate wrapper manifest after runner construction");
        assert!(
            runner
                .validate_bug806_provenance(&deck, Bug806Role::DataOwner)
                .is_err()
        );

        write_fixture_manifests(temporary.path());
        fs::write(
            temporary.path().join(FAMILY_PATH).join("unexpected.cir"),
            "unexpected\n.end\n",
        )
        .expect("add unexpected family member");
        assert!(
            runner
                .validate_bug806_provenance(&deck, Bug806Role::DataOwner)
                .is_err()
        );
    }

    #[test]
    fn bug806_provenance_rejects_source_output_and_exclusion_drift() {
        let (source_fixture, source_deck, source_runner) = fixture("source-drift");
        fs::write(
            source_fixture.path().join(OWNER_PATH),
            "changed BUG806 owner\n.end\n",
        )
        .expect("mutate BUG806 source");
        assert!(
            source_runner
                .validate_bug806_provenance(&source_deck, Bug806Role::DataOwner)
                .is_err()
        );

        let (output_fixture, output_deck, output_runner) = fixture("output-drift");
        fs::write(
            output_fixture
                .path()
                .join(OUTPUT_PATH)
                .join("dc_data_two_table.cir.prn"),
            "invented replacement\n",
        )
        .expect("mutate unrelated retained output");
        assert!(
            output_runner
                .validate_bug806_provenance(&output_deck, Bug806Role::DataOwner)
                .is_err()
        );

        let (exclusion_fixture, exclusion_deck, exclusion_runner) = fixture("exclusion-drift");
        let manifest_path = exclusion_fixture
            .path()
            .join(UPSTREAM_EXCLUSIONS_MANIFEST_FILE);
        let manifest = fs::read_to_string(&manifest_path).expect("read fixture exclusions");
        fs::write(
            &manifest_path,
            manifest.replacen(
                &format!(
                    "{BASELINE_PATH}\t{EXCLUSION_SOURCE}\t{RSPICE_INDEPENDENTLY_QUALIFIED_DISPOSITION}\t{BASELINE_CONTRACT}"
                ),
                &format!(
                    "{BASELINE_PATH}\t{EXCLUSION_SOURCE}\t{UPSTREAM_EXCLUDED_DISPOSITION}"
                ),
                1,
            ),
        )
        .expect("mutate BUG806 exclusion disposition");
        assert!(
            exclusion_runner
                .validate_bug806_provenance(&exclusion_deck, Bug806Role::DataOwner)
                .is_err()
        );
    }

    #[test]
    fn bug806_expired_shared_deadline_rejects_before_execution() {
        let root = corpus_root();
        let config = XyceRunnerConfig {
            max_time_per_test_ms: 1,
            ..XyceRunnerConfig::default()
        };
        let runner = XyceTestRunner::new(&root, config);
        let deck = XyceDeck {
            path: root.join(OWNER_PATH),
            section: XyceDeckSection::Netlists,
            relative_path: OWNER_PATH.to_string(),
        };
        let error = runner
            .validate_bug806_oracle(
                &deck,
                Bug806Role::DataOwner,
                Instant::now() - Duration::from_secs(1),
            )
            .expect_err("expired BUG806 deadline must reject");
        assert!(error.contains("deadline"));
    }
}
