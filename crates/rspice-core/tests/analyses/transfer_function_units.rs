//! Transfer units follow the actual elaborated input and output quantities.
use rspice_core::execution::{
    AnalysisInstanceId, AnalysisKind, AnalysisResultDocument, SignalUnit,
};
use rspice_core::{Engine, Netlist};

#[cfg_attr(target_arch = "wasm32", wasm_bindgen_test::wasm_bindgen_test)]
#[cfg_attr(not(target_arch = "wasm32"), test)]
fn all_four_transfer_quantities_keep_physical_gain_units() {
    for (body, input, output, current, gain, unit) in [
        (
            "V1 in 0 1\nR1 in out 1k\nR2 out 0 2k\n",
            "V1",
            "out",
            false,
            2.0 / 3.0,
            SignalUnit::Dimensionless,
        ),
        (
            "V1 in 0 1\nR1 in mid 1k\nVm mid out 0\nR2 out 0 2k\n",
            "V1",
            "Vm",
            true,
            1.0 / 3000.0,
            SignalUnit::Siemens,
        ),
        (
            "I1 0 out 0\nR1 out 0 3k\n",
            "I1",
            "out",
            false,
            3000.0,
            SignalUnit::Ohm,
        ),
        (
            "I1 0 mid 0\nVm mid out 0\nR1 out 0 3k\n",
            "I1",
            "Vm",
            true,
            1.0,
            SignalUnit::Dimensionless,
        ),
    ] {
        let netlist = Netlist::parse(&format!("TF units\n{body}.end\n")).unwrap();
        let result = Engine::default()
            .run_transfer_function(&netlist, output, None, current, input)
            .unwrap();
        assert!((result.gain - gain).abs() < gain.abs() * 1e-10 + 1e-15);
        let document = AnalysisResultDocument::from_transfer_function(
            AnalysisInstanceId::new(AnalysisKind::TransferFunction, 0),
            &result,
        )
        .unwrap()
        .build()
        .unwrap();
        let scalar = document
            .scalars()
            .iter()
            .find(|value| value.name() == "transfer_gain")
            .unwrap();
        assert_eq!(scalar.unit(), Some(&unit), "{input} -> {output}");
        assert_eq!(
            AnalysisResultDocument::from_json(&document.to_json().unwrap()).unwrap(),
            document
        );
    }
}

#[cfg_attr(target_arch = "wasm32", wasm_bindgen_test::wasm_bindgen_test)]
#[cfg_attr(not(target_arch = "wasm32"), test)]
fn hierarchical_current_input_keeps_transimpedance_and_probe_identity() {
    let netlist = Netlist::parse(
        "TF hierarchy\nX1 out drive\nR1 out 0 3k\n.subckt drive o\nI1 0 o 0\n.ends\n.end\n",
    )
    .unwrap();
    let result = Engine::default()
        .run_transfer_function(&netlist, "out", Some("0"), false, "x1.i1")
        .unwrap();
    assert!((result.gain - 3000.0).abs() < 1e-6);
    let document = AnalysisResultDocument::from_transfer_function(
        AnalysisInstanceId::new(AnalysisKind::TransferFunction, 0),
        &result,
    )
    .unwrap()
    .build()
    .unwrap();
    assert_eq!(document.scalars()[0].unit(), Some(&SignalUnit::Ohm));
    let wire = serde_json::to_value(&document).unwrap();
    assert_eq!(wire["payload"]["input"], "x1.i1");
    assert_eq!(wire["payload"]["output"], "V(out,0)");
}
