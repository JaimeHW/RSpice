//! A store of many runs: a directory holding `manifest.rsm` and one segment per member under
//! `runs/`.
//!
//! The manifest lists members in member-index order (never completion order) with what selects
//! each one (sweep or trial values, seed, corner, temperature), its analysis card, status, logical
//! digest and segment path. It is the store's only metadata file; each segment describes itself
//! without it. A finished member rewrites the manifest atomically: the new bytes go to a temporary
//! file that is made durable, then renamed over the old one, so a crash leaves either the old
//! manifest or the new one, never a mixture.

use crate::format::{Cursor, PutLe};
use crate::{Digest, RunStatus, StoreError};

const MANIFEST_MAGIC: [u8; 8] = *b"RSPMAN\r\n";
const MANIFEST_VERSION: u32 = 2;

/// One run of a store.
#[derive(Debug, Clone, PartialEq, Default)]
pub struct Member {
    pub index: u32,
    /// The segment's path relative to the store directory, `/`-separated.
    pub segment: String,
    pub analysis: String,
    pub sweep: Vec<(String, f64)>,
    pub seed: Option<u64>,
    pub corner: Option<String>,
    pub temperature: Option<f64>,
    /// `None` while the member runs.
    pub status: Option<RunStatus>,
    pub digest: Option<Digest>,
    /// The netlist fingerprint of the checkpoints the member's run wrote. A run resumed from one
    /// of them continues this member, and may append to its segment.
    pub checkpoint_fingerprint: Option<u64>,
}

/// The members of a store.
#[derive(Debug, Clone, PartialEq, Default)]
pub struct Manifest {
    members: Vec<Member>,
}

impl Manifest {
    pub fn members(&self) -> &[Member] {
        &self.members
    }

    /// Add `member`, or replace the member with its index; members stay in index order.
    pub fn upsert(&mut self, member: Member) {
        match self
            .members
            .binary_search_by_key(&member.index, |existing| existing.index)
        {
            Ok(at) => self.members[at] = member,
            Err(at) => self.members.insert(at, member),
        }
    }

    pub fn encode(&self) -> Vec<u8> {
        let mut out = Vec::new();
        out.extend_from_slice(&MANIFEST_MAGIC);
        out.put_u32(MANIFEST_VERSION);
        out.put_u32(self.members.len() as u32);
        for member in &self.members {
            out.put_u32(member.index);
            out.put_str(&member.segment);
            out.put_str(&member.analysis);
            out.put_u32(member.sweep.len() as u32);
            for (name, value) in &member.sweep {
                out.put_str(name);
                out.put_f64(*value);
            }
            match member.seed {
                Some(seed) => {
                    out.put_u8(1);
                    out.put_u64(seed);
                }
                None => out.put_u8(0),
            }
            match &member.corner {
                Some(corner) => {
                    out.put_u8(1);
                    out.put_str(corner);
                }
                None => out.put_u8(0),
            }
            match member.temperature {
                Some(temperature) => {
                    out.put_u8(1);
                    out.put_f64(temperature);
                }
                None => out.put_u8(0),
            }
            match &member.status {
                Some(status) => {
                    let (code, reason, message) = status.encode();
                    out.put_u8(1);
                    out.put_u8(code);
                    out.put_u8(reason);
                    out.put_str(message);
                }
                None => out.put_u8(0),
            }
            match member.digest {
                Some(digest) => {
                    out.put_u8(1);
                    out.extend_from_slice(&digest.0);
                }
                None => out.put_u8(0),
            }
            match member.checkpoint_fingerprint {
                Some(fingerprint) => {
                    out.put_u8(1);
                    out.put_u64(fingerprint);
                }
                None => out.put_u8(0),
            }
        }
        let crc = crc32fast::hash(&out);
        out.put_u32(crc);
        out
    }

    pub fn decode(bytes: &[u8]) -> Result<Self, StoreError> {
        if bytes.len() < 20 || bytes[..8] != MANIFEST_MAGIC {
            return Err(StoreError::corrupt(0, "not a results store manifest"));
        }
        let (body, crc) = bytes.split_at(bytes.len() - 4);
        if crc32fast::hash(body) != u32::from_le_bytes(crc.try_into().unwrap_or_default()) {
            return Err(StoreError::corrupt(
                0,
                "the manifest's checksum does not match",
            ));
        }
        let mut cursor = Cursor::new(body, 0);
        cursor.take(8)?;
        let version = cursor.u32()?;
        if version != MANIFEST_VERSION {
            return Err(cursor.error("unknown manifest version"));
        }
        let count = cursor.count(4)?;
        let mut members = Vec::with_capacity(count);
        let flag = |cursor: &mut Cursor<'_>| match cursor.u8()? {
            0 => Ok(false),
            1 => Ok(true),
            _ => Err(cursor.error("a presence flag is neither 0 nor 1")),
        };
        for _ in 0..count {
            let index = cursor.u32()?;
            let segment = cursor.string()?;
            let analysis = cursor.string()?;
            let sweeps = cursor.count(12)?;
            let mut sweep = Vec::with_capacity(sweeps);
            for _ in 0..sweeps {
                sweep.push((cursor.string()?, cursor.f64()?));
            }
            let seed = if flag(&mut cursor)? {
                Some(cursor.u64()?)
            } else {
                None
            };
            let corner = if flag(&mut cursor)? {
                Some(cursor.string()?)
            } else {
                None
            };
            let temperature = if flag(&mut cursor)? {
                Some(cursor.f64()?)
            } else {
                None
            };
            let status = if flag(&mut cursor)? {
                let (code, reason) = (cursor.u8()?, cursor.u8()?);
                let message = cursor.string()?;
                Some(
                    RunStatus::decode(code, reason, message)
                        .ok_or_else(|| cursor.error("unknown run status"))?,
                )
            } else {
                None
            };
            let digest = if flag(&mut cursor)? {
                let mut digest = [0u8; 32];
                digest.copy_from_slice(cursor.take(32)?);
                Some(Digest(digest))
            } else {
                None
            };
            let checkpoint_fingerprint = if flag(&mut cursor)? {
                Some(cursor.u64()?)
            } else {
                None
            };
            if members
                .last()
                .is_some_and(|last: &Member| last.index >= index)
            {
                return Err(cursor.error("manifest members are out of index order"));
            }
            members.push(Member {
                index,
                segment,
                analysis,
                sweep,
                seed,
                corner,
                temperature,
                status,
                digest,
                checkpoint_fingerprint,
            });
        }
        if cursor.remaining() != 0 {
            return Err(cursor.error("bytes follow the last manifest member"));
        }
        Ok(Self { members })
    }
}

/// A store directory on a file system.
#[cfg(any(unix, windows))]
#[derive(Debug, Clone)]
pub struct StoreDir {
    root: std::path::PathBuf,
}

#[cfg(any(unix, windows))]
impl StoreDir {
    pub const MANIFEST: &'static str = "manifest.rsm";
    const STAGED: &'static str = "manifest.rsm.new";

    pub fn new(root: impl Into<std::path::PathBuf>) -> Self {
        Self { root: root.into() }
    }

    pub fn root(&self) -> &std::path::Path {
        &self.root
    }

    /// The segment path a member's manifest entry names: `runs/<index>.rsr`.
    pub fn segment_name(index: u32) -> String {
        format!("runs/{index:06}.rsr")
    }

    /// Create the directory and its `runs/` folder.
    pub fn create(&self) -> Result<(), StoreError> {
        std::fs::create_dir_all(self.root.join("runs"))?;
        Ok(())
    }

    /// The manifest, or `None` before the first member finished.
    pub fn load_manifest(&self) -> Result<Option<Manifest>, StoreError> {
        match std::fs::read(self.root.join(Self::MANIFEST)) {
            Ok(bytes) => Manifest::decode(&bytes).map(Some),
            Err(error) if error.kind() == std::io::ErrorKind::NotFound => Ok(None),
            Err(error) => Err(error.into()),
        }
    }

    /// Replace the manifest atomically.
    pub fn save_manifest(&self, manifest: &Manifest) -> Result<(), StoreError> {
        self.stage_manifest(manifest)?;
        self.commit_manifest()
    }

    /// Write the new manifest beside the old one and make it durable.
    pub(crate) fn stage_manifest(&self, manifest: &Manifest) -> Result<(), StoreError> {
        use std::io::Write;
        let mut file = std::fs::File::create(self.root.join(Self::STAGED))?;
        file.write_all(&manifest.encode())?;
        file.sync_all()?;
        Ok(())
    }

    /// Rename the staged manifest over the old one, then make the rename durable.
    pub(crate) fn commit_manifest(&self) -> Result<(), StoreError> {
        std::fs::rename(self.root.join(Self::STAGED), self.root.join(Self::MANIFEST))?;
        #[cfg(unix)]
        std::fs::File::open(&self.root)?.sync_all()?;
        Ok(())
    }
}
