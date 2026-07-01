//! Platform-neutral data-file access for XSPICE code models.

use crate::Value;
use std::collections::HashMap;
use std::hash::{Hash, Hasher};
use std::path::{Path, PathBuf};
use std::sync::atomic::{AtomicU64, Ordering};
use std::sync::{Arc, Mutex, MutexGuard, OnceLock};
use std::time::UNIX_EPOCH;

#[derive(Debug, Clone, Copy, Hash, PartialEq, Eq)]
pub(crate) struct DataFileStamp {
    pub len: u64,
    pub modified_nanos: u128,
    pub content_hash: u64,
    pub virtual_file: bool,
}

#[derive(Debug, Clone)]
struct VirtualDataFile {
    contents: Arc<str>,
    stamp: DataFileStamp,
}

static VIRTUAL_DATA_FILE_EPOCH: AtomicU64 = AtomicU64::new(0);

fn virtual_files() -> &'static Mutex<HashMap<String, VirtualDataFile>> {
    static FILES: OnceLock<Mutex<HashMap<String, VirtualDataFile>>> = OnceLock::new();
    FILES.get_or_init(|| Mutex::new(HashMap::new()))
}

fn lock_virtual_files() -> MutexGuard<'static, HashMap<String, VirtualDataFile>> {
    virtual_files()
        .lock()
        .unwrap_or_else(|poisoned| poisoned.into_inner())
}

fn bump_virtual_data_file_epoch() {
    VIRTUAL_DATA_FILE_EPOCH.fetch_add(1, Ordering::AcqRel);
}

pub(crate) fn virtual_data_file_epoch() -> u64 {
    VIRTUAL_DATA_FILE_EPOCH.load(Ordering::Acquire)
}

pub(crate) fn sync_virtual_data_file_epoch(epoch: &mut u64) -> bool {
    let current = virtual_data_file_epoch();
    if *epoch == current {
        return false;
    }
    *epoch = current;
    true
}

fn content_hash<T: Hash + ?Sized>(contents: &T) -> u64 {
    let mut hasher = std::collections::hash_map::DefaultHasher::new();
    contents.hash(&mut hasher);
    hasher.finish()
}

pub(crate) fn parse_numeric_prefix_len(input: &str) -> Option<(Value, usize)> {
    fn ascii_starts_with_ignore_case(input: &[u8], pattern: &[u8]) -> bool {
        input.len() >= pattern.len()
            && input
                .iter()
                .zip(pattern.iter())
                .all(|(actual, expected)| actual.eq_ignore_ascii_case(expected))
    }

    let bytes = input.as_bytes();
    let mut index = 0usize;

    if matches!(bytes.get(index), Some(b'+' | b'-')) {
        index += 1;
    }
    let signed_index = index;

    let sign = if matches!(bytes.first(), Some(b'-')) {
        -1.0
    } else {
        1.0
    };
    for special in [b"infinity".as_slice(), b"inf".as_slice(), b"nan".as_slice()] {
        if ascii_starts_with_ignore_case(&bytes[signed_index..], special) {
            let end = signed_index + special.len();
            let value = if special == b"nan" {
                Value::NAN
            } else {
                sign * Value::INFINITY
            };
            return Some((value, end));
        }
    }

    let mut digits = 0usize;
    while matches!(bytes.get(index), Some(b'0'..=b'9')) {
        index += 1;
        digits += 1;
    }

    if matches!(bytes.get(index), Some(b'.')) {
        index += 1;
        while matches!(bytes.get(index), Some(b'0'..=b'9')) {
            index += 1;
            digits += 1;
        }
    }

    if digits == 0 {
        return None;
    }

    let mantissa_end = index;
    if matches!(bytes.get(index), Some(b'e' | b'E')) {
        index += 1;
        if matches!(bytes.get(index), Some(b'+' | b'-')) {
            index += 1;
        }
        let exponent_digit_start = index;
        while matches!(bytes.get(index), Some(b'0'..=b'9')) {
            index += 1;
        }
        if exponent_digit_start == index {
            index = mantissa_end;
        }
    }

    input[..index]
        .parse::<Value>()
        .ok()
        .map(|value| (value, index))
}

pub(crate) fn parse_numeric_prefix(input: &str) -> Option<Value> {
    parse_numeric_prefix_len(input).map(|(value, _)| value)
}

pub(crate) fn parse_ngspice_spice_value(token: &str) -> Value {
    let mut value_end = 0usize;
    let mut suffix = None;
    let mut chars = token.char_indices().take(81).enumerate();

    while let Some((_, (byte_index, ch))) = chars.next() {
        if ch.is_ascii_alphabetic() && ch != 'e' && ch != 'E' {
            suffix = Some((ch, chars.next().map(|(_, (_, next))| next)));
            value_end = byte_index;
            break;
        }
        if ch.is_ascii_whitespace() {
            value_end = byte_index;
            break;
        }
        value_end = byte_index + ch.len_utf8();
    }

    let scale = match suffix {
        Some(('t' | 'T', _)) => 1.0e12,
        Some(('g' | 'G', _)) => 1.0e9,
        Some(('k' | 'K', _)) => 1.0e3,
        Some(('u' | 'U', _)) => 1.0e-6,
        Some(('n' | 'N', _)) => 1.0e-9,
        Some(('p' | 'P', _)) => 1.0e-12,
        Some(('f' | 'F', _)) => 1.0e-15,
        Some(('a' | 'A', _)) => 1.0e-18,
        Some(('m' | 'M', next)) => match next {
            Some('e' | 'E') => 1.0e6,
            Some('i' | 'I') => 25.4e-6,
            _ => 1.0e-3,
        },
        Some(_) | None => 1.0,
    };

    parse_numeric_prefix(&token[..value_end]).unwrap_or(0.0) * scale
}

fn virtual_stamp(contents: &str) -> DataFileStamp {
    let hash = content_hash(contents);
    DataFileStamp {
        len: contents.len() as u64,
        modified_nanos: hash as u128,
        content_hash: hash,
        virtual_file: true,
    }
}

fn stamp_from_metadata(metadata: std::fs::Metadata, content_hash: u64) -> DataFileStamp {
    let modified_nanos = metadata
        .modified()
        .ok()
        .and_then(|time| time.duration_since(UNIX_EPOCH).ok())
        .map(|duration| duration.as_nanos())
        .unwrap_or(0);

    DataFileStamp {
        len: metadata.len(),
        modified_nanos,
        content_hash,
        virtual_file: false,
    }
}

fn ngspice_input_dir_path(path: &Path) -> Option<PathBuf> {
    if path.is_absolute() {
        return None;
    }
    let input_dir = std::env::var_os("NGSPICE_INPUT_DIR")?;
    if input_dir.as_os_str().is_empty() {
        return None;
    }
    Some(PathBuf::from(input_dir).join(path))
}

fn fallback_error(
    primary: &std::io::Error,
    fallback: &Path,
    fallback_err: std::io::Error,
) -> String {
    format!(
        "{}; NGSPICE_INPUT_DIR fallback '{}' failed: {}",
        primary,
        fallback.display(),
        fallback_err
    )
}

fn read_native_to_string_with_stamp(path: &str) -> Result<(Arc<str>, DataFileStamp), String> {
    fn read_path(path: &Path) -> Result<(Arc<str>, DataFileStamp), std::io::Error> {
        let contents = std::fs::read_to_string(path)?;
        let metadata = std::fs::metadata(path)?;
        let stamp = stamp_from_metadata(metadata, content_hash(contents.as_bytes()));
        Ok((Arc::<str>::from(contents), stamp))
    }

    let path_ref = Path::new(path);
    match read_path(path_ref) {
        Ok(file) => Ok(file),
        Err(primary) => {
            if let Some(fallback) = ngspice_input_dir_path(path_ref) {
                return read_path(&fallback)
                    .map_err(|fallback_err| fallback_error(&primary, &fallback, fallback_err));
            }
            Err(primary.to_string())
        }
    }
}

fn native_stamp(path: &str) -> Result<DataFileStamp, String> {
    read_native_to_string_with_stamp(path).map(|(_, stamp)| stamp)
}

fn registered(path: &str) -> Result<Option<VirtualDataFile>, String> {
    let files = lock_virtual_files();
    Ok(files.get(path).cloned())
}

/// Register or replace a virtual XSPICE data file.
///
/// This is the browser/mobile-compatible path for file-backed XSPICE models.
/// Desktop builds still fall back to native filesystem reads when no virtual
/// file has been registered for the requested path.
pub fn register_data_file(
    path: impl Into<String>,
    contents: impl Into<String>,
) -> Result<(), String> {
    let path = path.into();
    let contents = contents.into();
    let stamp = virtual_stamp(&contents);
    let file = VirtualDataFile {
        stamp,
        contents: Arc::<str>::from(contents),
    };
    {
        let mut files = lock_virtual_files();
        if let Some(existing) = files.get(&path)
            && existing.stamp == file.stamp
            && existing.contents == file.contents
        {
            return Ok(());
        }
        files.insert(path, file);
    }
    bump_virtual_data_file_epoch();
    Ok(())
}

/// Remove a virtual XSPICE data file.
pub fn unregister_data_file(path: &str) -> Result<(), String> {
    let removed = {
        let mut files = lock_virtual_files();
        files.remove(path).is_some()
    };
    if removed {
        bump_virtual_data_file_epoch();
    }
    Ok(())
}

/// Clear all virtual XSPICE data files.
pub fn clear_registered_data_files() -> Result<(), String> {
    let cleared = {
        let mut files = lock_virtual_files();
        let cleared = !files.is_empty();
        files.clear();
        cleared
    };
    if cleared {
        bump_virtual_data_file_epoch();
    }
    Ok(())
}

pub(crate) fn data_file_stamp(path: &str) -> Result<DataFileStamp, String> {
    if let Some(file) = registered(path)? {
        return Ok(file.stamp);
    }
    native_stamp(path)
}

pub(crate) fn virtual_data_file_stamp(path: &str) -> Option<DataFileStamp> {
    let files = lock_virtual_files();
    files.get(path).map(|file| file.stamp)
}

pub(crate) fn loaded_virtual_data_file_stamp(stamp: DataFileStamp) -> Option<DataFileStamp> {
    stamp.virtual_file.then_some(stamp)
}

pub(crate) fn read_to_string_with_stamp(path: &str) -> Result<(Arc<str>, DataFileStamp), String> {
    if let Some(file) = registered(path)? {
        return Ok((file.contents, file.stamp));
    }
    read_native_to_string_with_stamp(path)
}

pub(crate) fn read_to_string(path: &str) -> Result<Arc<str>, String> {
    read_to_string_with_stamp(path).map(|(contents, _)| contents)
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::sync::{Mutex, MutexGuard, OnceLock};
    use std::time::{SystemTime, UNIX_EPOCH};

    fn virtual_registry_test_lock() -> MutexGuard<'static, ()> {
        static LOCK: OnceLock<Mutex<()>> = OnceLock::new();
        LOCK.get_or_init(|| Mutex::new(()))
            .lock()
            .unwrap_or_else(|poisoned| poisoned.into_inner())
    }

    fn poison_virtual_files_lock() {
        let result = std::panic::catch_unwind(|| {
            let _guard = lock_virtual_files();
            panic!("poison XSPICE data-file registry for recovery test");
        });
        assert!(result.is_err(), "recovery test must poison the mutex");
    }

    #[test]
    fn virtual_data_file_registry_recovers_after_poison() {
        let _guard = virtual_registry_test_lock();
        poison_virtual_files_lock();

        register_data_file("virtual://poison/recovered", "0 1\n")
            .expect("register after poisoned registry");
        let contents =
            read_to_string("virtual://poison/recovered").expect("read after poisoned registry");
        assert_eq!(&*contents, "0 1\n");

        clear_registered_data_files().expect("clear after poisoned registry");
    }

    #[test]
    fn registering_identical_virtual_data_file_is_idempotent() {
        let _guard = virtual_registry_test_lock();
        let unique = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .expect("system clock after epoch")
            .as_nanos();
        let path = format!(
            "virtual://rspice/data-file/idempotent-{}-{unique}",
            std::process::id()
        );

        let _ = unregister_data_file(&path);
        register_data_file(&path, "0 1\n").expect("register first virtual data file");
        let (first_contents, first_stamp) =
            read_to_string_with_stamp(&path).expect("read first virtual data file");

        register_data_file(&path, "0 1\n").expect("register identical virtual data file");
        let (second_contents, second_stamp) =
            read_to_string_with_stamp(&path).expect("read identical virtual data file");

        assert_eq!(first_stamp, second_stamp);
        assert!(
            Arc::ptr_eq(&first_contents, &second_contents),
            "unchanged virtual file registrations should not replace the cached contents"
        );

        register_data_file(&path, "0 2\n").expect("replace virtual data file");
        let (third_contents, third_stamp) =
            read_to_string_with_stamp(&path).expect("read replaced virtual data file");

        assert_ne!(second_stamp, third_stamp);
        assert!(!Arc::ptr_eq(&second_contents, &third_contents));

        unregister_data_file(&path).expect("unregister virtual data file");
    }

    #[test]
    fn native_data_file_stamp_hashes_same_length_rewrites() {
        let unique = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .expect("system clock after epoch")
            .as_nanos();
        let dir = std::env::temp_dir().join(format!(
            "rspice-xspice-data-stamp-{}-{unique}",
            std::process::id()
        ));
        std::fs::create_dir_all(&dir).expect("create native data-file stamp dir");
        let path = dir.join("stim.dat");

        std::fs::write(&path, "0 0s\n").expect("write first native data file");
        let first = data_file_stamp(path.to_str().expect("temp path is utf-8"))
            .expect("stamp first native data file");

        std::fs::write(&path, "0 1s\n").expect("rewrite same-length native data file");
        let second = data_file_stamp(path.to_str().expect("temp path is utf-8"))
            .expect("stamp rewritten native data file");
        let _ = std::fs::remove_dir_all(dir);

        assert_eq!(first.len, second.len);
        assert!(!first.virtual_file);
        assert_ne!(
            first.content_hash, second.content_hash,
            "native XSPICE data-file cache keys must change when same-length contents change"
        );
    }

    #[test]
    fn ngspice_spice_value_parser_preserves_suffix_contract() {
        let cases: [(&str, Value); 17] = [
            ("1T", 1.0e12),
            ("1G", 1.0e9),
            ("1K", 1.0e3),
            ("1M", 1.0e-3),
            ("1ME", 1.0e6),
            ("1MEG", 1.0e6),
            ("1megohm", 1.0e6),
            ("1MI", 25.4e-6),
            ("1MIL", 25.4e-6),
            ("1mismatch", 25.4e-6),
            ("1U", 1.0e-6),
            ("1N", 1.0e-9),
            ("1P", 1.0e-12),
            ("1F", 1.0e-15),
            ("1A", 1.0e-18),
            ("2.5e3K", 2.5e6),
            ("2.5e-3MEG", 2.5e3),
        ];

        for (token, expected) in cases {
            let actual = parse_ngspice_spice_value(token);
            let tolerance = (expected.abs() * 1.0e-12).max(1.0e-24);
            assert!(
                (actual - expected).abs() <= tolerance,
                "token {token:?} parsed as {actual:e}, expected {expected:e}"
            );
        }
    }

    #[test]
    fn ngspice_spice_value_parser_preserves_legacy_81_char_limit() {
        let suffix_inside_limit = format!("1{}K", "0".repeat(79));
        let suffix_outside_limit = format!("1{}K", "0".repeat(80));

        assert_eq!(suffix_inside_limit.chars().count(), 81);
        assert_eq!(suffix_outside_limit.chars().count(), 82);

        let inside = parse_ngspice_spice_value(&suffix_inside_limit);
        let outside = parse_ngspice_spice_value(&suffix_outside_limit);

        assert!(
            (inside - 1.0e82).abs() <= 1.0e70,
            "suffix at the 81-character parser boundary should still apply, got {inside:e}"
        );
        assert!(
            (outside - 1.0e80).abs() <= 1.0e68,
            "suffix after the 81-character parser boundary should be ignored, got {outside:e}"
        );
    }

    #[test]
    fn numeric_prefix_parser_matches_strtod_prefix_style() {
        let cases: [(&str, Value, usize); 9] = [
            ("1e-9junk", 1.0e-9, 4),
            ("1e", 1.0, 1),
            ("1e+", 1.0, 1),
            ("1e+suffix", 1.0, 1),
            ("-.5suffix", -0.5, 3),
            (".5suffix", 0.5, 2),
            ("1.e2tail", 100.0, 4),
            ("+2.25,", 2.25, 5),
            ("-0.", -0.0, 3),
        ];

        for (input, expected_value, expected_len) in cases {
            let Some((actual_value, actual_len)) = parse_numeric_prefix_len(input) else {
                panic!("input {input:?} should have a numeric prefix");
            };
            assert_eq!(actual_len, expected_len, "prefix length for {input:?}");
            assert!(
                (actual_value - expected_value).abs() <= 1.0e-15,
                "prefix value for {input:?}: got {actual_value:e}, expected {expected_value:e}"
            );
        }

        let Some((infinite_value, infinite_len)) = parse_numeric_prefix_len("-infinitytail") else {
            panic!("infinity prefix should parse");
        };
        assert_eq!(infinite_len, "-infinity".len());
        assert!(infinite_value.is_infinite() && infinite_value.is_sign_negative());

        let Some((nan_value, nan_len)) = parse_numeric_prefix_len("NaNtail") else {
            panic!("NaN prefix should parse");
        };
        assert_eq!(nan_len, "NaN".len());
        assert!(nan_value.is_nan());

        assert!(parse_numeric_prefix_len(",1").is_none());
        assert!(parse_numeric_prefix_len("abc").is_none());
    }
}
