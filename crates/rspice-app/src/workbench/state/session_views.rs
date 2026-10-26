//! Session-scoped state for verification and the capability views.
//!
//! Everything here is retained only for the active session and never treated
//! as evidence: a regression receipt records what a review saw, but the source
//! runs and datasets remain the authoritative owners. That is why none of it
//! is durable — reopening the project must re-derive it.

use super::*;

/// Immutable regression comparison receipt retained only for the active
/// verification review session. The source runs and datasets remain the
/// authoritative evidence owners.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct RegressionComparisonReceipt {
    pub plan_id: crate::product::SimulationPlanId,
    pub plan_revision: crate::product::ObjectRevision,
    pub tolerance_digest: crate::product::ContentDigest,
    pub baseline_run: crate::product::RunId,
    pub candidate_run: crate::product::RunId,
    pub baseline_dataset: crate::product::DatasetId,
    pub candidate_dataset: crate::product::DatasetId,
    pub baseline_content_digest: crate::product::ContentDigest,
    pub candidate_content_digest: crate::product::ContentDigest,
    pub baseline_authority_digest: crate::product::ContentDigest,
    pub candidate_authority_digest: crate::product::ContentDigest,
    pub aligned_checks: usize,
    pub aligned_waveforms: usize,
    pub changed_checks: usize,
    #[serde(default)]
    pub passed_checks: usize,
    #[serde(default)]
    pub failed_checks: usize,
    #[serde(default)]
    pub passed_waveforms: usize,
    #[serde(default)]
    pub failed_waveforms: usize,
    #[serde(default)]
    pub unconfigured_targets: usize,
    #[serde(default)]
    pub unevaluated_targets: usize,
}

#[derive(Debug, Clone, PartialEq)]
pub struct RegressionToleranceDraft {
    pub target: crate::state::RegressionTargetSelector,
    pub method: crate::state::RegressionComparisonMethod,
    pub absolute_tolerance: String,
    pub relative_tolerance_percent: String,
    pub time_skew_allowance: String,
    pub comparison_window: String,
    pub dirty: bool,
    pub validation_error: Option<String>,
}

/// Runtime-only candidate value owned by the non-destructive parameter tuner.
/// The persisted `DesignVariable` remains authoritative until the user commits
/// the complete candidate set as one simulation-plan revision.
#[derive(Debug, Clone, PartialEq)]
pub struct TuningVariableDraft {
    pub variable_id: crate::product::DesignVariableId,
    pub baseline_expression: String,
    pub candidate_expression: String,
    pub validation_error: Option<String>,
    /// A proposed variable does not exist in the authoritative plan yet.
    /// Its candidate expression is still initialized from the selected
    /// instance literal, so this bit participates in dirty detection.
    pub proposed: bool,
}

impl TuningVariableDraft {
    pub fn is_dirty(&self) -> bool {
        self.proposed || self.candidate_expression.trim() != self.baseline_expression.trim()
    }
}

/// Runtime-only bridge between a selected schematic instance and the tuning
/// sandbox.  The proposed variable and binding are deliberately kept outside
/// both authoritative stores until the review dialog commits them together.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct TuningInstanceBindingDraft {
    pub component_id: u64,
    pub component_name: String,
    pub source_view: crate::state::CellViewRef,
    pub source_topology_version: u64,
    pub source_value: String,
    pub binding_expression: String,
    pub variable: crate::state::DesignVariable,
    pub creates_variable: bool,
}

impl TuningInstanceBindingDraft {
    #[must_use]
    pub fn requires_schematic_edit(&self) -> bool {
        self.source_value.trim() != self.binding_expression
    }
}

/// One limit cell of the Monte Carlo yield table, open for entry.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct YieldLimitDraft {
    /// The output whose limit is being entered, as the trials name it.
    pub output: String,
    pub bound: YieldLimitBound,
    /// What the reader has typed.
    pub text: String,
    /// What the cell said when it opened. Applying it unchanged changes
    /// nothing, so a limit spelled to four digits is never rounded by a click.
    pub seed: String,
    /// Set until the cell first paints, so the field takes the keyboard once.
    pub focus: bool,
}

/// Which bound of a limit a yield cell states.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum YieldLimitBound {
    Lower,
    Upper,
}

#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
pub struct VerificationSessionState {
    #[serde(default)]
    pub regression_baseline_run: Option<crate::product::RunId>,
    /// When enabled, process-corner cells are expressed as exact deltas from
    /// the revision-matched TT/nominal/room point retained by the active
    /// corner plan. The flag is review state only; result datasets remain
    /// immutable.
    #[serde(default)]
    pub corner_compare_nominal: bool,
    /// The Monte Carlo yield row whose histogram the page shows, by the
    /// specification's measurement name. Review state only.
    #[serde(skip)]
    pub yield_selected_spec: Option<String>,
    /// The yield table's limit cell open for entry. Transient: a limit is
    /// the plan's only once it is applied.
    #[serde(skip)]
    pub yield_limit_draft: Option<YieldLimitDraft>,
    #[serde(skip)]
    pub regression_comparison: Option<RegressionComparisonReceipt>,
    #[serde(skip)]
    pub regression_baseline_picker_open: bool,
    #[serde(skip)]
    pub regression_baseline_picker_selection: Option<crate::product::RunId>,
    #[serde(skip)]
    pub regression_selected_target: Option<crate::state::RegressionTargetSelector>,
    #[serde(skip)]
    pub regression_tolerance_drafts: Vec<RegressionToleranceDraft>,
    /// Stable plan/revision that owns `tuning_variables`. All three fields are
    /// transient so an interrupted sandbox can never restore as project data.
    #[serde(skip)]
    pub tuning_plan_id: Option<crate::product::SimulationPlanId>,
    #[serde(skip)]
    pub tuning_plan_revision: Option<crate::product::ObjectRevision>,
    #[serde(skip)]
    pub tuning_variables: Vec<TuningVariableDraft>,
    /// Optional instance Value-row proposal that opened this sandbox.
    #[serde(skip)]
    pub tuning_instance_binding: Option<TuningInstanceBindingDraft>,
    /// Selected variable card and one-shot editor focus request. Both are
    /// transient presentation state and never confer commit authority.
    #[serde(skip)]
    pub tuning_selected_variable: Option<crate::product::DesignVariableId>,
    #[serde(skip)]
    pub tuning_focus_variable: Option<crate::product::DesignVariableId>,
    /// Immutable retained run selected when the sandbox was opened. This is
    /// presentation state only; the run and dataset remain owned by
    /// `SimulationState` and are never copied into the tuner.
    #[serde(skip)]
    pub tuning_baseline_run: Option<crate::product::RunId>,
    /// Review gate opened by the mockup-specified tuning commit action. The
    /// dialog is transient and never restores across an application session.
    #[serde(skip)]
    pub tuning_review_open: bool,
    /// Why the last Verify action the reader took could not be carried out,
    /// stated under the toolbar until the next action. An action that worked
    /// leaves nothing here: its result is the page itself.
    #[serde(skip)]
    pub action_problem: Option<String>,
}

/// Corpus projection selected in the Models & PDKs catalog. This is
/// presentation state only: attaching a source or adding a project model is a
/// separate guarded engineering transaction.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default, Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub enum ModelsCatalogScope {
    #[default]
    Project,
    InstalledPacks,
    RSpiceLibrary,
}

impl ModelsCatalogScope {
    #[must_use]
    pub const fn label(self) -> &'static str {
        match self {
            Self::Project => "Project",
            Self::InstalledPacks => "Installed packs",
            Self::RSpiceLibrary => "RSpice library",
        }
    }
}

/// Everything a distributed-release confirmation states before it acts.
///
/// It is a captured projection rather than a live query: the dialog must show
/// the same release it will install, and re-reading the catalog while the
/// dialog is open could change the answer between the sentence and the click.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct PackReleaseConfirmation {
    pub name: String,
    pub version: String,
    pub spdx: String,
    pub archive_length: u64,
    pub parts: usize,
    pub capabilities: Vec<String>,
    /// Capabilities this engine build does not offer. Non-empty disables the
    /// action and is the reason shown for it.
    pub missing: Vec<String>,
    /// The one part this confirmation came to add, when it came from the
    /// component shelf rather than from the pack table.
    pub part: Option<String>,
    /// The installed version this replaces, when the action is an update.
    pub replaces: Option<String>,
}

impl PackReleaseConfirmation {
    /// Describes one published release from whatever the session already
    /// proved about it.
    ///
    /// The signed catalog is preferred because it is the only source that
    /// knows the download size; an installed release whose catalog entry has
    /// since been withdrawn is still described, from its own signed manifest,
    /// so a part already on this machine never becomes undescribable.
    #[must_use]
    pub fn for_release(
        service: &crate::services::model_hub::ModelHubService,
        pack_id: &str,
        version: &str,
        part: Option<String>,
    ) -> Option<Self> {
        let hub = service.hub()?;
        let listed = hub.snapshot().and_then(|snapshot| {
            snapshot
                .packs
                .iter()
                .find(|pack| pack.id == pack_id)
                .and_then(|pack| {
                    pack.releases
                        .iter()
                        .find(|release| release.version == version)
                        .map(|release| (pack, release))
                })
        });
        let missing =
            |capabilities: &[String]| crate::state::model_hub::missing_capabilities(capabilities);
        if let Some((pack, release)) = listed {
            return Some(Self {
                name: pack.name.clone(),
                version: release.version.clone(),
                spdx: release.spdx.clone(),
                archive_length: release.archive_length,
                parts: release.parts.len(),
                capabilities: release.capabilities.clone(),
                missing: missing(&release.capabilities),
                part,
                replaces: None,
            });
        }
        let installed = hub
            .installed()
            .iter()
            .find(|pack| pack.pack_id() == pack_id && pack.version() == version)?;
        Some(Self {
            name: installed.manifest.pack.name.clone(),
            version: installed.version().to_owned(),
            spdx: installed.manifest.license.spdx.clone(),
            archive_length: 0,
            parts: installed.manifest.parts.len(),
            capabilities: installed.manifest.requires.capabilities.clone(),
            missing: missing(&installed.manifest.requires.capabilities),
            part,
            replaces: None,
        })
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ModelsWorkbenchDialog {
    #[cfg(target_arch = "wasm32")]
    SelectBrowserImportRoot {
        candidates: Vec<String>,
        selected: usize,
    },
    SourcePreview {
        title: String,
        subtitle: String,
        source: String,
        /// Whether this source can be authored, which offers a route into
        /// Model Editor. The preview itself is never a writable buffer — it
        /// holds a copy of the retained bytes with nowhere to write back to.
        editable: bool,
    },
    CompareModels {
        left_library: String,
        left_model: String,
        /// The chosen counterpart as `(library, model)`. `None` means the
        /// dialog is still asking: a comparison against an arbitrary
        /// definition is not a comparison, so there is no default beyond the
        /// same card in another library.
        right: Option<(String, String)>,
    },
    ConfirmPack {
        pack_id: String,
        attach: bool,
        /// The distributed release this confirmation is about, when it is one.
        ///
        /// A shipped-corpus attach and a signed-release install are the same
        /// user decision — "commit this pack to this project" — asked about
        /// two different objects. One dialog asks it, and this is what makes
        /// the release case able to state its version, licence, size, and
        /// capability verdict before the user says yes.
        release: Option<Box<PackReleaseConfirmation>>,
    },
    ConfirmPart {
        pack_id: String,
        part_name: String,
    },
    /// The Model Hub's reference material, in one place: what the held catalog
    /// is, the contract it is accepted under, and the last thing this session
    /// tried to do with it.
    ///
    /// It carries no fields because it states nothing the workspace does not
    /// already hold. Capturing a copy would let the card and the page it was
    /// opened from disagree after a refresh lands underneath it.
    HeldCatalog,
    AuthorTechnologySymbolVariant {
        package_id: String,
        source_cell: String,
        target_library: String,
        target_cell: String,
    },
    DefinitionConflict {
        definition: String,
        scope: crate::state::model_library::ModelConsumerScope,
        providers: Vec<String>,
        selected_provider: String,
        reason: String,
        error: Option<String>,
    },
    BindingTrace {
        model: String,
        consumers: Vec<String>,
    },
    AddCorner {
        library: String,
        name: String,
        temperature_c: String,
        supply_factor: String,
    },
    EditCorner {
        library: String,
        original_name: String,
        duplicate: bool,
        name: String,
        description: String,
        nmos_corner: String,
        pmos_corner: String,
        temperature_c: String,
        supply_factor: String,
        minimum_temperature_c: String,
        maximum_temperature_c: String,
        required_domains: Vec<crate::state::model_library::CornerSectionDomain>,
        make_default: bool,
    },
    ConfirmDeleteCorner {
        library: String,
        corner: String,
    },
    BindCornerSection {
        library: String,
        corner: String,
        domain: crate::state::model_library::CornerSectionDomain,
        section: String,
    },
    /// The findings one library's last source-drift scan produced, and the
    /// one repair. It names the library rather than carrying the findings so
    /// the dialog and the pages behind it read the same retained report.
    ResolveDrift {
        library: String,
    },
}

/// Local presentation state for the canonical Jobs manager. Selection uses
/// stable run identity so history insertion, pruning, and reload cannot
/// silently retarget the inspector or exported manifest.
#[derive(Debug, Clone, Copy, Default, PartialEq, Eq)]
pub enum JobsPlanScope {
    ActivePlan,
    #[default]
    AllPlans,
    ManualDeck,
}

impl JobsPlanScope {
    pub const ALL: [Self; 3] = [Self::ActivePlan, Self::AllPlans, Self::ManualDeck];

    pub const fn label(self) -> &'static str {
        match self {
            Self::ActivePlan => "Active plan",
            Self::AllPlans => "All plans",
            Self::ManualDeck => "Manual decks",
        }
    }
}

#[derive(Debug, Clone, Default)]
pub struct JobsManagerState {
    pub selected_run_id: Option<crate::product::RunId>,
    pub scroll_offset: f32,
    pub plan_scope: JobsPlanScope,
}
