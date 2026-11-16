//! The run's own connect-rule block.
//!
//! Verilog-AMS converts at every analog/digital boundary with the connect
//! module a `connectrules` block selects (LRM 2.4 section 7.7). A design may
//! declare blocks in its own sources, and RSpice ships rule sets; this is the
//! block a run states for itself: which shipped modules convert, and the
//! supply, thresholds, drive and edges each one is given. It is the run's one
//! store of those values. The deck carries it as a `.CONNECTRULES` card
//! beside the `.OPTIONS CONNECTRULES` that selects it, so the deck alone says
//! what converts.
//!
//! A block may start from a shipped rule set. What it takes from that set is
//! kept with it as values, each marked as the set's, so the deck states every
//! number it converts with and the run says which ones were set here.

use serde::{Deserialize, Serialize};

/// The shipped connect modules a run's block can select, in the order a
/// block states them: the three logic converters, then the three real ones.
pub const RUN_CONNECT_MODULES: [&str; 6] = ["a2d", "d2a", "bidir", "r2e", "e2r", "rbidir"];

/// A `connectrules` block the run owns.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct RunConnectRules {
    /// The block's name, which `.OPTIONS CONNECTRULES` names while the run
    /// converts with it.
    pub name: String,
    /// The shipped rule set the block was started from.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub base: Option<String>,
    /// One `connect` statement per shipped module the block selects.
    pub statements: Vec<RunConnectStatement>,
}

/// One `connect` statement of a [`RunConnectRules`] block.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct RunConnectStatement {
    /// The shipped module: one of [`RUN_CONNECT_MODULES`].
    pub module: String,
    /// What the converter converts against. `None` states no supply, and the
    /// deck's is taken (`.param vcc`, the rail that reaches the net, or
    /// 3.3 V).
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub supply: Option<RunConnectSupply>,
    /// Whether the supply is the base set's.
    #[serde(default, skip_serializing_if = "std::ops::Not::not")]
    pub supply_from_base: bool,
    /// Every other parameter the statement sets, in the order written.
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub parameters: Vec<RunConnectParameter>,
}

/// A converter's supply.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum RunConnectSupply {
    /// A fixed level, in volts.
    Volts(f64),
    /// A deck net whose solved voltage the converter tracks while it runs:
    /// the boundary is sensitive to that supply.
    Net(String),
}

/// One parameter a statement sets.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct RunConnectParameter {
    pub name: String,
    /// In volts, seconds, ohms, or a fraction of the supply (`fthi`,
    /// `ftlo`). `txdel` and `rz` may be infinite, which is stored as `"inf"`.
    #[serde(with = "real_or_inf")]
    pub value: f64,
    /// Whether the value is the base set's.
    #[serde(default, skip_serializing_if = "std::ops::Not::not")]
    pub from_base: bool,
}

impl RunConnectRules {
    /// The name a run's block takes unless it is given another.
    pub const DEFAULT_NAME: &'static str = "run_rules";

    /// The statement for `module`, if the block selects it.
    #[must_use]
    pub fn statement(&self, module: &str) -> Option<&RunConnectStatement> {
        self.statements
            .iter()
            .find(|statement| statement.module == module)
    }

    /// The deck's `.CONNECTRULES` card: one `+ connect` line per statement,
    /// in Verilog-AMS statement form.
    #[must_use]
    pub fn card(&self) -> String {
        let mut lines = vec![format!(".CONNECTRULES {}", self.name)];
        for statement in &self.statements {
            let mut overrides = Vec::with_capacity(statement.parameters.len() + 1);
            match &statement.supply {
                Some(RunConnectSupply::Volts(volts)) => {
                    overrides.push(format!(".vsup({})", verilog_real(*volts)));
                }
                Some(RunConnectSupply::Net(net)) => overrides.push(format!(".vsup({net})")),
                None => {}
            }
            for parameter in &statement.parameters {
                overrides.push(format!(
                    ".{}({})",
                    parameter.name,
                    verilog_real(parameter.value)
                ));
            }
            if overrides.is_empty() {
                lines.push(format!("+ connect {};", statement.module));
            } else {
                lines.push(format!(
                    "+ connect {} #({});",
                    statement.module,
                    overrides.join(", ")
                ));
            }
        }
        lines.join("\n")
    }

    /// The deck nets the block's converters track as their supply, as
    /// `connect d2a #(.vsup(vdrv))` reads `vdrv`, in statement order.
    pub fn supply_nets(&self) -> impl Iterator<Item = &str> {
        self.statements
            .iter()
            .filter_map(|statement| match &statement.supply {
                Some(RunConnectSupply::Net(net)) => Some(net.as_str()),
                Some(RunConnectSupply::Volts(_)) | None => None,
            })
    }

    /// What is wrong with the block, one sentence each.
    #[must_use]
    pub fn problems(&self) -> Vec<String> {
        let mut problems = Vec::new();
        if !super::model::is_simple_identifier(&self.name) {
            problems.push(format!(
                "the block name '{}' is not a Verilog-AMS identifier",
                self.name
            ));
        } else if self
            .name
            .get(..RESERVED_PREFIX.len())
            .is_some_and(|head| head.eq_ignore_ascii_case(RESERVED_PREFIX))
        {
            problems.push(format!(
                "the block name '{}' takes the {RESERVED_PREFIX} prefix RSpice reserves for its \
                 shipped rule sets",
                self.name
            ));
        }
        if self.statements.is_empty() {
            problems.push(format!("block '{}' selects no connect module", self.name));
        }
        for (index, statement) in self.statements.iter().enumerate() {
            let module = statement.module.as_str();
            if !RUN_CONNECT_MODULES.contains(&module) {
                problems.push(format!(
                    "'{module}' is not a shipped connect module ({})",
                    RUN_CONNECT_MODULES.join(", ")
                ));
            }
            if self.statements[..index]
                .iter()
                .any(|earlier| earlier.module == module)
            {
                problems.push(format!("{module} is selected twice"));
            }
            match &statement.supply {
                Some(RunConnectSupply::Volts(volts)) if !(volts.is_finite() && *volts > 0.0) => {
                    problems.push(format!(
                        "{module}: the supply {volts} V is not a positive level"
                    ));
                }
                Some(RunConnectSupply::Net(net)) if !super::model::is_simple_identifier(net) => {
                    problems.push(format!(
                        "{module}: the supply net '{net}' is not a net name"
                    ));
                }
                _ => {}
            }
            for (position, parameter) in statement.parameters.iter().enumerate() {
                let name = parameter.name.as_str();
                if !super::model::is_simple_identifier(name) || name.eq_ignore_ascii_case("vsup") {
                    problems.push(format!("{module}: '{name}' is not a parameter name"));
                }
                if statement.parameters[..position]
                    .iter()
                    .any(|earlier| earlier.name.eq_ignore_ascii_case(name))
                {
                    problems.push(format!("{module}: {name} is set twice"));
                }
                if parameter.value.is_nan() || parameter.value == f64::NEG_INFINITY {
                    problems.push(format!(
                        "{module}: {name} = {} is not a value",
                        parameter.value
                    ));
                }
            }
        }
        problems
    }
}

/// A real that may be `+inf`, which JSON has no number for: stored as the
/// string `"inf"`.
mod real_or_inf {
    use serde::{Deserialize, Deserializer, Serializer};

    pub(super) fn serialize<S: Serializer>(value: &f64, serializer: S) -> Result<S::Ok, S::Error> {
        if *value == f64::INFINITY {
            serializer.serialize_str("inf")
        } else {
            serializer.serialize_f64(*value)
        }
    }

    pub(super) fn deserialize<'de, D: Deserializer<'de>>(deserializer: D) -> Result<f64, D::Error> {
        #[derive(Deserialize)]
        #[serde(untagged)]
        enum Stored {
            Number(f64),
            Text(String),
        }
        match Stored::deserialize(deserializer)? {
            Stored::Number(value) => Ok(value),
            Stored::Text(text) if text == "inf" => Ok(f64::INFINITY),
            Stored::Text(text) => Err(serde::de::Error::custom(format!(
                "'{text}' is not a connect-rule value; a number or \"inf\" is"
            ))),
        }
    }
}

/// The prefix of every shipped rule set's name, which a run's block may not
/// take.
const RESERVED_PREFIX: &str = "rspice_";

/// A real as Verilog-AMS writes it: shortest round-trip digits, `inf` for an
/// unbounded value.
fn verilog_real(value: f64) -> String {
    if value == f64::INFINITY {
        "inf".to_owned()
    } else {
        format!("{value:?}")
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn parameter(name: &str, value: f64, from_base: bool) -> RunConnectParameter {
        RunConnectParameter {
            name: name.to_owned(),
            value,
            from_base,
        }
    }

    fn block() -> RunConnectRules {
        RunConnectRules {
            name: RunConnectRules::DEFAULT_NAME.to_owned(),
            base: Some("rspice_1v8_fast".to_owned()),
            statements: vec![
                RunConnectStatement {
                    module: "a2d".to_owned(),
                    supply: Some(RunConnectSupply::Volts(1.8)),
                    supply_from_base: true,
                    parameters: vec![
                        parameter("vthi", 1.2, false),
                        parameter("vtlo", 0.6, false),
                        parameter("txdel", f64::INFINITY, true),
                    ],
                },
                RunConnectStatement {
                    module: "d2a".to_owned(),
                    supply: Some(RunConnectSupply::Net("vdd".to_owned())),
                    supply_from_base: false,
                    parameters: vec![parameter("trise", 1e-10, true)],
                },
                RunConnectStatement {
                    module: "bidir".to_owned(),
                    supply: None,
                    supply_from_base: false,
                    parameters: Vec::new(),
                },
            ],
        }
    }

    #[test]
    fn the_card_states_every_value_in_verilog_ams_statement_form() {
        assert_eq!(
            block().card(),
            ".CONNECTRULES run_rules\n+ connect a2d #(.vsup(1.8), .vthi(1.2), .vtlo(0.6), \
             .txdel(inf));\n+ connect d2a #(.vsup(vdd), .trise(1e-10));\n+ connect bidir;"
        );
    }

    #[test]
    fn a_sound_block_has_no_problems_and_a_broken_one_names_each() {
        assert!(block().problems().is_empty(), "{:?}", block().problems());
        let mut broken = block();
        broken.name = "rspice_mine".to_owned();
        broken.statements[0]
            .parameters
            .push(parameter("vthi", 1.0, false));
        broken.statements[1].supply = Some(RunConnectSupply::Volts(-1.0));
        broken.statements[2].module = "a2d".to_owned();
        broken.statements[2]
            .parameters
            .push(parameter("vtlo", f64::NAN, false));
        let problems = broken.problems();
        for expected in [
            "reserves for its shipped rule sets",
            "vthi is set twice",
            "the supply -1 V is not a positive level",
            "a2d is selected twice",
            "vtlo = NaN is not a value",
        ] {
            assert!(
                problems.iter().any(|problem| problem.contains(expected)),
                "{expected}: {problems:?}"
            );
        }
    }

    #[test]
    fn the_block_round_trips_through_its_stored_form() {
        let stored = serde_json::to_string(&block()).expect("serializes");
        let read: RunConnectRules = serde_json::from_str(&stored).expect("reads back");
        assert_eq!(read, block());
    }
}
