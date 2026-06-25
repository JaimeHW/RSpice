use std::path::{Component, Path, PathBuf};

use super::{GeneratedRustDevice, RustBackendError};

pub fn write_generated_device(
    root: impl AsRef<Path>,
    device: &GeneratedRustDevice,
) -> Result<Vec<PathBuf>, RustBackendError> {
    let root = root.as_ref();
    let device_dir = root.join(&device.folder_name);
    std::fs::create_dir_all(&device_dir).map_err(|error| {
        RustBackendError::internal(
            "<generated>",
            &device.module_name,
            format!(
                "failed to create generated device directory '{}': {error}",
                device_dir.display()
            ),
        )
    })?;

    let mut written = Vec::with_capacity(device.files.len());
    for file in &device.files {
        validate_relative_path(&file.relative_path, &device.module_name)?;

        let path = device_dir.join(&file.relative_path);
        if let Some(parent) = path.parent() {
            std::fs::create_dir_all(parent).map_err(|error| {
                RustBackendError::internal(
                    "<generated>",
                    &device.module_name,
                    format!(
                        "failed to create generated file parent '{}': {error}",
                        parent.display()
                    ),
                )
            })?;
        }
        std::fs::write(&path, &file.contents).map_err(|error| {
            RustBackendError::internal(
                "<generated>",
                &device.module_name,
                format!(
                    "failed to write generated file '{}': {error}",
                    path.display()
                ),
            )
        })?;
        written.push(path);
    }

    Ok(written)
}

fn validate_relative_path(path: &str, module_name: &str) -> Result<(), RustBackendError> {
    let path = Path::new(path);
    if path.is_absolute() {
        return Err(unsafe_path_error(module_name, path));
    }

    for component in path.components() {
        match component {
            Component::Normal(_) => {}
            _ => return Err(unsafe_path_error(module_name, path)),
        }
    }

    Ok(())
}

fn unsafe_path_error(module_name: &str, path: &Path) -> RustBackendError {
    RustBackendError::internal(
        "<generated>",
        module_name,
        format!("unsafe generated relative path '{}'", path.display()),
    )
}
