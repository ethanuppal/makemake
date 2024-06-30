// i tried and couldn't find a crate for this -- if you know of one, lmk

pub mod assignment;
pub mod directive;
pub mod emittable;
pub mod expr;
pub mod function;
pub mod makefile;
pub mod misc;
pub mod rule;
pub mod var;

#[cfg(test)]
mod tests {
    use insta::assert_snapshot;

    use crate::{
        expr::Expr,
        function::{Function, Substitution},
        makefile::Makefile,
        var::Variable
    };

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
            .cmd(
                Expr::from("foo -f ")
                    + a.into()
                    + " -o ".into()
                    + Variable::target().into()
                    + " -i ".into()
                    + Variable::first_dep().into()
            )
            .cmd(
                Expr::from("cc ")
                    .concat(Function::value("C"))
                    .concat(" and rest of cmd")
            );
        makefile.assign_without_overwrite("SIM", "icarus");

        assert_snapshot!(makefile.build());
    }
}
