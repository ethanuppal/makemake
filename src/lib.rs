//! # makemake
//!
//! ![CI](https://github.com/ethanuppal/makemake/actions/workflows/ci.yaml/badge.svg)
//! [![CodeFactor](https://www.codefactor.io/repository/github/ethanuppal/makemake/badge)](https://www.codefactor.io/repository/github/ethanuppal/makemake)
//!
//! `makemake` is a rust library for building Makefiles programmatically.
//! Traits allow `makemake` functions to take in pretty much any type as an
//! argument - you can set the value of a variable to a string, a function, or
//! even another variable, without needing `.into()`s everywhere. There's also a
//! helper `expr!` macro for building more complex expressions.
//!
//! Although version-agnostic, `makemake` does support GNU `make` features. If
//! you intend your Makefiles to be extremely portable (although even macOS
//! comes with GNU `make`), avoid those features as you would when writing
//! Makefiles by hand.
//!
//! ## Usage
//!
//! Run this command in your rust project root:
//! ```shell
//! cargo add makemake
//! ```
//! You can find the crate [on crates.io](https://crates.io/crates/makemake).
//!
//! Then, you can add
//! ```rust
//! use makemake::prelude::*;
//! ```
//! to appropriate files.
//!
//! ## Example
//!
//! Let's build a Makefile for a C project.
//! ```rs
//! let mut makefile = Makefile::new();
//! ```
//!
//! We'll want variables for the source files, object files, compiler, and
//! flags.
//! ```rs
//! let src = makefile.assign("SRC", Function::wildcard([expr!("src/*.c")]));
//! let obj = makefile.assign("OBJ", Substitution::new(src, ".c", ".o"));
//! let cc = makefile.assign_without_overwrite(
//!     "CC",
//!     Function::shell("which gcc || which clang")
//! );
//! let cflags = makefile.append("CFLAGS", "-std=c99 -Wall -Wextra");
//! let target = makefile.assign("TARGET", "main");
//! ```
//!
//! Next, we'll define the rule to create the target.
//! ```rs
//! makefile.rule(target).dep("main.c").dep(obj).cmd(expr!(
//!     cc;
//!     cflags;
//!     "-o";
//!     makefile.target_var();
//!     makefile.deps_var()
//! ));
//! ```
//! > In the `expr!` macro, use `;` to separate arguments by spaces and `,` to
//! > put them directly adjacent.
//!
//! Finally, we can print the resultant Makefile.
//! ```rs
//! print!("{}", makefile.build());
//! ```
//!
//! Indeed, we can use our Makefile to build an example project.
//! ```shell
//! cargo run --example c_project > examples/Makefile
//! cd examples
//! make && ./main
//! ```
//! The actual example (`c_project.rs` in the `examples/` directory) also comes
//! with a `make clean`!
//!
//! ### Setting Up Git Hooks
//!
//! After cloning the repository, run the following script to set up the hooks:
//! ```shell
//! /bin/sh setup_hooks.sh
//! ```
//!
//! ## License
//!
//! This project is licensed under the [LGPL License](LICENSE), a copy of which
//! is available in this directory.

pub mod assignment;
pub mod conditional;
pub mod directive;
pub mod emittable;
pub mod emitter;
pub mod expr;
pub mod function;
pub mod makefile;
pub mod misc;
pub mod prelude;
pub mod rrc;
pub mod rule;
pub mod symbol_context;
pub mod var;

#[cfg(test)]
mod tests {
    use crate::{
        emitter::Emitter,
        expr,
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
        makefile.assign("C", Function::subst("foo", "bar", a));
        let src = makefile.assign("SRC", Function::wildcard([expr!("*.c")]));
        makefile.assign("OBJ", Substitution::new(src, ".c", ".o"));
        makefile
            .rule("my_rule")
            .phony()
            .dep("dep1")
            .dep("dep2")
            .order_only_dep("oodep1")
            .order_only_dep("oodep2")
            .cmd(expr!(
                "foo -f";
                a;
                "-o";
                makefile.target_var();
                "-i";
                makefile.first_dep_var()
            ))
            .cmd(expr!("cc"; Function::value("C"), " and rest of cmd"));
        makefile.assign_without_overwrite("SIM", "icarus");
        makefile
            .branch_tree()
            .when_def("a", |e| e.comment("a"))
            .when_def("b", |e| e.comment("b"))
            .otherwise(|e| e.comment("c"));

        assert_snapshot!(makefile.build());
    }
}
