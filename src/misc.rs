use crate::{emittable::Emittable, symbol_context::SymbolContext};

pub struct Comment {
    text: String
}

impl Comment {
    pub fn new<S: AsRef<str>>(text: S) -> Self {
        Self {
            text: text.as_ref().to_string()
        }
    }
}

impl Emittable for Comment {
    fn emit(&self, _ctx: &mut SymbolContext) -> String {
        self.text
            .lines()
            .map(|line| format!("# {}", line))
            .collect::<Vec<_>>()
            .join("\n")
    }
}

pub struct Newline;

impl Emittable for Newline {
    fn emit(&self, _ctx: &mut SymbolContext) -> String {
        String::new()
    }
}
