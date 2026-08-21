//! The ledger of distributed model packs, and the shipped corpus beside it.
//!
//! Two scopes, two questions. The ledger answers "what do this machine and this
//! project hold, and what needs me"; the shelf beside it — `shelf.rs` — answers
//! "which part do I want". Everything else the hub knows about itself, its
//! signing key, its acceptance contract and the last thing it tried, is
//! reference material and lives behind one dialog, because a page that states
//! its contracts on every paint teaches nothing twice and costs the reader
//! every time.
//!
//! # The healthy state is silent
//!
//! One quiet status line, and no banner, no meter and no card. A cell, chip or
//! banner exists here only while the fact it carries would change what the
//! reader does next — which is why most rows leave their Attention cell blank
//! and why an installed release that re-proved says nothing at all in the
//! table. Its verdict is in the inspector, where somebody asking about that one
//! pack will find it.
//!
//! # One row per pack, not one per release
//!
//! A pack's release history is history: a reader deciding anything needs the
//! release they hold, the release on offer, and what this project committed to
//! — three facts about one pack, on one line. The full list of published
//! releases is a pane in the inspector, where the pack it belongs to is already
//! selected. Listing every release in the table put ten rows of history in
//! front of every reader to carry one row of news.
//!
//! The corpus table below the ledger is the pre-distribution mechanism — a
//! versioned tree on disk rather than a signed release — and renders only when
//! such a tree is actually installed, because merging the two would mean a row
//! that cannot honestly say where it came from.

use super::*;

use std::cmp::Ordering;

use rspice_pack::SnapshotPack;

use crate::services::model_hub::ModelHubService;
use crate::state::model_hub::{ArchiveEvidence, ReleaseDiff, missing_capabilities, precedence};
use crate::state::model_library::PackPartPin;
use crate::workbench::app::ModelHubRequest;
use crate::workbench::state::{ModelHubFacet, PackReProof, PackReleaseConfirmation};

/// One published release, as the inspector's Releases pane lists it.
#[derive(Debug, Clone, PartialEq, Eq)]
pub(super) struct HubPackRow {
    pub pack_id: String,
    pub name: String,
    pub category: String,
    pub version: String,
    pub state: HubPackState,
    pub spdx: String,
    pub archive_length: u64,
    pub parts: usize,
    pub capabilities: Vec<String>,
    /// What the startup sweep concluded about the archive on this machine.
    /// `None` for a release nothing here holds.
    pub archive: Option<ArchiveEvidence>,
}

/// How a re-proof verdict is filed, spelled in one place.
///
/// `pack_verification` is keyed by release rather than by pack, because a
/// verdict is about the bytes of one release; the ledger looks up the release
/// it holds. Two spellings of this key would mean a pack that had just been
/// re-proved still reporting that nothing had.
fn release_key(pack_id: &str, version: &str) -> String {
    format!("{pack_id}@{version}")
}

/// One pack, as the ledger lists it.
///
/// Everything on the line is about the pack rather than about a release of it:
/// which release this machine holds, what this project adopted, and the single
/// exception — if any — that needs a decision.
#[derive(Debug, Clone, PartialEq, Eq)]
pub(super) struct HubLedgerRow {
    pub pack_id: String,
    /// What the pack contains, in the catalog's own words.
    pub name: String,
    pub category: String,
    /// Parts in the release a reader would act on: the held one when this
    /// machine holds one, the newest offered otherwise.
    pub parts: usize,
    /// The release on this machine, and what its bytes proved to be.
    pub installed: Option<InstalledRelease>,
    /// What this project committed to from this pack.
    pub adoption: PackAdoption,
    /// The newest published release, when it supersedes the held one.
    pub update: Option<String>,
    /// Capabilities the newest offered release needs and this build lacks.
    pub missing: Vec<String>,
    /// Every published release, newest first.
    pub releases: Vec<HubPackRow>,
    /// Every part of this pack the project pinned, as the pins record them.
    ///
    /// `adoption` is the summary a cell can carry — how many, at which release,
    /// and whether it still holds. These are the individual commitments, which
    /// is what the inspector needs to offer one decision per part. They are
    /// carried on the row rather than looked up again because the pass that
    /// decides adoption already has them in hand.
    pub pins: Vec<PackPartPin>,
}

/// The release this machine holds, and the evidence about its bytes.
#[derive(Debug, Clone, PartialEq, Eq)]
pub(super) struct InstalledRelease {
    pub version: String,
    /// Whether the retained archive still hashes to the published digest.
    pub archive: Option<ArchiveEvidence>,
    /// The digest the retained archive actually has, which is what a project
    /// pin recorded and what it is compared against.
    pub archive_sha256: String,
}

/// What this project committed to from one pack.
///
/// A pin is attribution rather than a dependency: adding a part retains its
/// source bytes into the project, so a design still builds after the pack is
/// removed. What the pin buys is the ability to say *which release* those
/// bytes came from — which is why a pin whose release is gone is worth
/// reporting and is not an emergency.
#[derive(Debug, Clone, PartialEq, Eq)]
pub(super) enum PackAdoption {
    /// No library in this project came from this pack.
    None,
    Pinned {
        version: String,
        /// Libraries in this project pinned to that release.
        parts: usize,
        health: PinHealth,
    },
}

/// Whether what the pin names is still what is on this machine.
#[derive(Debug, Clone, PartialEq, Eq)]
pub(super) enum PinHealth {
    /// The pinned release is installed, at the archive the pin recorded.
    Matching,
    /// Installed, but at a different release than the pin names.
    Differs { installed: String },
    /// Installed at the pinned version, whose archive is no longer the one the
    /// pin recorded — so the pin names bytes this machine no longer has.
    ArchiveReplaced,
    /// The pinned release is not on this machine. The retained sources still
    /// execute; nothing here can re-prove them against the release.
    Absent,
}

/// The one thing about a pack that needs a decision, and how loudly to say it.
#[derive(Debug, Clone, PartialEq, Eq)]
pub(super) struct Attention {
    pub phrase: String,
    pub tone: AttentionTone,
    /// The same fact with its consequence attached, for the row's hover and
    /// for what a screen reader is told about the row.
    pub detail: String,
}

/// How loudly one attention phrase is painted.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(super) enum AttentionTone {
    Error,
    Warn,
}

/// The one exception a pack row is allowed to shout, in the order a reader
/// needs it.
///
/// Bytes that are not what was signed outrank a pin that names bytes this
/// machine no longer has, which outranks a re-proof that failed, which
/// outranks a release this build cannot run, which outranks an offer, which
/// outranks stale evidence. A pack with none of these has nothing to say and
/// says nothing — including a pack that re-proved cleanly, whose verdict is
/// reported in the inspector rather than shouted on every row.
fn pack_attention(row: &HubLedgerRow, proof: Option<&PackReProof>) -> Option<Attention> {
    let error = |phrase: String, detail: String| {
        Some(Attention {
            phrase,
            tone: AttentionTone::Error,
            detail,
        })
    };
    let warn = |phrase: String, detail: String| {
        Some(Attention {
            phrase,
            tone: AttentionTone::Warn,
            detail,
        })
    };
    if let Some(installed) = row.installed.as_ref() {
        if installed.archive == Some(ArchiveEvidence::DiffersFromCatalog) {
            return error(
                "archive differs".to_owned(),
                format!(
                    "The archive retained for {} no longer hashes to the digest the signed \
                     catalog publishes, so these are not the bytes this release was proved as.",
                    installed.version
                ),
            );
        }
        if matches!(
            row.adoption,
            PackAdoption::Pinned {
                health: PinHealth::ArchiveReplaced,
                ..
            }
        ) {
            return error(
                "pinned archive replaced".to_owned(),
                format!(
                    "This project records parts from {} against one archive digest, and the copy \
                     of {} on this machine has another. The retained sources still execute; what \
                     they can no longer be attributed to is the release on this machine.",
                    installed.version, installed.version
                ),
            );
        }
        if let Some(PackReProof::Failed(reason)) = proof {
            return warn(
                "re-proof failed".to_owned(),
                format!(
                    "Re-proving {} under the release key refused: {reason}. Nothing on this \
                     machine changed.",
                    installed.version
                ),
            );
        }
    }
    if !row.missing.is_empty() {
        return error(
            format!("needs {}", row.missing.join(" · ")),
            format!(
                "The newest release requires {}, which this build does not provide. Installing it \
                 would put definitions in the closure that no analysis here can evaluate.",
                plain_list(&row.missing)
            ),
        );
    }
    if let Some(update) = row.update.as_deref() {
        return warn(
            format!("update {update}"),
            match row.installed.as_ref() {
                Some(installed) => format!(
                    "{update} is offered against the installed {}. Updates are notified, never \
                     applied.",
                    installed.version
                ),
                None => format!("{update} is the release on offer."),
            },
        );
    }
    if let Some(installed) = row.installed.as_ref()
        && proof.is_none()
    {
        return warn(
            "never re-proved".to_owned(),
            format!(
                "Nothing has re-proved {} under the release key since this session started. \
                 Verifying reads the retained archive and changes nothing.",
                installed.version
            ),
        );
    }
    if row.installed.is_none()
        && let PackAdoption::Pinned { version, .. } = &row.adoption
    {
        return warn(
            "pin not installed".to_owned(),
            format!(
                "This project records parts from {version}, and that release is not on this \
                 machine. The retained sources still execute; nothing here can re-prove them \
                 against the release they came from.",
            ),
        );
    }
    None
}

/// What can be done with one published release right now.
#[derive(Debug, Clone, PartialEq, Eq)]
pub(super) enum HubPackState {
    Installed,
    Available,
    /// A newer release than the one installed. Both versions are named because
    /// the action is "replace this with that", not "get the latest".
    UpdateAvailable {
        installed: String,
    },
    /// This engine build does not offer what the release requires.
    Incompatible {
        missing: Vec<String>,
    },
}

impl HubPackState {
    pub(super) const fn pill(&self) -> &'static str {
        match self {
            Self::Installed => "installed",
            Self::Available => "available",
            Self::UpdateAvailable { .. } => "update",
            Self::Incompatible { .. } => "incompatible",
        }
    }
}

/// Which packs one facet admits.
///
/// `Installed` and `Available` are complements over the same list, so between
/// them they account for every pack exactly once. `Needs attention` and
/// `Pinned` are questions asked *across* that split and overlap it on purpose:
/// a pinned pack is installed or it is not, and saying so is the point of the
/// chip. The partition test in this module states both claims, because a chip
/// whose count does not describe the rows the table then shows is precisely
/// the defect one shared predicate exists to prevent.
fn ledger_matches(row: &HubLedgerRow, attention: Option<&Attention>, facet: ModelHubFacet) -> bool {
    match facet {
        ModelHubFacet::All => true,
        ModelHubFacet::NeedsAttention => attention.is_some(),
        ModelHubFacet::Installed => row.installed.is_some(),
        ModelHubFacet::Pinned => matches!(row.adoption, PackAdoption::Pinned { .. }),
        ModelHubFacet::Available => row.installed.is_none(),
    }
}

/// Everything the ledger reads about distributed packs in one frame.
///
/// It is built once per frame from the session hub and the project's own
/// libraries rather than queried per row. The projection is small, and
/// computing it in one place is what keeps the facet chips, the table, the
/// inspector and the status line from disagreeing about which release is
/// current.
#[derive(Debug, Default, Clone)]
pub(super) struct HubCatalog {
    pub packs: Vec<HubLedgerRow>,
    /// Whole days since the cached catalog was signed.
    pub age_days: Option<u64>,
    /// The day the held catalog was signed, as the signature carries it.
    pub signed: Option<String>,
    /// Why there is no hub at all, in words a user can act on.
    pub unavailable: Option<String>,
    pub stale: bool,
    /// Whether the catalog this hub cached failed verification and was
    /// discarded. Absent evidence and rejected evidence are different answers.
    pub cache_discarded: bool,
    /// What the held catalog is: digest, schema, signing instant, generation.
    pub identity: Option<crate::state::model_hub::CatalogIdentity>,
    /// The verifying key every verdict here was reached under.
    pub signing_key: String,
    /// Distinct licence identifiers the catalog publishes.
    pub licences: Vec<String>,
}

impl HubCatalog {
    /// Total parts across every listed pack, and how many are on this machine.
    fn part_totals(&self) -> (usize, usize) {
        let held = self
            .packs
            .iter()
            .filter(|pack| pack.installed.is_some())
            .map(|pack| pack.parts)
            .sum();
        (self.packs.iter().map(|pack| pack.parts).sum(), held)
    }
}

/// Projects the session hub and the project's pins into what the ledger lists.
pub(super) fn hub_catalog(service: &ModelHubService, state: &AppState) -> HubCatalog {
    let mut catalog = HubCatalog {
        age_days: service.catalog_age_days(),
        unavailable: service.unavailable_reason().map(str::to_owned),
        stale: service.catalog_is_stale(),
        cache_discarded: service.catalog_cache_discarded(),
        ..HubCatalog::default()
    };
    let Some(hub) = service.hub() else {
        return catalog;
    };
    catalog.identity = hub.catalog_identity().cloned();
    catalog.signed = catalog
        .identity
        .as_ref()
        .map(|identity| identity.generated_at.chars().take(10).collect());
    catalog.signing_key = rspice_pack::encode_hex(hub.anchor().key().as_bytes());
    let pins = project_pins(state);
    let installed = hub.installed();
    let mut releases: BTreeMap<String, Vec<HubPackRow>> = BTreeMap::new();
    if let Some(snapshot) = hub.snapshot() {
        for pack in &snapshot.packs {
            let held = installed
                .iter()
                .find(|candidate| candidate.pack_id() == pack.id)
                .map(|candidate| candidate.version());
            releases.entry(pack.id.clone()).or_default().extend(
                pack_rows(pack, held)
                    .into_iter()
                    .map(|mut row: HubPackRow| {
                        row.archive = hub.archive_evidence(&row.pack_id, &row.version);
                        row
                    }),
            );
        }
    }
    // A release installed from a catalog that has since dropped it is still on
    // this machine and still usable, so it is listed from its own manifest
    // rather than vanishing when the catalog forgets it.
    for pack in installed {
        let listed = releases.entry(pack.pack_id().to_owned()).or_default();
        if listed.iter().any(|row| row.version == pack.version()) {
            continue;
        }
        listed.push(HubPackRow {
            pack_id: pack.pack_id().to_owned(),
            name: pack.manifest.pack.name.clone(),
            category: pack.manifest.pack.category.clone(),
            version: pack.version().to_owned(),
            state: HubPackState::Installed,
            spdx: pack.manifest.license.spdx.clone(),
            archive_length: 0,
            parts: pack.manifest.parts.len(),
            capabilities: pack.manifest.requires.capabilities.clone(),
            archive: hub.archive_evidence(pack.pack_id(), pack.version()),
        });
    }
    let mut licences = BTreeSet::new();
    for (pack_id, mut rows) in releases {
        rows.sort_by(newest_release_first);
        licences.extend(rows.iter().map(|row| row.spdx.clone()));
        let installed = installed
            .iter()
            .find(|candidate| candidate.pack_id() == pack_id)
            .map(|candidate| InstalledRelease {
                version: candidate.version().to_owned(),
                archive: hub.archive_evidence(candidate.pack_id(), candidate.version()),
                archive_sha256: candidate.archive_sha256.clone(),
            });
        let pinned = pins.get(&pack_id).map_or(&[][..], Vec::as_slice);
        catalog
            .packs
            .push(ledger_row(&pack_id, rows, installed, pinned));
    }
    catalog.licences = licences.into_iter().collect();
    catalog
        .packs
        .sort_by(|left, right| left.name.cmp(&right.name));
    catalog
}

/// Every pack pin this project records, gathered in one pass over its
/// libraries.
///
/// One pass rather than one per pack: the ledger asks this question of every
/// row, and a project at the top of its range binds dozens of libraries.
fn project_pins(state: &AppState) -> BTreeMap<String, Vec<PackPartPin>> {
    let mut pins: BTreeMap<String, Vec<PackPartPin>> = BTreeMap::new();
    for library in state.model_library_manager.libraries_sorted() {
        if let Some(pin) = library.pack_pin.as_ref() {
            pins.entry(pin.pack_id.clone())
                .or_default()
                .push(pin.clone());
        }
    }
    pins
}

/// One ledger line, assembled from the releases, the machine and the project.
pub(super) fn ledger_row(
    pack_id: &str,
    releases: Vec<HubPackRow>,
    installed: Option<InstalledRelease>,
    pins: &[PackPartPin],
) -> HubLedgerRow {
    let newest = releases.first();
    // An update is a release that supersedes one this machine holds. A pack
    // nobody installed has nothing to update and nothing to decide, so its
    // Attention cell stays blank and the offer lives where it belongs — on the
    // Install control, which names the version it would fetch.
    let update = releases.iter().find_map(|row| match &row.state {
        HubPackState::UpdateAvailable { .. } => Some(row.version.clone()),
        _ => None,
    });
    let missing = newest
        .and_then(|row| match &row.state {
            HubPackState::Incompatible { missing } => Some(missing.clone()),
            _ => None,
        })
        .unwrap_or_default();
    // The release a reader would act on decides the part count: what is held,
    // or what is offered. A pack's whole publication history has no single
    // part count, and summing one would describe no release at all.
    let acted_on = installed
        .as_ref()
        .and_then(|held| releases.iter().find(|row| row.version == held.version))
        .or(newest);
    HubLedgerRow {
        pack_id: pack_id.to_owned(),
        name: acted_on.map(|row| row.name.clone()).unwrap_or_default(),
        category: acted_on.map(|row| row.category.clone()).unwrap_or_default(),
        parts: acted_on.map_or(0, |row| row.parts),
        adoption: pack_adoption(pins, installed.as_ref()),
        installed,
        update,
        missing,
        releases,
        pins: pins.to_vec(),
    }
}

/// Everything the packs page settles before the render borrow, and the one
/// request that can come out of it.
///
/// Both halves need the session hub, which the render context deliberately
/// does not carry. A cached catalog old enough that showing it without
/// checking would report last week's answer to this week's question asks for a
/// refresh — once per session, which is what the latch is for. The selected
/// pack's "what changed" answer is settled here too, and only when the
/// catalog, the pack or either release actually moved.
pub(super) fn prepare(
    service: &ModelHubService,
    state: &mut AppState,
    hub: &HubCatalog,
) -> Option<ModelHubRequest> {
    super::adoption::refresh_release_diff(service, state, hub);
    let view = &mut state.workbench.models_view;
    if hub.unavailable.is_some()
        || !hub.stale
        || view.model_import_in_progress
        || view.catalog_refresh_requested
    {
        return None;
    }
    view.catalog_refresh_requested = true;
    Some(ModelHubRequest::FetchSnapshot)
}

/// The pack the ledger has selected, or the first one it lists.
///
/// One resolution, in one place. The inspector and the "what changed"
/// projection computed for it must be about the same pack, and two spellings
/// of "which one is selected" would eventually let a reader read one pack's
/// releases beside another pack's diff.
pub(super) fn selected_row<'a>(hub: &'a HubCatalog, state: &AppState) -> Option<&'a HubLedgerRow> {
    state
        .workbench
        .models_view
        .selected_pack
        .as_deref()
        .and_then(|id| hub.packs.iter().find(|row| row.pack_id == id))
        .or_else(|| hub.packs.first())
}

/// What this project committed to from one pack, and whether it still holds.
fn pack_adoption(pins: &[PackPartPin], installed: Option<&InstalledRelease>) -> PackAdoption {
    // A project can hold parts from more than one release of a pack. The
    // newest pinned release is the one the cell names, and the count is every
    // pinned part, so the cell can never claim fewer parts than the project
    // has.
    let Some(newest) = pins
        .iter()
        .max_by(|left, right| precedence(&left.pack_version, &right.pack_version))
    else {
        return PackAdoption::None;
    };
    let health = match installed {
        None => PinHealth::Absent,
        Some(held) if held.version != newest.pack_version => PinHealth::Differs {
            installed: held.version.clone(),
        },
        Some(held) if held.archive_sha256 != newest.archive_sha256 => PinHealth::ArchiveReplaced,
        Some(_) => PinHealth::Matching,
    };
    PackAdoption::Pinned {
        version: newest.pack_version.clone(),
        parts: pins.len(),
        health,
    }
}

/// Every release one listed pack publishes, in the state this machine puts it.
///
/// Split out of [`hub_catalog`] because it is the whole of the version
/// reasoning and the only part of it a test can reach without a signed store:
/// which release is newest, and whether the newest one supersedes what is held.
fn pack_rows(pack: &SnapshotPack, held: Option<&str>) -> Vec<HubPackRow> {
    let newest = pack
        .releases
        .iter()
        .map(|release| release.version.as_str())
        .max_by(|left, right| precedence(left, right));
    pack.releases
        .iter()
        .map(|release| {
            let missing = missing_capabilities(&release.capabilities);
            let state = if !missing.is_empty() {
                HubPackState::Incompatible { missing }
            } else if held == Some(release.version.as_str()) {
                HubPackState::Installed
            } else if newest == Some(release.version.as_str())
                && held.is_some_and(|held| precedence(&release.version, held) == Ordering::Greater)
            {
                // Only the newest listed release offers an update. An older
                // one is history, and offering it would make every
                // superseded version look like an action.
                HubPackState::UpdateAvailable {
                    installed: held.unwrap_or_default().to_owned(),
                }
            } else {
                HubPackState::Available
            };
            HubPackRow {
                pack_id: pack.id.clone(),
                name: pack.name.clone(),
                category: pack.category.clone(),
                version: release.version.clone(),
                state,
                spdx: release.spdx.clone(),
                archive_length: release.archive_length,
                parts: release.parts.len(),
                capabilities: release.capabilities.clone(),
                archive: None,
            }
        })
        .collect()
}

/// Packs by name, and each pack's releases newest first.
///
/// The version half is semantic precedence rather than a byte comparison, so
/// the row the table puts at the top of a pack is the same release
/// [`pack_rows`] calls newest. Two orderings would put `9.0.0` above `10.0.0`
/// in one place and below it in the other.
fn newest_release_first(left: &HubPackRow, right: &HubPackRow) -> Ordering {
    left.name
        .cmp(&right.name)
        .then_with(|| precedence(&right.version, &left.version))
}

/// A byte count a person reads rather than counts.
pub(super) fn byte_size(bytes: u64) -> String {
    const UNITS: [(&str, u64); 3] = [("MB", 1024 * 1024), ("kB", 1024), ("B", 1)];
    if bytes == 0 {
        return "—".to_owned();
    }
    for (unit, scale) in UNITS {
        if bytes >= scale {
            let value = bytes as f64 / scale as f64;
            return if value >= 100.0 || scale == 1 {
                format!("{value:.0} {unit}")
            } else {
                format!("{value:.1} {unit}")
            };
        }
    }
    "—".to_owned()
}

/// The packs scope: the pack ledger, then the shipped corpus if present.
pub(super) fn packs_page(ui: &mut Ui, app: &mut ManagerRenderContext<'_>, hub: &HubCatalog) {
    catalog_status(ui, app, hub);
    exception_banner(ui, app);
    ledger(ui, app, hub);
    let packs = app
        .state
        .model_library_manager
        .spice_packs()
        .map(|index| index.packs().to_vec())
        .unwrap_or_default();
    if !packs.is_empty() {
        super::corpus::pack_catalog(ui, app, &packs);
    }
}

/// What the held catalog is, in one quiet phrase.
///
/// The healthy answer is one sentence and no adjectives: signed on a day, at a
/// generation, and verified. "Never fetched" is a real state and says so — a
/// client that has not asked yet knows nothing about what is published, which
/// is different from knowing that nothing is. A cached catalog that failed
/// verification is a third state again, because this client *did* ask and what
/// came back no longer proves; reporting that as the first told a machine with
/// a corrupted or substituted cache that it had simply never looked.
///
/// The generation clause is omitted rather than guessed when the held
/// snapshot came from the on-disk cache: the generation is a service handoff
/// field that never travelled inside the signed bytes, so this session has no
/// way to know it without asking again.
pub(super) fn catalog_summary(
    signed: Option<&str>,
    age_days: Option<u64>,
    generation: Option<u64>,
    cache_discarded: bool,
) -> String {
    let Some(signed) = signed else {
        return if cache_discarded {
            "The cached catalog failed verification and was discarded — refresh".to_owned()
        } else {
            "No catalog fetched".to_owned()
        };
    };
    let mut summary = format!("Catalog signed {signed}");
    if let Some(generation) = generation {
        summary.push_str(&format!(" · generation {generation}"));
    }
    summary.push_str(" · verified");
    // Age is the one thing worth adding, and only once the catalog has stopped
    // being current by the same threshold the workspace refreshes on. A
    // catalog signed on Tuesday needs no adverb on Thursday, and printing one
    // every day would train a reader to read past the line that matters.
    if age_days.is_some_and(|days| days >= crate::services::model_hub::CATALOG_STALE_AFTER_DAYS) {
        summary.push_str(&format!(" · {} days old", age_days.unwrap_or_default()));
    }
    summary
}

/// One line saying what the catalog is, and the two things to do about it.
///
/// It is a line rather than a card because it answers one question and is
/// wrong the moment it is acted on. A hub that could not open says so here
/// instead, since "signed on the 14th" would be a claim about nothing.
fn catalog_status(ui: &mut Ui, app: &mut ManagerRenderContext<'_>, hub: &HubCatalog) {
    let t = Tokens::get(ui.ctx());
    egui::Frame::NONE
        .inner_margin(egui::Margin::symmetric(12, 4))
        .show(ui, |ui| {
            ui.set_min_width(ui.available_width().max(1.0));
            ui.horizontal(|ui| {
                // The workspace paints itself as one continuous document and
                // zeroes item spacing to do it; a line of separate phrases has
                // to ask for its own gaps back or it reads as one run-on word.
                ui.spacing_mut().item_spacing.x = 10.0;
                if let Some(reason) = hub.unavailable.as_deref() {
                    announced(ui, RichText::new(reason).small().color(t.color.err), reason);
                    return;
                }
                let summary = catalog_summary(
                    hub.signed.as_deref(),
                    hub.age_days,
                    hub.identity
                        .as_ref()
                        .and_then(|identity| identity.generation),
                    hub.cache_discarded,
                );
                announced(
                    ui,
                    RichText::new(&summary)
                        .small()
                        .color(if hub.cache_discarded {
                            t.color.err
                        } else if hub.stale {
                            t.color.warn
                        } else {
                            t.color.text_dim
                        }),
                    &summary,
                );
                let (parts, held) = hub.part_totals();
                if parts > 0 {
                    let contents =
                        format!("{parts} parts · {} packs · {held} here", hub.packs.len());
                    ui.label(
                        RichText::new(&contents)
                            .small()
                            .monospace()
                            .color(t.color.text_faint),
                    );
                }
                ui.with_layout(Layout::right_to_left(Align::Center), |ui| {
                    let idle = !app.state.workbench.models_view.model_import_in_progress;
                    if ui
                        .add_enabled(idle, compact_button("Refresh catalog"))
                        .on_disabled_hover_text("Another model-source operation is still running.")
                        .clicked()
                    {
                        app.queue_model_hub(ModelHubRequest::FetchSnapshot);
                    }
                    if ui
                        .add(compact_button("Catalog details…"))
                        .on_hover_text(
                            "Who signed what this client holds, the contract it is accepted \
                             under, and the last thing this session tried.",
                        )
                        .clicked()
                    {
                        app.state.workbench.models_view.dialog =
                            Some(ModelsWorkbenchDialog::HeldCatalog);
                    }
                    if let Some(fraction) = app.state.workbench.models_view.model_import_progress {
                        ui.add(
                            egui::ProgressBar::new(fraction)
                                .desired_width(120.0)
                                .show_percentage(),
                        );
                    }
                });
            });
        });
}

/// One line of banner prose that a screen reader can actually reach.
///
/// A plain `ui.label` in this workspace publishes no accessibility node, which
/// would leave an error banner unreadable by exactly the reader who most needs
/// it read aloud. Every line of the banner goes through here instead.
pub(super) fn announced(ui: &mut Ui, text: RichText, label: &str) {
    announced_widget(ui, egui::Label::new(text), label);
}

/// The same, for a line that has to be told how to behave when it runs out of
/// room — truncating rather than pushing the control beside it off the pane.
///
/// What truncates is the painted glyphs. The announced label is the whole
/// sentence either way, which is the point of there being one door.
pub(super) fn announced_widget(ui: &mut Ui, widget: egui::Label, label: &str) {
    let response = ui.add(widget.sense(Sense::hover()));
    response
        .widget_info(|| egui::WidgetInfo::labeled(egui::WidgetType::Label, ui.is_enabled(), label));
    // `widget_info` alone publishes nothing for a non-interactive widget, so
    // the node is declared outright — the same route the preflight report's
    // panel headings take.
    ui.ctx().accesskit_node_builder(response.id, |node| {
        node.set_role(egui::accesskit::Role::Label);
        node.set_label(label);
    });
}

/// The one banner this page is allowed to raise.
///
/// It renders only while the last model-source or Model Hub operation ended in
/// a refusal, which is what makes the healthy page silent. Four things in one
/// breath, because a refusal split across a toast, a console line and a
/// disabled button is four places to look and no statement anywhere: what was
/// attempted, what refused it, what that left behind, and how to run it again.
///
/// The retry control is offered only for an operation the workspace can
/// re-issue from the receipt alone. An install is retried from the release row
/// that named the version, so a bare "retry" here would be a button that
/// guesses which release the reader meant.
fn exception_banner(ui: &mut Ui, app: &mut ManagerRenderContext<'_>) {
    let Some(Err(reason)) = app.state.workbench.models_view.action_receipt.clone() else {
        return;
    };
    let state = app.state.workbench.models_view.operational_state;
    let attempted = app
        .state
        .workbench
        .models_view
        .attempted_operation
        .clone()
        .unwrap_or(crate::workbench::state::ModelsAttemptedOperation {
            label: "model-source operation".to_owned(),
            reissuable: false,
        });
    let t = Tokens::get(ui.ctx());
    egui::Frame::NONE
        .fill(t.color.bg_inset)
        .stroke(Stroke::new(1.0, t.color.err))
        .inner_margin(egui::Margin::symmetric(12, 7))
        .show(ui, |ui| {
            ui.set_min_width(ui.available_width().max(1.0));
            ui.spacing_mut().item_spacing.y = 3.0;
            ui.horizontal(|ui| {
                ui.spacing_mut().item_spacing.x = 8.0;
                announced(
                    ui,
                    RichText::new(state.label())
                        .small()
                        .strong()
                        .color(t.color.err),
                    state.label(),
                );
                announced(
                    ui,
                    RichText::new(&attempted.label)
                        .small()
                        .monospace()
                        .color(t.color.text_dim),
                    &attempted.label,
                );
            });
            announced(
                ui,
                RichText::new(&reason).small().color(t.color.text),
                &reason,
            );
            announced(
                ui,
                RichText::new(state.consequence())
                    .small()
                    .color(t.color.text_dim),
                state.consequence(),
            );
            if attempted.reissuable
                && ui
                    .add_enabled(
                        !app.state.workbench.models_view.model_import_in_progress,
                        compact_button("Retry the catalog refresh"),
                    )
                    .on_disabled_hover_text("Another model-source operation is still running.")
                    .clicked()
            {
                app.queue_model_hub(ModelHubRequest::FetchSnapshot);
            }
        });
}

/// The ledger's columns, named once so the header and the rows cannot drift
/// apart and so the attention cell knows where it starts.
///
/// The proportions are the mockup's: identity narrow, contents wide enough to
/// read, and the two columns that carry a decision — Project and Attention —
/// given equal, generous width, because a truncated exception is a worse
/// answer than none.
const LEDGER_COLUMNS: [(&str, f32); 6] = [
    ("PACK", 0.16),
    ("CONTENTS", 0.30),
    ("PARTS", 0.07),
    ("INSTALLED", 0.11),
    ("PROJECT", 0.18),
    ("ATTENTION", 0.18),
];

/// Paints a toned cell the shared row painter cannot tone.
///
/// Every other cell in this table is a fact; these two carry urgency and
/// commitment, and a warning painted in the same dim grey as a pack identifier
/// is a warning nobody sees.
fn paint_toned_cell(ui: &Ui, rect: egui::Rect, column: usize, phrase: &str, color: Color32) {
    let start: f32 = LEDGER_COLUMNS[..column]
        .iter()
        .map(|(_, width)| width)
        .sum();
    let x = rect.left() + rect.width() * start + 5.0;
    let width = rect.width() * LEDGER_COLUMNS[column].1 - 9.0;
    ui.painter().text(
        egui::pos2(x, rect.center().y),
        egui::Align2::LEFT_CENTER,
        elide(ui, phrase, width, true),
        theme::mono(tokens::FS_0, FontWeight::Regular),
        color,
    );
}

/// What the Project cell says, and whether it says it quietly.
///
/// A pin whose release is installed and whose archive is the pinned one is the
/// healthy case and states only the commitment. Everything else appends the one
/// word that names how it diverged.
fn project_cell(adoption: &PackAdoption) -> Option<(String, bool)> {
    let PackAdoption::Pinned {
        version,
        parts,
        health,
    } = adoption
    else {
        return None;
    };
    let label = format!(
        "{parts} part{} @ {version}",
        if *parts == 1 { "" } else { "s" }
    );
    Some(match health {
        PinHealth::Matching => (label, false),
        PinHealth::Differs { installed } => (format!("{label} · differs from {installed}"), true),
        PinHealth::ArchiveReplaced => (format!("{label} · archive replaced"), true),
        PinHealth::Absent => (format!("{label} · not installed"), true),
    })
}

/// The pack ledger and the inspector under it.
fn ledger(ui: &mut Ui, app: &mut ManagerRenderContext<'_>, hub: &HubCatalog) {
    if hub.unavailable.is_some() {
        page_empty_state(
            ui,
            "Distributed model packs are unavailable",
            "RSpice could not open its pack store, so nothing can be installed on this machine. \
             Projects that already retained a part keep working.",
        );
        return;
    }
    if hub.packs.is_empty() {
        page_empty_state(
            ui,
            "No published model pack has been fetched",
            "Refresh the catalog to list the signed packs this build can install.",
        );
        return;
    }
    let facet = app.state.workbench.models_view.hub_facet;
    let query = app
        .state
        .workbench
        .models_view
        .catalog_query
        .trim()
        .to_ascii_lowercase();
    let visible = hub
        .packs
        .iter()
        .map(|row| {
            let attention = pack_attention(row, verification_of(app, row));
            (row, attention)
        })
        .filter(|(row, attention)| {
            ledger_matches(row, attention.as_ref(), facet)
                && (query.is_empty()
                    || format!("{} {} {}", row.pack_id, row.name, row.category)
                        .to_ascii_lowercase()
                        .contains(&query))
        })
        .collect::<Vec<_>>();
    let table_h = (ui.available_height() * 0.34).max(110.0);
    let t = Tokens::get(ui.ctx());
    egui::Frame::NONE.fill(t.color.bg_panel).show(ui, |ui| {
        table_header(ui, &LEDGER_COLUMNS);
        ScrollArea::vertical()
            .id_salt("models-hub-pack-table")
            .max_height(table_h)
            .show(ui, |ui| {
                if visible.is_empty() {
                    empty_state(
                        ui,
                        "No pack matches this facet.",
                        "Facets derive from the signed catalog, what this machine holds, and \
                         what this project pinned.",
                    );
                    if ui.button("Clear release filter").clicked() {
                        app.state.workbench.models_view.hub_facet = ModelHubFacet::All;
                        app.state.workbench.models_view.catalog_query.clear();
                    }
                }
                for (row, attention) in &visible {
                    ledger_line(ui, app, row, attention.as_ref());
                }
            });
    });
    catalog_footer(
        ui,
        visible.len(),
        hub.packs.len(),
        visible
            .iter()
            .filter(|(_, attention)| attention.is_some())
            .count(),
        "packs",
    );
    inspector(ui, app, hub);
}

/// The re-proof verdict recorded for a pack's installed release, if any.
fn verification_of<'a>(
    app: &'a ManagerRenderContext<'_>,
    row: &HubLedgerRow,
) -> Option<&'a PackReProof> {
    let installed = row.installed.as_ref()?;
    app.state
        .workbench
        .models_view
        .pack_verification
        .get(&release_key(&row.pack_id, &installed.version))
}

/// One ledger line: six cells, of which at most one is loud.
fn ledger_line(
    ui: &mut Ui,
    app: &mut ManagerRenderContext<'_>,
    row: &HubLedgerRow,
    attention: Option<&Attention>,
) {
    let t = Tokens::get(ui.ctx());
    let selected =
        app.state.workbench.models_view.selected_pack.as_deref() == Some(row.pack_id.as_str());
    let project = project_cell(&row.adoption);
    let response = selectable_data_row(
        ui,
        selected,
        &[
            (&row.pack_id, LEDGER_COLUMNS[0].1, true),
            (&row.name, LEDGER_COLUMNS[1].1, false),
            (&row.parts.to_string(), LEDGER_COLUMNS[2].1, true),
            (
                row.installed
                    .as_ref()
                    .map_or("", |installed| installed.version.as_str()),
                LEDGER_COLUMNS[3].1,
                true,
            ),
            // Both painted below, in the tone each one earns.
            ("", LEDGER_COLUMNS[4].1, true),
            ("", LEDGER_COLUMNS[5].1, true),
        ],
    );
    if let Some((label, diverged)) = project.as_ref() {
        paint_toned_cell(
            ui,
            response.rect,
            4,
            label,
            if *diverged {
                t.color.warn
            } else {
                t.color.text_dim
            },
        );
    }
    if let Some(attention) = attention {
        paint_toned_cell(
            ui,
            response.rect,
            5,
            &attention.phrase,
            match attention.tone {
                AttentionTone::Error => t.color.err,
                AttentionTone::Warn => t.color.warn,
            },
        );
    }
    // The row's cells are painter text and publish nothing, so the row's own
    // node carries what a screen reader has to hear: which pack, and the one
    // thing about it that needs a decision. Without this the loudest cell on
    // the page is the only one nobody can read.
    let announcement = match attention {
        Some(attention) => format!("{} · {}", row.pack_id, attention.detail),
        None => row.pack_id.clone(),
    };
    ui.ctx().accesskit_node_builder(response.id, |node| {
        node.set_label(announcement.clone());
    });
    let response = match attention {
        Some(attention) => response.on_hover_text(&attention.detail),
        None => response,
    };
    if response.clicked() {
        app.state.workbench.models_view.selected_pack = Some(row.pack_id.clone());
    }
}

/// What one selected pack is, and what can be done with it.
///
/// Compact by construction: the identity line, the releases this pack
/// publishes, and one Catalog section stating what this machine and this
/// project hold. Everything doctrinal — the signing key, the acceptance
/// contract — is behind the status line's one button rather than printed under
/// every selection. That button lives up there rather than here so it is
/// reachable in the state that most provokes the question: a page with no
/// catalog at all, which has no selection to inspect.
fn inspector(ui: &mut Ui, app: &mut ManagerRenderContext<'_>, hub: &HubCatalog) {
    let Some(row) = selected_row(hub, app.state).cloned() else {
        return;
    };
    let proof = verification_of(app, &row).cloned();
    let attention = pack_attention(&row, proof.as_ref());
    // Taken once for the frame, and only when it is about the pack on screen.
    // `refresh_release_diff` decides *whether* there is one; this only decides
    // that the one in hand describes this selection, so a stale projection
    // cannot outlive the row it was computed for by even one frame.
    let diff = app
        .state
        .workbench
        .models_view
        .release_diff
        .clone()
        .filter(|diff| diff.key.pack_id == row.pack_id);
    inspector_identity(ui, app, &row, attention.as_ref());
    // `columns` rather than a hand-measured `horizontal`: an allocated child
    // advances the cursor by the width of what it *contained*, not by the
    // track it was handed, so a short pane leaves the next one starting under
    // it and a tall one decides the row's height for both.
    ui.columns(2, |columns| {
        releases_pane(&mut columns[0], app, &row, diff.as_ref());
        catalog_pane(&mut columns[1], &row, proof.as_ref());
    });
}

/// The inspector's identity line and the actions this pack offers.
fn inspector_identity(
    ui: &mut Ui,
    app: &mut ManagerRenderContext<'_>,
    row: &HubLedgerRow,
    attention: Option<&Attention>,
) {
    let t = Tokens::get(ui.ctx());
    let idle = !app.state.workbench.models_view.model_import_in_progress;
    ui.horizontal_wrapped(|ui| {
        ui.spacing_mut().item_spacing.x = 10.0;
        ui.add_space(12.0);
        ui.label(RichText::new(&row.pack_id).monospace().strong());
        ui.label(RichText::new(&row.name).small().color(t.color.text_dim));
        if let Some(attention) = attention {
            announced(
                ui,
                RichText::new(&attention.phrase)
                    .small()
                    .monospace()
                    .color(match attention.tone {
                        AttentionTone::Error => t.color.err,
                        AttentionTone::Warn => t.color.warn,
                    }),
                &attention.detail,
            );
        }
        ui.with_layout(Layout::right_to_left(Align::Center), |ui| {
            pack_actions(ui, app, row, idle);
        });
    });
}

/// The controls one pack's state earns, and no others.
fn pack_actions(ui: &mut Ui, app: &mut ManagerRenderContext<'_>, row: &HubLedgerRow, idle: bool) {
    let offered = row
        .releases
        .iter()
        .find(|release| Some(&release.version) == row.update.as_ref());
    if let Some(installed) = row.installed.as_ref() {
        if let Some(offered) = offered
            && ui
                .add_enabled(idle, compact_button("Update"))
                .on_disabled_hover_text("Another model-source operation is still running.")
                .clicked()
        {
            app.state.workbench.models_view.dialog =
                Some(confirmation(offered, Some(installed.version.clone())));
        }
        if ui
            .add_enabled(idle, compact_button("Remove"))
            .on_disabled_hover_text("Another model-source operation is still running.")
            .clicked()
        {
            app.queue_model_hub(ModelHubRequest::RemovePack {
                pack_id: row.pack_id.clone(),
                version: installed.version.clone(),
            });
        }
        if ui
            .add_enabled(idle, compact_button("Verify installed"))
            .on_disabled_hover_text("Another model-source operation is still running.")
            .on_hover_text(
                "Reads the retained archive and re-proves it end to end under the release key. \
                 Nothing on this machine changes.",
            )
            .clicked()
        {
            app.queue_model_hub(ModelHubRequest::VerifyInstalled {
                pack_id: row.pack_id.clone(),
                version: installed.version.clone(),
            });
        }
        return;
    }
    let Some(offered) = offered.or_else(|| row.releases.first()) else {
        return;
    };
    if !row.missing.is_empty() {
        // A refusal states its own reason where the action would be, rather
        // than as a button that fails after being pressed.
        ui.add_enabled(false, compact_button("Install"))
            .on_disabled_hover_text(format!(
                "This build does not offer {}.",
                plain_list(&row.missing)
            ));
        return;
    }
    if ui
        .add_enabled(idle, compact_button("Install"))
        .on_disabled_hover_text("Another model-source operation is still running.")
        .clicked()
    {
        app.state.workbench.models_view.dialog = Some(confirmation(offered, None));
    }
}

/// Every release this pack publishes, newest first, and which one is held.
fn releases_pane(
    ui: &mut Ui,
    app: &mut ManagerRenderContext<'_>,
    row: &HubLedgerRow,
    diff: Option<&ReleaseDiff>,
) {
    let t = Tokens::get(ui.ctx());
    let published = format!("{} published", row.releases.len());
    detail_pane(ui, "RELEASES", Some(&published), |ui| {
        ScrollArea::vertical()
            .id_salt("models-hub-releases")
            .max_height(150.0)
            .show(ui, |ui| {
                for release in &row.releases {
                    let held = row
                        .installed
                        .as_ref()
                        .is_some_and(|installed| installed.version == release.version);
                    property(
                        ui,
                        &release.version,
                        &byte_size(release.archive_length),
                        if held {
                            "on this machine"
                        } else {
                            release.state.pill()
                        },
                    );
                }
            });
        ui.label(
            RichText::new(if row.missing.is_empty() {
                match row.releases.first() {
                    Some(release) if release.capabilities.is_empty() => {
                        "Needs nothing beyond the core engine.".to_owned()
                    }
                    Some(release) => format!(
                        "Needs {} — this build provides all of them.",
                        release.capabilities.join(" · ")
                    ),
                    None => String::new(),
                }
            } else {
                format!(
                    "Needs {} — not in this build, so installing is refused.",
                    plain_list(&row.missing)
                )
            })
            .small()
            .color(if row.missing.is_empty() {
                t.color.text_faint
            } else {
                t.color.err
            }),
        );
        if let Some(diff) = diff {
            super::adoption::pane(ui, app, row, diff);
        }
    });
}

/// What this machine holds and what this project adopted, in one pane.
///
/// They are the two halves of "can my results be attributed", and reading them
/// together is the point of the ledger.
fn catalog_pane(ui: &mut Ui, row: &HubLedgerRow, proof: Option<&PackReProof>) {
    let t = Tokens::get(ui.ctx());
    let licence = row
        .releases
        .first()
        .map(|release| release.spdx.clone())
        .unwrap_or_default();
    detail_pane(ui, "CATALOG", Some(&licence), |ui| {
        let mut verdict = None;
        match row.installed.as_ref() {
            Some(installed) => {
                property(ui, "Installed", &installed.version, "this machine");
                verdict = Some(evidence(installed, proof));
            }
            None => property(
                ui,
                "Installed",
                "nothing of this pack is here",
                "installing fetches and proves the release archive",
            ),
        }
        match project_cell(&row.adoption) {
            Some((label, _)) => property(ui, "In this project", &label, "recorded pin"),
            None => property(ui, "In this project", "no parts adopted", "no pin recorded"),
        }
        if let Some(update) = row.update.as_deref()
            && row.installed.is_some()
        {
            property(ui, "Offered", update, "updates are notified, never applied");
        }
        // The evidence is a sentence, so it gets a line rather than a cell. A
        // property value is a third of the pane wide and the painter clips
        // without an ellipsis, which turned "the retained archive no longer
        // hashes to the published digest" into "the retained archive no lon" —
        // a phrase that reads as reassurance.
        if let Some((value, origin)) = verdict {
            ui.label(
                RichText::new(&value)
                    .small()
                    .color(if value.starts_with("verified") {
                        t.color.text_dim
                    } else {
                        t.color.warn
                    }),
            );
            ui.label(RichText::new(origin).small().color(t.color.text_faint));
        }
    });
}

/// What is known about the bytes of one installed release, and who says so.
fn evidence(installed: &InstalledRelease, proof: Option<&PackReProof>) -> (String, &'static str) {
    match (installed.archive, proof) {
        (Some(ArchiveEvidence::DiffersFromCatalog), _) => (
            "the retained archive no longer hashes to the published digest".to_owned(),
            "startup comparison against the signed catalog",
        ),
        (_, Some(PackReProof::Failed(reason))) => {
            (reason.clone(), "re-proof under the release key")
        }
        (_, Some(PackReProof::Verified)) => (
            "verified end to end under the release key".to_owned(),
            "re-proved this session",
        ),
        (Some(ArchiveEvidence::MatchesCatalog), None) => (
            "the archive matches the published digest; nothing has re-proved it".to_owned(),
            "startup comparison against the signed catalog",
        ),
        (_, None) => (
            "nothing has re-proved this release".to_owned(),
            "no catalog entry to compare the archive against",
        ),
    }
}

/// Builds the confirmation this release's action needs.
fn confirmation(row: &HubPackRow, replaces: Option<String>) -> ModelsWorkbenchDialog {
    ModelsWorkbenchDialog::ConfirmPack {
        pack_id: row.pack_id.clone(),
        attach: true,
        release: Some(Box::new(PackReleaseConfirmation {
            name: row.name.clone(),
            version: row.version.clone(),
            spdx: row.spdx.clone(),
            archive_length: row.archive_length,
            parts: row.parts,
            capabilities: row.capabilities.clone(),
            missing: match &row.state {
                HubPackState::Incompatible { missing } => missing.clone(),
                _ => Vec::new(),
            },
            part: None,
            replaces,
        })),
    }
}

/// Renders the release half of the pack confirmation dialog.
///
/// It states everything the action costs and everything it commits to before
/// it happens: which release, under which licence, how many bytes, how many
/// parts, and whether this engine can run it. The primary action is disabled —
/// with the reason — when it cannot.
pub(super) fn release_confirmation(
    ui: &mut Ui,
    app: &mut ManagerRenderContext<'_>,
    pack_id: &str,
    release: &PackReleaseConfirmation,
) -> Option<bool> {
    let mut decision = None;
    ui.label(if release.replaces.is_some() {
        "RSpice will download and prove the newer release, then remove the older copy. A project \
         that already added a part keeps the exact bytes it was built against."
    } else if release.part.is_some() {
        "RSpice will download and prove this release, retain the chosen part's source into the \
         project, and arm it for placement."
    } else {
        "RSpice will download this release and verify it end to end against the compiled-in \
         release key before anything reaches disk."
    });
    ui.label(
        RichText::new(format!("{} {}", release.name, release.version))
            .monospace()
            .strong(),
    );
    card(ui, |ui| {
        card_title(ui, "WHAT WILL BE INSTALLED", Some(pack_id));
        property(ui, "License", &release.spdx, "signed manifest");
        property(
            ui,
            "Download",
            &byte_size(release.archive_length),
            "exact signed length",
        );
        property(
            ui,
            "Parts",
            &release.parts.to_string(),
            "addressable models",
        );
        property(
            ui,
            "Capability check",
            &if release.missing.is_empty() {
                "this build runs every part".to_owned()
            } else {
                format!("missing {}", plain_list(&release.missing))
            },
            if release.missing.is_empty() {
                "passed"
            } else {
                "failed"
            },
        );
        if let Some(part) = release.part.as_deref() {
            property(ui, "Part", part, "retained into the project");
        }
        if let Some(replaces) = release.replaces.as_deref() {
            property(ui, "Replaces", replaces, "removed after success");
        }
    });
    ui.horizontal(|ui| {
        if ui.button("Cancel").clicked() {
            decision = Some(false);
        }
        let installable =
            release.missing.is_empty() && !app.state.workbench.models_view.model_import_in_progress;
        if ui
            .add_enabled(installable, egui::Button::new("Install pack"))
            .on_disabled_hover_text(if release.missing.is_empty() {
                "Another model-source operation is still running.".to_owned()
            } else {
                format!(
                    "This build does not offer {}.",
                    plain_list(&release.missing)
                )
            })
            .clicked()
        {
            decision = Some(true);
        }
    });
    decision
}

/// Turns the request this confirmation describes into hub work.
pub(super) fn release_request(pack_id: &str, release: &PackReleaseConfirmation) -> ModelHubRequest {
    match release.replaces.as_deref() {
        Some(installed) => ModelHubRequest::UpdatePack {
            pack_id: pack_id.to_owned(),
            installed: installed.to_owned(),
            latest: release.version.clone(),
        },
        None => ModelHubRequest::InstallPack {
            pack_id: pack_id.to_owned(),
            version: release.version.clone(),
            part: release.part.clone(),
        },
    }
}

/// "a, b and c" — a list a sentence can contain.
pub(super) fn plain_list(values: &[String]) -> String {
    match values {
        [] => "nothing".to_owned(),
        [only] => only.clone(),
        [rest @ .., last] => format!("{} and {last}", rest.join(", ")),
    }
}

/// How many packs each ledger facet admits, in one pass over the ledger.
///
/// The chips and the table read the same predicate over the same projection,
/// which is what makes a count that disagrees with the rows shown impossible
/// rather than merely unlikely. One pass rather than one per chip because
/// deciding a row's attention builds the sentence that explains it, and doing
/// that five times per row to fill five counters is four times more prose than
/// anybody reads.
pub(super) fn ledger_facet_counts(
    hub: &HubCatalog,
    app: &ManagerRenderContext<'_>,
) -> [usize; ModelHubFacet::ALL.len()] {
    let mut counts = [0; ModelHubFacet::ALL.len()];
    for row in &hub.packs {
        let attention = pack_attention(row, verification_of(app, row));
        for (index, facet) in ModelHubFacet::ALL.into_iter().enumerate() {
            if ledger_matches(row, attention.as_ref(), facet) {
                counts[index] += 1;
            }
        }
    }
    counts
}

#[cfg(test)]
mod tests {
    use super::*;

    use crate::workbench::app_state::AppState;

    /// Renders one models-workspace fragment and returns its access tree.
    fn accessibility_nodes(
        state: &mut AppState,
        size: egui::Vec2,
        render: impl FnOnce(&mut Ui, &mut ManagerRenderContext<'_>),
    ) -> Vec<(egui::accesskit::NodeId, egui::accesskit::Node)> {
        let ctx = egui::Context::default();
        ctx.enable_accesskit();
        crate::ui::Theme::default().apply(&ctx);
        let mut pending = Vec::new();
        // `run_ui` takes an `FnMut`, so the one-shot renderer is parked where
        // the closure can take it rather than consuming a captured value.
        let mut render = Some(render);
        let output = ctx.run_ui(
            egui::RawInput {
                screen_rect: Some(egui::Rect::from_min_size(egui::Pos2::ZERO, size)),
                ..egui::RawInput::default()
            },
            |ctx| {
                egui::CentralPanel::default().show(ctx, |ui| {
                    let mut context = ManagerRenderContext {
                        state,
                        pending_actions: &mut pending,
                    };
                    if let Some(render) = render.take() {
                        render(ui, &mut context);
                    }
                });
            },
        );
        output
            .platform_output
            .accesskit_update
            .expect("the models workspace publishes an access tree")
            .nodes
    }

    fn button(
        nodes: &[(egui::accesskit::NodeId, egui::accesskit::Node)],
        label: &str,
    ) -> Option<egui::accesskit::Node> {
        nodes
            .iter()
            .find(|(_, node)| {
                node.role() == egui::accesskit::Role::Button && node.label() == Some(label)
            })
            .map(|(_, node)| node.clone())
    }

    fn labelled(nodes: &[(egui::accesskit::NodeId, egui::accesskit::Node)], label: &str) -> bool {
        nodes
            .iter()
            .any(|(_, node)| node.label().is_some_and(|found| found == label))
    }

    fn release(missing: &[&str]) -> PackReleaseConfirmation {
        PackReleaseConfirmation {
            name: "RSpice proving pack".to_owned(),
            version: "1.0.0".to_owned(),
            spdx: "LicenseRef-RSpice-Models".to_owned(),
            archive_length: 2 * 1024 * 1024,
            parts: 12,
            capabilities: vec!["subckt".to_owned(), "resistor".to_owned()],
            missing: missing.iter().map(|value| (*value).to_owned()).collect(),
            part: None,
            replaces: None,
        }
    }

    fn row(name: &str, version: &str, state: HubPackState) -> HubPackRow {
        HubPackRow {
            pack_id: format!("rspice-{}", name.to_ascii_lowercase()),
            name: name.to_owned(),
            category: "proving".to_owned(),
            version: version.to_owned(),
            state,
            spdx: "LicenseRef-RSpice-Models".to_owned(),
            archive_length: 1024 * 1024,
            parts: 4,
            capabilities: vec!["subckt".to_owned()],
            archive: None,
        }
    }

    /// One installed release, with whatever the startup sweep concluded.
    fn held(version: &str, archive: Option<ArchiveEvidence>) -> InstalledRelease {
        InstalledRelease {
            version: version.to_owned(),
            archive,
            archive_sha256: "a".repeat(64),
        }
    }

    /// One project pin naming a release of the proving pack.
    fn pin(version: &str, archive: &str) -> PackPartPin {
        PackPartPin {
            pack_id: "rspice-proving".to_owned(),
            pack_version: version.to_owned(),
            archive_sha256: archive.to_owned(),
            part_id: "opa2".to_owned(),
        }
    }

    /// One ledger line, assembled the way the projection assembles one.
    fn pack(releases: Vec<HubPackRow>, installed: Option<InstalledRelease>) -> HubLedgerRow {
        pinned_pack(releases, installed, &[])
    }

    fn pinned_pack(
        mut releases: Vec<HubPackRow>,
        installed: Option<InstalledRelease>,
        pins: &[PackPartPin],
    ) -> HubLedgerRow {
        releases.sort_by(newest_release_first);
        let pack_id = releases
            .first()
            .map_or_else(|| "rspice-proving".to_owned(), |row| row.pack_id.clone());
        ledger_row(&pack_id, releases, installed, pins)
    }

    /// A catalog holding the given packs, signed and verified.
    fn catalog(packs: Vec<HubLedgerRow>, age_days: Option<u64>, stale: bool) -> HubCatalog {
        HubCatalog {
            packs,
            age_days,
            signed: Some("2026-08-14".to_owned()),
            unavailable: None,
            stale,
            cache_discarded: false,
            identity: None,
            signing_key: "7ce1".to_owned(),
            licences: vec!["LicenseRef-RSpice-Models".to_owned()],
        }
    }

    /// One catalog-listed pack publishing the given releases, all runnable.
    fn listed(versions: &[&str]) -> SnapshotPack {
        SnapshotPack {
            id: "rspice-proving".to_owned(),
            name: "RSpice proving pack".to_owned(),
            category: "proving".to_owned(),
            releases: versions
                .iter()
                .map(|version| rspice_pack::SnapshotRelease {
                    version: (*version).to_owned(),
                    archive_sha256: "0".repeat(64),
                    archive_length: 1024 * 1024,
                    capabilities: vec!["subckt".to_owned()],
                    spdx: "LicenseRef-RSpice-Models".to_owned(),
                    parts: Vec::new(),
                })
                .collect(),
        }
    }

    fn state_of(rows: &[HubPackRow], version: &str) -> HubPackState {
        rows.iter()
            .find(|row| row.version == version)
            .unwrap_or_else(|| panic!("the table lists {version}"))
            .state
            .clone()
    }

    #[test]
    fn a_two_digit_major_release_supersedes_the_single_digit_one_installed() {
        // Byte order ranks `9.0.0` above `10.0.0`, so the shelf both picked
        // the wrong release as newest and refused the comparison that would
        // have caught it. A machine on 9.0.0 was told it was current.
        let rows = pack_rows(&listed(&["9.0.0", "10.0.0"]), Some("9.0.0"));
        assert_eq!(state_of(&rows, "9.0.0"), HubPackState::Installed);
        assert_eq!(
            state_of(&rows, "10.0.0"),
            HubPackState::UpdateAvailable {
                installed: "9.0.0".to_owned(),
            },
            "the newest release the catalog publishes is the one that updates"
        );
    }

    #[test]
    fn a_pre_release_never_supersedes_the_release_it_precedes() {
        // The same byte order ranks `1.2.0-rc.2` above `1.2.0`, which offered
        // an update that walks a machine backwards onto a candidate build.
        let rows = pack_rows(&listed(&["1.2.0", "1.2.0-rc.2"]), Some("1.2.0"));
        assert_eq!(state_of(&rows, "1.2.0"), HubPackState::Installed);
        assert_eq!(state_of(&rows, "1.2.0-rc.2"), HubPackState::Available);

        // And the inspector orders a pack's releases the same way, so the one
        // it puts first is the one the ledger calls newest.
        let mut ordered = ["1.2.0-rc.1", "1.2.0", "1.10.0", "1.9.0"]
            .map(|version| row("Proving", version, HubPackState::Available))
            .to_vec();
        ordered.sort_by(newest_release_first);
        assert_eq!(
            ordered
                .iter()
                .map(|row| row.version.as_str())
                .collect::<Vec<_>>(),
            ["1.10.0", "1.9.0", "1.2.0", "1.2.0-rc.1"]
        );
    }

    #[test]
    fn the_release_confirmation_states_its_cost_and_exposes_both_actions() {
        let mut state = AppState::default();
        let confirmation = release(&[]);
        let nodes = accessibility_nodes(&mut state, egui::vec2(640.0, 520.0), |ui, app| {
            release_confirmation(ui, app, "rspice-proving", &confirmation);
        });
        assert!(button(&nodes, "Cancel").is_some(), "cancel is reachable");
        let install = button(&nodes, "Install pack").expect("the primary action is reachable");
        assert!(
            !install.is_disabled(),
            "a compatible release is installable"
        );
        // The cost the dialog paints comes from the confirmation it captured,
        // which is what the request it dispatches is built from too. Asserting
        // the projection rather than the painted glyphs is deliberate: this
        // workspace's property rows are painter text and expose no
        // accessibility node, so a tree assertion here would prove nothing.
        assert_eq!(byte_size(confirmation.archive_length), "2.0 MB");
        assert_eq!(confirmation.spdx, "LicenseRef-RSpice-Models");
        assert_eq!(confirmation.parts, 12);
        assert_eq!(
            release_request("rspice-proving", &confirmation),
            ModelHubRequest::InstallPack {
                pack_id: "rspice-proving".to_owned(),
                version: "1.0.0".to_owned(),
                part: None,
            }
        );
    }

    #[test]
    fn a_confirmed_update_replaces_the_release_it_names() {
        let mut confirmation = release(&[]);
        confirmation.version = "1.1.0".to_owned();
        confirmation.replaces = Some("1.0.0".to_owned());
        assert_eq!(
            release_request("rspice-proving", &confirmation),
            ModelHubRequest::UpdatePack {
                pack_id: "rspice-proving".to_owned(),
                installed: "1.0.0".to_owned(),
                latest: "1.1.0".to_owned(),
            }
        );
    }

    #[test]
    fn an_incompatible_release_is_described_and_refused_rather_than_hidden() {
        let mut state = AppState::default();
        let confirmation = release(&["nonexistent-capability"]);
        let nodes = accessibility_nodes(&mut state, egui::vec2(640.0, 520.0), |ui, app| {
            release_confirmation(ui, app, "rspice-proving", &confirmation);
        });
        let install = button(&nodes, "Install pack").expect("the action is still present");
        assert!(
            install.is_disabled(),
            "an incompatible release cannot be installed"
        );
        assert_eq!(
            plain_list(&confirmation.missing),
            "nonexistent-capability",
            "the reason offered is the plain capability name"
        );
    }

    #[test]
    fn every_pack_state_is_reachable_and_offers_the_action_it_earns() {
        // One row per pack now, so the states are states of a *pack*: nothing
        // held, held and current, held and superseded, and offered but
        // unrunnable. Each earns exactly the controls it can honour.
        let offered = pack(vec![row("Proving", "1.0.0", HubPackState::Available)], None);
        let current = pack(
            vec![row("Proving", "1.0.0", HubPackState::Installed)],
            Some(held("1.0.0", Some(ArchiveEvidence::MatchesCatalog))),
        );
        let superseded = pack(
            vec![
                row("Proving", "1.0.0", HubPackState::Installed),
                row(
                    "Proving",
                    "1.1.0",
                    HubPackState::UpdateAvailable {
                        installed: "1.0.0".to_owned(),
                    },
                ),
            ],
            Some(held("1.0.0", Some(ArchiveEvidence::MatchesCatalog))),
        );
        let unrunnable = pack(
            vec![row(
                "Proving",
                "1.0.0",
                HubPackState::Incompatible {
                    missing: vec!["nonexistent-capability".to_owned()],
                },
            )],
            None,
        );

        for (ledger, offers, refuses) in [
            (offered, vec!["Install"], vec![]),
            (current, vec!["Verify installed", "Remove"], vec![]),
            (
                superseded,
                vec!["Update", "Verify installed", "Remove"],
                vec![],
            ),
            (unrunnable, vec![], vec!["Install"]),
        ] {
            let mut app_state = AppState::default();
            app_state.workbench.models_view.selected_pack = Some(ledger.pack_id.clone());
            let listed = catalog(vec![ledger], Some(2), false);
            let nodes =
                accessibility_nodes(&mut app_state, egui::vec2(1100.0, 760.0), move |ui, app| {
                    packs_page(ui, app, &listed);
                });
            for label in offers {
                let node = button(&nodes, label)
                    .unwrap_or_else(|| panic!("{label} is reachable on this pack"));
                assert!(!node.is_disabled(), "{label} is offered, not refused");
            }
            for label in refuses {
                let node = button(&nodes, label)
                    .unwrap_or_else(|| panic!("{label} states its refusal rather than vanishing"));
                assert!(node.is_disabled(), "{label} refuses with its reason");
            }
        }
    }

    /// The chips and the table are one predicate over one projection.
    ///
    /// The mockup shipped this defect first: facet counts derived from a
    /// second walk over the corpus could — and did — disagree with the rows
    /// the table then showed. `Installed` and `Available` are complements, so
    /// they must account for every pack exactly once, and every chip's count
    /// must equal the number of rows its own facet admits.
    #[test]
    fn the_facet_counts_partition_the_pack_total() {
        let named = |name: &str, state: HubPackState| {
            let mut listed = row(name, "1.0.0", state);
            listed.pack_id = format!("rspice-{}", name.to_ascii_lowercase());
            vec![listed]
        };
        let packs = vec![
            // Held, current, and nothing has re-proved it: needs attention.
            pack(
                named("Alpha", HubPackState::Installed),
                Some(held("1.0.0", Some(ArchiveEvidence::MatchesCatalog))),
            ),
            // Offered and runnable — an offer, which is also attention.
            pack(named("Beta", HubPackState::Available), None),
            // Held, matching, and pinned by this project.
            pinned_pack(
                named("Gamma", HubPackState::Installed),
                Some(held("1.0.0", Some(ArchiveEvidence::MatchesCatalog))),
                &[pin("1.0.0", &"a".repeat(64))],
            ),
            // Offered, and this build cannot run it.
            pack(
                named(
                    "Delta",
                    HubPackState::Incompatible {
                        missing: vec!["nonexistent-capability".to_owned()],
                    },
                ),
                None,
            ),
        ];
        let hub = catalog(packs, Some(1), false);
        let mut state = AppState::default();
        let mut pending = Vec::new();
        let app = ManagerRenderContext {
            state: &mut state,
            pending_actions: &mut pending,
        };

        let counts = ledger_facet_counts(&hub, &app);
        let count = |facet: ModelHubFacet| {
            counts[ModelHubFacet::ALL
                .iter()
                .position(|candidate| *candidate == facet)
                .expect("every facet is in the registry")]
        };
        assert_eq!(count(ModelHubFacet::All), hub.packs.len());
        assert_eq!(
            count(ModelHubFacet::Installed) + count(ModelHubFacet::Available),
            count(ModelHubFacet::All),
            "installed and available are complements, so they partition the ledger"
        );
        assert_eq!(count(ModelHubFacet::Installed), 2);
        assert_eq!(count(ModelHubFacet::Pinned), 1);

        // And every chip's count is the number of rows its facet admits, which
        // is what makes a chip that lies impossible rather than unlikely.
        for facet in ModelHubFacet::ALL {
            let admitted = hub
                .packs
                .iter()
                .filter(|row| ledger_matches(row, pack_attention(row, None).as_ref(), facet))
                .count();
            assert_eq!(count(facet), admitted, "{} chip", facet.label());
        }
    }

    /// The ladder reports the most decisive exception, and only that one.
    #[test]
    fn the_attention_ladder_reports_the_most_decisive_exception_first() {
        let phrase = |row: &HubLedgerRow, proof: Option<&PackReProof>| {
            pack_attention(row, proof).map(|attention| attention.phrase)
        };

        // Nothing held and something on offer: nothing to decide. The offer is
        // on the Install control, which names the version it would fetch; a
        // row that shouted "update 1.0.0" at every pack nobody installed would
        // make the whole column noise.
        let offered = pack(vec![row("Proving", "1.0.0", HubPackState::Available)], None);
        assert_eq!(phrase(&offered, None), None);

        // Held and re-proved this session: the silent state. The verdict is
        // reported in the inspector, where somebody asking about this one pack
        // will find it — a table that shouts "verified" on every healthy row
        // buries the rows that are not.
        let current = pack(
            vec![row("Proving", "1.0.0", HubPackState::Installed)],
            Some(held("1.0.0", Some(ArchiveEvidence::MatchesCatalog))),
        );
        assert_eq!(phrase(&current, Some(&PackReProof::Verified)), None);
        assert_eq!(phrase(&current, None), Some("never re-proved".to_owned()));
        assert_eq!(
            phrase(&current, Some(&PackReProof::Failed("truncated".to_owned()))),
            Some("re-proof failed".to_owned())
        );

        // The startup comparison outranks every later verdict: bytes that no
        // longer hash to the published digest are not this release.
        let replaced = pack(
            vec![row("Proving", "1.0.0", HubPackState::Installed)],
            Some(held("1.0.0", Some(ArchiveEvidence::DiffersFromCatalog))),
        );
        assert_eq!(
            phrase(&replaced, Some(&PackReProof::Verified)),
            Some("archive differs".to_owned())
        );

        // A pin naming an archive this machine no longer has outranks a failed
        // re-proof: the re-proof can be run again, and the bytes cannot.
        let repinned = pinned_pack(
            vec![row("Proving", "1.0.0", HubPackState::Installed)],
            Some(held("1.0.0", Some(ArchiveEvidence::MatchesCatalog))),
            &[pin("1.0.0", &"b".repeat(64))],
        );
        assert_eq!(
            phrase(
                &repinned,
                Some(&PackReProof::Failed("truncated".to_owned()))
            ),
            Some("pinned archive replaced".to_owned())
        );

        // A pin whose release the catalog no longer publishes is worth saying
        // and is not urgent: the retained sources still execute, and only the
        // attribution is unprovable.
        let withdrawn = pinned_pack(Vec::new(), None, &[pin("0.9.0", &"a".repeat(64))]);
        assert_eq!(
            phrase(&withdrawn, None),
            Some("pin not installed".to_owned())
        );
    }

    /// The Project cell states the commitment, and how it diverged.
    #[test]
    fn the_project_column_states_the_release_this_design_committed_to() {
        let matching = pinned_pack(
            vec![row("Proving", "1.0.0", HubPackState::Installed)],
            Some(held("1.0.0", Some(ArchiveEvidence::MatchesCatalog))),
            &[pin("1.0.0", &"a".repeat(64))],
        );
        assert_eq!(
            project_cell(&matching.adoption),
            Some(("1 part @ 1.0.0".to_owned(), false)),
            "a matching pin states the commitment and nothing else"
        );

        let stale = pinned_pack(
            vec![row("Proving", "1.1.0", HubPackState::Installed)],
            Some(held("1.1.0", Some(ArchiveEvidence::MatchesCatalog))),
            &[pin("1.0.0", &"a".repeat(64)), pin("1.0.0", &"a".repeat(64))],
        );
        assert_eq!(
            project_cell(&stale.adoption),
            Some(("2 parts @ 1.0.0 · differs from 1.1.0".to_owned(), true))
        );

        let unpinned = pack(vec![row("Proving", "1.0.0", HubPackState::Available)], None);
        assert_eq!(
            project_cell(&unpinned.adoption),
            None,
            "a pack this project never adopted leaves the cell blank"
        );
    }

    #[test]
    fn the_status_line_states_what_is_held_and_offers_the_two_things_to_do() {
        assert_eq!(
            catalog_summary(Some("2026-08-14"), Some(0), Some(41), false),
            "Catalog signed 2026-08-14 · generation 41 · verified"
        );
        assert_eq!(
            catalog_summary(Some("2026-08-14"), Some(9), None, false),
            "Catalog signed 2026-08-14 · verified · 9 days old",
            "a cached snapshot carries no generation, so none is claimed"
        );
        assert_eq!(
            catalog_summary(None, None, None, false),
            "No catalog fetched"
        );

        let mut state = AppState::default();
        let stale = catalog(
            vec![pack(
                vec![row("Proving", "1.0.0", HubPackState::Available)],
                None,
            )],
            Some(9),
            true,
        );
        let nodes = accessibility_nodes(&mut state, egui::vec2(1100.0, 760.0), move |ui, app| {
            packs_page(ui, app, &stale);
        });
        assert!(button(&nodes, "Refresh catalog").is_some());
        assert!(
            button(&nodes, "Catalog details…").is_some(),
            "the doctrine is one button away rather than printed on the page"
        );

        // A hub that could not open says that instead of a fresh-looking age.
        let mut state = AppState::default();
        let unavailable = HubCatalog {
            unavailable: Some("RSpice could not open its model-pack storage.".to_owned()),
            ..HubCatalog::default()
        };
        let nodes = accessibility_nodes(&mut state, egui::vec2(1100.0, 760.0), move |ui, app| {
            packs_page(ui, app, &unavailable);
        });
        assert!(
            button(&nodes, "Refresh catalog").is_none(),
            "there is nothing to refresh"
        );
        assert!(
            button(&nodes, "Install").is_none(),
            "and nothing to install"
        );
    }

    /// Every operational state the workspace can reach paints its own banner.
    ///
    /// The vocabulary existed for a release with no reader: fourteen variants,
    /// twenty-five writers, and `label()` behind `#[cfg(test)]`. The check that
    /// matters is therefore not "does one failure render" but "does every
    /// variant render", because a variant nothing paints is a variant nobody
    /// can act on.
    #[test]
    fn every_operational_state_paints_its_word_and_its_consequence() {
        use crate::workbench::state::{ModelsAttemptedOperation, ModelsOperationalState};

        for state in ModelsOperationalState::ALL {
            let mut app_state = AppState::default();
            app_state.workbench.models_view.operational_state = state;
            app_state.workbench.models_view.action_receipt =
                Some(Err("the pack format refused: truncated archive".to_owned()));
            app_state.workbench.models_view.attempted_operation = Some(ModelsAttemptedOperation {
                label: "model-pack install of 'rspice-proving 1.0.0'".to_owned(),
                reissuable: false,
            });
            let listed = catalog(
                vec![pack(
                    vec![row("Proving", "1.0.0", HubPackState::Available)],
                    None,
                )],
                Some(1),
                false,
            );
            let nodes =
                accessibility_nodes(&mut app_state, egui::vec2(1100.0, 760.0), move |ui, app| {
                    packs_page(ui, app, &listed);
                });
            assert!(
                labelled(&nodes, state.label()),
                "the {} banner names its state",
                state.label()
            );
            assert!(
                labelled(&nodes, state.consequence()),
                "the {} banner says what the failure left behind",
                state.label()
            );
            assert!(
                labelled(&nodes, "model-pack install of 'rspice-proving 1.0.0'"),
                "the {} banner names the operation it is about",
                state.label()
            );
            assert!(
                button(&nodes, "Retry the catalog refresh").is_none(),
                "an install is retried from its own pack row, never from a \
                 button that would have to guess the version"
            );
        }
    }

    #[test]
    fn a_failed_catalog_refresh_offers_the_one_retry_it_can_re_issue() {
        use crate::workbench::state::{ModelsAttemptedOperation, ModelsOperationalState};

        let mut app_state = AppState::default();
        app_state.workbench.models_view.operational_state = ModelsOperationalState::Offline;
        app_state.workbench.models_view.action_receipt =
            Some(Err("the model hub could not be reached".to_owned()));
        app_state.workbench.models_view.attempted_operation = Some(ModelsAttemptedOperation {
            label: "model-catalog refresh".to_owned(),
            reissuable: true,
        });
        let offline = catalog(
            vec![pack(
                vec![row("Proving", "1.0.0", HubPackState::Available)],
                None,
            )],
            None,
            true,
        );
        let nodes = accessibility_nodes(&mut app_state, egui::vec2(1100.0, 760.0), |ui, app| {
            packs_page(ui, app, &offline);
        });
        assert!(button(&nodes, "Retry the catalog refresh").is_some());

        // And a page whose last operation succeeded says nothing at all.
        let mut healthy = AppState::default();
        healthy.workbench.models_view.action_receipt = Some(Ok("installed".to_owned()));
        let current = catalog(
            vec![pack(
                vec![row("Proving", "1.0.0", HubPackState::Available)],
                None,
            )],
            Some(0),
            false,
        );
        let nodes = accessibility_nodes(&mut healthy, egui::vec2(1100.0, 760.0), |ui, app| {
            packs_page(ui, app, &current);
        });
        assert!(
            !labelled(&nodes, ModelsOperationalState::Ready.consequence()),
            "the healthy state is silent"
        );
    }

    /// Every proof state an installed release can be in reaches the reader.
    ///
    /// The re-proof itself has existed since packs shipped — `verify_installed`
    /// re-hashes the archive under the release key — with no control that
    /// called it and no cell that reported it, and the startup sweep computed
    /// each archive's digest and compared it to nothing. The phrases below are
    /// what those two facts look like once a reader can see them: the loud ones
    /// on the row, and every one of them in the inspector.
    #[test]
    fn an_installed_release_reports_what_re_proved_it_and_what_did_not() {
        let installed = held("1.0.0", Some(ArchiveEvidence::MatchesCatalog));
        assert_eq!(
            evidence(&installed, Some(&PackReProof::Verified)).1,
            "re-proved this session"
        );
        assert_eq!(
            evidence(&installed, None).0,
            "the archive matches the published digest; nothing has re-proved it"
        );
        assert_eq!(
            evidence(
                &installed,
                Some(&PackReProof::Failed("truncated archive".to_owned()))
            ),
            (
                "truncated archive".to_owned(),
                "re-proof under the release key"
            )
        );
        let replaced = held("1.0.0", Some(ArchiveEvidence::DiffersFromCatalog));
        assert_eq!(
            evidence(&replaced, Some(&PackReProof::Verified)).0,
            "the retained archive no longer hashes to the published digest"
        );

        // And the action that produces the verdict is reachable on exactly the
        // packs that can be re-proved.
        let mut state = AppState::default();
        state.workbench.models_view.selected_pack = Some("rspice-proving".to_owned());
        let field = catalog(
            vec![pack(
                vec![row("Proving", "1.0.0", HubPackState::Installed)],
                Some(installed),
            )],
            Some(1),
            false,
        );
        let nodes = accessibility_nodes(&mut state, egui::vec2(1100.0, 760.0), move |ui, app| {
            packs_page(ui, app, &field);
        });
        assert!(button(&nodes, "Verify installed").is_some());

        let mut state = AppState::default();
        state.workbench.models_view.selected_pack = Some("rspice-proving".to_owned());
        let offered = catalog(
            vec![pack(
                vec![row("Proving", "1.1.0", HubPackState::Available)],
                None,
            )],
            Some(1),
            false,
        );
        let nodes = accessibility_nodes(&mut state, egui::vec2(1100.0, 760.0), move |ui, app| {
            packs_page(ui, app, &offered);
        });
        assert!(
            button(&nodes, "Verify installed").is_none(),
            "there is nothing on this machine to re-prove"
        );
    }

    #[test]
    fn a_discarded_catalog_cache_is_not_reported_as_never_having_asked() {
        assert_eq!(
            catalog_summary(None, None, None, true),
            "The cached catalog failed verification and was discarded — refresh"
        );

        let mut state = AppState::default();
        let discarded = HubCatalog {
            packs: vec![pack(
                vec![row("Proving", "1.0.0", HubPackState::Available)],
                None,
            )],
            stale: true,
            cache_discarded: true,
            ..HubCatalog::default()
        };
        let nodes = accessibility_nodes(&mut state, egui::vec2(1100.0, 760.0), move |ui, app| {
            packs_page(ui, app, &discarded);
        });
        assert!(
            labelled(
                &nodes,
                "The cached catalog failed verification and was discarded — refresh"
            ),
            "the status line carries the discarded-cache state"
        );
        assert!(button(&nodes, "Refresh catalog").is_some());
    }

    #[test]
    fn a_byte_count_reads_as_a_size_rather_than_a_number() {
        assert_eq!(byte_size(0), "—");
        assert_eq!(byte_size(512), "512 B");
        assert_eq!(byte_size(2048), "2.0 kB");
        assert_eq!(byte_size(3 * 1024 * 1024), "3.0 MB");
        assert_eq!(byte_size(150 * 1024 * 1024), "150 MB");
    }
}
