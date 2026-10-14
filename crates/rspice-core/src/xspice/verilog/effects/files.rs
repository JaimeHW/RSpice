//! HDL file I/O (IEEE 1364-2005 sections 17.2.1 to 17.2.8), as the host
//! performs it (design P2 section 4.3).
//!
//! # Where a path goes
//!
//! A relative path resolves against the run directory: the deck's directory
//! on the command line, an explicit `--hdl-run-dir` or `.options hdl_rundir`,
//! the root of the browser worker's virtual file system, or what a testbench
//! caller names. Reads may name any path the host can read, as `.include`
//! does; in the browser that is the virtual data-file registry only. Writes
//! stay inside the run directory's subtree unless the run lifts the
//! confinement: a path that leaves it fails the way section 17.2.1 prescribes
//! for a file that cannot be opened (`$fopen` returns 0), with a located
//! warning.
//!
//! # Accepted-only
//!
//! Descriptor numbers, positions and contents are host state, so they are
//! deterministic and a rejected trial's clone restore takes them back. No
//! write reaches a file while a trial is open: each one is journaled as a
//! [`FileOp`], and the journal is applied to the run's [`HdlFileStore`] only
//! when the trial commits, together with the output records. A read-mode
//! `$fopen` snapshots the whole file when it opens, seen through the writes
//! the design has journaled but not yet released, so a retried trial reads
//! exactly what the first attempt read.
//!
//! # Descriptors
//!
//! Section 17.2.1's two kinds, numbered the way Icarus Verilog numbers them:
//! a file descriptor has bit 31 set, with 0, 1 and 2 the standard input,
//! output and error and opened files from 3, lowest free first; a
//! multichannel descriptor is one bit of 1 to 30, with bit 0 the standard
//! output.

mod calls;
pub(crate) mod readmem;
pub(crate) mod scan;

use std::collections::BTreeMap;
use std::path::{Component, Path, PathBuf};
use std::sync::{Arc, Mutex};

use serde::{Deserialize, Serialize};

/// Bit 31: a file descriptor rather than a multichannel descriptor.
pub(crate) const FD_FLAG: u32 = 0x8000_0000;
pub(crate) const STDIN: u32 = FD_FLAG;
pub(crate) const STDOUT: u32 = FD_FLAG | 1;
pub(crate) const STDERR: u32 = FD_FLAG | 2;
/// The first descriptor slot an `$fopen` hands out.
const FIRST_SLOT: usize = 3;
/// Multichannel bits 1 to 30 are files; bit 0 is the standard output.
const CHANNEL_BITS: usize = 31;

/// The C `errno` values `$ferror` reports, as Icarus reports them.
pub(crate) const EBADF: i32 = 9;
pub(crate) const ENOENT: i32 = 2;
pub(crate) const EINVAL: i32 = 22;
pub(crate) const EACCES: i32 = 13;
pub(crate) const EIO: i32 = 5;

/// Where a run's file writes land when its trials commit.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(crate) enum HdlFileTarget {
    /// The host's file system, under the run directory.
    Disk,
    /// Kept in the store and returned through the output records: the
    /// browser worker, and a testbench run that is asked not to touch disk.
    Memory,
}

impl HdlFileTarget {
    /// What a run writes to when nobody says: the disk, except in the
    /// browser, which has none.
    pub(crate) const fn platform_default() -> Self {
        if cfg!(target_arch = "wasm32") {
            Self::Memory
        } else {
            Self::Disk
        }
    }
}

/// One journaled change to a file, applied at commit.
#[derive(Debug, Clone, PartialEq, Eq)]
pub(crate) enum FileOp {
    /// Create the file, emptying it under `truncate` (`"w"`, `"w+"`, a
    /// multichannel `$fopen`, a dump file).
    Create { path: PathBuf, truncate: bool },
    /// Write at an offset (positioned handles).
    Write {
        path: PathBuf,
        offset: u64,
        bytes: Vec<u8>,
    },
    /// Write at the end (`"a"` handles).
    Append { path: PathBuf, bytes: Vec<u8> },
    /// Shorten to `length`: a resumed checkpoint discards what was written
    /// after it.
    Truncate { path: PathBuf, length: u64 },
    /// The design closed a handle on the file.
    Close { path: PathBuf },
}

impl FileOp {
    fn path(&self) -> &Path {
        match self {
            Self::Create { path, .. }
            | Self::Write { path, .. }
            | Self::Append { path, .. }
            | Self::Truncate { path, .. }
            | Self::Close { path } => path,
        }
    }

    /// Apply to an in-memory copy of the file.
    fn apply_to(&self, content: &mut Vec<u8>) {
        match self {
            Self::Create { truncate, .. } => {
                if *truncate {
                    content.clear();
                }
            }
            Self::Write { offset, bytes, .. } => {
                let start = usize::try_from(*offset).unwrap_or(usize::MAX);
                let end = start.saturating_add(bytes.len());
                if content.len() < end {
                    content.resize(end, 0);
                }
                content[start..end].copy_from_slice(bytes);
            }
            Self::Append { bytes, .. } => content.extend_from_slice(bytes),
            Self::Truncate { length, .. } => {
                content.truncate(usize::try_from(*length).unwrap_or(usize::MAX));
            }
            Self::Close { .. } => {}
        }
    }
}

/// The files a run's design reads and writes: what every trial of the run
/// shares, and the only thing a commit changes.
#[derive(Debug)]
pub(crate) struct HdlFileStore {
    target: HdlFileTarget,
    state: Mutex<StoreState>,
}

#[derive(Debug, Default)]
struct StoreState {
    /// What a [`HdlFileTarget::Memory`] store holds, by resolved path.
    memory: BTreeMap<PathBuf, Vec<u8>>,
    /// Files a [`HdlFileTarget::Disk`] store keeps open between commits.
    #[cfg(not(target_arch = "wasm32"))]
    open: BTreeMap<PathBuf, std::fs::File>,
}

impl HdlFileStore {
    pub(crate) fn new(target: HdlFileTarget) -> Self {
        Self {
            target,
            state: Mutex::new(StoreState::default()),
        }
    }

    fn state(&self) -> std::sync::MutexGuard<'_, StoreState> {
        self.state
            .lock()
            .unwrap_or_else(std::sync::PoisonError::into_inner)
    }

    /// What a [`HdlFileTarget::Memory`] store holds for `path`.
    #[cfg(test)]
    pub(crate) fn written(&self, path: &Path) -> Option<Vec<u8>> {
        self.state().memory.get(path).cloned()
    }

    /// The committed bytes of a file: written by this run in memory, then
    /// the virtual data-file registry, then (off the browser) the disk.
    /// `Ok(None)` is a file that does not exist.
    pub(crate) fn read(&self, path: &Path, name: &str) -> Result<Option<Vec<u8>>, String> {
        if let Some(bytes) = self.state().memory.get(path) {
            return Ok(Some(bytes.clone()));
        }
        for key in [path_key(path), name.to_owned()] {
            if let Some(contents) = crate::xspice::data_file::registered_contents(&key) {
                return Ok(Some(contents.as_bytes().to_vec()));
            }
        }
        #[cfg(not(target_arch = "wasm32"))]
        {
            let limit = crate::resource::ResourceLimits::default().max_external_data_bytes;
            match crate::resource::read_file_bytes_limited(
                path,
                crate::resource::ResourceKind::ExternalDataBytes,
                limit,
            ) {
                Ok(bytes) => Ok(Some(bytes)),
                Err(crate::resource::ResourceReadError::Io(error))
                    if error.kind() == std::io::ErrorKind::NotFound =>
                {
                    Ok(None)
                }
                Err(error) => Err(error.to_string()),
            }
        }
        #[cfg(target_arch = "wasm32")]
        {
            Ok(None)
        }
    }

    /// Whether a file could be created at `path`: its directory exists.
    pub(crate) fn can_create(&self, path: &Path) -> bool {
        if path.as_os_str().is_empty() || path.file_name().is_none() {
            return false;
        }
        if cfg!(target_arch = "wasm32") {
            return true;
        }
        match path.parent() {
            Some(parent) if !parent.as_os_str().is_empty() => {
                parent.is_dir()
                    || self
                        .state()
                        .memory
                        .keys()
                        .any(|key| key.starts_with(parent))
            }
            _ => true,
        }
    }

    /// Apply a committed journal; the failures it met, for warnings.
    pub(crate) fn apply(&self, ops: Vec<FileOp>) -> Vec<String> {
        let mut state = self.state();
        // A browser build keeps every file in memory, so nothing can fail.
        #[cfg_attr(target_arch = "wasm32", allow(unused_mut))]
        let mut failures = Vec::new();
        for op in ops {
            match self.target {
                HdlFileTarget::Memory => {
                    let content = state.memory.entry(op.path().to_path_buf()).or_default();
                    op.apply_to(content);
                }
                HdlFileTarget::Disk => {
                    #[cfg(not(target_arch = "wasm32"))]
                    if let Err(error) = apply_on_disk(&mut state.open, &op) {
                        failures.push(format!("{}: {error}", op.path().display()));
                    }
                    #[cfg(target_arch = "wasm32")]
                    {
                        let content = state.memory.entry(op.path().to_path_buf()).or_default();
                        op.apply_to(content);
                    }
                }
            }
        }
        failures
    }
}

#[cfg(not(target_arch = "wasm32"))]
fn apply_on_disk(open: &mut BTreeMap<PathBuf, std::fs::File>, op: &FileOp) -> std::io::Result<()> {
    use std::io::{Seek, SeekFrom, Write};
    let handle = |open: &mut BTreeMap<PathBuf, std::fs::File>,
                  path: &Path,
                  truncate: bool|
     -> std::io::Result<()> {
        if truncate || !open.contains_key(path) {
            let file = std::fs::OpenOptions::new()
                .read(true)
                .write(true)
                .create(true)
                .truncate(truncate)
                .open(path)?;
            open.insert(path.to_path_buf(), file);
        }
        Ok(())
    };
    match op {
        FileOp::Create { path, truncate } => handle(open, path, *truncate),
        FileOp::Write {
            path,
            offset,
            bytes,
        } => {
            handle(open, path, false)?;
            let file = open.get_mut(path.as_path()).expect("opened above");
            file.seek(SeekFrom::Start(*offset))?;
            file.write_all(bytes)
        }
        FileOp::Append { path, bytes } => {
            handle(open, path, false)?;
            let file = open.get_mut(path.as_path()).expect("opened above");
            file.seek(SeekFrom::End(0))?;
            file.write_all(bytes)
        }
        FileOp::Truncate { path, length } => {
            handle(open, path, false)?;
            open.get_mut(path.as_path())
                .expect("opened above")
                .set_len(*length)
        }
        FileOp::Close { path } => {
            if let Some(mut file) = open.remove(path.as_path()) {
                file.flush()?;
            }
            Ok(())
        }
    }
}

/// A path as the registry spells it: `/`-separated.
fn path_key(path: &Path) -> String {
    path.to_string_lossy().replace('\\', "/")
}

/// The file options a run fixes for its whole life.
#[derive(Debug, Clone)]
pub(crate) struct HdlFileOptions {
    /// What a relative path resolves against.
    pub run_directory: PathBuf,
    /// `--hdl-unrestricted-io`: writes may leave the run directory.
    pub unrestricted: bool,
    pub store: Arc<HdlFileStore>,
}

impl Default for HdlFileOptions {
    fn default() -> Self {
        Self::new(
            None,
            false,
            Arc::new(HdlFileStore::new(HdlFileTarget::platform_default())),
        )
    }
}

impl PartialEq for HdlFileOptions {
    fn eq(&self, other: &Self) -> bool {
        self.run_directory == other.run_directory
            && self.unrestricted == other.unrestricted
            && Arc::ptr_eq(&self.store, &other.store)
    }
}

impl Eq for HdlFileOptions {}

impl HdlFileOptions {
    /// Options over `run_directory`, made absolute against the process's
    /// working directory when it is relative; no directory is the working
    /// directory itself (the browser's virtual root).
    pub(crate) fn new(
        run_directory: Option<&Path>,
        unrestricted: bool,
        store: Arc<HdlFileStore>,
    ) -> Self {
        let directory = run_directory.map_or_else(PathBuf::new, Path::to_path_buf);
        let run_directory = if directory.is_absolute() {
            directory
        } else {
            match std::env::current_dir() {
                Ok(current) if !cfg!(target_arch = "wasm32") => current.join(directory),
                _ => directory,
            }
        };
        Self {
            run_directory: normalize(&run_directory),
            unrestricted,
            store,
        }
    }

    /// The path a design's file name names, for reading.
    pub(crate) fn resolve_read(&self, name: &str) -> Result<PathBuf, String> {
        if name.is_empty() {
            return Err("the file name is empty".into());
        }
        Ok(normalize(&self.run_directory.join(name)))
    }

    /// The path a design's file name names, for writing: inside the run
    /// directory unless the run lifted the confinement.
    pub(crate) fn resolve_write(&self, name: &str) -> Result<PathBuf, String> {
        let path = self.resolve_read(name)?;
        if self.unrestricted || self.confines(&path) {
            return Ok(path);
        }
        Err(format!(
            "`{name}` resolves to {}, outside the run directory {}; HDL writes stay \
             inside it unless the run passes --hdl-unrestricted-io",
            path.display(),
            self.run_directory.display()
        ))
    }

    fn confines(&self, path: &Path) -> bool {
        let escapes = path
            .components()
            .any(|component| component == Component::ParentDir);
        if escapes || (path.has_root() && !self.run_directory.has_root()) {
            return false;
        }
        path.starts_with(&self.run_directory) && path != self.run_directory
    }
}

/// Lexical normalization: `.` dropped, `..` folded into its parent. A `..`
/// that climbs above a relative start is kept, so the result still escapes;
/// one that climbs above a root stays at the root.
pub(crate) fn normalize(path: &Path) -> PathBuf {
    let mut out = PathBuf::new();
    let mut depth = 0usize;
    for component in path.components() {
        match component {
            Component::CurDir => {}
            Component::ParentDir => {
                if depth > 0 {
                    out.pop();
                    depth -= 1;
                } else if !out.has_root() {
                    out.push("..");
                }
            }
            Component::Normal(part) => {
                out.push(part);
                depth += 1;
            }
            Component::RootDir | Component::Prefix(_) => out.push(component.as_os_str()),
        }
    }
    out
}

/// How a handle was opened (section 17.2.1, table 17-7).
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub(crate) struct OpenMode {
    pub read: bool,
    pub write: bool,
    /// Writes go to the end of the file.
    pub append: bool,
    /// Opening empties the file.
    pub truncate: bool,
    /// Opening requires the file to exist.
    pub existing: bool,
}

impl OpenMode {
    /// A multichannel `$fopen(name)`: write, emptying the file.
    pub(crate) const CHANNEL: Self = Self {
        read: false,
        write: true,
        append: false,
        truncate: true,
        existing: false,
    };

    /// Table 17-7's spellings; `b` is accepted and means nothing on any
    /// host RSpice runs on.
    pub(crate) fn parse(spelling: &str) -> Option<Self> {
        let base: String = spelling.chars().filter(|c| *c != 'b').collect();
        let binary = spelling.chars().filter(|c| *c == 'b').count();
        if binary > 1 || spelling.starts_with('b') {
            return None;
        }
        let mode = |read, write, append, truncate, existing| Self {
            read,
            write,
            append,
            truncate,
            existing,
        };
        Some(match base.as_str() {
            "r" => mode(true, false, false, false, true),
            "w" => mode(false, true, false, true, false),
            "a" => mode(false, true, true, false, false),
            "r+" => mode(true, true, false, false, true),
            "w+" => mode(true, true, false, true, false),
            "a+" => mode(true, true, true, false, false),
            _ => return None,
        })
    }
}

/// One open file, as the design sees it.
#[derive(Debug, Clone)]
pub(crate) struct OpenFile {
    /// The name the design gave `$fopen`, for records and diagnostics.
    pub name: Arc<str>,
    pub path: PathBuf,
    pub mode: OpenMode,
    /// The file as this handle reads it: the snapshot taken at open, with the
    /// handle's own writes applied. `None` on a write-only handle.
    content: Option<Arc<Vec<u8>>>,
    /// Where the next read or write goes, ignoring pushed-back bytes.
    position: u64,
    /// How long the file is as this handle knows it.
    length: u64,
    /// `$ungetc`'s pushed-back bytes, the next to read last.
    pushback: Vec<u8>,
    /// C's end-of-file indicator: a read ran out.
    eof: bool,
    /// The `errno` of the last failed operation on the handle.
    error: i32,
}

impl OpenFile {
    fn new(name: &str, path: PathBuf, mode: OpenMode, content: Option<Vec<u8>>) -> Self {
        let length = content.as_ref().map_or(0, |content| content.len() as u64);
        Self {
            name: Arc::from(name),
            path,
            mode,
            content: content.map(Arc::new),
            position: 0,
            length,
            pushback: Vec::new(),
            eof: false,
            error: 0,
        }
    }

    /// The next byte, without consuming it.
    pub(crate) fn peek(&self) -> Option<u8> {
        if let Some(byte) = self.pushback.last() {
            return Some(*byte);
        }
        let content = self.content.as_ref()?;
        content.get(usize::try_from(self.position).ok()?).copied()
    }

    /// The next byte; at the end, sets the end-of-file indicator.
    pub(crate) fn next_byte(&mut self) -> Option<u8> {
        if let Some(byte) = self.pushback.pop() {
            return Some(byte);
        }
        match self.peek() {
            Some(byte) => {
                self.position += 1;
                Some(byte)
            }
            None => {
                self.eof = true;
                None
            }
        }
    }

    pub(crate) fn unget(&mut self, byte: u8) {
        self.pushback.push(byte);
        self.eof = false;
    }

    /// `$ftell`: the position a read would continue from.
    pub(crate) fn tell(&self) -> u64 {
        self.position.saturating_sub(self.pushback.len() as u64)
    }

    /// `$fseek`: `whence` 0 from the start, 1 from here, 2 from the end.
    pub(crate) fn seek(&mut self, offset: i64, whence: i64) -> bool {
        let base = match whence {
            0 => 0,
            1 => self.tell() as i64,
            2 => self.length as i64,
            _ => {
                self.error = EINVAL;
                return false;
            }
        };
        match base.checked_add(offset).filter(|target| *target >= 0) {
            Some(target) => {
                self.position = target as u64;
                self.pushback.clear();
                self.eof = false;
                true
            }
            None => {
                self.error = EINVAL;
                false
            }
        }
    }

    pub(crate) fn at_eof(&self) -> bool {
        self.eof
    }

    pub(crate) fn error(&self) -> i32 {
        self.error
    }

    pub(crate) fn readable(&self) -> bool {
        self.mode.read
    }

    /// Write `bytes` where the handle stands, returning the journal entry.
    fn write(&mut self, bytes: &[u8]) -> FileOp {
        let offset = if self.mode.append {
            self.length
        } else {
            self.position
        };
        self.pushback.clear();
        if let Some(content) = self.content.as_mut() {
            let content = Arc::make_mut(content);
            let start = usize::try_from(offset).unwrap_or(usize::MAX);
            let end = start.saturating_add(bytes.len());
            if content.len() < end {
                content.resize(end, 0);
            }
            content[start..end].copy_from_slice(bytes);
        }
        self.position = offset + bytes.len() as u64;
        self.length = self.length.max(self.position);
        if self.mode.append {
            FileOp::Append {
                path: self.path.clone(),
                bytes: bytes.to_vec(),
            }
        } else {
            FileOp::Write {
                path: self.path.clone(),
                offset,
                bytes: bytes.to_vec(),
            }
        }
    }

    fn content_hash(&self) -> Option<String> {
        self.content
            .as_ref()
            .map(|content| blake3::hash(content).to_hex().to_string())
    }
}

/// Where a write goes.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(crate) enum WriteTarget {
    Stdout,
    Stderr,
    /// A file descriptor's slot.
    Descriptor(usize),
    /// A multichannel descriptor's bit.
    Channel(usize),
}

/// Why `$fopen` returned 0.
#[derive(Debug, Clone, PartialEq, Eq)]
pub(crate) enum OpenFailure {
    /// The file does not exist, or its directory does not: section 17.2.1's
    /// ordinary failure, reported only through the value.
    Quiet,
    /// The call itself was wrong, or the sandbox refused the path: a warning.
    Warn(String),
}

/// The open files of a running design.
#[derive(Debug, Clone, Default)]
pub(crate) struct HdlFiles {
    /// Indexed by a file descriptor's low bits; 0 to 2 stay empty.
    descriptors: Vec<Option<OpenFile>>,
    /// Indexed by a multichannel descriptor's bit; 0 stays empty.
    channels: Vec<Option<OpenFile>>,
    /// Writes journaled since the last commit.
    pending: Vec<FileOp>,
}

impl HdlFiles {
    pub(crate) fn has_pending(&self) -> bool {
        !self.pending.is_empty()
    }

    /// Hand the journal to the store; the failures, for warnings.
    pub(crate) fn release(&mut self, store: &HdlFileStore) -> Vec<String> {
        if self.pending.is_empty() {
            return Vec::new();
        }
        store.apply(std::mem::take(&mut self.pending))
    }

    pub(crate) fn journal(&mut self, op: FileOp) {
        // Contiguous writes to one file coalesce, so a design that writes a
        // line per clock edge journals one entry per commit.
        if let Some(last) = self.pending.last_mut() {
            match (last, &op) {
                (
                    FileOp::Write {
                        path,
                        offset,
                        bytes,
                    },
                    FileOp::Write {
                        path: next_path,
                        offset: next_offset,
                        bytes: next,
                    },
                ) if path == next_path && *offset + bytes.len() as u64 == *next_offset => {
                    bytes.extend_from_slice(next);
                    return;
                }
                (
                    FileOp::Append { path, bytes },
                    FileOp::Append {
                        path: next_path,
                        bytes: next,
                    },
                ) if path == next_path => {
                    bytes.extend_from_slice(next);
                    return;
                }
                _ => {}
            }
        }
        self.pending.push(op);
    }

    /// The file at `path` as a read now sees it: the store's committed bytes
    /// with this run's journaled writes applied.
    pub(crate) fn view(
        &self,
        options: &HdlFileOptions,
        path: &Path,
        name: &str,
    ) -> Result<Option<Vec<u8>>, String> {
        let mut content = options.store.read(path, name)?;
        for op in self.pending.iter().filter(|op| op.path() == path) {
            let bytes = content.get_or_insert_with(Vec::new);
            op.apply_to(bytes);
        }
        Ok(content)
    }

    /// `$fopen(name)` or `$fopen(name, type)`: the descriptor, or why it is 0.
    pub(crate) fn open(
        &mut self,
        options: &HdlFileOptions,
        name: &str,
        mode: Option<&str>,
    ) -> Result<u32, OpenFailure> {
        let channel = mode.is_none();
        let mode = match mode {
            None => OpenMode::CHANNEL,
            Some(spelling) => OpenMode::parse(spelling).ok_or_else(|| {
                OpenFailure::Warn(format!("the mode argument ({spelling}) is invalid"))
            })?,
        };
        let path = if mode.write {
            options.resolve_write(name).map_err(OpenFailure::Warn)?
        } else {
            options.resolve_read(name).map_err(OpenFailure::Warn)?
        };
        let existing = if mode.read || mode.existing || mode.append {
            self.view(options, &path, name)
                .map_err(|error| OpenFailure::Warn(format!("cannot read `{name}`: {error}")))?
        } else {
            None
        };
        if mode.existing && existing.is_none() {
            return Err(OpenFailure::Quiet);
        }
        if mode.write && existing.is_none() && !options.store.can_create(&path) {
            return Err(OpenFailure::Quiet);
        }
        let length = existing.as_ref().map_or(0, |bytes| bytes.len() as u64);
        let content = if mode.read {
            Some(if mode.truncate {
                Vec::new()
            } else {
                existing.unwrap_or_default()
            })
        } else {
            None
        };
        let mut file = OpenFile::new(name, path.clone(), mode, content);
        if !mode.read && mode.append {
            file.length = length;
        }
        if mode.write {
            self.journal(FileOp::Create {
                path,
                truncate: mode.truncate,
            });
        }
        if channel {
            if self.channels.len() < CHANNEL_BITS {
                self.channels.resize(CHANNEL_BITS, None);
            }
            let Some(bit) = (1..CHANNEL_BITS).find(|bit| self.channels[*bit].is_none()) else {
                self.pending.pop();
                return Err(OpenFailure::Warn(
                    "every multichannel descriptor (30) is open".into(),
                ));
            };
            self.channels[bit] = Some(file);
            Ok(1 << bit)
        } else {
            if self.descriptors.len() < FIRST_SLOT {
                self.descriptors.resize(FIRST_SLOT, None);
            }
            let slot = match (FIRST_SLOT..self.descriptors.len())
                .find(|slot| self.descriptors[*slot].is_none())
            {
                Some(slot) => slot,
                None => {
                    self.descriptors.push(None);
                    self.descriptors.len() - 1
                }
            };
            if slot as u32 >= FD_FLAG {
                return Err(OpenFailure::Warn("no file descriptor is free".into()));
            }
            self.descriptors[slot] = Some(file);
            Ok(FD_FLAG | slot as u32)
        }
    }

    /// `$fclose`: close a file descriptor, or every file of a multichannel
    /// descriptor. The standard streams cannot be closed.
    pub(crate) fn close(&mut self, descriptor: u32) -> Result<(), String> {
        if descriptor & FD_FLAG != 0 {
            let slot = (descriptor & !FD_FLAG) as usize;
            let file = self
                .descriptors
                .get_mut(slot)
                .and_then(Option::take)
                .ok_or_else(|| invalid_descriptor(descriptor))?;
            self.journal(FileOp::Close { path: file.path });
            return Ok(());
        }
        if descriptor == 0 || !self.channel_bits_open(descriptor, false) {
            return Err(invalid_channel(descriptor));
        }
        for bit in 1..CHANNEL_BITS {
            if descriptor >> bit & 1 == 1
                && let Some(file) = self.channels.get_mut(bit).and_then(Option::take)
            {
                self.journal(FileOp::Close { path: file.path });
            }
        }
        Ok(())
    }

    /// Whether every file bit of a multichannel descriptor is open; bit 0,
    /// the standard output, counts as open only under `stdout`.
    fn channel_bits_open(&self, descriptor: u32, stdout: bool) -> bool {
        if descriptor & FD_FLAG != 0 {
            return false;
        }
        (0..CHANNEL_BITS).all(|bit| {
            descriptor >> bit & 1 == 0
                || if bit == 0 {
                    stdout
                } else {
                    self.channels.get(bit).is_some_and(Option::is_some)
                }
        })
    }

    /// Where a write to `descriptor` goes (`$fdisplay` and its kin).
    pub(crate) fn write_targets(&self, descriptor: u32) -> Result<Vec<WriteTarget>, String> {
        if descriptor & FD_FLAG != 0 {
            return match descriptor {
                STDOUT => Ok(vec![WriteTarget::Stdout]),
                STDERR => Ok(vec![WriteTarget::Stderr]),
                STDIN => Err(invalid_descriptor(descriptor)),
                _ => {
                    let slot = (descriptor & !FD_FLAG) as usize;
                    match self.descriptors.get(slot).and_then(Option::as_ref) {
                        Some(file) if file.mode.write => Ok(vec![WriteTarget::Descriptor(slot)]),
                        _ => Err(invalid_descriptor(descriptor)),
                    }
                }
            };
        }
        if descriptor == 0 || !self.channel_bits_open(descriptor, true) {
            return Err(invalid_channel(descriptor));
        }
        Ok((0..CHANNEL_BITS)
            .filter(|bit| descriptor >> bit & 1 == 1)
            .map(|bit| {
                if bit == 0 {
                    WriteTarget::Stdout
                } else {
                    WriteTarget::Channel(bit)
                }
            })
            .collect())
    }

    /// The handle a write target names, for its record.
    pub(crate) fn target_file(&self, target: WriteTarget) -> Option<&OpenFile> {
        match target {
            WriteTarget::Descriptor(slot) => self.descriptors.get(slot)?.as_ref(),
            WriteTarget::Channel(bit) => self.channels.get(bit)?.as_ref(),
            WriteTarget::Stdout | WriteTarget::Stderr => None,
        }
    }

    /// Write to a file target; the standard streams are the caller's.
    pub(crate) fn write(&mut self, target: WriteTarget, bytes: &[u8]) {
        let file = match target {
            WriteTarget::Descriptor(slot) => self.descriptors.get_mut(slot),
            WriteTarget::Channel(bit) => self.channels.get_mut(bit),
            WriteTarget::Stdout | WriteTarget::Stderr => None,
        };
        if let Some(Some(file)) = file {
            let op = file.write(bytes);
            self.journal(op);
        }
    }

    /// The open file a read-side call names.
    pub(crate) fn file_mut(&mut self, descriptor: u32) -> Result<&mut OpenFile, String> {
        if descriptor & FD_FLAG == 0 || descriptor == STDOUT || descriptor == STDERR {
            return Err(invalid_descriptor(descriptor));
        }
        self.descriptors
            .get_mut((descriptor & !FD_FLAG) as usize)
            .and_then(Option::as_mut)
            .ok_or_else(|| invalid_descriptor(descriptor))
    }

    /// A readable open file.
    pub(crate) fn reader(&mut self, descriptor: u32) -> Result<&mut OpenFile, String> {
        let file = self.file_mut(descriptor)?;
        if !file.readable() {
            file.error = EBADF;
            return Err(format!(
                "file descriptor 0x{descriptor:08x} was opened for writing only"
            ));
        }
        Ok(file)
    }
}

pub(crate) fn invalid_descriptor(descriptor: u32) -> String {
    format!("invalid file descriptor (0x{descriptor:08x})")
}

pub(crate) fn invalid_channel(descriptor: u32) -> String {
    format!("invalid MCD (0x{descriptor:08x})")
}

/// Text as the bytes a file receives: a character below 256 is the byte the
/// formatter or a string literal meant (`%u`, `%c`, `\ooo`); anything wider
/// keeps its UTF-8 spelling.
pub(crate) fn text_bytes(text: &str) -> Vec<u8> {
    let mut bytes = Vec::with_capacity(text.len());
    for character in text.chars() {
        let code = character as u32;
        if code < 256 {
            bytes.push(code as u8);
        } else {
            let mut buffer = [0; 4];
            bytes.extend_from_slice(character.encode_utf8(&mut buffer).as_bytes());
        }
    }
    bytes
}

/// A saved open handle (design P2 section 5.4, the effects section's file
/// table). A write is restored by truncating the file to what the handle
/// had written, so output after the checkpoint is discarded; a read is
/// restored by re-reading the file, which must be what the handle read.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub(crate) struct HandleImage {
    /// The descriptor `$fopen` returned.
    pub descriptor: u32,
    pub name: String,
    pub path: String,
    pub mode: OpenMode,
    pub position: u64,
    pub length: u64,
    /// BLAKE3 of what a readable handle reads.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub content: Option<String>,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub pushback: Vec<u8>,
    #[serde(default)]
    pub eof: bool,
    #[serde(default)]
    pub error: i32,
}

impl HdlFiles {
    pub(crate) fn capture(&self) -> Result<Vec<HandleImage>, String> {
        if !self.pending.is_empty() {
            return Err("HDL file writes are not released".into());
        }
        let descriptors = self
            .descriptors
            .iter()
            .enumerate()
            .filter_map(|(slot, file)| Some((FD_FLAG | slot as u32, file.as_ref()?)));
        let channels = self
            .channels
            .iter()
            .enumerate()
            .filter_map(|(bit, file)| Some((1u32 << bit, file.as_ref()?)));
        Ok(descriptors
            .chain(channels)
            .map(|(descriptor, file)| HandleImage {
                descriptor,
                name: file.name.to_string(),
                path: file.path.to_string_lossy().into_owned(),
                mode: file.mode,
                position: file.position,
                length: file.length,
                content: file.content_hash(),
                pushback: file.pushback.clone(),
                eof: file.eof,
                error: file.error,
            })
            .collect())
    }

    /// Reopen every saved handle. A written file is cut back to what it held
    /// at the checkpoint; a read file must hold what the handle read.
    pub(crate) fn restore(
        &mut self,
        options: &HdlFileOptions,
        images: &[HandleImage],
    ) -> Result<(), String> {
        *self = Self::default();
        for image in images {
            let path = PathBuf::from(&image.path);
            if image.mode.write && !options.unrestricted && !options.confines(&normalize(&path)) {
                return Err(format!(
                    "checkpointed HDL file {} is outside this run's directory {}",
                    image.path,
                    options.run_directory.display()
                ));
            }
            let content = if image.mode.read {
                let bytes = options
                    .store
                    .read(&path, &image.name)?
                    .ok_or_else(|| format!("checkpointed HDL file {} is gone", image.path))?;
                let length = usize::try_from(image.length).unwrap_or(usize::MAX);
                let bytes = bytes.get(..length).map(<[u8]>::to_vec).ok_or_else(|| {
                    format!(
                        "checkpointed HDL file {} is shorter than when it was saved",
                        image.path
                    )
                })?;
                let hash = blake3::hash(&bytes).to_hex().to_string();
                if image.content.as_deref() != Some(hash.as_str()) {
                    return Err(format!(
                        "checkpointed HDL file {} changed since it was saved",
                        image.path
                    ));
                }
                Some(bytes)
            } else {
                None
            };
            if image.mode.write {
                self.journal(FileOp::Truncate {
                    path: path.clone(),
                    length: image.length,
                });
            }
            let mut file = OpenFile::new(&image.name, path, image.mode, content);
            file.position = image.position;
            file.length = image.length;
            file.pushback.clone_from(&image.pushback);
            file.eof = image.eof;
            file.error = image.error;
            if image.descriptor & FD_FLAG != 0 {
                let slot = (image.descriptor & !FD_FLAG) as usize;
                if !(FIRST_SLOT..=1 << 20).contains(&slot) {
                    return Err(format!(
                        "checkpointed file descriptor 0x{:08x} is not one $fopen hands out",
                        image.descriptor
                    ));
                }
                if self.descriptors.len() <= slot {
                    self.descriptors.resize(slot + 1, None);
                }
                self.descriptors[slot] = Some(file);
            } else {
                if !image.descriptor.is_power_of_two() || image.descriptor < 2 {
                    return Err(format!(
                        "checkpointed multichannel descriptor 0x{:08x} is not one bit of 1 to 30",
                        image.descriptor
                    ));
                }
                let bit = image.descriptor.trailing_zeros() as usize;
                if self.channels.len() < CHANNEL_BITS {
                    self.channels.resize(CHANNEL_BITS, None);
                }
                self.channels[bit] = Some(file);
            }
        }
        Ok(())
    }
}

#[cfg(test)]
mod tests;
