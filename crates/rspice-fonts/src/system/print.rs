//! The installed faces print letters from, for the characters no embedded
//! face has.
//!
//! Print looks only where the application says ([`print_from`]). The desktop
//! names the operating system's faces, as the screen's worker does, and
//! orders the CJK faces as the screen does ([`set_print_order`]). A build
//! that names nothing prints such a character as the replacement square on
//! every machine.
//!
//! A face joins print the first time a cluster needs it, from the same copy
//! of its file the screen reads. A face whose licence forbids embedding it in
//! a document (OS/2 `fsType`) is passed over, so no export carries a face it
//! may not, and every format prints the square there alike.

use std::collections::{BTreeMap, HashMap, HashSet};
use std::path::PathBuf;
use std::sync::{Mutex, PoisonError, RwLock};

use skrifa::raw::TableProvider;
use skrifa::{FontRef, MetadataProvider};

use super::{Catalog, KnownFace, Source, SystemFace, known, precedence};
use crate::HanVariant;

/// A system face print letters in, by its place among the faces that have
/// joined print in this process.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct SystemFaceId(u32);

impl SystemFaceId {
    /// The face, read and parsed.
    #[must_use]
    pub fn face(self) -> &'static SystemFace {
        FACES.read().unwrap_or_else(PoisonError::into_inner)[self.0 as usize]
    }
}

/// Every face that has joined print, which a [`SystemFaceId`] indexes. A
/// face never leaves, so an id stays good for the life of the process.
static FACES: RwLock<Vec<&'static SystemFace>> = RwLock::new(Vec::new());

/// Where print looks, and what it has found there.
static LOOKUP: Mutex<Option<Lookup>> = Mutex::new(None);

/// Makes print letter what no embedded face has from `source`'s faces, with
/// the CJK faces in `order`. Replaces what print was told before.
pub fn print_from(source: Source, order: [HanVariant; 5]) {
    let lookup = Lookup {
        scan: source.known.is_none(),
        known: source.known,
        order,
        catalogs: None,
        chosen: HashMap::new(),
    };
    *LOOKUP.lock().unwrap_or_else(PoisonError::into_inner) = Some(lookup);
}

/// Makes print letter from no installed face.
pub fn print_from_none() {
    *LOOKUP.lock().unwrap_or_else(PoisonError::into_inner) = None;
}

/// Orders print's CJK faces as `order` says, from the next cluster on.
pub fn set_print_order(order: [HanVariant; 5]) {
    if let Some(lookup) = LOOKUP
        .lock()
        .unwrap_or_else(PoisonError::into_inner)
        .as_mut()
        && lookup.order != order
    {
        lookup.order = order;
        lookup.chosen.clear();
    }
}

/// The installed face print letters `cluster` in, when no embedded face has
/// all of it: the first face, in the screen's order, that has every
/// character of it that draws, with `true`; or else the first that has its
/// first character, with `false`. `None` when print looks nowhere or no
/// face it may embed has the cluster.
pub(crate) fn face_for(cluster: &str, drawn: &[char]) -> Option<(SystemFaceId, bool)> {
    let mut lookup = LOOKUP.lock().unwrap_or_else(PoisonError::into_inner);
    let lookup = lookup.as_mut()?;
    if let Some(chosen) = lookup.chosen.get(cluster) {
        return *chosen;
    }
    let first = cluster.chars().next()?;
    let wanted: Vec<char> = if drawn.is_empty() {
        vec![first]
    } else {
        drawn.to_vec()
    };
    let chosen = lookup.choose(&wanted, first);
    lookup.chosen.insert(cluster.to_owned(), chosen);
    chosen
}

struct Lookup {
    known: Option<Vec<KnownFace>>,
    /// Whether a character the named faces lack is looked for in every
    /// installed face.
    scan: bool,
    order: [HanVariant; 5],
    catalogs: Option<Catalogs>,
    /// The choice for each cluster asked about, until the order changes.
    chosen: HashMap<String, Option<(SystemFaceId, bool)>>,
}

struct Catalogs {
    known: Catalog,
    /// The characters each named face has, read the first time the face is
    /// asked about, by its place among the catalog's candidates.
    coverage: HashMap<usize, HashSet<u32>>,
    found: Option<Catalog>,
    /// The faces that have joined print, by file and index.
    joined: BTreeMap<(PathBuf, u32), SystemFaceId>,
}

impl Lookup {
    fn choose(&mut self, wanted: &[char], first: char) -> Option<(SystemFaceId, bool)> {
        let order = self.order;
        let scan = self.scan;
        let catalogs = self.catalogs.get_or_insert_with(|| Catalogs {
            known: Catalog::probe(&self.known.clone().unwrap_or_else(known::faces)),
            coverage: HashMap::new(),
            found: None,
            joined: BTreeMap::new(),
        });
        let mut ranked: Vec<usize> = (0..catalogs.known.candidates.len()).collect();
        ranked.sort_by_key(|&at| {
            let candidate = &catalogs.known.candidates[at];
            precedence(candidate.class, candidate.position, &order)
        });
        let mut partial = None;
        for at in ranked {
            if catalogs.known.candidates[at].refused {
                continue;
            }
            let Catalogs {
                known, coverage, ..
            } = &mut *catalogs;
            let has = coverage.entry(at).or_insert_with(|| {
                known
                    .database
                    .with_face_data(known.candidates[at].id, |data, index| {
                        FontRef::from_index(data, index)
                            .map(|font| {
                                font.charmap()
                                    .mappings()
                                    .map(|(character, _)| character)
                                    .collect()
                            })
                            .unwrap_or_default()
                    })
                    .unwrap_or_default()
            });
            let whole = wanted
                .iter()
                .all(|&character| has.contains(&u32::from(character)));
            if whole {
                if let Some(id) = catalogs.join(false, at) {
                    return Some((id, true));
                }
            } else if partial.is_none() && has.contains(&u32::from(first)) {
                partial = Some((false, at));
            }
        }
        if scan && partial.is_none() {
            let Catalogs { known, found, .. } = &mut *catalogs;
            let count = found
                .get_or_insert_with(|| Catalog::scan(known))
                .candidates
                .len();
            for at in 0..count {
                let found = catalogs.found.as_ref()?;
                if found.candidates[at].refused {
                    continue;
                }
                let covered = found
                    .database
                    .with_face_data(found.candidates[at].id, |data, index| {
                        FontRef::from_index(data, index).map_or((false, false), |font| {
                            let charmap = font.charmap();
                            (
                                wanted
                                    .iter()
                                    .all(|&character| charmap.map(character).is_some()),
                                charmap.map(first).is_some(),
                            )
                        })
                    })
                    .unwrap_or_default();
                match covered {
                    (true, _) => {
                        if let Some(id) = catalogs.join(true, at) {
                            return Some((id, true));
                        }
                    }
                    (false, true) if partial.is_none() => partial = Some((true, at)),
                    _ => {}
                }
            }
        }
        let (found, at) = partial?;
        catalogs.join(found, at).map(|id| (id, false))
    }
}

impl Catalogs {
    /// The id of the candidate at `at`, in the scan's catalog when `found`
    /// says so, joining it to print the first time. A face that does not
    /// load, or may not be embedded, is refused and never offered again.
    fn join(&mut self, found: bool, at: usize) -> Option<SystemFaceId> {
        let catalog = if found {
            self.found.as_mut()?
        } else {
            &mut self.known
        };
        let candidate = &catalog.candidates[at];
        let identity = (candidate.path.clone(), candidate.index);
        if let Some(id) = self.joined.get(&identity) {
            return Some(*id);
        }
        let Some(face) = SystemFace::load(candidate, &[]).filter(may_embed) else {
            catalog.candidates[at].refused = true;
            return None;
        };
        let mut faces = FACES.write().unwrap_or_else(PoisonError::into_inner);
        let id = SystemFaceId(u32::try_from(faces.len()).ok()?);
        faces.push(Box::leak(Box::new(face)));
        self.joined.insert(identity, id);
        Some(id)
    }
}

/// Whether `face`'s licence lets a document carry it: the OS/2 table's
/// `fsType` neither restricts embedding to the licensee, nor forbids
/// subsetting, nor allows only the bitmaps.
fn may_embed(face: &SystemFace) -> bool {
    FontRef::from_index(face.bytes(), face.index())
        .ok()
        .and_then(|font| font.os2().ok())
        .is_none_or(|os2| {
            let permissions = os2.fs_type();
            permissions & 0x000F != 0x0002 && permissions & 0x0300 == 0
        })
}

#[cfg(test)]
mod tests;
