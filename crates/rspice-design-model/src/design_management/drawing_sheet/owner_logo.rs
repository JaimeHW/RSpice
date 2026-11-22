//! The owner's logo: one picture the project gives every title block, which
//! each form seats in the cell that names the legal owner.
//!
//! The project keeps the PNG file as it was imported, its SHA-256 digest and
//! the words a reader who cannot see the picture is given in its place. The
//! form needs only the picture's proportions, read from the file's header;
//! the canvas and the printed page decode the pixels themselves. The bytes are
//! shared, so the drawing sheet a frame resolves carries the logo without
//! copying it.

use std::fmt;
use std::sync::Arc;

use base64::Engine as _;
use base64::engine::general_purpose::STANDARD as BASE64;
use serde::de::Error as _;
use serde::{Deserialize, Deserializer, Serialize, Serializer};
use sha2::{Digest as _, Sha256};

use super::*;

/// The eight bytes every PNG file starts with.
const PNG_SIGNATURE: [u8; 8] = [0x89, b'P', b'N', b'G', b'\r', b'\n', 0x1a, b'\n'];

/// The largest logo a project takes, the most one printed page in the
/// browser embeds for a picture: a logo that could not print is refused when
/// it is imported rather than when the sheet is printed.
pub const DRAWING_SHEET_OWNER_LOGO_MAX_BYTES: usize = 16 * 1_048_576;

/// The project's logo, as it was imported.
#[derive(Clone, PartialEq, Eq)]
pub struct DrawingSheetOwnerLogo {
    png: Arc<[u8]>,
    digest: ContentDigest,
    alternative_text: String,
    pixel_width: u32,
    pixel_height: u32,
}

impl fmt::Debug for DrawingSheetOwnerLogo {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter
            .debug_struct("DrawingSheetOwnerLogo")
            .field("digest", &self.digest)
            .field("bytes", &self.png.len())
            .field("pixel_width", &self.pixel_width)
            .field("pixel_height", &self.pixel_height)
            .field("alternative_text", &self.alternative_text)
            .finish()
    }
}

impl DrawingSheetOwnerLogo {
    /// The logo a PNG file holds, read by `alternative_text` where it cannot
    /// be seen. Refused unless the bytes are a PNG file with a picture in it.
    pub fn from_png(png: Vec<u8>, alternative_text: &str) -> Result<Self, DesignManagementError> {
        if png.len() > DRAWING_SHEET_OWNER_LOGO_MAX_BYTES {
            return Err(DesignManagementError::LimitExceeded {
                domain: "owner logo bytes",
                actual: png.len(),
                maximum: DRAWING_SHEET_OWNER_LOGO_MAX_BYTES,
            });
        }
        let (pixel_width, pixel_height) = png_pixel_size(&png).ok_or(
            DesignManagementError::InvalidOwnerLogo("The logo must be a PNG picture."),
        )?;
        let alternative_text = entered_text(alternative_text);
        validate_value("owner logo alternative text", &alternative_text, false)?;
        Ok(Self {
            digest: ContentDigest::from_bytes(Sha256::digest(&png).into()),
            png: png.into(),
            alternative_text,
            pixel_width,
            pixel_height,
        })
    }

    /// The PNG file.
    #[must_use]
    pub fn png(&self) -> &[u8] {
        &self.png
    }

    /// The PNG file's SHA-256 digest, which names the picture wherever it is
    /// cached or embedded.
    #[must_use]
    pub const fn digest(&self) -> ContentDigest {
        self.digest
    }

    /// What a reader who cannot see the logo is given in its place.
    #[must_use]
    pub fn alternative_text(&self) -> &str {
        &self.alternative_text
    }

    /// The picture's width and height in pixels.
    #[must_use]
    pub const fn pixel_size(&self) -> (u32, u32) {
        (self.pixel_width, self.pixel_height)
    }
}

/// The width and height a PNG file's header gives its picture, or `None` for
/// bytes that are not a PNG file or a picture of no pixels.
fn png_pixel_size(png: &[u8]) -> Option<(u32, u32)> {
    // The signature, then the header chunk: its 13-byte length, its type,
    // and the width and height first in its data.
    let header = png.get(..24)?;
    if header[..8] != PNG_SIGNATURE
        || header[8..12] != 13_u32.to_be_bytes()
        || &header[12..16] != b"IHDR"
    {
        return None;
    }
    let width = u32::from_be_bytes(header[16..20].try_into().ok()?);
    let height = u32::from_be_bytes(header[20..24].try_into().ok()?);
    // The PNG specification bounds each side by 2^31 - 1.
    let side = 1..=i32::MAX as u32;
    (side.contains(&width) && side.contains(&height)).then_some((width, height))
}

/// The logo as a project file stores it: the PNG file in Base64, the digest
/// that file must have, and the alternative text.
#[derive(Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
struct DrawingSheetOwnerLogoWire {
    png: String,
    digest: ContentDigest,
    alternative_text: String,
}

impl Serialize for DrawingSheetOwnerLogo {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        DrawingSheetOwnerLogoWire {
            png: BASE64.encode(&self.png),
            digest: self.digest,
            alternative_text: self.alternative_text.clone(),
        }
        .serialize(serializer)
    }
}

impl<'de> Deserialize<'de> for DrawingSheetOwnerLogo {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let wire = DrawingSheetOwnerLogoWire::deserialize(deserializer)?;
        let png = BASE64
            .decode(wire.png.as_bytes())
            .map_err(|error| D::Error::custom(format!("owner logo is not Base64: {error}")))?;
        let logo = Self::from_png(png, &wire.alternative_text).map_err(D::Error::custom)?;
        if logo.digest != wire.digest || logo.alternative_text != wire.alternative_text {
            return Err(D::Error::custom(
                DesignManagementError::SemanticDigestMismatch {
                    domain: "owner logo",
                    identity: wire.digest.to_string(),
                },
            ));
        }
        Ok(logo)
    }
}
