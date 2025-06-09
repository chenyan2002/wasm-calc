mod generated {
    wasmtime::component::bindgen!({
        path: "wit",
        world: "logger",
        //tracing: true,
    });
}

pub use self::generated::*;
