use std::collections::HashMap;

#[derive(Debug, Clone)]
pub enum Expr {
    Static(String),
    Dynamic(String),
    Contextual {
        ctx_var: String,
        branches: Vec<String>,
    },
}

#[derive(Debug, Clone)]
pub struct Val {
    pub expr: Expr,
}

#[derive(Debug, Clone)]
pub enum AttrValue {
    Static(String),
    Contextual {
        ctx_var: String,
        values: Vec<String>,
    },
}

#[derive(Debug, Clone)]
pub struct Element {
    pub tag: String, // h1, h2, p, img, etc.
    pub val: Option<Val>,
    pub attrs: HashMap<String, AttrValue>,
    pub children: Vec<Nodes>,
}

#[derive(Debug, Clone)]
pub struct ForEach {
    pub vars: Vec<String>,
    pub iterable: Expr,
    pub body: Vec<Nodes>,
}

#[derive(Debug, Clone)]
pub struct Placeholder {
    pub name: String,
    pub alias: Option<String>,
}

#[derive(Debug, Clone)]
pub struct MainLabel {
    pub name: String,
    pub nodes: Vec<Nodes>,
}

#[derive(Debug, Clone)]
pub struct AliasBlock {
    pub alias: String,
    pub body: Vec<Nodes>,
}

#[derive(Debug, Clone)]
pub enum Nodes {
    Element(Element),
    Val(Val),
    Placeholder(Placeholder),
    ForEach(ForEach),
    AliasBlock(AliasBlock),
}

pub struct DuxaParser(pub String);

impl DuxaParser {
    pub fn parse(&mut self) -> Result<MainLabel, Box<dyn std::error::Error>> {
        todo!("...");
    }

    pub fn parse_label_declaration() -> () {
        todo!("Parse a label like `.main > footer` or `.internal > second-internal`")
    }
    // TODO:
    // fn parse_attrs(...)
    // fn parse_val(...)
    // fn parse_ctx_expr(...)
    // fn parse_loop_block(...)
}
