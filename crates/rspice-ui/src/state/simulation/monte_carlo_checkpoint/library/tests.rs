use super::*;

#[test]
fn monte_carlo_checkpoint_file_round_trip_rejects_corruption_and_bounds() {
    let (_, bytes, _) =
        crate::simulation::runner::monte_carlo_checkpoint::tests::completed_checkpoint_fixture();
    let checkpoint = MonteCarloCheckpointEvidence::from_bytes(bytes.clone()).unwrap();
    let source = String::from_utf8(checkpoint.to_portable_file().unwrap()).unwrap();
    let restored = MonteCarloCheckpointEvidence::from_portable_file(&source).unwrap();
    assert_eq!(restored.bytes(), &*bytes);
    assert_eq!(restored, checkpoint);
    for change in 0..4 {
        let mut document: serde_json::Value = serde_json::from_str(&source).unwrap();
        match change {
            0 => document["version"] = serde_json::json!(2),
            1 => document["checkpoint"]["data"] = serde_json::json!("AAAA"),
            2 => {
                document["checkpoint"]["digest"] =
                    serde_json::to_value(ContentDigest::from_bytes([5; 32])).unwrap()
            }
            _ => document["unknown"] = serde_json::json!(true),
        }
        assert!(MonteCarloCheckpointEvidence::from_portable_file(&document.to_string()).is_err());
    }
    let mut library = MonteCarloCheckpointLibrary::default();
    assert!(
        library
            .insert_bounded("a".into(), checkpoint.clone(), bytes.len())
            .is_err()
    );
    assert!(library.is_empty());
    assert!(
        library
            .insert("import.rspice-mc".into(), checkpoint.clone())
            .unwrap()
    );
    assert!(
        !library
            .insert("duplicate.rspice-mc".into(), checkpoint.clone())
            .unwrap()
    );
    let mut entries = serde_json::to_value(&library).unwrap();
    let duplicate = entries[0].clone();
    entries.as_array_mut().unwrap().push(duplicate);
    assert!(serde_json::from_value::<MonteCarloCheckpointLibrary>(entries).is_err());
    let frozen = library.clone();
    assert!(library.remove(checkpoint.digest()));
    assert!(library.is_empty());
    assert_eq!(frozen.get(checkpoint.digest()), Some(&checkpoint));

    // Imported inputs survive project result persistence without native runs.
    let mut simulation = crate::state::SimulationState::default();
    simulation.imported_monte_carlo_checkpoints = frozen;
    let results = crate::io::ProjectSimulationResults::from_state(&simulation);
    assert!(!results.is_empty());
    let restored: crate::io::ProjectSimulationResults =
        serde_json::from_str(&serde_json::to_string(&results).unwrap()).unwrap();
    let restored = restored.into_simulation_state().unwrap();
    assert!(restored.runs.is_empty());
    assert_eq!(
        restored
            .imported_monte_carlo_checkpoints
            .get(checkpoint.digest()),
        Some(&checkpoint)
    );
}
