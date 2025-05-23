mod bindings;
use bindings::docs::calculator::calculate::{self, Op};

fn main() {
  let res = calculate::eval_expression(Op::Add, 2, 3);
  println!("{res}");
}

