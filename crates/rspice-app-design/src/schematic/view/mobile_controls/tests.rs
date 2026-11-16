//! What the touch strip asks the dispatcher about. The one-finger gestures,
//! driven through real frames by touch events the way a tablet reports them,
//! hold a whole application and are with the cases that drive one.

use super::*;

/// Every button's command the strip asks the dispatcher about is one the
/// design feature declares it reads, in the strip's order and ahead of Show
/// in netlist, which the navigator and the inspector read;
/// Cancel's is offered while an operation is live, and the two latches run
/// no command.
#[test]
fn the_strip_asks_only_the_answers_the_design_feature_reads() {
    let asked = [
        MobileCanvasAction::Rotate,
        MobileCanvasAction::Mirror,
        MobileCanvasAction::Undo,
        MobileCanvasAction::Redo,
        MobileCanvasAction::Fit,
    ]
    .map(|action| product_command(action).expect("a command button"));
    let foreign =
        <DesignFeature as crate::workbench::framework::feature::Feature>::FOREIGN_AVAILABILITY;
    assert_eq!(&foreign[..asked.len()], &asked);
    assert_eq!(
        &foreign[asked.len()..],
        &[crate::workbench::framework::command_registry::ids::SHOW_IN_NETLIST]
    );
    assert!(!foreign.contains(&ids::CANCEL));
}
