use std::time::Duration;

use crate::state::ViewType;
use crate::workbench::commands::{COMMAND_REGISTRY, Command, CommandPlatform, ShortcutContext};
use crate::workbench::state::Workspace;
use crate::workbench::{
    ChordTimeoutPolicy, ContextPrecedencePolicy, ShortcutPreferences, SingleKeyCanvasPolicy,
    shortcut_context_precedence_rank,
};

use super::ShortcutInputSnapshot;
use super::input_snapshot::ShortcutKeyPress;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(crate) struct ShortcutEnvironment {
    pub(crate) workspace: Workspace,
    pub(crate) active_view: ViewType,
    pub(crate) canvas_focus: bool,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub(crate) struct ShortcutResolution {
    pub(crate) command: Option<Command>,
    pub(crate) consume: Vec<(egui::Key, egui::Modifiers)>,
    pub(crate) repaint_after: Option<Duration>,
}

/// Transient sequence state. It is intentionally absent from session
/// persistence: a partially entered chord has no authority after restart.
#[derive(Debug, Clone, Default)]
pub(crate) struct ShortcutResolverState {
    pending: Vec<ShortcutKeyPress>,
    deferred: Vec<ShortcutKeyPress>,
    pending_since: Option<Duration>,
    last_now: Option<Duration>,
    observed_profile: Option<ShortcutPreferences>,
    observed_platform: Option<CommandPlatform>,
}

impl ShortcutResolverState {
    pub(crate) fn reset(&mut self) {
        self.clear_pending();
        self.deferred.clear();
    }

    fn clear_pending(&mut self) {
        self.pending.clear();
        self.pending_since = None;
    }

    /// Resolve before touching egui's input queue. `is_available` is evaluated
    /// before precedence so a disabled higher-tier command cannot steal a
    /// chord from an executable lower-tier command.
    pub(crate) fn resolve(
        &mut self,
        snapshot: &ShortcutInputSnapshot,
        profile: &ShortcutPreferences,
        platform: CommandPlatform,
        environment: ShortcutEnvironment,
        injected_now: Duration,
        mut is_available: impl FnMut(Command) -> bool,
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
            if profile.is_effective_profile_valid() {
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
                self.defer_remaining(&queued, event_index + 1, &mut consume);
                return ShortcutResolution {
                    command: analysis.best_exact(),
                    consume,
                    repaint_after: None,
                };
            }

            let previous = analyze_prefix(
                &self.pending,
                profile,
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
                    self.defer_remaining(&queued, event_index, &mut consume);
                    return ShortcutResolution {
                        command: Some(command),
                        consume,
                        repaint_after: None,
                    };
                }

                // A prefix-only chord was cancelled. Re-evaluate the mismatch
                // as a fresh first stroke so a real command is not lost.
                let fresh = analyze_prefix(
                    &[press],
                    profile,
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
                self.defer_remaining(&queued, event_index + 1, &mut consume);
                return ShortcutResolution {
                    command: fresh.best_exact(),
                    consume,
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
            self.defer_remaining(&queued, event_index + 1, &mut consume);
            return ShortcutResolution {
                command: analysis.best_exact(),
                consume,
                repaint_after: None,
            };
        }

        let repaint_after = self.pending_since.and_then(|started| {
            timeout.map(|timeout| timeout.saturating_sub(now.saturating_sub(started)))
        });
        ShortcutResolution {
            command: None,
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
    command: Command,
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

    fn best_exact(&self) -> Option<Command> {
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

#[allow(clippy::too_many_arguments)]
fn analyze_prefix(
    prefix: &[ShortcutKeyPress],
    profile: &ShortcutPreferences,
    platform: CommandPlatform,
    environment: ShortcutEnvironment,
    non_canvas_focus: bool,
    precedence: ContextPrecedencePolicy,
    single_key_policy: SingleKeyCanvasPolicy,
    is_available: &mut impl FnMut(Command) -> bool,
) -> PrefixAnalysis {
    let mut analysis = PrefixAnalysis::default();
    for (registry_index, command) in COMMAND_REGISTRY.iter().copied().enumerate() {
        let context = command.shortcut_context();
        if !context_is_active(context, environment)
            || (non_canvas_focus && context.suppressed_by_text_focus())
            || !is_available(command)
        {
            continue;
        }
        for (binding_index, binding) in profile.effective_bindings(command).into_iter().enumerate()
        {
            if !binding.supports(platform)
                || !single_key_canvas_binding_allowed(
                    context,
                    binding.sequence(),
                    environment.canvas_focus,
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

fn single_key_canvas_binding_allowed(
    context: ShortcutContext,
    sequence: &crate::workbench::ShortcutSequence,
    canvas_focus: bool,
    policy: SingleKeyCanvasPolicy,
) -> bool {
    if !matches!(
        context,
        ShortcutContext::EditContext
            | ShortcutContext::EngineeringCanvas
            | ShortcutContext::SymbolCanvas
    ) || sequence.strokes().len() != 1
        || sequence.strokes()[0].primary()
        || sequence.strokes()[0].alt()
        || sequence.strokes()[0].shift()
    {
        return true;
    }
    match policy {
        SingleKeyCanvasPolicy::CanvasFocusOnly => canvas_focus,
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
        ShortcutContext::EditContext => match environment.active_view {
            ViewType::Schematic | ViewType::Testbench => environment.workspace == Workspace::Design,
            ViewType::Symbol => {
                matches!(environment.workspace, Workspace::Design | Workspace::Models)
            }
            _ => false,
        },
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
    }
}

fn timeout_duration(policy: ChordTimeoutPolicy) -> Option<Duration> {
    policy.seconds().map(Duration::from_secs_f64)
}

#[cfg(test)]
mod tests {
    use egui::{Event, Key, Modifiers};

    use super::*;
    use crate::workbench::{ShortcutBindingSlot, ShortcutSequence, ShortcutStroke};

    fn primary() -> Modifiers {
        Modifiers {
            command: true,
            ctrl: true,
            ..Modifiers::NONE
        }
    }

    fn event(key: Key, modifiers: Modifiers, repeat: bool) -> Event {
        Event::Key {
            key,
            physical_key: Some(key),
            pressed: true,
            repeat,
            modifiers,
        }
    }

    fn snapshot(events: &[Event], non_canvas_focus: bool) -> ShortcutInputSnapshot {
        ShortcutInputSnapshot::from_events_for_test(events, non_canvas_focus)
    }

    fn design(canvas_focus: bool) -> ShortcutEnvironment {
        ShortcutEnvironment {
            workspace: Workspace::Design,
            active_view: ViewType::Schematic,
            canvas_focus,
        }
    }

    fn results() -> ShortcutEnvironment {
        ShortcutEnvironment {
            workspace: Workspace::Results,
            active_view: ViewType::Schematic,
            canvas_focus: false,
        }
    }

    fn models_symbol(canvas_focus: bool) -> ShortcutEnvironment {
        ShortcutEnvironment {
            workspace: Workspace::Models,
            active_view: ViewType::Symbol,
            canvas_focus,
        }
    }

    fn verification() -> ShortcutEnvironment {
        ShortcutEnvironment {
            workspace: Workspace::Verify,
            active_view: ViewType::Schematic,
            canvas_focus: false,
        }
    }

    fn bind(profile: &mut ShortcutPreferences, command: Command, strokes: Vec<ShortcutStroke>) {
        profile
            .set_binding(
                command,
                ShortcutBindingSlot::Primary,
                vec![CommandPlatform::Desktop],
                Some(ShortcutSequence::new(strokes).unwrap()),
            )
            .unwrap();
    }

    #[test]
    fn two_stroke_sequence_waits_and_dispatches_at_most_one_command() {
        let mut profile = ShortcutPreferences::default();
        bind(
            &mut profile,
            Command::CommandPalette,
            vec![
                ShortcutStroke::new(Key::K, true, false, false),
                ShortcutStroke::new(Key::P, true, false, false),
            ],
        );
        let mut state = ShortcutResolverState::default();
        let first = state.resolve(
            &snapshot(&[event(Key::K, primary(), false)], false),
            &profile,
            CommandPlatform::Desktop,
            design(false),
            Duration::ZERO,
            |_| true,
        );
        assert_eq!(first.command, None);
        assert_eq!(first.consume.len(), 1);
        let second = state.resolve(
            &snapshot(
                &[
                    event(Key::P, primary(), false),
                    event(Key::S, primary(), false),
                ],
                false,
            ),
            &profile,
            CommandPlatform::Desktop,
            design(false),
            Duration::from_millis(100),
            |_| true,
        );
        assert_eq!(second.command, Some(Command::CommandPalette));
        assert_eq!(second.consume.len(), 2);
        let deferred = state.resolve(
            &ShortcutInputSnapshot::empty_for_test(false),
            &profile,
            CommandPlatform::Desktop,
            design(false),
            Duration::from_millis(101),
            |_| true,
        );
        assert_eq!(deferred.command, Some(Command::Save));
    }

    #[test]
    fn exact_prefix_dispatches_on_injected_timeout_and_clock_never_rewinds() {
        let mut profile = ShortcutPreferences::default();
        bind(
            &mut profile,
            Command::Preferences,
            vec![ShortcutStroke::new(Key::Q, true, false, false)],
        );
        bind(
            &mut profile,
            Command::ZoomFit,
            vec![
                ShortcutStroke::new(Key::Q, true, false, false),
                ShortcutStroke::new(Key::P, true, false, false),
            ],
        );
        let mut state = ShortcutResolverState::default();
        let first = state.resolve(
            &snapshot(&[event(Key::Q, primary(), false)], false),
            &profile,
            CommandPlatform::Desktop,
            design(false),
            Duration::from_secs(5),
            |_| true,
        );
        assert_eq!(first.command, None);
        let not_rewound = state.resolve(
            &ShortcutInputSnapshot::empty_for_test(false),
            &profile,
            CommandPlatform::Desktop,
            design(false),
            Duration::from_secs(1),
            |_| true,
        );
        assert_eq!(not_rewound.command, None);
        let expired = state.resolve(
            &ShortcutInputSnapshot::empty_for_test(false),
            &profile,
            CommandPlatform::Desktop,
            design(false),
            Duration::from_millis(6_500),
            |_| true,
        );
        assert_eq!(expired.command, Some(Command::Preferences));
    }

    #[test]
    fn no_timeout_waits_until_a_mismatch_then_dispatches_exact_prefix() {
        let mut profile = ShortcutPreferences::default();
        profile
            .policies_mut()
            .set_chord_timeout(ChordTimeoutPolicy::NoTimeout);
        bind(
            &mut profile,
            Command::Preferences,
            vec![ShortcutStroke::new(Key::Q, true, false, false)],
        );
        bind(
            &mut profile,
            Command::ZoomFit,
            vec![
                ShortcutStroke::new(Key::Q, true, false, false),
                ShortcutStroke::new(Key::P, true, false, false),
            ],
        );
        let mut state = ShortcutResolverState::default();
        state.resolve(
            &snapshot(&[event(Key::Q, primary(), false)], false),
            &profile,
            CommandPlatform::Desktop,
            design(false),
            Duration::ZERO,
            |_| true,
        );
        let waiting = state.resolve(
            &ShortcutInputSnapshot::empty_for_test(false),
            &profile,
            CommandPlatform::Desktop,
            design(false),
            Duration::from_secs(60),
            |_| true,
        );
        assert_eq!(waiting.command, None);
        assert_eq!(waiting.repaint_after, None);
        let mismatch = state.resolve(
            &snapshot(&[event(Key::S, primary(), false)], false),
            &profile,
            CommandPlatform::Desktop,
            design(false),
            Duration::from_secs(61),
            |_| true,
        );
        assert_eq!(mismatch.command, Some(Command::Preferences));
        assert_eq!(mismatch.consume.len(), 1);
        let deferred = state.resolve(
            &ShortcutInputSnapshot::empty_for_test(false),
            &profile,
            CommandPlatform::Desktop,
            design(false),
            Duration::from_secs(62),
            |_| true,
        );
        assert_eq!(deferred.command, Some(Command::Save));
        assert!(deferred.consume.is_empty());
    }

    #[test]
    fn repeat_release_and_wrong_platform_do_not_resolve() {
        let release = Event::Key {
            key: Key::K,
            physical_key: Some(Key::K),
            pressed: false,
            repeat: false,
            modifiers: primary(),
        };
        let ignored_events = snapshot(&[event(Key::K, primary(), true), release], false);
        let mut state = ShortcutResolverState::default();
        assert_eq!(
            state
                .resolve(
                    &ignored_events,
                    &ShortcutPreferences::default(),
                    CommandPlatform::Browser,
                    design(false),
                    Duration::ZERO,
                    |_| true,
                )
                .command,
            None
        );

        let mut desktop_only = ShortcutPreferences::default();
        bind(
            &mut desktop_only,
            Command::CommandPalette,
            vec![ShortcutStroke::new(Key::F12, false, false, false)],
        );
        let browser = state.resolve(
            &snapshot(&[event(Key::F12, Modifiers::NONE, false)], false),
            &desktop_only,
            CommandPlatform::Browser,
            design(false),
            Duration::from_millis(1),
            |_| true,
        );
        assert_eq!(browser.command, None);
        assert!(browser.consume.is_empty());
        let desktop = state.resolve(
            &snapshot(&[event(Key::F12, Modifiers::NONE, false)], false),
            &desktop_only,
            CommandPlatform::Desktop,
            design(false),
            Duration::from_millis(2),
            |_| true,
        );
        assert_eq!(desktop.command, Some(Command::CommandPalette));
    }

    #[test]
    fn disabled_higher_candidate_does_not_consume_lower_eligible_command() {
        let mut profile = ShortcutPreferences::default();
        bind(
            &mut profile,
            Command::ZoomFit,
            vec![ShortcutStroke::new(Key::K, true, false, false)],
        );
        bind(
            &mut profile,
            Command::CommandPalette,
            vec![ShortcutStroke::new(Key::K, true, false, false)],
        );
        profile
            .policies_mut()
            .set_context_precedence(ContextPrecedencePolicy::EditorModalWorkspaceGlobal);
        let mut state = ShortcutResolverState::default();
        let resolved = state.resolve(
            &snapshot(&[event(Key::K, primary(), false)], false),
            &profile,
            CommandPlatform::Desktop,
            design(true),
            Duration::ZERO,
            |command| command != Command::ZoomFit,
        );
        assert_eq!(resolved.command, Some(Command::CommandPalette));
        assert_eq!(resolved.consume.len(), 1);
    }

    #[test]
    fn disabled_only_candidate_does_not_consume_the_key() {
        let mut profile = ShortcutPreferences::default();
        bind(
            &mut profile,
            Command::CommandPalette,
            vec![ShortcutStroke::new(Key::K, true, false, false)],
        );
        let mut state = ShortcutResolverState::default();
        let resolved = state.resolve(
            &snapshot(&[event(Key::K, primary(), false)], false),
            &profile,
            CommandPlatform::Desktop,
            design(false),
            Duration::ZERO,
            |_| false,
        );
        assert_eq!(resolved.command, None);
        assert!(resolved.consume.is_empty());
    }

    #[test]
    fn invalid_whole_profile_executes_immutable_defaults_only() {
        let source = serde_json::json!({
            "policies": {
                "single-key-canvas": "disabled",
                "chord-timeout": "no-timeout",
                "context-precedence": "editor-modal-workspace-global"
            },
            "commands": {
                Command::Save.stable_id(): {"bindings": [{
                    "slot": "primary",
                    "platforms": ["desktop"],
                    "sequence": [{"key": "DefinitelyNotAKey"}]
                }]},
                Command::CommandPalette.stable_id(): {"bindings": [{
                    "slot": "primary",
                    "platforms": ["desktop"],
                    "sequence": [{"key": "F12"}]
                }]}
            }
        });
        let profile: ShortcutPreferences = serde_json::from_value(source).unwrap();
        assert!(!profile.is_effective_profile_valid());
        let mut state = ShortcutResolverState::default();
        let fallback = state.resolve(
            &snapshot(&[event(Key::K, primary(), false)], false),
            &profile,
            CommandPlatform::Desktop,
            design(false),
            Duration::ZERO,
            |_| true,
        );
        assert_eq!(fallback.command, Some(Command::CommandPalette));
        let rejected_override = state.resolve(
            &snapshot(&[event(Key::F12, Modifiers::NONE, false)], false),
            &profile,
            CommandPlatform::Desktop,
            design(false),
            Duration::from_millis(1),
            |_| true,
        );
        assert_eq!(rejected_override.command, None);
        let rejected_policy = state.resolve(
            &snapshot(&[event(Key::W, Modifiers::NONE, false)], false),
            &profile,
            CommandPlatform::Desktop,
            design(true),
            Duration::from_millis(2),
            |_| true,
        );
        assert_eq!(rejected_policy.command, Some(Command::PlaceWire));
    }

    #[test]
    fn canvas_focus_policy_requires_exact_canvas_focus() {
        let profile = ShortcutPreferences::default();
        let mut state = ShortcutResolverState::default();
        let unfocused = state.resolve(
            &snapshot(&[event(Key::W, Modifiers::NONE, false)], false),
            &profile,
            CommandPlatform::Desktop,
            design(false),
            Duration::ZERO,
            |_| true,
        );
        assert_eq!(unfocused.command, None);
        let focused = state.resolve(
            &snapshot(&[event(Key::W, Modifiers::NONE, false)], false),
            &profile,
            CommandPlatform::Desktop,
            design(true),
            Duration::from_millis(1),
            |_| true,
        );
        assert_eq!(focused.command, Some(Command::PlaceWire));

        let model_symbol = state.resolve(
            &snapshot(&[event(Key::W, Modifiers::NONE, false)], false),
            &profile,
            CommandPlatform::Desktop,
            models_symbol(true),
            Duration::from_millis(2),
            |_| true,
        );
        assert_eq!(model_symbol.command, Some(Command::SymbolPolylineTool));
    }

    #[test]
    fn complete_symbol_tool_surface_resolves_through_the_central_registry() {
        let profile = ShortcutPreferences::default();
        let mut state = ShortcutResolverState::default();
        for (time, key, expected) in [
            (0, Key::S, Command::SelectTool),
            (1, Key::P, Command::SymbolPinTool),
            (2, Key::W, Command::SymbolPolylineTool),
            (3, Key::C, Command::SymbolCircleTool),
            (4, Key::A, Command::SymbolArcTool),
            (5, Key::D, Command::SymbolArrowTool),
            (6, Key::O, Command::SymbolDotTool),
            (7, Key::F, Command::ZoomFit),
        ] {
            let resolved = state.resolve(
                &snapshot(&[event(key, Modifiers::NONE, false)], false),
                &profile,
                CommandPlatform::Desktop,
                models_symbol(true),
                Duration::from_millis(time),
                |_| true,
            );
            assert_eq!(resolved.command, Some(expected), "key {key:?}");
        }
    }

    #[test]
    fn result_context_beats_global_only_when_enabled() {
        let mut profile = ShortcutPreferences::default();
        bind(
            &mut profile,
            Command::WaveformCalculator,
            vec![ShortcutStroke::new(Key::K, true, false, false)],
        );
        bind(
            &mut profile,
            Command::CommandPalette,
            vec![ShortcutStroke::new(Key::K, true, false, false)],
        );
        let mut state = ShortcutResolverState::default();
        let result = state.resolve(
            &snapshot(&[event(Key::K, primary(), false)], false),
            &profile,
            CommandPlatform::Desktop,
            results(),
            Duration::ZERO,
            |_| true,
        );
        assert_eq!(result.command, Some(Command::WaveformCalculator));
    }

    #[test]
    fn three_second_policy_uses_the_injected_monotonic_deadline() {
        let mut profile = ShortcutPreferences::default();
        profile
            .policies_mut()
            .set_chord_timeout(ChordTimeoutPolicy::ThreeSeconds);
        bind(
            &mut profile,
            Command::Preferences,
            vec![ShortcutStroke::new(Key::Q, true, false, false)],
        );
        bind(
            &mut profile,
            Command::ZoomFit,
            vec![
                ShortcutStroke::new(Key::Q, true, false, false),
                ShortcutStroke::new(Key::P, true, false, false),
            ],
        );
        let mut state = ShortcutResolverState::default();
        state.resolve(
            &snapshot(&[event(Key::Q, primary(), false)], false),
            &profile,
            CommandPlatform::Desktop,
            design(false),
            Duration::ZERO,
            |_| true,
        );
        assert_eq!(
            state
                .resolve(
                    &ShortcutInputSnapshot::empty_for_test(false),
                    &profile,
                    CommandPlatform::Desktop,
                    design(false),
                    Duration::from_millis(2_999),
                    |_| true,
                )
                .command,
            None
        );
        assert_eq!(
            state
                .resolve(
                    &ShortcutInputSnapshot::empty_for_test(false),
                    &profile,
                    CommandPlatform::Desktop,
                    design(false),
                    Duration::from_secs(3),
                    |_| true,
                )
                .command,
            Some(Command::Preferences)
        );
    }

    #[test]
    fn four_stroke_sequence_is_supported_without_dispatching_twice() {
        let mut profile = ShortcutPreferences::default();
        bind(
            &mut profile,
            Command::Preferences,
            vec![
                ShortcutStroke::new(Key::Q, true, false, false),
                ShortcutStroke::new(Key::W, true, false, false),
                ShortcutStroke::new(Key::E, true, false, false),
                ShortcutStroke::new(Key::R, true, false, false),
            ],
        );
        let events = [
            event(Key::Q, primary(), false),
            event(Key::W, primary(), false),
            event(Key::E, primary(), false),
            event(Key::R, primary(), false),
        ];
        let mut state = ShortcutResolverState::default();
        let resolution = state.resolve(
            &snapshot(&events, false),
            &profile,
            CommandPlatform::Desktop,
            design(false),
            Duration::ZERO,
            |_| true,
        );
        assert_eq!(resolution.command, Some(Command::Preferences));
        assert_eq!(resolution.consume.len(), 4);
    }

    #[test]
    fn single_key_canvas_require_alt_and_disabled_use_shared_projection() {
        let mut require_alt = ShortcutPreferences::default();
        require_alt
            .policies_mut()
            .set_single_key_canvas(SingleKeyCanvasPolicy::RequireAlt);
        let mut state = ShortcutResolverState::default();
        let plain = state.resolve(
            &snapshot(&[event(Key::W, Modifiers::NONE, false)], false),
            &require_alt,
            CommandPlatform::Desktop,
            design(false),
            Duration::ZERO,
            |_| true,
        );
        assert_eq!(plain.command, None);
        let alt = state.resolve(
            &snapshot(
                &[event(
                    Key::W,
                    Modifiers {
                        alt: true,
                        ..Modifiers::NONE
                    },
                    false,
                )],
                false,
            ),
            &require_alt,
            CommandPlatform::Desktop,
            design(false),
            Duration::from_millis(1),
            |_| true,
        );
        assert_eq!(alt.command, Some(Command::PlaceWire));

        let mut disabled = ShortcutPreferences::default();
        disabled
            .policies_mut()
            .set_single_key_canvas(SingleKeyCanvasPolicy::Disabled);
        let disabled_result = state.resolve(
            &snapshot(
                &[event(
                    Key::W,
                    Modifiers {
                        alt: true,
                        ..Modifiers::NONE
                    },
                    false,
                )],
                false,
            ),
            &disabled,
            CommandPlatform::Desktop,
            design(true),
            Duration::from_millis(2),
            |_| true,
        );
        assert_eq!(disabled_result.command, None);
    }

    #[test]
    fn application_chrome_and_editor_precedence_policy_changes_real_selection() {
        let mut chrome_first = ShortcutPreferences::default();
        bind(
            &mut chrome_first,
            Command::Cancel,
            vec![ShortcutStroke::new(Key::Q, true, false, false)],
        );
        bind(
            &mut chrome_first,
            Command::ZoomFit,
            vec![ShortcutStroke::new(Key::Q, true, false, false)],
        );
        let mut state = ShortcutResolverState::default();
        let chrome = state.resolve(
            &snapshot(&[event(Key::Q, primary(), false)], false),
            &chrome_first,
            CommandPlatform::Desktop,
            design(true),
            Duration::ZERO,
            |_| true,
        );
        assert_eq!(chrome.command, Some(Command::Cancel));

        let mut editor_first = chrome_first;
        editor_first
            .policies_mut()
            .set_context_precedence(ContextPrecedencePolicy::EditorModalWorkspaceGlobal);
        let editor = state.resolve(
            &snapshot(&[event(Key::Q, primary(), false)], false),
            &editor_first,
            CommandPlatform::Desktop,
            design(true),
            Duration::from_millis(1),
            |_| true,
        );
        assert_eq!(editor.command, Some(Command::ZoomFit));
    }

    #[test]
    fn non_canvas_focus_suppresses_editor_but_not_global_context() {
        let profile = ShortcutPreferences::default();
        let mut state = ShortcutResolverState::default();
        let editor = state.resolve(
            &snapshot(&[event(Key::W, Modifiers::NONE, false)], true),
            &profile,
            CommandPlatform::Desktop,
            design(true),
            Duration::ZERO,
            |_| true,
        );
        assert_eq!(editor.command, None);
        let global = state.resolve(
            &snapshot(&[event(Key::K, primary(), false)], true),
            &profile,
            CommandPlatform::Desktop,
            design(false),
            Duration::from_millis(1),
            |_| true,
        );
        assert_eq!(global.command, Some(Command::CommandPalette));
    }

    #[test]
    fn next_marker_primary_and_alternate_resolve_in_verify_then_design() {
        let profile = ShortcutPreferences::default();
        let mut state = ShortcutResolverState::default();
        for (time, key, environment) in [
            (0, Key::CloseBracket, verification()),
            (1, Key::F8, design(false)),
        ] {
            let resolved = state.resolve(
                &snapshot(&[event(key, Modifiers::NONE, false)], false),
                &profile,
                CommandPlatform::Desktop,
                environment,
                Duration::from_millis(time),
                |command| command == Command::NextViolation,
            );
            assert_eq!(resolved.command, Some(Command::NextViolation));
        }
    }
}
