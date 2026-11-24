//! Opening a deck, carried out by the shell.
//!
//! A deck is a file on the desktop. Opening one adds it to the open
//! project's decks and brings it to the front of the Netlist page; with no
//! project open, a new project is made around it first. A file that cannot
//! be read changes nothing. Read on the frame that opens it, both happen on a
//! copy of the whole application state that replaces it only once the deck
//! has opened. The application reads the file away from the frame instead,
//! its tab in front at once saying it is reading: a file that then does not
//! read as text is let go, and a project made for it gives way to what was
//! there before.

use crate::workbench::app_state::AppState;
#[cfg(not(target_arch = "wasm32"))]
use rspice_app_state::diagnostics::{LogSeverity, LogSource};
#[cfg(not(target_arch = "wasm32"))]
use rspice_app_workbench::workbench::localization::MessageCatalog;
use rspice_app_workbench::workbench::localization::MessageId;

#[cfg(target_arch = "wasm32")]
thread_local! {
    /// What the browser's picker delivered for an import of decks, with the
    /// lease it was asked under, until a frame takes it.
    static DECK_IMPORT: std::cell::RefCell<
        Option<(
            rspice_app_services::workbench::browser::file_import::TextImportToken,
            Result<Option<Vec<(String, Vec<u8>)>>, String>,
        )>,
    > = const { std::cell::RefCell::new(None) };

    /// What the browser's picker delivered for Add file…, with the lease it
    /// was asked under and the line's request, until a frame takes it.
    static DECK_FILE_PICK: std::cell::RefCell<
        Option<(
            rspice_app_services::workbench::browser::file_import::TextImportToken,
            rspice_app_state::state::netlist_diagnostics::DeckFileRequest,
            Result<Option<rspice_app_services::workbench::browser::file_import::PickedTextFile>, String>,
        )>,
    > = const { std::cell::RefCell::new(None) };
}

/// Opening a deck file, over the whole application state.
impl AppState {
    /// Import deck…, where there is no file system: ask for SPICE files, or
    /// a ZIP of them, and bring the decks among them into the project as
    /// text once the browser has read them.
    #[cfg(target_arch = "wasm32")]
    pub(crate) fn open_deck(&mut self) -> bool {
        use rspice_app_services::workbench::browser::file_import;
        let new_project = !self.platform.project_lifecycle.is_open();
        if let Some(reason) = self.deck_open_refusal(new_project) {
            self.prompt(reason);
            return false;
        }
        let token =
            match file_import::try_begin_text_import(file_import::BrowserTextImportKind::Netlist) {
                Ok(token) => token,
                Err(error) => {
                    self.prompt(error);
                    return false;
                }
            };
        file_import::pick_files(
            self.workbench
                .messages()
                .text(MessageId::NetlistDecksAndZipFilter),
            DECK_IMPORT_EXTENSIONS,
            move |result| {
                if file_import::text_import_is_current(token) {
                    DECK_IMPORT.with(|slot| *slot.borrow_mut() = Some((token, result)));
                }
            },
        );
        true
    }

    /// Bring in the decks the browser's picker read, or the file Add file…
    /// asked for, once it has.
    #[cfg(target_arch = "wasm32")]
    pub(crate) fn poll_deck_import(&mut self) {
        if let Some((token, request, result)) = DECK_FILE_PICK.with(|slot| slot.borrow_mut().take())
            && rspice_app_services::workbench::browser::file_import::finish_text_import(token)
        {
            let said = match result {
                Ok(Some(picked)) => {
                    let bytes = picked
                        .original_bytes
                        .unwrap_or_else(|| picked.contents.into_bytes());
                    self.add_deck_file_bytes(&request, &picked.name, &bytes)
                }
                Ok(None) => Ok(String::new()),
                Err(error) => Err(error),
            };
            let (Ok(said) | Err(said)) = said;
            if !said.is_empty() {
                self.prompt(said);
            }
        }
        let Some((token, result)) = DECK_IMPORT.with(|slot| slot.borrow_mut().take()) else {
            return;
        };
        if !rspice_app_services::workbench::browser::file_import::finish_text_import(token) {
            return;
        }
        match result {
            Ok(Some(files)) => {
                self.import_deck_files(files);
            }
            Ok(None) => {}
            Err(error) => self.prompt(error),
        }
    }

    /// Bring the decks among `files`, picked together or one ZIP, into the
    /// project as text, in a new project when none is open, and the first
    /// to the front.
    #[cfg(any(target_arch = "wasm32", test))]
    pub(crate) fn import_deck_files(&mut self, files: Vec<(String, Vec<u8>)>) -> bool {
        let decks = match rspice_app_code::workbench::workflows::deck_import::import_files(
            files,
            self.workbench.messages(),
        ) {
            Ok(decks) => decks,
            Err(error) => {
                self.prompt(error);
                return false;
            }
        };
        let new_project = !self.platform.project_lifecycle.is_open();
        if let Some(reason) = self.deck_open_refusal(new_project) {
            self.prompt(reason);
            return false;
        }
        let mut candidate = self.clone();
        if new_project {
            candidate.create_netlist_project(&decks[0].name);
        }
        let messages = self.workbench.messages();
        let names = match rspice_app_code::workbench::documents::deck_files::add_stored_decks(
            &mut candidate.model,
            &mut candidate.code.netlist,
            &mut candidate.code.netlist_open_documents,
            &mut candidate.runs,
            decks,
            messages,
        ) {
            Ok(names) => names,
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
            messages.format(MessageId::NetlistImported, &[("names", &names.join(", "))]),
        );
        true
    }

    /// Add file…: ask for the file `request` names, a file a line of the
    /// open deck kept as text names and the deck does not keep, and keep it
    /// with the deck where the line reads it. What was added, or why
    /// nothing was; nothing when the picker was closed.
    #[cfg(not(target_arch = "wasm32"))]
    pub(crate) fn add_deck_file(
        &mut self,
        request: rspice_app_state::state::netlist_diagnostics::DeckFileRequest,
    ) -> Result<String, String> {
        let messages = self.workbench.messages();
        let Some(path) =
            rspice_app_code::workbench::workflows::netlist_workflow::show_add_deck_file_dialog(
                &request.requested,
                messages,
            )
        else {
            return Ok(String::new());
        };
        let bytes = std::fs::read(&path).map_err(|error| {
            messages.format(
                MessageId::NetlistFileUnreadable,
                &[
                    ("name", &path.display().to_string()),
                    ("error", &error.to_string()),
                ],
            )
        })?;
        let name = path.file_name().map_or_else(
            || path.display().to_string(),
            |name| name.to_string_lossy().into_owned(),
        );
        self.add_deck_file_bytes(&request, &name, &bytes)
    }

    /// Add file…, where there is no file system: ask the browser's picker
    /// for the file `request` names; it is kept with the deck once read.
    #[cfg(target_arch = "wasm32")]
    pub(crate) fn add_deck_file(
        &mut self,
        request: rspice_app_state::state::netlist_diagnostics::DeckFileRequest,
    ) -> Result<String, String> {
        use rspice_app_services::workbench::browser::file_import;
        let token =
            file_import::try_begin_text_import(file_import::BrowserTextImportKind::Netlist)?;
        let keep_as = request.keep_as.clone();
        let messages = self.workbench.messages();
        file_import::pick_text_file(
            messages.text(MessageId::NetlistSpiceFilesFilter),
            DECK_FILE_EXTENSIONS,
            move |result| {
                if file_import::text_import_is_current(token) {
                    DECK_FILE_PICK.with(|slot| *slot.borrow_mut() = Some((token, request, result)));
                }
            },
        );
        Ok(messages.format(MessageId::NetlistPickFileToKeep, &[("path", &keep_as)]))
    }

    /// Keep `bytes`, the file `name` picked for `request`, with the open
    /// deck kept as text.
    pub(crate) fn add_deck_file_bytes(
        &mut self,
        request: &rspice_app_state::state::netlist_diagnostics::DeckFileRequest,
        name: &str,
        bytes: &[u8],
    ) -> Result<String, String> {
        let (text, encoding) = rspice_app_code::workbench::workflows::deck_import::picked_text(
            name,
            bytes,
            self.workbench.messages(),
        )?;
        rspice_app_code::workbench::documents::deck_files::add_file_to_deck(
            &mut self.model,
            &mut self.code.netlist,
            request,
            text,
            encoding,
            self.workbench.messages(),
        )
    }

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
            rspice_app_code::workbench::workflows::netlist_workflow::show_open_deck_dialog(
                self.workbench.messages(),
            )
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
                    self.workbench.messages(),
                )
            else {
                return false;
            };
            self.new_deck_at(&path, new_project)
        }
    }

    /// Save deck as…: ask where the open deck goes, write it there and read
    /// it from there from now on; a deck the project keeps as text is
    /// written as files, the deck and those kept with it.
    pub(crate) fn save_deck_as(&mut self) -> bool {
        #[cfg(target_arch = "wasm32")]
        {
            self.prompt(
                self.workbench
                    .messages()
                    .text(MessageId::NetlistDecksSavedOnDesktop),
            );
            false
        }
        #[cfg(not(target_arch = "wasm32"))]
        {
            let messages = self.workbench.messages();
            let Some(deck) = self.code.netlist.open_deck.as_ref() else {
                self.prompt(messages.text(MessageId::NetlistNoDeckOpen));
                return false;
            };
            let folder = if deck.is_stored() {
                self.model
                    .workspace
                    .content
                    .project
                    .data_root()
                    .map(std::path::Path::to_path_buf)
                    .or_else(dirs::document_dir)
            } else {
                deck.root.path.parent().map(std::path::Path::to_path_buf)
            };
            let Some(path) =
                rspice_app_code::workbench::workflows::netlist_workflow::show_deck_save_dialog(
                    &messages.text(if deck.is_stored() {
                        MessageId::NetlistSaveDeckAsFilesTitle
                    } else {
                        MessageId::NetlistSaveDeckAsTitle
                    }),
                    &deck.root.name(),
                    folder.as_deref(),
                    messages,
                )
            else {
                return false;
            };
            self.save_deck_as_at(&path)
        }
    }

    /// Export deck… or, `with_files`, Export deck and files (ZIP)…: the open
    /// deck the project keeps as text, alone or with its files as one ZIP,
    /// downloaded by the browser, or on the desktop written where the
    /// reader says.
    pub(crate) fn export_stored_deck(&mut self, with_files: bool) -> bool {
        let (name, bytes) =
            match rspice_app_code::workbench::documents::deck_files::export_stored_deck(
                &self.model,
                &self.code.netlist,
                with_files,
                self.workbench.messages(),
            ) {
                Ok(export) => export,
                Err(error) => {
                    self.prompt(error);
                    return false;
                }
            };
        #[cfg(target_arch = "wasm32")]
        let written = rspice_app_services::workbench::browser::download::download_bytes_file(
            std::path::Path::new(&name),
            &bytes,
            if with_files {
                "application/zip"
            } else {
                "text/plain"
            },
        );
        #[cfg(not(target_arch = "wasm32"))]
        let written = {
            use rspice_app_code::workbench::workflows::netlist_workflow as pickers;
            let messages = self.workbench.messages();
            let folder = self
                .model
                .workspace
                .content
                .project
                .data_root()
                .map(std::path::Path::to_path_buf)
                .or_else(dirs::document_dir);
            let picked = if with_files {
                pickers::show_zip_save_dialog(
                    &messages.text(MessageId::NetlistExportDeckAndFilesTitle),
                    &name,
                    folder.as_deref(),
                    messages,
                )
            } else {
                pickers::show_deck_save_dialog(
                    &messages.text(MessageId::NetlistExportDeckTitle),
                    &name,
                    folder.as_deref(),
                    messages,
                )
            };
            let Some(path) = picked else {
                return false;
            };
            std::fs::write(&path, &bytes).map_err(|error| not_written(messages, &path, &error))
        };
        match written {
            Ok(()) => {
                self.prompt(
                    self.workbench
                        .messages()
                        .format(MessageId::NetlistExported, &[("name", &name)]),
                );
                true
            }
            Err(error) => {
                self.prompt(error);
                false
            }
        }
    }

    /// Write the open deck to `path` and read it from there from now on.
    #[cfg(not(target_arch = "wasm32"))]
    pub(crate) fn save_deck_as_at(&mut self, path: &std::path::Path) -> bool {
        match rspice_app_code::workbench::documents::deck_files::save_deck_as(
            &mut self.model,
            &mut self.code.netlist,
            &mut self.code.netlist_open_documents,
            &mut self.runs,
            path,
            self.workbench.messages(),
        ) {
            Ok(name) => {
                self.prompt(
                    self.workbench
                        .messages()
                        .format(MessageId::NetlistSavedDeckAs, &[("name", &name)]),
                );
                true
            }
            Err(error) => {
                self.prompt(error);
                false
            }
        }
    }

    /// Save netlist as deck…: write the generated netlist to a new deck and
    /// open it; on the desktop a file, asked where, and in the browser text
    /// the project keeps. Nothing links the deck back to the design.
    pub(crate) fn save_generated_as_deck(&mut self) -> bool {
        if self
            .model
            .netlist_artifacts
            .generated_source
            .trim()
            .is_empty()
        {
            self.prompt(
                self.workbench
                    .messages()
                    .text(MessageId::NetlistGenerateBeforeSavingDeck),
            );
            return false;
        }
        #[cfg(target_arch = "wasm32")]
        {
            let text = self.generated_deck_text();
            self.add_stored_deck(Some(text))
        }
        #[cfg(not(target_arch = "wasm32"))]
        {
            if let Some(reason) = self.generated_deck_refusal() {
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
            let name = format!("{}.cir", self.model.export_file_stem("deck"));
            let Some(path) =
                rspice_app_code::workbench::workflows::netlist_workflow::show_deck_save_dialog(
                    &self
                        .workbench
                        .messages()
                        .text(MessageId::NetlistSaveNetlistAsDeckTitle),
                    &name,
                    folder.as_deref(),
                    self.workbench.messages(),
                )
            else {
                return false;
            };
            self.save_generated_as_deck_at(&path)
        }
    }

    /// Write the generated netlist to `path` as a deck, its first line
    /// naming the design and when, and open it as a deck of the project.
    #[cfg(not(target_arch = "wasm32"))]
    pub(crate) fn save_generated_as_deck_at(&mut self, path: &std::path::Path) -> bool {
        if let Some(reason) = self.generated_deck_refusal() {
            self.prompt(reason);
            return false;
        }
        if let Err(error) = std::fs::write(path, self.generated_deck_text()) {
            self.prompt(not_written(self.workbench.messages(), path, &error));
            return false;
        }
        let new_project = !self.platform.project_lifecycle.is_open();
        self.open_deck_at(path, new_project)
    }

    /// Why the generated netlist cannot become a deck now: a run in
    /// progress, a project open read-only, or a deck open with edits, which
    /// the new deck would put away.
    #[cfg(not(target_arch = "wasm32"))]
    fn generated_deck_refusal(&self) -> Option<String> {
        if let Some(reason) = self.deck_open_refusal(false) {
            return Some(reason);
        }
        self.code
            .netlist
            .open_deck
            .as_ref()
            .and_then(|deck| deck.unsaved())
            .map(|unsaved| {
                rspice_app_code::workbench::documents::deck_files::save_first(
                    self.workbench.messages(),
                    unsaved,
                )
            })
    }

    /// The generated netlist as a deck: a first line naming the design and,
    /// in UTC since the file travels, when it was saved.
    fn generated_deck_text(&self) -> String {
        use rspice_app_state::time_compat::{checked_unix_time_ms, utc_stamp};
        rspice_app_code::workbench::documents::deck_files::deck_from_generated(
            &self.model.netlist_artifacts.generated_source,
            &self
                .model
                .export_file_stem(self.model.workspace.content.project.name()),
            &utc_stamp(checked_unix_time_ms().unwrap_or(0)),
        )
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
        if let Err(error) = std::fs::write(path, deck.text.as_str()) {
            self.prompt(not_written(self.workbench.messages(), path, &error));
            return false;
        }
        self.open_deck_at(path, new_project)
    }

    /// Add a new deck the project keeps as text and bring it to the front,
    /// in a new project around it when none is open: what New deck makes
    /// where there is no file system.
    #[cfg(any(target_arch = "wasm32", test))]
    pub(crate) fn new_stored_deck(&mut self) -> bool {
        self.add_stored_deck(None)
    }

    /// Add a deck the project keeps as text, a title line and `.end` or
    /// `text`, and bring it to the front, in a new project around it when
    /// none is open.
    #[cfg(any(target_arch = "wasm32", test))]
    fn add_stored_deck(&mut self, text: Option<String>) -> bool {
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
        let messages = self.workbench.messages();
        let name = match rspice_app_code::workbench::documents::deck_files::new_stored_deck(
            &mut candidate.model,
            &mut candidate.code.netlist,
            &mut candidate.code.netlist_open_documents,
            &mut candidate.runs,
            text,
            messages,
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
            messages.format(MessageId::NetlistCreatedDeck, &[("name", &name)]),
        );
        true
    }

    /// Why no deck can be opened now: a run in progress, or a project open
    /// read-only that the deck would be added to.
    fn deck_open_refusal(&self, new_project: bool) -> Option<String> {
        let refusal = if self.model.simulation.execution.has_active_execution() {
            MessageId::NetlistDeckOpenWhileRunning
        } else if !new_project && self.model.safe_mode.project_read_only() {
            MessageId::NetlistDeckAddReadOnly
        } else {
            return None;
        };
        Some(self.workbench.messages().text(refusal))
    }

    /// Open the deck file at `path`, in a new project around it when
    /// `new_project` says so, else in the open project. The application
    /// reads the file away from the frame ([`Self::read_deck_away`]); a file
    /// that does not open is refused now.
    #[cfg(not(target_arch = "wasm32"))]
    pub(crate) fn open_deck_at(&mut self, path: &std::path::Path, new_project: bool) -> bool {
        if !self.may_open_deck(new_project) {
            return false;
        }
        let shown = std::path::absolute(path).is_ok_and(|path| {
            self.code
                .netlist
                .open_deck
                .as_ref()
                .is_some_and(|open| open.root.path == path)
        });
        // The deck already open comes to the front without a second read.
        if !shown && rspice_app_code::workbench::documents::deck_files::reads_decks_away() {
            return self.read_deck_away(path, new_project);
        }
        self.open_deck_with(path, new_project, |candidate, messages| {
            rspice_app_code::workbench::documents::deck_files::open_deck(
                &mut candidate.model,
                &mut candidate.code.netlist,
                &mut candidate.code.netlist_open_documents,
                &mut candidate.runs,
                path,
                messages,
            )
        })
    }

    /// Open the deck file at `path` as the application does: read away from
    /// the frame, with its tab in front at once, saying it is reading, as a
    /// deck the project lists is opened, in a project made around it now
    /// when `new_project` says so. Should the file not read as text, the
    /// frame its read lands in lets it go, and a project made for it gives
    /// way to what was there before ([`Self::poll_deck_open`]).
    #[cfg(not(target_arch = "wasm32"))]
    fn read_deck_away(&mut self, path: &std::path::Path, new_project: bool) -> bool {
        use rspice_app_code::workbench::documents::deck_files;
        let messages = self.workbench.messages();
        if let Some(refusal) = deck_files::deck_file_refusal(path, messages) {
            self.prompt(refusal);
            return false;
        }
        let name = path.file_name().map_or_else(
            || path.display().to_string(),
            |name| name.to_string_lossy().into_owned(),
        );
        // What comes back should the file not read: the state before the
        // project made for it, or for a deck still read into a project made
        // for that one.
        let opening = self.shell.opening_deck.take().filter(|opening| {
            self.code
                .netlist
                .reading_deck
                .as_ref()
                .is_some_and(|reading| reading.id == opening.id)
        });
        let before = opening
            .and_then(|opening| opening.before)
            .or_else(|| new_project.then(|| Box::new(self.clone())));
        if new_project {
            self.create_netlist_project(&name);
        }
        match deck_files::read_deck_file_away(
            &mut self.model,
            &mut self.code.netlist,
            &mut self.code.netlist_open_documents,
            path,
            messages,
        ) {
            Ok(id) => {
                self.workbench
                    .activate(rspice_app_state::workbench::workspace::Workspace::Netlist);
                self.shell.opening_deck = Some(OpeningDeck { id, name, before });
                true
            }
            Err(refusal) => {
                if let Some(before) = before {
                    self.commit_candidate(*before);
                }
                self.prompt(refusal);
                false
            }
        }
    }

    /// Follow a deck file opened from disk until its read lands: once it
    /// opened, the log says so. Should its file not have read as text, its
    /// open was undone as the read landed; a project made for it gives way
    /// now to what was there before, and the page says why.
    #[cfg(not(target_arch = "wasm32"))]
    pub(crate) fn poll_deck_open(&mut self) {
        let Some(id) = self.shell.opening_deck.as_ref().map(|opening| opening.id) else {
            return;
        };
        if self
            .code
            .netlist
            .reading_deck
            .as_ref()
            .is_some_and(|reading| reading.id == id)
        {
            return;
        }
        let Some(opening) = self.shell.opening_deck.take() else {
            return;
        };
        let refused = self.code.netlist.refused_open.take();
        if let Some((_, refusal)) = refused.filter(|(refused, _)| *refused == id) {
            if let Some(before) = opening.before {
                self.commit_candidate(*before);
                self.shell.opening_deck = None;
            }
            self.prompt(refusal);
            return;
        }
        if self
            .code
            .netlist
            .open_deck
            .as_ref()
            .is_some_and(|deck| deck.id == id)
        {
            let messages = self.workbench.messages();
            self.record(
                LogSeverity::Info,
                LogSource::Netlist,
                messages.format(MessageId::NetlistOpenedDeck, &[("name", &opening.name)]),
            );
        }
    }

    /// Whether a deck may open now, else the page says why not: no run in
    /// progress, no read-only project it would be added to, and no deck open
    /// with edits it would put away.
    #[cfg(not(target_arch = "wasm32"))]
    fn may_open_deck(&mut self, new_project: bool) -> bool {
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
            let refusal = rspice_app_code::workbench::documents::deck_files::save_first(
                self.workbench.messages(),
                unsaved,
            );
            self.prompt(refusal);
            return false;
        }
        true
    }

    /// Open the deck file at `path` with `open`, on a copy of the whole
    /// application state, in a new project around it when `new_project`
    /// says so: the copy replaces the state only once the deck has opened.
    #[cfg(not(target_arch = "wasm32"))]
    fn open_deck_with(
        &mut self,
        path: &std::path::Path,
        new_project: bool,
        open: impl FnOnce(&mut Self, MessageCatalog) -> Result<(), String>,
    ) -> bool {
        let name = path.file_name().map_or_else(
            || path.display().to_string(),
            |name| name.to_string_lossy().into_owned(),
        );
        let mut candidate = self.clone();
        if new_project {
            candidate.create_netlist_project(&name);
        }
        let messages = self.workbench.messages();
        if let Err(error) = open(&mut candidate, messages) {
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
            messages.format(MessageId::NetlistOpenedDeck, &[("name", &name)]),
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
            self.prompt(
                self.workbench
                    .messages()
                    .text(MessageId::NetlistDropOneDeck),
            );
            return false;
        };
        #[cfg(not(target_arch = "wasm32"))]
        {
            let Some(path) = file.path.clone() else {
                self.prompt(
                    self.workbench
                        .messages()
                        .text(MessageId::NetlistDropNotAFile),
                );
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

/// A deck file opened from disk while it is read away from the frame.
#[cfg(not(target_arch = "wasm32"))]
#[derive(Clone)]
pub(crate) struct OpeningDeck {
    /// The deck, whose tab is in front saying it is reading.
    pub(crate) id: rspice_app_state::state::DeckId,
    /// Its file's name, which the log says once it opened.
    name: String,
    /// What stood before the project made around it, which comes back
    /// should its file not read as text; `None` when it opened into the
    /// project open before.
    before: Option<Box<AppState>>,
}

#[cfg(not(target_arch = "wasm32"))]
impl std::fmt::Debug for OpeningDeck {
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        formatter
            .debug_struct("OpeningDeck")
            .field("id", &self.id)
            .field("name", &self.name)
            .field("made_a_project", &self.before.is_some())
            .finish()
    }
}

/// What Import deck… offers the browser's picker: SPICE files, and ZIPs of
/// them.
#[cfg(target_arch = "wasm32")]
const DECK_IMPORT_EXTENSIONS: &[&str] = &[
    "cir", "sp", "spi", "spice", "net", "ckt", "cki", "scs", "lib", "inc", "txt", "zip",
];

/// What Add file… offers the browser's picker: the files a deck's include
/// and library lines read.
#[cfg(target_arch = "wasm32")]
const DECK_FILE_EXTENSIONS: &[&str] = &[
    "inc", "lib", "mod", "mdl", "sub", "cir", "sp", "spi", "spice", "net", "ckt", "cki", "scs",
    "txt",
];

/// Why the file at `path` was not written.
#[cfg(not(target_arch = "wasm32"))]
fn not_written(messages: MessageCatalog, path: &std::path::Path, error: &std::io::Error) -> String {
    messages.format(
        MessageId::NetlistFileNotWritten,
        &[
            ("path", &path.display().to_string()),
            ("error", &error.to_string()),
        ],
    )
}
