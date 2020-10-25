use crate::tokenizer::Token;

#[derive(Debug)]
pub enum AstNode {
    Add(Node, Node),
    Sub(Node, Node),
    Mul(Node, Node),
    Div(Node, Node),
    Neg(Node),
    Eq(Node, Node),
    Ne(Node, Node),
    Lt(Node, Node),
    Le(Node, Node),
    Num(i32),
}

pub type Node = Box<AstNode>;

fn is_op(tokens: &[Token], op: &str) -> bool {
    if let [Token::Punct(s), ..] = tokens {
        *s == op
    } else {
        false
    }
}

// expr = equality
fn expr<'a>(tokens: &'a [Token]) -> (Node, &'a [Token<'a>]) {
    equality(tokens)
}

// equality = relational ("==" relational | "!=" relational)*
fn equality<'a>(tokens: &'a [Token]) -> (Node, &'a [Token<'a>]) {
    let (mut node, mut tokens) = relational(tokens);

    loop {
        if is_op(tokens, "==") {
            let (rhs, rest) = relational(&tokens[1..]);
            node = Box::new(AstNode::Eq(node, rhs));
            tokens = rest;
            continue;
        }

        if is_op(tokens, "!=") {
            let (rhs, rest) = relational(&tokens[1..]);
            node = Box::new(AstNode::Ne(node, rhs));
            tokens = rest;
            continue;
        }

        return (node, tokens);
    }
}

// relational = add ("<" add | "<=" add | ">" add | ">=" add)*
fn relational<'a>(tokens: &'a [Token]) -> (Node, &'a [Token<'a>]) {
    let (mut node, mut tokens) = add(tokens);

    loop {
        if is_op(tokens, "<") {
            let (rhs, rest) = add(&tokens[1..]);
            node = Box::new(AstNode::Lt(node, rhs));
            tokens = rest;
            continue;
        }

        if is_op(tokens, "<=") {
            let (rhs, rest) = add(&tokens[1..]);
            node = Box::new(AstNode::Le(node, rhs));
            tokens = rest;
            continue;
        }

        if is_op(tokens, ">") {
            let (lhs, rest) = add(&tokens[1..]);
            node = Box::new(AstNode::Lt(lhs, node));
            tokens = rest;
            continue;
        }

        if is_op(tokens, ">=") {
            let (lhs, rest) = add(&tokens[1..]);
            node = Box::new(AstNode::Le(lhs, node));
            tokens = rest;
            continue;
        }

        return (node, tokens);
    }
}

// add = mul ("+" mul | "-" mul)*
fn add<'a>(tokens: &'a [Token]) -> (Node, &'a [Token<'a>]) {
    let (mut node, mut tokens) = mul(tokens);

    loop {
        if is_op(tokens, "+") {
            let (rhs, rest) = mul(&tokens[1..]);
            node = Box::new(AstNode::Add(node, rhs));
            tokens = rest;
            continue;
        }

        if is_op(tokens, "-") {
            let (rhs, rest) = mul(&tokens[1..]);
            node = Box::new(AstNode::Sub(node, rhs));
            tokens = rest;
            continue;
        }

        return (node, tokens);
    }
}

// mul = unary ("*" unary | "/" unary)*
fn mul<'a>(tokens: &'a [Token]) -> (Node, &'a [Token<'a>]) {
    let (mut node, mut tokens) = unary(tokens);

    loop {
        if is_op(tokens, "*") {
            let (rhs, rest) = unary(&tokens[1..]);
            node = Box::new(AstNode::Mul(node, rhs));
            tokens = rest;
            continue;
        }

        if is_op(tokens, "/") {
            let (rhs, rest) = unary(&tokens[1..]);
            node = Box::new(AstNode::Div(node, rhs));
            tokens = rest;
            continue;
        }

        return (node, tokens);
    }
}

// unary = ("+" | "-") unary
//       | primary
fn unary<'a>(tokens: &'a [Token]) -> (Node, &'a [Token<'a>]) {
    if is_op(tokens, "+") {
        return unary(&tokens[1..]);
    }

    if is_op(tokens, "-") {
        let (op, rest) = unary(&tokens[1..]);
        return (Box::new(AstNode::Neg(op)), rest);
    }

    primary(tokens)
}

// primary = "(" expr ")" | num
fn primary<'a>(tokens: &'a [Token]) -> (Node, &'a [Token<'a>]) {
    if is_op(tokens, "(") {
        let (node, rest) = expr(&tokens[1..]);

        if !is_op(rest, ")") {
            panic!("Expected: )");
        }

        return (node, &rest[1..]);
    }

    if let Token::Num(num) = tokens[0] {
        return (Box::new(AstNode::Num(num)), &tokens[1..]);
    }

    panic!("expected an expression");
}

pub fn parse(tokens: &[Token]) -> Node {
    let (node, rest) = expr(tokens);

    if !rest.is_empty() {
        panic!("Extra token");
    }

    node
}

#[cfg(test)]
mod test {
    #[test]
    fn test_parse() {
        use super::{parse, AstNode};
        use crate::tokenizer;

        let tokens = tokenizer::tokenize("2+2");

        let node = parse(&tokens);

        if let AstNode::Add(..) = *node {
        } else {
            panic!("Fail");
        }
    }
}
