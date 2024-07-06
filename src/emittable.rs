use crate::symbol_context::SymbolContext;

/// A value that has a textual representation in a Makefile.
pub trait Emittable {
    /// Constructs a textual representation of this value in a Makefile.
    fn emit(&self, ctx: &mut SymbolContext) -> String;
}

/// Any [`Emittable`] value.
pub(crate) type EmittableRef = Box<dyn Emittable>;
