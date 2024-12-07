use std::fs::read_to_string;

struct Uninitialized;
struct Ready;

#[derive(Debug, PartialEq, Clone)]
enum Token {
    Number(i64),
    Comma,
    OpenParen,
    ClosedParen,
    Wordish(String),
    Undefined,
}

#[derive(Debug, PartialEq)]
enum SpliceState {
    Cut,
    Keep,
}

#[derive(Debug, PartialEq)]
struct TapeMachine<'a, State = Uninitialized> {
    tape: &'a Vec<char>,
    head: usize,
    tokenized_tape: Vec<Token>,
    spliced_tape: Vec<Token>,
    splice_from: Vec<Token>,
    splice_to: Vec<Token>,
    splice_state: SpliceState,
    _state: std::marker::PhantomData<State>,
}

impl<'a> TapeMachine<'a, Uninitialized> {
    fn from_tape(tape: &'a Vec<char>) -> Self {
        Self {
            tape,
            head: 0,
            tokenized_tape: Vec::new(),
            spliced_tape: Vec::new(),
            splice_from: Vec::new(),
            splice_to: Vec::new(),
            splice_state: SpliceState::Keep,
            _state: std::marker::PhantomData,
        }
    }

    fn with_splice_sequences(self, from: Vec<Token>, to: Vec<Token>) -> TapeMachine<'a, Ready> {
        TapeMachine {
            tape: self.tape,
            head: self.head,
            tokenized_tape: self.tokenized_tape,
            spliced_tape: self.spliced_tape,
            splice_from: from,
            splice_to: to,
            splice_state: self.splice_state,
            _state: std::marker::PhantomData,
        }
    }
}

impl<'a> TapeMachine<'a, Ready> {
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
            'a'..='z' | 'A'..='Z' | '\'' => {
                let word = self.discover_word();
                Some(Token::Wordish(word))
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
            if current_char.is_ascii_alphabetic() || current_char == '\'' {
                word.push(current_char);
                self.advance_head()
            } else {
                break;
            }
        }
        return word;
    }

    fn scan_splice_sequence(&self) -> Option<&[Token]> {
        let scan_length = self.splice_from.len().max(self.splice_to.len());
        if let Some(start) = self.tokenized_tape.len().checked_sub(scan_length) {
            Some(&self.tokenized_tape[start..self.tokenized_tape.len()])
        } else {
            None
        }
    }

    fn process(mut self) -> Vec<Token> {
        while let Some(token) = self.next_token() {
            match self.splice_state {
                SpliceState::Keep => {
                    self.tokenized_tape.push(token.clone());
                    self.spliced_tape.push(token);
                }
                SpliceState::Cut => {
                    self.tokenized_tape.push(token);
                }
            }
            if let Some(seq) = self.scan_splice_sequence() {
                self.splice_from
                    .iter()
                    .zip(seq)
                    .all(|(lhs, rhs)| match lhs {
                        &Token::Wordish(wrd) => {}
                    });
                if seq == self.splice_to {
                    self.splice_state = SpliceState::Cut;
                } else if seq == self.splice_from {
                    dbg!(seq);
                    self.splice_state = SpliceState::Keep;
                }
            }
        }
        self.spliced_tape
    }
}

fn main() {
    let content = read_to_string("src/day03/input.txt")
        .expect("Could not read input.txt")
        .chars()
        .collect::<Vec<char>>();
    let from = vec![
        Token::Wordish("do".to_string()),
        Token::OpenParen,
        Token::ClosedParen,
    ];
    let to = vec![
        Token::Wordish("don't".to_string()),
        Token::OpenParen,
        Token::ClosedParen,
    ];
    let tm = TapeMachine::from_tape(&content).with_splice_sequences(from, to);
    let parsed = tm.process();
    // dbg!(&parsed);
    // let x = parsed
    //     .windows(6)
    //     .filter_map(|win| {
    //         let mut it = win.iter();
    //         if let Some(Token::Wordish(wrd)) = it.next() {
    //             if wrd.contains(&"mul".to_string()) {
    //                 if let Some(Token::OpenParen) = it.next() {
    //                     if let Some(Token::Number(x)) = it.next() {
    //                         if let Some(Token::Comma) = it.next() {
    //                             if let Some(Token::Number(y)) = it.next() {
    //                                 if let Some(Token::ClosedParen) = it.next() {
    //                                     return Some(x * y);
    //                                 }
    //                             }
    //                         }
    //                     }
    //                 }
    //             }
    //         }
    //         return None;
    //     })
    //     .sum::<i64>();
    // println!("{x}");
}
