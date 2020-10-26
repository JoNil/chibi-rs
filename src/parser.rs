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

fn new_node<'a>(
    new_node: impl FnOnce(Node) -> AstNode,
    nt: (Node, &'a [Token<'a>]),
) -> (Node, &'a [Token<'a>]) {
    (Box::new(new_node(nt.0)), nt.1)
}

// expr = equality
fn expr<'a>(tokens: &'a [Token]) -> (Node, &'a [Token<'a>]) {
    equality(tokens)
}

// equality = relational ("==" relational | "!=" relational)*
fn equality<'a>(tokens: &'a [Token]) -> (Node, &'a [Token<'a>]) {
    let (mut node, mut tokens) = relational(tokens);

    loop {
        let nt = match tokens {
            [Token::Punct("=="), ..] => {
                new_node(move |rhs| AstNode::Eq(node, rhs), relational(&tokens[1..]))
            }
            [Token::Punct("!="), ..] => {
                new_node(move |rhs| AstNode::Ne(node, rhs), relational(&tokens[1..]))
            }
            _ => {
                return (node, tokens);
            }
        };

        node = nt.0;
        tokens = nt.1;
    }
}

// relational = add ("<" add | "<=" add | ">" add | ">=" add)*
fn relational<'a>(tokens: &'a [Token]) -> (Node, &'a [Token<'a>]) {
    let (mut node, mut tokens) = add(tokens);

    loop {
        let nt = match tokens {
            [Token::Punct("<"), ..] => {
                new_node(move |rhs| AstNode::Lt(node, rhs), add(&tokens[1..]))
            }
            [Token::Punct("<="), ..] => {
                new_node(move |rhs| AstNode::Le(node, rhs), add(&tokens[1..]))
            }
            [Token::Punct(">"), ..] => {
                new_node(move |lhs| AstNode::Lt(lhs, node), add(&tokens[1..]))
            }
            [Token::Punct(">="), ..] => {
                new_node(move |lhs| AstNode::Le(lhs, node), add(&tokens[1..]))
            }
            _ => {
                return (node, tokens);
            }
        };

        node = nt.0;
        tokens = nt.1;
    }
}

// add = mul ("+" mul | "-" mul)*
fn add<'a>(tokens: &'a [Token]) -> (Node, &'a [Token<'a>]) {
    let (mut node, mut tokens) = mul(tokens);

    loop {
        let nt = match tokens {
            [Token::Punct("+"), ..] => {
                new_node(move |rhs| AstNode::Add(node, rhs), mul(&tokens[1..]))
            }
            [Token::Punct("-"), ..] => {
                new_node(move |rhs| AstNode::Sub(node, rhs), mul(&tokens[1..]))
            }
            _ => {
                return (node, tokens);
            }
        };

        node = nt.0;
        tokens = nt.1;
    }
}

// mul = unary ("*" unary | "/" unary)*
fn mul<'a>(tokens: &'a [Token]) -> (Node, &'a [Token<'a>]) {
    let (mut node, mut tokens) = unary(tokens);

    loop {
        let nt = match tokens {
            [Token::Punct("*"), ..] => {
                new_node(move |rhs| AstNode::Mul(node, rhs), unary(&tokens[1..]))
            }
            [Token::Punct("/"), ..] => {
                new_node(move |rhs| AstNode::Div(node, rhs), unary(&tokens[1..]))
            }
            _ => {
                return (node, tokens);
            }
        };

        node = nt.0;
        tokens = nt.1;
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
