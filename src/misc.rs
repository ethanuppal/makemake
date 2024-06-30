use crate::emittable::MakefileEmittable;

pub struct Comment {
    text: String
}

impl Comment {
    pub fn new<S: AsRef<str>>(text: S) -> Self {
        Self {
            text: text.as_ref().to_string()
        }
    }
}

impl MakefileEmittable for Comment {
    fn emit(&self) -> String {
        self.text
            .lines()
            .map(|line| format!("# {}", line))
            .collect::<Vec<_>>()
            .join("\n")
    }
}

pub struct Newline;

impl MakefileEmittable for Newline {
    fn emit(&self) -> String {
        String::new()
    }
}
