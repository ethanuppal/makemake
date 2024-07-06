use crate::{
    emittable::Emittable,
    symbol_context::{Resolvable, SymbolContext, SymbolID}
};

#[derive(Clone, Copy)]
pub(crate) enum _Variable {
    Builtin(SymbolID),
    User(SymbolID)
}

#[derive(Clone, Copy)]
pub struct Variable {
    pub(crate) value: _Variable
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
        match self.value {
            _Variable::Builtin(id) | _Variable::User(id) => id
        }
    }
}

impl Emittable for Variable {
    fn emit(&self, ctx: &mut SymbolContext) -> String {
        let name = self.name(ctx);
        match self.value {
            _Variable::Builtin(_) => format!("${}", name),
            _Variable::User(_) => format!("$({})", name)
        }
    }
}
