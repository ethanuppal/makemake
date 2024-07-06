use std::fmt::Write;

use crate::{
    emittable::{Emittable, EmittableRef},
    emitter::EmittableContainer,
    expr::Expr,
    rrc::{rrc, RRC},
    symbol_context::{Resolvable, SymbolContext},
    var::Variable
};

pub(crate) enum Condition {
    Eq(Expr, Expr),
    Def(Variable),
    Undef(Variable)
}

impl Emittable for Condition {
    fn emit(&self, ctx: &mut SymbolContext) -> String {
        match self {
            Condition::Eq(lhs, rhs) => {
                format!("ifeq ({}, {})", lhs.emit(ctx), rhs.emit(ctx))
            }
            Condition::Def(var) => {
                format!("ifdef $({})", var.name(ctx))
            }
            Condition::Undef(var) => {
                format!("ifndef $({})", var.name(ctx))
            }
        }
    }
}

pub struct Branch {
    condition: Option<Condition>,
    contents: Vec<EmittableRef>,
    ctx: RRC<SymbolContext>
}

impl Branch {
    fn new(condition: Option<Condition>, ctx: RRC<SymbolContext>) -> Self {
        Self {
            condition,
            contents: vec![],
            ctx
        }
    }
}

impl EmittableContainer for Branch {
    fn add(&mut self, e: EmittableRef) {
        self.contents.push(e);
    }

    fn ctx(&mut self) -> RRC<SymbolContext> {
        self.ctx.clone()
    }
}

pub(crate) struct Conditional {
    branches: Vec<Branch>
}

impl Conditional {
    fn new() -> Self {
        Self { branches: vec![] }
    }

    fn add(&mut self, branch: Branch) {
        self.branches.push(branch);
    }
}

impl Emittable for Conditional {
    fn emit(&self, ctx: &mut SymbolContext) -> String {
        let mut result = String::new();
        for (i, branch) in self.branches.iter().enumerate() {
            if i > 0 {
                write!(&mut result, "else").unwrap();
            }
            if let Some(condition) = &branch.condition {
                if i > 0 {
                    result.push(' ');
                }
                writeln!(&mut result, "{}", condition.emit(ctx)).unwrap();
            } else {
                result.push('\n');
            }
            for content in &branch.contents {
                writeln!(&mut result, "{}", content.emit(ctx)).unwrap();
            }
        }
        write!(&mut result, "endif").unwrap();
        result
    }
}

#[derive(Clone)]
pub struct ConditionalRef {
    conditional: RRC<Conditional>,
    ctx: RRC<SymbolContext>
}

impl ConditionalRef {
    pub(crate) fn new(ctx: RRC<SymbolContext>) -> ConditionalRef {
        Self {
            conditional: rrc(Conditional::new()),
            ctx
        }
    }

    pub fn when_eq<E1: Into<Expr>, E2: Into<Expr>, F: FnOnce(&mut Branch)>(
        self, lhs: E1, rhs: E2, f: F
    ) -> ConditionalRef {
        self.build_conditional(Some(Condition::Eq(lhs.into(), rhs.into())), f)
    }

    pub fn when_def<V: Resolvable, F: FnOnce(&mut Branch)>(
        self, var: V, f: F
    ) -> ConditionalRef {
        let var = var.resolve(&mut self.ctx.borrow_mut());
        self.build_conditional(Some(Condition::Def(var)), f)
    }

    pub fn when_undef<V: Resolvable, F: FnOnce(&mut Branch)>(
        self, var: V, f: F
    ) -> ConditionalRef {
        let var = var.resolve(&mut self.ctx.borrow_mut());
        self.build_conditional(Some(Condition::Undef(var)), f)
    }

    pub fn otherwise<F: FnOnce(&mut Branch)>(self, f: F) -> ConditionalRef {
        self.build_conditional(None, f)
    }

    fn build_conditional<F: FnOnce(&mut Branch)>(
        self, condition: Option<Condition>, f: F
    ) -> ConditionalRef {
        let mut branch = Branch::new(condition, self.ctx.clone());
        f(&mut branch);
        self.conditional.borrow_mut().add(branch);
        self
    }
}

impl Emittable for ConditionalRef {
    fn emit(&self, ctx: &mut SymbolContext) -> String {
        self.conditional.borrow().emit(ctx)
    }
}
