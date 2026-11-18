//! Native save rules exercised through an in-memory compare-and-swap adapter.

use super::*;
use crate::persistence::{digest_bytes, serialized_project};
use crate::{ProjectLibraries, ProjectWorkspace};
use std::cell::{Cell, RefCell};

#[derive(Default)]
struct Storage {
    bytes: RefCell<Option<Vec<u8>>>,
    observations: Cell<usize>,
    publications: Cell<usize>,
    alias: Cell<bool>,
    steps: RefCell<Vec<&'static str>>,
    recovered_bytes: RefCell<Option<Vec<u8>>>,
    recovery_error: RefCell<Option<PersistenceError>>,
    observation_error: RefCell<Option<PersistenceError>>,
    publication_error: RefCell<Option<PersistenceError>>,
    publish_before_error: bool,
}

impl NativeProjectStorage for Storage {
    type ExpectedContent = Option<ContentDigest>;

    fn normalize_path(&self, path: &Path) -> Result<PathBuf, PersistenceError> {
        self.steps.borrow_mut().push("normalize");
        Ok(path.to_path_buf())
    }
    fn reconcile_publication(&self, _: &Path) -> Result<(), PersistenceError> {
        self.steps.borrow_mut().push("reconcile");
        if let Some(error) = self.recovery_error.borrow_mut().take() {
            return Err(error);
        }
        if let Some(recovered) = self.recovered_bytes.borrow_mut().take() {
            *self.bytes.borrow_mut() = Some(recovered);
        }
        Ok(())
    }
    fn read_project(&self, path: &Path) -> Result<ProjectRead, ProjectIoError> {
        self.steps.borrow_mut().push("read");
        let current = self.bytes.borrow();
        let bytes = current
            .as_deref()
            .ok_or_else(|| ProjectIoError::NotFound(path.to_path_buf()))?;
        ProjectRead::read(bytes, Some(path))
    }
    fn same_file(&self, _: &Path, _: &Path) -> Result<bool, PersistenceError> {
        Ok(self.alias.get())
    }
    fn observe_destination(&self, _: &Path) -> Result<Self::ExpectedContent, PersistenceError> {
        self.observations.set(self.observations.get() + 1);
        if let Some(error) = self.observation_error.borrow_mut().take() {
            return Err(error);
        }
        Ok(self.bytes.borrow().as_deref().map(digest_bytes))
    }
    fn accepted_content(&self, digest: ContentDigest) -> Self::ExpectedContent {
        Some(digest)
    }
    fn publish_with(
        &self,
        _: &Path,
        expected: Self::ExpectedContent,
        write: &dyn Fn(&mut dyn std::io::Write) -> std::io::Result<()>,
    ) -> Result<ContentDigest, PersistenceError> {
        let mut bytes = Vec::new();
        write(&mut bytes).map_err(PersistenceError::Io)?;
        let bytes = bytes.as_slice();
        self.publications.set(self.publications.get() + 1);
        if let Some(error) = self.publication_error.borrow_mut().take() {
            if self.publish_before_error {
                *self.bytes.borrow_mut() = Some(bytes.to_vec());
            }
            return Err(error);
        }
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
    let source_name = file.workspace.project.name().to_owned();
    destination.publish(&storage, file).unwrap();
    let copy: ProjectFile =
        serde_json::from_slice(storage.bytes.borrow().as_deref().unwrap()).unwrap();
    assert_ne!(copy.workspace.project.id(), source_identity);
    assert_eq!(copy.workspace.project.name(), source_name);
    assert_eq!(storage.observations.get(), 1);
}

struct NoSourceFiles;

impl HierarchySourceFiles for NoSourceFiles {
    fn source_paths_match(&self, _: &Path, _: &Path) -> bool {
        panic!("empty schematic project must not access source files")
    }
    fn configured_source_identity(&self, _: &Path) -> String {
        panic!("empty schematic project must not access source files")
    }
    fn validate_source_file(
        &self,
        _: &Path,
        _: rspice_design::library::ViewType,
        _: &rspice_design::schematic::component::LibraryCellInstance,
    ) -> Result<(), String> {
        panic!("empty schematic project must not access source files")
    }
}

#[test]
fn native_open_accepts_only_the_snapshot_read_after_recovery() {
    let path = Path::new("canonical.rspiceproj");
    let file = project();
    let project_id = file.workspace.project.id();
    let (bytes, digest) = serialized_project(&file).unwrap();
    let storage = Storage {
        bytes: RefCell::new(Some(b"unreconciled bytes".to_vec())),
        recovered_bytes: RefCell::new(Some(bytes.clone())),
        ..Storage::default()
    };
    let (decoded, binding) = NativeBinding::open(&storage, path, NoSourceFiles).unwrap();
    assert_eq!(decoded.file.workspace.project.id(), project_id);
    assert_eq!(decoded.file.workspace.project.path.as_deref(), Some(path));
    assert_eq!(binding.canonical_path, path);
    assert_eq!(binding.accepted_digest, digest);
    assert_eq!(storage.steps.take(), ["normalize", "reconcile", "read"]);
    assert_eq!(storage.bytes.borrow().as_deref(), Some(bytes.as_slice()));
    assert_eq!(storage.publications.get(), 0);

    *storage.bytes.borrow_mut() = None;
    assert!(matches!(NativeBinding::open(&storage, path, NoSourceFiles),
        Err(PersistenceError::Project(ProjectIoError::NotFound(missing))) if missing == path));
    assert_eq!(storage.steps.take(), ["normalize", "reconcile", "read"]);
}

#[test]
fn a_folder_that_cannot_be_written_is_read_as_it_stands() {
    let path = Path::new("canonical.rspiceproj");
    let file = project();
    let (bytes, digest) = serialized_project(&file).unwrap();
    let storage = Storage {
        bytes: RefCell::new(Some(bytes)),
        recovery_error: RefCell::new(Some(PersistenceError::NotWritable(std::io::Error::from(
            std::io::ErrorKind::PermissionDenied,
        )))),
        ..Storage::default()
    };
    let (decoded, binding) = NativeBinding::open(&storage, path, NoSourceFiles)
        .expect("a project in a folder that cannot be written still opens");
    assert_eq!(
        decoded.file.workspace.project.id(),
        file.workspace.project.id()
    );
    assert_eq!(binding.accepted_digest, digest);
    assert_eq!(storage.steps.take(), ["normalize", "reconcile", "read"]);

    // Any other failure to reconcile still stops the open before the read.
    *storage.recovery_error.borrow_mut() = Some(PersistenceError::LeaseBusy(path.to_path_buf()));
    assert!(matches!(
        NativeBinding::open(&storage, path, NoSourceFiles),
        Err(PersistenceError::LeaseBusy(_))
    ));
    assert_eq!(storage.steps.take(), ["normalize", "reconcile"]);
}

#[test]
fn native_restore_checks_receipt_recovery_bytes_and_decoded_identity_in_order() {
    let path = Path::new("canonical.rspiceproj");
    let file = project();
    let project_id = file.workspace.project.id().to_string();
    let (bytes, digest) = serialized_project(&file).unwrap();
    let receipt = NativeBindingReceipt {
        canonical_path: path.to_path_buf(),
        project_id: project_id.clone(),
        accepted_digest: digest,
    };
    let storage = Storage {
        bytes: RefCell::new(Some(bytes.clone())),
        ..Storage::default()
    };
    assert!(matches!(
        NativeBinding::restore(&storage, path, "different session", &receipt, NoSourceFiles),
        Err(PersistenceError::NativeReceiptMismatch(_))
    ));
    assert!(storage.steps.take().is_empty());
    assert!(matches!(
        NativeBinding::restore(
            &storage,
            Path::new("other.rspiceproj"),
            &project_id,
            &receipt,
            NoSourceFiles
        ),
        Err(PersistenceError::NativeReceiptMismatch(_))
    ));
    assert_eq!(storage.steps.take(), ["normalize"]);

    *storage.recovery_error.borrow_mut() =
        Some(PersistenceError::Platform("recovery unresolved".to_owned()));
    assert!(
        matches!(NativeBinding::restore(&storage, path, &project_id, &receipt, NoSourceFiles),
        Err(PersistenceError::Platform(message)) if message == "recovery unresolved")
    );
    assert_eq!(storage.steps.take(), ["normalize", "reconcile"]);
    storage.recovery_error.borrow_mut().take();

    for replacement in [vec![0xff], b"not JSON".to_vec()] {
        *storage.bytes.borrow_mut() = Some(replacement.clone());
        assert!(matches!(
            NativeBinding::restore(&storage, path, &project_id, &receipt, NoSourceFiles),
            Err(PersistenceError::ExternalChange)
        ));
        assert_eq!(storage.steps.take(), ["normalize", "reconcile", "read"]);
        let matching = NativeBindingReceipt {
            accepted_digest: digest_bytes(&replacement),
            ..receipt.clone()
        };
        assert!(matches!(
            NativeBinding::restore(&storage, path, &project_id, &matching, NoSourceFiles),
            Err(PersistenceError::Project(ProjectIoError::ParseError { .. }))
        ));
        assert_eq!(storage.steps.take(), ["normalize", "reconcile", "read"]);
    }

    let (other_bytes, other_digest) = serialized_project(&project()).unwrap();
    *storage.bytes.borrow_mut() = Some(other_bytes);
    let mismatched_identity = NativeBindingReceipt {
        accepted_digest: other_digest,
        ..receipt.clone()
    };
    assert!(
        matches!(NativeBinding::restore(&storage, path, &project_id, &mismatched_identity, NoSourceFiles),
        Err(PersistenceError::NativeReceiptMismatch(message)) if message.contains("project file identity"))
    );
    assert_eq!(storage.steps.take(), ["normalize", "reconcile", "read"]);

    *storage.bytes.borrow_mut() = Some(bytes.clone());
    let (decoded, binding) =
        NativeBinding::restore(&storage, path, &project_id, &receipt, NoSourceFiles).unwrap();
    assert_eq!(decoded.file.workspace.project.id().to_string(), project_id);
    assert_eq!(binding.native_receipt(&project_id), receipt);
    assert_eq!(storage.steps.take(), ["normalize", "reconcile", "read"]);
    assert_eq!(storage.bytes.borrow().as_deref(), Some(bytes.as_slice()));
    assert_eq!(storage.publications.get(), 0);
}

fn uncertain() -> PersistenceError {
    PersistenceError::PublicationUncertain {
        message: "unresolved publication".to_owned(),
        recovery_paths: vec![
            PathBuf::from("recovery slot.a"),
            PathBuf::from("predecessor.bak"),
        ],
    }
}

fn lease_busy() -> PersistenceError {
    PersistenceError::LeaseBusy(PathBuf::from("owned.lock"))
}

fn os_failure() -> PersistenceError {
    PersistenceError::Io(std::io::Error::from_raw_os_error(5))
}

fn assert_storage_error(actual: PersistenceError, expected: PersistenceError) {
    assert_eq!(actual.to_string(), expected.to_string());
    match (actual, expected) {
        (
            PersistenceError::PublicationUncertain {
                message,
                recovery_paths,
            },
            PersistenceError::PublicationUncertain {
                message: expected,
                recovery_paths: paths,
            },
        ) => {
            assert_eq!(message, expected);
            assert_eq!(recovery_paths, paths);
        }
        (PersistenceError::LeaseBusy(path), PersistenceError::LeaseBusy(expected)) => {
            assert_eq!(path, expected)
        }
        (PersistenceError::Io(error), PersistenceError::Io(expected)) => {
            assert_eq!(error.kind(), expected.kind());
            assert_eq!(error.raw_os_error(), expected.raw_os_error());
        }
        (actual, expected) => panic!("storage failure changed: {actual:?}, expected {expected:?}"),
    }
}

#[test]
fn native_recovery_and_observation_faults_precede_content_and_publication() {
    let path = Path::new("canonical.rspiceproj");
    let file = project();
    let project_id = file.workspace.project.id().to_string();
    let (bytes, accepted_digest) = serialized_project(&file).unwrap();
    let receipt = NativeBindingReceipt {
        canonical_path: path.to_path_buf(),
        project_id: project_id.clone(),
        accepted_digest,
    };
    for fault in [
        uncertain as fn() -> PersistenceError,
        lease_busy,
        os_failure,
    ] {
        let storage = Storage {
            bytes: RefCell::new(Some(bytes.clone())),
            recovery_error: RefCell::new(Some(fault())),
            ..Storage::default()
        };
        assert_storage_error(
            NativeBinding::open(&storage, path, NoSourceFiles)
                .err()
                .unwrap(),
            fault(),
        );
        assert_eq!(storage.steps.take(), ["normalize", "reconcile"]);
        *storage.recovery_error.borrow_mut() = Some(fault());
        assert_storage_error(
            NativeBinding::restore(&storage, path, &project_id, &receipt, NoSourceFiles)
                .err()
                .unwrap(),
            fault(),
        );
        assert_eq!(storage.steps.take(), ["normalize", "reconcile"]);
        assert_eq!(storage.bytes.borrow().as_deref(), Some(bytes.as_slice()));

        *storage.observation_error.borrow_mut() = Some(fault());
        let error = NativeSaveDestination::new(
            &storage,
            path,
            DestinationAuthority::UserSelected,
            None,
            None,
        )
        .err()
        .unwrap();
        let ProjectLifecycleError::Persistence(error) = error else {
            panic!("storage error was lost")
        };
        assert_storage_error(error, fault());
        *storage.observation_error.borrow_mut() = Some(fault());
        let error = NativeCopyDestination::new(&storage, path, None, None)
            .err()
            .unwrap();
        let ProjectLifecycleError::Persistence(error) = error else {
            panic!("storage error was lost")
        };
        assert_storage_error(error, fault());
        assert_eq!(storage.observations.get(), 2);
        assert_eq!(storage.publications.get(), 0);
    }
}

#[test]
fn uncertain_native_publication_never_returns_save_or_copy_acceptance() {
    for publish_before_error in [false, true] {
        for copy in [false, true] {
            let storage = Storage {
                bytes: RefCell::new(Some(b"accepted bytes".to_vec())),
                publication_error: RefCell::new(Some(uncertain())),
                publish_before_error,
                ..Storage::default()
            };
            let result = if copy {
                NativeCopyDestination::new(&storage, Path::new("copy.rspiceproj"), None, None)
                    .unwrap()
                    .publish(&storage, project())
            } else {
                selected(&storage)
                    .publish(&storage, project(), |_| Ok(DocumentRegistry::default()))
                    .map(|_| ())
            };
            let ProjectLifecycleError::Persistence(error) = result.unwrap_err() else {
                panic!("storage error was lost")
            };
            assert_storage_error(error, uncertain());
            assert_eq!(storage.publications.get(), 1);
            assert_eq!(storage.observations.get(), 1);
            if publish_before_error {
                assert_ne!(
                    storage.bytes.borrow().as_deref(),
                    Some(b"accepted bytes".as_slice())
                );
            } else {
                assert_eq!(
                    storage.bytes.borrow().as_deref(),
                    Some(b"accepted bytes".as_slice())
                );
            }
        }
    }
}

/// A reader that hands over at most two bytes a read, and fails once it has
/// been asked `fail_after` times.
struct ShortReader<'a> {
    bytes: &'a [u8],
    reads: usize,
    fail_after: Option<usize>,
}

impl std::io::Read for ShortReader<'_> {
    fn read(&mut self, buffer: &mut [u8]) -> std::io::Result<usize> {
        self.reads += 1;
        if self.fail_after.is_some_and(|limit| self.reads > limit) {
            return Err(std::io::Error::other("interrupted project read"));
        }
        let len = 2.min(self.bytes.len()).min(buffer.len());
        buffer[..len].copy_from_slice(&self.bytes[..len]);
        self.bytes = &self.bytes[len..];
        Ok(len)
    }
}

/// A file read in pieces is parsed and identified by exactly the bytes it
/// holds, however the reads fall.
#[test]
fn a_streamed_read_parses_and_identifies_exactly_the_bytes_read() {
    let text = crate::serialize_project_file(&project()).expect("the project serializes");
    let mut reader = ShortReader {
        bytes: text.as_bytes(),
        reads: 0,
        fail_after: None,
    };
    let read = ProjectRead::read(&mut reader, None).expect("the file is read");
    assert_eq!(read.digest(), digest_bytes(text.as_bytes()));
    assert!(reader.reads > 2);
    let decoded = read.decode(NoSourceFiles).expect("the project decodes");
    assert_eq!(
        crate::serialize_project_file(&decoded.file).expect("it serializes again"),
        text
    );
}

/// A file cut short still has a digest of what it holds, so a changed file
/// is told apart from a damaged one, and decoding says it is incomplete.
#[test]
fn a_truncated_file_is_read_to_its_end_and_decodes_as_incomplete() {
    let text = crate::serialize_project_file(&project()).expect("the project serializes");
    let cut = &text.as_bytes()[..text.len() / 2];
    let read = ProjectRead::read(cut, None).expect("a damaged file is still read");
    assert_eq!(read.digest(), digest_bytes(cut));
    let error = read
        .decode(NoSourceFiles)
        .expect_err("half a project does not decode");
    assert!(matches!(
        error,
        ProjectIoError::ParseError {
            damage: crate::FileDamage::Truncated,
            ..
        }
    ));
}

/// A read the disk interrupts is an I/O failure, never a partial project.
#[test]
fn an_interrupted_read_is_an_io_failure() {
    let text = crate::serialize_project_file(&project()).expect("the project serializes");
    let mut reader = ShortReader {
        bytes: text.as_bytes(),
        reads: 0,
        fail_after: Some(1),
    };
    assert!(matches!(
        ProjectRead::read(&mut reader, None),
        Err(ProjectIoError::Io(message)) if message.contains("interrupted project read")
    ));
}
