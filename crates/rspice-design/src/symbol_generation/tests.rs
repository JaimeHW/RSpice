use super::*;

fn spec(name: &str, direction: PortDirection) -> PortSpec {
    PortSpec {
        name: name.to_string(),
        direction,
    }
}

/// The 5T OTA interface — the canonical shape this generator serves.
fn ota_ports() -> Vec<PortSpec> {
    vec![
        spec("inp", PortDirection::In),
        spec("inn", PortDirection::In),
        spec("out", PortDirection::Out),
        spec("vdd", PortDirection::Supply),
        spec("vss", PortDirection::Supply),
    ]
}

fn side_of(symbol: &GeneratedSymbol, name: &str) -> SymbolPinSide {
    symbol
        .pins
        .iter()
        .find(|pin| pin.name == name)
        .map(|pin| pin.side)
        .unwrap_or_else(|| panic!("no pin {name}"))
}

fn offset_of(symbol: &GeneratedSymbol, name: &str) -> Point {
    symbol
        .pins
        .iter()
        .find(|pin| pin.name == name)
        .map(|pin| pin.offset)
        .unwrap_or_else(|| panic!("no pin {name}"))
}

#[test]
fn pins_stay_in_interface_order() {
    let symbol = generate_symbol(&ota_ports());
    let names: Vec<&str> = symbol.pins.iter().map(|p| p.name.as_str()).collect();
    assert_eq!(names, ["inp", "inn", "out", "vdd", "vss"]);
}

/// Catalog artwork matches its terminals against this block, so its
/// geometry is the one the artwork was drawn for.
#[test]
fn the_artwork_block_places_pins_by_declared_direction() {
    let symbol = generate_artwork_block(&ota_ports());
    let hw = symbol.width / 2;
    let (min, max) = symbol.body_bounds();
    assert_eq!(symbol.width, GENERATED_WIDTH);
    assert_eq!(symbol.pins[0].offset, Point::new(-hw, -10)); // inp
    assert_eq!(symbol.pins[1].offset, Point::new(-hw, 10)); // inn
    assert_eq!(symbol.pins[2].offset, Point::new(hw, 0)); // out
    assert_eq!(symbol.pins[3].offset, Point::new(0, min.y)); // vdd up
    assert_eq!(symbol.pins[4].offset, Point::new(0, max.y)); // vss down
    for ports in [
        vec![
            spec("in", PortDirection::In),
            spec("out", PortDirection::Out),
        ],
        vec![
            spec("cntl_in", PortDirection::In),
            spec("out", PortDirection::Out),
        ],
    ] {
        assert_eq!(
            generate_artwork_block(&ports).width,
            GENERATED_WIDTH,
            "{ports:?} must not widen the block artwork is drawn against"
        );
    }
}

#[test]
fn every_pin_lands_on_the_grid() {
    let mut ports = ota_ports();
    for i in 0..5 {
        ports.push(spec(&format!("d{i}"), PortDirection::In));
        ports.push(spec(&format!("q{i}"), PortDirection::Out));
    }
    ports.push(spec("vddio", PortDirection::Supply));
    ports.push(spec("clk", PortDirection::In));
    ports.push(spec("tj", PortDirection::InOut));
    let pins = ports
        .iter()
        .map(|port| crate::pin_roles::interface_pin(&port.name, port.direction, port.name == "tj"))
        .collect::<Vec<_>>();
    let symbol = generate_interface_symbol(&ports, &pins);
    for pin in &symbol.pins {
        assert_eq!(pin.offset.x % 10, 0, "{pin:?}");
        assert_eq!(pin.offset.y % 10, 0, "{pin:?}");
    }
}

#[test]
fn body_grows_with_the_busier_edge() {
    let mut ports = vec![spec("out", PortDirection::Out)];
    for i in 0..6 {
        ports.push(spec(&format!("in{i}"), PortDirection::In));
    }
    let symbol = generate_symbol(&ports);
    assert_eq!(symbol.height, 120); // six rows of PIN_PITCH
    let ys: std::collections::HashSet<i32> = symbol
        .pins
        .iter()
        .filter(|p| p.direction == PortDirection::In)
        .map(|p| p.offset.y)
        .collect();
    assert_eq!(ys.len(), 6);
    assert!(ys.iter().all(|y| y.abs() <= symbol.height / 2 - 10));
}

#[test]
fn extra_rails_fan_out_without_collisions() {
    let ports = vec![
        spec("vdd", PortDirection::Supply),
        spec("vss", PortDirection::Supply),
        spec("vdda", PortDirection::Supply),
        spec("vssa", PortDirection::Supply),
    ];
    let symbol = generate_symbol(&ports);
    let unique: std::collections::HashSet<(i32, i32)> = symbol
        .pins
        .iter()
        .map(|p| (p.offset.x, p.offset.y))
        .collect();
    assert_eq!(unique.len(), 4, "{:?}", symbol.pins);
    assert_eq!(side_of(&symbol, "vdda"), SymbolPinSide::Top);
    assert_eq!(side_of(&symbol, "vssa"), SymbolPinSide::Bottom);
}

#[test]
fn every_rail_lands_on_the_body_edge_it_is_drawn_against() {
    let ports: Vec<PortSpec> = (0..6)
        .map(|index| spec(&format!("v{index}"), PortDirection::Supply))
        .collect();
    let symbol = generate_symbol(&ports);
    let half_width = symbol.body_half_width();
    for pin in &symbol.pins {
        assert!(
            pin.offset.x.abs() < half_width,
            "rail {pin:?} hangs off a body of half-width {half_width}"
        );
    }
}

#[test]
fn minimum_body_for_tiny_interfaces() {
    let symbol = generate_symbol(&[spec("io", PortDirection::InOut)]);
    assert_eq!(
        symbol.dimensions(),
        (MIN_BODY_WIDTH + 2 * GENERATED_STUB, 40)
    );
    assert_eq!(
        symbol.pins[0].offset,
        Point::new(MIN_BODY_WIDTH / 2 + GENERATED_STUB, 0)
    );
}

#[test]
fn long_pin_names_are_printed_inside_the_body_without_meeting() {
    let ports = vec![
        spec("data_in[0]", PortDirection::In),
        spec("write_en", PortDirection::In),
        spec("data_out[0]", PortDirection::Out),
    ];
    let symbol = generate_symbol(&ports);
    let half_width = symbol.body_half_width();
    let body = Some(symbol.body_bounds());

    let left = &symbol.pins[0];
    let right = &symbol.pins[2];
    let left_end =
        pin_label_anchor(left.offset, left.side, body).x + name_span(&left.name, PRINTED_NAMES);
    let right_end =
        pin_label_anchor(right.offset, right.side, body).x - name_span(&right.name, PRINTED_NAMES);

    assert!(
        left_end <= half_width - GENERATED_LABEL_GUTTER,
        "left names must stay inside the body: {left_end} vs {half_width}"
    );
    assert!(
        right_end >= -half_width + GENERATED_LABEL_GUTTER,
        "right names must stay inside the body: {right_end} vs {half_width}"
    );
    assert!(
        left_end < right_end,
        "the two name columns must not meet: {left_end} vs {right_end}"
    );
}

#[test]
fn every_lead_reaches_the_body_from_its_own_edge() {
    let ports = vec![
        spec("data_in[0]", PortDirection::In),
        spec("data_in[1]", PortDirection::In),
        spec("data_in[2]", PortDirection::In),
        spec("data_in[3]", PortDirection::In),
        spec("write_en", PortDirection::In),
        spec("data_out[0]", PortDirection::Out),
        spec("vdd", PortDirection::Supply),
        spec("gnd", PortDirection::InOut),
    ];
    let symbol = generate_symbol(&ports);
    let (min, max) = symbol.body_bounds();

    for pin in &symbol.pins {
        let inner = lead_inner(pin.offset, pin.side, Some((min, max)));
        let on_outline = match pin.side {
            SymbolPinSide::Left => inner.x == min.x,
            SymbolPinSide::Right => inner.x == max.x,
            SymbolPinSide::Top => inner.y == min.y,
            SymbolPinSide::Bottom => inner.y == max.y,
        };
        assert!(
            on_outline,
            "{pin:?} lead ends at {inner:?}, not on the body {min:?}..{max:?}"
        );
        assert_eq!(
            (inner.x - pin.offset.x).abs() + (inner.y - pin.offset.y).abs(),
            GENERATED_STUB,
            "{pin:?} stands one grid off its edge"
        );
    }
}

/// An edge that carries rails stands one grid farther out, so a rail's name
/// never shares a line with the outermost side pin's name.
#[test]
fn rails_stand_a_grid_beyond_the_side_pins() {
    let ports = vec![
        spec("in", PortDirection::In),
        spec("clk", PortDirection::In),
        spec("en", PortDirection::In),
        spec("out", PortDirection::Out),
        spec("vdd", PortDirection::InOut),
        spec("vss", PortDirection::InOut),
    ];
    let symbol = generate_symbol(&ports);
    let (min, max) = symbol.body_bounds();
    assert_eq!((min, max), (Point::new(-30, -40), Point::new(30, 40)));
    assert_eq!(offset_of(&symbol, "in"), Point::new(-40, -20));
    assert_eq!(offset_of(&symbol, "vdd"), Point::new(0, -50));
    assert_eq!(offset_of(&symbol, "vss"), Point::new(0, 50));
    assert_eq!(symbol.height, 100);
    // The clock input carries its wedge, and its name starts past it.
    let clk = symbol
        .pins
        .iter()
        .position(|pin| pin.name == "clk")
        .unwrap();
    assert_eq!(symbol.marks[clk], PinMark::Clock);
    assert_eq!(symbol.labels[clk], PinLabel::InsideFrom(18));
    assert!(symbol.body_outline().contains(&(
        vec![Point::new(-30, 3), Point::new(-26, 0), Point::new(-30, -3)],
        false
    )));
    // The labels clear the rail terminals.
    assert_eq!(symbol.name_anchor(), Point::new(-30, -58));
    assert_eq!(symbol.value_anchor(), Point::new(-30, 58));
}

#[test]
fn a_name_too_long_for_the_nominal_body_widens_the_block_onto_the_grid() {
    let symbol = generate_symbol(&[
        spec("freq_in", PortDirection::In),
        spec("freq_out", PortDirection::Out),
    ]);

    assert!(symbol.width > MIN_BODY_WIDTH + 2 * GENERATED_STUB);
    assert_eq!(symbol.width % (2 * GENERATED_STUB), 0);
    assert_eq!(symbol.pins[0].offset, Point::new(-symbol.width / 2, 0));
    assert_eq!(symbol.pins[1].offset, Point::new(symbol.width / 2, 0));
}

#[test]
fn a_pathological_name_stops_the_block_growing_and_is_elided_to_fit() {
    let long = "d".repeat(GENERATED_PIN_LABEL_MAX_CHARS * 8);
    let capped = generate_symbol(&[spec(&long, PortDirection::In)]);
    let at_limit = generate_symbol(&[spec(
        &"d".repeat(GENERATED_PIN_LABEL_MAX_CHARS),
        PortDirection::In,
    )]);

    assert_eq!(capped.width, at_limit.width);
    assert_eq!(
        fit_pin_name(&long).chars().count(),
        GENERATED_PIN_LABEL_MAX_CHARS
    );
    assert!(fit_pin_name(&long).ends_with('…'));
    assert_eq!(fit_pin_name("data_in[0]"), "data_in[0]");
}

fn role_ports(names: &[&str]) -> Vec<PortSpec> {
    names
        .iter()
        .map(|name| spec(name, PortDirection::InOut))
        .collect()
}

#[test]
fn an_amplifier_pin_set_draws_the_triangle_with_plus_on_top() {
    // ADI order: noninverting, inverting, positive, negative, output.
    let ports = role_ports(&["1", "2", "99", "50", "45"]);
    let roles = [
        PinRole::NonInvertingInput,
        PinRole::InvertingInput,
        PinRole::PositiveSupply,
        PinRole::NegativeSupply,
        PinRole::Output,
    ];
    let symbol = generate_symbol_with_roles(&ports, &roles);
    assert_eq!(symbol.body, GeneratedBody::Amplifier);
    let placed = symbol
        .pins
        .iter()
        .map(|pin| (pin.name.as_str(), pin.side, pin.offset))
        .collect::<Vec<_>>();
    assert_eq!(
        placed,
        [
            ("1", SymbolPinSide::Left, Point::new(-40, -20)),
            ("2", SymbolPinSide::Left, Point::new(-40, 20)),
            ("99", SymbolPinSide::Top, Point::new(0, -40)),
            ("50", SymbolPinSide::Bottom, Point::new(0, 40)),
            ("45", SymbolPinSide::Right, Point::new(40, 0)),
        ]
    );
    // Every terminal is on the routing grid and every lead lands on the
    // body: inputs on the left edge, the output on the apex.
    let (min, max) = symbol.body_bounds();
    for pin in &symbol.pins {
        assert_eq!((pin.offset.x % 10, pin.offset.y % 10), (0, 0));
        let inner = lead_inner(pin.offset, pin.side, Some((min, max)));
        match pin.side {
            SymbolPinSide::Left => assert_eq!(inner.x, min.x),
            SymbolPinSide::Right => assert_eq!(inner, Point::new(max.x, 0)),
            SymbolPinSide::Top | SymbolPinSide::Bottom => assert_eq!(inner, pin.offset),
        }
    }
    let outline = symbol.body_outline();
    assert_eq!(
        outline[0],
        (
            vec![Point::new(-30, -40), Point::new(30, 0), Point::new(-30, 40)],
            true
        )
    );
    // The rail stubs reach the sloped edges at x = 0.
    assert!(outline.contains(&(vec![Point::new(0, -40), Point::new(0, -20)], false)));
    assert!(outline.contains(&(vec![Point::new(0, 40), Point::new(0, 20)], false)));
    // The `+` mark has its upright beside the upper input; the `-` has none.
    assert!(outline.contains(&(vec![Point::new(-22, -24), Point::new(-22, -16)], false)));
    assert!(outline.contains(&(vec![Point::new(-26, 20), Point::new(-18, 20)], false)));
    // The marks and the apex name the signal pins; rail names sit beside
    // their leads, outside the body.
    assert_eq!(
        symbol.labels,
        [
            PinLabel::Hidden,
            PinLabel::Hidden,
            PinLabel::Beside {
                offset: Point::new(4, 10),
                reach: Point::new(1, 0)
            },
            PinLabel::Beside {
                offset: Point::new(4, -10),
                reach: Point::new(1, 0)
            },
            PinLabel::Hidden,
        ]
    );
}

#[test]
fn three_and_four_pin_amplifiers_draw_the_triangle_too() {
    let opamp = generate_symbol(&[
        spec("inp", PortDirection::In),
        spec("inn", PortDirection::In),
        spec("out", PortDirection::Out),
    ]);
    assert_eq!(opamp.body, GeneratedBody::Amplifier);
    assert_eq!(opamp.pins.len(), 3);
    // Without rails the triangle draws no rail leads.
    assert!(
        !opamp
            .body_outline()
            .contains(&(vec![Point::new(0, -40), Point::new(0, -20)], false))
    );
    let single_supply = generate_symbol(&[
        spec("inp", PortDirection::In),
        spec("inn", PortDirection::In),
        spec("out", PortDirection::Out),
        spec("gnd", PortDirection::InOut),
    ]);
    assert_eq!(single_supply.body, GeneratedBody::Amplifier);
    assert_eq!(offset_of(&single_supply, "gnd"), Point::new(0, 40));
    // A sixth pin is not an amplifier's.
    let enabled = generate_symbol(&[
        spec("inp", PortDirection::In),
        spec("inn", PortDirection::In),
        spec("out", PortDirection::Out),
        spec("vdd", PortDirection::InOut),
        spec("vss", PortDirection::InOut),
        spec("en", PortDirection::In),
    ]);
    assert_eq!(enabled.body, GeneratedBody::Block);
}

/// A Verilog-A module declares its rails `inout`; their names put them on
/// top and bottom, and the declared inputs and outputs keep their sides.
#[test]
fn roles_read_from_names_put_supplies_top_and_bottom() {
    let ports = vec![
        spec("in", PortDirection::In),
        spec("en", PortDirection::In),
        spec("out", PortDirection::Out),
        spec("vref", PortDirection::Out),
        spec("vdd", PortDirection::InOut),
        spec("io", PortDirection::InOut),
        spec("vss", PortDirection::InOut),
    ];
    let symbol = generate_symbol(&ports);
    assert_eq!(symbol.body, GeneratedBody::Block);
    assert_eq!(side_of(&symbol, "in"), SymbolPinSide::Left);
    assert_eq!(side_of(&symbol, "en"), SymbolPinSide::Left);
    assert_eq!(side_of(&symbol, "vref"), SymbolPinSide::Right);
    assert_eq!(side_of(&symbol, "vdd"), SymbolPinSide::Top);
    assert_eq!(side_of(&symbol, "vss"), SymbolPinSide::Bottom);
    // Bidirectional pins follow the outputs on the right.
    assert!(offset_of(&symbol, "io").y > offset_of(&symbol, "vref").y);
    let vdd = symbol.pins.iter().find(|pin| pin.name == "vdd").unwrap();
    assert_eq!(vdd.direction, PortDirection::Supply);
}

#[test]
fn a_thermal_node_sits_right_of_the_bottom_rail() {
    let ports = vec![
        spec("in", PortDirection::In),
        spec("out", PortDirection::Out),
        spec("gnd", PortDirection::InOut),
        spec("t", PortDirection::InOut),
    ];
    let pins = [
        InterfacePin::new(PinRole::Input),
        InterfacePin::new(PinRole::Output),
        InterfacePin::new(PinRole::Ground),
        InterfacePin {
            role: PinRole::Bidirectional,
            mark: PinMark::Thermal,
        },
    ];
    let symbol = generate_interface_symbol(&ports, &pins);
    assert_eq!(offset_of(&symbol, "gnd").x, 0);
    assert_eq!(offset_of(&symbol, "t").x, 20);
    assert_eq!(side_of(&symbol, "t"), SymbolPinSide::Bottom);
    assert_eq!(symbol.marks[3], PinMark::Thermal);
}

#[test]
fn other_role_sets_draw_the_block_with_the_positive_rail_on_top() {
    let ports = role_ports(&["VEE", "IN", "VCC", "OUT"]);
    let roles = [
        PinRole::NegativeSupply,
        PinRole::Input,
        PinRole::PositiveSupply,
        PinRole::Output,
    ];
    let symbol = generate_symbol_with_roles(&ports, &roles);
    assert_eq!(symbol.body, GeneratedBody::Block);
    assert_eq!(side_of(&symbol, "VCC"), SymbolPinSide::Top);
    assert_eq!(side_of(&symbol, "VEE"), SymbolPinSide::Bottom);
    assert_eq!(side_of(&symbol, "IN"), SymbolPinSide::Left);
    assert_eq!(side_of(&symbol, "OUT"), SymbolPinSide::Right);
    // Roles that do not cover the interface are read from the names.
    assert_eq!(
        generate_symbol_with_roles(&ports, &roles[..2]),
        generate_symbol(&ports)
    );
}
