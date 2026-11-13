use rspice_core::{
    Engine, Netlist,
    netlist::{ElementKind, flatten_netlist_with_models},
};

#[test]
fn quoted_numeric_vectors_keep_the_deck_context_and_sample_sequence() {
    for field in [
        "real_array=[{sample()} {scale}]",
        "complex_array=[<sample() {scale}>]",
    ] {
        let (name, value) = field.split_once('=').unwrap();
        let decks = [field.to_string(), format!("{name}=\"{value}\"")].map(|assignment| {
            Netlist::parse(&format!(
                "* vector context\n.OPTIONS SEED=37\n.PARAM scale=2\n.FUNC sample() {{aunif(100,1)+scale}}\n\
                 A1 [in] print_param_types {assignment}\n.PARAM marker={{aunif(100,1)}}\n.END\n"
            )).unwrap()
        });
        assert_eq!(
            decks[0].params.get("marker"),
            decks[1].params.get("marker"),
            "{field}"
        );
        assert_eq!(
            format!("{:?}", decks[0].elements[0].kind),
            format!("{:?}", decks[1].elements[0].kind),
            "{field}"
        );
        for deck in &decks {
            Engine::default().build_circuit(deck).unwrap();
        }
    }
}

#[test]
fn quoted_vectors_resolve_and_sample_independently_in_each_instance() {
    let mut flattened = Vec::new();
    for quoted in [false, true] {
        let fields = [
            "real_array=[{aunif(100,1)} {value}]",
            "complex_array=[<{aunif(100,1)} {value}>]",
        ]
        .map(|field| {
            let (name, value) = field.split_once('=').unwrap();
            if quoted {
                format!("{name}=\"{value}\"")
            } else {
                field.to_string()
            }
        })
        .join(" ");
        let deck=Netlist::parse(&format!(
            "* scoped vector context\n.OPTIONS SEED=37\n.SUBCKT cell in PARAMS: value=0\n\
             A1 [in] print_param_types {fields}\n.ENDS\nX1 in cell value=4\nX2 in cell value=8\n.END\n"
        )).unwrap();
        let flat = flatten_netlist_with_models(&deck).unwrap();
        let mut numeric = Vec::new();
        for element in &flat.elements {
            let ElementKind::Xspice {
                real_vector_params,
                string_vector_params,
                ..
            } = &element.kind
            else {
                panic!("expected XSPICE");
            };
            numeric.push((real_vector_params.clone(), string_vector_params.clone()));
        }
        assert_eq!(numeric[0].0[0].1[1], 4.0);
        assert_eq!(numeric[1].0[0].1[1], 8.0);
        assert_ne!(numeric[0].0[0].1[0], numeric[1].0[0].1[0]);
        flattened.push(numeric);
        Engine::default().build_circuit(&deck).unwrap();
    }
    assert_eq!(flattened[0], flattened[1]);
}

#[test]
fn quoted_string_vectors_keep_literal_unbound_words_inside_subcircuits() {
    let deck=Netlist::parse("* string literal\n.SUBCKT cell in\nA1 [in] print_param_types string_array=\"[alpha beta]\"\n.ENDS\nX1 in cell\n.END\n").unwrap();
    Engine::default().build_circuit(&deck).unwrap();
}

#[test]
fn quoted_numeric_vectors_use_the_selected_expression_dialect() {
    use rspice_core::{config::ExpressionDialect, netlist::NetlistParseOptions};
    for (dialect, expected) in [
        (ExpressionDialect::Ngspice, 100.0_f64.ln()),
        (ExpressionDialect::Xyce, 2.0),
    ] {
        let options = NetlistParseOptions {
            expression_dialect: dialect,
            ..Default::default()
        };
        let deck = Netlist::parse_with_options(
            "* dialect\nA1 [in] print_param_types real_array=\"[{log(100)}]\"\n.END\n",
            options,
        )
        .unwrap();
        let ElementKind::Xspice {
            real_vector_params, ..
        } = &deck.elements[0].kind
        else {
            panic!("expected XSPICE");
        };
        assert_eq!(real_vector_params[0].1, [expected], "{dialect:?}");
    }
}
