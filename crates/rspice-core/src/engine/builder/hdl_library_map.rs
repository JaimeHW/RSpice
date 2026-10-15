//! `.options hdl_libmap`: a Verilog library map (IEEE 1364-2005 section 13.2).
//!
//! A map declares libraries and the files each holds:
//!
//! ```text
//! library rtlLib rtl/*.v;
//! library gateLib gate/.../*.vg, extra/;
//! include more.map;
//! config cfg; design work.top; default liblist gateLib rtlLib; endconfig
//! ```
//!
//! A path resolves against the directory of the map that writes it. `*` and
//! `?` match within one path component and `...` any number of directories;
//! a path that ends in `/` names every file of that directory. When a file
//! matches several paths, one that ends in a file name wins over one that
//! ends in a wildcarded name, which wins over a directory; two libraries left
//! after that is an error (section 13.2.1). A file no path matches is in
//! `work`.
//!
//! The map brings in the files it names, as a library declaration names its
//! library's sources: they join the compilation unit after the deck's own
//! sources, in map order. Its `config` blocks are handed to the compiler as
//! written, to be parsed like a configuration in a source.

use crate::{ElaborationError, ElaborationErrorKind, SimulationError};
use std::collections::{HashMap, HashSet};
use std::path::{Path, PathBuf};

/// How deep `include` may nest maps.
const MAX_MAP_DEPTH: usize = 16;
/// How many directory entries the wildcards of one map may visit.
const MAX_VISITED_ENTRIES: usize = 100_000;

fn map_error(
    path: &Path,
    kind: ElaborationErrorKind,
    detail: impl Into<String>,
) -> SimulationError {
    ElaborationError::new(kind, detail).in_source(path).into()
}

/// How a path specification ends, which orders matches of one file
/// (section 13.2.1): a later variant wins.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
enum Ending {
    Directory,
    Wildcard,
    File,
}

/// A file a path names: its canonical path, the path as resolved, and every
/// library that names it with how.
type MappedFile = (PathBuf, PathBuf, Vec<(usize, Ending)>);

/// A library map, read with every map it includes.
#[derive(Debug, Default)]
pub(super) struct LibraryMap {
    /// Library names in declaration order.
    libraries: Vec<String>,
    /// Each file a path names, in map order.
    files: Vec<MappedFile>,
    configs: Vec<rspice_veriloga::MapConfig>,
}

impl LibraryMap {
    /// Read `path` and every map it includes, and expand their paths.
    pub(super) fn read(path: &Path) -> Result<Self, SimulationError> {
        let mut reader = Reader {
            map: Self::default(),
            maps: HashSet::new(),
            index: HashMap::new(),
            visited: 0,
            root: path.to_path_buf(),
        };
        reader.read(path, 0)?;
        // The map files are no sources of a library, whatever a directory or
        // wildcard of theirs says.
        let maps = std::mem::take(&mut reader.maps);
        reader
            .map
            .files
            .retain(|(canonical, _, _)| !maps.contains(canonical));
        Ok(reader.map)
    }

    /// The libraries in declaration order.
    pub(super) fn declared(&self) -> Vec<String> {
        self.libraries.clone()
    }

    /// The map's `config` blocks, in map order.
    pub(super) fn configs(&self) -> Vec<rspice_veriloga::MapConfig> {
        self.configs.clone()
    }

    /// The library `file` is in: the one whose best path names it, or `work`.
    pub(super) fn library_of(&self, file: &Path) -> Result<String, SimulationError> {
        let canonical = super::veriloga_cache::canonicalize_for_cache(file);
        match self.files.iter().find(|(path, _, _)| *path == canonical) {
            Some((_, resolved, matches)) => self.choose(resolved, matches),
            None => Ok("work".into()),
        }
    }

    fn choose(&self, file: &Path, matches: &[(usize, Ending)]) -> Result<String, SimulationError> {
        let best = matches.iter().map(|(_, ending)| *ending).max();
        let mut libraries: Vec<usize> = matches
            .iter()
            .filter(|(_, ending)| Some(*ending) == best)
            .map(|(library, _)| *library)
            .collect();
        libraries.sort_unstable();
        libraries.dedup();
        match libraries.as_slice() {
            [library] => Ok(self.libraries[*library].clone()),
            _ => Err(map_error(
                file,
                ElaborationErrorKind::CompileRefusal,
                format!(
                    "the library map puts '{}' in libraries {}; a file belongs to one library \
                     (IEEE 1364-2005 section 13.2.1)",
                    file.display(),
                    libraries
                        .iter()
                        .map(|library| format!("'{}'", self.libraries[*library]))
                        .collect::<Vec<_>>()
                        .join(" and ")
                ),
            )),
        }
    }

    /// The files the map names that `named` (canonical paths) does not, in
    /// map order, as resolved.
    pub(super) fn other_files(&self, named: &HashSet<PathBuf>) -> Vec<PathBuf> {
        self.files
            .iter()
            .filter(|(canonical, _, _)| !named.contains(canonical))
            .map(|(_, resolved, _)| resolved.clone())
            .collect()
    }
}

struct Reader {
    map: LibraryMap,
    /// Every map read, canonical.
    maps: HashSet<PathBuf>,
    /// Canonical file path to its position in `map.files`.
    index: HashMap<PathBuf, usize>,
    visited: usize,
    root: PathBuf,
}

/// One token of a map: a word, `,` or `;`, with the byte it starts at and
/// its line.
struct Token<'a> {
    text: &'a str,
    start: usize,
    line: usize,
}

/// A map's words: runs of anything but white space, `,` and `;`. A comment
/// begins only where a word could: `rtl/*.v` is a path, not a comment.
fn tokens(text: &str) -> Vec<Token<'_>> {
    let bytes = text.as_bytes();
    let mut tokens = Vec::new();
    let mut at = 0;
    let mut line = 1;
    while at < bytes.len() {
        let byte = bytes[at];
        if byte == b'\n' {
            line += 1;
            at += 1;
        } else if byte.is_ascii_whitespace() {
            at += 1;
        } else if text[at..].starts_with("//") {
            while at < bytes.len() && bytes[at] != b'\n' {
                at += 1;
            }
        } else if text[at..].starts_with("/*") {
            let end = text[at + 2..]
                .find("*/")
                .map_or(bytes.len(), |end| at + 2 + end + 2);
            line += text[at..end].matches('\n').count();
            at = end;
        } else if byte == b',' || byte == b';' {
            tokens.push(Token {
                text: &text[at..at + 1],
                start: at,
                line,
            });
            at += 1;
        } else {
            let start = at;
            while at < bytes.len()
                && !bytes[at].is_ascii_whitespace()
                && bytes[at] != b','
                && bytes[at] != b';'
            {
                at += 1;
            }
            tokens.push(Token {
                text: &text[start..at],
                start,
                line,
            });
        }
    }
    tokens
}

impl Reader {
    fn read(&mut self, path: &Path, depth: usize) -> Result<(), SimulationError> {
        if depth > MAX_MAP_DEPTH {
            return Err(map_error(
                path,
                ElaborationErrorKind::CompileRefusal,
                format!("library maps include one another more than {MAX_MAP_DEPTH} deep"),
            ));
        }
        let canonical = super::veriloga_cache::canonicalize_for_cache(path);
        if !self.maps.insert(canonical) {
            return Err(map_error(
                path,
                ElaborationErrorKind::CompileRefusal,
                "this library map includes itself",
            ));
        }
        let text = std::fs::read_to_string(path).map_err(|error| {
            map_error(
                path,
                ElaborationErrorKind::MissingSource,
                format!("the library map cannot be read: {error}"),
            )
        })?;
        let directory = path.parent().unwrap_or(Path::new("")).to_path_buf();
        let tokens = tokens(&text);
        let mut at = 0;
        let unexpected = |token: Option<&Token<'_>>, expected: &str| {
            map_error(
                path,
                ElaborationErrorKind::CompileRefusal,
                match token {
                    Some(token) => format!(
                        "line {}: '{}' where the library map expects {expected}",
                        token.line, token.text
                    ),
                    None => format!("the library map ends where it expects {expected}"),
                },
            )
        };
        while let Some(token) = tokens.get(at) {
            match token.text {
                ";" => at += 1,
                "library" => {
                    let name = tokens
                        .get(at + 1)
                        .filter(|name| rspice_veriloga::UnitLibraries::is_library_name(name.text))
                        .ok_or_else(|| unexpected(tokens.get(at + 1), "a library name"))?;
                    let library = match self
                        .map
                        .libraries
                        .iter()
                        .position(|known| known == name.text)
                    {
                        Some(known) => known,
                        None => {
                            self.map.libraries.push(name.text.to_owned());
                            self.map.libraries.len() - 1
                        }
                    };
                    at += 2;
                    let incdir = |token: &Token<'_>| {
                        map_error(
                            path,
                            ElaborationErrorKind::CompileRefusal,
                            format!(
                                "line {}: a library's `-incdir` include directories are not \
                                 supported; give the include directories to the deck instead",
                                token.line
                            ),
                        )
                    };
                    loop {
                        let spec = tokens
                            .get(at)
                            .filter(|spec| spec.text != "," && spec.text != ";")
                            .ok_or_else(|| unexpected(tokens.get(at), "a file path"))?;
                        if spec.text == "-incdir" {
                            return Err(incdir(spec));
                        }
                        self.expand(path, &directory, spec.text, library)?;
                        at += 1;
                        match tokens.get(at) {
                            Some(token) if token.text == "," => at += 1,
                            Some(token) if token.text == ";" => {
                                at += 1;
                                break;
                            }
                            Some(token) if token.text == "-incdir" => return Err(incdir(token)),
                            other => return Err(unexpected(other, "`,` or `;`")),
                        }
                    }
                }
                "include" => {
                    let spec = tokens
                        .get(at + 1)
                        .filter(|spec| spec.text != "," && spec.text != ";")
                        .ok_or_else(|| unexpected(tokens.get(at + 1), "a library map path"))?;
                    if tokens.get(at + 2).map(|token| token.text) != Some(";") {
                        return Err(unexpected(tokens.get(at + 2), "`;`"));
                    }
                    let included = resolve(&directory, spec.text);
                    self.read(&included, depth + 1)?;
                    at += 3;
                }
                "config" => {
                    let mut end = at + 1;
                    let close = loop {
                        let Some(word) = tokens.get(end) else {
                            return Err(unexpected(None, "`endconfig`"));
                        };
                        if word.text == "endconfig" || word.text.starts_with("endconfig:") {
                            break end;
                        }
                        end += 1;
                    };
                    // IEEE 1800-2017's `endconfig : name`.
                    let mut last = close;
                    if tokens[close].text == "endconfig"
                        && tokens.get(close + 1).is_some_and(|token| token.text == ":")
                    {
                        last = (close + 2).min(tokens.len() - 1);
                    } else if tokens[close].text == "endconfig"
                        && tokens
                            .get(close + 1)
                            .is_some_and(|token| token.text.starts_with(':'))
                    {
                        last = close + 1;
                    }
                    let finish = tokens[last].start + tokens[last].text.len();
                    self.map.configs.push(rspice_veriloga::MapConfig {
                        path: path.to_path_buf(),
                        line: token.line,
                        text: text[token.start..finish].to_owned(),
                    });
                    at = last + 1;
                }
                _ => {
                    return Err(unexpected(
                        Some(token),
                        "a `library`, `include` or `config` statement",
                    ));
                }
            }
        }
        Ok(())
    }

    /// Add every file `spec` names to `library`.
    fn expand(
        &mut self,
        map: &Path,
        directory: &Path,
        spec: &str,
        library: usize,
    ) -> Result<(), SimulationError> {
        let names_directory = spec.ends_with('/') || spec.ends_with('\\');
        let resolved = resolve(directory, spec.trim_end_matches(['/', '\\']));
        let components: Vec<String> = resolved
            .components()
            .map(|component| component.as_os_str().to_string_lossy().into_owned())
            .collect();
        let wildcard = |part: &str| part.contains(['*', '?']) || part == "...";
        let ending = if names_directory {
            Ending::Directory
        } else if components.last().is_some_and(|last| wildcard(last)) {
            Ending::Wildcard
        } else {
            Ending::File
        };
        // The literal prefix, then a walk over the rest.
        let first_pattern = components
            .iter()
            .position(|part| wildcard(part))
            .unwrap_or(components.len());
        let base: PathBuf = components[..first_pattern].iter().collect();
        let mut found = Vec::new();
        if first_pattern == components.len() {
            if names_directory {
                self.directory_files(map, &base, &mut found)?;
            } else if base.is_file() {
                found.push(base);
            }
        } else {
            let mut directories = vec![base];
            let rest = &components[first_pattern..];
            for (position, part) in rest.iter().enumerate() {
                let last = position + 1 == rest.len();
                let mut next = Vec::new();
                for directory in &directories {
                    if part == "..." {
                        self.descendants(map, directory, &mut next)?;
                        continue;
                    }
                    for entry in self.entries(map, directory)? {
                        let name = entry
                            .file_name()
                            .map(|name| name.to_string_lossy().into_owned());
                        if name.is_some_and(|name| matches(part, &name)) {
                            next.push(entry);
                        }
                    }
                }
                directories = next;
                if last {
                    if names_directory || part == "..." {
                        for directory in std::mem::take(&mut directories) {
                            self.directory_files(map, &directory, &mut found)?;
                        }
                    } else {
                        found.extend(directories.drain(..).filter(|path| path.is_file()));
                    }
                } else {
                    directories.retain(|path| path.is_dir());
                }
            }
        }
        found.sort();
        for file in found {
            let canonical = super::veriloga_cache::canonicalize_for_cache(&file);
            let position = *self.index.entry(canonical.clone()).or_insert_with(|| {
                self.map.files.push((canonical, file, Vec::new()));
                self.map.files.len() - 1
            });
            self.map.files[position].2.push((library, ending));
        }
        Ok(())
    }

    fn entries(&mut self, map: &Path, directory: &Path) -> Result<Vec<PathBuf>, SimulationError> {
        let Ok(read) = std::fs::read_dir(directory) else {
            return Ok(Vec::new());
        };
        let mut entries = Vec::new();
        for entry in read.flatten() {
            self.visited += 1;
            if self.visited > MAX_VISITED_ENTRIES {
                return Err(map_error(
                    map,
                    ElaborationErrorKind::CompileRefusal,
                    format!(
                        "the library map's paths visit more than {MAX_VISITED_ENTRIES} directory \
                         entries; name narrower directories (map '{}')",
                        self.root.display()
                    ),
                ));
            }
            entries.push(entry.path());
        }
        entries.sort();
        Ok(entries)
    }

    fn directory_files(
        &mut self,
        map: &Path,
        directory: &Path,
        found: &mut Vec<PathBuf>,
    ) -> Result<(), SimulationError> {
        found.extend(
            self.entries(map, directory)?
                .into_iter()
                .filter(|path| path.is_file()),
        );
        Ok(())
    }

    /// `directory` and every directory below it.
    fn descendants(
        &mut self,
        map: &Path,
        directory: &Path,
        out: &mut Vec<PathBuf>,
    ) -> Result<(), SimulationError> {
        let mut pending = vec![directory.to_path_buf()];
        while let Some(next) = pending.pop() {
            for entry in self.entries(map, &next)? {
                if entry.is_dir() {
                    pending.push(entry);
                }
            }
            out.push(next);
        }
        out.sort();
        out.dedup();
        Ok(())
    }
}

/// `spec` against the directory of the map that writes it.
fn resolve(directory: &Path, spec: &str) -> PathBuf {
    let path = Path::new(spec);
    if path.has_root() || path.is_absolute() {
        path.to_path_buf()
    } else {
        directory.join(path)
    }
}

/// A path component against a pattern of `*` (any run) and `?` (one
/// character).
fn matches(pattern: &str, name: &str) -> bool {
    let pattern: Vec<char> = pattern.chars().collect();
    let name: Vec<char> = name.chars().collect();
    let (mut p, mut n) = (0, 0);
    let mut star: Option<(usize, usize)> = None;
    while n < name.len() {
        if p < pattern.len() && (pattern[p] == '?' || pattern[p] == name[n]) {
            p += 1;
            n += 1;
        } else if p < pattern.len() && pattern[p] == '*' {
            star = Some((p, n));
            p += 1;
        } else if let Some((star_p, star_n)) = star {
            p = star_p + 1;
            n = star_n + 1;
            star = Some((star_p, star_n + 1));
        } else {
            return false;
        }
    }
    pattern[p..].iter().all(|&character| character == '*')
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn wildcards_match_within_one_component() {
        assert!(matches("*.v", "leaf.v"));
        assert!(matches("l??f.v", "leaf.v"));
        assert!(matches("*", ""));
        assert!(matches("a*b*c", "aXXbYc"));
        assert!(!matches("*.v", "leaf.vg"));
        assert!(!matches("?.v", "ab.v"));
    }

    #[test]
    fn a_comment_begins_only_where_a_word_could() {
        let words: Vec<&str> = tokens("library a rtl/*.v; // note\n/* x */ include b.map;")
            .iter()
            .map(|token| token.text)
            .collect();
        assert_eq!(
            words,
            ["library", "a", "rtl/*.v", ";", "include", "b.map", ";"]
        );
    }
}
