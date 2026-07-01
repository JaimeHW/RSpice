#[cfg(windows)]
use std::env;
#[cfg(windows)]
use std::ffi::OsString;
#[cfg(windows)]
use std::fs;
#[cfg(windows)]
use std::path::{Path, PathBuf};
#[cfg(windows)]
use std::process::Command;

fn main() {
    println!("cargo:rerun-if-env-changed=PATH");
    println!("cargo:rerun-if-env-changed=PYO3_PYTHON");

    #[cfg(windows)]
    ensure_windows_python_runtime_is_available();
}

#[cfg(windows)]
fn ensure_windows_python_runtime_is_available() {
    let Some(runtime_dir) = discover_python_runtime_dir() else {
        println!(
            "cargo:warning=Could not determine the Python runtime directory for rspice-python tests"
        );
        return;
    };

    let Some(profile_dir) = target_profile_dir() else {
        println!("cargo:warning=Could not determine Cargo target profile directory");
        return;
    };

    let destinations = [profile_dir.clone(), profile_dir.join("deps")];
    for destination in destinations {
        if let Err(err) = fs::create_dir_all(&destination) {
            println!(
                "cargo:warning=Failed to create Python runtime destination '{}': {}",
                destination.display(),
                err
            );
            continue;
        }

        if let Err(err) = copy_runtime_dlls(&runtime_dir, &destination) {
            println!(
                "cargo:warning=Failed to stage Python runtime DLLs into '{}': {}",
                destination.display(),
                err
            );
        }
    }
}

#[cfg(windows)]
fn discover_python_runtime_dir() -> Option<PathBuf> {
    let interpreter = pyo3_build_config::get()
        .executable()
        .map(OsString::from)
        .or_else(|| env::var_os("PYO3_PYTHON"))
        .unwrap_or_else(|| OsString::from("python"));

    query_python_base_prefix(&interpreter).filter(|path| path.exists())
}

#[cfg(windows)]
fn query_python_base_prefix(interpreter: &OsString) -> Option<PathBuf> {
    let output = Command::new(interpreter)
        .args([
            "-c",
            "import pathlib, sys; print(pathlib.Path(sys.base_prefix).resolve())",
        ])
        .output()
        .ok()?;

    if !output.status.success() {
        return None;
    }

    let stdout = String::from_utf8(output.stdout).ok()?;
    let prefix = stdout.lines().next()?.trim();
    if prefix.is_empty() {
        return None;
    }

    Some(PathBuf::from(prefix))
}

#[cfg(windows)]
fn target_profile_dir() -> Option<PathBuf> {
    let out_dir = PathBuf::from(env::var_os("OUT_DIR")?);
    out_dir.ancestors().nth(3).map(Path::to_path_buf)
}

#[cfg(windows)]
fn copy_runtime_dlls(runtime_dir: &Path, destination: &Path) -> std::io::Result<()> {
    let mut copied_any = false;

    for entry in fs::read_dir(runtime_dir)? {
        let entry = entry?;
        let file_type = entry.file_type()?;
        if !file_type.is_file() {
            continue;
        }

        let file_name = entry.file_name();
        let normalized = file_name.to_string_lossy().to_ascii_lowercase();
        if !should_stage_runtime_dll(&normalized) {
            continue;
        }

        fs::copy(entry.path(), destination.join(&file_name))?;
        copied_any = true;
    }

    if copied_any {
        Ok(())
    } else {
        Err(std::io::Error::new(
            std::io::ErrorKind::NotFound,
            format!(
                "no Python runtime DLLs were found in '{}'",
                runtime_dir.display()
            ),
        ))
    }
}

#[cfg(windows)]
fn should_stage_runtime_dll(file_name: &str) -> bool {
    file_name == "python3.dll"
        || (file_name.starts_with("python") && file_name.ends_with(".dll"))
        || (file_name.starts_with("vcruntime") && file_name.ends_with(".dll"))
}
