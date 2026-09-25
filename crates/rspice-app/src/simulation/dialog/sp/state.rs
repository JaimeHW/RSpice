//! S-parameter draft adapter for placed schematic RF ports.

use crate::simulation::placed_sources::PlacedRfPort;
use rspice_simulation_contract::sp_draft::SpPlacedPort;

#[cfg(test)]
pub use rspice_simulation_contract::sp_draft::TOUCHSTONE_VERSIONS;
pub use rspice_simulation_contract::sp_draft::{
    SpDialogState, SpPortSource, TOUCHSTONE_VERSION_LABELS,
};

fn placed_view(port: &PlacedRfPort) -> SpPlacedPort<'_> {
    SpPlacedPort {
        reference: &port.reference,
        port_number: port.port_number,
        z0: &port.z0,
        nets: &port.nets,
    }
}

/// Resolve the portable draft against the placed ports visible in the app.
pub fn to_config(
    draft: &SpDialogState,
    design: Option<&[PlacedRfPort]>,
) -> Result<super::SpConfig, String> {
    let placed = design.map(|ports| ports.iter().map(placed_view).collect::<Vec<_>>());
    draft.to_config(placed.as_deref())
}

/// Explain a port roster refusal using the same resolution as dispatch.
pub fn port_roster_error(draft: &SpDialogState, placed: &[PlacedRfPort]) -> Option<String> {
    let placed = placed.iter().map(placed_view).collect::<Vec<_>>();
    draft.port_roster_error(&placed)
}

#[cfg(test)]
use super::{SpConfig, SpPortConfig, SpSweepType};

#[cfg(test)]
mod tests {
    use super::*;
    use crate::simulation::placed_sources::{RfPortMode, placed_rf_ports};
    use crate::state::{Component, ComponentType, Point, SchematicState};

    /// A placed RF port, resolved exactly as the design would resolve it.
    fn placed(references_and_params: &[(&str, &str)]) -> Vec<PlacedRfPort> {
        let mut schematic = SchematicState::default();
        schematic.components = references_and_params
            .iter()
            .enumerate()
            .map(|(index, (name, params))| {
                let mut component =
                    Component::new(index as u64 + 1, ComponentType::RfPort, Point::origin())
                        .with_name_value(*name, "");
                component.params = (*params).to_owned();
                component
            })
            .collect();
        placed_rf_ports(&schematic, None)
    }

    fn two_placed_ports() -> Vec<PlacedRfPort> {
        placed(&[("P1", "port=1 z0=50"), ("P2", "port=2 z0=75")])
    }

    fn dialog() -> SpDialogState {
        SpDialogState::from_config(&SpConfig::default())
    }

    fn choosing(state: &mut SpDialogState, source: SpPortSource) {
        state.port_source_idx = Some(source.index());
    }

    /// The design answers the question the form has not been asked.
    #[test]
    fn an_unchosen_form_reads_the_source_the_design_declares() {
        let state = dialog();
        assert_eq!(state.port_source(2), SpPortSource::Placed);
        assert_eq!(state.port_source(0), SpPortSource::AdHoc);
    }

    /// And once it has been asked, the answer stands: an analysis must not
    /// change which ports it measures because a port was added to the sheet.
    #[test]
    fn a_chosen_source_survives_an_edit_to_the_design() {
        let mut state = dialog();
        choosing(&mut state, SpPortSource::AdHoc);
        assert_eq!(state.port_source(2), SpPortSource::AdHoc);
        choosing(&mut state, SpPortSource::Placed);
        assert_eq!(state.port_source(0), SpPortSource::Placed);
    }

    /// A project saved before the switch existed carries no choice, and must
    /// still load — as the design's own answer, which is what the solver was
    /// already giving it.
    #[test]
    fn a_project_without_the_key_loads_unchosen() {
        let saved = serde_json::to_value(dialog()).expect("the dialog serializes");
        let mut without = saved.as_object().expect("an object").clone();
        assert!(
            without.remove("port_source_idx").is_some(),
            "the key is written"
        );
        let decoded: SpDialogState =
            serde_json::from_value(without.into()).expect("a project without the key decodes");

        assert_eq!(decoded.port_source_idx, None);
        assert_eq!(decoded.port_source(1), SpPortSource::Placed);
    }

    /// Restoring a draft must preserve its options and unfinished input.
    #[test]
    fn sp_noise_and_all_authored_settings_survive_draft_restoration() {
        let mut state = SpDialogState::from_config(&SpConfig {
            start_freq: 3e3,
            stop_freq: 7e6,
            num_points: 31,
            sweep_type: SpSweepType::Linear,
            z0: 75.0,
            ports: vec![SpPortConfig::differential(1, "P", "N")],
            do_noise: true,
            touchstone_export: false,
            touchstone_version: 1,
        });
        state.port_source_idx = Some(SpPortSource::AdHoc.index());
        state.start_freq = "unfinished".into();
        let authored = serde_json::to_value(&state).unwrap();
        let mut restored: SpDialogState = serde_json::from_value(authored.clone()).unwrap();
        assert!(!restored.initialized);
        for _ in 0..3 {
            restored.ensure_initialized();
        }
        assert_eq!(serde_json::to_value(&restored).unwrap(), authored);
        assert!(to_config(&restored, Some(&[])).is_err());
    }

    /// The chosen export version is part of the draft, and a project saved
    /// before the chooser existed opens on the version its exports were
    /// already written in.
    #[test]
    fn a_touchstone_version_choice_survives_a_reopen() {
        for (index, version) in TOUCHSTONE_VERSIONS.iter().copied().enumerate() {
            let mut state = dialog();
            state.set_touchstone_version_index(index);
            assert_eq!(state.touchstone_version_index(), index);

            let authored = serde_json::to_value(&state).expect("the dialog serializes");
            let mut restored: SpDialogState =
                serde_json::from_value(authored).expect("the draft decodes");
            restored.ensure_initialized();
            assert_eq!(restored.touchstone_version, version);
            assert_eq!(restored.touchstone_version_index(), index);
            assert_eq!(
                to_config(&restored, Some(&[]))
                    .expect("the draft is runnable")
                    .touchstone_version,
                version,
                "the chosen version must reach the configuration the export reads"
            );
        }

        // A project saved before the key existed must still open, on the
        // version it was already exporting in rather than on zero.
        let saved = serde_json::to_value(dialog()).expect("the dialog serializes");
        let mut without = saved.as_object().expect("an object").clone();
        assert!(
            without.remove("touchstone_version").is_some(),
            "the key is written"
        );
        let decoded: SpDialogState =
            serde_json::from_value(without.into()).expect("a project without the key decodes");
        assert_eq!(
            decoded.touchstone_version,
            SpConfig::default().touchstone_version
        );

        // A stored value the writer cannot produce resolves to an offered
        // position rather than painting the chooser out of range.
        let mut odd = dialog();
        odd.touchstone_version = 7;
        assert_eq!(odd.touchstone_version_index(), 1);
        assert_eq!(to_config(&odd, Some(&[])).unwrap().touchstone_version, 2);
    }

    #[test]
    fn initializing_a_decoded_state_keeps_its_chosen_source() {
        let mut state = SpDialogState {
            port_source_idx: Some(SpPortSource::AdHoc.index()),
            ..Default::default()
        };
        state.ensure_initialized();
        assert_eq!(state.port_source(3), SpPortSource::AdHoc);
    }

    /// The run's ports are the placed ports: their numbers, their nets, their
    /// impedances. Nothing from the form's table reaches the roster.
    #[test]
    fn placed_mode_derives_the_run_ports_from_the_placed_ports() {
        let mut state = dialog();
        choosing(&mut state, SpPortSource::Placed);
        state.ports[0].node_pos = "TYPED_IN".to_owned();

        let config =
            to_config(&state, Some(&two_placed_ports())).expect("two placed ports resolve");

        assert_eq!(
            config
                .ports
                .iter()
                .map(|port| (port.number, port.z0))
                .collect::<Vec<_>>(),
            vec![(1, Some(50.0)), (2, Some(75.0))]
        );
        assert!(
            config.ports.iter().all(|port| port.node_pos != "TYPED_IN"),
            "the ad-hoc table is not consulted: {:?}",
            config.ports
        );
    }

    /// The table is retained through a placed-mode run, so switching back
    /// restores what the user wrote.
    #[test]
    fn placed_mode_retains_the_ad_hoc_table_it_is_not_reading() {
        let mut state = dialog();
        choosing(&mut state, SpPortSource::Placed);
        state.ports[0].node_pos = "TYPED_IN".to_owned();
        to_config(&state, Some(&two_placed_ports())).expect("two placed ports resolve");

        choosing(&mut state, SpPortSource::AdHoc);
        let config = to_config(&state, Some(&[])).expect("the table resolves");
        assert_eq!(config.ports[0].node_pos, "TYPED_IN");
    }

    /// Two ports answering to one number is a matrix with no defined meaning,
    /// and the refusal names the numbers rather than picking a winner.
    #[test]
    fn duplicate_placed_port_numbers_refuse_and_name_the_numbers() {
        let mut state = dialog();
        choosing(&mut state, SpPortSource::Placed);
        let ports = placed(&[
            ("PA", "port=1"),
            ("PB", "port=2"),
            ("PC", "port=2"),
            ("PD", "port=3"),
            ("PE", "port=3"),
        ]);

        let error = to_config(&state, Some(&ports)).expect_err("a collision refuses the run");

        assert!(
            error.contains("port numbers 2 and 3"),
            "the colliding numbers are named: {error}"
        );
        assert!(
            error.contains("PA, PB, PC, PD and PE"),
            "the ports are named: {error}"
        );
    }

    /// A gap is not a smaller matrix. The refusal names the number nothing
    /// claims, because that is the one edit that fixes it.
    #[test]
    fn a_gap_in_the_placed_port_numbers_refuses_and_names_the_gap() {
        let mut state = dialog();
        choosing(&mut state, SpPortSource::Placed);
        let ports = placed(&[("P1", "port=1"), ("P2", "port=2"), ("P4", "port=4")]);

        let error = to_config(&state, Some(&ports)).expect_err("a gap refuses the run");

        assert!(error.contains("skip port 3"), "{error}");
        assert!(error.contains("P4 at 4"), "{error}");
        assert!(error.contains("number them 1 to 3"), "{error}");
    }

    /// Placed mode against a sheet that places nothing states what to do about
    /// it, both ways.
    #[test]
    fn placed_mode_without_placed_ports_refuses() {
        let mut state = dialog();
        choosing(&mut state, SpPortSource::Placed);

        let error = to_config(&state, Some(&[])).expect_err("no placed port is no run");
        assert!(error.contains("the sheet places none"), "{error}");
        assert!(error.contains("Ad-hoc node ports"), "{error}");

        let one = placed(&[("P1", "port=1")]);
        let config = to_config(&state, Some(&one)).expect("one placed port measures reflection");
        assert_eq!(config.ports.len(), 1);
    }

    #[test]
    fn a_single_ad_hoc_port_survives_form_initialization_and_config_round_trip() {
        let mut state = dialog();
        choosing(&mut state, SpPortSource::AdHoc);
        state.ports.truncate(1);
        state.ports[0].node_pos = "SENSE".into();
        for _ in 0..3 {
            state.ensure_initialized();
            assert_eq!(state.ports.len(), 1);
        }
        let config = to_config(&state, Some(&[])).unwrap();
        let restored = SpDialogState::from_config(&config);
        assert_eq!(restored.ports.len(), 1);
        assert_eq!(restored.ports[0].node_pos, "SENSE");
        for invalid in [f64::NAN, f64::INFINITY] {
            let mut bad = config.clone();
            bad.z0 = invalid;
            assert!(bad.validate().is_err());
            bad = config.clone();
            bad.start_freq = invalid;
            assert!(bad.validate().is_err());
            bad = config.clone();
            bad.stop_freq = invalid;
            assert!(bad.validate().is_err());
            bad = config.clone();
            bad.ports[0].z0 = Some(invalid);
            assert!(bad.validate().is_err());
        }
    }

    /// A caller that cannot see the design must not refuse a placed-mode
    /// analysis for placing nothing it was never shown.
    #[test]
    fn placed_mode_with_the_design_out_of_view_validates_only_what_it_can_see() {
        let mut state = dialog();
        choosing(&mut state, SpPortSource::Placed);

        let config = state.to_config(None).expect("the sweep still validates");
        assert!(config.ports.is_empty());

        state.stop_freq = "1k".to_owned();
        state.start_freq = "1Meg".to_owned();
        assert!(
            state.to_config(None).is_err(),
            "an inverted sweep is still refused"
        );
    }

    /// Ad-hoc mode beside placed ports is refused rather than warned about.
    ///
    /// The deck's own `P` cards win at the solver, so the table on this form
    /// never reaches the network — but it does reach the spec's port list, and
    /// every reader of that list takes it for the run's roster. Running would
    /// report a network measured on one set of ports under another set's
    /// reference impedances.
    #[test]
    fn ad_hoc_mode_beside_placed_ports_refuses_and_says_why() {
        let mut state = dialog();
        choosing(&mut state, SpPortSource::AdHoc);

        let error = to_config(&state, Some(&two_placed_ports()))
            .expect_err("two declarations of one thing is no run");

        assert!(error.contains("places 2 RF ports"), "{error}");
        assert!(error.contains("P1 and P2"), "{error}");
        assert!(error.contains("would not be used"), "{error}");
        assert!(error.contains("From placed RF ports"), "{error}");
    }

    /// And with nothing placed, ad-hoc mode is exactly what it always was.
    #[test]
    fn ad_hoc_mode_without_placed_ports_is_unchanged() {
        let mut state = dialog();
        state.z0 = " 0.05k ".into();
        state.num_points = " 10 ".into();
        state.ports[1].z0_override = true;
        state.ports[1].z0 = "0.075k".into();

        let from_design = to_config(&state, Some(&[])).expect("the table resolves");
        let blind = state.to_config(None).expect("the table resolves");

        assert_eq!(from_design.ports.len(), 2);
        assert_eq!(from_design.ports[0].node_pos, "IN");
        assert_eq!(from_design.ports[1].node_pos, "OUT");
        assert_eq!(blind.ports.len(), 2);
        assert_eq!(from_design.z0, 50.0);
        assert_eq!(blind.z0, 50.0);
        assert_eq!(from_design.ports[0].z0, None);
        assert_eq!(from_design.ports[1].z0, Some(75.0));
        assert_eq!(from_design.num_points, 10);
    }

    /// An impedance the port states as an expression is not a figure this
    /// surface can normalize to, and the `P` card carries it to the solver
    /// regardless. The description falls back to the run's reference impedance
    /// rather than inventing one.
    #[test]
    fn a_placed_impedance_that_is_not_a_number_falls_back_to_the_runs_z0() {
        let mut state = dialog();
        choosing(&mut state, SpPortSource::Placed);
        let ports = placed(&[("P1", "port=1 z0={ZL}"), ("P2", "port=2 z0=50")]);

        let config = to_config(&state, Some(&ports)).expect("the roster resolves");

        assert_eq!(config.ports[0].z0, None);
        assert_eq!(config.ports[1].z0, Some(50.0));
    }

    /// The mode label a placed row states is the derivation's own, not a
    /// second spelling of it.
    #[test]
    fn a_placed_port_row_reads_its_mode_from_the_derivation() {
        let ports = placed(&[("P1", "port=1 ac_mag=1"), ("P2", "port=2")]);
        assert_eq!(ports[0].mode, RfPortMode::AcDrive);
        assert_eq!(ports[0].mode.label(), "AC drive");
        assert_eq!(ports[1].mode.label(), "term");
    }
}
