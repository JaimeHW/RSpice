//! The instantiated hierarchy below one checked cellview.
//!
//! The walk decides which cellviews a check reads and where each occurs. It
//! extracts nothing itself: what it records is the shape of the hierarchy —
//! one node per distinct reached cellview, one edge per placed instance — so
//! each cellview is checked once however often it is placed, and its findings
//! are then stated at the occurrences that reach it.

use super::super::types::{DrcLocation, DrcResult, DrcSeverity, DrcViolation, DrcViolationType};
use super::{CheckReach, writes_its_own_text};
use crate::hierarchy::{HierarchySource, MasterKey};
use crate::schematic::component::Component;
use crate::schematic::component_type::ComponentType;
use crate::schematic::document::SchematicDocument;
use rspice_app_types::hierarchy_path::InstancePath;
use rspice_design_model::cell_view::CellViewRef;
use std::collections::{BTreeMap, HashMap, VecDeque};

/// The subject's index in the walk. It is the only cellview whose findings are
/// stated without an occurrence path.
pub(super) const SUBJECT: usize = 0;

/// What makes two reached cellviews one node of the walk.
///
/// Following placed bindings, a cellview is one wherever it is placed: what
/// lies below it is decided by its own drawing. Following a configuration it
/// is not, because the configuration may bind an instance differently at two
/// occurrences of the cellview that holds it. The plan's master identity is
/// exactly "this cellview with this resolved subtree", so that is the key, and
/// a node's first occurrence then speaks for all of its occurrences exactly.
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
enum ViewKey {
    Cellview(String),
    Master(MasterKey),
}

/// One cellview the walk reached.
pub(super) struct ReachedView<'a> {
    /// The cellview, or `None` for a subject the caller did not name.
    pub(super) reference: Option<CellViewRef>,
    pub(super) document: &'a SchematicDocument,
    /// Where the walk follows the plan, the occurrence path the plan is asked
    /// about this cellview's instances with: the first that reached it, which
    /// speaks for every occurrence because the plan's master identity is the
    /// node's key. The subject's is the root.
    pub(super) plan_path: Option<InstancePath>,
}

impl ReachedView<'_> {
    fn folded_reference(&self) -> Option<String> {
        self.reference
            .as_ref()
            .map(|reference| reference.key().to_ascii_lowercase())
    }
}

/// One placed instance, as an edge of the instantiated hierarchy.
struct InstanceEdge {
    parent: usize,
    child: usize,
    /// The instance reference as the drawing spells it: one path segment.
    instance: String,
}

/// One instance whose master is already one of its own enclosing cellviews.
struct RecursiveInstance {
    parent: usize,
    component_id: u64,
    component_name: String,
    repeated: CellViewRef,
    /// The enclosing cellviews from the repeated one down to the instance, as
    /// a reader can follow them.
    chain: String,
    /// The subject's own instance the cycle runs through, when the cellview
    /// the cycle repeats is the subject. A check of the subject alone states
    /// the cycle on that instance, because it is the subject's part in it.
    subject_entry: Option<(u64, String)>,
}

/// Exactly how many times one cellview occurs, and the first few paths that
/// reach it.
pub(super) struct Occurrences {
    pub(super) count: u64,
    /// The count stopped at `u64::MAX`; the true count is at least that.
    pub(super) saturated: bool,
    pub(super) paths: Vec<InstancePath>,
}

impl Occurrences {
    /// The count as a reader is told it.
    pub(super) fn count_label(&self) -> String {
        if self.saturated {
            format!("at least {}", self.count)
        } else {
            self.count.to_string()
        }
    }
}

/// How many occurrence paths one cellview's findings are repeated over.
///
/// A cellview placed a few times is worth reporting at each place: the defect
/// may be acceptable at one occurrence and not at another, and the terminal in
/// `X1` is not the terminal in `X2`. Past that the repetition would bury every
/// other finding in the report, so the finding is stated once with its exact
/// occurrence count — which is a count the reader can act on, and not an
/// occurrence quietly dropped.
pub(super) const MAX_REPORTED_OCCURRENCE_PATHS: usize = 8;

/// The instantiated hierarchy below one checked cellview.
///
/// Where an execution plan is rooted at the subject, the walk descends exactly
/// where the deck does: into the view the plan materializes at each
/// occurrence, and never into a stop view or a view that is written from its
/// own source. A plan decides that whether or not a configuration shaped it —
/// without one it still maps a placement on a cell's symbol to the cell's
/// schematic, and the deck is written from the plan either way. That needs the
/// plan's own occurrence paths, which start at the plan's root, so it holds
/// only when the subject is that root. Anywhere else — a check of one cellview
/// from the editor, or of a sub-cell — the walk follows the drawing's placed
/// bindings, which is what that drawing says it instantiates.
pub(super) struct InstanceGraph<'a> {
    pub(super) views: Vec<ReachedView<'a>>,
    edges: Vec<InstanceEdge>,
    recursive: Vec<RecursiveInstance>,
    index: HashMap<ViewKey, usize>,
}

impl<'a> InstanceGraph<'a> {
    /// Walk everything `subject` instantiates.
    ///
    /// `reference` names the subject's own cellview where the caller knows
    /// it. With it, an instance that binds the subject closes its cycle at the
    /// subject. Without it, such an instance opens a second view of the same
    /// document and the cycle is caught one level further down — the walk
    /// still terminates and still reports, but the subject's findings are
    /// stated once more under that instance.
    pub(super) fn walk(
        subject: &'a SchematicDocument,
        reference: Option<CellViewRef>,
        hierarchy: &HierarchySource<'a>,
    ) -> Self {
        // The plan's paths start at its root, so it can be asked about this
        // walk's occurrences only when the subject is that root.
        let follows_plan = reference.as_ref().is_some_and(|reference| {
            hierarchy
                .execution_plan()
                .is_some_and(|plan| plan.root().key().eq_ignore_ascii_case(&reference.key()))
        });
        let mut graph = Self {
            views: vec![ReachedView {
                reference: reference.clone(),
                document: subject,
                plan_path: follows_plan.then(InstancePath::root),
            }],
            edges: Vec::new(),
            recursive: Vec::new(),
            index: HashMap::new(),
        };
        if let Some(reference) = reference {
            graph.index.insert(
                ViewKey::Cellview(reference.key().to_ascii_lowercase()),
                SUBJECT,
            );
        }
        graph.descend(SUBJECT, hierarchy, &mut vec![(SUBJECT, None)]);
        graph
    }

    /// Add every instance of one cellview, descending into each master the
    /// first time it is reached.
    ///
    /// `enclosing` is the chain of views from the subject to this one, each
    /// with the instance that entered it. An instance whose master is already
    /// on it closes a cycle: it is reported and not followed, because there is
    /// no finite expansion to check.
    fn descend(
        &mut self,
        parent: usize,
        hierarchy: &HierarchySource<'a>,
        enclosing: &mut Vec<(usize, Option<(u64, String)>)>,
    ) {
        let document = self.views[parent].document;
        let parent_path = self.views[parent].plan_path.clone();
        let mut instances = document
            .components
            .iter()
            .filter(|component| component.kind == ComponentType::CellInstance)
            .collect::<Vec<_>>();
        instances.sort_by_key(|component| component.id);

        for instance in instances {
            // A finding below this instance is named by its occurrence path,
            // so the instance's own reference has to be a path segment — as it
            // has to be for the deck, which addresses the occurrence the same
            // way. An unusable reference is an error on this very instance
            // from the reference rules, and the depth and length limits are
            // the ones the hierarchy resolver stops at and reports; in every
            // case the design surface states the boundary, so the walk does
            // not invent a second finding for it.
            //
            // Following the plan, the plan is asked by occurrence path, and an
            // occurrence past the limits is one it could not resolve either.
            // Following the drawing, what an instance binds does not depend on
            // where its parent occurs, so the segment alone decides whether
            // the walk goes below it; which occurrences have a path is
            // `occurrences`' to work out. Taking the first path that reached
            // the parent instead made a subtree's walk depend on which
            // occurrence happened to be visited first.
            let child_path = match &parent_path {
                Some(parent_path) => match parent_path.child(&instance.name) {
                    Ok(path) => Some(path),
                    Err(_) => continue,
                },
                None if InstancePath::root().child(&instance.name).is_err() => continue,
                None => None,
            };
            let Some((reference, master, key)) =
                self.target(instance, child_path.as_ref(), hierarchy)
            else {
                continue;
            };

            let folded = reference.key().to_ascii_lowercase();
            if let Some(position) = enclosing.iter().position(|(view, _)| {
                self.views[*view].folded_reference().as_deref() == Some(folded.as_str())
            }) {
                let chain = self.chain(&enclosing[position..], &reference);
                let subject_entry = (enclosing[position].0 == SUBJECT).then(|| {
                    if parent == SUBJECT {
                        (instance.id, instance.name.clone())
                    } else {
                        enclosing[1]
                            .1
                            .clone()
                            .expect("every view below the subject was entered by an instance")
                    }
                });
                self.recursive.push(RecursiveInstance {
                    parent,
                    component_id: instance.id,
                    component_name: instance.name.clone(),
                    repeated: reference,
                    chain,
                    subject_entry,
                });
                continue;
            }

            let child = match self.index.get(&key).copied() {
                Some(child) => child,
                None => {
                    let child = self.views.len();
                    self.views.push(ReachedView {
                        reference: Some(reference),
                        document: master,
                        plan_path: child_path,
                    });
                    self.index.insert(key, child);
                    enclosing.push((child, Some((instance.id, instance.name.clone()))));
                    self.descend(child, hierarchy, enclosing);
                    enclosing.pop();
                    child
                }
            };
            self.edges.push(InstanceEdge {
                parent,
                child,
                instance: instance.name.clone(),
            });
        }
    }

    /// The schematic one instance descends into, or `None` where the deck
    /// descends into none.
    ///
    /// `path` is the instance's occurrence path where the walk follows the
    /// plan, and `None` where it follows the drawing.
    ///
    /// A binding that carries its own source or netlist text is a leaf: that
    /// text is what the deck writes, so there is no schematic below it for a
    /// rule to read.
    fn target(
        &self,
        instance: &Component,
        path: Option<&InstancePath>,
        hierarchy: &HierarchySource<'a>,
    ) -> Option<(CellViewRef, &'a SchematicDocument, ViewKey)> {
        if let Some(path) = path {
            // An occurrence the plan rejected or never reached is the plan's
            // to report, and the deck emits nothing below it.
            let execution = hierarchy.execution_binding(path)?;
            let materialized = execution.materialized_binding()?;
            if execution.stop_boundary() || writes_its_own_text(materialized) {
                return None;
            }
            let reference = execution.resolved_reference().clone();
            let master =
                hierarchy.master_view(&reference.library, &reference.cell, &reference.view)?;
            let key = hierarchy
                .execution_plan()
                .and_then(|plan| plan.occurrence_master(path))
                .cloned()
                .map_or_else(
                    || ViewKey::Cellview(reference.key().to_ascii_lowercase()),
                    ViewKey::Master,
                );
            return Some((reference, master, key));
        }
        let binding = instance.library_cell.as_ref()?;
        if writes_its_own_text(binding) {
            return None;
        }
        let master = hierarchy.schematic_master_for_binding(binding)?;
        let reference = HierarchySource::placed_master(binding);
        let key = ViewKey::Cellview(reference.key().to_ascii_lowercase());
        Some((reference, master, key))
    }

    /// The cycle as a reader follows it: from the repeated cellview down to the
    /// instance that reaches it again.
    fn chain(
        &self,
        enclosing: &[(usize, Option<(u64, String)>)],
        reference: &CellViewRef,
    ) -> String {
        let mut chain = enclosing
            .iter()
            .filter_map(|(view, _)| self.views[*view].reference.as_ref())
            .map(CellViewRef::display_path)
            .collect::<Vec<_>>();
        chain.push(reference.display_path());
        chain.join(" -> ")
    }

    /// State the cycles that belong in one cellview's own report.
    ///
    /// Across the hierarchy, a cycle is stated once, in the cellview whose
    /// instance closes it. A check of the subject alone states every cycle
    /// that runs through the subject, on the subject's own instance in it —
    /// otherwise a cycle between two cellviews would close in the other one
    /// whichever of the two was checked, and neither check would say so.
    pub(super) fn append_recursive_findings(
        &self,
        view: usize,
        reach: CheckReach,
        severity_overrides: &BTreeMap<DrcViolationType, DrcSeverity>,
        result: &mut DrcResult,
    ) {
        for recursive in &self.recursive {
            let (component_id, component_name, message) = if recursive.parent == view {
                let instance = instance_label(recursive.component_id, &recursive.component_name);
                (
                    recursive.component_id,
                    recursive.component_name.clone(),
                    format!(
                        "{instance} reaches {}, which already encloses it: {}",
                        recursive.repeated.display_path(),
                        recursive.chain
                    ),
                )
            } else if view == SUBJECT
                && reach == CheckReach::Cellview
                && let Some((id, name)) = recursive.subject_entry.as_ref()
            {
                let instance = instance_label(*id, name);
                (
                    *id,
                    name.clone(),
                    format!(
                        "{instance} reaches {} again through its instances: {}",
                        recursive.repeated.display_path(),
                        recursive.chain
                    ),
                )
            } else {
                continue;
            };
            let mut violation = DrcViolation::new(
                result.total_count(),
                DrcViolationType::RecursiveInstantiation,
                message,
                DrcLocation::Component {
                    id: component_id,
                    name: component_name,
                },
            );
            if let Some(severity) =
                severity_overrides.get(&DrcViolationType::RecursiveInstantiation)
            {
                violation.severity = *severity;
            }
            result.add_violation(violation);
        }
    }

    /// Exactly how often each cellview occurs, with the first few paths that
    /// reach it.
    ///
    /// The count is computed over the edges rather than by walking every
    /// occurrence, so a wide hierarchy costs nothing to count exactly even
    /// where enumerating its occurrences would be hopeless. Cycles were
    /// excluded as they were found, so what remains is acyclic and one pass in
    /// dependency order is enough. A count past `u64::MAX` stops there and
    /// says so rather than wrapping.
    pub(super) fn occurrences(&self) -> Vec<Occurrences> {
        let mut occurrences = self
            .views
            .iter()
            .map(|_| Occurrences {
                count: 0,
                saturated: false,
                paths: Vec::new(),
            })
            .collect::<Vec<_>>();
        occurrences[SUBJECT].count = 1;
        occurrences[SUBJECT].paths = vec![InstancePath::root()];

        let mut outgoing = vec![Vec::new(); self.views.len()];
        let mut remaining = vec![0usize; self.views.len()];
        for (index, edge) in self.edges.iter().enumerate() {
            outgoing[edge.parent].push(index);
            remaining[edge.child] += 1;
        }

        let mut ready = VecDeque::from([SUBJECT]);
        while let Some(parent) = ready.pop_front() {
            for edge in &outgoing[parent] {
                let edge = &self.edges[*edge];
                let (count, overflowed) = occurrences[edge.child]
                    .count
                    .overflowing_add(occurrences[parent].count);
                let saturated = overflowed
                    || occurrences[parent].saturated
                    || occurrences[edge.child].saturated;
                occurrences[edge.child].count = if overflowed { u64::MAX } else { count };
                occurrences[edge.child].saturated = saturated;
                let mut extended = Vec::new();
                for path in &occurrences[parent].paths {
                    if occurrences[edge.child].paths.len() + extended.len()
                        >= MAX_REPORTED_OCCURRENCE_PATHS
                    {
                        break;
                    }
                    // A path past the hierarchy's length limit cannot be
                    // stated; the count still includes it, and the statement
                    // says how many are missing.
                    if let Ok(path) = path.child(&edge.instance) {
                        extended.push(path);
                    }
                }
                occurrences[edge.child].paths.extend(extended);
                remaining[edge.child] -= 1;
                if remaining[edge.child] == 0 {
                    ready.push_back(edge.child);
                }
            }
        }

        // A cellview whose every occurrence is past the limits keeps no path:
        // the deck cannot address it either, and there is nowhere to state a
        // finding inside it.
        for occurrence in &mut occurrences {
            occurrence.count = occurrence.count.max(1);
        }
        occurrences
    }
}

/// What a finding calls one instance: its reference where it has one, and its
/// identity otherwise.
fn instance_label(id: u64, name: &str) -> String {
    if name.trim().is_empty() {
        format!("component #{id}")
    } else {
        name.to_owned()
    }
}
