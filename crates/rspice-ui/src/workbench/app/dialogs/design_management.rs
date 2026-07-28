//! Mockup-owned Design Management manager and subordinate transactions.
//!
//! The manager owns one isolated [`DesignManagementCatalog`] draft. Subordinate
//! workflows mutate that same draft atomically; only the manager's primary
//! action can publish the complete candidate into the project workspace.

use std::collections::{BTreeMap, BTreeSet};

use egui::{Align, Context, Frame, Layout, Margin, Sense, Stroke, Ui, Vec2, vec2};

use crate::diagnostics::ConsoleMessage;
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

use crate::workbench::app::dialogs::review_primitives::{input_field, read_only_field};
use crate::workbench::app::RSpiceApp;
use crate::workbench::app_state::DesignManagementHistoryEntry;
use crate::workbench::app_state::AppState;

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

mod controller;
mod manager;
mod operations;
mod subflows;
#[cfg(test)]
mod tests;
mod widgets;
