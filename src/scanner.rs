use unicode_segmentation::UnicodeSegmentation;

use crate::token::{Span, Token, TokenType};

pub struct Scanner<'a> {
    source: Vec<&'a str>,
    current: usize,
    previous: usize,
    //line: usize,
}

impl<'a, 'b> Scanner<'a> {
    pub fn new(src: &'a str) -> Scanner<'a> {
        Scanner {
            source: src.graphemes(true).collect::<Vec<&str>>(),
            current: 0,
            previous: 0,
            //line: 1,
        }
    }

    pub fn scan_token(&mut self) -> Token {
        let c = self.skip_whitespace().advance();

        match c {
            Some("+") => self.make_token(TokenType::Plus),
            Some("-") => self.make_token(TokenType::Minus),
            Some("/") => self.make_token(TokenType::Slash),
            Some("*") => self.make_token(TokenType::Star),
            Some("(") => self.make_token(TokenType::LeftParen),
            Some(")") => self.make_token(TokenType::RightParen),
            None => self.make_token(TokenType::Eof),
            _ if is_alpha(c.unwrap()) => self.identifier(),
            _ if is_digit(c.unwrap()) => self.number(),
            _ => {
                let msg = format!("Unexpected character: '{}'", c.unwrap());
                return self.make_error_token(msg);
            }
        }
    }

    fn advance(&mut self) -> Option<&str> {
        self.current = self.current + 1;

        if self.current > self.source.len() {
            None
        } else {
            Some(self.source[self.current - 1])
        }
    }

    fn peek(&mut self) -> Option<&str> {
        let index = self.current + 1;

        if index > self.source.len() {
            None
        } else {
            Some(self.source[index - 1])
        }
    }

    fn make_token(&mut self, token_type: TokenType) -> Token {
        let token = match self.current {
            _ if self.current > self.source.len() => {
                let span = Span::new(self.previous, self.current);
                Token::new(token_type, "".to_string(), span)
            }
            _ => {
                let value = self.source[self.previous..self.current].join("");
                let span = Span::new(self.previous, self.current);
                self.previous = self.current;

                Token::new(token_type, value, span)
            }
        };

        token
    }

    fn make_error_token(&mut self, msg: String) -> Token {
        let span = Span::new(self.previous, self.current);
        Token::new(TokenType::Error, msg, span)
    }

    fn number(&mut self) -> Token {
        while self.peek() != None && is_digit(self.peek().unwrap()) {
            self.advance();
        }

        self.make_token(TokenType::Number)
    }

    fn identifier(&mut self) -> Token {
        while self.peek() != None && is_alpha(self.peek().unwrap()) {
            self.advance();
        }

        let token_type = self.identifier_type();
        self.make_token(token_type)
    }

    fn identifier_type(&mut self) -> TokenType {
        let value = &self.source[self.previous..self.current].join("");
        match &value[..] {
            "true" => TokenType::True,
            "false" => TokenType::False,
            _ => TokenType::Ident, 
        }
    }

    fn skip_whitespace(&mut self) -> &mut Self {
        while self.peek() != None && is_whitespace(self.peek().unwrap()) {
            self.advance();
        }
        self.previous = self.current;
        self
    }
}

fn is_digit(string: &str) -> bool {
    string.as_bytes()[0].is_ascii_digit()
}

fn is_whitespace(string: &str) -> bool {
    matches!(
        string,
        // Usual ASCII suspects
        "\u{0009}"   // \t
        | "\u{000B}" // vertical tab
        | "\u{000C}" // form feed
        | "\u{000D}" // \r
        | "\u{0020}" // space

        // NEXT LINE from latin1
        | "\u{0085}"

        // Bidi markers
        | "\u{200E}" // LEFT-TO-RIGHT MARK
        | "\u{200F}" // RIGHT-TO-LEFT MARK

        // Dedicated whitespace characters from Unicode
        | "\u{2028}" // LINE SEPARATOR
        | "\u{2029}" // PARAGRAPH SEPARATOR
    )
}

fn is_alpha(string: &str) -> bool {
    string
        .bytes()
        .all(|b| matches!(b, b'a'..=b'z' | b'A'..=b'Z' | b'_'))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_init_scanner() {
        let source = String::from("123猫");
        let scanner = Scanner::new(&source);
        assert_eq!(scanner.source, ["1", "2", "3", "猫"]);
    }

    #[test]
    fn test_op_token_type() {
        let src = String::from("+-*/");
        let mut scanner = Scanner::new(&src);
        let token = scanner.scan_token();
        assert_eq!(token.token_type, TokenType::Plus);
        assert_eq!(token.value, "+");
        let token = scanner.scan_token();
        assert_eq!(token.token_type, TokenType::Minus);
        assert_eq!(token.value, "-");
        let token = scanner.scan_token();
        assert_eq!(token.token_type, TokenType::Star);
        assert_eq!(token.value, "*");
        let token = scanner.scan_token();
        assert_eq!(token.token_type, TokenType::Slash);
        assert_eq!(token.value, "/");
        let token = scanner.scan_token();
        assert_eq!(token.token_type, TokenType::Eof);
        assert_eq!(token.value, "");
    }

    #[test]
    fn test_number_token_type() {
        let src = String::from("123");
        let mut scanner = Scanner::new(&src);
        let token = scanner.scan_token();
        assert_eq!(token.token_type, TokenType::Number);
        assert_eq!(token.value, "123");
        let token = scanner.scan_token();
        assert_eq!(token.token_type, TokenType::Eof);
        assert_eq!(token.value, "");
    }

    #[test]
    fn test_true_token() {
        let mut scanner = Scanner::new("true");
        let token = scanner.scan_token();
        assert_eq!(token.token_type, TokenType::True);
        assert_eq!(token.value, "true");
    }

    #[test]
    fn test_false_token() {
        let mut scanner = Scanner::new("false");
        let token = scanner.scan_token();
        assert_eq!(token.token_type, TokenType::False);
        assert_eq!(token.value, "false");
    }

    #[test]
    fn test_identifier_token() {
        let mut scanner = Scanner::new("radishes cats");
        let token = scanner.scan_token();
        assert_eq!(token.token_type, TokenType::Ident);
        assert_eq!(token.value, "radishes");
        let token = scanner.scan_token();
        assert_eq!(token.token_type, TokenType::Ident);
        assert_eq!(token.value, "cats");
    }

    #[test]
    fn test_skip_whitespace() {
        let src = String::from("    ");
        let mut scanner = Scanner::new(&src);
        assert_eq!(scanner.scan_token().token_type, TokenType::Eof);
        let src = String::from("\r\r\t");
        let mut scanner = Scanner::new(&src);
        assert_eq!(scanner.scan_token().token_type, TokenType::Eof);
        let src = String::from("  123    + 45  ");
        let mut scanner = Scanner::new(&src);
        assert_eq!(scanner.scan_token().token_type, TokenType::Number);
        assert_eq!(scanner.scan_token().token_type, TokenType::Plus);
        assert_eq!(scanner.scan_token().token_type, TokenType::Number);
        assert_eq!(scanner.scan_token().token_type, TokenType::Eof);
    }

    #[test]
    fn test_unexpected_token_type() {
        let src = String::from("猫");
        let mut scanner = Scanner::new(&src);
        let token = scanner.scan_token();
        assert_eq!(token.token_type, TokenType::Error);
        assert_eq!(token.value, "Unexpected character: '猫'");
        let token = scanner.scan_token();
        assert_eq!(token.token_type, TokenType::Eof);
        assert_eq!(token.value, "");
    }
    #[test]
    fn test_multiple_tokens() {
        let src = String::from("1 + 23 + 456");
        let mut scanner = Scanner::new(&src);
        assert_eq!(scanner.scan_token().token_type, TokenType::Number);
        assert_eq!(scanner.scan_token().token_type, TokenType::Plus);
        assert_eq!(scanner.scan_token().token_type, TokenType::Number);
        assert_eq!(scanner.scan_token().token_type, TokenType::Plus);
        assert_eq!(scanner.scan_token().token_type, TokenType::Number);
        assert_eq!(scanner.scan_token().token_type, TokenType::Eof);
    }

    #[test]
    fn test_parentheses() {
        let mut scanner = Scanner::new("123 (456 789)");
        assert_eq!(scanner.scan_token().token_type, TokenType::Number);
        let token = scanner.scan_token();
        assert_eq!(token.token_type, TokenType::LeftParen);
        assert_eq!(token.value, "(");
        assert_eq!(scanner.scan_token().token_type, TokenType::Number);
        assert_eq!(scanner.scan_token().token_type, TokenType::Number);
        let token = scanner.scan_token();
        assert_eq!(token.token_type, TokenType::RightParen);
        assert_eq!(token.value, ")");
        assert_eq!(scanner.scan_token().token_type, TokenType::Eof);
    }

    #[test]
    fn test_token_span() {
        let src = String::from("789 102 猫");
        //                      0123456789
        let mut scanner = Scanner::new(&src);
        assert_eq!(scanner.source.len(), 9);
        let token = scanner.scan_token(); //123
        assert_eq!(token.span.start, 0);
        assert_eq!(token.span.end, 3);
        let token = scanner.scan_token(); //456
        assert_eq!(token.span.start, 4);
        assert_eq!(token.span.end, 7);
        let token = scanner.scan_token(); //猫
        assert_eq!(token.span.start, 8);
        assert_eq!(token.span.end, 9);
        let token = scanner.scan_token(); //Eof
        assert_eq!(token.span.start, 9);
        assert_eq!(token.span.end, 10);
    }

    #[test]
    fn test_empty_file() {
        let src = String::from("");
        let mut scanner = Scanner::new(&src);
        let token = scanner.scan_token();
        assert_eq!(token.token_type, TokenType::Eof);
    }

    #[test]
    fn test_is_alpha() {
        assert_eq!(is_alpha("l"), true);
        assert_eq!(is_alpha("L"), true);
        assert_eq!(is_alpha("_"), true);
        assert_eq!(is_alpha("1"), false);
        assert_eq!(is_alpha("?"), false);
        assert_eq!(is_alpha("猫"), false);
    }
}
