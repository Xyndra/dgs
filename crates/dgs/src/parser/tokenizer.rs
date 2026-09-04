#[derive(Debug, Clone, PartialEq)]
pub enum Token {
    Num(f64),
    Var(String),
    Plus,
    Minus,
    Star,
    Slash,
    Caret,
    LParen,
    RParen,
    Comma,
    Eof,
}

pub struct Tokenizer {
    chars: Vec<char>,
    pos: usize,
}

impl Tokenizer {
    pub fn new(input: &str) -> Self {
        Tokenizer {
            chars: input.chars().collect(),
            pos: 0,
        }
    }

    fn peek(&self) -> Option<char> {
        self.chars.get(self.pos).copied()
    }

    fn advance(&mut self) -> Option<char> {
        let ch = self.chars.get(self.pos).copied();
        if ch.is_some() {
            self.pos += 1;
        }
        ch
    }

    fn skip_whitespace(&mut self) {
        while let Some(ch) = self.peek() {
            if ch.is_whitespace() {
                self.advance();
            } else {
                break;
            }
        }
    }

    fn read_number(&mut self) -> f64 {
        let start = self.pos;
        while let Some(ch) = self.peek() {
            if ch.is_ascii_digit() {
                self.advance();
            } else {
                break;
            }
        }
        if self.peek() == Some('.') {
            self.advance();
            while let Some(ch) = self.peek() {
                if ch.is_ascii_digit() {
                    self.advance();
                } else {
                    break;
                }
            }
        }
        let s: String = self.chars[start..self.pos].iter().collect();
        s.parse::<f64>().unwrap_or(0.0)
    }

    fn read_var(&mut self) -> String {
        let start = self.pos;
        while let Some(ch) = self.peek() {
            if ch.is_alphanumeric() || ch == '_' {
                self.advance();
            } else {
                break;
            }
        }
        self.chars[start..self.pos].iter().collect()
    }

    pub fn next_token(&mut self) -> Token {
        self.skip_whitespace();
        match self.peek() {
            None => Token::Eof,
            Some(ch) => {
                if ch.is_ascii_digit() || (ch == '.' && self.pos + 1 < self.chars.len() && self.chars[self.pos + 1].is_ascii_digit()) {
                    Token::Num(self.read_number())
                } else if ch.is_alphabetic() {
                    Token::Var(self.read_var())
                } else {
                    self.advance();
                    match ch {
                        '+' => Token::Plus,
                        '-' => Token::Minus,
                        '*' => Token::Star,
                        '/' => Token::Slash,
                        '^' => Token::Caret,
                        '(' => Token::LParen,
                        ')' => Token::RParen,
                        ',' => Token::Comma,
                        _ => Token::Eof,
                    }
                }
            }
        }
    }

    pub fn tokenize(&mut self) -> Vec<Token> {
        let mut tokens = Vec::new();
        loop {
            let tok = self.next_token();
            let is_eof = tok == Token::Eof;
            tokens.push(tok);
            if is_eof {
                break;
            }
        }
        tokens
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_basic_tokens() {
        let tokens = Tokenizer::new("1 + 2").tokenize();
        assert_eq!(
            tokens,
            vec![Token::Num(1.0), Token::Plus, Token::Num(2.0), Token::Eof]
        );
    }

    #[test]
    fn test_variables() {
        let tokens = Tokenizer::new("x + t").tokenize();
        assert_eq!(
            tokens,
            vec![
                Token::Var("x".into()),
                Token::Plus,
                Token::Var("t".into()),
                Token::Eof
            ]
        );
    }

    #[test]
    fn test_function_call() {
        let tokens = Tokenizer::new("sin(x)").tokenize();
        assert_eq!(
            tokens,
            vec![
                Token::Var("sin".into()),
                Token::LParen,
                Token::Var("x".into()),
                Token::RParen,
                Token::Eof
            ]
        );
    }

    #[test]
    fn test_decimal_number() {
        let tokens = Tokenizer::new("3.14").tokenize();
        assert_eq!(tokens, vec![Token::Num(3.14), Token::Eof]);
    }

    #[test]
    fn test_leading_dot() {
        let tokens = Tokenizer::new(".5").tokenize();
        assert_eq!(tokens, vec![Token::Num(0.5), Token::Eof]);
    }

    #[test]
    fn test_operators() {
        let tokens = Tokenizer::new("^ * /").tokenize();
        assert_eq!(
            tokens,
            vec![Token::Caret, Token::Star, Token::Slash, Token::Eof]
        );
    }

    #[test]
    fn test_whitespace_skipped() {
        let tokens = Tokenizer::new("  1   +   2  ").tokenize();
        assert_eq!(
            tokens,
            vec![Token::Num(1.0), Token::Plus, Token::Num(2.0), Token::Eof]
        );
    }

    #[test]
    fn test_comma() {
        let tokens = Tokenizer::new("f(a, b)").tokenize();
        assert_eq!(
            tokens,
            vec![
                Token::Var("f".into()),
                Token::LParen,
                Token::Var("a".into()),
                Token::Comma,
                Token::Var("b".into()),
                Token::RParen,
                Token::Eof
            ]
        );
    }

    #[test]
    fn test_empty_input() {
        let tokens = Tokenizer::new("").tokenize();
        assert_eq!(tokens, vec![Token::Eof]);
    }
}
