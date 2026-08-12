//! The netlist navigator: outline, diff, and source mapping.
//!
//! The outline is projected from the canonical netlist document rather than
//! from the editor buffer, so what it lists is what would actually be
//! simulated. Source mapping resolves a line back to the include and directive
//! that produced it, which is the only way a generated line can be traced to
//! the deck an engineer wrote.
//!
//! Every category the outline parser produces is reachable exactly once. The
//! five an engineer navigates constantly become disclosable structure groups
//! whose children are the declarations themselves; the remaining seven are
//! counted in the semantic index and jump to their first card. A category that
//! appeared in neither would be something the deck states and the navigator
//! hides.

use std::collections::BTreeSet;

use super::*;
use crate::state::{OutlineSection, OutlineSectionKind};
use crate::workbench::documents::netlist_document::ActiveNetlistDocument;

/// Width reserved for a disclosure caret. Root and include rows leave it
/// empty so their icons line up with the groups' rather than sliding left.
const NETLIST_OUTLINE_CARET_COLUMN: f32 = 13.0;
const NETLIST_OUTLINE_CARET_SIZE: f32 = 9.0;
/// Indent of a declaration under its group, past the guide line.
const NETLIST_OUTLINE_CHILD_INDENT: f32 = 12.0;

pub(super) fn netlist(ui: &mut Ui, app: &mut RSpiceApp) {
    if app.state.ui.netlist.active_document == ActiveNetlistDocument::GeneratedDiff {
        netlist_diff(ui, app);
        return;
    }
    let root_label = active_netlist_artifact_name(&app.state);
    let projection = NetlistNavigatorProjection::from_source(
        &app.state.simulation.netlist_content,
        &app.state.workbench.navigator_query,
        &root_label,
        app.state.ui.netlist.active_document == ActiveNetlistDocument::Generated,
        &app.state.workbench.netlist_outline_collapsed,
        &retained_include_states(&app.state),
    );
    let active_line = app.state.ui.netlist.cursor_line.saturating_add(1);
    let height = outline_row_height(ui, &app.state);

    let mut goto = None;
    let mut toggled = None;

    ScrollArea::vertical()
        .id_salt("workbench.netlist.navigator")
        .auto_shrink([false, false])
        .show(ui, |ui| {
            ui.set_width(ui.available_width());

            if projection.root_row.is_some() || !projection.groups.is_empty() {
                section_header(
                    ui,
                    "Structure",
                    Some(&format!("{} lines", projection.line_count)),
                );
            }
            if let Some(root) = &projection.root_row
                && netlist_outline_row(
                    ui,
                    OutlineRowVisual {
                        label: &root.label,
                        meta: root.meta.as_deref(),
                        icon: Some(netlist_outline_icon(root.kind)),
                        shape: NetlistOutlineRowShape::Leaf,
                        selected: root.contains_line(active_line),
                        enabled: root.target_line.is_some(),
                        height,
                    },
                )
                .clicked()
            {
                goto = root.target_line;
            }
            for group in &projection.groups {
                if outline_group_row(ui, group, active_line, height) {
                    toggled = Some(group.section);
                }
                if !group.expanded {
                    continue;
                }
                if group.children.is_empty() {
                    outline_child_note(ui, group.empty_note, height);
                    continue;
                }
                uniform_rows(ui, height, group.children.len(), |ui, index| {
                    let child = &group.children[index];
                    if netlist_outline_row(
                        ui,
                        OutlineRowVisual {
                            label: &child.label,
                            meta: child.meta.as_deref(),
                            icon: None,
                            shape: NetlistOutlineRowShape::Child,
                            selected: child.contains_line(active_line),
                            enabled: true,
                            height,
                        },
                    )
                    .clicked()
                    {
                        goto = Some(child.line);
                    }
                });
            }

            if !projection.include_rows.is_empty() {
                section_header(ui, "Includes", Some(netlist_dependency_status(&app.state)));
                for row in &projection.include_rows {
                    if netlist_outline_row(
                        ui,
                        OutlineRowVisual {
                            label: &row.label,
                            meta: row.meta.as_deref(),
                            icon: Some(netlist_outline_icon(row.kind)),
                            shape: NetlistOutlineRowShape::Leaf,
                            selected: row.contains_line(active_line),
                            enabled: row.target_line.is_some(),
                            height,
                        },
                    )
                    .clicked()
                    {
                        goto = row.target_line;
                    }
                }
            }

            if !projection.semantic_rows.is_empty() {
                section_header(
                    ui,
                    "Semantic index",
                    Some(&format!("{} directives", projection.semantic_cards)),
                );
                for row in &projection.semantic_rows {
                    if netlist_outline_row(
                        ui,
                        OutlineRowVisual {
                            label: row.label,
                            meta: Some(&row.meta),
                            icon: None,
                            shape: NetlistOutlineRowShape::Index,
                            selected: false,
                            enabled: true,
                            height,
                        },
                    )
                    .clicked()
                    {
                        goto = Some(row.line);
                    }
                }
            }

            if projection.show_source_mapping {
                section_header(ui, "Source mapping", None);
                netlist_source_mapping(ui, app, active_line);
            }

            if projection.is_empty() {
                muted(
                    ui,
                    "No symbol, directive, or source line matches this filter.",
                );
            }
        });

    if let Some(section) = toggled {
        // Collapsing is stored, expanding clears the store, so the default for
        // a category the engineer has never touched stays "disclosed".
        let collapsed = &mut app.state.workbench.netlist_outline_collapsed;
        if !collapsed.remove(&section) {
            collapsed.insert(section);
        }
    }
    if let Some(line) = goto {
        // Keep selection feedback immediate and hand the exact one-based
        // declaration to the editor's caret/scroll transaction for the next
        // document frame.
        app.state.ui.netlist.cursor_line = line.saturating_sub(1);
        app.state.ui.netlist.requested_line = Some(line);
    }
}

fn outline_row_height(ui: &Ui, state: &crate::workbench::AppState) -> f32 {
    if state.workbench.coarse_pointer || ui.ctx().content_rect().width() <= 820.0 {
        NETLIST_OUTLINE_TOUCH_ROW_HEIGHT
    } else {
        NETLIST_OUTLINE_ROW_HEIGHT
    }
}

fn outline_group_row(
    ui: &mut Ui,
    group: &NetlistOutlineGroup,
    active_line: usize,
    height: f32,
) -> bool {
    let response = netlist_outline_row(
        ui,
        OutlineRowVisual {
            label: &group.row.label,
            meta: group.row.meta.as_deref(),
            icon: Some(netlist_outline_icon(group.row.kind)),
            shape: NetlistOutlineRowShape::Group {
                expanded: group.expanded,
            },
            // A collapsed group stands in for the declaration holding the
            // caret. An expanded one does not: its child is on screen and
            // says so itself, and two selected rows claim two carets.
            selected: !group.expanded && group.row.contains_line(active_line),
            enabled: true,
            height,
        },
    );
    let expanded = group.expanded;
    ui.ctx().accesskit_node_builder(response.id, |node| {
        node.set_expanded(expanded);
    });
    response.clicked()
}

pub(super) fn active_netlist_artifact_name(state: &crate::workbench::AppState) -> String {
    match state.ui.netlist.active_document {
        ActiveNetlistDocument::Generated => "generated.sp".to_owned(),
        ActiveNetlistDocument::OwnedSource => state
            .workspace
            .netlist_descriptor
            .as_ref()
            .map(|descriptor| descriptor.artifact_name.clone())
            .or_else(|| {
                state
                    .workspace
                    .netlist_source_path
                    .as_deref()
                    .and_then(std::path::Path::file_name)
                    .map(|name| name.to_string_lossy().into_owned())
            })
            .unwrap_or_else(|| "owned-source.sp".to_owned()),
        ActiveNetlistDocument::GeneratedDiff => "generated.diff".to_owned(),
    }
}

pub(super) fn active_canonical_netlist_document(
    state: &crate::workbench::AppState,
) -> Option<&crate::state::NetlistDocument> {
    match state.ui.netlist.active_document {
        ActiveNetlistDocument::Generated => state.ui.netlist.generated_document.as_ref(),
        ActiveNetlistDocument::OwnedSource => state.ui.netlist.owned_document.as_ref(),
        ActiveNetlistDocument::GeneratedDiff => None,
    }
}

/// What the canonical document knows about each retained dependency, keyed by
/// the locator its own card wrote. The section header states one verdict for
/// the whole closure; without this the row that earned an `error` cannot be
/// told from the ones that did not.
pub(super) fn retained_include_states(
    state: &crate::workbench::AppState,
) -> Vec<(String, &'static str)> {
    active_canonical_netlist_document(state).map_or_else(Vec::new, |document| {
        document
            .dependencies()
            .iter()
            .map(|dependency| {
                (
                    dependency.locator().logical_identity().to_owned(),
                    match dependency.resolution() {
                        crate::state::DependencyResolution::Missing { .. } => "missing",
                        crate::state::DependencyResolution::Unresolved => "unresolved",
                        crate::state::DependencyResolution::Resolved { .. } => {
                            dependency.authority().label()
                        }
                    },
                )
            })
            .collect()
    })
}

pub(super) fn netlist_dependency_status(state: &crate::workbench::AppState) -> &'static str {
    let Some(document) = active_canonical_netlist_document(state) else {
        return "unavailable";
    };
    if document.dependencies().iter().any(|dependency| {
        matches!(
            dependency.resolution(),
            crate::state::DependencyResolution::Missing { .. }
        )
    }) {
        "error"
    } else if document.dependency_graph_is_sealed() {
        "resolved"
    } else {
        "unresolved"
    }
}

/// One changed region of a revision comparison.
#[derive(Debug, Clone, PartialEq, Eq)]
pub(super) struct DiffHunk {
    pub(super) label: String,
    pub(super) meta: String,
    /// One-based line of the hunk header inside the comparison document.
    pub(super) line: usize,
    /// Last line of the hunk inside the comparison document.
    pub(super) end_line: usize,
}

impl DiffHunk {
    fn contains_line(&self, line: usize) -> bool {
        (self.line..=self.end_line).contains(&line)
    }
}

pub(super) fn netlist_diff(ui: &mut Ui, app: &mut RSpiceApp) {
    let query = NetlistNavigatorQuery::new(&app.state.workbench.navigator_query);
    let hunks = diff_hunks(&app.state.ui.netlist.generated_diff_source);
    let (additions, removals) = diff_totals(&app.state.ui.netlist.generated_diff_source);
    let matching = hunks
        .iter()
        .filter(|hunk| query.matches_diff_hunk(hunk))
        .cloned()
        .collect::<Vec<_>>();
    let height = outline_row_height(ui, &app.state);
    let active_line = app.state.ui.netlist.cursor_line.saturating_add(1);
    let mut goto = None;

    ScrollArea::vertical()
        .id_salt("workbench.netlist.diff.navigator")
        .auto_shrink([false, false])
        .show(ui, |ui| {
            ui.set_width(ui.available_width());
            section_header(
                ui,
                "Revision comparison",
                Some(&format!("+{additions} -{removals}")),
            );
            if hunks.is_empty() {
                muted(ui, "The two revisions declare the same source.");
                return;
            }
            if matching.is_empty() {
                muted(ui, "No changed region matches this filter.");
                return;
            }
            uniform_rows(ui, height, matching.len(), |ui, index| {
                let hunk = &matching[index];
                if netlist_outline_row(
                    ui,
                    OutlineRowVisual {
                        label: &hunk.label,
                        meta: Some(&hunk.meta),
                        // Every row in a comparison is a comparison; an icon
                        // repeating the section header decorates and no more.
                        icon: None,
                        shape: NetlistOutlineRowShape::Index,
                        selected: hunk.contains_line(active_line),
                        enabled: true,
                        height,
                    },
                )
                .clicked()
                {
                    goto = Some(hunk.line);
                }
            });
        });

    if let Some(line) = goto {
        app.state.ui.netlist.cursor_line = line.saturating_sub(1);
        app.state.ui.netlist.requested_line = Some(line);
    }
}

/// The changed regions of a unified diff, addressed by their position in the
/// comparison document — that is the buffer the editor is showing, so a hunk
/// row navigates to the header the engineer can actually see.
pub(super) fn diff_hunks(diff: &str) -> Vec<DiffHunk> {
    let mut hunks = Vec::<DiffHunk>::new();
    let mut additions = 0usize;
    let mut removals = 0usize;
    let mut previous = 0usize;
    let finish = |hunks: &mut Vec<DiffHunk>, additions: usize, removals: usize, end_line: usize| {
        if let Some(last) = hunks.last_mut() {
            last.meta = format!("+{additions} -{removals}");
            last.end_line = end_line.max(last.line);
        }
    };
    for (zero_line, line) in diff.lines().enumerate() {
        previous = zero_line + 1;
        if let Some(range) = line.strip_prefix("@@") {
            finish(&mut hunks, additions, removals, zero_line);
            additions = 0;
            removals = 0;
            hunks.push(DiffHunk {
                label: diff_hunk_label(range),
                meta: String::new(),
                line: zero_line + 1,
                end_line: zero_line + 1,
            });
        } else if line.starts_with('+') && !line.starts_with("+++") {
            additions += 1;
        } else if line.starts_with('-') && !line.starts_with("---") {
            removals += 1;
        }
    }
    finish(&mut hunks, additions, removals, previous);
    hunks
}

/// Name a hunk by the lines it occupies in the newer revision, which is the
/// side an engineer is reading the comparison to understand.
fn diff_hunk_label(range: &str) -> String {
    let new_side = range
        .split_whitespace()
        .find_map(|token| token.strip_prefix('+'));
    let Some((start, count)) = new_side.map(|side| match side.split_once(',') {
        Some((start, count)) => (start.parse::<usize>().ok(), count.parse::<usize>().ok()),
        None => (side.parse::<usize>().ok(), Some(1)),
    }) else {
        return "Changed region".to_owned();
    };
    match (start, count) {
        (Some(start), Some(0)) => format!("Removed before line {start}"),
        (Some(start), Some(1)) => format!("Line {start}"),
        (Some(start), Some(count)) => format!("Lines {start}\u{2013}{}", start + count - 1),
        _ => "Changed region".to_owned(),
    }
}

pub(super) fn diff_totals(diff: &str) -> (usize, usize) {
    diff.lines().fold((0, 0), |(added, removed), line| {
        if line.starts_with('+') && !line.starts_with("+++") {
            (added + 1, removed)
        } else if line.starts_with('-') && !line.starts_with("---") {
            (added, removed + 1)
        } else {
            (added, removed)
        }
    })
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(super) enum NetlistNavigatorRowKind {
    Root,
    Parameters,
    Instances,
    Models,
    Analyses,
    Measurements,
    Include,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub(super) struct NetlistNavigatorRow {
    pub(super) kind: NetlistNavigatorRowKind,
    pub(super) label: String,
    pub(super) meta: Option<String>,
    pub(super) target_line: Option<usize>,
    pub(super) source_ranges: Vec<(usize, usize)>,
}

impl NetlistNavigatorRow {
    pub(super) fn contains_line(&self, line: usize) -> bool {
        self.source_ranges
            .iter()
            .any(|(start, end)| (*start..=*end).contains(&line))
    }
}

/// One declaration under a structure group.
#[derive(Debug, Clone, PartialEq, Eq)]
pub(super) struct NetlistOutlineChild {
    pub(super) label: String,
    pub(super) meta: Option<String>,
    pub(super) line: usize,
    pub(super) end_line: usize,
}

impl NetlistOutlineChild {
    fn contains_line(&self, line: usize) -> bool {
        (self.line..=self.end_line).contains(&line)
    }
}

/// A disclosable structure group and the declarations it holds.
#[derive(Debug, Clone, PartialEq, Eq)]
pub(super) struct NetlistOutlineGroup {
    pub(super) row: NetlistNavigatorRow,
    pub(super) section: OutlineSectionKind,
    pub(super) children: Vec<NetlistOutlineChild>,
    pub(super) expanded: bool,
    /// What the deck does not declare, said in the group's own place. An empty
    /// group and a filtered-out group must not look alike.
    pub(super) empty_note: &'static str,
}

/// A parsed category the structure tree does not promote to a group.
#[derive(Debug, Clone, PartialEq, Eq)]
pub(super) struct NetlistSemanticRow {
    pub(super) label: &'static str,
    pub(super) meta: String,
    pub(super) line: usize,
}

struct GroupSpec {
    kind: NetlistNavigatorRowKind,
    section: OutlineSectionKind,
    label: &'static str,
    empty_note: &'static str,
}

const STRUCTURE_GROUPS: [GroupSpec; 5] = [
    GroupSpec {
        kind: NetlistNavigatorRowKind::Parameters,
        section: OutlineSectionKind::Parameters,
        label: "Parameters",
        empty_note: "No .param definitions.",
    },
    GroupSpec {
        kind: NetlistNavigatorRowKind::Instances,
        section: OutlineSectionKind::Devices,
        label: "Instances",
        empty_note: "No device instances.",
    },
    GroupSpec {
        kind: NetlistNavigatorRowKind::Models,
        section: OutlineSectionKind::Models,
        label: "Model bindings",
        empty_note: "No .model cards.",
    },
    GroupSpec {
        kind: NetlistNavigatorRowKind::Analyses,
        section: OutlineSectionKind::Analyses,
        label: "Analyses",
        empty_note: "No analysis directives.",
    },
    GroupSpec {
        kind: NetlistNavigatorRowKind::Measurements,
        section: OutlineSectionKind::Measurements,
        label: "Measurements",
        empty_note: "No .meas directives.",
    },
];

struct IndexSpec {
    section: OutlineSectionKind,
    label: &'static str,
    unit: &'static str,
}

const SEMANTIC_INDEX: [IndexSpec; 7] = [
    IndexSpec {
        section: OutlineSectionKind::Subcircuits,
        label: "Hierarchy",
        unit: "definition",
    },
    IndexSpec {
        section: OutlineSectionKind::Globals,
        label: "Globals",
        unit: "declaration",
    },
    IndexSpec {
        section: OutlineSectionKind::Functions,
        label: "Functions",
        unit: "definition",
    },
    IndexSpec {
        section: OutlineSectionKind::Options,
        label: "Solver options",
        unit: "card",
    },
    IndexSpec {
        section: OutlineSectionKind::Outputs,
        label: "Save and probe",
        unit: "directive",
    },
    IndexSpec {
        section: OutlineSectionKind::Conditionals,
        label: "Conditionals",
        unit: "card",
    },
    IndexSpec {
        section: OutlineSectionKind::Controls,
        label: "Control",
        unit: "directive",
    },
];

#[derive(Debug, Clone, PartialEq, Eq)]
pub(super) struct NetlistNavigatorProjection {
    pub(super) line_count: usize,
    pub(super) root_row: Option<NetlistNavigatorRow>,
    pub(super) groups: Vec<NetlistOutlineGroup>,
    pub(super) include_rows: Vec<NetlistNavigatorRow>,
    pub(super) semantic_rows: Vec<NetlistSemanticRow>,
    /// Cards covered by the semantic index, stated in its header so the
    /// section's own claim can be checked against the deck.
    pub(super) semantic_cards: usize,
    pub(super) show_source_mapping: bool,
}

impl NetlistNavigatorProjection {
    pub(super) fn from_source(
        source: &str,
        query: &str,
        root_label: &str,
        source_mapped: bool,
        collapsed: &BTreeSet<OutlineSectionKind>,
        include_states: &[(String, &'static str)],
    ) -> Self {
        let outline = NetlistOutline::parse(source);
        let query = NetlistNavigatorQuery::new(query);
        let entries = outline.entries();
        let lines = source.lines().collect::<Vec<_>>();
        let section_of = |kind: OutlineSectionKind| -> Vec<&OutlineEntry> {
            outline
                .sections()
                .iter()
                .find(|section| section.kind() == kind)
                .map(OutlineSection::entry_indices)
                .unwrap_or_default()
                .iter()
                .filter_map(|index| entries.get(*index))
                .collect()
        };

        let root_entries = section_of(OutlineSectionKind::Source);
        let root_target = root_entries.first().copied().or_else(|| entries.first());
        let root_row = query.matches_group(root_label, &root_entries).then(|| {
            navigator_group_row(
                NetlistNavigatorRowKind::Root,
                root_label,
                if source_mapped {
                    "root"
                } else {
                    "source of truth"
                }
                .to_owned(),
                root_target,
                &root_entries,
            )
        });

        let groups = STRUCTURE_GROUPS
            .iter()
            .filter_map(|spec| {
                outline_group(spec, &section_of(spec.section), &query, &lines, collapsed)
            })
            .collect::<Vec<_>>();

        let include_rows = section_of(OutlineSectionKind::Dependencies)
            .into_iter()
            .filter(|entry| query.matches_entry(entry))
            .map(|entry| {
                let label = include_entry_label(entry.label());
                NetlistNavigatorRow {
                    kind: NetlistNavigatorRowKind::Include,
                    // A locator the closure never retained is not "resolved"
                    // and not "missing" either; the row says where it is
                    // written and claims nothing about its fate.
                    meta: Some(
                        include_states
                            .iter()
                            .find(|(locator, _)| *locator == label)
                            .map_or_else(
                                || format!("line {}", entry.line()),
                                |(_, state)| (*state).to_owned(),
                            ),
                    ),
                    label,
                    target_line: Some(entry.line()),
                    source_ranges: vec![(entry.line(), entry.end_line())],
                }
            })
            .collect();

        let mut semantic_cards = 0usize;
        let semantic_rows = SEMANTIC_INDEX
            .iter()
            .filter_map(|spec| {
                let section = section_of(spec.section);
                let counted = if spec.section == OutlineSectionKind::Subcircuits {
                    // The section holds both ends of every definition; only the
                    // opening card is a definition.
                    section
                        .iter()
                        .filter(|entry| entry.kind() == OutlineEntryKind::Subcircuit)
                        .count()
                } else {
                    section.len()
                };
                if counted == 0 || !query.matches_group(spec.label, &section) {
                    return None;
                }
                // The header has to be the sum of the rows under it, or a
                // reader who adds them up finds the section disagreeing
                // with itself.
                semantic_cards += counted;
                Some(NetlistSemanticRow {
                    label: spec.label,
                    meta: format!(
                        "{counted} {}{}",
                        spec.unit,
                        if counted == 1 { "" } else { "s" }
                    ),
                    line: section.first().map_or(1, |entry| entry.line()),
                })
            })
            .collect::<Vec<_>>();

        Self {
            line_count: lines.len(),
            root_row,
            groups,
            include_rows,
            semantic_rows,
            semantic_cards,
            show_source_mapping: source_mapped
                && (query.matches_text("source mapping")
                    || query.matches_text("provenance")
                    || query.is_empty()),
        }
    }

    fn is_empty(&self) -> bool {
        self.root_row.is_none()
            && self.groups.is_empty()
            && self.include_rows.is_empty()
            && self.semantic_rows.is_empty()
            && !self.show_source_mapping
    }
}

fn outline_group(
    spec: &GroupSpec,
    section: &[&OutlineEntry],
    query: &NetlistNavigatorQuery,
    lines: &[&str],
    collapsed: &BTreeSet<OutlineSectionKind>,
) -> Option<NetlistOutlineGroup> {
    if !query.matches_group(spec.label, section) {
        return None;
    }
    // A filter names the declarations it kept, so the group discloses them
    // whatever the stored preference says.
    let filtering = !query.is_empty() && !query.matches_text(spec.label);
    let expanded = filtering || !collapsed.contains(&spec.section);
    let selected = if filtering {
        section
            .iter()
            .copied()
            .filter(|entry| query.matches_entry(entry))
            .collect::<Vec<_>>()
    } else {
        section.to_vec()
    };
    let meta = if filtering {
        format!("{} of {}", selected.len(), section.len())
    } else {
        section.len().to_string()
    };
    let children = if expanded {
        selected
            .iter()
            .map(|entry| outline_child(spec.section, entry, card_of(lines, entry)))
            .collect()
    } else {
        Vec::new()
    };
    Some(NetlistOutlineGroup {
        row: navigator_group_row(
            spec.kind,
            spec.label,
            meta,
            selected
                .first()
                .copied()
                .or_else(|| section.first().copied()),
            &selected,
        ),
        section: spec.section,
        children,
        expanded,
        empty_note: spec.empty_note,
    })
}

fn card_of<'a>(lines: &[&'a str], entry: &OutlineEntry) -> &'a str {
    entry
        .line()
        .checked_sub(1)
        .and_then(|index| lines.get(index))
        .copied()
        .unwrap_or_default()
}

/// Name one declaration and what it binds to. The outline entry keeps a head
/// and one detail token, which names a card but does not describe it, so the
/// binding is read back off the card itself.
fn outline_child(
    section: OutlineSectionKind,
    entry: &OutlineEntry,
    card: &str,
) -> NetlistOutlineChild {
    let tokens = crate::state::card_tokens(card);
    let (label, meta) = match section {
        OutlineSectionKind::Parameters => parameter_child(&tokens),
        OutlineSectionKind::Devices => (
            entry.label().to_owned(),
            device_binding(&tokens).map(str::to_owned),
        ),
        OutlineSectionKind::Models => (
            tokens
                .get(1)
                .cloned()
                .unwrap_or_else(|| entry.label().to_owned()),
            positional_tokens(&tokens).get(2).cloned(),
        ),
        OutlineSectionKind::Analyses => (
            tokens
                .first()
                .cloned()
                .unwrap_or_else(|| entry.label().to_owned()),
            (tokens.len() > 1).then(|| tokens[1..].join(" ")),
        ),
        OutlineSectionKind::Measurements => (
            tokens
                .get(2)
                .cloned()
                .unwrap_or_else(|| entry.label().to_owned()),
            tokens.get(1).cloned(),
        ),
        _ => (entry.label().to_owned(), None),
    };
    NetlistOutlineChild {
        label,
        meta,
        line: entry.line(),
        end_line: entry.end_line(),
    }
}

/// A `.param` card declares one or more names. One name is shown with the
/// expression it is bound to; several are listed without one, because the
/// row would otherwise attribute the first value to all of them.
fn parameter_child(tokens: &[String]) -> (String, Option<String>) {
    let assignments = parameter_assignments(tokens);
    match assignments.as_slice() {
        [] => (tokens.join(" "), None),
        [(name, value)] => (name.clone(), value.clone()),
        several => (
            several
                .iter()
                .map(|(name, _)| name.as_str())
                .collect::<Vec<_>>()
                .join(", "),
            None,
        ),
    }
}

/// The names a card assigns, each with its value where the deck wrote one the
/// row can quote whole. SPICE accepts `n=v`, `n =v`, `n= v` and `n = v`, and
/// all four have to read the same way.
fn parameter_assignments(tokens: &[String]) -> Vec<(String, Option<String>)> {
    // Reduce the card to lexemes and the `=` between them, so the four
    // spellings all become the same three-lexeme sequence.
    let mut lexemes = Vec::<&str>::new();
    for token in tokens.iter().skip(1) {
        let mut rest = token.as_str();
        while let Some(at) = rest.find('=') {
            if at > 0 {
                lexemes.push(&rest[..at]);
            }
            lexemes.push("=");
            rest = &rest[at + 1..];
        }
        if !rest.is_empty() {
            lexemes.push(rest);
        }
    }
    (1..lexemes.len().saturating_sub(1))
        .filter(|index| lexemes[*index] == "=")
        .filter_map(|index| {
            let (name, value) = (lexemes[index - 1], lexemes[index + 1]);
            (name != "=").then(|| {
                (
                    name.to_owned(),
                    (value != "=" && expression_is_whole(value)).then(|| value.to_owned()),
                )
            })
        })
        .collect()
}

/// Whether a value lexeme is the whole expression the deck wrote.
///
/// `gain = {2 * k}` arrives here already split on whitespace, and `{2` is not
/// the value of anything — a row that showed it would misquote the deck.
fn expression_is_whole(value: &str) -> bool {
    let mut depth = 0i32;
    for character in value.chars() {
        match character {
            '{' | '(' => depth += 1,
            '}' | ')' => depth -= 1,
            _ => {}
        }
        if depth < 0 {
            return false;
        }
    }
    depth == 0
}

/// Everything before the first `name=value` argument. A scan that only looked
/// for `=` inside a token would read the value of a spaced assignment as one
/// more positional field.
fn positional_tokens(tokens: &[String]) -> &[String] {
    for (index, token) in tokens.iter().enumerate() {
        if token.contains('=')
            || tokens
                .get(index + 1)
                .is_some_and(|next| next.starts_with('='))
        {
            return &tokens[..index];
        }
    }
    tokens
}

/// The model or subcircuit an instance binds to, when the card's element
/// letter fixes its terminal count and therefore says which field that is.
/// Anything else returns nothing rather than naming a node as a model.
fn device_binding(tokens: &[String]) -> Option<&str> {
    let positional = positional_tokens(tokens);
    let letter = positional.first()?.chars().next()?.to_ascii_uppercase();
    let binding = match letter {
        // Two terminals then the value expression.
        'R' | 'C' | 'L' => positional.get(3).map(String::as_str),
        // Fixed terminal count then the model or subcircuit master.
        'X' | 'D' | 'Q' | 'J' | 'Z' | 'M' => {
            let minimum = match letter {
                'X' => 2,
                'D' => 4,
                'M' => 6,
                _ => 5,
            };
            (positional.len() >= minimum)
                .then(|| positional.last())?
                .map(String::as_str)
        }
        _ => None,
    };
    binding.filter(|value| expression_is_whole(value))
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub(super) struct NetlistNavigatorQuery {
    pub(super) text: String,
    pub(super) line: Option<usize>,
}

impl NetlistNavigatorQuery {
    pub(super) fn new(query: &str) -> Self {
        let text = query.trim().to_lowercase();
        let line_literal = text
            .strip_prefix("line")
            .map(str::trim)
            .unwrap_or(&text)
            .trim_start_matches([':', '#']);
        let line = line_literal.parse::<usize>().ok().filter(|line| *line > 0);
        Self { text, line }
    }

    fn is_empty(&self) -> bool {
        self.text.is_empty()
    }

    fn matches_text(&self, text: &str) -> bool {
        self.is_empty() || text.to_lowercase().contains(&self.text)
    }

    fn matches_entry(&self, entry: &OutlineEntry) -> bool {
        self.is_empty()
            || self
                .line
                .is_some_and(|line| (entry.line()..=entry.end_line()).contains(&line))
            || entry.label().to_lowercase().contains(&self.text)
    }

    fn matches_group(&self, label: &str, entries: &[&OutlineEntry]) -> bool {
        self.matches_text(label) || entries.iter().any(|entry| self.matches_entry(entry))
    }

    fn matches_diff_hunk(&self, hunk: &DiffHunk) -> bool {
        self.matches_text(&hunk.label) || self.matches_text(&hunk.meta)
    }
}

pub(super) fn navigator_group_row(
    kind: NetlistNavigatorRowKind,
    label: &str,
    meta: String,
    target: Option<&OutlineEntry>,
    entries: &[&OutlineEntry],
) -> NetlistNavigatorRow {
    NetlistNavigatorRow {
        kind,
        label: label.to_owned(),
        meta: Some(meta),
        target_line: target.map(OutlineEntry::line),
        source_ranges: if entries.is_empty() {
            target
                .map(|entry| vec![(entry.line(), entry.end_line())])
                .unwrap_or_default()
        } else {
            entries
                .iter()
                .map(|entry| (entry.line(), entry.end_line()))
                .collect()
        },
    }
}

pub(super) fn include_entry_label(label: &str) -> String {
    label
        .split_once(char::is_whitespace)
        .map_or(label, |(_, locator)| locator)
        .trim_matches(['\'', '"'])
        .to_owned()
}

/// Which of `count` uniform rows a viewport can show, given the offset of the
/// first row from its top. `ScrollArea::show_rows` does this for a scroll area
/// it owns; the outline's declarations share one with the rest of the tree, so
/// the span is computed here and the remainder becomes space.
pub(super) fn visible_row_span(
    first_row_offset: f32,
    viewport_height: f32,
    row_height: f32,
    count: usize,
) -> std::ops::Range<usize> {
    if count == 0 || row_height <= 0.0 {
        return 0..0;
    }
    let first = ((-first_row_offset / row_height).floor()).max(0.0) as usize;
    let first = first.min(count);
    let end = (((viewport_height - first_row_offset) / row_height).ceil()).max(0.0) as usize;
    first..end.clamp(first, count)
}

fn uniform_rows(ui: &mut Ui, height: f32, count: usize, mut row: impl FnMut(&mut Ui, usize)) {
    let clip = ui.clip_rect();
    let span = visible_row_span(ui.cursor().top() - clip.top(), clip.height(), height, count);
    if span.start > 0 {
        ui.add_space(span.start as f32 * height);
    }
    for index in span.clone() {
        row(ui, index);
    }
    if span.end < count {
        ui.add_space((count - span.end) as f32 * height);
    }
}

/// How an outline row is placed: what occupies its left edge and where its
/// label starts.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(super) enum NetlistOutlineRowShape {
    /// Icon and label, with the caret column left empty so a root or include
    /// row aligns with the groups above and below it.
    Leaf,
    /// A caret stating the current disclosure, then icon and label.
    Group { expanded: bool },
    /// A declaration under a group: guide line, indent, monospaced label.
    Child,
    /// A semantic-index row: label and count only, flush to the panel edge.
    Index,
}

pub(super) struct OutlineRowVisual<'a> {
    pub(super) label: &'a str,
    pub(super) meta: Option<&'a str>,
    pub(super) icon: Option<WorkbenchIcon>,
    pub(super) shape: NetlistOutlineRowShape,
    pub(super) selected: bool,
    pub(super) enabled: bool,
    pub(super) height: f32,
}

impl NetlistOutlineRowShape {
    /// Left edge of the label text, measured from the row's left edge.
    pub(super) fn label_left(self) -> f32 {
        match self {
            Self::Index => NETLIST_OUTLINE_PADDING_X,
            Self::Leaf | Self::Group { .. } => {
                NETLIST_OUTLINE_PADDING_X
                    + NETLIST_OUTLINE_CARET_COLUMN
                    + NETLIST_OUTLINE_ICON_SIZE
                    + NETLIST_OUTLINE_ICON_GAP
            }
            Self::Child => {
                NETLIST_OUTLINE_PADDING_X
                    + NETLIST_OUTLINE_CARET_COLUMN
                    + NETLIST_OUTLINE_ICON_SIZE
                    + NETLIST_OUTLINE_ICON_GAP
                    + NETLIST_OUTLINE_CHILD_INDENT
            }
        }
    }
}

pub(super) fn netlist_outline_row(ui: &mut Ui, row: OutlineRowVisual<'_>) -> Response {
    let t = Tokens::get(ui.ctx());
    let (rect, response) = ui.allocate_exact_size(
        egui::vec2(ui.available_width(), row.height),
        if row.enabled {
            egui::Sense::click()
        } else {
            egui::Sense::hover()
        },
    );
    let label = row.label.to_owned();
    response.widget_info(|| {
        egui::WidgetInfo::selected(
            egui::WidgetType::SelectableLabel,
            row.enabled,
            row.selected,
            label.clone(),
        )
    });
    if !ui.is_rect_visible(rect) {
        return response;
    }
    if row.selected || (row.enabled && response.hovered()) {
        ui.painter().rect_filled(
            rect,
            0.0,
            if row.selected {
                t.color.accent_dim
            } else {
                t.color.bg_hover
            },
        );
    }
    if row.selected {
        ui.painter().rect_filled(
            egui::Rect::from_min_max(
                rect.left_top(),
                egui::pos2(rect.left() + 2.0, rect.bottom()),
            ),
            0.0,
            t.color.accent,
        );
    }

    let foreground = if !row.enabled {
        t.color.text_faint
    } else if row.selected {
        t.color.text
    } else {
        t.color.text_dim
    };
    let icon_center_x = rect.left()
        + NETLIST_OUTLINE_PADDING_X
        + NETLIST_OUTLINE_CARET_COLUMN
        + NETLIST_OUTLINE_ICON_SIZE * 0.5;
    if let NetlistOutlineRowShape::Group { expanded } = row.shape {
        let caret = if expanded {
            WorkbenchIcon::ChevronDown
        } else {
            WorkbenchIcon::ChevronRight
        };
        caret.paint(
            ui.painter(),
            egui::Rect::from_center_size(
                egui::pos2(
                    rect.left() + NETLIST_OUTLINE_PADDING_X + NETLIST_OUTLINE_CARET_COLUMN * 0.5,
                    rect.center().y,
                ),
                egui::Vec2::splat(NETLIST_OUTLINE_CARET_SIZE),
            ),
            t.color.text_faint,
        );
    }
    if row.shape == NetlistOutlineRowShape::Child {
        ui.painter().vline(
            icon_center_x,
            rect.y_range(),
            egui::Stroke::new(1.0, t.color.border),
        );
    }
    if let Some(icon) = row.icon {
        icon.paint(
            ui.painter(),
            egui::Rect::from_center_size(
                egui::pos2(icon_center_x, rect.center().y),
                egui::Vec2::splat(NETLIST_OUTLINE_ICON_SIZE),
            ),
            foreground,
        );
    }

    let meta_font = theme::mono(tokens::FS_0, FontWeight::Regular);
    let meta_width = row.meta.map_or(0.0, |meta| {
        ui.painter()
            .layout_no_wrap(meta.to_owned(), meta_font.clone(), t.color.text_faint)
            .size()
            .x
    });
    let label_left = rect.left() + row.shape.label_left();
    let label_right = if row.meta.is_some() {
        rect.right() - NETLIST_OUTLINE_PADDING_X - meta_width - NETLIST_OUTLINE_ICON_GAP
    } else {
        rect.right() - NETLIST_OUTLINE_PADDING_X
    };
    ui.painter()
        .with_clip_rect(egui::Rect::from_x_y_ranges(
            label_left..=label_right.max(label_left),
            rect.y_range(),
        ))
        .text(
            egui::pos2(label_left, rect.center().y),
            egui::Align2::LEFT_CENTER,
            row.label,
            if row.shape == NetlistOutlineRowShape::Child {
                theme::mono(tokens::FS_0, FontWeight::Regular)
            } else {
                theme::sans(tokens::FS_0, FontWeight::Regular)
            },
            foreground,
        );
    if let Some(meta) = row.meta {
        ui.painter().text(
            egui::pos2(rect.right() - NETLIST_OUTLINE_PADDING_X, rect.center().y),
            egui::Align2::RIGHT_CENTER,
            meta,
            meta_font,
            t.color.text_faint,
        );
    }
    theme::paint_focus_ring(ui, &response, rect);
    response
}

/// State, where the declarations would be, that a group holds none.
fn outline_child_note(ui: &mut Ui, note: &str, height: f32) {
    let t = Tokens::get(ui.ctx());
    let (rect, response) = ui.allocate_exact_size(
        egui::vec2(ui.available_width(), height),
        egui::Sense::hover(),
    );
    response
        .widget_info(|| egui::WidgetInfo::labeled(egui::WidgetType::Label, ui.is_enabled(), note));
    if !ui.is_rect_visible(rect) {
        return;
    }
    ui.painter().vline(
        rect.left()
            + NETLIST_OUTLINE_PADDING_X
            + NETLIST_OUTLINE_CARET_COLUMN
            + NETLIST_OUTLINE_ICON_SIZE * 0.5,
        rect.y_range(),
        egui::Stroke::new(1.0, t.color.border),
    );
    ui.painter().text(
        egui::pos2(
            rect.left() + NetlistOutlineRowShape::Child.label_left(),
            rect.center().y,
        ),
        egui::Align2::LEFT_CENTER,
        note,
        theme::sans(tokens::FS_0, FontWeight::Regular),
        t.color.text_faint,
    );
}

pub(super) fn netlist_outline_icon(kind: NetlistNavigatorRowKind) -> WorkbenchIcon {
    match kind {
        NetlistNavigatorRowKind::Root => WorkbenchIcon::Code,
        NetlistNavigatorRowKind::Parameters => WorkbenchIcon::Sliders,
        NetlistNavigatorRowKind::Instances => WorkbenchIcon::Component,
        NetlistNavigatorRowKind::Models => WorkbenchIcon::Models,
        NetlistNavigatorRowKind::Analyses => WorkbenchIcon::Simulate,
        NetlistNavigatorRowKind::Measurements => WorkbenchIcon::Target,
        NetlistNavigatorRowKind::Include => WorkbenchIcon::File,
    }
}

pub(super) fn netlist_source_mapping(ui: &mut Ui, app: &mut RSpiceApp, active_line: usize) {
    let t = Tokens::get(ui.ctx());
    let mapping = app
        .state
        .ui
        .netlist
        .generated_document
        .as_ref()
        .and_then(|document| document.generated_artifact().source_map_entry(active_line))
        .map(|entry| {
            (
                entry.cell_identity().to_owned(),
                entry.view_identity().to_owned(),
                entry.instance_identity().map(str::to_owned),
                entry.component_identity().map(str::to_owned),
            )
        });
    let Some((cell, view, instance, component)) = mapping else {
        muted(ui, "No generated provenance is mapped to the active line.");
        return;
    };
    egui::Frame::new()
        .inner_margin(egui::Margin {
            left: 10,
            right: 10,
            top: 8,
            bottom: 8,
        })
        .show(ui, |ui| {
            ui.set_width(ui.available_width().max(1.0));
            ui.label(
                egui::RichText::new(format!("Line {active_line} \u{b7} {cell}"))
                    .font(theme::sans(tokens::FS_0, FontWeight::Medium))
                    .color(t.color.text_dim),
            );
            ui.label(
                egui::RichText::new(view)
                    .font(theme::mono(tokens::FS_0, FontWeight::Regular))
                    .color(t.color.text_faint),
            );
            if let Some(instance) = instance.as_deref() {
                ui.label(
                    egui::RichText::new(format!("Instance {instance}"))
                        .font(theme::mono(tokens::FS_0, FontWeight::Regular))
                        .color(t.color.text_faint),
                );
            }
            if let Some(component_identity) = component
                && let Some(component_id) = component_identity
                    .rsplit('/')
                    .next()
                    .and_then(|value| value.parse::<u64>().ok())
                && ui.button("Cross-probe schematic component").clicked()
            {
                app.state.schematic.selection.clear();
                app.state.schematic.selection.select_component(component_id);
                app.state
                    .push_user_message(crate::diagnostics::ConsoleMessage::info(format!(
                        "Cross-probed generated line {active_line} to component {component_id}."
                    )));
            }
        });
}

/// A navigator row that states, in the row's own place, why a section is
/// empty — a filtered-out list and an empty design must never look alike.
pub(super) fn empty_navigator_row(ui: &mut Ui, message: &str) {
    let t = Tokens::get(ui.ctx());
    let (rect, response) = ui.allocate_exact_size(
        egui::vec2(ui.available_width().max(1.0), t.metrics.row_h.max(29.0)),
        egui::Sense::hover(),
    );
    ui.painter().text(
        egui::pos2(rect.left() + 10.0, rect.center().y),
        egui::Align2::LEFT_CENTER,
        message,
        theme::sans(tokens::FS_0, FontWeight::Regular),
        t.color.text_dim,
    );
    response.widget_info(|| {
        egui::WidgetInfo::labeled(egui::WidgetType::Label, ui.is_enabled(), message)
    });
}

pub(super) fn nav_row(
    ui: &mut Ui,
    icon: WorkbenchIcon,
    label: &str,
    selected: bool,
    meta: Option<&str>,
) -> bool {
    nav_row_response(ui, icon, label, selected, meta).clicked()
}

pub(super) fn nav_row_response(
    ui: &mut Ui,
    icon: WorkbenchIcon,
    label: &str,
    selected: bool,
    meta: Option<&str>,
) -> Response {
    nav_row_indented_response(ui, icon, label, selected, meta, 0)
}

pub(super) fn nav_row_indented(
    ui: &mut Ui,
    icon: WorkbenchIcon,
    label: &str,
    selected: bool,
    meta: Option<&str>,
    level: usize,
) -> bool {
    nav_row_indented_response(ui, icon, label, selected, meta, level).clicked()
}

pub(super) fn nav_row_indented_response(
    ui: &mut Ui,
    icon: WorkbenchIcon,
    label: &str,
    selected: bool,
    meta: Option<&str>,
    level: usize,
) -> Response {
    nav_row_indented_styled(ui, icon, label, selected, meta, level, false)
}

pub(super) fn schematic_nav_row_indented_response(
    ui: &mut Ui,
    icon: WorkbenchIcon,
    label: &str,
    selected: bool,
    meta: Option<&str>,
    level: usize,
    mono: bool,
    expanded: bool,
    child_guide: bool,
) -> Response {
    nav_row_indented_styled_with_metrics(
        ui,
        icon,
        label,
        selected,
        meta,
        level,
        mono,
        SCHEMATIC_NAV_ROW_HEIGHT,
        SCHEMATIC_NAV_LABEL_SIZE,
        SCHEMATIC_NAV_META_SIZE,
        expanded,
        child_guide,
        // accessibility-pointer-shim: the sense is forwarded to
        // `nav_row_indented_styled_with_metrics`, which allocates the row and
        // owns both its WidgetInfo and its focus ring.
        egui::Sense::click(),
    )
}

pub(super) fn schematic_nav_row_indented_drag_response(
    ui: &mut Ui,
    icon: WorkbenchIcon,
    label: &str,
    selected: bool,
    meta: Option<&str>,
    level: usize,
    mono: bool,
    expanded: bool,
    child_guide: bool,
) -> Response {
    nav_row_indented_styled_with_metrics(
        ui,
        icon,
        label,
        selected,
        meta,
        level,
        mono,
        SCHEMATIC_NAV_ROW_HEIGHT,
        SCHEMATIC_NAV_LABEL_SIZE,
        SCHEMATIC_NAV_META_SIZE,
        expanded,
        child_guide,
        // accessibility-pointer-shim: same forwarding wrapper, drag variant.
        egui::Sense::click_and_drag(),
    )
}

#[cfg(test)]
mod tests {
    use super::*;

    fn netlist_projection(source: &str, query: &str) -> NetlistNavigatorProjection {
        NetlistNavigatorProjection::from_source(
            source,
            query,
            "top.sp",
            true,
            &std::collections::BTreeSet::new(),
            &[],
        )
    }

    const OUTLINE_DECK: &str = "Precision amplifier\n.include models/base.lib\n.lib corners/process.lib TT\n.param gain=10 offset=1m\nR1 in out 1k\nXAMP in out opamp\n.model nch nmos\n.ac dec 10 1 1g\n.meas ac peak max v(out)\n.end\n";

    #[test]
    fn netlist_navigator_projects_exact_live_counts_and_include_lines() {
        let projection = netlist_projection(OUTLINE_DECK, "");

        assert_eq!(projection.line_count, 10);
        let count = |kind| {
            projection
                .groups
                .iter()
                .find(|group| group.row.kind == kind)
                .and_then(|group| group.row.meta.as_deref())
        };
        assert_eq!(
            projection
                .root_row
                .as_ref()
                .and_then(|row| row.meta.as_deref()),
            Some("root")
        );
        assert_eq!(count(NetlistNavigatorRowKind::Parameters), Some("1"));
        assert_eq!(count(NetlistNavigatorRowKind::Instances), Some("2"));
        let instances = projection
            .groups
            .iter()
            .find(|group| group.row.kind == NetlistNavigatorRowKind::Instances)
            .expect("instances group exists");
        assert!(instances.row.contains_line(5));
        assert!(instances.row.contains_line(6));
        assert_eq!(count(NetlistNavigatorRowKind::Models), Some("1"));
        assert_eq!(count(NetlistNavigatorRowKind::Analyses), Some("1"));
        assert_eq!(count(NetlistNavigatorRowKind::Measurements), Some("1"));
        assert_eq!(projection.include_rows.len(), 2);
        assert_eq!(projection.include_rows[0].label, "models/base.lib");
        assert_eq!(projection.include_rows[0].target_line, Some(2));
        assert_eq!(projection.include_rows[1].label, "corners/process.lib");
        assert_eq!(projection.include_rows[1].target_line, Some(3));
        assert!(projection.show_source_mapping);
    }

    #[test]
    fn an_include_row_names_the_fate_of_its_own_dependency() {
        let states = [
            ("models/base.lib".to_owned(), "missing"),
            ("corners/process.lib".to_owned(), "vendor source"),
        ];
        let projection = NetlistNavigatorProjection::from_source(
            OUTLINE_DECK,
            "",
            "top.sp",
            true,
            &std::collections::BTreeSet::new(),
            &states,
        );

        // The header states one verdict for the whole closure; the row that
        // earned it has to be identifiable.
        assert_eq!(projection.include_rows[0].meta.as_deref(), Some("missing"));
        assert_eq!(
            projection.include_rows[1].meta.as_deref(),
            Some("vendor source")
        );

        // A locator the closure never retained claims no fate at all.
        let unknown = netlist_projection(OUTLINE_DECK, "");
        assert_eq!(unknown.include_rows[0].meta.as_deref(), Some("line 2"));
    }

    #[test]
    fn every_outline_group_discloses_the_declarations_it_counts() {
        let projection = netlist_projection(OUTLINE_DECK, "");

        let children = |kind| {
            projection
                .groups
                .iter()
                .find(|group| group.row.kind == kind)
                .map(|group| {
                    group
                        .children
                        .iter()
                        .map(|child| (child.label.as_str(), child.meta.as_deref(), child.line))
                        .collect::<Vec<_>>()
                })
                .expect("group exists")
        };

        // A `.param` card that declares two names attributes no value to
        // either, because the row would otherwise give both the first one's.
        assert_eq!(
            children(NetlistNavigatorRowKind::Parameters),
            vec![("gain, offset", None, 4)]
        );
        // A resistor's value is positional; a subcircuit call's master is the
        // last positional field. Both are exact from the element letter.
        assert_eq!(
            children(NetlistNavigatorRowKind::Instances),
            vec![("R1", Some("1k"), 5), ("XAMP", Some("opamp"), 6)]
        );
        assert_eq!(
            children(NetlistNavigatorRowKind::Models),
            vec![("nch", Some("nmos"), 7)]
        );
        assert_eq!(
            children(NetlistNavigatorRowKind::Analyses),
            vec![(".ac", Some("dec 10 1 1g"), 8)]
        );
        assert_eq!(
            children(NetlistNavigatorRowKind::Measurements),
            vec![("peak", Some("ac"), 9)]
        );
    }

    #[test]
    fn a_collapsed_group_keeps_its_count_and_builds_no_children() {
        let collapsed =
            std::collections::BTreeSet::from([crate::state::OutlineSectionKind::Devices]);
        let projection = NetlistNavigatorProjection::from_source(
            OUTLINE_DECK,
            "",
            "top.sp",
            true,
            &collapsed,
            &[],
        );

        let instances = projection
            .groups
            .iter()
            .find(|group| group.row.kind == NetlistNavigatorRowKind::Instances)
            .expect("instances group exists");
        assert!(!instances.expanded);
        assert!(instances.children.is_empty());
        assert_eq!(instances.row.meta.as_deref(), Some("2"));
    }

    #[test]
    fn every_parsed_category_is_reachable_from_the_structure_tree_or_the_index() {
        let source = "deck\n.global vdd\n.func square(x) {x*x}\n.options reltol=1e-5\n.subckt amp in out\nM1 out in 0 0 nch\n.ends amp\n.if corner\n.save v(out)\n.endif\n.end\n";
        let projection = netlist_projection(source, "");

        let index = projection
            .semantic_rows
            .iter()
            .map(|row| (row.label, row.meta.as_str(), row.line))
            .collect::<Vec<_>>();
        assert_eq!(
            index,
            vec![
                // `.ends` shares the section but is not a definition.
                ("Hierarchy", "1 definition", 5),
                ("Globals", "1 declaration", 2),
                ("Functions", "1 definition", 3),
                ("Solver options", "1 card", 4),
                ("Save and probe", "1 directive", 9),
                ("Conditionals", "2 cards", 8),
                ("Control", "1 directive", 11),
            ]
        );
        // The header is the sum of the rows beneath it, so a reader who adds
        // them up gets the number the section claims.
        assert_eq!(
            projection.semantic_cards,
            index.len() + 1,
            "seven categories, one of which counts two conditionals"
        );
        assert_eq!(projection.semantic_cards, 8);
    }

    #[test]
    fn a_category_the_deck_does_not_declare_is_left_out_of_the_index() {
        let projection = netlist_projection("deck\nR1 in out 1k\n.end\n", "");

        assert!(
            projection
                .semantic_rows
                .iter()
                .all(|row| row.label != "Conditionals"),
            "an absent category must not be listed as present with a zero"
        );
        // A structure group stays, because it says in its own place what the
        // deck does not declare.
        assert!(projection.groups.iter().any(|group| group.row.kind
            == NetlistNavigatorRowKind::Measurements
            && group.children.is_empty()
            && !group.empty_note.is_empty()));
    }

    #[test]
    fn netlist_navigator_filter_matches_symbols_and_exact_source_lines() {
        let source = "deck\n.param gain=10\nR1 in out 1k\nR2 out 0 2k\n.end\n";

        let symbol = netlist_projection(source, "r2");
        assert!(symbol.root_row.is_none());
        assert_eq!(symbol.groups.len(), 1);
        assert_eq!(
            symbol.groups[0].row.kind,
            NetlistNavigatorRowKind::Instances
        );
        // A filtered count that showed the total would read as "this is
        // everything the deck declares".
        assert_eq!(symbol.groups[0].row.meta.as_deref(), Some("1 of 2"));
        assert_eq!(symbol.groups[0].row.target_line, Some(4));
        assert!(!symbol.groups[0].row.contains_line(3));
        assert!(symbol.groups[0].row.contains_line(4));
        assert!(!symbol.show_source_mapping);

        let line = netlist_projection(source, "line 2");
        assert_eq!(line.groups.len(), 1);
        assert_eq!(line.groups[0].row.kind, NetlistNavigatorRowKind::Parameters);
        assert_eq!(line.groups[0].row.target_line, Some(2));
    }

    #[test]
    fn a_filter_discloses_what_it_kept_even_where_the_group_was_collapsed() {
        let collapsed =
            std::collections::BTreeSet::from([crate::state::OutlineSectionKind::Devices]);
        let projection = NetlistNavigatorProjection::from_source(
            "deck\nR1 in out 1k\nR2 out 0 2k\n.end\n",
            "r2",
            "top.sp",
            true,
            &collapsed,
            &[],
        );

        assert_eq!(projection.groups.len(), 1);
        assert!(projection.groups[0].expanded);
        assert_eq!(projection.groups[0].children.len(), 1);
        assert_eq!(projection.groups[0].children[0].label, "R2");
    }

    #[test]
    fn an_instance_binding_is_named_only_where_the_element_letter_fixes_it() {
        let source =
            "deck\nM1 d g s b nch W=1u\nD1 a k dmod\nQ1 c b e qmod\nV1 in 0 DC 1.8\nD2 a k\n.end\n";
        let projection = netlist_projection(source, "");

        let instances = &projection
            .groups
            .iter()
            .find(|group| group.row.kind == NetlistNavigatorRowKind::Instances)
            .expect("instances group exists")
            .children;
        let binding = |name: &str| {
            instances
                .iter()
                .find(|child| child.label == name)
                .and_then(|child| child.meta.clone())
        };
        assert_eq!(binding("M1").as_deref(), Some("nch"));
        assert_eq!(binding("D1").as_deref(), Some("dmod"));
        assert_eq!(binding("Q1").as_deref(), Some("qmod"));
        // A source's argument list is not a model name.
        assert_eq!(binding("V1"), None);
        // A diode short of its model card must not report its cathode as one.
        assert_eq!(binding("D2"), None);
    }

    #[test]
    fn a_spaced_assignment_is_not_read_as_one_more_positional_field() {
        let projection =
            netlist_projection("deck\nM1 d g s b nch W = 1u\n.param gain = 10\n.end\n", "");

        let child = |kind| {
            projection
                .groups
                .iter()
                .find(|group| group.row.kind == kind)
                .and_then(|group| group.children.first().cloned())
                .expect("child exists")
        };
        assert_eq!(
            child(NetlistNavigatorRowKind::Instances).meta.as_deref(),
            Some("nch")
        );
        let parameter = child(NetlistNavigatorRowKind::Parameters);
        assert_eq!(parameter.label, "gain");
        assert_eq!(parameter.meta.as_deref(), Some("10"));
    }

    #[test]
    fn an_expression_split_across_lexemes_is_named_rather_than_misquoted() {
        let projection = netlist_projection(
            "deck\n.param gain = {2 * k}\n.param trim={2*k}\nR9 a b {1 * k}\n.end\n",
            "",
        );

        let parameters = &projection
            .groups
            .iter()
            .find(|group| group.row.kind == NetlistNavigatorRowKind::Parameters)
            .expect("parameters group exists")
            .children;
        // `{2` is not the value of anything.
        assert_eq!(parameters[0].label, "gain");
        assert_eq!(parameters[0].meta, None);
        // The same expression written without spaces survives whole.
        assert_eq!(parameters[1].label, "trim");
        assert_eq!(parameters[1].meta.as_deref(), Some("{2*k}"));

        let instance = &projection
            .groups
            .iter()
            .find(|group| group.row.kind == NetlistNavigatorRowKind::Instances)
            .expect("instances group exists")
            .children[0];
        assert_eq!(instance.label, "R9");
        assert_eq!(instance.meta, None);
    }

    #[test]
    fn a_comparison_navigator_lists_the_regions_that_changed() {
        let diff = "--- generated-aaaa\n+++ generated-bbbb\n@@ -1,4 +1,5 @@\n deck\n-R1 in out 1k\n+R1 in out 2k\n+R2 out 0 1k\n .end\n@@ -9,3 +10,3 @@\n .ac dec 10 1 1g\n-.meas ac peak max v(out)\n+.meas ac peak min v(out)\n";
        let hunks = diff_hunks(diff);

        assert_eq!(hunks.len(), 2);
        assert_eq!(hunks[0].label, "Lines 1\u{2013}5");
        assert_eq!(hunks[0].meta, "+2 -1");
        // The row navigates to the header inside the comparison document,
        // because that is the buffer the editor is showing.
        assert_eq!(hunks[0].line, 3);
        assert_eq!(hunks[1].label, "Lines 10\u{2013}12");
        assert_eq!(hunks[1].meta, "+1 -1");
        assert_eq!(hunks[1].line, 9);
        // A hunk owns the comparison lines up to the next header, so the row
        // can say which region the caret is standing in.
        assert_eq!(hunks[0].end_line, 8);
        assert_eq!(hunks[1].end_line, 12);
        // The `---`/`+++` header is not a change.
        assert_eq!(diff_totals(diff), (3, 2));
    }

    #[test]
    fn identical_revisions_produce_no_changed_regions() {
        assert!(
            diff_hunks("--- owned-r1-aaaa\n+++ owned-r2-aaaa\n No source changes\n").is_empty()
        );
    }

    #[test]
    fn a_viewport_draws_only_the_declarations_it_can_show() {
        // 27 px rows in a 540 px panel: twenty rows and the partly scrolled
        // one at each edge, not the fifty thousand a flat deck can declare.
        let span = visible_row_span(0.0, 540.0, 27.0, 50_000);
        assert_eq!(span.start, 0);
        assert!(span.end <= 21, "drew {} rows for 540 px", span.end);

        // Scrolled: the first row is above the viewport by 1000 px.
        let scrolled = visible_row_span(-1000.0, 540.0, 27.0, 50_000);
        assert_eq!(scrolled.start, 37);
        assert!(scrolled.end >= 58 && scrolled.end <= 59, "{scrolled:?}");

        // Fewer rows than the viewport holds, and none at all.
        assert_eq!(visible_row_span(0.0, 540.0, 27.0, 4), 0..4);
        assert_eq!(visible_row_span(0.0, 540.0, 27.0, 0), 0..0);
        // Scrolled past the end: an empty span, never a reversed range.
        let past = visible_row_span(-10_000.0, 540.0, 27.0, 10);
        assert!(past.start <= past.end);
    }

    #[test]
    fn netlist_navigator_geometry_matches_mockup_and_touch_contract() {
        assert_eq!(NETLIST_OUTLINE_ROW_HEIGHT, 27.0);
        assert_eq!(NETLIST_OUTLINE_TOUCH_ROW_HEIGHT, 44.0);
        assert_eq!(NETLIST_OUTLINE_PADDING_X, 9.0);
        assert_eq!(NETLIST_OUTLINE_ICON_GAP, 7.0);
        // A declaration hangs off its group's guide line, and every top-level
        // row reserves the caret column so their icons stay in one line.
        assert_eq!(
            NetlistOutlineRowShape::Leaf.label_left(),
            NetlistOutlineRowShape::Group { expanded: true }.label_left()
        );
        assert!(
            NetlistOutlineRowShape::Child.label_left() > NetlistOutlineRowShape::Leaf.label_left()
        );
        assert_eq!(
            NetlistOutlineRowShape::Index.label_left(),
            NETLIST_OUTLINE_PADDING_X
        );
    }
}
