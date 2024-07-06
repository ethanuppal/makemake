# makemake

![CI](https://github.com/ethanuppal/makemake/actions/workflows/ci.yaml/badge.svg)
[![CodeFactor](https://www.codefactor.io/repository/github/ethanuppal/makemake/badge)](https://www.codefactor.io/repository/github/ethanuppal/makemake)

`makemake` is a rust library for building Makefiles programmatically.
Traits allow `makemake` functions to take in pretty much any type as an
argument - you can set the value of a variable to a string, a function, or
even another variable, without needing `.into()`s everywhere. There's also a
helper `expr!` macro for building more complex expressions.

## Usage

Run this command in your rust project root:
```shell
cargo add makemake
```
You can find the crate [on crates.io](https://crates.io/crates/makemake).

Then, you can add
```rust
use makemake::prelude::*;
```
to appropriate files.

## Example

Let's build a Makefile for a C project.
```rs
let mut makefile = Makefile::new();
```

We'll want variables for the source files, object files, compiler, and
flags.
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
makefile.rule(target).dep("main.c").dep(obj).cmd(expr!(
    cc,
    cflags,
    "-o",
    makefile.target_var(),
    makefile.deps_var()
));
```

Finally, we can print the resultant Makefile.
```rs
print!("{}", makefile.build());
```

Indeed, we can use our Makefile to build an example project.
```shell
cargo run --example c_project > examples/Makefile
cd examples
make && ./main
```
The actual example also comes with a `make clean`!

### Setting Up Git Hooks

After cloning the repository, run the following script to set up the hooks:
```shell
/bin/sh setup_hooks.sh
```

## License

This project is licensed under the [MIT License](LICENSE), a copy of which
is available in this directory.