use super::*;

/// A held node's samples can differ in their last bit, and a fit to that span
/// fills the pane with rounding. A real ripple on the same level keeps its
/// span, and so does a small signal around zero.
#[test]
fn a_fit_reads_rounding_on_a_held_node_as_one_level() {
    let level = 0.999_995_000_024_839_f64;
    let next = f64::from_bits(level.to_bits() + 1);
    assert_eq!(
        pane_auto_y(Some((level, next)), None, &[]),
        Some((level / 1.1, next * 1.1))
    );
    for span in [(level, level + 1.0e-6), (-1.0e-15, 1.0e-15)] {
        assert_eq!(pane_auto_y(Some(span), None, &[]), Some(span));
    }
    assert_eq!(pane_auto_y(Some((0.0, 0.0)), None, &[]), Some((-1.0, 1.0)));
    assert_eq!(
        pane_auto_y(Some((-5.0, -5.0)), None, &[]),
        Some((-6.0, -4.0))
    );
}
