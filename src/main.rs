//! 解析流程：
//! 1、我们应该判断是否输入了终止条件；有的话，直接返回；
//! 2、逐字节读取，判断token，直到结尾；
//! 3、将token转换为node，同时应用运算符优先级规则
//! 4、输出结果。
use std::error::Error;
use std::fmt;
use std::io::Stdin;

fn main() {
    let mut buf = String::new();
    let std_in: Stdin = std::io::stdin();
    while std_in.read_line(&mut buf).is_ok() {
        let pure_buf = buf.trim_end();
        // if receive stop signal, stop it.
        if "bye" == pure_buf {
            println!("bye");
            break;
        }

        // change buf as bytes, so we can convert it into tokens.
        let text = pure_buf.trim_end();
        let mut lexer = Lexer::new(text);
        while lexer.advance().is_ok() {
            println!("{} {} {} {} {}", lexer.cur_start, lexer.cur_end, lexer.prev_end ,lexer.token, lexer.content());
            if lexer.token == Token::EOF {
                break;
            }
        }
        
        println!("{:?}", text);
        buf.clear();
    }
}

#[derive(Copy, Clone, Debug, PartialEq, Eq, PartialOrd, Ord)]
pub enum Token {
    EOF,
    NumValue,
    NumTypedValue,
    Plus,
    Equal,
    Space,
}

impl fmt::Display for Token {
    fn fmt<'f>(&self, formatter: &mut fmt::Formatter<'f>) -> Result<(), fmt::Error> {
        use Token::*;
        let s = match *self {
            EOF => "[end-of-file]",
            NumValue => "[Num]",
            NumTypedValue => "[NumTyped]",
            Plus => "+",
            Equal => "=",
            Space => "[space]",
            _ => {
                panic!("error token");
            }
        };
        fmt::Display::fmt(s, formatter)
    }
}

pub struct Lexer<'input> {
    text: &'input str,
    prev_end: usize,
    cur_start: usize,
    cur_end: usize,
    token: Token,
}

impl<'input> Lexer<'input> {
    pub fn new(text: &'input str) -> Lexer<'input> {
        Lexer {
            text,
            prev_end: 0,
            cur_start: 0,
            cur_end: 0,
            token: Token::EOF,
        }
    }

    pub fn peek(&self) -> Token {
        self.token
    }

    pub fn content(&self) -> &'input str {
        &self.text[self.cur_start..self.cur_end]
    }

    pub fn start_loc(&self) -> usize {
        self.cur_start
    }

    pub fn previous_end_loc(&self) -> usize {
        self.prev_end
    }

    pub fn advance(&mut self) -> Result<(), String> {
        self.prev_end = self.cur_end;
        self.cur_start = self.cur_end;
        let text = &self.text[self.cur_start..];
        let (tok, len) = find_token(text, self.cur_start)?;
        self.cur_end = self.cur_start + len;
        self.token = tok;
        Ok(())
    }
}

fn get_decimal_number(text: &str) -> (Token, usize) {
    let num_text_len = text
        .chars()
        .position(|c| !matches!(c, '0'..='9' | '_'))
        .unwrap_or(text.len());
    get_number_maybe_with_suffix(text, num_text_len)
}

// Return the length of the substring containing characters in [0-9a-fA-F].
fn get_hex_number(text: &str) -> (Token, usize) {
    let num_text_len = text
        .find(|c| !matches!(c, 'a'..='f' | 'A'..='F' | '0'..='9'| '_'))
        .unwrap_or(text.len());
    get_number_maybe_with_suffix(text, num_text_len)
}

// Given the text for a number literal and the length for the characters that match to the number
// portion, checks for a typed suffix.
fn get_number_maybe_with_suffix(text: &str, num_text_len: usize) -> (Token, usize) {
    let rest = &text[num_text_len..];
    if rest.starts_with("u8") {
        (Token::NumTypedValue, num_text_len + 2)
    } else if rest.starts_with("u64") || rest.starts_with("u16") || rest.starts_with("u32") {
        (Token::NumTypedValue, num_text_len + 3)
    } else if rest.starts_with("u128") || rest.starts_with("u256") {
        (Token::NumTypedValue, num_text_len + 4)
    } else {
        // No typed suffix
        (Token::NumValue, num_text_len)
    }
}

fn find_token(text: &str, start_offset: usize) -> Result<(Token, usize), String> {
    let c: char = match text.chars().next() {
        Some(next_char) => next_char,
        None => {
            return Ok((Token::EOF, 0));
        }
    };
    let (tok, len) = match c {
        '0'..='9' => {
            if text.starts_with("0x") && text.len() > 2 {
                let (tok, hex_len) = get_hex_number(&text[2..]);
                if hex_len == 0 {
                    (Token::NumValue, 1)
                } else {
                    (tok, 2+ hex_len)
                }
            } else {
                get_decimal_number(text)
            }
        }
        '+' => (Token::Plus, 1),
        '=' => {
            // if text.starts_with() { }
            (Token::Equal, 1)
        }
        ' ' => (Token::Space, 1),
        _ => {
            return Err(format!("Invalid character: '{}'", c));
        }
    };
    Ok((tok, len))
}



// is empty char
fn is_blank(ch: char) -> bool {
    return ch == ' ' || ch == '\t' || ch == '\n';
}
// is number
fn is_number(ch: char) -> bool {
    return ch >= '0' && ch <= '9'
}
