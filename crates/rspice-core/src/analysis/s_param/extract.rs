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
