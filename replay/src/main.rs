use wasmtime::component::{Component, Linker, ResourceTable};
use wasmtime::*;
use wasmtime_wasi::p2::{IoView, WasiCtx, WasiCtxBuilder, WasiView, add_to_linker_sync};

mod bindings;

struct Logger {
    wasi_ctx: WasiCtx,
    resource_table: ResourceTable,
    logger: Vec<String>,
}

fn main() -> anyhow::Result<()> {
    let engine = Engine::default();
    let mut linker = Linker::<Logger>::new(&engine);
    add_to_linker_sync(&mut linker)?;
    let wasi = WasiCtxBuilder::new().inherit_stdio().inherit_args().build();
    let state = Logger {
        wasi_ctx: wasi,
        resource_table: ResourceTable::new(),
        logger: Vec::new(),
    };
    let mut store = Store::new(&engine, state);

    let component = Component::from_file(&engine, "../target/wasm32-wasip1/release/composed.wasm")?;
    let instance = linker.instantiate(&mut store, &component)?;
    Ok(())
}

impl IoView for Logger {
    fn table(&mut self) -> &mut ResourceTable {
        &mut self.resource_table
    }
}
impl WasiView for Logger {
    fn ctx(&mut self) -> &mut WasiCtx {
        &mut self.wasi_ctx
    }
}
