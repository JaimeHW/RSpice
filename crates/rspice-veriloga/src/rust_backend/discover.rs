use std::path::{Path, PathBuf};

use super::RustBackendError;

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

fn module_names_in_source(_path: &Path, source: &str) -> Result<Vec<String>, RustBackendError> {
    let mut scanner = ModuleNameScanner::new(source);
    let mut modules = scanner.collect_modules();
    modules.sort();
    Ok(modules)
}

struct ModuleNameScanner<'a> {
    source: &'a str,
    cursor: usize,
    at_line_start: bool,
}

impl<'a> ModuleNameScanner<'a> {
    fn new(source: &'a str) -> Self {
        Self {
            source,
            cursor: 0,
            at_line_start: true,
        }
    }

    fn collect_modules(&mut self) -> Vec<String> {
        let mut modules = Vec::new();
        while let Some(token) = self.next_identifier() {
            if token == "module" {
                if let Some(name) = self.next_identifier() {
                    modules.push(name);
                }
            }
        }
        modules
    }

    fn next_identifier(&mut self) -> Option<String> {
        while self.cursor < self.source.len() {
            self.skip_insignificant();
            let rest = &self.source[self.cursor..];
            let Some(first) = rest.chars().next() else {
                return None;
            };

            if first == '\\' {
                return Some(self.take_escaped_identifier());
            }

            if is_identifier_start(first) {
                return Some(self.take_identifier());
            }

            self.advance_char(first);
        }
        None
    }

    fn skip_insignificant(&mut self) {
        loop {
            if self.cursor >= self.source.len() {
                return;
            }

            let rest = &self.source[self.cursor..];
            if rest.starts_with("//") {
                self.skip_line_comment();
                continue;
            }
            if rest.starts_with("/*") {
                self.skip_block_comment();
                continue;
            }

            let Some(ch) = rest.chars().next() else {
                return;
            };

            if ch == '"' {
                self.skip_string();
                continue;
            }

            if self.at_line_start && ch == '`' {
                self.skip_preprocessor_directive();
                continue;
            }

            if ch.is_whitespace() {
                self.advance_whitespace(ch);
                continue;
            }

            return;
        }
    }

    fn take_identifier(&mut self) -> String {
        let start = self.cursor;
        while self.cursor < self.source.len() {
            let ch = self.source[self.cursor..].chars().next().unwrap();
            if !is_identifier_continue(ch) {
                break;
            }
            self.advance_char(ch);
        }
        self.at_line_start = false;
        self.source[start..self.cursor].to_string()
    }

    fn take_escaped_identifier(&mut self) -> String {
        self.advance_char('\\');
        let start = self.cursor;
        while self.cursor < self.source.len() {
            let ch = self.source[self.cursor..].chars().next().unwrap();
            if ch.is_whitespace() {
                break;
            }
            self.advance_char(ch);
        }
        self.at_line_start = false;
        self.source[start..self.cursor].to_string()
    }

    fn skip_line_comment(&mut self) {
        while self.cursor < self.source.len() {
            let ch = self.source[self.cursor..].chars().next().unwrap();
            self.advance_char(ch);
            if ch == '\n' {
                self.at_line_start = true;
                break;
            }
        }
    }

    fn skip_block_comment(&mut self) {
        self.cursor += 2;
        while self.cursor < self.source.len() {
            if self.source[self.cursor..].starts_with("*/") {
                self.cursor += 2;
                self.at_line_start = false;
                return;
            }
            let ch = self.source[self.cursor..].chars().next().unwrap();
            self.advance_char(ch);
        }
    }

    fn skip_string(&mut self) {
        self.advance_char('"');
        let mut escaped = false;
        while self.cursor < self.source.len() {
            let ch = self.source[self.cursor..].chars().next().unwrap();
            self.advance_char(ch);
            if escaped {
                escaped = false;
            } else if ch == '\\' {
                escaped = true;
            } else if ch == '"' {
                break;
            }
        }
        self.at_line_start = false;
    }

    fn skip_preprocessor_directive(&mut self) {
        let mut continued = false;
        loop {
            let mut previous_significant = '\0';
            while self.cursor < self.source.len() {
                let ch = self.source[self.cursor..].chars().next().unwrap();
                self.advance_char(ch);
                if ch == '\n' {
                    continued = previous_significant == '\\';
                    self.at_line_start = true;
                    break;
                }
                if ch != '\r' {
                    previous_significant = ch;
                }
            }

            if !continued || self.cursor >= self.source.len() {
                return;
            }
        }
    }

    fn advance_whitespace(&mut self, ch: char) {
        self.advance_char(ch);
        if ch == '\n' {
            self.at_line_start = true;
        }
    }

    fn advance_char(&mut self, ch: char) {
        self.cursor += ch.len_utf8();
        if ch == '\n' {
            self.at_line_start = true;
        } else if !ch.is_whitespace() {
            self.at_line_start = false;
        }
    }
}

fn is_identifier_start(ch: char) -> bool {
    ch == '_' || ch == '$' || ch.is_ascii_alphabetic()
}

fn is_identifier_continue(ch: char) -> bool {
    is_identifier_start(ch) || ch.is_ascii_digit()
}
