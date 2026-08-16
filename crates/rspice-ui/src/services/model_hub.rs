//! The session's Model Hub, and why it is allowed to be absent.
//!
//! One [`ModelHub`] per running application, opened once at startup over the
//! store this platform has: a directory under local application data on the
//! desktop, memory in the browser. Opening it sweeps whatever a killed install
//! left behind, so recovery is a consequence of launching rather than a repair
//! anyone runs.
//!
//! # Absence is a state, not a failure
//!
//! A machine with no writable application-data directory, a build whose
//! compiled-in key is unusable, a browser with storage denied — each of these
//! makes the hub unavailable and *none* of them makes RSpice unusable. The
//! service therefore holds `Option<ModelHub>` beside a plain-language reason,
//! and every caller that cannot act says which of the two it is. Nothing in
//! the application asks whether the hub opened before deciding to run.
//!
//! # The background seam
//!
//! Installing is slow, so it happens on a worker (a thread natively, a task in
//! the browser) driven by the operation machine in
//! [`crate::workbench::app::dialogs`]. A worker does not borrow this service:
//! it opens its *own* hub over the same store through
//! [`ModelHubStoreHandle::open`], performs one operation, and the session
//! reloads afterwards. That is why the store handle is the shared thing and
//! the hub is not — two hubs over one store always agree, because the store is
//! where the state actually lives.

use crate::state::model_hub::{ModelHub, ModelHubError, ModelHubStore, TrustAnchor};

/// How long a cached catalog may go unrefreshed before a workspace says so.
pub(crate) const CATALOG_STALE_AFTER_DAYS: u64 = 7;

const SECONDS_PER_DAY: u64 = 24 * 60 * 60;

/// A handle that can open a hub over the session's store, from anywhere.
///
/// Cloning it shares the store rather than copying its contents: a filesystem
/// handle is a path, and a memory handle is a reference to the one allocation
/// this session's packs live in.
#[derive(Debug, Clone)]
pub(crate) enum ModelHubStoreHandle {
    #[cfg(not(target_arch = "wasm32"))]
    Filesystem {
        store: crate::state::model_hub::FilesystemModelHubStore,
        root: std::path::PathBuf,
    },
    /// The browser's store, and every test's. A desktop build never reaches
    /// it: a session whose filesystem store will not open has no hub at all
    /// rather than a silently non-durable one.
    #[cfg(any(test, target_arch = "wasm32"))]
    Memory(std::sync::Arc<crate::state::model_hub::MemoryModelHubStore>),
}

impl ModelHubStoreHandle {
    /// The handle a shipped build uses on this platform.
    pub(crate) fn production() -> Result<Self, ModelHubError> {
        #[cfg(not(target_arch = "wasm32"))]
        {
            let store = crate::state::model_hub::FilesystemModelHubStore::production()?;
            let root = store.pack_root();
            Ok(Self::Filesystem { store, root })
        }
        #[cfg(target_arch = "wasm32")]
        {
            Ok(Self::Memory(std::sync::Arc::new(
                crate::state::model_hub::MemoryModelHubStore::new(),
            )))
        }
    }

    fn boxed(&self) -> Box<dyn ModelHubStore> {
        match self {
            #[cfg(not(target_arch = "wasm32"))]
            Self::Filesystem { store, .. } => Box::new(store.clone()),
            #[cfg(any(test, target_arch = "wasm32"))]
            Self::Memory(store) => Box::new(std::sync::Arc::clone(store)),
        }
    }

    fn pack_root(&self) -> Option<std::path::PathBuf> {
        match self {
            #[cfg(not(target_arch = "wasm32"))]
            Self::Filesystem { root, .. } => Some(root.clone()),
            #[cfg(any(test, target_arch = "wasm32"))]
            Self::Memory(_) => None,
        }
    }

    /// Opens a hub over this store under the shipped trust anchor.
    pub(crate) fn open(&self) -> Result<ModelHub, ModelHubError> {
        ModelHub::open(TrustAnchor::release()?, self.boxed(), self.pack_root())
    }
}

/// The session's hub, or the reason there isn't one.
#[derive(Debug)]
pub(crate) struct ModelHubService {
    store: Option<ModelHubStoreHandle>,
    hub: Option<ModelHub>,
    unavailable: Option<String>,
}

impl ModelHubService {
    /// Opens the session hub, sweeping any interrupted install.
    pub(crate) fn open() -> Self {
        match ModelHubStoreHandle::production().and_then(|store| {
            let hub = store.open()?;
            Ok((store, hub))
        }) {
            Ok((store, hub)) => Self {
                store: Some(store),
                hub: Some(hub),
                unavailable: None,
            },
            Err(error) => Self::unavailable(unavailable_reason(&error)),
        }
    }

    /// A service with no hub at all, for tests and unconfigured builds.
    pub(crate) fn unavailable(reason: impl Into<String>) -> Self {
        Self {
            store: None,
            hub: None,
            unavailable: Some(reason.into()),
        }
    }

    /// A service over an explicit store. Tests use this to drive the real
    /// pipeline against fixture bytes; nothing about verification changes.
    #[cfg(test)]
    pub(crate) fn with_store(store: ModelHubStoreHandle, hub: ModelHub) -> Self {
        Self {
            store: Some(store),
            hub: Some(hub),
            unavailable: None,
        }
    }

    pub(crate) fn hub(&self) -> Option<&ModelHub> {
        self.hub.as_ref()
    }

    pub(crate) fn store(&self) -> Option<&ModelHubStoreHandle> {
        self.store.as_ref()
    }

    /// Why there is no hub, in words a user can act on.
    pub(crate) fn unavailable_reason(&self) -> Option<&str> {
        self.unavailable.as_deref()
    }

    /// Re-reads the store after a background operation changed it.
    ///
    /// A reload that fails does not discard the hub the session already has:
    /// the previous view is still a true description of bytes on this machine,
    /// and replacing it with nothing would turn a transient read error into a
    /// workspace that claims nothing is installed.
    pub(crate) fn reload(&mut self) -> Result<(), String> {
        let Some(store) = self.store.as_ref() else {
            return Err(self
                .unavailable
                .clone()
                .unwrap_or_else(|| "the model hub is unavailable".to_owned()));
        };
        match store.open() {
            Ok(hub) => {
                self.hub = Some(hub);
                Ok(())
            }
            Err(error) => Err(error.to_string()),
        }
    }

    /// Whole days since the cached catalog was generated.
    ///
    /// The instant comes from the *signed* snapshot, not from when the
    /// download happened, so a client cannot make a stale catalog look fresh
    /// by re-fetching the same generation.
    pub(crate) fn catalog_age_days(&self) -> Option<u64> {
        let generated = self.hub.as_ref()?.snapshot()?.generated_at.as_str();
        let generated = parse_rfc3339_seconds(generated)?;
        let now = crate::time_compat::unix_epoch().as_secs();
        Some(now.saturating_sub(generated) / SECONDS_PER_DAY)
    }

    /// Whether a workspace should refresh the catalog on open.
    ///
    /// No catalog at all counts as stale: the first thing a hub with nothing
    /// cached should do is fetch, and treating "absent" as "fresh" would make
    /// a new installation look like it had already checked.
    pub(crate) fn catalog_is_stale(&self) -> bool {
        match self.catalog_age_days() {
            Some(days) => days >= CATALOG_STALE_AFTER_DAYS,
            None => self.hub.is_some(),
        }
    }
}

/// Restates a hub failure as the reason a workspace shows.
fn unavailable_reason(error: &ModelHubError) -> String {
    match error {
        ModelHubError::TrustAnchorUnusable => {
            "This build carries an unusable model-hub signing key, so no pack can be verified. \
             Reinstall RSpice."
                .to_owned()
        }
        ModelHubError::Storage(detail) => {
            format!("RSpice could not open its model-pack storage: {detail}")
        }
        other => format!("The model hub is unavailable: {other}"),
    }
}

/// Seconds since the unix epoch for an RFC 3339 instant in UTC.
///
/// The snapshot format fixes the shape — `YYYY-MM-DDTHH:MM:SSZ` — so this
/// reads that shape and refuses anything else rather than pulling in a date
/// library to be lenient about a field the signature already constrains.
fn parse_rfc3339_seconds(value: &str) -> Option<u64> {
    let value = value.strip_suffix('Z')?;
    let (date, time) = value.split_once('T')?;
    let mut date = date.split('-');
    let year: i64 = date.next()?.parse().ok()?;
    let month: i64 = date.next()?.parse().ok()?;
    let day: i64 = date.next()?.parse().ok()?;
    if date.next().is_some() || !(1..=12).contains(&month) || !(1..=31).contains(&day) {
        return None;
    }
    let mut time = time.split(':');
    let hour: i64 = time.next()?.parse().ok()?;
    let minute: i64 = time.next()?.parse().ok()?;
    let second: i64 = time
        .next()?
        .split('.')
        .next()
        .unwrap_or_default()
        .parse()
        .ok()?;
    if time.next().is_some() || hour > 23 || minute > 59 || second > 60 {
        return None;
    }

    // Days from the civil calendar, by Howard Hinnant's algorithm: the shift
    // to a March-based year makes the leap day the last day of the year, which
    // is what removes every special case from the arithmetic.
    let year = if month <= 2 { year - 1 } else { year };
    let era = if year >= 0 { year } else { year - 399 } / 400;
    let year_of_era = year - era * 400;
    let day_of_year = (153 * (if month > 2 { month - 3 } else { month + 9 }) + 2) / 5 + day - 1;
    let day_of_era = year_of_era * 365 + year_of_era / 4 - year_of_era / 100 + day_of_year;
    let days = era * 146_097 + day_of_era - 719_468;
    u64::try_from(days * 86_400 + hour * 3600 + minute * 60 + second).ok()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn civil_instants_convert_to_the_unix_epoch_seconds_they_name() {
        assert_eq!(parse_rfc3339_seconds("1970-01-01T00:00:00Z"), Some(0));
        assert_eq!(
            parse_rfc3339_seconds("2026-08-15T09:30:00Z"),
            Some(1_786_786_200)
        );
        // A leap day, and the instant immediately after it.
        assert_eq!(
            parse_rfc3339_seconds("2024-02-29T00:00:00Z"),
            Some(1_709_164_800)
        );
        assert_eq!(
            parse_rfc3339_seconds("2024-03-01T00:00:00Z"),
            Some(1_709_251_200)
        );
        assert_eq!(
            parse_rfc3339_seconds("2026-08-15T09:30:00.123Z"),
            Some(1_786_786_200)
        );
    }

    #[test]
    fn a_malformed_instant_is_refused_rather_than_guessed() {
        for malformed in [
            "",
            "2026-08-15T09:30:00",
            "2026-08-15 09:30:00Z",
            "2026-13-15T09:30:00Z",
            "2026-08-15T25:30:00Z",
            "not-a-date",
        ] {
            assert_eq!(
                parse_rfc3339_seconds(malformed),
                None,
                "{malformed:?} must not parse"
            );
        }
    }

    #[test]
    fn an_unavailable_hub_states_the_reason_and_serves_no_state() {
        let service = ModelHubService::unavailable("storage is denied in this browser");
        assert!(service.hub().is_none());
        assert!(service.store().is_none());
        assert_eq!(
            service.unavailable_reason(),
            Some("storage is denied in this browser")
        );
        // Nothing to be stale about, and nothing to refresh.
        assert!(!service.catalog_is_stale());
        assert_eq!(service.catalog_age_days(), None);
    }

    #[test]
    fn a_hub_with_no_cached_catalog_asks_to_refresh() {
        let store = ModelHubStoreHandle::Memory(std::sync::Arc::new(
            crate::state::model_hub::MemoryModelHubStore::new(),
        ));
        let hub = store
            .open()
            .expect("a memory hub opens under the release anchor");
        let service = ModelHubService::with_store(store, hub);
        assert!(service.hub().is_some());
        assert_eq!(service.catalog_age_days(), None);
        assert!(service.catalog_is_stale());
    }
}
