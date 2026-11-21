//! The streamed reader against the whole-file reader, over the binary layouts simulators write.

use super::*;

/// A one-plot binary rawfile: `variables` as `(name, type)`, the coordinate first, and each row
/// as its values in file order, a complex value as its two parts.
fn binary_raw(
    plot: &str,
    flags: &str,
    command: &str,
    variables: &[(&str, &str)],
    rows: &[Vec<f64>],
    singles: bool,
) -> Vec<u8> {
    let complex = flags.contains("complex");
    let mut bytes = format!(
        "Title: streamed\nDate: today\nPlotname: {plot}\nFlags: {flags}\n{command}No. Variables: {}\nNo. Points: {}\nVariables:\n",
        variables.len(),
        rows.len()
    )
    .into_bytes();
    for (index, (name, kind)) in variables.iter().enumerate() {
        bytes.extend(format!("\t{index}\t{name}\t{kind}\n").bytes());
    }
    bytes.extend(b"Binary:\n");
    let width = if complex { 2 } else { 1 };
    for row in rows {
        for (index, value) in row.iter().enumerate() {
            if singles && index >= width {
                bytes.extend((*value as f32).to_le_bytes());
            } else {
                bytes.extend(value.to_le_bytes());
            }
        }
    }
    bytes
}

struct Streamed {
    plot: StreamedRawPlot,
    coordinate: Vec<f64>,
    real: Vec<Vec<f64>>,
    imag: Vec<Vec<f64>>,
    blocks: usize,
}

/// `bytes` read by the streamed reader, fed `chunk` bytes at a time.
fn streamed(bytes: &[u8], chunk: usize) -> Result<Streamed, String> {
    let start = RawStreamReader::start(bytes, bytes.len() as u64, ResourceLimits::default())
        .map_err(|error| error.to_string())?;
    let RawStreamStart::Streamed(mut reader) = start else {
        panic!("a plot the reader streams: {start:?}");
    };
    let plot = reader.plot().clone();
    let signals = plot.signals.len();
    let mut out = Streamed {
        coordinate: Vec::new(),
        real: vec![Vec::new(); signals],
        imag: vec![Vec::new(); if plot.complex { signals } else { 0 }],
        blocks: 0,
        plot,
    };
    let mut sink = |block: RawRowBlock<'_>| {
        out.coordinate.extend_from_slice(block.coordinate);
        for (column, values) in out.real.iter_mut().zip(block.real) {
            column.extend_from_slice(values);
        }
        for (column, values) in out.imag.iter_mut().zip(block.imag) {
            column.extend_from_slice(values);
        }
        out.blocks += 1;
        Ok(())
    };
    for bytes in bytes[reader.data_offset()..].chunks(chunk) {
        reader.feed(bytes, &mut sink)?;
    }
    reader.finish(&mut sink)?;
    Ok(out)
}

/// `bytes` read by the whole-file reader.
fn whole(bytes: &[u8]) -> Result<crate::numeric::DecodedNumericDataset, String> {
    let mut plots =
        super::super::decode_spice_raw_plots_with_log(bytes, None, ResourceLimits::default())
            .map_err(|error| error.to_string())?;
    assert_eq!(plots.len(), 1);
    Ok(plots.remove(0).dataset)
}

fn bits(values: &[f64]) -> Vec<u64> {
    values.iter().map(|value| value.to_bits()).collect()
}

/// Both readers give the same plot, bit for bit: its coordinate, each signal's parts, its names
/// and units.
fn assert_same(bytes: &[u8], chunk: usize) -> Streamed {
    let streamed = streamed(bytes, chunk).expect("the streamed reader reads it");
    let whole = whole(bytes).expect("the whole-file reader reads it");
    assert_eq!(streamed.plot.domain, whole.domain);
    assert_eq!(streamed.plot.coordinate_name, whole.coordinate_name);
    assert_eq!(streamed.plot.coordinate_unit, whole.coordinate_unit);
    assert_eq!(bits(&streamed.coordinate), bits(&whole.coordinate));
    assert_eq!(streamed.plot.signals.len(), whole.signals.len());
    for (index, signal) in whole.signals.iter().enumerate() {
        assert_eq!(streamed.plot.signals[index].0, signal.name);
        assert_eq!(streamed.plot.signals[index].1, signal.unit);
        assert_eq!(
            bits(&streamed.real[index]),
            bits(&signal.real),
            "{}",
            signal.name
        );
        match &signal.imag {
            Some(imag) => assert_eq!(bits(&streamed.imag[index]), bits(imag)),
            None => assert!(streamed.imag.is_empty()),
        }
    }
    streamed
}

const TRANSIENT: [(&str, &str); 3] = [
    ("time", "time"),
    ("V(out)", "voltage"),
    ("I(R1)", "device_current"),
];

/// A real plot of doubles, and LTspice's singles beside a double coordinate, read the same fed a
/// few bytes at a time as whole, a row split across feeds included.
#[test]
fn a_binary_plot_streams_to_the_values_the_whole_file_reader_reads() {
    let rows = (0..9)
        .map(|row| {
            let time = row as f64 * 1.0e-6;
            vec![time, (time * 1.0e5).sin(), -0.0 - time]
        })
        .collect::<Vec<_>>();
    for (flags, singles) in [("real double", false), ("real", true)] {
        let bytes = binary_raw("Transient Analysis", flags, "", &TRANSIENT, &rows, singles);
        for chunk in [1, 7, 64, bytes.len()] {
            assert_same(&bytes, chunk);
        }
    }
}

/// An AC plot's complex values stream as their two parts, and a coordinate with an imaginary
/// part is refused as the whole-file reader refuses it.
#[test]
fn a_complex_plot_streams_both_parts() {
    let variables = [("frequency", "frequency"), ("V(out)", "voltage")];
    let rows = (1..6)
        .map(|row| vec![row as f64 * 10.0, 0.0, 1.0 / row as f64, -(row as f64)])
        .collect::<Vec<_>>();
    for singles in [false, true] {
        let bytes = binary_raw("AC Analysis", "complex", "", &variables, &rows, singles);
        assert_same(&bytes, 5);
    }
    let mut skewed = rows.clone();
    skewed[2][1] = 1.0;
    let bytes = binary_raw("AC Analysis", "complex", "", &variables, &skewed, false);
    assert_eq!(
        streamed(&bytes, 3).err(),
        whole(&bytes).err(),
        "a coordinate with an imaginary part"
    );
}

/// LTspice sets the sign bit of each time its compression kept; both readers take the magnitude.
#[test]
fn ltspice_compression_marks_read_as_the_time_they_mark() {
    let rows = (0..6)
        .map(|row| {
            let time = row as f64 * 2.0e-9;
            vec![if row % 2 == 1 { -time } else { time }, 1.0, 2.0]
        })
        .collect::<Vec<_>>();
    let bytes = binary_raw(
        "Transient Analysis",
        "real forward",
        "Command: Linear Technology Corporation LTspice XVII\n",
        &TRANSIENT,
        &rows,
        true,
    );
    let streamed = assert_same(&bytes, 11);
    assert!(
        streamed
            .coordinate
            .iter()
            .all(|time| time.is_sign_positive())
    );
}

/// Rows past one block reach the sink a block at a time.
#[test]
fn a_plot_longer_than_a_block_reaches_the_sink_in_blocks() {
    let rows = (0..300_000)
        .map(|row| vec![row as f64, row as f64 * 0.5])
        .collect::<Vec<_>>();
    let bytes = binary_raw(
        "Transient Analysis",
        "real double",
        "",
        &TRANSIENT[..2],
        &rows,
        false,
    );
    let streamed = assert_same(&bytes, 1 << 20);
    assert_eq!(streamed.blocks, 2);
}

/// A value that is not a number, and a file that ends early, are refused in the whole-file
/// reader's words.
#[test]
fn a_bad_value_and_a_short_feed_are_refused() {
    let mut rows = (0..4)
        .map(|row| vec![row as f64, 1.0, 2.0])
        .collect::<Vec<_>>();
    rows[2][1] = f64::NAN;
    let bytes = binary_raw(
        "Transient Analysis",
        "real double",
        "",
        &TRANSIENT,
        &rows,
        false,
    );
    assert_eq!(streamed(&bytes, 13).err(), whole(&bytes).err());

    rows[2][1] = 1.0;
    let bytes = binary_raw(
        "Transient Analysis",
        "real double",
        "",
        &TRANSIENT,
        &rows,
        false,
    );
    let RawStreamStart::Streamed(mut reader) =
        RawStreamReader::start(&bytes, bytes.len() as u64, ResourceLimits::default()).unwrap()
    else {
        panic!("a plot the reader streams");
    };
    let mut sink = |_: RawRowBlock<'_>| Ok(());
    reader
        .feed(&bytes[reader.data_offset()..bytes.len() - 5], &mut sink)
        .unwrap();
    let error = reader.finish(&mut sink).unwrap_err();
    assert!(error.contains("ends after 3 of the 4 rows"), "{error}");
}

/// What the streamed reader does not read, it says so before a row, and the whole-file reader
/// reads instead: text values, column-by-column layout, a stepped sweep, a second plot behind the
/// first, and a header longer than the bytes read first.
#[test]
fn files_the_reader_does_not_stream_go_to_the_whole_file_reader() {
    let rows = (0..3)
        .map(|row| vec![row as f64, 1.0, 2.0])
        .collect::<Vec<_>>();
    let start = |bytes: &[u8], head: usize| match RawStreamReader::start(
        &bytes[..head],
        bytes.len() as u64,
        ResourceLimits::default(),
    )
    .unwrap()
    {
        RawStreamStart::WholeFile(why) => why,
        RawStreamStart::Streamed(_) => "streamed",
    };
    let binary =
        |flags: &str| binary_raw("Transient Analysis", flags, "", &TRANSIENT, &rows, false);
    let one = binary("real double");
    let header = one
        .windows(8)
        .position(|line| line == b"Binary:\n")
        .expect("a binary header");
    let mut text = one[..header].to_vec();
    text.extend(b"Values:\n0\t0\n\t1\n\t2\n");
    assert_eq!(start(&text, text.len()), "its values are text");
    let fast = binary("real double fastaccess");
    assert_eq!(
        start(&fast, fast.len()),
        "its values are stored column by column"
    );
    let stepped = binary("real double stepped");
    assert_eq!(
        start(&stepped, stepped.len()),
        "it holds every run of a .step sweep"
    );
    let mut two = binary("real double");
    two.extend(binary("real double"));
    assert_eq!(start(&two, two.len()), "its rows do not end the file");
    assert_eq!(
        start(&one, 40),
        "its header does not read in the bytes read first"
    );
    assert_eq!(start(&one, one.len()), "streamed");
}
