use crate::{emittable::MakefileEmittable, expr::Expr, var::Variable};

pub enum AssignmentKind {
    Overwrite,
    Underwrite,
    Append
}

impl MakefileEmittable for AssignmentKind {
    fn emit(&self) -> String {
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

impl MakefileEmittable for Assignment {
    fn emit(&self) -> String {
        format!(
            "{} {} {}",
            self.var.name(),
            self.kind.emit(),
            self.value.emit()
        )
    }
}
