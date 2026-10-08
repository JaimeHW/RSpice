//! Metadata arrays must not allocate beyond the plot's admitted column count.
use rspice_core::io::ltspice_raw::{
    parse_raw_plots_bytes_with_limits, raw_table_has_coordinate, raw_table_units,
    write_raw_metadata_options,
};

fn plot(metadata: &str, modern: bool) -> Vec<u8> {
    let mut bytes = b"Title: metadata\nPlotname: metadata\n".to_vec();
    let command = format!("RSpiceTableV4 {metadata}");
    if modern {
        write_raw_metadata_options(&mut bytes, &command).unwrap();
    } else {
        bytes.extend(format!("Command: {command}\n").as_bytes());
    }
    bytes.extend(b"Flags: complex\nNo. Variables: 2\nNo. Points: 0\nVariables:\n0 time time\n1 out voltage\nValues:\n");
    bytes
}

#[test]
fn excessive_arrays_are_refused_before_scanning_or_allocating_their_tail() {
    let cases = [
        r#"{"real_variables":[0,1,2,INVALID]}"#,
        r#"{"units":[null,null,null,INVALID]}"#,
        r#"{"text":{"variables":[["a","time"],["b","voltage"],["c","current"],INVALID]}}"#,
        // Escaped JSON keys have the same admission contract.
        r#"{"\u0075nits":[null,null,null,INVALID]}"#,
        r#"{"text":{"\u0076ariables":[[],[],[],INVALID]}}"#,
    ];
    for modern in [false, true] {
        for metadata in cases {
            let error =
                parse_raw_plots_bytes_with_limits(&plot(metadata, modern), Default::default())
                    .unwrap_err()
                    .to_string();
            assert!(
                error.contains("array exceeds declared variable count 2"),
                "{error}"
            );
        }
    }
}

#[test]
fn exact_metadata_arrays_preserve_units_and_escaped_labels() {
    let metadata = r#"{"real_variables":[0,1],"units":["s","V"],"layout":"coordinate-first","text":{"plot_name":"quoted table","variables":[[" time ","time"],["V(µ\nout)","voltage"]]}}"#;
    for modern in [false, true] {
        let file =
            parse_raw_plots_bytes_with_limits(&plot(metadata, modern), Default::default()).unwrap();
        let decoded = &file.plots[0];
        assert!(raw_table_has_coordinate(&decoded.header).unwrap());
        assert_eq!(
            raw_table_units(&decoded.header).unwrap(),
            Some(vec![Some("s".into()), Some("V".into())])
        );
        assert_eq!(decoded.header.plotname, "quoted table");
        assert_eq!(decoded.variables[0].name, " time ");
        assert_eq!(decoded.variables[1].name, "V(µ\nout)");
        assert!(
            decoded
                .waveforms
                .iter()
                .all(|waveform| waveform.y_imag.is_none())
        );
    }
}
