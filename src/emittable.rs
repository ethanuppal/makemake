use crate::symbol_context::SymbolContext;

pub trait Emittable {
    fn emit(&self, ctx: &mut SymbolContext) -> String;
}

pub(crate) type EmittableRef = Box<dyn Emittable>;
