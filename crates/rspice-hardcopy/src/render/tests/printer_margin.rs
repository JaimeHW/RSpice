//! A printer page is refused only for ink in the printer's unprintable
//! margin, and the refusal names the worst side in millimetres.

use super::*;

#[test]
fn the_refusal_names_the_worst_side_in_millimetres() {
    // A 1000 × 800 px page at 300 dpi whose driver cannot reach 50 px at the
    // left and top or 60 px at the right and bottom.
    let page = (1_000, 800);
    let printable = (50, 50, 890, 690);
    let inside = InkBounds {
        left: 50,
        top: 50,
        right: 940,
        bottom: 740,
    };
    assert_eq!(
        printer_margin_shortfall(Some(inside), page, printable, 300),
        None
    );
    assert_eq!(printer_margin_shortfall(None, page, printable, 300), None);

    // 10 px into the left margin and 30 px into the bottom one: the bottom is
    // the side the refusal names.
    let reaching = InkBounds {
        bottom: 770,
        left: 40,
        ..inside
    };
    let shortfall = printer_margin_shortfall(Some(reaching), page, printable, 300).unwrap();
    assert_eq!((shortfall.margin_um(), shortfall.ink_um()), (5_080, 2_540));
    assert_eq!(
        shortfall.to_string(),
        "The printer can't print within 5.1 mm of the paper's edge, but this page has ink \
         2.5 mm from it. Print it scaled to fit, or choose a larger paper."
    );
}
