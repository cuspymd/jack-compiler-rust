

pub struct JackTokenizer {
    lines: Vec<String>,
    current_line_number: i32,    
}

impl JackTokenizer {
    pub fn new(file_text: &str) -> JackTokenizer {
        JackTokenizer {
            lines: JackTokenizer::get_valid_lines(file_text),
            current_line_number: -1,
        }
    }

    fn get_valid_lines(file_text: &str) -> Vec<String> {
        file_text
            .lines()
            .map(|line| JackTokenizer::get_valid_text(line))
            .filter(|line| !line.is_empty())
            .collect()
    }

    fn get_valid_text(text: &str) -> String {
        let valid_text: &str = match text.split("//").next() {
            Some(first_part) => first_part,
            None => text
        };

        valid_text.trim().to_string()
    }

    pub fn has_more_tokens(&self) -> bool {
        self.current_line_number < self.lines.len() as i32 -1
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
    fn test_has_more_tokens_given_multi_line_comments() {
        let tokenizer = JackTokenizer::new("/* comment\nreturn;\n*/");
        assert!(!tokenizer.has_more_tokens());
    }
}
