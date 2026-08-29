//! Public transient-checkpoint persistence contract.

use rspice_core::engine::{
    Engine, SimulationConfig, TransientCheckpoint, TransientCheckpointEncoding,
};
use rspice_core::netlist::Netlist;

const PACKED_HEADER_BYTES: usize = 72;
const UNPACKED_LENGTH_OFFSET: usize = 24;
const COMPRESSED_LENGTH_OFFSET: usize = 32;

fn checkpoint_fixture() -> (Engine, Netlist, TransientCheckpoint) {
    let netlist = Netlist::parse(
        "checkpoint encoding contract\n\
         V1 in 0 PULSE(0 1 0 100n 100n 4u 10u)\n\
         R1 in out 1k\n\
         C1 out 0 2n\n\
         .TRAN 50n 12u\n\
         .END\n",
    )
    .expect("checkpoint fixture parses");
    let engine = Engine::new(SimulationConfig::default());
    let (_, checkpoint) = engine
        .run_tran_checkpointed(&netlist, 6.0e-6, 50.0e-9)
        .expect("checkpoint fixture first segment solves");
    (engine, netlist, checkpoint)
}

fn write_u64(bytes: &mut [u8], offset: usize, value: u64) {
    bytes[offset..offset + 8].copy_from_slice(&value.to_le_bytes());
}

#[test]
fn packed_and_unpacked_round_trip_exactly_and_are_not_format_aliases() {
    let (_, _, checkpoint) = checkpoint_fixture();
    let unpacked = checkpoint
        .to_bytes(TransientCheckpointEncoding::Unpacked)
        .expect("unpacked encoding succeeds");
    let packed = checkpoint
        .to_bytes(TransientCheckpointEncoding::Packed)
        .expect("packed encoding succeeds");

    assert_eq!(unpacked, checkpoint.to_text().as_bytes());
    assert_ne!(
        packed, unpacked,
        "packed bytes must not alias canonical text"
    );
    assert!(unpacked.starts_with(b"RSPICE-CHECKPOINT "));
    assert!(packed.starts_with(b"RSPICE-CPACK\0\0\0\0"));
    let restored_unpacked = TransientCheckpoint::from_bytes(&unpacked).unwrap();
    let restored_packed = TransientCheckpoint::from_bytes(&packed).unwrap();
    assert_eq!(restored_unpacked, checkpoint);
    assert_eq!(restored_packed, checkpoint);
    assert_eq!(
        restored_unpacked
            .to_bytes(TransientCheckpointEncoding::Unpacked)
            .unwrap(),
        unpacked,
        "unpacked round trip must preserve the exact canonical bytes"
    );
    assert_eq!(
        restored_packed
            .to_bytes(TransientCheckpointEncoding::Packed)
            .unwrap(),
        packed,
        "fixed-version packed output must be byte-for-byte deterministic"
    );
    assert!(
        TransientCheckpoint::from_bytes_with_encoding(
            &packed,
            TransientCheckpointEncoding::Unpacked,
            usize::MAX,
        )
        .expect_err("packed bytes cannot pass the unpacked decoder")
        .contains("packed checkpoint supplied")
    );
    assert!(
        TransientCheckpoint::from_bytes_with_encoding(
            &unpacked,
            TransientCheckpointEncoding::Packed,
            usize::MAX,
        )
        .expect_err("text cannot pass the packed decoder")
        .contains("magic")
    );
}

#[test]
fn packed_envelope_rejects_integrity_length_truncation_and_trailing_violations() {
    let (_, _, checkpoint) = checkpoint_fixture();
    let packed = checkpoint
        .to_bytes(TransientCheckpointEncoding::Packed)
        .expect("packed encoding succeeds");
    assert!(packed.len() > PACKED_HEADER_BYTES);

    for header_len in [0, 1, 15, 16, PACKED_HEADER_BYTES - 1] {
        assert!(
            TransientCheckpoint::from_bytes_with_encoding(
                &packed[..header_len],
                TransientCheckpointEncoding::Packed,
                usize::MAX,
            )
            .expect_err("every incomplete packed header boundary must fail")
            .contains("truncated packed checkpoint header"),
            "unexpected diagnostic for {header_len}-byte header"
        );
    }

    let mut maximum_unpacked = packed.clone();
    write_u64(&mut maximum_unpacked, UNPACKED_LENGTH_OFFSET, u64::MAX);
    let error = TransientCheckpoint::from_bytes(&maximum_unpacked)
        .expect_err("u64::MAX unpacked length must fail before allocation");
    assert!(
        error.contains("exceeds this platform") || error.contains("configured limit"),
        "unexpected maximum-unpacked-length diagnostic: {error}"
    );

    let mut maximum_compressed = packed.clone();
    write_u64(&mut maximum_compressed, COMPRESSED_LENGTH_OFFSET, u64::MAX);
    let error = TransientCheckpoint::from_bytes(&maximum_compressed)
        .expect_err("u64::MAX compressed length must fail before payload access");
    assert!(
        error.contains("exceeds this platform") || error.contains("length overflow"),
        "unexpected maximum-compressed-length diagnostic: {error}"
    );

    let mut corrupt = packed.clone();
    let last = corrupt.len() - 1;
    corrupt[last] ^= 0x01;
    let error = TransientCheckpoint::from_bytes(&corrupt)
        .expect_err("corrupt compressed data or integrity seal must fail");
    assert!(
        error.contains("zlib") || error.contains("integrity"),
        "unexpected corruption diagnostic: {error}"
    );

    let mut corrupt_seal = packed.clone();
    corrupt_seal[40] ^= 0x80;
    assert!(
        TransientCheckpoint::from_bytes(&corrupt_seal)
            .expect_err("an altered BLAKE3 seal must fail")
            .contains("BLAKE3 integrity check failed")
    );

    let declared_unpacked = u64::from_le_bytes(
        packed[UNPACKED_LENGTH_OFFSET..UNPACKED_LENGTH_OFFSET + 8]
            .try_into()
            .unwrap(),
    );
    let mut wrong_unpacked = packed.clone();
    write_u64(
        &mut wrong_unpacked,
        UNPACKED_LENGTH_OFFSET,
        declared_unpacked + 1,
    );
    assert!(
        TransientCheckpoint::from_bytes(&wrong_unpacked)
            .expect_err("wrong unpacked length must fail")
            .contains("length")
    );

    let declared_compressed = u64::from_le_bytes(
        packed[COMPRESSED_LENGTH_OFFSET..COMPRESSED_LENGTH_OFFSET + 8]
            .try_into()
            .unwrap(),
    );
    let mut wrong_compressed = packed.clone();
    write_u64(
        &mut wrong_compressed,
        COMPRESSED_LENGTH_OFFSET,
        declared_compressed - 1,
    );
    assert!(
        TransientCheckpoint::from_bytes(&wrong_compressed)
            .expect_err("declared short payload must expose trailing data")
            .contains("trailing")
    );

    assert!(
        TransientCheckpoint::from_bytes(&packed[..packed.len() - 1])
            .expect_err("truncated payload must fail")
            .contains("truncated")
    );
    let mut trailing = packed.clone();
    trailing.push(0);
    assert!(
        TransientCheckpoint::from_bytes(&trailing)
            .expect_err("trailing envelope bytes must fail")
            .contains("trailing")
    );

    let mut stream_trailing = packed.clone();
    stream_trailing.push(0);
    write_u64(
        &mut stream_trailing,
        COMPRESSED_LENGTH_OFFSET,
        declared_compressed + 1,
    );
    assert!(
        TransientCheckpoint::from_bytes(&stream_trailing)
            .expect_err("bytes after the zlib stream inside the envelope must fail")
            .contains("compressed stream has 1 trailing bytes")
    );
}

#[test]
fn checkpoint_decoding_and_file_reads_obey_independent_byte_limits() {
    let (_, _, checkpoint) = checkpoint_fixture();
    let unpacked = checkpoint
        .to_bytes(TransientCheckpointEncoding::Unpacked)
        .unwrap();
    let packed = checkpoint
        .to_bytes(TransientCheckpointEncoding::Packed)
        .unwrap();

    assert!(
        TransientCheckpoint::from_bytes_with_limit(&unpacked, unpacked.len() - 1)
            .expect_err("unpacked input over its decoded limit must fail")
            .contains("configured limit")
    );
    assert!(
        TransientCheckpoint::from_bytes_with_limit(&packed, unpacked.len() - 1)
            .expect_err("packed expansion over its decoded limit must fail")
            .contains("configured limit")
    );

    let directory = std::env::temp_dir().join(format!(
        "rspice-checkpoint-codec-{}-{}",
        std::process::id(),
        std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .unwrap()
            .as_nanos()
    ));
    std::fs::create_dir(&directory).unwrap();
    let path = directory.join("state.chk");
    checkpoint
        .save_with_encoding(&path, TransientCheckpointEncoding::Packed)
        .unwrap();
    assert!(
        TransientCheckpoint::load_with_limit(&path, packed.len() - 1, unpacked.len())
            .expect_err("encoded file limit must be enforced")
            .contains("encoded limit")
    );
    assert_eq!(
        TransientCheckpoint::load_with_limit(&path, packed.len(), unpacked.len()).unwrap(),
        checkpoint
    );
    std::fs::remove_dir_all(directory).unwrap();
}

#[test]
fn packed_and_unpacked_restores_produce_identical_state_and_trajectory() {
    let (engine, netlist, checkpoint) = checkpoint_fixture();
    let unpacked = TransientCheckpoint::from_bytes(
        &checkpoint
            .to_bytes(TransientCheckpointEncoding::Unpacked)
            .unwrap(),
    )
    .unwrap();
    let packed = TransientCheckpoint::from_bytes(
        &checkpoint
            .to_bytes(TransientCheckpointEncoding::Packed)
            .unwrap(),
    )
    .unwrap();
    assert_eq!(unpacked, packed);

    let (unpacked_result, unpacked_final) = engine
        .run_tran_resume(&netlist, &unpacked, 12.0e-6, 50.0e-9)
        .expect("unpacked checkpoint resumes");
    let (packed_result, packed_final) = engine
        .run_tran_resume(&netlist, &packed, 12.0e-6, 50.0e-9)
        .expect("packed checkpoint resumes");
    assert_eq!(unpacked_final, packed_final);
    assert_eq!(unpacked_result.time, packed_result.time);
    assert_eq!(unpacked_result.voltages, packed_result.voltages);
    assert_eq!(
        unpacked_result.branch_currents,
        packed_result.branch_currents
    );
}
