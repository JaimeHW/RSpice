use super::*;
use rspice_core::netlist::SourceSpec;
use std::io::Read as _;

const LABEL: &str = "ISSUE_565_566 STEP DATA PWL/table relation";
const FAMILY_DIRECTORY: &str = "Netlists/Certification_Tests/ISSUE_565_566";
const FAMILY_PREFIX: &str = "netlists/certification_tests/issue_565_566/";
const EXCLUSION_SOURCE: &str = "Netlists/Certification_Tests/ISSUE_565_566/exclude";
const OWNER_CONTRACT: &str = "issue565566_step_data_pwl_wrapper_owner";
const REFERENCE_CONTRACT: &str = "issue565566_step_data_table_reference";
const UPSTREAM_REGRESSION_COMMIT: &str = "d6e278e371ec2f3df1325dcff4552e585bc7ecc1";
const UPSTREAM_RELEASE_TAG: &str = "Release-7.10.0";
const HISTORICAL_RECORD_COUNT: usize = 8;
const HISTORICAL_RECORD_BYTES: usize = 1_946;
const HISTORICAL_RECORDS_SHA256: &str =
    "4cb9e0cf54c7e5999e7865bfe09f41893354341d1bc8b62ca98d38995a37c61b";
const HISTORICAL_RECORDS_BLAKE3: &str =
    "c771d302c873047c36ae979309cc96d2b1bef05e87d03acde8c18f2ed3870a84";
const HISTORICAL_ARTIFACTS: [(&str, usize, &str, &str); HISTORICAL_RECORD_COUNT] = [
    (
        "Netlists/Certification_Tests/ISSUE_565_566/CMakeLists.txt",
        1_584,
        "36c777b0e4e170903e5d16948e409af7ffaf1e396185a91d5e69c955673703bf",
        "c95849865040387df27ae71b7c5e656b324c7a4415aa0cd8f428ae9c45a6389e",
    ),
    (
        "Netlists/Certification_Tests/ISSUE_565_566/Manifest.txt",
        83,
        "4b13cba662040f48ef4e97868f022b666161a5ef65557c881cdd3541f22eac2d",
        "6eeabd9f53c46a951b7ec9cab6011292675ad82218319a0714c4da6a96c5bdea",
    ),
    (
        EXCLUSION_SOURCE,
        20,
        "25ee3584636cf671afb2e28d37fcaa8f09c814a6332f15319102fdec2b71b93c",
        "6bfc4517dd979ee8ca158308e3bb456081ba57187b0df94ce81533d197059099",
    ),
    (
        "Netlists/Certification_Tests/ISSUE_565_566/issue565_566.cir",
        224,
        "fa44216ccf8507d4ae071db7a983e567c9c4e859449ae3d914256471dda1bb22",
        "665c69ab3085f0a7980d63c183f45121364a94f42ab4cb93e85cc2a451a9ea33",
    ),
    (
        "Netlists/Certification_Tests/ISSUE_565_566/issue565_566.cir.sh",
        1_511,
        "332bd13d81e055a62a493214cc4896a75b805cc9bbe241942b701d18635aa0f9",
        "314932509f5e50f08f86e22ab4fb0617ba554334d96fbf159070ec1853a83e9d",
    ),
    (
        "Netlists/Certification_Tests/ISSUE_565_566/issue565_566ref.cir",
        231,
        "30b60cc0763182d13f995b756c34de314cf018547a49b83374959e4ee83bcb2c",
        "744b7f20d1b00fe7f9ad50cfb625c8e66243eb7ebfcf0a1920e6b4431dfad728",
    ),
    (
        "Netlists/Certification_Tests/ISSUE_565_566/tags",
        78,
        "f1475557a37439531fe2c617ab9fcc156a491c385780b6d365f8f5cb9b1dfd01",
        "50dc61025d2aee95f06252843bab426c3701fabd6d2810b1c5228a6905f69272",
    ),
    (
        "TestScripts/xyce_verify.pl",
        59_566,
        "6e5f84b1646b30d0e12879848d7653584b39472d640a14916ae8fda6e1df12b3",
        "5eadb6dab06ed3091ea114146bd4a574de83784f87be9843ad7b721b0a793665",
    ),
];

const RETAINED_RECORD_COUNT: usize = 2;
const RETAINED_RECORD_BYTES: usize = 304;
const RETAINED_RECORDS_SHA256: &str =
    "dcdd036af4dbb360c288532c868007d89186196788eb6b43294be326676f3968";
const RETAINED_RECORDS_BLAKE3: &str =
    "46b14a32169abec852b7212e5a3a0268773b2d0d6e553e539279e6b14d0c1183";
const RETAINED_ARTIFACTS: [(&str, usize, &str, &str); RETAINED_RECORD_COUNT] = [
    (
        "issue565_566.cir",
        224,
        "fa44216ccf8507d4ae071db7a983e567c9c4e859449ae3d914256471dda1bb22",
        "665c69ab3085f0a7980d63c183f45121364a94f42ab4cb93e85cc2a451a9ea33",
    ),
    (
        "issue565_566ref.cir",
        231,
        "30b60cc0763182d13f995b756c34de314cf018547a49b83374959e4ee83bcb2c",
        "744b7f20d1b00fe7f9ad50cfb625c8e66243eb7ebfcf0a1920e6b4431dfad728",
    ),
];

const STEP_VALUES: [Value; 3] = [-0.9, -0.72, -0.54];
const ANALYTIC_TIMES: [Value; 4] = [0.0, 0.2775e-9, 0.555e-9, 1.0];
// Release default PRN emits the independently accepted transient grid; TSTEP
// is not an OUTPUT resampling schedule. Bound each table by the suite's
// established Xyce oracle row ceiling and retain only one pair at a time.
const MAX_ROWS_PER_BATCH: usize = MAX_NATIVE_TRAN_ORACLE_STEPS as usize;
const MAX_DEFAULT_PRN_ROW_BYTES: usize = 64;
const MAX_STEPPED_PRN_BYTES: usize =
    STEP_VALUES.len() * MAX_ROWS_PER_BATCH * MAX_DEFAULT_PRN_ROW_BYTES + 128;

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub(super) enum Issue565566Role {
    PwlOwner,
    TableReference,
}

impl Issue565566Role {
    const ALL: [Self; 2] = [Self::PwlOwner, Self::TableReference];

    pub(super) fn for_record(record: &str) -> Option<Self> {
        let normalized = XyceTestRunner::normalize_manifest_key(record);
        Self::ALL
            .into_iter()
            .find(|role| role.record() == normalized)
    }

    pub(super) const fn contract(self) -> &'static str {
        match self {
            Self::PwlOwner => OWNER_CONTRACT,
            Self::TableReference => REFERENCE_CONTRACT,
        }
    }

    const fn path(self) -> &'static str {
        match self {
            Self::PwlOwner => "Netlists/Certification_Tests/ISSUE_565_566/issue565_566.cir",
            Self::TableReference => {
                "Netlists/Certification_Tests/ISSUE_565_566/issue565_566ref.cir"
            }
        }
    }

    const fn record(self) -> &'static str {
        match self {
            Self::PwlOwner => "netlists/certification_tests/issue_565_566/issue565_566.cir",
            Self::TableReference => {
                "netlists/certification_tests/issue_565_566/issue565_566ref.cir"
            }
        }
    }

    const fn file_name(self) -> &'static str {
        match self {
            Self::PwlOwner => "issue565_566.cir",
            Self::TableReference => "issue565_566ref.cir",
        }
    }
}

impl XyceTestRunner {
    pub(super) fn validate_issue565566_historical_provenance() -> Result<(), String> {
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
                "{LABEL} historical provenance changed: records={}, bytes={}, sha256={sha256}, blake3={content_blake3}",
                records.len(),
                stream.len()
            ));
        }
        Ok(())
    }

    fn validate_issue565566_directory(&self) -> Result<BTreeMap<String, Vec<u8>>, String> {
        let directory = self.root.join(FAMILY_DIRECTORY);
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
            let cap = expected_bytes
                .checked_mul(2)
                .and_then(|value| value.checked_add(3))
                .ok_or_else(|| format!("{LABEL} retained-size bound overflowed"))?;
            if metadata.len() > cap as u64 {
                return Err(format!("{LABEL} retained member {name:?} is oversized"));
            }
            let mut bytes = Vec::with_capacity((metadata.len() as usize).min(cap));
            fs::File::open(&path)
                .map_err(|error| format!("failed to open {LABEL} {name:?}: {error}"))?
                .take((cap + 1) as u64)
                .read_to_end(&mut bytes)
                .map_err(|error| format!("failed to read {LABEL} {name:?}: {error}"))?;
            if bytes.len() > cap {
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
        let sha256 = format!("{:x}", Sha256::digest(stream.as_bytes()));
        let content_blake3 = blake3::hash(stream.as_bytes()).to_hex().to_string();
        if records.len() != RETAINED_RECORD_COUNT
            || stream.len() != RETAINED_RECORD_BYTES
            || sha256 != RETAINED_RECORDS_SHA256
            || content_blake3 != RETAINED_RECORDS_BLAKE3
        {
            return Err(format!(
                "{LABEL} retained census changed: records={}, bytes={}, sha256={sha256}, blake3={content_blake3}",
                records.len(),
                stream.len()
            ));
        }
        Ok(observed)
    }

    fn validate_issue565566_provenance(
        &self,
        deck: &XyceDeck,
        role: Issue565566Role,
    ) -> Result<BTreeMap<String, Vec<u8>>, String> {
        Self::validate_issue565566_historical_provenance()?;
        if deck.section != XyceDeckSection::Netlists
            || Self::normalize_manifest_key(&deck.relative_path) != role.record()
            || Self::normalize_manifest_key(&self.relative_key(&deck.path)) != role.record()
            || !Self::same_path(&deck.path, &self.root.join(role.path()))
        {
            return Err(format!("recognized {LABEL} role is not canonical"));
        }
        let owners = Self::load_upstream_wrapper_decks(&self.root)
            .into_iter()
            .filter(|record| record.starts_with(FAMILY_PREFIX))
            .collect::<BTreeSet<_>>();
        if owners != BTreeSet::from([Issue565566Role::PwlOwner.record().to_string()]) {
            return Err(format!("{LABEL} wrapper ownership changed: {owners:?}"));
        }
        let exclusions = Self::load_upstream_exclusions(&self.root)
            .map_err(|error| format!("{LABEL} exclusion manifest is invalid: {error}"))?;
        if exclusions.contains_key(Issue565566Role::PwlOwner.record()) {
            return Err(format!("{LABEL} owner must not be excluded"));
        }
        let family = exclusions
            .iter()
            .filter(|(record, _)| record.starts_with(FAMILY_PREFIX))
            .collect::<BTreeMap<_, _>>();
        let row = family
            .get(&Issue565566Role::TableReference.record().to_string())
            .copied()
            .ok_or_else(|| format!("{LABEL} lost its reference qualification"))?;
        if family.len() != 1
            || row.source != EXCLUSION_SOURCE
            || !matches!(&row.disposition,
                XyceUpstreamExclusionDisposition::RspiceIndependentlyQualified { expected_contract }
                    if expected_contract == REFERENCE_CONTRACT)
        {
            return Err(format!("{LABEL} reference qualification changed"));
        }
        let members = self.validate_issue565566_directory()?;
        for member_role in Issue565566Role::ALL {
            self.reject_wrapper_output_artifacts(&self.root.join(member_role.path()))
                .map_err(|error| format!("{LABEL} {} {error}", member_role.file_name()))?;
        }
        let output = self
            .root
            .join("OutputData/Certification_Tests/ISSUE_565_566");
        match fs::symlink_metadata(&output) {
            Err(error) if error.kind() == std::io::ErrorKind::NotFound => {}
            Err(error) => return Err(format!("failed to inspect {LABEL} OutputData: {error}")),
            Ok(_) => return Err(format!("{LABEL} must not acquire invented numerical gold")),
        }
        Ok(members)
    }

    fn issue565566_plan(&self, role: Issue565566Role) -> Result<XyceStaticTranPlan, String> {
        let path = self.root.join(role.path());
        let plan = self.static_tran_plan_for_path_with_purpose(
            &path,
            match role {
                Issue565566Role::PwlOwner => {
                    XyceStaticTranPlanPurpose::GeneratedReferenceRelationalFamily
                }
                Issue565566Role::TableReference => XyceStaticTranPlanPurpose::RelationalFamily,
            },
        )?;
        Self::validate_issue565566_plan(role, &plan)?;
        Ok(plan)
    }

    fn validate_issue565566_plan(
        role: Issue565566Role,
        plan: &XyceStaticTranPlan,
    ) -> Result<(), String> {
        let probes = plan
            .print
            .as_ref()
            .map(|print| {
                print
                    .probes
                    .iter()
                    .map(|probe| Self::normalize_probe(probe))
                    .collect::<Vec<_>>()
            })
            .unwrap_or_default();
        if plan.deck_path.file_name().and_then(|name| name.to_str()) != Some(role.file_name())
            || !matches!(plan.oracle, XyceStaticTranOracle::None)
            || plan.contract
                != match role {
                    Issue565566Role::PwlOwner => XyceStaticTranContract::WrapperStatic,
                    Issue565566Role::TableReference => XyceStaticTranContract::PlainStatic,
                }
            || plan.comparison_mode != XyceStaticTranComparisonMode::Pointwise
            || plan.output_override
            || plan.timeint_conststep
            || plan.wrapper_tolerance.is_some()
            || plan.steps.len() != 1
            || probes != ["v(*)"]
            || plan.tran.step.to_bits() != 1.0e-6f64.to_bits()
            || plan.tran.stop.to_bits() != 1.0f64.to_bits()
            || plan.tran.start.is_some()
            || plan.tran.max_step.is_some()
            || plan.tran.uic
        {
            return Err(format!(
                "{LABEL} {} plan changed: {plan:?}",
                role.file_name()
            ));
        }
        let step = &plan.steps[0];
        if step.target != StepTarget::Param
            || !step.name.eq_ignore_ascii_case("psw")
            || step.param_name.is_some()
            || !matches!(&step.sweep, StepSweep::Data { table_name }
                if table_name.eq_ignore_ascii_case("psw"))
        {
            return Err(format!("{LABEL} typed STEP DATA plan changed: {step:?}"));
        }
        Ok(())
    }

    fn issue565566_nodes_match(actual: &[String], expected: &[&str]) -> bool {
        actual.len() == expected.len()
            && actual
                .iter()
                .zip(expected)
                .all(|(actual, expected)| actual.eq_ignore_ascii_case(expected))
    }

    fn issue565566_table_expression_matches(expression: &str) -> bool {
        expression
            .trim()
            .trim_start_matches('{')
            .trim_end_matches('}')
            .chars()
            .filter(|character| !character.is_whitespace())
            .collect::<String>()
            .eq_ignore_ascii_case("table(time,0,par0,0.555n,par1)")
    }

    fn validate_issue565566_netlist(
        role: Issue565566Role,
        netlist: &Netlist,
    ) -> Result<(), String> {
        if netlist.elements.len() != 2
            || netlist.data_tables.len() != 1
            || netlist.output_requests.len() != 1
            || netlist.analyses.len() != 2
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
            || !matches!(&netlist.analyses[0], AnalysisCommand::Step(step)
                if step.target == StepTarget::Param
                    && step.name.eq_ignore_ascii_case("psw")
                    && step.param_name.is_none()
                    && matches!(&step.sweep, StepSweep::Data { table_name }
                        if table_name.eq_ignore_ascii_case("psw")))
            || !matches!(&netlist.analyses[1], AnalysisCommand::Tran {
                step, stop, start: None, max_step: None, uic: false,
            } if step.to_bits() == 1.0e-6f64.to_bits()
                && stop.to_bits() == 1.0f64.to_bits())
        {
            return Err(format!(
                "{LABEL} {} typed envelope changed",
                role.file_name()
            ));
        }
        let params = netlist
            .params
            .all_params()
            .into_iter()
            .map(|(name, value)| (name.to_ascii_lowercase(), value.to_bits()))
            .collect::<BTreeMap<_, _>>();
        if params
            != BTreeMap::from([
                ("par0".to_string(), (-0.1f64).to_bits()),
                ("par1".to_string(), (-0.9f64).to_bits()),
            ])
        {
            return Err(format!("{LABEL} parameter defaults changed: {params:?}"));
        }
        let table = &netlist.data_tables[0];
        if !table.name.eq_ignore_ascii_case("psw")
            || table.params.len() != 1
            || !table.params[0].eq_ignore_ascii_case("par1")
            || table.rows.len() != STEP_VALUES.len()
            || table
                .rows
                .iter()
                .zip(STEP_VALUES)
                .any(|(row, expected)| row.len() != 1 || row[0].to_bits() != expected.to_bits())
        {
            return Err(format!("{LABEL} DATA table changed: {table:?}"));
        }
        let elements = netlist
            .elements
            .iter()
            .map(|element| (element.name.to_ascii_lowercase(), element))
            .collect::<BTreeMap<_, _>>();
        if elements.keys().map(String::as_str).collect::<BTreeSet<_>>()
            != BTreeSet::from([
                "rtest",
                match role {
                    Issue565566Role::PwlOwner => "v_ivcy",
                    Issue565566Role::TableReference => "b_ivcy",
                },
            ])
        {
            return Err(format!(
                "{LABEL} {} element inventory changed",
                role.file_name()
            ));
        }
        let resistor = elements["rtest"];
        if resistor.provenance != ElementProvenance::Authored
            || !Self::issue565566_nodes_match(&resistor.nodes, &["Y", "0"])
            || !matches!(&resistor.kind, ElementKind::Resistor {
                value, value_expr: None, model: None, instance_params, deferred_params,
            } if value.to_bits() == 1.0f64.to_bits()
                && instance_params.is_empty() && deferred_params.is_empty())
        {
            return Err(format!("{LABEL} resistor changed: {resistor:?}"));
        }
        let source = elements[match role {
            Issue565566Role::PwlOwner => "v_ivcy",
            Issue565566Role::TableReference => "b_ivcy",
        }];
        if source.provenance != ElementProvenance::Authored
            || !Self::issue565566_nodes_match(&source.nodes, &["Y", "0"])
        {
            return Err(format!(
                "{LABEL} {} source topology changed",
                role.file_name()
            ));
        }
        match (role, &source.kind) {
            (
                Issue565566Role::PwlOwner,
                ElementKind::VoltageSource(SourceSpec::Pwl {
                    points,
                    delay,
                    repeat_from,
                }),
            ) if delay.to_bits() == 0.0f64.to_bits()
                && repeat_from.is_none()
                && points.len() == 2
                && points[0].0.to_bits() == 0.0f64.to_bits()
                && points[0].1.to_bits() == (-0.1f64).to_bits()
                && points[1].0.to_bits() == 0.555e-9f64.to_bits()
                && points[1].1.to_bits() == (-0.9f64).to_bits() => {}
            (
                Issue565566Role::TableReference,
                ElementKind::BehavioralVoltage {
                    expression,
                    tc1,
                    tc2,
                    multiplicity,
                },
            ) if Self::issue565566_table_expression_matches(expression)
                && tc1.to_bits() == 0.0f64.to_bits()
                && tc2.to_bits() == 0.0f64.to_bits()
                && multiplicity.value.to_bits() == 1.0f64.to_bits()
                && multiplicity.value_expr.is_none()
                && !multiplicity.given => {}
            _ => {
                return Err(format!(
                    "{LABEL} {} source semantics changed: {source:?}",
                    role.file_name()
                ));
            }
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
            || !request.dependencies[0].symbol.eq_ignore_ascii_case("*")
        {
            return Err(format!("{LABEL} typed V(*) PRINT changed: {request:?}"));
        }
        Ok(())
    }

    fn validate_issue565566_materialization(
        role: Issue565566Role,
        run: &XyceStepRun,
        index: usize,
    ) -> Result<(), String> {
        let [coordinate] = run.step_values.as_slice() else {
            return Err(format!("{LABEL} step {index} lost its DATA coordinate"));
        };
        if coordinate.to_bits() != (index as Value).to_bits() {
            return Err(format!("{LABEL} DATA row order changed at step {index}"));
        }
        let params = run
            .netlist
            .params
            .all_params()
            .into_iter()
            .map(|(name, value)| (name.to_ascii_lowercase(), value))
            .collect::<BTreeMap<_, _>>();
        if params.get("par0").map(|value| value.to_bits()) != Some((-0.1f64).to_bits())
            || params.get("par1").map(|value| value.to_bits()) != Some(STEP_VALUES[index].to_bits())
        {
            return Err(format!(
                "{LABEL} step {index} did not bind par1 exactly: {params:?}"
            ));
        }
        let source_name = match role {
            Issue565566Role::PwlOwner => "v_ivcy",
            Issue565566Role::TableReference => "b_ivcy",
        };
        let source = run
            .netlist
            .elements
            .iter()
            .find(|element| element.name.eq_ignore_ascii_case(source_name))
            .ok_or_else(|| format!("{LABEL} step {index} lost {source_name}"))?;
        if source.provenance != ElementProvenance::Authored
            || !Self::issue565566_nodes_match(&source.nodes, &["Y", "0"])
        {
            return Err(format!(
                "{LABEL} step {index} source terminals/provenance changed: {source:?}"
            ));
        }
        match (role, &source.kind) {
            (
                Issue565566Role::PwlOwner,
                ElementKind::VoltageSource(SourceSpec::Pwl { points, .. }),
            ) if points.len() == 2 && points[1].1.to_bits() == STEP_VALUES[index].to_bits() => {}
            (
                Issue565566Role::TableReference,
                ElementKind::BehavioralVoltage {
                    expression,
                    tc1,
                    tc2,
                    multiplicity,
                },
            ) if Self::issue565566_table_expression_matches(expression)
                && tc1.to_bits() == 0.0f64.to_bits()
                && tc2.to_bits() == 0.0f64.to_bits()
                && multiplicity.value.to_bits() == 1.0f64.to_bits()
                && multiplicity.value_expr.is_none()
                && !multiplicity.given => {}
            _ => {
                return Err(format!(
                    "{LABEL} step {index} source binding changed: {source:?}"
                ));
            }
        }
        Ok(())
    }

    fn issue565566_projected_prn_tables(
        plan: &XyceStaticTranPlan,
        netlist: &Netlist,
        result: &TransientResult,
        abort: &dyn AbortSignal,
    ) -> Result<(XycePrnTable, XycePrnTable), String> {
        Self::validate_transient_result_time_grid(result)?;
        let projected = rspice_core::analysis::evaluate_tran_output_requests_with_abort(
            netlist,
            result,
            rspice_core::ResourceLimits::default(),
            abort,
        )
        .map_err(|error| format!("{LABEL} production V(*) projection failed: {error}"))?;
        let [(name, physical_type, values)] = projected.as_slice() else {
            return Err(format!(
                "{LABEL} V(*) projection produced {} column(s), expected one: {projected:?}",
                projected.len()
            ));
        };
        if name != "V(Y)" || *physical_type != "voltage" || values.len() != result.time.len() {
            return Err(format!(
                "{LABEL} V(*) projection changed its exact column/order/shape: {projected:?}"
            ));
        }

        let time_scale = Self::tran_print_time_scale_factor(&plan.source)?;
        let output_times = Self::xyce_verify_transient_output_times(plan, netlist, result)?;
        let mut cursor = 0usize;
        let mut rows = Vec::with_capacity(output_times.len());
        for (index, time) in output_times.into_iter().enumerate() {
            while result
                .time
                .get(cursor)
                .is_some_and(|candidate| candidate.to_bits() != time.to_bits())
            {
                cursor += 1;
            }
            let value = values
                .get(cursor)
                .copied()
                .ok_or_else(|| format!("{LABEL} projected output lost serialized TIME={time:e}"))?;
            rows.push(vec![index as Value, time * time_scale, value]);
            cursor += 1;
        }
        let table = XycePrnTable {
            columns: ["Index", "TIME", "V(Y)"].map(str::to_string).to_vec(),
            rows,
        };

        let mut analytic_rows = Vec::with_capacity(ANALYTIC_TIMES.len());
        for (index, time) in ANALYTIC_TIMES.into_iter().enumerate() {
            let value = Self::interpolate_transient_waveform_at(&result.time, values, time)?;
            analytic_rows.push(vec![index as Value, time * time_scale, value]);
        }
        let analytic = XycePrnTable {
            columns: ["Index", "TIME", "V(Y)"].map(str::to_string).to_vec(),
            rows: analytic_rows,
        };
        Ok((table, analytic))
    }

    fn validate_issue565566_table(
        role: Issue565566Role,
        step: usize,
        table: &XycePrnTable,
    ) -> Result<(), String> {
        if table.columns != ["Index", "TIME", "V(Y)"]
            || table.rows.len() < 2
            || table.rows.len() > MAX_ROWS_PER_BATCH
        {
            return Err(format!(
                "{LABEL} {} step {step} table shape changed: columns={:?}, rows={}",
                role.file_name(),
                table.columns,
                table.rows.len()
            ));
        }
        for (index, row) in table.rows.iter().enumerate() {
            if row.len() != 3
                || row.iter().any(|value| !value.is_finite())
                || row[0].to_bits() != (index as Value).to_bits()
                || row[1] < 0.0
                || row[1] > 1.0
            {
                return Err(format!(
                    "{LABEL} {} step {step} malformed row {index}",
                    role.file_name()
                ));
            }
        }
        if table.rows.windows(2).any(|pair| pair[0][1] >= pair[1][1])
            || table
                .rows
                .first()
                .is_none_or(|row| row[1].to_bits() != 0.0f64.to_bits())
            || table
                .rows
                .last()
                .is_none_or(|row| row[1].to_bits() != 1.0f64.to_bits())
        {
            return Err(format!(
                "{LABEL} {} step {step} time grid changed",
                role.file_name()
            ));
        }
        Ok(())
    }

    fn validate_issue565566_analytic_samples(
        role: Issue565566Role,
        step: usize,
        table: &XycePrnTable,
    ) -> Result<(), String> {
        if table.columns != ["Index", "TIME", "V(Y)"] || table.rows.len() != ANALYTIC_TIMES.len() {
            return Err(format!(
                "{LABEL} {} step {step} analytic sample shape changed",
                role.file_name()
            ));
        }
        let endpoint = STEP_VALUES[step];
        let expected_values = [-0.1, (-0.1 + endpoint) * 0.5, endpoint, endpoint];
        for (index, ((row, time), expected)) in table
            .rows
            .iter()
            .zip(ANALYTIC_TIMES)
            .zip(expected_values)
            .enumerate()
        {
            if row.len() != 3
                || row[0].to_bits() != (index as Value).to_bits()
                || row[1].to_bits() != time.to_bits()
                || Self::xyce_default_prn_text(row[2])? != Self::xyce_default_prn_text(expected)?
            {
                return Err(format!(
                    "{LABEL} {} step {step} failed analytic sample {index}: row={row:?}, expected TIME={time:e}, V(Y)={expected:e}",
                    role.file_name()
                ));
            }
        }
        Ok(())
    }

    fn compare_issue565566_relation(
        &self,
        step: usize,
        good: &XycePrnTable,
        test: &XycePrnTable,
    ) -> Result<(), String> {
        let exact = self.compare_serialized_default_prn_tables(good, test)?;
        if !exact.is_empty() {
            return Err(format!(
                "{LABEL} step {step} failed complete default-PRN byte equality: {exact:?}"
            ));
        }
        let directional = self.compare_xyce_verify_transient_tables_with_uniform_tolerance(
            good,
            test,
            XyceVerifyTransientTolerance::release_7_10_default(),
            XYCE_DEFAULT_PRN_SCIENTIFIC_PRECISION,
        )?;
        if !directional.is_empty() {
            return Err(format!(
                "{LABEL} step {step} failed directional Release-7.10 xyce_verify: {directional:?}"
            ));
        }
        Ok(())
    }

    fn extend_issue565566_prn_bytes(output: &mut Vec<u8>, bytes: &[u8]) -> Result<(), String> {
        let requested = output
            .len()
            .checked_add(bytes.len())
            .ok_or_else(|| format!("{LABEL} stepped PRN size overflowed"))?;
        if requested > MAX_STEPPED_PRN_BYTES {
            return Err(format!(
                "{LABEL} stepped PRN exceeds its {MAX_STEPPED_PRN_BYTES}-byte envelope"
            ));
        }
        output
            .try_reserve(bytes.len())
            .map_err(|error| format!("{LABEL} stepped PRN allocation failed: {error}"))?;
        output.extend_from_slice(bytes);
        Ok(())
    }

    fn append_issue565566_stepped_default_prn_bytes(
        output: &mut Vec<u8>,
        step: usize,
        table: &XycePrnTable,
        abort: &dyn AbortSignal,
    ) -> Result<(), String> {
        if step >= STEP_VALUES.len() || table.columns != ["Index", "TIME", "V(Y)"] {
            return Err(format!(
                "{LABEL} cannot serialize step {step} with columns {:?}",
                table.columns
            ));
        }
        if step == 0 {
            Self::extend_issue565566_prn_bytes(output, b"Index TIME V(Y)\n")?;
        }
        for (row_index, row) in table.rows.iter().enumerate() {
            if abort.is_aborted() {
                return Err(format!(
                    "{LABEL} exceeded its shared deadline during PRN output"
                ));
            }
            if row.len() != 3 || row[0].to_bits() != (row_index as Value).to_bits() {
                return Err(format!(
                    "{LABEL} step {step} cannot serialize its complete default PRN"
                ));
            }
            let line = format!(
                "{row_index} {} {}\n",
                Self::xyce_default_prn_text(row[1])?,
                Self::xyce_default_prn_text(row[2])?
            );
            if line.len() > MAX_DEFAULT_PRN_ROW_BYTES {
                return Err(format!(
                    "{LABEL} step {step} row {row_index} exceeds its canonical byte bound"
                ));
            }
            Self::extend_issue565566_prn_bytes(output, line.as_bytes())?;
        }
        if step + 1 == STEP_VALUES.len() {
            Self::extend_issue565566_prn_bytes(output, b"End of Xyce(TM) Parameter Sweep\n")?;
        }
        Ok(())
    }

    fn validate_issue565566_stepped_default_prn_bytes(
        bytes: &[u8],
        abort: &dyn AbortSignal,
    ) -> Result<(), String> {
        if bytes.len() > MAX_STEPPED_PRN_BYTES || bytes.contains(&b'\r') {
            return Err(format!("{LABEL} stepped PRN byte envelope changed"));
        }
        let text = std::str::from_utf8(bytes)
            .map_err(|error| format!("{LABEL} stepped PRN is not UTF-8: {error}"))?;
        let body = text
            .strip_suffix('\n')
            .ok_or_else(|| format!("{LABEL} stepped PRN lost its final newline"))?;
        let lines = body.split('\n').collect::<Vec<_>>();
        if lines.first().copied() != Some("Index TIME V(Y)")
            || lines.last().copied() != Some("End of Xyce(TM) Parameter Sweep")
            || lines.len() < 5
        {
            return Err(format!("{LABEL} stepped PRN header/footer changed"));
        }
        let mut segments = 0usize;
        let mut expected_index = 0usize;
        for (line_number, line) in lines[1..lines.len() - 1].iter().enumerate() {
            if abort.is_aborted() {
                return Err(format!(
                    "{LABEL} exceeded its shared deadline during PRN validation"
                ));
            }
            let fields = line.split(' ').collect::<Vec<_>>();
            if fields.len() != 3 || fields.iter().any(|field| field.is_empty()) {
                return Err(format!(
                    "{LABEL} stepped PRN line {} lost canonical field formatting",
                    line_number + 2
                ));
            }
            let index = fields[0].parse::<usize>().map_err(|error| {
                format!(
                    "{LABEL} stepped PRN has invalid Index '{}': {error}",
                    fields[0]
                )
            })?;
            if index == 0 {
                segments += 1;
                expected_index = 0;
            }
            if index != expected_index {
                return Err(format!(
                    "{LABEL} stepped PRN Index sequence changed at line {}: found {index}, expected {expected_index}",
                    line_number + 2
                ));
            }
            expected_index += 1;
            let time = fields[1].parse::<Value>().map_err(|error| {
                format!(
                    "{LABEL} stepped PRN has invalid TIME '{}': {error}",
                    fields[1]
                )
            })?;
            let value = fields[2].parse::<Value>().map_err(|error| {
                format!(
                    "{LABEL} stepped PRN has invalid V(Y) '{}': {error}",
                    fields[2]
                )
            })?;
            let canonical = format!(
                "{index} {} {}",
                Self::xyce_default_prn_text(time)?,
                Self::xyce_default_prn_text(value)?
            );
            if *line != canonical {
                return Err(format!(
                    "{LABEL} stepped PRN line {} is not canonical default serialization",
                    line_number + 2
                ));
            }
        }
        if segments != STEP_VALUES.len() {
            return Err(format!(
                "{LABEL} stepped PRN has {segments} segment(s), expected {}",
                STEP_VALUES.len()
            ));
        }
        Ok(())
    }

    fn compare_issue565566_complete_prn_bytes(
        owner: &[u8],
        reference: &[u8],
        abort: &dyn AbortSignal,
    ) -> Result<(), String> {
        Self::validate_issue565566_stepped_default_prn_bytes(owner, abort)?;
        Self::validate_issue565566_stepped_default_prn_bytes(reference, abort)?;
        if owner != reference {
            let offset = owner
                .iter()
                .zip(reference)
                .position(|(owner, reference)| owner != reference)
                .unwrap_or_else(|| owner.len().min(reference.len()));
            return Err(format!(
                "{LABEL} complete three-segment default PRNs differ at byte {offset}: owner={} bytes, reference={} bytes",
                owner.len(),
                reference.len()
            ));
        }
        Ok(())
    }

    pub(super) fn validate_issue565566_oracle(
        &self,
        deck: &XyceDeck,
        role: Issue565566Role,
        start: Instant,
    ) -> Result<(), String> {
        let abort = DeadlineAbort::new(start, self.config.max_time_per_test_ms.max(1));
        if abort.is_aborted() {
            return Err(format!("{LABEL} shared deadline expired before provenance"));
        }
        let members = self.validate_issue565566_provenance(deck, role)?;
        let mut plans = BTreeMap::new();
        let mut runs = BTreeMap::new();
        for member_role in Issue565566Role::ALL {
            let plan = self.issue565566_plan(member_role)?;
            let bytes = members
                .get(&member_role.file_name().to_ascii_lowercase())
                .ok_or_else(|| format!("{LABEL} lost {}", member_role.file_name()))?;
            if plan.source.as_bytes() != bytes.as_slice() {
                return Err(format!(
                    "{LABEL} {} source changed between reads",
                    member_role.file_name()
                ));
            }
            let netlist =
                Self::parse_xyce_netlist(&plan.source, &plan.deck_path).map_err(|error| {
                    format!("{LABEL} {} parse failed: {error}", member_role.file_name())
                })?;
            Self::validate_issue565566_netlist(member_role, &netlist)?;
            let expanded = Self::nested_step_runs_for_commands_with_limits_and_abort(
                &self.create_dc_engine(),
                &netlist,
                &plan.steps,
                StepPlanLimits::new(3, 1, 1, 3),
                &abort,
            )
            .map_err(|error| {
                format!(
                    "{LABEL} {} STEP expansion failed: {error}",
                    member_role.file_name()
                )
            })?;
            if expanded.len() != STEP_VALUES.len() {
                return Err(format!(
                    "{LABEL} {} expanded to {} runs",
                    member_role.file_name(),
                    expanded.len()
                ));
            }
            for (index, run) in expanded.iter().enumerate() {
                Self::validate_issue565566_materialization(member_role, run, index)?;
            }
            plans.insert(member_role, plan);
            runs.insert(member_role, expanded);
        }

        let mut owner_prn = Vec::new();
        let mut reference_prn = Vec::new();
        for (step, _) in STEP_VALUES.iter().enumerate() {
            let mut pair = BTreeMap::new();
            for member_role in Issue565566Role::ALL {
                let plan = &plans[&member_role];
                let run = &runs[&member_role][step];
                let result = self
                    .run_transient_family_netlist(plan, &run.netlist, start, None, None)
                    .map_err(|error| match error {
                        SimulationError::Aborted => format!("{LABEL} exceeded its shared deadline"),
                        other => format!(
                            "{LABEL} {} step {step} execution failed: {other}",
                            member_role.file_name()
                        ),
                    })?;
                let (table, analytic) =
                    Self::issue565566_projected_prn_tables(plan, &run.netlist, &result, &abort)?;
                Self::validate_issue565566_table(member_role, step, &table)?;
                Self::validate_issue565566_analytic_samples(member_role, step, &analytic)?;
                pair.insert(member_role, table);
                if abort.is_aborted() {
                    return Err(format!("{LABEL} exceeded its shared deadline between runs"));
                }
            }
            let owner = &pair[&Issue565566Role::PwlOwner];
            let reference = &pair[&Issue565566Role::TableReference];
            self.compare_issue565566_relation(step, owner, reference)?;
            Self::append_issue565566_stepped_default_prn_bytes(
                &mut owner_prn,
                step,
                owner,
                &abort,
            )?;
            Self::append_issue565566_stepped_default_prn_bytes(
                &mut reference_prn,
                step,
                reference,
                &abort,
            )?;
            // The table pair drops here before the next DATA coordinate is
            // simulated; only the bounded canonical byte streams are retained.
        }
        Self::compare_issue565566_complete_prn_bytes(&owner_prn, &reference_prn, &abort)?;
        self.validate_issue565566_provenance(deck, role)?;
        if abort.is_aborted() {
            return Err(format!(
                "{LABEL} exceeded its shared deadline after provenance"
            ));
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

    fn deck(root: &Path, role: Issue565566Role) -> XyceDeck {
        XyceDeck {
            path: root.join(role.path()),
            section: XyceDeckSection::Netlists,
            relative_path: role.path().to_string(),
        }
    }

    fn fixture(label: &str) -> (tempfile::TempDir, XyceDeck, XyceTestRunner) {
        let temporary = tempfile::Builder::new()
            .prefix(&format!("rspice-xyce-issue565566-{label}-"))
            .tempdir()
            .expect("create ISSUE565566 fixture");
        let root = temporary.path();
        let family = root.join(FAMILY_DIRECTORY);
        fs::create_dir_all(&family).expect("create family");
        let canonical = corpus_root().join(FAMILY_DIRECTORY);
        for (name, ..) in RETAINED_ARTIFACTS {
            fs::copy(canonical.join(name), family.join(name)).expect("copy retained member");
        }
        fs::write(
            root.join(HARNESS_MANIFEST_FILE),
            format!(
                "{}\t{REQUIRES_UPSTREAM_WRAPPER_CONTRACT}\n",
                Issue565566Role::PwlOwner.path()
            ),
        )
        .expect("write harness manifest");
        fs::write(
            root.join(UPSTREAM_EXCLUSIONS_MANIFEST_FILE),
            format!(
                "schema_version\t{UPSTREAM_EXCLUSIONS_SCHEMA_VERSION}\nsource_commit\t{UPSTREAM_EXCLUSIONS_SOURCE_COMMIT}\nsource_netlists_tree\t{UPSTREAM_EXCLUSIONS_SOURCE_NETLISTS_TREE}\n{}\t{EXCLUSION_SOURCE}\t{RSPICE_INDEPENDENTLY_QUALIFIED_DISPOSITION}\t{REFERENCE_CONTRACT}\n",
                Issue565566Role::TableReference.path()
            ),
        )
        .expect("write exclusion manifest");
        let owner = deck(root, Issue565566Role::PwlOwner);
        let runner = XyceTestRunner::new(root, XyceRunnerConfig::default());
        (temporary, owner, runner)
    }

    fn assert_provenance_mutation_rejected(label: &str, mutate: impl FnOnce(&Path)) {
        let (temporary, owner, runner) = fixture(label);
        runner
            .validate_issue565566_provenance(&owner, Issue565566Role::PwlOwner)
            .expect("canonical fixture");
        mutate(temporary.path());
        assert!(
            runner
                .validate_issue565566_provenance(&owner, Issue565566Role::PwlOwner)
                .is_err(),
            "{label} mutation must fail closed"
        );
    }

    fn stream_table(value: Value) -> XycePrnTable {
        XycePrnTable {
            columns: ["Index", "TIME", "V(Y)"].map(str::to_string).to_vec(),
            rows: vec![vec![0.0, 0.0, -0.1], vec![1.0, 1.0, value]],
        }
    }

    fn stepped_stream(tables: &[XycePrnTable; 3]) -> Result<Vec<u8>, String> {
        let mut output = Vec::new();
        for (step, table) in tables.iter().enumerate() {
            XyceTestRunner::append_issue565566_stepped_default_prn_bytes(
                &mut output,
                step,
                table,
                &rspice_core::abort_signal::NoAbort,
            )?;
        }
        Ok(output)
    }

    #[test]
    fn issue565566_historical_provenance_is_exact() {
        XyceTestRunner::validate_issue565566_historical_provenance()
            .expect("exact Release provenance");
    }

    #[test]
    fn issue565566_both_roles_execute_all_three_data_rows() {
        let root = corpus_root();
        let runner = XyceTestRunner::new(&root, XyceRunnerConfig::default());
        for role in Issue565566Role::ALL {
            runner
                .validate_issue565566_oracle(&deck(&root, role), role, Instant::now())
                .expect("execute strict ISSUE565566 relation");
        }
    }

    #[test]
    fn issue565566_typed_contract_rejects_data_topology_and_source_drift() {
        for role in Issue565566Role::ALL {
            let source = fs::read_to_string(corpus_root().join(role.path())).expect("read source");
            let parsed = XyceTestRunner::parse_xyce_netlist(&source, Path::new(role.path()))
                .expect("parse source");
            XyceTestRunner::validate_issue565566_netlist(role, &parsed)
                .expect("canonical typed contract");
            let mutations = match role {
                Issue565566Role::PwlOwner => vec![
                    source.replace("0.555n 'par1'", "0.556n 'par1'"),
                    source.replace("Rtest Y 0 1.0", "Rtest Y 0 2.0"),
                    source.replace("+ -0.72", "+ -0.71"),
                ],
                Issue565566Role::TableReference => vec![
                    source.replace("0.555n,par1", "0.556n,par1"),
                    source.replace("B_ivcY Y 0", "B_ivcY Y X"),
                    source.replace("+ -0.72", "+ -0.71"),
                ],
            };
            for mutation in mutations {
                let parsed = XyceTestRunner::parse_xyce_netlist(&mutation, Path::new(role.path()))
                    .expect("mutation remains parseable");
                assert!(XyceTestRunner::validate_issue565566_netlist(role, &parsed).is_err());
            }
        }
    }

    #[test]
    fn issue565566_materialized_table_rejects_expression_terminal_and_multiplicity_drift() {
        let role = Issue565566Role::TableReference;
        let source = fs::read_to_string(corpus_root().join(role.path())).expect("read source");
        let netlist = XyceTestRunner::parse_xyce_netlist(&source, Path::new(role.path()))
            .expect("parse source");
        let canonical = XyceStepRun {
            step_values: vec![0.0],
            netlist,
        };
        XyceTestRunner::validate_issue565566_materialization(role, &canonical, 0)
            .expect("canonical materialization");

        let source_index = canonical
            .netlist
            .elements
            .iter()
            .position(|element| element.name.eq_ignore_ascii_case("b_ivcy"))
            .expect("behavioral source");
        let mut wrong_expression = canonical.clone();
        let ElementKind::BehavioralVoltage { expression, .. } =
            &mut wrong_expression.netlist.elements[source_index].kind
        else {
            panic!("behavioral source kind");
        };
        *expression = "table(time,0,par0,0.556n,par1)".to_string();
        assert!(
            XyceTestRunner::validate_issue565566_materialization(role, &wrong_expression, 0)
                .is_err()
        );

        let mut wrong_terminal = canonical.clone();
        wrong_terminal.netlist.elements[source_index].nodes[0] = "X".to_string();
        assert!(
            XyceTestRunner::validate_issue565566_materialization(role, &wrong_terminal, 0).is_err()
        );

        let mut wrong_multiplicity = canonical;
        let ElementKind::BehavioralVoltage { multiplicity, .. } =
            &mut wrong_multiplicity.netlist.elements[source_index].kind
        else {
            panic!("behavioral source kind");
        };
        multiplicity.value = 2.0;
        multiplicity.given = true;
        assert!(
            XyceTestRunner::validate_issue565566_materialization(role, &wrong_multiplicity, 0)
                .is_err()
        );
    }

    #[test]
    fn issue565566_analytic_gate_rejects_midpoint_and_plateau_drift() {
        let canonical = XycePrnTable {
            columns: ["Index", "TIME", "V(Y)"].map(str::to_string).to_vec(),
            rows: ANALYTIC_TIMES
                .into_iter()
                .zip([-0.1, -0.5, -0.9, -0.9])
                .enumerate()
                .map(|(index, (time, value))| vec![index as Value, time, value])
                .collect(),
        };
        XyceTestRunner::validate_issue565566_analytic_samples(
            Issue565566Role::PwlOwner,
            0,
            &canonical,
        )
        .expect("canonical analytic samples");
        for (row, value) in [(1, -0.49), (3, -0.89)] {
            let mut wrong = canonical.clone();
            wrong.rows[row][2] = value;
            assert!(
                XyceTestRunner::validate_issue565566_analytic_samples(
                    Issue565566Role::PwlOwner,
                    0,
                    &wrong,
                )
                .is_err()
            );
        }
    }

    #[test]
    fn issue565566_relation_requires_exact_prn_and_directional_semantics() {
        let runner = XyceTestRunner::new(corpus_root(), XyceRunnerConfig::default());
        let table = XycePrnTable {
            columns: ["Index", "TIME", "V(Y)"].map(str::to_string).to_vec(),
            rows: vec![vec![0.0, 0.0, -0.1], vec![1.0, 1.0, -0.9]],
        };
        runner
            .compare_issue565566_relation(0, &table, &table)
            .expect("identical relation");
        let mut wrong = table.clone();
        wrong.rows[1][2] = -0.8;
        assert!(
            runner
                .compare_issue565566_relation(0, &table, &wrong)
                .is_err()
        );
    }

    #[test]
    fn issue565566_complete_prn_bytes_bind_framing_segments_and_step_order() {
        let tables = [stream_table(-0.9), stream_table(-0.72), stream_table(-0.54)];
        let canonical = stepped_stream(&tables).expect("serialize canonical stepped PRN");
        XyceTestRunner::compare_issue565566_complete_prn_bytes(
            &canonical,
            &canonical,
            &rspice_core::abort_signal::NoAbort,
        )
        .expect("canonical stream equality");

        let mut wrong_header = canonical.clone();
        wrong_header[0] = b'X';
        assert!(
            XyceTestRunner::compare_issue565566_complete_prn_bytes(
                &canonical,
                &wrong_header,
                &rspice_core::abort_signal::NoAbort,
            )
            .is_err()
        );
        let mut wrong_footer = canonical.clone();
        let footer = b"End of Xyce(TM) Parameter Sweep";
        let footer_offset = wrong_footer
            .windows(footer.len())
            .position(|window| window == footer)
            .expect("footer");
        wrong_footer[footer_offset] = b'X';
        assert!(
            XyceTestRunner::compare_issue565566_complete_prn_bytes(
                &canonical,
                &wrong_footer,
                &rspice_core::abort_signal::NoAbort,
            )
            .is_err()
        );

        let reordered = stepped_stream(&[tables[1].clone(), tables[0].clone(), tables[2].clone()])
            .expect("serialize reordered steps");
        assert!(
            XyceTestRunner::compare_issue565566_complete_prn_bytes(
                &canonical,
                &reordered,
                &rspice_core::abort_signal::NoAbort,
            )
            .is_err()
        );

        let mut missing_segment = Vec::new();
        XyceTestRunner::append_issue565566_stepped_default_prn_bytes(
            &mut missing_segment,
            0,
            &tables[0],
            &rspice_core::abort_signal::NoAbort,
        )
        .expect("first segment");
        XyceTestRunner::append_issue565566_stepped_default_prn_bytes(
            &mut missing_segment,
            2,
            &tables[2],
            &rspice_core::abort_signal::NoAbort,
        )
        .expect("last segment");
        assert!(
            XyceTestRunner::validate_issue565566_stepped_default_prn_bytes(
                &missing_segment,
                &rspice_core::abort_signal::NoAbort,
            )
            .is_err()
        );

        let mut stepnum = tables[0].clone();
        stepnum.columns[0] = "STEPNUM".to_string();
        assert!(
            XyceTestRunner::append_issue565566_stepped_default_prn_bytes(
                &mut Vec::new(),
                0,
                &stepnum,
                &rspice_core::abort_signal::NoAbort,
            )
            .is_err()
        );
    }

    #[test]
    fn issue565566_provenance_rejects_role_and_exclusion_metadata_drift() {
        assert_provenance_mutation_rejected("wrapper-owner", |root| {
            fs::write(
                root.join(HARNESS_MANIFEST_FILE),
                format!(
                    "{}\t{REQUIRES_UPSTREAM_WRAPPER_CONTRACT}\n",
                    Issue565566Role::TableReference.path()
                ),
            )
            .expect("change wrapper owner");
        });
        assert_provenance_mutation_rejected("exclusion-source", |root| {
            fs::write(
                root.join(UPSTREAM_EXCLUSIONS_MANIFEST_FILE),
                format!(
                    "schema_version\t{UPSTREAM_EXCLUSIONS_SCHEMA_VERSION}\nsource_commit\t{UPSTREAM_EXCLUSIONS_SOURCE_COMMIT}\nsource_netlists_tree\t{UPSTREAM_EXCLUSIONS_SOURCE_NETLISTS_TREE}\n{}\twrong/exclude\t{RSPICE_INDEPENDENTLY_QUALIFIED_DISPOSITION}\t{REFERENCE_CONTRACT}\n",
                    Issue565566Role::TableReference.path()
                ),
            )
            .expect("change exclusion source");
        });
        assert_provenance_mutation_rejected("exclusion-disposition", |root| {
            fs::write(
                root.join(UPSTREAM_EXCLUSIONS_MANIFEST_FILE),
                format!(
                    "schema_version\t{UPSTREAM_EXCLUSIONS_SCHEMA_VERSION}\nsource_commit\t{UPSTREAM_EXCLUSIONS_SOURCE_COMMIT}\nsource_netlists_tree\t{UPSTREAM_EXCLUSIONS_SOURCE_NETLISTS_TREE}\n{}\t{EXCLUSION_SOURCE}\t{UPSTREAM_EXCLUDED_DISPOSITION}\n",
                    Issue565566Role::TableReference.path()
                ),
            )
            .expect("change exclusion disposition");
        });
        assert_provenance_mutation_rejected("exclusion-contract", |root| {
            fs::write(
                root.join(UPSTREAM_EXCLUSIONS_MANIFEST_FILE),
                format!(
                    "schema_version\t{UPSTREAM_EXCLUSIONS_SCHEMA_VERSION}\nsource_commit\t{UPSTREAM_EXCLUSIONS_SOURCE_COMMIT}\nsource_netlists_tree\t{UPSTREAM_EXCLUSIONS_SOURCE_NETLISTS_TREE}\n{}\t{EXCLUSION_SOURCE}\t{RSPICE_INDEPENDENTLY_QUALIFIED_DISPOSITION}\twrong_contract\n",
                    Issue565566Role::TableReference.path()
                ),
            )
            .expect("change exclusion contract");
        });
    }

    #[test]
    fn issue565566_provenance_rejects_census_source_oversize_and_output_drift() {
        assert_provenance_mutation_rejected("extra-member", |root| {
            fs::write(root.join(FAMILY_DIRECTORY).join("extra.cir"), "extra\n")
                .expect("write extra");
        });
        assert_provenance_mutation_rejected("retained-source", |root| {
            fs::write(
                root.join(FAMILY_DIRECTORY)
                    .join(Issue565566Role::PwlOwner.file_name()),
                "changed\n",
            )
            .expect("change retained source");
        });
        assert_provenance_mutation_rejected("oversized-member", |root| {
            let expected_bytes = RETAINED_ARTIFACTS[0].1;
            let cap = expected_bytes * 2 + 3;
            fs::write(
                root.join(FAMILY_DIRECTORY).join(RETAINED_ARTIFACTS[0].0),
                vec![b'X'; cap + 1],
            )
            .expect("write oversized member");
        });
        assert_provenance_mutation_rejected("output-artifact", |root| {
            fs::write(
                root.join(FAMILY_DIRECTORY).join("issue565_566.cir.prn"),
                "invented\n",
            )
            .expect("write output artifact");
        });
    }

    #[test]
    fn issue565566_expired_deadline_fails_closed() {
        let root = corpus_root();
        let runner = XyceTestRunner::new(&root, XyceRunnerConfig::default());
        let expired = Instant::now()
            - Duration::from_millis(
                u64::try_from(runner.config.max_time_per_test_ms.max(1) + 1)
                    .expect("timeout fits u64"),
            );
        assert!(
            runner
                .validate_issue565566_oracle(
                    &deck(&root, Issue565566Role::PwlOwner),
                    Issue565566Role::PwlOwner,
                    expired,
                )
                .is_err()
        );
    }
}
