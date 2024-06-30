# makemake

![CI](https://github.com/ethanuppal/makemake/actions/workflows/ci.yaml/badge.svg)
[![CodeFactor](https://www.codefactor.io/repository/github/ethanuppal/makemake/badge)](https://www.codefactor.io/repository/github/ethanuppal/makemake)

`makemake` is a rust library for building Makefiles programmatically.

## Usage

Run this command in your rust project root:
```
cargo add makemake
```
You can find the crate [on crates.io](https://crates.io/crates/makemake).

## Example

Let's build a Makefile for a C project.
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

Finally, we can print the resultant Makefile.
```rs
println!("{}", makefile.build());
```

Indeed, we can use our Makefile to build an example project.
```shell
cargo run --example c_project > examples/Makefile
cd examples
make && ./main
```
The actual example also comes with a `make clean`!
