use std::path::{Path, PathBuf};

use super::RustBackendError;
use crate::{Lexer, Parser, SourceMap, ast};

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct VerilogASourceCandidate {
    pub path: PathBuf,
    pub modules: Vec<String>,
}

pub fn discover_veriloga_sources(
    root: impl AsRef<Path>,
) -> Result<Vec<VerilogASourceCandidate>, RustBackendError> {
    let root = root.as_ref();
    let mut files = Vec::new();
    collect_va_files(root, &mut files)?;
    files.sort();

    let mut candidates = Vec::new();
    for path in files {
        let source = std::fs::read_to_string(&path).map_err(|error| {
            RustBackendError::internal(
                path.display().to_string(),
                "<scan>",
                format!("failed to read candidate: {error}"),
            )
        })?;
        let modules = module_names_in_source(&path, &source)?;
        if !modules.is_empty() {
            candidates.push(VerilogASourceCandidate { path, modules });
        }
    }

    Ok(candidates)
}

fn collect_va_files(root: &Path, files: &mut Vec<PathBuf>) -> Result<(), RustBackendError> {
    for entry in std::fs::read_dir(root).map_err(|error| {
        RustBackendError::internal(
            root.display().to_string(),
            "<scan>",
            format!("failed to read directory: {error}"),
        )
    })? {
        let entry = entry.map_err(|error| {
            RustBackendError::internal(
                root.display().to_string(),
                "<scan>",
                format!("failed to read directory entry: {error}"),
            )
        })?;
        let path = entry.path();
        let file_type = entry.file_type().map_err(|error| {
            RustBackendError::internal(
                path.display().to_string(),
                "<scan>",
                format!("failed to read file type: {error}"),
            )
        })?;

        if file_type.is_dir() {
            collect_va_files(&path, files)?;
        } else if path.extension().and_then(|ext| ext.to_str()) == Some("va") {
            files.push(path.canonicalize().unwrap_or(path));
        }
    }
    Ok(())
}

fn module_names_in_source(path: &Path, source: &str) -> Result<Vec<String>, RustBackendError> {
    let mut source_map = SourceMap::new();
    let source_id = source_map.add_source_mut(path, source);
    let tokens = Lexer::new(source, source_id)
        .collect_tokens()
        .map_err(|error| {
            RustBackendError::internal(path.display().to_string(), "<scan>", error.to_string())
        })?;
    let parsed = Parser::new(&tokens).parse().map_err(|error| {
        RustBackendError::internal(path.display().to_string(), "<scan>", error.to_string())
    })?;

    let mut modules = parsed
        .items
        .iter()
        .filter_map(|item| match item {
            ast::Item::Module(module) => Some(module.name.to_string()),
            _ => None,
        })
        .collect::<Vec<_>>();
    modules.sort();
    Ok(modules)
}
