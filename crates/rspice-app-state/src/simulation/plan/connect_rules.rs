//! Editing the run's own connect-rule block, and reading the blocks a design
//! declares.
//!
//! The run's block ([`RunConnectRules`]) is stored with the plan's options and
//! written into the deck as a `.CONNECTRULES` card. It may start from a
//! shipped rule set: the set's values are copied in and marked as the set's,
//! and a value typed here replaces one and is marked as the run's. The Connect
//! rules page edits it one cell at a time through [`set_cell`].
//!
//! A design's own `connectrules` blocks live in its Verilog-AMS sources, or
//! in a model library bound to the plan, and are read here only to be
//! shown; the source is where they are edited.

use rspice_simulation_contract::options::{
    RunConnectParameter, RunConnectRules, RunConnectStatement, RunConnectSupply, format_si_value,
    parse_si_value,
};

/// The shipped set a new run block starts from unless another is selected.
pub const DEFAULT_BASE: &str = "rspice_builtin";

/// A column of the connect statement grid.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum ConnectRuleColumn {
    /// `vsup`: a level, a net the converter tracks live, or the deck's.
    Supply,
    /// `vthi` in volts or `fthi` as a fraction of the supply.
    ThresholdHigh,
    /// `vtlo` in volts or `ftlo` as a fraction of the supply.
    ThresholdLow,
    /// `rhi` for a logic driver, `rout` for a real one.
    DriveHigh,
    /// `rlo`.
    DriveLow,
    /// `trise`.
    Rise,
    /// `tfall`.
    Fall,
}

impl ConnectRuleColumn {
    pub const ALL: [Self; 7] = [
        Self::Supply,
        Self::ThresholdHigh,
        Self::ThresholdLow,
        Self::DriveHigh,
        Self::DriveLow,
        Self::Rise,
        Self::Fall,
    ];

    pub const fn label(self) -> &'static str {
        match self {
            Self::Supply => "Supply",
            Self::ThresholdHigh => "Vth high",
            Self::ThresholdLow => "Vth low",
            Self::DriveHigh => "R high",
            Self::DriveLow => "R low",
            Self::Rise => "Rise",
            Self::Fall => "Fall",
        }
    }

    /// The parameters this column states for `module`: the volt or ohm or
    /// second form first, then a threshold's fraction-of-supply form.
    fn parameters(self, module: &str) -> &'static [&'static str] {
        let logic_receiver = matches!(module, "a2d" | "bidir");
        let logic_driver = matches!(module, "d2a" | "bidir");
        let real_driver = matches!(module, "r2e" | "rbidir");
        match self {
            Self::Supply if matches!(module, "a2d" | "d2a" | "bidir") => &["vsup"],
            Self::ThresholdHigh if logic_receiver => &["vthi", "fthi"],
            Self::ThresholdLow if logic_receiver => &["vtlo", "ftlo"],
            Self::DriveHigh if logic_driver => &["rhi"],
            Self::DriveHigh if real_driver => &["rout"],
            Self::DriveLow if logic_driver => &["rlo"],
            Self::Rise if logic_driver || real_driver => &["trise"],
            Self::Fall if logic_driver || real_driver => &["tfall"],
            _ => &[],
        }
    }

    /// Whether `module` takes a value in this column.
    pub fn applies(self, module: &str) -> bool {
        !self.parameters(module).is_empty()
    }
}

/// What a shipped module converts, as the grid and the converter report
/// state it: between the analog side (`A`) and a logic (`D`) or real (`R`)
/// port.
pub fn conversion(module: &str) -> &'static str {
    match module {
        "a2d" => "A\u{2192}D",
        "d2a" => "D\u{2192}A",
        "bidir" => "A\u{2194}D",
        "r2e" => "R\u{2192}A",
        "e2r" => "A\u{2192}R",
        "rbidir" => "A\u{2194}R",
        _ => "",
    }
}

/// The statements a shipped rule set makes, every value marked as the set's.
///
/// The sets are compiled once per process: a page draws them every frame.
pub fn shipped_statements(set: &str) -> Option<Vec<RunConnectStatement>> {
    static SETS: std::sync::OnceLock<Vec<(String, Vec<RunConnectStatement>)>> =
        std::sync::OnceLock::new();
    let sets = SETS.get_or_init(|| {
        rspice_veriloga::connect::library::shipped_rule_sets()
            .into_iter()
            .filter_map(|(name, block)| {
                let statements = statements_of(&block, &name)?
                    .into_iter()
                    .map(|(mut statement, _)| {
                        statement.supply_from_base = statement.supply.is_some();
                        for parameter in &mut statement.parameters {
                            parameter.from_base = true;
                        }
                        statement
                    })
                    .collect();
                Some((name, statements))
            })
            .collect()
    });
    sets.iter()
        .find(|(name, _)| name == set)
        .map(|(_, statements)| statements.clone())
}

/// Every shipped rule set's name, in the order a list offers them. Listed
/// once per process: a page offers them every frame.
pub fn shipped_set_names() -> &'static [String] {
    static NAMES: std::sync::OnceLock<Vec<String>> = std::sync::OnceLock::new();
    NAMES.get_or_init(|| {
        rspice_veriloga::connect::library::shipped_rule_sets()
            .into_iter()
            .map(|(name, _)| name)
            .collect()
    })
}

/// The statements of block `name` in Verilog-AMS `source`, each with the
/// byte it starts at. `None` when the source does not compile.
fn statements_of(source: &str, name: &str) -> Option<Vec<(RunConnectStatement, usize)>> {
    let specification = rspice_veriloga::VerilogACompiler::default()
        .connect_specification_from_preprocessed(source)
        .ok()?;
    let table = specification.rules.select_block(name).ok()?;
    table
        .insertions()
        .iter()
        .map(|rule| {
            let numeric = rule.numeric_parameters().ok()?;
            let supply = rule
                .supply_net()
                .map(|net| RunConnectSupply::Net(net.to_string()))
                .or_else(|| {
                    numeric
                        .iter()
                        .find(|(parameter, _)| parameter.as_str() == "vsup")
                        .map(|(_, volts)| RunConnectSupply::Volts(*volts))
                });
            let parameters = numeric
                .into_iter()
                .filter(|(parameter, _)| parameter.as_str() != "vsup")
                .map(|(parameter, value)| RunConnectParameter {
                    name: parameter.to_string(),
                    value,
                    from_base: false,
                })
                .collect();
            Some((
                RunConnectStatement {
                    module: rule.connect_module.to_string(),
                    supply,
                    supply_from_base: false,
                    parameters,
                },
                rule.span.start as usize,
            ))
        })
        .collect()
}

/// A run block started from shipped set `base`.
pub fn started_from(base: &str) -> RunConnectRules {
    RunConnectRules {
        name: RunConnectRules::DEFAULT_NAME.to_owned(),
        base: Some(base.to_owned()),
        statements: shipped_statements(base).unwrap_or_default(),
    }
}

/// `block` started again from `base`: every value set here stays, and the
/// rest are `base`'s. A statement `base` does not make (a real converter)
/// keeps what was set here.
pub fn rebased(block: &RunConnectRules, base: &str) -> RunConnectRules {
    let mut next = started_from(base);
    next.name.clone_from(&block.name);
    for statement in &block.statements {
        let index = match next
            .statements
            .iter()
            .position(|target| target.module == statement.module)
        {
            Some(index) => index,
            None => {
                next.statements.push(RunConnectStatement {
                    module: statement.module.clone(),
                    supply: None,
                    supply_from_base: false,
                    parameters: Vec::new(),
                });
                next.statements.len() - 1
            }
        };
        let target = &mut next.statements[index];
        if !statement.supply_from_base && statement.supply.is_some() {
            target.supply.clone_from(&statement.supply);
            target.supply_from_base = false;
        }
        for parameter in statement
            .parameters
            .iter()
            .filter(|parameter| !parameter.from_base)
        {
            set_parameter(target, &parameter.name, parameter.value);
        }
    }
    next
}

/// Whether `block` selects `module`.
pub fn selects(block: &RunConnectRules, module: &str) -> bool {
    block.statement(module).is_some()
}

/// Select `module` in `block`, or stop selecting it.
pub fn set_selected(block: &mut RunConnectRules, module: &str, selected: bool) {
    if selected && !selects(block, module) {
        block.statements.push(RunConnectStatement {
            module: module.to_owned(),
            supply: None,
            supply_from_base: false,
            parameters: Vec::new(),
        });
    } else if !selected {
        block
            .statements
            .retain(|statement| statement.module != module);
    }
}

/// One cell of the grid as written: the value, and whether the base set
/// gave it. `None` when the statement states nothing there.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct CellText {
    pub text: String,
    pub from_base: bool,
}

/// The cell `column` of `statement`.
pub fn cell(statement: &RunConnectStatement, column: ConnectRuleColumn) -> Option<CellText> {
    if column == ConnectRuleColumn::Supply {
        return statement.supply.as_ref().map(|supply| CellText {
            text: match supply {
                RunConnectSupply::Volts(volts) => with_unit(*volts, "V"),
                RunConnectSupply::Net(net) => net.clone(),
            },
            from_base: statement.supply_from_base,
        });
    }
    let names = column.parameters(&statement.module);
    statement
        .parameters
        .iter()
        .find(|parameter| names.contains(&parameter.name.as_str()))
        .map(|parameter| CellText {
            text: value_text(&parameter.name, parameter.value),
            from_base: parameter.from_base,
        })
}

fn value_text(name: &str, value: f64) -> String {
    if value == f64::INFINITY {
        return "inf".to_owned();
    }
    match name {
        // A percentage rounded to nine places, so 0.7 reads as 70% rather
        // than as the binary product's last digits.
        "fthi" | "ftlo" => format!("{}%", format_si_value((value * 100.0 * 1e9).round() / 1e9)),
        "vthi" | "vtlo" => with_unit(value, "V"),
        "rhi" | "rlo" | "rout" => with_unit(value, "\u{3a9}"),
        _ => with_unit(value, "s"),
    }
}

/// `value` as the number, a space, then its SI prefix and `unit`: `1.2 V`,
/// `100 ps`, `40 \u{3a9}`.
fn with_unit(value: f64, unit: &str) -> String {
    let text = format_si_value(value);
    match text
        .find(|c: char| c.is_ascii_alphabetic())
        .filter(|at| !text[*at..].starts_with('e'))
    {
        Some(at) => format!("{} {}{unit}", &text[..at], &text[at..]),
        None => format!("{text} {unit}"),
    }
}

/// Set cell `column` of `module`'s statement from typed `text`.
///
/// Empty text takes the base set's value back, or states nothing where the
/// set has none. A supply is a level (`1.8`), a net the converter tracks
/// live (`vdd`), or `deck` for the deck's own. A threshold is volts, or a
/// fraction of the supply written with `%` (`70%`); writing one form drops
/// the other, since a statement may not give both. Everything else is a
/// number with an SI suffix (`100p`, `20`).
pub fn set_cell(
    block: &mut RunConnectRules,
    module: &str,
    column: ConnectRuleColumn,
    text: &str,
) -> Result<(), String> {
    let names = column.parameters(module);
    if names.is_empty() {
        return Err(format!("{module} takes no {}", column.label()));
    }
    let base = block
        .base
        .as_deref()
        .and_then(shipped_statements)
        .and_then(|statements| {
            statements
                .into_iter()
                .find(|statement| statement.module == module)
        });
    let statement = block
        .statements
        .iter_mut()
        .find(|statement| statement.module == module)
        .ok_or_else(|| format!("the block does not select {module}"))?;
    let text = text.trim();
    if column == ConnectRuleColumn::Supply {
        let (supply, from_base) = if text.is_empty() {
            let supply = base.as_ref().and_then(|base| base.supply.clone());
            let from_base = supply.is_some();
            (supply, from_base)
        } else if text.eq_ignore_ascii_case("deck") {
            (None, false)
        } else if let Ok(volts) = parse_volts(text) {
            if !(volts.is_finite() && volts > 0.0) {
                return Err(format!(
                    "a supply is a level above zero in volts, not '{text}'"
                ));
            }
            (Some(RunConnectSupply::Volts(volts)), false)
        } else if is_net_name(text) {
            (Some(RunConnectSupply::Net(text.to_owned())), false)
        } else {
            return Err(format!(
                "'{text}' is neither a level in volts nor a net name; write 1.8, vdd or deck"
            ));
        };
        statement.supply = supply;
        statement.supply_from_base = from_base;
        return Ok(());
    }
    if text.is_empty() {
        statement
            .parameters
            .retain(|parameter| !names.contains(&parameter.name.as_str()));
        if let Some(base) = &base {
            for parameter in base
                .parameters
                .iter()
                .filter(|parameter| names.contains(&parameter.name.as_str()))
            {
                statement.parameters.push(parameter.clone());
            }
        }
        return Ok(());
    }
    let label = column.label();
    let refused = || match column {
        ConnectRuleColumn::ThresholdHigh | ConnectRuleColumn::ThresholdLow => format!(
            "{label} is a level in volts (1.2 V) or a share of the supply (70%), not '{text}'"
        ),
        ConnectRuleColumn::DriveHigh | ConnectRuleColumn::DriveLow => {
            format!("{label} is a resistance above zero in ohms (50 \u{3a9}), not '{text}'")
        }
        _ => format!("{label} is a time of zero or more in seconds (100 ps), not '{text}'"),
    };
    let (name, value) = match names {
        [volts, fraction] => match text.strip_suffix('%') {
            Some(percent) => (
                *fraction,
                parse_number(percent.trim()).map_err(|_| refused())? / 100.0,
            ),
            None => (*volts, parse_volts(text).map_err(|_| refused())?),
        },
        [name] => (
            *name,
            parse_number(text.trim_end_matches(['s', '\u{3a9}']).trim()).map_err(|_| refused())?,
        ),
        _ => unreachable!("a column states one or two parameters"),
    };
    let in_range = match column {
        ConnectRuleColumn::DriveHigh | ConnectRuleColumn::DriveLow => value > 0.0,
        ConnectRuleColumn::Rise | ConnectRuleColumn::Fall => value >= 0.0,
        _ => true,
    };
    if !value.is_finite() || !in_range {
        return Err(refused());
    }
    statement
        .parameters
        .retain(|parameter| !names.contains(&parameter.name.as_str()));
    set_parameter(statement, name, value);
    Ok(())
}

/// State `name = value` in `statement` as the run's own, dropping the other
/// form of a threshold.
fn set_parameter(statement: &mut RunConnectStatement, name: &str, value: f64) {
    let alternative = match name {
        "vthi" => Some("fthi"),
        "fthi" => Some("vthi"),
        "vtlo" => Some("ftlo"),
        "ftlo" => Some("vtlo"),
        _ => None,
    };
    statement
        .parameters
        .retain(|parameter| parameter.name != name && Some(parameter.name.as_str()) != alternative);
    statement.parameters.push(RunConnectParameter {
        name: name.to_owned(),
        value,
        from_base: false,
    });
}

/// A number with an optional SI suffix, read with or without a space
/// before the suffix, as a cell shows it (`100 p`).
fn parse_number(text: &str) -> Result<f64, String> {
    let joined: String = text.split_whitespace().collect();
    parse_si_value(&joined).map_err(|_| format!("'{text}' is not a number"))
}

fn parse_volts(text: &str) -> Result<f64, String> {
    let trimmed = text.trim();
    let trimmed = trimmed
        .strip_suffix('V')
        .or_else(|| trimmed.strip_suffix('v'))
        .map_or(trimmed, str::trim_end);
    parse_number(trimmed)
}

fn is_net_name(text: &str) -> bool {
    let mut characters = text.chars();
    characters
        .next()
        .is_some_and(|first| first.is_ascii_alphabetic() || first == '_')
        && characters.all(|c| c.is_ascii_alphanumeric() || c == '_' || c == '$')
}

/// Where a declared `connectrules` block is written.
#[derive(Debug, Clone, PartialEq)]
pub enum DeclaredIn {
    /// One of the project's own sources, opened in its editor.
    Project(crate::state::ProjectSourceId),
    /// A model library bound to the plan, named as the plan binds it.
    Library(String),
}

/// A `connectrules` block the design's Verilog-AMS sources, or a model
/// library bound to the plan, declares.
#[derive(Debug, Clone, PartialEq)]
pub struct DeclaredConnectRules {
    pub name: String,
    pub origin: DeclaredIn,
    /// The file it is written in: a project source's logical path, or a
    /// library member's file name.
    pub logical_path: String,
    /// The `connectrules` line, 1-based.
    pub line: usize,
    /// Its statements, each with the line it starts on.
    pub statements: Vec<(RunConnectStatement, usize)>,
}

/// Every `connectrules` block the project's Verilog-AMS sources declare, in
/// source order, then every one the model libraries bound to the plan
/// declare, in binding order: the engine reads both when it builds the
/// run's decks. A source that does not compile on its own (one that includes
/// another, say) states its blocks to the engine only; its own editor
/// reports why.
pub fn declared_blocks(
    sources: &crate::state::ProjectSourceRegistry,
    libraries: &crate::state::ModelLibraryManager,
    bindings: &[rspice_model_library::SimulationPlanModelBinding],
) -> Vec<DeclaredConnectRules> {
    let mut blocks = Vec::new();
    for bundle in sources.iter_bundles() {
        let origin = DeclaredIn::Project(bundle.id());
        let root = bundle.root();
        blocks_in(root.content(), &origin, root.logical_path(), &mut blocks);
        for file in bundle.files() {
            blocks_in(file.content(), &origin, file.logical_path(), &mut blocks);
        }
    }
    for binding in bindings {
        let Some(library) = libraries.get_library(&binding.library_name) else {
            continue;
        };
        let origin = DeclaredIn::Library(binding.library_name.clone());
        for member in &library.source_contents {
            let Ok(content) = std::str::from_utf8(&member.bytes) else {
                continue;
            };
            let file = member.path.file_name().map_or_else(
                || member.path.to_string_lossy(),
                |name| name.to_string_lossy(),
            );
            blocks_in(content, &origin, &file, &mut blocks);
        }
    }
    blocks
}

/// Push the blocks `content` declares.
///
/// The source is read as the engine reads it, preprocessed: a file that
/// declares its own connect modules includes the standard headers for their
/// disciplines, and unexpanded it does not compile. Each line is the one in
/// `content` itself. An included header is written in place, so a block
/// stands its header's length further down the expanded text; the block's
/// own `connectrules` line says by how much, and its statements move with it.
fn blocks_in(
    content: &str,
    origin: &DeclaredIn,
    logical_path: &str,
    blocks: &mut Vec<DeclaredConnectRules>,
) {
    if !content.contains("connectrules") {
        return;
    }
    let Ok(expanded) =
        rspice_veriloga::preprocessor::Preprocessor::new().preprocess_source(content)
    else {
        return;
    };
    let Ok(specification) = rspice_veriloga::VerilogACompiler::default()
        .connect_specification_from_preprocessed(&expanded)
    else {
        return;
    };
    let line_of = |offset: usize| {
        expanded
            .get(..offset)
            .map_or(1, |head| head.matches('\n').count() + 1)
    };
    for block in specification.rules.blocks() {
        let expanded_line = line_of(block.span.start as usize);
        let line = declaration_line(content, &block.name).unwrap_or(expanded_line);
        let in_content = |offset: usize| (line_of(offset) + line).saturating_sub(expanded_line);
        blocks.push(DeclaredConnectRules {
            name: block.name.to_string(),
            origin: origin.clone(),
            logical_path: logical_path.to_owned(),
            line,
            statements: statements_of(&expanded, &block.name)
                .unwrap_or_default()
                .into_iter()
                .map(|(statement, offset)| (statement, in_content(offset)))
                .collect(),
        });
    }
}

/// The 1-based line of `content` that opens block `name`:
/// `connectrules name;`, as written.
fn declaration_line(content: &str, name: &str) -> Option<usize> {
    content
        .lines()
        .position(|line| {
            line.trim_start()
                .strip_prefix("connectrules")
                .filter(|rest| rest.starts_with(char::is_whitespace))
                .and_then(|rest| rest.trim_start().strip_prefix(name))
                .is_some_and(|rest| rest.trim_start().starts_with(';'))
        })
        .map(|index| index + 1)
}

#[cfg(test)]
mod tests {
    use super::*;

    /// A block in a source that includes the standard headers, as every file
    /// declaring its own connect modules does, is listed at its own line, its
    /// statements at theirs.
    #[test]
    fn a_block_beside_an_included_header_is_listed_at_its_own_lines() {
        let source = "`include \"disciplines.vams\"\nmodule moded(p, n);\n  inout p, n;\n  \
                      electrical p, n;\n  analog I(p, n) <+ V(p, n);\nendmodule\n\
                      connectrules Mine;\n  connect a2d #(.vsup(2.0));\nendconnectrules\n";
        let mut blocks = Vec::new();
        blocks_in(
            source,
            &DeclaredIn::Library("rules".to_owned()),
            "moded.va",
            &mut blocks,
        );
        let [block] = blocks.as_slice() else {
            panic!("one block: {blocks:#?}");
        };
        assert_eq!((block.name.as_str(), block.line), ("Mine", 7));
        let lines: Vec<(&str, usize)> = block
            .statements
            .iter()
            .map(|(statement, line)| (statement.module.as_str(), *line))
            .collect();
        assert_eq!(lines, [("a2d", 8)]);
    }

    #[test]
    fn a_block_started_from_a_shipped_set_carries_its_values_as_the_sets() {
        let block = started_from("rspice_1v8_fast");
        let modules: Vec<&str> = block
            .statements
            .iter()
            .map(|statement| statement.module.as_str())
            .collect();
        assert_eq!(modules, ["a2d", "d2a", "bidir"]);
        let a2d = block.statement("a2d").expect("a2d");
        assert_eq!(a2d.supply, Some(RunConnectSupply::Volts(1.8)));
        assert_eq!(
            cell(a2d, ConnectRuleColumn::ThresholdHigh),
            Some(CellText {
                text: "70%".to_owned(),
                from_base: true
            })
        );
        assert_eq!(
            cell(
                block.statement("d2a").expect("d2a"),
                ConnectRuleColumn::Rise
            )
            .map(|cell| cell.text),
            Some("100 ps".to_owned())
        );
    }

    #[test]
    fn a_typed_value_overrides_the_set_and_clearing_it_takes_the_set_back() {
        let mut block = started_from("rspice_3v3_fast");
        set_cell(&mut block, "a2d", ConnectRuleColumn::ThresholdHigh, "1.2").expect("volts");
        let a2d = block.statement("a2d").expect("a2d");
        assert_eq!(
            cell(a2d, ConnectRuleColumn::ThresholdHigh),
            Some(CellText {
                text: "1.2 V".to_owned(),
                from_base: false
            })
        );
        assert!(
            !a2d.parameters
                .iter()
                .any(|parameter| parameter.name == "fthi"),
            "a volt threshold drops the fraction"
        );
        set_cell(&mut block, "a2d", ConnectRuleColumn::Supply, "vdd").expect("a net");
        set_cell(&mut block, "d2a", ConnectRuleColumn::DriveHigh, "50").expect("ohms");
        assert_eq!(
            block.card(),
            ".CONNECTRULES run_rules\n+ connect a2d #(.vsup(vdd), .ftlo(0.3), .txdel(1e-9), \
             .vthi(1.2));\n+ connect d2a #(.vsup(3.3), .trise(1e-10), .tfall(1e-10), \
             .rhi(50.0));\n+ connect bidir #(.vsup(3.3), .ftlo(0.3), .fthi(0.7), .txdel(1e-9), \
             .trise(1e-10), .tfall(1e-10));"
        );

        set_cell(&mut block, "a2d", ConnectRuleColumn::ThresholdHigh, "").expect("clear");
        set_cell(&mut block, "a2d", ConnectRuleColumn::Supply, "").expect("clear");
        assert_eq!(
            block.statement("a2d"),
            started_from("rspice_3v3_fast")
                .statement("a2d")
                .map(|statement| {
                    let mut statement = statement.clone();
                    // Cleared values come back after the ones that stayed.
                    let fthi = statement.parameters.remove(1);
                    statement.parameters.push(fthi);
                    statement
                })
                .as_ref()
        );
        for (column, text) in [
            (ConnectRuleColumn::Supply, "-1"),
            (ConnectRuleColumn::Rise, "fast"),
            (ConnectRuleColumn::DriveHigh, "x"),
        ] {
            assert!(set_cell(&mut block, "d2a", column, text).is_err(), "{text}");
        }
        assert!(set_cell(&mut block, "a2d", ConnectRuleColumn::Rise, "1n").is_err());
    }

    #[test]
    fn rebasing_keeps_what_was_set_here() {
        let mut block = started_from("rspice_3v3_fast");
        set_cell(&mut block, "d2a", ConnectRuleColumn::DriveLow, "40").expect("ohms");
        set_selected(&mut block, "r2e", true);
        set_cell(&mut block, "r2e", ConnectRuleColumn::DriveHigh, "10").expect("rout");
        let moved = rebased(&block, "rspice_1v8_slow");
        assert_eq!(moved.base.as_deref(), Some("rspice_1v8_slow"));
        let d2a = moved.statement("d2a").expect("d2a");
        assert_eq!(d2a.supply, Some(RunConnectSupply::Volts(1.8)));
        assert_eq!(
            cell(d2a, ConnectRuleColumn::DriveLow).map(|cell| (cell.text, cell.from_base)),
            Some(("40 \u{3a9}".to_owned(), false))
        );
        assert_eq!(
            cell(d2a, ConnectRuleColumn::Rise).map(|cell| cell.text),
            Some("1 ns".to_owned())
        );
        assert!(selects(&moved, "r2e"));
        assert!(moved.problems().is_empty(), "{:?}", moved.problems());
    }

    /// A block a model library bound to the plan declares is listed, named
    /// by its library and file, since the engine reads it beside the
    /// design's own; an unbound library states nothing to the run.
    #[test]
    fn a_block_a_bound_library_declares_is_listed_with_its_library() {
        let mut rules = rspice_veriloga::connect::library::BUILTIN_CONNECT_MODULES
            .iter()
            .map(|(_, source)| *source)
            .collect::<String>();
        rules.push_str("\nconnectrules Low;\n  connect a2d #(.vsup(1.0));\nendconnectrules\n");
        let mut libraries = crate::state::ModelLibraryManager::new();
        let library = libraries
            .load_library_bundle(
                "connections.lib",
                vec![
                    (
                        "root.lib".to_owned(),
                        b".va \"rules.vams\" CONNECTIONS\n".to_vec(),
                    ),
                    ("rules.vams".to_owned(), rules.clone().into_bytes()),
                ],
                None,
            )
            .expect("the rules import");
        let binding = libraries
            .simulation_plan_binding(&library)
            .expect("the rules bind to the plan");
        let sources = crate::state::ProjectSourceRegistry::default();
        assert!(declared_blocks(&sources, &libraries, &[]).is_empty());
        let blocks = declared_blocks(&sources, &libraries, &[binding]);
        let [block] = blocks.as_slice() else {
            panic!("one block: {blocks:?}");
        };
        assert_eq!(block.name, "Low");
        assert_eq!(block.origin, DeclaredIn::Library(library));
        assert_eq!(block.logical_path, "rules.vams");
        let line = rules
            .lines()
            .position(|line| line.starts_with("connectrules Low"))
            .expect("the block's line")
            + 1;
        assert_eq!(block.line, line);
        let modules: Vec<&str> = block
            .statements
            .iter()
            .map(|(statement, _)| statement.module.as_str())
            .collect();
        assert_eq!(modules, ["a2d"]);
    }
}
