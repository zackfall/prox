use std::rc::Rc;

use crate::memory::grow_array;

pub type Value = f64;

#[derive(Debug, Clone)]
pub struct ValueArray {
    values: Rc<Vec<Value>>,
}

impl ValueArray {
    pub fn get_values(&mut self) -> Rc<Vec<Value>> {
        self.values.clone()
    }

    pub fn free_value(&mut self) {
        self.values = Rc::new(Vec::with_capacity(8));
    }

    pub fn len(&self) -> usize {
        self.values.len()
    }

    pub fn new() -> ValueArray {
        Self {
            values: Rc::new(Vec::with_capacity(8)),
        }
    }

    pub fn push_value(&mut self, value: Value) {
        let mut values = Rc::make_mut(&mut self.values);
        values.push(value);

        if values.capacity() == values.len() {
            self.values = grow_array(&mut values);
        }
    }
}
