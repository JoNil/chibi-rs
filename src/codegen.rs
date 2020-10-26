use crate::parser::Node;

struct CodeGen {
    out: String,
    depth: i32,
}

impl CodeGen {
    fn new() -> Self {
        Self {
            out: String::new(),
            depth: 0,
        }
    }

    fn emit(&mut self, asm: &str) {
        self.out.push_str(asm);
    }

    fn push(&mut self) {
        self.emit("  push %rax\n");
        self.depth += 1;
    }

    fn pop(&mut self, arg: &str) {
        self.emit(&format!("  pop {}\n", arg));
        self.depth -= 1;
    }

    fn gen_binary(&mut self, lhs: &Node, rhs: &Node) {
        self.gen_expr(rhs);
        self.push();
        self.gen_expr(lhs);
        self.pop("%rdi");
    }

    fn gen_expr(&mut self, node: &Node) {
        match node {
            Node::Num(val) => {
                self.emit(&format!("  mov ${}, %rax\n", val));
            }
            Node::Neg(lhs) => {
                self.gen_expr(lhs);
                self.emit("  neg %rax\n");
            }
            Node::Add(lhs, rhs) => {
                self.gen_binary(lhs, rhs);
                self.emit("  add %rdi, %rax\n");
            }
            Node::Sub(lhs, rhs) => {
                self.gen_binary(lhs, rhs);
                self.emit("  sub %rdi, %rax\n");
            }
            Node::Mul(lhs, rhs) => {
                self.gen_binary(lhs, rhs);
                self.emit("  imul %rdi, %rax\n");
            }
            Node::Div(lhs, rhs) => {
                self.gen_binary(lhs, rhs);
                self.emit("  cqo\n");
                self.emit("  idiv %rdi\n");
            }
            Node::Eq(lhs, rhs) | Node::Ne(lhs, rhs) | Node::Lt(lhs, rhs) | Node::Le(lhs, rhs) => {
                self.gen_binary(lhs, rhs);

                self.emit("  cmp %rdi, %rax\n");

                match node {
                    Node::Eq(..) => self.emit("  sete %al\n"),
                    Node::Ne(..) => self.emit("  setne %al\n"),
                    Node::Lt(..) => self.emit("  setl %al\n"),
                    Node::Le(..) => self.emit("  setle %al\n"),
                    _ => (),
                }

                self.emit("  movzb %al, %rax\n");
            }
        }
    }

    fn gen(self) -> String {
        assert!(self.depth == 0);
        self.out
    }
}

pub fn codegen(node: &Node) -> String {
    let mut gen = CodeGen::new();

    gen.emit("  .global main\n");
    gen.emit("main:\n");

    gen.gen_expr(node);
    gen.emit("  ret\n");

    gen.gen()
}
