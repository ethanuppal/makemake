use crate::{
    assignment::{Assignment, AssignmentKind},
    directive::Include,
    emittable::{MakefileEmittable, MakefileEmittableRef},
    expr::Expr,
    misc::{Comment, Newline},
    rule::RuleRef,
    var::Variable
};

#[derive(Default)]
pub struct Makefile {
    contents: Vec<MakefileEmittableRef>
}

impl Makefile {
    pub fn new() -> Self {
        Self { contents: vec![] }
    }

    pub fn comment<S: AsRef<str>>(&mut self, text: S) {
        self.add(Box::new(Comment::new(text)));
    }

    pub fn newline(&mut self) {
        self.add(Box::new(Newline));
    }

    pub fn assign<S: AsRef<str>, E: Into<Expr>>(
        &mut self, name: S, value: E
    ) -> Variable {
        self.build_assign(AssignmentKind::Overwrite, name, value)
    }

    pub fn assign_without_overwrite<S: AsRef<str>, E: Into<Expr>>(
        &mut self, name: S, value: E
    ) -> Variable {
        self.build_assign(AssignmentKind::Underwrite, name, value)
    }

    pub fn append<S: AsRef<str>, E: Into<Expr>>(
        &mut self, name: S, value: E
    ) -> Variable {
        self.build_assign(AssignmentKind::Append, name, value)
    }

    pub fn rule<E: Into<Expr>>(&mut self, target: E) -> RuleRef {
        let rule = RuleRef::new(target);
        self.add(Box::new(rule.clone()));
        rule
    }

    pub fn include<S: AsRef<str>>(&mut self, path_expr: S) {
        self.add(Box::new(Include::new(path_expr)));
    }

    pub fn build(&self) -> String {
        self.contents
            .iter()
            .map(|e| e.emit())
            .collect::<Vec<_>>()
            .join("\n")
    }

    fn build_assign<S: AsRef<str>, E: Into<Expr>>(
        &mut self, kind: AssignmentKind, name: S, value: E
    ) -> Variable {
        let var = Variable::new(name);
        self.add(Box::new(Assignment::new(kind, var.clone(), value)));
        var
    }

    fn add(&mut self, e: Box<dyn MakefileEmittable>) {
        self.contents.push(e);
    }
}
