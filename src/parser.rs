use crate::tokenizer::Token;

#[derive(Debug)]
pub enum Node {
    Add(Box<Node>, Box<Node>),
    Sub(Box<Node>, Box<Node>),
    Mul(Box<Node>, Box<Node>),
    Div(Box<Node>, Box<Node>),
    Neg(Box<Node>),
    Eq(Box<Node>, Box<Node>),
    Ne(Box<Node>, Box<Node>),
    Lt(Box<Node>, Box<Node>),
    Le(Box<Node>, Box<Node>),
    Num(i32),
}

fn is_op(rest: &[Token], op: &str) -> bool {
    if let [Token::Punct(s), ..] = rest {
        *s == op
    } else {
        false
    }
}

type NodeAndRest<'a> = (Box<Node>, &'a [Token<'a>]);

fn new_node(new_node: impl FnOnce(Box<Node>) -> Node, nr: NodeAndRest) -> NodeAndRest {
    (Box::new(new_node(nr.0)), nr.1)
}

// expr = equality
fn expr<'a>(rest: &'a [Token]) -> NodeAndRest<'a> {
    equality(rest)
}

// equality = cmp ("==" cmp | "!=" cmp)*
fn equality<'a>(rest: &'a [Token]) -> NodeAndRest<'a> {
    let (mut node, mut rest) = cmp(rest);

    loop {
        let nr = match rest {
            [Token::Punct("=="), ..] => new_node(move |rhs| Node::Eq(node, rhs), cmp(&rest[1..])),
            [Token::Punct("!="), ..] => new_node(move |rhs| Node::Ne(node, rhs), cmp(&rest[1..])),
            _ => {
                return (node, rest);
            }
        };

        node = nr.0;
        rest = nr.1;
    }
}

// cmp = add ("<" add | "<=" add | ">" add | ">=" add)*
fn cmp<'a>(rest: &'a [Token]) -> NodeAndRest<'a> {
    let (mut node, mut rest) = add(rest);

    loop {
        let nr = match rest {
            [Token::Punct("<"), ..] => new_node(move |rhs| Node::Lt(node, rhs), add(&rest[1..])),
            [Token::Punct("<="), ..] => new_node(move |rhs| Node::Le(node, rhs), add(&rest[1..])),
            [Token::Punct(">"), ..] => new_node(move |lhs| Node::Lt(lhs, node), add(&rest[1..])),
            [Token::Punct(">="), ..] => new_node(move |lhs| Node::Le(lhs, node), add(&rest[1..])),
            _ => {
                return (node, rest);
            }
        };

        node = nr.0;
        rest = nr.1;
    }
}

// add = mul ("+" mul | "-" mul)*
fn add<'a>(rest: &'a [Token]) -> NodeAndRest<'a> {
    let (mut node, mut rest) = mul(rest);

    loop {
        let nr = match rest {
            [Token::Punct("+"), ..] => new_node(move |rhs| Node::Add(node, rhs), mul(&rest[1..])),
            [Token::Punct("-"), ..] => new_node(move |rhs| Node::Sub(node, rhs), mul(&rest[1..])),
            _ => {
                return (node, rest);
            }
        };

        node = nr.0;
        rest = nr.1;
    }
}

// mul = unary ("*" unary | "/" unary)*
fn mul<'a>(rest: &'a [Token]) -> NodeAndRest<'a> {
    let (mut node, mut rest) = unary(rest);

    loop {
        let nr = match rest {
            [Token::Punct("*"), ..] => new_node(move |rhs| Node::Mul(node, rhs), unary(&rest[1..])),
            [Token::Punct("/"), ..] => new_node(move |rhs| Node::Div(node, rhs), unary(&rest[1..])),
            _ => {
                return (node, rest);
            }
        };

        node = nr.0;
        rest = nr.1;
    }
}

// unary = ("+" | "-") unary
//       | primary
fn unary<'a>(rest: &'a [Token]) -> NodeAndRest<'a> {
    if is_op(rest, "+") {
        return unary(&rest[1..]);
    }

    if is_op(rest, "-") {
        return new_node(Node::Neg, unary(&rest[1..]));
    }

    primary(rest)
}

// primary = "(" expr ")" | num
fn primary<'a>(rest: &'a [Token]) -> NodeAndRest<'a> {
    if is_op(rest, "(") {
        let (node, rest) = expr(&rest[1..]);

        if !is_op(rest, ")") {
            panic!("Expected: )");
        }

        return (node, &rest[1..]);
    }

    if let Token::Num(num) = rest[0] {
        return (Box::new(Node::Num(num)), &rest[1..]);
    }

    panic!("expected an expression");
}

pub fn parse(rest: &[Token]) -> Box<Node> {
    let (node, rest) = expr(rest);

    if !rest.is_empty() {
        panic!("Extra token");
    }

    node
}

#[cfg(test)]
mod test {
    #[test]
    fn test_parse() {
        use super::{parse, Node};
        use crate::tokenizer;

        let rest = tokenizer::tokenize("2+2");

        let node = parse(&rest);

        if let Node::Add(..) = *node {
        } else {
            panic!("Fail");
        }
    }
}
