//! Read-only access to the design a deck is generated from.
//!
//! This is the generator's whole view of the project: which schematic buffer
//! holds each Library/Cell/View master, which libraries authorize their
//! symbols and parameter contracts, which connectivity contract promotes a
//! label to a global, and which frozen execution plan resolved the hierarchy.
//! Deciding what each master is called and emitting its body belongs to
//! [`super::master_index`]; this module only answers questions about the
//! design.

use super::*;
use crate::state::workspace::{
    ConfigurationExecutionBinding, ConfigurationExecutionPlan, ConfigurationExecutionProjection,
    DesignProjection,
};
use crate::state::{LibraryCellInstance, LibraryManager, ResolvedCellSymbol, SymbolResolver};

/// Read-only access to project cell masters for hierarchical netlisting.
///
/// The workspace owns the design as schematic buffers keyed
/// `"library/cell/view"`; this index exposes the schematic views as
/// netlist masters, case-insensitively.
pub struct HierarchySource<'a> {
    masters: HashMap<String, &'a SchematicState>,
    libraries: Option<&'a LibraryManager>,
    schematic_buffers: Option<&'a HashMap<String, SchematicState>>,
    execution_plan: Option<ConfigurationExecutionPlan>,
    connectivity: Option<&'a crate::state::ConnectivityContract>,
    data_root: Option<std::path::PathBuf>,
}

impl<'a> HierarchySource<'a> {
    /// Index workspace schematic buffers (keys `"library/cell/view"`).
    pub fn from_buffers(buffers: &'a HashMap<String, SchematicState>) -> Self {
        let mut masters = HashMap::new();
        for (key, schematic) in buffers {
            let mut parts = key.split('/');
            let (Some(library), Some(cell), Some(view)) =
                (parts.next(), parts.next(), parts.next())
            else {
                continue;
            };
            masters.insert(Self::view_key(library, cell, view), schematic);
        }
        Self {
            masters,
            libraries: None,
            schematic_buffers: None,
            execution_plan: None,
            connectivity: None,
            data_root: None,
        }
    }

    /// Bind the directory that project-relative data-file references resolve
    /// against — normally the folder holding the `.rspiceproj`.
    ///
    /// Absent, a relative reference is emitted verbatim and the engine resolves
    /// it against the process working directory. That is right for a deck the
    /// user assembled by hand and wrong for a saved project, which is why the
    /// execution paths set it and inspection-only callers do not.
    pub fn with_data_root(mut self, root: impl Into<std::path::PathBuf>) -> Self {
        self.data_root = Some(root.into());
        self
    }

    /// Directory that project-relative data-file references resolve against.
    pub fn data_root(&self) -> Option<&std::path::Path> {
        self.data_root.as_deref()
    }

    /// Index workspace schematic buffers and library symbol metadata so placed
    /// cell instances can use the same authored terminal geometry as the UI.
    pub fn from_workspace(
        libraries: &'a LibraryManager,
        buffers: &'a HashMap<String, SchematicState>,
    ) -> Self {
        let mut source = Self::from_buffers(buffers);
        source.libraries = Some(libraries);
        source.schematic_buffers = Some(buffers);
        source
    }

    /// Bind the project's exact connectivity contract, so inspection and
    /// executable generation resolve technology globals and dialect aliases
    /// identically.
    ///
    /// A source with no contract promotes nothing: the contract is the only
    /// authority on which authored label is a global node, and a source that
    /// invented one would emit a deck the project disagrees with.
    pub fn with_connectivity(mut self, contract: &'a crate::state::ConnectivityContract) -> Self {
        self.connectivity = Some(contract);
        self
    }

    /// Bind a frozen design projection to the generator. The plan is cloned
    /// into this read-only source so later workspace edits cannot change a
    /// deck that is already being prepared.
    pub fn from_design_projection(
        libraries: &'a LibraryManager,
        projection: &'a DesignProjection,
    ) -> Self {
        let mut source = Self::from_workspace(libraries, projection.schematic_buffers())
            .with_connectivity(projection.connectivity());
        source.execution_plan = projection.plan().cloned();
        source
    }

    /// The same binding for a caller holding the shared execution handle.
    pub fn from_execution_projection(
        libraries: &'a LibraryManager,
        projection: &'a ConfigurationExecutionProjection,
    ) -> Self {
        Self::from_design_projection(libraries, projection)
    }

    /// An empty source — netlisting behaves exactly as before hierarchy
    /// support (every project-cell instance is an unresolved master).
    #[cfg(test)]
    pub fn empty() -> Self {
        Self {
            masters: HashMap::new(),
            libraries: None,
            schematic_buffers: None,
            execution_plan: None,
            connectivity: None,
            data_root: None,
        }
    }

    /// Canonical promoted global for an authored label, when the project
    /// contract promotes that exact label.
    pub(super) fn canonical_global_label(&self, name: &str) -> Option<String> {
        let contract = self.connectivity?;
        match contract.policy.global_promotion {
            crate::state::GlobalNetPromotionPolicy::ExplicitReviewedDeclaration => {
                if !name.ends_with('!') {
                    return None;
                }
                if contract.policy.alias_comparison
                    == crate::state::GlobalAliasComparisonPolicy::DialectCompatibility
                    && let Some(canonical) = contract.dialect_canonical_name(name)
                {
                    return Some(format!("{canonical}!"));
                }
                Some(name.to_owned())
            }
            crate::state::GlobalNetPromotionPolicy::TechnologyDefinedOnly => contract
                .technology_global_canonical_name(name)
                .map(str::to_owned),
        }
    }

    /// Canonical `.GLOBAL` node names in deterministic source order.
    pub(super) fn global_net_names(&self) -> Vec<String> {
        let Some(contract) = self.connectivity else {
            return Vec::new();
        };
        match contract.policy.global_promotion {
            crate::state::GlobalNetPromotionPolicy::ExplicitReviewedDeclaration => {
                let mut canonical = std::collections::BTreeSet::<String>::new();
                for declaration in self.explicit_global_declarations() {
                    canonical.insert(
                        self.canonical_global_label(&declaration)
                            .unwrap_or(declaration),
                    );
                }
                canonical.into_iter().collect()
            }
            crate::state::GlobalNetPromotionPolicy::TechnologyDefinedOnly => contract
                .technology_global_nets
                .as_ref()
                .map(|catalog| {
                    let mut names = catalog
                        .nets
                        .iter()
                        .map(|group| group.canonical_name.clone())
                        .collect::<Vec<_>>();
                    names.sort();
                    names.dedup();
                    names
                })
                .unwrap_or_default(),
        }
    }

    fn explicit_global_declarations(&self) -> Vec<String> {
        let mut declarations = self
            .schematic_buffers
            .into_iter()
            .flat_map(|buffers| buffers.values())
            .flat_map(|schematic| schematic.net_labels.iter())
            .filter(|label| label.name.ends_with('!'))
            .map(|label| label.name.clone())
            .collect::<Vec<_>>();
        declarations.sort();
        declarations.dedup();
        declarations
    }

    /// Register a master directly (tests, ad-hoc callers).
    #[cfg(test)]
    pub fn insert(&mut self, library: &str, cell: &str, schematic: &'a SchematicState) {
        self.masters
            .insert(Self::view_key(library, cell, "schematic"), schematic);
    }

    /// Resolve one exact Library/Cell/View schematic master.
    pub fn master_view(&self, library: &str, cell: &str, view: &str) -> Option<&'a SchematicState> {
        self.masters
            .get(&Self::view_key(library, cell, view))
            .copied()
    }

    pub(super) fn execution_binding(
        &self,
        instance_path: &InstancePath,
    ) -> Option<&ConfigurationExecutionBinding> {
        self.execution_plan
            .as_ref()
            .and_then(|plan| plan.binding(instance_path))
    }

    /// The frozen plan this source was bound to, when it carries one. A source
    /// indexed straight from workspace buffers — an inspection of one cell view
    /// rather than an execution — carries none.
    pub(super) const fn execution_plan(&self) -> Option<&ConfigurationExecutionPlan> {
        self.execution_plan.as_ref()
    }

    /// Whether a configuration may rebind this source's instances.
    ///
    /// Every projection seals a plan, but only a configuration re-selects which
    /// view an occurrence binds to. A caller with no instance path cannot ask
    /// the plan what one occurrence resolved to, and answering from the placed
    /// binding instead would judge a master the configuration may have
    /// replaced; such a caller asks this and declines to answer rather than
    /// answering wrongly. Without a configuration the placed binding is the
    /// resolved one, so there is nothing to decline.
    pub(crate) fn has_execution_plan(&self) -> bool {
        self.execution_plan
            .as_ref()
            .is_some_and(|plan| plan.configuration_id().is_some())
    }

    /// The exact cell view a placed binding names. A master is a Library/Cell/
    /// View, so the view is never dropped from the lookup: two views of one
    /// cell are two masters.
    pub(crate) fn schematic_master_for_binding(
        &self,
        binding: &LibraryCellInstance,
    ) -> Option<&'a SchematicState> {
        self.master_view(&binding.library, &binding.cell, &binding.view)
    }

    /// The view type of one cell view, or the schematic every buffer in this
    /// index is when no library is bound to say otherwise.
    pub(super) fn resolved_view_type(&self, reference: &CellViewRef) -> crate::state::ViewType {
        self.libraries
            .and_then(|libraries| libraries.get_library(&reference.library))
            .and_then(|library| library.get_cell(&reference.cell))
            .and_then(|cell| cell.get_view(&reference.view))
            .map_or(crate::state::ViewType::Schematic, |view| view.view_type)
    }

    /// The authoritative cell and view behind one reference, which is where a
    /// declared parameter contract lives. A source with no library bound to it
    /// has no authority to read one from.
    pub(super) fn cell_declaration(
        &self,
        reference: &CellViewRef,
    ) -> Option<(&'a crate::state::Cell, Option<&'a crate::state::View>)> {
        let cell = self
            .libraries?
            .get_library(&reference.library)?
            .get_cell(&reference.cell)?;
        Some((cell, cell.get_view(&reference.view)))
    }

    pub fn resolved_symbol_for(&self, binding: &LibraryCellInstance) -> Option<ResolvedCellSymbol> {
        let libraries = self.libraries?;
        let schematic_buffers = self.schematic_buffers?;
        SymbolResolver::new(libraries, schematic_buffers).resolve_binding(binding)
    }

    fn view_key(library: &str, cell: &str, view: &str) -> String {
        format!(
            "{}/{}/{}",
            library.to_ascii_lowercase(),
            cell.to_ascii_lowercase(),
            view.to_ascii_lowercase()
        )
    }
}

impl<'a> NetlistGenerator<'a> {
    /// Emit one `.SUBCKT` definition per master this schematic (transitively)
    /// instantiates. Runs after includes and before instances, so a definition
    /// precedes its first use, and it publishes the master index the instance
    /// pass then names its X-lines from.
    pub(super) fn generate_subcircuit_definitions(&mut self) {
        let Some(hierarchy) = self.hierarchy else {
            return;
        };
        let index = std::rc::Rc::new(MasterIndex::build(
            hierarchy,
            self.schematic,
            &self.hierarchy_path,
        ));
        self.record_defects(index.defects().to_vec());
        self.emission_map
            .extend(index.emission_map().iter().cloned());
        let emission = MasterIndex::emit(&index, hierarchy);
        self.masters = Some(index);

        self.errors.extend(emission.errors);
        self.warnings.extend(emission.warnings);
        self.record_defects(emission.defects);
        if !emission.blocks.is_empty() {
            self.lines.push("* Cell definitions".to_owned());
            self.lines.extend(emission.blocks);
            self.lines.push(String::new());
        }
    }

    /// Retain each typed defect and its rendering. The string half is what
    /// every existing consumer reads; the typed half is what a repair action
    /// can be attached to.
    fn record_defects(&mut self, defects: Vec<NetlistDefect>) {
        for defect in defects {
            self.errors.push(defect.to_string());
            self.defects.push(defect);
        }
    }
}

#[cfg(test)]
mod tests;
