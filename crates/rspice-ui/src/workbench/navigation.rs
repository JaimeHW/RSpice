//! Durable, object-aware navigation state for canonical workbench surfaces.
//!
//! The seven [`Workspace`](super::state::Workspace) values remain a rendering
//! projection for the primary shell. Navigation identity, browser history,
//! source return, and recent-task history are owned here and always use stable
//! [`SurfaceRoute`] values.

use std::collections::VecDeque;

use serde::{Deserialize, Deserializer, Serialize};

use super::{SurfaceId, SurfaceRoute};

const HISTORY_LIMIT: usize = 32;
const RECENT_LIMIT: usize = 16;
#[cfg(any(test, target_arch = "wasm32"))]
const BROWSER_EFFECT_LIMIT: usize = 64;

/// Browser-history effect requested by a committed navigation transition.
/// Runtime adapters consume this value; it is never persisted.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum BrowserHistoryEffect {
    Push(SurfaceRoute),
    Replace(SurfaceRoute),
    Traverse {
        delta: i32,
        destination: SurfaceRoute,
    },
}

/// Origin of a route transition. Browser-pop transitions must not push a
/// second browser history entry, while restore transitions canonicalize the
/// current entry with `replaceState`.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum RouteTransitionSource {
    User,
    BrowserPop,
    Restore,
}

/// Result of a route transition. Callers use this to restore focus and to
/// avoid treating same-route activation as a document mutation.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct RouteTransition {
    pub previous: SurfaceRoute,
    pub current: SurfaceRoute,
    pub changed: bool,
}

/// Versioned navigation state stored inside the application session.
///
/// Invalid historical entries are quarantined during deserialization rather
/// than causing the entire engineering session to be discarded. The current
/// route then recovers to Design and `recovered_invalid_routes()` remains true
/// until the application has surfaced the recovery boundary.
#[derive(Debug, Clone, Serialize)]
pub struct SurfaceNavigation {
    current: SurfaceRoute,
    back: Vec<SurfaceRoute>,
    forward: Vec<SurfaceRoute>,
    recent: Vec<SurfaceRoute>,
    #[serde(skip)]
    pending_browser_effects: VecDeque<BrowserHistoryEffect>,
    #[serde(skip)]
    browser_effect_queue_overflowed: bool,
    #[serde(skip)]
    recovered_invalid_routes: bool,
}

impl Default for SurfaceNavigation {
    fn default() -> Self {
        let current = SurfaceRoute::surface(SurfaceId::Design);
        Self {
            current,
            back: Vec::new(),
            forward: Vec::new(),
            recent: vec![current],
            pending_browser_effects: VecDeque::new(),
            browser_effect_queue_overflowed: false,
            recovered_invalid_routes: false,
        }
    }
}

impl<'de> Deserialize<'de> for SurfaceNavigation {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        #[derive(Deserialize, Default)]
        struct Wire {
            #[serde(default)]
            current: Option<String>,
            #[serde(default)]
            back: Vec<String>,
            #[serde(default)]
            forward: Vec<String>,
            #[serde(default)]
            recent: Vec<String>,
        }

        fn parse_routes(values: Vec<String>, invalid: &mut bool) -> Vec<SurfaceRoute> {
            values
                .into_iter()
                .filter_map(|value| match value.parse() {
                    Ok(route) => Some(route),
                    Err(_) => {
                        *invalid = true;
                        None
                    }
                })
                .collect()
        }

        let wire = Wire::deserialize(deserializer)?;
        let mut invalid = false;
        let current = match wire.current {
            Some(value) => match value.parse() {
                Ok(route) => route,
                Err(_) => {
                    invalid = true;
                    SurfaceRoute::surface(SurfaceId::Design)
                }
            },
            None => {
                invalid = true;
                SurfaceRoute::surface(SurfaceId::Design)
            }
        };
        let mut state = Self {
            current,
            back: parse_routes(wire.back, &mut invalid),
            forward: parse_routes(wire.forward, &mut invalid),
            recent: parse_routes(wire.recent, &mut invalid),
            pending_browser_effects: VecDeque::new(),
            browser_effect_queue_overflowed: false,
            recovered_invalid_routes: invalid,
        };
        state.normalize();
        Ok(state)
    }
}

impl SurfaceNavigation {
    #[must_use]
    pub const fn current(&self) -> SurfaceRoute {
        self.current
    }

    #[must_use]
    pub fn back_entries(&self) -> &[SurfaceRoute] {
        &self.back
    }

    #[must_use]
    pub fn forward_entries(&self) -> &[SurfaceRoute] {
        &self.forward
    }

    #[must_use]
    pub fn recent_entries(&self) -> &[SurfaceRoute] {
        &self.recent
    }

    #[must_use]
    pub const fn can_go_back(&self) -> bool {
        !self.back.is_empty()
    }

    #[must_use]
    pub const fn can_go_forward(&self) -> bool {
        !self.forward.is_empty()
    }

    #[must_use]
    pub const fn recovered_invalid_routes(&self) -> bool {
        self.recovered_invalid_routes
    }

    pub fn acknowledge_recovery(&mut self) {
        self.recovered_invalid_routes = false;
    }

    /// Commit a route selected inside the application.
    pub(crate) fn navigate(
        &mut self,
        route: SurfaceRoute,
        source: RouteTransitionSource,
    ) -> RouteTransition {
        if source == RouteTransitionSource::BrowserPop {
            return self.apply_browser_pop(route);
        }

        let previous = self.current;
        if previous == route {
            return RouteTransition {
                previous,
                current: route,
                changed: false,
            };
        }

        push_bounded(&mut self.back, previous, HISTORY_LIMIT);
        self.forward.clear();
        self.current = route;
        remember_recent(&mut self.recent, route);
        match source {
            RouteTransitionSource::User => {
                self.enqueue_browser_effect(BrowserHistoryEffect::Push(route));
            }
            RouteTransitionSource::Restore => {
                self.enqueue_browser_effect(BrowserHistoryEffect::Replace(route));
            }
            RouteTransitionSource::BrowserPop => self.clear_browser_effects(),
        }
        RouteTransition {
            previous,
            current: route,
            changed: true,
        }
    }

    /// Reconcile one route selected by host browser history without echoing a
    /// second history write. Known entries move between the bounded back and
    /// forward stacks; an unknown host entry starts a fresh in-app history so
    /// stale routes can never create a return loop.
    fn apply_browser_pop(&mut self, route: SurfaceRoute) -> RouteTransition {
        // Any browser-selected entry invalidates effects based on the former
        // host entry. This must also happen for a same-route pop; otherwise a
        // stale push/replace can echo after the browser event.
        self.clear_browser_effects();
        let previous = self.current;
        if previous == route {
            return RouteTransition {
                previous,
                current: route,
                changed: false,
            };
        }

        if let Some(index) = self.back.iter().rposition(|candidate| *candidate == route) {
            let crossed = self.back.split_off(index + 1);
            let destination = self.back[index];
            self.back.truncate(index);
            push_bounded(&mut self.forward, previous, HISTORY_LIMIT);
            for crossed_route in crossed.into_iter().rev() {
                push_bounded(&mut self.forward, crossed_route, HISTORY_LIMIT);
            }
            self.current = destination;
        } else if let Some(index) = self
            .forward
            .iter()
            .rposition(|candidate| *candidate == route)
        {
            let crossed = self.forward.split_off(index + 1);
            let destination = self.forward[index];
            self.forward.truncate(index);
            push_bounded(&mut self.back, previous, HISTORY_LIMIT);
            for crossed_route in crossed.into_iter().rev() {
                push_bounded(&mut self.back, crossed_route, HISTORY_LIMIT);
            }
            self.current = destination;
        } else {
            self.back.clear();
            self.forward.clear();
            self.current = route;
        }

        remember_recent(&mut self.recent, route);
        RouteTransition {
            previous,
            current: route,
            changed: true,
        }
    }

    /// Replace the current entry without creating an in-app back entry.
    pub(crate) fn replace(&mut self, route: SurfaceRoute, source: RouteTransitionSource) {
        self.current = route;
        remember_recent(&mut self.recent, route);
        match source {
            RouteTransitionSource::User | RouteTransitionSource::Restore => {
                self.enqueue_browser_effect(BrowserHistoryEffect::Replace(route));
            }
            RouteTransitionSource::BrowserPop => self.clear_browser_effects(),
        }
    }

    /// Navigate to the prior stable route. An in-app Back command traverses the
    /// host history instead of creating a duplicate entry.
    pub(crate) fn go_back(&mut self, source: RouteTransitionSource) -> Option<RouteTransition> {
        self.go_back_steps(1, source)
    }

    /// Atomically traverse up to `count` known back entries. Runtime adapters
    /// receive one bounded host-history effect for the entire committed
    /// traversal, never one effect per crossed route.
    pub(crate) fn go_back_steps(
        &mut self,
        count: usize,
        source: RouteTransitionSource,
    ) -> Option<RouteTransition> {
        let actual_count = count.min(self.back.len());
        if actual_count == 0 {
            return None;
        }

        let previous = self.current;
        let traversed = self.back.split_off(self.back.len() - actual_count);
        for route in traversed.into_iter().rev() {
            push_bounded(&mut self.forward, self.current, HISTORY_LIMIT);
            self.current = route;
        }
        remember_recent(&mut self.recent, self.current);
        match source {
            RouteTransitionSource::User => {
                self.enqueue_browser_effect(BrowserHistoryEffect::Traverse {
                    delta: -i32::try_from(actual_count).unwrap_or(i32::MAX),
                    destination: self.current,
                });
            }
            RouteTransitionSource::Restore => {
                self.enqueue_browser_effect(BrowserHistoryEffect::Replace(self.current));
            }
            RouteTransitionSource::BrowserPop => self.clear_browser_effects(),
        }
        Some(RouteTransition {
            previous,
            current: self.current,
            changed: true,
        })
    }

    /// Atomically traverse up to `count` known forward entries and emit one
    /// browser traversal effect for the final destination.
    pub(crate) fn go_forward_steps(
        &mut self,
        count: usize,
        source: RouteTransitionSource,
    ) -> Option<RouteTransition> {
        let actual_count = count.min(self.forward.len());
        if actual_count == 0 {
            return None;
        }

        let previous = self.current;
        let traversed = self.forward.split_off(self.forward.len() - actual_count);
        for route in traversed.into_iter().rev() {
            push_bounded(&mut self.back, self.current, HISTORY_LIMIT);
            self.current = route;
        }
        remember_recent(&mut self.recent, self.current);
        match source {
            RouteTransitionSource::User => {
                self.enqueue_browser_effect(BrowserHistoryEffect::Traverse {
                    delta: i32::try_from(actual_count).unwrap_or(i32::MAX),
                    destination: self.current,
                });
            }
            RouteTransitionSource::Restore => {
                self.enqueue_browser_effect(BrowserHistoryEffect::Replace(self.current));
            }
            RouteTransitionSource::BrowserPop => self.clear_browser_effects(),
        }
        Some(RouteTransition {
            previous,
            current: self.current,
            changed: true,
        })
    }

    pub(crate) fn take_browser_effect(&mut self) -> Option<BrowserHistoryEffect> {
        self.pending_browser_effects.pop_front()
    }

    #[cfg(any(test, target_arch = "wasm32"))]
    pub(crate) fn has_pending_browser_effects(&self) -> bool {
        self.browser_effect_queue_overflowed || !self.pending_browser_effects.is_empty()
    }

    #[cfg(any(test, target_arch = "wasm32"))]
    pub(crate) fn take_browser_effect_queue_overflowed(&mut self) -> bool {
        std::mem::take(&mut self.browser_effect_queue_overflowed)
    }

    pub(crate) fn clear_browser_effects(&mut self) {
        self.pending_browser_effects.clear();
        self.browser_effect_queue_overflowed = false;
    }

    fn enqueue_browser_effect(&mut self, effect: BrowserHistoryEffect) {
        #[cfg(any(test, target_arch = "wasm32"))]
        {
            if self.browser_effect_queue_overflowed {
                return;
            }
            if self.pending_browser_effects.len() >= BROWSER_EFFECT_LIMIT {
                self.pending_browser_effects.clear();
                self.browser_effect_queue_overflowed = true;
                return;
            }
            self.pending_browser_effects.push_back(effect);
        }
        #[cfg(not(any(test, target_arch = "wasm32")))]
        let _ = effect;
    }

    /// Begin a fresh host-browser session at the current route. Persisted task
    /// history remains useful on native targets, but it must never authorize a
    /// newly loaded browser tab to traverse entries owned by the embedding
    /// page or a prior process.
    #[cfg(any(test, target_arch = "wasm32"))]
    pub(crate) fn reset_history(&mut self) {
        self.back.clear();
        self.forward.clear();
        self.recent.clear();
        self.recent.push(self.current);
        self.clear_browser_effects();
    }

    /// Remove history entries whose executors are no longer available. The
    /// current route is deliberately left to the owning workbench so it can
    /// issue one explicit recovery diagnostic before replacement.
    pub(crate) fn retain_history(
        &mut self,
        mut predicate: impl FnMut(SurfaceRoute) -> bool,
    ) -> bool {
        let before = self.back.len() + self.forward.len() + self.recent.len();
        self.back.retain(|route| predicate(*route));
        self.forward.retain(|route| predicate(*route));
        self.recent.retain(|route| predicate(*route));
        let changed = before != self.back.len() + self.forward.len() + self.recent.len();
        // Executor availability is a runtime condition, not evidence that a
        // persisted route was malformed. The owning Workbench reports this
        // removal directly; only deserialization may set the malformed-route
        // recovery flag.
        self.normalize();
        changed
    }

    fn normalize(&mut self) {
        retain_newest_bounded(&mut self.back, HISTORY_LIMIT);
        retain_newest_bounded(&mut self.forward, HISTORY_LIMIT);
        deduplicate_bounded(&mut self.recent, RECENT_LIMIT);
        self.recent.retain(|route| *route != self.current);
        self.recent.push(self.current);
        if self.recent.len() > RECENT_LIMIT {
            let excess = self.recent.len() - RECENT_LIMIT;
            self.recent.drain(..excess);
        }
    }
}

fn retain_newest_bounded(entries: &mut Vec<SurfaceRoute>, limit: usize) {
    if entries.len() > limit {
        let excess = entries.len() - limit;
        entries.drain(..excess);
    }
}

fn push_bounded(entries: &mut Vec<SurfaceRoute>, route: SurfaceRoute, limit: usize) {
    entries.push(route);
    if entries.len() > limit {
        let excess = entries.len() - limit;
        entries.drain(..excess);
    }
}

fn remember_recent(entries: &mut Vec<SurfaceRoute>, route: SurfaceRoute) {
    entries.retain(|entry| *entry != route);
    push_bounded(entries, route, RECENT_LIMIT);
}

fn deduplicate_bounded(entries: &mut Vec<SurfaceRoute>, limit: usize) {
    let mut unique = Vec::with_capacity(entries.len().min(limit));
    for route in entries.drain(..).rev() {
        if !unique.contains(&route) {
            unique.push(route);
        }
        if unique.len() == limit {
            break;
        }
    }
    unique.reverse();
    *entries = unique;
}

#[cfg(test)]
mod tests {
    use super::*;

    fn route(id: SurfaceId) -> SurfaceRoute {
        SurfaceRoute::surface(id)
    }

    #[test]
    fn same_route_is_idempotent_and_never_requests_browser_history() {
        let mut navigation = SurfaceNavigation::default();
        let transition = navigation.navigate(route(SurfaceId::Design), RouteTransitionSource::User);
        assert!(!transition.changed);
        assert_eq!(navigation.take_browser_effect(), None);
        assert!(navigation.back_entries().is_empty());
    }

    #[test]
    fn browser_effects_preserve_every_transition_in_commit_order() {
        let mut navigation = SurfaceNavigation::default();
        let results = route(SurfaceId::Results);
        let models = route(SurfaceId::Models);

        navigation.navigate(results, RouteTransitionSource::User);
        navigation.navigate(models, RouteTransitionSource::User);

        assert_eq!(
            navigation.take_browser_effect(),
            Some(BrowserHistoryEffect::Push(results))
        );
        assert_eq!(
            navigation.take_browser_effect(),
            Some(BrowserHistoryEffect::Push(models))
        );
        assert_eq!(navigation.take_browser_effect(), None);
    }

    #[test]
    fn same_route_browser_pop_clears_stale_effects_without_echoing() {
        let mut navigation = SurfaceNavigation::default();
        let results = route(SurfaceId::Results);
        navigation.navigate(results, RouteTransitionSource::User);
        assert!(navigation.has_pending_browser_effects());

        let transition = navigation.navigate(results, RouteTransitionSource::BrowserPop);

        assert!(!transition.changed);
        assert_eq!(navigation.current(), results);
        assert!(!navigation.has_pending_browser_effects());
        assert_eq!(navigation.take_browser_effect(), None);
    }

    #[test]
    fn traversal_effect_retains_its_destination_ahead_of_later_effects() {
        let mut navigation = SurfaceNavigation::default();
        let design = route(SurfaceId::Design);
        let results = route(SurfaceId::Results);
        let models = route(SurfaceId::Models);
        navigation.navigate(results, RouteTransitionSource::User);
        assert_eq!(
            navigation.take_browser_effect(),
            Some(BrowserHistoryEffect::Push(results))
        );

        navigation
            .go_back(RouteTransitionSource::User)
            .expect("Design is in history");
        navigation.navigate(models, RouteTransitionSource::User);

        assert_eq!(
            navigation.take_browser_effect(),
            Some(BrowserHistoryEffect::Traverse {
                delta: -1,
                destination: design,
            })
        );
        assert_eq!(
            navigation.take_browser_effect(),
            Some(BrowserHistoryEffect::Push(models))
        );
    }

    #[test]
    fn browser_effect_queue_overflow_requires_explicit_canonical_recovery() {
        let mut navigation = SurfaceNavigation::default();
        let results = route(SurfaceId::Results);
        let models = route(SurfaceId::Models);

        for index in 0..=BROWSER_EFFECT_LIMIT {
            navigation.navigate(
                if index % 2 == 0 { results } else { models },
                RouteTransitionSource::User,
            );
        }

        assert!(navigation.has_pending_browser_effects());
        assert_eq!(navigation.take_browser_effect(), None);
        assert!(navigation.take_browser_effect_queue_overflowed());
        assert!(!navigation.take_browser_effect_queue_overflowed());
        assert!(!navigation.has_pending_browser_effects());
        assert_eq!(navigation.current(), results);
    }

    #[test]
    fn route_history_supports_exact_back_forward_and_source_return() {
        let mut navigation = SurfaceNavigation::default();
        navigation.navigate(route(SurfaceId::Results), RouteTransitionSource::User);
        navigation.navigate(
            route(SurfaceId::FeatureAvailability),
            RouteTransitionSource::User,
        );

        let back = navigation
            .go_back(RouteTransitionSource::User)
            .expect("source route exists");
        assert_eq!(back.current, route(SurfaceId::Results));
        assert!(navigation.can_go_forward());

        let forward = navigation
            .go_forward_steps(1, RouteTransitionSource::User)
            .expect("manager route exists");
        assert_eq!(forward.current, route(SurfaceId::FeatureAvailability));
    }

    #[test]
    fn multi_step_back_is_atomic_and_emits_one_source_specific_effect() {
        for (source, expected_effect) in [
            (
                RouteTransitionSource::User,
                Some(BrowserHistoryEffect::Traverse {
                    delta: -2,
                    destination: route(SurfaceId::Project),
                }),
            ),
            (
                RouteTransitionSource::Restore,
                Some(BrowserHistoryEffect::Replace(route(SurfaceId::Project))),
            ),
            (RouteTransitionSource::BrowserPop, None),
        ] {
            let mut navigation = SurfaceNavigation::default();
            navigation.navigate(route(SurfaceId::Project), RouteTransitionSource::User);
            navigation.navigate(route(SurfaceId::Results), RouteTransitionSource::User);
            navigation.navigate(
                route(SurfaceId::FeatureAvailability),
                RouteTransitionSource::User,
            );
            navigation.clear_browser_effects();

            let transition = navigation
                .go_back_steps(2, source)
                .expect("two back entries exist");
            assert_eq!(transition.previous, route(SurfaceId::FeatureAvailability));
            assert_eq!(transition.current, route(SurfaceId::Project));
            assert_eq!(navigation.back_entries(), &[route(SurfaceId::Design)]);
            assert_eq!(
                navigation.forward_entries(),
                &[
                    route(SurfaceId::FeatureAvailability),
                    route(SurfaceId::Results)
                ]
            );
            assert_eq!(navigation.take_browser_effect(), expected_effect);
        }
    }

    #[test]
    fn multi_step_back_clamps_to_known_history_and_zero_is_a_no_op() {
        let mut navigation = SurfaceNavigation::default();
        assert_eq!(
            navigation.go_back_steps(0, RouteTransitionSource::User),
            None
        );
        assert_eq!(navigation.take_browser_effect(), None);

        navigation.navigate(route(SurfaceId::Results), RouteTransitionSource::User);
        navigation.clear_browser_effects();
        let transition = navigation
            .go_back_steps(usize::MAX, RouteTransitionSource::User)
            .expect("one back entry exists");
        assert_eq!(transition.current, route(SurfaceId::Design));
        assert_eq!(
            navigation.take_browser_effect(),
            Some(BrowserHistoryEffect::Traverse {
                delta: -1,
                destination: route(SurfaceId::Design),
            })
        );
    }

    #[test]
    fn multi_step_forward_is_atomic_and_emits_one_traversal_effect() {
        let design = route(SurfaceId::Design);
        let results = route(SurfaceId::Results);
        let project = route(SurfaceId::Project);
        let mut navigation = SurfaceNavigation::default();
        navigation.navigate(results, RouteTransitionSource::User);
        navigation.navigate(project, RouteTransitionSource::User);
        navigation.go_back_steps(2, RouteTransitionSource::BrowserPop);
        assert_eq!(navigation.current(), design);
        assert_eq!(navigation.forward_entries(), &[project, results]);

        let transition = navigation
            .go_forward_steps(2, RouteTransitionSource::User)
            .expect("two forward entries exist");

        assert_eq!(transition.previous, design);
        assert_eq!(transition.current, project);
        assert_eq!(navigation.back_entries(), &[design, results]);
        assert!(navigation.forward_entries().is_empty());
        assert_eq!(
            navigation.take_browser_effect(),
            Some(BrowserHistoryEffect::Traverse {
                delta: 2,
                destination: project,
            })
        );
    }

    #[test]
    fn browser_pop_does_not_echo_a_browser_history_effect() {
        let mut navigation = SurfaceNavigation::default();
        navigation.navigate(route(SurfaceId::Results), RouteTransitionSource::BrowserPop);
        assert_eq!(navigation.take_browser_effect(), None);
        assert!(!navigation.can_go_back());
        assert!(!navigation.can_go_forward());
    }

    #[test]
    fn browser_pop_reconciles_known_back_forward_and_multistep_routes() {
        let mut navigation = SurfaceNavigation::default();
        navigation.navigate(route(SurfaceId::Project), RouteTransitionSource::User);
        navigation.navigate(route(SurfaceId::Results), RouteTransitionSource::User);
        navigation.navigate(
            route(SurfaceId::FeatureAvailability),
            RouteTransitionSource::User,
        );

        let back =
            navigation.navigate(route(SurfaceId::Project), RouteTransitionSource::BrowserPop);
        assert_eq!(back.current, route(SurfaceId::Project));
        assert_eq!(navigation.back_entries(), &[route(SurfaceId::Design)]);
        assert_eq!(
            navigation.forward_entries(),
            &[
                route(SurfaceId::FeatureAvailability),
                route(SurfaceId::Results)
            ]
        );
        assert_eq!(navigation.take_browser_effect(), None);

        let forward = navigation.navigate(
            route(SurfaceId::FeatureAvailability),
            RouteTransitionSource::BrowserPop,
        );
        assert_eq!(forward.current, route(SurfaceId::FeatureAvailability));
        assert_eq!(
            navigation.back_entries(),
            &[
                route(SurfaceId::Design),
                route(SurfaceId::Project),
                route(SurfaceId::Results)
            ]
        );
        assert!(navigation.forward_entries().is_empty());
        assert_eq!(navigation.take_browser_effect(), None);
    }

    #[test]
    fn serialized_navigation_retains_stable_object_aware_history() {
        let mut navigation = SurfaceNavigation::default();
        navigation.navigate(route(SurfaceId::Results), RouteTransitionSource::User);
        navigation.navigate(
            route(SurfaceId::FeatureAvailability),
            RouteTransitionSource::User,
        );
        let json = serde_json::to_string(&navigation).expect("navigation serializes");
        let restored: SurfaceNavigation = serde_json::from_str(&json).expect("navigation restores");
        assert_eq!(restored.current(), navigation.current());
        assert_eq!(restored.back_entries(), navigation.back_entries());
        assert_eq!(restored.recent_entries(), navigation.recent_entries());
        assert_eq!(restored.take_browser_effect_for_test(), None);
    }

    #[test]
    fn repeated_order_sensitive_history_survives_normalization_and_serde() {
        let route_a = route(SurfaceId::Design);
        let route_b = route(SurfaceId::Results);
        let mut navigation = SurfaceNavigation::default();
        navigation.navigate(route_b, RouteTransitionSource::User);
        navigation.navigate(route_a, RouteTransitionSource::User);

        assert_eq!(navigation.current(), route_a);
        assert_eq!(navigation.back_entries(), &[route_a, route_b]);

        let json = serde_json::to_string(&navigation).expect("navigation serializes");
        let restored: SurfaceNavigation = serde_json::from_str(&json).expect("navigation restores");
        assert_eq!(restored.current(), route_a);
        assert_eq!(restored.back_entries(), &[route_a, route_b]);

        let duplicate_stacks = r#"{
            "current":"?view=design",
            "back":["?view=design","?view=results","?view=design"],
            "forward":["?view=simulate","?view=results","?view=simulate"],
            "recent":[]
        }"#;
        let restored: SurfaceNavigation =
            serde_json::from_str(duplicate_stacks).expect("duplicate stacks restore");
        assert_eq!(restored.back_entries(), &[route_a, route_b, route_a]);
        assert_eq!(
            restored.forward_entries(),
            &[
                route(SurfaceId::Simulate),
                route_b,
                route(SurfaceId::Simulate)
            ]
        );
    }

    #[test]
    fn fresh_browser_session_discards_persisted_traversal_authority() {
        let mut navigation = SurfaceNavigation::default();
        navigation.navigate(route(SurfaceId::Results), RouteTransitionSource::User);
        navigation.navigate(
            route(SurfaceId::FeatureAvailability),
            RouteTransitionSource::User,
        );

        navigation.reset_history();

        assert_eq!(navigation.current(), route(SurfaceId::FeatureAvailability));
        assert!(navigation.back_entries().is_empty());
        assert!(navigation.forward_entries().is_empty());
        assert_eq!(
            navigation.recent_entries(),
            [route(SurfaceId::FeatureAvailability)]
        );
        assert_eq!(navigation.take_browser_effect(), None);
    }

    #[test]
    fn invalid_persisted_route_is_quarantined_without_losing_navigation_state() {
        let json = r#"{
            "current":"?surface=not-a-surface",
            "back":["?view=results","?surface=bad"],
            "forward":[],
            "recent":["?view=simulate"]
        }"#;
        let restored: SurfaceNavigation =
            serde_json::from_str(json).expect("invalid route is recoverable");
        assert_eq!(restored.current(), route(SurfaceId::Design));
        assert_eq!(restored.back_entries(), &[route(SurfaceId::Results)]);
        assert!(restored.recovered_invalid_routes());
    }

    impl SurfaceNavigation {
        fn take_browser_effect_for_test(&self) -> Option<BrowserHistoryEffect> {
            self.pending_browser_effects.front().copied()
        }
    }
}
