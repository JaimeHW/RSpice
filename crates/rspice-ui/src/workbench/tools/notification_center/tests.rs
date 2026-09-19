//! What the panel says, where it says it, and what a press on it does.
//!
//! The panel is found the way a reader finds it: its text is read back from
//! the shapes it painted, and its controls are pressed at the rectangles they
//! announced to AccessKit — so a control that reaches nobody cannot be pressed
//! here either.

use egui::{Rect, pos2, vec2};

use super::*;
use crate::ui::widgets::{MirroredEntry, ToastKind};
use crate::workbench::state::ConsolePage;
use crate::workbench::{RouteTransitionSource, SurfaceId, SurfaceRoute};

const TITLE_BAR: f32 = 35.0;
const DESKTOP: (f32, f32) = (1280.0, 760.0);
const TABLET: (f32, f32) = (820.0, 640.0);
const PHONE: (f32, f32) = (390.0, 780.0);

/// A press and a release at one point, which is what egui reads as a click.
fn click_events(at: Pos2) -> Vec<egui::Event> {
    vec![
        egui::Event::PointerMoved(at),
        egui::Event::PointerButton {
            pos: at,
            button: egui::PointerButton::Primary,
            pressed: true,
            modifiers: egui::Modifiers::default(),
        },
        egui::Event::PointerButton {
            pos: at,
            button: egui::PointerButton::Primary,
            pressed: false,
            modifiers: egui::Modifiers::default(),
        },
    ]
}

fn key_events(key: egui::Key) -> Vec<egui::Event> {
    [true, false]
        .into_iter()
        .map(|pressed| egui::Event::Key {
            key,
            physical_key: None,
            pressed,
            repeat: false,
            modifiers: egui::Modifiers::default(),
        })
        .collect()
}

/// The panel held open across several presses in one session.
struct Panel {
    ctx: egui::Context,
    app: RSpiceApp,
    size: (f32, f32),
    /// What each control announced, and where.
    controls: Vec<(String, Rect)>,
    /// Every string painted in the last pass, and where.
    texts: Vec<(String, Rect)>,
}

/// Four notices across both domains and both time groups: a finished run and a
/// failure just now, a warning and a routine line from earlier. Shared with
/// the renders next door, so what is looked at is what is asserted about.
pub(super) fn seed_session(ctx: &egui::Context, toasts: &mut Toasts) {
    toasts.synchronize_activity(
        2,
        [
            MirroredEntry {
                log_id: 0,
                category: NotificationCategory::System,
                kind: ToastKind::Info,
                message: "Recovery copy saved for amplifier.rsp".to_owned(),
                action: None,
                created: -7_200.0,
            },
            MirroredEntry {
                log_id: 1,
                category: NotificationCategory::Job,
                kind: ToastKind::Warn,
                message: "2 of 148 models were skipped: unsupported LEVEL in \
                          bsim_legacy.lib, and the rest of this sentence is here to run \
                          past the second line so the row has to cut it short."
                    .to_owned(),
                action: None,
                created: -1_800.0,
            },
        ],
    );
    toasts.notify_with_action(
        ctx,
        NotificationCategory::Job,
        ToastKind::Error,
        "Run 40 failed",
        "No analysis result was retained, so there is nothing to open in Results.",
        NotificationAction::ShowInConsole,
    );
    toasts.notify_with_action(
        ctx,
        NotificationCategory::Job,
        ToastKind::Success,
        "Run 41 complete",
        "3 retained analyses in this immutable dataset.",
        NotificationAction::OpenRunInResults { run_sequence: 41 },
    );
}

impl Panel {
    fn seeded(size: (f32, f32)) -> Self {
        Self::open(size, seed_session)
    }

    /// The seed is handed the notice stream and nothing else: that is all a
    /// fixture for this panel has any business writing to.
    fn open(size: (f32, f32), seed: impl FnOnce(&egui::Context, &mut Toasts)) -> Self {
        let ctx = egui::Context::default();
        crate::ui::Theme::default().apply(&ctx);
        ctx.enable_accesskit();
        let mut app = RSpiceApp::test_instance();
        seed(&ctx, &mut app.state.ui.toasts);
        app.state.workbench.notification_center_open = true;
        let mut panel = Self {
            ctx,
            app,
            size,
            controls: Vec::new(),
            texts: Vec::new(),
        };
        // An `Area` measures itself on its first pass and the fonts settle on
        // the second; the third is the one a reader would see.
        for _ in 0..3 {
            panel.pass(Vec::new());
        }
        panel
    }

    fn screen(&self) -> Rect {
        Rect::from_min_size(Pos2::ZERO, vec2(self.size.0, self.size.1))
    }

    /// Where the panel is, by the rule that put it there. No bell is drawn in
    /// this harness, so it hangs from the shell's trailing edge.
    fn bounds(&self) -> Rect {
        let placement = PanelPlacement::resolve(self.screen(), None, TITLE_BAR);
        Rect::from_min_size(placement.min, vec2(placement.width, placement.max_height))
    }

    fn pass(&mut self, events: Vec<egui::Event>) {
        /// A string is where it can be *seen*: a row scrolled under the foot
        /// is cut at the list's edge, and is not on top of the foot.
        fn walk(shape: &egui::Shape, clip: Rect, out: &mut Vec<(String, Rect)>) {
            match shape {
                egui::Shape::Text(text) => {
                    let seen = text.visual_bounding_rect().intersect(clip);
                    if seen.is_positive() {
                        out.push((text.galley.text().to_owned(), seen));
                    }
                }
                egui::Shape::Vec(shapes) => {
                    shapes.iter().for_each(|shape| walk(shape, clip, out));
                }
                _ => {}
            }
        }

        let app = &mut self.app;
        let output = self.ctx.run_ui(
            egui::RawInput {
                screen_rect: Some(Rect::from_min_size(
                    Pos2::ZERO,
                    vec2(self.size.0, self.size.1),
                )),
                events,
                ..egui::RawInput::default()
            },
            |ui| show(ui.ctx(), app, TITLE_BAR),
        );
        self.texts.clear();
        for clipped in &output.shapes {
            walk(&clipped.shape, clipped.clip_rect, &mut self.texts);
        }
        self.controls = output
            .platform_output
            .accesskit_update
            .map(|update| {
                update
                    .nodes
                    .iter()
                    .filter_map(|(_, node)| {
                        let bounds = node.bounds()?;
                        Some((
                            node.label()?.to_owned(),
                            Rect::from_min_max(
                                pos2(bounds.x0 as f32, bounds.y0 as f32),
                                pos2(bounds.x1 as f32, bounds.y1 as f32),
                            ),
                        ))
                    })
                    .collect()
            })
            .unwrap_or_default();
    }

    fn control(&self, matches: impl Fn(&str) -> bool) -> Rect {
        let hits: Vec<_> = self
            .controls
            .iter()
            .filter(|(label, _)| matches(label))
            .collect();
        assert_eq!(
            hits.len(),
            1,
            "expected exactly one matching control; the panel announces: {:#?}",
            self.controls
                .iter()
                .map(|(label, _)| label.as_str())
                .collect::<Vec<_>>()
        );
        hits[0].1
    }

    /// Press the one control whose announcement satisfies `matches`, then
    /// settle the pass the press produced.
    fn click(&mut self, matches: impl Fn(&str) -> bool) {
        let at = self.control(matches).center();
        self.pass(click_events(at));
        self.pass(Vec::new());
    }

    fn announces(&self, matches: impl Fn(&str) -> bool) -> bool {
        self.controls.iter().any(|(label, _)| matches(label))
    }

    fn paints(&self, wanted: &str) -> bool {
        self.texts.iter().any(|(text, _)| text == wanted)
    }

    fn painted(&self) -> Vec<&str> {
        self.texts.iter().map(|(text, _)| text.as_str()).collect()
    }

    fn showing(&self) -> bool {
        self.app.state.workbench.notification_center_showing()
    }

    fn toasts(&self) -> &Toasts {
        &self.app.state.ui.toasts
    }
}

#[test]
fn the_panel_states_the_session_in_one_grammar() {
    let panel = Panel::seeded(DESKTOP);
    for owed in [
        "Notifications",
        "4 unread",
        "Mark all read",
        "All",
        "Jobs",
        "System",
        "Unread only",
        "RECENT",
        "EARLIER THIS SESSION",
        "Run 41 complete",
        "Run 40 failed",
        "Warning",
        "Information",
        "Job \u{00b7} now",
        "Job \u{00b7} 30 min",
        "System \u{00b7} 2 hr",
        "Open in Results",
        "Show in Console",
        "This session \u{00b7} 4 of 50 kept",
        "Clear read",
    ] {
        assert!(
            panel.paints(owed),
            "the panel owes {owed:?} and painted {:#?}",
            panel.painted()
        );
    }
    // The counts ride beside the filters they belong to.
    for (segment, count) in [("All", 4), ("Jobs", 3), ("System", 1)] {
        assert!(
            panel.announces(|label| label == format!("{segment}, {count}")),
            "{segment} must announce that it holds {count}"
        );
    }
}

/// Nothing the panel paints leaves the panel, and no two strings share
/// ground, at the three sizes the shell is gated at.
#[test]
fn every_string_stays_inside_the_panel_and_clear_of_its_neighbours() {
    for size in [DESKTOP, TABLET, PHONE] {
        let panel = Panel::seeded(size);
        let bounds = panel.bounds().expand(0.5);
        assert!(
            panel.screen().contains_rect(panel.bounds()),
            "{size:?}: the panel {:?} leaves the screen",
            panel.bounds()
        );
        for (text, rect) in &panel.texts {
            assert!(
                bounds.contains_rect(*rect),
                "{size:?}: {text:?} at {rect:?} leaves the panel {bounds:?}"
            );
        }
        for (index, (text, rect)) in panel.texts.iter().enumerate() {
            for (other, other_rect) in &panel.texts[index + 1..] {
                assert!(
                    !rect.shrink(0.5).intersects(other_rect.shrink(0.5)),
                    "{size:?}: {text:?} at {rect:?} overlaps {other:?} at {other_rect:?}"
                );
            }
        }
    }
}

/// A console paragraph is one row, not the list: the message is held to two
/// lines, and says so with an ellipsis rather than by being clipped.
#[test]
fn a_long_message_is_cut_to_two_lines_and_marked_as_cut() {
    let panel = Panel::seeded(DESKTOP);
    let (text, rect) = panel
        .texts
        .iter()
        .find(|(text, _)| text.starts_with("2 of 148 models"))
        .expect("the warning's message is painted");
    assert!(
        rect.height() <= MESSAGE_LINE_HEIGHT * MESSAGE_MAX_ROWS as f32 + 1.0,
        "{text:?} runs to {} points",
        rect.height()
    );
}

#[test]
fn pressing_a_row_marks_that_row_read_and_no_other() {
    let mut panel = Panel::seeded(DESKTOP);
    panel.click(|label| label.contains("unread: Run 40 failed"));

    assert_eq!(panel.toasts().unread_count(), 3);
    assert!(panel.paints("3 unread"));
    assert!(
        panel.announces(|label| label.starts_with("Job Error: Run 40 failed")),
        "a read row stops announcing itself as unread, and stays where it was"
    );
    assert!(panel.showing(), "reading a row is not leaving the panel");
}

#[test]
fn mark_all_read_empties_the_unread_view_and_then_has_nothing_to_do() {
    let mut panel = Panel::seeded(DESKTOP);
    panel.click(|label| label == "Unread only");
    assert!(panel.paints("Run 41 complete"));

    panel.click(|label| label == "Mark all read");
    assert_eq!(panel.toasts().unread_count(), 0);
    assert!(panel.paints("Nothing unread"));
    assert!(!panel.paints("4 unread"));

    // Spent: a second press changes nothing because it reaches nothing.
    let at = panel.control(|label| label == "Mark all read").center();
    panel.pass(click_events(at));
    assert_eq!(panel.toasts().activity().len(), 4);
}

#[test]
fn a_domain_filter_hides_the_other_domain_without_discarding_it() {
    let mut panel = Panel::seeded(DESKTOP);
    panel.click(|label| label.starts_with("System,"));

    assert!(panel.paints("Information"));
    assert!(!panel.paints("Run 41 complete"));
    assert_eq!(panel.toasts().activity().len(), 4);
    assert_eq!(
        panel.app.state.workbench.notification_filter,
        NotificationFilter::System
    );
}

#[test]
fn a_filter_that_matches_nothing_says_why_the_list_is_empty() {
    let mut panel = Panel::open(DESKTOP, |ctx, toasts| {
        toasts.info(ctx, "project opened");
    });
    panel.click(|label| label.starts_with("Jobs,"));
    assert!(panel.paints("Nothing in this view"));

    let empty = Panel::open(DESKTOP, |_, _| {});
    assert!(empty.paints("No notifications yet"));
    assert!(empty.paints("This session \u{00b7} 0 of 50 kept"));
}

/// Under a pointer the dismiss mark arrives with the pointer; pressing it
/// removes that record and leaves the rest.
#[test]
fn a_row_under_the_pointer_offers_dismissal_and_dismissal_removes_it() {
    let mut panel = Panel::seeded(DESKTOP);
    assert!(
        !panel.announces(|label| label.starts_with("Dismiss ")),
        "no row is under the pointer, so no row carries the mark"
    );

    let ages = |panel: &Panel| {
        panel
            .texts
            .iter()
            .filter(|(text, _)| text == "Job \u{00b7} now")
            .count()
    };
    assert_eq!(ages(&panel), 2);
    let row = panel.control(|label| label.contains("unread: Run 41 complete"));
    panel.pass(vec![egui::Event::PointerMoved(row.center())]);
    panel.pass(Vec::new());
    assert_eq!(
        ages(&panel),
        1,
        "the hovered row gives up its age to the mark, and only that row"
    );
    panel.click(|label| label == "Dismiss Run 41 complete");

    assert_eq!(panel.toasts().activity().len(), 3);
    assert!(!panel.paints("Run 41 complete"));
    assert!(panel.paints("This session \u{00b7} 3 of 50 kept"));
}

/// Under a finger there is no hover to wait for: every row carries its mark,
/// and every target is at least the touch size.
#[test]
fn a_touch_sized_panel_shows_every_dismiss_mark_at_touch_size() {
    let panel = Panel::seeded(PHONE);
    let marks: Vec<_> = panel
        .controls
        .iter()
        .filter(|(label, _)| label.starts_with("Dismiss "))
        .collect();
    assert_eq!(marks.len(), 4);
    for (label, rect) in &panel.controls {
        let pressable = label.starts_with("Dismiss ")
            || label.starts_with("Open in Results: ")
            || label.starts_with("Show in Console: ")
            || matches!(
                label.as_str(),
                "Mark all read" | "Clear read" | "Unread only"
            );
        if pressable {
            assert!(
                rect.height() >= tokens::TOUCH_TARGET - 0.5,
                "{label:?} is {} points tall under a finger",
                rect.height()
            );
        }
    }
}

/// Touch size is a taller press and not a taller control: the filter band is
/// the height it is on a desktop, and what grows is what answers the finger.
#[test]
fn the_filter_takes_a_finger_without_growing_taller() {
    let drop_to_list = |panel: &Panel| {
        let of = |wanted: &str| {
            panel
                .texts
                .iter()
                .find(|(text, _)| text == wanted)
                .unwrap_or_else(|| panic!("{wanted:?} is not painted"))
                .1
                .center()
                .y
        };
        of("RECENT") - of("All")
    };
    let desktop = Panel::seeded(DESKTOP);
    let authored = drop_to_list(&desktop);
    for size in [TABLET, PHONE] {
        let panel = Panel::seeded(size);
        let drop = drop_to_list(&panel);
        assert!(
            (drop - authored).abs() <= 0.5,
            "{size:?}: the list starts {drop} points under the filter's label, \
             {authored} on a desktop"
        );
        let heading = panel
            .texts
            .iter()
            .find(|(text, _)| text == "RECENT")
            .expect("group heading")
            .1;
        for segment in ["All, 4", "Jobs, 3", "System, 1"] {
            let press = panel.control(|label| label == segment);
            assert!(
                press.height() >= tokens::TOUCH_TARGET - 0.5,
                "{size:?}: {segment:?} answers {} points of finger",
                press.height()
            );
            assert!(
                press.bottom() <= heading.top(),
                "{size:?}: {segment:?} answers presses meant for the list"
            );
        }
    }
}

#[test]
fn following_an_offer_reads_the_notice_closes_the_panel_and_goes_there() {
    let mut panel = Panel::seeded(DESKTOP);
    panel.app.state.workbench.console_visible = false;
    panel.app.state.workbench.console_page = ConsolePage::Problems;
    panel.click(|label| label == "Show in Console: Run 40 failed");

    assert!(
        !panel.showing(),
        "the offer leaves, and the panel goes with it"
    );
    assert!(panel.app.state.workbench.console_visible);
    assert_eq!(panel.app.state.workbench.console_page, ConsolePage::Console);
    let followed = panel
        .toasts()
        .activity()
        .iter()
        .find(|record| record.title() == "Run 40 failed")
        .expect("following a notice does not discard it");
    assert!(followed.is_read(), "following a notice is having read it");
}

#[test]
fn clear_read_drops_what_was_read_and_keeps_what_was_not() {
    let mut panel = Panel::seeded(DESKTOP);
    panel.click(|label| label.contains("unread: Run 40 failed"));
    panel.click(|label| label == "Clear read");

    assert_eq!(panel.toasts().activity().len(), 3);
    assert!(!panel.paints("Run 40 failed"));
    assert!(panel.paints("Run 41 complete"));
}

#[test]
fn escape_and_a_press_elsewhere_close_the_panel_and_a_press_inside_does_not() {
    let mut panel = Panel::seeded(DESKTOP);
    let inside = panel.bounds().center_top() + vec2(0.0, 20.0);
    panel.pass(click_events(inside));
    panel.pass(Vec::new());
    assert!(panel.showing(), "the panel's own head is not elsewhere");

    panel.pass(click_events(pos2(40.0, 400.0)));
    panel.pass(Vec::new());
    assert!(
        !panel.showing(),
        "a press on the workbench puts the panel away"
    );

    let mut panel = Panel::seeded(DESKTOP);
    panel.pass(key_events(egui::Key::Escape));
    assert!(!panel.showing());
}

#[test]
fn the_bell_is_a_toggle() {
    let mut state = AppState::default();
    assert!(!state.workbench.notification_center_showing());
    state.workbench.toggle_notification_center();
    assert!(state.workbench.notification_center_showing());
    assert!(
        state.workbench.application_modal_open(),
        "an open panel holds keyboard intent: shortcuts must not edit the document behind it"
    );
    state.workbench.toggle_notification_center();
    assert!(!state.workbench.notification_center_showing());
}

/// A phone-width shell draws no bell, so the panel must be reachable by name:
/// a registered command is what the palette searches.
#[test]
fn the_command_opens_the_panel_for_shells_that_draw_no_bell() {
    use crate::workbench::commands::vocabulary::{COMMAND_REGISTRY, Command};

    assert!(COMMAND_REGISTRY.contains(&Command::OpenNotifications));
    let mut app = RSpiceApp::test_instance();
    Command::OpenNotifications.execute(&mut app);
    assert!(app.state.workbench.notification_center_showing());
}

#[test]
fn a_deep_linked_panel_closes_back_to_where_it_was_opened_from() {
    let mut state = AppState::default();
    state
        .workbench
        .navigate(
            SurfaceRoute::surface(SurfaceId::NotificationCenter),
            RouteTransitionSource::BrowserPop,
        )
        .expect("notification route is executable");
    assert!(state.workbench.notification_center_showing());

    state.workbench.toggle_notification_center();
    assert!(!state.workbench.notification_center_showing());
    assert_ne!(
        state.workbench.current_route().surface_id(),
        SurfaceId::NotificationCenter
    );
}

#[test]
fn filters_project_domains_without_mutating_activity() {
    assert!(filter_includes(
        NotificationFilter::All,
        NotificationCategory::System
    ));
    assert!(filter_includes(
        NotificationFilter::Jobs,
        NotificationCategory::Job
    ));
    assert!(!filter_includes(
        NotificationFilter::Jobs,
        NotificationCategory::System
    ));
    assert!(!filter_includes(
        NotificationFilter::System,
        NotificationCategory::Job
    ));
}

#[test]
fn ages_are_session_relative_bounded_and_grouped_at_a_quarter_hour() {
    assert_eq!(age_label(2.0, 4.0), "now");
    assert_eq!(age_label(59.0, 0.0), "now");
    assert_eq!(age_label(180.0, 0.0), "3 min");
    assert_eq!(age_label(7_200.0, 0.0), "2 hr");

    assert!(is_recent(899.0, 0.0));
    assert!(!is_recent(900.0, 0.0));
}

#[test]
fn the_panel_hangs_under_the_bell_and_never_leaves_the_screen() {
    let screen = Rect::from_min_size(Pos2::ZERO, vec2(1280.0, 760.0));
    let bell = Rect::from_min_size(pos2(1180.0, 4.0), vec2(28.0, 27.0));
    let placed = PanelPlacement::resolve(screen, Some(bell), TITLE_BAR);

    assert_eq!(placed.width, PANEL_WIDTH);
    assert_eq!(placed.min.y, bell.bottom() + ANCHOR_GAP);
    assert_eq!(placed.min.x + placed.width, bell.right() + ANCHOR_OVERHANG);
    assert_eq!(placed.caret_x, Some(bell.center().x));
    assert_eq!(placed.max_height, PANEL_MAX_HEIGHT);

    // A bell hard against the trailing edge: the panel stops at the margin
    // and the caret, which would land on the corner, is not drawn.
    let cornered = Rect::from_min_size(pos2(1262.0, 4.0), vec2(16.0, 27.0));
    let placed = PanelPlacement::resolve(screen, Some(cornered), TITLE_BAR);
    assert_eq!(placed.min.x + placed.width, screen.right() - SCREEN_MARGIN);
    assert_eq!(placed.caret_x, None);

    // A short window: the panel gives up height before it leaves the screen.
    let short = Rect::from_min_size(Pos2::ZERO, vec2(1280.0, 420.0));
    let placed = PanelPlacement::resolve(short, Some(bell), TITLE_BAR);
    assert!(placed.min.y + placed.max_height <= short.bottom());
    assert!(placed.max_height < PANEL_MAX_HEIGHT);
}

#[test]
fn a_phone_gets_a_sheet_under_the_chrome_with_no_caret() {
    let screen = Rect::from_min_size(Pos2::ZERO, vec2(390.0, 780.0));
    let placed = PanelPlacement::resolve(screen, None, 40.0);

    assert_eq!(placed.min, pos2(SCREEN_MARGIN, 40.0 + ANCHOR_GAP));
    assert_eq!(placed.width, 390.0 - SCREEN_MARGIN * 2.0);
    assert_eq!(placed.max_height, 780.0 - SCREEN_MARGIN - placed.min.y);
    assert_eq!(placed.caret_x, None);

    let nothing = PanelPlacement::resolve(Rect::from_min_size(Pos2::ZERO, Vec2::ZERO), None, 0.0);
    assert_eq!((nothing.width, nothing.max_height), (0.0, 0.0));
}

#[test]
fn an_empty_list_says_why_it_is_empty() {
    assert_eq!(empty_copy(true, false).0, "No notifications yet");
    assert_eq!(empty_copy(true, true).0, "No notifications yet");
    assert_eq!(empty_copy(false, true).0, "Nothing unread");
    assert_eq!(empty_copy(false, false).0, "Nothing in this view");
}

#[test]
fn responsive_target_size_follows_the_shell_the_bell_is_in() {
    let ctx = egui::Context::default();
    let mut state = AppState::default();
    let at = |width: f32, state: &AppState| {
        let mut large = false;
        let _ = ctx.run_ui(
            egui::RawInput {
                screen_rect: Some(Rect::from_min_size(Pos2::ZERO, vec2(width, 700.0))),
                ..Default::default()
            },
            |ui| large = large_targets(ui.ctx(), state),
        );
        large
    };
    assert!(at(480.0, &state));
    assert!(!at(1_120.0, &state));
    state.workbench.coarse_pointer = true;
    assert!(at(1_120.0, &state));
}
