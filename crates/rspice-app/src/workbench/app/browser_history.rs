//! Browser Back and Forward, kept in step with the in-app route.
//!
//! The browser's history and the workbench's navigation stack are two ledgers
//! of one session. These functions adopt the address the page opened at,
//! apply authenticated popstate traversals, flush queued history effects, and
//! restart a bounded history session whenever the two disagree.

use egui::Context;

use crate::workbench::app_state::AppState;

/// Record a browser-history synchronization problem in the Console.
///
/// Each of these leaves the app working at the task on screen, so none is an
/// error. What the reader must know is that the browser's Back and Forward
/// may not do what they expect until synchronization recovers.
fn record_browser_history_warning(state: &mut AppState, text: String) {
    state.record(
        crate::diagnostics::LogSeverity::Warning,
        crate::diagnostics::LogSource::Studio,
        text,
    );
}

pub(super) fn initialize_browser_surface_navigation(state: &mut AppState, ctx: &Context) {
    use crate::workbench::RouteTransitionSource;
    use crate::workbench::browser::navigation::{
        current_location, install_popstate_listener, restart_history_session,
    };

    match current_location() {
        Ok(location) => {
            if let Some(route) = location.route()
                && let Err(error) = state
                    .workbench
                    .navigate(route, RouteTransitionSource::BrowserPop)
            {
                record_browser_history_warning(
                    state,
                    format!("The page in the link was not opened: {error}"),
                );
            }
        }
        Err(error) => record_browser_history_warning(
            state,
            format!(
                "The browser address is not a valid RSpice link; the current task was opened \
                 instead: {error}"
            ),
        ),
    }

    // A restored native-style stack is not browser traversal authority. The
    // new process starts with exactly its active route and takes ownership of
    // the current host entry through one canonical replaceState transaction.
    state
        .workbench
        .reset_navigation_history_for_fresh_browser_session();
    let listener_ready = match install_popstate_listener(ctx) {
        Ok(()) => true,
        Err(error) => {
            record_browser_history_warning(
                state,
                format!(
                    "Browser Back and Forward are off and will be set up again automatically: \
                     {error}"
                ),
            );
            false
        }
    };
    if listener_ready && let Err(error) = restart_history_session(state.workbench.current_route()) {
        record_browser_history_warning(
            state,
            format!(
                "Browser Back and Forward are off and will be set up again automatically; \
                 browser history could not be started at the current task: {error}"
            ),
        );
    }
}

pub(super) fn synchronize_browser_surface_navigation(state: &mut AppState) {
    use crate::workbench::browser::navigation::{
        ensure_popstate_listener, history_session_ready, poll_popstate, push_route, replace_route,
        traversal_in_flight, traversal_watchdog_expired, traverse_history,
    };
    use crate::workbench::{BrowserHistoryEffect, RouteTransitionSource};

    while let Some(event) = poll_popstate() {
        match event {
            Ok(event) => {
                let route = event.route();
                let availability =
                    crate::workbench::routing::availability::route_availability(route);
                let pending_effects = state.workbench.has_pending_browser_history_effects();
                let external_race = !event.initiated_by_app() && pending_effects;
                let applied = if !availability.can_open() {
                    Err(format!(
                        "{} is unavailable: {}",
                        route.surface_id().label(),
                        availability
                            .reason()
                            .unwrap_or("it cannot be opened in this build")
                    ))
                } else if external_race {
                    Err(
                        "the browser moved through history while the app was still navigating"
                            .to_owned(),
                    )
                } else if event.initiated_by_app() {
                    (state.workbench.current_route() == route || pending_effects)
                        .then_some(())
                        .ok_or_else(|| {
                            "the browser did not arrive at the page the app asked for".to_owned()
                        })
                } else {
                    apply_authenticated_browser_pop(state, event.delta(), route)
                };

                if let Err(error) = applied {
                    state.prompt(format!(
                        "Browser history could not open the requested page and returned to the \
                         current task: {error}"
                    ));
                    if availability.can_open() && !external_race {
                        // The browser entry is authenticated and available;
                        // make it authoritative, then start a bounded session
                        // rather than guessing from stale in-app stacks.
                        if state.workbench.current_route() != route {
                            let _ = state
                                .workbench
                                .navigate(route, RouteTransitionSource::BrowserPop);
                        }
                    }
                    recover_browser_history_at_active_task(state);
                    return;
                }
            }
            Err(error) => {
                state.prompt(format!(
                    "A browser history entry could not be read and was replaced with the current \
                     task: {error}"
                ));
                recover_browser_history_at_active_task(state);
                return;
            }
        }
    }

    if state
        .workbench
        .take_browser_history_effect_queue_overflowed()
    {
        record_browser_history_warning(
            state,
            "Browser history fell behind the app and was reset to the current task.".to_owned(),
        );
        recover_browser_history_at_active_task(state);
        return;
    }

    if traversal_watchdog_expired() {
        record_browser_history_warning(
            state,
            "The browser did not finish moving through history in time; the address was reset \
             to the current task."
                .to_owned(),
        );
        recover_browser_history_at_active_task(state);
        return;
    }

    if !history_session_ready() {
        state.workbench.clear_browser_history_effects();
        if ensure_popstate_listener().is_err() {
            return;
        }
        let canonical = state.workbench.current_route();
        match crate::workbench::browser::navigation::restart_history_session(canonical) {
            Ok(()) => {
                state
                    .workbench
                    .reset_navigation_history_for_fresh_browser_session();
                state.record(
                    crate::diagnostics::LogSeverity::Info,
                    crate::diagnostics::LogSource::Studio,
                    "Browser Back and Forward resumed at the current task; earlier browser \
                     history was cleared.",
                );
            }
            Err(_) => return,
        }
    }

    // `history.go` is asynchronous. No later effect may observe or mutate the
    // browser ledger until its exact destination is authenticated by popstate.
    if traversal_in_flight() {
        return;
    }

    while let Some(effect) = state.workbench.take_browser_history_effect() {
        let (result, traversal_started) = match effect {
            BrowserHistoryEffect::Push(route) => (push_route(route).map(|_| ()), false),
            BrowserHistoryEffect::Replace(route) => (replace_route(route).map(|_| ()), false),
            BrowserHistoryEffect::Traverse { delta, destination } => {
                (traverse_history(delta, destination), true)
            }
        };
        if let Err(error) = result {
            rollback_browser_navigation_after_sync_failure(state, error);
            return;
        }
        if traversal_started {
            return;
        }
    }
}

fn recover_browser_history_at_active_task(state: &mut AppState) {
    use crate::workbench::browser::navigation::{
        ensure_popstate_listener, restart_history_session,
    };

    state.workbench.clear_browser_history_effects();
    let canonical = state.workbench.current_route();
    if let Err(error) = ensure_popstate_listener() {
        record_browser_history_warning(
            state,
            format!(
                "Browser Back and Forward are off and will be set up again automatically: \
                 {error}"
            ),
        );
        return;
    }
    match restart_history_session(canonical) {
        Ok(()) => state
            .workbench
            .reset_navigation_history_for_fresh_browser_session(),
        Err(error) => record_browser_history_warning(
            state,
            format!(
                "Browser Back and Forward are off and will be set up again automatically; the \
                 address could not be reset to the current task: {error}"
            ),
        ),
    }
}

fn rollback_browser_navigation_after_sync_failure(
    state: &mut AppState,
    error: crate::workbench::browser::navigation::BrowserNavigationError,
) {
    use crate::workbench::RouteTransitionSource;
    use crate::workbench::browser::navigation::{active_browser_route, restart_history_session};

    match active_browser_route() {
        Ok(browser_route) => {
            record_browser_history_warning(
                state,
                format!(
                    "The browser address could not be updated; the app returned to the last page \
                     the browser recorded: {error}"
                ),
            );
            if state.workbench.current_route() != browser_route {
                let _ = state
                    .workbench
                    .navigate(browser_route, RouteTransitionSource::BrowserPop);
            }
            state.workbench.clear_browser_history_effects();
            state
                .workbench
                .reset_navigation_history_for_fresh_browser_session();
            if let Err(recovery_error) = restart_history_session(browser_route) {
                record_browser_history_warning(
                    state,
                    format!(
                        "Browser Back and Forward are off and will be set up again \
                         automatically; browser history could not be restarted: {recovery_error}"
                    ),
                );
            }
        }
        Err(_) => {
            record_browser_history_warning(
                state,
                format!(
                    "The browser address could not be updated, and there was no earlier page to \
                     return to: {error}"
                ),
            );
            recover_browser_history_at_active_task(state);
        }
    }
}

fn apply_authenticated_browser_pop(
    state: &mut AppState,
    delta: i32,
    route: crate::workbench::SurfaceRoute,
) -> Result<(), String> {
    use crate::workbench::RouteTransitionSource;

    if delta < 0 {
        let outside = || "the page is older than this session's history".to_owned();
        let steps = usize::try_from(delta.unsigned_abs()).map_err(|_| outside())?;
        if steps > state.workbench.back_route_count() {
            return Err(outside());
        }
        state
            .workbench
            .navigate_back_steps(steps, RouteTransitionSource::BrowserPop)
            .ok_or_else(outside)?;
    } else if delta > 0 {
        let outside = || "the page is newer than this session's history".to_owned();
        let steps = usize::try_from(delta).map_err(|_| outside())?;
        if steps > state.workbench.forward_route_count() {
            return Err(outside());
        }
        state
            .workbench
            .navigate_forward_steps(steps, RouteTransitionSource::BrowserPop)
            .ok_or_else(outside)?;
    }

    if state.workbench.current_route() == route {
        Ok(())
    } else {
        Err("the browser and the app disagree about which page is current".to_owned())
    }
}
