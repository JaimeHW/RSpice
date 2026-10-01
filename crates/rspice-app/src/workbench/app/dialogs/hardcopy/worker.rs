//! Browser transport and application-owned source capture for hardcopy workers.

pub(crate) use rspice_hardcopy::worker::decode_packaged_publication;
#[cfg(target_arch = "wasm32")]
use rspice_hardcopy::worker::validate_response_buffer_lengths;
use rspice_hardcopy::worker::{
    HARDCOPY_WORKER_PROTOCOL_VERSION, HardcopyWorkerCommand, HardcopyWorkerOperation,
    HardcopyWorkerRequest, publication_part_count,
};
#[cfg(test)]
use rspice_hardcopy::worker::{HardcopyWorkerResponse, execute_request};

#[cfg(target_arch = "wasm32")]
mod browser {
    use std::cell::{Cell, RefCell};
    use std::rc::Rc;

    use js_sys::{Array, Object, Reflect, Uint8Array};
    use wasm_bindgen::JsCast as _;
    use wasm_bindgen::prelude::*;

    use super::{
        HARDCOPY_WORKER_PROTOCOL_VERSION, HardcopyWorkerCommand, HardcopyWorkerOperation,
        HardcopyWorkerRequest, publication_part_count, validate_response_buffer_lengths,
    };

    #[derive(Debug, Clone, Copy, PartialEq, Eq)]
    pub(crate) struct HardcopyWorkerTicket {
        pub(crate) id: u32,
        pub(crate) epoch: u64,
        pub(crate) generation: u64,
        pub(crate) operation: HardcopyWorkerOperation,
        expected_buffer_count: Option<usize>,
    }

    /// Where a render lands: empty until the worker answers, then either the
    /// page buffers or the failure text, written exactly once.
    type RenderSlot = RefCell<Option<Result<Vec<Vec<u8>>, String>>>;

    struct ActiveHardcopyWorker {
        ticket: HardcopyWorkerTicket,
        worker: web_sys::Worker,
        result: Rc<RenderSlot>,
        _onmessage: Closure<dyn FnMut(web_sys::MessageEvent)>,
        _onerror: Closure<dyn FnMut(web_sys::ErrorEvent)>,
        _onmessageerror: Closure<dyn FnMut(web_sys::MessageEvent)>,
        deadline: crate::time_compat::Instant,
    }

    impl Drop for ActiveHardcopyWorker {
        fn drop(&mut self) {
            self.worker.set_onmessage(None);
            self.worker.set_onerror(None);
            self.worker.set_onmessageerror(None);
            self.worker.terminate();
        }
    }

    thread_local! {
        static NEXT_REQUEST_ID: Cell<u32> = const { Cell::new(0) };
        static ACTIVE_WORKER: RefCell<Option<ActiveHardcopyWorker>> = const { RefCell::new(None) };
    }

    pub(crate) fn start_source_resolution(
        prepared: crate::workbench::hardcopy_adapters::sources::PreparedRetainedHardcopyResolution,
        source_key: String,
        scope: crate::hardcopy::HardcopyScope,
        epoch: u64,
        generation: u64,
        repaint: egui::Context,
    ) -> Result<HardcopyWorkerTicket, String> {
        let snapshot = prepared
            .into_worker_snapshot_json()
            .map_err(|error| error.to_string())?;
        start(
            epoch,
            generation,
            HardcopyWorkerCommand::ResolveSource { source_key, scope },
            vec![snapshot],
            repaint,
        )
    }

    pub(crate) fn start_preview(
        plan: &crate::hardcopy::HardcopyPlan,
        source: &crate::workbench::hardcopy_adapters::sources::ResolvedHardcopyDocument,
        metadata: crate::workbench::hardcopy_adapters::render::HardcopySceneMetadata,
        page_indices: Vec<usize>,
        dpi: u16,
        epoch: u64,
        generation: u64,
        repaint: egui::Context,
    ) -> Result<HardcopyWorkerTicket, String> {
        let snapshot = source
            .worker_snapshot_json()
            .map_err(|error| error.to_string())?;
        start(
            epoch,
            generation,
            HardcopyWorkerCommand::Preview {
                plan_id: plan.id(),
                expected_plan_digest: plan.content_digest(),
                setup: plan.setup().clone(),
                metadata,
                page_indices,
                dpi,
            },
            vec![snapshot],
            repaint,
        )
    }

    pub(crate) fn start_publication(
        plan: &crate::hardcopy::HardcopyPlan,
        source: &crate::workbench::hardcopy_adapters::sources::ResolvedHardcopyDocument,
        metadata: crate::workbench::hardcopy_adapters::render::HardcopySceneMetadata,
        package_multi_part: bool,
        epoch: u64,
        generation: u64,
        repaint: egui::Context,
    ) -> Result<HardcopyWorkerTicket, String> {
        let snapshot = source
            .worker_snapshot_json()
            .map_err(|error| error.to_string())?;
        start(
            epoch,
            generation,
            HardcopyWorkerCommand::Publication {
                plan_id: plan.id(),
                expected_plan_digest: plan.content_digest(),
                expected_part_count: publication_part_count(plan),
                package_multi_part,
                setup: plan.setup().clone(),
                metadata,
            },
            vec![snapshot],
            repaint,
        )
    }

    fn start(
        epoch: u64,
        generation: u64,
        command: HardcopyWorkerCommand,
        buffers: Vec<Vec<u8>>,
        repaint: egui::Context,
    ) -> Result<HardcopyWorkerTicket, String> {
        if ACTIVE_WORKER.with(|active| active.borrow().is_some()) {
            return Err("A browser hardcopy worker operation is already active.".to_owned());
        }
        let id = allocate_request_id();
        let request = HardcopyWorkerRequest::try_new(id, epoch, generation, command)?;
        let operation = request.operation();
        let expected_buffer_count = Some(request.expected_response_buffer_count());
        let metadata = serde_wasm_bindgen::to_value(&request)
            .map_err(|error| format!("Could not encode hardcopy worker request: {error}"))?;
        let worker_url = worker_url()?;
        let options = web_sys::WorkerOptions::new();
        options.set_type(web_sys::WorkerType::Module);
        let worker =
            web_sys::Worker::new_with_options(&worker_url, &options).map_err(js_error_message)?;
        let ticket = HardcopyWorkerTicket {
            id,
            epoch,
            generation,
            operation,
            expected_buffer_count,
        };
        let result = Rc::new(RefCell::new(None));

        let message_result = Rc::clone(&result);
        let message_repaint = repaint.clone();
        let onmessage = Closure::<dyn FnMut(web_sys::MessageEvent)>::wrap(Box::new(
            move |event: web_sys::MessageEvent| {
                let data = event.data();
                let message_type = string_property(&data, "type");
                if message_type.as_deref() == Some("hardcopy-result") {
                    if numeric_property(&data, "id") != Some(id) {
                        complete_once(
                            &message_result,
                            &message_repaint,
                            Err(
                                "Browser hardcopy worker returned a stale outer response id."
                                    .to_owned(),
                            ),
                        );
                        return;
                    }
                    let parsed = Reflect::get(&data, &JsValue::from_str("response"))
                        .map_err(js_error_message)
                        .and_then(|response| parse_response(&response, ticket));
                    complete_once(&message_result, &message_repaint, parsed);
                } else if matches!(
                    message_type.as_deref(),
                    Some("hardcopy-error") | Some("error")
                ) {
                    let response_id = numeric_property(&data, "id").unwrap_or(0);
                    if response_id != id && response_id != 0 {
                        complete_once(
                            &message_result,
                            &message_repaint,
                            Err("Browser hardcopy worker returned a stale outer error id."
                                .to_owned()),
                        );
                        return;
                    }
                    complete_once(
                        &message_result,
                        &message_repaint,
                        Err(string_property(&data, "error")
                            .or_else(|| string_property(&data, "message"))
                            .unwrap_or_else(|| "Browser hardcopy worker failed.".to_owned())),
                    );
                } else if message_type.as_deref() != Some("ready") {
                    complete_once(
                        &message_result,
                        &message_repaint,
                        Err("Browser hardcopy worker returned an unexpected message.".to_owned()),
                    );
                }
            },
        ));
        worker.set_onmessage(Some(onmessage.as_ref().unchecked_ref()));

        let error_result = Rc::clone(&result);
        let error_repaint = repaint.clone();
        let onerror = Closure::<dyn FnMut(web_sys::ErrorEvent)>::wrap(Box::new(
            move |event: web_sys::ErrorEvent| {
                complete_once(
                    &error_result,
                    &error_repaint,
                    Err(if event.message().is_empty() {
                        "Browser hardcopy worker failed.".to_owned()
                    } else {
                        event.message()
                    }),
                );
            },
        ));
        worker.set_onerror(Some(onerror.as_ref().unchecked_ref()));

        let message_error_result = Rc::clone(&result);
        let message_error_repaint = repaint;
        let onmessageerror = Closure::<dyn FnMut(web_sys::MessageEvent)>::wrap(Box::new(
            move |_event: web_sys::MessageEvent| {
                complete_once(
                    &message_error_result,
                    &message_error_repaint,
                    Err("Browser hardcopy worker returned an unreadable message.".to_owned()),
                );
            },
        ));
        worker.set_onmessageerror(Some(onmessageerror.as_ref().unchecked_ref()));

        let message = Object::new();
        Reflect::set(
            &message,
            &JsValue::from_str("type"),
            &JsValue::from_str("run-hardcopy"),
        )
        .map_err(js_error_message)?;
        Reflect::set(
            &message,
            &JsValue::from_str("id"),
            &JsValue::from_f64(f64::from(id)),
        )
        .map_err(js_error_message)?;
        let request_value = Object::new();
        Reflect::set(&request_value, &JsValue::from_str("metadata"), &metadata)
            .map_err(js_error_message)?;
        let views = Array::new();
        let transfer = Array::new();
        for bytes in buffers {
            let view = Uint8Array::from(bytes.as_slice());
            transfer.push(&view.buffer());
            views.push(&view);
        }
        Reflect::set(&request_value, &JsValue::from_str("buffers"), &views)
            .map_err(js_error_message)?;
        Reflect::set(&message, &JsValue::from_str("request"), &request_value)
            .map_err(js_error_message)?;

        ACTIVE_WORKER.with(|active| {
            *active.borrow_mut() = Some(ActiveHardcopyWorker {
                ticket,
                worker: worker.clone(),
                result,
                _onmessage: onmessage,
                _onerror: onerror,
                _onmessageerror: onmessageerror,
                deadline: crate::time_compat::Instant::now()
                    + std::time::Duration::from_secs(match operation {
                        HardcopyWorkerOperation::ResolveSource
                        | HardcopyWorkerOperation::Preview => 120,
                        HardcopyWorkerOperation::Publication
                        | HardcopyWorkerOperation::PackagedPublication => 1_800,
                    }),
            });
        });
        if let Err(error) = worker.post_message_with_transfer(&message, &transfer) {
            cancel();
            return Err(format!(
                "Could not dispatch browser hardcopy work: {}",
                js_error_message(error)
            ));
        }
        Ok(ticket)
    }

    pub(crate) fn poll(expected: HardcopyWorkerTicket) -> Option<Result<Vec<Vec<u8>>, String>> {
        ACTIVE_WORKER.with(|active| {
            let mut active = active.borrow_mut();
            if active.as_ref().map(|worker| worker.ticket) != Some(expected) {
                return None;
            }
            if active
                .as_ref()
                .is_some_and(|worker| crate::time_compat::Instant::now() >= worker.deadline)
            {
                active.take();
                return Some(Err(
                    "Browser hardcopy worker exceeded its bounded execution deadline.".to_owned(),
                ));
            }
            let result = active
                .as_ref()
                .and_then(|worker| worker.result.borrow_mut().take());
            if result.is_some() {
                active.take();
            }
            result
        })
    }

    pub(crate) fn cancel() {
        ACTIVE_WORKER.with(|active| {
            active.borrow_mut().take();
        });
    }

    pub(crate) fn is_active() -> bool {
        ACTIVE_WORKER.with(|active| active.borrow().is_some())
    }

    fn parse_response(
        response: &JsValue,
        expected: HardcopyWorkerTicket,
    ) -> Result<Vec<Vec<u8>>, String> {
        if numeric_property(response, "protocolVersion") != Some(HARDCOPY_WORKER_PROTOCOL_VERSION) {
            return Err("Browser hardcopy worker returned an unsupported protocol.".to_owned());
        }
        if numeric_property(response, "id") != Some(expected.id) {
            return Err("Browser hardcopy worker returned a stale response id.".to_owned());
        }
        if string_property(response, "epoch") != Some(expected.epoch.to_string()) {
            return Err("Browser hardcopy worker returned a stale epoch.".to_owned());
        }
        if string_property(response, "generation") != Some(expected.generation.to_string()) {
            return Err("Browser hardcopy worker returned a stale generation.".to_owned());
        }
        if string_property(response, "operation").as_deref() != Some(expected.operation.as_str()) {
            return Err("Browser hardcopy worker returned the wrong operation.".to_owned());
        }
        let buffers = Reflect::get(response, &JsValue::from_str("buffers"))
            .map_err(js_error_message)?
            .dyn_into::<Array>()
            .map_err(|_| "Browser hardcopy response buffers are not an array.".to_owned())?;
        let buffer_count = buffers.length() as usize;
        let mut views = Vec::with_capacity(buffer_count);
        let mut lengths = Vec::with_capacity(buffer_count);
        for index in 0..buffers.length() {
            let view = buffers.get(index).dyn_into::<Uint8Array>().map_err(|_| {
                "Browser hardcopy response contains a non-Uint8Array buffer.".to_owned()
            })?;
            lengths.push(view.byte_length() as usize);
            views.push(view);
        }
        validate_response_buffer_lengths(
            expected.operation,
            expected.expected_buffer_count,
            lengths,
        )?;
        Ok(views.into_iter().map(|view| view.to_vec()).collect())
    }

    fn complete_once(
        slot: &RenderSlot,
        repaint: &egui::Context,
        result: Result<Vec<Vec<u8>>, String>,
    ) {
        let mut slot = slot.borrow_mut();
        if slot.is_none() {
            *slot = Some(result);
            repaint.request_repaint();
        }
    }

    fn allocate_request_id() -> u32 {
        NEXT_REQUEST_ID.with(|next| {
            let value = next.get().wrapping_add(1).max(1);
            next.set(value);
            value
        })
    }

    fn worker_url() -> Result<String, String> {
        Reflect::get(
            &js_sys::global(),
            &JsValue::from_str("__RSPICE_SIM_WORKER_URL"),
        )
        .map_err(js_error_message)?
        .as_string()
        .filter(|url| !url.trim().is_empty())
        .ok_or_else(|| "Browser hardcopy worker URL is unavailable.".to_owned())
    }

    fn string_property(value: &JsValue, property: &str) -> Option<String> {
        Reflect::get(value, &JsValue::from_str(property))
            .ok()
            .and_then(|value| value.as_string())
    }

    fn numeric_property(value: &JsValue, property: &str) -> Option<u32> {
        Reflect::get(value, &JsValue::from_str(property))
            .ok()
            .and_then(|value| value.as_f64())
            .filter(|value| {
                value.is_finite()
                    && *value >= 1.0
                    && *value <= f64::from(u32::MAX)
                    && value.fract() == 0.0
            })
            .map(|value| value as u32)
    }

    fn js_error_message(error: JsValue) -> String {
        error
            .as_string()
            .or_else(|| {
                Reflect::get(&error, &JsValue::from_str("message"))
                    .ok()
                    .and_then(|message| message.as_string())
            })
            .unwrap_or_else(|| "unknown JavaScript error".to_owned())
    }
}

#[cfg(target_arch = "wasm32")]
pub(crate) use browser::{
    HardcopyWorkerTicket, cancel, is_active, poll, start_preview, start_publication,
    start_source_resolution,
};

#[cfg(test)]
mod tests {
    use super::*;
    use crate::hardcopy::{
        BackgroundMode, Bleed, ColorMapping, DecorationSetup, FontPolicy,
        OutsideSheetContentPolicy, PageMargins, PhysicalPageSetup, RenderSetup, RenderTarget,
        ScaleMode, SchematicHardcopyExtent, SchematicHardcopySetup, TilingMode, TilingSetup,
        Watermark,
    };
    use crate::hardcopy::{
        HardcopyPlan, HardcopyPlanId, HardcopyScope, HardcopySetup, OutputFormat,
    };
    use crate::state::{Point, Wire};
    use crate::workbench::AppState;
    use crate::workbench::hardcopy_adapters::render::{
        HardcopyPreviewPage, RenderedHardcopyPublication,
    };
    use crate::workbench::hardcopy_adapters::sources::{
        PreparedRetainedHardcopyResolution, prepare_retained_hardcopy_resolution,
        resolve_retained_hardcopy_source,
    };
    use crate::workbench::state::WorkspaceDocumentId;
    use rspice_app_types::product::ContentDigest;
    use rspice_formats::zip::deterministic_stored_zip;
    use rspice_hardcopy::render::{HardcopyRenderer, HardcopySceneMetadata};
    use rspice_hardcopy::sources::ResolvedHardcopyDocument;
    use sha2::{Digest as _, Sha256};

    struct WorkerFixture {
        prepared: PreparedRetainedHardcopyResolution,
        source: ResolvedHardcopyDocument,
        source_key: String,
        scope: HardcopyScope,
    }

    fn worker_fixture() -> WorkerFixture {
        worker_fixture_with_wire_endpoint(80)
    }

    fn worker_fixture_with_wire_endpoint(endpoint_x: i32) -> WorkerFixture {
        let mut state = AppState::default();
        state
            .schematic
            .document_mut_for_test()
            .wires
            .push(Wire::segment(
                881,
                Point::new(-20, 5),
                Point::new(endpoint_x, 5),
            ));
        let active_view = state.workspace.content.active_view.clone();
        state
            .workbench
            .documents
            .activate(WorkspaceDocumentId::CellView(active_view));
        let source_key = format!(
            "project:{}:cell-view:{}",
            state.workspace.content.project.id().as_uuid(),
            state.workspace.content.active_key()
        );
        let scope = HardcopyScope::ActiveDocument;
        let source = resolve_retained_hardcopy_source(&state, &source_key, scope.clone())
            .expect("fixture source resolves");
        let prepared = prepare_retained_hardcopy_resolution(&state, &source_key, scope.clone())
            .expect("fixture source prepares");
        WorkerFixture {
            prepared,
            source,
            source_key,
            scope,
        }
    }

    fn two_page_setup() -> HardcopySetup {
        let defaults = HardcopySetup::default();
        let physical_page = PhysicalPageSetup::try_new(
            defaults.physical_page().paper().clone(),
            PageMargins::uniform(crate::hardcopy::Length::ZERO),
            Bleed::None,
            defaults.physical_page().orientation(),
        )
        .expect("zero-margin worker fixture page is valid");
        let tiling = TilingSetup::try_new(
            TilingMode::Manual {
                columns: 2,
                rows: 1,
            },
            defaults.tiling().overlap(),
            false,
        )
        .expect("two-page tiling is valid");
        let decorations = DecorationSetup::try_new(false, false, false, Watermark::None)
            .expect("undecorated worker fixture is valid");
        let schematic = SchematicHardcopySetup::new(
            SchematicHardcopyExtent::CompleteSchematicContent,
            OutsideSheetContentPolicy::ExtendOutput,
            true,
            true,
            true,
            true,
            true,
            false,
        );
        HardcopySetup::try_new_with_schematic(
            physical_page,
            ScaleMode::EngineeringOneToOne,
            tiling,
            defaults.render().clone(),
            decorations,
            schematic,
            defaults.print_mapping().clone(),
        )
        .expect("two-page setup is valid")
    }

    fn two_page_svg_setup() -> HardcopySetup {
        let defaults = two_page_setup();
        HardcopySetup::try_new_with_schematic(
            defaults.physical_page().clone(),
            defaults.scale(),
            defaults.tiling(),
            RenderSetup::try_new(
                RenderTarget::ExportArtifact,
                OutputFormat::SvgVector,
                ColorMapping::PrintSafeEngineeringPalette,
                BackgroundMode::White,
                FontPolicy::new(true, true),
                true,
            )
            .expect("SVG render setup is valid"),
            defaults.decorations().clone(),
            defaults.schematic(),
            defaults.print_mapping().clone(),
        )
        .expect("two-page SVG setup is valid")
    }

    fn fixture_plan(source: &ResolvedHardcopyDocument, setup: HardcopySetup) -> HardcopyPlan {
        let plan_id = HardcopyPlanId::new();
        let sections = source
            .hardcopy_sections_for_setup(crate::hardcopy::SchematicHardcopySetup::default())
            .expect("fixture sections resolve");
        if sections.is_empty() {
            HardcopyPlan::compile_with_id(
                plan_id,
                source.authority().clone(),
                setup,
                source.content_extent(),
            )
        } else {
            HardcopyPlan::compile_with_id_and_sections(
                plan_id,
                source.authority().clone(),
                setup,
                source.content_extent(),
                sections,
            )
        }
        .expect("fixture plan compiles")
    }

    fn fixture_metadata(source: &ResolvedHardcopyDocument) -> HardcopySceneMetadata {
        crate::workbench::hardcopy_adapters::render::source_metadata(source, "RSpice worker tests")
            .expect("fixture metadata is valid")
    }

    fn request(
        id: u32,
        epoch: u64,
        generation: u64,
        command: HardcopyWorkerCommand,
    ) -> HardcopyWorkerRequest {
        HardcopyWorkerRequest::try_new(id, epoch, generation, command)
            .expect("fixture request is valid")
    }

    fn execution_error(result: Result<HardcopyWorkerResponse, String>, context: &str) -> String {
        match result {
            Ok(_) => panic!("{context}"),
            Err(error) => error,
        }
    }

    #[test]
    fn resolve_source_preserves_exact_identity_and_response_ticket() {
        let fixture = worker_fixture();
        let expected = fixture.source.clone();
        let response = execute_request(
            request(
                31,
                44,
                55,
                HardcopyWorkerCommand::ResolveSource {
                    source_key: fixture.source_key,
                    scope: fixture.scope,
                },
            ),
            vec![
                fixture
                    .prepared
                    .into_worker_snapshot_json()
                    .expect("prepared snapshot encodes"),
            ],
        )
        .expect("source worker request succeeds");

        assert_eq!(
            response.protocol_version(),
            HARDCOPY_WORKER_PROTOCOL_VERSION
        );
        assert_eq!(response.id(), 31);
        assert_eq!(response.epoch(), "44");
        assert_eq!(response.generation(), "55");
        assert_eq!(response.operation(), HardcopyWorkerOperation::ResolveSource);
        let buffers = response.into_buffers();
        assert_eq!(buffers.len(), 1);
        let restored = ResolvedHardcopyDocument::from_worker_snapshot_json(&buffers[0])
            .expect("resolved worker snapshot decodes");
        assert_eq!(restored, expected);
    }

    #[test]
    fn resolve_source_rejects_requested_key_and_scope_mismatch() {
        let wrong_key = worker_fixture();
        let error = execution_error(
            execute_request(
                request(
                    1,
                    1,
                    1,
                    HardcopyWorkerCommand::ResolveSource {
                        source_key: format!("{}:other", wrong_key.source_key),
                        scope: wrong_key.scope,
                    },
                ),
                vec![
                    wrong_key
                        .prepared
                        .into_worker_snapshot_json()
                        .expect("prepared snapshot encodes"),
                ],
            ),
            "a different requested source key must fail",
        );
        assert!(error.contains("other than the requested retained identity"));

        let wrong_scope = worker_fixture();
        let error = execution_error(
            execute_request(
                request(
                    2,
                    1,
                    1,
                    HardcopyWorkerCommand::ResolveSource {
                        source_key: wrong_scope.source_key,
                        scope: HardcopyScope::CurrentSheet,
                    },
                ),
                vec![
                    wrong_scope
                        .prepared
                        .into_worker_snapshot_json()
                        .expect("prepared snapshot encodes"),
                ],
            ),
            "a different requested source scope must fail",
        );
        assert!(error.contains("other than the requested retained identity"));
    }

    #[test]
    fn preview_operation_returns_decodable_transfer_and_rejects_digest_mismatch() {
        let fixture = worker_fixture();
        let setup = HardcopySetup::default();
        let plan = fixture_plan(&fixture.source, setup.clone());
        let dpi = 72;
        let response = execute_request(
            request(
                90,
                12,
                34,
                HardcopyWorkerCommand::Preview {
                    plan_id: plan.id(),
                    expected_plan_digest: plan.content_digest(),
                    setup: setup.clone(),
                    metadata: fixture_metadata(&fixture.source),
                    page_indices: vec![0],
                    dpi,
                },
            ),
            vec![
                fixture
                    .source
                    .worker_snapshot_json()
                    .expect("source snapshot encodes"),
            ],
        )
        .expect("preview worker request succeeds");

        assert_eq!(response.operation(), HardcopyWorkerOperation::Preview);
        assert_eq!(response.id(), 90);
        assert_eq!(response.epoch(), "12");
        assert_eq!(response.generation(), "34");
        let buffers = response.into_buffers();
        assert_eq!(buffers.len(), 2);
        let preview = HardcopyPreviewPage::from_worker_transfer(
            &plan,
            &fixture.source,
            0,
            dpi,
            &buffers[0],
            buffers[1].clone(),
        )
        .expect("preview transfer authenticates and decodes");
        assert_eq!(preview.page_number(), 1);
        let manifest: serde_json::Value = serde_json::from_slice(&buffers[0]).unwrap();
        assert_eq!(manifest["dpi"].as_u64(), Some(u64::from(dpi)));
        assert!(!preview.rgba().is_empty());

        let mismatch = execution_error(
            execute_request(
                request(
                    91,
                    12,
                    35,
                    HardcopyWorkerCommand::Preview {
                        plan_id: plan.id(),
                        expected_plan_digest: ContentDigest::from_bytes([0xa5; 32]),
                        setup,
                        metadata: fixture_metadata(&fixture.source),
                        page_indices: vec![0],
                        dpi,
                    },
                ),
                vec![
                    fixture
                        .source
                        .worker_snapshot_json()
                        .expect("source snapshot encodes"),
                ],
            ),
            "a mismatched trusted plan digest must fail",
        );
        assert!(mismatch.contains("trusted plan digest"));
    }

    #[test]
    fn preview_operation_preserves_two_page_pair_order_and_count() {
        let fixture = worker_fixture_with_wire_endpoint(1_600);
        let setup = two_page_setup();
        let plan = fixture_plan(&fixture.source, setup.clone());
        assert_eq!(
            plan.pagination().pages().len(),
            2,
            "fixture must remain an exact two-page plan"
        );
        let dpi = 72;
        let response = execute_request(
            request(
                92,
                13,
                36,
                HardcopyWorkerCommand::Preview {
                    plan_id: plan.id(),
                    expected_plan_digest: plan.content_digest(),
                    setup,
                    metadata: fixture_metadata(&fixture.source),
                    page_indices: vec![0, 1],
                    dpi,
                },
            ),
            vec![
                fixture
                    .source
                    .worker_snapshot_json()
                    .expect("source snapshot encodes"),
            ],
        )
        .expect("two-page preview worker request succeeds");

        let buffers = response.into_buffers();
        assert_eq!(buffers.len(), 4);
        let previews = [0usize, 1]
            .into_iter()
            .map(|page_index| {
                let buffer_index = page_index * 2;
                HardcopyPreviewPage::from_worker_transfer(
                    &plan,
                    &fixture.source,
                    page_index,
                    dpi,
                    &buffers[buffer_index],
                    buffers[buffer_index + 1].clone(),
                )
                .expect("ordered preview pair authenticates")
            })
            .collect::<Vec<_>>();
        assert_eq!(
            previews
                .iter()
                .map(HardcopyPreviewPage::page_number)
                .collect::<Vec<_>>(),
            vec![1, 2]
        );
        assert_ne!(previews[0].coordinate(), previews[1].coordinate());
    }

    #[test]
    fn publication_operation_returns_exact_decodable_parts_and_rejects_count_mismatch() {
        let fixture = worker_fixture();
        let setup = HardcopySetup::default();
        let plan = fixture_plan(&fixture.source, setup.clone());
        let expected_part_count = publication_part_count(&plan);
        assert_eq!(expected_part_count, 1, "PDF publication is one artifact");

        let response = execute_request(
            request(
                101,
                22,
                33,
                HardcopyWorkerCommand::Publication {
                    plan_id: plan.id(),
                    expected_plan_digest: plan.content_digest(),
                    expected_part_count,
                    package_multi_part: false,
                    setup: setup.clone(),
                    metadata: fixture_metadata(&fixture.source),
                },
            ),
            vec![
                fixture
                    .source
                    .worker_snapshot_json()
                    .expect("source snapshot encodes"),
            ],
        )
        .expect("publication worker request succeeds");

        assert_eq!(response.operation(), HardcopyWorkerOperation::Publication);
        assert_eq!(response.id(), 101);
        assert_eq!(response.epoch(), "22");
        assert_eq!(response.generation(), "33");
        let buffers = response.into_buffers();
        assert_eq!(buffers.len(), expected_part_count + 1);
        let publication = RenderedHardcopyPublication::from_worker_transfer(
            &plan,
            &fixture.source,
            &buffers[0],
            buffers[1..].to_vec(),
        )
        .expect("publication transfer authenticates and decodes");
        assert_eq!(publication.format(), OutputFormat::PdfVector);
        assert_eq!(
            publication.page_count(),
            plan.pagination().pages().len() as u32
        );

        let mismatch = execution_error(
            execute_request(
                request(
                    102,
                    22,
                    34,
                    HardcopyWorkerCommand::Publication {
                        plan_id: plan.id(),
                        expected_plan_digest: plan.content_digest(),
                        expected_part_count: expected_part_count + 1,
                        package_multi_part: false,
                        setup,
                        metadata: fixture_metadata(&fixture.source),
                    },
                ),
                vec![
                    fixture
                        .source
                        .worker_snapshot_json()
                        .expect("source snapshot encodes"),
                ],
            ),
            "an inexact publication part count must fail",
        );
        assert!(mismatch.contains("part count does not match the exact plan"));
    }

    #[test]
    fn packaged_publication_is_deterministic_authority_bound_and_tamper_evident() {
        // Schematic coordinates are 254 um per unit. Extend the authenticated
        // source beyond one Letter-landscape printable viewport so this is a
        // real two-part SVG publication rather than a mislabeled fixture.
        let fixture = worker_fixture_with_wire_endpoint(1_420);
        let setup = two_page_svg_setup();
        let plan = fixture_plan(&fixture.source, setup.clone());
        let expected_part_count = publication_part_count(&plan);
        assert!(
            expected_part_count >= 2,
            "fixture must produce a multi-page SVG publication"
        );
        let metadata = fixture_metadata(&fixture.source);
        let response = execute_request(
            request(
                111,
                24,
                35,
                HardcopyWorkerCommand::Publication {
                    plan_id: plan.id(),
                    expected_plan_digest: plan.content_digest(),
                    expected_part_count,
                    package_multi_part: true,
                    setup,
                    metadata: metadata.clone(),
                },
            ),
            vec![
                fixture
                    .source
                    .worker_snapshot_json()
                    .expect("source snapshot encodes"),
            ],
        )
        .expect("packaged publication worker request succeeds");
        assert_eq!(
            response.operation(),
            HardcopyWorkerOperation::PackagedPublication
        );
        let buffers = response.into_buffers();
        assert_eq!(buffers.len(), 2);

        let packaged = decode_packaged_publication(&plan, &fixture.source, buffers.clone())
            .expect("packaged transfer authenticates");
        let (bytes, artifact, page_count) = packaged.into_parts();
        let independently_rendered =
            HardcopyRenderer::render_resolved(&plan, &fixture.source, metadata)
                .expect("independent publication renders");
        let expected_entries = independently_rendered
            .parts()
            .iter()
            .map(|part| (part.suggested_filename(), part.bytes()))
            .collect::<Vec<_>>();
        let expected_zip =
            deterministic_stored_zip(&expected_entries).expect("expected ZIP assembles");
        assert_eq!(
            bytes, expected_zip,
            "ZIP entry order, names, and bytes must match the ordered rendered publication"
        );
        assert_eq!(
            artifact.content_digest(),
            ContentDigest::from_bytes(Sha256::digest(&bytes).into())
        );
        assert_eq!(page_count, plan.pagination().pages().len() as u32);

        let mut tampered_bytes = buffers.clone();
        tampered_bytes[1][0] ^= 0x01;
        assert!(
            decode_packaged_publication(&plan, &fixture.source, tampered_bytes).is_err(),
            "tampered ZIP bytes must fail artifact authentication"
        );

        let mut manifest: serde_json::Value =
            serde_json::from_slice(&buffers[0]).expect("manifest decodes");
        manifest["planContentDigest"] = serde_json::to_value(ContentDigest::from_bytes(
            Sha256::digest(b"wrong hardcopy plan").into(),
        ))
        .expect("tampered digest serializes");
        let mut tampered_manifest = buffers;
        tampered_manifest[0] = serde_json::to_vec(&manifest).expect("tampered manifest encodes");
        assert!(
            decode_packaged_publication(&plan, &fixture.source, tampered_manifest).is_err(),
            "tampered authority metadata must fail closed"
        );
    }
}
