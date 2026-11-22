//! Live connectivity, typed-bus, and global-net review for the schematic workspace.
//!
//! The manager is an inspection surface first. Its report is rebuilt from the
//! same hierarchy-aware extraction and DRC inputs used by netlisting. Mutations
//! are explicit, stale guarded, validated on a cloned schematic, and published
//! as one undo transaction.

use rspice_design::schematic::document::SchematicDocument;
use std::collections::{BTreeSet, HashMap, HashSet};

use egui::{Context, Grid, RichText, ScrollArea, Ui, Vec2};

use crate::state::workspace::DesignProjection;
use crate::state::{
    BundleWidthMismatchPolicy, BusDeclaration, BusDirection, BusTargetKind, CellViewRef,
    ConnectivityContract, ConnectivityPolicy, GlobalAliasComparisonPolicy,
    GlobalNetPromotionPolicy, Point, SchematicSnapshot, SchematicState, Tool,
};
use crate::ui::theme::{self, FontWeight};
use crate::ui::tokens::{self, Tokens};
use crate::ui::widgets::{
    Button, CommandForm, Dialog, DialogChoice, DialogInitialFocus, DialogSize,
    DialogTransactionTone, FieldLine,
};
use crate::ui::widgets::{DIALOG_BODY_INSET, band_header};
use rspice_design::connectivity::summary::{DesignNet, NetClass, projection_nets};
use rspice_design::drc::{
    CheckReach, DrcLocation, DrcResult, DrcSeverity, DrcViolation, DrcViolationType,
    run_check_of_cellview,
};
use rspice_design::hierarchy::HierarchySource;

use crate::workbench::app::SchematicEditAuthority;
use crate::workbench::app::actions::workspace::NavigationParts;
use crate::workbench::app::actions::workspace::sync_active_schematic;
use crate::workbench::app::dialogs::design_request::DesignRequest;
use crate::workbench::app::schematic::conductor_routes::start_canvas_wire;
use crate::workbench::app_state::design_history::DesignPolicy;
use crate::workbench::app_state::{Model, PlatformState};
use crate::workbench::state::WorkbenchSession;
use rspice_app_runs::simulation::SimulationController;
use rspice_app_runs::workbench::slots::runs::RunsState;
use rspice_schematic_editor::session::visibility::SchematicReviewMarkerVisibility;

const TITLE: &str = "Connectivity, buses and global nets";
const DESCRIPTION: &str = "Every net, bus and global net in the schematic, and the repairs their problems need. Nothing changes until you apply a repair.";
pub const PRIMARY: &str = "Apply repairs";
/// Why "Draw bus" is disabled while the name is empty.
const BUS_DECLARATION_MISSING: &str = "Name the bus with its bits to draw it";
/// Why the manager's Apply is disabled while nothing is chosen to apply.
const NOTHING_TO_APPLY: &str = "Select a repair or change a policy to apply.";
/// Why "Draw bus" is disabled on a read-only schematic.
const READ_ONLY_STATUS: &str = "The active schematic or project is read-only.";

#[derive(Debug, Clone, Copy, Default, PartialEq, Eq)]
pub(crate) enum ConnectivityManagerTab {
    #[default]
    Connectivity,
    Buses,
    Globals,
    RepairPreview,
}

impl ConnectivityManagerTab {
    const ALL: [Self; 4] = [
        Self::Connectivity,
        Self::Buses,
        Self::Globals,
        Self::RepairPreview,
    ];

    const fn label(self) -> &'static str {
        match self {
            Self::Connectivity => "Connectivity",
            Self::Buses => "Buses",
            Self::Globals => "Global nets",
            Self::RepairPreview => "Repairs",
        }
    }
}

#[derive(Debug, Clone, Copy, Default, PartialEq, Eq)]
pub enum ConnectivityManagerPage {
    #[default]
    Manager,
    CreateBus,
}

#[derive(Debug, Clone)]
pub struct ConnectivityManagerDialogState {
    pub open: bool,
    pub page: ConnectivityManagerPage,
    pub(crate) active_tab: ConnectivityManagerTab,
    pub(crate) authority: Option<SchematicEditAuthority>,
    report: ConnectivityReport,
    selected_repairs: BTreeSet<String>,
    contract_at_open: ConnectivityContract,
    pub(crate) policy: ConnectivityPolicy,
    pub(crate) bus_declaration: String,
    endpoint_choices: HashMap<String, usize>,
    pub error: Option<String>,
    pub(crate) receipt: Option<String>,
    pub(crate) body_scroll_offset: f32,
}

impl Default for ConnectivityManagerDialogState {
    fn default() -> Self {
        Self {
            open: false,
            page: ConnectivityManagerPage::Manager,
            active_tab: ConnectivityManagerTab::Connectivity,
            authority: None,
            report: ConnectivityReport::default(),
            selected_repairs: BTreeSet::new(),
            contract_at_open: ConnectivityContract::default(),
            policy: ConnectivityPolicy::default(),
            // A declaration is authored, never proposed: the field opens
            // empty and the placeholder shows the syntax.
            bus_declaration: String::new(),
            endpoint_choices: HashMap::new(),
            error: None,
            receipt: None,
            body_scroll_offset: 0.0,
        }
    }
}

impl ConnectivityManagerDialogState {
    pub fn close(&mut self) {
        *self = Self::default();
    }

    fn selected_actionable_count(&self) -> usize {
        self.report
            .repairs
            .iter()
            .filter(|repair| {
                repair_takes_apply(repair) && self.selected_repairs.contains(&repair.id)
            })
            .count()
    }

    fn policy_changed(&self) -> bool {
        self.policy != self.contract_at_open.policy
    }
}

#[derive(Debug, Clone, Default)]
struct ConnectivityReport {
    nets: Vec<ConnectivityNetRow>,
    buses: Vec<ConnectivityBusRow>,
    globals: Vec<GlobalNetRow>,
    repairs: Vec<ConnectivityRepair>,
    drc: DrcResult,
}

#[derive(Debug, Clone)]
struct ConnectivityNetRow {
    name: String,
    kind: String,
    endpoints: usize,
    drivers: usize,
    loads: usize,
    passive: usize,
    discipline: String,
    issue: String,
    wire_ids: Vec<u64>,
    component_ids: Vec<u64>,
}

#[derive(Debug, Clone)]
struct ConnectivityBusRow {
    reveal: RevealTarget,
    declaration: String,
    members: usize,
    taps: usize,
    mapping: String,
    direction: String,
    status: String,
}

#[derive(Debug, Clone)]
struct GlobalNetRow {
    canonical: String,
    aliases: Vec<String>,
    pins: usize,
    scope: String,
    state: String,
    declarations: Vec<LabelOccurrence>,
    local_aliases: Vec<LabelOccurrence>,
}

/// One authored net label, located by the document it belongs to and its
/// object identity. No coordinate is carried: the occurrence is read from the
/// materialized projection, whose multi-sheet pages are namespaced apart,
/// while the canvas scrolls in the authored coordinates of the opened
/// document — which is therefore the only authority on where the label sits.
#[derive(Debug, Clone, PartialEq, Eq)]
struct LabelOccurrence {
    view_key: String,
    id: u64,
    name: String,
}

#[derive(Debug, Clone)]
struct EndpointChoice {
    /// The net or pin, as the reader knows it: `Net VOUT`, `Pin R3.2`.
    name: String,
    /// The name and how far it is from the pin, as the list shows it.
    label: String,
    start: Point,
    destination: Point,
}

#[derive(Debug, Clone)]
struct ConnectivityRepair {
    id: String,
    object: String,
    before: String,
    proposed: String,
    identity_effect: String,
    validation: String,
    action: Option<ConnectivityRepairAction>,
    endpoint_choices: Vec<EndpointChoice>,
    reveal: RevealTarget,
}

/// Whether Apply can carry `repair` out. An open pin's repair is a wire only
/// the reader can draw, so its row offers Draw wire and no Apply tick: a tick
/// Apply then skipped was a control that did nothing.
fn repair_takes_apply(repair: &ConnectivityRepair) -> bool {
    repair.action.is_some()
}

#[derive(Debug, Clone)]
enum ConnectivityRepairAction {
    RenameLabels {
        canonical: String,
        labels: Vec<(u64, String)>,
    },
}

#[derive(Debug, Clone)]
pub enum RevealTarget {
    Net {
        wire_ids: Vec<u64>,
        component_ids: Vec<u64>,
    },
    Bus {
        id: u64,
        point: Point,
    },
    BusTap {
        id: u64,
        point: Point,
    },
    Label {
        view_key: String,
        id: u64,
    },
    Violation(DrcLocation),
}

#[derive(Default, Debug, Clone)]
enum ConnectivityBodyAction {
    #[default]
    None,
    Reveal(RevealTarget),
    Refresh,
    CreateBus,
    ReviewAliases,
    RouteRepair(String),
    /// Open the generated netlist at the net's first card.
    OpenSourceMap(String),
}

pub fn open_connectivity_manager(
    dialog: &mut ConnectivityManagerDialogState,
    model: &mut Model,
    workbench: &mut WorkbenchSession,
    platform: &PlatformState,
) {
    // The command is greyed with no project open.
    if !platform.project_lifecycle.is_open() {
        return;
    }
    if !matches!(
        model.workspace.content.active_view_type(),
        Some(crate::state::ViewType::Schematic | crate::state::ViewType::Testbench)
    ) {
        workbench.prompt("Open a schematic or testbench to see its connectivity.");
        return;
    }
    model.retain_selection_on_active_sheet();
    crate::workbench::app::actions::workspace::sync_active_schematic(model, workbench);
    let contract = (*model.workspace.content.connectivity).clone();
    let (report, error) = match build_report(model, &contract.policy) {
        Ok(report) => (report, None),
        Err(error) => (ConnectivityReport::default(), Some(error)),
    };
    *dialog = ConnectivityManagerDialogState {
        open: true,
        authority: Some(SchematicEditAuthority::capture(&*model)),
        report,
        contract_at_open: contract.clone(),
        policy: contract.policy,
        error,
        ..ConnectivityManagerDialogState::default()
    };
}

/// The design as the policy on screen resolves it.
///
/// The dialog edits a policy before it is applied, and every pane has to
/// answer for the policy the user is looking at rather than the one on disk.
/// An unchanged policy reads the project's own memoized projection; an edited
/// one projects a workspace carrying it, so a preview is a real resolution
/// and never a second reading of the live editor buffers.
fn preview_projection(
    model: &Model,
    policy: &ConnectivityPolicy,
) -> Result<std::sync::Arc<DesignProjection>, String> {
    projection_with_open_view(model, policy, &model.schematic)
}

/// The design as `policy` resolves it with `open_view` standing for the open
/// view's buffer, which is how a repair candidate is projected exactly as the
/// design it would replace.
fn projection_with_open_view(
    model: &Model,
    policy: &ConnectivityPolicy,
    open_view: &SchematicState,
) -> Result<std::sync::Arc<DesignProjection>, String> {
    if model.workspace.content.connectivity.policy == *policy {
        return model
            .workspace
            .design_projection(
                &model.library_manager,
                &model.workspace.content.active_view,
                open_view,
            )
            .map_err(|error| error.to_string());
    }
    let mut preview = model.workspace.clone();
    preview.content.connectivity.policy = policy.clone();
    preview
        .design_projection(
            &model.library_manager,
            &model.workspace.content.active_view,
            open_view,
        )
        .map_err(|error| error.to_string())
}

/// The design check the manager states and validates repairs against: the
/// open view as the projection materialized it, checked as the cellview it
/// is.
///
/// The cellview is named rather than recognized from the buffer, so a
/// configured root is walked as its configuration runs it whichever document
/// holds the drawing — the projection's own buffer or a repair candidate's.
///
/// Checked under the configuration the project checks the open view under
/// everywhere else, with `policy` — the one on screen, which the projection
/// was resolved under — standing for the project's, so this page cannot state
/// a finding the Problems page and the run gate judge differently.
fn check_projected_open_view(
    model: &Model,
    policy: &ConnectivityPolicy,
    projection: &DesignProjection,
) -> Result<DrcResult, String> {
    let subject = projected_open_view(model, projection)?;
    let hierarchy =
        HierarchySource::from_design_projection(model.library_manager.catalog(), projection);
    let mut config = model.design_check_config(&model.workspace.content.active_view);
    config.connectivity = policy.clone();
    Ok(run_check_of_cellview(
        subject,
        &model.workspace.content.active_view,
        &hierarchy,
        config,
        CheckReach::Hierarchy,
    ))
}

/// The open view exactly as the projection materialized it: multi-sheet pages
/// namespaced apart, cross-sheet contracts realized, the active assembly
/// variant applied. Every electrical statement the manager makes is read from
/// this document, so the manager cannot report a net the generator will not
/// emit.
fn projected_open_view<'a>(
    model: &Model,
    projection: &'a DesignProjection,
) -> Result<&'a rspice_design::projection::ProjectedSchematic, String> {
    let active_key = model.workspace.content.active_view.key();
    projection
        .schematic_buffers()
        .iter()
        .find(|(key, _)| key.eq_ignore_ascii_case(&active_key))
        .map(|(_, schematic)| schematic)
        .ok_or_else(|| format!("The open view {active_key} is not part of the configured design."))
}

fn build_report(model: &Model, policy: &ConnectivityPolicy) -> Result<ConnectivityReport, String> {
    let projection = preview_projection(model, policy)?;
    let subject = projected_open_view(model, &projection)?;
    let design_nets = projection_nets(
        model.library_manager.catalog(),
        &projection,
        &model.workspace.content.active_view.key(),
    );
    let drc = check_projected_open_view(model, policy, &projection)?;
    // The bus rows and every reveal anchor below stay on the document the
    // canvas paints: a materialized page carries namespaced coordinates that
    // would scroll the editor off the design.
    let bus_analysis = canvas_bus_analysis(model, subject);
    let nets = design_nets
        .iter()
        .map(|net| build_net_row(net, subject, &drc))
        .collect();
    let buses = model
        .schematic
        .document()
        .buses
        .iter()
        .map(|bus| {
            let diagnostics = bus_analysis
                .diagnostics
                .iter()
                .filter(|diagnostic| diagnostic.bus_id == Some(bus.id))
                .map(|diagnostic| diagnostic.message.as_str())
                .collect::<Vec<_>>();
            let taps = model
                .schematic
                .document()
                .bus_taps
                .iter()
                .filter(|tap| tap.bus_id == bus.id)
                .collect::<Vec<_>>();
            let mapping = if taps.is_empty() {
                "No taps".to_owned()
            } else {
                let scalar = taps
                    .iter()
                    .filter(|tap| tap.target_kind() == BusTargetKind::Wire)
                    .count();
                format!(
                    "{} \u{00b7} {}",
                    counted(scalar, "wire", "wires"),
                    counted(taps.len() - scalar, "bus", "buses")
                )
            };
            let (declaration, members, direction) = bus.declaration.as_ref().map_or_else(
                || ("Undeclared".to_owned(), 0, "\u{2014}".to_owned()),
                |declaration| {
                    (
                        declaration.to_string(),
                        declaration.width(),
                        match declaration.direction() {
                            BusDirection::Ascending => "ascending",
                            BusDirection::Descending => "descending",
                        }
                        .to_owned(),
                    )
                },
            );
            ConnectivityBusRow {
                reveal: RevealTarget::Bus {
                    id: bus.id,
                    point: bus.points.first().copied().unwrap_or(Point::origin()),
                },
                declaration,
                members,
                taps: taps.len(),
                mapping,
                direction,
                status: if diagnostics.is_empty() {
                    "valid".to_owned()
                } else {
                    diagnostics.join(" ")
                },
            }
        })
        .collect::<Vec<_>>();
    let globals = build_global_rows(model, policy, &projection);
    let mut repairs = build_global_repairs(model, policy, &globals);
    repairs.extend(build_manual_repairs(
        model,
        &design_nets,
        &drc,
        &bus_analysis,
    ));
    Ok(ConnectivityReport {
        nets,
        buses,
        globals,
        repairs,
        drc,
    })
}

/// The open view's typed-bus findings as the design check judges them, each
/// located where its sheet draws it.
///
/// The judgement reads the projected page, which lays the sheets apart: a bus
/// drawn where another sheet's bus runs touches nothing, while one stem
/// declared in two notations is still one deck name across sheets. Each
/// finding's point is then taken back off its sheet's stride, onto the canvas.
fn canvas_bus_analysis(
    model: &Model,
    subject: &rspice_design::projection::ProjectedSchematic,
) -> rspice_design::connectivity::bus::BusConnectivityAnalysis {
    let mut analysis =
        rspice_design::connectivity::bus::analyze_bus_connectivity(subject.document());
    let key = model.workspace.content.active_schematic_reference().key();
    let Some(sheets) = model
        .workspace
        .content
        .design_management
        .sheet_catalog(&key)
    else {
        return analysis;
    };
    let on_canvas = |point| {
        rspice_design::schematic::owned::projected_point_sheet(sheets, point)
            .map_or(point, |(_, point)| point)
    };
    for diagnostic in &mut analysis.diagnostics {
        diagnostic.point = on_canvas(diagnostic.point);
    }
    for binding in &mut analysis.scalar_taps {
        binding.point = on_canvas(binding.point);
    }
    analysis
}

fn build_net_row(
    net: &DesignNet,
    schematic: &impl AsRef<SchematicDocument>,
    drc: &DrcResult,
) -> ConnectivityNetRow {
    let NetDrive {
        drivers,
        loads,
        passive,
        discipline,
    } = endpoint_contract(net, schematic);
    let issues = drc
        .violations()
        .iter()
        .filter(|violation| violation_targets_net(violation, net))
        .map(|violation| violation.message.as_str())
        .collect::<Vec<_>>();
    ConnectivityNetRow {
        name: net.name.clone(),
        kind: match net.class {
            NetClass::Ground => "Ground",
            NetClass::Supply => "Supply",
            NetClass::Signal if net.is_port() => "Port",
            NetClass::Signal => "Signal",
        }
        .to_owned(),
        endpoints: net.terminals.len(),
        drivers,
        loads,
        passive,
        discipline,
        issue: if issues.is_empty() {
            "None".to_owned()
        } else {
            issues.join(" ")
        },
        wire_ids: net.wire_ids.clone(),
        component_ids: net
            .terminals
            .iter()
            .map(|terminal| terminal.component_id)
            .collect(),
    }
}

/// How a net is driven, counted pin by pin from what each pin declares: an
/// interface port's direction, a source, or the direction a device's pin
/// has (an op-amp or controlled source output drives, its inputs load). A
/// pin that declares none, a resistor's or a transistor's, is passive and
/// neither drives nor loads, as KiCad and Altium count pin types. The
/// discipline is each pin's own.
fn endpoint_contract(net: &DesignNet, schematic: &impl AsRef<SchematicDocument>) -> NetDrive {
    let mut drive = NetDrive::default();
    let mut disciplines = BTreeSet::new();
    for terminal in &net.terminals {
        let Some(component) = schematic
            .as_ref()
            .components
            .iter()
            .find(|component| component.id == terminal.component_id)
        else {
            continue;
        };
        let direction = if let Some(contract) = component.port_contract() {
            disciplines.insert(contract.discipline.keyword());
            Some(contract.direction)
        } else {
            disciplines.insert(
                rspice_design::schematic::hierarchy::hierarchy_terminal_discipline(
                    component,
                    &terminal.pin,
                )
                .keyword(),
            );
            if matches!(
                component.kind,
                crate::state::ComponentType::VoltageSource
                    | crate::state::ComponentType::CurrentSource
            ) {
                Some(crate::state::PortDirection::Out)
            } else {
                rspice_design::schematic::hierarchy::hierarchy_terminal_direction(
                    component,
                    &terminal.pin,
                )
            }
        };
        match direction {
            Some(crate::state::PortDirection::Out) => drive.drivers += 1,
            Some(crate::state::PortDirection::In) => drive.loads += 1,
            Some(crate::state::PortDirection::InOut | crate::state::PortDirection::Supply) => {
                drive.drivers += 1;
                drive.loads += 1;
            }
            None => drive.passive += 1,
        }
    }
    drive.discipline = if disciplines.is_empty() {
        "electrical".to_owned()
    } else {
        disciplines.into_iter().collect::<Vec<_>>().join(" / ")
    };
    drive
}

/// What drives a net and what it drives, by pin.
#[derive(Debug, Default, PartialEq, Eq)]
struct NetDrive {
    drivers: usize,
    loads: usize,
    passive: usize,
    discipline: String,
}

fn violation_targets_net(violation: &DrcViolation, net: &DesignNet) -> bool {
    match &violation.location {
        DrcLocation::Node { net_name, .. } | DrcLocation::NetLabel { name: net_name } => {
            net_name.eq_ignore_ascii_case(&net.name)
        }
        DrcLocation::Wire { id } => net.wire_ids.contains(id),
        DrcLocation::Component { id, .. } => net
            .terminals
            .iter()
            .any(|terminal| terminal.component_id == *id),
        // An open terminal stands alone on a node of its own, and that node
        // is the row the finding belongs to: the component's other pins are
        // on other nets the finding says nothing about.
        DrcLocation::Terminal {
            component_id,
            pin_name,
            ..
        } => net.terminals.iter().any(|terminal| {
            terminal.component_id == *component_id && terminal.pin.eq_ignore_ascii_case(pin_name)
        }),
        // A finding inside an instantiated cellview is about that cellview's
        // nets, not this one's.
        DrcLocation::InsideInstance { .. }
        | DrcLocation::Point { .. }
        | DrcLocation::Bus { .. }
        | DrcLocation::BusTap { .. }
        | DrcLocation::Global
        | DrcLocation::CellView { .. }
        | DrcLocation::SymbolPin { .. } => false,
    }
}

/// Global-net rows over every view of the projected design.
///
/// A global is a cross-view claim, so it can only be counted over the whole
/// design at once — and only over the design as configured: an alias that a
/// variant removes is not a shadow anybody has to review.
fn build_global_rows(
    model: &Model,
    policy: &ConnectivityPolicy,
    projection: &DesignProjection,
) -> Vec<GlobalNetRow> {
    let mut preview_contract = (*model.workspace.content.connectivity).clone();
    preview_contract.policy = policy.clone();
    let contract = &preview_contract;
    let mut declarations = Vec::<LabelOccurrence>::new();
    let mut locals = Vec::<LabelOccurrence>::new();
    let mut nets_by_view = HashMap::<String, std::sync::Arc<Vec<DesignNet>>>::new();
    for (view_key, schematic) in projection.schematic_buffers() {
        let nets = projection_nets(model.library_manager.catalog(), projection, view_key);
        nets_by_view.insert(view_key.clone(), nets);
        for label in &schematic.document().net_labels {
            let occurrence = LabelOccurrence {
                view_key: view_key.clone(),
                id: label.id,
                name: label.name.clone(),
            };
            // A supply symbol declares its rail as a `!` name does: the label
            // states it, whatever the rail is called.
            let declares = label.global_declaration().is_some();
            match policy.global_promotion {
                GlobalNetPromotionPolicy::ExplicitReviewedDeclaration => {
                    if declares {
                        declarations.push(occurrence);
                    } else {
                        locals.push(occurrence);
                    }
                }
                GlobalNetPromotionPolicy::TechnologyDefinedOnly => {
                    if contract
                        .technology_global_canonical_name(&label.name)
                        .is_some()
                        || declares
                    {
                        declarations.push(occurrence);
                    } else {
                        locals.push(occurrence);
                    }
                }
            }
        }
    }
    declarations.sort_by(|left, right| {
        left.name
            .cmp(&right.name)
            .then_with(|| left.view_key.cmp(&right.view_key))
            .then_with(|| left.id.cmp(&right.id))
    });

    let mut rows = Vec::<GlobalNetRow>::new();
    for declaration in declarations {
        let technology_canonical = contract
            .technology_global_canonical_name(&declaration.name)
            .map(str::to_owned);
        let technology_authorized = technology_canonical.is_some();
        let canonical = technology_canonical.unwrap_or_else(|| declaration.name.clone());
        let existing = rows.iter_mut().find(|row| {
            alias_matches(
                &row.canonical,
                &canonical,
                policy.alias_comparison,
                contract,
            )
        });
        if let Some(row) = existing {
            row.declarations.push(declaration);
            continue;
        }
        let local_aliases = locals
            .iter()
            .filter(|local| {
                alias_matches(&local.name, &canonical, policy.alias_comparison, contract)
            })
            .cloned()
            .collect::<Vec<_>>();
        let pins = nets_by_view
            .values()
            .flat_map(|nets| nets.iter())
            .filter(|net| {
                alias_matches(&net.name, &canonical, policy.alias_comparison, contract)
                    || (policy.global_promotion == GlobalNetPromotionPolicy::TechnologyDefinedOnly
                        && contract.technology_global_canonical_name(&net.name)
                            == Some(canonical.as_str()))
            })
            .map(DesignNet::pin_count)
            .sum();
        let technology_only =
            policy.global_promotion == GlobalNetPromotionPolicy::TechnologyDefinedOnly;
        rows.push(GlobalNetRow {
            canonical,
            aliases: local_aliases
                .iter()
                .map(|alias| format!("{} ({})", alias.name, alias.view_key))
                .collect(),
            pins,
            scope: if technology_only {
                contract.technology_global_nets.as_ref().map_or_else(
                    || "the technology lists no global nets".to_owned(),
                    |catalog| format!("technology · {}", catalog.authority),
                )
            } else {
                "project".to_owned()
            },
            state: if technology_only && !technology_authorized {
                "not among the technology's global nets".to_owned()
            } else if local_aliases.is_empty() {
                "resolved".to_owned()
            } else {
                "local labels shadow it".to_owned()
            },
            declarations: vec![declaration],
            local_aliases,
        });
    }
    rows
}

/// "1 wire", "3 wires".
fn counted(count: usize, one: &str, many: &str) -> String {
    format!("{count} {}", if count == 1 { one } else { many })
}

fn alias_matches(
    left: &str,
    right: &str,
    comparison: GlobalAliasComparisonPolicy,
    contract: &ConnectivityContract,
) -> bool {
    contract.aliases_match(left, right, comparison)
}

fn build_global_repairs(
    model: &Model,
    policy: &ConnectivityPolicy,
    globals: &[GlobalNetRow],
) -> Vec<ConnectivityRepair> {
    if policy.global_promotion == GlobalNetPromotionPolicy::TechnologyDefinedOnly {
        return Vec::new();
    }
    let active_key = model.workspace.content.active_schematic_reference().key();
    let mut repairs = Vec::new();
    for global in globals {
        let active_aliases = global
            .local_aliases
            .iter()
            .filter(|alias| alias.view_key == active_key)
            .map(|alias| (alias.id, alias.name.clone()))
            .collect::<Vec<_>>();
        let reveal = global
            .declarations
            .first()
            .map(|label| RevealTarget::Label {
                view_key: label.view_key.clone(),
                id: label.id,
            })
            .unwrap_or(RevealTarget::Violation(DrcLocation::Global));
        if !active_aliases.is_empty() {
            repairs.push(ConnectivityRepair {
                id: format!("global-alias:{}:{active_key}", global.canonical),
                object: global.canonical.clone(),
                before: active_aliases
                    .iter()
                    .map(|(_, name)| name.as_str())
                    .collect::<Vec<_>>()
                    .join(", "),
                proposed: format!("Rename to {}", global.canonical),
                identity_effect: "Joins them into one global net".to_owned(),
                validation: "DRC and the bus check run before it applies".to_owned(),
                action: Some(ConnectivityRepairAction::RenameLabels {
                    canonical: global.canonical.clone(),
                    labels: active_aliases,
                }),
                endpoint_choices: Vec::new(),
                reveal: reveal.clone(),
            });
        }
        let external_aliases = global
            .local_aliases
            .iter()
            .filter(|alias| alias.view_key != active_key)
            .collect::<Vec<_>>();
        if !external_aliases.is_empty() {
            repairs.push(ConnectivityRepair {
                id: format!("global-alias-external:{}", global.canonical),
                object: global.canonical.clone(),
                before: external_aliases
                    .iter()
                    .map(|alias| format!("{} ({})", alias.name, alias.view_key))
                    .collect::<Vec<_>>()
                    .join(", "),
                proposed: "Open each view and rename its local name there".to_owned(),
                identity_effect: "Other views are not renamed from here".to_owned(),
                validation: "Renamed in its own view, with its own undo".to_owned(),
                action: None,
                endpoint_choices: Vec::new(),
                reveal: RevealTarget::Label {
                    view_key: external_aliases[0].view_key.clone(),
                    id: external_aliases[0].id,
                },
            });
        }
    }
    repairs
}

fn build_manual_repairs(
    model: &Model,
    nets: &[DesignNet],
    drc: &DrcResult,
    bus_analysis: &rspice_design::connectivity::bus::BusConnectivityAnalysis,
) -> Vec<ConnectivityRepair> {
    let mut repairs = Vec::new();
    for violation in drc.violations() {
        // A repair is a wire drawn in this view. A finding inside an
        // instantiated cellview has no endpoint here to choose and nowhere
        // here to reveal, and a row per occurrence of it would offer the same
        // nothing several times; it is repaired in its own cellview.
        if matches!(violation.location, DrcLocation::InsideInstance { .. }) {
            continue;
        }
        if !matches!(
            violation.violation_type,
            // Every unattached terminal belongs here, whatever the interface
            // declared it to be: the repair is the same wire either way.
            DrcViolationType::FloatingNode
                | DrcViolationType::UnconnectedPin
                | DrcViolationType::UndrivenInput
                | DrcViolationType::UnloadedOutput
                | DrcViolationType::DanglingWire
                | DrcViolationType::MalformedBus
                | DrcViolationType::UnnamedBus
                | DrcViolationType::BusRangeConflict
                | DrcViolationType::DanglingBusTap
                | DrcViolationType::MixedBusTap
                | DrcViolationType::DuplicateBusMemberDriver
                | DrcViolationType::ShortedOutputs
                | DrcViolationType::SourceToSource
        ) {
            continue;
        }
        repairs.push(ConnectivityRepair {
            id: format!("drc:{}", violation.id),
            object: violation.location.display(),
            before: violation.message.clone(),
            proposed: violation.violation_type.suggested_fix().to_owned(),
            identity_effect: "No net is renamed".to_owned(),
            validation: "Choose what to route to".to_owned(),
            action: None,
            endpoint_choices: endpoint_choices_for_violation(model, nets, violation),
            reveal: RevealTarget::Violation(violation.location.clone()),
        });
    }
    for diagnostic in &bus_analysis.diagnostics {
        if repairs
            .iter()
            .any(|repair| repair.before == diagnostic.message)
        {
            continue;
        }
        let reveal = if let Some(id) = diagnostic.tap_id {
            RevealTarget::BusTap {
                id,
                point: diagnostic.point,
            }
        } else if let Some(id) = diagnostic.bus_id {
            RevealTarget::Bus {
                id,
                point: diagnostic.point,
            }
        } else {
            RevealTarget::Violation(DrcLocation::Point {
                x: f64::from(diagnostic.point.x),
                y: f64::from(diagnostic.point.y),
            })
        };
        repairs.push(ConnectivityRepair {
            id: format!(
                "bus:{}:{}",
                diagnostic.bus_id.unwrap_or_default(),
                diagnostic.tap_id.unwrap_or_default()
            ),
            object: diagnostic
                .tap_id
                .or(diagnostic.bus_id)
                .and_then(|id| model.schematic.document().object_name(id))
                .unwrap_or_else(|| "A bus".to_owned()),
            before: diagnostic.message.clone(),
            proposed: "Check the bus name, the tap's bits and what the tap joins".to_owned(),
            identity_effect: "Bit names stay as they are".to_owned(),
            validation: "Fixed by hand on the sheet".to_owned(),
            action: None,
            endpoint_choices: Vec::new(),
            reveal,
        });
    }
    repairs
}

fn endpoint_choices_for_violation(
    model: &Model,
    nets: &[DesignNet],
    violation: &DrcViolation,
) -> Vec<EndpointChoice> {
    if !matches!(
        violation.violation_type,
        DrcViolationType::UnconnectedPin
            | DrcViolationType::UndrivenInput
            | DrcViolationType::UnloadedOutput
    ) {
        return Vec::new();
    }
    // The finding names the terminal, so the pin is read off the location.
    // Reading it out of a related-items position would keep working after the
    // location changed shape and quietly offer endpoints for the wrong pin.
    let DrcLocation::Terminal {
        component_id: id,
        pin_name,
        ..
    } = &violation.location
    else {
        return Vec::new();
    };
    let Some(component) = model
        .schematic
        .document()
        .components
        .iter()
        .find(|component| component.id == *id)
    else {
        return Vec::new();
    };
    let symbol_context = model.schematic_symbol_context();
    let Some(start) = symbol_context
        .named_terminal_points(component)
        .into_iter()
        .find_map(|(name, point)| (name == *pin_name).then_some(point))
    else {
        return Vec::new();
    };

    // A destination is a net or a pin on this sheet, named as the reader
    // knows it and offered nearest first: the wire a repair wants is almost
    // always to something close by. A net is reached at its nearest drawn
    // point; the pin's own net and the pin itself are not destinations.
    let document = model.schematic.document();
    let on_sheet =
        |id| crate::schematic::view::sheet_visibility::object_is_on_active_sheet(model, id);
    let millimetres = |point: Point| {
        let dx = f64::from(point.x) - f64::from(start.x);
        let dy = f64::from(point.y) - f64::from(start.y);
        dx.hypot(dy) / rspice_schematic_editor::view::drawing_sheet::DRAWING_SHEET_UNITS_PER_MM
    };
    let nearest = |points: &mut dyn Iterator<Item = Point>| {
        points.min_by(|left, right| millimetres(*left).total_cmp(&millimetres(*right)))
    };
    let wires_by_id: HashMap<u64, &crate::state::Wire> = document
        .wires
        .iter()
        .filter(|wire| on_sheet(wire.id))
        .map(|wire| (wire.id, wire))
        .collect();
    let mut labels_by_name = HashMap::<String, Vec<Point>>::new();
    for label in document
        .net_labels
        .iter()
        .filter(|label| on_sheet(label.id))
    {
        labels_by_name
            .entry(label.name.to_ascii_lowercase())
            .or_default()
            .push(label.pos);
    }
    let mut choices = Vec::<(f64, EndpointChoice)>::new();
    let mut offer = |name: String, destination: Point| {
        if destination == start {
            return;
        }
        let distance = millimetres(destination);
        choices.push((
            distance,
            EndpointChoice {
                label: format!("{name} \u{00b7} {distance:.1} mm"),
                name,
                start,
                destination,
            },
        ));
    };
    for net in nets {
        if net
            .terminals
            .iter()
            .any(|terminal| terminal.component_id == *id && terminal.pin == *pin_name)
        {
            continue;
        }
        let wires = net.wire_ids.iter().filter_map(|id| wires_by_id.get(id));
        let labels = labels_by_name
            .get(&net.name.to_ascii_lowercase())
            .into_iter()
            .flatten()
            .copied();
        let mut points = wires
            .flat_map(|wire| {
                wire.points.windows(2).map(|segment| {
                    rspice_design::schematic::bus::nearest_lattice_point_on_segment(
                        start, segment[0], segment[1],
                    )
                })
            })
            .chain(labels);
        if let Some(point) = nearest(&mut points) {
            offer(format!("Net {}", net.name), point);
        }
    }
    for other in &document.components {
        if !on_sheet(other.id) {
            continue;
        }
        for (name, point) in symbol_context.named_terminal_points(other) {
            if other.id == *id && name == *pin_name {
                continue;
            }
            offer(format!("Pin {}.{name}", other.name), point);
        }
    }
    choices
        .sort_by(|(left, a), (right, b)| left.total_cmp(right).then_with(|| a.name.cmp(&b.name)));
    choices.into_iter().map(|(_, choice)| choice).collect()
}

/// The declaration is used exactly as authored: `DATA[7:0]` and `DATA[0:7]`
/// are different vectors, and no project policy reorders one into the other.
fn validate_bus_draft(dialog: &ConnectivityManagerDialogState) -> Result<BusDeclaration, String> {
    let declaration =
        BusDeclaration::parse(dialog.bus_declaration.trim()).map_err(|error| error.to_string())?;
    declaration.validate().map_err(|error| error.to_string())?;
    Ok(declaration)
}

/// What the connectivity and bus manager draws over and changes: its draft,
/// the model and the session. Revealing a finding, opening the source map and
/// applying reviewed repairs reach the whole session, so it hands those to
/// the shell.
pub struct ConnectivityManager<'a> {
    pub dialog: &'a mut ConnectivityManagerDialogState,
    pub model: &'a mut Model,
    pub workbench: &'a mut WorkbenchSession,
}

impl ConnectivityManager<'_> {
    pub fn render_connectivity_manager_dialog(&mut self, ctx: &Context) {
        if !self.dialog.open {
            return;
        }
        let page = self.dialog.page;
        let stale = connectivity_authority_error(self.dialog, self.model);
        let write_allowed = !self.model.schematic.session.read_only
            && !self.model.active_view_read_only()
            && !self.model.safe_mode.project_read_only();
        let selected = self.dialog.selected_actionable_count();
        let bus_validation = validate_bus_draft(self.dialog);
        let policy_changed = self.dialog.policy_changed();
        let policy_authority_error = validate_policy_authority(
            &self.dialog.policy,
            &self.model.workspace.content.connectivity,
        )
        .err();
        let primary_enabled = match page {
            ConnectivityManagerPage::Manager => {
                write_allowed
                    && stale.is_none()
                    && policy_authority_error.is_none()
                    && (selected > 0 || policy_changed)
            }
            ConnectivityManagerPage::CreateBus => {
                write_allowed
                    && stale.is_none()
                    && policy_authority_error.is_none()
                    && bus_validation.is_ok()
            }
        };
        let error = self
            .dialog
            .error
            .as_deref()
            .or(stale.as_deref())
            .or(policy_authority_error.as_deref())
            .map(str::to_owned);
        if page == ConnectivityManagerPage::CreateBus {
            let status = if !write_allowed {
                Ok(Some(READ_ONLY_STATUS))
            } else {
                error.as_deref().map_or(Ok(None), Err)
            };
            let choice = create_bus_form(ctx, self.dialog, write_allowed, primary_enabled, status);
            match choice {
                DialogChoice::Primary => {
                    if let Err(error) = self.commit_connectivity_bus() {
                        self.dialog.error = Some(error);
                    }
                }
                DialogChoice::Ghost | DialogChoice::Cancelled => {
                    self.dialog.page = ConnectivityManagerPage::Manager;
                    self.dialog.error = None;
                    self.dialog.body_scroll_offset = 0.0;
                }
                DialogChoice::None | DialogChoice::Secondary => {}
            }
            return;
        }
        let mut body_scroll_offset = self.dialog.body_scroll_offset;
        let mut dialog = Dialog::new(TITLE, PRIMARY)
            .description(DESCRIPTION)
            .size(DialogSize::SimulationWorkflow)
            .initial_height(600.0)
            .flush_body()
            // The bands reach the frame's right edge; a withheld scroll
            // gutter left bare surface beside them.
            .overlay_scrollbar()
            .ghost("Close")
            .primary_enabled(primary_enabled)
            .primary_on_enter(false)
            .initial_focus(DialogInitialFocus::BodyControl)
            .body_scroll_offset(&mut body_scroll_offset);
        if !write_allowed {
            dialog = dialog.transaction_state(
                DialogTransactionTone::Progress,
                "Read-only",
                "The schematic or project is read-only: you can inspect and reveal, not repair.",
            );
        } else if let Some(error) = error.as_deref() {
            dialog = dialog.transaction_state(
                DialogTransactionTone::Error,
                "Nothing was applied",
                error,
            );
        } else if !primary_enabled {
            dialog = dialog.hint(NOTHING_TO_APPLY);
        }

        let mut action = ConnectivityBodyAction::None;
        let choice = dialog.show_with_initial_body_focus(ctx, |ui| {
            action = connectivity_manager_body(ui, self.dialog, write_allowed, stale.is_none());
            None
        });
        self.dialog.body_scroll_offset = body_scroll_offset;
        self.handle_connectivity_body_action(ctx, action);

        match choice {
            DialogChoice::Primary => {
                // The shell applies them straight after this step draws.
                self.workbench
                    .outbox
                    .push(DesignRequest::ApplyConnectivityRepairs);
            }
            DialogChoice::Ghost | DialogChoice::Cancelled => {
                self.dialog.close();
            }
            DialogChoice::None | DialogChoice::Secondary => {}
        }
    }

    fn handle_connectivity_body_action(&mut self, ctx: &Context, action: ConnectivityBodyAction) {
        match action {
            ConnectivityBodyAction::None => {}
            ConnectivityBodyAction::Refresh => {
                sync_active_schematic(self.model, self.workbench);
                let contract_changed =
                    *self.model.workspace.content.connectivity != self.dialog.contract_at_open;
                if contract_changed {
                    let contract = (*self.model.workspace.content.connectivity).clone();
                    self.dialog.policy = contract.policy.clone();
                    self.dialog.contract_at_open = contract;
                }
                let policy = self.dialog.policy.clone();
                let refreshed = build_report(self.model, &policy);
                let authority = SchematicEditAuthority::capture(&*self.model);
                let dialog = &mut *self.dialog;
                dialog.authority = Some(authority);
                match refreshed {
                    Ok(report) => {
                        // A tick stays on a repair the refresh still offers,
                        // so changing a policy does not undo the reader's
                        // choices; a repair that went takes its tick with it.
                        dialog.selected_repairs.retain(|id| {
                            report
                                .repairs
                                .iter()
                                .any(|repair| &repair.id == id && repair_takes_apply(repair))
                        });
                        // The refreshed table is the report of its refresh.
                        dialog.report = report;
                        dialog.error = None;
                        dialog.receipt = None;
                    }
                    Err(error) => {
                        dialog.selected_repairs.clear();
                        dialog.report = ConnectivityReport::default();
                        dialog.error = Some(error);
                        dialog.receipt = None;
                    }
                }
            }
            ConnectivityBodyAction::CreateBus => {
                self.dialog.page = ConnectivityManagerPage::CreateBus;
                self.dialog.error = None;
                self.dialog.body_scroll_offset = 0.0;
            }
            ConnectivityBodyAction::ReviewAliases => {
                let dialog = &mut *self.dialog;
                dialog.active_tab = ConnectivityManagerTab::RepairPreview;
                dialog.selected_repairs.extend(
                    dialog
                        .report
                        .repairs
                        .iter()
                        .filter(|repair| {
                            matches!(
                                repair.action.as_ref(),
                                Some(ConnectivityRepairAction::RenameLabels { .. })
                            )
                        })
                        .map(|repair| repair.id.clone()),
                );
            }
            ConnectivityBodyAction::Reveal(target) => {
                // The shell reveals it straight after this step draws.
                self.workbench
                    .outbox
                    .push(DesignRequest::RevealConnectivity(target));
            }
            ConnectivityBodyAction::RouteRepair(repair_id) => {
                if let Err(error) = self.arm_explicit_endpoint_route(ctx, &repair_id) {
                    self.dialog.error = Some(error);
                }
            }
            ConnectivityBodyAction::OpenSourceMap(net) => {
                // The shell opens it straight after this step draws.
                self.workbench
                    .outbox
                    .push(DesignRequest::OpenSourceMap(net));
            }
        }
    }

    fn commit_connectivity_bus(&mut self) -> Result<(), String> {
        connectivity_authority_error(self.dialog, self.model).map_or(Ok(()), Err)?;
        if self.model.schematic.session.read_only
            || self.model.active_view_read_only()
            || self.model.safe_mode.project_read_only()
        {
            return Err("The active schematic or project is read-only.".to_owned());
        }
        // Drawing a bus changes no policy: a policy edited on the Buses tab
        // reaches the project only through Apply.
        let declaration = validate_bus_draft(self.dialog)?;
        self.model.schematic.session.editor.bus_drawing.cancel();
        // The Bus strip holds the name, so every bus drawn next takes it.
        self.model.schematic.session.editor.bus_drawing.name = declaration.to_string();
        self.model.schematic.session.editor.tool = Tool::Bus;
        self.dialog.close();
        self.workbench.prompt(
            "Click where the bus starts and at each corner; double-click or press Enter to end it.",
        );
        Ok(())
    }

    fn arm_explicit_endpoint_route(
        &mut self,
        ctx: &Context,
        repair_id: &str,
    ) -> Result<(), String> {
        connectivity_authority_error(self.dialog, self.model).map_or(Ok(()), Err)?;
        if self.model.schematic.session.read_only
            || self.model.active_view_read_only()
            || self.model.safe_mode.project_read_only()
        {
            return Err("The active schematic or project is read-only.".to_owned());
        }
        let dialog = &*self.dialog;
        let repair = dialog
            .report
            .repairs
            .iter()
            .find(|repair| repair.id == repair_id)
            .ok_or_else(|| {
                "The selected repair no longer exists. Refresh the report.".to_owned()
            })?;
        let choice_index = dialog
            .endpoint_choices
            .get(repair_id)
            .copied()
            .ok_or_else(|| "Choose one destination endpoint first.".to_owned())?;
        let choice = repair
            .endpoint_choices
            .get(choice_index)
            .cloned()
            .ok_or_else(|| "The selected endpoint is stale. Refresh the report.".to_owned())?;
        if choice.start == choice.destination {
            return Err("The selected source and destination are identical.".to_owned());
        }
        let pin = repair.object.clone();
        self.dialog.close();
        self.workbench.workspace = crate::workbench::state::Workspace::Design;
        self.model.schematic.arm_tool(Tool::Wire);
        start_canvas_wire(self.model, self.workbench, ctx, choice.start);
        // Both ends in view: the wire starts at the pin and the reader
        // finishes it on the destination.
        self.model.schematic.session.editor.center_request = Some(Point::new(
            choice.start.x + (choice.destination.x - choice.start.x) / 2,
            choice.start.y + (choice.destination.y - choice.start.y) / 2,
        ));
        self.workbench.prompt(format!(
            "Wire started at {pin}. Finish it on {}.",
            choice.name
        ));
        Ok(())
    }
}

/// Reveal one finding of the report on the canvas, on the view it is drawn
/// on, over the navigation parts. The caller closes the manager first.
pub fn reveal_connectivity_target(mut navigation: NavigationParts<'_>, target: RevealTarget) {
    // The checks locate a terminal or a bare point on the projected
    // document, which lays later sheets a stride along; the canvas centres
    // on it where its sheet draws it. A point read off the editor's own
    // document is on no stride, and stays where it is.
    let sheets = navigation
        .model
        .workspace
        .content
        .design_management
        .sheet_catalog(
            &navigation
                .model
                .workspace
                .content
                .active_schematic_reference()
                .key(),
        );
    let target = match target {
        RevealTarget::Violation(DrcLocation::Terminal {
            component_id,
            component_name,
            pin_name,
            point,
        }) => RevealTarget::Violation(DrcLocation::Terminal {
            point: rspice_schematic_editor::view::violations::live_terminal_point(
                sheets,
                component_id,
                point,
            ),
            component_id,
            component_name,
            pin_name,
        }),
        RevealTarget::Violation(DrcLocation::Point { x, y }) => {
            let projected = Point::new(x as i32, y as i32);
            let point = sheets
                .and_then(|sheets| {
                    rspice_design::schematic::owned::projected_point_sheet(sheets, projected)
                })
                .map_or(projected, |(_, point)| point);
            RevealTarget::Violation(DrcLocation::Point {
                x: f64::from(point.x),
                y: f64::from(point.y),
            })
        }
        other => other,
    };
    navigation.wb.workspace = crate::workbench::state::Workspace::Design;
    if let RevealTarget::Label { view_key, .. } = &target
        && *view_key
            != navigation
                .model
                .workspace
                .content
                .active_schematic_reference()
                .key()
        && let Some(reference) = CellViewRef::from_key(view_key)
    {
        navigation.open_workspace_view(reference);
    }
    let schematic = &mut navigation.model.schematic;
    schematic.session.editor.selection.clear();
    schematic.session.editor.net_highlight.clear();
    match target {
        RevealTarget::Net {
            wire_ids,
            component_ids,
        } => {
            for id in &wire_ids {
                schematic.session.editor.selection.select_wire(*id);
            }
            for id in component_ids {
                schematic.session.editor.selection.select_component(id);
            }
            schematic
                .session
                .editor
                .net_highlight
                .highlight_wires(wire_ids.iter().copied().collect::<HashSet<_>>());
            schematic.session.editor.center_request = wire_ids
                .first()
                .and_then(|id| {
                    schematic
                        .document()
                        .wires
                        .iter()
                        .find(|wire| wire.id == *id)
                })
                .and_then(|wire| wire.points.first().copied());
        }
        RevealTarget::Bus { id, point } => {
            schematic.session.editor.selection.select_only_bus(id);
            schematic.session.editor.center_request = Some(point);
        }
        RevealTarget::BusTap { id, point } => {
            schematic.session.editor.selection.select_only_bus_tap(id);
            schematic.session.editor.center_request = Some(point);
        }
        RevealTarget::Label { view_key: _, id } => {
            schematic.session.editor.selection.select_only_net_label(id);
            schematic.session.editor.center_request = schematic
                .document()
                .net_labels
                .iter()
                .find(|label| label.id == id)
                .map(|label| label.pos);
        }
        RevealTarget::Violation(location) => {
            reveal_drc_location(schematic, &location);
        }
    }
}

/// Apply the repairs and the policy change the reader chose, as one undo
/// transaction validated on a copy, then rebuild the report over the result
/// and publish its check, which reads the runs as `review_markers` shows them.
pub fn apply_selected_connectivity_repairs(
    dialog: &mut ConnectivityManagerDialogState,
    model: &mut Model,
    workbench: &mut WorkbenchSession,
    runs: &mut RunsState,
    run_controller: &mut SimulationController,
    review_markers: SchematicReviewMarkerVisibility,
) -> Result<(), String> {
    connectivity_authority_error(dialog, model).map_or(Ok(()), Err)?;
    if model.schematic.session.read_only
        || model.active_view_read_only()
        || model.safe_mode.project_read_only()
    {
        return Err("The active schematic or project is read-only.".to_owned());
    }
    let selected = dialog
        .report
        .repairs
        .iter()
        .filter(|repair| repair_takes_apply(repair) && dialog.selected_repairs.contains(&repair.id))
        .cloned()
        .collect::<Vec<_>>();
    let policy_changed = dialog.policy_changed();
    if selected.is_empty() && !policy_changed {
        return Err(NOTHING_TO_APPLY.to_owned());
    }
    let mut contract_candidate = (*model.workspace.content.connectivity).clone();
    contract_candidate.policy = dialog.policy.clone();
    validate_policy_authority(&contract_candidate.policy, &contract_candidate)?;
    contract_candidate.validate()?;
    let mut candidate = model.schematic.clone();
    for repair in &selected {
        apply_repair_to_candidate(&mut candidate, repair)?;
    }
    let drawing = if selected.is_empty() {
        None
    } else {
        candidate.recalculate_runtime_state();
        validate_repair_candidate(model, &candidate, &contract_candidate.policy)?;
        let after = SchematicSnapshot::capture(candidate.document());
        if after.is_equal_document(model.schematic.document()) {
            return Err("The selected repairs no longer change the schematic.".to_owned());
        }
        Some(after)
    };
    // The repairs and the policy are one step of Undo: one press takes back
    // both, rather than putting the wires back and leaving the policy changed.
    let description = if selected.is_empty() {
        "change the connectivity policy"
    } else {
        "apply connectivity repairs"
    };
    let policy_candidate = DesignPolicy {
        connectivity: contract_candidate,
        ..DesignPolicy::of(model)
    };
    model.publish_design_policy(policy_candidate, drawing, description)?;
    if !selected.is_empty() {
        sync_active_schematic(model, workbench);
    }
    rspice_app_runs::workbench::preflight::invalidate(&mut runs.preflight, run_controller);
    model.netlist_artifacts.current_generation_input_digest = None;
    let count = selected.len();
    let policy = model.workspace.content.connectivity.policy.clone();
    let report = build_report(model, &policy)?;
    let authority = SchematicEditAuthority::capture(&*model);
    model
        .publish_active_design_check_result(report.drc.clone(), runs.slot_facts(review_markers))?;
    dialog.report = report;
    dialog.authority = Some(authority);
    dialog.contract_at_open = (*model.workspace.content.connectivity).clone();
    dialog.policy = policy;
    dialog.selected_repairs.clear();
    dialog.error = None;
    // How many repairs went in, and whether the policy went with them.
    // That the apply was one transaction is not the reader's problem.
    dialog.receipt = Some(format!(
        "Applied {count} repair{}{}",
        if count == 1 { "" } else { "s" },
        if policy_changed {
            " \u{00b7} policy saved"
        } else {
            ""
        }
    ));
    Ok(())
}

fn connectivity_authority_error(
    dialog: &ConnectivityManagerDialogState,
    model: &Model,
) -> Option<String> {
    let authority = dialog.authority.as_ref()?;
    let stale = !authority.matches_source(&crate::workbench::app::schematic_editor_request_source(
        model,
    )) || authority.view_path != model.workspace.content.active_view.display_path()
        || authority.grid_size != model.schematic.document().grid_size
        || authority.document_policy != model.schematic.document().document_policy
        || !authority
            .snapshot
            .is_equal_document(model.schematic.document())
        || dialog.contract_at_open != *model.workspace.content.connectivity;
    stale.then(|| {
        "The design changed after this report was made. Refresh it, then apply repairs or create the bus.".to_owned()
    })
}

fn validate_policy_authority(
    policy: &ConnectivityPolicy,
    contract: &ConnectivityContract,
) -> Result<(), String> {
    contract.validate()?;
    if policy.global_promotion == GlobalNetPromotionPolicy::TechnologyDefinedOnly {
        let catalog = contract.technology_global_nets.as_ref().ok_or_else(|| {
            "Technology-defined only needs the project's list of technology global nets.".to_owned()
        })?;
        catalog.validate()?;
    }
    if policy.alias_comparison == GlobalAliasComparisonPolicy::DialectCompatibility {
        let catalog = contract.dialect_aliases.as_ref().ok_or_else(|| {
            "Dialect compatibility needs the project's table of simulator aliases.".to_owned()
        })?;
        catalog.validate()?;
    }
    Ok(())
}

fn apply_repair_to_candidate(
    candidate: &mut SchematicState,
    repair: &ConnectivityRepair,
) -> Result<(), String> {
    match repair.action.as_ref() {
        Some(ConnectivityRepairAction::RenameLabels { canonical, labels }) => {
            candidate.repair_candidate_net_labels(canonical, labels)
        }
        None => Err("Choose what this repair routes to.".to_owned()),
    }
}

/// Refuse a candidate whose design check is worse than the design's.
///
/// The two are checked exactly alike: each projected under the policy the
/// repairs are applied with, and each checked as the open view's cellview.
/// Checking the candidate's editor buffer instead walked the placed bindings
/// where the baseline walked the configuration, so on a configured root a
/// master the configuration does not run was judged in one and not the other,
/// and every repair was refused for that master's findings.
fn validate_repair_candidate(
    model: &Model,
    candidate: &SchematicState,
    policy: &ConnectivityPolicy,
) -> Result<(), String> {
    // Typed-bus findings are counted on the same projected page as the check,
    // so neither count pairs buses drawn on different sheets.
    let check = |open_view: &SchematicState| -> Result<(DrcResult, usize), String> {
        let projection = projection_with_open_view(model, policy, open_view)?;
        let bus_findings = rspice_design::connectivity::bus::analyze_bus_connectivity(
            projected_open_view(model, &projection)?.document(),
        )
        .diagnostics
        .len();
        Ok((
            check_projected_open_view(model, policy, &projection)?,
            bus_findings,
        ))
    };
    let (before, before_bus) = check(&model.schematic)?;
    let (after, after_bus) = check(candidate)?;
    let before = severe_drc_counts(&before);
    for (kind, count) in severe_drc_counts(&after) {
        if count > before.get(&kind).copied().unwrap_or_default() {
            return Err(format!(
                "The repairs would add a new {} problem, so nothing was applied.",
                kind.description()
            ));
        }
    }
    if after_bus > before_bus {
        return Err("The repairs would add a new bus problem, so nothing was applied.".to_owned());
    }
    Ok(())
}

fn severe_drc_counts(result: &DrcResult) -> HashMap<DrcViolationType, usize> {
    let mut counts = HashMap::new();
    for violation in result
        .violations()
        .iter()
        .filter(|violation| violation.severity >= DrcSeverity::Error)
    {
        *counts.entry(violation.violation_type).or_default() += 1;
    }
    counts
}

fn reveal_drc_location(schematic: &mut SchematicState, location: &DrcLocation) {
    match location {
        DrcLocation::Point { x, y } => {
            schematic.session.editor.center_request = Some(Point::new(*x as i32, *y as i32));
        }
        DrcLocation::Component { id, .. } => {
            schematic
                .session
                .editor
                .selection
                .select_only_component(*id);
            schematic.session.editor.center_request = schematic
                .document()
                .components
                .iter()
                .find(|component| component.id == *id)
                .map(|component| component.pos);
        }
        DrcLocation::Wire { id } => {
            schematic.session.editor.selection.select_only_wire(*id);
            schematic.session.editor.center_request = schematic
                .document()
                .wires
                .iter()
                .find(|wire| wire.id == *id)
                .and_then(|wire| wire.points.first().copied());
        }
        DrcLocation::Bus { id } => {
            schematic.session.editor.selection.select_only_bus(*id);
            schematic.session.editor.center_request = schematic
                .document()
                .buses
                .iter()
                .find(|bus| bus.id == *id)
                .and_then(|bus| bus.points.first().copied());
        }
        DrcLocation::BusTap { id } => {
            schematic.session.editor.selection.select_only_bus_tap(*id);
            schematic.session.editor.center_request = schematic
                .document()
                .bus_taps
                .iter()
                .find(|tap| tap.id == *id)
                .map(|tap| tap.bus_point);
        }
        DrcLocation::NetLabel { name } => {
            if let Some(label) = schematic
                .document()
                .net_labels
                .iter()
                .find(|label| label.name == *name)
            {
                let id = label.id;
                let point = label.pos;
                schematic.session.editor.selection.select_only_net_label(id);
                schematic.session.editor.center_request = Some(point);
            }
        }
        DrcLocation::Node { net_name, .. } => {
            if let Some(label) = schematic
                .document()
                .net_labels
                .iter()
                .find(|label| label.name.eq_ignore_ascii_case(net_name))
            {
                let id = label.id;
                let point = label.pos;
                schematic.session.editor.selection.select_only_net_label(id);
                schematic.session.editor.center_request = Some(point);
            }
        }
        DrcLocation::Terminal {
            component_id,
            point,
            ..
        } => {
            schematic
                .session
                .editor
                .selection
                .select_only_component(*component_id);
            // The terminal, not the symbol origin: on a four-terminal device
            // they are different places.
            schematic.session.editor.center_request = Some(*point);
        }
        // A finding inside an instantiated cellview is not in this drawing.
        // Centring on the instance would claim it is.
        DrcLocation::InsideInstance { .. }
        | DrcLocation::Global
        | DrcLocation::CellView { .. }
        | DrcLocation::SymbolPin { .. } => {}
    }
}

fn connectivity_manager_body(
    ui: &mut Ui,
    dialog: &mut ConnectivityManagerDialogState,
    write_allowed: bool,
    current: bool,
) -> ConnectivityBodyAction {
    connectivity_tabs(ui, &mut dialog.active_tab);
    let action = match dialog.active_tab {
        ConnectivityManagerTab::Connectivity => connectivity_tab(ui, &dialog.report),
        ConnectivityManagerTab::Buses => buses_tab(ui, dialog, write_allowed, current),
        ConnectivityManagerTab::Globals => globals_tab(ui, dialog, write_allowed, current),
        ConnectivityManagerTab::RepairPreview => {
            repair_preview_tab(ui, dialog, write_allowed, current)
        }
    };
    ui.separator();
    let footer_action = inset_block(ui, 8, |ui| {
        let row = Vec2::new(ui.available_width(), Tokens::get(ui.ctx()).metrics.ctl_h);
        let action = ui
            .allocate_ui_with_layout(
                row,
                egui::Layout::left_to_right(egui::Align::Center),
                |ui| {
                    ui.label(
                        RichText::new(format!(
                            "{} nets \u{00b7} {} buses \u{00b7} {} DRC findings",
                            dialog.report.nets.len(),
                            dialog.report.buses.len(),
                            dialog.report.drc.violations().len()
                        ))
                        .color(Tokens::get(ui.ctx()).color.text_dim)
                        .monospace(),
                    );
                    ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
                        if Button::new("Refresh report").show(ui).clicked() {
                            return ConnectivityBodyAction::Refresh;
                        }
                        ConnectivityBodyAction::None
                    })
                    .inner
                },
            )
            .inner;
        if let Some(receipt) = dialog.receipt.as_deref() {
            ui.label(
                RichText::new(receipt)
                    .color(Tokens::get(ui.ctx()).color.text_dim)
                    .monospace(),
            );
        }
        action
    });
    if matches!(footer_action, ConnectivityBodyAction::Refresh) {
        footer_action
    } else {
        action
    }
}

fn connectivity_tabs(ui: &mut Ui, active: &mut ConnectivityManagerTab) {
    let labels = ConnectivityManagerTab::ALL.map(ConnectivityManagerTab::label);
    let mut index = ConnectivityManagerTab::ALL
        .iter()
        .position(|tab| tab == active)
        .unwrap_or_default();
    let _ = crate::ui::widgets::dialog_tabs(ui, "connectivity-tabs", &labels, &mut index);
    *active = ConnectivityManagerTab::ALL[index];
}

/// Content on the body's left edge and as far in from its right, `vertical`
/// points clear of the bands above and below it.
fn inset_block<R>(ui: &mut Ui, vertical: i8, add: impl FnOnce(&mut Ui) -> R) -> R {
    egui::Frame::NONE
        .inner_margin(egui::Margin::symmetric(DIALOG_BODY_INSET as i8, vertical))
        .show(ui, add)
        .inner
}

fn connectivity_tab(ui: &mut Ui, report: &ConnectivityReport) -> ConnectivityBodyAction {
    // No band: the tab above already names what the table is.
    inset_block(ui, 10, |ui| connectivity_table(ui, report))
}

fn connectivity_table(ui: &mut Ui, report: &ConnectivityReport) -> ConnectivityBodyAction {
    let mut action = ConnectivityBodyAction::None;
    ScrollArea::horizontal()
        .id_salt("connectivity-manager-nets")
        .show(ui, |ui| {
            Grid::new("connectivity-manager-net-grid")
                .num_columns(7)
                .striped(true)
                .min_col_width(90.0)
                .show(ui, |ui| {
                    table_header(
                        ui,
                        &[
                            "Signal",
                            "Kind",
                            "Endpoints",
                            "Drive / load / passive",
                            "Discipline",
                            "Issue",
                            "Show",
                        ],
                    );
                    if report.nets.is_empty() {
                        ui.label("No nets in the active schematic");
                        for _ in 1..7 {
                            ui.label("\u{2014}");
                        }
                        ui.end_row();
                    }
                    for net in &report.nets {
                        ui.monospace(&net.name);
                        ui.label(&net.kind);
                        ui.monospace(net.endpoints.to_string());
                        ui.monospace(format!("{} / {} / {}", net.drivers, net.loads, net.passive));
                        ui.monospace(&net.discipline);
                        ui.label(&net.issue);
                        // Reveal finds the net on the drawing; Netlist finds
                        // the first card the deck writes for it.
                        ui.horizontal(|ui| {
                            if Button::new("Reveal").show(ui).clicked() {
                                action = ConnectivityBodyAction::Reveal(RevealTarget::Net {
                                    wire_ids: net.wire_ids.clone(),
                                    component_ids: net.component_ids.clone(),
                                });
                            }
                            if Button::new("Netlist").show(ui).clicked() {
                                action = ConnectivityBodyAction::OpenSourceMap(net.name.clone());
                            }
                        });
                        ui.end_row();
                    }
                });
        });
    action
}

fn buses_tab(
    ui: &mut Ui,
    dialog: &mut ConnectivityManagerDialogState,
    write_allowed: bool,
    current: bool,
) -> ConnectivityBodyAction {
    band_header(ui, "Buses", None);
    let old_policy = dialog.policy.clone();
    let mut action = ConnectivityBodyAction::None;
    inset_block(ui, 10, |ui| {
        bus_table(ui, dialog, &mut action);
    });
    band_header(ui, "Bus policy", None);
    inset_block(ui, 10, |ui| {
        policy_combo(
            ui,
            "Width mismatch",
            &mut dialog.policy.width_mismatch,
            &BundleWidthMismatchPolicy::ALL,
            BundleWidthMismatchPolicy::label,
        );
        ui.add_space(8.0);
        if Button::new("Create bus\u{2026}")
            .enabled(write_allowed && current)
            .show(ui)
            .clicked()
        {
            action = ConnectivityBodyAction::CreateBus;
        }
    });
    if old_policy != dialog.policy && matches!(action, ConnectivityBodyAction::None) {
        action = ConnectivityBodyAction::Refresh;
    }
    action
}

fn bus_table(
    ui: &mut Ui,
    dialog: &ConnectivityManagerDialogState,
    action: &mut ConnectivityBodyAction,
) {
    ScrollArea::horizontal()
        .id_salt("connectivity-manager-buses")
        .show(ui, |ui| {
            Grid::new("connectivity-manager-bus-grid")
                .num_columns(7)
                .striped(true)
                .min_col_width(88.0)
                .show(ui, |ui| {
                    table_header(
                        ui,
                        &[
                            "Bus",
                            "Members",
                            "Taps",
                            "Taps to",
                            "Direction",
                            "Status",
                            "Action",
                        ],
                    );
                    if dialog.report.buses.is_empty() {
                        ui.label("No buses in the active schematic");
                        for _ in 1..7 {
                            ui.label("\u{2014}");
                        }
                        ui.end_row();
                    }
                    for bus in &dialog.report.buses {
                        ui.monospace(&bus.declaration);
                        ui.monospace(bus.members.to_string());
                        ui.monospace(bus.taps.to_string());
                        ui.label(&bus.mapping);
                        ui.monospace(&bus.direction);
                        status_label(ui, &bus.status, bus.status == "valid");
                        if Button::new("Reveal").show(ui).clicked() {
                            *action = ConnectivityBodyAction::Reveal(bus.reveal.clone());
                        }
                        ui.end_row();
                    }
                });
        });
}

fn globals_tab(
    ui: &mut Ui,
    dialog: &mut ConnectivityManagerDialogState,
    write_allowed: bool,
    current: bool,
) -> ConnectivityBodyAction {
    band_header(ui, "Global nets", None);
    let mut action = ConnectivityBodyAction::None;
    inset_block(ui, 10, |ui| global_table(ui, dialog, &mut action));
    band_header(ui, "Global-net policy", None);
    let old_policy = dialog.policy.clone();
    inset_block(ui, 10, |ui| {
        policy_combo(
            ui,
            "Promotion",
            &mut dialog.policy.global_promotion,
            &GlobalNetPromotionPolicy::ALL,
            GlobalNetPromotionPolicy::label,
        );
        policy_combo(
            ui,
            "Alias comparison",
            &mut dialog.policy.alias_comparison,
            &GlobalAliasComparisonPolicy::ALL,
            GlobalAliasComparisonPolicy::label,
        );
        ui.add_space(8.0);
        if Button::new("Review global aliases\u{2026}")
            .enabled(
                write_allowed
                    && current
                    && dialog.report.repairs.iter().any(|repair| {
                        matches!(
                            repair.action.as_ref(),
                            Some(ConnectivityRepairAction::RenameLabels { .. })
                        )
                    }),
            )
            .show(ui)
            .clicked()
        {
            action = ConnectivityBodyAction::ReviewAliases;
        }
    });
    if old_policy != dialog.policy {
        action = ConnectivityBodyAction::Refresh;
    }
    action
}

fn global_table(
    ui: &mut Ui,
    dialog: &ConnectivityManagerDialogState,
    action: &mut ConnectivityBodyAction,
) {
    ScrollArea::horizontal()
        .id_salt("connectivity-manager-globals")
        .show(ui, |ui| {
            Grid::new("connectivity-manager-global-grid")
                .num_columns(6)
                .striped(true)
                .min_col_width(96.0)
                .show(ui, |ui| {
                    table_header(
                        ui,
                        &["Global net", "Aliases", "Pins", "Scope", "State", "Action"],
                    );
                    if dialog.report.globals.is_empty() {
                        ui.label("No global labels in the active schematic");
                        for _ in 1..6 {
                            ui.label("\u{2014}");
                        }
                        ui.end_row();
                    }
                    for global in &dialog.report.globals {
                        ui.monospace(&global.canonical);
                        ui.monospace(if global.aliases.is_empty() {
                            "\u{2014}".to_owned()
                        } else {
                            global.aliases.join(", ")
                        });
                        ui.monospace(global.pins.to_string());
                        ui.label(&global.scope);
                        status_label(ui, &global.state, global.state == "resolved");
                        if let Some(label) = global.declarations.first()
                            && Button::new("Reveal").show(ui).clicked()
                        {
                            *action = ConnectivityBodyAction::Reveal(RevealTarget::Label {
                                view_key: label.view_key.clone(),
                                id: label.id,
                            });
                        }
                        ui.end_row();
                    }
                });
        });
}

fn repair_preview_tab(
    ui: &mut Ui,
    dialog: &mut ConnectivityManagerDialogState,
    write_allowed: bool,
    current: bool,
) -> ConnectivityBodyAction {
    band_header(ui, "Repairs", None);
    let mut action = ConnectivityBodyAction::None;
    inset_block(ui, 10, |ui| {
        ScrollArea::horizontal()
            .id_salt("connectivity-manager-repairs")
            .show(ui, |ui| {
                Grid::new("connectivity-manager-repair-grid")
                    .num_columns(7)
                    .striped(true)
                    .min_col_width(92.0)
                    .show(ui, |ui| {
                        table_header(
                            ui,
                            &[
                                "Apply",
                                "Object",
                                "Now",
                                "Repair",
                                "Effect on nets",
                                "Check",
                                "Action",
                            ],
                        );
                        if dialog.report.repairs.is_empty() {
                            ui.label("\u{2014}");
                            ui.label("No repairs needed");
                            for _ in 2..7 {
                                ui.label("\u{2014}");
                            }
                            ui.end_row();
                        }
                        for repair in &dialog.report.repairs {
                            let actionable =
                                repair.action.is_some() || !repair.endpoint_choices.is_empty();
                            let mut selected = dialog.selected_repairs.contains(&repair.id);
                            if !repair_takes_apply(repair) {
                                ui.label("\u{2014}");
                            } else if ui
                                .add_enabled(
                                    write_allowed && current,
                                    egui::Checkbox::without_text(&mut selected),
                                )
                                .changed()
                            {
                                if selected {
                                    dialog.selected_repairs.insert(repair.id.clone());
                                } else {
                                    dialog.selected_repairs.remove(&repair.id);
                                }
                            }
                            ui.monospace(&repair.object);
                            ui.label(&repair.before);
                            ui.label(&repair.proposed);
                            ui.label(&repair.identity_effect);
                            status_label(ui, &repair.validation, actionable);
                            ui.vertical(|ui| {
                                if Button::new("Reveal").show(ui).clicked() {
                                    action = ConnectivityBodyAction::Reveal(repair.reveal.clone());
                                }
                                if !repair.endpoint_choices.is_empty() {
                                    let selected = dialog
                                        .endpoint_choices
                                        .entry(repair.id.clone())
                                        .or_insert(0);
                                    *selected = (*selected)
                                        .min(repair.endpoint_choices.len().saturating_sub(1));
                                    crate::ui::widgets::Combo::from_id_salt((
                                        "repair-endpoint",
                                        &repair.id,
                                    ))
                                    .selected_text(&repair.endpoint_choices[*selected].label)
                                    .width(240.0)
                                    .show_ui(ui, |ui| {
                                        for (index, endpoint) in
                                            repair.endpoint_choices.iter().enumerate()
                                        {
                                            ui.selectable_value(selected, index, &endpoint.label);
                                        }
                                    });
                                    if Button::new("Route to endpoint")
                                        .enabled(write_allowed && current)
                                        .show(ui)
                                        .clicked()
                                    {
                                        action =
                                            ConnectivityBodyAction::RouteRepair(repair.id.clone());
                                    }
                                }
                            });
                            ui.end_row();
                        }
                    });
            });
        ui.horizontal(|ui| {
            if Button::new("Select all repairs")
                .enabled(write_allowed && current)
                .show(ui)
                .clicked()
            {
                dialog.selected_repairs.extend(
                    dialog
                        .report
                        .repairs
                        .iter()
                        .filter(|repair| repair_takes_apply(repair))
                        .map(|repair| repair.id.clone()),
                );
            }
        });
    });
    action
}

/// The bus declaration, asked as a command form before the bus router arms:
/// the width policy the bus will be checked against, the declaration, and
/// under it what the declaration declares or why it is refused.
fn create_bus_form(
    ctx: &Context,
    dialog: &mut ConnectivityManagerDialogState,
    write_allowed: bool,
    primary_enabled: bool,
    status: Result<Option<&str>, &str>,
) -> DialogChoice {
    let declaration_id = egui::Id::new("connectivity-create-bus-declaration");
    CommandForm::new(CREATE_BUS_TITLE, CREATE_BUS_PRIMARY)
        .describe("Name the bus with its bits, such as DATA[7:0], then draw it on the sheet.")
        .primary_enabled(primary_enabled)
        .fields_enabled(write_allowed)
        // A refusal of the declaration stands under it; the footer states
        // only what belongs to no field.
        .status(status)
        .show(ctx, |rows| {
            rows.value("Width mismatch", dialog.policy.width_mismatch.label());
            rows.text(
                "Name",
                declaration_id,
                &mut dialog.bus_declaration,
                "NAME[msb:lsb]",
            );
            let line = bus_declaration_line(&dialog.bus_declaration);
            rows.field_line(
                Some(declaration_id),
                match &line {
                    Ok(text) => FieldLine::Derived(text),
                    Err(text) => FieldLine::Error(text),
                },
            );
            Some(declaration_id)
        })
}

const CREATE_BUS_TITLE: &str = "Create bus";
pub const CREATE_BUS_PRIMARY: &str = "Draw bus";

/// The line under the declaration: what it declares, why it is refused, or,
/// while it is empty, what placement waits for.
fn bus_declaration_line(declaration: &str) -> Result<String, String> {
    let declaration = declaration.trim();
    if declaration.is_empty() {
        return Ok(BUS_DECLARATION_MISSING.to_owned());
    }
    BusDeclaration::parse(declaration)
        .map(|declaration| {
            format!(
                "{} members, {}",
                declaration.width(),
                match declaration.direction() {
                    BusDirection::Ascending => "ascending",
                    BusDirection::Descending => "descending",
                }
            )
        })
        .map_err(|error| error.to_string())
}

fn table_header(ui: &mut Ui, labels: &[&str]) {
    let color = Tokens::get(ui.ctx()).color.text_dim;
    for label in labels {
        ui.label(
            RichText::new(*label)
                .font(theme::sans(tokens::FS_0, FontWeight::SemiBold))
                .color(color),
        );
    }
    ui.end_row();
}

fn status_label(ui: &mut Ui, text: &str, good: bool) {
    let tokens = Tokens::get(ui.ctx());
    ui.label(
        RichText::new(text)
            .color(if good {
                tokens.color.ok
            } else {
                tokens.color.warn
            })
            .monospace(),
    );
}

fn policy_combo<T: Copy + PartialEq>(
    ui: &mut Ui,
    label: &str,
    value: &mut T,
    choices: &[T],
    display: impl Fn(T) -> &'static str,
) {
    Grid::new(ui.id().with(("policy", label)))
        .num_columns(2)
        .spacing(Vec2::new(20.0, 7.0))
        .show(ui, |ui| {
            ui.label(label);
            crate::ui::widgets::Combo::from_id_salt(("connectivity-policy", label))
                .selected_text(display(*value))
                .width(270.0)
                .show_ui(ui, |ui| {
                    for choice in choices {
                        ui.selectable_value(value, *choice, display(*choice));
                    }
                });
            ui.end_row();
        });
}

#[cfg(test)]
mod tests;
