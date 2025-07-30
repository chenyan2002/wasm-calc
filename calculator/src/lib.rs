#[allow(warnings)]
mod bindings;

use bindings::docs::adder::add::add;
use bindings::exports::docs::calculator::calculate::{Guest, Op, GuestHandle, Handle};
use bindings::docs::calculator::res::Res;

struct Component;

struct H;
impl GuestHandle for H {}

impl Guest for Component {
    type Handle = H;
    fn test(_x: Handle) {}
    fn eval_expression(op: Op, x: u32, y: u32) -> u32 {
        let res = Res::new();
        res.write(x);
        res.write(y);
        let res = Res::read(res);
        res.write(42);
        res.write(43);
        match op {
            Op::Add => add(x, y),
        }
    }
}

bindings::export!(Component with_types_in bindings);
