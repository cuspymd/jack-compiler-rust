use std::collections::HashMap;
use lazy_static::lazy_static;


#[derive(PartialEq, Debug)]
pub enum TokenType {
    Unknown,
    Keyword,
    Symbol,
    IntConst,
    StringConst,
    Identifier,
}

#[derive(PartialEq, Eq, Hash, Debug)]
pub enum KeywordType {
    Class,
    Method,
    Function,
    Constructor,
    Int,
    Boolean,
    Char,
    Void,
    Var,
    Static,
    Field,
    Let,
    Do,
    If,
    Else,
    While,
    Return,
    True,
    False,
    Null,
    This,
}

pub struct Token {
    token_type: TokenType,
    text: String,
}

impl Token {
    pub fn new(token_type: TokenType, token_text: &str) -> Token {
        let mut _token_type = token_type;

        if let TokenType::Identifier = _token_type {
            if STR_TO_KEYWORD_MAP.contains_key(token_text) {
                _token_type = TokenType::Keyword;
            } else if Token::is_numeric(token_text) {
                _token_type = TokenType::IntConst;
            }
        }

        Token {
            token_type: _token_type,
            text: token_text.to_string(),
        }
    }

    fn is_numeric(text: &str) -> bool {
        text.parse::<u16>().is_ok()
    }

    pub fn get_type(&self) -> &TokenType {
        &self.token_type
    }
    
    pub fn get_text(&self) -> &str {
        &self.text
    }
}

lazy_static! {
    pub static ref STR_TO_KEYWORD_MAP: HashMap<&'static str, KeywordType> = {
        let mut map = HashMap::new();
        map.insert("class", KeywordType::Class);
        map.insert("constructor", KeywordType::Constructor);
        map.insert("function", KeywordType::Function);
        map.insert("method", KeywordType::Method);
        map.insert("field", KeywordType::Field);
        map.insert("static", KeywordType::Static);
        map.insert("var", KeywordType::Var);
        map.insert("int", KeywordType::Int);
        map.insert("char", KeywordType::Char);
        map.insert("boolean", KeywordType::Boolean);
        map.insert("void", KeywordType::Void);
        map.insert("true", KeywordType::True);
        map.insert("false", KeywordType::False);
        map.insert("null", KeywordType::Null);
        map.insert("this", KeywordType::This);
        map.insert("let", KeywordType::Let);
        map.insert("do", KeywordType::Do);
        map.insert("if", KeywordType::If);
        map.insert("else", KeywordType::Else);
        map.insert("while", KeywordType::While);
        map.insert("return", KeywordType::Return);
        map
    };

    pub static ref KEYWORD_TO_STR_MAP: HashMap<KeywordType, &'static str> = {
        let mut map = HashMap::new();
        map.insert(KeywordType::Class, "class");
        map.insert(KeywordType::Constructor, "constructor");
        map.insert(KeywordType::Function, "function");
        map.insert(KeywordType::Method, "method");
        map.insert(KeywordType::Field, "field");
        map.insert(KeywordType::Static, "static");
        map.insert(KeywordType::Var, "var");
        map.insert(KeywordType::Int, "int");
        map.insert(KeywordType::Char, "char");
        map.insert(KeywordType::Boolean, "boolean");
        map.insert(KeywordType::Void, "void");
        map.insert(KeywordType::True, "true");
        map.insert(KeywordType::False, "false");
        map.insert(KeywordType::Null, "null");
        map.insert(KeywordType::This, "this");
        map.insert(KeywordType::Let, "let");
        map.insert(KeywordType::Do, "do");
        map.insert(KeywordType::If, "if");
        map.insert(KeywordType::Else, "else");
        map.insert(KeywordType::While, "while");
        map.insert(KeywordType::Return, "return");
        map
    };
}

