mod generated {
    wasmtime::component::bindgen!({
        path: "wit",
        world: "logger",
        //include_generated_code_from_file: true,
    });
}

pub use self::generated::*;
