//! C10 — digital-heavy (design §3 C10, amendment A5).
//!
//! The ISCAS-85 expectations are the Boolean function of the circuit's
//! gate-level rendering (`net/<c>.v`, Verilog primitives) evaluated on the
//! inputs the deck applies: every input low before the 45 ps strobe and high
//! before the 985 ps strobe in the 1 ns runs, and the random vectors of
//! `net/<c>_vectors.cir` (64; c6288: 8) 8 ns into each 10 ns vector, as many
//! as the quantity's window holds. The LFSR is the
//! right-shifting Fibonacci register with taps 16, 14, 13, 11 from 16'hACE1;
//! the counter counts clock rises; both feed a 16-bit R-2R ladder at 1 V,
//! which the decks sample 80 ns after each clock rise.

use std::collections::HashMap;

use super::{Context, Registry, values};
use crate::suites::mixed::compare::Expected;

pub const REGISTRY: Registry = &[
    ("vec", vec),
    ("inputs", inputs),
    ("seq", seq),
    ("count", count),
    ("vdac_lfsr", vdac_lfsr),
    ("vdac_count", vdac_count),
];

const LFSR_SEED: u16 = 0xACE1;
const ISCAS_STROBES: [f64; 2] = [45e-12, 985e-12];
const VECTOR_PERIOD: f64 = 10e-9;
const VECTOR_STROBE: f64 = 8e-9;
/// The most vectors a `net/<c>_vectors.cir` holds.
const VECTORS: usize = 64;
/// One ladder LSB is 1 V / 2^16.
const DAC_FULL_SCALE: f64 = 65536.0;

fn design_net(file: &str) -> std::path::PathBuf {
    crate::suites::mixed::corpus_root()
        .join("c10-digital")
        .join("net")
        .join(file)
}

/// The ISCAS circuit a variant renders (`c432-xs` → `c432`).
fn circuit(variant: &str) -> Result<&str, String> {
    variant
        .split_once('-')
        .map(|(circuit, _)| circuit)
        .filter(|circuit| circuit.starts_with('c'))
        .ok_or_else(|| format!("c10: variant {variant} is not an ISCAS rendering"))
}

/// A gate-level netlist: ports in order, and each driven net's primitive.
struct Netlist {
    inputs: Vec<String>,
    outputs: Vec<String>,
    gates: HashMap<String, (String, Vec<String>)>,
}

fn parse_netlist(circuit: &str) -> Result<Netlist, String> {
    let path = design_net(&format!("{circuit}.v"));
    let text =
        std::fs::read_to_string(&path).map_err(|error| format!("{}: {error}", path.display()))?;
    let mut statements = Vec::new();
    let mut current = String::new();
    for line in text.lines() {
        let line = line.split("//").next().unwrap_or_default();
        current.push(' ');
        current.push_str(line);
        while let Some(end) = current.find(';') {
            statements.push(current[..end].trim().to_owned());
            current = current[end + 1..].to_owned();
        }
    }
    let names = |list: &str| -> Vec<String> {
        list.split(',')
            .map(|name| name.trim().to_owned())
            .filter(|name| !name.is_empty())
            .collect()
    };
    let mut netlist = Netlist {
        inputs: Vec::new(),
        outputs: Vec::new(),
        gates: HashMap::new(),
    };
    for statement in statements {
        let (head, rest) = statement
            .split_once(char::is_whitespace)
            .unwrap_or((statement.as_str(), ""));
        match head {
            "input" => netlist.inputs.extend(names(rest)),
            "output" => netlist.outputs.extend(names(rest)),
            "not" | "buf" | "and" | "nand" | "or" | "nor" | "xor" | "xnor" => {
                let open = rest.find('(').ok_or("gate without terminals")?;
                let close = rest.rfind(')').ok_or("gate without terminals")?;
                let terminals = names(&rest[open + 1..close]);
                let (output, inputs) = terminals.split_first().ok_or("gate without output")?;
                netlist
                    .gates
                    .insert(output.clone(), (head.to_owned(), inputs.to_vec()));
            }
            _ => {}
        }
    }
    if netlist.inputs.is_empty() || netlist.outputs.is_empty() {
        return Err(format!("{}: no ports parsed", path.display()));
    }
    Ok(netlist)
}

fn evaluate(
    netlist: &Netlist,
    net: &str,
    values: &mut HashMap<String, bool>,
) -> Result<bool, String> {
    if let Some(value) = values.get(net) {
        return Ok(*value);
    }
    let (kind, operands) = netlist
        .gates
        .get(net)
        .ok_or_else(|| format!("net {net} has no driver"))?;
    let mut bits = Vec::with_capacity(operands.len());
    for operand in operands {
        bits.push(evaluate(netlist, operand, values)?);
    }
    let all = bits.iter().all(|bit| *bit);
    let any = bits.iter().any(|bit| *bit);
    let parity = bits.iter().filter(|bit| **bit).count() % 2 == 1;
    let value = match kind.as_str() {
        "not" => !bits[0],
        "buf" => bits[0],
        "and" => all,
        "nand" => !all,
        "or" => any,
        "nor" => !any,
        "xor" => parity,
        "xnor" => !parity,
        other => return Err(format!("unknown primitive {other}")),
    };
    values.insert(net.to_owned(), value);
    Ok(value)
}

/// The input vectors the strobes in `window` read, by analysis: the 1 ns
/// runs' all-low and all-high inputs, or the random vectors.
fn applied_vectors(
    netlist: &Netlist,
    circuit: &str,
    analysis: &str,
    window: [f64; 2],
) -> Result<Vec<HashMap<String, bool>>, String> {
    let inside = |time: f64| time >= window[0] && time <= window[1];
    if analysis.starts_with("tran-vec") {
        let table = vector_table(circuit)?;
        return (0..VECTORS)
            .filter(|k| inside(*k as f64 * VECTOR_PERIOD + VECTOR_STROBE))
            .map(|k| {
                netlist
                    .inputs
                    .iter()
                    .map(|input| {
                        let level = table
                            .get(input)
                            .and_then(|levels| levels.get(k))
                            .ok_or_else(|| format!("{circuit}: no vector {k} for input {input}"))?;
                        Ok((input.clone(), *level))
                    })
                    .collect()
            })
            .collect();
    }
    Ok(ISCAS_STROBES
        .iter()
        .enumerate()
        .filter(|(_, time)| inside(**time))
        .map(|(index, _)| {
            netlist
                .inputs
                .iter()
                .map(|input| (input.clone(), index == 1))
                .collect()
        })
        .collect())
}

/// Each input's level per vector, read from the PWL sources of
/// `net/<c>_vectors.cir` at 8 ns into each vector.
fn vector_table(circuit: &str) -> Result<HashMap<String, Vec<bool>>, String> {
    let path = design_net(&format!("{circuit}_vectors.cir"));
    let text =
        std::fs::read_to_string(&path).map_err(|error| format!("{}: {error}", path.display()))?;
    let mut logical: Vec<String> = Vec::new();
    for line in text.lines() {
        if line.starts_with('*') {
            continue;
        }
        if let Some(rest) = line.strip_prefix('+') {
            if let Some(last) = logical.last_mut() {
                last.push(' ');
                last.push_str(rest);
            }
        } else if !line.trim().is_empty() {
            logical.push(line.to_owned());
        }
    }
    let mut table = HashMap::new();
    for line in logical {
        let open = line.find("pwl(").ok_or("vector source without pwl")?;
        let node = line
            .split_whitespace()
            .nth(1)
            .ok_or("vector source without a node")?
            .to_owned();
        let tokens: Vec<&str> = line[open + 4..]
            .trim_end()
            .trim_end_matches(')')
            .split_whitespace()
            .collect();
        let mut points = Vec::new();
        for pair in tokens.chunks(2) {
            let [time, level] = pair else {
                return Err(format!("{node}: odd PWL"));
            };
            let time = time.strip_suffix('n').map_or_else(
                || time.parse::<f64>(),
                |nanoseconds| nanoseconds.parse::<f64>().map(|value| value * 1e-9),
            );
            let time = time.map_err(|_| format!("{node}: bad PWL time"))?;
            let level: f64 = level
                .parse()
                .map_err(|_| format!("{node}: bad PWL level"))?;
            points.push((time, level));
        }
        let levels = (0..VECTORS)
            .map(|k| {
                let at = k as f64 * VECTOR_PERIOD + VECTOR_STROBE;
                points
                    .iter()
                    .take_while(|(time, _)| *time <= at)
                    .last()
                    .map(|(_, level)| *level > 0.5)
                    .unwrap_or(false)
            })
            .collect();
        table.insert(node, levels);
    }
    Ok(table)
}

/// The quantity's nets with the variant's spelling stripped back to port
/// names (`d_g426`, `xdut.g426__event_0` → `g426`).
fn port_names(context: &Context<'_>) -> Result<Vec<String>, String> {
    let nets = context
        .quantity
        .measure
        .nets
        .as_ref()
        .and_then(|nets| nets.for_variant(context.variant))
        .ok_or("c10: the quantity names no nets for this variant")?;
    Ok(nets
        .into_iter()
        .map(|net| {
            let net = net.to_ascii_lowercase();
            let net = net.strip_prefix("xdut.").unwrap_or(&net).to_owned();
            let net = net.strip_suffix("__event_0").unwrap_or(&net).to_owned();
            net.strip_prefix("d_").unwrap_or(&net).to_owned()
        })
        .collect())
}

fn word(bits: impl IntoIterator<Item = bool>) -> f64 {
    bits.into_iter()
        .fold(0u64, |code, bit| (code << 1) | u64::from(bit)) as f64
}

/// The outputs the netlist computes from each applied vector, as the codes
/// the quantity reads (its nets, MSB first).
fn vec(context: &Context<'_>) -> Result<Expected, String> {
    let circuit = circuit(context.variant)?;
    let netlist = parse_netlist(circuit)?;
    let outputs = port_names(context)?;
    let window = context
        .quantity
        .measure
        .window
        .unwrap_or([0.0, f64::INFINITY]);
    let mut codes = Vec::new();
    for vector in applied_vectors(&netlist, circuit, &context.quantity.analysis, window)? {
        let mut known = vector.clone();
        let mut bits = Vec::with_capacity(outputs.len());
        for output in &outputs {
            bits.push(evaluate(&netlist, output, &mut known)?);
        }
        codes.push(word(bits));
    }
    Ok(values(codes))
}

/// The input levels each applied vector puts on the quantity's nets.
fn inputs(context: &Context<'_>) -> Result<Expected, String> {
    let circuit = circuit(context.variant)?;
    let netlist = parse_netlist(circuit)?;
    let names = port_names(context)?;
    let window = context
        .quantity
        .measure
        .window
        .unwrap_or([0.0, f64::INFINITY]);
    let mut codes = Vec::new();
    for vector in applied_vectors(&netlist, circuit, &context.quantity.analysis, window)? {
        let bits = names
            .iter()
            .map(|name| {
                vector
                    .get(name)
                    .copied()
                    .ok_or_else(|| format!("{name} is not an input of {circuit}"))
            })
            .collect::<Result<Vec<_>, _>>()?;
        codes.push(word(bits));
    }
    Ok(values(codes))
}

fn lfsr_step(state: u16) -> u16 {
    let bit = (state ^ (state >> 2) ^ (state >> 3) ^ (state >> 5)) & 1;
    (state >> 1) | (bit << 15)
}

/// The number of strobe rises in the window for a clock rising at `first` +
/// k·`period`.
fn rises(window: [f64; 2], first: f64, period: f64) -> usize {
    (0..)
        .map(|k| first + k as f64 * period)
        .take_while(|time| *time <= window[1])
        .filter(|time| *time >= window[0])
        .count()
}

/// The register before each clock rise: the seed, then one step per rise.
fn seq(context: &Context<'_>) -> Result<Expected, String> {
    let window = context
        .quantity
        .measure
        .window
        .ok_or("c10::seq needs its window")?;
    let mut state = LFSR_SEED;
    let mut codes = Vec::new();
    for _ in 0..rises(window, 50e-9, 100e-9) {
        codes.push(f64::from(state));
        state = lfsr_step(state);
    }
    Ok(values(codes))
}

/// The counter before each clock rise: the rises already counted.
fn count(context: &Context<'_>) -> Result<Expected, String> {
    let window = context
        .quantity
        .measure
        .window
        .ok_or("c10::count needs its window")?;
    Ok(values(
        (0..rises(window, 50.5e-9, 100e-9))
            .map(|k| k as f64)
            .collect(),
    ))
}

fn sample_count(context: &Context<'_>) -> Result<usize, String> {
    context
        .quantity
        .measure
        .times
        .map(|grid| grid.count)
        .ok_or_else(|| "c10: the DAC quantity needs its sample grid".to_owned())
}

/// The ladder 80 ns after clock rise k + 1: the state after k + 1 steps over
/// 2^16, in volts.
fn vdac_lfsr(context: &Context<'_>) -> Result<Expected, String> {
    let mut state = LFSR_SEED;
    let mut out = Vec::new();
    for _ in 0..sample_count(context)? {
        state = lfsr_step(state);
        out.push(f64::from(state) / DAC_FULL_SCALE);
    }
    Ok(values(out))
}

fn vdac_count(context: &Context<'_>) -> Result<Expected, String> {
    Ok(values(
        (1..=sample_count(context)?)
            .map(|k| k as f64 / DAC_FULL_SCALE)
            .collect(),
    ))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn lfsr_is_the_maximal_length_fibonacci_register() {
        // The seed's first successors (right shift, feedback into bit 15).
        assert_eq!(lfsr_step(0xACE1), 0x5670);
        let mut state = LFSR_SEED;
        let mut period = 0u32;
        loop {
            state = lfsr_step(state);
            period += 1;
            if state == LFSR_SEED {
                break;
            }
        }
        assert_eq!(period, 65535);
        assert_eq!(rises([0.0, 20e-6], 50e-9, 100e-9), 200);
    }

    #[test]
    fn gate_netlists_evaluate_by_primitive() {
        let mut netlist = Netlist {
            inputs: vec!["a".into(), "b".into()],
            outputs: vec!["y".into()],
            gates: HashMap::new(),
        };
        netlist
            .gates
            .insert("n".into(), ("nand".into(), vec!["a".into(), "b".into()]));
        netlist
            .gates
            .insert("y".into(), ("xor".into(), vec!["n".into(), "a".into()]));
        let mut known: HashMap<String, bool> =
            [("a".to_owned(), true), ("b".to_owned(), false)].into();
        assert!(!evaluate(&netlist, "y", &mut known).unwrap());
        assert_eq!(word([true, false, true]), 5.0);
    }
}
