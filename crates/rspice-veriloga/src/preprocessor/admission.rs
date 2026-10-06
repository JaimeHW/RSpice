//! Bounded filesystem reads for interactive compiler hosts.
use super::*;
use crate::PipelineControl;
use std::cell::RefCell;
use std::io::Read;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum SourceResource {
    RootBytes,
    RootLines,
    Dependencies,
    TotalSourceBytes,
    IncludeDepth,
    ExpandedBytes,
}

impl std::fmt::Display for SourceResource {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(match self {
            Self::RootBytes => "Root source bytes",
            Self::RootLines => "Root source lines",
            Self::Dependencies => "Dependency count",
            Self::TotalSourceBytes => "Dependency source",
            Self::IncludeDepth => "Include depth",
            Self::ExpandedBytes => "Expanded source",
        })
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, thiserror::Error)]
#[error("{resource} exceeds the provider limit of {limit} (requested {requested})")]
pub struct SourceResourceLimit {
    pub resource: SourceResource,
    pub requested: usize,
    pub limit: usize,
}

/// One compilation's source admission, with streaming checks in addition to
/// metadata checks so growing files cannot bypass the byte budget.
pub struct BoundedFileSystemSourceProvider<'a> {
    limits: SourceProviderLimits,
    root_bytes: usize,
    root_lines: usize,
    control: &'a dyn PipelineControl,
    admitted: RefCell<BTreeMap<PathBuf, usize>>,
}

impl<'a> BoundedFileSystemSourceProvider<'a> {
    pub fn new(
        limits: SourceProviderLimits,
        root_bytes: usize,
        root_lines: usize,
        control: &'a dyn PipelineControl,
    ) -> Self {
        Self {
            limits,
            root_bytes,
            root_lines,
            control,
            admitted: RefCell::new(BTreeMap::new()),
        }
    }

    fn load(&self, path: &Path, root: bool) -> Result<SourceDocument, PreprocessorError> {
        self.checkpoint()?;
        let io_error = |error| PreprocessorError::from_io(error, path);
        let path = path.canonicalize().map_err(io_error)?;
        let (mut file, metadata) = open_regular_source(&path).map_err(io_error)?;
        let admitted = self.admitted.borrow();
        let previous = admitted.get(&path).copied().unwrap_or(0);
        let used = admitted
            .values()
            .fold(0usize, |sum, bytes| sum.saturating_add(*bytes))
            .saturating_sub(previous);
        let dependencies = admitted
            .len()
            .saturating_add(usize::from(!admitted.contains_key(&path)));
        drop(admitted);
        let ensure = |resource, requested, limit| {
            if requested > limit {
                Err(PreprocessorError::resource_limit(
                    resource,
                    requested,
                    limit,
                    Some(path.clone()),
                    0,
                ))
            } else {
                Ok(())
            }
        };
        ensure(
            SourceResource::Dependencies,
            dependencies,
            self.limits.max_dependencies,
        )?;
        let ensure_bytes = |bytes| {
            if root {
                ensure(SourceResource::RootBytes, bytes, self.root_bytes)?;
            }
            ensure(
                SourceResource::TotalSourceBytes,
                used.saturating_add(bytes),
                self.limits.max_total_source_bytes,
            )
        };
        ensure_bytes(usize::try_from(metadata.len()).unwrap_or(usize::MAX))?;
        let mut bytes = Vec::new();
        let mut chunk = [0u8; 16 * 1024];
        let mut newlines = 0usize;
        loop {
            self.checkpoint()?;
            let count = match file.read(&mut chunk) {
                Ok(count) => count,
                // A signal may interrupt a read without making the source
                // invalid. Retry through the cancellation checkpoint.
                Err(error) if error.kind() == std::io::ErrorKind::Interrupted => continue,
                Err(error) => return Err(io_error(error)),
            };
            if count == 0 {
                break;
            }
            ensure_bytes(bytes.len().saturating_add(count))?;
            if root && self.root_lines != usize::MAX {
                newlines = newlines
                    .saturating_add(chunk[..count].iter().filter(|&&byte| byte == b'\n').count());
                // Match str::lines: LF terminates a line (including CRLF),
                // and a nonempty unterminated tail is one more line. Count
                // bytes before decoding so an over-budget source stops here.
                let lines = newlines.saturating_add(usize::from(chunk[count - 1] != b'\n'));
                ensure(SourceResource::RootLines, lines, self.root_lines)?;
            }
            bytes
                .try_reserve(count)
                .map_err(|error| io_error(std::io::Error::other(error)))?;
            bytes.extend_from_slice(&chunk[..count]);
        }
        let source = String::from_utf8(bytes).map_err(|error| {
            io_error(std::io::Error::new(std::io::ErrorKind::InvalidData, error))
        })?;
        self.admitted
            .borrow_mut()
            .insert(path.clone(), source.len());
        Ok(SourceDocument::provided(path, source))
    }
}

impl SourceProvider for BoundedFileSystemSourceProvider<'_> {
    fn load_root(&self, requested: &Path) -> Result<SourceDocument, PreprocessorError> {
        self.load(requested, true)
    }

    fn resolve_include(
        &self,
        including_file: Option<&Path>,
        include_paths: &[PathBuf],
        requested: &str,
    ) -> Result<Option<SourceDocument>, PreprocessorError> {
        for directory in including_file
            .and_then(Path::parent)
            .into_iter()
            .chain(include_paths.iter().map(PathBuf::as_path))
        {
            self.checkpoint()?;
            let path = directory.join(requested);
            if include_candidate_exists(&path)? {
                return self.load(&path, false).map(Some);
            }
        }
        Ok(None)
    }

    fn checkpoint(&self) -> Result<(), PreprocessorError> {
        if self.control.is_cancelled() {
            Err(PreprocessorError::cancelled())
        } else {
            Ok(())
        }
    }

    fn limits(&self) -> SourceProviderLimits {
        self.limits
    }
}
