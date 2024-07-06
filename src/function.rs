use crate::{
    emittable::Emittable,
    expr::{EmittableVec, Expr},
    symbol_context::{Resolvable, SymbolContext, VariablePromise}
};
use paste::paste;

/// A Makefile function.
pub struct Function {
    name: String,
    args: Vec<Expr>
}

fn fix_name(name: &str) -> String {
    let mut name = name.to_string();
    if name.ends_with('_') {
        name.pop();
    }
    name = name.replace('_', "-");
    name
}

macro_rules! func {
    ($name:ident $var_arg:ident..., $($arg:ident),+) => {
        paste! {
            pub fn $name<$([<T $arg>]: Into<Expr>),*, TVarArg: Into<Vec<Expr>>>(
                $var_arg: TVarArg,
                $([<$arg>]: [<T $arg>]),*
            ) -> Function {
                let mut args = vec![$([<$arg>].into()),*];
                args.extend($var_arg.into());
                Function {
                    name: fix_name(stringify!($name)),
                    args
                }
            }
        }
    };
    ($name:ident $($arg:ident)*, $var_arg:ident...) => {
        paste! {
            pub fn $name<$([<T $arg>]: Into<Expr>),*, TVarArg: Into<Vec<Expr>>>(
                $([<$arg>]: [<T $arg>]),*,
                $var_arg: TVarArg
            ) -> Function {
                let mut args = vec![$([<$arg>].into()),*];
                args.extend($var_arg.into());
                Function {
                    name: fix_name(stringify!($name)),
                    args
                }
            }
        }
    };
    ($name:ident $($arg:ident),*) => {
        paste! {
            pub fn $name<$([<T $arg>]: Into<Expr>),*>($($arg: [<T $arg>]),*) -> Function {
                Function {
                    name: fix_name(stringify!($name)),
                    args: vec![$($arg.into()),*]
                }
            }
        }
    };
    ($name:ident $var_arg:ident...) => {
        paste! {
            pub fn $name<TVarArg: Into<Vec<Expr>>>($var_arg: TVarArg) -> Function {
                Function {
                    name: fix_name(stringify!($name)),
                    args: $var_arg.into()
                }
            }
        }
    };
}

#[allow(dead_code)]
#[allow(non_camel_case_types)]
impl Function {
    func!(subst from,to,text);
    func!(patsubst pattern,replacement,text);
    func!(strip string);
    func!(findstring find,in_);
    func!(filter pattern...,text);
    func!(filter_out pattern...,text);
    func!(sort list);
    func!(word n,text);
    func!(words text);
    func!(wordlist s,e,text);
    func!(firstword names...);
    func!(lastword names...);
    func!(dir names...);
    func!(notdir names...);
    func!(suffix names...);
    func!(basename names...);
    func!(addsufix suffix,names...);
    func!(addprefix prefix,names...);
    func!(join list1,list2);
    func!(wildcard pattern...);
    func!(realpath names...);
    func!(abspath names...);
    func!(error text...);
    func!(warning text...);
    func!(shell command);
    func!(origin variable);
    func!(flavor variable);
    func!(let_ var...,words,text);
    func!(foreach var,words,text);
    func!(if_ condition,then_part,else_part);
    func!(or condition...);
    func!(and condition...);
    func!(intcmp lhs,rhs,lt_part,eq_part,gt_part);
    func!(call var,param...);
    func!(eval text);
    func!(value var);
}

impl Emittable for Function {
    fn emit(&self, ctx: &mut SymbolContext) -> String {
        format!("$({} {})", self.name, self.args.join_emit(",", ctx))
    }
}

/// A suffix substitution.
pub struct Substitution {
    var: VariablePromise,
    old_suffix: Expr,
    new_suffix: Expr
}

impl Substitution {
    /// Constructs a new suffix substitution that substitutes `old_suffix` for
    /// `new_suffix` in `var`.
    pub fn new<V: Into<VariablePromise>, E1: Into<Expr>, E2: Into<Expr>>(
        var: V, old_suffix: E1, new_suffix: E2
    ) -> Self {
        Self {
            var: var.into(),
            old_suffix: old_suffix.into(),
            new_suffix: new_suffix.into()
        }
    }
}

impl Emittable for Substitution {
    fn emit(&self, ctx: &mut SymbolContext) -> String {
        let old_suffix = self.old_suffix.emit(ctx);
        let new_suffix = self.new_suffix.emit(ctx);
        let name = self.var.name(ctx);
        format!("$({}:{}={})", name, old_suffix, new_suffix)
    }
}
