use std::rc::Rc;

pub fn grow_array<T: Clone>(array: &mut Vec<T>) -> Rc<Vec<T>> {
    let new_capacity = array.capacity() * 2;
    let mut new_array: Vec<T> = Vec::with_capacity(new_capacity);
    new_array.clone_from(&array);
    new_array.into()
}
