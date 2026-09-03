//! Scaffolding shared by the CLI integration-test binaries.
//!
//! Every test here drives the packaged `rspice` binary and reads what it
//! wrote, so each one needs an isolated directory, the fixture deck root, and
//! a JSON reader. Those had been copied into each test file, which is how
//! several copies of "isolated" came to mean different things: some removed
//! the directory when the test ended and some leaked it into the temporary
//! directory, and several composed a name with no per-call serial, so two
//! tests using one tag in one binary shared a directory and raced.
//!
//! Each test binary compiles this module and uses the part it needs, so an
//! item unused by one binary is not unused by the suite. That is what the
//! `dead_code` allowances below say; there is no crate- or module-scope
//! allowance.

use std::path::{Path, PathBuf};
use std::sync::atomic::{AtomicU64, Ordering};

/// A directory that exists for one test and is removed when it ends.
pub struct TestDirectory(PathBuf);

impl TestDirectory {
    // Callers use `test_dir`; the constructor is private to this module.
    fn new(tag: &str) -> Self {
        static NEXT: AtomicU64 = AtomicU64::new(0);
        let serial = NEXT.fetch_add(1, Ordering::Relaxed);
        // `CARGO_CRATE_NAME` is this test binary's own name, so two binaries
        // running the same tag concurrently cannot collide either.
        let path = std::env::temp_dir().join(format!(
            "rspice_{}_{}_{tag}_{serial}",
            env!("CARGO_CRATE_NAME"),
            std::process::id()
        ));
        std::fs::create_dir_all(&path)
            .unwrap_or_else(|error| panic!("create test directory {}: {error}", path.display()));
        Self(path)
    }

    // Callers that hold the directory by value use `Deref` instead.
    #[allow(dead_code)]
    pub fn path(&self) -> &Path {
        &self.0
    }

    // Callers that build one path use `Deref` to `Path::join` instead.
    #[allow(dead_code)]
    pub fn join(&self, path: impl AsRef<Path>) -> PathBuf {
        self.0.join(path)
    }
}

impl std::ops::Deref for TestDirectory {
    type Target = Path;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl AsRef<Path> for TestDirectory {
    fn as_ref(&self) -> &Path {
        &self.0
    }
}

impl Drop for TestDirectory {
    fn drop(&mut self) {
        let _ = std::fs::remove_dir_all(&self.0);
    }
}

/// An isolated directory for one test, named after this binary and `tag`.
///
/// The name carries the test binary, the process id, the caller's tag, and a
/// process-wide serial, so two tests in one binary, two binaries in one suite
/// run, and two suite runs on one machine never share a directory.
// Not every test binary creates directories.
#[allow(dead_code)]
pub fn test_dir(tag: &str) -> TestDirectory {
    TestDirectory::new(tag)
}

/// A checked-in regression deck, by file name.
// Not every test binary reads a checked-in deck.
#[allow(dead_code)]
pub fn fixture(name: &str) -> PathBuf {
    Path::new(env!("CARGO_MANIFEST_DIR"))
        .join("tests")
        .join("fixtures")
        .join("audit_regressions")
        .join(name)
}

/// One JSON artifact the CLI published, parsed.
// Not every test binary reads a JSON artifact.
#[allow(dead_code)]
pub fn read_json(path: &Path) -> serde_json::Value {
    let bytes =
        std::fs::read(path).unwrap_or_else(|error| panic!("read {}: {error}", path.display()));
    serde_json::from_slice(&bytes)
        .unwrap_or_else(|error| panic!("parse {}: {error}", path.display()))
}

/// Where an axis run commits the manifest that names its complete coordinate
/// set, and where it publishes the coordinate schema union.
///
/// These two are the only artifact names a test composes for itself, because
/// they are how it finds what the run published. Everything else a coordinate
/// published is read out of the run-set manifest rather than derived a second
/// time from the plan and the coordinate-tag sanitizer, which is how a test's
/// idea of a file name used to drift from the production one.
// Not every test binary runs an axis deck.
#[allow(dead_code)]
pub fn axis_run_set_path(requested: &Path) -> PathBuf {
    sibling_json(requested, "run_set")
}

// Not every test binary runs an axis deck.
#[allow(dead_code)]
pub fn axis_schema_manifest_path(requested: &Path) -> PathBuf {
    sibling_json(requested, "step_schema")
}

fn sibling_json(requested: &Path, tag: &str) -> PathBuf {
    let stem = requested
        .file_stem()
        .expect("output stem")
        .to_string_lossy();
    requested.with_file_name(format!("{stem}.{tag}.json"))
}

/// One coordinate of an axis run, exactly as the run-set manifest names it.
#[derive(Debug)]
pub struct AxisCoordinate {
    // Read by the tests that assert on coordinate identity, not by all of
    // them; the manifest declares all four and this type carries all four.
    #[allow(dead_code)]
    pub coordinate_id: String,
    pub ordinal: usize,
    pub tag: String,
    #[allow(dead_code)]
    pub assignment: String,
    /// Artifacts this coordinate published, in publication order.
    pub artifacts: Vec<PathBuf>,
}

impl AxisCoordinate {
    /// The single artifact this coordinate published.
    // Not every test binary runs a one-analysis axis deck.
    #[allow(dead_code)]
    pub fn only_artifact(&self) -> &Path {
        match self.artifacts.as_slice() {
            [artifact] => artifact,
            other => panic!(
                "coordinate {} published {} artifacts, not one: {other:?}",
                self.tag,
                other.len()
            ),
        }
    }

    /// The artifact this coordinate published for one analysis identity.
    // Not every test binary runs a multi-analysis axis deck.
    #[allow(dead_code)]
    pub fn artifact(&self, analysis_tag: &str) -> &Path {
        self.artifacts
            .iter()
            .find(|artifact| {
                artifact
                    .file_name()
                    .and_then(|name| name.to_str())
                    .is_some_and(|name| name.contains(&format!(".{analysis_tag}.")))
            })
            .unwrap_or_else(|| {
                panic!(
                    "coordinate {} published no {analysis_tag} artifact: {:?}",
                    self.tag, self.artifacts
                )
            })
    }
}

/// The transaction manifest an axis run commits after every coordinate
/// artifact: a reader that finds it knows every artifact it names is present
/// and complete.
#[derive(Debug)]
pub struct AxisRunSet {
    // Read by the tests that assert which axes a deck declared.
    #[allow(dead_code)]
    pub axes: Vec<String>,
    pub coordinates: Vec<AxisCoordinate>,
}

impl AxisRunSet {
    /// Read the manifest an axis run published beside `requested`.
    // Not every test binary runs an axis deck.
    #[allow(dead_code)]
    pub fn read(requested: &Path) -> Self {
        Self::try_read(requested).unwrap_or_else(|| {
            panic!(
                "no run-set manifest at {}",
                axis_run_set_path(requested).display()
            )
        })
    }

    /// The manifest, when the run committed one. A failed or cancelled axis
    /// run commits nothing, so its absence is the evidence that no coordinate
    /// set was published.
    // Not every test binary runs an axis deck.
    #[allow(dead_code)]
    pub fn try_read(requested: &Path) -> Option<Self> {
        let path = axis_run_set_path(requested);
        if !path.exists() {
            return None;
        }
        let document = read_json(&path);
        assert_eq!(
            document["kind"],
            "axis_coordinate_set",
            "{}",
            path.display()
        );
        let directory = path.parent().expect("manifest parent").to_path_buf();
        let coordinates = document["coordinates"]
            .as_array()
            .expect("manifest coordinates")
            .iter()
            .map(|coordinate| AxisCoordinate {
                coordinate_id: coordinate["coordinate_id"]
                    .as_str()
                    .expect("coordinate id")
                    .to_owned(),
                ordinal: usize::try_from(coordinate["ordinal"].as_u64().expect("ordinal"))
                    .expect("ordinal fits"),
                tag: coordinate["tag"]
                    .as_str()
                    .expect("coordinate tag")
                    .to_owned(),
                assignment: coordinate["assignment"]
                    .as_str()
                    .expect("coordinate assignment")
                    .to_owned(),
                artifacts: coordinate["artifacts"]
                    .as_array()
                    .expect("coordinate artifacts")
                    .iter()
                    .map(|name| directory.join(name.as_str().expect("artifact name")))
                    .collect(),
            })
            .collect::<Vec<_>>();
        assert_eq!(
            coordinates.len(),
            usize::try_from(
                document["coordinate_count"]
                    .as_u64()
                    .expect("coordinate count")
            )
            .expect("coordinate count fits"),
            "the manifest's coordinate count disagrees with its own list"
        );
        Some(Self {
            axes: document["axes"]
                .as_array()
                .expect("manifest axes")
                .iter()
                .map(|axis| axis.as_str().expect("axis kind").to_owned())
                .collect(),
            coordinates,
        })
    }

    /// One coordinate by its one-based ordinal.
    // Not every test binary indexes coordinates.
    #[allow(dead_code)]
    pub fn coordinate(&self, one_based: usize) -> &AxisCoordinate {
        let coordinate = self
            .coordinates
            .get(one_based.checked_sub(1).expect("one-based ordinal"))
            .unwrap_or_else(|| {
                panic!(
                    "the run set has {} coordinates, not {one_based}",
                    self.coordinates.len()
                )
            });
        assert_eq!(coordinate.ordinal, one_based, "manifest ordinals are dense");
        coordinate
    }

    /// Every artifact the set published, in coordinate then publication order.
    // Not every test binary enumerates artifacts.
    #[allow(dead_code)]
    pub fn artifacts(&self) -> Vec<PathBuf> {
        self.coordinates
            .iter()
            .flat_map(|coordinate| coordinate.artifacts.iter().cloned())
            .collect()
    }

    /// Everything a completed axis run leaves in its output directory: every
    /// coordinate artifact, the coordinate schema union, and this manifest.
    // Not every test binary compares directory contents.
    #[allow(dead_code)]
    pub fn published_files(&self, requested: &Path) -> Vec<PathBuf> {
        let mut files = self.artifacts();
        files.push(axis_schema_manifest_path(requested));
        files.push(axis_run_set_path(requested));
        files.sort();
        files
    }
}
