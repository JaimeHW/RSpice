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

use crate::abort_signal::{AbortSignal, NoAbort};
use crate::analysis::ac::AcResult;
use crate::netlist::Netlist;
use crate::{Complex64, Value};

use super::network::{NetworkError, s_column_from_port_voltages};
use super::ports::{PortError, SParameterPort, normalize_ports, set_excitations};

/// Ports whose per-frequency projection runs between two abort polls.
///
/// The per-port AC solve is polled on both sides, but the projection loop
/// that reads every port voltage at every frequency is `ports × points` work
/// of this analysis's own and has to be polled too. Sixteen matches the
/// solver's `ABORT_CHECK_INTERVAL` so cancellation latency is bounded by the
/// same order of work everywhere.
const ABORT_CHECK_INTERVAL: usize = 16;

/// Why an S-matrix could not be extracted.
#[derive(Debug, Clone, PartialEq)]
pub enum ExtractError {
    /// The deck's port declarations could not be used.
    Port(PortError),
    /// The wave-to-scattering conversion rejected its inputs.
    Network(NetworkError),
    /// The caller's AC solve failed or was cancelled.
    AcSolve(String),
    /// Extraction observed a cancellation request of its own.
    ///
    /// Distinct from [`Self::AcSolve`]: this is the extraction's own port and
    /// projection loops stopping, not the caller's solve reporting a failure
    /// whose text happens to mention cancellation.
    Aborted,
    /// The AC solve returned a different number of points than were requested.
    ///
    /// Reported rather than truncated: a short sweep means the solve gave up
    /// part way, and quietly keeping the points it did return would present a
    /// partial measurement as a complete one.
    PointCount { returned: usize, requested: usize },
    /// A declared port node was absent from the AC result basis, or its named
    /// coordinate had no corresponding voltage value.
    MissingNodeVoltage {
        node: String,
        node_names: usize,
        voltages: usize,
    },
}

impl std::fmt::Display for ExtractError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Port(error) => error.fmt(f),
            Self::Network(error) => error.fmt(f),
            Self::AcSolve(message) => write!(f, "S-parameter AC solve failed: {message}"),
            Self::Aborted => write!(f, "S-parameter extraction was cancelled"),
            Self::PointCount {
                returned,
                requested,
            } => write!(
                f,
                "S-parameter AC solve returned {returned} point(s) for {requested} requested \
                frequencies"
            ),
            Self::MissingNodeVoltage {
                node,
                node_names,
                voltages,
            } => write!(
                f,
                "S-parameter port node '{node}' is unavailable in the AC result \
                 ({node_names} node name(s), {voltages} voltage value(s))"
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

/// Complex voltage at one node.
///
/// A netlist's reference node never appears in the solution vector, so looking
/// it up has to succeed at zero. A declared non-ground port node, however, is
/// part of the measurement basis; treating a missing coordinate as zero would
/// manufacture a reflection or transmission result.
fn node_voltage(point: &AcResult, node: &str) -> Result<Complex64, ExtractError> {
    if node == "0" || node.eq_ignore_ascii_case("gnd") {
        return Ok(Complex64::new(0.0, 0.0));
    }
    let value = point
        .node_names
        .iter()
        .position(|name| name.eq_ignore_ascii_case(node))
        .and_then(|index| point.voltages.get(index).copied())
        .ok_or_else(|| ExtractError::MissingNodeVoltage {
            node: node.to_string(),
            node_names: point.node_names.len(),
            voltages: point.voltages.len(),
        })?;
    Ok(value)
}

/// Extract the full scattering matrix through a non-cancellable
/// compatibility path.
///
/// First-party surfaces call [`extract_s_matrix_with_abort`] instead; this
/// remains for third-party embedding that has no abort source to offer.
pub fn extract_s_matrix<F>(
    netlist: &Netlist,
    ports: &[SParameterPort],
    frequencies: &[Value],
    run_ac: F,
) -> Result<Vec<Vec<Vec<Complex64>>>, ExtractError>
where
    F: FnMut(&Netlist) -> Result<Vec<AcResult>, String>,
{
    extract_s_matrix_with_abort(netlist, ports, frequencies, run_ac, &NoAbort)
}

/// Extract the full scattering matrix, indexed `[row][column][frequency]`,
/// observing cooperative cancellation throughout.
///
/// `run_ac` receives a netlist with exactly one port driven and every other
/// source silenced, and must return one [`AcResult`] per requested frequency in
/// order. Returning an `Err` aborts the extraction, so a cancelled run reports
/// as a cancelled run rather than as a bad measurement.
///
/// `abort` is polled before and after every port's solve and on a fixed stride
/// through the projection that reads port voltages back, so an N-port sweep
/// cannot sit uncancellable between two AC solves. It is the extraction's own
/// bound; the caller's `run_ac` closure remains responsible for cancelling the
/// solve it runs.
pub fn extract_s_matrix_with_abort<F>(
    netlist: &Netlist,
    ports: &[SParameterPort],
    frequencies: &[Value],
    mut run_ac: F,
    abort: &dyn AbortSignal,
) -> Result<Vec<Vec<Vec<Complex64>>>, ExtractError>
where
    F: FnMut(&Netlist) -> Result<Vec<AcResult>, String>,
{
    let count = ports.len();
    let points = frequencies.len();

    check_abort(abort)?;

    // Normalize once, on this analysis's own copy. Every excitation then starts
    // from the same circuit, so a port cannot be given two reference impedances
    // by being normalized once per sweep step.
    let mut base = netlist.clone();
    let ports = normalize_ports(&mut base, ports)?;
    let reference_impedances = ports.iter().map(|port| port.z0).collect::<Vec<_>>();

    let zero = Complex64::new(0.0, 0.0);
    let mut s = vec![vec![vec![zero; points]; count]; count];

    for excited in 0..count {
        check_abort(abort)?;
        let mut driven = base.clone();
        set_excitations(&mut driven, &ports, excited)?;
        let solved = run_ac(&driven).map_err(ExtractError::AcSolve)?;
        check_abort(abort)?;
        if solved.len() != points {
            return Err(ExtractError::PointCount {
                returned: solved.len(),
                requested: points,
            });
        }

        for (index, point) in solved.iter().enumerate() {
            if index.is_multiple_of(ABORT_CHECK_INTERVAL) {
                check_abort(abort)?;
            }
            let voltages = ports
                .iter()
                .map(|port| {
                    Ok(node_voltage(point, &port.node_pos)? - node_voltage(point, &port.node_neg)?)
                })
                .collect::<Result<Vec<_>, ExtractError>>()?;
            let column = s_column_from_port_voltages(&voltages, excited, &reference_impedances)?;
            for (row, value) in column.into_iter().enumerate() {
                s[row][excited][index] = value;
            }
        }
    }

    Ok(s)
}

fn check_abort(abort: &dyn AbortSignal) -> Result<(), ExtractError> {
    if abort.is_aborted() {
        Err(ExtractError::Aborted)
    } else {
        Ok(())
    }
}
