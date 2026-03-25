use super::CadencePsfError;
use super::binary_io::peek_u32;
use std::collections::{HashMap, HashSet};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub(super) enum SectionKind {
    Header,
    Type,
    Sweep,
    Trace,
    Value,
}

impl SectionKind {
    fn from_u32(value: u32) -> Result<Self, CadencePsfError> {
        match value {
            0 => Ok(Self::Header),
            1 => Ok(Self::Type),
            2 => Ok(Self::Sweep),
            3 => Ok(Self::Trace),
            4 => Ok(Self::Value),
            other => Err(CadencePsfError::new(format!(
                "unexpected section kind id {}",
                other
            ))),
        }
    }
}

#[derive(Debug, Clone, Copy)]
pub(super) struct TocEntry {
    pub(super) start: usize,
    #[allow(dead_code)]
    pub(super) end: usize,
}

#[derive(Debug, Clone)]
pub(super) struct Toc {
    entries: HashMap<SectionKind, TocEntry>,
}

impl Toc {
    pub(super) fn section(&self, kind: SectionKind) -> Result<TocEntry, CadencePsfError> {
        self.entries
            .get(&kind)
            .copied()
            .ok_or_else(|| CadencePsfError::new(format!("missing {:?} section in TOC", kind)))
    }
}

pub(super) fn parse_toc(data: &[u8]) -> Result<Toc, CadencePsfError> {
    if data.len() < 12 {
        return Err(CadencePsfError::new(
            "PSF binary payload too small to contain TOC trailer",
        ));
    }

    let toc_offset = peek_u32(&data[data.len() - 4..]) as usize;
    if toc_offset >= data.len() {
        return Err(CadencePsfError::new(format!(
            "invalid TOC offset {} for payload size {}",
            toc_offset,
            data.len()
        )));
    }

    let toc_bytes = data.len().saturating_sub(toc_offset + 12);
    if toc_bytes == 0 || !toc_bytes.is_multiple_of(8) {
        return Err(CadencePsfError::new(format!(
            "invalid TOC span {} bytes",
            toc_bytes
        )));
    }

    let mut starts: Vec<(SectionKind, usize)> = Vec::new();
    let mut seen_kinds = HashSet::new();
    let num_entries = toc_bytes / 8;
    for i in 0..num_entries {
        let base = toc_offset + i * 8;
        let kind = SectionKind::from_u32(peek_u32(&data[base..base + 4]))?;
        if !seen_kinds.insert(kind) {
            return Err(CadencePsfError::new(format!(
                "duplicate TOC entry for section {:?}",
                kind
            )));
        }
        let start = peek_u32(&data[base + 4..base + 8]) as usize;
        if start >= data.len() {
            return Err(CadencePsfError::new(format!(
                "TOC entry start {} out of range",
                start
            )));
        }
        if start >= toc_offset {
            return Err(CadencePsfError::new(format!(
                "TOC entry start {} overlaps TOC table at offset {}",
                start, toc_offset
            )));
        }
        starts.push((kind, start));
    }

    starts.sort_by_key(|(_, start)| *start);
    let mut entries = HashMap::new();
    for idx in 0..starts.len() {
        let (kind, start) = starts[idx];
        let end = starts
            .get(idx + 1)
            .map(|(_, next_start)| *next_start)
            .unwrap_or(data.len());
        if end <= start {
            return Err(CadencePsfError::new(
                "TOC entries are not strictly increasing",
            ));
        }
        entries.insert(kind, TocEntry { start, end });
    }

    for kind in [
        SectionKind::Header,
        SectionKind::Type,
        SectionKind::Sweep,
        SectionKind::Trace,
        SectionKind::Value,
    ] {
        if !entries.contains_key(&kind) {
            return Err(CadencePsfError::new(format!(
                "PSF binary is missing required {:?} section",
                kind
            )));
        }
    }

    Ok(Toc { entries })
}

#[cfg(test)]
mod tests {
    use super::*;

    fn build_payload(entries: &[(u32, u32)], toc_offset: usize) -> Vec<u8> {
        let toc_bytes = entries.len() * 8;
        let total_len = toc_offset + toc_bytes + 12;
        let mut data = vec![0u8; total_len];
        for (idx, (kind, start)) in entries.iter().enumerate() {
            let base = toc_offset + idx * 8;
            data[base..base + 4].copy_from_slice(&kind.to_be_bytes());
            data[base + 4..base + 8].copy_from_slice(&start.to_be_bytes());
        }
        let trailer_pos = total_len - 4;
        data[trailer_pos..].copy_from_slice(&(toc_offset as u32).to_be_bytes());
        data
    }

    #[test]
    fn test_parse_toc_valid_layout_resolves_required_sections() {
        let toc_offset = 20usize;
        let payload = build_payload(&[(0, 0), (1, 4), (2, 8), (3, 12), (4, 16)], toc_offset);
        let toc = parse_toc(&payload).expect("valid TOC should parse");
        let header = toc
            .section(SectionKind::Header)
            .expect("header section should exist");
        let value = toc
            .section(SectionKind::Value)
            .expect("value section should exist");
        assert_eq!(header.start, 0);
        assert_eq!(header.end, 4);
        assert_eq!(value.start, 16);
        assert_eq!(value.end, payload.len());
    }

    #[test]
    fn test_parse_toc_rejects_unknown_section_kind() {
        let payload = build_payload(&[(99, 0), (1, 4), (2, 8), (3, 12), (4, 16)], 20);
        let err = parse_toc(&payload).expect_err("unknown section kind must fail");
        assert!(err.to_string().contains("unexpected section kind id"));
    }

    #[test]
    fn test_parse_toc_rejects_out_of_range_start_offset() {
        let payload = build_payload(&[(0, 9999), (1, 4), (2, 8), (3, 12), (4, 16)], 20);
        let err = parse_toc(&payload).expect_err("out-of-range start must fail");
        assert!(err.to_string().contains("out of range"));
    }

    #[test]
    fn test_parse_toc_rejects_non_increasing_starts() {
        let payload = build_payload(&[(0, 0), (1, 0), (2, 8), (3, 12), (4, 16)], 20);
        let err = parse_toc(&payload).expect_err("duplicate starts must fail");
        assert!(err.to_string().contains("not strictly increasing"));
    }

    #[test]
    fn test_parse_toc_rejects_missing_required_section() {
        let payload = build_payload(&[(0, 0), (1, 4), (2, 8), (3, 12)], 20);
        let err = parse_toc(&payload).expect_err("missing sections must fail");
        assert!(err.to_string().contains("missing required"));
    }

    #[test]
    fn test_parse_toc_rejects_duplicate_section_kind() {
        let payload = build_payload(&[(0, 0), (0, 2), (1, 4), (2, 8), (3, 12), (4, 16)], 24);
        let err = parse_toc(&payload).expect_err("duplicate section kind must fail");
        assert!(err.to_string().contains("duplicate TOC entry"));
    }

    #[test]
    fn test_parse_toc_rejects_section_start_overlapping_toc_table() {
        let payload = build_payload(&[(0, 0), (1, 4), (2, 8), (3, 12), (4, 25)], 20);
        let err = parse_toc(&payload).expect_err("section start in TOC table must fail");
        assert!(err.to_string().contains("overlaps TOC table"));
    }
}
