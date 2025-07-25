#[allow(warnings)]
mod bindings;

use bindings::docs::adder::add::add;
use bindings::exports::docs::calculator::calculate::{Guest, Op};
use bindings::docs::calculator::res::Res;

struct Component;

impl Guest for Component {
    fn eval_expression(op: Op, x: u32, y: u32) -> u32 {
        let res = Res::new();
        res.write(x);
        res.write(y);
        match op {
            Op::Add => add(x, y),
        }
    }
}

bindings::export!(Component with_types_in bindings);
