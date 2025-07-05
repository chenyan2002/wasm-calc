#[allow(warnings)]
mod bindings;

use bindings::component::recorder::logging::record;
use bindings::exports::docs::calculator::calculate::{Guest as CalcGuest, Op};

use wasm_wave::wasm::WasmValue;
use wasm_wave::{
    from_str, to_string,
    value::{Type, Value},
};

struct Component;

impl CalcGuest for Component {
    fn eval_expression(op: Op, x: u32, y: u32) -> u32 {
        let op_ty = Type::enum_ty(["Add"]).unwrap();
        let val_op = match op {
            Op::Add => Value::make_enum(&op_ty, "Add").unwrap(),
        };
        let input_ty = Type::tuple([op_ty, Type::U32, Type::U32]).unwrap();
        let input = Value::make_tuple(&input_ty, [val_op, x.into(), y.into()]).unwrap();
        record("eval_expression", &to_string(&input).unwrap(), "");

        use bindings::docs::calculator::calculate::{eval_expression, Op as ImportOp};
        let op = match op {
            Op::Add => ImportOp::Add,
        };
        let ret = eval_expression(op, x, y);
        let res: Value = ret.into();
        record("eval_expression", "", &to_string(&res).unwrap());
        ret
    }
}

bindings::export!(Component with_types_in bindings);
