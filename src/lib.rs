// i tried and couldn't find a crate for this -- if you know of one, lmk

pub mod assignment;
pub mod conditional;
pub mod directive;
pub mod emittable;
pub mod emitter;
pub mod expr;
pub mod function;
pub mod makefile;
pub mod misc;
pub mod rrc;
pub mod rule;
pub mod symbol_context;
pub mod var;

#[cfg(test)]
mod tests {
    use crate::{
        emitter::Emitter,
        expr,
        expr::Expr,
        function::{Function, Substitution},
        makefile::Makefile
    };
    use insta::assert_snapshot;

    #[test]
    fn test_example() {
        let mut makefile = Makefile::new();
        makefile.comment("This is a\ntesting makefile");
        makefile.newline();
        let a = makefile.assign("A", "foobar");
        makefile.assign("C", Function::subst("foo", "bar", a.clone()));
        let src =
            makefile.assign("SRC", Function::wildcard([Expr::from("*.c")]));
        makefile.assign("OBJ", Substitution::new(src, ".c", ".o"));
        makefile
            .rule("my_rule")
            .phony()
            .dep("dep1")
            .dep("dep2")
            .order_only_dep("oodep1")
            .order_only_dep("oodep2")
            .cmd(expr!(
                "foo -f",
                a,
                "-o",
                makefile.target_var(),
                "-i",
                makefile.first_dep_var()
            ))
            .cmd(expr!("cc", Function::value("C"), "and rest of cmd"));
        makefile.assign_without_overwrite("SIM", "icarus");
        makefile
            .branch_tree()
            .when_def("a", |e| e.comment("a"))
            .when_def("b", |e| e.comment("b"))
            .otherwise(|e| e.comment("c"));

        assert_snapshot!(makefile.build());
    }
}
