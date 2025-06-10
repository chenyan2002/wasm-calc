use wasmtime::component::{Component, Linker, ResourceTable};
use wasmtime::*;
use wasmtime_wasi::p2::{IoView, WasiCtx, WasiCtxBuilder, WasiView, add_to_linker_sync};

mod bindings;
use bindings::component::logger::trace::{Host, Mode, add_to_linker};

struct Logger {
    wasi_ctx: WasiCtx,
    resource_table: ResourceTable,
    logger: Vec<(String, String, String)>,
}
impl Host for Logger {
    fn get_mode(&mut self) -> Mode {
        Mode::Record
    }
    fn record(&mut self, method: String, input: String, output: String) {
        self.logger.push((method, input, output));
    }
    fn replay(&mut self, _method: String, _input: String) -> String {
        unreachable!()
    }
}

fn main() -> anyhow::Result<()> {
    let engine = Engine::default();
    let mut linker = Linker::<Logger>::new(&engine);
    add_to_linker(&mut linker, |logger| logger)?;
    add_to_linker_sync(&mut linker)?;
    let wasi = WasiCtxBuilder::new().inherit_stdio().inherit_args().build();
    let state = Logger {
        wasi_ctx: wasi,
        resource_table: ResourceTable::new(),
        logger: Vec::new(),
    };
    let mut store = Store::new(&engine, state);

    let component = Component::from_file(&engine, "../target/wasm32-wasip1/release/composed.wasm")?;
    let bindings = bindings::Logger::instantiate(&mut store, &component, &linker)?;
    bindings.docs_adder_add().call_add(&mut store, 3, 4)?;
    println!("{:?}", store.data().logger);
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
