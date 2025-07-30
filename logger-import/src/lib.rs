#[allow(warnings)]
mod bindings;

use bindings::component::recorder::logging::record;
use bindings::exports::docs::adder::add::Guest as AddGuest;

use wasm_wave::wasm::WasmValue;
use wasm_wave::{
    from_str, to_string,
    value::{Type, Value},
};

struct Component;

impl Into<bindings::docs::calculator::res::Res> for bindings::exports::docs::calculator::res::Res {
    fn into(self) -> bindings::docs::calculator::res::Res {
        self.into_inner()
    }
}
impl Into<bindings::exports::docs::calculator::res::Res> for bindings::docs::calculator::res::Res {
    fn into(self) -> bindings::exports::docs::calculator::res::Res {
        bindings::exports::docs::calculator::res::Res::new(self)
    }
}

impl bindings::exports::docs::calculator::res::GuestRes for bindings::docs::calculator::res::Res {
    fn new() -> Self {
        Self::new()
    }
    fn write(&self, x: u32) {
        //let resource = unsafe { Self::from_handle(self.take_handle()) };
        //resource.write(x)
        self.write(x)
    }
    fn read(x: bindings::exports::docs::calculator::res::Res) -> bindings::exports::docs::calculator::res::Res {
        //let ret = Self::read(x.into_inner());
        //bindings::exports::docs::calculator::res::Res::new(ret)
        Self::read(x.into()).into()
    }
}
impl bindings::exports::docs::calculator::res::Guest for Component {
    type Res = bindings::docs::calculator::res::Res;
}

impl AddGuest for Component {
    fn add(a: u32, b: u32) -> u32 {
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

bindings::export!(Component with_types_in bindings);
