use rspice_core::{Netlist, Value, engine::Engine};

#[test]
fn test_transient_ltra_model_preserves_initial_dc_level_before_first_edge() {
    let netlist_str = r#"
* Lossy LTRA startup should not self-launch before one delay.
V1 src 0 PULSE(5 0 15.9n 0.2n 0.2n 15.8n 32n)
Rsrc src n1 50
O1 n1 0 n2 0 LLINE
C1 n1 0 25.398e-15
C2 n2 0 7.398e-15
.MODEL LLINE LTRA R=12.45 G=0 L=8.972e-9 C=0.468e-12 LEN=16 STEPLIMIT COMPACTREL=1e-3 COMPACTABS=1e-14
.end
"#;
    let netlist = Netlist::parse(netlist_str).unwrap();
    let engine = Engine::default();
    let result = engine
        .run_tran(&netlist, 2e-9, 2e-12)
        .expect("lossy LTRA startup should converge");

    let n1_idx = result
        .node_names
        .iter()
        .position(|name| name.eq_ignore_ascii_case("n1"))
        .expect("n1 should exist");
    let n2_idx = result
        .node_names
        .iter()
        .position(|name| name.eq_ignore_ascii_case("n2"))
        .expect("n2 should exist");

    let mut n1_min = Value::INFINITY;
    let mut n2_min = Value::INFINITY;
    for (i, &time) in result.time.iter().enumerate() {
        if time <= 2e-9 {
            n1_min = n1_min.min(result.voltages[n1_idx][i]);
            n2_min = n2_min.min(result.voltages[n2_idx][i]);
        }
    }

    assert!(
        n1_min > 4.95,
        "lossy near-end node drooped before first source edge: min={}",
        n1_min
    );
    assert!(
        n2_min > 4.95,
        "lossy far-end node drooped before first source edge: min={}",
        n2_min
    );
}
