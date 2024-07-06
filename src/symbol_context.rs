use std::collections::HashMap;

use crate::var::{Variable, _Variable};

pub type ImmutableString = Box<str>;
pub type SymbolID = i32;

struct Symbol {
    key: ImmutableString,
    id: SymbolID
}

#[derive(Default)]
pub struct SymbolContext {
    store: HashMap<ImmutableString, Variable>,
    strings: Vec<ImmutableString>
}

impl SymbolContext {
    pub fn get<S: AsRef<str>>(&mut self, var: S) -> Variable {
        self.get_select::<S, false>(var)
    }

    pub fn name(&self, var: Variable) -> &ImmutableString {
        &self.strings[var.id() as usize]
    }

    #[inline(always)]
    pub(crate) fn get_select<S: AsRef<str>, const IS_BUILTIN: bool>(
        &mut self, var: S
    ) -> Variable {
        let var_name = var.as_ref();
        if let Some(var) = self.store.get(var_name) {
            *var
        } else {
            let Symbol { key, id } = self.new_symbol(var_name);
            let var = if IS_BUILTIN {
                Variable {
                    value: _Variable::Builtin(id)
                }
            } else {
                Variable {
                    value: _Variable::User(id)
                }
            };
            self.store.insert(key, var);
            var
        }
    }

    #[inline(always)]
    fn new_symbol(&mut self, symbol: &str) -> Symbol {
        let id = self.strings.len() as SymbolID;
        let key = ImmutableString::from(symbol);
        self.strings.push(key.clone());
        Symbol { key, id }
    }
}

pub trait Resolvable {
    fn resolve(&self, ctx: &mut SymbolContext) -> Variable;
    fn name<'a>(&self, ctx: &'a mut SymbolContext) -> &'a str;
}

impl<S: AsRef<str>> Resolvable for S {
    fn resolve(&self, ctx: &mut SymbolContext) -> Variable {
        ctx.get(self)
    }

    fn name<'a>(&self, ctx: &'a mut SymbolContext) -> &'a str {
        let var = self.resolve(ctx);
        ctx.name(var)
    }
}

impl Resolvable for Variable {
    fn resolve(&self, _ctx: &mut SymbolContext) -> Variable {
        *self
    }

    fn name<'a>(&self, ctx: &'a mut SymbolContext) -> &'a str {
        ctx.name(*self)
    }
}

pub struct VariablePromise {
    value: Box<dyn Resolvable>
}

impl<S: AsRef<str>> From<S> for VariablePromise {
    fn from(value: S) -> Self {
        VariablePromise {
            value: Box::new(value.as_ref().to_string())
        }
    }
}

impl From<Variable> for VariablePromise {
    fn from(value: Variable) -> Self {
        VariablePromise {
            value: Box::new(value)
        }
    }
}

impl Resolvable for VariablePromise {
    fn resolve(&self, ctx: &mut SymbolContext) -> Variable {
        self.value.resolve(ctx)
    }

    fn name<'a>(&self, ctx: &'a mut SymbolContext) -> &'a str {
        self.value.name(ctx)
    }
}
