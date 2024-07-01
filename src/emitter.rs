use crate::{
    assignment::{Assignment, AssignmentKind},
    conditional::ConditionalRef,
    directive::Include,
    emittable::EmittableRef,
    expr::Expr,
    misc::{Comment, Newline},
    rrc::RRC,
    rule::RuleRef,
    symbol_context::{Resolvable, SymbolContext},
    var::Variable
};

pub(crate) trait EmittableContainer {
    fn add(&mut self, e: EmittableRef);
    fn ctx(&mut self) -> RRC<SymbolContext>;

    fn build_assign<V: Resolvable, E: Into<Expr>>(
        &mut self, kind: AssignmentKind, var: V, value: E
    ) -> Variable {
        let var = var.resolve(&mut self.ctx().borrow_mut());
        self.add(Box::new(Assignment::new(kind, var, value)));
        var
    }
}

pub trait Emitter {
    fn var<S: Resolvable>(&mut self, name: S) -> Variable;

    fn target_var(&mut self) -> Variable;
    fn first_dep_var(&mut self) -> Variable;
    fn deps_var(&mut self) -> Variable;

    fn comment<S: AsRef<str>>(&mut self, text: S);
    fn newline(&mut self);
    fn assign<V: Resolvable, E: Into<Expr>>(
        &mut self, var: V, value: E
    ) -> Variable;
    fn assign_without_overwrite<V: Resolvable, E: Into<Expr>>(
        &mut self, var: V, value: E
    ) -> Variable;
    fn append<V: Resolvable, E: Into<Expr>>(
        &mut self, var: V, value: E
    ) -> Variable;
    fn include<S: AsRef<str>>(&mut self, path_expr: S);
    fn rule<E: Into<Expr>>(&mut self, target: E) -> RuleRef;
    fn branch_tree(&mut self) -> ConditionalRef;
}

impl<T: EmittableContainer> Emitter for T {
    fn var<S: Resolvable>(&mut self, name: S) -> Variable {
        name.resolve(&mut self.ctx().borrow_mut())
    }

    fn target_var(&mut self) -> Variable {
        Variable::target(&mut self.ctx().borrow_mut())
    }

    fn first_dep_var(&mut self) -> Variable {
        Variable::first_dep(&mut self.ctx().borrow_mut())
    }

    fn deps_var(&mut self) -> Variable {
        Variable::deps(&mut self.ctx().borrow_mut())
    }

    fn comment<S: AsRef<str>>(&mut self, text: S) {
        self.add(Box::new(Comment::new(text)));
    }

    fn newline(&mut self) {
        self.add(Box::new(Newline));
    }

    fn assign<V: Resolvable, E: Into<Expr>>(
        &mut self, var: V, value: E
    ) -> Variable {
        self.build_assign(AssignmentKind::Overwrite, var, value)
    }

    fn assign_without_overwrite<V: Resolvable, E: Into<Expr>>(
        &mut self, var: V, value: E
    ) -> Variable {
        self.build_assign(AssignmentKind::Underwrite, var, value)
    }

    fn append<V: Resolvable, E: Into<Expr>>(
        &mut self, var: V, value: E
    ) -> Variable {
        self.build_assign(AssignmentKind::Append, var, value)
    }

    fn include<S: AsRef<str>>(&mut self, path_expr: S) {
        self.add(Box::new(Include::new(path_expr)));
    }

    fn rule<E: Into<Expr>>(&mut self, target: E) -> RuleRef {
        let rule = RuleRef::new(target);
        self.add(Box::new(rule.clone()));
        rule
    }

    fn branch_tree(&mut self) -> ConditionalRef {
        let conditional = ConditionalRef::new(self.ctx());
        self.add(Box::new(conditional.clone()));
        conditional
    }
}
