use crate::emittable::MakefileEmittable;

#[derive(Clone)]
pub struct Variable {
    is_internal: bool,
    name: String
}

impl Variable {
    pub fn new<S: AsRef<str>>(name: S) -> Self {
        let name = name.as_ref();
        assert!(!name.is_empty());
        let (head, tail) = name.split_at(1);
        let head = head.chars().nth(0).unwrap();
        assert!(head.is_ascii_alphabetic() || head == '_');
        assert!(tail.chars().all(|c| c.is_ascii_alphanumeric() || c == '_'));
        Self {
            is_internal: false,
            name: name.to_string()
        }
    }

    fn internal<S: AsRef<str>>(name: S) -> Self {
        Self {
            is_internal: true,
            name: name.as_ref().to_string()
        }
    }

    pub fn name(&self) -> &str {
        &self.name
    }

    pub fn target() -> Self {
        Self::internal("@")
    }

    pub fn first_dep() -> Self {
        Self::internal("<")
    }

    pub fn deps() -> Self {
        Self::internal("^")
    }

    pub fn makefiles() -> Self {
        Variable::new("MAKEFILES")
    }
}

impl<T: AsRef<str>> From<T> for Variable {
    fn from(value: T) -> Self {
        Variable::new(value)
    }
}

impl MakefileEmittable for Variable {
    fn emit(&self) -> String {
        if self.is_internal {
            format!("${}", self.name)
        } else {
            format!("$({})", self.name)
        }
    }
}
