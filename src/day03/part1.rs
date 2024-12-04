use std::{fs::read_to_string, i64};

#[derive(Debug)]
enum Token {
    Number(i64),
    Comma,
    OpenParen,
    ClosedParen,
    Word(String),
    Undefined,
}

struct TapeMachine<'a> {
    tape: &'a Vec<char>,
    head: usize,
}

impl<'a> TapeMachine<'a> {
    fn from_chars(chars: &'a Vec<char>) -> Self {
        Self {
            tape: chars,
            head: 0,
        }
    }

    fn get_char_at_head(&self) -> char {
        self.tape[self.head]
    }

    fn advance_head(&mut self) {
        self.head += 1;
    }

    fn next_token(&mut self) -> Option<Token> {
        if self.head >= self.tape.len() {
            return None;
        }

        let current_char = self.get_char_at_head();

        match current_char {
            '0'..='9' => {
                let number = self.discover_number();
                Some(Token::Number(number))
            }
            ',' => {
                self.advance_head();
                Some(Token::Comma)
            }
            '(' => {
                self.advance_head();
                Some(Token::OpenParen)
            }
            ')' => {
                self.advance_head();
                Some(Token::ClosedParen)
            }
            'a'..='z' | 'A'..='Z' => {
                let word = self.discover_word();
                Some(Token::Word(word))
            }
            _ => {
                self.advance_head();
                Some(Token::Undefined)
            }
        }
    }

    fn discover_number(&mut self) -> i64 {
        let mut number = String::new();
        while self.head < self.tape.len() {
            let current_char = self.get_char_at_head();
            if current_char.is_ascii_digit() {
                number.push(current_char);
                self.advance_head()
            } else {
                break;
            }
        }
        number.parse().unwrap_or(0)
    }

    fn discover_word(&mut self) -> String {
        let mut word = String::new();
        while self.head < self.tape.len() {
            let current_char = self.get_char_at_head();
            if current_char.is_ascii_alphabetic() {
                word.push(current_char);
                self.advance_head()
            } else {
                break;
            }
        }
        return word;
    }

    fn play(&mut self) -> Vec<Token> {
        let mut out: Vec<Token> = Vec::new();
        while let Some(token) = self.next_token() {
            out.push(token);
        }
        return out;
    }
}

fn main() {
    let content = read_to_string("src/day03/input.txt")
        .expect("Could not read input.txt")
        .chars()
        .collect::<Vec<char>>();
    let mut tm = TapeMachine::from_chars(&content);
    let parsed = tm.play();
    let x = parsed
        .windows(6)
        .filter_map(|win| {
            let mut it = win.iter();
            if let Some(Token::Word(wrd)) = it.next() {
                if wrd == &"mul".to_string() {
                    if let Some(Token::OpenParen) = it.next() {
                        if let Some(Token::Number(x)) = it.next() {
                            if let Some(Token::Comma) = it.next() {
                                if let Some(Token::Number(y)) = it.next() {
                                    if let Some(Token::ClosedParen) = it.next() {
                                        return Some(x * y);
                                    }
                                }
                            }
                        }
                    }
                }
            }
            return None;
        })
        .sum::<i64>();
    println!("{x}");
}
