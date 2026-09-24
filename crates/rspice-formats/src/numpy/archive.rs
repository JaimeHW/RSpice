//! Named NumPy arrays in a deterministic stored ZIP archive.

use std::collections::BTreeSet;

use super::{MAX_COLUMNS, NamedArray, encode_complex_array, encode_real_array};
use crate::zip::deterministic_stored_zip;
use num_complex::Complex64;

/// A member name RSpice's own archive reader will accept.
///
/// The reader refuses a member whose name is absolute, contains `..`, or
/// contains a backslash, and it identifies an array by the file stem, so two
/// signals whose names differ only in case collide. Every one of those is
/// refused here, by name, rather than published as an archive this product
/// would not reopen.
fn archive_member_name(name: &str) -> Result<String, String> {
    if name.trim().is_empty() {
        return Err(
            "A signal with no name cannot become an archive member; export CSV instead.".to_owned(),
        );
    }
    if name.starts_with('/') || name.contains("..") || name.contains('\\') {
        return Err(format!(
            "'{name}' cannot be an archive member name: RSpice refuses an archive member that is \
             absolute, contains '..', or contains a backslash. Export CSV or an RSpice bundle."
        ));
    }
    Ok(format!("{name}.npy"))
}

/// One `.npy` member per signal, plus the coordinate, in a stored ZIP.
pub fn encode_npz(
    coordinate_name: &str,
    coordinate: &[f64],
    signals: &[NamedArray<'_>],
) -> Result<Vec<u8>, String> {
    let members = signals.len().checked_add(1).ok_or_else(|| {
        format!("This result needs too many archive members; RSpice reads at most {MAX_COLUMNS}.")
    })?;
    if members > MAX_COLUMNS {
        return Err(format!(
            "This result needs {members} archive members; RSpice reads at most {MAX_COLUMNS}."
        ));
    }
    if coordinate.is_empty() || signals.is_empty() {
        return Err("A NumPy archive needs coordinate samples and at least one signal.".into());
    }
    for signal in signals {
        if signal.real.len() != coordinate.len()
            || signal
                .imag
                .is_some_and(|imag| imag.len() != coordinate.len())
        {
            return Err(format!(
                "'{}' has {} samples against {} coordinate samples; the export is refused rather than padded or truncated.",
                signal.name,
                signal.real.len(),
                coordinate.len()
            ));
        }
    }
    let rows = [coordinate.len() as u64];
    let mut names = Vec::with_capacity(members);
    let mut seen = BTreeSet::new();
    names.push(archive_member_name(coordinate_name)?);
    seen.insert(coordinate_name.to_ascii_lowercase());
    for signal in signals {
        let member = archive_member_name(signal.name)?;
        if !seen.insert(signal.name.to_ascii_lowercase()) {
            return Err(format!(
                "Two signals both claim the archive member '{}'. An archive names its arrays, so \
                 the names have to differ; RSpice compares them without regard to case.",
                signal.name
            ));
        }
        names.push(member);
    }

    let mut payloads = Vec::with_capacity(members);
    payloads.push(encode_real_array(&rows, coordinate)?);
    for signal in signals {
        let bytes = match signal.imag {
            Some(imag) => {
                let values = signal
                    .real
                    .iter()
                    .zip(imag)
                    .map(|(re, im)| Complex64::new(*re, *im))
                    .collect::<Vec<_>>();
                encode_complex_array(&rows, &values)?
            }
            None => encode_real_array(&rows, signal.real)?,
        };
        payloads.push(bytes);
    }

    let entries = names
        .iter()
        .map(String::as_str)
        .zip(payloads.iter().map(Vec::as_slice))
        .collect::<Vec<_>>();
    deterministic_stored_zip(&entries)
}

#[cfg(test)]
mod tests {
    use super::{NamedArray, encode_npz};

    #[test]
    fn rejects_unbalanced_columns_before_writing_archive_members() {
        let signal = NamedArray {
            name: "V(out)",
            real: &[1.0],
            imag: None,
        };
        assert!(encode_npz("time", &[0.0, 1.0], &[signal]).is_err());
    }
}
