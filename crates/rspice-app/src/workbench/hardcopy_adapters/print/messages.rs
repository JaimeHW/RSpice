//! The words a print error uses for the values it names.

use super::{DuplexMode, HardcopyPlatformUnavailableReason};

/// The reason a print route is unavailable, as the error message says it.
pub(super) const fn platform_unavailable_text(
    reason: &HardcopyPlatformUnavailableReason,
) -> &'static str {
    match reason {
        HardcopyPlatformUnavailableReason::NativePrintingIsWindowsOnly => {
            "system printers are supported on Windows only"
        }
        HardcopyPlatformUnavailableReason::BrowserPrintingRequiresWebAssembly => {
            "browser printing is available in the web app only"
        }
        HardcopyPlatformUnavailableReason::DesktopPrintHandoffRequiresMacOsOrLinux => {
            "the desktop print handoff is available on macOS and Linux only"
        }
        HardcopyPlatformUnavailableReason::BrowserWindowApiUnavailable => {
            "the browser window is not available"
        }
        HardcopyPlatformUnavailableReason::BrowserBlobApiUnavailable => {
            "the browser cannot create the print document"
        }
    }
}

/// A duplex mode as the error message names it.
pub(super) const fn duplex_mode_text(mode: &DuplexMode) -> &'static str {
    match mode {
        DuplexMode::Off => "single-sided",
        DuplexMode::LongEdge => "long-edge duplex",
        DuplexMode::ShortEdge => "short-edge duplex",
    }
}
