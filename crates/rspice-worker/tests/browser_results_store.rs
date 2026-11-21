//! The results stores a browser keeps in its origin-private file system, reached through the
//! worker's own entry point in a dedicated worker, where synchronous access handles exist: a
//! segment written from its rows reads back with every sample once its store is closed and opened
//! again, and a store another holder has is refused.

#![cfg(target_arch = "wasm32")]

use js_sys::{Array, Float64Array, Reflect, Uint8Array};
use rspice_results_store::{
    Axis, Digest, MemoryIo, RunMeta, RunStatus, Schema, SegmentReader, SegmentWriter, Signal,
    SignalKind,
};
use rspice_simulation_contract::worker_transport::{
    ResultsStoreRequest, ResultsStoreResponse, ResultsStoreStanding, WorkerResultsSchema,
};
use wasm_bindgen::{JsCast, JsValue};
use wasm_bindgen_futures::JsFuture;
use wasm_bindgen_test::{console_log, wasm_bindgen_test, wasm_bindgen_test_configure};

wasm_bindgen_test_configure!(run_in_dedicated_worker);

/// The bytes of rows one request carries, as the page sends them.
const BYTES_PER_MESSAGE: usize = 8 << 20;

/// The bytes of a kept segment one read asks for, as the page asks.
const BYTES_PER_READ: u64 = 32 << 20;

async fn ask(request: &ResultsStoreRequest, buffers: Array) -> (ResultsStoreResponse, Array) {
    let answer = rspice_worker::run_rspice_ui_results_store_request(
        serde_json::to_string(request).unwrap(),
        buffers,
    )
    .await
    .unwrap_or_else(|error| panic!("{request:?}: {error:?}"));
    let response = Reflect::get(&answer, &"response".into())
        .unwrap()
        .as_string()
        .unwrap();
    let buffers = Reflect::get(&answer, &"buffers".into())
        .unwrap()
        .dyn_into::<Array>()
        .unwrap();
    (serde_json::from_str(&response).unwrap(), buffers)
}

/// Sample `row` of `signal` in the run seeded `seed`.
fn seeded(seed: u64, signal: u64, row: u64) -> f64 {
    let mut z = seed
        ^ (signal.wrapping_mul(0x9e37_79b9_7f4a_7c15))
        ^ row.wrapping_mul(0xbf58_476d_1ce4_e5b9);
    z = (z ^ (z >> 30)).wrapping_mul(0xbf58_476d_1ce4_e5b9);
    z = (z ^ (z >> 27)).wrapping_mul(0x94d0_49bb_1331_11eb);
    z ^= z >> 31;
    (z >> 11) as f64 / (1_u64 << 53) as f64 * 2.0 - 1.0
}

fn schema(signals: usize) -> Schema {
    Schema {
        x: Axis {
            name: "time".to_owned(),
            unit: "s".to_owned(),
        },
        signals: (0..signals)
            .map(|signal| Signal::new(&format!("v(n{signal})"), "V", SignalKind::Real))
            .collect(),
    }
}

fn meta() -> RunMeta {
    RunMeta {
        analysis: "tran".to_owned(),
        ..RunMeta::default()
    }
}

/// Rows `first .. first + count` of the seeded run: their times, and their values row-major.
fn rows(seed: u64, signals: usize, first: u64, count: usize) -> (Vec<f64>, Vec<f64>) {
    let x = (first..first + count as u64)
        .map(|row| row as f64 * 1e-9)
        .collect();
    let mut values = Vec::with_capacity(count * signals);
    for row in first..first + count as u64 {
        values.extend((0..signals as u64).map(|signal| seeded(seed, signal, row)));
    }
    (x, values)
}

fn rows_per_message(signals: usize) -> u64 {
    (BYTES_PER_MESSAGE / ((signals + 1) * 8)) as u64
}

/// The digest of the seeded run as the page holds it: written in the page's memory, as a page
/// holds a run's segment before it is kept.
fn page_digest(seed: u64, signals: usize, count: u64) -> Digest {
    let mut writer = SegmentWriter::create(MemoryIo::new(), &schema(signals), &meta()).unwrap();
    let per = rows_per_message(signals);
    let mut first = 0;
    while first < count {
        let take = per.min(count - first) as usize;
        let (x, values) = rows(seed, signals, first, take);
        writer.push_rows(&x, &values).unwrap();
        first += take as u64;
    }
    writer.finish(RunStatus::Completed).unwrap().0.digest
}

/// The bytes of the kept segment `digest`, checked and read as the page reads them.
async fn read_back(digest: &str) -> Vec<u8> {
    let len = match ask(
        &ResultsStoreRequest::Check {
            digest: digest.to_owned(),
        },
        Array::new(),
    )
    .await
    .0
    {
        ResultsStoreResponse::Checked { len } => len,
        other => panic!("the kept segment was not checked: {other:?}"),
    };
    let mut bytes = Vec::new();
    bytes.try_reserve_exact(len as usize).unwrap();
    while (bytes.len() as u64) < len {
        let offset = bytes.len() as u64;
        let ask_len = BYTES_PER_READ.min(len - offset);
        let (response, buffers) = ask(
            &ResultsStoreRequest::Read {
                digest: digest.to_owned(),
                offset,
                len: ask_len,
            },
            Array::new(),
        )
        .await;
        assert!(
            matches!(response, ResultsStoreResponse::Bytes),
            "{response:?}"
        );
        let chunk = buffers.get(0).dyn_into::<Uint8Array>().unwrap();
        assert_eq!(u64::from(chunk.length()), ask_len);
        let at = bytes.len();
        bytes.resize(at + ask_len as usize, 0);
        chunk.copy_to(&mut bytes[at..]);
    }
    bytes
}

fn now_ms() -> f64 {
    let performance = Reflect::get(&js_sys::global(), &"performance".into()).unwrap();
    let now = Reflect::get(&performance, &"now".into())
        .unwrap()
        .dyn_into::<js_sys::Function>()
        .unwrap();
    now.call0(&performance).unwrap().as_f64().unwrap()
}

/// Write the seeded run of `count` rows to the store `store` from its rows, close the store and
/// open it again, and read every sample back. Returns the milliseconds the round trip took and
/// the bytes this instance's memory grew to.
async fn round_trip(store: &str, seed: u64, signals: usize, count: u64) -> (f64, usize) {
    let started = now_ms();
    let digest = page_digest(seed, signals, count).to_string();
    let open = ResultsStoreRequest::Open {
        store: store.to_owned(),
    };
    match ask(&open, Array::new()).await.0 {
        ResultsStoreResponse::Opened {
            standing: ResultsStoreStanding::Held,
            segments,
        } => assert!(segments.is_empty(), "a fresh store keeps {segments:?}"),
        other => panic!("the store did not open: {other:?}"),
    }
    let sent = WorkerResultsSchema::of(&schema(signals), &meta()).unwrap();
    let write = match ask(
        &ResultsStoreRequest::Begin {
            schema: sent,
            digest: digest.clone(),
        },
        Array::new(),
    )
    .await
    .0
    {
        ResultsStoreResponse::Begun { write } => write,
        other => panic!("the write did not begin: {other:?}"),
    };
    let per = rows_per_message(signals);
    let mut first = 0;
    while first < count {
        let take = per.min(count - first) as usize;
        let (x, values) = rows(seed, signals, first, take);
        let buffers = Array::new();
        buffers.push(&Float64Array::from(x.as_slice()));
        buffers.push(&Float64Array::from(values.as_slice()));
        let taken = ask(&ResultsStoreRequest::Rows { write }, buffers).await.0;
        assert!(
            matches!(taken, ResultsStoreResponse::Taken { full: false }),
            "{taken:?}"
        );
        first += take as u64;
    }
    match ask(&ResultsStoreRequest::Finish { write }, Array::new())
        .await
        .0
    {
        ResultsStoreResponse::Finished { record, full } => {
            assert!(!full);
            assert_eq!(record.digest, digest);
            assert_eq!(record.rows, count);
        }
        other => panic!("the write did not finish: {other:?}"),
    }
    assert!(matches!(
        ask(&ResultsStoreRequest::Close, Array::new()).await.0,
        ResultsStoreResponse::Closed
    ));

    match ask(&open, Array::new()).await.0 {
        ResultsStoreResponse::Opened {
            standing: ResultsStoreStanding::Held,
            segments,
        } => assert!(
            segments.iter().any(|record| record.digest == digest),
            "the reopened store keeps {segments:?}"
        ),
        other => panic!("the store did not open again: {other:?}"),
    }
    let bytes = read_back(&digest).await;
    let mut reader = SegmentReader::open(MemoryIo::from_bytes(bytes)).unwrap();
    assert_eq!(reader.rows(), count);
    assert_eq!(reader.digest().unwrap().to_string(), digest);
    let mut first = 0;
    while first < count {
        let take = per.min(count - first) as usize;
        let (x, values) = rows(seed, signals, first, take);
        assert_eq!(
            reader.read_x(first, take).unwrap(),
            x,
            "times from row {first}"
        );
        for signal in 0..signals {
            let column = reader.read_slot(signal, first, take).unwrap();
            let expected: Vec<f64> = values
                .iter()
                .skip(signal)
                .step_by(signals)
                .copied()
                .collect();
            assert!(
                column
                    .iter()
                    .zip(&expected)
                    .all(|(read, wrote)| read.to_bits() == wrote.to_bits()),
                "signal {signal} from row {first}"
            );
        }
        first += take as u64;
    }
    let elapsed = now_ms() - started;
    let memory = wasm_bindgen::memory()
        .dyn_into::<js_sys::WebAssembly::Memory>()
        .unwrap()
        .buffer()
        .dyn_into::<js_sys::ArrayBuffer>()
        .unwrap()
        .byte_length() as usize;
    (elapsed, memory)
}

/// Let go of the store and delete its folder, so a run leaves the profile as it found it.
async fn remove_store(store: &str) {
    assert!(matches!(
        ask(&ResultsStoreRequest::Close, Array::new()).await.0,
        ResultsStoreResponse::Closed
    ));
    let remove = js_sys::Function::new_with_args(
        "name",
        "return navigator.storage.getDirectory()
            .then(root => root.getDirectoryHandle('rspice-results'))
            .then(stores => stores.removeEntry(name, {recursive: true}));",
    );
    let promise = remove
        .call1(&JsValue::NULL, &JsValue::from_str(store))
        .unwrap()
        .dyn_into::<js_sys::Promise>()
        .unwrap();
    JsFuture::from(promise).await.unwrap();
}

/// A run of a few mebibytes written to origin-private storage reads back with every sample after
/// its store is closed and opened again.
#[wasm_bindgen_test]
async fn a_segment_written_from_its_rows_reads_back_every_sample() {
    let store = "round-trip-small";
    let (elapsed, memory) = round_trip(store, 7, 3, 200_000).await;
    console_log!("6.1 MiB round trip: {elapsed:.0} ms, memory {memory} bytes");
    remove_store(store).await;
}

/// The same round trip at 1 GiB of samples: run with `--include-ignored`.
#[wasm_bindgen_test]
#[ignore = "writes 1 GiB to origin-private storage"]
async fn a_one_gibibyte_segment_reads_back_every_sample() {
    let store = "round-trip-gibibyte";
    let signals = 7;
    let count = (1_u64 << 30) / ((signals as u64 + 1) * 8);
    let (elapsed, memory) = round_trip(store, 11, signals, count).await;
    console_log!(
        "1 GiB round trip ({count} rows of {signals} signals): {elapsed:.0} ms, memory {memory} bytes"
    );
    remove_store(store).await;
}

/// A store another holder has, as another tab of this browser holds a project's store, is
/// refused: nothing is written to it and nothing is read from it.
#[wasm_bindgen_test]
async fn a_store_another_holder_has_is_refused() {
    let store = "held-elsewhere";
    let hold = js_sys::Function::new_with_args(
        "name",
        "return new Promise(granted => navigator.locks.request(name, {mode: 'exclusive'},
            () => { granted(); return new Promise(() => {}); }));",
    );
    let held = hold
        .call1(
            &JsValue::NULL,
            &JsValue::from_str(&format!("rspice-results-store-{store}")),
        )
        .unwrap()
        .dyn_into::<js_sys::Promise>()
        .unwrap();
    JsFuture::from(held).await.unwrap();
    let open = ResultsStoreRequest::Open {
        store: store.to_owned(),
    };
    match ask(&open, Array::new()).await.0 {
        ResultsStoreResponse::Opened {
            standing: ResultsStoreStanding::HeldElsewhere,
            segments,
        } => assert!(segments.is_empty()),
        other => panic!("a store held elsewhere opened: {other:?}"),
    }
    let sent = WorkerResultsSchema::of(&schema(1), &meta()).unwrap();
    let begun = ask(
        &ResultsStoreRequest::Begin {
            schema: sent,
            digest: page_digest(3, 1, 10).to_string(),
        },
        Array::new(),
    )
    .await
    .0;
    assert!(
        matches!(begun, ResultsStoreResponse::Failed { .. }),
        "a write began without the store: {begun:?}"
    );
}
