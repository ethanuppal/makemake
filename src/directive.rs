use crate::{
    emittable::Emittable,
    expr::{EmittableVec, Expr},
    symbol_context::SymbolContext
};

pub struct Directive {
    name: String,
    args: Vec<Expr>
}

impl Directive {
    pub fn new<S: AsRef<str>, E: Into<Vec<Expr>>>(name: S, args: E) -> Self {
        Self {
            name: name.as_ref().to_string(),
            args: args.into()
        }
    }
}

impl Emittable for Directive {
    fn emit(&self, ctx: &mut SymbolContext) -> String {
        format!("{} {}", self.name, self.args.join_emit(" ", ctx))
    }
}

pub struct Include {
    directive: Directive
}

impl Include {
    pub fn new<S: AsRef<str>>(path_expr: S) -> Self {
        Self {
            directive: Directive::new(
                "include",
                vec![Expr::from(path_expr.as_ref().to_string())]
            )
        }
    }
}

impl Emittable for Include {
    fn emit(&self, ctx: &mut SymbolContext) -> String {
        self.directive.emit(ctx)
    }
}
