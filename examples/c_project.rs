use makemake::{
    expr::Expr,
    function::{Function, Substitution},
    makefile::Makefile,
    var::Variable
};

fn main() {
    let mut makefile = Makefile::new();

    let src = makefile.assign("SRC", Function::wildcard(["src/*.c".into()]));
    let obj = makefile.assign("OBJ", Substitution::new(src, ".c", ".o"));
    let cc = makefile.assign_without_overwrite(
        "CC",
        Function::shell("which gcc || which clang")
    );
    let cflags = makefile.append("CFLAGS", "-std=c99 -Wall -Wextra");
    let target = makefile.assign("TARGET", "main");

    makefile.newline();

    makefile.rule(target).dep("main.c").dep(obj).cmd(
        Expr::from(cc)
            .then(cflags)
            .then("-o")
            .then(Variable::target())
            .then(Variable::deps())
    );

    println!("{}", makefile.build());
}
