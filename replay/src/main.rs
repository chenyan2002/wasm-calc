use wasmtime::component::{Component, Linker, ResourceTable, Resource};
use wasmtime::*;
use wasmtime_wasi::p2::{IoView, WasiCtx, WasiCtxBuilder, WasiView, add_to_linker_sync};

mod bindings;

use std::collections::{VecDeque, BTreeSet};
use bindings::docs::calculator::res::Res;

struct Logger {
    wasi_ctx: WasiCtx,
    resource_table: ResourceTable,
    logger: VecDeque<(String, String, String)>,
}
impl bindings::component::recorder::logging::Host for Logger {
    fn record(&mut self, method: String, input: String, output: String) {
        self.logger.push_back((method, input, output));
    }
    /*
    fn replay(&mut self, method: String, input: String) -> String {
        let res = self.logger.pop_front().unwrap();
        assert_eq!(res.0, method);
        assert_eq!(res.1, input);
        res.2
    }*/
}
impl bindings::docs::adder::add::Host for Logger {
    fn add(&mut self, a: u32, b: u32) -> u32 { a + b }
}

pub struct Set(BTreeSet<u32>);
impl bindings::docs::calculator::res::Host for Logger {}
impl bindings::docs::calculator::res::HostRes for Logger {
    fn new(&mut self) -> Resource<Res> {
        let id = self.resource_table.push(Set(BTreeSet::default())).unwrap();
        id
    }
    fn write(&mut self, res: Resource<Res>, x: u32) {
        debug_assert!(!res.owned());
        let obj: &mut Set = self.resource_table.get_mut(&res).unwrap();
        obj.0.insert(x);
        println!("{res:?}: {x}");
    }
    fn drop(&mut self, res: Resource<Res>) -> Result<()> {
        debug_assert!(res.owned());
        let _obj = self.resource_table.delete(res)?;
        Ok(())
    }
}

fn main() -> anyhow::Result<()> {
    let engine = Engine::default();
    let mut linker = Linker::<Logger>::new(&engine);
    bindings::component::recorder::logging::add_to_linker(&mut linker, |logger| logger)?;
    bindings::docs::adder::add::add_to_linker(&mut linker, |logger| logger)?;
    bindings::docs::calculator::res::add_to_linker(&mut linker, |logger| logger)?;
    add_to_linker_sync(&mut linker)?;
    let wasi = WasiCtxBuilder::new().inherit_stdio().inherit_args().build();
    let state = Logger {
        wasi_ctx: wasi,
        resource_table: ResourceTable::new(),
        logger: VecDeque::new(),
    };
    let mut store = Store::new(&engine, state);

    let component = Component::from_file(&engine, "../target/wasm32-wasip1/debug/composed.wasm")?;
    let bindings = bindings::Logger::instantiate(&mut store, &component, &linker)?;
    use bindings::exports::docs::calculator::calculate::Op;
    bindings.docs_calculator_calculate().call_eval_expression(&mut store, Op::Add, 3, 4)?;
    println!("Trace: {:?}", store.data().logger);
    //println!("Replay");
    //MODE.set(Mode::Replay);
    //let res = bindings.docs_adder_add().call_add(&mut store, 3, 4)?;
    //println!("{res}");
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
