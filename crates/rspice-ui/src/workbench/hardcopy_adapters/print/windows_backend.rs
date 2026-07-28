//! The Windows spooler backend.
//!
//! The Win32 calls are wrapped so a spool is transactional from the caller's
//! side: a job that fails or is cancelled part-way is aborted at the spooler
//! rather than left as a partial document in the queue. Device capabilities
//! are read from the driver and reported as observed, never defaulted — an
//! unreported tray or resolution is absent rather than assumed.

use super::*;

use std::ffi::c_void;
use std::mem::size_of;
use std::ptr::{null, null_mut};

use windows_sys::Win32::Foundation::{GetLastError, HWND};
use windows_sys::Win32::Graphics::Gdi::{
    BI_RGB, BITMAPINFO, BITMAPINFOHEADER, CreateDCW, DEVMODEW, DIB_RGB_COLORS, DM_COLLATE,
    DM_COPIES, DM_DEFAULTSOURCE, DM_DUPLEX, DM_IN_BUFFER, DM_IN_PROMPT, DM_ORIENTATION,
    DM_OUT_BUFFER, DM_PAPERSIZE, DM_PRINTQUALITY, DM_YRESOLUTION, DMCOLLATE_FALSE, DMCOLLATE_TRUE,
    DMDUP_HORIZONTAL, DMDUP_SIMPLEX, DMDUP_VERTICAL, DMORIENT_LANDSCAPE, DMORIENT_PORTRAIT,
    DeleteDC, GetDeviceCaps, HDC, HORZRES, LOGPIXELSX, LOGPIXELSY, PHYSICALHEIGHT, PHYSICALOFFSETX,
    PHYSICALOFFSETY, PHYSICALWIDTH, SetDIBitsToDevice, VERTRES,
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
    let paper_id = selected_paper_id
        .parse::<i16>()
        .map_err(|_| HardcopyPrintError::UnsupportedPaperIdentity(selected_paper_id.to_owned()))?;
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
    let title = wide(&format!("RSpice — {}", plan.source().display_name())).map_err(no_progress)?;
    let mut backend = WindowsGdiSpoolBackend::open(&printer, &job, title).map_err(no_progress)?;
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
    let port_name = string_from_wide_in_buffer(info.pPortName, &info_buffer, CAPABILITY_TEXT_CAP)?;
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
    let byte_len =
        unsafe { DocumentPropertiesW(null_mut(), handle, name.as_ptr(), null_mut(), null(), 0) };
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
    let maximum_copies = device_capability_scalar(
        device_wide,
        port_wide,
        DC_COPIES,
        devmode.as_ptr(),
    )?
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

fn validate_driver_devmode(printer: &mut ResolvedWindowsPrinter) -> Result<(), HardcopyPrintError> {
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
    if mode.dmFields & (DM_DUPLEX | DM_COPIES | DM_COLLATE) != (DM_DUPLEX | DM_COPIES | DM_COLLATE)
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
            std::slice::from_raw_parts(units.as_ptr().cast::<u8>(), std::mem::size_of_val(&units))
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
