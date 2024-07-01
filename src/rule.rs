use crate::{
    emittable::Emittable,
    expr::{EmittableVec, Expr},
    rrc::{rrc, RRC},
    symbol_context::SymbolContext
};
use std::fmt::Write;

struct Rule {
    is_phony: bool,
    target: Expr,
    dependencies: Vec<Expr>,
    order_only_dependencies: Vec<Expr>,
    commands: Vec<Expr>
}

impl Rule {
    fn new<E: Into<Expr>>(target: E) -> Self {
        Self {
            is_phony: false,
            target: target.into(),
            dependencies: vec![],
            order_only_dependencies: vec![],
            commands: vec![]
        }
    }
}

impl Emittable for Rule {
    fn emit(&self, ctx: &mut SymbolContext) -> String {
        let mut result = String::new();
        if self.is_phony {
            writeln!(&mut result, ".PHONY: {}", self.target.emit(ctx)).unwrap();
        }
        write!(
            &mut result,
            "{}: {}",
            self.target.emit(ctx),
            self.dependencies.join_emit(" ", ctx)
        )
        .unwrap();
        if !self.order_only_dependencies.is_empty() {
            write!(
                &mut result,
                " | {}",
                self.order_only_dependencies.join_emit(" ", ctx)
            )
            .unwrap();
        }
        for command in &self.commands {
            result.push('\n');
            write!(
                &mut result,
                "\t{}",
                command.emit(ctx).replace('\n', "\\\n")
            )
            .unwrap();
        }
        result
    }
}

#[derive(Clone)]
pub struct RuleRef {
    rule: RRC<Rule>
}

impl RuleRef {
    pub(crate) fn new<E: Into<Expr>>(target: E) -> Self {
        RuleRef {
            rule: rrc(Rule::new(target.into()))
        }
    }

    pub fn set_phony(&self) {
        self.rule.borrow_mut().is_phony = true;
    }

    pub fn add_dep<E: Into<Expr>>(&self, dep: E) {
        self.rule.borrow_mut().dependencies.push(dep.into());
    }

    pub fn add_order_only_dep<E: Into<Expr>>(&self, dep: E) {
        self.rule
            .borrow_mut()
            .order_only_dependencies
            .push(dep.into());
    }

    pub fn add_cmd<E: Into<Expr>>(&self, cmd: E) {
        self.rule.borrow_mut().commands.push(cmd.into());
    }

    pub fn phony(self) -> Self {
        self.set_phony();
        self
    }

    pub fn dep<E: Into<Expr>>(self, dep: E) -> Self {
        self.add_dep(dep);
        self
    }

    pub fn order_only_dep<E: Into<Expr>>(self, dep: E) -> Self {
        self.add_order_only_dep(dep);
        self
    }

    pub fn cmd<E: Into<Expr>>(self, cmd: E) -> Self {
        self.add_cmd(cmd);
        self
    }
}

impl Emittable for RuleRef {
    fn emit(&self, ctx: &mut SymbolContext) -> String {
        self.rule.borrow().emit(ctx)
    }
}
