mod generated {
    wasmtime::component::bindgen!({
        path: "wit",
        world: "logger",
        with: {
          "docs:calculator/res/res": crate::Set,
        }
        //include_generated_code_from_file: true,
    });
}

pub use self::generated::*;
