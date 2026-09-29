//! The data files file-backed sources read.
//!
//! A `PWL FILE=` card names a table, and three questions about it are asked
//! while a deck is written: which file the card is pointed at, whether that
//! file will load, and whether the run should be told the table came from
//! somewhere other than where the card says. The rule that answers the first
//! belongs to the supplied host, shared with every preview; this binds it
//! to what the generator was given — the project's data
//! folder and its stimulus library — so a run and a preview cannot read
//! different files.

use super::*;
use rspice_design::stimulus_library::definition::RetainedPwlFile;
use std::path::Path;

/// Host operations needed to resolve and validate file-backed source tables.
pub trait NetlistSourceHost {
    fn route_retaining(
        &self,
        stored: &str,
        data_root: Option<&Path>,
        table: Option<&RetainedPwlFile>,
    ) -> (TableRoute, Option<String>);
    fn is_file(&self, path: &str) -> std::io::Result<bool>;
    fn engine_refusal(&self, path: &str) -> Option<String>;
    fn named_file_matches(&self, path: &str, table: &RetainedPwlFile) -> Option<bool>;
}

/// The file a card's table is read from, and whose it is.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum TableRoute {
    /// The file the card names, resolved against the project's data folder.
    Named(String),
    /// The definition's retained copy, because the named file is not there.
    Retained(String),
}

impl TableRoute {
    /// The path the engine is given.
    #[must_use]
    pub fn path(&self) -> &str {
        match self {
            Self::Named(path) | Self::Retained(path) => path,
        }
    }
}

/// File-backed stimulus inputs for this generation. Inspection may omit both.
#[derive(Clone, Copy)]
pub struct NetlistSourceData<'a> {
    /// File authority explicitly supplied by the execution or inspection host.
    pub files: &'a dyn NetlistSourceHost,
    /// Directory against which project-relative data references resolve.
    pub data_root: Option<&'a Path>,
    /// Authored retained tables used when their named file is unavailable.
    pub stimulus_library: Option<&'a rspice_design::stimulus_library::library::StimulusLibrary>,
}

impl<'a> NetlistSourceData<'a> {
    /// Bind a file host without project-relative paths or retained source tables.
    pub fn new(files: &'a dyn NetlistSourceHost) -> Self {
        Self {
            files,
            data_root: None,
            stimulus_library: None,
        }
    }

    fn retained_table(
        &self,
        component: &Component,
    ) -> Option<&'a rspice_design::stimulus_library::definition::RetainedPwlFile> {
        self.stimulus_library?.retained_pwl_table(component)
    }
}

impl<'a> NetlistGenerator<'a> {
    /// The data-file reference a file-backed source stores.
    fn stored_data_file<'c>(
        component: &'c Component,
        params: &'c std::collections::HashMap<String, String>,
    ) -> &'c str {
        params
            .get("file")
            .map_or(component.value.as_str(), String::as_str)
            .trim()
    }

    /// The file a source's stored data-file reference is read from.
    ///
    /// Project files record the path relative to the project folder so a design
    /// survives being moved or handed to someone else; the engine opens what it
    /// is given and does not resolve against the deck, so the reference is made
    /// absolute against the bound data root. When the file it names is not
    /// there and the source's stimulus definition retains the table, the route
    /// is that retained copy instead.
    ///
    /// The second half is why a retained copy could not stand in, when it
    /// could not.
    pub(super) fn source_table_route(
        &self,
        component: &Component,
        stored: &str,
    ) -> (TableRoute, Option<String>) {
        self.source_data.files.route_retaining(
            stored,
            self.source_data.data_root,
            self.source_data.retained_table(component),
        )
    }

    /// Reason a file-backed PWL source cannot run, or `None` when its table is
    /// present — as the file the card names, or as the copy its stimulus
    /// definition retains — and is one the engine's loader reads.
    ///
    /// The engine already refuses to build a circuit whose PWL file will not
    /// load, but that happens after a run has been dispatched and reports a
    /// resolved absolute path the user never typed. Catching it here names the
    /// component and blocks the run before it starts.
    pub(super) fn pwl_data_file_defect(
        &self,
        component: &Component,
        params: &std::collections::HashMap<String, String>,
    ) -> Option<String> {
        let stored = Self::stored_data_file(component, params);
        if stored.is_empty() {
            return Some(format!(
                "{} '{}' has no data file selected",
                component.kind.display_name(),
                component.name
            ));
        }

        let (route, retained_failure) = self.source_table_route(component, stored);
        let unloadable = |path: &str| {
            self.source_data.files.engine_refusal(path).map(|refusal| {
                format!(
                    "{} '{}' data file '{}' is not a table the engine reads: {}",
                    component.kind.display_name(),
                    component.name,
                    stored,
                    refusal
                )
            })
        };
        // A definition's retained copy standing in for the file is a run that
        // can happen, and `pwl_table_notice` says that it did.
        let resolved = match route {
            TableRoute::Named(resolved) => resolved,
            TableRoute::Retained(retained) => return unloadable(&retained),
        };
        // Only a bound data root makes the reference checkable: without one a
        // relative path is resolved by the engine against its own working
        // directory, which is not this process's to test.
        if std::path::Path::new(&resolved).is_relative() {
            return None;
        }
        let retained_failure = retained_failure
            .map(|failure| format!("; {failure}"))
            .unwrap_or_default();
        match self.source_data.files.is_file(&resolved) {
            Ok(true) => unloadable(&resolved),
            Ok(false) => Some(format!(
                "{} '{}' data file '{}' is a directory{retained_failure}",
                component.kind.display_name(),
                component.name,
                stored
            )),
            Err(error) => Some(format!(
                "{} '{}' cannot read data file '{}': {}{retained_failure}",
                component.kind.display_name(),
                component.name,
                stored,
                error
            )),
        }
    }

    /// What a run should be told about where a file-backed source's table came
    /// from, or `None` when it is simply the file the card names.
    ///
    /// Two things are worth a line in the log. The run read the definition's
    /// retained copy because the named file is not reachable: the deck then
    /// carries a cache path nobody typed, and this is what explains it. Or the
    /// named file is there and no longer holds the bytes the definition
    /// retains: the run reads the file, as the card says, and the definition's
    /// digest describes something else.
    pub(super) fn pwl_table_notice(
        &self,
        component: &Component,
        params: &std::collections::HashMap<String, String>,
    ) -> Option<String> {
        let table = self.source_data.retained_table(component)?;
        let definition = &component.stimulus_provenance.as_ref()?.definition;
        let stored = Self::stored_data_file(component, params);
        match self.source_table_route(component, stored).0 {
            TableRoute::Retained(_) => Some(format!(
                "{} '{}' reads the copy of '{}' that stimulus definition '{definition}' retains: \
                 the file the card names, '{stored}', is not reachable here",
                component.kind.display_name(),
                component.name,
                table.file_name,
            )),
            TableRoute::Named(named) => (self.source_data.files.named_file_matches(&named, table)
                == Some(false))
            .then(|| {
                format!(
                    "{} '{}' reads '{stored}', which no longer holds the bytes stimulus \
                             definition '{definition}' retains; the run uses the file",
                    component.kind.display_name(),
                    component.name,
                )
            }),
        }
    }
}
