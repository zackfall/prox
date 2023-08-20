pub fn grow_array<T: Clone>(array: Vec<T>) -> Vec<T> {
    let new_capacity = array.capacity() * 2;
    let mut new_array: Vec<T> = Vec::with_capacity(new_capacity);
    new_array.clone_from(&array);
    new_array
}
