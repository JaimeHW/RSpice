//! Versioned mixed-runtime transport. Keep JSON as one owned allocation and
//! bounded wire records so byte budgets and cancellation precede reconstruction.
//! Elaborated circuit validation and typed reconstruction happen during inject.
use super::*;

pub(super) const FORMAT_VERSION: u32 = 57;
const MAX_BYTES: usize = 64 * 1024 * 1024;

#[derive(Clone, Debug, PartialEq)]
pub(super) struct Image(String);

#[derive(serde::Deserialize)]
struct Header {
    version: u32,
    time: u64,
    num_nodes: usize,
    matrix_size: usize,
}

impl Image {
    #[cfg(feature = "veriloga")]
    pub(super) fn capture(circuit: &CircuitData, time: f64) -> Result<Self, String> {
        let image = circuit.mixed_runtime_checkpoint(time, Default::default())?;
        let json = serde_json::to_string(&image).map_err(|error| error.to_string())?;
        let result = Self(json);
        result.validate(time, circuit.matrix_size())?;
        Ok(result)
    }

    pub(super) fn validate(&self, time: f64, size: usize) -> Result<(), String> {
        if self.0.len() > MAX_BYTES || self.0.contains(['\r', '\n']) {
            return Err("mixed checkpoint record exceeds its byte or line limit".into());
        }
        let header: Header = serde_json::from_str(&self.0).map_err(|error| error.to_string())?;
        if header.version != 1
            || header.time != time.to_bits()
            || header.num_nodes > size
            || header.matrix_size != size
        {
            return Err("mixed checkpoint header differs from the accepted circuit".into());
        }
        Ok(())
    }

    #[cfg(feature = "veriloga")]
    pub(super) fn prepare(
        &self,
        circuit: &CircuitData,
        time: f64,
    ) -> Result<crate::circuit::mixed_checkpoint::RestoredMixedCircuit, String> {
        let limits = crate::circuit::mixed_checkpoint::MixedCheckpointLimits::default();
        crate::circuit::mixed_checkpoint::MixedCircuitCheckpoint::decode(self.0.as_bytes(), limits)?
            .restore(circuit, time, limits)
    }

    pub(super) fn retained_values(&self) -> usize {
        self.0.len().div_ceil(8)
    }
}

pub(super) fn read(
    lines: &mut CheckpointLines<'_>,
    version: u32,
    budget: &mut CheckpointParseBudget,
) -> Result<Option<Image>, String> {
    if version < FORMAT_VERSION {
        return Ok(None);
    }
    let header = lines.next().ok_or("missing mixed runtime image header")?;
    match parse_count_header(header, "mixed_runtime_image")? {
        0 => Ok(None),
        1 => {
            if !cfg!(feature = "veriloga") {
                return Err("mixed runtime checkpoint requires a Verilog-AMS enabled build".into());
            }
            let header = lines.next().ok_or("missing mixed runtime byte count")?;
            let bytes = parse_count_header(header, "mixed_runtime_bytes")?;
            if bytes == 0 || bytes > MAX_BYTES {
                return Err("mixed checkpoint byte limit exceeded".into());
            }
            if bytes.div_ceil(CHECKPOINT_IO_CHUNK_BYTES) > lines.remaining() {
                return Err("mixed runtime byte count exceeds remaining records".into());
            }
            budget.charge_bytes(bytes, "mixed runtime")?;
            let mut json = String::new();
            json.try_reserve_exact(bytes).map_err(|e| e.to_string())?;
            while json.len() < bytes {
                let record = lines
                    .next()
                    .ok_or("truncated mixed runtime image")?
                    .strip_prefix("mixed_runtime ")
                    .ok_or("invalid mixed runtime record")?;
                if record.is_empty()
                    || record.len() > CHECKPOINT_IO_CHUNK_BYTES
                    || record.len() > bytes - json.len()
                {
                    return Err("mixed runtime record has an invalid length".into());
                }
                json.push_str(record);
            }
            Ok(Some(Image(json)))
        }
        _ => Err("mixed runtime image count must be zero or one".into()),
    }
}

pub(super) fn write(
    out: &mut String,
    image: Option<&Image>,
    abort: &dyn AbortSignal,
) -> Result<(), SimulationError> {
    match image {
        None => out.push_str("mixed_runtime_image 0\n"),
        Some(image) => {
            out.push_str(&format!(
                "mixed_runtime_image 1\nmixed_runtime_bytes {}\n",
                image.0.len()
            ));
            // String byte offsets are not necessarily UTF-8 boundaries.
            let mut remaining = image.0.as_str();
            while !remaining.is_empty() {
                check_checkpoint_abort(abort)?;
                let mut end = remaining.len().min(CHECKPOINT_IO_CHUNK_BYTES);
                while !remaining.is_char_boundary(end) {
                    end -= 1;
                }
                out.push_str("mixed_runtime ");
                out.push_str(&remaining[..end]);
                out.push('\n');
                remaining = &remaining[end..];
            }
        }
    }
    Ok(())
}

#[cfg(all(test, feature = "veriloga"))]
mod tests {
    use super::*;

    #[test]
    fn mixed_runtime_transport_is_chunked_and_charged_before_copying() {
        // Include multi-byte UTF-8 at a chunk boundary, as source/model names
        // and task strings can contain it. This test concerns transport only.
        let image = Image(format!(
            "{{\"version\":1,\"time\":0,\"num_nodes\":2,\"matrix_size\":2,\"text\":\"{}\"}}",
            "界".repeat(CHECKPOINT_IO_CHUNK_BYTES)
        ));
        let mut text = String::new();
        write(&mut text, Some(&image), &NoAbort).unwrap();
        assert!(text.lines().count() > 3);
        assert!(
            text.lines()
                .skip(2)
                .all(|s| s.len() <= CHECKPOINT_IO_CHUNK_BYTES + "mixed_runtime ".len())
        );
        let mut budget = CheckpointParseBudget::new(image.0.len());
        let decoded = read(
            &mut CheckpointLines::new(&text),
            FORMAT_VERSION,
            &mut budget,
        )
        .unwrap()
        .unwrap();
        assert_eq!(decoded, image);
        assert_eq!(budget.used, image.0.len());
        decoded.validate(0.0, 2).unwrap();
        let mut limited = CheckpointParseBudget::new(image.0.len() - 1);
        assert!(
            read(
                &mut CheckpointLines::new(&text),
                FORMAT_VERSION,
                &mut limited
            )
            .is_err()
        );
        assert_eq!(limited.used, 0);
        for malformed in [
            "mixed_runtime_image 2\n".to_owned(),
            format!("mixed_runtime_image 1\nmixed_runtime_bytes {MAX_BYTES}\nmixed_runtime {{}}\n"),
            "mixed_runtime_image 1\nmixed_runtime_bytes 1\nmixed_runtime {}\n".into(),
            "mixed_runtime_image 1\nmixed_runtime_bytes 2\nmixed_runtime \n".into(),
        ] {
            assert!(
                read(
                    &mut CheckpointLines::new(&malformed),
                    FORMAT_VERSION,
                    &mut CheckpointParseBudget::new(MAX_BYTES)
                )
                .is_err()
            );
        }
        let abort = crate::abort_signal::CountingAbort::new(1);
        assert!(write(&mut String::new(), Some(&image), &abort).is_err());
    }
}
