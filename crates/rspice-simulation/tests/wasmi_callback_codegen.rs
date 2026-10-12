//! Keep the Windows unoptimized host-call codegen regression small enough to
//! run independently of the GUI. With LLVM FastISel on Rust 1.94/1.95 x64 MSVC,
//! Caller::get_export("memory") dereferenced address 6 instead of the instance.
//! Both accepted and refused guest reads must return through the host normally.

use wasmi::{Caller, Config, EnforcedLimits, Engine, Extern, Linker, Module, Store};

fn callback_read(pointer: i32, length: i32) -> Result<Vec<u8>, String> {
    let bytes = wat::parse_str(format!(
        r#"(module
            (import "rspice" "read" (func $read (param i32 i32) (result i32)))
            (memory (export "memory") 1 1)
            (data (i32.const 8) "verified")
            (func (export "derive") (result i32)
                i32.const {pointer} i32.const {length}
                call $read drop i32.const 0))"#
    ))
    .unwrap();
    let mut config = Config::default();
    config.consume_fuel(true);
    config.enforced_limits(EnforcedLimits::strict());
    let engine = Engine::new(&config);
    let module = Module::new(&engine, &bytes).unwrap();
    type HostResult = Option<Result<Vec<u8>, String>>;
    let mut store = Store::new(&engine, None::<Result<Vec<u8>, String>>);
    store.set_fuel(1_000_000).unwrap();
    let mut linker = Linker::new(&engine);
    linker
        .func_wrap(
            "rspice",
            "read",
            |mut caller: Caller<'_, HostResult>, pointer: i32, length: i32| -> i32 {
                let memory = caller
                    .get_export("memory")
                    .and_then(Extern::into_memory)
                    .expect("guest exports memory");
                let mut data = vec![0; usize::try_from(length).unwrap()];
                let result = memory
                    .read(&caller, usize::try_from(pointer).unwrap(), &mut data)
                    .map(|()| data)
                    .map_err(|error| error.to_string());
                let status = if result.is_ok() { 0 } else { -1 };
                *caller.data_mut() = Some(result);
                status
            },
        )
        .unwrap();
    let instance = linker.instantiate_and_start(&mut store, &module).unwrap();
    let entry = instance
        .get_typed_func::<(), i32>(&store, "derive")
        .unwrap();
    assert_eq!(entry.call(&mut store, ()).unwrap(), 0);
    store.data().clone().expect("host callback ran")
}

#[test]
fn callback_can_read_the_calling_instances_exported_memory() {
    assert_eq!(callback_read(8, 8).unwrap(), b"verified");
}

#[test]
fn callback_preserves_bounds_error_when_guest_ignores_status() {
    assert!(
        callback_read(65_530, 32)
            .unwrap_err()
            .contains("out of bounds memory access")
    );
}
