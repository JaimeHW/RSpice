use serde::{Deserialize, Serialize};

/// Canonical commercial objects. Serialized names are the stable IDs used by
/// persistence, automation, deep links, and the executable design reference.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum ProductObjectKind {
    #[serde(rename = "project")]
    Project,
    #[serde(rename = "design")]
    Design,
    #[serde(rename = "testbench")]
    Testbench,
    #[serde(rename = "simulation-plan")]
    SimulationPlan,
    #[serde(rename = "run-set")]
    RunSet,
    #[serde(rename = "job")]
    Job,
    #[serde(rename = "run")]
    Run,
    #[serde(rename = "dataset")]
    Dataset,
    #[serde(rename = "result-document")]
    ResultDocument,
    #[serde(rename = "verification-plan")]
    VerificationPlan,
    #[serde(rename = "verification-evidence")]
    VerificationEvidence,
    #[serde(rename = "release-candidate")]
    ReleaseCandidate,
    #[serde(rename = "automation-pipeline")]
    AutomationPipeline,
    #[serde(rename = "model-binding")]
    ModelBinding,
}

impl ProductObjectKind {
    pub const ALL: [Self; 14] = [
        Self::Project,
        Self::Design,
        Self::Testbench,
        Self::SimulationPlan,
        Self::RunSet,
        Self::Job,
        Self::Run,
        Self::Dataset,
        Self::ResultDocument,
        Self::VerificationPlan,
        Self::VerificationEvidence,
        Self::ReleaseCandidate,
        Self::AutomationPipeline,
        Self::ModelBinding,
    ];

    #[must_use]
    pub const fn stable_id(self) -> &'static str {
        self.contract().stable_id
    }

    #[must_use]
    pub const fn contract(self) -> &'static ProductObjectContract {
        match self {
            Self::Project => &PROJECT,
            Self::Design => &DESIGN,
            Self::Testbench => &TESTBENCH,
            Self::SimulationPlan => &SIMULATION_PLAN,
            Self::RunSet => &RUN_SET,
            Self::Job => &JOB,
            Self::Run => &RUN,
            Self::Dataset => &DATASET,
            Self::ResultDocument => &RESULT_DOCUMENT,
            Self::VerificationPlan => &VERIFICATION_PLAN,
            Self::VerificationEvidence => &VERIFICATION_EVIDENCE,
            Self::ReleaseCandidate => &RELEASE_CANDIDATE,
            Self::AutomationPipeline => &AUTOMATION_PIPELINE,
            Self::ModelBinding => &MODEL_BINDING,
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub enum WorkflowOwner {
    ProjectLifecycle,
    DesignEntry,
    SimulationSetup,
    ExecutionManager,
    ResultsDataLifecycle,
    ResultsWorkspace,
    VerificationPlanning,
    VerificationSourceDomain,
    ReleaseClosure,
    Automation,
    ModelsAndLibraries,
}

impl WorkflowOwner {
    #[must_use]
    pub const fn label(self) -> &'static str {
        match self {
            Self::ProjectLifecycle => "project lifecycle",
            Self::DesignEntry => "design entry",
            Self::SimulationSetup => "simulation setup",
            Self::ExecutionManager => "execution manager",
            Self::ResultsDataLifecycle => "results data lifecycle",
            Self::ResultsWorkspace => "results workspace",
            Self::VerificationPlanning => "verification planning",
            Self::VerificationSourceDomain => "verification source domain",
            Self::ReleaseClosure => "release closure",
            Self::Automation => "automation",
            Self::ModelsAndLibraries => "models and libraries",
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub enum Mutability {
    Versioned,
    AppendOnlyStateMachine,
    Immutable,
    VersionedPresentation,
    AppendOnlyDisposition,
    DraftThenImmutable,
    VersionedOrchestration,
    VersionedReference,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct ProductObjectContract {
    pub kind: ProductObjectKind,
    pub stable_id: &'static str,
    pub label: &'static str,
    pub parent: Option<ProductObjectKind>,
    pub owner: WorkflowOwner,
    pub mutability: Mutability,
    pub identity: &'static str,
    pub contains: &'static [&'static str],
}

const PROJECT: ProductObjectContract = ProductObjectContract {
    kind: ProductObjectKind::Project,
    stable_id: "project",
    label: "Project",
    parent: None,
    owner: WorkflowOwner::ProjectLifecycle,
    mutability: Mutability::Versioned,
    identity: "project UUID + schema version",
    contains: &[
        "design",
        "testbench",
        "verification-plan",
        "automation-pipeline",
        "model-binding",
    ],
};

const DESIGN: ProductObjectContract = ProductObjectContract {
    kind: ProductObjectKind::Design,
    stable_id: "design",
    label: "Design",
    parent: Some(ProductObjectKind::Project),
    owner: WorkflowOwner::DesignEntry,
    mutability: Mutability::Versioned,
    identity: "library / cell / view + revision",
    contains: &["sheet", "layout-view", "source-view"],
};

const TESTBENCH: ProductObjectContract = ProductObjectContract {
    kind: ProductObjectKind::Testbench,
    stable_id: "testbench",
    label: "Testbench",
    parent: Some(ProductObjectKind::Project),
    owner: WorkflowOwner::SimulationSetup,
    mutability: Mutability::Versioned,
    identity: "testbench UUID + revision",
    contains: &["simulation-plan"],
};

const SIMULATION_PLAN: ProductObjectContract = ProductObjectContract {
    kind: ProductObjectKind::SimulationPlan,
    stable_id: "simulation-plan",
    label: "Simulation plan",
    parent: Some(ProductObjectKind::Testbench),
    owner: WorkflowOwner::SimulationSetup,
    mutability: Mutability::Versioned,
    identity: "plan UUID + revision",
    contains: &[
        "analysis",
        "variable",
        "output",
        "measurement",
        "run-set",
        "model-binding",
        "solver-policy",
        "save-policy",
    ],
};

const RUN_SET: ProductObjectContract = ProductObjectContract {
    kind: ProductObjectKind::RunSet,
    stable_id: "run-set",
    label: "Run set",
    parent: Some(ProductObjectKind::SimulationPlan),
    owner: WorkflowOwner::SimulationSetup,
    mutability: Mutability::Versioned,
    identity: "resolved dimension graph digest",
    contains: &["job"],
};

const JOB: ProductObjectContract = ProductObjectContract {
    kind: ProductObjectKind::Job,
    stable_id: "job",
    label: "Job",
    parent: Some(ProductObjectKind::RunSet),
    owner: WorkflowOwner::ExecutionManager,
    mutability: Mutability::AppendOnlyStateMachine,
    identity: "job UUID + resolved input digest",
    contains: &["run"],
};

const RUN: ProductObjectContract = ProductObjectContract {
    kind: ProductObjectKind::Run,
    stable_id: "run",
    label: "Run",
    parent: Some(ProductObjectKind::Job),
    owner: WorkflowOwner::ExecutionManager,
    mutability: Mutability::AppendOnlyStateMachine,
    identity: "run UUID + attempt number",
    contains: &["dataset"],
};

const DATASET: ProductObjectContract = ProductObjectContract {
    kind: ProductObjectKind::Dataset,
    stable_id: "dataset",
    label: "Dataset",
    parent: Some(ProductObjectKind::Run),
    owner: WorkflowOwner::ResultsDataLifecycle,
    mutability: Mutability::Immutable,
    identity: "content digest + run manifest",
    contains: &["result-document", "verification-evidence"],
};

const RESULT_DOCUMENT: ProductObjectContract = ProductObjectContract {
    kind: ProductObjectKind::ResultDocument,
    stable_id: "result-document",
    label: "Result document",
    parent: Some(ProductObjectKind::Dataset),
    owner: WorkflowOwner::ResultsWorkspace,
    mutability: Mutability::VersionedPresentation,
    identity: "document UUID + dataset bindings",
    contains: &["viewer", "trace", "measurement-view", "annotation"],
};

const VERIFICATION_PLAN: ProductObjectContract = ProductObjectContract {
    kind: ProductObjectKind::VerificationPlan,
    stable_id: "verification-plan",
    label: "Verification run plan",
    parent: Some(ProductObjectKind::Project),
    owner: WorkflowOwner::VerificationPlanning,
    mutability: Mutability::Versioned,
    identity: "plan UUID + requirement revision",
    contains: &["testbench-reference", "stage", "coverage-rule"],
};

const VERIFICATION_EVIDENCE: ProductObjectContract = ProductObjectContract {
    kind: ProductObjectKind::VerificationEvidence,
    stable_id: "verification-evidence",
    label: "Verification evidence",
    parent: Some(ProductObjectKind::Dataset),
    owner: WorkflowOwner::VerificationSourceDomain,
    mutability: Mutability::AppendOnlyDisposition,
    identity: "evidence UUID + source digest",
    contains: &["requirement-result", "review", "approval"],
};

const RELEASE_CANDIDATE: ProductObjectContract = ProductObjectContract {
    kind: ProductObjectKind::ReleaseCandidate,
    stable_id: "release-candidate",
    label: "Release candidate",
    parent: Some(ProductObjectKind::Project),
    owner: WorkflowOwner::ReleaseClosure,
    mutability: Mutability::DraftThenImmutable,
    identity: "candidate UUID + frozen scope digest",
    contains: &[
        "evidence-reference",
        "report-artifact",
        "approval",
        "distribution-package",
    ],
};

const AUTOMATION_PIPELINE: ProductObjectContract = ProductObjectContract {
    kind: ProductObjectKind::AutomationPipeline,
    stable_id: "automation-pipeline",
    label: "Automation pipeline",
    parent: Some(ProductObjectKind::Project),
    owner: WorkflowOwner::Automation,
    mutability: Mutability::VersionedOrchestration,
    identity: "pipeline UUID + revision",
    contains: &[
        "command-reference",
        "dependency",
        "trigger",
        "artifact-route",
    ],
};

const MODEL_BINDING: ProductObjectContract = ProductObjectContract {
    kind: ProductObjectKind::ModelBinding,
    stable_id: "model-binding",
    label: "Model binding",
    parent: Some(ProductObjectKind::Project),
    owner: WorkflowOwner::ModelsAndLibraries,
    mutability: Mutability::VersionedReference,
    identity: "model/library digest + qualified platform set",
    contains: &[
        "model-source",
        "qualification-evidence",
        "protected-ip-policy",
    ],
};

/// Commercial availability and integration boundary. Only `Production` can
/// contribute native RSpice sign-off evidence; qualified external evidence
/// keeps its producer identity and is never relabeled as native.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub enum Availability {
    Production,
    Preview,
    Compatibility,
    ExternalFirst,
    Connector,
    Roadmap,
}

impl Availability {
    #[must_use]
    pub const fn can_contribute_native_signoff(self) -> bool {
        matches!(self, Self::Production)
    }
}

/// Evidence-backed readiness vocabulary. Ordering is intentional: a later
/// stage may be claimed only when every earlier stage is proven for the exact
/// capability and candidate.
#[repr(u8)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub enum ReadinessStage {
    Registered,
    Specified,
    Interactive,
    Tested,
    EngineBacked,
    PlatformQualified,
    SignOffEligible,
}

impl ReadinessStage {
    pub const ALL: [Self; 7] = [
        Self::Registered,
        Self::Specified,
        Self::Interactive,
        Self::Tested,
        Self::EngineBacked,
        Self::PlatformQualified,
        Self::SignOffEligible,
    ];

    #[must_use]
    pub const fn requires(self, prerequisite: Self) -> bool {
        self as u8 >= prerequisite as u8
    }
}

#[cfg(test)]
mod tests {
    use std::collections::HashSet;

    use super::*;

    #[test]
    fn product_object_catalog_is_unique_and_rooted() {
        let ids: HashSet<_> = ProductObjectKind::ALL
            .iter()
            .map(|kind| kind.stable_id())
            .collect();
        assert_eq!(ids.len(), ProductObjectKind::ALL.len());
        assert_eq!(ProductObjectKind::ALL.len(), 14);

        for kind in ProductObjectKind::ALL {
            let contract = kind.contract();
            assert_eq!(contract.kind, kind);
            assert!(!contract.label.is_empty());
            assert!(!contract.identity.is_empty());
            let mut cursor = contract.parent;
            let mut depth = 0;
            while let Some(parent) = cursor {
                depth += 1;
                assert!(
                    depth <= ProductObjectKind::ALL.len(),
                    "parent cycle at {kind:?}"
                );
                cursor = parent.contract().parent;
            }
        }
        assert_eq!(ProductObjectKind::Project.contract().parent, None);
    }

    #[test]
    fn only_production_capabilities_are_native_signoff_eligible() {
        assert!(Availability::Production.can_contribute_native_signoff());
        for availability in [
            Availability::Preview,
            Availability::Compatibility,
            Availability::ExternalFirst,
            Availability::Connector,
            Availability::Roadmap,
        ] {
            assert!(!availability.can_contribute_native_signoff());
        }
    }

    #[test]
    fn readiness_stages_preserve_the_seven_gate_order() {
        assert_eq!(ReadinessStage::ALL.len(), 7);
        assert!(ReadinessStage::SignOffEligible.requires(ReadinessStage::PlatformQualified));
        assert!(!ReadinessStage::Tested.requires(ReadinessStage::EngineBacked));
    }
}
