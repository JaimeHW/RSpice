//! The drafts the Simulation Studio's workflow dialogs edit.
//!
//! `state.rs` is the shell's persisted and navigational record: routes, dock
//! geometry, which page each workspace is showing, which document is open.
//! These are the opposite kind of thing, and the module they came from already
//! said so — "editor drafts only... deliberately excluded from session
//! persistence so an interrupted or cancelled workflow can never become
//! authoritative project configuration after restart".
//!
//! That is the seam: everything here is one surface's uncommitted transaction,
//! discarded on cancel and on restart, and nothing here is read by the
//! navigation, layout or document-registry state it used to sit beside. Each
//! draft names only `crate::product` identities and `crate::simulation`
//! vocabulary, so the file sits at the same rank its parent does and adds no
//! edge of its own.
//!
//! The plan manager's routes are redesigned independently and each keeps its
//! committed inputs in a named group, so two routes growing at once merge as
//! two disjoint types rather than colliding on one field list. Splitting them
//! out of a 2500-line module extends that separation rather than starting it.

/// Transactional workflow currently owned by the Simulation Studio surface.
///
/// These are editor drafts only. They are deliberately excluded from session
/// persistence so an interrupted or cancelled workflow can never become
/// authoritative project configuration after restart.
#[derive(Debug, Clone)]
pub enum SimulationWorkflowDialog {
    PlanManager(SimulationPlanManagerDraft),
    ClonePlan(ClonePlanDraft),
    DesignVariable(DesignVariableDraft),
    SavedOutput(SavedOutputDraft),
    RenameAnalysis(RenameAnalysisDraft),
    AnalysisRunPoints(AnalysisRunPointsDraft),
}

/// The one analysis being renamed, and the wording proposed for it.
///
/// `subject` is the kind and identity of the analysis as they read at the
/// moment the dialog opened. The dialog states which analysis it is about
/// without holding a borrow on the plan, and the plan remains the only thing
/// that decides whether the proposed name is acceptable.
#[derive(Debug, Clone)]
pub struct RenameAnalysisDraft {
    pub instance_id: crate::product::AnalysisInstanceId,
    pub subject: String,
    pub name: String,
    pub validation_error: Option<String>,
}

impl RenameAnalysisDraft {
    /// Open on the name the analysis is currently shown as.
    ///
    /// An unnamed analysis is shown as its kind label, so the field opens with
    /// that label rather than empty: renaming starts from what is on screen,
    /// and an engineer who wanted "Startup transient" can edit rather than
    /// retype.
    pub fn for_instance(
        instance_id: crate::product::AnalysisInstanceId,
        subject: impl Into<String>,
        shown_as: impl Into<String>,
    ) -> Self {
        Self {
            instance_id,
            subject: subject.into(),
            name: shown_as.into(),
            validation_error: None,
        }
    }
}

/// Which resolved run-set points one analysis instance is being scoped to.
///
/// The draft holds point *identities* rather than positions or labels: the
/// dialog is open across frames, the run set is editable on another page, and a
/// position that shifted underneath would apply the selection to different
/// points than the ones that were ticked.
///
/// Nothing here is a copy of the run set. The picker renders the resolved point
/// table live and the draft records only what was chosen from it, so a point
/// that stops existing while the dialog is open stops being offered rather than
/// being committed from a stale list.
#[derive(Debug, Clone)]
pub struct AnalysisRunPointsDraft {
    pub instance: crate::product::AnalysisInstanceId,
    /// Chosen point identities, in the declared order they were offered in.
    pub selected: Vec<String>,
    pub validation_error: Option<String>,
}

impl AnalysisRunPointsDraft {
    /// Open the picker on one instance, pre-ticked with the points it visits.
    ///
    /// An instance that runs everywhere opens with every point ticked, so
    /// narrowing is a subtraction from what the plan does today rather than a
    /// list the operator has to rebuild.
    #[must_use]
    pub fn new(instance: crate::product::AnalysisInstanceId, selected: Vec<String>) -> Self {
        Self {
            instance,
            selected,
            validation_error: None,
        }
    }
}

#[derive(Debug, Clone)]
pub struct ClonePlanDraft {
    pub source_plan_id: crate::product::SimulationPlanId,
    pub name: String,
    pub copy_analyses_options: bool,
    pub copy_variables_outputs_specs: bool,
    pub copy_pvt_model_bindings: bool,
    pub copy_regression_baseline: bool,
    pub validation_error: Option<String>,
}

impl ClonePlanDraft {
    pub fn for_source(source_plan_id: crate::product::SimulationPlanId, source_name: &str) -> Self {
        Self {
            source_plan_id,
            name: format!("{source_name} · variant"),
            copy_analyses_options: true,
            copy_variables_outputs_specs: true,
            copy_pvt_model_bindings: true,
            copy_regression_baseline: false,
            validation_error: None,
        }
    }
}

#[derive(Debug, Clone)]
pub struct DesignVariableDraft {
    pub name: String,
    pub expression: String,
    pub quantity: usize,
    pub scope: usize,
    pub description: String,
    pub allowed_range: String,
    pub sweep_eligibility: usize,
    pub override_policy: usize,
    pub validation_error: Option<String>,
}

impl Default for DesignVariableDraft {
    fn default() -> Self {
        Self {
            name: "RLOAD_TEST".to_owned(),
            expression: "10 kohm".to_owned(),
            quantity: 0,
            scope: 0,
            description: "Verification load used by characterization plans".to_owned(),
            allowed_range: "1 kohm … 1 Mohm".to_owned(),
            sweep_eligibility: 0,
            override_policy: 0,
            validation_error: None,
        }
    }
}

#[derive(Debug, Clone)]
pub struct SavedOutputDraft {
    pub kind: usize,
    pub name: String,
    pub expression: String,
    pub compatible_analyses: usize,
    pub save_policy: usize,
    pub precision: usize,
    pub streaming: usize,
    pub validation_error: Option<String>,
}

impl Default for SavedOutputDraft {
    fn default() -> Self {
        Self {
            kind: 0,
            // Both name a signal in the user's own design. A new draft opens
            // empty rather than pre-filled with a node nothing here has seen.
            name: String::new(),
            expression: String::new(),
            compatible_analyses: 0,
            save_policy: 0,
            precision: 0,
            streaming: 0,
            validation_error: None,
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum SimulationPlanManagerMode {
    Browse,
    Create,
    Rename,
    Compare,
    Export,
    Import,
    Campaign,
    ConfirmArchive,
}

/// Which slice of the plan catalog the manager's table lists.
///
/// The variants are slices the catalog can actually produce, so a scope cannot
/// name a set that is empty by construction. This was a `usize` matched as
/// `1 => working, 2 => archived, _ => all`, which made every value outside that
/// range silently mean "all plans" — including a stale index left behind by a
/// control whose option list had changed.
///
/// [`Self::Working`] and [`Self::Archived`] partition the catalog: every plan is
/// in exactly one, so no plan is unreachable and no plan is listed twice.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum SimulationPlanScope {
    #[default]
    All,
    /// Not archived: the active plan and every retained one. The ordinary
    /// working view of the catalog.
    Working,
    Archived,
}

/// What a newly created plan owns from the moment it becomes active. The
/// manager's Create route is its only owner.
///
/// Every field names a configuration domain a stored plan itself owns — its
/// reference PVT point, its model closure, its solver options, its save policy.
/// Creating a plan can therefore state that configuration, instead of minting it
/// at defaults and leaving all four to be found and changed afterwards.
///
/// Three of the four are stated as inheritance rather than as a value, and one of
/// those three is not a free choice. `SimulationSavePolicy` is owned by
/// `app_state`, which this module sits far below and may not name; a policy field
/// here would either invert the layer order or stand up a second declaration of
/// the same five settings. Inheriting is the strongest form the input can take
/// from this rank, and it is a real one — the alternative to the active plan's
/// retention is the default retention, not an absent policy.
///
/// Inheritance also has to be a flag rather than a copied value, because the plan
/// being inherited from is whichever plan is active when the transaction commits,
/// and that need not be the plan that was active when this draft was built.
///
/// Every default is the corresponding type's own: `ReferencePoint`'s nominal
/// corner and temperature, and no inheritance of any of the three. That is
/// exactly the configuration a fresh root plan has always been given, so the
/// route's defaults and the catalog transaction agree rather than quietly
/// differing. Nothing here invents a corner, a temperature or a limit: this draft
/// is built knowing one plan's identity and name, which is no basis for any of
/// them.
#[derive(Debug, Clone, Default)]
pub struct NewSimulationPlanDraft {
    /// The corner and temperature the new plan resolves an undeclared run-set
    /// axis to, and the temperature its solver options carry. This is the type
    /// `ReferencePvtPoint` aliases, named at its own owner rather than through
    /// the alias, which lives above this module.
    pub reference_pvt: crate::simulation::run_set::ReferencePoint,
    /// Whether the new plan opens with the active plan's ordered model closure
    /// rather than an explicit empty one.
    pub inherit_model_closure: bool,
    /// Whether the new plan opens with the active plan's solver options rather
    /// than the engine's defaults.
    pub inherit_solver_options: bool,
    /// Whether the new plan opens with the active plan's result retention, live
    /// delivery, and failure-diagnostic policy rather than the default one.
    pub inherit_save_policy: bool,
}

/// The two plans the manager's Compare route diffs. That route is its only
/// owner.
///
/// Both sides are chosen. Comparing two plans that are neither of them open is
/// the case the route exists for, and the surface could not express it while one
/// side was fixed to whichever plan happened to be active.
///
/// `None` means "this side has not been picked", which is the state the manager
/// opens in: it is opened on a single plan, and a comparison needs two, so there
/// is no honest pair to seed here. The route resolves an unpicked side to the
/// plan it would have compared anyway — the active plan on the base, the
/// selected row on the target — so an unpicked pair states a comparison rather
/// than an empty surface, and picking a side narrows that pair instead of
/// replacing it.
#[derive(Debug, Clone, Copy, Default)]
pub struct SimulationPlanComparison {
    pub base_plan_id: Option<crate::product::SimulationPlanId>,
    pub target_plan_id: Option<crate::product::SimulationPlanId>,
}

/// The campaign the manager's Campaign route queues. That route is its only
/// owner.
///
/// A campaign is one authenticated run per member plan, dispatched in the order
/// declared here — so the members are an ordered list, and the name belongs to
/// the campaign rather than to any plan in it.
#[derive(Debug, Clone)]
pub struct SimulationCampaignDraft {
    pub name: String,
    pub member_ids: Vec<crate::product::SimulationPlanId>,
}

impl Default for SimulationCampaignDraft {
    fn default() -> Self {
        Self {
            name: "Simulation campaign".to_owned(),
            member_ids: Vec::new(),
        }
    }
}

/// Runtime-only state of the versioned Simulation Plan Manager.
///
/// The fields above the groups are the shell's: the selected row, the filter and
/// scope its table applies, the mode it dispatches on, and the one name and one
/// exchange payload that several routes share. Each route's own committed inputs
/// live in a named group instead of flat here.
///
/// That grouping is load-bearing, not tidiness. The routes are redesigned
/// independently, and this type lives in a file other work also edits — so a
/// route needing one more input extends its own group, and two routes doing that
/// at once merge as two disjoint types rather than colliding on one field list.
#[derive(Debug, Clone)]
pub struct SimulationPlanManagerDraft {
    pub selected_plan_id: crate::product::SimulationPlanId,
    pub filter: String,
    pub scope: SimulationPlanScope,
    pub mode: SimulationPlanManagerMode,
    pub name: String,
    pub exchange_text: String,
    pub new_plan: NewSimulationPlanDraft,
    pub comparison: SimulationPlanComparison,
    pub campaign: SimulationCampaignDraft,
    pub validation_error: Option<String>,
}

impl SimulationPlanManagerDraft {
    pub fn new(
        selected_plan_id: crate::product::SimulationPlanId,
        selected_name: impl Into<String>,
    ) -> Self {
        Self {
            selected_plan_id,
            filter: String::new(),
            scope: SimulationPlanScope::All,
            mode: SimulationPlanManagerMode::Browse,
            name: selected_name.into(),
            exchange_text: String::new(),
            new_plan: NewSimulationPlanDraft::default(),
            comparison: SimulationPlanComparison::default(),
            campaign: SimulationCampaignDraft::default(),
            validation_error: None,
        }
    }
}
