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
        let io_error = |error: std::io::Error| {
            PreprocessorError::new(error.to_string(), Some(path.to_path_buf()), 0)
        };
        let path = path.canonicalize().map_err(io_error)?;
        let mut file = std::fs::File::open(&path).map_err(io_error)?;
        let metadata = file.metadata().map_err(io_error)?;
        if !metadata.is_file() {
            return Err(io_error(std::io::Error::other(
                "source must be a regular file",
            )));
        }
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
        loop {
            self.checkpoint()?;
            let count = file.read(&mut chunk).map_err(io_error)?;
            if count == 0 {
                break;
            }
            ensure_bytes(bytes.len().saturating_add(count))?;
            bytes
                .try_reserve(count)
                .map_err(|error| io_error(std::io::Error::other(error)))?;
            bytes.extend_from_slice(&chunk[..count]);
        }
        let source = String::from_utf8(bytes).map_err(|error| {
            io_error(std::io::Error::new(std::io::ErrorKind::InvalidData, error))
        })?;
        if root {
            ensure(
                SourceResource::RootLines,
                source.lines().count(),
                self.root_lines,
            )?;
        }
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
            if path.exists() {
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
