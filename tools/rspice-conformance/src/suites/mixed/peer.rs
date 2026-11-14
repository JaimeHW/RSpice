//! `peer`: the corpus's `d_process` peer programs, served by this binary.
//!
//! A corpus deck names `process_file="rspice-mixed-corpus"` with
//! `process_params=["peer" "<name>"]`, and the runner puts its own directory
//! first on its case children's `PATH`, so the deck runs the same peer on
//! every machine with nothing to build.
//!
//! The protocol is ngspice's `d_process` pipe protocol, version 1: the
//! simulator writes a three-byte header (version, input bit count, output
//! bit count), which the peer echoes; then each clocked exchange is the
//! simulator's time as a native `f64` and the packed input bytes, answered
//! with the packed output bytes.

use std::io::{self, Read, Write};

/// Serve `name` on stdin and stdout until the simulator closes the pipe.
pub fn serve(name: &str) -> Result<(), String> {
    match name {
        "toggle" => toggle(&mut io::stdin().lock(), &mut io::stdout().lock())
            .map_err(|error| format!("peer toggle: {error}")),
        other => Err(format!(
            "unknown d_process peer '{other}'; the corpus has 'toggle'"
        )),
    }
}

/// Answer every exchange with output bit 0 flipped from its last answer,
/// starting from 0, whatever the inputs: the peer of the ngspice process
/// protocol's examples. A repeated exchange shows in its answers.
fn toggle(input: &mut impl Read, output: &mut impl Write) -> io::Result<()> {
    let mut header = [0u8; 3];
    input.read_exact(&mut header)?;
    output.write_all(&header)?;
    output.flush()?;
    let packed = |bits: u8| usize::from(bits).div_ceil(8);
    let mut exchange = vec![0u8; 8 + packed(header[1])];
    let mut answer = vec![0u8; packed(header[2])];
    let mut q = 0u8;
    loop {
        match input.read_exact(&mut exchange) {
            Ok(()) => {}
            Err(error) if error.kind() == io::ErrorKind::UnexpectedEof => return Ok(()),
            Err(error) => return Err(error),
        }
        q ^= 1;
        if let Some(first) = answer.first_mut() {
            *first = q;
        }
        output.write_all(&answer)?;
        output.flush()?;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn toggle_echoes_the_header_and_flips_its_bit_on_every_exchange() {
        let mut request = vec![1u8, 3, 9];
        for time in [1.0e-6_f64, 2.0e-6, 3.0e-6] {
            request.extend_from_slice(&time.to_ne_bytes());
            request.push(0b101);
        }
        let mut reply = Vec::new();
        toggle(&mut request.as_slice(), &mut reply).unwrap();
        assert_eq!(reply, [1, 3, 9, 1, 0, 0, 0, 1, 0]);
    }
}
