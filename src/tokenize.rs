#[derive(Debug)]
pub enum TokenKind {
    Punct,
    Num(i32),
}

#[derive(Debug)]
pub struct Token<'a> {
    pub kind: TokenKind,
    pub location: &'a str,
}

fn get_number(p: &str) -> &str {
    p.split(|c: char| !c.is_ascii_digit()).next().unwrap()
}

fn read_punct(p: &str) -> usize {
    if p.starts_with("==") || p.starts_with("!=") || p.starts_with("<=") || p.starts_with(">=") {
        2
    } else if p.chars().next().unwrap().is_ascii_punctuation() {
        1
    } else {
        0
    }
}

pub fn tokenize<'a>(input: &'a str) -> Vec<Token<'a>> {
    let mut tokens = Vec::new();
    let mut p = input;

    while !p.is_empty() {
        let next = p.chars().next().unwrap();

        if next.is_whitespace() {
            p = &p[next.len_utf8()..];
            continue;
        }

        if next.is_ascii_digit() {
            let number_str = get_number(p);
            let number = number_str.parse::<i32>().unwrap();
            tokens.push(Token {
                kind: TokenKind::Num(number),
                location: number_str,
            });
            p = &p[number_str.len()..];
            continue;
        }

        let punct_len = read_punct(p);
        if punct_len > 0 {
            tokens.push(Token {
                kind: TokenKind::Punct,
                location: &p[..punct_len],
            });
            p = &p[punct_len..];
            continue;
        }
    }

    tokens
}

#[cfg(test)]
mod test {
    #[test]
    fn test_get_number() {
        use super::get_number;

        assert_eq!(get_number("123   12  "), "123");
        assert_eq!(get_number("123     "), "123");
        assert_eq!(get_number("123a"), "123");
        assert_eq!(get_number("1"), "1");
    }

    #[test]
    fn test_tokenize() {
        use super::tokenize;

        let tokens = tokenize("123   12  ");
        assert_eq!(tokens.len(), 2);

        let tokens = tokenize("2+2 / 3");
        assert_eq!(tokens.len(), 5);
    }
}
