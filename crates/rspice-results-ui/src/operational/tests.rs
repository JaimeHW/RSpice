use super::*;
use egui::Rect;

/// The well the Results workspace hands a blocking state on a desktop.
const WELL: egui::Vec2 = egui::vec2(1280.0, 720.0);

#[test]
fn canonical_operational_registry_matches_the_result_contract() {
    assert_eq!(
        ResultOperationalState::ALL.map(ResultOperationalState::id),
        [
            "complete",
            "no-project",
            "no-dataset",
            "loading",
            "streaming",
            "partial",
            "stale",
            "failed",
            "corrupted",
            "unsupported",
            "offline",
            "storage-denied",
            "low-memory",
            "renderer-loss",
            "interrupted-operation",
            "recovered",
        ]
    );
    for state in ResultOperationalState::ALL {
        assert!(!state.label().is_empty());
        assert!(!state.message().is_empty());
        assert!(!state.recovery().is_empty());
    }
}

/// A blocking card is drawn once. It used to be laid out twice, once
/// invisibly to measure its height, and egui publishes the widgets of an
/// invisible pass too, so a screen reader met the card's title, message and
/// controls twice. The state itself is announced once, on the card shown.
#[test]
fn a_blocking_card_is_published_once() {
    let ctx = egui::Context::default();
    rspice_ui_kit::Theme::default().apply(&ctx);
    ctx.enable_accesskit();
    let status = ResultOperationalStatus::canonical(ResultOperationalState::Corrupted, true);
    assert!(status.blocks_visuals);
    let mut update = None;
    for _ in 0..3 {
        let input = egui::RawInput {
            screen_rect: Some(Rect::from_min_size(egui::Pos2::ZERO, WELL)),
            ..Default::default()
        };
        update = ctx
            .run_ui(input, |ui| {
                let _ = show(ui, &status, CardOffers::default);
            })
            .platform_output
            .accesskit_update;
    }
    let nodes = update.expect("AccessKit operational-card tree").nodes;
    let titles = nodes
        .iter()
        .filter(|(_, node)| {
            node.role() == egui::accesskit::Role::Label
                && node.value() == Some(status.state.label())
        })
        .count();
    let alerts = nodes
        .iter()
        .filter(|(_, node)| node.role() == egui::accesskit::Role::Alert)
        .count();
    assert_eq!(titles, 1, "the card's title is published {titles} times");
    assert_eq!(alerts, 1, "the corrupted state is announced {alerts} times");
}

/// Run again is the card's primary action where running the plan again makes
/// the result whole: out of date, cancelled, or cut short. It answers as Run
/// plan does, enabled or unavailable for that command's reason, and a failed
/// run's card offers what marks its failure instead.
#[test]
fn run_again_answers_as_run_plan_does() {
    use ResultOperationalState::{Failed, InterruptedOperation, Partial, Stale};
    let ctx = egui::Context::default();
    rspice_ui_kit::Theme::default().apply(&ctx);
    ctx.enable_accesskit();
    let run_again = |state: ResultOperationalState, unavailable: Option<&'static str>| {
        let status = ResultOperationalStatus::canonical(state, true);
        let mut nodes = Vec::new();
        for _ in 0..2 {
            let input = egui::RawInput {
                screen_rect: Some(Rect::from_min_size(egui::Pos2::ZERO, WELL)),
                ..Default::default()
            };
            nodes = ctx
                .run_ui(input, |ui| {
                    let _ = show(ui, &status, || CardOffers {
                        run_again: Some(RunAgainControl { unavailable }),
                        ..CardOffers::default()
                    });
                })
                .platform_output
                .accesskit_update
                .expect("AccessKit operational-card tree")
                .nodes;
        }
        nodes
            .into_iter()
            .find(|(_, node)| {
                node.role() == egui::accesskit::Role::Button && node.label() == Some("Run again")
            })
            .map(|(_, node)| node.is_disabled())
    };
    assert_eq!(run_again(Stale, None), Some(false));
    assert_eq!(run_again(InterruptedOperation, None), Some(false));
    assert_eq!(run_again(Partial, None), Some(false));
    assert_eq!(
        run_again(Stale, Some("a run is already in progress")),
        Some(true),
        "Run again is unavailable while Run plan is"
    );
    assert_eq!(run_again(Failed, None), None);
}
