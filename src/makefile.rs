use crate::{
    emittable::EmittableRef,
    emitter::EmittableContainer,
    rrc::{rrc, RRC},
    symbol_context::SymbolContext
};
use std::fmt::Write;

#[derive(Default)]
pub struct Makefile {
    contents: Vec<EmittableRef>,
    ctx: RRC<SymbolContext>
}

impl Makefile {
    pub fn new() -> Self {
        Self {
            contents: Vec::new(),
            ctx: rrc(SymbolContext::default())
        }
    }

    pub fn build(mut self) -> String {
        let mut result = String::new();
        for content in self.contents.drain(..) {
            writeln!(
                &mut result,
                "{}",
                content.emit(&mut *self.ctx.borrow_mut())
            )
            .unwrap();
        }
        result
    }
}

impl EmittableContainer for Makefile {
    fn add(&mut self, e: EmittableRef) {
        self.contents.push(e);
    }

    fn ctx(&mut self) -> RRC<SymbolContext> {
        self.ctx.clone()
    }
}
