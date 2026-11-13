//! AC table axes retain zero, repeated rows and row-local parameters; noise rejects DC.
use rspice_core::{Engine, Netlist};

const DECK: &str = "AC table\nV1 in 0 AC 1\n.param load=1k\nR1 in out 1k\nR2 out 0 {load}\n.data pts FREQ load\n1000 1000\n0 2000\n1000 500\n.enddata\n.end\n";

#[test]
fn ac_data_keeps_zero_and_repeated_frequency_rows_with_their_parameters() {
    let netlist = Netlist::parse(DECK).unwrap();
    let (rows, points) = Engine::default().run_ac_data(&netlist, "pts").unwrap();
    assert_eq!(rows.len(), 3);
    for (point, (frequency, voltage)) in
        points
            .iter()
            .zip([(1000.0, 0.5), (0.0, 2.0 / 3.0), (1000.0, 1.0 / 3.0)])
    {
        assert_eq!(point.frequency, frequency);
        let i = point
            .node_names
            .iter()
            .position(|name| name.eq_ignore_ascii_case("out"))
            .unwrap();
        assert!((point.voltages[i].re - voltage).abs() < 1e-10);
        assert!(point.voltages[i].im.abs() < 1e-12);
    }
}

#[test]
fn noise_rejects_zero_in_any_row_before_running_the_table() {
    let netlist = Netlist::parse(DECK).unwrap();
    let error = Engine::default()
        .run_noise_data_named_with_input_source(&netlist, "out", None, "V1", "pts", 300.15)
        .unwrap_err();
    assert!(error.to_string().contains("strictly positive"), "{error}");
}
