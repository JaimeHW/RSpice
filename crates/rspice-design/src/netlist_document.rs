//! The project-owned netlist source document.
//!
//! `ProjectWorkspace` stores a `NetlistDocument` directly, so the document
//! model, its include-directive outline, and its find/replace live here rather
//! than beside the Code & Automation surface that edits them. None of this
//! module touches egui: it owns source text, provenance, validation identity,
//! and document transitions, which must behave identically on native and in
//! the browser.

mod document;
mod outline;
mod search;

pub use document::{
    DependencyMetadata, DependencyResolution, DependencySourceAuthority, DiagnosticSeverity,
    DocumentOwnership, GeneratedArtifact, GeneratedProvenance, GeneratedSourceMapEntry,
    GenerationInput, NetlistDocument, NetlistDocumentId, SourceLocator, ValidationDiagnostic,
    content_digest,
};
pub use outline::{
    NetlistSourceIndex, OutlineEntry, OutlineEntryKind, OutlineSection, OutlineSectionKind,
};
pub use outline::{
    card_tokens, card_tokens_with_columns, parse_include_directives, same_include_graph,
};
pub use search::find_all_in_source_range_bounded_filter;
pub use search::{
    BoundedFindMatches, FindDirection, FindError, FindMatch, FindOptions,
    find_all_in_source_bounded, replace_source_ranges,
};

/// Bind canonical design-management and configuration provenance into generated source.
pub fn bind_generated_netlist_provenance(
    design_management: &rspice_design_model::design_management::DesignManagementCatalog,
    configuration_sets: &crate::configuration_set::ConfigurationSetCatalog,
    mut source: String,
) -> String {
    let insertion = source.find('\n').map_or(0, |index| index + 1);
    let mut provenance = design_management
        .semantic_digest()
        .map(|digest| format!("* RSpice design-management digest {digest}\n"))
        .unwrap_or_else(|error| format!("* RSpice design-management INVALID ({error})\n"));
    if let Some(configuration) = configuration_sets.active() {
        provenance.push_str(&format!(
            "* RSpice configuration-set {} revision {} digest {}\n",
            configuration.id(),
            configuration.revision(),
            configuration.semantic_digest()
        ));
    }
    source.insert_str(insertion, &provenance);
    source
}
