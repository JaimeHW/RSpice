//! One S-matrix extraction, shared by every front-end.
//!
//! Driving each port in turn and reading the response is identical work whether
//! the request came from the CLI, the Python bindings, or the desktop runner,
//! and it used to exist as three hand-rolled copies of the same sweep. They had
//! already drifted: two discovered ports from the deck while the third injected
//! its own ideal sources at nodes named in a dialog — which would have shorted
//! out the reference impedance of any port that was already a real one.
//!
//! The AC solve itself stays with the caller, as a closure. Each front-end owns
//! its own engine, its own cancellation, and its own error type, and none of
//! that belongs in here.

use crate::analysis::ac::AcResult;
use crate::netlist::Netlist;
use crate::{Complex64, Value};

use super::network::{NetworkError, s_column_from_port_voltages};
use super::ports::{PortError, SParameterPort, normalize_ports, set_excitations};

/// Why an S-matrix could not be extracted.
#[derive(Debug, Clone, PartialEq)]
pub enum ExtractError {
    /// The deck's port declarations could not be used.
    Port(PortError),
    /// The wave-to-scattering conversion rejected its inputs.
    Network(NetworkError),
    /// The caller's AC solve failed or was cancelled.
    AcSolve(String),
    /// The AC solve returned a different number of points than were requested.
    ///
    /// Reported rather than truncated: a short sweep means the solve gave up
    /// part way, and quietly keeping the points it did return would present a
    /// partial measurement as a complete one.
    PointCount { returned: usize, requested: usize },
}

impl std::fmt::Display for ExtractError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Port(error) => error.fmt(f),
            Self::Network(error) => error.fmt(f),
            Self::AcSolve(message) => write!(f, "S-parameter AC solve failed: {message}"),
            Self::PointCount {
                returned,
                requested,
            } => write!(
                f,
                "S-parameter AC solve returned {returned} point(s) for {requested} requested \
                 frequencies"
            ),
        }
    }
}

impl std::error::Error for ExtractError {}

impl From<PortError> for ExtractError {
    fn from(error: PortError) -> Self {
        Self::Port(error)
    }
}

impl From<NetworkError> for ExtractError {
    fn from(error: NetworkError) -> Self {
        Self::Network(error)
    }
}

/// Complex voltage at one node, with ground and unknown names reading as zero.
///
/// A netlist's reference node never appears in the solution vector, so looking
/// it up has to succeed at zero rather than fail. An unknown name reads zero for
/// the same reason it does everywhere else in SPICE: a node no element connects
/// to carries no signal.
fn node_voltage(point: &AcResult, node: &str) -> Complex64 {
    if node == "0" || node.eq_ignore_ascii_case("gnd") {
        return Complex64::new(0.0, 0.0);
    }
    point
        .node_names
        .iter()
        .position(|name| name.eq_ignore_ascii_case(node))
        .and_then(|index| point.voltages.get(index).copied())
        .unwrap_or_else(|| Complex64::new(0.0, 0.0))
}

/// Extract the full scattering matrix, indexed `[row][column][frequency]`.
///
/// `run_ac` receives a netlist with exactly one port driven and every other
/// source silenced, and must return one [`AcResult`] per requested frequency in
/// order. Returning an `Err` aborts the extraction, so a cancelled run reports
/// as a cancelled run rather than as a bad measurement.
pub fn extract_s_matrix<F>(
    netlist: &Netlist,
    ports: &[SParameterPort],
    frequencies: &[Value],
    mut run_ac: F,
) -> Result<Vec<Vec<Vec<Complex64>>>, ExtractError>
where
    F: FnMut(&Netlist) -> Result<Vec<AcResult>, String>,
{
    let count = ports.len();
    let points = frequencies.len();

    // Normalize once, on this analysis's own copy. Every excitation then starts
    // from the same circuit, so a port cannot be given two reference impedances
    // by being normalized once per sweep step.
    let mut base = netlist.clone();
    let ports = normalize_ports(&mut base, ports)?;
    let reference_impedances = ports.iter().map(|port| port.z0).collect::<Vec<_>>();

    let zero = Complex64::new(0.0, 0.0);
    let mut s = vec![vec![vec![zero; points]; count]; count];

    for excited in 0..count {
        let mut driven = base.clone();
        set_excitations(&mut driven, &ports, excited)?;
        let solved = run_ac(&driven).map_err(ExtractError::AcSolve)?;
        if solved.len() != points {
            return Err(ExtractError::PointCount {
                returned: solved.len(),
                requested: points,
            });
        }

        for (index, point) in solved.iter().enumerate() {
            let voltages = ports
                .iter()
                .map(|port| {
                    node_voltage(point, &port.node_pos) - node_voltage(point, &port.node_neg)
                })
                .collect::<Vec<_>>();
            let column = s_column_from_port_voltages(&voltages, excited, &reference_impedances)?;
            for (row, value) in column.into_iter().enumerate() {
                s[row][excited][index] = value;
            }
        }
    }

    Ok(s)
}

#[cfg(test)]
mod tests {
    use super::super::ports::collect_ports;
    use super::*;

    /// A resistive divider has an S-matrix anyone can write down, so it settles
    /// whether the whole path — normalization, excitation, solve, extraction —
    /// agrees with the physics rather than merely with itself.
    ///
    /// The AC solve here is the real engine, not a stub.
    fn s_at_dc(deck: &str) -> Vec<Vec<Complex64>> {
        let netlist = Netlist::parse(deck).expect("deck parses");
        let ports = collect_ports(&netlist).expect("ports collect");
        let engine = crate::engine::Engine::new(crate::config::SimulationConfig::default());
        let frequencies = vec![1.0];
        let s = extract_s_matrix(&netlist, &ports, &frequencies, |driven| {
            engine
                .run_ac(driven, &frequencies)
                .map_err(|error| error.to_string())
        })
        .expect("extraction succeeds");
        (0..ports.len())
            .map(|row| (0..ports.len()).map(|col| s[row][col][0]).collect())
            .collect()
    }

    /// `S11 = (R + Z2 - Z1)/(Z1 + R + Z2)`, `S21 = 2 sqrt(Z1 Z2)/(Z1 + R + Z2)`.
    fn assert_series_resistor(s: &[Vec<Complex64>], r: Value, z1: Value, z2: Value) {
        let total = z1 + r + z2;
        let expected = [
            [(r + z2 - z1) / total, 2.0 * (z1 * z2).sqrt() / total],
            [2.0 * (z1 * z2).sqrt() / total, (r + z1 - z2) / total],
        ];
        for (row, values) in expected.iter().enumerate() {
            for (col, value) in values.iter().enumerate() {
                assert!(
                    (s[row][col].re - value).abs() < 1e-9 && s[row][col].im.abs() < 1e-9,
                    "S[{row}][{col}] = {}, expected {value}",
                    s[row][col]
                );
            }
        }
    }

    #[test]
    fn xyce_port_elements_produce_the_closed_form_s_matrix() {
        let s = s_at_dc(
            "* series resistor between two P ports\n\
             P1 p1 0 PORT=1 Z0=75 AC 1\n\
             R1 p1 p2 50\n\
             P2 p2 0 PORT=2 Z0=50\n\
             .ac lin 1 1 1\n\
             .end\n",
        );
        assert_series_resistor(&s, 50.0, 75.0, 50.0);
    }

    /// The same network declared the ngspice way must measure the same, or the
    /// two front-end spellings describe different circuits.
    #[test]
    fn annotated_sources_produce_the_same_s_matrix_as_port_elements() {
        let s = s_at_dc(
            "* series resistor between two annotated ports\n\
             V1 p1 0 DC 0 AC 1 portnum=1 z0=75\n\
             R1 p1 p2 50\n\
             V2 p2 0 DC 0 AC 0 portnum=2 z0=50\n\
             .ac lin 1 1 1\n\
             .end\n",
        );
        assert_series_resistor(&s, 50.0, 75.0, 50.0);
    }
}
