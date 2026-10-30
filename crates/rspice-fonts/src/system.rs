//! The faces installed on a desktop, for the characters no embedded face has.
//!
//! A worker thread reads the font files, so a frame never waits on a disk. It
//! opens only the few files each operating system ships for the scripts the
//! embedded faces lack (see [`Source::system`]). A character none of those has
//! sends it through every installed face, once, and most sessions never need
//! that.
//!
//! A face joins only when a character asks for it. Its file is then read whole
//! into memory kept for the life of the process: egui holds the bytes of every
//! face it letters with and parses them again whenever its fonts are rebuilt,
//! so a font file replaced on disk afterwards changes nothing on screen. The
//! copy is parsed before it is handed over, since egui stops the application on
//! a face it cannot parse.
//!
//! A face joins in its upright Regular cut, which every role borrows whatever
//! its weight.

mod known;

use std::collections::{BTreeMap, HashSet};
use std::fmt;
use std::path::{Path, PathBuf};
use std::sync::Mutex;
use std::sync::mpsc::{self, Receiver, Sender};
use std::time::Duration;

use skrifa::{FontRef, MetadataProvider, charmap::Charmap};

use crate::HanVariant;

/// What a system face is to a chain, which decides where in it the face
/// stands.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum FaceClass {
    /// The operating system's face for a script the embedded faces lack, such
    /// as Arabic or Thai.
    Script,
    /// A face for one written form of the Han characters, with its Kana or
    /// Hangul.
    Han(HanVariant),
    /// A face for a rarer script or for symbols, which stands after the CJK
    /// faces so it never letters their punctuation.
    Extra,
    /// A face the scan of every installed face found.
    Found,
}

/// A face named ahead of time: a file, and the family in it to letter with.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct KnownFace {
    pub path: PathBuf,
    pub family: String,
    pub class: FaceClass,
}

impl KnownFace {
    #[must_use]
    pub fn new(path: impl Into<PathBuf>, family: impl Into<String>, class: FaceClass) -> Self {
        Self {
            path: path.into(),
            family: family.into(),
            class,
        }
    }
}

/// Where the worker looks for faces.
pub struct Source {
    /// The faces to offer, or none to offer the operating system's own and
    /// then every installed face.
    known: Option<Vec<KnownFace>>,
    gate: Option<Receiver<()>>,
}

impl Source {
    /// The operating system's faces: the files it ships for the scripts the
    /// embedded faces lack, then, for a character none of them has, every
    /// face installed on the machine.
    #[must_use]
    pub fn system() -> Self {
        Self {
            known: None,
            gate: None,
        }
    }

    /// Exactly these faces, in this order, and no others.
    #[must_use]
    pub fn files(known: Vec<KnownFace>) -> Self {
        Self {
            known: Some(known),
            gate: None,
        }
    }

    /// Opens no file until `gate` receives a message or disconnects.
    #[must_use]
    pub fn after(mut self, gate: Receiver<()>) -> Self {
        self.gate = Some(gate);
        self
    }
}

/// A face that joins the chains, read and parsed.
pub struct SystemFace {
    family: String,
    path: PathBuf,
    index: u32,
    class: FaceClass,
    position: usize,
    bytes: &'static [u8],
    charmap: Charmap<'static>,
}

impl SystemFace {
    /// Reads the candidate's file and holds the copy to what was scanned: the
    /// file may have changed since.
    fn load(candidate: &Candidate, covered: &[char]) -> Option<Self> {
        let bytes = std::fs::read(&candidate.path).ok()?;
        let font = FontRef::from_index(&bytes, candidate.index).ok()?;
        let charmap = font.charmap();
        if font.outline_glyphs().format().is_none()
            || !covered
                .iter()
                .all(|&character| charmap.map(character).is_some())
        {
            return None;
        }
        let bytes: &'static [u8] = bytes.leak();
        let charmap = FontRef::from_index(bytes, candidate.index).ok()?.charmap();
        Some(Self {
            family: candidate.family.clone(),
            path: candidate.path.clone(),
            index: candidate.index,
            class: candidate.class,
            position: candidate.position,
            bytes,
            charmap,
        })
    }

    #[must_use]
    pub fn family(&self) -> &str {
        &self.family
    }

    #[must_use]
    pub fn path(&self) -> &Path {
        &self.path
    }

    /// The face's index in its file, which a collection holds several of.
    #[must_use]
    pub const fn index(&self) -> u32 {
        self.index
    }

    #[must_use]
    pub const fn class(&self) -> FaceClass {
        self.class
    }

    /// The face's file, as it was read when the face joined.
    #[must_use]
    pub const fn bytes(&self) -> &'static [u8] {
        self.bytes
    }

    /// Whether the face has a glyph for `character`.
    #[must_use]
    pub fn covers(&self, character: char) -> bool {
        self.charmap.map(character).is_some()
    }

    /// A stable identifier for the face: egui's font-data key.
    #[must_use]
    pub fn key(&self) -> String {
        format!("system:{}#{}", self.path.display(), self.index)
    }

    /// Where the face stands among the system faces of every chain: the
    /// script faces, then the CJK faces in `order`, then the rarer scripts
    /// and symbols, then what the scan found, each in the order it is named.
    #[must_use]
    pub fn precedence(&self, order: &[HanVariant; 5]) -> (u8, usize) {
        precedence(self.class, self.position, order)
    }
}

impl fmt::Debug for SystemFace {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter
            .debug_struct("SystemFace")
            .field("family", &self.family)
            .field("path", &self.path)
            .field("index", &self.index)
            .field("class", &self.class)
            .finish_non_exhaustive()
    }
}

fn precedence(class: FaceClass, position: usize, order: &[HanVariant; 5]) -> (u8, usize) {
    match class {
        FaceClass::Script => (0, position),
        FaceClass::Han(form) => (
            1,
            order
                .iter()
                .position(|wanted| *wanted == form)
                .unwrap_or(order.len()),
        ),
        FaceClass::Extra => (2, position),
        FaceClass::Found => (3, position),
    }
}

/// What the worker made of one request.
#[derive(Debug, Default)]
pub struct Resolution {
    /// The faces that join, for characters the faces already there lack.
    pub joined: Vec<SystemFace>,
    /// The characters no face on the machine has.
    pub missing: Vec<char>,
}

enum Request {
    Characters(Vec<char>),
    Reorder([HanVariant; 5], Vec<char>),
}

/// The worker that finds faces for characters, and the way to it.
///
/// Every call returns at once; the answers queue until they are taken.
pub struct SystemFonts {
    requests: Sender<Request>,
    resolutions: Mutex<Receiver<Resolution>>,
}

impl SystemFonts {
    /// Starts the worker. It opens `source`'s files straight away, before
    /// any character is asked for, and stops once this handle is dropped.
    #[must_use]
    pub fn spawn(source: Source, order: [HanVariant; 5]) -> Self {
        let (requests, incoming) = mpsc::channel();
        let (outgoing, resolutions) = mpsc::channel();
        // A machine that cannot start a thread letters what the embedded
        // faces lack as the replacement square, as it would without this.
        let _ = std::thread::Builder::new()
            .name("rspice-system-fonts".to_owned())
            .spawn(move || run(source, order, &incoming, &outgoing));
        Self {
            requests,
            resolutions: Mutex::new(resolutions),
        }
    }

    /// Asks for faces that have `characters`. Answered by one [`Resolution`].
    pub fn request(&self, characters: Vec<char>) {
        let _ = self.requests.send(Request::Characters(characters));
    }

    /// Changes the order of the CJK faces, and asks again for `characters`,
    /// which may now want a face of another form. Answered by one
    /// [`Resolution`].
    pub fn reorder(&self, order: [HanVariant; 5], characters: Vec<char>) {
        let _ = self.requests.send(Request::Reorder(order, characters));
    }

    /// The next answer, if one is ready.
    #[must_use]
    pub fn try_resolution(&self) -> Option<Resolution> {
        self.resolutions.lock().ok()?.try_recv().ok()
    }

    /// The next answer, waiting up to `timeout` for it. A frame never calls
    /// this.
    #[must_use]
    pub fn wait_resolution(&self, timeout: Duration) -> Option<Resolution> {
        self.resolutions.lock().ok()?.recv_timeout(timeout).ok()
    }
}

fn run(
    source: Source,
    mut order: [HanVariant; 5],
    requests: &Receiver<Request>,
    resolutions: &Sender<Resolution>,
) {
    if let Some(gate) = &source.gate {
        let _ = gate.recv();
    }
    let scan = source.known.is_none();
    let mut known = Catalog::probe(&source.known.unwrap_or_else(known::faces));
    let mut found: Option<Catalog> = None;
    let mut joined = HashSet::new();
    for request in requests {
        let mut pending = match request {
            Request::Characters(characters) => characters,
            Request::Reorder(new_order, characters) => {
                order = new_order;
                characters
            }
        };
        pending.sort_unstable();
        pending.dedup();
        let mut resolution = Resolution::default();
        known.resolve(&order, &mut joined, &mut pending, &mut resolution);
        if !pending.is_empty() && scan {
            let found = found.get_or_insert_with(|| Catalog::scan(&known));
            found.resolve(&order, &mut joined, &mut pending, &mut resolution);
        }
        resolution.missing = pending;
        if resolutions.send(resolution).is_err() {
            return;
        }
    }
}

/// A face the worker may join, as the font database read it.
struct Candidate {
    id: fontdb::ID,
    family: String,
    path: PathBuf,
    index: u32,
    class: FaceClass,
    position: usize,
    /// Whether the face failed to load, so it is never offered again.
    refused: bool,
}

struct Catalog {
    database: fontdb::Database,
    candidates: Vec<Candidate>,
}

impl Catalog {
    /// Reads the named faces' files, and keeps each one's upright face of
    /// the named family nearest the Regular weight. A file that is absent or
    /// unreadable is passed over.
    fn probe(known: &[KnownFace]) -> Self {
        let mut database = fontdb::Database::new();
        let mut read = HashSet::new();
        let mut candidates = Vec::new();
        for (position, face) in known.iter().enumerate() {
            if read.insert(face.path.clone()) && database.load_font_file(&face.path).is_err() {
                continue;
            }
            let chosen = database
                .faces()
                .filter(|info| {
                    file_of(info) == Some(face.path.as_path())
                        && info.style == fontdb::Style::Normal
                        && info
                            .families
                            .iter()
                            .any(|(family, _)| family.eq_ignore_ascii_case(&face.family))
                })
                .min_by_key(|info| (info.weight.0.abs_diff(400), info.index));
            if let Some(info) = chosen {
                candidates.push(Candidate {
                    id: info.id,
                    family: face.family.clone(),
                    path: face.path.clone(),
                    index: info.index,
                    class: face.class,
                    position,
                    refused: false,
                });
            }
        }
        Self {
            database,
            candidates,
        }
    }

    /// Reads every face installed on the machine and keeps, per family, the
    /// upright face nearest the Regular weight, in family-name order. The
    /// faces `known` already offers are left out.
    fn scan(known: &Self) -> Self {
        let offered: HashSet<(&Path, u32)> = known
            .candidates
            .iter()
            .map(|candidate| (candidate.path.as_path(), candidate.index))
            .collect();
        let mut database = fontdb::Database::new();
        database.load_system_fonts();
        let mut nearest: BTreeMap<&str, &fontdb::FaceInfo> = BTreeMap::new();
        for info in database.faces() {
            let (Some(path), Some((family, _))) = (file_of(info), info.families.first()) else {
                continue;
            };
            if info.style != fontdb::Style::Normal || offered.contains(&(path, info.index)) {
                continue;
            }
            nearest
                .entry(family.as_str())
                .and_modify(|kept| {
                    if regular_first(info) < regular_first(kept) {
                        *kept = info;
                    }
                })
                .or_insert(info);
        }
        let candidates = nearest
            .into_iter()
            .enumerate()
            .filter_map(|(position, (family, info))| {
                Some(Candidate {
                    id: info.id,
                    family: family.to_owned(),
                    path: file_of(info)?.to_path_buf(),
                    index: info.index,
                    class: FaceClass::Found,
                    position,
                    refused: false,
                })
            })
            .collect();
        Self {
            database,
            candidates,
        }
    }

    /// Joins, for each pending character, the first face in precedence order
    /// that has it, and removes from `pending` every character a joined face
    /// has. A character already lettered by a face that joined earlier asks
    /// for nothing, unless a face of a form now ordered ahead of it has it.
    fn resolve(
        &mut self,
        order: &[HanVariant; 5],
        joined: &mut HashSet<(PathBuf, u32)>,
        pending: &mut Vec<char>,
        resolution: &mut Resolution,
    ) {
        let mut ranked: Vec<usize> = (0..self.candidates.len()).collect();
        ranked.sort_by_key(|&at| {
            let candidate = &self.candidates[at];
            precedence(candidate.class, candidate.position, order)
        });
        for at in ranked {
            if pending.is_empty() {
                return;
            }
            let candidate = &self.candidates[at];
            if candidate.refused {
                continue;
            }
            let covered = self
                .database
                .with_face_data(candidate.id, |data, index| {
                    FontRef::from_index(data, index)
                        .map(|font| {
                            let charmap = font.charmap();
                            pending
                                .iter()
                                .copied()
                                .filter(|&character| charmap.map(character).is_some())
                                .collect::<Vec<_>>()
                        })
                        .unwrap_or_default()
                })
                .unwrap_or_default();
            if covered.is_empty() {
                continue;
            }
            let identity = (candidate.path.clone(), candidate.index);
            if !joined.contains(&identity) {
                let Some(face) = SystemFace::load(candidate, &covered) else {
                    self.candidates[at].refused = true;
                    continue;
                };
                joined.insert(identity);
                resolution.joined.push(face);
            }
            pending.retain(|character| !covered.contains(character));
        }
    }
}

/// Orders a family's faces by distance from the Regular weight, then by file
/// and index so the choice is the same on every scan.
fn regular_first(info: &fontdb::FaceInfo) -> (u16, Option<&Path>, u32) {
    (info.weight.0.abs_diff(400), file_of(info), info.index)
}

fn file_of(info: &fontdb::FaceInfo) -> Option<&Path> {
    match &info.source {
        fontdb::Source::File(path) | fontdb::Source::SharedFile(path, _) => Some(path),
        fontdb::Source::Binary(_) => None,
    }
}

#[cfg(test)]
mod tests;
