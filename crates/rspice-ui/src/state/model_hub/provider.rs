//! One part index over every place a model can come from.
//!
//! A part is a part whether it is compiled into this build, installed from a
//! signed pack, merely listed in the catalog, or retained inside the open
//! project. Four sources, one row type: a caller sorts, filters, and searches
//! the whole library without first deciding which of the four it is looking
//! at, and every row says plainly where it came from and what can be done
//! with it right now.
//!
//! The row carries provenance and state as separate facts because they answer
//! separate questions. Provenance is where the bytes are; state is what the
//! engine can do about them. A catalog row for a pack this build cannot run is
//! still a real part with real provenance — it is simply not installable, and
//! saying so is more useful than hiding it.

use std::{collections::BTreeMap, path::PathBuf};

use rspice_pack::{PartKind, Snapshot, SnapshotRelease};

use super::{Recalls, store::InstalledPack};
use crate::state::model_library::{ModelLibrary, ModelSourceAuthority};

/// Where a part's definition comes from.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum PartProvenance {
    /// Compiled into this build. Always present, never installable.
    Foundation,
    /// A verified pack release on this machine.
    InstalledPack { pack_id: String, version: String },
    /// A published release listed by the catalog snapshot.
    RemoteRelease { pack_id: String, version: String },
    /// Bytes the open project retained when the part was added.
    ProjectRetained { library: String },
}

/// What a caller can do with a row now.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum PartState {
    /// Usable now, from bytes already on this machine.
    Installed,
    /// Published and installable, but not downloaded.
    Available,
    /// Installed, and the catalog lists a newer release of the same pack.
    UpdateAvailable { installed: String, latest: String },
    /// The release requires capabilities this engine build does not offer.
    Incompatible { missing: Vec<String> },
}

/// One addressable part, from whichever source holds it.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ModelHubPartRow {
    pub part_id: String,
    pub kind: PartKind,
    /// What class of device the part is. A pack declares this in its signed
    /// manifest, in the catalog's vocabulary — `diode`, `opamp`. A library has
    /// no such declaration, so a library row answers with the parsed card's
    /// own model type instead; the two vocabularies are not merged, because
    /// inventing a catalog class for a card that never claimed one would make
    /// the row assert something nobody wrote.
    pub device: String,
    pub terminals: Vec<String>,
    pub provenance: PartProvenance,
    pub state: PartState,
    /// The pack's display name, when the row came from a pack.
    ///
    /// A foundation or project-retained row has none. The project's own
    /// libraries are not packs, and a library retained *from* a pack records
    /// the pack's id — never the name that pack published itself under, which
    /// nothing in the project ever kept.
    pub pack_name: Option<String>,
    /// Where the defining source lives, when it lives anywhere addressable.
    /// A remote row has none: nothing has been downloaded.
    pub source: Option<PathBuf>,
}

impl ModelHubPartRow {
    /// The pack this row belongs to, when it belongs to one.
    pub fn pack_id(&self) -> Option<&str> {
        match &self.provenance {
            PartProvenance::InstalledPack { pack_id, .. }
            | PartProvenance::RemoteRelease { pack_id, .. } => Some(pack_id),
            PartProvenance::Foundation | PartProvenance::ProjectRetained { .. } => None,
        }
    }
}

/// Capabilities a release needs that this engine build does not offer.
pub fn missing_capabilities(required: &[String]) -> Vec<String> {
    required
        .iter()
        .filter(|capability| !rspice_core::engine_supports_capability(capability))
        .cloned()
        .collect()
}

/// Orders two semantic versions by precedence.
///
/// Enough of Semantic Versioning 2.0.0 to answer the one question the shelf
/// asks: is the catalog offering something newer than what is installed. Both
/// values have already been proved well formed — one by the signed manifest,
/// the other by the signed snapshot — so this compares rather than validates.
///
/// This is the crate's only version ordering. Every place that decides which
/// release is newest — the part index here, and the packs table in the Models
/// workspace — calls it, because a second ordering is a second answer: byte
/// comparison ranks `9.0.0` above `10.0.0` and a pre-release above its own
/// release, and a shelf that disagrees with the index about which release is
/// current offers an update that does not exist or hides one that does.
pub(crate) fn precedence(left: &str, right: &str) -> std::cmp::Ordering {
    fn split(value: &str) -> (Vec<u64>, Option<String>) {
        let core = value
            .split(['-', '+'])
            .next()
            .unwrap_or_default()
            .split('.')
            .map(|number| number.parse::<u64>().unwrap_or_default())
            .collect();
        let prerelease = value
            .split_once('-')
            .map(|(_, rest)| rest.split('+').next().unwrap_or_default().to_owned())
            .filter(|rest| !rest.is_empty());
        (core, prerelease)
    }

    let (left_core, left_pre) = split(left);
    let (right_core, right_pre) = split(right);
    left_core.cmp(&right_core).then_with(|| {
        // A release outranks any pre-release of the same core version.
        match (&left_pre, &right_pre) {
            (None, None) => std::cmp::Ordering::Equal,
            (None, Some(_)) => std::cmp::Ordering::Greater,
            (Some(_), None) => std::cmp::Ordering::Less,
            (Some(left), Some(right)) => left.cmp(right),
        }
    })
}

/// What class of device one of a project library's parts is.
///
/// A pack part carries the class its signed manifest declares. A library part
/// has no declaration to read, so this answers from what parsing produced: the
/// card's own model type, or — for a subcircuit, which is a netlist of devices
/// rather than a device — that it is a subcircuit.
///
/// Both maps are consulted because the two hold the same namespace at
/// different section scopes: `models` is the projection for the active corner,
/// and a card defined only outside it is still a part the project holds.
fn library_part_device(library: &ModelLibrary, part_id: &str, kind: PartKind) -> String {
    match kind {
        PartKind::Subckt => "subcircuit".to_owned(),
        PartKind::Model => library
            .models
            .get(part_id)
            .or_else(|| library.top_level_models.get(part_id))
            .map_or_else(
                || "unclassified".to_owned(),
                |model| model.model_type.display_name().to_owned(),
            ),
    }
}

/// The newest release of a pack the snapshot offers.
///
/// A recalled release is not a candidate. Offering an update onto one would be
/// the recall pointing at itself: the row would say "3.1.0 available" about the
/// exact release the publisher withdrew.
fn newest_release<'a>(
    releases: &'a [SnapshotRelease],
    pack_id: &str,
    recalled: &Recalls,
) -> Option<&'a SnapshotRelease> {
    releases
        .iter()
        .filter(|release| recalled.reason(pack_id, &release.version).is_none())
        .max_by(|left, right| precedence(&left.version, &right.version))
}

/// Builds the unified part index.
///
/// `libraries` is the project's model-library set, which supplies both the
/// embedded foundation rows and the project-retained rows; `installed` and
/// `snapshot` supply the pack rows. A part installed from a pack is reported
/// once, as installed — the catalog row it came from is not repeated, because
/// two rows for one part would make every count in the shelf wrong.
///
/// `snapshot` is what the hub may still *offer*: the caller passes `None` for
/// an expired catalog, and `recalled` withholds individual releases from an
/// unexpired one. Neither touches the rows for what is already here, which is
/// the point — a client that stopped listing a project's own parts because a
/// date passed would have made a trust check into an outage.
pub(crate) fn part_index(
    libraries: &[&ModelLibrary],
    installed: &[InstalledPack],
    snapshot: Option<&Snapshot>,
    recalled: &Recalls,
    pack_root: Option<&std::path::Path>,
) -> Vec<ModelHubPartRow> {
    let mut rows = Vec::new();
    let installed_versions: BTreeMap<&str, &InstalledPack> = installed
        .iter()
        .map(|pack| (pack.pack_id(), pack))
        .collect();

    for library in libraries {
        let provenance = match library.source_authority {
            ModelSourceAuthority::BuiltIn => PartProvenance::Foundation,
            _ => PartProvenance::ProjectRetained {
                library: library.name.clone(),
            },
        };
        // One row per definition name. A name defined as both a model card and
        // a subcircuit is one part with the subcircuit's interface, which is
        // also how the resolver reads it.
        let mut named: BTreeMap<String, PartKind> = BTreeMap::new();
        for name in library.models.keys().chain(library.top_level_models.keys()) {
            named.entry(name.clone()).or_insert(PartKind::Model);
        }
        for name in library.subcircuits.keys() {
            named.insert(name.clone(), PartKind::Subckt);
        }
        for (part_id, kind) in named {
            let terminals = library
                .subcircuits
                .get(&part_id)
                .map(|interface| interface.ports.clone())
                .unwrap_or_default();
            rows.push(ModelHubPartRow {
                device: library_part_device(library, &part_id, kind),
                part_id,
                kind,
                terminals,
                provenance: provenance.clone(),
                state: PartState::Installed,
                pack_name: None,
                source: library.root_path.clone(),
            });
        }
    }

    for pack in installed {
        let latest = snapshot
            .and_then(|snapshot| {
                snapshot
                    .packs
                    .iter()
                    .find(|listed| listed.id == pack.pack_id())
            })
            .and_then(|listed| newest_release(&listed.releases, pack.pack_id(), recalled))
            .map(|release| release.version.clone());
        let state = match latest {
            Some(latest) if precedence(&latest, pack.version()) == std::cmp::Ordering::Greater => {
                PartState::UpdateAvailable {
                    installed: pack.version().to_owned(),
                    latest,
                }
            }
            _ => PartState::Installed,
        };
        for part in &pack.manifest.parts {
            rows.push(ModelHubPartRow {
                part_id: part.id.clone(),
                kind: part.kind,
                device: part.device.clone(),
                terminals: part.terminals.clone(),
                provenance: PartProvenance::InstalledPack {
                    pack_id: pack.pack_id().to_owned(),
                    version: pack.version().to_owned(),
                },
                state: state.clone(),
                pack_name: Some(pack.manifest.pack.name.clone()),
                source: pack_root.map(|root| {
                    let mut path = root.join(pack.pack_id()).join(pack.version()).join("files");
                    for segment in part.source.path.split('/') {
                        path.push(segment);
                    }
                    path
                }),
            });
        }
    }

    if let Some(snapshot) = snapshot {
        for listed in &snapshot.packs {
            for release in &listed.releases {
                // An installed release of this pack is already reported above.
                if installed_versions
                    .get(listed.id.as_str())
                    .is_some_and(|installed| installed.version() == release.version)
                {
                    continue;
                }
                // A recalled release is not on offer. It is dropped rather
                // than shown refused: this index is what a reader picks a part
                // *from*, and a row nobody may act on is a row that costs
                // every reader a decision they cannot make. Where a recall
                // does have to be read — a release already installed — the
                // ledger says so on the pack's own line.
                if recalled.reason(&listed.id, &release.version).is_some() {
                    continue;
                }
                let missing = missing_capabilities(&release.capabilities);
                let state = if missing.is_empty() {
                    PartState::Available
                } else {
                    PartState::Incompatible { missing }
                };
                for part in &release.parts {
                    rows.push(ModelHubPartRow {
                        part_id: part.id.clone(),
                        kind: part.kind,
                        device: part.device.clone(),
                        terminals: part.terminals.clone(),
                        provenance: PartProvenance::RemoteRelease {
                            pack_id: listed.id.clone(),
                            version: release.version.clone(),
                        },
                        state: state.clone(),
                        pack_name: Some(listed.name.clone()),
                        source: None,
                    });
                }
            }
        }
    }

    rows.sort_by(|left, right| {
        left.part_id
            .cmp(&right.part_id)
            .then_with(|| format!("{:?}", left.provenance).cmp(&format!("{:?}", right.provenance)))
    });
    rows
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn semantic_precedence_ranks_releases_above_their_pre_releases() {
        assert_eq!(precedence("1.2.0", "1.1.9"), std::cmp::Ordering::Greater);
        assert_eq!(precedence("1.2.0", "1.2.0"), std::cmp::Ordering::Equal);
        assert_eq!(
            precedence("1.2.0", "1.2.0-rc.1"),
            std::cmp::Ordering::Greater
        );
        assert_eq!(
            precedence("1.2.0-rc.1", "1.2.0-rc.2"),
            std::cmp::Ordering::Less
        );
        assert_eq!(precedence("2.0.0", "10.0.0"), std::cmp::Ordering::Less);
    }

    #[test]
    fn an_unknown_capability_is_never_treated_as_satisfied() {
        assert!(missing_capabilities(&["subckt".to_owned()]).is_empty());
        assert_eq!(
            missing_capabilities(&["subckt".to_owned(), "nonexistent-capability".to_owned()]),
            vec!["nonexistent-capability".to_owned()]
        );
    }
}
