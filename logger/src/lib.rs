#[allow(warnings)]
mod bindings;

use bindings::component::logger::trace::{Mode, get_mode, record, replay};
use bindings::exports::docs::adder::add::Guest;

use wasm_wave::wasm::WasmValue;
use wasm_wave::{
    from_str, to_string,
    value::{Type, Value},
};

struct Component;

impl Guest for Component {
    fn add(a: u32, b: u32) -> u32 {
        match get_mode() {
            Mode::Replay => {
                let input: Value = (a, b).into();
                let res = replay("add", &to_string(&input).unwrap());
                from_str::<Value>(&Type::U32, &res).unwrap().unwrap_u32()
            }
            Mode::Record => {
                use bindings::docs::adder::add::add;
                let ret = add(a, b);
                let input: Value = (a, b).into();
                let res: Value = ret.into();
                record(
                    "add",
                    &to_string(&input).unwrap(),
                    &to_string(&res).unwrap(),
                );
                ret
            }
        }
    }
}

bindings::export!(Component with_types_in bindings);
