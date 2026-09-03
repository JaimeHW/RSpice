//! The `.SP` runner: one scattering sweep, and optionally its port noise.
//!
//! Every piece of this existed already — port collection, excitation, the
//! wave-to-scattering conversion, the port-noise covariance solve, the
//! two-port noise derivation — but no `Engine::run_*` method put them
//! together, so each frontend assembled the S-matrix itself. That is four
//! places deciding what a `.SP` card means, and it is why the browser API and
//! the engine adapter refused the family outright rather than guess.

use crate::abort_signal::{AbortSignal, NoAbort};
use crate::analysis::s_param::{
    ExtractError, PortNoiseAssembly, SMatrix, SParameterPort, SParameterResult,
    assemble_port_noise, collect_ports, extract_s_matrix_with_abort,
};
use crate::netlist::AnalysisCommand;
use crate::{Netlist, Value};

use super::{Engine, SimulationError};

/// One authored `.SP` card's complete typed result.
///
/// The scattering sweep and the port-noise sweep are separate published
/// documents with separate result families, so they are returned side by side
/// rather than folded into one.
#[derive(Debug, Clone)]
pub struct SParameterRun {
    /// Scattering parameters over the swept grid, in the exact shape the
    /// shared S-parameter document accepts.
    pub scattering: SParameterResult,
    /// The deck's declared ports, in port order, with the source names and
    /// reference-impedance realizations the sweep drove them through.
    pub ports: Vec<SParameterPort>,
    /// Port-noise evidence, present only when the card requested it.
    pub port_noise: Option<PortNoiseAssembly>,
}

impl Engine {
    /// Run one authored `.SP` card.
    ///
    /// The card supplies the frequency grid and whether port noise is
    /// requested; the deck's `portnum=` annotations supply the ports. Two-port
    /// noise parameters that are not physical are a typed failure, not a
    /// published placeholder: a noise figure that is present but meaningless
    /// will be believed.
    pub fn run_sp_with_abort(
        &self,
        netlist: &Netlist,
        card: &AnalysisCommand,
        abort: &dyn AbortSignal,
    ) -> Result<SParameterRun, SimulationError> {
        let AnalysisCommand::Sp {
            variation,
            points,
            start_freq,
            stop_freq,
            do_noise,
        } = card
        else {
            return Err(SimulationError::Netlist(
                "run_sp_with_abort was given a card that is not .SP".to_owned(),
            ));
        };
        let frequencies = card_frequency_grid(*variation, *points, *start_freq, *stop_freq, abort)?;
        self.run_sp_over_grid_with_abort(netlist, &frequencies, *do_noise, abort)
    }

    /// Non-abort form of [`Self::run_sp_with_abort`], for embedding with no
    /// cancellation source of its own.
    pub fn run_sp(
        &self,
        netlist: &Netlist,
        card: &AnalysisCommand,
    ) -> Result<SParameterRun, SimulationError> {
        self.run_sp_with_abort(netlist, card, &NoAbort)
    }

    /// Run a scattering sweep over an explicit frequency grid.
    ///
    /// The card-driven entry point resolves the grid and calls this; a caller
    /// that already has a grid — a `.LIN`-style Touchstone export, a study
    /// sweeping something else — uses it directly.
    pub fn run_sp_over_grid_with_abort(
        &self,
        netlist: &Netlist,
        frequencies: &[Value],
        do_noise: bool,
        abort: &dyn AbortSignal,
    ) -> Result<SParameterRun, SimulationError> {
        if abort.is_aborted() {
            return Err(SimulationError::Aborted);
        }
        if frequencies.is_empty() {
            return Err(SimulationError::Netlist(
                ".SP requires at least one sweep frequency".to_owned(),
            ));
        }
        self.ensure_analysis_points(frequencies.len())?;
        let ports = collect_ports(netlist).map_err(|error| {
            SimulationError::Netlist(format!(".SP port declarations are unusable: {error}"))
        })?;

        let cube = extract_s_matrix_with_abort(
            netlist,
            &ports,
            frequencies,
            |driven| {
                self.run_ac_with_abort(driven, frequencies, abort)
                    .map_err(|error| error.to_string())
            },
            abort,
        )
        .map_err(map_extract_error)?;
        if abort.is_aborted() {
            return Err(SimulationError::Aborted);
        }

        let count = ports.len();
        let reference_impedance = ports
            .first()
            .map(|port| port.z0)
            .ok_or_else(|| SimulationError::Netlist(".SP found no declared ports".to_owned()))?;
        let mut scattering = SParameterResult::new(
            reference_impedance,
            ports
                .iter()
                .map(|port| crate::analysis::s_param::Port {
                    number: port.number,
                    node_pos: port.node_pos.clone(),
                    node_neg: port.node_neg.clone(),
                    z0: port.z0,
                })
                .collect(),
        );
        for (index, frequency) in frequencies.iter().enumerate() {
            if index.is_multiple_of(16) && abort.is_aborted() {
                return Err(SimulationError::Aborted);
            }
            let mut matrix = SMatrix::new(*frequency, count);
            for row in 0..count {
                for column in 0..count {
                    let value = cube
                        .get(row)
                        .and_then(|entries| entries.get(column))
                        .and_then(|series| series.get(index))
                        .copied()
                        .ok_or_else(|| {
                            SimulationError::Circuit(format!(
                                "the extracted S-matrix has no entry S({},{}) at point {}",
                                row + 1,
                                column + 1,
                                index + 1
                            ))
                        })?;
                    matrix.set(row + 1, column + 1, value);
                }
            }
            scattering.add(matrix);
        }

        let port_noise = if do_noise {
            let temperature = netlist.options.temp.map_or(
                self.config().temperature,
                crate::constants::celsius_to_kelvin,
            );
            let sources = ports
                .iter()
                .map(|port| port.source_name.clone())
                .collect::<Vec<_>>();
            let points = self.run_port_noise_correlation_with_abort(
                netlist,
                &sources,
                frequencies,
                temperature,
                abort,
            )?;
            if points.len() != frequencies.len() {
                return Err(SimulationError::Circuit(format!(
                    "port-noise solve returned {} points for {} requested frequencies",
                    points.len(),
                    frequencies.len()
                )));
            }
            Some(
                assemble_port_noise(&ports, &scattering, points, temperature)
                    .map_err(|error| SimulationError::Circuit(error.to_string()))?,
            )
        } else {
            None
        };

        Ok(SParameterRun {
            scattering,
            ports,
            port_noise,
        })
    }
}

/// The frequency grid one authored card's sweep specification describes.
///
/// Cancellation during grid construction is a cancelled run, not a malformed
/// card, so the two failures stay distinguishable to the caller.
pub(crate) fn card_frequency_grid(
    variation: crate::netlist::FreqVariation,
    points: usize,
    start: Value,
    stop: Value,
    abort: &dyn AbortSignal,
) -> Result<Vec<Value>, SimulationError> {
    crate::analysis::ac::try_ac_sweep_frequencies_with_abort(variation, points, start, stop, abort)
        .map_err(|error| match error {
            crate::analysis::frequency_grid::FrequencyGridError::Aborted => {
                SimulationError::Aborted
            }
            other => SimulationError::Netlist(other.to_string()),
        })
}

fn map_extract_error(error: ExtractError) -> SimulationError {
    match error {
        ExtractError::Aborted => SimulationError::Aborted,
        other => SimulationError::Circuit(format!(".SP extraction failed: {other}")),
    }
}

#[cfg(test)]
mod tests {
    use crate::abort_signal::ImmediateAbort;
    use crate::engine::{Engine, SimulationConfig, SimulationError};
    use crate::netlist::{AnalysisCommand, Netlist};

    const TWO_PORT: &str = "Two-port attenuator\n\
         V1 p1 0 AC 1 portnum=1 z0=50\n\
         V2 p2 0 AC 0 portnum=2 z0=50\n\
         R1 p1 mid 25\n\
         R2 mid 0 50\n\
         R3 mid p2 25\n\
         .sp lin 3 1meg 3meg\n\
         .end\n";

    fn sp_card(netlist: &Netlist) -> AnalysisCommand {
        netlist
            .analyses
            .iter()
            .find(|command| matches!(command, AnalysisCommand::Sp { .. }))
            .expect("the deck authors a .SP card")
            .clone()
    }

    #[test]
    fn the_sp_runner_produces_a_full_scattering_sweep() {
        let netlist = Netlist::parse(TWO_PORT).expect("deck parses");
        let engine = Engine::new(SimulationConfig::default());
        let run = engine
            .run_sp(&netlist, &sp_card(&netlist))
            .expect(".SP runs");
        assert_eq!(run.scattering.num_ports, 2);
        assert_eq!(run.scattering.data.len(), 3);
        assert_eq!(run.ports.len(), 2);
        assert_eq!(run.scattering.frequencies(), vec![1.0e6, 2.0e6, 3.0e6]);
        for matrix in &run.scattering.data {
            for row in 1..=2 {
                for column in 1..=2 {
                    let value = matrix.get(row, column);
                    assert!(
                        value.re.is_finite() && value.im.is_finite(),
                        "S({row},{column}) at {} Hz is not finite",
                        matrix.frequency
                    );
                }
            }
        }
        assert!(run.port_noise.is_none(), "the card did not request noise");
    }

    #[test]
    fn a_resistive_pad_is_reciprocal_and_matched() {
        let netlist = Netlist::parse(TWO_PORT).expect("deck parses");
        let engine = Engine::new(SimulationConfig::default());
        let run = engine
            .run_sp(&netlist, &sp_card(&netlist))
            .expect(".SP runs");
        // A symmetric resistive pad is reciprocal (S21 == S12) and symmetric
        // (S11 == S22) at every frequency; both are properties of the network,
        // not of the extraction, so they check the runner end to end.
        for matrix in &run.scattering.data {
            assert!(
                (matrix.s21() - matrix.s12()).norm() < 1.0e-9,
                "a resistive pad must be reciprocal at {} Hz",
                matrix.frequency
            );
            assert!(
                (matrix.s11() - matrix.s22()).norm() < 1.0e-9,
                "a symmetric pad must have equal reflections at {} Hz",
                matrix.frequency
            );
        }
    }

    #[test]
    fn the_sp_runner_carries_port_noise_when_the_card_asks_for_it() {
        let netlist =
            Netlist::parse(&TWO_PORT.replace(".sp lin 3 1meg 3meg", ".sp lin 3 1meg 3meg 1"))
                .expect("deck parses");
        let engine = Engine::new(SimulationConfig::default());
        let run = engine
            .run_sp(&netlist, &sp_card(&netlist))
            .expect(".SP DONOISE runs");
        let sweep_points = run.scattering.data.len();
        let noise = run.port_noise.expect("the card requested port noise");
        assert_eq!(noise.points.len(), sweep_points);
        let two_port = noise
            .two_port
            .expect("a two-port network has noise figures");
        assert_eq!(two_port.len(), sweep_points);
        for parameters in two_port {
            assert!(parameters.valid, "an assembled figure is always physical");
            assert!(parameters.noise_factor >= 1.0 - 1.0e-9);
        }
    }

    #[test]
    fn a_deck_with_no_declared_ports_fails_before_any_solve() {
        let netlist = Netlist::parse(
            "No ports\n\
             V1 in 0 AC 1\n\
             R1 in 0 50\n\
             .sp lin 2 1meg 2meg\n\
             .end\n",
        )
        .expect("deck parses");
        let engine = Engine::new(SimulationConfig::default());
        let error = engine
            .run_sp(&netlist, &sp_card(&netlist))
            .expect_err("a deck with no portnum annotations must fail closed");
        assert!(
            error.to_string().contains("portnum"),
            "the failure names what the deck is missing: {error}"
        );
    }

    #[test]
    fn the_sp_runner_honours_its_abort_source() {
        let netlist = Netlist::parse(TWO_PORT).expect("deck parses");
        let engine = Engine::new(SimulationConfig::default());
        let error = engine
            .run_sp_with_abort(&netlist, &sp_card(&netlist), &ImmediateAbort)
            .expect_err("an aborted .SP must not produce a sweep");
        assert!(matches!(error, SimulationError::Aborted), "{error}");
    }

    #[test]
    fn a_card_that_is_not_sp_is_refused() {
        let netlist = Netlist::parse(TWO_PORT).expect("deck parses");
        let engine = Engine::new(SimulationConfig::default());
        engine
            .run_sp(&netlist, &AnalysisCommand::Op)
            .expect_err("only a .SP card selects the .SP runner");
    }
}
