//! `$readmemb` and `$readmemh` (IEEE 1364-2005 section 17.2.8).
//!
//! The file holds white space, `//` and `/* */` comments, `@hex` addresses
//! and words of binary or hexadecimal digits with `x`, `z`, `?` and `_`.
//! With no range in the call and no address in the file, words load from the
//! memory's lowest address toward its highest (the 1364-2005 rule); a start
//! address alone loads toward the highest address, and a start and finish
//! load toward the finish in whichever direction it lies. A word with fewer
//! digits than the memory is wide is zero-extended, one with more keeps its
//! low digits. The diagnostics follow section 17.2.8: an address outside the
//! memory or the requested range is an error that stops the load; a count of
//! words that differs from the range, when the file names no address, is a
//! warning. Each expected value in the tests below is an Icarus 13.0 run of
//! an RSpice-authored probe.

use rspice_veriloga::canonical_ir::digital_value::FourStateValue;
use rspice_veriloga::four_state::FourStateBit;

/// One `$readmem` call, its file already read.
pub(crate) struct Readmem<'a> {
    pub text: &'a [u8],
    /// 1 for `$readmemb`, 4 for `$readmemh`.
    pub bits_per_digit: u32,
    /// The memory's word width.
    pub width: u32,
    /// The memory's lowest and highest addresses.
    pub low: i64,
    pub high: i64,
    pub start: Option<i64>,
    pub finish: Option<i64>,
}

/// What the load does and says.
#[derive(Debug, Default, PartialEq)]
pub(crate) struct Loaded {
    /// Each word and the address it goes to, in file order.
    pub words: Vec<(i64, FourStateValue)>,
    pub warnings: Vec<String>,
    /// The error that stopped the load, if one did.
    pub error: Option<String>,
}

pub(crate) fn load(request: &Readmem<'_>) -> Loaded {
    let mut loaded = Loaded::default();
    let in_memory = |address: i64| (request.low..=request.high).contains(&address);
    for (name, address) in [("Start", request.start), ("Finish", request.finish)] {
        if let Some(address) = address
            && !in_memory(address)
        {
            loaded.error = Some(format!(
                "{name} address {address} is out of bounds for the memory [{}:{}]",
                request.low, request.high
            ));
            return loaded;
        }
    }
    let first = request.start.unwrap_or(request.low);
    let last = request.finish.unwrap_or(request.high);
    let step: i64 = if last < first { -1 } else { 1 };
    let in_range = |address: i64| {
        let (low, high) = if step > 0 {
            (first, last)
        } else {
            (last, first)
        };
        (low..=high).contains(&address)
    };
    let range = format!("[{first}:{last}]");
    let mut address = first;
    let mut addressed = false;
    let mut words = 0u64;
    let text = request.text;
    let mut at = 0;
    while at < text.len() {
        let byte = text[at];
        if byte.is_ascii_whitespace() {
            at += 1;
            continue;
        }
        if text[at..].starts_with(b"//") {
            while at < text.len() && text[at] != b'\n' {
                at += 1;
            }
            continue;
        }
        if text[at..].starts_with(b"/*") {
            match text[at + 2..].windows(2).position(|pair| pair == b"*/") {
                Some(end) => at += 2 + end + 2,
                None => {
                    loaded.error = Some("a `/*` comment is not closed".into());
                    return loaded;
                }
            }
            continue;
        }
        let start = at;
        while at < text.len()
            && !text[at].is_ascii_whitespace()
            && !text[at..].starts_with(b"//")
            && !text[at..].starts_with(b"/*")
        {
            at += 1;
        }
        let token = &text[start..at];
        if let Some(digits) = token.strip_prefix(b"@") {
            let value = std::str::from_utf8(digits)
                .ok()
                .map(|digits| digits.replace('_', ""))
                .and_then(|digits| i64::from_str_radix(&digits, 16).ok());
            let Some(value) = value else {
                loaded.error = Some(format!(
                    "invalid address `{}`",
                    String::from_utf8_lossy(token)
                ));
                return loaded;
            };
            if !in_range(value) || !in_memory(value) {
                loaded.error = Some(format!(
                    "address {value:#x} is outside the range {range} of the memory [{}:{}]",
                    request.low, request.high
                ));
                return loaded;
            }
            address = value;
            addressed = true;
            continue;
        }
        let word = match parse_word(token, request.bits_per_digit, request.width) {
            Ok((word, excess)) => {
                if excess > 0 {
                    loaded.warnings.push(format!(
                        "excess digits ({excess} of '{}') while reading {}-bit words",
                        String::from_utf8_lossy(token),
                        request.width
                    ));
                }
                word
            }
            Err(invalid) => {
                loaded.error = Some(format!("invalid input character: {}", char::from(invalid)));
                return loaded;
            }
        };
        if !in_range(address) {
            loaded.warnings.push(format!(
                "too many words in the file for the requested range {range}"
            ));
            return loaded;
        }
        loaded.words.push((address, word));
        words += 1;
        address += step;
    }
    let expected = first.abs_diff(last) + 1;
    if !addressed && words < expected {
        loaded.warnings.push(format!(
            "not enough words in the file for the requested range {range}"
        ));
    }
    loaded
}

/// One word's digits, most significant first, at `width` bits: the value and
/// how many digits did not fit; or the first byte that is no digit.
fn parse_word(token: &[u8], bits_per_digit: u32, width: u32) -> Result<(FourStateValue, u32), u8> {
    let radix = 1u32 << bits_per_digit;
    let mut bits = Vec::with_capacity(token.len() * bits_per_digit as usize);
    let mut digits = 0u32;
    for &byte in token {
        let group: Vec<FourStateBit> = match byte {
            b'_' => continue,
            b'x' | b'X' | b'?' => vec![FourStateBit::Unknown; bits_per_digit as usize],
            b'z' | b'Z' => vec![FourStateBit::HighImpedance; bits_per_digit as usize],
            _ => {
                let value = char::from(byte)
                    .to_digit(16)
                    .filter(|value| *value < radix)
                    .ok_or(byte)?;
                (0..bits_per_digit)
                    .rev()
                    .map(|bit| {
                        if value >> bit & 1 == 1 {
                            FourStateBit::One
                        } else {
                            FourStateBit::Zero
                        }
                    })
                    .collect()
            }
        };
        bits.extend(group);
        digits += 1;
    }
    if digits == 0 {
        return Err(b'_');
    }
    let fitting = width.div_ceil(bits_per_digit);
    let excess = digits.saturating_sub(fitting);
    let mut value = FourStateValue::zero(width);
    for (index, bit) in bits.iter().rev().enumerate() {
        if index as u32 >= width {
            break;
        }
        value.set_bit(index as u32, *bit);
    }
    Ok((value, excess))
}

#[cfg(test)]
mod tests {
    use super::*;

    fn bytes_of(loaded: &Loaded) -> Vec<(i64, String)> {
        loaded
            .words
            .iter()
            .map(|(address, value)| {
                let text: String = value
                    .bits_msb_first()
                    .iter()
                    .map(|bit| match bit {
                        FourStateBit::Zero => '0',
                        FourStateBit::One => '1',
                        FourStateBit::Unknown => 'x',
                        FourStateBit::HighImpedance => 'z',
                    })
                    .collect();
                (*address, text)
            })
            .collect()
    }

    fn request(text: &str, start: Option<i64>, finish: Option<i64>) -> Loaded {
        load(&Readmem {
            text: text.as_bytes(),
            bits_per_digit: 4,
            width: 8,
            low: 0,
            high: 7,
            start,
            finish,
        })
    }

    const FIVE: &str = "1 2 3\n4 // c\n/* x */ 5\n";

    /// Icarus 13.0, probe `p3.v`.
    #[test]
    fn addressing_matches_icarus() {
        let loaded = request(FIVE, None, None);
        let addresses: Vec<i64> = loaded.words.iter().map(|(address, _)| *address).collect();
        assert_eq!(addresses, [0, 1, 2, 3, 4]);
        assert_eq!(
            loaded.warnings,
            ["not enough words in the file for the requested range [0:7]"]
        );
        let loaded = request(FIVE, Some(6), Some(2));
        let addresses: Vec<i64> = loaded.words.iter().map(|(address, _)| *address).collect();
        assert_eq!(addresses, [6, 5, 4, 3, 2]);
        assert!(loaded.warnings.is_empty());
        let loaded = request(FIVE, Some(1), None);
        assert_eq!(loaded.words.first().map(|word| word.0), Some(1));
        assert_eq!(
            loaded.warnings,
            ["not enough words in the file for the requested range [1:7]"]
        );
        let loaded = request("@2 a\nb\n@0 c\n", None, None);
        assert_eq!(
            bytes_of(&loaded),
            [
                (2, "00001010".into()),
                (3, "00001011".into()),
                (0, "00001100".into())
            ]
        );
        assert!(loaded.warnings.is_empty());
        let loaded = request(FIVE, Some(0), Some(3));
        assert_eq!(loaded.words.len(), 4);
        assert_eq!(
            loaded.warnings,
            ["too many words in the file for the requested range [0:3]"]
        );
        assert!(request(FIVE, Some(-1), Some(7)).error.is_some());
        assert!(request(FIVE, Some(0), Some(8)).error.is_some());
    }

    #[test]
    fn words_fit_the_memory_width() {
        let loaded = request("x z 1x 123\n", None, None);
        assert_eq!(
            bytes_of(&loaded),
            [
                (0, "0000xxxx".into()),
                (1, "0000zzzz".into()),
                (2, "0001xxxx".into()),
                (3, "00100011".into())
            ]
        );
        assert_eq!(
            loaded.warnings[0],
            "excess digits (1 of '123') while reading 8-bit words"
        );
        let binary = load(&Readmem {
            text: FIVE.as_bytes(),
            bits_per_digit: 1,
            width: 8,
            low: 10,
            high: 13,
            start: None,
            finish: None,
        });
        assert_eq!(binary.words.len(), 1);
        assert_eq!(binary.error.as_deref(), Some("invalid input character: 2"));
    }
}
