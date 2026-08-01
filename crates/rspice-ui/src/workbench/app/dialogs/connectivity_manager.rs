//! Live connectivity, typed-bus, and global-net review for the schematic workspace.
//!
//! The manager is an inspection surface first. Its report is rebuilt from the
//! same hierarchy-aware extraction and DRC inputs used by netlisting. Mutations
//! are explicit, stale guarded, validated on a cloned schematic, and published
//! as one undo transaction.

use std::collections::{BTreeMap, BTreeSet, HashMap, HashSet};

use egui::{Context, Grid, RichText, ScrollArea, Ui, Vec2};

use crate::diagnostics::ConsoleMessage;
use crate::services::drc::{
    DrcConfig, DrcLocation, DrcResult, DrcSeverity, DrcViolation, DrcViolationType,
    run_drc_check_with_hierarchy_and_config,
};
use crate::simulation::netlist_gen::{
    DesignNet, HierarchySource, NetClass, design_nets_with_hierarchy,
};
use crate::state::{
    BundleDirection, BundleDiscipline, BundleExpansionPolicy, BundleIndexOrderPolicy,
    BundleWidthMismatchPolicy, BusDeclaration, BusDirection, BusTargetKind, CellViewRef,
    ConnectivityContract, ConnectivityPolicy, GlobalAliasComparisonPolicy,
    GlobalNetPromotionPolicy, LocalGlobalShadowingPolicy, NamedSignalBundleMember, Point,
    QualifiedNetReference, SchematicSnapshot, SchematicState, Tool,
};
use crate::ui::theme::{self, FontWeight};
use crate::ui::tokens::{self, Tokens};
use crate::ui::widgets::{
    Button, Dialog, DialogChoice, DialogInitialFocus, DialogSize, DialogTransactionTone,
};
use crate::workbench::design_system::section_header;

use crate::workbench::app::{RSpiceApp, SchematicEditAuthority};
use crate::workbench::app_state::AppState;

const EYEBROW: &str = "SCHEMATIC \u{00b7} ELECTRICAL INTENT \u{00b7} CROSS-SHEET RESOLUTION";
const TITLE: &str = "Connectivity, buses and signal bundles";
const DESCRIPTION: &str = "Inspect extracted nets, typed vector connectivity, global aliases, and reviewed corrective actions without changing electrical intent implicitly.";
const PRIMARY: &str = "Apply reviewed connectivity repairs";

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
            Self::Buses => "Buses & bundles",
            Self::Globals => "Global nets",
            Self::RepairPreview => "Repair preview",
        }
    }
}

#[derive(Debug, Clone, Copy, Default, PartialEq, Eq)]
pub(crate) enum ConnectivityManagerPage {
    #[default]
    Manager,
    CreateBundle,
}

#[derive(Debug, Clone)]
pub(crate) struct ConnectivityManagerDialogState {
    pub(crate) open: bool,
    pub(crate) page: ConnectivityManagerPage,
    pub(crate) active_tab: ConnectivityManagerTab,
    pub(crate) authority: Option<SchematicEditAuthority>,
    report: ConnectivityReport,
    selected_repairs: BTreeSet<String>,
    contract_at_open: ConnectivityContract,
    pub(crate) policy: ConnectivityPolicy,
    pub(crate) bundle_declaration: String,
    pub(crate) bundle_name: String,
    pub(crate) bundle_members: String,
    pub(crate) bundle_direction: BundleDirection,
    pub(crate) bundle_discipline: BundleDiscipline,
    endpoint_choices: HashMap<String, usize>,
    pub(crate) error: Option<String>,
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
            bundle_declaration: "DATA[7:0]".to_owned(),
            bundle_name: "SENSE_DIFF".to_owned(),
            bundle_members: "p=user/top/schematic::SENSE_P\nn=user/top/schematic::SENSE_N"
                .to_owned(),
            bundle_direction: BundleDirection::default(),
            bundle_discipline: BundleDiscipline::default(),
            endpoint_choices: HashMap::new(),
            error: None,
            receipt: None,
            body_scroll_offset: 0.0,
        }
    }
}

impl ConnectivityManagerDialogState {
    fn close(&mut self) {
        *self = Self::default();
    }

    fn selected_actionable_count(&self) -> usize {
        self.report
            .repairs
            .iter()
            .filter(|repair| repair.action.is_some() && self.selected_repairs.contains(&repair.id))
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
    discipline: String,
    issue: String,
    wire_ids: Vec<u64>,
    component_ids: Vec<u64>,
}

#[derive(Debug, Clone)]
struct ConnectivityBusRow {
    reveal: Option<RevealTarget>,
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

#[derive(Debug, Clone, PartialEq, Eq)]
struct LabelOccurrence {
    view_key: String,
    id: u64,
    name: String,
    point: Point,
}

#[derive(Debug, Clone)]
struct EndpointChoice {
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

#[derive(Debug, Clone)]
enum ConnectivityRepairAction {
    RenameLabels {
        canonical: String,
        labels: Vec<(u64, String)>,
    },
}

#[derive(Debug, Clone)]
enum RevealTarget {
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
        point: Point,
    },
    Violation(DrcLocation),
}

#[derive(Default, Debug, Clone)]
enum ConnectivityBodyAction {
    #[default]
    None,
    Reveal(RevealTarget),
    Refresh,
    CreateBundle,
    ReviewAliases,
    RouteRepair(String),
    OpenSourceMap,
}

pub(crate) fn open_connectivity_manager(state: &mut AppState) {
    if !state.project_lifecycle.project_open {
        state.push_user_message(ConsoleMessage::warning(
            "Connectivity and bus manager requires an open project.",
        ));
        return;
    }
    if !matches!(
        state.workspace.active_view_type(),
        crate::state::ViewType::Schematic | crate::state::ViewType::Testbench
    ) {
        state.push_user_message(ConsoleMessage::warning(
            "Connectivity and bus manager requires a schematic or testbench view.",
        ));
        return;
    }
    crate::schematic::view::retain_selection_on_active_sheet(state);
    state.sync_active_schematic_to_workspace();
    let contract = state.workspace.connectivity.clone();
    let report = build_report(state, &contract.policy);
    state.dialogs.connectivity_manager = ConnectivityManagerDialogState {
        open: true,
        authority: Some(SchematicEditAuthority::capture(state)),
        report,
        contract_at_open: contract.clone(),
        policy: contract.policy,
        ..ConnectivityManagerDialogState::default()
    };
}

fn build_report(state: &AppState, policy: &ConnectivityPolicy) -> ConnectivityReport {
    let mut preview_contract = state.workspace.connectivity.clone();
    preview_contract.policy = policy.clone();
    let hierarchy = HierarchySource::from_workspace_with_connectivity(
        &state.library_manager,
        &state.workspace.schematic_buffers,
        &preview_contract,
    );
    let design_nets = design_nets_with_hierarchy(&state.schematic, &hierarchy);
    let drc = run_drc_check_with_hierarchy_and_config(
        &state.schematic,
        &hierarchy,
        connectivity_drc_config(state),
    );
    let bus_analysis =
        crate::schematic::bus_connectivity::analyze_bus_connectivity(&state.schematic);
    let nets = design_nets
        .iter()
        .map(|net| build_net_row(net, &state.schematic, &drc))
        .collect();
    let buses = state
        .schematic
        .buses
        .iter()
        .map(|bus| {
            let diagnostics = bus_analysis
                .diagnostics
                .iter()
                .filter(|diagnostic| diagnostic.bus_id == Some(bus.id))
                .map(|diagnostic| diagnostic.message.as_str())
                .collect::<Vec<_>>();
            let taps = state
                .schematic
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
                format!("{scalar} scalar \u{00b7} {} vector", taps.len() - scalar)
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
                reveal: Some(RevealTarget::Bus {
                    id: bus.id,
                    point: bus.points.first().copied().unwrap_or(Point::origin()),
                }),
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
        .chain(
            state
                .workspace
                .connectivity
                .named_bundles
                .iter()
                .map(|bundle| ConnectivityBusRow {
                    reveal: None,
                    declaration: bundle.name.clone(),
                    members: bundle.members.len(),
                    taps: 0,
                    mapping: "named".to_owned(),
                    direction: bundle.direction.label().to_owned(),
                    status: format!(
                        "{} \u{00b7} {} exact bindings",
                        bundle.discipline.label(),
                        bundle.members.len()
                    ),
                }),
        )
        .collect::<Vec<_>>();
    let globals = build_global_rows(state, policy);
    let mut repairs = build_global_repairs(state, policy, &globals);
    repairs.extend(build_manual_repairs(state, &drc, &bus_analysis));
    ConnectivityReport {
        nets,
        buses,
        globals,
        repairs,
        drc,
    }
}

fn connectivity_drc_config(state: &AppState) -> DrcConfig {
    let root = crate::state::CellViewRef::new(
        &state.workspace.project.root_library,
        &state.workspace.project.top_cell,
        "schematic",
    );
    DrcConfig {
        check_missing_ground: state.workspace.active_schematic_reference() == root,
        ..DrcConfig::default()
    }
}

fn build_net_row(
    net: &DesignNet,
    schematic: &SchematicState,
    drc: &DrcResult,
) -> ConnectivityNetRow {
    let (drivers, loads, discipline) = endpoint_contract(net, schematic);
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

fn endpoint_contract(net: &DesignNet, schematic: &SchematicState) -> (usize, usize, String) {
    let mut drivers = 0;
    let mut loads = 0;
    let mut disciplines = BTreeSet::new();
    for terminal in &net.terminals {
        let Some(component) = schematic
            .components
            .iter()
            .find(|component| component.id == terminal.component_id)
        else {
            continue;
        };
        if let Some(contract) = component.port_contract() {
            disciplines.insert(contract.discipline.keyword());
            match contract.direction {
                crate::state::PortDirection::Out => drivers += 1,
                crate::state::PortDirection::In => loads += 1,
                crate::state::PortDirection::InOut | crate::state::PortDirection::Supply => {
                    drivers += 1;
                    loads += 1;
                }
            }
        } else if matches!(
            component.kind,
            crate::state::ComponentType::VoltageSource | crate::state::ComponentType::CurrentSource
        ) {
            drivers += 1;
            disciplines.insert("electrical");
        } else {
            loads += 1;
            disciplines.insert("electrical");
        }
    }
    let discipline = if disciplines.is_empty() {
        "electrical".to_owned()
    } else {
        disciplines.into_iter().collect::<Vec<_>>().join(" / ")
    };
    (drivers, loads, discipline)
}

fn violation_targets_net(violation: &DrcViolation, net: &DesignNet) -> bool {
    match &violation.location {
        DrcLocation::Node { net_name } | DrcLocation::NetLabel { name: net_name } => {
            net_name.eq_ignore_ascii_case(&net.name)
        }
        DrcLocation::Wire { id } => net.wire_ids.contains(id),
        DrcLocation::Component { id, .. } => net
            .terminals
            .iter()
            .any(|terminal| terminal.component_id == *id),
        _ => false,
    }
}

fn build_global_rows(state: &AppState, policy: &ConnectivityPolicy) -> Vec<GlobalNetRow> {
    let mut preview_contract = state.workspace.connectivity.clone();
    preview_contract.policy = policy.clone();
    let contract = &preview_contract;
    let mut declarations = Vec::<LabelOccurrence>::new();
    let mut locals = Vec::<LabelOccurrence>::new();
    let mut nets_by_view = HashMap::<String, Vec<DesignNet>>::new();
    let hierarchy = HierarchySource::from_workspace_with_connectivity(
        &state.library_manager,
        &state.workspace.schematic_buffers,
        contract,
    );
    for (view_key, schematic) in &state.workspace.schematic_buffers {
        let nets = design_nets_with_hierarchy(schematic, &hierarchy);
        nets_by_view.insert(view_key.clone(), nets);
        for label in &schematic.net_labels {
            let occurrence = LabelOccurrence {
                view_key: view_key.clone(),
                id: label.id,
                name: label.name.clone(),
                point: label.pos,
            };
            match policy.global_promotion {
                GlobalNetPromotionPolicy::ExplicitReviewedDeclaration => {
                    if label.name.ends_with('!') {
                        declarations.push(occurrence);
                    } else {
                        locals.push(occurrence);
                    }
                }
                GlobalNetPromotionPolicy::TechnologyDefinedOnly => {
                    if contract
                        .technology_global_canonical_name(&label.name)
                        .is_some()
                        || label.name.ends_with('!')
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
        let shadow_blocked = !local_aliases.is_empty()
            && policy.local_shadowing == LocalGlobalShadowingPolicy::BlockAndIdentifyScope;
        rows.push(GlobalNetRow {
            canonical,
            aliases: local_aliases
                .iter()
                .map(|alias| format!("{} ({})", alias.name, alias.view_key))
                .collect(),
            pins,
            scope: if technology_only {
                contract.technology_global_nets.as_ref().map_or_else(
                    || "technology catalog unavailable".to_owned(),
                    |catalog| format!("technology · {}", catalog.authority),
                )
            } else {
                "project".to_owned()
            },
            state: if technology_only && !technology_authorized {
                "authored declaration is absent from the technology catalog".to_owned()
            } else if shadow_blocked {
                "local shadow requires review".to_owned()
            } else if local_aliases.is_empty() {
                "resolved".to_owned()
            } else {
                "explicit local shadow allowed".to_owned()
            },
            declarations: vec![declaration],
            local_aliases,
        });
    }
    rows
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
    state: &AppState,
    policy: &ConnectivityPolicy,
    globals: &[GlobalNetRow],
) -> Vec<ConnectivityRepair> {
    if policy.local_shadowing == LocalGlobalShadowingPolicy::AllowExplicitLocalNet
        || policy.global_promotion == GlobalNetPromotionPolicy::TechnologyDefinedOnly
    {
        return Vec::new();
    }
    let active_key = state.workspace.active_schematic_reference().key();
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
                point: label.point,
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
                proposed: format!("Rename reviewed aliases to {}", global.canonical),
                identity_effect: "Merge labels into one explicit global identity".to_owned(),
                validation: "Candidate DRC and bus graph recheck".to_owned(),
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
                proposed: "Open each exact source view and review its local identity".to_owned(),
                identity_effect: "No cross-document rewrite is inferred".to_owned(),
                validation: "Requires the source view's own undo transaction".to_owned(),
                action: None,
                endpoint_choices: Vec::new(),
                reveal: RevealTarget::Label {
                    view_key: external_aliases[0].view_key.clone(),
                    id: external_aliases[0].id,
                    point: external_aliases[0].point,
                },
            });
        }
    }
    repairs
}

fn build_manual_repairs(
    state: &AppState,
    drc: &DrcResult,
    bus_analysis: &crate::schematic::bus_connectivity::BusConnectivityAnalysis,
) -> Vec<ConnectivityRepair> {
    let mut repairs = Vec::new();
    for violation in drc.violations() {
        if !matches!(
            violation.violation_type,
            DrcViolationType::FloatingNode
                | DrcViolationType::UnconnectedPin
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
            identity_effect: "No automatic identity rewrite".to_owned(),
            validation: "Manual endpoint choice required".to_owned(),
            action: None,
            endpoint_choices: endpoint_choices_for_violation(state, violation),
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
                .map_or_else(|| "Bus".to_owned(), |id| format!("Bus tap #{id}")),
            before: diagnostic.message.clone(),
            proposed: "Review the exact bus declaration, selector, and destination".to_owned(),
            identity_effect: "Preserve typed member identities".to_owned(),
            validation: "Manual endpoint choice required".to_owned(),
            action: None,
            endpoint_choices: Vec::new(),
            reveal,
        });
    }
    repairs
}

fn endpoint_choices_for_violation(
    state: &AppState,
    violation: &DrcViolation,
) -> Vec<EndpointChoice> {
    if violation.violation_type != DrcViolationType::UnconnectedPin {
        return Vec::new();
    }
    let DrcLocation::Component { id, .. } = &violation.location else {
        return Vec::new();
    };
    let Some(pin_name) = violation.related_items.get(1) else {
        return Vec::new();
    };
    let Some(component) = state
        .schematic
        .components
        .iter()
        .find(|component| component.id == *id)
    else {
        return Vec::new();
    };
    let symbol_context = crate::schematic::view::SchematicSymbolContext::from_state(state);
    let Some(start) = symbol_context
        .named_terminal_points(component)
        .into_iter()
        .find_map(|(name, point)| (name == *pin_name).then_some(point))
    else {
        return Vec::new();
    };

    let mut destinations = BTreeMap::<(i32, i32), String>::new();
    for wire in &state.schematic.wires {
        for point in &wire.points {
            destinations
                .entry((point.x, point.y))
                .or_insert_with(|| format!("Wire #{} at {}, {}", wire.id, point.x, point.y));
        }
    }
    for junction in &state.schematic.junctions {
        destinations
            .entry((junction.pos.x, junction.pos.y))
            .or_insert_with(|| {
                format!(
                    "Junction #{} at {}, {}",
                    junction.id, junction.pos.x, junction.pos.y
                )
            });
    }
    for label in &state.schematic.net_labels {
        destinations
            .entry((label.pos.x, label.pos.y))
            .or_insert_with(|| format!("Net {} at {}, {}", label.name, label.pos.x, label.pos.y));
    }
    for other in &state.schematic.components {
        for (name, point) in symbol_context.named_terminal_points(other) {
            if other.id == *id && point == start {
                continue;
            }
            destinations
                .entry((point.x, point.y))
                .or_insert_with(|| format!("{}.{name} at {}, {}", other.name, point.x, point.y));
        }
    }
    destinations.remove(&(start.x, start.y));
    if destinations.len() > 512 {
        return Vec::new();
    }
    destinations
        .into_iter()
        .map(|((x, y), label)| EndpointChoice {
            label,
            start,
            destination: Point::new(x, y),
        })
        .collect()
}

#[derive(Debug, Clone)]
enum BundleDraft {
    Named {
        name: String,
        direction: BundleDirection,
        discipline: BundleDiscipline,
        members: Vec<NamedSignalBundleMember>,
    },
    Indexed(BusDeclaration),
}

fn validate_bundle_draft(
    dialog: &ConnectivityManagerDialogState,
    workspace: &crate::state::ProjectWorkspace,
) -> Result<BundleDraft, String> {
    match dialog.policy.expansion {
        BundleExpansionPolicy::PositionalMapping => {
            let mut declaration = BusDeclaration::parse(dialog.bundle_declaration.trim())
                .map_err(|error| error.to_string())?;
            match dialog.policy.index_order {
                BundleIndexOrderPolicy::PreserveDeclaration => {}
                BundleIndexOrderPolicy::MsbFirst
                    if declaration.direction() == BusDirection::Ascending =>
                {
                    std::mem::swap(&mut declaration.msb, &mut declaration.lsb);
                }
                BundleIndexOrderPolicy::LsbFirst
                    if declaration.direction() == BusDirection::Descending =>
                {
                    std::mem::swap(&mut declaration.msb, &mut declaration.lsb);
                }
                BundleIndexOrderPolicy::MsbFirst | BundleIndexOrderPolicy::LsbFirst => {}
            }
            declaration.validate().map_err(|error| error.to_string())?;
            Ok(BundleDraft::Indexed(declaration))
        }
        BundleExpansionPolicy::NamedMemberMapping => {
            if dialog.bundle_members.len() > 4 * 1024 * 1024 {
                return Err("Named member bindings exceed the 4 MiB draft limit.".to_owned());
            }
            let name = dialog.bundle_name.trim().to_owned();
            let mut members = Vec::new();
            let nets_by_view = workspace
                .schematic_buffers
                .iter()
                .map(|(view_key, schematic)| {
                    (
                        view_key.as_str(),
                        crate::simulation::netlist_gen::design_nets(schematic)
                            .into_iter()
                            .map(|net| net.name)
                            .collect::<HashSet<_>>(),
                    )
                })
                .collect::<HashMap<_, _>>();
            for (line_index, raw_line) in dialog.bundle_members.lines().enumerate() {
                let line = raw_line.trim();
                if line.is_empty() {
                    continue;
                }
                if members.len() >= crate::state::MAX_NAMED_BUNDLE_MEMBERS {
                    return Err(format!(
                        "Named bundle exceeds the {} member limit.",
                        crate::state::MAX_NAMED_BUNDLE_MEMBERS
                    ));
                }
                let (member_name, target) = line.split_once('=').ok_or_else(|| {
                    format!(
                        "Member line {} must use member=library/cell/view::net.",
                        line_index + 1
                    )
                })?;
                let (view_key, net_name) = target.trim().split_once("::").ok_or_else(|| {
                    format!(
                        "Member line {} must contain one exact qualified net after '='.",
                        line_index + 1
                    )
                })?;
                let view_key = view_key.trim();
                let net_name = net_name.trim();
                let nets = nets_by_view.get(view_key).ok_or_else(|| {
                    format!(
                        "Member line {} references missing schematic view '{view_key}'.",
                        line_index + 1
                    )
                })?;
                if !nets.contains(net_name) {
                    return Err(format!(
                        "Member line {} references missing exact net '{view_key}::{net_name}'.",
                        line_index + 1
                    ));
                }
                members.push(NamedSignalBundleMember {
                    name: member_name.trim().to_owned(),
                    target: QualifiedNetReference::new(view_key, net_name),
                });
            }
            let mut candidate = workspace.connectivity.clone();
            candidate.insert_named_bundle(
                name.clone(),
                dialog.bundle_direction,
                dialog.bundle_discipline,
                members.clone(),
            )?;
            Ok(BundleDraft::Named {
                name,
                direction: dialog.bundle_direction,
                discipline: dialog.bundle_discipline,
                members,
            })
        }
    }
}

fn cell_view_reference_from_key(key: &str) -> Option<CellViewRef> {
    let mut parts = key.split('/');
    let (Some(library), Some(cell), Some(view), None) =
        (parts.next(), parts.next(), parts.next(), parts.next())
    else {
        return None;
    };
    Some(CellViewRef::new(library, cell, view))
}

impl RSpiceApp {
    pub(in crate::workbench) fn render_connectivity_manager_dialog(&mut self, ctx: &Context) {
        if !self.state.dialogs.connectivity_manager.open {
            return;
        }
        let page = self.state.dialogs.connectivity_manager.page;
        let stale = connectivity_authority_error(&self.state);
        let write_allowed = !self.state.schematic.read_only
            && !self.state.active_view_read_only()
            && !self.state.workbench.safe_mode.project_read_only();
        let selected = self
            .state
            .dialogs
            .connectivity_manager
            .selected_actionable_count();
        let bundle_validation = validate_bundle_draft(
            &self.state.dialogs.connectivity_manager,
            &self.state.workspace,
        );
        let policy_changed = self.state.dialogs.connectivity_manager.policy_changed();
        let policy_authority_error = validate_policy_authority(
            &self.state.dialogs.connectivity_manager.policy,
            &self.state.workspace.connectivity,
        )
        .err();
        let primary_enabled = match page {
            ConnectivityManagerPage::Manager => {
                write_allowed
                    && stale.is_none()
                    && policy_authority_error.is_none()
                    && (selected > 0 || policy_changed)
            }
            ConnectivityManagerPage::CreateBundle => {
                write_allowed
                    && stale.is_none()
                    && policy_authority_error.is_none()
                    && bundle_validation.is_ok()
            }
        };
        let (title, primary, description) = match page {
            ConnectivityManagerPage::Manager => (TITLE, PRIMARY, DESCRIPTION),
            ConnectivityManagerPage::CreateBundle
                if self.state.dialogs.connectivity_manager.policy.expansion
                    == BundleExpansionPolicy::NamedMemberMapping =>
            {
                (
                    "Create signal bundle",
                    "Create bundle",
                    "Bind every named member to one exact qualified project net.",
                )
            }
            ConnectivityManagerPage::CreateBundle => (
                "Create indexed bus",
                "Begin bus placement",
                "Validate one durable indexed declaration, then place its vector geometry with the schematic bus router.",
            ),
        };
        let bundle_error = (page == ConnectivityManagerPage::CreateBundle)
            .then(|| bundle_validation.as_ref().err().cloned())
            .flatten();
        let error = self
            .state
            .dialogs
            .connectivity_manager
            .error
            .as_deref()
            .or(stale.as_deref())
            .or(policy_authority_error.as_deref())
            .or(bundle_error.as_deref())
            .map(str::to_owned);
        let mut body_scroll_offset = self.state.dialogs.connectivity_manager.body_scroll_offset;
        let mut dialog = Dialog::new(EYEBROW, title, primary)
            .description(description)
            .size(DialogSize::SimulationWorkflow)
            .initial_height(600.0)
            .flush_body()
            .ghost(if page == ConnectivityManagerPage::Manager {
                "Close"
            } else {
                "Cancel"
            })
            .primary_enabled(primary_enabled)
            .primary_on_enter(false)
            .initial_focus(DialogInitialFocus::BodyControl)
            .body_scroll_offset(&mut body_scroll_offset);
        if !write_allowed {
            dialog = dialog.transaction_state(
                DialogTransactionTone::Progress,
                "Inspection only",
                "The active schematic or project is read-only. Reports and reveal actions remain available; mutations are disabled.",
            );
        } else if let Some(error) = error.as_deref() {
            dialog = dialog.transaction_state(
                DialogTransactionTone::Error,
                "Connectivity transaction cannot continue",
                error,
            );
        }

        let mut action = ConnectivityBodyAction::None;
        let choice = dialog.show_with_initial_body_focus(ctx, |ui| {
            action = if page == ConnectivityManagerPage::Manager {
                connectivity_manager_body(
                    ui,
                    &mut self.state.dialogs.connectivity_manager,
                    write_allowed,
                    stale.is_none(),
                )
            } else {
                create_bundle_body(
                    ui,
                    &mut self.state.dialogs.connectivity_manager,
                    write_allowed,
                )
            };
            None
        });
        self.state.dialogs.connectivity_manager.body_scroll_offset = body_scroll_offset;
        self.handle_connectivity_body_action(action);

        match choice {
            DialogChoice::Primary => match page {
                ConnectivityManagerPage::Manager => {
                    if let Err(error) = self.apply_selected_connectivity_repairs() {
                        self.state.dialogs.connectivity_manager.error = Some(error);
                    }
                }
                ConnectivityManagerPage::CreateBundle => {
                    if let Err(error) = self.commit_connectivity_bundle() {
                        self.state.dialogs.connectivity_manager.error = Some(error);
                    }
                }
            },
            DialogChoice::Ghost | DialogChoice::Cancelled => {
                if page == ConnectivityManagerPage::Manager {
                    self.state.dialogs.connectivity_manager.close();
                } else {
                    self.state.dialogs.connectivity_manager.page = ConnectivityManagerPage::Manager;
                    self.state.dialogs.connectivity_manager.error = None;
                    self.state.dialogs.connectivity_manager.body_scroll_offset = 0.0;
                }
            }
            DialogChoice::None | DialogChoice::Secondary => {}
        }
    }

    fn handle_connectivity_body_action(&mut self, action: ConnectivityBodyAction) {
        match action {
            ConnectivityBodyAction::None => {}
            ConnectivityBodyAction::Refresh => {
                self.state.sync_active_schematic_to_workspace();
                let contract_changed = self.state.workspace.connectivity
                    != self.state.dialogs.connectivity_manager.contract_at_open;
                if contract_changed {
                    let contract = self.state.workspace.connectivity.clone();
                    self.state.dialogs.connectivity_manager.policy = contract.policy.clone();
                    self.state.dialogs.connectivity_manager.contract_at_open = contract;
                }
                let policy = self.state.dialogs.connectivity_manager.policy.clone();
                let report = build_report(&self.state, &policy);
                let authority = SchematicEditAuthority::capture(&self.state);
                let dialog = &mut self.state.dialogs.connectivity_manager;
                dialog.report = report;
                dialog.authority = Some(authority);
                dialog.selected_repairs.clear();
                dialog.error = None;
                dialog.receipt =
                    Some("Connectivity report refreshed from the active design.".into());
            }
            ConnectivityBodyAction::CreateBundle => {
                self.state.dialogs.connectivity_manager.page =
                    ConnectivityManagerPage::CreateBundle;
                self.state.dialogs.connectivity_manager.error = None;
                self.state.dialogs.connectivity_manager.body_scroll_offset = 0.0;
            }
            ConnectivityBodyAction::ReviewAliases => {
                let dialog = &mut self.state.dialogs.connectivity_manager;
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
            ConnectivityBodyAction::Reveal(target) => self.reveal_connectivity_target(target),
            ConnectivityBodyAction::RouteRepair(repair_id) => {
                if let Err(error) = self.arm_explicit_endpoint_route(&repair_id) {
                    self.state.dialogs.connectivity_manager.error = Some(error);
                }
            }
            ConnectivityBodyAction::OpenSourceMap => {
                self.state.dialogs.connectivity_manager.close();
                self.state.workbench.workspace = crate::workbench::state::Workspace::Netlist;
                self.state.workbench.navigator_visible = true;
                self.state.workbench.drawer = Some(crate::workbench::state::Drawer::Navigator);
                self.state.ui.netlist.active_document =
                    crate::workbench::documents::netlist_document::ActiveNetlistDocument::Generated;
                self.state.workbench.navigator_query = "source mapping".to_owned();
                if let Some(line) = self
                    .state
                    .ui
                    .netlist
                    .generated_document
                    .as_ref()
                    .and_then(|document| document.generated_artifact().source_map().first())
                    .map(|entry| entry.generated_line())
                {
                    self.state.ui.netlist.cursor_line = line.saturating_sub(1);
                    self.state.ui.netlist.requested_line = Some(line);
                }
            }
        }
    }

    fn reveal_connectivity_target(&mut self, target: RevealTarget) {
        self.state.dialogs.connectivity_manager.close();
        self.state.workbench.workspace = crate::workbench::state::Workspace::Design;
        if let RevealTarget::Label { view_key, .. } = &target
            && *view_key != self.state.workspace.active_schematic_reference().key()
            && let Some(reference) = cell_view_reference_from_key(view_key)
        {
            self.state.open_workspace_view(reference);
        }
        let schematic = &mut self.state.schematic;
        schematic.selection.clear();
        schematic.net_highlight.clear();
        match target {
            RevealTarget::Net {
                wire_ids,
                component_ids,
            } => {
                for id in &wire_ids {
                    schematic.selection.select_wire(*id);
                }
                for id in component_ids {
                    schematic.selection.select_component(id);
                }
                schematic
                    .net_highlight
                    .highlight_wires(wire_ids.iter().copied().collect::<HashSet<_>>());
                schematic.center_request = wire_ids
                    .first()
                    .and_then(|id| schematic.wires.iter().find(|wire| wire.id == *id))
                    .and_then(|wire| wire.points.first().copied());
            }
            RevealTarget::Bus { id, point } => {
                schematic.selection.select_only_bus(id);
                schematic.center_request = Some(point);
            }
            RevealTarget::BusTap { id, point } => {
                schematic.selection.select_only_bus_tap(id);
                schematic.center_request = Some(point);
            }
            RevealTarget::Label {
                view_key: _,
                id,
                point,
            } => {
                schematic.selection.select_only_net_label(id);
                schematic.center_request = Some(point);
            }
            RevealTarget::Violation(location) => {
                reveal_drc_location(schematic, &location);
            }
        }
    }

    fn apply_selected_connectivity_repairs(&mut self) -> Result<(), String> {
        connectivity_authority_error(&self.state).map_or(Ok(()), Err)?;
        if self.state.schematic.read_only
            || self.state.active_view_read_only()
            || self.state.workbench.safe_mode.project_read_only()
        {
            return Err("The active schematic or project is read-only.".to_owned());
        }
        let dialog = &self.state.dialogs.connectivity_manager;
        let selected = dialog
            .report
            .repairs
            .iter()
            .filter(|repair| {
                repair.action.is_some() && dialog.selected_repairs.contains(&repair.id)
            })
            .cloned()
            .collect::<Vec<_>>();
        let policy_changed = dialog.policy_changed();
        if selected.is_empty() && !policy_changed {
            return Err("Select at least one validated repair or change a policy.".to_owned());
        }
        let mut contract_candidate = self.state.workspace.connectivity.clone();
        contract_candidate.policy = dialog.policy.clone();
        validate_policy_authority(&contract_candidate.policy, &contract_candidate)?;
        contract_candidate.validate()?;
        let mut candidate = self.state.schematic.clone();
        for repair in &selected {
            apply_repair_to_candidate(&mut candidate, repair)?;
        }
        if !selected.is_empty() {
            candidate.recalculate_runtime_state();
            validate_repair_candidate(&self.state, &candidate, &dialog.report.drc)?;
            let after = SchematicSnapshot::capture(&candidate);
            let changed = self.state.schematic.with_undo(
                "apply reviewed connectivity repairs",
                move |schematic| {
                    after.apply(schematic);
                    schematic.recalculate_runtime_state();
                },
            );
            if !changed {
                return Err(
                    "The reviewed repairs no longer change the active schematic.".to_owned(),
                );
            }
            self.state.sync_active_schematic_to_workspace();
        }
        if policy_changed {
            self.state.workspace.connectivity = contract_candidate;
            self.state.workspace.project_metadata_dirty = true;
        }
        self.invalidate_simulation_preflight();
        self.state.ui.netlist.current_generation_input_digest = None;
        let count = selected.len();
        let policy = self.state.workspace.connectivity.policy.clone();
        let report = build_report(&self.state, &policy);
        let authority = SchematicEditAuthority::capture(&self.state);
        self.state
            .publish_active_design_check_result(report.drc.clone())?;
        self.state.dialogs.drc_results = Some(report.drc.clone());
        self.state.dialogs.drc_checked_version = self.state.schematic.topology_version();
        let dialog = &mut self.state.dialogs.connectivity_manager;
        dialog.report = report;
        dialog.authority = Some(authority);
        dialog.contract_at_open = self.state.workspace.connectivity.clone();
        dialog.policy = policy;
        dialog.selected_repairs.clear();
        dialog.error = None;
        dialog.receipt = Some(format!(
            "Applied {count} reviewed repair{}{}.",
            if count == 1 { "" } else { "s" },
            if policy_changed {
                " and saved the project connectivity policy"
            } else {
                " atomically; one Undo restores the prior schematic"
            }
        ));
        Ok(())
    }

    fn commit_connectivity_bundle(&mut self) -> Result<(), String> {
        connectivity_authority_error(&self.state).map_or(Ok(()), Err)?;
        if self.state.schematic.read_only
            || self.state.active_view_read_only()
            || self.state.workbench.safe_mode.project_read_only()
        {
            return Err("The active schematic or project is read-only.".to_owned());
        }
        let draft = validate_bundle_draft(
            &self.state.dialogs.connectivity_manager,
            &self.state.workspace,
        )?;
        let mut contract = self.state.workspace.connectivity.clone();
        contract.policy = self.state.dialogs.connectivity_manager.policy.clone();
        validate_policy_authority(&contract.policy, &contract)?;
        match draft {
            BundleDraft::Named {
                name,
                direction,
                discipline,
                members,
            } => {
                contract.insert_named_bundle(name.clone(), direction, discipline, members)?;
                contract.validate()?;
                self.state.workspace.connectivity = contract;
                self.state.workspace.project_metadata_dirty = true;
                self.invalidate_simulation_preflight();
                self.state.ui.netlist.current_generation_input_digest = None;
                let policy = self.state.workspace.connectivity.policy.clone();
                let report = build_report(&self.state, &policy);
                let authority = SchematicEditAuthority::capture(&self.state);
                let dialog = &mut self.state.dialogs.connectivity_manager;
                dialog.page = ConnectivityManagerPage::Manager;
                dialog.active_tab = ConnectivityManagerTab::Buses;
                dialog.report = report;
                dialog.authority = Some(authority);
                dialog.contract_at_open = self.state.workspace.connectivity.clone();
                dialog.policy = policy;
                dialog.error = None;
                dialog.receipt = Some(format!(
                    "Created named bundle '{name}' with exact qualified member bindings."
                ));
            }
            BundleDraft::Indexed(declaration) => {
                contract.validate()?;
                self.state.workspace.connectivity = contract;
                self.state.workspace.project_metadata_dirty = true;
                self.state.schematic.bus_drawing.cancel();
                self.state.schematic.bus_drawing.declaration = Some(declaration);
                self.state.schematic.tool = Tool::Bus;
                self.state.dialogs.connectivity_manager.close();
                self.state.push_user_message(ConsoleMessage::info(
                    "Typed bus placement armed. Click the first point, route the bundle, then finish the bus.",
                ));
            }
        }
        Ok(())
    }

    fn arm_explicit_endpoint_route(&mut self, repair_id: &str) -> Result<(), String> {
        connectivity_authority_error(&self.state).map_or(Ok(()), Err)?;
        if self.state.schematic.read_only
            || self.state.active_view_read_only()
            || self.state.workbench.safe_mode.project_read_only()
        {
            return Err("The active schematic or project is read-only.".to_owned());
        }
        let dialog = &self.state.dialogs.connectivity_manager;
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
            .ok_or_else(|| "Choose one exact destination endpoint first.".to_owned())?;
        let choice = repair
            .endpoint_choices
            .get(choice_index)
            .cloned()
            .ok_or_else(|| "The selected endpoint is stale. Refresh the report.".to_owned())?;
        if choice.start == choice.destination {
            return Err("The selected source and destination are identical.".to_owned());
        }
        self.state.dialogs.connectivity_manager.close();
        self.state.workbench.workspace = crate::workbench::state::Workspace::Design;
        self.state.schematic.tool = Tool::Wire;
        self.state.schematic.start_wire(choice.start);
        self.state.schematic.center_request = Some(choice.destination);
        self.state.push_user_message(ConsoleMessage::info(format!(
            "Wire routing armed from {}, {} to {}. The destination is centered; route deliberately and finish on that exact endpoint.",
            choice.start.x, choice.start.y, choice.label
        )));
        Ok(())
    }
}

fn connectivity_authority_error(state: &AppState) -> Option<String> {
    let authority = state.dialogs.connectivity_manager.authority.as_ref()?;
    let stale = authority.design_execution_epoch != state.design_execution_epoch
        || authority.active_schematic_epoch != state.active_schematic_epoch
        || authority.topology_version != state.schematic.topology_version()
        || authority.view_path != state.workspace.active_view.display_path()
        || authority.grid_size != state.schematic.grid_size
        || authority.document_policy != state.schematic.document_policy
        || !authority.snapshot.is_equal_state(&state.schematic)
        || state.dialogs.connectivity_manager.contract_at_open != state.workspace.connectivity;
    stale.then(|| {
        "The design changed after this report was extracted. Refresh the report before applying repairs or creating a bundle.".to_owned()
    })
}

fn validate_policy_authority(
    policy: &ConnectivityPolicy,
    contract: &ConnectivityContract,
) -> Result<(), String> {
    contract.validate()?;
    if policy.global_promotion == GlobalNetPromotionPolicy::TechnologyDefinedOnly {
        let catalog = contract.technology_global_nets.as_ref().ok_or_else(|| {
            "Technology-defined global promotion requires a project-owned technology global-net catalog."
                .to_owned()
        })?;
        catalog.validate()?;
    }
    if policy.alias_comparison == GlobalAliasComparisonPolicy::DialectCompatibility {
        let catalog = contract.dialect_aliases.as_ref().ok_or_else(|| {
            "Dialect-compatible alias comparison requires an exact project-owned simulator dialect alias table."
                .to_owned()
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
            crate::state::NetLabel::validate_name(canonical, candidate.document_policy.net_naming)
                .map_err(|error| format!("Canonical global name is invalid: {error}."))?;
            for (id, expected) in labels {
                let label = candidate
                    .net_labels
                    .iter_mut()
                    .find(|label| label.id == *id)
                    .ok_or_else(|| format!("Net label #{id} no longer exists."))?;
                if label.name != *expected {
                    return Err(format!(
                        "Net label #{id} changed from '{expected}' to '{}'. Refresh the report.",
                        label.name
                    ));
                }
                label.name.clone_from(canonical);
            }
            Ok(())
        }
        None => Err("The selected finding requires a manual endpoint decision.".to_owned()),
    }
}

fn validate_repair_candidate(
    state: &AppState,
    candidate: &SchematicState,
    baseline_drc: &DrcResult,
) -> Result<(), String> {
    let hierarchy = HierarchySource::from_workspace_with_connectivity(
        &state.library_manager,
        &state.workspace.schematic_buffers,
        &state.workspace.connectivity,
    );
    let candidate_drc = run_drc_check_with_hierarchy_and_config(
        candidate,
        &hierarchy,
        connectivity_drc_config(state),
    );
    let before = severe_drc_counts(baseline_drc);
    let after = severe_drc_counts(&candidate_drc);
    for (kind, count) in after {
        if count > before.get(&kind).copied().unwrap_or_default() {
            return Err(format!(
                "Candidate validation introduced a new {} finding; no changes were applied.",
                kind.description()
            ));
        }
    }
    let before_bus = crate::schematic::bus_connectivity::analyze_bus_connectivity(&state.schematic)
        .diagnostics
        .len();
    let after_bus = crate::schematic::bus_connectivity::analyze_bus_connectivity(candidate)
        .diagnostics
        .len();
    if after_bus > before_bus {
        return Err(
            "Candidate validation introduced a new typed-bus diagnostic; no changes were applied."
                .to_owned(),
        );
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
            schematic.center_request = Some(Point::new(*x as i32, *y as i32));
        }
        DrcLocation::Component { id, .. } => {
            schematic.selection.select_only_component(*id);
            schematic.center_request = schematic
                .components
                .iter()
                .find(|component| component.id == *id)
                .map(|component| component.pos);
        }
        DrcLocation::Wire { id } => {
            schematic.selection.select_only_wire(*id);
            schematic.center_request = schematic
                .wires
                .iter()
                .find(|wire| wire.id == *id)
                .and_then(|wire| wire.points.first().copied());
        }
        DrcLocation::Bus { id } => {
            schematic.selection.select_only_bus(*id);
            schematic.center_request = schematic
                .buses
                .iter()
                .find(|bus| bus.id == *id)
                .and_then(|bus| bus.points.first().copied());
        }
        DrcLocation::BusTap { id } => {
            schematic.selection.select_only_bus_tap(*id);
            schematic.center_request = schematic
                .bus_taps
                .iter()
                .find(|tap| tap.id == *id)
                .map(|tap| tap.bus_point);
        }
        DrcLocation::NetLabel { name } => {
            if let Some(label) = schematic
                .net_labels
                .iter()
                .find(|label| label.name == *name)
            {
                let id = label.id;
                let point = label.pos;
                schematic.selection.select_only_net_label(id);
                schematic.center_request = Some(point);
            }
        }
        DrcLocation::Node { net_name } => {
            if let Some(label) = schematic
                .net_labels
                .iter()
                .find(|label| label.name.eq_ignore_ascii_case(net_name))
            {
                let id = label.id;
                let point = label.pos;
                schematic.selection.select_only_net_label(id);
                schematic.center_request = Some(point);
            }
        }
        DrcLocation::Global | DrcLocation::SymbolPin { .. } => {}
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
    let footer_action = ui
        .horizontal(|ui| {
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
        })
        .inner;
    if let Some(receipt) = dialog.receipt.as_deref() {
        ui.label(
            RichText::new(receipt)
                .color(Tokens::get(ui.ctx()).color.ok)
                .monospace(),
        );
    }
    if matches!(footer_action, ConnectivityBodyAction::Refresh) {
        footer_action
    } else {
        action
    }
}

fn connectivity_tabs(ui: &mut Ui, active: &mut ConnectivityManagerTab) {
    let tokens = Tokens::get(ui.ctx());
    ui.horizontal(|ui| {
        ui.spacing_mut().item_spacing.x = 0.0;
        for tab in ConnectivityManagerTab::ALL {
            let selected = *active == tab;
            let response = ui.add_sized(
                Vec2::new(132.0, 34.0),
                egui::Button::selectable(selected, tab.label()),
            );
            if response.clicked() {
                *active = tab;
            }
            if selected {
                ui.painter().hline(
                    response.rect.x_range(),
                    response.rect.bottom() - 1.0,
                    egui::Stroke::new(2.0, tokens.color.accent),
                );
            }
        }
    });
}

fn connectivity_tab(ui: &mut Ui, report: &ConnectivityReport) -> ConnectivityBodyAction {
    section_header(
        ui,
        "Extracted connectivity",
        Some("active view \u{00b7} hierarchy-resolved pins"),
    );
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
                            "Drivers / loads",
                            "Discipline",
                            "Issue",
                            "Repair",
                        ],
                    );
                    if report.nets.is_empty() {
                        ui.label("No extracted scalar nets");
                        for _ in 1..7 {
                            ui.label("\u{2014}");
                        }
                        ui.end_row();
                    }
                    for net in &report.nets {
                        ui.monospace(&net.name);
                        ui.label(&net.kind);
                        ui.monospace(net.endpoints.to_string());
                        ui.monospace(format!("{} / {}", net.drivers, net.loads));
                        ui.monospace(&net.discipline);
                        ui.label(&net.issue);
                        if Button::new("Reveal").show(ui).clicked() {
                            action = ConnectivityBodyAction::Reveal(RevealTarget::Net {
                                wire_ids: net.wire_ids.clone(),
                                component_ids: net.component_ids.clone(),
                            });
                        }
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
    section_header(ui, "Typed buses and bundles", Some("exact declarations"));
    let old_policy = dialog.policy.clone();
    let mut action = ConnectivityBodyAction::None;
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
                            "Bundle",
                            "Members",
                            "Taps",
                            "Mapping",
                            "Direction",
                            "Status",
                            "Action",
                        ],
                    );
                    if dialog.report.buses.is_empty() {
                        ui.label("No typed buses in the active schematic");
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
                        status_label(
                            ui,
                            &bus.status,
                            bus.status == "valid" || bus.reveal.is_none(),
                        );
                        if let Some(reveal) = &bus.reveal {
                            if Button::new("Reveal").show(ui).clicked() {
                                action = ConnectivityBodyAction::Reveal(reveal.clone());
                            }
                        } else {
                            ui.label("Bound");
                        }
                        ui.end_row();
                    }
                });
        });
    ui.add_space(8.0);
    section_header(ui, "Bundle policy", None);
    policy_combo(
        ui,
        "Expansion",
        &mut dialog.policy.expansion,
        &BundleExpansionPolicy::ALL,
        BundleExpansionPolicy::label,
    );
    policy_combo(
        ui,
        "Index ordering",
        &mut dialog.policy.index_order,
        &BundleIndexOrderPolicy::ALL,
        BundleIndexOrderPolicy::label,
    );
    policy_combo(
        ui,
        "Width mismatch",
        &mut dialog.policy.width_mismatch,
        &BundleWidthMismatchPolicy::ALL,
        BundleWidthMismatchPolicy::label,
    );
    property_line(ui, "Discipline conversion", "Declared connect rule only");
    ui.horizontal(|ui| {
        if Button::new("Create bundle\u{2026}")
            .enabled(write_allowed && current)
            .show(ui)
            .clicked()
        {
            action = ConnectivityBodyAction::CreateBundle;
        }
    });
    if old_policy != dialog.policy && matches!(action, ConnectivityBodyAction::None) {
        action = ConnectivityBodyAction::Refresh;
    }
    action
}

fn globals_tab(
    ui: &mut Ui,
    dialog: &mut ConnectivityManagerDialogState,
    write_allowed: bool,
    current: bool,
) -> ConnectivityBodyAction {
    section_header(ui, "Explicit global nets", Some("project scope"));
    let mut action = ConnectivityBodyAction::None;
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
                        ui.label("No explicit global labels in the active schematic");
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
                            action = ConnectivityBodyAction::Reveal(RevealTarget::Label {
                                view_key: label.view_key.clone(),
                                id: label.id,
                                point: label.point,
                            });
                        }
                        ui.end_row();
                    }
                });
        });
    ui.add_space(8.0);
    section_header(ui, "Global-net policy", None);
    let old_policy = dialog.policy.clone();
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
    policy_combo(
        ui,
        "Local shadowing",
        &mut dialog.policy.local_shadowing,
        &LocalGlobalShadowingPolicy::ALL,
        LocalGlobalShadowingPolicy::label,
    );
    ui.horizontal(|ui| {
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
        if old_policy != dialog.policy {
            action = ConnectivityBodyAction::Refresh;
        }
    });
    action
}

fn repair_preview_tab(
    ui: &mut Ui,
    dialog: &mut ConnectivityManagerDialogState,
    write_allowed: bool,
    current: bool,
) -> ConnectivityBodyAction {
    section_header(
        ui,
        "Ordered corrective action",
        Some("one undo transaction"),
    );
    ui.horizontal(|ui| {
        for (step, text) in [
            ("1", "Detect"),
            ("2", "Review"),
            ("3", "Apply"),
            ("4", "Recheck"),
        ] {
            ui.label(
                RichText::new(format!("{step} \u{00b7} {text}"))
                    .monospace()
                    .color(Tokens::get(ui.ctx()).color.text_dim),
            );
            if step != "4" {
                ui.separator();
            }
        }
    });
    let mut action = ConnectivityBodyAction::None;
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
                            "Before",
                            "Proposed repair",
                            "Identity effect",
                            "Validation",
                            "Action",
                        ],
                    );
                    if dialog.report.repairs.is_empty() {
                        ui.label("\u{2014}");
                        ui.label("No connectivity repairs are currently required");
                        for _ in 2..7 {
                            ui.label("\u{2014}");
                        }
                        ui.end_row();
                    }
                    for repair in &dialog.report.repairs {
                        let actionable =
                            repair.action.is_some() || !repair.endpoint_choices.is_empty();
                        let mut selected = dialog.selected_repairs.contains(&repair.id);
                        if ui
                            .add_enabled(
                                actionable && write_allowed && current,
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
                                egui::ComboBox::from_id_salt(("repair-endpoint", &repair.id))
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
                                    action = ConnectivityBodyAction::RouteRepair(repair.id.clone());
                                }
                            }
                        });
                        ui.end_row();
                    }
                });
        });
    ui.horizontal(|ui| {
        if Button::new("Select validated repairs")
            .enabled(write_allowed && current)
            .show(ui)
            .clicked()
        {
            dialog.selected_repairs.extend(
                dialog
                    .report
                    .repairs
                    .iter()
                    .filter(|repair| repair.action.is_some())
                    .map(|repair| repair.id.clone()),
            );
        }
        if Button::new("Open source map\u{2026}").show(ui).clicked() {
            action = ConnectivityBodyAction::OpenSourceMap;
        }
    });
    action
}

fn create_bundle_body(
    ui: &mut Ui,
    dialog: &mut ConnectivityManagerDialogState,
    write_allowed: bool,
) -> ConnectivityBodyAction {
    section_header(ui, "Bundle declaration", Some("typed design object"));
    Grid::new("connectivity-create-bundle")
        .num_columns(2)
        .spacing(Vec2::new(18.0, 10.0))
        .show(ui, |ui| {
            ui.label("Expansion");
            ui.monospace(dialog.policy.expansion.label());
            ui.end_row();
            ui.label("Index ordering");
            ui.monospace(dialog.policy.index_order.label());
            ui.end_row();
            ui.label("Width policy");
            ui.monospace(dialog.policy.width_mismatch.label());
            ui.end_row();
            ui.label("Discipline conversion");
            ui.monospace("Declared connect rule only");
            ui.end_row();
            if dialog.policy.expansion == BundleExpansionPolicy::NamedMemberMapping {
                ui.label("Bundle name");
                ui.add_enabled(
                    write_allowed,
                    egui::TextEdit::singleline(&mut dialog.bundle_name)
                        .hint_text("SENSE_DIFF")
                        .desired_width(330.0),
                );
                ui.end_row();
                ui.label("Direction");
                enum_combo(
                    ui,
                    "bundle-direction",
                    &mut dialog.bundle_direction,
                    &BundleDirection::ALL,
                    BundleDirection::label,
                );
                ui.end_row();
                ui.label("Discipline");
                enum_combo(
                    ui,
                    "bundle-discipline",
                    &mut dialog.bundle_discipline,
                    &BundleDiscipline::ALL,
                    BundleDiscipline::label,
                );
                ui.end_row();
            } else {
                ui.label("Declaration");
                ui.add_enabled(
                    write_allowed,
                    egui::TextEdit::singleline(&mut dialog.bundle_declaration)
                        .hint_text("DATA[7:0]")
                        .desired_width(330.0),
                );
                ui.end_row();
            }
        });
    if dialog.policy.expansion == BundleExpansionPolicy::NamedMemberMapping {
        ui.label("Member bindings");
        ui.add_enabled(
            write_allowed,
            egui::TextEdit::multiline(&mut dialog.bundle_members)
                .hint_text("p=user/top/schematic::SENSE_P")
                .desired_width(f32::INFINITY)
                .desired_rows(6),
        );
        ui.label(
            RichText::new(
                "One member per line: member=library/cell/view::net. Every target must exist exactly.",
            )
            .color(Tokens::get(ui.ctx()).color.text_dim),
        );
    }
    ui.add_space(10.0);
    if dialog.policy.expansion == BundleExpansionPolicy::PositionalMapping {
        match BusDeclaration::parse(dialog.bundle_declaration.trim()) {
            Ok(declaration) => {
                status_label(
                    ui,
                    &format!(
                        "{} members \u{00b7} {} \u{00b7} placement creates one undoable typed bus",
                        declaration.width(),
                        match declaration.direction() {
                            BusDirection::Ascending => "ascending",
                            BusDirection::Descending => "descending",
                        }
                    ),
                    true,
                );
            }
            Err(error) => status_label(ui, &error.to_string(), false),
        }
    }
    ConnectivityBodyAction::None
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

fn property_line(ui: &mut Ui, label: &str, value: &str) {
    Grid::new(ui.id().with(label))
        .num_columns(2)
        .spacing(Vec2::new(20.0, 7.0))
        .show(ui, |ui| {
            ui.label(label);
            ui.monospace(value);
            ui.end_row();
        });
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
            egui::ComboBox::from_id_salt(("connectivity-policy", label))
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

fn enum_combo<T: Copy + PartialEq>(
    ui: &mut Ui,
    id: &'static str,
    value: &mut T,
    choices: &[T],
    display: impl Fn(T) -> &'static str,
) {
    egui::ComboBox::from_id_salt(("connectivity-enum", id))
        .selected_text(display(*value))
        .width(270.0)
        .show_ui(ui, |ui| {
            for choice in choices {
                ui.selectable_value(value, *choice, display(*choice));
            }
        });
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::state::{NetLabel, NetNamingPolicy, Wire};

    #[test]
    fn global_alias_repairs_are_derived_from_real_label_ids() {
        let mut state = AppState::default();
        state
            .schematic
            .net_labels
            .push(NetLabel::new(10, Point::new(0, 0), "VDD!"));
        state
            .schematic
            .net_labels
            .push(NetLabel::new(11, Point::new(1, 0), "VDD"));
        state.sync_active_schematic_to_workspace();
        let globals = build_global_rows(&state, &ConnectivityPolicy::default());
        let repairs = build_global_repairs(&state, &ConnectivityPolicy::default(), &globals);
        assert_eq!(repairs.len(), 1);
        match repairs[0].action.as_ref().expect("action") {
            ConnectivityRepairAction::RenameLabels { canonical, labels } => {
                assert_eq!(canonical, "VDD!");
                assert_eq!(labels, &vec![(11, "VDD".to_owned())]);
            }
        }
    }

    #[test]
    fn technology_catalog_promotes_exact_declared_aliases() {
        let mut state = AppState::default();
        state
            .schematic
            .net_labels
            .push(NetLabel::new(10, Point::new(0, 0), "VCC"));
        state.workspace.connectivity.technology_global_nets =
            Some(crate::state::TechnologyGlobalNetCatalog {
                authority: "demo-pdk@1.0".to_owned(),
                nets: vec![crate::state::ConnectivityAliasGroup {
                    canonical_name: "VDD".to_owned(),
                    aliases: vec!["VCC".to_owned()],
                }],
            });
        state.sync_active_schematic_to_workspace();
        let policy = ConnectivityPolicy {
            global_promotion: GlobalNetPromotionPolicy::TechnologyDefinedOnly,
            ..ConnectivityPolicy::default()
        };
        validate_policy_authority(&policy, &state.workspace.connectivity).unwrap();
        let globals = build_global_rows(&state, &policy);
        assert_eq!(globals.len(), 1);
        assert_eq!(globals[0].canonical, "VDD");
        assert_eq!(globals[0].state, "resolved");
        assert!(globals[0].scope.contains("demo-pdk@1.0"));
    }

    #[test]
    fn dialect_alias_policy_uses_the_project_owned_table() {
        let mut contract = ConnectivityContract {
            dialect_aliases: Some(crate::state::DialectAliasCatalog {
                authority: "dialect-2026".to_owned(),
                groups: vec![crate::state::ConnectivityAliasGroup {
                    canonical_name: "0".to_owned(),
                    aliases: vec!["GND".to_owned()],
                }],
            }),
            ..ConnectivityContract::default()
        };
        contract.policy.alias_comparison = GlobalAliasComparisonPolicy::DialectCompatibility;
        validate_policy_authority(&contract.policy, &contract).unwrap();
        assert!(alias_matches(
            "0",
            "GND!",
            GlobalAliasComparisonPolicy::DialectCompatibility,
            &contract
        ));
    }

    #[test]
    fn alias_candidate_rejects_stale_label_identity() {
        let mut state = SchematicState::default();
        state.document_policy.net_naming = NetNamingPolicy::StrictCaseSensitive;
        state
            .net_labels
            .push(NetLabel::new(3, Point::origin(), "OLD"));
        let repair = ConnectivityRepair {
            id: "global-alias:1".to_owned(),
            object: "VDD!".to_owned(),
            before: "VDD".to_owned(),
            proposed: String::new(),
            identity_effect: String::new(),
            validation: String::new(),
            action: Some(ConnectivityRepairAction::RenameLabels {
                canonical: "VDD!".to_owned(),
                labels: vec![(3, "VDD".to_owned())],
            }),
            endpoint_choices: Vec::new(),
            reveal: RevealTarget::Label {
                view_key: "user/top/schematic".to_owned(),
                id: 3,
                point: Point::origin(),
            },
        };
        assert!(apply_repair_to_candidate(&mut state, &repair).is_err());
        assert_eq!(state.net_labels[0].name, "OLD");
    }

    #[test]
    fn typed_bundle_validation_preserves_declared_direction() {
        let descending = BusDeclaration::parse("DATA[7:0]").expect("valid");
        let ascending = BusDeclaration::parse("DATA[0:7]").expect("valid");
        assert_eq!(descending.direction(), BusDirection::Descending);
        assert_eq!(ascending.direction(), BusDirection::Ascending);
        assert_eq!(descending.width(), 8);
        assert_eq!(ascending.width(), 8);
    }

    #[test]
    fn named_bundle_draft_requires_an_exact_existing_qualified_net() {
        let mut state = AppState::default();
        state
            .schematic
            .net_labels
            .push(NetLabel::new(10, Point::origin(), "SENSE_P"));
        state
            .schematic
            .wires
            .push(Wire::segment(11, Point::origin(), Point::new(20, 0)));
        state.sync_active_schematic_to_workspace();
        let mut dialog = ConnectivityManagerDialogState::default();
        dialog.bundle_name = "SENSE".to_owned();
        dialog.bundle_members = format!(
            "p={}::SENSE_P",
            state.workspace.active_schematic_reference().key()
        );
        assert!(validate_bundle_draft(&dialog, &state.workspace).is_ok());
        dialog.bundle_members = format!(
            "p={}::MISSING",
            state.workspace.active_schematic_reference().key()
        );
        assert!(validate_bundle_draft(&dialog, &state.workspace).is_err());
    }
}
