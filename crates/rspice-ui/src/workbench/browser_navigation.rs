//! Browser location and history adapter for canonical workbench routes.
//!
//! The pure [`LocationSearch`] parser deliberately treats the route-owned
//! query keys as a closed protocol while retaining every unrelated query
//! parameter byte-for-byte. Browser APIs are isolated behind `wasm32` so the
//! protocol receives ordinary native unit-test coverage.

use std::{borrow::Cow, fmt, str::FromStr};

#[cfg(any(test, target_arch = "wasm32"))]
use std::collections::BTreeMap;

#[cfg(any(test, target_arch = "wasm32"))]
use serde::{Deserialize, Serialize};

use super::surface_route::{SurfaceRoute, SurfaceRouteParseError};

const VIEW_KEY: &str = "view";
const SURFACE_KEY: &str = "surface";
const OBJECT_KIND_KEY: &str = "object-kind";
const OBJECT_ID_KEY: &str = "object-id";

#[cfg(any(test, target_arch = "wasm32"))]
const HISTORY_STATE_NAMESPACE: &str = "rspice.surface-navigation";
#[cfg(any(test, target_arch = "wasm32"))]
const HISTORY_STATE_VERSION: u8 = 1;
#[cfg(any(test, target_arch = "wasm32"))]
const OWNED_HISTORY_LIMIT: i32 = 32;
#[cfg(any(test, target_arch = "wasm32"))]
const TRAVERSAL_WATCHDOG_TIMEOUT_MS: f64 = 5_000.0;

#[cfg(any(test, target_arch = "wasm32"))]
#[derive(Debug, Clone, Copy, Default, PartialEq)]
struct TraversalWatchdog {
    deadline_ms: Option<f64>,
}

#[cfg(any(test, target_arch = "wasm32"))]
impl TraversalWatchdog {
    fn arm(&mut self, now_ms: f64) {
        self.deadline_ms = now_ms
            .is_finite()
            .then_some(now_ms + TRAVERSAL_WATCHDOG_TIMEOUT_MS);
    }

    fn clear(&mut self) {
        self.deadline_ms = None;
    }

    fn expired(self, now_ms: f64) -> bool {
        now_ms.is_finite() && self.deadline_ms.is_some_and(|deadline| now_ms >= deadline)
    }
}

/// Keep the Rust callback registration alive until the external event adapter
/// confirms that its matching listener was removed. A failed adapter call must
/// leave the registration intact so a later teardown attempt can use the same
/// callback identity.
#[cfg(any(test, target_arch = "wasm32"))]
fn remove_listener_registration<T, E>(
    slot: &std::cell::RefCell<Option<T>>,
    remove: impl FnOnce(&T) -> Result<(), E>,
) -> Result<(), E> {
    let registration = slot.borrow();
    if let Some(listener) = registration.as_ref() {
        remove(listener)?;
    }
    drop(registration);
    slot.borrow_mut().take();
    Ok(())
}

/// A validated browser query with at most one canonical workbench route.
///
/// Route-owned parameters are normalized when serialized. Parameters owned by
/// the host page (for example `locale`, test controls, or campaign tags) keep
/// their original spelling, encoding, order, and duplicates.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct LocationSearch {
    route: Option<SurfaceRoute>,
    unrelated_parameters: Vec<String>,
}

impl LocationSearch {
    /// Parse a browser `Location.search` value. An empty string means that the
    /// URL has no query; a non-empty value must include its leading `?`.
    pub fn parse(search: &str) -> Result<Self, BrowserNavigationError> {
        search.parse()
    }

    /// The canonical route selected by this query, if one is present.
    #[must_use]
    pub const fn route(&self) -> Option<SurfaceRoute> {
        self.route
    }

    /// Raw host-owned parameters in their original order.
    pub fn unrelated_parameters(&self) -> impl ExactSizeIterator<Item = &str> {
        self.unrelated_parameters.iter().map(String::as_str)
    }

    /// Return an equivalent query selecting `route`.
    #[must_use]
    pub fn with_route(&self, route: SurfaceRoute) -> Self {
        Self {
            route: Some(route),
            unrelated_parameters: self.unrelated_parameters.clone(),
        }
    }

    /// Return the host-owned portion of this query with no workbench route.
    #[must_use]
    pub fn without_route(&self) -> Self {
        Self {
            route: None,
            unrelated_parameters: self.unrelated_parameters.clone(),
        }
    }

    fn render(&self) -> String {
        let mut parameters = Vec::with_capacity(
            self.unrelated_parameters.len() + usize::from(self.route.is_some()) * 3,
        );
        if let Some(route) = self.route {
            let canonical = route.to_string();
            parameters.extend(
                canonical
                    .strip_prefix('?')
                    .unwrap_or(&canonical)
                    .split('&')
                    .map(str::to_owned),
            );
        }
        parameters.extend(self.unrelated_parameters.iter().cloned());

        if parameters.is_empty() {
            String::new()
        } else {
            format!("?{}", parameters.join("&"))
        }
    }
}

impl fmt::Display for LocationSearch {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter.write_str(&self.render())
    }
}

impl FromStr for LocationSearch {
    type Err = BrowserNavigationError;

    fn from_str(search: &str) -> Result<Self, Self::Err> {
        if search.is_empty() {
            return Ok(Self {
                route: None,
                unrelated_parameters: Vec::new(),
            });
        }
        let Some(query) = search.strip_prefix('?') else {
            return Err(BrowserNavigationError::MissingQueryPrefix);
        };
        if query.is_empty() || search.trim() != search || search.contains('#') {
            return Err(BrowserNavigationError::MalformedSearch(search.to_owned()));
        }

        let mut view = None;
        let mut surface = None;
        let mut object_kind = None;
        let mut object_id = None;
        let mut unrelated_parameters = Vec::new();

        for (index, parameter) in query.split('&').enumerate() {
            if parameter.is_empty() {
                return Err(BrowserNavigationError::MalformedParameter(
                    parameter.to_owned(),
                ));
            }
            let raw_key = parameter.split_once('=').map_or(parameter, |(key, _)| key);
            if raw_key.is_empty() {
                return Err(BrowserNavigationError::MalformedParameter(
                    parameter.to_owned(),
                ));
            }
            let decoded_key = decode_query_key(raw_key)?;
            let Some(canonical_key) = reserved_key(&decoded_key) else {
                unrelated_parameters.push(parameter.to_owned());
                continue;
            };

            if raw_key != canonical_key {
                return Err(BrowserNavigationError::NonCanonicalReservedKey {
                    received: raw_key.to_owned(),
                    expected: canonical_key,
                });
            }
            let Some((_, value)) = parameter.split_once('=') else {
                return Err(BrowserNavigationError::MalformedParameter(
                    parameter.to_owned(),
                ));
            };
            if value.is_empty() || value.contains('=') {
                return Err(BrowserNavigationError::MalformedParameter(
                    parameter.to_owned(),
                ));
            }

            let slot = match canonical_key {
                VIEW_KEY => &mut view,
                SURFACE_KEY => &mut surface,
                OBJECT_KIND_KEY => &mut object_kind,
                OBJECT_ID_KEY => &mut object_id,
                _ => {
                    return Err(BrowserNavigationError::InvalidHistoryState(
                        "reserved query-key classification was inconsistent".to_owned(),
                    ));
                }
            };
            if slot.replace((index, value)).is_some() {
                return Err(BrowserNavigationError::DuplicateParameter(canonical_key));
            }
        }

        if view.is_some() && surface.is_some() {
            return Err(BrowserNavigationError::ConflictingSelectors);
        }
        let selector = view
            .map(|(index, value)| (index, VIEW_KEY, value))
            .or_else(|| surface.map(|(index, value)| (index, SURFACE_KEY, value)));

        let route = match (selector, object_kind, object_id) {
            (None, None, None) => None,
            (None, _, _) => return Err(BrowserNavigationError::OrphanedObjectReference),
            (Some(_), Some(_), None) | (Some(_), None, Some(_)) => {
                return Err(BrowserNavigationError::PartialObjectReference);
            }
            (Some((_, key, value)), None, None) => {
                Some(format!("?{key}={value}").parse::<SurfaceRoute>()?)
            }
            (
                Some((selector_index, key, value)),
                Some((kind_index, kind)),
                Some((id_index, id)),
            ) => {
                if !(selector_index < kind_index && kind_index < id_index) {
                    return Err(BrowserNavigationError::NonCanonicalRouteOrder);
                }
                Some(
                    format!("?{key}={value}&{OBJECT_KIND_KEY}={kind}&{OBJECT_ID_KEY}={id}")
                        .parse::<SurfaceRoute>()?,
                )
            }
        };

        Ok(Self {
            route,
            unrelated_parameters,
        })
    }
}

/// Parse only the canonical route while still validating the entire query.
pub fn route_from_search(search: &str) -> Result<Option<SurfaceRoute>, BrowserNavigationError> {
    Ok(LocationSearch::parse(search)?.route())
}

/// Replace or add the canonical route without losing host-owned parameters.
pub fn search_with_route(
    search: &str,
    route: SurfaceRoute,
) -> Result<String, BrowserNavigationError> {
    Ok(LocationSearch::parse(search)?.with_route(route).to_string())
}

/// Remove the canonical route without losing host-owned parameters.
pub fn search_without_route(search: &str) -> Result<String, BrowserNavigationError> {
    Ok(LocationSearch::parse(search)?.without_route().to_string())
}

fn reserved_key(decoded_key: &str) -> Option<&'static str> {
    match decoded_key {
        VIEW_KEY => Some(VIEW_KEY),
        SURFACE_KEY => Some(SURFACE_KEY),
        OBJECT_KIND_KEY => Some(OBJECT_KIND_KEY),
        OBJECT_ID_KEY => Some(OBJECT_ID_KEY),
        _ => None,
    }
}

/// Decode query keys only, so percent-encoded aliases of route-owned keys
/// cannot evade duplicate and conflict checks. Values of unrelated parameters
/// remain opaque and are never decoded or rewritten.
fn decode_query_key(raw: &str) -> Result<Cow<'_, str>, BrowserNavigationError> {
    if !raw
        .as_bytes()
        .iter()
        .any(|byte| matches!(byte, b'%' | b'+'))
    {
        return Ok(Cow::Borrowed(raw));
    }

    let bytes = raw.as_bytes();
    let mut decoded = Vec::with_capacity(bytes.len());
    let mut index = 0;
    while index < bytes.len() {
        match bytes[index] {
            b'%' => {
                let Some(high) = bytes.get(index + 1).and_then(|byte| hex_value(*byte)) else {
                    return Err(BrowserNavigationError::MalformedPercentEncoding(
                        raw.to_owned(),
                    ));
                };
                let Some(low) = bytes.get(index + 2).and_then(|byte| hex_value(*byte)) else {
                    return Err(BrowserNavigationError::MalformedPercentEncoding(
                        raw.to_owned(),
                    ));
                };
                decoded.push(high * 16 + low);
                index += 3;
            }
            b'+' => {
                decoded.push(b' ');
                index += 1;
            }
            byte => {
                decoded.push(byte);
                index += 1;
            }
        }
    }

    String::from_utf8(decoded)
        .map(Cow::Owned)
        .map_err(|_| BrowserNavigationError::MalformedPercentEncoding(raw.to_owned()))
}

const fn hex_value(byte: u8) -> Option<u8> {
    match byte {
        b'0'..=b'9' => Some(byte - b'0'),
        b'a'..=b'f' => Some(byte - b'a' + 10),
        b'A'..=b'F' => Some(byte - b'A' + 10),
        _ => None,
    }
}

#[derive(Debug, Clone, PartialEq, Eq, thiserror::Error)]
pub enum BrowserNavigationError {
    #[error("browser location search must be empty or start with `?`")]
    MissingQueryPrefix,
    #[error("malformed browser location search `{0}`")]
    MalformedSearch(String),
    #[error("malformed query parameter `{0}`")]
    MalformedParameter(String),
    #[error("malformed percent encoding in query key `{0}`")]
    MalformedPercentEncoding(String),
    #[error("non-canonical route key `{received}`; expected `{expected}`")]
    NonCanonicalReservedKey {
        received: String,
        expected: &'static str,
    },
    #[error("duplicate route parameter `{0}`")]
    DuplicateParameter(&'static str),
    #[error("browser location cannot contain both `view` and `surface`")]
    ConflictingSelectors,
    #[error("object route parameters require one canonical `view` or `surface` selector")]
    OrphanedObjectReference,
    #[error("object routes require both `object-kind` and `object-id`")]
    PartialObjectReference,
    #[error("route parameters must retain selector, object-kind, object-id order")]
    NonCanonicalRouteOrder,
    #[error(transparent)]
    SurfaceRoute(#[from] SurfaceRouteParseError),
    #[error("browser navigation session is not initialized")]
    BrowserSessionUnavailable,
    #[error("invalid RSpice browser history state: {0}")]
    InvalidHistoryState(String),
    #[error("browser history entry does not belong to the active RSpice session")]
    ForeignHistoryEntry,
    #[error("browser history traversal would leave the active RSpice session")]
    HistoryBoundary,
    #[error("an external browser event interrupted an in-app history traversal")]
    HistoryTraversalInterrupted,
    #[error("browser history route does not match the active RSpice task")]
    HistoryRouteMismatch,
    #[error("browser navigation API failed: {0}")]
    BrowserApi(String),
}

/// Structured identity written into `history.state` for every RSpice-owned
/// entry. Route text is retained as an independently validated integrity
/// check; navigation never identifies an entry by route text alone.
#[cfg(any(test, target_arch = "wasm32"))]
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
struct HistoryStateWire {
    namespace: String,
    version: u8,
    session_id: String,
    entry_id: u32,
    index: i32,
    route: String,
}

#[cfg(any(test, target_arch = "wasm32"))]
#[derive(Debug, Clone, PartialEq, Eq)]
struct HistoryEntry {
    session_id: String,
    entry_id: u32,
    index: i32,
    route: SurfaceRoute,
}

#[cfg(any(test, target_arch = "wasm32"))]
impl HistoryEntry {
    fn to_wire(&self) -> HistoryStateWire {
        HistoryStateWire {
            namespace: HISTORY_STATE_NAMESPACE.to_owned(),
            version: HISTORY_STATE_VERSION,
            session_id: self.session_id.clone(),
            entry_id: self.entry_id,
            index: self.index,
            route: self.route.to_string(),
        }
    }

    fn from_wire(wire: HistoryStateWire) -> Result<Self, BrowserNavigationError> {
        if wire.namespace != HISTORY_STATE_NAMESPACE || wire.version != HISTORY_STATE_VERSION {
            return Err(BrowserNavigationError::ForeignHistoryEntry);
        }
        let parsed_session = uuid::Uuid::parse_str(&wire.session_id).map_err(|_| {
            BrowserNavigationError::InvalidHistoryState(
                "sessionId is not a canonical UUID".to_owned(),
            )
        })?;
        if parsed_session.hyphenated().to_string() != wire.session_id {
            return Err(BrowserNavigationError::InvalidHistoryState(
                "sessionId is not canonical".to_owned(),
            ));
        }
        if wire.entry_id == 0 || wire.index < 0 {
            return Err(BrowserNavigationError::InvalidHistoryState(
                "entryId and index must be positive and non-negative, respectively".to_owned(),
            ));
        }
        let route = wire.route.parse::<SurfaceRoute>()?;
        if route.to_string() != wire.route {
            return Err(BrowserNavigationError::InvalidHistoryState(
                "route is not canonical".to_owned(),
            ));
        }
        Ok(Self {
            session_id: wire.session_id,
            entry_id: wire.entry_id,
            index: wire.index,
            route,
        })
    }
}

/// Pure transaction model behind the browser adapter. Plans are committed
/// only after the corresponding browser API succeeds, so adapter failures do
/// not advance identity, route, or traversal state.
#[cfg(any(test, target_arch = "wasm32"))]
#[derive(Debug, Clone)]
struct HistorySession {
    session_id: String,
    next_entry_id: u32,
    current: HistoryEntry,
    entries: BTreeMap<i32, HistoryEntry>,
    pending_traversal_index: Option<i32>,
}

#[cfg(any(test, target_arch = "wasm32"))]
impl HistorySession {
    fn new(session_id: String, route: SurfaceRoute) -> Result<Self, BrowserNavigationError> {
        let parsed_session = uuid::Uuid::parse_str(&session_id).map_err(|_| {
            BrowserNavigationError::InvalidHistoryState(
                "generated session ID is not a UUID".to_owned(),
            )
        })?;
        if parsed_session.hyphenated().to_string() != session_id {
            return Err(BrowserNavigationError::InvalidHistoryState(
                "generated session ID is not canonical".to_owned(),
            ));
        }
        let current = HistoryEntry {
            session_id: session_id.clone(),
            entry_id: 1,
            index: 0,
            route,
        };
        Ok(Self {
            session_id,
            next_entry_id: 2,
            entries: BTreeMap::from([(0, current.clone())]),
            current,
            pending_traversal_index: None,
        })
    }

    fn plan_push(&self, route: SurfaceRoute) -> Result<HistoryEntry, BrowserNavigationError> {
        if self.next_entry_id == u32::MAX {
            return Err(BrowserNavigationError::InvalidHistoryState(
                "history entry identity exhausted".to_owned(),
            ));
        }
        Ok(HistoryEntry {
            session_id: self.session_id.clone(),
            entry_id: self.next_entry_id,
            index: self.current.index.checked_add(1).ok_or_else(|| {
                BrowserNavigationError::InvalidHistoryState("history index exhausted".to_owned())
            })?,
            route,
        })
    }

    fn commit_push(&mut self, entry: HistoryEntry) -> Result<(), BrowserNavigationError> {
        if entry.session_id != self.session_id
            || entry.entry_id != self.next_entry_id
            || self.current.index.checked_add(1) != Some(entry.index)
        {
            return Err(BrowserNavigationError::InvalidHistoryState(
                "push plan no longer matches the active session".to_owned(),
            ));
        }
        let next_entry_id = self.next_entry_id.checked_add(1).ok_or_else(|| {
            BrowserNavigationError::InvalidHistoryState(
                "history entry identity exhausted during commit".to_owned(),
            )
        })?;
        self.entries.retain(|index, _| *index <= self.current.index);
        self.entries.insert(entry.index, entry.clone());
        let oldest_owned = entry.index.saturating_sub(OWNED_HISTORY_LIMIT);
        self.entries.retain(|index, _| *index >= oldest_owned);
        self.next_entry_id = next_entry_id;
        self.current = entry;
        self.pending_traversal_index = None;
        Ok(())
    }

    fn plan_replace(&self, route: SurfaceRoute) -> HistoryEntry {
        HistoryEntry {
            route,
            ..self.current.clone()
        }
    }

    fn commit_replace(&mut self, entry: HistoryEntry) -> Result<(), BrowserNavigationError> {
        if entry.session_id != self.session_id
            || entry.entry_id != self.current.entry_id
            || entry.index != self.current.index
        {
            return Err(BrowserNavigationError::InvalidHistoryState(
                "replace plan no longer matches the active session".to_owned(),
            ));
        }
        self.entries.insert(entry.index, entry.clone());
        self.current = entry;
        self.pending_traversal_index = None;
        Ok(())
    }

    fn plan_traversal(
        &self,
        delta: i32,
        expected_route: SurfaceRoute,
    ) -> Result<HistoryEntry, BrowserNavigationError> {
        let target_index = self
            .current
            .index
            .checked_add(delta)
            .ok_or(BrowserNavigationError::HistoryBoundary)?;
        let target = self
            .entries
            .get(&target_index)
            .ok_or(BrowserNavigationError::HistoryBoundary)?;
        if target.route != expected_route {
            return Err(BrowserNavigationError::HistoryRouteMismatch);
        }
        Ok(target.clone())
    }

    fn commit_traversal_started(
        &mut self,
        target: &HistoryEntry,
    ) -> Result<(), BrowserNavigationError> {
        if target.session_id != self.session_id || self.entries.get(&target.index) != Some(target) {
            return Err(BrowserNavigationError::InvalidHistoryState(
                "traversal plan no longer matches the active session".to_owned(),
            ));
        }
        self.pending_traversal_index = Some(target.index);
        Ok(())
    }

    fn reconcile_pop(
        &mut self,
        location_route: SurfaceRoute,
        entry: HistoryEntry,
    ) -> Result<(i32, bool), BrowserNavigationError> {
        let pending_traversal_index = self.pending_traversal_index.take();
        if entry.session_id != self.session_id {
            return Err(BrowserNavigationError::ForeignHistoryEntry);
        }
        let Some(known) = self.entries.get(&entry.index) else {
            return Err(BrowserNavigationError::ForeignHistoryEntry);
        };
        if known != &entry {
            return Err(BrowserNavigationError::InvalidHistoryState(
                "entry identity does not match the session ledger".to_owned(),
            ));
        }
        if entry.route != location_route {
            return Err(BrowserNavigationError::HistoryRouteMismatch);
        }
        if pending_traversal_index.is_some_and(|expected| expected != entry.index) {
            return Err(BrowserNavigationError::HistoryTraversalInterrupted);
        }
        let initiated_by_app = pending_traversal_index.is_some();
        let delta = entry
            .index
            .checked_sub(self.current.index)
            .ok_or(BrowserNavigationError::HistoryBoundary)?;
        self.current = entry;
        Ok((delta, initiated_by_app))
    }

    fn traversal_in_flight(&self) -> bool {
        self.pending_traversal_index.is_some()
    }
}

/// Result of a browser history write.
#[cfg(target_arch = "wasm32")]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum HistoryWrite {
    Updated,
    Unchanged,
}

#[cfg(target_arch = "wasm32")]
mod browser {
    use std::{
        cell::{Cell, RefCell},
        collections::VecDeque,
    };

    use wasm_bindgen::{JsCast as _, JsValue, closure::Closure};

    use super::{
        BrowserNavigationError, HistoryEntry, HistorySession, HistoryStateWire, HistoryWrite,
        LocationSearch, SurfaceRoute, TraversalWatchdog, remove_listener_registration,
    };

    const POPSTATE_QUEUE_LIMIT: usize = 32;
    type PopstateCallback = Closure<dyn FnMut(web_sys::PopStateEvent)>;

    thread_local! {
        static POPSTATE_QUEUE: RefCell<VecDeque<Result<BrowserPopEvent, BrowserNavigationError>>> =
            const { RefCell::new(VecDeque::new()) };
        static POPSTATE_LISTENER: RefCell<Option<PopstateCallback>> =
            const { RefCell::new(None) };
        static POPSTATE_REPAINT: RefCell<Option<egui::Context>> =
            const { RefCell::new(None) };
        static HISTORY_SESSION: RefCell<Option<HistorySession>> = const { RefCell::new(None) };
        static HISTORY_SESSION_READY: Cell<bool> = const { Cell::new(false) };
        static TRAVERSAL_WATCHDOG: RefCell<TraversalWatchdog> =
            const { RefCell::new(TraversalWatchdog { deadline_ms: None }) };
    }

    #[derive(Debug, Clone, Copy, PartialEq, Eq)]
    enum HistoryMode {
        Push,
        Replace,
    }

    /// One fully authenticated browser traversal. `delta` is derived from the
    /// entry indices in `history.state`, never inferred from route text.
    #[derive(Debug, Clone, Copy, PartialEq, Eq)]
    pub struct BrowserPopEvent {
        route: SurfaceRoute,
        delta: i32,
        initiated_by_app: bool,
    }

    impl BrowserPopEvent {
        #[must_use]
        pub const fn route(self) -> SurfaceRoute {
            self.route
        }

        #[must_use]
        pub const fn delta(self) -> i32 {
            self.delta
        }

        #[must_use]
        pub const fn initiated_by_app(self) -> bool {
            self.initiated_by_app
        }
    }

    pub fn current_location() -> Result<LocationSearch, BrowserNavigationError> {
        let search = browser_window()?
            .location()
            .search()
            .map_err(|error| browser_error("read location.search", error))?;
        LocationSearch::parse(&search)
    }

    /// Replace the current host entry with the root of a fresh, process-local
    /// RSpice history session. This intentionally does not import persisted
    /// in-app stacks: a newly loaded tab owns exactly one browser entry.
    pub fn restart_history_session(route: SurfaceRoute) -> Result<(), BrowserNavigationError> {
        let result = (|| {
            if !popstate_listener_ready() {
                return Err(BrowserNavigationError::BrowserApi(
                    "popstate listener is unavailable".to_owned(),
                ));
            }
            let session_id = uuid::Uuid::new_v4().hyphenated().to_string();
            let session = HistorySession::new(session_id, route)?;
            let window = browser_window()?;
            let next_url = url_for_route(&window, route, true)?;
            write_entry(&window, &session.current, &next_url, HistoryMode::Replace)?;

            HISTORY_SESSION.with(|slot| *slot.borrow_mut() = Some(session));
            POPSTATE_QUEUE.with(|queue| queue.borrow_mut().clear());
            Ok(())
        })();
        if result.is_ok() {
            HISTORY_SESSION_READY.with(|ready| ready.set(true));
            TRAVERSAL_WATCHDOG.with(|watchdog| watchdog.borrow_mut().clear());
        } else {
            disable_history_session();
        }
        result
    }

    #[must_use]
    pub fn history_session_ready() -> bool {
        HISTORY_SESSION_READY.with(Cell::get) && popstate_listener_ready()
    }

    #[must_use]
    pub fn traversal_in_flight() -> bool {
        if !history_session_ready() {
            return false;
        }
        HISTORY_SESSION.with(|slot| {
            slot.borrow()
                .as_ref()
                .is_some_and(HistorySession::traversal_in_flight)
        })
    }

    /// Fail closed if `history.go` was accepted but no authenticating
    /// `popstate` arrived within the bounded interval.
    #[must_use]
    pub fn traversal_watchdog_expired() -> bool {
        if !traversal_in_flight() {
            return false;
        }
        let expired =
            TRAVERSAL_WATCHDOG.with(|watchdog| watchdog.borrow().expired(js_sys::Date::now()));
        if expired {
            disable_history_session();
        }
        expired
    }

    /// Route currently committed by the browser adapter. This remains the
    /// rollback authority if a subsequent push/replace/traverse API fails.
    pub fn active_browser_route() -> Result<SurfaceRoute, BrowserNavigationError> {
        require_history_session_ready()?;
        HISTORY_SESSION.with(|slot| {
            slot.borrow()
                .as_ref()
                .map(|session| session.current.route)
                .ok_or(BrowserNavigationError::BrowserSessionUnavailable)
        })
    }

    /// Install one process-local popstate listener. Calling this again updates
    /// the repaint target without registering a duplicate DOM listener.
    pub fn install_popstate_listener(
        repaint_context: &egui::Context,
    ) -> Result<(), BrowserNavigationError> {
        POPSTATE_REPAINT.with(|slot| {
            *slot.borrow_mut() = Some(repaint_context.clone());
        });
        ensure_popstate_listener()
    }

    /// Retry listener installation after a transient browser failure. The
    /// repaint context is retained by the initial install attempt.
    pub fn ensure_popstate_listener() -> Result<(), BrowserNavigationError> {
        if POPSTATE_LISTENER.with(|slot| slot.borrow().is_some()) {
            return Ok(());
        }

        if POPSTATE_REPAINT.with(|slot| slot.borrow().is_none()) {
            disable_history_session();
            return Err(BrowserNavigationError::BrowserApi(
                "popstate repaint context is unavailable".to_owned(),
            ));
        }

        let window = match browser_window() {
            Ok(window) => window,
            Err(error) => {
                disable_history_session();
                return Err(error);
            }
        };
        let callback =
            Closure::<dyn FnMut(web_sys::PopStateEvent)>::new(|event: web_sys::PopStateEvent| {
                // `popstate` only queues an authenticated entry. It never writes
                // history, so applying a queued route cannot create an echo loop.
                queue_popstate(event.state());
                POPSTATE_REPAINT.with(|slot| {
                    if let Some(context) = slot.borrow().as_ref() {
                        context.request_repaint();
                    }
                });
            });
        if let Err(error) =
            window.add_event_listener_with_callback("popstate", callback.as_ref().unchecked_ref())
        {
            disable_history_session();
            return Err(browser_error("install popstate listener", error));
        }
        POPSTATE_LISTENER.with(|slot| {
            *slot.borrow_mut() = Some(callback);
        });
        Ok(())
    }

    /// Remove the listener and discard undelivered location events.
    pub fn uninstall_popstate_listener() -> Result<(), BrowserNavigationError> {
        if popstate_listener_ready() {
            let window = browser_window()?;
            POPSTATE_LISTENER.with(|slot| {
                remove_listener_registration(slot, |listener| {
                    window
                        .remove_event_listener_with_callback(
                            "popstate",
                            listener.as_ref().unchecked_ref(),
                        )
                        .map_err(|error| browser_error("remove popstate listener", error))
                })
            })?;
        }
        POPSTATE_REPAINT.with(|slot| *slot.borrow_mut() = None);
        POPSTATE_QUEUE.with(|queue| queue.borrow_mut().clear());
        HISTORY_SESSION.with(|slot| *slot.borrow_mut() = None);
        HISTORY_SESSION_READY.with(|ready| ready.set(false));
        TRAVERSAL_WATCHDOG.with(|watchdog| watchdog.borrow_mut().clear());
        Ok(())
    }

    /// Consume the oldest browser back/forward event, if any.
    pub fn poll_popstate() -> Option<Result<BrowserPopEvent, BrowserNavigationError>> {
        POPSTATE_QUEUE.with(|queue| queue.borrow_mut().pop_front())
    }

    pub fn push_route(route: SurfaceRoute) -> Result<HistoryWrite, BrowserNavigationError> {
        write_route(route, HistoryMode::Push)
    }

    pub fn replace_route(route: SurfaceRoute) -> Result<HistoryWrite, BrowserNavigationError> {
        write_route(route, HistoryMode::Replace)
    }

    /// Traverse existing browser history for an in-app Back/Forward command.
    /// The ensuing `popstate` event is the sole route-application owner.
    pub fn traverse_history(
        delta: i32,
        expected_route: SurfaceRoute,
    ) -> Result<(), BrowserNavigationError> {
        require_history_session_ready()?;
        if delta == 0 {
            return if active_browser_route()? == expected_route {
                Ok(())
            } else {
                Err(BrowserNavigationError::HistoryRouteMismatch)
            };
        }
        let window = browser_window()?;
        let target = HISTORY_SESSION.with(|slot| {
            let session = slot.borrow();
            let session = session
                .as_ref()
                .ok_or(BrowserNavigationError::BrowserSessionUnavailable)?;
            ensure_browser_matches_session(&window, session)?;
            session.plan_traversal(delta, expected_route)
        })?;
        window
            .history()
            .map_err(|error| browser_error("read window.history", error))?
            .go_with_delta(delta)
            .map_err(|error| browser_error("traverse browser history", error))?;
        HISTORY_SESSION.with(|slot| {
            let mut session = slot.borrow_mut();
            let session = session
                .as_mut()
                .ok_or(BrowserNavigationError::BrowserSessionUnavailable)?;
            session.commit_traversal_started(&target)
        })?;
        TRAVERSAL_WATCHDOG.with(|watchdog| watchdog.borrow_mut().arm(js_sys::Date::now()));
        Ok(())
    }

    fn write_route(
        route: SurfaceRoute,
        mode: HistoryMode,
    ) -> Result<HistoryWrite, BrowserNavigationError> {
        require_history_session_ready()?;
        let window = browser_window()?;
        let planned = HISTORY_SESSION.with(|slot| {
            let session = slot.borrow();
            let session = session
                .as_ref()
                .ok_or(BrowserNavigationError::BrowserSessionUnavailable)?;
            ensure_browser_matches_session(&window, session)?;
            if mode == HistoryMode::Push && session.current.route == route {
                return Ok::<Option<HistoryEntry>, BrowserNavigationError>(None);
            }
            Ok(Some(match mode {
                HistoryMode::Push => session.plan_push(route)?,
                HistoryMode::Replace => session.plan_replace(route),
            }))
        })?;
        let Some(planned) = planned else {
            return Ok(HistoryWrite::Unchanged);
        };

        let next_url = url_for_route(&window, route, false)?;
        write_entry(&window, &planned, &next_url, mode)?;
        HISTORY_SESSION.with(|slot| {
            let mut session = slot.borrow_mut();
            let session = session
                .as_mut()
                .ok_or(BrowserNavigationError::BrowserSessionUnavailable)?;
            match mode {
                HistoryMode::Push => session.commit_push(planned),
                HistoryMode::Replace => session.commit_replace(planned),
            }
        })?;
        Ok(HistoryWrite::Updated)
    }

    fn queue_popstate(state: JsValue) {
        if !history_session_ready() {
            POPSTATE_QUEUE.with(|queue| {
                let mut queue = queue.borrow_mut();
                if queue.is_empty() {
                    queue.push_back(Err(BrowserNavigationError::BrowserSessionUnavailable));
                }
            });
            return;
        }

        let overflowed = POPSTATE_QUEUE.with(|queue| {
            let mut queue = queue.borrow_mut();
            if queue.len() < POPSTATE_QUEUE_LIMIT {
                false
            } else {
                queue.clear();
                queue.push_back(Err(BrowserNavigationError::InvalidHistoryState(
                    "browser history event queue overflowed; a canonical restart is required"
                        .to_owned(),
                )));
                true
            }
        });
        if overflowed {
            // Do not reconcile the event after overflow: the address has moved
            // beyond the complete ordered event stream, so the ledger is no
            // longer authoritative until the application restarts it.
            disable_history_session();
            return;
        }

        let reconciled = reconcile_popstate(state);
        TRAVERSAL_WATCHDOG.with(|watchdog| watchdog.borrow_mut().clear());
        if reconciled.is_err() {
            disable_history_session();
        }
        POPSTATE_QUEUE.with(|queue| queue.borrow_mut().push_back(reconciled));
    }

    fn reconcile_popstate(state: JsValue) -> Result<BrowserPopEvent, BrowserNavigationError> {
        let location = current_location()?;
        let location_route = location
            .route()
            .ok_or(BrowserNavigationError::HistoryRouteMismatch)?;
        let entry = decode_entry(state)?;
        let (delta, initiated_by_app) = HISTORY_SESSION.with(|slot| {
            let mut session = slot.borrow_mut();
            session
                .as_mut()
                .ok_or(BrowserNavigationError::BrowserSessionUnavailable)?
                .reconcile_pop(location_route, entry)
        })?;
        Ok(BrowserPopEvent {
            route: location_route,
            delta,
            initiated_by_app,
        })
    }

    fn ensure_browser_matches_session(
        window: &web_sys::Window,
        session: &HistorySession,
    ) -> Result<(), BrowserNavigationError> {
        let location = current_location()?;
        if location.route() != Some(session.current.route) {
            return Err(BrowserNavigationError::HistoryRouteMismatch);
        }
        let state = window
            .history()
            .map_err(|error| browser_error("read window.history", error))?
            .state()
            .map_err(|error| browser_error("read history.state", error))?;
        if decode_entry(state)? != session.current {
            return Err(BrowserNavigationError::InvalidHistoryState(
                "current browser entry does not match the session ledger".to_owned(),
            ));
        }
        Ok(())
    }

    fn decode_entry(state: JsValue) -> Result<HistoryEntry, BrowserNavigationError> {
        if state.is_null() || state.is_undefined() {
            return Err(BrowserNavigationError::ForeignHistoryEntry);
        }
        let wire = serde_wasm_bindgen::from_value::<HistoryStateWire>(state).map_err(|error| {
            BrowserNavigationError::InvalidHistoryState(format!(
                "history.state is not an RSpice entry: {error}"
            ))
        })?;
        HistoryEntry::from_wire(wire)
    }

    fn url_for_route(
        window: &web_sys::Window,
        route: SurfaceRoute,
        recover_malformed_search: bool,
    ) -> Result<String, BrowserNavigationError> {
        let location = window.location();
        let current_search = location
            .search()
            .map_err(|error| browser_error("read location.search", error))?;
        let next_search = match LocationSearch::parse(&current_search) {
            Ok(query) => query.with_route(route).to_string(),
            Err(_) if recover_malformed_search => route.to_string(),
            Err(error) => return Err(error),
        };
        let pathname = location
            .pathname()
            .map_err(|error| browser_error("read location.pathname", error))?;
        let hash = location
            .hash()
            .map_err(|error| browser_error("read location.hash", error))?;
        Ok(format!("{pathname}{next_search}{hash}"))
    }

    fn write_entry(
        window: &web_sys::Window,
        entry: &HistoryEntry,
        url: &str,
        mode: HistoryMode,
    ) -> Result<(), BrowserNavigationError> {
        let state = serde_wasm_bindgen::to_value(&entry.to_wire()).map_err(|error| {
            BrowserNavigationError::InvalidHistoryState(format!(
                "could not serialize history.state: {error}"
            ))
        })?;
        let history = window
            .history()
            .map_err(|error| browser_error("read window.history", error))?;
        match mode {
            HistoryMode::Push => history
                .push_state_with_url(&state, "", Some(url))
                .map_err(|error| browser_error("push browser history", error)),
            HistoryMode::Replace => history
                .replace_state_with_url(&state, "", Some(url))
                .map_err(|error| browser_error("replace browser history", error)),
        }
    }

    fn browser_window() -> Result<web_sys::Window, BrowserNavigationError> {
        web_sys::window().ok_or_else(|| {
            BrowserNavigationError::BrowserApi("browser window is unavailable".to_owned())
        })
    }

    fn popstate_listener_ready() -> bool {
        POPSTATE_LISTENER.with(|slot| slot.borrow().is_some())
    }

    fn disable_history_session() {
        HISTORY_SESSION_READY.with(|ready| ready.set(false));
        HISTORY_SESSION.with(|slot| *slot.borrow_mut() = None);
        TRAVERSAL_WATCHDOG.with(|watchdog| watchdog.borrow_mut().clear());
    }

    fn require_history_session_ready() -> Result<(), BrowserNavigationError> {
        if history_session_ready() {
            Ok(())
        } else {
            Err(BrowserNavigationError::BrowserSessionUnavailable)
        }
    }

    fn browser_error(operation: &str, error: JsValue) -> BrowserNavigationError {
        let detail = error.as_string().unwrap_or_else(|| format!("{error:?}"));
        BrowserNavigationError::BrowserApi(format!("{operation}: {detail}"))
    }
}

#[cfg(target_arch = "wasm32")]
pub use browser::{
    BrowserPopEvent, active_browser_route, current_location, ensure_popstate_listener,
    history_session_ready, install_popstate_listener, poll_popstate, push_route, replace_route,
    restart_history_session, traversal_in_flight, traversal_watchdog_expired, traverse_history,
    uninstall_popstate_listener,
};

#[cfg(test)]
mod tests {
    use uuid::Uuid;

    use crate::product::{ObjectRef, ProductObjectKind};

    use super::*;
    use crate::workbench::{capability_workflow::CapabilityWorkflowId, surface_catalog::SurfaceId};

    fn object_route(surface_id: SurfaceId) -> SurfaceRoute {
        let object_ref = ObjectRef::new(
            ProductObjectKind::Project,
            Uuid::parse_str("123e4567-e89b-12d3-a456-426614174000").expect("fixture UUID"),
        )
        .expect("non-nil fixture object");
        SurfaceRoute::for_object(surface_id, object_ref)
    }

    #[test]
    fn every_canonical_surface_parses_with_the_exact_selector_split() {
        for surface_id in SurfaceId::ALL {
            let search = format!("?locale=en-US&{}&stress=long", &surface_id.deep_link()[1..]);
            let parsed = LocationSearch::parse(&search).expect("canonical route parses");
            assert_eq!(parsed.route(), Some(SurfaceRoute::surface(surface_id)));
            assert_eq!(
                parsed.unrelated_parameters().collect::<Vec<_>>(),
                ["locale=en-US", "stress=long"]
            );

            let selector = parsed.route().expect("route").to_string();
            if surface_id.workspace().is_some() {
                assert!(selector.starts_with("?view="));
            } else {
                assert!(selector.starts_with("?surface="));
            }
        }
    }

    #[test]
    fn every_capability_workflow_route_preserves_host_parameters() {
        let original = "?locale=en-US&view=design&campaign=qualification";

        for workflow in CapabilityWorkflowId::ALL {
            let route = SurfaceRoute::capability_workflow(workflow);
            let search = search_with_route(original, route).expect("workflow route rewrites");
            assert_eq!(
                search,
                format!(
                    "{}&locale=en-US&campaign=qualification",
                    workflow.deep_link()
                )
            );
            assert_eq!(
                route_from_search(&search).expect("workflow route parses"),
                Some(route)
            );
        }
    }

    #[test]
    fn empty_and_host_only_searches_have_no_route_and_roundtrip() {
        for search in [
            "",
            "?locale=en-US",
            "?flag&empty=&campaign=a=b&tag=one&tag=two",
            "?na%6De=encoded&stringStress=long",
        ] {
            let parsed = LocationSearch::parse(search).expect("host query parses");
            assert_eq!(parsed.route(), None);
            assert_eq!(parsed.to_string(), search);
            assert_eq!(search_without_route(search).expect("route removal"), search);
        }
    }

    #[test]
    fn rewriting_all_routes_preserves_unrelated_parameters_exactly() {
        let original = "?locale=ar-XB&campaign=a%2Fb&flag&tag=one&tag=two&view=design";
        for surface_id in SurfaceId::ALL {
            let route = SurfaceRoute::surface(surface_id);
            let rewritten = search_with_route(original, route).expect("rewrite succeeds");
            assert_eq!(
                route_from_search(&rewritten).expect("rewritten route"),
                Some(route)
            );
            assert_eq!(
                LocationSearch::parse(&rewritten)
                    .expect("rewritten query parses")
                    .unrelated_parameters()
                    .collect::<Vec<_>>(),
                [
                    "locale=ar-XB",
                    "campaign=a%2Fb",
                    "flag",
                    "tag=one",
                    "tag=two"
                ]
            );
        }
    }

    #[test]
    fn object_routes_allow_unrelated_interleaving_but_serialize_canonically() {
        let route = object_route(SurfaceId::FeatureAvailability);
        let search = concat!(
            "?locale=en-US&surface=feature-availability&campaign=qa&",
            "object-kind=project&tag=one&",
            "object-id=123e4567-e89b-12d3-a456-426614174000"
        );
        let parsed = LocationSearch::parse(search).expect("object route parses");
        assert_eq!(parsed.route(), Some(route));
        assert_eq!(
            parsed.to_string(),
            concat!(
                "?surface=feature-availability&object-kind=project&",
                "object-id=123e4567-e89b-12d3-a456-426614174000&",
                "locale=en-US&campaign=qa&tag=one"
            )
        );
    }

    #[test]
    fn route_removal_drops_all_route_owned_parameters_only() {
        let search = concat!(
            "?locale=en-US&surface=feature-availability&",
            "object-kind=project&object-id=123e4567-e89b-12d3-a456-426614174000&",
            "campaign=a%2Fb"
        );
        assert_eq!(
            search_without_route(search).expect("route removal succeeds"),
            "?locale=en-US&campaign=a%2Fb"
        );
    }

    #[test]
    fn duplicate_conflicting_and_encoded_route_keys_fail_closed() {
        let invalid = [
            "?view=design&view=results",
            "?surface=feature-availability&surface=preferences",
            "?view=design&surface=feature-availability",
            "?%76iew=design",
            "?s%75rface=feature-availability",
            "?object%2Dkind=project",
            "?object-id=123e4567-e89b-12d3-a456-426614174000&object-id=123e4567-e89b-12d3-a456-426614174001",
        ];
        for search in invalid {
            assert!(
                LocationSearch::parse(search).is_err(),
                "accepted ambiguous query `{search}`"
            );
        }
    }

    #[test]
    fn unknown_and_noncanonical_surface_ids_fail_closed() {
        for search in [
            "?view=unknown",
            "?surface=unknown",
            "?view=Project",
            "?surface=Feature-Availability",
            "?surface=project",
            "?view=feature-availability",
            "?view=%64esign",
            "?surface=feature%2Davailability",
        ] {
            assert!(
                LocationSearch::parse(search).is_err(),
                "accepted unknown or noncanonical route `{search}`"
            );
        }
    }

    #[test]
    fn partial_or_reordered_object_routes_fail_closed() {
        let id = "123e4567-e89b-12d3-a456-426614174000";
        let invalid = [
            "?object-kind=project",
            "?object-id=123e4567-e89b-12d3-a456-426614174000",
            "?view=project&object-kind=project",
            "?view=project&object-id=123e4567-e89b-12d3-a456-426614174000",
            "?object-kind=project&view=project&object-id=123e4567-e89b-12d3-a456-426614174000",
            "?view=project&object-id=123e4567-e89b-12d3-a456-426614174000&object-kind=project",
        ];
        for search in invalid {
            assert!(
                LocationSearch::parse(search).is_err(),
                "accepted incomplete object route `{search}`"
            );
        }

        let valid = format!("?view=project&object-kind=project&object-id={id}");
        assert_eq!(
            LocationSearch::parse(&valid)
                .expect("complete object route")
                .route(),
            Some(object_route(SurfaceId::Project))
        );
    }

    #[test]
    fn malformed_search_and_reserved_parameters_fail_closed() {
        for search in [
            "?",
            "view=design",
            " ?view=design",
            "?view=design ",
            "?view=design#fragment",
            "?view",
            "?view=",
            "?view=design=extra",
            "?=value",
            "?locale=en-US&&view=design",
            "?locale=en-US&",
            "?bad%=value",
            "?bad%2=value",
            "?bad%GG=value",
            "?bad%FF=value",
        ] {
            assert!(
                LocationSearch::parse(search).is_err(),
                "accepted malformed query `{search}`"
            );
        }
    }

    #[test]
    fn rewrite_refuses_to_hide_an_invalid_existing_route() {
        let replacement = SurfaceRoute::surface(SurfaceId::Design);
        for search in [
            "?surface=unknown&locale=en-US",
            "?view=design&view=results&locale=en-US",
            "?object-kind=project&locale=en-US",
        ] {
            assert!(
                search_with_route(search, replacement).is_err(),
                "rewrote invalid source query `{search}`"
            );
        }
    }

    fn history_session(route: SurfaceRoute) -> HistorySession {
        HistorySession::new("123e4567-e89b-12d3-a456-426614174000".to_owned(), route)
            .expect("canonical fixture session")
    }

    #[test]
    fn history_plans_are_transactional_until_the_browser_write_commits() {
        let design = SurfaceRoute::surface(SurfaceId::Design);
        let results = SurfaceRoute::surface(SurfaceId::Results);
        let mut session = history_session(design);
        let planned = session.plan_push(results).expect("push can be planned");

        assert_eq!(session.current.route, design);
        assert_eq!(session.next_entry_id, 2);
        assert_eq!(session.entries.len(), 1);

        session.commit_push(planned).expect("push commits");
        assert_eq!(session.current.route, results);
        assert_eq!(session.current.entry_id, 2);
        assert_eq!(session.current.index, 1);
        assert_eq!(session.next_entry_id, 3);

        let replacement = session.plan_replace(design);
        assert_eq!(session.current.route, results);
        assert_eq!(replacement.entry_id, session.current.entry_id);
        assert_eq!(replacement.index, session.current.index);
        session
            .commit_replace(replacement)
            .expect("replace commits");
        assert_eq!(session.current.route, design);
        assert_eq!(session.next_entry_id, 3);

        let stale = session.plan_push(results).expect("stale push plan");
        let models = session
            .plan_push(SurfaceRoute::surface(SurfaceId::Models))
            .expect("models push plan");
        session.commit_push(models).expect("models push commits");
        assert!(matches!(
            session.commit_push(stale),
            Err(BrowserNavigationError::InvalidHistoryState(_))
        ));
        assert_eq!(
            session.current.route,
            SurfaceRoute::surface(SurfaceId::Models)
        );
    }

    #[test]
    fn listener_registration_survives_adapter_removal_failure() {
        let slot = std::cell::RefCell::new(Some("registered callback"));

        let failure = remove_listener_registration(&slot, |_| Err("adapter removal failed"));
        assert_eq!(failure, Err("adapter removal failed"));
        assert_eq!(slot.borrow().as_deref(), Some("registered callback"));

        remove_listener_registration(&slot, |_| Ok::<(), &str>(()))
            .expect("retry removes the same callback");
        assert!(slot.borrow().is_none());
    }

    #[test]
    fn repeated_routes_and_multi_step_pops_resolve_by_entry_identity() {
        let design = SurfaceRoute::surface(SurfaceId::Design);
        let results = SurfaceRoute::surface(SurfaceId::Results);
        let mut session = history_session(design);
        let first_design = session.current.clone();

        let results_entry = session.plan_push(results).expect("results plan");
        session
            .commit_push(results_entry)
            .expect("results push commits");
        let repeated_design = session.plan_push(design).expect("repeat plan");
        session
            .commit_push(repeated_design)
            .expect("repeated route commits");

        let (delta, initiated) = session
            .reconcile_pop(design, first_design)
            .expect("original design identity remains known");
        assert_eq!(delta, -2);
        assert!(!initiated);
        assert_eq!(session.current.index, 0);
    }

    #[test]
    fn app_traversal_is_confirmed_only_by_its_exact_target_entry() {
        let design = SurfaceRoute::surface(SurfaceId::Design);
        let results = SurfaceRoute::surface(SurfaceId::Results);
        let mut session = history_session(design);
        let design_entry = session.current.clone();
        let results_entry = session.plan_push(results).expect("results plan");
        session
            .commit_push(results_entry)
            .expect("results push commits");

        let target = session
            .plan_traversal(-1, design)
            .expect("owned back entry");
        session
            .commit_traversal_started(&target)
            .expect("traversal begins");
        let (delta, initiated) = session
            .reconcile_pop(design, design_entry)
            .expect("target pop authenticates");
        assert_eq!(delta, -1);
        assert!(initiated);
    }

    #[test]
    fn external_pop_interrupting_an_app_traversal_is_rejected_without_ledger_advance() {
        let design = SurfaceRoute::surface(SurfaceId::Design);
        let results = SurfaceRoute::surface(SurfaceId::Results);
        let models = SurfaceRoute::surface(SurfaceId::Models);
        let mut session = history_session(design);
        let design_entry = session.current.clone();
        let results_entry = session.plan_push(results).expect("results plan");
        session
            .commit_push(results_entry)
            .expect("results push commits");
        let models_entry = session.plan_push(models).expect("models plan");
        session
            .commit_push(models_entry)
            .expect("models push commits");
        let target = session
            .plan_traversal(-1, results)
            .expect("results is the requested target");
        session
            .commit_traversal_started(&target)
            .expect("traversal begins");

        assert_eq!(
            session.reconcile_pop(design, design_entry),
            Err(BrowserNavigationError::HistoryTraversalInterrupted)
        );
        assert_eq!(session.current.route, models);
        assert!(!session.traversal_in_flight());
    }

    #[test]
    fn fresh_session_refuses_to_traverse_host_or_persisted_history() {
        let design = SurfaceRoute::surface(SurfaceId::Design);
        let session = history_session(design);
        assert_eq!(
            session.plan_traversal(-1, design),
            Err(BrowserNavigationError::HistoryBoundary)
        );
        assert_eq!(
            session.plan_traversal(1, design),
            Err(BrowserNavigationError::HistoryBoundary)
        );
    }

    #[test]
    fn push_after_back_uses_a_new_id_and_quarantines_discarded_forward_state() {
        let design = SurfaceRoute::surface(SurfaceId::Design);
        let results = SurfaceRoute::surface(SurfaceId::Results);
        let models = SurfaceRoute::surface(SurfaceId::Models);
        let verify = SurfaceRoute::surface(SurfaceId::Verify);
        let mut session = history_session(design);
        let results_entry = session.plan_push(results).expect("results plan");
        session
            .commit_push(results_entry.clone())
            .expect("results push commits");
        let discarded_models = session.plan_push(models).expect("models plan");
        session
            .commit_push(discarded_models.clone())
            .expect("models push commits");
        session
            .reconcile_pop(results, results_entry)
            .expect("back to results");

        let replacement = session.plan_push(verify).expect("branch plan");
        assert_eq!(replacement.index, discarded_models.index);
        assert!(replacement.entry_id > discarded_models.entry_id);
        session
            .commit_push(replacement)
            .expect("branch push commits");

        assert!(matches!(
            session.reconcile_pop(models, discarded_models),
            Err(BrowserNavigationError::InvalidHistoryState(_))
        ));
        assert_eq!(session.current.route, verify);
    }

    #[test]
    fn history_state_rejects_foreign_namespace_and_noncanonical_identity() {
        let route = SurfaceRoute::surface(SurfaceId::Design);
        let entry = history_session(route).current;
        let mut wire = entry.to_wire();
        wire.namespace = "host-page".to_owned();
        assert_eq!(
            HistoryEntry::from_wire(wire),
            Err(BrowserNavigationError::ForeignHistoryEntry)
        );

        let mut wire = entry.to_wire();
        wire.session_id = wire.session_id.to_uppercase();
        assert!(matches!(
            HistoryEntry::from_wire(wire),
            Err(BrowserNavigationError::InvalidHistoryState(_))
        ));
    }

    #[test]
    fn traversal_watchdog_expires_at_the_bounded_deadline_and_clears() {
        let mut watchdog = TraversalWatchdog::default();
        assert!(!watchdog.expired(100.0));

        watchdog.arm(100.0);
        assert!(!watchdog.expired(100.0 + TRAVERSAL_WATCHDOG_TIMEOUT_MS - 0.1));
        assert!(watchdog.expired(100.0 + TRAVERSAL_WATCHDOG_TIMEOUT_MS));

        watchdog.clear();
        assert!(!watchdog.expired(f64::MAX));
        watchdog.arm(f64::NAN);
        assert_eq!(watchdog.deadline_ms, None);
    }
}
