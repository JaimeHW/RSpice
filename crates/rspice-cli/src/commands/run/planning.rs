//! Admission for complete outer variants, corners, and canonical run axes.
use super::*;

pub(super) enum PreparedDeck {
    Single(Box<Netlist>),
    Corners(Vec<corners::PreparedCorner>),
}

impl PreparedDeck {
    pub fn prepare(source: &str, args: &RunArgs, config: &Config) -> Result<Self, CliError> {
        if args.corners.is_some() {
            Ok(Self::Corners(corners::prepare(source, args, config)?))
        } else {
            Ok(Self::Single(Box::new(load_netlist_from_source(
                source, args, config, false,
            )?)))
        }
    }

    pub fn preflight(
        &self,
        args: &RunArgs,
        config: &Config,
        multi_run: bool,
    ) -> Result<usize, CliError> {
        let check = |netlist: &Netlist, args: &RunArgs| {
            if multi_run
                && netlist
                    .options
                    .add_resistors
                    .as_ref()
                    .is_some_and(|policy| !policy.is_empty())
            {
                return Err(CliError::InvalidArgument {
                    message: ".PREPROCESS ADDRESISTORS is ambiguous in a multi-run .ALTER/.DATA/corner deck".into(),
                    suggestion: Some("run each expanded deck separately".into()),
                });
            }
            preflight_deck_run_count(netlist, args, config)
        };
        match self {
            Self::Single(netlist) => check(netlist, args),
            Self::Corners(corners) => {
                let mut count = 0usize;
                for corner in corners {
                    count = count.saturating_add(check(&corner.netlist, &corner.args)?);
                }
                Ok(count)
            }
        }
    }

    pub fn run(
        &self,
        args: &RunArgs,
        config: &Config,
        verbose: bool,
        quiet: bool,
        label: Option<&str>,
    ) -> Result<DeckOutcome, CliError> {
        match self {
            Self::Corners(corners) => corners::run(corners, args, config, quiet, label),
            Self::Single(netlist) => {
                if !quiet {
                    crate::commands::emit_netlist_diagnostics(netlist, false);
                }
                validate_pss_flag_conflict(netlist, args)?;
                validate_step_frontend_compatibility(netlist, args)?;
                let derived = materialize_addresistors_artifact(
                    netlist,
                    &args.input,
                    crate::commands::is_stdin(&args.input),
                    args.timeout,
                )?;
                let mut outcome = run_deck(netlist, args, config, verbose, quiet, label)?;
                if let Some(path) = derived {
                    outcome.outputs.push(path);
                }
                Ok(outcome)
            }
        }
    }
}
