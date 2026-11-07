//! The source program's own files, read in place.
//!
//! A drawing names symbols, model files and model cards that live beside it
//! or in the source program's installed library. They are read where they
//! are; nothing is copied.

use std::cell::OnceCell;
use std::collections::HashMap;
use std::path::{Path, PathBuf};

use super::import::{ForeignLibrary, ForeignModelFile, ForeignSymbolSource};
use super::{ForeignTool, decode_foreign_text, parse_asy};

/// A drawing's folder and the source program's installed library.
pub struct InstalledForeignLibrary {
    beside: PathBuf,
    symbol_roots: Vec<PathBuf>,
    /// Every installed symbol by its lower-case file stem, for a drawing
    /// that names a symbol without its folder, as LTspice allows.
    symbol_index: OnceCell<HashMap<String, PathBuf>>,
    model_roots: Vec<PathBuf>,
    card_files: Vec<PathBuf>,
    cards: OnceCell<Vec<String>>,
}

impl InstalledForeignLibrary {
    /// The library a drawing at `drawing` reads, with the source program's
    /// installation found where it installs itself.
    pub fn for_drawing(tool: ForeignTool, drawing: &Path) -> Self {
        let install = match tool {
            ForeignTool::Ltspice => ltspice_library_root(),
            ForeignTool::Qspice => qspice_install_root(),
        };
        Self::new(
            tool,
            drawing.parent().unwrap_or(Path::new(".")),
            install.as_deref(),
        )
    }

    /// The library of a drawing in `beside` with the source program's
    /// library at `install`: LTspice's `lib` folder, or QSPICE's install
    /// folder.
    pub fn new(tool: ForeignTool, beside: &Path, install: Option<&Path>) -> Self {
        let mut library = Self {
            beside: beside.to_path_buf(),
            symbol_roots: Vec::new(),
            symbol_index: OnceCell::new(),
            model_roots: vec![beside.to_path_buf()],
            card_files: Vec::new(),
            cards: OnceCell::new(),
        };
        let Some(install) = install else {
            return library;
        };
        match tool {
            ForeignTool::Ltspice => {
                library.symbol_roots.push(install.join("sym"));
                library.model_roots.push(install.join("sub"));
                library.model_roots.push(install.join("cmp"));
                library.card_files = files_in(&install.join("cmp"), |name| {
                    name.to_ascii_lowercase().starts_with("standard.")
                });
            }
            ForeignTool::Qspice => {
                library.model_roots.push(install.to_path_buf());
                library.card_files =
                    files_in(install, |name| name.to_ascii_lowercase().ends_with(".txt"));
            }
        }
        library
    }

    fn indexed_symbol(&self, name: &str) -> Option<&PathBuf> {
        let index = self.symbol_index.get_or_init(|| {
            let mut index = HashMap::new();
            for root in &self.symbol_roots {
                index_symbols(root, &mut index);
            }
            index
        });
        let stem = name.rsplit(['/', '\\']).next().unwrap_or(name);
        index.get(&stem.to_ascii_lowercase())
    }

    fn card_texts(&self) -> &[String] {
        self.cards.get_or_init(|| {
            self.card_files
                .iter()
                .filter_map(|path| std::fs::read(path).ok())
                .map(|bytes| decode_foreign_text(&bytes))
                .collect()
        })
    }
}

impl ForeignLibrary for InstalledForeignLibrary {
    fn symbol(&self, name: &str) -> Option<ForeignSymbolSource> {
        let relative = name
            .split(['/', '\\'])
            .filter(|part| !part.is_empty())
            .collect::<Vec<_>>()
            .join("/");
        let relative = format!("{relative}.asy");
        let candidates = std::iter::once(&self.beside)
            .chain(&self.symbol_roots)
            .map(|root| root.join(&relative))
            .chain(self.indexed_symbol(name).cloned());
        for path in candidates {
            let Ok(bytes) = std::fs::read(&path) else {
                continue;
            };
            let Ok(definition) = parse_asy(&decode_foreign_text(&bytes)) else {
                continue;
            };
            // A symbol drawn beside a schematic of the same name is a block
            // whose insides that schematic draws.
            let schematic = Some(path.with_extension("asc")).filter(|schematic| {
                path.parent() == Some(self.beside.as_path()) && schematic.is_file()
            });
            return Some(ForeignSymbolSource {
                definition,
                location: path.display().to_string(),
                schematic,
            });
        }
        None
    }

    fn model_file(&self, name: &str) -> Option<ForeignModelFile> {
        let relative = PathBuf::from(name.trim_matches('"').replace('\\', "/"));
        let path = if relative.is_absolute() {
            Some(relative).filter(|path| path.is_file())
        } else {
            self.model_roots
                .iter()
                .map(|root| root.join(&relative))
                .find(|path| path.is_file())
        }?;
        let encrypted = std::fs::read(&path)
            .map(|bytes| is_encrypted(&bytes))
            .unwrap_or(false);
        Some(ForeignModelFile { path, encrypted })
    }

    fn model_card(&self, name: &str) -> Option<String> {
        self.card_texts()
            .iter()
            .find_map(|text| model_card_in(text, name))
    }
}

/// Whether a model file is one only its vendor's simulator can read.
pub fn is_encrypted(bytes: &[u8]) -> bool {
    let head = decode_foreign_text(&bytes[..bytes.len().min(256)]);
    let head = head.trim_start();
    head.starts_with("* LTspice Encrypted File") || head.starts_with("<Binary File>")
}

/// The `.model` card naming `name` in a model file, continuation lines
/// included.
fn model_card_in(text: &str, name: &str) -> Option<String> {
    let mut lines = text.lines();
    while let Some(line) = lines.next() {
        let mut words = line.split_whitespace();
        let is_card = words
            .next()
            .is_some_and(|word| word.eq_ignore_ascii_case(".model"));
        let named = words.next().is_some_and(|word| {
            let word = word.split('(').next().unwrap_or(word);
            word.eq_ignore_ascii_case(name)
        });
        if !(is_card && named) {
            continue;
        }
        let mut card = line.trim_end().to_owned();
        for next in lines.by_ref() {
            if !next.trim_start().starts_with('+') {
                break;
            }
            card.push('\n');
            card.push_str(next.trim_end());
        }
        return Some(card);
    }
    None
}

fn index_symbols(folder: &Path, index: &mut HashMap<String, PathBuf>) {
    let Ok(entries) = std::fs::read_dir(folder) else {
        return;
    };
    let mut paths = entries
        .filter_map(Result::ok)
        .map(|entry| entry.path())
        .collect::<Vec<_>>();
    paths.sort();
    for path in paths {
        if path.is_dir() {
            index_symbols(&path, index);
        } else if path
            .extension()
            .is_some_and(|extension| extension.eq_ignore_ascii_case("asy"))
            && let Some(stem) = path.file_stem().and_then(|stem| stem.to_str())
        {
            index.entry(stem.to_ascii_lowercase()).or_insert(path);
        }
    }
}

fn files_in(folder: &Path, wanted: impl Fn(&str) -> bool) -> Vec<PathBuf> {
    let mut files = std::fs::read_dir(folder)
        .map(|entries| {
            entries
                .filter_map(Result::ok)
                .map(|entry| entry.path())
                .filter(|path| {
                    path.is_file()
                        && path
                            .file_name()
                            .and_then(|name| name.to_str())
                            .is_some_and(&wanted)
                })
                .collect::<Vec<_>>()
        })
        .unwrap_or_default();
    files.sort();
    files
}

/// LTspice's `lib` folder: under the local application data folder since
/// LTspice 24, under Documents before it.
fn ltspice_library_root() -> Option<PathBuf> {
    let candidates = [
        std::env::var_os("LOCALAPPDATA").map(|root| PathBuf::from(root).join("LTspice/lib")),
        std::env::var_os("USERPROFILE")
            .map(|root| PathBuf::from(root).join("Documents/LTspiceXVII/lib")),
    ];
    candidates
        .into_iter()
        .flatten()
        .find(|path| path.join("sym").is_dir())
}

fn qspice_install_root() -> Option<PathBuf> {
    std::env::var_os("ProgramFiles")
        .map(|root| PathBuf::from(root).join("QSPICE"))
        .filter(|path| path.is_dir())
}
