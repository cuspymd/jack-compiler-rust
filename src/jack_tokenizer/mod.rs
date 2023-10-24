pub mod token;
use regex::Regex;

use self::token::{Token, TokenType};


pub struct JackTokenizer {
    tokens: Vec<Token>,
    current_token_number: i32,
}

impl JackTokenizer {
    pub fn new(file_text: &str) -> JackTokenizer {
        JackTokenizer {
            tokens: JackTokenizer::parse_tokens(file_text),
            current_token_number: -1,
        }
    }

    fn parse_tokens(file_text: &str) -> Vec<Token> {
        let lines = JackTokenizer::get_valid_lines(file_text);
        let symbols = vec![
            "{", "}", "(", ")", "[", "]", ".", ",", ";",
            "+", "-", "*", "/", "&", "|", "<", ">", "=", "~"
        ];
        let mut tokens = Vec::new();

        for line in lines {
            let mut token_type = TokenType::Unknown;
            let mut token_start_index = 0;

            for i in 0..line.len() {
                let ch = &line[i..i+1];

                if symbols.contains(&ch) {
                    if token_type != TokenType::Unknown {
                        tokens.push(Token::new(token_type, &line[token_start_index..i]));
                        token_type = TokenType::Unknown;
                    }

                    tokens.push(Token::new(TokenType::Symbol, &line[i..i+1]));
                } else if ch == " " {
                    if token_type == TokenType::StringConst {
                        continue;
                    }
                    if token_type != TokenType::Unknown {
                        tokens.push(Token::new(token_type, &line[token_start_index..i]));
                        token_type = TokenType::Unknown;
                    }
                } else if ch == "\"" {
                    if token_type == TokenType::Unknown {
                        token_type = TokenType::StringConst;
                        token_start_index = i;
                    } else {
                        tokens.push(Token::new(token_type, &line[token_start_index..i]));
                        token_type = TokenType::Unknown;
                    }
                } else {
                    if token_type == TokenType::Unknown {
                        token_type = TokenType::Identifier;
                        token_start_index = i;
                    }
                }
            }
            if token_type != TokenType::Unknown {
                tokens.push(Token::new(token_type, &line[token_start_index..line.len()]));
            }
        }
        tokens
    }

    fn get_valid_lines(file_text: &str) -> Vec<String> {
        let code_text = JackTokenizer::delete_comments(file_text);
        code_text
            .lines()
            .map(|line| JackTokenizer::get_valid_text(line))
            .filter(|line| !line.is_empty())
            .collect()
    }

    fn delete_comments(text: &str) -> String {
        let re = Regex::new(r"(/\*(?s).*?\*/)|(//.*)").unwrap();
        re.replace_all(text, "").into_owned()
    }

    fn get_valid_text(text: &str) -> String {
        text.trim().to_string()
    }

    pub fn has_more_tokens(&self) -> bool {
        self.current_token_number < self.tokens.len() as i32 -1
    }

    pub fn advance(&mut self) {
        self.current_token_number += 1;
    }

    pub fn token_type(&self) -> &TokenType {
        self.tokens[self.current_token_number as usize].get_type()
    }
}

#[cfg(test)]
mod tests {
    use super::{*, token::STR_TO_KEYWORD_MAP};

    #[test]
    fn test_has_more_tokens_given_empty_line() {
        let tokenizer = JackTokenizer::new("");
        assert!(!tokenizer.has_more_tokens());

        let tokenizer = JackTokenizer::new("	");
        assert!(!tokenizer.has_more_tokens());

        let tokenizer = JackTokenizer::new("\n   \n     \n");
        assert!(!tokenizer.has_more_tokens());

        let tokenizer = JackTokenizer::new("\r\n   \r\n     \r\n");
        assert!(!tokenizer.has_more_tokens());

        let tokenizer = JackTokenizer::new("\n   \n     return;\n");
        assert!(tokenizer.has_more_tokens());
    }

    #[test]
    fn test_has_more_tokens_given_one_line_comment() {
        let tokenizer = JackTokenizer::new("// comment");
        assert!(!tokenizer.has_more_tokens());
    }

    #[test]
    fn test_has_more_tokens_given_multi_line_comment() {
        let tokenizer = JackTokenizer::new("/* comment\nreturn;\n*/");
        assert!(!tokenizer.has_more_tokens());
    }

    #[test]
    fn test_has_more_tokens_given_symbols() {
        let symbol_text = "{}()[].,;+-*/&|<>=~";
        let mut tokenizer = JackTokenizer::new(symbol_text);

        verify_has_more_tokens(&mut tokenizer, symbol_text.len());
    }

    #[test]
    fn test_has_more_tokens_given_symbols_with_space() {
        let symbol_text = "{}()[] .,;+    -*/&   |<>=~";
        let mut tokenizer = JackTokenizer::new(symbol_text);

        verify_has_more_tokens(&mut tokenizer, 19);
    }

    #[test]
    fn test_has_more_tokens_given_symbols_with_newline() {
        let symbol_text = "{}()[].,;+\n-*/&\n|<>=~";
        let mut tokenizer = JackTokenizer::new(symbol_text);

        verify_has_more_tokens(&mut tokenizer, 19);
    }

    #[test]
    fn test_has_more_tokens_given_symbols_with_keyword() {
        let symbol_text = "return;";
        let mut tokenizer = JackTokenizer::new(symbol_text);

        verify_has_more_tokens(&mut tokenizer, 2);
    }

    #[test]
    fn test_has_more_tokens_given_if_statement() {
        let symbol_text = "if (num > 0) {\n  num = num + 1;\n  return num;";
        let mut tokenizer = JackTokenizer::new(symbol_text);

        verify_has_more_tokens(&mut tokenizer, 16);
    }

    #[test]
    fn test_has_more_tokens_given_string() {
        let symbol_text = "\"this is string\"";
        let mut tokenizer = JackTokenizer::new(symbol_text);

        verify_has_more_tokens(&mut tokenizer, 1);
    }

    #[test]
    fn test_has_more_tokens_given_etc() {
        let symbol_text = "function test(int a){\n var num;\n let num=2;}";
        let mut tokenizer = JackTokenizer::new(symbol_text);

        verify_has_more_tokens(&mut tokenizer, 16);
    }

    fn verify_has_more_tokens(tokenizer: &mut JackTokenizer, n: usize) {
        for _ in 0..n {
            assert!(tokenizer.has_more_tokens());
            tokenizer.advance()
        }
        assert!(!tokenizer.has_more_tokens());
    }

    #[test]
    fn test_token_type_given_keyword() {
        for &keyword in STR_TO_KEYWORD_MAP.keys() {
            let mut tokenizer = JackTokenizer::new(keyword);
            tokenizer.advance();
            assert_eq!(&TokenType::Keyword, tokenizer.token_type());
        }
    }
}
