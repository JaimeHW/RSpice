/// Bounds every untrusted archive and catalog snapshot is read under.
///
/// The spec values are format constants, not tuning knobs: a pack that exceeds
/// them is invalid everywhere, so producers and consumers must agree. The
/// struct is injectable only so tests can prove each cap fires without
/// materializing hundreds of megabytes.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Limits {
    /// Largest `.rspicepack` archive accepted, measured on the compressed
    /// bytes as received.
    pub max_archive_bytes: usize,
    /// Largest total expanded size of all archive entries. Checked against the
    /// declared central-directory sizes before any byte is inflated, then
    /// re-proved per entry against the actual inflated length.
    pub max_expanded_bytes: usize,
    /// Largest number of archive entries, including `manifest.json` and
    /// `signature.ed25519`.
    pub max_files: usize,
    /// Largest compressed and expanded size of an encoded catalog snapshot.
    pub max_snapshot_bytes: usize,
}

impl Limits {
    /// The pack format's fixed limits.
    pub const SPEC: Self = Self {
        max_archive_bytes: 64 * 1024 * 1024,
        max_expanded_bytes: 256 * 1024 * 1024,
        max_files: 10_000,
        max_snapshot_bytes: 32 * 1024 * 1024,
    };
}

impl Default for Limits {
    fn default() -> Self {
        Self::SPEC
    }
}
