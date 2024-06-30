use crate::{
    emittable::MakefileEmittable,
    function::{Function, Substitution},
    var::Variable
};
use std::ops::{Add, AddAssign};

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

impl MakefileEmittable for Expr {
    fn emit(&self) -> String {
        match &self {
            Expr::Empty => String::new(),
            Expr::Raw(string) => string.clone(),
            Expr::Var(var) => var.emit(),
            Expr::Concat(list) => list.join_emit(""),
            Expr::SubstRef(subst) => subst.emit(),
            Expr::Function(func) => func.emit()
        }
    }
}

pub trait MakefileEmittableVec {
    fn join_emit<S: AsRef<str>>(&self, sep: S) -> String;
}

impl MakefileEmittableVec for Vec<Expr> {
    fn join_emit<S: AsRef<str>>(&self, sep: S) -> String {
        self.iter()
            .map(|e| e.emit())
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
