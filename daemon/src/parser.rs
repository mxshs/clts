use std::rc::Rc;
use std::cell::RefCell;
use std::collections::HashMap;

use super::tokenizer::*;

pub struct Parser {
    tokenizer: Rc<RefCell<Tokenizer>>,
    current_token: Option<Token>,
    p_map: HashMap<TokenType, BP>
}

pub enum Node {
    Group(Group),
    String(String),
    Pair(Pair),
    Omit
}

pub struct Group {
    pub name: String,
    pub pairs: Vec<Pair>,
}

pub struct Pair {
    pub from: String,
    pub to: String
}

struct BP (u8, u8);

impl Parser {
    pub fn new(tokenizer: Tokenizer) -> Parser {
        Parser{
            tokenizer: Rc::new(RefCell::new(tokenizer)),
            current_token: None,
            p_map:HashMap::from([
                (TokenType::LBR, BP(0, 3)),
                (TokenType::RBR, BP(0, 3)),
                (TokenType::Value, BP(2, 1)),
                (TokenType::ARROW, BP(3, 2)),
                (TokenType::NEWL, BP(1, 0)),
            ]),
        }
    }

    pub fn parse_expression(&mut self, power: u8) -> Option<Node> {
        self.advance();
        let mut lhs = self.parse_prefix()?;

        while self.get_lbp() > power {
            self.advance();
            lhs = self.parse_infix(lhs);
        }

        Some(lhs)
    }
    
    pub fn get_lbp(&self) -> u8 {
        self.p_map[&self.current_token.as_ref().unwrap().token_type].0
    }

    pub fn _get_rbp(&self) -> u8 {
        self.p_map[&self.current_token.as_ref().unwrap().token_type].1
    }

    fn advance(&mut self) {
        self.current_token = self.tokenizer.borrow_mut().next();
    }

    pub fn parse_statement(&mut self) -> Option<Node> {
        let tok = self.current_token.as_ref()?;

        match tok.token_type {
            TokenType::LBR => {
                let group_name = match self.parse_expression(3) {
                    Some(Node::String(name)) => name,
                    _ => panic!("invalid type for group name"),
                };

                self.advance();
                self.advance();

                let mut pairs = vec!();

                loop {
                    match self.current_token.as_ref() {
                        Some(tok) if tok.token_type == TokenType::LBR => break,
                        None => break,
                        _ => (), 
                    }

                    if let Some(Node::Pair(pair)) = self.parse_expression(1) {
                        pairs.push(pair);
                    } else {
                        break
                    }
                }

                Some(Node::Group(Group{
                    name: group_name,
                    pairs,
                }))

            },
            _ => None
        }
    }

    pub fn parse_prefix(&mut self) -> Option<Node> {
        let Token { value, token_type } = self.current_token.as_mut()?;

        match token_type {
            TokenType::Value => Some(Node::String(value.to_string())),
            _ => Some(Node::Omit)
        }
    }

    pub fn parse_infix(&mut self, lhs: Node) -> Node {
        let tok = &self.current_token.as_ref().unwrap().token_type;

        match tok {
            TokenType::ARROW => {
                if let Node::String(lhs) = lhs {
                    if let Some(Node::String(rhs)) = self.parse_expression(3) {
                        return Node::Pair(Pair{from: lhs, to: rhs})
                    }
                }

                panic!("invalid construct");
            },
            _ => lhs
        }
    }

    pub fn parse(&mut self) -> Vec<Node> {
        let mut program = vec!();
        self.advance();

        while let Some(statement) = self.parse_statement() {
            program.push(statement);
        }

        program
    }
}

