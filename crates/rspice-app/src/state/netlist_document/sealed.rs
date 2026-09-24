//! Authenticated in-memory materialization for retained netlist dependencies.
//!
//! Project persistence and archive import retain exact dependency bytes plus
//! explicit include edges. Every consumer expands those bytes through this
//! one filesystem-free resolver, so validation and execution cannot disagree
//! or reopen a mutable host file after review.

use std::collections::HashMap;
use std::path::PathBuf;

use rspice_core::netlist::{IncludeProcessor, SealedSourceBundle, SealedSourceEdge};

use super::{DependencyMetadata, NetlistDocumentId};

#[derive(Debug)]
pub(crate) struct ExpandedRetainedNetlist {
    pub source: String,
}

pub(crate) fn expand_retained_netlist_dependencies(
    document_id: NetlistDocumentId,
    root_source: &str,
    dependencies: &[DependencyMetadata],
) -> Result<ExpandedRetainedNetlist, String> {
    if dependencies.is_empty() {
        return Err("A retained netlist closure must contain at least one dependency.".to_owned());
    }
    let root = PathBuf::from(format!("C:/rspice/sealed-netlist/{document_id}/root.spice"));
    let member_path = |logical_identity: &str| {
        let identity = uuid::Uuid::new_v5(&document_id.as_uuid(), logical_identity.as_bytes());
        PathBuf::from(format!(
            "C:/rspice/sealed-netlist/{document_id}/{identity}.spice"
        ))
    };
    let mut paths = HashMap::with_capacity(dependencies.len());
    for dependency in dependencies {
        if paths
            .insert(
                dependency.locator().logical_identity().to_owned(),
                member_path(dependency.locator().logical_identity()),
            )
            .is_some()
        {
            return Err(format!(
                "Retained netlist closure repeats logical source identity '{}'.",
                dependency.locator().logical_identity()
            ));
        }
    }
    let mut sources = Vec::with_capacity(dependencies.len() + 1);
    sources.push((root.clone(), root_source.to_owned()));
    let mut edges = Vec::with_capacity(dependencies.len());
    for dependency in dependencies {
        let target = paths
            .get(dependency.locator().logical_identity())
            .cloned()
            .ok_or_else(|| {
                "Retained netlist dependency lost its virtual source identity.".to_owned()
            })?;
        let source = dependency.source().ok_or_else(|| {
            format!(
                "Retained netlist dependency '{}' has no authenticated source bytes.",
                dependency.requested_locator()
            )
        })?;
        sources.push((target.clone(), source.to_owned()));
        let owner = match dependency.parent() {
            Some(parent) => paths
                .get(parent.logical_identity())
                .cloned()
                .ok_or_else(|| {
                    format!(
                        "Retained netlist dependency '{}' references missing parent '{}'.",
                        dependency.requested_locator(),
                        parent.logical_identity()
                    )
                })?,
            None => root.clone(),
        };
        edges.push(SealedSourceEdge {
            owner,
            requested_path: dependency.requested_locator().to_owned(),
            target,
        });
    }
    let bundle = SealedSourceBundle::try_new_with_edges(sources, edges)
        .map_err(|error| format!("Retained netlist dependency closure is invalid: {error}"))?;
    let mut processor = IncludeProcessor::new_sealed(&root, bundle);
    let source = processor
        .expand_content(root_source, &root)
        .map_err(|error| {
            format!("Could not expand authenticated retained netlist dependencies: {error}")
        })?;
    Ok(ExpandedRetainedNetlist { source })
}
