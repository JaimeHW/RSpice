//! Model Hub work, on the model-catalog operation machine.
//!
//! Installing a signed pack is a model-catalog change like importing a model
//! source: it runs in the background, it can be superseded by the project
//! moving underneath it, and it ends in a receipt and a toast. So it rides the
//! machine the parent module already owns — the same in-progress gate, the
//! same authority recheck, the same completion queue — and this module holds
//! only what is specific to packs: the request vocabulary, the worker that
//! executes one request against a hub of its own, and the publication step
//! that lands the result in the session.
//!
//! # A worker does not borrow the session's hub
//!
//! It opens its own over the same store. Two hubs over one store always agree,
//! because the store is where the state lives, and the session re-reads it
//! when the operation returns. That is what keeps the whole background path
//! free of shared mutable application state.

#[cfg(all(test, not(target_arch = "wasm32")))]
mod tests;

use egui::Context;

use crate::diagnostics::ConsoleMessage;
use crate::workbench::app_state::AppState;
use crate::workbench::app_state::design_history::publish_model_library_candidate;
use crate::workbench::state::ModelsOperationalState;

/// One unit of Model Hub work, on either target.
///
/// It rides the same operation machine as every other model-catalog
/// operation — the same in-progress gate, the same authority recheck, the
/// same toast and receipt — because a pack install is a model-catalog change
/// like any other and a second machine would have to relearn all of it.
#[derive(Debug, Clone, PartialEq, Eq)]
pub(in crate::workbench) enum ModelHubRequest {
    /// Fetch, prove, and cache the signed catalog snapshot.
    FetchSnapshot,
    /// Download, prove, and install one published release. `part` asks for one
    /// of its parts to be retained into the project afterwards, which is what
    /// makes "add this part" a single gesture from the component shelf.
    InstallPack {
        pack_id: String,
        version: String,
        part: Option<String>,
    },
    /// Install a newer release, then remove the older one — in that order, so
    /// a failed download never leaves the machine with neither.
    UpdatePack {
        pack_id: String,
        installed: String,
        latest: String,
    },
    /// Move one part this project already retains onto a newer release.
    ///
    /// Per part, and always explicit. An update is notified and never applied,
    /// so nothing here replaces what the machine holds: the newer release is
    /// installed beside the older one if it is missing — bytes have to exist
    /// before they can be proved — the one named part is re-retained from it,
    /// and every other part pinned to the older release stays exactly where it
    /// is. `from_version` is carried so the receipt can say what moved rather
    /// than only where it landed.
    AdoptPart {
        pack_id: String,
        from_version: String,
        to_version: String,
        part_id: String,
    },
    RemovePack {
        pack_id: String,
        version: String,
    },
    /// Re-prove an installed release end to end under the release key.
    ///
    /// It reads and hashes the whole archive, which is why it is a background
    /// operation and never something a frame does on its own initiative.
    VerifyInstalled {
        pack_id: String,
        version: String,
    },
}

impl ModelHubRequest {
    pub(super) fn progress_label(&self) -> String {
        match self {
            Self::FetchSnapshot => "Fetching and proving the model catalog…".to_owned(),
            Self::InstallPack {
                pack_id, version, ..
            } => format!("Downloading and proving {pack_id} {version}…"),
            Self::UpdatePack {
                pack_id, latest, ..
            } => format!("Downloading and proving {pack_id} {latest}…"),
            Self::AdoptPart {
                part_id,
                to_version,
                ..
            } => format!("Re-retaining {part_id} from {to_version}…"),
            Self::RemovePack { pack_id, version } => {
                format!("Removing the installed copy of {pack_id} {version}…")
            }
            Self::VerifyInstalled { pack_id, version } => {
                format!("Re-proving the installed copy of {pack_id} {version}…")
            }
        }
    }

    pub(super) fn description(&self) -> String {
        match self {
            Self::FetchSnapshot => "model-catalog refresh".to_owned(),
            Self::InstallPack {
                pack_id, version, ..
            } => format!("model-pack install of '{pack_id} {version}'"),
            Self::UpdatePack {
                pack_id, latest, ..
            } => format!("model-pack update of '{pack_id}' to {latest}"),
            Self::AdoptPart {
                pack_id,
                part_id,
                to_version,
                ..
            } => format!("adoption of '{pack_id}' part '{part_id}' at {to_version}"),
            Self::RemovePack { pack_id, version } => {
                format!("model-pack removal of '{pack_id} {version}'")
            }
            Self::VerifyInstalled { pack_id, version } => {
                format!("model-pack re-proof of '{pack_id} {version}'")
            }
        }
    }

    /// The pack this request puts on the machine, when it is one that does.
    ///
    /// Only the three requests that can move an archive answer: an install, an
    /// update, and an adoption that needs its newer release fetched first. A
    /// refresh moves the catalog rather than a pack; a removal and a re-proof
    /// read what is already here. The ledger lights this pack's row while the
    /// operation runs, so answering for any of the other three would put a
    /// transfer state on a row where nothing is transferring.
    pub(super) fn landing_pack(&self) -> Option<String> {
        match self {
            Self::InstallPack { pack_id, .. }
            | Self::UpdatePack { pack_id, .. }
            | Self::AdoptPart { pack_id, .. } => Some(pack_id.clone()),
            Self::FetchSnapshot | Self::RemovePack { .. } | Self::VerifyInstalled { .. } => None,
        }
    }

    /// The installed release this request re-proves, when it is a re-proof.
    pub(super) fn verification_key(&self) -> Option<String> {
        match self {
            Self::VerifyInstalled { pack_id, version } => Some(format!("{pack_id}@{version}")),
            _ => None,
        }
    }

    /// The reason line a published project revision records.
    fn revision_reason(&self, part: &str) -> String {
        match self {
            Self::InstallPack { pack_id, .. } => {
                format!("add pack part {part} from {pack_id}")
            }
            Self::AdoptPart {
                pack_id,
                from_version,
                to_version,
                ..
            } => format!("adopt pack part {part} from {pack_id} {from_version} at {to_version}"),
            _ => format!("add pack part {part}"),
        }
    }
}

/// What one completed Model Hub operation asks the session to publish.
///
/// The receipt is always present because every hub operation changes what
/// this machine holds. The part is present only when the operation was also
/// asked to retain one into the project, which is the shelf's single gesture.
#[derive(Debug)]
pub(super) struct ModelHubOutput {
    pub(super) receipt: String,
    pub(super) part: Option<ModelHubPartOutput>,
}

#[derive(Debug)]
pub(super) struct ModelHubPartOutput {
    candidate: Box<crate::state::model_library::ModelLibraryManager>,
    library_name: String,
    part_id: String,
    /// The symbol placement to arm afterwards, when the part arrived because
    /// somebody asked for one to place. Adoption has none: the part is already
    /// in the design, and arming it would offer a second copy of something the
    /// reader only asked to move forward a release.
    placement: Option<crate::state::model_hub::PartPlacement>,
}

/// Determinate transfer progress, shared between a worker and the frame loop.
///
/// A download is the only part of a hub operation whose length is known in
/// advance, and it is known from the *signed snapshot* rather than from the
/// service, so the fraction cannot be stretched by an overstated handoff.
#[derive(Debug, Default)]
pub(super) struct ModelHubProgress {
    received: std::sync::atomic::AtomicU64,
    total: std::sync::atomic::AtomicU64,
}

impl ModelHubProgress {
    fn record(&self, received: u64, total: u64) {
        use std::sync::atomic::Ordering::Relaxed;
        self.received.store(received, Relaxed);
        self.total.store(total, Relaxed);
    }

    /// The completed fraction, or `None` while nothing is transferring.
    pub(super) fn fraction(&self) -> Option<f32> {
        use std::sync::atomic::Ordering::Relaxed;
        let total = self.total.load(Relaxed);
        if total == 0 {
            return None;
        }
        Some((self.received.load(Relaxed) as f32 / total as f32).clamp(0.0, 1.0))
    }

    pub(super) fn clear(&self) {
        self.record(0, 0);
    }
}

/// The one progress cell a worker writes and the frame loop reads.
pub(super) fn model_hub_progress() -> &'static ModelHubProgress {
    static PROGRESS: std::sync::OnceLock<ModelHubProgress> = std::sync::OnceLock::new();
    PROGRESS.get_or_init(ModelHubProgress::default)
}

/// Human-readable byte count for a receipt.
fn byte_size(bytes: u64) -> String {
    const UNITS: [(&str, u64); 3] = [("MB", 1024 * 1024), ("kB", 1024), ("B", 1)];
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
    "0 B".to_owned()
}

/// Runs one Model Hub operation to completion against its own hub.
///
/// The worker opens a hub over the *session's store* rather than borrowing the
/// session's hub: two hubs over one store always agree, because the store is
/// where the state lives, and the session reloads from it when this returns.
/// Nothing here mutates the application.
#[cfg(not(target_arch = "wasm32"))]
pub(super) fn run_model_hub_operation(
    store: &crate::services::model_hub::ModelHubStoreHandle,
    candidate: crate::state::model_library::ModelLibraryManager,
    request: &ModelHubRequest,
    progress: &ModelHubProgress,
) -> Result<ModelHubOutput, String> {
    let mut hub = store.open().map_err(|error| error.to_string())?;
    // The runtime is bound here so it outlives the transport that drives it:
    // dropping it first would leave a client whose executor is already gone.
    let (_runtime, transport) = model_hub_transport(progress)?;
    execute(&mut hub, candidate, request, &transport)
}

/// Executes one request against an open hub and a ready transport.
///
/// Everything platform-specific — which runtime drives the transfer, whether
/// the bytes were fetched now or primed a moment ago — is settled before this
/// is called. What remains is the same sequence on every target, which is what
/// makes the browser and the desktop provably the same pipeline rather than
/// two implementations that happen to agree.
pub(super) fn execute(
    hub: &mut crate::state::model_hub::ModelHub,
    mut candidate: crate::state::model_library::ModelLibraryManager,
    request: &ModelHubRequest,
    transport: &dyn crate::state::model_hub::ModelHubTransport,
) -> Result<ModelHubOutput, String> {
    use crate::state::model_hub::PartState;

    let hub = &mut *hub;
    match request {
        ModelHubRequest::FetchSnapshot => {
            let generation = hub
                .refresh_catalog(transport)
                .map_err(|error| error.to_string())?;
            let packs = hub.snapshot().map_or(0, |snapshot| snapshot.packs.len());
            Ok(ModelHubOutput {
                receipt: format!(
                    "Model catalog generation {generation} verified against the release key; \
                     {packs} pack{} listed.",
                    if packs == 1 { "" } else { "s" }
                ),
                part: None,
            })
        }
        ModelHubRequest::InstallPack {
            pack_id,
            version,
            part,
        } => {
            let installed = ensure_installed(hub, pack_id, version, transport)?;
            let mut receipt = format!(
                "{} {version} verified end to end under the release key and installed ({}).",
                installed.manifest.pack.name,
                byte_size(catalog_archive_length(hub, pack_id, version))
            );
            let Some(part_id) = part.as_deref() else {
                return Ok(ModelHubOutput {
                    receipt,
                    part: None,
                });
            };
            let output = retain_part(hub, &mut candidate, pack_id, version, part_id)?;
            receipt.push_str(&format!(
                " '{part_id}' was retained into the project as library '{}'.",
                output.library_name
            ));
            Ok(ModelHubOutput {
                receipt,
                part: Some(output),
            })
        }
        ModelHubRequest::UpdatePack {
            pack_id,
            installed,
            latest,
        } => {
            // Newer first. A failed download leaves the installed release
            // exactly where it was, which is why removal cannot come first.
            let release = ensure_installed(hub, pack_id, latest, transport)?;
            let removed = hub
                .uninstall(pack_id, installed)
                .map_err(|error| error.to_string())?;
            Ok(ModelHubOutput {
                receipt: format!(
                    "{} updated to {latest}{}. Projects that already retained a part keep the \
                     exact bytes they were built against.",
                    release.manifest.pack.name,
                    if removed {
                        format!(" and the {installed} copy was removed")
                    } else {
                        format!(", and the {installed} copy was already gone")
                    }
                ),
                part: None,
            })
        }
        ModelHubRequest::AdoptPart {
            pack_id,
            from_version,
            to_version,
            part_id,
        } => {
            let installed = ensure_installed(hub, pack_id, to_version, transport)?;
            // The standing a fresh install leaves behind, demanded again before
            // anything in the project moves: still published, still runnable,
            // and still hashing to the digest the signed catalog carries. An
            // already-installed release is not re-downloaded, so without this
            // the one case adoption would skip is the one that matters.
            hub.adoptable(pack_id, to_version)
                .map_err(|error| error.to_string())?;
            let library_name = hub
                .add_part_to_project(&mut candidate, pack_id, to_version, part_id)
                .map_err(|error| error.to_string())?;
            Ok(ModelHubOutput {
                receipt: format!(
                    "'{part_id}' was re-retained from {} {to_version}, proved end to end under \
                     the release key, and its pin moved off {from_version}. Every other part this \
                     project pinned to {from_version} is untouched.",
                    installed.manifest.pack.name
                ),
                part: Some(ModelHubPartOutput {
                    candidate: Box::new(candidate),
                    library_name,
                    part_id: part_id.clone(),
                    placement: None,
                }),
            })
        }
        ModelHubRequest::VerifyInstalled { pack_id, version } => {
            let verified = hub
                .verify_installed(pack_id, version)
                .map_err(|error| error.to_string())?;
            Ok(ModelHubOutput {
                receipt: format!(
                    "{pack_id} {version} re-proved end to end under the release key; {} source \
                     file{} match the signed manifest.",
                    verified.files.len(),
                    if verified.files.len() == 1 { "" } else { "s" }
                ),
                part: None,
            })
        }
        ModelHubRequest::RemovePack { pack_id, version } => {
            let removed = hub
                .uninstall(pack_id, version)
                .map_err(|error| error.to_string())?;
            if !removed {
                return Err(format!("{pack_id} {version} was already not installed."));
            }
            // Whether the catalog can offer it again is what the row will say
            // next, so the receipt says it now rather than leaving a hole.
            let reinstallable = hub.part_index(&[]).into_iter().any(|row| {
                row.pack_id() == Some(pack_id.as_str()) && row.state != PartState::Installed
            });
            Ok(ModelHubOutput {
                receipt: format!(
                    "{pack_id} {version} was removed from this machine{}. Projects keep the bytes \
                     they retained.",
                    if reinstallable {
                        " and the catalog still offers it"
                    } else {
                        ""
                    }
                ),
                part: None,
            })
        }
    }
}

/// Installs a release unless this machine already holds exactly it.
///
/// An installed release is immutable, so re-downloading one would move bytes
/// to reach a state that is already reached. Skipping it is what makes "add
/// this part" one gesture whether or not its pack happens to be installed.
fn ensure_installed(
    hub: &mut crate::state::model_hub::ModelHub,
    pack_id: &str,
    version: &str,
    transport: &dyn crate::state::model_hub::ModelHubTransport,
) -> Result<crate::state::model_hub::InstalledPack, String> {
    if let Some(installed) = hub
        .installed()
        .iter()
        .find(|pack| pack.pack_id() == pack_id && pack.version() == version)
    {
        return Ok(installed.clone());
    }
    let installed = hub
        .install(transport, pack_id, version)
        .map_err(|error| error.to_string());
    model_hub_progress().clear();
    installed
}

/// The archive length the signed catalog publishes for one release.
///
/// The single authority on how many bytes a release weighs: the receipt quotes
/// it, and a browser transfer divides by it. Reading it from the snapshot
/// rather than from a download handoff is what makes a progress bar a fact
/// about the release rather than about whatever the service claimed this time.
/// Zero when this session holds no snapshot describing that release.
fn catalog_archive_length(
    hub: &crate::state::model_hub::ModelHub,
    pack_id: &str,
    version: &str,
) -> u64 {
    hub.snapshot()
        .and_then(|snapshot| snapshot.packs.iter().find(|pack| pack.id == pack_id))
        .and_then(|pack| {
            pack.releases
                .iter()
                .find(|release| release.version == version)
        })
        .map_or(0, |release| release.archive_length)
}

/// Retains one installed part into a project candidate and plans its symbol.
///
/// The order matters: the bytes are retained first, so a part whose symbol
/// reference this build cannot resolve fails *before* the project is changed
/// — the candidate is discarded and nothing is published.
fn retain_part(
    hub: &crate::state::model_hub::ModelHub,
    candidate: &mut crate::state::model_library::ModelLibraryManager,
    pack_id: &str,
    version: &str,
    part_id: &str,
) -> Result<ModelHubPartOutput, String> {
    let manifest_part = hub
        .installed()
        .iter()
        .find(|pack| pack.pack_id() == pack_id && pack.version() == version)
        .and_then(|pack| {
            pack.manifest
                .parts
                .iter()
                .find(|part| {
                    part.id == part_id || part.aliases.iter().any(|alias| alias == part_id)
                })
                .cloned()
        })
        .ok_or_else(|| format!("{pack_id} {version} publishes no part named '{part_id}'."))?;

    let library_name = hub
        .add_part_to_project(candidate, pack_id, version, part_id)
        .map_err(|error| error.to_string())?;
    let source = candidate
        .get_library(&library_name)
        .and_then(|library| library.root_path.clone());
    let placement = crate::state::model_hub::plan_part_placement(
        &manifest_part,
        &library_name,
        source.as_deref(),
    )?;
    Ok(ModelHubPartOutput {
        candidate: Box::new(candidate.clone()),
        library_name,
        part_id: manifest_part.id,
        placement: Some(placement),
    })
}

/// The transport this platform drives the hub through.
///
/// It is built per operation because a download capability is short lived and
/// because a session that never installs a pack should never have opened a
/// network client at all.
#[cfg(not(target_arch = "wasm32"))]
fn model_hub_transport(
    progress: &ModelHubProgress,
) -> Result<
    (
        tokio::runtime::Runtime,
        crate::state::model_hub::CloudModelHubTransport,
    ),
    String,
> {
    // The hub pipeline is synchronous, so it needs a runtime whose own threads
    // drive IO — a current-thread runtime cannot be driven through a `Handle`
    // from outside, which is exactly how this worker uses it.
    let runtime = tokio::runtime::Builder::new_multi_thread()
        .worker_threads(1)
        .enable_all()
        .build()
        .map_err(|error| format!("The model-hub transfer runtime could not start: {error}"))?;
    let client = crate::services::cloud_account::CloudAccountService::unauthenticated_client()?;
    let handle = runtime.handle().clone();
    progress.clear();
    let transport = crate::state::model_hub::CloudModelHubTransport::new(client, handle)
        .with_progress(std::sync::Arc::new(|received, total| {
            model_hub_progress().record(received, total);
        }));
    Ok((runtime, transport))
}

/// Everything the browser has to fetch before the pipeline can run.
///
/// It is derived from the request and the hub's current state rather than
/// fetched unconditionally: an install whose pack is already present needs no
/// archive, and a refresh needs no archive at all. Naming the set up front is
/// what lets one `await` sequence serve every request shape.
#[cfg(target_arch = "wasm32")]
fn priming_plan(
    hub: &crate::state::model_hub::ModelHub,
    request: &ModelHubRequest,
) -> (bool, Option<(String, String)>) {
    let held = |pack_id: &str, version: &str| {
        hub.installed()
            .iter()
            .any(|pack| pack.pack_id() == pack_id && pack.version() == version)
    };
    match request {
        ModelHubRequest::FetchSnapshot => (true, None),
        ModelHubRequest::InstallPack {
            pack_id, version, ..
        } => (
            hub.snapshot().is_none(),
            (!held(pack_id, version)).then(|| (pack_id.clone(), version.clone())),
        ),
        ModelHubRequest::UpdatePack {
            pack_id, latest, ..
        } => (
            hub.snapshot().is_none(),
            (!held(pack_id, latest)).then(|| (pack_id.clone(), latest.clone())),
        ),
        ModelHubRequest::AdoptPart {
            pack_id,
            to_version,
            ..
        } => (
            hub.snapshot().is_none(),
            (!held(pack_id, to_version)).then(|| (pack_id.clone(), to_version.clone())),
        ),
        // Both read only what this machine already holds.
        ModelHubRequest::RemovePack { .. } | ModelHubRequest::VerifyInstalled { .. } => {
            (false, None)
        }
    }
}

/// Runs one Model Hub operation in the browser.
///
/// The fetches happen here, awaited, and the pipeline then runs synchronously
/// over bytes that have already arrived — the same [`execute`] the desktop
/// runs, with the same digest checks in the same order.
///
/// The archive fetch reports into the same progress cell the desktop worker
/// writes, against the length the held snapshot publishes. Without it a
/// browser install was a button press followed by a page that did not move
/// until the whole archive had landed, which for a megabyte on a slow
/// connection is a workspace that looks broken.
#[cfg(target_arch = "wasm32")]
pub(super) async fn run_browser_model_hub_operation(
    store: &crate::services::model_hub::ModelHubStoreHandle,
    candidate: crate::state::model_library::ModelLibraryManager,
    request: &ModelHubRequest,
) -> Result<ModelHubOutput, String> {
    let mut hub = store.open().map_err(|error| error.to_string())?;
    let (catalog, archive) = priming_plan(&hub, request);
    let transport = if catalog || archive.is_some() {
        let client = crate::services::cloud_account::CloudAccountService::unauthenticated_client()?;
        let transport = crate::state::model_hub::BrowserModelHubTransport::new(client)
            .with_progress(std::rc::Rc::new(|received, declared| {
                model_hub_progress().record(received, declared);
            }));
        if catalog {
            transport
                .prime_catalog()
                .await
                .map_err(|error| error.to_string())?;
        }
        if let Some((pack_id, version)) = archive.as_ref() {
            // Read before the fetch and from the *snapshot*, so the
            // denominator is the catalog's number rather than the download's
            // claim about itself. A session that had to fetch the catalog in
            // this same operation has no snapshot to read yet and reports an
            // indeterminate transfer instead of a made-up fraction.
            let declared = catalog_archive_length(&hub, pack_id, version);
            transport
                .prime_archive(pack_id, version, declared)
                .await
                .map_err(|error| error.to_string())?;
        }
        Some(transport)
    } else {
        None
    };
    // A request that primed nothing needs nothing: removal reads the store and
    // an already-installed release is already proved. The offline transport
    // makes that explicit rather than leaving a client open with no work.
    match transport {
        Some(transport) => execute(&mut hub, candidate, request, &transport),
        None => execute(
            &mut hub,
            candidate,
            request,
            &crate::state::model_hub::OfflineTransport,
        ),
    }
}

/// Publishes one completed Model Hub operation into the session.
///
/// The order is the argument. The session's hub is reloaded first, so every
/// surface already agrees about what is installed before a receipt claims it;
/// the project revision is published second, because a retained part must
/// exist before a placement can be armed against it; the placement is armed
/// last, and only if the revision was actually published. A failure at any
/// step leaves the earlier ones standing — they describe real state — and says
/// which step failed.
pub(super) fn publish_model_hub_output(
    ctx: &Context,
    state: &mut AppState,
    model_hub: &mut crate::services::model_hub::ModelHubService,
    request: &ModelHubRequest,
    output: ModelHubOutput,
) {
    if let Err(error) = model_hub.reload() {
        emit_model_hub_errors(
            ctx,
            state,
            vec![format!(
                "The operation completed but RSpice could not re-read its pack store: {error}"
            )],
        );
        return;
    }
    note_pack_verification(state, request, Ok(()));
    let ModelHubOutput { receipt, part } = output;
    let Some(part) = part else {
        apply_model_hub_receipt(ctx, state, receipt);
        return;
    };
    let ModelHubPartOutput {
        candidate,
        library_name,
        part_id,
        placement,
    } = part;
    if state.project_lifecycle.project_open {
        let published = publish_model_library_candidate(
            state,
            *candidate,
            &library_name,
            request.revision_reason(&part_id),
        );
        if let Err(error) = published {
            emit_model_hub_errors(ctx, state, vec![error]);
            return;
        }
    } else {
        state.model_library_manager = *candidate;
    }
    // A part that arrived to be placed is armed and the canvas takes focus; a
    // part that only moved onto a newer release did neither, and pulling the
    // reader into the schematic would answer a question nobody asked.
    let Some(placement) = placement else {
        apply_model_hub_receipt(ctx, state, receipt);
        return;
    };
    let armed = state.schematic.arm_pack_part(placement);
    crate::schematic::view::request_schematic_canvas_focus(ctx);
    apply_model_hub_receipt(
        ctx,
        state,
        format!("{receipt} {armed} is armed for placement."),
    );
}

/// Records what one re-proof concluded, when the operation was one.
///
/// The pack table's word about a release comes from here, so it is written at
/// the one place that holds both the request and its outcome: a receipt alone
/// knows what happened and not which release it happened to.
pub(super) fn note_pack_verification(
    state: &mut AppState,
    request: &ModelHubRequest,
    outcome: Result<(), String>,
) {
    let Some(key) = request.verification_key() else {
        return;
    };
    state.workbench.models_view.pack_verification.insert(
        key,
        match outcome {
            Ok(()) => crate::workbench::state::PackReProof::Verified,
            Err(reason) => crate::workbench::state::PackReProof::Failed(reason),
        },
    );
}

/// Records a successful hub receipt everywhere a receipt is read.
fn apply_model_hub_receipt(ctx: &Context, state: &mut AppState, receipt: String) {
    state.workbench.models_view.operational_state = ModelsOperationalState::Ready;
    state.workbench.models_view.action_receipt = Some(Ok(receipt.clone()));
    state.push_user_message(ConsoleMessage::info(receipt.clone()));
    state.ui.toasts.success(ctx, "Model hub", receipt);
}

/// Records a failed hub operation. Nothing partial is left behind: every step
/// that can fail either completed or changed nothing at all.
pub(super) fn emit_model_hub_errors(ctx: &Context, state: &mut AppState, errors: Vec<String>) {
    let first = errors
        .first()
        .cloned()
        .unwrap_or_else(|| "The model-hub operation failed without a diagnostic.".to_owned());
    for error in &errors {
        state.push_user_message(ConsoleMessage::error(error.clone()));
    }
    // Release the once-per-session stale-catalog latch. It exists to stop the
    // automatic refresh firing every frame the workspace is visible, not to
    // spend a session's only attempt on a machine that happened to be offline
    // when the workspace first opened: a later visit must be allowed to retry.
    state.workbench.models_view.catalog_refresh_requested = false;
    state.workbench.models_view.operational_state = ModelsOperationalState::from_failure(&first);
    state.workbench.models_view.action_receipt = Some(Err(errors.join("; ")));
    state
        .ui
        .toasts
        .error_with_title(ctx, "Model hub operation failed", first);
}
