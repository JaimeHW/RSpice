//! Staged analysis parsing preserves public card grammar and authored order.
use rspice_core::Netlist;
use rspice_core::netlist::expr::{ParamContext, eval_expression};
use rspice_core::netlist::{AnalysisCommand, FftWindow, OutputDirectiveKind};

#[cfg_attr(target_arch = "wasm32", wasm_bindgen_test::wasm_bindgen_test)]
#[cfg_attr(not(target_arch = "wasm32"), test)]
fn invalid_optional_values_are_not_silently_omitted() {
    for card in [
        ".TRAN 1n 1u {missing}",
        ".TRAN 1n 1u 0 {missing}",
        ".TEMP 25 {missing}",
        ".STEP PARAM gain LIST 1 {missing}",
        ".NOISE V(out) V1 LIN 2 1 10 {missing}",
        ".MC 2 uniform {missing}",
    ] {
        let error = Netlist::parse(&format!("Invalid optional operand\n{card}\n.end\n"))
            .expect_err(card);
        assert!(
            error.to_string().to_ascii_uppercase().contains("MISSING"),
            "{card}: {error}"
        );
    }
}

#[cfg_attr(target_arch = "wasm32", wasm_bindgen_test::wasm_bindgen_test)]
#[cfg_attr(not(target_arch = "wasm32"), test)]
fn ac_data_rejects_unconsumed_fields() {
    for tail in ["unexpected", "1", "{missing}", "DATA=other"] {
        let error = Netlist::parse(&format!(
            "AC table\n.data grid freq\n1\n.enddata\n.AC DATA=grid {tail}\n.end\n"
        ))
        .unwrap_err();
        assert!(error.to_string().contains(".AC"), "{error}");
    }
    let netlist =
        Netlist::parse("AC table\n.data grid freq\n1\n.enddata\n.AC DATA=grid\n.end\n").unwrap();
    assert!(matches!(
        netlist.analyses.as_slice(),
        [AnalysisCommand::AcData { .. }]
    ));
}

#[cfg_attr(target_arch = "wasm32", wasm_bindgen_test::wasm_bindgen_test)]
#[cfg_attr(not(target_arch = "wasm32"), test)]
fn mixed_analysis_and_output_cards_keep_their_authored_order() {
    let netlist = Netlist::parse("Card order\nV1 out 0 1 AC 1\nR1 out 0 1k\n.OP\n.DC V1 0 1 .1\n.AC LIN 2 1 10\n.TR 1n 1u\n.NOISE V(out) V1 LIN 2 1 10\n.FOURIER 1k V(out)\n.FFT V(out) NP=32 WINDOW=HANN\n.LIN SPARCALC=0\n.MC 2 uniform .1 seed 37\n.end\n").unwrap();
    assert!(matches!(
        netlist.analyses.as_slice(),
        [
            AnalysisCommand::Op,
            AnalysisCommand::Dc { .. },
            AnalysisCommand::Ac { .. },
            AnalysisCommand::Tran { .. },
            AnalysisCommand::Noise { .. },
            AnalysisCommand::Four { .. },
            AnalysisCommand::MonteCarlo(_)
        ]
    ));
    assert_eq!(netlist.fft_analyses.len(), 1);
    assert_eq!(netlist.fft_analyses[0].window, FftWindow::Hann);
    assert_eq!(netlist.output_requests.len(), 2);
    assert_eq!(
        netlist.output_requests[0].directive,
        OutputDirectiveKind::Four
    );
    assert_eq!(
        netlist.output_requests[1].directive,
        OutputDirectiveKind::Fft
    );
}

#[cfg_attr(target_arch = "wasm32", wasm_bindgen_test::wasm_bindgen_test)]
#[cfg_attr(not(target_arch = "wasm32"), test)]
fn sampled_analysis_fields_draw_once_in_card_order() {
    let netlist = Netlist::parse("Sampled cards\n.options seed=37\n.func sample(x) {x+aunif(0,1)}\n.DC V1 {sample(0)} 2 1\n.AC LIN 2 {sample(2)} 10\n.FFT V(out) START={sample(2)} NP=32\n.end\n").unwrap();
    let mut expected = ParamContext::new();
    expected.set_random_seed(37);
    let draw = || eval_expression("aunif(0,1)", &expected).unwrap();
    let AnalysisCommand::Dc { start, .. } = netlist.analyses[0] else {
        panic!("DC")
    };
    assert_eq!(start, draw());
    let AnalysisCommand::Ac { start_freq, .. } = netlist.analyses[1] else {
        panic!("AC")
    };
    assert_eq!(start_freq, 2.0 + draw());
    assert_eq!(netlist.fft_analyses[0].start, Some(2.0 + draw()));
    assert_eq!(
        eval_expression("aunif(0,1)", &netlist.params).unwrap(),
        draw()
    );
}
