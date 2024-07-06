use crate::{
    emittable::Emittable,
    expr::{EmittableVec, Expr},
    symbol_context::SymbolContext
};

/// A Makefile directive such as `include` (although, use [`Include`] for that).
pub struct Directive {
    name: String,
    args: Vec<Expr>
}

impl Directive {
    /// Constructs a new directive `name` with arguments `args`.
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

/// A helper for constructing an include [`Directive`].
pub struct Include {
    directive: Directive
}

impl Include {
    /// Constructs a new include directive for path `path_expr`.
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
