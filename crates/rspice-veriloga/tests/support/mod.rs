use std::ffi::OsString;
use std::path::{Path, PathBuf};

pub const BSIM4_VA_ENV: &str = "RSPICE_BSIM4_VA";

pub fn optional_bsim4_va_path(manifest_dir: &str) -> Option<PathBuf> {
    let fallback = Path::new(manifest_dir)
        .join("..")
        .join("..")
        .join("models")
        .join("veriloga")
        .join("bsim4.va");
    optional_bsim4_va_path_from(std::env::var_os(BSIM4_VA_ENV), &fallback)
}

pub fn optional_bsim4_va_path_from(
    configured: Option<OsString>,
    fallback: &Path,
) -> Option<PathBuf> {
    if let Some(raw) = configured {
        let path = PathBuf::from(raw);
        assert!(
            path.is_file(),
            "{BSIM4_VA_ENV} must point at an externally supplied BSIM4 Verilog-A source file: {}",
            path.display()
        );
        return Some(path);
    }

    fallback.is_file().then(|| fallback.to_path_buf())
}
