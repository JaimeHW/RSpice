//! Opening a deck, carried out by the shell.
//!
//! A deck is a file on the desktop. Opening one adds it to the open
//! project's decks and brings it to the front of the Netlist page; with no
//! project open, a new project is made around it first. Both happen on a
//! copy of the whole application state that replaces it only once the deck
//! has opened, so a file that cannot be read changes nothing.

use crate::workbench::app_state::AppState;
#[cfg(not(target_arch = "wasm32"))]
use rspice_app_state::diagnostics::{LogSeverity, LogSource};

/// Opening a deck file, over the whole application state.
impl AppState {
    /// Ask for a deck file and open it. A deck that could not be opened now
    /// is refused before any file is asked for.
    #[cfg(not(target_arch = "wasm32"))]
    pub(crate) fn open_deck(&mut self) -> bool {
        let new_project = !self.platform.project_lifecycle.is_open();
        if let Some(reason) = self.deck_open_refusal(new_project) {
            self.prompt(reason);
            return false;
        }
        let Some(path) =
            rspice_app_code::workbench::workflows::netlist_workflow::show_open_deck_dialog()
        else {
            return false;
        };
        self.open_deck_at(&path, new_project)
    }

    /// New deck: on the desktop, ask where its file goes, write a title line
    /// and `.end` there and open it; where there is no file system, a deck
    /// the project keeps as text. A deck that could not be added now is
    /// refused before any file is asked for.
    pub(crate) fn new_deck(&mut self) -> bool {
        #[cfg(target_arch = "wasm32")]
        {
            self.new_stored_deck()
        }
        #[cfg(not(target_arch = "wasm32"))]
        {
            let new_project = !self.platform.project_lifecycle.is_open();
            if let Some(reason) = self.deck_open_refusal(new_project) {
                self.prompt(reason);
                return false;
            }
            let folder = self
                .model
                .workspace
                .content
                .project
                .data_root()
                .map(std::path::Path::to_path_buf)
                .or_else(dirs::document_dir);
            let Some(path) =
                rspice_app_code::workbench::workflows::netlist_workflow::show_new_deck_dialog(
                    folder.as_deref(),
                )
            else {
                return false;
            };
            self.new_deck_at(&path, new_project)
        }
    }

    /// Write a new deck, a title line and `.end`, to `path` and open it, in a
    /// new project around it when `new_project` says so.
    #[cfg(not(target_arch = "wasm32"))]
    pub(crate) fn new_deck_at(&mut self, path: &std::path::Path, new_project: bool) -> bool {
        let deck = rspice_app_state::state::StoredDeck::new(
            &path
                .file_name()
                .map_or_else(String::new, |name| name.to_string_lossy().into_owned()),
        );
        if let Err(error) = std::fs::write(path, deck.text) {
            self.prompt(format!("{} could not be written: {error}", path.display()));
            return false;
        }
        self.open_deck_at(path, new_project)
    }

    /// Add a new deck the project keeps as text and bring it to the front,
    /// in a new project around it when none is open: what New deck makes
    /// where there is no file system.
    #[cfg(any(target_arch = "wasm32", test))]
    pub(crate) fn new_stored_deck(&mut self) -> bool {
        let new_project = !self.platform.project_lifecycle.is_open();
        if let Some(reason) = self.deck_open_refusal(new_project) {
            self.prompt(reason);
            return false;
        }
        let mut candidate = self.clone();
        if new_project {
            candidate.create_netlist_project(&rspice_app_state::state::new_stored_deck_name(
                std::iter::empty::<&str>(),
            ));
        }
        let name = match rspice_app_code::workbench::documents::deck_files::new_stored_deck(
            &mut candidate.model,
            &mut candidate.code.netlist,
            &mut candidate.code.netlist_open_documents,
            &mut candidate.runs,
        ) {
            Ok(name) => name,
            Err(error) => {
                self.prompt(error);
                return false;
            }
        };
        candidate
            .workbench
            .activate(rspice_app_state::workbench::workspace::Workspace::Netlist);
        self.commit_candidate(candidate);
        self.record(
            rspice_app_state::diagnostics::LogSeverity::Info,
            rspice_app_state::diagnostics::LogSource::Netlist,
            format!("Created deck {name}."),
        );
        true
    }

    /// Why no deck can be opened now: a run in progress, or a project open
    /// read-only that the deck would be added to.
    fn deck_open_refusal(&self, new_project: bool) -> Option<&'static str> {
        if self.model.simulation.execution.has_active_execution() {
            return Some("A deck cannot be opened while a simulation is running.");
        }
        if !new_project && self.model.safe_mode.project_read_only() {
            return Some("A deck cannot be added: the project is open read-only.");
        }
        None
    }

    /// A deck is a file, and the browser has no file to keep it in.
    #[cfg(target_arch = "wasm32")]
    pub(crate) fn open_deck(&mut self) -> bool {
        self.prompt("Decks open from files on the desktop.");
        false
    }

    /// Open the deck file at `path`, in a new project around it when
    /// `new_project` says so, else in the open project.
    #[cfg(not(target_arch = "wasm32"))]
    pub(crate) fn open_deck_at(&mut self, path: &std::path::Path, new_project: bool) -> bool {
        if let Some(reason) = self.deck_open_refusal(new_project) {
            self.prompt(reason);
            return false;
        }
        if let Some(unsaved) = self
            .code
            .netlist
            .open_deck
            .as_ref()
            .and_then(rspice_app_code::workbench::documents::deck_files::OpenDeck::unsaved)
        {
            self.prompt(format!(
                "Save {} before opening another deck.",
                unsaved.name()
            ));
            return false;
        }
        let name = path.file_name().map_or_else(
            || path.display().to_string(),
            |name| name.to_string_lossy().into_owned(),
        );
        let mut candidate = self.clone();
        if new_project {
            candidate.create_netlist_project(&name);
        }
        if let Err(error) = rspice_app_code::workbench::documents::deck_files::open_deck(
            &mut candidate.model,
            &mut candidate.code.netlist,
            &mut candidate.code.netlist_open_documents,
            &mut candidate.runs,
            path,
        ) {
            self.prompt(error);
            return false;
        }
        candidate
            .workbench
            .activate(rspice_app_state::workbench::workspace::Workspace::Netlist);
        self.commit_candidate(candidate);
        self.record(
            LogSeverity::Info,
            LogSource::Netlist,
            format!("Opened deck {name}."),
        );
        true
    }

    /// Open the one file dropped on the Netlist page, or on the launcher,
    /// as a deck: in a new project when `new_project` says so.
    pub(crate) fn open_dropped_deck(
        &mut self,
        dropped: &[egui::DroppedFile],
        new_project: bool,
    ) -> bool {
        if self.application_modal_open() {
            return false;
        }
        let [file] = dropped else {
            self.prompt("Drop one SPICE deck at a time.");
            return false;
        };
        #[cfg(not(target_arch = "wasm32"))]
        {
            let Some(path) = file.path.clone() else {
                self.prompt("Only a deck file on this computer opens as a deck.");
                return false;
            };
            self.open_deck_at(&path, new_project)
        }
        #[cfg(target_arch = "wasm32")]
        {
            let _ = (file, new_project);
            self.open_deck()
        }
    }

    /// Put `text` in front of the Netlist page as the project's one deck,
    /// open from a file named `deck.cir` that is never read: what a test that
    /// only edits and runs the deck opens.
    #[cfg(test)]
    pub(crate) fn seed_test_deck(&mut self, text: impl Into<String>) {
        let deck = rspice_app_code::workbench::documents::deck_files::OpenDeck::in_memory(
            "deck.cir", text,
        );
        self.model.workspace.content.decks = vec![rspice_app_state::state::ProjectDeck {
            id: deck.id,
            source: rspice_app_state::state::DeckSource::File {
                path: "deck.cir".into(),
            },
        }];
        rspice_app_code::workbench::documents::deck_files::install_open_deck(
            &mut self.model,
            &mut self.code.netlist,
            &mut self.code.netlist_open_documents,
            &mut self.runs,
            deck,
        );
    }

    /// Write `source` to a deck file and open it, as a test opens one: at
    /// `path` when given, else in a folder of its own under `name`.
    #[cfg(test)]
    pub(crate) fn open_test_deck(
        &mut self,
        source: String,
        path: Option<std::path::PathBuf>,
        name: &str,
    ) -> bool {
        let path = path.unwrap_or_else(|| {
            let folder =
                std::env::temp_dir().join(format!("rspice_test_deck_{}", uuid::Uuid::new_v4()));
            std::fs::create_dir_all(&folder).expect("test deck folder");
            folder.join(name)
        });
        std::fs::write(&path, source).expect("test deck written");
        let new_project = !self.platform.project_lifecycle.is_open();
        self.open_deck_at(&path, new_project)
    }
}
