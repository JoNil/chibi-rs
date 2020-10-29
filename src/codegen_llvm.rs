use std::path::Path;

use crate::parser::Node;
use inkwell::{builder::Builder, context::Context, values::IntValue, IntPredicate};

fn gen_expr<'ctx>(node: &Node, context: &'ctx Context, builder: &Builder<'ctx>) -> IntValue<'ctx> {
    match node {
        Node::Num(val) => context.i32_type().const_int(*val as u64, false),
        Node::Neg(expr) => {
            let val = gen_expr(expr, context, builder);
            builder.build_int_neg(val, "main")
        }
        Node::Add(lhs, rhs) => {
            let lhs = gen_expr(lhs, context, builder);
            let rhs = gen_expr(rhs, context, builder);
            builder.build_int_add(lhs, rhs, "main")
        }
        Node::Sub(lhs, rhs) => {
            let lhs = gen_expr(lhs, context, builder);
            let rhs = gen_expr(rhs, context, builder);
            builder.build_int_sub(lhs, rhs, "main")
        }
        Node::Mul(lhs, rhs) => {
            let lhs = gen_expr(lhs, context, builder);
            let rhs = gen_expr(rhs, context, builder);
            builder.build_int_mul(lhs, rhs, "main")
        }
        Node::Div(lhs, rhs) => {
            let lhs = gen_expr(lhs, context, builder);
            let rhs = gen_expr(rhs, context, builder);
            builder.build_int_signed_div(lhs, rhs, "main")
        }
        Node::Eq(lhs, rhs) | Node::Ne(lhs, rhs) | Node::Lt(lhs, rhs) | Node::Le(lhs, rhs) => {
            let lhs = gen_expr(lhs, context, builder);
            let rhs = gen_expr(rhs, context, builder);
            builder.build_int_compare(
                match node {
                    Node::Eq(..) => IntPredicate::EQ,
                    Node::Ne(..) => IntPredicate::NE,
                    Node::Lt(..) => IntPredicate::SLT,
                    Node::Le(..) => IntPredicate::SLE,
                    _ => panic!("Unknown Compare"),
                },
                lhs,
                rhs,
                "main",
            )
        }
    }
}

pub fn codegen(node: &Node) {
    let context = Context::create();
    let module = context.create_module("main");

    let main_type = context.i32_type().fn_type(&[], false);
    let main = module.add_function("main", main_type, None);

    let basic_block = context.append_basic_block(main, "entry");

    let builder = context.create_builder();
    builder.position_at_end(basic_block);

    let res = gen_expr(node, &context, &builder);

    builder.build_return(Some(&res));

    module.write_bitcode_to_path(Path::new("out.bc"));
}
