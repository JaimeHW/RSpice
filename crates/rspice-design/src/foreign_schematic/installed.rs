//! The source program's own files, read in place.
//!
//! A drawing names symbols, model files and model cards that live beside it
//! or in the source program's installed library. They are read where they
//! are; nothing is copied.

use std::cell::OnceCell;
use std::collections::HashMap;
use std::path::{Path, PathBuf};
use std::rc::Rc;

use super::import::{ForeignLibrary, ForeignModelFile, ForeignSymbolSource};
use super::{ForeignTool, decode_foreign_text, parse_asy};

/// A drawing's folder and the source program's installed library.
pub struct InstalledForeignLibrary {
    beside: PathBuf,
    /// Shared by every drawing of one import, so the installation is
    /// indexed and its model cards read once.
    install: Rc<Installation>,
}

/// The source program's installed symbols and models.
#[derive(Default)]
struct Installation {
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
        Self::new(tool, folder_of(drawing), install.as_deref())
    }

    /// The library of a drawing in `beside` with the source program's
    /// library at `install`: LTspice's `lib` folder, or QSPICE's install
    /// folder.
    pub fn new(tool: ForeignTool, beside: &Path, install: Option<&Path>) -> Self {
        let mut installation = Installation::default();
        match (tool, install) {
            (_, None) => {}
            (ForeignTool::Ltspice, Some(install)) => {
                installation.symbol_roots.push(install.join("sym"));
                installation.model_roots.push(install.join("sub"));
                installation.model_roots.push(install.join("cmp"));
                installation.card_files = files_in(&install.join("cmp"), |name| {
                    name.to_ascii_lowercase().starts_with("standard.")
                });
            }
            (ForeignTool::Qspice, Some(install)) => {
                installation.model_roots.push(install.to_path_buf());
                installation.card_files =
                    files_in(install, |name| name.to_ascii_lowercase().ends_with(".txt"));
            }
        }
        Self {
            beside: beside.to_path_buf(),
            install: Rc::new(installation),
        }
    }

    /// The same installation read for the drawing at `drawing`: a block's
    /// schematic, or the next drawing of a batch.
    #[must_use]
    pub fn for_another_drawing(&self, drawing: &Path) -> Self {
        Self {
            beside: folder_of(drawing).to_path_buf(),
            install: Rc::clone(&self.install),
        }
    }

    fn indexed_symbol(&self, name: &str) -> Option<&PathBuf> {
        let index = self.install.symbol_index.get_or_init(|| {
            let mut index = HashMap::new();
            for root in &self.install.symbol_roots {
                index_symbols(root, &mut index);
            }
            index
        });
        let stem = name.rsplit(['/', '\\']).next().unwrap_or(name);
        index.get(&stem.to_ascii_lowercase())
    }

    fn card_texts(&self) -> &[String] {
        self.install.cards.get_or_init(|| {
            self.install
                .card_files
                .iter()
                .filter_map(|path| std::fs::read(path).ok())
                .map(|bytes| decode_foreign_text(&bytes))
                .collect()
        })
    }
}

fn folder_of(drawing: &Path) -> &Path {
    drawing.parent().unwrap_or(Path::new("."))
}

/// The first bytes of a file, enough to tell an encrypted one.
fn file_head(path: &Path) -> Option<Vec<u8>> {
    use std::io::Read;
    let mut head = Vec::with_capacity(256);
    std::fs::File::open(path)
        .ok()?
        .take(256)
        .read_to_end(&mut head)
        .ok()?;
    Some(head)
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
            .chain(&self.install.symbol_roots)
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
            std::iter::once(&self.beside)
                .chain(&self.install.model_roots)
                .map(|root| root.join(&relative))
                .find(|path| path.is_file())
        }?;
        let encrypted = file_head(&path).is_some_and(|head| is_encrypted(&head));
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
