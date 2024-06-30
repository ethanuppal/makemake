# makemake

`makemake` is a rust library for building Makefiles programmatically.

## Example

Let's make a Makefile for a C project.
```rs
let mut makefile = Makefile::new();
```

We'll want variables for the source files, object files, compiler, and flags.
```rs
let src = makefile.assign("SRC", Function::wildcard(["src/*.c".into()]));
let obj = makefile.assign("OBJ", Substitution::new(src, ".c", ".o"));
let cc = makefile.assign_without_overwrite(
    "CC",
    Function::shell("which gcc || which clang")
);
let cflags = makefile.append("CFLAGS", "-std=c99 -Wall -Wextra");
let target = makefile.assign("TARGET", "main");
```

Next, we'll define the rule to create the target.
```rs
makefile.rule(target).dep("main.c").dep(obj).cmd(
    Expr::from(cc)
        .then(cflags)
        .then("-o")
        .then(Variable::target())
        .then(Variable::deps())
);
```

Finally, we can build and print the Makefile.
```rs
println!("{}", makefile.build());
```

We can use our Makefile to build an example project.
```shell
cargo run c_project > examples/Makefile
cd examples
make
./main
```
