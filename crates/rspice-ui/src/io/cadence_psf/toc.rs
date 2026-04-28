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
    if toc_bytes == 0 || !crate::utils::numeric::is_multiple_of(toc_bytes, 8) {
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

