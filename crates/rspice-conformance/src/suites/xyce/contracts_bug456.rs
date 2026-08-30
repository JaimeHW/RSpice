use super::*;
use rspice_core::engine::{
    SimulationConfigOverrides, TransientCheckpointEncoding, XyceRestartJobPlan,
    resolve_simulation_config,
};

const LABEL: &str = "BUG_456 transient-restart wrapper family";
const FAMILY_DIRECTORY: &str = "Netlists/Certification_Tests/BUG_456";
const FAMILY_PREFIX: &str = "netlists/certification_tests/bug_456/";
const OUTPUT_DIRECTORY: &str = "OutputData/Certification_Tests/BUG_456";
const EXCLUSION_SOURCE: &str = "Netlists/Certification_Tests/BUG_456/exclude";

pub(super) const BUG456_OWNER_CONTRACT: &str = "bug456_transient_restart_relational_wrapper_owner";
pub(super) const BUG456_WORKER_CONTRACT: &str = "bug456_transient_restart_relational_worker";

// Immutable upstream identities. The pre-trim commit contains the complete
// Release-7.10 family, including the eight Perl wrappers and their tag files.
const PRETRIM_COMMIT: &str = "80115a9277c0ddb3409acceb3d4e745fd11cddd4";
const PRETRIM_NETLISTS_TREE: &str = "3e34bfaafa890cb2e4457137b6a0e325c8c1e87d";
const UPSTREAM_COMMIT: &str = "d6e278e371ec2f3df1325dcff4552e585bc7ecc1";
const RELEASE_TAG: &str = "Release-7.10.0";
const RELEASE_TAG_OBJECT: &str = "2a339ec3845af0aef99a7e6cc488a41acf64f6ed";
const HISTORICAL_FAMILY_TREE: &str = "2595ff1c6a2f616beb6d449e3f3e2bfb9a056d27";
const RETAINED_FAMILY_TREE: &str = "90db8e26d27dca6fd334686b81aa68ff03d98b0c";
const HISTORICAL_RECORD_COUNT: usize = 42;
const HISTORICAL_FAMILY_CONTENT_BYTES: usize = 59_930;
const HISTORICAL_FAMILY_STREAM_BYTES: usize = 17_322;
const HISTORICAL_FAMILY_STREAM_SHA256: &str =
    "a2c9934a1268f998e34f663c340fcbe10d7ef723e8ec88623b9b0fdb01c3a03b";
const HISTORICAL_FAMILY_STREAM_BLAKE3: &str =
    "93dac9eda10b472ba7f2025713bdfcedfbd38147f9c5081627d12dc6217167cb";
const HISTORICAL_REMOVED: [&str; 18] = [
    "Manifest.txt",
    "bug456.cir.sh",
    "bug456.cir.tags",
    "bug456_unpacked.cir.sh",
    "bug456_unpacked.cir.tags",
    "bug456emit.cir.sh",
    "bug456emit.cir.tags",
    "bug456emit_gear.cir.sh",
    "bug456emit_gear.cir.tags",
    "bug456output.cir.sh",
    "bug456output.cir.tags",
    "bug456pp.cir.sh",
    "bug456pp.cir.tags",
    "bug456pp_unpacked.cir.sh",
    "bug456pp_unpacked.cir.tags",
    "exclude",
    "simple.cir.sh",
    "simple.cir.tags",
];

// xyce_verify.pl's direct import closure is the three XyceVerify modules
// below. Tools.pm is pinned separately in the same stream because it belongs
// to the regression harness that dispatched the historical wrappers. These
// are the immutable pre-trim vendored identities, not a claim about a
// separately recovered original-upstream Tools.pm revision.
const HISTORICAL_DEPENDENCIES: [(&str, usize, &str, &str, &str); 5] = [
    (
        "TestScripts/XyceRegression/Tools.pm",
        68_404,
        "17def57575eb3fd703978fd0634a58d6e679a3de",
        "a86524def2895930f2bb697c058850f231df6d623c279bd7482f30c5c39090b4",
        "11483735d34385359bbe0f981cbc8767c11e6c2b60c3f8723d49e2761479023a",
    ),
    (
        "TestScripts/XyceVerify/DCSources.pm",
        2_739,
        "5e2c06cc593fb9e89cefb221f274901b227342eb",
        "b2ddcab5ad5a89c428b9b4430190fa27ef7106da7e7afeb31452c81890a9a006",
        "0905f9dc79d7c5bdbe17e3c2360cd063d6fcbf41823a410f98b236783d109ad7",
    ),
    (
        "TestScripts/XyceVerify/DCSweep.pm",
        9_301,
        "dbd97a554c93829be74ff8a004f7b97f507be591",
        "2246da2374e6cce3ea516a50e472fb07f7481e8b0effb20d4a650e6b6cb1eda0",
        "b9cc7d905d001ebe2ace44936b9631e4bdcbf42bca4d4b34c5866262cd11d9a3",
    ),
    (
        "TestScripts/XyceVerify/StepSweep.pm",
        8_731,
        "6ba454fc66c19d883c7a8e29c4894eaf364b1f4b",
        "84b2d485c1848f2e456463de8a5015205d87c3db8a6d070547d6f9464618fed6",
        "db1b142ab3ae9163bbe02bd68b5b3a6311436adbf27c06d71a5c05df9b6973e7",
    ),
    (
        "TestScripts/xyce_verify.pl",
        59_566,
        "5809bf44e921762c87b658f096d34f81aca5ccfb",
        "6e5f84b1646b30d0e12879848d7653584b39472d640a14916ae8fda6e1df12b3",
        "5eadb6dab06ed3091ea114146bd4a574de83784f87be9843ad7b721b0a793665",
    ),
];
const HISTORICAL_DEPENDENCY_CONTENT_BYTES: usize = 148_741;
const HISTORICAL_DEPENDENCY_STREAM_BYTES: usize = 1_945;
const HISTORICAL_DEPENDENCY_STREAM_SHA256: &str =
    "33b5831139b00229c28e89517ec311534fd772dc697b6c42990ad9ab1174f1f6";
const HISTORICAL_DEPENDENCY_STREAM_BLAKE3: &str =
    "58bb81da9e1c51739783db572336099587e1a93b0778908fb5444b04b2d046b6";

const RETAINED_CONTENT_BYTES: usize = 33_061;
const RETAINED_STREAM_BYTES: usize = 3_101;
const RETAINED_STREAM_SHA256: &str =
    "7f53261adbf8a8acdf27175f59c4ad0c04e6068ec425cf14932ae528cd643127";
const RETAINED: [(&str, usize, &str); 24] = [
    (
        "bug456.cir",
        79,
        "90677fcfcdfdc4ee0e9b00b70c9d457f7546f70672a0d8fc7e0b74d3afce499d",
    ),
    (
        "bug456_unpacked.cir",
        79,
        "90677fcfcdfdc4ee0e9b00b70c9d457f7546f70672a0d8fc7e0b74d3afce499d",
    ),
    (
        "bug456emit.cir",
        88,
        "fc076620b72c1ea3aff5460eeccf10b0f42e14a4c243d60c5d175392f82d2a72",
    ),
    (
        "bug456emit_gear.cir",
        88,
        "fc076620b72c1ea3aff5460eeccf10b0f42e14a4c243d60c5d175392f82d2a72",
    ),
    (
        "bug456output.cir",
        79,
        "90677fcfcdfdc4ee0e9b00b70c9d457f7546f70672a0d8fc7e0b74d3afce499d",
    ),
    (
        "bug456pp.cir",
        79,
        "90677fcfcdfdc4ee0e9b00b70c9d457f7546f70672a0d8fc7e0b74d3afce499d",
    ),
    (
        "bug456pp_unpacked.cir",
        79,
        "90677fcfcdfdc4ee0e9b00b70c9d457f7546f70672a0d8fc7e0b74d3afce499d",
    ),
    (
        "converter_baseline.cir",
        1_738,
        "51d3ad8bc50357787c85b3aa7490fc1372eac764bb08c48b424355fb7781169c",
    ),
    (
        "converter_baseline_output.cir",
        1_746,
        "05362200b248645bb0c339a4159785545a6a4f182cbcb160fc80b79e1df29f1c",
    ),
    (
        "converter_baseline_unpacked.cir",
        1_720,
        "1ee9edbe9f95d93fa6e74e5b7dbc13b96dc5172c6856b1042c8d5202e7b0c322",
    ),
    (
        "converter_restart.cir",
        1_740,
        "f21c648fd343832c93bbc8baf2b16c67263c3afcd142a6cfdcbf54b312dd3367",
    ),
    (
        "converter_restart_output.cir",
        1_764,
        "47863fd6b05606af36c516192bf4a5c3f8fe201f645462aa1ad7dfd29a7fffbd",
    ),
    (
        "converter_restart_unpacked.cir",
        1_732,
        "de9800a31b973bfa99cdd4d01557cd1cd5689937a612a1b1b27912d7884ae52b",
    ),
    (
        "emitter_baseline.cir",
        976,
        "3413fc815161030cc8bbe77c5173e3279312a53237b65518e5bb99d31fdad315",
    ),
    (
        "emitter_baseline_gear.cir",
        1_009,
        "406d95adb204aa2509315403a5e195ca1fedfc6b1a362c4a639c54bbed91957f",
    ),
    (
        "emitter_restart.cir",
        958,
        "aababacf16a5367c9c733a225f0a012966a67a02e479ae0211573f7ada525dd0",
    ),
    (
        "emitter_restart_gear.cir",
        991,
        "9bc39d06d142d6efdfea6e9fd219f89d01be57c0fb600c09df55dd2079f03097",
    ),
    (
        "push_pull_xyce_baseline.cir",
        4_328,
        "5640d51decc783329da7fc9a097ad0267e982769a1a8df2a5bac9cdcc4945198",
    ),
    (
        "push_pull_xyce_baseline_unpacked.cir",
        4_346,
        "574d04136a131a34c73f67ba74a11aff9f20beda1b3bae1e70500d747dd9dedd",
    ),
    (
        "push_pull_xyce_restart.cir",
        4_344,
        "5a523b4170197dc624b14f75e81cbbb2996d52745aa16b036774b29b9dd96116",
    ),
    (
        "push_pull_xyce_restart_unpacked.cir",
        4_370,
        "7b4c07602145610984f7903435fda669172419e8619a1852f99cc217069655bc",
    ),
    (
        "simple.cir",
        86,
        "a835d7da822dfbebd6d5ad66b4b25032884238fb4d851cc4db7b8ebad8f9a35b",
    ),
    (
        "simple_baseline.cir",
        321,
        "f238060767c8fe25bb9b1957808e98720186160fbb2e0932fb730eaa034b23ca",
    ),
    (
        "simple_restart.cir",
        321,
        "ca9cbe5454829105692253b725732dfd3e3bb8d866ba5039ec0752b3d6965264",
    ),
];

const MAX_DIRECTORY_ENTRIES: usize = 16_384;
const MAX_MANIFEST_BYTES: u64 = 1_048_576;
const MAX_CHECKPOINT_BYTES: usize = 16 * 1024 * 1024;
// The exact simple.cir adaptive trajectory has just over 200k accepted
// points at Xyce's derived maximum step. Keep a bounded 25% safety margin;
// this is still below the runner's native 250k transient-oracle budget.
const MAX_RESULT_ROWS: usize = 250_000;
const MAX_RESTART_POINTS: usize = 64;
const HISTORICAL_GRID_ZERO_TOLERANCE: Value = 10.0e-15;

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub(super) enum Bug456Case {
    ConverterPacked,
    ConverterUnpacked,
    ConverterOutput,
    EmitterTrapGear,
    EmitterGear,
    PushPullPacked,
    PushPullUnpacked,
    SimpleUnpacked,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub(super) enum Bug456Member {
    Owner,
    Baseline,
    Restart,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub(super) struct Bug456Role {
    case: Bug456Case,
    member: Bug456Member,
}

impl Bug456Role {
    pub(super) const ALL: [Self; 24] = [
        Self::new(Bug456Case::ConverterPacked, Bug456Member::Owner),
        Self::new(Bug456Case::ConverterPacked, Bug456Member::Baseline),
        Self::new(Bug456Case::ConverterPacked, Bug456Member::Restart),
        Self::new(Bug456Case::ConverterUnpacked, Bug456Member::Owner),
        Self::new(Bug456Case::ConverterUnpacked, Bug456Member::Baseline),
        Self::new(Bug456Case::ConverterUnpacked, Bug456Member::Restart),
        Self::new(Bug456Case::ConverterOutput, Bug456Member::Owner),
        Self::new(Bug456Case::ConverterOutput, Bug456Member::Baseline),
        Self::new(Bug456Case::ConverterOutput, Bug456Member::Restart),
        Self::new(Bug456Case::EmitterTrapGear, Bug456Member::Owner),
        Self::new(Bug456Case::EmitterTrapGear, Bug456Member::Baseline),
        Self::new(Bug456Case::EmitterTrapGear, Bug456Member::Restart),
        Self::new(Bug456Case::EmitterGear, Bug456Member::Owner),
        Self::new(Bug456Case::EmitterGear, Bug456Member::Baseline),
        Self::new(Bug456Case::EmitterGear, Bug456Member::Restart),
        Self::new(Bug456Case::PushPullPacked, Bug456Member::Owner),
        Self::new(Bug456Case::PushPullPacked, Bug456Member::Baseline),
        Self::new(Bug456Case::PushPullPacked, Bug456Member::Restart),
        Self::new(Bug456Case::PushPullUnpacked, Bug456Member::Owner),
        Self::new(Bug456Case::PushPullUnpacked, Bug456Member::Baseline),
        Self::new(Bug456Case::PushPullUnpacked, Bug456Member::Restart),
        Self::new(Bug456Case::SimpleUnpacked, Bug456Member::Owner),
        Self::new(Bug456Case::SimpleUnpacked, Bug456Member::Baseline),
        Self::new(Bug456Case::SimpleUnpacked, Bug456Member::Restart),
    ];

    pub(super) const OWNERS: [Self; 8] = [
        Self::new(Bug456Case::ConverterPacked, Bug456Member::Owner),
        Self::new(Bug456Case::ConverterUnpacked, Bug456Member::Owner),
        Self::new(Bug456Case::ConverterOutput, Bug456Member::Owner),
        Self::new(Bug456Case::EmitterTrapGear, Bug456Member::Owner),
        Self::new(Bug456Case::EmitterGear, Bug456Member::Owner),
        Self::new(Bug456Case::PushPullPacked, Bug456Member::Owner),
        Self::new(Bug456Case::PushPullUnpacked, Bug456Member::Owner),
        Self::new(Bug456Case::SimpleUnpacked, Bug456Member::Owner),
    ];

    pub(super) const WORKERS: [Self; 16] = [
        Self::new(Bug456Case::ConverterPacked, Bug456Member::Baseline),
        Self::new(Bug456Case::ConverterPacked, Bug456Member::Restart),
        Self::new(Bug456Case::ConverterUnpacked, Bug456Member::Baseline),
        Self::new(Bug456Case::ConverterUnpacked, Bug456Member::Restart),
        Self::new(Bug456Case::ConverterOutput, Bug456Member::Baseline),
        Self::new(Bug456Case::ConverterOutput, Bug456Member::Restart),
        Self::new(Bug456Case::EmitterTrapGear, Bug456Member::Baseline),
        Self::new(Bug456Case::EmitterTrapGear, Bug456Member::Restart),
        Self::new(Bug456Case::EmitterGear, Bug456Member::Baseline),
        Self::new(Bug456Case::EmitterGear, Bug456Member::Restart),
        Self::new(Bug456Case::PushPullPacked, Bug456Member::Baseline),
        Self::new(Bug456Case::PushPullPacked, Bug456Member::Restart),
        Self::new(Bug456Case::PushPullUnpacked, Bug456Member::Baseline),
        Self::new(Bug456Case::PushPullUnpacked, Bug456Member::Restart),
        Self::new(Bug456Case::SimpleUnpacked, Bug456Member::Baseline),
        Self::new(Bug456Case::SimpleUnpacked, Bug456Member::Restart),
    ];

    const fn new(case: Bug456Case, member: Bug456Member) -> Self {
        Self { case, member }
    }

    pub(super) fn for_record(record: &str) -> Option<Self> {
        let normalized = XyceTestRunner::normalize_manifest_key(record);
        Self::ALL
            .into_iter()
            .find(|role| role.record() == normalized)
    }

    pub(super) const fn is_owner(self) -> bool {
        matches!(self.member, Bug456Member::Owner)
    }

    /// Owners enter the authoritative passing census only after their exact
    /// case-local save/resume oracle has completed successfully. This list is
    /// advanced methodically as the family is qualified.
    pub(super) const fn is_qualified_owner(self) -> bool {
        matches!(
            (self.case, self.member),
            (
                Bug456Case::ConverterUnpacked | Bug456Case::SimpleUnpacked,
                Bug456Member::Owner
            )
        )
    }

    pub(super) const fn contract(self) -> &'static str {
        if self.is_owner() {
            BUG456_OWNER_CONTRACT
        } else {
            BUG456_WORKER_CONTRACT
        }
    }

    const fn file_name(self) -> &'static str {
        match (self.case, self.member) {
            (Bug456Case::ConverterPacked, Bug456Member::Owner) => "bug456.cir",
            (Bug456Case::ConverterPacked, Bug456Member::Baseline) => "converter_baseline.cir",
            (Bug456Case::ConverterPacked, Bug456Member::Restart) => "converter_restart.cir",
            (Bug456Case::ConverterUnpacked, Bug456Member::Owner) => "bug456_unpacked.cir",
            (Bug456Case::ConverterUnpacked, Bug456Member::Baseline) => {
                "converter_baseline_unpacked.cir"
            }
            (Bug456Case::ConverterUnpacked, Bug456Member::Restart) => {
                "converter_restart_unpacked.cir"
            }
            (Bug456Case::ConverterOutput, Bug456Member::Owner) => "bug456output.cir",
            (Bug456Case::ConverterOutput, Bug456Member::Baseline) => {
                "converter_baseline_output.cir"
            }
            (Bug456Case::ConverterOutput, Bug456Member::Restart) => "converter_restart_output.cir",
            (Bug456Case::EmitterTrapGear, Bug456Member::Owner) => "bug456emit.cir",
            (Bug456Case::EmitterTrapGear, Bug456Member::Baseline) => "emitter_baseline.cir",
            (Bug456Case::EmitterTrapGear, Bug456Member::Restart) => "emitter_restart.cir",
            (Bug456Case::EmitterGear, Bug456Member::Owner) => "bug456emit_gear.cir",
            (Bug456Case::EmitterGear, Bug456Member::Baseline) => "emitter_baseline_gear.cir",
            (Bug456Case::EmitterGear, Bug456Member::Restart) => "emitter_restart_gear.cir",
            (Bug456Case::PushPullPacked, Bug456Member::Owner) => "bug456pp.cir",
            (Bug456Case::PushPullPacked, Bug456Member::Baseline) => "push_pull_xyce_baseline.cir",
            (Bug456Case::PushPullPacked, Bug456Member::Restart) => "push_pull_xyce_restart.cir",
            (Bug456Case::PushPullUnpacked, Bug456Member::Owner) => "bug456pp_unpacked.cir",
            (Bug456Case::PushPullUnpacked, Bug456Member::Baseline) => {
                "push_pull_xyce_baseline_unpacked.cir"
            }
            (Bug456Case::PushPullUnpacked, Bug456Member::Restart) => {
                "push_pull_xyce_restart_unpacked.cir"
            }
            (Bug456Case::SimpleUnpacked, Bug456Member::Owner) => "simple.cir",
            (Bug456Case::SimpleUnpacked, Bug456Member::Baseline) => "simple_baseline.cir",
            (Bug456Case::SimpleUnpacked, Bug456Member::Restart) => "simple_restart.cir",
        }
    }

    pub(super) fn path(self) -> String {
        format!("{FAMILY_DIRECTORY}/{}", self.file_name())
    }

    pub(super) fn record(self) -> String {
        format!("{FAMILY_PREFIX}{}", self.file_name().to_ascii_lowercase())
    }
}

#[derive(Debug, Clone, Copy)]
struct Bug456CaseSpec {
    case: Bug456Case,
    job: &'static str,
    packed: bool,
    stop: Value,
    seam: Value,
    print_step: Value,
    probes: &'static [&'static str],
    historical_grid_threshold: Option<Value>,
}

impl Bug456CaseSpec {
    const fn for_case(case: Bug456Case) -> Self {
        match case {
            Bug456Case::ConverterPacked => Self {
                case,
                job: "converter",
                packed: true,
                stop: 4.0e-4,
                seam: 2.0e-4,
                print_step: 0.0,
                probes: &["V(N8)"],
                historical_grid_threshold: Some(2.0e-4),
            },
            Bug456Case::ConverterUnpacked => Self {
                case,
                job: "converter_unpacked",
                packed: false,
                stop: 4.0e-4,
                seam: 2.0e-4,
                print_step: 0.0,
                probes: &["V(N8)"],
                historical_grid_threshold: None,
            },
            Bug456Case::ConverterOutput => Self {
                case,
                job: "converter_output",
                packed: true,
                stop: 4.0e-4,
                seam: 2.0e-4,
                print_step: 0.0,
                probes: &["V(N8)"],
                historical_grid_threshold: None,
            },
            Bug456Case::EmitterTrapGear => Self {
                case,
                job: "emitter",
                packed: true,
                stop: 5.0e-5,
                seam: 2.0e-5,
                print_step: 0.0,
                probes: &["V(1)", "V(Ve)"],
                historical_grid_threshold: Some(2.0e-5),
            },
            Bug456Case::EmitterGear => Self {
                case,
                job: "emittergear",
                packed: true,
                stop: 5.0e-5,
                seam: 2.0e-5,
                print_step: 0.0,
                probes: &["V(1)", "V(Ve)"],
                historical_grid_threshold: Some(2.0e-5),
            },
            Bug456Case::PushPullPacked => Self {
                case,
                job: "push_pull",
                packed: true,
                stop: 1.0e-3,
                seam: 5.0e-4,
                print_step: 1.0e-6,
                probes: &["V(HV)"],
                historical_grid_threshold: None,
            },
            Bug456Case::PushPullUnpacked => Self {
                case,
                job: "push_pull_unpacked",
                packed: false,
                stop: 1.0e-3,
                seam: 5.0e-4,
                print_step: 1.0e-6,
                probes: &["V(HV)"],
                historical_grid_threshold: None,
            },
            Bug456Case::SimpleUnpacked => Self {
                case,
                job: "restart",
                packed: false,
                stop: 1.0e-5,
                seam: 2.0e-6,
                print_step: 0.0,
                probes: &["V(1)", "V(2)", "V(3)"],
                // This is the historical wrapper's literal threshold. It is
                // above TSTOP and therefore intentionally leaves the extra
                // sequential grid loop vacuous after xyce_verify succeeds.
                historical_grid_threshold: Some(2.0e-4),
            },
        }
    }

    fn role(self, member: Bug456Member) -> Bug456Role {
        Bug456Role::new(self.case, member)
    }

    fn expected_schedule(self) -> Vec<Value> {
        let mut times = vec![0.0];
        let mut next = self.seam;
        while next <= self.stop {
            times.push(next);
            next += self.seam;
        }
        times
    }
}

#[derive(Debug)]
struct Bug456RuntimeSeal {
    retained: BTreeMap<String, Vec<u8>>,
}

impl XyceTestRunner {
    fn validate_bug456_static_provenance() -> Result<(), String> {
        let removed = HISTORICAL_REMOVED
            .iter()
            .map(|name| name.to_ascii_lowercase())
            .collect::<BTreeSet<_>>();
        let retained = RETAINED
            .iter()
            .map(|record| record.0.to_ascii_lowercase())
            .collect::<BTreeSet<_>>();
        let mut records = RETAINED
            .iter()
            .map(|(name, bytes, sha)| format!("{FAMILY_DIRECTORY}/{name}\t{bytes}\t{sha}"))
            .collect::<Vec<_>>();
        records.sort();
        let stream = records.join("\n");
        let content_bytes = RETAINED.iter().map(|record| record.1).sum::<usize>();
        let retained_malformed = RETAINED.iter().any(|record| {
            record.2.len() != 64
                || record
                    .2
                    .bytes()
                    .any(|byte| !byte.is_ascii_hexdigit() || byte.is_ascii_uppercase())
        });
        let mut dependency_records = HISTORICAL_DEPENDENCIES
            .iter()
            .map(|(path, bytes, blob, sha, b3)| {
                format!("{PRETRIM_COMMIT}\t{PRETRIM_NETLISTS_TREE}\t{UPSTREAM_COMMIT}\t{RELEASE_TAG}\t{RELEASE_TAG_OBJECT}\t{path}\t{bytes}\t{blob}\t{sha}\t{b3}")
            })
            .collect::<Vec<_>>();
        dependency_records.sort();
        let dependency_stream = dependency_records.join("\n");
        let dependency_content_bytes = HISTORICAL_DEPENDENCIES
            .iter()
            .map(|record| record.1)
            .sum::<usize>();
        let dependency_malformed = HISTORICAL_DEPENDENCIES.iter().any(|record| {
            record.2.len() != 40
                || record.3.len() != 64
                || record.4.len() != 64
                || record
                    .2
                    .bytes()
                    .chain(record.3.bytes())
                    .chain(record.4.bytes())
                    .any(|byte| !byte.is_ascii_hexdigit() || byte.is_ascii_uppercase())
        });
        let historical_metadata_malformed = HISTORICAL_FAMILY_STREAM_SHA256.len() != 64
            || HISTORICAL_FAMILY_STREAM_BLAKE3.len() != 64
            || HISTORICAL_FAMILY_STREAM_SHA256
                .bytes()
                .chain(HISTORICAL_FAMILY_STREAM_BLAKE3.bytes())
                .any(|byte| !byte.is_ascii_hexdigit() || byte.is_ascii_uppercase());
        if PRETRIM_COMMIT != UPSTREAM_EXCLUSIONS_SOURCE_COMMIT
            || PRETRIM_NETLISTS_TREE != UPSTREAM_EXCLUSIONS_SOURCE_NETLISTS_TREE
            || HISTORICAL_FAMILY_TREE.len() != 40
            || RETAINED_FAMILY_TREE.len() != 40
            || HISTORICAL_RECORD_COUNT != RETAINED.len() + HISTORICAL_REMOVED.len()
            || HISTORICAL_FAMILY_CONTENT_BYTES <= RETAINED_CONTENT_BYTES
            || HISTORICAL_FAMILY_STREAM_BYTES <= RETAINED_STREAM_BYTES
            || historical_metadata_malformed
            || removed.len() != HISTORICAL_REMOVED.len()
            || retained.len() != RETAINED.len()
            || !removed.is_disjoint(&retained)
            || content_bytes != RETAINED_CONTENT_BYTES
            || stream.len() != RETAINED_STREAM_BYTES
            || format!("{:x}", Sha256::digest(stream.as_bytes())) != RETAINED_STREAM_SHA256
            || retained_malformed
            || dependency_content_bytes != HISTORICAL_DEPENDENCY_CONTENT_BYTES
            || dependency_stream.len() != HISTORICAL_DEPENDENCY_STREAM_BYTES
            || format!("{:x}", Sha256::digest(dependency_stream.as_bytes()))
                != HISTORICAL_DEPENDENCY_STREAM_SHA256
            || blake3::hash(dependency_stream.as_bytes()).to_hex().as_str()
                != HISTORICAL_DEPENDENCY_STREAM_BLAKE3
            || dependency_malformed
        {
            let stream_sha = format!("{:x}", Sha256::digest(stream.as_bytes()));
            return Err(format!(
                "{LABEL} immutable provenance identities changed: retained={}/{content_bytes}, stream={}/{}, historical={HISTORICAL_RECORD_COUNT}",
                retained.len(),
                stream.len(),
                stream_sha
            ));
        }
        Ok(())
    }

    fn bug456_family_directory(
        &self,
        abort: &dyn AbortSignal,
    ) -> Result<contract_fs::AnchoredDirectory, String> {
        let root = contract_fs::open_root(&self.root, LABEL)?;
        let netlists = contract_fs::exact_child_directory(
            root,
            "Netlists",
            LABEL,
            MAX_DIRECTORY_ENTRIES,
            abort,
        )?;
        let certification = contract_fs::exact_child_directory(
            netlists,
            "Certification_Tests",
            LABEL,
            MAX_DIRECTORY_ENTRIES,
            abort,
        )?;
        contract_fs::exact_child_directory(
            certification,
            "BUG_456",
            LABEL,
            MAX_DIRECTORY_ENTRIES,
            abort,
        )
    }

    fn read_bug456_family(
        &self,
        abort: &dyn AbortSignal,
    ) -> Result<BTreeMap<String, Vec<u8>>, String> {
        Self::validate_bug456_static_provenance()?;
        let family = self.bug456_family_directory(abort)?;
        let names = contract_fs::member_names(
            &family,
            LABEL,
            "retained family",
            RETAINED.len() + 1,
            abort,
        )?;
        let expected_names = RETAINED
            .iter()
            .map(|record| record.0.to_string())
            .collect::<BTreeSet<_>>();
        if names.iter().cloned().collect::<BTreeSet<_>>() != expected_names
            || names.len() != RETAINED.len()
        {
            return Err(format!("{LABEL} retained family census changed: {names:?}"));
        }

        let mut retained = BTreeMap::new();
        for (name, expected_bytes, expected_sha) in RETAINED {
            if abort.is_aborted() {
                return Err(format!("{LABEL} source census aborted"));
            }
            let file = contract_fs::open_file_member(&family, name, LABEL, name)?;
            let read_bound = expected_bytes
                .checked_mul(2)
                .and_then(|value| value.checked_add(3))
                .and_then(|value| u64::try_from(value).ok())
                .ok_or_else(|| format!("{LABEL} {name} read bound overflowed"))?;
            let raw = contract_fs::read_bounded_raw(file, read_bound, LABEL, name, abort)?;
            let canonical = Self::canonical_lf_text_identity(&format!("{LABEL} {name}"), &raw)?;
            let sha = format!("{:x}", Sha256::digest(&canonical));
            if canonical.len() != expected_bytes || sha != expected_sha {
                return Err(format!(
                    "{LABEL} {name} changed: bytes={}, sha={sha}",
                    canonical.len()
                ));
            }
            std::str::from_utf8(&canonical)
                .map_err(|error| format!("{LABEL} {name} is not UTF-8: {error}"))?;
            if retained
                .insert(name.to_ascii_lowercase(), canonical)
                .is_some()
            {
                return Err(format!("{LABEL} duplicate case-folded member {name:?}"));
            }
        }
        Ok(retained)
    }

    fn read_bug456_root_manifest(
        &self,
        name: &str,
        label: &str,
        abort: &dyn AbortSignal,
    ) -> Result<String, String> {
        let root = contract_fs::open_root(&self.root, LABEL)?;
        let file =
            contract_fs::exact_child_file(&root, name, LABEL, label, MAX_DIRECTORY_ENTRIES, abort)?;
        let raw = contract_fs::read_bounded_raw(file, MAX_MANIFEST_BYTES, LABEL, label, abort)?;
        let canonical = Self::canonical_lf_text_identity(&format!("{LABEL} {label}"), &raw)?;
        String::from_utf8(canonical)
            .map_err(|error| format!("{LABEL} {label} is not UTF-8: {error}"))
    }

    fn validate_bug456_manifest_ownership(&self, abort: &dyn AbortSignal) -> Result<(), String> {
        let harness =
            self.read_bug456_root_manifest(HARNESS_MANIFEST_FILE, "harness manifest", abort)?;
        let expected_owners = Bug456Role::OWNERS
            .map(|role| format!("{}\t{REQUIRES_UPSTREAM_WRAPPER_CONTRACT}", role.path()))
            .into_iter()
            .collect::<BTreeSet<_>>();
        let actual_owners = harness
            .lines()
            .filter(|line| {
                line.replace('\\', "/")
                    .to_ascii_lowercase()
                    .contains(FAMILY_PREFIX)
            })
            .map(str::to_string)
            .collect::<BTreeSet<_>>();
        if actual_owners != expected_owners {
            return Err(format!(
                "{LABEL} owner-only harness manifest changed: {actual_owners:?}"
            ));
        }

        let exclusions = self.read_bug456_root_manifest(
            UPSTREAM_EXCLUSIONS_MANIFEST_FILE,
            "upstream exclusions manifest",
            abort,
        )?;
        let expected_workers = Bug456Role::WORKERS
            .map(|role| {
                format!(
                    "{}\t{EXCLUSION_SOURCE}\t{UPSTREAM_EXCLUDED_DISPOSITION}",
                    role.path()
                )
            })
            .into_iter()
            .collect::<BTreeSet<_>>();
        let actual_workers = exclusions
            .lines()
            .filter(|line| {
                line.replace('\\', "/")
                    .to_ascii_lowercase()
                    .contains(FAMILY_PREFIX)
            })
            .map(str::to_string)
            .collect::<BTreeSet<_>>();
        if actual_workers != expected_workers {
            return Err(format!(
                "{LABEL} worker exclusion ownership changed: {actual_workers:?}"
            ));
        }
        let parsed = Self::load_upstream_exclusions(&self.root)
            .map_err(|error| format!("{LABEL} exclusions are invalid: {error}"))?;
        for role in Bug456Role::WORKERS {
            let exclusion = parsed
                .get(&role.record())
                .ok_or_else(|| format!("{LABEL} lost {:?} exclusion", role))?;
            if exclusion.source != EXCLUSION_SOURCE
                || !matches!(
                    exclusion.disposition,
                    XyceUpstreamExclusionDisposition::Excluded
                )
            {
                return Err(format!(
                    "{LABEL} {:?} exclusion changed: {exclusion:?}",
                    role
                ));
            }
        }
        Ok(())
    }

    fn validate_bug456_provenance(
        &self,
        deck: &XyceDeck,
        role: Bug456Role,
        abort: &dyn AbortSignal,
    ) -> Result<Bug456RuntimeSeal, String> {
        if abort.is_aborted() {
            return Err(format!("{LABEL} provenance validation aborted"));
        }
        if deck.section != XyceDeckSection::Netlists
            || Self::normalize_manifest_key(&deck.relative_path) != role.record()
            || Self::normalize_manifest_key(&self.relative_key(&deck.path)) != role.record()
            || !Self::same_path(&deck.path, &self.root.join(role.path()))
        {
            return Err(format!("{LABEL} recognized {role:?} path is not canonical"));
        }
        let retained = self.read_bug456_family(abort)?;
        self.validate_bug456_manifest_ownership(abort)?;
        match fs::symlink_metadata(self.root.join(OUTPUT_DIRECTORY)) {
            Err(error) if error.kind() == std::io::ErrorKind::NotFound => {}
            Err(error) => return Err(format!("failed to inspect {LABEL} OutputData: {error}")),
            Ok(_) => return Err(format!("{LABEL} must not acquire invented numerical gold")),
        }
        for member in Bug456Role::ALL {
            self.reject_wrapper_output_artifacts(&self.root.join(member.path()))
                .map_err(|error| format!("{LABEL} {} {error}", member.file_name()))?;
        }
        Ok(Bug456RuntimeSeal { retained })
    }

    fn bug456_worker_plan(
        &self,
        role: Bug456Role,
        retained: &BTreeMap<String, Vec<u8>>,
    ) -> Result<(XyceStaticTranPlan, Netlist), String> {
        if role.is_owner() {
            return Err(format!("{LABEL} wrapper owner is not a simulation deck"));
        }
        let spec = Bug456CaseSpec::for_case(role.case);
        let source_bytes = retained
            .get(&role.file_name().to_ascii_lowercase())
            .ok_or_else(|| format!("{LABEL} lost {role:?} source"))?;
        let source = std::str::from_utf8(source_bytes)
            .map_err(|error| format!("{LABEL} {role:?} source is not UTF-8: {error}"))?;
        if Self::contains_control_block(source) {
            return Err(format!("{LABEL} does not admit simulator scripting blocks"));
        }
        Self::reject_unsupported_source_directives(source)?;
        let path = self.root.join(role.path());
        let netlist = Self::parse_xyce_netlist(source, &path)
            .map_err(|error| format!("{LABEL} {role:?} parse failed: {error}"))?;
        validate_output_symbols(&netlist)
            .map_err(|error| format!("{LABEL} {role:?} output symbols changed: {error}"))?;
        let tran = Self::single_tran_analysis(&netlist)?;
        if tran.step.to_bits() != spec.print_step.to_bits()
            || tran.stop.to_bits() != spec.stop.to_bits()
            || tran.start.is_some()
            || tran.max_step.is_some()
            || tran.uic
        {
            return Err(format!("{LABEL} {role:?} .TRAN changed: {tran:?}"));
        }
        let outputs = Self::print_output_requests(source, "TRAN")?;
        if !matches!(
            outputs.as_slice(),
            [output] if output.format.is_none()
                && output.file.is_none()
                && output.probes.len() == spec.probes.len()
                && output.probes.iter().zip(spec.probes).all(|(actual, expected)|
                    actual.eq_ignore_ascii_case(expected))
        ) {
            return Err(format!("{LABEL} {role:?} .PRINT changed: {outputs:?}"));
        }
        let restart = netlist
            .options
            .restart
            .as_ref()
            .ok_or_else(|| format!("{LABEL} {role:?} omitted typed restart options"))?;
        if restart.start_time.is_some()
            || restart.print_timeint_options.is_some()
            || !restart.intervals.is_empty()
        {
            return Err(format!(
                "{LABEL} {role:?} restart envelope changed: {restart:?}"
            ));
        }
        let restart_matches = match role.member {
            Bug456Member::Baseline => {
                restart.job.as_deref() == Some(spec.job)
                    && restart.file.is_none()
                    && restart.initial_interval.map(Value::to_bits) == Some(spec.seam.to_bits())
                    && restart.pack.unwrap_or(true) == spec.packed
            }
            Bug456Member::Restart => {
                restart.job.is_none()
                    && restart.initial_interval.is_none()
                    && restart.pack.is_none()
                    && restart.file.as_deref()
                        == XyceRestartJobPlan::new(
                            spec.job,
                            spec.seam,
                            &[],
                            spec.stop,
                            Some(spec.packed),
                            MAX_RESTART_POINTS,
                        )
                        .map_err(|error| format!("{LABEL} restart plan failed: {error}"))?
                        .logical_name(spec.seam)
                        .as_deref()
            }
            Bug456Member::Owner => false,
        };
        if !restart_matches {
            return Err(format!(
                "{LABEL} {role:?} restart options changed: {restart:?}"
            ));
        }
        let plan = XyceStaticTranPlan {
            deck_path: path,
            oracle: XyceStaticTranOracle::None,
            source: source.to_string(),
            print: Some(XycePrintRequest {
                probes: spec
                    .probes
                    .iter()
                    .map(|probe| (*probe).to_string())
                    .collect(),
            }),
            output_override: false,
            timeint_conststep: false,
            tran,
            steps: Vec::new(),
            contract: XyceStaticTranContract::PlainStatic,
            wrapper_tolerance: None,
            comparison_mode: XyceStaticTranComparisonMode::Pointwise,
        };
        Ok((plan, netlist))
    }

    fn bug456_resolved_config(
        &self,
        plan: &XyceStaticTranPlan,
        netlist: &Netlist,
    ) -> SimulationConfig {
        let mut base = self.xyce_engine_config(None);
        base.transient_initial_timestep = Self::xyce_initial_timestep_for_tran(&plan.tran);
        resolve_simulation_config(
            &base,
            Some(&netlist.options),
            &SimulationConfigOverrides::default(),
        )
    }

    fn bug456_require_result_horizon(
        label: &str,
        result: &TransientResult,
        start: Value,
        stop: Value,
    ) -> Result<(), String> {
        Self::validate_transient_result_time_grid(result)?;
        if result.time.is_empty()
            || result.time.len() > MAX_RESULT_ROWS
            || result.time.first().map(|value| value.to_bits()) != Some(start.to_bits())
            || result.time.last().map(|value| value.to_bits()) != Some(stop.to_bits())
            || result
                .voltages
                .iter()
                .chain(&result.branch_currents)
                .flat_map(|waveform| waveform.iter())
                .any(|value| !value.is_finite())
        {
            return Err(format!(
                "{LABEL} {label} result horizon/framing changed: rows={}, endpoints={:?}/{:?}",
                result.time.len(),
                result.time.first(),
                result.time.last()
            ));
        }
        Ok(())
    }

    fn bug456_round_trip_checkpoint(
        checkpoint: &TransientCheckpoint,
        encoding: TransientCheckpointEncoding,
    ) -> Result<TransientCheckpoint, String> {
        let encoded = checkpoint
            .to_bytes(encoding)
            .map_err(|error| format!("{LABEL} {encoding:?} encoding failed: {error}"))?;
        if encoded.is_empty() || encoded.len() > MAX_CHECKPOINT_BYTES {
            return Err(format!(
                "{LABEL} {encoding:?} checkpoint violated byte envelope: {}",
                encoded.len()
            ));
        }
        if encoding == TransientCheckpointEncoding::Unpacked
            && encoded.as_slice() != checkpoint.to_text().as_bytes()
        {
            return Err(format!(
                "{LABEL} unpacked checkpoint was not the canonical text encoding"
            ));
        }
        let restored =
            TransientCheckpoint::from_bytes_with_encoding(&encoded, encoding, MAX_CHECKPOINT_BYTES)
                .map_err(|error| format!("{LABEL} {encoding:?} decoding failed: {error}"))?;
        let wrong = match encoding {
            TransientCheckpointEncoding::Packed => TransientCheckpointEncoding::Unpacked,
            TransientCheckpointEncoding::Unpacked => TransientCheckpointEncoding::Packed,
        };
        if TransientCheckpoint::from_bytes_with_encoding(&encoded, wrong, MAX_CHECKPOINT_BYTES)
            .is_ok()
        {
            return Err(format!(
                "{LABEL} {encoding:?} checkpoint was accepted by the {wrong:?} decoder"
            ));
        }
        if restored != *checkpoint {
            return Err(format!("{LABEL} {encoding:?} checkpoint was not bit-exact"));
        }
        Ok(restored)
    }

    fn bug456_require_historical_grid(
        good: &XycePrnTable,
        test: &XycePrnTable,
        threshold: Value,
    ) -> Result<(), String> {
        let mut test_index = 0usize;
        for good_row in &good.rows {
            let good_time = Self::xyce_prn_scientific_roundtrip(
                *good_row
                    .get(1)
                    .ok_or_else(|| format!("{LABEL} GOOD PRN row omitted TIME"))?,
                XYCE_DEFAULT_PRN_SCIENTIFIC_PRECISION,
            )?;
            if good_time >= threshold - HISTORICAL_GRID_ZERO_TOLERANCE {
                let test_row = test.rows.get(test_index).ok_or_else(|| {
                    format!("{LABEL} historical sequential grid exhausted TEST at row {test_index}")
                })?;
                let test_time = Self::xyce_prn_scientific_roundtrip(
                    *test_row
                        .get(1)
                        .ok_or_else(|| format!("{LABEL} TEST PRN row omitted TIME"))?,
                    XYCE_DEFAULT_PRN_SCIENTIFIC_PRECISION,
                )?;
                if !(good_time >= test_time - HISTORICAL_GRID_ZERO_TOLERANCE
                    && good_time < test_time + HISTORICAL_GRID_ZERO_TOLERANCE)
                {
                    return Err(format!(
                        "{LABEL} historical sequential grid differs at TEST row {test_index}: GOOD={good_time:.17e}, TEST={test_time:.17e}"
                    ));
                }
                test_index += 1;
            }
        }
        Ok(())
    }

    fn validate_bug456_case(
        &self,
        spec: Bug456CaseSpec,
        retained: &BTreeMap<String, Vec<u8>>,
        abort: &dyn AbortSignal,
    ) -> Result<(), String> {
        let baseline_role = spec.role(Bug456Member::Baseline);
        let restart_role = spec.role(Bug456Member::Restart);
        let (baseline_plan, baseline_netlist) = self.bug456_worker_plan(baseline_role, retained)?;
        let (restart_plan, restart_netlist) = self.bug456_worker_plan(restart_role, retained)?;
        let baseline_config = self.bug456_resolved_config(&baseline_plan, &baseline_netlist);
        let restart_config = self.bug456_resolved_config(&restart_plan, &restart_netlist);
        if format!("{baseline_config:#?}") != format!("{restart_config:#?}") {
            return Err(format!(
                "{LABEL} {:?} save/resume resolved configurations differ: baseline={baseline_config:#?}, restart={restart_config:#?}",
                spec.case
            ));
        }

        let restart_options = baseline_netlist
            .options
            .restart
            .as_ref()
            .expect("typed baseline restart options were validated");
        let job_plan = XyceRestartJobPlan::new(
            restart_options
                .job
                .as_deref()
                .expect("typed baseline restart job was validated"),
            restart_options
                .initial_interval
                .expect("typed baseline restart interval was validated"),
            &restart_options.intervals,
            spec.stop,
            restart_options.pack,
            MAX_RESTART_POINTS,
        )
        .map_err(|error| format!("{LABEL} {:?} restart plan failed: {error}", spec.case))?;
        let expected_schedule = spec.expected_schedule();
        if job_plan
            .nominal_times()
            .iter()
            .copied()
            .map(Value::to_bits)
            .ne(expected_schedule.iter().copied().map(Value::to_bits))
            || job_plan.encoding()
                != if spec.packed {
                    TransientCheckpointEncoding::Packed
                } else {
                    TransientCheckpointEncoding::Unpacked
                }
        {
            return Err(format!(
                "{LABEL} {:?} restart schedule/encoding changed: {:?}/{:?}",
                spec.case,
                job_plan.nominal_times(),
                job_plan.encoding()
            ));
        }
        let restart_file = restart_netlist
            .options
            .restart
            .as_ref()
            .and_then(|restart| restart.file.as_deref())
            .ok_or_else(|| format!("{LABEL} {:?} restart deck lost FILE", spec.case))?;

        let baseline_max_step =
            Self::transient_family_max_step(&baseline_netlist, &baseline_plan.tran)?;
        let baseline_engine = Engine::new(baseline_config);
        let (baseline_result, checkpoints) = baseline_engine
            .run_tran_checkpoint_schedule_with_startup_mode_and_abort(
                &baseline_netlist,
                spec.stop,
                baseline_max_step,
                TransientStartupMode::from_uic(false),
                job_plan.nominal_times(),
                abort,
            )
            .map_err(|error| format!("{LABEL} {:?} baseline failed: {error}", spec.case))?;
        Self::bug456_require_result_horizon(
            &format!("{:?} baseline", spec.case),
            &baseline_result,
            0.0,
            spec.stop,
        )?;
        if checkpoints.len() != job_plan.nominal_times().len()
            || checkpoints
                .iter()
                .map(|scheduled| scheduled.nominal_time.to_bits())
                .ne(job_plan.nominal_times().iter().copied().map(Value::to_bits))
        {
            return Err(format!(
                "{LABEL} {:?} checkpoint schedule changed",
                spec.case
            ));
        }

        let mut seam_checkpoint = None;
        for scheduled in checkpoints {
            if abort.is_aborted() {
                return Err(format!(
                    "{LABEL} {:?} checkpoint validation aborted",
                    spec.case
                ));
            }
            let restored =
                Self::bug456_round_trip_checkpoint(&scheduled.checkpoint, job_plan.encoding())?;
            if job_plan.logical_name(scheduled.nominal_time).as_deref() == Some(restart_file)
                && seam_checkpoint.replace(restored).is_some()
            {
                return Err(format!(
                    "{LABEL} {:?} restart FILE selected multiple checkpoints",
                    spec.case
                ));
            }
        }
        let seam_checkpoint = seam_checkpoint.ok_or_else(|| {
            format!(
                "{LABEL} {:?} restart FILE {restart_file:?} did not select a scheduled checkpoint",
                spec.case
            )
        })?;
        if seam_checkpoint.time < spec.seam || seam_checkpoint.time >= spec.stop {
            return Err(format!(
                "{LABEL} {:?} seam checkpoint captured outside its accepted-step window at {:.17e}",
                spec.case, seam_checkpoint.time
            ));
        }

        let restart_max_step =
            Self::transient_family_max_step(&restart_netlist, &restart_plan.tran)?;
        if baseline_max_step.to_bits() != restart_max_step.to_bits() {
            return Err(format!(
                "{LABEL} {:?} save/resume max steps differ",
                spec.case
            ));
        }
        let restart_engine = Engine::new(restart_config);
        let (restart_result, final_checkpoint) = restart_engine
            .run_tran_restart_resume_with_abort(
                &restart_netlist,
                &seam_checkpoint,
                spec.stop,
                restart_max_step,
                abort,
            )
            .map_err(|error| format!("{LABEL} {:?} restart failed: {error}", spec.case))?;
        Self::bug456_require_result_horizon(
            &format!("{:?} restart", spec.case),
            &restart_result,
            seam_checkpoint.time,
            spec.stop,
        )?;
        if final_checkpoint.time.to_bits() != spec.stop.to_bits() {
            return Err(format!(
                "{LABEL} {:?} final checkpoint stopped at {:.17e}",
                spec.case, final_checkpoint.time
            ));
        }

        let good = Self::transient_family_result_to_prn_table(
            &baseline_plan,
            &baseline_netlist,
            &baseline_result,
        )?;
        let test = Self::transient_family_result_to_prn_table(
            &restart_plan,
            &restart_netlist,
            &restart_result,
        )?;
        if good.rows.is_empty()
            || test.rows.is_empty()
            || good.rows.len() > MAX_RESULT_ROWS
            || test.rows.len() > MAX_RESULT_ROWS
        {
            return Err(format!("{LABEL} {:?} PRN framing changed", spec.case));
        }
        let mismatches =
            self.compare_xyce_verify_transient_tables_with_abort(&good, &test, abort)?;
        if !mismatches.is_empty() {
            return Err(format!(
                "{LABEL} {:?} produced {} Release-7.10 xyce_verify mismatch(es): {mismatches:?}",
                spec.case,
                mismatches.len()
            ));
        }
        if let Some(threshold) = spec.historical_grid_threshold {
            Self::bug456_require_historical_grid(&good, &test, threshold)?;
        }
        Ok(())
    }

    pub(super) fn validate_bug456_oracle(
        &self,
        deck: &XyceDeck,
        role: Bug456Role,
        start: Instant,
    ) -> Result<(), String> {
        let abort = DeadlineAbort::new(start, self.config.max_time_per_test_ms.max(1));
        let seal = self.validate_bug456_provenance(deck, role, &abort)?;
        self.validate_bug456_case(Bug456CaseSpec::for_case(role.case), &seal.retained, &abort)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use rspice_core::abort_signal::NoAbort;

    fn corpus_root() -> PathBuf {
        Path::new(env!("CARGO_MANIFEST_DIR")).join("../../tests/xyce")
    }

    fn runner() -> XyceTestRunner {
        XyceTestRunner::new(corpus_root(), XyceRunnerConfig::default())
    }

    fn deck(root: &Path, role: Bug456Role) -> XyceDeck {
        XyceDeck {
            path: root.join(role.path()),
            section: XyceDeckSection::Netlists,
            relative_path: role.path(),
        }
    }

    #[test]
    fn bug456_role_map_is_exact_and_partitioned() {
        let all = Bug456Role::ALL.into_iter().collect::<BTreeSet<_>>();
        let owners = Bug456Role::OWNERS.into_iter().collect::<BTreeSet<_>>();
        let workers = Bug456Role::WORKERS.into_iter().collect::<BTreeSet<_>>();
        assert_eq!(all.len(), 24);
        assert_eq!(owners.len(), 8);
        assert_eq!(workers.len(), 16);
        assert!(owners.is_disjoint(&workers));
        assert_eq!(
            owners.union(&workers).copied().collect::<BTreeSet<_>>(),
            all
        );
        let records = Bug456Role::ALL
            .map(Bug456Role::record)
            .into_iter()
            .collect::<BTreeSet<_>>();
        assert_eq!(records.len(), 24);
        for role in Bug456Role::ALL {
            assert_eq!(Bug456Role::for_record(&role.path()), Some(role));
            assert_eq!(Bug456Role::for_record(&role.record()), Some(role));
        }
        assert_eq!(
            Bug456Role::for_record("Netlists/Certification_Tests/BUG_456/invented.cir"),
            None
        );
    }

    #[test]
    fn bug456_retained_family_and_current_ownership_are_sealed() {
        let runner = runner();
        let role = Bug456Role::OWNERS[0];
        let seal = runner
            .validate_bug456_provenance(&deck(&runner.root, role), role, &NoAbort)
            .expect("seal BUG456 retained family and current ownership");
        assert_eq!(seal.retained.len(), 24);
    }

    #[test]
    fn bug456_restart_plans_derive_exact_historical_schedules() {
        for owner in Bug456Role::OWNERS {
            let spec = Bug456CaseSpec::for_case(owner.case);
            let plan = XyceRestartJobPlan::new(
                spec.job,
                spec.seam,
                &[],
                spec.stop,
                Some(spec.packed),
                MAX_RESTART_POINTS,
            )
            .expect("construct bounded historical restart plan");
            assert!(
                plan.nominal_times()
                    .iter()
                    .copied()
                    .map(Value::to_bits)
                    .eq(spec.expected_schedule().into_iter().map(Value::to_bits))
            );
            assert_eq!(
                plan.logical_name(spec.seam).as_deref(),
                Some(match spec.case {
                    Bug456Case::ConverterPacked => "converter0.0002",
                    Bug456Case::ConverterUnpacked => "converter_unpacked0.0002",
                    Bug456Case::ConverterOutput => "converter_output0.0002",
                    Bug456Case::EmitterTrapGear => "emitter2e-05",
                    Bug456Case::EmitterGear => "emittergear2e-05",
                    Bug456Case::PushPullPacked => "push_pull0.0005",
                    Bug456Case::PushPullUnpacked => "push_pull_unpacked0.0005",
                    Bug456Case::SimpleUnpacked => "restart2e-06",
                })
            );
        }
    }

    #[test]
    fn bug456_historical_simple_grid_clause_remains_vacuous() {
        let columns = vec!["Index".into(), "TIME".into(), "V(1)".into()];
        let good = XycePrnTable {
            columns: columns.clone(),
            rows: vec![vec![0.0, 0.0, 1.0], vec![1.0, 1.0e-5, 1.0]],
        };
        let test = XycePrnTable {
            columns,
            rows: Vec::new(),
        };
        XyceTestRunner::bug456_require_historical_grid(&good, &test, 2.0e-4)
            .expect("the historical simple.cir threshold is intentionally above TSTOP");
    }
}
