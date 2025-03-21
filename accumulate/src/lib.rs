/// What should the type of _function be?
pub fn map<T, U>(input: Vec<T>, mut function: impl FnMut(T) -> U) -> Vec<U> {
    let mut list = Vec::with_capacity(input.len());
    for v in input {
        list.push(function(v))
    }
    list
}
