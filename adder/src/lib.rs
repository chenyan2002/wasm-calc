mod bindings;

use bindings::exports::docs::adder::add::Guest;

struct Component;

impl Guest for Component {
    fn add(a: u32, b: u32) -> u32 {
        println!("Real adder being called");
        a + b
    }
}

bindings::export!(Component with_types_in bindings);
