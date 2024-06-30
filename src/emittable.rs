pub trait MakefileEmittable {
    fn emit(&self) -> String;
}

pub(crate) type MakefileEmittableRef = Box<dyn MakefileEmittable>;
