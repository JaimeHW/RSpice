//! The one index of emitted subcircuit masters.
//!
//! A master is a Library/Cell/View paired with the digest of everything its
//! subtree resolves to, and it is emitted exactly once under exactly one name.
//! Two occurrences of one cellview that resolve identically therefore share a
//! body, and two that differ anywhere below them do not — which is the whole
//! reason master identity is a digest rather than a path.
//!
//! Every defect the walk finds is typed. The generator still receives each
//! one's rendering as a string so existing consumers keep working, but the
//! typed value is what a repair action can be attached to.

use std::collections::{BTreeMap, HashMap, HashSet};
use std::fmt;
use std::rc::Rc;

use super::*;
use rspice_design::hierarchy::{
    ConfigurationExecutionBinding, ConfigurationExecutionPlan, MasterKey, assign_master_names,
    master_closure_digest,
};
use rspice_design::schematic::component::LibraryCellInstance;
use rspice_design::schematic::interface_repair::same_terminal_contract;
use rspice_design_model::cell_view::CellViewRef;
use rspice_vocabulary::product::ContentDigest;

mod emission;
pub use emission::EmissionMap;
use emission::{EmissionEdge, EmissionNode, EmittedMaster};

/// One thing that stopped the deck from describing the design.
///
/// The kind is the point: a stale interface is repaired by re-placing an
/// instance, a missing default by declaring one on the cell, and a recursive
/// hierarchy by breaking the loop. A string cannot be dispatched on.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum NetlistDefect {
    /// A cell view instantiates itself, directly or through its descendants.
    RecursiveHierarchy { chain: Vec<CellViewRef> },
    /// An occurrence binds a cell view the projection carries no master for.
    MissingMaster {
        occurrence: InstancePath,
        master: CellViewRef,
    },
    /// The master's interface no longer matches the placed instance's.
    StaleInterface {
        occurrence: InstancePath,
        master: CellViewRef,
        expected: Vec<String>,
        actual: Vec<String>,
    },
    /// A cell declares a parameter the `.subckt` header cannot carry, because
    /// the contract gives it no value to declare.
    ParameterWithoutDefault {
        master: CellViewRef,
        parameter: String,
    },
    /// An interface port is shorted to the reference node inside its own cell.
    PortTiedToGround { master: CellViewRef, port: String },
    /// A vector join whose two ends declare different conductors. The deck
    /// would have to invent or drop conductors the drawing never named, so it
    /// is refused rather than emitted narrower.
    WidthMismatch {
        /// What owns the attachment, in the words the drawing uses.
        owner: String,
        /// Conductors each end declares. A repair reads the two numbers; the
        /// detail explains them.
        declared_width: usize,
        found_width: usize,
        /// The one rendering of this mismatch, written where it was found.
        detail: String,
    },
    /// The design named an instance the hierarchy grammar cannot address, or
    /// the plan carries no binding where one is required.
    UnresolvedOccurrence {
        occurrence: InstancePath,
        detail: String,
    },
}

impl NetlistDefect {
    /// Stable tag for what kind of repair this defect needs.
    ///
    /// The rendering below explains the defect to a reader; this names it to a
    /// surface, so an offered repair is selected by the kind rather than
    /// matched out of the sentence.
    pub const fn kind(&self) -> &'static str {
        match self {
            Self::RecursiveHierarchy { .. } => "recursive-hierarchy",
            Self::MissingMaster { .. } => "missing-master",
            Self::StaleInterface { .. } => "stale-interface",
            Self::ParameterWithoutDefault { .. } => "parameter-without-default",
            Self::PortTiedToGround { .. } => "port-tied-to-ground",
            Self::WidthMismatch { .. } => "vector-width-mismatch",
            Self::UnresolvedOccurrence { .. } => "unresolved-occurrence",
        }
    }
}

impl fmt::Display for NetlistDefect {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::RecursiveHierarchy { chain } => write!(
                formatter,
                "Recursive cell hierarchy: {}",
                chain
                    .iter()
                    .map(|reference| format!("{}/{}", reference.library, reference.cell))
                    .collect::<Vec<_>>()
                    .join(" -> ")
            ),
            Self::MissingMaster { occurrence, master } => write!(
                formatter,
                "Cell instance '{}' master not found in project: {} \
                 (open the cell once, or bind a source file)",
                instance_label(occurrence),
                master.display_path()
            ),
            Self::StaleInterface {
                occurrence,
                master,
                expected,
                actual,
            } => write!(
                formatter,
                "Cell instance '{}' is stale: master {}/{} interface no longer matches the \
                 instance terminal order — the master presents [{}], the instance was placed \
                 with [{}]; re-place the instance",
                instance_label(occurrence),
                master.library,
                master.cell,
                expected.join(", "),
                actual.join(", ")
            ),
            Self::ParameterWithoutDefault { master, parameter } => write!(
                formatter,
                "Cell {}/{} requires parameter '{parameter}' but declares no default, so the \
                 .subckt header cannot declare it",
                master.library, master.cell
            ),
            Self::PortTiedToGround { master, port } => write!(
                formatter,
                "Interface port '{port}' of cell '{}/{}' is tied to ground inside the cell — \
                 its pin would float at every instantiation",
                master.library, master.cell
            ),
            Self::WidthMismatch { detail, .. } => formatter.write_str(detail),
            Self::UnresolvedOccurrence { occurrence, detail } => {
                write!(formatter, "at {occurrence}: {detail}")
            }
        }
    }
}

/// The instance name an occurrence path ends in, which is what an author reads
/// on the canvas. The design root is named by the path itself.
fn instance_label(occurrence: &InstancePath) -> String {
    occurrence
        .segments()
        .last()
        .cloned()
        .unwrap_or_else(|| occurrence.to_string())
}

/// One occurrence as the emitted deck names it.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct EmissionRow {
    /// Exact design path of the instantiation.
    pub occurrence: InstancePath,
    /// `.subckt` name the deck emitted for it.
    pub master: String,
    /// Uppercased engine spelling of the occurrence scope, e.g. `X1.X2`.
    pub engine_prefix: String,
    /// Cell view the master was generated from.
    pub master_reference: CellViewRef,
}

/// The single stale-interface check.
///
/// A placement that never recorded a terminal order predates the interface
/// contract and resolves its order from the master instead, so it cannot be
/// stale.
///
/// The defect is boxed because it carries both interfaces in full, and a stale
/// interface is the rare hard stop while this check runs on every occurrence.
pub(super) fn validate_occurrence_interface(
    master_ports: &[String],
    occurrence: &InstancePath,
    master: &CellViewRef,
    terminal_order: &[String],
) -> Result<(), Box<NetlistDefect>> {
    if terminal_order.is_empty() || same_terminal_contract(terminal_order, master_ports) {
        return Ok(());
    }
    Err(Box::new(NetlistDefect::StaleInterface {
        occurrence: occurrence.clone(),
        master: master.clone(),
        expected: master_ports.to_vec(),
        actual: terminal_order.to_vec(),
    }))
}

/// One master the deck declares.
struct MasterDraft<'a> {
    reference: CellViewRef,
    schematic: &'a SchematicDocument,
    /// The occurrence the body is generated at. Every occurrence of one master
    /// resolves identically, so the first one is exact and any other would be.
    representative: InstancePath,
    /// The header's formals: one per conductor, vector ports already expanded
    /// into the deck bit names an engine can lex. The unexpanded port list
    /// stays the cell's contract and is what a stale-interface check compares.
    formals: Vec<String>,
}

/// Master identity for one root, resolved before a single line is emitted.
pub(super) struct MasterIndex<'a> {
    drafts: BTreeMap<MasterKey, MasterDraft<'a>>,
    names: BTreeMap<MasterKey, String>,
    /// Emission order, children before parents, each master once.
    order: Vec<MasterKey>,
    emission_map: EmissionMap,
    defects: Vec<NetlistDefect>,
}

impl<'a> MasterIndex<'a> {
    /// The `.subckt` name one occurrence instantiates.
    pub(super) fn occurrence_name(&self, occurrence: &InstancePath) -> Option<&str> {
        self.emission_map.master_at(occurrence)
    }

    pub(super) const fn emission_map(&self) -> &EmissionMap {
        &self.emission_map
    }

    pub(super) fn defects(&self) -> &[NetlistDefect] {
        &self.defects
    }

    /// Walk the design from `root`, deciding what every occurrence binds to and
    /// what each master is called, without emitting anything. A placement is
    /// walked once for each distinct way it is reached, however many times it
    /// is: the occurrences under a repeated one resolve as its first did.
    pub(super) fn build(
        hierarchy: &'a HierarchySource<'a>,
        root: &'a SchematicDocument,
        root_path: &InstancePath,
    ) -> Self {
        let mut walk = MasterWalk {
            hierarchy,
            plan: hierarchy.execution_plan(),
            discovered: Vec::new(),
            drafts: BTreeMap::new(),
            order: Vec::new(),
            nodes: vec![EmissionNode::default()],
            keys: vec![None],
            visits: HashMap::new(),
            reserved: HashSet::new(),
            ancestors: Vec::new(),
            defects: Vec::new(),
        };
        walk.visit(root, root_path, 0);

        let MasterWalk {
            plan,
            discovered,
            drafts,
            order,
            mut nodes,
            keys,
            mut reserved,
            defects,
            ..
        } = walk;

        // A plan that governs this root has already named its masters, and
        // adopting those names is what keeps the receipt and the deck from
        // disagreeing about what the design executed. Every name the plan
        // assigned is reserved, including one for a master this walk never
        // reached, so a name can never mean two things across two decks of
        // one design.
        if let Some(plan) = plan {
            reserved.extend(
                plan.masters()
                    .map(|(_, record)| record.name().to_ascii_lowercase()),
            );
        }
        let mut names = BTreeMap::new();
        let mut unnamed = Vec::new();
        for (key, reference) in &discovered {
            match plan.and_then(|plan| plan.master(key)) {
                Some(record) => {
                    names.insert(key.clone(), record.name().to_owned());
                }
                None => unnamed.push((key, reference)),
            }
        }
        names.extend(assign_master_names(unnamed, &reserved));

        let mut masters = Vec::new();
        let mut emitted = HashMap::new();
        for (node, key) in nodes.iter_mut().zip(keys) {
            let Some(key) = key else {
                continue;
            };
            let master = *emitted.entry(key.clone()).or_insert_with(|| {
                masters.push(EmittedMaster {
                    name: names[&key].clone(),
                    reference: drafts[&key].reference.clone(),
                });
                masters.len() - 1
            });
            node.master = Some(master);
        }
        let emission_map = EmissionMap::new(root_path.clone(), nodes, masters);

        Self {
            drafts,
            names,
            order,
            emission_map,
            defects,
        }
    }

    /// Emit every master body once, children first.
    ///
    /// The nested generator is the one that produces the top level, run at the
    /// master's own occurrence path, so a cell netlists identically whether it
    /// is the deck's root or four levels down. The index is passed by handle
    /// because those nested generators resolve their own X-lines through it.
    pub(super) fn emit(
        index: &Rc<Self>,
        hierarchy: &'a HierarchySource<'a>,
        source_data: NetlistSourceData<'a>,
    ) -> MasterEmission {
        let mut emission = MasterEmission::default();
        for key in &index.order {
            let draft = &index.drafts[key];
            let name = &index.names[key];
            let parameters = index.declared_parameters(hierarchy, draft, &mut emission);

            let mut nested = NetlistGenerator::with_master_index(
                draft.schematic,
                hierarchy,
                draft.representative.clone(),
                draft.reference.clone(),
                Rc::clone(index),
                source_data,
            );
            nested.extract_connectivity();
            // A port tied to ground inside the cell emits node 0 in the body
            // while the header still declares the port: the pin would float at
            // every instantiation.
            for component in &draft.schematic.components {
                let Some(spec) = component.port_spec() else {
                    continue;
                };
                if let Some((_, terminal)) = component.terminal_positions().into_iter().next()
                    && nested
                        .net_at(terminal)
                        .is_some_and(|net| net.spice_name() == "0")
                {
                    emission.defects.push(NetlistDefect::PortTiedToGround {
                        master: draft.reference.clone(),
                        port: spec.name.clone(),
                    });
                }
            }
            nested.generate_library_view_includes();
            nested.generate_instances();
            nested.generate_models();
            let context = draft.reference.display_path();
            emission.errors.extend(
                nested
                    .errors()
                    .iter()
                    .map(|error| format!("in master {name} ({context}): {error}")),
            );
            emission.warnings.extend(
                nested
                    .warnings()
                    .iter()
                    .map(|warning| format!("in master {name} ({context}): {warning}")),
            );

            let header = nested.cell_origin();
            emission.blocks.push_for(String::new(), header.clone());
            emission.blocks.push_for(
                format!(".subckt {name} {}{parameters}", draft.formals.join(" ")),
                WrittenLine {
                    nets: draft.formals.clone(),
                    ..header.clone()
                },
            );
            let mut body = nested.take_lines();
            body.retain(|line| line != "* Circuit netlist" && !line.is_empty());
            emission.blocks.append(body);
            emission.blocks.push_for(format!(".ends {name}"), header);
        }
        emission
    }

    /// The formal parameters the header declares, in contract order.
    ///
    /// Only a declared default can become a formal: a `.subckt` header has no
    /// syntax for a parameter with no value, so a required one without a
    /// default is a defect rather than a silently dropped declaration.
    fn declared_parameters(
        &self,
        hierarchy: &HierarchySource<'_>,
        draft: &MasterDraft<'_>,
        emission: &mut MasterEmission,
    ) -> String {
        let Some((cell, view)) = hierarchy.cell_declaration(&draft.reference) else {
            return String::new();
        };
        let contract = match rspice_design::library::metadata::cell_parameter_contract(cell, view) {
            Ok(contract) => contract,
            Err(error) => {
                emission.errors.push(format!(
                    "cell {} has an invalid parameter contract: {error}",
                    draft.reference.display_path()
                ));
                return String::new();
            }
        };
        let mut formals = Vec::new();
        for parameter in &contract {
            match parameter.default_value.as_deref() {
                Some(default) => formals.push(format!("{}={default}", parameter.name)),
                None if parameter.required => {
                    emission
                        .defects
                        .push(NetlistDefect::ParameterWithoutDefault {
                            master: draft.reference.clone(),
                            parameter: parameter.name.clone(),
                        });
                }
                None => {}
            }
        }
        if formals.is_empty() {
            String::new()
        } else {
            format!(" params: {}", formals.join(" "))
        }
    }
}

/// Everything one emission pass produces.
#[derive(Debug, Default)]
pub(super) struct MasterEmission {
    pub(super) blocks: WrittenLines,
    pub(super) defects: Vec<NetlistDefect>,
    pub(super) errors: Vec<String>,
    pub(super) warnings: Vec<String>,
}

/// The walk that decides master identity.
struct MasterWalk<'a> {
    hierarchy: &'a HierarchySource<'a>,
    plan: Option<&'a ConfigurationExecutionPlan>,
    /// Masters in first-occurrence order, which is the order they are named in.
    discovered: Vec<(MasterKey, CellViewRef)>,
    drafts: BTreeMap<MasterKey, MasterDraft<'a>>,
    order: Vec<MasterKey>,
    /// One node per placement context walked; the top level is the first.
    nodes: Vec<EmissionNode>,
    /// The master each node was emitted against, when it was.
    keys: Vec<Option<MasterKey>>,
    /// The node and digest each placement context resolved to.
    visits: HashMap<VisitContext, (Option<ContentDigest>, usize)>,
    /// Folded names already spoken for by model-bound engine modules.
    reserved: HashSet<String>,
    /// The cell views on the path from the root to here, folded key first.
    ancestors: Vec<(String, CellViewRef)>,
    defects: Vec<NetlistDefect>,
}

/// Everything a placement's walk depends on besides its path: the drawn
/// placement, the cells above it, and what the plan binds there. Two
/// occurrences alike in these resolve alike, down to every one under them.
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
struct VisitContext {
    component: usize,
    ancestors: String,
    binding: usize,
}

impl<'a> MasterWalk<'a> {
    /// Resolve every cell instance placed in `schematic`, as the children of
    /// `node`, returning the closure digests of the masters they bind to.
    fn visit(
        &mut self,
        schematic: &'a SchematicDocument,
        path: &InstancePath,
        node: usize,
    ) -> Vec<ContentDigest> {
        let mut digests = Vec::new();
        for component in &schematic.components {
            if component.kind != ComponentType::CellInstance {
                continue;
            }
            let child_path = match path.child(&component.name) {
                Ok(child_path) => child_path,
                Err(error) => {
                    self.defects.push(NetlistDefect::UnresolvedOccurrence {
                        occurrence: path.clone(),
                        detail: format!("instance '{}' has no path: {error}", component.name),
                    });
                    continue;
                }
            };
            let context = VisitContext {
                component: std::ptr::from_ref(component).addr(),
                ancestors: self
                    .ancestors
                    .iter()
                    .map(|(identity, _)| identity.as_str())
                    .collect::<Vec<_>>()
                    .join("|"),
                binding: self
                    .plan
                    .and_then(|plan| plan.binding(&child_path))
                    .map_or(0, |binding| std::ptr::from_ref(binding).addr()),
            };
            let (digest, child) = match self.visits.get(&context) {
                Some(&visited) => visited,
                None => {
                    let child = self.nodes.len();
                    self.nodes.push(EmissionNode::default());
                    self.keys.push(None);
                    let digest = self.visit_occurrence(component, &child_path, child);
                    self.visits.insert(context, (digest, child));
                    (digest, child)
                }
            };
            self.nodes[node]
                .children
                .push(EmissionEdge::new(&component.name, child));
            if let Some(digest) = digest {
                digests.push(digest);
            }
        }
        digests
    }

    /// Resolve the occurrence at `path` as `node`.
    fn visit_occurrence(
        &mut self,
        component: &Component,
        path: &InstancePath,
        node: usize,
    ) -> Option<ContentDigest> {
        let binding = self.binding(component, path)?;
        // A source-backed, built-in, or template-bound master is an engine
        // module the deck references by its own name: it declares no `.subckt`
        // of ours, and it owns no descendants we emit.
        if binding.source_path.is_some()
            || binding.netlist_template.is_some()
            || binding.is_executable_builtin()
        {
            if let Some(module) = binding
                .module_name
                .as_deref()
                .map(str::trim)
                .filter(|module| !module.is_empty())
            {
                self.reserved.insert(module.to_ascii_lowercase());
            }
            return None;
        }

        let reference = self.reference(&binding, path);
        let identity = reference.key().to_ascii_lowercase();
        if let Some(start) = self
            .ancestors
            .iter()
            .position(|(ancestor, _)| *ancestor == identity)
        {
            let chain = self.ancestors[start..]
                .iter()
                .map(|(_, ancestor)| ancestor.clone())
                .chain(std::iter::once(reference))
                .collect::<Vec<_>>();
            self.defects
                .push(NetlistDefect::RecursiveHierarchy { chain });
            return None;
        }

        let Some(master) =
            self.hierarchy
                .master_view(&reference.library, &reference.cell, &reference.view)
        else {
            self.defects.push(NetlistDefect::MissingMaster {
                occurrence: path.clone(),
                master: reference,
            });
            return None;
        };
        let ports = master
            .interface_ports()
            .into_iter()
            .map(|port| port.name)
            .collect::<Vec<_>>();
        if let Err(defect) =
            validate_occurrence_interface(&ports, path, &reference, &binding.terminal_order)
        {
            self.defects.push(*defect);
            return None;
        }

        self.ancestors.push((identity, reference.clone()));
        let children = self.visit(master, path, node);
        self.ancestors.pop();

        let closure = self.plan.and_then(|plan| plan.binding(path)).map_or_else(
            || {
                master_closure_digest(
                    &reference,
                    self.hierarchy.resolved_view_type(&reference),
                    None,
                    false,
                    None,
                    &children,
                )
            },
            ConfigurationExecutionBinding::binding_closure_digest,
        );
        // The plan already decided which master this occurrence binds to;
        // recomputing the key here would be a second answer to that question.
        // Only an occurrence no plan governs is keyed from the walk's own
        // digest.
        let key = self
            .plan
            .and_then(|plan| plan.occurrence_master(path))
            .cloned()
            .unwrap_or_else(|| MasterKey::new(&reference, closure));
        if !self.drafts.contains_key(&key) {
            self.discovered.push((key.clone(), reference.clone()));
            self.drafts.insert(
                key.clone(),
                MasterDraft {
                    reference,
                    schematic: master,
                    representative: path.clone(),
                    formals: super::vector_nets::interface_formals(master),
                },
            );
            // Post-order: the walk above has already registered every master
            // this one instantiates, so a definition precedes its first use.
            self.order.push(key.clone());
        }
        self.keys[node] = Some(key);
        Some(closure)
    }

    /// The executable binding of one occurrence. Rejected plan entries never
    /// fall back to placed metadata; standalone inspection carries no plan.
    fn binding(
        &mut self,
        component: &Component,
        path: &InstancePath,
    ) -> Option<LibraryCellInstance> {
        match self.hierarchy.materialized_binding(path) {
            Ok(Some(binding)) => return Some(binding.clone()),
            Ok(None) => {}
            Err(detail) => {
                self.defects.push(NetlistDefect::UnresolvedOccurrence {
                    occurrence: path.clone(),
                    detail,
                });
                return None;
            }
        }
        match component.library_cell.as_ref() {
            Some(binding) => Some(binding.clone()),
            None => {
                self.defects.push(NetlistDefect::UnresolvedOccurrence {
                    occurrence: path.clone(),
                    detail: format!(
                        "cell instance '{}' is missing library binding metadata",
                        component.name
                    ),
                });
                None
            }
        }
    }

    /// The cell view an occurrence resolves to. The plan names it exactly; a
    /// placed binding names it as drawn.
    fn reference(&self, binding: &LibraryCellInstance, path: &InstancePath) -> CellViewRef {
        self.plan.and_then(|plan| plan.binding(path)).map_or_else(
            || CellViewRef::new(&binding.library, &binding.cell, &binding.view),
            |execution| execution.resolved_reference().clone(),
        )
    }
}
