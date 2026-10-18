//! Where the canvas and the SVG export stand an instance's name and value, a
//! net label's name and connector tab, and the bus, tap and probe text (U-4).
//! The printed page reads the same layout, so these positions are the ones a
//! reviewer finds on paper too.

use egui::{Align2, Pos2, Rect, Vec2};
use rspice_design::schematic::{
    component::Component, component_type::ComponentType, net_label::NetLabel, rotation::Rotation,
};
use rspice_design_model::{Point, design_management::CrossSheetPortDirection};

use super::{SvgExportConfig, primitive_labels};
use crate::view::{drawing, net_labels};

const TOLERANCE: f32 = 1e-4;

fn close(actual: Pos2, expected: (f32, f32)) -> bool {
    (actual.x - expected.0).abs() <= TOLERANCE && (actual.y - expected.1).abs() <= TOLERANCE
}

/// One instance at canvas (100, 200) and zoom 2: where its name and value
/// stand and how they hang, for every mirror of the given rotation.
struct Case {
    kind: ComponentType,
    rotation: Rotation,
    name: ((f32, f32), Align2),
    value: ((f32, f32), Align2),
}

const fn case(
    kind: ComponentType,
    rotation: Rotation,
    name: ((f32, f32), Align2),
    value: ((f32, f32), Align2),
) -> Case {
    Case {
        kind,
        rotation,
        name,
        value,
    }
}

#[test]
fn the_canvas_stands_names_and_values_outside_the_body_for_every_turn_and_mirror() {
    use Align2 as A;
    use ComponentType as K;
    use Rotation as R;
    let cases = [
        // Pins on a horizontal axis: name above, value below.
        case(
            K::Resistor,
            R::R0,
            ((100.0, 172.0), A::CENTER_BOTTOM),
            ((100.0, 228.0), A::CENTER_TOP),
        ),
        case(
            K::Resistor,
            R::R180,
            ((100.0, 172.0), A::CENTER_BOTTOM),
            ((100.0, 228.0), A::CENTER_TOP),
        ),
        // Turned a quarter, the pins stand vertical: name left, value right.
        case(
            K::Resistor,
            R::R90,
            ((72.0, 200.0), A::RIGHT_CENTER),
            ((128.0, 200.0), A::LEFT_CENTER),
        ),
        case(
            K::Resistor,
            R::R270,
            ((72.0, 200.0), A::RIGHT_CENTER),
            ((128.0, 200.0), A::LEFT_CENTER),
        ),
        case(
            K::VoltageSource,
            R::R0,
            ((64.0, 200.0), A::RIGHT_CENTER),
            ((136.0, 200.0), A::LEFT_CENTER),
        ),
        case(
            K::VoltageSource,
            R::R90,
            ((100.0, 164.0), A::CENTER_BOTTOM),
            ((100.0, 236.0), A::CENTER_TOP),
        ),
        // A tall device: name above, value to the right, past the turned body.
        // Unturned, the value stands just above the middle, where the bulk
        // lead runs out, on the drain's side, away from the source the bulk
        // is tied toward.
        case(
            K::Nmos,
            R::R0,
            ((100.0, 112.0), A::CENTER_BOTTOM),
            ((148.0, 192.0), A::LEFT_BOTTOM),
        ),
        case(
            K::Nmos,
            R::R180,
            ((100.0, 112.0), A::CENTER_BOTTOM),
            ((148.0, 192.0), A::LEFT_BOTTOM),
        ),
        case(
            K::Nmos,
            R::R90,
            ((100.0, 152.0), A::CENTER_BOTTOM),
            ((188.0, 200.0), A::LEFT_CENTER),
        ),
        case(
            K::NpnBjt,
            R::R270,
            ((100.0, 152.0), A::CENTER_BOTTOM),
            ((188.0, 200.0), A::LEFT_CENTER),
        ),
        case(
            K::CellInstance,
            R::R0,
            ((100.0, 152.0), A::CENTER_BOTTOM),
            ((100.0, 248.0), A::CENTER_TOP),
        ),
        case(
            K::CellInstance,
            R::R90,
            ((52.0, 200.0), A::RIGHT_CENTER),
            ((148.0, 200.0), A::LEFT_CENTER),
        ),
    ];
    for case in cases {
        for (mirror_h, mirror_v) in [(false, false), (true, false), (false, true), (true, true)] {
            let mut component = Component::new(1, case.kind, Point::new(0, 0));
            component.rotation = case.rotation;
            component.mirror_h = mirror_h;
            component.mirror_v = mirror_v;
            let layout = drawing::compute_label_layout(Pos2::new(100.0, 200.0), 2.0, &component);
            let context = format!(
                "{:?} {:?} mirror ({mirror_h}, {mirror_v})",
                case.kind, case.rotation
            );
            // A tall device whose body is turned to face left letters its
            // value on the left, ending at the body: the mirror image of
            // where it stands unturned, away from the gate.
            let faces_left = component.transform_point(Point::new(1, 0)).x < 0;
            let tall = matches!(case.kind, K::Nmos | K::NpnBjt);
            let mut value = case.value;
            if tall && faces_left {
                value = (
                    (200.0 - value.0.0, value.0.1),
                    A([egui::Align::Max, value.1.y()]),
                );
            }
            // Unturned with its source on top (flipped, or turned end for
            // end), its value hangs below the middle instead: the mirror
            // image about the middle.
            let source_above = component.transform_point(Point::new(0, 1)).y < 0;
            if tall && !case.rotation.is_vertical() && source_above {
                value = (
                    (value.0.0, 400.0 - value.0.1),
                    A([value.1.x(), egui::Align::Min]),
                );
            }
            assert!(
                close(layout.name_pos, case.name.0),
                "{context}: name at {:?}",
                layout.name_pos
            );
            assert_eq!(layout.name_align, case.name.1, "{context}: name hang");
            assert!(
                close(layout.value_pos, value.0),
                "{context}: value at {:?}",
                layout.value_pos
            );
            assert_eq!(layout.value_align, value.1, "{context}: value hang");
        }
    }
}

#[test]
fn the_svg_letters_names_and_values_where_the_canvas_layout_puts_them() {
    let config = SvgExportConfig::default();
    let mut upright = Component::new(1, ComponentType::Resistor, Point::new(0, 100));
    upright.name = "R1".to_owned();
    upright.value = "10k".to_owned();
    let mut turned = Component::new(2, ComponentType::Nmos, Point::new(120, 60));
    turned.name = "M1".to_owned();
    turned.value = "nch".to_owned();
    turned.rotation = Rotation::R90;
    turned.mirror_h = true;

    let labels = primitive_labels(&upright, &config);
    let found: Vec<_> = labels
        .iter()
        .map(|label| (label.text.as_str(), label.at, label.align))
        .collect();
    assert_eq!(
        found,
        [
            ("R1", (0.0, 860.0), Align2::CENTER_BOTTOM),
            ("10k", (0.0, 1140.0), Align2::CENTER_TOP),
        ]
    );

    let labels = primitive_labels(&turned, &config);
    let found: Vec<_> = labels
        .iter()
        .map(|label| (label.text.as_str(), label.at, label.align))
        .collect();
    assert_eq!(
        found,
        [
            ("M1", (1200.0, 360.0), Align2::CENTER_BOTTOM),
            ("nch", (1640.0, 600.0), Align2::LEFT_CENTER),
        ]
    );
}

#[test]
fn a_net_label_and_its_connector_tab_stand_where_they_always_have() {
    let local = NetLabel::new(1, Point::new(-40, 0), "VOUT");
    let layout = net_labels::world_layout(&local);
    assert_eq!(layout.name, "VOUT");
    assert_eq!(layout.font_size, 11.0);
    assert!(close(layout.corner, (-33.0, -5.0)));
    assert_eq!(layout.diamond_half, 2.25);
    assert!(layout.tab.is_none());

    let blank = NetLabel::new(2, Point::new(0, 0), "  ");
    assert_eq!(net_labels::world_layout(&blank).name, "<unnamed>");

    let connector = NetLabel::off_sheet(
        3,
        Point::new(-40, 0),
        "VOUT",
        CrossSheetPortDirection::Output,
    );
    let layout = net_labels::world_layout(&connector);
    let (tab, direction) = layout.tab.expect("an off-sheet connector has a tab");
    assert_eq!(direction, CrossSheetPortDirection::Output);
    // After four 6.7-wu glyphs and a 3-wu gap, 12 wu wide and one 1.3-em
    // line of 11-wu type tall, standing on the name's baseline corner.
    assert!(close(tab.min, (-3.2, -19.3)), "tab at {:?}", tab.min);
    assert!(close(tab.max, (8.8, -5.0)), "tab to {:?}", tab.max);

    let marks = net_labels::connector_tab_marks(
        Rect::from_min_size(Pos2::ZERO, Vec2::new(12.0, 13.0)),
        CrossSheetPortDirection::Output,
    );
    // The stem, then the head at the right, where the signal goes.
    assert_eq!(marks.len(), 3);
    assert!(close(marks[0][0], (2.64, 6.5)) && close(marks[0][1], (9.36, 6.5)));
    assert!(close(marks[1][0], (9.36, 6.5)) && close(marks[1][1], (7.44, 3.9)));
    assert!(close(marks[2][0], (9.36, 6.5)) && close(marks[2][1], (7.44, 9.1)));
}

#[test]
fn bus_tap_and_probe_text_hang_off_the_offsets_the_canvas_names() {
    assert_eq!(drawing::BUS_DECLARATION_OFFSET_WU, Vec2::new(8.0, -8.0));
    assert_eq!(drawing::TAP_SLICE_OFFSET_WU, Vec2::new(5.0, -7.0));
    assert_eq!(drawing::PROBE_LABEL_OFFSET_WU, Vec2::new(13.0, -9.0));
    assert_eq!(drawing::SMALL_TEXT_WORLD, 9.0);
    assert_eq!(drawing::INSTANCE_NAME_WORLD, 10.0);
    assert_eq!(drawing::INSTANCE_VALUE_WORLD, 9.0);
}
