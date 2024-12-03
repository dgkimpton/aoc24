use crate::calculator::{ElvishCalculator, Operation};
use std::collections::VecDeque;

#[derive(Debug, PartialEq)]
enum TokenState {
    BEGIN,
    M,
    U,
    L,
    OpenParen,
    Arg1,
    Comma,
    Arg2,
    CloseParen,

    D,
    O,
    OpenParenDo,
    CloseParenDo,
    N,
    Appostrophe,
    T,
    OpenParenDont,
    CloseParenDont,
}

#[derive(Debug, PartialEq)]
enum ParseResult {
    AcceptedChar,
    ConsumedUpTillNow,
    RejectedChar,
}

#[derive(Debug)]
pub struct ElvishMachineLanguageParser<'a> {
    support_enabled_toggle: bool,
    calculator: &'a mut ElvishCalculator,

    state: TokenState,
    partial_token: String,
    arg1: Option<i32>,
    arg2: Option<i32>,
}

impl<'a> ElvishMachineLanguageParser<'a> {
    pub fn new(calculator: &'a mut ElvishCalculator, support_enabled_toggle: bool) -> Self {
        Self {
            calculator,
            support_enabled_toggle,
            state: TokenState::BEGIN,
            arg1: None,
            arg2: None,
            partial_token: String::new(),
        }
    }

    pub fn load_string(&mut self, input: &str) {
        let mut char_source = input.chars();

        let mut read_behind_buffer = VecDeque::<char>::new();
        let mut i = 0;

        loop {
            if i >= read_behind_buffer.len() {
                match char_source.next() {
                    Some(c) => read_behind_buffer.push_back(c),
                    None => break,
                }
            }

            let c = read_behind_buffer[i];

            match self.load(c) {
                ParseResult::AcceptedChar => {
                    i = i + 1;
                }
                ParseResult::ConsumedUpTillNow => {
                    read_behind_buffer = read_behind_buffer.split_off(i);
                    i = 0;
                }
                ParseResult::RejectedChar => {
                    read_behind_buffer.remove(0);
                    i = 0;
                }
            }
        }

        self.load(' '); // simulate eof
    }

    fn reset_state(&mut self) -> TokenState {
        self.arg1 = None;
        self.arg2 = None;
        self.partial_token.clear();
        TokenState::BEGIN
    }

    fn load(&mut self, c: char) -> ParseResult {
        self.state = match self.state {
            TokenState::BEGIN if c == 'm' => TokenState::M,
            TokenState::BEGIN if c == 'd' && self.support_enabled_toggle => TokenState::D,
            TokenState::BEGIN => TokenState::BEGIN,

            TokenState::M if c == 'u' => TokenState::U,
            TokenState::M => TokenState::BEGIN,

            TokenState::U if c == 'l' => TokenState::L,
            TokenState::U => TokenState::BEGIN,

            TokenState::L if c == '(' => TokenState::OpenParen,
            TokenState::L => TokenState::BEGIN,

            TokenState::OpenParen if c.is_ascii_digit() => {
                self.state = TokenState::Arg1;
                self.load(c);
                TokenState::Arg1
            }
            TokenState::OpenParen => self.reset_state(),

            TokenState::Arg1 if self.partial_token.len() < 3 && c.is_ascii_digit() => {
                self.partial_token.push(c);
                TokenState::Arg1
            }
            TokenState::Arg1 if c == ',' => {
                let next_state = match self.partial_token.parse::<i32>() {
                    Ok(num) => {
                        self.arg1 = Some(num);
                        TokenState::Comma
                    }
                    Err(_) => self.reset_state(),
                };
                self.partial_token.clear();
                next_state
            }
            TokenState::Arg1 => self.reset_state(),

            TokenState::Comma if c.is_ascii_digit() => {
                self.state = TokenState::Arg2;
                self.load(c);
                TokenState::Arg2
            }
            TokenState::Comma => self.reset_state(),

            TokenState::Arg2 if self.partial_token.len() < 3 && c.is_ascii_digit() => {
                self.partial_token.push(c);
                TokenState::Arg2
            }
            TokenState::Arg2 if c == ')' => {
                let next_state = match self.partial_token.parse::<i32>() {
                    Ok(num) => {
                        self.arg2 = Some(num);
                        TokenState::CloseParen
                    }
                    Err(_) => TokenState::BEGIN,
                };
                self.partial_token.clear();
                next_state
            }

            TokenState::Arg2 => self.reset_state(),

            TokenState::D if c == 'o' => TokenState::O,
            TokenState::D => TokenState::BEGIN,

            TokenState::O if c == 'n' => TokenState::N,
            TokenState::O if c == '(' => TokenState::OpenParenDo,
            TokenState::O => TokenState::BEGIN,

            TokenState::OpenParenDo if c == ')' => TokenState::CloseParenDo,
            TokenState::OpenParenDo => TokenState::BEGIN,

            TokenState::N if c == '\'' => TokenState::Appostrophe,
            TokenState::N => TokenState::BEGIN,

            TokenState::Appostrophe if c == 't' => TokenState::T,
            TokenState::Appostrophe => TokenState::BEGIN,

            TokenState::T if c == '(' => TokenState::OpenParenDont,
            TokenState::T => TokenState::BEGIN,

            TokenState::OpenParenDont if c == ')' => TokenState::CloseParenDont,
            TokenState::OpenParenDont => TokenState::BEGIN,

            TokenState::CloseParen => panic!("failed to consume mul"),
            TokenState::CloseParenDo => panic!("failed to consume do"),
            TokenState::CloseParenDont => panic!("failed to consume don't"),
        };

        match self.state {
            TokenState::BEGIN => ParseResult::RejectedChar,
            TokenState::CloseParen => {
                self.calculator
                    .enter(Operation::MUL(self.arg1.unwrap(), self.arg2.unwrap()));
                self.state = self.reset_state();
                ParseResult::ConsumedUpTillNow
            }
            TokenState::CloseParenDo => {
                self.calculator.enter(Operation::COND(true));
                self.state = TokenState::BEGIN;
                ParseResult::ConsumedUpTillNow
            }
            TokenState::CloseParenDont => {
                self.calculator.enter(Operation::COND(false));
                self.state = TokenState::BEGIN;
                ParseResult::ConsumedUpTillNow
            }
            _ => ParseResult::AcceptedChar,
        }
    }
}
