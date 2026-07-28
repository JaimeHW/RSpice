//! Native-printer and browser-print handoff adapters.
//!
//! Rendering stays in `workbench::hardcopy_render`; this module owns the
//! irreversible platform boundary. A successful native result means that the
//! Windows spooler accepted the complete GDI job. A successful browser result
//! means only that the browser accepted navigation of a user-initiated window
//! to a Blob-backed print document. Neither result claims that a browser print
//! dialog opened or that paper was produced.

// The browser build retains the shared native-print contracts so persisted
// setup and receipt data have one schema across targets. Their platform
// adapters are intentionally unreachable on wasm.
#![cfg_attr(target_arch = "wasm32", allow(dead_code))]

use std::collections::BTreeSet;
use std::sync::Arc;
use std::sync::atomic::{AtomicBool, Ordering};

use serde::{Deserialize, Deserializer, Serialize};
use sha2::{Digest as _, Sha256};

use crate::hardcopy::{
    CancellationPhase, DuplexMode, HardcopyFailureCode, HardcopyOutcome, HardcopyPlan,
    OutputFormat, PrinterJobSettings, PrinterMediaSource, PrinterRasterGeometry, RenderTarget,
    ResolvedOrientation,
};
use crate::product::ContentDigest;
use crate::workbench::hardcopy_adapters::render::{
    RenderedHardcopyPublication, RenderedPrinterPages,
};

const CAPABILITY_SCHEMA_VERSION: u32 = 1;
const MAX_PLATFORM_PRINTERS: usize = 4_096;
const MAX_CAPABILITIES_PER_DEVICE: usize = 4_096;
const MAX_PLATFORM_TEXT_BYTES: usize = 4_096;
const MAX_RECEIPT_MESSAGE_BYTES: usize = 2_048;
const PAPER_DIMENSION_TOLERANCE_UM: u64 = 100;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub enum HardcopyPlatformUnavailableReason {
    NativePrintingIsWindowsOnly,
    BrowserPrintingRequiresWebAssembly,
    BrowserWindowApiUnavailable,
    BrowserBlobApiUnavailable,
}

/// Compile-time truth about whether the current target has a native printer
/// backend. Callers use this to remove or disable native-only controls instead
/// of presenting a path that can never succeed.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum NativePrintPlatformSupport {
    Available,
    // This variant is part of the cross-target capability contract even
    // though a Windows build can only construct `Available`.
    #[cfg_attr(target_os = "windows", allow(dead_code))]
    Unavailable(HardcopyPlatformUnavailableReason),
}

#[cfg(target_os = "windows")]
#[must_use]
pub const fn native_print_platform_support() -> NativePrintPlatformSupport {
    NativePrintPlatformSupport::Available
}

#[cfg(not(target_os = "windows"))]
#[must_use]
pub const fn native_print_platform_support() -> NativePrintPlatformSupport {
    NativePrintPlatformSupport::Unavailable(
        HardcopyPlatformUnavailableReason::NativePrintingIsWindowsOnly,
    )
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct PrinterPaperCapability {
    platform_id: i16,
    display_name: String,
    portrait_width_um: u64,
    portrait_height_um: u64,
}

impl PrinterPaperCapability {
    pub fn try_new(
        platform_id: i16,
        display_name: impl Into<String>,
        width_um: u64,
        height_um: u64,
    ) -> Result<Self, HardcopyPrintError> {
        let display_name = checked_text("paper name", display_name.into())?;
        if platform_id <= 0 || width_um == 0 || height_um == 0 {
            return Err(HardcopyPrintError::InvalidCapabilitySnapshot(
                "printer paper identity and dimensions must be positive".to_owned(),
            ));
        }
        let (portrait_width_um, portrait_height_um) = if width_um <= height_um {
            (width_um, height_um)
        } else {
            (height_um, width_um)
        };
        Ok(Self {
            platform_id,
            display_name,
            portrait_width_um,
            portrait_height_um,
        })
    }

    #[must_use]
    pub const fn platform_id(&self) -> i16 {
        self.platform_id
    }

    #[must_use]
    pub fn display_name(&self) -> &str {
        &self.display_name
    }

    #[must_use]
    pub const fn portrait_dimensions_um(&self) -> (u64, u64) {
        (self.portrait_width_um, self.portrait_height_um)
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct PrinterTrayCapability {
    platform_id: i16,
    display_name: String,
}

impl PrinterTrayCapability {
    pub fn try_new(
        platform_id: i16,
        display_name: impl Into<String>,
    ) -> Result<Self, HardcopyPrintError> {
        if platform_id <= 0 {
            return Err(HardcopyPrintError::InvalidCapabilitySnapshot(
                "printer tray identity must be positive".to_owned(),
            ));
        }
        Ok(Self {
            platform_id,
            display_name: checked_text("tray name", display_name.into())?,
        })
    }

    #[must_use]
    pub const fn platform_id(&self) -> i16 {
        self.platform_id
    }

    #[must_use]
    pub fn display_name(&self) -> &str {
        &self.display_name
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Serialize, Deserialize)]
pub struct PrinterResolutionCapability {
    horizontal_dpi: u16,
    vertical_dpi: u16,
}

impl PrinterResolutionCapability {
    pub fn try_new(horizontal_dpi: u16, vertical_dpi: u16) -> Result<Self, HardcopyPrintError> {
        if !(72..=9_600).contains(&horizontal_dpi) || !(72..=9_600).contains(&vertical_dpi) {
            return Err(HardcopyPrintError::InvalidCapabilitySnapshot(format!(
                "printer resolution {horizontal_dpi}x{vertical_dpi} DPI is outside 72..=9600"
            )));
        }
        Ok(Self {
            horizontal_dpi,
            vertical_dpi,
        })
    }

    #[must_use]
    pub const fn horizontal_dpi(self) -> u16 {
        self.horizontal_dpi
    }

    #[must_use]
    pub const fn vertical_dpi(self) -> u16 {
        self.vertical_dpi
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct PrinterCapabilityDescriptor {
    pub device_id: String,
    pub display_name: String,
    pub driver_name: String,
    pub port_name: String,
    pub driver_spec_version: u16,
    pub driver_version: u16,
    pub papers: Vec<PrinterPaperCapability>,
    pub trays: Vec<PrinterTrayCapability>,
    pub resolutions: Vec<PrinterResolutionCapability>,
    pub duplex_modes: Vec<DuplexMode>,
    pub maximum_copies: u16,
    pub supports_collation: bool,
    pub supports_color: bool,
}

/// Immutable, canonicalized printer capability snapshot. The digest excludes
/// transient queue status and binds everything that can affect plan execution.
#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct PrinterCapabilitySnapshot {
    schema_version: u32,
    device_id: String,
    display_name: String,
    driver_name: String,
    port_name: String,
    driver_spec_version: u16,
    driver_version: u16,
    papers: Vec<PrinterPaperCapability>,
    trays: Vec<PrinterTrayCapability>,
    resolutions: Vec<PrinterResolutionCapability>,
    duplex_modes: Vec<DuplexMode>,
    maximum_copies: u16,
    supports_collation: bool,
    supports_color: bool,
    content_digest: ContentDigest,
}

#[derive(Deserialize)]
struct PersistedPrinterCapabilitySnapshot {
    schema_version: u32,
    device_id: String,
    display_name: String,
    driver_name: String,
    port_name: String,
    driver_spec_version: u16,
    driver_version: u16,
    papers: Vec<PrinterPaperCapability>,
    trays: Vec<PrinterTrayCapability>,
    resolutions: Vec<PrinterResolutionCapability>,
    duplex_modes: Vec<DuplexMode>,
    maximum_copies: u16,
    supports_collation: bool,
    supports_color: bool,
    content_digest: ContentDigest,
}

impl<'de> Deserialize<'de> for PrinterCapabilitySnapshot {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let persisted = PersistedPrinterCapabilitySnapshot::deserialize(deserializer)?;
        if persisted.schema_version != CAPABILITY_SCHEMA_VERSION {
            return Err(serde::de::Error::custom(format!(
                "unsupported printer capability schema version {}",
                persisted.schema_version
            )));
        }
        let expected_digest = persisted.content_digest;
        let snapshot = Self::try_from_descriptor(PrinterCapabilityDescriptor {
            device_id: persisted.device_id,
            display_name: persisted.display_name,
            driver_name: persisted.driver_name,
            port_name: persisted.port_name,
            driver_spec_version: persisted.driver_spec_version,
            driver_version: persisted.driver_version,
            papers: persisted.papers,
            trays: persisted.trays,
            resolutions: persisted.resolutions,
            duplex_modes: persisted.duplex_modes,
            maximum_copies: persisted.maximum_copies,
            supports_collation: persisted.supports_collation,
            supports_color: persisted.supports_color,
        })
        .map_err(serde::de::Error::custom)?;
        if snapshot.content_digest != expected_digest {
            return Err(serde::de::Error::custom(
                "printer capability snapshot digest mismatch",
            ));
        }
        Ok(snapshot)
    }
}

#[derive(Serialize)]
struct PrinterCapabilityMaterial<'a> {
    schema_version: u32,
    device_id: &'a str,
    display_name: &'a str,
    driver_name: &'a str,
    port_name: &'a str,
    driver_spec_version: u16,
    driver_version: u16,
    papers: &'a [PrinterPaperCapability],
    trays: &'a [PrinterTrayCapability],
    resolutions: &'a [PrinterResolutionCapability],
    duplex_modes: &'a [DuplexMode],
    maximum_copies: u16,
    supports_collation: bool,
    supports_color: bool,
}

impl PrinterCapabilitySnapshot {
    pub fn try_from_descriptor(
        descriptor: PrinterCapabilityDescriptor,
    ) -> Result<Self, HardcopyPrintError> {
        let mut papers = descriptor.papers;
        let mut trays = descriptor.trays;
        let mut resolutions = descriptor.resolutions;
        let mut duplex_modes = descriptor.duplex_modes;
        if papers.is_empty() || resolutions.is_empty() {
            return Err(HardcopyPrintError::InvalidCapabilitySnapshot(
                "printer must report at least one paper and one resolution".to_owned(),
            ));
        }
        if papers.len() > MAX_CAPABILITIES_PER_DEVICE
            || trays.len() > MAX_CAPABILITIES_PER_DEVICE
            || resolutions.len() > MAX_CAPABILITIES_PER_DEVICE
        {
            return Err(HardcopyPrintError::InvalidCapabilitySnapshot(
                "printer capability count exceeds the supported limit".to_owned(),
            ));
        }
        if !(1..=999).contains(&descriptor.maximum_copies) {
            return Err(HardcopyPrintError::InvalidCapabilitySnapshot(
                "maximum copies must be in 1..=999".to_owned(),
            ));
        }
        papers.sort_by(|left, right| {
            left.platform_id
                .cmp(&right.platform_id)
                .then_with(|| left.display_name.cmp(&right.display_name))
        });
        trays.sort_by(|left, right| {
            left.platform_id
                .cmp(&right.platform_id)
                .then_with(|| left.display_name.cmp(&right.display_name))
        });
        resolutions.sort_unstable();
        duplex_modes.sort_unstable_by_key(|mode| duplex_sort_key(*mode));
        resolutions.dedup();
        duplex_modes.dedup();
        if duplex_modes.is_empty() || !duplex_modes.contains(&DuplexMode::Off) {
            duplex_modes.insert(0, DuplexMode::Off);
        }
        ensure_unique_capabilities(&papers, |paper| paper.platform_id, "paper platform id")?;
        ensure_unique_capabilities(&trays, |tray| tray.platform_id, "tray platform id")?;
        ensure_unique_capabilities(
            &trays,
            |tray| tray.display_name.to_lowercase(),
            "tray display name",
        )?;

        let device_id = checked_text("printer device identity", descriptor.device_id)?;
        let display_name = checked_text("printer display name", descriptor.display_name)?;
        let driver_name = checked_text("printer driver name", descriptor.driver_name)?;
        let port_name = checked_text("printer port name", descriptor.port_name)?;
        let material = PrinterCapabilityMaterial {
            schema_version: CAPABILITY_SCHEMA_VERSION,
            device_id: &device_id,
            display_name: &display_name,
            driver_name: &driver_name,
            port_name: &port_name,
            driver_spec_version: descriptor.driver_spec_version,
            driver_version: descriptor.driver_version,
            papers: &papers,
            trays: &trays,
            resolutions: &resolutions,
            duplex_modes: &duplex_modes,
            maximum_copies: descriptor.maximum_copies,
            supports_collation: descriptor.supports_collation,
            supports_color: descriptor.supports_color,
        };
        let content_digest = canonical_digest(b"rspice-printer-capability-v1", &material)?;
        Ok(Self {
            schema_version: CAPABILITY_SCHEMA_VERSION,
            device_id,
            display_name,
            driver_name,
            port_name,
            driver_spec_version: descriptor.driver_spec_version,
            driver_version: descriptor.driver_version,
            papers,
            trays,
            resolutions,
            duplex_modes,
            maximum_copies: descriptor.maximum_copies,
            supports_collation: descriptor.supports_collation,
            supports_color: descriptor.supports_color,
            content_digest,
        })
    }

    #[must_use]
    pub fn device_id(&self) -> &str {
        &self.device_id
    }

    #[must_use]
    pub fn display_name(&self) -> &str {
        &self.display_name
    }

    #[must_use]
    pub fn driver_name(&self) -> &str {
        &self.driver_name
    }

    #[must_use]
    // Retained for authenticated diagnostic/export surfaces; the current
    // dialog intentionally identifies queues by device id and display name.
    pub fn port_name(&self) -> &str {
        &self.port_name
    }

    #[must_use]
    pub fn papers(&self) -> &[PrinterPaperCapability] {
        &self.papers
    }

    #[must_use]
    pub fn trays(&self) -> &[PrinterTrayCapability] {
        &self.trays
    }

    #[must_use]
    pub fn resolutions(&self) -> &[PrinterResolutionCapability] {
        &self.resolutions
    }

    #[must_use]
    pub fn duplex_modes(&self) -> &[DuplexMode] {
        &self.duplex_modes
    }

    #[must_use]
    pub const fn maximum_copies(&self) -> u16 {
        self.maximum_copies
    }

    #[must_use]
    pub const fn supports_collation(&self) -> bool {
        self.supports_collation
    }

    #[must_use]
    pub const fn supports_color(&self) -> bool {
        self.supports_color
    }

    #[must_use]
    pub const fn content_digest(&self) -> ContentDigest {
        self.content_digest
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct PrinterCatalogEntry {
    capabilities: PrinterCapabilitySnapshot,
    is_default: bool,
    queue_status_flags: u32,
}

impl PrinterCatalogEntry {
    #[must_use]
    pub const fn capabilities(&self) -> &PrinterCapabilitySnapshot {
        &self.capabilities
    }

    #[must_use]
    pub const fn is_default(&self) -> bool {
        self.is_default
    }

    #[must_use]
    // The raw spooler flags are an authenticated diagnostic value. Product
    // controls consume the normalized readiness state instead.
    pub const fn queue_status_flags(&self) -> u32 {
        self.queue_status_flags
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct PrinterDiscoveryFailure {
    pub device_id: String,
    pub message: String,
}

#[derive(Debug, Clone, Default, PartialEq, Eq, Serialize, Deserialize)]
pub struct PrinterDiscoveryReport {
    printers: Vec<PrinterCatalogEntry>,
    failures: Vec<PrinterDiscoveryFailure>,
}

impl PrinterDiscoveryReport {
    #[must_use]
    pub fn printers(&self) -> &[PrinterCatalogEntry] {
        &self.printers
    }

    #[must_use]
    // Discovery failures are exposed for diagnostics even when the current
    // dialog has at least one usable printer and therefore does not show them.
    pub fn failures(&self) -> &[PrinterDiscoveryFailure] {
        &self.failures
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ResolvedNativePrinterJob {
    paper_platform_id: i16,
    source_platform_id: Option<i16>,
    resolution_dpi: u16,
    raster_geometry: PrinterRasterGeometry,
    orientation: ResolvedOrientation,
    duplex: DuplexMode,
    copies: u16,
    collate: bool,
}

// The Windows backend consumes this sealed value by field inside its child
// module; getters remain the platform-neutral inspection contract and are
// also exercised by contract tests.
impl ResolvedNativePrinterJob {
    #[must_use]
    pub const fn paper_platform_id(&self) -> i16 {
        self.paper_platform_id
    }

    #[must_use]
    pub const fn source_platform_id(&self) -> Option<i16> {
        self.source_platform_id
    }

    #[must_use]
    pub const fn resolution_dpi(&self) -> u16 {
        self.resolution_dpi
    }

    #[must_use]
    pub const fn raster_geometry(&self) -> PrinterRasterGeometry {
        self.raster_geometry
    }

    #[must_use]
    pub const fn orientation(&self) -> ResolvedOrientation {
        self.orientation
    }

    #[must_use]
    pub const fn duplex(&self) -> DuplexMode {
        self.duplex
    }

    #[must_use]
    pub const fn copies(&self) -> u16 {
        self.copies
    }

    #[must_use]
    pub const fn collate(&self) -> bool {
        self.collate
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct PrinterDriverSettingsSuggestion {
    pub paper_platform_id: Option<i16>,
    pub media_source: PrinterMediaSource,
    pub resolution_dpi: Option<u16>,
    pub duplex: DuplexMode,
    pub copies: u16,
    pub collate: bool,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum PrinterDriverPropertiesOutcome {
    Accepted {
        capabilities: PrinterCapabilitySnapshot,
        suggestion: PrinterDriverSettingsSuggestion,
    },
    Cancelled,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct NativeWindowHandle(pub isize);

#[derive(Debug, Clone, Default)]
pub struct HardcopyCancellationToken(Arc<AtomicBool>);

impl HardcopyCancellationToken {
    pub fn cancel(&self) {
        self.0.store(true, Ordering::Release);
    }

    #[must_use]
    pub fn is_cancelled(&self) -> bool {
        self.0.load(Ordering::Acquire)
    }
}

#[derive(Debug, thiserror::Error, Clone, PartialEq, Eq)]
pub enum HardcopyPrintError {
    #[error("hardcopy platform unavailable: {0:?}")]
    #[cfg_attr(target_os = "windows", allow(dead_code))]
    PlatformUnavailable(HardcopyPlatformUnavailableReason),
    #[error("invalid printer capability snapshot: {0}")]
    InvalidCapabilitySnapshot(String),
    #[error("hardcopy plan is not a native system-printer plan")]
    NativePrinterPlanRequired,
    #[error("hardcopy plan is not a browser print-document plan")]
    #[cfg_attr(not(target_arch = "wasm32"), allow(dead_code))]
    BrowserPrintPlanRequired,
    #[error("printer capability snapshot belongs to {observed}, expected {expected}")]
    PrinterIdentityMismatch { expected: String, observed: String },
    #[error("printer capabilities changed after the immutable plan was compiled")]
    PrinterCapabilitiesChanged,
    #[error("printer does not support the planned paper dimensions")]
    UnsupportedPaper,
    #[error("selected printer paper identity {0:?} is invalid or unavailable")]
    UnsupportedPaperIdentity(String),
    #[error("planned printable ink lies outside the selected driver printable rectangle")]
    PlannedInkOutsidePrintableArea,
    #[error("printer does not support the planned media source: {0}")]
    UnsupportedMediaSource(String),
    #[error("printer does not support the planned {0} DPI resolution")]
    UnsupportedResolution(u16),
    #[error("printer does not support the planned {0:?} duplex mode")]
    UnsupportedDuplex(DuplexMode),
    #[error("printer supports at most {supported} copies, but the plan requires {requested}")]
    UnsupportedCopyCount { requested: u16, supported: u16 },
    #[error("printer does not support collation")]
    UnsupportedCollation,
    #[error("rendered printer publication does not match the immutable plan: {0}")]
    PrinterPublicationMismatch(String),
    #[error("browser print document does not match the immutable plan: {0}")]
    #[cfg_attr(not(target_arch = "wasm32"), allow(dead_code))]
    BrowserPublicationMismatch(String),
    #[error("{operation} failed with Windows error {code}")]
    Windows { operation: &'static str, code: u32 },
    #[error("printer driver rejected {operation}")]
    DriverRejected { operation: &'static str },
    #[error("printer driver reported invalid physical or printable raster geometry: {0}")]
    InvalidDriverGeometry(String),
    #[error("spool operation failed ({original}); abort cleanup also failed ({cleanup})")]
    SpoolCleanupFailed { original: String, cleanup: String },
    #[error("browser rejected {operation}: {message}")]
    #[cfg_attr(not(target_arch = "wasm32"), allow(dead_code))]
    Browser {
        operation: &'static str,
        message: String,
    },
    #[error("could not authenticate capability snapshot: {0}")]
    Digest(String),
}

/// Native spool failure with exact progress preserved at the irreversible
/// platform boundary. `pages_completed` counts only pages for which GDI
/// accepted `EndPage`; the current page is never guessed as complete.
#[derive(Debug, thiserror::Error, Clone, PartialEq, Eq)]
#[error("{error} after {pages_completed} completed page(s)")]
pub struct HardcopySpoolFailure {
    error: HardcopyPrintError,
    pages_completed: u32,
}

impl HardcopySpoolFailure {
    fn new(error: HardcopyPrintError, pages_completed: u32) -> Self {
        Self {
            error,
            pages_completed,
        }
    }

    #[must_use]
    pub const fn error(&self) -> &HardcopyPrintError {
        &self.error
    }

    #[must_use]
    pub const fn pages_completed(&self) -> u32 {
        self.pages_completed
    }

    #[must_use]
    pub fn failure_outcome(&self) -> HardcopyOutcome {
        self.error.failure_outcome(self.pages_completed)
    }

    #[must_use]
    pub fn into_parts(self) -> (HardcopyPrintError, u32) {
        (self.error, self.pages_completed)
    }
}

pub type HardcopySpoolResult = Result<HardcopyOutcome, HardcopySpoolFailure>;

impl HardcopyPrintError {
    #[must_use]
    pub fn failure_outcome(&self, pages_completed: u32) -> HardcopyOutcome {
        let (code, retryable) = match self {
            Self::Windows { code: 5, .. } => (HardcopyFailureCode::PermissionDenied, false),
            Self::Windows { code: 8 | 14, .. } => (HardcopyFailureCode::InsufficientMemory, true),
            Self::PlatformUnavailable(_)
            | Self::Windows { .. }
            | Self::DriverRejected { .. }
            | Self::InvalidDriverGeometry(_)
            | Self::SpoolCleanupFailed { .. } => (HardcopyFailureCode::DeviceUnavailable, true),
            Self::PrinterCapabilitiesChanged
            | Self::UnsupportedPaper
            | Self::UnsupportedPaperIdentity(_)
            | Self::PlannedInkOutsidePrintableArea
            | Self::UnsupportedMediaSource(_)
            | Self::UnsupportedResolution(_)
            | Self::UnsupportedDuplex(_)
            | Self::UnsupportedCopyCount { .. }
            | Self::UnsupportedCollation
            | Self::InvalidCapabilitySnapshot(_)
            | Self::PrinterIdentityMismatch { .. } => {
                (HardcopyFailureCode::InvalidPrinterConfiguration, false)
            }
            Self::NativePrinterPlanRequired
            | Self::BrowserPrintPlanRequired
            | Self::PrinterPublicationMismatch(_)
            | Self::BrowserPublicationMismatch(_)
            | Self::Digest(_) => (HardcopyFailureCode::InternalFailure, false),
            Self::Browser { .. } => (HardcopyFailureCode::DeviceUnavailable, true),
        };
        HardcopyOutcome::Failed {
            code,
            message: bounded_receipt_message(self.to_string()),
            pages_completed,
            retryable,
        }
    }
}

fn bounded_receipt_message(mut message: String) -> String {
    if message.len() <= MAX_RECEIPT_MESSAGE_BYTES {
        return message;
    }
    let mut end = MAX_RECEIPT_MESSAGE_BYTES - 3;
    while !message.is_char_boundary(end) {
        end -= 1;
    }
    message.truncate(end);
    message.push_str("...");
    message
}

pub fn resolve_native_printer_job(
    plan: &HardcopyPlan,
    capabilities: &PrinterCapabilitySnapshot,
) -> Result<ResolvedNativePrinterJob, HardcopyPrintError> {
    let RenderTarget::SystemPrinter { printer_id, job } = plan.setup().render().target() else {
        return Err(HardcopyPrintError::NativePrinterPlanRequired);
    };
    if plan.setup().render().format() != OutputFormat::NativePrinter {
        return Err(HardcopyPrintError::NativePrinterPlanRequired);
    }
    if printer_id != capabilities.device_id() {
        return Err(HardcopyPrintError::PrinterIdentityMismatch {
            expected: printer_id.clone(),
            observed: capabilities.device_id().to_owned(),
        });
    }
    if job.capabilities_digest() != capabilities.content_digest() {
        return Err(HardcopyPrintError::PrinterCapabilitiesChanged);
    }
    resolve_job_settings(plan, job, capabilities)
}

fn resolve_job_settings(
    plan: &HardcopyPlan,
    job: &PrinterJobSettings,
    capabilities: &PrinterCapabilitySnapshot,
) -> Result<ResolvedNativePrinterJob, HardcopyPrintError> {
    let (paper_width, paper_height) = plan.setup().physical_page().paper().portrait_dimensions();
    let selected_paper_id = job.selected_paper_id().parse::<i16>().map_err(|_| {
        HardcopyPrintError::UnsupportedPaperIdentity(job.selected_paper_id().to_owned())
    })?;
    let paper = capabilities
        .papers
        .iter()
        .find(|paper| paper.platform_id == selected_paper_id)
        .ok_or_else(|| {
            HardcopyPrintError::UnsupportedPaperIdentity(job.selected_paper_id().to_owned())
        })?;
    if !within_paper_tolerance(paper.portrait_width_um, paper_width.micrometres())
        || !within_paper_tolerance(paper.portrait_height_um, paper_height.micrometres())
    {
        return Err(HardcopyPrintError::UnsupportedPaper);
    }
    validate_planned_printable_area(plan, job.raster_geometry())?;
    if !capabilities.resolutions.iter().any(|resolution| {
        resolution.horizontal_dpi == job.resolution_dpi()
            && resolution.vertical_dpi == job.resolution_dpi()
    }) {
        return Err(HardcopyPrintError::UnsupportedResolution(
            job.resolution_dpi(),
        ));
    }
    if !capabilities.duplex_modes.contains(&job.duplex()) {
        return Err(HardcopyPrintError::UnsupportedDuplex(job.duplex()));
    }
    if job.copies() > capabilities.maximum_copies {
        return Err(HardcopyPrintError::UnsupportedCopyCount {
            requested: job.copies(),
            supported: capabilities.maximum_copies,
        });
    }
    if job.collate() && !capabilities.supports_collation {
        return Err(HardcopyPrintError::UnsupportedCollation);
    }
    let source_platform_id =
        match job.media_source() {
            PrinterMediaSource::AutomaticCompatibleTray => None,
            PrinterMediaSource::NamedTray(name) => Some(
                capabilities
                    .trays
                    .iter()
                    .find(|tray| tray.display_name == *name)
                    .ok_or_else(|| HardcopyPrintError::UnsupportedMediaSource(name.clone()))?
                    .platform_id,
            ),
            PrinterMediaSource::ManualFeed => Some(
                capabilities
                    .trays
                    .iter()
                    .find(|tray| matches!(tray.platform_id, 4 | 6))
                    .ok_or_else(|| {
                        HardcopyPrintError::UnsupportedMediaSource("manual feed".to_owned())
                    })?
                    .platform_id,
            ),
            PrinterMediaSource::Roll { width } => {
                if capabilities.papers.iter().any(|paper| {
                    within_paper_tolerance(paper.portrait_width_um, width.micrometres())
                }) {
                    None
                } else {
                    return Err(HardcopyPrintError::UnsupportedMediaSource(format!(
                        "{} um roll",
                        width.micrometres()
                    )));
                }
            }
        };
    Ok(ResolvedNativePrinterJob {
        paper_platform_id: paper.platform_id,
        source_platform_id,
        resolution_dpi: job.resolution_dpi(),
        raster_geometry: job.raster_geometry(),
        orientation: plan.pagination().geometry().orientation(),
        duplex: job.duplex(),
        copies: job.copies(),
        collate: job.collate(),
    })
}

fn validate_printer_publication(
    plan: &HardcopyPlan,
    pages: &RenderedPrinterPages,
    job: &ResolvedNativePrinterJob,
) -> Result<(), HardcopyPrintError> {
    let expected_pages = plan.pagination().pages().len();
    if pages.pages().len() != expected_pages {
        return Err(HardcopyPrintError::PrinterPublicationMismatch(format!(
            "rendered {} pages, expected {expected_pages}",
            pages.pages().len()
        )));
    }
    let (expected_width, expected_height) = job.raster_geometry.physical_size_px();
    for (index, page) in pages.pages().iter().enumerate() {
        if page.page_number() != index as u32 + 1
            || page.dpi() != job.resolution_dpi
            || page.width() != expected_width
            || page.height() != expected_height
            || page.rgba().len() != page.width() as usize * page.height() as usize * 4
        {
            return Err(HardcopyPrintError::PrinterPublicationMismatch(format!(
                "page {} geometry, order, resolution, or byte length differs from the plan",
                index + 1
            )));
        }
    }
    Ok(())
}

fn validate_planned_printable_area(
    plan: &HardcopyPlan,
    raster: PrinterRasterGeometry,
) -> Result<(), HardcopyPrintError> {
    let geometry = plan.pagination().geometry();
    let (physical_width, physical_height) = geometry.physical_size();
    let (raster_width, raster_height) = raster.physical_size_px();
    let planned = geometry.printable_rect();
    let scale_floor = |position_um: u64, physical_um: u64, pixels: u32| -> u32 {
        ((u128::from(position_um) * u128::from(pixels)) / u128::from(physical_um)) as u32
    };
    let scale_ceil = |position_um: u64, physical_um: u64, pixels: u32| -> u32 {
        (u128::from(position_um) * u128::from(pixels)).div_ceil(u128::from(physical_um)) as u32
    };
    let planned_left = scale_floor(
        planned.x.micrometres(),
        physical_width.micrometres(),
        raster_width,
    );
    let planned_top = scale_floor(
        planned.y.micrometres(),
        physical_height.micrometres(),
        raster_height,
    );
    let planned_right = scale_ceil(
        planned.x.micrometres() + planned.width.micrometres(),
        physical_width.micrometres(),
        raster_width,
    );
    let planned_bottom = scale_ceil(
        planned.y.micrometres() + planned.height.micrometres(),
        physical_height.micrometres(),
        raster_height,
    );
    let (device_x, device_y, device_width, device_height) = raster.printable_rect_px();
    let device_right = device_x + device_width;
    let device_bottom = device_y + device_height;
    if planned_left < device_x
        || planned_top < device_y
        || planned_right > device_right
        || planned_bottom > device_bottom
    {
        return Err(HardcopyPrintError::PlannedInkOutsidePrintableArea);
    }
    Ok(())
}

fn within_paper_tolerance(observed: u64, expected: u64) -> bool {
    observed.abs_diff(expected) <= PAPER_DIMENSION_TOLERANCE_UM
}

fn duplex_sort_key(value: DuplexMode) -> u8 {
    match value {
        DuplexMode::Off => 0,
        DuplexMode::LongEdge => 1,
        DuplexMode::ShortEdge => 2,
    }
}

fn checked_text(field: &'static str, value: String) -> Result<String, HardcopyPrintError> {
    if value.is_empty()
        || value.trim() != value
        || value.len() > MAX_PLATFORM_TEXT_BYTES
        || value
            .chars()
            .any(|character| character == '\0' || (character.is_control() && character != '\t'))
    {
        Err(HardcopyPrintError::InvalidCapabilitySnapshot(format!(
            "{field} is empty, untrimmed, contains control characters, or is too long"
        )))
    } else {
        Ok(value)
    }
}

fn ensure_unique_capabilities<T, K: Ord>(
    values: &[T],
    key: impl Fn(&T) -> K,
    field: &'static str,
) -> Result<(), HardcopyPrintError> {
    let mut seen = BTreeSet::new();
    if values.iter().any(|value| !seen.insert(key(value))) {
        Err(HardcopyPrintError::InvalidCapabilitySnapshot(format!(
            "duplicate {field}"
        )))
    } else {
        Ok(())
    }
}

fn canonical_digest<T: Serialize>(
    domain: &[u8],
    value: &T,
) -> Result<ContentDigest, HardcopyPrintError> {
    let bytes =
        serde_json::to_vec(value).map_err(|error| HardcopyPrintError::Digest(error.to_string()))?;
    let mut digest = Sha256::new();
    digest.update(domain);
    digest.update((bytes.len() as u64).to_le_bytes());
    digest.update(bytes);
    Ok(ContentDigest::from_bytes(digest.finalize().into()))
}

#[cfg(target_os = "windows")]
pub fn discover_native_printers() -> Result<PrinterDiscoveryReport, HardcopyPrintError> {
    windows_backend::discover_native_printers()
}

#[cfg(not(target_os = "windows"))]
pub fn discover_native_printers() -> Result<PrinterDiscoveryReport, HardcopyPrintError> {
    Err(HardcopyPrintError::PlatformUnavailable(
        HardcopyPlatformUnavailableReason::NativePrintingIsWindowsOnly,
    ))
}

#[cfg(target_os = "windows")]
pub fn show_native_printer_properties(
    printer_id: &str,
    owner: Option<NativeWindowHandle>,
) -> Result<PrinterDriverPropertiesOutcome, HardcopyPrintError> {
    windows_backend::show_native_printer_properties(printer_id, owner)
}

/// Resolve one exact native paper/orientation/DPI mode before plan
/// compilation. The returned geometry must be stored in `PrinterJobSettings`;
/// spooling re-resolves and byte-for-byte compares it against the current
/// driver mode.
#[cfg(target_os = "windows")]
pub fn resolve_native_printer_mode(
    capabilities: &PrinterCapabilitySnapshot,
    selected_paper_id: &str,
    resolution_dpi: u16,
    orientation: ResolvedOrientation,
) -> Result<PrinterRasterGeometry, HardcopyPrintError> {
    windows_backend::resolve_native_printer_mode(
        capabilities,
        selected_paper_id,
        resolution_dpi,
        orientation,
    )
}

#[cfg(not(target_os = "windows"))]
pub fn resolve_native_printer_mode(
    _capabilities: &PrinterCapabilitySnapshot,
    _selected_paper_id: &str,
    _resolution_dpi: u16,
    _orientation: ResolvedOrientation,
) -> Result<PrinterRasterGeometry, HardcopyPrintError> {
    Err(HardcopyPrintError::PlatformUnavailable(
        HardcopyPlatformUnavailableReason::NativePrintingIsWindowsOnly,
    ))
}

#[cfg(not(target_os = "windows"))]
pub fn show_native_printer_properties(
    _printer_id: &str,
    _owner: Option<NativeWindowHandle>,
) -> Result<PrinterDriverPropertiesOutcome, HardcopyPrintError> {
    Err(HardcopyPrintError::PlatformUnavailable(
        HardcopyPlatformUnavailableReason::NativePrintingIsWindowsOnly,
    ))
}

#[cfg(target_os = "windows")]
pub fn spool_native_hardcopy(
    plan: &HardcopyPlan,
    pages: &RenderedPrinterPages,
    capabilities: &PrinterCapabilitySnapshot,
    cancellation: &HardcopyCancellationToken,
) -> HardcopySpoolResult {
    windows_backend::spool_native_hardcopy(plan, pages, capabilities, cancellation)
}

#[cfg(not(target_os = "windows"))]
pub fn spool_native_hardcopy(
    _plan: &HardcopyPlan,
    _pages: &RenderedPrinterPages,
    _capabilities: &PrinterCapabilitySnapshot,
    _cancellation: &HardcopyCancellationToken,
) -> HardcopySpoolResult {
    Err(HardcopySpoolFailure::new(
        HardcopyPrintError::PlatformUnavailable(
            HardcopyPlatformUnavailableReason::NativePrintingIsWindowsOnly,
        ),
        0,
    ))
}

#[cfg(target_arch = "wasm32")]
pub(crate) use browser_backend::BrowserPrintReservation;

#[cfg(target_arch = "wasm32")]
pub(crate) fn reserve_browser_print_window() -> Result<BrowserPrintReservation, HardcopyPrintError>
{
    browser_backend::reserve_browser_print_window()
}

#[cfg(target_arch = "wasm32")]
pub(crate) fn finalize_browser_print(
    reservation: BrowserPrintReservation,
    plan: &HardcopyPlan,
    publication: &RenderedHardcopyPublication,
) -> Result<HardcopyOutcome, HardcopyPrintError> {
    browser_backend::finalize_browser_print(reservation, plan, publication)
}

#[cfg(target_arch = "wasm32")]
pub fn handoff_browser_print(
    plan: &HardcopyPlan,
    publication: &RenderedHardcopyPublication,
) -> Result<HardcopyOutcome, HardcopyPrintError> {
    let reservation = reserve_browser_print_window()?;
    finalize_browser_print(reservation, plan, publication)
}

#[cfg(not(target_arch = "wasm32"))]
// Kept as the platform-neutral boundary so callers can compile one workflow;
// desktop builds report the unsupported browser handoff deterministically.
pub fn handoff_browser_print(
    _plan: &HardcopyPlan,
    _publication: &RenderedHardcopyPublication,
) -> Result<HardcopyOutcome, HardcopyPrintError> {
    Err(HardcopyPrintError::PlatformUnavailable(
        HardcopyPlatformUnavailableReason::BrowserPrintingRequiresWebAssembly,
    ))
}

trait NativeSpoolBackend {
    fn start_job(&mut self) -> Result<String, HardcopyPrintError>;
    fn start_page(&mut self) -> Result<(), HardcopyPrintError>;
    fn write_page(
        &mut self,
        page: &crate::workbench::hardcopy_adapters::render::PrinterRasterPage,
    ) -> Result<(), HardcopyPrintError>;
    fn end_page(&mut self) -> Result<(), HardcopyPrintError>;
    fn finish_job(&mut self) -> Result<(), HardcopyPrintError>;
    fn abort_job(&mut self) -> Result<(), HardcopyPrintError>;
}

fn abort_after_error<B: NativeSpoolBackend>(
    backend: &mut B,
    original: HardcopyPrintError,
) -> HardcopyPrintError {
    match backend.abort_job() {
        Ok(()) => original,
        Err(cleanup) => HardcopyPrintError::SpoolCleanupFailed {
            original: original.to_string(),
            cleanup: cleanup.to_string(),
        },
    }
}

fn abort_for_cancellation<B: NativeSpoolBackend>(
    backend: &mut B,
    pages_completed: u32,
) -> Result<(), HardcopySpoolFailure> {
    backend.abort_job().map_err(|cleanup| {
        HardcopySpoolFailure::new(
            HardcopyPrintError::SpoolCleanupFailed {
                original: "cancellation requested".to_owned(),
                cleanup: cleanup.to_string(),
            },
            pages_completed,
        )
    })
}

fn spool_transaction<B: NativeSpoolBackend>(
    backend: &mut B,
    pages: &RenderedPrinterPages,
    device_id: &str,
    cancellation: &HardcopyCancellationToken,
) -> HardcopySpoolResult {
    if cancellation.is_cancelled() {
        return Ok(HardcopyOutcome::Cancelled {
            phase: CancellationPhase::Preparing,
            pages_completed: 0,
            reason: Some("Cancelled before the native spool job was opened".to_owned()),
        });
    }
    let job_id = backend
        .start_job()
        .map_err(|error| HardcopySpoolFailure::new(error, 0))?;
    let mut pages_completed = 0_u32;
    for page in pages.pages() {
        if cancellation.is_cancelled() {
            abort_for_cancellation(backend, pages_completed)?;
            return Ok(HardcopyOutcome::Cancelled {
                phase: CancellationPhase::Spooling,
                pages_completed,
                reason: Some("Cancelled while submitting pages to the native spooler".to_owned()),
            });
        }
        if let Err(error) = backend
            .start_page()
            .and_then(|()| backend.write_page(page))
            .and_then(|()| backend.end_page())
        {
            return Err(HardcopySpoolFailure::new(
                abort_after_error(backend, error),
                pages_completed,
            ));
        }
        pages_completed += 1;
    }
    if cancellation.is_cancelled() {
        abort_for_cancellation(backend, pages_completed)?;
        return Ok(HardcopyOutcome::Cancelled {
            phase: CancellationPhase::Spooling,
            pages_completed,
            reason: Some("Cancelled before the native spool job was committed".to_owned()),
        });
    }
    if let Err(error) = backend.finish_job() {
        return Err(HardcopySpoolFailure::new(
            abort_after_error(backend, error),
            pages_completed,
        ));
    }
    Ok(HardcopyOutcome::SpoolAccepted {
        device_id: device_id.to_owned(),
        job_id,
        pages_accepted: pages_completed,
        source_artifact_digest: pages.digest(),
    })
}

#[cfg(target_os = "windows")]
mod windows_backend {
    use std::ffi::c_void;
    use std::mem::size_of;
    use std::ptr::{null, null_mut};

    use windows_sys::Win32::Foundation::{GetLastError, HWND};
    use windows_sys::Win32::Graphics::Gdi::{
        BI_RGB, BITMAPINFO, BITMAPINFOHEADER, CreateDCW, DEVMODEW, DIB_RGB_COLORS, DM_COLLATE,
        DM_COPIES, DM_DEFAULTSOURCE, DM_DUPLEX, DM_IN_BUFFER, DM_IN_PROMPT, DM_ORIENTATION,
        DM_OUT_BUFFER, DM_PAPERSIZE, DM_PRINTQUALITY, DM_YRESOLUTION, DMCOLLATE_FALSE,
        DMCOLLATE_TRUE, DMDUP_HORIZONTAL, DMDUP_SIMPLEX, DMDUP_VERTICAL, DMORIENT_LANDSCAPE,
        DMORIENT_PORTRAIT, DeleteDC, GetDeviceCaps, HDC, HORZRES, LOGPIXELSX, LOGPIXELSY,
        PHYSICALHEIGHT, PHYSICALOFFSETX, PHYSICALOFFSETY, PHYSICALWIDTH, SetDIBitsToDevice,
        VERTRES,
    };
    use windows_sys::Win32::Graphics::Printing::{
        ClosePrinter, DocumentPropertiesW, EnumPrintersW, GetPrinterW, OpenPrinterW,
        PRINTER_ATTRIBUTE_DEFAULT, PRINTER_ENUM_CONNECTIONS, PRINTER_ENUM_LOCAL, PRINTER_HANDLE,
        PRINTER_INFO_2W, PRINTER_INFO_4W,
    };
    use windows_sys::Win32::Storage::Xps::{
        AbortDoc, DC_BINNAMES, DC_BINS, DC_COLLATE, DC_COLORDEVICE, DC_COPIES, DC_DUPLEX,
        DC_ENUMRESOLUTIONS, DC_PAPERNAMES, DC_PAPERS, DC_PAPERSIZE, DOCINFOW, DeviceCapabilitiesW,
        EndDoc, EndPage, StartDocW, StartPage,
    };

    use super::*;

    const MAX_DEVMODE_BYTES: usize = 1_048_576;
    const CAPABILITY_TEXT_CAP: usize = 32_768;

    struct PrinterHandle(PRINTER_HANDLE);

    impl Drop for PrinterHandle {
        fn drop(&mut self) {
            if !self.0.Value.is_null() {
                // SAFETY: the handle is owned by this guard and closed once.
                unsafe { ClosePrinter(self.0) };
            }
        }
    }

    struct DevModeBuffer {
        storage: Vec<usize>,
        byte_len: usize,
    }

    impl DevModeBuffer {
        fn new(byte_len: usize) -> Result<Self, HardcopyPrintError> {
            if byte_len < size_of::<DEVMODEW>() || byte_len > MAX_DEVMODE_BYTES {
                return Err(HardcopyPrintError::InvalidCapabilitySnapshot(format!(
                    "driver returned invalid DEVMODE size {byte_len}"
                )));
            }
            let words = byte_len.div_ceil(size_of::<usize>());
            Ok(Self {
                storage: vec![0; words],
                byte_len,
            })
        }

        fn as_ptr(&self) -> *const DEVMODEW {
            self.storage.as_ptr().cast()
        }

        fn as_mut_ptr(&mut self) -> *mut DEVMODEW {
            self.storage.as_mut_ptr().cast()
        }

        fn header(&self) -> DEVMODEW {
            // SAFETY: storage is pointer-aligned and DocumentPropertiesW initialized
            // at least a complete DEVMODEW, checked by `new`.
            unsafe { self.as_ptr().read() }
        }
    }

    struct ResolvedWindowsPrinter {
        capabilities: PrinterCapabilitySnapshot,
        handle: PrinterHandle,
        name_wide: Vec<u16>,
        port_wide: Vec<u16>,
        devmode: DevModeBuffer,
        attributes: u32,
        status: u32,
    }

    pub(super) fn discover_native_printers() -> Result<PrinterDiscoveryReport, HardcopyPrintError> {
        let names = enum_printer_names()?;
        let mut report = PrinterDiscoveryReport::default();
        for name in names {
            match resolve_printer(&name) {
                Ok(printer) => report.printers.push(PrinterCatalogEntry {
                    capabilities: printer.capabilities,
                    is_default: printer.attributes & PRINTER_ATTRIBUTE_DEFAULT != 0,
                    queue_status_flags: printer.status,
                }),
                Err(error) => report.failures.push(PrinterDiscoveryFailure {
                    device_id: name,
                    message: error.to_string(),
                }),
            }
        }
        report.printers.sort_by(|left, right| {
            right
                .is_default
                .cmp(&left.is_default)
                .then_with(|| {
                    left.capabilities
                        .display_name
                        .cmp(&right.capabilities.display_name)
                })
                .then_with(|| {
                    left.capabilities
                        .device_id
                        .cmp(&right.capabilities.device_id)
                })
        });
        report
            .failures
            .sort_by(|left, right| left.device_id.cmp(&right.device_id));
        Ok(report)
    }

    pub(super) fn show_native_printer_properties(
        printer_id: &str,
        owner: Option<NativeWindowHandle>,
    ) -> Result<PrinterDriverPropertiesOutcome, HardcopyPrintError> {
        let mut printer = resolve_printer(printer_id)?;
        // SAFETY: the optional HWND is borrowed only for this modal call; the
        // printer handle, NUL-terminated name, and aligned input/output
        // DEVMODE buffers stay live, and the documented result is validated.
        let result = unsafe {
            DocumentPropertiesW(
                owner.map_or(null_mut(), |handle| handle.0 as HWND),
                printer.handle.0,
                printer.name_wide.as_ptr(),
                printer.devmode.as_mut_ptr(),
                printer.devmode.as_ptr(),
                DM_IN_PROMPT | DM_IN_BUFFER | DM_OUT_BUFFER,
            )
        };
        if result == 2 {
            return Ok(PrinterDriverPropertiesOutcome::Cancelled);
        }
        if result != 1 {
            return Err(HardcopyPrintError::DriverRejected {
                operation: "printer driver properties",
            });
        }
        let suggestion = suggestion_from_devmode(&printer.devmode, &printer.capabilities);
        Ok(PrinterDriverPropertiesOutcome::Accepted {
            capabilities: printer.capabilities,
            suggestion,
        })
    }

    pub(super) fn resolve_native_printer_mode(
        capabilities: &PrinterCapabilitySnapshot,
        selected_paper_id: &str,
        resolution_dpi: u16,
        orientation: ResolvedOrientation,
    ) -> Result<PrinterRasterGeometry, HardcopyPrintError> {
        let paper_id = selected_paper_id.parse::<i16>().map_err(|_| {
            HardcopyPrintError::UnsupportedPaperIdentity(selected_paper_id.to_owned())
        })?;
        if !capabilities
            .papers
            .iter()
            .any(|paper| paper.platform_id == paper_id)
        {
            return Err(HardcopyPrintError::UnsupportedPaperIdentity(
                selected_paper_id.to_owned(),
            ));
        }
        if !capabilities.resolutions.iter().any(|resolution| {
            resolution.horizontal_dpi == resolution_dpi && resolution.vertical_dpi == resolution_dpi
        }) {
            return Err(HardcopyPrintError::UnsupportedResolution(resolution_dpi));
        }
        let mut printer = resolve_printer(capabilities.device_id())?;
        if printer.capabilities.content_digest() != capabilities.content_digest() {
            return Err(HardcopyPrintError::PrinterCapabilitiesChanged);
        }
        apply_mode_to_devmode(&mut printer.devmode, paper_id, resolution_dpi, orientation);
        validate_driver_devmode(&mut printer)?;
        validate_devmode_mode(&printer.devmode, paper_id, resolution_dpi, orientation)?;
        let hdc = create_printer_dc(&printer)?;
        let geometry = device_raster_geometry(hdc);
        // SAFETY: `hdc` was returned by CreateDCW above and has not been
        // transferred or deleted; this is its single unconditional release.
        unsafe { DeleteDC(hdc) };
        geometry
    }

    pub(super) fn spool_native_hardcopy(
        plan: &HardcopyPlan,
        pages: &RenderedPrinterPages,
        capabilities: &PrinterCapabilitySnapshot,
        cancellation: &HardcopyCancellationToken,
    ) -> HardcopySpoolResult {
        let no_progress = |error| HardcopySpoolFailure::new(error, 0);
        let job = resolve_native_printer_job(plan, capabilities).map_err(no_progress)?;
        validate_printer_publication(plan, pages, &job).map_err(no_progress)?;
        let RenderTarget::SystemPrinter { printer_id, .. } = plan.setup().render().target() else {
            return Err(no_progress(HardcopyPrintError::NativePrinterPlanRequired));
        };
        let mut printer = resolve_printer(printer_id).map_err(no_progress)?;
        if printer.capabilities.content_digest() != capabilities.content_digest() {
            return Err(no_progress(HardcopyPrintError::PrinterCapabilitiesChanged));
        }
        apply_job_to_devmode(&mut printer.devmode, &job).map_err(no_progress)?;
        validate_driver_devmode(&mut printer).map_err(no_progress)?;
        validate_devmode_matches_job(&printer.devmode, &job).map_err(no_progress)?;
        let title =
            wide(&format!("RSpice — {}", plan.source().display_name())).map_err(no_progress)?;
        let mut backend =
            WindowsGdiSpoolBackend::open(&printer, &job, title).map_err(no_progress)?;
        spool_transaction(&mut backend, pages, printer_id, cancellation)
    }

    fn enum_printer_names() -> Result<Vec<String>, HardcopyPrintError> {
        let flags = PRINTER_ENUM_LOCAL | PRINTER_ENUM_CONNECTIONS;
        let mut needed = 0_u32;
        let mut returned = 0_u32;
        // SAFETY: null output with zero size is the documented sizing call.
        let first =
            unsafe { EnumPrintersW(flags, null(), 4, null_mut(), 0, &mut needed, &mut returned) };
        if first != 0 && returned == 0 {
            return Ok(Vec::new());
        }
        if needed == 0 {
            return Ok(Vec::new());
        }
        let mut buffer = vec![0_u8; needed as usize];
        // SAFETY: `buffer` owns exactly `needed` writable bytes for the
        // documented second EnumPrintersW call; all count pointers are valid
        // for the duration of the call and the return value is checked.
        let ok = unsafe {
            EnumPrintersW(
                flags,
                null(),
                4,
                buffer.as_mut_ptr(),
                needed,
                &mut needed,
                &mut returned,
            )
        };
        if ok == 0 {
            return Err(last_windows_error("EnumPrintersW"));
        }
        if returned as usize > MAX_PLATFORM_PRINTERS {
            return Err(HardcopyPrintError::InvalidCapabilitySnapshot(
                "Windows printer count exceeds the supported limit".to_owned(),
            ));
        }
        let records_bytes = (returned as usize)
            .checked_mul(size_of::<PRINTER_INFO_4W>())
            .ok_or_else(|| {
                HardcopyPrintError::InvalidCapabilitySnapshot(
                    "Windows printer record byte count overflowed".to_owned(),
                )
            })?;
        if records_bytes > buffer.len() {
            return Err(HardcopyPrintError::InvalidCapabilitySnapshot(
                "Windows printer records exceed the returned buffer".to_owned(),
            ));
        }
        let mut names = Vec::with_capacity(returned as usize);
        for index in 0..returned as usize {
            // SAFETY: `records_bytes <= buffer.len()` above and `index` is
            // strictly below `returned`; the pointer is not dereferenced here.
            let pointer = unsafe {
                buffer
                    .as_ptr()
                    .add(index * size_of::<PRINTER_INFO_4W>())
                    .cast::<PRINTER_INFO_4W>()
            };
            // SAFETY: EnumPrintersW wrote `returned` complete level-4 records.
            let info = unsafe { pointer.read_unaligned() };
            names.push(string_from_wide_in_buffer(
                info.pPrinterName,
                &buffer,
                CAPABILITY_TEXT_CAP,
            )?);
        }
        names.sort();
        names.dedup();
        Ok(names)
    }

    fn resolve_printer(printer_id: &str) -> Result<ResolvedWindowsPrinter, HardcopyPrintError> {
        let name_wide = wide(printer_id)?;
        let mut handle = PRINTER_HANDLE { Value: null_mut() };
        // SAFETY: `name_wide` is NUL-terminated and lives through the call;
        // `handle` is a valid writable out-parameter and the result is checked.
        let ok = unsafe { OpenPrinterW(name_wide.as_ptr(), &mut handle, null()) };
        if ok == 0 || handle.Value.is_null() {
            return Err(last_windows_error("OpenPrinterW"));
        }
        let handle = PrinterHandle(handle);
        let info_buffer = get_printer_info_2(handle.0)?;
        // SAFETY: `get_printer_info_2` verifies the buffer can hold one
        // PRINTER_INFO_2W; unaligned read is required for a byte buffer.
        let info = unsafe {
            info_buffer
                .as_ptr()
                .cast::<PRINTER_INFO_2W>()
                .read_unaligned()
        };
        let display_name =
            string_from_wide_in_buffer(info.pPrinterName, &info_buffer, CAPABILITY_TEXT_CAP)?;
        let driver_name =
            string_from_wide_in_buffer(info.pDriverName, &info_buffer, CAPABILITY_TEXT_CAP)?;
        let port_name =
            string_from_wide_in_buffer(info.pPortName, &info_buffer, CAPABILITY_TEXT_CAP)?;
        let port_wide = wide(&port_name)?;
        let devmode = default_devmode(handle.0, &name_wide)?;
        let capabilities = resolve_capabilities(
            printer_id,
            &display_name,
            &driver_name,
            &port_name,
            &name_wide,
            &port_wide,
            &devmode,
        )?;
        Ok(ResolvedWindowsPrinter {
            capabilities,
            handle,
            name_wide,
            port_wide,
            devmode,
            attributes: info.Attributes,
            status: info.Status,
        })
    }

    fn get_printer_info_2(handle: PRINTER_HANDLE) -> Result<Vec<u8>, HardcopyPrintError> {
        let mut needed = 0_u32;
        // SAFETY: this is the documented zero-length sizing call; `handle`
        // remains owned by PrinterHandle and `needed` is a valid out-pointer.
        unsafe { GetPrinterW(handle, 2, null_mut(), 0, &mut needed) };
        if needed < size_of::<PRINTER_INFO_2W>() as u32 {
            return Err(last_windows_error("GetPrinterW size query"));
        }
        let mut buffer = vec![0_u8; needed as usize];
        // SAFETY: `buffer` provides the exact writable byte capacity requested
        // by the sizing call, and the API result is checked before use.
        let ok = unsafe { GetPrinterW(handle, 2, buffer.as_mut_ptr(), needed, &mut needed) };
        if ok == 0 {
            return Err(last_windows_error("GetPrinterW"));
        }
        Ok(buffer)
    }

    fn default_devmode(
        handle: PRINTER_HANDLE,
        name: &[u16],
    ) -> Result<DevModeBuffer, HardcopyPrintError> {
        // SAFETY: `name` is a retained NUL-terminated UTF-16 string and
        // `handle` is live; null output requests only the required byte count.
        let byte_len = unsafe {
            DocumentPropertiesW(null_mut(), handle, name.as_ptr(), null_mut(), null(), 0)
        };
        if byte_len <= 0 {
            return Err(HardcopyPrintError::DriverRejected {
                operation: "DEVMODE size query",
            });
        }
        let mut buffer = DevModeBuffer::new(byte_len as usize)?;
        // SAFETY: DevModeBuffer is DEVMODEW-aligned, owns `byte_len` writable
        // bytes, and remains live through the checked driver call.
        let result = unsafe {
            DocumentPropertiesW(
                null_mut(),
                handle,
                name.as_ptr(),
                buffer.as_mut_ptr(),
                null(),
                DM_OUT_BUFFER,
            )
        };
        if result != 1 {
            return Err(HardcopyPrintError::DriverRejected {
                operation: "default DEVMODE resolution",
            });
        }
        let header = buffer.header();
        let declared = usize::from(header.dmSize) + usize::from(header.dmDriverExtra);
        if (header.dmSize as usize) < size_of::<DEVMODEW>() || declared > buffer.byte_len {
            return Err(HardcopyPrintError::InvalidCapabilitySnapshot(
                "printer driver returned an inconsistent DEVMODE header".to_owned(),
            ));
        }
        Ok(buffer)
    }

    #[allow(clippy::too_many_arguments)]
    fn resolve_capabilities(
        device_id: &str,
        display_name: &str,
        driver_name: &str,
        port_name: &str,
        device_wide: &[u16],
        port_wide: &[u16],
        devmode: &DevModeBuffer,
    ) -> Result<PrinterCapabilitySnapshot, HardcopyPrintError> {
        let header = devmode.header();
        let papers = paper_capabilities(device_wide, port_wide, devmode)?;
        let trays = tray_capabilities(device_wide, port_wide, devmode)?;
        let resolutions = resolution_capabilities(device_wide, port_wide, devmode, &header)?;
        let duplex_supported =
            device_capability_scalar(device_wide, port_wide, DC_DUPLEX, devmode.as_ptr())? > 0;
        let mut duplex_modes = vec![DuplexMode::Off];
        if duplex_supported {
            duplex_modes.extend([DuplexMode::LongEdge, DuplexMode::ShortEdge]);
        }
        let maximum_copies =
            device_capability_scalar(device_wide, port_wide, DC_COPIES, devmode.as_ptr())?
                .clamp(1, 999) as u16;
        let supports_collation =
            device_capability_scalar(device_wide, port_wide, DC_COLLATE, devmode.as_ptr())? > 0;
        let supports_color =
            device_capability_scalar(device_wide, port_wide, DC_COLORDEVICE, devmode.as_ptr())? > 0;
        PrinterCapabilitySnapshot::try_from_descriptor(PrinterCapabilityDescriptor {
            device_id: device_id.to_owned(),
            display_name: display_name.to_owned(),
            driver_name: driver_name.to_owned(),
            port_name: port_name.to_owned(),
            driver_spec_version: header.dmSpecVersion,
            driver_version: header.dmDriverVersion,
            papers,
            trays,
            resolutions,
            duplex_modes,
            maximum_copies,
            supports_collation,
            supports_color,
        })
    }

    fn paper_capabilities(
        device: &[u16],
        port: &[u16],
        devmode: &DevModeBuffer,
    ) -> Result<Vec<PrinterPaperCapability>, HardcopyPrintError> {
        let count = capability_count(device, port, DC_PAPERS, devmode)?;
        let ids = capability_i16(device, port, DC_PAPERS, devmode, count)?;
        let names = capability_fixed_wide(device, port, DC_PAPERNAMES, devmode, count, 64)?;
        let sizes = capability_i32_pairs(device, port, DC_PAPERSIZE, devmode, count)?;
        if ids.len() != names.len() || ids.len() != sizes.len() {
            return Err(HardcopyPrintError::InvalidCapabilitySnapshot(
                "printer paper capability arrays disagree in length".to_owned(),
            ));
        }
        ids.into_iter()
            .zip(names)
            .zip(sizes)
            .map(|((id, name), (width_tenth_mm, height_tenth_mm))| {
                if width_tenth_mm <= 0 || height_tenth_mm <= 0 {
                    return Err(HardcopyPrintError::InvalidCapabilitySnapshot(
                        "printer reported a non-positive paper dimension".to_owned(),
                    ));
                }
                PrinterPaperCapability::try_new(
                    id,
                    name,
                    width_tenth_mm as u64 * 100,
                    height_tenth_mm as u64 * 100,
                )
            })
            .collect()
    }

    fn tray_capabilities(
        device: &[u16],
        port: &[u16],
        devmode: &DevModeBuffer,
    ) -> Result<Vec<PrinterTrayCapability>, HardcopyPrintError> {
        let count = capability_count(device, port, DC_BINS, devmode)?;
        let ids = capability_i16(device, port, DC_BINS, devmode, count)?;
        let names = capability_fixed_wide(device, port, DC_BINNAMES, devmode, count, 24)?;
        if ids.len() != names.len() {
            return Err(HardcopyPrintError::InvalidCapabilitySnapshot(
                "printer tray capability arrays disagree in length".to_owned(),
            ));
        }
        ids.into_iter()
            .zip(names)
            .map(|(id, name)| PrinterTrayCapability::try_new(id, name))
            .collect()
    }

    fn resolution_capabilities(
        device: &[u16],
        port: &[u16],
        devmode: &DevModeBuffer,
        header: &DEVMODEW,
    ) -> Result<Vec<PrinterResolutionCapability>, HardcopyPrintError> {
        // SAFETY: device/port are NUL-terminated retained UTF-16 buffers,
        // `devmode` is aligned and initialized, and null output requests only
        // the documented resolution-pair count.
        let count_result = unsafe {
            DeviceCapabilitiesW(
                device.as_ptr(),
                port.as_ptr(),
                DC_ENUMRESOLUTIONS,
                null_mut(),
                devmode.as_ptr(),
            )
        };
        if count_result > 0 {
            let pairs = capability_i32_pairs(
                device,
                port,
                DC_ENUMRESOLUTIONS,
                devmode,
                count_result as usize,
            )?;
            let mut resolutions = pairs
                .into_iter()
                .filter_map(|(x, y)| {
                    let x = u16::try_from(x).ok()?;
                    let y = u16::try_from(y).ok()?;
                    PrinterResolutionCapability::try_new(x, y).ok()
                })
                .collect::<Vec<_>>();
            resolutions.sort_unstable();
            resolutions.dedup();
            if !resolutions.is_empty() {
                return Ok(resolutions);
            }
        }
        // SAFETY: the DEVMODE anonymous union variant is the documented
        // printer variant and `header` came from a validated printer DEVMODE.
        let anonymous = unsafe { header.Anonymous1.Anonymous1 };
        let x = anonymous.dmPrintQuality;
        let y = header.dmYResolution;
        if x >= 72 && y >= 72 {
            return Ok(vec![PrinterResolutionCapability::try_new(
                x as u16, y as u16,
            )?]);
        }
        Err(HardcopyPrintError::InvalidCapabilitySnapshot(
            "printer did not report any concrete positive DPI resolutions".to_owned(),
        ))
    }

    fn capability_count(
        device: &[u16],
        port: &[u16],
        capability: u16,
        devmode: &DevModeBuffer,
    ) -> Result<usize, HardcopyPrintError> {
        // SAFETY: device/port are NUL-terminated and live, `devmode` points to
        // an initialized aligned DEVMODE, and null output is the sizing form.
        let count = unsafe {
            DeviceCapabilitiesW(
                device.as_ptr(),
                port.as_ptr(),
                capability,
                null_mut(),
                devmode.as_ptr(),
            )
        };
        if count < 0 {
            return Err(last_windows_error("DeviceCapabilitiesW count"));
        }
        let count = count as usize;
        if count > MAX_CAPABILITIES_PER_DEVICE {
            return Err(HardcopyPrintError::InvalidCapabilitySnapshot(
                "printer capability count exceeds the supported limit".to_owned(),
            ));
        }
        Ok(count)
    }

    fn capability_i16(
        device: &[u16],
        port: &[u16],
        capability: u16,
        devmode: &DevModeBuffer,
        count: usize,
    ) -> Result<Vec<i16>, HardcopyPrintError> {
        let mut values = vec![0_i16; count];
        // SAFETY: `values` owns `count * size_of::<i16>()` writable bytes,
        // matching this capability's documented element type; all input
        // pointers remain live and the returned element count is validated.
        let result = unsafe {
            DeviceCapabilitiesW(
                device.as_ptr(),
                port.as_ptr(),
                capability,
                values.as_mut_ptr().cast(),
                devmode.as_ptr(),
            )
        };
        if result < 0 || result as usize != count {
            return Err(last_windows_error("DeviceCapabilitiesW i16 array"));
        }
        Ok(values)
    }

    fn capability_i32_pairs(
        device: &[u16],
        port: &[u16],
        capability: u16,
        devmode: &DevModeBuffer,
        count: usize,
    ) -> Result<Vec<(i32, i32)>, HardcopyPrintError> {
        let mut values = vec![[0_i32; 2]; count];
        // SAFETY: `values` owns `count` writable i32 pairs as required by this
        // capability; input pointers remain valid and count is checked.
        let result = unsafe {
            DeviceCapabilitiesW(
                device.as_ptr(),
                port.as_ptr(),
                capability,
                values.as_mut_ptr().cast(),
                devmode.as_ptr(),
            )
        };
        if result < 0 || result as usize != count {
            return Err(last_windows_error("DeviceCapabilitiesW pair array"));
        }
        Ok(values
            .into_iter()
            .map(|value| (value[0], value[1]))
            .collect())
    }

    fn capability_fixed_wide(
        device: &[u16],
        port: &[u16],
        capability: u16,
        devmode: &DevModeBuffer,
        count: usize,
        width: usize,
    ) -> Result<Vec<String>, HardcopyPrintError> {
        let units = count.checked_mul(width).ok_or_else(|| {
            HardcopyPrintError::InvalidCapabilitySnapshot(
                "Windows fixed-width capability buffer overflowed".to_owned(),
            )
        })?;
        let mut values = vec![0_u16; units];
        // SAFETY: `values` owns exactly `count * width` writable UTF-16 units
        // for this fixed-width capability; inputs remain live and the driver
        // result count is validated before decoding.
        let result = unsafe {
            DeviceCapabilitiesW(
                device.as_ptr(),
                port.as_ptr(),
                capability,
                values.as_mut_ptr(),
                devmode.as_ptr(),
            )
        };
        if result < 0 || result as usize != count {
            return Err(last_windows_error("DeviceCapabilitiesW text array"));
        }
        values
            .chunks_exact(width)
            .map(|value| {
                let end = value
                    .iter()
                    .position(|unit| *unit == 0)
                    .unwrap_or(value.len());
                let text = String::from_utf16_lossy(&value[..end]).trim().to_owned();
                checked_text("Windows printer capability name", text)
            })
            .collect()
    }

    fn device_capability_scalar(
        device: &[u16],
        port: &[u16],
        capability: u16,
        devmode: *const DEVMODEW,
    ) -> Result<i32, HardcopyPrintError> {
        // SAFETY: device/port are NUL-terminated retained buffers and
        // `devmode` is either null or an initialized aligned DEVMODE supplied
        // by this module; null output requests a scalar result.
        let value = unsafe {
            DeviceCapabilitiesW(
                device.as_ptr(),
                port.as_ptr(),
                capability,
                null_mut(),
                devmode,
            )
        };
        if value < 0 {
            Err(last_windows_error("DeviceCapabilitiesW scalar"))
        } else {
            Ok(value)
        }
    }

    fn apply_job_to_devmode(
        devmode: &mut DevModeBuffer,
        job: &ResolvedNativePrinterJob,
    ) -> Result<(), HardcopyPrintError> {
        apply_mode_to_devmode(
            devmode,
            job.paper_platform_id,
            job.resolution_dpi,
            job.orientation,
        );
        // SAFETY: DevModeBuffer guarantees DEVMODEW alignment and at least a
        // full initialized DEVMODEW for the duration of this exclusive borrow.
        let mode = unsafe { &mut *devmode.as_mut_ptr() };
        mode.dmFields |= DM_DUPLEX | DM_COPIES | DM_COLLATE;
        // SAFETY: this is the printer-specific anonymous DEVMODE union variant
        // selected by the Windows printing contract.
        let common = unsafe { &mut mode.Anonymous1.Anonymous1 };
        mode.dmDuplex = match job.duplex {
            DuplexMode::Off => DMDUP_SIMPLEX,
            DuplexMode::LongEdge => DMDUP_VERTICAL,
            DuplexMode::ShortEdge => DMDUP_HORIZONTAL,
        };
        common.dmCopies = job.copies as i16;
        mode.dmCollate = if job.collate {
            DMCOLLATE_TRUE
        } else {
            DMCOLLATE_FALSE
        };
        if let Some(source) = job.source_platform_id {
            mode.dmFields |= DM_DEFAULTSOURCE;
            common.dmDefaultSource = source;
        } else {
            mode.dmFields &= !DM_DEFAULTSOURCE;
        }
        Ok(())
    }

    fn apply_mode_to_devmode(
        devmode: &mut DevModeBuffer,
        paper_id: i16,
        resolution_dpi: u16,
        orientation: ResolvedOrientation,
    ) {
        // SAFETY: DevModeBuffer guarantees DEVMODEW alignment and size, and
        // the exclusive borrow prevents aliases while fields are updated.
        let mode = unsafe { &mut *devmode.as_mut_ptr() };
        mode.dmFields |= DM_PAPERSIZE | DM_ORIENTATION | DM_PRINTQUALITY | DM_YRESOLUTION;
        // SAFETY: printer DEVMODEW uses the documented printer union variant.
        let common = unsafe { &mut mode.Anonymous1.Anonymous1 };
        common.dmPaperSize = paper_id;
        common.dmOrientation = match orientation {
            ResolvedOrientation::Portrait => DMORIENT_PORTRAIT as i16,
            ResolvedOrientation::Landscape => DMORIENT_LANDSCAPE as i16,
        };
        common.dmPrintQuality = resolution_dpi as i16;
        mode.dmYResolution = resolution_dpi as i16;
    }

    fn validate_driver_devmode(
        printer: &mut ResolvedWindowsPrinter,
    ) -> Result<(), HardcopyPrintError> {
        // SAFETY: the printer handle/name are live; DevModeBuffer owns aligned
        // initialized input/output storage and the result is checked before
        // any driver-substituted fields are consumed.
        let result = unsafe {
            DocumentPropertiesW(
                null_mut(),
                printer.handle.0,
                printer.name_wide.as_ptr(),
                printer.devmode.as_mut_ptr(),
                printer.devmode.as_ptr(),
                DM_IN_BUFFER | DM_OUT_BUFFER,
            )
        };
        if result == 1 {
            Ok(())
        } else {
            Err(HardcopyPrintError::DriverRejected {
                operation: "immutable print settings validation",
            })
        }
    }

    fn validate_devmode_matches_job(
        devmode: &DevModeBuffer,
        job: &ResolvedNativePrinterJob,
    ) -> Result<(), HardcopyPrintError> {
        validate_devmode_mode(
            devmode,
            job.paper_platform_id,
            job.resolution_dpi,
            job.orientation,
        )?;
        let mode = devmode.header();
        // SAFETY: `mode` was copied from a validated printer DEVMODEW and this
        // is its documented printer union variant.
        let common = unsafe { mode.Anonymous1.Anonymous1 };
        let expected_duplex = match job.duplex {
            DuplexMode::Off => DMDUP_SIMPLEX,
            DuplexMode::LongEdge => DMDUP_VERTICAL,
            DuplexMode::ShortEdge => DMDUP_HORIZONTAL,
        };
        let expected_collate = if job.collate {
            DMCOLLATE_TRUE
        } else {
            DMCOLLATE_FALSE
        };
        let source_matches = job.source_platform_id.is_none_or(|expected| {
            mode.dmFields & DM_DEFAULTSOURCE != 0 && common.dmDefaultSource == expected
        });
        if mode.dmFields & (DM_DUPLEX | DM_COPIES | DM_COLLATE)
            != (DM_DUPLEX | DM_COPIES | DM_COLLATE)
            || mode.dmDuplex != expected_duplex
            || common.dmCopies != job.copies as i16
            || mode.dmCollate != expected_collate
            || !source_matches
        {
            return Err(HardcopyPrintError::DriverRejected {
                operation: "exact immutable print settings (driver substituted a value)",
            });
        }
        Ok(())
    }

    fn validate_devmode_mode(
        devmode: &DevModeBuffer,
        paper_id: i16,
        resolution_dpi: u16,
        orientation: ResolvedOrientation,
    ) -> Result<(), HardcopyPrintError> {
        let mode = devmode.header();
        // SAFETY: `mode` came from a validated printer DEVMODEW and the
        // printer-specific anonymous union variant is active.
        let common = unsafe { mode.Anonymous1.Anonymous1 };
        let expected_orientation = match orientation {
            ResolvedOrientation::Portrait => DMORIENT_PORTRAIT as i16,
            ResolvedOrientation::Landscape => DMORIENT_LANDSCAPE as i16,
        };
        let required = DM_PAPERSIZE | DM_ORIENTATION | DM_PRINTQUALITY | DM_YRESOLUTION;
        if mode.dmFields & required != required
            || common.dmPaperSize != paper_id
            || common.dmOrientation != expected_orientation
            || common.dmPrintQuality != resolution_dpi as i16
            || mode.dmYResolution != resolution_dpi as i16
        {
            return Err(HardcopyPrintError::DriverRejected {
                operation: "exact paper, orientation, and resolution (driver substituted a value)",
            });
        }
        Ok(())
    }

    fn suggestion_from_devmode(
        devmode: &DevModeBuffer,
        capabilities: &PrinterCapabilitySnapshot,
    ) -> PrinterDriverSettingsSuggestion {
        let mode = devmode.header();
        // SAFETY: `mode` is a validated printer DEVMODEW; Windows defines this
        // anonymous union arm for printer settings.
        let common = unsafe { mode.Anonymous1.Anonymous1 };
        let media_source = if mode.dmFields & DM_DEFAULTSOURCE != 0 {
            capabilities
                .trays
                .iter()
                .find(|tray| tray.platform_id == common.dmDefaultSource)
                .map(|tray| {
                    if matches!(tray.platform_id, 4 | 6) {
                        PrinterMediaSource::ManualFeed
                    } else {
                        PrinterMediaSource::NamedTray(tray.display_name.clone())
                    }
                })
                .unwrap_or(PrinterMediaSource::AutomaticCompatibleTray)
        } else {
            PrinterMediaSource::AutomaticCompatibleTray
        };
        let resolution_dpi = (common.dmPrintQuality >= 72
            && mode.dmYResolution >= 72
            && common.dmPrintQuality == mode.dmYResolution)
            .then_some(common.dmPrintQuality as u16);
        let duplex = match mode.dmDuplex {
            DMDUP_VERTICAL => DuplexMode::LongEdge,
            DMDUP_HORIZONTAL => DuplexMode::ShortEdge,
            _ => DuplexMode::Off,
        };
        PrinterDriverSettingsSuggestion {
            paper_platform_id: (mode.dmFields & DM_PAPERSIZE != 0).then_some(common.dmPaperSize),
            media_source,
            resolution_dpi,
            duplex,
            copies: u16::try_from(common.dmCopies).unwrap_or(1).clamp(1, 999),
            collate: mode.dmCollate == DMCOLLATE_TRUE,
        }
    }

    fn create_printer_dc(printer: &ResolvedWindowsPrinter) -> Result<HDC, HardcopyPrintError> {
        let driver = wide("WINSPOOL")?;
        // SAFETY: all UTF-16 inputs are NUL-terminated and retained for the
        // call, and the aligned initialized DEVMODE remains live. The returned
        // HDC is checked and ownership transfers to the caller.
        let hdc = unsafe {
            CreateDCW(
                driver.as_ptr(),
                printer.name_wide.as_ptr(),
                printer.port_wide.as_ptr(),
                printer.devmode.as_ptr(),
            )
        };
        if hdc.is_null() {
            Err(last_windows_error("CreateDCW"))
        } else {
            Ok(hdc)
        }
    }

    fn device_raster_geometry(hdc: HDC) -> Result<PrinterRasterGeometry, HardcopyPrintError> {
        // SAFETY: callers pass a live printer HDC owned by this module;
        // GetDeviceCaps only reads it and each result is range-validated below.
        let physical_width = unsafe { GetDeviceCaps(hdc, PHYSICALWIDTH as i32) };
        // SAFETY: same live HDC invariant as above.
        let physical_height = unsafe { GetDeviceCaps(hdc, PHYSICALHEIGHT as i32) };
        // SAFETY: same live HDC invariant as above.
        let printable_x = unsafe { GetDeviceCaps(hdc, PHYSICALOFFSETX as i32) };
        // SAFETY: same live HDC invariant as above.
        let printable_y = unsafe { GetDeviceCaps(hdc, PHYSICALOFFSETY as i32) };
        // SAFETY: same live HDC invariant as above.
        let printable_width = unsafe { GetDeviceCaps(hdc, HORZRES as i32) };
        // SAFETY: same live HDC invariant as above.
        let printable_height = unsafe { GetDeviceCaps(hdc, VERTRES as i32) };
        let values = [
            physical_width,
            physical_height,
            printable_x,
            printable_y,
            printable_width,
            printable_height,
        ];
        if values.iter().any(|value| *value < 0) {
            return Err(HardcopyPrintError::InvalidDriverGeometry(
                "GDI returned a negative device-capability value".to_owned(),
            ));
        }
        PrinterRasterGeometry::try_new(
            physical_width as u32,
            physical_height as u32,
            printable_x as u32,
            printable_y as u32,
            printable_width as u32,
            printable_height as u32,
        )
        .map_err(|error| HardcopyPrintError::InvalidDriverGeometry(error.to_string()))
    }

    struct WindowsGdiSpoolBackend {
        hdc: HDC,
        title: Vec<u16>,
        expected_width: u32,
        expected_height: u32,
        offset_x: i32,
        offset_y: i32,
        job_started: bool,
    }

    impl WindowsGdiSpoolBackend {
        fn open(
            printer: &ResolvedWindowsPrinter,
            job: &ResolvedNativePrinterJob,
            title: Vec<u16>,
        ) -> Result<Self, HardcopyPrintError> {
            let hdc = create_printer_dc(printer)?;
            // SAFETY: `hdc` is live and exclusively owned here; capability
            // results are compared against the sealed job before retention.
            let dpi_x = unsafe { GetDeviceCaps(hdc, LOGPIXELSX as i32) };
            // SAFETY: same live, owned HDC invariant as above.
            let dpi_y = unsafe { GetDeviceCaps(hdc, LOGPIXELSY as i32) };
            let geometry = match device_raster_geometry(hdc) {
                Ok(geometry) => geometry,
                Err(error) => {
                    // SAFETY: ownership of `hdc` has not escaped and this error
                    // branch performs its single release.
                    unsafe { DeleteDC(hdc) };
                    return Err(error);
                }
            };
            let (expected_width, expected_height) = geometry.physical_size_px();
            let (offset_x, offset_y, _, _) = geometry.printable_rect_px();
            if dpi_x != i32::from(job.resolution_dpi)
                || dpi_y != i32::from(job.resolution_dpi)
                || geometry != job.raster_geometry
            {
                // SAFETY: ownership of `hdc` has not escaped and this rejected
                // configuration branch performs its single release.
                unsafe { DeleteDC(hdc) };
                return Err(HardcopyPrintError::DriverRejected {
                    operation: "planned page geometry and resolution",
                });
            }
            Ok(Self {
                hdc,
                title,
                expected_width,
                expected_height,
                offset_x: offset_x as i32,
                offset_y: offset_y as i32,
                job_started: false,
            })
        }
    }

    impl Drop for WindowsGdiSpoolBackend {
        fn drop(&mut self) {
            if self.job_started {
                // SAFETY: `self.hdc` is live and owned by this guard; AbortDoc
                // is the required best-effort rollback for an open job.
                unsafe { AbortDoc(self.hdc) };
            }
            if !self.hdc.is_null() {
                // SAFETY: this guard exclusively owns the HDC and releases it
                // exactly once when it leaves scope.
                unsafe { DeleteDC(self.hdc) };
            }
        }
    }

    impl NativeSpoolBackend for WindowsGdiSpoolBackend {
        fn start_job(&mut self) -> Result<String, HardcopyPrintError> {
            let info = DOCINFOW {
                cbSize: size_of::<DOCINFOW>() as i32,
                lpszDocName: self.title.as_ptr(),
                lpszOutput: null(),
                lpszDatatype: null(),
                fwType: 0,
            };
            // SAFETY: `self.hdc` is live; DOCINFOW and its NUL-terminated title
            // remain valid through the call, whose positive job id is checked.
            let id = unsafe { StartDocW(self.hdc, &info) };
            if id <= 0 {
                return Err(last_windows_error("StartDocW"));
            }
            self.job_started = true;
            Ok(id.to_string())
        }

        fn start_page(&mut self) -> Result<(), HardcopyPrintError> {
            // SAFETY: `self.hdc` owns a successfully started document and the
            // return code is checked before raster transfer.
            if unsafe { StartPage(self.hdc) } <= 0 {
                Err(last_windows_error("StartPage"))
            } else {
                Ok(())
            }
        }

        fn write_page(
            &mut self,
            page: &crate::workbench::hardcopy_adapters::render::PrinterRasterPage,
        ) -> Result<(), HardcopyPrintError> {
            if page.width() != self.expected_width || page.height() != self.expected_height {
                return Err(HardcopyPrintError::PrinterPublicationMismatch(format!(
                    "driver physical raster is {}x{}, rendered page is {}x{}",
                    self.expected_width,
                    self.expected_height,
                    page.width(),
                    page.height()
                )));
            }
            let bgrx = rgba_to_bgrx(page.rgba());
            let info = BITMAPINFO {
                bmiHeader: BITMAPINFOHEADER {
                    biSize: size_of::<BITMAPINFOHEADER>() as u32,
                    biWidth: page.width() as i32,
                    biHeight: -(page.height() as i32),
                    biPlanes: 1,
                    biBitCount: 32,
                    biCompression: BI_RGB,
                    biSizeImage: bgrx.len() as u32,
                    biXPelsPerMeter: dpi_to_pixels_per_metre(page.dpi()),
                    biYPelsPerMeter: dpi_to_pixels_per_metre(page.dpi()),
                    biClrUsed: 0,
                    biClrImportant: 0,
                },
                bmiColors: [Default::default()],
            };
            // SAFETY: `self.hdc` owns the active page; `bgrx` has exactly
            // width*height*4 bytes, BITMAPINFO describes that top-down buffer,
            // and all pointers remain live until the checked call returns.
            let written = unsafe {
                SetDIBitsToDevice(
                    self.hdc,
                    -self.offset_x,
                    -self.offset_y,
                    page.width(),
                    page.height(),
                    0,
                    0,
                    0,
                    page.height(),
                    bgrx.as_ptr().cast::<c_void>(),
                    &info,
                    DIB_RGB_COLORS,
                )
            };
            if written == 0 {
                Err(last_windows_error("SetDIBitsToDevice"))
            } else if written != page.height() as i32 {
                Err(HardcopyPrintError::DriverRejected {
                    operation: "complete page raster transfer",
                })
            } else {
                Ok(())
            }
        }

        fn end_page(&mut self) -> Result<(), HardcopyPrintError> {
            // SAFETY: `self.hdc` owns the active page; the return value is
            // validated before the page is counted as completed.
            if unsafe { EndPage(self.hdc) } <= 0 {
                Err(last_windows_error("EndPage"))
            } else {
                Ok(())
            }
        }

        fn finish_job(&mut self) -> Result<(), HardcopyPrintError> {
            // SAFETY: `self.hdc` owns a started document with no active page;
            // the result is checked before clearing the ownership state.
            if unsafe { EndDoc(self.hdc) } <= 0 {
                Err(last_windows_error("EndDoc"))
            } else {
                self.job_started = false;
                Ok(())
            }
        }

        fn abort_job(&mut self) -> Result<(), HardcopyPrintError> {
            if self.job_started {
                // SAFETY: `self.hdc` owns the started document; AbortDoc is
                // called at most once here because success clears the flag.
                if unsafe { AbortDoc(self.hdc) } <= 0 {
                    return Err(last_windows_error("AbortDoc"));
                }
                self.job_started = false;
            }
            Ok(())
        }
    }

    fn rgba_to_bgrx(rgba: &[u8]) -> Vec<u8> {
        let mut output = Vec::with_capacity(rgba.len());
        for pixel in rgba.chunks_exact(4) {
            output.extend_from_slice(&[pixel[2], pixel[1], pixel[0], 0]);
        }
        output
    }

    fn dpi_to_pixels_per_metre(dpi: u16) -> i32 {
        ((u32::from(dpi) * 10_000 + 127) / 254) as i32
    }

    fn wide(value: &str) -> Result<Vec<u16>, HardcopyPrintError> {
        checked_text("Windows printer string", value.to_owned())?;
        let mut wide = value.encode_utf16().collect::<Vec<_>>();
        wide.push(0);
        Ok(wide)
    }

    fn string_from_wide_in_buffer(
        pointer: *const u16,
        buffer: &[u8],
        maximum_units: usize,
    ) -> Result<String, HardcopyPrintError> {
        if pointer.is_null() || maximum_units == 0 || maximum_units > CAPABILITY_TEXT_CAP {
            return Err(HardcopyPrintError::InvalidCapabilitySnapshot(
                "Windows printer record contains an invalid string bound".to_owned(),
            ));
        }
        let buffer_start = buffer.as_ptr() as usize;
        let buffer_end = buffer_start.checked_add(buffer.len()).ok_or_else(|| {
            HardcopyPrintError::InvalidCapabilitySnapshot(
                "Windows printer buffer address overflowed".to_owned(),
            )
        })?;
        let string_start = pointer as usize;
        if string_start < buffer_start
            || string_start >= buffer_end
            || !string_start.is_multiple_of(std::mem::align_of::<u16>())
        {
            return Err(HardcopyPrintError::InvalidCapabilitySnapshot(
                "Windows printer string pointer lies outside its returned buffer".to_owned(),
            ));
        }
        let remaining_bytes = buffer_end - string_start;
        let available_units = remaining_bytes / size_of::<u16>();
        let scan_units = available_units.min(maximum_units);
        if scan_units == 0 {
            return Err(HardcopyPrintError::InvalidCapabilitySnapshot(
                "Windows printer string has no readable units".to_owned(),
            ));
        }
        // SAFETY: the pointer address was proven aligned and within `buffer`;
        // `scan_units` is bounded by the remaining complete u16 elements, so
        // the slice is formed before and instead of any unbounded dereference.
        let bounded = unsafe { std::slice::from_raw_parts(pointer, scan_units) };
        if let Some(length) = bounded.iter().position(|unit| *unit == 0) {
            return checked_text(
                "Windows printer string",
                String::from_utf16_lossy(&bounded[..length]),
            );
        }
        Err(HardcopyPrintError::InvalidCapabilitySnapshot(
            "Windows printer string is not terminated within the safety limit".to_owned(),
        ))
    }

    fn last_windows_error(operation: &'static str) -> HardcopyPrintError {
        HardcopyPrintError::Windows {
            operation,
            // SAFETY: GetLastError has no pointer or ownership preconditions
            // and is called immediately on the current thread's failure path.
            code: unsafe { GetLastError() },
        }
    }

    #[cfg(test)]
    mod windows_tests {
        use super::*;

        #[test]
        fn aligned_devmode_storage_can_hold_driver_private_bytes() {
            let bytes = size_of::<DEVMODEW>() + 257;
            let buffer = DevModeBuffer::new(bytes).unwrap();
            assert_eq!(
                (buffer.as_ptr() as usize) % std::mem::align_of::<DEVMODEW>(),
                0
            );
            assert!(std::mem::size_of_val(buffer.storage.as_slice()) >= bytes);
        }

        #[test]
        fn rgba_to_bgrx_is_channel_exact_and_opaque_padding_is_zero() {
            assert_eq!(
                rgba_to_bgrx(&[1, 2, 3, 255, 40, 50, 60, 255]),
                vec![3, 2, 1, 0, 60, 50, 40, 0]
            );
        }

        #[test]
        fn wide_string_decode_is_bounded_by_its_returned_buffer() {
            let units = [b'R' as u16, b'S' as u16, 0, b'X' as u16];
            // SAFETY: `units` remains live and the byte slice covers exactly
            // its initialized storage without exceeding the allocation.
            let bytes = unsafe {
                std::slice::from_raw_parts(
                    units.as_ptr().cast::<u8>(),
                    std::mem::size_of_val(&units),
                )
            };
            assert_eq!(
                string_from_wide_in_buffer(units.as_ptr(), bytes, units.len()).unwrap(),
                "RS"
            );
            let outside = units.as_ptr().wrapping_add(units.len());
            assert!(string_from_wide_in_buffer(outside, bytes, units.len()).is_err());

            let unterminated = [b'R' as u16, b'S' as u16];
            // SAFETY: same exact initialized-array byte view invariant as
            // above; the decoder must reject it without scanning beyond it.
            let unterminated_bytes = unsafe {
                std::slice::from_raw_parts(
                    unterminated.as_ptr().cast::<u8>(),
                    std::mem::size_of_val(&unterminated),
                )
            };
            assert!(
                string_from_wide_in_buffer(
                    unterminated.as_ptr(),
                    unterminated_bytes,
                    unterminated.len(),
                )
                .is_err()
            );
        }
    }
}

#[cfg(target_arch = "wasm32")]
mod browser_backend {
    use std::cell::Cell;
    use std::rc::Rc;

    use js_sys::{Array, Uint8Array};
    use wasm_bindgen::JsCast as _;
    use wasm_bindgen::closure::Closure;
    use web_sys::{Blob, BlobPropertyBag, Url};

    use super::*;

    pub(crate) struct BrowserPrintReservation {
        popup: Option<web_sys::Window>,
    }

    impl BrowserPrintReservation {
        pub(crate) fn cancel(mut self) {
            if let Some(popup) = self.popup.take() {
                close_or_log(&popup, "closing reserved print window");
            }
        }

        fn take_popup(&mut self) -> Result<web_sys::Window, HardcopyPrintError> {
            self.popup
                .take()
                .ok_or_else(|| HardcopyPrintError::Browser {
                    operation: "using reserved print window",
                    message: "the reserved browser print window was already consumed".to_owned(),
                })
        }
    }

    impl Drop for BrowserPrintReservation {
        fn drop(&mut self) {
            if let Some(popup) = self.popup.take() {
                close_or_log(&popup, "closing abandoned reserved print window");
            }
        }
    }

    pub(super) fn reserve_browser_print_window()
    -> Result<BrowserPrintReservation, HardcopyPrintError> {
        let host = web_sys::window().ok_or(HardcopyPrintError::PlatformUnavailable(
            HardcopyPlatformUnavailableReason::BrowserWindowApiUnavailable,
        ))?;
        match host.open_with_url_and_target("about:blank#rspice-print-reserved", "_blank") {
            Ok(Some(popup)) => Ok(BrowserPrintReservation { popup: Some(popup) }),
            Ok(None) => Err(HardcopyPrintError::Browser {
                operation: "reserving print window",
                message: "the browser blocked the user-initiated print window".to_owned(),
            }),
            Err(error) => Err(browser_error("reserving print window", error)),
        }
    }

    pub(super) fn finalize_browser_print(
        mut reservation: BrowserPrintReservation,
        plan: &HardcopyPlan,
        publication: &RenderedHardcopyPublication,
    ) -> Result<HardcopyOutcome, HardcopyPrintError> {
        if plan.setup().render().format() != OutputFormat::BrowserPrintDocument
            || !matches!(
                plan.setup().render().target(),
                RenderTarget::BrowserPrintDialog
            )
        {
            return Err(HardcopyPrintError::BrowserPrintPlanRequired);
        }
        if publication.format() != OutputFormat::BrowserPrintDocument
            || publication.page_count() != plan.pagination().pages().len() as u32
        {
            return Err(HardcopyPrintError::BrowserPublicationMismatch(
                "format or page count differs from the plan".to_owned(),
            ));
        }
        let part = publication.single_part().ok_or_else(|| {
            HardcopyPrintError::BrowserPublicationMismatch(
                "browser print publication must contain exactly one HTML document".to_owned(),
            )
        })?;
        if part.media_type() != "text/html" {
            return Err(HardcopyPrintError::BrowserPublicationMismatch(
                "browser print publication is not HTML".to_owned(),
            ));
        }
        let html = std::str::from_utf8(part.bytes()).map_err(|error| {
            HardcopyPrintError::BrowserPublicationMismatch(format!(
                "browser print HTML is not UTF-8: {error}"
            ))
        })?;
        if !html.contains(&format!(
            "name=\"rspice-plan-digest\" content=\"{}\"",
            plan.content_digest()
        )) || !html.contains("Content-Security-Policy")
            || html.contains("<script")
        {
            return Err(HardcopyPrintError::BrowserPublicationMismatch(
                "browser print HTML is not the expected self-contained authenticated document"
                    .to_owned(),
            ));
        }

        let host = web_sys::window().ok_or(HardcopyPrintError::PlatformUnavailable(
            HardcopyPlatformUnavailableReason::BrowserWindowApiUnavailable,
        ))?;
        let array = Array::new();
        let bytes = Uint8Array::new_with_length(part.bytes().len() as u32);
        bytes.copy_from(part.bytes());
        array.push(&bytes);
        let options = BlobPropertyBag::new();
        options.set_type("text/html;charset=utf-8");
        let blob = match Blob::new_with_u8_array_sequence_and_options(&array, &options) {
            Ok(blob) => blob,
            Err(error) => return Err(browser_error("creating print document Blob", error)),
        };
        let blob_url = match Url::create_object_url_with_blob(&blob) {
            Ok(url) => url,
            Err(error) => return Err(browser_error("creating print document URL", error)),
        };
        let popup = match reservation.take_popup() {
            Ok(popup) => popup,
            Err(error) => {
                revoke_or_log(&blob_url, "revoking unclaimed print document URL");
                return Err(error);
            }
        };

        let lifecycle_complete = Rc::new(Cell::new(false));
        let load_window = popup.clone();
        let load_url = blob_url.clone();
        let load_complete = lifecycle_complete.clone();
        let on_load = Closure::once_into_js(move |_event: web_sys::Event| {
            print_loaded_document(&load_window, &load_url, &load_complete);
        });
        if let Err(error) = popup.add_event_listener_with_callback("load", on_load.unchecked_ref())
        {
            lifecycle_complete.set(true);
            close_or_log(&popup, "closing print window after load-handler failure");
            revoke_or_log(
                &blob_url,
                "revoking print document URL after load-handler failure",
            );
            return Err(browser_error("installing print load handler", error));
        }

        if popup.closed().unwrap_or(true) {
            lifecycle_complete.set(true);
            revoke_or_log(&blob_url, "revoking print URL for a closed reservation");
            return Err(HardcopyPrintError::Browser {
                operation: "using reserved print window",
                message: "the reserved browser print window was closed before rendering finished"
                    .to_owned(),
            });
        }
        if let Err(error) = popup.location().set_href(&blob_url) {
            lifecycle_complete.set(true);
            close_or_log(&popup, "closing print window after navigation failure");
            revoke_or_log(
                &blob_url,
                "revoking print document URL after navigation failure",
            );
            return Err(browser_error("navigating reserved print window", error));
        }

        // Browser navigation and load errors are asynchronous and cannot alter
        // the synchronous navigation receipt. A bounded watchdog nevertheless
        // guarantees cleanup and emits a truthful diagnostic when no load
        // event arrives.
        let timeout_window = popup.clone();
        let timeout_url = blob_url.clone();
        let timeout_complete = lifecycle_complete.clone();
        let on_timeout = Closure::once_into_js(move || {
            if timeout_complete.replace(true) {
                return;
            }
            web_sys::console::error_1(&wasm_bindgen::JsValue::from_str(
                "RSpice browser print document did not finish loading within 60 seconds",
            ));
            close_or_log(&timeout_window, "closing timed-out browser print window");
            revoke_or_log(&timeout_url, "revoking timed-out print document URL");
        });
        if let Err(error) = host.set_timeout_with_callback_and_timeout_and_arguments_0(
            on_timeout.unchecked_ref(),
            60_000,
        ) {
            lifecycle_complete.set(true);
            close_or_log(&popup, "closing print window after watchdog failure");
            revoke_or_log(
                &blob_url,
                "revoking print document URL after watchdog failure",
            );
            return Err(browser_error("installing print navigation watchdog", error));
        }

        let navigation_id = uuid::Uuid::new_v4().to_string();
        Ok(HardcopyOutcome::BrowserPrintNavigationAccepted {
            navigation_id,
            pages_accepted: publication.page_count(),
            source_artifact_digest: publication.digest(),
        })
    }

    fn print_loaded_document(window: &web_sys::Window, url: &str, complete: &Cell<bool>) {
        if complete.replace(true) {
            return;
        }
        let print_result = window
            .focus()
            .map_err(|error| ("focusing print window", error))
            .and_then(|()| {
                window
                    .print()
                    .map_err(|error| ("opening browser print dialog", error))
            });
        if let Err((operation, error)) = print_result {
            log_browser_exception(operation, error);
            close_or_log(window, "closing print window after print failure");
        }
        revoke_or_log(url, "revoking loaded print document URL");
    }

    fn close_or_log(window: &web_sys::Window, operation: &'static str) {
        if let Err(error) = window.close() {
            log_browser_exception(operation, error);
        }
    }

    fn revoke_or_log(url: &str, operation: &'static str) {
        if let Err(error) = Url::revoke_object_url(url) {
            log_browser_exception(operation, error);
        }
    }

    fn log_browser_exception(operation: &'static str, error: wasm_bindgen::JsValue) {
        web_sys::console::error_1(&wasm_bindgen::JsValue::from_str(
            &browser_error(operation, error).to_string(),
        ));
    }

    fn browser_error(operation: &'static str, error: wasm_bindgen::JsValue) -> HardcopyPrintError {
        HardcopyPrintError::Browser {
            operation,
            message: error
                .as_string()
                .unwrap_or_else(|| "browser API returned a non-text exception".to_owned()),
        }
    }
}

#[cfg(test)]
mod tests {
    use std::cell::RefCell;
    use std::rc::Rc;

    use uuid::Uuid;

    use super::*;
    use crate::hardcopy::{
        ActiveHardcopySource, BackgroundMode, Bleed, ColorMapping, ContentExtent, DecorationSetup,
        FontPolicy, HardcopyDocumentId, HardcopyDocumentKind, HardcopyPlanId, HardcopyScope,
        HardcopySetup, Length, Orientation, PageMargins, PaperSize, PhysicalPageSetup,
        PrintMappingTable, PrinterRasterGeometry, RenderSetup, ScaleMode, StandardPaper,
        TilingMode, TilingSetup, Watermark,
    };
    use crate::product::ObjectRevision;
    use crate::workbench::hardcopy_adapters::render::{
        HardcopyRenderer, HardcopyScene, HardcopySceneMetadata,
    };

    fn digest(value: u8) -> ContentDigest {
        ContentDigest::from_bytes([value; 32])
    }

    fn capability_descriptor() -> PrinterCapabilityDescriptor {
        PrinterCapabilityDescriptor {
            device_id: "engineering-printer-04".to_owned(),
            display_name: "Engineering Printer 04".to_owned(),
            driver_name: "RSpice deterministic test driver".to_owned(),
            port_name: "TESTPORT:".to_owned(),
            driver_spec_version: 1,
            driver_version: 42,
            papers: vec![
                PrinterPaperCapability::try_new(5, "Legal", 215_900, 355_600).unwrap(),
                PrinterPaperCapability::try_new(1, "Letter", 215_900, 279_400).unwrap(),
                PrinterPaperCapability::try_new(
                    8,
                    "Letter-compatible engineering stock",
                    215_900,
                    279_400,
                )
                .unwrap(),
            ],
            trays: vec![
                PrinterTrayCapability::try_new(4, "Manual feed").unwrap(),
                PrinterTrayCapability::try_new(1, "Upper tray").unwrap(),
            ],
            resolutions: vec![
                PrinterResolutionCapability::try_new(600, 600).unwrap(),
                PrinterResolutionCapability::try_new(300, 300).unwrap(),
                PrinterResolutionCapability::try_new(72, 72).unwrap(),
            ],
            duplex_modes: vec![DuplexMode::ShortEdge, DuplexMode::Off, DuplexMode::LongEdge],
            maximum_copies: 99,
            supports_collation: true,
            supports_color: true,
        }
    }

    fn source() -> ActiveHardcopySource {
        ActiveHardcopySource::try_new(
            HardcopyDocumentId::try_from_uuid(Uuid::from_u128(1)).unwrap(),
            ObjectRevision::INITIAL,
            digest(0x11),
            "top · schematic",
            HardcopyDocumentKind::SchematicOrSymbol,
            HardcopyScope::CurrentSheet,
        )
        .unwrap()
    }

    fn raster_geometry(dpi: u16) -> PrinterRasterGeometry {
        let pixels = |micrometres: u64| {
            u32::try_from((u128::from(micrometres) * u128::from(dpi)).div_ceil(25_400)).unwrap()
        };
        let width = pixels(279_400);
        let height = pixels(215_900);
        PrinterRasterGeometry::try_new(width, height, 0, 0, width, height).unwrap()
    }

    fn native_plan(
        capabilities: &PrinterCapabilitySnapshot,
        media: PrinterMediaSource,
        dpi: u16,
        duplex: DuplexMode,
        copies: u16,
        collate: bool,
    ) -> HardcopyPlan {
        native_plan_with_layout(
            capabilities,
            media,
            dpi,
            duplex,
            copies,
            collate,
            ContentExtent::try_new(
                Length::from_micrometres(100_000),
                Length::from_micrometres(60_000),
            )
            .unwrap(),
            ScaleMode::FitPrintableArea,
            TilingMode::SinglePage,
        )
    }

    #[allow(clippy::too_many_arguments)]
    fn native_plan_with_layout(
        capabilities: &PrinterCapabilitySnapshot,
        media: PrinterMediaSource,
        dpi: u16,
        duplex: DuplexMode,
        copies: u16,
        collate: bool,
        content_extent: ContentExtent,
        scale: ScaleMode,
        tiling: TilingMode,
    ) -> HardcopyPlan {
        let setup = HardcopySetup::try_new(
            PhysicalPageSetup::try_new(
                PaperSize::Standard(StandardPaper::Letter),
                PageMargins::uniform(Length::from_micrometres(10_000)),
                Bleed::None,
                Orientation::Landscape,
            )
            .unwrap(),
            scale,
            TilingSetup::try_new(tiling, Length::ZERO, false).unwrap(),
            RenderSetup::try_new(
                RenderTarget::SystemPrinter {
                    printer_id: capabilities.device_id().to_owned(),
                    job: PrinterJobSettings::try_new(
                        capabilities.content_digest(),
                        "1",
                        raster_geometry(dpi),
                        media,
                        dpi,
                        duplex,
                        copies,
                        collate,
                    )
                    .unwrap(),
                },
                OutputFormat::NativePrinter,
                ColorMapping::PrintSafeEngineeringPalette,
                BackgroundMode::White,
                FontPolicy::new(true, false),
                true,
            )
            .unwrap(),
            DecorationSetup::try_new(false, false, false, Watermark::None).unwrap(),
            PrintMappingTable::default(),
        )
        .unwrap();
        HardcopyPlan::compile_with_id(
            HardcopyPlanId::try_from_uuid(Uuid::from_u128(2)).unwrap(),
            source(),
            setup,
            content_extent,
        )
        .unwrap()
    }

    fn rendered_pages(plan: &HardcopyPlan, dpi: u16) -> RenderedPrinterPages {
        let metadata = HardcopySceneMetadata::try_new("test", "RSpice").unwrap();
        let scene = HardcopyScene::try_new(plan.content_extent(), metadata, Vec::new(), Vec::new())
            .unwrap();
        HardcopyRenderer::render_printer_pages(plan, &scene, dpi).unwrap()
    }

    fn native_plan_for_exact_mode(
        capabilities: &PrinterCapabilitySnapshot,
        selected_paper_id: &str,
        geometry: PrinterRasterGeometry,
        dpi: u16,
    ) -> HardcopyPlan {
        let setup = HardcopySetup::try_new(
            PhysicalPageSetup::try_new(
                PaperSize::Standard(StandardPaper::Letter),
                PageMargins::uniform(Length::from_micrometres(10_000)),
                Bleed::None,
                Orientation::Landscape,
            )
            .unwrap(),
            ScaleMode::FitPrintableArea,
            TilingSetup::try_new(TilingMode::SinglePage, Length::ZERO, false).unwrap(),
            RenderSetup::try_new(
                RenderTarget::SystemPrinter {
                    printer_id: capabilities.device_id().to_owned(),
                    job: PrinterJobSettings::try_new(
                        capabilities.content_digest(),
                        selected_paper_id,
                        geometry,
                        PrinterMediaSource::AutomaticCompatibleTray,
                        dpi,
                        DuplexMode::Off,
                        1,
                        false,
                    )
                    .unwrap(),
                },
                OutputFormat::NativePrinter,
                ColorMapping::PrintSafeEngineeringPalette,
                BackgroundMode::White,
                FontPolicy::new(true, false),
                true,
            )
            .unwrap(),
            DecorationSetup::try_new(false, false, false, Watermark::None).unwrap(),
            PrintMappingTable::default(),
        )
        .unwrap();
        HardcopyPlan::compile_with_id(
            HardcopyPlanId::try_from_uuid(Uuid::from_u128(3)).unwrap(),
            source(),
            setup,
            ContentExtent::try_new(
                Length::from_micrometres(100_000),
                Length::from_micrometres(60_000),
            )
            .unwrap(),
        )
        .unwrap()
    }

    #[test]
    fn capability_digest_is_order_independent_and_changes_with_real_capabilities() {
        let first =
            PrinterCapabilitySnapshot::try_from_descriptor(capability_descriptor()).unwrap();
        let mut reordered = capability_descriptor();
        reordered.papers.reverse();
        reordered.trays.reverse();
        reordered.resolutions.reverse();
        reordered.duplex_modes.reverse();
        let second = PrinterCapabilitySnapshot::try_from_descriptor(reordered).unwrap();
        assert_eq!(first, second);

        let mut changed = capability_descriptor();
        changed.maximum_copies = 12;
        let changed = PrinterCapabilitySnapshot::try_from_descriptor(changed).unwrap();
        assert_ne!(first.content_digest(), changed.content_digest());
    }

    #[test]
    fn capability_deserialization_revalidates_the_digest() {
        let snapshot =
            PrinterCapabilitySnapshot::try_from_descriptor(capability_descriptor()).unwrap();
        let mut encoded = serde_json::to_value(&snapshot).unwrap();
        encoded["maximum_copies"] = serde_json::json!(7);
        assert!(serde_json::from_value::<PrinterCapabilitySnapshot>(encoded).is_err());
    }

    #[test]
    fn failure_outcomes_always_fit_the_receipt_contract() {
        let error = HardcopyPrintError::InvalidCapabilitySnapshot("é".repeat(2_000));
        let HardcopyOutcome::Failed { message, .. } = error.failure_outcome(0) else {
            panic!("failure adapter must produce a failed outcome");
        };
        assert!(message.len() <= MAX_RECEIPT_MESSAGE_BYTES);
        assert!(message.ends_with("..."));
    }

    #[test]
    fn exact_plan_settings_resolve_without_substitution() {
        let capabilities =
            PrinterCapabilitySnapshot::try_from_descriptor(capability_descriptor()).unwrap();
        let plan = native_plan(
            &capabilities,
            PrinterMediaSource::NamedTray("Upper tray".to_owned()),
            600,
            DuplexMode::LongEdge,
            3,
            true,
        );
        let resolved = resolve_native_printer_job(&plan, &capabilities).unwrap();
        assert_eq!(resolved.paper_platform_id(), 1);
        assert_eq!(resolved.source_platform_id(), Some(1));
        assert_eq!(resolved.resolution_dpi(), 600);
        assert_eq!(resolved.orientation(), ResolvedOrientation::Landscape);
        assert_eq!(resolved.duplex(), DuplexMode::LongEdge);
        assert_eq!(resolved.copies(), 3);
        assert!(resolved.collate());
    }

    #[test]
    fn paper_resolution_uses_the_selected_identity_not_same_size_fallback() {
        let capabilities =
            PrinterCapabilitySnapshot::try_from_descriptor(capability_descriptor()).unwrap();
        let exact = native_plan_for_exact_mode(&capabilities, "8", raster_geometry(600), 600);
        assert_eq!(
            resolve_native_printer_job(&exact, &capabilities)
                .unwrap()
                .paper_platform_id(),
            8
        );

        let unavailable =
            native_plan_for_exact_mode(&capabilities, "999", raster_geometry(600), 600);
        assert_eq!(
            resolve_native_printer_job(&unavailable, &capabilities),
            Err(HardcopyPrintError::UnsupportedPaperIdentity(
                "999".to_owned()
            ))
        );
    }

    #[test]
    fn device_printable_rectangle_must_contain_all_planned_ink() {
        let capabilities =
            PrinterCapabilitySnapshot::try_from_descriptor(capability_descriptor()).unwrap();
        let full = raster_geometry(600);
        let (width, height) = full.physical_size_px();
        let clipped =
            PrinterRasterGeometry::try_new(width, height, 300, 300, width - 600, height - 600)
                .unwrap();
        let plan = native_plan_for_exact_mode(&capabilities, "1", clipped, 600);
        assert_eq!(
            resolve_native_printer_job(&plan, &capabilities),
            Err(HardcopyPrintError::PlannedInkOutsidePrintableArea)
        );
    }

    #[test]
    fn stale_digest_and_unsupported_options_fail_closed() {
        let capabilities =
            PrinterCapabilitySnapshot::try_from_descriptor(capability_descriptor()).unwrap();
        let plan = native_plan(
            &capabilities,
            PrinterMediaSource::AutomaticCompatibleTray,
            600,
            DuplexMode::Off,
            1,
            false,
        );
        let mut changed = capability_descriptor();
        changed.maximum_copies = 1;
        let changed = PrinterCapabilitySnapshot::try_from_descriptor(changed).unwrap();
        assert_eq!(
            resolve_native_printer_job(&plan, &changed),
            Err(HardcopyPrintError::PrinterCapabilitiesChanged)
        );

        let unsupported = native_plan(
            &capabilities,
            PrinterMediaSource::NamedTray("Missing tray".to_owned()),
            600,
            DuplexMode::Off,
            1,
            false,
        );
        assert!(matches!(
            resolve_native_printer_job(&unsupported, &capabilities),
            Err(HardcopyPrintError::UnsupportedMediaSource(_))
        ));
    }

    #[derive(Default)]
    struct FakeSpoolState {
        events: Vec<&'static str>,
        pages_written: u32,
        fail_on_page: Option<u32>,
        cancel_after_page: Option<u32>,
        abort_fails: bool,
    }

    struct FakeSpoolBackend {
        state: Rc<RefCell<FakeSpoolState>>,
        cancellation: HardcopyCancellationToken,
    }

    impl NativeSpoolBackend for FakeSpoolBackend {
        fn start_job(&mut self) -> Result<String, HardcopyPrintError> {
            self.state.borrow_mut().events.push("start-job");
            Ok("job-17".to_owned())
        }

        fn start_page(&mut self) -> Result<(), HardcopyPrintError> {
            self.state.borrow_mut().events.push("start-page");
            Ok(())
        }

        fn write_page(
            &mut self,
            _page: &crate::workbench::hardcopy_adapters::render::PrinterRasterPage,
        ) -> Result<(), HardcopyPrintError> {
            let mut state = self.state.borrow_mut();
            state.pages_written += 1;
            state.events.push("write-page");
            if state.fail_on_page == Some(state.pages_written) {
                return Err(HardcopyPrintError::DriverRejected {
                    operation: "fake raster write",
                });
            }
            Ok(())
        }

        fn end_page(&mut self) -> Result<(), HardcopyPrintError> {
            let mut state = self.state.borrow_mut();
            state.events.push("end-page");
            if state.cancel_after_page == Some(state.pages_written) {
                self.cancellation.cancel();
            }
            Ok(())
        }

        fn finish_job(&mut self) -> Result<(), HardcopyPrintError> {
            self.state.borrow_mut().events.push("finish");
            Ok(())
        }

        fn abort_job(&mut self) -> Result<(), HardcopyPrintError> {
            let mut state = self.state.borrow_mut();
            state.events.push("abort");
            if state.abort_fails {
                Err(HardcopyPrintError::DriverRejected {
                    operation: "fake abort",
                })
            } else {
                Ok(())
            }
        }
    }

    #[test]
    fn spool_transaction_reports_acceptance_only_after_complete_commit() {
        let capabilities =
            PrinterCapabilitySnapshot::try_from_descriptor(capability_descriptor()).unwrap();
        let plan = native_plan(
            &capabilities,
            PrinterMediaSource::AutomaticCompatibleTray,
            72,
            DuplexMode::Off,
            1,
            false,
        );
        let pages = rendered_pages(&plan, 72);
        let state = Rc::new(RefCell::new(FakeSpoolState::default()));
        let cancellation = HardcopyCancellationToken::default();
        let mut backend = FakeSpoolBackend {
            state: state.clone(),
            cancellation: cancellation.clone(),
        };
        let outcome = spool_transaction(
            &mut backend,
            &pages,
            capabilities.device_id(),
            &cancellation,
        )
        .unwrap();
        assert!(matches!(
            outcome,
            HardcopyOutcome::SpoolAccepted {
                pages_accepted: 1,
                ..
            }
        ));
        assert_eq!(state.borrow().events.last(), Some(&"finish"));
        assert!(!state.borrow().events.contains(&"abort"));
    }

    #[test]
    fn spool_transaction_aborts_on_write_failure_and_cancellation() {
        let capabilities =
            PrinterCapabilitySnapshot::try_from_descriptor(capability_descriptor()).unwrap();
        let plan = native_plan(
            &capabilities,
            PrinterMediaSource::AutomaticCompatibleTray,
            72,
            DuplexMode::Off,
            1,
            false,
        );
        let pages = rendered_pages(&plan, 72);

        let failed_state = Rc::new(RefCell::new(FakeSpoolState {
            fail_on_page: Some(1),
            ..Default::default()
        }));
        let token = HardcopyCancellationToken::default();
        let mut failed = FakeSpoolBackend {
            state: failed_state.clone(),
            cancellation: token.clone(),
        };
        let failure = spool_transaction(&mut failed, &pages, "printer", &token).unwrap_err();
        assert_eq!(failure.pages_completed(), 0);
        assert_eq!(failed_state.borrow().events.last(), Some(&"abort"));

        let tiled_plan = native_plan_with_layout(
            &capabilities,
            PrinterMediaSource::AutomaticCompatibleTray,
            72,
            DuplexMode::Off,
            1,
            false,
            ContentExtent::try_new(
                Length::from_micrometres(600_000),
                Length::from_micrometres(100_000),
            )
            .unwrap(),
            ScaleMode::EngineeringOneToOne,
            TilingMode::Automatic,
        );
        let tiled_pages = rendered_pages(&tiled_plan, 72);
        assert!(tiled_pages.pages().len() > 1);
        let partial_state = Rc::new(RefCell::new(FakeSpoolState {
            fail_on_page: Some(2),
            ..Default::default()
        }));
        let token = HardcopyCancellationToken::default();
        let mut partial = FakeSpoolBackend {
            state: partial_state.clone(),
            cancellation: token.clone(),
        };
        let failure = spool_transaction(&mut partial, &tiled_pages, "printer", &token).unwrap_err();
        assert_eq!(failure.pages_completed(), 1);
        assert!(matches!(
            failure.failure_outcome(),
            HardcopyOutcome::Failed {
                pages_completed: 1,
                ..
            }
        ));
        assert_eq!(partial_state.borrow().events.last(), Some(&"abort"));

        let cancelled_state = Rc::new(RefCell::new(FakeSpoolState {
            cancel_after_page: Some(1),
            ..Default::default()
        }));
        let token = HardcopyCancellationToken::default();
        let mut cancelled = FakeSpoolBackend {
            state: cancelled_state.clone(),
            cancellation: token.clone(),
        };
        let outcome = spool_transaction(&mut cancelled, &pages, "printer", &token).unwrap();
        assert!(matches!(
            outcome,
            HardcopyOutcome::Cancelled {
                pages_completed: 1,
                ..
            }
        ));
        assert_eq!(cancelled_state.borrow().events.last(), Some(&"abort"));

        let cancelled_cleanup_state = Rc::new(RefCell::new(FakeSpoolState {
            cancel_after_page: Some(1),
            abort_fails: true,
            ..Default::default()
        }));
        let token = HardcopyCancellationToken::default();
        let mut cancelled_cleanup = FakeSpoolBackend {
            state: cancelled_cleanup_state,
            cancellation: token.clone(),
        };
        assert!(matches!(
            spool_transaction(&mut cancelled_cleanup, &pages, "printer", &token),
            Err(HardcopySpoolFailure {
                error: HardcopyPrintError::SpoolCleanupFailed { .. },
                pages_completed: 1,
            })
        ));

        let cleanup_state = Rc::new(RefCell::new(FakeSpoolState {
            fail_on_page: Some(1),
            abort_fails: true,
            ..Default::default()
        }));
        let token = HardcopyCancellationToken::default();
        let mut cleanup_failed = FakeSpoolBackend {
            state: cleanup_state,
            cancellation: token.clone(),
        };
        assert!(matches!(
            spool_transaction(&mut cleanup_failed, &pages, "printer", &token),
            Err(HardcopySpoolFailure {
                error: HardcopyPrintError::SpoolCleanupFailed { .. },
                pages_completed: 0,
            })
        ));
    }

    #[test]
    fn publication_preflight_rejects_plan_dpi_mismatch() {
        let capabilities =
            PrinterCapabilitySnapshot::try_from_descriptor(capability_descriptor()).unwrap();
        let plan = native_plan(
            &capabilities,
            PrinterMediaSource::AutomaticCompatibleTray,
            72,
            DuplexMode::Off,
            1,
            false,
        );
        let pages = rendered_pages(&plan, 72);
        let mut resolved = resolve_native_printer_job(&plan, &capabilities).unwrap();
        resolved.resolution_dpi = 600;
        assert!(matches!(
            validate_printer_publication(&plan, &pages, &resolved),
            Err(HardcopyPrintError::PrinterPublicationMismatch(_))
        ));
    }
}
