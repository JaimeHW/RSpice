//! The simulation-preflight report and the destinations its findings name.
//!
//! This is pure data that run preparation produces and that run, model,
//! framework and session code read. It names no presentation type, so it
//! sits in the shared model rather than in any one reader.

use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Default, Serialize, Deserialize)]
pub enum ModelsPage {
    #[default]
    #[serde(alias = "Catalog")]
    Models,
    #[serde(alias = "Libraries")]
    Symbols,
    #[serde(alias = "Pdk")]
    Corners,
    Bins,
    #[serde(alias = "Behavioral")]
    Include,
    Qualification,
}

impl ModelsPage {
    pub const ALL: [Self; 6] = [
        Self::Models,
        Self::Symbols,
        Self::Corners,
        Self::Bins,
        Self::Include,
        Self::Qualification,
    ];

    pub const fn label(self) -> &'static str {
        match self {
            Self::Models => "Models",
            Self::Symbols => "Symbols & CDF",
            Self::Corners => "Corners & sections",
            Self::Bins => "Bins & geometry",
            Self::Include => "Include graph",
            Self::Qualification => "Qualification",
        }
    }
}

/// Destination that can resolve a blocking simulation-preflight finding.
/// The dialog stores semantic destinations instead of callbacks so a report
/// remains deterministic for the exact project revision that produced it.
///
/// A destination carries the identity of the object it lands on rather than
/// leaving the reader to find it again. The identity is threaded from the
/// same values the finding's prose was built from — never parsed back out of
/// that prose, which would make the sentence a load-bearing format.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum PreflightRemediation {
    DesignChecks,
    /// The Analyses page, open on the analysis the finding is about.
    SimulationPlan {
        /// The plan instance the finding names, taken from the instance the
        /// check was reading. `None` for a finding about the plan as a whole
        /// — no analysis enabled, a plan that will not load, a stage of run
        /// preparation that names no analysis — and the page then opens as
        /// the reader left it.
        analysis: Option<crate::product::AnalysisInstanceId>,
    },
    /// The Solver & convergence page, where the plan's engine options are
    /// set: a finding whose remedy is one of them and that no drawn object
    /// owns, such as a design's connect rules with no block named.
    SolverOptions,
    ProjectTechnology,
    /// One page of the Models workspace, and the object it opens on. A
    /// model-binding finding is repaired where the bindings live — Corners &
    /// sections — and not by re-running the source checks that reported it.
    ///
    /// `library` is `None` for a finding that names no single library; the
    /// page then opens on whatever it already had selected, which is the
    /// honest answer when the finding is about the closure rather than about
    /// one of its members.
    Models {
        page: ModelsPage,
        library: Option<String>,
        /// The corner inside `library` the finding is about, when it is about
        /// one. Never `Some` without a `library` — a corner name is only
        /// unique within its library.
        corner: Option<String>,
    },
    /// The netlist source the run's executable deck is generated from.
    ///
    /// A netlist-stage preparation failure is a defect in the deck, so it is
    /// repaired in the Netlist workspace. It used to route to the design
    /// checks, which re-ran the checks that had already passed and then
    /// opened the Verify workspace — never the offending source.
    NetlistSource {
        /// The 1-based deck line the failure named, where it named one.
        ///
        /// Threaded from the parser's own report, never read back out of the
        /// finding's prose. `None` means the deck opens without the cursor
        /// being moved — the finding knew the document but not the line, and
        /// landing on an arbitrary one would be worse than landing on none.
        line: Option<usize>,
    },
    /// The project's design rule setup, where the deck a finding is about is
    /// edited.
    RuleSetup,
    /// One object drawn in the configured design, in the cell view that draws
    /// it.
    ///
    /// An engine refusal names deck nodes and elements. They are read back to
    /// the drawing through the hierarchy the deck was written from, so the
    /// finding opens the owning cell view — a sub-cell's, for a node inside
    /// an instance — and selects what the engine named there.
    Drawing {
        cell_view: crate::state::CellViewRef,
        object: DrawnObject,
    },
}

/// An object on one sheet, named the way the sheet names it.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum DrawnObject {
    Component {
        id: u64,
        name: String,
    },
    /// A net, selected through one wire it runs on, or through a component
    /// on it when it is drawn pin to pin.
    Net {
        name: String,
        wire: Option<u64>,
        component: Option<u64>,
    },
}

impl DrawnObject {
    /// The name the drawing gives it.
    pub fn name(&self) -> &str {
        match self {
            Self::Component { name, .. } | Self::Net { name, .. } => name,
        }
    }
}

impl PreflightRemediation {
    /// The Models destination for a finding that names no single library.
    pub const fn models_page(page: ModelsPage) -> Self {
        Self::Models {
            page,
            library: None,
            corner: None,
        }
    }

    /// The Corners & sections destination for a finding about one library,
    /// and optionally one corner inside it.
    pub fn model_corner(library: impl Into<String>, corner: Option<String>) -> Self {
        Self::Models {
            page: ModelsPage::Corners,
            library: Some(library.into()),
            corner,
        }
    }
}

/// One non-blocking finding in a simulation-preflight report.
///
/// An advisory is something the run will proceed *through*, so its whole value
/// is that a reader can weigh it — and weighing it usually means going and
/// looking at the thing it names. It carried only prose for a release, which
/// left every advisory ending in an instruction to navigate somewhere the
/// report could have offered to open.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct PreflightAdvisory {
    pub message: String,
    /// Where this is repaired, when repairing it has a home. `None` for an
    /// advisory that states a fact rather than naming a thing to change.
    pub remediation: Option<PreflightRemediation>,
    /// What it is about: the kind of fact it states and the object it states
    /// it of. Two advisories of one pass are two findings by this, whatever
    /// order the pass reported them in, so a pass that reports them in
    /// another order keeps each one's identity.
    pub about: (&'static str, String),
}

impl PreflightAdvisory {
    /// An advisory of `kind` about `object`, with no destination of its own.
    pub fn about(kind: &'static str, object: impl Into<String>, message: String) -> Self {
        Self {
            message,
            remediation: None,
            about: (kind, object.into()),
        }
    }
}

/// An advisory stated as prose alone, by a check that names no object: the
/// sentence is all that tells it apart.
impl From<String> for PreflightAdvisory {
    fn from(message: String) -> Self {
        Self {
            about: ("statement", message.clone()),
            message,
            remediation: None,
        }
    }
}

/// One ordered, actionable finding in a simulation-preflight report.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct PreflightIssue {
    pub check: String,
    pub observed: String,
    pub required: String,
    pub remediation: PreflightRemediation,
}

/// Immutable snapshot the preflight pass retains and a run is queued against.
/// Its findings are published to the Problems page. It is intentionally
/// runtime-only: a saved project persists the simulation plan, while
/// validation evidence must be regenerated for the live revision.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct PreflightReport {
    pub project_revision: u64,
    /// Exact configured library/cell/view root whose topology was inspected.
    pub topology_root: String,
    pub topology_revision: u64,
    /// Canonical configured hierarchy closure, keyed by LCV identity. This
    /// catches edits to referenced child schematics without coupling evidence
    /// to whichever unrelated document happens to be active in the editor.
    pub topology_closure: Vec<(String, u64)>,
    /// Stable identity and exact revision of the active simulation plan that
    /// was inspected. `None` is retained only for a blocked report produced
    /// while a legacy or otherwise unavailable plan is being diagnosed.
    pub simulation_plan_id: Option<crate::product::SimulationPlanId>,
    pub simulation_plan_revision: Option<crate::product::ObjectRevision>,
    pub blockers: Vec<PreflightIssue>,
    pub advisories: Vec<PreflightAdvisory>,
    /// Present only when the controller retained a real authorized immutable
    /// execution snapshot. Blocked reports never fabricate contract fields.
    pub prepared: Option<PreparedPreflightContract>,
}

impl PreflightReport {
    /// A retained report with no findings and no authorization, for the tests
    /// that only need preflight evidence to exist so they can watch a plan
    /// mutation drop it.
    #[cfg(test)]
    pub fn retained_for_test(project_revision: u64) -> Self {
        Self {
            project_revision,
            topology_root: String::new(),
            topology_revision: 0,
            topology_closure: Vec::new(),
            simulation_plan_id: None,
            simulation_plan_revision: None,
            blockers: Vec::new(),
            advisories: Vec::new(),
            prepared: None,
        }
    }

    /// Whether the immutable report contains a complete authorization. Call
    /// `is_runnable_for` at every live UI or dispatch boundary so currentness
    /// is evaluated as well.
    pub fn is_runnable(&self) -> bool {
        self.blockers.is_empty() && self.prepared.is_some()
    }

    pub fn is_current_for(
        &self,
        project_revision: u64,
        topology_root: &str,
        topology_revision: u64,
        topology_closure: &[(String, u64)],
        simulation_plan: Option<(
            crate::product::SimulationPlanId,
            crate::product::ObjectRevision,
        )>,
    ) -> bool {
        let plan_is_current = match (
            self.simulation_plan_id,
            self.simulation_plan_revision,
            simulation_plan,
        ) {
            (Some(report_id), Some(report_revision), Some((live_id, live_revision))) => {
                report_id == live_id && report_revision == live_revision
            }
            (None, None, None) => true,
            _ => false,
        };
        self.project_revision == project_revision
            && self.topology_root.eq_ignore_ascii_case(topology_root)
            && self.topology_revision == topology_revision
            && self.topology_closure == topology_closure
            && plan_is_current
    }

    pub fn is_runnable_for(
        &self,
        project_revision: u64,
        topology_root: &str,
        topology_revision: u64,
        topology_closure: &[(String, u64)],
        simulation_plan: Option<(
            crate::product::SimulationPlanId,
            crate::product::ObjectRevision,
        )>,
    ) -> bool {
        self.is_runnable()
            && self.is_current_for(
                project_revision,
                topology_root,
                topology_revision,
                topology_closure,
                simulation_plan,
            )
    }
}

/// Display-safe copy of the authoritative prepared snapshot metadata.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct PreparedPreflightContract {
    pub snapshot_digest: crate::product::ContentDigest,
    pub source_digest: crate::product::ContentDigest,
    pub receipt_digest: crate::product::ContentDigest,
    pub receipt_label: &'static str,
    pub analysis_ids: Vec<crate::product::ContentDigest>,
    pub task_count: usize,
    pub saved_output_contract_count: usize,
    pub pvt_point_count: usize,
    pub target: &'static str,
    pub save_policy: &'static str,
    pub model_identity_count: usize,
}
