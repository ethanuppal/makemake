use std::ops::{Add, AddAssign};

use crate::{
    emittable::Emittable,
    function::{Function, Substitution},
    symbol_context::SymbolContext,
    var::Variable
};

pub enum Expr {
    Empty,
    Raw(String),
    Var(Variable),
    Concat(Vec<Expr>),
    SubstRef(Box<Substitution>),
    Function(Function)
}

impl Expr {
    pub fn concat<E: Into<Expr>>(self, expr: E) -> Expr {
        Self::Concat(match self {
            Self::Concat(mut list) => {
                list.push(expr.into());
                list
            }
            other => {
                vec![other, expr.into()]
            }
        })
    }

    pub fn then<E: Into<Expr>>(self, expr: E) -> Expr {
        self.concat(" ").concat(expr)
    }
}

impl Default for Expr {
    fn default() -> Self {
        Self::Empty
    }
}

impl<T: AsRef<str>> From<T> for Expr {
    fn from(value: T) -> Self {
        Self::Raw(value.as_ref().to_string())
    }
}

impl From<Variable> for Expr {
    fn from(value: Variable) -> Self {
        Self::Var(value)
    }
}

impl From<Function> for Expr {
    fn from(value: Function) -> Self {
        Self::Function(value)
    }
}

impl From<Substitution> for Expr {
    fn from(value: Substitution) -> Self {
        Self::SubstRef(Box::new(value))
    }
}

impl Emittable for Expr {
    fn emit(&self, ctx: &mut SymbolContext) -> String {
        match &self {
            Expr::Empty => String::new(),
            Expr::Raw(string) => string.clone(),
            Expr::Var(var) => var.emit(ctx),
            Expr::Concat(list) => list.join_emit("", ctx),
            Expr::SubstRef(subst) => subst.emit(ctx),
            Expr::Function(func) => func.emit(ctx)
        }
    }
}

pub trait EmittableVec {
    fn join_emit<S: AsRef<str>>(
        &self, sep: S, ctx: &mut SymbolContext
    ) -> String;
}

impl EmittableVec for Vec<Expr> {
    fn join_emit<S: AsRef<str>>(
        &self, sep: S, ctx: &mut SymbolContext
    ) -> String {
        self.iter()
            .map(|e| e.emit(ctx))
            .collect::<Vec<_>>()
            .join(sep.as_ref())
    }
}

impl Add for Expr {
    type Output = Expr;

    fn add(self, rhs: Self) -> Self::Output {
        self.concat(rhs)
    }
}

impl AddAssign for Expr {
    fn add_assign(&mut self, rhs: Self) {
        *self = std::mem::take(self).concat(rhs);
    }
}

#[macro_export]
macro_rules! expr {
    ($first:expr) => {
        $crate::Expr::from($first)
    };
    ($first:expr, $($rest:expr),+ $(,)?) => {{
        let mut expr = $crate::expr::Expr::from($first);
        $(
            expr = expr.then($crate::expr::Expr::from($rest));
        )*
        expr
    }};
}
