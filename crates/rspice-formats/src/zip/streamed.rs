//! A stored ZIP archive written as its members are made.
//!
//! A member's bytes go to the output as they come, so a member of any size costs no memory:
//! its local header states no checksum and no size, and a data descriptor after its bytes states
//! them (general purpose flag bit 3), as every streaming ZIP writer writes them. The central
//! directory follows the last member. A member, an offset or a count past what ZIP32 can state
//! is stated in ZIP64 records instead.
//!
//! Every reader that opens an archive by its central directory (Python's `zipfile`, NumPy's
//! `load`, the `zip` crate this product reads with) reads such an archive as it reads any other.

use std::io::{self, Write};

/// The most ZIP32 states in a size or an offset field; the value itself says "see ZIP64".
const ZIP32_LIMIT: u64 = 0xffff_ffff;
/// The most members ZIP32's end record counts.
const ZIP32_MEMBERS: usize = 0xffff;
/// General purpose flags: sizes in a data descriptor (bit 3), names in UTF-8 (bit 11).
const FLAGS: u16 = 0x0808;
/// MS-DOS date 1980-01-01, the fixed date every member carries so an archive is its contents.
const DOS_DATE: u16 = 0x0021;

/// One member written, for the central directory.
struct Written {
    name: Vec<u8>,
    crc: u32,
    size: u64,
    offset: u64,
    /// Its local header carries a ZIP64 field, so its descriptor states eight-byte sizes.
    zip64: bool,
}

/// The member being written.
struct Open {
    member: Written,
    crc: flate2::Crc,
}

/// A stored ZIP archive written to `out` as its members are made: [`Self::start`] a member,
/// write its bytes through [`Write`], and [`Self::finish`] the archive.
pub struct StreamedZip<'w> {
    out: &'w mut dyn Write,
    written: u64,
    members: Vec<Written>,
    open: Option<Open>,
}

impl<'w> StreamedZip<'w> {
    /// An archive written to `out`.
    pub fn new(out: &'w mut dyn Write) -> Self {
        Self {
            out,
            written: 0,
            members: Vec::new(),
            open: None,
        }
    }

    /// Start the member `name`, ending the one before it. `bytes` is about how many it holds:
    /// a member that may reach 4 GiB is begun with a ZIP64 field, which a member that turns out
    /// larger than its start allowed for is refused without.
    pub fn start(&mut self, name: &str, bytes: u64) -> io::Result<()> {
        self.end_member()?;
        let name = name.as_bytes().to_vec();
        let name_len = u16::try_from(name.len())
            .map_err(|_| io::Error::other("a ZIP member name is longer than 65535 bytes"))?;
        // A margin over the estimate, so a member near the limit is stated in ZIP64.
        let zip64 = bytes.saturating_add(bytes / 8).saturating_add(1 << 20) >= ZIP32_LIMIT
            || self.written >= ZIP32_LIMIT;
        let mut header = Vec::with_capacity(30 + name.len() + 20);
        push_u32(&mut header, 0x0403_4b50);
        push_u16(&mut header, if zip64 { 45 } else { 20 });
        push_u16(&mut header, FLAGS);
        push_u16(&mut header, 0);
        push_u16(&mut header, 0);
        push_u16(&mut header, DOS_DATE);
        push_u32(&mut header, 0);
        let size = if zip64 { u32::MAX } else { 0 };
        push_u32(&mut header, size);
        push_u32(&mut header, size);
        push_u16(&mut header, name_len);
        push_u16(&mut header, if zip64 { 20 } else { 0 });
        header.extend_from_slice(&name);
        if zip64 {
            push_u16(&mut header, 0x0001);
            push_u16(&mut header, 16);
            header.extend_from_slice(&0_u64.to_le_bytes());
            header.extend_from_slice(&0_u64.to_le_bytes());
        }
        let offset = self.written;
        self.put(&header)?;
        self.open = Some(Open {
            member: Written {
                name,
                crc: 0,
                size: 0,
                offset,
                zip64,
            },
            crc: flate2::Crc::new(),
        });
        Ok(())
    }

    /// End the member being written, if one is: its data descriptor.
    fn end_member(&mut self) -> io::Result<()> {
        let Some(Open { mut member, crc }) = self.open.take() else {
            return Ok(());
        };
        member.crc = crc.sum();
        if !member.zip64 && member.size >= ZIP32_LIMIT {
            return Err(io::Error::other(
                "a ZIP member grew past 4 GiB without the ZIP64 field its start allowed for",
            ));
        }
        let mut descriptor = Vec::with_capacity(24);
        push_u32(&mut descriptor, 0x0807_4b50);
        push_u32(&mut descriptor, member.crc);
        if member.zip64 {
            descriptor.extend_from_slice(&member.size.to_le_bytes());
            descriptor.extend_from_slice(&member.size.to_le_bytes());
        } else {
            push_u32(&mut descriptor, member.size as u32);
            push_u32(&mut descriptor, member.size as u32);
        }
        self.put(&descriptor)?;
        self.members.push(member);
        Ok(())
    }

    /// End the last member and write the central directory. Returns the archive's length.
    pub fn finish(mut self) -> io::Result<u64> {
        self.end_member()?;
        let directory_offset = self.written;
        let mut directory = Vec::new();
        for member in &self.members {
            let large_size = member.size >= ZIP32_LIMIT;
            let large_offset = member.offset >= ZIP32_LIMIT;
            let mut extra = Vec::new();
            if large_size || large_offset {
                let mut fields = Vec::new();
                if large_size {
                    fields.extend_from_slice(&member.size.to_le_bytes());
                    fields.extend_from_slice(&member.size.to_le_bytes());
                }
                if large_offset {
                    fields.extend_from_slice(&member.offset.to_le_bytes());
                }
                push_u16(&mut extra, 0x0001);
                push_u16(&mut extra, fields.len() as u16);
                extra.extend_from_slice(&fields);
            }
            let version = if member.zip64 || !extra.is_empty() {
                45
            } else {
                20
            };
            push_u32(&mut directory, 0x0201_4b50);
            push_u16(&mut directory, version);
            push_u16(&mut directory, version);
            push_u16(&mut directory, FLAGS);
            push_u16(&mut directory, 0);
            push_u16(&mut directory, 0);
            push_u16(&mut directory, DOS_DATE);
            push_u32(&mut directory, member.crc);
            let size = if large_size {
                u32::MAX
            } else {
                member.size as u32
            };
            push_u32(&mut directory, size);
            push_u32(&mut directory, size);
            push_u16(&mut directory, member.name.len() as u16);
            push_u16(&mut directory, extra.len() as u16);
            push_u16(&mut directory, 0);
            push_u16(&mut directory, 0);
            push_u16(&mut directory, 0);
            push_u32(&mut directory, 0);
            push_u32(
                &mut directory,
                if large_offset {
                    u32::MAX
                } else {
                    member.offset as u32
                },
            );
            directory.extend_from_slice(&member.name);
            directory.extend_from_slice(&extra);
        }
        let directory_len = directory.len() as u64;
        self.put(&directory)?;
        let count = self.members.len();
        let zip64 = count >= ZIP32_MEMBERS
            || directory_offset >= ZIP32_LIMIT
            || directory_len >= ZIP32_LIMIT;
        let mut end = Vec::with_capacity(98);
        if zip64 {
            let record = self.written;
            push_u32(&mut end, 0x0606_4b50);
            end.extend_from_slice(&44_u64.to_le_bytes());
            push_u16(&mut end, 45);
            push_u16(&mut end, 45);
            push_u32(&mut end, 0);
            push_u32(&mut end, 0);
            end.extend_from_slice(&(count as u64).to_le_bytes());
            end.extend_from_slice(&(count as u64).to_le_bytes());
            end.extend_from_slice(&directory_len.to_le_bytes());
            end.extend_from_slice(&directory_offset.to_le_bytes());
            push_u32(&mut end, 0x0706_4b50);
            push_u32(&mut end, 0);
            end.extend_from_slice(&record.to_le_bytes());
            push_u32(&mut end, 1);
        }
        let count16 = u16::try_from(count)
            .ok()
            .filter(|_| !zip64)
            .unwrap_or(u16::MAX);
        push_u32(&mut end, 0x0605_4b50);
        push_u16(&mut end, 0);
        push_u16(&mut end, 0);
        push_u16(&mut end, count16);
        push_u16(&mut end, count16);
        push_u32(&mut end, clamp32(directory_len, zip64));
        push_u32(&mut end, clamp32(directory_offset, zip64));
        push_u16(&mut end, 0);
        self.put(&end)?;
        self.out.flush()?;
        Ok(self.written)
    }

    /// Write archive structure, outside any member.
    fn put(&mut self, bytes: &[u8]) -> io::Result<()> {
        self.out.write_all(bytes)?;
        self.written += bytes.len() as u64;
        Ok(())
    }
}

impl Write for StreamedZip<'_> {
    fn write(&mut self, bytes: &[u8]) -> io::Result<usize> {
        let Some(open) = self.open.as_mut() else {
            return Err(io::Error::other("no ZIP member is being written"));
        };
        let written = self.out.write(bytes)?;
        open.crc.update(&bytes[..written]);
        open.member.size += written as u64;
        self.written += written as u64;
        Ok(written)
    }

    fn flush(&mut self) -> io::Result<()> {
        self.out.flush()
    }
}

/// `value` in a ZIP32 field: itself, or the "see ZIP64" mark when ZIP64 states it.
fn clamp32(value: u64, zip64: bool) -> u32 {
    if zip64 { u32::MAX } else { value as u32 }
}

fn push_u16(bytes: &mut Vec<u8>, value: u16) {
    bytes.extend_from_slice(&value.to_le_bytes());
}

fn push_u32(bytes: &mut Vec<u8>, value: u32) {
    bytes.extend_from_slice(&value.to_le_bytes());
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::io::Read as _;

    fn read_back(bytes: &[u8]) -> Vec<(String, Vec<u8>)> {
        let mut archive = zip::ZipArchive::new(std::io::Cursor::new(bytes)).expect("it opens");
        (0..archive.len())
            .map(|index| {
                let mut member = archive.by_index(index).expect("a member");
                let mut content = Vec::new();
                member.read_to_end(&mut content).expect("it reads");
                (member.name().expect("a name").into_owned(), content)
            })
            .collect()
    }

    #[test]
    fn a_streamed_archive_reads_back_member_for_member() {
        let mut bytes = Vec::new();
        let mut archive = StreamedZip::new(&mut bytes);
        archive.start("time.npy", 5).unwrap();
        archive.write_all(b"hello").unwrap();
        archive.start("r=2k/V(out).npy", 0).unwrap();
        archive.start("empty.npy", 0).unwrap();
        archive.write_all(&[7; 70_000]).unwrap();
        let length = archive.finish().unwrap();
        assert_eq!(length, bytes.len() as u64);
        let members = read_back(&bytes);
        assert_eq!(members.len(), 3);
        assert_eq!(members[0], ("time.npy".to_owned(), b"hello".to_vec()));
        assert_eq!(members[1], ("r=2k/V(out).npy".to_owned(), Vec::new()));
        assert_eq!(members[2].1, vec![7; 70_000]);
    }

    /// A member begun as one that may pass 4 GiB carries ZIP64 sizes, and still reads back.
    #[test]
    fn a_member_begun_large_states_zip64_sizes_and_reads_back() {
        let mut bytes = Vec::new();
        let mut archive = StreamedZip::new(&mut bytes);
        archive.start("large.npy", u64::MAX).unwrap();
        archive.write_all(b"not so large").unwrap();
        archive.finish().unwrap();
        // The local header's ZIP64 field.
        assert_eq!(&bytes[30 + 9..30 + 9 + 4], &[1, 0, 16, 0]);
        assert_eq!(
            read_back(&bytes),
            [("large.npy".to_owned(), b"not so large".to_vec())]
        );
    }
}
