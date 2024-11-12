#[derive(Debug, PartialEq, Eq)]
pub enum Comparison {
    Equal,
    Sublist,
    Superlist,
    Unequal,
}

pub fn sublist<T: PartialEq>(_first_list: &[T], _second_list: &[T]) -> Comparison {
    if _first_list == _second_list {
        Comparison::Equal
    } else if _second_list.starts_with(_first_list) || _second_list.ends_with(_first_list) {
        Comparison::Sublist
    } else if _first_list.starts_with(_second_list) || _first_list.ends_with(_second_list) {
        Comparison::Superlist
    } else if _first_list.len() > _second_list.len() {
        sublist(&_first_list[1..], _second_list)
    } else if _second_list.len() > _first_list.len() {
        sublist(_first_list, &_second_list[1..])
    } else {
        Comparison::Unequal
    }
}
