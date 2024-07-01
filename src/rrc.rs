use std::{cell::RefCell, rc::Rc};

pub type RRC<T> = Rc<RefCell<T>>;

pub fn rrc<T>(value: T) -> RRC<T> {
    Rc::new(RefCell::new(value))
}
