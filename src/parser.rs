use crate::lexer::Token;
use anyhow::{Result, anyhow};
pub struct DuxaParser(pub Vec<Token>);

struct Context {
    context_value: i8,
    values: [String],
}
enum Component {
    Tag(H1),
    Placeholder(String),
    Fill,
    ChildLabels,
}
struct H1 {
    val: String,
    ctx: Box<Context>,
}
struct Label {
    alias: String,
    componets: Vec<Component>,
}

impl DuxaParser {
    pub fn parse(&mut self) -> Result<Label> {
        if self.0.len() == 0 {
            return Err(anyhow!("It's empty bro")); 
        }
        let mut tokens = self.0.iter();
        while let Some(token) = tokens.next() {
            match token {
                Token::Dot => {
                    if let Some(next_token) = tokens.next() {
                        match next_token {
                            Token::Ident(ident) => {}
                            _ => {
                                return Err(anyhow!("Can you dig it, sucka"));
                            }
                        }
                    }
                },
                Token::Ident(ident) => {
                    todo!()
                },
                _ => return Err(anyhow!("Not implemented"))

            }
        }
    }
    todo!();
}

    pub fn parse_label(&mut self) -> () {}
    // TODO:
    // fn parse_attrs(...)
    // fn parse_val(...)
    // fn parse_ctx_expr(...)
    // fn parse_loop_block(...)
}
