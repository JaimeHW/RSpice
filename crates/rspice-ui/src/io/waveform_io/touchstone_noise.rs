//! Two-port noise interchange. Touchstone uses dB/MA regardless of the
//! network format, and normalizes Rn only in v1 (IBIS Touchstone 2.0,
//! "Noise Parameters"). Conventional noise figures refer to a 290 K source.

use super::{SignalType, WaveformDataset, WaveformSignal};

pub(super) const REFERENCE_TEMPERATURE: f64 = 290.0;

pub(super) fn validate_frequencies(frequencies: &[f64]) -> Result<(), String> {
    if frequencies.is_empty()
        || frequencies.iter().any(|f| !f.is_finite() || *f <= 0.0)
        || frequencies.windows(2).any(|pair| pair[0] >= pair[1])
    {
        return Err(
            "Touchstone frequency samples must be finite, positive and strictly increasing".into(),
        );
    }
    Ok(())
}

/// Return wire records at 290 K. The option-line resistance is the source
/// reference, which the writer chooses to equal network port 1's reference.
pub(super) fn noise_records_for_write(
    dataset: &WaveformDataset,
    network_frequencies: &[f64],
    ports: usize,
    reference: f64,
    version: u32,
) -> Result<Vec<[f64; 5]>, String> {
    let mut components: [Option<&WaveformSignal>; 4] = [None; 4];
    for signal in &dataset.signals {
        let name = signal.name.trim().to_ascii_lowercase();
        let slot = match name.as_str() {
            "fmin" => 0,
            "rn" => 1,
            "sopt_re" | "re(sopt)" => 2,
            "sopt_im" | "im(sopt)" => 3,
            _ => continue,
        };
        if components[slot].replace(signal).is_some() {
            return Err(format!(
                "Duplicate Touchstone noise component '{}'",
                signal.name
            ));
        }
    }
    if components.iter().all(Option::is_none) {
        if dataset
            .metadata
            .contains_key("noise_reference_temperature_kelvin")
        {
            return Err("Touchstone noise export requires the two-port Fmin, Rn and complex Sopt traces; use a result bundle for full port-noise covariance".into());
        }
        return Ok(Vec::new());
    }
    if ports != 2 {
        return Err("Touchstone noise data requires exactly two ports".into());
    }
    let [Some(fmin), Some(rn), Some(real), Some(imag)] = components else {
        return Err("Touchstone noise export requires Fmin, Rn and both Sopt components".into());
    };
    for signal in [fmin, real, imag] {
        if !matches!(signal.unit.trim(), "" | "1") {
            return Err(format!(
                "Touchstone noise component '{}' requires linear dimensionless values, not '{}'",
                signal.name, signal.unit
            ));
        }
    }
    let unit = rn.unit.trim();
    if !matches!(unit, "" | "Ω")
        && !unit.eq_ignore_ascii_case("ohm")
        && !unit.eq_ignore_ascii_case("ohms")
    {
        return Err(format!(
            "Touchstone Rn requires resistance in ohms, not '{}'",
            rn.unit
        ));
    }
    let frequencies = fmin.x_values.as_deref().unwrap_or(network_frequencies);
    validate_frequencies(frequencies)?;
    if frequencies[0] > network_frequencies[network_frequencies.len() - 1] {
        return Err(
            "First Touchstone noise frequency must not exceed the last network frequency".into(),
        );
    }
    for signal in [fmin, rn, real, imag] {
        if signal.data.len() != frequencies.len()
            || signal.x_values.as_deref().unwrap_or(network_frequencies) != frequencies
        {
            return Err(format!(
                "Touchstone noise component '{}' does not match its frequency grid",
                signal.name
            ));
        }
    }
    let temperature = dataset
        .metadata
        .get("noise_reference_temperature_kelvin")
        .map(|value| {
            value
                .parse::<f64>()
                .map_err(|_| "Invalid noise reference temperature".to_owned())
        })
        .transpose()?
        .unwrap_or(REFERENCE_TEMPERATURE);
    if !temperature.is_finite() || temperature <= 0.0 {
        return Err("Noise reference temperature must be finite and positive".into());
    }
    let ratio = temperature / REFERENCE_TEMPERATURE;
    frequencies
        .iter()
        .enumerate()
        .map(|(i, frequency)| {
            let factor = fmin.data[i];
            let resistance = rn.data[i];
            let re = real.data[i];
            let im = imag.data[i];
            validate_parameters(factor, resistance, re, im)?;
            // F(T0)-1 = (F(T)-1)*T/T0. Rn is also proportional to the
            // normalized noise covariance; Gamma_opt is invariant under scaling.
            let excess_290 = (factor - 1.0) * ratio;
            let resistance_290 = resistance * ratio;
            let wire_resistance = if version < 2 {
                resistance_290 / reference
            } else {
                resistance_290
            };
            if !excess_290.is_finite()
                || !wire_resistance.is_finite()
                || (factor > 1.0 && excess_290 == 0.0)
                || (resistance > 0.0 && wire_resistance == 0.0)
            {
                return Err(
                    "Touchstone noise normalization exceeds the representable range".into(),
                );
            }
            Ok([
                *frequency,
                // ln_1p retains a noise figure below one ULP of a linear
                // factor; forming 1 + excess first would round it to zero dB.
                (10.0 / std::f64::consts::LN_10) * excess_290.ln_1p(),
                re.hypot(im),
                im.atan2(re).to_degrees(),
                wire_resistance,
            ])
        })
        .collect()
}

fn validate_parameters(factor: f64, resistance: f64, re: f64, im: f64) -> Result<(), String> {
    if !factor.is_finite()
        || factor < 1.0
        || !resistance.is_finite()
        || resistance < 0.0
        || !re.is_finite()
        || !im.is_finite()
        || re.hypot(im) > 1.0 + 1.0e-12
    {
        return Err(
            "Touchstone noise parameters require finite Fmin >= 1, Rn >= 0 and |Sopt| <= 1".into(),
        );
    }
    Ok(())
}

/// Preserve the independent noise sweep as ordinary traces. Translate
/// Gamma_opt to network port 1's reference: [Reference] does not apply to
/// the wire noise values, which always use the option-line resistance.
pub(super) fn append_noise_signals(
    dataset: &mut WaveformDataset,
    records: &[[f64; 5]],
    frequency_scale: f64,
    option_reference: f64,
    network_reference: f64,
    version: u32,
) -> Result<(), String> {
    if records.is_empty() {
        return Ok(());
    }
    let mut frequencies = Vec::with_capacity(records.len());
    let mut values: [Vec<f64>; 4] = std::array::from_fn(|_| Vec::with_capacity(records.len()));
    let scale = option_reference.max(network_reference);
    let old = option_reference / scale;
    let new = network_reference / scale;
    let shift = (old - new) / (old + new);
    for [frequency, db, magnitude, angle, resistance] in records {
        frequencies.push(frequency * frequency_scale);
        let factor = 10.0_f64.powf(db / 10.0);
        let normalized_resistance = if version < 2 {
            resistance * option_reference
        } else {
            *resistance
        };
        if *resistance > 0.0 && normalized_resistance == 0.0 {
            return Err(
                "Touchstone noise resistance normalization underflows the representable range"
                    .into(),
            );
        }
        let resistance = normalized_resistance;
        if *db < 0.0 || *magnitude < 0.0 || !angle.is_finite() {
            return Err("Touchstone noise figure and optimum magnitude must be nonnegative and phase finite".into());
        }
        let (sin, cos) = angle.to_radians().sin_cos();
        let re = magnitude * cos;
        let im = magnitude * sin;
        validate_parameters(factor, resistance, re, im)?;
        let denom_re = 1.0 + shift * re;
        let denom_im = shift * im;
        let denominator = denom_re * denom_re + denom_im * denom_im;
        let out_re = ((re + shift) * denom_re + im * denom_im) / denominator;
        let out_im = (im * denom_re - (re + shift) * denom_im) / denominator;
        validate_parameters(factor, resistance, out_re, out_im)?;
        for (column, value) in values.iter_mut().zip([factor, resistance, out_re, out_im]) {
            column.push(value);
        }
    }
    validate_frequencies(&frequencies)?;
    let network = &dataset
        .x_signal
        .as_ref()
        .ok_or("Missing network frequency axis")?
        .data;
    if frequencies[0] > network[network.len() - 1] {
        return Err(
            "First Touchstone noise frequency must not exceed the last network frequency".into(),
        );
    }
    let separate_axis = (&frequencies != network).then_some(frequencies);
    for ((name, unit), data) in [
        ("Fmin", "1"),
        ("Rn", "Ω"),
        ("Sopt_RE", "1"),
        ("Sopt_IM", "1"),
    ]
    .into_iter()
    .zip(values)
    {
        let mut signal = WaveformSignal::new(name, SignalType::Unknown);
        signal.unit = unit.into();
        signal.data = data;
        signal.x_values = separate_axis.clone();
        dataset.add_signal(signal);
    }
    dataset.metadata.insert(
        "noise_reference_temperature_kelvin".into(),
        REFERENCE_TEMPERATURE.to_string(),
    );
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::io::waveform_io::{WaveformFormat, WaveformWriter, read_touchstone_bytes};

    fn fixture(version: u32, reference: &str, noise: &str) -> String {
        let network = "1 0.1 0 0.5 0 0.2 0 0.1 0\n3 0.2 0 0.6 0 0.2 0 0.2 0\n";
        if version == 1 {
            format!("# MHz S RI R 75\n{network}{noise}")
        } else {
            format!(
                "[Version] 2.0\n# MHz S RI R 75\n[Number of Ports] 2\n[Two-Port Data Order] 21_12\n[Number of Frequencies] 2\n[Number of Noise Frequencies] 3\n{reference}[Network Data]\n{network}[Noise Data]\n{noise}[End]\n"
            )
        }
    }

    #[test]
    fn touchstone_noise_round_trip_keeps_independent_grid_units_and_phase() {
        for version in [1, 2] {
            let resistance = if version == 1 { "0.2" } else { "15" };
            let noise =
                format!("1.5 3 0.5 -90 {resistance}\n2 0 1 0 {resistance}\n4 6 0 0 {resistance}\n");
            let dataset =
                read_touchstone_bytes("noise.s2p", fixture(version, "", &noise).as_bytes())
                    .unwrap();
            assert_eq!(
                dataset.metadata["noise_reference_temperature_kelvin"],
                "290"
            );
            assert_eq!(dataset.get_signal("Rn").unwrap().data, [15.0; 3]);
            assert_eq!(dataset.get_signal("Rn").unwrap().unit, "Ω");
            assert_eq!(
                dataset.get_signal("Rn").unwrap().x_values.as_deref(),
                Some([1.5e6, 2.0e6, 4.0e6].as_slice())
            );
            assert!((dataset.get_signal("Sopt_IM").unwrap().data[0] + 0.5).abs() < 1e-14);
            assert!(
                (dataset.get_signal("Fmin").unwrap().data[0] - 10.0_f64.powf(0.3)).abs() < 1e-14
            );
            let text = WaveformWriter::new(WaveformFormat::Touchstone)
                .write_text(&dataset)
                .unwrap();
            let restored = read_touchstone_bytes("noise.s2p", text.as_bytes()).unwrap();
            for name in ["Rn", "Fmin", "Sopt_RE", "Sopt_IM"] {
                let expected = dataset.get_signal(name).unwrap();
                let actual = restored.get_signal(name).unwrap();
                assert_eq!(actual.x_values, expected.x_values);
                for (a, b) in actual.data.iter().zip(&expected.data) {
                    assert!((a - b).abs() < 1e-13, "{name}: {a} != {b}");
                }
            }
            assert!(
                WaveformWriter::new(WaveformFormat::Csv)
                    .write_text(&dataset)
                    .unwrap_err()
                    .contains("identical coordinates")
            );
        }
    }

    #[test]
    fn touchstone_noise_option_reference_is_independent_of_network_reference() {
        let source = fixture(
            2,
            "[Reference] 25 100\n",
            "1.5 0 0 0 15\n2 0 1 0 15\n3 0 1 180 15\n",
        );
        let mut dataset = read_touchstone_bytes("noise.ts", source.as_bytes()).unwrap();
        let real = &dataset.get_signal("Sopt_RE").unwrap().data;
        // A source matched to the option line (75 ohms) reflects +0.5 in
        // the 25-ohm port-1 reference; open and short remain +1 and -1.
        for (actual, expected) in real.iter().zip([0.5, 1.0, -1.0]) {
            assert!((actual - expected).abs() < 1e-14);
        }
        let text = WaveformWriter::new(WaveformFormat::Touchstone)
            .write_text(&dataset)
            .unwrap();
        assert!(text.contains("# Hz S RI R 25"));
        assert!(text.contains("[Reference] 2.5e1 1e2"));
        let restored = read_touchstone_bytes("noise.ts", text.as_bytes()).unwrap();
        for (actual, expected) in restored
            .get_signal("Sopt_RE")
            .unwrap()
            .data
            .iter()
            .zip(real)
        {
            assert!((actual - expected).abs() < 1e-14);
        }
        // Reference impedances are authored values. An absolute tolerance
        // must not collapse distinct small impedances to the first port's R.
        dataset
            .metadata
            .insert("z0_ports".into(), "0.001,0.0010000000000000002".into());
        let text = WaveformWriter::new(WaveformFormat::Touchstone)
            .write_text(&dataset)
            .unwrap();
        let restored = read_touchstone_bytes("noise.ts", text.as_bytes()).unwrap();
        assert_eq!(restored.metadata["z0_ports"], dataset.metadata["z0_ports"]);
        dataset
            .metadata
            .insert("touchstone_version".into(), "1".into());
        assert!(
            WaveformWriter::new(WaveformFormat::Touchstone)
                .write_text(&dataset)
                .is_err()
        );
    }

    #[test]
    fn touchstone_noise_export_converts_thermal_normalization_and_v1_resistance() {
        let mut dataset =
            read_touchstone_bytes("noise.s2p", fixture(1, "", "1 0 0 0 0.2\n").as_bytes()).unwrap();
        dataset
            .metadata
            .insert("noise_reference_temperature_kelvin".into(), "580".into());
        dataset
            .signals
            .iter_mut()
            .find(|signal| signal.name == "Fmin")
            .unwrap()
            .data[0] = 2.0;
        for version in [1, 2] {
            let rows = noise_records_for_write(&dataset, &[1e6, 3e6], 2, 75.0, version).unwrap();
            assert!((rows[0][1] - 10.0 * 3.0_f64.log10()).abs() < 1e-14);
            assert_eq!(rows[0][4], if version == 1 { 0.4 } else { 30.0 });
        }
        assert_eq!(dataset.get_signal("Rn").unwrap().data, [15.0]);
        dataset
            .metadata
            .insert("noise_reference_temperature_kelvin".into(), "145".into());
        dataset
            .signals
            .iter_mut()
            .find(|signal| signal.name == "Fmin")
            .unwrap()
            .data[0] = 1.0 + f64::EPSILON;
        let rows = noise_records_for_write(&dataset, &[1e6, 3e6], 2, 75.0, 2).unwrap();
        let expected = (10.0 / std::f64::consts::LN_10) * (f64::EPSILON / 2.0);
        assert!((rows[0][1] - expected).abs() < expected * 1e-14);
    }

    #[test]
    fn touchstone_noise_rejects_incomplete_nonphysical_or_misaligned_data() {
        let underflow = fixture(1, "", "1 0 0 0 5e-324\n").replace("R 75", "R 1e-308");
        assert!(
            read_touchstone_bytes("underflow.s2p", underflow.as_bytes())
                .unwrap_err()
                .contains("underflows")
        );
        let valid = fixture(2, "", "1.5 3 0.5 -90 15\n2 0 1 0 15\n4 6 0 0 15\n");
        for bad in [
            valid.replace("Noise Frequencies] 3", "Noise Frequencies] 2"),
            valid.replace("[Noise Data]", "[Noise Data] 1"),
            valid.replace("[Noise Data]", "[Noise Data]\n[Noise Data]"),
            valid.replace("1.5 3 0.5 -90 15", "1.5 3 0.5 -90"),
            valid.replace("1.5 3 0.5 -90 15", "1.5 -3 0.5 -90 15"),
            valid.replace("1.5 3 0.5 -90 15", "1.5 3 1.5 -90 15"),
            valid.replace("1.5 3 0.5 -90 15", "1.5 3 0.5 -90 -15"),
            valid.replace("1.5 3 0.5 -90 15", "1.5 3 NaN -90 15"),
            valid.replace("1.5 3 0.5 -90 15", "5 3 0.5 -90 15"),
            valid.replace("2 0 1 0 15", "1.5 0 1 0 15"),
            valid.replace("[Noise Data]", "# Hz S RI R 50\n[Noise Data]"),
            valid.replace("[Noise Data]", "[Network Data]\n[Noise Data]"),
        ] {
            assert!(
                read_touchstone_bytes("bad.ts", bad.as_bytes()).is_err(),
                "accepted {bad}"
            );
        }
        let dataset = read_touchstone_bytes("noise.ts", valid.as_bytes()).unwrap();
        for name in ["Fmin", "Rn", "Sopt_RE", "Sopt_IM"] {
            let mut missing = dataset.clone();
            missing.signals.retain(|signal| signal.name != name);
            assert!(
                WaveformWriter::new(WaveformFormat::Touchstone)
                    .write_text(&missing)
                    .is_err()
            );
        }
        for temperature in ["0", "-1", "NaN", "inf", "bad"] {
            let mut bad = dataset.clone();
            bad.metadata.insert(
                "noise_reference_temperature_kelvin".into(),
                temperature.into(),
            );
            assert!(
                WaveformWriter::new(WaveformFormat::Touchstone)
                    .write_text(&bad)
                    .is_err()
            );
        }
        for (name, unit) in [("Rn", "mΩ"), ("Fmin", "dB"), ("Sopt_RE", "V")] {
            let mut bad = dataset.clone();
            bad.signals
                .iter_mut()
                .find(|signal| signal.name == name)
                .unwrap()
                .unit = unit.into();
            assert!(
                WaveformWriter::new(WaveformFormat::Touchstone)
                    .write_text(&bad)
                    .is_err()
            );
        }
    }
}
