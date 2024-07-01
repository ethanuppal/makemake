use crate::{
    emittable::Emittable,
    symbol_context::{Resolvable, SymbolContext, SymbolID}
};

#[derive(Clone, Copy)]
pub enum Variable {
    Builtin(SymbolID),
    User(SymbolID)
}

impl Variable {
    pub(crate) fn target(ctx: &mut SymbolContext) -> Self {
        ctx.get_select::<_, true>("@")
    }

    pub(crate) fn first_dep(ctx: &mut SymbolContext) -> Self {
        ctx.get_select::<_, true>("<")
    }

    pub(crate) fn deps(ctx: &mut SymbolContext) -> Self {
        ctx.get_select::<_, true>("^")
    }

    pub(crate) fn id(&self) -> SymbolID {
        match &self {
            Self::Builtin(id) | Self::User(id) => *id
        }
    }
}

impl Emittable for Variable {
    fn emit(&self, ctx: &mut SymbolContext) -> String {
        let name = self.name(ctx);
        match &self {
            Self::Builtin(_) => format!("${}", name),
            Self::User(_) => format!("$({})", name)
        }
    }
}
