//! The project's own netlist source replaced in place, keeping what its
//! include cards still reach.

use std::collections::HashSet;

use super::Model;

/// How an update of the project's own netlist source that a live session
/// authorized went.
#[derive(Debug)]
pub enum LiveOwnedSource {
    /// The project's source already reads this; nothing changed.
    Unchanged,
    /// The source was replaced. It carries the root document rewritten over
    /// it, when the project keeps one, for the netlist editor to adopt.
    Applied(Option<crate::state::NetlistDocument>),
    /// Nothing changed: the project holds no source of its own, or the update
    /// would break the project's sources.
    Rejected,
}

impl Model {
    /// Apply a live-session-authorized owned-source update independently of
    /// the currently visible netlist document. Repeated delivery is
    /// convergence. `shows_owned_root` says whether the netlist editor shows
    /// the project's own root source, the source a run then reads too.
    ///
    /// The update builds its result in a copy of the model and installs it
    /// only once the project's sources validate, carrying across the notices
    /// raised and not yet shown, which a copy starts without.
    pub fn apply_live_owned_source(
        &mut self,
        source: String,
        shows_owned_root: bool,
    ) -> LiveOwnedSource {
        let Some(current) = self.workspace.content.netlist_source.as_deref() else {
            return LiveOwnedSource::Rejected;
        };
        if current == source {
            return LiveOwnedSource::Unchanged;
        }
        let next_document = if let Some(document) = &self.workspace.content.netlist_document {
            let Ok(next) = replace_root_document_preserving_dependencies(document, &source) else {
                return LiveOwnedSource::Rejected;
            };
            Some(next)
        } else {
            None
        };
        let mut candidate = self.clone();
        if !candidate
            .workspace
            .content
            .replace_editable_netlist_source(source.clone())
        {
            return LiveOwnedSource::Rejected;
        }
        if shows_owned_root {
            candidate.simulation.source.netlist_content = source;
        }
        if let Some(document) = &next_document {
            if let Some(descriptor) = candidate.workspace.content.netlist_descriptor.as_mut() {
                prune_owned_include_authority(descriptor, document);
            }
            candidate.workspace.content.netlist_document = Some(document.clone());
        }
        candidate.invalidate_source_evidence();
        if candidate
            .workspace
            .content
            .validate_simulation_configuration()
            .is_err()
        {
            return LiveOwnedSource::Rejected;
        }
        candidate.notices.prepend(std::mem::take(&mut self.notices));
        *self = candidate;
        LiveOwnedSource::Applied(next_document)
    }
}

/// `document` rewritten over `source`, keeping the dependencies its include
/// cards still reach.
pub fn replace_root_document_preserving_dependencies(
    document: &crate::state::NetlistDocument,
    source: &str,
) -> Result<crate::state::NetlistDocument, String> {
    let next_dependencies = dependencies_for_rewritten_root(document, source)?;
    let mut next = document.clone();
    next.replace_editable_source(next.content_digest(), source.as_bytes().to_vec())
        .map_err(|error| error.to_string())?;
    next.acknowledge_dependencies(next.content_digest(), next_dependencies)
        .map_err(|error| error.to_string())?;
    Ok(next)
}

/// Rebind unchanged direct include edges to their new source-order indices and
/// retain only transitive members still reachable from those edges. New or
/// renamed include cards enter the closure unresolved and can then use the
/// ordinary relink/resolution workflow. This makes adding, ordering, and
/// removing include cards a normal atomic editor operation instead of
/// discarding every unaffected dependency or leaving stale owned authority.
fn dependencies_for_rewritten_root(
    document: &crate::state::NetlistDocument,
    source: &str,
) -> Result<Vec<crate::state::DependencyMetadata>, String> {
    let includes = crate::state::parse_include_directives(source);
    let original = document.dependencies();
    let mut next = Vec::new();
    let mut reachable = HashSet::new();

    for (index, include) in includes.iter().enumerate() {
        let requested = include.locator();
        if let Some(existing) = original.iter().find(|dependency| {
            dependency.direct_include_index().is_some()
                && dependency.parent().is_none()
                && dependency.requested_locator() == requested
        }) {
            let retained = crate::state::DependencyMetadata::unresolved_direct_to(
                index,
                requested,
                existing.locator().clone(),
            )
            .map_err(|error| error.to_string())?
            .with_authority(existing.authority())
            .with_resolution(existing.resolution().clone())
            .map_err(|error| error.to_string())?;
            reachable.insert(retained.locator().logical_identity().to_owned());
            next.push(retained);
        } else {
            let display_name = std::path::Path::new(requested).file_name().map_or_else(
                || requested.to_owned(),
                |name| name.to_string_lossy().into_owned(),
            );
            let locator = crate::state::SourceLocator::try_new(requested, display_name)
                .map_err(|error| error.to_string())?;
            reachable.insert(locator.logical_identity().to_owned());
            next.push(
                crate::state::DependencyMetadata::unresolved_direct_to(index, requested, locator)
                    .map_err(|error| error.to_string())?,
            );
        }
    }

    loop {
        let before = next.len();
        let newly_reachable = original
            .iter()
            .filter(|dependency| {
                dependency
                    .parent()
                    .is_some_and(|parent| reachable.contains(parent.logical_identity()))
                    && !reachable.contains(dependency.locator().logical_identity())
            })
            .cloned()
            .collect::<Vec<_>>();
        for dependency in newly_reachable {
            reachable.insert(dependency.locator().logical_identity().to_owned());
            next.push(dependency);
        }
        if next.len() == before {
            break;
        }
    }
    Ok(next)
}

/// Keep in `descriptor` only the owned includes `document` still depends on.
pub fn prune_owned_include_authority(
    descriptor: &mut crate::state::OwnedNetlistDescriptor,
    document: &crate::state::NetlistDocument,
) {
    let retained = document
        .dependencies()
        .iter()
        .map(|dependency| dependency.locator().logical_identity())
        .collect::<HashSet<_>>();
    descriptor
        .owned_includes
        .retain(|include| retained.contains(include.logical_identity.as_str()));
}
