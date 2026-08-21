//! PDK setup workflow.

mod model_hub;

pub(in crate::workbench) use model_hub::ModelHubRequest;
#[cfg(not(target_arch = "wasm32"))]
use model_hub::{ModelHubOutput, ModelHubProgress, run_model_hub_operation};
use model_hub::{model_hub_progress, publish_model_hub_output};

use std::path::Path;

#[cfg(not(target_arch = "wasm32"))]
use std::collections::VecDeque;
#[cfg(not(target_arch = "wasm32"))]
use std::sync::{Mutex, OnceLock};

use egui::Context;

use crate::diagnostics::ConsoleMessage;
use crate::workbench::app::RSpiceApp;
use crate::workbench::app_state::AppState;
use crate::workbench::app_state::design_history::{
    publish_model_library_candidate, publish_model_library_set_candidate,
};
use crate::workbench::app_state::session::pdk_settings::PdkSettingsDialogResult;
use crate::workbench::state::{ModelsAttemptedOperation, ModelsOperationalState};

/// Records what the workspace is attempting, so a later receipt can name it.
///
/// Every long-running model-source operation passes through one of the four
/// starts below, which is why this is recorded there rather than at the
/// twenty-odd places that write a receipt: a receipt knows its outcome and
/// nothing about its cause.
///
/// `landing_pack` names the pack the operation puts on this machine, when
/// there is one. It is taken here for the same reason the label is: this is
/// the one place that still holds the request, and the ledger's in-flight cell
/// otherwise had nothing but a formatted progress sentence to recover a pack
/// identifier from.
fn note_models_operation(
    state: &mut AppState,
    label: String,
    reissuable: bool,
    landing_pack: Option<String>,
) {
    state.workbench.models_view.attempted_operation = Some(ModelsAttemptedOperation {
        label,
        reissuable,
        landing_pack,
    });
}

#[cfg(target_arch = "wasm32")]
struct BrowserModelImport {
    authority: BrowserModelImportAuthority,
    result: Result<Option<BrowserParsedModelImport>, String>,
    root_candidates: Option<Vec<String>>,
}

#[cfg(target_arch = "wasm32")]
struct BrowserPendingModelRoot {
    authority: BrowserModelImportAuthority,
    display_name: String,
    files: Vec<(String, Vec<u8>)>,
    candidates: Vec<String>,
}

#[cfg(target_arch = "wasm32")]
#[derive(Debug, Clone)]
struct BrowserModelImportAuthority {
    project_id: Option<String>,
    project_revision: u64,
    catalog_digest: crate::product::ContentDigest,
    replace_library: Option<String>,
}

#[cfg(target_arch = "wasm32")]
struct BrowserParsedModelImport {
    display_name: String,
    file_count: usize,
    library: crate::state::model_library::ModelLibrary,
}

#[cfg(target_arch = "wasm32")]
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
#[serde(deny_unknown_fields)]
struct BrowserModelImportWorkerMetadata {
    protocol_version: u16,
    display_name: String,
    root_name: String,
    file_names: Vec<String>,
}

#[cfg(target_arch = "wasm32")]
const BROWSER_MODEL_IMPORT_PROTOCOL_VERSION: u16 = 4;

#[cfg(not(target_arch = "wasm32"))]
#[derive(Debug, Clone)]
struct NativeModelImportAuthority {
    project_id: Option<String>,
    project_revision: u64,
    catalog_digest: crate::product::ContentDigest,
}

#[cfg(not(target_arch = "wasm32"))]
#[derive(Debug, Clone)]
struct NativePdkConfigurationOperation {
    previous: crate::state::pdk_config::PdkConfig,
    config: crate::state::pdk_config::PdkConfig,
    technology_binding: Option<crate::state::workspace::ProjectTechnologyBinding>,
}

#[cfg(not(target_arch = "wasm32"))]
#[derive(Debug, Clone)]
enum NativeModelCatalogOperation {
    ImportFile {
        path: std::path::PathBuf,
    },
    RefreshLibrary {
        library: String,
        root: std::path::PathBuf,
        section: Option<String>,
        pack_id: Option<String>,
    },
    AttachPack {
        pack_id: String,
    },
    AddPart {
        pack_id: String,
        part_name: String,
    },
    ApplyConfiguration(Box<NativePdkConfigurationOperation>),
    ModelHub(ModelHubRequest),
}

#[cfg(not(target_arch = "wasm32"))]
impl NativeModelCatalogOperation {
    fn progress_label(&self) -> String {
        match self {
            Self::ImportFile { path } => format!(
                "Authenticating and parsing '{}' in the background…",
                path.file_name()
                    .and_then(|name| name.to_str())
                    .unwrap_or("model source")
            ),
            Self::RefreshLibrary { library, .. } => {
                format!("Refreshing and authenticating model library '{library}'…")
            }
            Self::AttachPack { pack_id } => {
                format!("Authenticating and attaching model pack '{pack_id}'…")
            }
            Self::AddPart { part_name, .. } => {
                format!("Authenticating and adding model part '{part_name}'…")
            }
            Self::ApplyConfiguration(_) => {
                "Discovering and authenticating configured PDK model sources…".to_owned()
            }
            Self::ModelHub(request) => request.progress_label(),
        }
    }

    fn description(&self) -> String {
        match self {
            Self::ImportFile { path } => format!("model import from '{}'", path.display()),
            Self::RefreshLibrary { library, .. } => {
                format!("model-library refresh for '{library}'")
            }
            Self::AttachPack { pack_id } => format!("model-pack attach for '{pack_id}'"),
            Self::AddPart { part_name, pack_id } => {
                format!("model-part add for '{part_name}' from '{pack_id}'")
            }
            Self::ApplyConfiguration(_) => "PDK model-source configuration update".to_owned(),
            Self::ModelHub(request) => request.description(),
        }
    }

    /// The pack this operation puts on the machine, when it is one that does.
    ///
    /// Only a hub request can be one. Attaching a shipped-corpus pack names a
    /// pack too, but it copies a source tree this machine already has rather
    /// than fetching a signed archive, so nothing about it is a transfer to
    /// report.
    fn landing_pack(&self) -> Option<String> {
        match self {
            Self::ModelHub(request) => request.landing_pack(),
            Self::ImportFile { .. }
            | Self::RefreshLibrary { .. }
            | Self::AttachPack { .. }
            | Self::AddPart { .. }
            | Self::ApplyConfiguration(_) => None,
        }
    }
}

#[cfg(not(target_arch = "wasm32"))]
struct NativePdkConfigurationOutput {
    previous: crate::state::pdk_config::PdkConfig,
    config: crate::state::pdk_config::PdkConfig,
    candidate: crate::state::model_library::ModelLibraryManager,
    loaded: usize,
}

#[cfg(not(target_arch = "wasm32"))]
enum NativeModelCatalogOutput {
    Library {
        candidate: Box<crate::state::model_library::ModelLibraryManager>,
        library_name: String,
        library_stats: Option<(usize, usize)>,
    },
    Configuration(Box<NativePdkConfigurationOutput>),
    ModelHub(Box<ModelHubOutput>),
}
#[cfg(not(target_arch = "wasm32"))]
struct NativeModelImport {
    authority: NativeModelImportAuthority,
    operation: NativeModelCatalogOperation,
    result: Result<NativeModelCatalogOutput, Vec<String>>,
}

#[cfg(not(target_arch = "wasm32"))]
static NATIVE_MODEL_IMPORTS: OnceLock<Mutex<VecDeque<NativeModelImport>>> = OnceLock::new();

#[cfg(not(target_arch = "wasm32"))]
fn native_model_imports() -> &'static Mutex<VecDeque<NativeModelImport>> {
    NATIVE_MODEL_IMPORTS.get_or_init(|| Mutex::new(VecDeque::new()))
}

#[cfg(target_arch = "wasm32")]
thread_local! {
    static BROWSER_MODEL_IMPORTS: std::cell::RefCell<std::collections::VecDeque<BrowserModelImport>> =
        const { std::cell::RefCell::new(std::collections::VecDeque::new()) };
    static BROWSER_PENDING_MODEL_ROOT: std::cell::RefCell<Option<BrowserPendingModelRoot>> =
        const { std::cell::RefCell::new(None) };
}

#[cfg(any(test, target_arch = "wasm32"))]
fn prepare_pdk_configuration(
    state: &AppState,
    config: crate::state::pdk_config::PdkConfig,
) -> Result<
    (
        crate::state::pdk_config::PdkConfig,
        crate::state::model_library::ModelLibraryManager,
        usize,
    ),
    Vec<String>,
> {
    if state.project_lifecycle.project_open && state.workbench.safe_mode.project_read_only() {
        return Err(vec![
            "PDK settings cannot change while the project is read-only.".to_owned(),
        ]);
    }
    prepare_pdk_configuration_candidate(
        state.model_library_manager.clone(),
        &state.pdk_config,
        state.workspace.project.technology_binding(),
        config,
    )
}

fn prepare_pdk_configuration_candidate(
    mut candidate: crate::state::model_library::ModelLibraryManager,
    previous: &crate::state::pdk_config::PdkConfig,
    technology_binding: Option<&crate::state::workspace::ProjectTechnologyBinding>,
    mut config: crate::state::pdk_config::PdkConfig,
) -> Result<
    (
        crate::state::pdk_config::PdkConfig,
        crate::state::model_library::ModelLibraryManager,
        usize,
    ),
    Vec<String>,
> {
    let loaded = candidate.replace_from_pdk_config(Some(previous), &mut config)?;
    candidate
        .validate_attached_technology(technology_binding)
        .map_err(|error| {
            vec![format!(
                "PDK settings would invalidate the attached project technology: {error}"
            )]
        })?;
    Ok((config, candidate, loaded))
}

#[cfg(test)]
fn apply_pdk_configuration_with_persistence(
    state: &mut AppState,
    config: crate::state::pdk_config::PdkConfig,
    persist: impl FnMut(&crate::state::pdk_config::PdkConfig) -> Result<(), String>,
) -> Result<usize, Vec<String>> {
    let previous = state.pdk_config.clone();
    let prepared = prepare_pdk_configuration(state, config)?;
    publish_prepared_pdk_configuration_with_persistence(state, previous, prepared, persist)
}

fn publish_prepared_pdk_configuration_with_persistence(
    state: &mut AppState,
    previous: crate::state::pdk_config::PdkConfig,
    prepared: (
        crate::state::pdk_config::PdkConfig,
        crate::state::model_library::ModelLibraryManager,
        usize,
    ),
    mut persist: impl FnMut(&crate::state::pdk_config::PdkConfig) -> Result<(), String>,
) -> Result<usize, Vec<String>> {
    let (config, candidate, loaded) = prepared;
    persist(&config).map_err(|error| {
        vec![format!(
            "PDK settings were not applied because the configuration could not be persisted: {error}"
        )]
    })?;

    let publication = if state.project_lifecycle.project_open {
        publish_model_library_set_candidate(state, candidate, "apply configured PDK model sources")
            .map(|_| ())
    } else {
        state.model_library_manager = candidate;
        Ok(())
    };
    if let Err(error) = publication {
        let rollback = persist(&previous).err();
        let mut errors = vec![error];
        if let Some(rollback) = rollback {
            errors.push(format!(
                "PDK configuration rollback also failed; verify the persisted settings before restarting: {rollback}"
            ));
        }
        return Err(errors);
    }

    state.pdk_config = config;
    Ok(loaded)
}

fn emit_pdk_save_warning(state: &mut AppState, operation: &str, error: impl AsRef<str>) {
    state.push_user_message(ConsoleMessage::warning(format!(
        "PDK configuration {} but could not be persisted: {}",
        operation,
        error.as_ref()
    )));
}

fn emit_pdk_apply_messages(state: &mut AppState, load_result: Result<usize, Vec<String>>) {
    match load_result {
        Ok(count) => {
            state.push_user_message(ConsoleMessage::info(format!(
                "PDK settings applied: {} libraries loaded",
                count
            )));
        }
        Err(errors) => {
            state.push_user_message(ConsoleMessage::warning(format!(
                "PDK settings were not applied; no configured model libraries changed ({} errors)",
                errors.len()
            )));
            for error in errors {
                state.push_user_message(ConsoleMessage::error(error));
            }
        }
    }
}

fn emit_pdk_file_load_success_message(
    state: &mut AppState,
    path: &Path,
    library_name: &str,
    library_stats: Option<(usize, usize)>,
) {
    state.pdk_config.add_recent_file(path);
    if let Err(err) = state.pdk_config.save() {
        emit_pdk_save_warning(state, "was updated", err.to_string());
    }

    state.push_user_message(ConsoleMessage::info(format!(
        "Loaded library '{}' from {}",
        library_name,
        path.display()
    )));

    if let Some((model_count, corner_count)) = library_stats {
        state.push_user_message(ConsoleMessage::info(format!(
            "  {} models, {} corners available",
            model_count, corner_count
        )));
    }
}

impl RSpiceApp {
    pub(in crate::workbench) fn start_model_source_import(&mut self, _ctx: &Context) {
        if self.state.workbench.models_view.model_import_in_progress {
            self.state.push_user_message(ConsoleMessage::warning(
                "A model-source import is already authenticating and parsing.".to_owned(),
            ));
            return;
        }
        #[cfg(target_arch = "wasm32")]
        {
            note_models_operation(&mut self.state, "model import".to_owned(), false, None);
            self.state.workbench.models_view.model_import_in_progress = true;
            self.state.workbench.models_view.model_import_label =
                Some("Selecting and reading browser model sources…".to_owned());
            if let Err(error) = start_browser_model_import(_ctx, &self.state, None) {
                self.state.workbench.models_view.model_import_in_progress = false;
                self.state.workbench.models_view.model_import_label = None;
                self.state.push_user_message(ConsoleMessage::error(error));
            }
        }
        #[cfg(not(target_arch = "wasm32"))]
        {
            let picked = rfd::FileDialog::new()
                .add_filter(
                    "SPICE model source",
                    &["lib", "model", "mod", "spice", "cir", "inc", "scs"],
                )
                .pick_file();
            if let Some(path) = picked {
                self.start_native_model_source_parse(_ctx, path);
            }
        }
    }

    #[cfg(target_arch = "wasm32")]
    pub(in crate::workbench) fn select_browser_model_import_root(
        &mut self,
        ctx: &Context,
        root_name: String,
    ) {
        let pending = BROWSER_PENDING_MODEL_ROOT.with(|pending| pending.borrow_mut().take());
        let Some(pending) = pending else {
            self.state.push_user_message(ConsoleMessage::error(
                "The pending browser model source selection is no longer available.".to_owned(),
            ));
            self.state.workbench.models_view.dialog = None;
            return;
        };
        if !pending
            .candidates
            .iter()
            .any(|candidate| candidate == &root_name)
        {
            self.state.push_user_message(ConsoleMessage::error(format!(
                "Browser model source '{root_name}' is not an available entry file."
            )));
            BROWSER_PENDING_MODEL_ROOT.with(|slot| *slot.borrow_mut() = Some(pending));
            return;
        }
        let metadata = BrowserModelImportWorkerMetadata {
            protocol_version: BROWSER_MODEL_IMPORT_PROTOCOL_VERSION,
            display_name: pending.display_name.clone(),
            root_name: root_name.clone(),
            file_names: pending.files.iter().map(|(name, _)| name.clone()).collect(),
        };
        let authority = pending.authority.clone();
        let buffers = pending
            .files
            .iter()
            .map(|(_, bytes)| bytes.clone())
            .collect();
        if let Err(error) =
            browser_model_import_worker::start(metadata, buffers, authority, ctx.clone())
        {
            BROWSER_PENDING_MODEL_ROOT.with(|slot| *slot.borrow_mut() = Some(pending));
            self.state.push_user_message(ConsoleMessage::error(error));
            return;
        }
        self.state.workbench.models_view.dialog = None;
        self.state.workbench.models_view.model_import_in_progress = true;
        self.state.workbench.models_view.model_import_label = Some(format!(
            "Authenticating model source tree from '{root_name}'..."
        ));
    }

    #[cfg(target_arch = "wasm32")]
    pub(in crate::workbench) fn cancel_browser_model_import_root(&mut self) {
        BROWSER_PENDING_MODEL_ROOT.with(|pending| {
            pending.borrow_mut().take();
        });
        self.state.workbench.models_view.dialog = None;
        self.state.workbench.models_view.operational_state = ModelsOperationalState::Cancelled;
        self.state.workbench.models_view.action_receipt = Some(Ok(
            "Browser model-source selection was cancelled.".to_owned(),
        ));
    }

    #[cfg(not(target_arch = "wasm32"))]
    fn start_native_model_source_parse(&mut self, ctx: &Context, path: std::path::PathBuf) {
        self.start_native_model_catalog_operation(
            ctx,
            NativeModelCatalogOperation::ImportFile { path },
        );
    }

    pub(in crate::workbench) fn start_model_library_refresh(
        &mut self,
        ctx: &Context,
        library_name: String,
    ) {
        let Some(library) = self
            .state
            .model_library_manager
            .get_library(&library_name)
            .cloned()
        else {
            self.state.push_user_message(ConsoleMessage::error(format!(
                "Model library '{library_name}' no longer exists."
            )));
            return;
        };
        let Some(root) = library.root_path.clone() else {
            self.state.push_user_message(ConsoleMessage::error(format!(
                "Model library '{library_name}' has no refreshable source root."
            )));
            return;
        };
        #[cfg(not(target_arch = "wasm32"))]
        self.start_native_model_catalog_operation(
            ctx,
            NativeModelCatalogOperation::RefreshLibrary {
                library: library_name,
                root,
                section: library.selected_corner,
                pack_id: library.pack_id,
            },
        );
        #[cfg(target_arch = "wasm32")]
        {
            if self.state.workbench.models_view.model_import_in_progress {
                self.state.push_user_message(ConsoleMessage::warning(
                    "A model-source operation is already authenticating and parsing.".to_owned(),
                ));
                return;
            }
            note_models_operation(
                &mut self.state,
                format!("model-library refresh for '{library_name}'"),
                false,
                None,
            );
            self.state.workbench.models_view.model_import_in_progress = true;
            self.state.workbench.models_view.model_import_label = Some(format!(
                "Selecting replacement sources for '{library_name}'…"
            ));
            if let Err(error) = start_browser_model_import(ctx, &self.state, Some(library_name)) {
                self.state.workbench.models_view.model_import_in_progress = false;
                self.state.workbench.models_view.model_import_label = None;
                self.state.push_user_message(ConsoleMessage::error(error));
            }
            let _ = root;
        }
    }

    /// Attaches one pack of the shipped model tree to this project.
    ///
    /// That tree is a directory of source files shipped beside the desktop
    /// application, so a browser session has none and the operation refuses
    /// rather than doing something else. It used to open a file picker: a
    /// control labelled "Attach pack" that instead asked the reader to go and
    /// find a model source, which is a different operation with a different
    /// outcome and a different provenance.
    pub(in crate::workbench) fn start_model_pack_attach(&mut self, ctx: &Context, pack_id: String) {
        #[cfg(not(target_arch = "wasm32"))]
        self.start_native_model_catalog_operation(
            ctx,
            NativeModelCatalogOperation::AttachPack { pack_id },
        );
        #[cfg(target_arch = "wasm32")]
        {
            let _ = ctx;
            self.refuse_shipped_tree_operation(
                format!("model-pack attach for '{pack_id}'"),
                format!(
                    "The shipped model tree is a directory beside the desktop application, and a \
                     browser session has no copy of it, so '{pack_id}' cannot be attached from \
                     one. Signed packs from the Model Hub install here as normal."
                ),
            );
        }
    }

    /// Retains one part of the shipped model tree into this project.
    ///
    /// Refused in a browser session for the same reason, worded for the part
    /// rather than the pack because that is what the reader asked for. It used
    /// to open the same file picker behind a console line promising the
    /// selection would be retained "as a generic project import" — which is
    /// neither the part that was asked for nor an import anybody chose.
    pub(in crate::workbench) fn start_model_part_add(
        &mut self,
        ctx: &Context,
        pack_id: String,
        part_name: String,
    ) {
        #[cfg(not(target_arch = "wasm32"))]
        self.start_native_model_catalog_operation(
            ctx,
            NativeModelCatalogOperation::AddPart { pack_id, part_name },
        );
        #[cfg(target_arch = "wasm32")]
        {
            let _ = ctx;
            self.refuse_shipped_tree_operation(
                format!("model-part add for '{part_name}' from '{pack_id}'"),
                format!(
                    "The shipped model tree is a directory beside the desktop application, and a \
                     browser session has no copy of it, so '{part_name}' cannot be added from \
                     one. Signed packs from the Model Hub retain their parts here as normal."
                ),
            );
        }
    }

    /// Records a shipped-tree operation this host cannot perform.
    ///
    /// It ends in the same three places every other refusal does — the
    /// attempted-operation record, the receipt the exception banner reads, and
    /// the console — because a refusal arriving by a private route would be a
    /// second failure vocabulary for one platform.
    #[cfg(target_arch = "wasm32")]
    fn refuse_shipped_tree_operation(&mut self, attempted: String, reason: String) {
        note_models_operation(&mut self.state, attempted, false, None);
        self.state.workbench.models_view.operational_state =
            ModelsOperationalState::from_failure(&reason);
        self.state.workbench.models_view.action_receipt = Some(Err(reason.clone()));
        self.state
            .push_user_message(ConsoleMessage::warning(reason));
    }

    #[cfg(not(target_arch = "wasm32"))]
    fn start_native_pdk_configuration_apply(
        &mut self,
        ctx: &Context,
        config: crate::state::pdk_config::PdkConfig,
    ) {
        if self.state.project_lifecycle.project_open
            && self.state.workbench.safe_mode.project_read_only()
        {
            emit_pdk_apply_messages(
                &mut self.state,
                Err(vec![
                    "PDK settings cannot change while the project is read-only.".to_owned(),
                ]),
            );
            return;
        }
        self.start_native_model_catalog_operation(
            ctx,
            NativeModelCatalogOperation::ApplyConfiguration(Box::new(
                NativePdkConfigurationOperation {
                    previous: self.state.pdk_config.clone(),
                    config,
                    technology_binding: self.state.workspace.project.technology_binding().cloned(),
                },
            )),
        );
    }

    /// Drains completed model-catalog work and republishes live progress.
    ///
    /// It is called from the frame loop rather than from any one surface,
    /// because a hub install started from the component shelf must finish even
    /// if the user has since walked away from the Models workspace.
    pub(in crate::workbench) fn pump_model_catalog_operations(&mut self, ctx: &Context) {
        #[cfg(not(target_arch = "wasm32"))]
        poll_native_model_imports(ctx, &mut self.state, &mut self.model_hub);
        #[cfg(target_arch = "wasm32")]
        {
            poll_browser_model_imports(ctx, &mut self.state);
            poll_browser_model_hub_operations(ctx, &mut self.state, &mut self.model_hub);
        }
        self.state.workbench.models_view.model_import_progress = self
            .state
            .workbench
            .models_view
            .model_import_in_progress
            .then(|| model_hub_progress().fraction())
            .flatten();
        self.report_recalled_pins(ctx);
    }

    /// Tells the reader when this project is pinned to a recalled release.
    ///
    /// It runs from the frame loop rather than from the Models workspace for
    /// the reason the workspace itself is not the audience: a project is
    /// opened into the Design surface, and a recall a reader only learns by
    /// walking to another workspace is a recall they learn after the work is
    /// done. Opening a project moves the pin half of the scope, so the first
    /// frame after the open is where this fires.
    ///
    /// It reports and refuses nothing. Retained bytes are untouched, every
    /// library still resolves, and a design pinned to a recalled release
    /// solves exactly as it did — which is the whole of decision D-E, and the
    /// reason this is a warning and not a blocked project.
    fn report_recalled_pins(&mut self, ctx: &Context) {
        use crate::workbench::state::RecallNoticeScope;

        let Some(identity) = self
            .model_hub
            .hub()
            .and_then(crate::state::model_hub::ModelHub::catalog_identity)
        else {
            // No catalog is no news. The latch is deliberately left alone, so
            // the first refresh of the session reports against a real one.
            return;
        };
        let scope = RecallNoticeScope {
            catalog_digest: identity.digest.clone(),
            pack_pin_key: crate::state::model_hub::pack_pin_key(&self.state.model_library_manager),
        };
        if self.state.workbench.models_view.recall_notice.as_ref() == Some(&scope) {
            return;
        }
        self.state.workbench.models_view.recall_notice = Some(scope);
        let Some(hub) = self.model_hub.hub() else {
            return;
        };
        let recalled = crate::state::model_hub::recalled_pins(
            hub.recalls(),
            &self.state.model_library_manager,
        );
        if recalled.is_empty() {
            return;
        }
        for pin in &recalled {
            self.state.push_user_message(ConsoleMessage::warning(format!(
                "'{}' is pinned to {} {}, which the publisher recalled, giving the reason '{}'. \
                 The bytes this project retained are unchanged and still solve; nothing further \
                 can be taken from that release.",
                pin.library, pin.pack_id, pin.version, pin.reason
            )));
        }
        self.state.ui.toasts.warn_with_title(
            ctx,
            "Recalled model release",
            format!(
                "{} librar{} in this project {} pinned to a recalled pack release. The console \
                 names each one.",
                recalled.len(),
                if recalled.len() == 1 { "y" } else { "ies" },
                if recalled.len() == 1 { "is" } else { "are" }
            ),
        );
    }

    /// Starts one Model Hub operation on the shared operation machine.
    pub(in crate::workbench) fn start_model_hub_operation(
        &mut self,
        ctx: &Context,
        operation: ModelHubRequest,
    ) {
        if self.model_hub.hub().is_none() {
            let reason = self
                .model_hub
                .unavailable_reason()
                .unwrap_or("The model hub is unavailable on this machine.")
                .to_owned();
            note_models_operation(
                &mut self.state,
                operation.description(),
                matches!(operation, ModelHubRequest::FetchSnapshot),
                operation.landing_pack(),
            );
            self.state.workbench.models_view.operational_state =
                ModelsOperationalState::from_failure(&reason);
            self.state.workbench.models_view.action_receipt = Some(Err(reason.clone()));
            self.state
                .push_user_message(ConsoleMessage::warning(reason));
            return;
        }
        #[cfg(not(target_arch = "wasm32"))]
        self.start_native_model_catalog_operation(
            ctx,
            NativeModelCatalogOperation::ModelHub(operation),
        );
        #[cfg(target_arch = "wasm32")]
        self.start_browser_model_hub_operation(ctx, operation);
    }

    #[cfg(not(target_arch = "wasm32"))]
    fn start_native_model_catalog_operation(
        &mut self,
        ctx: &Context,
        operation: NativeModelCatalogOperation,
    ) {
        if self.state.workbench.models_view.model_import_in_progress {
            self.state.push_user_message(ConsoleMessage::warning(
                "A model-source operation is already authenticating and parsing.".to_owned(),
            ));
            return;
        }
        let authority = capture_native_model_import_authority(&self.state);
        let candidate = self.state.model_library_manager.clone();
        let store = self.model_hub.store().cloned();
        note_models_operation(
            &mut self.state,
            operation.description(),
            matches!(
                operation,
                NativeModelCatalogOperation::ModelHub(ModelHubRequest::FetchSnapshot)
            ),
            operation.landing_pack(),
        );
        self.state.workbench.models_view.model_import_in_progress = true;
        self.state.workbench.models_view.model_import_label = Some(operation.progress_label());
        model_hub_progress().clear();
        let repaint = ctx.clone();
        let worker_operation = operation.clone();
        let spawn = std::thread::Builder::new()
            .name("rspice-model-import".to_owned())
            .spawn(move || {
                let result = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
                    run_native_model_catalog_operation(
                        candidate,
                        &worker_operation,
                        store.as_ref(),
                        model_hub_progress(),
                    )
                }))
                .unwrap_or_else(|_| {
                    Err(vec![
                        "The model parser terminated unexpectedly; no catalog state was published."
                            .to_owned(),
                    ])
                });
                let completion = NativeModelImport {
                    authority,
                    operation: worker_operation,
                    result,
                };
                match native_model_imports().lock() {
                    Ok(mut queue) => queue.push_back(completion),
                    Err(poisoned) => poisoned.into_inner().push_back(completion),
                }
                repaint.request_repaint();
            });
        if let Err(error) = spawn {
            self.state.workbench.models_view.model_import_in_progress = false;
            self.state.workbench.models_view.model_import_label = None;
            let message = format!("Could not start the background model-source operation: {error}");
            self.state.workbench.models_view.operational_state =
                ModelsOperationalState::from_failure(&message);
            self.state.workbench.models_view.action_receipt = Some(Err(message.clone()));
            self.state.push_user_message(ConsoleMessage::error(message));
        }
    }

    pub(in crate::workbench) fn process_pdk_settings_dialog(&mut self, ctx: &Context) {
        self.pump_model_catalog_operations(ctx);
        let result = super::pdk_settings::render_pdk_settings_dialog(
            ctx,
            &mut self.state.pdk_settings_dialog,
        );
        match result {
            PdkSettingsDialogResult::Applied(config) => {
                let config = *config;
                #[cfg(not(target_arch = "wasm32"))]
                self.start_native_pdk_configuration_apply(ctx, config);
                #[cfg(target_arch = "wasm32")]
                match prepare_pdk_configuration(&self.state, config) {
                    Ok((config, candidate, loaded)) => {
                        if let Err(error) = self
                            .start_browser_pdk_settings_publication(ctx, config, candidate, loaded)
                        {
                            emit_pdk_apply_messages(&mut self.state, Err(vec![error]));
                        }
                    }
                    Err(errors) => emit_pdk_apply_messages(&mut self.state, Err(errors)),
                }
            }
            PdkSettingsDialogResult::LoadFile(path) => {
                #[cfg(not(target_arch = "wasm32"))]
                self.start_native_model_source_parse(ctx, path);
                #[cfg(target_arch = "wasm32")]
                let _ = path;
            }
            PdkSettingsDialogResult::ImportBrowserFile => {
                self.start_model_source_import(ctx);
            }
            PdkSettingsDialogResult::Cancelled => {}
            PdkSettingsDialogResult::None => {}
        }
    }
}

#[cfg(not(target_arch = "wasm32"))]
fn capture_native_model_import_authority(state: &AppState) -> NativeModelImportAuthority {
    NativeModelImportAuthority {
        project_id: state
            .project_lifecycle
            .project_open
            .then(|| state.workspace.project.id().to_string()),
        project_revision: state.workspace.project.revision().get(),
        catalog_digest: state.model_library_manager.execution_catalog_digest(),
    }
}

#[cfg(not(target_arch = "wasm32"))]
fn native_model_import_authority_is_current(
    authority: &NativeModelImportAuthority,
    state: &AppState,
) -> bool {
    authority.project_id
        == state
            .project_lifecycle
            .project_open
            .then(|| state.workspace.project.id().to_string())
        && authority.project_revision == state.workspace.project.revision().get()
        && authority.catalog_digest == state.model_library_manager.execution_catalog_digest()
}

#[cfg(not(target_arch = "wasm32"))]
fn run_native_model_catalog_operation(
    mut candidate: crate::state::model_library::ModelLibraryManager,
    operation: &NativeModelCatalogOperation,
    store: Option<&crate::services::model_hub::ModelHubStoreHandle>,
    progress: &ModelHubProgress,
) -> Result<NativeModelCatalogOutput, Vec<String>> {
    if let NativeModelCatalogOperation::ModelHub(request) = operation {
        let store = store.ok_or_else(|| {
            vec![
                "The model hub is unavailable on this machine, so no pack can be installed or \
                 removed."
                    .to_owned(),
            ]
        })?;
        return run_model_hub_operation(store, candidate, request, progress)
            .map(|output| NativeModelCatalogOutput::ModelHub(Box::new(output)))
            .map_err(|error| vec![error]);
    }
    let library = match operation {
        NativeModelCatalogOperation::ImportFile { path } => candidate
            .load_library_file(path, None)
            .map_err(|error| vec![error])?,
        NativeModelCatalogOperation::RefreshLibrary {
            library,
            root,
            section,
            pack_id,
        } => {
            let loaded = if let Some(pack_id) = pack_id {
                candidate.refresh_spice_pack(pack_id)
            } else {
                candidate.load_library_file(root, section.as_deref())
            }
            .map_err(|error| vec![error])?;
            if loaded != *library {
                return Err(vec![format!(
                    "Refresh resolved library '{loaded}' instead of expected '{library}'."
                )]);
            }
            loaded
        }
        NativeModelCatalogOperation::AttachPack { pack_id } => candidate
            .attach_spice_pack(pack_id)
            .map_err(|error| vec![error])?,
        NativeModelCatalogOperation::AddPart { pack_id, part_name } => candidate
            .add_spice_part(pack_id, part_name)
            .map_err(|error| vec![error])?,
        NativeModelCatalogOperation::ApplyConfiguration(operation) => {
            let NativePdkConfigurationOperation {
                previous,
                config,
                technology_binding,
            } = operation.as_ref();
            let (config, candidate, loaded) = prepare_pdk_configuration_candidate(
                candidate,
                previous,
                technology_binding.as_ref(),
                config.clone(),
            )?;
            return Ok(NativeModelCatalogOutput::Configuration(Box::new(
                NativePdkConfigurationOutput {
                    previous: previous.clone(),
                    config,
                    candidate,
                    loaded,
                },
            )));
        }
        NativeModelCatalogOperation::ModelHub(_) => {
            // Answered above, before any project candidate was touched.
            unreachable!("a model hub operation builds no library candidate")
        }
    };
    let library_stats = candidate
        .get_library(&library)
        .map(|library| (library.model_count(), library.corner_count()));
    Ok(NativeModelCatalogOutput::Library {
        candidate: Box::new(candidate),
        library_name: library,
        library_stats,
    })
}

#[cfg(not(target_arch = "wasm32"))]
fn publish_native_model_catalog_library(
    state: &mut AppState,
    operation: &NativeModelCatalogOperation,
    candidate: crate::state::model_library::ModelLibraryManager,
    library_name: &str,
    library_stats: Option<(usize, usize)>,
) -> Result<String, Vec<String>> {
    if let NativeModelCatalogOperation::AddPart { part_name, .. } = operation
        && state
            .model_library_manager
            .get_library(library_name)
            .is_some()
    {
        return Ok(format!(
            "Part '{part_name}' is already available through library '{library_name}'."
        ));
    }

    let reason = match operation {
        NativeModelCatalogOperation::ImportFile { path } => {
            format!("import model source {}", path.display())
        }
        NativeModelCatalogOperation::RefreshLibrary { library, .. } => {
            format!("refresh model library {library}")
        }
        NativeModelCatalogOperation::AttachPack { pack_id } => {
            format!("attach model pack {pack_id}")
        }
        NativeModelCatalogOperation::AddPart { part_name, .. } => {
            format!("add shipped model part {part_name}")
        }
        NativeModelCatalogOperation::ApplyConfiguration(_)
        | NativeModelCatalogOperation::ModelHub(_) => {
            return Err(vec![
                "Internal model-source operation/result mismatch.".to_owned(),
            ]);
        }
    };
    let revision = if state.project_lifecycle.project_open {
        Some(
            publish_model_library_candidate(state, candidate, library_name, reason)
                .map_err(|error| vec![error])?,
        )
    } else {
        state.model_library_manager = candidate;
        None
    };
    let location = revision.map_or_else(
        || "in the session catalog".to_owned(),
        |revision| format!("at project revision {}", revision.get()),
    );
    let message = match operation {
        NativeModelCatalogOperation::ImportFile { path } => {
            emit_pdk_file_load_success_message(state, path.as_path(), library_name, library_stats);
            format!(
                "Imported model library '{library_name}' from a background-authenticated source closure {location}."
            )
        }
        NativeModelCatalogOperation::RefreshLibrary { .. } => {
            format!("Refreshed and pinned model library '{library_name}' {location}.")
        }
        NativeModelCatalogOperation::AttachPack { pack_id } => {
            format!("Attached pack '{pack_id}' as library '{library_name}' {location}.")
        }
        NativeModelCatalogOperation::AddPart { pack_id, part_name } => {
            format!("Added '{part_name}' from pack '{pack_id}' {location}.")
        }
        NativeModelCatalogOperation::ApplyConfiguration(_)
        | NativeModelCatalogOperation::ModelHub(_) => unreachable!(),
    };
    Ok(message)
}

#[cfg(not(target_arch = "wasm32"))]
fn poll_native_model_imports(
    ctx: &Context,
    state: &mut AppState,
    model_hub: &mut crate::services::model_hub::ModelHubService,
) {
    let completions = match native_model_imports().lock() {
        Ok(mut queue) => queue.drain(..).collect::<Vec<_>>(),
        Err(poisoned) => poisoned.into_inner().drain(..).collect::<Vec<_>>(),
    };
    for completion in completions {
        state.workbench.models_view.model_import_in_progress = false;
        state.workbench.models_view.model_import_label = None;
        state.workbench.models_view.model_import_progress = None;
        model_hub_progress().clear();
        // A re-proof's verdict is recorded whichever way it went, before any
        // of the publishing below can return early on an unrelated condition.
        if let (NativeModelCatalogOperation::ModelHub(request), Err(errors)) =
            (&completion.operation, &completion.result)
        {
            model_hub::note_pack_verification(state, request, Err(errors.join("; ")));
        }
        // The authority recheck exists to stop a candidate parsed against an
        // older project from being published over a newer one. A hub
        // operation that only changed this machine produced no candidate, so
        // rechecking it would discard a true result for an unrelated reason.
        let carries_project_candidate = match &completion.result {
            Ok(NativeModelCatalogOutput::ModelHub(output)) => output.part.is_some(),
            Err(_)
                if matches!(
                    completion.operation,
                    NativeModelCatalogOperation::ModelHub(_)
                ) =>
            {
                false
            }
            _ => true,
        };
        if carries_project_candidate
            && !native_model_import_authority_is_current(&completion.authority, state)
        {
            let error = format!(
                "The {} finished after the project or model catalog changed; its parsed candidate was discarded without mutation. Retry against the current project.",
                completion.operation.description()
            );
            state.workbench.models_view.operational_state = ModelsOperationalState::Stale;
            state.workbench.models_view.action_receipt = Some(Err(error.clone()));
            state.push_user_message(ConsoleMessage::warning(error.clone()));
            state
                .ui
                .toasts
                .warn_with_title(ctx, "Stale model import discarded", error);
            continue;
        }
        match completion.result {
            Ok(NativeModelCatalogOutput::Library {
                candidate,
                library_name,
                library_stats,
            }) => match publish_native_model_catalog_library(
                state,
                &completion.operation,
                *candidate,
                &library_name,
                library_stats,
            ) {
                Ok(receipt) => {
                    state.workbench.models_view.operational_state = ModelsOperationalState::Ready;
                    state.workbench.models_view.action_receipt = Some(Ok(receipt.clone()));
                    state.push_user_message(ConsoleMessage::info(receipt.clone()));
                    state
                        .ui
                        .toasts
                        .success(ctx, "Model catalog updated", receipt);
                }
                Err(errors) => {
                    emit_native_model_catalog_errors(ctx, state, errors);
                }
            },
            Ok(NativeModelCatalogOutput::ModelHub(output)) => {
                let request = match &completion.operation {
                    NativeModelCatalogOperation::ModelHub(request) => request.clone(),
                    _ => {
                        emit_native_model_catalog_errors(
                            ctx,
                            state,
                            vec!["Internal model-hub operation/result mismatch.".to_owned()],
                        );
                        continue;
                    }
                };
                publish_model_hub_output(ctx, state, model_hub, &request, *output);
            }
            Ok(NativeModelCatalogOutput::Configuration(output)) => {
                let NativePdkConfigurationOutput {
                    previous,
                    config,
                    candidate,
                    loaded,
                } = *output;
                if state.pdk_config != previous {
                    let error = "PDK settings changed while configured sources were loading; the stale candidate was discarded without mutation. Apply the current settings again."
                        .to_owned();
                    state.workbench.models_view.operational_state = ModelsOperationalState::Stale;
                    state.workbench.models_view.action_receipt = Some(Err(error.clone()));
                    state.push_user_message(ConsoleMessage::warning(error.clone()));
                    state
                        .ui
                        .toasts
                        .warn_with_title(ctx, "Stale PDK settings discarded", error);
                    continue;
                }
                let result = publish_prepared_pdk_configuration_with_persistence(
                    state,
                    previous,
                    (config, candidate, loaded),
                    |config| config.save().map_err(|error| error.to_string()),
                );
                match result {
                    Ok(loaded) => {
                        emit_pdk_apply_messages(state, Ok(loaded));
                        let receipt = format!(
                            "Applied PDK settings from a background-authenticated candidate ({loaded} libraries loaded)."
                        );
                        state.workbench.models_view.operational_state =
                            ModelsOperationalState::Ready;
                        state.workbench.models_view.action_receipt = Some(Ok(receipt.clone()));
                        state
                            .ui
                            .toasts
                            .success(ctx, "PDK settings applied", receipt);
                    }
                    Err(errors) => {
                        emit_pdk_apply_messages(state, Err(errors.clone()));
                        emit_native_model_catalog_errors(ctx, state, errors);
                    }
                }
            }
            Err(errors) => {
                emit_native_model_catalog_errors(ctx, state, errors);
            }
        }
    }
}

#[cfg(not(target_arch = "wasm32"))]
fn emit_native_model_catalog_errors(ctx: &Context, state: &mut AppState, errors: Vec<String>) {
    let first = errors
        .first()
        .cloned()
        .unwrap_or_else(|| "The model-source operation failed without a diagnostic.".to_owned());
    for error in &errors {
        state.push_user_message(ConsoleMessage::error(error.clone()));
    }
    state.workbench.models_view.operational_state = ModelsOperationalState::from_failure(&first);
    state.workbench.models_view.action_receipt = Some(Err(errors.join("; ")));
    state
        .ui
        .toasts
        .error_with_title(ctx, "Model-source operation failed", first);
}

#[cfg(any(test, target_arch = "wasm32"))]
fn normalize_browser_directory_member_names(raw_paths: &[String]) -> Result<Vec<String>, String> {
    if raw_paths.is_empty() {
        return Err("The selected model-source folder contains no files.".to_owned());
    }

    let mut directory_root: Option<String> = None;
    let mut members = Vec::with_capacity(raw_paths.len());
    for raw_path in raw_paths {
        let normalized = raw_path.replace('\\', "/");
        let (root, member) = normalized.split_once('/').ok_or_else(|| {
            format!("Browser folder selection did not preserve the relative path for '{raw_path}'.")
        })?;
        if root.is_empty() || member.is_empty() {
            return Err(format!(
                "Browser folder selection returned an invalid relative path '{raw_path}'."
            ));
        }
        if let Some(expected) = directory_root.as_deref() {
            if root != expected {
                return Err(format!(
                    "Browser folder selection returned more than one source root ('{expected}' and '{root}')."
                ));
            }
        } else {
            directory_root = Some(root.to_owned());
        }
        members.push(
            crate::state::model_library::normalize_browser_bundle_member_path(member).map_err(
                |error| format!("Browser source member '{raw_path}' is invalid: {error}"),
            )?,
        );
    }
    Ok(members)
}

#[cfg(target_arch = "wasm32")]
fn browser_picker_js_error(error: wasm_bindgen::JsValue) -> String {
    use js_sys::Reflect;
    use wasm_bindgen::JsValue;

    error
        .as_string()
        .or_else(|| {
            Reflect::get(&error, &JsValue::from_str("message"))
                .ok()
                .and_then(|message| message.as_string())
        })
        .unwrap_or_else(|| "unknown browser error".to_owned())
}

#[cfg(target_arch = "wasm32")]
async fn pick_browser_model_source_tree() -> Result<Option<Vec<(String, Vec<u8>)>>, String> {
    use js_sys::{Promise, Reflect, Uint8Array};
    use wasm_bindgen::{JsCast, JsValue, closure::Closure};
    use wasm_bindgen_futures::JsFuture;

    let window = web_sys::window().ok_or_else(|| "Browser window is unavailable.".to_owned())?;
    let document = window
        .document()
        .ok_or_else(|| "Browser document is unavailable.".to_owned())?;
    let input = document
        .create_element("input")
        .map_err(browser_picker_js_error)?
        .dyn_into::<web_sys::HtmlInputElement>()
        .map_err(|_| "Browser could not create a model-source folder picker.".to_owned())?;
    input.set_type("file");
    input.set_multiple(true);
    input
        .set_attribute("webkitdirectory", "")
        .map_err(browser_picker_js_error)?;
    input
        .set_attribute("directory", "")
        .map_err(browser_picker_js_error)?;
    input
        .set_attribute("style", "display:none")
        .map_err(browser_picker_js_error)?;
    input
        .set_attribute("aria-label", "Select model source folder")
        .map_err(browser_picker_js_error)?;
    if !Reflect::has(input.as_ref(), &JsValue::from_str("webkitdirectory"))
        .map_err(browser_picker_js_error)?
    {
        return Err(
            "This browser does not support folder-preserving model-source selection.".to_owned(),
        );
    }
    document
        .body()
        .ok_or_else(|| "Browser document has no body for the source picker.".to_owned())?
        .append_child(&input)
        .map_err(browser_picker_js_error)?;

    let change_input = input.clone();
    let cancel_input = input.clone();
    let selection = Promise::new(&mut |resolve, _reject| {
        let change_resolve = resolve.clone();
        let change_input = change_input.clone();
        let on_change = Closure::once_into_js(move |_event: web_sys::Event| {
            let _ = change_resolve.call1(&JsValue::UNDEFINED, &JsValue::TRUE);
            change_input.remove();
        });
        let cancel_resolve = resolve.clone();
        let cancel_input = cancel_input.clone();
        let on_cancel = Closure::once_into_js(move |_event: web_sys::Event| {
            let _ = cancel_resolve.call1(&JsValue::UNDEFINED, &JsValue::FALSE);
            cancel_input.remove();
        });
        let _ = Reflect::set(input.as_ref(), &JsValue::from_str("onchange"), &on_change);
        let _ = Reflect::set(input.as_ref(), &JsValue::from_str("oncancel"), &on_cancel);
    });
    input.click();
    let selected = JsFuture::from(selection)
        .await
        .map_err(browser_picker_js_error)?
        .as_bool()
        .unwrap_or(false);
    let _ = Reflect::set(
        input.as_ref(),
        &JsValue::from_str("onchange"),
        &JsValue::NULL,
    );
    let _ = Reflect::set(
        input.as_ref(),
        &JsValue::from_str("oncancel"),
        &JsValue::NULL,
    );
    input.remove();
    if !selected {
        return Ok(None);
    }

    let files = input
        .files()
        .ok_or_else(|| "Browser folder selection returned no file list.".to_owned())?;
    let file_count = usize::try_from(files.length())
        .map_err(|_| "Selected source tree exceeds browser file-count limits.".to_owned())?;
    if file_count == 0 {
        return Ok(None);
    }
    if file_count > crate::state::MAX_PROJECT_SOURCE_FILES {
        return Err(format!(
            "Selected model source tree contains {file_count} files; the limit is {}.",
            crate::state::MAX_PROJECT_SOURCE_FILES
        ));
    }

    let mut browser_files = Vec::with_capacity(file_count);
    let mut raw_paths = Vec::with_capacity(file_count);
    for index in 0..files.length() {
        let file = files
            .get(index)
            .ok_or_else(|| format!("Browser omitted selected source file {}.", index + 1))?;
        let relative_path = Reflect::get(file.as_ref(), &JsValue::from_str("webkitRelativePath"))
            .map_err(browser_picker_js_error)?
            .as_string()
            .filter(|path| !path.is_empty())
            .ok_or_else(|| {
                format!(
                    "Browser did not preserve the relative path for selected source '{}'.",
                    file.name()
                )
            })?;
        raw_paths.push(relative_path);
        browser_files.push(file);
    }
    let member_names = normalize_browser_directory_member_names(&raw_paths)?;

    let mut total_bytes = 0usize;
    let mut loaded = Vec::with_capacity(file_count);
    for (name, file) in member_names.into_iter().zip(browser_files) {
        let blob: &web_sys::Blob = file.as_ref();
        let size = blob.size();
        if !size.is_finite() || size < 0.0 {
            return Err(format!(
                "Selected model source '{name}' reported an invalid size."
            ));
        }
        let buffer = JsFuture::from(blob.array_buffer())
            .await
            .map_err(browser_picker_js_error)?;
        let view = Uint8Array::new(&buffer);
        let length = usize::try_from(view.length())
            .map_err(|_| format!("Selected model source '{name}' exceeds browser limits."))?;
        total_bytes = total_bytes
            .checked_add(length)
            .ok_or_else(|| "Selected model source tree size overflowed.".to_owned())?;
        if total_bytes > crate::state::MAX_PROJECT_SOURCE_BUNDLE_BYTES {
            return Err(format!(
                "Selected model source tree exceeds the supported {}-byte limit.",
                crate::state::MAX_PROJECT_SOURCE_BUNDLE_BYTES
            ));
        }
        let mut bytes = vec![0; length];
        view.copy_to(&mut bytes);
        loaded.push((name, bytes));
    }
    loaded.sort_by(|left, right| {
        left.0
            .to_ascii_lowercase()
            .cmp(&right.0.to_ascii_lowercase())
            .then_with(|| left.0.cmp(&right.0))
    });
    Ok(Some(loaded))
}

#[cfg(target_arch = "wasm32")]
fn start_browser_model_import(
    ctx: &Context,
    state: &AppState,
    replace_library: Option<String>,
) -> Result<(), String> {
    let authority = BrowserModelImportAuthority {
        project_id: state
            .project_lifecycle
            .project_open
            .then(|| state.workspace.project.id().to_string()),
        project_revision: state.workspace.project.revision().get(),
        catalog_digest: state.model_library_manager.execution_catalog_digest(),
        replace_library: replace_library.clone(),
    };
    let repaint = ctx.clone();
    wasm_bindgen_futures::spawn_local(async move {
        let result = pick_browser_model_source_tree().await;
        match result {
            Ok(Some(files)) => {
                let file_count = files.len();
                let display_name = if file_count == 1 {
                    files[0].0.clone()
                } else if let Some(replace_library) = replace_library.as_deref() {
                    format!("{replace_library}.lib")
                } else {
                    let first_stem = Path::new(&files[0].0)
                        .file_stem()
                        .and_then(|name| name.to_str())
                        .filter(|name| !name.is_empty())
                        .unwrap_or("browser-model");
                    format!("{first_stem}-bundle.lib")
                };
                let candidates = browser_model_root_candidates(&files);
                let root_name = match candidates.as_slice() {
                    [root] => root.clone(),
                    [] => {
                        queue_browser_model_import(BrowserModelImport {
                            authority,
                            result: Err(
                                "The selected source tree contains no supported SPICE or Spectre entry file."
                                    .to_owned(),
                            ),
                            root_candidates: None,
                        });
                        repaint.request_repaint();
                        return;
                    }
                    _ => {
                        BROWSER_PENDING_MODEL_ROOT.with(|pending| {
                            *pending.borrow_mut() = Some(BrowserPendingModelRoot {
                                authority: authority.clone(),
                                display_name,
                                files,
                                candidates: candidates.clone(),
                            });
                        });
                        queue_browser_model_import(BrowserModelImport {
                            authority,
                            result: Ok(None),
                            root_candidates: Some(candidates),
                        });
                        repaint.request_repaint();
                        return;
                    }
                };
                let metadata = BrowserModelImportWorkerMetadata {
                    protocol_version: BROWSER_MODEL_IMPORT_PROTOCOL_VERSION,
                    display_name,
                    root_name,
                    file_names: files.iter().map(|(name, _)| name.clone()).collect(),
                };
                if let Err(error) = browser_model_import_worker::start(
                    metadata,
                    files.into_iter().map(|(_, bytes)| bytes).collect(),
                    authority.clone(),
                    repaint.clone(),
                ) {
                    queue_browser_model_import(BrowserModelImport {
                        authority,
                        result: Err(error),
                        root_candidates: None,
                    });
                    repaint.request_repaint();
                }
            }
            Ok(None) => {
                queue_browser_model_import(BrowserModelImport {
                    authority,
                    result: Ok(None),
                    root_candidates: None,
                });
                repaint.request_repaint();
            }
            Err(error) => {
                queue_browser_model_import(BrowserModelImport {
                    authority,
                    result: Err(error),
                    root_candidates: None,
                });
                repaint.request_repaint();
            }
        }
    });
    Ok(())
}

#[cfg(target_arch = "wasm32")]
fn browser_model_root_candidates(files: &[(String, Vec<u8>)]) -> Vec<String> {
    files
        .iter()
        .filter_map(|(name, _)| {
            Path::new(name)
                .extension()
                .and_then(|extension| extension.to_str())
                .filter(|extension| {
                    matches!(
                        extension.to_ascii_lowercase().as_str(),
                        "lib" | "model" | "mod" | "spice" | "cir" | "inc" | "scs"
                    )
                })
                .map(|_| name.clone())
        })
        .collect()
}

/// Completed browser Model Hub work, waiting for the frame loop.
#[cfg(target_arch = "wasm32")]
struct BrowserModelHubCompletion {
    authority: BrowserModelImportAuthority,
    request: ModelHubRequest,
    result: Result<model_hub::ModelHubOutput, String>,
}

#[cfg(target_arch = "wasm32")]
thread_local! {
    static BROWSER_MODEL_HUB: std::cell::RefCell<std::collections::VecDeque<BrowserModelHubCompletion>> =
        const { std::cell::RefCell::new(std::collections::VecDeque::new()) };
}

#[cfg(target_arch = "wasm32")]
impl RSpiceApp {
    /// Starts one Model Hub operation as a browser task.
    ///
    /// The shape is the desktop's with the thread replaced by a task: capture
    /// the authority stamp before going async, work on a candidate nothing
    /// else can see, queue the completion, and wake the frame loop.
    fn start_browser_model_hub_operation(&mut self, ctx: &Context, request: ModelHubRequest) {
        if self.state.workbench.models_view.model_import_in_progress {
            self.state.push_user_message(ConsoleMessage::warning(
                "A model-source operation is already authenticating and parsing.".to_owned(),
            ));
            return;
        }
        let Some(store) = self.model_hub.store().cloned() else {
            self.state.push_user_message(ConsoleMessage::warning(
                "The model hub is unavailable in this browser session.".to_owned(),
            ));
            return;
        };
        let authority = BrowserModelImportAuthority {
            project_id: self
                .state
                .project_lifecycle
                .project_open
                .then(|| self.state.workspace.project.id().to_string()),
            project_revision: self.state.workspace.project.revision().get(),
            catalog_digest: self.state.model_library_manager.execution_catalog_digest(),
            replace_library: None,
        };
        let candidate = self.state.model_library_manager.clone();
        note_models_operation(
            &mut self.state,
            request.description(),
            matches!(request, ModelHubRequest::FetchSnapshot),
            request.landing_pack(),
        );
        self.state.workbench.models_view.model_import_in_progress = true;
        self.state.workbench.models_view.model_import_label = Some(request.progress_label());
        model_hub_progress().clear();
        let repaint = ctx.clone();
        wasm_bindgen_futures::spawn_local(async move {
            let result =
                model_hub::run_browser_model_hub_operation(&store, candidate, &request).await;
            BROWSER_MODEL_HUB.with(|queue| {
                queue.borrow_mut().push_back(BrowserModelHubCompletion {
                    authority,
                    request,
                    result,
                });
            });
            repaint.request_repaint();
        });
    }
}

/// Drains completed browser Model Hub work into the session.
#[cfg(target_arch = "wasm32")]
fn poll_browser_model_hub_operations(
    ctx: &Context,
    state: &mut AppState,
    model_hub: &mut crate::services::model_hub::ModelHubService,
) {
    let completions =
        BROWSER_MODEL_HUB.with(|queue| queue.borrow_mut().drain(..).collect::<Vec<_>>());
    for completion in completions {
        state.workbench.models_view.model_import_in_progress = false;
        state.workbench.models_view.model_import_label = None;
        state.workbench.models_view.model_import_progress = None;
        model_hub_progress().clear();
        match completion.result {
            Ok(output) => {
                // Only a result that carries a project candidate can be made
                // stale by the project moving; a pack that landed on this
                // machine landed regardless of what the project did.
                let current = state
                    .project_lifecycle
                    .project_open
                    .then(|| state.workspace.project.id().to_string());
                let authority_current = completion.authority.project_id == current
                    && completion.authority.project_revision
                        == state.workspace.project.revision().get()
                    && completion.authority.catalog_digest
                        == state.model_library_manager.execution_catalog_digest();
                if output.part.is_some() && !authority_current {
                    let error = format!(
                        "The {} finished after the project changed; its retained candidate was \
                         discarded without mutation. Retry against the current project.",
                        completion.request.description()
                    );
                    state.workbench.models_view.operational_state = ModelsOperationalState::Stale;
                    state.workbench.models_view.action_receipt = Some(Err(error.clone()));
                    state.push_user_message(ConsoleMessage::warning(error.clone()));
                    state
                        .ui
                        .toasts
                        .warn_with_title(ctx, "Stale model-hub result discarded", error);
                    continue;
                }
                publish_model_hub_output(ctx, state, model_hub, &completion.request, output);
            }
            Err(error) => {
                model_hub::note_pack_verification(state, &completion.request, Err(error.clone()));
                model_hub::emit_model_hub_errors(ctx, state, vec![error]);
            }
        }
    }
}

#[cfg(target_arch = "wasm32")]
fn queue_browser_model_import(completion: BrowserModelImport) {
    BROWSER_MODEL_IMPORTS.with(|queue| queue.borrow_mut().push_back(completion));
}

#[cfg(target_arch = "wasm32")]
fn poll_browser_model_imports(ctx: &Context, state: &mut AppState) {
    let completions =
        BROWSER_MODEL_IMPORTS.with(|queue| queue.borrow_mut().drain(..).collect::<Vec<_>>());
    for completion in completions {
        state.workbench.models_view.model_import_in_progress = false;
        state.workbench.models_view.model_import_label = None;
        browser_model_import_worker::finish();
        let current_project_id = state
            .project_lifecycle
            .project_open
            .then(|| state.workspace.project.id().to_string());
        let authority_current = completion.authority.project_id == current_project_id
            && completion.authority.project_revision == state.workspace.project.revision().get()
            && completion.authority.catalog_digest
                == state.model_library_manager.execution_catalog_digest();
        let replace_library = completion.authority.replace_library.clone();
        if let Some(candidates) = completion.root_candidates {
            if authority_current {
                state.workbench.models_view.dialog = Some(
                    crate::workbench::state::ModelsWorkbenchDialog::SelectBrowserImportRoot {
                        candidates,
                        selected: 0,
                    },
                );
                // This dialog renders only inside the Models surface while it
                // counts as application-modal everywhere, so opening it for a
                // user who has navigated away leaves an invisible modal with
                // no way to answer it. Every other producer of this dialog
                // either lives in the Models surface or navigates there first.
                state
                    .workbench
                    .activate(crate::workbench::state::Workspace::Models);
                state.workbench.models_page = crate::workbench::state::ModelsPage::Models;
                state.workbench.models_view.operational_state = ModelsOperationalState::Ready;
                state.workbench.models_view.action_receipt = Some(Ok(
                    "Select the executable entry file for the browser model source tree."
                        .to_owned(),
                ));
            } else {
                BROWSER_PENDING_MODEL_ROOT.with(|pending| {
                    pending.borrow_mut().take();
                });
                let error = "The project or model catalog changed while browser model sources were being selected; the pending selection was discarded."
                    .to_owned();
                state.workbench.models_view.operational_state = ModelsOperationalState::Stale;
                state.workbench.models_view.action_receipt = Some(Err(error.clone()));
                state.push_user_message(ConsoleMessage::error(error));
            }
            continue;
        }
        let result = completion.result.and_then(|parsed| {
            let Some(parsed) = parsed else {
                return Ok(None);
            };
            if !authority_current {
                return Err(
                    "The project or model catalog changed while the browser worker was parsing; the stale candidate was discarded without mutation."
                        .to_owned(),
                );
            }
            let library_name = parsed.library.name.clone();
            if let Some(expected) = replace_library.as_deref() {
                if !library_name.eq_ignore_ascii_case(expected) {
                    return Err(format!(
                        "Replacement sources resolved library '{library_name}' instead of expected '{expected}'; the candidate was discarded."
                    ));
                }
                if state.model_library_manager.get_library(expected).is_none() {
                    return Err(format!(
                        "Model library '{expected}' was removed while replacement sources were parsing."
                    ));
                }
            } else if state
                .model_library_manager
                .get_library(&library_name)
                .is_some()
            {
                return Err(format!(
                    "Model library '{library_name}' was added while the worker was parsing; the candidate was discarded."
                ));
            }
            if parsed.library.pack_id.is_some() {
                return Err(
                    "Browser worker incorrectly assigned official pack authority to a user import."
                        .to_owned(),
                );
            }
            let mut candidate = state.model_library_manager.clone();
            candidate.add_library(parsed.library);
            if state.project_lifecycle.project_open {
                publish_model_library_candidate(
                    state,
                    candidate,
                    &library_name,
                    if replace_library.is_some() {
                        format!("refresh browser model source bundle {}", parsed.display_name)
                    } else {
                        format!("import browser model source bundle {}", parsed.display_name)
                    },
                )?;
            } else {
                state.model_library_manager = candidate;
            }
            Ok(Some((
                library_name,
                parsed.file_count,
                replace_library.is_some(),
            )))
        });
        match result {
            Ok(Some((library, file_count, replaced))) => {
                let message = format!(
                    "{} browser model library '{library}' from {file_count} authenticated source file{} with exact retained bytes",
                    if replaced { "Refreshed" } else { "Imported" },
                    if file_count == 1 { "" } else { "s" }
                );
                state.workbench.models_view.operational_state = ModelsOperationalState::Ready;
                state.workbench.models_view.action_receipt = Some(Ok(message.clone()));
                state.push_user_message(ConsoleMessage::info(message.clone()));
                state.ui.toasts.success(
                    ctx,
                    if replaced {
                        "Model library refreshed"
                    } else {
                        "Model library imported"
                    },
                    message,
                );
            }
            Ok(None) => {
                state.workbench.models_view.operational_state = ModelsOperationalState::Cancelled;
                state.workbench.models_view.action_receipt = Some(Ok(
                    "Browser model-source selection was cancelled.".to_owned(),
                ));
            }
            Err(error) => {
                state.workbench.models_view.operational_state = if authority_current {
                    ModelsOperationalState::from_failure(&error)
                } else {
                    ModelsOperationalState::Stale
                };
                state.workbench.models_view.action_receipt = Some(Err(error.clone()));
                state.push_user_message(ConsoleMessage::error(error.clone()));
                state
                    .ui
                    .toasts
                    .error_with_title(ctx, "Model import failed", error);
            }
        }
    }
}

#[cfg(target_arch = "wasm32")]
mod browser_model_import_worker {
    use std::cell::{Cell, RefCell};
    use std::rc::Rc;

    use js_sys::{Array, Object, Reflect, Uint8Array};
    use wasm_bindgen::JsCast as _;
    use wasm_bindgen::prelude::*;

    use super::*;

    pub(super) const MAX_RESPONSE_BYTES: usize = 256 * 1024 * 1024;

    struct ActiveWorker {
        worker: web_sys::Worker,
        _onmessage: Closure<dyn FnMut(web_sys::MessageEvent)>,
        _onerror: Closure<dyn FnMut(web_sys::ErrorEvent)>,
        _onmessageerror: Closure<dyn FnMut(web_sys::MessageEvent)>,
    }

    impl Drop for ActiveWorker {
        fn drop(&mut self) {
            self.worker.set_onmessage(None);
            self.worker.set_onerror(None);
            self.worker.set_onmessageerror(None);
            self.worker.terminate();
        }
    }

    thread_local! {
        static NEXT_REQUEST_ID: Cell<u32> = const { Cell::new(0) };
        static ACTIVE_WORKER: RefCell<Option<ActiveWorker>> = const { RefCell::new(None) };
    }

    pub(super) fn start(
        metadata: BrowserModelImportWorkerMetadata,
        buffers: Vec<Vec<u8>>,
        authority: BrowserModelImportAuthority,
        repaint: egui::Context,
    ) -> Result<(), String> {
        if ACTIVE_WORKER.with(|active| active.borrow().is_some()) {
            return Err("A browser model parser worker is already active.".to_owned());
        }
        if metadata.file_names.len() != buffers.len() {
            return Err("Browser model import metadata does not match its buffers.".to_owned());
        }
        let id = NEXT_REQUEST_ID.with(|next| {
            let id = next.get().wrapping_add(1).max(1);
            next.set(id);
            id
        });
        let display_name = metadata.display_name.clone();
        let file_count = metadata.file_names.len();
        let metadata = serde_wasm_bindgen::to_value(&metadata)
            .map_err(|error| format!("Could not encode model-import metadata: {error}"))?;
        let request_buffers = Array::new();
        let transfer = Array::new();
        for bytes in buffers {
            let length = u32::try_from(bytes.len())
                .map_err(|_| "A selected model source exceeds browser array limits.".to_owned())?;
            let view = Uint8Array::new_with_length(length);
            view.copy_from(&bytes);
            transfer.push(&view.buffer());
            request_buffers.push(&view);
        }
        let request = Object::new();
        Reflect::set(&request, &JsValue::from_str("metadata"), &metadata)
            .map_err(js_error_message)?;
        Reflect::set(&request, &JsValue::from_str("buffers"), &request_buffers)
            .map_err(js_error_message)?;

        let options = web_sys::WorkerOptions::new();
        options.set_type(web_sys::WorkerType::Module);
        let worker = web_sys::Worker::new_with_options(&worker_url()?, &options)
            .map_err(js_error_message)?;
        let completed = Rc::new(Cell::new(false));

        let success_authority = authority.clone();
        let success_repaint = repaint.clone();
        let success_completed = Rc::clone(&completed);
        let onmessage = Closure::<dyn FnMut(web_sys::MessageEvent)>::wrap(Box::new(
            move |event: web_sys::MessageEvent| {
                let data = event.data();
                let response_id = numeric_property(&data, "id").unwrap_or(0);
                if response_id != id {
                    return;
                }
                let result = match string_property(&data, "type").as_deref() {
                    Some("model-import-result") => {
                        Reflect::get(&data, &JsValue::from_str("response"))
                            .map_err(js_error_message)
                            .and_then(|response| decode_response(&response))
                            .map(|library| {
                                Some(BrowserParsedModelImport {
                                    display_name: display_name.clone(),
                                    file_count,
                                    library,
                                })
                            })
                    }
                    Some("model-import-error") | Some("error") => {
                        Err(string_property(&data, "error")
                            .or_else(|| string_property(&data, "message"))
                            .unwrap_or_else(|| "Browser model parser worker failed.".to_owned()))
                    }
                    _ => return,
                };
                complete_once(
                    &success_completed,
                    &success_repaint,
                    success_authority.clone(),
                    result,
                );
            },
        ));
        worker.set_onmessage(Some(onmessage.as_ref().unchecked_ref()));

        let error_authority = authority.clone();
        let error_repaint = repaint.clone();
        let error_completed = Rc::clone(&completed);
        let onerror = Closure::<dyn FnMut(web_sys::ErrorEvent)>::wrap(Box::new(
            move |event: web_sys::ErrorEvent| {
                complete_once(
                    &error_completed,
                    &error_repaint,
                    error_authority.clone(),
                    Err(if event.message().is_empty() {
                        "Browser model parser worker failed.".to_owned()
                    } else {
                        event.message()
                    }),
                );
            },
        ));
        worker.set_onerror(Some(onerror.as_ref().unchecked_ref()));

        let message_authority = authority;
        let message_repaint = repaint;
        let message_completed = completed;
        let onmessageerror = Closure::<dyn FnMut(web_sys::MessageEvent)>::wrap(Box::new(
            move |_event: web_sys::MessageEvent| {
                complete_once(
                    &message_completed,
                    &message_repaint,
                    message_authority.clone(),
                    Err("Browser model parser worker returned an unreadable message.".to_owned()),
                );
            },
        ));
        worker.set_onmessageerror(Some(onmessageerror.as_ref().unchecked_ref()));

        let message = Object::new();
        Reflect::set(
            &message,
            &JsValue::from_str("type"),
            &JsValue::from_str("run-model-import"),
        )
        .map_err(js_error_message)?;
        Reflect::set(
            &message,
            &JsValue::from_str("id"),
            &JsValue::from_f64(f64::from(id)),
        )
        .map_err(js_error_message)?;
        Reflect::set(&message, &JsValue::from_str("request"), &request)
            .map_err(js_error_message)?;
        ACTIVE_WORKER.with(|active| {
            *active.borrow_mut() = Some(ActiveWorker {
                worker: worker.clone(),
                _onmessage: onmessage,
                _onerror: onerror,
                _onmessageerror: onmessageerror,
            });
        });
        if let Err(error) = worker.post_message_with_transfer(&message, &transfer) {
            finish();
            return Err(format!(
                "Could not dispatch browser model import: {}",
                js_error_message(error)
            ));
        }
        Ok(())
    }

    pub(super) fn finish() {
        ACTIVE_WORKER.with(|active| {
            active.borrow_mut().take();
        });
    }

    fn complete_once(
        completed: &Cell<bool>,
        repaint: &egui::Context,
        authority: BrowserModelImportAuthority,
        result: Result<Option<BrowserParsedModelImport>, String>,
    ) {
        if completed.replace(true) {
            return;
        }
        queue_browser_model_import(BrowserModelImport {
            authority,
            result,
            root_candidates: None,
        });
        repaint.request_repaint();
    }

    fn decode_response(
        value: &JsValue,
    ) -> Result<crate::state::model_library::ModelLibrary, String> {
        let protocol = numeric_property(value, "protocolVersion")
            .ok_or_else(|| "Model-import worker response has no protocol version.".to_owned())?;
        if protocol != u32::from(BROWSER_MODEL_IMPORT_PROTOCOL_VERSION) {
            return Err(format!(
                "Unsupported model-import worker protocol {protocol}."
            ));
        }
        let bytes =
            Reflect::get(value, &JsValue::from_str("libraryBytes")).map_err(js_error_message)?;
        let bytes = Uint8Array::new(&bytes);
        let length = usize::try_from(bytes.length())
            .map_err(|_| "Model-import worker response exceeds host limits.".to_owned())?;
        if length == 0 || length > MAX_RESPONSE_BYTES {
            return Err(format!(
                "Model-import worker response contains {length} bytes; the supported range is 1..={MAX_RESPONSE_BYTES}."
            ));
        }
        let mut encoded = vec![0; length];
        bytes.copy_to(&mut encoded);
        serde_json::from_slice(&encoded)
            .map_err(|error| format!("Model-import worker returned invalid library data: {error}"))
    }

    fn worker_url() -> Result<String, String> {
        Reflect::get(
            &js_sys::global(),
            &JsValue::from_str("__RSPICE_SIM_WORKER_URL"),
        )
        .map_err(js_error_message)?
        .as_string()
        .filter(|url| !url.trim().is_empty())
        .ok_or_else(|| "Browser model parser worker URL is unavailable.".to_owned())
    }

    fn string_property(value: &JsValue, property: &str) -> Option<String> {
        Reflect::get(value, &JsValue::from_str(property))
            .ok()
            .and_then(|value| value.as_string())
    }

    fn numeric_property(value: &JsValue, property: &str) -> Option<u32> {
        Reflect::get(value, &JsValue::from_str(property))
            .ok()
            .and_then(|value| value.as_f64())
            .filter(|value| {
                value.is_finite()
                    && *value >= 0.0
                    && *value <= f64::from(u32::MAX)
                    && value.fract() == 0.0
            })
            .map(|value| value as u32)
    }

    fn js_error_message(error: JsValue) -> String {
        error
            .as_string()
            .or_else(|| {
                Reflect::get(&error, &JsValue::from_str("message"))
                    .ok()
                    .and_then(|message| message.as_string())
            })
            .unwrap_or_else(|| "unknown JavaScript error".to_owned())
    }
}

#[cfg(target_arch = "wasm32")]
pub(crate) fn run_model_import_worker_request_value(
    request: wasm_bindgen::JsValue,
) -> Result<wasm_bindgen::JsValue, wasm_bindgen::JsValue> {
    use js_sys::{Array, Object, Reflect, Uint8Array};
    use wasm_bindgen::JsValue;

    let metadata = Reflect::get(&request, &JsValue::from_str("metadata")).map_err(|error| error)?;
    let metadata: BrowserModelImportWorkerMetadata = serde_wasm_bindgen::from_value(metadata)
        .map_err(|error| JsValue::from_str(&format!("Invalid model-import metadata: {error}")))?;
    if metadata.protocol_version != BROWSER_MODEL_IMPORT_PROTOCOL_VERSION {
        return Err(JsValue::from_str(&format!(
            "Unsupported model-import protocol {}.",
            metadata.protocol_version
        )));
    }
    let buffers = Reflect::get(&request, &JsValue::from_str("buffers"))?;
    if !Array::is_array(&buffers) {
        return Err(JsValue::from_str(
            "Model-import request buffers are not an array.",
        ));
    }
    let buffers = Array::from(&buffers);
    if usize::try_from(buffers.length()).ok() != Some(metadata.file_names.len()) {
        return Err(JsValue::from_str(
            "Model-import file metadata does not match the transferred buffers.",
        ));
    }
    let mut total_bytes = 0usize;
    let mut files = Vec::with_capacity(metadata.file_names.len());
    for (index, name) in metadata.file_names.iter().enumerate() {
        let value =
            buffers.get(u32::try_from(index).map_err(|_| {
                JsValue::from_str("Model-import buffer index exceeds browser limits.")
            })?);
        let view = Uint8Array::new(&value);
        let length = usize::try_from(view.length())
            .map_err(|_| JsValue::from_str("Model-import buffer exceeds host limits."))?;
        total_bytes = total_bytes
            .checked_add(length)
            .ok_or_else(|| JsValue::from_str("Model-import buffer size overflowed."))?;
        if total_bytes > crate::state::MAX_PROJECT_SOURCE_BUNDLE_BYTES {
            return Err(JsValue::from_str(
                "Model-import source bundle exceeds the supported byte limit.",
            ));
        }
        let mut bytes = vec![0; length];
        view.copy_to(&mut bytes);
        files.push((name.clone(), bytes));
    }

    let mut manager = crate::state::model_library::ModelLibraryManager::new();
    let library_name = manager
        .load_library_bundle_from_root(&metadata.display_name, &metadata.root_name, files, None)
        .map_err(|error| JsValue::from_str(&error))?;
    let library = manager.remove_library(&library_name).ok_or_else(|| {
        JsValue::from_str("Parsed model library disappeared before worker publication.")
    })?;
    let encoded = serde_json::to_vec(&library)
        .map_err(|error| JsValue::from_str(&format!("Could not encode parsed library: {error}")))?;
    if encoded.is_empty() || encoded.len() > browser_model_import_worker::MAX_RESPONSE_BYTES {
        return Err(JsValue::from_str(
            "Parsed model library exceeds the browser worker response limit.",
        ));
    }
    let length = u32::try_from(encoded.len())
        .map_err(|_| JsValue::from_str("Parsed model library exceeds browser array limits."))?;
    let bytes = Uint8Array::new_with_length(length);
    bytes.copy_from(&encoded);
    let response = Object::new();
    Reflect::set(
        &response,
        &JsValue::from_str("protocolVersion"),
        &JsValue::from_f64(f64::from(BROWSER_MODEL_IMPORT_PROTOCOL_VERSION)),
    )?;
    Reflect::set(&response, &JsValue::from_str("libraryBytes"), &bytes)?;
    Ok(response.into())
}

#[cfg(all(test, not(target_arch = "wasm32")))]
mod tests {
    use super::*;
    use crate::state::pdk_config::PdkConfig;

    #[test]
    fn browser_directory_selection_strips_only_the_selected_root() {
        let members = normalize_browser_directory_member_names(&[
            "vendor-pdk/corners/tt.scs".to_owned(),
            "vendor-pdk/models/mos/device.scs".to_owned(),
            "vendor-pdk/models/bjt/device.scs".to_owned(),
        ])
        .expect("browser-relative directory paths are preserved");
        assert_eq!(
            members,
            [
                "corners/tt.scs",
                "models/mos/device.scs",
                "models/bjt/device.scs"
            ]
        );
    }

    #[test]
    fn browser_directory_selection_rejects_flattened_or_mixed_roots() {
        let flattened = normalize_browser_directory_member_names(&["device.scs".to_owned()])
            .expect_err("a browser must preserve the selected relative path");
        assert!(flattened.contains("did not preserve"), "{flattened}");

        let mixed = normalize_browser_directory_member_names(&[
            "first/device.scs".to_owned(),
            "second/device.scs".to_owned(),
        ])
        .expect_err("one import cannot silently combine directory roots");
        assert!(mixed.contains("more than one source root"), "{mixed}");
    }

    fn configured_root(label: &str, file_name: &str, model_name: &str) -> std::path::PathBuf {
        let root = std::env::temp_dir().join(format!(
            "rspice-pdk-workflow-{label}-{}",
            uuid::Uuid::new_v4()
        ));
        std::fs::create_dir_all(&root).expect("create configured source root");
        std::fs::write(
            root.join(file_name),
            format!(".model {model_name} NMOS (LEVEL=1)\n"),
        )
        .expect("write configured model source");
        root
    }

    #[test]
    fn configured_sources_publish_and_unload_as_one_guarded_project_transaction() {
        let alpha_root = configured_root("alpha", "alpha.lib", "alpha_n");
        let beta_root = configured_root("beta", "beta.lib", "beta_n");
        let mut state = AppState::default();
        state.project_lifecycle.project_open = true;
        state.model_library_manager.clear();
        state.pdk_config = PdkConfig::new();
        let initial_revision = state.workspace.project.revision();
        let initial_epoch = state.design_execution_epoch;
        let mut config = PdkConfig::new();
        config.add_library_path(alpha_root.to_string_lossy().into_owned());
        config.add_library_path(beta_root.to_string_lossy().into_owned());

        assert_eq!(
            apply_pdk_configuration_with_persistence(&mut state, config, |_| Ok(()))
                .expect("configured sources publish"),
            2
        );
        let first_revision = state.workspace.project.revision();
        assert!(first_revision > initial_revision);
        assert_eq!(state.design_execution_epoch, initial_epoch.wrapping_add(1));
        assert!(state.workspace.project_metadata_dirty);
        assert!(state.model_library_manager.get_library("alpha").is_some());
        assert!(state.model_library_manager.get_library("beta").is_some());
        assert_eq!(state.pdk_config.managed_model_sources.len(), 2);
        assert!(state.can_undo_project_design());

        let mut disabled = state.pdk_config.clone();
        disabled.toggle_path_enabled(0);
        assert_eq!(
            apply_pdk_configuration_with_persistence(&mut state, disabled, |_| Ok(()))
                .expect("disabled source publishes"),
            1
        );
        assert!(state.workspace.project.revision() > first_revision);
        assert_eq!(state.design_execution_epoch, initial_epoch.wrapping_add(2));
        assert!(state.model_library_manager.get_library("alpha").is_none());
        assert!(state.model_library_manager.get_library("beta").is_some());

        assert_eq!(
            state
                .undo_project_design()
                .expect("undo configured source set"),
            Some("apply configured PDK model sources".to_owned())
        );
        assert!(state.model_library_manager.get_library("alpha").is_some());
        assert!(state.model_library_manager.get_library("beta").is_some());
        assert_eq!(state.design_execution_epoch, initial_epoch.wrapping_add(3));

        std::fs::remove_dir_all(alpha_root).expect("remove alpha source root");
        std::fs::remove_dir_all(beta_root).expect("remove beta source root");
    }

    #[test]
    fn persistence_failure_leaves_configuration_manager_and_project_unchanged() {
        let root = configured_root("persistence", "persisted.lib", "persisted_n");
        let mut state = AppState::default();
        state.project_lifecycle.project_open = true;
        state.model_library_manager.clear();
        state.pdk_config = PdkConfig::new();
        let revision = state.workspace.project.revision();
        let epoch = state.design_execution_epoch;
        let mut config = PdkConfig::new();
        config.add_library_path(root.to_string_lossy().into_owned());

        let errors = apply_pdk_configuration_with_persistence(&mut state, config, |_| {
            Err("storage unavailable".to_owned())
        })
        .expect_err("failed persistence rejects the candidate");

        assert!(
            errors
                .iter()
                .any(|error| error.contains("storage unavailable"))
        );
        assert!(
            state
                .model_library_manager
                .get_library("persisted")
                .is_none()
        );
        assert!(state.pdk_config.library_paths().is_empty());
        assert_eq!(state.workspace.project.revision(), revision);
        assert_eq!(state.design_execution_epoch, epoch);
        assert!(!state.can_undo_project_design());

        std::fs::remove_dir_all(root).expect("remove persistence source root");
    }

    #[test]
    fn startup_service_hydrates_persisted_sources_without_dirtying_a_closed_project() {
        let root = configured_root("startup", "startup.lib", "startup_n");
        let mut state = AppState::default();
        state.model_library_manager.clear();
        let revision = state.workspace.project.revision();
        let epoch = state.design_execution_epoch;
        let mut config = PdkConfig::new();
        config.add_library_path(root.to_string_lossy().into_owned());

        state.pdk_config = config;
        let (manager, loaded) = crate::workbench::app_state::restore_session_model_library_manager(
            &mut state.pdk_config,
        )
        .expect("startup configured source hydration");
        state.model_library_manager = manager;
        assert_eq!(loaded, 1);
        assert!(state.model_library_manager.get_library("startup").is_some());
        assert_eq!(state.workspace.project.revision(), revision);
        assert_eq!(state.design_execution_epoch, epoch);
        assert!(!state.workspace.project_metadata_dirty);
        assert!(!state.can_undo_project_design());

        std::fs::remove_dir_all(root).expect("remove startup source root");
    }

    #[test]
    fn background_catalog_candidate_is_isolated_and_rejected_after_authority_drift() {
        let root = configured_root("background", "background.lib", "background_n");
        let path = root.join("background.lib");
        let mut state = AppState::default();
        state.project_lifecycle.project_open = true;
        state.model_library_manager.clear();
        let authority = capture_native_model_import_authority(&state);
        assert!(native_model_import_authority_is_current(&authority, &state));

        let output = run_native_model_catalog_operation(
            state.model_library_manager.clone(),
            &NativeModelCatalogOperation::ImportFile { path: path.clone() },
            None,
            model_hub_progress(),
        )
        .expect("background candidate parses");
        let NativeModelCatalogOutput::Library {
            candidate,
            library_name,
            ..
        } = output
        else {
            panic!("file import returns a library candidate");
        };
        assert_eq!(library_name, "background");
        assert!(candidate.get_library("background").is_some());
        assert!(
            state
                .model_library_manager
                .get_library("background")
                .is_none()
        );

        state
            .model_library_manager
            .add_library(crate::state::model_library::ModelLibrary::new(
                "concurrent-change",
            ));
        assert!(!native_model_import_authority_is_current(
            &authority, &state
        ));
        assert!(
            state
                .model_library_manager
                .get_library("background")
                .is_none()
        );

        std::fs::remove_dir_all(root).expect("remove background source root");
    }

    #[test]
    fn background_authority_binds_the_exact_project_lifecycle() {
        let mut state = AppState::default();
        state.project_lifecycle.project_open = true;
        let authority = capture_native_model_import_authority(&state);
        assert!(native_model_import_authority_is_current(&authority, &state));

        state.project_lifecycle.project_open = false;
        assert!(!native_model_import_authority_is_current(
            &authority, &state
        ));
    }
}
