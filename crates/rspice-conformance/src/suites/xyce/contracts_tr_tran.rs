use super::*;
use rspice_core::netlist::SourceSpec;
use std::io::Read as _;

const LABEL: &str = "TR_TRAN transient-analysis alias relation";
const UPSTREAM_COMMIT: &str = "d6e278e371ec2f3df1325dcff4552e585bc7ecc1";
const UPSTREAM_TAG: &str = "Release-7.10.0";
const OWNER_PATH: &str = "Netlists/TR_TRAN/tr2.cir";
const REFERENCE_PATH: &str = "Netlists/TR_TRAN/tran2.cir";
const OWNER_RECORD: &str = "netlists/tr_tran/tr2.cir";
const REFERENCE_RECORD: &str = "netlists/tr_tran/tran2.cir";
const EXCLUSION_SOURCE: &str = "Netlists/TR_TRAN/exclude";
const OWNER_CONTRACT: &str = "tr_tran_short_alias_wrapper_owner";
const REFERENCE_CONTRACT: &str = "tr_tran_long_form_reference";

const HISTORICAL_RECORD_COUNT: usize = 11;
const HISTORICAL_RECORD_BYTES: usize = 2_415;
const HISTORICAL_RECORDS_SHA256: &str =
    "5724b02adf03208fc8d7b0dff7290adc977eaa2b31c260ff9a5fe4d8375fdf0d";
const HISTORICAL_RECORDS_BLAKE3: &str =
    "b88b6ce7cb8a1aa0fa9ecc642675db380a22f6d0a1305d156a26f2cf4e342e86";
const HISTORICAL_ARTIFACTS: [(&str, usize, &str, &str); 11] = [
    (
        "Netlists/TR_TRAN/CMakeLists.txt",
        2_828,
        "204b0557a60b11af80508efb38d85d16e050c1d7e51f4c6147c3f1c2e3781ba3",
        "822bc52a3a356b884e7dcd6a9d0bd6feb242f8158ed8e7be1defa44c04d9365d",
    ),
    (
        "Netlists/TR_TRAN/Manifest.txt",
        68,
        "9a09a45419d5abeaf91ff4aaa60214284176a6821cce71dafb1be716f719c9d5",
        "45e070b406528ba8380beba9899cdeaa05dba0761889f3a1249f4868bd2399fc",
    ),
    (
        "Netlists/TR_TRAN/exclude",
        21,
        "e96efa342094e319378ced068ff7096a807e90d159618a15e0c55c827dda57bf",
        "71bf8120c4e9598520ec6917bc5fa54b4c66bf681f35f6afc3231c6360826026",
    ),
    (
        "Netlists/TR_TRAN/tags",
        55,
        "e1eccba8e2df683ea43f67883aeed9b4d60e61d96f20b91e4f3dd933edc65221",
        "95b6cbada9546bf7edc281a229667bc84bae0c4a08f92e23c5f6b923ea85a2bf",
    ),
    (
        OWNER_PATH,
        384,
        "2d530a48ebee9bbd6ee04bdfcdb897ad12f89d8204037aa742cc5508696cc186",
        "38b12c464b4ca3bdc0485a105ed120048100412b2a5e1673421491182adaf2fa",
    ),
    (
        "Netlists/TR_TRAN/tr2.cir.sh",
        1_484,
        "28479fffe230c1172842105cd4f96c4f555ce0a2444ae955bb8809e86d7d3954",
        "749b8d960077662ee670405ebd3bcfe785f8ae452e89cdf9bc4aa3205636266c",
    ),
    (
        REFERENCE_PATH,
        388,
        "e6e631e216f9044c61fd1c2ced04cfdbc1e5bd753c461f611031b3e04247e27a",
        "a37090eaf9b4ebb7bc0e5df3b8787679473c0ba80ab85d1aa794eaa20c1da547",
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

const RETAINED_RECORD_COUNT: usize = 4;
const RETAINED_RECORD_BYTES: usize = 571;
const RETAINED_RECORDS_SHA256: &str =
    "0cccb2148eaec27764d028ae587d84a27032dac98b2620f7ff1849dd7b863a69";
const RETAINED_RECORDS_BLAKE3: &str =
    "1078d354aa84edc1d4830e0f2ec7b924c0d652ca81d905f08fc79fd4f5eb56b8";
const RETAINED_ARTIFACTS: [(&str, usize, &str, &str); 4] = [
    (
        "tr.cir",
        5_753,
        "220639054ba9fe5b7bee2775455ae925566c33539cd151880d45f27023b26953",
        "99867d43259d6196332310733bbd2dcc2e17787c63299c66136d1875726d8b0d",
    ),
    (
        "tr2.cir",
        384,
        "2d530a48ebee9bbd6ee04bdfcdb897ad12f89d8204037aa742cc5508696cc186",
        "38b12c464b4ca3bdc0485a105ed120048100412b2a5e1673421491182adaf2fa",
    ),
    (
        "tran.cir",
        5_757,
        "eea770722f5ce9986e5b7ae475913a83383a65eb1cb9a4ed3f67e9d64f5ee123",
        "e9df61624b221fe9ea554483c8772e503f8332d45d2c757797e6ea21c7cbdd81",
    ),
    (
        "tran2.cir",
        388,
        "e6e631e216f9044c61fd1c2ced04cfdbc1e5bd753c461f611031b3e04247e27a",
        "a37090eaf9b4ebb7bc0e5df3b8787679473c0ba80ab85d1aa794eaa20c1da547",
    ),
];

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(super) enum TrTranRole {
    ShortOwner,
    LongReference,
}

impl TrTranRole {
    const ALL: [Self; 2] = [Self::ShortOwner, Self::LongReference];

    pub(super) fn for_record(record: &str) -> Option<Self> {
        let record = XyceTestRunner::normalize_manifest_key(record);
        Self::ALL.into_iter().find(|role| role.record() == record)
    }

    pub(super) fn contract(self) -> &'static str {
        match self {
            Self::ShortOwner => OWNER_CONTRACT,
            Self::LongReference => REFERENCE_CONTRACT,
        }
    }

    fn path(self) -> &'static str {
        match self {
            Self::ShortOwner => OWNER_PATH,
            Self::LongReference => REFERENCE_PATH,
        }
    }

    fn record(self) -> &'static str {
        match self {
            Self::ShortOwner => OWNER_RECORD,
            Self::LongReference => REFERENCE_RECORD,
        }
    }

    fn file_name(self) -> &'static str {
        self.path().rsplit('/').next().expect("TR_TRAN file name")
    }
}

impl XyceTestRunner {
    pub(super) fn tr_tran_historical_provenance_records() -> Vec<String> {
        let mut records = HISTORICAL_ARTIFACTS
            .into_iter()
            .map(|(path, bytes, sha256, blake3)| {
                format!("{UPSTREAM_COMMIT}\t{UPSTREAM_TAG}\t{path}\t{bytes}\t{sha256}\t{blake3}")
            })
            .collect::<Vec<_>>();
        records.sort();
        records
    }

    pub(super) fn validate_tr_tran_historical_provenance() -> Result<(), String> {
        let records = Self::tr_tran_historical_provenance_records();
        let stream = records.join("\n");
        let sha256 = format!("{:x}", Sha256::digest(stream.as_bytes()));
        let blake3 = blake3::hash(stream.as_bytes()).to_hex().to_string();
        if records.len() != HISTORICAL_RECORD_COUNT
            || stream.len() != HISTORICAL_RECORD_BYTES
            || sha256 != HISTORICAL_RECORDS_SHA256
            || blake3 != HISTORICAL_RECORDS_BLAKE3
        {
            return Err(format!(
                "{LABEL} Release provenance changed: records={}/{}, sha256={sha256}, blake3={blake3}",
                records.len(),
                stream.len()
            ));
        }
        Ok(())
    }

    fn validate_tr_tran_directory(&self) -> Result<BTreeMap<String, Vec<u8>>, String> {
        let directory = self.root.join("Netlists/TR_TRAN");
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

    fn validate_tr_tran_provenance(
        &self,
        deck: &XyceDeck,
        role: TrTranRole,
    ) -> Result<BTreeMap<String, Vec<u8>>, String> {
        Self::validate_tr_tran_historical_provenance()?;
        if deck.section != XyceDeckSection::Netlists
            || Self::normalize_manifest_key(&deck.relative_path) != role.record()
            || Self::normalize_manifest_key(&self.relative_key(&deck.path)) != role.record()
            || !Self::same_path(&deck.path, &self.root.join(role.path()))
        {
            return Err(format!(
                "recognized {LABEL} role is not at its canonical path"
            ));
        }
        let prefix = "netlists/tr_tran/";
        let owners = Self::load_upstream_wrapper_decks(&self.root)
            .into_iter()
            .filter(|record| record.starts_with(prefix))
            .collect::<BTreeSet<_>>();
        if owners
            != BTreeSet::from([
                "netlists/tr_tran/tr.cir".to_string(),
                OWNER_RECORD.to_string(),
            ])
        {
            return Err(format!("{LABEL} wrapper ownership changed: {owners:?}"));
        }
        let exclusions = Self::load_upstream_exclusions(&self.root)
            .map_err(|error| format!("{LABEL} exclusion manifest is invalid: {error}"))?;
        for owner in ["netlists/tr_tran/tr.cir", OWNER_RECORD] {
            if exclusions.contains_key(owner) {
                return Err(format!(
                    "{LABEL} wrapper owner {owner} must not be excluded"
                ));
            }
        }
        let family = exclusions
            .iter()
            .filter(|(record, _)| record.starts_with(prefix))
            .collect::<BTreeMap<_, _>>();
        if family.len() != 2 {
            return Err(format!("{LABEL} exclusion census changed"));
        }
        let long = family
            .get(&"netlists/tr_tran/tran.cir".to_string())
            .copied()
            .ok_or_else(|| format!("{LABEL} lost tran.cir exclusion"))?;
        if long.source != EXCLUSION_SOURCE
            || !matches!(long.disposition, XyceUpstreamExclusionDisposition::Excluded)
        {
            return Err(format!("{LABEL} tran.cir exclusion role changed"));
        }
        let reference = family
            .get(&REFERENCE_RECORD.to_string())
            .copied()
            .ok_or_else(|| format!("{LABEL} lost tran2.cir reference"))?;
        if reference.source != EXCLUSION_SOURCE
            || !matches!(&reference.disposition,
                XyceUpstreamExclusionDisposition::RspiceIndependentlyQualified { expected_contract }
                    if expected_contract == REFERENCE_CONTRACT)
        {
            return Err(format!("{LABEL} tran2.cir qualification changed"));
        }
        let members = self.validate_tr_tran_directory()?;
        for role in TrTranRole::ALL {
            self.reject_wrapper_output_artifacts(&self.root.join(role.path()))
                .map_err(|error| format!("{LABEL} {} {error}", role.file_name()))?;
        }
        let output = self.root.join("OutputData/TR_TRAN");
        match fs::symlink_metadata(&output) {
            Err(error) if error.kind() == std::io::ErrorKind::NotFound => {}
            Err(error) => return Err(format!("failed to inspect {LABEL} OutputData: {error}")),
            Ok(_) => return Err(format!("{LABEL} must not acquire invented numerical gold")),
        }
        Ok(members)
    }

    fn tr_tran_nodes_match(actual: &[String], expected: &[&str]) -> bool {
        actual.len() == expected.len()
            && actual
                .iter()
                .zip(expected)
                .all(|(actual, expected)| actual.eq_ignore_ascii_case(expected))
    }

    fn validate_tr_tran_typed_contract(
        &self,
        role: TrTranRole,
        source: &str,
        path: &Path,
    ) -> Result<(XyceStaticTranPlan, Netlist, XyceAnalyticRcSpecification), String> {
        let purpose = match role {
            TrTranRole::ShortOwner => XyceStaticTranPlanPurpose::AnalyticOracle,
            TrTranRole::LongReference => XyceStaticTranPlanPurpose::RelationalFamily,
        };
        let plan = self.static_tran_plan_for_path_with_purpose(path, purpose)?;
        let expected_contract = match role {
            TrTranRole::ShortOwner => XyceStaticTranContract::WrapperStatic,
            TrTranRole::LongReference => XyceStaticTranContract::PlainStatic,
        };
        if plan.deck_path != path
            || plan.source.as_bytes() != source.as_bytes()
            || !matches!(plan.oracle, XyceStaticTranOracle::None)
            || plan.contract != expected_contract
            || plan.comparison_mode != XyceStaticTranComparisonMode::Pointwise
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
                role.file_name()
            ));
        }
        let comp_tolerances = Self::xyce_verify_comp_tolerances(source, &["V(1)".to_string()])?;
        if comp_tolerances.as_slice().first().is_none_or(|tolerance| {
            comp_tolerances.len() != 1
                || tolerance.relative.to_bits() != 5.0e-3f64.to_bits()
                || tolerance.absolute.to_bits() != 5.0e-7f64.to_bits()
                || tolerance.zero.to_bits() != XYCE_VERIFY_DEFAULT_ZERO_TOLERANCE.to_bits()
                || tolerance.absolute_difference.to_bits()
                    != XYCE_VERIFY_DEFAULT_ABSOLUTE_DIFFERENCE_TOLERANCE.to_bits()
                || tolerance.offset.to_bits() != 0.0f64.to_bits()
        }) {
            return Err(format!(
                "{LABEL} {} *COMP tolerance changed",
                role.file_name()
            ));
        }
        let netlist = Self::parse_xyce_netlist(source, path)
            .map_err(|error| format!("{LABEL} {} no longer parses: {error}", role.file_name()))?;
        if netlist.elements.len() != 3
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
            || !Self::analytic_timeint_only_options_match(&netlist.options, None, None, None, None)
        {
            return Err(format!(
                "{LABEL} {} typed envelope changed",
                role.file_name()
            ));
        }
        let [capacitor, resistor, voltage] = netlist.elements.as_slice() else {
            unreachable!("TR_TRAN element count checked")
        };
        if !capacitor.name.eq_ignore_ascii_case("C1")
            || capacitor.provenance != ElementProvenance::Authored
            || !Self::tr_tran_nodes_match(&capacitor.nodes, &["1", "0"])
            || !matches!(&capacitor.kind, ElementKind::Capacitor {
                value, value_expr: None, initial_voltage: Some(initial_voltage), model: None,
                instance_params, deferred_params,
            } if value.to_bits() == 1.0e-6f64.to_bits()
                && initial_voltage.to_bits() == 1.0f64.to_bits()
                && instance_params.is_empty() && deferred_params.is_empty())
            || !resistor.name.eq_ignore_ascii_case("R1")
            || resistor.provenance != ElementProvenance::Authored
            || !Self::tr_tran_nodes_match(&resistor.nodes, &["1", "2"])
            || !matches!(&resistor.kind, ElementKind::Resistor {
                value, value_expr: None, model: None, instance_params, deferred_params,
            } if value.to_bits() == 1.0e3f64.to_bits()
                && instance_params.is_empty() && deferred_params.is_empty())
            || !voltage.name.eq_ignore_ascii_case("V1")
            || voltage.provenance != ElementProvenance::Authored
            || !Self::tr_tran_nodes_match(&voltage.nodes, &["2", "0"])
            || !matches!(&voltage.kind, ElementKind::VoltageSource(SourceSpec::Dc(value))
                if value.to_bits() == 0.0f64.to_bits())
        {
            return Err(format!(
                "{LABEL} {} passive topology changed",
                role.file_name()
            ));
        }
        if !matches!(&netlist.analyses[0], AnalysisCommand::Tran {
            step, stop, start: None, max_step: None, uic: false,
        } if step.to_bits() == 0.0f64.to_bits() && stop.to_bits() == 5.0e-3f64.to_bits())
        {
            return Err(format!(
                "{LABEL} {} typed transient changed",
                role.file_name()
            ));
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
            || !matches!(netlist.saves.signals.as_slice(),
                [rspice_core::netlist::SaveSignal::Voltage(node)] if node == "1")
        {
            return Err(format!(
                "{LABEL} {} PRINT request changed",
                role.file_name()
            ));
        }
        Ok((
            plan,
            netlist,
            XyceAnalyticRcSpecification {
                output_node: "1".to_string(),
                source_value: 0.0,
                initial_voltage: 1.0,
                resistance: 1.0e3,
                capacitance: 1.0e-6,
                time_constant: 1.0e-3,
            },
        ))
    }

    fn validate_tr_tran_table(
        role: TrTranRole,
        table: &XycePrnTable,
        stop: Value,
    ) -> Result<(), String> {
        if table.columns.len() != 3
            || !table.columns[0].eq_ignore_ascii_case("Index")
            || !table.columns[1].eq_ignore_ascii_case("TIME")
            || !table.columns[2].eq_ignore_ascii_case("V(1)")
            || table.rows.len() < 3
        {
            return Err(format!(
                "{LABEL} {} table shape changed: {table:?}",
                role.file_name()
            ));
        }
        let mut prior_time = None;
        for (index, row) in table.rows.iter().enumerate() {
            if row.len() != 3
                || row.iter().any(|value| !value.is_finite())
                || row[0].to_bits() != (index as Value).to_bits()
                || prior_time.is_some_and(|prior| row[1] <= prior)
            {
                return Err(format!(
                    "{LABEL} {} row {index} is malformed",
                    role.file_name()
                ));
            }
            prior_time = Some(row[1]);
        }
        let first = &table.rows[0];
        let last = table.rows.last().expect("TR_TRAN table is nonempty");
        if first[1].to_bits() != 0.0f64.to_bits()
            || (first[2] - 1.0).abs() > 1.0e-12
            || (last[1] - stop).abs() > stop.abs().max(1.0) * 1.0e-12
            || last[2] >= 0.02
        {
            return Err(format!(
                "{LABEL} {} domain or decay changed",
                role.file_name()
            ));
        }
        Ok(())
    }

    pub(super) fn validate_tr_tran_oracle(
        &self,
        deck: &XyceDeck,
        role: TrTranRole,
        start: Instant,
    ) -> Result<(), String> {
        let abort = DeadlineAbort::new(start, self.config.max_time_per_test_ms.max(1));
        if abort.is_aborted() {
            return Err(format!("{LABEL} deadline expired before provenance"));
        }
        let members = self.validate_tr_tran_provenance(deck, role)?;
        let owner_bytes = members
            .get("tr2.cir")
            .ok_or_else(|| format!("{LABEL} lost tr2.cir"))?;
        let reference_bytes = members
            .get("tran2.cir")
            .ok_or_else(|| format!("{LABEL} lost tran2.cir"))?;
        let owner_source = std::str::from_utf8(owner_bytes)
            .map_err(|error| format!("{LABEL} tr2.cir is not UTF-8: {error}"))?;
        let reference_source = std::str::from_utf8(reference_bytes)
            .map_err(|error| format!("{LABEL} tran2.cir is not UTF-8: {error}"))?;
        let normalized_owner = owner_source
            .replace(".print TR v(1)", ".print TRAN v(1)")
            .replace(".TR 0 5ms", ".TRAN 0 5ms");
        if normalized_owner.as_bytes() != reference_source.as_bytes() {
            return Err(format!(
                "{LABEL} sources no longer differ only by TR/TRAN aliases"
            ));
        }

        let run = |member_role: TrTranRole| {
            let bytes = members
                .get(member_role.file_name())
                .ok_or_else(|| format!("{LABEL} lost {}", member_role.file_name()))?;
            let source = std::str::from_utf8(bytes)
                .map_err(|error| format!("{LABEL} source is not UTF-8: {error}"))?;
            let (plan, _parsed, specification) = self.validate_tr_tran_typed_contract(
                member_role,
                source,
                &self.root.join(member_role.path()),
            )?;
            let (executed, result) = self
                .run_transient_family_plan(&plan, start, None, None)
                .map_err(|error| match error {
                    SimulationError::Aborted => format!("{LABEL} execution exceeded deadline"),
                    other => format!("{LABEL} execution failed: {other}"),
                })?;
            let table = Self::transient_family_result_to_prn_table(&plan, &executed, &result)?;
            Self::validate_tr_tran_table(member_role, &table, plan.tran.stop)?;
            Self::validate_analytic_rc_initial_sample(&table, &specification)?;
            Self::validate_analytic_rc_complete_time_domain(&table, plan.tran.stop)?;
            let analytic = Self::analytic_rc_reference_table(&table, &specification)?;
            let tolerance = XyceVerifyTransientTolerance {
                relative: 5.0e-3,
                absolute: 5.0e-7,
                zero: XYCE_VERIFY_DEFAULT_ZERO_TOLERANCE,
                absolute_difference: XYCE_VERIFY_DEFAULT_ABSOLUTE_DIFFERENCE_TOLERANCE,
                offset: 0.0,
            };
            let mismatches = self.compare_xyce_verify_transient_tables_with_uniform_tolerance(
                &analytic,
                &table,
                tolerance,
                XYCE_DEFAULT_PRN_SCIENTIFIC_PRECISION,
            )?;
            if !mismatches.is_empty() {
                return Err(format!(
                    "{LABEL} {} violated the analytic RC oracle: {mismatches:?}",
                    member_role.file_name()
                ));
            }
            Ok::<_, String>(table)
        };

        let owner = run(TrTranRole::ShortOwner)?;
        if abort.is_aborted() {
            return Err(format!("{LABEL} deadline expired between independent runs"));
        }
        let reference = run(TrTranRole::LongReference)?;
        let mismatches = self.compare_serialized_default_prn_tables(&owner, &reference)?;
        if !mismatches.is_empty() {
            return Err(format!(
                "{LABEL} failed its historical byte relation: {mismatches:?}"
            ));
        }
        self.validate_tr_tran_provenance(deck, role)?;
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

    fn fixture(label: &str) -> (tempfile::TempDir, XyceDeck, XyceTestRunner) {
        let temporary = tempfile::Builder::new()
            .prefix(&format!("rspice-xyce-tr-tran-{label}-"))
            .tempdir()
            .expect("create TR_TRAN fixture");
        let root = temporary.path();
        let family = root.join("Netlists/TR_TRAN");
        fs::create_dir_all(&family).expect("create TR_TRAN family");
        let canonical = corpus_root().join("Netlists/TR_TRAN");
        for (name, ..) in RETAINED_ARTIFACTS {
            fs::copy(canonical.join(name), family.join(name)).expect("copy TR_TRAN member");
        }
        fs::write(
            root.join(HARNESS_MANIFEST_FILE),
            format!(
                "Netlists/TR_TRAN/tr.cir\t{REQUIRES_UPSTREAM_WRAPPER_CONTRACT}\n{OWNER_PATH}\t{REQUIRES_UPSTREAM_WRAPPER_CONTRACT}\n"
            ),
        )
        .expect("write wrapper manifest");
        fs::write(
            root.join(UPSTREAM_EXCLUSIONS_MANIFEST_FILE),
            format!(
                "schema_version\t{UPSTREAM_EXCLUSIONS_SCHEMA_VERSION}\nsource_commit\t{UPSTREAM_EXCLUSIONS_SOURCE_COMMIT}\nsource_netlists_tree\t{UPSTREAM_EXCLUSIONS_SOURCE_NETLISTS_TREE}\nNetlists/TR_TRAN/tran.cir\t{EXCLUSION_SOURCE}\t{UPSTREAM_EXCLUDED_DISPOSITION}\n{REFERENCE_PATH}\t{EXCLUSION_SOURCE}\t{RSPICE_INDEPENDENTLY_QUALIFIED_DISPOSITION}\t{REFERENCE_CONTRACT}\n"
            ),
        )
        .expect("write exclusion manifest");
        let runner = XyceTestRunner::new(root, XyceRunnerConfig::default());
        let deck = XyceDeck {
            path: root.join(OWNER_PATH),
            section: XyceDeckSection::Netlists,
            relative_path: OWNER_PATH.to_string(),
        };
        (temporary, deck, runner)
    }

    #[test]
    fn tr_tran_historical_provenance_is_exact() {
        XyceTestRunner::validate_tr_tran_historical_provenance()
            .expect("TR_TRAN Release provenance remains exact");
    }

    #[test]
    fn tr_tran_both_roles_parse_and_execute_as_transient() {
        let runner = XyceTestRunner::new(corpus_root(), XyceRunnerConfig::default());
        for role in TrTranRole::ALL {
            let path = corpus_root().join(role.path());
            let source = fs::read_to_string(&path).expect("read TR_TRAN member");
            let (plan, netlist, _) = runner
                .validate_tr_tran_typed_contract(role, &source, &path)
                .expect("TR and TRAN must share typed transient semantics");
            assert!(matches!(
                netlist.analyses.as_slice(),
                [AnalysisCommand::Tran { .. }]
            ));
            assert_eq!(
                netlist.output_requests[0].analysis,
                Some(rspice_core::netlist::OutputAnalysisKind::Tran)
            );
            assert_eq!(plan.tran.stop.to_bits(), 5.0e-3f64.to_bits());
        }
    }

    #[test]
    fn tr_tran_live_oracle_executes_both_roles() {
        let runner = XyceTestRunner::new(corpus_root(), XyceRunnerConfig::default());
        for role in TrTranRole::ALL {
            let deck = XyceDeck {
                path: corpus_root().join(role.path()),
                section: XyceDeckSection::Netlists,
                relative_path: role.path().to_string(),
            };
            runner
                .validate_tr_tran_oracle(&deck, role, Instant::now())
                .expect("TR/TRAN relation should execute natively");
        }
    }

    #[test]
    fn tr_tran_analytic_oracle_rejects_shared_wrong_decay() {
        let mut table = XycePrnTable {
            columns: vec!["Index".into(), "TIME".into(), "V(1)".into()],
            rows: vec![
                vec![0.0, 0.0, 1.0],
                vec![1.0, 1.0e-3, (-1.0f64).exp()],
                vec![2.0, 5.0e-3, (-5.0f64).exp()],
            ],
        };
        XyceTestRunner::validate_tr_tran_table(TrTranRole::ShortOwner, &table, 5.0e-3)
            .expect("canonical decay-shaped table passes structural guard");
        table.rows[1][2] = 0.0;
        let specification = XyceAnalyticRcSpecification {
            output_node: "1".into(),
            source_value: 0.0,
            initial_voltage: 1.0,
            resistance: 1.0e3,
            capacitance: 1.0e-6,
            time_constant: 1.0e-3,
        };
        let reference = XyceTestRunner::analytic_rc_reference_table(&table, &specification)
            .expect("build counterfactual analytic table");
        let runner = XyceTestRunner::new(corpus_root(), XyceRunnerConfig::default());
        let mismatches = runner
            .compare_xyce_verify_transient_tables_with_uniform_tolerance(
                &reference,
                &table,
                XyceVerifyTransientTolerance {
                    relative: 5.0e-3,
                    absolute: 5.0e-7,
                    zero: XYCE_VERIFY_DEFAULT_ZERO_TOLERANCE,
                    absolute_difference: XYCE_VERIFY_DEFAULT_ABSOLUTE_DIFFERENCE_TOLERANCE,
                    offset: 0.0,
                },
                XYCE_DEFAULT_PRN_SCIENTIFIC_PRECISION,
            )
            .expect("compare counterfactual table");
        assert!(!mismatches.is_empty());
    }

    #[test]
    fn tr_tran_provenance_rejects_source_role_and_output_drift() {
        let (_temporary, deck, runner) = fixture("mutations");
        runner
            .validate_tr_tran_provenance(&deck, TrTranRole::ShortOwner)
            .expect("canonical fixture passes");
        fs::write(runner.root.join(REFERENCE_PATH), "* changed\n").expect("mutate reference");
        assert!(
            runner
                .validate_tr_tran_provenance(&deck, TrTranRole::ShortOwner)
                .is_err()
        );

        let (_temporary, deck, runner) = fixture("role");
        fs::write(
            runner.root.join(UPSTREAM_EXCLUSIONS_MANIFEST_FILE),
            format!(
                "schema_version\t{UPSTREAM_EXCLUSIONS_SCHEMA_VERSION}\nsource_commit\t{UPSTREAM_EXCLUSIONS_SOURCE_COMMIT}\nsource_netlists_tree\t{UPSTREAM_EXCLUSIONS_SOURCE_NETLISTS_TREE}\nNetlists/TR_TRAN/tran.cir\t{EXCLUSION_SOURCE}\t{UPSTREAM_EXCLUDED_DISPOSITION}\n{REFERENCE_PATH}\twrong/exclude\t{RSPICE_INDEPENDENTLY_QUALIFIED_DISPOSITION}\t{REFERENCE_CONTRACT}\n"
            ),
        )
        .expect("mutate role");
        assert!(
            runner
                .validate_tr_tran_provenance(&deck, TrTranRole::ShortOwner)
                .is_err()
        );

        let (_temporary, deck, runner) = fixture("output");
        fs::create_dir_all(runner.root.join("OutputData/TR_TRAN"))
            .expect("invent output directory");
        assert!(
            runner
                .validate_tr_tran_provenance(&deck, TrTranRole::ShortOwner)
                .is_err()
        );

        let (_temporary, deck, runner) = fixture("extra-member");
        fs::write(
            runner.root.join("Netlists/TR_TRAN/unreviewed.cir"),
            "unreviewed sibling\n.end\n",
        )
        .expect("invent extra family member");
        assert!(
            runner
                .validate_tr_tran_provenance(&deck, TrTranRole::ShortOwner)
                .is_err()
        );
    }

    #[test]
    fn tr_tran_typed_contract_rejects_analysis_probe_topology_and_domain_drift() {
        for (label, from, to) in [
            ("analysis", ".TR 0 5ms", ".DC V1 0 1 1"),
            ("probe", ".print TR v(1)", ".print TR v(2)"),
            ("capacitance", "c1 1 0 1uF IC=1", "c1 1 0 2uF IC=1"),
            ("stop", ".TR 0 5ms", ".TR 0 6ms"),
        ] {
            let (_temporary, _deck, runner) = fixture(label);
            let path = runner.root.join(OWNER_PATH);
            let canonical = fs::read_to_string(&path).expect("read canonical short form");
            let mutated = canonical.replacen(from, to, 1);
            assert_ne!(mutated, canonical, "mutation {label} must apply");
            fs::write(&path, &mutated).expect("write typed mutation");
            assert!(
                runner
                    .validate_tr_tran_typed_contract(TrTranRole::ShortOwner, &mutated, &path,)
                    .is_err(),
                "typed mutation {label} must fail closed"
            );
        }
    }

    #[test]
    fn tr_tran_expired_deadline_fails_closed() {
        let (_temporary, deck, mut runner) = fixture("deadline");
        runner.config.max_time_per_test_ms = 1;
        assert!(
            runner
                .validate_tr_tran_oracle(
                    &deck,
                    TrTranRole::ShortOwner,
                    Instant::now() - Duration::from_millis(10),
                )
                .is_err()
        );
    }
}
