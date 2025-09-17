use regex::Regex;
use std::fmt::{Display, Formatter};

extern crate regex;

#[derive(Debug)]
pub enum TokenKind {
    //keywords
    EndOfFunctionKeyWord,
    ConstKeyword,
    Let,
    If,
    Else,
    True,
    False,
    Func,
    //literals
    IntLiteral(i64),
    FuncName(String),
    VariableName(String),
    //identifiers
    ArrowFunction,
    LeftParen,
    RightParen,
    LeftCurly,
    RightCurly,
    SemiColon,
    TypeOf,
    SpecialType,
    Equals,
    //unidentified
    Unidentified,
    //Types
    String(String),
    Char,
    Number,
}

impl Display for TokenKind {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            TokenKind::SemiColon => write!(f, "SemiColon"),
            TokenKind::EndOfFunctionKeyWord => write!(f, "return"),
            TokenKind::ConstKeyword => write!(f, "const"),
            TokenKind::IntLiteral(n) => write!(f, "{}", n),
            TokenKind::Let => write!(f, "let"),
            TokenKind::If => write!(f, "if"),
            TokenKind::Else => write!(f, "else"),
            TokenKind::True => write!(f, "true"),
            TokenKind::False => write!(f, "false"),
            TokenKind::Func => write!(f, "func"),
            TokenKind::ArrowFunction => write!(f, "=>"),
            TokenKind::LeftParen => write!(f, "("),
            TokenKind::RightParen => write!(f, ")"),
            TokenKind::LeftCurly => write!(f, "{{"),
            TokenKind::RightCurly => write!(f, "}}"),
            TokenKind::Unidentified => write!(f, ""),
            TokenKind::SpecialType => write!(f, "::"),
            TokenKind::TypeOf => write!(f, ":"),
            TokenKind::FuncName(s) => write!(f, "{}", s),
            TokenKind::Equals => write!(f, "="),
            TokenKind::VariableName(s) => write!(f, "{}", s),
            TokenKind::String(s) => write!(f, "{}", s),
            TokenKind::Char => write!(f, "char"),
            TokenKind::Number => write!(f, "number"),
        }
    }
}

#[derive(Debug)]
pub struct TextSpan {
    size: usize,
    index: usize,
}

#[derive(Debug)]
pub struct Token {
    pub kind: TokenKind,
    pub span: TextSpan,
}

pub fn split_words_into_vec(stream_buffer: String) -> Vec<String> {
    let re = Regex::new(r"[^(;){:})\s]+|;+|:+|[(]+|[)]+|[{]+|[}]+").unwrap();
    let vec_words: Vec<String> = re
        .find_iter(&stream_buffer)
        .map(|m| m.as_str().to_owned())
        .collect();

    println!("cleaned words: {:?}", vec_words);

    return vec_words;
}

fn is_keyword(token: &str) -> bool {
    //split into char vec
    let characters: Vec<char> = token.chars().collect();

    if characters[0].is_alphabetic() {
        return true;
    }
    return false;
}

fn is_identifier(token: &str) -> bool {
    let characters: Vec<char> = token.chars().collect();

    if !characters[0].is_alphabetic() {
        return true;
    }
    return false;
}

pub fn tokenizer(stream_buffer: String) -> Vec<Token> {
    let mut tokens: Vec<Token> = Vec::new();
    let vec_words = split_words_into_vec(stream_buffer);
    let mut iter = vec_words.iter().enumerate(); // Use enumerate for index and value
    while let Some((original_idx, x)) = iter.next() {
        if let Ok(int_val) = x.parse::<i64>() {
            tokens.push(Token {
                kind: TokenKind::IntLiteral(int_val),
                span: TextSpan {
                    size: (std::mem::size_of::<i64>()),
                    index: (original_idx),
                },
            });
        } else if is_keyword(x.as_str()) {
            let kind = match x.as_str() {
                "let" => TokenKind::Let,
                "if" => TokenKind::If,
                "else" => TokenKind::Else,
                "true" => TokenKind::True,
                "false" => TokenKind::False,
                "func" => TokenKind::Func,
                "return" => TokenKind::EndOfFunctionKeyWord,
                "const" => TokenKind::ConstKeyword,
                _ => TokenKind::Unidentified, // Should ideally not be hit if is_keyword is accurate
            };
            tokens.push(Token {
                kind,
                span: TextSpan {
                    size: x.len(),
                    index: original_idx,
                },
            });
        }
        if x.as_str() == "func" {
            if let Some((func_name_idx, func_name_word)) = iter.next() {
                // This consumes the next word!
                tokens.push(Token {
                    kind: TokenKind::FuncName(func_name_word.to_string()),
                    span: TextSpan {
                        size: func_name_word.len(),
                        index: func_name_idx,
                    },
                });
            } else {
                // Handle error: "func" keyword without a function name
                eprintln!(
                    "Error: 'func' keyword found without a function name at index {}",
                    original_idx
                );
                tokens.push(Token {
                    kind: TokenKind::Unidentified, // Or a specific error token
                    span: TextSpan {
                        size: 0,
                        index: original_idx + 1, // Point to where the name should be
                    },
                });
            }
        }

        if x.as_str() == "let" || x.as_str() == "const" {
            if let Some((func_name_idx, func_name_word)) = iter.next() {
                // This consumes the next word!
                tokens.push(Token {
                    kind: TokenKind::VariableName(func_name_word.to_string()),
                    span: TextSpan {
                        size: func_name_word.len(),
                        index: func_name_idx,
                    },
                });
            } else {
                // Handle error: "const" or "let" keyword without a variable name
                eprintln!(
                    "Error: 'let'or 'const' keyword found without a variable name at index {}",
                    original_idx
                );
                tokens.push(Token {
                    kind: TokenKind::Unidentified, // Or a specific error token
                    span: TextSpan {
                        size: 0,
                        index: original_idx + 1, // Point to where the name should be
                    },
                });
            }
        } else if is_identifier(x.as_str()) {
            let kind = match x.as_str() {
                ":" => TokenKind::TypeOf,
                "::" => TokenKind::SpecialType,
                "(" => TokenKind::LeftParen,
                ")" => TokenKind::RightParen,
                "{" => TokenKind::RightCurly,
                "}" => TokenKind::LeftCurly,
                ";" => TokenKind::SemiColon,
                "=>" => TokenKind::ArrowFunction,
                "=" => TokenKind::Equals,
                _ => TokenKind::Unidentified, // Should not be hit if is_operator is exhaustive
            };
            tokens.push(Token {
                kind,
                span: TextSpan {
                    size: x.len(),
                    index: original_idx,
                },
            });
        }
        if x.as_str() == ":" {
            if let Some((func_name_idx, func_name_word)) = iter.next() {
                let kind = match func_name_word.as_str() {
                    "String" => TokenKind::String(x.to_string()),
                    "Number" => TokenKind::Number,
                    "Char" => TokenKind::Char,
                    _ => TokenKind::Unidentified,
                };
                tokens.push(Token {
                    kind,
                    span: TextSpan {
                        size: x.len(),
                        index: func_name_idx,
                    },
                })
            }
        }
    }

    println!("{:#?}", tokens);

    return tokens;
}
