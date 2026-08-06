//! Validation routines for the durable hardcopy contract.
//!
//! Keeping these checks beside the data model makes deserialization and live
//! construction enforce the same bounded, fail-closed rules.

use super::*;

pub(super) fn validate_text(
    field: &'static str,
    value: &str,
    maximum: usize,
) -> Result<(), HardcopyError> {
    if value.is_empty()
        || value.trim() != value
        || value.len() > maximum
        || value.chars().any(char::is_control)
    {
        Err(HardcopyError::InvalidText { field, maximum })
    } else {
        Ok(())
    }
}

pub(super) fn validate_page_dimension(
    field: &'static str,
    value: Length,
) -> Result<(), HardcopyError> {
    if (MIN_PAGE_DIMENSION_UM..=MAX_PAGE_DIMENSION_UM).contains(&value.0) {
        Ok(())
    } else {
        Err(HardcopyError::InvalidPageDimension { field })
    }
}

pub(super) fn validate_print_feature(
    field: &'static str,
    value: Length,
) -> Result<(), HardcopyError> {
    if (1..=25_000).contains(&value.0) {
        Ok(())
    } else {
        Err(HardcopyError::InvalidPrintFeatureSize { field })
    }
}

pub(super) fn validate_margins(
    width: Length,
    height: Length,
    margins: PageMargins,
) -> Result<(), HardcopyError> {
    let horizontal = margins.left.0.checked_add(margins.right.0);
    let vertical = margins.top.0.checked_add(margins.bottom.0);
    if horizontal.is_none_or(|value| value >= width.0)
        || vertical.is_none_or(|value| value >= height.0)
    {
        Err(HardcopyError::MarginsExhaustPage)
    } else {
        Ok(())
    }
}

pub(super) fn checked_sub(
    value: u64,
    consumed: u64,
    label: &'static str,
) -> Result<u64, HardcopyError> {
    value
        .checked_sub(consumed)
        .ok_or(HardcopyError::GeometryUnderflow(label))
}

pub(super) fn validate_accepted_pages(observed: u32, expected: u32) -> Result<(), HardcopyError> {
    if observed == expected {
        Ok(())
    } else {
        Err(HardcopyError::AcceptedPageCountMismatch { expected, observed })
    }
}

pub(super) fn greatest_common_divisor(mut left: u64, mut right: u64) -> u64 {
    while right != 0 {
        let remainder = left % right;
        left = right;
        right = remainder;
    }
    left
}

pub(super) fn ratio_less_than(left: ScaleRatio, right: ScaleRatio) -> bool {
    u128::from(left.numerator) * u128::from(right.denominator)
        < u128::from(right.numerator) * u128::from(left.denominator)
}

pub(super) fn scale_ceil(value: u64, scale: ScaleRatio) -> Result<u64, HardcopyError> {
    let scaled = u128::from(value)
        .checked_mul(u128::from(scale.numerator))
        .ok_or(HardcopyError::ScaledGeometryOverflow)?;
    let denominator = u128::from(scale.denominator);
    let rounded = scaled
        .checked_add(denominator - 1)
        .ok_or(HardcopyError::ScaledGeometryOverflow)?
        / denominator;
    u64::try_from(rounded).map_err(|_| HardcopyError::ScaledGeometryOverflow)
}

pub(super) fn scale_floor(value: u64, scale: ScaleRatio) -> Result<u64, HardcopyError> {
    let product = u128::from(value)
        .checked_mul(u128::from(scale.numerator))
        .ok_or(HardcopyError::ScaledGeometryOverflow)?;
    u64::try_from(product / u128::from(scale.denominator))
        .map_err(|_| HardcopyError::ScaledGeometryOverflow)
}

pub(super) fn minimum_tiles(
    content: u64,
    viewport: u64,
    overlap: u64,
) -> Result<u16, HardcopyError> {
    if content <= viewport {
        return Ok(1);
    }
    let step = viewport
        .checked_sub(overlap)
        .ok_or(HardcopyError::OverlapExhaustsPage)?;
    if step == 0 {
        return Err(HardcopyError::OverlapExhaustsPage);
    }
    let remainder = content - viewport;
    let additional = remainder.div_ceil(step);
    let count = 1_u64
        .checked_add(additional)
        .ok_or(HardcopyError::ScaledGeometryOverflow)?;
    let count = u16::try_from(count).map_err(|_| HardcopyError::TooManyPages(u32::MAX))?;
    Ok(count)
}

pub(super) fn spreadsheet_column(zero_based: u16) -> String {
    let mut value = u32::from(zero_based) + 1;
    let mut reversed = Vec::new();
    while value != 0 {
        value -= 1;
        reversed.push((b'A' + (value % 26) as u8) as char);
        value /= 26;
    }
    reversed.into_iter().rev().collect()
}

pub(super) fn canonical_digest(
    domain: &[u8],
    value: &impl Serialize,
) -> Result<ContentDigest, HardcopyError> {
    let payload = serde_json::to_vec(value)
        .map_err(|error| HardcopyError::CanonicalSerialization(error.to_string()))?;
    let mut digest = Sha256::new();
    digest.update(domain);
    digest.update((payload.len() as u64).to_le_bytes());
    digest.update(payload);
    Ok(ContentDigest::from_bytes(digest.finalize().into()))
}

#[derive(Serialize)]
struct SavedSetupMaterial<'a> {
    schema_version: u32,
    document_kind: HardcopyDocumentKind,
    scope: &'a HardcopyScope,
    setup: &'a HardcopySetup,
}

pub(super) fn saved_setup_digest(
    document_kind: HardcopyDocumentKind,
    scope: &HardcopyScope,
    setup: &HardcopySetup,
) -> Result<ContentDigest, HardcopyError> {
    canonical_digest(
        b"rspice-hardcopy-document-setup-v1",
        &SavedSetupMaterial {
            schema_version: SETUP_STORE_SCHEMA_VERSION,
            document_kind,
            scope,
            setup,
        },
    )
}
