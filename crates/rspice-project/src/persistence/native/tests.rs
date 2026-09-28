//! Native save rules exercised through an in-memory compare-and-swap adapter.

use super::*;
use crate::persistence::digest_bytes;
use crate::{ProjectLibraries, ProjectWorkspace};
use std::cell::{Cell, RefCell};

#[derive(Default)]
struct Storage {
    bytes: RefCell<Option<Vec<u8>>>,
    observations: Cell<usize>,
    publications: Cell<usize>,
    alias: Cell<bool>,
}

impl NativeProjectStorage for Storage {
    type ExpectedContent = Option<ContentDigest>;

    fn normalize_path(&self, path: &Path) -> Result<PathBuf, PersistenceError> {
        Ok(path.to_path_buf())
    }
    fn same_file(&self, _: &Path, _: &Path) -> Result<bool, PersistenceError> {
        Ok(self.alias.get())
    }
    fn observe_destination(&self, _: &Path) -> Result<Self::ExpectedContent, PersistenceError> {
        self.observations.set(self.observations.get() + 1);
        Ok(self.bytes.borrow().as_deref().map(digest_bytes))
    }
    fn accepted_content(&self, digest: ContentDigest) -> Self::ExpectedContent {
        Some(digest)
    }
    fn publish(
        &self,
        _: &Path,
        expected: Self::ExpectedContent,
        bytes: &[u8],
    ) -> Result<ContentDigest, PersistenceError> {
        self.publications.set(self.publications.get() + 1);
        let mut current = self.bytes.borrow_mut();
        if current.as_deref().map(digest_bytes) != expected {
            return Err(PersistenceError::ExternalChange);
        }
        *current = Some(bytes.to_vec());
        Ok(digest_bytes(bytes))
    }
}

fn project() -> ProjectFile {
    use rspice_design::library::{Cell, Library, View, ViewType};
    let workspace = ProjectWorkspace::default();
    let mut cell = Cell::new(workspace.project.top_cell.clone());
    cell.add_view(View::new(
        workspace.active_view.view.clone(),
        ViewType::Schematic,
    ));
    let mut library = Library::new(workspace.project.root_library.clone());
    library.add_cell(cell);
    let mut libraries = ProjectLibraries::default();
    libraries.add_library(library);
    ProjectFile::new(workspace, libraries)
}

fn selected(storage: &Storage) -> NativeSaveDestination<Option<ContentDigest>> {
    NativeSaveDestination::new(
        storage,
        Path::new("saved.rspiceproj"),
        DestinationAuthority::UserSelected,
        None,
        None,
    )
    .unwrap()
}

#[test]
fn canonical_save_requires_readable_path_authority_and_retains_accepted_bytes() {
    let storage = Storage::default();
    let path = Path::new("canonical.rspiceproj");
    let binding = NativeBinding {
        canonical_path: path.to_path_buf(),
        accepted_digest: digest_bytes(b"accepted"),
    };
    let unreadable = UnreadableNativeBinding {
        canonical_path: path.to_path_buf(),
        reason: "restart validation failed".to_owned(),
    };
    for authority in [
        DestinationAuthority::Canonical,
        DestinationAuthority::UserSelected,
    ] {
        assert!(
            matches!(NativeSaveDestination::new(&storage, path, authority, Some(&binding), Some(&unreadable)),
            Err(ProjectLifecycleError::UnreadableCanonical(reason)) if reason == unreadable.reason)
        );
    }
    for (path, baseline) in [
        (path, None),
        (Path::new("other.rspiceproj"), Some(&binding)),
    ] {
        assert!(matches!(
            NativeSaveDestination::new(
                &storage,
                path,
                DestinationAuthority::Canonical,
                baseline,
                None
            ),
            Err(ProjectLifecycleError::UnreadableCanonical(_))
        ));
    }
    *storage.bytes.borrow_mut() = Some(b"external edit".to_vec());
    let destination = NativeSaveDestination::new(
        &storage,
        path,
        DestinationAuthority::Canonical,
        Some(&binding),
        None,
    )
    .unwrap();
    assert!(matches!(
        destination.publish(&storage, project(), |_| Ok(DocumentRegistry::default())),
        Err(ProjectLifecycleError::Persistence(
            PersistenceError::ExternalChange
        ))
    ));
    assert_eq!(storage.observations.get(), 0);
    assert_eq!(
        storage.bytes.borrow().as_deref(),
        Some(b"external edit".as_slice())
    );
}

#[test]
fn preparation_and_serialization_finish_before_native_publication() {
    let storage = Storage::default();
    assert!(matches!(
        selected(&storage).publish(&storage, project(), |_| {
            Err(ProjectLifecycleError::InvalidState(
                "invalid draft".to_owned(),
            ))
        }),
        Err(ProjectLifecycleError::InvalidState(_))
    ));
    assert_eq!(storage.publications.get(), 0);
    let compared = Cell::new(false);
    let invalid = ProjectFile::new(ProjectWorkspace::default(), ProjectLibraries::default());
    assert!(
        selected(&storage)
            .publish(&storage, invalid, |_| {
                compared.set(true);
                Ok(DocumentRegistry::default())
            })
            .is_err()
    );
    assert!(compared.get());
    assert_eq!(storage.publications.get(), 0);
    assert!(storage.bytes.borrow().is_none());

    let file = project();
    let identity = file.workspace.project.id();
    let published = selected(&storage)
        .publish(&storage, file, |candidate| {
            assert_eq!(
                candidate.workspace.project.path.as_deref(),
                Some(Path::new("saved.rspiceproj"))
            );
            Ok(DocumentRegistry::default())
        })
        .unwrap();
    assert_eq!(published.candidate.workspace.project.id(), identity);
    assert_eq!(
        published.binding.canonical_path,
        Path::new("saved.rspiceproj")
    );
    let bytes = storage.bytes.borrow();
    assert_eq!(
        published.binding.accepted_digest,
        digest_bytes(bytes.as_deref().unwrap())
    );
    assert_eq!(
        serialized_project(&published.candidate)
            .unwrap()
            .0
            .as_slice(),
        bytes.as_deref().unwrap()
    );
}

#[test]
fn selected_destination_retains_its_expectation_through_preparation() {
    for previous in [None, Some(b"picker-time bytes".to_vec())] {
        let storage = Storage {
            bytes: RefCell::new(previous),
            ..Storage::default()
        };
        let destination = selected(&storage);
        let result = destination.publish(&storage, project(), |_| {
            *storage.bytes.borrow_mut() = Some(b"late change".to_vec());
            Ok(DocumentRegistry::default())
        });
        assert!(matches!(
            result,
            Err(ProjectLifecycleError::Persistence(
                PersistenceError::ExternalChange
            ))
        ));
        assert_eq!(storage.observations.get(), 1);
        assert_eq!(
            storage.bytes.borrow().as_deref(),
            Some(b"late change".as_slice())
        );
    }
}

#[test]
fn independent_copy_rejects_canonical_paths_and_aliases_and_forks_identity() {
    let storage = Storage::default();
    let source = Path::new("source.rspiceproj");
    let unreadable = UnreadableNativeBinding {
        canonical_path: source.to_path_buf(),
        reason: "unreadable".to_owned(),
    };
    for (canonical, rejected) in [(Some(source), None), (None, Some(&unreadable))] {
        assert!(matches!(
            NativeCopyDestination::new(&storage, source, canonical, rejected),
            Err(ProjectLifecycleError::CopyDestinationIsCanonical)
        ));
        storage.alias.set(true);
        assert!(matches!(
            NativeCopyDestination::new(
                &storage,
                Path::new("alias.rspiceproj"),
                canonical,
                rejected
            ),
            Err(ProjectLifecycleError::CopyDestinationIsCanonical)
        ));
        storage.alias.set(false);
    }
    assert_eq!(storage.observations.get(), 0);
    let destination =
        NativeCopyDestination::new(&storage, Path::new("copy.rspiceproj"), Some(source), None)
            .unwrap();
    let file = project();
    let source_identity = file.workspace.project.id();
    let source_name = file.workspace.project.name.clone();
    destination.publish(&storage, file).unwrap();
    let copy: ProjectFile =
        serde_json::from_slice(storage.bytes.borrow().as_deref().unwrap()).unwrap();
    assert_ne!(copy.workspace.project.id(), source_identity);
    assert_eq!(copy.workspace.project.name, source_name);
    assert_eq!(storage.observations.get(), 1);
}
