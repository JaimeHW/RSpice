//! The CUPS print backend of macOS and Linux.
//!
//! The queues are what `lpstat -e` lists, the system default what `lpstat -d`
//! names, and each queue's papers, trays, sides and resolutions what
//! `lpoptions -p <queue> -l` offers. A job is the plan's printer pages as a
//! PDF, one full-page image a page, handed to the queue with `lp`; CUPS copies
//! the file as it accepts the job, and its request id is the receipt's job.
//!
//! CUPS names no hardware margin through these tools, so a page's printable
//! area is the whole sheet and the plan's own margins keep ink off the edge.
//!
//! Reading the tools' output and writing the PDF are plain functions, tested
//! on every host against captured output; running the tools is the macOS and
//! Linux backend's alone.

use super::{
    DuplexMode, HardcopyCancellationToken, HardcopyPrintError, HardcopySpoolFailure,
    HardcopySpoolResult, NativeSpoolBackend, PrinterCapabilityDescriptor,
    PrinterCapabilitySnapshot, PrinterPaperCapability, PrinterResolutionCapability,
    PrinterTrayCapability, resolve_native_printer_job, spool_transaction,
    validate_printer_publication,
};
use rspice_hardcopy::render::{PrinterRasterPage, RenderedPrinterPages};
use rspice_hardcopy_contract::{
    HardcopyPlan, PrinterRasterGeometry, RenderTarget, ResolvedOrientation,
};

/// The resolution a queue that names none is drawn at.
const FALLBACK_DPI: u16 = 600;
/// Most copies one job asks for: the capability contract's own ceiling.
const MAXIMUM_COPIES: u16 = 999;
/// The platform id a manual-feed tray carries, as the print contract reads
/// manual feed.
const MANUAL_TRAY_ID: i16 = 4;
/// The first platform id of any other tray, clear of the manual ones.
const FIRST_TRAY_ID: i16 = 100;

/// One option `lpoptions -l` lists: its keyword, its choices and the one the
/// queue defaults to.
#[derive(Debug, Clone, PartialEq, Eq)]
pub(super) struct CupsOption {
    pub keyword: String,
    pub choices: Vec<String>,
    pub default: Option<String>,
}

/// The options `lpoptions -l` lists, one a line:
/// `PageSize/Media Size: *Letter Legal A4`, the default starred.
pub(super) fn parse_options(output: &str) -> Vec<CupsOption> {
    output
        .lines()
        .filter_map(|line| {
            let (name, choices) = line.split_once(':')?;
            let keyword = name.split('/').next()?.trim();
            if keyword.is_empty() {
                return None;
            }
            let mut default = None;
            let choices = choices
                .split_whitespace()
                .map(|choice| match choice.strip_prefix('*') {
                    Some(starred) => {
                        default = Some(starred.to_owned());
                        starred.to_owned()
                    }
                    None => choice.to_owned(),
                })
                .collect();
            Some(CupsOption {
                keyword: keyword.to_owned(),
                choices,
                default,
            })
        })
        .collect()
}

/// The queues `lpstat -e` lists, one a line.
pub(super) fn parse_destinations(output: &str) -> Vec<String> {
    output
        .lines()
        .map(str::trim)
        .filter(|line| !line.is_empty())
        .map(str::to_owned)
        .collect()
}

/// The system default queue `lpstat -d` names, if any:
/// `system default destination: Office`.
pub(super) fn parse_default_destination(output: &str) -> Option<String> {
    output.lines().find_map(|line| {
        let (_, name) = line.split_once("destination:")?;
        let name = name.trim();
        (!name.is_empty()).then(|| name.to_owned())
    })
}

/// The request id `lp` reports for the job it accepted:
/// `request id is Office-42 (1 file(s))`.
pub(super) fn parse_request_id(output: &str) -> Option<String> {
    output.lines().find_map(|line| {
        let rest = line.trim().strip_prefix("request id is ")?;
        let id = rest.split_whitespace().next()?;
        (!id.is_empty()).then(|| id.to_owned())
    })
}

/// The portrait size of a CUPS page-size keyword in micrometres: the PPD
/// specification's standard names, PWG self-describing names
/// (`iso_a4_210x297mm`, `na_letter_8.5x11in`) and `w<width>h<height>` in
/// points. A named variant after a dot (`A4.Borderless`) is the size
/// before it.
pub(super) fn page_size_um(keyword: &str) -> Option<(u64, u64)> {
    let points = |width: f64, height: f64| {
        Some((
            (width * 25_400.0 / 72.0).round() as u64,
            (height * 25_400.0 / 72.0).round() as u64,
        ))
    };
    let base = match keyword.split_once('.') {
        Some((head, variant)) if variant.starts_with(|c: char| c.is_ascii_alphabetic()) => head,
        _ => keyword,
    };
    let standard = match base.to_ascii_lowercase().as_str() {
        "letter" => points(612.0, 792.0),
        "legal" => points(612.0, 1008.0),
        "executive" => points(522.0, 756.0),
        "statement" => points(396.0, 612.0),
        "tabloid" | "ledger" | "11x17" => points(792.0, 1224.0),
        "a5" => Some((148_000, 210_000)),
        "a4" => Some((210_000, 297_000)),
        "a3" => Some((297_000, 420_000)),
        "a2" => Some((420_000, 594_000)),
        "a1" => Some((594_000, 841_000)),
        "a0" => Some((841_000, 1_189_000)),
        "b5" => Some((182_000, 257_000)),
        "b4" => Some((257_000, 364_000)),
        "env10" => points(297.0, 684.0),
        "envdl" => Some((110_000, 220_000)),
        "envc5" => Some((162_000, 229_000)),
        _ => None,
    };
    if standard.is_some() {
        return standard;
    }
    if let Some(rest) = base.strip_prefix('w')
        && let Some((width, height)) = rest.split_once('h')
    {
        return points(width.parse().ok()?, height.parse().ok()?);
    }
    // PWG 5101.1: the last field is `<width>x<height><unit>`.
    let size = base.rsplit('_').next()?;
    let (scale, size) = if let Some(size) = size.strip_suffix("mm") {
        (1_000.0, size)
    } else {
        (25_400.0, size.strip_suffix("in")?)
    };
    let (width, height) = size.split_once('x')?;
    let (width, height): (f64, f64) = (width.parse().ok()?, height.parse().ok()?);
    (width > 0.0 && height > 0.0).then(|| {
        (
            (width * scale).round() as u64,
            (height * scale).round() as u64,
        )
    })
}

/// A resolution choice, `600dpi` or `1200x600dpi`.
fn parse_resolution(choice: &str) -> Option<(u16, u16)> {
    let value = choice.strip_suffix("dpi")?;
    match value.split_once('x') {
        Some((horizontal, vertical)) => Some((horizontal.parse().ok()?, vertical.parse().ok()?)),
        None => {
            let dpi = value.parse().ok()?;
            Some((dpi, dpi))
        }
    }
}

/// The sides a `Duplex` choice prints.
fn duplex_mode(choice: &str) -> Option<DuplexMode> {
    match choice {
        "None" => Some(DuplexMode::Off),
        "DuplexNoTumble" => Some(DuplexMode::LongEdge),
        "DuplexTumble" => Some(DuplexMode::ShortEdge),
        _ => None,
    }
}

/// The `sides` value `lp` takes for a duplex mode.
pub(super) const fn sides(mode: DuplexMode) -> &'static str {
    match mode {
        DuplexMode::Off => "one-sided",
        DuplexMode::LongEdge => "two-sided-long-edge",
        DuplexMode::ShortEdge => "two-sided-short-edge",
    }
}

/// Whether an `InputSlot` choice is the manual feed.
fn is_manual_feed(choice: &str) -> bool {
    choice.to_ascii_lowercase().contains("manual")
}

/// What a queue offers, from its `lpoptions -l`, as the print contract
/// describes a printer; with the platform id of the paper it defaults to.
/// Papers are numbered from 1 in the order the queue lists them, and a
/// page-size keyword whose size is not known is left out.
pub(super) fn describe_queue(
    queue: &str,
    options: &[CupsOption],
) -> Result<(PrinterCapabilityDescriptor, Option<i16>), HardcopyPrintError> {
    let option = |keyword: &str| options.iter().find(|option| option.keyword == keyword);
    let mut papers = Vec::new();
    let mut default_paper = None;
    if let Some(page_size) = option("PageSize") {
        for choice in &page_size.choices {
            let Some((width, height)) = page_size_um(choice) else {
                continue;
            };
            let id = i16::try_from(papers.len() + 1).map_err(|_| {
                HardcopyPrintError::InvalidCapabilitySnapshot(
                    "the queue lists more papers than a printer can".to_owned(),
                )
            })?;
            if page_size.default.as_deref() == Some(choice.as_str()) {
                default_paper = Some(id);
            }
            papers.push(PrinterPaperCapability::try_new(id, choice, width, height)?);
        }
    }
    let mut trays = Vec::new();
    if let Some(input_slot) = option("InputSlot") {
        let mut next = FIRST_TRAY_ID;
        let mut manual = false;
        for choice in &input_slot.choices {
            let id = if is_manual_feed(choice) && !manual {
                manual = true;
                MANUAL_TRAY_ID
            } else {
                next += 1;
                next - 1
            };
            trays.push(PrinterTrayCapability::try_new(id, choice)?);
        }
    }
    let mut resolutions = option("Resolution")
        .map(|resolution| {
            resolution
                .choices
                .iter()
                .filter_map(|choice| parse_resolution(choice))
                .filter_map(|(horizontal, vertical)| {
                    PrinterResolutionCapability::try_new(horizontal, vertical).ok()
                })
                .collect::<Vec<_>>()
        })
        .unwrap_or_default();
    if resolutions.is_empty() {
        resolutions.push(PrinterResolutionCapability::try_new(
            FALLBACK_DPI,
            FALLBACK_DPI,
        )?);
    }
    let duplex_modes = option("Duplex").map_or_else(
        || vec![DuplexMode::Off],
        |duplex| {
            duplex
                .choices
                .iter()
                .filter_map(|choice| duplex_mode(choice))
                .collect()
        },
    );
    let supports_color = option("ColorModel").is_none_or(|color| {
        color
            .choices
            .iter()
            .any(|choice| !choice.eq_ignore_ascii_case("gray") && !choice.contains("Gray"))
    });
    Ok((
        PrinterCapabilityDescriptor {
            device_id: queue.to_owned(),
            display_name: queue.to_owned(),
            driver_name: "CUPS".to_owned(),
            port_name: "CUPS".to_owned(),
            driver_spec_version: 0,
            driver_version: 0,
            papers,
            trays,
            resolutions,
            duplex_modes,
            maximum_copies: MAXIMUM_COPIES,
            supports_collation: true,
            supports_color,
        },
        default_paper,
    ))
}

/// The arguments `lp` submits a job with: the queue, the title, the copies,
/// the paper and tray keywords, the sides, and the file, printed as drawn.
pub(super) struct LpJob<'a> {
    pub queue: &'a str,
    pub title: &'a str,
    pub copies: u16,
    pub collate: bool,
    pub media: &'a str,
    pub input_slot: Option<&'a str>,
    pub duplex: DuplexMode,
    pub file: &'a std::path::Path,
}

impl LpJob<'_> {
    pub(super) fn arguments(&self) -> Vec<std::ffi::OsString> {
        let mut arguments: Vec<std::ffi::OsString> = vec![
            "-d".into(),
            self.queue.into(),
            "-t".into(),
            self.title.into(),
            "-n".into(),
            self.copies.to_string().into(),
            "-o".into(),
            format!("media={}", self.media).into(),
            "-o".into(),
            format!("sides={}", sides(self.duplex)).into(),
            "-o".into(),
            "print-scaling=none".into(),
        ];
        if self.copies > 1 {
            arguments.push("-o".into());
            arguments.push(format!("collate={}", self.collate).into());
        }
        if let Some(slot) = self.input_slot {
            arguments.push("-o".into());
            arguments.push(format!("InputSlot={slot}").into());
        }
        arguments.push("--".into());
        arguments.push(self.file.as_os_str().to_owned());
        arguments
    }
}

/// A PDF of full-page images, written a page at a time: each page is its
/// raster at the size it prints, the pixels RGB, run-length encoded so the
/// white of a drawing costs next to nothing.
#[derive(Default)]
pub(super) struct RasterPdf {
    pages: Vec<RasterPdfPage>,
}

/// One page: its size in points and its encoded image.
struct RasterPdfPage {
    width_pt: f64,
    height_pt: f64,
    width_px: u32,
    height_px: u32,
    image: Vec<u8>,
}

impl RasterPdf {
    /// Add a `width` × `height` RGBA page drawn at `dpi`.
    pub(super) fn add_page(&mut self, width: u32, height: u32, dpi: u16, rgba: &[u8]) {
        let points = |pixels: u32| f64::from(pixels) * 72.0 / f64::from(dpi);
        let mut rgb = Vec::with_capacity(rgba.len() / 4 * 3);
        for pixel in rgba.chunks_exact(4) {
            rgb.extend_from_slice(&pixel[..3]);
        }
        self.pages.push(RasterPdfPage {
            width_pt: points(width),
            height_pt: points(height),
            width_px: width,
            height_px: height,
            image: run_length_encode(&rgb),
        });
    }

    /// The document: a catalog, the page tree, then for each page its page
    /// object, its content stream and its image, then the cross-reference
    /// table.
    #[must_use]
    pub(super) fn finish(self) -> Vec<u8> {
        let mut out = b"%PDF-1.4\n%\xE2\xE3\xCF\xD3\n".to_vec();
        let mut offsets = Vec::new();
        let mut object = |out: &mut Vec<u8>, body: &[u8]| {
            offsets.push(out.len());
            out.extend_from_slice(format!("{} 0 obj\n", offsets.len()).as_bytes());
            out.extend_from_slice(body);
            out.extend_from_slice(b"\nendobj\n");
        };
        let count = self.pages.len();
        let kids = (0..count)
            .map(|index| format!("{} 0 R", 3 + index * 3))
            .collect::<Vec<_>>()
            .join(" ");
        object(&mut out, b"<< /Type /Catalog /Pages 2 0 R >>");
        object(
            &mut out,
            format!("<< /Type /Pages /Kids [{kids}] /Count {count} >>").as_bytes(),
        );
        for (index, page) in self.pages.iter().enumerate() {
            let (content_id, image_id) = (4 + index * 3, 5 + index * 3);
            let (width, height) = (page.width_pt, page.height_pt);
            object(
                &mut out,
                format!(
                    "<< /Type /Page /Parent 2 0 R /MediaBox [0 0 {width:.3} {height:.3}] \
                     /Resources << /XObject << /Page {image_id} 0 R >> >> \
                     /Contents {content_id} 0 R >>"
                )
                .as_bytes(),
            );
            let content = format!("q {width:.3} 0 0 {height:.3} 0 0 cm /Page Do Q");
            object(
                &mut out,
                format!(
                    "<< /Length {} >>\nstream\n{content}\nendstream",
                    content.len()
                )
                .as_bytes(),
            );
            let mut body = format!(
                "<< /Type /XObject /Subtype /Image /Width {} /Height {} \
                 /ColorSpace /DeviceRGB /BitsPerComponent 8 /Filter /RunLengthDecode \
                 /Length {} >>\nstream\n",
                page.width_px,
                page.height_px,
                page.image.len()
            )
            .into_bytes();
            body.extend_from_slice(&page.image);
            body.extend_from_slice(b"\nendstream");
            object(&mut out, &body);
        }
        let xref = out.len();
        out.extend_from_slice(
            format!("xref\n0 {}\n0000000000 65535 f \n", offsets.len() + 1).as_bytes(),
        );
        for offset in &offsets {
            out.extend_from_slice(format!("{offset:010} 00000 n \n").as_bytes());
        }
        out.extend_from_slice(
            format!(
                "trailer\n<< /Size {} /Root 1 0 R >>\nstartxref\n{xref}\n%%EOF\n",
                offsets.len() + 1
            )
            .as_bytes(),
        );
        out
    }
}

/// `bytes` in the PDF `RunLengthDecode` encoding: a run of two or more equal
/// bytes as a length byte of `257 - n` and the byte, anything else copied
/// after a length byte of `n - 1`, at most 128 bytes a record, then the
/// end-of-data byte 128.
pub(super) fn run_length_encode(bytes: &[u8]) -> Vec<u8> {
    let mut out = Vec::with_capacity(bytes.len() / 32 + 2);
    let mut index = 0;
    while index < bytes.len() {
        let run = bytes[index..]
            .iter()
            .take(128)
            .take_while(|byte| **byte == bytes[index])
            .count();
        if run >= 2 {
            out.push((257 - run) as u8);
            out.push(bytes[index]);
            index += run;
            continue;
        }
        let start = index;
        while index < bytes.len()
            && index - start < 128
            && !(index + 1 < bytes.len() && bytes[index] == bytes[index + 1])
        {
            index += 1;
        }
        out.push((index - start - 1) as u8);
        out.extend_from_slice(&bytes[start..index]);
    }
    out.push(128);
    out
}

/// The selected paper at `resolution_dpi`, turned to `orientation`. CUPS
/// names no hardware margin, so the whole sheet is printable.
pub(super) fn resolve_native_printer_mode(
    capabilities: &PrinterCapabilitySnapshot,
    selected_paper_id: &str,
    resolution_dpi: u16,
    orientation: ResolvedOrientation,
) -> Result<PrinterRasterGeometry, HardcopyPrintError> {
    let paper_id = selected_paper_id
        .parse::<i16>()
        .map_err(|_| HardcopyPrintError::UnsupportedPaperIdentity(selected_paper_id.to_owned()))?;
    let paper = capabilities
        .papers
        .iter()
        .find(|paper| paper.platform_id == paper_id)
        .ok_or_else(|| {
            HardcopyPrintError::UnsupportedPaperIdentity(selected_paper_id.to_owned())
        })?;
    if !capabilities.resolutions.iter().any(|resolution| {
        resolution.horizontal_dpi == resolution_dpi && resolution.vertical_dpi == resolution_dpi
    }) {
        return Err(HardcopyPrintError::UnsupportedResolution(resolution_dpi));
    }
    let pixels = |micrometres: u64| {
        u32::try_from(micrometres * u64::from(resolution_dpi) / 25_400).unwrap_or(u32::MAX)
    };
    let (short, long) = (
        pixels(paper.portrait_width_um),
        pixels(paper.portrait_height_um),
    );
    let (width, height) = match orientation {
        ResolvedOrientation::Portrait => (short, long),
        ResolvedOrientation::Landscape => (long, short),
    };
    PrinterRasterGeometry::try_new(width, height, 0, 0, width, height)
        .map_err(|error| HardcopyPrintError::InvalidDriverGeometry(error.to_string()))
}

/// How a finished job reaches the queue: the `lp` call, given the job's
/// arguments, answering what `lp` printed.
pub(super) type Submit<'a> = &'a dyn Fn(&LpJob<'_>) -> Result<String, HardcopyPrintError>;

/// A job in the making: the pages gathered into a PDF, written to a private
/// folder under `folder` and handed to `submit` as the job is finished.
struct CupsSpool<'a> {
    queue: &'a str,
    title: String,
    copies: u16,
    collate: bool,
    media: String,
    input_slot: Option<String>,
    duplex: DuplexMode,
    folder: &'a std::path::Path,
    submit: Submit<'a>,
    pdf: RasterPdf,
    request_id: Option<String>,
}

impl NativeSpoolBackend for CupsSpool<'_> {
    fn start_job(&mut self) -> Result<String, HardcopyPrintError> {
        Ok(String::new())
    }

    fn start_page(&mut self) -> Result<(), HardcopyPrintError> {
        Ok(())
    }

    fn write_page(&mut self, page: &PrinterRasterPage) -> Result<(), HardcopyPrintError> {
        self.pdf
            .add_page(page.width(), page.height(), page.dpi(), page.rgba());
        Ok(())
    }

    fn end_page(&mut self) -> Result<(), HardcopyPrintError> {
        Ok(())
    }

    fn finish_job(&mut self) -> Result<(), HardcopyPrintError> {
        let io = |operation| {
            move |error: std::io::Error| HardcopyPrintError::Cups {
                operation,
                message: error.to_string(),
            }
        };
        let directory = self
            .folder
            .join(format!("rspice-print-{}", uuid::Uuid::new_v4().simple()));
        std::fs::create_dir_all(&directory).map_err(io("creating the print job's folder"))?;
        let file = directory.join("job.pdf");
        let submitted = std::fs::write(&file, std::mem::take(&mut self.pdf).finish())
            .map_err(io("writing the print job"))
            .and_then(|()| {
                (self.submit)(&LpJob {
                    queue: self.queue,
                    title: &self.title,
                    copies: self.copies,
                    collate: self.collate,
                    media: &self.media,
                    input_slot: self.input_slot.as_deref(),
                    duplex: self.duplex,
                    file: &file,
                })
            });
        // CUPS holds its own copy of a file it accepted.
        let _ = std::fs::remove_dir_all(&directory);
        self.request_id = parse_request_id(&submitted?);
        Ok(())
    }

    fn abort_job(&mut self) -> Result<(), HardcopyPrintError> {
        self.pdf = RasterPdf::default();
        Ok(())
    }

    fn accepted_job_id(&mut self) -> Option<String> {
        self.request_id.take()
    }
}

/// Print `pages` on the plan's queue: the job checked against the plan and
/// the queue's capabilities as the Windows spool checks it, the paper and
/// tray named by the queue's own keywords, the PDF written under `folder` and
/// handed to `submit`.
pub(super) fn spool_with(
    plan: &HardcopyPlan,
    pages: &RenderedPrinterPages,
    capabilities: &PrinterCapabilitySnapshot,
    cancellation: &HardcopyCancellationToken,
    folder: &std::path::Path,
    submit: Submit<'_>,
) -> HardcopySpoolResult {
    let no_progress = |error| HardcopySpoolFailure::new(error, 0);
    let job = resolve_native_printer_job(plan, capabilities).map_err(no_progress)?;
    validate_printer_publication(plan, pages, &job).map_err(no_progress)?;
    let RenderTarget::SystemPrinter { printer_id, .. } = plan.setup().render().target() else {
        return Err(no_progress(HardcopyPrintError::NativePrinterPlanRequired));
    };
    let media = capabilities
        .papers
        .iter()
        .find(|paper| paper.platform_id == job.paper_platform_id)
        .map(|paper| paper.display_name.clone())
        .ok_or_else(|| {
            no_progress(HardcopyPrintError::UnsupportedPaperIdentity(
                job.paper_platform_id.to_string(),
            ))
        })?;
    let input_slot = job.source_platform_id.and_then(|id| {
        capabilities
            .trays
            .iter()
            .find(|tray| tray.platform_id == id)
            .map(|tray| tray.display_name.clone())
    });
    let mut backend = CupsSpool {
        queue: printer_id,
        title: format!("{} (RSpice)", plan.source().display_name()),
        copies: job.copies,
        collate: job.collate,
        media,
        input_slot,
        duplex: job.duplex,
        folder,
        submit,
        pdf: RasterPdf::default(),
        request_id: None,
    };
    spool_transaction(&mut backend, pages, printer_id, cancellation)
}

#[cfg(any(target_os = "macos", target_os = "linux"))]
pub(super) use host::{discover_native_printers, spool_native_hardcopy};

/// Running the CUPS tools: the macOS and Linux half of the backend.
#[cfg(any(target_os = "macos", target_os = "linux"))]
mod host {
    use super::super::{
        HardcopyCancellationToken, HardcopySpoolResult, PrinterCapabilitySnapshot,
        PrinterCatalogEntry, PrinterDiscoveryFailure, PrinterDiscoveryReport,
    };
    use super::*;
    use rspice_hardcopy::render::RenderedPrinterPages;
    use rspice_hardcopy_contract::HardcopyPlan;

    /// Run a CUPS tool and keep what it printed, or say why it failed.
    fn run(
        operation: &'static str,
        program: &str,
        arguments: &[std::ffi::OsString],
    ) -> Result<String, HardcopyPrintError> {
        let output = std::process::Command::new(program)
            .args(arguments)
            .stdin(std::process::Stdio::null())
            .output()
            .map_err(|error| HardcopyPrintError::Cups {
                operation,
                message: error.to_string(),
            })?;
        if !output.status.success() {
            let stderr = String::from_utf8_lossy(&output.stderr);
            return Err(HardcopyPrintError::Cups {
                operation,
                message: format!("{program} exited with {}: {}", output.status, stderr.trim()),
            });
        }
        Ok(String::from_utf8_lossy(&output.stdout).into_owned())
    }

    fn describe(
        queue: &str,
    ) -> Result<(PrinterCapabilitySnapshot, Option<i16>), HardcopyPrintError> {
        let options = run(
            "reading the queue's options",
            "lpoptions",
            &["-p".into(), queue.into(), "-l".into()],
        )?;
        let (descriptor, default_paper) = describe_queue(queue, &parse_options(&options))?;
        Ok((
            PrinterCapabilitySnapshot::try_from_descriptor(descriptor)?,
            default_paper,
        ))
    }

    pub(in super::super) fn discover_native_printers()
    -> Result<PrinterDiscoveryReport, HardcopyPrintError> {
        let queues =
            parse_destinations(&run("listing the print queues", "lpstat", &["-e".into()])?);
        // A host with no default says so on stderr and exits non-zero.
        let default = run("naming the default queue", "lpstat", &["-d".into()])
            .ok()
            .and_then(|output| parse_default_destination(&output));
        let mut report = PrinterDiscoveryReport::default();
        for queue in queues {
            match describe(&queue) {
                Ok((capabilities, default_paper_platform_id)) => {
                    report.printers.push(PrinterCatalogEntry {
                        is_default: default.as_deref() == Some(queue.as_str()),
                        capabilities,
                        queue_status_flags: 0,
                        default_paper_platform_id,
                    });
                }
                Err(error) => report.failures.push(PrinterDiscoveryFailure {
                    device_id: queue,
                    message: error.to_string(),
                }),
            }
        }
        report.printers.sort_by(|left, right| {
            right.is_default.cmp(&left.is_default).then_with(|| {
                left.capabilities
                    .device_id
                    .cmp(&right.capabilities.device_id)
            })
        });
        Ok(report)
    }

    pub(in super::super) fn spool_native_hardcopy(
        plan: &HardcopyPlan,
        pages: &RenderedPrinterPages,
        capabilities: &PrinterCapabilitySnapshot,
        cancellation: &HardcopyCancellationToken,
    ) -> HardcopySpoolResult {
        spool_with(
            plan,
            pages,
            capabilities,
            cancellation,
            &std::env::temp_dir(),
            &|job| run("submitting the job", "lp", &job.arguments()),
        )
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    /// `lpoptions -p Office -l` for a PostScript laser printer's PPD.
    const LASER: &str = "PageSize/Media Size: Letter Legal Executive *A4 A5 Env10 EnvDL w288h432 Custom.WIDTHxHEIGHT\n\
InputSlot/Media Source: *Auto Tray1 Tray2 ManualFeed\n\
Duplex/2-Sided Printing: *None DuplexNoTumble DuplexTumble\n\
Resolution/Resolution: 300dpi *600dpi 1200x600dpi\n\
ColorModel/Color Mode: *Gray\n";

    /// `lpoptions -p Photo -l` for a driverless (IPP Everywhere) queue.
    const DRIVERLESS: &str = "PageSize/Media Size: *na_letter_8.5x11in iso_a4_210x297mm na_index-4x6_4x6in\n\
ColorModel/Output Mode: Gray *RGB\n";

    fn snapshot(queue: &str, options: &str) -> (PrinterCapabilitySnapshot, Option<i16>) {
        let (descriptor, default_paper) = describe_queue(queue, &parse_options(options)).unwrap();
        (
            PrinterCapabilitySnapshot::try_from_descriptor(descriptor).unwrap(),
            default_paper,
        )
    }

    /// Every option `lpoptions -l` lists is read with its choices and its
    /// starred default, whatever the text after the keyword says.
    #[test]
    fn lpoptions_lines_read_as_keyword_choices_and_default() {
        let options = parse_options(LASER);
        assert_eq!(options.len(), 5);
        assert_eq!(options[2].keyword, "Duplex");
        assert_eq!(
            options[2].choices,
            ["None", "DuplexNoTumble", "DuplexTumble"]
        );
        assert_eq!(options[0].default.as_deref(), Some("A4"));
        assert_eq!(options[3].default.as_deref(), Some("600dpi"));
    }

    /// A queue's papers keep its own keywords and their sizes, numbered in the
    /// order it lists them; the trays, sides and resolutions follow its
    /// options, and the manual feed is the tray the contract reads as one.
    #[test]
    fn a_laser_queue_describes_papers_trays_sides_and_resolutions() {
        let (capabilities, default_paper) = snapshot("Office", LASER);
        let papers = capabilities
            .papers()
            .iter()
            .map(|paper| {
                (
                    paper.platform_id(),
                    paper.display_name().to_owned(),
                    paper.portrait_dimensions_um(),
                )
            })
            .collect::<Vec<_>>();
        assert_eq!(
            papers,
            [
                (1, "Letter".to_owned(), (215_900, 279_400)),
                (2, "Legal".to_owned(), (215_900, 355_600)),
                (3, "Executive".to_owned(), (184_150, 266_700)),
                (4, "A4".to_owned(), (210_000, 297_000)),
                (5, "A5".to_owned(), (148_000, 210_000)),
                (6, "Env10".to_owned(), (104_775, 241_300)),
                (7, "EnvDL".to_owned(), (110_000, 220_000)),
                (8, "w288h432".to_owned(), (101_600, 152_400)),
            ]
        );
        assert_eq!(default_paper, Some(4));
        let trays = capabilities
            .trays()
            .iter()
            .map(|tray| (tray.platform_id(), tray.display_name()))
            .collect::<Vec<_>>();
        assert_eq!(
            trays,
            [
                (4, "ManualFeed"),
                (100, "Auto"),
                (101, "Tray1"),
                (102, "Tray2")
            ]
        );
        assert_eq!(
            capabilities.duplex_modes(),
            [DuplexMode::Off, DuplexMode::LongEdge, DuplexMode::ShortEdge]
        );
        let resolutions = capabilities
            .resolutions()
            .iter()
            .map(|resolution| (resolution.horizontal_dpi(), resolution.vertical_dpi()))
            .collect::<Vec<_>>();
        assert_eq!(resolutions, [(300, 300), (600, 600), (1200, 600)]);
        assert!(!capabilities.supports_color());
        assert_eq!(capabilities.device_id(), "Office");
    }

    /// A driverless queue names its papers the PWG way and lists no
    /// resolution, tray or sides: the sizes come from the names, the raster
    /// is drawn at 600 dpi, and it prints one side from its default tray.
    #[test]
    fn a_driverless_queue_reads_pwg_sizes_and_falls_back() {
        let (capabilities, default_paper) = snapshot("Photo", DRIVERLESS);
        let sizes = capabilities
            .papers()
            .iter()
            .map(PrinterPaperCapability::portrait_dimensions_um)
            .collect::<Vec<_>>();
        assert_eq!(
            sizes,
            [(215_900, 279_400), (210_000, 297_000), (101_600, 152_400)]
        );
        assert_eq!(default_paper, Some(1));
        assert!(capabilities.trays().is_empty());
        assert_eq!(capabilities.duplex_modes(), [DuplexMode::Off]);
        assert_eq!(capabilities.resolutions().len(), 1);
        assert_eq!(capabilities.resolutions()[0].horizontal_dpi(), 600);
        assert!(capabilities.supports_color());
    }

    /// A queue that lists no paper whose size is known is no printer.
    #[test]
    fn a_queue_without_a_known_paper_is_refused() {
        let options = parse_options("PageSize/Media Size: *Custom.WIDTHxHEIGHT\n");
        let (descriptor, _) = describe_queue("Plotter", &options).unwrap();
        assert!(PrinterCapabilitySnapshot::try_from_descriptor(descriptor).is_err());
    }

    /// `lpstat` and `lp` are read for the queues, the default and the job.
    #[test]
    fn lpstat_and_lp_output_name_queues_default_and_job() {
        assert_eq!(parse_destinations("Office\n\nPhoto\n"), ["Office", "Photo"]);
        assert_eq!(
            parse_default_destination("system default destination: Office\n").as_deref(),
            Some("Office")
        );
        assert_eq!(
            parse_default_destination("no system default destination\n"),
            None
        );
        assert_eq!(
            parse_request_id("request id is Office-42 (1 file(s))\n").as_deref(),
            Some("Office-42")
        );
        assert_eq!(
            parse_request_id("lp: Error - no default destination\n"),
            None
        );
    }

    /// The page is the selected paper at the job's resolution, turned to the
    /// page's orientation, printable to its edges; a resolution the queue
    /// does not list is refused.
    #[test]
    fn the_printer_mode_is_the_paper_at_the_resolution() {
        let (capabilities, _) = snapshot("Office", LASER);
        let geometry =
            resolve_native_printer_mode(&capabilities, "1", 300, ResolvedOrientation::Landscape)
                .unwrap();
        assert_eq!(geometry.physical_size_px(), (3_300, 2_550));
        assert_eq!(geometry.printable_rect_px(), (0, 0, 3_300, 2_550));
        assert!(matches!(
            resolve_native_printer_mode(&capabilities, "1", 72, ResolvedOrientation::Portrait),
            Err(HardcopyPrintError::UnsupportedResolution(72))
        ));
        assert!(matches!(
            resolve_native_printer_mode(&capabilities, "99", 300, ResolvedOrientation::Portrait),
            Err(HardcopyPrintError::UnsupportedPaperIdentity(_))
        ));
    }

    /// The run-length encoding is the PDF filter's: runs, literals, the
    /// 128-byte record limit, and the end-of-data byte; it decodes back.
    #[test]
    fn run_length_encoding_is_the_pdf_filters() {
        assert_eq!(run_length_encode(&[0, 0, 0]), [254, 0, 128]);
        assert_eq!(run_length_encode(&[1, 2, 3]), [2, 1, 2, 3, 128]);
        assert_eq!(run_length_encode(&[1, 2, 2]), [0, 1, 255, 2, 128]);
        assert_eq!(
            run_length_encode(&[255; 300]),
            [129, 255, 129, 255, 213, 255, 128]
        );
        let bytes = (0..1_000_u32)
            .map(|index| {
                if index % 7 < 3 {
                    255
                } else {
                    (index % 251) as u8
                }
            })
            .collect::<Vec<_>>();
        let encoded = run_length_encode(&bytes);
        let mut decoded = Vec::new();
        let mut at = 0;
        loop {
            let length = encoded[at];
            match length {
                128 => break,
                0..=127 => {
                    let count = usize::from(length) + 1;
                    decoded.extend_from_slice(&encoded[at + 1..at + 1 + count]);
                    at += 1 + count;
                }
                _ => {
                    decoded.extend(std::iter::repeat_n(
                        encoded[at + 1],
                        257 - usize::from(length),
                    ));
                    at += 2;
                }
            }
        }
        assert_eq!(decoded, bytes);
    }

    /// `lp` is asked for the queue, title, copies, paper, tray and sides, the
    /// page printed as drawn, collation only for several copies.
    #[test]
    fn lp_is_asked_for_exactly_the_job() {
        let file = std::path::Path::new("/tmp/job.pdf");
        let job = LpJob {
            queue: "Office",
            title: "top (RSpice)",
            copies: 2,
            collate: true,
            media: "A4",
            input_slot: Some("Tray2"),
            duplex: DuplexMode::LongEdge,
            file,
        };
        let arguments = job
            .arguments()
            .into_iter()
            .map(|argument| argument.to_string_lossy().into_owned())
            .collect::<Vec<_>>();
        assert_eq!(
            arguments,
            [
                "-d",
                "Office",
                "-t",
                "top (RSpice)",
                "-n",
                "2",
                "-o",
                "media=A4",
                "-o",
                "sides=two-sided-long-edge",
                "-o",
                "print-scaling=none",
                "-o",
                "collate=true",
                "-o",
                "InputSlot=Tray2",
                "--",
                file.to_str().unwrap(),
            ]
        );
        let single = LpJob {
            copies: 1,
            input_slot: None,
            ..job
        };
        assert!(!single.arguments().iter().any(|argument| {
            let argument = argument.to_string_lossy();
            argument.starts_with("collate") || argument.starts_with("InputSlot")
        }));
    }
}
