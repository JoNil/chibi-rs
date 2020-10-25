enum TokenKind {
    Punct,
    Mum(i32),
    Eof,
}

struct Token<'a> {
    kind: TokenKind,
    location: &'a str,
}

pub fn tokenize<'a>(input: &'a str) -> Vec<Token<'a>> {
    let mut tokens = Vec::new();
    let mut p = input;

    while !p.is_empty() {
        if p.chars().next().unwrap().is_whitespace() {
            p = &p[1..];
            continue;
        }

        if p.chars().next().unwrap().is_ascii_digit() {

            p.chars().

        }
    }

    tokens
}
