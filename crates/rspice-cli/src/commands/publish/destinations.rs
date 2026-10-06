//! Invocation-wide destination ownership, including reports written after results.
use crate::cli::CliError;
use std::cell::RefCell;
use std::collections::HashMap;
use std::path::{Component, Path, PathBuf};
use std::sync::{Arc, Mutex};

thread_local! {
    static ACTIVE: RefCell<Option<Arc<Destinations>>> = const { RefCell::new(None) };
}

#[derive(Default)]
pub(crate) struct Destinations {
    state: Mutex<State>,
}

#[derive(Default)]
struct State {
    paths: HashMap<PathBuf, String>,
    collision: Option<String>,
}

pub(crate) struct DestinationScope {
    previous: Option<Arc<Destinations>>,
}

impl Drop for DestinationScope {
    fn drop(&mut self) {
        ACTIVE.with(|active| *active.borrow_mut() = self.previous.take());
    }
}

fn key(path: &Path) -> std::io::Result<PathBuf> {
    let absolute = std::path::absolute(path)?;
    let mut normalized = PathBuf::new();
    for component in absolute.components() {
        match component {
            Component::CurDir => {}
            Component::ParentDir => {
                normalized.pop();
            }
            component => normalized.push(component.as_os_str()),
        }
    }
    // Resolve existing directory aliases without requiring a destination to
    // exist. Atomic replacement owns a directory entry, not a hard-link inode.
    let mut parent = normalized.parent();
    while let Some(candidate) = parent {
        if let Ok(canonical) = candidate.canonicalize() {
            if let Ok(suffix) = normalized.strip_prefix(candidate) {
                normalized = canonical.join(suffix);
            }
            break;
        }
        parent = candidate.parent();
    }
    #[cfg(windows)]
    {
        normalized = PathBuf::from(normalized.to_string_lossy().to_lowercase());
    }
    Ok(normalized)
}

fn conflict(message: String) -> CliError {
    CliError::InvalidArgument {
        message,
        suggestion: Some(
            "choose distinct destinations for results, checkpoints and reports".into(),
        ),
    }
}

pub(crate) fn begin(
    declared: &[(&str, &Path)],
    reports: &[(&str, &Path)],
) -> Result<(Arc<Destinations>, DestinationScope), CliError> {
    let registry = Arc::new(Destinations::default());
    let mut known = HashMap::new();
    for &(role, path) in declared.iter().chain(reports) {
        let key = key(path).map_err(|error| CliError::output_error(path, error))?;
        if let Some(previous) = known.insert(key, role) {
            return Err(conflict(format!(
                "{role} and {previous} share output destination '{}'",
                path.display()
            )));
        }
    }
    for &(role, path) in reports {
        registry.claim(path, role)?;
    }
    let scope = enter(registry.clone());
    Ok((registry, scope))
}

pub(crate) fn current() -> Option<Arc<Destinations>> {
    ACTIVE.with(|active| active.borrow().clone())
}

pub(crate) fn enter(registry: Arc<Destinations>) -> DestinationScope {
    let previous = ACTIVE.with(|active| active.borrow_mut().replace(registry));
    DestinationScope { previous }
}

impl Destinations {
    fn lock(&self) -> Result<std::sync::MutexGuard<'_, State>, CliError> {
        self.state.lock().map_err(|_| CliError::InternalError {
            message: "output destination registry was poisoned".into(),
        })
    }

    fn claim(&self, path: &Path, role: &str) -> Result<(), CliError> {
        let key = key(path).map_err(|error| CliError::output_error(path, error))?;
        let mut state = self.lock()?;
        if let Some(previous) = state.paths.get(&key) {
            let message = format!("{role} collides with {previous} at '{}'", path.display());
            state.collision = Some(message.clone());
            return Err(conflict(message));
        }
        state.paths.insert(key, role.to_string());
        Ok(())
    }

    pub(crate) fn finish(&self) -> Result<(), CliError> {
        match self.lock()?.collision.clone() {
            Some(message) => Err(conflict(message)),
            None => Ok(()),
        }
    }
}

pub(crate) fn claim(path: &Path) -> Result<(), CliError> {
    current().map_or(Ok(()), |registry| registry.claim(path, "result/checkpoint"))
}
