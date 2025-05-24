#[allow(warnings)]
mod bindings;

use bindings::exports::docs::adder::add::Guest;
use bindings::component::logger::trace::{Mode, get_mode, record, replay};

struct Component;

impl Guest for Component {
    fn add(a: u32, b: u32) -> u32 {
        match get_mode() {
            Mode::Replay => {
                let res = replay("add", &format!("({a}, {b})"));
                res.parse().unwrap()
            }
            Mode::Record => {
                use bindings::docs::adder::add::add;
                let ret = add(a, b);
                record("add", &format!("({a}, {b})"), &format!("{ret}"));
                ret
            }
        }
    }
}

bindings::export!(Component with_types_in bindings);
