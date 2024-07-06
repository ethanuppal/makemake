use std::fmt::Write;

use crate::{
    emittable::EmittableRef, emitter::EmittableContainer, rrc::RRC,
    symbol_context::SymbolContext
};

/// A Makefile.
#[derive(Default)]
pub struct Makefile {
    contents: Vec<EmittableRef>,
    ctx: RRC<SymbolContext>
}

impl Makefile {
    /// Constructs an empty Makefile.
    pub fn new() -> Self {
        Self::default()
    }

    /// Converts this Makefile into its textual representation.
    pub fn build(mut self) -> String {
        let mut result = String::new();
        for content in self.contents.drain(..) {
            writeln!(
                &mut result,
                "{}",
                content.emit(&mut self.ctx.borrow_mut())
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
