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
    DesignVariableImport(DesignVariableImportDraft),
}

/// Why a spec-sheet import, or one row of it, cannot be adopted.
///
/// Every variant carries an id. A refusal that is only a sentence can be read
/// and nothing else: it cannot be counted, grouped, matched in a test, or
/// looked up by an engineer who has met it before, and rewording it silently
/// changes whatever was matching on it. The import used to refuse in prose
/// alone, so a sheet with forty rows and three different problems produced one
/// sentence about whichever row failed first.
///
/// The set is deliberately small — five kinds of thing a sheet gets wrong, not
/// one variant per message. A finer split would have to be kept in step with
/// the wording, which is the coupling these exist to remove.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum VariableImportRefusal {
    /// The sheet is not a table this import can read: a column it does not
    /// name, a column named twice, a required column missing, malformed CSV,
    /// or more rows or bytes than one import may carry.
    Schema { line: u64, detail: String },
    /// A row names a variable that cannot be a parameter identifier.
    Identifier {
        line: u64,
        subject: String,
        detail: String,
    },
    /// A value does not read as the quantity its row declares.
    Dimension {
        line: u64,
        subject: String,
        detail: String,
    },
    /// A row's bounds are half-written, inconsistent, or exclude its own value.
    Bounds {
        line: u64,
        subject: String,
        detail: String,
    },
    /// A name the plan already owns, or one this sheet uses twice.
    Collision {
        line: u64,
        subject: String,
        detail: String,
    },
}

impl VariableImportRefusal {
    /// The stable identity of this kind of refusal.
    ///
    /// These are contract, not decoration: they are asserted by name in tests
    /// and are what an engineer searches for. The spellings do not change with
    /// the wording beside them.
    #[must_use]
    pub const fn id(&self) -> &'static str {
        match self {
            Self::Schema { .. } => "VARIMP-SCHEMA",
            Self::Identifier { .. } => "VARIMP-IDENTIFIER",
            Self::Dimension { .. } => "VARIMP-DIMENSION",
            Self::Bounds { .. } => "VARIMP-BOUNDS",
            Self::Collision { .. } => "VARIMP-COLLISION",
        }
    }

    /// The file's own line, counted the way an editor counts it.
    #[must_use]
    pub const fn line(&self) -> u64 {
        match self {
            Self::Schema { line, .. }
            | Self::Identifier { line, .. }
            | Self::Dimension { line, .. }
            | Self::Bounds { line, .. }
            | Self::Collision { line, .. } => *line,
        }
    }

    /// The variable this refusal is about, where it is about one. A schema
    /// refusal is about the sheet, so it names nothing.
    #[must_use]
    pub fn subject(&self) -> Option<&str> {
        match self {
            Self::Schema { .. } => None,
            Self::Identifier { subject, .. }
            | Self::Dimension { subject, .. }
            | Self::Bounds { subject, .. }
            | Self::Collision { subject, .. } => Some(subject),
        }
    }

    /// The sentence saying how, without the identity in front of it.
    #[must_use]
    pub fn detail(&self) -> &str {
        match self {
            Self::Schema { detail, .. }
            | Self::Identifier { detail, .. }
            | Self::Dimension { detail, .. }
            | Self::Bounds { detail, .. }
            | Self::Collision { detail, .. } => detail,
        }
    }

    /// The same refusal, reported against `line`.
    ///
    /// A row is checked without reference to where it came from — the checks
    /// are about the values, not the file — so the line is bound afterwards by
    /// the reader that knows it. Without this the caller would have to rebuild
    /// each variant to change one field.
    #[must_use]
    pub fn at_line(self, line: u64) -> Self {
        match self {
            Self::Schema { detail, .. } => Self::Schema { line, detail },
            Self::Identifier {
                subject, detail, ..
            } => Self::Identifier {
                line,
                subject,
                detail,
            },
            Self::Dimension {
                subject, detail, ..
            } => Self::Dimension {
                line,
                subject,
                detail,
            },
            Self::Bounds {
                subject, detail, ..
            } => Self::Bounds {
                line,
                subject,
                detail,
            },
            Self::Collision {
                subject, detail, ..
            } => Self::Collision {
                line,
                subject,
                detail,
            },
        }
    }

    /// The refusal as one reported line: identity, where, what, and how.
    #[must_use]
    pub fn message(&self) -> String {
        match self.subject() {
            Some(subject) => format!(
                "{} \u{00b7} line {} \u{00b7} {subject} \u{00b7} {}",
                self.id(),
                self.line(),
                self.detail()
            ),
            None => format!(
                "{} \u{00b7} line {} \u{00b7} {}",
                self.id(),
                self.line(),
                self.detail()
            ),
        }
    }
}

/// One spec-sheet row, resolved against the mapping the dialog is showing.
///
/// A refused row is kept rather than dropped. An import that silently omitted
/// the rows it could not read would leave the engineer comparing counts to work
/// out what had happened to the other four.
#[derive(Debug, Clone)]
pub struct DesignVariableImportRow {
    /// The file's own line number.
    pub line: u64,
    /// The name as written. A row refused *for* its name still has to be
    /// identifiable in the table, so this is the raw cell rather than a
    /// validated identifier.
    pub name: String,
    /// The row resolved against the current column mapping.
    pub draft: DesignVariableDraft,
    /// Why this row cannot be adopted, if it cannot.
    pub refusal: Option<VariableImportRefusal>,
    /// Whether the operator has this row ticked. A refused row is never ticked
    /// and cannot be.
    pub accepted: bool,
}

impl DesignVariableImportRow {
    /// Whether this row is one the sheet can contribute at all.
    #[must_use]
    pub const fn is_adoptable(&self) -> bool {
        self.refusal.is_none()
    }
}

/// A spec sheet part-way through being imported.
///
/// The sheet is read once and kept as cells. Everything the dialog offers —
/// which column feeds which field, which rows to take, what scope to adopt at —
/// re-resolves those same cells, so changing a mapping does not ask for the
/// file again and no control edits the sheet.
///
/// [`Self::rows`] is a derived view, rebuilt when a choice changes rather than
/// on every frame: resolving five hundred rows means several hundred quantity
/// parses, and a dialog that did that at frame rate would be doing it for the
/// whole time it was open.
#[derive(Debug, Clone)]
pub struct DesignVariableImportDraft {
    /// The file the sheet came from, as receipts and refusals name it.
    pub source_name: String,
    /// The sheet's own header cells, in file order. These are what the mapping
    /// controls offer, so an engineer binds a column by the name they wrote.
    pub headers: Vec<String>,
    /// Every data row's raw cells, with the line each came from. Parsed once,
    /// when the sheet arrived.
    pub cells: Vec<(u64, Vec<String>)>,
    /// Which sheet field each declared column reads from: indexed by declared
    /// column, holding a position in [`Self::headers`]. `None` is a column the
    /// sheet does not supply, which falls back to the create dialog's default.
    pub binding: Vec<Option<usize>>,
    /// The ownership scope this import adopts at.
    pub scope: usize,
    /// Whether [`Self::scope`] replaces the sheet's own scope column. Off, each
    /// row keeps what it declares; on, one choice covers the whole sheet, which
    /// is the common case for a sheet written without scopes in mind.
    pub override_scope: bool,
    /// The resolved rows, in file order.
    pub rows: Vec<DesignVariableImportRow>,
    /// A refusal about the sheet itself rather than any row of it. While this
    /// is set there is nothing to preview and nothing to adopt.
    pub sheet_refusal: Option<VariableImportRefusal>,
    pub validation_error: Option<String>,
}

impl DesignVariableImportDraft {
    /// Open on a sheet that has been read but not yet resolved.
    ///
    /// The caller supplies the parse because parsing is the import surface's
    /// job, not this file's; what belongs here is the shape of the transaction
    /// while it is open.
    #[must_use]
    pub fn new(
        source_name: impl Into<String>,
        headers: Vec<String>,
        cells: Vec<(u64, Vec<String>)>,
        binding: Vec<Option<usize>>,
    ) -> Self {
        Self {
            source_name: source_name.into(),
            headers,
            cells,
            binding,
            scope: 0,
            override_scope: false,
            rows: Vec::new(),
            sheet_refusal: None,
            validation_error: None,
        }
    }

    /// A sheet that could not be read far enough to preview anything.
    #[must_use]
    pub fn refused(source_name: impl Into<String>, refusal: VariableImportRefusal) -> Self {
        Self {
            source_name: source_name.into(),
            headers: Vec::new(),
            cells: Vec::new(),
            binding: Vec::new(),
            scope: 0,
            override_scope: false,
            rows: Vec::new(),
            sheet_refusal: Some(refusal),
            validation_error: None,
        }
    }

    /// The rows the operator has ticked.
    ///
    /// Deliberately *not* filtered by whether a row can be adopted. Filtering
    /// here is how a ticked row that cannot land gets dropped without anyone
    /// being told, which is the one thing this import must never do: what lands
    /// is what was on screen, or nothing does. A ticked row that is refused
    /// stops the commit instead, and the surface says which row and why.
    pub fn accepted_rows(&self) -> impl Iterator<Item = &DesignVariableImportRow> {
        self.rows.iter().filter(|row| row.accepted)
    }

    /// How many rows are ticked.
    #[must_use]
    pub fn accepted_count(&self) -> usize {
        self.accepted_rows().count()
    }
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
