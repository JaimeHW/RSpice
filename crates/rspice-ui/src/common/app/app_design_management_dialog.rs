//! Mockup-owned Design Management manager and subordinate transactions.
//!
//! The manager owns one isolated [`DesignManagementCatalog`] draft. Subordinate
//! workflows mutate that same draft atomically; only the manager's primary
//! action can publish the complete candidate into the project workspace.

use std::collections::{BTreeMap, BTreeSet};

use egui::{Align, Context, Frame, Layout, Margin, Sense, Stroke, Ui, Vec2, vec2};

use crate::state::{
    AnnotationCollisionPolicy, AnnotationObject, AnnotationPosition, AnnotationPrefixAllocation,
    AnnotationRangeScope, AnnotationReservedRange, AssemblyVariantDraft, AssemblyVariantId,
    CrossSheetDiscipline, CrossSheetPortAnchor, CrossSheetPortDefinition, CrossSheetPortDirection,
    CrossSheetPortEndpoint, CrossSheetSignalType, DesignManagementCatalog,
    HierarchyAuditConfiguration, HierarchyAuditRequest, HierarchyAuditSubject,
    ImportedReferencePolicy, MissingReplacementPolicy, ModelEquivalencePolicy,
    MoveBoundaryResolution, MoveSelectionRequest, PortDirection, PortDiscipline,
    ProtectedReferencePolicy, RenumberOrder, RenumberRequest, RenumberScope,
    ReorderCrossReferences, ReorderPageNumbering, SchematicObjectKey, SheetDefinition,
    SheetDeleteBehavior, SheetId, SheetPageNumbering, SheetPortPolicy, SheetTemplate,
    VariantInheritance, VariantMatrixEdit, VariantQualificationPlan,
};
use crate::ui::theme::{self, FontWeight};
use crate::ui::tokens::{self, Tokens};
use crate::ui::widgets::{
    Button, Dialog, DialogChoice, DialogInitialFocus, DialogSize, DialogTransactionTone,
};
use crate::workbench::design_system::{property_row, property_row_toned, section_header};

use super::app_mockup_review::{input_field, read_only_field};
use super::{AppState, ConsoleMessage, DesignManagementHistoryEntry, RSpiceApp};

const MANAGER_EYEBROW: &str = "SCHEMATIC \u{00b7} DESIGN IDENTITY \u{00b7} TRANSACTIONAL AUTHORING";
const MANAGER_TITLE: &str = "Sheets, variants, annotation and hierarchy";
const MANAGER_PRIMARY: &str = "Apply reviewed design changes";
const MANAGER_DESCRIPTION: &str = "Manage stable sheet identity, governed assembly variants, reference-designator ownership, and exact hierarchy resolution.";
const MANAGER_INITIAL_HEIGHT: f32 = 590.0;
const SUBFLOW_INITIAL_HEIGHT: f32 = 610.0;
const TAB_HEIGHT: f32 = 34.0;
const TAB_MIN_WIDTH: f32 = 112.0;
const SPLIT_BREAKPOINT: f32 = 720.0;
const MAIN_SPLIT_LEFT_FRACTION: f32 = 0.5;
const SUBFLOW_LEFT_FRACTION: f32 = 0.66;

#[derive(Debug, Clone, Copy, Default, PartialEq, Eq)]
pub(crate) enum DesignManagementTab {
    #[default]
    Sheets,
    Variants,
    Annotation,
    Hierarchy,
}

impl DesignManagementTab {
    const ALL: [Self; 4] = [
        Self::Sheets,
        Self::Variants,
        Self::Annotation,
        Self::Hierarchy,
    ];

    const fn label(self) -> &'static str {
        match self {
            Self::Sheets => "Sheets",
            Self::Variants => "Variants",
            Self::Annotation => "Annotation",
            Self::Hierarchy => "Hierarchy",
        }
    }
}

#[derive(Debug, Clone, Copy, Default, PartialEq, Eq)]
pub(crate) enum DesignManagementPage {
    #[default]
    Manager,
    NewSheet,
    ReorderSheets,
    MoveSelection,
    NewVariant,
    CompareVariants,
    VariantMatrix,
    RenumberPreview,
    AnnotationPolicy,
    HierarchyAudit,
}

impl DesignManagementPage {
    const fn title(self) -> &'static str {
        match self {
            Self::Manager => MANAGER_TITLE,
            Self::NewSheet => "Create schematic sheet",
            Self::ReorderSheets => "Reorder schematic sheets",
            Self::MoveSelection => "Move selection to another sheet",
            Self::NewVariant => "Create assembly variant",
            Self::CompareVariants => "Compare assembly variants",
            Self::VariantMatrix => "Variant substitution matrix",
            Self::RenumberPreview => "Preview reference-designator renumbering",
            Self::AnnotationPolicy => "Edit annotation policy",
            Self::HierarchyAudit => "Audit hierarchy and view resolution",
        }
    }

    const fn eyebrow(self) -> &'static str {
        match self {
            Self::Manager => MANAGER_EYEBROW,
            Self::NewSheet => "SCHEMATIC \u{00b7} STABLE SHEET IDENTITY",
            Self::ReorderSheets => "SCHEMATIC \u{00b7} PRESENTATION ORDER",
            Self::MoveSelection => "SCHEMATIC \u{00b7} CONNECTIVITY-PRESERVING MOVE",
            Self::NewVariant => "SCHEMATIC \u{00b7} VERSIONED ASSEMBLY INTENT",
            Self::CompareVariants => "SCHEMATIC \u{00b7} SEMANTIC VARIANT DELTA",
            Self::VariantMatrix => "SCHEMATIC \u{00b7} CONFIGURATION CONTROL",
            Self::RenumberPreview => "SCHEMATIC \u{00b7} STABLE-ID MAPPING",
            Self::AnnotationPolicy => "SCHEMATIC \u{00b7} REFERENCE OWNERSHIP",
            Self::HierarchyAudit => "SCHEMATIC \u{00b7} CONFIGURATION PREFLIGHT",
        }
    }

    const fn primary(self) -> &'static str {
        match self {
            Self::Manager => MANAGER_PRIMARY,
            Self::NewSheet => "Create sheet",
            Self::ReorderSheets => "Apply order",
            Self::MoveSelection => "Create reviewed move",
            Self::NewVariant => "Create variant",
            Self::CompareVariants => "Open comparison",
            Self::VariantMatrix => "Apply reviewed substitutions",
            Self::RenumberPreview => "Create renumber transaction",
            Self::AnnotationPolicy => "Save policy",
            Self::HierarchyAudit => "Run audit",
        }
    }

    const fn code(self) -> &'static str {
        match self {
            Self::Manager => "DESIGN",
            Self::NewSheet => "SHEET+",
            Self::ReorderSheets => "ORDER",
            Self::MoveSelection => "MOVE",
            Self::NewVariant => "VAR+",
            Self::CompareVariants => "VAR\u{0394}",
            Self::VariantMatrix => "MATRIX",
            Self::RenumberPreview => "REFDES",
            Self::AnnotationPolicy => "POLICY",
            Self::HierarchyAudit => "HIER",
        }
    }

    const fn invariant(self) -> &'static str {
        match self {
            Self::Manager => "one reversible working-revision transaction",
            Self::NewSheet => "stable cross-sheet port identities",
            Self::ReorderSheets => "connectivity and stable IDs unchanged",
            Self::MoveSelection => "typed boundary nets preserved",
            Self::NewVariant => "parent remains immutable",
            Self::CompareVariants => "no source mutation",
            Self::VariantMatrix => "qualified replacement policy",
            Self::RenumberPreview => "external and locked references retained",
            Self::AnnotationPolicy => "import source map retained",
            Self::HierarchyAudit => "protected boundaries verified",
        }
    }
}

#[derive(Debug, Clone, Copy, Default, PartialEq, Eq)]
enum MoveBoundaryPolicy {
    #[default]
    TypedPorts,
    ReviewedGlobalAliases,
    Block,
}

#[derive(Debug, Clone, Copy, Default, PartialEq, Eq)]
enum MoveHierarchyEffect {
    #[default]
    SameCell,
    CreateChildCell,
}

#[derive(Debug, Clone, Copy, Default, PartialEq, Eq)]
enum VariantDifferenceClasses {
    #[default]
    DevicesValuesDnpModels,
    ConnectivityOnly,
}

#[derive(Debug, Clone, Copy, Default, PartialEq, Eq)]
enum VariantMatrixScope {
    #[default]
    AllControlledInstances,
    CurrentHierarchy,
}

#[derive(Debug, Clone, Copy, Default, PartialEq, Eq)]
enum RenumberScopeChoice {
    #[default]
    WholeProject,
    CurrentHierarchy,
    CurrentSheet,
}

#[derive(Debug, Clone, Copy, Default, PartialEq, Eq)]
enum HierarchyViewChecks {
    #[default]
    AllDeclaredFallbacks,
    SelectedHierarchy,
}

#[derive(Debug, Clone, Copy, Default, PartialEq, Eq)]
enum ProtectedBoundaryChecks {
    #[default]
    SignaturesAndPins,
    PinsOnly,
}

#[derive(Debug, Clone, PartialEq, Eq)]
struct SubflowInputs {
    sheet_name: String,
    sheet_insert_after: Option<SheetId>,
    sheet_template: SheetTemplate,
    sheet_port_policy: SheetPortPolicy,
    reorder_order_text: String,
    reorder_page_numbering: ReorderPageNumbering,
    move_destination: Option<SheetId>,
    move_boundary_policy: MoveBoundaryPolicy,
    move_hierarchy_effect: MoveHierarchyEffect,
    variant_name: String,
    variant_parent: Option<AssemblyVariantId>,
    variant_inheritance: VariantInheritance,
    variant_qualification: VariantQualificationPlan,
    compare_reference: Option<AssemblyVariantId>,
    compare_target: Option<AssemblyVariantId>,
    difference_classes: VariantDifferenceClasses,
    matrix_scope: VariantMatrixScope,
    missing_replacement: MissingReplacementPolicy,
    model_equivalence: ModelEquivalencePolicy,
    renumber_scope: RenumberScopeChoice,
    renumber_order: RenumberOrder,
    protected_references: ProtectedReferencePolicy,
    prefix_allocation: AnnotationPrefixAllocation,
    reserved_ranges: String,
    imported_ids: ImportedReferencePolicy,
    audit_configuration: Option<crate::state::ConfigurationSetId>,
    audit_view_checks: HierarchyViewChecks,
    audit_protected_boundaries: ProtectedBoundaryChecks,
}

impl Default for SubflowInputs {
    fn default() -> Self {
        Self {
            sheet_name: "Power and references".to_owned(),
            sheet_insert_after: None,
            sheet_template: SheetTemplate::AnalogSchematic,
            sheet_port_policy: SheetPortPolicy::TypedOffSheetPorts,
            reorder_order_text: String::new(),
            reorder_page_numbering: ReorderPageNumbering::UpdatePrintPageNumbers,
            move_destination: None,
            move_boundary_policy: MoveBoundaryPolicy::TypedPorts,
            move_hierarchy_effect: MoveHierarchyEffect::SameCell,
            variant_name: "Automotive high-temperature".to_owned(),
            variant_parent: None,
            variant_inheritance: VariantInheritance::OverrideChangedObjectsOnly,
            variant_qualification: VariantQualificationPlan::InvalidateAffectedTests,
            compare_reference: None,
            compare_target: None,
            difference_classes: VariantDifferenceClasses::DevicesValuesDnpModels,
            matrix_scope: VariantMatrixScope::AllControlledInstances,
            missing_replacement: MissingReplacementPolicy::Block,
            model_equivalence: ModelEquivalencePolicy::RequireQualifiedReplacement,
            renumber_scope: RenumberScopeChoice::WholeProject,
            renumber_order: RenumberOrder::HierarchyThenCoordinates,
            protected_references: ProtectedReferencePolicy::RetainLockedAndExternalIds,
            prefix_allocation: AnnotationPrefixAllocation::ByDeviceFamily,
            reserved_ranges: "Project-owned ranges".to_owned(),
            imported_ids: ImportedReferencePolicy::PreserveWithSourceMap,
            audit_configuration: None,
            audit_view_checks: HierarchyViewChecks::AllDeclaredFallbacks,
            audit_protected_boundaries: ProtectedBoundaryChecks::SignaturesAndPins,
        }
    }
}

#[derive(Debug, Clone)]
pub(crate) struct DesignManagementDialogState {
    pub(crate) open: bool,
    page: DesignManagementPage,
    active_tab: DesignManagementTab,
    owner_key: String,
    original: Option<DesignManagementCatalog>,
    draft: Option<DesignManagementCatalog>,
    inputs: SubflowInputs,
    input_baseline: Option<SubflowInputs>,
    selection_object_ids: Vec<u64>,
    all_object_ids: Vec<u64>,
    selection_summary: String,
    default_sheet_name: String,
    error: Option<String>,
    receipt: Option<String>,
    discard_confirmation: bool,
    body_scroll_offset: f32,
}

impl Default for DesignManagementDialogState {
    fn default() -> Self {
        Self {
            open: false,
            page: DesignManagementPage::Manager,
            active_tab: DesignManagementTab::Sheets,
            owner_key: String::new(),
            original: None,
            draft: None,
            inputs: SubflowInputs::default(),
            input_baseline: None,
            selection_object_ids: Vec::new(),
            all_object_ids: Vec::new(),
            selection_summary: "No complete schematic objects selected".to_owned(),
            default_sheet_name: "Sheet 1".to_owned(),
            error: None,
            receipt: None,
            discard_confirmation: false,
            body_scroll_offset: 0.0,
        }
    }
}

impl DesignManagementDialogState {
    fn open(
        &mut self,
        catalog: &DesignManagementCatalog,
        owner_key: String,
        selection_object_ids: Vec<u64>,
        all_object_ids: Vec<u64>,
        selection_summary: String,
        default_sheet_name: String,
    ) -> Result<(), String> {
        let mut draft = catalog.clone();
        if draft.sheet_catalog(&owner_key).is_none() {
            draft
                .bootstrap_for_cell_view(
                    &owner_key,
                    default_sheet_name.clone(),
                    all_object_ids.iter().copied(),
                )
                .map_err(|error| error.to_string())?;
        } else if let Some(sheet_catalog) = draft.sheet_catalog_mut(&owner_key) {
            let revision = sheet_catalog.revision();
            let active = sheet_catalog.active_sheet_id();
            sheet_catalog
                .reconcile_object_assignments(revision, all_object_ids.iter().copied(), active)
                .map_err(|error| error.to_string())?;
        }
        *self = Self {
            open: true,
            owner_key,
            original: Some(catalog.clone()),
            draft: Some(draft),
            selection_object_ids,
            all_object_ids,
            selection_summary,
            default_sheet_name,
            ..Self::default()
        };
        self.reset_inputs_for_page(DesignManagementPage::Manager, None, None);
        Ok(())
    }

    pub(crate) fn close_and_discard(&mut self) {
        *self = Self::default();
    }

    fn dirty(&self) -> bool {
        self.original
            .as_ref()
            .zip(self.draft.as_ref())
            .is_some_and(|(original, draft)| original != draft)
    }

    fn subflow_dirty(&self) -> bool {
        self.input_baseline
            .as_ref()
            .is_some_and(|baseline| baseline != &self.inputs)
    }

    fn reset_inputs_for_page(
        &mut self,
        page: DesignManagementPage,
        configurations: Option<&crate::state::ConfigurationSetCatalog>,
        active_sheet: Option<SheetId>,
    ) {
        let mut inputs = SubflowInputs::default();
        if let Some(draft) = self.draft.as_ref() {
            if let Some(sheets) = draft.sheet_catalog(&self.owner_key) {
                inputs.sheet_insert_after = sheets.active_sheet_id();
                inputs.reorder_order_text = sheets
                    .sheets()
                    .iter()
                    .map(|sheet| sheet.name())
                    .collect::<Vec<_>>()
                    .join(" \u{2192} ");
                inputs.move_destination = sheets
                    .sheets()
                    .iter()
                    .map(|sheet| sheet.id())
                    .find(|id| Some(*id) != active_sheet)
                    .or(active_sheet);
            }
            let variants = draft.variants();
            inputs.variant_parent = variants.active_variant_id();
            inputs.compare_reference = variants
                .active_variant_id()
                .or_else(|| variants.variants().first().map(|variant| variant.id()));
            inputs.compare_target = variants
                .variants()
                .iter()
                .map(|variant| variant.id())
                .find(|id| Some(*id) != inputs.compare_reference);
            let policy = draft.annotation().policy().definition();
            inputs.prefix_allocation = policy.prefix_allocation;
            inputs.imported_ids = policy.imported_ids;
            inputs.reserved_ranges = format_reserved_ranges(&policy.reserved_ranges);
        }
        if let Some(configurations) = configurations {
            inputs.audit_configuration = configurations.active_configuration_id().or_else(|| {
                configurations
                    .configurations()
                    .first()
                    .map(|configuration| configuration.id())
            });
        }
        self.page = page;
        self.inputs = inputs.clone();
        self.input_baseline = (page != DesignManagementPage::Manager).then_some(inputs);
        self.error = None;
        self.discard_confirmation = false;
        self.body_scroll_offset = 0.0;
    }
}

pub(crate) fn open_design_management_dialog(state: &mut AppState) {
    if !state.project_lifecycle.project_open {
        state.push_user_message(ConsoleMessage::warning(
            "Design Management requires an open project.".to_owned(),
        ));
        return;
    }
    if !matches!(
        state.workspace.active_view_type(),
        crate::state::ViewType::Schematic | crate::state::ViewType::Testbench
    ) {
        state.push_user_message(ConsoleMessage::warning(
            "Design Management requires a schematic or testbench design context.".to_owned(),
        ));
        return;
    }
    crate::schematic::view::retain_selection_on_active_sheet(state);
    let owner_key = state.workspace.active_schematic_reference().key();
    let selection_object_ids = selected_stable_object_ids(&state.schematic);
    let all_object_ids = all_stable_object_ids(&state.schematic);
    let selection_summary = selected_object_summary(&state.schematic, &selection_object_ids);
    let default_sheet_name = state.workspace.active_view.cell.clone();
    let catalog = state.workspace.design_management.clone();
    if let Err(error) = state.dialogs.design_management.open(
        &catalog,
        owner_key.clone(),
        selection_object_ids.clone(),
        all_object_ids.clone(),
        selection_summary.clone(),
        default_sheet_name.clone(),
    ) {
        state.dialogs.design_management = DesignManagementDialogState {
            open: true,
            owner_key,
            original: Some(catalog),
            draft: None,
            selection_object_ids,
            all_object_ids,
            selection_summary,
            default_sheet_name,
            error: Some(error.clone()),
            ..DesignManagementDialogState::default()
        };
        state.push_user_message(ConsoleMessage::error(format!(
            "Design Management could not open: {error}"
        )));
    }
}

fn all_stable_object_ids(schematic: &crate::state::SchematicState) -> Vec<u64> {
    let mut ids = BTreeSet::new();
    ids.extend(schematic.components.iter().map(|object| object.id));
    ids.extend(schematic.wires.iter().map(|object| object.id));
    ids.extend(schematic.buses.iter().map(|object| object.id));
    ids.extend(schematic.bus_taps.iter().map(|object| object.id));
    ids.extend(schematic.junctions.iter().map(|object| object.id));
    ids.extend(schematic.net_labels.iter().map(|object| object.id));
    ids.extend(schematic.design_notes.iter().map(|object| object.id));
    ids.extend(
        schematic
            .documentation_shapes
            .iter()
            .map(|object| object.id),
    );
    ids.into_iter().collect()
}

fn selected_stable_object_ids(schematic: &crate::state::SchematicState) -> Vec<u64> {
    let selection = &schematic.selection;
    let mut ids = BTreeSet::new();
    ids.extend(selection.components.iter().copied());
    ids.extend(selection.all_selected_wire_ids());
    ids.extend(selection.buses.iter().copied());
    ids.extend(selection.bus_taps.iter().copied());
    ids.extend(selection.net_labels.iter().copied());
    ids.extend(selection.design_notes.iter().copied());
    ids.extend(selection.documentation_shapes.iter().copied());
    ids.extend(selection.junctions.iter().filter_map(|selected| {
        schematic
            .junctions
            .iter()
            .find(|junction| junction.pos == selected.pos)
            .map(|junction| junction.id)
    }));
    ids.into_iter().collect()
}

fn selected_object_summary(
    schematic: &crate::state::SchematicState,
    selected_ids: &[u64],
) -> String {
    if selected_ids.is_empty() {
        return "No complete schematic objects selected".to_owned();
    }
    let mut names = schematic
        .components
        .iter()
        .filter(|component| selected_ids.binary_search(&component.id).is_ok())
        .map(|component| component.name.clone())
        .take(5)
        .collect::<Vec<_>>();
    let named = names.len();
    let remaining = selected_ids.len().saturating_sub(named);
    if remaining > 0 {
        names.push(format!(
            "{remaining} other object{}",
            if remaining == 1 { "" } else { "s" }
        ));
    }
    names.join(" \u{00b7} ")
}

fn format_reserved_ranges(ranges: &[AnnotationReservedRange]) -> String {
    if ranges.is_empty() {
        return "Project-owned ranges".to_owned();
    }
    ranges
        .iter()
        .map(|range| {
            format!(
                "{} {}\u{2026}{}",
                range.prefixes.join(", "),
                range.first,
                range.last
            )
        })
        .collect::<Vec<_>>()
        .join("; ")
}

fn main_split_widths(available: f32) -> (f32, f32) {
    let usable = available.max(0.0);
    let left = (usable * MAIN_SPLIT_LEFT_FRACTION).floor();
    (left, (usable - left).max(0.0))
}

fn subflow_split_widths(available: f32) -> (f32, f32) {
    let usable = available.max(0.0);
    let left = (usable * SUBFLOW_LEFT_FRACTION).floor();
    (left, (usable - left).max(0.0))
}

#[derive(Debug, Clone, Copy, Default, PartialEq, Eq)]
enum DesignManagementBodyAction {
    #[default]
    None,
    Open(DesignManagementPage),
    OpenConfigurationBinding,
    OpenConfigurationSets,
}

impl RSpiceApp {
    pub(super) fn render_design_management_dialog(&mut self, ctx: &Context) {
        if !self.state.dialogs.design_management.open {
            return;
        }
        let page = self.state.dialogs.design_management.page;
        let manager_dirty = self.state.dialogs.design_management.dirty();
        let subflow_dirty = self.state.dialogs.design_management.subflow_dirty();
        let dirty = if page == DesignManagementPage::Manager {
            manager_dirty
        } else {
            subflow_dirty
        };
        let discard = self.state.dialogs.design_management.discard_confirmation;
        let validation = validate_design_management_page(&self.state, page);
        let write_allowed = !self.state.workbench.safe_mode.project_read_only();
        let primary_enabled = write_allowed
            && validation.is_ok()
            && if page == DesignManagementPage::Manager {
                manager_dirty
            } else {
                true
            };
        let description = if page == DesignManagementPage::Manager {
            MANAGER_DESCRIPTION
        } else {
            "Review the exact source, proposed transaction, resulting revision, and invariant before applying this operation to the manager draft."
        };
        let initial_height = if page == DesignManagementPage::Manager {
            MANAGER_INITIAL_HEIGHT
        } else {
            SUBFLOW_INITIAL_HEIGHT
        };
        let size = if page == DesignManagementPage::Manager {
            DialogSize::SimulationWorkflow
        } else {
            DialogSize::WideWorkflow
        };
        let ghost = if discard {
            "Discard changes"
        } else if page == DesignManagementPage::Manager {
            "Close"
        } else {
            "Cancel"
        };
        let error = self.state.dialogs.design_management.error.clone();
        let mut body_scroll_offset = self.state.dialogs.design_management.body_scroll_offset;
        let mut dialog = Dialog::new(page.eyebrow(), page.title(), page.primary())
            .description(description)
            .size(size)
            .initial_height(initial_height)
            .flush_body()
            .ghost(ghost)
            .primary_enabled(primary_enabled)
            .primary_on_enter(false)
            .initial_focus(DialogInitialFocus::BodyControl)
            .body_scroll_offset(&mut body_scroll_offset);
        if dirty && !discard {
            dialog = dialog.retain_on_cancel_focus(DialogInitialFocus::Ghost);
        }
        if discard {
            dialog = dialog.transaction_state(
                DialogTransactionTone::Error,
                "Discard uncommitted design-management changes?",
                "The live project remains unchanged. Subordinate operations already added to this manager draft are discarded together.",
            );
        } else if let Some(error) = error
            .as_deref()
            .or_else(|| validation.as_ref().err().map(String::as_str))
        {
            dialog = dialog.transaction_state(
                DialogTransactionTone::Error,
                "Design transaction cannot continue",
                error,
            );
        }

        let mut body_action = DesignManagementBodyAction::None;
        let choice = dialog.show_with_initial_body_focus(ctx, |ui| {
            let response = if page == DesignManagementPage::Manager {
                body_action = design_management_manager_body(
                    ui,
                    &mut self.state.dialogs.design_management,
                    &self.state.workspace,
                    &self.state.library_manager,
                    &self.state.schematic,
                    write_allowed,
                );
                None
            } else {
                Some(design_management_subflow_body(
                    ui,
                    &mut self.state.dialogs.design_management,
                    &self.state.workspace,
                    &self.state.library_manager,
                    &self.state.schematic,
                    write_allowed,
                ))
            };
            response.map(|response| response.id)
        });
        self.state.dialogs.design_management.body_scroll_offset = body_scroll_offset;
        self.handle_design_management_body_action(body_action);
        match choice {
            DialogChoice::Primary => {
                let result = if page == DesignManagementPage::Manager {
                    self.publish_design_management_draft()
                } else {
                    self.commit_design_management_subflow(page)
                };
                if let Err(error) = result {
                    self.state.dialogs.design_management.error = Some(error);
                }
            }
            DialogChoice::Ghost | DialogChoice::Cancelled => {
                if dirty && !discard {
                    self.state.dialogs.design_management.discard_confirmation = true;
                } else if page == DesignManagementPage::Manager {
                    self.state.dialogs.design_management.close_and_discard();
                    super::close_design_management_dialog_route(&mut self.state);
                } else {
                    let active_sheet = self
                        .state
                        .dialogs
                        .design_management
                        .draft
                        .as_ref()
                        .and_then(|catalog| {
                            catalog.sheet_catalog(&self.state.dialogs.design_management.owner_key)
                        })
                        .and_then(|catalog| catalog.active_sheet_id());
                    let configurations = self.state.workspace.configuration_sets.clone();
                    self.state.dialogs.design_management.reset_inputs_for_page(
                        DesignManagementPage::Manager,
                        Some(&configurations),
                        active_sheet,
                    );
                }
            }
            DialogChoice::None | DialogChoice::Secondary => {}
        }
    }

    fn handle_design_management_body_action(&mut self, action: DesignManagementBodyAction) {
        match action {
            DesignManagementBodyAction::None => {}
            DesignManagementBodyAction::Open(page) => {
                let active_sheet = self
                    .state
                    .dialogs
                    .design_management
                    .draft
                    .as_ref()
                    .and_then(|catalog| {
                        catalog.sheet_catalog(&self.state.dialogs.design_management.owner_key)
                    })
                    .and_then(|catalog| catalog.active_sheet_id());
                let configurations = self.state.workspace.configuration_sets.clone();
                self.state.dialogs.design_management.reset_inputs_for_page(
                    page,
                    Some(&configurations),
                    active_sheet,
                );
            }
            DesignManagementBodyAction::OpenConfigurationSets => {
                if self.state.dialogs.design_management.dirty() {
                    self.state.dialogs.design_management.error = Some(
                        "Apply or discard this Design Management draft before opening configuration bindings."
                            .to_owned(),
                    );
                    return;
                }
                self.state.dialogs.design_management.close_and_discard();
                super::close_design_management_dialog_route(&mut self.state);
                super::open_configuration_sets_dialog(&mut self.state);
            }
            DesignManagementBodyAction::OpenConfigurationBinding => {
                if self.state.dialogs.design_management.dirty() {
                    self.state.dialogs.design_management.error = Some(
                        "Apply or discard this Design Management draft before opening configuration bindings."
                            .to_owned(),
                    );
                    return;
                }
                self.state.dialogs.design_management.close_and_discard();
                super::close_design_management_dialog_route(&mut self.state);
                super::open_configuration_binding_dialog(&mut self.state);
            }
        }
    }

    fn publish_design_management_draft(&mut self) -> Result<(), String> {
        let before = self.state.workspace.design_management.clone();
        let draft = self
            .state
            .dialogs
            .design_management
            .draft
            .clone()
            .ok_or_else(|| "The Design Management draft is unavailable.".to_owned())?;
        draft.validate().map_err(|error| error.to_string())?;
        let schematic_tx = self
            .state
            .prepare_design_management_schematic_transaction(&draft)?;
        let owner = self.state.workspace.active_schematic_reference();
        let committed_revision = self
            .state
            .workspace
            .replace_design_management(draft)
            .map_err(|error| error.to_string())?;
        self.state
            .apply_design_management_schematic_transaction(&schematic_tx);
        let candidate = self.state.workspace.design_management.clone();
        self.state
            .record_design_management_transaction(DesignManagementHistoryEntry {
                description: "apply reviewed design-management changes".to_owned(),
                owner,
                before,
                after: candidate.clone(),
                before_schematics: schematic_tx.before,
                after_schematics: schematic_tx.after,
                committed_revision,
            });
        self.state.design_execution_epoch = self.state.design_execution_epoch.wrapping_add(1);
        self.simulation_controller.clear_prepared_run();
        self.state.ui.netlist.current_generation_input_digest = None;
        self.state.push_user_message(ConsoleMessage::info(format!(
            "Design Management revision {} was published as project revision {:?}.",
            candidate.revision(),
            committed_revision
        )));
        let owner_key = self.state.dialogs.design_management.owner_key.clone();
        let selection_object_ids = self
            .state
            .dialogs
            .design_management
            .selection_object_ids
            .clone();
        let selection_summary = self
            .state
            .dialogs
            .design_management
            .selection_summary
            .clone();
        let all_object_ids = self.state.dialogs.design_management.all_object_ids.clone();
        let default_sheet_name = self
            .state
            .dialogs
            .design_management
            .default_sheet_name
            .clone();
        self.state.dialogs.design_management.open(
            &candidate,
            owner_key,
            selection_object_ids,
            all_object_ids,
            selection_summary,
            default_sheet_name,
        )?;
        self.state.dialogs.design_management.receipt = Some(format!(
            "Published design-management revision {} \u{00b7} one project transaction",
            candidate.revision()
        ));
        Ok(())
    }
}

fn validate_design_management_page(
    state: &AppState,
    page: DesignManagementPage,
) -> Result<(), String> {
    let dialog = &state.dialogs.design_management;
    let draft = dialog
        .draft
        .as_ref()
        .ok_or_else(|| "The Design Management draft is unavailable.".to_owned())?;
    draft.validate().map_err(|error| error.to_string())?;
    match page {
        DesignManagementPage::Manager => {
            if !dialog.dirty() {
                return Err("No reviewed design changes are pending.".to_owned());
            }
        }
        DesignManagementPage::NewSheet => {
            if dialog.inputs.sheet_name.trim().is_empty() {
                return Err("Sheet name is required.".to_owned());
            }
        }
        DesignManagementPage::ReorderSheets => {
            let count = draft
                .sheet_catalog(&dialog.owner_key)
                .map_or(0, |catalog| catalog.sheets().len());
            if count < 2 {
                return Err(
                    "At least two complete sheet identities are required to reorder.".to_owned(),
                );
            }
            reorder_sheet_ids(dialog)?;
        }
        DesignManagementPage::MoveSelection => {
            if dialog.selection_object_ids.is_empty() {
                return Err(
                    "Select one or more complete schematic objects before moving them.".to_owned(),
                );
            }
            let destination = dialog
                .inputs
                .move_destination
                .ok_or_else(|| "Destination sheet is required.".to_owned())?;
            let source = dialog
                .selection_object_ids
                .iter()
                .find_map(|id| draft.sheet_for_object_or_active(&dialog.owner_key, *id));
            if source == Some(destination) {
                return Err("Destination must differ from the source sheet.".to_owned());
            }
            if dialog.inputs.move_hierarchy_effect == MoveHierarchyEffect::SameCell
                && dialog.inputs.move_boundary_policy == MoveBoundaryPolicy::Block
            {
                return Err("Boundary-net policy blocks this move.".to_owned());
            }
            let selected_components = state
                .schematic
                .components
                .iter()
                .filter(|component| dialog.selection_object_ids.contains(&component.id))
                .count();
            if selected_components != dialog.selection_object_ids.len()
                || selected_components == 0
                || state.schematic.selection.count() != state.schematic.selection.components.len()
            {
                return Err(
                    "Connectivity-preserving sheet and hierarchy moves require the same complete instance-only selection captured when Design Management opened."
                        .to_owned(),
                );
            }
        }
        DesignManagementPage::NewVariant => {
            if dialog.inputs.variant_name.trim().is_empty() {
                return Err("Variant name is required.".to_owned());
            }
        }
        DesignManagementPage::CompareVariants => {
            let reference = dialog
                .inputs
                .compare_reference
                .ok_or_else(|| "Reference variant is required.".to_owned())?;
            let comparison = dialog
                .inputs
                .compare_target
                .ok_or_else(|| "Comparison variant is required.".to_owned())?;
            if reference == comparison {
                return Err("Reference and comparison variants must be different.".to_owned());
            }
        }
        DesignManagementPage::VariantMatrix => {
            if draft.variants().variants().is_empty() {
                return Err("Create at least one governed assembly variant first.".to_owned());
            }
        }
        DesignManagementPage::RenumberPreview => {
            if state.schematic.components.is_empty() {
                return Err("The active schematic has no instances to annotate.".to_owned());
            }
        }
        DesignManagementPage::AnnotationPolicy => {
            parse_reserved_ranges(&dialog.inputs.reserved_ranges, dialog)?;
        }
        DesignManagementPage::HierarchyAudit => {}
    }
    Ok(())
}

fn design_management_manager_body(
    ui: &mut Ui,
    dialog: &mut DesignManagementDialogState,
    workspace: &crate::state::ProjectWorkspace,
    libraries: &crate::state::LibraryManager,
    schematic: &crate::state::SchematicState,
    write_allowed: bool,
) -> DesignManagementBodyAction {
    let first_focus = design_management_tabs(ui, &mut dialog.active_tab);
    let action = match dialog.active_tab {
        DesignManagementTab::Sheets => sheets_manager_body(ui, dialog, write_allowed),
        DesignManagementTab::Variants => variants_manager_body(ui, dialog, write_allowed),
        DesignManagementTab::Annotation => annotation_manager_body(ui, dialog, write_allowed),
        DesignManagementTab::Hierarchy => {
            hierarchy_manager_body(ui, dialog, workspace, libraries, schematic, write_allowed)
        }
    };
    concept_banner(
        ui,
        "Every change is one reversible working-revision transaction. Existing results retain their original hierarchy, variant, and annotation manifests.",
        true,
    );
    if let Some(receipt) = dialog.receipt.as_deref() {
        receipt_banner(ui, receipt);
    }
    let _ = first_focus;
    action
}

fn design_management_tabs(ui: &mut Ui, active: &mut DesignManagementTab) -> egui::Response {
    let t = Tokens::get(ui.ctx());
    let available = ui.available_width();
    let (strip_rect, strip_response) =
        ui.allocate_exact_size(vec2(available, TAB_HEIGHT), Sense::hover());
    ui.painter().rect_filled(strip_rect, 0.0, t.color.bg_panel);
    ui.painter().hline(
        strip_rect.x_range(),
        strip_rect.bottom(),
        Stroke::new(1.0, t.color.border_strong),
    );
    let mut x = strip_rect.left();
    let mut first = None;
    for tab in DesignManagementTab::ALL {
        let rect = egui::Rect::from_min_size(
            egui::pos2(x, strip_rect.top()),
            vec2(TAB_MIN_WIDTH, TAB_HEIGHT),
        );
        let response = ui.interact(
            rect,
            ui.id().with(("design-management-tab", tab.label())),
            Sense::click(),
        );
        response.widget_info(|| {
            egui::WidgetInfo::selected(
                egui::WidgetType::SelectableLabel,
                ui.is_enabled(),
                *active == tab,
                tab.label(),
            )
        });
        if response.clicked() {
            *active = tab;
        }
        if *active == tab {
            ui.painter().rect_filled(rect, 0.0, t.color.bg_elevated);
            ui.painter().hline(
                egui::Rangef::new(rect.left(), rect.right()),
                rect.bottom() - 1.0,
                Stroke::new(2.0, t.color.accent),
            );
        }
        ui.painter().vline(
            rect.right(),
            rect.y_range(),
            Stroke::new(1.0, t.color.border),
        );
        ui.painter().text(
            rect.left_center() + vec2(12.0, 0.0),
            egui::Align2::LEFT_CENTER,
            tab.label(),
            theme::sans(tokens::FS_1, FontWeight::Regular),
            if *active == tab {
                t.color.text
            } else {
                t.color.text_dim
            },
        );
        first.get_or_insert(response);
        x += TAB_MIN_WIDTH;
    }
    first.unwrap_or(strip_response)
}

fn sheets_manager_body(
    ui: &mut Ui,
    dialog: &mut DesignManagementDialogState,
    write_allowed: bool,
) -> DesignManagementBodyAction {
    let Some(draft) = dialog.draft.as_mut() else {
        return DesignManagementBodyAction::None;
    };
    let mut action = DesignManagementBodyAction::None;
    split_surface(ui, MAIN_SPLIT_LEFT_FRACTION, |left, right| {
        let catalog = draft.sheet_catalog(&dialog.owner_key);
        let sheets = catalog
            .map(|catalog| {
                catalog
                    .sheets()
                    .iter()
                    .map(|sheet| (sheet.id(), sheet.name().to_owned()))
                    .collect::<Vec<_>>()
            })
            .unwrap_or_default();
        let active_id = catalog.and_then(|catalog| catalog.active_sheet_id());
        let ports = catalog
            .map(|catalog| catalog.cross_sheet_ports().to_vec())
            .unwrap_or_default();
        section_header(
            left,
            "Multi-sheet design",
            Some(&format!("{} sheets \u{00b7} connected", sheets.len())),
        );
        paint_table_header(
            left,
            &[0.12, 0.31, 0.16, 0.22, 0.19],
            &["Order", "Sheet", "Ports", "Off-sheet nets", "State"],
        );
        if sheets.is_empty() {
            empty_table_row(
                left,
                "No governed sheets. Create the first stable sheet identity.",
            );
        } else {
            for (index, (id, name)) in sheets.iter().enumerate() {
                let port_count = ports
                    .iter()
                    .filter(|port| {
                        let definition = port.definition();
                        definition.first.sheet_id == *id || definition.second.sheet_id == *id
                    })
                    .count();
                let values = [
                    format!("{:02}", index + 1),
                    name.clone(),
                    port_count.to_string(),
                    port_count.to_string(),
                    if Some(*id) == active_id {
                        "current"
                    } else {
                        "governed"
                    }
                    .to_owned(),
                ];
                paint_table_row(left, &[0.12, 0.31, 0.16, 0.22, 0.19], &values, None);
            }
        }
        toolbar(left, |ui| {
            if Button::new("New sheet\u{2026}")
                .enabled(write_allowed)
                .show(ui)
                .clicked()
            {
                action = DesignManagementBodyAction::Open(DesignManagementPage::NewSheet);
            }
            if Button::new("Reorder\u{2026}")
                .enabled(write_allowed && sheets.len() > 1)
                .show(ui)
                .clicked()
            {
                action = DesignManagementBodyAction::Open(DesignManagementPage::ReorderSheets);
            }
            if Button::new("Move selection to sheet\u{2026}")
                .enabled(
                    write_allowed && sheets.len() > 1 && !dialog.selection_object_ids.is_empty(),
                )
                .show(ui)
                .clicked()
            {
                action = DesignManagementBodyAction::Open(DesignManagementPage::MoveSelection);
            }
        });

        section_header(right, "Sheet contract", None);
        let mut selected = active_id;
        if setting_combo_by_id(right, "Active sheet", &sheets, &mut selected, write_allowed)
            && let Some(id) = selected
            && let Some(catalog) = draft.sheet_catalog_mut(&dialog.owner_key)
        {
            apply_domain_result(
                &mut dialog.error,
                catalog.set_active(id),
                "Active sheet changed.",
            );
        }
        if let Some(settings) = draft
            .sheet_catalog(&dialog.owner_key)
            .map(|catalog| catalog.settings().clone())
        {
            let mut connector = settings.connector_policy;
            let mut page_numbering = settings.page_numbering;
            let mut delete_behavior = settings.delete_behavior;
            let mut changed = false;
            changed |= setting_combo(
                right,
                "Off-sheet connector policy",
                &[
                    (
                        crate::state::OffSheetConnectorPolicy::TypedPortsWithExplicitDirection,
                        "Typed ports with explicit direction",
                    ),
                    (
                        crate::state::OffSheetConnectorPolicy::NamedConnectorsCompatibility,
                        "Named connectors \u{00b7} compatibility",
                    ),
                ],
                &mut connector,
                write_allowed,
            );
            changed |= setting_combo(
                right,
                "Page numbering",
                &[
                    (
                        SheetPageNumbering::StableProjectOrder,
                        "Stable project order",
                    ),
                    (SheetPageNumbering::PerPrintSet, "Per print set"),
                ],
                &mut page_numbering,
                write_allowed,
            );
            changed |= setting_combo(
                right,
                "Delete behavior",
                &[
                    (
                        SheetDeleteBehavior::BlockWhileReferenced,
                        "Block while referenced",
                    ),
                    (
                        SheetDeleteBehavior::MoveReferencesToReviewedReplacement,
                        "Move references to reviewed replacement",
                    ),
                ],
                &mut delete_behavior,
                write_allowed,
            );
            if changed && let Some(catalog) = draft.sheet_catalog_mut(&dialog.owner_key) {
                let revision = catalog.revision();
                apply_domain_result(
                    &mut dialog.error,
                    catalog.set_settings(
                        revision,
                        crate::state::SheetCatalogSettings {
                            connector_policy: connector,
                            page_numbering,
                            delete_behavior,
                        },
                    ),
                    "Sheet contract updated.",
                );
            }
        } else {
            muted_note(right, "Create a sheet to establish the sheet contract.");
        }
        concept_banner(
            right,
            "Sheet identity is stable across reordering, printing, and annotation. Moving objects creates explicit ports and preserves source-object history.",
            false,
        );
    });
    action
}

fn variants_manager_body(
    ui: &mut Ui,
    dialog: &mut DesignManagementDialogState,
    write_allowed: bool,
) -> DesignManagementBodyAction {
    let Some(draft) = dialog.draft.as_mut() else {
        return DesignManagementBodyAction::None;
    };
    let mut action = DesignManagementBodyAction::None;
    split_surface(ui, MAIN_SPLIT_LEFT_FRACTION, |left, right| {
        let variants = draft
            .variants()
            .variants()
            .iter()
            .map(|variant| {
                (
                    variant.id(),
                    variant.name().to_owned(),
                    variant.definition().parent.as_ref().map(|parent| parent.id),
                    variant.definition().overrides.clone(),
                )
            })
            .collect::<Vec<_>>();
        let names = variants
            .iter()
            .map(|(id, name, _, _)| (*id, name.clone()))
            .collect::<Vec<_>>();
        let active = draft.variants().active_variant_id();
        section_header(
            left,
            "Assembly variants",
            Some(&format!("{} governed variants", variants.len())),
        );
        paint_table_header(
            left,
            &[0.25, 0.20, 0.19, 0.12, 0.24],
            &["Variant", "Parent", "Substitutions", "DNP", "Qualification"],
        );
        if variants.is_empty() {
            empty_table_row(
                left,
                "No assembly variants. Create the first governed variant.",
            );
        } else {
            for (id, name, parent, overrides) in &variants {
                let parent_name = parent
                    .and_then(|parent| names.iter().find(|(id, _)| *id == parent))
                    .map_or_else(|| "base".to_owned(), |(_, name)| name.clone());
                let substitutions = overrides
                    .values()
                    .filter(|value| {
                        matches!(
                            value,
                            crate::state::VariantObjectOverride::Substitute { .. }
                        )
                    })
                    .count();
                let dnp = overrides
                    .values()
                    .filter(|value| {
                        matches!(
                            value,
                            crate::state::VariantObjectOverride::DoNotPopulate { .. }
                        )
                    })
                    .count();
                let values = [
                    name.clone(),
                    parent_name,
                    substitutions.to_string(),
                    dnp.to_string(),
                    if Some(*id) == active {
                        "current"
                    } else {
                        "governed"
                    }
                    .to_owned(),
                ];
                paint_table_row(left, &[0.25, 0.20, 0.19, 0.12, 0.24], &values, None);
            }
        }
        toolbar(left, |ui| {
            if Button::new("New variant\u{2026}")
                .enabled(write_allowed)
                .show(ui)
                .clicked()
            {
                action = DesignManagementBodyAction::Open(DesignManagementPage::NewVariant);
            }
            if Button::new("Compare variants\u{2026}")
                .enabled(variants.len() > 1)
                .show(ui)
                .clicked()
            {
                action = DesignManagementBodyAction::Open(DesignManagementPage::CompareVariants);
            }
            if Button::new("Substitution matrix\u{2026}")
                .enabled(write_allowed && !variants.is_empty())
                .show(ui)
                .clicked()
            {
                action = DesignManagementBodyAction::Open(DesignManagementPage::VariantMatrix);
            }
        });

        let mut selected = active;
        if setting_combo_by_id(
            right,
            "Active assembly variant",
            &names,
            &mut selected,
            write_allowed,
        ) && let Some(id) = selected
        {
            apply_domain_result(
                &mut dialog.error,
                draft.variants_mut().set_active(id),
                "Active variant changed.",
            );
        }
        let settings = draft.variants().settings().clone();
        let mut inheritance = settings.inheritance;
        let mut missing = settings.missing_replacement;
        let mut compatibility = settings.result_compatibility;
        let mut changed = false;
        changed |= setting_combo(
            right,
            "Inheritance",
            &[
                (
                    VariantInheritance::OverrideChangedObjectsOnly,
                    "Override changed objects only",
                ),
                (
                    VariantInheritance::IndependentReviewedCopy,
                    "Materialize reviewed copy",
                ),
            ],
            &mut inheritance,
            write_allowed,
        );
        changed |= setting_combo(
            right,
            "Missing replacement",
            &[
                (MissingReplacementPolicy::Block, "Block netlist and release"),
                (
                    MissingReplacementPolicy::ExplicitDoNotPopulate,
                    "DNP with explicit approval",
                ),
            ],
            &mut missing,
            write_allowed,
        );
        changed |= setting_combo(
            right,
            "Result compatibility",
            &[
                (
                    crate::state::VariantResultCompatibility::ExactVariantIdentityRequired,
                    "Exact variant identity required",
                ),
                (
                    crate::state::VariantResultCompatibility::AllowReviewedOverlay,
                    "Allow reviewed overlay",
                ),
            ],
            &mut compatibility,
            write_allowed,
        );
        if changed {
            apply_domain_result(
                &mut dialog.error,
                draft
                    .variants_mut()
                    .set_settings(crate::state::AssemblyVariantSettings {
                        inheritance,
                        missing_replacement: missing,
                        model_equivalence: settings.model_equivalence,
                        result_compatibility: compatibility,
                    }),
                "Assembly variant contract updated.",
            );
        }
        let resolved = active
            .and_then(|id| draft.variants().resolve(id).ok())
            .map(|resolved| resolved.overrides)
            .unwrap_or_default();
        let substitutions = resolved
            .values()
            .filter_map(|value| match value {
                crate::state::VariantObjectOverride::Substitute { replacement } => {
                    Some(replacement)
                }
                crate::state::VariantObjectOverride::DoNotPopulate { .. } => None,
            })
            .collect::<Vec<_>>();
        let equivalent = substitutions
            .iter()
            .filter(|replacement| replacement.port_equivalence_digest.is_some())
            .count();
        let model_reviews = substitutions
            .iter()
            .filter(|replacement| {
                replacement.qualification != crate::state::VariantQualificationState::Current
            })
            .count();
        property_row_toned(
            right,
            "Port equivalence",
            &format!("{equivalent} / {}", substitutions.len()),
            if equivalent == substitutions.len() {
                Tokens::get(right.ctx()).color.ok
            } else {
                Tokens::get(right.ctx()).color.warn
            },
        );
        property_row_toned(
            right,
            "Model sections",
            &format!("{model_reviews} reviews"),
            if model_reviews == 0 {
                Tokens::get(right.ctx()).color.ok
            } else {
                Tokens::get(right.ctx()).color.warn
            },
        );
        property_row(right, "Release evidence", "variant-bound");
    });
    action
}

fn annotation_manager_body(
    ui: &mut Ui,
    dialog: &mut DesignManagementDialogState,
    write_allowed: bool,
) -> DesignManagementBodyAction {
    let Some(draft) = dialog.draft.as_mut() else {
        return DesignManagementBodyAction::None;
    };
    let mut action = DesignManagementBodyAction::None;
    split_surface(ui, MAIN_SPLIT_LEFT_FRACTION, |left, right| {
        let policy = draft.annotation().policy().clone();
        let ranges = policy.definition().reserved_ranges.clone();
        section_header(
            left,
            "Reference-designator ownership",
            Some(if draft.annotation().journal().is_empty() {
                "preview required"
            } else {
                "mapping current"
            }),
        );
        paint_table_header(
            left,
            &[0.24, 0.19, 0.23, 0.13, 0.21],
            &["Scope", "Prefix", "Reserved range", "Used", "Collision"],
        );
        if ranges.is_empty() {
            empty_table_row(
                left,
                "No reserved ranges. Project allocation is policy-owned.",
            );
        } else {
            for range in &ranges {
                let scope = match &range.scope {
                    AnnotationRangeScope::Project => "/top".to_owned(),
                    AnnotationRangeScope::Sheet { sheet_id } => draft
                        .sheet_catalog(&dialog.owner_key)
                        .and_then(|catalog| catalog.find(*sheet_id))
                        .map_or_else(|| sheet_id.to_string(), |sheet| sheet.name().to_owned()),
                    AnnotationRangeScope::Hierarchy { path } => path.clone(),
                };
                let values = [
                    scope,
                    range.prefixes.join(", "),
                    format!("{}\u{2026}{}", range.first, range.last),
                    annotation_range_used(draft, range).to_string(),
                    "none".to_owned(),
                ];
                paint_table_row(left, &[0.24, 0.19, 0.23, 0.13, 0.21], &values, None);
            }
        }
        toolbar(left, |ui| {
            if Button::new("Preview renumbering\u{2026}")
                .enabled(write_allowed)
                .show(ui)
                .clicked()
            {
                action = DesignManagementBodyAction::Open(DesignManagementPage::RenumberPreview);
            }
            if Button::new("Edit annotation policy\u{2026}")
                .enabled(write_allowed)
                .show(ui)
                .clicked()
            {
                action = DesignManagementBodyAction::Open(DesignManagementPage::AnnotationPolicy);
            }
        });

        let mut definition = policy.definition().clone();
        let mut changed = false;
        changed |= setting_combo(
            right,
            "Reference designators",
            &[
                (
                    crate::state::ReferenceDesignatorBehavior::StableAcrossVariants,
                    "Stable across variants",
                ),
                (
                    crate::state::ReferenceDesignatorBehavior::RenumberSelectedScope,
                    "Renumber selected scope",
                ),
            ],
            &mut definition.reference_designators,
            write_allowed,
        );
        changed |= setting_combo(
            right,
            "Annotation scope",
            &[
                (
                    crate::state::DefaultAnnotationScope::WholeProject,
                    "Whole project",
                ),
                (
                    crate::state::DefaultAnnotationScope::CurrentHierarchy,
                    "Current hierarchy",
                ),
                (
                    crate::state::DefaultAnnotationScope::CurrentSheet,
                    "Current sheet",
                ),
            ],
            &mut definition.default_scope,
            write_allowed,
        );
        changed |= setting_combo(
            right,
            "Collision policy",
            &[
                (
                    AnnotationCollisionPolicy::PreviewAndBlock,
                    "Preview and block",
                ),
                (
                    AnnotationCollisionPolicy::AllocateNextFreeRange,
                    "Allocate next free range",
                ),
            ],
            &mut definition.collision_policy,
            write_allowed,
        );
        changed |= setting_combo(
            right,
            "Backannotation",
            &[
                (
                    crate::state::BackannotationPolicy::GenerateReviewedMapping,
                    "Generate reviewed mapping",
                ),
                (crate::state::BackannotationPolicy::Disabled, "Disabled"),
            ],
            &mut definition.backannotation,
            write_allowed,
        );
        if changed {
            let revision = policy.revision();
            apply_domain_result(
                &mut dialog.error,
                draft.annotation_mut().update_policy(revision, definition),
                "Annotation contract updated.",
            );
        }
        concept_banner(
            right,
            "A renumber operation never rewrites historical netlists, results, reports, or review anchors. It creates a stable old-to-new mapping.",
            true,
        );
    });
    action
}

fn hierarchy_manager_body(
    ui: &mut Ui,
    dialog: &mut DesignManagementDialogState,
    workspace: &crate::state::ProjectWorkspace,
    libraries: &crate::state::LibraryManager,
    schematic: &crate::state::SchematicState,
    write_allowed: bool,
) -> DesignManagementBodyAction {
    let Some(draft) = dialog.draft.as_mut() else {
        return DesignManagementBodyAction::None;
    };
    let mut action = DesignManagementBodyAction::None;
    let resolution =
        workspace.resolve_hierarchy_with_active(libraries, &workspace.active_view, schematic);
    split_surface(ui, MAIN_SPLIT_LEFT_FRACTION, |left, right| {
        section_header(
            left,
            "Hierarchy and view resolution",
            Some(&format!(
                "{} cells \u{00b7} {} views",
                resolution.bindings.len(),
                resolution.total_instances
            )),
        );
        paint_table_header(
            left,
            &[0.25, 0.19, 0.17, 0.22, 0.17],
            &[
                "Instance path",
                "Cell",
                "Design view",
                "Simulation view",
                "Configuration",
            ],
        );
        if resolution.bindings.is_empty() {
            empty_table_row(left, "The active hierarchy has no child view bindings.");
        } else {
            for binding in &resolution.bindings {
                let path = binding
                    .instance_paths
                    .first()
                    .cloned()
                    .unwrap_or_else(|| binding.reference.display_path());
                let configuration = resolution.configuration_id.map_or_else(
                    || "active project".to_owned(),
                    |id| {
                        workspace.configuration_sets.find(id).map_or_else(
                            || id.to_string(),
                            |configuration| configuration.name().to_owned(),
                        )
                    },
                );
                let values = [
                    path,
                    binding.reference.cell.clone(),
                    binding.reference.view.clone(),
                    binding
                        .stop_view
                        .clone()
                        .unwrap_or_else(|| binding.reference.view.clone()),
                    configuration,
                ];
                paint_table_row(left, &[0.25, 0.19, 0.17, 0.22, 0.17], &values, None);
            }
        }
        toolbar(left, |ui| {
            if Button::new("Edit hierarchy binding\u{2026}")
                .show(ui)
                .clicked()
            {
                action = DesignManagementBodyAction::OpenConfigurationBinding;
            }
            if Button::new("Configuration sets\u{2026}").show(ui).clicked() {
                action = DesignManagementBodyAction::OpenConfigurationSets;
            }
            if Button::new("Audit unresolved views\u{2026}")
                .enabled(write_allowed)
                .show(ui)
                .clicked()
            {
                action = DesignManagementBodyAction::Open(DesignManagementPage::HierarchyAudit);
            }
        });

        let mut settings = draft.hierarchy_settings().clone();
        let mut changed = false;
        changed |= setting_combo(
            right,
            "Edit-in-place depth",
            &[
                (
                    crate::state::EditInPlaceDepth::CurrentAndParentContext,
                    "Current + parent context",
                ),
                (crate::state::EditInPlaceDepth::CurrentOnly, "Current only"),
                (
                    crate::state::EditInPlaceDepth::TwoParentLevels,
                    "Two parent levels",
                ),
            ],
            &mut settings.edit_in_place_depth,
            write_allowed,
        );
        changed |= setting_combo(
            right,
            "Missing view",
            &[
                (
                    crate::state::MissingHierarchyViewPolicy::BlockNetlist,
                    "Block netlist",
                ),
                (
                    crate::state::MissingHierarchyViewPolicy::UseDeclaredFallbackOrder,
                    "Use declared fallback order",
                ),
            ],
            &mut settings.missing_view,
            write_allowed,
        );
        changed |= setting_combo(
            right,
            "Black-box policy",
            &[
                (
                    crate::state::HierarchyBlackBoxPolicy::RequireSignedBoundaryContract,
                    "Require signed boundary contract",
                ),
                (
                    crate::state::HierarchyBlackBoxPolicy::AllowProjectAbstract,
                    "Allow project abstract",
                ),
            ],
            &mut settings.black_box,
            write_allowed,
        );
        changed |= setting_combo(
            right,
            "Cycle detection",
            &[(
                crate::state::HierarchyCyclePolicy::BlockSaveAndIdentifyPath,
                "Block save and identify path",
            )],
            &mut settings.cycle_detection,
            write_allowed,
        );
        if changed {
            apply_domain_result(
                &mut dialog.error,
                draft.set_hierarchy_settings(settings),
                "Hierarchy contract updated.",
            );
        }
        let t = Tokens::get(right.ctx());
        property_row_toned(
            right,
            "Unresolved views",
            &resolution.unresolved_instances().to_string(),
            if resolution.is_valid() {
                t.color.ok
            } else {
                t.color.err
            },
        );
        let protected =
            draft
                .hierarchy_audits()
                .last()
                .map_or("not audited".to_owned(), |receipt| {
                    if receipt.passed() {
                        "qualified"
                    } else {
                        "findings"
                    }
                    .to_owned()
                });
        property_row(right, "Protected boundaries", &protected);
        let digest = resolution
            .configuration_digest
            .map_or_else(|| "active-project".to_owned(), |digest| digest.to_string());
        property_row(right, "Configuration digest", &digest);
    });
    action
}

fn annotation_range_used(
    catalog: &DesignManagementCatalog,
    range: &AnnotationReservedRange,
) -> usize {
    catalog
        .annotation()
        .journal()
        .iter()
        .flat_map(|entry| entry.mappings().values())
        .filter(|mapping| {
            range.prefixes.iter().any(|prefix| {
                mapping.new_reference.starts_with(prefix)
                    && mapping.new_reference[prefix.len()..]
                        .parse::<u32>()
                        .is_ok_and(|number| number >= range.first && number <= range.last)
            })
        })
        .count()
}

fn design_management_subflow_body(
    ui: &mut Ui,
    dialog: &mut DesignManagementDialogState,
    workspace: &crate::state::ProjectWorkspace,
    libraries: &crate::state::LibraryManager,
    schematic: &crate::state::SchematicState,
    write_allowed: bool,
) -> egui::Response {
    let page = dialog.page;
    let before = subflow_before(dialog, page);
    let subject = subflow_subject(dialog, page);
    let after = subflow_after(dialog, page);
    let mut first_response = None;
    split_surface(ui, SUBFLOW_LEFT_FRACTION, |left, right| {
        section_header(
            left,
            &format!("{} \u{00b7} semantic preview", page.code()),
            Some(&format!(
                "revision {}",
                dialog.draft.as_ref().map_or(0, |draft| draft.revision())
            )),
        );
        semantic_change_map(left, &before, &subject, &after, page.title());
        let preview_tokens = Tokens::get(left.ctx());
        left.columns(2, |columns| {
            let (preview, properties) = columns.split_at_mut(1);
            schematic_preview(&mut preview[0], page.code());
            property_row_toned(
                &mut properties[0],
                "Invariant",
                page.invariant(),
                preview_tokens.color.ok,
            );
            property_row(
                &mut properties[0],
                "Validation",
                "connectivity \u{00b7} hierarchy \u{00b7} configuration",
            );
            property_row(&mut properties[0], "Recovery", "one explicit predecessor");
            property_row_toned(
                &mut properties[0],
                "Stale results",
                "enumerated after preview",
                preview_tokens.color.warn,
            );
        });

        section_header(right, "Transaction inputs", Some("source resolved"));
        first_response = Some(subflow_fields(
            right,
            dialog,
            workspace,
            libraries,
            schematic,
            write_allowed,
        ));
        if let Some(receipt) = dialog.receipt.as_deref() {
            receipt_banner(right, receipt);
        }
    });
    first_response.unwrap_or_else(|| {
        let (_, response) = ui.allocate_exact_size(Vec2::ZERO, Sense::hover());
        response
    })
}

fn subflow_fields(
    ui: &mut Ui,
    dialog: &mut DesignManagementDialogState,
    workspace: &crate::state::ProjectWorkspace,
    _libraries: &crate::state::LibraryManager,
    _schematic: &crate::state::SchematicState,
    write_allowed: bool,
) -> egui::Response {
    let page = dialog.page;
    match page {
        DesignManagementPage::Manager => unreachable!("manager has a dedicated body"),
        DesignManagementPage::NewSheet => {
            let sheet_error = dialog
                .inputs
                .sheet_name
                .trim()
                .is_empty()
                .then_some("Sheet name is required.");
            let response = input_field(
                ui,
                "Sheet name",
                &mut dialog.inputs.sheet_name,
                "Power and references",
                sheet_error,
                "Stable, project-owned sheet name.",
            );
            let sheets = sheet_choices(dialog);
            field_combo_by_id(
                ui,
                "Insert after",
                &sheets,
                &mut dialog.inputs.sheet_insert_after,
                write_allowed,
            );
            field_combo(
                ui,
                "Template",
                &[
                    (SheetTemplate::AnalogSchematic, "Analog schematic"),
                    (
                        SheetTemplate::MixedSignalSchematic,
                        "Mixed-signal schematic",
                    ),
                    (SheetTemplate::BlankGovernedSheet, "Blank governed sheet"),
                ],
                &mut dialog.inputs.sheet_template,
                write_allowed,
            );
            field_combo(
                ui,
                "Port policy",
                &[
                    (SheetPortPolicy::TypedOffSheetPorts, "Typed off-sheet ports"),
                    (SheetPortPolicy::NoAutomaticPorts, "No automatic ports"),
                ],
                &mut dialog.inputs.sheet_port_policy,
                write_allowed,
            );
            response
        }
        DesignManagementPage::ReorderSheets => {
            let order_error = reorder_sheet_ids(dialog).err();
            let response = input_field(
                ui,
                "New order",
                &mut dialog.inputs.reorder_order_text,
                "AFE core \u{2192} Bias and reference",
                order_error.as_deref(),
                "Complete sheet names separated by arrows, in the reviewed presentation order.",
            );
            field_combo(
                ui,
                "Page numbering",
                &[
                    (
                        ReorderPageNumbering::UpdatePrintPageNumbers,
                        "Update print page numbers",
                    ),
                    (
                        ReorderPageNumbering::RetainExplicitPageNumbers,
                        "Retain explicit page numbers",
                    ),
                ],
                &mut dialog.inputs.reorder_page_numbering,
                write_allowed,
            );
            read_only_field(
                ui,
                "Cross references",
                "Update display only \u{00b7} stable IDs retained",
                "Reordering cannot mutate connectivity or stable identities.",
            );
            response
        }
        DesignManagementPage::MoveSelection => {
            let response = read_only_field(
                ui,
                "Selection",
                &dialog.selection_summary,
                "Complete selected objects captured when Design Management opened.",
            );
            let sheets = sheet_choices(dialog);
            field_combo_by_id(
                ui,
                "Destination",
                &sheets,
                &mut dialog.inputs.move_destination,
                write_allowed,
            );
            field_combo(
                ui,
                "Boundary nets",
                &[
                    (MoveBoundaryPolicy::TypedPorts, "Create typed ports"),
                    (
                        MoveBoundaryPolicy::ReviewedGlobalAliases,
                        "Create reviewed global aliases",
                    ),
                    (MoveBoundaryPolicy::Block, "Block move"),
                ],
                &mut dialog.inputs.move_boundary_policy,
                write_allowed,
            );
            field_combo(
                ui,
                "Hierarchy effect",
                &[
                    (
                        MoveHierarchyEffect::SameCell,
                        "Same cell \u{00b7} sheet move",
                    ),
                    (
                        MoveHierarchyEffect::CreateChildCell,
                        "Create child cell instead",
                    ),
                ],
                &mut dialog.inputs.move_hierarchy_effect,
                write_allowed,
            );
            response
        }
        DesignManagementPage::NewVariant => {
            let variant_error = dialog
                .inputs
                .variant_name
                .trim()
                .is_empty()
                .then_some("Variant name is required.");
            let response = input_field(
                ui,
                "Name",
                &mut dialog.inputs.variant_name,
                "Automotive high-temperature",
                variant_error,
                "Stable assembly-variant identity.",
            );
            let variants = variant_choices(dialog);
            field_combo_optional_base(
                ui,
                "Parent",
                &variants,
                &mut dialog.inputs.variant_parent,
                "Base design",
                write_allowed,
            );
            field_combo(
                ui,
                "Inheritance",
                &[
                    (
                        VariantInheritance::OverrideChangedObjectsOnly,
                        "Override changed objects only",
                    ),
                    (
                        VariantInheritance::IndependentReviewedCopy,
                        "Independent reviewed copy",
                    ),
                ],
                &mut dialog.inputs.variant_inheritance,
                write_allowed,
            );
            field_combo(
                ui,
                "Qualification",
                &[
                    (
                        VariantQualificationPlan::InvalidateAffectedTests,
                        "Invalidate affected tests",
                    ),
                    (
                        VariantQualificationPlan::CreateEmptyQualificationPlan,
                        "Create empty qualification plan",
                    ),
                ],
                &mut dialog.inputs.variant_qualification,
                write_allowed,
            );
            response
        }
        DesignManagementPage::CompareVariants => {
            let variants = variant_choices(dialog);
            let response = field_combo_by_id(
                ui,
                "Reference",
                &variants,
                &mut dialog.inputs.compare_reference,
                true,
            );
            field_combo_by_id(
                ui,
                "Comparison",
                &variants,
                &mut dialog.inputs.compare_target,
                true,
            );
            field_combo(
                ui,
                "Difference classes",
                &[
                    (
                        VariantDifferenceClasses::DevicesValuesDnpModels,
                        "Devices + values + DNP + models",
                    ),
                    (
                        VariantDifferenceClasses::ConnectivityOnly,
                        "Connectivity only",
                    ),
                ],
                &mut dialog.inputs.difference_classes,
                true,
            );
            response
        }
        DesignManagementPage::VariantMatrix => {
            let response = field_combo(
                ui,
                "Scope",
                &[
                    (
                        VariantMatrixScope::AllControlledInstances,
                        "All variant-controlled instances",
                    ),
                    (VariantMatrixScope::CurrentHierarchy, "Current hierarchy"),
                ],
                &mut dialog.inputs.matrix_scope,
                write_allowed,
            );
            field_combo(
                ui,
                "Missing replacement",
                &[
                    (MissingReplacementPolicy::Block, "Block"),
                    (
                        MissingReplacementPolicy::ExplicitDoNotPopulate,
                        "Explicit DNP",
                    ),
                ],
                &mut dialog.inputs.missing_replacement,
                write_allowed,
            );
            field_combo(
                ui,
                "Model equivalence",
                &[
                    (
                        ModelEquivalencePolicy::RequireQualifiedReplacement,
                        "Require qualified replacement",
                    ),
                    (
                        ModelEquivalencePolicy::AllowReviewCandidate,
                        "Allow review candidate",
                    ),
                ],
                &mut dialog.inputs.model_equivalence,
                write_allowed,
            );
            response
        }
        DesignManagementPage::RenumberPreview => {
            let response = field_combo(
                ui,
                "Scope",
                &[
                    (RenumberScopeChoice::WholeProject, "Whole project"),
                    (RenumberScopeChoice::CurrentHierarchy, "Current hierarchy"),
                    (RenumberScopeChoice::CurrentSheet, "Current sheet"),
                ],
                &mut dialog.inputs.renumber_scope,
                write_allowed,
            );
            field_combo(
                ui,
                "Order",
                &[
                    (
                        RenumberOrder::HierarchyThenCoordinates,
                        "Hierarchy then coordinates",
                    ),
                    (
                        RenumberOrder::SheetThenCoordinates,
                        "Sheet then coordinates",
                    ),
                    (RenumberOrder::ConnectivityOrder, "Connectivity order"),
                ],
                &mut dialog.inputs.renumber_order,
                write_allowed,
            );
            field_combo(
                ui,
                "Protected references",
                &[
                    (
                        ProtectedReferencePolicy::RetainLockedAndExternalIds,
                        "Retain locked and external IDs",
                    ),
                    (
                        ProtectedReferencePolicy::IncludeAfterReview,
                        "Include after review",
                    ),
                ],
                &mut dialog.inputs.protected_references,
                write_allowed,
            );
            response
        }
        DesignManagementPage::AnnotationPolicy => {
            let range_error = parse_reserved_ranges(&dialog.inputs.reserved_ranges, dialog).err();
            let response = field_combo(
                ui,
                "Prefix allocation",
                &[
                    (
                        AnnotationPrefixAllocation::ByDeviceFamily,
                        "By device family",
                    ),
                    (AnnotationPrefixAllocation::BySheet, "By sheet"),
                    (AnnotationPrefixAllocation::ByHierarchy, "By hierarchy"),
                ],
                &mut dialog.inputs.prefix_allocation,
                write_allowed,
            );
            input_field(
                ui,
                "Reserved ranges",
                &mut dialog.inputs.reserved_ranges,
                "R,C 1\u{2026}399; U 400\u{2026}599",
                range_error.as_deref(),
                "Project-owned prefix ranges separated by semicolons.",
            );
            field_combo(
                ui,
                "Imported IDs",
                &[
                    (
                        ImportedReferencePolicy::PreserveWithSourceMap,
                        "Preserve with source map",
                    ),
                    (
                        ImportedReferencePolicy::NormalizeAfterReview,
                        "Normalize after review",
                    ),
                ],
                &mut dialog.inputs.imported_ids,
                write_allowed,
            );
            response
        }
        DesignManagementPage::HierarchyAudit => {
            let configurations = workspace
                .configuration_sets
                .configurations()
                .iter()
                .map(|configuration| (configuration.id(), configuration.name().to_owned()))
                .collect::<Vec<_>>();
            let response = field_combo_optional_base(
                ui,
                "Configuration",
                &configurations,
                &mut dialog.inputs.audit_configuration,
                "active project",
                true,
            );
            field_combo(
                ui,
                "View checks",
                &[
                    (
                        HierarchyViewChecks::AllDeclaredFallbacks,
                        "All declared fallbacks",
                    ),
                    (HierarchyViewChecks::SelectedHierarchy, "Selected hierarchy"),
                ],
                &mut dialog.inputs.audit_view_checks,
                true,
            );
            field_combo(
                ui,
                "Protected boundaries",
                &[
                    (
                        ProtectedBoundaryChecks::SignaturesAndPins,
                        "Validate signatures and pins",
                    ),
                    (ProtectedBoundaryChecks::PinsOnly, "Pins only"),
                ],
                &mut dialog.inputs.audit_protected_boundaries,
                true,
            );
            response
        }
    }
}

fn split_surface(ui: &mut Ui, left_fraction: f32, content: impl FnOnce(&mut Ui, &mut Ui)) {
    let t = Tokens::get(ui.ctx());
    let available = ui.available_width().max(1.0);
    let stacked = available < SPLIT_BREAKPOINT;
    let origin = ui.next_widget_position();
    let padding = tokens::SP_5;
    let extent = 10_000.0;

    if stacked {
        ui.columns(2, |columns| {
            let (left, right) = columns.split_at_mut(1);
            content(&mut left[0], &mut right[0]);
        });
        return;
    }

    let (left_width, right_width) =
        if (left_fraction - MAIN_SPLIT_LEFT_FRACTION).abs() < f32::EPSILON {
            main_split_widths(available)
        } else {
            subflow_split_widths(available)
        };
    let divider_x = origin.x + left_width;
    let left_rect = egui::Rect::from_min_size(
        origin + vec2(padding, padding),
        vec2((left_width - padding * 2.0).max(1.0), extent),
    );
    let right_rect = egui::Rect::from_min_size(
        egui::pos2(divider_x + padding, origin.y + padding),
        vec2((right_width - padding * 2.0).max(1.0), extent),
    );
    let mut left_ui = ui.new_child(
        egui::UiBuilder::new()
            .max_rect(left_rect)
            .layout(Layout::top_down(Align::Min)),
    );
    let mut right_ui = ui.new_child(
        egui::UiBuilder::new()
            .max_rect(right_rect)
            .layout(Layout::top_down(Align::Min)),
    );
    content(&mut left_ui, &mut right_ui);
    let height = left_ui
        .min_rect()
        .height()
        .max(right_ui.min_rect().height())
        + padding * 2.0;
    let surface = egui::Rect::from_min_size(origin, vec2(available, height.max(80.0)));
    ui.painter().rect_stroke(
        surface,
        0.0,
        Stroke::new(1.0, t.color.border_strong),
        egui::StrokeKind::Inside,
    );
    ui.painter().vline(
        divider_x,
        surface.y_range(),
        Stroke::new(1.0, t.color.border_strong),
    );
    ui.advance_cursor_after_rect(surface);
}

fn concept_banner(ui: &mut Ui, text: &str, warning: bool) {
    let t = Tokens::get(ui.ctx());
    Frame::NONE
        .fill(if warning {
            t.color.accent_dim
        } else {
            t.color.bg_panel
        })
        .stroke(Stroke::new(
            1.0,
            if warning {
                t.color.warn
            } else {
                t.color.border
            },
        ))
        .inner_margin(Margin::same(10))
        .show(ui, |ui| {
            ui.label(
                egui::RichText::new(text)
                    .font(theme::sans(tokens::FS_1, FontWeight::Regular))
                    .color(if warning {
                        t.color.text
                    } else {
                        t.color.text_dim
                    }),
            );
        });
}

fn receipt_banner(ui: &mut Ui, text: &str) {
    let t = Tokens::get(ui.ctx());
    Frame::NONE
        .fill(t.color.bg_panel)
        .stroke(Stroke::new(1.0, t.color.ok))
        .inner_margin(Margin::same(10))
        .show(ui, |ui| {
            ui.label(
                egui::RichText::new(format!("Committed · {text}"))
                    .font(theme::mono(tokens::FS_0, FontWeight::Regular))
                    .color(t.color.ok),
            );
        });
}

fn toolbar(ui: &mut Ui, content: impl FnOnce(&mut Ui)) {
    let t = Tokens::get(ui.ctx());
    Frame::NONE
        .fill(t.color.bg_panel)
        .inner_margin(Margin {
            left: 8,
            right: 8,
            top: 7,
            bottom: 7,
        })
        .show(ui, |ui| {
            ui.horizontal_wrapped(content);
        });
}

fn paint_table_header(ui: &mut Ui, weights: &[f32], labels: &[&str]) {
    let values = labels
        .iter()
        .map(|label| (*label).to_owned())
        .collect::<Vec<_>>();
    paint_table_cells(ui, weights, &values, true, None);
}

fn paint_table_row(ui: &mut Ui, weights: &[f32], values: &[String], tone: Option<egui::Color32>) {
    paint_table_cells(ui, weights, values, false, tone);
}

fn paint_table_cells(
    ui: &mut Ui,
    weights: &[f32],
    values: &[String],
    header: bool,
    tone: Option<egui::Color32>,
) {
    let t = Tokens::get(ui.ctx());
    let height = if header { 27.0 } else { 29.0 };
    let (rect, response) =
        ui.allocate_exact_size(vec2(ui.available_width(), height), Sense::hover());
    ui.painter().rect_filled(
        rect,
        0.0,
        if header {
            t.color.bg_panel_2
        } else if response.hovered() {
            t.color.bg_hover
        } else {
            t.color.bg_inset
        },
    );
    ui.painter().hline(
        rect.x_range(),
        rect.bottom(),
        Stroke::new(1.0, t.color.border),
    );
    let total = weights.iter().sum::<f32>().max(f32::EPSILON);
    let mut x = rect.left();
    for (index, value) in values.iter().enumerate() {
        let weight = weights.get(index).copied().unwrap_or(1.0);
        let width = if index + 1 == values.len() {
            rect.right() - x
        } else {
            rect.width() * weight / total
        };
        let cell = egui::Rect::from_min_max(
            egui::pos2(x, rect.top()),
            egui::pos2((x + width).min(rect.right()), rect.bottom()),
        );
        if index > 0 {
            ui.painter().vline(
                cell.left(),
                cell.y_range(),
                Stroke::new(1.0, t.color.border),
            );
        }
        ui.painter()
            .with_clip_rect(cell.shrink2(vec2(7.0, 2.0)))
            .text(
                cell.left_center() + vec2(7.0, 0.0),
                egui::Align2::LEFT_CENTER,
                value,
                if header {
                    theme::sans(tokens::FS_0, FontWeight::SemiBold)
                } else {
                    theme::mono(tokens::FS_0, FontWeight::Regular)
                },
                tone.unwrap_or(if header {
                    t.color.text_dim
                } else {
                    t.color.text
                }),
            );
        x += width;
    }
}

fn empty_table_row(ui: &mut Ui, text: &str) {
    paint_table_row(
        ui,
        &[1.0],
        &[text.to_owned()],
        Some(Tokens::get(ui.ctx()).color.text_dim),
    );
}

fn muted_note(ui: &mut Ui, text: &str) {
    ui.label(
        egui::RichText::new(text)
            .font(theme::sans(tokens::FS_1, FontWeight::Regular))
            .color(Tokens::get(ui.ctx()).color.text_dim),
    );
}

fn combo_field<T: Copy + PartialEq>(
    ui: &mut Ui,
    id_namespace: &'static str,
    label: &str,
    options: &[(T, &str)],
    selected: &mut T,
    enabled: bool,
) -> egui::Response {
    let t = Tokens::get(ui.ctx());
    ui.label(
        egui::RichText::new(label)
            .font(theme::sans(tokens::FS_0, FontWeight::Regular))
            .color(t.color.text_dim),
    );
    let selected_text = options
        .iter()
        .find(|(value, _)| value == selected)
        .map_or("Select", |(_, text)| *text);
    ui.add_enabled_ui(enabled, |ui| {
        egui::ComboBox::from_id_salt((id_namespace, label))
            .selected_text(selected_text)
            .width(ui.available_width())
            .show_ui(ui, |ui| {
                for (value, text) in options {
                    ui.selectable_value(selected, *value, *text);
                }
            })
            .response
    })
    .inner
}

fn setting_combo<T: Copy + PartialEq>(
    ui: &mut Ui,
    label: &str,
    options: &[(T, &str)],
    selected: &mut T,
    enabled: bool,
) -> bool {
    combo_field(
        ui,
        "design-management-setting",
        label,
        options,
        selected,
        enabled,
    )
    .changed()
}

fn field_combo<T: Copy + PartialEq>(
    ui: &mut Ui,
    label: &str,
    options: &[(T, &str)],
    selected: &mut T,
    enabled: bool,
) -> egui::Response {
    combo_field(
        ui,
        "design-management-field",
        label,
        options,
        selected,
        enabled,
    )
}

fn id_combo<T: Copy + PartialEq + std::fmt::Display>(
    ui: &mut Ui,
    id_namespace: &'static str,
    label: &str,
    options: &[(T, String)],
    selected: &mut Option<T>,
    base: Option<&str>,
    enabled: bool,
) -> egui::Response {
    let t = Tokens::get(ui.ctx());
    ui.label(
        egui::RichText::new(label)
            .font(theme::sans(tokens::FS_0, FontWeight::Regular))
            .color(t.color.text_dim),
    );
    let selected_text = selected
        .and_then(|id| options.iter().find(|(candidate, _)| *candidate == id))
        .map_or_else(
            || base.unwrap_or("Select").to_owned(),
            |(_, name)| name.clone(),
        );
    ui.add_enabled_ui(enabled, |ui| {
        egui::ComboBox::from_id_salt((id_namespace, label))
            .selected_text(selected_text)
            .width(ui.available_width())
            .show_ui(ui, |ui| {
                if let Some(base) = base {
                    ui.selectable_value(selected, None, base);
                }
                for (id, name) in options {
                    ui.selectable_value(selected, Some(*id), name);
                }
            })
            .response
    })
    .inner
}

fn setting_combo_by_id<T: Copy + PartialEq + std::fmt::Display>(
    ui: &mut Ui,
    label: &str,
    options: &[(T, String)],
    selected: &mut Option<T>,
    enabled: bool,
) -> bool {
    id_combo(
        ui,
        "design-management-setting-id",
        label,
        options,
        selected,
        None,
        enabled,
    )
    .changed()
}

fn field_combo_by_id<T: Copy + PartialEq + std::fmt::Display>(
    ui: &mut Ui,
    label: &str,
    options: &[(T, String)],
    selected: &mut Option<T>,
    enabled: bool,
) -> egui::Response {
    id_combo(
        ui,
        "design-management-field-id",
        label,
        options,
        selected,
        None,
        enabled,
    )
}

fn field_combo_optional_base<T: Copy + PartialEq + std::fmt::Display>(
    ui: &mut Ui,
    label: &str,
    options: &[(T, String)],
    selected: &mut Option<T>,
    base: &str,
    enabled: bool,
) -> egui::Response {
    id_combo(
        ui,
        "design-management-field-base",
        label,
        options,
        selected,
        Some(base),
        enabled,
    )
}

fn apply_domain_result<T, E: ToString>(
    error: &mut Option<String>,
    result: Result<T, E>,
    _receipt: &str,
) {
    *error = result.err().map(|error| error.to_string());
}

fn sheet_choices(dialog: &DesignManagementDialogState) -> Vec<(SheetId, String)> {
    dialog
        .draft
        .as_ref()
        .and_then(|draft| draft.sheet_catalog(&dialog.owner_key))
        .map(|catalog| {
            catalog
                .sheets()
                .iter()
                .map(|sheet| (sheet.id(), sheet.name().to_owned()))
                .collect()
        })
        .unwrap_or_default()
}

fn variant_choices(dialog: &DesignManagementDialogState) -> Vec<(AssemblyVariantId, String)> {
    dialog
        .draft
        .as_ref()
        .map(|draft| {
            draft
                .variants()
                .variants()
                .iter()
                .map(|variant| (variant.id(), variant.name().to_owned()))
                .collect()
        })
        .unwrap_or_default()
}

fn reorder_sheet_ids(dialog: &DesignManagementDialogState) -> Result<Vec<SheetId>, String> {
    let catalog = dialog
        .draft
        .as_ref()
        .and_then(|draft| draft.sheet_catalog(&dialog.owner_key))
        .ok_or_else(|| "No governed sheet catalog is available.".to_owned())?;
    let names = dialog
        .inputs
        .reorder_order_text
        .split(['→', '>'])
        .map(str::trim)
        .filter(|name| !name.is_empty())
        .collect::<Vec<_>>();
    if names.len() != catalog.sheets().len() {
        return Err("List every sheet exactly once in the reviewed order.".to_owned());
    }
    let mut result = Vec::with_capacity(names.len());
    for name in names {
        let id = catalog
            .sheets()
            .iter()
            .find(|sheet| sheet.name().eq_ignore_ascii_case(name))
            .map(|sheet| sheet.id())
            .ok_or_else(|| format!("Unknown sheet `{name}`."))?;
        if result.contains(&id) {
            return Err(format!("Sheet `{name}` appears more than once."));
        }
        result.push(id);
    }
    Ok(result)
}

fn parse_reserved_ranges(
    text: &str,
    dialog: &DesignManagementDialogState,
) -> Result<Vec<AnnotationReservedRange>, String> {
    if text.trim().eq_ignore_ascii_case("Project-owned ranges") {
        return Ok(dialog
            .draft
            .as_ref()
            .map(|draft| {
                draft
                    .annotation()
                    .policy()
                    .definition()
                    .reserved_ranges
                    .clone()
            })
            .unwrap_or_default());
    }
    let mut ranges = Vec::new();
    for segment in text
        .split(';')
        .map(str::trim)
        .filter(|value| !value.is_empty())
    {
        let split = segment
            .char_indices()
            .find(|(_, character)| character.is_ascii_digit())
            .map(|(index, _)| index)
            .ok_or_else(|| format!("Range `{segment}` is missing a numeric interval."))?;
        let prefixes = segment[..split]
            .trim()
            .trim_end_matches(',')
            .split(',')
            .map(str::trim)
            .filter(|prefix| !prefix.is_empty())
            .map(str::to_owned)
            .collect::<Vec<_>>();
        if prefixes.is_empty() {
            return Err(format!("Range `{segment}` is missing a device prefix."));
        }
        let interval = segment[split..].trim();
        let bounds = interval
            .split(['…', '-', '–'])
            .map(str::trim)
            .collect::<Vec<_>>();
        if bounds.len() != 2 {
            return Err(format!(
                "Range `{segment}` must have a first and last number."
            ));
        }
        let first = bounds[0]
            .parse::<u32>()
            .map_err(|_| format!("Range `{segment}` has an invalid first number."))?;
        let last = bounds[1]
            .parse::<u32>()
            .map_err(|_| format!("Range `{segment}` has an invalid last number."))?;
        ranges.push(AnnotationReservedRange {
            scope: AnnotationRangeScope::Project,
            prefixes,
            first,
            last,
        });
    }
    if ranges.is_empty() {
        return Err("Enter at least one reserved range.".to_owned());
    }
    Ok(ranges)
}

fn semantic_change_map(ui: &mut Ui, before: &str, subject: &str, after: &str, operation: &str) {
    let t = Tokens::get(ui.ctx());
    ui.horizontal(|ui| {
        for (index, (label, value, detail)) in [
            ("Current source", before, "unchanged until commit"),
            ("Proposed transaction", subject, operation),
            ("Result", after, "new stable revision"),
        ]
        .into_iter()
        .enumerate()
        {
            Frame::NONE
                .fill(t.color.bg_inset)
                .stroke(Stroke::new(1.0, t.color.border))
                .inner_margin(Margin::same(8))
                .show(ui, |ui| {
                    ui.set_min_width(116.0);
                    ui.label(
                        egui::RichText::new(label)
                            .size(tokens::FS_0)
                            .color(t.color.text_dim),
                    );
                    ui.label(
                        egui::RichText::new(value)
                            .size(tokens::FS_1)
                            .color(t.color.text),
                    );
                    ui.label(
                        egui::RichText::new(detail)
                            .size(tokens::FS_0)
                            .color(t.color.text_dim),
                    );
                });
            if index < 2 {
                ui.label(egui::RichText::new("→").color(t.color.accent));
            }
        }
    });
}

fn schematic_preview(ui: &mut Ui, code: &str) {
    let t = Tokens::get(ui.ctx());
    let (rect, _) = ui.allocate_exact_size(vec2(ui.available_width(), 150.0), Sense::hover());
    ui.painter().rect_filled(rect, 0.0, t.color.canvas_bg);
    ui.painter().rect_stroke(
        rect,
        0.0,
        Stroke::new(1.0, t.color.border),
        egui::StrokeKind::Inside,
    );
    for x in (0..16).map(|index| rect.left() + 8.0 + index as f32 * 24.0) {
        for y in (0..7).map(|index| rect.top() + 8.0 + index as f32 * 24.0) {
            if rect.contains(egui::pos2(x, y)) {
                ui.painter()
                    .circle_filled(egui::pos2(x, y), 1.0, t.color.canvas_grid);
            }
        }
    }
    let center = rect.center();
    ui.painter().line_segment(
        [center - vec2(94.0, 0.0), center + vec2(94.0, 0.0)],
        Stroke::new(1.5, t.color.wire),
    );
    let opamp_center = center - vec2(42.0, 0.0);
    let opamp = [
        opamp_center + vec2(-24.0, -25.0),
        opamp_center + vec2(-24.0, 25.0),
        opamp_center + vec2(24.0, 0.0),
        opamp_center + vec2(-24.0, -25.0),
    ];
    ui.painter().add(egui::Shape::line(
        opamp.to_vec(),
        Stroke::new(1.5, t.color.symbol),
    ));
    ui.painter().text(
        opamp_center + vec2(-15.0, -10.0),
        egui::Align2::CENTER_CENTER,
        "−",
        theme::sans(tokens::FS_1, FontWeight::Regular),
        t.color.symbol,
    );
    ui.painter().text(
        opamp_center + vec2(-15.0, 10.0),
        egui::Align2::CENTER_CENTER,
        "+",
        theme::sans(tokens::FS_1, FontWeight::Regular),
        t.color.symbol,
    );
    let resistor_center = center + vec2(56.0, 0.0);
    let resistor = [
        resistor_center + vec2(-28.0, 0.0),
        resistor_center + vec2(-21.0, -9.0),
        resistor_center + vec2(-14.0, 9.0),
        resistor_center + vec2(-7.0, -9.0),
        resistor_center + vec2(0.0, 9.0),
        resistor_center + vec2(7.0, -9.0),
        resistor_center + vec2(14.0, 9.0),
        resistor_center + vec2(21.0, -9.0),
        resistor_center + vec2(28.0, 0.0),
    ];
    ui.painter().add(egui::Shape::line(
        resistor.to_vec(),
        Stroke::new(1.5, t.color.symbol),
    ));
    ui.painter().text(
        rect.center_bottom() - vec2(0.0, 14.0),
        egui::Align2::CENTER_BOTTOM,
        code,
        theme::mono(tokens::FS_1, FontWeight::SemiBold),
        t.color.accent,
    );
}

fn subflow_before(dialog: &DesignManagementDialogState, page: DesignManagementPage) -> String {
    match page {
        DesignManagementPage::NewSheet
        | DesignManagementPage::ReorderSheets
        | DesignManagementPage::MoveSelection => dialog
            .draft
            .as_ref()
            .and_then(|draft| draft.sheet_catalog(&dialog.owner_key))
            .map_or_else(
                || "no sheet catalog".to_owned(),
                |catalog| format!("sheet revision {}", catalog.revision()),
            ),
        DesignManagementPage::NewVariant
        | DesignManagementPage::CompareVariants
        | DesignManagementPage::VariantMatrix => format!(
            "{} governed variants",
            dialog
                .draft
                .as_ref()
                .map_or(0, |draft| draft.variants().variants().len())
        ),
        DesignManagementPage::RenumberPreview | DesignManagementPage::AnnotationPolicy => format!(
            "annotation policy {}",
            dialog
                .draft
                .as_ref()
                .map_or(0, |draft| draft.annotation().policy().revision())
        ),
        DesignManagementPage::HierarchyAudit => "active hierarchy resolution".to_owned(),
        DesignManagementPage::Manager => "working revision".to_owned(),
    }
}

fn subflow_subject(dialog: &DesignManagementDialogState, page: DesignManagementPage) -> String {
    match page {
        DesignManagementPage::NewSheet => dialog.inputs.sheet_name.clone(),
        DesignManagementPage::ReorderSheets => dialog.inputs.reorder_order_text.clone(),
        DesignManagementPage::MoveSelection => dialog.selection_summary.clone(),
        DesignManagementPage::NewVariant => dialog.inputs.variant_name.clone(),
        DesignManagementPage::CompareVariants => "exact variant delta".to_owned(),
        DesignManagementPage::VariantMatrix => "governed override matrix".to_owned(),
        DesignManagementPage::RenumberPreview => "stable old-to-new mapping".to_owned(),
        DesignManagementPage::AnnotationPolicy => dialog.inputs.reserved_ranges.clone(),
        DesignManagementPage::HierarchyAudit => "configuration-bound audit".to_owned(),
        DesignManagementPage::Manager => "reviewed aggregate".to_owned(),
    }
}

fn subflow_after(dialog: &DesignManagementDialogState, _page: DesignManagementPage) -> String {
    format!(
        "draft revision {}",
        dialog
            .draft
            .as_ref()
            .map_or(0, DesignManagementCatalog::revision)
    )
}

impl RSpiceApp {
    fn commit_design_management_subflow(
        &mut self,
        page: DesignManagementPage,
    ) -> Result<(), String> {
        validate_design_management_page(&self.state, page)?;
        if page == DesignManagementPage::MoveSelection
            && self
                .state
                .dialogs
                .design_management
                .inputs
                .move_hierarchy_effect
                == MoveHierarchyEffect::CreateChildCell
        {
            if self.state.dialogs.design_management.dirty() {
                return Err(
                    "Apply the pending Design Management draft before transferring this selection to Create hierarchy."
                        .to_owned(),
                );
            }
            if !super::app_create_hierarchy_dialog::create_hierarchy_available(&self.state) {
                return Err(
                    "Create hierarchy requires the retained complete instance-only selection and a writable active schematic."
                        .to_owned(),
                );
            }
            self.state.dialogs.design_management.close_and_discard();
            super::close_design_management_dialog_route(&mut self.state);
            super::app_create_hierarchy_dialog::open_create_hierarchy_dialog(&mut self.state);
            return Ok(());
        }
        let owner_key = self.state.dialogs.design_management.owner_key.clone();
        let inputs = self.state.dialogs.design_management.inputs.clone();
        let selection = self
            .state
            .dialogs
            .design_management
            .selection_object_ids
            .clone();
        let all_objects = self.state.dialogs.design_management.all_object_ids.clone();

        let (tab, receipt) = match page {
            DesignManagementPage::Manager => {
                return Err("The manager publishes through its reviewed primary action.".to_owned());
            }
            DesignManagementPage::NewSheet => {
                let draft = self
                    .state
                    .dialogs
                    .design_management
                    .draft
                    .as_mut()
                    .ok_or_else(|| "The Design Management draft is unavailable.".to_owned())?;
                let catalog = draft
                    .ensure_sheet_catalog(&owner_key)
                    .map_err(|error| error.to_string())?;
                let used_pages = catalog
                    .sheets()
                    .iter()
                    .filter_map(|sheet| sheet.definition().explicit_page_number)
                    .collect::<BTreeSet<_>>();
                let explicit_page_number =
                    (1..=u32::MAX)
                        .find(|page| !used_pages.contains(page))
                        .ok_or_else(|| "No free sheet page number remains.".to_owned())?;
                let id = catalog
                    .create_sheet(
                        SheetDefinition {
                            name: inputs.sheet_name.trim().to_owned(),
                            template: inputs.sheet_template,
                            port_policy: inputs.sheet_port_policy,
                            explicit_page_number: Some(explicit_page_number),
                        },
                        inputs.sheet_insert_after,
                    )
                    .map_err(|error| error.to_string())?;
                if catalog.object_assignments().is_empty() && !all_objects.is_empty() {
                    catalog
                        .assign_objects(catalog.revision(), id, all_objects)
                        .map_err(|error| error.to_string())?;
                }
                (
                    DesignManagementTab::Sheets,
                    format!(
                        "Created stable sheet `{}` · {}",
                        inputs.sheet_name.trim(),
                        id
                    ),
                )
            }
            DesignManagementPage::ReorderSheets => {
                let ordered_ids = reorder_sheet_ids(&self.state.dialogs.design_management)?;
                let draft = self
                    .state
                    .dialogs
                    .design_management
                    .draft
                    .as_mut()
                    .ok_or_else(|| "The Design Management draft is unavailable.".to_owned())?;
                let catalog = draft
                    .sheet_catalog_mut(&owner_key)
                    .ok_or_else(|| "No governed sheet catalog is available.".to_owned())?;
                let revision = catalog
                    .reorder(
                        catalog.revision(),
                        ordered_ids,
                        inputs.reorder_page_numbering,
                        ReorderCrossReferences::UpdateDisplayOnlyStableIdsRetained,
                    )
                    .map_err(|error| error.to_string())?;
                (
                    DesignManagementTab::Sheets,
                    format!("Applied reviewed sheet order · catalog revision {revision}"),
                )
            }
            DesignManagementPage::MoveSelection => {
                let destination = inputs
                    .move_destination
                    .ok_or_else(|| "Destination sheet is required.".to_owned())?;
                let connectivity =
                    super::app_create_hierarchy_dialog::selected_component_sheet_move_plan(
                        &self.state,
                    )?;
                if connectivity.source_component_ids != selection {
                    return Err(
                        "The retained selection changed while the sheet move was being reviewed. Close and reopen Design Management."
                            .to_owned(),
                    );
                }
                let draft = self
                    .state
                    .dialogs
                    .design_management
                    .draft
                    .as_mut()
                    .ok_or_else(|| "The Design Management draft is unavailable.".to_owned())?;
                let catalog = draft
                    .sheet_catalog_mut(&owner_key)
                    .ok_or_else(|| "No governed sheet catalog is available.".to_owned())?;
                let active = catalog.active_sheet_id();
                catalog
                    .reconcile_object_assignments(catalog.revision(), all_objects, active)
                    .map_err(|error| error.to_string())?;
                let source = connectivity
                    .source_component_ids
                    .first()
                    .and_then(|object_id| catalog.sheet_for_object(*object_id))
                    .ok_or_else(|| {
                        "The selected instances have no governed source-sheet identity.".to_owned()
                    })?;
                if inputs.move_boundary_policy == MoveBoundaryPolicy::ReviewedGlobalAliases {
                    let mut settings = catalog.settings().clone();
                    settings.connector_policy =
                        crate::state::OffSheetConnectorPolicy::NamedConnectorsCompatibility;
                    if settings != *catalog.settings() {
                        catalog
                            .set_settings(catalog.revision(), settings)
                            .map_err(|error| error.to_string())?;
                    }
                }
                let boundary_ports = connectivity
                    .boundaries
                    .into_iter()
                    .map(|boundary| CrossSheetPortDefinition {
                        net_name: boundary.net_name,
                        first: CrossSheetPortEndpoint {
                            sheet_id: source,
                            anchor: CrossSheetPortAnchor::WirePoint {
                                wire_id: boundary.stationary_wire_id,
                                point: boundary.stationary_point,
                            },
                        },
                        second: CrossSheetPortEndpoint {
                            sheet_id: destination,
                            anchor: CrossSheetPortAnchor::ComponentTerminal {
                                component_id: boundary.moved_component_id,
                                terminal_name: boundary.moved_terminal_name,
                            },
                        },
                        direction: match boundary.direction {
                            PortDirection::In => CrossSheetPortDirection::Input,
                            PortDirection::Out => CrossSheetPortDirection::Output,
                            PortDirection::InOut => CrossSheetPortDirection::InOut,
                            PortDirection::Supply => CrossSheetPortDirection::Supply,
                        },
                        signal_type: match (boundary.direction, boundary.discipline) {
                            (PortDirection::Supply, _) => CrossSheetSignalType::Power,
                            (_, PortDiscipline::Logic) => CrossSheetSignalType::Logic,
                            _ => CrossSheetSignalType::Analog,
                        },
                        discipline: match boundary.discipline {
                            PortDiscipline::Electrical => CrossSheetDiscipline::Electrical,
                            PortDiscipline::Logic => CrossSheetDiscipline::Logic,
                            PortDiscipline::Wreal => CrossSheetDiscipline::Wreal,
                            PortDiscipline::Thermal => CrossSheetDiscipline::Thermal,
                        },
                    })
                    .collect::<Vec<_>>();
                let boundary_resolution = if boundary_ports.is_empty() {
                    MoveBoundaryResolution::VerifiedNoBoundaryNets
                } else {
                    MoveBoundaryResolution::ExplicitPorts {
                        ports: boundary_ports,
                    }
                };
                let moved = catalog
                    .move_selection(MoveSelectionRequest {
                        expected_catalog_revision: catalog.revision(),
                        object_ids: connectivity.moved_object_ids,
                        destination_sheet_id: destination,
                        boundary_resolution,
                    })
                    .map_err(|error| error.to_string())?;
                (
                    DesignManagementTab::Sheets,
                    format!(
                        "Moved {} objects · {} explicit boundary ports · catalog revision {}",
                        moved.object_ids.len(),
                        moved.created_port_ids.len(),
                        moved.catalog_revision
                    ),
                )
            }
            DesignManagementPage::NewVariant => {
                let draft = self
                    .state
                    .dialogs
                    .design_management
                    .draft
                    .as_mut()
                    .ok_or_else(|| "The Design Management draft is unavailable.".to_owned())?;
                let id = draft
                    .variants_mut()
                    .create(AssemblyVariantDraft {
                        name: inputs.variant_name.trim().to_owned(),
                        parent_id: inputs.variant_parent,
                        inheritance: inputs.variant_inheritance,
                        qualification_plan: inputs.variant_qualification,
                        overrides: BTreeMap::new(),
                    })
                    .map_err(|error| error.to_string())?;
                (
                    DesignManagementTab::Variants,
                    format!(
                        "Created governed assembly variant `{}` · {id}",
                        inputs.variant_name.trim()
                    ),
                )
            }
            DesignManagementPage::CompareVariants => {
                let reference = inputs
                    .compare_reference
                    .ok_or_else(|| "Reference variant is required.".to_owned())?;
                let comparison = inputs
                    .compare_target
                    .ok_or_else(|| "Comparison variant is required.".to_owned())?;
                let result = self
                    .state
                    .dialogs
                    .design_management
                    .draft
                    .as_ref()
                    .ok_or_else(|| "The Design Management draft is unavailable.".to_owned())?
                    .variants()
                    .compare(reference, comparison)
                    .map_err(|error| error.to_string())?;
                let (difference_count, class_label) = match inputs.difference_classes {
                    VariantDifferenceClasses::DevicesValuesDnpModels => {
                        (result.differences.len(), "device/value/DNP/model")
                    }
                    VariantDifferenceClasses::ConnectivityOnly => (
                        variant_connectivity_difference_count(&self.state, reference, comparison)?,
                        "connectivity",
                    ),
                };
                (
                    DesignManagementTab::Variants,
                    format!(
                        "Compared immutable variants · {difference_count} {class_label} differences · {}",
                        result.semantic_digest
                    ),
                )
            }
            DesignManagementPage::VariantMatrix => {
                let draft = self
                    .state
                    .dialogs
                    .design_management
                    .draft
                    .as_mut()
                    .ok_or_else(|| "The Design Management draft is unavailable.".to_owned())?;
                let mut variants = draft.variants().clone();
                let mut settings = variants.settings().clone();
                settings.missing_replacement = inputs.missing_replacement;
                settings.model_equivalence = inputs.model_equivalence;
                let settings_changed = settings != *variants.settings();
                if settings_changed {
                    variants
                        .set_settings(settings)
                        .map_err(|error| error.to_string())?;
                }
                let edits = variants
                    .variants()
                    .iter()
                    .flat_map(|variant| {
                        variant
                            .definition()
                            .overrides
                            .iter()
                            .filter_map(|(object, value)| {
                                let in_scope = inputs.matrix_scope
                                    == VariantMatrixScope::AllControlledInstances
                                    || object.cell_view_key() == owner_key;
                                in_scope.then(|| VariantMatrixEdit {
                                    variant_id: variant.id(),
                                    expected_revision: variant.revision(),
                                    object: object.clone(),
                                    replacement: match value {
                                        crate::state::VariantObjectOverride::Substitute {
                                            replacement,
                                        } => Some(replacement.clone()),
                                        crate::state::VariantObjectOverride::DoNotPopulate {
                                            ..
                                        } => None,
                                    },
                                })
                            })
                    })
                    .collect::<Vec<_>>();
                if !edits.is_empty() {
                    match variants.apply_substitution_matrix(
                        edits,
                        inputs.missing_replacement,
                        inputs.model_equivalence,
                    ) {
                        Ok(_) | Err(crate::state::DesignManagementError::NoChanges(_)) => {}
                        Err(error) => return Err(error.to_string()),
                    }
                }
                if !settings_changed {
                    return Err(
                        "The governed substitution policies and existing overrides are unchanged."
                            .to_owned(),
                    );
                }
                *draft.variants_mut() = variants;
                (
                    DesignManagementTab::Variants,
                    format!(
                        "Validated {} governed override cells and applied substitution policies",
                        draft
                            .variants()
                            .variants()
                            .iter()
                            .map(|variant| variant.definition().overrides.len())
                            .sum::<usize>()
                    ),
                )
            }
            DesignManagementPage::RenumberPreview => {
                let design_management = self
                    .state
                    .dialogs
                    .design_management
                    .draft
                    .as_ref()
                    .ok_or_else(|| "The Design Management draft is unavailable.".to_owned())?;
                let request = renumber_request(
                    &self.state.workspace,
                    &self.state.schematic,
                    design_management,
                    &owner_key,
                    &inputs,
                )?;
                let draft = self
                    .state
                    .dialogs
                    .design_management
                    .draft
                    .as_mut()
                    .ok_or_else(|| "The Design Management draft is unavailable.".to_owned())?;
                let preview = draft
                    .annotation()
                    .preview_renumbering(&request)
                    .map_err(|error| error.to_string())?;
                let mapping_count = preview.mappings.len();
                let id = draft
                    .annotation_mut()
                    .commit_renumbering(&preview, &request)
                    .map_err(|error| error.to_string())?;
                (
                    DesignManagementTab::Annotation,
                    format!("Created reviewed mapping {id} · {mapping_count} reference changes"),
                )
            }
            DesignManagementPage::AnnotationPolicy => {
                let ranges = parse_reserved_ranges(
                    &inputs.reserved_ranges,
                    &self.state.dialogs.design_management,
                )?;
                let draft = self
                    .state
                    .dialogs
                    .design_management
                    .draft
                    .as_mut()
                    .ok_or_else(|| "The Design Management draft is unavailable.".to_owned())?;
                let policy = draft.annotation().policy().clone();
                let mut definition = policy.definition().clone();
                definition.prefix_allocation = inputs.prefix_allocation;
                definition.reserved_ranges = ranges;
                definition.imported_ids = inputs.imported_ids;
                let revision = draft
                    .annotation_mut()
                    .update_policy(policy.revision(), definition)
                    .map_err(|error| error.to_string())?;
                (
                    DesignManagementTab::Annotation,
                    format!("Saved annotation policy revision {revision}"),
                )
            }
            DesignManagementPage::HierarchyAudit => {
                let resolution = self.state.workspace.resolve_hierarchy_with_active(
                    &self.state.library_manager,
                    &self.state.workspace.active_view,
                    &self.state.schematic,
                );
                let configuration = match inputs.audit_configuration {
                    Some(id) => {
                        let configuration = self
                            .state
                            .workspace
                            .configuration_sets
                            .find(id)
                            .ok_or_else(|| {
                                "The selected configuration no longer exists.".to_owned()
                            })?;
                        HierarchyAuditConfiguration::ConfigurationSet {
                            id,
                            revision: configuration.revision(),
                            semantic_digest: configuration.semantic_digest(),
                        }
                    }
                    None => HierarchyAuditConfiguration::ActiveProject,
                };
                let subjects = hierarchy_audit_subjects(&resolution, inputs.audit_view_checks);
                let request = HierarchyAuditRequest {
                    configuration,
                    view_checks: match inputs.audit_view_checks {
                        HierarchyViewChecks::AllDeclaredFallbacks => {
                            crate::state::HierarchyViewChecks::AllDeclaredFallbacks
                        }
                        HierarchyViewChecks::SelectedHierarchy => {
                            crate::state::HierarchyViewChecks::SelectedHierarchy
                        }
                    },
                    protected_boundaries: match inputs.audit_protected_boundaries {
                        ProtectedBoundaryChecks::SignaturesAndPins => {
                            crate::state::ProtectedBoundaryChecks::ValidateSignaturesAndPins
                        }
                        ProtectedBoundaryChecks::PinsOnly => {
                            crate::state::ProtectedBoundaryChecks::PinsOnly
                        }
                    },
                    subjects,
                    boundary_evidence: Vec::new(),
                };
                let draft = self
                    .state
                    .dialogs
                    .design_management
                    .draft
                    .as_mut()
                    .ok_or_else(|| "The Design Management draft is unavailable.".to_owned())?;
                let id = draft
                    .run_and_record_hierarchy_audit(&request)
                    .map_err(|error| error.to_string())?;
                let audit = draft
                    .hierarchy_audits()
                    .last()
                    .ok_or_else(|| "The hierarchy audit receipt was not retained.".to_owned())?;
                (
                    DesignManagementTab::Hierarchy,
                    format!(
                        "Recorded hierarchy audit {id} · {} resolved subjects · {} findings",
                        audit.resolved_subjects(),
                        audit.findings().len()
                    ),
                )
            }
        };

        let dialog = &mut self.state.dialogs.design_management;
        dialog.page = DesignManagementPage::Manager;
        dialog.active_tab = tab;
        dialog.input_baseline = None;
        dialog.error = None;
        dialog.receipt = Some(receipt);
        dialog.discard_confirmation = false;
        dialog.body_scroll_offset = 0.0;
        Ok(())
    }
}
fn renumber_request(
    workspace: &crate::state::ProjectWorkspace,
    active_schematic: &crate::state::SchematicState,
    design_management: &DesignManagementCatalog,
    owner_key: &str,
    inputs: &SubflowInputs,
) -> Result<RenumberRequest, String> {
    let mut documents = workspace.schematic_buffers.clone();
    documents.insert(owner_key.to_owned(), active_schematic.clone());
    let mut objects = Vec::new();
    for (cell_view_key, schematic) in &documents {
        let hierarchy_path = if cell_view_key == owner_key {
            "/top".to_owned()
        } else {
            format!("/{}", cell_view_key.replace(['/', '\\'], "_"))
        };
        for component in &schematic.components {
            let prefix = component.kind.spice_prefix();
            if prefix.is_empty() || component.name.trim().is_empty() {
                continue;
            }
            let object = SchematicObjectKey::new(cell_view_key, component.id)
                .map_err(|error| error.to_string())?;
            let sheet_id =
                design_management.sheet_for_object_or_active(cell_view_key, component.id);
            objects.push(AnnotationObject {
                object,
                current_reference: component.name.clone(),
                device_family: prefix.to_owned(),
                sheet_id,
                hierarchy_path: hierarchy_path.clone(),
                position: AnnotationPosition {
                    x: i64::from(component.pos.x),
                    y: i64::from(component.pos.y),
                },
                connectivity_order: None,
                locked: false,
                external: false,
                imported: false,
            });
        }
    }
    let scope = match inputs.renumber_scope {
        RenumberScopeChoice::WholeProject => RenumberScope::WholeProject,
        RenumberScopeChoice::CurrentHierarchy => RenumberScope::CurrentHierarchy {
            path: "/top".to_owned(),
        },
        RenumberScopeChoice::CurrentSheet => {
            let sheet_id = design_management
                .sheet_catalog(owner_key)
                .and_then(|catalog| catalog.active_sheet_id())
                .ok_or_else(|| "The active sheet identity is unavailable.".to_owned())?;
            RenumberScope::CurrentSheet { sheet_id }
        }
    };
    Ok(RenumberRequest {
        scope,
        order: inputs.renumber_order,
        protected_references: inputs.protected_references,
        protected_reviewed: inputs.protected_references
            == ProtectedReferencePolicy::IncludeAfterReview,
        objects,
    })
}

fn variant_connectivity_difference_count(
    state: &AppState,
    reference: AssemblyVariantId,
    comparison: AssemblyVariantId,
) -> Result<usize, String> {
    let reference = variant_connectivity_signature(state, reference)?;
    let comparison = variant_connectivity_signature(state, comparison)?;
    Ok(reference.symmetric_difference(&comparison).count())
}

/// Produce a name-independent connectivity partition for every materialized
/// schematic owned by one variant. Each set member is the sorted list of
/// stable component-terminal identities sharing a canonical electrical net.
/// Comparing these partitions detects DNP and substitution topology effects
/// without treating value/model-only changes as connectivity differences.
fn variant_connectivity_signature(
    state: &AppState,
    variant_id: AssemblyVariantId,
) -> Result<BTreeSet<Vec<String>>, String> {
    let mut workspace = state.workspace.clone();
    workspace
        .design_management
        .variants_mut()
        .set_active(variant_id)
        .map_err(|error| error.to_string())?;
    let projection = workspace
        .configuration_execution_projection(
            &state.library_manager,
            &state.workspace.active_view,
            &state.schematic,
        )
        .map_err(|error| error.to_string())?;
    let hierarchy = crate::simulation::netlist_gen::HierarchySource::from_workspace(
        &state.library_manager,
        projection.schematic_buffers(),
    );
    let mut groups = BTreeMap::<(String, String), Vec<String>>::new();
    for (cell_view_key, schematic) in projection.schematic_buffers() {
        let generated = crate::simulation::netlist_gen::generate_netlist_hierarchical(
            schematic,
            &[],
            &hierarchy,
        );
        if !generated.errors.is_empty() {
            return Err(format!(
                "Variant connectivity could not be resolved for `{cell_view_key}`: {}",
                generated.errors.join(" ")
            ));
        }
        for component in &schematic.components {
            for (terminal_name, point) in component.terminal_positions_resolved(None) {
                let net_name = generated
                    .point_to_net
                    .get(&point)
                    .cloned()
                    .unwrap_or_else(|| format!("__unconnected_{}_{}", component.id, terminal_name));
                groups
                    .entry((cell_view_key.to_ascii_lowercase(), net_name))
                    .or_default()
                    .push(format!(
                        "{}#{}:{}",
                        cell_view_key.to_ascii_lowercase(),
                        component.id,
                        terminal_name.to_ascii_lowercase()
                    ));
            }
        }
    }
    Ok(groups
        .into_values()
        .map(|mut endpoints| {
            endpoints.sort();
            endpoints.dedup();
            endpoints
        })
        .collect())
}

fn hierarchy_audit_subjects(
    resolution: &crate::state::workspace::HierarchyResolution,
    view_checks: HierarchyViewChecks,
) -> Vec<HierarchyAuditSubject> {
    let mut subjects = resolution
        .bindings
        .iter()
        .flat_map(|binding| {
            binding.instance_paths.iter().filter_map(move |path| {
                if view_checks == HierarchyViewChecks::SelectedHierarchy
                    && !path.starts_with("/top")
                {
                    return None;
                }
                let resolved = binding.status.is_resolved().then(|| {
                    binding
                        .stop_view
                        .clone()
                        .unwrap_or_else(|| binding.reference.view.clone())
                });
                Some(HierarchyAuditSubject {
                    instance_path: path.clone(),
                    cell_name: binding.reference.cell.clone(),
                    design_view: binding.reference.view.clone(),
                    declared_fallbacks: binding.view_search_order.clone(),
                    resolved_simulation_view: resolved.clone(),
                    fallback_used: binding.used_review_fallback.then_some(resolved).flatten(),
                    child_instance_paths: Vec::new(),
                    protected_boundary_id: None,
                })
            })
        })
        .collect::<Vec<_>>();
    let paths = subjects
        .iter()
        .map(|subject| subject.instance_path.clone())
        .collect::<Vec<_>>();
    for subject in &mut subjects {
        let prefix = format!("{}/", subject.instance_path.trim_end_matches('/'));
        subject.child_instance_paths = paths
            .iter()
            .filter(|path| {
                path.starts_with(&prefix)
                    && !path[prefix.len()..].contains('/')
                    && **path != subject.instance_path
            })
            .cloned()
            .collect();
    }
    subjects
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn manager_and_subflow_splits_match_the_mockup_contract() {
        assert_eq!(main_split_widths(760.0), (380.0, 380.0));
        assert_eq!(subflow_split_widths(980.0), (646.0, 334.0));
    }

    #[test]
    fn arrow_order_requires_each_sheet_once() {
        let mut catalog = DesignManagementCatalog::default();
        catalog
            .bootstrap_for_cell_view("work/top/schematic", "A", [1])
            .unwrap();
        let sheets = catalog.sheet_catalog_mut("work/top/schematic").unwrap();
        sheets
            .create_sheet(
                SheetDefinition {
                    name: "B".to_owned(),
                    template: SheetTemplate::AnalogSchematic,
                    port_policy: SheetPortPolicy::TypedOffSheetPorts,
                    explicit_page_number: Some(2),
                },
                sheets.active_sheet_id(),
            )
            .unwrap();
        let mut dialog = DesignManagementDialogState {
            owner_key: "work/top/schematic".to_owned(),
            draft: Some(catalog),
            ..DesignManagementDialogState::default()
        };
        dialog.inputs.reorder_order_text = "B → A".to_owned();
        assert_eq!(reorder_sheet_ids(&dialog).unwrap().len(), 2);
        dialog.inputs.reorder_order_text = "A → A".to_owned();
        assert!(reorder_sheet_ids(&dialog).is_err());
    }

    #[test]
    fn connectivity_only_variant_comparison_detects_dnp_topology_change() {
        let mut state = AppState::default();
        let component = state.schematic.add_component(
            crate::state::ComponentType::Resistor,
            crate::state::Point::origin(),
        );
        let owner = state.workspace.active_view.key();
        state
            .workspace
            .schematic_buffers
            .insert(owner.clone(), state.schematic.clone());
        let reference = state
            .workspace
            .design_management
            .variants_mut()
            .create(AssemblyVariantDraft {
                name: "Populated".to_owned(),
                parent_id: None,
                inheritance: VariantInheritance::OverrideChangedObjectsOnly,
                qualification_plan: VariantQualificationPlan::InvalidateAffectedTests,
                overrides: BTreeMap::new(),
            })
            .expect("reference variant");
        let object = SchematicObjectKey::new(&owner, component).expect("scoped component");
        let comparison = state
            .workspace
            .design_management
            .variants_mut()
            .create(AssemblyVariantDraft {
                name: "DNP".to_owned(),
                parent_id: None,
                inheritance: VariantInheritance::OverrideChangedObjectsOnly,
                qualification_plan: VariantQualificationPlan::InvalidateAffectedTests,
                overrides: BTreeMap::from([(
                    object,
                    crate::state::VariantObjectOverride::DoNotPopulate {
                        approval_reference: "review-1".to_owned(),
                    },
                )]),
            })
            .expect("comparison variant");

        assert!(
            variant_connectivity_difference_count(&state, reference, comparison)
                .expect("connectivity comparison")
                > 0
        );
    }
}
