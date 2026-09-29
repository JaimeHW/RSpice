//! Borrowed schematic, catalog and frozen hierarchy authority shared by design queries and netlisting.

use super::{ConfigurationExecutionBinding, ConfigurationExecutionPlan};
use crate::connectivity_contract::{
    ConnectivityContract, GlobalAliasComparisonPolicy, GlobalNetPromotionPolicy,
};
use crate::library::{Cell, LibraryCatalog, View, ViewType};
use crate::projection::{ConfigurationExecutionProjection, DesignProjection};
use crate::resolved_symbol::ResolvedCellSymbol;
use crate::schematic::component::LibraryCellInstance;
use crate::schematic::document::SchematicDocument;
use rspice_app_types::hierarchy_path::InstancePath;
use rspice_design_model::cell_view::CellViewRef;
use std::collections::HashMap;

/// Borrow the source map's symbol and global declarations without copying it.
trait SchematicBuffers {
    fn resolve_symbol(
        &self,
        libraries: &LibraryCatalog,
        binding: &LibraryCellInstance,
    ) -> Option<ResolvedCellSymbol>;
    fn explicit_global_declarations(&self) -> Vec<String>;
}

impl<S: AsRef<SchematicDocument>> SchematicBuffers for HashMap<String, S> {
    fn resolve_symbol(
        &self,
        libraries: &LibraryCatalog,
        binding: &LibraryCellInstance,
    ) -> Option<ResolvedCellSymbol> {
        crate::symbol_resolver::SymbolResolver::new(libraries, self).resolve_binding(binding)
    }
    fn explicit_global_declarations(&self) -> Vec<String> {
        let mut declarations = self
            .values()
            .flat_map(|schematic| schematic.as_ref().net_labels.iter())
            .filter(|label| label.name.ends_with('!'))
            .map(|label| label.name.clone())
            .collect::<Vec<_>>();
        declarations.sort();
        declarations.dedup();
        declarations
    }
}

/// Read-only access to project cell masters for hierarchical netlisting.
///
/// The workspace owns the design as schematic buffers keyed
/// `"library/cell/view"`; this index exposes the schematic views as
/// netlist masters, case-insensitively.
pub struct HierarchySource<'a> {
    masters: HashMap<String, &'a SchematicDocument>,
    libraries: Option<&'a LibraryCatalog>,
    schematic_buffers: Option<&'a dyn SchematicBuffers>,
    execution_plan: Option<ConfigurationExecutionPlan>,
    connectivity: Option<&'a ConnectivityContract>,
}

impl<'a> HierarchySource<'a> {
    /// Index workspace schematic buffers (keys `"library/cell/view"`).
    pub fn from_buffers<S: AsRef<SchematicDocument>>(buffers: &'a HashMap<String, S>) -> Self {
        let mut masters = HashMap::new();
        for (key, schematic) in buffers {
            let mut parts = key.split('/');
            let (Some(library), Some(cell), Some(view)) =
                (parts.next(), parts.next(), parts.next())
            else {
                continue;
            };
            masters.insert(Self::view_key(library, cell, view), schematic.as_ref());
        }
        Self {
            masters,
            libraries: None,
            schematic_buffers: None,
            execution_plan: None,
            connectivity: None,
        }
    }

    /// Index schematic buffers and governed library symbol metadata so placed
    /// cell instances can use the same authored terminal geometry as the UI.
    pub fn from_workspace<S: AsRef<SchematicDocument>>(
        libraries: &'a LibraryCatalog,
        buffers: &'a HashMap<String, S>,
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
    pub fn with_connectivity(mut self, contract: &'a ConnectivityContract) -> Self {
        self.connectivity = Some(contract);
        self
    }

    /// Bind a frozen design projection to the generator. The plan is cloned
    /// into this read-only source so later workspace edits cannot change a
    /// deck that is already being prepared.
    pub fn from_design_projection(
        libraries: &'a LibraryCatalog,
        projection: &'a DesignProjection,
    ) -> Self {
        let mut source = Self::from_workspace(libraries, projection.schematic_buffers())
            .with_connectivity(projection.connectivity());
        source.execution_plan = Some(projection.plan().clone());
        source
    }

    /// The same binding for a caller holding the shared execution handle.
    pub fn from_execution_projection(
        libraries: &'a LibraryCatalog,
        projection: &'a ConfigurationExecutionProjection,
    ) -> Self {
        Self::from_design_projection(libraries, projection)
    }

    /// Canonical promoted global for an authored label, when the project
    /// contract promotes that exact label.
    pub fn canonical_global_label(&self, name: &str) -> Option<String> {
        let contract = self.connectivity?;
        match contract.policy.global_promotion {
            GlobalNetPromotionPolicy::ExplicitReviewedDeclaration => {
                if !name.ends_with('!') {
                    return None;
                }
                if contract.policy.alias_comparison
                    == GlobalAliasComparisonPolicy::DialectCompatibility
                    && let Some(canonical) = contract.dialect_canonical_name(name)
                {
                    return Some(format!("{canonical}!"));
                }
                Some(name.to_owned())
            }
            GlobalNetPromotionPolicy::TechnologyDefinedOnly => contract
                .technology_global_canonical_name(name)
                .map(str::to_owned),
        }
    }

    /// Canonical `.GLOBAL` node names in deterministic source order.
    pub fn global_net_names(&self) -> Vec<String> {
        let Some(contract) = self.connectivity else {
            return Vec::new();
        };
        match contract.policy.global_promotion {
            GlobalNetPromotionPolicy::ExplicitReviewedDeclaration => {
                let mut canonical = std::collections::BTreeSet::<String>::new();
                for declaration in self.explicit_global_declarations() {
                    canonical.insert(
                        self.canonical_global_label(&declaration)
                            .unwrap_or(declaration),
                    );
                }
                canonical.into_iter().collect()
            }
            GlobalNetPromotionPolicy::TechnologyDefinedOnly => contract
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
        self.schematic_buffers
            .map(SchematicBuffers::explicit_global_declarations)
            .unwrap_or_default()
    }

    /// Resolve one exact Library/Cell/View schematic master.
    pub fn master_view(
        &self,
        library: &str,
        cell: &str,
        view: &str,
    ) -> Option<&'a SchematicDocument> {
        self.masters
            .get(&Self::view_key(library, cell, view))
            .copied()
    }

    pub fn execution_binding(
        &self,
        instance_path: &InstancePath,
    ) -> Option<&ConfigurationExecutionBinding> {
        self.execution_plan
            .as_ref()
            .and_then(|plan| plan.binding(instance_path))
    }

    /// A frozen plan is authoritative even when it rejected an occurrence.
    /// `None` permits placed-binding inspection only when there is no plan.
    pub fn materialized_binding(
        &self,
        path: &InstancePath,
    ) -> Result<Option<&LibraryCellInstance>, String> {
        let Some(plan) = self.execution_plan.as_ref() else {
            return Ok(None);
        };
        plan.binding(path)
            .and_then(ConfigurationExecutionBinding::materialized_binding)
            .map(Some)
            .ok_or_else(|| {
                format!("the execution plan did not resolve a materialized binding at {path}")
            })
    }

    /// The frozen plan this source was bound to, when it carries one. A source
    /// indexed straight from workspace buffers — an inspection of one cell view
    /// rather than an execution — carries none.
    pub const fn execution_plan(&self) -> Option<&ConfigurationExecutionPlan> {
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
    pub fn has_execution_plan(&self) -> bool {
        self.execution_plan
            .as_ref()
            .is_some_and(|plan| plan.configuration_id().is_some())
    }

    /// The exact cell view a placed binding names. A master is a Library/Cell/
    /// View, so the view is never dropped from the lookup: two views of one
    /// cell are two masters.
    pub fn schematic_master_for_binding(
        &self,
        binding: &LibraryCellInstance,
    ) -> Option<&'a SchematicDocument> {
        self.master_view(&binding.library, &binding.cell, &binding.view)
    }

    /// The view type of one cell view, or the schematic every buffer in this
    /// index is when no library is bound to say otherwise.
    pub fn resolved_view_type(&self, reference: &CellViewRef) -> ViewType {
        self.libraries
            .and_then(|libraries| libraries.get_library(&reference.library))
            .and_then(|library| library.get_cell(&reference.cell))
            .and_then(|cell| cell.get_view(&reference.view))
            .map_or(ViewType::Schematic, |view| view.view_type)
    }

    /// The authoritative cell and view behind one reference, which is where a
    /// declared parameter contract lives. A source with no library bound to it
    /// has no authority to read one from.
    pub fn cell_declaration(
        &self,
        reference: &CellViewRef,
    ) -> Option<(&'a Cell, Option<&'a View>)> {
        let cell = self
            .libraries?
            .get_library(&reference.library)?
            .get_cell(&reference.cell)?;
        Some((cell, cell.get_view(&reference.view)))
    }

    pub fn resolved_symbol_for(&self, binding: &LibraryCellInstance) -> Option<ResolvedCellSymbol> {
        let libraries = self.libraries?;
        let schematic_buffers = self.schematic_buffers?;
        schematic_buffers.resolve_symbol(libraries, binding)
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
