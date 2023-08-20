use crate::memory::grow_array;

pub type Value = f64;

#[derive(Debug, Clone)]
pub struct ValueArray {
    values: Vec<Value>,
}

impl ValueArray {
    pub fn get_values(&mut self) -> Vec<Value> {
        self.values.clone()
    }

    pub fn free_value(&mut self) {
        self.values = Vec::with_capacity(8);
    }

    pub fn len(&self) -> usize {
        self.values.len()
    }

    #[must_use]
    pub fn is_empty(&self) -> bool {
        self.len() == 0
    }

    pub fn new() -> ValueArray {
        Self {
            values: Vec::with_capacity(8),
        }
    }

    pub fn push_value(&mut self, value: Value) {
        self.values.push(value);

        if self.values.capacity() == self.values.len() {
            self.values = grow_array(self.values.clone());
        }
    }
}

impl Default for ValueArray {
    fn default() -> Self {
        Self::new()
    }
}
