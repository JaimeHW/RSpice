//! Chord resolution.
//!
//! Matches a frame of input against the active profile, holding partial
//! prefixes across frames until the sequence completes or times out. A
//! command is only fired when it is also available in the current context.

use std::time::Duration;

use crate::state::ViewType;
use crate::state::command_catalog::{CommandCatalog, CommandKey, CommandPlatform, ShortcutContext};
use crate::state::shortcuts::{
    ChordTimeoutPolicy, ContextPrecedencePolicy, ShortcutPreferences, SingleKeyCanvasPolicy,
    shortcut_context_precedence_rank,
};
use crate::workbench::state::Workspace;

use super::ShortcutInputSnapshot;
use super::input_snapshot::ShortcutKeyPress;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct ShortcutEnvironment {
    pub workspace: Workspace,
    pub active_view: ViewType,
    pub canvas_focus: bool,
    /// The source editor the page shows as its document holds keyboard
    /// focus. Its line and fold keys act only then.
    pub source_editor_focus: bool,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ShortcutResolution {
    pub command: Option<CommandKey>,
    pub consume: Vec<(egui::Key, egui::Modifiers)>,
    /// Live presses before this index complete the command; later presses
    /// belong to its continuation and may need a different input owner.
    pub deferred_start: usize,
    pub repaint_after: Option<Duration>,
}

/// Transient sequence state. It is intentionally absent from session
/// persistence: a partially entered chord has no authority after restart.
#[derive(Debug, Clone, Default)]
pub struct ShortcutResolverState {
    pending: Vec<ShortcutKeyPress>,
    deferred: Vec<ShortcutKeyPress>,
    pending_since: Option<Duration>,
    last_now: Option<Duration>,
    observed_profile: Option<ShortcutPreferences>,
    observed_platform: Option<CommandPlatform>,
}

impl ShortcutResolverState {
    pub fn reset(&mut self) {
        self.clear_pending();
        self.deferred.clear();
    }

    fn clear_pending(&mut self) {
        self.pending.clear();
        self.pending_since = None;
    }

    /// Resolve before touching egui's input queue. `is_available` is evaluated
    /// before precedence so a disabled higher-tier command cannot steal a
    /// chord from an executable lower-tier command. `catalog` is the shell's
    /// list of commands the profile binds.
    pub fn resolve(
        &mut self,
        snapshot: &ShortcutInputSnapshot,
        profile: &ShortcutPreferences,
        catalog: &impl CommandCatalog,
        platform: CommandPlatform,
        environment: ShortcutEnvironment,
        injected_now: Duration,
        mut is_available: impl FnMut(CommandKey) -> bool,
    ) -> ShortcutResolution {
        let now = self.monotonic_now(injected_now);
        if self.observed_profile.as_ref() != Some(profile)
            || self.observed_platform != Some(platform)
        {
            self.reset();
            self.observed_profile = Some(profile.clone());
            self.observed_platform = Some(platform);
        }

        // A malformed known command invalidates the entire persisted profile.
        // Keep policy projection on the same fail-closed boundary as bindings:
        // mixing default bindings with untrusted custom policies would produce
        // a third, never-authored runtime profile.
        let (timeout_policy, precedence, single_key_policy) =
            if profile.is_effective_profile_valid(catalog) {
                (
                    profile.policies().chord_timeout(),
                    profile.policies().context_precedence(),
                    profile.policies().single_key_canvas(),
                )
            } else {
                (
                    ChordTimeoutPolicy::default(),
                    ContextPrecedencePolicy::default(),
                    SingleKeyCanvasPolicy::default(),
                )
            };
        let timeout = timeout_duration(timeout_policy);
        if !self.pending.is_empty() {
            let pending_analysis = analyze_prefix(
                &self.pending,
                profile,
                catalog,
                platform,
                environment,
                snapshot.has_non_canvas_focus(),
                precedence,
                single_key_policy,
                &mut is_available,
            );
            if pending_analysis.is_empty() {
                self.clear_pending();
            } else if timeout.is_some_and(|timeout| {
                self.pending_since
                    .is_some_and(|started| now.saturating_sub(started) >= timeout)
            }) {
                let command = pending_analysis.best_exact();
                self.clear_pending();
                if command.is_some() {
                    let consume = snapshot
                        .key_presses()
                        .iter()
                        .map(|press| (press.key(), press.modifiers()))
                        .collect();
                    self.deferred.extend(snapshot.key_presses().iter().copied());
                    return ShortcutResolution {
                        command,
                        consume,
                        deferred_start: 0,
                        repaint_after: None,
                    };
                }
            } else if !pending_analysis.has_longer {
                self.clear_pending();
                let consume = snapshot
                    .key_presses()
                    .iter()
                    .map(|press| (press.key(), press.modifiers()))
                    .collect();
                self.deferred.extend(snapshot.key_presses().iter().copied());
                return ShortcutResolution {
                    command: pending_analysis.best_exact(),
                    consume,
                    deferred_start: 0,
                    repaint_after: None,
                };
            }
        }

        let mut consume = Vec::new();
        let queued = self
            .deferred
            .drain(..)
            .map(|press| (press, false))
            .chain(
                snapshot
                    .key_presses()
                    .iter()
                    .copied()
                    .map(|press| (press, true)),
            )
            .collect::<Vec<_>>();
        for (event_index, (press, live)) in queued.iter().copied().enumerate() {
            if self.pending.is_empty() {
                let prefix = [press];
                let analysis = analyze_prefix(
                    &prefix,
                    profile,
                    catalog,
                    platform,
                    environment,
                    snapshot.has_non_canvas_focus(),
                    precedence,
                    single_key_policy,
                    &mut is_available,
                );
                if analysis.is_empty() {
                    continue;
                }
                if live {
                    consume.push((press.key(), press.modifiers()));
                }
                if analysis.has_longer {
                    self.pending.push(press);
                    self.pending_since = Some(now);
                    continue;
                }
                let deferred_start = consume.len();
                self.defer_remaining(&queued, event_index + 1, &mut consume);
                return ShortcutResolution {
                    command: analysis.best_exact(),
                    consume,
                    deferred_start,
                    repaint_after: None,
                };
            }

            let previous = analyze_prefix(
                &self.pending,
                profile,
                catalog,
                platform,
                environment,
                snapshot.has_non_canvas_focus(),
                precedence,
                single_key_policy,
                &mut is_available,
            );
            let mut extended = self.pending.clone();
            extended.push(press);
            let analysis = analyze_prefix(
                &extended,
                profile,
                catalog,
                platform,
                environment,
                snapshot.has_non_canvas_focus(),
                precedence,
                single_key_policy,
                &mut is_available,
            );
            if analysis.is_empty() {
                self.clear_pending();
                if let Some(command) = previous.best_exact() {
                    // Preserve the mismatch and every later event for the next
                    // resolver pass. Live egui events are consumed now so no
                    // legacy handler can execute a second product command in
                    // this frame.
                    let deferred_start = consume.len();
                    self.defer_remaining(&queued, event_index, &mut consume);
                    return ShortcutResolution {
                        command: Some(command),
                        consume,
                        deferred_start,
                        repaint_after: None,
                    };
                }

                // A prefix-only chord was cancelled. Re-evaluate the mismatch
                // as a fresh first stroke so a real command is not lost.
                let fresh = analyze_prefix(
                    &[press],
                    profile,
                    catalog,
                    platform,
                    environment,
                    snapshot.has_non_canvas_focus(),
                    precedence,
                    single_key_policy,
                    &mut is_available,
                );
                if fresh.is_empty() {
                    continue;
                }
                if live {
                    consume.push((press.key(), press.modifiers()));
                }
                if fresh.has_longer {
                    self.pending.push(press);
                    self.pending_since = Some(now);
                    continue;
                }
                let deferred_start = consume.len();
                self.defer_remaining(&queued, event_index + 1, &mut consume);
                return ShortcutResolution {
                    command: fresh.best_exact(),
                    consume,
                    deferred_start,
                    repaint_after: None,
                };
            }

            if live {
                consume.push((press.key(), press.modifiers()));
            }
            if analysis.has_longer {
                self.pending = extended;
                self.pending_since = Some(now);
                continue;
            }
            self.clear_pending();
            let deferred_start = consume.len();
            self.defer_remaining(&queued, event_index + 1, &mut consume);
            return ShortcutResolution {
                command: analysis.best_exact(),
                consume,
                deferred_start,
                repaint_after: None,
            };
        }

        let repaint_after = self.pending_since.and_then(|started| {
            timeout.map(|timeout| timeout.saturating_sub(now.saturating_sub(started)))
        });
        ShortcutResolution {
            command: None,
            deferred_start: consume.len(),
            consume,
            repaint_after,
        }
    }

    fn monotonic_now(&mut self, injected_now: Duration) -> Duration {
        let now = self
            .last_now
            .map_or(injected_now, |previous| previous.max(injected_now));
        self.last_now = Some(now);
        now
    }

    fn defer_remaining(
        &mut self,
        queued: &[(ShortcutKeyPress, bool)],
        start: usize,
        consume: &mut Vec<(egui::Key, egui::Modifiers)>,
    ) {
        for (press, live) in queued.iter().copied().skip(start) {
            self.deferred.push(press);
            if live {
                consume.push((press.key(), press.modifiers()));
            }
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
struct ExactCandidate {
    command: CommandKey,
    tier: u8,
    registry_index: usize,
    binding_index: usize,
}

#[derive(Debug, Default)]
struct PrefixAnalysis {
    exact: Vec<ExactCandidate>,
    has_longer: bool,
}

impl PrefixAnalysis {
    fn is_empty(&self) -> bool {
        self.exact.is_empty() && !self.has_longer
    }

    fn best_exact(&self) -> Option<CommandKey> {
        self.exact
            .iter()
            .min_by_key(|candidate| {
                (
                    candidate.tier,
                    candidate.registry_index,
                    candidate.binding_index,
                )
            })
            .map(|candidate| candidate.command)
    }
}

fn analyze_prefix(
    prefix: &[ShortcutKeyPress],
    profile: &ShortcutPreferences,
    catalog: &impl CommandCatalog,
    platform: CommandPlatform,
    environment: ShortcutEnvironment,
    non_canvas_focus: bool,
    precedence: ContextPrecedencePolicy,
    single_key_policy: SingleKeyCanvasPolicy,
    is_available: &mut impl FnMut(CommandKey) -> bool,
) -> PrefixAnalysis {
    let mut analysis = PrefixAnalysis::default();
    for (registry_index, command) in catalog.commands().iter().copied().enumerate() {
        let Some(context) = catalog.context(command) else {
            continue;
        };
        if !context_is_active(context, environment)
            || (non_canvas_focus && context.suppressed_by_text_focus())
            || !is_available(command)
        {
            continue;
        }
        for (binding_index, binding) in profile
            .effective_bindings(catalog, command)
            .into_iter()
            .enumerate()
        {
            if !binding.supports(platform)
                || !single_key_canvas_binding_allowed(
                    context,
                    binding.sequence(),
                    environment,
                    single_key_policy,
                )
            {
                continue;
            }
            let strokes = binding.sequence().strokes();
            if prefix.len() > strokes.len()
                || !prefix
                    .iter()
                    .copied()
                    .zip(strokes)
                    .all(|(actual, expected)| {
                        actual.matches(
                            expected.key(),
                            expected.primary(),
                            expected.alt(),
                            expected.shift(),
                        )
                    })
            {
                continue;
            }
            if prefix.len() == strokes.len() {
                analysis.exact.push(ExactCandidate {
                    command,
                    tier: shortcut_context_precedence_rank(context, precedence),
                    registry_index,
                    binding_index,
                });
            } else {
                analysis.has_longer = true;
            }
        }
    }
    analysis
}

/// Whether a bare single-key binding may fire on the surface in front of the
/// reader.
///
/// The requirement is about the *drawing canvases*: a bare letter must not
/// place a wire or arm a tool while the reader is working in a side panel,
/// so those bindings wait for the canvas to hold focus. `+`, `−` and `F` are
/// bare keys of exactly that kind, and they are also how a result sheet is
/// magnified and fitted.
///
/// Results has no drawing canvas. `engineering_canvas_has_focus` answers
/// `false` for that workspace by construction, so requiring canvas focus
/// there requires a condition that can never hold — which is why the
/// magnification chords the View menu prints did nothing on a result sheet.
/// Typing is still covered: this context is suppressed by text focus, so a
/// bare `F` typed into a filter field never reaches here.
fn single_key_canvas_binding_allowed(
    context: ShortcutContext,
    sequence: &crate::state::shortcuts::ShortcutSequence,
    environment: ShortcutEnvironment,
    policy: SingleKeyCanvasPolicy,
) -> bool {
    if !matches!(
        context,
        ShortcutContext::EditContext
            | ShortcutContext::EngineeringCanvas
            | ShortcutContext::SymbolCanvas
    ) || environment.workspace == Workspace::Results
        || sequence.strokes().len() != 1
        || sequence.strokes()[0].primary()
        || sequence.strokes()[0].alt()
        || sequence.strokes()[0].shift()
    {
        return true;
    }
    match policy {
        SingleKeyCanvasPolicy::CanvasFocusOnly => environment.canvas_focus,
        // RequireAlt is transformed in ShortcutPreferences::effective_bindings
        // so execution and every display projection share the exact sequence.
        SingleKeyCanvasPolicy::RequireAlt => false,
        // Disabled bindings are filtered in the same shared projection.
        SingleKeyCanvasPolicy::Disabled => false,
    }
}

fn context_is_active(context: ShortcutContext, environment: ShortcutEnvironment) -> bool {
    match context {
        ShortcutContext::Global | ShortcutContext::ApplicationChrome => true,
        // Must agree with `ShortcutContext::matches`, which the menus and the
        // palette ask the same question of. Results carries the magnification
        // and copy commands; the schematic-editing family that shares this
        // context is filtered out there by `is_available`.
        ShortcutContext::EditContext => {
            environment.workspace == Workspace::Results
                || match environment.active_view {
                    ViewType::Schematic | ViewType::Testbench => {
                        environment.workspace == Workspace::Design
                    }
                    ViewType::Symbol => {
                        matches!(environment.workspace, Workspace::Design | Workspace::Models)
                    }
                    _ => false,
                }
        }
        ShortcutContext::EngineeringCanvas => {
            environment.workspace == Workspace::Design
                && matches!(
                    environment.active_view,
                    ViewType::Schematic | ViewType::Testbench
                )
        }
        ShortcutContext::SymbolCanvas => {
            matches!(environment.workspace, Workspace::Design | Workspace::Models)
                && environment.active_view == ViewType::Symbol
        }
        ShortcutContext::DesignWorkspace => environment.workspace == Workspace::Design,
        ShortcutContext::SimulationWorkspace => environment.workspace == Workspace::Simulate,
        ShortcutContext::ResultsWorkspace => environment.workspace == Workspace::Results,
        ShortcutContext::VerificationWorkspace => environment.workspace == Workspace::Verify,
        ShortcutContext::ViolationNavigation => {
            matches!(environment.workspace, Workspace::Design | Workspace::Verify)
        }
        ShortcutContext::RunnableProject => true,
        ShortcutContext::SourceEditor => environment.source_editor_focus,
    }
}

fn timeout_duration(policy: ChordTimeoutPolicy) -> Option<Duration> {
    policy.seconds().map(Duration::from_secs_f64)
}
