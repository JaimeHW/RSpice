//! The files one compile of a deck's HDL reads from the host, and the sealed
//! bundle that holds their bytes.
//!
//! A source's `` `include `` headers are read where the engine's compiler
//! reads them, beside the file that includes them, and every read goes
//! through the deck's include chain so the run records it as a dependency.
//! The bundle names each file by its path below the deepest directory that
//! holds them all, so an `` `include `` resolves in the bundle to the file it
//! resolved to on the host; a closure the bundle resolves differently is
//! refused rather than compiled from other bytes.

use std::cell::RefCell;
use std::path::{Component, Path, PathBuf};

use rspice_core::abort_signal::NoAbort;
use rspice_core::netlist::{IncludeProcessor, ParseError, ParseWithAbortError};
use rspice_veriloga::preprocessor::MacroDef;
use rspice_veriloga::{
    Preprocessor, PreprocessorError, SourceDocument, SourceDocumentOrigin, SourceProvider,
    VirtualSourceBundle, VirtualSourceFile,
};

/// One file a compile reads: its canonical host path and its text.
#[derive(Debug, Clone)]
pub(super) struct HostSource {
    pub(super) path: PathBuf,
    pub(super) text: String,
}

/// The files a compile of `roots`, in order and under `defines`, reads from
/// the host: each root, then each header it includes, in the order the
/// preprocessor first loads them. Macros carry from one root into the next,
/// as they do in a compilation unit, so a header a later root includes only
/// under an earlier root's macro is found. A header is read through
/// `processor`, recording it as a dependency of the run.
pub(super) fn read_closure(
    processor: &mut IncludeProcessor,
    roots: &[HostSource],
    defines: &[(String, Option<String>)],
) -> Result<Vec<HostSource>, String> {
    let provider = HostHeaders {
        processor: RefCell::new(processor),
        roots,
        read: RefCell::new(Vec::new()),
    };
    let mut preprocessor = Preprocessor::new();
    for (name, value) in defines {
        preprocessor.define(name, MacroDef::simple(value.as_deref().unwrap_or("")));
    }
    for root in roots {
        preprocessor
            .preprocess_provider_root(&provider, &root.path)
            .map_err(|error| error.to_string())?;
    }
    Ok(provider.read.into_inner())
}

/// A source provider over the host that reads a header as the engine's
/// compiler does, beside the file that includes it, and through the deck's
/// include chain, so every byte it serves is a recorded dependency.
struct HostHeaders<'a, 'p> {
    processor: RefCell<&'p mut IncludeProcessor>,
    roots: &'a [HostSource],
    read: RefCell<Vec<HostSource>>,
}

impl HostHeaders<'_, '_> {
    fn record(&self, path: &Path, text: &str) {
        let mut read = self.read.borrow_mut();
        if !read.iter().any(|source| source.path == path) {
            read.push(HostSource {
                path: path.to_path_buf(),
                text: text.to_owned(),
            });
        }
    }
}

impl SourceProvider for HostHeaders<'_, '_> {
    fn load_root(&self, requested: &Path) -> Result<SourceDocument, PreprocessorError> {
        let root = self
            .roots
            .iter()
            .find(|root| root.path == requested)
            .ok_or_else(|| {
                PreprocessorError::new(
                    format!("'{}' is not a source of this compile", requested.display()),
                    Some(requested.to_path_buf()),
                    0,
                )
            })?;
        self.record(&root.path, &root.text);
        Ok(SourceDocument::provided(&root.path, &root.text))
    }

    fn resolve_include(
        &self,
        including_file: Option<&Path>,
        include_paths: &[PathBuf],
        requested: &str,
    ) -> Result<Option<SourceDocument>, PreprocessorError> {
        let candidates = including_file
            .and_then(Path::parent)
            .into_iter()
            .chain(include_paths.iter().map(PathBuf::as_path))
            .map(|directory| directory.join(requested));
        for candidate in candidates {
            // A broken link or an unreadable entry holds its place in the
            // search, as it does for the engine's compiler.
            match std::fs::symlink_metadata(&candidate) {
                Err(error) if error.kind() == std::io::ErrorKind::NotFound => continue,
                Err(error) => return Err(PreprocessorError::from_io(error, &candidate)),
                Ok(_) => {}
            }
            let owner = including_file.unwrap_or(&candidate);
            if Path::new(requested).is_absolute() {
                return Err(PreprocessorError::new(
                    format!(
                        "`include \"{requested}\" names its header by an absolute path; a sealed \
                         source names its headers relative to itself"
                    ),
                    including_file.map(Path::to_path_buf),
                    include_line(owner, requested),
                ));
            }
            let dependency = self
                .processor
                .borrow_mut()
                .read_verilog_source_with_abort(
                    owner,
                    include_line(owner, requested),
                    &candidate.to_string_lossy(),
                    &NoAbort,
                )
                .map_err(|error| {
                    let detail = match error {
                        ParseWithAbortError::Parse(ParseError::Syntax { message, .. }) => message,
                        other => other.to_string(),
                    };
                    PreprocessorError::new(detail, Some(candidate.clone()), 0)
                })?;
            let path = dependency.resolved_path();
            self.record(path, dependency.source());
            return Ok(Some(SourceDocument::provided(path, dependency.source())));
        }
        Ok(None)
    }
}

/// The 1-based line of `owner` whose `` `include `` names `requested`, or 0.
fn include_line(owner: &Path, requested: &str) -> usize {
    std::fs::read_to_string(owner)
        .ok()
        .and_then(|text| {
            text.lines()
                .position(|line| line.contains("`include") && line.contains(requested))
        })
        .map_or(0, |index| index + 1)
}

/// The sealed bundle of `sources`, each named by its path below the deepest
/// directory that holds them all; the first is the bundle's root. Returns the
/// bundle and each source's name in it, in order.
pub(super) fn bundle(sources: &[HostSource]) -> Result<(VirtualSourceBundle, Vec<String>), String> {
    let mut common: Option<Vec<Component<'_>>> = None;
    for source in sources {
        let parent: Vec<Component<'_>> = source
            .path
            .parent()
            .map_or_else(Vec::new, |parent| parent.components().collect());
        common = Some(match common {
            None => parent,
            Some(common) => common
                .into_iter()
                .zip(parent)
                .take_while(|(left, right)| left == right)
                .map(|(left, _)| left)
                .collect(),
        });
    }
    let common: PathBuf = common.unwrap_or_default().into_iter().collect();
    let names = sources
        .iter()
        .map(|source| {
            let relative = source.path.strip_prefix(&common).map_err(|_| {
                format!(
                    "'{}' shares no directory with the other sources of its compile",
                    source.path.display()
                )
            })?;
            let mut name = String::new();
            for component in relative.components() {
                let Component::Normal(part) = component else {
                    return Err(format!(
                        "'{}' cannot be named inside a sealed bundle",
                        source.path.display()
                    ));
                };
                if !name.is_empty() {
                    name.push('/');
                }
                name.push_str(&part.to_string_lossy());
            }
            Ok(name)
        })
        .collect::<Result<Vec<_>, String>>()?;
    let root = names.first().cloned().unwrap_or_default();
    let bundle = VirtualSourceBundle::new(
        root,
        sources
            .iter()
            .zip(&names)
            .map(|(source, name)| VirtualSourceFile::new(name.clone(), source.text.clone())),
    )
    .map_err(|error| error.to_string())?;
    Ok((bundle, names))
}

/// Whether the bundle's preparation read exactly the files the host
/// preparation did, in the same order: otherwise an `` `include `` resolved
/// to another file in the bundle than on the host. Names the first that
/// differs.
pub(super) fn same_closure(
    prepared: &rspice_veriloga::PreparedVirtualSource,
    names: &[String],
) -> Result<(), String> {
    let compiled: Vec<&str> = prepared
        .dependency_closure()
        .iter()
        .filter(|document| document.origin == SourceDocumentOrigin::Provider)
        .map(|document| document.logical_path.as_str())
        .collect();
    if compiled
        .iter()
        .copied()
        .eq(names.iter().map(String::as_str))
    {
        return Ok(());
    }
    let differs = compiled
        .iter()
        .zip(names)
        .find(|(compiled, read)| *compiled != read)
        .map_or_else(
            || format!("{} files against {}", compiled.len(), names.len()),
            |(compiled, read)| format!("'{compiled}' where the host read '{read}'"),
        );
    Err(format!(
        "an `include resolves differently from sealed bytes than from the host ({differs}); \
         name each header relative to the file that includes it"
    ))
}
