use super::*;

const LABEL: &str = "BUG_1284 transient-restart transmission-line family";
const FAMILY_PREFIX: &str = "netlists/certification_tests/bug_1284/";
const FAMILY_DIRECTORY: &str = "Netlists/Certification_Tests/BUG_1284";
const EXCLUSION_SOURCE: &str = "Netlists/Certification_Tests/BUG_1284/exclude";

/// Native relational contract reported by the zero-byte wrapper owner.
pub(super) const BUG1284_OWNER_CONTRACT: &str =
    "bug1284_transient_restart_relational_wrapper_owner";
/// Native relational contract reported by each independently qualified worker.
pub(super) const BUG1284_WORKER_CONTRACT: &str = "bug1284_transient_restart_relational_worker";

const UPSTREAM_REGRESSION_COMMIT: &str = "d6e278e371ec2f3df1325dcff4552e585bc7ecc1";
const UPSTREAM_RELEASE_TAG: &str = "Release-7.10.0";
const VENDORING_COMMIT: &str = "317c587f7";
const HARNESS_TRIM_COMMIT: &str = "2e55e96a2";
const HISTORICAL_RECORD_COUNT: usize = 5;
const HISTORICAL_RECORD_BYTES: usize = 1_171;
const HISTORICAL_RECORDS_SHA256: &str =
    "30c10e7af5c1750da1e6d59191fd3220a335f09638f492ed000904e189a25ffc";
const HISTORICAL_RECORDS_BLAKE3: &str =
    "0031a59426f93d967ba1d014eb9211d8258e3d8e98134d08e441327d0f37ea3c";
const HISTORICAL_AUDIT_ORDER: [&str; HISTORICAL_RECORD_COUNT] = [
    "Netlists/Certification_Tests/BUG_1284/bug_1284.cir.sh",
    "Netlists/Certification_Tests/BUG_1284/exclude",
    "Netlists/Certification_Tests/BUG_1284/Manifest.txt",
    "Netlists/Certification_Tests/BUG_1284/tags",
    "TestScripts/xyce_verify.pl",
];
const HISTORICAL_ARTIFACTS: [(&str, usize, &str, &str); HISTORICAL_RECORD_COUNT] = [
    (
        "Netlists/Certification_Tests/BUG_1284/Manifest.txt",
        113,
        "e9c463f94221511030089456b45f8e40ab71b38a8773d9b7519addff4e983f24",
        "1a281726fed3ac52b02ef79e3d84c1ba9de92e0fb6123b4cae21a2f8a908fd3b",
    ),
    (
        "Netlists/Certification_Tests/BUG_1284/bug_1284.cir.sh",
        2_416,
        "c4e3910c69d8fdd82846b8c86becd8b147181ae3e8ae1c7040d8fa5e032365ec",
        "8268728a4a84d455be1c9d0c6ac0c88b0e263c77c99f46740f38531cec66804e",
    ),
    (
        "Netlists/Certification_Tests/BUG_1284/exclude",
        64,
        "1986dacd8be46db3f994abc768166367baf31de0358601a6d0380634c793db8e",
        "8ac0ffb41958db0c77461b0be981f59b86d80719dc5b573a2273f5c6b617ad91",
    ),
    (
        "Netlists/Certification_Tests/BUG_1284/tags",
        26,
        "bf067d656d5b7fac75e501e081ab8053923ff6f7ea1465e89d12dd3c66f3b601",
        "b9742cd676fd44242fa109cdac80e67108828ad45235d7b27ca6f8f5f5a7330b",
    ),
    (
        "TestScripts/xyce_verify.pl",
        59_566,
        "6e5f84b1646b30d0e12879848d7653584b39472d640a14916ae8fda6e1df12b3",
        "5eadb6dab06ed3091ea114146bd4a574de83784f87be9843ad7b721b0a793665",
    ),
];

const RETAINED_RECORD_COUNT: usize = 5;
const RETAINED_RECORD_BYTES: usize = 944;
const RETAINED_RECORDS_SHA256: &str =
    "cc0e425b6ae68fbd2e618a2423819bad7db50fa5bacbbac04061335e694df4b9";
const RETAINED_RECORDS_BLAKE3: &str =
    "34c1acaaf011305a840649b476cebbaf1abfcda200397af34f3fb247c81d4e37";
// Preserve the culture-aware filename order used by the original audit as
// explicit data so the aggregate identity is deterministic on every host.
const RETAINED_AUDIT_ORDER: [&str; RETAINED_RECORD_COUNT] = [
    "bug_1284_baseline.cir",
    "bug_1284_first.cir",
    "bug_1284_restarted.cir",
    "bug_1284.cir",
    "README",
];
const RETAINED_ARTIFACTS: [(&str, usize, &str, &str); RETAINED_RECORD_COUNT] = [
    (
        "README",
        958,
        "96195d6e557611c183f135ca7634faf8164c237984fb9f4574f6193a1b8dbef3",
        "2756a63942d038c50b498025c32af984cdb60fdb7d9eb114f620bfc8d3f10b2d",
    ),
    (
        "bug_1284.cir",
        0,
        "e3b0c44298fc1c149afbf4c8996fb92427ae41e4649b934ca495991b7852b855",
        "af1349b9f5f9a1a6a0404dea36dcc9499bcb25c9adc112b7cc9a93cae41f3262",
    ),
    (
        "bug_1284_baseline.cir",
        1_312,
        "9d48e45f9ec01db1c5497240166bea54e83ea4a08707b185b31492959b3b1ccb",
        "5626b2c82731172bd6e23172e0b5e621918709aa670fa5482b8a1ef30440bee8",
    ),
    (
        "bug_1284_first.cir",
        1_365,
        "99327d46b75bd0cb3bd83368e00a71df19a2a99097faba8793869a9a98681d98",
        "80de800cfe6594b8fbe68e22650689270cbf4d51e676684cca7861de757e4acf",
    ),
    (
        "bug_1284_restarted.cir",
        1_351,
        "37a3239a4d07c292ca050e878f8dc5d9d11fab6d54c7381e720a4b9b69b45ebc",
        "2dc8050d7fb4cf12e06ded39c19738b9602f3a6183347f7cc5277ea249f86d8f",
    ),
];

const RESTART_JOB: &str = "trans_test";
const RESTART_FILE: &str = "trans_test2e-08";
// The parser applies the authored `N` suffix as an IEEE-754 multiplication.
// Preserve that same operation here so bit-exact horizon checks do not replace
// the deck's parsed semantics with a separately rounded decimal literal.
const NANOSECOND: Value = 1.0e-9;
const PRINT_STEP: Value = 0.25 * NANOSECOND;
const FIRST_STOP: Value = 20.0 * NANOSECOND;
const FINAL_STOP: Value = 50.0 * NANOSECOND;
const RESTART_INTERVAL: Value = 5.0 * NANOSECOND;
const HISTORICAL_SAVE_TIMES: [Value; 5] = [
    0.0,
    5.0 * NANOSECOND,
    10.0 * NANOSECOND,
    15.0 * NANOSECOND,
    20.0 * NANOSECOND,
];

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub(super) enum Bug1284Role {
    WrapperOwner,
    Baseline,
    First,
    Restarted,
}

impl Bug1284Role {
    pub(super) const ALL: [Self; 4] = [
        Self::WrapperOwner,
        Self::Baseline,
        Self::First,
        Self::Restarted,
    ];
    pub(super) const WORKERS: [Self; 3] = [Self::Baseline, Self::First, Self::Restarted];

    pub(super) fn for_record(record: &str) -> Option<Self> {
        let record = XyceTestRunner::normalize_manifest_key(record);
        Self::ALL.into_iter().find(|role| role.record() == record)
    }

    pub(super) const fn contract(self) -> &'static str {
        match self {
            Self::WrapperOwner => BUG1284_OWNER_CONTRACT,
            Self::Baseline | Self::First | Self::Restarted => BUG1284_WORKER_CONTRACT,
        }
    }

    pub(super) const fn path(self) -> &'static str {
        match self {
            Self::WrapperOwner => "Netlists/Certification_Tests/BUG_1284/bug_1284.cir",
            Self::Baseline => "Netlists/Certification_Tests/BUG_1284/bug_1284_baseline.cir",
            Self::First => "Netlists/Certification_Tests/BUG_1284/bug_1284_first.cir",
            Self::Restarted => "Netlists/Certification_Tests/BUG_1284/bug_1284_restarted.cir",
        }
    }

    pub(super) fn record(self) -> &'static str {
        match self {
            Self::WrapperOwner => "netlists/certification_tests/bug_1284/bug_1284.cir",
            Self::Baseline => "netlists/certification_tests/bug_1284/bug_1284_baseline.cir",
            Self::First => "netlists/certification_tests/bug_1284/bug_1284_first.cir",
            Self::Restarted => "netlists/certification_tests/bug_1284/bug_1284_restarted.cir",
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub(super) struct Bug1284HistoricalProvenance {
    pub upstream_commit: &'static str,
    pub release_tag: &'static str,
    pub vendoring_commit: &'static str,
    pub harness_trim_commit: &'static str,
    pub artifacts: [(&'static str, usize, &'static str, &'static str); HISTORICAL_RECORD_COUNT],
    pub record_bytes: usize,
    pub records_sha256: &'static str,
    pub records_blake3: &'static str,
}

#[derive(Debug, Clone, PartialEq)]
#[cfg(test)]
pub(super) struct Bug1284RestartOracle {
    pub print_step: Value,
    pub first_stop: Value,
    pub final_stop: Value,
    pub restart_interval: Value,
    pub save_times: [Value; 5],
    pub restart_job: &'static str,
    pub restart_file: &'static str,
}

impl XyceTestRunner {
    /// Immutable audit identity for wrapper artifacts removed during corpus trimming.
    ///
    /// The aggregate digests were calculated over the five Release-7.10 artifact
    /// records containing their full per-file SHA-256 and BLAKE3 identities. The
    /// transformed vendored blobs are deliberately not substituted here.
    pub(super) fn bug1284_historical_provenance() -> Bug1284HistoricalProvenance {
        Bug1284HistoricalProvenance {
            upstream_commit: UPSTREAM_REGRESSION_COMMIT,
            release_tag: UPSTREAM_RELEASE_TAG,
            vendoring_commit: VENDORING_COMMIT,
            harness_trim_commit: HARNESS_TRIM_COMMIT,
            artifacts: HISTORICAL_ARTIFACTS,
            record_bytes: HISTORICAL_RECORD_BYTES,
            records_sha256: HISTORICAL_RECORDS_SHA256,
            records_blake3: HISTORICAL_RECORDS_BLAKE3,
        }
    }

    pub(super) fn validate_bug1284_historical_provenance() -> Result<(), String> {
        let provenance = Self::bug1284_historical_provenance();
        let mut records = provenance
            .artifacts
            .into_iter()
            .map(|(path, bytes, sha256, content_blake3)| {
                format!(
                    "{}\t{}\t{path}\t{bytes}\t{sha256}\t{content_blake3}",
                    provenance.upstream_commit, provenance.release_tag
                )
            })
            .collect::<Vec<_>>();
        records.sort_by_key(|record| {
            HISTORICAL_AUDIT_ORDER
                .iter()
                .position(|path| record.contains(&format!("\t{path}\t")))
                .unwrap_or(usize::MAX)
        });
        let stream = records.join("\n");
        let sha256 = format!("{:x}", Sha256::digest(stream.as_bytes()));
        let content_blake3 = blake3::hash(stream.as_bytes()).to_hex().to_string();
        if provenance.artifacts.len() != HISTORICAL_RECORD_COUNT
            || provenance
                .artifacts
                .iter()
                .any(|(path, bytes, artifact_sha256, artifact_blake3)| {
                    path.is_empty()
                        || *bytes == 0
                        || path.contains('\\')
                        || !path.is_ascii()
                        || Path::new(path).is_absolute()
                        || artifact_sha256.len() != 64
                        || !artifact_sha256
                            .bytes()
                            .all(|byte| byte.is_ascii_hexdigit() && !byte.is_ascii_uppercase())
                        || artifact_blake3.len() != 64
                        || !artifact_blake3
                            .bytes()
                            .all(|byte| byte.is_ascii_hexdigit() && !byte.is_ascii_uppercase())
                })
            || records.len() != HISTORICAL_RECORD_COUNT
            || stream.len() != provenance.record_bytes
            || sha256 != provenance.records_sha256
            || content_blake3 != provenance.records_blake3
            || UPSTREAM_EXCLUSIONS_SOURCE_COMMIT != "80115a9277c0ddb3409acceb3d4e745fd11cddd4"
        {
            return Err(format!(
                "{LABEL} historical Release-7.10 provenance changed: records={}/{HISTORICAL_RECORD_COUNT}, bytes={}/{}, sha256={sha256}, blake3={content_blake3}; vendor={}, trim={}",
                records.len(),
                stream.len(),
                provenance.record_bytes,
                provenance.vendoring_commit,
                provenance.harness_trim_commit,
            ));
        }
        Ok(())
    }

    #[cfg(test)]
    pub(super) fn bug1284_restart_oracle() -> Bug1284RestartOracle {
        Bug1284RestartOracle {
            print_step: PRINT_STEP,
            first_stop: FIRST_STOP,
            final_stop: FINAL_STOP,
            restart_interval: RESTART_INTERVAL,
            save_times: HISTORICAL_SAVE_TIMES,
            restart_job: RESTART_JOB,
            restart_file: RESTART_FILE,
        }
    }

    fn validate_bug1284_role_source(role: Bug1284Role, source: &str) -> Result<(), String> {
        if role == Bug1284Role::WrapperOwner {
            return source
                .is_empty()
                .then_some(())
                .ok_or_else(|| format!("{LABEL} wrapper owner must remain zero bytes"));
        }

        let significant = source
            .lines()
            .map(str::trim)
            .filter(|line| !line.is_empty() && !line.starts_with('*'))
            .map(|line| line.to_ascii_lowercase())
            .collect::<Vec<_>>();
        let expected_tran = match role {
            Bug1284Role::Baseline | Bug1284Role::Restarted => ".tran 0.25n 50n",
            Bug1284Role::First => ".tran 0.25n 20n",
            Bug1284Role::WrapperOwner => unreachable!(),
        };
        let expected_restart = match role {
            Bug1284Role::Baseline => None,
            Bug1284Role::First => Some(".options restart job=trans_test initial_interval=5n"),
            Bug1284Role::Restarted => Some(".options restart file=trans_test2e-08"),
            Bug1284Role::WrapperOwner => unreachable!(),
        };
        let mut expected = vec![
            "transmission line circuit",
            "vin 1 0 pulse(0 5 0 0.1n 0.1n 5n 25n)",
            "rin 1 2 50",
            "tline 2 0 3 0 z0=50 td=10n",
            "rl 3 0 50",
            expected_tran,
            ".print tran v(2) v(3)",
        ];
        if let Some(restart) = expected_restart {
            expected.push(restart);
        }
        expected.push(".end");
        if significant.iter().map(String::as_str).ne(expected) {
            return Err(format!(
                "{LABEL} {role:?} authored circuit/restart semantics changed: {significant:?}"
            ));
        }
        Ok(())
    }

    pub(super) fn validate_bug1284_provenance(
        &self,
        deck: &XyceDeck,
        role: Bug1284Role,
    ) -> Result<BTreeMap<Bug1284Role, Vec<u8>>, String> {
        Self::validate_bug1284_historical_provenance()?;
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

        let expected_artifacts = RETAINED_ARTIFACTS
            .into_iter()
            .map(|artifact| (artifact.0.to_ascii_lowercase(), artifact))
            .collect::<BTreeMap<_, _>>();
        let mut names = Vec::new();
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
                    "{LABEL} member '{}' must be a regular non-symlink file",
                    path.display()
                ));
            }
            let name = entry
                .file_name()
                .to_str()
                .ok_or_else(|| format!("{LABEL} member name is not UTF-8"))?
                .to_string();
            let key = name.to_ascii_lowercase();
            let Some((expected_name, expected_bytes, expected_sha256, expected_blake3)) =
                expected_artifacts.get(&key).copied()
            else {
                return Err(format!("{LABEL} contains unexpected member {name:?}"));
            };
            if name != expected_name {
                return Err(format!(
                    "{LABEL} member spelling changed: expected {expected_name:?}, got {name:?}"
                ));
            }
            let bytes = fs::read(&path)
                .map_err(|error| format!("failed to read {}: {error}", path.display()))?;
            let sha256 = format!("{:x}", Sha256::digest(&bytes));
            let content_blake3 = blake3::hash(&bytes).to_hex().to_string();
            if bytes.len() != expected_bytes
                || sha256 != expected_sha256
                || content_blake3 != expected_blake3
            {
                return Err(format!(
                    "{LABEL} retained artifact {name:?} changed: bytes={}, sha256={sha256}, blake3={content_blake3}",
                    bytes.len()
                ));
            }
            names.push(key);
            records.push(format!(
                "{FAMILY_DIRECTORY}/{expected_name}\t{expected_bytes}\t{expected_sha256}\t{expected_blake3}"
            ));
            if let Some(member_role) =
                Bug1284Role::for_record(&format!("{FAMILY_DIRECTORY}/{expected_name}"))
            {
                let source = std::str::from_utf8(&bytes)
                    .map_err(|error| format!("{LABEL} {member_role:?} is not UTF-8: {error}"))?;
                Self::validate_bug1284_role_source(member_role, source)?;
                self.reject_wrapper_output_artifacts(&path)
                    .map_err(|error| format!("{LABEL} {expected_name} {error}"))?;
                sources.insert(member_role, bytes);
            }
        }
        names.sort();
        records.sort_by_key(|record| {
            RETAINED_AUDIT_ORDER
                .iter()
                .position(|name| record.starts_with(&format!("{FAMILY_DIRECTORY}/{name}\t")))
                .unwrap_or(usize::MAX)
        });
        let stream = records.join("\n");
        let sha256 = format!("{:x}", Sha256::digest(stream.as_bytes()));
        let content_blake3 = blake3::hash(stream.as_bytes()).to_hex().to_string();
        if names != expected_artifacts.keys().cloned().collect::<Vec<_>>()
            || records.len() != RETAINED_RECORD_COUNT
            || stream.len() != RETAINED_RECORD_BYTES
            || sha256 != RETAINED_RECORDS_SHA256
            || content_blake3 != RETAINED_RECORDS_BLAKE3
            || sources.len() != Bug1284Role::ALL.len()
        {
            return Err(format!(
                "{LABEL} retained census changed: records={}/{RETAINED_RECORD_COUNT}, bytes={}/{RETAINED_RECORD_BYTES}, sha256={sha256}, blake3={content_blake3}, names={names:?}",
                records.len(),
                stream.len()
            ));
        }

        let actual_owners = self
            .upstream_wrapper_decks
            .iter()
            .filter(|record| record.starts_with(FAMILY_PREFIX))
            .cloned()
            .collect::<BTreeSet<_>>();
        if actual_owners != BTreeSet::from([Bug1284Role::WrapperOwner.record().to_string()]) {
            return Err(format!(
                "{LABEL} wrapper-owner census changed: {actual_owners:?}"
            ));
        }

        let exclusions = Self::load_upstream_exclusions(&self.root)
            .map_err(|error| format!("{LABEL} exclusions are invalid: {error}"))?;
        let actual_exclusions = exclusions
            .keys()
            .filter(|record| record.starts_with(FAMILY_PREFIX))
            .cloned()
            .collect::<BTreeSet<_>>();
        let expected_exclusions = Bug1284Role::WORKERS
            .into_iter()
            .map(|member| member.record().to_string())
            .collect::<BTreeSet<_>>();
        if actual_exclusions != expected_exclusions {
            return Err(format!(
                "{LABEL} exclusion census changed: {actual_exclusions:?}"
            ));
        }
        for member in Bug1284Role::WORKERS {
            let exclusion = exclusions
                .get(member.record())
                .ok_or_else(|| format!("{LABEL} lost {member:?} exclusion"))?;
            if exclusion.source != EXCLUSION_SOURCE
                || !matches!(
                    &exclusion.disposition,
                    XyceUpstreamExclusionDisposition::RspiceIndependentlyQualified {
                        expected_contract
                    } if expected_contract == BUG1284_WORKER_CONTRACT
                )
            {
                return Err(format!(
                    "{LABEL} {member:?} independent qualification changed: {exclusion:?}"
                ));
            }
        }
        if exclusions.contains_key(Bug1284Role::WrapperOwner.record()) {
            return Err(format!("{LABEL} wrapper owner must not be excluded"));
        }

        let output = self.root.join("OutputData/Certification_Tests/BUG_1284");
        match fs::symlink_metadata(&output) {
            Err(error) if error.kind() == std::io::ErrorKind::NotFound => {}
            Err(error) => return Err(format!("failed to inspect {LABEL} OutputData: {error}")),
            Ok(_) => return Err(format!("{LABEL} must not acquire numerical gold")),
        }
        Ok(sources)
    }

    fn bug1284_worker_plan(
        &self,
        role: Bug1284Role,
    ) -> Result<(XyceStaticTranPlan, Netlist), String> {
        if role == Bug1284Role::WrapperOwner {
            return Err(format!("{LABEL} wrapper owner is not a simulation deck"));
        }
        let deck = XyceDeck {
            path: self.root.join(role.path()),
            relative_path: role.path().to_string(),
            section: XyceDeckSection::Netlists,
        };
        let plan = self.static_tran_plan_for_deck_with_purpose(
            &deck,
            XyceStaticTranPlanPurpose::Bug1284TransientRestartRelationalFamily,
        )?;
        let netlist = Self::parse_xyce_netlist(&plan.source, &plan.deck_path)
            .map_err(|error| format!("{LABEL} {role:?} parse failed: {error}"))?;
        let print = plan.require_print(LABEL)?;
        let expected_stop = if role == Bug1284Role::First {
            FIRST_STOP
        } else {
            FINAL_STOP
        };
        let restart_is_exact = match (role, netlist.options.restart.as_ref()) {
            (Bug1284Role::Baseline, None) => true,
            (Bug1284Role::First, Some(restart)) => {
                restart.pack.is_none()
                    && restart.print_timeint_options.is_none()
                    && restart.job.as_deref() == Some(RESTART_JOB)
                    && restart.start_time.is_none()
                    && restart.file.is_none()
                    && restart.initial_interval.map(Value::to_bits)
                        == Some(RESTART_INTERVAL.to_bits())
                    && restart.intervals.is_empty()
            }
            (Bug1284Role::Restarted, Some(restart)) => {
                restart.pack.is_none()
                    && restart.print_timeint_options.is_none()
                    && restart.job.is_none()
                    && restart.start_time.is_none()
                    && restart.file.as_deref() == Some(RESTART_FILE)
                    && restart.initial_interval.is_none()
                    && restart.intervals.is_empty()
            }
            _ => false,
        };
        if plan.deck_path != deck.path
            || !matches!(plan.oracle, XyceStaticTranOracle::None)
            || plan.output_override
            || plan.timeint_conststep
            || !plan.steps.is_empty()
            || plan.tran.step.to_bits() != PRINT_STEP.to_bits()
            || plan.tran.stop.to_bits() != expected_stop.to_bits()
            || plan.tran.start.is_some()
            || plan.tran.max_step.is_some()
            || plan.tran.uic
            || print.probes.iter().map(String::as_str).ne(["V(2)", "V(3)"])
            || !restart_is_exact
        {
            return Err(format!(
                "{LABEL} {role:?} parsed plan/restart semantics changed: plan={plan:?}, restart={:?}",
                netlist.options.restart
            ));
        }
        Ok((plan, netlist))
    }

    fn bug1284_round_trip_checkpoint(
        checkpoint: &TransientCheckpoint,
    ) -> Result<TransientCheckpoint, String> {
        let text = checkpoint.to_text();
        let restored = TransientCheckpoint::from_text(&text)
            .map_err(|error| format!("{LABEL} checkpoint parse failed: {error}"))?;
        if restored != *checkpoint || restored.to_text() != text {
            return Err(format!(
                "{LABEL} checkpoint at t={:.17e} did not round-trip bit-exactly",
                checkpoint.time
            ));
        }
        Ok(restored)
    }

    fn bug1284_require_result_horizon(
        role: &str,
        result: &TransientResult,
        start: Value,
        stop: Value,
    ) -> Result<(), String> {
        Self::validate_transient_result_time_grid(result)?;
        let actual_start = result
            .time
            .first()
            .copied()
            .ok_or_else(|| format!("{LABEL} {role} produced no samples"))?;
        let actual_stop = result.time.last().copied().expect("checked nonempty");
        if actual_start.to_bits() != start.to_bits() || actual_stop.to_bits() != stop.to_bits() {
            return Err(format!(
                "{LABEL} {role} horizon changed: [{actual_start:.17e}, {actual_stop:.17e}], expected [{start:.17e}, {stop:.17e}]"
            ));
        }
        Ok(())
    }

    fn bug1284_compare_restart_relation(
        &self,
        label: &str,
        baseline: &XycePrnTable,
        restarted: &XycePrnTable,
    ) -> Result<(), String> {
        let mismatches = self.compare_bug1284_restart_tables(baseline, restarted)?;
        if mismatches.is_empty() {
            Ok(())
        } else {
            Err(format!(
                "{LABEL} {label} produced {} default xyce_verify mismatch(es): {mismatches:?}",
                mismatches.len()
            ))
        }
    }

    pub(super) fn validate_bug1284_oracle(
        &self,
        deck: &XyceDeck,
        role: Bug1284Role,
        start: Instant,
    ) -> Result<(), String> {
        let abort = DeadlineAbort::new(start, self.config.max_time_per_test_ms.max(1));
        if abort.is_aborted() {
            return Err(format!(
                "{LABEL} deadline expired before provenance validation"
            ));
        }
        self.validate_bug1284_provenance(deck, role)?;
        let (baseline_plan, baseline_netlist) = self.bug1284_worker_plan(Bug1284Role::Baseline)?;
        let (first_plan, first_netlist) = self.bug1284_worker_plan(Bug1284Role::First)?;
        let (restarted_plan, restarted_netlist) =
            self.bug1284_worker_plan(Bug1284Role::Restarted)?;

        let baseline_max_step =
            Self::transient_family_max_step(&baseline_netlist, &baseline_plan.tran)?;
        let first_max_step = Self::transient_family_max_step(&first_netlist, &first_plan.tran)?;
        let startup_mode = TransientStartupMode::from_uic(false);
        let baseline_engine = self.create_xyce_static_tran_engine(
            None,
            Self::xyce_initial_timestep_for_tran(&baseline_plan.tran),
        );
        let baseline_result = baseline_engine
            .run_tran_with_startup_mode_and_abort(
                &baseline_netlist,
                FINAL_STOP,
                baseline_max_step,
                startup_mode,
                &abort,
            )
            .map_err(|error| format!("{LABEL} baseline run failed: {error}"))?;
        Self::bug1284_require_result_horizon("baseline", &baseline_result, 0.0, FINAL_STOP)?;

        let first_engine = self.create_xyce_static_tran_engine(
            None,
            Self::xyce_initial_timestep_for_tran(&first_plan.tran),
        );
        let (first_result, checkpoints) = first_engine
            .run_tran_checkpoint_schedule_with_startup_mode_and_abort(
                &first_netlist,
                FIRST_STOP,
                first_max_step,
                startup_mode,
                &HISTORICAL_SAVE_TIMES,
                &abort,
            )
            .map_err(|error| format!("{LABEL} first/checkpoint run failed: {error}"))?;
        Self::bug1284_require_result_horizon("first", &first_result, 0.0, FIRST_STOP)?;
        if checkpoints.len() != HISTORICAL_SAVE_TIMES.len()
            || checkpoints
                .iter()
                .map(|scheduled| scheduled.nominal_time.to_bits())
                .ne(HISTORICAL_SAVE_TIMES.map(Value::to_bits))
        {
            return Err(format!(
                "{LABEL} nominal checkpoint schedule changed: {:?}",
                checkpoints
                    .iter()
                    .map(|scheduled| scheduled.nominal_time)
                    .collect::<Vec<_>>()
            ));
        }
        for (index, scheduled) in checkpoints.iter().enumerate() {
            let actual = scheduled.checkpoint.time;
            let next_nominal = HISTORICAL_SAVE_TIMES.get(index + 1).copied();
            if !actual.is_finite()
                || actual < scheduled.nominal_time
                || next_nominal.is_some_and(|next| actual >= next)
                || actual > FIRST_STOP
            {
                return Err(format!(
                    "{LABEL} checkpoint for nominal {:.17e} was captured outside its accepted-step window at {actual:.17e}; next nominal={next_nominal:?}",
                    scheduled.nominal_time
                ));
            }
        }

        let checkpoint_at = |time: Value| -> Result<TransientCheckpoint, String> {
            let scheduled = checkpoints
                .iter()
                .find(|scheduled| scheduled.nominal_time.to_bits() == time.to_bits())
                .ok_or_else(|| format!("{LABEL} omitted checkpoint at {time:.17e}"))?;
            Self::bug1284_round_trip_checkpoint(&scheduled.checkpoint)
        };
        let historical_checkpoint = checkpoint_at(FIRST_STOP)?;
        let strengthened_checkpoint = checkpoint_at(RESTART_INTERVAL)?;
        let resume =
            |label: &str, checkpoint: &TransientCheckpoint| -> Result<TransientResult, String> {
                let engine = self.create_xyce_static_tran_engine(
                    None,
                    Self::xyce_initial_timestep_for_tran(&restarted_plan.tran),
                );
                let (result, final_checkpoint) = engine
                    .run_tran_restart_resume_with_abort(
                        &restarted_netlist,
                        checkpoint,
                        FINAL_STOP,
                        first_max_step,
                        &abort,
                    )
                    .map_err(|error| format!("{LABEL} {label} resume failed: {error}"))?;
                Self::bug1284_require_result_horizon(label, &result, checkpoint.time, FINAL_STOP)?;
                if final_checkpoint.time.to_bits() != FINAL_STOP.to_bits() {
                    return Err(format!(
                        "{LABEL} {label} final checkpoint stopped at {:.17e}",
                        final_checkpoint.time
                    ));
                }
                Ok(result)
            };
        let historical_result = resume("historical 20 ns", &historical_checkpoint)?;
        let strengthened_result = resume("strengthened 5 ns", &strengthened_checkpoint)?;

        let baseline_table = Self::transient_family_result_to_prn_table(
            &baseline_plan,
            &baseline_netlist,
            &baseline_result,
        )?;
        let historical_table = Self::transient_family_result_to_prn_table(
            &restarted_plan,
            &restarted_netlist,
            &historical_result,
        )?;
        let strengthened_table = Self::transient_family_result_to_prn_table(
            &restarted_plan,
            &restarted_netlist,
            &strengthened_result,
        )?;
        self.bug1284_compare_restart_relation(
            "historical 20 ns relation",
            &baseline_table,
            &historical_table,
        )?;
        self.bug1284_compare_restart_relation(
            "strengthened nonquiescent 5 ns relation",
            &baseline_table,
            &strengthened_table,
        )?;
        if abort.is_aborted() {
            return Err(format!("{LABEL} execution exceeded its deadline"));
        }
        self.validate_bug1284_provenance(deck, role)?;
        Ok(())
    }

    /// Apply the unmodified Release-7.10 default `xyce_verify` transient metric.
    /// The baseline is the GOOD table and the resumed tail is the TEST table.
    pub(super) fn compare_bug1284_restart_tables(
        &self,
        baseline: &XycePrnTable,
        restarted: &XycePrnTable,
    ) -> Result<Vec<XyceValueMismatch>, String> {
        let expected_columns = ["Index", "TIME", "V(2)", "V(3)"];
        for (label, table) in [("baseline", baseline), ("restarted", restarted)] {
            if table
                .columns
                .iter()
                .map(String::as_str)
                .ne(expected_columns)
            {
                return Err(format!(
                    "{LABEL} {label} projection changed: {:?}",
                    table.columns
                ));
            }
        }
        self.compare_xyce_verify_transient_tables_with_uniform_tolerance(
            baseline,
            restarted,
            XyceVerifyTransientTolerance::release_7_10_default(),
            XYCE_DEFAULT_PRN_SCIENTIFIC_PRECISION,
        )
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

    fn deck(root: &Path, role: Bug1284Role) -> XyceDeck {
        XyceDeck {
            path: root.join(role.path()),
            relative_path: role.path().to_string(),
            section: XyceDeckSection::Netlists,
        }
    }

    fn bug1284_fixture(label: &str) -> (tempfile::TempDir, XyceDeck) {
        let temporary = tempfile::Builder::new()
            .prefix(&format!("rspice-xyce-bug1284-{label}-"))
            .tempdir()
            .expect("create BUG1284 fixture root");
        let root = temporary.path();
        let family = root.join(FAMILY_DIRECTORY);
        fs::create_dir_all(&family).expect("create BUG1284 fixture family");
        let canonical = corpus_root().join(FAMILY_DIRECTORY);
        for (name, ..) in RETAINED_ARTIFACTS {
            fs::copy(canonical.join(name), family.join(name))
                .expect("copy BUG1284 retained artifact");
        }
        fs::write(
            root.join(HARNESS_MANIFEST_FILE),
            format!(
                "{}\t{REQUIRES_UPSTREAM_WRAPPER_CONTRACT}\n",
                Bug1284Role::WrapperOwner.path()
            ),
        )
        .expect("write BUG1284 fixture harness manifest");
        let mut exclusions = vec![
            format!("schema_version\t{UPSTREAM_EXCLUSIONS_SCHEMA_VERSION}"),
            format!("source_commit\t{UPSTREAM_EXCLUSIONS_SOURCE_COMMIT}"),
            format!("source_netlists_tree\t{UPSTREAM_EXCLUSIONS_SOURCE_NETLISTS_TREE}"),
        ];
        exclusions.extend(Bug1284Role::WORKERS.map(|role| {
            format!(
                "{}\t{EXCLUSION_SOURCE}\t{RSPICE_INDEPENDENTLY_QUALIFIED_DISPOSITION}\t{BUG1284_WORKER_CONTRACT}",
                role.path()
            )
        }));
        fs::write(
            root.join(UPSTREAM_EXCLUSIONS_MANIFEST_FILE),
            exclusions.join("\n") + "\n",
        )
        .expect("write BUG1284 fixture exclusions");
        let owner = deck(root, Bug1284Role::WrapperOwner);
        (temporary, owner)
    }

    #[test]
    fn bug1284_roles_paths_and_future_contracts_are_bijective() {
        let records = Bug1284Role::ALL
            .into_iter()
            .map(Bug1284Role::record)
            .collect::<BTreeSet<_>>();
        assert_eq!(records.len(), Bug1284Role::ALL.len());
        for role in Bug1284Role::ALL {
            assert_eq!(Bug1284Role::for_record(role.path()), Some(role));
            assert_eq!(Bug1284Role::for_record(role.record()), Some(role));
            assert_eq!(
                role.contract(),
                if role == Bug1284Role::WrapperOwner {
                    BUG1284_OWNER_CONTRACT
                } else {
                    BUG1284_WORKER_CONTRACT
                }
            );
        }
        assert_eq!(Bug1284Role::for_record("BUG_1284/invented.cir"), None);
    }

    #[test]
    fn bug1284_historical_and_retained_provenance_is_exact() {
        XyceTestRunner::validate_bug1284_historical_provenance()
            .expect("historical BUG1284 provenance remains exact");
        let root = corpus_root();
        let runner = XyceTestRunner::new(&root, XyceRunnerConfig::default());
        for role in Bug1284Role::ALL {
            runner
                .validate_bug1284_provenance(&deck(&root, role), role)
                .unwrap_or_else(|error| panic!("{role:?} provenance failed: {error}"));
        }
    }

    #[test]
    fn bug1284_historical_restart_schedule_and_filename_are_exact() {
        let oracle = XyceTestRunner::bug1284_restart_oracle();
        assert_eq!(oracle.print_step.to_bits(), (0.25 * 1.0e-9f64).to_bits());
        assert_eq!(oracle.first_stop.to_bits(), (20.0 * 1.0e-9f64).to_bits());
        assert_eq!(oracle.final_stop.to_bits(), (50.0 * 1.0e-9f64).to_bits());
        assert_eq!(
            oracle.restart_interval.to_bits(),
            (5.0 * 1.0e-9f64).to_bits()
        );
        assert_eq!(
            oracle.save_times.map(Value::to_bits),
            [
                0.0,
                5.0 * 1.0e-9,
                10.0 * 1.0e-9,
                15.0 * 1.0e-9,
                20.0 * 1.0e-9,
            ]
            .map(Value::to_bits)
        );
        assert_eq!(oracle.restart_job, "trans_test");
        assert_eq!(oracle.restart_file, "trans_test2e-08");
    }

    #[test]
    fn bug1284_restart_options_cannot_masquerade_as_analytic_timeint_only_options() {
        let path = corpus_root().join(Bug1284Role::First.path());
        let source = fs::read_to_string(&path).expect("read BUG1284 first-run deck");
        let netlist = XyceTestRunner::parse_xyce_netlist(&source, &path)
            .expect("parse BUG1284 first-run deck");
        assert!(netlist.options.restart.is_some());
        assert!(!XyceTestRunner::analytic_timeint_only_options_match(
            &netlist.options,
            None,
            None,
            None,
            None,
        ));
    }

    #[test]
    fn bug1284_provenance_rejects_source_census_manifest_exclusion_and_output_drift() {
        let (temporary, owner) = bug1284_fixture("canonical");
        XyceTestRunner::new(temporary.path(), XyceRunnerConfig::default())
            .validate_bug1284_provenance(&owner, Bug1284Role::WrapperOwner)
            .expect("canonical BUG1284 fixture passes");

        let (temporary, owner) = bug1284_fixture("source");
        fs::write(
            temporary.path().join(Bug1284Role::First.path()),
            "Transmission Line Circuit\n* changed\n.end\n",
        )
        .expect("mutate BUG1284 source");
        assert!(
            XyceTestRunner::new(temporary.path(), XyceRunnerConfig::default())
                .validate_bug1284_provenance(&owner, Bug1284Role::WrapperOwner)
                .is_err()
        );

        let (temporary, owner) = bug1284_fixture("census");
        fs::write(
            temporary.path().join(FAMILY_DIRECTORY).join("invented.cir"),
            "* invented\n",
        )
        .expect("add BUG1284 family member");
        assert!(
            XyceTestRunner::new(temporary.path(), XyceRunnerConfig::default())
                .validate_bug1284_provenance(&owner, Bug1284Role::WrapperOwner)
                .is_err()
        );

        let (temporary, owner) = bug1284_fixture("manifest");
        fs::write(
            temporary.path().join(HARNESS_MANIFEST_FILE),
            format!(
                "{}\t{REQUIRES_UPSTREAM_WRAPPER_CONTRACT}\n",
                Bug1284Role::Baseline.path()
            ),
        )
        .expect("mutate BUG1284 owner manifest");
        assert!(
            XyceTestRunner::new(temporary.path(), XyceRunnerConfig::default())
                .validate_bug1284_provenance(&owner, Bug1284Role::WrapperOwner)
                .is_err()
        );

        let (temporary, owner) = bug1284_fixture("exclusion");
        let exclusions = temporary.path().join(UPSTREAM_EXCLUSIONS_MANIFEST_FILE);
        let text = fs::read_to_string(&exclusions).expect("read BUG1284 exclusions");
        fs::write(
            &exclusions,
            text.replacen(EXCLUSION_SOURCE, "invented/exclude", 1),
        )
        .expect("mutate BUG1284 exclusion provenance");
        assert!(
            XyceTestRunner::new(temporary.path(), XyceRunnerConfig::default())
                .validate_bug1284_provenance(&owner, Bug1284Role::WrapperOwner)
                .is_err()
        );

        let (temporary, owner) = bug1284_fixture("output");
        fs::create_dir_all(
            temporary
                .path()
                .join("OutputData/Certification_Tests/BUG_1284"),
        )
        .expect("create fabricated BUG1284 numerical gold");
        assert!(
            XyceTestRunner::new(temporary.path(), XyceRunnerConfig::default())
                .validate_bug1284_provenance(&owner, Bug1284Role::WrapperOwner)
                .is_err()
        );
    }

    #[test]
    fn bug1284_default_integrated_rms_oracle_accepts_equal_tail_and_rejects_history_loss() {
        let runner = XyceTestRunner::new(corpus_root(), XyceRunnerConfig::default());
        let columns = ["Index", "TIME", "V(2)", "V(3)"]
            .map(str::to_string)
            .to_vec();
        let baseline = XycePrnTable {
            columns: columns.clone(),
            rows: vec![
                vec![0.0, 0.0, 0.0, 0.0],
                vec![1.0, 5.0e-9, 2.5, 0.0],
                vec![2.0, 10.0e-9, 2.5, 2.5],
                vec![3.0, 20.0e-9, 0.0, 2.5],
                vec![4.0, 50.0e-9, 0.0, 0.0],
            ],
        };
        let restarted = XycePrnTable {
            columns: columns.clone(),
            rows: baseline.rows[2..].to_vec(),
        };
        assert!(
            runner
                .compare_bug1284_restart_tables(&baseline, &restarted)
                .expect("compare exact resumed tail")
                .is_empty()
        );

        let mut history_lost = restarted;
        history_lost.rows[0][3] = 0.0;
        history_lost.rows[1][3] = 0.0;
        assert!(
            !runner
                .compare_bug1284_restart_tables(&baseline, &history_lost)
                .expect("compare lost transmission-line history")
                .is_empty()
        );

        let malformed = XycePrnTable {
            columns: vec!["Index".into(), "TIME".into(), "V(2)".into()],
            rows: vec![vec![0.0, 0.0, 0.0], vec![1.0, 1.0, 0.0]],
        };
        assert!(
            runner
                .compare_bug1284_restart_tables(&baseline, &malformed)
                .is_err()
        );
    }
}
