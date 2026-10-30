//! The faces a hardcopy's text was lettered in, as its receipt records them.
//!
//! Two prints of one plan differ if the faces behind their text differ: a
//! machine with another version of a system face letters other glyphs. The
//! receipt names every face by family, weight and version, binds its exact
//! bytes by SHA-256, and says where it came from, so such a difference is
//! explainable from the receipts alone.

use serde::{Deserialize, Serialize};

use super::{HardcopyError, MAX_TEXT_BYTES, validate_text};
use rspice_vocabulary::product::ContentDigest;

/// The most faces one receipt names.
const MAX_FACES: usize = 64;
const MAX_PATH_BYTES: usize = 1_024;

/// Where a face's bytes came from.
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "kebab-case", tag = "kind", deny_unknown_fields)]
pub enum HardcopyFaceSource {
    /// Embedded in RSpice, the same on every machine.
    Bundled,
    /// Installed on the machine that lettered the text, read from `path`.
    System { path: String },
}

/// One face a hardcopy's text was lettered in.
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct HardcopyFace {
    family: String,
    weight: u16,
    version: String,
    sha256: ContentDigest,
    source: HardcopyFaceSource,
}

impl HardcopyFace {
    pub fn try_new(
        family: impl Into<String>,
        weight: u16,
        version: impl Into<String>,
        sha256: ContentDigest,
        source: HardcopyFaceSource,
    ) -> Result<Self, HardcopyError> {
        let face = Self {
            family: family.into(),
            weight,
            version: version.into(),
            sha256,
            source,
        };
        face.validate()?;
        Ok(face)
    }

    pub(super) fn validate(&self) -> Result<(), HardcopyError> {
        validate_text("face family", &self.family, MAX_TEXT_BYTES)?;
        validate_text("face version", &self.version, MAX_TEXT_BYTES)?;
        if !(1..=1_000).contains(&self.weight) {
            return Err(HardcopyError::InvalidFaceWeight(self.weight));
        }
        if let HardcopyFaceSource::System { path } = &self.source {
            validate_text("face path", path, MAX_PATH_BYTES)?;
        }
        Ok(())
    }

    #[must_use]
    pub fn family(&self) -> &str {
        &self.family
    }

    /// The `OS/2` weight class, 400 for Regular.
    #[must_use]
    pub const fn weight(&self) -> u16 {
        self.weight
    }

    /// The face's own version string, from its `name` table.
    #[must_use]
    pub fn version(&self) -> &str {
        &self.version
    }

    /// The SHA-256 of the face's file.
    #[must_use]
    pub const fn sha256(&self) -> ContentDigest {
        self.sha256
    }

    #[must_use]
    pub const fn source(&self) -> &HardcopyFaceSource {
        &self.source
    }
}

/// `faces` in their canonical order, each once, so equal sets record equally.
pub(super) fn canonical_faces(
    mut faces: Vec<HardcopyFace>,
) -> Result<Vec<HardcopyFace>, HardcopyError> {
    faces.sort();
    faces.dedup();
    if faces.len() > MAX_FACES {
        return Err(HardcopyError::TooManyFaces(faces.len()));
    }
    for face in &faces {
        face.validate()?;
    }
    Ok(faces)
}
