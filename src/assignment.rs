use crate::{
    emittable::Emittable,
    expr::Expr,
    symbol_context::{Resolvable, SymbolContext},
    var::Variable
};

pub enum AssignmentKind {
    Overwrite,
    Underwrite,
    Append
}

impl Emittable for AssignmentKind {
    fn emit(&self, _ctx: &mut SymbolContext) -> String {
        match &self {
            Self::Overwrite => "=",
            Self::Underwrite => "?=",
            Self::Append => "+="
        }
        .to_string()
    }
}

pub struct Assignment {
    kind: AssignmentKind,
    var: Variable,
    value: Expr
}

impl Assignment {
    pub(crate) fn new<E: Into<Expr>>(
        kind: AssignmentKind, var: Variable, value: E
    ) -> Self {
        Self {
            kind,
            var,
            value: value.into()
        }
    }
}

impl Emittable for Assignment {
    fn emit(&self, ctx: &mut SymbolContext) -> String {
        let kind = self.kind.emit(ctx);
        let value = self.value.emit(ctx);
        let name = self.var.name(ctx);
        format!("{} {} {}", name, kind, value)
    }
}
