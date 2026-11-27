//! What a prepared run sets the engine to, as its run records it.
//!
//! The temperatures and the steps come from the tasks the run dispatches,
//! each at the point it solves. The design variables and the options come
//! from the deck as the design or its author wrote it, before the files it
//! includes and the models bound to it are read in, so that a model
//! library's own parameters are not taken for the design's.

use super::*;
use rspice_results::provenance::AnalysisResultFamilyMember;
use rspice_results::run_facts::RunFacts;

/// The temperature SPICE solves at when nothing sets one: ngspice's
/// `REFTEMP`, which the engine keeps.
const NOMINAL_TEMPERATURE_C: f64 = 27.0;

/// The design variables and options `decks` state at their top level, read
/// in order, so a later deck's value for a name is the one that stands, as
/// the engine reads a later card. An option set to the value the product
/// states on every deck it writes is no difference, and the temperature is
/// a fact of its own.
pub(crate) fn stated_facts(decks: &[&str]) -> RunFacts {
    let mut facts = RunFacts::default();
    for deck in decks {
        read_stated(&mut facts, deck);
    }
    let defaults = {
        let mut defaults = RunFacts::default();
        read_stated(
            &mut defaults,
            &rspice_simulation_contract::options::SimulationOptions::default().to_spice_options(),
        );
        defaults.options
    };
    facts
        .options
        .retain(|option| option.name != "TEMP" && !defaults.contains(option));
    facts
}

/// Read the `.param` and `.options` cards `deck` states outside any
/// subcircuit, library section or control block onto `facts`.
fn read_stated(facts: &mut RunFacts, deck: &str) {
    // The keyword each open block is closed by, innermost last.
    let mut open: Vec<&'static str> = Vec::new();
    for (_, keyword, card) in crate::netlist_gen::directive_cards(deck) {
        let body = card
            .split_once(char::is_whitespace)
            .map_or("", |(_, body)| body);
        let closes = match keyword.as_str() {
            ".subckt" => Some(".ends"),
            ".control" => Some(".endc"),
            // `.lib name` opens a section; `.lib file name` reads one.
            ".lib" if body.split_whitespace().count() == 1 => Some(".endl"),
            _ => None,
        };
        if let Some(closes) = closes {
            open.push(closes);
            continue;
        }
        if open.last() == Some(&keyword.as_str()) {
            open.pop();
            continue;
        }
        if !open.is_empty() {
            continue;
        }
        match keyword.as_str() {
            ".param" | ".params" => {
                for (name, value) in assignments(body) {
                    // A function's definition is no variable.
                    if !value.is_empty() && !name.contains('(') {
                        facts.set_variable(&name, &readable(&value));
                    }
                }
            }
            ".option" | ".options" | ".opt" => {
                for (name, value) in assignments(body) {
                    facts.set_option(&name.to_ascii_uppercase(), &value);
                }
            }
            _ => {}
        }
    }
}

/// `value` as a reader writes it. A plan's design variable reaches the deck
/// as its value in seventeen-digit scientific notation, `2.00000000000000000e3`,
/// which is read back in SPICE's own notation, `2k`; any other value stands
/// as its deck spells it.
fn readable(value: &str) -> String {
    match value.parse::<f64>() {
        Ok(number) if format!("{number:.17e}") == value => {
            rspice_results::number_format::spice_coordinate(number, "")
        }
        _ => value.to_owned(),
    }
}

/// The `name=value` assignments of a card's body, in order; a word that
/// assigns nothing, an option switched on by its name alone, has an empty
/// value. An expression in braces, parentheses or quotes is one value, and
/// a comment ends the card.
fn assignments(body: &str) -> Vec<(String, String)> {
    let mut words: Vec<String> = Vec::new();
    let mut word = String::new();
    let mut depth = 0_i32;
    let mut quote = None;
    let flush = |word: &mut String, words: &mut Vec<String>| {
        if !word.is_empty() {
            words.push(std::mem::take(word));
        }
    };
    for character in body.chars() {
        if let Some(open) = quote {
            word.push(character);
            if character == open {
                quote = None;
            }
            continue;
        }
        match character {
            '\'' | '"' => {
                quote = Some(character);
                word.push(character);
            }
            '{' | '(' | '[' => {
                depth += 1;
                word.push(character);
            }
            '}' | ')' | ']' => {
                depth -= 1;
                word.push(character);
            }
            ';' if depth == 0 => break,
            '$' if depth == 0 && word.is_empty() => break,
            '=' if depth == 0 => {
                flush(&mut word, &mut words);
                words.push("=".to_owned());
            }
            ',' if depth == 0 => flush(&mut word, &mut words),
            character if character.is_whitespace() && depth == 0 => {
                flush(&mut word, &mut words);
            }
            character => word.push(character),
        }
    }
    flush(&mut word, &mut words);
    let mut assignments = Vec::new();
    let mut index = 0;
    while index < words.len() {
        let name = &words[index];
        if name == "=" {
            index += 1;
            continue;
        }
        match (words.get(index + 1), words.get(index + 2)) {
            (Some(equals), Some(value)) if equals == "=" && value != "=" => {
                assignments.push((name.clone(), value.clone()));
                index += 3;
            }
            _ => {
                assignments.push((name.clone(), String::new()));
                index += 1;
            }
        }
    }
    assignments
}

impl PreparedRunSnapshot {
    /// This snapshot with the facts its run records: `stated`, the design
    /// variables and options its deck states, with the temperature each task
    /// it dispatches solves at and what its points step. `deck_temperature`
    /// is the temperature its own deck sets, when it sets one, as the parse
    /// of it read it.
    #[must_use]
    pub(crate) fn with_facts(
        mut self,
        mut stated: RunFacts,
        deck_temperature: Option<f64>,
    ) -> Self {
        // A declaration whose points were expanded assembles their family
        // and reaches no engine, so it solves at no temperature of its own.
        let declarations: HashSet<AnalysisInstanceId> = self
            .tasks
            .iter()
            .filter(|task| task.declared_point.is_some())
            .map(|task| task.authored_instance_id)
            .collect();
        for task in &self.tasks {
            if declarations.contains(&task.instance_id) {
                continue;
            }
            let operating_point = match &task.task.config {
                Some(rspice_simulation_contract::config::AnalysisConfig::DcOp(config)) => {
                    Some(config.temperature_celsius)
                }
                _ => None,
            };
            stated.add_temperature(
                task.pvt_point
                    .as_ref()
                    .map(rspice_results::provenance::AnalysisResultPvtPoint::temperature_celsius)
                    .or_else(|| {
                        task.execution_environment
                            .as_ref()
                            .map(|environment| environment.temperature_celsius)
                    })
                    .or(operating_point)
                    .or(deck_temperature)
                    .unwrap_or(NOMINAL_TEMPERATURE_C),
            );
            if let Some(point) = &task.pvt_point {
                for swept in point.swept_values() {
                    stated.add_swept(&swept.name, &swept.value);
                }
            }
            if let Some(AnalysisResultFamilyMember::StepPoint { coordinate, .. }) =
                &task.family_member
            {
                // A stepped temperature is among the temperatures above.
                for axis in coordinate
                    .iter()
                    .filter(|axis| !axis.name.eq_ignore_ascii_case("temp"))
                {
                    stated.add_swept(
                        &axis.name,
                        &rspice_results::number_format::spice_coordinate(axis.value, &axis.unit),
                    );
                }
            }
        }
        self.facts = Some(stated);
        self
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn a_deck_states_its_top_level_variables_and_options_and_nothing_inside_a_block() {
        let facts = stated_facts(&[concat!(
            "* amp\n",
            ".param rload = 2k cl={2*1p} f(x)={x*2} ; the load\n",
            ".subckt buf a b\n.param rload=9k\n.options reltol=1\n.ends\n",
            ".lib tt\n.param vth0=0.4\n.endl\n",
            ".options reltol=1e-4\n+ abstol=1e-10 temp=85\n",
            ".param rload=3k\n",
            ".end\n",
        )]);
        let pairs = |settings: &[rspice_results::run_facts::RunSetting]| {
            settings
                .iter()
                .map(|setting| format!("{}={}", setting.name, setting.value))
                .collect::<Vec<_>>()
        };
        assert_eq!(pairs(&facts.variables), ["rload=3k", "cl={2*1p}"]);
        assert_eq!(pairs(&facts.options), ["RELTOL=1e-4", "ABSTOL=1e-10"]);
    }

    #[test]
    fn the_options_the_product_states_on_every_deck_are_no_difference() {
        let options = rspice_simulation_contract::options::SimulationOptions {
            reltol: 1e-4,
            ..Default::default()
        };
        let facts = stated_facts(&[&options.to_spice_options()]);
        assert_eq!(facts.options.len(), 1, "{:?}", facts.options);
        assert_eq!(facts.options[0].name, "RELTOL");
    }
}
