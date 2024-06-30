use crate::{
    emittable::MakefileEmittable,
    expr::{Expr, MakefileEmittableVec}
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

impl MakefileEmittable for Directive {
    fn emit(&self) -> String {
        format!("{} {}", self.name, self.args.join_emit(" "))
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

impl MakefileEmittable for Include {
    fn emit(&self) -> String {
        self.directive.emit()
    }
}
