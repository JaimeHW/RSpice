use super::*;
use crate::ResourceLimits;
use crate::io::parse_raw_plots_reader_with_limits;
use std::io::Cursor;

fn fixture() -> RawFile {
    let source = "Title: event\nPlotname: Digital Events (rspice-digital-events/1)\nFlags: real\nNo. Variables: 2\nNo. Points: 2\nVariables:\n0 time time\n1 D(clk) digital\nValues:\n0 0 0\n1 1 1\n";
    parse_raw_plots_reader_with_limits(&mut Cursor::new(source), ResourceLimits::default()).unwrap()
}

#[test]
fn explicit_tables_do_not_become_event_histories_from_their_display_title() {
    for kind in [RawEventKind::Digital, RawEventKind::Real, RawEventKind::Bus] {
        let mut file = fixture();
        let plot = &mut file.plots[0];
        plot.header.plotname = kind.plot_name().into();
        let mut metadata = b"Title: table\nPlotname: table\n".to_vec();
        crate::io::ltspice_raw::write_raw_table_layout_metadata(
            &mut metadata,
            &[],
            &[Some("ms".into()), None],
            None,
        )
        .unwrap();
        metadata.extend_from_slice(b"Flags: real\nNo. Variables: 2\nNo. Points: 1\nVariables:\n0 time time\n1 V(out) voltage\nValues:\n0 0 1\n");
        plot.header.command = crate::io::parse_raw_reader(&mut Cursor::new(metadata))
            .unwrap()
            .header
            .command;
        let traces = decode_event_plots(&file).unwrap();
        assert!(traces.digital_traces.is_empty());
        assert!(traces.real_traces.is_empty());
        assert!(traces.digital_buses.is_empty());

        // Removing the explicit layout restores the legacy event contract.
        file.plots[0].header.command =
            "RSpiceTableV2 {\"real_variables\":[],\"units\":[\"ms\",null]}".into();
        assert!(decode_event_plots(&file).is_err());
    }
}

#[test]
fn public_event_decoder_rejects_partial_columns_and_nonfinite_times() {
    for mutation in 0..7 {
        let mut file = fixture();
        let plot = &mut file.plots[0];
        match mutation {
            0 => {
                plot.waveforms.pop();
            }
            1 => {
                plot.waveforms[1].y.pop();
            }
            2 => {
                plot.header.no_points += 1;
            }
            3 => {
                plot.header.no_variables += 1;
            }
            4 => {
                plot.waveforms[0].y[1] = f64::NAN;
            }
            5 => {
                plot.waveforms[0].y[1] = f64::INFINITY;
            }
            6 => {
                plot.waveforms[0].y_imag = Some(vec![0.0, 1.0]);
            }
            _ => unreachable!(),
        }
        assert!(
            matches!(
                decode_event_plots(&file),
                Err(EventPlotError::InvalidData { .. })
            ),
            "{mutation}"
        );
    }
}

#[test]
fn invalid_bus_columns_are_checked_even_when_scalar_members_take_precedence() {
    let mut file = fixture();
    let mut bus = fixture().plots.remove(0);
    bus.header.plotname = RawEventKind::Bus.plot_name().to_owned();
    bus.header.title = "data[0:0]".into();
    bus.waveforms[1].y[1] = 13.0;
    file.plots.push(bus);
    assert!(matches!(
        decode_event_plots(&file),
        Err(EventPlotError::DigitalCode { value: 13.0, .. })
    ));
}

#[test]
fn real_event_histories_reject_nonfinite_values() {
    for value in [f64::NAN, f64::INFINITY, f64::NEG_INFINITY] {
        let mut file = fixture();
        let plot = &mut file.plots[0];
        plot.header.plotname = RawEventKind::Real.plot_name().to_owned();
        plot.variables[1].name = "E(ctrl)".into();
        plot.variables[1].var_type = "real".into();
        plot.waveforms[1].y[1] = value;
        assert!(matches!(
            decode_event_plots(&file),
            Err(EventPlotError::InvalidData { .. })
        ));
    }
}
