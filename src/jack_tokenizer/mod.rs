pub mod token;
use regex::Regex;

use self::token::{Token, TokenType};


pub struct JackTokenizer<'a> {
    tokens: Vec<Token<'a>>,
    current_token_number: i32,
    is_start: bool,
}

impl<'a> JackTokenizer<'a> {
    pub fn new(file_text: &str) -> JackTokenizer {
        JackTokenizer {
            tokens: JackTokenizer::parse_tokens(file_text),
            current_token_number: -1,
            is_start: false,
        }
    }

    fn parse_tokens(file_text: &str) -> Vec<Token<'a>> {
        let lines = JackTokenizer::get_valid_lines(file_text);
        let symbols = vec![
            '{', '}', '(', ')', '[', ']', '.', ',', ';',
            '+', '-', '*', '/', '&', '|', '<', '>', '=', '~'
        ];
        let mut tokens = Vec::new();

        for line in lines {
            let mut token_type = TokenType::Unknown;
            let mut token_start_index = 0;

            for ch in line.chars() {
                if symbols.contains(&ch) {
                    if token_type != TokenType::Unknown {

                    }
                } else if ch == ' ' {
                    
                } else if ch == '"' {
                    
                } else {
                    
                }
            }
        }
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
        self.current_line_number < self.lines.len() as i32 -1
    }

    pub fn advance(&self) {
        
    }
}

#[cfg(test)]
mod tests {
    use super::*;

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
        let tokenizer = JackTokenizer::new(symbol_text);

        verify_has_more_tokens(&tokenizer, symbol_text.len());
    }

    fn verify_has_more_tokens(tokenizer: &JackTokenizer, n: usize) {
        for _ in 0..n {
            assert!(tokenizer.has_more_tokens());
            tokenizer.advance()
        }
        assert!(!tokenizer.has_more_tokens());
    }
}
