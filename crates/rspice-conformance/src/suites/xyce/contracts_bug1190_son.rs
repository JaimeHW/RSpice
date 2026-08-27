use super::*;

const LABEL: &str = "BUG_1190_SON process-parameter alias family";
const FAMILY_PREFIX: &str = "netlists/certification_tests/bug_1190_son/";
const FAMILY_DIRECTORY: &str = "Netlists/Certification_Tests/BUG_1190_SON";
const EXCLUSION_SOURCE: &str = "Netlists/Certification_Tests/BUG_1190_SON/exclude";
const OWNER_CONTRACT: &str = "bug1190_son_process_parameter_alias_relational_wrapper_owner";
const CONTROL_CONTRACT: &str = "bug1190_son_direct_model_parameter_relational_control";

const UPSTREAM_REGRESSION_COMMIT: &str = "d6e278e371ec2f3df1325dcff4552e585bc7ecc1";
const UPSTREAM_RELEASE_TAG: &str = "Release-7.10.0";
const VENDORING_COMMIT: &str = "317c587f7";
const HARNESS_TRIM_COMMIT: &str = "2e55e96a2";
const HISTORICAL_RECORD_COUNT: usize = 7;
const HISTORICAL_RECORD_BYTES: usize = 1_222;
const HISTORICAL_RECORDS_SHA256: &str =
    "8c7732c02c86f5c1fedab293b83e1ccf4329628290c2b134474f48b0ebb968b0";
const HISTORICAL_ARTIFACTS: [(&str, usize, &str); HISTORICAL_RECORD_COUNT] = [
    (
        "Netlists/Certification_Tests/BUG_1190_SON/Manifest.txt",
        225,
        "cf929bf9d2c0f0963242c5fc5a58e39373ed8e7c4242555951f8e5e4920bf280",
    ),
    (
        "Netlists/Certification_Tests/BUG_1190_SON/bsim3.cir.sh",
        2_298,
        "00f9eaad24eefe13ebbaf83a7c2113b3d60c6f084dabf822cd03f36383899c84",
    ),
    (
        "Netlists/Certification_Tests/BUG_1190_SON/bsim4.cir.sh",
        2_298,
        "00f9eaad24eefe13ebbaf83a7c2113b3d60c6f084dabf822cd03f36383899c84",
    ),
    (
        "Netlists/Certification_Tests/BUG_1190_SON/diode.cir.sh",
        2_228,
        "aaa1906d44712270ec344e89031567140e40ff88d13387e7950b9f71c0171cfd",
    ),
    (
        EXCLUSION_SOURCE,
        89,
        "1efbb176ad58cdb0ee187e48f43087293979279d1c20be9e7c32af48ef7b5f49",
    ),
    (
        "Netlists/Certification_Tests/BUG_1190_SON/tags",
        36,
        "0ea1756eda58e92ec6ea26316f966c5ad5fd72e2a7000c36359f928731c74b9f",
    ),
    (
        "TestScripts/file_compare.pl",
        7_465,
        "a700143baddab265ca2e74d69541432fb27ae66600c3fee71968797fc78efcbf0",
    ),
];

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub(super) enum Bug1190SonRole {
    Bsim3Owner,
    Bsim3Control,
    Bsim4Owner,
    Bsim4Control,
    DiodeOwner,
    DiodeControl,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Bug1190SonPair {
    Bsim3,
    Bsim4,
    Diode,
}

impl Bug1190SonRole {
    const ALL: [Self; 6] = [
        Self::Bsim3Owner,
        Self::Bsim3Control,
        Self::Bsim4Owner,
        Self::Bsim4Control,
        Self::DiodeOwner,
        Self::DiodeControl,
    ];
    const OWNERS: [Self; 3] = [Self::Bsim3Owner, Self::Bsim4Owner, Self::DiodeOwner];
    const CONTROLS: [Self; 3] = [Self::Bsim3Control, Self::Bsim4Control, Self::DiodeControl];

    pub(super) fn for_record(record: &str) -> Option<Self> {
        let record = XyceTestRunner::normalize_manifest_key(record);
        Self::ALL.into_iter().find(|role| role.record() == record)
    }

    pub(super) fn contract(self) -> &'static str {
        if self.is_owner() {
            OWNER_CONTRACT
        } else {
            CONTROL_CONTRACT
        }
    }

    fn is_owner(self) -> bool {
        matches!(self, Self::Bsim3Owner | Self::Bsim4Owner | Self::DiodeOwner)
    }

    fn pair(self) -> Bug1190SonPair {
        match self {
            Self::Bsim3Owner | Self::Bsim3Control => Bug1190SonPair::Bsim3,
            Self::Bsim4Owner | Self::Bsim4Control => Bug1190SonPair::Bsim4,
            Self::DiodeOwner | Self::DiodeControl => Bug1190SonPair::Diode,
        }
    }

    fn path(self) -> &'static str {
        match self {
            Self::Bsim3Owner => "Netlists/Certification_Tests/BUG_1190_SON/bsim3.cir",
            Self::Bsim3Control => "Netlists/Certification_Tests/BUG_1190_SON/bsim3_modpar.cir",
            Self::Bsim4Owner => "Netlists/Certification_Tests/BUG_1190_SON/bsim4.cir",
            Self::Bsim4Control => "Netlists/Certification_Tests/BUG_1190_SON/bsim4_modpar.cir",
            Self::DiodeOwner => "Netlists/Certification_Tests/BUG_1190_SON/diode.cir",
            Self::DiodeControl => "Netlists/Certification_Tests/BUG_1190_SON/diodeRef.cir",
        }
    }

    fn record(self) -> &'static str {
        match self {
            Self::Bsim3Owner => "netlists/certification_tests/bug_1190_son/bsim3.cir",
            Self::Bsim3Control => "netlists/certification_tests/bug_1190_son/bsim3_modpar.cir",
            Self::Bsim4Owner => "netlists/certification_tests/bug_1190_son/bsim4.cir",
            Self::Bsim4Control => "netlists/certification_tests/bug_1190_son/bsim4_modpar.cir",
            Self::DiodeOwner => "netlists/certification_tests/bug_1190_son/diode.cir",
            Self::DiodeControl => "netlists/certification_tests/bug_1190_son/dioderef.cir",
        }
    }

    fn file_name(self) -> &'static str {
        Path::new(self.path())
            .file_name()
            .and_then(|name| name.to_str())
            .expect("BUG1190 role path has a UTF-8 file name")
    }

    fn source_identity(self) -> (usize, &'static str, &'static str) {
        match self {
            Self::Bsim3Owner => (
                2_019,
                "52e8e3f4b71d31ee623d41359cb007dfce515db4bc3946a2522adcf8de82a3b0",
                "f0428a4a81a105e89675e46d3773b1ca268b9cbbf30466f45d27165f3fa2c9ee",
            ),
            Self::Bsim3Control => (
                2_006,
                "b9153810e795a6456d3f0362241038857c9012bccadd0cc12f8dcf3e38e82f9f",
                "53c4c08a1473c701ba229fc3505c3c9b9ce1d6f02be0657027a38f80fb440226",
            ),
            Self::Bsim4Owner => (
                7_835,
                "3e7ef07130e5662e65e5eba49ca72ac364fea76f34dea34f1a9be61cd02e6763",
                "f251b43614d11b3817e5d86d4664b36b78e4b82734da95e3f7dbc7e97f6c280a",
            ),
            Self::Bsim4Control => (
                7_788,
                "79944ba5d4e86bc778302d38b1d9e977c1d0eeedcdfb1316cc6ecd4c87442431",
                "d1db6f27dd40e06ee6682a99eb76ec1fab000ebb4eff962d65287601286aa8ac",
            ),
            Self::DiodeOwner => (
                304,
                "64446ca7c39d71dedb31d1d6d8d7f55cc6b61f851ad710271d303d7f72d79124",
                "61c0a4f4ff4aa9935b22d21bfc384b2fbfa467eda0ecd3b9220d43506c2e5794",
            ),
            Self::DiodeControl => (
                262,
                "1ff64d3a8fb4da8bc17431280ac6641fa07c26676d0f4658032520c08bce605a",
                "7263ced101e667a49f4242ab793fff79a55441f56704ebbc7f74ebe6a106aead",
            ),
        }
    }
}

impl Bug1190SonPair {
    fn roles(self) -> (Bug1190SonRole, Bug1190SonRole) {
        match self {
            Self::Bsim3 => (Bug1190SonRole::Bsim3Owner, Bug1190SonRole::Bsim3Control),
            Self::Bsim4 => (Bug1190SonRole::Bsim4Owner, Bug1190SonRole::Bsim4Control),
            Self::Diode => (Bug1190SonRole::DiodeOwner, Bug1190SonRole::DiodeControl),
        }
    }

    fn expected_run_count(self) -> usize {
        match self {
            Self::Bsim3 => 30,
            Self::Bsim4 => 27,
            Self::Diode => 6,
        }
    }
}

impl XyceTestRunner {
    fn bug1190_son_historical_records() -> Vec<String> {
        let mut records = HISTORICAL_ARTIFACTS
            .into_iter()
            .map(|(path, bytes, sha256)| {
                format!(
                    "{UPSTREAM_REGRESSION_COMMIT}\t{UPSTREAM_RELEASE_TAG}\t{path}\t{bytes}\t{sha256}"
                )
            })
            .collect::<Vec<_>>();
        records.sort();
        records
    }

    fn validate_bug1190_son_historical_provenance() -> Result<(), String> {
        let records = Self::bug1190_son_historical_records();
        let stream = records.join("\n");
        let sha256 = format!("{:x}", Sha256::digest(stream.as_bytes()));
        if records.len() != HISTORICAL_RECORD_COUNT
            || stream.len() != HISTORICAL_RECORD_BYTES
            || sha256 != HISTORICAL_RECORDS_SHA256
            || UPSTREAM_EXCLUSIONS_SOURCE_COMMIT != "80115a9277c0ddb3409acceb3d4e745fd11cddd4"
        {
            return Err(format!(
                "{LABEL} historical wrapper provenance changed: records={}/{}, bytes={}/{}, sha256={sha256}; vendor={VENDORING_COMMIT}, trim={HARNESS_TRIM_COMMIT}",
                records.len(),
                HISTORICAL_RECORD_COUNT,
                stream.len(),
                HISTORICAL_RECORD_BYTES
            ));
        }
        Ok(())
    }

    fn validate_bug1190_son_provenance(
        &self,
        deck: &XyceDeck,
        role: Bug1190SonRole,
    ) -> Result<BTreeMap<Bug1190SonRole, Vec<u8>>, String> {
        Self::validate_bug1190_son_historical_provenance()?;
        if deck.section != XyceDeckSection::Netlists
            || Self::normalize_manifest_key(&deck.relative_path) != role.record()
            || Self::normalize_manifest_key(&self.relative_key(&deck.path)) != role.record()
            || !Self::same_path(&deck.path, &self.root.join(role.path()))
        {
            return Err(format!("recognized {LABEL} role {role:?} is not canonical"));
        }

        let family = self.root.join(FAMILY_DIRECTORY);
        let metadata = fs::symlink_metadata(&family)
            .map_err(|error| format!("failed to inspect {LABEL} directory: {error}"))?;
        if metadata.file_type().is_symlink() || !metadata.file_type().is_dir() {
            return Err(format!(
                "{LABEL} directory must be a regular non-symlink directory"
            ));
        }
        let expected_names = BTreeSet::from([
            "bsim3.cir".to_string(),
            "bsim3_modpar.cir".to_string(),
            "bsim4.cir".to_string(),
            "bsim4_modpar.cir".to_string(),
            "diode.cir".to_string(),
            "diodeRef.cir".to_string(),
            "mutInd1.cir".to_string(),
            "mutInd1_baseline.cir".to_string(),
            "mutInd2.cir".to_string(),
            "mutInd2_baseline.cir".to_string(),
        ]);
        let mut actual_names = BTreeSet::new();
        for entry in fs::read_dir(&family)
            .map_err(|error| format!("failed to enumerate {LABEL}: {error}"))?
        {
            let entry =
                entry.map_err(|error| format!("failed to inspect {LABEL} member: {error}"))?;
            let member = entry.path();
            let metadata = fs::symlink_metadata(&member)
                .map_err(|error| format!("failed to inspect {}: {error}", member.display()))?;
            if metadata.file_type().is_symlink() || !metadata.file_type().is_file() {
                return Err(format!(
                    "{LABEL} member '{}' must be a regular non-symlink file",
                    member.display()
                ));
            }
            let name = entry
                .file_name()
                .to_str()
                .ok_or_else(|| format!("{LABEL} member name is not UTF-8"))?
                .to_string();
            if !actual_names.insert(name.clone()) {
                return Err(format!("{LABEL} contains duplicate member {name:?}"));
            }
        }
        if actual_names != expected_names {
            return Err(format!(
                "{LABEL} physical ten-deck census changed: {actual_names:?}"
            ));
        }

        let expected_owners = BTreeSet::from([
            Bug1190SonRole::Bsim3Owner.record().to_string(),
            Bug1190SonRole::Bsim4Owner.record().to_string(),
            Bug1190SonRole::DiodeOwner.record().to_string(),
            "netlists/certification_tests/bug_1190_son/mutind1.cir".to_string(),
            "netlists/certification_tests/bug_1190_son/mutind2.cir".to_string(),
        ]);
        let actual_owners = self
            .upstream_wrapper_decks
            .iter()
            .filter(|record| record.starts_with(FAMILY_PREFIX))
            .cloned()
            .collect::<BTreeSet<_>>();
        if actual_owners != expected_owners {
            return Err(format!(
                "{LABEL} wrapper-owner census changed: {actual_owners:?}"
            ));
        }

        let exclusions = Self::load_upstream_exclusions(&self.root)
            .map_err(|error| format!("{LABEL} exclusions are invalid: {error}"))?;
        let expected_exclusions = BTreeSet::from([
            Bug1190SonRole::Bsim3Control.record().to_string(),
            Bug1190SonRole::Bsim4Control.record().to_string(),
            Bug1190SonRole::DiodeControl.record().to_string(),
            "netlists/certification_tests/bug_1190_son/mutind1_baseline.cir".to_string(),
            "netlists/certification_tests/bug_1190_son/mutind2_baseline.cir".to_string(),
        ]);
        let actual_exclusions = exclusions
            .keys()
            .filter(|record| record.starts_with(FAMILY_PREFIX))
            .cloned()
            .collect::<BTreeSet<_>>();
        if actual_exclusions != expected_exclusions {
            return Err(format!(
                "{LABEL} exclusion census changed: {actual_exclusions:?}"
            ));
        }
        for control in Bug1190SonRole::CONTROLS {
            let qualification = exclusions
                .get(control.record())
                .ok_or_else(|| format!("{LABEL} lost {:?} qualification", control))?;
            if qualification.source != EXCLUSION_SOURCE
                || !matches!(
                    &qualification.disposition,
                    XyceUpstreamExclusionDisposition::RspiceIndependentlyQualified {
                        expected_contract
                    } if expected_contract == CONTROL_CONTRACT
                )
            {
                return Err(format!(
                    "{LABEL} {:?} qualification changed: {qualification:?}",
                    control
                ));
            }
        }
        for owner in Bug1190SonRole::OWNERS {
            if exclusions.contains_key(owner.record()) {
                return Err(format!("{LABEL} owner {owner:?} must not be excluded"));
            }
        }

        let output = self
            .root
            .join("OutputData/Certification_Tests/BUG_1190_SON");
        match fs::symlink_metadata(&output) {
            Err(error) if error.kind() == std::io::ErrorKind::NotFound => {}
            Err(error) => return Err(format!("failed to inspect {LABEL} OutputData: {error}")),
            Ok(_) => return Err(format!("{LABEL} must not acquire numerical gold")),
        }

        let mut sources = BTreeMap::new();
        for member in Bug1190SonRole::ALL {
            let path = self.root.join(member.path());
            let bytes = fs::read(&path)
                .map_err(|error| format!("failed to read {}: {error}", path.display()))?;
            let canonical = Self::canonical_lf_text_identity(LABEL, &bytes)?;
            let (expected_bytes, expected_sha256, expected_blake3) = member.source_identity();
            let sha256 = format!("{:x}", Sha256::digest(&canonical));
            let content_blake3 = blake3::hash(&canonical).to_hex().to_string();
            if canonical.len() != expected_bytes
                || sha256 != expected_sha256
                || content_blake3 != expected_blake3
            {
                return Err(format!(
                    "{LABEL} source {} changed: bytes={}, sha256={sha256}, blake3={content_blake3}",
                    member.file_name(),
                    canonical.len()
                ));
            }
            self.reject_wrapper_output_artifacts(&path)
                .map_err(|error| format!("{LABEL} {} {error}", member.file_name()))?;
            sources.insert(member, bytes);
        }
        Ok(sources)
    }

    fn exact_bug1190_linear_step(
        step: &StepCommand,
        target: StepTarget,
        name: &str,
        parameter: Option<&str>,
        expected: (Value, Value, Value),
    ) -> bool {
        step.target == target
            && step.name.eq_ignore_ascii_case(name)
            && match (&step.param_name, parameter) {
                (None, None) => true,
                (Some(actual), Some(expected)) => actual.eq_ignore_ascii_case(expected),
                _ => false,
            }
            && matches!(
                &step.sweep,
                StepSweep::Linear { start, stop, step }
                    if start.to_bits() == expected.0.to_bits()
                        && stop.to_bits() == expected.1.to_bits()
                        && step.to_bits() == expected.2.to_bits()
            )
    }

    fn output_operands_are_exact(
        netlist: &Netlist,
        analysis: rspice_core::netlist::OutputAnalysisKind,
        expected: &[&str],
    ) -> bool {
        let [request] = netlist.output_requests.as_slice() else {
            return false;
        };
        request.directive == OutputDirectiveKind::Print
            && request.analysis == Some(analysis)
            && request.name.is_none()
            && request
                .operands
                .iter()
                .map(String::as_str)
                .eq(expected.iter().copied())
    }

    fn validate_bug1190_son_dc_plan(
        &self,
        role: Bug1190SonRole,
        source: &str,
    ) -> Result<(XyceStaticDcPlan, Netlist), String> {
        let path = self.root.join(role.path());
        let plan = self.static_dc_plan_for_source_with_execution_dir(
            &path,
            source.to_string(),
            ExpressionDialect::Xyce,
            None,
        )?;
        let (expected_probes, expected_primary, expected_secondary): (
            &[&str],
            (Value, Value, Value),
            (Value, Value, Value),
        ) = match role.pair() {
            Bug1190SonPair::Bsim3 => (
                ["v(2)", "v(1)", "i(vds)", "M1:L", "M1:W"].as_slice(),
                (0.0, 3.5, 0.5),
                (0.0, 3.5, 0.5),
            ),
            Bug1190SonPair::Bsim4 => (
                ["v(2)", "v(1)", "i(vds)", "M1:RBDB", "M1:RBSB", "M1:RBPS"].as_slice(),
                (0.0, 1.2, 0.2),
                (0.2, 1.2, 0.2),
            ),
            Bug1190SonPair::Diode => return Err(format!("{LABEL} diode role is not DC")),
        };
        let Some(secondary) = plan.dc.sweep2.as_ref() else {
            return Err(format!("{LABEL} {role:?} lost its secondary DC sweep"));
        };
        if plan.deck_path != path
            || plan.execution_dir.is_some()
            || plan.expression_dialect != ExpressionDialect::Xyce
            || plan.parameter_redefinition_policy != ParameterRedefinitionPolicy::UseLast
            || !plan.diagnostics.is_empty()
            || plan.print_format.is_some()
            || plan.dc_data.is_some()
            || plan
                .print
                .probes
                .iter()
                .map(String::as_str)
                .ne(expected_probes.iter().copied())
            || !plan.dc.source.eq_ignore_ascii_case("VDS")
            || plan.dc.mode != DcSweepMode::Linear
            || plan.dc.start.to_bits() != expected_primary.0.to_bits()
            || plan.dc.stop.to_bits() != expected_primary.1.to_bits()
            || plan.dc.step.to_bits() != expected_primary.2.to_bits()
            || !secondary.source.eq_ignore_ascii_case("VGS")
            || secondary.mode != DcSweepMode::Linear
            || secondary.start.to_bits() != expected_secondary.0.to_bits()
            || secondary.stop.to_bits() != expected_secondary.1.to_bits()
            || secondary.step.to_bits() != expected_secondary.2.to_bits()
        {
            return Err(format!("{LABEL} {role:?} DC plan changed: {plan:?}"));
        }

        let steps_ok = match role {
            Bug1190SonRole::Bsim3Owner => matches!(plan.steps.as_slice(), [first, second]
                if Self::exact_bug1190_linear_step(first, StepTarget::Param, "N1L", None, (0.35e-6, 0.4e-6, 0.01e-6))
                    && Self::exact_bug1190_linear_step(second, StepTarget::Param, "N1W", None, (8.0e-6, 12.0e-6, 1.0e-6))),
            Bug1190SonRole::Bsim3Control => matches!(plan.steps.as_slice(), [first, second]
                if Self::exact_bug1190_linear_step(first, StepTarget::Device, "N1", Some("L"), (0.35e-6, 0.4e-6, 0.01e-6))
                    && Self::exact_bug1190_linear_step(second, StepTarget::Device, "N1", Some("W"), (8.0e-6, 12.0e-6, 1.0e-6))),
            Bug1190SonRole::Bsim4Owner => matches!(plan.steps.as_slice(), [first, second, third]
                if Self::exact_bug1190_linear_step(first, StepTarget::Param, "gp_rbdb", None, (14.0, 16.0, 1.0))
                    && Self::exact_bug1190_linear_step(second, StepTarget::Param, "gp_rbsb", None, (14.0, 16.0, 1.0))
                    && Self::exact_bug1190_linear_step(third, StepTarget::Param, "gp_rbps", None, (14.0, 16.0, 1.0))),
            Bug1190SonRole::Bsim4Control => matches!(plan.steps.as_slice(), [first, second, third]
                if Self::exact_bug1190_linear_step(first, StepTarget::Device, "N1", Some("RBDB"), (14.0, 16.0, 1.0))
                    && Self::exact_bug1190_linear_step(second, StepTarget::Device, "N1", Some("RBSB"), (14.0, 16.0, 1.0))
                    && Self::exact_bug1190_linear_step(third, StepTarget::Device, "N1", Some("RBPS"), (14.0, 16.0, 1.0))),
            Bug1190SonRole::DiodeOwner | Bug1190SonRole::DiodeControl => false,
        };
        if !steps_ok {
            return Err(format!(
                "{LABEL} {role:?} STEP plan changed: {:?}",
                plan.steps
            ));
        }

        let netlist = Self::parse_xyce_netlist(source, &path)
            .map_err(|error| format!("{LABEL} {role:?} no longer parses: {error}"))?;
        if !netlist.diagnostics.is_empty()
            || netlist.models.len() != 1
            || netlist.analyses.len() != plan.steps.len() + 1
            || netlist
                .analyses
                .iter()
                .filter(|analysis| matches!(analysis, AnalysisCommand::Dc { .. }))
                .count()
                != 1
            || netlist.analyses.iter().any(|analysis| {
                !matches!(
                    analysis,
                    AnalysisCommand::Dc { .. } | AnalysisCommand::Step(_)
                )
            })
            || !netlist.data_tables.is_empty()
            || !netlist.measurements.is_empty()
            || !netlist.subcircuits.is_empty()
            || !Self::output_operands_are_exact(
                &netlist,
                rspice_core::netlist::OutputAnalysisKind::Dc,
                expected_probes,
            )
        {
            return Err(format!(
                "{LABEL} {role:?} typed DC envelope changed: diagnostics={:?}, models={}, analyses={:?}, data={}, measures={}, subcircuits={}, output={:?}",
                netlist.diagnostics,
                netlist.models.len(),
                netlist.analyses,
                netlist.data_tables.len(),
                netlist.measurements.len(),
                netlist.subcircuits.len(),
                netlist.output_requests,
            ));
        }
        Ok((plan, netlist))
    }

    fn validate_bug1190_son_tran_plan(
        &self,
        role: Bug1190SonRole,
    ) -> Result<(XyceStaticTranPlan, Netlist), String> {
        let path = self.root.join(role.path());
        let purpose = XyceStaticTranPlanPurpose::Bug1190SonProcessParameterRelationalFamily;
        let plan = self.static_tran_family_plan_for_path(&path, purpose)?;
        let expected_probes = ["I(VMON)", "V(3)", "{V(3)/I(VMON)}"];
        let expected_contract = if role.is_owner() {
            XyceStaticTranContract::WrapperStatic
        } else {
            XyceStaticTranContract::PlainStatic
        };
        let print = plan.require_print(LABEL)?;
        let [step] = plan.steps.as_slice() else {
            return Err(format!("{LABEL} {role:?} requires exactly one STEP"));
        };
        let step_ok = match role {
            Bug1190SonRole::DiodeOwner => Self::exact_bug1190_linear_step(
                step,
                StepTarget::Param,
                "gpIS",
                None,
                (100.0 * 1.0e-15, 150.0 * 1.0e-15, 10.0 * 1.0e-15),
            ),
            Bug1190SonRole::DiodeControl => Self::exact_bug1190_linear_step(
                step,
                StepTarget::Device,
                "DMOD",
                Some("IS"),
                (100.0 * 1.0e-15, 150.0 * 1.0e-15, 10.0 * 1.0e-15),
            ),
            _ => false,
        };
        if plan.deck_path != path
            || plan.contract != expected_contract
            || !matches!(plan.oracle, XyceStaticTranOracle::None)
            || plan.output_override
            || plan.timeint_conststep
            || plan.wrapper_tolerance.is_some()
            || !matches!(
                plan.comparison_mode,
                XyceStaticTranComparisonMode::Pointwise
            )
            || plan.tran.step.to_bits() != 0.0f64.to_bits()
            || plan.tran.stop.to_bits() != 0.5e-3f64.to_bits()
            || plan.tran.start.is_some()
            || plan.tran.max_step.is_some()
            || plan.tran.uic
            || print
                .probes
                .iter()
                .map(String::as_str)
                .ne(expected_probes.iter().copied())
            || !step_ok
        {
            return Err(format!("{LABEL} {role:?} TRAN plan changed: {plan:?}"));
        }
        let netlist = Self::parse_xyce_netlist(&plan.source, &path)
            .map_err(|error| format!("{LABEL} {role:?} no longer parses: {error}"))?;
        if !netlist.diagnostics.is_empty()
            || netlist.models.len() != 1
            || netlist.analyses.len() != plan.steps.len() + 1
            || netlist
                .analyses
                .iter()
                .filter(|analysis| matches!(analysis, AnalysisCommand::Tran { .. }))
                .count()
                != 1
            || netlist.analyses.iter().any(|analysis| {
                !matches!(
                    analysis,
                    AnalysisCommand::Tran { .. } | AnalysisCommand::Step(_)
                )
            })
            || !netlist.data_tables.is_empty()
            || !netlist.measurements.is_empty()
            || !netlist.subcircuits.is_empty()
            || !Self::output_operands_are_exact(
                &netlist,
                rspice_core::netlist::OutputAnalysisKind::Tran,
                &expected_probes,
            )
        {
            return Err(format!(
                "{LABEL} {role:?} typed TRAN envelope changed: diagnostics={:?}, models={}, analyses={:?}, data={}, measures={}, subcircuits={}, output={:?}",
                netlist.diagnostics,
                netlist.models.len(),
                netlist.analyses,
                netlist.data_tables.len(),
                netlist.measurements.len(),
                netlist.subcircuits.len(),
                netlist.output_requests,
            ));
        }
        Ok((plan, netlist))
    }

    pub(super) fn validate_bug1190_son_static_tran_contract(
        netlist: &Netlist,
    ) -> Result<(), String> {
        if netlist.elements.len() != 4
            || netlist.models.len() != 1
            || !netlist.subcircuits.is_empty()
            || !netlist.data_tables.is_empty()
            || !netlist.measurements.is_empty()
        {
            return Err(format!(
                "{LABEL} requires its exact four-element diode topology"
            ));
        }
        for element in &netlist.elements {
            match &element.kind {
                ElementKind::VoltageSource(spec) => {
                    Self::validate_static_step_tran_source_spec(&element.name, spec)?;
                }
                ElementKind::Resistor { .. } => {
                    Self::validate_static_step_resistor_contract(netlist, &element.name)?;
                }
                ElementKind::Diode {
                    model,
                    instance_params,
                    deferred_params,
                } => {
                    if !element.name.eq_ignore_ascii_case("D1")
                        || element.nodes != ["3", "0"]
                        || !model.eq_ignore_ascii_case("DMOD")
                        || !instance_params.is_empty()
                        || !deferred_params.is_empty()
                    {
                        return Err(format!("{LABEL} diode instance envelope changed"));
                    }
                    let model = &netlist.models[0];
                    let numeric_is = model
                        .params
                        .iter()
                        .filter(|(name, _)| name.eq_ignore_ascii_case("IS"))
                        .collect::<Vec<_>>();
                    let expression_is = model
                        .expr_params
                        .iter()
                        .filter(|(name, _)| name.eq_ignore_ascii_case("IS"))
                        .collect::<Vec<_>>();
                    let numeric_ok = matches!(numeric_is.as_slice(), [(_, value)] if value.is_finite() && *value > 0.0)
                        && expression_is.is_empty();
                    let expression_ok = numeric_is.is_empty()
                        && matches!(expression_is.as_slice(), [(_, expression)] if expression.trim().trim_matches(['{', '}']).eq_ignore_ascii_case("pIS"));
                    if !model.name.eq_ignore_ascii_case("DMOD")
                        || !model.model_type.eq_ignore_ascii_case("D")
                        || (!numeric_ok && !expression_ok)
                        || model.params.len() + model.expr_params.len() != 1
                        || !model.string_params.is_empty()
                        || !model.string_vector_params.is_empty()
                        || !model.real_vector_params.is_empty()
                        || !model.real_vector_expr_params.is_empty()
                        || !model.integer_vector_params.is_empty()
                    {
                        return Err(format!("{LABEL} diode model envelope changed: {model:?}"));
                    }
                }
                other => {
                    return Err(format!(
                        "{LABEL} contains unsupported element '{}': {other:?}",
                        element.name
                    ));
                }
            }
        }
        Ok(())
    }

    fn bug1190_son_expected_coordinates(steps: &[StepCommand]) -> Vec<Vec<Value>> {
        let grids = steps
            .iter()
            .map(|step| step.sweep.values())
            .collect::<Vec<_>>();
        let run_count = grids.iter().map(Vec::len).product::<usize>();
        (0..run_count)
            .map(|run_index| {
                let mut stride = 1usize;
                grids
                    .iter()
                    .map(|grid| {
                        let value = grid[(run_index / stride) % grid.len()];
                        stride *= grid.len();
                        value
                    })
                    .collect()
            })
            .collect()
    }

    fn bug1190_son_model_param(
        netlist: &Netlist,
        model_name: &str,
        parameter: &str,
    ) -> Result<Value, String> {
        let model = netlist
            .models
            .iter()
            .find(|model| model.name.eq_ignore_ascii_case(model_name))
            .ok_or_else(|| format!("{LABEL} lost model {model_name}"))?;
        let values = model
            .params
            .iter()
            .filter(|(name, _)| name.eq_ignore_ascii_case(parameter))
            .map(|(_, value)| *value)
            .collect::<Vec<_>>();
        let [value] = values.as_slice() else {
            return Err(format!(
                "{LABEL} model {model_name} parameter {parameter} is not one exact scalar: {values:?}"
            ));
        };
        Ok(*value)
    }

    fn validate_bug1190_son_dc_table(
        pair: Bug1190SonPair,
        plan: &XyceStaticDcPlan,
        coordinates: &[Value],
        table: &XycePrnTable,
    ) -> Result<(), String> {
        let (expected_columns, parameter_offset) = match pair {
            Bug1190SonPair::Bsim3 => (vec!["Index", "v(2)", "v(1)", "i(vds)", "M1:L", "M1:W"], 4),
            Bug1190SonPair::Bsim4 => (
                vec![
                    "Index", "v(2)", "v(1)", "i(vds)", "M1:RBDB", "M1:RBSB", "M1:RBPS",
                ],
                4,
            ),
            Bug1190SonPair::Diode => return Err(format!("{LABEL} diode table is not DC")),
        };
        let primary = plan.dc.primary_spec().points();
        let secondary = plan
            .dc
            .sweep2
            .as_ref()
            .expect("BUG1190 DC plan has secondary sweep")
            .spec()
            .points();
        if table.columns != expected_columns || table.rows.len() != primary.len() * secondary.len()
        {
            return Err(format!(
                "{LABEL} {pair:?} rendered DC shape changed: {:?}/{}",
                table.columns,
                table.rows.len()
            ));
        }
        let mut nonzero_current = false;
        for (index, row) in table.rows.iter().enumerate() {
            if row.len() != table.columns.len() || row.iter().any(|value| !value.is_finite()) {
                return Err(format!(
                    "{LABEL} {pair:?} DC row {index} is invalid: {row:?}"
                ));
            }
            let expected_primary = primary[index % primary.len()];
            let expected_secondary = secondary[index / primary.len()];
            let exact_after_print = |actual: Value, expected: Value| -> Result<bool, String> {
                Ok(Self::xyce_default_prn_roundtrip(actual)?.to_bits()
                    == Self::xyce_default_prn_roundtrip(expected)?.to_bits())
            };
            if row[0].to_bits() != (index as Value).to_bits()
                || !exact_after_print(row[1], expected_primary)?
                || !exact_after_print(row[2], expected_secondary)?
                || coordinates
                    .iter()
                    .enumerate()
                    .any(|(coordinate, expected)| {
                        !exact_after_print(row[parameter_offset + coordinate], *expected)
                            .unwrap_or(false)
                    })
            {
                return Err(format!(
                    "{LABEL} {pair:?} DC row {index} lost its axes or model-coordinate projection: {row:?}"
                ));
            }
            nonzero_current |= row[3].abs() > 1.0e-20;
        }
        if !nonzero_current {
            return Err(format!(
                "{LABEL} {pair:?} DC result is electrically vacuous"
            ));
        }
        Ok(())
    }

    /// Enforce the intended numeric tolerance without Release 7.10
    /// `file_compare.pl`'s malformed FFT-phase escape clause. The BUG1190
    /// tables contain no phase columns, so every serialized numeric cell must
    /// satisfy the ordinary absolute-and-relative comparison.
    fn validate_bug1190_son_phase_safe_equivalence(
        control: &XycePrnTable,
        owner: &XycePrnTable,
    ) -> Result<(), String> {
        let tolerance = XyceFileCompareTolerance::BUG1190_SON_PROCESS_PARAMETER.validate()?;
        if control.columns != owner.columns {
            return Err(format!(
                "{LABEL} phase-safe headers differ: control {:?}, owner {:?}",
                control.columns, owner.columns
            ));
        }
        if control.rows.len() != owner.rows.len() {
            return Err(format!(
                "{LABEL} phase-safe row counts differ: control {}, owner {}",
                control.rows.len(),
                owner.rows.len()
            ));
        }

        for (row_index, (control_row, owner_row)) in
            control.rows.iter().zip(&owner.rows).enumerate()
        {
            if control_row.len() != control.columns.len() || owner_row.len() != owner.columns.len()
            {
                return Err(format!(
                    "{LABEL} phase-safe row {row_index} width differs from its header: control {}/{}, owner {}/{}",
                    control_row.len(),
                    control.columns.len(),
                    owner_row.len(),
                    owner.columns.len()
                ));
            }
            for (column_index, (&control_value, &owner_value)) in
                control_row.iter().zip(owner_row).enumerate()
            {
                let probe = &control.columns[column_index];
                let expected =
                    Self::xyce_default_prn_roundtrip(control_value).map_err(|error| {
                        format!(
                            "{LABEL} cannot serialize control {probe} at row {row_index}: {error}"
                        )
                    })?;
                let actual = Self::xyce_default_prn_roundtrip(owner_value).map_err(|error| {
                    format!("{LABEL} cannot serialize owner {probe} at row {row_index}: {error}")
                })?;
                let absolute_error = (actual - expected).abs();
                let relative_error = absolute_error / expected.abs();
                let exact = actual == expected;
                let both_zero = actual.abs() <= tolerance.zero && expected.abs() <= tolerance.zero;
                let within_both =
                    absolute_error < tolerance.absolute && relative_error < tolerance.relative;
                if !exact && !both_zero && !within_both {
                    return Err(format!(
                        "{LABEL} phase-safe comparison differs at row {row_index}, {probe}: control={expected}, owner={actual}, absolute error={absolute_error}, relative error={relative_error}"
                    ));
                }
            }
        }
        Ok(())
    }

    fn run_bug1190_son_dc_pair(
        &self,
        pair: Bug1190SonPair,
        sources: &BTreeMap<Bug1190SonRole, Vec<u8>>,
        start: Instant,
    ) -> Result<(), String> {
        let (owner_role, control_role) = pair.roles();
        let source = |role| -> Result<&str, String> {
            std::str::from_utf8(
                sources
                    .get(&role)
                    .ok_or_else(|| format!("{LABEL} lost {role:?} source"))?,
            )
            .map_err(|error| format!("{LABEL} {role:?} is not UTF-8: {error}"))
        };
        let (owner_plan, owner_netlist) =
            self.validate_bug1190_son_dc_plan(owner_role, source(owner_role)?)?;
        let (control_plan, control_netlist) =
            self.validate_bug1190_son_dc_plan(control_role, source(control_role)?)?;
        let expected = Self::bug1190_son_expected_coordinates(&owner_plan.steps);
        if expected.len() != pair.expected_run_count()
            || Self::bug1190_son_expected_coordinates(&control_plan.steps) != expected
        {
            return Err(format!("{LABEL} {pair:?} Cartesian STEP grid changed"));
        }

        let owner_engine = self.create_dc_engine();
        let control_engine = self.create_dc_engine();
        let abort = DeadlineAbort::new(start, self.config.max_time_per_test_ms.max(1));
        let owner_steps = owner_engine
            .plan_step_commands_with_abort(
                &owner_netlist,
                &owner_plan.steps,
                xyce_step_plan_limits(),
                &abort,
            )
            .map_err(|error| format!("{LABEL} {pair:?} owner STEP planning failed: {error}"))?;
        let control_steps = control_engine
            .plan_step_commands_with_abort(
                &control_netlist,
                &control_plan.steps,
                xyce_step_plan_limits(),
                &abort,
            )
            .map_err(|error| format!("{LABEL} {pair:?} control STEP planning failed: {error}"))?;
        if owner_steps.total_runs() != expected.len()
            || control_steps.total_runs() != expected.len()
        {
            return Err(format!("{LABEL} {pair:?} materialized run count changed"));
        }

        let (model, parameters): (&str, &[&str]) = match pair {
            Bug1190SonPair::Bsim3 => ("N1", &["L", "W"]),
            Bug1190SonPair::Bsim4 => ("N1", &["RBDB", "RBSB", "RBPS"]),
            Bug1190SonPair::Diode => unreachable!(),
        };
        let mut observed = BTreeSet::new();
        let mut total_rows = 0usize;
        for (run_index, coordinates) in expected.iter().enumerate() {
            if abort.is_aborted() {
                return Err(format!("{LABEL} {pair:?} exceeded its deadline"));
            }
            let (owner_values, owner_run) = owner_engine
                .materialize_step_run_with_abort(&owner_steps, run_index, &abort)
                .map_err(|error| format!("{LABEL} {pair:?} owner run {run_index} failed: {error}"))?
                .into_parts();
            let (control_values, control_run) = control_engine
                .materialize_step_run_with_abort(&control_steps, run_index, &abort)
                .map_err(|error| {
                    format!("{LABEL} {pair:?} control run {run_index} failed: {error}")
                })?
                .into_parts();
            if owner_values
                .iter()
                .map(|value| value.to_bits())
                .ne(coordinates.iter().map(|value| value.to_bits()))
                || control_values
                    .iter()
                    .map(|value| value.to_bits())
                    .ne(coordinates.iter().map(|value| value.to_bits()))
            {
                return Err(format!(
                    "{LABEL} {pair:?} run {run_index} lost first-declared-fastest STEP order"
                ));
            }
            observed.insert(
                coordinates
                    .iter()
                    .map(|value| value.to_bits())
                    .collect::<Vec<_>>(),
            );
            for (parameter_index, parameter) in parameters.iter().enumerate() {
                let expected_value = coordinates[parameter_index];
                let owner_value = Self::bug1190_son_model_param(&owner_run, model, parameter)?;
                let control_value = Self::bug1190_son_model_param(&control_run, model, parameter)?;
                if owner_value.to_bits() != expected_value.to_bits()
                    || control_value.to_bits() != expected_value.to_bits()
                {
                    return Err(format!(
                        "{LABEL} {pair:?} run {run_index} effective {model}:{parameter} differs: owner={owner_value}, control={control_value}, expected={expected_value}"
                    ));
                }
            }

            let owner_results = owner_engine
                .run_dc_sweep2_spec_with_report_and_abort(
                    &owner_run,
                    &owner_plan.dc.source,
                    &owner_plan.dc.primary_spec(),
                    owner_plan.dc.sweep2.as_ref(),
                    &abort,
                )
                .map_err(|error| format!("{LABEL} {pair:?} owner DC run failed: {error}"))?;
            let control_results = control_engine
                .run_dc_sweep2_spec_with_report_and_abort(
                    &control_run,
                    &control_plan.dc.source,
                    &control_plan.dc.primary_spec(),
                    control_plan.dc.sweep2.as_ref(),
                    &abort,
                )
                .map_err(|error| format!("{LABEL} {pair:?} control DC run failed: {error}"))?;
            let owner_table = self.dc_projection_results_to_prn_table(
                &owner_plan.print,
                &owner_plan.dc,
                &owner_run,
                &owner_results,
            )?;
            let control_table = self.dc_projection_results_to_prn_table(
                &control_plan.print,
                &control_plan.dc,
                &control_run,
                &control_results,
            )?;
            Self::validate_bug1190_son_dc_table(pair, &owner_plan, coordinates, &owner_table)?;
            Self::validate_bug1190_son_dc_table(pair, &control_plan, coordinates, &control_table)?;
            total_rows += owner_table.rows.len();
            let mismatches = self.compare_release_7_10_file_compare_tables(
                &control_table,
                &owner_table,
                XyceFileCompareTolerance::BUG1190_SON_PROCESS_PARAMETER,
            )?;
            if !mismatches.is_empty() {
                return Err(format!(
                    "{LABEL} {pair:?} run {run_index} differs from its direct-model control: {mismatches:?}"
                ));
            }
            Self::validate_bug1190_son_phase_safe_equivalence(&control_table, &owner_table)
                .map_err(|error| format!("{LABEL} {pair:?} run {run_index}: {error}"))?;
        }
        let expected_rows = match pair {
            Bug1190SonPair::Bsim3 => 1_920,
            Bug1190SonPair::Bsim4 => 1_134,
            Bug1190SonPair::Diode => unreachable!(),
        };
        if observed.len() != expected.len() || total_rows != expected_rows {
            return Err(format!(
                "{LABEL} {pair:?} execution coverage changed: coordinates={}, rows={total_rows}",
                observed.len()
            ));
        }
        Ok(())
    }

    fn validate_bug1190_son_tran_table(table: &XycePrnTable) -> Result<String, String> {
        let expected_columns = ["Index", "TIME", "I(VMON)", "V(3)", "{V(3)/I(VMON)}"];
        if table
            .columns
            .iter()
            .map(String::as_str)
            .ne(expected_columns)
            || table.rows.len() < 2
        {
            return Err(format!(
                "{LABEL} diode rendered TRAN shape changed: {:?}/{}",
                table.columns,
                table.rows.len()
            ));
        }
        let mut fingerprint = String::new();
        for (index, row) in table.rows.iter().enumerate() {
            if row.len() != expected_columns.len()
                || row.iter().any(|value| !value.is_finite())
                || row[0].to_bits() != (index as Value).to_bits()
                || (index > 0 && row[1] <= table.rows[index - 1][1])
            {
                return Err(format!(
                    "{LABEL} diode TRAN row {index} is invalid: {row:?}"
                ));
            }
            let quotient = row[3] / row[2];
            let error = (row[4] - quotient).abs();
            if !quotient.is_finite() || error > 1.0e-10_f64.max(quotient.abs() * 1.0e-10) {
                return Err(format!(
                    "{LABEL} diode expression projection changed at row {index}: {row:?}"
                ));
            }
            for value in &row[2..] {
                let serialized = Self::xyce_default_prn_roundtrip(*value)?;
                fingerprint.push_str(&format!("{:016x}", serialized.to_bits()));
            }
        }
        if Self::xyce_default_prn_roundtrip(table.rows[0][1])?.to_bits() != 0.0f64.to_bits()
            || Self::xyce_default_prn_roundtrip(table.rows.last().expect("nonempty")[1])?.to_bits()
                != Self::xyce_default_prn_roundtrip(0.5e-3)?.to_bits()
        {
            return Err(format!("{LABEL} diode TRAN endpoints changed"));
        }
        Ok(blake3::hash(fingerprint.as_bytes()).to_hex().to_string())
    }

    fn run_bug1190_son_diode_pair(
        &self,
        sources: &BTreeMap<Bug1190SonRole, Vec<u8>>,
        start: Instant,
    ) -> Result<(), String> {
        let pair = Bug1190SonPair::Diode;
        let (owner_role, control_role) = pair.roles();
        let (owner_plan, owner_netlist) = self.validate_bug1190_son_tran_plan(owner_role)?;
        let (control_plan, control_netlist) = self.validate_bug1190_son_tran_plan(control_role)?;
        if owner_plan.source.as_bytes()
            != sources
                .get(&owner_role)
                .expect("provenance returned owner source")
            || control_plan.source.as_bytes()
                != sources
                    .get(&control_role)
                    .expect("provenance returned control source")
        {
            return Err(format!(
                "{LABEL} diode plan source differs from provenance bytes"
            ));
        }
        let expected = Self::bug1190_son_expected_coordinates(&owner_plan.steps);
        if expected.len() != pair.expected_run_count()
            || Self::bug1190_son_expected_coordinates(&control_plan.steps) != expected
        {
            return Err(format!("{LABEL} diode STEP grid changed"));
        }
        let owner_engine = self.create_xyce_engine();
        let control_engine = self.create_xyce_engine();
        let abort = DeadlineAbort::new(start, self.config.max_time_per_test_ms.max(1));
        let owner_steps = owner_engine
            .plan_step_commands_with_abort(
                &owner_netlist,
                &owner_plan.steps,
                xyce_step_plan_limits(),
                &abort,
            )
            .map_err(|error| format!("{LABEL} diode owner STEP planning failed: {error}"))?;
        let control_steps = control_engine
            .plan_step_commands_with_abort(
                &control_netlist,
                &control_plan.steps,
                xyce_step_plan_limits(),
                &abort,
            )
            .map_err(|error| format!("{LABEL} diode control STEP planning failed: {error}"))?;
        if owner_steps.total_runs() != expected.len()
            || control_steps.total_runs() != expected.len()
        {
            return Err(format!("{LABEL} diode materialized run count changed"));
        }

        let mut owner_fingerprints = BTreeSet::new();
        let mut control_fingerprints = BTreeSet::new();
        for (run_index, coordinates) in expected.iter().enumerate() {
            if abort.is_aborted() {
                return Err(format!("{LABEL} diode exceeded its deadline"));
            }
            let (owner_values, owner_run) = owner_engine
                .materialize_step_run_with_abort(&owner_steps, run_index, &abort)
                .map_err(|error| format!("{LABEL} diode owner run {run_index} failed: {error}"))?
                .into_parts();
            let (control_values, control_run) = control_engine
                .materialize_step_run_with_abort(&control_steps, run_index, &abort)
                .map_err(|error| format!("{LABEL} diode control run {run_index} failed: {error}"))?
                .into_parts();
            if owner_values.len() != 1
                || control_values.len() != 1
                || owner_values[0].to_bits() != coordinates[0].to_bits()
                || control_values[0].to_bits() != coordinates[0].to_bits()
                || Self::bug1190_son_model_param(&owner_run, "DMOD", "IS")?.to_bits()
                    != coordinates[0].to_bits()
                || Self::bug1190_son_model_param(&control_run, "DMOD", "IS")?.to_bits()
                    != coordinates[0].to_bits()
            {
                return Err(format!(
                    "{LABEL} diode run {run_index} lost its effective DMOD:IS coordinate"
                ));
            }
            let owner_result = self
                .run_transient_family_netlist(&owner_plan, &owner_run, start, None, None)
                .map_err(|error| format!("{LABEL} diode owner TRAN failed: {error}"))?;
            let control_result = self
                .run_transient_family_netlist(&control_plan, &control_run, start, None, None)
                .map_err(|error| format!("{LABEL} diode control TRAN failed: {error}"))?;
            let owner_table =
                Self::transient_family_result_to_prn_table(&owner_plan, &owner_run, &owner_result)?;
            let control_table = Self::transient_family_result_to_prn_table(
                &control_plan,
                &control_run,
                &control_result,
            )?;
            owner_fingerprints.insert(Self::validate_bug1190_son_tran_table(&owner_table)?);
            control_fingerprints.insert(Self::validate_bug1190_son_tran_table(&control_table)?);
            let mismatches = self.compare_release_7_10_file_compare_tables(
                &control_table,
                &owner_table,
                XyceFileCompareTolerance::BUG1190_SON_PROCESS_PARAMETER,
            )?;
            if !mismatches.is_empty() {
                return Err(format!(
                    "{LABEL} diode run {run_index} differs from its direct-model control: {mismatches:?}"
                ));
            }
            Self::validate_bug1190_son_phase_safe_equivalence(&control_table, &owner_table)
                .map_err(|error| format!("{LABEL} diode run {run_index}: {error}"))?;
        }
        if owner_fingerprints.len() != expected.len()
            || control_fingerprints.len() != expected.len()
        {
            return Err(format!(
                "{LABEL} diode STEP coordinates do not produce six distinct electrical waveforms"
            ));
        }
        Ok(())
    }

    pub(super) fn validate_bug1190_son_oracle(
        &self,
        deck: &XyceDeck,
        role: Bug1190SonRole,
        start: Instant,
    ) -> Result<(), String> {
        let abort = DeadlineAbort::new(start, self.config.max_time_per_test_ms.max(1));
        if abort.is_aborted() {
            return Err(format!(
                "{LABEL} deadline expired before provenance validation"
            ));
        }
        let sources = self.validate_bug1190_son_provenance(deck, role)?;
        match role.pair() {
            Bug1190SonPair::Bsim3 => {
                self.run_bug1190_son_dc_pair(Bug1190SonPair::Bsim3, &sources, start)?
            }
            Bug1190SonPair::Bsim4 => {
                self.run_bug1190_son_dc_pair(Bug1190SonPair::Bsim4, &sources, start)?
            }
            Bug1190SonPair::Diode => self.run_bug1190_son_diode_pair(&sources, start)?,
        }
        if abort.is_aborted() {
            return Err(format!("{LABEL} execution exceeded its deadline"));
        }
        self.validate_bug1190_son_provenance(deck, role)?;
        if abort.is_aborted() {
            return Err(format!("{LABEL} final provenance exceeded its deadline"));
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

    fn bug1190_fixture(label: &str) -> (tempfile::TempDir, XyceDeck) {
        let temporary = tempfile::Builder::new()
            .prefix(&format!("rspice-xyce-bug1190-son-{label}-"))
            .tempdir()
            .expect("create BUG1190 fixture root");
        let root = temporary.path();
        let family = root.join(FAMILY_DIRECTORY);
        fs::create_dir_all(&family).expect("create BUG1190 fixture family");
        let canonical = corpus_root();
        for name in [
            "bsim3.cir",
            "bsim3_modpar.cir",
            "bsim4.cir",
            "bsim4_modpar.cir",
            "diode.cir",
            "diodeRef.cir",
            "mutInd1.cir",
            "mutInd1_baseline.cir",
            "mutInd2.cir",
            "mutInd2_baseline.cir",
        ] {
            fs::copy(
                canonical.join(FAMILY_DIRECTORY).join(name),
                family.join(name),
            )
            .expect("copy BUG1190 family member");
        }
        fs::write(
            root.join(HARNESS_MANIFEST_FILE),
            [
                "Netlists/Certification_Tests/BUG_1190_SON/bsim3.cir\trequires_upstream_wrapper",
                "Netlists/Certification_Tests/BUG_1190_SON/bsim4.cir\trequires_upstream_wrapper",
                "Netlists/Certification_Tests/BUG_1190_SON/diode.cir\trequires_upstream_wrapper",
                "Netlists/Certification_Tests/BUG_1190_SON/mutInd1.cir\trequires_upstream_wrapper",
                "Netlists/Certification_Tests/BUG_1190_SON/mutInd2.cir\trequires_upstream_wrapper",
            ]
            .join("\n")
                + "\n",
        )
        .expect("write BUG1190 fixture harness manifest");
        fs::write(
            root.join(UPSTREAM_EXCLUSIONS_MANIFEST_FILE),
            [
                format!("schema_version\t{UPSTREAM_EXCLUSIONS_SCHEMA_VERSION}"),
                format!("source_commit\t{UPSTREAM_EXCLUSIONS_SOURCE_COMMIT}"),
                format!("source_netlists_tree\t{UPSTREAM_EXCLUSIONS_SOURCE_NETLISTS_TREE}"),
                format!("{}\t{EXCLUSION_SOURCE}\t{RSPICE_INDEPENDENTLY_QUALIFIED_DISPOSITION}\t{CONTROL_CONTRACT}", Bug1190SonRole::Bsim3Control.path()),
                format!("{}\t{EXCLUSION_SOURCE}\t{RSPICE_INDEPENDENTLY_QUALIFIED_DISPOSITION}\t{CONTROL_CONTRACT}", Bug1190SonRole::Bsim4Control.path()),
                format!("{}\t{EXCLUSION_SOURCE}\t{RSPICE_INDEPENDENTLY_QUALIFIED_DISPOSITION}\t{CONTROL_CONTRACT}", Bug1190SonRole::DiodeControl.path()),
                format!("Netlists/Certification_Tests/BUG_1190_SON/mutInd1_baseline.cir\t{EXCLUSION_SOURCE}\t{RSPICE_INDEPENDENTLY_QUALIFIED_DISPOSITION}\t{XYCE_BUG1190_MUTUAL_INDUCTOR_BASELINE_CONTRACT}"),
                format!("Netlists/Certification_Tests/BUG_1190_SON/mutInd2_baseline.cir\t{EXCLUSION_SOURCE}\t{RSPICE_INDEPENDENTLY_QUALIFIED_DISPOSITION}\t{XYCE_BUG1190_MUTUAL_INDUCTOR_BASELINE_CONTRACT}"),
            ]
            .join("\n")
                + "\n",
        )
        .expect("write BUG1190 fixture exclusion manifest");
        let role = Bug1190SonRole::Bsim3Owner;
        let deck = XyceDeck {
            path: root.join(role.path()),
            relative_path: role.path().to_string(),
            section: XyceDeckSection::Netlists,
        };
        (temporary, deck)
    }

    #[test]
    fn bug1190_son_roles_paths_and_contracts_are_bijective() {
        let records = Bug1190SonRole::ALL
            .into_iter()
            .map(Bug1190SonRole::record)
            .collect::<BTreeSet<_>>();
        assert_eq!(records.len(), Bug1190SonRole::ALL.len());
        for role in Bug1190SonRole::ALL {
            assert_eq!(Bug1190SonRole::for_record(role.path()), Some(role));
            assert_eq!(
                role.contract(),
                if role.is_owner() {
                    OWNER_CONTRACT
                } else {
                    CONTROL_CONTRACT
                }
            );
        }
    }

    #[test]
    fn bug1190_son_historical_and_retained_provenance_is_exact() {
        XyceTestRunner::validate_bug1190_son_historical_provenance()
            .expect("historical BUG1190 wrapper provenance remains exact");
        let root = corpus_root();
        let runner = XyceTestRunner::new(&root, XyceRunnerConfig::default());
        for role in Bug1190SonRole::ALL {
            let deck = XyceDeck {
                path: root.join(role.path()),
                relative_path: role.path().to_string(),
                section: XyceDeckSection::Netlists,
            };
            runner
                .validate_bug1190_son_provenance(&deck, role)
                .unwrap_or_else(|error| panic!("{role:?} provenance failed: {error}"));
        }
    }

    #[test]
    fn bug1190_son_provenance_rejects_source_census_manifest_exclusion_and_output_drift() {
        let (temporary, deck) = bug1190_fixture("canonical");
        XyceTestRunner::new(temporary.path(), XyceRunnerConfig::default())
            .validate_bug1190_son_provenance(&deck, Bug1190SonRole::Bsim3Owner)
            .expect("canonical BUG1190 fixture passes");

        let (temporary, deck) = bug1190_fixture("source");
        fs::write(
            temporary.path().join(Bug1190SonRole::DiodeOwner.path()),
            "* changed\n",
        )
        .expect("mutate BUG1190 source");
        assert!(
            XyceTestRunner::new(temporary.path(), XyceRunnerConfig::default())
                .validate_bug1190_son_provenance(&deck, Bug1190SonRole::Bsim3Owner)
                .is_err()
        );

        let (temporary, deck) = bug1190_fixture("census");
        fs::write(
            temporary.path().join(FAMILY_DIRECTORY).join("invented.cir"),
            "* invented\n",
        )
        .expect("add BUG1190 source member");
        assert!(
            XyceTestRunner::new(temporary.path(), XyceRunnerConfig::default())
                .validate_bug1190_son_provenance(&deck, Bug1190SonRole::Bsim3Owner)
                .is_err()
        );

        let (temporary, deck) = bug1190_fixture("manifest");
        let manifest = temporary.path().join(HARNESS_MANIFEST_FILE);
        let text = fs::read_to_string(&manifest).expect("read BUG1190 harness manifest");
        fs::write(
            &manifest,
            text.replacen(
                "Netlists/Certification_Tests/BUG_1190_SON/bsim3.cir\trequires_upstream_wrapper",
                "Netlists/Certification_Tests/BUG_1190_SON/bsim3_modpar.cir\trequires_upstream_wrapper",
                1,
            ),
        )
        .expect("mutate BUG1190 harness owner");
        assert!(
            XyceTestRunner::new(temporary.path(), XyceRunnerConfig::default())
                .validate_bug1190_son_provenance(&deck, Bug1190SonRole::Bsim3Owner)
                .is_err()
        );

        let (temporary, deck) = bug1190_fixture("exclusion");
        let exclusions = temporary.path().join(UPSTREAM_EXCLUSIONS_MANIFEST_FILE);
        let text = fs::read_to_string(&exclusions).expect("read BUG1190 exclusions");
        fs::write(
            &exclusions,
            text.replacen(CONTROL_CONTRACT, OWNER_CONTRACT, 1),
        )
        .expect("mutate BUG1190 control qualification");
        assert!(
            XyceTestRunner::new(temporary.path(), XyceRunnerConfig::default())
                .validate_bug1190_son_provenance(&deck, Bug1190SonRole::Bsim3Owner)
                .is_err()
        );

        let (temporary, deck) = bug1190_fixture("output");
        fs::create_dir_all(
            temporary
                .path()
                .join("OutputData/Certification_Tests/BUG_1190_SON"),
        )
        .expect("create fabricated BUG1190 numerical gold");
        assert!(
            XyceTestRunner::new(temporary.path(), XyceRunnerConfig::default())
                .validate_bug1190_son_provenance(&deck, Bug1190SonRole::Bsim3Owner)
                .is_err()
        );

        let (temporary, deck) = bug1190_fixture("sidecar");
        fs::write(
            temporary
                .path()
                .join(format!("{}.prn", Bug1190SonRole::Bsim3Owner.path())),
            "invented output\n",
        )
        .expect("create fabricated BUG1190 wrapper sidecar");
        assert!(
            XyceTestRunner::new(temporary.path(), XyceRunnerConfig::default())
                .validate_bug1190_son_provenance(&deck, Bug1190SonRole::Bsim3Owner)
                .is_err()
        );
    }

    #[test]
    fn bug1190_son_all_six_plans_preserve_authored_projection_and_step_shape() {
        let root = corpus_root();
        let runner = XyceTestRunner::new(&root, XyceRunnerConfig::default());
        for role in Bug1190SonRole::ALL {
            if role.pair() == Bug1190SonPair::Diode {
                runner
                    .validate_bug1190_son_tran_plan(role)
                    .unwrap_or_else(|error| panic!("{role:?} plan failed: {error}"));
            } else {
                let source =
                    fs::read_to_string(root.join(role.path())).expect("read BUG1190 DC deck");
                runner
                    .validate_bug1190_son_dc_plan(role, &source)
                    .unwrap_or_else(|error| panic!("{role:?} plan failed: {error}"));
            }
        }
    }

    #[test]
    fn bug1190_son_oracle_rejects_an_expired_deadline() {
        let root = corpus_root();
        let runner = XyceTestRunner::new(
            &root,
            XyceRunnerConfig {
                max_time_per_test_ms: 1,
                ..XyceRunnerConfig::default()
            },
        );
        let role = Bug1190SonRole::Bsim3Owner;
        let deck = XyceDeck {
            path: root.join(role.path()),
            relative_path: role.path().to_string(),
            section: XyceDeckSection::Netlists,
        };
        assert!(
            runner
                .validate_bug1190_son_oracle(&deck, role, Instant::now() - Duration::from_secs(1),)
                .is_err()
        );
    }

    #[test]
    fn bug1190_son_phase_safe_equivalence_rejects_current_and_time_mutations() {
        let root = corpus_root();
        let runner = XyceTestRunner::new(&root, XyceRunnerConfig::default());
        for (probe, control_value, owner_value) in [("i(vds)", 1.0e-3, 2.0e-3), ("TIME", 0.1, 0.2)]
        {
            let control = XycePrnTable {
                columns: vec![probe.to_string()],
                rows: vec![vec![control_value]],
            };
            let owner = XycePrnTable {
                columns: control.columns.clone(),
                rows: vec![vec![owner_value]],
            };
            assert!(
                runner
                    .compare_release_7_10_file_compare_tables(
                        &control,
                        &owner,
                        XyceFileCompareTolerance::BUG1190_SON_PROCESS_PARAMETER,
                    )
                    .expect("historical comparator accepts the preserved phase-clause case")
                    .is_empty(),
                "fixture must expose the historical phase-clause weakness for {probe}"
            );
            assert!(
                XyceTestRunner::validate_bug1190_son_phase_safe_equivalence(&control, &owner)
                    .is_err(),
                "phase-safe comparison must reject the mutated {probe} value"
            );
        }
    }
}
