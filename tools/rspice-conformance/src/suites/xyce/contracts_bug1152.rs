use super::*;

const LABEL: &str = "BUG_1152 device/node namespace separation";
const FAMILY_PREFIX: &str = "netlists/certification_tests/bug_1152/";
const FAMILY_DIRECTORY: &str = "Netlists/Certification_Tests/BUG_1152";
const OUTPUT_DIRECTORY: &str = "OutputData/Certification_Tests/BUG_1152";
const EXCLUSION_SOURCE: &str = "Netlists/Certification_Tests/BUG_1152/exclude";
const RLC_PATH: &str = "Netlists/Certification_Tests/BUG_1152/bug_1152.cir";
const COLLISION_PATH: &str = "Netlists/Certification_Tests/BUG_1152/bug_767b.cir";
const CONTROL_PATH: &str = "Netlists/Certification_Tests/BUG_1152/bug_767b_nodupes.cir";
const RLC_RECORD: &str = "netlists/certification_tests/bug_1152/bug_1152.cir";
const COLLISION_RECORD: &str = "netlists/certification_tests/bug_1152/bug_767b.cir";
const CONTROL_RECORD: &str = "netlists/certification_tests/bug_1152/bug_767b_nodupes.cir";
const RLC_CONTRACT: &str = "bug1152_rlc_device_node_name_analytic_tran_wrapper_owner";
const COLLISION_CONTRACT: &str = "bug1152_duplicate_device_node_name_relational_wrapper_owner";
const CONTROL_CONTRACT: &str = "bug1152_duplicate_device_node_name_relational_control";
const UPSTREAM_REGRESSION_COMMIT: &str = "d6e278e371ec2f3df1325dcff4552e585bc7ecc1";
const UPSTREAM_RELEASE_TAG: &str = "Release-7.10.0";
const HISTORICAL_RECORD_COUNT: usize = 12;
const HISTORICAL_RECORD_BYTES: usize = 2_895;
const HISTORICAL_RECORDS_SHA256: &str =
    "634b8e48265c780b2b85f80877b3882b0ee69c8e9da4c846240d0cc58637202b";
const HISTORICAL_RECORDS_BLAKE3: &str =
    "a52048b552a06787e43e66498c4fd573aa4f1f955f47aec7ed396bf750439db1";

// The family did not retain numerical gold. The first wrapper generated one
// analytic current column at runtime; the second compared two fresh simulator
// runs. Keep the removed wrappers, generator, metadata, and the exact Release
// comparator in the provenance boundary so this native reconstruction cannot
// silently drift into a different oracle.
const HISTORICAL_ARTIFACTS: [(&str, usize, &str, &str); 12] = [
    (
        "Netlists/Certification_Tests/BUG_1152/Manifest.txt",
        153,
        "b3a7a633b3e900d0dd9c5b727b2aed6c188daec1bd54bda7950c41df5c1a75df",
        "c7fa04329b2a0aaa3f9e4055acc2edb2be14b6e65060be128c0ec34e61a08378",
    ),
    (
        "Netlists/Certification_Tests/BUG_1152/README",
        1_005,
        "c6b9aecac4de0edb08654532238661d2523815e6a55012025aac715afbf5a2cb",
        "4e9c0c3f799a4bf90daea5c5950d92a0c0a7f9766033010e362862497ca43c1a",
    ),
    (
        RLC_PATH,
        1_205,
        "1f05eeb9d8c7f7c09f0c228b108e43a6fd9bf34ae659a62dca785bc64d63f999",
        "df2f75e06b363416d4ea41c7057810ee42b099fb2f4cd59c9e84f6bb47ad4897",
    ),
    (
        "Netlists/Certification_Tests/BUG_1152/bug_1152.cir.prn.gs.pl",
        547,
        "8301f6419c455cc245127c6dcabd30902cdcf2abbba5aaeb7eaa12651282d5ee",
        "8bd9e50fcc8296f11e65a9315b00040b45b4aec223733336db0a1b03fb56348b",
    ),
    (
        "Netlists/Certification_Tests/BUG_1152/bug_1152.cir.sh",
        1_267,
        "9cbfb7eda7b80fe7aa6c2bdd7f0b848fea1138e48db4237c316ff231dfa1ff5a",
        "a9a83794c16b0ef4d4c0371a93c0d94b15823262f377399e5a1ae0ee1aa82ecf",
    ),
    (
        "Netlists/Certification_Tests/BUG_1152/bug_1152.cir.tags",
        26,
        "1e6b928df7201ec2ef59e9058e3fd4fb216589d8245520003d5603a8f13b4c5b",
        "734e1696605bc661d095ac763edd46b3c764489f5166f85f03781e02239c8cba",
    ),
    (
        COLLISION_PATH,
        1_390,
        "39e8375c87e5e4910f14861b4714e1f6695c46da154c8e0fa5e157a688ee5a54",
        "ccdfd8e535d2bccf69ea36d8b43e878a45ddaf5124b4e730ed997740a692ff18",
    ),
    (
        "Netlists/Certification_Tests/BUG_1152/bug_767b.cir.sh",
        1_239,
        "59a857759d770ef21d2726cc795133b46f594ae52014dfc9ac482323517db02c",
        "05a16fbfb4eaa8ada23ab0f9689181de852516bf45acbe72297c332497e5d3b6",
    ),
    (
        "Netlists/Certification_Tests/BUG_1152/bug_767b.cir.tags",
        26,
        "1e6b928df7201ec2ef59e9058e3fd4fb216589d8245520003d5603a8f13b4c5b",
        "734e1696605bc661d095ac763edd46b3c764489f5166f85f03781e02239c8cba",
    ),
    (
        CONTROL_PATH,
        1_390,
        "8507fc6a025e82f8437bb15a44bff4b60ea5c9b6493426e2ec82494e7991fdb0",
        "5e7203645f39500afae6c5ba2a4a7dd328f9ea8cd6917b712c1af4fccd95d5b4",
    ),
    (
        "Netlists/Certification_Tests/BUG_1152/exclude",
        42,
        "4ed29eb665e383784b006fd62799dd2344d2bfeb15490c532fcfc9678957b0dc",
        "14afc632aef553d25d2c002bf2b111a8059ac2b98ea4e78e226a0c058764101a",
    ),
    (
        "TestScripts/xyce_verify.pl",
        59_566,
        "6e5f84b1646b30d0e12879848d7653584b39472d640a14916ae8fda6e1df12b3",
        "5eadb6dab06ed3091ea114146bd4a574de83784f87be9843ad7b721b0a793665",
    ),
];

const RETAINED_ARTIFACTS: [(&str, usize, &str, &str); 4] = [
    (
        "README",
        1_005,
        "c6b9aecac4de0edb08654532238661d2523815e6a55012025aac715afbf5a2cb",
        "4e9c0c3f799a4bf90daea5c5950d92a0c0a7f9766033010e362862497ca43c1a",
    ),
    (
        "bug_1152.cir",
        1_205,
        "1f05eeb9d8c7f7c09f0c228b108e43a6fd9bf34ae659a62dca785bc64d63f999",
        "df2f75e06b363416d4ea41c7057810ee42b099fb2f4cd59c9e84f6bb47ad4897",
    ),
    (
        "bug_767b.cir",
        1_390,
        "39e8375c87e5e4910f14861b4714e1f6695c46da154c8e0fa5e157a688ee5a54",
        "ccdfd8e535d2bccf69ea36d8b43e878a45ddaf5124b4e730ed997740a692ff18",
    ),
    (
        "bug_767b_nodupes.cir",
        1_390,
        "8507fc6a025e82f8437bb15a44bff4b60ea5c9b6493426e2ec82494e7991fdb0",
        "5e7203645f39500afae6c5ba2a4a7dd328f9ea8cd6917b712c1af4fccd95d5b4",
    ),
];

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub(super) enum Bug1152Role {
    RlcDeviceNodeNameOwner,
    DuplicateDeviceNodeOwner,
    NoDuplicateControl,
}

impl Bug1152Role {
    const ALL: [Self; 3] = [
        Self::RlcDeviceNodeNameOwner,
        Self::DuplicateDeviceNodeOwner,
        Self::NoDuplicateControl,
    ];

    pub(super) fn for_record(relative_path: &str) -> Option<Self> {
        match XyceTestRunner::normalize_manifest_key(relative_path).as_str() {
            RLC_RECORD => Some(Self::RlcDeviceNodeNameOwner),
            COLLISION_RECORD => Some(Self::DuplicateDeviceNodeOwner),
            CONTROL_RECORD => Some(Self::NoDuplicateControl),
            _ => None,
        }
    }

    pub(super) fn contract(self) -> &'static str {
        match self {
            Self::RlcDeviceNodeNameOwner => RLC_CONTRACT,
            Self::DuplicateDeviceNodeOwner => COLLISION_CONTRACT,
            Self::NoDuplicateControl => CONTROL_CONTRACT,
        }
    }

    fn path(self) -> &'static str {
        match self {
            Self::RlcDeviceNodeNameOwner => RLC_PATH,
            Self::DuplicateDeviceNodeOwner => COLLISION_PATH,
            Self::NoDuplicateControl => CONTROL_PATH,
        }
    }

    fn record(self) -> &'static str {
        match self {
            Self::RlcDeviceNodeNameOwner => RLC_RECORD,
            Self::DuplicateDeviceNodeOwner => COLLISION_RECORD,
            Self::NoDuplicateControl => CONTROL_RECORD,
        }
    }

    fn file_name(self) -> &'static str {
        Path::new(self.path())
            .file_name()
            .and_then(|name| name.to_str())
            .expect("BUG1152 paths use UTF-8 file names")
    }

    fn label(self) -> &'static str {
        match self {
            Self::RlcDeviceNodeNameOwner => "RLC analytic wrapper owner",
            Self::DuplicateDeviceNodeOwner => "duplicate-name relational wrapper owner",
            Self::NoDuplicateControl => "renamed-node relational control",
        }
    }
}

impl XyceTestRunner {
    pub(super) fn bug1152_historical_oracle_provenance_records() -> Vec<String> {
        let mut records = HISTORICAL_ARTIFACTS
            .into_iter()
            .map(|(path, bytes, sha256, content_blake3)| {
                format!(
                    "{UPSTREAM_REGRESSION_COMMIT}\t{UPSTREAM_RELEASE_TAG}\t{path}\t{bytes}\t{sha256}\t{content_blake3}"
                )
            })
            .collect::<Vec<_>>();
        records.sort();
        records
    }

    pub(super) fn validate_bug1152_historical_oracle_provenance() -> Result<(), String> {
        let records = Self::bug1152_historical_oracle_provenance_records();
        let stream = records.join("\n");
        let sha256 = format!("{:x}", Sha256::digest(stream.as_bytes()));
        let content_blake3 = blake3::hash(stream.as_bytes()).to_hex().to_string();
        if records.len() != HISTORICAL_RECORD_COUNT
            || stream.len() != HISTORICAL_RECORD_BYTES
            || sha256 != HISTORICAL_RECORDS_SHA256
            || content_blake3 != HISTORICAL_RECORDS_BLAKE3
        {
            return Err(format!(
                "{LABEL} Release-7.10 oracle provenance changed: records={}/{}, sha256={sha256}, blake3={content_blake3}",
                records.len(),
                stream.len()
            ));
        }
        Ok(())
    }

    fn validate_bug1152_source_directory(
        directory: &Path,
    ) -> Result<BTreeMap<String, Vec<u8>>, String> {
        let metadata = fs::symlink_metadata(directory)
            .map_err(|error| format!("failed to inspect {LABEL} directory: {error}"))?;
        if metadata.file_type().is_symlink() || !metadata.file_type().is_dir() {
            return Err(format!(
                "{LABEL} source directory must be a regular non-symlink directory"
            ));
        }
        let expected = RETAINED_ARTIFACTS
            .into_iter()
            .map(|artifact| (artifact.0.to_ascii_lowercase(), artifact))
            .collect::<BTreeMap<_, _>>();
        let mut observed = BTreeMap::new();
        for entry in fs::read_dir(directory)
            .map_err(|error| format!("failed to read {LABEL} directory: {error}"))?
        {
            let entry =
                entry.map_err(|error| format!("failed to inspect {LABEL} member: {error}"))?;
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
            if observed.contains_key(&key) {
                return Err(format!("{LABEL} contains a case-colliding member {name:?}"));
            }
            let Some((expected_name, expected_bytes, expected_sha256, expected_blake3)) =
                expected.get(&key).copied()
            else {
                return Err(format!(
                    "{LABEL} acquired unexpected source member {name:?}"
                ));
            };
            if name != expected_name {
                return Err(format!(
                    "{LABEL} member case changed: expected {expected_name:?}, got {name:?}"
                ));
            }
            let bytes = fs::read(&path)
                .map_err(|error| format!("failed to read {}: {error}", path.display()))?;
            let canonical = Self::canonical_lf_text_identity(LABEL, &bytes)?;
            let sha256 = format!("{:x}", Sha256::digest(&canonical));
            let content_blake3 = blake3::hash(&canonical).to_hex().to_string();
            if canonical.len() != expected_bytes
                || sha256 != expected_sha256
                || content_blake3 != expected_blake3
            {
                return Err(format!("{LABEL} member {name:?} content changed"));
            }
            observed.insert(key, bytes);
        }
        if observed.len() != expected.len() {
            return Err(format!(
                "{LABEL} source census changed: expected {}, got {}",
                expected.len(),
                observed.len()
            ));
        }
        Ok(observed)
    }

    fn validate_bug1152_provenance(
        &self,
        deck: &XyceDeck,
        role: Bug1152Role,
    ) -> Result<BTreeMap<Bug1152Role, Vec<u8>>, String> {
        Self::validate_bug1152_historical_oracle_provenance()?;
        if deck.section != XyceDeckSection::Netlists
            || Self::normalize_manifest_key(&deck.relative_path) != role.record()
            || Self::normalize_manifest_key(&self.relative_key(&deck.path)) != role.record()
            || !Self::same_path(&deck.path, &self.root.join(role.path()))
        {
            return Err(format!(
                "recognized {LABEL} {} is not backed by its canonical path",
                role.label()
            ));
        }

        let owners = self
            .upstream_wrapper_decks
            .iter()
            .filter(|record| record.starts_with(FAMILY_PREFIX))
            .map(String::as_str)
            .collect::<BTreeSet<_>>();
        if owners != BTreeSet::from([RLC_RECORD, COLLISION_RECORD]) {
            return Err(format!("{LABEL} wrapper ownership changed: {owners:?}"));
        }

        let exclusions = Self::load_upstream_exclusions(&self.root)
            .map_err(|error| format!("{LABEL} exclusion manifest is invalid: {error}"))?;
        let family_exclusions = exclusions
            .iter()
            .filter(|(record, _)| record.starts_with(FAMILY_PREFIX))
            .collect::<BTreeMap<_, _>>();
        if family_exclusions.len() != 1 || !exclusions.contains_key(CONTROL_RECORD) {
            return Err(format!(
                "{LABEL} requires exactly its one independently-qualified control: {:?}",
                family_exclusions.keys().collect::<Vec<_>>()
            ));
        }
        let control = exclusions
            .get(CONTROL_RECORD)
            .expect("family exclusion key was checked");
        if control.source != EXCLUSION_SOURCE
            || !matches!(
                &control.disposition,
                XyceUpstreamExclusionDisposition::RspiceIndependentlyQualified {
                    expected_contract
                } if expected_contract == CONTROL_CONTRACT
            )
        {
            return Err(format!(
                "{LABEL} control exclusion provenance changed: {control:?}"
            ));
        }

        let retained = Self::validate_bug1152_source_directory(&self.root.join(FAMILY_DIRECTORY))?;
        match fs::symlink_metadata(self.root.join(OUTPUT_DIRECTORY)) {
            Err(error) if error.kind() == std::io::ErrorKind::NotFound => {}
            Err(error) => return Err(format!("failed to inspect {LABEL} OutputData: {error}")),
            Ok(_) => {
                return Err(format!(
                    "{LABEL} must not acquire checked-in numerical gold"
                ));
            }
        }
        for member in Bug1152Role::ALL {
            self.reject_wrapper_output_artifacts(&self.root.join(member.path()))
                .map_err(|error| format!("{LABEL} {} {error}", member.label()))?;
        }

        Bug1152Role::ALL
            .into_iter()
            .map(|member| {
                retained
                    .get(&member.file_name().to_ascii_lowercase())
                    .cloned()
                    .map(|bytes| (member, bytes))
                    .ok_or_else(|| format!("{LABEL} lost {}", member.file_name()))
            })
            .collect()
    }

    fn bug1152_nodes_match(actual: &[String], expected: &[&str]) -> bool {
        actual.len() == expected.len()
            && actual
                .iter()
                .zip(expected)
                .all(|(actual, expected)| actual.eq_ignore_ascii_case(expected))
    }

    fn validate_bug1152_rlc_plan(
        &self,
        source: &str,
        path: &Path,
    ) -> Result<(XyceStaticTranPlan, Netlist), String> {
        let plan = self.static_tran_plan_for_path_with_purpose(
            path,
            XyceStaticTranPlanPurpose::AnalyticOracle,
        )?;
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
        if plan.deck_path != path
            || plan.source != source
            || !matches!(plan.oracle, XyceStaticTranOracle::None)
            || plan.contract != XyceStaticTranContract::WrapperStatic
            || plan.comparison_mode != XyceStaticTranComparisonMode::Pointwise
            || probes != ["v(vin)", "{i(vin)-1.0}"]
            || !plan.steps.is_empty()
            || plan.output_override
            || plan.timeint_conststep
            || plan.wrapper_tolerance.is_some()
            || plan.tran.step.to_bits() != 0.01f64.to_bits()
            || plan.tran.stop.to_bits() != 10.0f64.to_bits()
            || plan.tran.start.map(Value::to_bits) != Some(0.0f64.to_bits())
            || plan.tran.max_step.is_some()
            || plan.tran.uic
        {
            return Err(format!("{LABEL} RLC plan changed: {plan:?}"));
        }

        let netlist = Self::parse_xyce_netlist(source, path)
            .map_err(|error| format!("{LABEL} RLC owner no longer parses: {error}"))?;
        if netlist.elements.len() != 4
            || !netlist.models.is_empty()
            || !netlist.subcircuits.is_empty()
            || netlist.analyses.len() != 1
            || netlist.output_requests.len() != 1
            || !netlist.diagnostics.is_empty()
            || netlist.options.timeint_reltol.map(Value::to_bits) != Some(1.0e-3f64.to_bits())
        {
            return Err(format!(
                "{LABEL} RLC typed envelope changed: elements={:?}, analyses={:?}, outputs={:?}, diagnostics={:?}",
                netlist.elements, netlist.analyses, netlist.output_requests, netlist.diagnostics
            ));
        }
        let find = |name: &str| {
            netlist
                .elements
                .iter()
                .find(|element| element.name.eq_ignore_ascii_case(name))
        };
        let resistor = find("R1").ok_or_else(|| format!("{LABEL} lost R1"))?;
        let inductor = find("L1").ok_or_else(|| format!("{LABEL} lost L1"))?;
        let capacitor = find("C1").ok_or_else(|| format!("{LABEL} lost C1"))?;
        let source_element = find("VIN").ok_or_else(|| format!("{LABEL} lost VIN source"))?;
        if !Self::bug1152_nodes_match(&resistor.nodes, &["vin", "2"])
            || !matches!(&resistor.kind, ElementKind::Resistor { value, value_expr: None, model: None, instance_params, deferred_params }
                if value.to_bits() == 3.0f64.to_bits()
                    && instance_params.is_empty() && deferred_params.is_empty())
            || !Self::bug1152_nodes_match(&inductor.nodes, &["2", "3"])
            || !matches!(&inductor.kind, ElementKind::Inductor { value, value_expr: None, initial_current: None, model: None, instance_params, deferred_params }
                if value.to_bits() == 1.0f64.to_bits()
                    && instance_params.is_empty() && deferred_params.is_empty())
            || !Self::bug1152_nodes_match(&capacitor.nodes, &["3", "0"])
            || !matches!(&capacitor.kind, ElementKind::Capacitor { value, value_expr: None, initial_voltage: None, model: None, instance_params, deferred_params }
                if value.to_bits() == 0.5f64.to_bits()
                    && instance_params.is_empty() && deferred_params.is_empty())
            || !source_element
                .name
                .eq_ignore_ascii_case(&source_element.nodes[0])
            || !Self::bug1152_nodes_match(&source_element.nodes, &["vin", "0"])
        {
            return Err(format!(
                "{LABEL} RLC topology changed: {:?}",
                netlist.elements
            ));
        }
        let ElementKind::VoltageSource(rspice_core::netlist::SourceSpec::DcTransient {
            dc_value,
            transient,
        }) = &source_element.kind
        else {
            return Err(format!("{LABEL} VIN lost its DC-plus-PULSE source form"));
        };
        if dc_value.to_bits() != 10.0f64.to_bits()
            || !matches!(transient.as_ref(), rspice_core::netlist::SourceSpec::Pulse {
                v1, v2, delay, rise, fall, width, period, pulse_count, width_defaults_to_zero,
            } if v1.to_bits() == 0.0f64.to_bits()
                && v2.to_bits() == 10.0f64.to_bits()
                && delay.to_bits() == 0.0f64.to_bits()
                && rise.to_bits() == 0.0f64.to_bits()
                && fall.to_bits() == 0.0f64.to_bits()
                && width.to_bits() == 10.0f64.to_bits()
                && period.to_bits() == 10.0f64.to_bits()
                && pulse_count.to_bits() == 0.0f64.to_bits()
                && !width_defaults_to_zero)
        {
            return Err(format!("{LABEL} VIN source parameters changed"));
        }
        validate_output_symbols(&netlist)
            .map_err(|error| format!("{LABEL} RLC node/device probes do not resolve: {error}"))?;
        Ok((plan, netlist))
    }

    fn bug1152_rlc_reference(actual: &XycePrnTable) -> Result<XycePrnTable, String> {
        if actual.columns.len() != 4 {
            return Err(format!(
                "{LABEL} RLC table requires Index/TIME/V(VIN)/expression, got {:?}",
                actual.columns
            ));
        }
        let mut rows = Vec::with_capacity(actual.rows.len());
        for (row_index, row) in actual.rows.iter().enumerate() {
            if row.len() != 4 || row.iter().any(|value| !value.is_finite()) {
                return Err(format!("{LABEL} RLC row {row_index} is malformed: {row:?}"));
            }
            // The removed Perl generator read the already serialized time and
            // copied the serialized node voltage from the candidate PRN. Only
            // its current-expression column was independently analytic.
            let time = Self::xyce_default_prn_roundtrip(row[1])?;
            let voltage = Self::xyce_default_prn_roundtrip(row[2])?;
            let current_expression = Self::xyce_default_prn_roundtrip(
                10.0 * ((-2.0 * time).exp() - (-time).exp()) - 1.0,
            )?;
            rows.push(vec![row[0], time, voltage, current_expression]);
        }
        Ok(XycePrnTable {
            columns: actual.columns.clone(),
            rows,
        })
    }

    fn run_bug1152_rlc_oracle(&self, source: &str, start: Instant) -> Result<(), String> {
        let path = self.root.join(RLC_PATH);
        let (plan, _) = self.validate_bug1152_rlc_plan(source, &path)?;
        let (netlist, result) = self
            .run_transient_family_plan(&plan, start, None, None)
            .map_err(|error| format!("{LABEL} RLC simulation failed: {error}"))?;
        let actual = Self::transient_family_result_to_prn_table(&plan, &netlist, &result)
            .map_err(|error| format!("{LABEL} RLC PRN projection failed: {error}"))?;
        if actual.rows.len() < 100
            || actual.rows.first().is_none_or(|row| {
                row.len() != 4 || row[1].abs() > 1.0e-15 || row[2].abs() > 1.0e-12
            })
            || actual
                .rows
                .last()
                .is_none_or(|row| row.len() != 4 || (row[1] - 10.0).abs() > 1.0e-9)
            || actual
                .rows
                .iter()
                .skip(1)
                .any(|row| (row[2] - 10.0).abs() > 1.0e-8)
            || actual
                .rows
                .iter()
                .map(|row| row[3])
                .fold(Value::INFINITY, Value::min)
                > -3.0
        {
            return Err(format!(
                "{LABEL} RLC output is incomplete or trivial: columns={:?}, rows={}",
                actual.columns,
                actual.rows.len()
            ));
        }
        let reference = Self::bug1152_rlc_reference(&actual)?;
        let mismatches = self.compare_xyce_verify_transient_tables(&reference, &actual)?;
        if !mismatches.is_empty() {
            return Err(format!(
                "{LABEL} RLC current differs from the Release analytic generator: {mismatches:?}"
            ));
        }
        Ok(())
    }

    fn validate_bug1152_relational_plan(
        &self,
        role: Bug1152Role,
        source: &str,
        path: &Path,
    ) -> Result<(XyceStaticTranPlan, Netlist), String> {
        let purpose = match role {
            Bug1152Role::DuplicateDeviceNodeOwner => {
                XyceStaticTranPlanPurpose::GeneratedReferenceRelationalFamily
            }
            Bug1152Role::NoDuplicateControl => XyceStaticTranPlanPurpose::RelationalFamily,
            Bug1152Role::RlcDeviceNodeNameOwner => {
                return Err(format!("{LABEL} RLC role is not a relational worker"));
            }
        };
        let plan = self.static_tran_plan_for_path_with_purpose(path, purpose)?;
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
        let expected_contract = if role == Bug1152Role::DuplicateDeviceNodeOwner {
            XyceStaticTranContract::WrapperStatic
        } else {
            XyceStaticTranContract::PlainStatic
        };
        if plan.deck_path != path
            || plan.source != source
            || !matches!(plan.oracle, XyceStaticTranOracle::None)
            || plan.contract != expected_contract
            || plan.comparison_mode != XyceStaticTranComparisonMode::Pointwise
            || probes != ["{v(20)+1}", "v(3)"]
            || !plan.steps.is_empty()
            || plan.output_override
            || plan.timeint_conststep
            || plan.wrapper_tolerance.is_some()
            || plan.tran.step.to_bits() != 0.2e-6f64.to_bits()
            || plan.tran.stop.to_bits() != 3.0e-3f64.to_bits()
            || plan.tran.start.map(Value::to_bits) != Some(0.5e-3f64.to_bits())
            || plan.tran.max_step.is_some()
            || plan.tran.uic
        {
            return Err(format!("{LABEL} {} plan changed: {plan:?}", role.label()));
        }

        let netlist = Self::parse_xyce_netlist(source, path)
            .map_err(|error| format!("{LABEL} {} no longer parses: {error}", role.label()))?;
        let [subcircuit] = netlist.subcircuits.as_slice() else {
            return Err(format!("{LABEL} {} requires one OPAMP#0", role.label()));
        };
        if netlist.elements.len() != 13
            || netlist.models.len() != 1
            || netlist.analyses.len() != 1
            || netlist.output_requests.len() != 1
            || !netlist.diagnostics.is_empty()
            || !subcircuit.name.eq_ignore_ascii_case("OPAMP#0")
            || !Self::bug1152_nodes_match(&subcircuit.ports, &["2", "3", "6", "7", "4"])
            || subcircuit.elements.len() != 28
        {
            return Err(format!(
                "{LABEL} {} typed envelope changed: top={}, models={}, subcircuits={:?}, diagnostics={:?}",
                role.label(),
                netlist.elements.len(),
                netlist.models.len(),
                netlist.subcircuits,
                netlist.diagnostics
            ));
        }
        let resistor = subcircuit
            .elements
            .iter()
            .find(|element| element.name.eq_ignore_ascii_case("R1"))
            .ok_or_else(|| format!("{LABEL} {} lost internal resistor R1", role.label()))?;
        if !Self::bug1152_nodes_match(&resistor.nodes, &["11", "12"])
            || !matches!(&resistor.kind, ElementKind::Resistor { value, .. }
                if value.to_bits() == 5_000.0f64.to_bits())
        {
            return Err(format!(
                "{LABEL} {} internal device R1 changed",
                role.label()
            ));
        }
        let collision_node = if role == Bug1152Role::DuplicateDeviceNodeOwner {
            "R1"
        } else {
            "14"
        };
        let colliding_members = ["B", "C2", "RO", "L", "RL"];
        for member in colliding_members {
            let element = subcircuit
                .elements
                .iter()
                .find(|element| element.name.eq_ignore_ascii_case(member))
                .ok_or_else(|| format!("{LABEL} {} lost internal {member}", role.label()))?;
            if !element
                .nodes
                .iter()
                .any(|node| node.eq_ignore_ascii_case(collision_node))
            {
                return Err(format!(
                    "{LABEL} {} internal {member} lost node {collision_node}",
                    role.label()
                ));
            }
        }
        let r1_node_uses = subcircuit
            .elements
            .iter()
            .flat_map(|element| &element.nodes)
            .filter(|node| node.eq_ignore_ascii_case("R1"))
            .count();
        let node14_uses = subcircuit
            .elements
            .iter()
            .flat_map(|element| &element.nodes)
            .filter(|node| node.eq_ignore_ascii_case("14"))
            .count();
        if (role == Bug1152Role::DuplicateDeviceNodeOwner
            && (r1_node_uses != 5 || node14_uses != 0))
            || (role == Bug1152Role::NoDuplicateControl && (r1_node_uses != 0 || node14_uses != 5))
        {
            return Err(format!(
                "{LABEL} {} namespace delta changed: R1-node uses={r1_node_uses}, 14-node uses={node14_uses}",
                role.label()
            ));
        }
        validate_output_symbols(&netlist).map_err(|error| {
            format!(
                "{LABEL} {} output symbols do not resolve: {error}",
                role.label()
            )
        })?;
        Ok((plan, netlist))
    }

    fn run_bug1152_relational_oracle(
        &self,
        sources: &BTreeMap<Bug1152Role, Vec<u8>>,
        start: Instant,
    ) -> Result<(), String> {
        let mut outputs = BTreeMap::new();
        let mut owner_source = None;
        for role in [
            Bug1152Role::DuplicateDeviceNodeOwner,
            Bug1152Role::NoDuplicateControl,
        ] {
            let source = std::str::from_utf8(
                sources
                    .get(&role)
                    .ok_or_else(|| format!("{LABEL} lost {} source", role.label()))?,
            )
            .map_err(|error| format!("{LABEL} {} is not UTF-8: {error}", role.label()))?;
            if role == Bug1152Role::DuplicateDeviceNodeOwner {
                owner_source = Some(source.to_string());
            }
            let path = self.root.join(role.path());
            let (plan, _) = self.validate_bug1152_relational_plan(role, source, &path)?;
            let (netlist, result) = self
                .run_transient_family_plan(&plan, start, None, None)
                .map_err(|error| format!("{LABEL} {} simulation failed: {error}", role.label()))?;
            let table = Self::transient_family_result_to_prn_table(&plan, &netlist, &result)
                .map_err(|error| format!("{LABEL} {} PRN failed: {error}", role.label()))?;
            let expression_min = table
                .rows
                .iter()
                .map(|row| row.get(2).copied().unwrap_or(Value::NAN))
                .fold(Value::INFINITY, Value::min);
            let expression_max = table
                .rows
                .iter()
                .map(|row| row.get(2).copied().unwrap_or(Value::NAN))
                .fold(Value::NEG_INFINITY, Value::max);
            let input_min = table
                .rows
                .iter()
                .map(|row| row.get(3).copied().unwrap_or(Value::NAN))
                .fold(Value::INFINITY, Value::min);
            let input_max = table
                .rows
                .iter()
                .map(|row| row.get(3).copied().unwrap_or(Value::NAN))
                .fold(Value::NEG_INFINITY, Value::max);
            if table.columns.len() != 4
                || table.rows.len() < 20
                || table
                    .rows
                    .iter()
                    .any(|row| row.len() != 4 || row.iter().any(|value| !value.is_finite()))
                || table.rows.first().is_none_or(|row| {
                    row[1] < 0.5e-3 - 1.0e-12 || row[1] >= 0.75e-3 || row[3].abs() > 1.0e-8
                })
                || table
                    .rows
                    .last()
                    .is_none_or(|row| (row[1] - 3.0e-3).abs() > 1.0e-12)
                || expression_max - expression_min < 0.1
                || input_max - input_min < 4.9
            {
                return Err(format!(
                    "{LABEL} {} produced an invalid or trivial table: columns={:?}, rows={}, expression-range={:?}, input-range={:?}",
                    role.label(),
                    table.columns,
                    table.rows.len(),
                    expression_max - expression_min,
                    input_max - input_min,
                ));
            }
            outputs.insert(role, table);
        }
        let good = outputs
            .get(&Bug1152Role::DuplicateDeviceNodeOwner)
            .expect("both BUG1152 relational workers ran");
        let test = outputs
            .get(&Bug1152Role::NoDuplicateControl)
            .expect("both BUG1152 relational workers ran");
        let tolerances = Self::xyce_verify_comp_tolerances(
            owner_source.as_deref().expect("owner source was recorded"),
            &good.columns[2..],
        )?;
        if tolerances.len() != 2
            || tolerances[0].relative.to_bits() != 0.02f64.to_bits()
            || tolerances[1].relative.to_bits() != 0.01f64.to_bits()
        {
            return Err(format!(
                "{LABEL} *COMP tolerance contract changed: {tolerances:?}"
            ));
        }
        let mismatches = self.compare_xyce_verify_transient_tables_with_probe_tolerances(
            good,
            test,
            &tolerances,
            XYCE_DEFAULT_PRN_SCIENTIFIC_PRECISION,
        )?;
        if !mismatches.is_empty() {
            return Err(format!(
                "{LABEL} duplicate-name owner differs from renamed control: {mismatches:?}"
            ));
        }
        Ok(())
    }

    pub(super) fn validate_bug1152_oracle(
        &self,
        deck: &XyceDeck,
        role: Bug1152Role,
        start: Instant,
    ) -> Result<(), String> {
        let abort = DeadlineAbort::new(start, self.config.max_time_per_test_ms.max(1));
        if abort.is_aborted() {
            return Err(format!("{LABEL} deadline expired before validation"));
        }
        let sources = self.validate_bug1152_provenance(deck, role)?;
        match role {
            Bug1152Role::RlcDeviceNodeNameOwner => {
                let source = std::str::from_utf8(
                    sources
                        .get(&role)
                        .ok_or_else(|| format!("{LABEL} lost RLC owner source"))?,
                )
                .map_err(|error| format!("{LABEL} RLC owner is not UTF-8: {error}"))?;
                self.run_bug1152_rlc_oracle(source, start)?;
            }
            Bug1152Role::DuplicateDeviceNodeOwner | Bug1152Role::NoDuplicateControl => {
                self.run_bug1152_relational_oracle(&sources, start)?;
            }
        }
        if abort.is_aborted() {
            return Err(format!(
                "{LABEL} execution exceeded timeout ({}ms)",
                self.config.max_time_per_test_ms
            ));
        }
        self.validate_bug1152_provenance(deck, role)?;
        if abort.is_aborted() {
            return Err(format!("{LABEL} final provenance exceeded timeout"));
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

    fn deck(root: &Path, role: Bug1152Role) -> XyceDeck {
        XyceDeck {
            path: root.join(role.path()),
            section: XyceDeckSection::Netlists,
            relative_path: role.path().to_string(),
        }
    }

    fn fixture(label: &str) -> (tempfile::TempDir, XyceTestRunner) {
        let temporary = tempfile::Builder::new()
            .prefix(&format!("rspice-xyce-bug1152-{label}-"))
            .tempdir()
            .expect("create BUG1152 fixture root");
        let root = temporary.path();
        let family = root.join(FAMILY_DIRECTORY);
        fs::create_dir_all(&family).expect("create BUG1152 family");
        let canonical = corpus_root().join(FAMILY_DIRECTORY);
        for (name, ..) in RETAINED_ARTIFACTS {
            fs::copy(canonical.join(name), family.join(name))
                .expect("copy canonical BUG1152 member");
        }
        fs::write(
            root.join(HARNESS_MANIFEST_FILE),
            format!(
                "{RLC_PATH}\t{REQUIRES_UPSTREAM_WRAPPER_CONTRACT}\n{COLLISION_PATH}\t{REQUIRES_UPSTREAM_WRAPPER_CONTRACT}\n"
            ),
        )
        .expect("write BUG1152 wrapper manifest");
        fs::write(
            root.join(UPSTREAM_EXCLUSIONS_MANIFEST_FILE),
            format!(
                "schema_version\t{UPSTREAM_EXCLUSIONS_SCHEMA_VERSION}\nsource_commit\t{UPSTREAM_EXCLUSIONS_SOURCE_COMMIT}\nsource_netlists_tree\t{UPSTREAM_EXCLUSIONS_SOURCE_NETLISTS_TREE}\n{CONTROL_PATH}\t{EXCLUSION_SOURCE}\t{RSPICE_INDEPENDENTLY_QUALIFIED_DISPOSITION}\t{CONTROL_CONTRACT}\n"
            ),
        )
        .expect("write BUG1152 exclusion manifest");
        let runner = XyceTestRunner::new(root, XyceRunnerConfig::default());
        (temporary, runner)
    }

    #[test]
    fn bug1152_historical_provenance_is_exact() {
        XyceTestRunner::validate_bug1152_historical_oracle_provenance()
            .expect("Release-7.10 BUG1152 provenance remains exact");
    }

    #[test]
    fn bug1152_typed_plans_preserve_both_namespace_contracts() {
        let root = corpus_root();
        let runner = XyceTestRunner::new(&root, XyceRunnerConfig::default());
        let rlc_source = fs::read_to_string(root.join(RLC_PATH)).expect("read BUG1152 RLC");
        runner
            .validate_bug1152_rlc_plan(&rlc_source, &root.join(RLC_PATH))
            .expect("RLC node/device-name contract remains exact");
        for role in [
            Bug1152Role::DuplicateDeviceNodeOwner,
            Bug1152Role::NoDuplicateControl,
        ] {
            let source = fs::read_to_string(root.join(role.path())).expect("read BUG1152 worker");
            runner
                .validate_bug1152_relational_plan(role, &source, &root.join(role.path()))
                .unwrap_or_else(|error| panic!("canonical {role:?} failed: {error}"));
        }
    }

    #[test]
    fn bug1152_analytic_rlc_oracle_executes() {
        let root = corpus_root();
        let runner = XyceTestRunner::new(&root, XyceRunnerConfig::default());
        runner
            .validate_bug1152_oracle(
                &deck(&root, Bug1152Role::RlcDeviceNodeNameOwner),
                Bug1152Role::RlcDeviceNodeNameOwner,
                Instant::now(),
            )
            .expect("canonical BUG1152 analytic owner passes");
    }

    #[test]
    fn bug1152_relational_oracle_executes_both_topologies() {
        let root = corpus_root();
        let runner = XyceTestRunner::new(&root, XyceRunnerConfig::default());
        runner
            .validate_bug1152_oracle(
                &deck(&root, Bug1152Role::DuplicateDeviceNodeOwner),
                Bug1152Role::DuplicateDeviceNodeOwner,
                Instant::now(),
            )
            .expect("canonical BUG1152 collision relation passes");
    }

    #[test]
    fn bug1152_provenance_rejects_source_manifest_exclusion_and_output_drift() {
        let (temporary, runner) = fixture("source");
        let requested = deck(temporary.path(), Bug1152Role::RlcDeviceNodeNameOwner);
        runner
            .validate_bug1152_provenance(&requested, Bug1152Role::RlcDeviceNodeNameOwner)
            .expect("canonical BUG1152 fixture passes");
        fs::write(temporary.path().join(RLC_PATH), "* mutated\n").expect("mutate BUG1152 source");
        assert!(
            runner
                .validate_bug1152_provenance(&requested, Bug1152Role::RlcDeviceNodeNameOwner)
                .is_err()
        );

        let (temporary, _) = fixture("manifest");
        fs::write(
            temporary.path().join(HARNESS_MANIFEST_FILE),
            format!("{RLC_PATH}\t{REQUIRES_UPSTREAM_WRAPPER_CONTRACT}\n"),
        )
        .expect("remove BUG1152 wrapper owner");
        let runner = XyceTestRunner::new(temporary.path(), XyceRunnerConfig::default());
        assert!(
            runner
                .validate_bug1152_provenance(
                    &deck(temporary.path(), Bug1152Role::RlcDeviceNodeNameOwner),
                    Bug1152Role::RlcDeviceNodeNameOwner,
                )
                .is_err()
        );

        let (temporary, _) = fixture("exclusion");
        fs::write(
            temporary.path().join(UPSTREAM_EXCLUSIONS_MANIFEST_FILE),
            format!(
                "schema_version\t{UPSTREAM_EXCLUSIONS_SCHEMA_VERSION}\nsource_commit\t{UPSTREAM_EXCLUSIONS_SOURCE_COMMIT}\nsource_netlists_tree\t{UPSTREAM_EXCLUSIONS_SOURCE_NETLISTS_TREE}\n{CONTROL_PATH}\t{EXCLUSION_SOURCE}\t{UPSTREAM_EXCLUDED_DISPOSITION}\n"
            ),
        )
        .expect("demote BUG1152 control");
        let runner = XyceTestRunner::new(temporary.path(), XyceRunnerConfig::default());
        assert!(
            runner
                .validate_bug1152_provenance(
                    &deck(temporary.path(), Bug1152Role::RlcDeviceNodeNameOwner),
                    Bug1152Role::RlcDeviceNodeNameOwner,
                )
                .is_err()
        );

        let (temporary, runner) = fixture("output");
        fs::create_dir_all(temporary.path().join(OUTPUT_DIRECTORY))
            .expect("fabricate BUG1152 OutputData");
        assert!(
            runner
                .validate_bug1152_provenance(
                    &deck(temporary.path(), Bug1152Role::RlcDeviceNodeNameOwner),
                    Bug1152Role::RlcDeviceNodeNameOwner,
                )
                .is_err()
        );
    }
}
