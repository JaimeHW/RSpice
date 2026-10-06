use rspice_veriloga::preprocessor::SourceResource;
use rspice_veriloga::{
    Preprocessor, PreprocessorError, SourceDocument, SourceProvider, SourceProviderLimits,
};
use std::path::{Path, PathBuf};
use std::sync::atomic::{AtomicUsize, Ordering};

struct Source {
    source: String,
    remaining: AtomicUsize,
    limit: usize,
}
impl SourceProvider for Source {
    fn load_root(&self, path: &Path) -> Result<SourceDocument, PreprocessorError> {
        Ok(SourceDocument::provided(path, &self.source))
    }
    fn resolve_include(
        &self,
        _: Option<&Path>,
        _: &[PathBuf],
        _: &str,
    ) -> Result<Option<SourceDocument>, PreprocessorError> {
        Ok(None)
    }
    fn checkpoint(&self) -> Result<(), PreprocessorError> {
        self.remaining
            .fetch_update(Ordering::Relaxed, Ordering::Relaxed, |remaining| {
                remaining.checked_sub(1)
            })
            .map(|_| ())
            .map_err(|_| PreprocessorError::cancelled())
    }
    fn limits(&self) -> SourceProviderLimits {
        SourceProviderLimits {
            max_expanded_bytes: self.limit,
            ..SourceProviderLimits::UNBOUNDED
        }
    }
}

#[test]
fn cancellation_is_polled_during_preprocessing_not_only_at_phase_boundaries() {
    let source = Source {
        source: "parameter real R=1;\n".repeat(2000),
        remaining: AtomicUsize::new(10),
        limit: usize::MAX,
    };
    let error = Preprocessor::new()
        .preprocess_provider_root(&source, Path::new("root.va"))
        .unwrap_err();
    assert!(error.cancelled);
    assert!(error.resource_limit.is_none());
}

#[test]
fn macro_expansion_retains_structured_admission_details() {
    let source = Source {
        source: "`define X abcdefghijklmnopqrstuvwxyz\n`X `X `X\n".into(),
        remaining: AtomicUsize::new(100),
        limit: 32,
    };
    let error = Preprocessor::new()
        .preprocess_provider_root(&source, Path::new("root.va"))
        .unwrap_err();
    let limit = error.resource_limit.unwrap();
    assert_eq!(limit.resource, SourceResource::ExpandedBytes);
    assert_eq!(limit.limit, 32);
    assert!(limit.requested > limit.limit);
    assert!(!error.cancelled);
}
