//! Bounded authentication and decoding for frozen report figures.

use super::*;

pub(super) fn validate_frozen_report_png(
    block_id: ReportBlockId,
    artifact: &FrozenReportArtifact,
) -> Result<(u32, u32), HardcopySourceError> {
    if artifact.media_type() != "image/png" {
        return Err(HardcopySourceError::UnsupportedAuthenticatedReportBlock {
            block_id,
            kind: "frozen plot figure",
            reason: format!(
                "unsupported artifact media type `{}`; expected image/png",
                artifact.media_type()
            ),
        });
    }
    let computed = ContentDigest::from_bytes(Sha256::digest(artifact.payload()).into());
    if computed != artifact.content_digest() {
        return Err(HardcopySourceError::InvalidReportSource(format!(
            "frozen plot block {block_id} artifact digest does not authenticate its payload"
        )));
    }
    if artifact.payload().len() > rspice_results::visualization_raster::MAX_RASTER_ARTIFACT_BYTES {
        return Err(HardcopySourceError::InvalidReportSource(format!(
            "frozen plot block {block_id} exceeds the PNG artifact byte limit"
        )));
    }
    if !png_has_exact_terminal_iend(artifact.payload()) {
        return Err(HardcopySourceError::InvalidReportSource(format!(
            "frozen plot block {block_id} has malformed chunks or bytes after IEND"
        )));
    }

    let decoder = png::Decoder::new(std::io::Cursor::new(artifact.payload()));
    let mut reader = decoder.read_info().map_err(|error| {
        HardcopySourceError::InvalidReportSource(format!(
            "frozen plot block {block_id} is not a valid PNG: {error}"
        ))
    })?;
    let header = reader.info();
    let width = header.width;
    let height = header.height;
    if !(rspice_results::visualization_raster::MIN_RASTER_DIMENSION
        ..=rspice_results::visualization_raster::MAX_RASTER_DIMENSION)
        .contains(&width)
        || !(rspice_results::visualization_raster::MIN_RASTER_DIMENSION
            ..=rspice_results::visualization_raster::MAX_RASTER_DIMENSION)
            .contains(&height)
    {
        return Err(HardcopySourceError::InvalidReportSource(format!(
            "frozen plot block {block_id} dimensions {width}x{height} are outside the governed raster bounds"
        )));
    }
    let pixels = usize::try_from(width)
        .ok()
        .and_then(|width| {
            usize::try_from(height)
                .ok()
                .and_then(|height| width.checked_mul(height))
        })
        .ok_or_else(|| {
            HardcopySourceError::InvalidReportSource(format!(
                "frozen plot block {block_id} dimensions overflow"
            ))
        })?;
    if pixels > rspice_results::visualization_raster::MAX_RASTER_PIXELS {
        return Err(HardcopySourceError::InvalidReportSource(format!(
            "frozen plot block {block_id} exceeds the governed pixel limit"
        )));
    }
    if header.color_type != png::ColorType::Rgb
        || header.bit_depth != png::BitDepth::Eight
        || header.trns.is_some()
        || header.animation_control.is_some()
    {
        return Err(HardcopySourceError::InvalidReportSource(format!(
            "frozen plot block {block_id} must be a single-frame opaque RGB8 PNG"
        )));
    }
    let expected_bytes = pixels.checked_mul(3).ok_or_else(|| {
        HardcopySourceError::InvalidReportSource(format!(
            "frozen plot block {block_id} decoded byte count overflowed"
        ))
    })?;
    let output_size = reader.output_buffer_size().ok_or_else(|| {
        HardcopySourceError::InvalidReportSource(format!(
            "frozen plot block {block_id} has no bounded decoded size"
        ))
    })?;
    if output_size != expected_bytes {
        return Err(HardcopySourceError::InvalidReportSource(format!(
            "frozen plot block {block_id} decoded byte count does not match RGB8 dimensions"
        )));
    }
    let mut decoded = Vec::new();
    decoded.try_reserve_exact(output_size).map_err(|_| {
        HardcopySourceError::InvalidReportSource(format!(
            "frozen plot block {block_id} decoded buffer allocation failed"
        ))
    })?;
    decoded.resize(output_size, 0);
    let frame = reader.next_frame(&mut decoded).map_err(|error| {
        HardcopySourceError::InvalidReportSource(format!(
            "frozen plot block {block_id} PNG payload failed full decode: {error}"
        ))
    })?;
    if frame.width != width
        || frame.height != height
        || frame.color_type != png::ColorType::Rgb
        || frame.bit_depth != png::BitDepth::Eight
        || frame.buffer_size() != expected_bytes
    {
        return Err(HardcopySourceError::InvalidReportSource(format!(
            "frozen plot block {block_id} decoded frame contradicts its authenticated header"
        )));
    }
    Ok((width, height))
}

pub(super) fn png_has_exact_terminal_iend(payload: &[u8]) -> bool {
    const SIGNATURE: &[u8; 8] = b"\x89PNG\r\n\x1a\n";
    if payload.len() < SIGNATURE.len() || &payload[..8] != SIGNATURE {
        return false;
    }
    let mut offset = 8usize;
    let mut saw_ihdr = false;
    while offset < payload.len() {
        let Some(header_end) = offset.checked_add(8) else {
            return false;
        };
        if header_end > payload.len() {
            return false;
        }
        let length = u32::from_be_bytes(
            payload[offset..offset + 4]
                .try_into()
                .expect("four-byte PNG length"),
        ) as usize;
        let chunk_type = &payload[offset + 4..offset + 8];
        if !saw_ihdr {
            if chunk_type != b"IHDR" {
                return false;
            }
            saw_ihdr = true;
        }
        let Some(chunk_end) = header_end
            .checked_add(length)
            .and_then(|end| end.checked_add(4))
        else {
            return false;
        };
        if chunk_end > payload.len() {
            return false;
        }
        if chunk_type == b"IEND" {
            return length == 0 && chunk_end == payload.len();
        }
        offset = chunk_end;
    }
    false
}
