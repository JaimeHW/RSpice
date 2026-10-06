//! Invocation-wide destination ownership, including reports written after results.
use crate::cli::CliError;
use std::cell::RefCell;
use std::collections::{HashMap, HashSet};
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
    declared: HashMap<PathBuf, String>,
    sources: HashSet<PathBuf>,
    checkpoint_inputs: HashMap<PathBuf, bool>,
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
            "choose output destinations distinct from each other and from source files".into(),
        ),
    }
}

/// Refuse to replace any source directory entry with a generated artifact.
/// Directory aliases and Windows case folding use the same rules as outputs.
pub(crate) fn protect_sources<'a>(
    output: &Path,
    sources: impl IntoIterator<Item = &'a Path>,
) -> Result<(), CliError> {
    let output_key = key(output).map_err(|error| CliError::output_error(output, error))?;
    for source in sources {
        let source_key = key(source).map_err(|error| CliError::InputReadError {
            path: source.to_path_buf(),
            source: error,
        })?;
        if output_key == source_key {
            return Err(CliError::InvalidArgument {
                message: format!(
                    "output '{}' would overwrite source '{}'",
                    output.display(),
                    source.display()
                ),
                suggestion: Some("choose an output path distinct from every source file".into()),
            });
        }
    }
    Ok(())
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
        registry.claim(path, role, false)?;
    }
    registry.lock()?.declared = known
        .into_iter()
        .map(|(path, role)| (path, role.into()))
        .collect();
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
    /// Resume state may only be replaced by an explicitly requested checkpoint
    /// at the same path. Ordinary artifacts and reports never own this input.
    pub(crate) fn protect_checkpoint(
        &self,
        source: &Path,
        renewal: Option<&Path>,
    ) -> Result<(), CliError> {
        let source_key = key(source).map_err(|error| CliError::InputReadError {
            path: source.to_path_buf(),
            source: error,
        })?;
        let renewable = renewal
            .map(key)
            .transpose()
            .map_err(|error| CliError::output_error(renewal.unwrap_or(source), error))?
            .is_some_and(|path| path == source_key);
        let mut state = self.lock()?;
        let conflict_role = state.paths.get(&source_key).or_else(|| {
            state
                .declared
                .get(&source_key)
                .filter(|role| !renewable || *role != "checkpoint")
        });
        if let Some(role) = conflict_role {
            let message = format!(
                "{role} would overwrite checkpoint source '{}'",
                source.display()
            );
            state.collision = Some(message.clone());
            return Err(conflict(message));
        }
        state
            .checkpoint_inputs
            .entry(source_key)
            .and_modify(|allowed| *allowed &= renewable)
            .or_insert(renewable);
        Ok(())
    }

    /// Reading a source more than once is valid; replacing it with output is not.
    pub(crate) fn protect(&self, source: &Path) -> Result<(), CliError> {
        let source_key = key(source).map_err(|error| CliError::InputReadError {
            path: source.to_path_buf(),
            source: error,
        })?;
        let mut state = self.lock()?;
        if let Some(role) = state
            .paths
            .get(&source_key)
            .or_else(|| state.declared.get(&source_key))
        {
            let message = format!("{role} would overwrite source '{}'", source.display());
            state.collision = Some(message.clone());
            return Err(conflict(message));
        }
        state.sources.insert(source_key);
        Ok(())
    }

    fn lock(&self) -> Result<std::sync::MutexGuard<'_, State>, CliError> {
        self.state.lock().map_err(|_| CliError::InternalError {
            message: "output destination registry was poisoned".into(),
        })
    }

    fn claim(&self, path: &Path, role: &str, checkpoint: bool) -> Result<(), CliError> {
        let key = key(path).map_err(|error| CliError::output_error(path, error))?;
        let mut state = self.lock()?;
        if state.sources.contains(&key)
            || state
                .checkpoint_inputs
                .get(&key)
                .is_some_and(|renewable| !checkpoint || !renewable)
        {
            let message = format!("{role} would overwrite source '{}'", path.display());
            state.collision = Some(message.clone());
            return Err(conflict(message));
        }
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

pub(crate) fn protect(source: &Path) -> Result<(), CliError> {
    current().map_or(Ok(()), |registry| registry.protect(source))
}

pub(crate) fn claim(path: &Path) -> Result<(), CliError> {
    current().map_or(Ok(()), |registry| registry.claim(path, "result", false))
}

pub(crate) fn claim_checkpoint(path: &Path) -> Result<(), CliError> {
    current().map_or(Ok(()), |registry| registry.claim(path, "checkpoint", true))
}
