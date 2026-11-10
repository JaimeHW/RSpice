//! Finding the Verilog-A sources in a model tree and how to preprocess them.
//!
//! [`discover_veriloga_sources`] scans for `.va` sources and reports the
//! modules each declares. Two on-disk markers steer it: a directory holding
//! `.rspice-veriloga-skip` is excluded entirely, and a
//! `.rspice-veriloga-profile` file supplies the [`VerilogACompileProfile`] —
//! the `defines`/`undefines` a source needs to preprocess, and optional
//! `namespace SOURCE::MODULE=HEX_ID` declarations that preserve published names.
//! A profile's `name SOURCE::MODULE=NAME` line gives a module the built-in name
//! it publishes, where its own module name is one another release publishes.
//! A profile's `skip SOURCE` line names a file that declares a module but is
//! not a root of its release: another root includes it, so building it alone
//! would publish a second, partial copy of that root's model.
//!
//! Results are sorted, since discovery order determines generation order and
//! generation has to be reproducible.

use std::collections::{BTreeMap, BTreeSet, HashSet};
use std::path::{Path, PathBuf};

use super::RustBackendError;
use crate::{Preprocessor, preprocessor::MacroDef};

pub const VERILOGA_DISCOVERY_SKIP_MARKER: &str = ".rspice-veriloga-skip";
pub const VERILOGA_COMPILE_PROFILE_FILE_NAME: &str = ".rspice-veriloga-profile";

#[derive(Debug, Clone, Default, PartialEq, Eq)]
pub struct VerilogACompileProfile {
    pub defines: Vec<(String, Option<String>)>,
    pub undefines: Vec<String>,
    /// Authored namespaces keep published model paths stable as sources change.
    pub namespaces: BTreeMap<String, String>,
    /// Authored built-in names, by module. Two releases of one model often
    /// share a module name; naming the later one keeps the published name of
    /// the first, where the registry would otherwise suffix both.
    pub public_names: BTreeMap<String, String>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct VerilogASourceCandidate {
    pub path: PathBuf,
    pub modules: Vec<String>,
    pub compile_profile: VerilogACompileProfile,
}

pub fn discover_veriloga_sources(
    root: impl AsRef<Path>,
) -> Result<Vec<VerilogASourceCandidate>, RustBackendError> {
    let root = root
        .as_ref()
        .canonicalize()
        .unwrap_or_else(|_| root.as_ref().to_path_buf());
    let root = root.as_path();
    let mut files = Vec::new();
    collect_va_files(root, &mut files)?;
    files.sort();

    let mut candidates = Vec::new();
    for path in files {
        let (compile_profile, skipped) = compile_profile_for_source(root, &path)?;
        if skipped {
            continue;
        }
        let modules = module_names_in_file(root, &path, &compile_profile)?;
        for module in compile_profile.namespaces.keys() {
            if !modules.contains(module) {
                return Err(RustBackendError::internal(
                    path.display().to_string(),
                    module,
                    "namespace selects a module absent from the preprocessed source",
                ));
            }
        }
        for module in compile_profile.public_names.keys() {
            if !modules.contains(module) {
                return Err(RustBackendError::internal(
                    path.display().to_string(),
                    module,
                    "name selects a module absent from the preprocessed source",
                ));
            }
        }
        if !modules.is_empty() {
            candidates.push(VerilogASourceCandidate {
                path,
                modules,
                compile_profile,
            });
        }
    }

    Ok(candidates)
}

fn collect_va_files(root: &Path, files: &mut Vec<PathBuf>) -> Result<(), RustBackendError> {
    if root.join(VERILOGA_DISCOVERY_SKIP_MARKER).is_file() {
        return Ok(());
    }

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
        } else if has_veriloga_extension(&path) {
            files.push(path.canonicalize().unwrap_or(path));
        }
    }
    Ok(())
}

fn has_veriloga_extension(path: &Path) -> bool {
    path.extension()
        .and_then(|ext| ext.to_str())
        .is_some_and(|ext| ext.eq_ignore_ascii_case("va"))
}

fn module_names_in_file(
    root: &Path,
    path: &Path,
    profile: &VerilogACompileProfile,
) -> Result<Vec<String>, RustBackendError> {
    let mut preprocessor = Preprocessor::new();
    preprocessor.add_include_path(root);
    for name in &profile.undefines {
        preprocessor.undefine(name);
    }
    for (name, value) in &profile.defines {
        preprocessor.define(name, MacroDef::simple(value.as_deref().unwrap_or_default()));
    }
    let preprocessed = match preprocessor.preprocess_file(path) {
        Ok(preprocessed) => preprocessed,
        // A fragment that another file includes after defining its macros
        // (EKV3's `ekv3_include/*.va`) cannot be read on its own, so it is
        // no compilation root. A root that stopped reading this way would
        // drop out of the generated catalog, which the manifest shows.
        Err(error) if error.names_an_undefined_macro() => return Ok(Vec::new()),
        Err(error) => {
            return Err(RustBackendError::internal(
                path.display().to_string(),
                "<scan>",
                error.to_string(),
            ));
        }
    };
    Ok(module_names_in_source(&preprocessed))
}

/// The profile that applies to `path`, and whether a profile skips it.
fn compile_profile_for_source(
    root: &Path,
    path: &Path,
) -> Result<(VerilogACompileProfile, bool), RustBackendError> {
    let mut directories = Vec::new();
    let mut current = path.parent();
    while let Some(directory) = current {
        if !directory.starts_with(root) {
            break;
        }
        directories.push(directory.to_path_buf());
        if directory == root {
            break;
        }
        current = directory.parent();
    }
    directories.reverse();

    let mut defines = BTreeMap::<String, Option<String>>::new();
    let mut undefines = BTreeSet::<String>::new();
    let mut namespaces = BTreeMap::<String, String>::new();
    let mut public_names = BTreeMap::<String, String>::new();
    let mut skipped = false;
    for directory in directories {
        let profile_path = directory.join(VERILOGA_COMPILE_PROFILE_FILE_NAME);
        if !profile_path.is_file() {
            continue;
        }
        let contents = std::fs::read_to_string(&profile_path).map_err(|error| {
            RustBackendError::internal(
                profile_path.display().to_string(),
                "<profile>",
                format!("failed to read compile profile: {error}"),
            )
        })?;
        apply_compile_profile(
            &profile_path,
            &contents,
            path,
            &mut defines,
            &mut undefines,
            &mut namespaces,
            &mut public_names,
            &mut skipped,
        )?;
    }

    Ok((
        VerilogACompileProfile {
            defines: defines.into_iter().collect(),
            undefines: undefines.into_iter().collect(),
            namespaces,
            public_names,
        },
        skipped,
    ))
}

fn apply_compile_profile(
    path: &Path,
    contents: &str,
    source: &Path,
    defines: &mut BTreeMap<String, Option<String>>,
    undefines: &mut BTreeSet<String>,
    namespaces: &mut BTreeMap<String, String>,
    public_names: &mut BTreeMap<String, String>,
    skipped: &mut bool,
) -> Result<(), RustBackendError> {
    for (index, raw_line) in contents.lines().enumerate() {
        let line_number = index + 1;
        let line = raw_line.trim();
        if line.is_empty() || line.starts_with('#') {
            continue;
        }
        let (directive, arguments) = line
            .split_once(char::is_whitespace)
            .map(|(directive, arguments)| (directive, arguments.trim()))
            .unwrap_or((line, ""));
        match directive {
            "namespace" => {
                let (selector, namespace) = arguments.split_once('=').ok_or_else(|| {
                    profile_error(
                        path,
                        line_number,
                        "expected namespace SOURCE::MODULE=HEX_ID",
                    )
                })?;
                let (relative_source, module) =
                    selector.trim().rsplit_once("::").ok_or_else(|| {
                        profile_error(
                            path,
                            line_number,
                            "namespace selector must name SOURCE::MODULE",
                        )
                    })?;
                let namespace = namespace.trim();
                if namespace.len() != 8 || !namespace.bytes().all(|byte| byte.is_ascii_hexdigit()) {
                    return Err(profile_error(
                        path,
                        line_number,
                        "namespace must contain exactly eight hexadecimal digits",
                    ));
                }
                let module = module.trim();
                if module.is_empty() {
                    return Err(profile_error(
                        path,
                        line_number,
                        "namespace module cannot be empty",
                    ));
                }
                let selected_source =
                    profile_source(path, line_number, "namespace", relative_source)?;
                if selected_source == source
                    && namespaces
                        .insert(module.to_string(), namespace.to_ascii_lowercase())
                        .is_some()
                {
                    return Err(profile_error(
                        path,
                        line_number,
                        "duplicate namespace for source module",
                    ));
                }
            }
            "name" => {
                let (selector, public_name) = arguments.split_once('=').ok_or_else(|| {
                    profile_error(path, line_number, "expected name SOURCE::MODULE=NAME")
                })?;
                let (relative_source, module) =
                    selector.trim().rsplit_once("::").ok_or_else(|| {
                        profile_error(path, line_number, "name selector must name SOURCE::MODULE")
                    })?;
                let (module, public_name) = (module.trim(), public_name.trim());
                if module.is_empty() {
                    return Err(profile_error(
                        path,
                        line_number,
                        "name module cannot be empty",
                    ));
                }
                if !public_name
                    .chars()
                    .next()
                    .is_some_and(|first| first.is_ascii_alphabetic() || first == '_')
                    || !public_name
                        .chars()
                        .all(|character| character.is_ascii_alphanumeric() || character == '_')
                {
                    return Err(profile_error(
                        path,
                        line_number,
                        "a built-in name is a letter or underscore followed by letters, digits and underscores",
                    ));
                }
                if profile_source(path, line_number, "name", relative_source)? == source
                    && public_names
                        .insert(module.to_string(), public_name.to_string())
                        .is_some()
                {
                    return Err(profile_error(
                        path,
                        line_number,
                        "duplicate name for source module",
                    ));
                }
            }
            "skip" => {
                if arguments.split_whitespace().count() != 1 {
                    return Err(profile_error(
                        path,
                        line_number,
                        "skip requires exactly one source path",
                    ));
                }
                if profile_source(path, line_number, "skip", arguments)? == source {
                    *skipped = true;
                }
            }
            "define" => {
                let (name, value) = arguments
                    .split_once('=')
                    .map(|(name, value)| (name.trim(), Some(value.trim().to_string())))
                    .unwrap_or((arguments, None));
                validate_profile_macro(path, line_number, name)?;
                if value.as_deref().is_some_and(str::is_empty) {
                    return Err(profile_error(
                        path,
                        line_number,
                        "macro value cannot be empty",
                    ));
                }
                undefines.remove(name);
                defines.insert(name.to_string(), value);
            }
            "undef" => {
                if arguments.split_whitespace().count() != 1 {
                    return Err(profile_error(
                        path,
                        line_number,
                        "undef requires exactly one macro name",
                    ));
                }
                validate_profile_macro(path, line_number, arguments)?;
                defines.remove(arguments);
                undefines.insert(arguments.to_string());
            }
            _ => {
                return Err(profile_error(
                    path,
                    line_number,
                    "expected `define NAME[=VALUE]`, `undef NAME`, `namespace SOURCE::MODULE=HEX_ID`, `name SOURCE::MODULE=NAME`, or `skip SOURCE`",
                ));
            }
        }
    }
    Ok(())
}

/// The file a profile line names, relative to the profile's directory.
fn profile_source(
    path: &Path,
    line_number: usize,
    directive: &str,
    relative_source: &str,
) -> Result<PathBuf, RustBackendError> {
    let relative_source = Path::new(relative_source.trim());
    if relative_source.as_os_str().is_empty()
        || relative_source
            .components()
            .any(|component| !matches!(component, std::path::Component::Normal(_)))
    {
        return Err(profile_error(
            path,
            line_number,
            &format!("{directive} source must be a relative path inside the profile directory"),
        ));
    }
    let selected_source = path
        .parent()
        .unwrap_or_else(|| Path::new("."))
        .join(relative_source);
    if !selected_source.is_file() {
        return Err(profile_error(
            path,
            line_number,
            &format!("{directive} source file does not exist"),
        ));
    }
    selected_source.canonicalize().map_err(|error| {
        profile_error(
            path,
            line_number,
            &format!("cannot resolve {directive} source: {error}"),
        )
    })
}

fn validate_profile_macro(
    path: &Path,
    line_number: usize,
    name: &str,
) -> Result<(), RustBackendError> {
    let mut chars = name.chars();
    let valid = chars
        .next()
        .is_some_and(|first| first == '_' || first.is_ascii_alphabetic())
        && chars.all(|character| character == '_' || character.is_ascii_alphanumeric());
    if valid {
        Ok(())
    } else {
        Err(profile_error(path, line_number, "invalid macro name"))
    }
}

fn profile_error(path: &Path, line_number: usize, message: &str) -> RustBackendError {
    RustBackendError::internal(
        path.display().to_string(),
        "<profile>",
        format!("line {line_number}: {message}"),
    )
}

fn module_names_in_source(source: &str) -> Vec<String> {
    let mut scanner = ModuleNameScanner::new(source);
    let mut modules = scanner.collect_modules();
    modules.sort();
    modules
}

struct ModuleNameScanner<'a> {
    source: &'a str,
    cursor: usize,
    at_line_start: bool,
    defines: HashSet<String>,
    conditionals: Vec<ConditionalFrame>,
}

#[derive(Debug, Clone, Copy)]
struct ConditionalFrame {
    parent_active: bool,
    condition_active: bool,
    branch_taken: bool,
}

impl<'a> ModuleNameScanner<'a> {
    fn new(source: &'a str) -> Self {
        Self {
            source,
            cursor: 0,
            at_line_start: true,
            defines: HashSet::new(),
            conditionals: Vec::new(),
        }
    }

    fn collect_modules(&mut self) -> Vec<String> {
        let mut modules = Vec::new();
        while let Some(token) = self.next_identifier() {
            if token == "module" && self.is_active() {
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
                self.handle_preprocessor_directive();
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

    fn handle_preprocessor_directive(&mut self) {
        let directive = self.take_preprocessor_directive();
        let mut parts = directive.split_whitespace();
        let Some(keyword) = parts.next().map(|part| part.trim_start_matches('`')) else {
            return;
        };
        match keyword {
            "define" => {
                if self.is_active()
                    && let Some(name) = parts.next()
                {
                    self.defines.insert(name.to_string());
                }
            }
            "undef" => {
                if self.is_active()
                    && let Some(name) = parts.next()
                {
                    self.defines.remove(name);
                }
            }
            "ifdef" | "ifndef" => {
                let parent_active = self.is_active();
                let defined = parts.next().is_some_and(|name| self.defines.contains(name));
                let condition_active = if keyword == "ifdef" {
                    defined
                } else {
                    !defined
                };
                self.conditionals.push(ConditionalFrame {
                    parent_active,
                    condition_active,
                    branch_taken: parent_active && condition_active,
                });
            }
            "else" => {
                if let Some(frame) = self.conditionals.last_mut() {
                    let activate = frame.parent_active && !frame.branch_taken;
                    frame.condition_active = activate;
                    frame.branch_taken |= activate;
                }
            }
            "elsif" => {
                let defined = parts.next().is_some_and(|name| self.defines.contains(name));
                if let Some(frame) = self.conditionals.last_mut() {
                    let activate = frame.parent_active && !frame.branch_taken && defined;
                    frame.condition_active = activate;
                    frame.branch_taken |= activate;
                }
            }
            "endif" => {
                self.conditionals.pop();
            }
            _ => {}
        }
    }

    fn take_preprocessor_directive(&mut self) -> String {
        let start = self.cursor;
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
                return self.source[start..self.cursor].to_string();
            }
        }
    }

    fn is_active(&self) -> bool {
        self.conditionals
            .last()
            .is_none_or(|frame| frame.parent_active && frame.condition_active)
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

#[cfg(test)]
mod tests {
    /// A `.va` fragment that uses the macros of the file including it is no
    /// compilation root, and reading it alone must not stop discovery: the
    /// preprocessor refuses its undefined macros, where it used to leave them
    /// in the text.
    #[test]
    fn a_fragment_that_needs_its_includers_macros_is_not_a_root() {
        let root = std::env::temp_dir().join(format!(
            "rspice-veriloga-discover-fragment-{}",
            std::process::id()
        ));
        let _ = std::fs::remove_dir_all(&root);
        std::fs::create_dir_all(root.join("inc")).unwrap();
        std::fs::write(
            root.join("top.va"),
            "`define GAIN 2.0\nmodule top(p, n); inout p, n; electrical p, n;\n\
             analog begin\n`include \"inc/body.va\"\nend\nendmodule\n",
        )
        .unwrap();
        std::fs::write(
            root.join("inc").join("body.va"),
            "I(p, n) <+ `GAIN * V(p, n);\n",
        )
        .unwrap();
        let found = super::discover_veriloga_sources(&root);
        let _ = std::fs::remove_dir_all(&root);
        let modules: Vec<_> = found
            .unwrap_or_else(|error| panic!("{error}"))
            .into_iter()
            .flat_map(|candidate| candidate.modules)
            .collect();
        assert_eq!(modules, ["top"]);
    }

    /// A release root that another root includes, as BSIM-CMG 107's
    /// bsimcmg.va includes bsimcmg_main.va after defining __RDSMOD__, is left
    /// out when the profile skips it; a skip that names no file is refused.
    #[test]
    fn a_profile_skip_leaves_an_included_root_out() {
        let root = std::env::temp_dir().join(format!(
            "rspice-veriloga-discover-skip-{}",
            std::process::id()
        ));
        let _ = std::fs::remove_dir_all(&root);
        std::fs::create_dir_all(root.join("code")).unwrap();
        std::fs::write(
            root.join("code").join("main.va"),
            "module dev(p, n); inout p, n; electrical p, n;\n\
             analog I(p, n) <+ V(p, n);\nendmodule\n",
        )
        .unwrap();
        std::fs::write(
            root.join("code").join("dev.va"),
            "`define EXTRA\n`include \"main.va\"\n",
        )
        .unwrap();
        let roots = |profile: &str| {
            std::fs::write(
                root.join(super::VERILOGA_COMPILE_PROFILE_FILE_NAME),
                profile,
            )
            .unwrap();
            super::discover_veriloga_sources(&root).map(|found| {
                found
                    .into_iter()
                    .map(|candidate| {
                        candidate
                            .path
                            .file_name()
                            .unwrap()
                            .to_string_lossy()
                            .into_owned()
                    })
                    .collect::<Vec<_>>()
            })
        };
        let before = roots("");
        let after = roots("skip code/main.va\n");
        let missing = roots("skip code/absent.va\n");
        let _ = std::fs::remove_dir_all(&root);
        assert_eq!(before.unwrap(), ["dev.va", "main.va"]);
        assert_eq!(after.unwrap(), ["dev.va"]);
        assert!(missing.is_err());
    }

    /// `name` gives a module its built-in name; a name that is not an
    /// identifier, or that selects a module the source lacks, is refused.
    #[test]
    fn a_profile_name_names_the_module_it_selects() {
        let root = std::env::temp_dir().join(format!(
            "rspice-veriloga-discover-name-{}",
            std::process::id()
        ));
        let _ = std::fs::remove_dir_all(&root);
        std::fs::create_dir_all(&root).unwrap();
        std::fs::write(
            root.join("dev.va"),
            "module dev(p, n); inout p, n; electrical p, n;\n\
             analog I(p, n) <+ V(p, n);\nendmodule\n",
        )
        .unwrap();
        let names = |profile: &str| {
            std::fs::write(
                root.join(super::VERILOGA_COMPILE_PROFILE_FILE_NAME),
                profile,
            )
            .unwrap();
            super::discover_veriloga_sources(&root).map(|found| {
                found
                    .into_iter()
                    .flat_map(|candidate| candidate.compile_profile.public_names)
                    .collect::<Vec<_>>()
            })
        };
        let named = names("name dev.va::dev=dev107\n");
        let unnamed = names("");
        let bad_name = names("name dev.va::dev=107dev\n");
        let absent_module = names("name dev.va::other=dev107\n");
        let _ = std::fs::remove_dir_all(&root);
        assert_eq!(named.unwrap(), [("dev".to_string(), "dev107".to_string())]);
        assert!(unnamed.unwrap().is_empty());
        assert!(bad_name.is_err());
        assert!(absent_module.is_err());
    }
}
