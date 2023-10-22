use std::collections::HashMap;

pub enum TokenType {
    Unknown,
    Keyword,
    Symbol,
    IntConst,
    StringConst,
    Identifier,
}

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

pub struct Token<'a> {
    token_type: TokenType,
    text: String,
    str_to_keyword_map: HashMap<&'a str, KeywordType>,
    keyword_to_str_map: HashMap<KeywordType, &'a str>,
}

impl<'a> Token<'a> {
    // pub fn new()
    
}
